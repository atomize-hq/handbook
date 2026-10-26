use super::*;

thread_local!{
    static STOP:Cell<Option<&'static str>>=const{Cell::new(None)};
    static CHANGE:RefCell<Option<Box<dyn FnOnce()>>>=const{RefCell::new(None)};
}
pub(crate) fn boundary(phase:&str){
    if STOP.with(|s|s.get()==Some(phase)){STOP.with(|s|s.set(None));CLOCK.with(|c|c.set(Some(Duration::from_millis(50))));}
    if phase=="physical-stage-before-install"{if let Some(change)=CHANGE.with(|c|c.borrow_mut().take()){change();}}
}
struct Stop;
impl Drop for Stop{fn drop(&mut self){STOP.with(|s|s.set(None));CHANGE.with(|c|{c.borrow_mut().take();});}}

impl Fixture {
    fn construction(&self)->(StoreConfig,Vec<u8>){
        let mut next=self.w.clone();next.jobs.clear();next.reconciled_absent+=1;
        next.transition_parent=Some(self.c.physical.as_ref().unwrap().plan.clone());
        next.garbage.push(plan_name(next.transition_parent.as_ref().unwrap()).unwrap());next.garbage.sort();next.garbage.dedup();
        let bytes=io::json(&next).unwrap();let mut target=self.c.clone();target.physical.as_mut().unwrap().plan=io::digest(&bytes);
        io::write_new(&self.root.join(selector_stage(&self.w.generation.id).unwrap()),&io::json(&target).unwrap(),&mut Metrics::default()).unwrap();
        io::write_new(&self.root.join(work_stage(&self.w.generation.id).unwrap()),&bytes,&mut Metrics::default()).unwrap();
        assert!(!self.root.join(plan_name(&target.physical.as_ref().unwrap().plan).unwrap()).exists());
        (target,bytes)
    }
}

fn replace_construction(f:&Fixture,mut target:StoreConfig,bytes:&[u8])->StoreConfig{
    target.physical.as_mut().unwrap().plan=io::digest(bytes);
    fs::write(f.root.join(work_stage(&f.w.generation.id).unwrap()),bytes).unwrap();
    fs::write(f.root.join(selector_stage(&f.w.generation.id).unwrap()),io::json(&target).unwrap()).unwrap();target
}

fn resume_aborted_owner_adoption(installed:bool){
    let clock=Clock::new();let f=Fixture::new();let owner="aborted-owner";let owned=format!("packs/{owner}/{}.pack","a".repeat(64));
    let journal=json!({"version":2,"id":owner,"status":"aborted-owned","expected_config":io::digest(&io::json(&f.c).unwrap()),"catalog":null,"old":[],"reserved_bytes":0,"used_bytes":0,"deleted_bytes":0,"deferred_bytes":0,"ownership_transferred":false});
    io::write_new(&f.root.join("PACKING.json"),&io::json(&journal).unwrap(),&mut Metrics::default()).unwrap();
    io::write_new(&f.root.join(&owned),b"aborted payload",&mut Metrics::default()).unwrap();
    DEFER.with(|v|v.set(true));let mut first=Metrics::default();
    let result=crate::packing::run(&f.root,&json!({"operation":"cleanup"}),&mut first).unwrap();
    assert_eq!(result["maintenance_deferral"],"deferred:cleanup-entry-budget");drop(first);clock.reset();
    let stage=f.root.join(selector_stage(&f.w.generation.id).unwrap());let target:StoreConfig=strict(&fs::read(&stage).unwrap()).unwrap();
    let path=f.root.join(plan_name(&target.physical.as_ref().unwrap().plan).unwrap());let bytes=fs::read(&path).unwrap();let planned:Work=strict(&bytes).unwrap();
    assert_eq!(planned.birth_claims,vec![owner]);assert!(planned.garbage.contains(&owned));
    assert_eq!(config(&f.root,&mut Metrics::default()).unwrap().physical,f.c.physical);
    assert_eq!(strict::<serde_json::Value>(&fs::read(f.root.join("PACKING.json")).unwrap()).unwrap()["ownership_transferred"],false);
    // Use the exact payload produced by real adoption at the earlier complete
    // construction frontier, as well as its naturally deferred installed form.
    if !installed{fs::rename(&path,f.root.join(work_stage(&f.w.generation.id).unwrap())).unwrap();}
    let mut m=cleanup_metrics();let recovered=load(&f.root,&f.c,&mut m).unwrap();
    assert_eq!(recovered.birth_claims,vec![owner]);assert!(recovered.garbage.contains(&owned));
    assert_eq!(m.bytes_written,0);assert_eq!(m.file_syncs,if installed{0}else{1});assert_eq!(fs::read(&path).unwrap(),bytes);
    assert_eq!(config(&f.root,&mut Metrics::default()).unwrap().physical,target.physical);
    let mut retry=Metrics::default();crate::packing::run(&f.root,&json!({"operation":"cleanup"}),&mut retry).unwrap();
    assert_eq!(strict::<serde_json::Value>(&fs::read(f.root.join("PACKING.json")).unwrap()).unwrap()["ownership_transferred"],true);
    let selected=config(&f.root,&mut Metrics::default()).unwrap();let selected=load(&f.root,&selected,&mut Metrics::default()).unwrap();
    assert_eq!(selected.birth_claims,vec![owner]);
}

#[test]
fn complete_construction_resumes_aborted_owner_adoption(){resume_aborted_owner_adoption(false);}

#[test]
fn installed_checkpoint_resumes_aborted_owner_adoption(){resume_aborted_owner_adoption(true);}

#[test]
fn construction_deferrals_preserve_progress_at_each_persistence_frontier(){
    for phase in ["physical-stage-before-sync","physical-stage-after-sync","physical-stage-before-install","physical-stage-install-durable","physical-stage-before-selection"]{
        let clock=Clock::new();let _stop=Stop;let f=Fixture::new();let(target,bytes)=f.construction();let selector=f.root.join(selector_stage(&f.w.generation.id).unwrap());
        STOP.with(|s|s.set(Some(phase)));let mut m=cleanup_metrics();let error=load(&f.root,&f.c,&mut m).err().expect(phase);
        assert_eq!(error.to_string(),"deferred:cleanup-entry-budget");assert_eq!(CLOCK.with(Cell::get),Some(Duration::from_millis(50)));
        assert_eq!(config(&f.root,&mut Metrics::default()).unwrap().physical,f.c.physical);assert!(selector.exists());assert_eq!(m.bytes_written,0);
        let installed=f.root.join(plan_name(&target.physical.as_ref().unwrap().plan).unwrap()).exists();
        assert_eq!(installed,matches!(phase,"physical-stage-install-durable"|"physical-stage-before-selection"));
        clock.reset();let mut retry=cleanup_metrics();let w=load(&f.root,&f.c,&mut retry).unwrap();
        assert!(w.jobs.is_empty());assert_eq!(w.reconciled_absent,1);assert_eq!(retry.bytes_written,0);
        assert_eq!(retry.file_syncs,if installed{0}else{1});assert!(CLOCK.with(Cell::get).unwrap()<Duration::from_millis(50));
        assert_eq!(config(&f.root,&mut Metrics::default()).unwrap().physical,target.physical);
        assert_eq!(fs::read(f.root.join(plan_name(&target.physical.as_ref().unwrap().plan).unwrap())).unwrap(),bytes);
        eprintln!("construction deferral/retry {phase}: same progress selected, payload writes0");
    }
}

#[test]
fn construction_invalid_owner_or_frozen_state_refuses_before_sync_or_install(){
    let _clock=Clock::new();
    for case in ["selector-owner","selector-generation","selector-ordinal","selector-catalog","selector-mode","work-owner","work-generation","work-ordinal","work-catalog","stale-parent","physical-parent","pack-identity","birth-claims","duplicate-work","duplicate-selector"]{
        let f=Fixture::new();let(mut target,bytes)=f.construction();let mut w:Work=strict(&bytes).unwrap();
        match case{
            "selector-owner"=>target.store_id="foreign".into(),"selector-generation"=>target.physical.as_mut().unwrap().id="foreign".into(),
            "selector-ordinal"=>target.physical.as_mut().unwrap().ordinal+=1,"selector-catalog"=>target.catalog=Some("f".repeat(64)),
            "selector-mode"=>target.mode="B".into(),"work-owner"=>w.store_id="foreign".into(),
            "work-generation"=>w.generation.id="other".into(),"work-ordinal"=>w.generation.ordinal+=1,
            "work-catalog"=>w.generation.catalog="f".repeat(64),"stale-parent"=>w.transition_parent=Some("f".repeat(64)),
            "physical-parent"=>w.physical_parent="f".repeat(64),"pack-identity"=>w.generation.packs.clear(),
            "birth-claims"=>w.birth_claims.push("../foreign-birth".into()),_=>{},
        }
        let mut raw=io::json(&w).unwrap();if case=="duplicate-work"{raw.splice(1..1,b"\"version\":2,".iter().copied());}
        target=replace_construction(&f,target,&raw);
        if case=="duplicate-selector"{let mut b=io::json(&target).unwrap();b.splice(1..1,b"\"version\":3,".iter().copied());fs::write(f.root.join(selector_stage(&f.w.generation.id).unwrap()),b).unwrap();}
        let before=fs::read(f.root.join("STORE.json")).unwrap();let mut m=cleanup_metrics();let err=load(&f.root,&f.c,&mut m).err().expect(case);
        assert!(!err.to_string().starts_with("deferred:"),"{case}: {err}");assert_eq!(m.file_syncs,0);assert_eq!(m.bytes_written,0);assert_eq!(m.namespace_ops,0);
        assert_eq!(fs::read(f.root.join("STORE.json")).unwrap(),before);assert!(f.root.join(work_stage(&f.w.generation.id).unwrap()).exists());
        assert!(!f.root.join(plan_name(&target.physical.as_ref().unwrap().plan).unwrap()).exists());
    }
}

#[test]
fn construction_revalidates_actual_namespace_and_bytes_before_install(){
    let _clock=Clock::new();
    for case in ["payload-inode","selector-inode","payload-bytes","plans-directory"]{
        let _stop=Stop;let f=Fixture::new();f.construction();let root=f.root.clone();
        CHANGE.with(|c|*c.borrow_mut()=Some(Box::new(move||{
            match case{
                "payload-inode"|"selector-inode"=>{
                    let name=if case=="payload-inode"{work_stage("checkpoint-g1").unwrap()}else{selector_stage("checkpoint-g1").unwrap()};let path=root.join(name);let bytes=fs::read(&path).unwrap();
                    fs::rename(&path,root.join("retained-original-file")).unwrap();fs::write(path,bytes).unwrap();
                },
                "payload-bytes"=>{let path=root.join(work_stage("checkpoint-g1").unwrap());let mut bytes=fs::read(&path).unwrap();bytes[0]=b'[';fs::write(path,bytes).unwrap();},
                _=>{fs::rename(root.join("physical-plans"),root.join("retained-original-plans")).unwrap();fs::create_dir(root.join("physical-plans")).unwrap();},
            }
        })));
        let before=fs::read(f.root.join("STORE.json")).unwrap();let mut m=cleanup_metrics();let error=load(&f.root,&f.c,&mut m).err().expect(case);
        assert!(error.to_string().starts_with("integrity:construction-"),"{case}: {error}");assert_eq!(m.file_syncs,1,"recheck occurs after stage sync");assert_eq!(m.namespace_ops,0);assert_eq!(m.bytes_written,0);
        assert_eq!(fs::read(f.root.join("STORE.json")).unwrap(),before);
    }
}

// Native child, selected only by the parent test below. Process-local injection
// cannot change the environment of concurrently running ordinary tests.
#[test]
fn native_recovery_child(){
    let Some(root)=std::env::var_os("EXP_STAGE_RECOVERY_CHILD") else{return;};
    let _clock=Clock::new();let root=PathBuf::from(root);let c=config(&root,&mut Metrics::default()).unwrap();
    load(&root,&c,&mut cleanup_metrics()).unwrap();panic!("requested native cut was not reached");
}

#[test]
fn native_stops_preserve_authority_across_sync_install_and_selection(){
    use std::process::{Command,Stdio};
    let _clock=Clock::new();
    let mut phases=vec!["physical-stage-before-sync","physical-stage-after-sync","physical-stage-before-install","physical-stage-install-durable","physical-stage-before-selection","physical-progress-selected"];
    #[cfg(unix)] phases.push("immutable-name-installed");
    for phase in phases{
        let f=Fixture::new();let(target,bytes)=f.construction();let selector=f.root.join(selector_stage(&f.w.generation.id).unwrap());let plan=f.root.join(plan_name(&target.physical.as_ref().unwrap().plan).unwrap());
        let mut command=Command::new(std::env::current_exe().unwrap());command.args(["--exact","physical_lifetime::checkpoint_tests::stage_recovery_tests::native_recovery_child","--nocapture","--test-threads=1"])
            .env("EXP_STAGE_RECOVERY_CHILD",&f.root).env("EXP_EXIT",phase).env_remove("EXP_ERROR").env_remove("EXP_BARRIER").env_remove("EXP_NTH").stdout(Stdio::piped()).stderr(Stdio::piped());
        let child=command.spawn().unwrap();let pid=child.id();let output=child.wait_with_output().unwrap();
        fs::write(f.root.join("native-cut.stdout"),&output.stdout).unwrap();fs::write(f.root.join("native-cut.stderr"),&output.stderr).unwrap();assert_eq!(output.status.code(),Some(86),"{phase}: {}",String::from_utf8_lossy(&output.stderr));
        let after_cas=phase=="physical-progress-selected";let c=config(&f.root,&mut Metrics::default()).unwrap();assert_eq!(c.physical,if after_cas{target.physical.clone()}else{f.c.physical.clone()});
        assert_eq!(selector.exists(),!after_cas);
        let installed=matches!(phase,"physical-stage-install-durable"|"physical-stage-before-selection"|"physical-progress-selected"|"immutable-name-installed");assert_eq!(plan.exists(),installed,"{phase}");
        let mut m=cleanup_metrics();let w=load(&f.root,&c,&mut m).unwrap();assert!(w.jobs.is_empty());assert_eq!(w.reconciled_absent,1);assert_eq!(m.bytes_written,0);
        assert_eq!(m.file_syncs,if installed{0}else{1},"{phase}");assert_eq!(config(&f.root,&mut Metrics::default()).unwrap().physical,target.physical);assert_eq!(fs::read(&plan).unwrap(),bytes);
        // Once selected, any surviving construction name is disposable, while
        // an old parent still present remains explicitly queued for cleanup.
        let mut again=cleanup_metrics();let w=load(&f.root,&target,&mut again).unwrap();assert!(w.jobs.is_empty());assert_eq!(w.reconciled_absent,1);assert_eq!(again.bytes_written,0);
        if after_cas{assert!(w.garbage.contains(&plan_name(&f.c.physical.as_ref().unwrap().plan).unwrap()));}
        fs::write(f.root.join("native-cut-receipt.json"),io::json(&json!({"phase":phase,"pid":pid,"exit":86,"original_wait":true,"stdout_eof":true,"stderr_eof":true,"stdout_bytes":output.stdout.len(),"stderr_bytes":output.stderr.len(),"selected_before_recovery":c.physical,"target":target.physical,"installed_at_cut":installed,"recovery_payload_bytes_written":m.bytes_written,"recovery_file_syncs":m.file_syncs})).unwrap()).unwrap();
        eprintln!("native-stage-cut phase={phase} pid={pid} exit86 waited/EOF; immutable_before_recovery={installed}; resumed_same_progress=true");
    }
}

#[test]
fn incomplete_or_wrong_digest_construction_is_not_selected(){
    let _clock=Clock::new();
    for bytes in [b"{\"version\":2,".as_slice(),b"null",b"{\"version\":2}"]{
        let f=Fixture::new();let(target,_)=f.construction();fs::write(f.root.join(work_stage(&f.w.generation.id).unwrap()),bytes).unwrap();
        let mut m=cleanup_metrics();let w=load(&f.root,&f.c,&mut m).unwrap();assert_eq!(w.jobs.len(),1);assert_eq!(m.file_syncs,0);assert_eq!(m.bytes_written,0);
        assert_eq!(config(&f.root,&mut Metrics::default()).unwrap().physical,f.c.physical);assert!(!f.root.join(plan_name(&target.physical.as_ref().unwrap().plan).unwrap()).exists());
    }
}

#[test]
fn construction_work_bound_accepts_limit_and_refuses_oversize_before_sync(){
    let _clock=Clock::new();
    for size in [WORK_LIMIT,WORK_LIMIT+1]{
        let f=Fixture::new();let(target,bytes)=f.construction();let mut w:Work=strict(&bytes).unwrap();
        w.garbage.extend((0..1100).map(|n|format!("physical-pins/p{n:04}.lock")));
        let mut left=size-io::json(&w).unwrap().len();
        for name in w.garbage.iter_mut().filter(|n|n.starts_with("physical-pins/")){
            let length=name.len()-"physical-pins/".len()-5;let n=left.min(99-length);let at=name.len()-5;name.insert_str(at,&"x".repeat(n));left-=n;
        }
        assert_eq!(left,0);let bytes=io::json(&w).unwrap();assert_eq!(bytes.len(),size);let target=replace_construction(&f,target,&bytes);let mut m=cleanup_metrics();
        let recovered=load(&f.root,&f.c,&mut m);
        if size==WORK_LIMIT{
            assert!(recovered.unwrap().jobs.is_empty());assert_eq!(m.file_syncs,1);assert_eq!(fs::read(f.root.join(plan_name(&target.physical.as_ref().unwrap().plan).unwrap())).unwrap(),bytes);
        }else{
            assert_eq!(recovered.err().unwrap().to_string(),"integrity:construction-file-bound");assert_eq!(m.file_syncs,0);assert_eq!(m.namespace_ops,0);assert_eq!(config(&f.root,&mut Metrics::default()).unwrap().physical,f.c.physical);
        }
        assert_eq!(m.bytes_written,0);
    }
}

#[test]
fn construction_rechecks_reader_parent_and_native_selection_gates(){
    let clock=Clock::new();
    for gate in ["physical-capture.lock","writer.lock"]{
        let f=Fixture::new();let(target,bytes)=f.construction();let lock=lock_file(&f.root.join(gate),true).unwrap();FileExt::try_lock_exclusive(&lock).unwrap();
        let mut m=cleanup_metrics();let e=load(&f.root,&f.c,&mut m).err().unwrap();assert!(e.to_string().starts_with("deferred:work-"));assert_eq!(m.bytes_written,0);
        assert_eq!(config(&f.root,&mut Metrics::default()).unwrap().physical,f.c.physical);assert_eq!(fs::read(f.root.join(plan_name(&target.physical.as_ref().unwrap().plan).unwrap())).unwrap(),bytes);
        drop(lock);let mut retry=cleanup_metrics();assert!(load(&f.root,&f.c,&mut retry).unwrap().jobs.is_empty());assert_eq!(retry.file_syncs,0);
    }
    let f=Fixture::new();f.construction();let mut changed=f.c.clone();changed.fallback=true;PARENT_RACE.with(|p|*p.borrow_mut()=Some(changed.clone()));
    let mut m=cleanup_metrics();assert_eq!(load(&f.root,&f.c,&mut m).err().unwrap().to_string(),"conflict:work-parent");assert_eq!(m.bytes_written,0);
    assert_eq!(fs::read(f.root.join("STORE.json")).unwrap(),io::json(&changed).unwrap());
    clock.reset();let mut f=Fixture::new();let reader=f.hold_older_reader();let(target,bytes)=f.construction();let mut planned:Work=strict(&bytes).unwrap();planned.older.clear();let raw=io::json(&planned).unwrap();let target=replace_construction(&f,target,&raw);
    let mut m=cleanup_metrics();assert_eq!(load(&f.root,&f.c,&mut m).err().unwrap().to_string(),"deferred:work-readers");assert_eq!(m.file_syncs,0);assert_eq!(m.namespace_ops,0);
    assert!(!f.root.join(plan_name(&target.physical.as_ref().unwrap().plan).unwrap()).exists());drop(reader);
    let mut retry=cleanup_metrics();assert!(load(&f.root,&f.c,&mut retry).unwrap().older.is_empty());
}

#[test]
fn completed_removal_credit_and_absence_replay_are_preserved_once(){
    let _clock=Clock::new();let f=Fixture::new();let(target,bytes)=f.construction();let name=f.root.join("physical-pins/finished.lock");fs::write(&name,b"removed").unwrap();
    assert_eq!(crate::lifetime::remove_checked(&f.root,&name,&mut Metrics::default()).unwrap(),7);
    let mut planned:Work=strict(&bytes).unwrap();planned.removed_files=1;planned.removed_bytes=7;let target=replace_construction(&f,target,&io::json(&planned).unwrap());
    let mut m=cleanup_metrics();let selected=load(&f.root,&f.c,&mut m).unwrap();assert_eq!((selected.removed_files,selected.removed_bytes,selected.reconciled_absent),(1,7,1));assert_eq!(m.bytes_written,0);
    let mut retry=cleanup_metrics();let selected=load(&f.root,&target,&mut retry).unwrap();assert_eq!((selected.removed_files,selected.removed_bytes,selected.reconciled_absent),(1,7,1));assert_eq!(retry.file_syncs,0);
}

#[cfg(unix)]
#[test]
fn construction_symlinks_aliases_and_directory_substitution_refuse(){
    use std::os::unix::fs::symlink;
    let _clock=Clock::new();
    for case in ["work-symlink","selector-symlink","work-hardlink","progress-symlink","plans-symlink"]{
        let f=Fixture::new();f.construction();let work=f.root.join(work_stage(&f.w.generation.id).unwrap());let selector=f.root.join(selector_stage(&f.w.generation.id).unwrap());
        match case{
            "work-symlink"|"selector-symlink"=>{let p=if case=="work-symlink"{work}else{selector};let foreign=f.root.with_file_name("retained-foreign-file");fs::rename(&p,&foreign).unwrap();symlink(&foreign,&p).unwrap();},
            "work-hardlink"=>fs::hard_link(&work,f.root.with_file_name("retained-alias")).unwrap(),
            _=>{let p=f.root.join(if case=="progress-symlink"{"physical-progress"}else{"physical-plans"});let foreign=f.root.with_file_name("retained-foreign-dir");fs::rename(&p,&foreign).unwrap();symlink(&foreign,&p).unwrap();},
        }
        let before=fs::read(f.root.join("STORE.json")).unwrap();let mut m=cleanup_metrics();let e=load(&f.root,&f.c,&mut m).err().expect(case);assert!(e.to_string().starts_with("integrity:construction-"),"{case}: {e}");
        assert_eq!(m.file_syncs,0);assert_eq!(m.namespace_ops,0);assert_eq!(fs::read(f.root.join("STORE.json")).unwrap(),before);
    }
}

#[test]
fn complete_uninstalled_stage_resumes_same_progress_without_payload_rewrite(){
    let _clock=Clock::new();let f=Fixture::new();let(target,bytes)=f.construction();
    let mut m=cleanup_metrics();let selected=load(&f.root,&f.c,&mut m).unwrap();
    assert!(selected.jobs.is_empty(),"complete owned construction progress must survive recovery");
    assert_eq!(selected.reconciled_absent,1);
    assert_eq!(config(&f.root,&mut Metrics::default()).unwrap().physical,target.physical);
    assert_eq!(fs::read(f.root.join(plan_name(&target.physical.as_ref().unwrap().plan).unwrap())).unwrap(),bytes);
    assert_eq!(m.bytes_written,0,"recovery must finish persistence, not reconstruct payloads");
    assert_eq!(m.file_syncs,1,"construction must be re-synced before immutable installation");
}
