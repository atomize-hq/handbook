//! Immutable projection map plus durable target-bound A refresh plans.
use crate::{io::{self,Metrics,Result},objects::{self,Id,Objects},graph::{self,GraphRoot,Delta},store::{Store,Candidate},model::{self,Selector}};
use serde::{Serialize,Deserialize};
use serde_json::{json,Value};
use std::{collections::{BTreeMap,BTreeSet},fs,path::{Path,PathBuf},time::Instant};
#[derive(Clone,Serialize,Deserialize,PartialEq,Eq)]
#[serde(deny_unknown_fields)]
pub struct FileSpec{pub source:String,pub digest:Id,
    #[serde(default,skip_serializing_if="Option::is_none")]
    pub projection_profile:Option<u32>,
}
// Experimental controls identify new immutable targets; replay always uses the
// prepared target's own marker. Historical profiles never acquire this field.
pub fn new_profile()->u32{std::env::var("EXP_NAVIGATION").ok().and_then(|v|v.parse().ok()).filter(|v|[2,3,4].contains(v)).unwrap_or(4)}
#[derive(Clone,Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
struct Watermark{selected:Selector,sequence:u64,graph:GraphRoot,projection:Option<Id>}
#[derive(Clone,Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
struct Action{path:String,prior:Option<FileSpec>,desired:Option<FileSpec>}
#[derive(Clone,Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
struct Plan{version:u32,id:String,target:Watermark,prior:Option<Watermark>,full:bool,extras:BTreeSet<String>,actions:Vec<Action>}
pub(crate) fn derived(root:&Path,bytes:&[u8],m:&mut Metrics)->Result<FileSpec>{
    let digest=io::digest(bytes);let source=format!("derived/{}/{}",&digest[..2],digest);let path=root.join(&source);
    m.replay_files.insert(source.clone(),digest.clone());
    if path.exists(){io::checked(&path,&digest,m)?;}
    else{
        let stage=path.with_file_name(io::unique("derived-stage"));
        io::write_new(&stage,bytes,m)?;io::install_original(&stage,&path,&digest,m)?;io::sync_dir(path.parent().unwrap(),m)?;
    }
    Ok(FileSpec{source,digest,projection_profile:None})
}
fn safe_path(path:&str)->Result<()>{
    if path.len()>1000||path.contains('\\')||Path::new(path).components().any(|p|!matches!(p,std::path::Component::Normal(_))){
        return Err("integrity:projection-path".into());
    }Ok(())
}
fn nav(root:&Path,o:&mut Objects,g:&GraphRoot,id:&str)->Result<Option<FileSpec>>{
    let Some(h)=graph::header(o,g,id)? else{return Ok(None)};
    let mut b=format!("# {}\n\n{} — {}\n\n[Exact source](../{}) · [Collection](../NAVIGATION.md)\n\n",h.doc.id,h.doc.title.replace(['\r','\n']," "),h.doc.kind,h.doc.path);
    for (_,e) in graph::outgoing(o,g,id)?{
        if e.external{b.push_str(&format!("- {}: {} (external; availability not promised)\n",e.kind,e.target));}
        else{
            let t=graph::header(o,g,&e.target)?.ok_or("integrity:navigation-target")?;
            b.push_str(&format!("- {}: [{}](../{})\n",e.kind,e.target,t.doc.path));
        }
    }Ok(Some(derived(root,b.as_bytes(),o.metrics)?))
}
fn static_files(root:&Path,o:&mut Objects,version:u32)->Result<BTreeMap<String,Vec<u8>>>{
    let mut files=BTreeMap::new();
    let text=b"# Collection navigation\n\nExact authored sources are in docs/. Read navigation/ in stable ID order for titles, kinds, source paths and typed local/external relationships. Use ordinary listing, text search, copying and SHA-256 tools.\n\n[Overview](navigation/d000000.md) | [Packet](packet/context.md) | [Ordered provenance](packet/provenance.json)\n\nReadable files do not grant edit/import authority. A may mix or lag; B requires one positively selected retained generation path.\n";
    for (path,b) in [("NAVIGATION.md",text.as_slice()),("packet/context.md",b"".as_slice()),("packet/provenance.json",b"{\"requested\":false,\"recipe\":\"not-requested\",\"renderer\":\"exact-concat-v1\",\"inputs\":[],\"omissions\":[\"no new context packet requested\"],\"errors\":[],\"output_sha256\":\"e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855\"}".as_slice())]{
        if version>=2&&path=="NAVIGATION.md"{continue;}
        let mut f=derived(root,b,o.metrics)?;
        if version>=3&&path=="packet/provenance.json"{f.projection_profile=Some(version);}
        files.insert(path.into(),objects::encode(&f,o)?);
    }Ok(files)
}
// Version 2 supplies only metadata absent from the exact authored files. One
// small collection map is intentional here; large-layout costs are unmeasured.
fn collection(root:&Path,o:&mut Objects,g:&GraphRoot)->Result<FileSpec>{
    let mut b=String::from("version: 2\ndocuments:\n");
    for (_,h) in objects::range::<graph::Header>(o,&g.tree,"D/")?{
        b.push_str(&format!("  - id: {}\n    title: {}\n    kind: {}\n    source: {}\n    relationships:",serde_json::to_string(&h.doc.id)?,serde_json::to_string(&h.doc.title)?,serde_json::to_string(&h.doc.kind)?,serde_json::to_string(&h.doc.path)?));
        let edges=graph::outgoing(o,g,&h.doc.id)?;
        if edges.is_empty(){b.push_str(" []\n");}else{
            b.push('\n');
            for (_,e) in edges{b.push_str(&format!("      - kind: {}\n        target: {}\n        external: {}\n",serde_json::to_string(&e.kind)?,serde_json::to_string(&e.target)?,e.external));}
        }
    }
    o.metrics.navigation_parts+=1;o.metrics.navigation_bytes+=b.len() as u64;
    derived(root,b.as_bytes(),o.metrics)
}
pub fn profile(o:&mut Objects,projection:&Option<Id>)->Result<u32>{
    if let Some(f)=objects::point::<FileSpec>(o,projection,"packet/provenance.json")?{
        if let Some(v)=f.projection_profile{if [3,4].contains(&v){return Ok(v);}return Err("integrity:projection-profile".into());}
    }
    Ok(if objects::point::<FileSpec>(o,projection,"collection.yaml")?.is_some(){2}else{1})
}
fn namespace(o:&mut Objects,projection:&Option<Id>,g:&GraphRoot,paths:impl Iterator<Item=String>)->Result<()>{
    for path in paths{
        safe_path(&path)?;
        if objects::point::<FileSpec>(o,projection,&path)?.is_none(){continue;}
        let mut parent=Path::new(&path).parent();
        while let Some(p)=parent{
            if let Some(s)=p.to_str(){if !s.is_empty()&&objects::point::<FileSpec>(o,projection,s)?.is_some(){return Err("semantic:projection-file-ancestor".into());}}
            parent=p.parent();
        }
        let prefix=format!("{path}/");
        if !objects::range::<FileSpec>(o,projection,&prefix)?.is_empty(){return Err("semantic:projection-file-ancestor".into());}
        // The graph's path namespace is case folded on every supported host.
        // Check that contract too, before any A selection or B materialization.
        if path.starts_with("docs/"){
            let lower=path.to_lowercase();let mut parent=Path::new(&lower).parent();
            while let Some(p)=parent{
                let key=format!("P/{}",p.to_string_lossy());
                if objects::point::<String>(o,&g.tree,&key)?.is_some(){return Err("semantic:projection-file-ancestor".into());}
                parent=p.parent();
            }
            if !objects::range::<String>(o,&g.tree,&format!("P/{lower}/"))?.is_empty(){return Err("semantic:projection-file-ancestor".into());}
        }
    }Ok(())
}
pub fn full(root:&Path,o:&mut Objects,g:&GraphRoot,version:u32)->Result<Option<Id>>{
    let mut files=static_files(root,o,version)?;
    if [2,3].contains(&version){files.remove("NAVIGATION.md");files.insert("collection.yaml".into(),objects::encode(&collection(root,o,g)?,o)?);}
    if version==4{files.extend(crate::navigation::full(root,o,g)?);}
    for (_,h) in objects::range::<graph::Header>(o,&g.tree,"D/")?{
        files.insert(h.doc.path.clone(),objects::encode(&FileSpec{source:h.doc.original,digest:h.doc.digest,projection_profile:None},o)?);
        if version==1{let n=nav(root,o,g,&h.doc.id)?.ok_or("integrity:navigation")?;
        files.insert(format!("navigation/{}.md",h.doc.id),objects::encode(&n,o)?);}
    }
    let paths:Vec<_>=files.keys().cloned().collect();let result=objects::bulk(o,files)?;
    namespace(o,&result,g,paths.into_iter())?;Ok(result)
}
pub fn delta(root:&Path,o:&mut Objects,old:&Option<Id>,d:&Delta,version:u32)->Result<(Option<Id>,BTreeSet<String>)>{
    if profile(o,old)?!=version{
        // New preparation upgrades only its new immutable target. Old roots and
        // already-sealed legacy candidates retain their exact byte identities.
        let result=full(root,o,&d.graph,version)?;
        let prior=objects::entries(o,old)?;let next=objects::entries(o,&result)?;
        let paths=prior.keys().chain(next.keys()).filter(|p|prior.get(*p)!=next.get(*p)).cloned().collect();
        return Ok((result,paths));
    }
    let mut changes=BTreeMap::new();
    for h in d.old.values().flatten(){changes.insert(h.doc.path.clone(),vec![]);}
    for h in d.final_docs.values().flatten(){changes.insert(h.doc.path.clone(),objects::encode(&FileSpec{source:h.doc.original.clone(),digest:h.doc.digest.clone(),projection_profile:None},o)?);}
    if version==2||(version==3&&!d.navigation_facts.is_empty()){changes.insert("collection.yaml".into(),objects::encode(&collection(root,o,&d.graph)?,o)?);}
    if version==4{changes.extend(crate::navigation::delta(root,o,old,d)?);}
    for id in d.navigation.iter().filter(|_|version==1){
        let key=format!("navigation/{id}.md");
        changes.insert(key,nav(root,o,&d.graph,id)?.map(|f|objects::encode(&f,o)).transpose()?.unwrap_or_default());
    }
    let mut actual=BTreeMap::new();let mut paths=BTreeSet::new();
    for (p,b) in changes{
        let prior=crate::disk::DiskIndex::open(o,old.clone())?.get(o,&p)?.unwrap_or_default();
        if prior!=b{paths.insert(p.clone());actual.insert(p,b);}
    }
    // A no-packet transaction explicitly clears any preceding packet status.
    for (p,b) in static_files(root,o,version)?{
        if version>=2&&p=="NAVIGATION.md"{continue;}
        let prior=crate::disk::DiskIndex::open(o,old.clone())?.get(o,&p)?.unwrap_or_default();
        if prior!=b{paths.insert(p.clone());actual.insert(p,b);}
    }
    let result=objects::update(o,old,actual)?;namespace(o,&result,&d.graph,paths.iter().cloned())?;Ok((result,paths))
}
fn map(root:&Path,projection:&Option<Id>,m:&mut Metrics)->Result<BTreeMap<String,FileSpec>>{
    let mut o=objects::object_store(root,m);
    objects::entries(&mut o,projection)?.into_iter().map(|(p,b)|{safe_path(&p)?;Ok((p,objects::decode(&b,&mut o)?))}).collect()
}
pub fn generation(store:&Store,c:&Candidate,m:&mut Metrics)->Result<Id>{
    let started=Instant::now();
    let dest=store.root.join(format!("generations/{}",c.attempt));
    let files=map(&store.root,&c.projection,m)?;
    for (p,f) in &files{
        io::copy(&store.root.join(&f.source),&dest.join(p),m)?;
        io::checked(&dest.join(p),&f.digest,m)?;
    }
    let b=io::json(&files)?;io::write_new(&dest.join("MANIFEST.json"),&b,m)?;
    io::checkpoint("generation-installed")?;
    m.elapsed("B-generation-inclusive",started);
    Ok(io::digest(&b))
}
pub fn verify_generation(store:&Store,c:&Candidate,m:&mut Metrics)->Result<()>{
    let started=Instant::now();let out=verify_generation_parts(store,&c.attempt,&c.projection,c.generation_manifest.as_ref().ok_or("integrity:missing-B-readiness")?,m);
    m.elapsed("B-generation-verification-inclusive",started);out
}
fn verify_generation_parts(store:&Store,attempt:&str,projection:&Option<Id>,hash:&str,m:&mut Metrics)->Result<()>{
    let dest=store.root.join(format!("generations/{attempt}"));
    let files:BTreeMap<String,FileSpec>=serde_json::from_slice(&io::checked(&dest.join("MANIFEST.json"),hash,m)?)?;
    if files!=map(&store.root,projection,m)?{return Err("integrity:B-projection-binding".into());}
    for (p,f) in &files{safe_path(p)?;io::checked(&dest.join(p),&f.digest,m)?;}
    let mut actual=vec![];crate::lifetime::files(&dest,&mut actual)?;
    let names:BTreeSet<_>=actual.iter().map(|p|p.strip_prefix(&dest).unwrap().to_string_lossy().replace('\\',"/")).filter(|s|s!="MANIFEST.json").collect();
    if names!=files.keys().cloned().collect(){return Err("integrity:B-namespace".into());}
    io::checkpoint("generation-verified")
}
fn packet_content(store:&Store,g:&GraphRoot,m:&mut Metrics)->Result<(BTreeMap<String,FileSpec>,Vec<u8>,Vec<u8>)>{
    let mut o=objects::object_store(&store.root,m);let s=graph::all(&mut o,g)?;drop(o);
    model::validate(&s)?;
    let ids:Vec<_>=s.docs.keys().take(8).cloned().collect();
    let mut included:BTreeSet<_>=ids.iter().cloned().collect();
    loop{
        let extra:Vec<_>=included.iter().flat_map(|id|s.docs[id].edges.iter().filter(|e|!e.external).map(|e|e.target.clone())).collect();
        let n=included.len();included.extend(extra);if n==included.len(){break;}
    }
    let mut files=BTreeMap::new();let mut output=vec![];let mut inputs=vec![];
    for id in &included{
        let d=&s.docs[id];files.insert(d.path.clone(),FileSpec{source:d.original.clone(),digest:d.digest.clone(),projection_profile:None});
        if ids.contains(id){
            let b=io::checked(&store.root.join(&d.original),&d.digest,m)?;
            output.extend(format!("\n\n<!-- source {} sha256 {} -->\n",d.id,d.digest).as_bytes());output.extend(&b);
            inputs.push(json!({"id":id,"revision":d.digest,"path":format!("../{}",d.path),"sha256":d.digest,"bytes":d.bytes}));
        }
    }
    let prov=io::json(&json!({"requested":true,"recipe":"ordered-first-eight-v1","renderer":"exact-concat-v1","inputs":inputs,
        "omissions":["documents beyond first eight omitted from assembled output; required local dependencies retained"],"errors":[],"output_sha256":io::digest(&output)}))?;
    Ok((files,output,prov))
}
pub fn packet(store:&Store,g:&GraphRoot,projection:&Option<Id>,attempt:&str,m:&mut Metrics)->Result<Option<Id>>{
    let t=Instant::now();let (files,output,prov)=packet_content(store,g,m)?;
    let dest=store.root.join(format!("attempts/{attempt}/packet"));
    let mut manifest=BTreeMap::new();
    for (p,f) in files{io::copy(&store.root.join(f.source),&dest.join(&p),m)?;io::checked(&dest.join(&p),&f.digest,m)?;manifest.insert(p,f.digest);}
    for (p,b) in [("packet/context.md",&output),("packet/provenance.json",&prov)]{
        io::write_new(&dest.join(p),b,m)?;manifest.insert(p.into(),io::digest(b));
    }
    // Existing source/provenance/recipe files suffice for retained packets.
    // Exact closure verification precedes manifest sealing.
    for (p,h) in &manifest{io::checked(&dest.join(p),h,m)?;}
    io::write_new(&dest.join("MANIFEST.json"),&io::json(&manifest)?,m)?;
    let mut o=objects::object_store(&store.root,m);let mut updates=BTreeMap::new();
    let version=profile(&mut o,projection)?;
    for (p,b) in [("packet/context.md",output),("packet/provenance.json",prov)]{
        let mut f=derived(&store.root,&b,o.metrics)?;
        if version>=3&&p=="packet/provenance.json"{f.projection_profile=Some(version);}
        updates.insert(p.into(),objects::encode(&f,&mut o)?);
    }
    let result=objects::update(&mut o,projection,updates)?;o.metrics.elapsed("packet-inclusive",t);Ok(result)
}
pub fn packet_verify_replay(store:&Store,g:&GraphRoot,projection:&Option<Id>,attempt:&str,m:&mut Metrics)->Result<Option<Id>>{
    let (files,output,prov)=packet_content(store,g,m)?;
    let dest=store.root.join(format!("attempts/{attempt}/packet"));
    let manifest:BTreeMap<String,String>=serde_json::from_slice(&io::read(&dest.join("MANIFEST.json"),m)?)?;
    let mut expected:BTreeMap<_,_>=files.iter().map(|(p,f)|(p.clone(),f.digest.clone())).collect();
    expected.insert("packet/context.md".into(),io::digest(&output));expected.insert("packet/provenance.json".into(),io::digest(&prov));
    let version={let mut o=objects::object_store(&store.root,m);profile(&mut o,projection)?};let legacy=version==1;
    if legacy{expected.insert("NAVIGATION.md".into(),io::digest(b"# Exact retained packet\n\nSources are in docs/. Ordered inputs, exact paths, hashes, omissions and recipe are in packet/provenance.json.\n"));}
    if manifest!=expected{return Err("integrity:packet-manifest-completeness".into());}
    let mut actual=vec![];crate::lifetime::files(&dest,&mut actual)?;
    let names:BTreeSet<_>=actual.iter().map(|p|p.strip_prefix(&dest).unwrap().to_string_lossy().replace('\\',"/")).filter(|p|p!="MANIFEST.json").collect();
    if names!=expected.keys().cloned().collect(){return Err("integrity:packet-namespace".into());}
    for (p,h) in &expected{io::checked(&dest.join(p),h,m)?;}
    for (p,f) in files{
        if manifest.get(&p)!=Some(&f.digest){return Err("integrity:packet-replay-binding".into());}
        io::checked(&dest.join(&p),&f.digest,m)?;
    }
    let mut o=objects::object_store(&store.root,m);let mut updates=BTreeMap::new();
    for (p,b) in [("packet/context.md",output),("packet/provenance.json",prov)]{
        io::checked(&dest.join(p),&io::digest(&b),o.metrics)?;
        let mut f=derived(&store.root,&b,o.metrics)?;
        if version>=3&&p=="packet/provenance.json"{f.projection_profile=Some(version);}
        updates.insert(p.into(),objects::encode(&f,&mut o)?);
    }objects::update(&mut o,projection,updates)
}
pub fn verify_capsule(store:&Store,g:&GraphRoot,packet:&crate::store::PacketRef,m:&mut Metrics)->Result<()>{
    if !model::safe_id(&packet.attempt)||packet.graph_digest!=io::digest(&io::json(g)?){return Err("integrity:packet-graph-binding".into());}
    let dest=store.root.join(format!("attempts/{}/packet",packet.attempt));
    let manifest:BTreeMap<String,String>=serde_json::from_slice(&io::checked(&dest.join("MANIFEST.json"),&packet.manifest,m)?)?;
    for (p,h) in &manifest{safe_path(p)?;io::checked(&dest.join(p),h,m)?;}
    let mut paths=vec![];crate::lifetime::files(&dest,&mut paths)?;
    let names:BTreeSet<_>=paths.iter().map(|p|p.strip_prefix(&dest).unwrap().to_string_lossy().replace('\\',"/")).filter(|p|p!="MANIFEST.json").collect();
    if names!=manifest.keys().cloned().collect(){return Err("integrity:packet-namespace".into());}Ok(())
}
fn watermark(store:&Store,m:&mut Metrics)->Result<Option<Watermark>>{
    let path=store.root.join("VIEW.json");if !path.exists(){return Ok(None);}
    let mark:Watermark=serde_json::from_slice(&io::read(&path,m)?)?;validate_mark(store,&mark,m)?;Ok(Some(mark))
}
fn validate_mark(store:&Store,w:&Watermark,m:&mut Metrics)->Result<()>{
    let p=store.member(&w.selected,m)?;
    if p.graph!=w.graph||p.projection!=w.projection||p.sequence!=w.sequence{return Err("integrity:watermark-binding".into());}Ok(())
}
fn pending(store:&Store,m:&mut Metrics)->Result<Option<Plan>>{
    let path=store.root.join("PENDING.json");if !path.exists(){return Ok(None);}
    let plan:Plan=serde_json::from_slice(&io::read(&path,m)?)?;
    if plan.version!=1||!model::safe_id(&plan.id){return Err("integrity:refresh-version".into());}
    validate_mark(store,&plan.target,m)?;if let Some(w)=&plan.prior{validate_mark(store,w,m)?;}
    Ok(Some(plan))
}
fn derive_actions(store:&Store,target:&Watermark,prior:Option<&Watermark>,full:bool,extras:&BTreeSet<String>,m:&mut Metrics)->Result<Vec<Action>>{
    let mut paths=BTreeSet::new();let mut o=objects::object_store(&store.root,m);
    if full||prior.is_none(){
        paths.extend(objects::entries(&mut o,&target.projection)?.into_keys());
        if let Some(p)=prior{paths.extend(objects::entries(&mut o,&p.projection)?.into_keys());}
        paths.extend(extras.iter().cloned());
    }else{
        if !extras.is_empty(){return Err("integrity:incremental-extra-actions".into());}
        let p=store.publication(&target.selected,o.metrics)?;
        let lo=format!("{:020}/",prior.unwrap().sequence+1);let hi=format!("{:020}/",target.sequence+1);
        for (key,_) in crate::disk::DiskIndex::open(&mut o,p.changes)?.range(&mut o,&lo,&hi)?{
            paths.insert(key.split_once('/').ok_or("integrity:change-range")?.1.to_owned());
        }
    }
    let mut out=vec![];
    for path in paths{
        safe_path(&path)?;
        let old=if let Some(p)=prior{objects::point(&mut o,&p.projection,&path)?}else{None};
        let desired=objects::point(&mut o,&target.projection,&path)?;
        out.push(Action{path,prior:old,desired});
    }Ok(out)
}
pub fn protects(store:&Store,s:&Selector,m:&mut Metrics)->Result<bool>{
    Ok(watermark(store,m)?.is_some_and(|w|w.selected==*s)||pending(store,m)?.is_some_and(|p|p.target.selected==*s||p.prior.is_some_and(|w|w.selected==*s)))
}
pub fn protected_roots(store:&Store,m:&mut Metrics)->Result<Vec<(GraphRoot,Option<Id>)>>{
    let mut out=vec![];
    if let Some(w)=watermark(store,m)?{out.push((w.graph,w.projection));}
    if let Some(p)=pending(store,m)?{
        out.push((p.target.graph,p.target.projection));if let Some(w)=p.prior{out.push((w.graph,w.projection));}
    }Ok(out)
}
pub fn pending_changes(store:&Store,m:&mut Metrics)->Result<Option<Option<Id>>>{
    pending(store,m)?.map(|p|Ok(store.publication(&p.target.selected,m)?.changes)).transpose()
}
fn execute_plan(store:&Store,plan:&Plan,m:&mut Metrics)->Result<()>{
    validate_mark(store,&plan.target,m)?;if let Some(w)=&plan.prior{validate_mark(store,w,m)?;}
    let expected=derive_actions(store,&plan.target,plan.prior.as_ref(),plan.full,&plan.extras,m)?;
    if io::json(&expected)?!=io::json(&plan.actions)?{return Err("integrity:refresh-plan-actions".into());}
    let root=&store.root;let view=root.join("view");
    // Keep the sealed plan's byte order for old pending plans, but execute
    // removals deepest first and installations shallowest first on every replay.
    let mut ordered:Vec<_>=plan.actions.iter().collect();
    ordered.sort_by_key(|a|(a.desired.is_some(),if a.desired.is_none(){usize::MAX-a.path.matches('/').count()}else{a.path.matches('/').count()},a.path.clone()));
    for action in ordered{
        safe_path(&action.path)?;m.touched_paths+=1;
        let dest=view.join(&action.path);
        // An ancestor file may still obscure this obsolete descendant after an
        // interrupted installation. The desired namespace cannot contain both.
        let actual=if dest.is_file(){Some(io::read(&dest,m)?)}else{None};
        let hash=actual.as_ref().map(|b|io::digest(b));
        if hash.as_ref()==action.desired.as_ref().map(|f|&f.digest)&&!dest.is_dir(){continue;}
        if let Some(bytes)=actual{
            if action.prior.as_ref().is_none_or(|f|Some(&f.digest)!=hash.as_ref()){
                let backup=root.join(format!("divergence/{}/{}/{}",plan.id,io::digest(action.path.as_bytes()),io::digest(&bytes)));
                if backup.exists(){io::checked(&backup,&io::digest(&bytes),m)?;}
                else{
                    let stage=backup.with_file_name(io::unique("preserve-stage"));io::write_new(&stage,&bytes,m)?;
                    io::install_original(&stage,&backup,&io::digest(&bytes),m)?;io::sync_dir(backup.parent().unwrap(),m)?;
                }
                io::checkpoint("refresh-preserved")?;
            }
        }
        if let Some(want)=&action.desired{
            if dest.is_dir(){
                // Known obsolete files were removed first. Preserve any remaining
                // externally added subtree before replacing its namespace.
                let backup=root.join(format!("divergence/{}/namespace/{}",plan.id,io::digest(action.path.as_bytes())));
                io::mkdir(&backup,m)?;
                // A user may recreate the obstructing directory after an exit.
                // Preserve each incarnation without overwriting the earlier one.
                let backup=backup.join(io::unique("tree"));
                fs::rename(&dest,&backup)?;io::sync_dir(dest.parent().unwrap(),m)?;io::sync_dir(backup.parent().unwrap(),m)?;
                io::checkpoint("refresh-namespace-preserved")?;
            }
            let stage=root.join(format!("refresh-stages/{}/{}",plan.id,io::digest(action.path.as_bytes())));
            if stage.exists(){crate::lifetime::remove_checked(root,&stage,m)?;}
            io::copy(&root.join(&want.source),&stage,m)?;io::checked(&stage,&want.digest,m)?;
            io::mkdir(dest.parent().unwrap(),m)?;io::replace(&stage,&dest,m)?;
            io::sync_dir(dest.parent().unwrap(),m)?;io::sync_dir(stage.parent().unwrap(),m)?;
        }else if dest.is_file(){crate::lifetime::remove_checked(root,&dest,m)?;}
        // Remove only empty directories under the engine's readable root. This
        // never recursively deletes an external subtree.
        let mut parent=if action.prior.is_some()||action.desired.is_some(){dest.parent()}else{None};
        while let Some(p)=parent{
            if p==view||!p.starts_with(&view){break;}
            match io::remove_dir(p){Ok(())=>{io::sync_dir(p.parent().unwrap(),m)?;m.namespace_ops+=1;},Err(_)=>break}
            parent=p.parent();
        }
        io::checkpoint("refresh-namespace")?;
        io::checkpoint("refresh-file")?;
    }
    io::checkpoint("refresh-before-complete")?;
    io::atomic(&root.join("VIEW.json"),&io::json(&plan.target)?,m)?;
    io::checkpoint("refresh-complete")?;
    crate::lifetime::remove_checked(root,&root.join("PENDING.json"),m)?;Ok(())
}
pub fn refresh(store:&Store,full_audit:bool,m:&mut Metrics)->Result<()>{
    if store.config.mode!="A"{return Err("refused:A-only".into());}
    let t=Instant::now();
    if let Some(plan)=pending(store,m)?{execute_plan(store,&plan,m)?;}
    let head=store.snapshot(m)?;let prior=watermark(store,m)?;
    if !full_audit&&prior.as_ref().is_some_and(|w|w.selected==head.selected){return Ok(());}
    let target=Watermark{selected:head.selected,sequence:head.publication.sequence,graph:head.publication.graph,projection:head.publication.projection};
    let mut extras=BTreeSet::new();
    if full_audit{
        let mut actual=vec![];crate::lifetime::files(&store.root.join("view"),&mut actual)?;
        extras.extend(actual.iter().map(|p|p.strip_prefix(store.root.join("view")).unwrap().to_string_lossy().replace('\\',"/")));
    }
    let actions=derive_actions(store,&target,prior.as_ref(),full_audit,&extras,m)?;
    let plan=Plan{version:1,id:io::unique("refresh"),target,prior,full:full_audit,extras,actions};
    io::atomic(&store.root.join("PENDING.json"),&io::json(&plan)?,m)?;io::checkpoint("refresh-plan")?;
    execute_plan(store,&plan,m)?;m.elapsed("A-completion-inclusive",t);Ok(())
}
pub fn audit_view(store:&Store,m:&mut Metrics)->Result<Value>{
    let head=store.snapshot(m)?;let files=map(&store.root,&head.publication.projection,m)?;
    let mut missing=vec![];let mut edited=vec![];
    for (path,f) in &files{
        let p=store.root.join("view").join(path);
        if !p.exists(){missing.push(path);}else if io::digest(&io::read(&p,m)?)!=f.digest{edited.push(path);}
    }
    Ok(json!({"missing":missing,"edited":edited,"marker_is_integrity_proof":false}))
}
pub fn full_oracle(store:&Store,m:&mut Metrics)->Result<()>{
    let head=store.snapshot(m)?;let mut o=objects::object_store(&store.root,m);
    graph::audit(&mut o,&head.publication.graph)?;
    let mut actual=objects::entries(&mut o,&head.publication.projection)?;
    // Observe selected evidence before the renderer can recreate missing objects.
    for (path,bytes) in &actual{
        safe_path(path)?;let file:FileSpec=objects::decode(bytes,&mut o)?;
        safe_path(&file.source)?;io::checked(&store.root.join(&file.source),&file.digest,o.metrics)?;
    }
    let version=profile(&mut o,&head.publication.projection)?;
    let expected=full(&store.root,&mut o,&head.publication.graph,version)?;
    let mut expected=objects::entries(&mut o,&expected)?;
    // Packet work is independently checked against exact selected graph/closure.
    for path in ["packet/context.md","packet/provenance.json"]{actual.remove(path);expected.remove(path);}
    if actual!=expected{return Err("integrity:full-projection-oracle".into());}
    drop(o);if let Some(p)=&head.publication.packet_ref{verify_capsule(store,&head.publication.graph,p,m)?;}
    Ok(())
}
