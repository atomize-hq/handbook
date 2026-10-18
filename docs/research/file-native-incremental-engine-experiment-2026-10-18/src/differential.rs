//! Independent full-map baseline apply/validation versus final-overlay execution.
use crate::{io::{self,Metrics,Result},model::{self,Op},objects,graph};
use serde_json::{json,Value};
use std::path::Path;
pub fn run(root:&Path,n:usize,seed:u64,m:&mut Metrics)->Result<Value>{
    if root.exists()||![128,1024].contains(&n)||![11,29,71,113].contains(&seed){return Err("differential:frozen-allocation".into());}
    io::mkdir(root,m)?;let mut state=model::fixture(root,"project",n,m)?;
    let mut o=objects::object_store(root,m);let mut g=graph::build(&mut o,&state,false)?;drop(o);
    let mut random=seed;let mut accepted=0;let mut refused=0;
    fn next(r:&mut u64)->u64{*r=r.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);*r}
    for batch in 0..250{
        let mut ops=vec![];
        for _ in 0..1+next(&mut random)%4{
            let id=format!("d{:06}",next(&mut random)%n as u64);let choice=next(&mut random)%7;
            let (kind,text,target,value)=match choice{
                0=>("label",format!("label-{}",next(&mut random)%32),String::new(),0),
                1=>("rename",format!("title-{}",next(&mut random)%16),String::new(),0),
                2=>("cost",String::new(),String::new(),next(&mut random)%700),
                3=>("edge","requires".into(),format!("d{:06}",next(&mut random)%(n as u64+3)),0),
                4=>("delete",String::new(),String::new(),0),
                5=>("cost",String::new(),String::new(),u64::MAX),
                _=>("cost",String::new(),String::new(),0)
            };ops.push(Op{kind:kind.into(),id,text,target,value});
        }
        let mut full=state.clone();
        let expected=model::apply(root,&mut full,&ops,m);
        let mut o=objects::object_store(root,m);let incremental=graph::transition(root,&mut o,&g,&ops);
        if expected.is_ok()!=incremental.is_ok(){return Err(format!("differential:acceptance:{seed}:{n}:{batch}:expected={:?}:actual={:?}",expected.err().map(|e|e.to_string()),incremental.err().map(|e|e.to_string())).into());}
        match incremental{
            Ok(delta)=>{
                accepted+=1;g=delta.graph;state=full;
                let actual=graph::all(&mut o,&g)?;
                if serde_json::to_vec(&actual)?!=serde_json::to_vec(&state)?{return Err(format!("differential:documents:{seed}:{n}:{batch}").into());}
                graph::audit(&mut o,&g)?;
            },
            Err(_)=>refused+=1
        }
    }
    let mut o=objects::object_store(root,m);graph::audit(&mut o,&g)?;
    Ok(json!({"seed":seed,"n":n,"batches":250,"accepted":accepted,"refused":refused,"documents_and_all_logical_indexes":"equal","graph":g}))
}
