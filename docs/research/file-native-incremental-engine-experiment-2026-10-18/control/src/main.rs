//! Standalone, bounded process-fault experiment; not a production storage API.
mod io;
mod model;
mod disk;
mod backend;
mod lifecycle;
use backend::{Objects,Id};
const MAX_OBJECT:usize=1024*1024;
use io::{Metrics,Result};
use model::*;
use fs2::FileExt;
use serde::{Serialize,Deserialize};
use serde_json::{json,Value};
use std::{collections::{BTreeMap,BTreeSet},fs::{self,File,OpenOptions},io::Read,path::{Path,PathBuf},time::{Duration,Instant}};

fn selector(root:&Path,m:&mut Metrics)->Result<Option<Selector>> {
    let p=root.join("SELECTED.json");
    if !p.exists() {return Ok(None);} Ok(Some(serde_json::from_slice(&io::read(&p,m)?)?))
}
fn publication(root:&Path,s:&Selector,m:&mut Metrics)->Result<Publication> {
    if !safe_id(&s.publication) {return Err("integrity:publication-id".into());}
    let p:Publication=serde_json::from_slice(&io::checked(&root.join(format!("publications/{}.json",s.publication)),&s.digest,m)?)?;
    if p.id!=s.publication {return Err("integrity:publication-identity".into());} Ok(p)
}
fn state(root:&Path,c:&Candidate,m:&mut Metrics)->Result<State> {
    lifecycle::check_live(root,c)?;
    let s:State=backend::load(root,c,m)?;
    validate(&s)?; Ok(s)
}
fn lineage(root:&Path,head:Option<Selector>,m:&mut Metrics)->Result<Vec<Publication>> {
    let mut out=vec![];let mut next=head;let mut seen=BTreeSet::new();
    while let Some(s)=next {
        if !seen.insert(s.publication.clone())||seen.len()>1000 {return Err("integrity:lineage-cycle-or-cap".into());}
        let p=publication(root,&s,m)?;next=p.predecessor.clone();out.push(p);
    } Ok(out)
}
fn retained_publication(root:&Path,selected:&Selector,m:&mut Metrics)->Result<Publication>{
    // Hash-valid complete candidates/publications can be unselected orphans.
    // Explicit historical reads bind only positively selected lineage.
    if !lineage(root,selector(root,m)?,m)?.iter().any(|p|p.id==selected.publication){return Err("refused:not-selected-lineage".into());}
    publication(root,selected,m)
}
fn resolve(root:&Path,id:&str,intent:Option<&str>,m:&mut Metrics)->Result<Value> {
    for p in lineage(root,selector(root,m)?,m)? {
        if p.candidate.request.request==id {
            if intent.is_some_and(|s|s!=p.candidate.intent) {return Err("refused:request-id-intent-mismatch".into());}
            return Ok(p.result);
        }
    }
    Ok(json!({"outcome":"absent-as-of","terminal_noncommit":false}))
}
fn lock(root:&Path,m:&mut Metrics)->Result<File> {
    let t=Instant::now();let f=OpenOptions::new().create(true).truncate(false).read(true).write(true).open(root.join("writer.lock"))?;
    loop {match f.try_lock_exclusive(){Ok(())=>break,Err(e)=>{
        if let Ok(id)=std::env::var("EXP_CANCEL_ID") {if root.join(format!("cancel-signals/{}",io::digest(id.as_bytes()))).exists(){m.elapsed("lock-wait",t);return Err("cancelled:while-waiting".into());}}
        if t.elapsed()>Duration::from_millis(std::env::var("EXP_WAIT_MS").ok().and_then(|s|s.parse().ok()).unwrap_or(15000)){m.elapsed("lock-wait",t);return Err(format!("busy:lock-timeout:{e}").into());}
        std::thread::sleep(Duration::from_millis(2));
    }}}
    m.elapsed("lock-wait",t);io::checkpoint("lock-acquired")?;Ok(f)
}
fn navigation(s:&State)->String {
    let mut text=format!("# {} collection\n\nStart with [overview]({}). Stable IDs survive title changes.\n\nExact files retain authored bytes. YAML/JSON are sources; NAVIGATION contains typed relationships. External unavailable sources are permitted; local missing targets are errors.\n\n[Saved packet](packet/context.md) · [ordered provenance](packet/provenance.json)\n\n",s.consumer,s.docs.get("d000000").map(|d|d.path.as_str()).unwrap_or(""));
    for d in s.docs.values() {
        text.push_str(&format!("## {}\n\n{} — [{}]({})\n\n",d.id,d.title.replace(['\r','\n']," "),d.kind,d.path));
        for e in &d.edges {text.push_str(&if e.external{format!("- {}: {} (external; availability not promised)\n",e.kind,e.target)}else{format!("- {}: [{}]({})\n",e.kind,e.target,s.docs[&e.target].path)});}
    } text
}
#[derive(Serialize,Deserialize)]
struct Manifest { files:BTreeMap<String,String> }
fn collect_names(root:&Path,dir:&Path,out:&mut BTreeSet<String>)->Result<()> {
    for item in fs::read_dir(dir)? {let p=item?.path();let meta=fs::symlink_metadata(&p)?;
        if meta.file_type().is_symlink() {return Err("integrity:symlink".into());}
        if meta.is_dir(){collect_names(root,&p,out)?;}else{out.insert(p.strip_prefix(root)?.to_string_lossy().replace('\\',"/"));}
    } Ok(())
}
fn closure(root:&Path,dest:&Path,s:&State,all:bool,packet:bool,m:&mut Metrics)->Result<String> {
    let started=Instant::now(); eprintln!("progress:materialization-start:{}",s.docs.len());
    io::mkdir(dest,m)?; let mut files=BTreeMap::new();
    let packet_ids:Vec<_>=if packet{s.docs.keys().take(8).cloned().collect()}else{vec![]};
    let mut included:BTreeSet<String>=if all{s.docs.keys().cloned().collect()}else{packet_ids.iter().cloned().collect()};
    loop {
        let extra:Vec<_>=included.iter().flat_map(|id|s.docs[id].edges.iter().filter(|e|!e.external).map(|e|e.target.clone())).collect();
        let old=included.len();included.extend(extra);if old==included.len(){break;}
    }
    let mut output=Vec::new();let mut inputs=Vec::new();
    for d in s.docs.values().filter(|d|included.contains(&d.id)) {
        let source=root.join(&d.original);let path=dest.join(&d.path);
        io::copy(&source,&path,m)?;files.insert(d.path.clone(),d.digest.clone());
        if packet_ids.contains(&d.id) {
            let bytes=io::checked(&path,&d.digest,m)?;
            output.extend(format!("\n\n<!-- source {} sha256 {} -->\n",d.id,d.digest).as_bytes());output.extend(&bytes);
            inputs.push(json!({"id":d.id,"revision":d.digest,"path":format!("../{}",d.path),"sha256":d.digest,"bytes":d.bytes}));
        }
    }
    let mut readable_state=s.clone();readable_state.docs.retain(|id,_|included.contains(id));
    let nav_time=Instant::now();let nav=navigation(&readable_state);m.elapsed("navigation-render",nav_time);
    m.elapsed("materialization-inclusive",started);eprintln!("progress:verification-start:{}",files.len());
    let prov=io::json(&json!({"requested":packet,"recipe":if packet{"ordered-first-eight-v1"}else{"not-requested"},"renderer":"exact-concat-v1","inputs":inputs,"omissions":if packet{vec!["documents beyond first eight omitted from assembled output; required local dependencies retained"]}else{vec!["no new context packet requested"]},"errors":[],"output_sha256":io::digest(&output)}))?;
    for (name,bytes) in [("NAVIGATION.md",nav.into_bytes()),("packet/context.md",output),("packet/provenance.json",prov)] {
        io::write_new(&dest.join(name),&bytes,m)?; files.insert(name.into(),io::digest(&bytes));
    }
    io::checkpoint("closure-before-verify")?;
    let t=Instant::now();for (p,h) in &files {let b=io::checked(&dest.join(p),h,m)?;verify_links(dest,p,&b)?;}
    let namespace=Instant::now();let mut actual=BTreeSet::new();collect_names(dest,dest,&mut actual)?;m.elapsed("namespace-enumeration",namespace);
    if actual!=files.keys().cloned().collect(){return Err("integrity:closure-namespace".into());}
    m.elapsed("destination-verification",t);io::checkpoint("closure-verified")?;eprintln!("progress:verification-complete:{}",files.len());
    let bytes=io::json(&Manifest{files})?;io::write_new(&dest.join("MANIFEST.json"),&bytes,m)?;
    io::checkpoint("closure-sealed")?;Ok(io::digest(&bytes))
}
fn verify_closure(root:&Path,c:&Candidate,m:&mut Metrics)->Result<()> {
    let dir=root.join(&c.closure);
    let manifest:Manifest=serde_json::from_slice(&io::checked(&dir.join("MANIFEST.json"),&c.manifest_digest,m)?)?;
    for (p,h) in &manifest.files {let b=io::checked(&dir.join(p),h,m)?;verify_links(&dir,p,&b)?;}
    let mut actual=BTreeSet::new();collect_names(&dir,&dir,&mut actual)?;actual.remove("MANIFEST.json");
    if actual!=manifest.files.keys().cloned().collect(){return Err("integrity:closure-namespace".into());}Ok(())
}
fn verify_links(root:&Path,file:&str,bytes:&[u8])->Result<()> {
    // Fixture-local grammar: Markdown links plus explicit source NAVIGATION
    // locators. This is not a general Markdown/YAML/JSON reference parser.
    let text=std::str::from_utf8(bytes)?;let mut targets=vec![];let mut rest=text;
    while let Some((_,after))=rest.split_once("](") {
        let (target,next)=after.split_once(')').ok_or("integrity:unfinished-link")?;targets.push(target);rest=next;
    }
    if file.starts_with("docs/")&&file.ends_with(".txt")||file.starts_with("docs/")&&file.ends_with(".md")||file.ends_with(".yaml") {
        targets.extend(text.split_whitespace().filter(|token|token.ends_with("NAVIGATION.md")));
    }
    for target in targets {
        if target.starts_with("https://")||target.starts_with('#'){continue;}
        let target=target.split('#').next().unwrap_or("");
        if target.is_empty(){continue;}if target.contains(':')||target.contains('\\'){return Err("integrity:nonlocal-link".into());}
        let mut relative=PathBuf::from(file).parent().unwrap().to_path_buf();
        for part in Path::new(target).components(){match part {
            std::path::Component::Normal(p)=>relative.push(p),
            std::path::Component::ParentDir=>{if !relative.pop(){return Err("integrity:link-escape".into());}},
            std::path::Component::CurDir=>{},_=>return Err("integrity:absolute-link".into()),
        }}
        if !root.join(&relative).exists(){return Err(format!("integrity:missing-readable-link:{file}:{target}").into());}
    }Ok(())
}
fn prepare(root:&Path,mode:&str,request:Request,initial:Option<State>,m:&mut Metrics)->Result<Candidate> {
    // GC takes this gate exclusively. A paused preparation is never presumed dead.
    let preparation=OpenOptions::new().create(true).truncate(false).read(true).write(true).open(root.join("preparation.lock"))?;
    FileExt::lock_shared(&preparation)?;
    let t=Instant::now(); let expected=if initial.is_some(){None}else{selector(root,m)?};
    if request.base.is_none()&&request.ops.iter().any(|o|o.kind=="import"){return Err("refused:import-base-required".into());}
    if request.base.is_some() && request.base!=expected {return Err("conflict:captured-base".into());}
    let mut s=if let Some(s)=initial{s}else{state(root,&publication(root,expected.as_ref().ok_or("integrity:uninitialized")?,m)?.candidate,m)?};
    let before=s.clone();let intent=io::digest(&io::json(&request)?);
    apply(root,&mut s,&request.ops,m)?;
    let attempt=io::unique("attempt");let generation=io::unique("generation");
    let dir=root.join(format!("attempts/{attempt}"));io::mkdir(dir.parent().unwrap(),m)?;fs::create_dir(&dir)?;
    io::sync_dir(dir.parent().unwrap(),m)?;
    let state_path=format!("attempts/{attempt}/state.json");let bytes=backend::save(root,&s,&before,expected.as_ref(),m)?;
    io::write_new(&root.join(&state_path),&bytes,m)?;io::checkpoint("state-installed")?;
    let mut changed=vec![];let mut removed=vec![];
    for (id,d) in &s.docs {if before.docs.get(id).is_none_or(|old|old.path!=d.path||old.digest!=d.digest){changed.push(id.clone());}}
    for (id,d) in &before.docs {if s.docs.get(id).is_none_or(|new|new.path!=d.path){removed.push(d.path.clone());}}
    if expected.is_none(){changed=s.docs.keys().cloned().collect();}
    let closure_path=if mode=="B"{format!("generations/{generation}")}else{format!("attempts/{attempt}/retained-packet")};
    let stage_path=if mode=="B"{format!("attempts/{attempt}/generation-stage")}else{closure_path.clone()};
    let manifest_digest=closure(root,&root.join(&stage_path),&s,mode=="B",request.packet,m)?;
    if mode=="B" {
        io::mkdir(root.join(&closure_path).parent().unwrap(),m)?;
        // Unique target; never replace a populated directory.
        fs::rename(root.join(&stage_path),root.join(&closure_path))?;
        io::sync_dir(root.join(&closure_path).parent().unwrap(),m)?;
        io::sync_dir(&dir,m)?;io::checkpoint("generation-installed")?;
    }
    let c=Candidate{attempt,generation,mode:mode.into(),request,expected,intent,state:state_path,state_digest:io::digest(&bytes),closure:closure_path,manifest_digest,changed,removed};
    io::write_new(&dir.join("candidate.json"),&io::json(&c)?,m)?;io::checkpoint("candidate-ready")?;m.elapsed("preparation-inclusive",t);Ok(c)
}
fn establish_predecessor(root:&Path,head:&Option<Selector>,m:&mut Metrics)->Result<()> {
    // All dependencies were synced before the selector could become visible.
    // Re-sync the visible selector and containing name, even after lost reply.
    if let Some(s)=head {publication(root,s,m)?;io::sync_file(&root.join("SELECTED.json"),m)?;io::sync_dir(root,m)?;}
    io::checkpoint("predecessor-established")?;Ok(())
}
fn preserve_derived(root:&Path,path:&Path,desired:&[u8],prior:Option<&[u8]>,m:&mut Metrics)->Result<()> {
    if path.exists(){let actual=io::read(path,m)?;if actual!=desired && prior.is_none_or(|old|actual!=old){
        let keep=root.join("divergence").join(io::unique("preserved-derived")).join(path.strip_prefix(root.join("view"))?);
        io::copy(path,&keep,m)?;eprintln!("divergence-preserved:{}",keep.display());io::checkpoint("view-divergence-preserved")?;
    }}Ok(())
}
fn packet_bytes(root:&Path,p:&Publication,name:&str,m:&mut Metrics)->Result<Vec<u8>>{
    let closure=root.join(&p.candidate.closure);let manifest:Manifest=serde_json::from_slice(&io::checked(&closure.join("MANIFEST.json"),&p.candidate.manifest_digest,m)?)?;
    let key=format!("packet/{name}");io::checked(&closure.join(&key),manifest.files.get(&key).ok_or("integrity:packet-manifest-entry")?,m)
}
fn refresh(root:&Path,p:&Publication,m:&mut Metrics)->Result<()> {
    let t=Instant::now();let s=state(root,&p.candidate,m)?;let view=root.join("view");io::mkdir(&view,m)?;
    let prior_status=fs::read(root.join("VIEW-STATUS.json")).ok().and_then(|b|serde_json::from_slice::<Value>(&b).ok());
    let prior_publication=if let Some(status)=prior_status.filter(|v|v["status"]=="complete") {
        let target=status["target_publication"].as_str().unwrap_or("");
        lineage(root,selector(root,m)?,m)?.into_iter().find(|old|old.id==target).filter(|old|lifecycle::check_live(root,&old.candidate).is_ok())
    }else{None};
    let prior_state=prior_publication.as_ref().map(|old|state(root,&old.candidate,m)).transpose()?;
    // A previous refresh may be incomplete. Compare each desired path with actual
    // bytes: copying only the latest delta would leave an older failed delta stale.
    let status=json!({"target_publication":p.id,"status":"pending","completed":0,"total":s.docs.len(),"exact_state":p.candidate.state,"retained_packet":p.candidate.closure});
    io::atomic(&root.join("VIEW-STATUS.json"),&io::json(&status)?,m)?;
    let desired:BTreeSet<_>=s.docs.values().map(|d|d.path.clone()).collect();
    let mut actual=BTreeSet::new();collect_names(&view,&view,&mut actual)?;
    for stale in actual.iter().filter(|p|p.starts_with("docs/")&&!desired.contains(*p)) {let keep=root.join("divergence").join(io::unique("removed-path")).join(stale);io::copy(&view.join(stale),&keep,m)?;fs::remove_file(view.join(stale))?;io::sync_dir(view.join(stale).parent().unwrap(),m)?;}
    for (i,d) in s.docs.values().enumerate() {
        let dest=view.join(&d.path);
        let old_doc=prior_state.as_ref().and_then(|old|old.docs.get(&d.id));
        let known=old_doc.is_some_and(|old|old.path==d.path&&old.digest==d.digest);
        // Fast incremental refresh assumes unchanged managed paths. Explicit
        // view-audit detects later damage; resume always inspects actual bytes.
        let actual=if known&&std::env::var("EXP_REBUILD").is_err(){None}else if dest.exists(){Some(io::digest(&io::read(&dest,m)?))}else{Some(String::new())};
        let equal=actual.as_ref().map_or(true,|hash|hash==&d.digest);
        let divergence=actual.as_ref().is_some_and(|hash|!hash.is_empty()&&hash!=&d.digest&&!old_doc.is_some_and(|old|old.digest==*hash));
        if divergence {let keep=root.join("divergence").join(io::unique("preserved")).join(&d.path);io::copy(&dest,&keep,m)?;eprintln!("divergence-preserved:{}",keep.display());}
        if !equal {
            io::mkdir(dest.parent().unwrap(),m)?;let stage=dest.with_file_name(io::unique("view-stage"));
            io::copy(&root.join(&d.original),&stage,m)?;io::checked(&stage,&d.digest,m)?;
            io::replace(&stage,&dest,m)?;io::sync_dir(dest.parent().unwrap(),m)?;io::checkpoint("view-file-replaced")?;
        }
        if i==0 {io::checkpoint("view-first-document")?;}
    }
    let nav_time=Instant::now();let nav=navigation(&s);let old_nav=prior_state.as_ref().map(navigation);m.elapsed("navigation-render",nav_time);
    preserve_derived(root,&view.join("NAVIGATION.md"),nav.as_bytes(),old_nav.as_ref().map(|s|s.as_bytes()),m)?;io::atomic(&view.join("NAVIGATION.md"),nav.as_bytes(),m)?;
    for name in ["context.md","provenance.json"] {
        let dest=view.join("packet").join(name);io::mkdir(dest.parent().unwrap(),m)?;
        let bytes=packet_bytes(root,p,name,m)?;let old=prior_publication.as_ref().map(|old|packet_bytes(root,old,name,m)).transpose()?;
        preserve_derived(root,&dest,&bytes,old.as_deref(),m)?;io::atomic(&dest,&bytes,m)?;io::checkpoint("view-derived-written")?;
    }
    io::checkpoint("view-before-complete")?;
    io::atomic(&root.join("VIEW-STATUS.json"),&io::json(&json!({"target_publication":p.id,"status":"complete","completed":s.docs.len(),"total":s.docs.len(),"exact_state":p.candidate.state,"retained_packet":p.candidate.closure}))?,m)?;
    m.elapsed("view-refresh-inclusive",t);Ok(())
}
fn select_locked(root:&Path,c:&Candidate,m:&mut Metrics)->Result<Value> {
    let t=Instant::now();let head=selector(root,m)?;establish_predecessor(root,&head,m)?;
    io::checkpoint("before-dedup")?;
    let resolved=resolve(root,&c.request.request,Some(&c.intent),m)?;
    if resolved["outcome"]!="absent-as-of" {return Ok(json!({"status":"deduplicated","result":resolved}));}
    lifecycle::check_candidate(root,c,m)?;
    io::checkpoint("after-dedup")?;
    if root.join(format!("fences/{}.json",io::digest(c.request.request.as_bytes()))).exists() || root.join(format!("cancel-signals/{}",io::digest(c.request.request.as_bytes()))).exists() {return Err("refused:cancelled-fenced".into());}
    if c.expected!=head {return Err("conflict:expected-publication".into());}
    io::checkpoint("expected-checked")?;
    let s=state(root,c,m)?;let validation=Instant::now();validate(&s)?;m.elapsed("gate-validation",validation);
    if c.mode=="B"{verify_closure(root,c,m)?;} // installed names are checked at selection
    let id=io::unique("publication");
    let result=json!({"outcome":"accepted","request":c.request.request,"publication":id,"state_digest":c.state_digest,"changed":c.changed,"removed":c.removed});
    let p=Publication{id:id.clone(),predecessor:head,candidate:c.clone(),result:result.clone()};
    let bytes=io::json(&p)?;let sel=Selector{publication:id.clone(),digest:io::digest(&bytes)};
    io::write_new(&root.join(format!("publications/{id}.json")),&bytes,m)?;io::checkpoint("publication-installed")?;
    let stage=root.join(io::unique("selector-stage"));io::write_new(&stage,&io::json(&sel)?,m)?;io::checkpoint("selector-staged")?;
    let mut boundary=false;
    let delivery=(||->Result<Value>{
    io::replace(&stage,&root.join("SELECTED.json"),m)?;io::checkpoint("selector-visible")?;
    io::sync_dir(root,m)?;boundary=true;io::checkpoint("selection-boundary")?;m.elapsed("selection-inclusive",t);
    let refresh_result=if c.mode=="A"{refresh(root,&p,m)}else{Ok(())};
    // A refresh errors cannot turn an accepted publication into noncommit.
    let status=match refresh_result {Ok(())=>"published",Err(_)=>"published/view-pending"};
    let view_error=refresh_result.err().map(|e|e.to_string());
    io::checkpoint("receipt-before")?;
    io::atomic(&root.join(format!("receipts/{}.json",io::digest(c.request.request.as_bytes()))),&io::json(&result)?,m)?;
    io::checkpoint("receipt-written")?;io::checkpoint("reply")?;
    Ok(json!({"status":status,"result":result,"view_error":view_error,"selected":sel,"readable":if c.mode=="B"{c.closure.clone()}else{"view".into()}}))
    })();
    match delivery {Ok(v)=>Ok(v),Err(e)=>Ok(json!({"status":if boundary{"published/reply-pending"}else{"selection-uncertain"},"result":if boundary{result}else{Value::Null},"proposed_publication":sel,"delivery_error":e.to_string(),"resolve_request":c.request.request}))}
}
fn publish(root:&Path,mode:&str,r:Request,policy:&str,m:&mut Metrics)->Result<Value> {
    if !["A","B"].contains(&mode){return Err("invalid-mode".into());}
    let intent=io::digest(&io::json(&r)?);
    let original=resolve(root,&r.request,Some(&intent),m)?;
    if original["outcome"]!="absent-as-of"{let _guard=lock(root,m)?;establish_predecessor(root,&selector(root,m)?,m)?;return Ok(json!({"status":"deduplicated","result":original}));}
    io::checkpoint("request-unresolved")?;
    if policy=="slot" {
        let _guard=lock(root,m)?;let t=Instant::now();
        let result=(||->Result<Value>{
        establish_predecessor(root,&selector(root,m)?,m)?;
        let original=resolve(root,&r.request,Some(&intent),m)?;
        if original["outcome"]!="absent-as-of"{return Ok(json!({"status":"deduplicated","result":original}));}
        if root.join(format!("cancel-signals/{}",io::digest(r.request.as_bytes()))).exists(){return Err("cancelled:signal-awaits-fence".into());}
        // Null base explicitly opts pure fixture commands into admission-time
        // capture. Import must carry an exact captured base.
        if r.base.is_none()&&r.ops.iter().any(|o|o.kind=="import"){return Err("refused:import-base-required".into());}
        let c=prepare(root,mode,r,None,m)?;select_locked(root,&c,m)
        })();m.elapsed("lock-hold",t);result
    }else if policy=="speculative"{
        let request_id=r.request.clone();
        let c=match prepare(root,mode,r,None,m){Ok(c)=>c,Err(error)=>{
            // An accepted same-intent predecessor may make a non-idempotent
            // command fail during preparation (e.g. deletion already applied).
            let _guard=lock(root,m)?;let t=Instant::now();establish_predecessor(root,&selector(root,m)?,m)?;
            let original=resolve(root,&request_id,Some(&intent),m)?;m.elapsed("lock-hold",t);
            if original["outcome"]!="absent-as-of"{return Ok(json!({"status":"deduplicated","result":original}));}return Err(error);
        }};
        let _guard=lock(root,m)?;let t=Instant::now();let result=select_locked(root,&c,m);m.elapsed("lock-hold",t);result
    }else{Err("invalid-policy".into())}
}
fn inspect(root:&Path,audit:bool,m:&mut Metrics)->Result<Value> {
    let t=Instant::now();let selected=selector(root,m)?.ok_or("integrity:missing-selector")?;
    let history=lineage(root,Some(selected.clone()),m)?;let p=history.first().ok_or("integrity:empty-history")?;
    let s=state(root,&p.candidate,m)?;
    if audit {
        for prior in &history {
            if lifecycle::check_live(root,&prior.candidate).is_err(){continue;}
            let s=state(root,&prior.candidate,m)?;
            for d in s.docs.values(){io::checked(&root.join(&d.original),&d.digest,m)?;}
            verify_closure(root,&prior.candidate,m)?;
        }
    }
    m.elapsed(if audit{"audit-inclusive"}else{"lookup-inclusive"},t);
    Ok(json!({"selected":selected,"candidate":p.candidate,"count":s.docs.len(),"history":history.iter().map(|p|json!({"publication":p.id,"request":p.candidate.request.request,"result":p.result})).collect::<Vec<_>>(),"positive":s.docs.get("d000001"),"negative":s.docs.get("not-present")}))
}
fn run(v:Value,m:&mut Metrics)->Result<Value> {
    let root=PathBuf::from(v["root"].as_str().ok_or("root-required")?);
    let action=v["action"].as_str().ok_or("action-required")?;
    if action!="init"&&!root.join("SELECTED.json").is_file(){return Err("integrity:missing-selected-authority".into());}
    match action {
        "init"=>{
            if root.exists(){return Err("refuse-existing-root".into());}io::mkdir(&root,m)?;
            io::write_new(&root.join("STORE.json"),&io::json(&json!({"version":1,"mode":v["mode"],"consumer":v["consumer"]}))?,m)?;
            io::write_new(&root.join("README.md"),b"# File-native collection\n\nSELECTED.json is the sole publication authority. Follow its publication record and candidate. B: open the retained generation path once, then NAVIGATION.md. A: view/NAVIGATION.md may be mixed or stale; VIEW-STATUS.json names its target and immutable state/retained packet route. Edits are not automatically admitted. Never infer selection from highest directory number or time.\n",m)?;
            let _guard=lock(&root,m)?;let t=Instant::now();
            let s=fixture(&root,v["consumer"].as_str().ok_or("consumer")?,v["count"].as_u64().ok_or("count")? as usize,m)?;
            let c=prepare(&root,v["mode"].as_str().ok_or("mode")?,Request{request:"initial".into(),base:None,ops:vec![],packet:v["packet"].as_bool().unwrap_or(true)},Some(s),m)?;
            let out=select_locked(&root,&c,m);m.elapsed("initialization-inclusive",t);out
        }
        "inspect"=>inspect(&root,v["audit"].as_bool().unwrap_or(false),m),
        "point"=>{
            let sel:Selector=if v["selected"].is_object(){serde_json::from_value(v["selected"].clone())?}else{selector(&root,m)?.ok_or("missing-selected")?};
            let p=retained_publication(&root,&sel,m)?;lifecycle::check_live(&root,&p.candidate)?;
            let t=Instant::now();let value=backend::point(&root,&p.candidate,v["id"].as_str().ok_or("id")?,m)?;m.elapsed("point-inclusive",t);
            Ok(json!({"selected":sel,"value":value}))
        }
        "packet"=>{
            let _guard=lock(&root,m)?;let sel:Selector=if v["selected"].is_object(){serde_json::from_value(v["selected"].clone())?}else{selector(&root,m)?.ok_or("missing-selected")?};
            let p=retained_publication(&root,&sel,m)?;let s=state(&root,&p.candidate,m)?;
            let dest=format!("packets/{}",io::unique("packet"));let hash=closure(&root,&root.join(&dest),&s,false,true,m)?;
            lifecycle::register_packet(&root,&dest,&sel,&s,m)?;
            Ok(json!({"path":dest,"manifest_digest":hash,"selected":sel}))
        }
        "view-audit"=>{
            let sel=selector(&root,m)?.ok_or("missing-selected")?;let p=publication(&root,&sel,m)?;let s=state(&root,&p.candidate,m)?;
            let mut missing=vec![];let mut edited=vec![];
            for d in s.docs.values(){let path=root.join("view").join(&d.path);if !path.exists(){missing.push(d.id.clone());}else if io::digest(&io::read(&path,m)?)!=d.digest{edited.push(d.id.clone());}}
            let nav=navigation(&s);let mut derived=vec![("NAVIGATION.md".to_string(),nav.into_bytes())];
            for name in ["context.md","provenance.json"]{derived.push((format!("packet/{name}"),packet_bytes(&root,&p,name,m)?));}
            for (name,expected) in derived {let path=root.join("view").join(&name);if !path.exists(){missing.push(name);}else if io::read(&path,m)?!=expected{edited.push(name);}}
            Ok(json!({"missing":missing,"edited":edited,"selected":sel,"marker_is_integrity_proof":false}))
        }
        "lifecycle"=>lifecycle::run(&root,&v,m),
        "cancel-signal"=>{
            let id=v["request_id"].as_str().ok_or("request_id")?;io::atomic(&root.join(format!("cancel-signals/{}",io::digest(id.as_bytes()))),b"cancel",m)?;
            Ok(json!({"status":"signal-only-not-terminal","request":id}))
        }
        "io-probe"=>{
            let dest=root.join("probe/nested/content");io::mkdir(dest.parent().unwrap(),m)?;
            let b=v["text"].as_str().unwrap_or("same verified bytes").as_bytes();let stage=dest.with_file_name(io::unique("probe-stage"));
            if v["short"].as_bool().unwrap_or(false){io::write_new(&stage,&b[..b.len()/2],m)?;return Err("injected:short-stage".into());}
            io::write_new(&stage,b,m)?;io::install_original(&stage,&dest,&io::digest(b),m)?;io::sync_dir(dest.parent().unwrap(),m)?;
            Ok(json!({"sha256":io::digest(&io::checked(&dest,&io::digest(b),m)?)}))
        }
        "publish"=>publish(&root,v["mode"].as_str().ok_or("mode")?,serde_json::from_value(v["request"].clone())?,v["policy"].as_str().unwrap_or("speculative"),m),
        "prepare"=>{
            let c=prepare(&root,v["mode"].as_str().ok_or("mode")?,serde_json::from_value(v["request"].clone())?,None,m)?;
            Ok(json!({"candidate":c,"file":format!("attempts/{}/candidate.json",c.attempt)}))
        }
        "select"=>{
            let c:Candidate=serde_json::from_slice(&io::read(&root.join(v["candidate"].as_str().ok_or("candidate")?),m)?)?;
            let _guard=lock(&root,m)?;let t=Instant::now();let out=select_locked(&root,&c,m);m.elapsed("lock-hold",t);out
        }
        "resolve"=>resolve(&root,v["request_id"].as_str().ok_or("request_id")?,None,m),
        "resume"=>{
            std::env::set_var("EXP_REBUILD","1");
            let _guard=lock(&root,m)?;let sel=selector(&root,m)?.ok_or("no-selected")?;let p=publication(&root,&sel,m)?;
            if p.candidate.mode!="A"{return Err("resume-only-A".into());}let t=Instant::now();refresh(&root,&p,m)?;m.elapsed("recovery-inclusive",t);Ok(json!({"status":"view-resumed","publication":p.id}))
        }
        "cancel"=>{
            let _guard=lock(&root,m)?;let id=v["request_id"].as_str().ok_or("request_id")?;
            let existing=resolve(&root,id,None,m)?;if existing["outcome"]!="absent-as-of"{return Ok(existing);}
            io::atomic(&root.join(format!("fences/{}.json",io::digest(id.as_bytes()))),&io::json(&json!({"request":id,"fenced":true}))?,m)?;
            Ok(json!({"outcome":"cancelled-fenced","terminal_noncommit":true}))
        }
        "repeat-error"=>{
            let r:Request=serde_json::from_value(v["request"].clone())?;
            let mut errors=vec![];std::env::set_var("EXP_ERROR","candidate-ready");
            for _ in 0..3 {errors.push(publish(&root,v["mode"].as_str().ok_or("mode")?,r.clone(),"speculative",m).unwrap_err().to_string());}
            std::env::remove_var("EXP_ERROR");let out=publish(&root,v["mode"].as_str().ok_or("mode")?,r,"speculative",m)?;Ok(json!({"errors":errors,"final":out}))
        }
        _=>Err("unknown-action".into()),
    }
}
fn main() {
    let start=Instant::now();let mut m=Metrics::default();let mut input=String::new();std::io::stdin().read_to_string(&mut input).unwrap();
    let result=(||{if input.len()>4*1024*1024{return Err("request-cap".into());}let v=strict_json(&input)?;run(v,&mut m)})();
    let (status,result,error)=match result {Ok(v)=>("ok",v,None),Err(e)=>("error",Value::Null,Some(e.to_string()))};
    println!("{}",json!({"status":status,"result":result,"error":error,"metrics":m,"wall_ns":start.elapsed().as_nanos(),"peak_memory_bytes":io::peak_memory()}));
    if status=="error"{std::process::exit(2);}
}

fn strict_json(text:&str)->Result<Value> {
    // Parse before Value can collapse duplicate fields, including nested ops.
    struct Strict(Value);
    impl<'de> Deserialize<'de> for Strict {
        fn deserialize<D:serde::Deserializer<'de>>(d:D)->std::result::Result<Self,D::Error>{
            struct Visitor;
            impl<'de> serde::de::Visitor<'de> for Visitor {
                type Value=Strict;
                fn expecting(&self,f:&mut std::fmt::Formatter)->std::fmt::Result{f.write_str("JSON without duplicate keys")}
                fn visit_bool<E:serde::de::Error>(self,v:bool)->std::result::Result<Strict,E>{Ok(Strict(json!(v)))}
                fn visit_i64<E:serde::de::Error>(self,v:i64)->std::result::Result<Strict,E>{Ok(Strict(json!(v)))}
                fn visit_u64<E:serde::de::Error>(self,v:u64)->std::result::Result<Strict,E>{Ok(Strict(json!(v)))}
                fn visit_f64<E:serde::de::Error>(self,v:f64)->std::result::Result<Strict,E>{Ok(Strict(json!(v)))}
                fn visit_str<E:serde::de::Error>(self,v:&str)->std::result::Result<Strict,E>{Ok(Strict(json!(v)))}
                fn visit_unit<E:serde::de::Error>(self)->std::result::Result<Strict,E>{Ok(Strict(Value::Null))}
                fn visit_seq<A:serde::de::SeqAccess<'de>>(self,mut a:A)->std::result::Result<Strict,A::Error>{let mut v=vec![];while let Some(x)=a.next_element::<Strict>()?{v.push(x.0);}Ok(Strict(Value::Array(v)))}
                fn visit_map<A:serde::de::MapAccess<'de>>(self,mut a:A)->std::result::Result<Strict,A::Error>{
                    let mut v=serde_json::Map::new();while let Some(k)=a.next_key::<String>()? {
                        if v.contains_key(&k){return Err(serde::de::Error::custom(format!("duplicate-key:{k}")));}v.insert(k,a.next_value::<Strict>()?.0);
                    } Ok(Strict(Value::Object(v)))
                }
            } d.deserialize_any(Visitor)
        }
    }
    let mut d=serde_json::Deserializer::from_str(text);let v=Strict::deserialize(&mut d)?.0;d.end()?;Ok(v)
}
