use super::*;
use std::{cell::{Cell,RefCell},fs,path::PathBuf};
use serde_json::json;

thread_local! {
    static CLOCK:Cell<Option<Duration>>=const{Cell::new(None)};
    static DEFER:Cell<bool>=const{Cell::new(false)};
    static PARENT_RACE:RefCell<Option<StoreConfig>>=const{RefCell::new(None)};
}
pub(super) fn elapsed()->Option<Duration>{CLOCK.with(Cell::get)}
pub(super) fn after_plan_durable(){
    if DEFER.with(|v|v.replace(false)){CLOCK.with(|v|v.set(Some(Duration::from_millis(50))));}
}
pub(super) fn before_selection(root:&Path,_m:&mut Metrics)->Result<()>{
    if let Some(c)=PARENT_RACE.with(|v|v.borrow_mut().take()){
        let mut other=Metrics::default();let store=Store{root:root.into(),config:c.clone()};
        let _writer=store.try_writer(&mut other)?.ok_or("test:writer-not-available")?;
        io::atomic(&root.join("STORE.json"),&io::json(&c)?,&mut other)?;
    }
    Ok(())
}
struct Clock;
impl Clock {
    fn new()->Self{CLOCK.with(|v|v.set(Some(Duration::ZERO)));Self}
    fn reset(&self){CLOCK.with(|v|v.set(Some(Duration::ZERO)));}
}
impl Drop for Clock{fn drop(&mut self){CLOCK.with(|v|v.set(None));DEFER.with(|v|v.set(false));PARENT_RACE.with(|v|{v.borrow_mut().take();});}}

struct Fixture{root:PathBuf,c:StoreConfig,w:Work}
impl Fixture {
    fn new()->Self{
        let root=PathBuf::from(std::env::var_os("EXP_PACK_TEST_ROOT").expect("explicit fixture root"))
            .join(io::unique("checkpoint-resume")).join("disposable/store");
        let mut m=Metrics::default();
        let mut builder=pack::Builder::new("result",0);
        builder.push(pack::Key::new(&format!("results/{}.json","a".repeat(64))).unwrap(),b"fixed-result").unwrap();
        let(mut d,body,index)=builder.finish().unwrap();d.instance=Some("checkpoint-g1".into());d.segment=1;
        io::write_new(&root.join(d.path("pack").unwrap()),&body,&mut m).unwrap();
        io::write_new(&root.join(d.path("idx").unwrap()),&index,&mut m).unwrap();
        let cat=io::json(&json!({"version":4,"store_id":"checkpoint-store","packs":[d.clone()]})).unwrap();
        let catalog=io::digest(&cat);
        io::write_new(&root.join(format!("pack-catalogs/{catalog}.json")),&cat,&mut m).unwrap();
        let w=Work{discovery:None,version:2,store_id:"checkpoint-store".into(),physical_parent:"0".repeat(64),transition_parent:None,birth_claims:vec![],
            generation:Generation{id:"checkpoint-g1".into(),ordinal:2,catalog:catalog.clone(),plan:None,packs:vec![d.clone()]},older:vec![],
            jobs:vec![LooseJob{descriptor:d,frontier:2,position:0}],retired:vec![],garbage:vec![],removed_bytes:0,removed_files:0,reconciled_absent:0};
        let plan=persist_plan(&root,&w,&mut m).unwrap();
        let c=StoreConfig{version:3,store_id:w.store_id.clone(),mode:"A".into(),consumer:"project".into(),fallback:false,navigation_version:4,
            catalog:Some(catalog),physical:Some(Binding{id:w.generation.id.clone(),ordinal:2,plan})};
        io::write_new(&root.join("STORE.json"),&io::json(&c).unwrap(),&mut m).unwrap();
        io::write_new(&root.join(pin_name(&w.generation.id).unwrap()),&io::json(&identity(&c).unwrap()).unwrap(),&mut m).unwrap();
        io::write_new(&root.join("DISPOSABLE.json"),&io::json(&root.canonicalize().unwrap().to_string_lossy().as_ref()).unwrap(),&mut m).unwrap();
        Self{root,c,w}
    }
    fn pending(&self,clock:&Clock)->(StoreConfig,Vec<u8>){
        let mut advanced=self.w.clone();advanced.jobs.clear();advanced.reconciled_absent=1;
        let mut m=Metrics::default();m.cleanup_budget=Some(CleanupBudget::new());DEFER.with(|v|v.set(true));
        let error=save(&self.root,self.c.physical.as_ref().unwrap(),&mut advanced,&mut m).unwrap_err();
        assert_eq!(error.to_string(),"deferred:cleanup-entry-budget");clock.reset();
        let staged:StoreConfig=strict(&fs::read(self.root.join(selector_stage(&self.w.generation.id).unwrap())).unwrap()).unwrap();
        let bytes=fs::read(self.root.join(plan_name(&staged.physical.as_ref().unwrap().plan).unwrap())).unwrap();
        assert_eq!(config(&self.root,&mut Metrics::default()).unwrap().physical,self.c.physical);
        (staged,bytes)
    }
    fn hold_older_reader(&mut self)->Metrics{
        let older=Generation{id:"checkpoint-g0".into(),ordinal:1,catalog:self.w.generation.catalog.clone(),plan:Some("b".repeat(64)),packs:self.w.generation.packs.clone()};
        let mut old=self.c.clone();old.physical=Some(Binding{id:older.id.clone(),ordinal:older.ordinal,plan:older.plan.clone().unwrap()});
        let mut m=Metrics::default();
        io::write_new(&self.root.join(pin_name(&older.id).unwrap()),&io::json(&identity(&old).unwrap()).unwrap(),&mut m).unwrap();
        io::atomic(&self.root.join("STORE.json"),&io::json(&old).unwrap(),&mut m).unwrap();
        let mut reader=Metrics::default();Store::open(&self.root,&mut reader).unwrap();
        self.w.older.push(older);self.c.physical.as_mut().unwrap().plan=persist_plan(&self.root,&self.w,&mut m).unwrap();
        io::atomic(&self.root.join("STORE.json"),&io::json(&self.c).unwrap(),&mut m).unwrap();
        reader
    }
}

fn cleanup_metrics()->Metrics{let mut m=Metrics::default();m.cleanup_budget=Some(CleanupBudget::new());m}

#[test]
fn partial_work_stage_is_not_adopted(){
    let _clock=Clock::new();let f=Fixture::new();let mut target=f.c.clone();target.physical.as_mut().unwrap().plan="f".repeat(64);
    let selector=f.root.join(selector_stage(&f.w.generation.id).unwrap());
    let partial=f.root.join(work_stage(&f.w.generation.id).unwrap());
    fs::write(&selector,io::json(&target).unwrap()).unwrap();fs::write(&partial,b"{\"version\":2,").unwrap();
    let before=fs::read(f.root.join("STORE.json")).unwrap();
    let selected=load(&f.root,&f.c,&mut cleanup_metrics()).unwrap();
    assert_eq!(selected.jobs.len(),1);assert_eq!(selected.jobs[0].position,0);
    assert_eq!(fs::read(f.root.join("STORE.json")).unwrap(),before);
    assert!(!selector.exists());assert!(!partial.exists());
}

#[test]
fn invalid_pending_checkpoints_refuse_before_selection_or_removal(){
    let clock=Clock::new();
    for case in ["selector-owner","selector-generation","selector-ordinal","selector-catalog","selector-mode","path",
        "work-owner","work-generation","work-ordinal","work-catalog","stale-parent","physical-parent","pack-identity","digest","duplicate-selector","duplicate-work"]{
        let f=Fixture::new();let(mut target,bytes)=f.pending(&clock);let mut planned:Work=strict(&bytes).unwrap();
        match case{
            "selector-owner"=>target.store_id="foreign-store".into(),
            "selector-generation"=>target.physical.as_mut().unwrap().id="foreign-generation".into(),
            "selector-ordinal"=>target.physical.as_mut().unwrap().ordinal+=1,
            "selector-catalog"=>target.catalog=Some("f".repeat(64)),
            "selector-mode"=>target.mode="B".into(),
            "path"=>target.physical.as_mut().unwrap().plan="../outside".into(),
            "work-owner"=>planned.store_id="foreign-store".into(),
            "work-generation"=>planned.generation.id="foreign-generation".into(),
            "work-ordinal"=>planned.generation.ordinal+=1,
            "work-catalog"=>planned.generation.catalog="f".repeat(64),
            "stale-parent"=>planned.transition_parent=Some("f".repeat(64)),
            "physical-parent"=>planned.physical_parent="f".repeat(64),
            "pack-identity"=>planned.generation.packs.clear(),
            _=>{},
        }
        let raw=if case=="duplicate-work"{let mut raw=io::json(&planned).unwrap();raw.splice(1..1,b"\"version\":2,".iter().copied());raw}else{io::json(&planned).unwrap()};
        if case.starts_with("work-")||matches!(case,"stale-parent"|"physical-parent"|"pack-identity"|"duplicate-work"){
            target.physical.as_mut().unwrap().plan=io::digest(&raw);
            io::write_new(&f.root.join(plan_name(&target.physical.as_ref().unwrap().plan).unwrap()),&raw,&mut Metrics::default()).unwrap();
        }
        let selector=f.root.join(selector_stage(&f.w.generation.id).unwrap());let mut selector_bytes=io::json(&target).unwrap();
        if case=="duplicate-selector"{selector_bytes.splice(1..1,b"\"version\":3,".iter().copied());}
        fs::write(&selector,&selector_bytes).unwrap();
        if case=="digest"{fs::write(f.root.join(plan_name(&target.physical.as_ref().unwrap().plan).unwrap()),b"corrupt").unwrap();}
        let before=fs::read(f.root.join("STORE.json")).unwrap();let mut m=cleanup_metrics();
        let error=load(&f.root,&f.c,&mut m).err().expect(case);
        assert!(!error.to_string().starts_with("deferred:"),"{case}: {error}");
        assert_eq!(fs::read(f.root.join("STORE.json")).unwrap(),before,"{case}");
        assert_eq!(fs::read(&selector).unwrap(),selector_bytes,"{case}");
        assert_eq!(m.namespace_ops,0,"{case}: invalid ownership must not remove anything");
        assert!(m.physical_work_holds_ns.is_empty(),"{case}");
        eprintln!("pending refusal {case}: {error}");
    }
}

#[test]
fn capture_and_writer_contention_preserve_pending_checkpoint_for_resume(){
    let clock=Clock::new();
    for gate in ["physical-capture.lock","writer.lock"]{
        let f=Fixture::new();let(target,bytes)=f.pending(&clock);
        let lock=lock_file(&f.root.join(gate),true).unwrap();FileExt::try_lock_exclusive(&lock).unwrap();
        let mut m=cleanup_metrics();let error=load(&f.root,&f.c,&mut m).err().expect("held native gate must defer");
        assert_eq!(error.to_string(),if gate=="writer.lock"{"deferred:work-writer"}else{"deferred:work-capture"});
        assert_eq!(config(&f.root,&mut Metrics::default()).unwrap().physical,f.c.physical);
        assert_eq!(fs::read(f.root.join(plan_name(&target.physical.as_ref().unwrap().plan).unwrap())).unwrap(),bytes);
        assert!(f.root.join(selector_stage(&f.w.generation.id).unwrap()).exists());
        drop(lock);let mut m=cleanup_metrics();let selected=load(&f.root,&f.c,&mut m).unwrap();
        assert!(selected.jobs.is_empty());assert_eq!(m.bytes_written,0);
        assert_eq!(config(&f.root,&mut Metrics::default()).unwrap().physical,target.physical);
    }
}

#[test]
fn resumed_commit_rechecks_actual_parent_under_native_gate(){
    let clock=Clock::new();let f=Fixture::new();let(target,bytes)=f.pending(&clock);
    let mut changed=f.c.clone();changed.fallback=true;PARENT_RACE.with(|v|*v.borrow_mut()=Some(changed.clone()));
    let mut m=cleanup_metrics();let error=load(&f.root,&f.c,&mut m).err().expect("changed parent must refuse");
    assert_eq!(error.to_string(),"conflict:work-parent");
    assert_eq!(fs::read(f.root.join("STORE.json")).unwrap(),io::json(&changed).unwrap());
    assert_eq!(fs::read(f.root.join(plan_name(&target.physical.as_ref().unwrap().plan).unwrap())).unwrap(),bytes);
    assert!(f.root.join(selector_stage(&f.w.generation.id).unwrap()).exists());assert_eq!(m.bytes_written,0);
    assert_eq!(m.physical_work_holds_ns.len(),1);
}

#[test]
fn save_does_not_overwrite_resumed_progress_with_stale_caller_work(){
    let clock=Clock::new();let f=Fixture::new();let(target,_)=f.pending(&clock);
    let mut stale=f.w.clone();let mut m=cleanup_metrics();save(&f.root,f.c.physical.as_ref().unwrap(),&mut stale,&mut m).unwrap();
    assert!(stale.jobs.is_empty());assert_eq!(stale.reconciled_absent,1);assert_eq!(m.bytes_written,0);
    assert_eq!(config(&f.root,&mut Metrics::default()).unwrap().physical,target.physical);
}

#[test]
fn non_cleanup_load_does_not_promote_pending_work(){
    let clock=Clock::new();let f=Fixture::new();let(target,_)=f.pending(&clock);
    let selected=load(&f.root,&f.c,&mut Metrics::default()).unwrap();
    assert_eq!(selected.jobs.len(),1);assert_eq!(selected.jobs[0].position,0);
    assert_eq!(config(&f.root,&mut Metrics::default()).unwrap().physical,f.c.physical);
    assert!(!f.root.join(plan_name(&target.physical.as_ref().unwrap().plan).unwrap()).exists());
}

#[test]
fn resumed_work_keeps_real_old_reader_and_retirement_protection(){
    let clock=Clock::new();let mut f=Fixture::new();let mut old_reader=f.hold_older_reader();let(target,_)=f.pending(&clock);
    let key=f.w.jobs[0].descriptor.first.clone();
    assert_eq!(io::read(&f.root.join(&key),&mut old_reader).unwrap(),b"fixed-result");
    let mut m=Metrics::default();let result=crate::packing::run(&f.root,&json!({"operation":"cleanup"}),&mut m).unwrap();
    assert_eq!(result["status"],"cleaning");assert_eq!(result["retained_generations"],2);assert_eq!(result["blocked_checks"],1);
    assert_eq!(config(&f.root,&mut Metrics::default()).unwrap().physical,target.physical);assert_eq!(m.bytes_written,0);
    assert!(!drained(&f.root,&f.w.older[0],&f.c.store_id,&mut Metrics::default()).unwrap());
    assert_eq!(io::read(&f.root.join(&key),&mut old_reader).unwrap(),b"fixed-result");
    let mut fresh=Metrics::default();Store::open(&f.root,&mut fresh).unwrap();
    assert_eq!(io::read(&f.root.join(&key),&mut fresh).unwrap(),b"fixed-result");
    drop(old_reader);assert!(drained(&f.root,&f.w.older[0],&f.c.store_id,&mut Metrics::default()).unwrap());
}

#[test]
fn resumed_checkpoint_cannot_drop_a_native_pinned_generation(){
    let clock=Clock::new();let mut f=Fixture::new();let reader=f.hold_older_reader();let(mut target,bytes)=f.pending(&clock);
    let mut planned:Work=strict(&bytes).unwrap();planned.older.clear();let bytes=io::json(&planned).unwrap();let hash=io::digest(&bytes);
    let path=f.root.join(plan_name(&hash).unwrap());io::write_new(&path,&bytes,&mut Metrics::default()).unwrap();
    target.physical.as_mut().unwrap().plan=hash;
    let stage=f.root.join(selector_stage(&f.w.generation.id).unwrap());fs::write(&stage,io::json(&target).unwrap()).unwrap();
    let error=load(&f.root,&f.c,&mut cleanup_metrics()).err().expect("live reader must prevent retirement");
    assert_eq!(error.to_string(),"deferred:work-readers");
    assert_eq!(config(&f.root,&mut Metrics::default()).unwrap().physical,f.c.physical);
    assert_eq!(fs::read(&path).unwrap(),bytes);assert!(stage.exists());
    drop(reader);
}

#[test]
fn cleanup_resumes_durable_checkpoint_without_persistence_replay(){
    let clock=Clock::new();let f=Fixture::new();let(staged,bytes)=f.pending(&clock);
    let path=f.root.join(plan_name(&staged.physical.as_ref().unwrap().plan).unwrap());
    #[cfg(unix)] let inode={use std::os::unix::fs::MetadataExt;fs::metadata(&path).unwrap().ino()};
    let mut m=Metrics::default();let result=crate::packing::run(&f.root,&json!({"operation":"cleanup"}),&mut m).unwrap();
    assert_eq!(result["status"],"complete");
    assert_eq!(config(&f.root,&mut Metrics::default()).unwrap().physical,staged.physical);
    assert_eq!(fs::read(&path).unwrap(),bytes);
    #[cfg(unix)] {use std::os::unix::fs::MetadataExt;assert_eq!(fs::metadata(&path).unwrap().ino(),inode);}
    assert_eq!(m.bytes_written,0,"resuming must not rewrite checkpoint payloads");
    assert_eq!(m.file_syncs,0,"already durable candidate needs no file rewrite/sync");
    let selected=load(&f.root,&config(&f.root,&mut Metrics::default()).unwrap(),&mut Metrics::default()).unwrap();
    assert!(selected.jobs.is_empty());assert_eq!(selected.reconciled_absent,1);
    eprintln!("durable checkpoint resumed: selected jobs1->0 reconciled0->1; original candidate inode/bytes retained; checkpoint bytes_written=0 file_syncs=0");
}
