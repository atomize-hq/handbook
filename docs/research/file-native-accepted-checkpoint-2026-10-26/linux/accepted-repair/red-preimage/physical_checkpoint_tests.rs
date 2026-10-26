use super::*;
use std::{cell::Cell,fs,path::PathBuf};
use serde_json::json;

thread_local! {
    static CLOCK:Cell<Option<Duration>>=const{Cell::new(None)};
    static DEFER:Cell<bool>=const{Cell::new(false)};
}
pub(super) fn elapsed()->Option<Duration>{CLOCK.with(Cell::get)}
pub(super) fn after_plan_durable(){
    if DEFER.with(|v|v.replace(false)){CLOCK.with(|v|v.set(Some(Duration::from_millis(50))));}
}
struct Clock;
impl Clock {
    fn new()->Self{CLOCK.with(|v|v.set(Some(Duration::ZERO)));Self}
    fn reset(&self){CLOCK.with(|v|v.set(Some(Duration::ZERO)));}
}
impl Drop for Clock{fn drop(&mut self){CLOCK.with(|v|v.set(None));DEFER.with(|v|v.set(false));}}

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
