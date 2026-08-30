#[path = "support/hcm_4_6_snapshot_support.rs"]
mod support;

use handbook_sdk::contract_membrane::{
    canonical_json_bytes, ContractLimits, SessionGroundingBoundaryKindV1,
    SessionGroundingBoundaryRequestV1, SessionGroundingBoundaryUseCaseOutcomeV1,
    SessionGroundingRecoveryActionV1, SessionGroundingTerminalV1, TerminalStatus,
};
use handbook_sdk::HandbookSdkV1;
use serde_json::Value;
use support::{capture_body, operation_invocation, target_body, workspace, CompleteSnapshotSource};

#[test]
fn direct_sdk_preserves_all_four_operation_results_replay_receipt_resolution_and_handoff() {
    let repo = tempfile::tempdir().unwrap();
    let sdk = HandbookSdkV1::open(repo.path());
    let limits = ContractLimits::packet0();
    let mut source = CompleteSnapshotSource::new();
    let start = operation_invocation(
        "snapshot.capture@1.0.0",
        capture_body("sdk-packet-6", "start", "sdk-packet-6-start-key"),
    );
    let started = sdk
        .invoke_with_snapshot_source(&start, &limits, &mut source)
        .unwrap();
    let started_value = serde_json::to_value(&started).unwrap();
    assert_eq!(started.terminal_status(), TerminalStatus::Ok);
    assert_eq!(started_value["response"]["data"]["outcome"], "started");
    assert_eq!(
        started_value["response"]["data"]["currentness"],
        "owner_derived_current"
    );
    assert_eq!(
        started_value["response"]["data"]["predecessor_outcome"],
        "no_applicable_prior"
    );
    assert_eq!(started_value["response"]["data"]["replayed"], false);
    assert_eq!(
        started_value["response"]["receipts"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    assert_eq!(source.observed.len(), 42);

    source.reset_counters();
    source.fail_all_observations = true;
    let replay = sdk
        .invoke_with_snapshot_source(&start, &limits, &mut source)
        .unwrap();
    let replay_value = serde_json::to_value(&replay).unwrap();
    assert_eq!(replay_value["response"]["data"]["replayed"], true);
    assert_eq!(
        replay_value["response"]["data"]["receipt"],
        started_value["response"]["data"]["receipt"]
    );
    assert_eq!(
        replay_value["response"]["data"]["recovery_action"],
        "accepted_idempotent_replay"
    );
    assert!(source.observed.is_empty());
    assert!(source.reads.is_empty());

    let target = target_body(&started_value);
    for operation_ref in [
        "snapshot.read@1.0.0",
        "snapshot.delta@1.0.0",
        "snapshot.project@1.0.0",
    ] {
        let request = operation_invocation(operation_ref, target.clone());
        let response = sdk.invoke(&request, &limits).unwrap();
        assert_eq!(
            response.terminal_status(),
            TerminalStatus::Ok,
            "{operation_ref}"
        );
        let value = serde_json::to_value(&response).unwrap();
        if operation_ref == "snapshot.read@1.0.0" {
            assert_eq!(value["response"]["data"]["descriptor"]["current"], true);
        } else if operation_ref == "snapshot.delta@1.0.0" {
            assert_eq!(value["response"]["data"]["outcome"], "absent");
            assert_eq!(value["response"]["data"]["current"], false);
        } else {
            assert_eq!(value["response"]["data"]["current"], true);
        }
        if operation_ref == "snapshot.project@1.0.0" {
            assert_eq!(
                value["response"]["data"]["resolution_stack"]["ref"],
                "handbook.context-resolution-stack.packet-6@1.0.0"
            );
            assert_eq!(
                value["response"]["data"]["omissions"]
                    .as_array()
                    .unwrap()
                    .len(),
                3
            );
            assert!(value["response"]["data"]["refusals"]
                .as_array()
                .unwrap()
                .is_empty());
        }
        sdk.readmit_invocation_response(&request, &response, &limits)
            .unwrap();
        assert_public(&value, repo.path());
    }

    source.fail_all_observations = false;
    source.advance_handoff("handoff/packet-6-sdk");
    let end = operation_invocation(
        "snapshot.capture@1.0.0",
        capture_body("sdk-packet-6", "end", "sdk-packet-6-end-key"),
    );
    let finalized = sdk
        .invoke_with_snapshot_source(&end, &limits, &mut source)
        .unwrap();
    let finalized_value = serde_json::to_value(&finalized).unwrap();
    assert_eq!(finalized_value["response"]["data"]["outcome"], "finalized");
    assert_eq!(
        finalized_value["response"]["data"]["handoff_refs"],
        serde_json::json!(["handoff/packet-6-sdk"])
    );
    assert!(finalized_value["response"]["data"]["delta"].is_object());
    assert_public(&finalized_value, repo.path());

    let delta = sdk
        .invoke(
            &operation_invocation("snapshot.delta@1.0.0", target_body(&finalized_value)),
            &limits,
        )
        .unwrap();
    let delta_value = serde_json::to_value(delta).unwrap();
    assert_eq!(delta_value["response"]["data"]["outcome"], "found");
    assert_eq!(
        delta_value["response"]["data"]["changed"].as_u64().unwrap()
            + delta_value["response"]["data"]["unchanged"]
                .as_u64()
                .unwrap(),
        15
    );
}

#[test]
fn direct_fixed_use_case_preserves_final_delta_handoff_and_recomposes_exact_replay_without_capture()
{
    let repo = tempfile::tempdir().unwrap();
    let sdk = HandbookSdkV1::open(repo.path());
    let limits = ContractLimits::packet0();
    let mut source = CompleteSnapshotSource::new();
    let start = SessionGroundingBoundaryRequestV1::new(
        workspace(),
        "use-case-packet-6",
        SessionGroundingBoundaryKindV1::Start,
        "use-case-packet-6-start-key",
    )
    .unwrap();
    let started = sdk
        .run_session_grounding_boundary_use_case(&start, &mut source, &limits)
        .unwrap();
    let Some(SessionGroundingBoundaryUseCaseOutcomeV1::Composed(started)) =
        started.session_grounding_boundary_outcome(&limits).unwrap()
    else {
        panic!("start must compose")
    };
    assert_eq!(started.terminal(), SessionGroundingTerminalV1::Started);

    source.advance_handoff("handoff/packet-6-use-case");
    let end = SessionGroundingBoundaryRequestV1::new(
        workspace(),
        "use-case-packet-6",
        SessionGroundingBoundaryKindV1::End,
        "use-case-packet-6-end-key",
    )
    .unwrap();
    let finalized_response = sdk
        .run_session_grounding_boundary_use_case(&end, &mut source, &limits)
        .unwrap();
    let Some(SessionGroundingBoundaryUseCaseOutcomeV1::Composed(finalized)) = finalized_response
        .session_grounding_boundary_outcome(&limits)
        .unwrap()
    else {
        panic!("end must compose")
    };
    assert_eq!(finalized.terminal(), SessionGroundingTerminalV1::Finalized);
    assert_eq!(finalized.flow_status(), finalized.terminal());
    assert_eq!(finalized.pipeline_status(), finalized.terminal());
    assert!(finalized.delta_record().is_some());
    assert_eq!(
        finalized
            .handoff_refs()
            .iter()
            .map(|value| value.as_str())
            .collect::<Vec<_>>(),
        vec!["handoff/packet-6-use-case"]
    );
    let receipt = finalized.receipt_record().clone();
    let delta = finalized.delta_record().cloned();

    source.reset_counters();
    source.fail_all_observations = true;
    let replayed_response = sdk
        .run_session_grounding_boundary_use_case(&end, &mut source, &limits)
        .unwrap();
    let Some(SessionGroundingBoundaryUseCaseOutcomeV1::Composed(replayed)) = replayed_response
        .session_grounding_boundary_outcome(&limits)
        .unwrap()
    else {
        panic!("exact replay must purely recompose")
    };
    assert!(replayed.replayed());
    assert_eq!(replayed.receipt_record(), &receipt);
    assert_eq!(replayed.delta_record(), delta.as_ref());
    assert_eq!(
        replayed
            .handoff_refs()
            .iter()
            .map(|value| value.as_str())
            .collect::<Vec<_>>(),
        vec!["handoff/packet-6-use-case"]
    );
    assert_eq!(
        replayed.recovery_action(),
        SessionGroundingRecoveryActionV1::DeliveryRecoveredByExactReplay
    );
    assert!(source.observed.is_empty());
    assert!(source.reads.is_empty());
}

fn assert_public(value: &Value, root: &std::path::Path) {
    let wire = canonical_json_bytes(value, &ContractLimits::packet0()).unwrap();
    let text = String::from_utf8(wire).unwrap();
    for forbidden in [
        root.to_string_lossy().as_ref(),
        "sdk-packet-6-start-key",
        "sdk-packet-6-end-key",
        "handbook.snapshot-observation.",
    ] {
        assert!(
            !text.contains(forbidden),
            "public response leaked {forbidden}"
        );
    }
}
