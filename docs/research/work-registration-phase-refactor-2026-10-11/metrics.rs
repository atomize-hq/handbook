//! Disposable research telemetry only. No stored authority/cache/profile changes.
use std::{cell::{Cell,RefCell},collections::BTreeMap,fs::File,os::windows::io::AsRawHandle,time::Instant};
use serde_json::{json,Value};
#[derive(Default)] struct Stat{calls:u64,wall:f64,own_wall:f64,cpu:f64,own_cpu:f64}
#[derive(Debug)]struct Frame{name:&'static str,child_wall:f64,child_cpu:f64,observation:usize}
thread_local! {
 static ACTIVE:Cell<bool>=const{Cell::new(false)};
 static STACK:RefCell<Vec<Frame>>=const{RefCell::new(Vec::new())};
 static STATS:RefCell<BTreeMap<(&'static str,usize),Stat>>=RefCell::new(BTreeMap::new());
 static COUNTS:RefCell<BTreeMap<(String,usize),(u64,u64)>>=RefCell::new(BTreeMap::new());
 static GAUGES:RefCell<BTreeMap<&'static str,u64>>=RefCell::new(BTreeMap::new());
 static PATHS:RefCell<BTreeMap<(String,String,usize),(u64,u64)>>=RefCell::new(BTreeMap::new());
 static LOCKS:RefCell<BTreeMap<usize,(String,Instant,f64)>>=RefCell::new(BTreeMap::new());
 static SCANS:Cell<usize>=const{Cell::new(0)};
}
#[repr(C)]#[derive(Default)]struct FileTime{low:u32,high:u32}
#[link(name="kernel32")]extern "system"{
 fn GetCurrentProcess()->*mut std::ffi::c_void;
 fn GetProcessTimes(h:*mut std::ffi::c_void,c:*mut FileTime,e:*mut FileTime,k:*mut FileTime,u:*mut FileTime)->i32;
 fn GetProcessHandleCount(h:*mut std::ffi::c_void,n:*mut u32)->i32;
}
pub fn handles()->u32{let mut n=0;unsafe{assert_ne!(GetProcessHandleCount(GetCurrentProcess(),&mut n),0);}n}
fn cpu_ms()->f64{let(mut c,mut e,mut k,mut u)=(FileTime::default(),FileTime::default(),FileTime::default(),FileTime::default());unsafe{assert_ne!(GetProcessTimes(GetCurrentProcess(),&mut c,&mut e,&mut k,&mut u),0);}
 (((k.high as u64)<<32|k.low as u64)+((u.high as u64)<<32|u.low as u64))as f64/10000.0}
fn observation()->usize{STACK.with(|s|s.borrow().last().map_or(0,|f|f.observation))}
fn active()->bool{ACTIVE.with(Cell::get)}
pub struct Span(Option<(&'static str,Instant,f64)>);
impl Span{pub fn enter(name:&'static str)->Self{if !active(){return Self(None);}let start=Instant::now();let cpu=cpu_ms();
 let n=if name=="work_scan"{SCANS.with(|n|{let v=n.get()+1;n.set(v);v})}else{observation()};
 STACK.with(|s|s.borrow_mut().push(Frame{name,child_wall:0.0,child_cpu:0.0,observation:n}));Self(Some((name,start,cpu)))}}
impl Drop for Span{fn drop(&mut self){if let Some((name,start,cpu))=self.0{let elapsed=start.elapsed().as_secs_f64()*1000.0;let used=cpu_ms()-cpu;
 let frame=STACK.with(|s|{let mut stack=s.borrow_mut();let f=stack.pop().unwrap();if let Some(p)=stack.last_mut(){p.child_wall+=elapsed;p.child_cpu+=used;}f});
 STATS.with(|s|{let mut map=s.borrow_mut();let v=map.entry((name,frame.observation)).or_default();v.calls+=1;v.wall+=elapsed;v.cpu+=used;v.own_wall+=elapsed-frame.child_wall;v.own_cpu+=used-frame.child_cpu;});}}}
pub fn count(name:&str,units:u64){if active(){COUNTS.with(|s|{let mut m=s.borrow_mut();let v=m.entry((name.into(),observation())).or_default();v.0+=1;v.1+=units;});}}
pub fn gauge(name:&'static str,v:u64){if active(){GAUGES.with(|s|{let mut m=s.borrow_mut();let old=m.entry(name).or_default();*old=(*old).max(v);});}}
pub fn count_path(kind:&str,path:&str,units:u64){if active(){PATHS.with(|s|{let mut m=s.borrow_mut();let v=m.entry((kind.into(),path.into(),observation())).or_default();v.0+=1;v.1+=units;});}}
pub fn lock_acquired(file:&File,path:&str){if active(){LOCKS.with(|m|{m.borrow_mut().insert(file.as_raw_handle()as usize,(path.into(),Instant::now(),cpu_ms()));});}}
pub fn lock_released(file:&File){if active(){LOCKS.with(|m|{if let Some((path,start,cpu))=m.borrow_mut().remove(&(file.as_raw_handle()as usize)){
  let elapsed=start.elapsed().as_secs_f64()*1000.0;let used=cpu_ms()-cpu;
  PATHS.with(|s|{let mut m=s.borrow_mut();let v=m.entry(("lock_held_us".into(),path,0)).or_default();v.0+=1;v.1+=(elapsed*1000.0)as u64;});
  count("lock_held_cpu_us",(used*1000.0)as u64);
 } });}}
pub fn begin(){STATS.with(|s|s.borrow_mut().clear());COUNTS.with(|s|s.borrow_mut().clear());PATHS.with(|s|s.borrow_mut().clear());GAUGES.with(|s|s.borrow_mut().clear());SCANS.with(|s|s.set(0));LOCKS.with(|s|assert!(s.borrow().is_empty(),"unreleased research lock entries: {:?}",s.borrow()));ACTIVE.with(|s|s.set(true));}
pub fn finish()->Value{ACTIVE.with(|s|s.set(false));STACK.with(|s|assert!(s.borrow().is_empty(),"unclosed research frames: {:?}",s.borrow()));let remaining=LOCKS.with(|s|s.borrow().len());
 let spans=STATS.with(|s|s.borrow().iter().map(|((name,n),v)|json!({"symbol":name,"observation":n,"calls":v.calls,"inclusive_wall_ms":v.wall,"exclusive_wall_ms":v.own_wall,"inclusive_cpu_ms":v.cpu,"exclusive_cpu_ms":v.own_cpu})).collect::<Vec<_>>());
 let counts=COUNTS.with(|s|s.borrow().iter().map(|((name,n),(calls,units))|json!({"counter":name,"observation":n,"calls":calls,"units":units})).collect::<Vec<_>>());
 let paths=PATHS.with(|s|s.borrow().iter().map(|((kind,path,n),(calls,units))|json!({"kind":kind,"path":path,"observation":n,"calls":calls,"units":units})).collect::<Vec<_>>());
 let gauges=GAUGES.with(|s|json!(*s.borrow()));json!({"spans":spans,"counts":counts,"paths":paths,"gauges":gauges,"retained_locks_after_phase":remaining})}
