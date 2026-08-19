use super::{
    canonical_json_bytes, parse_canonical_json, BoundedString, BoundedVec, CapabilityEntry,
    ContractError, ContractLimits, ContractValue, CurrentTransportEvidence,
    CurrentTransportEvidenceSealed, DeclaredWrite, ExactBinding, ExactRef, Fingerprint,
    IdempotencyKeyConsumer, IdempotencyKeyConsumerSealed, IdempotencyStage, IdempotencyState,
    JsonPointer, OperationAdmissionResult, OperationClass, OperationDefinition,
    OperationIdempotency, OperationOutcomeSchemas, OperationOwnerBinding, OperationRequestV1,
    OperationResponseV1, OperationSchemaValidator, OperationTransport, OwnerAdmissionToken,
    Problem, ProblemBinding, ProblemCategory, RequestId, RetryDirective, SafeU64, SemVer,
    TerminalFields, TerminalStatus, UtcTimestamp, WriteReceipt,
};
use handbook_contracts::{
    ContractApprovalClassV1 as OwnerApprovalClass, ContractDefinitionV1 as OwnerDefinition,
    ContractLifecycleTransitionInputV1 as OwnerTransition, ContractOwnerRepositoryErrorKindV1,
    ContractOwnerRepositoryV1, DefinitionListRequestV1, DefinitionReadOutcomeV1,
    ExactBindingV1 as OwnerBinding, ExactRefV1 as OwnerRef, FingerprintV1 as OwnerFingerprint,
    LifecycleEvidenceKindV1 as OwnerEvidenceKind, LifecycleReasonTextV1,
    LifecycleReasonV1 as OwnerReason, LifecycleStateV1 as OwnerState,
    LifecycleSupportingEvidenceV1 as OwnerEvidence, OwnerMutationDispositionV1,
};
use handbook_engine::{
    CredentialAuthorityAdmissionErrorKindV1, CredentialAuthorityAdmissionServiceV1,
    NativeAuthenticatorPortErrorV1, NativeAuthenticatorPortV1,
};
use serde::{Deserialize, Serialize};
use std::path::Path;

pub const CONTRACT_DEFINITION_LIST_REF: &str = "contract.definition.list@1.0.0";
pub const CONTRACT_DEFINITION_READ_REF: &str = "contract.definition.read@1.0.0";
pub const CONTRACT_DEFINITION_APPEND_REF: &str = "contract.definition.append@1.0.0";
pub const CONTRACT_LIFECYCLE_TRANSITION_REF: &str = "contract.lifecycle.transition@1.0.0";
pub const CONTRACT_OPERATION_REFS: [&str; 4] = [
    CONTRACT_DEFINITION_LIST_REF,
    CONTRACT_DEFINITION_READ_REF,
    CONTRACT_DEFINITION_APPEND_REF,
    CONTRACT_LIFECYCLE_TRANSITION_REF,
];

const DETAILS_SCHEMA_REF: &str = "handbook.contract-operation-details@1.0.0";
const RETENTION_UNTIL_UTC: &str = "9999-12-31T23:59:59Z";

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "contract-schema-generation", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct ContractDefinitionDtoV1 {
    contract: ExactBinding,
    document: ContractValue,
}

impl ContractDefinitionDtoV1 {
    pub fn new(contract: ExactBinding, document: ContractValue) -> Result<Self, ContractError> {
        let value = Self { contract, document };
        value.owner_definition()?;
        Ok(value)
    }

    pub fn contract(&self) -> &ExactBinding {
        &self.contract
    }

    pub fn document(&self) -> &ContractValue {
        &self.document
    }

    fn owner_definition(&self) -> Result<OwnerDefinition, ContractError> {
        let definition: OwnerDefinition = owner_from_contract_value(&self.document)?;
        if definition.contract_ref().as_str() != self.contract.reference().as_str()
            || definition.definition_fingerprint().as_str() != self.contract.fingerprint().as_str()
        {
            return Err(ContractError::FingerprintMismatch);
        }
        Ok(definition)
    }
}

macro_rules! semantic_document_dto {
    ($name:ident) => {
        #[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
        #[cfg_attr(feature = "contract-schema-generation", derive(schemars::JsonSchema))]
        #[serde(deny_unknown_fields)]
        pub struct $name {
            pub binding: ExactBinding,
            pub document: ContractValue,
        }
    };
}

semantic_document_dto!(ContractClaimDtoV1);
semantic_document_dto!(ContractLifecyclePolicyDtoV1);
semantic_document_dto!(ContractLifecycleTransitionDtoV1);
semantic_document_dto!(ContractEvaluationLifecycleBasisDtoV1);
semantic_document_dto!(ContractApplicabilityOutcomeDtoV1);

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "contract-schema-generation", derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum ContractLifecycleStateDtoV1 {
    Draft,
    ReviewReady,
    Locked,
    Active,
    Deprecated,
    Closed,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "contract-schema-generation", derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum ContractApprovalClassDtoV1 {
    DefinitionAuthor,
    IndependentLock,
    Activation,
    Lifecycle,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "contract-schema-generation", derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum ContractLifecycleEvidenceKindDtoV1 {
    DefinitionValidation,
    IndependentReview,
    Rejection,
    ApplicabilityClosure,
    MatcherClosure,
    PolicyClosure,
    CurrentnessClosure,
    Replacement,
    NoReplacementRationale,
    RetentionSafety,
    ReferenceSafety,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "contract-schema-generation", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct ContractLifecycleEvidenceDtoV1 {
    kind: ContractLifecycleEvidenceKindDtoV1,
    binding: ExactBinding,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "contract-schema-generation", derive(schemars::JsonSchema))]
#[serde(
    tag = "kind",
    content = "detail",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum ContractLifecycleReasonDtoV1 {
    ReadyForReview,
    Withdrawn,
    IndependentlyLocked,
    Rejected,
    Activated,
    DeprecatedWithReplacement(ExactBinding),
    DeprecatedWithoutReplacement(BoundedString<4096>),
    ClosedAfterRetention,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "contract-schema-generation", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct ContractLifecycleTransitionInputDtoV1 {
    expected_lifecycle_fingerprint: Fingerprint,
    from_state: ContractLifecycleStateDtoV1,
    to_state: ContractLifecycleStateDtoV1,
    approval_class: ContractApprovalClassDtoV1,
    supporting_evidence: BoundedVec<ContractLifecycleEvidenceDtoV1, 32>,
    reason: ContractLifecycleReasonDtoV1,
}

impl ContractLifecycleTransitionInputDtoV1 {
    fn into_owner(self) -> Result<OwnerTransition, ContractError> {
        Ok(OwnerTransition {
            expected_lifecycle_fingerprint: owner_fingerprint(
                &self.expected_lifecycle_fingerprint,
            )?,
            from_state: owner_state(self.from_state),
            to_state: owner_state(self.to_state),
            approval_class: owner_approval(self.approval_class),
            supporting_evidence: self
                .supporting_evidence
                .as_slice()
                .iter()
                .map(|evidence| {
                    Ok(OwnerEvidence::new(
                        owner_evidence_kind(evidence.kind),
                        owner_binding(&evidence.binding)?,
                    ))
                })
                .collect::<Result<Vec<_>, ContractError>>()?,
            reason: owner_reason(self.reason)?,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "contract-schema-generation", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct ContractDefinitionListRequestV1 {
    limit: SafeU64,
    root: Option<Fingerprint>,
    cursor: Option<BoundedString<4096>>,
}

impl ContractDefinitionListRequestV1 {
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
        cursor: String,
    ) -> Result<Self, ContractError> {
        let value = Self {
            limit: SafeU64::new(limit)?,
            root: Some(root),
            cursor: Some(BoundedString::new(cursor)?),
        };
        value.validate()?;
        Ok(value)
    }

    fn validate(&self) -> Result<(), ContractError> {
        if !(1..=100).contains(&self.limit.get()) || self.root.is_some() != self.cursor.is_some() {
            Err(ContractError::InvalidInvariant)
        } else {
            Ok(())
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "contract-schema-generation", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct ContractDefinitionReadRequestV1 {
    contract: ExactBinding,
}

impl ContractDefinitionReadRequestV1 {
    pub fn new(contract: ExactBinding) -> Self {
        Self { contract }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "contract-schema-generation", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct ContractDefinitionAppendRequestV1 {
    definition: ContractDefinitionDtoV1,
}

impl ContractDefinitionAppendRequestV1 {
    pub fn new(definition: ContractDefinitionDtoV1) -> Self {
        Self { definition }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "contract-schema-generation", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct ContractLifecycleTransitionRequestV1 {
    contract: ExactBinding,
    transition: ContractLifecycleTransitionInputDtoV1,
}

impl ContractLifecycleTransitionRequestV1 {
    pub fn new(contract: ExactBinding, transition: ContractLifecycleTransitionInputDtoV1) -> Self {
        Self {
            contract,
            transition,
        }
    }
}

#[derive(Clone, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "contract-schema-generation", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub(super) struct AppendWireV1 {
    definition: ContractDefinitionDtoV1,
    idempotency_key: String,
}

#[derive(Clone, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "contract-schema-generation", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub(super) struct TransitionWireV1 {
    contract: ExactBinding,
    transition: ContractLifecycleTransitionInputDtoV1,
    idempotency_key: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "contract-schema-generation", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct ContractDefinitionListEntryV1 {
    contract: ExactBinding,
    record_fingerprint: Fingerprint,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "contract-schema-generation", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct ContractLifecycleHeadDtoV1 {
    state: ContractLifecycleStateDtoV1,
    transition_ref: Option<BoundedString<1024>>,
    lifecycle_fingerprint: Fingerprint,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "contract-schema-generation", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct ContractDefinitionListResultV1 {
    root: Fingerprint,
    entries: BoundedVec<ContractDefinitionListEntryV1, 100>,
    next_cursor: Option<BoundedString<4096>>,
    retention: ContractListRetentionV1,
    current: bool,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "contract-schema-generation", derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum ContractListRetentionV1 {
    CurrentOnly,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "contract-schema-generation", derive(schemars::JsonSchema))]
#[serde(tag = "outcome", rename_all = "snake_case", deny_unknown_fields)]
pub enum ContractDefinitionReadResultV1 {
    Found {
        definition: ContractDefinitionDtoV1,
        record_fingerprint: Fingerprint,
        lifecycle_head: ContractLifecycleHeadDtoV1,
    },
    Absent {
        contract: ExactBinding,
    },
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "contract-schema-generation", derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum ContractMutationDispositionV1 {
    Committed,
    Replayed,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "contract-schema-generation", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct ContractDefinitionAppendResultV1 {
    owner_request_fingerprint: Fingerprint,
    contract: ExactBinding,
    record_fingerprint: Fingerprint,
    lifecycle_head: ContractLifecycleHeadDtoV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "contract-schema-generation", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct ContractLifecycleTransitionResultV1 {
    owner_request_fingerprint: Fingerprint,
    transition_ref: BoundedString<1024>,
    transition_fingerprint: Fingerprint,
    lifecycle_head: ContractLifecycleHeadDtoV1,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "contract-schema-generation", derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum ContractProblemCauseV1 {
    InvalidRequest,
    BoundExceeded,
    RecordSubstituted,
    IdempotencyConflict,
    LifecycleRefused,
    UnsafeFilesystem,
    DurabilityUncertain,
    RecoveryAmbiguous,
    AuthorityAbsent,
    AuthorityAmbiguous,
    AuthorityStale,
    AuthenticationRefused,
    AuthenticatorUnavailable,
    AuthorityDurabilityViolation,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "contract-schema-generation", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct ContractOperationProblemDetailsV1 {
    cause: ContractProblemCauseV1,
}

pub struct ContractOperationSchemaValidator;

impl OperationSchemaValidator for ContractOperationSchemaValidator {
    fn validate_instance(
        &self,
        schema: &ExactBinding,
        instance: &ContractValue,
    ) -> Result<(), ContractError> {
        let bytes = canonical_json_bytes(instance, &ContractLimits::packet0())?;
        match schema.reference().as_str() {
            "handbook.operation.contract-definition-list-request@1.0.0" => {
                let value = parse_canonical_json::<ContractDefinitionListRequestV1>(
                    &bytes,
                    &ContractLimits::packet0(),
                )?;
                value.validate()
            }
            "handbook.operation.contract-definition-list-result@1.0.0" => {
                parse::<ContractDefinitionListResultV1>(&bytes)
            }
            "handbook.operation.contract-definition-read-request@1.0.0" => {
                parse::<ContractDefinitionReadRequestV1>(&bytes)
            }
            "handbook.operation.contract-definition-read-result@1.0.0" => {
                parse::<ContractDefinitionReadResultV1>(&bytes)
            }
            "handbook.operation.contract-definition-append-request@1.0.0" => {
                let value =
                    parse_canonical_json::<AppendWireV1>(&bytes, &ContractLimits::packet0())?;
                value.definition.owner_definition().map(|_| ())
            }
            "handbook.operation.contract-definition-append-result@1.0.0" => {
                parse::<ContractDefinitionAppendResultV1>(&bytes)
            }
            "handbook.operation.contract-lifecycle-transition-request@1.0.0" => {
                parse::<TransitionWireV1>(&bytes)
            }
            "handbook.operation.contract-lifecycle-transition-result@1.0.0" => {
                parse::<ContractLifecycleTransitionResultV1>(&bytes)
            }
            DETAILS_SCHEMA_REF => parse::<ContractOperationProblemDetailsV1>(&bytes),
            _ => Err(ContractError::InvalidInvariant),
        }
    }
}

fn parse<T: for<'de> Deserialize<'de> + Serialize>(bytes: &[u8]) -> Result<(), ContractError> {
    parse_canonical_json::<T>(bytes, &ContractLimits::packet0()).map(|_| ())
}

pub fn contract_operation_definitions() -> Result<Vec<OperationDefinition>, ContractError> {
    CONTRACT_OPERATION_REFS
        .into_iter()
        .map(contract_operation_definition)
        .collect()
}

pub fn contract_operation_definition(
    operation_ref: &str,
) -> Result<OperationDefinition, ContractError> {
    if !CONTRACT_OPERATION_REFS.contains(&operation_ref) {
        return Err(ContractError::InvalidInvariant);
    }
    let limits = ContractLimits::packet0();
    let owner = handbook_contracts::operation_admission::operation_owner_version_evidence();
    let request_schema = schema_binding(&request_schema_ref(operation_ref))?;
    let result_schema = schema_binding(&result_schema_ref(operation_ref))?;
    let class = operation_class(operation_ref);
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
        result_schema.clone(),
        OperationOutcomeSchemas::new(
            schema_binding("handbook.operation-blocker@1.0.0")?,
            schema_binding("handbook.operation-refusal@1.0.0")?,
            schema_binding("handbook.operation-error@1.0.0")?,
        )?,
        BoundedVec::new(Vec::new())?,
        class,
        operation_idempotency(operation_ref)?,
        BoundedVec::new(operation_write_set(operation_ref, result_schema)?)?,
        BoundedVec::new(vec![
            OperationTransport::CliJson,
            OperationTransport::RustSdk,
            OperationTransport::Tauri,
        ])?,
        None,
        &limits,
    )
}

pub(crate) fn current_contract_capabilities() -> Result<Vec<CapabilityEntry>, ContractError> {
    contract_operation_definitions()?
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
                &ContractTransportEvidence { owner },
                &ContractLimits::packet0(),
            )
        })
        .collect()
}

struct ContractTransportEvidence {
    owner: OwnerAdmissionToken,
}
impl CurrentTransportEvidenceSealed for ContractTransportEvidence {}
impl CurrentTransportEvidence for ContractTransportEvidence {
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

pub fn contract_definition_list_operation_request(
    request_id: Option<&str>,
    body: ContractDefinitionListRequestV1,
) -> Result<OperationRequestV1, ContractError> {
    operation_request(
        CONTRACT_DEFINITION_LIST_REF,
        request_id,
        super::artifact_operation::to_contract_value(&body)?,
    )
}

pub fn contract_definition_read_operation_request(
    request_id: Option<&str>,
    body: ContractDefinitionReadRequestV1,
) -> Result<OperationRequestV1, ContractError> {
    operation_request(
        CONTRACT_DEFINITION_READ_REF,
        request_id,
        super::artifact_operation::to_contract_value(&body)?,
    )
}

pub fn contract_definition_append_operation_request(
    request_id: Option<&str>,
    idempotency_key: &str,
    body: ContractDefinitionAppendRequestV1,
) -> Result<OperationRequestV1, ContractError> {
    operation_request(
        CONTRACT_DEFINITION_APPEND_REF,
        request_id,
        super::artifact_operation::to_contract_value(&AppendWireV1 {
            definition: body.definition,
            idempotency_key: idempotency_key.to_owned(),
        })?,
    )
}

pub fn contract_lifecycle_transition_operation_request(
    request_id: Option<&str>,
    idempotency_key: &str,
    body: ContractLifecycleTransitionRequestV1,
) -> Result<OperationRequestV1, ContractError> {
    operation_request(
        CONTRACT_LIFECYCLE_TRANSITION_REF,
        request_id,
        super::artifact_operation::to_contract_value(&TransitionWireV1 {
            contract: body.contract,
            transition: body.transition,
            idempotency_key: idempotency_key.to_owned(),
        })?,
    )
}

fn operation_request(
    operation_ref: &str,
    request_id: Option<&str>,
    body: ContractValue,
) -> Result<OperationRequestV1, ContractError> {
    let definition = contract_operation_definition(operation_ref)?;
    OperationRequestV1::new(
        schema_binding("handbook.operation-request@1.0.0")?,
        request_id.map(RequestId::parse).transpose()?,
        &definition,
        body,
        &ContractOperationSchemaValidator,
        &ContractLimits::packet0(),
    )
}

pub(crate) fn execute_contract_operation(
    repo_root: &Path,
    request: &OperationRequestV1,
) -> Result<OperationResponseV1, ContractError> {
    execute_contract_operation_with_authenticator(repo_root, request, &mut UnavailableAuthenticator)
}

pub fn execute_contract_operation_with_authenticator<P: NativeAuthenticatorPortV1>(
    repo_root: &Path,
    request: &OperationRequestV1,
    port: &mut P,
) -> Result<OperationResponseV1, ContractError> {
    let definition = contract_operation_definition(request.operation().reference().as_str())?;
    if request.operation() != &definition.binding()
        || request.body_schema() != definition.request_schema()
    {
        return Err(ContractError::InvalidInvariant);
    }
    match request.operation().reference().as_str() {
        CONTRACT_DEFINITION_LIST_REF => execute_list(repo_root, request, &definition),
        CONTRACT_DEFINITION_READ_REF => execute_read(repo_root, request, &definition),
        CONTRACT_DEFINITION_APPEND_REF => execute_append(repo_root, request, &definition, port),
        CONTRACT_LIFECYCLE_TRANSITION_REF => {
            execute_transition(repo_root, request, &definition, port)
        }
        _ => Err(ContractError::InvalidInvariant),
    }
}

fn execute_list(
    repo_root: &Path,
    request: &OperationRequestV1,
    definition: &OperationDefinition,
) -> Result<OperationResponseV1, ContractError> {
    let body: ContractDefinitionListRequestV1 = from_body(request)?;
    body.validate()?;
    let owner_request = match (&body.root, &body.cursor) {
        (None, None) => DefinitionListRequestV1::first(body.limit.get() as usize),
        (Some(root), Some(cursor)) => DefinitionListRequestV1::continue_from(
            body.limit.get() as usize,
            owner_fingerprint(root)?,
            cursor.as_str(),
        ),
        _ => return Err(ContractError::InvalidInvariant),
    };
    let result = owner_request.and_then(|owner_request| {
        ContractOwnerRepositoryV1::open(repo_root)?.list_definitions(owner_request)
    });
    match result {
        Ok(page) => {
            let entries = page
                .entries()
                .iter()
                .map(|entry| {
                    Ok(ContractDefinitionListEntryV1 {
                        contract: ExactBinding::new(
                            ExactRef::parse(entry.contract_ref().as_str())?,
                            Fingerprint::parse(entry.definition_fingerprint().as_str())?,
                        ),
                        record_fingerprint: Fingerprint::parse(
                            entry.record_fingerprint().as_str(),
                        )?,
                    })
                })
                .collect::<Result<Vec<_>, ContractError>>()?;
            success_response(
                request,
                definition,
                super::artifact_operation::to_contract_value(&ContractDefinitionListResultV1 {
                    root: Fingerprint::parse(page.root().as_str())?,
                    entries: BoundedVec::new(entries)?,
                    next_cursor: page
                        .next_cursor()
                        .map(|value| BoundedString::new(value.to_owned()))
                        .transpose()?,
                    retention: ContractListRetentionV1::CurrentOnly,
                    current: page.is_current(),
                })?,
                IdempotencyState::not_applicable()?,
                BoundedVec::new(Vec::new())?,
            )
        }
        Err(error) => {
            owner_problem_response(request, definition, owner_problem(error.kind()), false)
        }
    }
}

fn execute_read(
    repo_root: &Path,
    request: &OperationRequestV1,
    definition: &OperationDefinition,
) -> Result<OperationResponseV1, ContractError> {
    let body: ContractDefinitionReadRequestV1 = from_body(request)?;
    let owner_ref = owner_ref(body.contract.reference())?;
    let owner_fp = owner_fingerprint(body.contract.fingerprint())?;
    let result = ContractOwnerRepositoryV1::open(repo_root)
        .and_then(|owner| owner.read_definition(&owner_ref, &owner_fp));
    match result {
        Ok(DefinitionReadOutcomeV1::Found(record)) => {
            let owner_definition = record.definition().definition();
            let document = owner_to_contract_value(owner_definition)?;
            let dto = ContractDefinitionDtoV1::new(body.contract.clone(), document)?;
            let data = ContractDefinitionReadResultV1::Found {
                definition: dto,
                record_fingerprint: Fingerprint::parse(record.record_fingerprint().as_str())?,
                lifecycle_head: lifecycle_head(record.lifecycle_head())?,
            };
            success_response(
                request,
                definition,
                super::artifact_operation::to_contract_value(&data)?,
                IdempotencyState::not_applicable()?,
                BoundedVec::new(Vec::new())?,
            )
        }
        Ok(DefinitionReadOutcomeV1::Absent) => success_response(
            request,
            definition,
            super::artifact_operation::to_contract_value(
                &ContractDefinitionReadResultV1::Absent {
                    contract: body.contract,
                },
            )?,
            IdempotencyState::not_applicable()?,
            BoundedVec::new(Vec::new())?,
        ),
        Err(error) => {
            owner_problem_response(request, definition, owner_problem(error.kind()), false)
        }
    }
}

fn execute_append<P: NativeAuthenticatorPortV1>(
    repo_root: &Path,
    request: &OperationRequestV1,
    definition: &OperationDefinition,
    port: &mut P,
) -> Result<OperationResponseV1, ContractError> {
    let body: ContractDefinitionAppendRequestV1 = from_body(request)?;
    let owner_definition = body.definition.owner_definition()?;
    let prepared = handbook_contracts::prepare_contract_definition_append(owner_definition)
        .map_err(|_| ContractError::InvalidInvariant)?;
    let mut bridge = AppendBridge {
        repo_root,
        port,
        prepared: Some(prepared),
        result: None,
    };
    let scoped = request
        .with_owner_idempotency_key(&mut bridge)
        .ok_or(ContractError::InvalidInvariant)?;
    match bridge.result.ok_or(ContractError::InvalidInvariant)? {
        Ok(execution) => {
            let owner_disposition = execution.disposition();
            let data = ContractDefinitionAppendResultV1 {
                owner_request_fingerprint: Fingerprint::parse(
                    execution.request_fingerprint().as_str(),
                )?,
                contract: ExactBinding::new(
                    ExactRef::parse(execution.contract_ref().as_str())?,
                    Fingerprint::parse(execution.definition_fingerprint().as_str())?,
                ),
                record_fingerprint: Fingerprint::parse(execution.record_fingerprint().as_str())?,
                lifecycle_head: lifecycle_head(execution.lifecycle_head())?,
            };
            mutation_success(
                request,
                definition,
                scoped,
                disposition(owner_disposition),
                super::artifact_operation::to_contract_value(&data)?,
                vec![
                    (
                        "contract_definition",
                        "handbook.contract-definition-record@1.0.0",
                        data.record_fingerprint.clone(),
                    ),
                    (
                        "contract_lifecycle_head",
                        "handbook.contract-lifecycle-head@1.0.0",
                        data.lifecycle_head.lifecycle_fingerprint.clone(),
                    ),
                ],
            )
        }
        Err(BridgeError::Owner(kind)) => {
            owner_problem_response(request, definition, owner_problem(kind), true)
        }
        Err(BridgeError::Authority(kind)) => {
            owner_problem_response(request, definition, authority_problem(kind), true)
        }
    }
}

fn execute_transition<P: NativeAuthenticatorPortV1>(
    repo_root: &Path,
    request: &OperationRequestV1,
    definition: &OperationDefinition,
    port: &mut P,
) -> Result<OperationResponseV1, ContractError> {
    let body: ContractLifecycleTransitionRequestV1 = from_body(request)?;
    let owner_ref = owner_ref(body.contract.reference())?;
    let owner_fp = owner_fingerprint(body.contract.fingerprint())?;
    let owner = match ContractOwnerRepositoryV1::open(repo_root) {
        Ok(owner) => owner,
        Err(error) => {
            return owner_problem_response(request, definition, owner_problem(error.kind()), true)
        }
    };
    let prepared = match owner.prepare_lifecycle_transition(
        &owner_ref,
        &owner_fp,
        body.transition.into_owner()?,
    ) {
        Ok(prepared) => prepared,
        Err(error) => {
            return owner_problem_response(request, definition, owner_problem(error.kind()), true)
        }
    };
    let mut bridge = TransitionBridge {
        repo_root,
        port,
        prepared: Some(prepared),
        result: None,
    };
    let scoped = request
        .with_owner_idempotency_key(&mut bridge)
        .ok_or(ContractError::InvalidInvariant)?;
    match bridge.result.ok_or(ContractError::InvalidInvariant)? {
        Ok(execution) => {
            let owner_disposition = execution.disposition();
            let data = ContractLifecycleTransitionResultV1 {
                owner_request_fingerprint: Fingerprint::parse(
                    execution.request_fingerprint().as_str(),
                )?,
                transition_ref: BoundedString::new(execution.transition_ref().to_owned())?,
                transition_fingerprint: Fingerprint::parse(
                    execution.transition_fingerprint().as_str(),
                )?,
                lifecycle_head: lifecycle_head(execution.lifecycle_head())?,
            };
            mutation_success(
                request,
                definition,
                scoped,
                disposition(owner_disposition),
                super::artifact_operation::to_contract_value(&data)?,
                vec![
                    (
                        "contract_lifecycle_head",
                        "handbook.contract-lifecycle-head@1.0.0",
                        data.lifecycle_head.lifecycle_fingerprint.clone(),
                    ),
                    (
                        "contract_lifecycle_transition",
                        "handbook.contract-lifecycle-transition-record@1.0.0",
                        data.transition_fingerprint.clone(),
                    ),
                ],
            )
        }
        Err(BridgeError::Owner(kind)) => {
            owner_problem_response(request, definition, owner_problem(kind), true)
        }
        Err(BridgeError::Authority(kind)) => {
            owner_problem_response(request, definition, authority_problem(kind), true)
        }
    }
}

struct AppendBridge<'a, P> {
    repo_root: &'a Path,
    port: &'a mut P,
    prepared: Option<handbook_contracts::PreparedContractDefinitionAppendV1>,
    result: Option<Result<handbook_contracts::DefinitionAppendExecutionV1, BridgeError>>,
}
impl<P: NativeAuthenticatorPortV1> IdempotencyKeyConsumerSealed for AppendBridge<'_, P> {
    fn consume_owner_idempotency_key(&mut self, _pointer: &str, bytes: &[u8]) -> Fingerprint {
        let key = std::str::from_utf8(bytes).expect("raw key admission validated UTF-8");
        let prepared = self.prepared.take().expect("one-shot key bridge");
        let authority_request = prepared.authority_request();
        let mut service =
            CredentialAuthorityAdmissionServiceV1::new(self.repo_root, BorrowedPort(self.port));
        self.result = Some(
            match service.admit(authority_request, |admission| {
                ContractOwnerRepositoryV1::open(self.repo_root)
                    .and_then(|owner| owner.append_definition(prepared, admission, key))
            }) {
                Ok(result) => result.map_err(|error| BridgeError::Owner(error.kind())),
                Err(error) => Err(BridgeError::Authority(error.kind())),
            },
        );
        super::sha256_fingerprint(bytes)
    }
}
impl<P: NativeAuthenticatorPortV1> IdempotencyKeyConsumer for AppendBridge<'_, P> {}

struct TransitionBridge<'a, P> {
    repo_root: &'a Path,
    port: &'a mut P,
    prepared: Option<handbook_contracts::PreparedOwnerLifecycleTransitionV1>,
    result: Option<Result<handbook_contracts::LifecycleTransitionExecutionV1, BridgeError>>,
}
impl<P: NativeAuthenticatorPortV1> IdempotencyKeyConsumerSealed for TransitionBridge<'_, P> {
    fn consume_owner_idempotency_key(&mut self, _pointer: &str, bytes: &[u8]) -> Fingerprint {
        let key = std::str::from_utf8(bytes).expect("raw key admission validated UTF-8");
        let prepared = self.prepared.take().expect("one-shot key bridge");
        let authority_request = prepared.authority_request();
        let mut service =
            CredentialAuthorityAdmissionServiceV1::new(self.repo_root, BorrowedPort(self.port));
        self.result = Some(
            match service.admit(authority_request, |admission| {
                ContractOwnerRepositoryV1::open(self.repo_root)
                    .and_then(|owner| owner.transition_lifecycle(prepared, admission, key))
            }) {
                Ok(result) => result.map_err(|error| BridgeError::Owner(error.kind())),
                Err(error) => Err(BridgeError::Authority(error.kind())),
            },
        );
        super::sha256_fingerprint(bytes)
    }
}
impl<P: NativeAuthenticatorPortV1> IdempotencyKeyConsumer for TransitionBridge<'_, P> {}

struct BorrowedPort<'a, P>(&'a mut P);
impl<P: NativeAuthenticatorPortV1> NativeAuthenticatorPortV1 for BorrowedPort<'_, P> {
    fn make_credential(
        &mut self,
        request: &[u8],
    ) -> Result<Vec<u8>, NativeAuthenticatorPortErrorV1> {
        self.0.make_credential(request)
    }
    fn get_assertion(&mut self, request: &[u8]) -> Result<Vec<u8>, NativeAuthenticatorPortErrorV1> {
        self.0.get_assertion(request)
    }
}
struct UnavailableAuthenticator;
impl NativeAuthenticatorPortV1 for UnavailableAuthenticator {
    fn make_credential(
        &mut self,
        _request: &[u8],
    ) -> Result<Vec<u8>, NativeAuthenticatorPortErrorV1> {
        Err(NativeAuthenticatorPortErrorV1::Unavailable)
    }
    fn get_assertion(
        &mut self,
        _request: &[u8],
    ) -> Result<Vec<u8>, NativeAuthenticatorPortErrorV1> {
        Err(NativeAuthenticatorPortErrorV1::Unavailable)
    }
}

#[derive(Clone, Copy)]
enum BridgeError {
    Owner(ContractOwnerRepositoryErrorKindV1),
    Authority(CredentialAuthorityAdmissionErrorKindV1),
}

#[derive(Clone, Copy)]
struct ProblemProjection {
    cause: ContractProblemCauseV1,
    status: TerminalStatus,
    stage: IdempotencyStage,
    category: ProblemCategory,
    retry: RetryDirective,
}

fn owner_problem(kind: ContractOwnerRepositoryErrorKindV1) -> ProblemProjection {
    use ContractOwnerRepositoryErrorKindV1 as K;
    let (cause, category, retry) = match kind {
        K::InvalidRequest => (
            ContractProblemCauseV1::InvalidRequest,
            ProblemCategory::Schema,
            RetryDirective::AfterRequestChange,
        ),
        K::BoundExceeded => (
            ContractProblemCauseV1::BoundExceeded,
            ProblemCategory::Schema,
            RetryDirective::AfterRequestChange,
        ),
        K::RecordSubstituted => (
            ContractProblemCauseV1::RecordSubstituted,
            ProblemCategory::Precondition,
            RetryDirective::AfterRequestChange,
        ),
        K::IdempotencyConflict => (
            ContractProblemCauseV1::IdempotencyConflict,
            ProblemCategory::Idempotency,
            RetryDirective::AfterRequestChange,
        ),
        K::LifecycleRefused => (
            ContractProblemCauseV1::LifecycleRefused,
            ProblemCategory::Precondition,
            RetryDirective::AfterAuthorityChange,
        ),
        K::UnsafeFilesystem => (
            ContractProblemCauseV1::UnsafeFilesystem,
            ProblemCategory::Safety,
            RetryDirective::Never,
        ),
        K::DurabilityUncertain => (
            ContractProblemCauseV1::DurabilityUncertain,
            ProblemCategory::Safety,
            RetryDirective::Never,
        ),
        K::RecoveryAmbiguous => (
            ContractProblemCauseV1::RecoveryAmbiguous,
            ProblemCategory::Safety,
            RetryDirective::Never,
        ),
    };
    ProblemProjection {
        cause,
        status: TerminalStatus::Refused,
        stage: IdempotencyStage::PreconditionValidation,
        category,
        retry,
    }
}

fn authority_problem(kind: CredentialAuthorityAdmissionErrorKindV1) -> ProblemProjection {
    use CredentialAuthorityAdmissionErrorKindV1 as K;
    let (cause, status, stage, category, retry) = match kind {
        K::InvalidRequest => (
            ContractProblemCauseV1::InvalidRequest,
            TerminalStatus::Refused,
            IdempotencyStage::AuthorityValidation,
            ProblemCategory::Authority,
            RetryDirective::AfterRequestChange,
        ),
        K::AuthorityAbsent => (
            ContractProblemCauseV1::AuthorityAbsent,
            TerminalStatus::Refused,
            IdempotencyStage::AuthorityValidation,
            ProblemCategory::Authority,
            RetryDirective::AfterAuthorityChange,
        ),
        K::AuthorityAmbiguous => (
            ContractProblemCauseV1::AuthorityAmbiguous,
            TerminalStatus::Refused,
            IdempotencyStage::AuthorityValidation,
            ProblemCategory::Authority,
            RetryDirective::AfterAuthorityChange,
        ),
        K::StaleAuthority => (
            ContractProblemCauseV1::AuthorityStale,
            TerminalStatus::Refused,
            IdempotencyStage::AuthorityValidation,
            ProblemCategory::Authority,
            RetryDirective::AfterAuthorityChange,
        ),
        K::AuthenticationRefused => (
            ContractProblemCauseV1::AuthenticationRefused,
            TerminalStatus::Refused,
            IdempotencyStage::AuthorityValidation,
            ProblemCategory::Authority,
            RetryDirective::AfterAuthorityChange,
        ),
        K::AuthenticatorUnavailable => (
            ContractProblemCauseV1::AuthenticatorUnavailable,
            TerminalStatus::Blocked,
            IdempotencyStage::CapabilityValidation,
            ProblemCategory::Prerequisite,
            RetryDirective::AfterRecheck,
        ),
        K::DurabilityViolation => (
            ContractProblemCauseV1::AuthorityDurabilityViolation,
            TerminalStatus::Refused,
            IdempotencyStage::AuthorityValidation,
            ProblemCategory::Safety,
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

fn owner_problem_response(
    request: &OperationRequestV1,
    definition: &OperationDefinition,
    projection: ProblemProjection,
    mutation: bool,
) -> Result<OperationResponseV1, ContractError> {
    let limits = ContractLimits::packet0();
    let details = ContractOperationProblemDetailsV1 {
        cause: projection.cause,
    };
    let details_value = super::artifact_operation::to_contract_value(&details)?;
    let recheck = if projection.status == TerminalStatus::Blocked {
        Some(super::RecheckCondition::new(
            schema_binding(DETAILS_SCHEMA_REF)?,
            details_value.clone(),
            &limits,
        )?)
    } else {
        None
    };
    let code = format!("contract.{}", cause_name(projection.cause));
    let problem = Problem::new(
        BoundedString::new(code.clone())?,
        BoundedString::new(code)?,
        projection.category,
        definition.binding(),
        (projection.status == TerminalStatus::Refused).then(|| definition.binding()),
        schema_binding(DETAILS_SCHEMA_REF)?,
        details_value,
        BoundedVec::new(Vec::new())?,
        projection.retry,
        recheck,
        None,
        &limits,
    )?;
    let binding = ProblemBinding::new(
        problem.problem_id().clone(),
        problem.problem_fingerprint().clone(),
    )?;
    let terminal = match projection.status {
        TerminalStatus::Blocked => TerminalFields::new(
            TerminalStatus::Blocked,
            None,
            BoundedVec::new(vec![problem])?,
            BoundedVec::new(Vec::new())?,
            BoundedVec::new(Vec::new())?,
        )?,
        TerminalStatus::Refused => TerminalFields::new(
            TerminalStatus::Refused,
            None,
            BoundedVec::new(Vec::new())?,
            BoundedVec::new(vec![problem])?,
            BoundedVec::new(Vec::new())?,
        )?,
        _ => return Err(ContractError::InvalidInvariant),
    };
    let idempotency = if mutation {
        IdempotencyState::not_established(projection.stage, binding)?
    } else {
        IdempotencyState::not_applicable()?
    };
    response_from_parts(
        request,
        definition,
        terminal,
        idempotency,
        BoundedVec::new(Vec::new())?,
    )
}

fn success_response(
    request: &OperationRequestV1,
    definition: &OperationDefinition,
    data: ContractValue,
    idempotency: IdempotencyState,
    receipts: BoundedVec<WriteReceipt, 128>,
) -> Result<OperationResponseV1, ContractError> {
    let terminal = TerminalFields::new(
        TerminalStatus::Ok,
        Some(data),
        BoundedVec::new(Vec::new())?,
        BoundedVec::new(Vec::new())?,
        BoundedVec::new(Vec::new())?,
    )?;
    response_from_parts(request, definition, terminal, idempotency, receipts)
}

fn mutation_success(
    request: &OperationRequestV1,
    definition: &OperationDefinition,
    scoped_key: Fingerprint,
    disposition: ContractMutationDispositionV1,
    data: ContractValue,
    records: Vec<(&str, &str, Fingerprint)>,
) -> Result<OperationResponseV1, ContractError> {
    let limits = ContractLimits::packet0();
    let replayed = disposition == ContractMutationDispositionV1::Replayed;
    let receipts = BoundedVec::new(
        records
            .into_iter()
            .map(|(kind, reference, fingerprint)| {
                WriteReceipt::new(
                    BoundedString::new(kind.to_owned())?,
                    ExactBinding::new(ExactRef::parse(reference)?, fingerprint),
                    BoundedString::new(
                        definition
                            .write_set()
                            .iter()
                            .find(|write| write.record_kind() == kind)
                            .ok_or(ContractError::InvalidInvariant)?
                            .authority_class()
                            .to_owned(),
                    )?,
                    data.clone(),
                    BoundedString::new(definition.write_set()[0].atomic_group().to_owned())?,
                    &limits,
                )
            })
            .collect::<Result<Vec<_>, ContractError>>()?,
    )?;
    let terminal = TerminalFields::new(
        TerminalStatus::Ok,
        Some(data),
        BoundedVec::new(Vec::new())?,
        BoundedVec::new(Vec::new())?,
        BoundedVec::new(Vec::new())?,
    )?;
    let retention = UtcTimestamp::parse(RETENTION_UNTIL_UTC)?;
    let original = if replayed {
        Some(
            response_from_parts(
                request,
                definition,
                terminal.clone(),
                IdempotencyState::established(
                    scoped_key.clone(),
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
        terminal,
        IdempotencyState::established(
            scoped_key,
            request.request_fingerprint().clone(),
            replayed,
            original,
            retention,
        )?,
        receipts,
    )
}

fn response_from_parts(
    request: &OperationRequestV1,
    definition: &OperationDefinition,
    terminal: TerminalFields<ContractValue>,
    idempotency: IdempotencyState,
    receipts: BoundedVec<WriteReceipt, 128>,
) -> Result<OperationResponseV1, ContractError> {
    OperationResponseV1::new(
        schema_binding("handbook.operation-response@1.0.0")?,
        request.request_id().cloned(),
        request.request_fingerprint().clone(),
        definition,
        terminal,
        idempotency,
        receipts,
        BoundedVec::new(Vec::new())?,
        BoundedVec::new(Vec::new())?,
        &ContractOperationSchemaValidator,
        &ContractLimits::packet0(),
    )
}

fn operation_class(operation_ref: &str) -> OperationClass {
    match operation_ref {
        CONTRACT_DEFINITION_APPEND_REF => OperationClass::AppendOnly,
        CONTRACT_LIFECYCLE_TRANSITION_REF => OperationClass::CompareAndWrite,
        _ => OperationClass::ReadOnly,
    }
}

fn operation_idempotency(operation_ref: &str) -> Result<OperationIdempotency, ContractError> {
    match operation_ref {
        CONTRACT_DEFINITION_APPEND_REF => {
            OperationIdempotency::key_required(JsonPointer::parse("/body/idempotency_key")?)
        }
        CONTRACT_LIFECYCLE_TRANSITION_REF => OperationIdempotency::compare_and_write(
            JsonPointer::parse("/body/idempotency_key")?,
            BoundedVec::new(vec![
                JsonPointer::parse("/body/contract/fingerprint")?,
                JsonPointer::parse("/body/contract/ref")?,
                JsonPointer::parse("/body/transition/expected_lifecycle_fingerprint")?,
            ])?,
        ),
        _ => Ok(OperationIdempotency::safe()),
    }
}

fn operation_write_set(
    operation_ref: &str,
    condition_schema: ExactBinding,
) -> Result<Vec<DeclaredWrite>, ContractError> {
    let write = |kind: &str, authority: &str, group: &str| {
        DeclaredWrite::new(
            BoundedString::new(kind.to_owned())?,
            BoundedString::new(authority.to_owned())?,
            BoundedString::new(group.to_owned())?,
            condition_schema.clone(),
        )
    };
    match operation_ref {
        CONTRACT_DEFINITION_APPEND_REF => Ok(vec![
            write(
                "contract_definition",
                "contract_definition_author",
                "contract_definition_append",
            )?,
            write(
                "contract_lifecycle_head",
                "contract_definition_author",
                "contract_definition_append",
            )?,
        ]),
        CONTRACT_LIFECYCLE_TRANSITION_REF => Ok(vec![
            write(
                "contract_lifecycle_head",
                "contract_lifecycle",
                "contract_lifecycle_transition",
            )?,
            write(
                "contract_lifecycle_transition",
                "contract_lifecycle",
                "contract_lifecycle_transition",
            )?,
        ]),
        _ => Ok(Vec::new()),
    }
}

fn request_schema_ref(operation_ref: &str) -> String {
    format!(
        "handbook.operation.{}-request@1.0.0",
        operation_ref.trim_end_matches("@1.0.0").replace('.', "-")
    )
}
fn result_schema_ref(operation_ref: &str) -> String {
    format!(
        "handbook.operation.{}-result@1.0.0",
        operation_ref.trim_end_matches("@1.0.0").replace('.', "-")
    )
}
fn schema_binding(reference: &str) -> Result<ExactBinding, ContractError> {
    super::production_witness::current_schema_binding(reference)
}
fn from_body<T: for<'de> Deserialize<'de> + Serialize>(
    request: &OperationRequestV1,
) -> Result<T, ContractError> {
    owner_from_contract_value(request.body())
}
fn owner_from_contract_value<T: for<'de> Deserialize<'de>>(
    value: &ContractValue,
) -> Result<T, ContractError> {
    serde_json::from_slice(&canonical_json_bytes(value, &ContractLimits::packet0())?)
        .map_err(|_| ContractError::InvalidInvariant)
}
fn owner_to_contract_value<T: Serialize>(value: &T) -> Result<ContractValue, ContractError> {
    super::artifact_operation::to_contract_value(value)
}
fn owner_ref(value: &ExactRef) -> Result<OwnerRef, ContractError> {
    OwnerRef::parse(value.as_str()).map_err(|_| ContractError::InvalidExactRef)
}
fn owner_fingerprint(value: &Fingerprint) -> Result<OwnerFingerprint, ContractError> {
    OwnerFingerprint::parse(value.as_str()).map_err(|_| ContractError::InvalidFingerprint)
}
fn owner_binding(value: &ExactBinding) -> Result<OwnerBinding, ContractError> {
    Ok(OwnerBinding::new(
        owner_ref(value.reference())?,
        owner_fingerprint(value.fingerprint())?,
    ))
}

fn lifecycle_head(
    value: &handbook_contracts::LifecycleHeadV1,
) -> Result<ContractLifecycleHeadDtoV1, ContractError> {
    Ok(ContractLifecycleHeadDtoV1 {
        state: sdk_state(value.state()),
        transition_ref: value
            .transition_ref()
            .map(|reference| BoundedString::new(reference.to_owned()))
            .transpose()?,
        lifecycle_fingerprint: Fingerprint::parse(value.lifecycle_fingerprint().as_str())?,
    })
}
fn disposition(value: OwnerMutationDispositionV1) -> ContractMutationDispositionV1 {
    match value {
        OwnerMutationDispositionV1::Committed => ContractMutationDispositionV1::Committed,
        OwnerMutationDispositionV1::Replayed => ContractMutationDispositionV1::Replayed,
    }
}
fn sdk_state(value: OwnerState) -> ContractLifecycleStateDtoV1 {
    match value {
        OwnerState::Draft => ContractLifecycleStateDtoV1::Draft,
        OwnerState::ReviewReady => ContractLifecycleStateDtoV1::ReviewReady,
        OwnerState::Locked => ContractLifecycleStateDtoV1::Locked,
        OwnerState::Active => ContractLifecycleStateDtoV1::Active,
        OwnerState::Deprecated => ContractLifecycleStateDtoV1::Deprecated,
        OwnerState::Closed => ContractLifecycleStateDtoV1::Closed,
    }
}
fn owner_state(value: ContractLifecycleStateDtoV1) -> OwnerState {
    match value {
        ContractLifecycleStateDtoV1::Draft => OwnerState::Draft,
        ContractLifecycleStateDtoV1::ReviewReady => OwnerState::ReviewReady,
        ContractLifecycleStateDtoV1::Locked => OwnerState::Locked,
        ContractLifecycleStateDtoV1::Active => OwnerState::Active,
        ContractLifecycleStateDtoV1::Deprecated => OwnerState::Deprecated,
        ContractLifecycleStateDtoV1::Closed => OwnerState::Closed,
    }
}
fn owner_approval(value: ContractApprovalClassDtoV1) -> OwnerApprovalClass {
    match value {
        ContractApprovalClassDtoV1::DefinitionAuthor => OwnerApprovalClass::DefinitionAuthor,
        ContractApprovalClassDtoV1::IndependentLock => OwnerApprovalClass::IndependentLock,
        ContractApprovalClassDtoV1::Activation => OwnerApprovalClass::Activation,
        ContractApprovalClassDtoV1::Lifecycle => OwnerApprovalClass::Lifecycle,
    }
}
fn owner_evidence_kind(value: ContractLifecycleEvidenceKindDtoV1) -> OwnerEvidenceKind {
    match value {
        ContractLifecycleEvidenceKindDtoV1::DefinitionValidation => {
            OwnerEvidenceKind::DefinitionValidation
        }
        ContractLifecycleEvidenceKindDtoV1::IndependentReview => {
            OwnerEvidenceKind::IndependentReview
        }
        ContractLifecycleEvidenceKindDtoV1::Rejection => OwnerEvidenceKind::Rejection,
        ContractLifecycleEvidenceKindDtoV1::ApplicabilityClosure => {
            OwnerEvidenceKind::ApplicabilityClosure
        }
        ContractLifecycleEvidenceKindDtoV1::MatcherClosure => OwnerEvidenceKind::MatcherClosure,
        ContractLifecycleEvidenceKindDtoV1::PolicyClosure => OwnerEvidenceKind::PolicyClosure,
        ContractLifecycleEvidenceKindDtoV1::CurrentnessClosure => {
            OwnerEvidenceKind::CurrentnessClosure
        }
        ContractLifecycleEvidenceKindDtoV1::Replacement => OwnerEvidenceKind::Replacement,
        ContractLifecycleEvidenceKindDtoV1::NoReplacementRationale => {
            OwnerEvidenceKind::NoReplacementRationale
        }
        ContractLifecycleEvidenceKindDtoV1::RetentionSafety => OwnerEvidenceKind::RetentionSafety,
        ContractLifecycleEvidenceKindDtoV1::ReferenceSafety => OwnerEvidenceKind::ReferenceSafety,
    }
}
fn owner_reason(value: ContractLifecycleReasonDtoV1) -> Result<OwnerReason, ContractError> {
    Ok(match value {
        ContractLifecycleReasonDtoV1::ReadyForReview => OwnerReason::ReadyForReview,
        ContractLifecycleReasonDtoV1::Withdrawn => OwnerReason::Withdrawn,
        ContractLifecycleReasonDtoV1::IndependentlyLocked => OwnerReason::IndependentlyLocked,
        ContractLifecycleReasonDtoV1::Rejected => OwnerReason::Rejected,
        ContractLifecycleReasonDtoV1::Activated => OwnerReason::Activated,
        ContractLifecycleReasonDtoV1::DeprecatedWithReplacement(binding) => {
            OwnerReason::DeprecatedWithReplacement(owner_binding(&binding)?)
        }
        ContractLifecycleReasonDtoV1::DeprecatedWithoutReplacement(detail) => {
            OwnerReason::DeprecatedWithoutReplacement(
                LifecycleReasonTextV1::parse(detail.as_str())
                    .map_err(|_| ContractError::InvalidString)?,
            )
        }
        ContractLifecycleReasonDtoV1::ClosedAfterRetention => OwnerReason::ClosedAfterRetention,
    })
}
fn cause_name(value: ContractProblemCauseV1) -> &'static str {
    match value {
        ContractProblemCauseV1::InvalidRequest => "invalid_request",
        ContractProblemCauseV1::BoundExceeded => "bound_exceeded",
        ContractProblemCauseV1::RecordSubstituted => "record_substituted",
        ContractProblemCauseV1::IdempotencyConflict => "idempotency_conflict",
        ContractProblemCauseV1::LifecycleRefused => "lifecycle_refused",
        ContractProblemCauseV1::UnsafeFilesystem => "unsafe_filesystem",
        ContractProblemCauseV1::DurabilityUncertain => "durability_uncertain",
        ContractProblemCauseV1::RecoveryAmbiguous => "recovery_ambiguous",
        ContractProblemCauseV1::AuthorityAbsent => "authority_absent",
        ContractProblemCauseV1::AuthorityAmbiguous => "authority_ambiguous",
        ContractProblemCauseV1::AuthorityStale => "authority_stale",
        ContractProblemCauseV1::AuthenticationRefused => "authentication_refused",
        ContractProblemCauseV1::AuthenticatorUnavailable => "authenticator_unavailable",
        ContractProblemCauseV1::AuthorityDurabilityViolation => "authority_durability_violation",
    }
}

pub(crate) fn inventory_row_is_contract(operation_ref: &ExactRef) -> bool {
    CONTRACT_OPERATION_REFS.contains(&operation_ref.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mutation_projection_validates_receipts_and_preserves_one_replay_result() {
        let fingerprint = |digit: char| {
            Fingerprint::parse(&format!("sha256:{}", digit.to_string().repeat(64))).unwrap()
        };
        let contract = ExactBinding::new(
            ExactRef::parse("handbook.contract.sdk-projection-test@1.0.0").unwrap(),
            fingerprint('1'),
        );
        let request = contract_lifecycle_transition_operation_request(
            Some("contract-transition-projection"),
            "owner-key",
            ContractLifecycleTransitionRequestV1::new(
                contract,
                ContractLifecycleTransitionInputDtoV1 {
                    expected_lifecycle_fingerprint: fingerprint('2'),
                    from_state: ContractLifecycleStateDtoV1::Draft,
                    to_state: ContractLifecycleStateDtoV1::ReviewReady,
                    approval_class: ContractApprovalClassDtoV1::DefinitionAuthor,
                    supporting_evidence: BoundedVec::new(Vec::new()).unwrap(),
                    reason: ContractLifecycleReasonDtoV1::ReadyForReview,
                },
            ),
        )
        .unwrap();
        let definition = contract_operation_definition(CONTRACT_LIFECYCLE_TRANSITION_REF).unwrap();
        let result = ContractLifecycleTransitionResultV1 {
            owner_request_fingerprint: fingerprint('3'),
            transition_ref: BoundedString::new(
                "handbook.contract-lifecycle-transition.sdk-projection-test@1.0.0".to_owned(),
            )
            .unwrap(),
            transition_fingerprint: fingerprint('4'),
            lifecycle_head: ContractLifecycleHeadDtoV1 {
                state: ContractLifecycleStateDtoV1::ReviewReady,
                transition_ref: Some(
                    BoundedString::new(
                        "handbook.contract-lifecycle-transition.sdk-projection-test@1.0.0"
                            .to_owned(),
                    )
                    .unwrap(),
                ),
                lifecycle_fingerprint: fingerprint('5'),
            },
        };
        let data = super::super::artifact_operation::to_contract_value(&result).unwrap();
        let records = || {
            vec![
                (
                    "contract_lifecycle_head",
                    "handbook.contract-lifecycle-head@1.0.0",
                    fingerprint('5'),
                ),
                (
                    "contract_lifecycle_transition",
                    "handbook.contract-lifecycle-transition-record@1.0.0",
                    fingerprint('4'),
                ),
            ]
        };
        let key = request.idempotency_key_fingerprint().unwrap().clone();
        let committed = mutation_success(
            &request,
            &definition,
            key.clone(),
            ContractMutationDispositionV1::Committed,
            data.clone(),
            records(),
        )
        .unwrap();
        let replayed = mutation_success(
            &request,
            &definition,
            key,
            ContractMutationDispositionV1::Replayed,
            data,
            records(),
        )
        .unwrap();

        assert_eq!(committed.receipts().len(), 2);
        assert_eq!(replayed.receipts(), committed.receipts());
        replayed.verify_replay_against(&committed).unwrap();
        let bytes = canonical_json_bytes(&replayed, &ContractLimits::packet0()).unwrap();
        OperationResponseV1::parse_and_admit(
            &bytes,
            &definition,
            &ContractOperationSchemaValidator,
            &ContractLimits::packet0(),
        )
        .unwrap();
    }
}
