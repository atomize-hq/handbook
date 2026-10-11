//! Independent native scratch roots; owner operations only, no copied proof.
#[allow(dead_code, unused_imports)]
#[path = "../../../../../crates/sdk/tests/work_kind_support/mod.rs"]
mod kind_support;
#[allow(dead_code, unused_imports)]
#[path = "../../../../../crates/sdk/tests/work_state_support/mod.rs"]
mod work_support;
use handbook_engine::work_state::NativeWorkStateV1 as Native;
use handbook_sdk::{HandbookSdkV1, local_check::*, local_setup::*, work_state::*};
use serde_json::{json, Value};
use std::{fs, path::Path, time::Instant};

#[repr(C)] #[derive(Default)] struct FileTime { low:u32, high:u32 }
#[link(name="kernel32")] extern "system" {
    fn GetCurrentProcess()->*mut std::ffi::c_void;
    fn GetProcessTimes(h:*mut std::ffi::c_void,c:*mut FileTime,e:*mut FileTime,k:*mut FileTime,u:*mut FileTime)->i32;
    fn GetProcessHandleCount(h:*mut std::ffi::c_void,n:*mut u32)->i32;
}
fn handles()->u32 { let mut n=0; unsafe{assert_ne!(GetProcessHandleCount(GetCurrentProcess(),&mut n),0);} n }
#[repr(C)]#[derive(Default)]struct Memory { cb:u32,faults:u32,peak_working:usize,working:usize,peak_paged:usize,paged:usize,peak_nonpaged:usize,nonpaged:usize,pagefile:usize,peak_pagefile:usize,private:usize }
#[link(name="psapi")]extern "system" {fn GetProcessMemoryInfo(h:*mut std::ffi::c_void,m:*mut Memory,n:u32)->i32;}
fn memory()->Value {let mut m=Memory::default();m.cb=std::mem::size_of::<Memory>() as u32;unsafe{assert_ne!(GetProcessMemoryInfo(GetCurrentProcess(),&mut m,m.cb),0);}json!({"private_bytes":m.private,"working_set_bytes":m.working,"os_peak_working_set_bytes":m.peak_working})}
fn cpu_ms()->f64 {let(mut c,mut e,mut k,mut u)=(FileTime::default(),FileTime::default(),FileTime::default(),FileTime::default());
    unsafe{assert_ne!(GetProcessTimes(GetCurrentProcess(),&mut c,&mut e,&mut k,&mut u),0);}
    (((k.high as u64)<<32|k.low as u64)+((u.high as u64)<<32|u.low as u64)) as f64/10000.0 }
fn bytes(p:&Path)->u64 {if p.is_file(){return fs::metadata(p).unwrap().len();} fs::read_dir(p).unwrap().map(|e|bytes(&e.unwrap().path())).sum()}
fn save(p:&Path,v:&impl serde::Serialize) {fs::write(p,serde_json_canonicalizer::to_vec(v).unwrap()).unwrap();}
fn setup(sdk:&HandbookSdkV1,sample:usize)->(LocalInitialSetupLocatorV1,LocalCheckLocatorV1) {
    let mut r=work_support::supplied_initial_request();
    if sample==4||sample==6 {r.terms.instruction="N".repeat(4000);r.terms.approved_content_fingerprint=r.terms.fingerprint_approved_content().unwrap();}
    let p=sdk.prepare_local_initial_setup(&r).unwrap();let initial=p.locator().clone();let out=sdk.establish_local_initial_setup(p);assert!(out.stopped.is_none(),"{out:?}");
    let fp=sdk.resolve_local_initial_setup_request(&initial).unwrap().terms.unwrap().outcome.unwrap().outcome_fingerprint;
    let mut t=work_support::supplied_successor_terms(&initial,&fp);
    if sample==4||sample==6 {t.instruction="C".repeat(4000);t.approved_content_fingerprint=t.fingerprint_approved_content().unwrap();}
    let p=sdk.prepare_local_check_terms(&initial,t).unwrap();let check=p.locator().clone();assert!(sdk.adopt_local_check_terms(p).outcome.is_some());(initial,check)
}
fn scope()->WorkRegistrationScopeV1 {
    let d=work_requirements_definition_from_intrinsic_v1(&work_support::supplied_intrinsic_definition("1.0.0"));
    let WorkRegistrationScopeV1::Explicit(mut m)=work_support::menu_scope(d) else {unreachable!()};
    m.clauses[0].label_max_bytes=4096;m.clauses[0].work_question_max_bytes=4096;WorkRegistrationScopeV1::Explicit(m)
}
fn activate(sdk:&HandbookSdkV1,initial:&LocalInitialSetupLocatorV1,check:&LocalCheckLocatorV1,key:&str,transition:WorkTermsTransitionV1<'_>)->WorkGrantLocatorV1 {
    let mut t=kind_support::terms(check,key);t.scope=scope();t.approved_content_fingerprint=t.fingerprint_approved_content().unwrap();
    let p=sdk.prepare_work_registration_terms(initial,check,t,transition).unwrap();let g=p.locator().clone();let r=sdk.activate_work_registration_terms(p);assert_eq!(r.disposition,WorkDispositionV1::Committed,"{r:?}");g
}
fn establish_kind(sdk:&HandbookSdkV1,initial:&LocalInitialSetupLocatorV1,check:&LocalCheckLocatorV1,reference:&str,key:&str,role:usize)->WorkKindBindingV1 {
    let WorkRegistrationScopeV1::Explicit(s)=scope() else {unreachable!()};
    let mut d=kind_support::definition(reference);d.template=s.clauses[role].clone();
    let p=sdk.prepare_work_kind_establishment(initial,check,d.clone(),kind_support::approval(sdk,initial,check,&d,key)).unwrap();
    let r=sdk.establish_work_kind(p);assert_eq!(r.disposition,WorkDispositionV1::Committed,"{r:?}");r.outcome.unwrap().binding
}
fn input(g:&WorkGrantLocatorV1,key:&str,kind:Option<&WorkKindBindingV1>,role:usize,near:bool)->WorkRegistrationInputV1 {
    let composition=match role {
        0=>WorkCompositionV1::ordinary_leaf(),
        1=>WorkCompositionV1::planning_output("Exact planning role question"),
        2=>WorkCompositionV1::requirements_input(work_requirements_definition_from_intrinsic_v1(&work_support::supplied_intrinsic_definition("1.0.0"))),
        _=>unreachable!()};
    WorkRegistrationInputV1{request_key:key.into(),item_key:key.into(),label:if near {"é".repeat(if role==0{2048}else{64})}else{"Leaf".into()},
        question:if near {"é".repeat(if role==0{2048}else{128})}else{"Exact bounded work question".into()},actor:g.terms.agent.clone(),expected_state:"absent".into(),
        composition:kind.map_or_else(||composition.clone().into(),|k|WorkCompositionInputV1::kind(k.clone(),composition.role.clone())),
        display_level:match role{0=>None,1=>Some("planning".into()),2=>Some("implementation".into()),_=>unreachable!()}}
}
fn populate(root:&Path,cohort:&str,history:usize,sample:usize) {
    fs::create_dir_all(root).unwrap();let sdk=HandbookSdkV1::open(root);let(initial,check)=setup(&sdk,sample);
    let mut g=activate(&sdk,&initial,&check,"grant-0",WorkTermsTransitionV1::Initial);let mut n=1;
    if sample==2||sample==3||sample==5 {for k in 1..3 {g=activate(&sdk,&initial,&check,&format!("grant-{k}"),WorkTermsTransitionV1::ScopeChange(&g));n+=1;}}
    let governed=sample==1||sample==3||sample==5;let near=sample==5||sample==7;
    if governed {kind_support::govern(&sdk,&initial);g=activate(&sdk,&initial,&check,"grant-G",WorkTermsTransitionV1::EstablishFromDeferred(&g));n+=1;}
    let grant_lineage=n;
    let role=if sample==3||sample==9 {1}else if sample==4 {2}else{0};let mut kinds=Vec::new();
    if cohort=="kind" {kinds.push(establish_kind(&sdk,&initial,&check,"bench.leaf@1.0.0","kind-1",role));n+=1;
        if sample==3||sample==4||sample==5||sample==8 {kinds.push(establish_kind(&sdk,&initial,&check,"bench.leaf@2.0.0","kind-2",role));n+=1;}}
    let mut first=None;while n<history {let k=if kinds.is_empty(){None}else{Some(&kinds[n%kinds.len()])};
        let p=sdk.prepare_work_registration(&initial,&g,input(&g,&format!("item-{n:02}"),k,role,near)).unwrap();first.get_or_insert(p.locator().clone());
        let r=sdk.register_work_item(p);assert_eq!(r.disposition,WorkDispositionV1::Committed,"{r:?}");n+=1;}
    save(&root.join("fixture.initial.json"),&initial);save(&root.join("fixture.grant.json"),&g);
    save(&root.join("fixture.input.json"),&input(&g,"measured-item",kinds.first(),role,near));
    save(&root.join("fixture.first.json"),&first);
    println!("{}",json!({"event":"population","root":root,"cohort":cohort,"history":n,"sample":sample,"basis":if governed{"G"}else{"D"},
        "grant_lineage":grant_lineage,"role":role,"kind_versions":kinds.len(),"near_bound_utf8":near,"large_native":sample==4||sample==6,
        "work_bytes":bytes(&root.join(".handbook/work-state")),"native_bytes":bytes(&root.join(".handbook/local-charter"))+bytes(&root.join(".handbook/local-operating"))+bytes(&root.join(".handbook/admission-governance"))}));
}
fn inspect(root:&Path,cohort:&str,history:usize,sample:usize) {
    let g:WorkGrantLocatorV1=serde_json::from_slice(&fs::read(root.join("fixture.grant.json")).unwrap()).unwrap();
    let first:Option<WorkRequestLocatorV1>=serde_json::from_slice(&fs::read(root.join("fixture.first.json")).unwrap()).unwrap();
    assert!(matches!(Native::current_item(root,&g,&first.unwrap().input.item_key).unwrap(),WorkCurrentItemV1::Open(_)));
    let actual=fs::read_dir(root.join(".handbook/work-state/v1")).unwrap().filter(|e|{let e=e.as_ref().unwrap();let n=e.file_name();let n=n.to_str().unwrap();e.file_type().unwrap().is_dir()&&["grant-","kind-","item-","revoke-"].iter().any(|p|n.starts_with(p))}).count();assert_eq!(actual,history);
    println!("{}",json!({"event":"population","root":root,"cohort":cohort,"history":history,"sample":sample,"basis":"D","grant_lineage":1,"role":0,"kind_versions":if cohort=="kind"{1}else{0},
        "near_bound_utf8":false,"large_native":false,"work_bytes":bytes(&root.join(".handbook/work-state")),"native_bytes":bytes(&root.join(".handbook/local-charter"))+bytes(&root.join(".handbook/local-operating"))+bytes(&root.join(".handbook/admission-governance")),
        "reconciliation":"Original baseline sample0 population completed after scheduler interruption; this command verifies history without creating an effect."}));
}
#[cfg(feature="profile")] fn begin(){handbook_engine::research_profile::begin();}
#[cfg(not(feature="profile"))] fn begin(){}
#[cfg(feature="profile")] fn finish()->Value{handbook_engine::research_profile::finish()}
#[cfg(not(feature="profile"))] fn finish()->Value{json!(null)}
fn finish_resources()->Value {
    let mut p=finish();if let Some(paths)=p.get_mut("paths").and_then(Value::as_array_mut){
        let mut totals=std::collections::BTreeMap::new();for path in paths.iter(){let key=(path["kind"].as_str().unwrap().to_owned(),path["observation"].as_u64().unwrap());let v=totals.entry(key).or_insert((0u64,0u64));v.0+=path["calls"].as_u64().unwrap();v.1+=path["units"].as_u64().unwrap();}
        paths.retain(|path|path["kind"]=="lock_held_us");p["path_totals"]=json!(totals.into_iter().map(|((kind,observation),(calls,units))|json!({"kind":kind,"observation":observation,"calls":calls,"units":units})).collect::<Vec<_>>());
    }p
}
fn measure(root:&Path,cohort:&str,history:usize,sample:usize,variant:&str,on:bool) {
    let initial:LocalInitialSetupLocatorV1=serde_json::from_slice(&fs::read(root.join("fixture.initial.json")).unwrap()).unwrap();
    let g:WorkGrantLocatorV1=serde_json::from_slice(&fs::read(root.join("fixture.grant.json")).unwrap()).unwrap();
    let input:WorkRegistrationInputV1=serde_json::from_slice(&fs::read(root.join("fixture.input.json")).unwrap()).unwrap();
    let sdk=HandbookSdkV1::open(root);let b=bytes(&root.join(".handbook/work-state"));let before=handles();
    if on {begin();}let pair_start=Instant::now();let pair_cpu=cpu_ms();let prep_start=Instant::now();let prep_cpu=cpu_ms();
    let p=sdk.prepare_work_registration(&initial,&g,input).unwrap();let prep_wall=prep_start.elapsed().as_secs_f64()*1000.0;let prep_used=cpu_ms()-prep_cpu;
    let prep_profile=if on{finish()}else{json!(null)};let save_start=Instant::now();let original=p.locator().clone();let locator_bytes=original.canonical_json_bytes().unwrap();
    let mut f=fs::File::create(root.join("saved-original.json")).unwrap();use std::io::Write;f.write_all(&locator_bytes).unwrap();f.sync_all().unwrap();drop(f);
    let save_wall=save_start.elapsed().as_secs_f64()*1000.0;
    if on {begin();}let invoke_start=Instant::now();let invoke_cpu=cpu_ms();let mut max_handles=handles();
    #[cfg(feature="profile")]
    let r=Native::register_with_boundary_for_test(root,p,&mut |_|{max_handles=max_handles.max(handles());Ok(())});
    #[cfg(not(feature="profile"))]
    let r=sdk.register_work_item(p);
    let invoke_wall=invoke_start.elapsed().as_secs_f64()*1000.0;let invoke_used=cpu_ms()-invoke_cpu;let pair_wall=pair_start.elapsed().as_secs_f64()*1000.0;let pair_used=cpu_ms()-pair_cpu;
    let invoke_profile=if on {finish()}else{json!(null)};assert_eq!(r.disposition,WorkDispositionV1::Committed,"{r:?}");assert_eq!(r.outcome.unwrap().locator,original);
    println!("{}",json!({"event":"measurement","variant":variant,"cohort":cohort,"history":history,"sample":sample,"instrumented":on,"root":root,
        "preparation_wall_ms":prep_wall,"preparation_cpu_ms":prep_used,"saving_wall_ms":save_wall,"locator_bytes":locator_bytes.len(),
        "invocation_wall_ms":invoke_wall,"invocation_cpu_ms":invoke_used,"pair_wall_ms":pair_wall,"pair_cpu_ms":pair_used,"work_bytes":b,
        "handles_before":before,"handles_after":handles(),"boundary_max_handles":max_handles,"preparation_profile":prep_profile,"invocation_profile":invoke_profile}));
}
fn resource_call(phase:&str,cycle:usize,variant:&str,on:bool,f:impl FnOnce()->WorkExecutionV1<WorkRegistrationReceiptV1>)->WorkExecutionV1<WorkRegistrationReceiptV1> {
    let before=handles();let mem_before=memory();if on{begin();}let start=Instant::now();let cpu=cpu_ms();let r=f();let wall=start.elapsed().as_secs_f64()*1000.0;let used=cpu_ms()-cpu;
    let profile=if on{finish_resources()}else{json!(null)};let after=handles();assert_eq!(after,before,"resource release: {phase} {cycle}");
    println!("{}",json!({"event":"resource","variant":variant,"phase":phase,"cycle":cycle,"disposition":r.disposition,"refusal":r.refusal.as_ref().map(|e|&e.code),
        "wall_ms":wall,"cpu_ms":used,"handles_before":before,"handles_after":after,"memory_before":mem_before,"memory_after":memory(),"profile":profile}));r
}
fn resources(root:&Path,variant:&str,on:bool) {
    use handbook_engine::work_state::WorkWriteBoundaryV1 as B;
    let initial:LocalInitialSetupLocatorV1=serde_json::from_slice(&fs::read(root.join("fixture.initial.json")).unwrap()).unwrap();
    let g:WorkGrantLocatorV1=serde_json::from_slice(&fs::read(root.join("fixture.grant.json")).unwrap()).unwrap();
    let original:WorkRequestLocatorV1=serde_json::from_slice(&fs::read(root.join("saved-original.json")).unwrap()).unwrap();
    let sdk=HandbookSdkV1::open(root);let process_before=handles();
    for cycle in 0..6 {let r=resource_call("committed_replay",cycle,variant,on,||sdk.recover_work_registration_request(&original));assert_eq!(r.disposition,WorkDispositionV1::Replayed);assert_eq!(r.outcome.unwrap().locator,original);}
    for cycle in 0..3 {for (ix,stage) in [B::BeforeIntent,B::Intent,B::Snapshot,B::PublicationAttempt,B::Reply].into_iter().enumerate(){
        let p=sdk.prepare_work_registration(&initial,&g,input(&g,&format!("resource-{cycle}-{ix}"),None,0,false)).unwrap();let locator=p.locator().clone();
        let r=resource_call(&format!("injected-{stage:?}"),cycle,variant,on,||Native::register_with_boundary_for_test(root,p,|at|if at==stage{Err(kind_support::interruption())}else{Ok(())}));
        assert!(matches!(r.disposition,WorkDispositionV1::Refused|WorkDispositionV1::Uncertain),"injected {stage:?}: {r:?}");
        let r=resource_call(&format!("recover-{stage:?}"),cycle,variant,on,||sdk.recover_work_registration_request(&locator));
        let expected=match stage {B::BeforeIntent=>WorkDispositionV1::Unaccepted,B::Intent|B::Snapshot=>WorkDispositionV1::Committed,B::PublicationAttempt=>WorkDispositionV1::Uncertain,B::Reply=>WorkDispositionV1::Replayed,_=>unreachable!()};
        assert_eq!(r.disposition,expected,"recovery {stage:?}: {r:?}");
        if let Some(outcome)=r.outcome{assert_eq!(outcome.locator,locator);}
    }}
    // A same-process current read waits on the held work lock. The controlled
    // 200 ms delay belongs only to this race sample, never uncontended timing.
    let race_before=handles();let p=sdk.prepare_work_registration(&initial,&g,input(&g,"resource-lock-race",None,0,false)).unwrap();
    let (tx,rx)=std::sync::mpsc::channel();let child_root=root.to_path_buf();let child_grant=g.clone();let child_variant=variant.to_owned();
    let child=std::thread::Builder::new().stack_size(32*1024*1024).spawn(move||{rx.recv().unwrap();if on{begin();}let start=Instant::now();let cpu=cpu_ms();
        let r=Native::current_item(&child_root,&child_grant,"resource-lock-race");let wall=start.elapsed().as_secs_f64()*1000.0;let used=cpu_ms()-cpu;let profile=if on{finish_resources()}else{json!(null)};
        assert!(matches!(r.unwrap(),WorkCurrentItemV1::Open(_)));println!("{}",json!({"event":"lock_wait","variant":child_variant,"controlled_delay_ms":200,"wall_ms":wall,"cpu_ms":used,"profile":profile}));}).unwrap();
    let r=Native::register_with_boundary_for_test(root,p,|stage|{if stage==B::BeforeIntent{tx.send(()).unwrap();std::thread::sleep(std::time::Duration::from_millis(200));}Ok(())});
    assert_eq!(r.disposition,WorkDispositionV1::Committed);child.join().unwrap();assert_eq!(handles(),race_before);
    let pending=(0..6).map(|n|sdk.prepare_work_registration(&initial,&g,input(&g,&format!("resource-refused-{n}"),None,0,false)).unwrap()).collect::<Vec<_>>();
    kind_support::revoke(&sdk,&g);
    for (cycle,p) in pending.into_iter().enumerate(){let r=resource_call("revoked_fresh_invocation",cycle,variant,on,||sdk.register_work_item(p));assert_eq!(r.disposition,WorkDispositionV1::Refused);
        let r=resource_call("revoked_original_replay",cycle,variant,on,||sdk.recover_work_registration_request(&original));assert_eq!(r.disposition,WorkDispositionV1::Replayed);assert_eq!(r.outcome.unwrap().locator,original);}
    assert_eq!(handles(),process_before);println!("{}",json!({"event":"resources_complete","variant":variant,"handles_before":process_before,"handles_after":handles(),"cycles":3,"operations":48}));
}
fn run(){let a:Vec<String>=std::env::args().collect();let root=Path::new(&a[2]);let c=&a[3];let h=a[4].parse().unwrap();let s=a[5].parse().unwrap();
    match a[1].as_str(){"populate"=>populate(root,c,h,s),"inspect"=>inspect(root,c,h,s),"measure"=>measure(root,c,h,s,&a[6],a.get(7).map_or(false,|v|v=="on")),"resources"=>resources(root,&a[6],a.get(7).map_or(false,|v|v=="on")),_=>panic!("populate|inspect|measure|resources")}}
fn main(){std::thread::Builder::new().stack_size(32*1024*1024).spawn(run).unwrap().join().unwrap();}
