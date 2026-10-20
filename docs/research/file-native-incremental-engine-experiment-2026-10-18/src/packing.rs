//! Physical locations only. Semantic selection and content lifetime stay in Store.
use crate::{io::{self,Metrics,Result},pack::{self,Descriptor,Index,Key},store::Store,lifetime};
use fs2::FileExt;
use crate::physical_lifetime as physical;
use serde::{Serialize,Deserialize};
use serde_json::{json,Value};
use std::{collections::{BTreeMap,BTreeSet},fs::{File,OpenOptions},path::{Path,PathBuf},rc::Rc,time::{Duration,Instant}};

#[derive(Clone,Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
struct Catalog { version:u32,store_id:String,packs:Vec<Descriptor> }
pub(crate) struct PhysicalRead {
    _lease:File,root:PathBuf,config:crate::store::StoreConfig,catalog:Option<Catalog>,cache:BTreeMap<String,Rc<Index>>,cache_bytes:usize,borrowed_index_bytes:usize,
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
            if cat.version!=(if c.version==3{2}else{1})||cat.store_id!=c.store_id||cat.packs.len()>64{return Err("integrity:catalog-version-count".into());}
            let mut previous:Option<&Descriptor>=None;let mut hashes=BTreeSet::new();
            for d in &cat.packs {
                if c.version==2&&d.instance.is_some()||c.version==3&&d.instance.is_none()||!crate::objects::valid_hash(&d.pack)||!crate::objects::valid_hash(&d.index)||d.pack_bytes>pack::MAX_PACK as u64||d.index_bytes>pack::MAX_INDEX as u64||d.records==0||d.records as usize>pack::MAX_RECORDS||d.codec>1||Key::new(&d.first)?.role()!=Some(d.role.as_str())||Key::new(&d.last)?.role()!=Some(d.role.as_str())||d.first>d.last||!hashes.insert(d.index.clone()) {
                    return Err("integrity:catalog-descriptor".into());
                }
                if previous.is_some_and(|p|(&p.role,&p.first)>=(&d.role,&d.first)||p.role==d.role&&p.last>=d.first){return Err("integrity:catalog-order-overlap".into());}
                for (name,len) in [(d.path("pack")?,d.pack_bytes),(d.path("idx")?,d.index_bytes)] {
                    m.cleanup_step()?;if root.join(name).metadata().map_err(|e|format!("integrity:catalog-required-file:{e}"))?.len()!=len{return Err("integrity:catalog-size-binding".into());}
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
        Ok(Self{_lease:lease,root:root.into(),config:c,catalog,cache:BTreeMap::new(),cache_bytes:0,borrowed_index_bytes:0})
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
    let (c,lease)=physical::capture(root,m)?;
    m.physical=Some(PhysicalRead::from_lease(root,lease,c,m)?);
    io::checkpoint("pack-read-lease")?;io::checkpoint("pack-catalog-captured")
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
    let id=stage.file_name().and_then(|s|s.to_str()).ok_or("integrity:birth-id")?;
    if !crate::model::safe_id(id){return Err("integrity:birth-id".into());}
    let destination=root.join(format!("packs/{id}/{name}"));
    // Birth ids are never reused, including identical content. A failed build
    // owns this directory until explicit abort/reaping; no existing-path adopt.
    if io::digest(bytes)!=hash{return Err("integrity:install-container-hash".into());}
    io::write_new(&destination,bytes,m)
}
fn flush(root:&Path,stage:&Path,builder:pack::Builder,m:&mut Metrics)->Result<Descriptor> {
    let (mut d,pack,index)=builder.finish()?;
    d.instance=Some(stage.file_name().and_then(|s|s.to_str()).ok_or("integrity:birth-id")?.into());
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
        let root=std::env::var_os("EXP_PACK_TEST_ROOT").map(PathBuf::from).unwrap_or_else(||PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target")).join(io::unique("packing-cache-boundary"));
        std::fs::create_dir_all(root.join("packs")).unwrap();
        let lease=OpenOptions::new().create_new(true).read(true).write(true).open(root.join("lease")).unwrap();
        let mut physical=PhysicalRead{_lease:lease,root:root.clone(),config:crate::store::StoreConfig{version:1,store_id:"cache-test".into(),mode:"A".into(),consumer:"project".into(),fallback:false,navigation_version:4,catalog:None,physical:None},catalog:None,cache:BTreeMap::new(),cache_bytes:0,borrowed_index_bytes:0};let mut metrics=Metrics::default();
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

fn selected_packs(m:&Metrics)->Vec<Descriptor>{m.physical.as_ref().and_then(|p|p.catalog.as_ref()).map_or(vec![],|c|c.packs.clone())}
fn blank_work(c:&crate::store::StoreConfig,id:&str,ordinal:u64,packs:Vec<Descriptor>,hash:&str,parent:&str)->physical::Work{
    physical::Work{version:2,store_id:c.store_id.clone(),physical_parent:parent.into(),transition_parent:None,birth_claims:vec![],generation:physical::Generation{id:id.into(),ordinal,catalog:hash.into(),plan:None,packs},older:vec![],jobs:vec![],retired:vec![],garbage:vec![],removed_bytes:0,removed_files:0,reconciled_absent:0}
}
fn append_work(root:&Path,c:&crate::store::StoreConfig,id:&str,packs:Vec<Descriptor>,hash:&str,old:&[Descriptor],parent:&str,m:&mut Metrics)->Result<physical::Work>{
    let ordinal=next_ordinal(c)?;
    let mut w=if c.version==3{physical::load(root,c,m)?}else{blank_work(c,id,ordinal,vec![],hash,parent)};
    if c.version==3{w.older.push(physical::generation(c,old.to_vec())?);}
    else if let Some(previous)=&c.catalog{w.garbage.push(format!("pack-catalogs/{previous}.json"));}
    for d in old{physical::add_retired(&mut w,d.clone());}
    w.physical_parent=parent.into();
    w.generation=physical::Generation{id:id.into(),ordinal,catalog:hash.into(),plan:None,packs:packs.clone()};
    // A committed physical successor advances the ordinal before old claims
    // disappear. Unselected candidates cannot clear the parent authority.
    w.birth_claims.clear();w.transition_parent=None;
    w.garbage.push(format!("physical-stages/{id}/STORE.json"));
    for d in packs{w.jobs.push(physical::LooseJob{descriptor:d,frontier:ordinal,position:0});}
    physical::validate_work(&w)?;Ok(w)
}
fn next_ordinal(c:&crate::store::StoreConfig)->Result<u64>{
    c.physical.as_ref().map_or(Some(1),|b|b.ordinal.checked_add(1)).ok_or_else(||"quota:physical-generation-overflow".into())
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
        if directory.exists(){
            m.cleanup_step()?;for (n,e) in std::fs::read_dir(&directory)?.enumerate(){
                m.cleanup_step()?;
                if n>=128{return Err("quota:orphan-container-bound".into());}
                let e=e?;if !e.file_type()?.is_file(){return Err("integrity:orphan-container-type".into());}
                let name=e.file_name().to_str().ok_or("integrity:orphan-container-name")?.to_owned();
                w.garbage.push(format!("packs/{}/{name}",owner.id));
            }
        }
        if let Some(hash)=&owner.catalog{w.garbage.push(format!("pack-catalogs/{hash}.json"));}
        if let Some(plan)=&owner.plan{
            let path=root.join(format!("physical-plans/{plan}.json"));
            if path.exists(){
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
    for d in &cat.packs {
        let bound=Index::allocation_bound(d)?;
        if let Some(p)=m.physical.as_mut(){p.borrowed_index_bytes=bound;}
        let mut p=m.physical.take().ok_or("integrity:activation-capture")?;p.make_room(0,m)?;m.physical=Some(p);
        let index=Index::load(root,d,m)?;
        for n in 0..d.records as usize{let e=index.entry(n)?;index.read(root,&e,pack::MAX_RAW,m)?;}
        drop(index);if let Some(p)=m.physical.as_mut(){p.borrowed_index_bytes=0;}
    }
    physical::load(root,target,m)?;
    let stage=root.join(format!("physical-stages/{}/STORE.json",j.id));
    let target_bytes=io::json(target)?;
    if target_bytes.len()>physical::BINDING_LIMIT{return Err("quota:physical-binding-bound".into());}
    if stage.exists(){if pack::native(&stage,physical::BINDING_LIMIT,m)?!=target_bytes{return Err("integrity:activation-stage".into());}}
    else{io::write_new(&stage,&target_bytes,m)?;}
    // A builder's old pin must end before a one-time legacy drain. M prevents
    // another maintenance owner from replacing/reclaiming its input meanwhile.
    m.physical=None;io::checkpoint("pack-reader-drain")?;
    let wait=Instant::now();let cap=std::env::var("EXP_WAIT_MS").ok().and_then(|s|s.parse().ok()).unwrap_or(15000);
    loop {
        if physical::cancelled(){return Err("cancelled:pack-barrier".into());}
        let q=physical::wait_lock(root,"physical-capture.lock",true)?;
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
        let Some(writer)=store.try_writer(m)? else{
            drop(legacy);drop(q);
            record_activation_hold(m,hold)?;
            if wait.elapsed()>Duration::from_millis(cap){return Err("busy:activation-writer".into());}
            std::thread::sleep(Duration::from_millis(2));continue;
        };
        let result=io::replace(&stage,&root.join("STORE.json"),m).and_then(|_|io::sync_dir(root,m));
        drop(writer);drop(legacy);drop(q);record_activation_hold(m,hold)?;m.elapsed("physical-activation-hold",hold);m.elapsed("physical-activation-wait-inclusive",wait);
        // No injected barrier or journal I/O runs under Q/W.
        result?;io::checkpoint("physical-after-store-replace")?;std::fs::remove_dir(stage.parent().ok_or("integrity:stage-parent")?)?;io::checkpoint("pack-catalog")?;io::checkpoint("physical-after-selection")?;return Ok(());
    }
}
fn try_remove(root:&Path,name:&str,w:&mut physical::Work,m:&mut Metrics)->Result<bool>{
    m.cleanup_step()?;let path=root.join(name);if !present(&path)?{w.reconciled_absent+=1;return Ok(true);}
    io::checkpoint("physical-before-delete")?;m.cleanup_step()?;let started=Instant::now();
    if name.strip_prefix("packs/").is_some_and(crate::model::safe_id){
        let removed=std::fs::remove_dir(&path);m.elapsed("physical-directory-remove-native",started);
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
fn present(path:&Path)->Result<bool>{
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
        m.cleanup_step()?;let index_started=Instant::now();let index=Index::load(root,&job.descriptor,m)?;m.elapsed("cleanup-legacy-whole-index-indivisible",index_started);
        if !m.cleanup_available(){return Err("deferred:legacy-whole-index-budget".into());}
        while w.jobs[n].position<job.descriptor.records&&m.cleanup_available(){
            if physical::cancelled(){physical::save(root,binding,&mut w,m)?;return Err("cancelled:physical-cleanup".into());}
            candidates+=1;m.cleanup_step()?;let e=index.entry(w.jobs[n].position as usize)?;let path=root.join(&e.key.0);
            if c.mode=="B"&&e.key.role()==Some("publication"){
                m.cleanup_step()?;let record_started=Instant::now();let raw=index.read(root,&e,pack::MAX_RAW,m)?;m.elapsed("cleanup-legacy-record-indivisible",record_started);m.cleanup_step()?;let loose=pack::native(&path,pack::MAX_RAW,m)?;
                if raw!=loose{return Err("integrity:B-loose-control-publication-missing-or-conflict".into());}
                loose_controls+=1;loose_control_bytes+=loose.len() as u64;w.jobs[n].position+=1;continue;
            }
            if !present(&path)?{w.reconciled_absent+=1;w.jobs[n].position+=1;continue;}
            // A successor may omit retired candidate/allocation records. Such
            // loose names remain with semantic GC; do not delete via a stale job.
            let mut p=m.physical.take().ok_or("integrity:cleanup-capture")?;
            m.cleanup_step()?;let locate_started=Instant::now();let replacement=p.locate(&e.key,m);m.physical=Some(p);m.elapsed("cleanup-legacy-location-indivisible",locate_started);let replacement=replacement?;m.cleanup_step()?;
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
            if let Some(id)=&d.instance{let dir=root.join(format!("packs/{id}"));match std::fs::remove_dir(&dir){Ok(())=>{},Err(e) if matches!(e.kind(),std::io::ErrorKind::DirectoryNotEmpty|std::io::ErrorKind::NotFound)=>{},Err(e)=>return Err(e.into())}}
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
                m.cleanup_step()?;match std::fs::remove_dir(dir){Ok(())=>{},Err(e) if e.kind()==std::io::ErrorKind::NotFound=>{},Err(e) if name.starts_with("physical-stages/")&&e.kind()==std::io::ErrorKind::DirectoryNotEmpty=>{blocked+=1;n+=1;continue;},Err(e) if e.kind()==std::io::ErrorKind::DirectoryNotEmpty=>{},Err(e)=>return Err(e.into())}
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
    let previous=if root.join("PACKING.json").exists(){Some(strict::<Journal>(&pack::native(&root.join("PACKING.json"),65536,m)?)?)}else{None};
    if operation=="status"{
        return Ok(json!({"journal":previous,"selected_work":if store.config.version==3{Some(physical::load(root,&store.config,m)?)}else{None}}));
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
    if operation=="build"||operation=="repack"{
        if let Some(old)=&previous{
            let selected=store.config.physical.as_ref().map(|b|&b.id)==Some(&old.id);
            if !selected&&!["registered","cleaning","complete","aborted-owned"].contains(&old.status.as_str()){return Err("refused:unfinished-maintenance-resume-or-abort".into());}
        }
        let construction=Instant::now();
        // Reservation transfer recovery remains with semantic writers. A physical
        // build conservatively excludes still-reserved attempts; it must not
        // authenticate legacy indexes while holding W merely to release them.
        // Checkpoint 1 deliberately retains legacy discovery. It is not an
        // incremental algorithm and must be replaced before large rolling cells.
        let expected_bytes=pack::native(&root.join("STORE.json"),physical::BINDING_LIMIT,m)?;
        let expected:crate::store::StoreConfig=strict(&expected_bytes)?;
        if io::json(&expected)?!=io::json(&store.config)?{return Err("conflict:physical-capture-parent".into());}
        let expected_config=io::digest(&expected_bytes);
        let keys=cohort(&store,v["include_attempts"].as_bool().unwrap_or(false),operation=="repack",m)?;
        let old=selected_packs(m);let reserve=v["reserve_bytes"].as_u64().unwrap_or(256*1024*1024);
        if reserve>4*1024*1024*1024||reserve<1024*1024{return Err("refused:maintenance-reservation-bound".into());}
        let codec=match v["codec"].as_str().unwrap_or("raw"){"raw"=>0,"zstd"=>1,_=>return Err("refused:codec".into())};
        let ordinal=next_ordinal(&store.config)?;
        let birth=format!("physical-{ordinal}-{}",io::unique("birth"));
        if store.config.version==3&&physical::load(root,&store.config,m)?.birth_claims.contains(&birth){return Err("integrity:physical-birth-id-reuse".into());}
        if previous.as_ref().is_some_and(|old|old.id==birth||old.abandoned.iter().any(|owner|owner.id==birth))||root.join(format!("packs/{birth}")).exists()||root.join(format!("physical-pins/{birth}.lock")).exists()||root.join(format!("physical-stages/{birth}")).exists()||root.join(format!("physical-progress/{birth}.work-stage")).exists()||root.join(format!("physical-progress/{birth}.selector-stage")).exists(){return Err("integrity:physical-birth-id-reuse".into());}
        let abandoned=previous.as_ref().map(abandoned_owners).transpose()?.unwrap_or_default();
        if abandoned.len()>=64{return Err("quota:maintenance-owner-bound-cleanup-first".into());}
        j=Journal{version:2,id:birth,status:"building".into(),expected_config,catalog:None,old:old.clone(),reserved_bytes:reserve,used_bytes:0,deleted_bytes:0,deferred_bytes:0,target:None,abandoned,ownership_transferred:false};
        save(root,&j,m)?;io::checkpoint("pack-reservation")?;m.quota_limit=Some(reserve);m.quota_spent=0;
        let stage=root.join(format!("physical-stages/{}",j.id));let mut packs=vec![];let mut excluded=0u64;let mut excluded_bytes=0u64;
        let mut ordered:Vec<_>=keys.into_iter().collect();ordered.sort_by(|a,b|(a.role(),&a.0).cmp(&(b.role(),&b.0)));
        let mut builder:Option<pack::Builder>=None;let mut role=String::new();let mut logical=0u64;let mut records=0u64;
        for key in ordered{
            let raw=io::read(&root.join(&key.0),m)?;
            if raw.len()>pack::MAX_RAW{excluded+=1;excluded_bytes+=raw.len() as u64;continue;}
            if root.join(&key.0).exists()&&pack::native(&root.join(&key.0),128*1024*1024,m)?!=raw{return Err("integrity:conflicting-loose-duplicate".into());}
            if builder.as_ref().is_some_and(|b|role!=key.role().unwrap()||!b.fits(&key,raw.len())){packs.push(flush(root,&stage,builder.take().unwrap(),m)?);}
            if builder.is_none(){role=key.role().unwrap().into();builder=Some(pack::Builder::new(&role,codec));}
            logical+=raw.len() as u64;records+=1;builder.as_mut().unwrap().push(key,&raw)?;
        }
        if let Some(b)=builder{if !b.is_empty(){packs.push(flush(root,&stage,b,m)?);}}
        if packs.len()>64{return Err("refused:registered-pack-cap".into());}
        let cat=Catalog{version:2,store_id:store.config.store_id.clone(),packs:packs.clone()};let bytes=io::json(&cat)?;
        if bytes.len()>physical::CATALOG_LIMIT{return Err("refused:catalog-size".into());}
        let hash=io::digest(&bytes);j.catalog=Some(hash.clone());save(root,&j,m)?;
        io::write_new(&root.join(format!("pack-catalogs/{hash}.json")),&bytes,m)?;
        let mut work=append_work(root,&store.config,&j.id,packs,&hash,&old,&j.expected_config,m)?;
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
        m.elapsed("pack-construction-inclusive",construction);m.pack_eligible_records=records;m.pack_eligible_raw_bytes=logical;m.pack_excluded_records=excluded;m.pack_excluded_bytes=excluded_bytes;
        io::checkpoint("pack-before-catalog")?;io::checkpoint("physical-before-selection")?;
    }else if operation=="resume"{
        j=previous.ok_or("refused:no-maintenance")?;
        if j.version!=2||j.target.is_none()||!["installed","registered","cleaning","complete"].contains(&j.status.as_str()){return Err("refused:incomplete-build-owned".into());}
    }else{return Err("refused:packing-operation".into());}
    activate(root,&store,&j,m)?;m.physical=None;capture(root,m)?;let selected=captured_config(m)?;
    if selected.physical.as_ref().map(|b|&b.id)!=Some(&j.id){return Err("conflict:physical-catalog".into());}
    j.status="registered".into();save(root,&j,m)?;io::checkpoint("pack-accounting")?;drop(protection);
    if v["cleanup"].as_bool()==Some(true){return cleanup(root,&selected,m);}
    Ok(json!({"status":"registered","catalog":selected.catalog,"generation":selected.physical,"reserved_bytes":j.reserved_bytes,"used_bytes":j.used_bytes,"successfully_deleted_bytes_recorded":0,"historical_allocation_owners_unchanged":true}))
}
