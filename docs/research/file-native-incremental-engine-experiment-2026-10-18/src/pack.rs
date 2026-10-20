//! Private bounded immutable record containers. No semantic membership authority.
use crate::io::{self, Metrics, Result};
use serde::{Deserialize, Serialize};
use std::{fs::File, io::{Read, Seek, SeekFrom}, path::Path};

pub(crate) const MAX_PACK: usize = 64 * 1024 * 1024;
pub(crate) const MAX_INDEX: usize = 16 * 1024 * 1024;
pub(crate) const MAX_RAW: usize = 8 * 1024 * 1024;
pub(crate) const MAX_RECORDS: usize = 65536;
pub(crate) const MAX_CACHE: usize = 64 * 1024 * 1024;
const MAGIC: &[u8;8] = b"FNPIDX01";
const HEADER: usize = 8+32+8+1+4;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub(crate) struct Descriptor {
    pub pack: String, pub index: String, pub pack_bytes: u64, pub index_bytes: u64,
    pub records: u32, pub codec: u8, pub role: String, pub first: String, pub last: String,
    #[serde(default,skip_serializing_if="Option::is_none")]
    pub instance: Option<String>,
    #[serde(default,skip_serializing_if="is_zero")]
    pub segment:u64,
    #[serde(default,skip_serializing_if="Option::is_none")]
    pub page:Option<crate::paged::Root>,
}
fn is_zero(n:&u64)->bool{*n==0}
impl Descriptor {
    pub fn path(&self, extension:&str)->Result<String> {
        let hash=match extension{"pack"=>&self.pack,"idx"=>&self.index,_=>return Err("integrity:container-extension".into())};
        if !crate::objects::valid_hash(hash){return Err("integrity:container-hash".into());}
        match &self.instance {
            Some(id) if crate::model::safe_id(id)=>Ok(format!("packs/{id}/{hash}.{extension}")),
            None=>Ok(format!("packs/{hash}.{extension}")),
            _=>Err("integrity:container-instance".into()),
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Key(pub String);
impl Key {
    pub fn new(path: &str) -> Result<Self> {
        let key=Self(path.into());
        if path.len()>512 || path.contains('\\') || path.contains(':') || path.split('/').any(|s|s.is_empty()||s=="."||s=="..") || key.role().is_none() {
            return Err("integrity:packed-key-type".into());
        }
        Ok(key)
    }
    pub fn role(&self) -> Option<&'static str> {
        let p:Vec<_>=self.0.split('/').collect();
        let json_id=|s:&str|s.strip_suffix(".json").is_some_and(crate::model::safe_id);
        match p.as_slice() {
            ["publications",id] if json_id(id)=>Some("publication"),
            ["results",id] if id.strip_suffix(".json").is_some_and(crate::objects::valid_hash)=>Some("result"),
            ["attempt-hints",id] if json_id(id)=>Some("hint"),
            ["attempts",attempt,"candidate.json"] if crate::model::safe_id(attempt)=>Some("candidate"),
            ["attempts",attempt,"allocations",id] if crate::model::safe_id(attempt)&&json_id(id)=>Some("allocation"),
            _=>None,
        }
    }
}
fn hash_bytes(hash:&str)->Result<Vec<u8>> {
    if !crate::objects::valid_hash(hash){return Err("integrity:pack-hash".into());}
    (0..64).step_by(2).map(|i|Ok(u8::from_str_radix(&hash[i..i+2],16)?)).collect()
}
fn hex(bytes:&[u8])->String{const H:&[u8]=b"0123456789abcdef";let mut out=String::with_capacity(bytes.len()*2);for b in bytes{out.push(H[(b>>4) as usize] as char);out.push(H[(b&15) as usize] as char);}out}
pub(crate) fn native(path:&Path,limit:usize,m:&mut Metrics)->Result<Vec<u8>> {
    let mut f=File::open(path).map_err(|e|format!("integrity:pack-required:{}:{e}",path.display()))?;
    m.pack_metadata_stats+=1;let length=f.metadata()?.len();if length>limit as u64{return Err("integrity:pack-file-bound".into());}
    let mut b=vec![0;length as usize];f.read_exact(&mut b)?;
    if f.read(&mut [0])?!=0{return Err("integrity:pack-file-changed".into());}
    m.opens+=1;m.bytes_read+=b.len() as u64;m.pack_storage_bytes+=b.len() as u64;Ok(b)
}
pub(crate) struct Index { bytes:Vec<u8>, offsets:Vec<usize>, pub descriptor:Descriptor, paged:Option<crate::paged::Reader> }
#[derive(Clone)]
pub(crate) struct Entry { pub key:Key,pub offset:u64,pub encoded:usize,pub raw:usize,pub hash:String }
impl Index {
    pub fn allocation_bound(d:&Descriptor)->Result<usize>{
        if d.page.is_some(){return Ok(crate::paged::MEMORY_BOUND);}
        let bytes=usize::try_from(d.index_bytes)?;let offsets=(d.records as usize).checked_mul(std::mem::size_of::<usize>()).ok_or("integrity:index-memory-overflow")?;
        bytes.checked_add(offsets).and_then(|n|n.checked_add(4096)).ok_or_else(||"integrity:index-memory-overflow".into())
    }
    pub fn load(root:&Path,d:&Descriptor,m:&mut Metrics)->Result<Self> {
        if !crate::objects::valid_hash(&d.pack)||!crate::objects::valid_hash(&d.index)||d.pack_bytes>MAX_PACK as u64||d.index_bytes>MAX_INDEX as u64||d.records==0||d.records as usize>MAX_RECORDS||d.codec>1 {
            return Err("integrity:pack-descriptor-bound".into());
        }
        if d.page.is_some(){return Ok(Self{bytes:vec![],offsets:vec![],descriptor:d.clone(),paged:Some(crate::paged::Reader::open(root,d,m)?)});}
        // Reject a changed file before allocating beyond the incoming charge.
        let b=native(&root.join(d.path("idx")?),usize::try_from(d.index_bytes)?,m)?;
        m.pack_legacy_index_bytes+=b.len() as u64;
        if b.len()!=d.index_bytes as usize||io::digest(&b)!=d.index||b.len()<HEADER||&b[..8]!=MAGIC||hex(&b[8..40])!=d.pack||u64::from_le_bytes(b[40..48].try_into()?)!=d.pack_bytes||b[48]!=d.codec||u32::from_le_bytes(b[49..53].try_into()?)!=d.records {
            return Err("integrity:index-pack-binding".into());
        }
        if root.join(d.path("pack")?).metadata().map_err(|e|format!("integrity:pack-missing:{e}"))?.len()!=d.pack_bytes{return Err("integrity:pack-size-binding".into());}
        let mut index=Self{bytes:b,offsets:Vec::with_capacity(d.records as usize),descriptor:d.clone(),paged:None};
        let mut position=HEADER;let mut last=String::new();let mut end=0u64;
        for _ in 0..d.records {
            index.offsets.push(position);let (e,next)=index.at(position)?;
            if e.key.0<=last||e.key.role()!=Some(d.role.as_str())||e.offset!=end||e.raw>MAX_RAW||e.encoded>MAX_RAW+65536||e.offset.checked_add(e.encoded as u64).is_none_or(|n|n>d.pack_bytes) {
                return Err("integrity:index-order-offset-length".into());
            }
            if d.codec==0&&e.raw!=e.encoded{return Err("integrity:raw-codec-length".into());}
            last=e.key.0;end=e.offset+e.encoded as u64;position=next;
        }
        if position!=index.bytes.len()||end!=d.pack_bytes||index.at(index.offsets[0])?.0.key.0!=d.first||last!=d.last||index.memory()>MAX_CACHE{return Err("integrity:index-coverage".into());}
        m.pack_index_loads+=1;Ok(index)
    }
    fn at(&self,p:usize)->Result<(Entry,usize)> {
        let n=u16::from_le_bytes(self.bytes.get(p..p+2).ok_or("integrity:index-truncated")?.try_into()?) as usize;
        if n>512{return Err("integrity:index-key-bound".into());}
        let start=p.checked_add(2).ok_or("integrity:index-overflow")?;let tail=start.checked_add(n).ok_or("integrity:index-overflow")?;
        let key=Key::new(std::str::from_utf8(self.bytes.get(start..tail).ok_or("integrity:index-truncated")?)?)?;
        let bytes=self.bytes.get(tail..tail+48).ok_or("integrity:index-truncated")?;
        Ok((Entry{key,offset:u64::from_le_bytes(bytes[..8].try_into()?),encoded:u32::from_le_bytes(bytes[8..12].try_into()?) as usize,raw:u32::from_le_bytes(bytes[12..16].try_into()?) as usize,hash:hex(&bytes[16..48])},tail+48))
    }
    pub fn entry(&self,n:usize,m:&mut Metrics)->Result<Entry>{if let Some(p)=&self.paged{return p.ordinal(n,m);}Ok(self.at(*self.offsets.get(n).ok_or("integrity:index-ordinal")?)?.0)}
    pub fn find(&self,key:&Key,m:&mut Metrics)->Result<Option<Entry>> {
        if let Some(p)=&self.paged{return p.find(key,m);}
        let mut lo=0;let mut hi=self.offsets.len();
        while lo<hi {let mid=(lo+hi)/2;let e=self.entry(mid,m)?;match e.key.cmp(key){std::cmp::Ordering::Less=>lo=mid+1,std::cmp::Ordering::Greater=>hi=mid,std::cmp::Ordering::Equal=>return Ok(Some(e))}}
        Ok(None)
    }
    pub fn memory(&self)->usize{if self.paged.is_some(){return crate::paged::MEMORY_BOUND;}self.bytes.capacity()+self.offsets.capacity()*std::mem::size_of::<usize>()+std::mem::size_of::<Self>()+1024+self.descriptor.pack.capacity()+self.descriptor.index.capacity()+self.descriptor.role.capacity()+self.descriptor.first.capacity()+self.descriptor.last.capacity()+self.descriptor.instance.as_ref().map_or(0,String::capacity)}
    pub fn audit(&self,m:&mut Metrics)->Result<()>{if let Some(p)=&self.paged{p.audit(m)?;}Ok(())}
    pub fn read(&self,root:&Path,e:&Entry,limit:usize,m:&mut Metrics)->Result<Vec<u8>> {
        read_record(root,&self.descriptor,e,limit,m)
    }
}
pub(crate) fn read_record(root:&Path,d:&Descriptor,e:&Entry,limit:usize,m:&mut Metrics)->Result<Vec<u8>> {
        if e.raw>limit||e.raw>MAX_RAW{return Err("integrity:packed-read-bound".into());}
        io::checkpoint("pack-record-located")?;
        if e.encoded>MAX_RAW+65536||e.offset.checked_add(e.encoded as u64).is_none_or(|n|n>d.pack_bytes){return Err("integrity:pack-record-extent".into());}
        let mut f=File::open(root.join(d.path("pack")?)).map_err(|e|format!("integrity:pack-required:{e}"))?;
        m.pack_metadata_stats+=1;
        if f.metadata()?.len()!=d.pack_bytes{return Err("integrity:pack-size-binding".into());}
        f.seek(SeekFrom::Start(e.offset))?;let mut encoded=vec![0;e.encoded];f.read_exact(&mut encoded)?;
        m.opens+=1;m.bytes_read+=e.encoded as u64;m.pack_storage_bytes+=e.encoded as u64;m.pack_records_read+=1;
        let raw=decode(&encoded,e.raw,d.codec)?;
        if raw.len()!=e.raw||io::digest(&raw)!=e.hash{return Err("integrity:pack-raw-identity".into());}
        m.pack_decoded_bytes+=raw.len() as u64;m.record_bytes(&root.join(&e.key.0),raw.len() as u64,false);
        Ok(raw)
}
pub(crate) fn decode(encoded:&[u8],raw:usize,codec:u8)->Result<Vec<u8>> {
    if raw>MAX_RAW||encoded.len()>MAX_RAW+65536{return Err("integrity:codec-bound".into());}
    match codec {
        0=>Ok(encoded.to_vec()),
        1=>{
            // One ordinary frame, no skippable/dictionary/concatenated frames.
            if encoded.get(..4)!=Some(&[0x28,0xb5,0x2f,0xfd])||zstd::zstd_safe::find_frame_compressed_size(encoded).map_err(|e|format!("integrity:zstd-frame:{e}"))?!=encoded.len(){return Err("integrity:zstd-single-frame".into());}
            // Streaming API enforces WindowLogMax on its internal history buffer;
            // one-shot APIs use caller output as history and do not provide the
            // same independent window rejection. Never allocate from frame size.
            let mut decoder=zstd::stream::read::Decoder::new(std::io::Cursor::new(encoded))?;
            decoder.window_log_max(23)?;
            let mut output=vec![0;raw];decoder.read_exact(&mut output)?;
            if decoder.read(&mut [0])?!=0{return Err("integrity:zstd-raw-length".into());}Ok(output)
        },_=>Err("integrity:codec-version".into())
    }
}
pub(crate) struct Builder { pub pack:Vec<u8>, entries:Vec<Entry>,raw_bytes:usize,index_bytes:usize,encoded_bound:usize,role:String,codec:u8 }
impl Builder {
    pub fn new(role:&str,codec:u8)->Self{Self{pack:vec![],entries:vec![],raw_bytes:0,index_bytes:HEADER,encoded_bound:0,role:role.into(),codec}}
    pub fn is_empty(&self)->bool{self.entries.is_empty()}
    pub fn fits(&self,key:&Key,raw:usize)->bool {
        self.entries.len()<MAX_RECORDS && self.raw_bytes+raw<=60*1024*1024 && crate::paged::size_bound(self.entries.len()+1,self.index_bytes-HEADER+key.0.len()+50).is_ok_and(|n|n<=MAX_INDEX) && self.encoded_bound+zstd::zstd_safe::compress_bound(raw)<=MAX_PACK
    }
    pub fn push(&mut self,key:Key,raw:&[u8])->Result<()> {
        if raw.len()>MAX_RAW||key.role()!=Some(self.role.as_str())||!self.fits(&key,raw.len())||self.entries.last().is_some_and(|e|e.key>=key){return Err("refused:pack-builder-bound-order".into());}
        let encoded=match self.codec{0=>raw.to_vec(),1=>zstd::bulk::compress(raw,3)?,_=>return Err("integrity:codec-version".into())};
        if self.pack.len()+encoded.len()>MAX_PACK{return Err("refused:encoded-pack-bound".into());}
        self.index_bytes+=key.0.len()+50;
        self.encoded_bound+=zstd::zstd_safe::compress_bound(raw.len());
        self.entries.push(Entry{key,offset:self.pack.len() as u64,encoded:encoded.len(),raw:raw.len(),hash:io::digest(raw)});
        self.pack.extend(encoded);self.raw_bytes+=raw.len();Ok(())
    }
    pub fn finish(self)->Result<(Descriptor,Vec<u8>,Vec<u8>)> {
        if self.entries.is_empty(){return Err("refused:empty-pack".into());}
        let hash=io::digest(&self.pack);
        let mut d=Descriptor{page:None,instance:None,segment:0,pack:hash,index:String::new(),pack_bytes:self.pack.len() as u64,index_bytes:0,records:self.entries.len() as u32,codec:self.codec,role:self.role,first:self.entries[0].key.0.clone(),last:self.entries.last().unwrap().key.0.clone()};
        let (bytes,page)=crate::paged::build(&d,&self.entries)?;d.index=io::digest(&bytes);d.index_bytes=bytes.len() as u64;d.page=Some(page);Ok((d,self.pack,bytes))
    }
    #[cfg(test)]
    pub fn finish_legacy(self)->Result<(Descriptor,Vec<u8>,Vec<u8>)> {
        if self.entries.is_empty(){return Err("refused:empty-pack".into());}
        let hash=io::digest(&self.pack);let mut index=MAGIC.to_vec();index.extend(hash_bytes(&hash)?);index.extend((self.pack.len() as u64).to_le_bytes());index.push(self.codec);index.extend((self.entries.len() as u32).to_le_bytes());
        for e in &self.entries {index.extend((e.key.0.len() as u16).to_le_bytes());index.extend(e.key.0.as_bytes());index.extend(e.offset.to_le_bytes());index.extend((e.encoded as u32).to_le_bytes());index.extend((e.raw as u32).to_le_bytes());index.extend(hash_bytes(&e.hash)?);}
        if index.len()>MAX_INDEX{return Err("refused:index-size".into());}
        let d=Descriptor{page:None,instance:None,segment:0,pack:hash,index:io::digest(&index),pack_bytes:self.pack.len() as u64,index_bytes:index.len() as u64,records:self.entries.len() as u32,codec:self.codec,role:self.role,first:self.entries[0].key.0.clone(),last:self.entries.last().unwrap().key.0.clone()};
        Ok((d,self.pack,index))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn decoder_never_uses_advertised_size(){
        let frame=zstd::bulk::compress(&vec![7;1024*1024],3).unwrap();
        assert!(decode(&frame,1,1).is_err());assert!(decode(&frame,MAX_RAW+1,1).is_err());
        let mut extra=frame.clone();extra.extend(&frame);assert!(decode(&extra,2*1024*1024,1).is_err());
        assert_eq!(decode(&frame,1024*1024,1).unwrap(),vec![7;1024*1024]);
        let mut large_window=zstd::stream::encode_all(std::io::Cursor::new(b"small payload"),3).unwrap();
        assert_eq!(large_window[4]&32,0);large_window[5]=0xf8;
        assert!(decode(&large_window,13,1).is_err());
    }
    #[test] fn typed_keys_exclude_controls_and_active_payloads(){
        for key in ["STORE.json","attempts/a/packet/MANIFEST.json","originals/a","attempts/../candidate.json","/results/a.json"]{assert!(Key::new(key).is_err());}
        assert_eq!(Key::new("attempts/a/candidate.json").unwrap().role(),Some("candidate"));
    }
}
