use handbook_engine::{
    DefinitionFingerprint, SnapshotGroundingOwnerV1, SnapshotSlotObservationV1,
    SnapshotSourceFamilyV1, SnapshotSourcePortErrorV1, SnapshotSourcePortV1,
};
use handbook_sdk::contract_membrane::{
    canonical_json_bytes, parse_canonical_json, snapshot_operation_definition, ContractLimits,
    ContractValue, ExactBinding, ExactRef, Fingerprint, InvocationRequestV1, OperationRequestV1,
    ProductionWitness, SnapshotOperationSchemaValidator, UtcTimestamp,
};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};

pub fn workspace() -> ExactBinding {
    ExactBinding::new(
        ExactRef::parse("handbook.workspace.packet-6-proof@1.0.0").unwrap(),
        Fingerprint::parse(
            "sha256:6666666666666666666666666666666666666666666666666666666666666666",
        )
        .unwrap(),
    )
}

pub fn witness() -> ProductionWitness {
    ProductionWitness::from_current(UtcTimestamp::parse("2031-01-01T00:00:00Z").unwrap()).unwrap()
}

pub fn operation_invocation(operation_ref: &str, body: Value) -> InvocationRequestV1 {
    let limits = ContractLimits::packet0();
    let definition = snapshot_operation_definition(operation_ref).unwrap();
    let schema = witness()
        .schema_manifest()
        .iter()
        .find(|entry| entry.schema().reference().as_str() == "handbook.operation-request@1.0.0")
        .unwrap()
        .schema()
        .clone();
    let body: ContractValue =
        parse_canonical_json(&canonical_json_bytes(&body, &limits).unwrap(), &limits).unwrap();
    let request = OperationRequestV1::new(
        schema,
        None,
        &definition,
        body,
        &SnapshotOperationSchemaValidator,
        &limits,
    )
    .unwrap();
    InvocationRequestV1::operation(request, &limits).unwrap()
}

pub fn target_body(response: &Value) -> Value {
    json!({
        "workspace": workspace(),
        "expected_boundary": {
            "exact_ref": response["response"]["data"]["boundary"]["exact_ref"],
            "fingerprint": response["response"]["data"]["boundary"]["fingerprint"],
        }
    })
}

pub fn capture_body(session: &str, boundary: &str, key: &str) -> Value {
    json!({
        "workspace": workspace(),
        "host_session_ref": session,
        "boundary": boundary,
        "idempotency_key": key,
    })
}

pub struct CompleteSnapshotSource {
    observations: BTreeMap<(SnapshotSourceFamilyV1, &'static str), SnapshotSlotObservationV1>,
    values: BTreeMap<(SnapshotSourceFamilyV1, &'static str), Value>,
    pub observed: Vec<(SnapshotSourceFamilyV1, &'static str)>,
    pub reads: BTreeSet<(SnapshotSourceFamilyV1, &'static str)>,
    pub fail_all_observations: bool,
}

impl CompleteSnapshotSource {
    pub fn new() -> Self {
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
                let value = json!({"ref": reference});
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
                &fingerprint(json!({"redacted": true})),
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
        let decision = values[&(SnapshotSourceFamilyV1::Evidence, "contracts_decision")].clone();
        observations.insert(
            (SnapshotSourceFamilyV1::Evidence, "contracts_decision"),
            SnapshotSlotObservationV1::derived(
                "handbook.snapshot-observation.evidence.contracts-decision@1.0.0",
                &fingerprint(decision),
            )
            .unwrap(),
        );
        let resolution = json!({"resolution": {
            "stack_fingerprint": "sha256:6666666666666666666666666666666666666666666666666666666666666666",
            "stack_ref": "handbook.context-resolution-stack.packet-6@1.0.0",
            "values": ["task", "repository", "session", "bounded", "execution", "verified"]
        }});
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

    #[allow(dead_code)]
    pub fn advance_handoff(&mut self, handoff_ref: &str) {
        let value = json!({"refs": [handoff_ref]});
        self.observations.insert(
            (SnapshotSourceFamilyV1::Work, "handoff"),
            SnapshotSlotObservationV1::retained(
                "handbook.snapshot-observation.work.handoff-packet-6@1.0.0",
                &fingerprint(value.clone()),
            )
            .unwrap(),
        );
        self.values
            .insert((SnapshotSourceFamilyV1::Work, "handoff"), value);
    }

    #[allow(dead_code)]
    pub fn reset_counters(&mut self) {
        self.observed.clear();
        self.reads.clear();
    }
}

impl SnapshotSourcePortV1 for CompleteSnapshotSource {
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
