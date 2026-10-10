//! Native scratch stores only; no copied physical authority and no cap changes.
#[allow(dead_code, unused_imports)]
#[path = "../../../../../crates/sdk/tests/work_kind_support/mod.rs"]
mod support;
use handbook_engine::{research_profile as prof, work_state::NativeWorkStateV1 as Native};
use handbook_sdk::{HandbookSdkV1, work_state::*};
use serde_json::json;
use std::{fs,path::Path,time::Instant};

fn mode(v:&str) { std::env::set_var("HCM_RESEARCH_VARIANT",v); }
fn bytes(p:&Path)->u64 { if p.is_file(){return fs::metadata(p).unwrap().len();}
    fs::read_dir(p).unwrap().map(|e|bytes(&e.unwrap().path())).sum() }
fn input(g:&WorkGrantLocatorV1,key:&str,kind:Option<&WorkKindBindingV1>)->WorkRegistrationInputV1 {
    WorkRegistrationInputV1 {request_key:key.into(),item_key:key.into(),label:"Leaf".into(),
        question:if key.len()%2==0 {"Exact bounded work question".into()} else {"q".repeat(256)},
        actor:g.terms.agent.clone(),expected_state:"absent".into(),
        composition:kind.map_or_else(||WorkCompositionV1::ordinary_leaf().into(),|k|WorkCompositionInputV1::kind(k.clone(),None)),display_level:None}
}
fn timed<T>(cohort:&str,op:&str,history:usize,sample:usize,v:&str,on:bool,b:u64,f:impl FnOnce()->T)->T {
    mode(v);if on {prof::begin();}
    let before=prof::handles();let cpu=prof::cpu_ms();let start=Instant::now();let result=f();
    let wall=start.elapsed().as_secs_f64()*1000.0;let used=prof::cpu_ms()-cpu;
    let profile=if on {prof::finish()} else {json!(null)};
    println!("{}",json!({"cohort":cohort,"operation":op,"history_slots_at_entry":history,"sample":sample,
        "variant":v,"instrumented":on,"wall_ms":wall,"cpu_ms":used,"retained_work_bytes":b,
        "handles_before":before,"handles_after":prof::handles(),"profile":profile}));
    result
}
fn reads() {
    for cohort in ["explicit","kind-DG"] {
        mode("combined");let dir=tempfile::Builder::new().prefix("hcm-native-opt-").tempdir().unwrap();
        let sdk=HandbookSdkV1::open(dir.path());let (initial,check)=support::setup(&sdk);
        let mut g=support::activate(&sdk,&initial,&check,"grant",None);let mut n=1;
        let mut kinds=Vec::new();let mut first=None;
        if cohort=="kind-DG" {kinds.push(support::establish_kind(&sdk,&initial,&check,"kind-a","team.leaf@1.0.0"));n+=1;}
        let targets=if cohort=="explicit"{vec![8,16,31,32]}else{vec![16,32]};
        for target in targets {
            while n<target {
                mode("combined");
                if cohort=="kind-DG" && n==8 {
                    support::govern(&sdk,&initial);g=support::activate(&sdk,&initial,&check,"grant-G",Some(&g));n+=1;
                    kinds.push(support::establish_kind(&sdk,&initial,&check,"kind-b","team.leaf@2.0.0"));n+=1;continue;
                }
                let key=format!("item-{n:02}");let k=if kinds.is_empty(){None}else{Some(&kinds[n%kinds.len()])};
                let p=sdk.prepare_work_registration(&initial,&g,input(&g,&key,k)).unwrap();
                first.get_or_insert(p.locator().clone());
                let r=sdk.register_work_item(p);assert_eq!(r.disposition,WorkDispositionV1::Committed,"{r:?}");n+=1;
            }
            let b=bytes(&dir.path().join(".handbook/work-state"));let original=first.as_ref().unwrap();
            let variants=["base","owner","binding","semantic","combined","direct","combined-direct"];
            for sample in 0..8 {
                let on=sample%2==1;
                for ix in 0..variants.len() {
                    let v=variants[(ix+sample)%variants.len()];
                    // G current state belongs to newest grant; historical original uses D.
                    let r=timed(cohort,"current_item",n,sample,v,on,b,||Native::current_item(dir.path(),&g,&original.input.item_key));
                    assert!(matches!(r.unwrap(),WorkCurrentItemV1::Open(_)));
                }
            }
            if target==31 || (cohort=="kind-DG" && target==16) {
                for sample in 0..6 {for ix in 0..variants.len() {
                    let v=variants[(ix+sample)%variants.len()];
                    let r=timed(cohort,"preparation",n,sample,v,false,b,||sdk.prepare_work_registration(&initial,&g,input(&g,"next-item",kinds.last())));
                    assert!(r.is_ok(),"{r:?}");
                }}
            }
            if target==32 {
                for sample in 0..6 {for v in ["base","combined","combined-direct"] {
                    let r=timed(cohort,"historical_resolve",n,sample,v,false,b,||sdk.resolve_work_registration_request(original));
                    assert_eq!(r.disposition,WorkDispositionV1::Replayed);
                    let r=timed(cohort,"committed_recovery",n,sample,v,false,b,||sdk.recover_work_registration_request(original));
                    assert_eq!(r.disposition,WorkDispositionV1::Replayed);
                    let r=timed(cohort,"current_grant",n,sample,v,false,b,||sdk.resolve_current_work_registration_terms(&initial,&g));
                    assert!(r.is_ok(),"{r:?}");
                    let r=timed(cohort,"governance_basis",n,sample,v,false,b,||sdk.observe_local_check_basis(&initial));
                    assert!(r.is_ok());
                }}
            }
        }
        println!("{}",json!({"event":"cohort_complete","cohort":cohort,"history":n,"kind_versions":kinds.len(),
            "native_proof_populations":if cohort=="kind-DG"{2}else{1},"fixed_selected_item":first.unwrap().input.item_key}));
    }
}
fn effects() {
    let variants=["base","semantic","binding","combined","combined-direct"];
    let mut stores=Vec::new();
    for v in variants {
        mode("combined");let dir=tempfile::Builder::new().prefix("hcm-native-effects-").tempdir().unwrap();
        let sdk=HandbookSdkV1::open(dir.path());let(initial,check)=support::setup(&sdk);
        let g=support::activate(&sdk,&initial,&check,"grant",None);
        for n in 1..25 {let p=sdk.prepare_work_registration(&initial,&g,input(&g,&format!("item-{n:02}"),None)).unwrap();
            assert_eq!(sdk.register_work_item(p).disposition,WorkDispositionV1::Committed);}
        stores.push((v,dir,sdk,initial,g));
    }
    for sample in 0..7 {
        for ix in 0..stores.len() {
            let selected=if sample%2==0{(ix+sample)%stores.len()}else{(stores.len()-1-ix+sample)%stores.len()};
            let(v,dir,sdk,initial,g)=&stores[selected];let n=25+sample;
            let b=bytes(&dir.path().join(".handbook/work-state"));let key=format!("effect-{sample}");
            let p=timed("explicit","fresh_preparation",n,sample,v,sample==6,b,||sdk.prepare_work_registration(initial,g,input(g,&key,None))).unwrap();
            let original=p.locator().clone();let before=prof::handles();let mut handle_max=before;
            let r=timed("explicit","fresh_invocation",n,sample,v,sample==6,b,||Native::register_with_boundary_for_test(dir.path(),p,&mut |_|{
                handle_max=handle_max.max(prof::handles());Ok(())}));
            assert_eq!(r.disposition,WorkDispositionV1::Committed,"{r:?}");
            assert_eq!(r.outcome.unwrap().locator,original);
            println!("{}",json!({"event":"boundary_handles","variant":v,"history":n,"sampled_max":handle_max}));
        }
    }
    for (v,dir,sdk,_,g) in &stores {
        mode(v);
        let request=sdk.prepare_work_registration_revocation(g,"revoke-full",&g.terms.revocation_principal,"human.revoke","Revoke original registration permission").unwrap();
        let b=bytes(&dir.path().join(".handbook/work-state"));
        let r=timed("explicit","saturated_revocation",32,0,v,false,b,||sdk.revoke_work_registration_terms(&request));
        assert_eq!(r.disposition,WorkDispositionV1::Refused,"{r:?}");
        println!("{}",json!({"event":"saturated_revocation_refusal","variant":v,"result":format!("{r:?}")}));
    }
    println!("{}",json!({"event":"effects_complete","slots":32,"variants":variants}));
}
fn pending() {
    for v in ["base","combined"] {
        mode("combined");let dir=tempfile::Builder::new().prefix("hcm-native-pending-").tempdir().unwrap();
        let sdk=HandbookSdkV1::open(dir.path());let(initial,check)=support::setup(&sdk);
        let g=support::activate(&sdk,&initial,&check,"grant",None);
        for n in 1..31 {let p=sdk.prepare_work_registration(&initial,&g,input(&g,&format!("item-{n:02}"),None)).unwrap();
            assert_eq!(sdk.register_work_item(p).disposition,WorkDispositionV1::Committed);}
        mode(v);let p=sdk.prepare_work_registration(&initial,&g,input(&g,"accepted-pending",None)).unwrap();
        let original=p.locator().clone();
        let r=Native::register_with_boundary_for_test(dir.path(),p,|stage|{
            if stage==handbook_engine::work_state::WorkWriteBoundaryV1::Snapshot{Err(support::interruption())}else{Ok(())}});
        assert_eq!(r.disposition,WorkDispositionV1::Uncertain);
        let b=bytes(&dir.path().join(".handbook/work-state"));
        let r=timed("explicit","accepted_pending_recovery",32,0,v,true,b,||sdk.recover_work_registration_request(&original));
        assert_eq!(r.disposition,WorkDispositionV1::Committed,"{r:?}");assert_eq!(r.outcome.unwrap().locator,original);
    }
    println!("{}",json!({"event":"pending_complete","slots":32,"samples_per_variant":1}));
}
fn probes() {
    let dir=tempfile::tempdir().unwrap();fs::create_dir_all(dir.path().join(".handbook/selected")).unwrap();
    fs::write(dir.path().join(".handbook/selected/value.json"),b"exact").unwrap();
    fs::write(dir.path().join(".handbook/unrelated.json"),b"other").unwrap();
    fs::hard_link(dir.path().join(".handbook/unrelated.json"),dir.path().join("alias.json")).unwrap();
    mode("base");let guarded=prof::probe_optional(dir.path(),".handbook/selected/value.json");
    let direct=prof::probe_direct(dir.path(),".handbook/selected/value.json");
    assert!(guarded.is_err());assert!(direct.is_ok());
    println!("{}",json!({"event":"direct_read_departure","guarded":format!("{guarded:?}"),"direct":format!("{direct:?}")}));
}
fn run() {match std::env::args().nth(1).as_deref(){Some("reads")=>reads(),Some("effects")=>effects(),Some("pending")=>pending(),Some("probes")=>probes(),_=>panic!("reads|effects|pending|probes")}}
fn main(){std::thread::Builder::new().stack_size(32*1024*1024).spawn(run).unwrap().join().unwrap();}
