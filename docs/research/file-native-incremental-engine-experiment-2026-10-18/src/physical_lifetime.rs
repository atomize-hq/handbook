//! Private cross-process physical lifetime protocol. STORE is the selection
//! authority; journals only checkpoint work. Ordinary capture never reads work.
use crate::{io::{self,Metrics,Result},pack::{self,Descriptor},store::{Store,StoreConfig}};
use fs2::FileExt;
use serde::{Serialize,Deserialize};
use std::{fs::{File,OpenOptions},io::{Read,Seek,SeekFrom},path::Path,time::{Duration,Instant}};

pub(crate) const BINDING_LIMIT:usize=4096;
pub(crate) const CATALOG_LIMIT:usize=59*1024;
const WORK_LIMIT:usize=4*1024*1024;
const MAX_GENERATIONS:usize=64;
const MAX_JOBS:usize=4096;

#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Eq)]
#[serde(deny_unknown_fields)]
pub struct Binding {pub id:String,pub ordinal:u64,pub plan:String}
#[derive(Clone,Serialize,Deserialize,PartialEq,Eq)]
#[serde(deny_unknown_fields)]
struct PinIdentity {version:u32,store_id:String,id:String,ordinal:u64,catalog:String}

pub(crate) fn strict<T:serde::de::DeserializeOwned>(bytes:&[u8])->Result<T>{
    Ok(serde_json::from_value(crate::api::strict_json(std::str::from_utf8(bytes)?)?)?)
}
// Explicit Windows sharing and noninheritance. Never duplicate a pin handle:
// fs2's Windows duplicate implementation explicitly requests inheritance.
pub(crate) fn lock_file(path:&Path,create:bool)->Result<File>{
    let mut options=OpenOptions::new();options.read(true).write(true).create(create).truncate(false);
    #[cfg(windows)] {use std::os::windows::fs::OpenOptionsExt;options.share_mode(3);}
    let f=options.open(path)?;
    #[cfg(windows)] {
        use std::os::windows::io::AsRawHandle;
        use windows_sys::Win32::Foundation::{SetHandleInformation,HANDLE_FLAG_INHERIT};
        if unsafe{SetHandleInformation(f.as_raw_handle(),HANDLE_FLAG_INHERIT,0)}==0{return Err(std::io::Error::last_os_error().into());}
    }
    Ok(f)
}
pub(crate) fn contended(e:&std::io::Error)->bool{e.kind()==fs2::lock_contended_error().kind()}
pub(crate) fn cancelled()->bool{std::env::var("EXP_PACK_CANCEL").ok().is_some_and(|p|Path::new(&p).exists())}
pub(crate) fn wait_lock(root:&Path,name:&str,exclusive:bool)->Result<File>{
    let f=lock_file(&root.join(name),true)?;let start=Instant::now();
    let cap=std::env::var("EXP_WAIT_MS").ok().and_then(|s|s.parse().ok()).unwrap_or(15000);
    loop {
        let result=if exclusive{FileExt::try_lock_exclusive(&f)}else{FileExt::try_lock_shared(&f)};
        match result {Ok(())=>return Ok(f),Err(e) if contended(&e)=>{},Err(e)=>return Err(e.into())}
        if cancelled(){return Err("cancelled:pack-barrier".into());}
        if start.elapsed()>Duration::from_millis(cap){return Err("busy:physical-readers".into());}
        std::thread::sleep(Duration::from_millis(2));
    }
}
pub(crate) fn config(root:&Path,m:&mut Metrics)->Result<StoreConfig>{
    let c:StoreConfig=strict(&pack::native(&root.join("STORE.json"),BINDING_LIMIT,m)?)?;
    match (c.version,&c.physical,&c.catalog) {
        (1,None,None)|(2,None,Some(_))=>{},
        (3,Some(b),Some(h)) if crate::model::safe_id(&b.id)&&b.ordinal>0&&crate::objects::valid_hash(&b.plan)&&crate::objects::valid_hash(h)=>{},
        _=>return Err("integrity:physical-binding-version".into()),
    }
    Ok(c)
}
fn pin_name(id:&str)->Result<String>{
    if !crate::model::safe_id(id){return Err("integrity:generation-id".into());}
    Ok(format!("physical-pins/{id}.lock"))
}
fn identity(c:&StoreConfig)->Result<PinIdentity>{
    let b=c.physical.as_ref().ok_or("integrity:generation-binding")?;
    Ok(PinIdentity{version:1,store_id:c.store_id.clone(),id:b.id.clone(),ordinal:b.ordinal,catalog:c.catalog.clone().ok_or("integrity:generation-catalog")?})
}
pub(crate) fn create_pin(root:&Path,c:&StoreConfig,m:&mut Metrics)->Result<()>{
    let i=identity(c)?;io::write_new(&root.join(pin_name(&i.id)?),&io::json(&i)?,m)
}
fn validate_pin(f:&mut File,expected:&PinIdentity,m:&mut Metrics)->Result<()>{
    if f.metadata()?.len()>1024{return Err("integrity:pin-bound".into());}
    f.seek(SeekFrom::Start(0))?;let mut bytes=vec![];f.take(1025).read_to_end(&mut bytes)?;
    if bytes.len()>1024||strict::<PinIdentity>(&bytes)?!=*expected{return Err("integrity:pin-binding".into());}
    m.opens+=1;m.bytes_read+=bytes.len() as u64;Ok(())
}
// Q is held for one binding read and one native pin acquisition/identity read.
// Catalog/index loading and every checkpoint wait occur after Q is dropped.
pub(crate) fn capture(root:&Path,m:&mut Metrics)->Result<(StoreConfig,File)>{
    loop {
        let q=wait_lock(root,"physical-capture.lock",false)?;let t=Instant::now();let c=config(root,m)?;
        if c.version==3 {
            let expected=identity(&c)?;let mut pin=lock_file(&root.join(pin_name(&expected.id)?),false)?;
            FileExt::try_lock_shared(&pin).map_err(|e|format!("integrity:selected-pin-unavailable:{e}"))?;
            validate_pin(&mut pin,&expected,m)?;drop(q);m.elapsed("physical-capture-gate-hold",t);
            return Ok((c,pin));
        }
        drop(q);
        // Conversion may race the preliminary read. Acquire the old lease and
        // reread; a post-conversion legacy lease supplies no v3 protection.
        let lease=wait_lock(root,"pack-readers.lock",false)?;let c=config(root,m)?;
        if c.version==3{drop(lease);continue;}
        return Ok((c,lease));
    }
}

#[derive(Clone,Serialize,Deserialize,PartialEq,Eq)]
#[serde(deny_unknown_fields)]
pub(crate) struct Generation {pub id:String,pub ordinal:u64,pub catalog:String,pub plan:Option<String>,pub packs:Vec<Descriptor>}
#[derive(Clone,Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LooseJob {pub descriptor:Descriptor,pub frontier:u64,pub position:u32}
#[derive(Clone,Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Retired {pub descriptor:Descriptor,pub pack_done:bool,pub index_done:bool}
#[derive(Clone,Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Work {
    pub version:u32,pub store_id:String,pub transition_parent:Option<String>,pub birth_claims:Vec<String>,pub generation:Generation,pub older:Vec<Generation>,
    // Descriptor+ordinal cursors stream the legacy whole indexes. No key array,
    // predecessor-plan chain or one permanent file per activation is retained.
    pub jobs:Vec<LooseJob>,pub retired:Vec<Retired>,pub garbage:Vec<String>,
    pub removed_bytes:u64,pub removed_files:u64,pub reconciled_absent:u64,
}
fn progress_name(id:&str)->Result<String>{
    if !crate::model::safe_id(id){return Err("integrity:progress-id".into());}Ok(format!("physical-progress/{id}.json"))
}
fn plan_name(hash:&str)->Result<String>{
    if !crate::objects::valid_hash(hash){return Err("integrity:work-hash".into());}Ok(format!("physical-plans/{hash}.json"))
}
fn metadata_path(name:&str)->bool{
    let parts:Vec<_>=name.split('/').collect();
    match parts.as_slice(){
        ["physical-pins",id]=>id.strip_suffix(".lock").is_some_and(crate::model::safe_id),
        ["physical-progress",id]=>[".json",".work-stage",".selector-stage"].iter().any(|suffix|id.strip_suffix(suffix).is_some_and(crate::model::safe_id)),
        ["physical-plans"|"pack-catalogs",id]=>id.strip_suffix(".json").is_some_and(crate::objects::valid_hash),
        ["physical-stages",id,"STORE.json"]=>crate::model::safe_id(id),
        ["packs",id,file]=>crate::model::safe_id(id)&&file.rsplit_once('.').is_some_and(|(h,ext)|crate::objects::valid_hash(h)&&matches!(ext,"pack"|"idx")),
        _=>false,
    }
}
pub(crate) fn validate_work(w:&Work)->Result<()>{
    if w.birth_claims.len()>64||w.birth_claims.iter().any(|id|!crate::model::safe_id(id))||w.transition_parent.as_ref().is_some_and(|h|!crate::objects::valid_hash(h))||w.version!=2||!crate::model::safe_id(&w.generation.id)||w.generation.ordinal==0||!crate::objects::valid_hash(&w.generation.catalog)||w.generation.packs.len()>64||w.older.len()>=MAX_GENERATIONS||w.jobs.len()>MAX_JOBS||w.retired.len()>MAX_JOBS||w.garbage.len()>MAX_JOBS{return Err("quota:physical-bookkeeping-bound".into());}
    let mut ids=std::collections::BTreeSet::new();
    for g in &w.older {
        if !crate::model::safe_id(&g.id)||g.ordinal>=w.generation.ordinal||!ids.insert(g.id.clone())||!crate::objects::valid_hash(&g.catalog)||g.plan.as_ref().is_none_or(|h|!crate::objects::valid_hash(h))||g.packs.len()>64{return Err("integrity:retained-generation".into());}
    }
    for j in &w.jobs{if j.position>j.descriptor.records||j.frontier>w.generation.ordinal||j.frontier==0{return Err("integrity:cleanup-cursor".into());}}
    for d in w.generation.packs.iter().chain(w.older.iter().flat_map(|g|&g.packs)).chain(w.jobs.iter().map(|j|&j.descriptor)).chain(w.retired.iter().map(|r|&r.descriptor)){d.path("pack")?;d.path("idx")?;}
    if w.garbage.iter().any(|p|!metadata_path(p)){return Err("integrity:work-metadata-path".into());}
    Ok(())
}
pub(crate) fn persist_plan(root:&Path,w:&Work,m:&mut Metrics)->Result<String>{
    validate_work(w)?;let bytes=io::json(w)?;if bytes.len()>WORK_LIMIT{return Err("quota:physical-work-bytes".into());}
    let hash=io::digest(&bytes);let path=root.join(plan_name(&hash)?);
    if path.exists(){if pack::native(&path,WORK_LIMIT,m)?!=bytes{return Err("integrity:existing-work-plan".into());}}
    else{
        let stage=root.join(work_stage(&w.generation.id)?);io::write_new(&stage,&bytes,m)?;
        io::mkdir(path.parent().ok_or("integrity:work-plan-parent")?,m)?;
        // Only complete synced bytes receive a hashed immutable name. A native
        // death during write leaves the fixed, durably owned work-stage instead.
        io::install_original(&stage,&path,&hash,m)?;
    }Ok(hash)
}
fn work_stage(id:&str)->Result<String>{
    if !crate::model::safe_id(id){return Err("integrity:progress-stage-id".into());}Ok(format!("physical-progress/{id}.work-stage"))
}
fn selector_stage(id:&str)->Result<String>{
    if !crate::model::safe_id(id){return Err("integrity:progress-stage-id".into());}Ok(format!("physical-progress/{id}.selector-stage"))
}
fn task_present(root:&Path,name:&str)->Result<bool>{
    let path=root.join(name);
    if native_present(&path)?{return Ok(true);}
    if name.starts_with("physical-stages/"){return native_present(path.parent().ok_or("integrity:stage-parent")?);}
    Ok(false)
}
fn native_present(path:&Path)->Result<bool>{
    match std::fs::symlink_metadata(path){Ok(_)=>Ok(true),Err(e) if e.kind()==std::io::ErrorKind::NotFound=>Ok(false),Err(e)=>Err(e.into())}
}
// One deterministic stage per generation. It owns the candidate plan hash before
// that plan can be created. No random 4MiB progress stages or directory scan.
fn recover_stage(root:&Path,c:&StoreConfig,w:&Work,m:&mut Metrics)->Result<()>{
    let b=c.physical.as_ref().ok_or("integrity:work-binding")?;let stage=root.join(selector_stage(&b.id)?);
    let work_stage=root.join(work_stage(&b.id)?);
    if !native_present(&stage)?{
        if native_present(&work_stage)?{crate::lifetime::remove_checked(root,&work_stage,m)?;}
        return Ok(());
    }
    let bytes=pack::native(&stage,BINDING_LIMIT,m)?;
    {
        let candidate:StoreConfig=strict(&bytes).map_err(|e|format!("integrity:work-stage-selector:{e}"))?;
        if candidate.store_id!=c.store_id||candidate.catalog!=c.catalog||candidate.physical.as_ref().is_none_or(|s|s.id!=b.id||s.ordinal!=b.ordinal){return Err("integrity:foreign-work-stage".into());}
        let next=candidate.physical.as_ref().unwrap();
        if next.plan!=b.plan&&!w.older.iter().any(|g|g.plan.as_ref()==Some(&next.plan)){
            let orphan=root.join(plan_name(&next.plan)?);
            if native_present(&orphan)?{
                let data=pack::native(&orphan,WORK_LIMIT,m)?;
                if io::digest(&data)!=next.plan{return Err("integrity:work-stage-plan-digest".into());}
                let planned:Work=strict(&data)?;validate_work(&planned)?;
                if planned.transition_parent.as_ref()!=Some(&b.plan)||planned.store_id!=c.store_id||planned.generation.id!=b.id||planned.generation.ordinal!=b.ordinal||Some(&planned.generation.catalog)!=c.catalog.as_ref(){return Err("integrity:work-stage-plan-owner".into());}
                crate::lifetime::remove_checked(root,&orphan,m)?;
            }
        }
    }
    // Unknown/corrupt ownership refuses before removal. A valid stage and a
    // verified same-birth transition root identify only disposable checkpoints;
    // ordinary readers never acquire work-root ownership.
    if native_present(&work_stage)?{crate::lifetime::remove_checked(root,&work_stage,m)?;}
    crate::lifetime::remove_checked(root,&stage,m)?;Ok(())
}
fn record_work_hold(m:&mut Metrics,hold:Instant)->Result<()>{
    if m.physical_work_holds_ns.len()>=10000{return Err("quota:work-attempt-bound".into());}
    m.physical_work_holds_ns.push(hold.elapsed().as_nanos().min(u64::MAX as u128) as u64);Ok(())
}
// Progress is a selected immutable root, not a mutable journal. Its CAS changes
// only the work identity within the same pinned physical generation/catalog.
pub(crate) fn save(root:&Path,b:&Binding,w:&mut Work,m:&mut Metrics)->Result<()>{
    let c=config(root,m)?;let current=c.physical.as_ref().ok_or("integrity:work-binding")?;
    if current.id!=b.id||current.ordinal!=b.ordinal||Some(&w.generation.catalog)!=c.catalog.as_ref(){return Err("conflict:work-generation".into());}
    recover_stage(root,&c,w,m)?;
    let canonical=load(root,&c,m)?;
    // Absent immutable-instance names are native completion facts. Do not make
    // an idle cleanup publish another root merely to remove an absent tombstone.
    if io::json(&canonical)?==io::json(w)?{return Ok(());}
    let old=current.plan.clone();w.transition_parent=Some(old.clone());let old_name=plan_name(&old)?;
    if !w.garbage.contains(&old_name){w.garbage.push(old_name.clone());}
    w.garbage.sort();w.garbage.dedup();validate_work(w)?;
    let work_bytes=io::json(w)?;if work_bytes.len()>WORK_LIMIT{return Err("quota:physical-work-bytes".into());}
    let hash=io::digest(&work_bytes);let mut target=c.clone();target.physical.as_mut().unwrap().plan=hash.clone();
    let stage=root.join(selector_stage(&b.id)?);io::write_new(&stage,&io::json(&target)?,m)?;
    if persist_plan(root,w,m)?!=hash{return Err("integrity:work-plan-correspondence".into());}
    io::checkpoint("physical-progress-plan-installed")?;
    let expected=io::digest(&io::json(&c)?);let store=Store{root:root.into(),config:c};let wait=Instant::now();
    let cap=std::env::var("EXP_WAIT_MS").ok().and_then(|s|s.parse().ok()).unwrap_or(15000);
    loop{
        if cancelled(){return Err("cancelled:physical-cleanup".into());}
        let q=wait_lock(root,"physical-capture.lock",true)?;let hold=Instant::now();
        let actual=pack::native(&root.join("STORE.json"),BINDING_LIMIT,m)?;
        if io::digest(&actual)!=expected{drop(q);record_work_hold(m,hold)?;return Err("conflict:work-parent".into());}
        let Some(writer)=store.try_writer(m)? else{
            drop(q);record_work_hold(m,hold)?;if wait.elapsed()>Duration::from_millis(cap){return Err("busy:work-writer".into());}
            std::thread::sleep(Duration::from_millis(2));continue;
        };
        let selected=io::replace(&stage,&root.join("STORE.json"),m).and_then(|_|io::sync_dir(root,m));
        drop(writer);drop(q);record_work_hold(m,hold)?;m.elapsed("physical-work-selection-hold",hold);selected?;break;
    }
    io::checkpoint("physical-progress-selected")?;
    // The new immutable root owns this exact obsolete plan. No ordinary reader
    // reads a work root. Retained generations/build owners remain explicit.
    if !w.older.iter().any(|g|g.plan.as_ref()==Some(&old))&&native_present(&root.join(&old_name))?{
        match crate::lifetime::remove_checked(root,&root.join(&old_name),m){
            Ok(_)=>{w.garbage.retain(|p|p!=&old_name);io::checkpoint("physical-work-old-root-delete")?;},
            Err(e) if e.downcast_ref::<std::io::Error>().is_some_and(|e|matches!(e.raw_os_error(),Some(32|33)))=>{},
            Err(e)=>return Err(e),
        }
    }
    Ok(())
}
pub(crate) fn load(root:&Path,c:&StoreConfig,m:&mut Metrics)->Result<Work>{
    let b=c.physical.as_ref().ok_or("integrity:work-binding")?;
    if native_present(&root.join(progress_name(&b.id)?))?{return Err("integrity:unexpected-mutable-progress".into());}
    let bytes=pack::native(&root.join(plan_name(&b.plan)?),WORK_LIMIT,m)?;
    if io::digest(&bytes)!=b.plan{return Err("integrity:work-plan-digest".into());}
    let mut work:Work=strict(&bytes)?;validate_work(&work)?;
    if work.store_id!=c.store_id||work.generation.id!=b.id||work.generation.ordinal!=b.ordinal||Some(&work.generation.catalog)!=c.catalog.as_ref(){return Err("integrity:work-selection-binding".into());}
    recover_stage(root,c,&work,m)?;
    let mut pending=vec![];
    for name in &work.garbage{if task_present(root,name)?{pending.push(name.clone());}}
    // Selected generation identity owns its exact activation-stage namespace.
    // A death after STORE replacement can leave only the empty directory.
    for id in std::iter::once(&work.generation.id).chain(work.older.iter().map(|g|&g.id)){
        let name=format!("physical-stages/{id}/STORE.json");
        if task_present(root,&name)?&&!pending.contains(&name){pending.push(name);}
    }
    work.garbage=pending;Ok(work)
}
pub(crate) fn generation(c:&StoreConfig,packs:Vec<Descriptor>)->Result<Generation>{
    let b=c.physical.as_ref().ok_or("integrity:generation-binding")?;
    Ok(Generation{id:b.id.clone(),ordinal:b.ordinal,catalog:c.catalog.clone().ok_or("integrity:generation-catalog")?,plan:Some(b.plan.clone()),packs})
}
pub(crate) fn drained(root:&Path,g:&Generation,store_id:&str,m:&mut Metrics)->Result<bool>{
    let mut f=lock_file(&root.join(pin_name(&g.id)?),false)?;
    match FileExt::try_lock_exclusive(&f){Ok(())=>{},Err(e) if contended(&e)=>return Ok(false),Err(e)=>return Err(e.into())}
    validate_pin(&mut f,&PinIdentity{version:1,store_id:store_id.into(),id:g.id.clone(),ordinal:g.ordinal,catalog:g.catalog.clone()},m)?;
    // No new acquisition of a sealed id is possible. The exclusive handle can
    // close before its name is deleted; generation ids are never reused.
    Ok(true)
}
pub(crate) fn reap_generation(w:&mut Work,index:usize)->Result<()>{
    let g=w.older.remove(index);w.garbage.push(pin_name(&g.id)?);w.garbage.push(progress_name(&g.id)?);
    if let Some(h)=g.plan{w.garbage.push(plan_name(&h)?);}
    if g.catalog!=w.generation.catalog&&!w.older.iter().any(|o|o.catalog==g.catalog){w.garbage.push(format!("pack-catalogs/{}.json",g.catalog));}
    w.garbage.sort();w.garbage.dedup();Ok(())
}
pub(crate) fn referenced(w:&Work,d:&Descriptor,extension:&str)->Result<bool>{
    let path=d.path(extension)?;
    for live in w.generation.packs.iter().chain(w.older.iter().flat_map(|g|&g.packs)).chain(w.jobs.iter().map(|j|&j.descriptor)) {
        if live.path(extension)?==path{return Ok(true);}
    }
    Ok(false)
}
pub(crate) fn add_retired(w:&mut Work,d:Descriptor){
    if !w.retired.iter().any(|r|r.descriptor==d){w.retired.push(Retired{descriptor:d,pack_done:false,index_done:false});}
}
pub(crate) fn garbage_referenced(w:&Work,b:&Binding,name:&str)->Result<bool>{
    if name==pin_name(&b.id)?||name==progress_name(&b.id)?||name==plan_name(&b.plan)?||name==format!("pack-catalogs/{}.json",w.generation.catalog){return Ok(true);}
    for g in &w.older{
        if name==pin_name(&g.id)?||name==progress_name(&g.id)?||name==format!("pack-catalogs/{}.json",g.catalog)||g.plan.as_ref().is_some_and(|h|name==format!("physical-plans/{h}.json")){return Ok(true);}
    }
    for d in w.generation.packs.iter().chain(w.older.iter().flat_map(|g|&g.packs)).chain(w.jobs.iter().map(|j|&j.descriptor)){
        if name==d.path("pack")?||name==d.path("idx")?{return Ok(true);}
    }
    Ok(false)
}
