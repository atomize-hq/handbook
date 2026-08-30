#![cfg(feature = "contract-schema-generation")]

use handbook_dock::DockManifestRegistryV1;
use handbook_engine::{
    DefinitionFingerprint, SnapshotGroundingOwnerV1, SnapshotSlotObservationV1,
    SnapshotSourceFamilyV1, SnapshotSourcePortErrorV1, SnapshotSourcePortV1,
};
use handbook_sdk::contract_membrane::{
    admit_operation_definition, canonical_json_bytes, operation_inventory, parse_canonical_json,
    snapshot_operation_definition, AdmissionReason, ContractLimits, ContractValue, DiscoveryState,
    ExactBinding, ExactRef, Fingerprint, InvocationRequestV1, OperationAdmissionResult,
    OperationOwner, OperationRequestV1, OperationSchemaValidator, OperationTransport,
    ProductionWitness, SnapshotOperationSchemaValidator, UtcTimestamp,
};
use handbook_sdk::HandbookSdkV1;
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use tempfile::tempdir;

const SNAPSHOT_OPERATIONS: [&str; 4] = [
    "snapshot.capture@1.0.0",
    "snapshot.read@1.0.0",
    "snapshot.delta@1.0.0",
    "snapshot.project@1.0.0",
];

const ABSENT_RESERVED: [&str; 4] = [
    "projection.definition.read@1.0.0",
    "projection.create@1.0.0",
    "snapshot.verify_current@1.0.0",
    "snapshot.resolve_applicable@1.0.0",
];

const PACKET_FIVE_ROOTS: [&str; 10] = [
    "handbook.operation.snapshot-capture-request@1.0.0",
    "handbook.operation.snapshot-capture-result@1.0.0",
    "handbook.operation.snapshot-delta-request@1.0.0",
    "handbook.operation.snapshot-delta-result@1.0.0",
    "handbook.operation.snapshot-project-request@1.0.0",
    "handbook.operation.snapshot-project-result@1.0.0",
    "handbook.operation.snapshot-read-request@1.0.0",
    "handbook.operation.snapshot-read-result@1.0.0",
    "handbook.use-case.session-grounding-boundary-details@1.0.0",
    "handbook.use-case.session-grounding-boundary-result@1.0.0",
];

const FIXED_USE_CASES: [&str; 7] = [
    "handbook.use-case.approver-administration@1.0.0",
    "handbook.use-case.author-charter@1.0.0",
    "handbook.use-case.author-project-context@1.0.0",
    "handbook.use-case.pipeline-route-resolve-and-persist@1.0.0",
    "handbook.use-case.repository-doctor@1.0.0",
    "handbook.use-case.repository-setup@1.0.0",
    "handbook.use-case.session-grounding-boundary@1.0.0",
];

#[test]
fn packet_five_closes_exactly_four_rows_at_exact_final_counts() {
    let rows = operation_inventory();
    assert_eq!(rows.len(), 62);
    for operation_ref in SNAPSHOT_OPERATIONS {
        let row = rows
            .iter()
            .find(|row| row.operation_ref().as_str() == operation_ref)
            .expect("reserved snapshot row remains in the inventory");
        assert_eq!(row.owner(), OperationOwner::Engine);
        assert_eq!(row.admission_reason(), AdmissionReason::Admitted);
        assert_eq!(row.discovery(), DiscoveryState::Discoverable);
        assert_eq!(
            row.current_transports(),
            [
                OperationTransport::CliJson,
                OperationTransport::RustSdk,
                OperationTransport::Tauri,
            ]
        );
    }
    for operation_ref in ABSENT_RESERVED {
        let row = rows
            .iter()
            .find(|row| row.operation_ref().as_str() == operation_ref)
            .expect("reserved row remains in the inventory");
        assert_ne!(row.admission_reason(), AdmissionReason::Admitted);
        assert_eq!(row.discovery(), DiscoveryState::Omit);
        assert!(row.current_transports().is_empty());
    }
    for operation_ref in SNAPSHOT_OPERATIONS {
        let definition = snapshot_operation_definition(operation_ref).unwrap();
        assert!(matches!(
            admit_operation_definition(&definition).unwrap(),
            OperationAdmissionResult::Admitted(_)
        ));
    }
    let witness =
        ProductionWitness::from_current(UtcTimestamp::parse("2031-01-01T00:00:00Z").unwrap())
            .unwrap();
    assert_eq!(witness.capabilities().len(), 35);
    assert_eq!(witness.schema_manifest().len(), 180);
    assert_eq!(DockManifestRegistryV1::production().production_len(), 1);
    assert_eq!(
        witness.catalog_roots().operations().total_entries().get(),
        35
    );

    let roots = witness
        .schema_manifest()
        .iter()
        .map(|entry| entry.schema().reference().as_str())
        .collect::<BTreeSet<_>>();
    assert!(PACKET_FIVE_ROOTS.iter().all(|root| roots.contains(root)));
    assert!(witness.capabilities().iter().all(|entry| {
        !entry
            .operation()
            .reference()
            .as_str()
            .starts_with("handbook.use-case.")
    }));
    assert_eq!(FIXED_USE_CASES.len(), 7);
    for use_case in FIXED_USE_CASES {
        assert!(rows
            .iter()
            .all(|row| row.operation_ref().as_str() != use_case));
        assert!(witness
            .capabilities()
            .iter()
            .all(|entry| entry.operation().reference().as_str() != use_case));
    }
}

#[test]
fn snapshot_request_schemas_are_closed_and_bounded() {
    let validator = SnapshotOperationSchemaValidator;
    let capture = snapshot_operation_definition("snapshot.capture@1.0.0").unwrap();
    let valid = contract_value(json!({
        "workspace": workspace(),
        "host_session_ref": "bounded-session",
        "boundary": "start",
        "idempotency_key": "bounded-key"
    }));
    validator
        .validate_instance(capture.request_schema(), &valid)
        .unwrap();
    let unknown = contract_value(json!({
        "workspace": workspace(),
        "host_session_ref": "bounded-session",
        "boundary": "start",
        "idempotency_key": "bounded-key",
        "caller_predecessor": "forbidden"
    }));
    assert!(validator
        .validate_instance(capture.request_schema(), &unknown)
        .is_err());
    let oversized = contract_value(json!({
        "workspace": workspace(),
        "host_session_ref": "x".repeat(1025),
        "boundary": "start",
        "idempotency_key": "bounded-key"
    }));
    assert!(validator
        .validate_instance(capture.request_schema(), &oversized)
        .is_err());

    let read = snapshot_operation_definition("snapshot.read@1.0.0").unwrap();
    let substituted_shape = contract_value(json!({
        "workspace": workspace(),
        "expected_boundary": {
            "exact_ref": "owner-content-addressed-ref",
            "fingerprint": "not-a-fingerprint",
            "caller_current": true
        }
    }));
    assert!(validator
        .validate_instance(read.request_schema(), &substituted_shape)
        .is_err());
}

#[test]
fn snapshot_operations_dispatch_owner_results_without_raw_source_aggregates() {
    let repo = tempdir().unwrap();
    let sdk = HandbookSdkV1::open(repo.path());
    let limits = ContractLimits::packet0();
    let witness =
        ProductionWitness::from_current(UtcTimestamp::parse("2031-01-01T00:00:00Z").unwrap())
            .unwrap();
    let mut port = CompletePort::successful();
    let workspace = workspace();
    let start = operation_invocation(
        &witness,
        "snapshot.capture@1.0.0",
        json!({
            "workspace": workspace,
            "host_session_ref": "packet-5-session",
            "boundary": "start",
            "idempotency_key": "packet-5-start-key"
        }),
    );
    let first = sdk
        .invoke_with_snapshot_source(&start, &limits, &mut port)
        .unwrap();
    let first_json = serde_json::to_value(&first).unwrap();
    assert_eq!(first_json["response"]["status"], "ok");
    assert_eq!(
        first_json["response"]["receipts"].as_array().unwrap().len(),
        1
    );
    assert_eq!(first_json["response"]["data"]["outcome"], "started");
    assert_eq!(port.observed.len(), 42);
    let boundary = first_json["response"]["data"]["boundary"].clone();
    let boundary = json!({
        "exact_ref": boundary["exact_ref"],
        "fingerprint": boundary["fingerprint"],
    });

    port.observed.clear();
    port.reads.clear();
    port.fail_all_observations = true;
    let replay = sdk
        .invoke_with_snapshot_source(&start, &limits, &mut port)
        .unwrap();
    let replay_json = serde_json::to_value(&replay).unwrap();
    assert_eq!(replay_json["response"]["data"]["replayed"], true);
    assert!(port.observed.is_empty());
    assert!(port.reads.is_empty());

    let conflict = operation_invocation(
        &witness,
        "snapshot.capture@1.0.0",
        json!({
            "workspace": workspace,
            "host_session_ref": "substituted-session",
            "boundary": "start",
            "idempotency_key": "packet-5-start-key"
        }),
    );
    let conflict = sdk
        .invoke_with_snapshot_source(&conflict, &limits, &mut port)
        .unwrap();
    assert_eq!(
        conflict.terminal_status(),
        handbook_sdk::contract_membrane::TerminalStatus::Refused
    );
    let conflict_json = serde_json::to_value(&conflict).unwrap();
    assert_eq!(
        conflict_json["response"]["refusals"][0]["details"]["error_kind"],
        "idempotency_conflict"
    );
    assert_eq!(
        conflict_json["response"]["refusals"][0]["details"]["mutation_effect"],
        "write_nothing"
    );
    assert!(conflict_json["response"]["receipts"]
        .as_array()
        .unwrap()
        .is_empty());

    for operation_ref in [
        "snapshot.read@1.0.0",
        "snapshot.delta@1.0.0",
        "snapshot.project@1.0.0",
    ] {
        let request = operation_invocation(
            &witness,
            operation_ref,
            json!({"workspace": workspace, "expected_boundary": boundary}),
        );
        let response = sdk
            .invoke(&request, &limits)
            .unwrap_or_else(|error| panic!("{operation_ref}: {error:?}"));
        assert_eq!(
            response.terminal_status(),
            handbook_sdk::contract_membrane::TerminalStatus::Ok
        );
        let bytes = canonical_json_bytes(&response, &limits).unwrap();
        let text = std::str::from_utf8(&bytes).unwrap();
        assert!(!text.contains("handbook.snapshot-observation."));
        sdk.readmit_invocation_response(&request, &response, &limits)
            .unwrap();
    }

    port.fail_all_observations = false;
    let end = operation_invocation(
        &witness,
        "snapshot.capture@1.0.0",
        json!({
            "workspace": workspace,
            "host_session_ref": "packet-5-session",
            "boundary": "end",
            "idempotency_key": "packet-5-end-key"
        }),
    );
    let finalized = sdk
        .invoke_with_snapshot_source(&end, &limits, &mut port)
        .unwrap();
    let finalized = serde_json::to_value(finalized).unwrap();
    assert_eq!(finalized["response"]["data"]["outcome"], "finalized");
    let end_boundary = json!({
        "exact_ref": finalized["response"]["data"]["boundary"]["exact_ref"],
        "fingerprint": finalized["response"]["data"]["boundary"]["fingerprint"],
    });
    let delta_request = operation_invocation(
        &witness,
        "snapshot.delta@1.0.0",
        json!({"workspace": workspace, "expected_boundary": end_boundary}),
    );
    let delta = sdk.invoke(&delta_request, &limits).unwrap();
    let delta = serde_json::to_value(delta).unwrap();
    assert_eq!(delta["response"]["data"]["outcome"], "found");
    assert_eq!(
        delta["response"]["data"]["changed"].as_u64().unwrap()
            + delta["response"]["data"]["unchanged"].as_u64().unwrap(),
        15
    );
}

fn workspace() -> ExactBinding {
    ExactBinding::new(
        ExactRef::parse("handbook.workspace.packet-5-test@1.0.0").unwrap(),
        Fingerprint::parse(
            "sha256:5555555555555555555555555555555555555555555555555555555555555555",
        )
        .unwrap(),
    )
}

fn operation_invocation(
    witness: &ProductionWitness,
    operation_ref: &str,
    body: Value,
) -> InvocationRequestV1 {
    let limits = ContractLimits::packet0();
    let definition = snapshot_operation_definition(operation_ref).unwrap();
    let schema = witness
        .schema_manifest()
        .iter()
        .find(|entry| entry.schema().reference().as_str() == "handbook.operation-request@1.0.0")
        .unwrap()
        .schema()
        .clone();
    let body: ContractValue =
        parse_canonical_json(&canonical_json_bytes(&body, &limits).unwrap(), &limits).unwrap();
    let operation = OperationRequestV1::new(
        schema,
        None,
        &definition,
        body,
        &SnapshotOperationSchemaValidator,
        &limits,
    )
    .unwrap();
    InvocationRequestV1::operation(operation, &limits).unwrap()
}

fn contract_value(value: Value) -> ContractValue {
    let limits = ContractLimits::packet0();
    parse_canonical_json(&canonical_json_bytes(&value, &limits).unwrap(), &limits).unwrap()
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
        let decision = values
            .get(&(SnapshotSourceFamilyV1::Evidence, "contracts_decision"))
            .unwrap()
            .clone();
        observations.insert(
            (SnapshotSourceFamilyV1::Evidence, "contracts_decision"),
            SnapshotSlotObservationV1::derived(
                "handbook.snapshot-observation.evidence.contracts-decision@1.0.0",
                &fingerprint(decision),
            )
            .unwrap(),
        );
        let resolution = json!({"resolution":{"stack_fingerprint":"sha256:5555555555555555555555555555555555555555555555555555555555555555","stack_ref":"handbook.context-resolution-stack.packet-5@1.0.0","values":["task","repository","session","bounded","execution","verified"]}});
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
