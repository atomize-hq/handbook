//! Physical locations only. Semantic selection and content lifetime stay in Store.
use crate::{io::{self,Metrics,Result},pack::{self,Descriptor,Index,Key},store::Store,lifetime};
use fs2::FileExt;
use serde::{Serialize,Deserialize};
use serde_json::{json,Value};
use std::{collections::{BTreeMap,BTreeSet},fs::{File,OpenOptions},path::{Path,PathBuf},rc::Rc,time::{Duration,Instant}};

#[derive(Clone,Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
struct Catalog { version:u32,store_id:String,packs:Vec<Descriptor> }
pub(crate) struct PhysicalRead {
    _lease:File,root:PathBuf,catalog:Option<Catalog>,cache:BTreeMap<String,Rc<Index>>,cache_bytes:usize,borrowed_index_bytes:usize,
}
fn strict<T:serde::de::DeserializeOwned>(bytes:&[u8])->Result<T>{Ok(serde_json::from_value(crate::api::strict_json(std::str::from_utf8(bytes)?)?)?)}
fn gate(root:&Path,exclusive:bool)->Result<File> {
    let file=OpenOptions::new().create(true).truncate(false).read(true).write(true).open(root.join("pack-readers.lock"))?;
    let start=Instant::now();let cap=std::env::var("EXP_WAIT_MS").ok().and_then(|s|s.parse().ok()).unwrap_or(15000);
    loop {
        let result=if exclusive{FileExt::try_lock_exclusive(&file)}else{FileExt::try_lock_shared(&file)};
        if result.is_ok(){return Ok(file);}
        if std::env::var("EXP_PACK_CANCEL").ok().is_some_and(|p|Path::new(&p).exists()){return Err("cancelled:pack-barrier".into());}
        if start.elapsed()>Duration::from_millis(cap){return Err("busy:physical-readers".into());}
        std::thread::sleep(Duration::from_millis(2));
    }
}
fn config(root:&Path,m:&mut Metrics)->Result<crate::store::StoreConfig>{strict(&pack::native(&root.join("STORE.json"),65536,m)?)}
fn catalog(root:&Path,c:&crate::store::StoreConfig,m:&mut Metrics)->Result<Option<Catalog>> {
    match (c.version,c.catalog.as_deref()) {
        (1,None)=>Ok(None),
        (2,Some(hash)) if crate::objects::valid_hash(hash)=>{
            let b=pack::native(&root.join(format!("pack-catalogs/{hash}.json")),65536,m)?;
            if io::digest(&b)!=hash{return Err("integrity:catalog-digest".into());}
            m.pack_catalog_bytes+=b.len() as u64;
            let cat:Catalog=strict(&b)?;
            if cat.version!=1||cat.store_id!=c.store_id||cat.packs.len()>64{return Err("integrity:catalog-version-count".into());}
            let mut previous:Option<&Descriptor>=None;let mut hashes=BTreeSet::new();
            for d in &cat.packs {
                if !crate::objects::valid_hash(&d.pack)||!crate::objects::valid_hash(&d.index)||d.pack_bytes>pack::MAX_PACK as u64||d.index_bytes>pack::MAX_INDEX as u64||d.records==0||d.records as usize>pack::MAX_RECORDS||d.codec>1||Key::new(&d.first)?.role()!=Some(d.role.as_str())||Key::new(&d.last)?.role()!=Some(d.role.as_str())||d.first>d.last||!hashes.insert(d.index.clone()) {
                    return Err("integrity:catalog-descriptor".into());
                }
                if previous.is_some_and(|p|(&p.role,&p.first)>=(&d.role,&d.first)||p.role==d.role&&p.last>=d.first){return Err("integrity:catalog-order-overlap".into());}
                for (name,len) in [(format!("packs/{}.pack",d.pack),d.pack_bytes),(format!("packs/{}.idx",d.index),d.index_bytes)] {
                    if root.join(name).metadata().map_err(|e|format!("integrity:catalog-required-file:{e}"))?.len()!=len{return Err("integrity:catalog-size-binding".into());}
                }
                previous=Some(d);
            }
            Ok(Some(cat))
        },_=>Err("integrity:store-catalog-version".into())
    }
}
impl PhysicalRead {
    fn from_lease(root:&Path,lease:File,m:&mut Metrics)->Result<Self> {
        let c=config(root,m)?;let catalog=catalog(root,&c,m)?;
        Ok(Self{_lease:lease,root:root.into(),catalog,cache:BTreeMap::new(),cache_bytes:0,borrowed_index_bytes:0})
    }
    fn make_room(&mut self,incoming:usize,m:&mut Metrics)->Result<()> {
        let bound=self.borrowed_index_bytes.checked_add(incoming).ok_or("integrity:index-memory-overflow")?;
        if bound>pack::MAX_CACHE{return Err("integrity:index-cache-bound".into());}
        while self.cache_bytes+bound>pack::MAX_CACHE {
            let key=self.cache.keys().next().ok_or("integrity:cache-accounting")?.clone();
            self.cache_bytes-=self.cache.remove(&key).unwrap().memory();
        }
        // Charged transient upper bound, including an index being loaded and
        // a verifier's simultaneously live index, not only the final cache.
        m.pack_cache_peak_bytes=m.pack_cache_peak_bytes.max((self.cache_bytes+bound) as u64);Ok(())
    }
    fn index(&mut self,d:&Descriptor,m:&mut Metrics)->Result<Rc<Index>> {
        if let Some(index)=self.cache.get(&d.index){m.pack_cache_hits+=1;return Ok(index.clone());}
        let bound=Index::allocation_bound(d)?;
        // Evict before allocation: the loading index is included in residency.
        self.make_room(bound,m)?;
        let index=Rc::new(Index::load(&self.root,d,m)?);let bytes=index.memory();
        if bytes>bound{return Err("integrity:index-cache-estimate".into());}
        self.cache_bytes+=bytes;m.pack_cache_peak_bytes=m.pack_cache_peak_bytes.max(self.cache_bytes as u64);self.cache.insert(d.index.clone(),index.clone());Ok(index)
    }
    fn locate(&mut self,key:&Key,m:&mut Metrics)->Result<Option<(Rc<Index>,pack::Entry)>> {
        let Some(cat)=&self.catalog else{return Ok(None)};
        let desc=cat.packs.iter().find(|d|Some(d.role.as_str())==key.role()&&d.first<=key.0&&key.0<=d.last).cloned();
        if let Some(d)=desc {let index=self.index(&d,m)?;if let Some(e)=index.find(key)?{return Ok(Some((index,e)));}}
        Ok(None)
    }
}
pub(crate) fn capture(root:&Path,m:&mut Metrics)->Result<()> {
    if let Some(p)=&m.physical {
        if p.root!=root{return Err("integrity:physical-root-rebind".into());}
        return Ok(());
    }
    let lease=gate(root,false)?;io::checkpoint("pack-read-lease")?;
    m.physical=Some(PhysicalRead::from_lease(root,lease,m)?);io::checkpoint("pack-catalog-captured")
}
fn key_for(path:&Path,m:&Metrics)->Option<(PathBuf,Key)> {
    let root=m.store_root.as_ref()?;let relative=path.strip_prefix(root).ok()?;
    let name=relative.components().map(|c|c.as_os_str().to_string_lossy()).collect::<Vec<_>>().join("/");
    Key::new(&name).ok().map(|key|(root.clone(),key))
}
pub(crate) fn read(path:&Path,limit:usize,m:&mut Metrics)->Result<Option<Vec<u8>>> {
    let Some((root,key))=key_for(path,m) else{return Ok(None)};
    capture(&root,m)?;
    let mut physical=m.physical.take().ok_or("integrity:physical-capture")?;
    let result=(||->Result<Option<Vec<u8>>>{
        if let Some((index,e))=physical.locate(&key,m)?{return Ok(Some(index.read(&root,&e,limit,m)?));}Ok(None)
    })();m.physical=Some(physical);result
}
pub(crate) fn exists(path:&Path,m:&mut Metrics)->Result<bool> {
    let Some((root,key))=key_for(path,m) else{return Ok(path.exists())};capture(&root,m)?;
    let mut physical=m.physical.take().ok_or("integrity:physical-capture")?;
    let result=physical.locate(&key,m).map(|v|v.is_some()||path.exists());m.physical=Some(physical);result
}
pub(crate) fn probe(store:&Store,v:&Value,m:&mut Metrics)->Result<Value> {
    let count=v["count"].as_u64().unwrap_or(500);let requests=v["requests"].as_array().ok_or("requests")?;
    if count==0||count>5000||requests.len()>64{return Err("refused:lookup-sample-bound".into());}
    let mut rows=vec![];
    for request in requests {
        let id=request["id"].as_str().ok_or("lookup-id")?;let category=request["category"].as_str().ok_or("lookup-category")?;
        let first=Instant::now();let expected=store.resolve(id,None,m)?;let first_ns=first.elapsed().as_nanos();let mut times=Vec::with_capacity(count as usize);
        let (opens,storage,decoded,hits)=(m.opens,m.pack_storage_bytes,m.pack_decoded_bytes,m.pack_cache_hits);
        for _ in 0..count {let t=Instant::now();let result=store.resolve(id,None,m)?;times.push(t.elapsed().as_nanos());if result!=expected{return Err("integrity:lookup-outcome-changed".into());}}
        let mut sorted=times.clone();sorted.sort_unstable();
        rows.push(json!({"category":category,"id":id,"first_lookup_ns":first_ns,"warm_ns":times,"warm_median_ns":sorted[sorted.len()/2],"warm_p95_ns":sorted[(sorted.len()*95/100).min(sorted.len()-1)],"warm_min_ns":sorted[0],"warm_max_ns":sorted.last(),"result_digest":io::digest(&serde_json::to_vec(&expected)?),"opens":m.opens-opens,"pack_storage_bytes":m.pack_storage_bytes-storage,"decoded_record_bytes":m.pack_decoded_bytes-decoded,"index_cache_hits":m.pack_cache_hits-hits}));
    }
    Ok(json!({"rows":rows,"cache_qualification":"same-process warm after separately timed first lookup; device/OS cache eviction unmeasured"}))
}
fn all_mapped(m:&mut Metrics)->Result<Vec<Key>> {
    let mut physical=m.physical.take().ok_or("integrity:physical-capture")?;
    let result=(||->Result<Vec<Key>>{
        let mut keys=vec![];
        for d in physical.catalog.as_ref().map_or(vec![],|c|c.packs.clone()) {
            let index=physical.index(&d,m)?;
            for n in 0..d.records as usize{keys.push(index.entry(n)?.key);}
        }Ok(keys)
    })();m.physical=Some(physical);result
}
#[derive(Clone,Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
struct Journal { version:u32,id:String,status:String,expected_config:String,catalog:Option<String>,old:Vec<Descriptor>,reserved_bytes:u64,used_bytes:u64,deleted_bytes:u64,deferred_bytes:u64 }
fn save(root:&Path,j:&Journal,m:&mut Metrics)->Result<()> {io::atomic(&root.join("PACKING.json"),&io::json(j)?,m)}
fn authorized(root:&Path,m:&mut Metrics)->Result<()> {
    let allowed:String=strict(&pack::native(&root.join("DISPOSABLE.json"),65536,m)?)?;
    if allowed!=root.canonicalize()?.to_string_lossy(){return Err("refused:disposable-identity".into());}Ok(())
}
fn sharing_refusal(error:&(dyn std::error::Error+'static))->bool {
    #[cfg(windows)] {return error.downcast_ref::<std::io::Error>().is_some_and(|e|matches!(e.raw_os_error(),Some(32|33)));}
    #[cfg(not(windows))] {let _=error;false}
}
fn cohort(store:&Store,include_attempts:bool,repack:bool,m:&mut Metrics)->Result<BTreeSet<Key>> {
    let mut keys:BTreeSet<_>=all_mapped(m)?.into_iter().collect();let frozen=!keys.is_empty();
    if frozen&&!include_attempts&&!repack{return Ok(keys);}
    let mut attempts=BTreeMap::new();let mut live_owners=BTreeSet::new();let head=store.snapshot(m)?;
    let mut selected=store.membership_entries(m)?;selected.push((head.selected.publication.clone(),head.selected));
    for (_,s) in selected {
        let p=store.publication(&s,m)?;
        if lifetime::live(&store.root,&s).is_ok(){
            live_owners.insert(p.attempt.clone());
            if let Some(packet)=&p.packet_ref{live_owners.insert(packet.attempt.clone());}
        }
        if !frozen||keys.contains(&Key::new(&format!("publications/{}.json",p.id))?) {
            attempts.insert(p.attempt.clone(),(p.request.clone(),p.intent.clone()));
            if !frozen {
                for name in [format!("publications/{}.json",p.id),format!("results/{}.json",p.result_hash),format!("attempt-hints/{}.json",p.attempt)] {keys.insert(Key::new(&name)?);}
            }
        }
    }
    let active=lifetime::reservations(&store.root,m)?;
    if include_attempts {
        let mut paths=vec![];lifetime::files(&store.root.join("attempts"),&mut paths)?;
        for path in paths {
            let relative=path.strip_prefix(&store.root)?.components().map(|c|c.as_os_str().to_string_lossy()).collect::<Vec<_>>().join("/");
            let Ok(key)=Key::new(&relative) else{continue};
            if !matches!(key.role(),Some("candidate"|"allocation")){continue;}
            let attempt=key.0.split('/').nth(1).ok_or("integrity:attempt-key")?;
            if !attempts.contains_key(attempt)||active.contains_key(attempt)||repack&&!live_owners.contains(attempt){continue;}
            if key.role()==Some("candidate") {
                let c:crate::store::Candidate=strict(&io::read(&path,m)?)?;let (request,intent)=&attempts[attempt];
                if c.version!=2||c.purpose!="publication"||c.attempt!=attempt||c.store_id!=store.config.store_id||&c.request.request!=request||&c.intent!=intent{return Err("integrity:finalized-candidate-binding".into());}
            } else {
                #[derive(Deserialize)] #[serde(deny_unknown_fields)] struct Allocation{publication:String,result:String}
                let a:Allocation=strict(&io::read(&path,m)?)?;
                if !crate::model::safe_id(&a.publication)||!crate::objects::valid_hash(&a.result){return Err("integrity:finalized-allocation-binding".into());}
            }
            keys.insert(key);
        }
    }
    if repack {keys.retain(|key|!matches!(key.role(),Some("candidate"|"allocation"))||key.0.split('/').nth(1).is_some_and(|attempt|live_owners.contains(attempt)));}
    Ok(keys)
}
fn install(root:&Path,stage:&Path,bytes:&[u8],name:&str,hash:&str,m:&mut Metrics)->Result<()> {
    io::mkdir(stage,m)?;let private=stage.join(name);io::write_new(&private,bytes,m)?;
    let destination=root.join(format!("packs/{name}"));io::mkdir(destination.parent().unwrap(),m)?;
    io::install_original(&private,&destination,hash,m)?;io::sync_dir(destination.parent().unwrap(),m)
}
fn flush(root:&Path,stage:&Path,builder:pack::Builder,m:&mut Metrics)->Result<Descriptor> {
    let (d,pack,index)=builder.finish()?;
    io::checkpoint("pack-build")?;
    install(root,stage,&pack,&format!("{}.pack",d.pack),&d.pack,m)?;io::checkpoint("pack-index")?;
    install(root,stage,&index,&format!("{}.idx",d.index),&d.index,m)?;io::checkpoint("pack-install")?;
    let started=Instant::now();let bound=Index::allocation_bound(&d)?;
    if let Some(physical)=m.physical.as_mut(){physical.borrowed_index_bytes=bound;}
    let mut physical=m.physical.take().ok_or("integrity:verification-capture")?;physical.make_room(0,m)?;m.physical=Some(physical);
    let checked=Index::load(root,&d,m)?;
    for n in 0..d.records as usize {
        let e=checked.entry(n)?;let raw=checked.read(root,&e,pack::MAX_RAW,m)?;
        let original=io::read(&root.join(&e.key.0),m)?;
        if raw!=original{return Err("integrity:pack-complete-input-correspondence".into());}
    }
    drop(checked);if let Some(physical)=m.physical.as_mut(){physical.borrowed_index_bytes=0;}
    m.elapsed("pack-complete-verification",started);Ok(d)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn loading_indexes_evicts_before_transient_allocation(){
        let root=PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target").join(io::unique("packing-cache-boundary"));
        std::fs::create_dir_all(root.join("packs")).unwrap();
        let lease=OpenOptions::new().create_new(true).read(true).write(true).open(root.join("lease")).unwrap();
        let mut physical=PhysicalRead{_lease:lease,root:root.clone(),catalog:None,cache:BTreeMap::new(),cache_bytes:0,borrowed_index_bytes:0};let mut metrics=Metrics::default();
        let mut descriptors=vec![];
        for part in 0..5 {
            let mut builder=pack::Builder::new("allocation",0);
            for n in 0..56000 {let key=Key::new(&format!("attempts/{}-{part}/allocations/{}-{n:06}.json","a".repeat(80),"b".repeat(75))).unwrap();builder.push(key,b"").unwrap();}
            let (d,p,i)=builder.finish().unwrap();std::fs::write(root.join(format!("packs/{}.pack",d.pack)),p).unwrap();std::fs::write(root.join(format!("packs/{}.idx",d.index)),i).unwrap();descriptors.push(d);
        }
        let total:usize=descriptors.iter().map(|d|Index::allocation_bound(d).unwrap()).sum();assert!(total>pack::MAX_CACHE);
        for d in &descriptors {
            let index=physical.index(d,&mut metrics).unwrap();assert_eq!(index.entry(0).unwrap().key.0,d.first);
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
        eprintln!("cache-boundary-receipt root={} total_index_charge={} transient_charged_peak={} remaining_cache_entries={} changed_index_refused_before_allocation=true",root.display(),total,metrics.pack_cache_peak_bytes,physical.cache.len());
    }
}
pub(crate) fn run(root:&Path,v:&Value,m:&mut Metrics)->Result<Value> {
    m.store_root=Some(root.into());
    let maintenance=OpenOptions::new().create(true).truncate(false).read(true).write(true).open(root.join("pack-maintenance.lock"))?;
    FileExt::try_lock_exclusive(&maintenance).map_err(|e|format!("busy:competing-maintenance:{e}"))?;
    let store=Store::open(root,m)?;authorized(root,m)?;
    let operation=v["operation"].as_str().ok_or("operation")?;
    if operation=="status"{return Ok(json!({"journal":if root.join("PACKING.json").exists(){Some(strict::<Journal>(&pack::native(&root.join("PACKING.json"),65536,m)?)?)}else{None}}));}
    if operation=="abort" {
        let mut j:Journal=strict(&pack::native(&root.join("PACKING.json"),65536,m)?)?;
        if j.status!="building"{return Err("refused:only-incomplete-unselected-build-abort".into());}
        j.status="aborted-owned".into();save(root,&j,m)?;return Ok(json!({"status":j.status,"allocation_released":false}));
    }
    let protection=store.preparation_gate(false)?;
    let mut journal;
    if operation=="build"||operation=="repack" {
        if root.join("PACKING.json").exists() {
            let old:Journal=strict(&pack::native(&root.join("PACKING.json"),65536,m)?)?;
            if !["registered","complete","aborted-owned"].contains(&old.status.as_str()){return Err("refused:unfinished-maintenance-resume-or-abort".into());}
            io::mkdir(&root.join("packing-journals"),m)?;io::write_new(&root.join(format!("packing-journals/{}.json",old.id)),&io::json(&old)?,m)?;
        }
        let codec=match v["codec"].as_str().unwrap_or("raw"){"raw"=>0,"zstd"=>1,_=>return Err("refused:codec".into())};
        let construction=Instant::now();let guard=store.lock(m)?;lifetime::recover_transfers(&store,m)?;
        let expected_config=io::digest(&pack::native(&root.join("STORE.json"),65536,m)?);
        let keys=cohort(&store,v["include_attempts"].as_bool().unwrap_or(false),operation=="repack",m)?;let old=m.physical.as_ref().and_then(|p|p.catalog.as_ref()).map_or(vec![],|c|c.packs.clone());drop(guard);
        let reserve=v["reserve_bytes"].as_u64().unwrap_or(256*1024*1024);
        if reserve>4*1024*1024*1024||reserve<1024*1024{return Err("refused:maintenance-reservation-bound".into());}
        journal=Journal{version:1,id:io::unique("packing"),status:"building".into(),expected_config,catalog:None,old,reserved_bytes:reserve,used_bytes:0,deleted_bytes:0,deferred_bytes:0};
        io::mkdir(&root.join("packing-owners"),m)?;io::write_new(&root.join(format!("packing-owners/{}.json",journal.id)),&io::json(&journal)?,m)?;
        save(root,&journal,m)?;io::checkpoint("pack-reservation")?;
        m.quota_limit=Some(reserve);m.quota_spent=0;
        let stage=root.join(format!("packing-stages/{}",journal.id));let mut packs=vec![];let mut excluded=0u64;let mut excluded_bytes=0u64;
        // Role then key ordering keeps permanent and content-lifetime containers separate.
        let mut ordered:Vec<_>=keys.into_iter().collect();ordered.sort_by(|a,b|(a.role(),&a.0).cmp(&(b.role(),&b.0)));
        let mut builder:Option<pack::Builder>=None;let mut role=String::new();let mut logical=0u64;let mut records=0u64;
        for key in ordered {
            let raw=io::read(&root.join(&key.0),m)?;
            if raw.len()>pack::MAX_RAW{excluded+=1;excluded_bytes+=raw.len() as u64;continue;}
            if root.join(&key.0).exists() {
                let loose=pack::native(&root.join(&key.0),128*1024*1024,m)?;
                if loose!=raw{return Err("integrity:conflicting-loose-duplicate".into());}
            }
            if builder.as_ref().is_some_and(|b|role!=key.role().unwrap()||!b.fits(&key,raw.len())) {
                packs.push(flush(root,&stage,builder.take().unwrap(),m)?);
            }
            if builder.is_none(){role=key.role().unwrap().into();builder=Some(pack::Builder::new(&role,codec));}
            logical+=raw.len() as u64;records+=1;builder.as_mut().unwrap().push(key,&raw)?;
        }
        if let Some(b)=builder{if !b.is_empty(){packs.push(flush(root,&stage,b,m)?);}}
        if packs.len()>64{return Err("refused:registered-pack-cap".into());}
        let cat=Catalog{version:1,store_id:store.config.store_id.clone(),packs};let bytes=io::json(&cat)?;
        if bytes.len()>65536{return Err("refused:catalog-size".into());}
        let hash=io::digest(&bytes);io::mkdir(&root.join("pack-catalogs"),m)?;
        let path=root.join(format!("pack-catalogs/{hash}.json"));
        if path.exists(){if pack::native(&path,65536,m)?!=bytes{return Err("integrity:catalog-conflict".into());}}
        else{io::write_new(&path,&bytes,m)?;}
        let mut candidate=store.config.clone();candidate.version=2;candidate.catalog=Some(hash.clone());catalog(root,&candidate,m)?;
        journal.catalog=Some(hash);journal.status="installed".into();journal.used_bytes=m.quota_spent;m.quota_limit=None;save(root,&journal,m)?;
        m.elapsed("pack-construction-inclusive",construction);io::checkpoint("pack-before-catalog")?;
        m.pack_eligible_records=records;m.pack_eligible_raw_bytes=logical;m.pack_excluded_records=excluded;m.pack_excluded_bytes=excluded_bytes;
    } else if operation=="resume"||operation=="cleanup" {
        journal=strict(&pack::native(&root.join("PACKING.json"),65536,m)?)?;
        if journal.version!=1||journal.catalog.is_none()||!["installed","registered","cleaning","complete"].contains(&journal.status.as_str()){return Err("refused:incomplete-build-owned".into());}
    } else{return Err("refused:packing-operation".into());}
    // Never hold writer.lock while draining API readers (which may need that writer).
    m.physical=None;io::checkpoint("pack-reader-drain")?;let drain=Instant::now();let exclusive=gate(root,true)?;m.elapsed("pack-reader-drain",drain);let registration=Instant::now();
    let guard=store.lock(m)?;let mut c=config(root,m)?;let current_hash=io::digest(&pack::native(&root.join("STORE.json"),65536,m)?);
    let target=journal.catalog.clone().ok_or("integrity:maintenance-catalog")?;
    if c.catalog.as_deref()!=Some(&target) {
        if journal.status!="installed"||current_hash!=journal.expected_config{return Err("conflict:physical-catalog".into());}
        c.version=2;c.catalog=Some(target.clone());catalog(root,&c,m)?;
        io::atomic(&root.join("STORE.json"),&io::json(&c)?,m)?;io::checkpoint("pack-catalog")?;
    }
    m.physical=Some(PhysicalRead::from_lease(root,exclusive,m)?);
    journal.status="registered".into();save(root,&journal,m)?;io::checkpoint("pack-accounting")?;m.elapsed("pack-registration-inclusive",registration);
    let mut retained_loose_control_records=0u64;let mut retained_loose_control_bytes=0u64;
    if operation=="cleanup"||v["cleanup"].as_bool()==Some(true) {
        let deletion=Instant::now();journal.status="cleaning".into();save(root,&journal,m)?;
        let keys=all_mapped(m)?;let mut deferred=0u64;
        for key in keys {
            // Ordinary B readers capture SELECTED once and follow its loose
            // publication to a generation. Historical selectors need the same
            // binding. These permanent control duplicates survive retirement;
            // generation content still has its independent existing lifetime.
            let control=c.mode=="B"&&key.role()==Some("publication");
            let path=root.join(&key.0);if !path.exists(){
                if control{return Err("integrity:B-loose-control-publication-missing".into());}
                continue;
            }
            let canonical=io::read(&path,m)?;let loose=pack::native(&path,pack::MAX_RAW,m)?;
            if canonical!=loose{return Err("integrity:cleanup-conflicting-duplicate".into());}
            if control{retained_loose_control_records+=1;retained_loose_control_bytes+=loose.len() as u64;continue;}
            match lifetime::remove_checked(root,&path,m) {
                Ok(n)=>{journal.deleted_bytes+=n;io::checkpoint("pack-loose-delete")?;},
                Err(e)=>{
                    if sharing_refusal(e.as_ref())&&path.exists(){deferred+=path.metadata()?.len();eprintln!("deferred-delete:{}:{e}",path.display());continue;}
                    return Err(e);
                }
            }
        }
        let new=m.physical.as_ref().and_then(|p|p.catalog.as_ref()).ok_or("integrity:cleanup-catalog")?.packs.clone();
        for old in &journal.old {
            for (name,retained) in [(format!("packs/{}.pack",old.pack),new.iter().any(|d|d.pack==old.pack)),(format!("packs/{}.idx",old.index),new.iter().any(|d|d.index==old.index))] {
                let path=root.join(name);if retained||!path.exists(){continue;}
                match lifetime::remove_checked(root,&path,m) {
                    Ok(n)=>{journal.deleted_bytes+=n;io::checkpoint("pack-old-delete")?;},
                    Err(e)=>{
                        if sharing_refusal(e.as_ref())&&path.exists(){deferred+=path.metadata()?.len();eprintln!("deferred-old-delete:{}:{e}",path.display());continue;}
                        return Err(e);
                    }
                }
            }
        }
        journal.deferred_bytes=deferred;journal.status=if deferred==0{"complete"}else{"cleaning"}.into();save(root,&journal,m)?;m.elapsed("pack-deletion-inclusive",deletion);
    }
    drop(guard);drop(protection);Ok(json!({"status":journal.status,"catalog":target,"reserved_bytes":journal.reserved_bytes,"used_bytes":journal.used_bytes,"successfully_deleted_bytes_recorded":journal.deleted_bytes,"deferred_bytes":journal.deferred_bytes,"retained_loose_control_records":retained_loose_control_records,"retained_loose_control_bytes":retained_loose_control_bytes,"retained_loose_control_scope":"mapped B publication duplicates; excludes new loose tail; no deletion credit","historical_allocation_owners_unchanged":true}))
}
