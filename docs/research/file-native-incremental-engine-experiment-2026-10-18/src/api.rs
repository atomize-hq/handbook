use crate::{io::{self,Metrics,Result},objects,graph,store::{Store,Preparation},projection,lifetime,model::{Request,Op,Selector}};
use serde_json::{Value,json};
use serde::Deserialize;
use std::{io::{Read,Write},path::PathBuf,time::Instant,fs::OpenOptions};
pub fn run(v:Value,m:&mut Metrics,consumer:Option<&str>)->Result<Value>{
    let root=PathBuf::from(v["root"].as_str().ok_or("root")?);let action=v["action"].as_str().ok_or("action")?;
    if action=="differential"{return crate::differential::run(&root,v["count"].as_u64().ok_or("count")? as usize,v["seed"].as_u64().ok_or("seed")?,m);}
    if action=="init"{
        return Store::init(&root,v["mode"].as_str().unwrap_or("A"),consumer.or(v["consumer"].as_str()).unwrap_or("project"),
            v["count"].as_u64().ok_or("count")? as usize,v["fallback"].as_bool().unwrap_or(false),m);
    }
    let store=Store::open(&root,m)?;
    if consumer.is_some_and(|c|c!=store.config.consumer){return Err("refused:consumer-kind".into());}
    match action{
        "validation-trace"=>{
            let snapshot=store.snapshot(m)?;let ops:Vec<Op>=serde_json::from_value(v["ops"].clone())?;
            let mut full_metrics=Metrics::default();let started=Instant::now();
            let mut o=objects::object_store(&root,&mut full_metrics);
            let mut full=graph::all(&mut o,&snapshot.publication.graph)?;drop(o);
            crate::model::apply(&root,&mut full,&ops,&mut full_metrics)?;
            let full_ns=started.elapsed().as_nanos();
            let mut incremental_metrics=Metrics::default();let started=Instant::now();
            let p=match store.prepare(Request{request:io::unique("trace"),base:Some(snapshot.selected),ops,packet:false},&mut incremental_metrics)?{
                Preparation::Ready(p)=>p,_=>return Err("trace:unexpected-dedup".into())
            };
            let incremental_ns=started.elapsed().as_nanos();
            full_metrics.finish();incremental_metrics.finish();
            let path=Store::candidate_path(&p);
            let c:crate::store::Candidate=serde_json::from_slice(&io::read(&root.join(&path),m)?)?;
            let mut o=objects::object_store(&root,m);
            if serde_json::to_vec(&full)?!=serde_json::to_vec(&graph::all(&mut o,&c.graph)?)?{return Err("trace:oracle-mismatch".into());}
            graph::audit(&mut o,&c.graph)?;
            Ok(json!({"full_load_semantic_validation_ns":full_ns,"incremental_complete_preparation_ns":incremental_ns,
                "full_metrics":full_metrics,"incremental_metrics":incremental_metrics,"candidate":path,
                "scope":"reference includes full load+semantic validation; incremental includes graph/projection persistence, reservation and sealing; explicit comparison audit excluded from these clocks"}))
        },
        "publish"=>store.publish(serde_json::from_value(v["request"].clone())?,m),
        "prepare"=>match store.prepare(serde_json::from_value(v["request"].clone())?,m)?{
            Preparation::Existing(result)=>Ok(json!({"status":"deduplicated","result":result})),
            Preparation::Ready(p)=>Ok(json!({"candidate":Store::candidate_path(&p)}))
        },
        "select"=>store.select_file(v["candidate"].as_str().ok_or("candidate")?,m),
        "resolve"=>if v["intent"].is_string(){store.resolve(v["request_id"].as_str().ok_or("request_id")?,v["intent"].as_str(),m)}
            else{Ok(serde_json::to_value(store.resolve_typed(v["request_id"].as_str().ok_or("request_id")?,m)?)?)},
        "checkpoint"=>store.checkpoint(v["request_id"].as_str().ok_or("request_id")?,m),
        "cancel"=>store.cancel(v["request_id"].as_str().ok_or("request_id")?,m),
        "cancel-signal"=>{
            let id=v["request_id"].as_str().ok_or("request_id")?;
            io::atomic(&root.join(format!("cancel-signals/{}",io::digest(id.as_bytes()))),b"signal",m)?;
            Ok(json!({"status":"signal-only-not-terminal"}))
        },
        "lifecycle"=>lifetime::run(&store,&v,m),
        "inspect"=>{
            let s=store.snapshot(m)?;
            Ok(json!({"selected":s.selected,"publication":s.publication,"history_depth":s.publication.sequence+1}))
        },
        "point"=>{
            let head=store.snapshot(m)?;
            io::checkpoint("reader-opened")?;
            let sel:Selector=if v["selected"].is_object(){serde_json::from_value(v["selected"].clone())?}else{head.selected};
            let p=store.member(&sel,m)?;lifetime::live(&root,&sel)?;
            let mut o=objects::object_store(&root,m);
            let h=graph::header(&mut o,&p.graph,v["id"].as_str().ok_or("id")?)?;
            Ok(json!({"selected":sel,"value":h}))
        },
        "audit"=>{
            projection::full_oracle(&store,m)?;
            let s=store.snapshot(m)?;let mut o=objects::object_store(&root,m);
            for (_,h) in objects::range::<graph::Header>(&mut o,&s.publication.graph.tree,"D/")?{
                io::checked(&root.join(h.doc.original),&h.doc.digest,o.metrics)?;
            }
            Ok(json!({"status":"full-graph-index-source-projection-audit","selected":s.selected}))
        },
        "dump"=>{
            let s=store.snapshot(m)?;let mut o=objects::object_store(&root,m);
            Ok(json!({"state":graph::all(&mut o,&s.publication.graph)?,"graph":s.publication.graph}))
        },
        "view-audit"=>projection::audit_view(&store,m),
        "resume"|"refresh"=>{
            let _guard=store.lock(m)?;projection::refresh(&store,action=="resume",m)?;
            Ok(json!({"status":if action=="resume"{"audited-rebuilt"}else{"pending-refreshed"}}))
        },
        "packet"=>{
            let sel=store.snapshot(m)?.selected;
            let request=Request{request:io::unique("packet"),base:Some(sel),ops:vec![],packet:true};
            match store.prepare(request,m)?{
                Preparation::Existing(_)=>Err("integrity:packet-id-reuse".into()),
                Preparation::Ready(p)=>{
                    let path=Store::candidate_path(&p);
                    Ok(json!({"path":path.replace("/candidate.json","/packet"),"candidate":path,"selection_unchanged":true}))
                }
            }
        },
        "history"=>history(&store,&v,m),
        _=>Err("unknown-action".into())
    }
}
fn history(store:&Store,v:&Value,m:&mut Metrics)->Result<Value>{
    let count=v["count"].as_u64().ok_or("count")?;if count>100_000{return Err("history-count-cap".into());}
    let offset=v["start"].as_u64().unwrap_or(0);
    let mut options=OpenOptions::new();options.write(true);
    if offset==0{options.create_new(true);}else{options.append(true);}
    let mut ledger=options.open(store.root.join("expected-ledger.jsonl"))?;
    let head=store.snapshot(m)?;let mut o=objects::object_store(&store.root,m);
    if head.publication.sequence!=offset{return Err("history:exact-depth-required".into());}
    let doc=graph::header(&mut o,&head.publication.graph,"d000000")?.ok_or("history-doc")?.doc;drop(o);
    let source_path=store.root.join("HISTORY-SOURCE.json");
    if offset==0{io::write_new(&source_path,&io::json(&doc)?,m)?;}
    let origin:crate::model::Doc=serde_json::from_slice(&io::read(&source_path,m)?)?;
    let original=io::checked(&store.root.join(&origin.original),&origin.digest,m)?;
    let mut variants=[original.clone(),original.clone()];let last=variants[1].len()-1;
    variants[1][last]=if variants[1][last]==b' ' {b'\n'}else{b' '};
    let started=Instant::now();let mut selected=head.selected;let mut counts=std::collections::BTreeMap::new();
    for i in offset+1..=offset+count{
        let category=match i%5{0=>"maintenance",1=>"content",2=>"metadata",3=>"equal-content",_=>"restore"};
        *counts.entry(category).or_insert(0u64)+=1;
        let ops=match i%5{
            1|4=>vec![Op{kind:"import".into(),id:"d000000".into(),text:String::from_utf8(variants[usize::from(i%5==1)].clone())?,target:String::new(),value:0}],
            2=>vec![Op{kind:"label".into(),id:format!("d{:06}",i%16),text:format!("history-{}-{}",i%16,(i/5)%2),target:String::new(),value:0}],
            3=>vec![Op{kind:"cost".into(),id:format!("d{:06}",i%16),text:String::new(),target:String::new(),value:0}],
            _=>vec![]
        };
        let request=Request{request:format!("history-{i:06}"),base:Some(selected.clone()),ops,packet:false};
        let t=Instant::now();let mut per=Metrics::default();let out=store.publish(request,&mut per)?;
        if out["status"]!="published"||out["result"]["outcome"]!="accepted"{return Err(format!("history:uncompleted-boundary:{out}").into());}
        selected=serde_json::from_value(out["selected"].clone())?;
        per.finish();
        let row=json!({"index":i,"category":category,"result":out["result"],"selected":selected,"engine_ns":t.elapsed().as_nanos(),"metrics":per});
        serde_json::to_writer(&mut ledger,&row)?;ledger.write_all(b"\n")?;ledger.sync_all()?;
        if i%100==0{eprintln!("history-progress:{i}:{}",started.elapsed().as_secs());}
        if i==1000||i==10000{io::checkpoint(&format!("history-depth-{i}"))?;}
    }
    Ok(json!({"completed":count,"selected":selected,"categories":counts,"resident_ns":started.elapsed().as_nanos(),"independent_ledger":"expected-ledger.jsonl"}))
}
pub fn main_for(consumer:Option<&str>){
    std::env::set_var("EXP_CODEC","compact");std::env::set_var("EXP_LAYOUT","sharded");
    let t=Instant::now();let mut m=Metrics::default();let mut bytes=vec![];
    let parsed=(||->Result<Value>{
        std::io::stdin().take(4*1024*1024+1).read_to_end(&mut bytes)?;
        if bytes.len()>4*1024*1024{return Err("request-cap".into());}
        let v=strict_json(std::str::from_utf8(&bytes)?)?;run(v,&mut m,consumer)
    })();
    let (status,result,error)=match parsed{Ok(v)=>("ok",v,None),Err(e)=>("error",Value::Null,Some(e.to_string()))};
    m.finish();
    println!("{}",json!({"status":status,"result":result,"error":error,"metrics":m,"wall_ns":t.elapsed().as_nanos(),"peak_memory_bytes":io::peak_memory()}));
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
