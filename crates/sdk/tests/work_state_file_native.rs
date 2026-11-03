//! New greenfield production-route proof. Existing byte fixtures and semantic
//! prefix/authority tests remain independent; no old-store import is claimed.
#[allow(dead_code)]
mod local_contract_support;
#[allow(dead_code)]
mod work_kind_support;
use handbook_engine::work_state::{NativeWorkStateV1, WorkWriteBoundaryV1 as B};
use handbook_sdk::work_state::*;
use handbook_sdk::{HandbookSdkV1, SetupMode, SetupRequest};
use work_kind_support::*;
#[test]
fn real_sdk_indexed_lifecycle_continues_past_old_slots_with_exact_originals() {
    let dir = tempfile::tempdir().unwrap();
    let sdk = HandbookSdkV1::open(dir.path());
    let (initial, check) = setup(&sdk);
    let kind = establish_kind(&sdk, &initial, &check, "kind", "project.native@1.0.0");
    let grant = activate(&sdk, &initial, &check, "grant", None);
    let mut originals = Vec::new();
    for n in 0..40 {
        let prepared = sdk
            .prepare_work_registration(&initial, &grant, input(&grant, &kind, &format!("item-{n}")))
            .unwrap();
        let original = prepared.locator().clone();
        assert_eq!(
            sdk.resolve_work_registration_request(&original).disposition,
            WorkDispositionV1::Unaccepted
        );
        let out = sdk.register_work_item(prepared);
        assert_eq!(out.disposition, WorkDispositionV1::Committed, "{out:?}");
        originals.push(out.outcome.unwrap());
    }
    let counts = sdk.work_storage_statistics().unwrap();
    assert_eq!(counts.live_records, 40);
    assert_eq!(counts.retained_original_outcomes, 42);
    assert!(counts.database_publications > counts.retained_original_outcomes);
    let early = originals.first().unwrap();
    assert_eq!(
        sdk.resolve_current_work_item(&grant, &early.locator.input.item_key)
            .unwrap(),
        WorkCurrentItemV1::Open(early.snapshot.clone())
    );
    let next = sdk
        .prepare_work_registration_terms(
            &initial,
            &check,
            terms(&check, "fresh-scope"),
            WorkTermsTransitionV1::ScopeChange(&grant),
        )
        .unwrap();
    let next_grant = next.locator().clone();
    assert_eq!(
        sdk.activate_work_registration_terms(next).disposition,
        WorkDispositionV1::Committed
    );
    assert!(sdk
        .resolve_current_work_registration_terms(&initial, &grant)
        .is_err());
    let revoke = sdk
        .prepare_work_registration_revocation(
            &next_grant,
            "fresh-revoke",
            &next_grant.terms.revocation_principal,
            "human",
            "revoke after old capacity",
        )
        .unwrap();
    assert_eq!(
        sdk.revoke_work_registration_terms(&revoke).disposition,
        WorkDispositionV1::Committed
    );
    for early in &originals {
        assert_eq!(
            sdk.resolve_work_registration_request(&early.locator)
                .outcome
                .as_ref(),
            Some(early)
        );
        assert_eq!(
            sdk.recover_work_registration_request(&early.locator)
                .outcome
                .as_ref(),
            Some(early)
        );
    }
    assert_eq!(
        sdk.inspect_work_completion_support(&early.locator)
            .unwrap()
            .completion_support,
        "unsupported"
    );
    while !sdk
        .maintain_work_storage(WorkMaintenanceBudgetV1::default())
        .unwrap()
        .complete
    {}
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
    let sdk = HandbookSdkV1::open(dir.path());
    assert_eq!(
        sdk.resolve_work_registration_request(&early.locator)
            .outcome
            .as_ref(),
        Some(early)
    );
    assert_eq!(sdk.work_storage_statistics().unwrap().live_records, 40);
}
#[test]
fn original_prefix_recovery_and_pending_administration_keep_the_same_authority_rule() {
    for at in [
        B::AcceptanceReady,
        B::Intent,
        B::Decision,
        B::Snapshot,
        B::PublicationAttempt,
        B::Terminal,
        B::Reply,
    ] {
        let dir = tempfile::tempdir().unwrap();
        let sdk = HandbookSdkV1::open(dir.path());
        let (initial, check) = setup(&sdk);
        let kind = establish_kind(&sdk, &initial, &check, "kind", "project.native@1.0.0");
        let grant = activate(&sdk, &initial, &check, "grant", None);
        let prepared = sdk
            .prepare_work_registration(&initial, &grant, input(&grant, &kind, "original"))
            .unwrap();
        let locator = prepared.locator().clone();
        let out = NativeWorkStateV1::register_with_boundary_for_test(dir.path(), prepared, |b| {
            if b == at {
                Err(handbook_engine::local_authority::LocalAuthorityRefusalV1 {
                    code: "interrupt".into(),
                    detail: "native boundary interruption".into(),
                    unresolved_request: None,
                })
            } else {
                Ok(())
            }
        });
        assert!(out.outcome.is_none());
        let resolved = sdk.resolve_work_registration_request(&locator);
        if matches!(at, B::Terminal | B::Reply) {
            assert_eq!(resolved.disposition, WorkDispositionV1::Replayed);
        } else if at == B::AcceptanceReady {
            // Fresh Item acceptance begins only when the complete R selects.
            assert_eq!(resolved.disposition, WorkDispositionV1::Unaccepted);
            assert_eq!(sdk.recover_work_registration_request(&locator).disposition,
                WorkDispositionV1::Unaccepted);
        } else {
            // PublicationAttempt now means complete native Owned checkpoint
            // BEFORE outputs, not a separately selected attempted-only row.
            // Exact negative evidence permits SAME-original continuation.

            assert_eq!(resolved.disposition, WorkDispositionV1::Pending);
            let recovered = sdk.recover_work_registration_request(&locator);
            assert_eq!(
                recovered.disposition,
                WorkDispositionV1::Committed,
                "{at:?}: {recovered:?}"
            );
        }
    }
}

#[test]
fn four_sdk_callers_three_sdk_readers_one_native_pinned_reader_and_sdk_maintainer() {
    use std::sync::{Arc, Barrier};
    let dir = tempfile::tempdir().unwrap();
    let sdk = HandbookSdkV1::open(dir.path());
    let (initial, check) = setup(&sdk);
    let kind = establish_kind(&sdk, &initial, &check, "kind", "project.native@1.0.0");
    let grant = activate(&sdk, &initial, &check, "grant", None);
    let seed = sdk
        .register_work_item(
            sdk.prepare_work_registration(&initial, &grant, input(&grant, &kind, "seed"))
                .unwrap(),
        )
        .outcome
        .unwrap();
    sdk.with_retained_work_snapshot(|old| {
        let barrier = Arc::new(Barrier::new(9));
        let mut callers = Vec::new();
        for n in 0..4 {
            let root = dir.path().to_path_buf();
            let initial = initial.clone();
            let grant = grant.clone();
            let kind = kind.clone();
            let b = barrier.clone();
            callers.push(std::thread::spawn(move || {
                let sdk = HandbookSdkV1::open(root);
                let mut first = input(&grant, &kind, &format!("contender-{n}"));
                first.item_key = "shared-item".into();
                let prepared = sdk
                    .prepare_work_registration(&initial, &grant, first)
                    .unwrap();
                b.wait();
                let contender = sdk.register_work_item(prepared);
                for k in 0..8 {
                    let p = sdk
                        .prepare_work_registration(
                            &initial,
                            &grant,
                            input(&grant, &kind, &format!("caller-{n}-{k}")),
                        )
                        .unwrap();
                    assert_eq!(
                        sdk.register_work_item(p).disposition,
                        WorkDispositionV1::Committed
                    );
                }
                contender.disposition
            }));
        }
        let mut readers = Vec::new();
        for _ in 0..3 {
            let root = dir.path().to_path_buf();
            let grant = grant.clone();
            let snapshot = seed.snapshot.clone();
            let b = barrier.clone();
            readers.push(std::thread::spawn(move || {
                let sdk = HandbookSdkV1::open(root);
                b.wait();
                for _ in 0..8 {
                    assert_eq!(
                        sdk.resolve_current_work_item(&grant, "seed").unwrap(),
                        WorkCurrentItemV1::Open(snapshot.clone())
                    );
                    std::thread::yield_now();
                }
            }));
        }
        let root = dir.path().to_path_buf();
        let b = barrier.clone();
        let maintainer = std::thread::spawn(move || {
            let sdk = HandbookSdkV1::open(root);
            b.wait();
            let mut blocked = false;
            for _ in 0..8 {
                blocked |= sdk
                    .maintain_work_storage(WorkMaintenanceBudgetV1::default())
                    .unwrap()
                    .blocked_by_reader;
                std::thread::yield_now();
            }
            blocked
        });
        barrier.wait();
        for _ in 0..8 {
            assert_eq!(old.original(&seed.locator).outcome.as_ref(), Some(&seed));
        }
        let outcomes: Vec<_> = callers.into_iter().map(|t| t.join().unwrap()).collect();
        for t in readers {
            t.join().unwrap();
        }
        assert!(maintainer.join().unwrap());
        assert_eq!(
            outcomes
                .iter()
                .filter(|d| **d == WorkDispositionV1::Committed)
                .count(),
            1
        );
        assert_eq!(
            outcomes
                .iter()
                .filter(|d| **d == WorkDispositionV1::Refused)
                .count(),
            3
        );
    })
    .unwrap();
    while !sdk
        .maintain_work_storage(WorkMaintenanceBudgetV1::default())
        .unwrap()
        .complete
    {}
    assert_eq!(sdk.work_storage_statistics().unwrap().live_records, 34);
    assert_eq!(
        sdk.resolve_work_registration_request(&seed.locator)
            .outcome
            .as_ref(),
        Some(&seed)
    );
}

#[test]
fn valid_large_kind_keeps_individual_record_bounds_and_item_preflight_before_reservation() {
    let dir = tempfile::tempdir().unwrap();
    let sdk = HandbookSdkV1::open(dir.path());
    let (initial, check) = setup(&sdk);
    let mut content = definition("project.kind.item-bound@1.0.0");
    content.template.display_levels = (0..3750)
        .map(|index| format!("level-{index:05}-{}", "x".repeat(116)))
        .collect();
    let prepared = sdk
        .prepare_work_kind_establishment(
            &initial,
            &check,
            content.clone(),
            approval(&sdk, &initial, &check, &content, "large-committed-kind"),
        )
        .unwrap();
    let original_kind = prepared.locator().clone();
    let receipt = sdk.establish_work_kind(prepared);
    assert_eq!(
        receipt.disposition,
        WorkDispositionV1::Committed,
        "{receipt:?}"
    );
    let receipt = receipt.outcome.unwrap();
    assert!(serde_json::to_vec(&receipt).unwrap().len() <= 1024 * 1024);
    assert!(matches!(
        sdk.resolve_work_kind(&receipt.binding).unwrap(),
        WorkCurrentKindV1::Committed(_)
    ));
    assert_eq!(
        sdk.recover_work_kind_request(&original_kind)
            .outcome
            .as_ref(),
        Some(&receipt)
    );
    let grant = activate(&sdk, &initial, &check, "grant", None);
    let prepared = sdk
        .prepare_work_registration(
            &initial,
            &grant,
            input(&grant, &receipt.binding, "oversized-derived-item"),
        )
        .unwrap();
    let original = prepared.locator().clone();
    assert!(original.canonical_json_bytes().unwrap().len() <= 1024 * 1024);
    let before = sdk.work_storage_statistics().unwrap();
    let selector = dir
        .path()
        .join(".handbook/work-state/v1/database/SELECTED.json");
    let before_selector = std::fs::read(&selector).unwrap();
    let anchors = [
        ".handbook-work-state.v1.json",
        ".handbook/work-state/v1/owner.json",
    ]
    .map(|p| (p, std::fs::read(dir.path().join(p)).unwrap()));
    let execution = sdk.register_work_item(prepared);
    assert_eq!(
        execution.disposition,
        WorkDispositionV1::Refused,
        "{execution:?}"
    );
    assert_eq!(execution.refusal.unwrap().code, "work_codec");
    assert_eq!(
        sdk.resolve_work_registration_request(&original).disposition,
        WorkDispositionV1::Unaccepted
    );
    let after = sdk.work_storage_statistics().unwrap();
    assert_eq!(after.database_publications, before.database_publications);
    assert_eq!(
        after.retained_original_outcomes,
        before.retained_original_outcomes
    );
    assert_eq!(after.retained_record_bytes, before.retained_record_bytes);
    assert_eq!(after.live_records, before.live_records);
    assert_eq!(std::fs::read(selector).unwrap(), before_selector);
    for (path, bytes) in anchors {
        assert_eq!(std::fs::read(dir.path().join(path)).unwrap(), bytes);
    }
}

#[test]
fn t03_d01_same_original_native_nonselection_allows_positive_noncommit() {
    let dir = tempfile::tempdir().unwrap();
    let sdk = HandbookSdkV1::open(dir.path());
    let (initial, check) = setup(&sdk);
    let kind = establish_kind(
        &sdk,
        &initial,
        &check,
        "d01-kind",
        "project.native-d01@1.0.0",
    );
    let grant = activate(&sdk, &initial, &check, "d01-grant", None);
    let prepared = sdk
        .prepare_work_registration(&initial, &grant, input(&grant, &kind, "d01-original"))
        .unwrap();
    let original = prepared.locator().clone();
    let bytes = original.canonical_json_bytes().unwrap();
    let first = NativeWorkStateV1::register_with_boundary_for_test(dir.path(), prepared, |at| {
        if at == B::Snapshot {
            Err(LocalAuthorityRefusalV1 {
                code: "valid_interruption".into(),
                detail: "completed accepted snapshot".into(),
                unresolved_request: None,
            })
        } else {
            Ok(())
        }
    });
    assert_eq!(first.disposition, WorkDispositionV1::Uncertain);
    let fired = std::cell::Cell::new(false);
    let result = NativeWorkStateV1::recover_with_boundary_for_test(dir.path(), &original, |at| {
        if at == B::NativeBeforeSelection && !fired.replace(true) {
            Err(LocalAuthorityRefusalV1 {
                code: "valid_native_interruption".into(),
                detail: "actual native outputs owned before selector".into(),
                unresolved_request: None,
            })
        } else {
            Ok(())
        }
    });
    assert!(fired.get());
    assert_eq!(
        result.disposition,
        WorkDispositionV1::NonCommitted,
        "SAME original must close without unrelated publication: {result:?}"
    );
    assert_eq!(original.canonical_json_bytes().unwrap(), bytes);
    assert_eq!(
        sdk.recover_work_registration_request(&original).disposition,
        WorkDispositionV1::NonCommitted
    );
}

#[test]
fn exact_checkpoint_terminal_ownership_survives_nonapplication_and_later_scope_head() {
    for lose_authority in [false,true]{
        let dir=tempfile::tempdir().unwrap();let sdk=HandbookSdkV1::open(dir.path());
        let (initial,check)=setup(&sdk);let kind=establish_kind(&sdk,&initial,&check,"s01-kind","project.native-s01@1.0.0");let grant=activate(&sdk,&initial,&check,"s01-grant",None);
        let prepared=sdk.prepare_work_registration(&initial,&grant,input(&grant,&kind,"s01-original")).unwrap();
        let locator=prepared.locator().clone();let canonical=locator.canonical_json_bytes().unwrap();
        let mut terminal_owned=None;
        let out=NativeWorkStateV1::register_with_boundary_for_test(dir.path(),prepared,|at|{
            if at==B::PublicationAttempt {
                let cell:serde_json::Value=serde_json::from_slice(&std::fs::read(dir.path().join(".handbook/work-state/v1/database/active.json")).unwrap()).unwrap();
                assert_eq!(cell["state"],"owned");let branch=&cell["value"]["positive"];
                let evidence=&branch["original"]["evidence"];
                let payload:Vec<u8>=serde_json::from_value(evidence["payload"].clone()).unwrap();
                let proof:serde_json::Value=serde_json::from_slice(&payload).unwrap();
                assert_eq!(proof["checkpoint_role"],"terminal-attempt");assert_eq!(proof["work_request_name"],"item-s01-original");
                assert_eq!(proof["logical_target"],"terminal.json");
                let checkpoint:serde_json::Value=serde_json::from_str(proof["checkpoint_utf8"].as_str().unwrap()).unwrap();
                assert_eq!(checkpoint["target"],"terminal.json");assert_eq!(checkpoint["intent_fingerprint"],proof["work_intent_identity"]);
                assert_eq!(evidence["binding"]["candidate"]["token"],branch["candidate"]["token"]);
                assert_eq!(evidence["binding"]["candidate"]["sequence"],branch["candidate"]["version"]["sequence"]);
                assert_eq!(evidence["binding"]["expected"],cell["value"]["prior"]["version"]);
                assert_eq!(proof["attempted_record_key"],"record/item-s01-original/attempted.json");
                assert_eq!(proof["terminal_record_key"],"record/item-s01-original/terminal.json");
                for id in branch["owned"].as_array().unwrap(){let id=id.as_str().unwrap();assert!(!dir.path().join(format!(".handbook/work-state/v1/database/objects/{}/{}/{}",&id[..2],&id[2..4],id)).exists());}
                terminal_owned=Some(evidence.clone());
                return Err(LocalAuthorityRefusalV1{code:"valid_terminal_owned_interruption".into(),detail:"exact terminal owned before outputs".into(),unresolved_request:None});
            }Ok(())
        });
        assert_eq!(out.disposition,WorkDispositionV1::Uncertain);let owned=terminal_owned.unwrap();
        // Existing public resolve reconciles metadata only. No Work effect can
        // be installed from this evidence, with or without live authority.
        let pending=sdk.resolve_work_registration_request(&locator);assert_eq!(pending.disposition,WorkDispositionV1::Pending,"{pending:?}");
        let negative=NativeWorkStateV1::checkpoint_evidence_for_test(dir.path(),&locator,None).unwrap();
        assert_eq!(negative["outcome"]["disposition"],"NotSelected");assert_eq!(negative["outcome"]["value"],serde_json::json!([]));
        let mut expected=owned.clone();expected["binding"]["resolving"]=negative["evidence"]["binding"]["resolving"].clone();
        assert_eq!(negative["evidence"],expected);let original_key=negative["reference"]["key"].as_str().unwrap().to_owned();
        if lose_authority {
            assert!(sdk.run_setup(&SetupRequest{mode:SetupMode::Refresh,rewrite:false,reset_state:true}).unwrap().reset_applied);
            let recovered=sdk.recover_work_registration_request(&locator);assert_eq!(recovered.disposition,WorkDispositionV1::NonCommitted,"{recovered:?}");
            assert!(recovered.outcome.is_none());
        }else{
            let recovered=sdk.recover_work_registration_request(&locator);assert_eq!(recovered.disposition,WorkDispositionV1::Committed,"{recovered:?}");
            assert_eq!(recovered.outcome.unwrap().locator,locator);
            let next=sdk.prepare_work_registration(&initial,&grant,input(&grant,&kind,"s01-unrelated")).unwrap();
            assert_eq!(sdk.register_work_item(next).disposition,WorkDispositionV1::Committed);
        }
        while !sdk.maintain_work_storage(WorkMaintenanceBudgetV1::default()).unwrap().complete{}
        drop(sdk);let sdk=HandbookSdkV1::open(dir.path());
        let older=NativeWorkStateV1::checkpoint_evidence_for_test(dir.path(),&locator,Some(&original_key)).unwrap();
        assert_eq!(older["reference"],negative["reference"]);assert_eq!(older["evidence"],negative["evidence"]);assert_eq!(older["outcome"],negative["outcome"]);
        let head=NativeWorkStateV1::checkpoint_evidence_for_test(dir.path(),&locator,None).unwrap();
        assert_ne!(head["reference"],negative["reference"]);
        assert_eq!(locator.canonical_json_bytes().unwrap(),canonical);
        let resolved=sdk.resolve_work_registration_request(&locator);
        assert_eq!(resolved.disposition,if lose_authority{WorkDispositionV1::NonCommitted}else{WorkDispositionV1::Replayed});
    }
}

#[test]
fn exact_original_resolution_refuses_lost_established_anchor_without_recreation(){
    let dir=tempfile::tempdir().unwrap();let sdk=HandbookSdkV1::open(dir.path());
    let (initial,check)=setup(&sdk);let kind=establish_kind(&sdk,&initial,&check,"anchor-kind","project.native-anchor@1.0.0");let grant=activate(&sdk,&initial,&check,"anchor-grant",None);
    let prepared=sdk.prepare_work_registration(&initial,&grant,input(&grant,&kind,"anchor-original")).unwrap();
    let original=prepared.locator().clone();let canonical=original.canonical_json_bytes().unwrap();
    let committed=sdk.register_work_item(prepared);assert_eq!(committed.disposition,WorkDispositionV1::Committed);
    let replay=sdk.resolve_work_registration_request(&original);assert_eq!(replay.disposition,WorkDispositionV1::Replayed);assert_eq!(replay.outcome,committed.outcome);
    let paths=[".handbook/work-state/v1/owner.json",".handbook-work-state-bootstrap.v1.json",".handbook/work-state/v1/database/STORE.json",".handbook/work-state/v1/database/READY.json",".handbook/work-state/v1/database/SELECTED.json"];
    let before=paths.map(|p|(p,std::fs::read(dir.path().join(p)).unwrap()));
    #[cfg(unix)]let lock_identity={use std::os::unix::fs::MetadataExt;let m=std::fs::metadata(dir.path().join(".handbook/work-state/work.lock")).unwrap();(m.dev(),m.ino())};
    let anchor=dir.path().join(".handbook-work-state.v1.json");std::fs::remove_file(&anchor).unwrap();
    drop(sdk);let sdk=HandbookSdkV1::open(dir.path());
    // Exactly ONE resolution after the ONLY authorized loss; no recovery,
    // registration, restoration, new key, or other loss/corruption operation.
    let lost=sdk.resolve_work_registration_request(&original);
    assert_eq!(lost.disposition,WorkDispositionV1::Uncertain);assert!(lost.outcome.is_none());
    let refusal=lost.refusal.unwrap();assert_eq!(refusal.code,"work_proof_missing");assert!(refusal.detail.contains("loss"),"{refusal:?}");
    assert!(!anchor.exists());assert!(!dir.path().join(".handbook/work-state-bootstrap").exists());
    for (path,bytes) in before{assert_eq!(std::fs::read(dir.path().join(path)).unwrap(),bytes);}
    #[cfg(unix)]{use std::os::unix::fs::MetadataExt;let m=std::fs::metadata(dir.path().join(".handbook/work-state/work.lock")).unwrap();assert_eq!((m.dev(),m.ino()),lock_identity);}
    assert_eq!(original.canonical_json_bytes().unwrap(),canonical);
}

#[allow(dead_code)]
#[path="../examples/work_state_capacity.rs"]
mod capacity_consumer;
#[test]
fn finite_same_saved_original_consumer_records_uncertainty_and_native_recovery(){
    for interruption in [B::BeforeIntent,B::NativeBeforeOwnership,B::Snapshot,B::PublicationAttempt]{
        // External trace file exists before root/authority capture, and compiler
        // scratch uses another parent. No proof file is retained owner data.
        let trace_dir=tempfile::tempdir().unwrap();let mut trace=std::fs::File::create(trace_dir.path().join("transitions.jsonl")).unwrap();
        let dir=tempfile::tempdir().unwrap();let sdk=HandbookSdkV1::open(dir.path());
        let (initial,check)=setup(&sdk);let kind=establish_kind(&sdk,&initial,&check,"consumer-kind","project.native-consumer@1.0.0");let grant=activate(&sdk,&initial,&check,"consumer-grant",None);
        let submitted=input(&grant,&kind,"consumer-original");
        let prepared=sdk.prepare_work_registration(&initial,&grant,submitted.clone()).unwrap();
        let saved=prepared.locator().clone();let canonical=saved.canonical_json_bytes().unwrap();
        {use std::io::Write;let mut file=std::fs::File::create(trace_dir.path().join("saved-original.json")).unwrap();file.write_all(&canonical).unwrap();file.sync_all().unwrap();}
        let first=NativeWorkStateV1::register_with_boundary_for_test(dir.path(),prepared,|at|{
            if at==interruption{Err(LocalAuthorityRefusalV1{code:"valid_consumer_interruption".into(),detail:format!("{interruption:?}"),unresolved_request:None})}else{Ok(())}
        });
        // BeforeIntent truthfully refuses before acceptance; a caller that
        // received Refused must STOP rather than silently treating it uncertain.
        if interruption==B::BeforeIntent {
            let result=capacity_consumer::finish_same_original(&sdk,&saved,first,||panic!("terminal refusal cannot retry"),&mut trace);
            assert!(result.is_err());assert_eq!(sdk.resolve_work_registration_request(&saved).disposition,WorkDispositionV1::Unaccepted);
            let trace_bytes=std::fs::read_to_string(trace_dir.path().join("transitions.jsonl")).unwrap();
            println!("T03_CONSUMER_TRACE_BEGIN {interruption:?}");
            print!("{trace_bytes}");
            println!("T03_CONSUMER_TRACE_END {interruption:?}");
            continue;
        }
        assert_eq!(first.disposition,WorkDispositionV1::Uncertain);
        let receipt=capacity_consumer::finish_same_original(&sdk,&saved,first,||sdk.prepare_work_registration(&initial,&grant,submitted.clone()).unwrap(),&mut trace).unwrap();
        assert_eq!(receipt.locator.canonical_json_bytes().unwrap(),canonical);
        let resolved=sdk.resolve_work_registration_request(&saved);assert_eq!(resolved.disposition,WorkDispositionV1::Replayed);assert_eq!(resolved.outcome,Some(receipt));
        let trace_bytes=std::fs::read_to_string(trace_dir.path().join("transitions.jsonl")).unwrap();
        println!("T03_CONSUMER_TRACE_BEGIN {interruption:?}");
        print!("{trace_bytes}");
        println!("T03_CONSUMER_TRACE_END {interruption:?}");
        assert!(trace_bytes.contains("\"stage\":\"invoke\""));assert!(trace_bytes.contains("\"stage\":\"resolve\""));assert_eq!(trace_bytes.contains("\"stage\":\"recover\""),interruption!=B::NativeBeforeOwnership);
        assert!(trace_bytes.contains("uncertain"));assert_eq!(trace_bytes.contains("retry-same-original"),interruption==B::NativeBeforeOwnership);
    }
}

#[test]
fn terminal_preownership_refusal_keeps_same_original_recoverable(){
    let dir=tempfile::tempdir().unwrap();let sdk=HandbookSdkV1::open(dir.path());
    let (initial,check)=setup(&sdk);let kind=establish_kind(&sdk,&initial,&check,"preowned-kind","project.native-preowned@1.0.0");let grant=activate(&sdk,&initial,&check,"preowned-grant",None);
    let prepared=sdk.prepare_work_registration(&initial,&grant,input(&grant,&kind,"preowned-original")).unwrap();
    let saved=prepared.locator().clone();let bytes=saved.canonical_json_bytes().unwrap();let mut stage=0;
    let result=NativeWorkStateV1::register_with_boundary_for_test(dir.path(),prepared,|at|{
        if at==B::NativeBeforeOwnership{
            stage+=1;
            // Atomic accepted-ready R, then the separate coupled terminal T.
            if stage==2{
                let cell:serde_json::Value=serde_json::from_slice(&std::fs::read(dir.path().join(".handbook/work-state/v1/database/active.json")).unwrap()).unwrap();
                assert_eq!(cell["state"],"idle");
                return Err(LocalAuthorityRefusalV1{code:"fresh_preownership_refusal".into(),detail:"terminal native ownership not yet accepted".into(),unresolved_request:None});
            }
        }Ok(())
    });
    assert_eq!(stage,2);assert_eq!(result.disposition,WorkDispositionV1::Uncertain);assert_eq!(result.refusal.unwrap().code,"fresh_preownership_refusal");
    let pending=sdk.resolve_work_registration_request(&saved);assert_eq!(pending.disposition,WorkDispositionV1::Pending);
    let recovered=sdk.recover_work_registration_request(&saved);assert_eq!(recovered.disposition,WorkDispositionV1::Committed,"{recovered:?}");assert_eq!(recovered.outcome.unwrap().locator.canonical_json_bytes().unwrap(),bytes);
}

#[test]
fn exact_original_resolution_refuses_lost_established_work_lock_without_recreation(){
    let dir=tempfile::tempdir().unwrap();let sdk=HandbookSdkV1::open(dir.path());
    let (initial,check)=setup(&sdk);
    let kind=establish_kind(&sdk,&initial,&check,"lock-kind","project.native-lock@1.0.0");
    let grant=activate(&sdk,&initial,&check,"lock-grant",None);
    let prepared=sdk.prepare_work_registration(&initial,&grant,input(&grant,&kind,"lock-original")).unwrap();
    let original=prepared.locator().clone();let canonical=original.canonical_json_bytes().unwrap();
    let committed=sdk.register_work_item(prepared);
    assert_eq!(committed.disposition,WorkDispositionV1::Committed,"{committed:?}");
    let replay=sdk.resolve_work_registration_request(&original);
    assert_eq!(replay.disposition,WorkDispositionV1::Replayed,"{replay:?}");
    assert_eq!(replay.outcome,committed.outcome);
    let paths=[".handbook-work-state.v1.json",".handbook-work-state-bootstrap.v1.json",".handbook/work-state/v1/owner.json",".handbook/work-state/v1/database/STORE.json",".handbook/work-state/v1/database/READY.json",".handbook/work-state/v1/database/SELECTED.json"];
    let before=paths.map(|path|(path,std::fs::read(dir.path().join(path)).unwrap()));
    let lock_path=dir.path().join(".handbook/work-state/work.lock");
    #[cfg(unix)]{
        use std::os::unix::fs::MetadataExt;
        let metadata=std::fs::metadata(&lock_path).unwrap();
        let identity=(metadata.dev(),metadata.ino());
        println!("original_established_work_lock_identity device={} inode={}",identity.0,identity.1);
    }
    drop(sdk);std::fs::remove_file(&lock_path).unwrap();
    let sdk=HandbookSdkV1::open(dir.path());
    // ONLY this single resolution follows the ONLY authorized fixture loss.
    // No recover/register/restore/new key or other artifact alteration.
    let lost=sdk.resolve_work_registration_request(&original);
    println!("lost_established_work_lock_resolution={lost:?}");
    assert_eq!(lost.disposition,WorkDispositionV1::Uncertain);
    assert!(lost.outcome.is_none());
    let refusal=lost.refusal.unwrap();
    assert_eq!(refusal.code,"work_retained_state");
    assert_eq!(refusal.detail,"lock file disappeared before acquisition");
    assert_eq!(std::fs::symlink_metadata(&lock_path).unwrap_err().kind(),std::io::ErrorKind::NotFound);
    assert_eq!(std::fs::symlink_metadata(dir.path().join(".handbook/work-state-bootstrap")).unwrap_err().kind(),std::io::ErrorKind::NotFound);
    for(path,bytes)in before{assert_eq!(std::fs::read(dir.path().join(path)).unwrap(),bytes);}
    assert_eq!(original.canonical_json_bytes().unwrap(),canonical);
}

#[test]
fn exact_original_resolution_refuses_lost_established_native_gate_without_recreation() {
    use std::io::Write;
    use std::os::unix::fs::MetadataExt;
    let evidence = std::path::PathBuf::from(std::env::var_os("D04_GATE_EVIDENCE_DIR").expect("owned gate evidence directory"));
    let dir = tempfile::tempdir().unwrap().keep();
    println!("gate_fixture={}", dir.display());
    std::fs::write(evidence.join("fixture-path.txt"), dir.as_os_str().as_encoded_bytes()).unwrap();
    let sdk = HandbookSdkV1::open(&dir);
    let (initial, check) = setup(&sdk);
    let kind = establish_kind(&sdk, &initial, &check, "gate-kind", "project.native-gate@1.0.0");
    let grant = activate(&sdk, &initial, &check, "gate-grant", None);
    let prepared = sdk.prepare_work_registration(&initial, &grant, input(&grant, &kind, "gate-original")).unwrap();
    let original = prepared.locator().clone();
    let canonical = original.canonical_json_bytes().unwrap();
    let mut saved = std::fs::OpenOptions::new().write(true).create_new(true).open(evidence.join("original.json")).unwrap();
    saved.write_all(&canonical).unwrap(); saved.sync_all().unwrap();
    std::fs::File::open(&evidence).unwrap().sync_all().unwrap(); drop(saved);
    let committed = sdk.register_work_item(prepared);
    assert_eq!(committed.disposition, WorkDispositionV1::Committed, "{committed:?}");
    let replay = sdk.resolve_work_registration_request(&original);
    assert_eq!(replay.disposition, WorkDispositionV1::Replayed, "{replay:?}");
    assert_eq!(replay.outcome, committed.outcome);
    std::fs::write(evidence.join("baseline.json"), serde_json::to_vec(&serde_json::json!({"committed":committed,"replayed":replay})).unwrap()).unwrap();
    let paths = [".handbook-work-state.v1.json", ".handbook-work-state-bootstrap.v1.json", ".handbook/work-state/v1/owner.json", ".handbook/work-state/v1/database/STORE.json", ".handbook/work-state/v1/database/READY.json", ".handbook/work-state/v1/database/SELECTED.json", ".handbook/work-state/v1/database/active.json", ".handbook/work-state/v1/database/maintenance.json"];
    let before = paths.map(|path| (path, std::fs::read(dir.join(path)).unwrap()));
    std::fs::write(evidence.join("controls-before.json"), serde_json::to_vec(&before).unwrap()).unwrap();
    let outer_lock = dir.join(".handbook/work-state/work.lock");
    let outer_metadata = std::fs::symlink_metadata(&outer_lock).unwrap();
    let outer_identity = (outer_metadata.dev(), outer_metadata.ino());
    let gate = dir.join(".handbook/work-state/v1/database/gate.lock");
    let gate_metadata = std::fs::symlink_metadata(&gate).unwrap();
    assert!(gate_metadata.is_file()); assert!(!gate_metadata.file_type().is_symlink());
    let gate_bytes = std::fs::read(&gate).unwrap();
    std::fs::write(evidence.join("gate-before.json"), serde_json::to_vec(&serde_json::json!({"dev":gate_metadata.dev(),"inode":gate_metadata.ino(),"bytes":gate_bytes,"outer_lock_dev":outer_identity.0,"outer_lock_inode":outer_identity.1})).unwrap()).unwrap();
    drop(sdk);
    std::fs::remove_file(&gate).unwrap();
    let sdk = HandbookSdkV1::open(&dir);
    // Exactly one resolution follows this sole established-native-gate loss.
    // No recovery, registration, restoration, new key, or second alteration.
    let lost = sdk.resolve_work_registration_request(&original);
    println!("lost_established_native_gate_resolution={lost:?}");
    std::fs::write(evidence.join("resolution.json"), serde_json::to_vec(&lost).unwrap()).unwrap();
    let after = paths.map(|path| (path, std::fs::read(dir.join(path)).unwrap()));
    std::fs::write(evidence.join("controls-after.json"), serde_json::to_vec(&after).unwrap()).unwrap();
    assert_eq!(lost.disposition, WorkDispositionV1::Uncertain);
    assert!(lost.outcome.is_none());
    let refusal = lost.refusal.unwrap();
    assert_eq!(refusal.code, "work_database");
    assert!(refusal.detail.contains("lock file disappeared before acquisition"), "{refusal:?}");
    assert_eq!(std::fs::symlink_metadata(&gate).unwrap_err().kind(), std::io::ErrorKind::NotFound);
    assert_eq!(std::fs::symlink_metadata(dir.join(".handbook/work-state-bootstrap")).unwrap_err().kind(), std::io::ErrorKind::NotFound);
    assert_eq!(before, after);
    let after_lock = std::fs::symlink_metadata(&outer_lock).unwrap();
    assert_eq!((after_lock.dev(), after_lock.ino()), outer_identity);
    assert_eq!(std::fs::read(evidence.join("original.json")).unwrap(), canonical);
    assert_eq!(original.canonical_json_bytes().unwrap(), canonical);
}

// D04 application-key injection only: this test-local capability forwards the
// unchanged production WorkFs operations and lock identities. It is not a
// native-format decoder, a permission token, or a production entrypoint.
use handbook_engine::{RepositoryExclusiveLockV1, RepositoryReadV1, RetainedRepositoryRootV1};
use file_native::{Backend, Storage};
#[derive(Clone)]
struct D04WorkFs<'a> {
    pub root: &'a RetainedRepositoryRootV1,
    pub prefix: String,
    pub physical: String,
}
impl D04WorkFs<'_> {
    fn path(&self, p: &str) -> String {
        format!("{}/{p}", self.prefix)
    }
}
impl Storage for D04WorkFs<'_> {
    fn validate(&self) -> file_native::Result<()> {
        if self.root.governance_physical_root_fingerprint_v1()? != self.physical {
            return Err("original work physical root required".into());
        }
        Ok(())
    }
    fn read(&self, p: &str, n: usize) -> file_native::Result<Option<Vec<u8>>> {
        self.validate()?;
        // Qualified only for an already-initialized .handbook and the
        // library's generated relative database paths. Keep the production
        // owner-read parent classification without exposing its private API.
        let path = self.path(p);
        let parts = path.split('/').collect::<Vec<_>>();
        let mut parent = ".handbook".to_owned();
        for part in parts.iter().skip(1).take(parts.len().saturating_sub(2)) {
            let entries = self.root.list_directory_bounded(&parent, 4096)?;
            match entries.iter().find(|entry| entry.name() == *part) {
                None => return Ok(None),
                Some(entry) if entry.kind() == handbook_engine::RepositoryDirectoryEntryKindV1::Directory => {}
                // Same boxed Display as the private donor. This test-only
                // classification error is not the private concrete error type.
                Some(_) => return Err("local owner parent is not a native directory".into()),
            }
            parent.push('/');
            parent.push_str(part);
        }
        Ok(match self.root.read_regular_bounded(&path, n)? {
            RepositoryReadV1::Absent => None,
            RepositoryReadV1::Found(b) => Some(b),
        })
    }
    fn mkdir(&self, p: &str) -> file_native::Result<()> {
        self.validate()?;
        self.root.create_directories(&self.path(p))?;
        Ok(())
    }
    fn write_new(&self, p: &str, b: &[u8]) -> file_native::Result<()> {
        self.validate()?;
        self.root.write_new_durable(&self.path(p), b)?;
        self.validate()
    }
    fn rename_new(&self, a: &str, b: &str) -> file_native::Result<()> {
        self.validate()?;
        self.root
            .rename_noreplace_durable(&self.path(a), &self.path(b))?;
        self.validate()
    }
    fn displace(&self, a: &str, expected: &[u8], b: &str) -> file_native::Result<()> {
        self.validate()?;
        self.root
            .displace_exact_durable(&self.path(a), expected, &self.path(b))?;
        self.validate()
    }
    fn remove(&self, p: &str) -> file_native::Result<()> {
        self.validate()?;
        if self.read(p, 16 * 1024 * 1024)?.is_some() {
            self.root.remove_regular_durable(&self.path(p))?;
        }
        self.validate()
    }
    fn directory_sync_supported(&self) -> bool {
        cfg!(unix)
    }
}
impl Backend for D04WorkFs<'_> {
    type Guard = RepositoryExclusiveLockV1;
    fn lock(&self, p: &str) -> file_native::Result<Self::Guard> {
        self.validate()?;
        Ok(self
            .root
            .acquire_existing_exclusive_lock_v1(&self.path(p))?)
    }
    fn try_lock(&self, p: &str) -> file_native::Result<Option<Self::Guard>> {
        self.validate()?;
        Ok(self
            .root
            .try_acquire_existing_exclusive_lock_v1(&self.path(p))?)
    }
    fn lock_identity(&self, g: &Self::Guard) -> file_native::Result<String> {
        Ok(g.physical_fingerprint_v1()?)
    }
    fn validate_lock(&self, p: &str, g: &Self::Guard) -> file_native::Result<()> {
        g.validate_at(self.root, &self.path(p))?;
        self.validate()
    }
}

fn d04_native_store<'a>(root: &'a handbook_engine::RetainedRepositoryRootV1, namespace: &str) -> file_native::Database<D04WorkFs<'a>> {
    // Positive precondition, never implicit initialization or directory creation.
    root.list_directory_bounded(".handbook", 4096)
        .expect("normal typed SDK baseline has an initialized native .handbook");
    file_native::Database::open_for_recovery(D04WorkFs {
        root,
        prefix: ".handbook/work-state/v1/database".into(),
        physical: root.governance_physical_root_fingerprint_v1().unwrap(),
    }, namespace).unwrap()
}
#[derive(Clone, Copy, Debug)]
enum D04Case { Header, Terminal, Grant, Kind }
fn d04_run_indexed_case(case: D04Case) {
    use std::io::Write;
    use std::os::unix::fs::MetadataExt;
    let label = match case { D04Case::Header => "H", D04Case::Terminal => "T", D04Case::Grant => "A", D04Case::Kind => "K" };
    let evidence = std::path::PathBuf::from(std::env::var_os("D04_INDEXED_EVIDENCE_DIR").expect("owned indexed-integrity evidence"));
    let dir = tempfile::tempdir().unwrap().keep();
    println!("indexed_case={label} fixture={}", dir.display());
    std::fs::write(evidence.join("fixture-path.txt"), dir.as_os_str().as_encoded_bytes()).unwrap();
    let sdk = HandbookSdkV1::open(&dir);
    let (initial, check) = setup(&sdk);
    let kind = establish_kind(&sdk, &initial, &check, "indexed-kind", "project.indexed-integrity@1.0.0");
    let grant = activate(&sdk, &initial, &check, "indexed-grant", None);
    let prepared = sdk.prepare_work_registration(&initial, &grant, input(&grant, &kind, "indexed-original-a")).unwrap();
    let original = prepared.locator().clone();
    let canonical = original.canonical_json_bytes().unwrap();
    let mut saved = std::fs::OpenOptions::new().write(true).create_new(true).open(evidence.join("original.json")).unwrap();
    saved.write_all(&canonical).unwrap(); saved.sync_all().unwrap();
    std::fs::File::open(&evidence).unwrap().sync_all().unwrap(); drop(saved);
    let committed = sdk.register_work_item(prepared);
    assert_eq!(committed.disposition, WorkDispositionV1::Committed, "{committed:?}");
    let replay = sdk.resolve_work_registration_request(&original);
    assert_eq!(replay.disposition, WorkDispositionV1::Replayed, "{replay:?}");
    assert_eq!(replay.outcome, committed.outcome);
    std::fs::write(evidence.join("baseline.json"), serde_json::to_vec(&serde_json::json!({"committed":committed,"replayed":replay})).unwrap()).unwrap();
    let mut secondary_name = None;
    if matches!(case, D04Case::Terminal) {
        let prepared_b = sdk.prepare_work_registration(&initial, &grant, input(&grant, &kind, "indexed-original-b")).unwrap();
        let original_b = prepared_b.locator().clone();
        let mut saved_b = std::fs::OpenOptions::new().write(true).create_new(true).open(evidence.join("secondary-original.json")).unwrap();
        saved_b.write_all(&original_b.canonical_json_bytes().unwrap()).unwrap(); saved_b.sync_all().unwrap();
        std::fs::File::open(&evidence).unwrap().sync_all().unwrap(); drop(saved_b);
        let committed_b = sdk.register_work_item(prepared_b);
        assert_eq!(committed_b.disposition, WorkDispositionV1::Committed, "{committed_b:?}");
        let replay_b = sdk.resolve_work_registration_request(&original_b);
        assert_eq!(replay_b.disposition, WorkDispositionV1::Replayed, "{replay_b:?}");
        assert_eq!(replay_b.outcome, committed_b.outcome);
        std::fs::write(evidence.join("secondary-baseline.json"), serde_json::to_vec(&serde_json::json!({"committed":committed_b,"replayed":replay_b})).unwrap()).unwrap();
        secondary_name = Some(format!("item-{}", original_b.input.request_key));
    }
    let original_native = NativeWorkStateV1::checkpoint_evidence_for_test(&dir, &original, None).unwrap();
    std::fs::write(evidence.join("original-native-before.json"), serde_json::to_vec(&original_native).unwrap()).unwrap();
    let namespace = original_native["evidence"]["binding"]["namespace"].as_str().unwrap().to_owned();
    let native_scope = original_native["evidence"]["scope"].as_str().unwrap().to_owned();
    let native_key = original_native["reference"]["key"].as_str().unwrap().to_owned();
    let name = format!("item-{}", original.input.request_key);
    let grant_name = format!("grant-{}", original.grant.terms.request_key);
    let kind_name = format!("kind-{}", original.kind_provenance.as_ref().unwrap().establishment.locator.approval.request_key);
    let paths = [".handbook-work-state.v1.json", ".handbook-work-state-bootstrap.v1.json", ".handbook/work-state/v1/owner.json", ".handbook/work-state/v1/database/STORE.json", ".handbook/work-state/v1/database/READY.json", ".handbook/work-state/v1/database/SELECTED.json", ".handbook/work-state/v1/database/active.json", ".handbook/work-state/v1/database/maintenance.json"];
    let baseline_controls = paths.map(|path| (path, std::fs::read(dir.join(path)).unwrap()));
    std::fs::write(evidence.join("controls-before-injection.json"), serde_json::to_vec(&baseline_controls).unwrap()).unwrap();
    let lock_paths = [".handbook/work-state/work.lock", ".handbook/work-state/v1/database/gate.lock"];
    let lock_identities = lock_paths.map(|path| { let m = std::fs::symlink_metadata(dir.join(path)).unwrap(); assert!(m.is_file()); assert!(!m.file_type().is_symlink()); (path, m.dev(), m.ino(), std::fs::read(dir.join(path)).unwrap()) });
    std::fs::write(evidence.join("lock-identities-before.json"), serde_json::to_vec(&lock_identities).unwrap()).unwrap();
    drop(sdk);
    {
        let root = handbook_engine::RetainedRepositoryRootV1::open_exact(&dir).unwrap();
        let _owner_lock = root.acquire_existing_exclusive_lock_v1(".handbook/work-state/work.lock").unwrap();
        let db = d04_native_store(&root, &namespace);
        let positive_route = format!("item/{}", original.item_identity);
        assert_eq!(db.get(&positive_route).unwrap(), Some(name.as_bytes().to_vec()));
        let target = match case {
            D04Case::Header => format!("request/{name}"),
            D04Case::Terminal => format!("record/{name}/terminal.json"),
            D04Case::Grant => format!("request/{grant_name}"),
            D04Case::Kind => format!("request/{kind_name}"),
        };
        let before_value = db.get(&target).unwrap().expect("exact populated target");
        let replacement = match case {
            D04Case::Terminal => Some(db.get(&format!("record/{}/terminal.json", secondary_name.as_ref().unwrap())).unwrap().unwrap()),
            D04Case::Kind => Some(b"{".to_vec()),
            _ => None,
        };
        if let Some(bytes) = &replacement { assert_ne!(&before_value, bytes); }
        let version = db.version().unwrap();
        let mut tx = file_native::Transaction::new(version, format!("d04-logical-alteration-{label}"), Vec::new());
        match &replacement { Some(bytes) => tx.put(&target, bytes.clone()).unwrap(), None => tx.delete(&target).unwrap() }
        std::fs::write(evidence.join("sole-transaction.json"), serde_json::to_vec(&tx).unwrap()).unwrap();
        let mutation = db.publish(tx).unwrap();
        assert_eq!(mutation.disposition, file_native::OriginalDisposition::Selected);
        assert_eq!(db.get(&target).unwrap(), replacement);
        assert_eq!(db.get(&positive_route).unwrap(), Some(name.as_bytes().to_vec()));
        let native_after = db.reconcile_evidence(&native_scope, Some(&native_key)).unwrap().original.unwrap();
        let native_after_json = serde_json::json!({"reference":native_after.reference,"outcome":native_after.outcome,"evidence":native_after.evidence});
        assert_eq!(native_after_json["reference"], original_native["reference"]);
        assert_eq!(native_after_json["outcome"], original_native["outcome"]);
        assert_eq!(native_after_json["evidence"], original_native["evidence"]);
        std::fs::write(evidence.join("mutation.json"), serde_json::to_vec(&serde_json::json!({"label":label,"target":target,"prior_value":before_value,"replacement":replacement,"publication_outcome":mutation,"positive_route":positive_route,"retained_original_native":native_after_json})).unwrap()).unwrap();
    }
    let post_injection = paths.map(|path| (path, std::fs::read(dir.join(path)).unwrap()));
    // The one deliberate publication changes SELECTED/active. Authority and
    // store establishment bytes stay exact; compare final consumption to this
    // post-injection baseline, not to an invented unchanged selector.
    for i in 0..5 { assert_eq!(baseline_controls[i], post_injection[i]); }
    std::fs::write(evidence.join("controls-after-injection.json"), serde_json::to_vec(&post_injection).unwrap()).unwrap();
    let sdk = HandbookSdkV1::open(&dir);
    // The sole post-alteration SDK action. No recovery/register/restore follows.
    let resolved = sdk.resolve_work_registration_request(&original);
    println!("indexed_case={label} resolution={resolved:?}");
    std::fs::write(evidence.join("resolution.json"), serde_json::to_vec(&resolved).unwrap()).unwrap();
    let after = paths.map(|path| (path, std::fs::read(dir.join(path)).unwrap()));
    std::fs::write(evidence.join("controls-after-resolution.json"), serde_json::to_vec(&after).unwrap()).unwrap();
    assert_eq!(after, post_injection);
    for (path, dev, inode, bytes) in lock_identities {
        let m = std::fs::symlink_metadata(dir.join(path)).unwrap();
        assert_eq!((m.dev(), m.ino()), (dev, inode));
        assert_eq!(std::fs::read(dir.join(path)).unwrap(), bytes);
    }
    assert_eq!(std::fs::symlink_metadata(dir.join(".handbook/work-state-bootstrap")).unwrap_err().kind(), std::io::ErrorKind::NotFound);
    assert_eq!(std::fs::read(evidence.join("original.json")).unwrap(), canonical);
    assert_eq!(original.canonical_json_bytes().unwrap(), canonical);
    assert_eq!(resolved.disposition, WorkDispositionV1::Uncertain, "{resolved:?}");
    assert!(resolved.outcome.is_none());
    let refusal = resolved.refusal.expect("missing/substituted required proof must be explicit");
    let expected_code = match case { D04Case::Header => "work_item_history_missing", D04Case::Terminal => "work_history_integrity", D04Case::Grant => "work_dependency_missing", D04Case::Kind => "work_codec" };
    assert_eq!(refusal.code, expected_code, "{refusal:?}");
}
#[test]
fn indexed_accepted_header_loss_never_becomes_unaccepted() { d04_run_indexed_case(D04Case::Header); }
#[test]
fn indexed_terminal_substitution_refuses_without_outcome() { d04_run_indexed_case(D04Case::Terminal); }
#[test]
fn indexed_original_requires_exact_grant_dependency() { d04_run_indexed_case(D04Case::Grant); }
#[test]
fn indexed_malformed_kind_header_refuses_without_outcome() { d04_run_indexed_case(D04Case::Kind); }

#[test]
fn original_absence_and_retained_snapshot_keep_positive_witness_distinction() {
    use std::io::Write;
    use std::os::unix::fs::MetadataExt;
    let evidence_override = std::env::var_os("D04_FINITE_EVIDENCE_DIR");
    let evidence_temp = evidence_override.is_none().then(|| tempfile::tempdir().unwrap());
    let evidence = evidence_override.map(std::path::PathBuf::from).unwrap_or_else(|| evidence_temp.as_ref().unwrap().path().to_path_buf());
    let fixture = tempfile::tempdir().unwrap();
    let dir = if evidence_temp.is_some() { fixture.path().to_path_buf() } else { fixture.keep() };
    println!("finite_witness_fixture={}", dir.display());
    std::fs::write(evidence.join("fixture-path.txt"), dir.as_os_str().as_encoded_bytes()).unwrap();
    let sdk = HandbookSdkV1::open(&dir);
    let (initial, check) = setup(&sdk);
    let kind = establish_kind(&sdk, &initial, &check, "witness-kind", "project.native-witness@1.0.0");
    let grant = activate(&sdk, &initial, &check, "witness-grant", None);
    let prepared = sdk.prepare_work_registration(&initial, &grant, input(&grant, &kind, "witness-original-u")).unwrap();
    let original = prepared.locator().clone();
    let canonical = original.canonical_json_bytes().unwrap();
    let mut saved = std::fs::OpenOptions::new().write(true).create_new(true).open(evidence.join("original.json")).unwrap();
    saved.write_all(&canonical).unwrap(); saved.sync_all().unwrap();
    std::fs::File::open(&evidence).unwrap().sync_all().unwrap(); drop(saved);
    let never = sdk.resolve_work_registration_request(&original);
    let never_retained = sdk.with_retained_work_snapshot(|snapshot| snapshot.original(&original)).unwrap();
    std::fs::write(evidence.join("never-accepted.json"), serde_json::to_vec(&serde_json::json!({"live":never,"retained":never_retained})).unwrap()).unwrap();
    for reply in [&never, &never_retained] {
        assert_eq!(reply.disposition, WorkDispositionV1::Unaccepted, "{reply:?}");
        assert!(reply.outcome.is_none()); assert!(reply.refusal.is_none());
    }
    let committed = sdk.register_work_item(prepared);
    assert_eq!(committed.disposition, WorkDispositionV1::Committed, "{committed:?}");
    assert_eq!(committed.outcome.as_ref().unwrap().locator, original);
    let replay = sdk.resolve_work_registration_request(&original);
    assert_eq!(replay.disposition, WorkDispositionV1::Replayed, "{replay:?}");
    assert_eq!(replay.outcome, committed.outcome);
    std::fs::write(evidence.join("baseline.json"), serde_json::to_vec(&serde_json::json!({"committed":committed,"replayed":replay})).unwrap()).unwrap();
    let mut other_input = input(&grant, &kind, "witness-original-v");
    other_input.item_key = original.input.item_key.clone();
    let other = sdk.prepare_work_registration(&initial, &grant, other_input).unwrap().locator().clone();
    assert_eq!(other.item_identity, original.item_identity);
    assert_ne!(other.input.request_key, original.input.request_key);
    let other_canonical = other.canonical_json_bytes().unwrap();
    let mut saved_other = std::fs::OpenOptions::new().write(true).create_new(true).open(evidence.join("other-original.json")).unwrap();
    saved_other.write_all(&other_canonical).unwrap(); saved_other.sync_all().unwrap();
    std::fs::File::open(&evidence).unwrap().sync_all().unwrap(); drop(saved_other);
    let other_live = sdk.resolve_work_registration_request(&other);
    let other_retained = sdk.with_retained_work_snapshot(|snapshot| snapshot.original(&other)).unwrap();
    std::fs::write(evidence.join("same-item-other-request.json"), serde_json::to_vec(&serde_json::json!({"live":other_live,"retained":other_retained})).unwrap()).unwrap();
    for reply in [&other_live, &other_retained] {
        assert_eq!(reply.disposition, WorkDispositionV1::Unaccepted, "{reply:?}");
        assert!(reply.outcome.is_none()); assert!(reply.refusal.is_none());
    }
    let native = NativeWorkStateV1::checkpoint_evidence_for_test(&dir, &original, None).unwrap();
    std::fs::write(evidence.join("original-native-before.json"), serde_json::to_vec(&native).unwrap()).unwrap();
    let namespace = native["evidence"]["binding"]["namespace"].as_str().unwrap().to_owned();
    let scope = native["evidence"]["scope"].as_str().unwrap().to_owned();
    let native_key = native["reference"]["key"].as_str().unwrap().to_owned();
    let paths = [".handbook-work-state.v1.json", ".handbook-work-state-bootstrap.v1.json", ".handbook/work-state/v1/owner.json", ".handbook/work-state/v1/database/STORE.json", ".handbook/work-state/v1/database/READY.json", ".handbook/work-state/v1/database/SELECTED.json", ".handbook/work-state/v1/database/active.json", ".handbook/work-state/v1/database/maintenance.json"];
    let baseline_controls = paths.map(|path| (path, std::fs::read(dir.join(path)).unwrap()));
    let lock_paths = [".handbook/work-state/work.lock", ".handbook/work-state/v1/database/gate.lock"];
    let lock_identities = lock_paths.map(|path| { let m = std::fs::symlink_metadata(dir.join(path)).unwrap(); assert!(m.is_file()); assert!(!m.file_type().is_symlink()); (path, m.dev(), m.ino(), std::fs::read(dir.join(path)).unwrap()) });
    std::fs::write(evidence.join("lock-identities-before.json"), serde_json::to_vec(&lock_identities).unwrap()).unwrap();
    drop(sdk);
    {
        let root = handbook_engine::RetainedRepositoryRootV1::open_exact(&dir).unwrap();
        let _owner_lock = root.acquire_existing_exclusive_lock_v1(".handbook/work-state/work.lock").unwrap();
        let db = d04_native_store(&root, &namespace);
        let name = format!("item-{}", original.input.request_key);
        let route = format!("item/{}", original.item_identity);
        let target = format!("request/{name}");
        let before_value = db.get(&target).unwrap().unwrap();
        let route_before = db.get(&route).unwrap();
        assert_eq!(route_before, Some(name.as_bytes().to_vec()));
        let mut tx = file_native::Transaction::new(db.version().unwrap(), "d04-finite-header-deletion", Vec::new());
        tx.delete(&target).unwrap();
        std::fs::write(evidence.join("sole-transaction.json"), serde_json::to_vec(&tx).unwrap()).unwrap();
        let mutation = db.publish(tx).unwrap();
        assert_eq!(mutation.disposition, file_native::OriginalDisposition::Selected);
        assert!(db.get(&target).unwrap().is_none());
        let route_after = db.get(&route).unwrap();
        assert_eq!(route_after, route_before);
        let old = db.reconcile_evidence(&scope, Some(&native_key)).unwrap().original.unwrap();
        let old_json = serde_json::json!({"reference":old.reference,"outcome":old.outcome,"evidence":old.evidence});
        for key in ["reference", "outcome", "evidence"] { assert_eq!(old_json[key], native[key]); }
        std::fs::write(evidence.join("mutation.json"), serde_json::to_vec(&serde_json::json!({"target":target,"prior_value":before_value,"publication_outcome":mutation,"positive_route":route,"route_value_before":route_before,"route_value_after":route_after,"retained_original_native":old_json})).unwrap()).unwrap();
    }
    let post_injection = paths.map(|path| (path, std::fs::read(dir.join(path)).unwrap()));
    for i in 0..5 { assert_eq!(baseline_controls[i], post_injection[i]); }
    std::fs::write(evidence.join("controls-after-injection.json"), serde_json::to_vec(&post_injection).unwrap()).unwrap();
    let sdk = HandbookSdkV1::open(&dir);
    // One post-alteration historical consumption. Normal lease metadata is
    // permitted; no Work recovery/registration/restoration occurs.
    let retained = sdk.with_retained_work_snapshot(|snapshot| snapshot.original(&original)).unwrap();
    println!("retained_positive_witness_resolution={retained:?}");
    std::fs::write(evidence.join("retained-resolution.json"), serde_json::to_vec(&retained).unwrap()).unwrap();
    let after = paths.map(|path| (path, std::fs::read(dir.join(path)).unwrap()));
    std::fs::write(evidence.join("controls-after-retained-resolution.json"), serde_json::to_vec(&after).unwrap()).unwrap();
    assert_eq!(after, post_injection);
    for (path, dev, inode, bytes) in lock_identities {
        let m = std::fs::symlink_metadata(dir.join(path)).unwrap();
        assert_eq!((m.dev(), m.ino()), (dev, inode));
        assert_eq!(std::fs::read(dir.join(path)).unwrap(), bytes);
    }
    assert_eq!(std::fs::symlink_metadata(dir.join(".handbook/work-state-bootstrap")).unwrap_err().kind(), std::io::ErrorKind::NotFound);
    assert_eq!(std::fs::read(evidence.join("original.json")).unwrap(), canonical);
    assert_eq!(original.canonical_json_bytes().unwrap(), canonical);
    assert_eq!(std::fs::read(evidence.join("other-original.json")).unwrap(), other_canonical);
    assert_eq!(retained.disposition, WorkDispositionV1::Uncertain, "{retained:?}");
    assert!(retained.outcome.is_none());
    assert_eq!(retained.refusal.as_ref().unwrap().code, "work_item_history_missing");
}

#[test]
fn indexed_native_evidence_payload_substitution_refuses_without_outcome() {
    use std::io::Write;
    use std::os::unix::fs::MetadataExt;
    let evidence_override = std::env::var_os("D04_NATIVE_EVIDENCE_DIR");
    let evidence_temp = evidence_override.is_none().then(|| tempfile::tempdir().unwrap());
    let evidence = evidence_override.map(std::path::PathBuf::from).unwrap_or_else(|| evidence_temp.as_ref().unwrap().path().to_path_buf());
    let fixture = tempfile::tempdir().unwrap();
    let dir = if evidence_temp.is_some() { fixture.path().to_path_buf() } else { fixture.keep() };
    println!("native_evidence_fixture={}", dir.display());
    std::fs::write(evidence.join("fixture-path.txt"), dir.as_os_str().as_encoded_bytes()).unwrap();
    let sdk = HandbookSdkV1::open(&dir);
    let (initial, check) = setup(&sdk);
    let kind = establish_kind(&sdk, &initial, &check, "native-proof-kind", "project.native-proof@1.0.0");
    let grant = activate(&sdk, &initial, &check, "native-proof-grant", None);
    let mut originals = Vec::new();
    let mut canonicals = Vec::new();
    for (label, key) in [("a", "native-original-a"), ("b", "native-original-b")] {
        let prepared = sdk.prepare_work_registration(&initial, &grant, input(&grant, &kind, key)).unwrap();
        let original = prepared.locator().clone();
        let canonical = original.canonical_json_bytes().unwrap();
        let mut saved = std::fs::OpenOptions::new().write(true).create_new(true).open(evidence.join(format!("original-{label}.json"))).unwrap();
        saved.write_all(&canonical).unwrap(); saved.sync_all().unwrap();
        std::fs::File::open(&evidence).unwrap().sync_all().unwrap(); drop(saved);
        let committed = sdk.register_work_item(prepared);
        assert_eq!(committed.disposition, WorkDispositionV1::Committed, "{committed:?}");
        assert_eq!(committed.outcome.as_ref().unwrap().locator, original);
        let replay = sdk.resolve_work_registration_request(&original);
        assert_eq!(replay.disposition, WorkDispositionV1::Replayed, "{replay:?}");
        assert_eq!(replay.outcome, committed.outcome);
        std::fs::write(evidence.join(format!("baseline-{label}.json")), serde_json::to_vec(&serde_json::json!({"committed":committed,"replayed":replay})).unwrap()).unwrap();
        originals.push(original); canonicals.push(canonical);
    }
    let original = &originals[0];
    let native_a = NativeWorkStateV1::checkpoint_evidence_for_test(&dir, original, None).unwrap();
    let native_b = NativeWorkStateV1::checkpoint_evidence_for_test(&dir, &originals[1], None).unwrap();
    std::fs::write(evidence.join("native-a-before.json"), serde_json::to_vec(&native_a).unwrap()).unwrap();
    std::fs::write(evidence.join("native-b-before.json"), serde_json::to_vec(&native_b).unwrap()).unwrap();
    let namespace = native_a["evidence"]["binding"]["namespace"].as_str().unwrap().to_owned();
    assert_eq!(native_b["evidence"]["binding"]["namespace"], namespace);
    let scope_a = native_a["evidence"]["scope"].as_str().unwrap().to_owned();
    let scope_b = native_b["evidence"]["scope"].as_str().unwrap().to_owned();
    assert_ne!(scope_a, scope_b);
    let key_a = native_a["reference"]["key"].as_str().unwrap().to_owned();
    let key_b = native_b["reference"]["key"].as_str().unwrap().to_owned();
    // Only the public opaque payload Vec<u8> is projected; no WorkProof decode.
    let payload_b: Vec<u8> = serde_json::from_value(native_b["evidence"]["payload"].clone()).unwrap();
    assert_ne!(native_a["evidence"]["payload"], native_b["evidence"]["payload"]);
    let paths = [".handbook-work-state.v1.json", ".handbook-work-state-bootstrap.v1.json", ".handbook/work-state/v1/owner.json", ".handbook/work-state/v1/database/STORE.json", ".handbook/work-state/v1/database/READY.json", ".handbook/work-state/v1/database/SELECTED.json", ".handbook/work-state/v1/database/active.json", ".handbook/work-state/v1/database/maintenance.json"];
    let baseline_controls = paths.map(|path| (path, std::fs::read(dir.join(path)).unwrap()));
    std::fs::write(evidence.join("controls-before-publication.json"), serde_json::to_vec(&baseline_controls).unwrap()).unwrap();
    let lock_paths = [".handbook/work-state/work.lock", ".handbook/work-state/v1/database/gate.lock"];
    let lock_identities = lock_paths.map(|path| { let m = std::fs::symlink_metadata(dir.join(path)).unwrap(); assert!(m.is_file()); assert!(!m.file_type().is_symlink()); (path, m.dev(), m.ino(), std::fs::read(dir.join(path)).unwrap()) });
    std::fs::write(evidence.join("lock-identities-before.json"), serde_json::to_vec(&lock_identities).unwrap()).unwrap();
    drop(sdk);
    {
        let root = handbook_engine::RetainedRepositoryRootV1::open_exact(&dir).unwrap();
        let _owner_lock = root.acquire_existing_exclusive_lock_v1(".handbook/work-state/work.lock").unwrap();
        let db = d04_native_store(&root, &namespace);
        let name = format!("item-{}", original.input.request_key);
        let record_paths = [format!("request/{name}"), format!("record/{name}/intent.json"), format!("record/{name}/terminal.json")];
        let before_records = record_paths.each_ref().map(|key| (key.clone(), db.get(key).unwrap().expect("accepted A record")));
        let before_head = db.reconcile_evidence(&scope_a, None).unwrap().original.unwrap();
        let before_head_json = serde_json::json!({"reference":before_head.reference,"outcome":before_head.outcome,"evidence":before_head.evidence});
        for key in ["reference", "outcome", "evidence"] { assert_eq!(before_head_json[key], native_a[key]); }
        let version = db.version().unwrap();
        std::fs::write(evidence.join("head-before.json"), serde_json::to_vec(&serde_json::json!({"version":version,"head":before_head_json,"a_records":before_records})).unwrap()).unwrap();
        let mut tx = file_native::Transaction::new(version.clone(), "d04-native-evidence-substitution", Vec::new());
        tx.set_evidence(scope_a.clone(), payload_b.clone()).unwrap();
        let tx_json = serde_json::to_value(&tx).unwrap();
        assert!(tx_json["changes"].as_object().unwrap().is_empty());
        std::fs::write(evidence.join("sole-transaction.json"), serde_json::to_vec(&tx).unwrap()).unwrap();
        let publication = db.publish(tx).unwrap();
        assert_eq!(publication.disposition, file_native::OriginalDisposition::Selected);
        let after_records = record_paths.each_ref().map(|key| (key.clone(), db.get(key).unwrap().expect("unchanged accepted A record")));
        assert_eq!(after_records, before_records);
        let head = db.reconcile_evidence(&scope_a, None).unwrap();
        let new_original = head.original.unwrap();
        assert_eq!(new_original.reference.key, "d04-native-evidence-substitution");
        assert_eq!(new_original.outcome, publication);
        assert_eq!(new_original.evidence.scope, scope_a);
        assert_eq!(new_original.evidence.payload, payload_b);
        assert_eq!(new_original.evidence.binding.expected, version);
        assert_eq!(new_original.evidence.binding.predecessor, version);
        assert_eq!(new_original.evidence.binding.key, new_original.reference.key);
        assert_eq!(new_original.evidence.binding.fingerprint, new_original.reference.fingerprint);
        assert_eq!(serde_json::to_value(&new_original.evidence.previous).unwrap(), native_a["reference"]);
        let new_json = serde_json::json!({"reference":new_original.reference,"outcome":new_original.outcome,"evidence":new_original.evidence});
        let mut retained_originals = Vec::new();
        for (scope, key, baseline) in [(&scope_a, &key_a, &native_a), (&scope_b, &key_b, &native_b)] {
            let old = db.reconcile_evidence(scope, Some(key)).unwrap().original.unwrap();
            let old_json = serde_json::json!({"reference":old.reference,"outcome":old.outcome,"evidence":old.evidence});
            for field in ["reference", "outcome", "evidence"] { assert_eq!(old_json[field], baseline[field]); }
            retained_originals.push(old_json);
        }
        std::fs::write(evidence.join("publication.json"), serde_json::to_vec(&serde_json::json!({"publication":publication,"at":head.at,"new_original":new_json,"old_originals":retained_originals,"a_records_after":after_records})).unwrap()).unwrap();
    }
    let post_publication = paths.map(|path| (path, std::fs::read(dir.join(path)).unwrap()));
    for i in 0..5 { assert_eq!(baseline_controls[i], post_publication[i]); }
    std::fs::write(evidence.join("controls-after-publication.json"), serde_json::to_vec(&post_publication).unwrap()).unwrap();
    let sdk = HandbookSdkV1::open(&dir);
    // Exactly one post-alteration SDK action; no recover/register/restore follows.
    let resolved = sdk.resolve_work_registration_request(original);
    println!("native_evidence_resolution={resolved:?}");
    std::fs::write(evidence.join("resolution.json"), serde_json::to_vec(&resolved).unwrap()).unwrap();
    let after = paths.map(|path| (path, std::fs::read(dir.join(path)).unwrap()));
    std::fs::write(evidence.join("controls-after-resolution.json"), serde_json::to_vec(&after).unwrap()).unwrap();
    assert_eq!(after, post_publication);
    for (path, dev, inode, bytes) in lock_identities {
        let m = std::fs::symlink_metadata(dir.join(path)).unwrap();
        assert_eq!((m.dev(), m.ino()), (dev, inode));
        assert_eq!(std::fs::read(dir.join(path)).unwrap(), bytes);
    }
    assert_eq!(std::fs::symlink_metadata(dir.join(".handbook/work-state-bootstrap")).unwrap_err().kind(), std::io::ErrorKind::NotFound);
    for (i, label) in ["a", "b"].iter().enumerate() {
        assert_eq!(std::fs::read(evidence.join(format!("original-{label}.json"))).unwrap(), canonicals[i]);
        assert_eq!(originals[i].canonical_json_bytes().unwrap(), canonicals[i]);
    }
    assert_eq!(resolved.disposition, WorkDispositionV1::Uncertain, "{resolved:?}");
    assert!(resolved.outcome.is_none());
    assert_eq!(resolved.refusal.as_ref().unwrap().code, "work_native_proof");
}

#[test]
fn fresh_item_two_publications_preserve_exact_records_and_native_originals() {
    use sha2::{Digest, Sha256};
    use serde_json::{json, Value};
    let canonical = |v: &Value| serde_json_canonicalizer::to_vec(v).unwrap();
    let digest = |bytes: &[u8]| format!("sha256:{:x}", Sha256::digest(bytes));
    let fingerprint = |domain: &str, content: &Value| {
        digest(&canonical(&json!({"domain":domain,"content":content})))
    };
    let dir = tempfile::tempdir().unwrap();
    let sdk = HandbookSdkV1::open(dir.path());
    let (initial, check) = setup(&sdk);
    let grant = activate(&sdk, &initial, &check, "oracle-grant", None);
    let submitted = WorkRegistrationInputV1 {
        request_key: "oracle-original".into(), item_key: "oracle-item".into(),
        label: "independent label".into(), question: "independent question".into(),
        actor: grant.terms.agent.clone(), expected_state: "absent".into(),
        composition: WorkCompositionV1::ordinary_leaf().into(), display_level: None,
    };
    let prepared = sdk.prepare_work_registration(&initial, &grant, submitted).unwrap();
    let locator = prepared.locator().clone();
    let saved = locator.canonical_json_bytes().unwrap();
    let before = sdk.work_storage_statistics().unwrap();
    let base = dir.path().join(".handbook/work-state/v1/database");
    let binding: Value = serde_json::from_slice(&std::fs::read(base.join("STORE.json")).unwrap()).unwrap();
    let namespace = binding["namespace"].as_str().unwrap();
    let mut plans = Vec::new();
    let mut selected = Vec::new();
    let mut observations = Vec::new();
    let mut emitted = Vec::new();
    let result = NativeWorkStateV1::register_with_boundary_for_test(dir.path(), prepared, |at| {
        emitted.push(at);
        if at == B::NativeOwned {
            let active: Value = serde_json::from_slice(&std::fs::read(base.join("active.json")).unwrap()).unwrap();
            assert_eq!(active["state"], "owned");
            for id in active["value"]["positive"]["owned"].as_array().unwrap() {
                let id = id.as_str().unwrap();
                assert!(!base.join(format!("objects/{}/{}/{}", &id[..2], &id[2..4], id)).exists());
            }
            plans.push(active["value"].clone());
        }
        if at == B::NativeOutputs {
            let plan = plans.last().unwrap();
            for id in plan["positive"]["owned"].as_array().unwrap() {
                let id = id.as_str().unwrap();
                assert!(base.join(format!("objects/{}/{}/{}", &id[..2], &id[2..4], id)).is_file());
            }
        }
        if matches!(at, B::Intent | B::Decision | B::Snapshot | B::Terminal) {
            let selector: Value = serde_json::from_slice(&std::fs::read(base.join("SELECTED.json")).unwrap()).unwrap();
            let active: Value = serde_json::from_slice(&std::fs::read(base.join("active.json")).unwrap()).unwrap();
            assert_eq!(active["state"], "idle");
            assert_eq!(active["value"], selector);
            let root = handbook_engine::RetainedRepositoryRootV1::open_exact(dir.path()).unwrap();
            let db = d04_native_store(&root, namespace);
            let snapshot = db.snapshot().unwrap();
            let mut records = std::collections::BTreeMap::new();
            let mut after = None;
            loop {
                let page = snapshot.page(after.as_deref(), 16).unwrap();
                records.extend(page.records);
                if page.complete { break; }
                after = page.next;
            }
            selected.push(selector);
            observations.push(records);
        }
        Ok(())
    });
    assert_eq!(result.disposition, WorkDispositionV1::Committed, "{result:?}");
    assert_eq!(plans.len(), 2);
    assert_eq!(selected.len(), 4);
    assert_eq!(selected[0], selected[1]);
    assert_eq!(selected[1], selected[2]); // I/D/S are observations of the SAME R.
    for removed in [B::Reservation, B::AcceptanceCandidate, B::StagedIntent] {
        assert!(!emitted.contains(&removed));
    }
    let name = "item-oracle-original";
    let r = &observations[0];
    let t = &observations[3];
    let read = |rows: &std::collections::BTreeMap<String, Vec<u8>>, file: &str| {
        rows[&format!("record/{name}/{file}")].clone()
    };
    let intent_bytes = read(r, "intent.json");
    let intent: Value = serde_json::from_slice(&intent_bytes).unwrap();
    assert_eq!(canonical(&intent), intent_bytes);
    assert_eq!(intent["schema"], "handbook.work-intent@2.0.0");
    assert_eq!(intent["operation"], json!({"operation":"item","locator":locator}));
    let owner: Value = serde_json::from_slice(&std::fs::read(dir.path().join(".handbook/work-state/v1/owner.json")).unwrap()).unwrap();
    assert_eq!(intent["owner_identity"], owner["identity"]);
    let descriptor = serde_json::to_value(WorkDescriptorV1::ordinary_leaf()).unwrap();
    let descriptor_bytes = canonical(&descriptor);
    let expected_snapshot = json!({
        "version":"3","project_identity":locator.grant.check.predecessor.charter.binding.initialization_claim.proposed_repository_identity(),"item_identity":locator.item_identity,
        "item_key":locator.input.item_key,"label":locator.input.label,"question":locator.input.question,
        "state":"open",
        "descriptor":descriptor,"descriptor_bytes":descriptor_bytes,
        "descriptor_fingerprint":digest(&descriptor_bytes),
        "registration_request_fingerprint":locator.fingerprint
    });
    let receipt_fp = fingerprint("handbook.work-registration-receipt@2.0.0",
        &json!([locator,expected_snapshot,"register_open_leaf","local_advisory_claims"]));
    let receipt = json!({"locator":locator,"snapshot":expected_snapshot,"effect":"register_open_leaf",
        "assurance":"local_advisory_claims","receipt_fingerprint":receipt_fp});
    assert_eq!(serde_json::to_value(result.outcome.as_ref().unwrap()).unwrap(), receipt);
    let output = json!({"kind":"item","value":receipt});
    let decision = json!({"intent_fingerprint":fingerprint("handbook.work-intent@2.0.0",&intent),"output":output});
    let terminal = json!({"decision_fingerprint":fingerprint("handbook.work-decision@2.0.0",&decision),"effect":"committed"});
    let publication = json!({"intent_fingerprint":decision["intent_fingerprint"],"target":"terminal.json"});
    for (file, expected) in [
        ("decision.json",canonical(&decision)),("snapshot.json",canonical(&output)),
        ("not-attempted.json",canonical(&publication)),("terminal.pending",canonical(&terminal)),
    ] { assert_eq!(read(r,file), expected, "{file}"); }
    for (file, expected) in [
        ("intent.json",intent_bytes.clone()),("decision.json",canonical(&decision)),
        ("snapshot.json",canonical(&output)),("attempted.json",canonical(&publication)),
        ("terminal.json",canonical(&terminal)),
    ] { assert_eq!(read(t,file), expected, "{file}"); }
    for (rows, files) in [
        (r,vec!["decision.json","intent.json","not-attempted.json","snapshot.json","terminal.pending"]),
        (t,vec!["attempted.json","decision.json","intent.json","snapshot.json","terminal.json"]),
    ] {
        let header: Value = serde_json::from_slice(&rows[&format!("request/{name}")]).unwrap();
        assert_eq!(header,json!({"name":name,"staging":false,"intent_fingerprint":digest(&intent_bytes),"files":files}));
        let actual: Vec<_> = rows.keys().filter_map(|k|k.strip_prefix(&format!("record/{name}/"))).collect();
        assert_eq!(actual,files);
        assert_eq!(rows[&format!("item/{}",locator.item_identity)],name.as_bytes());
    }
    let after = sdk.work_storage_statistics().unwrap();
    assert_eq!(after.database_publications.checked_sub(before.database_publications),Some(2));
    assert_eq!(after.live_records.checked_sub(before.live_records),Some(1));
    assert_eq!(after.retained_original_outcomes.checked_sub(before.retained_original_outcomes),Some(1));
    let bytes: u64 = t.iter().filter(|(k,_)|k.starts_with(&format!("record/{name}/"))).map(|(_,b)|b.len() as u64).sum();
    assert_eq!(after.retained_record_bytes.checked_sub(before.retained_record_bytes),Some(bytes));
    let mut originals = Vec::new();
    for (i, role) in ["prefix-not-attempted","terminal-attempt"].iter().enumerate() {
        let key = format!("{name}:{}",before.database_publications+i as u64);
        let original = NativeWorkStateV1::checkpoint_evidence_for_test(dir.path(),&locator,Some(&key)).unwrap();
        assert_eq!(original["reference"]["key"],key);
        assert_eq!(original["outcome"]["disposition"],"Selected");
        assert_eq!(original["evidence"]["binding"]["expected"]["sequence"],before.database_publications+i as u64);
        assert_eq!(original["evidence"]["binding"]["candidate"]["sequence"],before.database_publications+i as u64+1);
        let payload: Vec<u8> = serde_json::from_value(original["evidence"]["payload"].clone()).unwrap();
        let checkpoint: Value = serde_json::from_slice(&payload).unwrap();
        assert_eq!(checkpoint["checkpoint_role"],*role);
        assert_eq!(checkpoint["work_intent_identity"],fingerprint("handbook.work-intent@2.0.0",&intent));
        assert_eq!(checkpoint["work_intent_bytes_fingerprint"],digest(&intent_bytes));
        let sequence = before.database_publications+i as u64+1;
        let h = format!("{sequence:016x}");
        let event: Value = serde_json::from_slice(&std::fs::read(base.join(format!("retirements/{}/{}/{}/{}/{}/{}/{}/{}",&h[..2],&h[2..4],&h[4..6],&h[6..8],&h[8..10],&h[10..12],&h[12..14],h))).unwrap()).unwrap();
        assert_eq!(event,plans[i]["positive"]["event"]);
        assert_eq!(event["sequence"],sequence);
        assert_eq!(original["evidence"]["binding"]["candidate"]["token"],plans[i]["positive"]["candidate"]["token"]);
        originals.push(original);
    }
    assert_eq!(originals[1]["evidence"]["previous"],originals[0]["reference"]);
    assert_eq!(plans[1]["prior"],selected[0]);
    println!("TASK13_INDEPENDENT_ORACLE {}",json!({"saved":serde_json::from_slice::<Value>(&saved).unwrap(),"receipt":receipt,"R":r,"T":t,"plans":plans,"originals":originals,"before":before,"after":after,"retained_item_bytes":bytes}));
    while !sdk.maintain_work_storage(WorkMaintenanceBudgetV1::default()).unwrap().complete {}
    drop(sdk);
    let sdk = HandbookSdkV1::open(dir.path());
    assert_eq!(sdk.resolve_work_registration_request(&locator).outcome,result.outcome);
    assert_eq!(locator.canonical_json_bytes().unwrap(),saved);
    for original in originals {
        let key = original["reference"]["key"].as_str().unwrap();
        let retained = NativeWorkStateV1::checkpoint_evidence_for_test(dir.path(),&locator,Some(key)).unwrap();
        assert_eq!(retained["reference"],original["reference"]);
        assert_eq!(retained["outcome"],original["outcome"]);
        assert_eq!(retained["evidence"],original["evidence"]);
    }
}

#[test]
fn fresh_item_native_r_and_t_interruptions_preserve_acceptance_truth() {
    for publication in [1,2] {
        for at in [B::NativeBeforeOwnership,B::NativeOwned,B::NativeOutputs,B::NativeBeforeSelection,B::NativeSelectorDisplaced] {
            for lose_authority in [false,true] {
                let dir = tempfile::tempdir().unwrap();
                let sdk = HandbookSdkV1::open(dir.path());
                let (initial,check) = setup(&sdk);
                let kind = establish_kind(&sdk,&initial,&check,"interrupt-kind","project.interrupt@1.0.0");
                let grant = activate(&sdk,&initial,&check,"interrupt-grant",None);
                let submitted = input(&grant,&kind,"interrupt-original");
                let prepared = sdk.prepare_work_registration(&initial,&grant,submitted.clone()).unwrap();
                let locator = prepared.locator().clone();
                let canonical = locator.canonical_json_bytes().unwrap();
                let mut observed = 0;
                let interrupted = NativeWorkStateV1::register_with_boundary_for_test(dir.path(),prepared,|point| {
                    if point==at {
                        observed+=1;
                        if observed==publication {
                            return Err(LocalAuthorityRefusalV1{code:"task13_native_interruption".into(),detail:format!("{publication}:{at:?}"),unresolved_request:None});
                        }
                    }
                    Ok(())
                });
                assert_eq!(observed,publication);
                assert_eq!(interrupted.disposition,WorkDispositionV1::Uncertain);
                let resolved = sdk.resolve_work_registration_request(&locator);
                let expected = if publication==1 { WorkDispositionV1::Unaccepted } else { WorkDispositionV1::Pending };
                assert_eq!(resolved.disposition,expected,"{publication} {at:?}: {resolved:?}");
                assert!(resolved.outcome.is_none());
                if lose_authority {
                    assert!(sdk.run_setup(&SetupRequest{mode:SetupMode::Refresh,rewrite:false,reset_state:true}).unwrap().reset_applied);
                }
                let recovered = sdk.recover_work_registration_request(&locator);
                let expected = if publication==1 { WorkDispositionV1::Unaccepted } else if lose_authority { WorkDispositionV1::NonCommitted } else { WorkDispositionV1::Committed };
                assert_eq!(recovered.disposition,expected,"{publication} {at:?} authorityloss={lose_authority}: {recovered:?}");
                if publication==1 && !lose_authority {
                    let retry = sdk.prepare_work_registration(&initial,&grant,submitted).unwrap();
                    assert_eq!(retry.locator().canonical_json_bytes().unwrap(),canonical);
                    assert_eq!(sdk.register_work_item(retry).disposition,WorkDispositionV1::Committed);
                }
                assert_eq!(locator.canonical_json_bytes().unwrap(),canonical);
                println!("TASK13_NATIVE_CUT publication={publication} at={at:?} authorityloss={lose_authority} resolved={:?} recovered={:?}",resolved.disposition,recovered.disposition);
            }
        }
    }
}

#[test]
#[ignore = "requires four exact-baseline native fixtures; run explicitly with TASK13_LEGACY_FIXTURES"]
fn old_item_prefixes_recover_without_rewriting_originals() {
    use serde_json::{json, Value};
    let parent=std::path::PathBuf::from(std::env::var_os("TASK13_LEGACY_FIXTURES").expect("exact-baseline fixture parent"));
    let manifest:Vec<Value>=serde_json::from_slice(&std::fs::read(parent.join("manifest.json")).unwrap()).unwrap();
    assert_eq!(manifest.len(),4);
    for (index,entry) in manifest.into_iter().enumerate() {
        let path=std::path::PathBuf::from(entry["path"].as_str().unwrap());
        assert_eq!(path,parent.join(format!("prefix-{index}")));
        let canonical:Vec<u8>=serde_json::from_value(entry["canonical"].clone()).unwrap();
        let locator=WorkRequestLocatorV1::from_json_bytes(&canonical).unwrap();
        assert_eq!(serde_json::to_value(&locator).unwrap(),entry["locator"]);
        assert_eq!(locator.canonical_json_bytes().unwrap(),canonical);
        let base=path.join(".handbook/work-state/v1/database");
        let store:Value=serde_json::from_slice(&std::fs::read(base.join("STORE.json")).unwrap()).unwrap();
        {
            let root=handbook_engine::RetainedRepositoryRootV1::open_exact(&path).unwrap();
            let db=d04_native_store(&root,store["namespace"].as_str().unwrap());
            let header:Value=serde_json::from_slice(&db.get("request/item-old-original").unwrap().unwrap()).unwrap();
            let expected_files=match index {
                0=>vec!["intent.json"],
                1=>vec!["intent.json","not-attempted.json"],
                2=>vec!["decision.json","intent.json","not-attempted.json"],
                _=>vec!["decision.json","intent.json","not-attempted.json","snapshot.json","terminal.pending"],
            };
            assert_eq!(header["staging"],index==0);
            assert_eq!(header["files"],json!(expected_files));
            assert_eq!(db.version().unwrap().sequence,
                entry["after"]["database_publications"].as_u64().unwrap());
        }
        let sdk=HandbookSdkV1::open(&path);
        assert_eq!(sdk.resolve_work_registration_request(&locator).disposition,WorkDispositionV1::Pending);
        assert_eq!(entry["originals"].as_array().unwrap().len(),index+1);
        for original in entry["originals"].as_array().unwrap() {
            let retained=NativeWorkStateV1::checkpoint_evidence_for_test(&path,&locator,Some(original["reference"]["key"].as_str().unwrap())).unwrap();
            assert_eq!(retained["reference"],original["reference"]);
            assert_eq!(retained["outcome"],original["outcome"]);
            assert_eq!(retained["evidence"],original["evidence"]);
        }
        if index==0 {
            assert!(sdk.run_setup(&SetupRequest{mode:SetupMode::Refresh,rewrite:false,reset_state:true}).unwrap().reset_applied);
        }
        let result=sdk.recover_work_registration_request(&locator);
        assert_eq!(result.disposition,
            if index==0 {WorkDispositionV1::Uncertain}else{WorkDispositionV1::Committed},
            "old prefix {index}: {result:?}");
        while !sdk.maintain_work_storage(WorkMaintenanceBudgetV1::default()).unwrap().complete {}
        drop(sdk);
        let sdk=HandbookSdkV1::open(&path);
        assert_eq!(sdk.resolve_work_registration_request(&locator).disposition,
            if index==0 {WorkDispositionV1::Pending}else{WorkDispositionV1::Replayed});
        for original in entry["originals"].as_array().unwrap() {
            let retained=NativeWorkStateV1::checkpoint_evidence_for_test(&path,&locator,Some(original["reference"]["key"].as_str().unwrap())).unwrap();
            assert_eq!(retained["reference"],original["reference"]);
            assert_eq!(retained["outcome"],original["outcome"]);
            assert_eq!(retained["evidence"],original["evidence"]);
        }
        assert_eq!(locator.canonical_json_bytes().unwrap(),canonical);
        println!("TASK13_OLD_PREFIX_RECOVERED index={index} boundary={} result={:?} old_originals={}",entry["boundary"],result.disposition,index+1);
    }
}
