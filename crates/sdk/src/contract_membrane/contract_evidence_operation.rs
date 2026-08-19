use super::{
    canonical_json_bytes, parse_canonical_json, BoundedString, BoundedVec, CapabilityEntry,
    ContractError, ContractLimits, ContractValue, CorrelationRef, CurrentTransportEvidence,
    CurrentTransportEvidenceSealed, DeclaredWrite, ExactBinding, ExactRef, Fingerprint,
    JsonPointer, OperationAdmissionResult, OperationClass, OperationDefinition,
    OperationIdempotency, OperationOutcomeSchemas, OperationOwnerBinding, OperationRequestV1,
    OperationResponseV1, OperationSchemaValidator, OperationTransport, OwnerAdmissionToken,
    Problem, ProblemBinding, ProblemCategory, RequestId, RetryDirective, SafeU64, SemVer,
    TerminalFields, TerminalStatus, UtcTimestamp, WriteReceipt,
};
use handbook_contracts::{
    ClaimVerdictEvaluationErrorKindV1, ContentAddressedBindingV1, ContractEvaluationRunV1,
    ContractOwnerRepositoryErrorKindV1, ContractOwnerRepositoryV1, EvaluationPolicyRegistryV1,
    EvidenceCandidateSourceV1, EvidenceListRequestV1 as OwnerEvidenceListRequest,
    EvidenceReadOutcomeV1 as OwnerEvidenceReadOutcome, FingerprintV1 as OwnerFingerprint,
    GateEvaluationErrorKindV1, OwnerMutationDispositionV1,
};
use handbook_engine::{ContextResolutionStackDefinition, SourceCurrentnessPortV1};
use serde::{Deserialize, Serialize};
use std::path::Path;

pub const CONTRACT_EVIDENCE_LIST_REF: &str = "contract.evidence.list@1.0.0";
pub const CONTRACT_EVIDENCE_READ_REF: &str = "contract.evidence.read@1.0.0";
pub const CONTRACT_EVIDENCE_APPEND_REF: &str = "contract.evidence.append@1.0.0";
pub const CONTRACT_VERDICT_EVALUATE_REF: &str = "contract.verdict.evaluate@1.0.0";
pub const CONTRACT_GATE_EVALUATE_REF: &str = "contract.gate.evaluate@1.0.0";
pub const CONTRACT_EVIDENCE_OPERATION_REFS: [&str; 5] = [
    CONTRACT_EVIDENCE_LIST_REF,
    CONTRACT_EVIDENCE_READ_REF,
    CONTRACT_EVIDENCE_APPEND_REF,
    CONTRACT_VERDICT_EVALUATE_REF,
    CONTRACT_GATE_EVALUATE_REF,
];

pub const CONTRACT_EVIDENCE_DETAILS_SCHEMA_REF: &str =
    "handbook.contract-evidence-operation-details@1.0.0";

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "contract-schema-generation", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct ContractRecordBindingDtoV1 {
    #[serde(rename = "ref")]
    reference: BoundedString<4096>,
    fingerprint: Fingerprint,
}

impl ContractRecordBindingDtoV1 {
    pub fn new(reference: &str, fingerprint: Fingerprint) -> Result<Self, ContractError> {
        Ok(Self {
            reference: BoundedString::new(reference.to_owned())?,
            fingerprint,
        })
    }

    pub fn reference(&self) -> &str {
        self.reference.as_str()
    }

    pub fn fingerprint(&self) -> &Fingerprint {
        &self.fingerprint
    }
}

macro_rules! semantic_document_dto {
    ($name:ident) => {
        #[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
        #[cfg_attr(feature = "contract-schema-generation", derive(schemars::JsonSchema))]
        #[serde(deny_unknown_fields)]
        pub struct $name {
            binding: ContractRecordBindingDtoV1,
            document: ContractValue,
        }

        impl $name {
            pub fn new(binding: ContractRecordBindingDtoV1, document: ContractValue) -> Self {
                Self { binding, document }
            }

            pub fn binding(&self) -> &ContractRecordBindingDtoV1 {
                &self.binding
            }

            pub fn document(&self) -> &ContractValue {
                &self.document
            }
        }
    };
}

semantic_document_dto!(ContractEvidenceCandidateDtoV1);
semantic_document_dto!(ContractEvidenceRecordDtoV1);
semantic_document_dto!(ContractEvidenceAdmissionPolicyDtoV1);
semantic_document_dto!(ContractEvidenceFreshnessPolicyDtoV1);
semantic_document_dto!(ContractEvidenceStabilityPolicyDtoV1);
semantic_document_dto!(ContractEvaluationRunDtoV1);
semantic_document_dto!(ContractClaimVerdictDtoV1);
semantic_document_dto!(ContractGatePolicyDtoV1);
semantic_document_dto!(ContractGateResultDtoV1);
semantic_document_dto!(ContractLocalCloseoutPolicyDtoV1);
semantic_document_dto!(ContractLocalCloseoutResultDtoV1);

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "contract-schema-generation", derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum ContractEvidenceProblemCauseV1 {
    Malformed,
    OverLimit,
    Missing,
    Unknown,
    Stale,
    Substituted,
    Duplicate,
    Conflict,
    IneligibleCandidate,
    InsufficientResolution,
    RepositoryUnavailable,
    CandidateSourceUnavailable,
    DurabilityUncertain,
    InternalInvariant,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "contract-schema-generation", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct ContractEvidenceOperationDetailsDtoV1 {
    cause: ContractEvidenceProblemCauseV1,
    related_refs: BoundedVec<BoundedString<4096>, 128>,
    diagnostics: BoundedVec<BoundedString<4096>, 128>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "contract-schema-generation", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct ContractEvidenceListRequestV1 {
    limit: SafeU64,
    root: Option<Fingerprint>,
    cursor: Option<BoundedString<4096>>,
}

impl ContractEvidenceListRequestV1 {
    pub fn first(limit: u64) -> Result<Self, ContractError> {
        let value = Self {
            limit: SafeU64::new(limit)?,
            root: None,
            cursor: None,
        };
        value.validate()?;
        Ok(value)
    }

    pub fn continue_from(
        limit: u64,
        root: Fingerprint,
        cursor: &str,
    ) -> Result<Self, ContractError> {
        let value = Self {
            limit: SafeU64::new(limit)?,
            root: Some(root),
            cursor: Some(BoundedString::new(cursor.to_owned())?),
        };
        value.validate()?;
        Ok(value)
    }

    fn validate(&self) -> Result<(), ContractError> {
        if !(1..=100).contains(&self.limit.get()) || self.root.is_some() != self.cursor.is_some() {
            return Err(ContractError::InvalidInvariant);
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "contract-schema-generation", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct ContractEvidenceListEntryV1 {
    sequence: SafeU64,
    evidence: ContractRecordBindingDtoV1,
    evaluation_run: ContractRecordBindingDtoV1,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "contract-schema-generation", derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum ContractEvidenceRetentionV1 {
    ImmutableAll,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "contract-schema-generation", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct ContractEvidenceListResultV1 {
    root: Fingerprint,
    entries: BoundedVec<ContractEvidenceListEntryV1, 100>,
    next_cursor: Option<BoundedString<4096>>,
    retention: ContractEvidenceRetentionV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "contract-schema-generation", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct ContractEvidenceReadRequestV1 {
    evidence: ContractRecordBindingDtoV1,
}

impl ContractEvidenceReadRequestV1 {
    pub fn new(evidence: ContractRecordBindingDtoV1) -> Self {
        Self { evidence }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "contract-schema-generation", derive(schemars::JsonSchema))]
#[serde(tag = "outcome", rename_all = "snake_case", deny_unknown_fields)]
pub enum ContractEvidenceReadResultV1 {
    Found {
        evidence: ContractEvidenceRecordDtoV1,
    },
    Absent {
        requested: ContractRecordBindingDtoV1,
    },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "contract-schema-generation", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct ContractEvidenceAppendRequestV1 {
    candidate: ContractRecordBindingDtoV1,
}

impl ContractEvidenceAppendRequestV1 {
    pub fn new(candidate: ContractRecordBindingDtoV1) -> Self {
        Self { candidate }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "contract-schema-generation", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub(crate) struct ContractEvidenceAppendWireV1 {
    candidate: ContractRecordBindingDtoV1,
    idempotency_key: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "contract-schema-generation", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct ContractEvidenceAppendReceiptDtoV1 {
    sequence: SafeU64,
    request_fingerprint: Fingerprint,
    candidate: ContractRecordBindingDtoV1,
    evidence: ContractRecordBindingDtoV1,
    receipt_fingerprint: Fingerprint,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "contract-schema-generation", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct ContractEvidenceAppendResultV1 {
    evidence: ContractEvidenceRecordDtoV1,
    receipt: ContractEvidenceAppendReceiptDtoV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "contract-schema-generation", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct ContractVerdictEvaluateRequestV1 {
    evaluation_run: ContractEvaluationRunDtoV1,
}

impl ContractVerdictEvaluateRequestV1 {
    pub fn new(evaluation_run: ContractEvaluationRunDtoV1) -> Self {
        Self { evaluation_run }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "contract-schema-generation", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct ContractVerdictEvaluateResultV1 {
    evidence_repository_root: Fingerprint,
    verdicts: BoundedVec<ContractClaimVerdictDtoV1, 128>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "contract-schema-generation", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct ContractGateEvaluateRequestV1 {
    evaluation_run: ContractEvaluationRunDtoV1,
}

impl ContractGateEvaluateRequestV1 {
    pub fn new(evaluation_run: ContractEvaluationRunDtoV1) -> Self {
        Self { evaluation_run }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "contract-schema-generation", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct ContractGateEvaluateResultV1 {
    verdicts: BoundedVec<ContractClaimVerdictDtoV1, 128>,
    gate: ContractGateResultDtoV1,
    local_closeout: ContractLocalCloseoutResultDtoV1,
}

pub struct ContractEvidenceOperationSchemaValidator;

impl OperationSchemaValidator for ContractEvidenceOperationSchemaValidator {
    fn validate_instance(
        &self,
        schema: &ExactBinding,
        instance: &ContractValue,
    ) -> Result<(), ContractError> {
        let bytes = canonical_json_bytes(instance, &ContractLimits::packet0())?;
        match schema.reference().as_str() {
            "handbook.operation.contract-evidence-list-request@1.0.0" => {
                parse_canonical_json::<ContractEvidenceListRequestV1>(
                    &bytes,
                    &ContractLimits::packet0(),
                )?
                .validate()
            }
            "handbook.operation.contract-evidence-list-result@1.0.0" => {
                parse::<ContractEvidenceListResultV1>(&bytes)
            }
            "handbook.operation.contract-evidence-read-request@1.0.0" => {
                parse::<ContractEvidenceReadRequestV1>(&bytes)
            }
            "handbook.operation.contract-evidence-read-result@1.0.0" => {
                parse::<ContractEvidenceReadResultV1>(&bytes)
            }
            "handbook.operation.contract-evidence-append-request@1.0.0" => {
                parse::<ContractEvidenceAppendWireV1>(&bytes)
            }
            "handbook.operation.contract-evidence-append-result@1.0.0" => {
                parse::<ContractEvidenceAppendResultV1>(&bytes)
            }
            "handbook.operation.contract-verdict-evaluate-request@1.0.0" => {
                parse::<ContractVerdictEvaluateRequestV1>(&bytes)
            }
            "handbook.operation.contract-verdict-evaluate-result@1.0.0" => {
                parse::<ContractVerdictEvaluateResultV1>(&bytes)
            }
            "handbook.operation.contract-gate-evaluate-request@1.0.0" => {
                parse::<ContractGateEvaluateRequestV1>(&bytes)
            }
            "handbook.operation.contract-gate-evaluate-result@1.0.0" => {
                parse::<ContractGateEvaluateResultV1>(&bytes)
            }
            CONTRACT_EVIDENCE_DETAILS_SCHEMA_REF => {
                parse::<ContractEvidenceOperationDetailsDtoV1>(&bytes)
            }
            _ => Err(ContractError::InvalidInvariant),
        }
    }
}

fn parse<T: for<'de> Deserialize<'de> + Serialize>(bytes: &[u8]) -> Result<(), ContractError> {
    parse_canonical_json::<T>(bytes, &ContractLimits::packet0()).map(|_| ())
}

pub fn contract_evidence_list_operation_request(
    request_id: Option<&str>,
    body: ContractEvidenceListRequestV1,
) -> Result<OperationRequestV1, ContractError> {
    operation_request(
        CONTRACT_EVIDENCE_LIST_REF,
        request_id,
        super::artifact_operation::to_contract_value(&body)?,
    )
}

pub fn contract_evidence_read_operation_request(
    request_id: Option<&str>,
    body: ContractEvidenceReadRequestV1,
) -> Result<OperationRequestV1, ContractError> {
    operation_request(
        CONTRACT_EVIDENCE_READ_REF,
        request_id,
        super::artifact_operation::to_contract_value(&body)?,
    )
}

pub fn contract_evidence_append_operation_request(
    request_id: Option<&str>,
    idempotency_key: &str,
    body: ContractEvidenceAppendRequestV1,
) -> Result<OperationRequestV1, ContractError> {
    operation_request(
        CONTRACT_EVIDENCE_APPEND_REF,
        request_id,
        super::artifact_operation::to_contract_value(&ContractEvidenceAppendWireV1 {
            candidate: body.candidate,
            idempotency_key: idempotency_key.to_owned(),
        })?,
    )
}

pub fn contract_verdict_evaluate_operation_request(
    request_id: Option<&str>,
    body: ContractVerdictEvaluateRequestV1,
) -> Result<OperationRequestV1, ContractError> {
    operation_request(
        CONTRACT_VERDICT_EVALUATE_REF,
        request_id,
        super::artifact_operation::to_contract_value(&body)?,
    )
}

pub fn contract_gate_evaluate_operation_request(
    request_id: Option<&str>,
    body: ContractGateEvaluateRequestV1,
) -> Result<OperationRequestV1, ContractError> {
    operation_request(
        CONTRACT_GATE_EVALUATE_REF,
        request_id,
        super::artifact_operation::to_contract_value(&body)?,
    )
}

fn operation_request(
    operation_ref: &str,
    request_id: Option<&str>,
    body: ContractValue,
) -> Result<OperationRequestV1, ContractError> {
    let definition = contract_evidence_operation_definition(operation_ref)?;
    OperationRequestV1::new(
        schema_binding("handbook.operation-request@1.0.0")?,
        request_id.map(RequestId::parse).transpose()?,
        &definition,
        body,
        &ContractEvidenceOperationSchemaValidator,
        &ContractLimits::packet0(),
    )
}

pub fn execute_contract_evidence_operation(
    repo_root: &Path,
    request: &OperationRequestV1,
) -> Result<OperationResponseV1, ContractError> {
    let definition =
        contract_evidence_operation_definition(request.operation().reference().as_str())?;
    if request.operation() != &definition.binding()
        || request.body_schema() != definition.request_schema()
    {
        return Err(ContractError::InvalidInvariant);
    }
    match request.operation().reference().as_str() {
        CONTRACT_EVIDENCE_LIST_REF => execute_list(repo_root, request, &definition),
        CONTRACT_EVIDENCE_READ_REF => execute_read(repo_root, request, &definition),
        CONTRACT_EVIDENCE_APPEND_REF => unavailable_response(
            request,
            &definition,
            ContractEvidenceProblemCauseV1::CandidateSourceUnavailable,
            true,
        ),
        CONTRACT_VERDICT_EVALUATE_REF | CONTRACT_GATE_EVALUATE_REF => unavailable_response(
            request,
            &definition,
            ContractEvidenceProblemCauseV1::RepositoryUnavailable,
            false,
        ),
        _ => Err(ContractError::InvalidInvariant),
    }
}

#[allow(clippy::too_many_arguments)]
pub fn execute_contract_evidence_operation_with_context<S, C>(
    repo_root: &Path,
    request: &OperationRequestV1,
    source: &mut S,
    source_currentness: &mut C,
    resolution_stack: &ContextResolutionStackDefinition,
    policy_registry: &EvaluationPolicyRegistryV1,
) -> Result<OperationResponseV1, ContractError>
where
    S: EvidenceCandidateSourceV1,
    C: SourceCurrentnessPortV1,
{
    let definition =
        contract_evidence_operation_definition(request.operation().reference().as_str())?;
    if request.operation() != &definition.binding()
        || request.body_schema() != definition.request_schema()
    {
        return Err(ContractError::InvalidInvariant);
    }
    match request.operation().reference().as_str() {
        CONTRACT_EVIDENCE_LIST_REF => execute_list(repo_root, request, &definition),
        CONTRACT_EVIDENCE_READ_REF => execute_read(repo_root, request, &definition),
        CONTRACT_EVIDENCE_APPEND_REF => execute_append(
            repo_root,
            request,
            &definition,
            source,
            source_currentness,
            resolution_stack,
            policy_registry,
        ),
        CONTRACT_VERDICT_EVALUATE_REF => execute_verdict(
            repo_root,
            request,
            &definition,
            source,
            source_currentness,
            resolution_stack,
            policy_registry,
        ),
        CONTRACT_GATE_EVALUATE_REF => execute_gate(
            repo_root,
            request,
            &definition,
            source,
            source_currentness,
            resolution_stack,
            policy_registry,
        ),
        _ => Err(ContractError::InvalidInvariant),
    }
}

fn execute_list(
    repo_root: &Path,
    request: &OperationRequestV1,
    definition: &OperationDefinition,
) -> Result<OperationResponseV1, ContractError> {
    let body: ContractEvidenceListRequestV1 = from_body(request)?;
    body.validate()?;
    let owner_request = match (&body.root, &body.cursor) {
        (None, None) => OwnerEvidenceListRequest::first(body.limit.get() as usize),
        (Some(root), Some(cursor)) => OwnerEvidenceListRequest::continue_from(
            body.limit.get() as usize,
            OwnerFingerprint::parse(root.as_str()).map_err(|_| ContractError::InvalidInvariant)?,
            cursor.as_str(),
        ),
        _ => return Err(ContractError::InvalidInvariant),
    }
    .map_err(|_| ContractError::InvalidInvariant)?;
    let page = match ContractOwnerRepositoryV1::open(repo_root)
        .and_then(|owner| owner.list_evidence(owner_request))
    {
        Ok(page) => page,
        Err(error) => {
            return problem_response(request, definition, owner_problem(error.kind()), false)
        }
    };
    let entries = page
        .entries()
        .iter()
        .map(|entry| {
            Ok(ContractEvidenceListEntryV1 {
                sequence: SafeU64::new(entry.sequence())?,
                evidence: record_binding(
                    entry.evidence_ref(),
                    entry.evidence_fingerprint().as_str(),
                )?,
                evaluation_run: record_binding(
                    entry.evaluation_run().reference(),
                    entry.evaluation_run().fingerprint().as_str(),
                )?,
            })
        })
        .collect::<Result<Vec<_>, ContractError>>()?;
    let result = ContractEvidenceListResultV1 {
        root: Fingerprint::parse(page.root().as_str())?,
        entries: BoundedVec::new(entries)?,
        next_cursor: page
            .next_cursor()
            .map(|cursor| BoundedString::new(cursor.to_owned()))
            .transpose()?,
        retention: ContractEvidenceRetentionV1::ImmutableAll,
    };
    success_response(request, definition, &result)
}

fn execute_read(
    repo_root: &Path,
    request: &OperationRequestV1,
    definition: &OperationDefinition,
) -> Result<OperationResponseV1, ContractError> {
    let body: ContractEvidenceReadRequestV1 = from_body(request)?;
    let fingerprint = OwnerFingerprint::parse(body.evidence.fingerprint().as_str())
        .map_err(|_| ContractError::InvalidInvariant)?;
    let result = match ContractOwnerRepositoryV1::open(repo_root)
        .and_then(|owner| owner.read_evidence(body.evidence.reference(), &fingerprint))
    {
        Ok(result) => result,
        Err(error) => {
            return problem_response(request, definition, owner_problem(error.kind()), false)
        }
    };
    let result = match result {
        OwnerEvidenceReadOutcome::Found(stored) => ContractEvidenceReadResultV1::Found {
            evidence: evidence_record_dto(stored.record())?,
        },
        OwnerEvidenceReadOutcome::Absent => ContractEvidenceReadResultV1::Absent {
            requested: body.evidence,
        },
    };
    success_response(request, definition, &result)
}

#[allow(clippy::too_many_arguments)]
fn execute_append<S, C>(
    repo_root: &Path,
    request: &OperationRequestV1,
    definition: &OperationDefinition,
    source: &mut S,
    source_currentness: &mut C,
    resolution_stack: &ContextResolutionStackDefinition,
    policy_registry: &EvaluationPolicyRegistryV1,
) -> Result<OperationResponseV1, ContractError>
where
    S: EvidenceCandidateSourceV1,
    C: SourceCurrentnessPortV1,
{
    let body: ContractEvidenceAppendRequestV1 = from_body(request)?;
    let candidate = match ContentAddressedBindingV1::new(
        body.candidate.reference(),
        OwnerFingerprint::parse(body.candidate.fingerprint().as_str())
            .map_err(|_| ContractError::InvalidInvariant)?,
    ) {
        Ok(candidate) => candidate,
        Err(_) => {
            return problem_response(
                request,
                definition,
                cause_problem(ContractEvidenceProblemCauseV1::Malformed),
                true,
            )
        }
    };
    let owner = match ContractOwnerRepositoryV1::open(repo_root) {
        Ok(owner) => owner,
        Err(error) => {
            return problem_response(request, definition, owner_problem(error.kind()), true)
        }
    };
    let mut bridge = EvidenceAppendBridge {
        owner: &owner,
        source,
        source_currentness,
        resolution_stack,
        policy_registry,
        candidate: &candidate,
        result: None,
    };
    let scoped_key = request
        .with_owner_idempotency_key(&mut bridge)
        .ok_or(ContractError::InvalidInvariant)?;
    let execution = match bridge.result.ok_or(ContractError::InvalidInvariant)? {
        Ok(execution) => execution,
        Err(error) => {
            return problem_response(request, definition, owner_problem(error.kind()), true);
        }
    };
    let receipt = execution.receipt();
    let replayed = execution.disposition() == OwnerMutationDispositionV1::Replayed;
    let result = ContractEvidenceAppendResultV1 {
        evidence: evidence_record_dto(execution.record())?,
        receipt: ContractEvidenceAppendReceiptDtoV1 {
            sequence: SafeU64::new(receipt.sequence())?,
            request_fingerprint: Fingerprint::parse(receipt.request_fingerprint().as_str())?,
            candidate: record_binding(
                receipt.candidate().reference(),
                receipt.candidate().fingerprint().as_str(),
            )?,
            evidence: record_binding(
                receipt.evidence_ref(),
                receipt.evidence_fingerprint().as_str(),
            )?,
            receipt_fingerprint: Fingerprint::parse(receipt.receipt_fingerprint().as_str())?,
        },
    };
    mutation_success_response(request, definition, scoped_key, replayed, &result)
}

#[allow(clippy::too_many_arguments)]
fn execute_verdict<S, C>(
    repo_root: &Path,
    request: &OperationRequestV1,
    definition: &OperationDefinition,
    source: &mut S,
    source_currentness: &mut C,
    resolution_stack: &ContextResolutionStackDefinition,
    policy_registry: &EvaluationPolicyRegistryV1,
) -> Result<OperationResponseV1, ContractError>
where
    S: EvidenceCandidateSourceV1,
    C: SourceCurrentnessPortV1,
{
    let body: ContractVerdictEvaluateRequestV1 = from_body(request)?;
    let run = match readmit_evaluation_run(&body.evaluation_run) {
        Ok(run) => run,
        Err(cause) => return problem_response(request, definition, cause_problem(cause), false),
    };
    let owner = match ContractOwnerRepositoryV1::open(repo_root) {
        Ok(owner) => owner,
        Err(error) => {
            return problem_response(request, definition, owner_problem(error.kind()), false)
        }
    };
    let evaluation = match handbook_contracts::evaluate_claim_verdicts_v1(
        &owner,
        source,
        source_currentness,
        resolution_stack,
        policy_registry,
        &run,
    ) {
        Ok(evaluation) => evaluation,
        Err(error) => {
            return problem_response(request, definition, verdict_problem(error.kind()), false)
        }
    };
    let verdicts = evaluation
        .verdicts()
        .iter()
        .map(|verdict| {
            semantic_dto(
                verdict.record_ref(),
                verdict.record_fingerprint().as_str(),
                verdict,
            )
        })
        .collect::<Result<Vec<_>, ContractError>>()?;
    success_response(
        request,
        definition,
        &ContractVerdictEvaluateResultV1 {
            evidence_repository_root: Fingerprint::parse(
                evaluation.evidence_repository_root().as_str(),
            )?,
            verdicts: BoundedVec::new(verdicts)?,
        },
    )
}

#[allow(clippy::too_many_arguments)]
fn execute_gate<S, C>(
    repo_root: &Path,
    request: &OperationRequestV1,
    definition: &OperationDefinition,
    source: &mut S,
    source_currentness: &mut C,
    resolution_stack: &ContextResolutionStackDefinition,
    policy_registry: &EvaluationPolicyRegistryV1,
) -> Result<OperationResponseV1, ContractError>
where
    S: EvidenceCandidateSourceV1,
    C: SourceCurrentnessPortV1,
{
    let body: ContractGateEvaluateRequestV1 = from_body(request)?;
    let run = match readmit_evaluation_run(&body.evaluation_run) {
        Ok(run) => run,
        Err(cause) => return problem_response(request, definition, cause_problem(cause), false),
    };
    let owner = match ContractOwnerRepositoryV1::open(repo_root) {
        Ok(owner) => owner,
        Err(error) => {
            return problem_response(request, definition, owner_problem(error.kind()), false)
        }
    };
    let evaluation = match handbook_contracts::evaluate_gate_and_local_closeout_v1(
        &owner,
        source,
        source_currentness,
        resolution_stack,
        policy_registry,
        &run,
    ) {
        Ok(evaluation) => evaluation,
        Err(error) => {
            return problem_response(request, definition, gate_problem(error.kind()), false)
        }
    };
    let verdicts = evaluation
        .verdicts()
        .iter()
        .map(|verdict| {
            semantic_dto(
                verdict.record_ref(),
                verdict.record_fingerprint().as_str(),
                verdict,
            )
        })
        .collect::<Result<Vec<_>, ContractError>>()?;
    success_response(
        request,
        definition,
        &ContractGateEvaluateResultV1 {
            verdicts: BoundedVec::new(verdicts)?,
            gate: semantic_dto(
                evaluation.gate_result().record_ref(),
                evaluation.gate_result().record_fingerprint().as_str(),
                evaluation.gate_result(),
            )?,
            local_closeout: semantic_dto(
                evaluation.local_closeout_result().record_ref(),
                evaluation
                    .local_closeout_result()
                    .record_fingerprint()
                    .as_str(),
                evaluation.local_closeout_result(),
            )?,
        },
    )
}

fn readmit_evaluation_run(
    value: &ContractEvaluationRunDtoV1,
) -> Result<ContractEvaluationRunV1, ContractEvidenceProblemCauseV1> {
    let bytes = canonical_json_bytes(value.document(), &ContractLimits::packet0())
        .map_err(|_| ContractEvidenceProblemCauseV1::Malformed)?;
    let run: ContractEvaluationRunV1 = parse_canonical_json(&bytes, &ContractLimits::packet0())
        .map_err(|_| ContractEvidenceProblemCauseV1::Malformed)?;
    if value.binding().reference() != run.evaluation_run_ref()
        || value.binding().fingerprint().as_str() != run.evaluation_run_fingerprint().as_str()
    {
        return Err(ContractEvidenceProblemCauseV1::Substituted);
    }
    Ok(run)
}

fn semantic_dto<T: Serialize, D>(
    reference: &str,
    fingerprint: &str,
    value: &T,
) -> Result<D, ContractError>
where
    D: FromSemanticDocument,
{
    D::from_semantic_document(
        record_binding(reference, fingerprint)?,
        super::artifact_operation::to_contract_value(value)?,
    )
}

trait FromSemanticDocument: Sized {
    fn from_semantic_document(
        binding: ContractRecordBindingDtoV1,
        document: ContractValue,
    ) -> Result<Self, ContractError>;
}

impl FromSemanticDocument for ContractClaimVerdictDtoV1 {
    fn from_semantic_document(
        binding: ContractRecordBindingDtoV1,
        document: ContractValue,
    ) -> Result<Self, ContractError> {
        Ok(Self::new(binding, document))
    }
}

impl FromSemanticDocument for ContractGateResultDtoV1 {
    fn from_semantic_document(
        binding: ContractRecordBindingDtoV1,
        document: ContractValue,
    ) -> Result<Self, ContractError> {
        Ok(Self::new(binding, document))
    }
}

impl FromSemanticDocument for ContractLocalCloseoutResultDtoV1 {
    fn from_semantic_document(
        binding: ContractRecordBindingDtoV1,
        document: ContractValue,
    ) -> Result<Self, ContractError> {
        Ok(Self::new(binding, document))
    }
}

struct EvidenceAppendBridge<'a, S, C> {
    owner: &'a ContractOwnerRepositoryV1,
    source: &'a mut S,
    source_currentness: &'a mut C,
    resolution_stack: &'a ContextResolutionStackDefinition,
    policy_registry: &'a EvaluationPolicyRegistryV1,
    candidate: &'a ContentAddressedBindingV1,
    result: Option<
        Result<
            handbook_contracts::EvidenceAppendExecutionV1,
            handbook_contracts::ContractOwnerRepositoryErrorV1,
        >,
    >,
}

impl<S, C> super::IdempotencyKeyConsumerSealed for EvidenceAppendBridge<'_, S, C>
where
    S: EvidenceCandidateSourceV1,
    C: SourceCurrentnessPortV1,
{
    fn consume_owner_idempotency_key(&mut self, _pointer: &str, bytes: &[u8]) -> Fingerprint {
        let key = std::str::from_utf8(bytes).expect("raw key admission validated UTF-8");
        self.result = Some(self.owner.append_evidence_candidate_v1(
            self.source,
            self.source_currentness,
            self.resolution_stack,
            self.policy_registry,
            self.candidate,
            key,
        ));
        super::sha256_fingerprint(bytes)
    }
}

impl<S, C> super::IdempotencyKeyConsumer for EvidenceAppendBridge<'_, S, C>
where
    S: EvidenceCandidateSourceV1,
    C: SourceCurrentnessPortV1,
{
}

fn evidence_record_dto(
    record: &handbook_contracts::EvidenceRecordV1,
) -> Result<ContractEvidenceRecordDtoV1, ContractError> {
    Ok(ContractEvidenceRecordDtoV1::new(
        record_binding(record.record_ref(), record.record_fingerprint().as_str())?,
        super::artifact_operation::to_contract_value(record)?,
    ))
}

fn record_binding(
    reference: &str,
    fingerprint: &str,
) -> Result<ContractRecordBindingDtoV1, ContractError> {
    ContractRecordBindingDtoV1::new(reference, Fingerprint::parse(fingerprint)?)
}

fn from_body<T: for<'de> Deserialize<'de> + Serialize>(
    request: &OperationRequestV1,
) -> Result<T, ContractError> {
    let bytes = canonical_json_bytes(request.body(), &ContractLimits::packet0())?;
    parse_canonical_json(&bytes, &ContractLimits::packet0())
}

fn success_response<T: Serialize>(
    request: &OperationRequestV1,
    definition: &OperationDefinition,
    result: &T,
) -> Result<OperationResponseV1, ContractError> {
    let terminal = TerminalFields::new(
        TerminalStatus::Ok,
        Some(super::artifact_operation::to_contract_value(result)?),
        BoundedVec::new(Vec::new())?,
        BoundedVec::new(Vec::new())?,
        BoundedVec::new(Vec::new())?,
    )?;
    OperationResponseV1::new(
        schema_binding("handbook.operation-response@1.0.0")?,
        request.request_id().cloned(),
        request.request_fingerprint().clone(),
        definition,
        terminal,
        super::IdempotencyState::not_applicable()?,
        BoundedVec::new(Vec::new())?,
        BoundedVec::new(Vec::new())?,
        BoundedVec::new(Vec::new())?,
        &ContractEvidenceOperationSchemaValidator,
        &ContractLimits::packet0(),
    )
}

fn mutation_success_response(
    request: &OperationRequestV1,
    definition: &OperationDefinition,
    scoped_key: Fingerprint,
    replayed: bool,
    result: &ContractEvidenceAppendResultV1,
) -> Result<OperationResponseV1, ContractError> {
    let limits = ContractLimits::packet0();
    let data = super::artifact_operation::to_contract_value(result)?;
    let receipt = WriteReceipt::new(
        BoundedString::new("contract_evidence_record".to_owned())?,
        ExactBinding::new(
            ExactRef::parse("handbook.contract-evidence-record@1.0.0")?,
            result.evidence.binding().fingerprint().clone(),
        ),
        BoundedString::new("contract_evidence_admission".to_owned())?,
        data.clone(),
        BoundedString::new("contract_evidence_append".to_owned())?,
        &limits,
    )?;
    let terminal = TerminalFields::new(
        TerminalStatus::Ok,
        Some(data),
        BoundedVec::new(Vec::new())?,
        BoundedVec::new(Vec::new())?,
        BoundedVec::new(Vec::new())?,
    )?;
    let receipts = BoundedVec::new(vec![receipt])?;
    let retention = UtcTimestamp::parse("9999-12-31T23:59:59Z")?;
    let original = if replayed {
        Some(
            OperationResponseV1::new(
                schema_binding("handbook.operation-response@1.0.0")?,
                request.request_id().cloned(),
                request.request_fingerprint().clone(),
                definition,
                terminal.clone(),
                super::IdempotencyState::established(
                    scoped_key.clone(),
                    request.request_fingerprint().clone(),
                    false,
                    None,
                    retention.clone(),
                )?,
                receipts.clone(),
                BoundedVec::new(Vec::new())?,
                BoundedVec::new(Vec::new())?,
                &ContractEvidenceOperationSchemaValidator,
                &limits,
            )?
            .result_fingerprint()
            .clone(),
        )
    } else {
        None
    };
    OperationResponseV1::new(
        schema_binding("handbook.operation-response@1.0.0")?,
        request.request_id().cloned(),
        request.request_fingerprint().clone(),
        definition,
        terminal,
        super::IdempotencyState::established(
            scoped_key,
            request.request_fingerprint().clone(),
            replayed,
            original,
            retention,
        )?,
        receipts,
        BoundedVec::new(Vec::new())?,
        BoundedVec::new(Vec::new())?,
        &ContractEvidenceOperationSchemaValidator,
        &limits,
    )
}

fn unavailable_response(
    request: &OperationRequestV1,
    definition: &OperationDefinition,
    cause: ContractEvidenceProblemCauseV1,
    mutation: bool,
) -> Result<OperationResponseV1, ContractError> {
    problem_response(request, definition, cause_problem(cause), mutation)
}

#[derive(Clone, Copy)]
struct ProblemProjection {
    cause: ContractEvidenceProblemCauseV1,
    status: TerminalStatus,
    stage: super::IdempotencyStage,
    category: ProblemCategory,
    retry: RetryDirective,
}

fn owner_problem(kind: ContractOwnerRepositoryErrorKindV1) -> ProblemProjection {
    use ContractOwnerRepositoryErrorKindV1 as K;
    cause_problem(match kind {
        K::InvalidRequest => ContractEvidenceProblemCauseV1::Malformed,
        K::EvidenceCandidateMissing => ContractEvidenceProblemCauseV1::Missing,
        K::EvidenceCandidateIneligible => ContractEvidenceProblemCauseV1::IneligibleCandidate,
        K::EvidenceResolutionInsufficient => ContractEvidenceProblemCauseV1::InsufficientResolution,
        K::EvidenceSourceRefused => ContractEvidenceProblemCauseV1::Stale,
        K::EvidenceSourceUnavailable => ContractEvidenceProblemCauseV1::CandidateSourceUnavailable,
        K::BoundExceeded => ContractEvidenceProblemCauseV1::OverLimit,
        K::RecordSubstituted => ContractEvidenceProblemCauseV1::Substituted,
        K::IdempotencyConflict => ContractEvidenceProblemCauseV1::Conflict,
        K::LifecycleRefused => ContractEvidenceProblemCauseV1::Stale,
        K::UnsafeFilesystem | K::RecoveryAmbiguous => {
            ContractEvidenceProblemCauseV1::InternalInvariant
        }
        K::DurabilityUncertain => ContractEvidenceProblemCauseV1::DurabilityUncertain,
    })
}

fn verdict_problem(kind: ClaimVerdictEvaluationErrorKindV1) -> ProblemProjection {
    cause_problem(match kind {
        ClaimVerdictEvaluationErrorKindV1::RepositoryUnavailable => {
            ContractEvidenceProblemCauseV1::RepositoryUnavailable
        }
        ClaimVerdictEvaluationErrorKindV1::RepositoryCorrupt => {
            ContractEvidenceProblemCauseV1::Substituted
        }
        ClaimVerdictEvaluationErrorKindV1::InternalInvariant => {
            ContractEvidenceProblemCauseV1::InternalInvariant
        }
    })
}

fn gate_problem(kind: GateEvaluationErrorKindV1) -> ProblemProjection {
    cause_problem(match kind {
        GateEvaluationErrorKindV1::RepositoryUnavailable => {
            ContractEvidenceProblemCauseV1::RepositoryUnavailable
        }
        GateEvaluationErrorKindV1::RepositoryCorrupt => ContractEvidenceProblemCauseV1::Substituted,
        GateEvaluationErrorKindV1::InternalInvariant => {
            ContractEvidenceProblemCauseV1::InternalInvariant
        }
    })
}

fn cause_problem(cause: ContractEvidenceProblemCauseV1) -> ProblemProjection {
    use ContractEvidenceProblemCauseV1 as C;
    let (status, stage, category, retry) = match cause {
        C::Malformed | C::OverLimit | C::Missing | C::Unknown | C::Duplicate => (
            TerminalStatus::Refused,
            super::IdempotencyStage::RequestValidation,
            ProblemCategory::Schema,
            RetryDirective::AfterRequestChange,
        ),
        C::Stale | C::IneligibleCandidate | C::InsufficientResolution => (
            TerminalStatus::Refused,
            super::IdempotencyStage::PreconditionValidation,
            ProblemCategory::Precondition,
            RetryDirective::AfterAuthorityChange,
        ),
        C::Substituted => (
            TerminalStatus::Refused,
            super::IdempotencyStage::SafetyValidation,
            ProblemCategory::Safety,
            RetryDirective::Never,
        ),
        C::Conflict => (
            TerminalStatus::Refused,
            super::IdempotencyStage::IdempotencyValidation,
            ProblemCategory::Idempotency,
            RetryDirective::AfterRequestChange,
        ),
        C::RepositoryUnavailable | C::CandidateSourceUnavailable => (
            TerminalStatus::Error,
            super::IdempotencyStage::ExecutionStart,
            ProblemCategory::Implementation,
            RetryDirective::Transient,
        ),
        C::DurabilityUncertain => (
            TerminalStatus::Error,
            super::IdempotencyStage::ExecutionStart,
            ProblemCategory::Safety,
            RetryDirective::Never,
        ),
        C::InternalInvariant => (
            TerminalStatus::Error,
            super::IdempotencyStage::ExecutionStart,
            ProblemCategory::Implementation,
            RetryDirective::Never,
        ),
    };
    ProblemProjection {
        cause,
        status,
        stage,
        category,
        retry,
    }
}

fn problem_response(
    request: &OperationRequestV1,
    definition: &OperationDefinition,
    projection: ProblemProjection,
    mutation: bool,
) -> Result<OperationResponseV1, ContractError> {
    let limits = ContractLimits::packet0();
    let details = ContractEvidenceOperationDetailsDtoV1 {
        cause: projection.cause,
        related_refs: BoundedVec::new(Vec::new())?,
        diagnostics: BoundedVec::new(Vec::new())?,
    };
    let code = format!("contract_evidence.{}", cause_name(projection.cause));
    let correlation_ref = if projection.status == TerminalStatus::Error {
        Some(CorrelationRef::new(BoundedString::new(format!(
            "corr_{}",
            request
                .request_fingerprint()
                .as_str()
                .strip_prefix("sha256:")
                .ok_or(ContractError::InvalidInvariant)?
        ))?)?)
    } else {
        None
    };
    let problem = Problem::new(
        BoundedString::new(code.clone())?,
        BoundedString::new(code)?,
        projection.category,
        definition.binding(),
        (projection.status == TerminalStatus::Refused).then(|| definition.binding()),
        schema_binding(CONTRACT_EVIDENCE_DETAILS_SCHEMA_REF)?,
        super::artifact_operation::to_contract_value(&details)?,
        BoundedVec::new(Vec::new())?,
        projection.retry,
        None,
        correlation_ref,
        &limits,
    )?;
    let binding = ProblemBinding::new(
        problem.problem_id().clone(),
        problem.problem_fingerprint().clone(),
    )?;
    let terminal = match projection.status {
        TerminalStatus::Refused => TerminalFields::new(
            TerminalStatus::Refused,
            None,
            BoundedVec::new(Vec::new())?,
            BoundedVec::new(vec![problem])?,
            BoundedVec::new(Vec::new())?,
        )?,
        TerminalStatus::Error => TerminalFields::new(
            TerminalStatus::Error,
            None,
            BoundedVec::new(Vec::new())?,
            BoundedVec::new(Vec::new())?,
            BoundedVec::new(vec![problem])?,
        )?,
        _ => return Err(ContractError::InvalidInvariant),
    };
    OperationResponseV1::new(
        schema_binding("handbook.operation-response@1.0.0")?,
        request.request_id().cloned(),
        request.request_fingerprint().clone(),
        definition,
        terminal,
        if mutation {
            super::IdempotencyState::not_established(projection.stage, binding)?
        } else {
            super::IdempotencyState::not_applicable()?
        },
        BoundedVec::new(Vec::new())?,
        BoundedVec::new(Vec::new())?,
        BoundedVec::new(Vec::new())?,
        &ContractEvidenceOperationSchemaValidator,
        &limits,
    )
}

fn cause_name(cause: ContractEvidenceProblemCauseV1) -> &'static str {
    use ContractEvidenceProblemCauseV1 as C;
    match cause {
        C::Malformed => "malformed",
        C::OverLimit => "over_limit",
        C::Missing => "missing",
        C::Unknown => "unknown",
        C::Stale => "stale",
        C::Substituted => "substituted",
        C::Duplicate => "duplicate",
        C::Conflict => "conflict",
        C::IneligibleCandidate => "ineligible_candidate",
        C::InsufficientResolution => "insufficient_resolution",
        C::RepositoryUnavailable => "repository_unavailable",
        C::CandidateSourceUnavailable => "candidate_source_unavailable",
        C::DurabilityUncertain => "durability_uncertain",
        C::InternalInvariant => "internal_invariant",
    }
}

pub fn contract_evidence_operation_definitions() -> Result<Vec<OperationDefinition>, ContractError>
{
    CONTRACT_EVIDENCE_OPERATION_REFS
        .into_iter()
        .map(contract_evidence_operation_definition)
        .collect()
}

pub fn contract_evidence_operation_definition(
    operation_ref: &str,
) -> Result<OperationDefinition, ContractError> {
    if !CONTRACT_EVIDENCE_OPERATION_REFS.contains(&operation_ref) {
        return Err(ContractError::InvalidInvariant);
    }
    let owner = handbook_contracts::operation_admission::operation_owner_version_evidence();
    let request_schema = schema_binding(&request_schema_ref(operation_ref))?;
    let result_schema = schema_binding(&result_schema_ref(operation_ref))?;
    let class = if operation_ref == CONTRACT_EVIDENCE_APPEND_REF {
        OperationClass::AppendOnly
    } else {
        OperationClass::ReadOnly
    };
    let idempotency = if operation_ref == CONTRACT_EVIDENCE_APPEND_REF {
        OperationIdempotency::key_required(JsonPointer::parse("/body/idempotency_key")?)?
    } else {
        OperationIdempotency::safe()
    };
    let write_set = if operation_ref == CONTRACT_EVIDENCE_APPEND_REF {
        vec![DeclaredWrite::new(
            BoundedString::new("contract_evidence_record".to_owned())?,
            BoundedString::new("contract_evidence_admission".to_owned())?,
            BoundedString::new("contract_evidence_append".to_owned())?,
            result_schema.clone(),
        )?]
    } else {
        Vec::new()
    };
    OperationDefinition::new(
        schema_binding("handbook.operation-definition@1.0.0")?,
        ExactRef::parse(operation_ref)?,
        OperationOwnerBinding::new(
            BoundedString::new(owner.owner_crate.to_owned())?,
            SemVer::parse(owner.owner_version)?,
            ExactBinding::new(
                ExactRef::parse(owner.owner_source_ref)?,
                super::sha256_fingerprint(owner.source_bytes),
            ),
        )?,
        request_schema.clone(),
        result_schema,
        OperationOutcomeSchemas::new(
            schema_binding("handbook.operation-blocker@1.0.0")?,
            schema_binding("handbook.operation-refusal@1.0.0")?,
            schema_binding("handbook.operation-error@1.0.0")?,
        )?,
        BoundedVec::new(Vec::new())?,
        class,
        idempotency,
        BoundedVec::new(write_set)?,
        BoundedVec::new(vec![
            OperationTransport::CliJson,
            OperationTransport::RustSdk,
            OperationTransport::Tauri,
        ])?,
        None,
        &ContractLimits::packet0(),
    )
}

pub(crate) fn current_contract_evidence_capabilities() -> Result<Vec<CapabilityEntry>, ContractError>
{
    contract_evidence_operation_definitions()?
        .into_iter()
        .map(|definition| {
            let owner = match super::admit_operation_definition(&definition)? {
                OperationAdmissionResult::Admitted(owner) => owner,
                OperationAdmissionResult::Omitted(_) => {
                    return Err(ContractError::InvalidInvariant)
                }
            };
            CapabilityEntry::new(
                &definition,
                &ContractEvidenceTransportEvidence { owner },
                &ContractLimits::packet0(),
            )
        })
        .collect()
}

struct ContractEvidenceTransportEvidence {
    owner: OwnerAdmissionToken,
}

impl CurrentTransportEvidenceSealed for ContractEvidenceTransportEvidence {}

impl CurrentTransportEvidence for ContractEvidenceTransportEvidence {
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

fn request_schema_ref(operation_ref: &str) -> String {
    format!(
        "handbook.operation.{}-request@1.0.0",
        operation_ref
            .strip_suffix("@1.0.0")
            .unwrap_or(operation_ref)
            .replace('.', "-")
    )
}

fn result_schema_ref(operation_ref: &str) -> String {
    format!(
        "handbook.operation.{}-result@1.0.0",
        operation_ref
            .strip_suffix("@1.0.0")
            .unwrap_or(operation_ref)
            .replace('.', "-")
    )
}

fn schema_binding(reference: &str) -> Result<ExactBinding, ContractError> {
    super::production_witness::current_schema_binding(reference)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contract_membrane::IdempotencyStateRef;

    fn fingerprint(byte: char) -> Fingerprint {
        Fingerprint::parse(&format!("sha256:{}", byte.to_string().repeat(64))).unwrap()
    }

    #[test]
    fn append_replay_preserves_result_and_receipt_identity() {
        let candidate =
            ContractRecordBindingDtoV1::new("producer/candidate@1.0.0", fingerprint('a')).unwrap();
        let request = contract_evidence_append_operation_request(
            Some("packet6-append-replay"),
            "packet6-secret-key",
            ContractEvidenceAppendRequestV1::new(candidate.clone()),
        )
        .unwrap();
        let definition =
            contract_evidence_operation_definition(CONTRACT_EVIDENCE_APPEND_REF).unwrap();
        let evidence_binding = ContractRecordBindingDtoV1::new(
            "handbook.contract.fixture@1.0.0#evidence/fixture",
            fingerprint('b'),
        )
        .unwrap();
        let result = ContractEvidenceAppendResultV1 {
            evidence: ContractEvidenceRecordDtoV1::new(
                evidence_binding.clone(),
                ContractValue::null(),
            ),
            receipt: ContractEvidenceAppendReceiptDtoV1 {
                sequence: SafeU64::new(1).unwrap(),
                request_fingerprint: fingerprint('c'),
                candidate,
                evidence: evidence_binding,
                receipt_fingerprint: fingerprint('d'),
            },
        };
        let scoped = request.idempotency_key_fingerprint().unwrap().clone();
        let committed =
            mutation_success_response(&request, &definition, scoped.clone(), false, &result)
                .unwrap();
        let replayed =
            mutation_success_response(&request, &definition, scoped, true, &result).unwrap();

        assert_eq!(
            committed.result_fingerprint(),
            replayed.result_fingerprint()
        );
        assert_eq!(committed.receipts(), replayed.receipts());
        assert!(matches!(
            committed.idempotency().as_ref(),
            IdempotencyStateRef::Established {
                replayed: false,
                original_result_fingerprint: None,
                ..
            }
        ));
        assert!(matches!(
            replayed.idempotency().as_ref(),
            IdempotencyStateRef::Established {
                replayed: true,
                original_result_fingerprint: Some(original),
                ..
            } if original == committed.result_fingerprint()
        ));
        let replayed_bytes = canonical_json_bytes(&replayed, &ContractLimits::packet0()).unwrap();
        assert!(!String::from_utf8_lossy(&replayed_bytes).contains("packet6-secret-key"));
        assert_eq!(
            OperationResponseV1::parse_and_admit(
                &replayed_bytes,
                &definition,
                &ContractEvidenceOperationSchemaValidator,
                &ContractLimits::packet0(),
            )
            .unwrap(),
            replayed
        );
    }

    #[test]
    fn owner_and_evaluation_failures_map_to_exhaustive_typed_terminal_causes() {
        use ClaimVerdictEvaluationErrorKindV1 as Verdict;
        use ContractEvidenceProblemCauseV1 as Cause;
        use ContractOwnerRepositoryErrorKindV1 as Owner;
        use GateEvaluationErrorKindV1 as Gate;

        for (kind, cause, status) in [
            (
                Owner::InvalidRequest,
                Cause::Malformed,
                TerminalStatus::Refused,
            ),
            (
                Owner::EvidenceCandidateMissing,
                Cause::Missing,
                TerminalStatus::Refused,
            ),
            (
                Owner::EvidenceCandidateIneligible,
                Cause::IneligibleCandidate,
                TerminalStatus::Refused,
            ),
            (
                Owner::EvidenceResolutionInsufficient,
                Cause::InsufficientResolution,
                TerminalStatus::Refused,
            ),
            (
                Owner::EvidenceSourceRefused,
                Cause::Stale,
                TerminalStatus::Refused,
            ),
            (
                Owner::EvidenceSourceUnavailable,
                Cause::CandidateSourceUnavailable,
                TerminalStatus::Error,
            ),
            (
                Owner::UnsafeFilesystem,
                Cause::InternalInvariant,
                TerminalStatus::Error,
            ),
            (
                Owner::BoundExceeded,
                Cause::OverLimit,
                TerminalStatus::Refused,
            ),
            (
                Owner::DurabilityUncertain,
                Cause::DurabilityUncertain,
                TerminalStatus::Error,
            ),
            (
                Owner::RecoveryAmbiguous,
                Cause::InternalInvariant,
                TerminalStatus::Error,
            ),
            (
                Owner::IdempotencyConflict,
                Cause::Conflict,
                TerminalStatus::Refused,
            ),
            (
                Owner::RecordSubstituted,
                Cause::Substituted,
                TerminalStatus::Refused,
            ),
            (
                Owner::LifecycleRefused,
                Cause::Stale,
                TerminalStatus::Refused,
            ),
        ] {
            let projection = owner_problem(kind);
            assert_eq!(projection.cause, cause);
            assert_eq!(projection.status, status);
        }

        for (kind, cause, status) in [
            (
                Verdict::RepositoryUnavailable,
                Cause::RepositoryUnavailable,
                TerminalStatus::Error,
            ),
            (
                Verdict::RepositoryCorrupt,
                Cause::Substituted,
                TerminalStatus::Refused,
            ),
            (
                Verdict::InternalInvariant,
                Cause::InternalInvariant,
                TerminalStatus::Error,
            ),
        ] {
            let projection = verdict_problem(kind);
            assert_eq!(projection.cause, cause);
            assert_eq!(projection.status, status);
        }

        for (kind, cause, status) in [
            (
                Gate::RepositoryUnavailable,
                Cause::RepositoryUnavailable,
                TerminalStatus::Error,
            ),
            (
                Gate::RepositoryCorrupt,
                Cause::Substituted,
                TerminalStatus::Refused,
            ),
            (
                Gate::InternalInvariant,
                Cause::InternalInvariant,
                TerminalStatus::Error,
            ),
        ] {
            let projection = gate_problem(kind);
            assert_eq!(projection.cause, cause);
            assert_eq!(projection.status, status);
        }
    }
}
