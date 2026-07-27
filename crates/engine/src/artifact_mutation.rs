use crate::artifact_intake::{
    CoverageEvaluationOutcomeV1, CoverageEvaluationV1, CoverageSubmissionV1,
    MAX_ARTIFACT_INPUT_DOCUMENT_BYTES,
};
use crate::artifact_intake_registry::AcquisitionModeV1;
pub use crate::artifact_lineage_store::{
    EstablishedRefusalCodeV1, EstablishedRefusalV1, GenericAuthoritativeOutputV1,
    GenericAuthorityClassV1, GenericDomainMutationResultV1, GenericExecutionDispositionV1,
    GenericMutationExecutionV1, GenericRefusalLayerV1,
};
use crate::artifact_lineage_store::{
    GenericArtifactLineageStoreV1, GenericArtifactOperationV1, GenericCommitPlanV1,
    GenericEvaluationContextV1, GenericInstallModeV1, GenericLineageStoreErrorKindV1,
    GenericLineageStoreErrorV1, GenericMutationRequestV1, GenericPlannedOutcomeV1,
    GenericPlannedOutputV1, GenericReceiptClassV1, GenericRefusalPlanV1,
};
use crate::artifact_repository::{
    ArtifactRepositoryAuthorityGuardV1, ArtifactRepositoryV1, ArtifactTargetV1,
};
use crate::canonical_yaml::{canonical_yaml_bytes, parse_canonical_yaml};
use crate::{parse_definition_yaml, DefinitionFingerprint, ExactDefinitionRef};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fmt;
use std::path::Path;

pub const HCM_2_3_OWNER_SUBJECT_FINGERPRINT: &str =
    "sha256:de116ba4567332ff9a3e9b5da11320987f04b7b87241f5d9e3b6fb2e19042ba1";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ArtifactMutationErrorKindV1 {
    InvalidRequest,
    RepositoryIdentity,
    Evaluation,
    Store,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtifactMutationErrorV1 {
    kind: ArtifactMutationErrorKindV1,
    detail: String,
}

impl ArtifactMutationErrorV1 {
    pub fn kind(&self) -> ArtifactMutationErrorKindV1 {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

impl fmt::Display for ArtifactMutationErrorV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.detail)
    }
}

impl std::error::Error for ArtifactMutationErrorV1 {}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct IntakeAppendDocumentV1 {
    idempotency_key: String,
    acquisition_mode: AcquisitionModeV1,
    expected_current_artifact_fingerprint: Option<String>,
    coverage_submissions: Vec<CoverageSubmissionV1>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct CandidateAppendDocumentV1 {
    idempotency_key: String,
    intake_record_ref: String,
    intake_record_fingerprint: String,
    expected_candidate_fingerprint: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct PromotionDocumentV1 {
    idempotency_key: String,
    candidate_ref: String,
    candidate_fingerprint: String,
    expected_current_artifact_fingerprint: Option<String>,
}

#[derive(Clone, Debug)]
pub struct ArtifactMutationServiceV1;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ArtifactCandidatePreviewV1 {
    pub operation_id: String,
    pub outcome: String,
    pub operation_context_fingerprint: String,
    pub intake_record_ref: String,
    pub intake_record_fingerprint: String,
    pub basis_artifact_fingerprint: Option<String>,
    pub normalized_content: Value,
    pub normalized_content_ref: String,
    pub normalized_content_fingerprint: String,
    pub validation_result: Value,
    pub validation_result_ref: String,
    pub validation_result_fingerprint: String,
    pub candidate: Value,
    pub candidate_ref: String,
    pub candidate_fingerprint: String,
}

fn request_authority(
    repo_root: &Path,
    authority: &ArtifactRepositoryAuthorityGuardV1,
    kind_ref: &str,
    instance_id: &str,
) -> Result<(String, String), ArtifactMutationErrorV1> {
    let repository = ArtifactRepositoryV1::open_under_authority(repo_root, authority)
        .map_err(repository_authority_error)?;
    let target =
        ArtifactTargetV1::parse(kind_ref, instance_id).map_err(repository_authority_error)?;
    let context = repository
        .operation_context_under_lock(target.kind_ref(), target.instance_id())
        .map_err(repository_authority_error)?;
    let canonical_ref = repository
        .canonical_path_under_lock(&target)
        .map_err(repository_authority_error)?;
    Ok((context.context_fingerprint().to_string(), canonical_ref))
}

fn require_revalidated_plan(
    planned: &GenericPlannedOutcomeV1,
    observed: GenericPlannedOutcomeV1,
) -> Result<(), GenericLineageStoreErrorV1> {
    if planned != &observed {
        return Err(control_error(
            "operation authority or evaluated plan changed before intent establishment",
        ));
    }
    Ok(())
}

pub(crate) fn validate_persisted_intent_authority(
    repo_root: &Path,
    authority: &ArtifactRepositoryAuthorityGuardV1,
    intent: &Value,
) -> Result<(), GenericLineageStoreErrorV1> {
    let subject = intent
        .get("request_subject")
        .ok_or_else(|| control_error("persisted request subject is absent"))?;
    let kind_ref = string_field(subject, "kind_ref")?;
    let instance_id = string_field(subject, "instance_id")?;
    let repository =
        ArtifactRepositoryV1::open_under_authority(repo_root, authority).map_err(|error| {
            control_error(format!("persisted authority resolution failed: {error}"))
        })?;
    let target = ArtifactTargetV1::parse(kind_ref, instance_id)
        .map_err(|error| control_error(format!("persisted target resolution failed: {error}")))?;
    let context = repository
        .operation_context_under_lock(target.kind_ref(), target.instance_id())
        .map_err(|error| control_error(format!("persisted context resolution failed: {error}")))?;
    if string_field(subject, "operation_context_fingerprint")?
        != context.context_fingerprint().as_str()
        || string_field(intent, "operation_context_fingerprint")?
            != context.context_fingerprint().as_str()
    {
        return Err(control_error(
            "persisted intent context is not current repository authority",
        ));
    }
    if string_field(intent, "operation_id")? == "artifact.candidate.promote" {
        let canonical_ref = repository
            .canonical_path_under_lock(&target)
            .map_err(|error| {
                control_error(format!("persisted canonical target failed: {error}"))
            })?;
        if string_field(subject, "canonical_artifact_ref")? != canonical_ref {
            return Err(control_error(
                "persisted promotion canonical target is not current descriptor authority",
            ));
        }
    }
    Ok(())
}

pub(crate) fn validate_persisted_output_authority(
    repo_root: &Path,
    authority: &ArtifactRepositoryAuthorityGuardV1,
    intent: &Value,
    ordinal: usize,
    final_ordinal: usize,
    bytes: &[u8],
) -> Result<(), GenericLineageStoreErrorV1> {
    let subject = intent
        .get("request_subject")
        .ok_or_else(|| control_error("persisted request subject is absent"))?;
    let kind_ref = string_field(subject, "kind_ref")?;
    let instance_id = string_field(subject, "instance_id")?;
    let operation = string_field(intent, "operation_id")?;
    let is_runtime_record = match operation {
        "intake.record.append" => ordinal == final_ordinal,
        "artifact.candidate.append" => matches!(ordinal, 1 | 2),
        "artifact.candidate.promote" => ordinal == 1,
        _ => false,
    };
    if !is_runtime_record {
        return Ok(());
    }
    let repository =
        ArtifactRepositoryV1::open_under_authority(repo_root, authority).map_err(|error| {
            control_error(format!(
                "persisted output authority resolution failed: {error}"
            ))
        })?;
    let target = ArtifactTargetV1::parse(kind_ref, instance_id).map_err(|error| {
        control_error(format!(
            "persisted output target resolution failed: {error}"
        ))
    })?;
    let context = repository
        .operation_context_under_lock(target.kind_ref(), target.instance_id())
        .map_err(|error| {
            control_error(format!(
                "persisted output context resolution failed: {error}"
            ))
        })?;
    let record: Value = serde_json::from_slice(bytes)
        .map_err(|_| control_error("persisted runtime output is not JSON"))?;
    let equals =
        |field: &str, expected: &str| record.get(field).and_then(Value::as_str) == Some(expected);
    if !equals("target_instance_id", context.instance_id().as_str())
        || !equals(
            "operation_context_fingerprint",
            context.context_fingerprint().as_str(),
        )
        || !equals("profile_ref", context.profile_ref().as_str())
        || !equals(
            "resolved_profile_fingerprint",
            context.resolved_profile_fingerprint().as_str(),
        )
        || (operation != "artifact.candidate.promote"
            && (!equals("target_kind_ref", context.kind_ref().as_str())
                || !equals("target_schema_ref", context.schema_ref().as_str())))
    {
        return Err(control_error(
            "persisted runtime output is not current repository context authority",
        ));
    }
    if operation == "intake.record.append"
        && (record.get("intake_definition_ref").and_then(Value::as_str)
            != context
                .intake_definition_ref()
                .map(ExactDefinitionRef::as_str)
            || record
                .get("intake_definition_fingerprint")
                .and_then(Value::as_str)
                != context
                    .intake_definition_fingerprint()
                    .map(DefinitionFingerprint::as_str))
    {
        return Err(control_error(
            "persisted intake output is not current intake authority",
        ));
    }
    if operation == "artifact.candidate.append" && ordinal == 1 {
        let schema_closure_fingerprint = repository
            .registry()
            .kind(context.kind_ref())
            .ok_or_else(|| control_error("persisted validation kind authority is absent"))?
            .schema_closure_fingerprint();
        let layers = record
            .get("layers")
            .ok_or_else(|| control_error("persisted validation layers are absent"))?;
        if layers
            .get("structural")
            .and_then(|layer| layer.get("schema_ref"))
            .and_then(Value::as_str)
            != Some(context.schema_ref().as_str())
            || layers
                .get("structural")
                .and_then(|layer| layer.get("schema_closure_fingerprint"))
                .and_then(Value::as_str)
                != Some(schema_closure_fingerprint.as_str())
            || layers
                .get("intake")
                .and_then(|layer| layer.get("intake_definition_ref"))
                .and_then(Value::as_str)
                != context
                    .intake_definition_ref()
                    .map(ExactDefinitionRef::as_str)
            || layers
                .get("intake")
                .and_then(|layer| layer.get("intake_definition_fingerprint"))
                .and_then(Value::as_str)
                != context
                    .intake_definition_fingerprint()
                    .map(DefinitionFingerprint::as_str)
        {
            return Err(control_error(
                "persisted validation output is not current schema/intake authority",
            ));
        }
    }
    if operation == "artifact.candidate.promote" {
        let expected = context
            .resolved_definitions()
            .iter()
            .map(|binding| {
                json!({
                    "definition_ref": binding.definition_ref.as_str(),
                    "definition_fingerprint": binding.definition_fingerprint.as_str(),
                })
            })
            .collect::<Vec<_>>();
        if record.get("resolved_definitions") != Some(&Value::Array(expected)) {
            return Err(control_error(
                "persisted promotion definitions are not current context authority",
            ));
        }
    }
    Ok(())
}

impl ArtifactMutationServiceV1 {
    pub fn intake_append(
        repo_root: impl AsRef<Path>,
        kind_ref: &str,
        instance_id: &str,
        request_bytes: &[u8],
    ) -> Result<GenericMutationExecutionV1, ArtifactMutationErrorV1> {
        let repo_root = repo_root.as_ref();
        let document = parse_intake_append(request_bytes)?;
        let authority = ArtifactRepositoryAuthorityGuardV1::acquire(repo_root)
            .map_err(repository_authority_error)?;
        let (operation_context_fingerprint, _) =
            request_authority(repo_root, &authority, kind_ref, instance_id)?;
        let repository_identity = authority.repository_identity_fingerprint().to_string();
        let coverage_value =
            serde_json::to_value(&document.coverage_submissions).map_err(|_| {
                mutation_error(
                    ArtifactMutationErrorKindV1::InvalidRequest,
                    "coverage submissions could not be canonicalized",
                )
            })?;
        let coverage_input_fingerprint = DefinitionFingerprint::from_json_value(&coverage_value)
            .map_err(|_| {
                mutation_error(
                    ArtifactMutationErrorKindV1::InvalidRequest,
                    "coverage submissions could not be fingerprinted",
                )
            })?;
        let request_subject = json!({
            "operation_id": "intake.record.append",
            "kind_ref": kind_ref,
            "instance_id": instance_id,
            "acquisition_mode": mode_name(document.acquisition_mode),
            "expected_current_artifact_fingerprint": document.expected_current_artifact_fingerprint,
            "coverage_input_fingerprint": coverage_input_fingerprint.as_str(),
            "operation_context_fingerprint": operation_context_fingerprint,
        });
        let request = GenericMutationRequestV1 {
            repository_identity_fingerprint: repository_identity,
            owner_contract_subject_fingerprint: HCM_2_3_OWNER_SUBJECT_FINGERPRINT.to_string(),
            operation: GenericArtifactOperationV1::IntakeRecordAppend,
            idempotency_key: document.idempotency_key.clone(),
            request_subject,
        };
        let store = GenericArtifactLineageStoreV1::new(repo_root);
        store
            .execute_authorized(
                request,
                || {
                    authority.require_current_identity().map_err(|error| {
                        control_error(format!("repository authority changed: {error}"))
                    })
                },
                |intent| validate_persisted_intent_authority(repo_root, &authority, intent),
                |intent, ordinal, final_ordinal, bytes| {
                    validate_persisted_output_authority(
                        repo_root,
                        &authority,
                        intent,
                        ordinal,
                        final_ordinal,
                        bytes,
                    )
                },
                |evaluation_context| {
                    let repository =
                        ArtifactRepositoryV1::open_under_authority(repo_root, &authority).map_err(
                            |error| control_error(format!("repository resolution failed: {error}")),
                        )?;
                    build_intake_plan(
                        &repository,
                        kind_ref,
                        instance_id,
                        &document,
                        evaluation_context,
                    )
                },
                |evaluation_context, planned| {
                    let repository = ArtifactRepositoryV1::open_under_authority(
                        repo_root, &authority,
                    )
                    .map_err(|error| {
                        control_error(format!("repository revalidation failed: {error}"))
                    })?;
                    let observed = build_intake_plan(
                        &repository,
                        kind_ref,
                        instance_id,
                        &document,
                        evaluation_context.clone(),
                    )?;
                    require_revalidated_plan(planned, observed)
                },
            )
            .map_err(|error| {
                mutation_error(
                    ArtifactMutationErrorKindV1::Store,
                    format!("generic lineage store refused: {}", error.detail()),
                )
            })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn candidate_validate(
        repo_root: impl AsRef<Path>,
        kind_ref: &str,
        instance_id: &str,
        intake_record_ref: &str,
        intake_record_fingerprint: &str,
        expected_current_artifact_fingerprint: Option<&str>,
    ) -> Result<ArtifactCandidatePreviewV1, ArtifactMutationErrorV1> {
        let repo_root = repo_root.as_ref();
        let authority = ArtifactRepositoryAuthorityGuardV1::acquire(repo_root)
            .map_err(repository_authority_error)?;
        let store = GenericArtifactLineageStoreV1::new(repo_root);
        store.evaluate_committed_read_with(
            authority.repository_identity_fingerprint().as_str(),
            HCM_2_3_OWNER_SUBJECT_FINGERPRINT,
            |error| {
                mutation_error(
                    ArtifactMutationErrorKindV1::Evaluation,
                    format!("candidate validation refused: {}", error.detail()),
                )
            },
            |intent| validate_persisted_intent_authority(repo_root, &authority, intent),
            |intent, ordinal, final_ordinal, bytes| {
                validate_persisted_output_authority(
                    repo_root,
                    &authority,
                    intent,
                    ordinal,
                    final_ordinal,
                    bytes,
                )
            },
            || {
                authority
                    .require_current_identity()
                    .map_err(repository_authority_error)?;
                let repository = ArtifactRepositoryV1::open_under_authority(repo_root, &authority)
                    .map_err(repository_authority_error)?;
                candidate_preview(
                    &repository,
                    kind_ref,
                    instance_id,
                    intake_record_ref,
                    intake_record_fingerprint,
                    expected_current_artifact_fingerprint,
                )
                .map_err(|error| {
                    mutation_error(
                        ArtifactMutationErrorKindV1::Evaluation,
                        format!("candidate validation refused: {}", error.detail()),
                    )
                })
            },
        )
    }

    pub fn candidate_append(
        repo_root: impl AsRef<Path>,
        kind_ref: &str,
        instance_id: &str,
        request_bytes: &[u8],
    ) -> Result<GenericMutationExecutionV1, ArtifactMutationErrorV1> {
        let repo_root = repo_root.as_ref();
        let document = parse_candidate_append(request_bytes)?;
        let authority = ArtifactRepositoryAuthorityGuardV1::acquire(repo_root)
            .map_err(repository_authority_error)?;
        let (operation_context_fingerprint, _) =
            request_authority(repo_root, &authority, kind_ref, instance_id)?;
        let repository_identity = authority.repository_identity_fingerprint().to_string();
        let request = GenericMutationRequestV1 {
            repository_identity_fingerprint: repository_identity,
            owner_contract_subject_fingerprint: HCM_2_3_OWNER_SUBJECT_FINGERPRINT.to_string(),
            operation: GenericArtifactOperationV1::ArtifactCandidateAppend,
            idempotency_key: document.idempotency_key.clone(),
            request_subject: json!({
                "operation_id": "artifact.candidate.append",
                "kind_ref": kind_ref,
                "instance_id": instance_id,
                "intake_record_ref": document.intake_record_ref,
                "intake_record_fingerprint": document.intake_record_fingerprint,
                "expected_candidate_fingerprint": document.expected_candidate_fingerprint,
                "operation_context_fingerprint": operation_context_fingerprint,
            }),
        };
        let store = GenericArtifactLineageStoreV1::new(repo_root);
        store
            .execute_authorized(
                request,
                || {
                    authority.require_current_identity().map_err(|error| {
                        control_error(format!("repository authority changed: {error}"))
                    })
                },
                |intent| validate_persisted_intent_authority(repo_root, &authority, intent),
                |intent, ordinal, final_ordinal, bytes| {
                    validate_persisted_output_authority(
                        repo_root,
                        &authority,
                        intent,
                        ordinal,
                        final_ordinal,
                        bytes,
                    )
                },
                |_| {
                    let repository =
                        ArtifactRepositoryV1::open_under_authority(repo_root, &authority).map_err(
                            |error| control_error(format!("repository resolution failed: {error}")),
                        )?;
                    build_candidate_append_plan(&repository, kind_ref, instance_id, &document)
                },
                |_, planned| {
                    let repository = ArtifactRepositoryV1::open_under_authority(
                        repo_root, &authority,
                    )
                    .map_err(|error| {
                        control_error(format!("repository revalidation failed: {error}"))
                    })?;
                    let observed =
                        build_candidate_append_plan(&repository, kind_ref, instance_id, &document)?;
                    require_revalidated_plan(planned, observed)
                },
            )
            .map_err(|error| {
                mutation_error(
                    ArtifactMutationErrorKindV1::Store,
                    format!("generic lineage store refused: {}", error.detail()),
                )
            })
    }

    pub fn promote(
        repo_root: impl AsRef<Path>,
        kind_ref: &str,
        instance_id: &str,
        request_bytes: &[u8],
    ) -> Result<GenericMutationExecutionV1, ArtifactMutationErrorV1> {
        let repo_root = repo_root.as_ref();
        let document = parse_promotion(request_bytes)?;
        let authority = ArtifactRepositoryAuthorityGuardV1::acquire(repo_root)
            .map_err(repository_authority_error)?;
        let (operation_context_fingerprint, canonical_artifact_ref) =
            request_authority(repo_root, &authority, kind_ref, instance_id)?;
        let repository_identity = authority.repository_identity_fingerprint().to_string();
        let request = GenericMutationRequestV1 {
            repository_identity_fingerprint: repository_identity,
            owner_contract_subject_fingerprint: HCM_2_3_OWNER_SUBJECT_FINGERPRINT.to_string(),
            operation: GenericArtifactOperationV1::ArtifactCandidatePromote,
            idempotency_key: document.idempotency_key.clone(),
            request_subject: json!({
                "operation_id": "artifact.candidate.promote",
                "kind_ref": kind_ref,
                "instance_id": instance_id,
                "candidate_ref": document.candidate_ref,
                "candidate_fingerprint": document.candidate_fingerprint,
                "expected_current_artifact_fingerprint": document.expected_current_artifact_fingerprint,
                "operation_context_fingerprint": operation_context_fingerprint,
                "canonical_artifact_ref": canonical_artifact_ref,
            }),
        };
        let store = GenericArtifactLineageStoreV1::new(repo_root);
        store
            .execute_authorized(
                request,
                || {
                    authority.require_current_identity().map_err(|error| {
                        control_error(format!("repository authority changed: {error}"))
                    })
                },
                |intent| validate_persisted_intent_authority(repo_root, &authority, intent),
                |intent, ordinal, final_ordinal, bytes| {
                    validate_persisted_output_authority(
                        repo_root,
                        &authority,
                        intent,
                        ordinal,
                        final_ordinal,
                        bytes,
                    )
                },
                |_| {
                    let repository =
                        ArtifactRepositoryV1::open_under_authority(repo_root, &authority).map_err(
                            |error| control_error(format!("repository resolution failed: {error}")),
                        )?;
                    build_promotion_plan(&repository, kind_ref, instance_id, &document)
                },
                |_, planned| {
                    let repository = ArtifactRepositoryV1::open_under_authority(
                        repo_root, &authority,
                    )
                    .map_err(|error| {
                        control_error(format!("repository revalidation failed: {error}"))
                    })?;
                    let observed =
                        build_promotion_plan(&repository, kind_ref, instance_id, &document)?;
                    require_revalidated_plan(planned, observed)
                },
            )
            .map_err(|error| {
                mutation_error(
                    ArtifactMutationErrorKindV1::Store,
                    format!("generic lineage store refused: {}", error.detail()),
                )
            })
    }
}

#[allow(clippy::too_many_arguments)]
fn candidate_preview(
    repository: &ArtifactRepositoryV1,
    kind_ref: &str,
    instance_id: &str,
    intake_record_ref: &str,
    intake_record_fingerprint: &str,
    expected_current_artifact_fingerprint: Option<&str>,
) -> Result<ArtifactCandidatePreviewV1, GenericLineageStoreErrorV1> {
    let target = ArtifactTargetV1::parse(kind_ref, instance_id)
        .map_err(|error| control_error(format!("target resolution failed: {error}")))?;
    let context = repository
        .operation_context_under_lock(target.kind_ref(), target.instance_id())
        .map_err(|error| control_error(format!("operation context failed: {error}")))?;
    let supplied_fingerprint = DefinitionFingerprint::parse(intake_record_fingerprint)
        .map_err(|_| control_error("intake record fingerprint is invalid"))?;
    let expected_intake_ref = format!(
        ".handbook/state/artifacts/{}/intake-records/intake_{}.json",
        target.instance_id().as_str(),
        fingerprint_hex(&supplied_fingerprint)
    );
    if intake_record_ref != expected_intake_ref {
        return Err(control_error(
            "intake record ref does not bind the requested target and fingerprint",
        ));
    }

    let store = GenericArtifactLineageStoreV1::new(repository.repo_root());
    let intake_bytes = store.read_committed_authoritative_during_evaluation(
        intake_record_ref,
        intake_record_fingerprint,
    )?;
    let intake = parse_runtime_record(
        &intake_bytes,
        "handbook.artifact-intake-record",
        "1.2",
        "intake_record_id",
        "record_fingerprint",
        "intake",
        intake_record_fingerprint,
    )?;
    require_exact_object_fields(
        &intake,
        &[
            "schema_id",
            "schema_version",
            "intake_definition_ref",
            "intake_definition_fingerprint",
            "acquisition_mode",
            "target_kind_ref",
            "target_instance_id",
            "target_schema_ref",
            "profile_ref",
            "resolved_profile_fingerprint",
            "operation_context_fingerprint",
            "consumer",
            "coverage_results",
            "basis_artifact_fingerprint",
            "prompt_event_refs",
            "finalized_at_utc",
            "intake_record_id",
            "record_fingerprint",
        ],
        "intake record",
    )?;
    validate_intake_bindings(&intake, repository, &target, &context)?;

    let basis = nullable_fingerprint_field(&intake, "basis_artifact_fingerprint")?;
    if let Some(requested) = expected_current_artifact_fingerprint {
        let requested = if requested == "absent" {
            None
        } else {
            Some(
                DefinitionFingerprint::parse(requested)
                    .map_err(|_| control_error("expected current fingerprint is invalid"))?,
            )
        };
        if requested != basis {
            return Err(control_error(
                "expected current fingerprint does not match the committed intake basis",
            ));
        }
    }
    let observed = repository
        .current_artifact_fingerprint_under_lock(&target)
        .map_err(|error| control_error(format!("current artifact observation failed: {error}")))?;
    if observed != basis {
        return Err(control_error(
            "the canonical artifact no longer matches the committed intake basis",
        ));
    }

    let definition = repository
        .intake_definition_under_lock(&target)
        .map_err(|error| control_error(format!("intake definition failed: {error}")))?;
    let coverage_results = array_field(&intake, "coverage_results")?;
    if coverage_results.len() != definition.coverage().len() {
        return Err(control_error(
            "committed intake coverage cardinality is not exact",
        ));
    }

    let mut normalized_content = Value::Object(serde_json::Map::new());
    let mut field_sources = Vec::new();
    for (retained, coverage) in coverage_results.iter().zip(definition.coverage()) {
        require_exact_object_fields(
            retained,
            &[
                "coverage_id",
                "applicability",
                "source_kind",
                "value_ref",
                "value_fingerprint",
                "evidence_refs",
                "confidence",
                "freshness",
                "sensitivity",
                "evaluation",
                "contradiction_refs",
                "waiver_ref",
            ],
            "intake coverage result",
        )?;
        if string_field(retained, "coverage_id")? != coverage.coverage_id()
            || string_field(retained, "applicability")? != "applicable"
            || string_field(retained, "source_kind")? != "user_declaration"
            || string_field(retained, "confidence")? != "high"
            || string_field(retained, "sensitivity")? != "internal"
            || string_field(retained, "evaluation")? != "satisfied"
            || !array_field(retained, "evidence_refs")?.is_empty()
            || !array_field(retained, "contradiction_refs")?.is_empty()
            || !retained.get("freshness").is_some_and(Value::is_null)
            || !retained.get("waiver_ref").is_some_and(Value::is_null)
        {
            return Err(control_error(
                "committed intake coverage is not the exact admitted satisfied form",
            ));
        }
        let value_ref = string_field(retained, "value_ref")?;
        let value_fingerprint = string_field(retained, "value_fingerprint")?;
        let value_fingerprint_parsed = DefinitionFingerprint::parse(value_fingerprint)
            .map_err(|_| control_error("intake value fingerprint is invalid"))?;
        let expected_value_ref = format!(
            ".handbook/evidence/artifacts/{}/intake-values/value_{}.json",
            target.instance_id().as_str(),
            fingerprint_hex(&value_fingerprint_parsed)
        );
        if value_ref != expected_value_ref {
            return Err(control_error(
                "intake value ref does not bind the target and fingerprint",
            ));
        }
        let value_bytes = store.read_committed_closure_during_evaluation(
            intake_record_ref,
            intake_record_fingerprint,
            value_ref,
            value_fingerprint,
        )?;
        let value = parse_jcs_lf_value(&value_bytes)?;
        for target_path in coverage.target_paths() {
            insert_json_pointer(&mut normalized_content, target_path, value.clone())?;
            field_sources.push(json!({
                "target_path": target_path,
                "coverage_id": coverage.coverage_id(),
                "source_kind": "user_declaration",
                "value_ref": value_ref,
                "value_fingerprint": value_fingerprint,
            }));
        }
    }

    repository
        .registry()
        .validate_json(target.instance_id(), &normalized_content)
        .map_err(|_| control_error("candidate content failed structural validation"))?;
    let kind = repository
        .registry()
        .kind(target.kind_ref())
        .ok_or_else(|| control_error("selected kind disappeared"))?;
    let normalized_content_fingerprint =
        DefinitionFingerprint::from_json_value(&normalized_content)
            .map_err(|_| control_error("candidate content fingerprint failed"))?;
    let normalized_content_ref = format!(
        ".handbook/evidence/artifacts/{}/content/content_{}.json",
        target.instance_id().as_str(),
        fingerprint_hex(&normalized_content_fingerprint)
    );

    let validation_subject = json!({
        "schema_id": "handbook.artifact-validation-result",
        "schema_version": "1.0",
        "target_kind_ref": target.kind_ref().as_str(),
        "target_instance_id": target.instance_id().as_str(),
        "target_schema_ref": kind.canonical_schema_ref().as_str(),
        "profile_ref": context.profile_ref().as_str(),
        "resolved_profile_fingerprint": context.resolved_profile_fingerprint().as_str(),
        "operation_context_fingerprint": context.context_fingerprint().as_str(),
        "intake_record_ref": intake_record_ref,
        "intake_record_fingerprint": intake_record_fingerprint,
        "normalized_content_ref": normalized_content_ref,
        "normalized_content_fingerprint": normalized_content_fingerprint.as_str(),
        "basis_artifact_fingerprint": basis.as_ref().map(DefinitionFingerprint::as_str),
        "layers": {
            "source": {"status": "pass"},
            "registration": {"status": "pass"},
            "canonical_syntax": {"status": "pass"},
            "structural": {
                "status": "pass",
                "schema_ref": kind.canonical_schema_ref().as_str(),
                "schema_closure_fingerprint": kind.schema_closure_fingerprint().as_str(),
            },
            "semantic": {
                "status": "not_applicable",
                "reason": "no_semantic_validator",
            },
            "intake": {
                "status": "pass",
                "intake_definition_ref": definition.exact_ref().as_str(),
                "intake_definition_fingerprint": definition.definition_fingerprint().as_str(),
            },
            "approval": {
                "status": "not_applicable",
                "reason": "null_approval_policy",
            },
            "external_evidence": {
                "status": "not_applicable",
                "reason": "not_selected",
            },
        },
        "outcome": "valid",
    });
    let validation = finalize_record(
        validation_subject,
        "validation_result_id",
        "validation_result_fingerprint",
        "validation",
    )?;
    let validation_result_ref = format!(
        ".handbook/evidence/artifacts/{}/validation-results/{}.json",
        target.instance_id().as_str(),
        validation.id
    );
    let validation_value = parse_jcs_lf_value(&validation.bytes)?;

    let candidate_subject = json!({
        "schema_id": "handbook.artifact-candidate",
        "schema_version": "1.4",
        "intake_record_ref": intake_record_ref,
        "intake_record_fingerprint": intake_record_fingerprint,
        "target_kind_ref": target.kind_ref().as_str(),
        "target_instance_id": target.instance_id().as_str(),
        "target_schema_ref": kind.canonical_schema_ref().as_str(),
        "profile_ref": context.profile_ref().as_str(),
        "resolved_profile_fingerprint": context.resolved_profile_fingerprint().as_str(),
        "operation_context_fingerprint": context.context_fingerprint().as_str(),
        "normalized_content_ref": normalized_content_ref,
        "normalized_content_fingerprint": normalized_content_fingerprint.as_str(),
        "field_sources": field_sources,
        "validation_result_refs": [validation_result_ref],
        "unresolved_coverage_ids": [],
        "promotion_eligibility": "eligible_without_approval",
        "required_approval_policy_ref": null,
        "basis_artifact_fingerprint": basis.as_ref().map(DefinitionFingerprint::as_str),
    });
    let candidate = finalize_record(
        candidate_subject,
        "candidate_id",
        "candidate_fingerprint",
        "candidate",
    )?;
    let candidate_ref = format!(
        ".handbook/evidence/artifacts/{}/candidates/{}.json",
        target.instance_id().as_str(),
        candidate.id
    );
    let candidate_value = parse_jcs_lf_value(&candidate.bytes)?;

    Ok(ArtifactCandidatePreviewV1 {
        operation_id: "artifact.candidate.validate".to_string(),
        outcome: "valid".to_string(),
        operation_context_fingerprint: context.context_fingerprint().to_string(),
        intake_record_ref: intake_record_ref.to_string(),
        intake_record_fingerprint: intake_record_fingerprint.to_string(),
        basis_artifact_fingerprint: basis.as_ref().map(ToString::to_string),
        normalized_content,
        normalized_content_ref,
        normalized_content_fingerprint: normalized_content_fingerprint.to_string(),
        validation_result: validation_value,
        validation_result_ref,
        validation_result_fingerprint: validation.fingerprint,
        candidate: candidate_value,
        candidate_ref,
        candidate_fingerprint: candidate.fingerprint,
    })
}

fn validate_intake_bindings(
    intake: &Value,
    repository: &ArtifactRepositoryV1,
    target: &ArtifactTargetV1,
    context: &crate::artifact_operation_context::ArtifactOperationContextV1,
) -> Result<(), GenericLineageStoreErrorV1> {
    let definition = repository
        .intake_definition_under_lock(target)
        .map_err(|error| control_error(format!("intake definition failed: {error}")))?;
    let kind = repository
        .registry()
        .kind(target.kind_ref())
        .ok_or_else(|| control_error("selected kind disappeared"))?;
    if string_field(intake, "intake_definition_ref")? != definition.exact_ref().as_str()
        || string_field(intake, "intake_definition_fingerprint")?
            != definition.definition_fingerprint().as_str()
        || string_field(intake, "target_kind_ref")? != target.kind_ref().as_str()
        || string_field(intake, "target_instance_id")? != target.instance_id().as_str()
        || string_field(intake, "target_schema_ref")? != kind.canonical_schema_ref().as_str()
        || string_field(intake, "profile_ref")? != context.profile_ref().as_str()
        || string_field(intake, "resolved_profile_fingerprint")?
            != context.resolved_profile_fingerprint().as_str()
        || string_field(intake, "operation_context_fingerprint")?
            != context.context_fingerprint().as_str()
    {
        return Err(control_error(
            "committed intake does not match the current exact operation context",
        ));
    }
    let mode = string_field(intake, "acquisition_mode")?;
    if !matches!(mode, "guided_adaptive" | "express" | "agent_assisted") {
        return Err(control_error(
            "committed intake acquisition mode is invalid",
        ));
    }
    require_exact_object_fields(
        intake
            .get("consumer")
            .ok_or_else(|| control_error("intake consumer is absent"))?,
        &["kind", "id", "version"],
        "intake consumer",
    )?;
    let consumer = intake
        .get("consumer")
        .expect("consumer presence was checked");
    if string_field(consumer, "kind")? != "handbook_cli"
        || string_field(consumer, "id")? != "handbook"
        || string_field(consumer, "version")? != engine_release_version()?
        || !array_field(intake, "prompt_event_refs")?.is_empty()
        || string_field(intake, "finalized_at_utc")?.is_empty()
    {
        return Err(control_error(
            "committed intake consumer or finalization binding is invalid",
        ));
    }
    Ok(())
}

fn parse_runtime_record(
    bytes: &[u8],
    schema_id: &str,
    schema_version: &str,
    id_field: &str,
    fingerprint_field: &str,
    id_prefix: &str,
    expected_fingerprint: &str,
) -> Result<Value, GenericLineageStoreErrorV1> {
    let value = parse_jcs_lf_value(bytes)?;
    if string_field(&value, "schema_id")? != schema_id
        || string_field(&value, "schema_version")? != schema_version
        || string_field(&value, fingerprint_field)? != expected_fingerprint
    {
        return Err(control_error(
            "runtime record family, version, or fingerprint is incompatible",
        ));
    }
    let fingerprint = DefinitionFingerprint::parse(expected_fingerprint)
        .map_err(|_| control_error("runtime record fingerprint is invalid"))?;
    let expected_id = format!("{id_prefix}_{}", fingerprint_hex(&fingerprint));
    if string_field(&value, id_field)? != expected_id {
        return Err(control_error("runtime record ID does not derive exactly"));
    }
    let mut subject = value.clone();
    let subject = subject
        .as_object_mut()
        .ok_or_else(|| control_error("runtime record is not an object"))?;
    subject.remove(id_field);
    subject.remove(fingerprint_field);
    let computed = DefinitionFingerprint::from_json_value(&Value::Object(subject.clone()))
        .map_err(|_| control_error("runtime record fingerprint could not be recomputed"))?;
    if computed != fingerprint {
        return Err(control_error(
            "runtime record fingerprint does not recompute",
        ));
    }
    Ok(value)
}

fn parse_jcs_lf_value(bytes: &[u8]) -> Result<Value, GenericLineageStoreErrorV1> {
    if bytes.is_empty() || bytes.last() != Some(&b'\n') {
        return Err(control_error("retained JSON is not JCS plus one LF"));
    }
    let value: Value = serde_json::from_slice(&bytes[..bytes.len() - 1])
        .map_err(|_| control_error("retained JSON is malformed"))?;
    if jcs_lf(&value)? != bytes {
        return Err(control_error("retained JSON is not exact JCS plus one LF"));
    }
    Ok(value)
}

fn require_exact_object_fields(
    value: &Value,
    fields: &[&str],
    label: &str,
) -> Result<(), GenericLineageStoreErrorV1> {
    let object = value
        .as_object()
        .ok_or_else(|| control_error(format!("{label} is not an object")))?;
    if object.len() != fields.len() || fields.iter().any(|field| !object.contains_key(*field)) {
        return Err(control_error(format!("{label} field set is not exact")));
    }
    Ok(())
}

fn string_field<'a>(value: &'a Value, field: &str) -> Result<&'a str, GenericLineageStoreErrorV1> {
    value
        .get(field)
        .and_then(Value::as_str)
        .ok_or_else(|| control_error(format!("{field} is not a string")))
}

fn array_field<'a>(
    value: &'a Value,
    field: &str,
) -> Result<&'a [Value], GenericLineageStoreErrorV1> {
    value
        .get(field)
        .and_then(Value::as_array)
        .map(Vec::as_slice)
        .ok_or_else(|| control_error(format!("{field} is not an array")))
}

fn nullable_fingerprint_field(
    value: &Value,
    field: &str,
) -> Result<Option<DefinitionFingerprint>, GenericLineageStoreErrorV1> {
    match value.get(field) {
        Some(Value::Null) => Ok(None),
        Some(Value::String(value)) => DefinitionFingerprint::parse(value)
            .map(Some)
            .map_err(|_| control_error(format!("{field} is not a fingerprint"))),
        _ => Err(control_error(format!(
            "{field} is neither null nor a fingerprint"
        ))),
    }
}

fn insert_json_pointer(
    root: &mut Value,
    pointer: &str,
    value: Value,
) -> Result<(), GenericLineageStoreErrorV1> {
    let encoded = pointer
        .strip_prefix('/')
        .ok_or_else(|| control_error("coverage target path is not a JSON pointer"))?;
    if encoded.is_empty() {
        return Err(control_error("coverage cannot replace the document root"));
    }
    let segments = encoded
        .split('/')
        .map(|segment| segment.replace("~1", "/").replace("~0", "~"))
        .collect::<Vec<_>>();
    let mut current = root;
    for segment in &segments[..segments.len() - 1] {
        let object = current
            .as_object_mut()
            .ok_or_else(|| control_error("coverage target paths conflict"))?;
        current = object
            .entry(segment.clone())
            .or_insert_with(|| Value::Object(serde_json::Map::new()));
    }
    let object = current
        .as_object_mut()
        .ok_or_else(|| control_error("coverage target paths conflict"))?;
    if object
        .insert(segments.last().expect("pointer is nonempty").clone(), value)
        .is_some()
    {
        return Err(control_error("coverage target paths are not unique"));
    }
    Ok(())
}

fn parse_candidate_append(
    bytes: &[u8],
) -> Result<CandidateAppendDocumentV1, ArtifactMutationErrorV1> {
    require_mutation_input_document_bound(bytes)?;
    let value = parse_definition_yaml(bytes).map_err(|_| {
        mutation_error(
            ArtifactMutationErrorKindV1::InvalidRequest,
            "candidate append request is not one duplicate-safe document",
        )
    })?;
    require_exact_fields(
        &value,
        &[
            "idempotency_key",
            "intake_record_ref",
            "intake_record_fingerprint",
            "expected_candidate_fingerprint",
        ],
    )?;
    serde_json::from_value(value).map_err(|_| {
        mutation_error(
            ArtifactMutationErrorKindV1::InvalidRequest,
            "candidate append request does not match the closed request shape",
        )
    })
}

fn build_candidate_append_plan(
    repository: &ArtifactRepositoryV1,
    kind_ref: &str,
    instance_id: &str,
    document: &CandidateAppendDocumentV1,
) -> Result<GenericPlannedOutcomeV1, GenericLineageStoreErrorV1> {
    DefinitionFingerprint::parse(&document.expected_candidate_fingerprint)
        .map_err(|_| control_error("expected candidate fingerprint is invalid"))?;
    let preview = candidate_preview(
        repository,
        kind_ref,
        instance_id,
        &document.intake_record_ref,
        &document.intake_record_fingerprint,
        None,
    )?;
    if preview.candidate_fingerprint != document.expected_candidate_fingerprint {
        return Ok(GenericPlannedOutcomeV1::Refuse(GenericRefusalPlanV1 {
            operation_context_fingerprint: preview.operation_context_fingerprint,
            expected_basis_fingerprint: preview.basis_artifact_fingerprint,
            refusal: EstablishedRefusalV1 {
                code: EstablishedRefusalCodeV1::StaleBasis,
                layer: GenericRefusalLayerV1::Currentness,
                expected_fingerprint: Some(document.expected_candidate_fingerprint.clone()),
                observed_fingerprint: Some(preview.candidate_fingerprint),
            },
        }));
    }
    let outputs = vec![
        GenericPlannedOutputV1 {
            token: "normalized-content".to_string(),
            authority_class: GenericAuthorityClassV1::SubordinateClosure,
            final_ref: preview.normalized_content_ref,
            bytes: jcs_lf(&preview.normalized_content)?,
            output_fingerprint: preview.normalized_content_fingerprint,
            install_mode: GenericInstallModeV1::CreateNew,
            receipt_class: None,
        },
        GenericPlannedOutputV1 {
            token: "validation-result".to_string(),
            authority_class: GenericAuthorityClassV1::SubordinateClosure,
            final_ref: preview.validation_result_ref,
            bytes: jcs_lf(&preview.validation_result)?,
            output_fingerprint: preview.validation_result_fingerprint,
            install_mode: GenericInstallModeV1::CreateNew,
            receipt_class: None,
        },
        GenericPlannedOutputV1 {
            token: "candidate-record".to_string(),
            authority_class: GenericAuthorityClassV1::SemanticRecord,
            final_ref: preview.candidate_ref,
            bytes: jcs_lf(&preview.candidate)?,
            output_fingerprint: preview.candidate_fingerprint,
            install_mode: GenericInstallModeV1::CreateNew,
            receipt_class: Some(GenericReceiptClassV1::SemanticRecord),
        },
    ];
    Ok(GenericPlannedOutcomeV1::Commit(GenericCommitPlanV1 {
        operation_context_fingerprint: preview.operation_context_fingerprint,
        expected_basis_fingerprint: preview.basis_artifact_fingerprint,
        sampled_finalized_at_utc: None,
        outputs,
    }))
}

fn parse_promotion(bytes: &[u8]) -> Result<PromotionDocumentV1, ArtifactMutationErrorV1> {
    require_mutation_input_document_bound(bytes)?;
    let value = parse_definition_yaml(bytes).map_err(|_| {
        mutation_error(
            ArtifactMutationErrorKindV1::InvalidRequest,
            "promotion request is not one duplicate-safe document",
        )
    })?;
    require_exact_fields(
        &value,
        &[
            "idempotency_key",
            "candidate_ref",
            "candidate_fingerprint",
            "expected_current_artifact_fingerprint",
        ],
    )?;
    serde_json::from_value(value).map_err(|_| {
        mutation_error(
            ArtifactMutationErrorKindV1::InvalidRequest,
            "promotion request does not match the closed request shape",
        )
    })
}

fn build_promotion_plan(
    repository: &ArtifactRepositoryV1,
    kind_ref: &str,
    instance_id: &str,
    document: &PromotionDocumentV1,
) -> Result<GenericPlannedOutcomeV1, GenericLineageStoreErrorV1> {
    let candidate_fingerprint = DefinitionFingerprint::parse(&document.candidate_fingerprint)
        .map_err(|_| control_error("candidate fingerprint is invalid"))?;
    let target = ArtifactTargetV1::parse(kind_ref, instance_id)
        .map_err(|error| control_error(format!("target resolution failed: {error}")))?;
    let context = repository
        .operation_context_under_lock(target.kind_ref(), target.instance_id())
        .map_err(|error| control_error(format!("operation context failed: {error}")))?;
    let expected_candidate_ref = format!(
        ".handbook/evidence/artifacts/{}/candidates/candidate_{}.json",
        target.instance_id().as_str(),
        fingerprint_hex(&candidate_fingerprint)
    );
    if document.candidate_ref != expected_candidate_ref {
        return Err(control_error(
            "candidate ref does not bind the requested target and fingerprint",
        ));
    }
    let store = GenericArtifactLineageStoreV1::new(repository.repo_root());
    let candidate_bytes = store.read_committed_authoritative_during_evaluation(
        &document.candidate_ref,
        &document.candidate_fingerprint,
    )?;
    let candidate = parse_runtime_record(
        &candidate_bytes,
        "handbook.artifact-candidate",
        "1.4",
        "candidate_id",
        "candidate_fingerprint",
        "candidate",
        &document.candidate_fingerprint,
    )?;
    require_exact_object_fields(
        &candidate,
        &[
            "schema_id",
            "schema_version",
            "intake_record_ref",
            "intake_record_fingerprint",
            "target_kind_ref",
            "target_instance_id",
            "target_schema_ref",
            "profile_ref",
            "resolved_profile_fingerprint",
            "operation_context_fingerprint",
            "normalized_content_ref",
            "normalized_content_fingerprint",
            "field_sources",
            "validation_result_refs",
            "unresolved_coverage_ids",
            "promotion_eligibility",
            "required_approval_policy_ref",
            "basis_artifact_fingerprint",
            "candidate_id",
            "candidate_fingerprint",
        ],
        "candidate record",
    )?;
    let intake_record_ref = string_field(&candidate, "intake_record_ref")?;
    let intake_record_fingerprint = string_field(&candidate, "intake_record_fingerprint")?;
    let preview = candidate_preview(
        repository,
        kind_ref,
        instance_id,
        intake_record_ref,
        intake_record_fingerprint,
        None,
    )?;
    if candidate != preview.candidate
        || document.candidate_fingerprint != preview.candidate_fingerprint
        || document.candidate_ref != preview.candidate_ref
    {
        return Err(control_error(
            "retained candidate does not independently reproduce under current authority",
        ));
    }

    let expected = document
        .expected_current_artifact_fingerprint
        .as_deref()
        .map(|value| {
            DefinitionFingerprint::parse(value)
                .map_err(|_| control_error("expected current fingerprint is invalid"))
        })
        .transpose()?;
    let candidate_basis = preview
        .basis_artifact_fingerprint
        .as_deref()
        .map(|value| {
            DefinitionFingerprint::parse(value)
                .map_err(|_| control_error("candidate basis fingerprint is invalid"))
        })
        .transpose()?;
    if expected != candidate_basis {
        return Ok(GenericPlannedOutcomeV1::Refuse(GenericRefusalPlanV1 {
            operation_context_fingerprint: context.context_fingerprint().to_string(),
            expected_basis_fingerprint: expected.as_ref().map(ToString::to_string),
            refusal: EstablishedRefusalV1 {
                code: EstablishedRefusalCodeV1::StaleCurrentArtifact,
                layer: GenericRefusalLayerV1::Currentness,
                expected_fingerprint: expected.as_ref().map(ToString::to_string),
                observed_fingerprint: candidate_basis.as_ref().map(ToString::to_string),
            },
        }));
    }

    let retained_content = store.read_committed_closure_during_evaluation(
        &document.candidate_ref,
        &document.candidate_fingerprint,
        &preview.normalized_content_ref,
        &preview.normalized_content_fingerprint,
    )?;
    if retained_content != jcs_lf(&preview.normalized_content)? {
        return Err(control_error(
            "retained candidate content does not match the reproduced preview",
        ));
    }
    let retained_validation = store.read_committed_closure_during_evaluation(
        &document.candidate_ref,
        &document.candidate_fingerprint,
        &preview.validation_result_ref,
        &preview.validation_result_fingerprint,
    )?;
    if retained_validation != jcs_lf(&preview.validation_result)? {
        return Err(control_error(
            "retained validation result does not match the reproduced preview",
        ));
    }

    let canonical_bytes = canonical_yaml_bytes(&preview.normalized_content)
        .map_err(|error| control_error(format!("canonical YAML emission failed: {error}")))?;
    let reparsed = parse_canonical_yaml(&canonical_bytes)
        .map_err(|error| control_error(format!("canonical YAML reparse failed: {error}")))?;
    if reparsed != preview.normalized_content {
        return Err(control_error(
            "canonical YAML does not round-trip to candidate content",
        ));
    }
    repository
        .registry()
        .validate_json(target.instance_id(), &reparsed)
        .map_err(|_| control_error("promoted canonical YAML failed structural validation"))?;
    let canonical_fingerprint = DefinitionFingerprint::from_bytes(&canonical_bytes);
    let canonical_ref = repository
        .canonical_path_under_lock(&target)
        .map_err(|error| control_error(format!("canonical target failed: {error}")))?;
    let resolved_definitions = context
        .resolved_definitions()
        .iter()
        .map(|binding| {
            json!({
                "definition_ref": binding.definition_ref.as_str(),
                "definition_fingerprint": binding.definition_fingerprint.as_str(),
            })
        })
        .collect::<Vec<_>>();
    let promotion_subject = json!({
        "schema_id": "handbook.artifact-promotion-record",
        "schema_version": "1.2",
        "candidate_ref": document.candidate_ref,
        "candidate_fingerprint": document.candidate_fingerprint,
        "target_instance_id": target.instance_id().as_str(),
        "expected_current_artifact_fingerprint": expected.as_ref().map(DefinitionFingerprint::as_str),
        "profile_ref": context.profile_ref().as_str(),
        "resolved_profile_fingerprint": context.resolved_profile_fingerprint().as_str(),
        "operation_context_fingerprint": context.context_fingerprint().as_str(),
        "resolved_definitions": resolved_definitions,
        "approval_refs": [],
        "validation_result_refs": [preview.validation_result_ref],
        "decision": "not_required",
        "authorized_by_ref": null,
        "canonical_artifact_ref": canonical_ref,
        "canonical_artifact_fingerprint": canonical_fingerprint.as_str(),
    });
    let promotion = finalize_record(
        promotion_subject,
        "promotion_id",
        "promotion_fingerprint",
        "promotion",
    )?;
    Ok(GenericPlannedOutcomeV1::Commit(GenericCommitPlanV1 {
        operation_context_fingerprint: context.context_fingerprint().to_string(),
        expected_basis_fingerprint: expected.as_ref().map(ToString::to_string),
        sampled_finalized_at_utc: None,
        outputs: vec![
            GenericPlannedOutputV1 {
                token: "canonical-artifact".to_string(),
                authority_class: GenericAuthorityClassV1::CanonicalTruth,
                final_ref: canonical_ref.to_string(),
                bytes: canonical_bytes,
                output_fingerprint: canonical_fingerprint.to_string(),
                install_mode: GenericInstallModeV1::ReplaceIfCurrent,
                receipt_class: Some(GenericReceiptClassV1::CanonicalTruth),
            },
            GenericPlannedOutputV1 {
                token: "promotion-record".to_string(),
                authority_class: GenericAuthorityClassV1::SemanticRecord,
                final_ref: format!(
                    ".handbook/state/artifacts/{}/promotion-records/{}.json",
                    target.instance_id().as_str(),
                    promotion.id
                ),
                bytes: promotion.bytes,
                output_fingerprint: promotion.fingerprint,
                install_mode: GenericInstallModeV1::CreateNew,
                receipt_class: Some(GenericReceiptClassV1::SemanticRecord),
            },
        ],
    }))
}

fn parse_intake_append(bytes: &[u8]) -> Result<IntakeAppendDocumentV1, ArtifactMutationErrorV1> {
    require_mutation_input_document_bound(bytes)?;
    let value = parse_definition_yaml(bytes).map_err(|_| {
        mutation_error(
            ArtifactMutationErrorKindV1::InvalidRequest,
            "intake append request is not one duplicate-safe document",
        )
    })?;
    require_exact_fields(
        &value,
        &[
            "idempotency_key",
            "acquisition_mode",
            "expected_current_artifact_fingerprint",
            "coverage_submissions",
        ],
    )?;
    serde_json::from_value(value).map_err(|_| {
        mutation_error(
            ArtifactMutationErrorKindV1::InvalidRequest,
            "intake append request does not match the closed request shape",
        )
    })
}

fn build_intake_plan(
    repository: &ArtifactRepositoryV1,
    kind_ref: &str,
    instance_id: &str,
    document: &IntakeAppendDocumentV1,
    evaluation_context: GenericEvaluationContextV1,
) -> Result<GenericPlannedOutcomeV1, GenericLineageStoreErrorV1> {
    let target = ArtifactTargetV1::parse(kind_ref, instance_id)
        .map_err(|error| control_error(format!("target resolution failed: {error}")))?;
    let context = repository
        .operation_context_under_lock(target.kind_ref(), target.instance_id())
        .map_err(|error| control_error(format!("operation context failed: {error}")))?;
    let expected =
        parse_nullable_fingerprint(document.expected_current_artifact_fingerprint.as_deref())?;
    let observed = repository
        .current_artifact_fingerprint_under_lock(&target)
        .map_err(|error| control_error(format!("current artifact observation failed: {error}")))?;
    if expected != observed {
        return Ok(stale_current_refusal(
            &context,
            expected.as_ref(),
            observed.as_ref(),
        ));
    }
    let evaluation = match repository.evaluate_intake_under_lock(
        target.kind_ref(),
        target.instance_id(),
        document.acquisition_mode,
        expected.clone(),
        &document.coverage_submissions,
    ) {
        Ok(evaluation) => evaluation,
        Err(error)
            if error.kind()
                == crate::artifact_repository::ArtifactRepositoryErrorKindV1::StructuralValidation =>
        {
            return Ok(GenericPlannedOutcomeV1::Refuse(GenericRefusalPlanV1 {
                operation_context_fingerprint: context.context_fingerprint().to_string(),
                expected_basis_fingerprint: expected.as_ref().map(ToString::to_string),
                refusal: EstablishedRefusalV1 {
                    code: EstablishedRefusalCodeV1::StructuralValidationFailed,
                    layer: GenericRefusalLayerV1::Structural,
                    expected_fingerprint: None,
                    observed_fingerprint: None,
                },
            }))
        }
        Err(_) => {
            return Ok(GenericPlannedOutcomeV1::Refuse(GenericRefusalPlanV1 {
                operation_context_fingerprint: context.context_fingerprint().to_string(),
                expected_basis_fingerprint: expected.as_ref().map(ToString::to_string),
                refusal: EstablishedRefusalV1 {
                    code: EstablishedRefusalCodeV1::IntakeCoverageBlocked,
                    layer: GenericRefusalLayerV1::Intake,
                    expected_fingerprint: None,
                    observed_fingerprint: None,
                },
            }))
        }
    };
    if evaluation.outcome == CoverageEvaluationOutcomeV1::Blocked {
        return Ok(GenericPlannedOutcomeV1::Refuse(GenericRefusalPlanV1 {
            operation_context_fingerprint: context.context_fingerprint().to_string(),
            expected_basis_fingerprint: expected.as_ref().map(ToString::to_string),
            refusal: EstablishedRefusalV1 {
                code: EstablishedRefusalCodeV1::IntakeCoverageBlocked,
                layer: GenericRefusalLayerV1::Intake,
                expected_fingerprint: None,
                observed_fingerprint: None,
            },
        }));
    }
    if repository
        .registry()
        .validate_json(target.instance_id(), &evaluation.normalized_content)
        .is_err()
    {
        return Ok(GenericPlannedOutcomeV1::Refuse(GenericRefusalPlanV1 {
            operation_context_fingerprint: context.context_fingerprint().to_string(),
            expected_basis_fingerprint: expected.as_ref().map(ToString::to_string),
            refusal: EstablishedRefusalV1 {
                code: EstablishedRefusalCodeV1::StructuralValidationFailed,
                layer: GenericRefusalLayerV1::Structural,
                expected_fingerprint: None,
                observed_fingerprint: None,
            },
        }));
    }
    let finalized_at_utc = evaluation_context
        .sampled_finalized_at_utc
        .ok_or_else(|| control_error("intake append timestamp was not sampled"))?;
    intake_commit_plan(repository, &target, &context, &evaluation, finalized_at_utc)
}

fn intake_commit_plan(
    repository: &ArtifactRepositoryV1,
    target: &ArtifactTargetV1,
    context: &crate::artifact_operation_context::ArtifactOperationContextV1,
    evaluation: &CoverageEvaluationV1,
    finalized_at_utc: String,
) -> Result<GenericPlannedOutcomeV1, GenericLineageStoreErrorV1> {
    let definition = repository
        .intake_definition_under_lock(target)
        .map_err(|error| control_error(format!("intake definition failed: {error}")))?;
    let kind = repository
        .registry()
        .kind(target.kind_ref())
        .ok_or_else(|| control_error("selected kind disappeared"))?;
    let mut outputs = Vec::new();
    let mut retained_coverage = Vec::new();
    let mut emitted_values = std::collections::BTreeSet::new();
    let mut emitted_tokens = std::collections::BTreeSet::new();
    for result in &evaluation.coverage_results {
        let value = result
            .value
            .as_ref()
            .ok_or_else(|| control_error("satisfied coverage omitted its value"))?;
        let fingerprint = DefinitionFingerprint::from_json_value(value)
            .map_err(|_| control_error("coverage value fingerprint failed"))?;
        let relative_ref = format!(
            ".handbook/evidence/artifacts/{}/intake-values/value_{}.json",
            target.instance_id().as_str(),
            fingerprint_hex(&fingerprint)
        );
        let mut token = result
            .coverage_id
            .rsplit('.')
            .next()
            .map(|suffix| format!("{}-value", suffix.replace('_', "-")))
            .ok_or_else(|| control_error("coverage ID token is absent"))?;
        if !emitted_tokens.insert(token.clone()) {
            token = format!("coverage-{:04}-value", outputs.len());
            if !emitted_tokens.insert(token.clone()) {
                return Err(control_error("coverage output token collision"));
            }
        }
        if emitted_values.insert(fingerprint.clone()) {
            outputs.push(GenericPlannedOutputV1 {
                token,
                authority_class: GenericAuthorityClassV1::SubordinateClosure,
                final_ref: relative_ref.clone(),
                bytes: jcs_lf(value)?,
                output_fingerprint: fingerprint.to_string(),
                install_mode: GenericInstallModeV1::CreateNew,
                receipt_class: None,
            });
        }
        retained_coverage.push(json!({
            "coverage_id": result.coverage_id,
            "applicability": "applicable",
            "source_kind": "user_declaration",
            "value_ref": relative_ref,
            "value_fingerprint": fingerprint.as_str(),
            "evidence_refs": [],
            "confidence": "high",
            "freshness": null,
            "sensitivity": "internal",
            "evaluation": "satisfied",
            "contradiction_refs": [],
            "waiver_ref": null,
        }));
    }
    if outputs.is_empty() || outputs.len() > 15 {
        return Err(control_error(
            "intake distinct value closure is outside 1..=15 outputs",
        ));
    }
    let subject = json!({
        "schema_id": "handbook.artifact-intake-record",
        "schema_version": "1.2",
        "intake_definition_ref": definition.exact_ref().as_str(),
        "intake_definition_fingerprint": definition.definition_fingerprint().as_str(),
        "acquisition_mode": mode_name(evaluation.acquisition_mode),
        "target_kind_ref": target.kind_ref().as_str(),
        "target_instance_id": target.instance_id().as_str(),
        "target_schema_ref": kind.canonical_schema_ref().as_str(),
        "profile_ref": context.profile_ref().as_str(),
        "resolved_profile_fingerprint": context.resolved_profile_fingerprint().as_str(),
        "operation_context_fingerprint": context.context_fingerprint().as_str(),
        "consumer": {
            "kind": "handbook_cli",
            "id": "handbook",
            "version": engine_release_version()?,
        },
        "coverage_results": retained_coverage,
        "basis_artifact_fingerprint": evaluation.basis_artifact_fingerprint.as_ref().map(DefinitionFingerprint::as_str),
        "prompt_event_refs": [],
        "finalized_at_utc": finalized_at_utc,
    });
    let record = finalize_record(subject, "intake_record_id", "record_fingerprint", "intake")?;
    outputs.push(GenericPlannedOutputV1 {
        token: "intake-record".to_string(),
        authority_class: GenericAuthorityClassV1::SemanticRecord,
        final_ref: format!(
            ".handbook/state/artifacts/{}/intake-records/{}.json",
            target.instance_id().as_str(),
            record.id
        ),
        bytes: record.bytes,
        output_fingerprint: record.fingerprint,
        install_mode: GenericInstallModeV1::CreateNew,
        receipt_class: Some(GenericReceiptClassV1::SemanticRecord),
    });
    Ok(GenericPlannedOutcomeV1::Commit(GenericCommitPlanV1 {
        operation_context_fingerprint: context.context_fingerprint().to_string(),
        expected_basis_fingerprint: evaluation
            .basis_artifact_fingerprint
            .as_ref()
            .map(ToString::to_string),
        sampled_finalized_at_utc: Some(finalized_at_utc),
        outputs,
    }))
}

struct FinalizedRecord {
    id: String,
    fingerprint: String,
    bytes: Vec<u8>,
}

fn finalize_record(
    mut subject: Value,
    id_field: &str,
    fingerprint_field: &str,
    id_prefix: &str,
) -> Result<FinalizedRecord, GenericLineageStoreErrorV1> {
    let fingerprint = DefinitionFingerprint::from_json_value(&subject)
        .map_err(|_| control_error("runtime record fingerprint failed"))?;
    let id = format!("{id_prefix}_{}", fingerprint_hex(&fingerprint));
    let object = subject
        .as_object_mut()
        .ok_or_else(|| control_error("runtime record subject is not an object"))?;
    object.insert(id_field.to_string(), Value::String(id.clone()));
    object.insert(
        fingerprint_field.to_string(),
        Value::String(fingerprint.to_string()),
    );
    Ok(FinalizedRecord {
        id,
        fingerprint: fingerprint.to_string(),
        bytes: jcs_lf(&subject)?,
    })
}

fn stale_current_refusal(
    context: &crate::artifact_operation_context::ArtifactOperationContextV1,
    expected: Option<&DefinitionFingerprint>,
    observed: Option<&DefinitionFingerprint>,
) -> GenericPlannedOutcomeV1 {
    GenericPlannedOutcomeV1::Refuse(GenericRefusalPlanV1 {
        operation_context_fingerprint: context.context_fingerprint().to_string(),
        expected_basis_fingerprint: expected.map(ToString::to_string),
        refusal: EstablishedRefusalV1 {
            code: EstablishedRefusalCodeV1::StaleCurrentArtifact,
            layer: GenericRefusalLayerV1::Currentness,
            expected_fingerprint: expected.map(ToString::to_string),
            observed_fingerprint: observed.map(ToString::to_string),
        },
    })
}

fn parse_nullable_fingerprint(
    value: Option<&str>,
) -> Result<Option<DefinitionFingerprint>, GenericLineageStoreErrorV1> {
    value
        .map(|value| {
            DefinitionFingerprint::parse(value)
                .map_err(|_| control_error("expected current fingerprint is invalid"))
        })
        .transpose()
}

fn repository_authority_error(
    error: crate::artifact_repository::ArtifactRepositoryErrorV1,
) -> ArtifactMutationErrorV1 {
    mutation_error(
        ArtifactMutationErrorKindV1::RepositoryIdentity,
        format!("repository authority refused: {error}"),
    )
}

fn require_exact_fields(value: &Value, fields: &[&str]) -> Result<(), ArtifactMutationErrorV1> {
    let object = value.as_object().ok_or_else(|| {
        mutation_error(
            ArtifactMutationErrorKindV1::InvalidRequest,
            "mutation request must be one object",
        )
    })?;
    if object.len() != fields.len() || fields.iter().any(|field| !object.contains_key(*field)) {
        return Err(mutation_error(
            ArtifactMutationErrorKindV1::InvalidRequest,
            "mutation request field set is not exact",
        ));
    }
    Ok(())
}

fn require_mutation_input_document_bound(bytes: &[u8]) -> Result<(), ArtifactMutationErrorV1> {
    if bytes.len() > MAX_ARTIFACT_INPUT_DOCUMENT_BYTES {
        return Err(mutation_error(
            ArtifactMutationErrorKindV1::InvalidRequest,
            "mutation request exceeds the 1 MiB limit",
        ));
    }
    Ok(())
}

fn engine_release_version() -> Result<&'static str, GenericLineageStoreErrorV1> {
    let version = include_str!("../../../VERSION").trim();
    if version.is_empty() {
        return Err(control_error("engine release version is empty"));
    }
    Ok(version)
}

fn mode_name(mode: AcquisitionModeV1) -> &'static str {
    match mode {
        AcquisitionModeV1::GuidedAdaptive => "guided_adaptive",
        AcquisitionModeV1::Express => "express",
        AcquisitionModeV1::AgentAssisted => "agent_assisted",
    }
}

fn fingerprint_hex(fingerprint: &DefinitionFingerprint) -> &str {
    fingerprint
        .as_str()
        .strip_prefix("sha256:")
        .expect("DefinitionFingerprint always has the sha256 prefix")
}

fn jcs_lf(value: &Value) -> Result<Vec<u8>, GenericLineageStoreErrorV1> {
    let mut bytes = serde_json_canonicalizer::to_vec(value)
        .map_err(|_| control_error("RFC 8785 serialization failed"))?;
    bytes.push(b'\n');
    Ok(bytes)
}

fn control_error(detail: impl Into<String>) -> GenericLineageStoreErrorV1 {
    GenericLineageStoreErrorV1::new(GenericLineageStoreErrorKindV1::InvalidPlan, detail)
}

fn mutation_error(
    kind: ArtifactMutationErrorKindV1,
    detail: impl Into<String>,
) -> ArtifactMutationErrorV1 {
    ArtifactMutationErrorV1 {
        kind,
        detail: detail.into(),
    }
}

pub fn execution_disposition_name(disposition: GenericExecutionDispositionV1) -> &'static str {
    match disposition {
        GenericExecutionDispositionV1::Committed => "committed",
        GenericExecutionDispositionV1::Refused => "refused",
        GenericExecutionDispositionV1::Replayed => "replayed",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn padded_document(value: Value, length: usize) -> Vec<u8> {
        let mut bytes = serde_json::to_vec(&value).expect("test document JSON");
        assert!(bytes.len() <= length);
        bytes.resize(length, b' ');
        bytes
    }

    #[test]
    fn mutation_parsers_accept_the_exact_shared_limit_and_refuse_one_byte_more() {
        let documents = [
            (
                "intake",
                json!({
                    "idempotency_key": "bounded_intake_0001",
                    "acquisition_mode": "express",
                    "expected_current_artifact_fingerprint": null,
                    "coverage_submissions": []
                }),
            ),
            (
                "candidate",
                json!({
                    "idempotency_key": "bounded_candidate_0001",
                    "intake_record_ref": ".handbook/state/artifacts/example/intake-records/intake_example.json",
                    "intake_record_fingerprint": "sha256:1111111111111111111111111111111111111111111111111111111111111111",
                    "expected_candidate_fingerprint": "sha256:2222222222222222222222222222222222222222222222222222222222222222"
                }),
            ),
            (
                "promotion",
                json!({
                    "idempotency_key": "bounded_promotion_0001",
                    "candidate_ref": ".handbook/state/artifacts/example/candidates/candidate_example.json",
                    "candidate_fingerprint": "sha256:3333333333333333333333333333333333333333333333333333333333333333",
                    "expected_current_artifact_fingerprint": null
                }),
            ),
        ];
        for (operation, document) in documents {
            let exact = padded_document(document.clone(), MAX_ARTIFACT_INPUT_DOCUMENT_BYTES);
            let over = padded_document(document, MAX_ARTIFACT_INPUT_DOCUMENT_BYTES + 1);
            let (accepted, refused) = match operation {
                "intake" => (
                    parse_intake_append(&exact).is_ok(),
                    parse_intake_append(&over).unwrap_err(),
                ),
                "candidate" => (
                    parse_candidate_append(&exact).is_ok(),
                    parse_candidate_append(&over).unwrap_err(),
                ),
                "promotion" => (
                    parse_promotion(&exact).is_ok(),
                    parse_promotion(&over).unwrap_err(),
                ),
                _ => unreachable!(),
            };
            assert!(accepted, "operation: {operation}");
            assert_eq!(
                refused.kind(),
                ArtifactMutationErrorKindV1::InvalidRequest,
                "operation: {operation}"
            );
        }
    }
}
