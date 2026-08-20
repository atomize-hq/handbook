use super::{
    canonical_json_bytes, parse_canonical_json, BoundedString, BoundedVec, CapabilityEntry,
    ContractError, ContractEvaluationRunDtoV1, ContractLimits, ContractRecordBindingDtoV1,
    ContractValue, CurrentTransportEvidence, CurrentTransportEvidenceSealed, DeclaredWrite,
    ExactBinding, ExactRef, Fingerprint, JsonPointer, OperationAdmissionResult, OperationClass,
    OperationDefinition, OperationIdempotency, OperationOutcomeSchemas, OperationOwnerBinding,
    OperationRequestV1, OperationResponseV1, OperationSchemaValidator, OperationTransport,
    OwnerAdmissionToken, Problem, ProblemBinding, ProblemCategory, RequestId, RetryDirective,
    SafeU64, SemVer, TerminalFields, TerminalStatus, UtcTimestamp, WriteReceipt,
};
use handbook_contracts::{
    ContractOwnerRepositoryErrorKindV1, ContractOwnerRepositoryV1, EvaluationCaseV1,
    EvaluationPolicyRegistryV1, EvaluationRunNonceV1, EvaluationSubjectV1,
    ExactBindingV1 as OwnerBinding, ExactRefV1 as OwnerExactRef, FingerprintV1 as OwnerFingerprint,
    MatcherBindingV1, PrepareEvaluationRunInputV1, ResolutionEnvelopeV1, StableIdV1,
};
use serde::{Deserialize, Serialize};
use std::path::Path;

pub const DOCK_MANIFEST_LIST_REF: &str = "dock.manifest.list@1.0.0";
pub const DOCK_MANIFEST_READ_REF: &str = "dock.manifest.read@1.0.0";
pub const DOCK_RUN_REF: &str = "dock.run@1.0.0";
pub const DOCK_OPERATION_REFS: [&str; 3] =
    [DOCK_MANIFEST_LIST_REF, DOCK_MANIFEST_READ_REF, DOCK_RUN_REF];
pub const DOCK_OPERATION_DETAILS_SCHEMA_REF: &str = "handbook.dock-operation-details@1.0.0";

macro_rules! dock_document_dto {
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

dock_document_dto!(DockManifestDtoV1);
dock_document_dto!(DockProcessRequestDtoV1);
dock_document_dto!(DockProcessResultDtoV1);
dock_document_dto!(DockExecutionRecordDtoV1);

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "contract-schema-generation", derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum DockOperationProblemCauseV1 {
    Malformed,
    OverLimit,
    Missing,
    Unknown,
    Stale,
    Substituted,
    Conflict,
    UnsupportedTarget,
    ManifestUnavailable,
    LaunchRefused,
    TimedOut,
    Cancelled,
    ProcessFailed,
    ProtocolError,
    CleanupUncertain,
    DurabilityUncertain,
    InternalInvariant,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "contract-schema-generation", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct DockOperationDetailsDtoV1 {
    cause: DockOperationProblemCauseV1,
    related_refs: BoundedVec<BoundedString<4096>, 128>,
    diagnostics: BoundedVec<BoundedString<4096>, 128>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "contract-schema-generation", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct DockManifestListRequestV1 {
    limit: SafeU64,
    root: Option<Fingerprint>,
    cursor: Option<BoundedString<4096>>,
}

impl DockManifestListRequestV1 {
    pub fn first(limit: u64) -> Result<Self, ContractError> {
        let request = Self {
            limit: SafeU64::new(limit)?,
            root: None,
            cursor: None,
        };
        request.validate()?;
        Ok(request)
    }

    pub fn continue_from(
        limit: u64,
        root: Fingerprint,
        cursor: &str,
    ) -> Result<Self, ContractError> {
        let request = Self {
            limit: SafeU64::new(limit)?,
            root: Some(root),
            cursor: Some(BoundedString::new(cursor.to_owned())?),
        };
        request.validate()?;
        Ok(request)
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
pub struct DockManifestListEntryDtoV1 {
    manifest: ContractRecordBindingDtoV1,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "contract-schema-generation", derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum DockManifestRetentionDtoV1 {
    ImmutableExact,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "contract-schema-generation", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct DockManifestListResultV1 {
    root: Fingerprint,
    entries: BoundedVec<DockManifestListEntryDtoV1, 100>,
    next_cursor: Option<BoundedString<4096>>,
    retention: DockManifestRetentionDtoV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "contract-schema-generation", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct DockManifestReadRequestV1 {
    manifest: ContractRecordBindingDtoV1,
}

impl DockManifestReadRequestV1 {
    pub fn new(manifest: ContractRecordBindingDtoV1) -> Self {
        Self { manifest }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "contract-schema-generation", derive(schemars::JsonSchema))]
#[serde(tag = "outcome", rename_all = "snake_case", deny_unknown_fields)]
pub enum DockManifestReadResultV1 {
    Found {
        manifest: DockManifestDtoV1,
    },
    Absent {
        requested: ContractRecordBindingDtoV1,
    },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "contract-schema-generation", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct DockEvaluationPreparationDtoV1 {
    contract: ContractRecordBindingDtoV1,
    expected_lifecycle_fingerprint: Fingerprint,
    subject: ContractValue,
    cases: BoundedVec<ContractValue, 128>,
    requested_resolution: ContractValue,
    matcher: ContractValue,
    freshness_policies: BoundedVec<ContractRecordBindingDtoV1, 32>,
    stability_policies: BoundedVec<ContractRecordBindingDtoV1, 32>,
    gate_policy: ContractRecordBindingDtoV1,
    local_closeout_policy: ContractRecordBindingDtoV1,
    nonce: BoundedString<255>,
}

impl DockEvaluationPreparationDtoV1 {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        contract: ContractRecordBindingDtoV1,
        expected_lifecycle_fingerprint: Fingerprint,
        subject: ContractValue,
        cases: BoundedVec<ContractValue, 128>,
        requested_resolution: ContractValue,
        matcher: ContractValue,
        freshness_policies: BoundedVec<ContractRecordBindingDtoV1, 32>,
        stability_policies: BoundedVec<ContractRecordBindingDtoV1, 32>,
        gate_policy: ContractRecordBindingDtoV1,
        local_closeout_policy: ContractRecordBindingDtoV1,
        nonce: BoundedString<255>,
    ) -> Self {
        Self {
            contract,
            expected_lifecycle_fingerprint,
            subject,
            cases,
            requested_resolution,
            matcher,
            freshness_policies,
            stability_policies,
            gate_policy,
            local_closeout_policy,
            nonce,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "contract-schema-generation", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct DockInlineJsonInputDtoV1 {
    role: BoundedString<255>,
    media_type: BoundedString<255>,
    schema: ContractRecordBindingDtoV1,
    content: ContractValue,
}

impl DockInlineJsonInputDtoV1 {
    pub fn new(
        role: BoundedString<255>,
        media_type: BoundedString<255>,
        schema: ContractRecordBindingDtoV1,
        content: ContractValue,
    ) -> Self {
        Self {
            role,
            media_type,
            schema,
            content,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "contract-schema-generation", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct DockProtocolLimitsDtoV1 {
    request_bytes: SafeU64,
    result_bytes: SafeU64,
    stderr_bytes: SafeU64,
    diagnostic_count: SafeU64,
    diagnostic_bytes: SafeU64,
    candidate_count: SafeU64,
    candidate_bytes: SafeU64,
    input_count: SafeU64,
    wall_timeout_ms: SafeU64,
    cancellation_grace_ms: SafeU64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DockProtocolLimitsInputV1 {
    pub request_bytes: u64,
    pub result_bytes: u64,
    pub stderr_bytes: u64,
    pub diagnostic_count: u64,
    pub diagnostic_bytes: u64,
    pub candidate_count: u64,
    pub candidate_bytes: u64,
    pub input_count: u64,
    pub wall_timeout_ms: u64,
    pub cancellation_grace_ms: u64,
}

impl DockProtocolLimitsDtoV1 {
    pub fn new(input: DockProtocolLimitsInputV1) -> Result<Self, ContractError> {
        Ok(Self {
            request_bytes: SafeU64::new(input.request_bytes)?,
            result_bytes: SafeU64::new(input.result_bytes)?,
            stderr_bytes: SafeU64::new(input.stderr_bytes)?,
            diagnostic_count: SafeU64::new(input.diagnostic_count)?,
            diagnostic_bytes: SafeU64::new(input.diagnostic_bytes)?,
            candidate_count: SafeU64::new(input.candidate_count)?,
            candidate_bytes: SafeU64::new(input.candidate_bytes)?,
            input_count: SafeU64::new(input.input_count)?,
            wall_timeout_ms: SafeU64::new(input.wall_timeout_ms)?,
            cancellation_grace_ms: SafeU64::new(input.cancellation_grace_ms)?,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "contract-schema-generation", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct DockRunRequestV1 {
    manifest: ContractRecordBindingDtoV1,
    evaluation: DockEvaluationPreparationDtoV1,
    requested_evidence_kinds: BoundedVec<BoundedString<255>, 128>,
    inputs: BoundedVec<DockInlineJsonInputDtoV1, 128>,
    limits: DockProtocolLimitsDtoV1,
}

impl DockRunRequestV1 {
    pub fn new(
        manifest: ContractRecordBindingDtoV1,
        evaluation: DockEvaluationPreparationDtoV1,
        requested_evidence_kinds: BoundedVec<BoundedString<255>, 128>,
        inputs: BoundedVec<DockInlineJsonInputDtoV1, 128>,
        limits: DockProtocolLimitsDtoV1,
    ) -> Self {
        Self {
            manifest,
            evaluation,
            requested_evidence_kinds,
            inputs,
            limits,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "contract-schema-generation", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub(crate) struct DockRunWireV1 {
    manifest: ContractRecordBindingDtoV1,
    evaluation: DockEvaluationPreparationDtoV1,
    requested_evidence_kinds: BoundedVec<BoundedString<255>, 128>,
    inputs: BoundedVec<DockInlineJsonInputDtoV1, 128>,
    limits: DockProtocolLimitsDtoV1,
    idempotency_key: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "contract-schema-generation", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct DockRunResultV1 {
    evaluation_run: ContractEvaluationRunDtoV1,
    execution_record: DockExecutionRecordDtoV1,
    candidates: BoundedVec<ContractRecordBindingDtoV1, 128>,
}

impl DockRunResultV1 {
    pub fn evaluation_run(&self) -> &ContractEvaluationRunDtoV1 {
        &self.evaluation_run
    }

    pub fn execution_record(&self) -> &DockExecutionRecordDtoV1 {
        &self.execution_record
    }

    pub fn candidates(&self) -> &[ContractRecordBindingDtoV1] {
        self.candidates.as_slice()
    }
}

pub struct DockOperationSchemaValidator;

impl OperationSchemaValidator for DockOperationSchemaValidator {
    fn validate_instance(
        &self,
        schema: &ExactBinding,
        instance: &ContractValue,
    ) -> Result<(), ContractError> {
        let bytes = canonical_json_bytes(instance, &ContractLimits::packet0())?;
        match schema.reference().as_str() {
            "handbook.dock-manifest@1.0.0" => parse::<DockManifestDtoV1>(&bytes),
            "handbook.dock-process-request@1.0.0" => parse::<DockProcessRequestDtoV1>(&bytes),
            "handbook.dock-process-result@1.0.0" => parse::<DockProcessResultDtoV1>(&bytes),
            "handbook.dock-execution-record@1.0.0" => parse::<DockExecutionRecordDtoV1>(&bytes),
            DOCK_OPERATION_DETAILS_SCHEMA_REF => parse::<DockOperationDetailsDtoV1>(&bytes),
            "handbook.operation.dock-manifest-list-request@1.0.0" => {
                parse_canonical_json::<DockManifestListRequestV1>(
                    &bytes,
                    &ContractLimits::packet0(),
                )?
                .validate()
            }
            "handbook.operation.dock-manifest-list-result@1.0.0" => {
                parse::<DockManifestListResultV1>(&bytes)
            }
            "handbook.operation.dock-manifest-read-request@1.0.0" => {
                parse::<DockManifestReadRequestV1>(&bytes)
            }
            "handbook.operation.dock-manifest-read-result@1.0.0" => {
                parse::<DockManifestReadResultV1>(&bytes)
            }
            "handbook.operation.dock-run-request@1.0.0" => parse::<DockRunWireV1>(&bytes),
            "handbook.operation.dock-run-result@1.0.0" => parse::<DockRunResultV1>(&bytes),
            _ => Err(ContractError::InvalidInvariant),
        }
    }
}

fn parse<T: for<'de> Deserialize<'de> + Serialize>(bytes: &[u8]) -> Result<(), ContractError> {
    parse_canonical_json::<T>(bytes, &ContractLimits::packet0()).map(|_| ())
}

pub fn dock_manifest_list_operation_request(
    request_id: Option<&str>,
    body: DockManifestListRequestV1,
) -> Result<OperationRequestV1, ContractError> {
    operation_request(
        DOCK_MANIFEST_LIST_REF,
        request_id,
        super::artifact_operation::to_contract_value(&body)?,
    )
}

pub fn dock_manifest_read_operation_request(
    request_id: Option<&str>,
    body: DockManifestReadRequestV1,
) -> Result<OperationRequestV1, ContractError> {
    operation_request(
        DOCK_MANIFEST_READ_REF,
        request_id,
        super::artifact_operation::to_contract_value(&body)?,
    )
}

pub fn dock_run_operation_request(
    request_id: Option<&str>,
    idempotency_key: &str,
    body: DockRunRequestV1,
) -> Result<OperationRequestV1, ContractError> {
    operation_request(
        DOCK_RUN_REF,
        request_id,
        super::artifact_operation::to_contract_value(&DockRunWireV1 {
            manifest: body.manifest,
            evaluation: body.evaluation,
            requested_evidence_kinds: body.requested_evidence_kinds,
            inputs: body.inputs,
            limits: body.limits,
            idempotency_key: idempotency_key.to_owned(),
        })?,
    )
}

fn operation_request(
    operation_ref: &str,
    request_id: Option<&str>,
    body: ContractValue,
) -> Result<OperationRequestV1, ContractError> {
    let definition = dock_operation_definition(operation_ref)?;
    OperationRequestV1::new(
        schema_binding("handbook.operation-request@1.0.0")?,
        request_id.map(RequestId::parse).transpose()?,
        &definition,
        body,
        &DockOperationSchemaValidator,
        &ContractLimits::packet0(),
    )
}

pub fn execute_dock_operation(
    repo_root: &Path,
    request: &OperationRequestV1,
) -> Result<OperationResponseV1, ContractError> {
    let policies =
        EvaluationPolicyRegistryV1::new().map_err(|_| ContractError::InvalidInvariant)?;
    execute_dock_operation_with_context(repo_root, request, &policies)
}

pub fn execute_dock_operation_with_context(
    repo_root: &Path,
    request: &OperationRequestV1,
    policies: &EvaluationPolicyRegistryV1,
) -> Result<OperationResponseV1, ContractError> {
    let definition = dock_operation_definition(request.operation().reference().as_str())?;
    if request.operation() != &definition.binding()
        || request.body_schema() != definition.request_schema()
    {
        return Err(ContractError::InvalidInvariant);
    }
    match request.operation().reference().as_str() {
        DOCK_MANIFEST_LIST_REF => execute_manifest_list(request, &definition),
        DOCK_MANIFEST_READ_REF => execute_manifest_read(request, &definition),
        DOCK_RUN_REF => execute_run(repo_root, request, &definition, policies),
        _ => Err(ContractError::InvalidInvariant),
    }
}

/// Owner/runtime capabilities for the only Dock operation that can launch.
/// The caller cannot bypass Contracts preparation and Dock re-admission: this
/// capability is consumed only after both checks have succeeded.
pub struct DockRunExecutionContextV1<'a> {
    pub policies: &'a EvaluationPolicyRegistryV1,
    pub manifest: &'a handbook_dock::DockManifestV1,
    pub resolved_bundle: &'a handbook_dock::DockResolvedFirstPartyBundleV1,
    pub store: &'a handbook_dock::DockOperationalStoreV1,
    pub cancellation: &'a handbook_dock::DockCancellationV1,
}

pub fn execute_dock_operation_with_runtime(
    repo_root: &Path,
    request: &OperationRequestV1,
    context: DockRunExecutionContextV1<'_>,
) -> Result<OperationResponseV1, ContractError> {
    let definition = dock_operation_definition(request.operation().reference().as_str())?;
    if request.operation().reference().as_str() != DOCK_RUN_REF
        || request.operation() != &definition.binding()
        || request.body_schema() != definition.request_schema()
    {
        return Err(ContractError::InvalidInvariant);
    }
    execute_run_with_runtime(repo_root, request, &definition, context)
}

fn execute_run(
    repo_root: &Path,
    request: &OperationRequestV1,
    definition: &OperationDefinition,
    policies: &EvaluationPolicyRegistryV1,
) -> Result<OperationResponseV1, ContractError> {
    let _admitted = match prepare_current_run(repo_root, request, policies) {
        Ok(admitted) => admitted,
        Err(cause) => return problem_response(request, definition, cause),
    };
    let body: DockRunRequestV1 = from_body(request)?;

    let manifest_ref = OwnerExactRef::parse(body.manifest.reference())
        .map_err(|_| ContractError::InvalidInvariant)?;
    let manifest_fingerprint = OwnerFingerprint::parse(body.manifest.fingerprint().as_str())
        .map_err(|_| ContractError::InvalidInvariant)?;
    match handbook_dock::DockManifestRegistryV1::production()
        .read(&manifest_ref, &manifest_fingerprint)
        .map_err(|_| ContractError::InvalidInvariant)?
    {
        handbook_dock::DockManifestReadOutcomeV1::Absent => problem_response(
            request,
            definition,
            DockOperationProblemCauseV1::ManifestUnavailable,
        ),
        handbook_dock::DockManifestReadOutcomeV1::Found(_) => problem_response(
            request,
            definition,
            DockOperationProblemCauseV1::LaunchRefused,
        ),
    }
}

fn execute_run_with_runtime(
    repo_root: &Path,
    request: &OperationRequestV1,
    definition: &OperationDefinition,
    context: DockRunExecutionContextV1<'_>,
) -> Result<OperationResponseV1, ContractError> {
    let admitted = match prepare_current_run(repo_root, request, context.policies) {
        Ok(admitted) => admitted,
        Err(cause) => return problem_response(request, definition, cause),
    };
    let body: DockRunRequestV1 = from_body(request)?;

    // The manifest, process inputs, limits, raw key, and executor are first
    // touched only after the exact current Contracts run is prepared and
    // independently re-admitted above.
    if owner_binding(&body.manifest)? != context.manifest.binding() {
        return problem_response(
            request,
            definition,
            DockOperationProblemCauseV1::Substituted,
        );
    }
    let process_request = match dock_process_request(&body, context.manifest, admitted.binding()) {
        Ok(process_request) => process_request,
        Err(cause) => return problem_response(request, definition, cause),
    };
    let idempotency_request_fingerprint =
        OwnerFingerprint::parse(request.request_fingerprint().as_str())
            .map_err(|_| ContractError::InvalidInvariant)?;
    let mut bridge = DockRunBridge {
        store: context.store,
        resolved_bundle: context.resolved_bundle,
        request: &process_request,
        idempotency_request_fingerprint: &idempotency_request_fingerprint,
        evaluation_run_jcs: admitted.canonical_json(),
        cancellation: context.cancellation,
        result: None,
    };
    let scoped_key = request
        .with_owner_idempotency_key(&mut bridge)
        .ok_or(ContractError::InvalidInvariant)?;
    let receipt = match bridge.result.ok_or(ContractError::InvalidInvariant)? {
        Ok(receipt) => receipt,
        Err(error) => return problem_response(request, definition, executor_problem(error.kind())),
    };
    let replayed = receipt.disposition() == handbook_dock::DockStoreDispositionV1::Replayed;
    let evaluation_run = receipt.execution_record().evaluation_run();
    let result = DockRunResultV1 {
        evaluation_run: ContractEvaluationRunDtoV1::new(
            record_binding(
                evaluation_run.reference(),
                evaluation_run.fingerprint().as_str(),
            )?,
            parse_canonical_json(receipt.evaluation_run_jcs(), &ContractLimits::packet0())?,
        ),
        execution_record: DockExecutionRecordDtoV1::new(
            record_binding(
                receipt.execution_record().record_ref(),
                receipt.execution_record().record_fingerprint().as_str(),
            )?,
            super::artifact_operation::to_contract_value(receipt.execution_record())?,
        ),
        candidates: BoundedVec::new(
            receipt
                .candidates()
                .iter()
                .map(|candidate| {
                    record_binding(candidate.reference(), candidate.fingerprint().as_str())
                })
                .collect::<Result<Vec<_>, ContractError>>()?,
        )?,
    };
    mutation_success_response(request, definition, scoped_key, replayed, &result)
}

fn prepare_current_run(
    repo_root: &Path,
    request: &OperationRequestV1,
    policies: &EvaluationPolicyRegistryV1,
) -> Result<handbook_dock::DockAdmittedEvaluationRunV1, DockOperationProblemCauseV1> {
    // Decode only the Contracts preparation projection. The manifest, process
    // inputs, limits, raw key, and executor body remain unobserved until the
    // resulting current run is independently re-admitted by Dock.
    let bytes = canonical_json_bytes(request.body(), &ContractLimits::packet0())
        .map_err(|_| DockOperationProblemCauseV1::Malformed)?;
    let value = parse_canonical_json::<serde_json::Value>(&bytes, &ContractLimits::packet0())
        .map_err(|_| DockOperationProblemCauseV1::Malformed)?;
    let evaluation = value
        .as_object()
        .and_then(|body| body.get("evaluation"))
        .ok_or(DockOperationProblemCauseV1::Malformed)?;
    let evaluation_bytes = canonical_json_bytes(evaluation, &ContractLimits::packet0())
        .map_err(|_| DockOperationProblemCauseV1::Malformed)?;
    let evaluation = parse_canonical_json::<DockEvaluationPreparationDtoV1>(
        &evaluation_bytes,
        &ContractLimits::packet0(),
    )
    .map_err(|_| DockOperationProblemCauseV1::Malformed)?;
    let owner =
        ContractOwnerRepositoryV1::open(repo_root).map_err(|error| owner_problem(error.kind()))?;
    let preparation =
        owner_preparation_input(&evaluation).map_err(|_| DockOperationProblemCauseV1::Malformed)?;
    let run = owner
        .prepare_evaluation_run_v1(preparation, policies)
        .map_err(|error| owner_problem(error.kind()))?;
    handbook_dock::readmit_current_evaluation_run_v1(&owner, &run, policies)
        .map_err(|_| DockOperationProblemCauseV1::Stale)
}

struct DockRunBridge<'a> {
    store: &'a handbook_dock::DockOperationalStoreV1,
    resolved_bundle: &'a handbook_dock::DockResolvedFirstPartyBundleV1,
    request: &'a handbook_dock::DockProcessRequestV1,
    idempotency_request_fingerprint: &'a OwnerFingerprint,
    evaluation_run_jcs: &'a [u8],
    cancellation: &'a handbook_dock::DockCancellationV1,
    result:
        Option<Result<handbook_dock::DockExecutionReceiptV1, handbook_dock::DockExecutorErrorV1>>,
}

impl super::IdempotencyKeyConsumerSealed for DockRunBridge<'_> {
    fn consume_owner_idempotency_key(&mut self, _pointer: &str, bytes: &[u8]) -> Fingerprint {
        let key = std::str::from_utf8(bytes).expect("raw key admission validated UTF-8");
        self.result = Some(
            handbook_dock::DockProcessExecutorV1::new().execute_with_idempotency_request(
                handbook_dock::DockExecutionInputV1 {
                    store: self.store,
                    resolved_bundle: self.resolved_bundle,
                    request: self.request,
                    evaluation_run_jcs: self.evaluation_run_jcs,
                    idempotency_key: key,
                    cancellation: self.cancellation,
                },
                self.idempotency_request_fingerprint,
            ),
        );
        super::sha256_fingerprint(bytes)
    }
}

impl super::IdempotencyKeyConsumer for DockRunBridge<'_> {}

fn dock_process_request(
    body: &DockRunRequestV1,
    manifest: &handbook_dock::DockManifestV1,
    run: &handbook_dock::DockEvaluationRunBindingV1,
) -> Result<handbook_dock::DockProcessRequestV1, DockOperationProblemCauseV1> {
    let requested_evidence_kinds = body
        .requested_evidence_kinds
        .as_slice()
        .iter()
        .map(|kind| {
            StableIdV1::parse(kind.as_str()).map_err(|_| DockOperationProblemCauseV1::Malformed)
        })
        .collect::<Result<Vec<_>, _>>()?;
    let inputs = body
        .inputs
        .as_slice()
        .iter()
        .map(|input| {
            let content = serde_json::from_slice(&canonical_json_bytes(
                &input.content,
                &ContractLimits::packet0(),
            )?)
            .map_err(|_| ContractError::InvalidInvariant)?;
            handbook_dock::DockInlineJsonInputV1::new(
                StableIdV1::parse(input.role.as_str())
                    .map_err(|_| ContractError::InvalidInvariant)?,
                input.media_type.as_str(),
                owner_binding(&input.schema)?,
                content,
            )
            .map_err(|_| ContractError::InvalidInvariant)
        })
        .collect::<Result<Vec<_>, ContractError>>()
        .map_err(|_| DockOperationProblemCauseV1::Malformed)?;
    let usize_value = |value: SafeU64| {
        usize::try_from(value.get()).map_err(|_| DockOperationProblemCauseV1::OverLimit)
    };
    let limits =
        handbook_dock::DockProtocolLimitsV1::new(handbook_dock::DockProtocolLimitInputV1 {
            request_bytes: usize_value(body.limits.request_bytes)?,
            result_bytes: usize_value(body.limits.result_bytes)?,
            stderr_bytes: usize_value(body.limits.stderr_bytes)?,
            diagnostic_bytes: usize_value(body.limits.diagnostic_bytes)?,
            candidate_bytes: usize_value(body.limits.candidate_bytes)?,
            input_count: usize_value(body.limits.input_count)?,
            candidate_count: usize_value(body.limits.candidate_count)?,
            diagnostic_count: usize_value(body.limits.diagnostic_count)?,
            wall_timeout_ms: body.limits.wall_timeout_ms.get(),
            cancellation_grace_ms: body.limits.cancellation_grace_ms.get(),
        })
        .map_err(|_| DockOperationProblemCauseV1::OverLimit)?;
    handbook_dock::DockProcessRequestV1::new(
        manifest,
        handbook_dock::DockProcessRequestInputV1 {
            run: run.clone(),
            requested_evidence_kinds,
            inputs,
            effective_limits: limits,
        },
    )
    .map_err(|_| DockOperationProblemCauseV1::LaunchRefused)
}

fn executor_problem(kind: handbook_dock::DockExecutorErrorKindV1) -> DockOperationProblemCauseV1 {
    use handbook_dock::DockExecutorErrorKindV1 as K;
    match kind {
        K::AdmissionRefused => DockOperationProblemCauseV1::LaunchRefused,
        K::IdempotencyConflict => DockOperationProblemCauseV1::Conflict,
        K::CancelledBeforeLaunch => DockOperationProblemCauseV1::Cancelled,
        K::DurabilityUncertain => DockOperationProblemCauseV1::DurabilityUncertain,
        K::InternalInvariant => DockOperationProblemCauseV1::InternalInvariant,
    }
}

fn owner_preparation_input(
    input: &DockEvaluationPreparationDtoV1,
) -> Result<PrepareEvaluationRunInputV1, ContractError> {
    PrepareEvaluationRunInputV1::new(
        owner_binding(&input.contract)?,
        OwnerFingerprint::parse(input.expected_lifecycle_fingerprint.as_str())
            .map_err(|_| ContractError::InvalidInvariant)?,
        owner_value::<EvaluationSubjectV1>(&input.subject)?,
        input
            .cases
            .as_slice()
            .iter()
            .map(owner_value::<EvaluationCaseV1>)
            .collect::<Result<Vec<_>, ContractError>>()?,
        owner_value::<ResolutionEnvelopeV1>(&input.requested_resolution)?,
        owner_value::<MatcherBindingV1>(&input.matcher)?,
        input
            .freshness_policies
            .as_slice()
            .iter()
            .map(owner_binding)
            .collect::<Result<Vec<_>, ContractError>>()?,
        input
            .stability_policies
            .as_slice()
            .iter()
            .map(owner_binding)
            .collect::<Result<Vec<_>, ContractError>>()?,
        owner_binding(&input.gate_policy)?,
        owner_binding(&input.local_closeout_policy)?,
        EvaluationRunNonceV1::parse(input.nonce.as_str())
            .map_err(|_| ContractError::InvalidInvariant)?,
    )
    .map_err(|_| ContractError::InvalidInvariant)
}

fn owner_binding(input: &ContractRecordBindingDtoV1) -> Result<OwnerBinding, ContractError> {
    Ok(OwnerBinding::new(
        OwnerExactRef::parse(input.reference()).map_err(|_| ContractError::InvalidInvariant)?,
        OwnerFingerprint::parse(input.fingerprint().as_str())
            .map_err(|_| ContractError::InvalidInvariant)?,
    ))
}

fn owner_value<T: for<'de> Deserialize<'de>>(input: &ContractValue) -> Result<T, ContractError> {
    serde_json::from_slice(&canonical_json_bytes(input, &ContractLimits::packet0())?)
        .map_err(|_| ContractError::InvalidInvariant)
}

fn owner_problem(kind: ContractOwnerRepositoryErrorKindV1) -> DockOperationProblemCauseV1 {
    use ContractOwnerRepositoryErrorKindV1 as K;
    match kind {
        K::InvalidRequest => DockOperationProblemCauseV1::Malformed,
        K::BoundExceeded => DockOperationProblemCauseV1::OverLimit,
        K::LifecycleRefused => DockOperationProblemCauseV1::Stale,
        K::RecordSubstituted => DockOperationProblemCauseV1::Substituted,
        K::IdempotencyConflict => DockOperationProblemCauseV1::Conflict,
        K::DurabilityUncertain => DockOperationProblemCauseV1::DurabilityUncertain,
        K::EvidenceCandidateMissing => DockOperationProblemCauseV1::Missing,
        K::EvidenceCandidateIneligible
        | K::EvidenceResolutionInsufficient
        | K::EvidenceSourceRefused
        | K::EvidenceSourceUnavailable => DockOperationProblemCauseV1::Stale,
        K::UnsafeFilesystem | K::RecoveryAmbiguous => {
            DockOperationProblemCauseV1::InternalInvariant
        }
    }
}

fn problem_response(
    request: &OperationRequestV1,
    definition: &OperationDefinition,
    cause: DockOperationProblemCauseV1,
) -> Result<OperationResponseV1, ContractError> {
    let limits = ContractLimits::packet0();
    let projection = cause_problem(cause);
    let details = DockOperationDetailsDtoV1 {
        cause,
        related_refs: BoundedVec::new(Vec::new())?,
        diagnostics: BoundedVec::new(Vec::new())?,
    };
    let code = format!("dock.{}", cause_name(cause));
    let problem = Problem::new(
        BoundedString::new(code.clone())?,
        BoundedString::new(code)?,
        projection.category,
        definition.binding(),
        Some(definition.binding()),
        schema_binding(DOCK_OPERATION_DETAILS_SCHEMA_REF)?,
        super::artifact_operation::to_contract_value(&details)?,
        BoundedVec::new(Vec::new())?,
        projection.retry,
        None,
        None,
        &limits,
    )?;
    let binding = ProblemBinding::new(
        problem.problem_id().clone(),
        problem.problem_fingerprint().clone(),
    )?;
    let (refusals, errors) = if projection.status == TerminalStatus::Error {
        (
            BoundedVec::new(Vec::new())?,
            BoundedVec::new(vec![problem])?,
        )
    } else {
        (
            BoundedVec::new(vec![problem])?,
            BoundedVec::new(Vec::new())?,
        )
    };
    let idempotency = if definition.class() == OperationClass::ReadOnly {
        super::IdempotencyState::not_applicable()?
    } else {
        super::IdempotencyState::not_established(projection.stage, binding)?
    };
    OperationResponseV1::new(
        schema_binding("handbook.operation-response@1.0.0")?,
        request.request_id().cloned(),
        request.request_fingerprint().clone(),
        definition,
        TerminalFields::new(
            projection.status,
            None,
            BoundedVec::new(Vec::new())?,
            refusals,
            errors,
        )?,
        idempotency,
        BoundedVec::new(Vec::new())?,
        BoundedVec::new(Vec::new())?,
        BoundedVec::new(Vec::new())?,
        &DockOperationSchemaValidator,
        &limits,
    )
}

#[derive(Clone, Copy)]
struct DockProblemProjectionV1 {
    status: TerminalStatus,
    stage: super::IdempotencyStage,
    category: ProblemCategory,
    retry: RetryDirective,
}

fn cause_problem(cause: DockOperationProblemCauseV1) -> DockProblemProjectionV1 {
    use DockOperationProblemCauseV1 as C;
    let (status, stage, category, retry) = match cause {
        C::Malformed | C::OverLimit | C::Missing | C::Unknown => (
            TerminalStatus::Refused,
            super::IdempotencyStage::RequestValidation,
            ProblemCategory::Schema,
            RetryDirective::AfterRequestChange,
        ),
        C::Stale | C::ManifestUnavailable => (
            TerminalStatus::Refused,
            super::IdempotencyStage::PreconditionValidation,
            ProblemCategory::Precondition,
            RetryDirective::AfterAuthorityChange,
        ),
        C::Substituted | C::UnsupportedTarget | C::LaunchRefused => (
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
        C::TimedOut | C::Cancelled | C::ProcessFailed | C::ProtocolError => (
            TerminalStatus::Error,
            super::IdempotencyStage::ExecutionStart,
            ProblemCategory::Implementation,
            RetryDirective::Transient,
        ),
        C::CleanupUncertain | C::DurabilityUncertain => (
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
    DockProblemProjectionV1 {
        status,
        stage,
        category,
        retry,
    }
}

fn cause_name(cause: DockOperationProblemCauseV1) -> &'static str {
    use DockOperationProblemCauseV1 as C;
    match cause {
        C::Malformed => "malformed",
        C::OverLimit => "over_limit",
        C::Missing => "missing",
        C::Unknown => "unknown",
        C::Stale => "stale",
        C::Substituted => "substituted",
        C::Conflict => "conflict",
        C::UnsupportedTarget => "unsupported_target",
        C::ManifestUnavailable => "manifest_unavailable",
        C::LaunchRefused => "launch_refused",
        C::TimedOut => "timed_out",
        C::Cancelled => "cancelled",
        C::ProcessFailed => "process_failed",
        C::ProtocolError => "protocol_error",
        C::CleanupUncertain => "cleanup_uncertain",
        C::DurabilityUncertain => "durability_uncertain",
        C::InternalInvariant => "internal_invariant",
    }
}

fn execute_manifest_list(
    request: &OperationRequestV1,
    definition: &OperationDefinition,
) -> Result<OperationResponseV1, ContractError> {
    let body: DockManifestListRequestV1 = from_body(request)?;
    body.validate()?;
    let owner_request = match (&body.root, &body.cursor) {
        (None, None) => handbook_dock::DockManifestListRequestV1::first(body.limit.get() as usize),
        (Some(root), Some(cursor)) => handbook_dock::DockManifestListRequestV1::continue_from(
            body.limit.get() as usize,
            OwnerFingerprint::parse(root.as_str()).map_err(|_| ContractError::InvalidInvariant)?,
            cursor.as_str(),
        ),
        _ => return Err(ContractError::InvalidInvariant),
    }
    .map_err(|_| ContractError::InvalidInvariant)?;
    let page = handbook_dock::DockManifestRegistryV1::production()
        .list(owner_request)
        .map_err(|_| ContractError::InvalidInvariant)?;
    let result = DockManifestListResultV1 {
        root: Fingerprint::parse(page.root().as_str())?,
        entries: BoundedVec::new(
            page.entries()
                .iter()
                .map(|entry| {
                    Ok(DockManifestListEntryDtoV1 {
                        manifest: record_binding(
                            entry.manifest_ref().as_str(),
                            entry.manifest_fingerprint().as_str(),
                        )?,
                    })
                })
                .collect::<Result<Vec<_>, ContractError>>()?,
        )?,
        next_cursor: page
            .next_cursor()
            .map(|cursor| BoundedString::new(cursor.to_owned()))
            .transpose()?,
        retention: DockManifestRetentionDtoV1::ImmutableExact,
    };
    success_response(request, definition, &result)
}

fn execute_manifest_read(
    request: &OperationRequestV1,
    definition: &OperationDefinition,
) -> Result<OperationResponseV1, ContractError> {
    let body: DockManifestReadRequestV1 = from_body(request)?;
    let owner_ref = OwnerExactRef::parse(body.manifest.reference())
        .map_err(|_| ContractError::InvalidInvariant)?;
    let owner_fingerprint = OwnerFingerprint::parse(body.manifest.fingerprint().as_str())
        .map_err(|_| ContractError::InvalidInvariant)?;
    let result = match handbook_dock::DockManifestRegistryV1::production()
        .read(&owner_ref, &owner_fingerprint)
        .map_err(|_| ContractError::InvalidInvariant)?
    {
        handbook_dock::DockManifestReadOutcomeV1::Found(manifest) => {
            DockManifestReadResultV1::Found {
                manifest: DockManifestDtoV1::new(
                    record_binding(
                        manifest.dock_ref().as_str(),
                        manifest.manifest_fingerprint().as_str(),
                    )?,
                    super::artifact_operation::to_contract_value(&manifest)?,
                ),
            }
        }
        handbook_dock::DockManifestReadOutcomeV1::Absent => DockManifestReadResultV1::Absent {
            requested: body.manifest,
        },
    };
    success_response(request, definition, &result)
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
        &DockOperationSchemaValidator,
        &ContractLimits::packet0(),
    )
}

fn mutation_success_response(
    request: &OperationRequestV1,
    definition: &OperationDefinition,
    scoped_key: Fingerprint,
    replayed: bool,
    result: &DockRunResultV1,
) -> Result<OperationResponseV1, ContractError> {
    let limits = ContractLimits::packet0();
    let data = super::artifact_operation::to_contract_value(result)?;
    let receipt = WriteReceipt::new(
        BoundedString::new("dock_execution_record".to_owned())?,
        ExactBinding::new(
            ExactRef::parse("handbook.dock-execution-record@1.0.0")?,
            result.execution_record.binding().fingerprint().clone(),
        ),
        BoundedString::new("dock_operational_execution".to_owned())?,
        data.clone(),
        BoundedString::new("dock_run".to_owned())?,
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
                &DockOperationSchemaValidator,
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
        &DockOperationSchemaValidator,
        &limits,
    )
}

fn record_binding(
    reference: &str,
    fingerprint: &str,
) -> Result<ContractRecordBindingDtoV1, ContractError> {
    ContractRecordBindingDtoV1::new(reference, Fingerprint::parse(fingerprint)?)
}

pub fn dock_operation_definitions() -> Result<Vec<OperationDefinition>, ContractError> {
    DOCK_OPERATION_REFS
        .into_iter()
        .map(dock_operation_definition)
        .collect()
}

pub fn dock_operation_definition(
    operation_ref: &str,
) -> Result<OperationDefinition, ContractError> {
    if !DOCK_OPERATION_REFS.contains(&operation_ref) {
        return Err(ContractError::InvalidInvariant);
    }
    let owner = handbook_dock::operation_admission::operation_owner_version_evidence();
    let result_schema = schema_binding(&result_schema_ref(operation_ref))?;
    let (class, idempotency, writes) = if operation_ref == DOCK_RUN_REF {
        (
            OperationClass::AppendOnly,
            OperationIdempotency::key_required(JsonPointer::parse("/body/idempotency_key")?)?,
            vec![DeclaredWrite::new(
                BoundedString::new("dock_execution_record".to_owned())?,
                BoundedString::new("dock_operational_execution".to_owned())?,
                BoundedString::new("dock_run".to_owned())?,
                result_schema.clone(),
            )?],
        )
    } else {
        (
            OperationClass::ReadOnly,
            OperationIdempotency::safe(),
            Vec::new(),
        )
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
        schema_binding(&request_schema_ref(operation_ref))?,
        result_schema,
        OperationOutcomeSchemas::new(
            schema_binding("handbook.operation-blocker@1.0.0")?,
            schema_binding("handbook.operation-refusal@1.0.0")?,
            schema_binding("handbook.operation-error@1.0.0")?,
        )?,
        BoundedVec::new(Vec::new())?,
        class,
        idempotency,
        BoundedVec::new(writes)?,
        BoundedVec::new(vec![
            OperationTransport::CliJson,
            OperationTransport::RustSdk,
            OperationTransport::Tauri,
        ])?,
        None,
        &ContractLimits::packet0(),
    )
}

pub(crate) fn current_dock_capabilities() -> Result<Vec<CapabilityEntry>, ContractError> {
    dock_operation_definitions()?
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
                &DockTransportEvidence { owner },
                &ContractLimits::packet0(),
            )
        })
        .collect()
}

struct DockTransportEvidence {
    owner: OwnerAdmissionToken,
}

impl CurrentTransportEvidenceSealed for DockTransportEvidence {}

impl CurrentTransportEvidence for DockTransportEvidence {
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
    use crate::contract_membrane::{IdempotencyStage, IdempotencyStateRef};

    fn fingerprint(byte: char) -> Fingerprint {
        Fingerprint::parse(&format!("sha256:{}", byte.to_string().repeat(64))).unwrap()
    }

    fn binding(reference: &str, byte: char) -> ContractRecordBindingDtoV1 {
        ContractRecordBindingDtoV1::new(reference, fingerprint(byte)).unwrap()
    }

    #[test]
    fn run_receipt_replay_and_conflict_projection_are_exact() {
        let body = DockRunRequestV1 {
            manifest: binding("dock.fixture@1.0.0", '1'),
            evaluation: DockEvaluationPreparationDtoV1 {
                contract: binding("contract.fixture@1.0.0", '2'),
                expected_lifecycle_fingerprint: fingerprint('3'),
                subject: ContractValue::null(),
                cases: BoundedVec::new(Vec::new()).unwrap(),
                requested_resolution: ContractValue::null(),
                matcher: ContractValue::null(),
                freshness_policies: BoundedVec::new(Vec::new()).unwrap(),
                stability_policies: BoundedVec::new(Vec::new()).unwrap(),
                gate_policy: binding("policy.gate@1.0.0", '4'),
                local_closeout_policy: binding("policy.closeout@1.0.0", '5'),
                nonce: BoundedString::new("packet4-nonce".to_owned()).unwrap(),
            },
            requested_evidence_kinds: BoundedVec::new(Vec::new()).unwrap(),
            inputs: BoundedVec::new(Vec::new()).unwrap(),
            limits: DockProtocolLimitsDtoV1 {
                request_bytes: SafeU64::new(1).unwrap(),
                result_bytes: SafeU64::new(1).unwrap(),
                stderr_bytes: SafeU64::new(1).unwrap(),
                diagnostic_count: SafeU64::new(1).unwrap(),
                diagnostic_bytes: SafeU64::new(1).unwrap(),
                candidate_count: SafeU64::new(1).unwrap(),
                candidate_bytes: SafeU64::new(1).unwrap(),
                input_count: SafeU64::new(1).unwrap(),
                wall_timeout_ms: SafeU64::new(1).unwrap(),
                cancellation_grace_ms: SafeU64::new(1).unwrap(),
            },
        };
        let request =
            dock_run_operation_request(Some("packet4-run-replay"), "packet4-secret-key", body)
                .unwrap();
        assert!(from_body::<DockRunRequestV1>(&request).is_ok());
        assert!(from_body::<DockRunWireV1>(&request).is_err());
        let definition = dock_operation_definition(DOCK_RUN_REF).unwrap();
        let result = DockRunResultV1 {
            evaluation_run: ContractEvaluationRunDtoV1::new(
                binding("evaluation.fixture@1.0.0", '6'),
                ContractValue::null(),
            ),
            execution_record: DockExecutionRecordDtoV1::new(
                binding("execution.fixture@1.0.0", '7'),
                ContractValue::null(),
            ),
            candidates: BoundedVec::new(Vec::new()).unwrap(),
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
        assert_eq!(committed.receipts().len(), 1);
        assert_eq!(
            committed.receipts()[0].record_kind().as_str(),
            "dock_execution_record"
        );
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
        let bytes = canonical_json_bytes(&replayed, &ContractLimits::packet0()).unwrap();
        assert!(!String::from_utf8_lossy(&bytes).contains("packet4-secret-key"));
        assert_eq!(
            OperationResponseV1::parse_and_admit(
                &bytes,
                &definition,
                &DockOperationSchemaValidator,
                &ContractLimits::packet0(),
            )
            .unwrap(),
            replayed
        );
        assert_eq!(
            executor_problem(handbook_dock::DockExecutorErrorKindV1::IdempotencyConflict),
            DockOperationProblemCauseV1::Conflict
        );
        let conflict = cause_problem(DockOperationProblemCauseV1::Conflict);
        assert_eq!(conflict.status, TerminalStatus::Refused);
        assert_eq!(conflict.stage, IdempotencyStage::IdempotencyValidation);
        assert_eq!(conflict.category, ProblemCategory::Idempotency);
        let durability = cause_problem(DockOperationProblemCauseV1::DurabilityUncertain);
        assert_eq!(durability.status, TerminalStatus::Error);
        assert_eq!(durability.stage, IdempotencyStage::ExecutionStart);
        assert_eq!(durability.category, ProblemCategory::Safety);
        let invariant = cause_problem(DockOperationProblemCauseV1::InternalInvariant);
        assert_eq!(invariant.status, TerminalStatus::Error);
        assert_eq!(invariant.category, ProblemCategory::Implementation);
    }
}
