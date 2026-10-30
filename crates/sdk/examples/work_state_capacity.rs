//! Linux principal capacity proof through the normal typed SDK. This is one
//! required operating cohort, not a benchmark sweep or synthetic initial load.
#[allow(dead_code)]
#[path = "../tests/local_contract_support/mod.rs"]
mod local_contract_support;
#[allow(dead_code)]
#[path = "../tests/work_kind_support/mod.rs"]
mod support;
use handbook_sdk::work_state::*;
use handbook_sdk::{HandbookSdkV1, SetupMode, SetupRequest};
use std::{
    path::{Path, PathBuf},
    time::Instant,
};
fn native_bytes(path: &Path) -> (u64, u64, u64) {
    let mut bytes = 0;
    let mut blocks = 0;
    let mut files = 0;
    for e in std::fs::read_dir(path).unwrap() {
        let p = e.unwrap().path();
        let m = std::fs::symlink_metadata(&p).unwrap();
        if m.is_dir() {
            let (a, b, c) = native_bytes(&p);
            bytes += a;
            blocks += b;
            files += c;
        } else {
            assert!(m.is_file());
            bytes += m.len();
            files += 1;
            #[cfg(unix)]
            {
                use std::os::unix::fs::MetadataExt;
                blocks += m.blocks() * 512;
            }
        }
    }
    (bytes, blocks, files)
}

// Finite same-original protocol. All observations (including uncertainty) are
// external proof data; no transition supplies replacement authority or a new key.
pub fn finish_same_original(
    sdk:&HandbookSdkV1, locator:&WorkRequestLocatorV1,
    mut result:WorkExecutionV1<WorkRegistrationReceiptV1>,
    mut retry:impl FnMut()->PreparedWorkRegistrationV1,
    trace:&mut std::fs::File,
)->Result<WorkRegistrationReceiptV1,String>{
    use std::io::Write;
    let canonical=locator.canonical_json_bytes().map_err(|e|format!("{e:?}"))?;
    let mut record=|stage:&str,result:&WorkExecutionV1<WorkRegistrationReceiptV1>|->Result<(),String>{
        writeln!(trace,"{}",serde_json::json!({"request_key":locator.input.request_key,"stage":stage,"execution":result})).map_err(|e|e.to_string())?;
        trace.sync_data().map_err(|e|e.to_string())
    };
    record("invoke",&result)?;
    for attempt in 0..2 {
        if matches!(result.disposition,WorkDispositionV1::Committed|WorkDispositionV1::Replayed){
            let receipt=result.outcome.ok_or("terminal success missing exact receipt")?;
            if receipt.locator.canonical_json_bytes().map_err(|e|format!("{e:?}"))?!=canonical{return Err("terminal receipt changed saved original".into());}
            return Ok(receipt);
        }
        if matches!(result.disposition,WorkDispositionV1::Uncertain|WorkDispositionV1::Pending){
            result=sdk.resolve_work_registration_request(locator);record("resolve",&result)?;
            if matches!(result.disposition,WorkDispositionV1::Pending|WorkDispositionV1::Uncertain){
                result=sdk.recover_work_registration_request(locator);record("recover",&result)?;
            }
            if matches!(result.disposition,WorkDispositionV1::Committed|WorkDispositionV1::Replayed){continue;}
        }
        if result.disposition==WorkDispositionV1::Unaccepted&&attempt==0 {
            let prepared=retry();
            if prepared.locator().canonical_json_bytes().map_err(|e|format!("{e:?}"))?!=canonical{return Err("repreparation changed saved original".into());}
            result=sdk.register_work_item(prepared);record("retry-same-original",&result)?;
        }else{return Err(format!("bounded original protocol stopped: {result:?}"));}
    }
    if matches!(result.disposition,WorkDispositionV1::Committed|WorkDispositionV1::Replayed){
        let receipt=result.outcome.ok_or("terminal success missing receipt")?;
        if receipt.locator.canonical_json_bytes().map_err(|e|format!("{e:?}"))?!=canonical{return Err("terminal receipt changed saved original".into());}
        return Ok(receipt);
    }
    Err(format!("same original remained unresolved after finite protocol: {result:?}"))
}
fn main() {
    let root = PathBuf::from(
        std::env::args()
            .nth(1)
            .expect("owned empty project path required"),
    );
    let proof=PathBuf::from(std::env::args().nth(2).expect("precreated external proof directory required"));
    assert!(proof.is_dir(),"proof directory must exist before retained project capture");
    assert!(!proof.starts_with(&root)&&!root.starts_with(&proof),"proof path must be disjoint");
    let mut trace=std::fs::OpenOptions::new().create_new(true).write(true).open(proof.join("original-transitions.jsonl")).unwrap();
    let mut current=std::fs::OpenOptions::new().create_new(true).write(true).open(proof.join("current-original.json")).unwrap();
    assert!(!root.exists(), "never replace an existing project");
    std::fs::create_dir_all(&root).unwrap();
    let started = Instant::now();
    let sdk = HandbookSdkV1::open(&root);
    let (initial, check) = support::setup(&sdk);
    let grant = support::activate(&sdk, &initial, &check, "capacity-grant", None);
    let mut early = Vec::new();
    for n in 0..100_000 {
        let mut input = WorkRegistrationInputV1 {
            request_key: format!("capacity-{n:06}"),
            item_key: format!("capacity-item-{n:06}"),
            label: "L".repeat(1024),
            question: "Q".repeat(1024),
            actor: grant.terms.agent.clone(),
            expected_state: "absent".into(),
            composition: WorkCompositionV1::ordinary_leaf().into(),
            display_level: None,
        };
        input.label.replace_range(..8, "Capacity");
        let prepared = sdk
            .prepare_work_registration(&initial, &grant, input.clone())
            .unwrap();
        let locator=prepared.locator().clone();
        {use std::io::{Write,Seek,SeekFrom};current.set_len(0).unwrap();current.seek(SeekFrom::Start(0)).unwrap();current.write_all(&locator.canonical_json_bytes().unwrap()).unwrap();current.sync_all().unwrap();}
        let out = sdk.register_work_item(prepared);
        let receipt=finish_same_original(&sdk,&locator,out,||sdk.prepare_work_registration(&initial,&grant,input.clone()).unwrap(),&mut trace)
            .unwrap_or_else(|e|panic!("at {n}, SAME saved original: {e}"));
        if [0, 50_000, 99_999].contains(&n) {
            early.push(receipt);
        }
        if n % 128 == 0 {
            sdk.maintain_work_storage(WorkMaintenanceBudgetV1 {
                items: 4096,
                bytes: 64 * 1024 * 1024,
            })
            .unwrap();
        }
        if n == 99 || n % 1000 == 999 {
            println!(
                "{}",
                serde_json::json!({"published_work_items":n+1,"elapsed_seconds":started.elapsed().as_secs_f64(),"counts":sdk.work_storage_statistics().unwrap()})
            );
        }
    }
    let counts = sdk.work_storage_statistics().unwrap();
    assert_eq!(counts.live_records, 100_000);
    assert_eq!(counts.retained_original_outcomes, 100_001);
    while !sdk
        .maintain_work_storage(WorkMaintenanceBudgetV1 {
            items: 4096,
            bytes: 64 * 1024 * 1024,
        })
        .unwrap()
        .complete
    {}
    // Independent bounded API recounts actual current item keys, original
    // terminal-backed Work requests, and actual retained exact artifact bytes.
    let (item_count, outcome_count, record_bytes) = sdk
        .with_retained_work_snapshot(|snapshot| {
            let mut item_count = 0u64;
            let mut outcome_count = 0u64;
            let mut record_bytes = 0u64;
            let mut after = None;
            loop {
                let page = snapshot.page(after.as_deref(), 16).unwrap();
                for record in page.records {
                    record_bytes += record.retained_record_bytes;
                    if let Some(WorkRetainedOutcomeV1::Item(receipt)) = record.outcome {
                        assert_eq!(receipt.snapshot.state, "open");
                        item_count += 1;
                        outcome_count += 1;
                    }
                }
                if page.complete {
                    break;
                }
                after = page.next;
                assert!(after.is_some());
            }
            (item_count, outcome_count, record_bytes)
        })
        .unwrap();
    assert_eq!(item_count, 100_000);
    assert_eq!(outcome_count, 100_000);
    assert!(
        record_bytes >= 1024 * 1024 * 1024,
        "actual retained owner record bytes {record_bytes}"
    );
    for receipt in &early {
        assert_eq!(
            sdk.resolve_work_registration_request(&receipt.locator)
                .outcome
                .as_ref(),
            Some(receipt)
        );
        assert_eq!(
            sdk.resolve_current_work_item(&grant, &receipt.locator.input.item_key)
                .unwrap(),
            WorkCurrentItemV1::Open(receipt.snapshot.clone())
        );
    }
    // Continue actual work and fresh administration beyond every operating target.
    let next = sdk
        .prepare_work_registration_terms(
            &initial,
            &check,
            support::terms(&check, "capacity-fresh-admin"),
            WorkTermsTransitionV1::ScopeChange(&grant),
        )
        .unwrap();
    let successor = next.locator().clone();
    assert_eq!(
        sdk.activate_work_registration_terms(next).disposition,
        WorkDispositionV1::Committed
    );
    for n in 0..32 {
        let input = WorkRegistrationInputV1 {
            request_key: format!("beyond-{n}"),
            item_key: format!("beyond-{n}"),
            label: "Beyond target".into(),
            question: "Continued authorized publication".into(),
            actor: successor.terms.agent.clone(),
            expected_state: "absent".into(),
            composition: WorkCompositionV1::ordinary_leaf().into(),
            display_level: None,
        };
        let p = sdk
            .prepare_work_registration(&initial, &successor, input)
            .unwrap();
        assert_eq!(
            sdk.register_work_item(p).disposition,
            WorkDispositionV1::Committed
        );
    }
    let revoke = sdk
        .prepare_work_registration_revocation(
            &successor,
            "capacity-fresh-revoke",
            &successor.terms.revocation_principal,
            "human.capacity",
            "Revoke after demonstrated operating range",
        )
        .unwrap();
    assert_eq!(
        sdk.revoke_work_registration_terms(&revoke).disposition,
        WorkDispositionV1::Committed
    );
    let final_counts = sdk.work_storage_statistics().unwrap();
    assert_eq!(final_counts.live_records, 100_032);
    for receipt in &early {
        assert_eq!(
            sdk.recover_work_registration_request(&receipt.locator)
                .outcome
                .as_ref(),
            Some(receipt)
        );
    }
    assert!(
        sdk.run_setup(&SetupRequest {
            mode: SetupMode::Refresh,
            rewrite: false,
            reset_state: true
        })
        .unwrap()
        .reset_applied
    );
    drop(sdk);
    let reopened = HandbookSdkV1::open(&root);
    for receipt in &early {
        assert_eq!(
            reopened
                .resolve_work_registration_request(&receipt.locator)
                .outcome
                .as_ref(),
            Some(receipt)
        );
    }
    let native = native_bytes(&root.join(".handbook/work-state"));
    println!(
        "{}",
        serde_json::json!({"complete":true,"live_work_items_recount":item_count,"retained_original_work_outcomes_recount":outcome_count,"retained_exact_owner_record_bytes_recount":record_bytes,"native_file_bytes":native.0,"native_allocated_bytes":native.1,"native_files":native.2,"final_counts":final_counts,"elapsed_seconds":started.elapsed().as_secs_f64(),"root":root})
    );
}
