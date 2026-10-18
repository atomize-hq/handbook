"""One-shot deterministic source extension; inputs are the copied landed baseline.

Run only when reproducing source construction in a fresh directory. Final source
is committed directly; this script documents transformations rather than running
automatically at build time.
"""
from pathlib import Path
H=Path(__file__).resolve().parent
def change(file,old,new):
    p=H/file;s=p.read_text(encoding='utf8');assert old in s,(file,old[:80]);p.write_text(s.replace(old,new),encoding='utf8')
disk=(H.parent/'durable-membership-prototype-2026-10-12/src/disk.rs').read_text(encoding='utf8').split('#[cfg(test)]')[0]
disk=disk.replace('const MAX_VALUE_BYTES: usize = 1024;','const MAX_VALUE_BYTES: usize = 8192;')
disk+='''
impl DiskIndex {
    pub fn entries(&mut self, store: &mut Objects) -> Result<Vec<(String,Vec<u8>)>> {
        fn visit(store:&mut Objects,id:&str,expected:Option<&Bounds>,out:&mut Vec<(String,Vec<u8>)>,seen:&mut HashSet<Id>)->Result<()> {
            if !seen.insert(id.to_owned()) || seen.len()>MAX_AUDIT_NODES {return Err("integrity:cow-cycle-or-cap".into());}
            match read_node(store,id,expected)? {
                Node::Leaf{pairs}=>out.extend(pairs),
                Node::Branch{height,children}=>for c in children {visit(store,&c.id,Some(&expected_child(&c,height-1)),out,seen)?;},
            } Ok(())
        }
        let mut out=vec![];if let Some(id)=&self.root {visit(store,id,None,&mut out,&mut HashSet::new())?;}Ok(out)
    }
}
'''
(H/'src/disk.rs').write_text(disk,encoding='utf8')
change('src/main.rs','mod model;','mod model;\nmod disk;\nmod backend;\nmod lifecycle;\nuse backend::{Objects,Id};\nconst MAX_OBJECT:usize=1024*1024;')
change('src/model.rs','pub base:Option<Selector>, pub ops:Vec<Op>','pub base:Option<Selector>, pub ops:Vec<Op>, #[serde(default="packet_default")] pub packet:bool')
change('src/model.rs','pub fn safe_id','pub fn packet_default()->bool {true}\npub fn safe_id')
change('src/main.rs','let s:State=serde_json::from_slice(&io::checked(&root.join(&c.state),&c.state_digest,m)?)?;','lifecycle::check_live(root,c)?;\n    let s:State=backend::load(root,c,m)?;')
change('src/main.rs','if t.elapsed()>Duration::from_secs(15){return Err(format!("busy:lock-timeout:{e}").into());}', 'if let Ok(id)=std::env::var("EXP_CANCEL_ID") {if root.join(format!("cancel-signals/{}",io::digest(id.as_bytes()))).exists(){m.elapsed("lock-wait",t);return Err("cancelled:while-waiting".into());}}\n        if t.elapsed()>Duration::from_millis(std::env::var("EXP_WAIT_MS").ok().and_then(|s|s.parse().ok()).unwrap_or(15000)){m.elapsed("lock-wait",t);return Err(format!("busy:lock-timeout:{e}").into());}')
change('src/main.rs','fn navigation(s:&State)->String {','fn navigation(s:&State)->String {')
change('src/main.rs','fn closure(root:&Path,dest:&Path,s:&State,all:bool,m:&mut Metrics)->Result<String> {','fn closure(root:&Path,dest:&Path,s:&State,all:bool,packet:bool,m:&mut Metrics)->Result<String> {\n    let started=Instant::now(); eprintln!("progress:materialization-start:{}",s.docs.len());')
change('src/main.rs','let packet_ids:Vec<_>=s.docs.keys().take(8).cloned().collect();','let packet_ids:Vec<_>=if packet{s.docs.keys().take(8).cloned().collect()}else{vec![]};')
change('src/main.rs','let nav=navigation(&readable_state);','let nav=navigation(&readable_state);\n    m.elapsed("materialization-inclusive",started);eprintln!("progress:verification-start:{}",files.len());')
change('src/main.rs','m.elapsed("destination-verification",t);io::checkpoint("closure-verified")?;','m.elapsed("destination-verification",t);io::checkpoint("closure-verified")?;eprintln!("progress:verification-complete:{}",files.len());')
change('src/main.rs','let bytes=io::json(&s)?;\n    io::write_new(&root.join(&state_path),&bytes,m)?;', 'let bytes=backend::save(root,&s,&before,expected.as_ref(),m)?;\n    io::write_new(&root.join(&state_path),&bytes,m)?;')
change('src/main.rs','&s,mode=="B",m)?','&s,mode=="B",request.packet,m)?')
change('src/main.rs','Request{request:"initial".into(),base:None,ops:vec![]}', 'Request{request:"initial".into(),base:None,ops:vec![],packet:v["packet"].as_bool().unwrap_or(true)}')
change('src/main.rs','let prior_status=fs::read(root.join("VIEW-STATUS.json")).ok()', 'let prior_status=fs::read(root.join("VIEW-STATUS.json")).ok()')
# A complete marker is a performance hint, not evidence of continuing byte integrity.
# Preserve divergent source bytes before explicit rebuild or refresh replacement.
change('src/main.rs','let known=prior_state.as_ref().and_then(|old|old.docs.get(&d.id)).is_some_and(|old|old.path==d.path&&old.digest==d.digest);','let _known=prior_state.as_ref().and_then(|old|old.docs.get(&d.id)).is_some_and(|old|old.path==d.path&&old.digest==d.digest);')
change('src/main.rs','let equal=if known{true}else if dest.exists(){io::digest(&io::read(&dest,m)?)==d.digest}else{false};','let equal=if dest.exists(){io::digest(&io::read(&dest,m)?)==d.digest}else{false};\n        if !equal && dest.exists() {let keep=root.join("divergence").join(io::unique("preserved")).join(&d.path);io::copy(&dest,&keep,m)?;eprintln!("divergence-preserved:{}",keep.display());}')
change('src/main.rs','for stale in actual.iter().filter(|p|p.starts_with("docs/")&&!desired.contains(*p)) {fs::remove_file(view.join(stale))?;', 'for stale in actual.iter().filter(|p|p.starts_with("docs/")&&!desired.contains(*p)) {let keep=root.join("divergence").join(io::unique("removed-path")).join(stale);io::copy(&view.join(stale),&keep,m)?;fs::remove_file(view.join(stale))?;')
change('src/main.rs','verify_closure(root,&prior.candidate,m)?;', 'verify_closure(root,&prior.candidate,m)?;')
change('src/main.rs','let s=state(root,&prior.candidate,m)?;', 'if lifecycle::is_retired(root,&prior.id){continue;}\n            let s=state(root,&prior.candidate,m)?;')
change('src/main.rs','"inspect"=>inspect(&root,v["audit"].as_bool().unwrap_or(false),m),','''"inspect"=>inspect(&root,v["audit"].as_bool().unwrap_or(false),m),
        "point"=>{
            let sel:Selector=if v["selected"].is_object(){serde_json::from_value(v["selected"].clone())?}else{selector(&root,m)?.ok_or("missing-selected")?};
            let p=publication(&root,&sel,m)?;lifecycle::check_live(&root,&p.candidate)?;
            let t=Instant::now();let value=backend::point(&root,&p.candidate,v["id"].as_str().ok_or("id")?,m)?;m.elapsed("point-inclusive",t);
            Ok(json!({"selected":sel,"value":value}))
        }
        "packet"=>{
            let _guard=lock(&root,m)?;let sel:Selector=if v["selected"].is_object(){serde_json::from_value(v["selected"].clone())?}else{selector(&root,m)?.ok_or("missing-selected")?};
            let p=publication(&root,&sel,m)?;let s=state(&root,&p.candidate,m)?;
            let dest=format!("packets/{}",io::unique("packet"));let hash=closure(&root,&root.join(&dest),&s,false,true,m)?;
            lifecycle::register_packet(&root,&dest,&sel,&s,m)?;
            Ok(json!({"path":dest,"manifest_digest":hash,"selected":sel}))
        }
        "view-audit"=>{
            let sel=selector(&root,m)?.ok_or("missing-selected")?;let p=publication(&root,&sel,m)?;let s=state(&root,&p.candidate,m)?;
            let mut missing=vec![];let mut edited=vec![];
            for d in s.docs.values(){let path=root.join("view").join(&d.path);if !path.exists(){missing.push(d.id.clone());}else if io::digest(&io::read(&path,m)?)!=d.digest{edited.push(d.id.clone());}}
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
        }''')
# terminal cancellation fence is also checked immediately after waiting for a slot
change('src/main.rs','let _guard=lock(root,m)?;let t=Instant::now();\n        // Null base', 'let _guard=lock(root,m)?;let t=Instant::now();\n        if root.join(format!("cancel-signals/{}",io::digest(r.request.as_bytes()))).exists(){return Err("cancelled:signal-awaits-fence".into());}\n        // Null base')
# Respect a signal in speculative selection too; durable fence is explicit cancel.
change('src/main.rs','if root.join(format!("fences/{}.json",io::digest(c.request.request.as_bytes()))).exists()', 'if root.join(format!("fences/{}.json",io::digest(c.request.request.as_bytes()))).exists() || root.join(format!("cancel-signals/{}",io::digest(c.request.request.as_bytes()))).exists()')
