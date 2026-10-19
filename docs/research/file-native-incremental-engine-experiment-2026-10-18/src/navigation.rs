//! Profile 4: readable groups, binary stable-ID routing and local split/merge.
//! This is an experiment-private projection, never source or graph authority.
use crate::{graph::{self,Delta,GraphRoot,Header},io::{self,Result},objects::{self,Id,Objects},projection::{self,FileSpec},model::Edge};
use serde::Deserialize;
use std::{collections::{BTreeMap,BTreeSet},path::Path};
const RECORDS:usize=512;
const BYTES:usize=256*1024;
const INLINE_EDGES:usize=128;
const EDGE_RECORDS:usize=256;
const DESCRIPTION:&[u8]=b"version: 4\nlayout: stable-id-sha256-binary-prefix\ngroups: navigation/groups/\nrelationships: navigation/relationships/\nmaximum_group_records: 512\nmaximum_part_bytes: 262144\nsource_complete_kind: self-describing-json\nsource_complete_rule: exact id, title, kind and ordered relationships in source JSON; location is its ordinary tree path\n";
struct Record{id:String,body:String,adj:Vec<(String,String)>}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CompleteSource{id:String,title:String,kind:String,relationships:Vec<Edge>}
fn complete(root:&Path,o:&mut Objects,h:&Header,edges:&[(u64,Edge)])->Result<bool>{
    if h.doc.kind!="self-describing-json"{return Ok(false);}
    let b=io::checked(&root.join(&h.doc.original),&h.doc.digest,o.metrics)?;
    let parsed=std::str::from_utf8(&b).ok().and_then(|s|crate::api::strict_json(s).ok()).and_then(|v|serde_json::from_value::<CompleteSource>(v).ok());
    let Some(s)=parsed else{return Ok(false)};
    let actual:Vec<_>=edges.iter().map(|(_,e)|e).collect();
    Ok(s.id==h.doc.id&&s.title==h.doc.title&&s.kind==h.doc.kind&&serde_json::to_vec(&s.relationships)?==serde_json::to_vec(&actual)?)
}
fn bits(id:&str)->String{io::digest(id.as_bytes()).bytes().flat_map(|b|{let v=if b<=b'9'{b-b'0'}else{b-b'a'+10};(0..4).rev().map(move|n|if v&(1<<n)==0{'0'}else{'1'})}).collect()}
fn path(prefix:&str)->String{format!("navigation/groups/{}.yaml",if prefix.is_empty(){"root"}else{prefix})}
fn heading(prefix:&str)->String{format!("version: 4\nstable_id_sha256_prefix: {}\ndocuments:\n",serde_json::to_string(prefix).unwrap())}
fn fits(prefix:&str,rows:&[Record])->bool{rows.len()<=RECORDS&&heading(prefix).len()+rows.iter().map(|r|r.body.len()).sum::<usize>()<=BYTES}
fn edge(n:u64,e:&Edge,indent:usize)->Result<String>{
    let p=" ".repeat(indent);Ok(format!("{p}- ordinal: {n}\n{p}  kind: {}\n{p}  target: {}\n{p}  external: {}\n",serde_json::to_string(&e.kind)?,serde_json::to_string(&e.target)?,e.external))
}
fn record(root:&Path,o:&mut Objects,g:&GraphRoot,h:Header)->Result<Option<Record>>{
    let edges=graph::outgoing(o,g,&h.doc.id)?;
    if complete(root,o,&h,&edges)?{return Ok(None);}
    let d=h.doc;let mut body=format!("  - id: {}\n    title: {}\n    kind: {}\n    source: {}\n",serde_json::to_string(&d.id)?,serde_json::to_string(&d.title)?,serde_json::to_string(&d.kind)?,serde_json::to_string(&d.path)?);
    let inline:Vec<_>=edges.iter().map(|(n,e)|edge(*n,e,6)).collect::<Result<_>>()?;
    let mut adj=vec![];
    if edges.len()<=INLINE_EDGES&&body.len()+inline.iter().map(String::len).sum::<usize>()<BYTES/2{
        if edges.is_empty(){body.push_str("    relationships: []\n");}else{body.push_str("    relationships:\n");for row in inline{body.push_str(&row);}}
    }else{
        let mut chunk=String::new();let mut count=0;let mut number=0;
        for (n,e) in &edges{
            let row=edge(*n,e,2)?;
            let header=format!("version: 4\nsource_id: {}\nrelationships:\n",serde_json::to_string(&d.id)?);
            if count>0&&(count==EDGE_RECORDS||chunk.len()+row.len()>BYTES){
                let p=format!("navigation/relationships/{}/{number:06}.yaml",d.id);adj.push((p,chunk));number+=1;chunk=String::new();count=0;
            }
            if count==0{chunk.push_str(&header);}chunk.push_str(&row);count+=1;
        }
        if count>0{let p=format!("navigation/relationships/{}/{number:06}.yaml",d.id);adj.push((p,chunk));number+=1;}
        body.push_str(&format!("    relationships_directory: {}\n    relationships_parts: {number}\n",serde_json::to_string(&format!("navigation/relationships/{}/",d.id))?));
    }
    if body.len()+heading("").len()>BYTES{return Err("navigation:single-document-part-bound".into());}
    Ok(Some(Record{id:d.id,body,adj}))
}
fn emit(root:&Path,o:&mut Objects,out:&mut BTreeMap<String,Vec<u8>>,name:String,b:&str)->Result<()>{
    if b.len()>BYTES{
        if !name.starts_with("navigation/relationships/")||b.len()>crate::MAX_OBJECT+1024{return Err("navigation:part-byte-bound".into());}
        o.metrics.navigation_oversize_relationship_parts+=1;
    }
    o.metrics.navigation_parts+=1;o.metrics.navigation_bytes+=b.len() as u64;
    let f=projection::derived(root,b.as_bytes(),o.metrics)?;out.insert(name,objects::encode(&f,o)?);Ok(())
}
fn split(root:&Path,o:&mut Objects,out:&mut BTreeMap<String,Vec<u8>>,prefix:&str,mut rows:Vec<Record>)->Result<()>{
    rows.sort_by(|a,b|a.id.cmp(&b.id));
    if fits(prefix,&rows){
        let mut b=heading(prefix);for r in rows{b.push_str(&r.body);for (p,s) in r.adj{emit(root,o,out,p,&s)?;}}
        return emit(root,o,out,path(prefix),&b);
    }
    if prefix.len()>=256{return Err("navigation:hash-collision-bound".into());}
    let (left,right)=rows.into_iter().partition(|r|bits(&r.id).as_bytes()[prefix.len()]==b'0');
    split(root,o,out,&format!("{prefix}0"),left)?;split(root,o,out,&format!("{prefix}1"),right)
}
pub(crate) fn full(root:&Path,o:&mut Objects,g:&GraphRoot)->Result<BTreeMap<String,Vec<u8>>>{
    let mut rows=vec![];for (_,h) in objects::range::<Header>(o,&g.tree,"D/")?{if let Some(r)=record(root,o,g,h)?{rows.push(r);}}
    let mut out=BTreeMap::new();if !rows.is_empty(){split(root,o,&mut out,"",rows)?;emit(root,o,&mut out,"navigation.yaml".into(),std::str::from_utf8(DESCRIPTION)?)?;}Ok(out)
}
fn spec(o:&mut Objects,old:&Option<Id>,out:&BTreeMap<String,Vec<u8>>,p:&str)->Result<Option<FileSpec>>{
    if let Some(b)=out.get(p){return if b.is_empty(){Ok(None)}else{Ok(Some(objects::decode(b,o)?))};}objects::point(o,old,p)
}
fn ids(root:&Path,o:&mut Objects,f:&FileSpec,out:&mut BTreeMap<String,Vec<u8>>)->Result<BTreeSet<String>>{
    let b=io::checked(&root.join(&f.source),&f.digest,o.metrics)?;let mut ids=BTreeSet::new();let mut directory=None;
    for line in std::str::from_utf8(&b)?.lines(){
        if let Some(s)=line.strip_prefix("  - id: "){if !ids.insert(serde_json::from_str(s)?){return Err("integrity:navigation-duplicate-id".into());}}
        if let Some(s)=line.strip_prefix("    relationships_directory: "){directory=Some(serde_json::from_str::<String>(s)?);}
        if let Some(s)=line.strip_prefix("    relationships_parts: "){
            let p=directory.take().ok_or("integrity:navigation-adjacency-directory")?;let count:usize=serde_json::from_str(s)?;
            if !p.starts_with("navigation/relationships/")||count>1_000_000{return Err("integrity:navigation-adjacency-path".into());}
            for n in 0..count{out.insert(format!("{p}{n:06}.yaml"),vec![]);}
        }
    }Ok(ids)
}
fn records(root:&Path,o:&mut Objects,g:&GraphRoot,members:&BTreeSet<String>)->Result<Vec<Record>>{
    let mut rows=vec![];for id in members{if let Some(h)=graph::header(o,g,id)?{if let Some(r)=record(root,o,g,h)?{rows.push(r);}}}Ok(rows)
}
pub(crate) fn delta(root:&Path,o:&mut Objects,old:&Option<Id>,d:&Delta)->Result<BTreeMap<String,Vec<u8>>>{
    let mut out=BTreeMap::new();let mut covered=BTreeSet::new();let mut dirty=BTreeSet::new();
    for id in d.navigation_facts.union(&d.navigation_sources){
        let prior=d.old.get(id).and_then(Option::as_ref);let next=d.final_docs.get(id).and_then(Option::as_ref);
        let native=prior.is_some_and(|h|h.doc.kind=="self-describing-json")||next.is_some_and(|h|h.doc.kind=="self-describing-json");
        if native{
            let old_complete=if let Some(h)=prior{if h.doc.kind=="self-describing-json"{let edges=graph::outgoing(o,&d.prior_graph,id)?;complete(root,o,h,&edges)?}else{false}}else{false};
            let new_complete=if let Some(h)=next{if h.doc.kind=="self-describing-json"{let edges=graph::outgoing(o,&d.graph,id)?;complete(root,o,h,&edges)?}else{false}}else{false};
            let old_supplement=prior.is_some()&&!old_complete;let new_supplement=next.is_some()&&!new_complete;
            if !old_supplement&&!new_supplement{continue;}
            if old_supplement==new_supplement&&!d.navigation_facts.contains(id){continue;}
        }
        dirty.insert(id.clone());
    }
    for id in &dirty{
        if covered.contains(id){continue;}
        let route=bits(id);let mut prefix=String::new();let mut members=BTreeSet::new();
        if spec(o,old,&out,"navigation.yaml")?.is_some(){
            let mut found=false;for n in 0..=256{let p=&route[..n];if let Some(f)=spec(o,old,&out,&path(p))?{prefix=p.into();members=ids(root,o,&f,&mut out)?;out.insert(path(p),vec![]);found=true;break;}}
            if !found{return Err("integrity:navigation-routing".into());}
        }
        members.insert(id.clone());let mut rows=records(root,o,&d.graph,&members)?;
        // A branch sibling cannot fit: it was split by these same monotone
        // record/byte bounds. Only two leaf siblings can merge locally.
        while !prefix.is_empty(){
            let parent=&prefix[..prefix.len()-1];let sibling=format!("{parent}{}",if prefix.ends_with('0'){'1'}else{'0'});
            let Some(f)=spec(o,old,&out,&path(&sibling))? else{break};
            let mut scratch=BTreeMap::new();let sibling_ids=ids(root,o,&f,&mut scratch)?;let mut joined=members.clone();joined.extend(sibling_ids);
            let joined_rows=records(root,o,&d.graph,&joined)?;
            if !fits(parent,&joined_rows){break;}
            out.extend(scratch);out.insert(path(&sibling),vec![]);members=joined;rows=joined_rows;prefix=parent.into();
        }
        covered.extend(members);
        if prefix.is_empty()&&rows.is_empty(){out.insert(path(""),vec![]);out.insert("navigation.yaml".into(),vec![]);}
        else{split(root,o,&mut out,&prefix,rows)?;emit(root,o,&mut out,"navigation.yaml".into(),std::str::from_utf8(DESCRIPTION)?)?;}
    }Ok(out)
}
