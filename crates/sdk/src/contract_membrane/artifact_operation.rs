use super::{
    canonical_json_bytes, parse_canonical_json, sha256_fingerprint, BoundedString, BoundedVec,
    CapabilityEntry, ContractError, ContractLimits, ContractValue, CorrelationRef,
    CurrentTransportEvidence, CurrentTransportEvidenceSealed, DeclaredWrite, ExactBinding,
    ExactRef, Fingerprint, IdempotencyKeyConsumer, IdempotencyKeyConsumerSealed, IdempotencyStage,
    IdempotencyState, JsonPointer, OperationAdmissionResult, OperationClass, OperationDefinition,
    OperationIdempotency, OperationOutcomeSchemas, OperationOwnerBinding, OperationRequestV1,
    OperationResponseV1, OperationSchemaValidator, OperationTransport, OwnerAdmissionToken,
    Problem, ProblemBinding, ProblemCategory, RecheckCondition, RequestId, RetryDirective, SemVer,
    TerminalFields, TerminalStatus, UtcTimestamp, WriteReceipt,
};
use crate::artifact::{ArtifactAcquisitionMode, ArtifactDocument, ArtifactInputDocument};
use serde::{Deserialize, Serialize};
use std::path::Path;

pub(crate) const ARTIFACT_DETAILS_SCHEMA_REF: &str = "handbook.artifact-operation-details@1.0.0";
pub(crate) const REPOSITORY_RECHECK_SCHEMA_REF: &str =
    "handbook.artifact-recheck.repository-authority@1.0.0";
pub(crate) const PLATFORM_RECHECK_SCHEMA_REF: &str =
    "handbook.artifact-recheck.supported-platform@1.0.0";
pub(crate) const CANONICAL_CONDITION_SCHEMA_REF: &str =
    "handbook.artifact-write-condition.canonical-truth@1.0.0";
pub(crate) const SEMANTIC_CONDITION_SCHEMA_REF: &str =
    "handbook.artifact-write-condition.semantic-record@1.0.0";

pub(crate) const ARTIFACT_OPERATION_REFS: [&str; 10] = [
    "artifact.kind.list@1.0.0",
    "artifact.instance.list@1.0.0",
    "artifact.read@1.0.0",
    "artifact.validate@1.0.0",
    "intake.definition.read@1.0.0",
    "intake.coverage.evaluate@1.0.0",
    "intake.record.append@1.0.0",
    "artifact.candidate.validate@1.0.0",
    "artifact.candidate.append@1.0.0",
    "artifact.candidate.promote@1.0.0",
];

#[derive(Clone, Debug)]
pub enum ArtifactOperationInputV1 {
    KindList,
    InstanceList,
    Read {
        kind_ref: String,
        instance_id: String,
    },
    Validate {
        kind_ref: String,
        instance_id: String,
    },
    IntakeDefinitionRead {
        kind_ref: String,
        instance_id: String,
    },
    IntakeCoverageEvaluate {
        kind_ref: String,
        instance_id: String,
        mode: ArtifactAcquisitionMode,
        expected_current_artifact_fingerprint: Option<String>,
        input: ArtifactInputDocument,
    },
    IntakeRecordAppend {
        kind_ref: String,
        instance_id: String,
        input: ArtifactInputDocument,
    },
    CandidateValidate {
        kind_ref: String,
        instance_id: String,
        intake_record_ref: String,
        intake_record_fingerprint: String,
        expected_current_artifact_fingerprint: Option<String>,
    },
    CandidateAppend {
        kind_ref: String,
        instance_id: String,
        input: ArtifactInputDocument,
    },
    CandidatePromote {
        kind_ref: String,
        instance_id: String,
        input: ArtifactInputDocument,
    },
}

pub(crate) fn request_schema_ref(operation_ref: &str) -> String {
    let operation = operation_ref
        .strip_suffix("@1.0.0")
        .unwrap_or(operation_ref);
    format!(
        "handbook.operation.{}-request@1.0.0",
        operation.replace('.', "-")
    )
}

pub(crate) fn result_schema_ref(operation_ref: &str) -> String {
    let operation = operation_ref
        .strip_suffix("@1.0.0")
        .unwrap_or(operation_ref);
    format!(
        "handbook.operation.{}-result@1.0.0",
        operation.replace('.', "-")
    )
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct EmptyRequestV1 {}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct TargetRequestV1 {
    pub kind_ref: String,
    pub instance_id: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct CoverageSubmissionDtoV1 {
    pub coverage_id: String,
    pub state: String,
    pub source_kind: String,
    pub value: Option<ArtifactDocument>,
    pub specificity: String,
    pub confidence: String,
    pub contradiction_refs: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct IntakeEvaluateRequestV1 {
    pub kind_ref: String,
    pub instance_id: String,
    pub acquisition_mode: String,
    pub expected_current_artifact_fingerprint: Option<String>,
    pub coverage_submissions: Vec<CoverageSubmissionDtoV1>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct IntakeAppendRequestV1 {
    pub kind_ref: String,
    pub instance_id: String,
    pub acquisition_mode: String,
    pub expected_current_artifact_fingerprint: Option<String>,
    pub coverage_submissions: Vec<CoverageSubmissionDtoV1>,
    pub idempotency_key: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct CandidateValidateRequestV1 {
    pub kind_ref: String,
    pub instance_id: String,
    pub intake_record_ref: String,
    pub intake_record_fingerprint: String,
    pub expected_current_artifact_fingerprint: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct CandidateAppendRequestV1 {
    pub kind_ref: String,
    pub instance_id: String,
    pub intake_record_ref: String,
    pub intake_record_fingerprint: String,
    pub expected_candidate_fingerprint: String,
    pub idempotency_key: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PromoteRequestV1 {
    pub kind_ref: String,
    pub instance_id: String,
    pub candidate_ref: String,
    pub candidate_fingerprint: String,
    pub expected_current_artifact_fingerprint: Option<String>,
    pub publisher_authority: Option<ArtifactDocument>,
    pub idempotency_key: String,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct CoverageInputDocumentV1 {
    coverage_submissions: Vec<CoverageSubmissionDtoV1>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct IntakeAppendInputDocumentV1 {
    idempotency_key: String,
    acquisition_mode: String,
    expected_current_artifact_fingerprint: Option<String>,
    coverage_submissions: Vec<CoverageSubmissionDtoV1>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CandidateAppendInputDocumentV1 {
    idempotency_key: String,
    intake_record_ref: String,
    intake_record_fingerprint: String,
    expected_candidate_fingerprint: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct PromoteInputDocumentV1 {
    idempotency_key: String,
    candidate_ref: String,
    candidate_fingerprint: String,
    expected_current_artifact_fingerprint: Option<String>,
    publisher_authority: Option<ArtifactDocument>,
}

#[derive(Serialize)]
#[serde(deny_unknown_fields)]
struct IntakeOwnerRequestV1<'a> {
    idempotency_key: &'a str,
    acquisition_mode: &'a str,
    expected_current_artifact_fingerprint: &'a Option<String>,
    coverage_submissions: &'a [CoverageSubmissionDtoV1],
}

#[derive(Serialize)]
#[serde(deny_unknown_fields)]
struct CandidateAppendOwnerRequestV1<'a> {
    idempotency_key: &'a str,
    intake_record_ref: &'a str,
    intake_record_fingerprint: &'a str,
    expected_candidate_fingerprint: &'a str,
}

#[derive(Serialize)]
#[serde(deny_unknown_fields)]
struct PromoteOwnerRequestV1<'a> {
    idempotency_key: &'a str,
    candidate_ref: &'a str,
    candidate_fingerprint: &'a str,
    expected_current_artifact_fingerprint: &'a Option<String>,
    publisher_authority: &'a Option<ArtifactDocument>,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct IntakeAppendSanitizedV1 {
    kind_ref: String,
    instance_id: String,
    acquisition_mode: String,
    expected_current_artifact_fingerprint: Option<String>,
    coverage_submissions: Vec<CoverageSubmissionDtoV1>,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct CandidateAppendSanitizedV1 {
    kind_ref: String,
    instance_id: String,
    intake_record_ref: String,
    intake_record_fingerprint: String,
    expected_candidate_fingerprint: String,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct PromoteSanitizedV1 {
    kind_ref: String,
    instance_id: String,
    candidate_ref: String,
    candidate_fingerprint: String,
    expected_current_artifact_fingerprint: Option<String>,
    publisher_authority: Option<ArtifactDocument>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ArtifactKindDtoV1 {
    pub kind_ref: String,
    pub definition_fingerprint: String,
    pub schema_ref: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct KindListResultV1 {
    pub operation_id: String,
    pub selection_fingerprint: String,
    pub kinds: Vec<ArtifactKindDtoV1>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ArtifactInstanceDtoV1 {
    pub instance_id: String,
    pub kind_ref: String,
    pub canonical_path: String,
    pub intake_definition_ref: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct InstanceListResultV1 {
    pub operation_id: String,
    pub selection_fingerprint: String,
    pub instances: Vec<ArtifactInstanceDtoV1>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ReadResultV1 {
    pub operation_id: String,
    pub operation_context_fingerprint: String,
    pub kind_ref: String,
    pub instance_id: String,
    pub canonical_path: String,
    pub artifact_fingerprint: String,
    pub content: ArtifactDocument,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ValidationLayersV1 {
    pub structural: String,
    pub semantic: String,
    pub intake: String,
    pub approval: String,
    pub external_evidence: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ValidateResultV1 {
    pub operation_id: String,
    pub operation_context_fingerprint: String,
    pub artifact_fingerprint: String,
    pub outcome: String,
    pub content: ArtifactDocument,
    pub layers: ValidationLayersV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct CoverageDefinitionDtoV1 {
    pub coverage_id: String,
    pub target_paths: Vec<String>,
    pub minimum_specificity: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct IntakeDefinitionResultV1 {
    pub operation_id: String,
    pub kind_ref: String,
    pub instance_id: String,
    pub intake_definition_ref: String,
    pub intake_definition_fingerprint: String,
    pub candidate_schema_ref: String,
    pub coverage: Vec<CoverageDefinitionDtoV1>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct IntakeEvaluationResultV1 {
    pub schema_id: String,
    pub schema_version: String,
    pub operation_context_fingerprint: String,
    pub acquisition_mode: String,
    pub basis_artifact_fingerprint: Option<String>,
    pub coverage_results: Vec<ArtifactDocument>,
    pub normalized_content: ArtifactDocument,
    pub field_sources: Vec<ArtifactDocument>,
    pub outcome: String,
    pub evaluation_fingerprint: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct CandidateValidationResultV1 {
    pub operation_id: String,
    pub outcome: String,
    pub operation_context_fingerprint: String,
    pub intake_record_ref: String,
    pub intake_record_fingerprint: String,
    pub basis_artifact_fingerprint: Option<String>,
    pub normalized_content: ArtifactDocument,
    pub normalized_content_ref: String,
    pub normalized_content_fingerprint: String,
    pub validation_result: ArtifactDocument,
    pub validation_result_ref: String,
    pub validation_result_fingerprint: String,
    pub candidate: ArtifactDocument,
    pub candidate_ref: String,
    pub candidate_fingerprint: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ArtifactMutationRefusalV1 {
    pub code: String,
    pub layer: String,
    pub expected_fingerprint: Option<String>,
    pub observed_fingerprint: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ArtifactAuthoritativeOutputV1 {
    pub authority_class: String,
    #[serde(rename = "ref")]
    pub reference: String,
    pub fingerprint: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ArtifactMutationResultDataV1 {
    pub schema_id: String,
    pub schema_version: String,
    pub operation_id: String,
    pub transaction_id: String,
    pub request_fingerprint: String,
    pub outcome: String,
    pub refusal: Option<ArtifactMutationRefusalV1>,
    pub authoritative_outputs: Vec<ArtifactAuthoritativeOutputV1>,
    pub internal_transaction_evidence_ref: String,
    pub internal_transaction_evidence_fingerprint: String,
    pub result_fingerprint: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum ArtifactProblemCauseV1 {
    SelectionUnavailable,
    ProfileUnavailable,
    RegistryUnavailable,
    IntakeSourceUnavailable,
    RepositoryIdentityUnavailable,
    ArtifactUnavailable,
    UnsupportedPlatform,
    SelectionAdmission,
    IntakeAdmission,
    CanonicalSyntax,
    StructuralValidation,
    InvalidRequest,
    TargetResolution,
    NotApplicable,
    StaleOperationContext,
    StaleCurrentArtifact,
    StaleBasis,
    IdempotencyConflict,
    IdempotencyExpired,
    UnsafeReference,
    RequestBoundExceeded,
    UnsafeFilesystem,
    RetainedStateBoundExceeded,
    Io,
    InvalidRetainedState,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ArtifactProblemDetailsV1 {
    pub operation_id: String,
    pub cause: ArtifactProblemCauseV1,
    pub detail: String,
    pub mutation_result: Option<ArtifactMutationResultDataV1>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ArtifactRecheckV1 {
    pub operation_id: String,
    pub cause: ArtifactProblemCauseV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ArtifactWriteConditionV1 {
    pub operation_id: String,
    pub transaction_id: String,
    pub authority_class: String,
    #[serde(rename = "ref")]
    pub reference: String,
    pub fingerprint: String,
}

pub(crate) struct ArtifactSchemaValidator;

impl OperationSchemaValidator for ArtifactSchemaValidator {
    fn validate_instance(
        &self,
        schema: &ExactBinding,
        instance: &ContractValue,
    ) -> Result<(), ContractError> {
        let limits = ContractLimits::packet0();
        let bytes = canonical_json_bytes(instance, &limits)?;
        let reference = schema.reference().as_str();
        validate_checked_schema(schema, &bytes, &limits)?;
        match reference {
            "handbook.operation.artifact-kind-list-request@1.0.0"
            | "handbook.operation.artifact-instance-list-request@1.0.0" => {
                parse_canonical_json::<EmptyRequestV1>(&bytes, &limits).map(|_| ())
            }
            "handbook.operation.artifact-read-request@1.0.0"
            | "handbook.operation.artifact-validate-request@1.0.0"
            | "handbook.operation.intake-definition-read-request@1.0.0" => {
                parse_canonical_json::<TargetRequestV1>(&bytes, &limits).map(|_| ())
            }
            "handbook.operation.intake-coverage-evaluate-request@1.0.0" => {
                parse_canonical_json::<IntakeEvaluateRequestV1>(&bytes, &limits).map(|_| ())
            }
            "handbook.operation.intake-record-append-request@1.0.0" => {
                parse_canonical_json::<IntakeAppendRequestV1>(&bytes, &limits).map(|_| ())
            }
            "handbook.operation.artifact-candidate-validate-request@1.0.0" => {
                parse_canonical_json::<CandidateValidateRequestV1>(&bytes, &limits).map(|_| ())
            }
            "handbook.operation.artifact-candidate-append-request@1.0.0" => {
                parse_canonical_json::<CandidateAppendRequestV1>(&bytes, &limits).map(|_| ())
            }
            "handbook.operation.artifact-candidate-promote-request@1.0.0" => {
                parse_canonical_json::<PromoteRequestV1>(&bytes, &limits).map(|_| ())
            }
            "handbook.operation.artifact-kind-list-result@1.0.0" => {
                parse_canonical_json::<KindListResultV1>(&bytes, &limits).map(|_| ())
            }
            "handbook.operation.artifact-instance-list-result@1.0.0" => {
                parse_canonical_json::<InstanceListResultV1>(&bytes, &limits).map(|_| ())
            }
            "handbook.operation.artifact-read-result@1.0.0" => {
                parse_canonical_json::<ReadResultV1>(&bytes, &limits).map(|_| ())
            }
            "handbook.operation.artifact-validate-result@1.0.0" => {
                parse_canonical_json::<ValidateResultV1>(&bytes, &limits).map(|_| ())
            }
            "handbook.operation.intake-definition-read-result@1.0.0" => {
                parse_canonical_json::<IntakeDefinitionResultV1>(&bytes, &limits).map(|_| ())
            }
            "handbook.operation.intake-coverage-evaluate-result@1.0.0" => {
                parse_canonical_json::<IntakeEvaluationResultV1>(&bytes, &limits).map(|_| ())
            }
            "handbook.operation.artifact-candidate-validate-result@1.0.0" => {
                parse_canonical_json::<CandidateValidationResultV1>(&bytes, &limits).map(|_| ())
            }
            "handbook.operation.intake-record-append-result@1.0.0" => validate_mutation_result(
                &bytes,
                "intake.record.append",
                &["semantic_record"],
                &limits,
            ),
            "handbook.operation.artifact-candidate-append-result@1.0.0" => {
                validate_mutation_result(
                    &bytes,
                    "artifact.candidate.append",
                    &["semantic_record"],
                    &limits,
                )
            }
            "handbook.operation.artifact-candidate-promote-result@1.0.0" => {
                validate_mutation_result(
                    &bytes,
                    "artifact.candidate.promote",
                    &["canonical_truth", "semantic_record"],
                    &limits,
                )
            }
            ARTIFACT_DETAILS_SCHEMA_REF => {
                parse_canonical_json::<ArtifactProblemDetailsV1>(&bytes, &limits).map(|_| ())
            }
            REPOSITORY_RECHECK_SCHEMA_REF => {
                let recheck = parse_canonical_json::<ArtifactRecheckV1>(&bytes, &limits)?;
                if matches!(
                    recheck.cause,
                    ArtifactProblemCauseV1::SelectionUnavailable
                        | ArtifactProblemCauseV1::ProfileUnavailable
                        | ArtifactProblemCauseV1::RegistryUnavailable
                        | ArtifactProblemCauseV1::IntakeSourceUnavailable
                        | ArtifactProblemCauseV1::RepositoryIdentityUnavailable
                        | ArtifactProblemCauseV1::ArtifactUnavailable
                ) {
                    Ok(())
                } else {
                    Err(ContractError::InvalidInvariant)
                }
            }
            PLATFORM_RECHECK_SCHEMA_REF => {
                let recheck = parse_canonical_json::<ArtifactRecheckV1>(&bytes, &limits)?;
                if recheck.cause == ArtifactProblemCauseV1::UnsupportedPlatform {
                    Ok(())
                } else {
                    Err(ContractError::InvalidInvariant)
                }
            }
            CANONICAL_CONDITION_SCHEMA_REF => {
                let condition = parse_canonical_json::<ArtifactWriteConditionV1>(&bytes, &limits)?;
                validate_write_condition(&condition, "canonical_truth")
            }
            SEMANTIC_CONDITION_SCHEMA_REF => {
                let condition = parse_canonical_json::<ArtifactWriteConditionV1>(&bytes, &limits)?;
                validate_write_condition(&condition, "semantic_record")
            }
            _ => Err(ContractError::InvalidInvariant),
        }
    }
}

fn validate_checked_schema(
    binding: &ExactBinding,
    instance_bytes: &[u8],
    _limits: &ContractLimits,
) -> Result<(), ContractError> {
    let schema_source = artifact_schema_source(binding.reference().as_str())
        .ok_or(ContractError::InvalidInvariant)?;
    if binding.fingerprint() != &sha256_fingerprint(schema_source.as_bytes()) {
        return Err(ContractError::InvalidInvariant);
    }
    let schema = serde_json::from_str::<serde_json::Value>(schema_source)
        .map_err(|_| ContractError::InvalidJson)?;
    let instance = serde_json::from_slice::<serde_json::Value>(instance_bytes)
        .map_err(|_| ContractError::InvalidJson)?;
    let validator = jsonschema::draft202012::options()
        .with_pattern_options(jsonschema::PatternOptions::fancy_regex())
        .build(&schema)
        .map_err(|_| ContractError::InvalidInvariant)?;
    if validator.is_valid(&instance) {
        Ok(())
    } else {
        Err(ContractError::InvalidInvariant)
    }
}

fn artifact_schema_source(reference: &str) -> Option<&'static str> {
    macro_rules! schema {
        ($directory:literal) => {
            include_str!(concat!(
                "../../schemas/2020-12/",
                $directory,
                "/1.0.0.schema.json"
            ))
        };
    }
    Some(match reference {
        "handbook.operation.artifact-kind-list-request@1.0.0" => {
            schema!("handbook.operation.artifact-kind-list-request")
        }
        "handbook.operation.artifact-kind-list-result@1.0.0" => {
            schema!("handbook.operation.artifact-kind-list-result")
        }
        "handbook.operation.artifact-instance-list-request@1.0.0" => {
            schema!("handbook.operation.artifact-instance-list-request")
        }
        "handbook.operation.artifact-instance-list-result@1.0.0" => {
            schema!("handbook.operation.artifact-instance-list-result")
        }
        "handbook.operation.artifact-read-request@1.0.0" => {
            schema!("handbook.operation.artifact-read-request")
        }
        "handbook.operation.artifact-read-result@1.0.0" => {
            schema!("handbook.operation.artifact-read-result")
        }
        "handbook.operation.artifact-validate-request@1.0.0" => {
            schema!("handbook.operation.artifact-validate-request")
        }
        "handbook.operation.artifact-validate-result@1.0.0" => {
            schema!("handbook.operation.artifact-validate-result")
        }
        "handbook.operation.intake-definition-read-request@1.0.0" => {
            schema!("handbook.operation.intake-definition-read-request")
        }
        "handbook.operation.intake-definition-read-result@1.0.0" => {
            schema!("handbook.operation.intake-definition-read-result")
        }
        "handbook.operation.intake-coverage-evaluate-request@1.0.0" => {
            schema!("handbook.operation.intake-coverage-evaluate-request")
        }
        "handbook.operation.intake-coverage-evaluate-result@1.0.0" => {
            schema!("handbook.operation.intake-coverage-evaluate-result")
        }
        "handbook.operation.intake-record-append-request@1.0.0" => {
            schema!("handbook.operation.intake-record-append-request")
        }
        "handbook.operation.intake-record-append-result@1.0.0" => {
            schema!("handbook.operation.intake-record-append-result")
        }
        "handbook.operation.artifact-candidate-validate-request@1.0.0" => {
            schema!("handbook.operation.artifact-candidate-validate-request")
        }
        "handbook.operation.artifact-candidate-validate-result@1.0.0" => {
            schema!("handbook.operation.artifact-candidate-validate-result")
        }
        "handbook.operation.artifact-candidate-append-request@1.0.0" => {
            schema!("handbook.operation.artifact-candidate-append-request")
        }
        "handbook.operation.artifact-candidate-append-result@1.0.0" => {
            schema!("handbook.operation.artifact-candidate-append-result")
        }
        "handbook.operation.artifact-candidate-promote-request@1.0.0" => {
            schema!("handbook.operation.artifact-candidate-promote-request")
        }
        "handbook.operation.artifact-candidate-promote-result@1.0.0" => {
            schema!("handbook.operation.artifact-candidate-promote-result")
        }
        ARTIFACT_DETAILS_SCHEMA_REF => schema!("handbook.artifact-operation-details"),
        REPOSITORY_RECHECK_SCHEMA_REF => {
            schema!("handbook.artifact-recheck.repository-authority")
        }
        PLATFORM_RECHECK_SCHEMA_REF => schema!("handbook.artifact-recheck.supported-platform"),
        CANONICAL_CONDITION_SCHEMA_REF => {
            schema!("handbook.artifact-write-condition.canonical-truth")
        }
        SEMANTIC_CONDITION_SCHEMA_REF => {
            schema!("handbook.artifact-write-condition.semantic-record")
        }
        _ => return None,
    })
}

fn validate_mutation_result(
    bytes: &[u8],
    operation_id: &str,
    expected_authority_classes: &[&str],
    limits: &ContractLimits,
) -> Result<(), ContractError> {
    let result = parse_canonical_json::<ArtifactMutationResultDataV1>(bytes, limits)?;
    let fingerprints_are_valid = Fingerprint::parse(&result.request_fingerprint).is_ok()
        && Fingerprint::parse(&result.internal_transaction_evidence_fingerprint).is_ok()
        && Fingerprint::parse(&result.result_fingerprint).is_ok()
        && result.authoritative_outputs.iter().all(|output| {
            Fingerprint::parse(&output.fingerprint).is_ok()
                && !output.reference.is_empty()
                && output.reference.len() <= 1024
        });
    let refusal_is_valid = result.refusal.as_ref().is_none_or(|refusal| {
        refusal
            .expected_fingerprint
            .as_deref()
            .is_none_or(|value| Fingerprint::parse(value).is_ok())
            && refusal
                .observed_fingerprint
                .as_deref()
                .is_none_or(|value| Fingerprint::parse(value).is_ok())
            && matches!(
                (refusal.code.as_str(), refusal.layer.as_str()),
                ("canonical_syntax_invalid", "canonical_syntax")
                    | ("structural_validation_failed", "structural")
                    | ("intake_coverage_blocked", "intake")
                    | ("stale_basis", "currentness")
                    | ("stale_current_artifact", "currentness")
                    | ("operation_ineligible", "eligibility")
                    | ("publication_basis_conflict", "publication")
            )
    });
    let outcome_is_valid = match result.outcome.as_str() {
        "committed" => {
            result.refusal.is_none()
                && result.authoritative_outputs.len() == expected_authority_classes.len()
                && result
                    .authoritative_outputs
                    .iter()
                    .zip(expected_authority_classes)
                    .all(|(output, expected)| output.authority_class == *expected)
        }
        "refused" => result.refusal.is_some() && result.authoritative_outputs.is_empty(),
        _ => false,
    };
    if result.schema_id == "handbook.generic-domain-mutation-result"
        && result.schema_version == "1.0"
        && result.operation_id == operation_id
        && !result.transaction_id.is_empty()
        && result.transaction_id.len() <= 255
        && !result.internal_transaction_evidence_ref.is_empty()
        && result.internal_transaction_evidence_ref.len() <= 1024
        && fingerprints_are_valid
        && refusal_is_valid
        && outcome_is_valid
    {
        Ok(())
    } else {
        Err(ContractError::InvalidInvariant)
    }
}

fn validate_write_condition(
    condition: &ArtifactWriteConditionV1,
    expected_authority_class: &str,
) -> Result<(), ContractError> {
    if !condition.operation_id.is_empty()
        && condition.operation_id.len() <= 255
        && !condition.transaction_id.is_empty()
        && condition.transaction_id.len() <= 255
        && condition.authority_class == expected_authority_class
        && !condition.reference.is_empty()
        && condition.reference.len() <= 1024
        && Fingerprint::parse(&condition.fingerprint).is_ok()
    {
        Ok(())
    } else {
        Err(ContractError::InvalidInvariant)
    }
}

pub fn artifact_operation_definitions() -> Result<Vec<OperationDefinition>, ContractError> {
    ARTIFACT_OPERATION_REFS
        .into_iter()
        .map(artifact_operation_definition)
        .collect()
}

pub(crate) fn current_artifact_capabilities() -> Result<Vec<CapabilityEntry>, ContractError> {
    artifact_operation_definitions()?
        .into_iter()
        .map(|definition| {
            let owner = match super::admit_operation_definition(&definition)? {
                OperationAdmissionResult::Admitted(owner) => owner,
                OperationAdmissionResult::Omitted(_) => {
                    return Err(ContractError::InvalidInvariant)
                }
            };
            let current_transports: &'static [OperationTransport] =
                &[OperationTransport::CliJson, OperationTransport::Tauri];
            let evidence = ArtifactCliTransportEvidence {
                owner,
                current_transports,
            };
            CapabilityEntry::new(&definition, &evidence, &ContractLimits::packet0())
        })
        .collect()
}

struct ArtifactCliTransportEvidence {
    owner: OwnerAdmissionToken,
    current_transports: &'static [OperationTransport],
}

impl CurrentTransportEvidenceSealed for ArtifactCliTransportEvidence {}

impl CurrentTransportEvidence for ArtifactCliTransportEvidence {
    fn owner_admission(&self) -> &OwnerAdmissionToken {
        &self.owner
    }

    fn current_transports(&self) -> &[OperationTransport] {
        self.current_transports
    }
}

pub fn artifact_operation_definition(
    operation_ref: &str,
) -> Result<OperationDefinition, ContractError> {
    if !ARTIFACT_OPERATION_REFS.contains(&operation_ref) {
        return Err(ContractError::InvalidInvariant);
    }
    let limits = ContractLimits::packet0();
    let owner = handbook_engine::operation_admission::operation_owner_version_evidence();
    let (class, idempotency, write_set) = definition_behavior(operation_ref)?;
    OperationDefinition::new(
        schema_binding("handbook.operation-definition@1.0.0")?,
        ExactRef::parse(operation_ref)?,
        OperationOwnerBinding::new(
            BoundedString::new(owner.owner_crate.to_owned())?,
            SemVer::parse(owner.owner_version)?,
            ExactBinding::new(
                ExactRef::parse(owner.owner_source_ref)?,
                sha256_fingerprint(owner.source_bytes),
            ),
        )?,
        schema_binding(&request_schema_ref(operation_ref))?,
        schema_binding(&result_schema_ref(operation_ref))?,
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
        &limits,
    )
}

pub fn artifact_operation_request(
    request_id: Option<&str>,
    input: ArtifactOperationInputV1,
) -> Result<OperationRequestV1, ContractError> {
    let (operation_ref, wire_body) = match input {
        ArtifactOperationInputV1::KindList => (
            "artifact.kind.list@1.0.0",
            to_contract_value(&EmptyRequestV1 {})?,
        ),
        ArtifactOperationInputV1::InstanceList => (
            "artifact.instance.list@1.0.0",
            to_contract_value(&EmptyRequestV1 {})?,
        ),
        ArtifactOperationInputV1::Read {
            kind_ref,
            instance_id,
        } => (
            "artifact.read@1.0.0",
            to_contract_value(&TargetRequestV1 {
                kind_ref,
                instance_id,
            })?,
        ),
        ArtifactOperationInputV1::Validate {
            kind_ref,
            instance_id,
        } => (
            "artifact.validate@1.0.0",
            to_contract_value(&TargetRequestV1 {
                kind_ref,
                instance_id,
            })?,
        ),
        ArtifactOperationInputV1::IntakeDefinitionRead {
            kind_ref,
            instance_id,
        } => (
            "intake.definition.read@1.0.0",
            to_contract_value(&TargetRequestV1 {
                kind_ref,
                instance_id,
            })?,
        ),
        ArtifactOperationInputV1::IntakeCoverageEvaluate {
            kind_ref,
            instance_id,
            mode,
            expected_current_artifact_fingerprint,
            input,
        } => {
            let document: CoverageInputDocumentV1 = parse_input_document(&input)?;
            (
                "intake.coverage.evaluate@1.0.0",
                to_contract_value(&IntakeEvaluateRequestV1 {
                    kind_ref,
                    instance_id,
                    acquisition_mode: acquisition_mode_name(mode).to_owned(),
                    expected_current_artifact_fingerprint,
                    coverage_submissions: document.coverage_submissions,
                })?,
            )
        }
        ArtifactOperationInputV1::IntakeRecordAppend {
            kind_ref,
            instance_id,
            input,
        } => {
            let document: IntakeAppendInputDocumentV1 = parse_input_document(&input)?;
            (
                "intake.record.append@1.0.0",
                to_contract_value(&IntakeAppendRequestV1 {
                    kind_ref,
                    instance_id,
                    acquisition_mode: document.acquisition_mode,
                    expected_current_artifact_fingerprint: document
                        .expected_current_artifact_fingerprint,
                    coverage_submissions: document.coverage_submissions,
                    idempotency_key: document.idempotency_key,
                })?,
            )
        }
        ArtifactOperationInputV1::CandidateValidate {
            kind_ref,
            instance_id,
            intake_record_ref,
            intake_record_fingerprint,
            expected_current_artifact_fingerprint,
        } => (
            "artifact.candidate.validate@1.0.0",
            to_contract_value(&CandidateValidateRequestV1 {
                kind_ref,
                instance_id,
                intake_record_ref,
                intake_record_fingerprint,
                expected_current_artifact_fingerprint,
            })?,
        ),
        ArtifactOperationInputV1::CandidateAppend {
            kind_ref,
            instance_id,
            input,
        } => {
            let document: CandidateAppendInputDocumentV1 = parse_input_document(&input)?;
            (
                "artifact.candidate.append@1.0.0",
                to_contract_value(&CandidateAppendRequestV1 {
                    kind_ref,
                    instance_id,
                    intake_record_ref: document.intake_record_ref,
                    intake_record_fingerprint: document.intake_record_fingerprint,
                    expected_candidate_fingerprint: document.expected_candidate_fingerprint,
                    idempotency_key: document.idempotency_key,
                })?,
            )
        }
        ArtifactOperationInputV1::CandidatePromote {
            kind_ref,
            instance_id,
            input,
        } => {
            let document: PromoteInputDocumentV1 = parse_input_document(&input)?;
            (
                "artifact.candidate.promote@1.0.0",
                to_contract_value(&PromoteRequestV1 {
                    kind_ref,
                    instance_id,
                    candidate_ref: document.candidate_ref,
                    candidate_fingerprint: document.candidate_fingerprint,
                    expected_current_artifact_fingerprint: document
                        .expected_current_artifact_fingerprint,
                    publisher_authority: document.publisher_authority,
                    idempotency_key: document.idempotency_key,
                })?,
            )
        }
    };
    let definition = artifact_operation_definition(operation_ref)?;
    OperationRequestV1::new(
        schema_binding("handbook.operation-request@1.0.0")?,
        request_id.map(RequestId::parse).transpose()?,
        &definition,
        wire_body,
        &ArtifactSchemaValidator,
        &ContractLimits::packet0(),
    )
}

fn parse_input_document<T: for<'de> Deserialize<'de>>(
    input: &ArtifactInputDocument,
) -> Result<T, ContractError> {
    let value = handbook_engine::parse_definition_yaml(input.bytes())
        .map_err(|_| ContractError::InvalidJson)?;
    serde_json::from_value(value).map_err(|_| ContractError::InvalidJson)
}

fn acquisition_mode_name(mode: ArtifactAcquisitionMode) -> &'static str {
    match mode {
        ArtifactAcquisitionMode::GuidedAdaptive => "guided_adaptive",
        ArtifactAcquisitionMode::Express => "express",
        ArtifactAcquisitionMode::AgentAssisted => "agent_assisted",
    }
}

pub(crate) fn execute_artifact_operation(
    repo_root: &Path,
    request: &OperationRequestV1,
) -> Result<OperationResponseV1, ContractError> {
    let operation_ref = request.operation().reference().as_str();
    let definition = artifact_operation_definition(operation_ref)?;
    if request.operation() != &definition.binding()
        || request.body_schema() != definition.request_schema()
    {
        return Err(ContractError::InvalidInvariant);
    }
    match super::admit_operation_definition(&definition)? {
        OperationAdmissionResult::Admitted(owner)
            if owner.operation_ref() == definition.operation().as_str()
                && owner.definition_fingerprint()
                    == definition.definition_fingerprint().as_str() => {}
        _ => return Err(ContractError::InvalidInvariant),
    }

    match operation_ref {
        "artifact.kind.list@1.0.0" => {
            parse_body::<EmptyRequestV1>(request)?;
            let repository = match artifact_repository(repo_root) {
                Ok(repository) => repository,
                Err(error) => return repository_problem_response(request, &definition, error),
            };
            let selection_fingerprint = match repository.selection_fingerprint() {
                Ok(value) => value.to_string(),
                Err(error) => return repository_problem_response(request, &definition, error),
            };
            let kinds = match repository.list_kinds() {
                Ok(kinds) => kinds,
                Err(error) => return repository_problem_response(request, &definition, error),
            }
            .into_iter()
            .map(|kind| ArtifactKindDtoV1 {
                kind_ref: kind.kind_ref.to_string(),
                definition_fingerprint: kind.definition_fingerprint.to_string(),
                schema_ref: kind.schema_ref.to_string(),
            })
            .collect();
            ok_response(
                request,
                &definition,
                &KindListResultV1 {
                    operation_id: "artifact.kind.list".to_owned(),
                    selection_fingerprint,
                    kinds,
                },
            )
        }
        "artifact.instance.list@1.0.0" => {
            parse_body::<EmptyRequestV1>(request)?;
            let repository = match artifact_repository(repo_root) {
                Ok(repository) => repository,
                Err(error) => return repository_problem_response(request, &definition, error),
            };
            let selection_fingerprint = match repository.selection_fingerprint() {
                Ok(value) => value.to_string(),
                Err(error) => return repository_problem_response(request, &definition, error),
            };
            let instances = match repository.list_instances() {
                Ok(instances) => instances,
                Err(error) => return repository_problem_response(request, &definition, error),
            }
            .into_iter()
            .map(|instance| ArtifactInstanceDtoV1 {
                instance_id: instance.instance_id.to_string(),
                kind_ref: instance.kind_ref.to_string(),
                canonical_path: instance.canonical_path,
                intake_definition_ref: instance
                    .intake_definition_ref
                    .map(|value| value.to_string()),
            })
            .collect();
            ok_response(
                request,
                &definition,
                &InstanceListResultV1 {
                    operation_id: "artifact.instance.list".to_owned(),
                    selection_fingerprint,
                    instances,
                },
            )
        }
        "artifact.read@1.0.0" => {
            let body = parse_body::<TargetRequestV1>(request)?;
            let repository = match artifact_repository(repo_root) {
                Ok(repository) => repository,
                Err(error) => return repository_problem_response(request, &definition, error),
            };
            let target = match handbook_engine::artifact_repository::ArtifactTargetV1::parse(
                &body.kind_ref,
                &body.instance_id,
            ) {
                Ok(target) => target,
                Err(error) => return repository_problem_response(request, &definition, error),
            };
            let result = match repository.read(target.kind_ref(), target.instance_id()) {
                Ok(result) => result,
                Err(error) => return repository_problem_response(request, &definition, error),
            };
            ok_response(
                request,
                &definition,
                &ReadResultV1 {
                    operation_id: result.operation_id.as_str().to_owned(),
                    operation_context_fingerprint: result.operation_context_fingerprint.to_string(),
                    kind_ref: result.kind_ref.to_string(),
                    instance_id: result.instance_id.to_string(),
                    canonical_path: result.canonical_path,
                    artifact_fingerprint: result.artifact_fingerprint.to_string(),
                    content: artifact_document(result.content),
                },
            )
        }
        "artifact.validate@1.0.0" => {
            let body = parse_body::<TargetRequestV1>(request)?;
            let repository = match artifact_repository(repo_root) {
                Ok(repository) => repository,
                Err(error) => return repository_problem_response(request, &definition, error),
            };
            let target = match handbook_engine::artifact_repository::ArtifactTargetV1::parse(
                &body.kind_ref,
                &body.instance_id,
            ) {
                Ok(target) => target,
                Err(error) => return repository_problem_response(request, &definition, error),
            };
            let result = match repository.validate(target.kind_ref(), target.instance_id()) {
                Ok(result) => result,
                Err(error) => return repository_problem_response(request, &definition, error),
            };
            let layer = |status| {
                match status {
                handbook_engine::artifact_operations::ArtifactValidationLayerStatusV1::Pass => "pass".to_owned(),
                handbook_engine::artifact_operations::ArtifactValidationLayerStatusV1::NotApplicable => "not_applicable".to_owned(),
            }
            };
            ok_response(
                request,
                &definition,
                &ValidateResultV1 {
                    operation_id: result.operation_id.as_str().to_owned(),
                    operation_context_fingerprint: result.operation_context_fingerprint.to_string(),
                    artifact_fingerprint: result.artifact_fingerprint.to_string(),
                    outcome: "valid".to_owned(),
                    content: artifact_document(result.content),
                    layers: ValidationLayersV1 {
                        structural: "pass".to_owned(),
                        semantic: layer(result.semantic_status),
                        intake: layer(result.intake_status),
                        approval: layer(result.approval_status),
                        external_evidence: layer(result.external_evidence_status),
                    },
                },
            )
        }
        "intake.definition.read@1.0.0" => {
            let body = parse_body::<TargetRequestV1>(request)?;
            let repository = match artifact_repository(repo_root) {
                Ok(repository) => repository,
                Err(error) => return repository_problem_response(request, &definition, error),
            };
            let target = match handbook_engine::artifact_repository::ArtifactTargetV1::parse(
                &body.kind_ref,
                &body.instance_id,
            ) {
                Ok(target) => target,
                Err(error) => return repository_problem_response(request, &definition, error),
            };
            let result = match repository.intake_definition(&target) {
                Ok(result) => result,
                Err(error) => return repository_problem_response(request, &definition, error),
            };
            let coverage = result
                .coverage()
                .iter()
                .map(|row| CoverageDefinitionDtoV1 {
                    coverage_id: row.coverage_id().to_owned(),
                    target_paths: row.target_paths().to_vec(),
                    minimum_specificity: format!("{:?}", row.minimum_specificity()).to_lowercase(),
                })
                .collect();
            ok_response(
                request,
                &definition,
                &IntakeDefinitionResultV1 {
                    operation_id: "intake.definition.read".to_owned(),
                    kind_ref: body.kind_ref,
                    instance_id: body.instance_id,
                    intake_definition_ref: result.exact_ref().to_string(),
                    intake_definition_fingerprint: result.definition_fingerprint().to_string(),
                    candidate_schema_ref: result.candidate_schema_ref().to_string(),
                    coverage,
                },
            )
        }
        "intake.coverage.evaluate@1.0.0" => {
            let body = parse_body::<IntakeEvaluateRequestV1>(request)?;
            let repository = match artifact_repository(repo_root) {
                Ok(repository) => repository,
                Err(error) => return repository_problem_response(request, &definition, error),
            };
            let target = match handbook_engine::artifact_repository::ArtifactTargetV1::parse(
                &body.kind_ref,
                &body.instance_id,
            ) {
                Ok(target) => target,
                Err(error) => return repository_problem_response(request, &definition, error),
            };
            let mode = match body.acquisition_mode.as_str() {
                "guided_adaptive" => {
                    handbook_engine::artifact_intake_registry::AcquisitionModeV1::GuidedAdaptive
                }
                "express" => handbook_engine::artifact_intake_registry::AcquisitionModeV1::Express,
                "agent_assisted" => {
                    handbook_engine::artifact_intake_registry::AcquisitionModeV1::AgentAssisted
                }
                _ => return Err(ContractError::InvalidInvariant),
            };
            let input_bytes = serde_json::to_vec(&CoverageInputDocumentV1 {
                coverage_submissions: body.coverage_submissions,
            })
            .map_err(|_| ContractError::InvalidInvariant)?;
            let evaluation = match repository.evaluate_intake_document(
                &target,
                mode,
                Some(
                    body.expected_current_artifact_fingerprint
                        .as_deref()
                        .unwrap_or("absent"),
                ),
                &input_bytes,
            ) {
                Ok(evaluation) => evaluation,
                Err(error) => return repository_problem_response(request, &definition, error),
            };
            let value = evaluation
                .to_json_value()
                .map_err(|_| ContractError::InvalidInvariant)?;
            let result: IntakeEvaluationResultV1 =
                serde_json::from_value(value).map_err(|_| ContractError::InvalidInvariant)?;
            ok_response(request, &definition, &result)
        }
        "artifact.candidate.validate@1.0.0" => {
            let body = parse_body::<CandidateValidateRequestV1>(request)?;
            match handbook_engine::artifact_mutation::ArtifactMutationServiceV1::candidate_validate(
                repo_root,
                &body.kind_ref,
                &body.instance_id,
                &body.intake_record_ref,
                &body.intake_record_fingerprint,
                Some(
                    body.expected_current_artifact_fingerprint
                        .as_deref()
                        .unwrap_or("absent"),
                ),
            ) {
                Ok(preview) => {
                    let result = CandidateValidationResultV1 {
                        operation_id: preview.operation_id,
                        outcome: preview.outcome,
                        operation_context_fingerprint: preview.operation_context_fingerprint,
                        intake_record_ref: preview.intake_record_ref,
                        intake_record_fingerprint: preview.intake_record_fingerprint,
                        basis_artifact_fingerprint: preview.basis_artifact_fingerprint,
                        normalized_content: artifact_document(preview.normalized_content),
                        normalized_content_ref: preview.normalized_content_ref,
                        normalized_content_fingerprint: preview.normalized_content_fingerprint,
                        validation_result: artifact_document(preview.validation_result),
                        validation_result_ref: preview.validation_result_ref,
                        validation_result_fingerprint: preview.validation_result_fingerprint,
                        candidate: artifact_document(preview.candidate),
                        candidate_ref: preview.candidate_ref,
                        candidate_fingerprint: preview.candidate_fingerprint,
                    };
                    ok_response(request, &definition, &result)
                }
                Err(error) => mutation_error_response(request, &definition, error),
            }
        }
        "intake.record.append@1.0.0" => {
            let body = parse_body::<IntakeAppendSanitizedV1>(request)?;
            execute_mutation(
                repo_root,
                request,
                &definition,
                ArtifactMutationBody::Intake(&body),
            )
        }
        "artifact.candidate.append@1.0.0" => {
            let body = parse_body::<CandidateAppendSanitizedV1>(request)?;
            execute_mutation(
                repo_root,
                request,
                &definition,
                ArtifactMutationBody::CandidateAppend(&body),
            )
        }
        "artifact.candidate.promote@1.0.0" => {
            let body = parse_body::<PromoteSanitizedV1>(request)?;
            execute_mutation(
                repo_root,
                request,
                &definition,
                ArtifactMutationBody::Promote(&body),
            )
        }
        _ => Err(ContractError::InvalidInvariant),
    }
}

fn artifact_repository(
    repo_root: &Path,
) -> Result<
    handbook_engine::artifact_repository::ArtifactRepositoryV1,
    handbook_engine::artifact_repository::ArtifactRepositoryErrorV1,
> {
    handbook_engine::artifact_repository::ArtifactRepositoryV1::open(repo_root)
}

fn artifact_document(value: serde_json::Value) -> ArtifactDocument {
    match value {
        serde_json::Value::Null => ArtifactDocument::Null,
        serde_json::Value::Bool(value) => ArtifactDocument::Boolean(value),
        serde_json::Value::Number(value) => ArtifactDocument::Number(value.to_string()),
        serde_json::Value::String(value) => ArtifactDocument::String(value),
        serde_json::Value::Array(values) => {
            ArtifactDocument::Array(values.into_iter().map(artifact_document).collect())
        }
        serde_json::Value::Object(values) => ArtifactDocument::Object(
            values
                .into_iter()
                .map(|(key, value)| (key, artifact_document(value)))
                .collect(),
        ),
    }
}

enum ArtifactMutationBody<'a> {
    Intake(&'a IntakeAppendSanitizedV1),
    CandidateAppend(&'a CandidateAppendSanitizedV1),
    Promote(&'a PromoteSanitizedV1),
}

struct ArtifactMutationOwnerBridge<'a> {
    repo_root: &'a Path,
    body: ArtifactMutationBody<'a>,
    pointer_observed: bool,
    result: Option<
        Result<
            handbook_engine::artifact_mutation::GenericMutationExecutionV1,
            handbook_engine::artifact_mutation::ArtifactMutationErrorV1,
        >,
    >,
}

impl IdempotencyKeyConsumerSealed for ArtifactMutationOwnerBridge<'_> {
    fn consume_owner_idempotency_key(&mut self, pointer: &str, bytes: &[u8]) -> Fingerprint {
        self.pointer_observed = pointer == "/body/idempotency_key";
        let key = std::str::from_utf8(bytes).expect("admitted idempotency key remains ASCII");
        self.result = Some(match self.body {
            ArtifactMutationBody::Intake(body) => {
                let document = serde_json::to_vec(&IntakeOwnerRequestV1 {
                    idempotency_key: key,
                    acquisition_mode: &body.acquisition_mode,
                    expected_current_artifact_fingerprint: &body
                        .expected_current_artifact_fingerprint,
                    coverage_submissions: &body.coverage_submissions,
                })
                .expect("typed intake owner request serializes");
                handbook_engine::artifact_mutation::ArtifactMutationServiceV1::intake_append(
                    self.repo_root,
                    &body.kind_ref,
                    &body.instance_id,
                    &document,
                )
            }
            ArtifactMutationBody::CandidateAppend(body) => {
                let document = serde_json::to_vec(&CandidateAppendOwnerRequestV1 {
                    idempotency_key: key,
                    intake_record_ref: &body.intake_record_ref,
                    intake_record_fingerprint: &body.intake_record_fingerprint,
                    expected_candidate_fingerprint: &body.expected_candidate_fingerprint,
                })
                .expect("typed candidate owner request serializes");
                handbook_engine::artifact_mutation::ArtifactMutationServiceV1::candidate_append(
                    self.repo_root,
                    &body.kind_ref,
                    &body.instance_id,
                    &document,
                )
            }
            ArtifactMutationBody::Promote(body) => {
                let document = serde_json::to_vec(&PromoteOwnerRequestV1 {
                    idempotency_key: key,
                    candidate_ref: &body.candidate_ref,
                    candidate_fingerprint: &body.candidate_fingerprint,
                    expected_current_artifact_fingerprint: &body
                        .expected_current_artifact_fingerprint,
                    publisher_authority: &body.publisher_authority,
                })
                .expect("typed promotion owner request serializes");
                handbook_engine::artifact_mutation::ArtifactMutationServiceV1::promote(
                    self.repo_root,
                    &body.kind_ref,
                    &body.instance_id,
                    &document,
                )
            }
        });
        sha256_fingerprint(bytes)
    }
}

impl IdempotencyKeyConsumer for ArtifactMutationOwnerBridge<'_> {}

fn execute_mutation(
    repo_root: &Path,
    request: &OperationRequestV1,
    definition: &OperationDefinition,
    body: ArtifactMutationBody<'_>,
) -> Result<OperationResponseV1, ContractError> {
    let mut bridge = ArtifactMutationOwnerBridge {
        repo_root,
        body,
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
        Ok(execution) => project_mutation_execution(request, definition, execution),
        Err(error) => mutation_error_response(request, definition, error),
    }
}

fn project_mutation_execution(
    request: &OperationRequestV1,
    definition: &OperationDefinition,
    execution: handbook_engine::artifact_mutation::GenericMutationExecutionV1,
) -> Result<OperationResponseV1, ContractError> {
    use handbook_engine::artifact_mutation::GenericAuthorityClassV1;
    let result = execution.result;
    let evidence_ref = result
        .internal_transaction_evidence_ref
        .ok_or(ContractError::InvalidInvariant)?;
    let evidence_fingerprint = result
        .internal_transaction_evidence_fingerprint
        .ok_or(ContractError::InvalidInvariant)?;
    let mut public_outputs = Vec::new();
    let mut receipt_record_refs = Vec::new();
    for output in result.authoritative_outputs {
        let authority_class = match output.authority_class {
            GenericAuthorityClassV1::SubordinateClosure => continue,
            GenericAuthorityClassV1::SemanticRecord => "semantic_record",
            GenericAuthorityClassV1::CanonicalTruth => "canonical_truth",
        };
        receipt_record_refs.push(
            handbook_engine::artifact_mutation::ArtifactMutationServiceV1::receipt_record_ref(
                &result.operation_id,
                &output,
            )
            .ok_or(ContractError::InvalidInvariant)?,
        );
        public_outputs.push(ArtifactAuthoritativeOutputV1 {
            authority_class: authority_class.to_owned(),
            reference: output.relative_ref,
            fingerprint: output.fingerprint,
        });
    }
    let refusal = result.refusal.map(|refusal| ArtifactMutationRefusalV1 {
        code: established_refusal_code(refusal.code).to_owned(),
        layer: established_refusal_layer(refusal.layer).to_owned(),
        expected_fingerprint: refusal.expected_fingerprint,
        observed_fingerprint: refusal.observed_fingerprint,
    });
    let data = ArtifactMutationResultDataV1 {
        schema_id: result.schema_id,
        schema_version: result.schema_version,
        operation_id: result.operation_id,
        transaction_id: result.transaction_id,
        request_fingerprint: result.request_fingerprint,
        outcome: result.outcome,
        refusal,
        authoritative_outputs: public_outputs,
        internal_transaction_evidence_ref: evidence_ref,
        internal_transaction_evidence_fingerprint: evidence_fingerprint,
        result_fingerprint: result.result_fingerprint,
    };
    let replayed = matches!(
        execution.disposition,
        handbook_engine::artifact_mutation::GenericExecutionDispositionV1::Replayed
    );
    match data.outcome.as_str() {
        "committed" => {
            if data.refusal.is_some() {
                return Err(ContractError::InvalidInvariant);
            }
            let receipts = mutation_receipts(definition, &data, &receipt_record_refs)?;
            let terminal = TerminalFields::new(
                TerminalStatus::Ok,
                Some(to_contract_value(&data)?),
                BoundedVec::new(Vec::new())?,
                BoundedVec::new(Vec::new())?,
                BoundedVec::new(Vec::new())?,
            )?;
            established_mutation_response(request, definition, terminal, receipts, replayed)
        }
        "refused" => {
            if !data.authoritative_outputs.is_empty() || data.refusal.is_none() {
                return Err(ContractError::InvalidInvariant);
            }
            established_refusal_response(request, definition, data, replayed)
        }
        _ => Err(ContractError::InvalidInvariant),
    }
}

fn established_refusal_code(
    code: handbook_engine::artifact_mutation::EstablishedRefusalCodeV1,
) -> &'static str {
    use handbook_engine::artifact_mutation::EstablishedRefusalCodeV1 as Code;
    match code {
        Code::CanonicalSyntaxInvalid => "canonical_syntax_invalid",
        Code::StructuralValidationFailed => "structural_validation_failed",
        Code::IntakeCoverageBlocked => "intake_coverage_blocked",
        Code::StaleBasis => "stale_basis",
        Code::StaleCurrentArtifact => "stale_current_artifact",
        Code::OperationIneligible => "operation_ineligible",
        Code::PublicationBasisConflict => "publication_basis_conflict",
    }
}

fn established_refusal_layer(
    layer: handbook_engine::artifact_mutation::GenericRefusalLayerV1,
) -> &'static str {
    use handbook_engine::artifact_mutation::GenericRefusalLayerV1 as Layer;
    match layer {
        Layer::CanonicalSyntax => "canonical_syntax",
        Layer::Structural => "structural",
        Layer::Intake => "intake",
        Layer::Currentness => "currentness",
        Layer::Eligibility => "eligibility",
        Layer::Publication => "publication",
    }
}

fn mutation_receipts(
    definition: &OperationDefinition,
    data: &ArtifactMutationResultDataV1,
    receipt_record_refs: &[&str],
) -> Result<BoundedVec<WriteReceipt, 128>, ContractError> {
    if definition.write_set().len() != data.authoritative_outputs.len()
        || receipt_record_refs.len() != data.authoritative_outputs.len()
    {
        return Err(ContractError::InvalidInvariant);
    }
    let limits = ContractLimits::packet0();
    let mut receipts = Vec::with_capacity(definition.write_set().len());
    for ((declared, output), record_ref) in definition
        .write_set()
        .iter()
        .zip(&data.authoritative_outputs)
        .zip(receipt_record_refs)
    {
        if declared.authority_class() != output.authority_class {
            return Err(ContractError::InvalidInvariant);
        }
        let condition = to_contract_value(&ArtifactWriteConditionV1 {
            operation_id: data.operation_id.clone(),
            transaction_id: data.transaction_id.clone(),
            authority_class: output.authority_class.clone(),
            reference: output.reference.clone(),
            fingerprint: output.fingerprint.clone(),
        })?;
        ArtifactSchemaValidator.validate_instance(declared.condition_schema(), &condition)?;
        receipts.push(WriteReceipt::new(
            BoundedString::new(declared.record_kind().to_owned())?,
            ExactBinding::new(
                ExactRef::parse(record_ref)?,
                Fingerprint::parse(&output.fingerprint)?,
            ),
            BoundedString::new(output.authority_class.clone())?,
            condition,
            BoundedString::new(declared.atomic_group().to_owned())?,
            &limits,
        )?);
    }
    BoundedVec::new(receipts)
}

fn established_mutation_response(
    request: &OperationRequestV1,
    definition: &OperationDefinition,
    terminal: TerminalFields<ContractValue>,
    receipts: BoundedVec<WriteReceipt, 128>,
    replayed: bool,
) -> Result<OperationResponseV1, ContractError> {
    let scoped_key = request
        .idempotency_key_fingerprint()
        .cloned()
        .ok_or(ContractError::InvalidInvariant)?;
    let retention = UtcTimestamp::parse("9999-12-31T23:59:59Z")?;
    let original = if replayed {
        let original = response_from_parts(
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
        )?;
        Some(original.result_fingerprint().clone())
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

#[derive(Clone, Copy)]
struct ArtifactProblemProjection {
    cause: ArtifactProblemCauseV1,
    status: TerminalStatus,
    stage: IdempotencyStage,
    category: ProblemCategory,
    retry: RetryDirective,
    platform_recheck: bool,
}

fn repository_problem_response(
    request: &OperationRequestV1,
    definition: &OperationDefinition,
    error: handbook_engine::artifact_repository::ArtifactRepositoryErrorV1,
) -> Result<OperationResponseV1, ContractError> {
    use handbook_engine::artifact_repository::ArtifactRepositoryErrorKindV1 as Cause;
    let blocked = |cause| ArtifactProblemProjection {
        cause,
        status: TerminalStatus::Blocked,
        stage: IdempotencyStage::AuthorityValidation,
        category: ProblemCategory::Prerequisite,
        retry: RetryDirective::AfterRecheck,
        platform_recheck: false,
    };
    let refused = |cause, category, retry, stage| ArtifactProblemProjection {
        cause,
        status: TerminalStatus::Refused,
        stage,
        category,
        retry,
        platform_recheck: false,
    };
    let projection = match error.cause() {
        Cause::SelectionRead => blocked(ArtifactProblemCauseV1::SelectionUnavailable),
        Cause::ProfileResolution => blocked(ArtifactProblemCauseV1::ProfileUnavailable),
        Cause::ArtifactRegistryResolution => blocked(ArtifactProblemCauseV1::RegistryUnavailable),
        Cause::IntakeSourceRead => blocked(ArtifactProblemCauseV1::IntakeSourceUnavailable),
        Cause::RepositoryIdentity => blocked(ArtifactProblemCauseV1::RepositoryIdentityUnavailable),
        Cause::ArtifactRead => blocked(ArtifactProblemCauseV1::ArtifactUnavailable),
        Cause::SelectionAdmission => refused(
            ArtifactProblemCauseV1::SelectionAdmission,
            ProblemCategory::Schema,
            RetryDirective::AfterAuthorityChange,
            IdempotencyStage::AuthorityValidation,
        ),
        Cause::IntakeAdmission => refused(
            ArtifactProblemCauseV1::IntakeAdmission,
            ProblemCategory::Schema,
            RetryDirective::AfterAuthorityChange,
            IdempotencyStage::AuthorityValidation,
        ),
        Cause::TargetResolution | Cause::OperationTargetMismatch => refused(
            ArtifactProblemCauseV1::TargetResolution,
            ProblemCategory::Resolution,
            RetryDirective::AfterRequestChange,
            IdempotencyStage::ResolutionValidation,
        ),
        Cause::OperationNotApplicable => refused(
            ArtifactProblemCauseV1::NotApplicable,
            ProblemCategory::Capability,
            RetryDirective::Never,
            IdempotencyStage::CapabilityValidation,
        ),
        Cause::StaleCurrentArtifact => refused(
            ArtifactProblemCauseV1::StaleCurrentArtifact,
            ProblemCategory::Precondition,
            RetryDirective::AfterRequestChange,
            IdempotencyStage::PreconditionValidation,
        ),
        Cause::OperationStaleContext => refused(
            ArtifactProblemCauseV1::StaleOperationContext,
            ProblemCategory::Precondition,
            RetryDirective::AfterRequestChange,
            IdempotencyStage::PreconditionValidation,
        ),
        Cause::RequestBoundExceeded => refused(
            ArtifactProblemCauseV1::RequestBoundExceeded,
            ProblemCategory::Safety,
            RetryDirective::AfterRequestChange,
            IdempotencyStage::SafetyValidation,
        ),
        Cause::OperationCanonicalSyntax => refused(
            ArtifactProblemCauseV1::CanonicalSyntax,
            ProblemCategory::Schema,
            RetryDirective::AfterRequestChange,
            IdempotencyStage::RequestValidation,
        ),
        Cause::StructuralValidation | Cause::OperationStructural => refused(
            ArtifactProblemCauseV1::StructuralValidation,
            ProblemCategory::Schema,
            RetryDirective::AfterRequestChange,
            IdempotencyStage::RequestValidation,
        ),
        Cause::ArtifactOperation
        | Cause::IntakeEvaluation
        | Cause::InvalidRequest
        | Cause::IntakeMissingCoverage
        | Cause::IntakeDuplicateCoverage
        | Cause::IntakeUnknownCoverage
        | Cause::IntakeBlockingUnknown
        | Cause::IntakeBlockingContradiction
        | Cause::IntakeWrongSource
        | Cause::IntakeMissingValue
        | Cause::IntakeLowSpecificity
        | Cause::IntakeLowConfidence
        | Cause::IntakeUnsupportedMode
        | Cause::IntakeInvalidTarget
        | Cause::IntakeCanonicalization => refused(
            ArtifactProblemCauseV1::InvalidRequest,
            ProblemCategory::Schema,
            RetryDirective::AfterRequestChange,
            IdempotencyStage::RequestValidation,
        ),
    };
    problem_response(request, definition, projection, error.detail(), None)
}

fn mutation_error_response(
    request: &OperationRequestV1,
    definition: &OperationDefinition,
    error: handbook_engine::artifact_mutation::ArtifactMutationErrorV1,
) -> Result<OperationResponseV1, ContractError> {
    use handbook_engine::artifact_mutation::ArtifactMutationErrorKindV1 as Cause;
    let refused = |cause, category, retry, stage| ArtifactProblemProjection {
        cause,
        status: TerminalStatus::Refused,
        stage,
        category,
        retry,
        platform_recheck: false,
    };
    let implementation = |cause, retry| ArtifactProblemProjection {
        cause,
        status: TerminalStatus::Error,
        stage: IdempotencyStage::ExecutionStart,
        category: ProblemCategory::Implementation,
        retry,
        platform_recheck: false,
    };
    let projection = match error.cause() {
        Cause::RepositoryIdentity
        | Cause::AuthorityUnavailable
        | Cause::IntakeRecordUnavailable => ArtifactProblemProjection {
            cause: ArtifactProblemCauseV1::RepositoryIdentityUnavailable,
            status: TerminalStatus::Blocked,
            stage: IdempotencyStage::AuthorityValidation,
            category: ProblemCategory::Prerequisite,
            retry: RetryDirective::AfterRecheck,
            platform_recheck: false,
        },
        Cause::UnsupportedPlatform => ArtifactProblemProjection {
            cause: ArtifactProblemCauseV1::UnsupportedPlatform,
            status: TerminalStatus::Blocked,
            stage: IdempotencyStage::CapabilityValidation,
            category: ProblemCategory::Prerequisite,
            retry: RetryDirective::AfterRecheck,
            platform_recheck: true,
        },
        Cause::SelectionAdmission => refused(
            ArtifactProblemCauseV1::SelectionAdmission,
            ProblemCategory::Schema,
            RetryDirective::AfterAuthorityChange,
            IdempotencyStage::AuthorityValidation,
        ),
        Cause::IntakeAdmission => refused(
            ArtifactProblemCauseV1::IntakeAdmission,
            ProblemCategory::Schema,
            RetryDirective::AfterAuthorityChange,
            IdempotencyStage::AuthorityValidation,
        ),
        Cause::InvalidRequest | Cause::ValidationRefusal | Cause::IntakeRecordMismatch => refused(
            ArtifactProblemCauseV1::InvalidRequest,
            ProblemCategory::Schema,
            RetryDirective::AfterRequestChange,
            IdempotencyStage::RequestValidation,
        ),
        Cause::RequestBoundExceeded => refused(
            ArtifactProblemCauseV1::RequestBoundExceeded,
            ProblemCategory::Safety,
            RetryDirective::AfterRequestChange,
            IdempotencyStage::SafetyValidation,
        ),
        Cause::TargetResolution => refused(
            ArtifactProblemCauseV1::TargetResolution,
            ProblemCategory::Resolution,
            RetryDirective::AfterRequestChange,
            IdempotencyStage::ResolutionValidation,
        ),
        Cause::OperationNotApplicable => refused(
            ArtifactProblemCauseV1::NotApplicable,
            ProblemCategory::Capability,
            RetryDirective::Never,
            IdempotencyStage::CapabilityValidation,
        ),
        Cause::StaleCurrentArtifact => refused(
            ArtifactProblemCauseV1::StaleCurrentArtifact,
            ProblemCategory::Precondition,
            RetryDirective::AfterRequestChange,
            IdempotencyStage::PreconditionValidation,
        ),
        Cause::BasisMismatch => refused(
            ArtifactProblemCauseV1::StaleBasis,
            ProblemCategory::Precondition,
            RetryDirective::AfterRequestChange,
            IdempotencyStage::PreconditionValidation,
        ),
        Cause::IdempotencyConflict => refused(
            ArtifactProblemCauseV1::IdempotencyConflict,
            ProblemCategory::Idempotency,
            RetryDirective::AfterRequestChange,
            IdempotencyStage::IdempotencyValidation,
        ),
        Cause::IdempotencyExpired => refused(
            ArtifactProblemCauseV1::IdempotencyExpired,
            ProblemCategory::Idempotency,
            RetryDirective::AfterRequestChange,
            IdempotencyStage::IdempotencyValidation,
        ),
        Cause::UnsafeReference => refused(
            ArtifactProblemCauseV1::UnsafeReference,
            ProblemCategory::Safety,
            RetryDirective::Never,
            IdempotencyStage::SafetyValidation,
        ),
        Cause::UnsafeFilesystem => implementation(
            ArtifactProblemCauseV1::UnsafeFilesystem,
            RetryDirective::Never,
        ),
        Cause::RetainedStateBoundExceeded => implementation(
            ArtifactProblemCauseV1::RetainedStateBoundExceeded,
            RetryDirective::Never,
        ),
        Cause::Io => implementation(ArtifactProblemCauseV1::Io, RetryDirective::Transient),
        Cause::Evaluation | Cause::Store | Cause::InvalidRetainedState => implementation(
            ArtifactProblemCauseV1::InvalidRetainedState,
            RetryDirective::Never,
        ),
    };
    problem_response(request, definition, projection, error.detail(), None)
}

fn established_refusal_response(
    request: &OperationRequestV1,
    definition: &OperationDefinition,
    data: ArtifactMutationResultDataV1,
    replayed: bool,
) -> Result<OperationResponseV1, ContractError> {
    let refusal = data
        .refusal
        .as_ref()
        .ok_or(ContractError::InvalidInvariant)?;
    let (cause, category, retry, stage) = match refusal.code.as_str() {
        "canonical_syntax_invalid" => (
            ArtifactProblemCauseV1::CanonicalSyntax,
            ProblemCategory::Schema,
            RetryDirective::AfterRequestChange,
            IdempotencyStage::RequestValidation,
        ),
        "structural_validation_failed" | "intake_coverage_blocked" => (
            ArtifactProblemCauseV1::StructuralValidation,
            ProblemCategory::Schema,
            RetryDirective::AfterRequestChange,
            IdempotencyStage::RequestValidation,
        ),
        "stale_basis" | "publication_basis_conflict" => (
            ArtifactProblemCauseV1::StaleBasis,
            ProblemCategory::Precondition,
            RetryDirective::AfterRequestChange,
            IdempotencyStage::PreconditionValidation,
        ),
        "stale_current_artifact" => (
            ArtifactProblemCauseV1::StaleCurrentArtifact,
            ProblemCategory::Precondition,
            RetryDirective::AfterRequestChange,
            IdempotencyStage::PreconditionValidation,
        ),
        "operation_ineligible" => (
            ArtifactProblemCauseV1::NotApplicable,
            ProblemCategory::Capability,
            RetryDirective::Never,
            IdempotencyStage::CapabilityValidation,
        ),
        _ => return Err(ContractError::InvalidInvariant),
    };
    let projection = ArtifactProblemProjection {
        cause,
        status: TerminalStatus::Refused,
        stage,
        category,
        retry,
        platform_recheck: false,
    };
    let problem = projected_problem(
        request,
        definition,
        projection,
        "the established artifact mutation was refused",
        Some(data),
    )?;
    let terminal = TerminalFields::new(
        TerminalStatus::Refused,
        None,
        BoundedVec::new(Vec::new())?,
        BoundedVec::new(vec![problem])?,
        BoundedVec::new(Vec::new())?,
    )?;
    established_mutation_response(
        request,
        definition,
        terminal,
        BoundedVec::new(Vec::new())?,
        replayed,
    )
}

fn projected_problem(
    request: &OperationRequestV1,
    definition: &OperationDefinition,
    projection: ArtifactProblemProjection,
    detail: &str,
    mutation_result: Option<ArtifactMutationResultDataV1>,
) -> Result<Problem, ContractError> {
    let limits = ContractLimits::packet0();
    let operation_id = definition
        .operation()
        .as_str()
        .strip_suffix("@1.0.0")
        .ok_or(ContractError::InvalidInvariant)?;
    let cause = artifact_cause_name(projection.cause);
    let details = to_contract_value(&ArtifactProblemDetailsV1 {
        operation_id: operation_id.to_owned(),
        cause: projection.cause,
        detail: detail.to_owned(),
        mutation_result,
    })?;
    let details_schema = schema_binding(ARTIFACT_DETAILS_SCHEMA_REF)?;
    ArtifactSchemaValidator.validate_instance(&details_schema, &details)?;
    let rule = if projection.status == TerminalStatus::Refused {
        Some(ExactBinding::new(
            ExactRef::parse(&format!(
                "handbook.artifact-rule.{}@1.0.0",
                cause.replace('_', "-")
            ))?,
            definition.owner().source().fingerprint().clone(),
        ))
    } else {
        None
    };
    let recheck = if projection.status == TerminalStatus::Blocked {
        let condition = to_contract_value(&ArtifactRecheckV1 {
            operation_id: operation_id.to_owned(),
            cause: projection.cause,
        })?;
        let schema = schema_binding(if projection.platform_recheck {
            PLATFORM_RECHECK_SCHEMA_REF
        } else {
            REPOSITORY_RECHECK_SCHEMA_REF
        })?;
        ArtifactSchemaValidator.validate_instance(&schema, &condition)?;
        Some(RecheckCondition::new(schema, condition, &limits)?)
    } else {
        None
    };
    let correlation_ref = if projection.status == TerminalStatus::Error {
        let digest = sha256_fingerprint(
            format!(
                "handbook.artifact-error-correlation-v1\0{}\0{cause}",
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
    Problem::new(
        BoundedString::new(format!("artifact.{cause}"))?,
        BoundedString::new(format!("artifact.{cause}"))?,
        projection.category,
        definition.binding(),
        rule,
        details_schema,
        details,
        BoundedVec::new(Vec::new())?,
        projection.retry,
        recheck,
        correlation_ref,
        &limits,
    )
}

fn problem_response(
    request: &OperationRequestV1,
    definition: &OperationDefinition,
    projection: ArtifactProblemProjection,
    detail: &str,
    mutation_result: Option<ArtifactMutationResultDataV1>,
) -> Result<OperationResponseV1, ContractError> {
    let limits = ContractLimits::packet0();
    let operation_id = definition
        .operation()
        .as_str()
        .strip_suffix("@1.0.0")
        .ok_or(ContractError::InvalidInvariant)?;
    let cause = artifact_cause_name(projection.cause);
    let details = to_contract_value(&ArtifactProblemDetailsV1 {
        operation_id: operation_id.to_owned(),
        cause: projection.cause,
        detail: detail.to_owned(),
        mutation_result,
    })?;
    let details_schema = schema_binding(ARTIFACT_DETAILS_SCHEMA_REF)?;
    ArtifactSchemaValidator.validate_instance(&details_schema, &details)?;
    let rule = if projection.status == TerminalStatus::Refused {
        Some(ExactBinding::new(
            ExactRef::parse(&format!(
                "handbook.artifact-rule.{}@1.0.0",
                cause.replace('_', "-")
            ))?,
            definition.owner().source().fingerprint().clone(),
        ))
    } else {
        None
    };
    let recheck = if projection.status == TerminalStatus::Blocked {
        let condition = to_contract_value(&ArtifactRecheckV1 {
            operation_id: operation_id.to_owned(),
            cause: projection.cause,
        })?;
        let schema = schema_binding(if projection.platform_recheck {
            PLATFORM_RECHECK_SCHEMA_REF
        } else {
            REPOSITORY_RECHECK_SCHEMA_REF
        })?;
        ArtifactSchemaValidator.validate_instance(&schema, &condition)?;
        Some(RecheckCondition::new(schema, condition, &limits)?)
    } else {
        None
    };
    let correlation_ref = if projection.status == TerminalStatus::Error {
        let digest = sha256_fingerprint(
            format!(
                "handbook.artifact-error-correlation-v1\0{}\0{cause}",
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
    let problem_id = BoundedString::new(format!("artifact.{cause}"))?;
    let problem = Problem::new(
        problem_id.clone(),
        BoundedString::new(format!("artifact.{cause}"))?,
        projection.category,
        definition.binding(),
        rule,
        details_schema,
        details,
        BoundedVec::new(Vec::new())?,
        projection.retry,
        recheck,
        correlation_ref,
        &limits,
    )?;
    let binding = ProblemBinding::new(problem_id, problem.problem_fingerprint().clone())?;
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
        TerminalStatus::Error => TerminalFields::new(
            TerminalStatus::Error,
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
        IdempotencyState::not_established(projection.stage, binding)?
    };
    response_from_parts(
        request,
        definition,
        terminal,
        idempotency,
        BoundedVec::new(Vec::new())?,
    )
}

fn artifact_cause_name(cause: ArtifactProblemCauseV1) -> &'static str {
    match cause {
        ArtifactProblemCauseV1::SelectionUnavailable => "selection_unavailable",
        ArtifactProblemCauseV1::ProfileUnavailable => "profile_unavailable",
        ArtifactProblemCauseV1::RegistryUnavailable => "registry_unavailable",
        ArtifactProblemCauseV1::IntakeSourceUnavailable => "intake_source_unavailable",
        ArtifactProblemCauseV1::RepositoryIdentityUnavailable => "repository_identity_unavailable",
        ArtifactProblemCauseV1::ArtifactUnavailable => "artifact_unavailable",
        ArtifactProblemCauseV1::UnsupportedPlatform => "unsupported_platform",
        ArtifactProblemCauseV1::SelectionAdmission => "selection_admission",
        ArtifactProblemCauseV1::IntakeAdmission => "intake_admission",
        ArtifactProblemCauseV1::CanonicalSyntax => "canonical_syntax",
        ArtifactProblemCauseV1::StructuralValidation => "structural_validation",
        ArtifactProblemCauseV1::InvalidRequest => "invalid_request",
        ArtifactProblemCauseV1::TargetResolution => "target_resolution",
        ArtifactProblemCauseV1::NotApplicable => "not_applicable",
        ArtifactProblemCauseV1::StaleOperationContext => "stale_operation_context",
        ArtifactProblemCauseV1::StaleCurrentArtifact => "stale_current_artifact",
        ArtifactProblemCauseV1::StaleBasis => "stale_basis",
        ArtifactProblemCauseV1::IdempotencyConflict => "idempotency_conflict",
        ArtifactProblemCauseV1::IdempotencyExpired => "idempotency_expired",
        ArtifactProblemCauseV1::UnsafeReference => "unsafe_reference",
        ArtifactProblemCauseV1::RequestBoundExceeded => "request_bound_exceeded",
        ArtifactProblemCauseV1::UnsafeFilesystem => "unsafe_filesystem",
        ArtifactProblemCauseV1::RetainedStateBoundExceeded => "retained_state_bound_exceeded",
        ArtifactProblemCauseV1::Io => "io",
        ArtifactProblemCauseV1::InvalidRetainedState => "invalid_retained_state",
    }
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
        &ArtifactSchemaValidator,
        &ContractLimits::packet0(),
    )
}

fn parse_body<T: for<'de> Deserialize<'de> + Serialize>(
    request: &OperationRequestV1,
) -> Result<T, ContractError> {
    let limits = ContractLimits::packet0();
    parse_canonical_json(&canonical_json_bytes(request.body(), &limits)?, &limits)
}

fn ok_response<T: Serialize>(
    request: &OperationRequestV1,
    definition: &OperationDefinition,
    data: &T,
) -> Result<OperationResponseV1, ContractError> {
    let limits = ContractLimits::packet0();
    OperationResponseV1::new(
        schema_binding("handbook.operation-response@1.0.0")?,
        request.request_id().cloned(),
        request.request_fingerprint().clone(),
        definition,
        TerminalFields::new(
            TerminalStatus::Ok,
            Some(to_contract_value(data)?),
            BoundedVec::new(Vec::new())?,
            BoundedVec::new(Vec::new())?,
            BoundedVec::new(Vec::new())?,
        )?,
        IdempotencyState::not_applicable()?,
        BoundedVec::new(Vec::new())?,
        BoundedVec::new(Vec::new())?,
        BoundedVec::new(Vec::new())?,
        &ArtifactSchemaValidator,
        &limits,
    )
}

fn definition_behavior(
    operation_ref: &str,
) -> Result<(OperationClass, OperationIdempotency, Vec<DeclaredWrite>), ContractError> {
    let key_pointer = || JsonPointer::parse("/body/idempotency_key");
    let write = |record_kind: &str,
                 authority_class: &str,
                 atomic_group: &str,
                 condition_schema: &str|
     -> Result<DeclaredWrite, ContractError> {
        DeclaredWrite::new(
            BoundedString::new(record_kind.to_owned())?,
            BoundedString::new(authority_class.to_owned())?,
            BoundedString::new(atomic_group.to_owned())?,
            schema_binding(condition_schema)?,
        )
    };
    match operation_ref {
        "intake.record.append@1.0.0" => Ok((
            OperationClass::AppendOnly,
            OperationIdempotency::key_required(key_pointer()?)?,
            vec![write(
                "intake_record",
                "semantic_record",
                "intake_record_append",
                SEMANTIC_CONDITION_SCHEMA_REF,
            )?],
        )),
        "artifact.candidate.append@1.0.0" => Ok((
            OperationClass::AppendOnly,
            OperationIdempotency::key_required(key_pointer()?)?,
            vec![write(
                "artifact_candidate",
                "semantic_record",
                "artifact_candidate_append",
                SEMANTIC_CONDITION_SCHEMA_REF,
            )?],
        )),
        "artifact.candidate.promote@1.0.0" => {
            let basis = [
                "/body/candidate_fingerprint",
                "/body/candidate_ref",
                "/body/expected_current_artifact_fingerprint",
                "/body/instance_id",
                "/body/kind_ref",
                "/body/publisher_authority",
            ]
            .into_iter()
            .map(JsonPointer::parse)
            .collect::<Result<Vec<_>, _>>()?;
            Ok((
                OperationClass::CompareAndWrite,
                OperationIdempotency::compare_and_write(key_pointer()?, BoundedVec::new(basis)?)?,
                vec![
                    write(
                        "canonical_artifact",
                        "canonical_truth",
                        "artifact_candidate_promote",
                        CANONICAL_CONDITION_SCHEMA_REF,
                    )?,
                    write(
                        "promotion_record",
                        "semantic_record",
                        "artifact_candidate_promote",
                        SEMANTIC_CONDITION_SCHEMA_REF,
                    )?,
                ],
            ))
        }
        _ => Ok((
            OperationClass::ReadOnly,
            OperationIdempotency::safe(),
            Vec::new(),
        )),
    }
}

pub(crate) fn schema_binding(reference: &str) -> Result<ExactBinding, ContractError> {
    super::production_witness::current_schema_binding(reference)
}

pub(crate) fn to_contract_value<T: Serialize>(value: &T) -> Result<ContractValue, ContractError> {
    let limits = ContractLimits::packet0();
    parse_canonical_json(&canonical_json_bytes(value, &limits)?, &limits)
}

#[derive(Serialize)]
#[serde(deny_unknown_fields)]
struct ArtifactLegacyAuthoritativeOutputV1<'a> {
    authority_class: &'a str,
    fingerprint: &'a str,
    #[serde(rename = "ref")]
    reference: &'a str,
}

#[derive(Serialize)]
#[serde(deny_unknown_fields)]
struct ArtifactLegacyRefusalV1<'a> {
    code: &'a str,
    expected_fingerprint: &'a Option<String>,
    layer: &'a str,
    observed_fingerprint: &'a Option<String>,
}

#[derive(Serialize)]
#[serde(deny_unknown_fields)]
struct ArtifactLegacyMutationResultV1<'a> {
    authoritative_outputs: Vec<ArtifactLegacyAuthoritativeOutputV1<'a>>,
    execution_disposition: &'static str,
    internal_transaction_evidence_fingerprint: &'a str,
    internal_transaction_evidence_ref: &'a str,
    operation_id: &'a str,
    outcome: &'a str,
    refusal: Option<ArtifactLegacyRefusalV1<'a>>,
    request_fingerprint: &'a str,
    result_fingerprint: &'a str,
    schema_id: &'a str,
    schema_version: &'a str,
    transaction_id: &'a str,
}

pub fn render_artifact_operation_legacy(
    response: &OperationResponseV1,
    pretty: bool,
) -> Result<Vec<u8>, ContractError> {
    let operation_ref = response.operation().reference().as_str();
    let mutation = matches!(
        operation_ref,
        "intake.record.append@1.0.0"
            | "artifact.candidate.append@1.0.0"
            | "artifact.candidate.promote@1.0.0"
    );
    let mutation_data = if mutation {
        match artifact_mutation_result(response) {
            Ok(data) => Some(data),
            Err(_) if response.terminal().status() != TerminalStatus::Ok => None,
            Err(error) => return Err(error),
        }
    } else {
        None
    };
    let mut bytes = if let Some(data) = mutation_data {
        let disposition = match response.idempotency().as_ref() {
            super::IdempotencyStateRef::Established { replayed: true, .. } => "replayed",
            super::IdempotencyStateRef::Established {
                replayed: false, ..
            } if data.outcome == "committed" => "committed",
            super::IdempotencyStateRef::Established {
                replayed: false, ..
            } if data.outcome == "refused" => "refused",
            _ => return Err(ContractError::InvalidInvariant),
        };
        let legacy = ArtifactLegacyMutationResultV1 {
            authoritative_outputs: data
                .authoritative_outputs
                .iter()
                .map(|output| ArtifactLegacyAuthoritativeOutputV1 {
                    authority_class: &output.authority_class,
                    fingerprint: &output.fingerprint,
                    reference: &output.reference,
                })
                .collect(),
            execution_disposition: disposition,
            internal_transaction_evidence_fingerprint: &data
                .internal_transaction_evidence_fingerprint,
            internal_transaction_evidence_ref: &data.internal_transaction_evidence_ref,
            operation_id: &data.operation_id,
            outcome: &data.outcome,
            refusal: data
                .refusal
                .as_ref()
                .map(|refusal| ArtifactLegacyRefusalV1 {
                    code: &refusal.code,
                    expected_fingerprint: &refusal.expected_fingerprint,
                    layer: &refusal.layer,
                    observed_fingerprint: &refusal.observed_fingerprint,
                }),
            request_fingerprint: &data.request_fingerprint,
            result_fingerprint: &data.result_fingerprint,
            schema_id: &data.schema_id,
            schema_version: &data.schema_version,
            transaction_id: &data.transaction_id,
        };
        if pretty {
            serde_json::to_vec_pretty(&legacy)
        } else {
            serde_json::to_vec(&legacy)
        }
        .map_err(|_| ContractError::InvalidJson)?
    } else if response.terminal().status() == TerminalStatus::Ok {
        let data = response
            .terminal()
            .data()
            .ok_or(ContractError::InvalidInvariant)?;
        if pretty {
            serde_json::to_vec_pretty(data)
        } else {
            serde_json::to_vec(data)
        }
        .map_err(|_| ContractError::InvalidJson)?
    } else {
        let problem = response
            .terminal()
            .blockers()
            .first()
            .or_else(|| response.terminal().refusals().first())
            .or_else(|| response.terminal().errors().first())
            .ok_or(ContractError::InvalidInvariant)?;
        let details = from_contract_value::<ArtifactProblemDetailsV1>(problem.details())?;
        if pretty {
            format!(
                "artifact operation {}: {}\n",
                terminal_status_name(response.terminal().status()),
                details.detail
            )
            .into_bytes()
        } else {
            let mut bytes = serde_json::to_vec(&details).map_err(|_| ContractError::InvalidJson)?;
            bytes.push(b'\n');
            bytes
        }
    };
    if bytes.last() != Some(&b'\n') {
        bytes.push(b'\n');
    }
    Ok(bytes)
}

fn terminal_status_name(status: TerminalStatus) -> &'static str {
    match status {
        TerminalStatus::Ok => "ok",
        TerminalStatus::Blocked => "blocked",
        TerminalStatus::Refused => "refused",
        TerminalStatus::Error => "error",
    }
}

pub fn artifact_operation_problem_detail(
    response: &OperationResponseV1,
) -> Result<Option<String>, ContractError> {
    let problem = response
        .terminal()
        .blockers()
        .first()
        .or_else(|| response.terminal().refusals().first())
        .or_else(|| response.terminal().errors().first());
    problem
        .map(|problem| {
            from_contract_value::<ArtifactProblemDetailsV1>(problem.details())
                .map(|details| details.detail)
        })
        .transpose()
}

fn artifact_mutation_result(
    response: &OperationResponseV1,
) -> Result<ArtifactMutationResultDataV1, ContractError> {
    if let Some(data) = response.terminal().data() {
        return from_contract_value(data);
    }
    let problem = response
        .terminal()
        .refusals()
        .first()
        .ok_or(ContractError::InvalidInvariant)?;
    from_contract_value::<ArtifactProblemDetailsV1>(problem.details())?
        .mutation_result
        .ok_or(ContractError::InvalidInvariant)
}

fn from_contract_value<T: for<'de> Deserialize<'de> + Serialize>(
    value: &ContractValue,
) -> Result<T, ContractError> {
    let limits = ContractLimits::packet0();
    parse_canonical_json(&canonical_json_bytes(value, &limits)?, &limits)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn runtime_validator_enforces_result_and_problem_schema_constants_and_bounds() {
        let invalid_result = to_contract_value(&json!({
            "operation_id": "artifact.instance.list",
            "selection_fingerprint": "sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
            "kinds": []
        }))
        .unwrap();
        assert!(ArtifactSchemaValidator
            .validate_instance(
                &schema_binding("handbook.operation.artifact-kind-list-result@1.0.0").unwrap(),
                &invalid_result,
            )
            .is_err());

        let invalid_details = to_contract_value(&json!({
            "operation_id": "x".repeat(256),
            "cause": "profile_unavailable",
            "detail": "bounded detail",
            "mutation_result": null
        }))
        .unwrap();
        assert!(ArtifactSchemaValidator
            .validate_instance(
                &schema_binding(ARTIFACT_DETAILS_SCHEMA_REF).unwrap(),
                &invalid_details,
            )
            .is_err());
    }
}
