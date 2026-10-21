//! Bounded live reservations; permanent terminal evidence is point addressed.
use crate::{io::{self,Metrics,Result},model::Selector,store::{Store,Candidate,Publication},objects};
use serde::{Serialize,Deserialize};
use serde_json::{json,Value};
use std::{collections::{BTreeMap,BTreeSet},fs::{self,File,OpenOptions},io::{Read,Write},path::{Path,PathBuf},time::Instant};
const MAX_RESERVATIONS:usize=8;
const MAX_BYTES:u64=32*1024*1024*1024;
const REGISTRY_LIMIT:usize=65536;
const RECEIPT_LIMIT:usize=8192;
const MAX_TERMINAL:u32=4096;
const HANDOFF_MARGIN:u64=128*1024;
const CONTROL_MARGIN:u64=128*1024;
const TERMINAL_REASON:&str="definitive-expected-publication-conflict";
#[derive(Clone,Serialize,Deserialize,PartialEq,Eq)]
#[serde(deny_unknown_fields)]
pub struct Reservation{pub request:String,pub intent:String,pub base:Option<Selector>,pub bytes:u64,pub state:String,pub used:u64,pub candidate_digest:Option<String>}
#[derive(Clone,Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
struct TerminalRegistry{version:u32,store_id:String,active:BTreeMap<String,Reservation>,legacy:BTreeSet<String>,terminal_count:u32,terminal_bytes:u64}
#[derive(Clone)]
enum Registry{V1(BTreeMap<String,Reservation>),V2(TerminalRegistry)}
impl Registry{
    fn active(&self)->&BTreeMap<String,Reservation>{match self{Self::V1(a)=>a,Self::V2(v)=>&v.active}}
    fn active_mut(&mut self)->&mut BTreeMap<String,Reservation>{match self{Self::V1(a)=>a,Self::V2(v)=>&mut v.active}}
    fn validate(&self)->Result<()>{
        let active=self.active();let sum=active.values().try_fold(0u64,|n,r|n.checked_add(r.bytes)).ok_or("quota:overflow")?;
        if active.len()>MAX_RESERVATIONS||sum>MAX_BYTES{return Err("integrity:quota-registry".into());}
        if let Self::V2(v)=self{
            let total=sum.checked_add(v.terminal_bytes).and_then(|n|n.checked_add(CONTROL_MARGIN)).ok_or("quota:overflow")?;
            if v.version!=2||!crate::model::safe_id(&v.store_id)||v.legacy.len()>MAX_RESERVATIONS||v.legacy.iter().any(|a|!active.contains_key(a))||v.terminal_count>MAX_TERMINAL||total>MAX_BYTES||
                (v.terminal_count==0)!=(v.terminal_bytes==0)||v.terminal_bytes<u64::from(v.terminal_count)*HANDOFF_MARGIN||
                active.iter().any(|(a,r)|!crate::model::safe_id(a)||r.request.len()>256||!objects::valid_hash(&r.intent)||r.used>r.bytes||
                    !matches!(r.state.as_str(),"preparing"|"sealed"|"sealed-preselection"|"selection-started"|"abandoned")||
                    v.legacy.contains(a)&&matches!(r.state.as_str(),"sealed-preselection"|"selection-started")){
                return Err("integrity:terminal-registry".into());
            }
        }Ok(())
    }
    fn bytes(&self)->Result<Vec<u8>>{self.validate()?;let b=match self{Self::V1(a)=>io::json(a)?,Self::V2(v)=>io::json(v)?};if b.len()>REGISTRY_LIMIT{return Err("quota:registry-byte-bound".into());}Ok(b)}
    fn decode(bytes:&[u8])->Result<Self>{
        let v=crate::api::strict_json(std::str::from_utf8(bytes)?)?;
        let r=if v.get("version").and_then(Value::as_u64).is_some(){Self::V2(serde_json::from_value(v)?)}else{Self::V1(serde_json::from_value(v)?)};r.validate()?;Ok(r)
    }
}
// Native bounded controls: never capture Q, route through an index or hash a
// candidate. Release uses these exact bytes as its W-held CAS tokens.
fn control(root:&Path,name:&str,limit:usize,m:&mut Metrics)->Result<Option<Vec<u8>>>{
    let path=root.join(name);let mut f=match File::open(&path){Ok(f)=>f,Err(e) if e.kind()==std::io::ErrorKind::NotFound=>return Ok(None),Err(e)=>return Err(e.into())};
    m.io_metadata_stats+=1;let md=f.metadata()?;if !md.is_file(){return Err("integrity:control-file-type".into());}let n=usize::try_from(md.len())?;if n>limit{return Err("integrity:control-byte-bound".into());}
    let mut bytes=vec![0;n];f.read_exact(&mut bytes)?;if f.read(&mut [0])?!=0{return Err("integrity:control-changed".into());}
    m.opens+=1;m.bytes_read+=n as u64;m.record_bytes(&path,n as u64,false);Ok(Some(bytes))
}
fn read_registry(root:&Path,m:&mut Metrics)->Result<(Registry,Option<Vec<u8>>)>{
    let raw=control(root,"RESERVATIONS.json",REGISTRY_LIMIT,m)?;
    let registry=match &raw{Some(b)=>Registry::decode(b)?,None=>Registry::V1(BTreeMap::new())};
    if let Registry::V2(v)=&registry{if crate::packing::captured_config(m)?.store_id!=v.store_id{return Err("integrity:terminal-store-binding".into());}}
    Ok((registry,raw))
}
pub fn reservations(root:&Path,m:&mut Metrics)->Result<BTreeMap<String,Reservation>>{
    Ok(read_registry(root,m)?.0.active().clone())
}
fn selectable(registry:&Registry,attempt:&str,r:&Reservation)->bool{
    match registry{Registry::V1(_)=>r.state=="sealed",Registry::V2(v) if v.legacy.contains(attempt)=>r.state=="sealed",Registry::V2(_)=>sealed(r)}
}
pub(crate) fn selection_owner(root:&Path,attempt:&str,m:&mut Metrics)->Result<Reservation>{
    let (registry,_)=read_registry(root,m)?;let r=registry.active().get(attempt).ok_or("refused:unreserved")?;
    if !selectable(&registry,attempt,r){return Err("integrity:reservation-binding-or-unsealed".into());}Ok(r.clone())
}
fn write_reservations(root:&Path,r:&Registry,m:&mut Metrics)->Result<()>{
    io::atomic(&root.join("RESERVATIONS.json"),&r.bytes()?,m)
}
fn stage_names(attempt:&str)->[String;2]{[format!("attempts/{attempt}/terminal-receipt-stage"),format!("attempts/{attempt}/terminal-control-stage")]}
pub fn reserve(root:&Path,attempt:&str,r:Reservation,m:&mut Metrics)->Result<()>{
    let (mut all,_)=read_registry(root,m)?;
    if all.active().len()>=MAX_RESERVATIONS||all.active().values().map(|r|r.bytes as u128).sum::<u128>()+r.bytes as u128>MAX_BYTES as u128{
        return Err("quota:admission-full".into());
    }
    if matches!(all,Registry::V2(_)){
        if !crate::model::safe_id(attempt){return Err("refused:attempt".into());}
        for name in [format!("abandoned/{attempt}"),format!("attempts/{attempt}"),format!("generations/{attempt}")]{m.io_metadata_stats+=1;if root.join(name).try_exists()?{return Err("integrity:attempt-reuse".into());}}
    }
    if all.active_mut().insert(attempt.into(),r).is_some(){return Err("integrity:attempt-reuse".into());}
    write_reservations(root,&all,m)?;io::checkpoint("reservation-durable")
}
pub fn mark_sealed(root:&Path,attempt:&str,m:&mut Metrics)->Result<()>{
    let (mut all,_)=read_registry(root,m)?;let new_v2=matches!(&all,Registry::V2(v) if !v.legacy.contains(attempt));
    let bytes=io::read(&root.join(format!("attempts/{attempt}/candidate.json")),m)?;
    let ordinary=if new_v2{let c:Candidate=serde_json::from_value(crate::api::strict_json(std::str::from_utf8(&bytes)?)?)?;c.purpose=="publication"&&!c.request.packet&&c.checkpoint.is_none()&&c.packet_ref.is_none()}else{false};
    // The durable NEW preselection phase reserves these two private slots.
    // No file may predate that claim; later partial stages remain attempt-owned.
    if ordinary{for name in stage_names(attempt){m.io_metadata_stats+=1;match fs::symlink_metadata(root.join(name)){Ok(_)=>return Err("refused:preexisting-terminal-stage-at-seal".into()),Err(e) if e.kind()==std::io::ErrorKind::NotFound=>{},Err(e)=>return Err(e.into())}}}
    let r=all.active_mut().get_mut(attempt).ok_or("integrity:missing-reservation")?;
    if new_v2&&r.state!="preparing"{return Err("integrity:seal-state".into());}
    r.state=if ordinary{"sealed-preselection"}else{"sealed"}.into();r.used=m.quota_spent.checked_add(65536).ok_or("quota:overflow")?;
    r.candidate_digest=Some(io::digest(&bytes));
    write_reservations(root,&all,m)?;io::checkpoint("seal")
}
pub fn selection_grant(root:&Path,attempt:&str,m:&mut Metrics)->Result<()>{
    const GRANT:u64=8*1024*1024;
    let (mut all,_)=read_registry(root,m)?;let new_v2=matches!(&all,Registry::V2(v) if !v.legacy.contains(attempt));
    if matches!(all,Registry::V2(_)){m.io_metadata_stats+=1;if root.join(format!("abandoned/{attempt}")).try_exists()?{return Err("refused:fenced-selection-grant".into());}}
    let r=all.active_mut().get_mut(attempt).ok_or("integrity:missing-reservation")?;
    if new_v2{if !sealed(r){return Err("integrity:selection-state".into());}r.state="selection-started".into();}
    r.used=r.used.checked_add(GRANT+65536).ok_or("quota:overflow")?;
    if r.used>r.bytes{return Err("quota:selection-reservation-exhausted".into());}
    m.quota_limit=None;write_reservations(root,&all,m)?;
    if new_v2{io::checkpoint("selection-started-durable")?;}
    m.quota_limit=Some(GRANT);m.quota_spent=0;io::checkpoint("selection-budget")
}
pub(crate) fn sealed(r:&Reservation)->bool{matches!(r.state.as_str(),"sealed"|"sealed-preselection"|"selection-started")}
pub fn transfer(root:&Path,attempt:&str,m:&mut Metrics)->Result<()>{
    io::checkpoint("accounting-transfer")?;
    let (mut all,_)=read_registry(root,m)?;
    if let Registry::V2(v)=&mut all{
        m.io_metadata_stats+=1;if root.join(format!("abandoned/{attempt}")).try_exists()?{return Err("refused:fenced-owner-transfer".into());}
        if v.active.get(attempt).is_some_and(|r|!v.legacy.contains(attempt)&&r.state!="selection-started"){return Err("integrity:transfer-without-selection-grant".into());}
        v.legacy.remove(attempt);
    }
    all.active_mut().remove(attempt);
    write_reservations(root,&all,m)?;io::checkpoint("accounting-release")
}
pub fn recover_transfers(store:&Store,m:&mut Metrics)->Result<()>{
    let all=reservations(&store.root,m)?;
    for (attempt,r) in all{
        if let Some(found)=store.accepted(&r.request,None,m)?{
            // A different attempt accepting the same request does not transfer
            // this attempt's private allocations. Keep those charged.
            if found.attempt==attempt{
                if found.intent!=r.intent{return Err("integrity:allocation-transfer-intent".into());}
                let p=store.member(&found.selected,m)?;
                if p.allocation_owner.attempt!=attempt||p.allocation_owner.conservative_bytes<r.bytes||!p.allocation_owner.includes_intermediates_and_stages{
                    return Err("integrity:allocation-transfer-binding".into());
                }
                transfer(&store.root,&attempt,m)?;
            }
        }
    }Ok(())
}
#[derive(Serialize,Deserialize,PartialEq,Eq)]
#[serde(deny_unknown_fields)]
struct TerminalReceipt{
    version:u32,store_id:String,attempt:String,candidate_digest:String,request:String,intent:String,expected:Selector,
    reason:String,used_bytes:u64,original_allowance:u64,charge_bytes:u64,handoff_margin:u64,control_stage:String,
}
impl TerminalReceipt{
    fn validate(&self,store:&Store,attempt:&str)->Result<()>{
        if self.version!=1||self.store_id!=store.config.store_id||self.attempt!=attempt||!crate::model::safe_id(attempt)||
            !objects::valid_hash(&self.candidate_digest)||!objects::valid_hash(&self.intent)||self.request.len()>256||
            self.reason!=TERMINAL_REASON||self.handoff_margin!=HANDOFF_MARGIN||self.control_stage!=stage_names(attempt)[1]||
            self.used_bytes>self.original_allowance||self.used_bytes.checked_add(HANDOFF_MARGIN)!=Some(self.charge_bytes)||self.charge_bytes>self.original_allowance{
            return Err("integrity:terminal-receipt-binding".into());
        }Ok(())
    }
}
// All ancestor walks/proofs run before W. Stage ownership comes from the exact
// active preselection claim, before creation; final receipts are never repaired.
fn proof_paths(root:&Path,names:&[String],m:&mut Metrics)->Result<()>{
    let mut paths=vec![root.to_path_buf()];for name in names{paths.push(root.join(name));}
    for path in paths{
        for ancestor in path.ancestors(){
            m.io_metadata_stats+=1;match fs::symlink_metadata(ancestor){
                Ok(md)=>{
                    #[cfg(windows)] {use std::os::windows::fs::MetadataExt;if md.file_attributes()&0x400!=0{return Err("refused:terminal-reparse".into());}}
                    if md.file_type().is_symlink(){return Err("refused:terminal-symlink".into());}
                    if ancestor==path.as_path()&&path.as_path()!=root&&!md.is_file(){return Err("refused:terminal-control-file-type".into());}
                },Err(e) if e.kind()==std::io::ErrorKind::NotFound=>{},Err(e)=>return Err(e.into()),
            }
        }
    }Ok(())
}
fn disposable(store:&Store,m:&mut Metrics)->Result<()>{
    if !store.root.components().any(|c|c.as_os_str()=="disposable"){return Err("refused:disposable-root".into());}
    let bytes=control(&store.root,"DISPOSABLE.json",8192,m)?.ok_or("refused:disposable-setup-required")?;
    let allowed:String=serde_json::from_value(crate::api::strict_json(std::str::from_utf8(&bytes)?)?)?;
    m.pack_path_canonicalizations+=1;if allowed!=store.root.canonicalize()?.to_string_lossy(){return Err("refused:disposable-identity".into());}Ok(())
}
fn physical_token(store:&Store,m:&mut Metrics)->Result<Vec<u8>>{
    let raw=control(&store.root,"STORE.json",REGISTRY_LIMIT,m)?.ok_or("integrity:missing-store")?;
    let current:crate::store::StoreConfig=serde_json::from_value(crate::api::strict_json(std::str::from_utf8(&raw)?)?)?;
    if io::json(&current)?!=io::json(&crate::packing::captured_config(m)?)?||current.store_id!=store.config.store_id{return Err("retry:terminal-physical-capture".into());}Ok(raw)
}
fn enable_terminal(store:&Store,v:&Value,m:&mut Metrics)->Result<Value>{
    if v["explicit_disposable_opt_in"]!=true{return Err("refused:explicit-terminal-opt-in-required".into());}
    stage_options(false)?; // Fail before mode mutation on unsupported nofollow hosts.
    let _gate=store.preparation_gate(false)?;crate::packing::capture(&store.root,m)?;
    proof_paths(&store.root,&["RESERVATIONS.json".into(),"STORE.json".into(),"DISPOSABLE.json".into()],m)?;disposable(store,m)?;
    let token=physical_token(store,m)?;let (registry,old)=read_registry(&store.root,m)?;
    let next=match registry{Registry::V1(active)=>{let legacy=active.keys().cloned().collect();Registry::V2(TerminalRegistry{version:2,store_id:store.config.store_id.clone(),active,legacy,terminal_count:0,terminal_bytes:0})},Registry::V2(_)=>return Ok(json!({"status":"terminal-v2-already-enabled","reclaimed_bytes":0}))};
    let bytes=next.bytes()?;io::checkpoint("terminal-enable-proved")?;
    let _writer=store.lock(m)?;
    if control(&store.root,"STORE.json",REGISTRY_LIMIT,m)?.as_ref()!=Some(&token)||control(&store.root,"RESERVATIONS.json",REGISTRY_LIMIT,m)?!=old{return Ok(json!({"status":"deferred","reason":"terminal-enable-control-race","reclaimed_bytes":0}));}
    m.quota_limit=None;io::atomic(&store.root.join("RESERVATIONS.json"),&bytes,m)?;io::checkpoint("terminal-v2-enabled")?;
    Ok(json!({"status":"terminal-v2-enabled","legacy_owners":next.active().len(),"active_cap":MAX_RESERVATIONS,"terminal_cap":MAX_TERMINAL,"reclaimed_bytes":0}))
}
#[derive(Clone,Copy,PartialEq,Eq)]
struct FileIdentity{volume:u64,file:u64}
fn single_file(f:&File,m:&mut Metrics)->Result<FileIdentity>{
    m.io_metadata_stats+=1;let md=f.metadata()?;if !md.is_file(){return Err("refused:terminal-file-type".into());}
    #[cfg(unix)]{use std::os::unix::fs::MetadataExt;if md.nlink()!=1{return Err("refused:terminal-file-links".into());}Ok(FileIdentity{volume:md.dev(),file:md.ino()})}
    #[cfg(windows)]{
        use std::os::windows::io::AsRawHandle;
        use windows_sys::Win32::Storage::FileSystem::{GetFileInformationByHandle,BY_HANDLE_FILE_INFORMATION,FILE_ATTRIBUTE_REPARSE_POINT};
        let mut info:BY_HANDLE_FILE_INFORMATION=unsafe{std::mem::zeroed()};m.io_metadata_stats+=1;
        if unsafe{GetFileInformationByHandle(f.as_raw_handle() as _,&mut info)}==0{return Err(std::io::Error::last_os_error().into());}
        if info.dwFileAttributes&FILE_ATTRIBUTE_REPARSE_POINT!=0{return Err("refused:terminal-reparse".into());}
        if info.nNumberOfLinks!=1{return Err("refused:terminal-file-links".into());}
        Ok(FileIdentity{volume:u64::from(info.dwVolumeSerialNumber),file:u64::from(info.nFileIndexHigh)<<32|u64::from(info.nFileIndexLow)})
    }
    #[cfg(not(any(unix,windows)))]{Err("refused:terminal-file-identity-platform".into())}
}
fn nofollow(o:&mut OpenOptions)->Result<()>{
    #[cfg(windows)]{use std::os::windows::fs::OpenOptionsExt;use windows_sys::Win32::Storage::FileSystem::FILE_FLAG_OPEN_REPARSE_POINT;o.custom_flags(FILE_FLAG_OPEN_REPARSE_POINT);Ok(())}
    // Selected native hosts; ABI values verified against locked libc0.2.191:
    // unix/linux_like/linux/gnu/b64/x86_64 and unix/bsd (Darwin inherits BSD).
    #[cfg(all(target_os="linux",target_arch="x86_64"))]{use std::os::unix::fs::OpenOptionsExt;o.custom_flags(0x20000);Ok(())}
    #[cfg(target_os="macos")]{use std::os::unix::fs::OpenOptionsExt;o.custom_flags(0x100);Ok(())}
    #[cfg(not(any(windows,target_os="macos",all(target_os="linux",target_arch="x86_64"))))]{let _=o;Err("refused:terminal-nofollow-platform".into())}
}
fn stage_options(create:bool)->Result<OpenOptions>{
    let mut o=OpenOptions::new();o.read(true).write(true).create_new(create);
    #[cfg(windows)]{use std::os::windows::fs::OpenOptionsExt;use windows_sys::Win32::Storage::FileSystem::FILE_SHARE_READ;o.share_mode(FILE_SHARE_READ);}
    if !create{nofollow(&mut o)?;}
    #[cfg(not(any(windows,target_os="macos",all(target_os="linux",target_arch="x86_64"))))]return Err("refused:terminal-nofollow-platform".into());
    Ok(o)
}
struct Stage{path:PathBuf,file:Option<File>,identity:Option<FileIdentity>,limit:usize}
fn prove_stage(root:&Path,name:&str,allowed:bool,limit:usize,m:&mut Metrics)->Result<Stage>{
    let path=root.join(name);m.io_metadata_stats+=1;
    let md=match fs::symlink_metadata(&path){Ok(md)=>Some(md),Err(e) if e.kind()==std::io::ErrorKind::NotFound=>None,Err(e)=>return Err(e.into())};
    let (file,identity)=if let Some(md)=md{
        if !allowed{return Err("refused:unowned-terminal-stage".into());}
        if !md.is_file()||md.file_type().is_symlink()||md.len()>limit as u64{return Err("refused:terminal-stage-type-size".into());}
        let f=stage_options(false)?.open(&path)?;m.opens+=1;let identity=single_file(&f,m)?;
        #[cfg(unix)]{use std::os::unix::fs::MetadataExt;if identity.volume!=md.dev()||identity.file!=md.ino(){return Err("retry:terminal-stage-identity".into());}}
        fs2::FileExt::try_lock_exclusive(&f).map_err(|e|format!("busy:terminal-stage:{e}"))?;
        (Some(f),Some(identity))
    }else{(None,None)};
    Ok(Stage{path,file,identity,limit})
}
fn write_stage(mut stage:Stage,bytes:&[u8],phase:&str,m:&mut Metrics)->Result<Option<FileIdentity>>{
    if bytes.is_empty()||bytes.len()>stage.limit{return Err("integrity:terminal-stage-write-bound".into());}
    m.charge(bytes.len() as u64+4096)?;
    let already_locked=stage.file.is_some();let mut f=if let Some(f)=stage.file.take(){f}else{
        match stage_options(true)?.open(&stage.path){Ok(f)=>{m.opens+=1;m.namespace_ops+=1;f},Err(e) if e.kind()==std::io::ErrorKind::AlreadyExists=>return Ok(None),Err(e)=>return Err(e.into())}
    };
    let identity=single_file(&f,m)?;if stage.identity.is_some_and(|old|old!=identity){return Err("retry:terminal-stage-identity".into());}
    m.io_metadata_stats+=1;if f.metadata()?.len()>stage.limit as u64{return Err("refused:terminal-stage-size".into());}
    if !already_locked{fs2::FileExt::try_lock_exclusive(&f).map_err(|e|format!("busy:terminal-stage:{e}"))?;}
    io::checkpoint(&format!("{phase}-created"))?;
    // Crucially, validate the OPENED handle before truncating any existing file.
    let started=Instant::now();f.set_len(0)?;io::checkpoint(&format!("{phase}-before-write"))?;
    let split=(bytes.len()/2).max(1).min(bytes.len());f.write_all(&bytes[..split])?;m.bytes_written+=split as u64;m.record_bytes(&stage.path,split as u64,true);
    io::checkpoint(&format!("{phase}-partial"))?;
    f.write_all(&bytes[split..])?;m.bytes_written+=(bytes.len()-split) as u64;m.record_bytes(&stage.path,(bytes.len()-split) as u64,true);
    io::checkpoint(&format!("{phase}-write"))?;if phase=="terminal-receipt-stage"{io::checkpoint("file-write")?;}
    f.sync_all()?;m.file_syncs+=1;m.elapsed("terminal-stage-write-sync",started);single_file(&f,m)?;drop(f);io::sync_dir(stage.path.parent().ok_or("stage-parent")?,m)?;
    io::checkpoint(&format!("{phase}-staged"))?;Ok(Some(identity))
}
fn receipt_control(root:&Path,name:&str,m:&mut Metrics)->Result<Option<Vec<u8>>>{
    let mut o=OpenOptions::new();o.read(true);nofollow(&mut o)?;
    let path=root.join(name);let mut f=match o.open(&path){Ok(f)=>f,Err(e) if e.kind()==std::io::ErrorKind::NotFound=>return Ok(None),Err(e)=>return Err(e.into())};
    m.opens+=1;single_file(&f,m)?;m.io_metadata_stats+=1;let n=usize::try_from(f.metadata()?.len())?;if n>RECEIPT_LIMIT{return Err("integrity:terminal-receipt-byte-bound".into());}
    let mut bytes=vec![0;n];f.read_exact(&mut bytes)?;if f.read(&mut [0])?!=0{return Err("integrity:terminal-receipt-changed".into());}m.bytes_read+=n as u64;m.record_bytes(&path,n as u64,false);Ok(Some(bytes))
}
fn terminal_control(store:&Store,stage:Stage,bytes:&[u8],m:&mut Metrics)->Result<bool>{
    let path=stage.path.clone();if write_stage(stage,bytes,"terminal-control",m)?.is_none(){return Ok(false);}
    io::replace(&path,&store.root.join("RESERVATIONS.json"),m)?;
    io::checkpoint("terminal-control-visible")?;io::sync_dir(&store.root,m)?;io::sync_dir(path.parent().ok_or("stage-parent")?,m)?;io::checkpoint("terminal-control-durable")?;Ok(true)
}
fn release_conflict(store:&Store,v:&Value,m:&mut Metrics)->Result<Value>{
    let attempt=v["attempt"].as_str().ok_or("attempt")?;if !crate::model::safe_id(attempt){return Err("refused:attempt".into());}
    let _gate=store.preparation_gate(false)?;crate::packing::capture(&store.root,m)?;
    let candidate_path=format!("attempts/{attempt}/candidate.json");let receipt_path=format!("abandoned/{attempt}");let [receipt_stage_path,stage_path]=stage_names(attempt);
    proof_paths(&store.root,&[candidate_path.clone(),receipt_path.clone(),receipt_stage_path.clone(),stage_path.clone(),"RESERVATIONS.json".into(),"SELECTED.json".into(),"STORE.json".into(),"DISPOSABLE.json".into()],m)?;
    disposable(store,m)?;let physical=physical_token(store,m)?;
    let selected=control(&store.root,"SELECTED.json",REGISTRY_LIMIT,m)?.ok_or("integrity:missing-selected")?;
    let selected_id:Selector=serde_json::from_value(crate::api::strict_json(std::str::from_utf8(&selected)?)?)?;
    let (registry,old)=read_registry(&store.root,m)?;let mut next=match registry{Registry::V2(r)=>r,Registry::V1(_)=>return Err("refused:terminal-v2-not-enabled".into())};
    let candidate=io::read_bounded(&store.root.join(&candidate_path),128*1024*1024,m)?;
    let digest=io::digest(&candidate);let c:Candidate=serde_json::from_value(crate::api::strict_json(std::str::from_utf8(&candidate)?)?)?;
    if c.version!=2||c.purpose!="publication"||c.store_id!=store.config.store_id||c.attempt!=attempt||c.request.packet||c.checkpoint.is_some()||c.packet_ref.is_some()||io::digest(&io::json(&c.request)?)!=c.intent{return Err("integrity:terminal-candidate-binding".into());}
    // None first: an intent mismatch must never be interpreted as absence.
    if let Some(found)=store.accepted(&c.request.request,None,m)?{
        if found.attempt==attempt{return Err("refused:selected-attempt-resolve".into());}
        if next.active.contains_key(attempt){return Err("refused:other-attempt-accepted".into());}
        // A later attempt accepting the logical request cannot own this exact
        // already-committed terminal receipt. Verify that receipt below.
    }
    let expected=c.expected.as_ref().ok_or("refused:terminal-initial-candidate")?;
    let head=store.head_unleased(m)?;if head.selected!=selected_id{return Ok(json!({"status":"deferred","reason":"terminal-proof-head-race","reclaimed_bytes":0}));}
    let past=store.member(expected,m)?;
    if expected==&selected_id||past.sequence>=head.publication.sequence{return Err("refused:terminal-base-not-historical".into());}
    let existing=receipt_control(&store.root,&receipt_path,m)?;
    let (receipt,idempotent)=if let Some(r)=next.active.get(attempt){
        if next.legacy.contains(attempt)||r.state!="sealed-preselection"||r.request!=c.request.request||r.intent!=c.intent||r.base!=c.expected||r.candidate_digest.as_deref()!=Some(digest.as_str()){return Err("refused:terminal-not-new-preselection-owner".into());}
        let charge=r.used.checked_add(HANDOFF_MARGIN).ok_or("quota:overflow")?;
        let receipt=TerminalReceipt{version:1,store_id:store.config.store_id.clone(),attempt:attempt.into(),candidate_digest:digest.clone(),request:r.request.clone(),intent:r.intent.clone(),expected:expected.clone(),reason:TERMINAL_REASON.into(),used_bytes:r.used,original_allowance:r.bytes,charge_bytes:charge,handoff_margin:HANDOFF_MARGIN,control_stage:stage_path.clone()};
        receipt.validate(store,attempt)?;
        next.terminal_count=next.terminal_count.checked_add(1).ok_or("quota:overflow")?;next.terminal_bytes=next.terminal_bytes.checked_add(charge).ok_or("quota:overflow")?;next.active.remove(attempt);
        (receipt,false)
    }else{
        let raw=existing.as_ref().ok_or("refused:no-terminal-owner")?;let receipt:TerminalReceipt=serde_json::from_value(crate::api::strict_json(std::str::from_utf8(raw)?)?)?;
        receipt.validate(store,attempt)?;
        if receipt.candidate_digest!=digest||receipt.request!=c.request.request||receipt.intent!=c.intent||receipt.expected!=*expected||next.terminal_count==0||next.terminal_bytes<receipt.charge_bytes{return Err("integrity:terminal-idempotency-binding".into());}
        (receipt,true)
    };
    let next=Registry::V2(next);let replacement=next.bytes()?;let receipt_bytes=io::json(&receipt)?;
    if receipt_bytes.len()>RECEIPT_LIMIT{return Err("quota:terminal-receipt-byte-bound".into());}
    if existing.as_ref().is_some_and(|b|*b!=receipt_bytes){return Err("integrity:terminal-receipt-exact-bytes".into());}
    let receipt_hash=io::digest(&receipt_bytes);
    // Both fixed slots were absent before this exact NEW preselection owner
    // became durable. That active claim, not a filename/hash, owns partials.
    let receipt_stage=if !idempotent{Some(prove_stage(&store.root,&receipt_stage_path,true,RECEIPT_LIMIT,m)?)}else{None};
    let control_stage=if !idempotent{Some(prove_stage(&store.root,&stage_path,true,REGISTRY_LIMIT,m)?)}else{None};
    // This bounded directory preparation is outside W and creates no receipt.
    if !idempotent{io::mkdir(&store.root.join("abandoned"),m)?;}
    io::checkpoint("terminal-proved")?;
    let _writer=store.lock(m)?;
    if control(&store.root,"RESERVATIONS.json",REGISTRY_LIMIT,m)?!=old||control(&store.root,"SELECTED.json",REGISTRY_LIMIT,m)?.as_ref()!=Some(&selected)||control(&store.root,"STORE.json",REGISTRY_LIMIT,m)?.as_ref()!=Some(&physical){return Ok(json!({"status":"deferred","reason":"terminal-control-race","reclaimed_bytes":0}));}
    // A different preproof may have observed None before a cooperating writer
    // published a complete receipt and died with the SAME registry token.
    let actual=receipt_control(&store.root,&receipt_path,m)?;
    if actual.as_ref().is_some_and(|b|*b!=receipt_bytes){return Err("integrity:terminal-receipt-exact-bytes".into());}
    if idempotent&&actual.is_none(){return Err("integrity:missing-terminal-receipt".into());}
    if !idempotent{
        m.quota_limit=None;io::checkpoint("terminal-before-receipt")?;
        if actual.is_none(){
            if write_stage(receipt_stage.unwrap(),&receipt_bytes,"terminal-receipt-stage",m)?.is_none(){return Ok(json!({"status":"deferred","reason":"terminal-receipt-stage-race","reclaimed_bytes":0}));}
            io::checkpoint("terminal-receipt-before-publish")?;
            // W serializes all final-fence writers; actual absence was checked
            // above. Only full synced stage bytes become the immutable final.
            io::replace(&store.root.join(&receipt_stage_path),&store.root.join(&receipt_path),m)?;
            io::checkpoint("terminal-receipt-published")?;
            io::sync_dir(store.root.join(&receipt_stage_path).parent().ok_or("stage-parent")?,m)?;
        }else{drop(receipt_stage);io::sync_file(&store.root.join(&receipt_path),m)?;}
        io::sync_dir(&store.root.join("abandoned"),m)?;io::checkpoint("terminal-receipt-durable")?;
        if !terminal_control(store,control_stage.unwrap(),&replacement,m)?{return Ok(json!({"status":"deferred","reason":"terminal-control-stage-race","reclaimed_bytes":0}));}
    }
    io::checkpoint("terminal-reply")?;
    Ok(json!({"status":if idempotent{"terminal-already-charged"}else{"terminal-charged"},"attempt":attempt,"receipt_sha256":receipt_hash,"charge_bytes":receipt.charge_bytes,"unused_allowance_released":receipt.original_allowance-receipt.charge_bytes,"reclaimed_bytes":0}))
}
pub fn live(root:&Path,selector:&Selector)->Result<()>{
    if root.join(format!("retired/{}.json",selector.publication)).exists(){return Err("retired:snapshot".into());}Ok(())
}
fn pins(root:&Path,m:&mut Metrics)->Result<BTreeMap<String,Selector>>{
    let p=root.join("PINS.json");
    if !p.exists(){return Ok(BTreeMap::new());}
    Ok(serde_json::from_slice(&io::read_bounded(&p,128*1024,m)?)?)
}
pub fn files(dir:&Path,out:&mut Vec<PathBuf>)->Result<()>{
    if !dir.exists(){return Ok(());}
    for e in io::read_dir(dir)?{
        let p=e?.path();let md=fs::symlink_metadata(&p)?;
        if md.file_type().is_symlink(){return Err("refused:symlink".into());}
        if md.is_dir(){files(&p,out)?;}else{out.push(p);}
    }Ok(())
}
pub fn remove_checked(root:&Path,path:&Path,m:&mut Metrics)->Result<u64>{
    m.pack_path_canonicalizations+=1;let base=root.canonicalize()?;m.pack_path_canonicalizations+=1;let resolved=path.canonicalize()?;
    if resolved==base||!resolved.starts_with(&base){return Err("refused:unsafe-delete".into());}
    m.pack_metadata_stats+=1;if fs::symlink_metadata(path)?.file_type().is_symlink(){
        return Err("refused:unsafe-delete".into());
    }
    m.pack_metadata_stats+=1;let n=path.metadata()?.len();io::forbidden();fs::remove_file(path)?;io::sync_dir(path.parent().unwrap(),m)?;m.namespace_ops+=1;Ok(n)
}
pub fn run(store:&Store,v:&Value,m:&mut Metrics)->Result<Value>{
    let op=v["operation"].as_str().ok_or("operation")?;
    if op=="enable-terminal-v2"{return enable_terminal(store,v,m);}
    if op=="release-definitive-conflict"{return release_conflict(store,v,m);}
    if op=="collect"{
        if matches!(read_registry(&store.root,m)?.0,Registry::V2(_)){return Err("refused:terminal-v2-collect-unimplemented".into());}
        // P before W closes the v1-entry/opt-in race. Opt-in owns shared P;
        // no mode conversion can cross this second check while P is exclusive.
        let gate=store.preparation_gate(true)?;
        if matches!(read_registry(&store.root,m)?.0,Registry::V2(_)){return Err("refused:terminal-v2-collect-unimplemented".into());}
        // Never wait for W while owning exclusive P: refresh/checkpoint can
        // already own W and need shared P. Contention drops P on return.
        let _writer=store.try_writer(m)?.ok_or("busy:collect-writer")?;return collect(store,m,gate);
    }
    let _guard=store.lock(m)?;let root=&store.root;
    let head=store.head_unleased(m)?;
    if op=="quota"{let (registry,_)=read_registry(root,m)?;return Ok(match registry{Registry::V1(a)=>json!({"reservations":a}),Registry::V2(v)=>json!({"registry_version":2,"reservations":v.active,"terminal_count":v.terminal_count,"terminal_bytes":v.terminal_bytes,"control_margin":CONTROL_MARGIN,"reclaimed_bytes":0})});}
    if op=="setup"{
        if !root.components().any(|c|c.as_os_str()=="disposable"){return Err("refused:disposable-root".into());}
        io::atomic(&root.join("DISPOSABLE.json"),&io::json(&root.canonicalize()?.to_string_lossy())?,m)?;
        return Ok(json!({"status":"disposable"}));
    }
    if op=="abandon"{
        let gate=store.preparation_gate(true)?;
        let attempt=v["attempt"].as_str().ok_or("attempt")?;
        if !crate::model::safe_id(attempt){return Err("refused:attempt".into());}
        let (mut all,_)=read_registry(root,m)?;
        if matches!(all,Registry::V2(_)){if let Some(bytes)=control(root,&format!("abandoned/{attempt}"),RECEIPT_LIMIT,m)?{if bytes!=b"fenced"{return Err("refused:terminal-receipt-abandon".into());}}}
        let r=all.active_mut().get_mut(attempt).ok_or("refused:no-reservation")?;
        if store.accepted(&r.request,None,m)?.is_some_and(|a|a.attempt==attempt){return Err("refused:selected-attempt".into());}
        io::atomic(&root.join(format!("abandoned/{attempt}")),b"fenced",m)?;io::checkpoint("abandonment-fenced")?;
        r.state="abandoned".into();write_reservations(root,&all,m)?;drop(gate);
        return Ok(json!({"status":"abandoned-charged-until-gc"}));
    }
    let selected:Selector=if v["selected"].is_object(){serde_json::from_value(v["selected"].clone())?}else{head.selected.clone()};
    store.member(&selected,m)?;
    let mut p=pins(root,m)?;
    match op{
        "pin"=>{
            live(root,&selected)?;let name=v["name"].as_str().ok_or("name")?;
            if !crate::model::safe_id(name)||p.len()>=256&&!p.contains_key(name){return Err("refused:pin-limit".into());}
            if p.get(name).is_some_and(|old|old!=&selected){return Err("refused:pin-rebind".into());}
            p.insert(name.into(),selected);io::atomic(&root.join("PINS.json"),&io::json(&p)?,m)?;
            io::checkpoint("pin-registered")?;Ok(json!({"status":"pinned"}))
        },
        "release"=>{
            p.remove(v["name"].as_str().ok_or("name")?);io::atomic(&root.join("PINS.json"),&io::json(&p)?,m)?;
            io::checkpoint("pin-release")?;Ok(json!({"status":"released"}))
        },
        "retire"=>{
            let _readers=store.preparation_gate(true)?;
            if selected==head.selected||p.values().any(|s|s==&selected)||crate::projection::protects(store,&selected,m)?{
                return Err("refused:protected-snapshot".into());
            }
            for r in reservations(root,m)?.values(){if r.state!="abandoned"&&r.base.as_ref()==Some(&selected){return Err("refused:candidate-base".into());}}
            io::atomic(&root.join(format!("retired/{}.json",selected.publication)),&io::json(&selected)?,m)?;
            io::checkpoint("retirement-durable")?;Ok(json!({"status":"retired"}))
        },
        _=>Err("unknown-lifecycle".into())
    }
}
fn collect(store:&Store,m:&mut Metrics,_gate:File)->Result<Value>{
    let root=&store.root;
    let allowed:String=serde_json::from_slice(&io::read(&root.join("DISPOSABLE.json"),m)?)?;
    if allowed!=root.canonicalize()?.to_string_lossy(){return Err("refused:disposable-identity".into());}
    recover_transfers(store,m)?;
    let head=store.head_unleased(m)?;let mut keep=BTreeSet::new();
    let mut o=objects::object_store(root,m);
    for r in [&head.publication.membership,&head.publication.outcomes,&head.publication.changes]{
        keep.extend(objects::reachable(&mut o,r)?);
    }
    drop(o);
    let mut contents=vec![];let mut evidence=vec![head.publication.clone()];
    for (_,r) in store.membership_entries(m)?{evidence.push(store.publication(&r,m)?);}
    for p in &evidence{if live(root,&p.selector()?).is_ok(){contents.push((p.graph.clone(),p.projection.clone()));}}
    contents.extend(crate::projection::protected_roots(store,m)?);
    if let Some(changes)=crate::projection::pending_changes(store,m)?{
        let mut o=objects::object_store(root,m);keep.extend(objects::reachable(&mut o,&changes)?);
    }
    let mut keep_files=BTreeSet::new();
    let all=reservations(root,m)?;
    let mut abandoned=vec![];
    for (attempt,r) in &all{
        if r.state=="abandoned"||root.join(format!("abandoned/{attempt}")).exists(){abandoned.push(attempt.clone());continue;}
        let path=root.join(format!("attempts/{attempt}/candidate.json"));
        if r.state!="sealed"||!path.exists(){return Err("refused:uncharged-or-unsealed-root-unknown".into());}
        let bytes=io::read(&path,m)?;
        if r.candidate_digest.as_deref()!=Some(io::digest(&bytes).as_str()){return Err("integrity:sealed-candidate-digest".into());}
        let c:Candidate=serde_json::from_slice(&bytes)?;
        if c.version!=2||c.attempt!=*attempt||c.intent!=r.intent||c.request.request!=r.request||c.expected!=r.base||!["publication","packet"].contains(&c.purpose.as_str()){
            return Err("integrity:sealed-candidate-binding".into());
        }
        {
            let mut o=objects::object_store(root,m);
            for id in &c.replay_objects{o.read(id)?;keep.insert(id.clone());}
        }
        for (path,hash) in &c.replay_files{
            if !(path.starts_with("originals/")||path.starts_with("derived/"))||Path::new(path).components().any(|p|!matches!(p,std::path::Component::Normal(_))){return Err("integrity:replay-file-path".into());}
            io::checked(&root.join(path),hash,m)?;keep_files.insert(root.join(path));
        }
        contents.push((c.graph.clone(),c.projection.clone()));
        if let Some(base)=&c.expected{
            let p=store.publication(base,m)?;
            contents.push((p.graph,p.projection));
            if c.purpose=="publication"{
                let mut o=objects::object_store(root,m);
                for r in [&p.membership,&p.outcomes,&p.changes]{keep.extend(objects::reachable(&mut o,r)?);}
            }
        }
        let mut o=objects::object_store(root,m);
        keep.extend(objects::reachable(&mut o,&c.changes)?);
        if let Some(cp)=&c.checkpoint{keep.extend(objects::reachable(&mut o,&cp.membership)?);keep.extend(objects::reachable(&mut o,&cp.outcomes)?);}
    }
    let content_visits=contents.len();
    let mut unique_contents=BTreeSet::new();
    for (g,projection) in contents{
        if !unique_contents.insert((g.tree.clone(),projection.clone())){continue;}
        let mut o=objects::object_store(root,m);
        keep.extend(objects::reachable(&mut o,&g.tree)?);keep.extend(objects::reachable(&mut o,&projection)?);
        for (_,h) in objects::range::<crate::graph::Header>(&mut o,&g.tree,"D/")?{keep_files.insert(root.join(h.doc.original));}
        for (_,b) in objects::entries(&mut o,&projection)?{
            let f:crate::projection::FileSpec=objects::decode(&b,&mut o)?;keep_files.insert(root.join(f.source));
        }
    }
    let mut paths=vec![];for name in ["objects","originals","derived"]{files(&root.join(name),&mut paths)?;}
    let mut removed=0;let mut bytes=0;
    for path in paths{
        let is_object=path.starts_with(root.join("objects"));
        let retained=if is_object{keep.contains(path.file_name().unwrap().to_string_lossy().as_ref())}else{keep_files.contains(&path)};
        if !retained{bytes+=remove_checked(root,&path,m)?;removed+=1;io::checkpoint("reclaim-file")?;}
    }
    // Atomic control replacement may die with a private stage. Both writer and
    // exclusive preparation gates are held, so no cooperating operation uses it.
    for dir in [root.clone(),root.join("attempt-hints"),root.join("abandoned"),root.join("fences"),root.join("retired"),root.join("cancel-signals")]{
        if !dir.exists(){continue;}
        for entry in io::read_dir(dir)?{
            let path=entry?.path();
            if path.file_name().unwrap().to_string_lossy().starts_with("stage-")&&path.is_file(){
                bytes+=remove_checked(root,&path,m)?;removed+=1;
            }
        }
    }
    // Readable copies/attempt staging for retired or fenced work have independent
    // lifetime. Accepted publication/result evidence lives outside these trees.
    let packet_owners:BTreeSet<_>=evidence.iter().filter(|p|live(root,&p.selector().unwrap()).is_ok()).filter_map(|p|p.packet_ref.as_ref().map(|r|r.attempt.clone())).collect();
    for p in &evidence{
        if live(root,&p.selector()?).is_err(){
            for name in [format!("generations/{}",p.attempt),format!("attempts/{}",p.attempt)]{
                if name.starts_with("attempts/")&&packet_owners.contains(&p.attempt){continue;}
                let mut paths=vec![];files(&root.join(name),&mut paths)?;
                for path in paths{bytes+=remove_checked(root,&path,m)?;removed+=1;io::checkpoint("reclaim-file")?;}
            }
        }
    }
    // Unselected result/publication evidence and selector stages belong to the
    // allocating attempt, including death before publication installation.
    let accepted:BTreeSet<_>=evidence.iter().map(|p|p.id.clone()).collect();
    for attempt in abandoned{
        let mut orphan_results=BTreeSet::new();
        let mut owned=vec![];files(&root.join(format!("attempts/{attempt}/allocations")),&mut owned)?;
        for path in owned{
            if path.extension().is_none_or(|e|e!="json"){continue;}
            let record:serde_json::Value=serde_json::from_slice(&io::read(&path,m)?)?;
            let id=record["publication"].as_str().ok_or("integrity:allocation-record")?;
            let hash=record["result"].as_str().ok_or("integrity:allocation-record")?;
            if !crate::model::safe_id(id)||!objects::valid_hash(hash){return Err("integrity:allocation-record".into());}
            if accepted.contains(id){return Err("integrity:abandoned-selected-allocation".into());}
            let p=root.join(format!("publications/{id}.json"));
            if p.exists(){bytes+=remove_checked(root,&p,m)?;removed+=1;}
            orphan_results.insert(hash.to_owned());
        }
        for hash in orphan_results{
            if evidence.iter().any(|p|p.result_hash==hash){continue;}
            let p=root.join(format!("results/{hash}.json"));
            if p.exists(){bytes+=remove_checked(root,&p,m)?;removed+=1;}
        }
        let hint=root.join(format!("attempt-hints/{attempt}.json"));
        if hint.exists(){bytes+=remove_checked(root,&hint,m)?;removed+=1;}
        for name in [format!("attempts/{attempt}"),format!("generations/{attempt}")]{
            let mut paths=vec![];files(&root.join(name),&mut paths)?;
            for path in paths{bytes+=remove_checked(root,&path,m)?;removed+=1;io::checkpoint("reclaim-file")?;}
        }
        io::checkpoint("reclaim-accounting")?;transfer(root,&attempt,m)?;
    }
    Ok(json!({"status":"collected","files":removed,"logical_bytes":bytes,"accepted_evidence_preserved":true,"content_root_visits":content_visits,"unique_content_roots":unique_contents.len()}))
}

#[cfg(test)]
mod tests{
    use super::*;
    fn owner()->Reservation{Reservation{request:"request".into(),intent:"a".repeat(64),base:None,bytes:64*1024*1024,state:"preparing".into(),used:0,candidate_digest:None}}
    fn v2()->TerminalRegistry{TerminalRegistry{version:2,store_id:"store-test".into(),active:BTreeMap::new(),legacy:BTreeSet::new(),terminal_count:0,terminal_bytes:0}}
    #[test]fn v1_bytes_and_old_parser_boundary_are_preserved(){
        let mut original=BTreeMap::new();original.insert("attempt-legacy".to_string(),owner());original.insert("version".to_string(),owner());let bytes=io::json(&original).unwrap();let decoded=Registry::decode(&bytes).unwrap();assert!(matches!(&decoded,Registry::V1(_)));assert_eq!(decoded.bytes().unwrap(),bytes);
        let mut current=v2();current.active=original.clone();current.legacy.extend(original.keys().cloned());let bytes=Registry::V2(current).bytes().unwrap();assert!(serde_json::from_slice::<BTreeMap<String,Reservation>>(&bytes).is_err());
        let value:Value=serde_json::from_slice(&bytes).unwrap();assert_eq!(value["active"],serde_json::to_value(original).unwrap());let mut bad=value.clone();bad["extra"]=json!(true);assert!(Registry::decode(&serde_json::to_vec(&bad).unwrap()).is_err());assert!(Registry::decode(b"{\"version\":2,\"version\":2}").is_err());
    }
    #[test]fn terminal_caps_and_charge_are_independent_of_live_slots(){
        let mut state=v2();for n in 0..8{state.active.insert(format!("attempt-{n}"),owner());}state.terminal_count=2400;state.terminal_bytes=2400*(HANDOFF_MARGIN+65536);assert!(Registry::V2(state.clone()).validate().is_ok());
        state.active.insert("ninth".into(),owner());assert!(Registry::V2(state.clone()).validate().is_err());state.active.remove("ninth");state.terminal_count=MAX_TERMINAL+1;assert!(Registry::V2(state.clone()).validate().is_err());state.terminal_count=2400;state.terminal_bytes=u64::MAX;assert!(Registry::V2(state.clone()).validate().is_err());
        state.terminal_bytes=MAX_BYTES;assert!(Registry::V2(state.clone()).validate().is_err());state.terminal_count=0;state.terminal_bytes=1;assert!(Registry::V2(state).validate().is_err());
    }
    #[test]fn legacy_owner_cannot_acquire_new_preselection_proof(){
        let mut state=v2();let mut r=owner();r.state="sealed".into();state.active.insert("legacy".into(),r);state.legacy.insert("legacy".into());assert!(Registry::V2(state.clone()).validate().is_ok());
        for phase in ["sealed","sealed-preselection","selection-started"]{
            let mut r=owner();r.state=phase.into();r.candidate_digest=Some("c".repeat(64));let mut old:BTreeMap<String,Reservation>=BTreeMap::new();old.insert("legacy".into(),r.clone());let old=Registry::decode(&io::json(&old).unwrap()).unwrap();assert_eq!(selectable(&old,"legacy",old.active().get("legacy").unwrap()),phase=="sealed");
            assert_eq!(selectable(&Registry::V2(state.clone()),"legacy",&r),phase=="sealed");let mut current=state.clone();current.active.insert("new".into(),r);let current=Registry::decode(&Registry::V2(current).bytes().unwrap()).unwrap();assert!(selectable(&current,"new",current.active().get("new").unwrap()));
        }
        state.active.get_mut("legacy").unwrap().state="sealed-preselection".into();assert!(Registry::V2(state.clone()).validate().is_err());state.active.get_mut("legacy").unwrap().state="selection-started".into();assert!(Registry::V2(state).validate().is_err());
    }
    #[test]fn stage_handle_aliases_refuse_before_truncation(){
        let root=PathBuf::from(std::env::var_os("EXP_PACK_TEST_ROOT").expect("explicit fault fixture root required")).join(io::unique("stage-handle-fault")).join("disposable/store");
        fs::create_dir_all(root.join("attempts/owned")).unwrap();fs::create_dir_all(root.join("abandoned")).unwrap();let target=root.join("SELECTED.json");let original=b"tiny owned authority canary";fs::write(&target,original).unwrap();let mut m=Metrics::default();
        // Deliberate fault links only within this NEW tiny disposable fixture.
        // Every name remains retained and is fully charged; no dedup credit.
        for name in stage_names("owned"){fs::hard_link(&target,root.join(&name)).unwrap();let error=prove_stage(&root,&name,true,REGISTRY_LIMIT,&mut m).err().unwrap();assert!(error.to_string().contains("terminal-file-links"));assert_eq!(fs::read(&target).unwrap(),original);}
        fs::hard_link(&target,root.join("abandoned/owned")).unwrap();assert!(receipt_control(&root,"abandoned/owned",&mut m).err().unwrap().to_string().contains("terminal-file-links"));assert_eq!(fs::read(&target).unwrap(),original);
        let name="attempts/owned/singleton";fs::write(root.join(name),b"old").unwrap();let stage=prove_stage(&root,name,true,RECEIPT_LIMIT,&mut m).unwrap();assert!(write_stage(stage,b"new owned bytes","test-stage",&mut m).unwrap().is_some());assert_eq!(fs::read(root.join(name)).unwrap(),b"new owned bytes");
        let name="attempts/owned/appeared";let stage=prove_stage(&root,name,true,RECEIPT_LIMIT,&mut m).unwrap();fs::write(root.join(name),b"unexpected").unwrap();assert!(write_stage(stage,b"must not overwrite","test-stage",&mut m).unwrap().is_none());assert_eq!(fs::read(root.join(name)).unwrap(),b"unexpected");
    }
}
