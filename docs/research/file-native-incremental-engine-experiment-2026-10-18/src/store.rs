//! I1: selected head direct, predecessor-only immutable history indexes.
use crate::{io::{self,Metrics,Result},objects::{self,Id},graph::{self,GraphRoot},model::{self,Selector,Request},lifetime,projection};
use serde::{Serialize,Deserialize};
use serde_json::{json,Value};
use fs2::FileExt;
use std::{collections::BTreeMap,fs::{self,File,OpenOptions},path::{Path,PathBuf},time::{Instant,Duration}};
#[derive(Clone,Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StoreConfig{pub version:u32,pub store_id:String,pub mode:String,pub consumer:String,pub fallback:bool}
pub struct Store{pub root:PathBuf,pub config:StoreConfig}
pub struct LockGuard{_file:File,started:Instant,counter:std::sync::Arc<std::sync::atomic::AtomicU64>}
impl Drop for LockGuard{
    fn drop(&mut self){self.counter.fetch_add(self.started.elapsed().as_nanos().min(u64::MAX as u128) as u64,std::sync::atomic::Ordering::Relaxed);}
}
#[derive(Clone,Serialize,Deserialize,PartialEq,Eq)]
#[serde(deny_unknown_fields)]
pub struct OutcomeRef{pub request:String,pub intent:String,pub selected:Selector,pub result_hash:Id,pub attempt:String}
#[derive(Clone,Serialize,Deserialize,PartialEq,Eq)]
#[serde(deny_unknown_fields)]
pub struct Checkpoint{pub source:Selector,pub coverage:u64,pub version:u32,pub membership:Option<Id>,pub outcomes:Option<Id>}
#[derive(Clone,Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Candidate{
    pub purpose:String,pub version:u32,pub store_id:String,pub attempt:String,pub expected:Option<Selector>,pub request:Request,pub intent:String,
    pub graph:GraphRoot,pub projection:Option<Id>,pub changes:Option<Id>,pub changed:Vec<String>,pub removed:Vec<String>,
    pub generation_manifest:Option<Id>,pub checkpoint:Option<Checkpoint>,
    pub packet_ref:Option<PacketRef>,
}
#[derive(Clone,Serialize,Deserialize,PartialEq,Eq)]
#[serde(deny_unknown_fields)]
pub struct PacketRef{pub attempt:String,pub manifest:Id,pub graph_digest:Id}
pub struct Prepared{candidate:Candidate}
pub enum Preparation{Existing(Value),Ready(Prepared)}
#[derive(Clone,Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Publication{
    pub version:u32,pub store_id:String,pub id:String,pub sequence:u64,pub predecessor:Option<Selector>,
    pub graph:GraphRoot,pub projection:Option<Id>,pub changes:Option<Id>,pub membership:Option<Id>,pub outcomes:Option<Id>,
    pub request:String,pub intent:String,pub result_hash:Id,pub attempt:String,pub generation_manifest:Option<Id>,
    pub allocation_owner:AllocationOwner,
    pub packet_ref:Option<PacketRef>,
}
#[derive(Clone,Serialize,Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AllocationOwner{pub attempt:String,pub conservative_bytes:u64,pub includes_intermediates_and_stages:bool}
impl Publication{
    pub fn selector(&self)->Result<Selector>{Ok(Selector{publication:self.id.clone(),digest:io::digest(&io::json(self)?)})}
    fn outcome_ref(&self)->Result<OutcomeRef>{Ok(OutcomeRef{request:self.request.clone(),intent:self.intent.clone(),selected:self.selector()?,result_hash:self.result_hash.clone(),attempt:self.attempt.clone()})}
}
pub struct Snapshot{pub selected:Selector,pub publication:Publication,_lease:Option<File>}
pub type Transaction=Request;
#[derive(Clone,Serialize,Deserialize)]
#[serde(tag="outcome",deny_unknown_fields)]
pub enum Resolution{
    #[serde(rename="accepted")]
    Accepted{request:String,publication:String,state_digest:String,changed:Vec<String>,removed:Vec<String>},
    #[serde(rename="absent-as-of")]
    AbsentAsOf{terminal_noncommit:bool},
    #[serde(rename="cancelled-fenced")]
    CancelledFenced{terminal_noncommit:bool},
}
impl Store{
    pub fn resolve_typed(&self,id:&str,m:&mut Metrics)->Result<Resolution>{
        Ok(serde_json::from_value(self.resolve(id,None,m)?)?)
    }
}
impl Store{
    pub fn open(root:&Path,m:&mut Metrics)->Result<Self>{
        m.store_root=Some(root.into());
        let config:StoreConfig=serde_json::from_slice(&io::read_bounded(&root.join("STORE.json"),65536,m)?)?;
        if config.version!=1||!["A","B"].contains(&config.mode.as_str())||!["project","research"].contains(&config.consumer.as_str()){
            return Err("integrity:store-version-kind".into());
        }
        Ok(Self{root:root.into(),config})
    }
    pub fn lock(&self,m:&mut Metrics)->Result<LockGuard>{
        m.store_root=Some(self.root.clone());
        let t=Instant::now();let f=OpenOptions::new().create(true).truncate(false).read(true).write(true).open(self.root.join("writer.lock"))?;
        loop{match f.try_lock_exclusive(){
            Ok(())=>break,Err(e)=>{
                if let Ok(id)=std::env::var("EXP_CANCEL_ID"){
                    if self.root.join(format!("cancel-signals/{}",io::digest(id.as_bytes()))).exists(){
                        m.elapsed("lock-wait",t);return Err("cancelled:while-waiting".into());
                    }
                }
                let cap=std::env::var("EXP_WAIT_MS").ok().and_then(|s|s.parse().ok()).unwrap_or(15000);
                if t.elapsed()>Duration::from_millis(cap){m.elapsed("lock-wait",t);return Err(format!("busy:lock-timeout:{e}").into());}
                std::thread::sleep(Duration::from_millis(2));
            }
        }}
        m.elapsed("lock-wait",t);io::checkpoint("lock-acquired")?;
        Ok(LockGuard{_file:f,started:Instant::now(),counter:m.lock_hold_counter.clone()})
    }
    pub fn preparation_gate(&self,exclusive:bool)->Result<File>{
        let f=OpenOptions::new().create(true).truncate(false).read(true).write(true).open(self.root.join("preparation.lock"))?;
        if exclusive{FileExt::try_lock_exclusive(&f).map_err(|e|format!("busy:active-preparation:{e}"))?;}
        else{FileExt::lock_shared(&f)?;}
        Ok(f)
    }
    fn selected(&self,m:&mut Metrics)->Result<Option<Selector>>{
        m.store_root=Some(self.root.clone());
        let p=self.root.join("SELECTED.json");
        if !p.exists(){return Ok(None);}
        Ok(Some(serde_json::from_slice(&io::read_bounded(&p,65536,m)?)?))
    }
    pub fn publication(&self,s:&Selector,m:&mut Metrics)->Result<Publication>{
        m.store_root=Some(self.root.clone());
        if !model::safe_id(&s.publication)||!objects::valid_hash(&s.digest){return Err("integrity:selector".into());}
        let p:Publication=serde_json::from_slice(&io::checked(&self.root.join(format!("publications/{}.json",s.publication)),&s.digest,m)?)?;
        if p.version!=1||p.store_id!=self.config.store_id||p.id!=s.publication||p.graph.version!=1{return Err("integrity:publication-version-binding".into());}
        Ok(p)
    }
    pub(crate) fn head_unleased(&self,m:&mut Metrics)->Result<Snapshot>{
        let selected=self.selected(m)?.ok_or("integrity:missing-selected")?;
        let publication=self.publication(&selected,m)?;
        let mut o=objects::object_store(&self.root,m);
        for root in [&publication.graph.tree,&publication.projection,&publication.membership,&publication.outcomes,&publication.changes]{
            crate::disk::DiskIndex::open(&mut o,root.clone())?;
        }
        if publication.sequence>0&&(publication.membership.is_none()||publication.outcomes.is_none()){return Err("integrity:history-coverage".into());}
        Ok(Snapshot{selected,publication,_lease:None})
    }
    pub fn snapshot(&self,m:&mut Metrics)->Result<Snapshot>{
        let lease=self.preparation_gate(false)?;
        let mut head=self.head_unleased(m)?;
        lifetime::live(&self.root,&head.selected)?;
        head._lease=Some(lease);Ok(head)
    }
    pub(crate) fn member(&self,s:&Selector,m:&mut Metrics)->Result<Publication>{
        let head=self.head_unleased(m)?;
        if head.selected!=*s{
            let mut o=objects::object_store(&self.root,m);
            let digest:Option<String>=objects::point(&mut o,&head.publication.membership,&s.publication)?;
            if digest.as_deref()!=Some(s.digest.as_str()){return Err("refused:not-selected-history".into());}
        }
        self.publication(s,m)
    }
    pub(crate) fn accepted(&self,id:&str,intent:Option<&str>,m:&mut Metrics)->Result<Option<OutcomeRef>>{
        let Some(sel)=self.selected(m)? else{return Ok(None)};
        let head=self.publication(&sel,m)?;
        if head.sequence>0&&(head.membership.is_none()||head.outcomes.is_none()){return Err("integrity:history-coverage".into());}
        {
            let mut o=objects::object_store(&self.root,m);
            crate::disk::DiskIndex::open(&mut o,head.membership.clone())?;
            crate::disk::DiskIndex::open(&mut o,head.outcomes.clone())?;
        }
        io::checkpoint("resolve-history-captured")?;
        let found=if head.request==id{Some(head.outcome_ref()?)}else{
            let mut o=objects::object_store(&self.root,m);
            let found:Option<OutcomeRef>=objects::point(&mut o,&head.outcomes,&io::digest(id.as_bytes()))?;
            if let Some(r)=&found{
                let digest:Option<String>=objects::point(&mut o,&head.membership,&r.selected.publication)?;
                if digest.as_deref()!=Some(r.selected.digest.as_str())||r.request!=id{return Err("integrity:outcome-membership".into());}
            }found
        };
        if found.as_ref().is_some_and(|r|intent.is_some_and(|s|s!=r.intent)){return Err("refused:request-id-intent-mismatch".into());}
        Ok(found)
    }
    fn result(&self,r:&OutcomeRef,m:&mut Metrics)->Result<Value>{
        let b=io::checked(&self.root.join(format!("results/{}.json",r.result_hash)),&r.result_hash,m)?;
        Ok(serde_json::from_slice(&b)?)
    }
    pub fn resolve(&self,id:&str,intent:Option<&str>,m:&mut Metrics)->Result<Value>{
        let _lease=self.preparation_gate(false)?;io::checkpoint("resolve-opened")?;
        if let Some(r)=self.accepted(id,intent,m)?{return self.result(&r,m);}
        let fence=self.root.join(format!("fences/{}",io::digest(id.as_bytes())));
        if fence.exists(){
            let bound:String=serde_json::from_slice(&io::read_bounded(&fence,65536,m)?)?;
            if bound!=id{return Err("integrity:fence-binding".into());}
            return Ok(json!({"outcome":"cancelled-fenced","terminal_noncommit":true}));
        }
        Ok(json!({"outcome":"absent-as-of","terminal_noncommit":false}))
    }
    pub(crate) fn membership_entries(&self,m:&mut Metrics)->Result<Vec<(String,Selector)>>{
        let head=self.head_unleased(m)?;let mut o=objects::object_store(&self.root,m);let mut out=vec![];
        for (id,b) in objects::entries(&mut o,&head.publication.membership)?{
            let digest:String=objects::decode(&b,&mut o)?;out.push((id.clone(),Selector{publication:id,digest}));
        }Ok(out)
    }
    fn predecessor_established(&self,m:&mut Metrics)->Result<()>{
        if self.selected(m)?.is_some(){io::sync_file(&self.root.join("SELECTED.json"),m)?;io::sync_dir(&self.root,m)?;}
        io::checkpoint("predecessor-established")
    }
    fn refuse_cancel(&self,id:&str,m:&mut Metrics)->Result<()>{
        if self.resolve(id,None,m)?["outcome"]=="cancelled-fenced"||self.root.join(format!("cancel-signals/{}",io::digest(id.as_bytes()))).exists(){
            return Err("refused:cancelled-fenced".into());
        }Ok(())
    }
    pub fn prepare(&self,r:Request,m:&mut Metrics)->Result<Preparation>{
        let t=Instant::now();let intent=io::digest(&io::json(&r)?);
        let _gate=self.preparation_gate(false)?;
        let guard=self.lock(m)?;let hold=Instant::now();self.predecessor_established(m)?;
        if let Some(found)=self.accepted(&r.request,Some(&intent),m)?{return Ok(Preparation::Existing(self.result(&found,m)?));}
        self.refuse_cancel(&r.request,m)?;
        lifetime::recover_transfers(self,m)?;
        let head=self.head_unleased(m)?;
        if r.base.as_ref().is_some_and(|s|s!=&head.selected){return Err("conflict:captured-base".into());}
        if r.base.is_none()&&r.ops.iter().any(|o|o.kind=="import"){return Err("refused:import-base-required".into());}
        if r.ops.len()>256||r.request.len()>256{return Err("refused:request-limit".into());}
        let attempt=io::unique("attempt");
        let per_document=u64::from(self.config.mode=="B")*300_000+u64::from(r.packet)*160_000;
        let allowance=head.publication.graph.count.checked_mul(per_document).ok_or("quota:overflow")?
            +64*1024*1024+r.ops.iter().map(|o|o.text.len() as u64*64).sum::<u64>();
        lifetime::reserve(&self.root,&attempt,lifetime::Reservation{request:r.request.clone(),intent:intent.clone(),base:Some(head.selected.clone()),bytes:allowance,state:"preparing".into(),used:0,candidate_digest:None},m)?;
        m.quota_limit=Some(allowance);m.quota_spent=0;
        m.elapsed("lock-hold",hold);drop(guard);io::checkpoint("allocation")?;
        let c=self.compute(&head,r,intent,attempt,m)?;
        self.save_candidate(&c,m)?;let _guard=self.lock(m)?;lifetime::mark_sealed(&self.root,&c.attempt,m)?;
        m.elapsed("preparation-inclusive",t);Ok(Preparation::Ready(Prepared{candidate:c}))
    }
    fn compute(&self,head:&Snapshot,r:Request,intent:String,attempt:String,m:&mut Metrics)->Result<Candidate>{
        let mut o=objects::object_store(&self.root,m);
        let graph_started=Instant::now();
        let delta=graph::transition(&self.root,&mut o,&head.publication.graph,&r.ops)?;
        o.metrics.elapsed("graph-transition-inclusive",graph_started);
        io::checkpoint("semantic-root")?;
        let projection_started=Instant::now();
        let (projection,changed_paths)=projection::delta(&self.root,&mut o,&head.publication.projection,&delta)?;
        o.metrics.elapsed("projection-prepare-inclusive",projection_started);
        io::checkpoint("projection-root")?;
        let mut changes=BTreeMap::new();
        for path in changed_paths{changes.insert(format!("{:020}/{path}",head.publication.sequence+1),serde_json::to_vec(&true)?);}
        let changes=objects::update(&mut o,&head.publication.changes,changes)?;
        drop(o);
        let removed=delta.old.iter().filter_map(|(id,h)|h.as_ref().filter(|old|delta.final_docs[id].as_ref().is_none_or(|n|n.doc.path!=old.doc.path)).map(|h|h.doc.path.clone())).collect();
        let mut c=Candidate{purpose:"publication".into(),version:1,store_id:self.config.store_id.clone(),attempt,expected:Some(head.selected.clone()),request:r,intent,
            graph:delta.graph,projection,changes,changed:delta.changed.into_iter().collect(),removed,generation_manifest:None,checkpoint:None,packet_ref:None};
        if c.request.packet{
            c.projection=projection::packet(self,&c.graph,&c.projection,&c.attempt,m)?;
            c.packet_ref=Some(PacketRef{attempt:c.attempt.clone(),manifest:io::digest(&io::read(&self.root.join(format!("attempts/{}/packet/MANIFEST.json",c.attempt)),m)?),graph_digest:io::digest(&io::json(&c.graph)?)});
            // Packet/provenance dependencies may change without document/path delta.
            let mut o=objects::object_store(&self.root,m);let mut updates=BTreeMap::new();
            for p in ["packet/context.md","packet/provenance.json"]{updates.insert(format!("{:020}/{p}",head.publication.sequence+1),serde_json::to_vec(&true)?);}
            c.changes=objects::update(&mut o,&c.changes,updates)?;
        }
        if self.config.mode=="B"{c.generation_manifest=Some(projection::generation(self,&c,m)?);}
        Ok(c)
    }
    fn save_candidate(&self,c:&Candidate,m:&mut Metrics)->Result<()>{
        io::write_new(&self.root.join(format!("attempts/{}/candidate.json",c.attempt)),&io::json(c)?,m)?;
        io::checkpoint("candidate-ready")
    }
    pub fn candidate_path(p:&Prepared)->String{format!("attempts/{}/candidate.json",p.candidate.attempt)}
    pub fn select_file(&self,path:&str,m:&mut Metrics)->Result<Value>{
        if Path::new(path).components().any(|c|!matches!(c,std::path::Component::Normal(_))){return Err("refused:candidate-path".into());}
        let parts:Vec<_>=path.split('/').collect();
        if parts.len()!=3||parts[0]!="attempts"||!model::safe_id(parts[1])||parts[2]!="candidate.json"{return Err("refused:candidate-path".into());}
        let guard=self.lock(m)?;let t=Instant::now();self.predecessor_established(m)?;
        let hint=self.root.join(format!("attempt-hints/{}.json",parts[1]));
        if hint.exists(){
            let selected:Selector=serde_json::from_slice(&io::read_bounded(&hint,65536,m)?)?;
            let p=self.publication(&selected,m)?;
            if p.attempt!=parts[1]{return Err("integrity:attempt-hint-binding".into());}
            if let Some(found)=self.accepted(&p.request,Some(&p.intent),m)?{
                if found.attempt==parts[1]{return self.result(&found,m);}
            }
        }
        let c:Candidate=serde_json::from_slice(&io::read(&self.root.join(path),m)?)?;
        if c.attempt!=parts[1]{return Err("integrity:candidate-path-binding".into());}
        if let Some(found)=self.accepted(&c.request.request,Some(&c.intent),m)?{return self.result(&found,m);}
        self.check_candidate(&c,m)?;
        let head=self.head_unleased(m)?;
        if c.expected.as_ref()!=Some(&head.selected){return Err("conflict:expected-publication".into());}
        if let Some(cp)=&c.checkpoint{
            let valid_op=c.request.ops.len()==1&&c.request.ops[0].kind=="checkpoint"&&c.request.ops[0].value==1&&c.request.ops[0].id.is_empty()&&c.request.ops[0].text.is_empty()&&c.request.ops[0].target.is_empty();
            if !valid_op||c.request.packet||cp.source!=head.selected||cp.coverage!=head.publication.sequence||cp.version!=1||
                c.graph!=head.publication.graph||c.projection!=head.publication.projection||c.changes!=head.publication.changes||
                c.packet_ref!=head.publication.packet_ref||!c.changed.is_empty()||!c.removed.is_empty(){
                return Err("integrity:checkpoint-replay-binding".into());
            }
            let mut o=objects::object_store(&self.root,m);
            for (prepared,selected) in [(&cp.membership,&head.publication.membership),(&cp.outcomes,&head.publication.outcomes)]{
                let a=objects::entries(&mut o,prepared)?;let b=objects::entries(&mut o,selected)?;
                fn decoded(o:&mut objects::Objects,entries:BTreeMap<String,Vec<u8>>)->Result<BTreeMap<String,Value>>{
                    entries.into_iter().map(|(k,b)|Ok((k,objects::decode(&b,o)?))).collect()
                }
                if decoded(&mut o,a)?!=decoded(&mut o,b)?{return Err("integrity:checkpoint-replay-indexes".into());}
            }
            drop(o);return self.select_locked(c,m);
        }
        let replay_start=Instant::now();
        m.verify_only=true;
        // Replay only pure graph/projection changes. A distinct scratch attempt
        // avoids trusting/replacing prepared B/packet destinations.
        let mut o=objects::object_store(&self.root,m);
        let d=graph::transition(&self.root,&mut o,&head.publication.graph,&c.request.ops)?;
        let (projection,paths)=projection::delta(&self.root,&mut o,&head.publication.projection,&d)?;
        let mut updates=BTreeMap::new();for p in paths{updates.insert(format!("{:020}/{p}",head.publication.sequence+1),serde_json::to_vec(&true)?);}
        let mut changes=objects::update(&mut o,&head.publication.changes,updates)?;
        drop(o);
        let mut expected_projection=projection.clone();
        if c.request.packet{
            expected_projection=projection::packet_verify_replay(self,&d.graph,&projection,&c.attempt,m)?;
            let mut o=objects::object_store(&self.root,m);let mut updates=BTreeMap::new();
            for p in ["packet/context.md","packet/provenance.json"]{updates.insert(format!("{:020}/{p}",head.publication.sequence+1),serde_json::to_vec(&true)?);}
            changes=objects::update(&mut o,&changes,updates)?;
        }
        if c.graph!=d.graph||c.projection!=expected_projection||c.changes!=changes||c.changed!=d.changed.into_iter().collect::<Vec<_>>(){
            return Err("integrity:candidate-replay".into());
        }
        let removed:Vec<_>=d.old.iter().filter_map(|(id,h)|h.as_ref().filter(|old|d.final_docs[id].as_ref().is_none_or(|n|n.doc.path!=old.doc.path)).map(|h|h.doc.path.clone())).collect();
        if c.removed!=removed{return Err("integrity:candidate-summary".into());}
        if c.request.packet!=c.packet_ref.is_some(){return Err("integrity:packet-ref-required".into());}
        if c.request.packet{
            let expected_packet=PacketRef{attempt:c.attempt.clone(),manifest:io::digest(&io::read(&self.root.join(format!("attempts/{}/packet/MANIFEST.json",c.attempt)),m)?),graph_digest:io::digest(&io::json(&d.graph)?)};
            if c.packet_ref.as_ref()!=Some(&expected_packet){return Err("integrity:packet-ref-replay".into());}
        }
        m.verify_only=false;
        m.elapsed("incremental-replay",replay_start);
        let out=self.select_locked(c,m);m.elapsed("lock-hold",t);drop(guard);out
    }
    fn check_candidate(&self,c:&Candidate,m:&mut Metrics)->Result<()>{
        if c.purpose!="publication"||c.version!=1||c.store_id!=self.config.store_id||!model::safe_id(&c.attempt)||io::digest(&io::json(&c.request)?)!=c.intent{return Err("integrity:candidate-version-intent".into());}
        if self.root.join(format!("abandoned/{}",c.attempt)).exists(){return Err("refused:abandoned".into());}
        let all=lifetime::reservations(&self.root,m)?;let r=all.get(&c.attempt).ok_or("refused:unreserved")?;
        if r.request!=c.request.request||r.intent!=c.intent||r.base!=c.expected||r.state!="sealed"||r.candidate_digest.as_deref()!=Some(io::digest(&io::read(&self.root.join(format!("attempts/{}/candidate.json",c.attempt)),m)?).as_str()){return Err("integrity:reservation-binding-or-unsealed".into());}
        m.quota_limit=Some(r.bytes);m.quota_spent=0;self.refuse_cancel(&c.request.request,m)
    }
    pub fn select_prepared(&self,p:Prepared,m:&mut Metrics)->Result<Value>{
        let _guard=self.lock(m)?;let t=Instant::now();
        let out=self.select_locked(p.candidate,m);m.elapsed("lock-hold",t);out
    }
    fn select_locked(&self,c:Candidate,m:&mut Metrics)->Result<Value>{
        let t=Instant::now();self.predecessor_established(m)?;
        if let Some(found)=self.accepted(&c.request.request,Some(&c.intent),m)?{return self.result(&found,m);}
        self.check_candidate(&c,m)?;
        let selected=self.selected(m)?;
        if selected!=c.expected{return Err("conflict:expected-publication".into());}
        io::checkpoint("expected-checked")?;
        lifetime::selection_grant(&self.root,&c.attempt,m)?;
        if self.config.mode=="B"{projection::verify_generation(self,&c,m)?;}
        if let Some(packet)=&c.packet_ref{projection::verify_capsule(self,&c.graph,packet,m)?;}
        let head=selected.as_ref().map(|s|self.publication(s,m)).transpose()?;
        let mut membership=head.as_ref().and_then(|p|p.membership.clone());
        let mut outcomes=head.as_ref().and_then(|p|p.outcomes.clone());
        if let Some(cp)=&c.checkpoint{
            if selected.as_ref()!=Some(&cp.source)||cp.version!=1||cp.coverage!=head.as_ref().unwrap().sequence{return Err("conflict:checkpoint-source".into());}
            membership=cp.membership.clone();outcomes=cp.outcomes.clone();
        }
        if let Some(p)=&head{
            let mut o=objects::object_store(&self.root,m);
            membership=objects::update(&mut o,&membership,BTreeMap::from([(p.id.clone(),serde_json::to_vec(&selected.as_ref().unwrap().digest)?)]))?;
            io::checkpoint("membership-root")?;
            let outcome_bytes=objects::encode(&p.outcome_ref()?,&mut o)?;
            outcomes=objects::update(&mut o,&outcomes,BTreeMap::from([(io::digest(p.request.as_bytes()),outcome_bytes)]))?;
            io::checkpoint("outcome-root")?;
        }
        let id=io::unique("publication");
        let result=json!({"outcome":"accepted","request":c.request.request,"publication":id,"state_digest":io::digest(&io::json(&c.graph)?),"changed":c.changed,"removed":c.removed});
        let result_bytes=io::json(&result)?;let result_hash=io::digest(&result_bytes);
        io::atomic(&self.root.join(format!("attempts/{}/allocations/{id}.json",c.attempt)),&io::json(&json!({"publication":id,"result":result_hash}))?,m)?;
        io::write_new(&self.root.join(format!("results/{result_hash}.json")),&result_bytes,m)?;io::checkpoint("result-installed")?;
        let p=Publication{version:1,store_id:self.config.store_id.clone(),id:id.clone(),sequence:head.as_ref().map_or(0,|p|p.sequence+1),predecessor:selected,
            graph:c.graph,projection:c.projection,changes:c.changes,membership,outcomes,request:c.request.request,intent:c.intent,result_hash,attempt:c.attempt.clone(),generation_manifest:c.generation_manifest,
            allocation_owner:AllocationOwner{attempt:c.attempt.clone(),conservative_bytes:lifetime::reservations(&self.root,m)?.get(&c.attempt).ok_or("integrity:allocation-owner")?.bytes,includes_intermediates_and_stages:true},packet_ref:c.packet_ref};
        let bytes=io::json(&p)?;let selected=p.selector()?;
        io::write_new(&self.root.join(format!("publications/{id}.json")),&bytes,m)?;io::checkpoint("publication-installed")?;
        io::atomic(&self.root.join(format!("attempt-hints/{}.json",c.attempt)),&io::json(&selected)?,m)?;io::checkpoint("attempt-hint")?;
        let stage=self.root.join(format!("attempts/{}/{}",c.attempt,io::unique("selector-stage")));io::write_new(&stage,&io::json(&selected)?,m)?;io::checkpoint("selector-staged")?;
        let mut boundary=false;
        let delivery=(||->Result<()>{
            io::replace(&stage,&self.root.join("SELECTED.json"),m)?;io::checkpoint("selector-visible")?;
            io::sync_dir(&self.root,m)?;boundary=true;io::checkpoint("selection-boundary")?;m.elapsed("selection-inclusive",t);
            lifetime::transfer(&self.root,&c.attempt,m)?;
            m.quota_limit=None;
            if self.config.mode=="A"&&std::env::var("EXP_DEFER_REFRESH").is_err(){projection::refresh(self,false,m)?;}
            io::checkpoint("reply")?;Ok(())
        })();
        Ok(json!({"status":if delivery.is_ok(){"published"}else if boundary{"published/delivery-pending"}else{"selection-uncertain"},
            "selected":selected,"result":if boundary{result}else{Value::Null},"delivery_error":delivery.err().map(|e|e.to_string()),
            "readable":if self.config.mode=="B"{format!("generations/{}",p.attempt)}else{"view".into()}}))
    }
    pub fn publish(&self,r:Request,m:&mut Metrics)->Result<Value>{
        match self.prepare(r,m)?{Preparation::Existing(result)=>Ok(json!({"status":"deduplicated","result":result})),Preparation::Ready(p)=>self.select_prepared(p,m)}
    }
    pub fn assemble_snapshot(&self,selected:&Selector,m:&mut Metrics)->Result<Value>{
        let _gate=self.preparation_gate(false)?;let _guard=self.lock(m)?;
        let p=self.member(selected,m)?;lifetime::live(&self.root,selected)?;
        let attempt=io::unique("packet");let request=Request{request:io::unique("assembly"),base:Some(selected.clone()),ops:vec![],packet:true};
        let intent=io::digest(&io::json(&request)?);
        let allowance=p.graph.count.checked_mul(160_000).ok_or("quota:overflow")?+64*1024*1024;
        lifetime::reserve(&self.root,&attempt,lifetime::Reservation{request:request.request.clone(),intent:intent.clone(),base:Some(selected.clone()),bytes:allowance,state:"preparing".into(),used:0,candidate_digest:None},m)?;
        m.quota_limit=Some(allowance);m.quota_spent=0;
        let projection=projection::packet(self,&p.graph,&p.projection,&attempt,m)?;
        let manifest=io::digest(&io::read(&self.root.join(format!("attempts/{attempt}/packet/MANIFEST.json")),m)?);
        let packet_ref=PacketRef{attempt:attempt.clone(),manifest:manifest.clone(),graph_digest:io::digest(&io::json(&p.graph)?)};
        let c=Candidate{purpose:"packet".into(),version:1,store_id:self.config.store_id.clone(),attempt:attempt.clone(),expected:Some(selected.clone()),request,intent,
            graph:p.graph,projection,changes:None,changed:vec![],removed:vec![],generation_manifest:None,checkpoint:None,packet_ref:Some(packet_ref)};
        self.save_candidate(&c,m)?;lifetime::mark_sealed(&self.root,&attempt,m)?;
        Ok(json!({"path":format!("attempts/{attempt}/packet"),"candidate":format!("attempts/{attempt}/candidate.json"),"selected":selected,
            "manifest_digest":manifest,"selection_unchanged":true}))
    }
    pub fn init(root:&Path,mode:&str,consumer:&str,n:usize,fallback:bool,m:&mut Metrics)->Result<Value>{
        m.store_root=Some(root.into());
        if root.exists()||!["A","B"].contains(&mode)||!["project","research"].contains(&consumer)||n<10||n>100_000{return Err("refused:init-arguments-existing".into());}
        io::mkdir(root,m)?;
        let config=StoreConfig{version:1,store_id:io::unique("store"),mode:mode.into(),consumer:consumer.into(),fallback};
        io::write_new(&root.join("STORE.json"),&io::json(&config)?,m)?;
        io::write_new(&root.join("README.md"),b"# File-native collection\n\nRead SELECTED.json once and verify its publication digest. The publication binds exact immutable state. B: retain generations/<attempt>/ once and read NAVIGATION.md. A: view/NAVIGATION.md may mix/lag; VIEW.json names completed state and PENDING.json records unfinished work. Ordinary edits are not automatically admitted.\n",m)?;
        let store=Self{root:root.into(),config};let _guard=store.lock(m)?;
        let attempt=io::unique("attempt");let request=Request{request:"initial".into(),base:None,ops:vec![],packet:false};let intent=io::digest(&io::json(&request)?);
        let allowance=n as u64*300_000+64*1024*1024;
        lifetime::reserve(root,&attempt,lifetime::Reservation{request:"initial".into(),intent:intent.clone(),base:None,bytes:allowance,state:"preparing".into(),used:0,candidate_digest:None},m)?;
        m.quota_limit=Some(allowance);m.quota_spent=0;
        let s=model::fixture(root,consumer,n,m)?;let mut o=objects::object_store(root,m);
        let graph=graph::build(&mut o,&s,fallback)?;io::checkpoint("semantic-root")?;
        let projection=projection::full(root,&mut o,&graph)?;
        io::checkpoint("projection-root")?;drop(o);
        let mut c=Candidate{purpose:"publication".into(),version:1,store_id:store.config.store_id.clone(),attempt,expected:None,request,intent,graph,projection,changes:None,
            changed:s.docs.keys().cloned().collect(),removed:vec![],generation_manifest:None,checkpoint:None,packet_ref:None};
        if mode=="B"{c.generation_manifest=Some(projection::generation(&store,&c,m)?);}
        store.save_candidate(&c,m)?;lifetime::mark_sealed(root,&c.attempt,m)?;store.select_locked(c,m)
    }
    pub fn checkpoint(&self,id:&str,m:&mut Metrics)->Result<Value>{
        let _guard=self.lock(m)?;self.predecessor_established(m)?;
        let head=self.snapshot(m)?;
        let request=Request{request:id.into(),base:None,ops:vec![model::Op{kind:"checkpoint".into(),id:String::new(),text:String::new(),target:String::new(),value:1}],packet:false};
        let intent=io::digest(&io::json(&request)?);
        if let Some(found)=self.accepted(id,Some(&intent),m)?{return self.result(&found,m);}
        let attempt=io::unique("checkpoint");let allowance=256*1024*1024;
        lifetime::reserve(&self.root,&attempt,lifetime::Reservation{request:id.into(),intent:intent.clone(),base:Some(head.selected.clone()),bytes:allowance,state:"preparing".into(),used:0,candidate_digest:None},m)?;
        m.quota_limit=Some(allowance);m.quota_spent=0;
        let mut members=BTreeMap::new();let mut outcomes=BTreeMap::new();let mut next=head.publication.predecessor.clone();let mut count=0;
        while let Some(s)=next{
            count+=1;if count>200_000{return Err("refused:reference-walker-cap".into());}
            let p=self.publication(&s,m)?;next=p.predecessor.clone();
            if members.insert(p.id.clone(),serde_json::to_vec(&s.digest)?).is_some(){return Err("integrity:history-cycle".into());}
            outcomes.insert(io::digest(p.request.as_bytes()),serde_json::to_vec(&p.outcome_ref()?)?);
        }
        let mut o=objects::object_store(&self.root,m);
        let decoded_outcomes: BTreeMap<_,_>=objects::entries(&mut o,&head.publication.outcomes)?.into_iter().map(|(k,b)|Ok((k,serde_json::to_vec(&objects::decode::<OutcomeRef>(&b,&mut o)?)?))).collect::<Result<_>>()?;
        if members!=objects::entries(&mut o,&head.publication.membership)?||outcomes!=decoded_outcomes||count!=head.publication.sequence{
            return Err("integrity:checkpoint-coverage".into());
        }
        let membership=objects::bulk(&mut o,members)?;let outcomes=objects::bulk(&mut o,outcomes)?;
        io::checkpoint("checkpoint-built")?;io::checkpoint("checkpoint-verified")?;drop(o);
        let mut c=Candidate{purpose:"publication".into(),version:1,store_id:self.config.store_id.clone(),attempt,expected:Some(head.selected.clone()),request,intent,
            graph:head.publication.graph.clone(),projection:head.publication.projection.clone(),changes:head.publication.changes.clone(),
            changed:vec![],removed:vec![],generation_manifest:head.publication.generation_manifest.clone(),
            checkpoint:Some(Checkpoint{source:head.selected,coverage:count,version:1,membership,outcomes}),packet_ref:head.publication.packet_ref.clone()};
        if self.config.mode=="B"{c.generation_manifest=Some(projection::generation(self,&c,m)?);}
        self.save_candidate(&c,m)?;lifetime::mark_sealed(&self.root,&c.attempt,m)?;self.select_locked(c,m)
    }
    pub fn cancel(&self,id:&str,m:&mut Metrics)->Result<Value>{
        let _guard=self.lock(m)?;self.predecessor_established(m)?;
        if let Some(found)=self.accepted(id,None,m)?{return self.result(&found,m);}
        io::atomic(&self.root.join(format!("fences/{}",io::digest(id.as_bytes()))),&io::json(&id)?,m)?;
        io::checkpoint("cancel-durable")?;self.resolve(id,None,m)
    }
}
