//! Private FNPIDX02: authenticated bounded pages in one immutable index file.
//! Point reads verify their route; audit verifies the complete file/tree.
use crate::{io::{self,Metrics,Result},pack::{self,Descriptor,Entry,Key}};
use serde::{Serialize,Deserialize};
use sha2::{Digest,Sha256};
use std::{cell::RefCell,collections::BTreeMap,fs::File,io::{Read,Seek,SeekFrom},path::{Path,PathBuf},rc::Rc,time::Instant};
pub(crate) const PAGE:usize=16384;
pub(crate) const SLOTS:usize=48;
pub(crate) const MEMORY_BOUND:usize=1024*1024;
const MAGIC:&[u8;8]=b"FNPIDX02";
const CACHE_PAGES:usize=12;
#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Eq)]
#[serde(deny_unknown_fields)]
pub(crate) struct Root {pub version:u32,pub hash:String,pub bytes:u32,pub height:u8}
pub(crate) fn validate(d:&Descriptor)->Result<()>{
    let p=d.page.as_ref().ok_or("integrity:paged-discriminator")?;
    if p.version!=2||!crate::objects::valid_hash(&p.hash)||p.height>2||p.bytes==0||p.bytes as usize>PAGE||p.bytes as u64>d.index_bytes||d.index_bytes>pack::MAX_INDEX as u64||d.records==0||d.records as usize>pack::MAX_RECORDS{return Err("integrity:paged-root-bound".into());}
    typed(&d.first)?;typed(&d.last)?;Ok(())
}
fn typed(s:&str)->Result<Key>{let k=Key::new(s)?;if s.len()>225{return Err("integrity:paged-key-bound".into());}Ok(k)}
fn role(s:&str)->Result<u8>{match s{"publication"=>Ok(0),"result"=>Ok(1),"hint"=>Ok(2),"candidate"=>Ok(3),"allocation"=>Ok(4),_=>Err("integrity:paged-role".into())}}
fn hash(s:&str)->Result<Vec<u8>>{if !crate::objects::valid_hash(s){return Err("integrity:paged-hash".into());}(0..64).step_by(2).map(|i|Ok(u8::from_str_radix(&s[i..i+2],16)?)).collect()}
fn hex(b:&[u8])->String{b.iter().map(|n|format!("{n:02x}")).collect()}
fn string(out:&mut Vec<u8>,s:&str)->Result<()>{typed(s)?;out.extend((s.len() as u16).to_le_bytes());out.extend(s.as_bytes());Ok(())}
pub(crate) fn size_bound(n:usize,entries:usize)->Result<usize>{
    if n==0||n>pack::MAX_RECORDS{return Err("refused:paged-record-count".into());}
    let leaves=n.checked_add(SLOTS-1).ok_or("integrity:paged-overflow")?/SLOTS;
    let mid=if leaves>SLOTS{leaves.checked_add(SLOTS-1).ok_or("integrity:paged-overflow")?/SLOTS}else{0};
    let pages=leaves.checked_add(mid).and_then(|v|v.checked_add(usize::from(leaves>1))).ok_or("integrity:paged-overflow")?;
    entries.checked_add(pages.checked_mul(1024).ok_or("integrity:paged-overflow")?).and_then(|v|v.checked_add((pages-1).checked_mul(291)?)).ok_or_else(||"integrity:paged-overflow".into())
}
#[derive(Clone)]
struct Child {first:String,offset:u64,bytes:u32,hash:String,records:u32,start:u64,end:u64}
struct Node {last:String,entries:Vec<Entry>,children:Vec<Child>}
impl Node{fn memory(&self)->usize{std::mem::size_of::<Self>()+self.last.capacity()+self.entries.capacity()*std::mem::size_of::<Entry>()+self.entries.iter().map(|e|e.key.0.capacity()+e.hash.capacity()).sum::<usize>()+self.children.capacity()*std::mem::size_of::<Child>()+self.children.iter().map(|e|e.first.capacity()+e.hash.capacity()).sum::<usize>()}}
struct Cursor<'a>{b:&'a[u8],p:usize}
impl<'a> Cursor<'a>{
    fn take(&mut self,n:usize)->Result<&'a[u8]>{let end=self.p.checked_add(n).ok_or("integrity:paged-overflow")?;let v=self.b.get(self.p..end).ok_or("integrity:paged-truncated")?;self.p=end;Ok(v)}
    fn u8(&mut self)->Result<u8>{Ok(self.take(1)?[0])}
    fn u16(&mut self)->Result<u16>{Ok(u16::from_le_bytes(self.take(2)?.try_into()?))}
    fn u32(&mut self)->Result<u32>{Ok(u32::from_le_bytes(self.take(4)?.try_into()?))}
    fn u64(&mut self)->Result<u64>{Ok(u64::from_le_bytes(self.take(8)?.try_into()?))}
    fn key(&mut self)->Result<String>{let n=self.u16()? as usize;if n==0||n>225{return Err("integrity:paged-key-bound".into());}let s=std::str::from_utf8(self.take(n)?)?;typed(s)?;Ok(s.into())}
}
fn parse(bytes:&[u8],d:&Descriptor,expected:&Child,height:u8,upper:Option<&str>,last:Option<&str>)->Result<Node>{
    if bytes.len()>PAGE||bytes.is_empty(){return Err("integrity:paged-page-bound".into());}
    let mut c=Cursor{b:bytes,p:0};if c.take(8)?!=MAGIC{return Err("integrity:paged-magic".into());}
    let h=c.u8()?;let r=c.u8()?;let codec=c.u8()?;let reserved=c.u8()?;let records=c.u32()?;let n=c.u16()? as usize;
    let start=c.u64()?;let end=c.u64()?;let pack_hash=hex(c.take(32)?);let pack_bytes=c.u64()?;let first=c.key()?;let final_key=c.key()?;
    if h!=height||h>2||r!=role(&d.role)?||codec!=d.codec||reserved!=0||records!=expected.records||n==0||n>SLOTS||start!=expected.start||end!=expected.end||start>end||end>d.pack_bytes||pack_hash!=d.pack||pack_bytes!=d.pack_bytes||first!=expected.first||first>final_key||upper.is_some_and(|u|final_key.as_str()>=u)||last.is_some_and(|l|final_key!=l)||typed(&first)?.role()!=Some(d.role.as_str())||typed(&final_key)?.role()!=Some(d.role.as_str()){
        return Err("integrity:paged-parent-binding".into());
    }
    let mut entries=Vec::with_capacity(if h==0{n}else{0});let mut children=Vec::with_capacity(if h>0{n}else{0});let mut previous=String::new();let mut position=start;let mut total=0u32;
    for _ in 0..n{
        let key=c.key()?;
        if key<=previous||key<first||key>final_key||typed(&key)?.role()!=Some(d.role.as_str()){return Err("integrity:paged-order".into());}
        if h==0{
            let offset=c.u64()?;let encoded=c.u32()? as usize;let raw=c.u32()? as usize;let raw_hash=hex(c.take(32)?);
            if offset!=position||raw>pack::MAX_RAW||encoded>pack::MAX_RAW+65536||d.codec==0&&raw!=encoded{return Err("integrity:paged-record-bound".into());}
            position=offset.checked_add(encoded as u64).ok_or("integrity:paged-overflow")?;
            if position>end{return Err("integrity:paged-pack-coverage".into());}
            entries.push(Entry{key:Key(key.clone()),offset,encoded,raw,hash:raw_hash});total=total.checked_add(1).ok_or("integrity:paged-overflow")?;
        }else{
            let offset=c.u64()?;let length=c.u32()?;let child_hash=hex(c.take(32)?);let count=c.u32()?;let lo=c.u64()?;let hi=c.u64()?;
            if length==0||length as usize>PAGE||offset.checked_add(length as u64).is_none_or(|v|v>d.index_bytes)||count==0||count as usize>SLOTS.pow(h as u32)||lo!=position||hi<lo||hi>end{return Err("integrity:paged-child-bound".into());}
            position=hi;total=total.checked_add(count).ok_or("integrity:paged-overflow")?;children.push(Child{first:key.clone(),offset,bytes:length,hash:child_hash,records:count,start:lo,end:hi});
        }
        previous=key;
    }
    if c.p!=bytes.len()||total!=records||position!=end||if h==0{entries.first().unwrap().key.0!=first||previous!=final_key}else{children.first().unwrap().first!=first}{return Err("integrity:paged-coverage".into());}
    Ok(Node{last:final_key,entries,children})
}
type Pages=BTreeMap<(u64,u32,String),Rc<Vec<u8>>>;
pub(crate) struct Reader{root:PathBuf,d:Descriptor,pages:RefCell<Pages>}
impl Reader{
    pub fn open(root:&Path,d:&Descriptor,m:&mut Metrics)->Result<Self>{validate(d)?;let reader=Self{root:root.into(),d:d.clone(),pages:RefCell::new(BTreeMap::new())};let (e,h)=reader.root();let b=reader.page(&e,m)?;let t=Instant::now();let node=parse(&b,d,&e,h,None,Some(&d.last))?;m.elapsed("pack-page-parse",t);m.pack_pages_parsed+=1;m.pack_page_parse_peak_bytes=m.pack_page_parse_peak_bytes.max(node.memory() as u64);m.pack_index_loads+=1;Ok(reader)}
    fn root(&self)->(Child,u8){let r=self.d.page.as_ref().unwrap();(Child{first:self.d.first.clone(),offset:0,bytes:r.bytes,hash:r.hash.clone(),records:self.d.records,start:0,end:self.d.pack_bytes},r.height)}
    fn page(&self,e:&Child,m:&mut Metrics)->Result<Rc<Vec<u8>>>{
        let key=(e.offset,e.bytes,e.hash.clone());if let Some(b)=self.pages.borrow().get(&key){m.pack_cache_hits+=1;return Ok(b.clone());}
        m.cleanup_step()?;
        {let mut cache=self.pages.borrow_mut();if cache.len()>=CACHE_PAGES{let old=cache.iter().find(|(_,b)|Rc::strong_count(b)==1).map(|(k,_)|k.clone()).ok_or("integrity:paged-borrow-budget")?;cache.remove(&old);}}
        if e.bytes==0||e.bytes as usize>PAGE||e.offset.checked_add(e.bytes as u64).is_none_or(|n|n>self.d.index_bytes){return Err("integrity:paged-read-extent".into());}
        let started=Instant::now();let mut f=File::open(self.root.join(self.d.path("idx")?))?;m.pack_metadata_stats+=1;
        if f.metadata()?.len()!=self.d.index_bytes{return Err("integrity:paged-file-size".into());}f.seek(SeekFrom::Start(e.offset))?;
        let mut b=vec![0;e.bytes as usize];f.read_exact(&mut b)?;m.opens+=1;m.bytes_read+=b.len() as u64;m.pack_storage_bytes+=b.len() as u64;m.pack_index_metadata_bytes+=b.len() as u64;m.pack_paged_index_metadata_bytes+=b.len() as u64;m.pack_page_reads+=1;m.elapsed("pack-page-read",started);
        let resident=self.pages.borrow().iter().map(|((_,_,h),b)|h.capacity()+b.capacity()+192).sum::<usize>()+b.capacity()+e.hash.capacity()+192;
        if resident>MEMORY_BOUND/2{return Err("integrity:paged-cache-allocation".into());}m.pack_page_resident_peak_bytes=m.pack_page_resident_peak_bytes.max(resident as u64);
        let started=Instant::now();if io::digest(&b)!=e.hash{return Err("integrity:paged-page-hash".into());}m.pack_page_hash_bytes+=b.len() as u64;m.elapsed("pack-page-hash",started);
        let b=Rc::new(b);self.pages.borrow_mut().insert(key,b.clone());Ok(b)
    }
    fn seek(&self,key:Option<&Key>,ordinal:Option<usize>,m:&mut Metrics)->Result<Option<Entry>>{
        if let Some(k)=key{typed(&k.0)?;}if ordinal.is_some_and(|n|n>=self.d.records as usize){return Err("integrity:index-ordinal".into());}
        let (mut expected,mut height)=self.root();let mut upper:Option<String>=None;let mut last=Some(self.d.last.clone());let mut ordinal=ordinal.unwrap_or(0);
        loop{
            let b=self.page(&expected,m)?;let t=Instant::now();let node=parse(&b,&self.d,&expected,height,upper.as_deref(),last.as_deref())?;m.elapsed("pack-page-parse",t);m.pack_pages_parsed+=1;
            m.pack_page_parse_peak_bytes=m.pack_page_parse_peak_bytes.max(node.memory() as u64);
            if height==0{return Ok(if let Some(k)=key{node.entries.into_iter().find(|e|e.key==*k)}else{node.entries.into_iter().nth(ordinal)});}
            let n=if let Some(k)=key{match node.children.iter().rposition(|c|c.first<=k.0){Some(n)=>n,None=>return Ok(None)}}else{let mut chosen=None;for (n,c) in node.children.iter().enumerate(){if ordinal<c.records as usize{chosen=Some(n);break;}ordinal-=c.records as usize;}chosen.ok_or("integrity:paged-ordinal-coverage")?};
            if let Some(next)=node.children.get(n+1){upper=Some(next.first.clone());last=None;}else{last=Some(node.last);}
            expected=node.children[n].clone();height-=1;
        }
    }
    pub fn find(&self,key:&Key,m:&mut Metrics)->Result<Option<Entry>>{self.seek(Some(key),None,m)}
    pub fn ordinal(&self,n:usize,m:&mut Metrics)->Result<Entry>{self.seek(None,Some(n),m)?.ok_or_else(||"integrity:paged-ordinal".into())}
    pub fn audit(&self,m:&mut Metrics)->Result<()>{
        self.pages.borrow_mut().clear();verify_file(&self.root.join(self.d.path("idx")?),self.d.index_bytes,&self.d.index,m)?;
        let (e,h)=self.root();let mut next=0u64;self.walk(&e,h,None,Some(&self.d.last),&mut next,m)?;
        if next!=self.d.index_bytes{return Err("integrity:paged-file-coverage".into());}Ok(())
    }
    fn walk(&self,e:&Child,h:u8,upper:Option<&str>,last:Option<&str>,next:&mut u64,m:&mut Metrics)->Result<()>{
        if e.offset!=*next{return Err("integrity:paged-alias-gap".into());}*next=next.checked_add(e.bytes as u64).ok_or("integrity:paged-overflow")?;
        let b=self.page(e,m)?;let t=Instant::now();let node=parse(&b,&self.d,e,h,upper,last)?;m.elapsed("pack-page-parse",t);m.pack_pages_parsed+=1;m.pack_page_parse_peak_bytes=m.pack_page_parse_peak_bytes.max(node.memory() as u64);
        for (n,child) in node.children.iter().enumerate(){self.walk(child,h-1,node.children.get(n+1).map(|c|c.first.as_str()).or(upper),if n+1==node.children.len(){Some(&node.last)}else{None},next,m)?;}
        Ok(())
    }
}
pub(crate) fn verify_file(path:&Path,bytes:u64,hash:&str,m:&mut Metrics)->Result<()>{
    let mut f=File::open(path)?;m.pack_metadata_stats+=1;if f.metadata()?.len()!=bytes{return Err("integrity:audit-file-size".into());}
    let mut digest=Sha256::new();let mut buffer=[0u8;65536];let mut total=0u64;
    loop{let n=f.read(&mut buffer)?;if n==0{break;}total=total.checked_add(n as u64).ok_or("integrity:audit-overflow")?;if total>bytes{return Err("integrity:audit-file-changed".into());}digest.update(&buffer[..n]);}
    m.opens+=1;m.bytes_read+=total;m.pack_storage_bytes+=total;m.pack_full_audit_bytes+=total;
    if total!=bytes||format!("{:x}",digest.finalize())!=hash{return Err("integrity:audit-file-hash".into());}Ok(())
}
struct BuildNode{start:usize,end:usize,height:u8,offset:usize,bytes:usize,hash:String,children:Vec<BuildNode>}
fn header(out:&mut Vec<u8>,d:&Descriptor,es:&[Entry],height:u8,slots:usize)->Result<()>{
    out.extend(MAGIC);out.extend([height,role(&d.role)?,d.codec,0]);out.extend((es.len() as u32).to_le_bytes());out.extend((slots as u16).to_le_bytes());out.extend(es[0].offset.to_le_bytes());out.extend(es.last().unwrap().offset.checked_add(es.last().unwrap().encoded as u64).ok_or("integrity:paged-overflow")?.to_le_bytes());out.extend(hash(&d.pack)?);out.extend(d.pack_bytes.to_le_bytes());string(out,&es[0].key.0)?;string(out,&es.last().unwrap().key.0)?;Ok(())
}
fn assign(node:&mut BuildNode,entries:&[Entry],next:&mut usize)->Result<()>{
    let es=&entries[node.start..node.end];let header=78usize.checked_add(es[0].key.0.len()).and_then(|n|n.checked_add(es.last().unwrap().key.0.len())).ok_or("integrity:paged-overflow")?;
    let body=if node.height==0{es.iter().map(|e|e.key.0.len()+50).sum::<usize>()}else{node.children.iter().map(|c|entries[c.start].key.0.len()+66).sum::<usize>()};
    node.bytes=header.checked_add(body).ok_or("integrity:paged-overflow")?;if node.bytes>PAGE{return Err("refused:paged-page-size".into());}node.offset=*next;*next=next.checked_add(node.bytes).ok_or("integrity:paged-overflow")?;
    for child in &mut node.children{assign(child,entries,next)?;}Ok(())
}
fn emit(node:&mut BuildNode,d:&Descriptor,entries:&[Entry],out:&mut[u8])->Result<()>{
    for child in &mut node.children{emit(child,d,entries,out)?;}
    let es=&entries[node.start..node.end];let mut page=Vec::with_capacity(node.bytes);header(&mut page,d,es,node.height,if node.height==0{es.len()}else{node.children.len()})?;
    if node.height==0{for e in es{string(&mut page,&e.key.0)?;page.extend(e.offset.to_le_bytes());page.extend((e.encoded as u32).to_le_bytes());page.extend((e.raw as u32).to_le_bytes());page.extend(hash(&e.hash)?);}}
    else{for child in &node.children{string(&mut page,&entries[child.start].key.0)?;page.extend((child.offset as u64).to_le_bytes());page.extend((child.bytes as u32).to_le_bytes());page.extend(hash(&child.hash)?);page.extend(((child.end-child.start) as u32).to_le_bytes());page.extend(entries[child.start].offset.to_le_bytes());let end=&entries[child.end-1];page.extend(end.offset.checked_add(end.encoded as u64).ok_or("integrity:paged-overflow")?.to_le_bytes());}}
    if page.len()!=node.bytes{return Err("integrity:paged-builder-size".into());}node.hash=io::digest(&page);out[node.offset..node.offset+node.bytes].copy_from_slice(&page);Ok(())
}
pub(crate) fn build(d:&Descriptor,entries:&[Entry],approved_output:usize)->Result<(Vec<u8>,Root)>{
    let entry_bytes=entries.iter().try_fold(0usize,|n,e|->Result<usize>{typed(&e.key.0)?;n.checked_add(e.key.0.len()+50).ok_or_else(||"integrity:paged-overflow".into())})?;
    if size_bound(entries.len(),entry_bytes)?>pack::MAX_INDEX{return Err("refused:paged-index-size".into());}
    let mut nodes=entries.chunks(SLOTS).enumerate().map(|(n,es)|BuildNode{start:n*SLOTS,end:n*SLOTS+es.len(),height:0,offset:0,bytes:0,hash:String::new(),children:vec![]}).collect::<Vec<_>>();
    while nodes.len()>1{let mut next=vec![];let mut it=nodes.into_iter();loop{let children=it.by_ref().take(SLOTS).collect::<Vec<_>>();if children.is_empty(){break;}next.push(BuildNode{start:children[0].start,end:children.last().unwrap().end,height:children[0].height+1,offset:0,bytes:0,hash:String::new(),children});}nodes=next;}
    let mut root=nodes.pop().ok_or("refused:empty-paged-index")?;let mut length=0;assign(&mut root,entries,&mut length)?;if length>pack::MAX_INDEX||root.height>2{return Err("refused:paged-index-size".into());}
    if length>approved_output{return Err("integrity:unreserved-index-output".into());}
    let mut out=vec![0;length];emit(&mut root,d,entries,&mut out)?;Ok((out,Root{version:2,hash:root.hash,bytes:root.bytes as u32,height:root.height}))
}
#[cfg(test)]
mod tests{
    use super::*;
    use std::io::Write;
    fn fixture(n:usize)->(PathBuf,Descriptor){
        let root=PathBuf::from(std::env::var_os("EXP_PACK_TEST_ROOT").expect("explicit fixture root required")).join(io::unique("paged"));std::fs::create_dir_all(root.join("packs")).unwrap();
        let mut b=pack::Builder::new("result",0);for k in 0..n{b.push(Key::new(&format!("results/{:064x}.json",k*2)).unwrap(),b"raw").unwrap();}
        let (d,p,i)=b.finish().unwrap();std::fs::write(root.join(d.path("pack").unwrap()),p).unwrap();std::fs::write(root.join(d.path("idx").unwrap()),i).unwrap();(root,d)
    }
    fn patch(root:&Path,d:&Descriptor,offset:u64)->u8{let mut f=std::fs::OpenOptions::new().read(true).write(true).open(root.join(d.path("idx").unwrap())).unwrap();f.seek(SeekFrom::Start(offset)).unwrap();let mut b=[0];f.read_exact(&mut b).unwrap();f.seek(SeekFrom::Start(offset)).unwrap();f.write_all(&[b[0]^1]).unwrap();b[0]}
    #[test]fn paged_exact_geometry_and_closed_keys(){
        for (s,len) in [(format!("publications/{}.json","a".repeat(99)),117),(format!("results/{}.json","a".repeat(64)),77),(format!("attempt-hints/{}.json","a".repeat(99)),118),(format!("attempts/{}/candidate.json","a".repeat(99)),123),(format!("attempts/{}/allocations/{}.json","a".repeat(99),"b".repeat(99)),225)]{assert_eq!(s.len(),len);typed(&s).unwrap();}
        for s in [format!("publications/{}.json","a".repeat(100)),format!("results/{}.json","a".repeat(65)),"attempts/a/extra/candidate.json".into(),"publications/é.json".into()]{assert!(typed(&s).is_err());}
        assert_eq!(size_bound(55366,55366*275).unwrap(),16777059);assert!(size_bound(55367,55367*275).unwrap()>pack::MAX_INDEX);assert!(size_bound(65536,65536*275).unwrap()>pack::MAX_INDEX);assert!(size_bound(1,usize::MAX).is_err());
    }
    #[test]fn paged_positive_negative_boundary_and_cache(){
        let (root,d)=fixture(2400);let mut m=Metrics::default();let reader=Reader::open(&root,&d,&mut m).unwrap();assert_eq!(d.page.as_ref().unwrap().height,2);
        for n in [0,47,48,2303,2304,2399]{let before=m.pack_page_reads;let key=Key::new(&format!("results/{:064x}.json",n*2)).unwrap();assert_eq!(reader.find(&key,&mut m).unwrap().unwrap().key,key);assert!(m.pack_page_reads-before<=3);assert_eq!(reader.ordinal(n,&mut m).unwrap().key,key);}
        for n in (0..2400).step_by(48){let key=Key::new(&format!("results/{:064x}.json",n*2+1)).unwrap();assert!(reader.find(&key,&mut m).unwrap().is_none());assert!(reader.pages.borrow().len()<=CACHE_PAGES);}
        let cached:usize=reader.pages.borrow().iter().map(|((_,_,h),b)|h.capacity()+b.capacity()+192).sum();assert!(cached+65536+3*48*512<MEMORY_BOUND);reader.audit(&mut m).unwrap();
        eprintln!("paged-route-receipt pages={} cache_pages={} actual_cached_capacity={} charged_reader_bound={}",m.pack_page_reads,reader.pages.borrow().len(),cached,MEMORY_BOUND);
    }
    #[test]fn paged_touched_untouched_and_malformed_refusal(){
        let (root,d)=fixture(120);let mut m=Metrics::default();let reader=Reader::open(&root,&d,&mut m).unwrap();let (e,h)=reader.root();let bytes=reader.page(&e,&mut m).unwrap();let node=parse(&bytes,&d,&e,h,None,Some(&d.last)).unwrap();let last=node.children.last().unwrap().clone();
        patch(&root,&d,last.offset);let fresh=Reader::open(&root,&d,&mut m).unwrap();let first=Key::new(&d.first).unwrap();assert!(fresh.find(&first,&mut m).unwrap().is_some());assert!(fresh.find(&Key::new(&d.last).unwrap(),&mut m).is_err());assert!(fresh.audit(&mut m).is_err());patch(&root,&d,last.offset);
        for at in [11usize,16]{let mut invalid=bytes.to_vec();invalid[at]=255;assert!(parse(&invalid,&d,&e,h,None,Some(&d.last)).is_err());}
        assert!(parse(&bytes[..bytes.len()-1],&d,&e,h,None,Some(&d.last)).is_err());let mut wrong=d.clone();wrong.page.as_mut().unwrap().version=99;assert!(Reader::open(&root,&wrong,&mut m).is_err());
    }
    #[test]fn paged_maximum_keys_split_before_total_index_bound(){
        let mut builder=pack::Builder::new("allocation",0);let attempt="a".repeat(99);let mut count=0usize;
        loop{let key=Key::new(&format!("attempts/{attempt}/allocations/{count:099}.json")).unwrap();if !builder.fits(&key,0){break;}builder.push(key,b"").unwrap();count+=1;}
        assert_eq!(count,55366);let (d,pack,index)=builder.finish().unwrap();assert!(pack.is_empty());assert_eq!(d.records,count as u32);assert!(index.len()<=pack::MAX_INDEX);assert_eq!(index.len() as u64,d.index_bytes);assert_eq!(d.page.as_ref().unwrap().height,2);
        eprintln!("paged-maximum-key-builder records={} actual_index_bytes={} serialized_upper={}",count,index.len(),size_bound(count,count*275).unwrap());
    }
}
