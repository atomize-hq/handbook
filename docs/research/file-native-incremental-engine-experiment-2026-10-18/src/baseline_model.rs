use crate::io::{self,Metrics,Result};
use serde::{Deserialize,Serialize};
use std::{collections::{BTreeMap,BTreeSet},path::Path,time::Instant};
#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Eq)]
#[serde(deny_unknown_fields)]
pub struct Selector { pub publication: String, pub digest: String }
#[derive(Clone,Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Edge { pub kind:String, pub target:String, pub external:bool }
#[derive(Clone,Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Doc {
    pub id:String, pub title:String, pub path:String, pub original:String,
    pub digest:String, pub bytes:u64, pub kind:String, pub label:Option<String>,
    pub cost:u64, pub edges:Vec<Edge>,
}
#[derive(Clone,Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
pub struct State { pub consumer:String, pub budget:u64, pub docs:BTreeMap<String,Doc> }
#[derive(Clone,Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Op {
    pub kind:String, pub id:String,
    #[serde(default)] pub text:String,
    #[serde(default)] pub target:String,
    #[serde(default)] pub value:u64,
}
#[derive(Clone,Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Request { pub request:String, pub base:Option<Selector>, pub ops:Vec<Op>, #[serde(default="packet_default")] pub packet:bool }
#[derive(Clone,Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Candidate {
    pub attempt:String, pub generation:String, pub mode:String, pub request:Request,
    pub expected:Option<Selector>,
    pub intent:String, pub state:String, pub state_digest:String,
    pub closure:String, pub manifest_digest:String,
    pub changed:Vec<String>, pub removed:Vec<String>,
}
#[derive(Clone,Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Publication {
    pub id:String, pub predecessor:Option<Selector>, pub candidate:Candidate,
    pub result:serde_json::Value,
}
pub fn packet_default()->bool {true}
pub fn safe_id(s:&str)->bool { !s.is_empty() && s.len()<100 && s.bytes().all(|b|b.is_ascii_alphanumeric()||b==b'-'||b==b'_') }
pub fn path_for(id:&str,title:&str,ext:&str)->String {
    let slug:String=title.chars().map(|c|if c.is_ascii_alphanumeric(){c.to_ascii_lowercase()}else{'-'}).take(38).collect();
    format!("docs/item-{}-{}.{}",slug,id,ext)
}
pub fn validate(state:&State)->Result<()> {
    let mut paths=BTreeSet::new(); let mut labels=BTreeSet::new(); let mut cost=0u64;
    for (id,d) in &state.docs {
        if !safe_id(id)||d.id!=*id { return Err("semantic:id".into()); }
        if !d.path.starts_with("docs/item-")||d.path.contains("..")||d.path.contains('\\')||!paths.insert(d.path.to_lowercase()) {return Err("semantic:path".into());}
        if let Some(l)=&d.label { if !labels.insert(l) {return Err("semantic:unique".into());} }
        cost=cost.checked_add(d.cost).ok_or("semantic:overflow")?;
        for e in &d.edges {
            if !["requires","cites","constituent","attachment"].contains(&e.kind.as_str()) {return Err("semantic:edge-kind".into());}
            if !e.external && !state.docs.contains_key(&e.target) {return Err("semantic:local-reference".into());}
            if e.external && !e.target.starts_with("https://") {return Err("semantic:external-locator".into());}
        }
    }
    if cost>state.budget {return Err("semantic:budget".into());} Ok(())
}
pub fn put_original(root:&Path,d:&mut Doc,bytes:&[u8],m:&mut Metrics)->Result<()> {
    d.digest=io::digest(bytes); d.bytes=bytes.len() as u64;
    let name=Path::new(&d.path).file_name().ok_or("bad-path")?.to_string_lossy();
    d.original=if std::env::var("EXP_LAYOUT").ok().as_deref()==Some("sharded") {
        format!("originals/{}/{}-{name}",&d.digest[..2],d.digest)
    }else{format!("originals/{}/{name}",d.digest)};
    let path=root.join(&d.original);
    if path.exists(){io::checked(&path,&d.digest,m)?;}else{
        io::checkpoint("payload-before-write")?;io::mkdir(path.parent().unwrap(),m)?;
        let stage=path.with_file_name(io::unique("payload-stage"));
        if std::env::var("EXP_ERROR").ok().as_deref()==Some("payload-partial-write") {
            io::write_new(&stage,&bytes[..bytes.len()/2],m)?;
            return Err("injected-io:payload-partial-write".into());
        }
        io::write_new(&stage,bytes,m)?;io::checked(&stage,&d.digest,m)?;
        if path.exists(){io::checked(&path,&d.digest,m)?;}else{io::install_original(&stage,&path,&d.digest,m)?;}
        io::sync_dir(path.parent().unwrap(),m)?;io::checkpoint("payload-installed")?;
    }
    Ok(())
}
pub fn fixture(root:&Path,consumer:&str,n:usize,m:&mut Metrics)->Result<State> {
    let creation=Instant::now();
    let mut s=State{consumer:consumer.into(),budget:1000,docs:BTreeMap::new()};
    for i in 0..n {
        if i%1000==0 { eprintln!("progress:creation:{i}/{n}:{}",serde_json::to_string(&m)?); }
        let id=format!("d{i:06}");
        let special=["Overview","Requirement α","Decision café","CON","con.","AUX ","Case","case","é","e\u{301}"];
        let title=if i<10{special[i].to_owned()}else{format!("{} source {i}",consumer)};
        let ext=match i%10{0|1=>"yaml",2=>"json",9=>"txt",_=>"md"};
        let mut d=Doc{id:id.clone(),title:title.clone(),path:path_for(&id,&title,ext),original:String::new(),digest:String::new(),bytes:0,kind:match i%10{0=>"overview",1=>"requirement",2=>"exact-json",_=>if consumer=="project"{"decision-source"}else{"note-quote"}}.into(),label:None,cost:0,edges:vec![]};
        if i==0 {d.edges.push(Edge{kind:"requires".into(),target:"d000001".into(),external:false});}
        if i==0&&n>=10 {d.edges.push(Edge{kind:"attachment".into(),target:"d000009".into(),external:false});}
        if i==9 {d.kind="attachment".into();d.path=format!("docs/item-attachments-{id}/evidence/deep/source.txt");}
        if i==1 {d.edges.push(Edge{kind:"cites".into(),target:"d000002".into(),external:false});}
        if i==2 {d.edges.push(Edge{kind:"cites".into(),target:"https://unavailable.invalid/source".into(),external:true});}
        if i==3 {d.label=Some("one-label".into());}
        if i==0&&n>=10000 { for k in 1..=5000 {d.edges.push(Edge{kind:"constituent".into(),target:format!("d{k:06}"),external:false});} }
        let size=match i%10{0..=6=>4096,7|8=>16384,_=>65536};
        let line=if i%2==0{"\n"}else{"\r\n"};
        let nav=format!("{}NAVIGATION.md","../".repeat(Path::new(&d.path).parent().unwrap().components().count()));
        let head=if ext=="json"{format!("{{\r\n  \"id\": \"{id}\", \"exact_number\": 1.00, \"text\": \"café α\"\r\n}}\r\n")}
            else if ext=="yaml" {format!("id: {id}{line}title: \"{title}\"{line}consumer: {consumer}{line}# Relationships and exact paths: ../NAVIGATION.md{line}")}
            else {format!("# {title}{line}{line}Stable identity: {id}. Collection: {consumer}.{line}See {nav} for exact relationships.{line}")};
        let mut b=head.into_bytes();
        if ext=="json" {b.resize(size,b' ');} else {let prose=format!("Evidence {id}: a reusable source preserves exact captured text.{line}");while b.len()+prose.len()<=size {b.extend(prose.as_bytes());} b.resize(size,b' ');}
        put_original(root,&mut d,&b,m)?; s.docs.insert(id,d);
    }
    eprintln!("progress:creation-complete:{n}:{}",serde_json::to_string(&m)?);
    m.elapsed("creation-inclusive",creation);validate(&s)?; Ok(s)
}
pub fn apply(root:&Path,s:&mut State,ops:&[Op],m:&mut Metrics)->Result<()> {
    for o in ops {
        if o.kind=="delete" {s.docs.remove(&o.id).ok_or("semantic:missing-document")?;continue;}
        let d=s.docs.get_mut(&o.id).ok_or("semantic:missing-document")?;
        match o.kind.as_str() {
            "edit"=>{let mut b=io::checked(&root.join(&d.original),&d.digest,m)?;b.extend(o.text.as_bytes());put_original(root,d,&b,m)?;}
            "import"=>{put_original(root,d,o.text.as_bytes(),m)?;}
            "rename"=>{d.title=o.text.clone();let ext=Path::new(&d.path).extension().unwrap().to_string_lossy().to_string();d.path=path_for(&d.id,&d.title,&ext);}
            "edge"=>d.edges.push(Edge{kind:o.text.clone(),target:o.target.clone(),external:false}),
            "label"=>d.label=Some(o.text.clone()),
            "cost"=>d.cost=o.value,
            _=>return Err("semantic:unknown-operation".into()),
        }
    }
    let t=Instant::now();validate(s)?;m.elapsed("semantic-validation",t);Ok(())
}
