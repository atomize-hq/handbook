//! Disposable-fixture retention proof. All operations share the publication lock.
//! Publication records/outcomes/fences are never collected. No TTL or dead-reader inference.
use crate::{io::{self,Metrics,Result},model::*,backend};
use serde::{Serialize,Deserialize};
use serde_json::{json,Value};
use std::{path::Path,collections::{BTreeMap,BTreeSet},fs};
#[derive(Default,Serialize,Deserialize)]
struct Registry{pins:BTreeMap<String,Selector>,packets:BTreeMap<String,Selector>,abandoned:BTreeSet<String>}
fn get(root:&Path,m:&mut Metrics)->Result<Registry>{let p=root.join("RETENTION.json");if p.exists(){Ok(serde_json::from_slice(&io::read(&p,m)?)?)}else{Ok(Registry::default())}}
fn put(root:&Path,r:&Registry,m:&mut Metrics)->Result<()>{io::atomic(&root.join("RETENTION.json"),&io::json(r)?,m)}
pub fn check_live(root:&Path,c:&Candidate)->Result<()>{
    if root.join("retired-attempts").join(format!("{}.json",c.attempt)).exists(){return Err("retired:snapshot".into());}Ok(())
}
pub fn check_candidate(root:&Path,c:&Candidate,m:&mut Metrics)->Result<()>{if get(root,m)?.abandoned.contains(&c.attempt){return Err("refused:abandoned-candidate".into());}check_live(root,c)}
pub fn register_packet(root:&Path,path:&str,sel:&Selector,_s:&State,m:&mut Metrics)->Result<()>{let mut r=get(root,m)?;r.packets.insert(path.into(),sel.clone());put(root,&r,m)}
fn safe_file(root:&Path,p:&Path)->Result<()> {
    let base=root.canonicalize()?;let target=p.canonicalize()?;
    if !target.starts_with(&base)||target==base||fs::symlink_metadata(p)?.file_type().is_symlink(){return Err("refused:unsafe-reclamation-path".into());}Ok(())
}
fn files(dir:&Path,out:&mut Vec<std::path::PathBuf>)->Result<()>{
    if !dir.exists(){return Ok(());}for x in fs::read_dir(dir)?{let p=x?.path();let md=fs::symlink_metadata(&p)?;if md.file_type().is_symlink(){return Err("refused:reclamation-symlink".into());}if md.is_dir(){files(&p,out)?;}else{out.push(p);}}Ok(())
}
pub fn run(root:&Path,v:&Value,m:&mut Metrics)->Result<Value>{
    let _guard=crate::lock(root,m)?;let mut r=get(root,m)?;let op=v["operation"].as_str().ok_or("operation")?;
    let head=crate::selector(root,m)?.ok_or("missing-selected")?;
    let selected:Selector=if v["selected"].is_object(){serde_json::from_value(v["selected"].clone())?}else{head.clone()};
    let p=crate::retained_publication(root,&selected,m)?;
    match op {
        "setup"=>{
            if !root.components().any(|c|c.as_os_str()=="disposable"){return Err("refused:disposable-root-required".into());}
            io::atomic(&root.join("DISPOSABLE.json"),&io::json(&json!({"root":root.canonicalize()?.to_string_lossy()}))?,m)?;Ok(json!({"status":"disposable-registered"}))
        }
        "pin"=>{check_live(root,&p.candidate)?;let name=v["name"].as_str().ok_or("name")?;if r.pins.get(name).is_some_and(|old|old!=&selected){return Err("refused:pin-name-already-bound".into());}r.pins.insert(name.into(),selected);put(root,&r,m)?;io::checkpoint("pin-registered")?;Ok(json!({"status":"pinned"}))}
        "release"=>{r.pins.remove(v["name"].as_str().ok_or("name")?);put(root,&r,m)?;Ok(json!({"status":"released"}))}
        "release-packet"=>{r.packets.remove(v["path"].as_str().ok_or("path")?);put(root,&r,m)?;Ok(json!({"status":"packet-released"}))}
        "abandon"=>{
            let file=v["candidate"].as_str().ok_or("candidate")?;let c:Candidate=serde_json::from_slice(&io::read(&root.join(file),m)?)?;
            if crate::lineage(root,Some(head),m)?.iter().any(|p|p.candidate.attempt==c.attempt){return Err("refused:selected-attempt".into());}
            r.abandoned.insert(c.attempt);put(root,&r,m)?;Ok(json!({"status":"abandoned"}))
        }
        "retire"=>{
            if selected==head||r.pins.values().chain(r.packets.values()).any(|s|s==&selected){return Err("refused:retained-snapshot".into());}
            // One atomic record gates retirement. The publication-side record is
            // a discoverability cache written only after the gate is durable.
            let record=io::json(&json!({"status":"retired","selected":selected,"attempt":p.candidate.attempt,"outcome_preserved":true}))?;
            io::atomic(&root.join("retired-attempts").join(format!("{}.json",p.candidate.attempt)),&record,m)?;
            io::checkpoint("retirement-gate-durable")?;
            io::atomic(&root.join("retired-publications").join(format!("{}.json",p.id)),&record,m)?;
            io::checkpoint("snapshot-retired")?;Ok(json!({"status":"retired"}))
        }
        "collect"=>{
            let gate=fs::OpenOptions::new().create(true).truncate(false).read(true).write(true).open(root.join("preparation.lock"))?;
            // Bounded refusal, preserving paused preparation. No automatic expiry.
            fs2::FileExt::try_lock_exclusive(&gate).map_err(|e|format!("busy:active-preparation:{e}"))?;
            let allowed:Value=serde_json::from_slice(&io::read(&root.join("DISPOSABLE.json"),m)?)?;
            if allowed["root"]!=root.canonicalize()?.to_string_lossy().as_ref(){return Err("refused:disposable-identity".into());}
            let history=crate::lineage(root,Some(head),m)?;
            let selected_attempts:BTreeSet<_>=history.iter().map(|p|p.candidate.attempt.clone()).collect();
            let mut keep=BTreeSet::new();let mut eligible=vec![];let mut candidates=vec![];
            let mut all=vec![];files(&root.join("attempts"),&mut all)?;
            for path in all.iter().filter(|p|p.file_name().is_some_and(|n|n=="candidate.json")){
                let c:Candidate=serde_json::from_slice(&io::read(path,m)?)?;
                let retired=check_live(root,&c).is_err();let abandoned=r.abandoned.contains(&c.attempt);
                if retired||abandoned {
                    // Keep candidate and publication evidence for truthful resolution.
                    eligible.push(root.join(&c.state));files(&root.join(&c.closure),&mut eligible)?;
                }else{candidates.push(c);}
            }
            // Unsealed partial preparations may contain references not yet in a
            // candidate. Refuse reclamation until explicitly handled by operator.
            for path in all.iter().filter(|p|p.file_name().is_some_and(|n|n=="state.json")) {
                let attempt=path.parent().unwrap().file_name().unwrap().to_string_lossy();
                if !all.iter().any(|p|p==&path.parent().unwrap().join("candidate.json"))&&!selected_attempts.contains(attempt.as_ref()) {return Err("refused:unsealed-preparation".into());}
            }
            for c in candidates {
                let s=backend::load(root,&c,m)?;crate::model::validate(&s)?;
                for d in s.docs.values(){io::checked(&root.join(&d.original),&d.digest,m)?;keep.insert(root.join(&d.original));}
                for id in backend::reachable(root,&c,m)?{keep.insert(root.join("objects").join(&id[..2]).join(id));}
            }
            let mut immutable=vec![];files(&root.join("originals"),&mut immutable)?;files(&root.join("objects"),&mut immutable)?;
            eligible.extend(immutable.into_iter().filter(|p|!keep.contains(p)));
            // Repeated sweep is idempotent after process interruption. Never sweep
            // publications, outcome lineage, fences or copied external packets.
            eligible.sort();eligible.dedup();let mut bytes=0;let mut count=0;
            for p in eligible{if !p.exists(){continue;}safe_file(root,&p)?;bytes+=p.metadata()?.len();fs::remove_file(&p)?;io::sync_dir(p.parent().unwrap(),m)?;count+=1;io::checkpoint("reclaim-file")?;}
            Ok(json!({"status":"collected","files":count,"logical_bytes":bytes,"outcomes_retained":true}))
        }
        _=>Err("unknown-lifecycle-operation".into())
    }
}
