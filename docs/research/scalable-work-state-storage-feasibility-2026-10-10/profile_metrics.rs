//! Research-only wall spans and Windows process CPU counters, isolated archive.
use std::{cell::RefCell, collections::BTreeMap, sync::atomic::{AtomicBool, Ordering}, time::Instant};
use serde_json::{json, Value};
static ACTIVE: AtomicBool = AtomicBool::new(false);
#[derive(Default)]
struct Stat { calls: u64, wall: f64, own_wall: f64, cpu: f64, own_cpu: f64 }
struct Frame { child_wall: f64, child_cpu: f64 }
thread_local! {
    static STACK: RefCell<Vec<Frame>> = const { RefCell::new(Vec::new()) };
    static STATS: RefCell<BTreeMap<&'static str, Stat>> = RefCell::new(BTreeMap::new());
}
#[repr(C)]
#[derive(Default)]
struct FileTime { low: u32, high: u32 }
#[link(name="kernel32")]
extern "system" {
    fn GetCurrentProcess() -> *mut std::ffi::c_void;
    fn GetProcessTimes(h: *mut std::ffi::c_void, c: *mut FileTime, e: *mut FileTime, k: *mut FileTime, u: *mut FileTime) -> i32;
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
        let start = Instant::now(); let cpu = cpu_ms();
        STACK.with(|s| s.borrow_mut().push(Frame {child_wall:0.0,child_cpu:0.0}));
        Self(Some((name, start, cpu)))
    }
}
impl Drop for Span {
    fn drop(&mut self) {
        if let Some((name, start, cpu)) = self.0 {
            let elapsed = start.elapsed().as_secs_f64()*1000.0;
            let used = cpu_ms()-cpu;
            let frame = STACK.with(|s| {
                let mut stack=s.borrow_mut(); let frame=stack.pop().unwrap();
                if let Some(parent)=stack.last_mut() {parent.child_wall+=elapsed;parent.child_cpu+=used;}
                frame
            });
            STATS.with(|s| {let mut map=s.borrow_mut(); let v=map.entry(name).or_default();
                v.calls+=1;v.wall+=elapsed;v.cpu+=used;
                v.own_wall+=elapsed-frame.child_wall;v.own_cpu+=used-frame.child_cpu;});
        }
    }
}
pub fn begin() { STATS.with(|s| s.borrow_mut().clear()); ACTIVE.store(true, Ordering::Relaxed); }
pub fn finish() -> Value {
    ACTIVE.store(false, Ordering::Relaxed);
    STATS.with(|s| json!(s.borrow().iter().map(|(name,v)| json!({"symbol":name,"calls":v.calls,
        "inclusive_wall_ms":v.wall,"exclusive_wall_ms":v.own_wall,
        "inclusive_cpu_ms":v.cpu,"exclusive_cpu_ms":v.own_cpu})).collect::<Vec<_>>()))
}
