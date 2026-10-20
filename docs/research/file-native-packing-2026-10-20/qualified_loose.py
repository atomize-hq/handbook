"""Exact 4bde core plus a separate resident timing consumer, no engine edits."""
import hashlib,json,subprocess,sys
from pathlib import Path
HERE=Path(__file__).resolve().parent;WORK=HERE.parents[2]
prefix='docs/research/file-native-incremental-engine-experiment-2026-10-18/'
pin='4bde156323812ef18c0dea6798cfc2108aaad933';root=Path('C:/file-native-packing-20261020/qualified-loose-core')
assert not root.exists();root.mkdir()
paths=subprocess.check_output(['git','ls-tree','-r','--name-only',pin,'--',prefix],cwd=WORK).decode().splitlines()
bindings={}
for path in paths:
    name=path[len(prefix):]
    if name not in ['Cargo.toml','Cargo.lock'] and not name.startswith(('src/','consumers/project/','consumers/research/')):continue
    raw=subprocess.check_output(['git','show',pin+':'+path],cwd=WORK);p=root/name;p.parent.mkdir(parents=True,exist_ok=True);p.write_bytes(raw);bindings[name]=hashlib.sha256(raw).hexdigest()
expected=json.loads(Path('C:/Users/spmcc/.codex/worktrees/file-native-qnp-integration-20261019/handbook/docs/research/file-native-navigation-2026-10-19/results/source-binding.json').read_bytes())['source_sha256']
for name,h in expected.items():assert bindings[('' if name.startswith('Cargo') else 'src/')+name]==h
main=r'''use file_native_incremental::{io::{self,Metrics,Result},store::Store};
use serde_json::{Value,json};use std::{io::Read,path::Path,time::Instant};
fn run(v:Value,m:&mut Metrics)->Result<Value>{
 let root=Path::new(v["root"].as_str().ok_or("root")?);let store=Store::open(root,m)?;
 let count=v["count"].as_u64().unwrap_or(500);let requests=v["requests"].as_array().ok_or("requests")?;
 if count==0||count>5000||requests.len()>64{return Err("sample-bound".into());}let mut rows=vec![];
 for request in requests{let id=request["id"].as_str().ok_or("id")?;let category=request["category"].as_str().ok_or("category")?;
  let t=Instant::now();let expected=store.resolve(id,None,m)?;let first=t.elapsed().as_nanos();let mut times=vec![];let opens=m.opens;
  for _ in 0..count{let t=Instant::now();let result=store.resolve(id,None,m)?;times.push(t.elapsed().as_nanos());if result!=expected{return Err("outcome-changed".into());}}
  let mut sorted=times.clone();sorted.sort_unstable();rows.push(json!({"category":category,"id":id,"first_lookup_ns":first,"warm_ns":times,"warm_median_ns":sorted[sorted.len()/2],"warm_p95_ns":sorted[(sorted.len()*95/100).min(sorted.len()-1)],"warm_min_ns":sorted[0],"warm_max_ns":sorted.last(),"result_digest":io::digest(&serde_json::to_vec(&expected)?),"opens":m.opens-opens}));
 }Ok(json!({"rows":rows,"qualification":"exact qualified 4bde library with separately bound resident measurement consumer; warm after first lookup; caches uncontrolled"}))
}
fn main(){std::env::set_var("EXP_CODEC","compact");std::env::set_var("EXP_LAYOUT","sharded");let t=Instant::now();let mut m=Metrics::default();let mut input=vec![];std::io::stdin().take(4*1024*1024+1).read_to_end(&mut input).unwrap();assert!(input.len()<=4*1024*1024);let result=run(serde_json::from_slice(&input).unwrap(),&mut m);m.finish();match result{Ok(v)=>println!("{}",json!({"status":"ok","result":v,"metrics":m,"wall_ns":t.elapsed().as_nanos(),"peak_memory_bytes":io::peak_memory()})),Err(e)=>{println!("{}",json!({"status":"error","error":e.to_string(),"metrics":m}));std::process::exit(2);}}}
'''
p=root/'src/bin/qualified_lookup.rs';p.parent.mkdir(exist_ok=True);p.write_text(main,encoding='utf-8')
result=dict(core_pin=pin,core_unchanged=True,core_sha256=bindings,consumer_source_sha256=hashlib.sha256(p.read_bytes()).hexdigest(),root=str(root),qualified_original_exe='C:/file-native-navigation-20261019/bin/final.exe',qualified_original_exe_sha256='d4b7c495abfc029e8fabd6fcfc75501c5de04ce72dd38847a69bc8824022303f')
Path('C:/file-native-packing-20261020/receipts/qualified-loose-source.json').write_text(json.dumps(result,indent=2));print(json.dumps(dict(core_pin=pin,core_files=len(bindings),consumer_source_sha256=result['consumer_source_sha256'])))
