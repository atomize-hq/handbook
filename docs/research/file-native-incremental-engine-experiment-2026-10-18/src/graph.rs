//! I1 final-overlay constraint preservation. Full recomputation is an explicit oracle.
use crate::{io::{self,Metrics,Result},objects::{self,Id,Objects},model::{self,Doc,Edge,Op,State}};
use serde::{Serialize,Deserialize};
use std::{collections::{BTreeMap,BTreeSet},path::Path};
#[derive(Clone,Debug,Serialize,Deserialize,PartialEq,Eq)]
#[serde(deny_unknown_fields)]
pub struct GraphRoot{
    pub version:u32,pub consumer:String,pub budget:u64,pub cost:u64,pub count:u64,pub tree:Option<Id>,
    pub fallback:bool,
}
#[derive(Clone,Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Header{pub doc:Doc,pub next:u64}
pub fn edge_key(id:&str,n:u64)->String{format!("E/{id}/{n:016x}")}
fn reverse_key(target:&str,id:&str,n:u64)->String{format!("R/{target}/{id}/{n:016x}")}
pub struct KindRules{pub source_kind:&'static str,pub forbidden_source_kind:&'static str}
pub fn kind_rules(consumer:&str)->Result<KindRules>{
    match consumer{
        "project"=>Ok(KindRules{source_kind:"decision-source",forbidden_source_kind:"note-quote"}),
        "research"=>Ok(KindRules{source_kind:"note-quote",forbidden_source_kind:"decision-source"}),
        _=>Err("semantic:consumer-adapter".into())
    }
}
fn local(d:&Doc,consumer:&str)->Result<()>{
    if d.kind==kind_rules(consumer)?.forbidden_source_kind{return Err("semantic:consumer-kind".into());}
    if !model::safe_id(&d.id)||!d.path.starts_with("docs/item-")||d.path.contains("..")||d.path.contains('\\')||d.path.len()>900
        ||d.title.len()>4096||d.kind.is_empty()||d.kind.len()>64||d.label.as_ref().is_some_and(|s|s.len()>900)
        ||!objects::valid_hash(&d.digest)||d.bytes>128*1024 {
        return Err("semantic:local-I1".into());
    }Ok(())
}
fn edge_rule(e:&Edge,target:Option<&Header>)->Result<()>{
    if !["requires","cites","constituent","attachment"].contains(&e.kind.as_str()){return Err("semantic:edge-kind".into());}
    if e.external {if !e.target.starts_with("https://"){return Err("semantic:external-locator".into());}}
    else{
        let target=target.ok_or("semantic:local-reference")?;
        if target.doc.kind=="blocked"{return Err("semantic:target-kind-I1".into());}
    }Ok(())
}
pub fn header(o:&mut Objects,g:&GraphRoot,id:&str)->Result<Option<Header>>{
    o.metrics.documents_visited+=1;objects::point(o,&g.tree,&format!("D/{id}"))
}
pub fn outgoing(o:&mut Objects,g:&GraphRoot,id:&str)->Result<Vec<(u64,Edge)>>{
    let rows=objects::range::<Edge>(o,&g.tree,&format!("E/{id}/"))?;
    o.metrics.edges_visited+=rows.len() as u64;
    rows.into_iter().map(|(k,e)|Ok((u64::from_str_radix(k.rsplit('/').next().unwrap(),16)?,e))).collect()
}
pub fn all(o:&mut Objects,g:&GraphRoot)->Result<State>{
    let mut docs=BTreeMap::new();
    for (_,h) in objects::range::<Header>(o,&g.tree,"D/")?{docs.insert(h.doc.id.clone(),h.doc);}
    o.metrics.documents_visited+=docs.len() as u64;
    for (key,e) in objects::range::<Edge>(o,&g.tree,"E/")?{
        o.metrics.edges_visited+=1;
        let id=key.split('/').nth(1).ok_or("integrity:edge-key")?;
        docs.get_mut(id).ok_or("integrity:edge-source")?.edges.push(e);
    }
    Ok(State{consumer:g.consumer.clone(),budget:g.budget,docs})
}
pub fn recompute(s:&State)->Result<(BTreeMap<String,Vec<u8>>,u64)>{
    model::validate(s)?;let mut pairs=BTreeMap::new();let mut cost=0u64;
    for (id,d) in &s.docs{
        local(d,&s.consumer)?;cost=cost.checked_add(d.cost).ok_or("semantic:overflow")?;
        let mut h=Header{doc:d.clone(),next:d.edges.len() as u64};h.doc.edges.clear();
        pairs.insert(format!("D/{id}"),serde_json::to_vec(&h)?);
        pairs.insert(format!("P/{}",d.path.to_lowercase()),serde_json::to_vec(id)?);
        if let Some(label)=&d.label{pairs.insert(format!("L/{label}"),serde_json::to_vec(id)?);}
        for (n,e) in d.edges.iter().enumerate(){
            let target=s.docs.get(&e.target).map(|d|Header{doc:d.clone(),next:0});
            edge_rule(e,target.as_ref())?;
            pairs.insert(edge_key(id,n as u64),serde_json::to_vec(e)?);
            if !e.external{pairs.insert(reverse_key(&e.target,id,n as u64),serde_json::to_vec(e)?);}
        }
    }Ok((pairs,cost))
}
pub fn build(o:&mut Objects,s:&State,fallback:bool)->Result<GraphRoot>{
    let (pairs,cost)=recompute(s)?;
    let tree=objects::bulk(o,pairs)?;
    let g=GraphRoot{version:1,consumer:s.consumer.clone(),budget:s.budget,cost,count:s.docs.len() as u64,tree,fallback};
    audit(o,&g)?;Ok(g)
}
pub fn audit(o:&mut Objects,g:&GraphRoot)->Result<()>{
    if g.version!=1{return Err("integrity:graph-version".into());}
    let s=all(o,g)?;let (_,cost)=recompute(&s)?;
    if cost!=g.cost||s.docs.len() as u64!=g.count{return Err("integrity:aggregate".into());}
    // Ordinals need not be dense after removal. Rebuild all projections using the
    // stored ordinals, separately from the delta updater.
    let actual=objects::entries(o,&g.tree)?;let mut expected=BTreeMap::new();
    for (key,b) in &actual{
        if key.starts_with("D/"){
            let h:Header=objects::decode(b,o)?;local(&h.doc,&g.consumer)?;
            if key!=&format!("D/{}",h.doc.id)||!h.doc.edges.is_empty(){return Err("integrity:header-binding".into());}
            expected.insert(key.clone(),serde_json::to_vec(&h)?);
            expected.insert(format!("P/{}",h.doc.path.to_lowercase()),serde_json::to_vec(&h.doc.id)?);
            if let Some(l)=&h.doc.label{expected.insert(format!("L/{l}"),serde_json::to_vec(&h.doc.id)?);}
        }
    }
    for (key,b) in &actual{if key.starts_with("E/"){
        let mut parts=key.split('/');parts.next();let id=parts.next().ok_or("integrity:edge-key")?;
        let n=u64::from_str_radix(parts.next().ok_or("integrity:ordinal")?,16)?;
        let h:Header=objects::point(o,&g.tree,&format!("D/{id}"))?.ok_or("integrity:edge-source")?;
        if parts.next().is_some()||n>=h.next{return Err("integrity:ordinal".into());}
        let e:Edge=objects::decode(b,o)?;
        expected.insert(key.clone(),serde_json::to_vec(&e)?);
        if !e.external{expected.insert(reverse_key(&e.target,id,n),serde_json::to_vec(&e)?);}
    }}
    let decoded:BTreeMap<_,_>=actual.into_iter().map(|(k,b)|{
        let raw=if b.first()==Some(&b'@'){o.read(std::str::from_utf8(&b[1..])?)?}else{b};Ok((k,raw))
    }).collect::<Result<_>>()?;
    if decoded!=expected{return Err("integrity:index-projection-missing-extra-multiplicity".into());}
    if g.fallback{full_fallback(&s)?;}Ok(())
}
fn full_fallback(s:&State)->Result<()>{
    let mut seen=BTreeSet::new();
    for d in s.docs.values(){if !seen.insert(&d.title){return Err("semantic:opaque-global-title".into());}}Ok(())
}
pub struct Delta{
    pub graph:GraphRoot,pub changed:BTreeSet<String>,pub navigation:BTreeSet<String>,
    pub navigation_facts:BTreeSet<String>,pub navigation_sources:BTreeSet<String>,pub prior_graph:GraphRoot,
    pub old:BTreeMap<String,Option<Header>>,pub final_docs:BTreeMap<String,Option<Header>>,
}
pub fn transition(root:&Path,o:&mut Objects,g:&GraphRoot,ops:&[Op])->Result<Delta>{
    if g.version!=1||ops.len()>256{return Err("refused:graph-version-or-batch-limit".into());}
    let mut old:BTreeMap<String,Option<Header>>=BTreeMap::new();
    let mut final_docs=old.clone();
    let mut edges:BTreeMap<(String,u64),Option<Edge>>=BTreeMap::new();
    let mut old_edges:BTreeMap<(String,u64),Option<Edge>>=BTreeMap::new();
    let mut budget=g.budget;
    for op in ops{
        if op.kind=="budget"{budget=op.value;continue;}
        if !model::safe_id(&op.id){return Err("semantic:id".into());}
        if !old.contains_key(&op.id){
            let h=header(o,g,&op.id)?;old.insert(op.id.clone(),h.clone());final_docs.insert(op.id.clone(),h);
        }
        let slot=final_docs.get_mut(&op.id).unwrap();let h=slot.as_mut().ok_or("semantic:missing-document")?;
        match op.kind.as_str(){
            "delete"=>{
                for (n,e) in outgoing(o,g,&op.id)?{
                    old_edges.entry((op.id.clone(),n)).or_insert(Some(e));edges.insert((op.id.clone(),n),None);
                }
                for ((id,_),edge) in edges.iter_mut(){if id==&op.id{*edge=None;}}
                *slot=None;
            },
            "rename"=>{h.doc.title=op.text.clone();let ext=Path::new(&h.doc.path).extension().ok_or("semantic:path-extension")?.to_string_lossy();h.doc.path=model::path_for(&op.id,&op.text,&ext);},
            "path"=>h.doc.path=op.text.clone(),
            "label"=>h.doc.label=Some(op.text.clone()),
            "unlabel"=>h.doc.label=None,
            "cost"=>h.doc.cost=op.value,
            "kind"=>h.doc.kind=op.text.clone(),
            "import"=>{
                if op.text.len()>128*1024{return Err("semantic:source-bound".into());}
                model::put_original(root,&mut h.doc,op.text.as_bytes(),o.metrics)?;
            },
            "edit"=>{
                let mut b=io::checked(&root.join(&h.doc.original),&h.doc.digest,o.metrics)?;
                b.extend(op.text.as_bytes());if b.len()>128*1024{return Err("semantic:source-bound".into());}
                model::put_original(root,&mut h.doc,&b,o.metrics)?;
            },
            "edge"=>{
                if h.next>=1_000_000{return Err("semantic:edge-limit".into());}
                let key=(op.id.clone(),h.next);h.next+=1;
                old_edges.insert(key.clone(),None);
                edges.insert(key,Some(Edge{kind:op.text.clone(),target:op.target.clone(),external:false}));
            },
            "drop-edge"|"replace-edge"=>{
                let key=(op.id.clone(),op.value);
                let prior=if let Some(e)=edges.get(&key){e.clone()}else{objects::point(o,&g.tree,&edge_key(&op.id,op.value))?};
                prior.as_ref().ok_or("semantic:missing-edge")?;
                if !old_edges.contains_key(&key){old_edges.insert(key.clone(),prior);}
                edges.insert(key,if op.kind=="drop-edge"{None}else{Some(Edge{kind:op.text.clone(),target:op.target.clone(),external:false})});
            },
            _=>return Err("semantic:unknown-operation".into())
        }
    }
    let mut changes:BTreeMap<String,Vec<u8>>=BTreeMap::new();
    let mut cost=g.cost as u128;let mut count=g.count;
    for (id,prior) in &old{
        if let Some(h)=prior{
            cost=cost.checked_sub(h.doc.cost as u128).ok_or("integrity:aggregate-underflow")?;
            changes.insert(format!("D/{id}"),vec![]);changes.insert(format!("P/{}",h.doc.path.to_lowercase()),vec![]);
            if let Some(l)=&h.doc.label{changes.insert(format!("L/{l}"),vec![]);}
        }
        if prior.is_some()&&final_docs[id].is_none(){count=count.checked_sub(1).ok_or("integrity:count")?;}
    }
    for (id,next) in &final_docs{if let Some(h)=next{
        local(&h.doc,&g.consumer)?;cost=cost.checked_add(h.doc.cost as u128).ok_or("semantic:overflow")?;
        for key in std::iter::once(format!("P/{}",h.doc.path.to_lowercase())).chain(h.doc.label.as_ref().map(|l|format!("L/{l}"))){
            let existing:Option<String>=if let Some(b)=changes.get(&key){if b.is_empty(){None}else{Some(serde_json::from_slice(b)?)}}
                else{objects::point(o,&g.tree,&key)?};
            if existing.as_ref().is_some_and(|other|other!=id){return Err("semantic:unique".into());}
            changes.insert(key,serde_json::to_vec(id)?);
        }
        changes.insert(format!("D/{id}"),objects::encode(h,o)?);
    }}
    if cost>u64::MAX as u128{return Err("semantic:overflow".into());}
    if cost>budget as u128{return Err("semantic:budget".into());}
    for ((id,n),prior) in &old_edges{
        changes.insert(edge_key(id,*n),vec![]);
        if let Some(e)=prior{if !e.external{changes.insert(reverse_key(&e.target,id,*n),vec![]);}}
    }
    let mut navigation:BTreeSet<String>=final_docs.keys().cloned().collect();
    for ((id,n),next) in &edges{if let Some(e)=next{
        let target=if e.external{None}else if let Some(t)=final_docs.get(&e.target){t.clone()}else{header(o,g,&e.target)?};
        edge_rule(e,target.as_ref())?;
        let b=objects::encode(e,o)?;changes.insert(edge_key(id,*n),b.clone());
        if !e.external{changes.insert(reverse_key(&e.target,id,*n),b);}
    }}
    for (target,next) in &final_docs{
        let prior=old[target].as_ref();
        let affect=match (prior,next.as_ref()){
            (Some(a),Some(b))=>a.doc.path!=b.doc.path||a.doc.kind!=b.doc.kind,
            (Some(_),None)=>true,_=>false
        };
        if affect{
            for (key,e) in objects::range::<Edge>(o,&g.tree,&format!("R/{target}/"))?{
                o.metrics.edges_visited+=1;
                let mut parts=key.split('/');parts.next();parts.next();
                let source=parts.next().ok_or("integrity:reverse")?.to_owned();
                let n=u64::from_str_radix(parts.next().ok_or("integrity:reverse-ordinal")?,16)?;
                if old_edges.contains_key(&(source.clone(),n)){continue;}
                let source_alive=if let Some(s)=final_docs.get(&source){s.is_some()}else{header(o,g,&source)?.is_some()};
                if source_alive{edge_rule(&e,next.as_ref())?;navigation.insert(source);}
            }
        }
    }
    let tree=objects::update(o,&g.tree,changes)?;
    let graph=GraphRoot{version:1,consumer:g.consumer.clone(),budget,cost:cost as u64,count,tree,fallback:g.fallback};
    if g.fallback{full_fallback(&all(o,&graph)?)?;}
    let mut navigation_facts=BTreeSet::new();
    for (id,next) in &final_docs{
        let same=match (old[id].as_ref(),next.as_ref()){
            (Some(a),Some(b))=>a.doc.id==b.doc.id&&a.doc.title==b.doc.title&&a.doc.kind==b.doc.kind&&a.doc.path==b.doc.path,
            (None,None)=>true,_=>false,
        };
        if !same{navigation_facts.insert(id.clone());}
    }
    for (key,prior) in &old_edges{
        if serde_json::to_vec(prior)?!=serde_json::to_vec(&edges[key])?{navigation_facts.insert(key.0.clone());}
    }
    let navigation_sources=final_docs.iter().filter_map(|(id,next)|match(old[id].as_ref(),next.as_ref()){
        (Some(a),Some(b)) if (a.doc.kind=="self-describing-json"||b.doc.kind=="self-describing-json")&&a.doc.digest!=b.doc.digest=>Some(id.clone()),_=>None
    }).collect();
    Ok(Delta{graph,changed:final_docs.keys().cloned().collect(),navigation,navigation_facts,navigation_sources,prior_graph:g.clone(),old,final_docs})
}
