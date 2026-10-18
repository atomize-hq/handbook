//! Reused lazy disk tree; no backend HEAD. Only application SELECTED is authority.
use crate::{io::{self,Metrics,Result},model::*,disk::DiskIndex};
use serde::{Serialize,Deserialize};
use std::{path::{Path,PathBuf},collections::BTreeMap,time::Instant};
pub type Id=String;
pub struct Objects<'a>{pub base:PathBuf,pub metrics:&'a mut Metrics}
impl Objects<'_>{
    pub fn read(&mut self,id:&str)->Result<Vec<u8>> {
        if id.len()!=64||!id.bytes().all(|b|b.is_ascii_hexdigit()){return Err("integrity:object-id".into());}
        let b=io::checked(&self.base.join("objects").join(&id[..2]).join(id),id,self.metrics)?;
        self.metrics.bytes_metadata_read+=b.len() as u64;Ok(b)
    }
    pub fn write(&mut self,b:&[u8])->Result<Id>{
        if b.len()>crate::MAX_OBJECT{return Err("object-cap".into());}
        let id=io::digest(b);let dest=self.base.join("objects").join(&id[..2]).join(&id);
        if dest.exists(){self.read(&id)?;io::sync_dir(dest.parent().unwrap(),self.metrics)?;return Ok(id);}
        io::mkdir(dest.parent().unwrap(),self.metrics)?;let stage=dest.with_file_name(io::unique("object-stage"));
        io::write_new(&stage,b,self.metrics)?;io::install_original(&stage,&dest,&id,self.metrics)?;io::sync_dir(dest.parent().unwrap(),self.metrics)?;
        self.metrics.bytes_metadata_written+=b.len() as u64;io::checkpoint("cow-object-installed")?;Ok(id)
    }
}
#[derive(Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Descriptor{pub format:String,pub root:Option<Id>,pub consumer:String,pub budget:u64,pub count:usize}
fn objects<'a>(root:&Path,m:&'a mut Metrics)->Objects<'a>{Objects{base:root.to_path_buf(),metrics:m}}
fn encode(d:&Doc,o:&mut Objects)->Result<Vec<u8>>{
    let b=serde_json::to_vec(d)?;
    if b.len()>4096{let id=o.write(&b)?;Ok(format!("@{id}").into_bytes())}else{Ok(b)}
}
fn decode(b:&[u8],o:&mut Objects)->Result<Doc>{
    let owned;if b.first()==Some(&b'@'){owned=o.read(std::str::from_utf8(&b[1..])?)?;serde_json::from_slice(&owned).map_err(Into::into)}else{serde_json::from_slice(b).map_err(Into::into)}
}
pub fn save(root:&Path,s:&State,before:&State,expected:Option<&Selector>,m:&mut Metrics)->Result<Vec<u8>>{
    let t=Instant::now();eprintln!("progress:metadata-start:{}",s.docs.len());
    if std::env::var("EXP_BACKEND").ok().as_deref()!=Some("cow") {
        let b=io::json(s)?;m.bytes_metadata_written+=b.len() as u64;m.elapsed("metadata-build",t);return Ok(b);
    }
    let mut old=None;
    if let Some(sel)=expected {
        let p=crate::publication(root,sel,m)?;
        let b=io::checked(&root.join(&p.candidate.state),&p.candidate.state_digest,m)?;
        let d:Descriptor=serde_json::from_slice(&b)?;old=d.root;
    }
    let mut o=objects(root,m);
    let mut index=if expected.is_none(){
        let mut pairs=vec![];for (id,d) in &s.docs{pairs.push((id.clone(),encode(d,&mut o)?));}DiskIndex::bulk(&mut o,pairs)?
    }else{
        let mut index=DiskIndex::open(&mut o,old)?;let mut changes=vec![];
        for (id,d) in &s.docs{if before.docs.get(id).map(serde_json::to_vec).transpose()?.as_deref()!=Some(serde_json::to_vec(d)?.as_slice()){changes.push((id.clone(),encode(d,&mut o)?));}}
        for id in before.docs.keys(){if !s.docs.contains_key(id){changes.push((id.clone(),vec![]));}}
        for batch in changes.chunks(256){index.put_batch(&mut o,batch.to_vec())?;}index
    };
    let id=index.persist(&mut o)?;io::checkpoint("cow-root-prepared")?;
    let b=io::json(&Descriptor{format:"cow-documents-v1".into(),root:id,consumer:s.consumer.clone(),budget:s.budget,count:s.docs.len()})?;
    m.bytes_metadata_written+=b.len() as u64;m.elapsed("metadata-build",t);eprintln!("progress:metadata-prepared:{}",s.docs.len());Ok(b)
}
pub fn load(root:&Path,c:&Candidate,m:&mut Metrics)->Result<State>{
    let b=io::checked(&root.join(&c.state),&c.state_digest,m)?;m.bytes_metadata_read+=b.len() as u64;
    if let Ok(d)=serde_json::from_slice::<Descriptor>(&b){
        if d.format!="cow-documents-v1"{return Err("integrity:descriptor-version".into());}
        let mut o=objects(root,m);let mut idx=DiskIndex::open(&mut o,d.root)?;let mut docs=BTreeMap::new();
        for (id,b) in idx.entries(&mut o)?{if !b.is_empty(){let doc=decode(&b,&mut o)?;if doc.id!=id{return Err("integrity:cow-doc-key".into());}docs.insert(id,doc);}}
        if docs.len()!=d.count{return Err("integrity:cow-count".into());}Ok(State{consumer:d.consumer,budget:d.budget,docs})
    }else{Ok(serde_json::from_slice(&b)?)}
}
pub fn point(root:&Path,c:&Candidate,key:&str,m:&mut Metrics)->Result<Option<Doc>>{
    let b=io::checked(&root.join(&c.state),&c.state_digest,m)?;m.bytes_metadata_read+=b.len() as u64;
    if let Ok(d)=serde_json::from_slice::<Descriptor>(&b){
        let mut o=objects(root,m);let mut idx=DiskIndex::open(&mut o,d.root)?;
        let value=idx.get(&mut o,key)?;value.filter(|b|!b.is_empty()).map(|b|decode(&b,&mut o)).transpose()
    }else{let s:State=serde_json::from_slice(&b)?;crate::model::validate(&s)?;Ok(s.docs.get(key).cloned())}
}
/// Full immutable metadata reachability for reclamation. Audit the actual tree
/// first, then parse child/value references; never infer liveness from filenames.
pub fn reachable(root:&Path,c:&Candidate,m:&mut Metrics)->Result<Vec<String>>{
    let b=io::checked(&root.join(&c.state),&c.state_digest,m)?;
    let Ok(d)=serde_json::from_slice::<Descriptor>(&b) else{return Ok(vec![])};
    let mut o=objects(root,m);let mut idx=DiskIndex::open(&mut o,d.root.clone())?;idx.audit(&mut o)?;
    fn visit(o:&mut Objects,id:&str,out:&mut Vec<String>)->Result<()>{
        if out.iter().any(|s|s==id){return Ok(());}let b=o.read(id)?;out.push(id.into());let v:serde_json::Value=serde_json::from_slice(&b)?;
        if v["kind"]=="branch"{for c in v["children"].as_array().ok_or("integrity:children")?{visit(o,c["id"].as_str().ok_or("integrity:child")?,out)?;}}
        if v["kind"]=="leaf"{let pairs:Vec<(String,Vec<u8>)>=serde_json::from_value(v["pairs"].clone())?;for (_,b) in pairs{if b.first()==Some(&b'@'){let id=std::str::from_utf8(&b[1..])?;o.read(id)?;out.push(id.into());}}}Ok(())
    }
    let mut out=vec![];if let Some(id)=d.root{visit(&mut o,&id,&mut out)?;}Ok(out)
}
