//! Authenticated selected predecessor suffixes. This cursor is maintenance work,
//! never semantic acceptance or an ordinary request-history lookup index.
use crate::{io::{self,Metrics,Result},model::Selector,pack::{self,Key},store::{Store,Publication,Candidate},lifetime};
use serde::{Serialize,Deserialize};
use serde_json::Value;
use std::collections::{BTreeSet,BTreeMap};

const MAX_DEFERRED:usize=128;
const MAX_STATE:usize=32*1024;
const MAX_KEYS:usize=16*1024*1024;
#[derive(Clone,Serialize,Deserialize,PartialEq,Eq)]
#[serde(deny_unknown_fields)]
pub(crate) struct Cutoff{pub selected:Selector,pub sequence:u64}
#[derive(Clone,Serialize,Deserialize,PartialEq,Eq)]
#[serde(deny_unknown_fields)]
pub(crate) struct Interval{pub target:Cutoff,pub next:Option<Selector>,pub expected_sequence:u64}
#[derive(Clone,Serialize,Deserialize,PartialEq,Eq)]
#[serde(deny_unknown_fields)]
pub(crate) struct Deferred{pub publication:Cutoff,pub roles:u8,pub reason:String}
#[derive(Clone,Serialize,Deserialize,PartialEq,Eq)]
#[serde(deny_unknown_fields)]
pub(crate) struct Discovery{pub version:u32,pub completed:Option<Cutoff>,pub interval:Option<Interval>,pub deferred:Vec<Deferred>,pub include_attempts:bool}
fn valid_cutoff(c:&Cutoff)->bool{crate::model::safe_id(&c.selected.publication)&&crate::objects::valid_hash(&c.selected.digest)}
pub(crate) fn validate(d:&Discovery)->Result<()>{
    if d.version!=1||d.deferred.len()>MAX_DEFERRED||d.completed.as_ref().is_some_and(|c|!valid_cutoff(c))||d.interval.as_ref().is_some_and(|i|!valid_cutoff(&i.target)||i.next.as_ref().is_some_and(|s|!crate::model::safe_id(&s.publication)||!crate::objects::valid_hash(&s.digest))||i.expected_sequence>i.target.sequence||d.completed.as_ref().is_some_and(|c|c.sequence>i.target.sequence)){return Err("integrity:discovery-state".into());}
    let mut ids=BTreeSet::new();
    for item in &d.deferred{
        if !valid_cutoff(&item.publication)||item.roles==0||item.roles>31||!ids.insert(item.publication.selected.publication.clone())||!matches!(item.reason.as_str(),"reservation"|"retired"|"oversize"|"partial"){return Err("integrity:deferred-eligibility".into());}
    }
    if io::json(d)?.len()>MAX_STATE{return Err("quota:discovery-state-bytes".into());}Ok(())
}
fn keys(p:&Publication)->Result<[Key;5]>{Ok([
    Key::new(&format!("publications/{}.json",p.id))?,Key::new(&format!("results/{}.json",p.result_hash))?,Key::new(&format!("attempt-hints/{}.json",p.attempt))?,
    Key::new(&format!("attempts/{}/candidate.json",p.attempt))?,Key::new(&format!("attempts/{}/allocations/{}.json",p.attempt,p.id))?,
])}
fn insert(out:&mut BTreeSet<Key>,key:Key)->Result<()>{
    // At most five keys per publication and bounded publication chunks. Charge
    // conservative tree/string overhead before insertion, including duplicates.
    if out.len().checked_add(1).is_none_or(|n|n*768>MAX_KEYS){return Err("quota:discovery-key-bookkeeping".into());}
    out.insert(key);Ok(())
}
fn eligible(store:&Store,c:&Cutoff,p:&Publication,mask:u8,active:&BTreeMap<String,lifetime::Reservation>,out:&mut BTreeSet<Key>,m:&mut Metrics)->Result<Option<Deferred>>{
    let names=keys(p)?;let mut deferred=0u8;let mut reason="partial";
    if p.sequence!=c.sequence||p.selector()?!=c.selected||p.allocation_owner.attempt!=p.attempt||!p.allocation_owner.includes_intermediates_and_stages{return Err("integrity:discovery-publication-binding".into());}
    for (n,key) in names.into_iter().enumerate(){
        let bit=1u8<<n;if mask&bit==0{continue;}
        if n>=3{
            if active.contains_key(&p.attempt){deferred|=bit;reason="reservation";continue;}
            if lifetime::live(&store.root,&c.selected).is_err(){deferred|=bit;reason="retired";continue;}
        }
        if crate::packing::mapped(&key,m)?{continue;}
        let path=store.root.join(&key.0);
        // A reservation owns candidates/allocations until transfer completes.
        // No recursive attempt-directory discovery, including retry allocations.
        let raw=io::read(&path,m)?;
        if raw.len()>pack::MAX_RAW{deferred|=bit;reason="oversize";continue;}
        match n{
            0=>if io::digest(&raw)!=c.selected.digest{return Err("integrity:discovery-publication-digest".into());},
            1=>{if io::digest(&raw)!=p.result_hash{return Err("integrity:discovery-result-digest".into());}let _:Value=crate::physical_lifetime::strict(&raw)?;},
            2=>{let hint:Selector=crate::physical_lifetime::strict(&raw)?;if hint!=c.selected{return Err("integrity:discovery-hint".into());}},
            3=>{let candidate:Candidate=crate::physical_lifetime::strict(&raw)?;
                if candidate.version!=2||candidate.purpose!="publication"||candidate.store_id!=p.store_id||candidate.attempt!=p.attempt||candidate.request.request!=p.request||candidate.intent!=p.intent||candidate.expected!=p.predecessor{return Err("integrity:discovery-candidate".into());}},
            4=>{#[derive(Deserialize)] #[serde(deny_unknown_fields)] struct Allocation{publication:String,result:String}
                let a:Allocation=crate::physical_lifetime::strict(&raw)?;if a.publication!=p.id||a.result!=p.result_hash{return Err("integrity:discovery-allocation".into());}},
            _=>unreachable!(),
        }
        insert(out,key)?;
    }
    Ok(if deferred==0{None}else{Some(Deferred{publication:c.clone(),roles:deferred,reason:reason.into()})})
}
pub(crate) fn discover(store:&Store,prior:Option<Discovery>,v:&Value,m:&mut Metrics)->Result<(Discovery,BTreeSet<Key>)>{
    let bootstrap=prior.is_none();
    if bootstrap&&v["bootstrap"].as_bool()!=Some(true){return Err("refused:explicit-discovery-bootstrap-required".into());}
    let mut d=prior.unwrap_or(Discovery{version:1,completed:None,interval:None,deferred:vec![],include_attempts:v["include_attempts"].as_bool().unwrap_or(true)});
    validate(&d)?;
    if v["include_attempts"].as_bool().is_some_and(|b|b!=d.include_attempts){return Err("refused:discovery-cohort-change-requires-reconciliation".into());}
    m.rolling_bootstrap=bootstrap;
    let limit=v["discovery_limit"].as_u64().unwrap_or(4096);
    if limit==0||limit>4096{return Err("refused:discovery-chunk-bound".into());}
    let active=lifetime::reservations(&store.root,m)?;let mut out=BTreeSet::new();
    let deferred=std::mem::take(&mut d.deferred);
    for old in deferred{
        if crate::physical_lifetime::cancelled(){return Err("cancelled:rolling-discovery".into());}
        let p=store.publication(&old.publication.selected,m)?;m.rolling_deferred_visited+=1;
        if let Some(next)=eligible(store,&old.publication,&p,old.roles,&active,&mut out,m)?{d.deferred.push(next);}
    }
    if d.interval.is_none(){
        let h=store.snapshot(m)?;let target=Cutoff{selected:h.selected,sequence:h.publication.sequence};
        if d.completed.as_ref()==Some(&target){validate(&d)?;return Ok((d,out));}
        if d.completed.as_ref().is_some_and(|c|c.sequence>=target.sequence){return Err("integrity:discovery-non-descendant-head".into());}
        d.interval=Some(Interval{target:target.clone(),next:Some(target.selected.clone()),expected_sequence:target.sequence});
    }
    let mut count=0;
    loop{
        let i=d.interval.as_ref().ok_or("integrity:discovery-interval")?.clone();
        let reached=match (&i.next,&d.completed){(Some(next),Some(c))=>next==&c.selected&&(i.expected_sequence==c.sequence),(None,None)=>true,_=>false};
        if reached{d.completed=Some(i.target);d.interval=None;break;}
        if count>=limit{break;}
        let next=i.next.ok_or("integrity:discovery-cutoff-lineage")?;
        if d.completed.as_ref().is_some_and(|c|i.expected_sequence<=c.sequence){return Err("integrity:discovery-cutoff-lineage".into());}
        if crate::physical_lifetime::cancelled(){return Err("cancelled:rolling-discovery".into());}
        // Reserve one deferred slot before consuming a publication; a full set
        // applies explicit backpressure without advancing over an unknown hole.
        if d.deferred.len()>=MAX_DEFERRED{return Err("quota:deferred-eligibility-bound".into());}
        let p=store.publication(&next,m)?;m.rolling_publications_visited+=1;
        if p.sequence!=i.expected_sequence||p.sequence==0&&p.predecessor.is_some()||p.sequence>0&&p.predecessor.is_none(){return Err("integrity:discovery-predecessor-sequence".into());}
        let c=Cutoff{selected:next,sequence:p.sequence};let mask=if d.include_attempts{31}else{7};
        if let Some(item)=eligible(store,&c,&p,mask,&active,&mut out,m)?{d.deferred.push(item);}
        let interval=d.interval.as_mut().unwrap();interval.next=p.predecessor;interval.expected_sequence=p.sequence.saturating_sub(1);count+=1;
        io::checkpoint("rolling-discovery-publication")?;
    }
    validate(&d)?;io::checkpoint("rolling-discovery-complete")?;Ok((d,out))
}
