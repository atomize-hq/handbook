//! Copies supplied test authority into fresh temporary roots only.
#[allow(dead_code, unused_imports)]
#[path = "../../../../../crates/sdk/tests/work_kind_support/mod.rs"]
mod support;
use handbook_engine::{research_profile as prof, work_state::NativeWorkStateV1 as Native};
use handbook_sdk::{HandbookSdkV1, work_state::*};
use serde_json::json;
use std::{fs, path::Path, time::Instant};

fn profile_run() {
    let args:Vec<_>=std::env::args().collect();
    let output=Path::new(&args[1]); fs::create_dir_all(output).unwrap();
    for cohort in ["shared","distinct"] {
        let dir=tempfile::Builder::new().prefix("hcm-scalable-native-").tempdir().unwrap();
        let sdk=HandbookSdkV1::open(dir.path());
        let (initial,check)=support::setup(&sdk);
        let mut grant=support::activate(&sdk,&initial,&check,"grant",None);
        let mut first=None;
        let targets=if cohort=="shared" {vec![8,16,31,32]} else {vec![16]};
        let mut history=1;
        for target in targets {
            while history<target {
                if cohort=="distinct" && history==8 {
                    support::govern(&sdk,&initial);
                    grant=support::activate(&sdk,&initial,&check,"grant-governed",Some(&grant));
                    history+=1;
                }
                let key=format!("item-{history}");
                let input=WorkRegistrationInputV1 {request_key:key.clone(),item_key:key,label:"Leaf".into(),
                    question:if history%2==0 {"Exact bounded work question".into()} else {"q".repeat(256)},
                    actor:grant.terms.agent.clone(),expected_state:"absent".into(),
                    composition:WorkCompositionV1::ordinary_leaf().into(),display_level:None};
                let p=sdk.prepare_work_registration(&initial,&grant,input).unwrap();
                let original=p.locator().clone();
                assert_eq!(sdk.register_work_item(p).disposition,WorkDispositionV1::Committed);
                first.get_or_insert(original); history+=1;
            }
            let original=first.as_ref().unwrap();
            // Every SDK call reopens native owner; first timed read has no warmup.
            for sample in 0..6 {
                let enabled=sample%2==1;
                if enabled {prof::begin();}
                let cpu=prof::cpu_ms();let start=Instant::now();
                let result=Native::current_item(dir.path(),&grant,&original.input.item_key).unwrap();
                let wall=start.elapsed().as_secs_f64()*1000.0;let used=prof::cpu_ms()-cpu;
                assert!(matches!(result,WorkCurrentItemV1::Open(_)));
                let spans=if enabled {prof::finish()} else {json!([])};
                println!("{}",json!({"cohort":cohort,"history":history,"operation":"current_item","sample":sample,
                    "instrumented":enabled,"first_reopen":sample==0,"wall_ms":wall,"cpu_ms":used,"spans":spans}));
            }
            if cohort=="shared" && target==31 {
                let p=sdk.prepare_work_registration(&initial,&grant,WorkRegistrationInputV1 {
                    request_key:"profile-effect".into(),item_key:"profile-effect".into(),label:"Leaf".into(),
                    question:"Exact bounded work question".into(),actor:grant.terms.agent.clone(),
                    expected_state:"absent".into(),composition:WorkCompositionV1::ordinary_leaf().into(),display_level:None}).unwrap();
                prof::begin();let cpu=prof::cpu_ms();let start=Instant::now();
                let r=sdk.register_work_item(p);let wall=start.elapsed().as_secs_f64()*1000.0;let used=prof::cpu_ms()-cpu;
                assert_eq!(r.disposition,WorkDispositionV1::Committed);history+=1;
                println!("{}",json!({"cohort":cohort,"history":31,"operation":"registration_effect","instrumented":true,
                    "wall_ms":wall,"cpu_ms":used,"spans":prof::finish()}));
            }
            if cohort=="shared" && target==32 {
                let root=dir.path().join(".handbook/work-state/v1");
                let mut found=false;
                for entry in fs::read_dir(root).unwrap() {
                    let entry=entry.unwrap();
                    let intent=entry.path().join("intent.json");
                    if !intent.is_file() {continue;}
                    let data=fs::read(&intent).unwrap();let value:serde_json::Value=serde_json::from_slice(&data).unwrap();
                    if !found && value["operation"].to_string().contains("item-") {
                        fs::write(output.join("native-intent.json"),&data).unwrap();
                        fs::write(output.join("native-authority.json"),serde_json_canonicalizer::to_vec(&value["authority"]).unwrap()).unwrap();
                        fs::write(output.join("native-terminal.json"),fs::read(entry.path().join("terminal.json")).unwrap()).unwrap();
                        found=true;
                    }
                }
                assert!(found);
            }
        }
    }
}
fn main() {
    std::thread::Builder::new().stack_size(32*1024*1024).spawn(profile_run).unwrap().join().unwrap();
}
