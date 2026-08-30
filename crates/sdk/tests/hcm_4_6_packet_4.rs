use handbook_engine::{
    DefinitionFingerprint, SnapshotGroundingOwnerV1, SnapshotSlotObservationV1,
    SnapshotSourceFamilyV1, SnapshotSourcePortErrorV1, SnapshotSourcePortV1,
};
use handbook_sdk::contract_membrane::{
    canonical_json_bytes, operation_inventory, ContractLimits, ExactBinding, ExactRef, Fingerprint,
    ProductionWitness, SessionGroundingBoundaryKindV1, SessionGroundingBoundaryRequestV1,
    SessionGroundingBoundaryUseCaseOutcomeV1, SessionGroundingPredecessorV1,
    SessionGroundingRecoveryActionV1, SessionGroundingTerminalV1, UseCaseRequestBodyV1,
    UseCaseRequestV1, UtcTimestamp,
};
use handbook_sdk::HandbookSdkV1;
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use tempfile::tempdir;

fn workspace() -> ExactBinding {
    ExactBinding::new(
        ExactRef::parse("handbook.workspace.packet-4-test@1.0.0").unwrap(),
        Fingerprint::parse(
            "sha256:4444444444444444444444444444444444444444444444444444444444444444",
        )
        .unwrap(),
    )
}

#[test]
fn fixed_session_boundary_request_is_exact_closed_canonical_and_debug_redacted() {
    let limits = ContractLimits::packet0();
    for kind in [
        SessionGroundingBoundaryKindV1::Start,
        SessionGroundingBoundaryKindV1::End,
    ] {
        let details = SessionGroundingBoundaryRequestV1::new(
            workspace(),
            "session-packet-4",
            kind,
            "owner-secret-key",
        )
        .unwrap();
        assert!(!format!("{details:?}").contains("owner-secret-key"));
        let request = UseCaseRequestV1::session_grounding_boundary(details, &limits).unwrap();
        assert_eq!(
            request.use_case().reference().as_str(),
            "handbook.use-case.session-grounding-boundary@1.0.0"
        );
        assert!(matches!(
            request.request(),
            UseCaseRequestBodyV1::SessionGroundingBoundary(_)
        ));
        let bytes = canonical_json_bytes(&request, &limits).unwrap();
        let admitted = UseCaseRequestV1::parse_and_admit(&bytes, &limits).unwrap();
        assert_eq!(admitted, request);
    }
}

#[test]
fn start_replay_and_final_end_preserve_exact_owner_flow_and_pipeline_truth() {
    let repo = tempdir().unwrap();
    let sdk = HandbookSdkV1::open(repo.path());
    let limits = ContractLimits::packet0();
    let mut port = CompletePort::successful();
    let start = SessionGroundingBoundaryRequestV1::new(
        workspace(),
        "session-packet-4",
        SessionGroundingBoundaryKindV1::Start,
        "start-owner-key",
    )
    .unwrap();

    let start_response = sdk
        .run_session_grounding_boundary_use_case(&start, &mut port, &limits)
        .unwrap();
    let Some(SessionGroundingBoundaryUseCaseOutcomeV1::Composed(start_result)) = start_response
        .session_grounding_boundary_outcome(&limits)
        .unwrap()
    else {
        panic!("start must compose")
    };
    assert_eq!(start_result.terminal(), SessionGroundingTerminalV1::Started);
    assert_eq!(start_result.flow_status(), start_result.terminal());
    assert_eq!(start_result.pipeline_status(), start_result.terminal());
    assert_eq!(start_result.sequence(), 1);
    assert_eq!(
        start_result.predecessor(),
        &SessionGroundingPredecessorV1::NoApplicablePrior
    );
    assert!(start_result.delta_record().is_none());
    assert!(!start_result.replayed());
    assert_eq!(
        start_result.recovery_action(),
        SessionGroundingRecoveryActionV1::None
    );
    let receipt = start_result.receipt_record().clone();
    assert_eq!(port.observed.len(), 42);
    assert_eq!(port.reads.len(), 11);

    port.reset_counters();
    port.fail_all_observations = true;
    let replay_response = sdk
        .run_session_grounding_boundary_use_case(&start, &mut port, &limits)
        .unwrap();
    let Some(SessionGroundingBoundaryUseCaseOutcomeV1::Composed(replay)) = replay_response
        .session_grounding_boundary_outcome(&limits)
        .unwrap()
    else {
        panic!("exact replay must recompose")
    };
    assert!(replay.replayed());
    assert_eq!(replay.receipt_record(), &receipt);
    assert_eq!(
        replay.recovery_action(),
        SessionGroundingRecoveryActionV1::DeliveryRecoveredByExactReplay
    );
    assert!(port.observed.is_empty());
    assert!(port.reads.is_empty());

    port.fail_all_observations = false;
    port.advance(
        SnapshotSourceFamilyV1::Work,
        "handoff",
        "handbook.snapshot-observation.work.handoff-packet-4@1.0.0",
        json!({"refs":["handoff/ref-packet-4"]}),
    );
    let end = SessionGroundingBoundaryRequestV1::new(
        workspace(),
        "session-packet-4",
        SessionGroundingBoundaryKindV1::End,
        "end-owner-key",
    )
    .unwrap();
    let end_response = sdk
        .run_session_grounding_boundary_use_case(&end, &mut port, &limits)
        .unwrap();
    let Some(SessionGroundingBoundaryUseCaseOutcomeV1::Composed(final_result)) = end_response
        .session_grounding_boundary_outcome(&limits)
        .unwrap()
    else {
        panic!("end must compose")
    };
    assert_eq!(
        final_result.terminal(),
        SessionGroundingTerminalV1::Finalized
    );
    assert_eq!(final_result.flow_status(), final_result.terminal());
    assert_eq!(final_result.pipeline_status(), final_result.terminal());
    assert_eq!(final_result.sequence(), 2);
    assert!(matches!(
        final_result.predecessor(),
        SessionGroundingPredecessorV1::OpenStart { .. }
    ));
    assert!(final_result.delta_record().is_some());
    assert_eq!(
        final_result
            .handoff_refs()
            .iter()
            .map(|value| value.as_str())
            .collect::<Vec<_>>(),
        vec!["handoff/ref-packet-4"]
    );

    for response in [start_response, replay_response, end_response] {
        let bytes = canonical_json_bytes(&response, &limits).unwrap();
        let admitted =
            serde_json::from_slice::<handbook_sdk::contract_membrane::UseCaseResponseV1>(&bytes)
                .unwrap();
        assert_eq!(admitted, response);
    }
}

#[test]
fn owner_failure_is_write_nothing_and_fixed_use_case_stays_out_of_discovery() {
    const USE_CASE_REF: &str = "handbook.use-case.session-grounding-boundary@1.0.0";
    let repo = tempdir().unwrap();
    let sdk = HandbookSdkV1::open(repo.path());
    let limits = ContractLimits::packet0();
    let mut port = CompletePort::successful();
    port.fail_all_observations = true;
    let request = SessionGroundingBoundaryRequestV1::new(
        workspace(),
        "failed-session-packet-4",
        SessionGroundingBoundaryKindV1::Start,
        "failed-owner-key",
    )
    .unwrap();
    let response = sdk
        .run_session_grounding_boundary_use_case(&request, &mut port, &limits)
        .unwrap();
    let Some(SessionGroundingBoundaryUseCaseOutcomeV1::OwnerFailed(failure)) = response
        .session_grounding_boundary_outcome(&limits)
        .unwrap()
    else {
        panic!("unavailable source must return the typed owner failure")
    };
    assert_eq!(failure.error_class(), "unavailable");
    assert_eq!(failure.mutation_effect(), "write_nothing");
    assert_eq!(
        failure.recovery_action(),
        SessionGroundingRecoveryActionV1::RecheckSameRequest
    );
    assert!(serde_json::to_value(&response).unwrap()["receipts"]
        .as_array()
        .unwrap()
        .is_empty());

    assert!(operation_inventory()
        .iter()
        .all(|row| row.operation_ref().as_str() != USE_CASE_REF));
    let witness =
        ProductionWitness::from_current(UtcTimestamp::parse("2030-01-01T00:00:00Z").unwrap())
            .unwrap();
    assert!(witness
        .capabilities()
        .iter()
        .all(|entry| entry.operation().reference().as_str() != USE_CASE_REF));
    assert!(witness.schema_manifest().iter().all(|entry| !entry
        .schema()
        .reference()
        .as_str()
        .contains("session-grounding-boundary")));
}

struct CompletePort {
    observations: BTreeMap<(SnapshotSourceFamilyV1, &'static str), SnapshotSlotObservationV1>,
    values: BTreeMap<(SnapshotSourceFamilyV1, &'static str), Value>,
    observed: Vec<(SnapshotSourceFamilyV1, &'static str)>,
    reads: BTreeSet<(SnapshotSourceFamilyV1, &'static str)>,
    fail_all_observations: bool,
}

impl CompletePort {
    fn successful() -> Self {
        let owner = SnapshotGroundingOwnerV1::new();
        let mut observations = BTreeMap::new();
        let mut values = BTreeMap::new();
        for contract in owner.source_contracts() {
            for &slot in contract.source_slots() {
                let reference = format!(
                    "handbook.snapshot-observation.{}.{}@1.0.0",
                    contract.family().as_str(),
                    slot.replace('_', "-")
                );
                let value = json!({"ref":reference});
                observations.insert(
                    (contract.family(), slot),
                    SnapshotSlotObservationV1::retained(&reference, &fingerprint(value.clone()))
                        .unwrap(),
                );
                values.insert((contract.family(), slot), value);
            }
        }
        observations.insert(
            (SnapshotSourceFamilyV1::Git, "worktree"),
            SnapshotSlotObservationV1::redacted(
                "handbook.snapshot-observation.git.worktree@1.0.0",
                &fingerprint(json!({"redacted":true})),
            )
            .unwrap(),
        );
        observations.insert(
            (SnapshotSourceFamilyV1::Evidence, "dock_execution_candidate"),
            SnapshotSlotObservationV1::omitted("absent_or_inapplicable"),
        );
        observations.insert(
            (SnapshotSourceFamilyV1::Work, "handoff"),
            SnapshotSlotObservationV1::inapplicable("policy_inapplicable"),
        );
        let contracts_decision = values
            .get(&(SnapshotSourceFamilyV1::Evidence, "contracts_decision"))
            .unwrap()
            .clone();
        observations.insert(
            (SnapshotSourceFamilyV1::Evidence, "contracts_decision"),
            SnapshotSlotObservationV1::derived(
                "handbook.snapshot-observation.evidence.contracts-decision@1.0.0",
                &fingerprint(contracts_decision),
            )
            .unwrap(),
        );
        let resolution = json!({
            "resolution": {
                "stack_fingerprint": "sha256:4444444444444444444444444444444444444444444444444444444444444444",
                "stack_ref": "handbook.context-resolution-stack.packet-4@1.0.0",
                "values": ["task", "repository", "session", "bounded", "execution", "verified"]
            }
        });
        observations.insert(
            (
                SnapshotSourceFamilyV1::Handbook,
                "schema_artifact_resolution",
            ),
            SnapshotSlotObservationV1::retained(
                "handbook.snapshot-observation.handbook.schema-artifact-resolution@1.0.0",
                &fingerprint(resolution.clone()),
            )
            .unwrap(),
        );
        values.insert(
            (
                SnapshotSourceFamilyV1::Handbook,
                "schema_artifact_resolution",
            ),
            resolution,
        );
        Self {
            observations,
            values,
            observed: Vec::new(),
            reads: BTreeSet::new(),
            fail_all_observations: false,
        }
    }

    fn advance(
        &mut self,
        family: SnapshotSourceFamilyV1,
        slot: &'static str,
        reference: &str,
        value: Value,
    ) {
        self.observations.insert(
            (family, slot),
            SnapshotSlotObservationV1::retained(reference, &fingerprint(value.clone())).unwrap(),
        );
        self.values.insert((family, slot), value);
    }

    fn reset_counters(&mut self) {
        self.observed.clear();
        self.reads.clear();
    }
}

impl SnapshotSourcePortV1 for CompletePort {
    fn observe_slot(
        &mut self,
        family: SnapshotSourceFamilyV1,
        slot: &'static str,
    ) -> Result<SnapshotSlotObservationV1, SnapshotSourcePortErrorV1> {
        if self.fail_all_observations {
            return Err(SnapshotSourcePortErrorV1::unavailable());
        }
        self.observed.push((family, slot));
        self.observations
            .get(&(family, slot))
            .cloned()
            .ok_or_else(SnapshotSourcePortErrorV1::missing)
    }

    fn read_bounded_slot(
        &mut self,
        family: SnapshotSourceFamilyV1,
        slot: &'static str,
    ) -> Result<Value, SnapshotSourcePortErrorV1> {
        self.reads.insert((family, slot));
        self.values
            .get(&(family, slot))
            .cloned()
            .ok_or_else(SnapshotSourcePortErrorV1::missing)
    }
}

fn fingerprint(value: Value) -> String {
    DefinitionFingerprint::from_json_value(&value)
        .unwrap()
        .to_string()
}
