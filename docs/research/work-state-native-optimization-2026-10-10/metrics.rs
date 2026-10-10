//! Research-only, no persistent telemetry. Based on preceding native profile.
use std::{cell::RefCell, collections::BTreeMap, sync::atomic::{AtomicBool, Ordering}, time::Instant};
use serde_json::{json, Value};
static ACTIVE: AtomicBool = AtomicBool::new(false);
#[derive(Default)]
struct Stat { calls: u64, wall: f64, own_wall: f64, cpu: f64, own_cpu: f64 }
struct Frame { child_wall: f64, child_cpu: f64 }
thread_local! {
    static STACK: RefCell<Vec<Frame>> = const { RefCell::new(Vec::new()) };
    static STATS: RefCell<BTreeMap<&'static str, Stat>> = RefCell::new(BTreeMap::new());
    static PATHS: RefCell<BTreeMap<(String,String), (u64,u64)>> = RefCell::new(BTreeMap::new());
}
#[repr(C)]
#[derive(Default)]
struct FileTime { low: u32, high: u32 }
#[link(name="kernel32")]
extern "system" {
    fn GetCurrentProcess() -> *mut std::ffi::c_void;
    fn GetProcessTimes(h: *mut std::ffi::c_void, c: *mut FileTime, e: *mut FileTime, k: *mut FileTime, u: *mut FileTime) -> i32;
    fn GetProcessHandleCount(h: *mut std::ffi::c_void, count: *mut u32) -> i32;
}
pub fn handles() -> u32 {
    let mut count=0;
    unsafe { assert_ne!(GetProcessHandleCount(GetCurrentProcess(), &mut count), 0); }
    count
}
pub fn cpu_ms() -> f64 {
    let (mut c, mut e, mut k, mut u) = (FileTime::default(), FileTime::default(), FileTime::default(), FileTime::default());
    unsafe { assert_ne!(GetProcessTimes(GetCurrentProcess(), &mut c, &mut e, &mut k, &mut u), 0); }
    (((k.high as u64) << 32 | k.low as u64) + ((u.high as u64) << 32 | u.low as u64)) as f64 / 10000.0
}
pub struct Span(Option<(&'static str, Instant, f64)>);
impl Span {
    pub fn enter(name: &'static str) -> Self {
        if !ACTIVE.load(Ordering::Relaxed) { return Self(None); }
        let start=Instant::now(); let cpu=cpu_ms();
        STACK.with(|s| s.borrow_mut().push(Frame {child_wall:0.0,child_cpu:0.0}));
        Self(Some((name,start,cpu)))
    }
}
impl Drop for Span {
    fn drop(&mut self) {
        if let Some((name,start,cpu))=self.0 {
            let elapsed=start.elapsed().as_secs_f64()*1000.0; let used=cpu_ms()-cpu;
            let frame=STACK.with(|s| { let mut stack=s.borrow_mut(); let frame=stack.pop().unwrap();
                if let Some(parent)=stack.last_mut() {parent.child_wall+=elapsed;parent.child_cpu+=used;} frame });
            STATS.with(|s| {let mut map=s.borrow_mut();let v=map.entry(name).or_default();
                v.calls+=1;v.wall+=elapsed;v.cpu+=used;v.own_wall+=elapsed-frame.child_wall;v.own_cpu+=used-frame.child_cpu;});
        }
    }
}
pub fn count_path(kind: &str,path: &str,n:u64) {
    if ACTIVE.load(Ordering::Relaxed) {
        PATHS.with(|s| {let mut map=s.borrow_mut();let v=map.entry((kind.into(),path.into())).or_default();v.0+=1;v.1+=n;});
    }
}
pub fn begin() {
    STATS.with(|s| s.borrow_mut().clear());PATHS.with(|s| s.borrow_mut().clear());ACTIVE.store(true,Ordering::Relaxed);
}
pub fn finish() -> Value {
    ACTIVE.store(false,Ordering::Relaxed);
    let spans=STATS.with(|s| json!(s.borrow().iter().map(|(name,v)| json!({"symbol":name,"calls":v.calls,
        "inclusive_wall_ms":v.wall,"exclusive_wall_ms":v.own_wall,
        "inclusive_cpu_ms":v.cpu,"exclusive_cpu_ms":v.own_cpu})).collect::<Vec<_>>()));
    let paths=PATHS.with(|s| json!(s.borrow().iter().map(|((kind,path),(calls,n))|
        json!({"kind":kind,"path":path,"calls":calls,"units":n})).collect::<Vec<_>>()));
    json!({"spans":spans,"paths":paths})
}
pub fn probe_optional(repo: &std::path::Path, path: &str) -> Result<crate::RepositoryReadV1, crate::RepositoryDurabilityErrorV1> {
    crate::RetainedRepositoryRootV1::open_exact(repo)?.read_local_owner_optional_bounded(path,1024)
}
pub fn probe_direct(repo: &std::path::Path, path: &str) -> Result<crate::RepositoryReadV1, crate::RepositoryDurabilityErrorV1> {
    crate::RetainedRepositoryRootV1::open_exact(repo)?.read_regular_bounded(path,1024)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn research_parent_binding_keeps_fresh_leaf_bytes_and_releases_on_drop() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<crate::RetainedRepositoryRootV1>();
        let dir=tempfile::tempdir().unwrap();
        let parent=dir.path().join(".handbook/work-state/v1");std::fs::create_dir_all(&parent).unwrap();
        std::fs::write(parent.join("value.json"),b"first").unwrap();
        let root=crate::RetainedRepositoryRootV1::open_exact(dir.path()).unwrap();
        assert_eq!(root.read_regular_bounded(".handbook/work-state/v1/value.json",10).unwrap(),crate::RepositoryReadV1::Found(b"first".to_vec()));
        std::fs::write(parent.join("value.json"),b"other").unwrap();
        assert_eq!(root.read_regular_bounded(".handbook/work-state/v1/value.json",10).unwrap(),crate::RepositoryReadV1::Found(b"other".to_vec()));
        assert!(std::fs::rename(&parent,dir.path().join("moved")).is_err());
        drop(root);
        std::fs::rename(&parent,dir.path().join("moved")).unwrap();
    }
    #[test]
    fn research_bootstrap_absence_remains_distinct_from_substitution() {
        let dir=tempfile::tempdir().unwrap();
        assert_eq!(probe_optional(dir.path(),".handbook/work-state/v1/value.json").unwrap(),crate::RepositoryReadV1::Absent);
        std::fs::create_dir_all(dir.path().join(".handbook")).unwrap();
        std::fs::write(dir.path().join(".handbook/work-state"),b"not a directory").unwrap();
        assert!(probe_optional(dir.path(),".handbook/work-state/v1/value.json").is_err());
    }
    #[test]
    fn research_held_parent_rejects_new_hardlink_on_each_inventory() {
        let dir=tempfile::tempdir().unwrap();let parent=dir.path().join(".handbook/work-state/v1");std::fs::create_dir_all(&parent).unwrap();
        std::fs::write(parent.join("value.json"),b"first").unwrap();
        let root=crate::RetainedRepositoryRootV1::open_exact(dir.path()).unwrap();
        assert_eq!(root.list_directory_bounded(".handbook/work-state/v1",10).unwrap().len(),1);
        std::fs::hard_link(parent.join("value.json"),dir.path().join("alias")).unwrap();
        assert!(root.list_directory_bounded(".handbook/work-state/v1",10).is_err());
        assert!(root.read_regular_bounded(".handbook/work-state/v1/value.json",10).is_err());
    }
}
