//! Record-subset geometry harness. It creates no STORE or SELECTED control.
use serde::{Deserialize,Serialize};
use std::{fs::{File,OpenOptions},io::{Read,Write,Seek,SeekFrom,BufRead,BufReader},path::{Path,PathBuf},time::Instant};
mod io {
 use super::*;use sha2::{Digest,Sha256};pub type Result<T>=std::result::Result<T,Box<dyn std::error::Error>>;
 #[derive(Default,Serialize)] pub struct Metrics{pub opens:u64,pub bytes_read:u64,pub pack_storage_bytes:u64,pub pack_records_read:u64,pub pack_decoded_bytes:u64,pub pack_index_loads:u64,pub logical_record_bytes:u64}
 impl Metrics{pub fn record_bytes(&mut self,_path:&Path,n:u64,_write:bool){self.logical_record_bytes+=n;}}
 pub fn digest(b:&[u8])->String{format!("{:x}",Sha256::digest(b))}pub fn checkpoint(_name:&str)->Result<()>{Ok(())}
}
mod model{pub fn safe_id(s:&str)->bool{!s.is_empty()&&s.len()<100&&s.bytes().all(|b|b.is_ascii_alphanumeric()||b==b'-'||b==b'_')}}
mod objects{pub fn valid_hash(s:&str)->bool{s.len()==64&&s.bytes().all(|b|b.is_ascii_digit()||(b'a'..=b'f').contains(&b))}}
// Compile the actual frozen pack codec/index implementation, not a second layout.
#[path="../../file-native-incremental-engine-experiment-2026-10-18/src/pack.rs"] mod pack;
#[derive(Deserialize)]#[serde(deny_unknown_fields)]struct Input{role:String,key:String,raw_length:usize,sha256:String,offset:u64}
fn save(path:&Path,bytes:&[u8])->io::Result<()>{let mut f=OpenOptions::new().write(true).create_new(true).open(path)?;f.write_all(bytes)?;f.sync_all()?;Ok(())}
fn flush(root:&Path,builder:pack::Builder,m:&mut io::Metrics)->io::Result<pack::Descriptor>{
 let(d,bytes,index)=builder.finish()?;save(&root.join(format!("packs/{}.pack",d.pack)),&bytes)?;save(&root.join(format!("packs/{}.idx",d.index)),&index)?;
 let checked=pack::Index::load(root,&d,m)?;for n in 0..d.records as usize{let e=checked.entry(n)?;checked.read(root,&e,pack::MAX_RAW,m)?;}Ok(d)
}
fn run()->io::Result<()>{
 let args:Vec<_>=std::env::args().collect();if args.len()!=5{return Err("usage: corpus inventory output raw|zstd".into());}
 let started=Instant::now();let mut corpus=File::open(&args[1])?;let bytes=corpus.metadata()?.len();if bytes>64*1024*1024{return Err("subset-corpus-bound".into());}
 let inventory=File::open(&args[2])?;if inventory.metadata()?.len()>16*1024*1024{return Err("subset-inventory-bound".into());}
 let mut rows=vec![];for line in BufReader::new(inventory).lines(){let line=line?;if line.len()>4096||rows.len()>=50000{return Err("subset-row-bound".into());}rows.push(serde_json::from_str::<Input>(&line)?);}
 rows.sort_by(|a,b|(&a.role,&a.key).cmp(&(&b.role,&b.key)));let root=PathBuf::from(&args[3]);if root.exists(){return Err("fresh-output-required".into());}std::fs::create_dir_all(root.join("packs"))?;
 let codec=match args[4].as_str(){"raw"=>0,"zstd"=>1,_=>return Err("codec".into())};let mut builder:Option<pack::Builder>=None;let mut role=String::new();let mut packs=vec![];let mut m=io::Metrics::default();let mut raw_bytes=0u64;let mut seen=std::collections::BTreeSet::new();
 for row in rows{
  let key=pack::Key::new(&row.key)?;if key.role()!=Some(row.role.as_str())||!seen.insert(row.key)||row.raw_length>pack::MAX_RAW||row.offset.checked_add(row.raw_length as u64).is_none_or(|n|n>bytes){return Err("subset-input-binding".into());}
  corpus.seek(SeekFrom::Start(row.offset))?;let mut raw=vec![0;row.raw_length];corpus.read_exact(&mut raw)?;if io::digest(&raw)!=row.sha256{return Err("subset-input-digest".into());}raw_bytes+=raw.len() as u64;
  if builder.as_ref().is_some_and(|b|role!=row.role||!b.fits(&key,raw.len())){packs.push(flush(&root,builder.take().unwrap(),&mut m)?);}
  if builder.is_none(){role=row.role;builder=Some(pack::Builder::new(&role,codec));}builder.as_mut().unwrap().push(key,&raw)?;
 }
 if let Some(b)=builder{packs.push(flush(&root,b,&mut m)?);}let result=serde_json::json!({"profile":"authenticated-record-subset-nonoperational","codec":codec,"records":seen.len(),"raw_bytes":raw_bytes,"pack_plus_index_bytes":packs.iter().map(|d|d.pack_bytes+d.index_bytes).sum::<u64>(),"packs":packs,"self_verification_metrics":m,"seconds":started.elapsed().as_secs_f64(),"no_operational_controls":true});save(&root.join("sample.json"),&serde_json::to_vec_pretty(&result)?)?;println!("{result}");Ok(())
}
fn main(){if let Err(e)=run(){eprintln!("{e}");std::process::exit(2);}}
