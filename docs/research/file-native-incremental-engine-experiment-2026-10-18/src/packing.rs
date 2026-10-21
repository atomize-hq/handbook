//! Physical locations only. Semantic selection and content lifetime stay in Store.
use crate::{io::{self,Metrics,Result},pack::{self,Descriptor,Index,Key},store::Store,lifetime};
use fs2::FileExt;
use crate::physical_lifetime as physical;
use serde::{Serialize,Deserialize};
use serde_json::{json,Value};
use sha2::{Digest,Sha256};
use std::{collections::{BTreeMap,BTreeSet},fs::{File,OpenOptions},path::{Path,PathBuf},rc::Rc,time::{Duration,Instant}};

#[derive(Clone,Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
struct Catalog { version:u32,store_id:String,packs:Vec<Descriptor> }
pub(crate) struct PhysicalRead {
    _lease:File,root:PathBuf,config:crate::store::StoreConfig,catalog:Option<Catalog>,cache:BTreeMap<String,Rc<Index>>,cache_bytes:usize,borrowed_index_bytes:usize,constructor_index_bytes:usize,construction_aux_bytes:usize,
}
fn strict<T:serde::de::DeserializeOwned>(bytes:&[u8])->Result<T>{Ok(serde_json::from_value(crate::api::strict_json(std::str::from_utf8(bytes)?)?)?)}
fn config(root:&Path,m:&mut Metrics)->Result<crate::store::StoreConfig>{physical::config(root,m)}
pub(crate) fn captured_config(m:&Metrics)->Result<crate::store::StoreConfig>{
    Ok(m.physical.as_ref().ok_or("integrity:physical-capture")?.config.clone())
}
fn catalog(root:&Path,c:&crate::store::StoreConfig,m:&mut Metrics)->Result<Option<Catalog>> {
    match (c.version,c.catalog.as_deref()) {
        (1,None)=>Ok(None),
        (2|3,Some(hash)) if crate::objects::valid_hash(hash)=>{
            let limit=if c.version==3{physical::CATALOG_LIMIT}else{65536};
            m.cleanup_step()?;let b=pack::native(&root.join(format!("pack-catalogs/{hash}.json")),limit,m)?;
            if io::digest(&b)!=hash{return Err("integrity:catalog-digest".into());}
            m.pack_catalog_bytes+=b.len() as u64;
            let cat:Catalog=strict(&b)?;
            if !(if c.version==3{matches!(cat.version,2|3|4)}else{cat.version==1})||cat.store_id!=c.store_id||cat.packs.len()>64{return Err("integrity:catalog-version-count".into());}
            let mut previous:Option<&Descriptor>=None;let mut hashes=BTreeSet::new();let mut segments=BTreeSet::new();
            for d in &cat.packs {
                if d.page.is_some(){if cat.version!=4{return Err("integrity:paged-catalog-version".into());}crate::paged::validate(d)?;}
                if c.version==2&&d.instance.is_some()||c.version==3&&d.instance.is_none()&&cat.version<3||!crate::objects::valid_hash(&d.pack)||!crate::objects::valid_hash(&d.index)||d.pack_bytes>pack::MAX_PACK as u64||d.index_bytes>pack::MAX_INDEX as u64||d.records==0||d.records as usize>pack::MAX_RECORDS||d.codec>1||Key::new(&d.first)?.role()!=Some(d.role.as_str())||Key::new(&d.last)?.role()!=Some(d.role.as_str())||d.first>d.last||!hashes.insert(d.path("idx")?)||cat.version<3&&d.segment!=0 {
                    return Err("integrity:catalog-descriptor".into());
                }
                segments.insert(d.segment);
                if segments.len()>4{return Err("integrity:catalog-segment-cap".into());}
                if previous.is_some_and(|p|(p.segment,&p.role,&p.first)>=(d.segment,&d.role,&d.first)||p.segment==d.segment&&p.role==d.role&&p.last>=d.first){return Err("integrity:catalog-order-overlap".into());}
                for (name,len) in [(d.path("pack")?,d.pack_bytes),(d.path("idx")?,d.index_bytes)] {
                    m.cleanup_step()?;m.pack_metadata_stats+=1;if root.join(name).metadata().map_err(|e|format!("integrity:catalog-required-file:{e}"))?.len()!=len{return Err("integrity:catalog-size-binding".into());}
                }
                previous=Some(d);
            }
            Ok(Some(cat))
        },_=>Err("integrity:store-catalog-version".into())
    }
}
impl PhysicalRead {
    fn from_lease(root:&Path,lease:File,c:crate::store::StoreConfig,m:&mut Metrics)->Result<Self> {
        let catalog=catalog(root,&c,m)?;
        Ok(Self{_lease:lease,root:root.into(),config:c,catalog,cache:BTreeMap::new(),cache_bytes:0,borrowed_index_bytes:0,constructor_index_bytes:0,construction_aux_bytes:0})
    }
    fn make_room(&mut self,incoming:usize,m:&mut Metrics)->Result<()> {
        self.make_room_limit(incoming,pack::MAX_CACHE,m)
    }
    fn make_room_limit(&mut self,incoming:usize,limit:usize,m:&mut Metrics)->Result<()> {
        let bound=self.borrowed_index_bytes.checked_add(self.constructor_index_bytes).and_then(|n|n.checked_add(self.construction_aux_bytes)).and_then(|n|n.checked_add(incoming)).ok_or("integrity:index-memory-overflow")?;
        if bound>limit{return Err("integrity:index-cache-bound".into());}
        while self.cache_bytes.checked_add(bound).is_none_or(|n|n>limit) {
            let key=self.cache.keys().next().ok_or("integrity:cache-accounting")?.clone();
            self.cache_bytes-=self.cache.remove(&key).unwrap().memory();
        }
        // Charged transient upper bound, including an index being loaded and
        // a verifier's simultaneously live index, not only the final cache.
        m.pack_cache_peak_bytes=m.pack_cache_peak_bytes.max((self.cache_bytes+bound) as u64);m.pack_constructor_peak_bytes=m.pack_constructor_peak_bytes.max((self.constructor_index_bytes+self.construction_aux_bytes) as u64);Ok(())
    }
    fn index(&mut self,d:&Descriptor,m:&mut Metrics)->Result<Rc<Index>> {
        let cache_key=if let Some(p)=&d.page{format!("{}:{}:{}:{}:{}",d.path("idx")?,p.version,p.hash,p.bytes,p.height)}else{d.path("idx")?};
        if let Some(index)=self.cache.get(&cache_key){m.pack_cache_hits+=1;return Ok(index.clone());}
        let bound=Index::allocation_bound(d)?;
        // Evict before allocation: the loading index is included in residency.
        self.make_room(bound,m)?;
        let index=Rc::new(Index::load(&self.root,d,m)?);let bytes=index.memory();
        if bytes>bound{return Err("integrity:index-cache-estimate".into());}
        self.cache_bytes+=bytes;m.pack_cache_peak_bytes=m.pack_cache_peak_bytes.max(self.cache_bytes as u64);self.cache.insert(cache_key,index.clone());Ok(index)
    }
    fn locate(&mut self,key:&Key,limit:usize,m:&mut Metrics)->Result<Option<(Descriptor,pack::Entry,Vec<u8>)>> {
        let descriptors=self.catalog.as_ref().map_or(vec![],|cat|cat.packs.iter().filter(|d|Some(d.role.as_str())==key.role()&&d.first<=key.0&&key.0<=d.last).cloned().collect::<Vec<_>>());
        let paged_only=descriptors.iter().all(|d|d.page.is_some());let before_pages=m.pack_page_reads;let before_bytes=m.pack_index_metadata_bytes;let before_paged=m.pack_paged_index_metadata_bytes;let before_legacy=m.pack_legacy_index_bytes;m.pack_physical_key_lookups+=1;m.pack_key_relevant_max=m.pack_key_relevant_max.max(descriptors.len() as u64);
        let result=(||->Result<Option<(Descriptor,pack::Entry,Vec<u8>)>>{
        if descriptors.len()>4{return Err("integrity:relevant-segment-cap".into());}
        let mut found:Option<(Descriptor,pack::Entry,Vec<u8>)>=None;
        let mut entries:Vec<(Descriptor,pack::Entry)>=vec![];
        for d in descriptors {
            // No Rc survives a subsequent cache load: eviction never leaves an
            // uncharged borrowed index. Every relevant route is authenticated.
            let index=self.index(&d,m)?;
            if let Some(e)=index.find(key,m)? {
                if let Some((_,old))=entries.first() {
                    if old.hash!=e.hash||old.raw!=e.raw{return Err("integrity:segment-conflicting-identity".into());}
                }entries.push((d,e));
            }
        }
        // Complete every route before applying the caller bound; no payload
        // allocation, file read or decode occurs until all mapped sizes fit.
        if entries.iter().any(|(_,e)|e.raw>limit){return Err("integrity:packed-read-bound".into());}
        for (d,e) in entries{let raw=pack::read_record(&self.root,&d,&e,limit,m)?;if let Some((_,_,bytes))=&found{if bytes!=&raw{return Err("integrity:segment-conflicting-raw".into());}}else{found=Some((d,e,raw));}}
        Ok(found)
        })();
        let pages=m.pack_page_reads-before_pages;let bytes=m.pack_index_metadata_bytes-before_bytes;m.pack_key_page_read_max=m.pack_key_page_read_max.max(pages);m.pack_key_index_bytes_max=m.pack_key_index_bytes_max.max(bytes);
        m.pack_key_paged_index_bytes_max=m.pack_key_paged_index_bytes_max.max(m.pack_paged_index_metadata_bytes-before_paged);m.pack_key_legacy_index_bytes_max=m.pack_key_legacy_index_bytes_max.max(m.pack_legacy_index_bytes-before_legacy);
        if !paged_only{m.pack_key_legacy_or_mixed_lookups+=1;}
        if paged_only&&(pages>12||bytes>192*1024){return Err("integrity:paged-physical-key-work-bound".into());}result
    }
}
pub(crate) fn capture(root:&Path,m:&mut Metrics)->Result<()> {
    if let Some(p)=&m.physical {
        if p.root!=root{return Err("integrity:physical-root-rebind".into());}
        return Ok(());
    }
    let before=m.bytes_read;let (c,lease)=physical::capture(root,m)?;
    m.physical=Some(PhysicalRead::from_lease(root,lease,c,m)?);
    let bytes=m.bytes_read-before;m.physical_capture_bytes+=bytes;
    if m.physical.as_ref().and_then(|p|p.catalog.as_ref()).is_some_and(|c|c.version==4)&&bytes>65536{return Err("integrity:paged-startup-budget".into());}
    io::checkpoint("pack-read-lease")?;io::checkpoint("pack-catalog-captured")
}
fn key_for(path:&Path,m:&Metrics)->Option<(PathBuf,Key)> {
    let root=m.store_root.as_ref()?;let relative=path.strip_prefix(root).ok()?;
    let name=relative.components().map(|c|c.as_os_str().to_string_lossy()).collect::<Vec<_>>().join("/");
    Key::new(&name).ok().map(|key|(root.clone(),key))
}
fn lookup_bound(key:&Key)->usize{if key.role()==Some("hint"){65536}else{pack::MAX_RAW}}
pub(crate) fn read(path:&Path,limit:usize,m:&mut Metrics)->Result<Option<Vec<u8>>> {
    let Some((root,key))=key_for(path,m) else{return Ok(None)};
    capture(&root,m)?;
    let mut physical=m.physical.take().ok_or("integrity:physical-capture")?;
    let result=(||->Result<Option<Vec<u8>>>{
        if let Some((_,_,raw))=physical.locate(&key,limit,m)?{return Ok(Some(raw));}Ok(None)
    })();m.physical=Some(physical);result
}
pub(crate) fn exists(path:&Path,m:&mut Metrics)->Result<bool> {
    let Some((root,key))=key_for(path,m) else{return Ok(pack::path_exists(path,m))};capture(&root,m)?;
    let mut physical=m.physical.take().ok_or("integrity:physical-capture")?;
    let result=physical.locate(&key,lookup_bound(&key),m).and_then(|v|if v.is_some(){Ok(true)}else{present(path,m)});m.physical=Some(physical);result
}
pub(crate) fn probe(store:&Store,v:&Value,m:&mut Metrics)->Result<Value> {
    if v.get("concurrent_pacing").is_some(){return paced_probe(store,v,m);}
    let count=v["count"].as_u64().unwrap_or(500);let requests=v["requests"].as_array().ok_or("requests")?;
    if count==0||count>5000||requests.len()>64{return Err("refused:lookup-sample-bound".into());}
    let mut rows=vec![];
    for request in requests {
        let id=request["id"].as_str().ok_or("lookup-id")?;let category=request["category"].as_str().ok_or("lookup-category")?;
        let first=Instant::now();let expected=store.resolve(id,None,m)?;let first_ns=first.elapsed().as_nanos();let mut times=Vec::with_capacity(count as usize);
        let (opens,storage,decoded,hits)=(m.opens,m.pack_storage_bytes,m.pack_decoded_bytes,m.pack_cache_hits);
        for _ in 0..count {let t=Instant::now();let result=store.resolve(id,None,m)?;times.push(t.elapsed().as_nanos());if result!=expected{return Err("integrity:lookup-outcome-changed".into());}}
        rows.push(probe_row(id,category,first_ns,times,(m.opens-opens,m.pack_storage_bytes-storage,m.pack_decoded_bytes-decoded,m.pack_cache_hits-hits),&expected)?);
    }
    Ok(json!({"rows":rows,"cache_qualification":LOOKUP_CACHE}))
}

const LOOKUP_CACHE:&str="same-process warm after separately timed first lookup; device/OS cache eviction unmeasured";

fn probe_row(id:&str,cat:&str,ns:u128,times:Vec<u128>,d:(u64,u64,u64,u64),v:&Value)->Result<Value>{
let mut s=times.clone();s.sort_unstable();Ok(json!({"category":cat,"id":id,"first_lookup_ns":ns,"warm_ns":times,"warm_median_ns":s[s.len()/2],"warm_p95_ns":s[(s.len()*95/100).min(s.len()-1)],"warm_min_ns":s[0],"warm_max_ns":s.last(),"result_digest":io::digest(&serde_json::to_vec(v)?),"opens":d.0,"pack_storage_bytes":d.1,"decoded_record_bytes":d.2,"index_cache_hits":d.3}))
}

#[derive(Clone,Copy,Serialize,Deserialize,PartialEq,Eq)]
#[serde(rename_all="kebab-case")]
enum PaceFamily {Cp4Control,F05Selected}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Pace {version:u8,phase_id:String,family:PaceFamily,reader:u8,batch:u8,control_dir:PathBuf,anchor_sha256:Option<String>,ready_sha256:Option<String>}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct PaceAnchor {clock_kind:String,version:u8,phase_id:String,frequency:u64,origin:u64}
fn pace_in(v:&Value)->Result<Pace>{let p:Pace=serde_json::from_value(v["concurrent_pacing"].clone())?;let hashes=[&p.anchor_sha256,&p.ready_sha256];if p.version!=1||!crate::model::safe_id(&p.phase_id)||p.reader>3||p.batch>9||v["count"].as_u64()!=Some(1000)||v["requests"].as_array().is_none_or(|a|a.len()!=1)||hashes.iter().any(|h|if p.batch==0{h.is_some()}else{h.as_ref().is_none_or(|s|!crate::objects::valid_hash(s))}){return Err("pace:input".into());}Ok(p)
}
fn pace_due(origin:u64,f:u64,reader:u8,batch:u8,j:u64)->Result<u64>{if f==0||reader>3||batch>9||j>=1000{return Err("pace:schedule".into());}let us=if reader==0{16000000+(batch as u128*1000+j as u128)*2900}else{(batch as u128*1000+j as u128)*4500};
u64::try_from(origin as u128+us*f as u128/1000000).map_err(|_|"pace:overflow".into())
}

fn pace_kind()->&'static str {
    #[cfg(windows)] {return "windows-qpc";}
    #[cfg(all(target_os="linux",target_pointer_width="64"))] {return "linux-clock-monotonic";}
    #[cfg(all(target_os="macos",target_pointer_width="64"))] {return "macos-clock-monotonic";}
    #[allow(unreachable_code)] "unsupported"
}
fn pace_nanos(sec:i64,nsec:i64)->Result<u64>{
    if sec<0||!(0..1_000_000_000).contains(&nsec){return Err("pace:timespec".into());}
    (sec as u64).checked_mul(1_000_000_000).and_then(|v|v.checked_add(nsec as u64)).ok_or_else(||"pace:clock-overflow".into())
}
fn pace_qpc()->Result<(u64,u64)>{
#[cfg(windows)] {
    #[link(name="kernel32")] extern "system"{fn QueryPerformanceCounter(p:*mut i64)->i32;fn QueryPerformanceFrequency(p:*mut i64)->i32;}
    let(mut c,mut f)=(0,0);
    if unsafe{QueryPerformanceCounter(&mut c)}==0||unsafe{QueryPerformanceFrequency(&mut f)}==0||c<0||f<=0{return Err("pace:QPC".into());}return Ok((c as u64,f as u64));
}

#[cfg(all(any(target_os="linux",target_os="macos"),target_pointer_width="64"))] {
    use std::os::raw::c_long;
    #[repr(C)] struct Timespec {sec:c_long,nsec:c_long}
    #[cfg(target_os="linux")] type ClockId=std::os::raw::c_int;
    #[cfg(target_os="macos")] type ClockId=std::os::raw::c_uint;
    extern "C" {fn clock_gettime(id:ClockId,value:*mut Timespec)->std::os::raw::c_int;}
    #[cfg(target_os="linux")] let id:ClockId=1;
    #[cfg(target_os="macos")] let id:ClockId=6;
    let mut t=Timespec{sec:0,nsec:0};
    if unsafe{clock_gettime(id,&mut t)}!=0{return Err(std::io::Error::last_os_error().into());}
    return Ok((pace_nanos(t.sec as i64,t.nsec as i64)?,1_000_000_000));
}
#[cfg(not(any(windows,all(any(target_os="linux",target_os="macos"),target_pointer_width="64"))))] {Err("pace:clock".into())}

}
fn pace_plain(path:&Path)->Result<()>{let m=std::fs::symlink_metadata(path)?;
#[cfg(windows)] {use std::os::windows::fs::MetadataExt;if m.file_attributes()&0x400!=0{return Err("pace:reparse".into());}}if m.file_type().is_symlink(){return Err("pace:link".into());}Ok(())
}
fn pace_open(path:&Path,new:bool)->Result<File>{
pace_plain(path.parent().ok_or("pace:invalid")?)?;if !new{pace_plain(path)?;}let mut o=OpenOptions::new();o.read(true).write(new).create_new(new);
#[cfg(windows)] {use std::os::windows::fs::OpenOptionsExt;o.share_mode(1);}let f=o.open(path)?;if !f.metadata()?.is_file(){return Err("pace:file".into());}Ok(f)
}
fn pace_bytes(f:&File)->Result<Vec<u8>>{
use std::io::Read;let mut b=vec![];f.take(4097).read_to_end(&mut b)?;if b.len()>4096{return Err("pace:size".into());}Ok(b)
}

fn pace_allowed(root:&Path,family:PaceFamily)->bool {
    // Source-owned closed namespaces. A request/env cannot add an allowed root.
    #[cfg(windows)]
    let (prefix,selected)=(
        r"\\?\C:\file-native-online-cp1-20261020\fixtures\cp3",
        &[
            r"\\?\C:\file-native-online-matrix-20261020\history\H1000-baseline\disposable\store",
            r"\\?\C:\file-native-online-matrix-20261020\history\authentic-N128\disposable\store",
        ][..]);
    #[cfg(target_os="macos")]
    let (prefix,selected):(&str,&[&str])=("/private/tmp/file-native-online-macos-20261020/fixtures",&[]);
    #[cfg(target_os="linux")]
    let (prefix,selected):(&str,&[&str])=("/home/spenser/file-native-online-linux-20261020/fixtures",&[]);
    #[cfg(not(any(windows,target_os="macos",target_os="linux")))]
    let (prefix,selected):(&str,&[&str])=("",&[]);
    match family {
        PaceFamily::F05Selected=>selected.iter().any(|p|root==Path::new(p)),
        PaceFamily::Cp4Control=>{
            if prefix.is_empty(){return false;}
            let Ok(tail)=root.strip_prefix(Path::new(prefix)) else{return false};
            let Some(parts)=tail.iter().map(|s|s.to_str()).collect::<Option<Vec<_>>>() else{return false};
            if cfg!(windows){return parts.len()==3&&parts[0].starts_with("cp4-")&&crate::model::safe_id(parts[0])&&parts[1]=="disposable"&&parts[2]=="store";}
            if parts.len()!=5||!parts[1..3].iter().all(|s|crate::model::safe_id(s))||parts[3]!="disposable"||parts[4]!="store"{return false;}
            let Some(pin)=parts[0].strip_prefix("native-paced-cp4-").or_else(||parts[0].strip_prefix("native-final-")) else{return false};
            pin.len()==40&&pin.bytes().all(|b|matches!(b,b'0'..=b'9'|b'a'..=b'f'))
        }
    }
}
fn pace_dir(store:&Store,p:&Pace,m:&mut Metrics)->Result<PathBuf>{if !store.root.is_absolute()||[store.root.as_path(),p.control_dir.as_path()].iter().any(|p|p.components().any(|c|matches!(c,std::path::Component::ParentDir))){return Err("pace:path-components".into());}let root=store.root.canonicalize()?;let permit=PathBuf::from(std::env::var("FILE_NATIVE_PACED_ROOT")?);if std::env::var("FILE_NATIVE_STAGE_JOB").ok().as_deref()!=Some("1")||permit!=root||!pace_allowed(&root,p.family){return Err("pace:root".into());}let family=root.parent().ok_or("pace:family")?;let dir=family.join("lookup-pace").join(&p.phase_id);for path in dir.ancestors().chain(store.root.ancestors()){pace_plain(path)?;}if family.file_name().is_none_or(|n|n!="disposable")||p.control_dir!=dir||dir.canonicalize()?!=dir||!dir.is_dir(){return Err("pace:dir".into());}
authorized(&root,m)?;Ok(dir)
}
fn pace_fit(h:&Value)->bool{serde_json::to_vec(h).is_ok_and(|b|b.len()<=2048)&&h["milestones"].as_array().is_some_and(|a|a.len()<=10)}

// PhysicalRead is not Clone; capture moves its sole noninheritable File.
// Cache Rcs contain Index/page bytes, never the generation lease. No lease
// accessor/try_clone exists; all resolve-local files/Rcs are gone on return.
fn pace_drop(m:&mut Metrics)->Result<(u64,u64)>{
    m.finish();
    let p=m.physical.as_ref().ok_or("pace:physical")?;
    let current=p.cache.values().try_fold(0usize,|n,i|n.checked_add(i.memory()).ok_or("pace:cache-overflow"))?;
    if current!=p.cache_bytes||p.borrowed_index_bytes!=0||p.constructor_index_bytes!=0||p.construction_aux_bytes!=0||p.cache.values().any(|i|Rc::strong_count(i)!=1)||current>pack::MAX_CACHE{return Err("pace:live-cache-alias-or-accounting".into());}
    m.pack_cache_peak_bytes=m.pack_cache_peak_bytes.max(current as u64);
    let peak=m.pack_cache_peak_bytes;
    let PhysicalRead{_lease:lease,cache,..}=m.physical.take().ok_or("pace:physical")?;
    drop(cache);drop(lease);Ok((current as u64,peak))
}
fn paced_probe(store:&Store,v:&Value,m:&mut Metrics)->Result<Value>{
use std::io::Write;let p=pace_in(v)?;let dir=pace_dir(store,&p,m)?;let (_,f)=pace_qpc()?;let r=&v["requests"][0];let id=r["id"].as_str().ok_or("pace:invalid")?;let cat=r["category"].as_str().ok_or("pace:invalid")?;if id.is_empty()||id.len()>256||cat.is_empty()||cat.len()>64||!cat.is_ascii(){return Err("pace:row".into());}let snap=store.snapshot(m)?;let c=captured_config(m)?;let g=c.physical.as_ref().ok_or("pace:v3")?;let cap=pace_qpc()?.0;let first=Instant::now();let want=store.resolve(id,None,m)?;let ns=first.elapsed().as_nanos();let fd=pace_qpc()?.0;let rp=dir.join(format!("reader-{}.ready.json",p.reader));let ready=if p.batch==0{
    let v=json!({"clock_kind":pace_kind(),"family":p.family,"version":1,"phase_id":p.phase_id,"reader":p.reader,"batch":0,"pid":std::process::id(),"request":id,"category":cat,"selector":snap.selected,"physical":g,"catalog":c.catalog,"capture_qpc":cap,"first_done_qpc":fd,"ready_qpc":pace_qpc()?.0,"frequency":f});
    let b=serde_json::to_vec(&v)?;if b.len()>4096{return Err("pace:size".into());}
    let mut file=pace_open(&rp,true)?;file.write_all(&b)?;file.sync_all()?;file
}else{pace_open(&rp,false)?};
use std::io::{Seek,SeekFrom};let mut ready=&ready;ready.seek(SeekFrom::Start(0))?;let rh=io::digest(&pace_bytes(ready)?);if p.ready_sha256.as_ref().is_some_and(|h|*h!=rh){return Err("pace:rebound".into());}let wait=Instant::now();let af=loop{
    match pace_open(&dir.join("anchor.json"),false){
        Ok(file)=>break file,
        Err(e) if e.downcast_ref::<std::io::Error>().is_some_and(|e|e.kind()==std::io::ErrorKind::NotFound)=>{},
        Err(e)=>return Err(e)
    }
    if wait.elapsed()>=Duration::from_secs(5){return Err("pace:anchor-timeout".into());}std::thread::sleep(Duration::from_millis(1));
};let b=pace_bytes(&af)?;let ah=io::digest(&b);let a:PaceAnchor=strict(&b)?;let now=pace_qpc()?.0;if a.clock_kind!=pace_kind()||a.version!=1||a.phase_id!=p.phase_id||a.frequency!=f||a.origin==0||p.anchor_sha256.as_ref().is_some_and(|h|*h!=ah)||p.batch==0&&(a.origin<=now||a.origin-now>f.checked_mul(2).ok_or("pace:invalid")?){return Err("pace:anchor".into());}
let(opens,storage,decoded,hits)=(m.opens,m.pack_storage_bytes,m.pack_decoded_bytes,m.pack_cache_hits);
let(mut times,mut marks)=(Vec::with_capacity(1000),vec![]);let(mut waits,mut lag,mut late,mut catch)=(0u64,0u64,0u64,0u64);for j in 0..1000 {
    let due=pace_due(a.origin,f,p.reader,p.batch,j)?;let pre=pace_qpc()?.0;if pre>due{catch+=1;}
    if pre<due{std::thread::sleep(Duration::from_secs_f64((due-pre) as f64/f as f64));}
    let start=pace_qpc()?.0;waits=waits.checked_add(start.checked_sub(pre).ok_or("pace:invalid")?).ok_or("pace:invalid")?;
    let d=start.saturating_sub(due);if d>0{late+=1;}lag=lag.max(d);
    let t=Instant::now();let result=store.resolve(id,None,m)?;times.push(t.elapsed().as_nanos());let end=pace_qpc()?.0;
    if result!=want{return Err("integrity:lookup-outcome-changed".into());}
    if j%111==0{marks.push([p.batch as u64*1000+j,start,end]);}
}let out=probe_row(id,cat,ns,times,(m.opens-opens,m.pack_storage_bytes-storage,m.pack_decoded_bytes-decoded,m.pack_cache_hits-hits),&want)?;let succ=config(&store.root,m)?.physical.ok_or("pace:invalid")?.ordinal.checked_sub(g.ordinal).ok_or("pace:invalid")?;let selected=snap.selected.clone();let io_done=pace_qpc()?.0;drop(snap);let (cache_current,cache_peak)=pace_drop(m)?;let dropped=pace_qpc()?.0;let h=json!({"clock_kind":pace_kind(),"family":p.family,"version":1,"phase_id":p.phase_id,"pid":std::process::id(),"reader":p.reader,"batch":p.batch,"frequency":f,"origin":a.origin,"period_us":if p.reader==0{2900}else{4500},"hold_us":if p.reader==0{16000000}else{0},"completed":1000,"first_read":marks[0][1],"last_read":marks[9][2],"io_done":io_done,"pins_dropped":dropped,"reply_ready":pace_qpc()?.0,"wait_ticks":waits,"max_lag_ticks":lag,"late":late,"catchup":catch,"deadline_ok":lag<=f/10&&succ<=60,"successors":succ,"physical":g,"catalog":c.catalog,"selector":selected,"cache_current_bytes":cache_current,"cache_peak_bytes":cache_peak,"cache_after_drop_bytes":0,"anchor_sha256":ah,"ready_sha256":rh,"milestones":marks});if !pace_fit(&h){return Err("pace:size".into());}
Ok(json!({"rows":[out],"concurrent_pacing":h,"cache_qualification":LOOKUP_CACHE}))
}

#[cfg(test)]mod pace_tests{use super::*;
#[test]fn closed(){let mut v=json!({"count":1000,"requests":[{}],"concurrent_pacing":{"version":1,"phase_id":"p","family":"cp4-control","reader":0,"batch":0,"control_dir":"x"}});assert!(pace_in(&v).is_ok());v["concurrent_pacing"]["reader"]=json!(4);assert!(pace_in(&v).is_err());v["concurrent_pacing"]["reader"]=json!(0);v["concurrent_pacing"]["extra"]=json!(0);assert!(pace_in(&v).is_err());
assert!(!pace_allowed(Path::new("/protected/disposable/store"),PaceFamily::F05Selected));
#[cfg(windows)] assert!(pace_allowed(Path::new(r"\\?\C:\file-native-online-matrix-20261020\history\H1000-baseline\disposable\store"),PaceFamily::F05Selected));
#[cfg(target_os="macos")] assert!(pace_allowed(Path::new("/private/tmp/file-native-online-macos-20261020/fixtures/native-final-0000000000000000000000000000000000000000/A/attempt/disposable/store"),PaceFamily::Cp4Control));
}
#[test]fn schedule(){for(r,b,j,us)in[(0,0,0,16000000),(0,0,999,18897100),(0,9,999,44997100),(1,9,999,44995500)]{assert_eq!(pace_due(7,1000000,r,b,j).unwrap(),us+7);}assert!(pace_due(u64::MAX,u64::MAX,0,0,0).is_err());assert_eq!(pace_nanos(2,3).unwrap(),2000000003);assert!(pace_nanos(-1,0).is_err());assert!(pace_nanos(i64::MAX,0).is_err());assert!(pace_nanos(1,1000000000).is_err());}
#[test]fn header(){assert!(!pace_fit(&json!({"milestones":vec![[0;3];11]})));assert!(!pace_fit(&json!({"milestones":[],"x":"x".repeat(2048)})));}
}

fn all_mapped(m:&mut Metrics)->Result<Vec<Key>> {
    let mut physical=m.physical.take().ok_or("integrity:physical-capture")?;
    let result=(||->Result<Vec<Key>>{
        let mut keys=vec![];
        for d in physical.catalog.as_ref().map_or(vec![],|c|c.packs.clone()) {
            let index=physical.index(&d,m)?;
            for n in 0..d.records as usize{keys.push(index.entry(n,m)?.key);}
        }Ok(keys)
    })();m.physical=Some(physical);result
}
#[derive(Clone,Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
struct BirthOwner {id:String,catalog:Option<String>,plan:Option<String>}
#[derive(Clone,Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
struct Journal { version:u32,id:String,status:String,expected_config:String,catalog:Option<String>,old:Vec<Descriptor>,reserved_bytes:u64,used_bytes:u64,deleted_bytes:u64,deferred_bytes:u64,#[serde(default,skip_serializing_if="Option::is_none")] target:Option<crate::store::StoreConfig>,#[serde(default,skip_serializing_if="Vec::is_empty")] abandoned:Vec<BirthOwner>,#[serde(default)] ownership_transferred:bool }
fn save(root:&Path,j:&Journal,m:&mut Metrics)->Result<()> {
    let bytes=io::json(j)?;if bytes.len()>65536||j.abandoned.len()>64{return Err("quota:maintenance-owner-bound".into());}
    io::atomic(&root.join("PACKING.json"),&bytes,m)
}
fn authorized(root:&Path,m:&mut Metrics)->Result<()> {
    let allowed:String=strict(&pack::native(&root.join("DISPOSABLE.json"),65536,m)?)?;
    m.pack_path_canonicalizations+=1;if allowed!=root.canonicalize()?.to_string_lossy(){return Err("refused:disposable-identity".into());}Ok(())
}
fn sharing_refusal(error:&(dyn std::error::Error+'static))->bool {
    #[cfg(windows)] {return error.downcast_ref::<std::io::Error>().is_some_and(|e|matches!(e.raw_os_error(),Some(32|33)));}
    #[cfg(not(windows))] {let _=error;false}
}
pub(crate) fn mapped(key:&Key,m:&mut Metrics)->Result<bool>{
    let mut physical=m.physical.take().ok_or("integrity:discovery-capture")?;
    let result=physical.locate(key,lookup_bound(key),m).map(|entry|entry.is_some());m.physical=Some(physical);result
}
fn consolidate(keys:&mut BTreeSet<Key>,old:&[Descriptor],limit:usize,m:&mut Metrics)->Result<Vec<Descriptor>>{
    let mut groups:BTreeMap<u64,Vec<Descriptor>>=BTreeMap::new();
    for d in old{groups.entry(d.segment).or_default().push(d.clone());}
    if keys.is_empty()||groups.len()<limit{return Ok(old.to_vec());}
    let mut sizes=groups.iter().map(|(id,ds)|(*id,ds.iter().map(|d|d.pack_bytes).sum::<u64>())).collect::<Vec<_>>();
    // Preserve a large stable base whenever smaller delta segments suffice.
    let stable=groups.keys().next().copied();
    if groups.len()>1{sizes.retain(|(id,_)|Some(*id)!=stable);}
    sizes.sort_by_key(|(id,bytes)|(*bytes,*id));
    let remove=sizes.into_iter().take(2).map(|(id,_)|id).collect::<BTreeSet<_>>();
    for d in old.iter().filter(|d|remove.contains(&d.segment)){
        let mut physical=m.physical.take().ok_or("integrity:consolidation-capture")?;
        let index=physical.index(d,m);m.physical=Some(physical);let index=index?;
        for n in 0..d.records as usize{
            let e=index.entry(n,m)?;
            m.rolling_collateral_raw_bytes=m.rolling_collateral_raw_bytes.checked_add(e.raw as u64).ok_or("quota:collateral-overflow")?;
            if m.rolling_collateral_raw_bytes>64*1024*1024{return Err("quota:collateral-rewrite-bound".into());}
            if (keys.len()+1)*768>16*1024*1024{return Err("quota:consolidation-bookkeeping".into());}
            keys.insert(e.key);m.rolling_collateral_records+=1;
        }
    }
    Ok(old.iter().filter(|d|!remove.contains(&d.segment)).cloned().collect())
}
struct OutputOwner{id:String,created:BTreeMap<String,(String,usize)>}
fn reserve_constructor(bytes:usize,m:&mut Metrics)->Result<()>{
    let mut p=m.physical.take().ok_or("integrity:constructor-capture")?;p.constructor_index_bytes=bytes;let result=p.make_room(0,m);m.physical=Some(p);result
}
fn new_builder(role:&str,codec:u8,m:&mut Metrics)->Result<pack::Builder>{reserve_constructor(4096,m)?;Ok(pack::Builder::new(role,codec))}
fn push_builder(builder:&mut pack::Builder,key:Key,raw:&[u8],m:&mut Metrics)->Result<()>{
    reserve_constructor(builder.push_bound(&key)?,m)?;builder.push(key,raw)?;let actual=builder.metadata_memory();m.pack_constructor_metadata_peak_bytes=m.pack_constructor_metadata_peak_bytes.max(actual as u64);reserve_constructor(actual,m)
}
fn install(root:&Path,stage:&Path,bytes:&[u8],name:&str,hash:&str,m:&mut Metrics,owner:&mut OutputOwner)->Result<()> {
    let id=stage.file_name().and_then(|s|s.to_str()).ok_or("integrity:birth-id")?;
    if !crate::model::safe_id(id)||id!=owner.id{return Err("integrity:birth-id".into());}
    let destination=root.join(format!("packs/{id}/{name}"));
    // Birth ids are never reused, including identical content. A failed build
    // owns this directory until explicit abort/reaping; no existing-path adopt.
    if io::digest(bytes)!=hash{return Err("integrity:install-container-hash".into());}
    if let Some((created_hash,created_bytes))=owner.created.get(name){
        if created_hash!=hash||*created_bytes!=bytes.len(){return Err("integrity:owned-output-conflict".into());}
        // This table is populated only AFTER successful creation in this live
        // invocation. The already durable building journal owns the fresh birth;
        // a crash cannot resume installation or adopt a pre-existing pathname.
        crate::paged::verify_file(&destination,bytes.len() as u64,hash,m)?;m.pack_output_same_owner_reuses+=1;return Ok(());
    }
    if owner.created.len()>=128{return Err("quota:owned-output-count".into());}
    io::write_new(&destination,bytes,m)?;owner.created.insert(name.into(),(hash.into(),bytes.len()));m.pack_output_created_files+=1;Ok(())
}
fn identity(hash:&mut Sha256,e:&pack::Entry){io::forbidden();hash.update((e.key.0.len() as u64).to_le_bytes());hash.update(e.key.0.as_bytes());hash.update((e.raw as u64).to_le_bytes());hash.update(e.hash.as_bytes());}
fn flush(root:&Path,stage:&Path,builder:pack::Builder,m:&mut Metrics,expected_identity:Option<String>,owner:&mut OutputOwner)->Result<Descriptor> {
    let approved=builder.finish_bound()?;reserve_constructor(approved,m)?;
    let (mut d,pack,index)=builder.finish_reserved(approved)?;
    m.pack_constructor_serialized_peak_bytes=m.pack_constructor_serialized_peak_bytes.max(index.capacity() as u64);reserve_constructor(index.capacity(),m)?;
    d.instance=Some(stage.file_name().and_then(|s|s.to_str()).ok_or("integrity:birth-id")?.into());
    io::checkpoint("pack-build")?;
    install(root,stage,&pack,&format!("{}.pack",d.pack),&d.pack,m,owner)?;io::checkpoint("pack-index")?;
    install(root,stage,&index,&format!("{}.idx",d.index),&d.index,m,owner)?;io::checkpoint("pack-install")?;
    drop(index);drop(pack);reserve_constructor(0,m)?;
    let started=Instant::now();let bound=Index::allocation_bound(&d)?;
    let previous_borrow=m.physical.as_ref().map_or(0,|p|p.borrowed_index_bytes);
    if let Some(physical)=m.physical.as_mut(){physical.borrowed_index_bytes=previous_borrow.checked_add(bound).ok_or("integrity:index-memory-overflow")?;}
    let mut physical=m.physical.take().ok_or("integrity:verification-capture")?;physical.make_room(0,m)?;m.physical=Some(physical);
    let checked=Index::load(root,&d,m)?;
    checked.audit(m)?;crate::paged::verify_file(&root.join(d.path("pack")?),d.pack_bytes,&d.pack,m)?;
    let mut actual_identity=Sha256::new();
    for n in 0..d.records as usize {
        let e=checked.entry(n,m)?;let raw=checked.read(root,&e,pack::MAX_RAW,m)?;
        identity(&mut actual_identity,&e);
        if expected_identity.is_none(){let original=io::read(&root.join(&e.key.0),m)?;if raw!=original{return Err("integrity:pack-complete-input-correspondence".into());}}
    }
    if expected_identity.as_ref().is_some_and(|expected|format!("{:x}",actual_identity.finalize())!=*expected){return Err("integrity:conversion-complete-input-correspondence".into());}
    drop(checked);if let Some(physical)=m.physical.as_mut(){physical.borrowed_index_bytes=previous_borrow;}
    m.elapsed("pack-complete-verification",started);Ok(d)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn empty_reader(root:&Path)->PhysicalRead{std::fs::create_dir_all(root).unwrap();PhysicalRead{_lease:physical::lock_file(&root.join("lease"),true).unwrap(),root:root.into(),config:crate::store::StoreConfig{version:1,store_id:"test".into(),mode:"A".into(),consumer:"project".into(),fallback:false,navigation_version:4,catalog:None,physical:None},catalog:None,cache:BTreeMap::new(),cache_bytes:0,borrowed_index_bytes:0,constructor_index_bytes:0,construction_aux_bytes:0}}
    fn write_pair(root:&Path,d:&Descriptor,p:&[u8],i:&[u8]){std::fs::create_dir_all(root.join(d.path("idx").unwrap()).parent().unwrap()).unwrap();std::fs::write(root.join(d.path("idx").unwrap()),i).unwrap();std::fs::write(root.join(d.path("pack").unwrap()),p).unwrap();}
    #[test]fn production_builder_preserves_near_limit_index_bytes(){
        let root=PathBuf::from(std::env::var_os("EXP_PACK_TEST_ROOT").expect("explicit fixture root required")).join(io::unique("constructor-near-index-limit"));
        let mut m=Metrics::default();m.physical=Some(empty_reader(&root));let mut builder=new_builder("allocation",0,&mut m).unwrap();
        fn key(n:usize)->Key{Key::new(&format!("attempts/{}/allocations/{}{:06}.json","a".repeat(99),"b".repeat(93),n)).unwrap()}
        for n in 0..55366{let k=key(n);assert_eq!(k.0.len(),225);push_builder(&mut builder,k,b"",&mut m).unwrap();}
        assert!(!builder.fits(&key(55366),0));let approved=builder.finish_bound().unwrap();reserve_constructor(approved,&mut m).unwrap();
        assert!(m.pack_cache_peak_bytes<=pack::MAX_CACHE as u64);assert!(m.pack_constructor_metadata_peak_bytes>16*1024*1024);
        let (descriptor,body,index)=builder.finish_reserved(approved).unwrap();assert!(body.is_empty());assert_eq!(descriptor.records,55366);assert_eq!(index.len(),16191779);
        reserve_constructor(index.capacity(),&mut m).unwrap();let expected=io::digest(&index);drop(index);drop(body);reserve_constructor(0,&mut m).unwrap();
        // A separate direct builder must serialize identically; cached accounting
        // and production reservations have no effect on descriptor/index bytes.
        let mut reference=pack::Builder::new("allocation",0);for n in 0..55366{reference.push(key(n),b"").unwrap();}
        let (other,body,index)=reference.finish().unwrap();assert_eq!(descriptor,other);assert!(body.is_empty());assert_eq!(io::digest(&index),expected);
    }
    #[test]fn caller_limit_precedes_raw_and_zstd_bodies_after_all_routes(){
        for codec in [0,1]{let root=PathBuf::from(std::env::var_os("EXP_PACK_TEST_ROOT").unwrap()).join(io::unique("caller-limit"));let mut p=empty_reader(&root);let key=Key::new(&format!("results/{}.json","a".repeat(64))).unwrap();let mut builder=pack::Builder::new("result",codec);builder.push(key.clone(),&[42;256]).unwrap();let (mut d,pack,bytes)=builder.finish().unwrap();d.instance=Some("first".into());d.segment=1;write_pair(&root,&d,&pack,&bytes);p.catalog=Some(Catalog{version:4,store_id:"test".into(),packs:vec![d.clone()]});let mut m=Metrics::default();
            assert!(p.locate(&key,1,&mut m).err().unwrap().to_string().contains("packed-read-bound"));assert_eq!(m.pack_records_read,0);assert_eq!(m.pack_decoded_bytes,0);assert_eq!(p.locate(&key,256,&mut m).unwrap().unwrap().2,vec![42;256]);
            let mut late=d;late.instance=Some("late".into());late.segment=2;let mut damaged=bytes;damaged[0]^=1;write_pair(&root,&late,&pack,&damaged);p.catalog.as_mut().unwrap().packs.push(late);let read=m.pack_records_read;let decoded=m.pack_decoded_bytes;assert!(p.locate(&key,1,&mut m).err().unwrap().to_string().contains("paged-page-hash"));assert_eq!(m.pack_records_read,read);assert_eq!(m.pack_decoded_bytes,decoded);
        }
        let root=PathBuf::from(std::env::var_os("EXP_PACK_TEST_ROOT").unwrap()).join(io::unique("hint-exists-bound"));let mut p=empty_reader(&root);let key=Key::new("attempt-hints/a.json").unwrap();let mut b=pack::Builder::new("hint",1);b.push(key.clone(),&vec![42;65537]).unwrap();let (mut d,pack,bytes)=b.finish().unwrap();d.instance=Some("hint".into());write_pair(&root,&d,&pack,&bytes);p.catalog=Some(Catalog{version:4,store_id:"test".into(),packs:vec![d]});let mut m=Metrics::default();m.store_root=Some(root.clone());m.physical=Some(p);assert!(exists(&root.join(key.0),&mut m).err().unwrap().to_string().contains("packed-read-bound"));assert_eq!(m.pack_records_read,0);assert_eq!(m.pack_decoded_bytes,0);
    }
    #[test]fn constructor_reservation_evicts_before_allocating_output(){
        let root=PathBuf::from(std::env::var_os("EXP_PACK_TEST_ROOT").unwrap()).join(io::unique("constructor-budget"));let mut p=empty_reader(&root);let mut m=Metrics::default();
        for part in 0..5{let mut b=pack::Builder::new("result",0);for n in 0..100{b.push(Key::new(&format!("results/{:064x}.json",part*100+n)).unwrap(),b"").unwrap();}let (mut d,pack,bytes)=b.finish_legacy().unwrap();d.instance=Some(format!("legacy-{part}"));write_pair(&root,&d,&pack,&bytes);p.index(&d,&mut m).unwrap();}
        let limit=96*1024;let output=48*1024;let borrowed=vec![0u8;8*1024];p.borrowed_index_bytes=borrowed.capacity();assert!(p.cache_bytes+borrowed.capacity()+output>limit);let old=p.cache.len();p.constructor_index_bytes=output;p.make_room_limit(0,limit,&mut m).unwrap();assert!(p.cache.len()<old);let actual_output=vec![0u8;output];assert!(p.cache_bytes+borrowed.capacity()+actual_output.capacity()<=limit);assert!(m.pack_cache_peak_bytes<=limit as u64);
        p.constructor_index_bytes=limit+1;assert!(p.make_room_limit(0,limit,&mut m).is_err());assert!(60*1024*1024usize+16191779>pack::MAX_CACHE);
        let mut b=pack::Builder::new("result",0);b.push(Key::new(&format!("results/{}.json","b".repeat(64))).unwrap(),b"raw").unwrap();let bound=b.finish_bound().unwrap();assert!(b.finish_reserved(bound-1).is_err());
    }
    #[test]fn mixed_capture_and_cold_warm_measurements_reconcile(){
        let root=PathBuf::from(std::env::var_os("EXP_PACK_TEST_ROOT").unwrap()).join(io::unique("mixed-stats"));std::fs::create_dir_all(&root).unwrap();let key=Key::new(&format!("results/{}.json","c".repeat(64))).unwrap();let mut descriptors=vec![];
        for legacy in [true,false]{let mut b=pack::Builder::new("result",0);b.push(key.clone(),&[7;256]).unwrap();let (mut d,pack,bytes)=if legacy{b.finish_legacy().unwrap()}else{b.finish().unwrap()};d.instance=Some(if legacy{"legacy"}else{"paged"}.into());d.segment=if legacy{1}else{2};write_pair(&root,&d,&pack,&bytes);descriptors.push(d);}
        let cat=Catalog{version:4,store_id:"mixed".into(),packs:descriptors.clone()};let data=io::json(&cat).unwrap();let hash=io::digest(&data);std::fs::create_dir_all(root.join("pack-catalogs")).unwrap();std::fs::write(root.join(format!("pack-catalogs/{hash}.json")),data).unwrap();let c=crate::store::StoreConfig{version:3,store_id:"mixed".into(),mode:"A".into(),consumer:"project".into(),fallback:false,navigation_version:4,catalog:Some(hash),physical:Some(physical::Binding{id:"mixed".into(),ordinal:1,plan:"a".repeat(64)})};std::fs::write(root.join("STORE.json"),io::json(&c).unwrap()).unwrap();physical::create_pin(&root,&c,&mut Metrics::default()).unwrap();
        let mut m=Metrics::default();m.store_root=Some(root.clone());capture(&root,&mut m).unwrap();assert_eq!(m.pack_metadata_stats,7);let mut p=m.physical.take().unwrap();p.locate(&key,256,&mut m).unwrap();assert_eq!(m.pack_metadata_stats,12);assert_eq!(m.pack_page_reads,1);assert_eq!(m.pack_pages_parsed,2);assert_eq!(m.pack_key_legacy_or_mixed_lookups,1);assert_eq!(m.pack_key_index_bytes_max,descriptors.iter().map(|d|d.index_bytes).sum::<u64>());assert_eq!(m.pack_key_legacy_index_bytes_max,descriptors[0].index_bytes);assert_eq!(m.pack_key_paged_index_bytes_max,descriptors[1].index_bytes);
        let before=(m.pack_metadata_stats,m.pack_page_reads,m.pack_pages_parsed,m.pack_index_metadata_bytes);p.locate(&key,256,&mut m).unwrap();assert_eq!(m.pack_metadata_stats-before.0,2);assert_eq!(m.pack_page_reads-before.1,0);assert_eq!(m.pack_pages_parsed-before.2,1);assert_eq!(m.pack_index_metadata_bytes-before.3,0);
    }
    #[test]fn paged_cache_identity_preserves_physical_instance(){
        let root=PathBuf::from(std::env::var_os("EXP_PACK_TEST_ROOT").expect("explicit fixture root required")).join(io::unique("paged-instance"));
        let mut builder=pack::Builder::new("result",0);builder.push(Key::new(&format!("results/{}.json","a".repeat(64))).unwrap(),b"raw").unwrap();let (mut a,pack,bytes)=builder.finish().unwrap();a.instance=Some("first".into());let mut b=a.clone();b.instance=Some("second".into());
        for d in [&a,&b]{std::fs::create_dir_all(root.join(format!("packs/{}",d.instance.as_ref().unwrap()))).unwrap();std::fs::write(root.join(d.path("pack").unwrap()),&pack).unwrap();std::fs::write(root.join(d.path("idx").unwrap()),&bytes).unwrap();}
        let lease=physical::lock_file(&root.join("lease"),true).unwrap();let mut p=PhysicalRead{_lease:lease,root:root.clone(),config:crate::store::StoreConfig{version:1,store_id:"cache-test".into(),mode:"A".into(),consumer:"project".into(),fallback:false,navigation_version:4,catalog:None,physical:None},catalog:None,cache:BTreeMap::new(),cache_bytes:0,borrowed_index_bytes:0,constructor_index_bytes:0,construction_aux_bytes:0};let mut m=Metrics::default();p.index(&a,&mut m).unwrap();
        let mut damaged=bytes;damaged[0]^=1;std::fs::write(root.join(b.path("idx").unwrap()),damaged).unwrap();assert!(p.index(&b,&mut m).is_err());assert_eq!(p.cache.len(),1);assert!(m.pack_cache_peak_bytes<=pack::MAX_CACHE as u64);
    }
    #[test]fn paged_four_routes_negative_duplicates_and_conflict(){
        let root=PathBuf::from(std::env::var_os("EXP_PACK_TEST_ROOT").expect("explicit fixture root required")).join(io::unique("paged-four"));std::fs::create_dir_all(&root).unwrap();
        let mut descriptors=vec![];
        for part in 0..5{
            let mut builder=pack::Builder::new("result",0);for n in 0..2400{builder.push(Key::new(&format!("results/{:064x}.json",n*2)).unwrap(),if part==4&&n==1200{b"BAD"}else{b"raw"}).unwrap();}
            let (mut d,pack,bytes)=builder.finish().unwrap();d.instance=Some(format!("route-{part}"));d.segment=part+1;std::fs::create_dir_all(root.join(format!("packs/route-{part}"))).unwrap();std::fs::write(root.join(d.path("pack").unwrap()),pack).unwrap();std::fs::write(root.join(d.path("idx").unwrap()),bytes).unwrap();descriptors.push(d);
        }
        let lease=physical::lock_file(&root.join("lease"),true).unwrap();let mut p=PhysicalRead{_lease:lease,root:root.clone(),config:crate::store::StoreConfig{version:3,store_id:"four-routes".into(),mode:"A".into(),consumer:"project".into(),fallback:false,navigation_version:4,catalog:None,physical:None},catalog:Some(Catalog{version:4,store_id:"four-routes".into(),packs:descriptors[..4].to_vec()}),cache:BTreeMap::new(),cache_bytes:0,borrowed_index_bytes:0,constructor_index_bytes:0,construction_aux_bytes:0};let mut m=Metrics::default();
        let hit=Key::new(&format!("results/{:064x}.json",1200*2)).unwrap();let value=p.locate(&hit,pack::MAX_RAW,&mut m).unwrap().unwrap();assert_eq!(value.2,b"raw");assert_eq!(m.pack_key_relevant_max,4);assert_eq!(m.pack_key_page_read_max,12);assert!(m.pack_key_index_bytes_max<=192*1024);
        let missing=Key::new(&format!("results/{:064x}.json",1200*2+1)).unwrap();assert!(p.locate(&missing,pack::MAX_RAW,&mut m).unwrap().is_none());
        p.catalog.as_mut().unwrap().packs[3]=descriptors[4].clone();assert!(p.locate(&hit,pack::MAX_RAW,&mut m).err().unwrap().to_string().contains("segment-conflicting"));
    }
    #[test] fn loading_indexes_evicts_before_transient_allocation(){
        let root=std::env::var_os("EXP_PACK_TEST_ROOT").map(PathBuf::from).unwrap_or_else(||PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target")).join(io::unique("packing-cache-boundary"));
        std::fs::create_dir_all(root.join("packs")).unwrap();
        let lease=OpenOptions::new().create_new(true).read(true).write(true).open(root.join("lease")).unwrap();
        let mut physical=PhysicalRead{_lease:lease,root:root.clone(),config:crate::store::StoreConfig{version:1,store_id:"cache-test".into(),mode:"A".into(),consumer:"project".into(),fallback:false,navigation_version:4,catalog:None,physical:None},catalog:None,cache:BTreeMap::new(),cache_bytes:0,borrowed_index_bytes:0,constructor_index_bytes:0,construction_aux_bytes:0};let mut metrics=Metrics::default();
        let mut descriptors=vec![];
        for part in 0..5 {
            let mut builder=pack::Builder::new("allocation",0);
            for n in 0..56000 {let key=Key::new(&format!("attempts/{}-{part}/allocations/{}-{n:06}.json","a".repeat(80),"b".repeat(75))).unwrap();builder.push(key,b"").unwrap();}
            let (d,p,i)=builder.finish_legacy().unwrap();std::fs::write(root.join(format!("packs/{}.pack",d.pack)),p).unwrap();std::fs::write(root.join(format!("packs/{}.idx",d.index)),i).unwrap();descriptors.push(d);
        }
        let total:usize=descriptors.iter().map(|d|Index::allocation_bound(d).unwrap()).sum();assert!(total>pack::MAX_CACHE);
        for d in &descriptors {
            let index=physical.index(d,&mut metrics).unwrap();assert_eq!(index.entry(0,&mut metrics).unwrap().key.0,d.first);
            assert!(physical.cache_bytes+physical.borrowed_index_bytes<=pack::MAX_CACHE);
        }
        assert!(physical.cache.len()<5);assert!(metrics.pack_cache_peak_bytes<=pack::MAX_CACHE as u64);assert!(metrics.pack_cache_peak_bytes>48*1024*1024);
        // Maintenance verification also pins a separate incoming index while old
        // catalog indexes are loaded: that storage shares the same budget.
        physical.borrowed_index_bytes=Index::allocation_bound(&descriptors[0]).unwrap();physical.make_room(0,&mut metrics).unwrap();
        for d in descriptors.iter().skip(1){physical.index(d,&mut metrics).unwrap();}
        assert!(physical.cache_bytes+physical.borrowed_index_bytes<=pack::MAX_CACHE);assert!(metrics.pack_cache_peak_bytes<=pack::MAX_CACHE as u64);
        physical.borrowed_index_bytes=0;
        let mut changed=descriptors[0].clone();changed.index="f".repeat(64);changed.index_bytes=53;changed.records=1;
        std::fs::copy(root.join(format!("packs/{}.idx",descriptors[0].index)),root.join(format!("packs/{}.idx",changed.index))).unwrap();
        let before=physical.cache_bytes;
        let error=match physical.index(&changed,&mut metrics){Ok(_)=>panic!("changed oversized index accepted"),Err(error)=>error};
        assert!(error.to_string().contains("pack-file-bound"));assert!(physical.cache_bytes<=before);assert!(metrics.pack_cache_peak_bytes<=pack::MAX_CACHE as u64);
        // Reuse the SAME actual source files. Reserve an independent source
        // allocation before loading it, while the existing real cache is live.
        let source_charge=Index::allocation_bound(&descriptors[0]).unwrap();physical.borrowed_index_bytes=source_charge;physical.make_room(0,&mut metrics).unwrap();
        let source=Index::load(&root,&descriptors[0],&mut metrics).unwrap();assert!(source.memory()<=source_charge);metrics.physical=Some(physical);
        let mut builder=new_builder("allocation",0,&mut metrics).unwrap();for n in 0..descriptors[0].records as usize{let e=source.entry(n,&mut metrics).unwrap();push_builder(&mut builder,e.key,b"",&mut metrics).unwrap();}
        let approved=builder.finish_bound().unwrap();let live=metrics.physical.as_ref().unwrap();let cache_before=live.cache.len();let before_finish=live.cache_bytes+source_charge+approved;
        assert!(before_finish>pack::MAX_CACHE);assert!(source_charge+approved<=pack::MAX_CACHE);
        reserve_constructor(approved,&mut metrics).unwrap();assert!(metrics.physical.as_ref().unwrap().cache.len()<cache_before);
        let (out,body,index)=builder.finish_reserved(approved).unwrap();assert_eq!(out.records,descriptors[0].records);assert!(body.is_empty());assert!(index.capacity()<=pack::MAX_INDEX);reserve_constructor(index.capacity(),&mut metrics).unwrap();
        assert!(metrics.pack_cache_peak_bytes<=pack::MAX_CACHE as u64);let output_capacity=index.capacity();drop(index);drop(body);reserve_constructor(0,&mut metrics).unwrap();
        let peak=metrics.pack_cache_peak_bytes;assert!(reserve_constructor(usize::MAX,&mut metrics).err().unwrap().to_string().contains("index-memory-overflow"));assert_eq!(metrics.pack_cache_peak_bytes,peak);reserve_constructor(0,&mut metrics).unwrap();
        drop(source);metrics.physical.as_mut().unwrap().borrowed_index_bytes=0;
        eprintln!("cache-boundary-receipt root={} total_index_charge={} transient_charged_peak={} remaining_cache_entries={} source_charge={} before_finish={} approved_constructor={} actual_output_capacity={} changed_index_refused_before_allocation=true constructor_eviction_before_output=true overflow_refused_before_allocation=true extra_fixture_bytes=0",root.display(),total,metrics.pack_cache_peak_bytes,metrics.physical.as_ref().unwrap().cache.len(),source_charge,before_finish,approved,output_capacity);
    }
}

fn selected_packs(m:&Metrics)->Vec<Descriptor>{m.physical.as_ref().and_then(|p|p.catalog.as_ref()).map_or(vec![],|c|c.packs.clone())}
fn blank_work(c:&crate::store::StoreConfig,id:&str,ordinal:u64,packs:Vec<Descriptor>,hash:&str,parent:&str)->physical::Work{
    physical::Work{discovery:None,version:2,store_id:c.store_id.clone(),physical_parent:parent.into(),transition_parent:None,birth_claims:vec![],generation:physical::Generation{id:id.into(),ordinal,catalog:hash.into(),plan:None,packs},older:vec![],jobs:vec![],retired:vec![],garbage:vec![],removed_bytes:0,removed_files:0,reconciled_absent:0}
}
fn append_work(root:&Path,c:&crate::store::StoreConfig,id:&str,packs:Vec<Descriptor>,hash:&str,old:&[Descriptor],parent:&str,m:&mut Metrics)->Result<physical::Work>{
    let ordinal=next_ordinal(c)?;
    let mut w=if c.version==3{physical::load(root,c,m)?}else{blank_work(c,id,ordinal,vec![],hash,parent)};
    if c.version==3{w.older.push(physical::generation(c,old.to_vec())?);}
    else if let Some(previous)=&c.catalog{w.garbage.push(format!("pack-catalogs/{previous}.json"));}
    for d in old{
        let pack_live=packs.iter().any(|p|p.path("pack").ok()==d.path("pack").ok());
        let index_live=packs.iter().any(|p|p.path("idx").ok()==d.path("idx").ok());
        if !pack_live||!index_live{
            if let Some(retired)=w.retired.iter_mut().find(|r|r.descriptor==*d){retired.pack_done|=pack_live;retired.index_done|=index_live;}
            else{w.retired.push(physical::Retired{descriptor:d.clone(),pack_done:pack_live,index_done:index_live});}
        }
    }
    w.physical_parent=parent.into();
    w.generation=physical::Generation{id:id.into(),ordinal,catalog:hash.into(),plan:None,packs:packs.clone()};
    // A committed physical successor advances the ordinal before old claims
    // disappear. Unselected candidates cannot clear the parent authority.
    w.birth_claims.clear();w.transition_parent=None;
    w.garbage.push(format!("physical-stages/{id}/STORE.json"));
    for d in packs{if !old.contains(&d){w.jobs.push(physical::LooseJob{descriptor:d,frontier:ordinal,position:0});}}
    physical::validate_work(&w)?;Ok(w)
}
fn next_ordinal(c:&crate::store::StoreConfig)->Result<u64>{
    c.physical.as_ref().map_or(Some(1),|b|b.ordinal.checked_add(1)).ok_or_else(||"quota:physical-generation-overflow".into())
}
// Deliberately full, explicit representation work. It neither advances a
// semantic discovery cursor nor materializes the complete mapped key set.
fn converted_output(output:&mut Vec<Descriptor>,d:Descriptor,m:&mut Metrics)->Result<()>{
    let mut identity=d.clone();identity.segment=0;
    if output.iter().any(|old|{let mut old=old.clone();old.segment=0;old==identity}){m.pack_conversion_duplicate_records+=d.records as u64;return Ok(());}
    if output.len()>=64{return Err("quota:conversion-pack-cap".into());}output.push(d);Ok(())
}
fn convert_pairs(root:&Path,stage:&Path,old:&[Descriptor],m:&mut Metrics,owner:&mut OutputOwner)->Result<(Vec<Descriptor>,u64,u64)>{
    let mut output=vec![];let mut records=0u64;let mut raw_bytes=0u64;
    for d in old{
        if physical::cancelled(){return Err("cancelled:full-conversion".into());}
        let charge=Index::allocation_bound(d)?;
        let mut physical=m.physical.take().ok_or("integrity:conversion-capture")?;
        if physical.borrowed_index_bytes!=0{return Err("integrity:conversion-borrow".into());}
        physical.borrowed_index_bytes=charge;physical.make_room(0,m)?;m.physical=Some(physical);
        let index=Index::load(root,d,m)?;index.audit(m)?;crate::paged::verify_file(&root.join(d.path("pack")?),d.pack_bytes,&d.pack,m)?;
        let mut builder=new_builder(&d.role,d.codec,m)?;let mut source_identity=Sha256::new();
        for n in 0..d.records as usize{
            if physical::cancelled(){return Err("cancelled:full-conversion".into());}
            let e=index.entry(n,m)?;let raw=index.read(root,&e,pack::MAX_RAW,m)?;
            if !builder.fits(&e.key,raw.len()){
                let expected=format!("{:x}",std::mem::take(&mut source_identity).finalize());
                let mut new=flush(root,stage,builder,m,Some(expected),owner)?;new.segment=d.segment;converted_output(&mut output,new,m)?;builder=new_builder(&d.role,d.codec,m)?;
            }
            records=records.checked_add(1).ok_or("integrity:conversion-overflow")?;raw_bytes=raw_bytes.checked_add(raw.len() as u64).ok_or("integrity:conversion-overflow")?;
            identity(&mut source_identity,&e);push_builder(&mut builder,e.key,&raw,m)?;
        }
        if !builder.is_empty(){let expected=format!("{:x}",source_identity.finalize());let mut new=flush(root,stage,builder,m,Some(expected),owner)?;new.segment=d.segment;converted_output(&mut output,new,m)?;}
        drop(index);if let Some(p)=m.physical.as_mut(){p.borrowed_index_bytes=0;}
        if output.len()>64{return Err("quota:conversion-pack-cap".into());}
    }
    m.pack_conversion_input_records=records;m.pack_conversion_output_descriptor_records=output.iter().map(|d|d.records as u64).sum();Ok((output,records,raw_bytes))
}
fn abandoned_owners(old:&Journal)->Result<Vec<BirthOwner>>{
    if old.version!=2||old.status!="aborted-owned"{return Ok(vec![]);}
    let mut owners=old.abandoned.clone();
    if !old.ownership_transferred{owners.push(BirthOwner{id:old.id.clone(),catalog:old.catalog.clone(),plan:old.target.as_ref().and_then(|c|c.physical.as_ref()).map(|b|b.plan.clone())});}
    owners.sort_by(|a,b|a.id.cmp(&b.id));
    for pair in owners.windows(2){if pair[0].id==pair[1].id&&(pair[0].catalog!=pair[1].catalog||pair[0].plan!=pair[1].plan){return Err("integrity:aborted-owner-conflict".into());}}
    owners.dedup_by(|a,b|a.id==b.id);
    if owners.len()>64||owners.iter().any(|o|!crate::model::safe_id(&o.id)||o.catalog.as_ref().is_some_and(|h|!crate::objects::valid_hash(h))||o.plan.as_ref().is_some_and(|h|!crate::objects::valid_hash(h))){return Err("quota:maintenance-owner-bound".into());}
    Ok(owners)
}
fn inherit_aborted(root:&Path,owners:&[BirthOwner],w:&mut physical::Work,m:&mut Metrics)->Result<()>{
    for owner in owners{
        m.cleanup_step()?;
        // Enumerate only the <=128 files under an explicit durable birth owner.
        // This is neither a historical directory discovery nor a journal chain.
        let directory=root.join(format!("packs/{}",owner.id));
        w.garbage.push(format!("packs/{}",owner.id));
        if pack::path_exists(&directory,m){
            m.cleanup_step()?;for (n,e) in io::read_dir(&directory)?.enumerate(){
                m.cleanup_step()?;
                if n>=128{return Err("quota:orphan-container-bound".into());}
                let e=e?;m.pack_metadata_stats+=1;if !e.file_type()?.is_file(){return Err("integrity:orphan-container-type".into());}
                let name=e.file_name().to_str().ok_or("integrity:orphan-container-name")?.to_owned();
                w.garbage.push(format!("packs/{}/{name}",owner.id));
            }
        }
        if let Some(hash)=&owner.catalog{w.garbage.push(format!("pack-catalogs/{hash}.json"));}
        if let Some(plan)=&owner.plan{
            let path=root.join(format!("physical-plans/{plan}.json"));
            if pack::path_exists(&path,m){
                m.cleanup_step()?;let bytes=pack::native(&path,physical::WORK_LIMIT,m)?;m.cleanup_step()?;
                if io::digest(&bytes)!=*plan{return Err("integrity:aborted-plan-digest".into());}
                let candidate:physical::Work=strict(&bytes)?;physical::validate_work(&candidate)?;
                if candidate.store_id!=w.store_id||candidate.generation.id!=owner.id||Some(&candidate.generation.catalog)!=owner.catalog.as_ref(){return Err("integrity:aborted-plan-owner".into());}
                // Transitive ownership is flattened before this plan retires.
                w.garbage.extend(candidate.garbage);
            }
            w.garbage.push(format!("physical-plans/{plan}.json"));
        }
        w.garbage.push(format!("physical-progress/{}.work-stage",owner.id));w.garbage.push(format!("physical-progress/{}.selector-stage",owner.id));w.garbage.push(format!("physical-pins/{}.lock",owner.id));w.garbage.push(format!("physical-progress/{}.json",owner.id));w.garbage.push(format!("physical-stages/{}/STORE.json",owner.id));
    }
    w.garbage.sort();w.garbage.dedup();physical::validate_work(w)
}
fn adopt_aborted(root:&Path,c:&crate::store::StoreConfig,j:&mut Journal,m:&mut Metrics)->Result<bool>{
    let owners=abandoned_owners(j)?;let Some(owner)=owners.first() else{return Ok(true)};
    m.cleanup_step()?;let mut work=physical::load(root,c,m)?;
    if owner.id==work.generation.id||work.older.iter().any(|g|g.id==owner.id){return Err("integrity:aborted-selected-owner".into());}
    if !work.birth_claims.contains(&owner.id){work.birth_claims.push(owner.id.clone());}
    if let Err(e)=inherit_aborted(root,std::slice::from_ref(owner),&mut work,m){
        if e.to_string().starts_with("quota:physical-"){return Ok(false);}return Err(e);
    }
    physical::save(root,c.physical.as_ref().ok_or("integrity:work-binding")?,&mut work,m)?;
    // Durable selected ownership precedes dropping any journal reference. Death
    // between these writes leaves duplicate ownership, never an orphan.
    if owner.id==j.id{j.ownership_transferred=true;}else{j.abandoned.retain(|o|o.id!=owner.id);}
    m.cleanup_step()?;save(root,j,m)?;Ok(true)
}
// The STORE stage, complete candidate validation and all work parsing precede
// exclusion. Only a bounded STORE CAS and native replacement are inside Q/W.
fn record_activation_hold(m:&mut Metrics,hold:Instant)->Result<()>{
    if m.physical_activation_holds_ns.len()>=10000{return Err("quota:activation-attempt-bound".into());}
    m.physical_activation_holds_ns.push(hold.elapsed().as_nanos().min(u64::MAX as u128) as u64);Ok(())
}
fn activate(root:&Path,store:&Store,j:&Journal,m:&mut Metrics)->Result<()> {
    let target=j.target.as_ref().ok_or("integrity:activation-target")?;
    let now=config(root,m)?;
    if now.physical.as_ref().map(|b|&b.id)==Some(&j.id){return Ok(());}
    if j.status!="installed"{return Err("refused:unselected-incomplete-plan".into());}
    if target.version!=3||target.store_id!=now.store_id||target.mode!=now.mode||target.consumer!=now.consumer||target.fallback!=now.fallback||target.navigation_version!=now.navigation_version{return Err("integrity:activation-frozen-config".into());}
    let binding=target.physical.as_ref().ok_or("integrity:activation-binding")?;
    if binding.id!=j.id||binding.ordinal!=next_ordinal(&now)?||target.catalog!=j.catalog||j.abandoned.iter().any(|o|o.id==binding.id){return Err("integrity:activation-candidate-owner".into());}
    let work=physical::load(root,target,m)?;
    if work.physical_parent!=j.expected_config{return Err("integrity:activation-parent-plan".into());}
    let _candidate_pin=physical::candidate_pin(root,target,m)?;

    // Revalidate every candidate record outside Q/W, including a resumed build.
    let cat=catalog(root,target,m)?.ok_or("integrity:activation-catalog")?;
    let upgrade=cat.version==4&&m.physical.as_ref().and_then(|p|p.catalog.as_ref()).is_none_or(|c|c.version<4);
    let upgrade_pins=if upgrade{physical::upgrade_handles(root,&now,m)?}else{vec![]};
    let inherited=selected_packs(m);
    for d in &cat.packs {
        // Continuous immutable parent descriptors are not newly constructed
        // representations. Authenticate their selected identity, not old bodies.
        if inherited.contains(d){continue;}
        let bound=Index::allocation_bound(d)?;
        if let Some(p)=m.physical.as_mut(){p.borrowed_index_bytes=bound;}
        let mut p=m.physical.take().ok_or("integrity:activation-capture")?;p.make_room(0,m)?;m.physical=Some(p);
        let index=Index::load(root,d,m)?;
        index.audit(m)?;crate::paged::verify_file(&root.join(d.path("pack")?),d.pack_bytes,&d.pack,m)?;
        for n in 0..d.records as usize{let e=index.entry(n,m)?;index.read(root,&e,pack::MAX_RAW,m)?;}
        drop(index);if let Some(p)=m.physical.as_mut(){p.borrowed_index_bytes=0;}
    }
    physical::load(root,target,m)?;
    let stage=root.join(format!("physical-stages/{}/STORE.json",j.id));
    let target_bytes=io::json(target)?;
    if target_bytes.len()>physical::BINDING_LIMIT{return Err("quota:physical-binding-bound".into());}
    if pack::path_exists(&stage,m){if pack::native(&stage,physical::BINDING_LIMIT,m)?!=target_bytes{return Err("integrity:activation-stage".into());}}
    else{io::write_new(&stage,&target_bytes,m)?;}
    // A builder's old pin must end before a one-time legacy drain. M prevents
    // another maintenance owner from replacing/reclaiming its input meanwhile.
    m.physical=None;io::checkpoint("pack-reader-drain")?;
    let wait=Instant::now();let cap=std::env::var("EXP_WAIT_MS").ok().and_then(|s|s.parse().ok()).unwrap_or(15000);
    loop {
        if physical::cancelled(){return Err("cancelled:pack-barrier".into());}
        let q=io::GlobalGate::new(physical::wait_lock(root,"physical-capture.lock",true)?,false);
        let hold=Instant::now();
        let current_bytes=pack::native(&root.join("STORE.json"),physical::BINDING_LIMIT,m)?;
        let c:crate::store::StoreConfig=strict(&current_bytes)?;
        if io::digest(&current_bytes)!=j.expected_config{drop(q);record_activation_hold(m,hold)?;return Err("conflict:physical-catalog".into());}
        // Initial conversion must drain old binaries. Never wait for legacy or
        // W under Q: fresh nested Metrics can request Q while already holding W.
        let legacy=if c.version<3{
            let f=physical::lock_file(&root.join("pack-readers.lock"),true)?;
            match FileExt::try_lock_exclusive(&f){Ok(())=>Some(f),Err(e) if physical::contended(&e)=>{drop(q);record_activation_hold(m,hold)?;if wait.elapsed()>Duration::from_millis(cap){return Err("busy:legacy-readers".into());}std::thread::sleep(Duration::from_millis(2));continue;},Err(e)=>return Err(e.into())}
        }else{None};
        let mut locked=0usize;
        for pin in &upgrade_pins{match FileExt::try_lock_exclusive(pin){Ok(())=>locked+=1,Err(e) if physical::contended(&e)=>break,Err(e)=>return Err(e.into())}}
        if locked!=upgrade_pins.len(){
            for pin in upgrade_pins.iter().take(locked){FileExt::unlock(pin)?;}drop(legacy);drop(q);m.physical_upgrade_holds_ns.push(hold.elapsed().as_nanos().min(u64::MAX as u128) as u64);record_activation_hold(m,hold)?;
            if wait.elapsed()>Duration::from_millis(cap){return Err("busy:upgrade-readers".into());}std::thread::sleep(Duration::from_millis(2));continue;
        }
        let Some(writer)=store.try_writer(m)? else{
            for pin in &upgrade_pins{FileExt::unlock(pin)?;}
            drop(legacy);drop(q);
            record_activation_hold(m,hold)?;
            if wait.elapsed()>Duration::from_millis(cap){return Err("busy:activation-writer".into());}
            std::thread::sleep(Duration::from_millis(2));continue;
        };
        let result=io::replace(&stage,&root.join("STORE.json"),m).and_then(|_|io::sync_dir(root,m));
        if upgrade{m.physical_upgrade_holds_ns.push(hold.elapsed().as_nanos().min(u64::MAX as u128) as u64);m.physical_upgrade_wait_ns=wait.elapsed().as_nanos().min(u64::MAX as u128) as u64;}
        drop(writer);drop(legacy);drop(q);record_activation_hold(m,hold)?;m.elapsed("physical-activation-hold",hold);m.elapsed("physical-activation-wait-inclusive",wait);
        // No injected barrier or journal I/O runs under Q/W.
        result?;io::checkpoint("physical-after-store-replace")?;io::remove_dir(stage.parent().ok_or("integrity:stage-parent")?)?;io::checkpoint("pack-catalog")?;io::checkpoint("physical-after-selection")?;return Ok(());
    }
}
fn try_remove(root:&Path,name:&str,w:&mut physical::Work,m:&mut Metrics)->Result<bool>{
    m.cleanup_step()?;let path=root.join(name);if !present(&path,m)?{w.reconciled_absent+=1;return Ok(true);}
    io::checkpoint("physical-before-delete")?;m.cleanup_step()?;let started=Instant::now();
    if name.strip_prefix("packs/").is_some_and(crate::model::safe_id){
        let removed=io::remove_dir(&path);m.elapsed("physical-directory-remove-native",started);
        return match removed{Ok(())=>{m.namespace_ops+=1;io::checkpoint("physical-owned-directory-delete")?;Ok(true)},Err(e) if e.kind()==std::io::ErrorKind::NotFound=>Ok(true),Err(e) if e.kind()==std::io::ErrorKind::DirectoryNotEmpty||matches!(e.raw_os_error(),Some(32|33))=>Ok(false),Err(e)=>Err(e.into())};
    }
    let removed=lifetime::remove_checked(root,&path,m);
    let ns=started.elapsed().as_nanos();m.ns.entry("physical-delete-native-max".into()).and_modify(|n|*n=(*n).max(ns)).or_insert(ns);
    m.elapsed("physical-delete-native-total",started);
    match removed {
        Ok(n)=>{w.removed_bytes+=n;w.removed_files+=1;Ok(true)},
        Err(e) if e.downcast_ref::<std::io::Error>().is_some_and(|e|e.kind()==std::io::ErrorKind::NotFound)=>{w.reconciled_absent+=1;Ok(true)},
        Err(e) if sharing_refusal(e.as_ref())=>{eprintln!("deferred-delete:{}:{e}",path.display());Ok(false)},
        Err(e)=>Err(e),
    }
}
fn present(path:&Path,m:&mut Metrics)->Result<bool>{
    m.pack_metadata_stats+=1;
    match std::fs::symlink_metadata(path){Ok(_)=>Ok(true),Err(e) if e.kind()==std::io::ErrorKind::NotFound=>Ok(false),Err(e)=>Err(e.into())}
}
fn cleanup(root:&Path,c:&crate::store::StoreConfig,m:&mut Metrics)->Result<Value>{
    if m.cleanup_budget.is_none(){m.cleanup_budget=Some(physical::CleanupBudget::new());}
    let started=m.cleanup_budget.as_ref().unwrap().started;m.cleanup_step()?;let binding=c.physical.as_ref().ok_or("integrity:cleanup-binding")?;
    let mut w=physical::load(root,c,m)?;if w.generation.packs!=selected_packs(m){return Err("integrity:work-catalog-correspondence".into());}let before_bytes=w.removed_bytes;let before_files=w.removed_files;
    let mut candidates=0usize;let mut blocked=0u64;let mut loose_controls=0u64;let mut loose_control_bytes=0u64;
    // Nonblocking checks cover every retained old generation, including those
    // with an unmapped loose fallback. A drain is persisted before pin removal.
    let mut n=0;
    while n<w.older.len()&&m.cleanup_available(){
        candidates+=1;m.cleanup_step()?;
        if physical::drained(root,&w.older[n],&c.store_id,m)?{
            physical::reap_generation(&mut w,n)?;physical::save(root,binding,&mut w,m)?;
        }else{n+=1;blocked+=1;}
    }
    let mut n=0;
    while n<w.jobs.len()&&m.cleanup_available(){
        let job=w.jobs[n].clone();
        if w.older.iter().any(|g|g.ordinal<job.frontier){blocked+=1;n+=1;continue;}
        let bound=Index::allocation_bound(&job.descriptor)?;
        if let Some(p)=m.physical.as_mut(){p.borrowed_index_bytes=bound;}
        let mut p=m.physical.take().ok_or("integrity:cleanup-capture")?;p.make_room(0,m)?;m.physical=Some(p);
        m.cleanup_step()?;let index_started=Instant::now();let index=Index::load(root,&job.descriptor,m)?;m.elapsed(if job.descriptor.page.is_some(){"cleanup-paged-open"}else{"cleanup-legacy-whole-index-indivisible"},index_started);
        if !m.cleanup_available(){return Err("deferred:index-open-budget".into());}
        while w.jobs[n].position<job.descriptor.records&&m.cleanup_available(){
            if physical::cancelled(){physical::save(root,binding,&mut w,m)?;return Err("cancelled:physical-cleanup".into());}
            candidates+=1;m.cleanup_step()?;let e=index.entry(w.jobs[n].position as usize,m)?;let path=root.join(&e.key.0);
            if c.mode=="B"&&e.key.role()==Some("publication"){
                m.cleanup_step()?;let record_started=Instant::now();let raw=index.read(root,&e,pack::MAX_RAW,m)?;m.elapsed("cleanup-legacy-record-indivisible",record_started);m.cleanup_step()?;let loose=pack::native(&path,pack::MAX_RAW,m)?;
                if raw!=loose{return Err("integrity:B-loose-control-publication-missing-or-conflict".into());}
                loose_controls+=1;loose_control_bytes+=loose.len() as u64;w.jobs[n].position+=1;continue;
            }
            if !present(&path,m)?{w.reconciled_absent+=1;w.jobs[n].position+=1;continue;}
            // A successor may omit retired candidate/allocation records. Such
            // loose names remain with semantic GC; do not delete via a stale job.
            let mut p=m.physical.take().ok_or("integrity:cleanup-capture")?;
            m.cleanup_step()?;let locate_started=Instant::now();let replacement=p.locate(&e.key,pack::MAX_RAW,m);m.physical=Some(p);m.elapsed("cleanup-legacy-location-indivisible",locate_started);let replacement=replacement?;m.cleanup_step()?;
            if replacement.is_none(){
                if matches!(e.key.role(),Some("candidate"|"allocation")){w.jobs[n].position+=1;continue;}
                return Err("integrity:cleanup-selected-replacement-missing".into());
            }
            m.cleanup_step()?;let record_started=Instant::now();let raw=index.read(root,&e,pack::MAX_RAW,m)?;m.elapsed("cleanup-legacy-record-indivisible",record_started);m.cleanup_step()?;let canonical=io::read(&path,m)?;m.cleanup_step()?;let loose=pack::native(&path,pack::MAX_RAW,m)?;
            if raw!=canonical||raw!=loose{return Err("integrity:cleanup-conflicting-duplicate".into());}
            if !try_remove(root,&e.key.0,&mut w,m)?{blocked+=1;break;}
            w.jobs[n].position+=1;io::checkpoint("pack-loose-delete")?;
        }
        drop(index);if let Some(p)=m.physical.as_mut(){p.borrowed_index_bytes=0;}
        if w.jobs[n].position==job.descriptor.records{w.jobs.remove(n);}else{n+=1;}
    }
    // Publish completed cursor/coverage before any replacement job container
    // can disappear. Restart of an older immutable root can still replay safely.
    physical::save(root,binding,&mut w,m)?;
    let live_binding=config(root,m)?.physical.ok_or("integrity:cleanup-binding")?;
    let mut n=0;
    while n<w.retired.len()&&m.cleanup_available(){
        let d=w.retired[n].descriptor.clone();
        for extension in ["pack","idx"]{
            if !m.cleanup_available(){break;}
            let done=if extension=="pack"{w.retired[n].pack_done}else{w.retired[n].index_done};
            if done{continue;}
            candidates+=1;m.cleanup_step()?;if physical::referenced(&w,&d,extension)?{blocked+=1;continue;}
            if try_remove(root,&d.path(extension)?,&mut w,m)?{
                if extension=="pack"{w.retired[n].pack_done=true;}else{w.retired[n].index_done=true;}
                io::checkpoint("pack-old-delete")?;
            }else{blocked+=1;}
        }
        if w.retired[n].pack_done&&w.retired[n].index_done{
            if let Some(id)=&d.instance{let dir=root.join(format!("packs/{id}"));match io::remove_dir(&dir){Ok(())=>{},Err(e) if matches!(e.kind(),std::io::ErrorKind::DirectoryNotEmpty|std::io::ErrorKind::NotFound)=>{},Err(e)=>return Err(e.into())}}
            w.retired.remove(n);
        }else{n+=1;}
    }
    let mut n=0;
    while n<w.garbage.len()&&m.cleanup_available(){
        candidates+=1;m.cleanup_step()?;let name=w.garbage[n].clone();
        if physical::garbage_referenced(&w,&live_binding,&name)?{blocked+=1;n+=1;continue;}
        let before_delete=w.removed_files;
        if try_remove(root,&name,&mut w,m)?{
            if name.starts_with("packs/")&&name.split('/').count()==3&&w.removed_files>before_delete{io::checkpoint("physical-after-owned-file-delete")?;}
            if name.split('/').count()==3&&(name.starts_with("packs/")||name.starts_with("physical-stages/")){
                let dir=root.join(&name).parent().ok_or("integrity:garbage-parent")?.to_path_buf();
                m.cleanup_step()?;match io::remove_dir(dir){Ok(())=>{},Err(e) if e.kind()==std::io::ErrorKind::NotFound=>{},Err(e) if name.starts_with("physical-stages/")&&e.kind()==std::io::ErrorKind::DirectoryNotEmpty=>{blocked+=1;n+=1;continue;},Err(e) if e.kind()==std::io::ErrorKind::DirectoryNotEmpty=>{},Err(e)=>return Err(e.into())}
            }
            w.garbage.remove(n);
        }else{blocked+=1;n+=1;}
    }
    physical::save(root,binding,&mut w,m)?;m.elapsed("pack-deletion-inclusive",started);
    let final_binding=config(root,m)?.physical.ok_or("integrity:cleanup-binding")?;
    let protected_metadata=0usize;
    let pending=!w.jobs.is_empty()||!w.retired.is_empty()||w.garbage.len()>protected_metadata||!w.older.is_empty();
    Ok(json!({"status":if pending{"cleaning"}else{"complete"},"catalog":c.catalog,"generation":final_binding,"candidates":m.cleanup_budget.as_ref().map_or(candidates,|b|b.candidates),"batch_ns":started.elapsed().as_nanos(),"cooperative_limit_ms":50,"completion_basis":"selected immutable Work plus native absence of immutable instance names; absent metadata tombstones do not create work","candidate_limit":1000,"blocked_checks":blocked,"pending_loose_jobs":w.jobs.len(),"pending_old_containers":w.retired.len(),"pending_metadata":w.garbage.len(),"protected_metadata_until_successor":protected_metadata,"retained_generations":w.older.len()+1,"successfully_deleted_bytes_recorded":w.removed_bytes,"deleted_this_batch_bytes":w.removed_bytes-before_bytes,"deleted_this_batch_files":w.removed_files-before_files,"reconciled_absent_without_credit":w.reconciled_absent,"retained_loose_control_records":loose_controls,"retained_loose_control_bytes":loose_control_bytes,"native_allocation_credit":0,"historical_allocation_owners_unchanged":true}))
}
pub(crate) fn run(root:&Path,v:&Value,m:&mut Metrics)->Result<Value> {
    if v["operation"].as_str()==Some("cleanup"){m.cleanup_budget=Some(physical::CleanupBudget::new());}
    let result=run_inner(root,v,m);
    if let Some(b)=&m.cleanup_budget{
        let elapsed=b.started.elapsed().as_nanos();let count=b.candidates;
        m.ns.insert("cleanup-entry-inclusive".into(),elapsed);m.ns.insert("cleanup-budget-candidates".into(),count as u128);
        if let Err(e)=&result{if e.to_string().starts_with("deferred:"){
            return Ok(json!({"status":"cleaning","maintenance_deferral":e.to_string(),"completed":false,"batch_ns":elapsed,"candidates":count,"candidate_limit":1000,"cooperative_limit_ms":50,"selected_authority":"STORE.json","legacy_whole_index_indivisible":true,"native_allocation_credit":0}));
        }}
    }result
}
fn run_inner(root:&Path,v:&Value,m:&mut Metrics)->Result<Value> {
    m.store_root=Some(root.into());let maintenance=physical::lock_file(&root.join("pack-maintenance.lock"),true)?;
    FileExt::try_lock_exclusive(&maintenance).map_err(|e|format!("busy:competing-maintenance:{e}"))?;
    m.cleanup_step()?;let store=Store::open(root,m)?;m.cleanup_step()?;authorized(root,m)?;let operation=v["operation"].as_str().ok_or("operation")?;
    let previous=if pack::path_exists(&root.join("PACKING.json"),m){Some(strict::<Journal>(&pack::native(&root.join("PACKING.json"),65536,m)?)?)}else{None};
    if operation=="status"{
        return Ok(json!({"journal":previous,"selected_work":if store.config.version==3{Some(physical::load(root,&store.config,m)?)}else{None}}));
    }
    if operation=="verify"{
        let packs=selected_packs(m);let mut records=0u64;
        for d in &packs{
            let mut p=m.physical.take().ok_or("integrity:verify-capture")?;p.borrowed_index_bytes=Index::allocation_bound(d)?;p.make_room(0,m)?;m.physical=Some(p);
            let index=Index::load(root,d,m)?;index.audit(m)?;crate::paged::verify_file(&root.join(d.path("pack")?),d.pack_bytes,&d.pack,m)?;
            for n in 0..d.records as usize{let e=index.entry(n,m)?;index.read(root,&e,pack::MAX_RAW,m)?;records+=1;}
            drop(index);if let Some(p)=m.physical.as_mut(){p.borrowed_index_bytes=0;}
        }
        return Ok(json!({"status":"complete-physical-audit","packs":packs.len(),"records":records}));
    }
    if operation=="abort"{
        let mut j=previous.ok_or("refused:no-maintenance")?;
        if store.config.physical.as_ref().map(|b|&b.id)==Some(&j.id)||!["building","installed"].contains(&j.status.as_str()){return Err("refused:only-unselected-build-abort".into());}
        j.status="aborted-owned".into();save(root,&j,m)?;return Ok(json!({"status":j.status,"allocation_released":false}));
    }
    if operation=="cleanup"{
        m.cleanup_step()?;
        if store.config.version!=3{return Err("refused:legacy-cleanup-requires-generation-build".into());}
        let mut aborted=previous.filter(|j|j.status=="aborted-owned");
        let adoption=Instant::now();let adopted=if let Some(j)=aborted.as_mut(){adopt_aborted(root,&store.config,j,m)?}else{true};m.elapsed("cleanup-adoption-inclusive",adoption);
        let fresh=config(root,m)?;let mut result=cleanup(root,&fresh,m)?;
        let remaining=aborted.as_ref().map(abandoned_owners).transpose()?.map_or(0,|v|v.len());
        result["pending_aborted_owners"]=json!(remaining);result["aborted_owner_adoption_backpressure"]=json!(!adopted);
        if remaining>0{result["status"]=json!("cleaning");}return Ok(result);
    }
    let protection=store.preparation_gate(false)?;
    let mut j;
    if operation=="build"||operation=="repack"||operation=="convert"{
        let conversion=operation=="convert";
        if conversion&&v["force"].as_bool()!=Some(true){return Err("refused:explicit-full-conversion-required".into());}
        if let Some(old)=&previous{
            let selected=store.config.physical.as_ref().map(|b|&b.id)==Some(&old.id);
            if !selected&&!["registered","cleaning","complete","aborted-owned"].contains(&old.status.as_str()){return Err("refused:unfinished-maintenance-resume-or-abort".into());}
        }
        let construction=Instant::now();
        // Reservation transfer recovery remains with semantic writers. A physical
        // build conservatively excludes still-reserved attempts; it must not
        // authenticate legacy indexes while holding W merely to release them.
        let expected_bytes=pack::native(&root.join("STORE.json"),physical::BINDING_LIMIT,m)?;
        let expected:crate::store::StoreConfig=strict(&expected_bytes)?;
        if io::json(&expected)?!=io::json(&store.config)?{return Err("conflict:physical-capture-parent".into());}
        let expected_config=io::digest(&expected_bytes);
        let prior=if store.config.version==3{physical::load(root,&store.config,m)?.discovery}else{None};
        let (discovery,mut keys)=if conversion{(prior,BTreeSet::new())}else{let (d,keys)=crate::rolling::discover(&store,prior,v,m)?;(Some(d),keys)};
        let old=selected_packs(m);
        let segment_limit=v["segment_limit"].as_u64().unwrap_or(4);
        if segment_limit==0||segment_limit>4{return Err("refused:segment-limit".into());}
        let retained=if conversion{vec![]}else{consolidate(&mut keys,&old,segment_limit as usize,m)?};
        if conversion&&old.is_empty(){return Err("refused:full-conversion-needs-mapped-input".into());}
        // A low-count automatic refusal creates no build ownership/output.
        let mut potential_removable=0u64;
        for key in &keys{
            if store.config.mode=="B"&&key.role()==Some("publication"){continue;}
            m.pack_metadata_stats+=1;match std::fs::symlink_metadata(root.join(&key.0)){
                Ok(meta) if meta.file_type().is_file()=>potential_removable+=1,
                Ok(_)=>return Err("integrity:automatic-loose-file-type".into()),
                Err(e) if e.kind()==std::io::ErrorKind::NotFound=>{},Err(e)=>return Err(e.into()),
            }
        }
        if v["force"].as_bool()!=Some(true)&&potential_removable<128{return Err("refused:automatic-compaction-benefit".into());}
        let reserve=v["reserve_bytes"].as_u64().unwrap_or(256*1024*1024);
        if reserve>4*1024*1024*1024||reserve<1024*1024{return Err("refused:maintenance-reservation-bound".into());}
        let codec=match v["codec"].as_str().unwrap_or("raw"){"raw"=>0,"zstd"=>1,_=>return Err("refused:codec".into())};
        let ordinal=next_ordinal(&store.config)?;
        let birth=format!("physical-{ordinal}-{}",io::unique("birth"));
        if store.config.version==3&&physical::load(root,&store.config,m)?.birth_claims.contains(&birth){return Err("integrity:physical-birth-id-reuse".into());}
        if previous.as_ref().is_some_and(|old|old.id==birth||old.abandoned.iter().any(|owner|owner.id==birth))||pack::path_exists(&root.join(format!("packs/{birth}")),m)||pack::path_exists(&root.join(format!("physical-pins/{birth}.lock")),m)||pack::path_exists(&root.join(format!("physical-stages/{birth}")),m)||pack::path_exists(&root.join(format!("physical-progress/{birth}.work-stage")),m)||pack::path_exists(&root.join(format!("physical-progress/{birth}.selector-stage")),m){return Err("integrity:physical-birth-id-reuse".into());}
        let abandoned=previous.as_ref().map(abandoned_owners).transpose()?.unwrap_or_default();
        if abandoned.len()>=64{return Err("quota:maintenance-owner-bound-cleanup-first".into());}
        {let mut p=m.physical.take().ok_or("integrity:construction-capture")?;p.construction_aux_bytes=keys.len().checked_mul(768).and_then(|n|n.checked_add(1024*1024)).ok_or("integrity:construction-memory-overflow")?;let result=p.make_room(0,m);m.physical=Some(p);result?;}
        j=Journal{version:2,id:birth,status:"building".into(),expected_config,catalog:None,old:old.clone(),reserved_bytes:reserve,used_bytes:0,deleted_bytes:0,deferred_bytes:0,target:None,abandoned,ownership_transferred:false};
        save(root,&j,m)?;io::checkpoint("pack-reservation")?;m.quota_limit=Some(reserve);m.quota_spent=0;
        let mut output_owner=OutputOwner{id:j.id.clone(),created:BTreeMap::new()};
        let stage=root.join(format!("physical-stages/{}",j.id));let mut packs=vec![];let mut excluded=0u64;let mut excluded_bytes=0u64;
        let mut ordered:Vec<_>=keys.into_iter().collect();ordered.sort_by(|a,b|(a.role(),&a.0).cmp(&(b.role(),&b.0)));
        let mut builder:Option<pack::Builder>=None;let mut role=String::new();let mut logical=0u64;let mut records=0u64;let mut removable=(0u64,0u64);
        if conversion{
            let (converted,count,bytes)=convert_pairs(root,&stage,&old,m,&mut output_owner)?;packs=converted;records=count;logical=bytes;
        }else{for key in ordered{
            let raw=io::read(&root.join(&key.0),m)?;
            if raw.len()>pack::MAX_RAW{excluded+=1;excluded_bytes+=raw.len() as u64;continue;}
            if pack::path_exists(&root.join(&key.0),m)&&!(store.config.mode=="B"&&key.role()==Some("publication")){removable.0+=1;removable.1+=raw.len() as u64;}
            if pack::path_exists(&root.join(&key.0),m)&&pack::native(&root.join(&key.0),128*1024*1024,m)?!=raw{return Err("integrity:conflicting-loose-duplicate".into());}
            if builder.as_ref().is_some_and(|b|role!=key.role().unwrap()||!b.fits(&key,raw.len())){packs.push(flush(root,&stage,builder.take().unwrap(),m,None,&mut output_owner)?);}
            if builder.is_none(){role=key.role().unwrap().into();builder=Some(new_builder(&role,codec,m)?);}
            logical+=raw.len() as u64;records+=1;push_builder(builder.as_mut().unwrap(),key,&raw,m)?;
        }}
        if let Some(b)=builder{if !b.is_empty(){packs.push(flush(root,&stage,b,m,None,&mut output_owner)?);}}
        let added_files=packs.len()*2+3;

        if v["force"].as_bool()!=Some(true)&&(removable.0<128||removable.0<4*added_files as u64||removable.1<=m.quota_spent+physical::WORK_LIMIT as u64+physical::CATALOG_LIMIT as u64+4096){
            // Declining a completely owned candidate is a durable abort, not an
            // unfinished build requiring a manual repair before the next try.
            j.status="aborted-owned".into();j.used_bytes=m.quota_spent;m.quota_limit=None;save(root,&j,m)?;
            return Err("refused:automatic-compaction-benefit".into());
        }
        if !conversion{for d in &mut packs{d.segment=ordinal;}}
        packs.extend(retained);
        packs.sort_by(|a,b|(a.segment,&a.role,&a.first).cmp(&(b.segment,&b.role,&b.first)));
        if packs.len()>64{return Err("refused:registered-pack-cap".into());}
        let cat=Catalog{version:4,store_id:store.config.store_id.clone(),packs:packs.clone()};let bytes=io::json(&cat)?;
        if bytes.len()>physical::CATALOG_LIMIT{return Err("refused:catalog-size".into());}
        let hash=io::digest(&bytes);j.catalog=Some(hash.clone());save(root,&j,m)?;
        let catalog_path=root.join(format!("pack-catalogs/{hash}.json"));
        if pack::path_exists(&catalog_path,m){if pack::native(&catalog_path,physical::CATALOG_LIMIT,m)?!=bytes{return Err("integrity:existing-catalog-conflict".into());}}
        else{io::write_new(&catalog_path,&bytes,m)?;}
        let mut work=append_work(root,&store.config,&j.id,packs,&hash,&old,&j.expected_config,m)?;
        work.discovery=discovery;
        inherit_aborted(root,&j.abandoned,&mut work,m)?;
        // Initial legacy cleanup may have retained additional old containers.
        if store.config.version<3{if let Some(old_j)=&previous{for d in &old_j.old{physical::add_retired(&mut work,d.clone());}}}
        let mut target=store.config.clone();target.version=3;target.catalog=Some(hash.clone());
        let plan=io::digest(&io::json(&work)?);
        target.physical=Some(physical::Binding{id:j.id.clone(),ordinal:work.generation.ordinal,plan:plan.clone()});
        j.target=Some(target.clone());save(root,&j,m)?;
        if physical::persist_plan(root,&work,m)?!=plan{return Err("integrity:plan-correspondence".into());}
        physical::create_pin(root,&target,m)?;catalog(root,&target,m)?;
        j.catalog=Some(hash);j.target=Some(target);j.status="installed".into();j.used_bytes=m.quota_spent;m.quota_limit=None;save(root,&j,m)?;
        m.elapsed("pack-construction-inclusive",construction);m.pack_eligible_records=records;m.pack_eligible_raw_bytes=logical;m.pack_excluded_records+=excluded;m.pack_excluded_bytes+=excluded_bytes;
        io::checkpoint("pack-before-catalog")?;io::checkpoint("physical-before-selection")?;
    }else if operation=="resume"{
        j=previous.ok_or("refused:no-maintenance")?;
        if j.version!=2||j.target.is_none()||!["installed","registered","cleaning","complete"].contains(&j.status.as_str()){return Err("refused:incomplete-build-owned".into());}
    }else{return Err("refused:packing-operation".into());}
    if let Some(p)=m.physical.as_mut(){p.constructor_index_bytes=0;p.construction_aux_bytes=0;}
    activate(root,&store,&j,m)?;m.physical=None;capture(root,m)?;let selected=captured_config(m)?;
    if selected.physical.as_ref().map(|b|&b.id)!=Some(&j.id){return Err("conflict:physical-catalog".into());}
    j.status="registered".into();save(root,&j,m)?;io::checkpoint("pack-accounting")?;drop(protection);
    if v["cleanup"].as_bool()==Some(true){return cleanup(root,&selected,m);}
    Ok(json!({"status":"registered","catalog":selected.catalog,"generation":selected.physical,"discovery":physical::load(root,&selected,m)?.discovery,"forced_correctness":v["force"].as_bool()==Some(true),"reserved_bytes":j.reserved_bytes,"used_bytes":j.used_bytes,"successfully_deleted_bytes_recorded":0,"historical_allocation_owners_unchanged":true}))
}

#[cfg(test)]
mod boundary_tests {
use super::*;
#[test]fn catalog_count_bound(){
    let root=PathBuf::from(std::env::var_os("EXP_PACK_TEST_ROOT").unwrap()).join(io::unique("catalog-bound"));let mut m=Metrics::default();let mut ds=vec![];
    for i in 0..65 {let mut b=pack::Builder::new("result",0);b.push(Key::new(&format!("results/{i:064x}.json")).unwrap(),&[i as u8]).unwrap();let n=b.finish_bound().unwrap();let(mut d,body,index)=b.finish_reserved(n).unwrap();d.instance=Some("catalog-unit".into());d.segment=1;
        if i<64{io::write_new(&root.join(d.path("pack").unwrap()),&body,&mut m).unwrap();io::write_new(&root.join(d.path("idx").unwrap()),&index,&mut m).unwrap();}ds.push(d);
    }
    let mut c=crate::store::StoreConfig{version:3,store_id:"unit".into(),mode:"A".into(),consumer:"project".into(),fallback:false,navigation_version:4,catalog:None,physical:Some(physical::Binding{id:"catalog-unit".into(),ordinal:1,plan:"0".repeat(64)})};
    for count in [64,65]{let cat=Catalog{version:4,store_id:c.store_id.clone(),packs:ds[..count].to_vec()};let bytes=io::json(&cat).unwrap();assert!(bytes.len()<=physical::CATALOG_LIMIT);let hash=io::digest(&bytes);io::write_new(&root.join(format!("pack-catalogs/{hash}.json")),&bytes,&mut m).unwrap();c.catalog=Some(hash);
        let result=catalog(&root,&c,&mut m);if count==64{assert_eq!(result.unwrap().unwrap().packs.len(),64);}else{assert_eq!(result.err().unwrap().to_string(),"integrity:catalog-version-count");}
    }
    eprintln!("boundary production-catalog descriptors64=accepted descriptors65=count-refusal real-pairs=64");
}
#[test]fn install_birth_no_adoption(){
    let root=PathBuf::from(std::env::var_os("EXP_PACK_TEST_ROOT").unwrap()).join(io::unique("birth-bound"));let mut m=Metrics::default();let bytes=b"owned-canary";let h=io::digest(bytes);let name=format!("{h}.pack");
    let mut old=OutputOwner{id:"old".into(),created:BTreeMap::new()};let oldstage=root.join("physical-stages/old");install(&root,&oldstage,bytes,&name,&h,&mut m,&mut old).unwrap();
    let oldpath=root.join(format!("packs/old/{name}"));let before=std::fs::read(&oldpath).unwrap();
    let mut restarted=OutputOwner{id:"old".into(),created:BTreeMap::new()};assert!(install(&root,&oldstage,bytes,&name,&h,&mut m,&mut restarted).is_err());assert!(restarted.created.is_empty());assert_eq!(std::fs::read(&oldpath).unwrap(),before);
    let stage=root.join("physical-stages/fresh");let mut fresh=OutputOwner{id:"fresh".into(),created:BTreeMap::new()};install(&root,&stage,bytes,&name,&h,&mut m,&mut fresh).unwrap();install(&root,&stage,bytes,&name,&h,&mut m,&mut fresh).unwrap();assert_eq!(m.pack_output_same_owner_reuses,1);assert_eq!(std::fs::read(oldpath).unwrap(),before);
    eprintln!("boundary production-install stale-owner=refused fresh-birth=created same-live-owner=verified-reuse");
}
}
