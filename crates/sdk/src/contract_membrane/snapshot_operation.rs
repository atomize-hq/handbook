use super::bootstrap::{CurrentTransportEvidence, CurrentTransportEvidenceSealed};
use super::{
    admit_operation_definition, canonical_json_bytes, parse_canonical_json, sha256_fingerprint,
    BoundedString, BoundedVec, CapabilityEntry, ContractError, ContractLimits, ContractValue,
    CorrelationRef, DeclaredWrite, ExactBinding, ExactRef, Fingerprint, IdempotencyKeyConsumer,
    IdempotencyKeyConsumerSealed, IdempotencyStage, IdempotencyState, JsonPointer,
    OperationAdmissionResult, OperationClass, OperationDefinition, OperationIdempotency,
    OperationOutcomeSchemas, OperationOwnerBinding, OperationRequestV1, OperationResponseV1,
    OperationSchemaValidator, OperationTransport, OwnerAdmissionToken, Problem, ProblemBinding,
    ProblemCategory, RecheckCondition, RetryDirective, SemVer, TerminalFields, TerminalStatus,
    UtcTimestamp, WriteReceipt,
};
use handbook_engine::{
    SnapshotBoundaryKindV1, SnapshotCaptureOutcomeV1, SnapshotCaptureRequestV1,
    SnapshotGroundingErrorClassV1, SnapshotGroundingErrorKindV1, SnapshotGroundingOwnerV1,
    SnapshotRecordBindingV1, SnapshotSourcePortV1,
};
use serde::{Deserialize, Serialize};
use std::path::Path;

pub const SNAPSHOT_OPERATION_REFS: [&str; 4] = [
    "snapshot.capture@1.0.0",
    "snapshot.delta@1.0.0",
    "snapshot.project@1.0.0",
    "snapshot.read@1.0.0",
];

pub struct SnapshotOperationSchemaValidator;

#[derive(Clone, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SnapshotCaptureRequestDtoV1 {
    pub workspace: ExactBinding,
    pub host_session_ref: String,
    pub boundary: SnapshotBoundaryDtoV1,
    pub idempotency_key: String,
}

impl std::fmt::Debug for SnapshotCaptureRequestDtoV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SnapshotCaptureRequestDtoV1")
            .field("workspace", &self.workspace)
            .field("host_session_ref", &self.host_session_ref)
            .field("boundary", &self.boundary)
            .field("idempotency_key", &"<redacted>")
            .finish()
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SnapshotBoundaryDtoV1 {
    Start,
    End,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SnapshotTargetRequestDtoV1 {
    pub workspace: ExactBinding,
    pub expected_boundary: SnapshotBoundaryBindingDtoV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SnapshotBoundaryBindingDtoV1 {
    pub exact_ref: String,
    pub fingerprint: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct SnapshotCaptureSanitizedV1 {
    workspace: ExactBinding,
    host_session_ref: String,
    boundary: SnapshotBoundaryDtoV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct SnapshotCaptureResultV1 {
    outcome: String,
    sequence: u64,
    predecessor_outcome: String,
    predecessor_boundary_ref: Option<String>,
    boundary: SnapshotRecordBindingV1,
    accounting: SnapshotRecordBindingV1,
    delta: Option<SnapshotRecordBindingV1>,
    grounding: SnapshotRecordBindingV1,
    receipt: SnapshotRecordBindingV1,
    replayed: bool,
    handoff_refs: Vec<String>,
    currentness: String,
    recovery_action: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct SnapshotReadResultV1 {
    outcome: String,
    descriptor: Option<SnapshotDescriptorDtoV1>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct SnapshotDescriptorDtoV1 {
    boundary: SnapshotRecordBindingV1,
    boundary_kind: String,
    sequence: u64,
    predecessor_outcome: String,
    accounting: SnapshotRecordBindingV1,
    delta: Option<SnapshotRecordBindingV1>,
    grounding: SnapshotRecordBindingV1,
    source_accounting: Vec<handbook_engine::SnapshotAccountingEntryV1>,
    current: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct SnapshotDeltaResultV1 {
    outcome: String,
    delta: Option<SnapshotRecordBindingV1>,
    before_ref: Option<String>,
    before_fingerprint: Option<String>,
    before_sequence: Option<u64>,
    after_ref: Option<String>,
    after_fingerprint: Option<String>,
    after_sequence: Option<u64>,
    pair_kind: Option<String>,
    changed: u64,
    unchanged: u64,
    current: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct SnapshotProjectionResultV1 {
    outcome: String,
    grounding: Option<SnapshotRecordBindingV1>,
    capture_ref: Option<String>,
    capture_fingerprint: Option<String>,
    definition: Option<ExactBinding>,
    disclosure: Option<ExactBinding>,
    lossiness: Option<String>,
    omissions: Vec<handbook_engine::SnapshotAccountingEntryV1>,
    refusals: Vec<handbook_engine::SnapshotAccountingEntryV1>,
    required_rechecks: Vec<String>,
    resolution_stack: Option<ExactBinding>,
    resolution_values: Vec<String>,
    handoff_refs: Vec<String>,
    current: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct SnapshotProblemDetailsV1 {
    outcome: String,
    operation_id: String,
    error_class: String,
    error_kind: String,
    mutation_effect: String,
    recovery_action: String,
    source_accounting: Vec<handbook_engine::SnapshotAccountingEntryV1>,
}

impl OperationSchemaValidator for SnapshotOperationSchemaValidator {
    fn validate_instance(
        &self,
        schema: &ExactBinding,
        instance: &ContractValue,
    ) -> Result<(), ContractError> {
        let source =
            schema_source(schema.reference().as_str()).ok_or(ContractError::InvalidInvariant)?;
        if schema.fingerprint() != &super::sha256_fingerprint(source.as_bytes()) {
            return Err(ContractError::InvalidInvariant);
        }
        let authored: serde_json::Value =
            serde_json::from_str(source).map_err(|_| ContractError::InvalidJson)?;
        let instance = serde_json::to_value(instance).map_err(|_| ContractError::InvalidJson)?;
        let validator = jsonschema::draft202012::options()
            .with_pattern_options(jsonschema::PatternOptions::fancy_regex())
            .build(&authored)
            .map_err(|_| ContractError::InvalidInvariant)?;
        validator
            .is_valid(&instance)
            .then_some(())
            .ok_or(ContractError::InvalidInvariant)
    }
}

pub fn snapshot_operation_definitions() -> Result<Vec<OperationDefinition>, ContractError> {
    SNAPSHOT_OPERATION_REFS
        .into_iter()
        .map(snapshot_operation_definition)
        .collect()
}

pub fn snapshot_operation_definition(
    operation_ref: &str,
) -> Result<OperationDefinition, ContractError> {
    if !SNAPSHOT_OPERATION_REFS.contains(&operation_ref) {
        return Err(ContractError::InvalidInvariant);
    }
    let limits = ContractLimits::packet0();
    let owner = handbook_engine::operation_admission::operation_owner_version_evidence();
    let capture = operation_ref == "snapshot.capture@1.0.0";
    let result_schema = schema_ref(operation_ref, "result");
    OperationDefinition::new(
        binding("handbook.operation-definition@1.0.0")?,
        ExactRef::parse(operation_ref)?,
        OperationOwnerBinding::new(
            BoundedString::new(owner.owner_crate.to_owned())?,
            SemVer::parse(owner.owner_version)?,
            ExactBinding::new(
                ExactRef::parse(owner.owner_source_ref)?,
                super::sha256_fingerprint(owner.source_bytes),
            ),
        )?,
        binding(&schema_ref(operation_ref, "request"))?,
        binding(&result_schema)?,
        OperationOutcomeSchemas::new(
            binding("handbook.operation-blocker@1.0.0")?,
            binding("handbook.operation-refusal@1.0.0")?,
            binding("handbook.operation-error@1.0.0")?,
        )?,
        BoundedVec::new(Vec::new())?,
        if capture {
            OperationClass::AppendOnly
        } else {
            OperationClass::ReadOnly
        },
        if capture {
            OperationIdempotency::key_required(JsonPointer::parse("/body/idempotency_key")?)?
        } else {
            OperationIdempotency::safe()
        },
        BoundedVec::new(if capture {
            vec![DeclaredWrite::new(
                BoundedString::new("snapshot_boundary".to_owned())?,
                BoundedString::new("semantic_record".to_owned())?,
                BoundedString::new("snapshot_owner_transaction".to_owned())?,
                binding(&result_schema)?,
            )?]
        } else {
            Vec::new()
        })?,
        BoundedVec::new(vec![
            OperationTransport::CliJson,
            OperationTransport::RustSdk,
            OperationTransport::Tauri,
        ])?,
        None,
        &limits,
    )
}

pub(crate) fn current_snapshot_capabilities() -> Result<Vec<CapabilityEntry>, ContractError> {
    snapshot_operation_definitions()?
        .into_iter()
        .map(|definition| {
            let owner = match admit_operation_definition(&definition)? {
                OperationAdmissionResult::Admitted(owner) => owner,
                OperationAdmissionResult::Omitted(_) => {
                    return Err(ContractError::InvalidInvariant)
                }
            };
            CapabilityEntry::new(
                &definition,
                &SnapshotTransportEvidence { owner },
                &ContractLimits::packet0(),
            )
        })
        .collect()
}

struct SnapshotTransportEvidence {
    owner: OwnerAdmissionToken,
}
impl CurrentTransportEvidenceSealed for SnapshotTransportEvidence {}
impl CurrentTransportEvidence for SnapshotTransportEvidence {
    fn owner_admission(&self) -> &OwnerAdmissionToken {
        &self.owner
    }
    fn current_transports(&self) -> &[OperationTransport] {
        &[
            OperationTransport::CliJson,
            OperationTransport::RustSdk,
            OperationTransport::Tauri,
        ]
    }
}

pub(crate) fn execute_snapshot_operation(
    repo_root: &Path,
    request: &OperationRequestV1,
    source_port: Option<&mut dyn SnapshotSourcePortV1>,
) -> Result<OperationResponseV1, ContractError> {
    let operation_ref = request.operation().reference().as_str();
    let definition = snapshot_operation_definition(operation_ref)?;
    if request.operation() != &definition.binding()
        || request.body_schema() != definition.request_schema()
    {
        return Err(ContractError::InvalidInvariant);
    }
    match admit_operation_definition(&definition)? {
        OperationAdmissionResult::Admitted(owner)
            if owner.operation_ref() == operation_ref
                && owner.definition_fingerprint()
                    == definition.definition_fingerprint().as_str() => {}
        _ => return Err(ContractError::InvalidInvariant),
    }
    match operation_ref {
        "snapshot.capture@1.0.0" => {
            let source_port = source_port.ok_or(ContractError::InvalidInvariant)?;
            execute_capture(repo_root, request, &definition, source_port)
        }
        "snapshot.read@1.0.0" => execute_read(repo_root, request, &definition),
        "snapshot.delta@1.0.0" => execute_delta(repo_root, request, &definition),
        "snapshot.project@1.0.0" => execute_project(repo_root, request, &definition),
        _ => Err(ContractError::InvalidInvariant),
    }
}

struct CaptureOwnerBridge<'a> {
    repo_root: &'a Path,
    body: &'a SnapshotCaptureSanitizedV1,
    source_port: &'a mut dyn SnapshotSourcePortV1,
    pointer_observed: bool,
    result: Option<Result<SnapshotCaptureOutcomeV1, handbook_engine::SnapshotCaptureErrorV1>>,
}

impl IdempotencyKeyConsumerSealed for CaptureOwnerBridge<'_> {
    fn consume_owner_idempotency_key(&mut self, pointer: &str, bytes: &[u8]) -> Fingerprint {
        self.pointer_observed = pointer == "/body/idempotency_key";
        self.result = Some((|| {
            let owner = SnapshotGroundingOwnerV1::new();
            let binding = owner
                .bind_repository(
                    self.repo_root,
                    self.body.workspace.reference().as_str(),
                    self.body.workspace.fingerprint().as_str(),
                )
                .map_err(|_| malformed_capture_error())?;
            let stream = owner.bind_session_stream(&binding);
            let capture = SnapshotCaptureRequestV1::new(
                bytes,
                &self.body.host_session_ref,
                match self.body.boundary {
                    SnapshotBoundaryDtoV1::Start => SnapshotBoundaryKindV1::Start,
                    SnapshotBoundaryDtoV1::End => SnapshotBoundaryKindV1::End,
                },
            )?;
            let mut forwarding = DynSourcePortV1 {
                inner: self.source_port,
            };
            owner.capture_boundary(&binding, &stream, &capture, &mut forwarding)
        })());
        sha256_fingerprint(bytes)
    }
}

impl IdempotencyKeyConsumer for CaptureOwnerBridge<'_> {}

struct DynSourcePortV1<'a> {
    inner: &'a mut dyn SnapshotSourcePortV1,
}

impl SnapshotSourcePortV1 for DynSourcePortV1<'_> {
    fn observe_slot(
        &mut self,
        family: handbook_engine::SnapshotSourceFamilyV1,
        slot: &'static str,
    ) -> Result<
        handbook_engine::SnapshotSlotObservationV1,
        handbook_engine::SnapshotSourcePortErrorV1,
    > {
        self.inner.observe_slot(family, slot)
    }

    fn read_bounded_slot(
        &mut self,
        family: handbook_engine::SnapshotSourceFamilyV1,
        slot: &'static str,
    ) -> Result<serde_json::Value, handbook_engine::SnapshotSourcePortErrorV1> {
        self.inner.read_bounded_slot(family, slot)
    }
}

fn execute_capture(
    repo_root: &Path,
    request: &OperationRequestV1,
    definition: &OperationDefinition,
    source_port: &mut dyn SnapshotSourcePortV1,
) -> Result<OperationResponseV1, ContractError> {
    let body = parse_body::<SnapshotCaptureSanitizedV1>(request)?;
    validate_session_ref(&body.host_session_ref)?;
    let mut bridge = CaptureOwnerBridge {
        repo_root,
        body: &body,
        source_port,
        pointer_observed: false,
        result: None,
    };
    request
        .with_owner_idempotency_key(&mut bridge)
        .ok_or(ContractError::InvalidInvariant)?;
    if !bridge.pointer_observed {
        return Err(ContractError::InvalidInvariant);
    }
    match bridge.result.ok_or(ContractError::InvalidInvariant)? {
        Ok(outcome) => capture_response(request, definition, outcome),
        Err(error) => {
            let accounting = error
                .accounting()
                .iter()
                .map(|entry| handbook_engine::SnapshotAccountingEntryV1 {
                    family: entry.family().as_str().to_owned(),
                    slot: entry.slot().to_owned(),
                    disposition: entry.disposition().to_owned(),
                })
                .collect();
            snapshot_problem_response(request, definition, error.kind(), accounting)
        }
    }
}

fn capture_response(
    request: &OperationRequestV1,
    definition: &OperationDefinition,
    outcome: SnapshotCaptureOutcomeV1,
) -> Result<OperationResponseV1, ContractError> {
    let (
        outcome_name,
        sequence,
        predecessor,
        boundary,
        accounting,
        delta,
        grounding,
        receipt,
        replayed,
        handoff_refs,
    ) = match &outcome {
        SnapshotCaptureOutcomeV1::Started(value) => (
            "started",
            value.sequence(),
            value.predecessor_boundary_ref(),
            value.boundary_record(),
            value.accounting_record(),
            value.delta_record(),
            value.grounding_record(),
            value.receipt_record(),
            value.replayed(),
            Vec::new(),
        ),
        SnapshotCaptureOutcomeV1::Finalized(value) => (
            "finalized",
            value.sequence(),
            value.predecessor_boundary_ref(),
            value.boundary_record(),
            value.accounting_record(),
            value.delta_record(),
            value.grounding_record(),
            value.receipt_record(),
            value.replayed(),
            value.handoff_refs().to_vec(),
        ),
    };
    let record =
        |class: &str, value: &handbook_engine::SnapshotCommittedRecordV1| SnapshotRecordBindingV1 {
            class: class.to_owned(),
            exact_ref: value.exact_ref().to_owned(),
            fingerprint: value.fingerprint().to_owned(),
        };
    let data = SnapshotCaptureResultV1 {
        outcome: outcome_name.to_owned(),
        sequence,
        predecessor_outcome: if predecessor.is_some() {
            "applicable_prior"
        } else {
            "no_applicable_prior"
        }
        .to_owned(),
        predecessor_boundary_ref: predecessor.map(str::to_owned),
        boundary: record("boundary", boundary),
        accounting: record("accounting", accounting),
        delta: delta.map(|value| record("delta", value)),
        grounding: record("grounding", grounding),
        receipt: record("receipt", receipt),
        replayed,
        handoff_refs,
        currentness: "owner_derived_current".to_owned(),
        recovery_action: if replayed {
            "accepted_idempotent_replay"
        } else {
            "none"
        }
        .to_owned(),
    };
    let data_value = to_contract_value(&data)?;
    let declared = definition
        .write_set()
        .first()
        .ok_or(ContractError::InvalidInvariant)?;
    let receipts = BoundedVec::new(vec![WriteReceipt::new(
        BoundedString::new(declared.record_kind().to_owned())?,
        ExactBinding::new(
            ExactRef::parse("handbook.snapshot-grounding-owner-receipt@1.0.0")?,
            Fingerprint::parse(receipt.fingerprint())?,
        ),
        BoundedString::new(declared.authority_class().to_owned())?,
        data_value.clone(),
        BoundedString::new(declared.atomic_group().to_owned())?,
        &ContractLimits::packet0(),
    )?])?;
    established_response(request, definition, data_value, receipts, replayed)
}

fn execute_read(
    repo_root: &Path,
    request: &OperationRequestV1,
    definition: &OperationDefinition,
) -> Result<OperationResponseV1, ContractError> {
    let body = parse_body::<SnapshotTargetRequestDtoV1>(request)?;
    let (owner, binding, stream) = owner_target(repo_root, &body)?;
    let descriptor = match owner.read_current_snapshot(
        &binding,
        &stream,
        &body.expected_boundary.exact_ref,
        &body.expected_boundary.fingerprint,
    ) {
        Ok(value) => value,
        Err(error) => {
            return snapshot_problem_response(request, definition, error.kind(), Vec::new())
        }
    };
    let descriptor = descriptor.map(|value| SnapshotDescriptorDtoV1 {
        boundary: value.boundary,
        boundary_kind: value.boundary_kind,
        sequence: value.sequence,
        predecessor_outcome: match value.predecessor {
            handbook_engine::SnapshotPredecessorOutcomeV1::NoApplicablePrior => {
                "no_applicable_prior"
            }
            handbook_engine::SnapshotPredecessorOutcomeV1::ApplicablePrior { .. } => {
                "applicable_prior"
            }
        }
        .to_owned(),
        accounting: value.accounting,
        delta: value.delta,
        grounding: value.grounding,
        source_accounting: value.source_accounting,
        current: value.current,
    });
    ok_response(
        request,
        definition,
        &SnapshotReadResultV1 {
            outcome: if descriptor.is_some() {
                "found"
            } else {
                "absent"
            }
            .to_owned(),
            descriptor,
        },
    )
}

fn execute_delta(
    repo_root: &Path,
    request: &OperationRequestV1,
    definition: &OperationDefinition,
) -> Result<OperationResponseV1, ContractError> {
    let body = parse_body::<SnapshotTargetRequestDtoV1>(request)?;
    let (owner, binding, stream) = owner_target(repo_root, &body)?;
    let value = match owner.read_current_delta(
        &binding,
        &stream,
        &body.expected_boundary.exact_ref,
        &body.expected_boundary.fingerprint,
    ) {
        Ok(value) => value,
        Err(error) => {
            return snapshot_problem_response(request, definition, error.kind(), Vec::new())
        }
    };
    let data = match value {
        Some(value) => SnapshotDeltaResultV1 {
            outcome: "found".to_owned(),
            delta: Some(value.delta),
            before_ref: Some(value.before_ref),
            before_fingerprint: Some(value.before_fingerprint),
            before_sequence: Some(value.before_sequence),
            after_ref: Some(value.after_ref),
            after_fingerprint: Some(value.after_fingerprint),
            after_sequence: Some(value.after_sequence),
            pair_kind: Some(value.pair_kind),
            changed: value.changed,
            unchanged: value.unchanged,
            current: value.current,
        },
        None => SnapshotDeltaResultV1 {
            outcome: "absent".to_owned(),
            delta: None,
            before_ref: None,
            before_fingerprint: None,
            before_sequence: None,
            after_ref: None,
            after_fingerprint: None,
            after_sequence: None,
            pair_kind: None,
            changed: 0,
            unchanged: 0,
            current: false,
        },
    };
    ok_response(request, definition, &data)
}

fn execute_project(
    repo_root: &Path,
    request: &OperationRequestV1,
    definition: &OperationDefinition,
) -> Result<OperationResponseV1, ContractError> {
    let body = parse_body::<SnapshotTargetRequestDtoV1>(request)?;
    let (owner, binding, stream) = owner_target(repo_root, &body)?;
    let value = match owner.read_current_projection(
        &binding,
        &stream,
        &body.expected_boundary.exact_ref,
        &body.expected_boundary.fingerprint,
    ) {
        Ok(value) => value,
        Err(error) => {
            return snapshot_problem_response(request, definition, error.kind(), Vec::new())
        }
    };
    let data = match value {
        Some(value) => SnapshotProjectionResultV1 {
            outcome: "found".to_owned(),
            grounding: Some(value.grounding),
            capture_ref: Some(value.capture_ref),
            capture_fingerprint: Some(value.capture_fingerprint),
            definition: Some(ExactBinding::new(
                ExactRef::parse(&value.definition_ref)?,
                Fingerprint::parse(&value.definition_fingerprint)?,
            )),
            disclosure: Some(ExactBinding::new(
                ExactRef::parse(&value.disclosure_ref)?,
                Fingerprint::parse(&value.disclosure_fingerprint)?,
            )),
            lossiness: Some(value.lossiness),
            omissions: value.omissions,
            refusals: value.refusals,
            required_rechecks: value
                .required_rechecks
                .into_iter()
                .map(|item| format!("{}:{}", item.family, item.basis))
                .collect(),
            resolution_stack: Some(ExactBinding::new(
                ExactRef::parse(&value.resolution_stack_ref)?,
                Fingerprint::parse(&value.resolution_stack_fingerprint)?,
            )),
            resolution_values: value.resolution_values.into_iter().collect(),
            handoff_refs: value.handoff_refs,
            current: value.current,
        },
        None => SnapshotProjectionResultV1 {
            outcome: "absent".to_owned(),
            grounding: None,
            capture_ref: None,
            capture_fingerprint: None,
            definition: None,
            disclosure: None,
            lossiness: None,
            omissions: Vec::new(),
            refusals: Vec::new(),
            required_rechecks: Vec::new(),
            resolution_stack: None,
            resolution_values: Vec::new(),
            handoff_refs: Vec::new(),
            current: false,
        },
    };
    ok_response(request, definition, &data)
}

fn owner_target(
    repo_root: &Path,
    body: &SnapshotTargetRequestDtoV1,
) -> Result<
    (
        SnapshotGroundingOwnerV1,
        handbook_engine::SnapshotRepositoryBindingV1,
        handbook_engine::SnapshotStreamIdentityV1,
    ),
    ContractError,
> {
    let owner = SnapshotGroundingOwnerV1::new();
    let binding = owner
        .bind_repository(
            repo_root,
            body.workspace.reference().as_str(),
            body.workspace.fingerprint().as_str(),
        )
        .map_err(|_| ContractError::InvalidInvariant)?;
    let stream = owner.bind_session_stream(&binding);
    Ok((owner, binding, stream))
}

fn snapshot_problem_response(
    request: &OperationRequestV1,
    definition: &OperationDefinition,
    kind: SnapshotGroundingErrorKindV1,
    source_accounting: Vec<handbook_engine::SnapshotAccountingEntryV1>,
) -> Result<OperationResponseV1, ContractError> {
    let limits = ContractLimits::packet0();
    let class = kind.class();
    let status = match class {
        SnapshotGroundingErrorClassV1::Refusal | SnapshotGroundingErrorClassV1::Conflict => {
            TerminalStatus::Refused
        }
        SnapshotGroundingErrorClassV1::Unavailable => TerminalStatus::Blocked,
        SnapshotGroundingErrorClassV1::Error | SnapshotGroundingErrorClassV1::PostCommit => {
            TerminalStatus::Error
        }
    };
    let cause = snapshot_error_kind_name(kind);
    let details = to_contract_value(&SnapshotProblemDetailsV1 {
        outcome: snapshot_error_class_name(class).to_owned(),
        operation_id: definition
            .operation()
            .as_str()
            .strip_suffix("@1.0.0")
            .ok_or(ContractError::InvalidInvariant)?
            .to_owned(),
        error_class: snapshot_error_class_name(class).to_owned(),
        error_kind: cause.to_owned(),
        mutation_effect: match kind.mutation_effect() {
            handbook_engine::SnapshotGroundingMutationEffectV1::WriteNothing => "write_nothing",
            handbook_engine::SnapshotGroundingMutationEffectV1::CommittedReceiptPreserved => {
                "committed_receipt_preserved"
            }
        }
        .to_owned(),
        recovery_action: match class {
            SnapshotGroundingErrorClassV1::Conflict => "change_request",
            SnapshotGroundingErrorClassV1::Unavailable => "retry_after_recheck",
            SnapshotGroundingErrorClassV1::PostCommit => "resolve_committed_receipt",
            SnapshotGroundingErrorClassV1::Error => "contact_operator",
            SnapshotGroundingErrorClassV1::Refusal => "change_request",
        }
        .to_owned(),
        source_accounting,
    })?;
    SnapshotOperationSchemaValidator.validate_instance(definition.result_schema(), &details)?;
    let details_schema = definition.result_schema().clone();
    let rule = if status == TerminalStatus::Refused {
        Some(ExactBinding::new(
            ExactRef::parse(&format!(
                "handbook.snapshot-rule.{}@1.0.0",
                cause.replace('_', "-")
            ))?,
            definition.owner().source().fingerprint().clone(),
        ))
    } else {
        None
    };
    let recheck = if status == TerminalStatus::Blocked {
        Some(RecheckCondition::new(
            details_schema.clone(),
            details.clone(),
            &limits,
        )?)
    } else {
        None
    };
    let correlation_ref = if status == TerminalStatus::Error {
        let digest = sha256_fingerprint(
            format!(
                "handbook.snapshot-error-correlation-v1\0{}\0{cause}",
                request.request_fingerprint().as_str()
            )
            .as_bytes(),
        );
        Some(CorrelationRef::new(BoundedString::new(format!(
            "corr_{}",
            digest.as_str().trim_start_matches("sha256:")
        ))?)?)
    } else {
        None
    };
    let problem_id = BoundedString::new(format!("snapshot.{cause}"))?;
    let problem = Problem::new(
        problem_id.clone(),
        BoundedString::new(format!("snapshot.{cause}"))?,
        snapshot_problem_category(kind),
        definition.binding(),
        rule,
        details_schema,
        details,
        BoundedVec::new(Vec::new())?,
        snapshot_retry_directive(class),
        recheck,
        correlation_ref,
        &limits,
    )?;
    let problem_binding = ProblemBinding::new(problem_id, problem.problem_fingerprint().clone())?;
    let terminal = match status {
        TerminalStatus::Blocked => TerminalFields::new(
            status,
            None,
            BoundedVec::new(vec![problem])?,
            BoundedVec::new(Vec::new())?,
            BoundedVec::new(Vec::new())?,
        )?,
        TerminalStatus::Refused => TerminalFields::new(
            status,
            None,
            BoundedVec::new(Vec::new())?,
            BoundedVec::new(vec![problem])?,
            BoundedVec::new(Vec::new())?,
        )?,
        TerminalStatus::Error => TerminalFields::new(
            status,
            None,
            BoundedVec::new(Vec::new())?,
            BoundedVec::new(Vec::new())?,
            BoundedVec::new(vec![problem])?,
        )?,
        TerminalStatus::Ok => return Err(ContractError::InvalidInvariant),
    };
    let idempotency = if matches!(definition.idempotency(), OperationIdempotency::Safe) {
        IdempotencyState::not_applicable()?
    } else {
        IdempotencyState::not_established(snapshot_idempotency_stage(kind), problem_binding)?
    };
    OperationResponseV1::new(
        binding("handbook.operation-response@1.0.0")?,
        request.request_id().cloned(),
        request.request_fingerprint().clone(),
        definition,
        terminal,
        idempotency,
        BoundedVec::new(Vec::new())?,
        BoundedVec::new(Vec::new())?,
        BoundedVec::new(Vec::new())?,
        &SnapshotOperationSchemaValidator,
        &limits,
    )
}

fn snapshot_problem_category(kind: SnapshotGroundingErrorKindV1) -> ProblemCategory {
    use SnapshotGroundingErrorKindV1 as K;
    match kind {
        K::MalformedInput => ProblemCategory::Schema,
        K::InputLimitExceeded | K::RedactionUncertain => ProblemCategory::Safety,
        K::UnknownReference
        | K::SubstitutedReference
        | K::RepositoryMismatch
        | K::PolicyMismatch
        | K::UnknownFamily
        | K::UnknownRule
        | K::AdapterMismatch
        | K::MatcherFailure
        | K::IncompleteAccounting
        | K::InsufficientResolution => ProblemCategory::Authority,
        K::SourceDrift
        | K::IncompatiblePredecessor
        | K::SequenceConflict
        | K::RetentionConflict => ProblemCategory::Precondition,
        K::IdempotencyConflict => ProblemCategory::Idempotency,
        K::RequiredSourceUnavailable | K::LockUnavailable => ProblemCategory::Prerequisite,
        K::CorruptStore
        | K::RecoveryFailed
        | K::DurabilityUncertain
        | K::CompositionFailedAfterCommit
        | K::DeliveryUncertainAfterCommit => ProblemCategory::Implementation,
    }
}

fn snapshot_retry_directive(class: SnapshotGroundingErrorClassV1) -> RetryDirective {
    match class {
        SnapshotGroundingErrorClassV1::Refusal | SnapshotGroundingErrorClassV1::Conflict => {
            RetryDirective::AfterRequestChange
        }
        SnapshotGroundingErrorClassV1::Unavailable => RetryDirective::AfterRecheck,
        SnapshotGroundingErrorClassV1::Error => RetryDirective::Transient,
        SnapshotGroundingErrorClassV1::PostCommit => RetryDirective::Never,
    }
}

fn snapshot_idempotency_stage(kind: SnapshotGroundingErrorKindV1) -> IdempotencyStage {
    use SnapshotGroundingErrorKindV1 as K;
    match kind {
        K::MalformedInput => IdempotencyStage::RequestValidation,
        K::InputLimitExceeded | K::RedactionUncertain => IdempotencyStage::SafetyValidation,
        K::IdempotencyConflict => IdempotencyStage::IdempotencyValidation,
        K::SourceDrift
        | K::IncompatiblePredecessor
        | K::SequenceConflict
        | K::RetentionConflict => IdempotencyStage::PreconditionValidation,
        K::UnknownReference
        | K::SubstitutedReference
        | K::RepositoryMismatch
        | K::PolicyMismatch
        | K::UnknownFamily
        | K::UnknownRule
        | K::AdapterMismatch
        | K::MatcherFailure
        | K::IncompleteAccounting
        | K::InsufficientResolution
        | K::RequiredSourceUnavailable
        | K::LockUnavailable => IdempotencyStage::AuthorityValidation,
        K::CorruptStore
        | K::RecoveryFailed
        | K::DurabilityUncertain
        | K::CompositionFailedAfterCommit
        | K::DeliveryUncertainAfterCommit => IdempotencyStage::ExecutionStart,
    }
}

fn snapshot_error_class_name(class: SnapshotGroundingErrorClassV1) -> &'static str {
    match class {
        SnapshotGroundingErrorClassV1::Refusal => "refused",
        SnapshotGroundingErrorClassV1::Conflict => "conflict",
        SnapshotGroundingErrorClassV1::Unavailable => "unavailable",
        SnapshotGroundingErrorClassV1::Error => "error",
        SnapshotGroundingErrorClassV1::PostCommit => "post_commit",
    }
}

fn snapshot_error_kind_name(kind: SnapshotGroundingErrorKindV1) -> &'static str {
    use SnapshotGroundingErrorKindV1 as K;
    match kind {
        K::MalformedInput => "malformed_input",
        K::InputLimitExceeded => "input_limit_exceeded",
        K::UnknownReference => "unknown_reference",
        K::SubstitutedReference => "substituted_reference",
        K::RepositoryMismatch => "repository_mismatch",
        K::PolicyMismatch => "policy_mismatch",
        K::UnknownFamily => "unknown_family",
        K::UnknownRule => "unknown_rule",
        K::AdapterMismatch => "adapter_mismatch",
        K::MatcherFailure => "matcher_failure",
        K::SourceDrift => "source_drift",
        K::IncompatiblePredecessor => "incompatible_predecessor",
        K::SequenceConflict => "sequence_conflict",
        K::IncompleteAccounting => "incomplete_accounting",
        K::InsufficientResolution => "insufficient_resolution",
        K::RedactionUncertain => "redaction_uncertain",
        K::RetentionConflict => "retention_conflict",
        K::CorruptStore => "corrupt_store",
        K::IdempotencyConflict => "idempotency_conflict",
        K::RequiredSourceUnavailable => "required_source_unavailable",
        K::LockUnavailable => "lock_unavailable",
        K::RecoveryFailed => "recovery_failed",
        K::DurabilityUncertain => "durability_uncertain",
        K::CompositionFailedAfterCommit => "composition_failed_after_commit",
        K::DeliveryUncertainAfterCommit => "delivery_uncertain_after_commit",
    }
}

fn established_response(
    request: &OperationRequestV1,
    definition: &OperationDefinition,
    data: ContractValue,
    receipts: BoundedVec<WriteReceipt, 128>,
    replayed: bool,
) -> Result<OperationResponseV1, ContractError> {
    let key = request
        .idempotency_key_fingerprint()
        .cloned()
        .ok_or(ContractError::InvalidInvariant)?;
    let retention = UtcTimestamp::parse("9999-12-31T23:59:59Z")?;
    let original = if replayed {
        Some(
            response_from_parts(
                request,
                definition,
                data.clone(),
                IdempotencyState::established(
                    key.clone(),
                    request.request_fingerprint().clone(),
                    false,
                    None,
                    retention.clone(),
                )?,
                receipts.clone(),
            )?
            .result_fingerprint()
            .clone(),
        )
    } else {
        None
    };
    response_from_parts(
        request,
        definition,
        data,
        IdempotencyState::established(
            key,
            request.request_fingerprint().clone(),
            replayed,
            original,
            retention,
        )?,
        receipts,
    )
}

fn ok_response<T: Serialize>(
    request: &OperationRequestV1,
    definition: &OperationDefinition,
    data: &T,
) -> Result<OperationResponseV1, ContractError> {
    let data = to_contract_value(data)?;
    SnapshotOperationSchemaValidator.validate_instance(definition.result_schema(), &data)?;
    response_from_parts(
        request,
        definition,
        data,
        IdempotencyState::not_applicable()?,
        BoundedVec::new(Vec::new())?,
    )
}

fn response_from_parts(
    request: &OperationRequestV1,
    definition: &OperationDefinition,
    data: ContractValue,
    idempotency: IdempotencyState,
    receipts: BoundedVec<WriteReceipt, 128>,
) -> Result<OperationResponseV1, ContractError> {
    OperationResponseV1::new(
        binding("handbook.operation-response@1.0.0")?,
        request.request_id().cloned(),
        request.request_fingerprint().clone(),
        definition,
        TerminalFields::new(
            TerminalStatus::Ok,
            Some(data),
            BoundedVec::new(Vec::new())?,
            BoundedVec::new(Vec::new())?,
            BoundedVec::new(Vec::new())?,
        )?,
        idempotency,
        receipts,
        BoundedVec::new(Vec::new())?,
        BoundedVec::new(Vec::new())?,
        &SnapshotOperationSchemaValidator,
        &ContractLimits::packet0(),
    )
}

fn parse_body<T: for<'de> Deserialize<'de> + Serialize>(
    request: &OperationRequestV1,
) -> Result<T, ContractError> {
    let limits = ContractLimits::packet0();
    parse_canonical_json(&canonical_json_bytes(request.body(), &limits)?, &limits)
}

fn to_contract_value<T: Serialize>(value: &T) -> Result<ContractValue, ContractError> {
    let limits = ContractLimits::packet0();
    parse_canonical_json(&canonical_json_bytes(value, &limits)?, &limits)
}

fn validate_session_ref(value: &str) -> Result<(), ContractError> {
    if value.is_empty() || value.len() > 1024 || value.chars().any(char::is_control) {
        Err(ContractError::InvalidInvariant)
    } else {
        Ok(())
    }
}

fn malformed_capture_error() -> handbook_engine::SnapshotCaptureErrorV1 {
    SnapshotCaptureRequestV1::new([], "", SnapshotBoundaryKindV1::Start)
        .expect_err("empty session must produce the stable malformed-input owner error")
}

fn schema_ref(operation_ref: &str, suffix: &str) -> String {
    let operation = operation_ref
        .strip_suffix("@1.0.0")
        .unwrap_or(operation_ref);
    format!(
        "handbook.operation.{}-{suffix}@1.0.0",
        operation.replace('.', "-")
    )
}

fn binding(reference: &str) -> Result<ExactBinding, ContractError> {
    super::production_witness::current_schema_binding(reference)
}

fn schema_source(reference: &str) -> Option<&'static str> {
    macro_rules! schema {
        ($name:literal) => {
            include_str!(concat!(
                "../../schemas/2020-12/",
                $name,
                "/1.0.0.schema.json"
            ))
        };
    }
    Some(match reference {
        "handbook.operation.snapshot-capture-request@1.0.0" => {
            schema!("handbook.operation.snapshot-capture-request")
        }
        "handbook.operation.snapshot-capture-result@1.0.0" => {
            schema!("handbook.operation.snapshot-capture-result")
        }
        "handbook.operation.snapshot-delta-request@1.0.0" => {
            schema!("handbook.operation.snapshot-delta-request")
        }
        "handbook.operation.snapshot-delta-result@1.0.0" => {
            schema!("handbook.operation.snapshot-delta-result")
        }
        "handbook.operation.snapshot-project-request@1.0.0" => {
            schema!("handbook.operation.snapshot-project-request")
        }
        "handbook.operation.snapshot-project-result@1.0.0" => {
            schema!("handbook.operation.snapshot-project-result")
        }
        "handbook.operation.snapshot-read-request@1.0.0" => {
            schema!("handbook.operation.snapshot-read-request")
        }
        "handbook.operation.snapshot-read-result@1.0.0" => {
            schema!("handbook.operation.snapshot-read-result")
        }
        _ => return None,
    })
}
