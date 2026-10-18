use crate::{io::{self,Metrics,Result},disk::DiskIndex};
use serde::{Serialize,de::DeserializeOwned};
use std::{path::{Path,PathBuf},collections::{BTreeMap,BTreeSet}};
pub type Id=String;
pub struct Objects<'a>{pub base:PathBuf,pub metrics:&'a mut Metrics}
pub fn valid_hash(id:&str)->bool{id.len()==64&&id.bytes().all(|b|b.is_ascii_digit()||(b'a'..=b'f').contains(&b))}
impl Objects<'_>{
    pub fn read(&mut self,id:&str)->Result<Vec<u8>>{
        if !valid_hash(id){return Err("integrity:object-id".into());}
        let b=io::read_bounded(&self.base.join("objects").join(&id[..2]).join(id),crate::MAX_OBJECT,self.metrics)?;
        if io::digest(&b)!=id{return Err("integrity:object-digest".into());}
        self.metrics.page_reads+=1;self.metrics.unique_pages.insert(id.into());
        self.metrics.bytes_metadata_read+=b.len() as u64;Ok(b)
    }
    pub fn write(&mut self,b:&[u8])->Result<Id>{
        if b.len()>crate::MAX_OBJECT{return Err("object-cap".into());}
        let id=io::digest(b);let dest=self.base.join("objects").join(&id[..2]).join(&id);
        if dest.exists(){self.read(&id)?;return Ok(id);}
        io::mkdir(dest.parent().unwrap(),self.metrics)?;
        let stage=dest.with_file_name(io::unique("object-stage"));
        io::write_new(&stage,b,self.metrics)?;io::install_original(&stage,&dest,&id,self.metrics)?;
        io::sync_dir(dest.parent().unwrap(),self.metrics)?;
        self.metrics.bytes_metadata_written+=b.len() as u64;
        io::checkpoint("cow-object-installed")?;Ok(id)
    }
}
pub fn encode<T:Serialize>(v:&T,o:&mut Objects)->Result<Vec<u8>>{
    let b=serde_json::to_vec(v)?;
    if b.len()>4096{Ok(format!("@{}",o.write(&b)?).into_bytes())}else{Ok(b)}
}
pub fn decode<T:DeserializeOwned>(b:&[u8],o:&mut Objects)->Result<T>{
    if b.first()==Some(&b'@'){
        let id=std::str::from_utf8(&b[1..])?;
        Ok(serde_json::from_slice(&o.read(id)?)?)
    }else{Ok(serde_json::from_slice(b)?)}
}
pub fn object_store<'a>(root:&Path,m:&'a mut Metrics)->Objects<'a>{Objects{base:root.into(),metrics:m}}
pub fn point<T:DeserializeOwned>(o:&mut Objects,root:&Option<Id>,key:&str)->Result<Option<T>>{
    DiskIndex::open(o,root.clone())?.get(o,key)?.filter(|b|!b.is_empty()).map(|b|decode(&b,o)).transpose()
}
pub fn range<T:DeserializeOwned>(o:&mut Objects,root:&Option<Id>,prefix:&str)->Result<Vec<(String,T)>>{
    DiskIndex::open(o,root.clone())?.range(o,prefix,&format!("{prefix}\u{10ffff}"))?
        .into_iter().filter(|(_,b)|!b.is_empty()).map(|(k,b)|Ok((k,decode(&b,o)?))).collect()
}
pub fn update(o:&mut Objects,root:&Option<Id>,changes:BTreeMap<String,Vec<u8>>)->Result<Option<Id>>{
    let mut index=DiskIndex::open(o,root.clone())?;
    let all:Vec<_>=changes.into_iter().collect();o.metrics.index_updates+=all.len() as u64;
    for chunk in all.chunks(256){index.put_batch(o,chunk.to_vec())?;}
    index.persist(o)
}
pub fn bulk(o:&mut Objects,pairs:BTreeMap<String,Vec<u8>>)->Result<Option<Id>>{
    DiskIndex::bulk(o,pairs.into_iter().collect())?.persist(o)
}
pub fn entries(o:&mut Objects,root:&Option<Id>)->Result<BTreeMap<String,Vec<u8>>>{
    Ok(DiskIndex::open(o,root.clone())?.entries(o)?.into_iter().filter(|(_,b)|!b.is_empty()).collect())
}
pub fn reachable(o:&mut Objects,root:&Option<Id>)->Result<BTreeSet<Id>>{
    Ok(DiskIndex::open(o,root.clone())?.reachable(o)?.into_iter().collect())
}
