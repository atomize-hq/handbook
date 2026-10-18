//! Bounded live reservations; permanent terminal evidence is point addressed.
use crate::{io::{self,Metrics,Result},model::Selector,store::{Store,Candidate,Publication},objects};
use serde::{Serialize,Deserialize};
use serde_json::{json,Value};
use std::{collections::{BTreeMap,BTreeSet},fs,path::{Path,PathBuf}};
const MAX_RESERVATIONS:usize=8;
const MAX_BYTES:u64=32*1024*1024*1024;
#[derive(Clone,Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Reservation{pub request:String,pub intent:String,pub base:Option<Selector>,pub bytes:u64,pub state:String,pub used:u64,pub candidate_digest:Option<String>}
pub fn reservations(root:&Path,m:&mut Metrics)->Result<BTreeMap<String,Reservation>>{
    let p=root.join("RESERVATIONS.json");
    if !p.exists(){return Ok(BTreeMap::new());}
    let r:BTreeMap<String,Reservation>=serde_json::from_slice(&io::read_bounded(&p,65536,m)?)?;
    if r.len()>MAX_RESERVATIONS||r.values().map(|r|r.bytes as u128).sum::<u128>()>MAX_BYTES as u128{
        return Err("integrity:quota-registry".into());
    }Ok(r)
}
fn write_reservations(root:&Path,r:&BTreeMap<String,Reservation>,m:&mut Metrics)->Result<()>{
    io::atomic(&root.join("RESERVATIONS.json"),&io::json(r)?,m)
}
pub fn reserve(root:&Path,attempt:&str,r:Reservation,m:&mut Metrics)->Result<()>{
    let mut all=reservations(root,m)?;
    if all.len()>=MAX_RESERVATIONS||all.values().map(|r|r.bytes as u128).sum::<u128>()+r.bytes as u128>MAX_BYTES as u128{
        return Err("quota:admission-full".into());
    }
    if all.insert(attempt.into(),r).is_some(){return Err("integrity:attempt-reuse".into());}
    write_reservations(root,&all,m)?;io::checkpoint("reservation-durable")
}
pub fn mark_sealed(root:&Path,attempt:&str,m:&mut Metrics)->Result<()>{
    let mut all=reservations(root,m)?;let r=all.get_mut(attempt).ok_or("integrity:missing-reservation")?;
    r.state="sealed".into();r.used=m.quota_spent.checked_add(65536).ok_or("quota:overflow")?;
    r.candidate_digest=Some(io::digest(&io::read(&root.join(format!("attempts/{attempt}/candidate.json")),m)?));
    write_reservations(root,&all,m)?;io::checkpoint("seal")
}
pub fn selection_grant(root:&Path,attempt:&str,m:&mut Metrics)->Result<()>{
    const GRANT:u64=8*1024*1024;
    let mut all=reservations(root,m)?;let r=all.get_mut(attempt).ok_or("integrity:missing-reservation")?;
    r.used=r.used.checked_add(GRANT+65536).ok_or("quota:overflow")?;
    if r.used>r.bytes{return Err("quota:selection-reservation-exhausted".into());}
    m.quota_limit=None;write_reservations(root,&all,m)?;
    m.quota_limit=Some(GRANT);m.quota_spent=0;io::checkpoint("selection-budget")
}
pub fn transfer(root:&Path,attempt:&str,m:&mut Metrics)->Result<()>{
    io::checkpoint("accounting-transfer")?;
    let mut all=reservations(root,m)?;all.remove(attempt);
    write_reservations(root,&all,m)?;io::checkpoint("accounting-release")
}
pub fn recover_transfers(store:&Store,m:&mut Metrics)->Result<()>{
    let all=reservations(&store.root,m)?;
    for (attempt,r) in all{
        if let Some(found)=store.accepted(&r.request,Some(&r.intent),m)?{
            // A different attempt accepting the same request does not transfer
            // this attempt's private allocations. Keep those charged.
            if found.attempt==attempt{
                let p=store.member(&found.selected,m)?;
                if p.allocation_owner.attempt!=attempt||p.allocation_owner.conservative_bytes<r.bytes||!p.allocation_owner.includes_intermediates_and_stages{
                    return Err("integrity:allocation-transfer-binding".into());
                }
                transfer(&store.root,&attempt,m)?;
            }
        }
    }Ok(())
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
    for e in fs::read_dir(dir)?{
        let p=e?.path();let md=fs::symlink_metadata(&p)?;
        if md.file_type().is_symlink(){return Err("refused:symlink".into());}
        if md.is_dir(){files(&p,out)?;}else{out.push(p);}
    }Ok(())
}
pub fn remove_checked(root:&Path,path:&Path,m:&mut Metrics)->Result<u64>{
    let base=root.canonicalize()?;let resolved=path.canonicalize()?;
    if resolved==base||!resolved.starts_with(&base)||fs::symlink_metadata(path)?.file_type().is_symlink(){
        return Err("refused:unsafe-delete".into());
    }
    let n=path.metadata()?.len();fs::remove_file(path)?;io::sync_dir(path.parent().unwrap(),m)?;m.namespace_ops+=1;Ok(n)
}
pub fn run(store:&Store,v:&Value,m:&mut Metrics)->Result<Value>{
    let _guard=store.lock(m)?;let root=&store.root;
    let head=store.head_unleased(m)?;
    let op=v["operation"].as_str().ok_or("operation")?;
    if op=="quota"{return Ok(json!({"reservations":reservations(root,m)?}));}
    if op=="setup"{
        if !root.components().any(|c|c.as_os_str()=="disposable"){return Err("refused:disposable-root".into());}
        io::atomic(&root.join("DISPOSABLE.json"),&io::json(&root.canonicalize()?.to_string_lossy())?,m)?;
        return Ok(json!({"status":"disposable"}));
    }
    if op=="abandon"{
        let gate=store.preparation_gate(true)?;
        let attempt=v["attempt"].as_str().ok_or("attempt")?;
        if !crate::model::safe_id(attempt){return Err("refused:attempt".into());}
        let mut all=reservations(root,m)?;let r=all.get_mut(attempt).ok_or("refused:no-reservation")?;
        if store.accepted(&r.request,Some(&r.intent),m)?.is_some_and(|a|a.attempt==attempt){return Err("refused:selected-attempt".into());}
        io::atomic(&root.join(format!("abandoned/{attempt}")),b"fenced",m)?;io::checkpoint("abandonment-fenced")?;
        r.state="abandoned".into();write_reservations(root,&all,m)?;drop(gate);
        return Ok(json!({"status":"abandoned-charged-until-gc"}));
    }
    if op=="collect"{return collect(store,m);}
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
fn collect(store:&Store,m:&mut Metrics)->Result<Value>{
    let _gate=store.preparation_gate(true)?;let root=&store.root;
    let allowed:String=serde_json::from_slice(&io::read(&root.join("DISPOSABLE.json"),m)?)?;
    if allowed!=root.canonicalize()?.to_string_lossy(){return Err("refused:disposable-identity".into());}
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
    for (g,projection) in contents{
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
        for entry in fs::read_dir(dir)?{
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
    Ok(json!({"status":"collected","files":removed,"logical_bytes":bytes,"accepted_evidence_preserved":true}))
}
