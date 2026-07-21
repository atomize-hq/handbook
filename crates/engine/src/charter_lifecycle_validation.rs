use crate::canonical_repo_support::CanonicalWorkspace;
use crate::charter_authority_transaction::{
    CharterAuthorityTransactionServiceV1, CommittedCharterAuthorityV1,
};
use crate::charter_intake::{
    validate_candidate_field_source_bijection, CharterCandidateBundle,
    CharterCandidateEvaluationV12, CharterCandidateV12, CharterValidationResultBindingV13,
    COVERAGE_ORDER,
};
use crate::charter_lifecycle::CharterLifecycleState;
use crate::charter_lifecycle_store::{CharterLifecycleStoreV1, RetainedLifecycleAuthorityV1};
use crate::charter_lineage_store::{
    create_new_file, create_safe_directories, reject_reparse_or_symlink, sync_directory,
    validate_record, CandidateBundlePersistenceV1, LineageRecordClassV1, TrustedLineageStoreV1,
};
use crate::profile_builtins;
use crate::semantic_capability_registry::SemanticCapabilityRegistry;
use crate::{
    load_shipped_charter_definition_registry, parse_schema_json, DefinitionFingerprint,
    ExactDefinitionRef, ResolvedProfileDecisions, SymbolicId,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::fmt;
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use std::time::{SystemTime, UNIX_EPOCH};

pub const MAX_LIFECYCLE_VALIDATION_RESULT_BYTES: usize = 262_144;
const RESULT_PARTITION: &str = "lifecycle-validation-results";
const CANDIDATE_INVENTORY_ROOT: &str = ".handbook/evidence/charter/candidates";
const RESULT_INVENTORY_ROOT: &str = ".handbook/state/lifecycle-validation-results";
const MAX_INVENTORY_ENTRIES: usize = 4_096;
const MAX_INVENTORY_FILE_BYTES: usize = 262_144;
const MAX_INVENTORY_AGGREGATE_BYTES: u64 = 1_073_741_824;
const MAX_INVENTORY_FILENAME_BYTES: usize = 128;
const CANONICAL_CHARTER_REF: &str = ".handbook/project/charter.yaml";
const SELECTED_PROFILE_REF: &str = "handbook.profile.shipped-root@1.1.0";
const SELECTED_PROFILE_FINGERPRINT: &str =
    "sha256:6a7b41befa77b999b9ee20f513636051726a8401a81bf2f369501e8f3dd4fa74";
const LIFECYCLE_POLICY_REF: &str = "handbook.lifecycle.constitutional-review-lock@1.0.0";
const LIFECYCLE_POLICY_FINGERPRINT: &str =
    "sha256:88caafb9caaf137647c42a91cd2762ac0871e0a20e2a1844c2c0076d5fb43cc3";
const LIFECYCLE_VALIDATION_RESULT_SCHEMA_V10: &[u8] =
    include_bytes!("contracts/lifecycle-validation-result-1.0.0.schema.json");
static LIFECYCLE_VALIDATION_RESULT_VALIDATOR_V10: OnceLock<Result<jsonschema::Validator, String>> =
    OnceLock::new();

const DEFINITION_BINDINGS: [(&str, &str, &str); 13] = [
    (
        "approval_policy",
        "handbook.approval.constitutional-candidate@1.0.0",
        "sha256:67466e0c26e48e7e25705b6a601e487b74923371d886d759fcd9face2e793c29",
    ),
    (
        "capability_contract",
        "handbook.capabilities.constitutional-root@1.0.0",
        "sha256:1d4a1c2f85158c14524559e6846bf805eff4e531d8a78ac5c4890e2a4c0b0998",
    ),
    (
        "intake_definition",
        "handbook.intake.charter@1.0.0",
        "sha256:a92229722f25119c7d91137e1feef4ce51b88ae766ce308b585d37f39eb52d1c",
    ),
    (
        "lifecycle_policy",
        LIFECYCLE_POLICY_REF,
        LIFECYCLE_POLICY_FINGERPRINT,
    ),
    (
        "reassessment_trigger",
        "handbook.intake-trigger.production-posture-changed@1.0.0",
        "sha256:35df6e191f201517abd07f428b87b9e3970117fb70110b801947116cb04c69f6",
    ),
    (
        "reassessment_trigger",
        "handbook.intake-trigger.trust-boundary-changed@1.0.0",
        "sha256:12950ab5172acce8b0b944f630791a4ffda4960095248fa3b5bea833f7fa4766",
    ),
    (
        "renderer_definition",
        "handbook.renderer.charter-review-markdown@1.0.0",
        "sha256:68f6fceedaab6133364d18a2b474694d1b44b91d73c05d0af65db1aa58e80fa7",
    ),
    (
        "review_trigger",
        "handbook.lifecycle-trigger.charter-amendment-proposed@1.0.0",
        "sha256:9672246337ff266fc07f67053ca053736cb0650d2ad5e053e4a19304acd48ed8",
    ),
    (
        "semantic_validator",
        "handbook.semantic-validation.constitutional-root@1.0.0",
        "sha256:be0fb9fd4ee98e9fc1c384b710d61198e103f2bca6ac6ef2bbe14957808c9738",
    ),
    (
        "semantic_validator",
        "handbook.semantic-validation.constitutional-root@1.1.0",
        "sha256:10703119fbb0a4cfcab3fcdc4c618df269933591caf653786fdd8fee8cfc3c10",
    ),
    (
        "target_kind",
        "handbook.artifact-kind.project-authority@1.1.0",
        "sha256:3b3d0b353d9c45c20781c3f4b79e30cc27847fb168d1860115f2f941b8bbef0e",
    ),
    (
        "target_schema",
        "handbook.schemas.artifacts.project-authority@1.1.0",
        "sha256:7420efe464c45e17319c56a233f9a54960c52f81b502ed4ffb59a479474f9836",
    ),
    (
        "waiver_policy",
        "handbook.waiver.constitutional-intake@1.0.0",
        "sha256:704acdfe61b4d76ece2ceac72eb99df65dff2d2998b876990f24c3410fc317e3",
    ),
];

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LifecycleDefinitionBindingV10 {
    pub definition_role: String,
    pub definition_ref: String,
    pub definition_fingerprint: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LifecycleActiveObservationV10 {
    pub observation_ref: String,
    pub observation_fingerprint: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CharterLifecycleValidationResultV10 {
    pub schema_id: String,
    pub schema_version: String,
    pub validation_result_id: String,
    pub candidate_subject_fingerprint: String,
    pub intake_record_ref: String,
    pub intake_record_fingerprint: String,
    pub normalized_content_ref: String,
    pub normalized_content_fingerprint: String,
    pub target_instance_id: String,
    pub canonical_artifact_ref: String,
    pub basis_artifact_fingerprint: Option<String>,
    pub observed_current_artifact_fingerprint: Option<String>,
    pub profile_ref: String,
    pub resolved_profile_fingerprint: String,
    pub resolved_definitions: Vec<LifecycleDefinitionBindingV10>,
    pub lifecycle_policy_ref: String,
    pub lifecycle_policy_fingerprint: String,
    pub lifecycle_head_ref: Option<String>,
    pub lifecycle_head_fingerprint: Option<String>,
    pub lifecycle_state: String,
    pub lifecycle_state_fingerprint: Option<String>,
    pub active_observations: Vec<LifecycleActiveObservationV10>,
    pub reopened_coverage_ids: Vec<String>,
    pub validation_status: String,
    pub validated_at_utc: String,
    pub validation_result_fingerprint: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LifecycleValidationErrorKindV1 {
    AuthorityMismatch,
    InvalidResult,
    ExistingBytesMismatch,
    UnsafeFilesystem,
    IoFailure,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LifecycleValidationErrorV1 {
    kind: LifecycleValidationErrorKindV1,
    detail: String,
}

impl LifecycleValidationErrorV1 {
    fn new(kind: LifecycleValidationErrorKindV1, detail: impl Into<String>) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }

    pub fn kind(&self) -> LifecycleValidationErrorKindV1 {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

impl fmt::Display for LifecycleValidationErrorV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.detail)
    }
}

impl std::error::Error for LifecycleValidationErrorV1 {}

#[derive(Clone, Debug)]
pub(crate) struct CharterLifecycleValidationServiceV1 {
    repo_root: PathBuf,
}

pub(crate) struct RetainedLifecycleValidationV1 {
    pub(crate) relative_ref: String,
    pub(crate) bytes: Vec<u8>,
}

pub(crate) struct FinalizedCandidatePublicationV1 {
    pub(crate) bundle: CharterCandidateBundle,
    pub(crate) persistence: CandidateBundlePersistenceV1,
}

impl CharterLifecycleValidationServiceV1 {
    pub(crate) fn new(repo_root: impl AsRef<Path>) -> Self {
        Self {
            repo_root: repo_root.as_ref().to_path_buf(),
        }
    }

    pub(crate) fn finalize(
        &self,
        decisions: &ResolvedProfileDecisions,
        evaluation: CharterCandidateEvaluationV12,
    ) -> Result<FinalizedCandidatePublicationV1, LifecycleValidationErrorV1> {
        validate_definition_authority(decisions)?;
        let _author_lock = AuthorLockV1::acquire(&self.repo_root)?;
        let transactions = CharterAuthorityTransactionServiceV1::new(&self.repo_root);
        let retained = transactions.begin_retained_authority().map_err(|failure| {
            authority_error(format!(
                "promotion authority recovery/retention refused: {}",
                failure.detail()
            ))
        })?;
        let current = retained.read_committed_charter().map_err(|failure| {
            authority_error(format!(
                "committed Charter observation refused: {}",
                failure.detail()
            ))
        })?;
        let lifecycle = CharterLifecycleStoreV1::new(&self.repo_root)
            .observe_retained_locked()
            .map_err(|failure| {
                authority_error(format!(
                    "retained lifecycle observation refused: {}",
                    failure.detail()
                ))
            })?;
        let mut result = build_result(decisions, &evaluation, current.as_ref(), &lifecycle)?;
        let validation_result_ref = result_ref(&result.validation_result_fingerprint);
        let second_inventory = inventory_author_authority_stable(&self.repo_root)?;
        let replay = classify_author_replay(
            &second_inventory,
            &evaluation.candidate_subject.candidate_subject_fingerprint,
            &validation_result_ref,
            &result.validation_result_fingerprint,
        )?;
        let lineage = TrustedLineageStoreV1::new(&self.repo_root);
        let (candidate, persistence) = match replay {
            AuthorReplayV1::FirstAuthoring => {
                result.validated_at_utc = current_utc();
                validate_result_record(&result, Some(&validation_result_ref))?;
                let result_bytes = encode_result(&result)?;
                self.publish_result_create_new(&validation_result_ref, &result_bytes)?;
                let binding = result_binding(&validation_result_ref, &result, &result_bytes)?;
                let candidate = finalize_candidate(&evaluation, binding)?;
                #[cfg(test)]
                refuse_candidate_publication_after_result_for_test()?;
                let bundle = CharterCandidateBundle {
                    intake: evaluation.intake.clone(),
                    candidate: candidate.clone(),
                    normalized_content: evaluation.normalized_content.clone(),
                    charter: evaluation.charter.clone(),
                };
                let persistence = lineage
                    .persist_candidate_bundle(&bundle)
                    .map_err(|failure| {
                        authority_error(format!(
                            "candidate publication refused after exact result publication: {}",
                            failure.detail()
                        ))
                    })?;
                (candidate, persistence)
            }
            AuthorReplayV1::Exact {
                candidate,
                result_bytes,
            } => {
                let candidate = *candidate;
                let binding = candidate.validation_result_binding.clone();
                validate_bound_result_bytes(
                    &result_bytes,
                    &binding,
                    &evaluation.candidate_subject.candidate_subject_fingerprint,
                )?;
                let content = lineage
                    .read_candidate_content(&candidate.normalized_content_ref)
                    .map_err(|failure| {
                        authority_error(format!(
                            "replay candidate content refused: {}",
                            failure.detail()
                        ))
                    })?;
                if content != evaluation.normalized_content {
                    return Err(authority_error(
                        "replay candidate content differs from recomputed normalized content",
                    ));
                }
                let intake_fingerprint = fingerprint_from_bound_ref(
                    &candidate.intake_record_ref,
                    "intake-records/intake_",
                    ".json",
                )?;
                lineage
                    .read_record(
                        LineageRecordClassV1::Intake,
                        &candidate.intake_record_ref,
                        &intake_fingerprint,
                    )
                    .map_err(|failure| {
                        authority_error(format!(
                            "replay intake authority refused: {}",
                            failure.detail()
                        ))
                    })?;
                let persistence = CandidateBundlePersistenceV1 {
                    normalized_content_ref: candidate.normalized_content_ref.clone(),
                    intake_ref: candidate.intake_record_ref.clone(),
                    candidate_ref: format!("candidates/{}.json", candidate.candidate_id),
                };
                (candidate, persistence)
            }
        };
        drop(retained);
        Ok(FinalizedCandidatePublicationV1 {
            bundle: CharterCandidateBundle {
                intake: evaluation.intake,
                candidate,
                normalized_content: evaluation.normalized_content,
                charter: evaluation.charter,
            },
            persistence,
        })
    }

    pub(crate) fn validate_current_retained(
        &self,
        decisions: &ResolvedProfileDecisions,
        candidate: &Value,
        normalized_content: &[u8],
        current: Option<&CommittedCharterAuthorityV1>,
        lifecycle: &RetainedLifecycleAuthorityV1,
    ) -> Result<RetainedLifecycleValidationV1, LifecycleValidationErrorV1> {
        validate_definition_authority(decisions)?;
        let candidate_object = candidate
            .as_object()
            .ok_or_else(|| authority_error("candidate authority is not an object"))?;
        let binding = parse_candidate_result_binding(candidate_object)?;
        let relative_ref = binding.validation_result_ref.clone();
        let bytes = read_result_file(&self.repo_root, &relative_ref)?;
        validate_bound_result_bytes(
            &bytes,
            &binding,
            candidate_string(candidate, "candidate_subject_fingerprint")?,
        )?;
        let result = validate_result_bytes(&bytes, Some(&relative_ref))?;

        let mut subject_preimage = candidate.clone();
        let subject = subject_preimage
            .as_object_mut()
            .expect("validated candidate object");
        for field in [
            "candidate_id",
            "candidate_fingerprint",
            "candidate_subject_fingerprint",
            "validation_result_binding",
        ] {
            subject.remove(field);
        }
        let candidate_subject_fingerprint =
            DefinitionFingerprint::from_json_value(&subject_preimage)
                .map_err(|_| authority_error("candidate subject cannot be recomputed"))?
                .to_string();
        if candidate_string(candidate, "candidate_subject_fingerprint")?
            != candidate_subject_fingerprint
            || result.candidate_subject_fingerprint != candidate_subject_fingerprint
        {
            return Err(authority_error(
                "candidate and lifecycle-validation result subject identities disagree",
            ));
        }

        let intake_ref = candidate_string(candidate, "intake_record_ref")?;
        let intake_fingerprint =
            fingerprint_from_bound_ref(intake_ref, "intake-records/intake_", ".json")?;
        let intake = TrustedLineageStoreV1::new(&self.repo_root)
            .read_record_by_ref(
                LineageRecordClassV1::Intake,
                intake_ref,
                Some(&intake_fingerprint),
            )
            .map_err(|failure| {
                authority_error(format!(
                    "candidate intake authority refused: {}",
                    failure.detail()
                ))
            })?;
        for field in [
            "target_kind_ref",
            "target_instance_id",
            "profile_ref",
            "resolved_profile_fingerprint",
            "basis_artifact_fingerprint",
        ] {
            if candidate.get(field) != intake.get(field) {
                return Err(authority_error(format!(
                    "candidate/intake `{field}` lineage is not exact-equal",
                )));
            }
        }
        validate_candidate_field_source_bijection(candidate, &intake, normalized_content).map_err(
            |failure| {
                authority_error(format!(
                    "candidate populated-leaf provenance replay refused: {}",
                    failure.detail()
                ))
            },
        )?;
        if result.intake_record_ref != intake_ref
            || result.intake_record_fingerprint != intake_fingerprint
            || result.normalized_content_ref
                != candidate_string(candidate, "normalized_content_ref")?
            || result.normalized_content_fingerprint
                != DefinitionFingerprint::from_bytes(normalized_content).to_string()
            || result.target_instance_id != candidate_string(candidate, "target_instance_id")?
            || result.profile_ref != candidate_string(candidate, "profile_ref")?
            || result.resolved_profile_fingerprint
                != candidate_string(candidate, "resolved_profile_fingerprint")?
        {
            return Err(authority_error(
                "lifecycle-validation result does not bind the exact intake/content/profile subject",
            ));
        }

        let observed = current.map(|authority| authority.canonical_fingerprint.clone());
        if candidate.get("basis_artifact_fingerprint")
            != Some(&observed.clone().map_or(Value::Null, Value::String))
            || result.basis_artifact_fingerprint != observed
            || result.observed_current_artifact_fingerprint != observed
        {
            return Err(authority_error(
                "lifecycle-validation result canonical basis is stale or transplanted",
            ));
        }
        match (
            current,
            lifecycle.authority.as_ref(),
            lifecycle.canonical_bytes.as_ref(),
        ) {
            (None, None, None) => {
                if result.lifecycle_state != "absent"
                    || result.lifecycle_head_ref.is_some()
                    || result.lifecycle_head_fingerprint.is_some()
                    || result.lifecycle_state_fingerprint.is_some()
                    || !result.active_observations.is_empty()
                    || !result.reopened_coverage_ids.is_empty()
                {
                    return Err(authority_error(
                        "create validation authority does not have the exact absent lifecycle closure",
                    ));
                }
            }
            (Some(current), Some(authority), Some(lifecycle_bytes))
                if current.canonical_bytes == *lifecycle_bytes
                    && current.canonical_fingerprint == authority.canonical_fingerprint =>
            {
                let expected_observations = authority
                    .active_observation_refs
                    .iter()
                    .zip(&authority.active_observation_fingerprints)
                    .map(|(observation_ref, observation_fingerprint)| {
                        LifecycleActiveObservationV10 {
                            observation_ref: observation_ref.clone(),
                            observation_fingerprint: observation_fingerprint.clone(),
                        }
                    })
                    .collect::<Vec<_>>();
                let reopened = authority
                    .reopened_coverage_ids
                    .iter()
                    .collect::<BTreeSet<_>>();
                if reopened.len() != authority.reopened_coverage_ids.len() {
                    return Err(authority_error(
                        "current lifecycle authority contains duplicate reopened coverage",
                    ));
                }
                let expected_reopened = COVERAGE_ORDER
                    .iter()
                    .filter(|coverage_id| reopened.contains(&coverage_id.to_string()))
                    .map(|coverage_id| (*coverage_id).to_owned())
                    .collect::<Vec<_>>();
                if expected_reopened.len() != reopened.len()
                    || result.lifecycle_head_ref.as_deref()
                        != Some(authority.lifecycle_transition_ref.as_str())
                    || result.lifecycle_head_fingerprint.as_deref()
                        != Some(authority.lifecycle_transition_fingerprint.as_str())
                    || result.lifecycle_state != lifecycle_state_name(authority.state)
                    || result.lifecycle_state_fingerprint.as_deref()
                        != Some(authority.state_fingerprint.as_str())
                    || result.active_observations != expected_observations
                    || result.reopened_coverage_ids != expected_reopened
                {
                    return Err(authority_error(
                        "lifecycle-validation result is stale, reordered, incomplete, or excess",
                    ));
                }
                for coverage_id in &expected_reopened {
                    let covered = intake["coverage_results"]
                        .as_array()
                        .is_some_and(|entries| {
                            entries.iter().any(|entry| {
                                entry["coverage_id"] == coverage_id.as_str()
                                    && matches!(
                                        entry["evaluation"].as_str(),
                                        Some("satisfied" | "waived")
                                    )
                            })
                        });
                    if !covered {
                        return Err(authority_error(format!(
                            "reopened lifecycle coverage `{coverage_id}` is absent from the retained intake",
                        )));
                    }
                }
            }
            _ => {
                return Err(authority_error(
                    "canonical promotion authority and lifecycle authority are inconsistent",
                ))
            }
        }
        Ok(RetainedLifecycleValidationV1 {
            relative_ref,
            bytes,
        })
    }

    fn publish_result_create_new(
        &self,
        relative_ref: &str,
        bytes: &[u8],
    ) -> Result<(), LifecycleValidationErrorV1> {
        let partition = self.repo_root.join(RESULT_INVENTORY_ROOT);
        create_safe_directories(&self.repo_root, &partition).map_err(|failure| {
            filesystem_error(format!(
                "validation-result partition creation refused: {}",
                failure.detail()
            ))
        })?;
        let path = self.repo_root.join(".handbook/state").join(relative_ref);
        let (temporary_path, mut file) = create_result_temporary(&partition)?;
        if let Err(failure) = file.write_all(bytes).and_then(|_| file.sync_all()) {
            let _ = fs::remove_file(&temporary_path);
            return Err(io_error(format!(
                "lifecycle-validation result write/fsync failed: {failure}"
            )));
        }
        drop(file);
        if let Err(failure) = rename_result_create_new(&temporary_path, &path) {
            let _ = fs::remove_file(&temporary_path);
            return Err(LifecycleValidationErrorV1::new(
                LifecycleValidationErrorKindV1::ExistingBytesMismatch,
                format!("lifecycle-validation result create-new publication refused: {failure}"),
            ));
        }
        sync_directory(&partition).map_err(|failure| {
            io_error(format!(
                "lifecycle-validation result parent fsync failed: {}",
                failure.detail()
            ))
        })?;
        if read_result_file(&self.repo_root, relative_ref)? != bytes {
            return Err(LifecycleValidationErrorV1::new(
                LifecycleValidationErrorKindV1::ExistingBytesMismatch,
                "published lifecycle-validation result bytes are not exact-equal",
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
thread_local! {
    static REFUSE_CANDIDATE_PUBLICATION_AFTER_RESULT: std::cell::Cell<bool> = const {
        std::cell::Cell::new(false)
    };
}

#[cfg(test)]
fn refuse_candidate_publication_after_result_for_test() -> Result<(), LifecycleValidationErrorV1> {
    let refuse = REFUSE_CANDIDATE_PUBLICATION_AFTER_RESULT.with(|selected| selected.replace(false));
    if refuse {
        return Err(authority_error(
            "test-only candidate publication refusal after exact result publication",
        ));
    }
    Ok(())
}

pub(crate) fn validate_candidate_exact_result_authority(
    repo_root: &Path,
    candidate: &Value,
) -> Result<RetainedLifecycleValidationV1, LifecycleValidationErrorV1> {
    let object = candidate
        .as_object()
        .ok_or_else(|| authority_error("candidate authority is not an object"))?;
    let binding = parse_candidate_result_binding(object)?;
    let subject = candidate_subject_fingerprint(candidate)?;
    if candidate_string(candidate, "candidate_subject_fingerprint")? != subject {
        return Err(authority_error(
            "candidate subject fingerprint does not independently recompute",
        ));
    }
    validate_candidate_field_source_authority(repo_root, candidate)?;
    let bytes = read_result_file(repo_root, &binding.validation_result_ref)?;
    validate_bound_result_bytes(&bytes, &binding, &subject)?;
    Ok(RetainedLifecycleValidationV1 {
        relative_ref: binding.validation_result_ref,
        bytes,
    })
}

fn validate_candidate_field_source_authority(
    repo_root: &Path,
    candidate: &Value,
) -> Result<(), LifecycleValidationErrorV1> {
    let lineage = TrustedLineageStoreV1::new(repo_root);
    let intake_ref = candidate_string(candidate, "intake_record_ref")?;
    let intake_fingerprint =
        fingerprint_from_bound_ref(intake_ref, "intake-records/intake_", ".json")?;
    let intake = lineage
        .read_record_by_ref(
            LineageRecordClassV1::Intake,
            intake_ref,
            Some(&intake_fingerprint),
        )
        .map_err(|failure| {
            authority_error(format!(
                "candidate retained intake provenance refused: {}",
                failure.detail()
            ))
        })?;
    let normalized_content_ref = candidate_string(candidate, "normalized_content_ref")?;
    let normalized_content = lineage
        .read_candidate_content(normalized_content_ref)
        .map_err(|failure| {
            authority_error(format!(
                "candidate retained normalized-content provenance refused: {}",
                failure.detail()
            ))
        })?;
    validate_candidate_field_source_bijection(candidate, &intake, &normalized_content).map_err(
        |failure| {
            authority_error(format!(
                "candidate populated-leaf provenance replay refused: {}",
                failure.detail()
            ))
        },
    )
}

fn create_result_temporary(
    partition: &Path,
) -> Result<(PathBuf, fs::File), LifecycleValidationErrorV1> {
    for _ in 0..16 {
        let mut nonce = [0_u8; 16];
        getrandom::fill(&mut nonce)
            .map_err(|_| io_error("lifecycle-validation temporary entropy failed"))?;
        let token = nonce
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect::<String>();
        let path = partition.join(format!(".lifecycle-validation-result-{token}.tmp"));
        match create_new_file(&path) {
            Ok(file) => return Ok((path, file)),
            Err(failure) if failure.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(_) => return Err(io_error("lifecycle-validation temporary create-new failed")),
        }
    }
    Err(io_error(
        "lifecycle-validation temporary collision budget exhausted",
    ))
}

#[cfg(any(
    target_os = "android",
    target_os = "linux",
    target_os = "macos",
    target_os = "ios",
    target_os = "tvos",
    target_os = "visionos",
    target_os = "watchos",
    target_os = "redox",
))]
fn rename_result_create_new(source: &Path, target: &Path) -> Result<(), std::io::Error> {
    use rustix::fs::{renameat_with, RenameFlags, CWD};

    renameat_with(CWD, source, CWD, target, RenameFlags::NOREPLACE).map_err(std::io::Error::from)
}

#[cfg(windows)]
fn rename_result_create_new(source: &Path, target: &Path) -> Result<(), std::io::Error> {
    let temporary = tempfile::TempPath::try_from_path(source.to_path_buf())?;
    match temporary.persist_noclobber(target) {
        Ok(()) => Ok(()),
        Err(failure) => {
            let tempfile::PathPersistError { error, path } = failure;
            let _ = path.keep();
            Err(error)
        }
    }
}

#[cfg(all(
    unix,
    not(any(
        target_os = "android",
        target_os = "linux",
        target_os = "macos",
        target_os = "ios",
        target_os = "tvos",
        target_os = "visionos",
        target_os = "watchos",
        target_os = "redox",
    ))
))]
fn rename_result_create_new(_source: &Path, _target: &Path) -> Result<(), std::io::Error> {
    Err(std::io::Error::new(
        std::io::ErrorKind::Unsupported,
        "atomic create-new result rename is unavailable on this platform",
    ))
}

#[cfg(all(not(unix), not(windows)))]
fn rename_result_create_new(_source: &Path, _target: &Path) -> Result<(), std::io::Error> {
    Err(std::io::Error::new(
        std::io::ErrorKind::Unsupported,
        "atomic create-new result rename is unavailable on this platform",
    ))
}

fn build_result(
    decisions: &ResolvedProfileDecisions,
    evaluation: &CharterCandidateEvaluationV12,
    current: Option<&CommittedCharterAuthorityV1>,
    lifecycle: &RetainedLifecycleAuthorityV1,
) -> Result<CharterLifecycleValidationResultV10, LifecycleValidationErrorV1> {
    let observed = current.map(|authority| authority.canonical_fingerprint.clone());
    if evaluation.candidate_subject.basis_artifact_fingerprint != observed {
        return Err(authority_error(
            "candidate subject basis is stale against committed Charter authority",
        ));
    }
    match (
        current,
        lifecycle.authority.as_ref(),
        lifecycle.canonical_bytes.as_ref(),
    ) {
        (None, None, None) => {}
        (Some(current), Some(authority), Some(lifecycle_bytes))
            if current.canonical_bytes == *lifecycle_bytes
                && current.canonical_fingerprint == authority.canonical_fingerprint => {}
        _ => {
            return Err(authority_error(
                "canonical promotion authority and lifecycle authority are not exact-equal",
            ))
        }
    }

    let mut active_observations = Vec::new();
    let mut reopened_coverage_ids = Vec::new();
    let (head_ref, head_fingerprint, state, state_fingerprint) = if let Some(authority) =
        lifecycle.authority.as_ref()
    {
        if authority.active_observation_refs.len()
            != authority.active_observation_fingerprints.len()
        {
            return Err(authority_error(
                "committed lifecycle head has incomplete active-observation bindings",
            ));
        }
        for (observation_ref, observation_fingerprint) in authority
            .active_observation_refs
            .iter()
            .zip(&authority.active_observation_fingerprints)
        {
            active_observations.push(LifecycleActiveObservationV10 {
                observation_ref: observation_ref.clone(),
                observation_fingerprint: observation_fingerprint.clone(),
            });
        }
        let reopened = authority
            .reopened_coverage_ids
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>();
        if reopened.len() != authority.reopened_coverage_ids.len() {
            return Err(authority_error(
                "committed lifecycle head contains duplicate reopened coverage",
            ));
        }
        for coverage_id in COVERAGE_ORDER {
            if reopened.contains(coverage_id) {
                let covered = evaluation.intake.coverage_results.iter().any(|entry| {
                    entry.coverage_id == coverage_id
                        && matches!(entry.evaluation.as_str(), "satisfied" | "waived")
                });
                if !covered {
                    return Err(authority_error(format!(
                            "reopened lifecycle coverage `{coverage_id}` is not satisfied by the intake",
                        )));
                }
                reopened_coverage_ids.push(coverage_id.to_owned());
            }
        }
        if reopened_coverage_ids.len() != reopened.len() {
            return Err(authority_error(
                "committed lifecycle head contains an unknown reopened coverage ID",
            ));
        }
        (
            Some(authority.lifecycle_transition_ref.clone()),
            Some(authority.lifecycle_transition_fingerprint.clone()),
            lifecycle_state_name(authority.state).to_owned(),
            Some(authority.state_fingerprint.clone()),
        )
    } else {
        (None, None, "absent".to_owned(), None)
    };

    let intake_fingerprint = evaluation.intake.record_fingerprint.clone();
    let content_fingerprint =
        DefinitionFingerprint::from_bytes(&evaluation.normalized_content).to_string();
    let mut result = CharterLifecycleValidationResultV10 {
        schema_id: "handbook.lifecycle-validation-result".to_owned(),
        schema_version: "1.0".to_owned(),
        validation_result_id: String::new(),
        candidate_subject_fingerprint: evaluation
            .candidate_subject
            .candidate_subject_fingerprint
            .clone(),
        intake_record_ref: evaluation.candidate_subject.intake_record_ref.clone(),
        intake_record_fingerprint: intake_fingerprint,
        normalized_content_ref: evaluation.candidate_subject.normalized_content_ref.clone(),
        normalized_content_fingerprint: content_fingerprint,
        target_instance_id: "project_authority".to_owned(),
        canonical_artifact_ref: CANONICAL_CHARTER_REF.to_owned(),
        basis_artifact_fingerprint: observed.clone(),
        observed_current_artifact_fingerprint: observed,
        profile_ref: decisions.profile_ref().as_str().to_owned(),
        resolved_profile_fingerprint: decisions.profile_definition_fingerprint().to_string(),
        resolved_definitions: definition_bindings(),
        lifecycle_policy_ref: LIFECYCLE_POLICY_REF.to_owned(),
        lifecycle_policy_fingerprint: LIFECYCLE_POLICY_FINGERPRINT.to_owned(),
        lifecycle_head_ref: head_ref,
        lifecycle_head_fingerprint: head_fingerprint,
        lifecycle_state: state,
        lifecycle_state_fingerprint: state_fingerprint,
        active_observations,
        reopened_coverage_ids,
        validation_status: "passed".to_owned(),
        validated_at_utc: String::new(),
        validation_result_fingerprint: String::new(),
    };
    let fingerprint = DefinitionFingerprint::from_json_value(&result_identity_preimage(&result)?)
        .map_err(|_| invalid_result("validation-result fingerprint preimage is not canonical"))?
        .to_string();
    let hex = fingerprint
        .strip_prefix("sha256:")
        .expect("engine fingerprints are normalized");
    result.validation_result_id = format!("lifecycle-validation-result_{hex}");
    result.validation_result_fingerprint = fingerprint;
    Ok(result)
}

fn finalize_candidate(
    evaluation: &CharterCandidateEvaluationV12,
    validation_result_binding: CharterValidationResultBindingV13,
) -> Result<CharterCandidateV12, LifecycleValidationErrorV1> {
    let subject = &evaluation.candidate_subject;
    let mut candidate = CharterCandidateV12 {
        schema_id: subject.schema_id.clone(),
        schema_version: subject.schema_version.clone(),
        candidate_id: String::new(),
        intake_record_ref: subject.intake_record_ref.clone(),
        target_kind_ref: subject.target_kind_ref.clone(),
        target_instance_id: subject.target_instance_id.clone(),
        target_schema_ref: subject.target_schema_ref.clone(),
        profile_ref: subject.profile_ref.clone(),
        resolved_profile_fingerprint: subject.resolved_profile_fingerprint.clone(),
        basis_artifact_fingerprint: subject.basis_artifact_fingerprint.clone(),
        normalized_content_ref: subject.normalized_content_ref.clone(),
        field_sources: subject.field_sources.clone(),
        unresolved_coverage_ids: subject.unresolved_coverage_ids.clone(),
        promotion_eligibility: subject.promotion_eligibility.clone(),
        required_approval_policy_ref: subject.required_approval_policy_ref.clone(),
        candidate_subject_fingerprint: subject.candidate_subject_fingerprint.clone(),
        validation_result_binding,
        candidate_fingerprint: String::new(),
    };
    let mut preimage = serde_json::to_value(&candidate)
        .map_err(|_| invalid_result("final candidate preimage serialization failed"))?;
    let object = preimage
        .as_object_mut()
        .ok_or_else(|| invalid_result("final candidate preimage is not an object"))?;
    object.remove("candidate_id");
    object.remove("candidate_fingerprint");
    let fingerprint = DefinitionFingerprint::from_json_value(&preimage)
        .map_err(|_| invalid_result("final candidate preimage is not canonical"))?
        .to_string();
    let hex = fingerprint
        .strip_prefix("sha256:")
        .expect("engine fingerprints are normalized");
    candidate.candidate_id = format!("candidate_{hex}");
    candidate.candidate_fingerprint = fingerprint;
    Ok(candidate)
}

fn validate_definition_authority(
    decisions: &ResolvedProfileDecisions,
) -> Result<(), LifecycleValidationErrorV1> {
    if decisions.profile_ref().as_str() != SELECTED_PROFILE_REF
        || decisions.profile_definition_fingerprint().as_str() != SELECTED_PROFILE_FINGERPRINT
    {
        return Err(authority_error(
            "selected profile ref/fingerprint is not the reviewed Charter 1.1 authority",
        ));
    }
    let registry = load_shipped_charter_definition_registry().map_err(|_| {
        authority_error("shipped Charter definition registry could not be resolved")
    })?;
    registry
        .validate_selected_decisions(decisions)
        .map_err(|_| authority_error("selected Charter definition closure is not exact"))?;
    for (_, reference, fingerprint) in DEFINITION_BINDINGS {
        if let Ok(exact_ref) = ExactDefinitionRef::parse(reference) {
            if let Some(record) = registry.record(&exact_ref) {
                if record.definition_fingerprint().as_str() != fingerprint {
                    return Err(authority_error(format!(
                        "definition `{reference}` fingerprint is stale or forged",
                    )));
                }
            }
        }
    }
    let instance_id = SymbolicId::parse("project_authority")
        .map_err(|_| authority_error("project_authority instance ID is invalid"))?;
    let instance = decisions
        .registry()
        .instance(&instance_id)
        .ok_or_else(|| authority_error("selected Charter instance is absent"))?;
    let kind = decisions
        .registry()
        .kind(instance.kind_ref())
        .ok_or_else(|| authority_error("selected Charter kind is absent"))?;
    if kind.definition_fingerprint().as_str() != DEFINITION_BINDINGS[10].2
        || kind.schema_entry_fingerprint().as_str() != DEFINITION_BINDINGS[11].2
    {
        return Err(authority_error(
            "selected Charter kind/schema producer fingerprints are stale",
        ));
    }
    let capability = instance
        .capabilities()
        .iter()
        .find(|entry| entry.contract_ref().as_str() == DEFINITION_BINDINGS[1].1)
        .ok_or_else(|| authority_error("constitutional capability contract is absent"))?;
    if capability.contract_fingerprint().as_str() != DEFINITION_BINDINGS[1].2 {
        return Err(authority_error(
            "constitutional capability contract fingerprint is stale",
        ));
    }
    let observed_validators = capability
        .semantic_validators()
        .iter()
        .map(|entry| {
            (
                entry.exact_ref().as_str(),
                entry.profile_fingerprint().as_str(),
            )
        })
        .collect::<Vec<_>>();
    if observed_validators != vec![(DEFINITION_BINDINGS[8].1, DEFINITION_BINDINGS[8].2)] {
        return Err(authority_error(
            "constitutional capability validator closure is stale or reordered",
        ));
    }
    let capability_ref = ExactDefinitionRef::parse(DEFINITION_BINDINGS[1].1)
        .map_err(|_| authority_error("constitutional capability ref is invalid"))?;
    let validator_10_ref = ExactDefinitionRef::parse(DEFINITION_BINDINGS[8].1)
        .map_err(|_| authority_error("constitutional validator 1.0 ref is invalid"))?;
    let validator_11_ref = ExactDefinitionRef::parse(DEFINITION_BINDINGS[9].1)
        .map_err(|_| authority_error("constitutional validator 1.1 ref is invalid"))?;
    let capability_source = profile_builtins::definition(&capability_ref)
        .ok_or_else(|| authority_error("constitutional capability source is absent"))?;
    let validator_10_source = profile_builtins::definition(&validator_10_ref)
        .ok_or_else(|| authority_error("constitutional validator 1.0 source is absent"))?;
    let validator_11_source = profile_builtins::definition(&validator_11_ref)
        .ok_or_else(|| authority_error("constitutional validator 1.1 source is absent"))?;
    let semantic_registry = SemanticCapabilityRegistry::load_admitted(
        &[(capability_ref.clone(), capability_source.bytes)]
            .iter()
            .map(|(reference, bytes)| (reference, *bytes))
            .collect::<Vec<_>>(),
        &[
            (validator_10_ref.clone(), validator_10_source.bytes),
            (validator_11_ref.clone(), validator_11_source.bytes),
        ]
        .iter()
        .map(|(reference, bytes)| (reference, *bytes))
        .collect::<Vec<_>>(),
    )
    .map_err(|_| authority_error("constitutional semantic registry is not exact"))?;
    for (reference, fingerprint) in [
        (&validator_10_ref, DEFINITION_BINDINGS[8].2),
        (&validator_11_ref, DEFINITION_BINDINGS[9].2),
    ] {
        if semantic_registry
            .validator(reference)
            .is_none_or(|validator| validator.profile_fingerprint().as_str() != fingerprint)
        {
            return Err(authority_error(format!(
                "semantic validator `{}` fingerprint is stale",
                reference.as_str()
            )));
        }
    }
    Ok(())
}

pub(crate) fn definition_bindings() -> Vec<LifecycleDefinitionBindingV10> {
    DEFINITION_BINDINGS
        .iter()
        .map(
            |(role, reference, fingerprint)| LifecycleDefinitionBindingV10 {
                definition_role: (*role).to_owned(),
                definition_ref: (*reference).to_owned(),
                definition_fingerprint: (*fingerprint).to_owned(),
            },
        )
        .collect()
}

fn encode_result(
    record: &CharterLifecycleValidationResultV10,
) -> Result<Vec<u8>, LifecycleValidationErrorV1> {
    let mut bytes = serde_json_canonicalizer::to_vec(record)
        .map_err(|_| invalid_result("lifecycle-validation result canonicalization failed"))?;
    bytes.push(b'\n');
    if bytes.len() > MAX_LIFECYCLE_VALIDATION_RESULT_BYTES {
        return Err(invalid_result(
            "lifecycle-validation result exceeds the 262,144-byte persistence bound",
        ));
    }
    Ok(bytes)
}

pub(crate) fn validate_result_bytes(
    bytes: &[u8],
    expected_ref: Option<&str>,
) -> Result<CharterLifecycleValidationResultV10, LifecycleValidationErrorV1> {
    if bytes.is_empty()
        || bytes.len() > MAX_LIFECYCLE_VALIDATION_RESULT_BYTES
        || !bytes.ends_with(b"\n")
        || bytes[..bytes.len() - 1].ends_with(b"\n")
    {
        return Err(invalid_result(
            "lifecycle-validation result must be bounded exact JCS plus one LF",
        ));
    }
    let jcs = &bytes[..bytes.len() - 1];
    let value = parse_schema_json(jcs)
        .map_err(|_| invalid_result("lifecycle-validation result is not duplicate-free JSON"))?;
    let canonical = serde_json_canonicalizer::to_vec(&value)
        .map_err(|_| invalid_result("lifecycle-validation result cannot be canonicalized"))?;
    if canonical != jcs {
        return Err(invalid_result(
            "lifecycle-validation result bytes are not exact RFC 8785 JCS",
        ));
    }
    validate_result_schema_v10(&value)?;
    let record: CharterLifecycleValidationResultV10 = serde_json::from_value(value)
        .map_err(|_| invalid_result("lifecycle-validation result violates its closed shape"))?;
    validate_result_record(&record, expected_ref)?;
    Ok(record)
}

fn validate_result_schema_v10(value: &Value) -> Result<(), LifecycleValidationErrorV1> {
    let validator = LIFECYCLE_VALIDATION_RESULT_VALIDATOR_V10.get_or_init(|| {
        let schema = parse_schema_json(LIFECYCLE_VALIDATION_RESULT_SCHEMA_V10)
            .map_err(|_| "embedded lifecycle-validation result schema is invalid".to_owned())?;
        jsonschema::draft202012::options()
            .with_pattern_options(jsonschema::PatternOptions::regex())
            .should_validate_formats(true)
            .build(&schema)
            .map_err(|_| "embedded lifecycle-validation result validator is invalid".to_owned())
    });
    let validator = validator
        .as_ref()
        .map_err(|detail| invalid_result(detail.clone()))?;
    if validator.is_valid(value) {
        Ok(())
    } else {
        Err(invalid_result(
            "lifecycle-validation result violates its frozen 1.0 schema",
        ))
    }
}

fn validate_result_record(
    record: &CharterLifecycleValidationResultV10,
    expected_ref: Option<&str>,
) -> Result<(), LifecycleValidationErrorV1> {
    if record.schema_id != "handbook.lifecycle-validation-result"
        || record.schema_version != "1.0"
        || record.target_instance_id != "project_authority"
        || record.canonical_artifact_ref != CANONICAL_CHARTER_REF
        || record.profile_ref != SELECTED_PROFILE_REF
        || record.resolved_profile_fingerprint != SELECTED_PROFILE_FINGERPRINT
        || record.resolved_definitions != definition_bindings()
        || record.lifecycle_policy_ref != LIFECYCLE_POLICY_REF
        || record.lifecycle_policy_fingerprint != LIFECYCLE_POLICY_FINGERPRINT
        || record.validation_status != "passed"
        || !is_utc_timestamp(&record.validated_at_utc)
    {
        return Err(invalid_result(
            "lifecycle-validation result constants or definition closure are invalid",
        ));
    }
    for fingerprint in [
        Some(record.candidate_subject_fingerprint.as_str()),
        Some(record.intake_record_fingerprint.as_str()),
        Some(record.normalized_content_fingerprint.as_str()),
        record.basis_artifact_fingerprint.as_deref(),
        record.observed_current_artifact_fingerprint.as_deref(),
        record.lifecycle_head_fingerprint.as_deref(),
        record.lifecycle_state_fingerprint.as_deref(),
        Some(record.validation_result_fingerprint.as_str()),
    ]
    .into_iter()
    .flatten()
    {
        DefinitionFingerprint::parse(fingerprint).map_err(|_| {
            invalid_result("validation-result fingerprint is not lowercase SHA-256")
        })?;
    }
    let computed = DefinitionFingerprint::from_json_value(&result_identity_preimage(record)?)
        .map_err(|_| invalid_result("validation-result identity cannot be recomputed"))?
        .to_string();
    if computed != record.validation_result_fingerprint {
        return Err(invalid_result(
            "validation-result fingerprint does not match its exact preimage",
        ));
    }
    let hex = computed
        .strip_prefix("sha256:")
        .expect("validated fingerprint");
    if record.validation_result_id != format!("lifecycle-validation-result_{hex}")
        || expected_ref.is_some_and(|reference| reference != result_ref(&computed))
    {
        return Err(invalid_result(
            "validation-result ID/ref does not match its content-addressed fingerprint",
        ));
    }
    let active_unique = record
        .active_observations
        .iter()
        .map(|entry| (&entry.observation_ref, &entry.observation_fingerprint))
        .collect::<BTreeSet<_>>();
    let reopened_unique = record.reopened_coverage_ids.iter().collect::<BTreeSet<_>>();
    if active_unique.len() != record.active_observations.len()
        || reopened_unique.len() != record.reopened_coverage_ids.len()
    {
        return Err(invalid_result(
            "validation-result observation/coverage arrays contain duplicates",
        ));
    }
    if record.lifecycle_state == "absent" {
        if record.basis_artifact_fingerprint.is_some()
            || record.observed_current_artifact_fingerprint.is_some()
            || record.lifecycle_head_ref.is_some()
            || record.lifecycle_head_fingerprint.is_some()
            || record.lifecycle_state_fingerprint.is_some()
            || !record.active_observations.is_empty()
            || !record.reopened_coverage_ids.is_empty()
        {
            return Err(invalid_result(
                "absent lifecycle authority must have the exact null/empty closure",
            ));
        }
    } else if !matches!(
        record.lifecycle_state.as_str(),
        "current" | "review_required" | "reassessment_required"
    ) || record.basis_artifact_fingerprint.is_none()
        || record.observed_current_artifact_fingerprint.is_none()
        || record.lifecycle_head_ref.is_none()
        || record.lifecycle_head_fingerprint.is_none()
        || record.lifecycle_state_fingerprint.is_none()
    {
        return Err(invalid_result(
            "non-absent lifecycle authority is incomplete or has an unknown state",
        ));
    }
    Ok(())
}

fn result_identity_preimage(
    record: &CharterLifecycleValidationResultV10,
) -> Result<Value, LifecycleValidationErrorV1> {
    let mut value = serde_json::to_value(record)
        .map_err(|_| invalid_result("validation-result preimage serialization failed"))?;
    let object = value
        .as_object_mut()
        .ok_or_else(|| invalid_result("validation-result preimage is not an object"))?;
    object.remove("validation_result_id");
    object.remove("validation_result_fingerprint");
    object.remove("validated_at_utc");
    Ok(value)
}

fn read_result_file(
    repo_root: &Path,
    relative_ref: &str,
) -> Result<Vec<u8>, LifecycleValidationErrorV1> {
    read_state_file(repo_root, relative_ref, "result")
}

fn read_state_file(
    repo_root: &Path,
    relative_ref: &str,
    label: &str,
) -> Result<Vec<u8>, LifecycleValidationErrorV1> {
    read_state_file_bounded(
        repo_root,
        relative_ref,
        label,
        MAX_LIFECYCLE_VALIDATION_RESULT_BYTES,
    )
}

fn read_state_file_bounded(
    repo_root: &Path,
    relative_ref: &str,
    label: &str,
    max_bytes: usize,
) -> Result<Vec<u8>, LifecycleValidationErrorV1> {
    let path = repo_root.join(".handbook/state").join(relative_ref);
    reject_reparse_or_symlink(&path, false).map_err(|failure| {
        filesystem_error(format!(
            "lifecycle-validation {label} path is unsafe: {}",
            failure.detail()
        ))
    })?;
    let workspace = CanonicalWorkspace::new(repo_root);
    let repo_relative = format!(".handbook/state/{relative_ref}");
    let normalized = workspace
        .normalize_repo_relative(&repo_relative)
        .map_err(|_| filesystem_error("validation-result ref is not repository-relative"))?;
    let file = workspace.trusted_read_strict(&normalized).map_err(|_| {
        filesystem_error(format!(
            "validation-result {label} cannot be read without following"
        ))
    })?;
    let (bytes, exceeded) = file
        .read_bytes_bounded(max_bytes)
        .map_err(|_| io_error("lifecycle-validation result read failed"))?;
    if exceeded {
        return Err(invalid_result(
            "lifecycle-validation result exceeds the exact byte bound",
        ));
    }
    Ok(bytes)
}

fn result_binding(
    relative_ref: &str,
    record: &CharterLifecycleValidationResultV10,
    result_bytes: &[u8],
) -> Result<CharterValidationResultBindingV13, LifecycleValidationErrorV1> {
    let binding = CharterValidationResultBindingV13 {
        validation_result_ref: relative_ref.to_owned(),
        validation_result_fingerprint: record.validation_result_fingerprint.clone(),
        result_document_sha256: DefinitionFingerprint::from_bytes(result_bytes).to_string(),
        result_byte_length: result_bytes.len(),
    };
    validate_result_binding_shape(&binding)?;
    Ok(binding)
}

fn parse_candidate_result_binding(
    candidate: &serde_json::Map<String, Value>,
) -> Result<CharterValidationResultBindingV13, LifecycleValidationErrorV1> {
    let value = candidate
        .get("validation_result_binding")
        .cloned()
        .ok_or_else(|| authority_error("candidate exact-result binding is absent"))?;
    let binding: CharterValidationResultBindingV13 = serde_json::from_value(value)
        .map_err(|_| authority_error("candidate exact-result binding is not exact-closed"))?;
    validate_result_binding_shape(&binding)?;
    Ok(binding)
}

fn validate_result_binding_shape(
    binding: &CharterValidationResultBindingV13,
) -> Result<(), LifecycleValidationErrorV1> {
    let semantic = DefinitionFingerprint::parse(&binding.validation_result_fingerprint)
        .map_err(|_| authority_error("candidate semantic result fingerprint is invalid"))?
        .to_string();
    DefinitionFingerprint::parse(&binding.result_document_sha256)
        .map_err(|_| authority_error("candidate exact result-document digest is invalid"))?;
    if binding.result_byte_length == 0
        || binding.result_byte_length > MAX_LIFECYCLE_VALIDATION_RESULT_BYTES
        || fingerprint_from_bound_ref(
            &binding.validation_result_ref,
            "lifecycle-validation-results/lifecycle-validation-result_",
            ".json",
        )? != semantic
    {
        return Err(authority_error(
            "candidate exact-result binding ref, fingerprint, or byte bound is invalid",
        ));
    }
    Ok(())
}

fn validate_bound_result_bytes(
    bytes: &[u8],
    binding: &CharterValidationResultBindingV13,
    expected_candidate_subject_fingerprint: &str,
) -> Result<CharterLifecycleValidationResultV10, LifecycleValidationErrorV1> {
    validate_result_binding_shape(binding)?;
    if bytes.len() != binding.result_byte_length
        || DefinitionFingerprint::from_bytes(bytes).to_string() != binding.result_document_sha256
    {
        return Err(LifecycleValidationErrorV1::new(
            LifecycleValidationErrorKindV1::ExistingBytesMismatch,
            "exact lifecycle-validation result digest or LF-inclusive byte length disagrees with candidate authority",
        ));
    }
    let record = validate_result_bytes(bytes, Some(&binding.validation_result_ref))?;
    if record.validation_result_fingerprint != binding.validation_result_fingerprint
        || record.candidate_subject_fingerprint != expected_candidate_subject_fingerprint
    {
        return Err(authority_error(
            "candidate exact-result binding crosses semantic result or candidate-subject authority",
        ));
    }
    Ok(record)
}
#[derive(Clone, Debug, Eq, PartialEq)]
struct InventoryFileIdentityV1([u64; 6]);

#[derive(Clone, Debug, Eq, PartialEq)]
struct InventoryEntryTupleV1 {
    name: String,
    identity: InventoryFileIdentityV1,
    byte_length: u64,
    sha256: [u8; 32],
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum InventoryRootTupleV1 {
    Missing,
    Present {
        identity: InventoryFileIdentityV1,
        entries: Vec<InventoryEntryTupleV1>,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct AuthorInventoryTupleV1 {
    candidates: InventoryRootTupleV1,
    results: InventoryRootTupleV1,
}

struct CandidateInventoryEntryV1 {
    value: Value,
}

struct ResultInventoryEntryV1 {
    relative_ref: String,
    bytes: Vec<u8>,
}

struct AuthorInventoryV1 {
    tuple: AuthorInventoryTupleV1,
    candidates: Vec<CandidateInventoryEntryV1>,
    results: Vec<ResultInventoryEntryV1>,
}

enum AuthorReplayV1 {
    FirstAuthoring,
    Exact {
        candidate: Box<CharterCandidateV12>,
        result_bytes: Vec<u8>,
    },
}

#[derive(Clone, Copy)]
enum InventoryStoreKindV1 {
    Candidate,
    Result,
}

fn inventory_author_authority(
    repo_root: &Path,
) -> Result<AuthorInventoryV1, LifecycleValidationErrorV1> {
    let (candidate_tuple, candidates, _) = scan_inventory_store(
        repo_root,
        CANDIDATE_INVENTORY_ROOT,
        InventoryStoreKindV1::Candidate,
    )?;
    let (result_tuple, _, results) = scan_inventory_store(
        repo_root,
        RESULT_INVENTORY_ROOT,
        InventoryStoreKindV1::Result,
    )?;
    Ok(AuthorInventoryV1 {
        tuple: AuthorInventoryTupleV1 {
            candidates: candidate_tuple,
            results: result_tuple,
        },
        candidates,
        results,
    })
}

fn inventory_author_authority_stable(
    repo_root: &Path,
) -> Result<AuthorInventoryV1, LifecycleValidationErrorV1> {
    inventory_author_authority_stable_with(repo_root, || {})
}

fn inventory_author_authority_stable_with(
    repo_root: &Path,
    between_scans: impl FnOnce(),
) -> Result<AuthorInventoryV1, LifecycleValidationErrorV1> {
    let first_inventory = inventory_author_authority(repo_root)?;
    between_scans();
    let second_inventory = inventory_author_authority(repo_root)?;
    if first_inventory.tuple != second_inventory.tuple {
        return Err(authority_error(
            "candidate/result stores changed during the stable author inventory",
        ));
    }
    Ok(second_inventory)
}

fn scan_inventory_store(
    repo_root: &Path,
    root_relative: &str,
    kind: InventoryStoreKindV1,
) -> Result<
    (
        InventoryRootTupleV1,
        Vec<CandidateInventoryEntryV1>,
        Vec<ResultInventoryEntryV1>,
    ),
    LifecycleValidationErrorV1,
> {
    let root = repo_root.join(root_relative);
    let root_metadata = match fs::symlink_metadata(&root) {
        Ok(metadata) => metadata,
        Err(failure) if failure.kind() == std::io::ErrorKind::NotFound => {
            return Ok((InventoryRootTupleV1::Missing, Vec::new(), Vec::new()))
        }
        Err(_) => return Err(filesystem_error("author inventory root metadata failed")),
    };
    reject_reparse_or_symlink(&root, true).map_err(|failure| {
        filesystem_error(format!(
            "author inventory root is unsafe: {}",
            failure.detail()
        ))
    })?;
    if !root_metadata.is_dir() {
        return Err(filesystem_error(
            "author inventory root is not a safe directory",
        ));
    }
    let root_identity = inventory_path_identity(&root, true)?;
    let mut entries = Vec::new();
    let mut candidates = Vec::new();
    let mut results = Vec::new();
    let mut aggregate = 0_u64;
    for entry in
        fs::read_dir(&root).map_err(|_| filesystem_error("author inventory enumeration failed"))?
    {
        let entry = entry.map_err(|_| filesystem_error("author inventory entry failed"))?;
        if !inventory_entry_count_within_bound(entries.len().saturating_add(1)) {
            return Err(invalid_result(
                "author inventory exceeds 4096 immediate entries",
            ));
        }
        let raw_name = entry.file_name();
        let name = raw_name
            .to_str()
            .ok_or_else(|| filesystem_error("author inventory filename is not UTF-8"))?;
        if !inventory_filename_length_within_bound(name.len()) {
            return Err(invalid_result(
                "author inventory filename exceeds 128 UTF-8 bytes",
            ));
        }
        let owned = match kind {
            InventoryStoreKindV1::Candidate => owned_candidate_filename(name),
            InventoryStoreKindV1::Result => owned_result_filename(name),
        };
        if !owned {
            return Err(filesystem_error(
                "author inventory contains an unowned immediate entry name",
            ));
        }
        let path = entry.path();
        reject_reparse_or_symlink(&path, false).map_err(|failure| {
            filesystem_error(format!(
                "author inventory entry is unsafe: {}",
                failure.detail()
            ))
        })?;
        let before = fs::symlink_metadata(&path)
            .map_err(|_| filesystem_error("author inventory entry metadata failed"))?;
        if !before.is_file() {
            return Err(filesystem_error(
                "author inventory entry is not a regular file",
            ));
        }
        if !inventory_file_length_within_bound(before.len()) {
            return Err(invalid_result(
                "author inventory entry exceeds 262144 bytes",
            ));
        }
        let before_identity = inventory_path_identity(&path, false)?;
        aggregate = inventory_aggregate_after(aggregate, before.len())?;
        let repo_relative = format!("{root_relative}/{name}");
        let workspace = CanonicalWorkspace::new(repo_root);
        let normalized = workspace
            .normalize_repo_relative(&repo_relative)
            .map_err(|_| filesystem_error("author inventory path is not normalized"))?;
        let file = workspace
            .trusted_read_strict(&normalized)
            .map_err(|_| filesystem_error("author inventory entry no-follow open failed"))?;
        let (bytes, exceeded) = file
            .read_bytes_bounded(MAX_INVENTORY_FILE_BYTES)
            .map_err(|_| io_error("author inventory entry read failed"))?;
        if exceeded || bytes.len() as u64 != before.len() {
            return Err(invalid_result(
                "author inventory entry changed or exceeded its bound during read",
            ));
        }
        let after = fs::symlink_metadata(&path)
            .map_err(|_| filesystem_error("author inventory post-read metadata failed"))?;
        if before_identity != inventory_path_identity(&path, false)? || before.len() != after.len()
        {
            return Err(authority_error(
                "author inventory entry identity changed during read",
            ));
        }
        match kind {
            InventoryStoreKindV1::Candidate => {
                let value = validate_inventory_candidate(name, &bytes)?;
                if value.get("schema_version").and_then(Value::as_str) == Some("1.3") {
                    validate_candidate_field_source_authority(repo_root, &value)?;
                }
                candidates.push(CandidateInventoryEntryV1 { value });
            }
            InventoryStoreKindV1::Result => {
                let relative_ref = format!("{RESULT_PARTITION}/{name}");
                validate_result_bytes(&bytes, Some(&relative_ref))?;
                results.push(ResultInventoryEntryV1 {
                    relative_ref,
                    bytes: bytes.clone(),
                });
            }
        }
        entries.push(InventoryEntryTupleV1 {
            name: name.to_owned(),
            identity: before_identity,
            byte_length: before.len(),
            sha256: Sha256::digest(&bytes).into(),
        });
    }
    entries.sort_by(|left, right| left.name.as_bytes().cmp(right.name.as_bytes()));
    let root_after = fs::symlink_metadata(&root)
        .map_err(|_| filesystem_error("author inventory root changed during scan"))?;
    if inventory_path_identity(&root, true)? != root_identity || !root_after.is_dir() {
        return Err(authority_error(
            "author inventory root identity changed during scan",
        ));
    }
    Ok((
        InventoryRootTupleV1::Present {
            identity: root_identity,
            entries,
        },
        candidates,
        results,
    ))
}

fn inventory_entry_count_within_bound(count: usize) -> bool {
    count <= MAX_INVENTORY_ENTRIES
}

fn inventory_file_length_within_bound(byte_length: u64) -> bool {
    byte_length <= MAX_INVENTORY_FILE_BYTES as u64
}

fn inventory_filename_length_within_bound(byte_length: usize) -> bool {
    byte_length <= MAX_INVENTORY_FILENAME_BYTES
}

fn inventory_aggregate_after(
    aggregate: u64,
    byte_length: u64,
) -> Result<u64, LifecycleValidationErrorV1> {
    aggregate
        .checked_add(byte_length)
        .filter(|total| *total <= MAX_INVENTORY_AGGREGATE_BYTES)
        .ok_or_else(|| invalid_result("author inventory aggregate byte bound exceeded"))
}

fn classify_author_replay(
    inventory: &AuthorInventoryV1,
    expected_subject_fingerprint: &str,
    expected_result_ref: &str,
    expected_result_fingerprint: &str,
) -> Result<AuthorReplayV1, LifecycleValidationErrorV1> {
    let mut matching = Vec::new();
    for entry in &inventory.candidates {
        if entry.value.get("schema_version").and_then(Value::as_str) != Some("1.3") {
            continue;
        }
        let object = entry
            .value
            .as_object()
            .ok_or_else(|| invalid_result("candidate inventory record is not an object"))?;
        let recomputed_subject = candidate_subject_fingerprint(&entry.value)?;
        let binding = parse_candidate_result_binding(object)?;
        let subject_matches = recomputed_subject == expected_subject_fingerprint;
        let semantic_matches = binding.validation_result_ref == expected_result_ref
            && binding.validation_result_fingerprint == expected_result_fingerprint;
        if subject_matches != semantic_matches {
            return Err(authority_error(
                "candidate inventory contains crossed subject/result authority",
            ));
        }
        if subject_matches && semantic_matches {
            matching.push(Box::new(
                serde_json::from_value::<CharterCandidateV12>(entry.value.clone())
                    .map_err(|_| invalid_result("matching candidate is not exact-closed 1.3"))?,
            ));
        }
    }
    if matching.len() > 1 {
        return Err(authority_error(
            "multiple candidate 1.3 records match the author replay selector",
        ));
    }
    let expected_result = inventory
        .results
        .iter()
        .find(|entry| entry.relative_ref == expected_result_ref);
    match (matching.pop(), expected_result) {
        (None, None) => Ok(AuthorReplayV1::FirstAuthoring),
        (None, Some(_)) => Err(authority_error(
            "orphan lifecycle-validation result is preserved and refused",
        )),
        (Some(_), None) => Err(authority_error(
            "candidate replay is missing its exact lifecycle-validation result",
        )),
        (Some(candidate), Some(result)) => {
            validate_bound_result_bytes(
                &result.bytes,
                &candidate.validation_result_binding,
                expected_subject_fingerprint,
            )?;
            Ok(AuthorReplayV1::Exact {
                candidate,
                result_bytes: result.bytes.clone(),
            })
        }
    }
}

fn validate_inventory_candidate(
    filename: &str,
    bytes: &[u8],
) -> Result<Value, LifecycleValidationErrorV1> {
    let value = parse_schema_json(bytes)
        .map_err(|_| invalid_result("candidate inventory entry is not duplicate-free JSON"))?;
    if serde_json_canonicalizer::to_vec(&value)
        .map_err(|_| invalid_result("candidate inventory entry cannot be canonicalized"))?
        != bytes
    {
        return Err(invalid_result(
            "candidate inventory entry is not exact RFC 8785 JCS",
        ));
    }
    let version = value
        .get("schema_version")
        .and_then(Value::as_str)
        .ok_or_else(|| invalid_result("candidate inventory schema version is absent"))?;
    let record_id = if version == "1.3" {
        validate_record(LineageRecordClassV1::Candidate, bytes)
            .map_err(|failure| {
                invalid_result(format!(
                    "candidate 1.3 inventory validation refused: {}",
                    failure.detail()
                ))
            })?
            .record_id
    } else if matches!(version, "1.0" | "1.1" | "1.2") {
        validate_historical_candidate(&value, version)?
    } else {
        return Err(invalid_result(
            "candidate inventory contains an unsupported schema version",
        ));
    };
    if filename != format!("{record_id}.json") {
        return Err(authority_error(
            "candidate inventory filename and content-addressed ID disagree",
        ));
    }
    Ok(value)
}

fn validate_historical_candidate(
    value: &Value,
    version: &str,
) -> Result<String, LifecycleValidationErrorV1> {
    const V10_KEYS: [&str; 16] = [
        "schema_id",
        "schema_version",
        "candidate_id",
        "intake_record_ref",
        "target_kind_ref",
        "target_instance_id",
        "target_schema_ref",
        "profile_ref",
        "resolved_profile_fingerprint",
        "normalized_content_ref",
        "field_sources",
        "validation_result_refs",
        "unresolved_coverage_ids",
        "promotion_eligibility",
        "required_approval_policy_ref",
        "candidate_fingerprint",
    ];
    const V11_KEYS: [&str; 17] = [
        "schema_id",
        "schema_version",
        "candidate_id",
        "intake_record_ref",
        "target_kind_ref",
        "target_instance_id",
        "target_schema_ref",
        "profile_ref",
        "resolved_profile_fingerprint",
        "basis_artifact_fingerprint",
        "normalized_content_ref",
        "field_sources",
        "validation_result_refs",
        "unresolved_coverage_ids",
        "promotion_eligibility",
        "required_approval_policy_ref",
        "candidate_fingerprint",
    ];
    const V12_KEYS: [&str; 18] = [
        "schema_id",
        "schema_version",
        "candidate_id",
        "intake_record_ref",
        "target_kind_ref",
        "target_instance_id",
        "target_schema_ref",
        "profile_ref",
        "resolved_profile_fingerprint",
        "basis_artifact_fingerprint",
        "normalized_content_ref",
        "field_sources",
        "unresolved_coverage_ids",
        "promotion_eligibility",
        "required_approval_policy_ref",
        "candidate_subject_fingerprint",
        "validation_result_refs",
        "candidate_fingerprint",
    ];
    let object = value
        .as_object()
        .ok_or_else(|| invalid_result("historical candidate is not an object"))?;
    let keys: &[&str] = match version {
        "1.0" => &V10_KEYS,
        "1.1" => &V11_KEYS,
        "1.2" => &V12_KEYS,
        _ => {
            return Err(invalid_result(
                "historical candidate has an unsupported schema version",
            ))
        }
    };
    if object.len() != keys.len()
        || keys.iter().any(|key| !object.contains_key(*key))
        || value.get("schema_id").and_then(Value::as_str) != Some("handbook.artifact-candidate")
        || value.get("schema_version").and_then(Value::as_str) != Some(version)
    {
        return Err(invalid_result(
            "historical candidate does not have its exact closed field set",
        ));
    }

    for field in ["target_kind_ref", "target_schema_ref", "profile_ref"] {
        ExactDefinitionRef::parse(candidate_string(value, field)?).map_err(|_| {
            invalid_result(format!(
                "historical candidate {field} is not an exact definition ref"
            ))
        })?;
    }
    ExactDefinitionRef::parse(candidate_string(value, "required_approval_policy_ref")?)
        .map_err(|_| invalid_result("historical candidate approval-policy ref is invalid"))?;
    SymbolicId::parse(candidate_string(value, "target_instance_id")?)
        .map_err(|_| invalid_result("historical candidate target instance ID is invalid"))?;
    for field in ["resolved_profile_fingerprint", "candidate_fingerprint"] {
        DefinitionFingerprint::parse(candidate_string(value, field)?).map_err(|_| {
            invalid_result(format!(
                "historical candidate {field} is not lowercase SHA-256"
            ))
        })?;
    }
    if candidate_string(value, "promotion_eligibility")? != "requires_approval" {
        return Err(invalid_result(
            "historical candidate promotion eligibility is not exact",
        ));
    }
    validate_historical_intake_ref(candidate_string(value, "intake_record_ref")?)?;
    validate_historical_content_ref(candidate_string(value, "normalized_content_ref")?, version)?;

    if version != "1.0" {
        match object.get("basis_artifact_fingerprint") {
            Some(Value::Null) => {}
            Some(Value::String(fingerprint)) => {
                DefinitionFingerprint::parse(fingerprint).map_err(|_| {
                    invalid_result(
                        "historical candidate basis fingerprint is not lowercase SHA-256",
                    )
                })?;
            }
            _ => {
                return Err(invalid_result(
                    "historical candidate basis fingerprint must be null or lowercase SHA-256",
                ))
            }
        }
    }

    validate_historical_field_sources(value)?;
    validate_historical_coverage_ids(value)?;
    validate_historical_result_refs(value, version)?;

    let fingerprint = candidate_string(value, "candidate_fingerprint")?;
    let mut preimage = value.clone();
    let preimage_object = preimage.as_object_mut().expect("validated object");
    preimage_object.remove("candidate_id");
    preimage_object.remove("candidate_fingerprint");
    let computed = DefinitionFingerprint::from_json_value(&preimage)
        .map_err(|_| invalid_result("historical candidate preimage is not canonical"))?
        .to_string();
    if computed != fingerprint {
        return Err(authority_error(
            "historical candidate final identity does not recompute",
        ));
    }
    let record_id = format!(
        "candidate_{}",
        fingerprint
            .strip_prefix("sha256:")
            .expect("validated fingerprint")
    );
    if candidate_string(value, "candidate_id")? != record_id {
        return Err(authority_error(
            "historical candidate ID does not equal its final fingerprint",
        ));
    }
    if version == "1.2" {
        let subject = candidate_subject_fingerprint(value)?;
        if candidate_string(value, "candidate_subject_fingerprint")? != subject {
            return Err(authority_error(
                "historical candidate subject identity does not recompute",
            ));
        }
    }
    Ok(record_id)
}

fn validate_historical_intake_ref(reference: &str) -> Result<(), LifecycleValidationErrorV1> {
    if is_content_addressed_ref(reference, "intake-records/intake_", ".json")
        || SymbolicId::parse(reference).is_ok()
    {
        Ok(())
    } else {
        Err(invalid_result(
            "historical candidate intake ref is not safe exact authority",
        ))
    }
}

fn validate_historical_content_ref(
    reference: &str,
    version: &str,
) -> Result<(), LifecycleValidationErrorV1> {
    let content_addressed =
        is_content_addressed_ref(reference, "candidate-content/charter_", ".yaml");
    let legacy = matches!(version, "1.0" | "1.1")
        && reference.starts_with("candidates/")
        && reference.ends_with(".yaml")
        && is_safe_relative_reference(reference);
    if content_addressed || legacy {
        Ok(())
    } else {
        Err(invalid_result(
            "historical candidate normalized-content ref is not safe for its version",
        ))
    }
}

fn validate_historical_field_sources(value: &Value) -> Result<(), LifecycleValidationErrorV1> {
    let sources = value
        .get("field_sources")
        .and_then(Value::as_array)
        .filter(|sources| !sources.is_empty())
        .ok_or_else(|| invalid_result("historical candidate field_sources must be nonempty"))?;
    let mut target_paths = BTreeSet::new();
    for source in sources {
        let object = source
            .as_object()
            .ok_or_else(|| invalid_result("historical candidate field source must be an object"))?;
        if object.len() != 3
            || !object.contains_key("target_path")
            || !object.contains_key("coverage_id")
            || !object.contains_key("source_kind")
        {
            return Err(invalid_result(
                "historical candidate field source is not exact-closed",
            ));
        }
        let target_path = object
            .get("target_path")
            .and_then(Value::as_str)
            .filter(|path| is_safe_candidate_target_path(path))
            .ok_or_else(|| invalid_result("historical candidate target path is invalid"))?;
        if !target_paths.insert(target_path) {
            return Err(invalid_result(
                "historical candidate target paths are not unique",
            ));
        }
        object
            .get("coverage_id")
            .and_then(Value::as_str)
            .filter(|id| is_dotted_symbolic_id(id))
            .ok_or_else(|| invalid_result("historical candidate coverage ID is invalid"))?;
        let source_kind = object
            .get("source_kind")
            .and_then(Value::as_str)
            .ok_or_else(|| invalid_result("historical candidate source kind is absent"))?;
        if !matches!(
            source_kind,
            "user_declaration"
                | "evidenced_inference"
                | "deterministic_default"
                | "known_unknown"
                | "contradiction"
                | "waiver"
        ) {
            return Err(invalid_result(
                "historical candidate source kind is unsupported",
            ));
        }
    }
    Ok(())
}

fn validate_historical_coverage_ids(value: &Value) -> Result<(), LifecycleValidationErrorV1> {
    let ids = value
        .get("unresolved_coverage_ids")
        .and_then(Value::as_array)
        .ok_or_else(|| {
            invalid_result("historical candidate unresolved coverage must be an array")
        })?;
    let mut unique = BTreeSet::new();
    for id in ids {
        let id = id
            .as_str()
            .filter(|id| is_dotted_symbolic_id(id))
            .ok_or_else(|| {
                invalid_result("historical candidate unresolved coverage ID is invalid")
            })?;
        if !unique.insert(id) {
            return Err(invalid_result(
                "historical candidate unresolved coverage IDs are not unique",
            ));
        }
    }
    Ok(())
}

fn validate_historical_result_refs(
    value: &Value,
    version: &str,
) -> Result<(), LifecycleValidationErrorV1> {
    let refs = value
        .get("validation_result_refs")
        .and_then(Value::as_array)
        .ok_or_else(|| invalid_result("historical candidate result refs must be an array"))?;
    if version == "1.2" && refs.len() != 1 {
        return Err(invalid_result(
            "historical candidate 1.2 must bind exactly one semantic result ref",
        ));
    }
    let mut unique = BTreeSet::new();
    for reference in refs {
        let reference = reference
            .as_str()
            .ok_or_else(|| invalid_result("historical candidate result ref is not a string"))?;
        let exact_result = is_content_addressed_ref(
            reference,
            "lifecycle-validation-results/lifecycle-validation-result_",
            ".json",
        );
        let legacy_result =
            matches!(version, "1.0" | "1.1") && SymbolicId::parse(reference).is_ok();
        if !(exact_result || legacy_result) || !unique.insert(reference) {
            return Err(invalid_result(
                "historical candidate result refs are unsafe or duplicated",
            ));
        }
    }
    Ok(())
}

fn is_content_addressed_ref(reference: &str, prefix: &str, suffix: &str) -> bool {
    reference
        .strip_prefix(prefix)
        .and_then(|value| value.strip_suffix(suffix))
        .is_some_and(|hex| {
            hex.len() == 64
                && hex
                    .bytes()
                    .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
        })
}

fn is_safe_relative_reference(reference: &str) -> bool {
    (1..=1024).contains(&reference.len())
        && reference.is_ascii()
        && !reference.starts_with('/')
        && !reference.contains('\\')
        && reference.split('/').all(|component| {
            !component.is_empty()
                && !matches!(component, "." | "..")
                && component.bytes().all(|byte| {
                    byte.is_ascii_lowercase()
                        || byte.is_ascii_digit()
                        || matches!(byte, b'_' | b'-' | b'.')
                })
        })
}

fn is_safe_candidate_target_path(path: &str) -> bool {
    (2..=1024).contains(&path.len())
        && path.is_ascii()
        && path.starts_with('/')
        && path[1..].split('/').all(|component| {
            !component.is_empty()
                && component.bytes().all(|byte| {
                    byte.is_ascii_lowercase()
                        || byte.is_ascii_digit()
                        || matches!(byte, b'_' | b'-')
                })
        })
}

fn is_dotted_symbolic_id(value: &str) -> bool {
    (1..=256).contains(&value.len())
        && value.is_ascii()
        && value
            .split('.')
            .all(|segment| SymbolicId::parse(segment).is_ok())
}

fn candidate_subject_fingerprint(value: &Value) -> Result<String, LifecycleValidationErrorV1> {
    let mut preimage = value.clone();
    let object = preimage
        .as_object_mut()
        .ok_or_else(|| invalid_result("candidate subject preimage is not an object"))?;
    for field in [
        "candidate_id",
        "candidate_fingerprint",
        "candidate_subject_fingerprint",
        "validation_result_binding",
        "validation_result_refs",
    ] {
        object.remove(field);
    }
    DefinitionFingerprint::from_json_value(&preimage)
        .map(|fingerprint| fingerprint.to_string())
        .map_err(|_| invalid_result("candidate subject preimage is not canonical"))
}

fn owned_candidate_filename(name: &str) -> bool {
    owned_sha256_filename(name, "candidate_", 79)
}

fn owned_result_filename(name: &str) -> bool {
    owned_sha256_filename(name, "lifecycle-validation-result_", 97)
}

fn owned_sha256_filename(name: &str, prefix: &str, exact_len: usize) -> bool {
    if !name.is_ascii() || name.len() != exact_len {
        return false;
    }
    name.strip_prefix(prefix)
        .and_then(|rest| rest.strip_suffix(".json"))
        .is_some_and(|hex| {
            hex.len() == 64
                && hex
                    .bytes()
                    .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
        })
}

fn inventory_path_identity(
    path: &Path,
    is_directory: bool,
) -> Result<InventoryFileIdentityV1, LifecycleValidationErrorV1> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        let metadata = fs::symlink_metadata(path)
            .map_err(|_| filesystem_error("author inventory identity lookup failed"))?;
        if metadata.is_dir() != is_directory {
            return Err(filesystem_error(
                "author inventory native file identity type is unsafe",
            ));
        }
        Ok(InventoryFileIdentityV1([
            metadata.dev(),
            metadata.ino(),
            metadata.len(),
            metadata.mtime() as u64,
            metadata.mtime_nsec() as u64,
            metadata.mode() as u64,
        ]))
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;

        let metadata = fs::symlink_metadata(path)
            .map_err(|_| filesystem_error("author inventory identity lookup failed"))?;
        if metadata.is_dir() != is_directory {
            return Err(filesystem_error(
                "author inventory native file identity type is unsafe",
            ));
        }
        let (volume, identity_high, identity_low) = match file_id::get_file_id(path)
            .map_err(|_| filesystem_error("author inventory native file identity lookup failed"))?
        {
            file_id::FileId::LowRes {
                volume_serial_number,
                file_index,
            } => (u64::from(volume_serial_number), 0, file_index),
            file_id::FileId::HighRes {
                volume_serial_number,
                file_id,
            } => (volume_serial_number, (file_id >> 64) as u64, file_id as u64),
            file_id::FileId::Inode { .. } => {
                return Err(filesystem_error(
                    "author inventory returned a non-Windows file identity",
                ))
            }
        };
        Ok(InventoryFileIdentityV1([
            volume,
            identity_high,
            identity_low,
            metadata.file_size(),
            metadata.last_write_time(),
            u64::from(metadata.file_attributes()),
        ]))
    }
    #[cfg(all(not(unix), not(windows)))]
    {
        let _ = (path, is_directory);
        Err(filesystem_error(
            "author inventory file identity is unsupported on this platform",
        ))
    }
}

struct AuthorLockV1 {
    file: File,
}

impl AuthorLockV1 {
    fn acquire(repo_root: &Path) -> Result<Self, LifecycleValidationErrorV1> {
        let lock_root = repo_root.join(".handbook/state/locks");
        create_safe_directories(repo_root, &lock_root).map_err(|failure| {
            filesystem_error(format!("author lock root refused: {}", failure.detail()))
        })?;
        let path = lock_root.join("author-project_authority.lock");
        let file = match create_new_file(&path) {
            Ok(file) => file,
            Err(failure) if failure.kind() == std::io::ErrorKind::AlreadyExists => {
                reject_reparse_or_symlink(&path, false).map_err(|failure| {
                    filesystem_error(format!("author lock path is unsafe: {}", failure.detail()))
                })?;
                let mut options = OpenOptions::new();
                options.read(true).write(true);
                #[cfg(windows)]
                {
                    use std::os::windows::fs::OpenOptionsExt;
                    const FILE_FLAG_OPEN_REPARSE_POINT: u32 = 0x0020_0000;
                    options.custom_flags(FILE_FLAG_OPEN_REPARSE_POINT);
                }
                #[cfg(unix)]
                {
                    use std::os::unix::fs::OpenOptionsExt;
                    options.custom_flags(rustix::fs::OFlags::NOFOLLOW.bits() as i32);
                }
                options
                    .open(&path)
                    .map_err(|_| filesystem_error("author lock no-follow open failed"))?
            }
            Err(_) => return Err(io_error("author lock create-new failed")),
        };
        if !file
            .metadata()
            .map_err(|_| filesystem_error("author lock metadata failed"))?
            .is_file()
        {
            return Err(filesystem_error("author lock is not a regular file"));
        }
        file.lock()
            .map_err(|_| io_error("exclusive author lock acquisition failed"))?;
        Ok(Self { file })
    }
}

impl Drop for AuthorLockV1 {
    fn drop(&mut self) {
        let _ = self.file.unlock();
    }
}

fn result_ref(fingerprint: &str) -> String {
    let hex = fingerprint
        .strip_prefix("sha256:")
        .expect("validated result fingerprint");
    format!("{RESULT_PARTITION}/lifecycle-validation-result_{hex}.json")
}

fn candidate_string<'a>(
    value: &'a Value,
    field: &str,
) -> Result<&'a str, LifecycleValidationErrorV1> {
    value
        .get(field)
        .and_then(Value::as_str)
        .ok_or_else(|| authority_error(format!("candidate `{field}` is not a string")))
}

fn fingerprint_from_bound_ref(
    reference: &str,
    prefix: &str,
    suffix: &str,
) -> Result<String, LifecycleValidationErrorV1> {
    let hex = reference
        .strip_prefix(prefix)
        .and_then(|rest| rest.strip_suffix(suffix))
        .ok_or_else(|| authority_error("content-addressed authority ref is malformed"))?;
    let fingerprint = format!("sha256:{hex}");
    DefinitionFingerprint::parse(&fingerprint)
        .map_err(|_| authority_error("content-addressed authority ref is not lowercase SHA-256"))?;
    Ok(fingerprint)
}

fn lifecycle_state_name(state: CharterLifecycleState) -> &'static str {
    match state {
        CharterLifecycleState::Current => "current",
        CharterLifecycleState::ReviewRequired => "review_required",
        CharterLifecycleState::ReassessmentRequired => "reassessment_required",
    }
}

fn is_utc_timestamp(value: &str) -> bool {
    value.len() >= 20
        && value.ends_with('Z')
        && value.as_bytes().get(4) == Some(&b'-')
        && value.as_bytes().get(7) == Some(&b'-')
        && value.as_bytes().get(10) == Some(&b'T')
        && value.as_bytes().get(13) == Some(&b':')
        && value.as_bytes().get(16) == Some(&b':')
}

fn current_utc() -> String {
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs() as i64)
        .unwrap_or(0);
    let days = seconds.div_euclid(86_400);
    let seconds_of_day = seconds.rem_euclid(86_400);
    let (year, month, day) = civil_date_from_unix_days(days);
    let hour = seconds_of_day / 3_600;
    let minute = seconds_of_day % 3_600 / 60;
    let second = seconds_of_day % 60;
    format!("{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}Z")
}

fn civil_date_from_unix_days(days: i64) -> (i64, i64, i64) {
    let shifted = days + 719_468;
    let era = if shifted >= 0 {
        shifted
    } else {
        shifted - 146_096
    } / 146_097;
    let day_of_era = shifted - era * 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let mut year = year_of_era + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_prime = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_prime + 2) / 5 + 1;
    let month = month_prime + if month_prime < 10 { 3 } else { -9 };
    if month <= 2 {
        year += 1;
    }
    (year, month, day)
}

fn authority_error(detail: impl Into<String>) -> LifecycleValidationErrorV1 {
    LifecycleValidationErrorV1::new(LifecycleValidationErrorKindV1::AuthorityMismatch, detail)
}

fn invalid_result(detail: impl Into<String>) -> LifecycleValidationErrorV1 {
    LifecycleValidationErrorV1::new(LifecycleValidationErrorKindV1::InvalidResult, detail)
}

fn filesystem_error(detail: impl Into<String>) -> LifecycleValidationErrorV1 {
    LifecycleValidationErrorV1::new(LifecycleValidationErrorKindV1::UnsafeFilesystem, detail)
}

fn io_error(detail: impl Into<String>) -> LifecycleValidationErrorV1 {
    LifecycleValidationErrorV1::new(LifecycleValidationErrorKindV1::IoFailure, detail)
}

#[cfg(test)]
mod exact_result_inventory_tests {
    use super::*;
    use crate::{
        parse_definition_yaml, resolve_shipped_profile_decisions, CharterAcquisitionMode,
        CharterAuthorPersistenceServiceV1, CharterCoverageSubmission, CharterIntakeConsumer,
        CharterIntakeEnvelope, CharterIntakeSourceKind,
    };

    const RUNTIME_VECTORS: &[u8] = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../docs/specs/handbook-contract-membrane/slices/HCM-2.2/contracts/runtime-record-fingerprint-vectors-v1.0.json"
    ));
    const AUTHORITY_REPAIR_VECTORS: &[u8] = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../docs/specs/handbook-contract-membrane/slices/HCM-2.2/contracts/authority-repair-runtime-vectors-v1.0.json"
    ));
    const CANONICAL_CHARTER: &[u8] = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../docs/specs/handbook-contract-membrane/slices/HCM-2.2/contracts/canonical-charter-boundary-v1.1.yaml"
    ));

    #[test]
    fn inventory_limit_and_owned_name_boundaries_match_the_normative_vectors() {
        let vectors: Value = serde_json::from_slice(AUTHORITY_REPAIR_VECTORS).unwrap();
        let limits = &vectors["author_inventory_contract"]["limits_per_root"];
        assert_eq!(limits["max_immediate_entries"], MAX_INVENTORY_ENTRIES);
        assert_eq!(limits["max_entry_bytes"], MAX_INVENTORY_FILE_BYTES);
        assert_eq!(limits["max_aggregate_bytes"], MAX_INVENTORY_AGGREGATE_BYTES);
        assert_eq!(
            limits["max_filename_utf8_bytes"],
            MAX_INVENTORY_FILENAME_BYTES
        );

        assert!(inventory_entry_count_within_bound(4095));
        assert!(inventory_entry_count_within_bound(4096));
        assert!(!inventory_entry_count_within_bound(4097));
        assert!(inventory_file_length_within_bound(262_143));
        assert!(inventory_file_length_within_bound(262_144));
        assert!(!inventory_file_length_within_bound(262_145));
        assert_eq!(
            inventory_aggregate_after(0, 1_073_741_824).unwrap(),
            1_073_741_824
        );
        assert!(inventory_aggregate_after(0, 1_073_741_825).is_err());
        assert!(inventory_aggregate_after(u64::MAX, 1).is_err());
        assert!(inventory_filename_length_within_bound(127));
        assert!(inventory_filename_length_within_bound(128));
        assert!(!inventory_filename_length_within_bound(129));

        let candidate_name = format!("candidate_{}.json", "0".repeat(64));
        let result_name = format!("lifecycle-validation-result_{}.json", "0".repeat(64));
        assert_eq!(candidate_name.len(), 79);
        assert_eq!(result_name.len(), 97);
        assert!(owned_candidate_filename(&candidate_name));
        assert!(owned_result_filename(&result_name));
        assert!(!owned_candidate_filename(&format!(
            "candidate_{}.json",
            "0".repeat(63)
        )));
        assert!(!owned_result_filename(&format!(
            "lifecycle-validation-result_{}.json",
            "0".repeat(65)
        )));
    }

    #[test]
    fn stable_inventory_refuses_an_owned_file_added_between_complete_scans() {
        let repo = tempfile::tempdir().unwrap();
        let vectors: Value = serde_json::from_slice(RUNTIME_VECTORS).unwrap();
        let candidate = vectors["vectors"]
            .as_array()
            .unwrap()
            .iter()
            .find(|row| row["record_class"] == "candidate")
            .unwrap()["record"]
            .clone();
        let intake = vectors["vectors"]
            .as_array()
            .unwrap()
            .iter()
            .find(|row| row["record_class"] == "intake")
            .unwrap()["record"]
            .clone();
        let intake_ref = candidate["intake_record_ref"].as_str().unwrap();
        let intake_path = repo.path().join(".handbook/state").join(intake_ref);
        fs::create_dir_all(intake_path.parent().unwrap()).unwrap();
        fs::write(
            intake_path,
            serde_json_canonicalizer::to_vec(&intake).unwrap(),
        )
        .unwrap();
        let content_ref = candidate["normalized_content_ref"].as_str().unwrap();
        let content_path = repo.path().join(".handbook/state").join(content_ref);
        fs::create_dir_all(content_path.parent().unwrap()).unwrap();
        fs::write(content_path, CANONICAL_CHARTER).unwrap();
        let candidate_id = candidate["candidate_id"].as_str().unwrap();
        let candidate_path = repo
            .path()
            .join(CANDIDATE_INVENTORY_ROOT)
            .join(format!("{candidate_id}.json"));
        let outcome = inventory_author_authority_stable_with(repo.path(), || {
            fs::create_dir_all(candidate_path.parent().unwrap()).unwrap();
            fs::write(
                &candidate_path,
                serde_json_canonicalizer::to_vec(&candidate).unwrap(),
            )
            .unwrap();
        });
        let failure = match outcome {
            Err(failure) => failure,
            Ok(_) => panic!("a complete-scan tuple mutation must refuse"),
        };

        assert_eq!(
            failure.kind(),
            LifecycleValidationErrorKindV1::AuthorityMismatch
        );
        assert_eq!(
            failure.detail(),
            "candidate/result stores changed during the stable author inventory"
        );
        assert!(candidate_path.is_file());
    }

    #[test]
    fn post_result_candidate_publication_failure_preserves_an_unadopted_orphan() {
        let repo = tempfile::tempdir().unwrap();
        let decisions =
            resolve_shipped_profile_decisions(Path::new(env!("CARGO_MANIFEST_DIR"))).unwrap();
        let service = CharterAuthorPersistenceServiceV1::new(repo.path());
        REFUSE_CANDIDATE_PUBLICATION_AFTER_RESULT.with(|selected| selected.set(true));

        service
            .persist(&decisions, test_author_envelope(), None)
            .expect_err("the test-only post-result candidate refusal must surface");
        let result_root = repo.path().join(RESULT_INVENTORY_ROOT);
        let result_path = fs::read_dir(&result_root)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .find(|path| path.extension().and_then(|value| value.to_str()) == Some("json"))
            .expect("the exact result orphan is preserved");
        let result_bytes = fs::read(&result_path).unwrap();
        let candidate_root = repo.path().join(CANDIDATE_INVENTORY_ROOT);
        assert!(
            !candidate_root.exists() || fs::read_dir(&candidate_root).unwrap().next().is_none(),
            "no candidate may be inferred after the injected failure"
        );

        service
            .persist(&decisions, test_author_envelope(), None)
            .expect_err("retry must refuse rather than adopt the result orphan");
        assert_eq!(fs::read(result_path).unwrap(), result_bytes);
        assert!(!candidate_root.exists() || fs::read_dir(candidate_root).unwrap().next().is_none());
    }

    fn test_author_envelope() -> CharterIntakeEnvelope {
        let observational = [
            "project_shape.definition",
            "delivery.constraints",
            "operational_reality.production_state",
            "risk.domains",
            "debt.register",
        ];
        let normative = [
            "delivery.default_implications",
            "engineering_posture.baseline",
            "policy.authority_and_revision",
            "governance.decision_authority",
            "governance.required_approvals",
            "governance.exception_policy",
            "engineering_posture.dimensions",
            "engineering_posture.red_lines",
            "governance.review_triggers",
            "governance.reassessment_triggers",
            "decisions.records",
        ];
        let mut coverage = Vec::new();
        for coverage_id in observational {
            coverage.push(CharterCoverageSubmission {
                coverage_id: coverage_id.to_owned(),
                source_kind: CharterIntakeSourceKind::EvidencedInference,
                value_ref: format!("intake-values/{coverage_id}.json"),
                evidence_refs: vec![format!("evidence.{coverage_id}")],
                confidence: "high".to_owned(),
                freshness: Some("session".to_owned()),
                sensitivity: "internal".to_owned(),
                contradiction_refs: vec![],
                waiver_ref: None,
            });
        }
        for coverage_id in normative {
            coverage.push(CharterCoverageSubmission {
                coverage_id: coverage_id.to_owned(),
                source_kind: CharterIntakeSourceKind::UserDeclaration,
                value_ref: format!("intake-values/{coverage_id}.json"),
                evidence_refs: vec![],
                confidence: "high".to_owned(),
                freshness: None,
                sensitivity: "internal".to_owned(),
                contradiction_refs: vec![],
                waiver_ref: None,
            });
        }
        CharterIntakeEnvelope {
            mode: CharterAcquisitionMode::Express,
            content: parse_definition_yaml(CANONICAL_CHARTER).unwrap(),
            coverage,
            consumer: CharterIntakeConsumer {
                kind: "handbook_skill".to_owned(),
                id: "handbook".to_owned(),
                version: "1.1".to_owned(),
            },
            prompt_event_refs: vec![],
            finalized_at_utc: "2026-07-21T18:00:00Z".to_owned(),
            expected_current_fingerprint: None,
        }
    }
}
