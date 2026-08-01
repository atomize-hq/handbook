use crate::approver_registry_mutation::recover_complete_registry_approval_authority_locked;
use crate::approver_registry_observation::observe_committed_approver_registry_if_present_locked;
use crate::artifact_intake::{
    evaluate_coverage, CoverageEvaluationOutcomeV1, CoverageEvaluationV1, CoverageInputDocumentV1,
    CoverageSubmissionV1,
};
use crate::artifact_intake_registry::{
    AcquisitionModeV1, ArtifactIntakeDefinitionV1, ArtifactIntakeRegistry, ArtifactIntakeSourceV1,
    ArtifactRegistrationErrorV1, RepositoryProfileSelectionV1, REPOSITORY_PROFILE_SELECTION_PATH,
};
use crate::artifact_lineage_store::GenericArtifactLineageStoreV1;
use crate::artifact_operation_context::{ArtifactOperationAuthorityV1, ArtifactOperationContextV1};
use crate::artifact_operations::{
    list_artifact_instances, list_artifact_kinds, ArtifactInstanceListItemV1,
    ArtifactKindListItemV1, ArtifactOperationErrorV1, ArtifactOperationServiceV1,
    ArtifactReadResultV1, ArtifactValidationResultV1,
};
use crate::charter_authority_workflow::RegistryAuthorityLocks;
use crate::stable_role_registry::read_trusted_repo_source;
use crate::{
    resolve_profile_selection, ArtifactInstanceDescriptor, DefinitionFingerprint, DefinitionSource,
    ExactDefinitionRef, RegistryLoadErrorKind, ResolvedArtifactRegistry, ResolvedInstanceProfile,
    SourceByteBudget, SymbolicId, VocabularyDefinition, REPOSITORY_IDENTITY_REPO_PATH,
};
use std::fmt;
use std::path::{Path, PathBuf};

pub(crate) struct ArtifactRepositoryAuthorityGuardV1 {
    repo_root: PathBuf,
    repository_identity_fingerprint: DefinitionFingerprint,
    registry_state_fingerprint: Option<String>,
    _locks: RegistryAuthorityLocks,
}

impl ArtifactRepositoryAuthorityGuardV1 {
    pub(crate) fn acquire(repo_root: &Path) -> Result<Self, ArtifactRepositoryErrorV1> {
        let locks = RegistryAuthorityLocks::acquire(repo_root).map_err(|_| {
            repository_error(
                ArtifactRepositoryErrorKindV1::RepositoryIdentity,
                "repository authority locks could not be acquired",
            )
        })?;
        recover_complete_registry_approval_authority_locked(repo_root).map_err(|_| {
            repository_error(
                ArtifactRepositoryErrorKindV1::RepositoryIdentity,
                "repository approval authority could not be recovered",
            )
        })?;
        let observation = observe_committed_approver_registry_if_present_locked(repo_root)
            .map_err(|_| {
                repository_error(
                    ArtifactRepositoryErrorKindV1::RepositoryIdentity,
                    "committed repository authority could not be observed",
                )
            })?;
        let identity = read_authorized_repository_identity(repo_root)?;
        if observation.as_ref().is_some_and(|registry| {
            registry
                .state
                .get("repository_identity_fingerprint")
                .and_then(serde_json::Value::as_str)
                != Some(identity.as_str())
        }) {
            return Err(repository_error(
                ArtifactRepositoryErrorKindV1::RepositoryIdentity,
                "repository identity disagrees with committed registry authority",
            ));
        }
        Ok(Self {
            repo_root: repo_root.to_path_buf(),
            repository_identity_fingerprint: identity,
            registry_state_fingerprint: observation.map(|registry| registry.state_fingerprint),
            _locks: locks,
        })
    }

    pub(crate) fn repository_identity_fingerprint(&self) -> &DefinitionFingerprint {
        &self.repository_identity_fingerprint
    }

    pub(crate) fn require_current_identity(&self) -> Result<(), ArtifactRepositoryErrorV1> {
        let identity = read_authorized_repository_identity(&self.repo_root)?;
        let observation = observe_committed_approver_registry_if_present_locked(&self.repo_root)
            .map_err(|_| {
                repository_error(
                    ArtifactRepositoryErrorKindV1::RepositoryIdentity,
                    "committed repository authority could not be re-observed",
                )
            })?;
        if identity != self.repository_identity_fingerprint
            || observation
                .as_ref()
                .map(|registry| registry.state_fingerprint.as_str())
                != self.registry_state_fingerprint.as_deref()
            || observation.as_ref().is_some_and(|registry| {
                registry
                    .state
                    .get("repository_identity_fingerprint")
                    .and_then(serde_json::Value::as_str)
                    != Some(identity.as_str())
            })
        {
            return Err(repository_error(
                ArtifactRepositoryErrorKindV1::RepositoryIdentity,
                "repository authority changed during the operation",
            ));
        }
        Ok(())
    }
}

fn read_authorized_repository_identity(
    repo_root: &Path,
) -> Result<DefinitionFingerprint, ArtifactRepositoryErrorV1> {
    let mut budget = SourceByteBudget::default();
    let (_, bytes) =
        read_trusted_repo_source(repo_root, REPOSITORY_IDENTITY_REPO_PATH, &mut budget).map_err(
            |_| {
                repository_error(
                    ArtifactRepositoryErrorKindV1::RepositoryIdentity,
                    "the durable repository identity is unavailable or unsafe",
                )
            },
        )?;
    std::str::from_utf8(&bytes)
        .ok()
        .and_then(|value| DefinitionFingerprint::parse(value).ok())
        .ok_or_else(|| {
            repository_error(
                ArtifactRepositoryErrorKindV1::RepositoryIdentity,
                "the durable repository identity is malformed",
            )
        })
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ArtifactRepositoryErrorKindV1 {
    SelectionRead,
    SelectionAdmission,
    ProfileResolution,
    ArtifactRegistryResolution,
    IntakeSourceRead,
    IntakeAdmission,
    RepositoryIdentity,
    TargetResolution,
    ArtifactRead,
    ArtifactOperation,
    IntakeEvaluation,
    StructuralValidation,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtifactRepositoryErrorV1 {
    kind: ArtifactRepositoryErrorKindV1,
    detail: &'static str,
}

impl ArtifactRepositoryErrorV1 {
    pub fn kind(&self) -> ArtifactRepositoryErrorKindV1 {
        self.kind
    }

    pub fn detail(&self) -> &'static str {
        self.detail
    }
}

impl fmt::Display for ArtifactRepositoryErrorV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.detail)
    }
}

impl std::error::Error for ArtifactRepositoryErrorV1 {}

#[derive(Clone, Debug)]
pub struct ArtifactRepositoryV1 {
    repo_root: PathBuf,
    repository_identity_fingerprint: DefinitionFingerprint,
    selection_fingerprint: DefinitionFingerprint,
    profile: ResolvedInstanceProfile,
    registry: ResolvedArtifactRegistry,
    intake_registry: ArtifactIntakeRegistry,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtifactTargetV1 {
    kind_ref: ExactDefinitionRef,
    instance_id: SymbolicId,
}

impl ArtifactTargetV1 {
    pub fn parse(kind_ref: &str, instance_id: &str) -> Result<Self, ArtifactRepositoryErrorV1> {
        let kind_ref = ExactDefinitionRef::parse(kind_ref).map_err(|_| {
            repository_error(
                ArtifactRepositoryErrorKindV1::TargetResolution,
                "the requested artifact kind ref is invalid",
            )
        })?;
        let instance_id = SymbolicId::parse(instance_id).map_err(|_| {
            repository_error(
                ArtifactRepositoryErrorKindV1::TargetResolution,
                "the requested artifact instance ID is invalid",
            )
        })?;
        Ok(Self {
            kind_ref,
            instance_id,
        })
    }

    pub fn kind_ref(&self) -> &ExactDefinitionRef {
        &self.kind_ref
    }

    pub fn instance_id(&self) -> &SymbolicId {
        &self.instance_id
    }
}

impl ArtifactRepositoryV1 {
    pub fn open(repo_root: impl AsRef<Path>) -> Result<Self, ArtifactRepositoryErrorV1> {
        let repo_root = repo_root.as_ref();
        let authority = ArtifactRepositoryAuthorityGuardV1::acquire(repo_root)?;
        Self::open_under_authority(repo_root, &authority)
    }

    pub(crate) fn open_under_authority(
        repo_root: &Path,
        authority: &ArtifactRepositoryAuthorityGuardV1,
    ) -> Result<Self, ArtifactRepositoryErrorV1> {
        authority.require_current_identity()?;
        let mut source_budget = SourceByteBudget::default();
        let (_, selection_bytes) = read_trusted_repo_source(
            repo_root,
            REPOSITORY_PROFILE_SELECTION_PATH,
            &mut source_budget,
        )
        .map_err(|_| {
            repository_error(
                ArtifactRepositoryErrorKindV1::SelectionRead,
                "the fixed repository profile-selection record could not be read safely",
            )
        })?;
        let selection =
            RepositoryProfileSelectionV1::from_json_bytes(&selection_bytes).map_err(|_| {
                repository_error(
                    ArtifactRepositoryErrorKindV1::SelectionAdmission,
                    "the fixed repository profile-selection record was not admitted",
                )
            })?;
        let profile =
            resolve_profile_selection(repo_root, selection.profile_request()).map_err(|_| {
                repository_error(
                    ArtifactRepositoryErrorKindV1::ProfileResolution,
                    "the selected repository profile did not resolve",
                )
            })?;
        let registry = ResolvedArtifactRegistry::from_profile(&profile).map_err(|_| {
            repository_error(
                ArtifactRepositoryErrorKindV1::ArtifactRegistryResolution,
                "the selected artifact registry did not resolve",
            )
        })?;
        let intake_registry = load_repository_intakes(
            repo_root,
            selection.intake_definition_sources(),
            &mut source_budget,
        )?;
        intake_registry
            .validate_against_registry(&registry)
            .map_err(intake_admission_error)?;
        let identity = authority.repository_identity_fingerprint().clone();

        Ok(Self {
            repo_root: repo_root.to_path_buf(),
            repository_identity_fingerprint: identity,
            selection_fingerprint: selection.selection_fingerprint().clone(),
            profile,
            registry,
            intake_registry,
        })
    }

    pub(crate) fn repo_root(&self) -> &Path {
        &self.repo_root
    }

    pub fn selection_fingerprint(
        &self,
    ) -> Result<DefinitionFingerprint, ArtifactRepositoryErrorV1> {
        self.with_recovered_repository(|repository| {
            Ok(repository.selection_fingerprint_under_lock().clone())
        })
    }

    pub(crate) fn selection_fingerprint_under_lock(&self) -> &DefinitionFingerprint {
        &self.selection_fingerprint
    }

    pub(crate) fn registry(&self) -> &ResolvedArtifactRegistry {
        &self.registry
    }

    pub fn selected_profile_ref(&self) -> Result<ExactDefinitionRef, ArtifactRepositoryErrorV1> {
        self.with_recovered_repository(|repository| Ok(repository.profile.exact_ref().clone()))
    }

    pub fn resolved_vocabulary(&self) -> Result<VocabularyDefinition, ArtifactRepositoryErrorV1> {
        self.with_recovered_repository(|repository| Ok(repository.profile.vocabulary().clone()))
    }

    pub fn list_kinds(&self) -> Result<Vec<ArtifactKindListItemV1>, ArtifactRepositoryErrorV1> {
        self.with_recovered_repository(|repository| Ok(repository.list_kinds_under_lock()))
    }

    pub(crate) fn list_kinds_under_lock(&self) -> Vec<ArtifactKindListItemV1> {
        list_artifact_kinds(&self.registry)
    }

    pub fn list_instances(
        &self,
    ) -> Result<Vec<ArtifactInstanceListItemV1>, ArtifactRepositoryErrorV1> {
        self.with_recovered_repository(|repository| Ok(repository.list_instances_under_lock()))
    }

    pub(crate) fn list_instances_under_lock(&self) -> Vec<ArtifactInstanceListItemV1> {
        list_artifact_instances(&self.registry)
    }

    pub fn operation_context(
        &self,
        kind_ref: &ExactDefinitionRef,
        instance_id: &SymbolicId,
    ) -> Result<ArtifactOperationContextV1, ArtifactRepositoryErrorV1> {
        self.with_recovered_repository(|repository| {
            repository.operation_context_under_lock(kind_ref, instance_id)
        })
    }

    pub(crate) fn operation_context_under_lock(
        &self,
        kind_ref: &ExactDefinitionRef,
        instance_id: &SymbolicId,
    ) -> Result<ArtifactOperationContextV1, ArtifactRepositoryErrorV1> {
        let descriptor = self
            .profile
            .artifact_instances()
            .instance(instance_id)
            .ok_or_else(|| {
                repository_error(
                    ArtifactRepositoryErrorKindV1::TargetResolution,
                    "the requested artifact instance is not selected",
                )
            })?;
        ArtifactOperationContextV1::resolve(
            &self.registry,
            &self.intake_registry,
            kind_ref,
            instance_id,
            ArtifactOperationAuthorityV1 {
                repository_identity_fingerprint: self.repository_identity_fingerprint.clone(),
                selection_fingerprint: self.selection_fingerprint.clone(),
                selected_profile_definition_fingerprint: self
                    .profile
                    .selected_profile_definition_fingerprint()
                    .clone(),
                descriptor_fingerprint: descriptor_fingerprint(descriptor)?,
            },
        )
        .map_err(|_| {
            repository_error(
                ArtifactRepositoryErrorKindV1::TargetResolution,
                "the requested artifact target did not resolve under current authority",
            )
        })
    }

    pub fn read(
        &self,
        kind_ref: &ExactDefinitionRef,
        instance_id: &SymbolicId,
    ) -> Result<ArtifactReadResultV1, ArtifactRepositoryErrorV1> {
        self.with_recovered_repository(|repository| {
            repository.read_under_lock(kind_ref, instance_id)
        })
    }

    pub(crate) fn read_under_lock(
        &self,
        kind_ref: &ExactDefinitionRef,
        instance_id: &SymbolicId,
    ) -> Result<ArtifactReadResultV1, ArtifactRepositoryErrorV1> {
        let context = self.operation_context_under_lock(kind_ref, instance_id)?;
        let bytes = self.read_canonical_bytes(&context)?;
        self.operation_service(&context)?
            .read_from_bytes(&bytes)
            .map_err(operation_error)
    }

    pub fn validate(
        &self,
        kind_ref: &ExactDefinitionRef,
        instance_id: &SymbolicId,
    ) -> Result<ArtifactValidationResultV1, ArtifactRepositoryErrorV1> {
        self.with_recovered_repository(|repository| {
            repository.validate_under_lock(kind_ref, instance_id)
        })
    }

    pub(crate) fn validate_under_lock(
        &self,
        kind_ref: &ExactDefinitionRef,
        instance_id: &SymbolicId,
    ) -> Result<ArtifactValidationResultV1, ArtifactRepositoryErrorV1> {
        let context = self.operation_context_under_lock(kind_ref, instance_id)?;
        let bytes = self.read_canonical_bytes(&context)?;
        self.operation_service(&context)?
            .validate_from_bytes(&bytes)
            .map_err(operation_error)
    }

    pub fn evaluate_intake(
        &self,
        kind_ref: &ExactDefinitionRef,
        instance_id: &SymbolicId,
        acquisition_mode: AcquisitionModeV1,
        expected_current: Option<DefinitionFingerprint>,
        submissions: &[CoverageSubmissionV1],
    ) -> Result<CoverageEvaluationV1, ArtifactRepositoryErrorV1> {
        self.with_recovered_repository(|repository| {
            repository.evaluate_intake_under_lock(
                kind_ref,
                instance_id,
                acquisition_mode,
                expected_current,
                submissions,
            )
        })
    }

    pub(crate) fn evaluate_intake_under_lock(
        &self,
        kind_ref: &ExactDefinitionRef,
        instance_id: &SymbolicId,
        acquisition_mode: AcquisitionModeV1,
        expected_current: Option<DefinitionFingerprint>,
        submissions: &[CoverageSubmissionV1],
    ) -> Result<CoverageEvaluationV1, ArtifactRepositoryErrorV1> {
        let context = self.operation_context_under_lock(kind_ref, instance_id)?;
        let definition = self
            .operation_service(&context)?
            .intake_definition()
            .map_err(operation_error)?;
        let evaluation = evaluate_coverage(
            &context,
            definition,
            acquisition_mode,
            expected_current,
            submissions,
        )
        .map_err(|_| {
            repository_error(
                ArtifactRepositoryErrorKindV1::IntakeEvaluation,
                "the supplied intake coverage did not evaluate",
            )
        })?;
        if evaluation.outcome == CoverageEvaluationOutcomeV1::Complete {
            self.registry
                .validate_json(instance_id, &evaluation.normalized_content)
                .map_err(|_| {
                    repository_error(
                        ArtifactRepositoryErrorKindV1::StructuralValidation,
                        "the constructed intake candidate is structurally invalid",
                    )
                })?;
        }
        Ok(evaluation)
    }

    pub fn evaluate_intake_document(
        &self,
        target: &ArtifactTargetV1,
        acquisition_mode: AcquisitionModeV1,
        expected_current: Option<&str>,
        input_bytes: &[u8],
    ) -> Result<CoverageEvaluationV1, ArtifactRepositoryErrorV1> {
        crate::artifact_intake::require_artifact_input_document_bound(input_bytes).map_err(
            |_| {
                repository_error(
                    ArtifactRepositoryErrorKindV1::IntakeEvaluation,
                    "the coverage input document was not admitted",
                )
            },
        )?;
        self.with_recovered_repository(|repository| {
            repository.evaluate_intake_document_under_lock(
                target,
                acquisition_mode,
                expected_current,
                input_bytes,
            )
        })
    }

    fn evaluate_intake_document_under_lock(
        &self,
        target: &ArtifactTargetV1,
        acquisition_mode: AcquisitionModeV1,
        expected_current: Option<&str>,
        input_bytes: &[u8],
    ) -> Result<CoverageEvaluationV1, ArtifactRepositoryErrorV1> {
        let compare_current = expected_current.is_some();
        let expected_current = match expected_current {
            None | Some("absent") => None,
            Some(value) => Some(DefinitionFingerprint::parse(value).map_err(|_| {
                repository_error(
                    ArtifactRepositoryErrorKindV1::IntakeEvaluation,
                    "the expected current artifact fingerprint is invalid",
                )
            })?),
        };
        if compare_current
            && self.current_artifact_fingerprint_under_lock(target)? != expected_current
        {
            return Err(repository_error(
                ArtifactRepositoryErrorKindV1::IntakeEvaluation,
                "the expected current artifact fingerprint is stale",
            ));
        }
        let input = CoverageInputDocumentV1::from_bytes(input_bytes).map_err(|_| {
            repository_error(
                ArtifactRepositoryErrorKindV1::IntakeEvaluation,
                "the coverage input document was not admitted",
            )
        })?;
        self.evaluate_intake_under_lock(
            target.kind_ref(),
            target.instance_id(),
            acquisition_mode,
            expected_current,
            &input.coverage_submissions,
        )
    }

    pub fn intake_definition(
        &self,
        target: &ArtifactTargetV1,
    ) -> Result<ArtifactIntakeDefinitionV1, ArtifactRepositoryErrorV1> {
        self.with_recovered_repository(|repository| repository.intake_definition_under_lock(target))
    }

    pub(crate) fn intake_definition_under_lock(
        &self,
        target: &ArtifactTargetV1,
    ) -> Result<ArtifactIntakeDefinitionV1, ArtifactRepositoryErrorV1> {
        let context = self.operation_context_under_lock(target.kind_ref(), target.instance_id())?;
        let reference = context.intake_definition_ref().ok_or_else(|| {
            repository_error(
                ArtifactRepositoryErrorKindV1::ArtifactOperation,
                "the selected artifact has no intake definition",
            )
        })?;
        self.intake_registry
            .definition(reference)
            .cloned()
            .ok_or_else(|| {
                repository_error(
                    ArtifactRepositoryErrorKindV1::ArtifactOperation,
                    "the selected intake definition is unavailable",
                )
            })
    }

    pub fn current_artifact_fingerprint(
        &self,
        target: &ArtifactTargetV1,
    ) -> Result<Option<DefinitionFingerprint>, ArtifactRepositoryErrorV1> {
        self.with_recovered_repository(|repository| {
            repository.current_artifact_fingerprint_under_lock(target)
        })
    }

    pub(crate) fn current_artifact_fingerprint_under_lock(
        &self,
        target: &ArtifactTargetV1,
    ) -> Result<Option<DefinitionFingerprint>, ArtifactRepositoryErrorV1> {
        let context = self.operation_context_under_lock(target.kind_ref(), target.instance_id())?;
        let instance = self
            .registry
            .instance(context.instance_id())
            .expect("operation context resolved the selected instance");
        let mut budget = SourceByteBudget::default();
        match read_trusted_repo_source(&self.repo_root, instance.canonical_path(), &mut budget) {
            Ok((_, bytes)) => Ok(Some(DefinitionFingerprint::from_bytes(&bytes))),
            Err(error) if error.kind() == RegistryLoadErrorKind::MissingSource => Ok(None),
            Err(_) => Err(repository_error(
                ArtifactRepositoryErrorKindV1::ArtifactRead,
                "the descriptor-selected canonical artifact could not be observed safely",
            )),
        }
    }

    pub fn canonical_path(
        &self,
        target: &ArtifactTargetV1,
    ) -> Result<String, ArtifactRepositoryErrorV1> {
        self.with_recovered_repository(|repository| repository.canonical_path_under_lock(target))
    }

    pub(crate) fn canonical_path_under_lock(
        &self,
        target: &ArtifactTargetV1,
    ) -> Result<String, ArtifactRepositoryErrorV1> {
        let instance = self
            .registry
            .instance(target.instance_id())
            .ok_or_else(|| {
                repository_error(
                    ArtifactRepositoryErrorKindV1::TargetResolution,
                    "the requested artifact instance is not selected",
                )
            })?;
        if instance.kind_ref() != target.kind_ref() {
            return Err(repository_error(
                ArtifactRepositoryErrorKindV1::TargetResolution,
                "the requested kind does not match the selected instance",
            ));
        }
        Ok(instance.canonical_path().to_owned())
    }

    fn operation_service<'a>(
        &'a self,
        context: &'a ArtifactOperationContextV1,
    ) -> Result<ArtifactOperationServiceV1<'a>, ArtifactRepositoryErrorV1> {
        ArtifactOperationServiceV1::new(&self.registry, &self.intake_registry, context)
            .map_err(operation_error)
    }

    fn read_canonical_bytes(
        &self,
        context: &ArtifactOperationContextV1,
    ) -> Result<Vec<u8>, ArtifactRepositoryErrorV1> {
        let instance = self
            .registry
            .instance(context.instance_id())
            .ok_or_else(|| {
                repository_error(
                    ArtifactRepositoryErrorKindV1::TargetResolution,
                    "the operation-context instance is no longer selected",
                )
            })?;
        let mut budget = SourceByteBudget::default();
        read_trusted_repo_source(&self.repo_root, instance.canonical_path(), &mut budget)
            .map(|(_, bytes)| bytes)
            .map_err(|_| {
                repository_error(
                    ArtifactRepositoryErrorKindV1::ArtifactRead,
                    "the descriptor-selected canonical artifact could not be read safely",
                )
            })
    }

    fn with_recovered_repository<T>(
        &self,
        evaluate: impl FnOnce(&ArtifactRepositoryV1) -> Result<T, ArtifactRepositoryErrorV1>,
    ) -> Result<T, ArtifactRepositoryErrorV1> {
        let authority = ArtifactRepositoryAuthorityGuardV1::acquire(&self.repo_root)?;
        let store = GenericArtifactLineageStoreV1::new(&self.repo_root);
        store.evaluate_committed_read_with(
            authority.repository_identity_fingerprint().as_str(),
            crate::artifact_mutation::HCM_2_3_OWNER_SUBJECT_FINGERPRINT,
            |_| {
                repository_error(
                    ArtifactRepositoryErrorKindV1::ArtifactRead,
                    "generic artifact recovery refused the repository read",
                )
            },
            |intent| {
                crate::artifact_mutation::validate_persisted_intent_authority(
                    &self.repo_root,
                    &authority,
                    intent,
                )
            },
            |intent, ordinal, final_ordinal, bytes| {
                crate::artifact_mutation::validate_persisted_output_authority(
                    &self.repo_root,
                    &authority,
                    intent,
                    ordinal,
                    final_ordinal,
                    bytes,
                )
            },
            || {
                authority.require_current_identity()?;
                let repository = Self::open_under_authority(&self.repo_root, &authority)?;
                evaluate(&repository)
            },
        )
    }
}

fn load_repository_intakes(
    repo_root: &Path,
    bindings: &[crate::DefinitionSourceBinding],
    source_budget: &mut SourceByteBudget,
) -> Result<ArtifactIntakeRegistry, ArtifactRepositoryErrorV1> {
    let mut sources = Vec::new();
    let mut builtin_compatibility_refs = std::collections::BTreeSet::new();
    for binding in bindings {
        let bytes = match &binding.source {
            DefinitionSource::BuiltIn(exact_ref) => {
                let bytes = crate::profile_builtins::definition(exact_ref).ok_or_else(|| {
                    repository_error(
                        ArtifactRepositoryErrorKindV1::IntakeSourceRead,
                        "a built-in intake definition is absent from the compile-time allowlist",
                    )
                })?;
                if binding.definition_ref.as_str() == "handbook.intake.charter@1.0.0" {
                    builtin_compatibility_refs.insert(binding.definition_ref.clone());
                }
                bytes.bytes.to_vec()
            }
            DefinitionSource::RepositoryPath(path) => {
                if crate::profile_builtins::definition(&binding.definition_ref).is_some() {
                    return Err(repository_error(
                        ArtifactRepositoryErrorKindV1::IntakeSourceRead,
                        "a package-owned intake definition must use its built-in source",
                    ));
                }
                let (_, bytes) =
                    read_trusted_repo_source(repo_root, path, source_budget).map_err(|_| {
                        repository_error(
                            ArtifactRepositoryErrorKindV1::IntakeSourceRead,
                            "a repository-bound intake definition could not be read safely",
                        )
                    })?;
                bytes
            }
        };
        sources.push(ArtifactIntakeSourceV1 {
            definition_ref: binding.definition_ref.clone(),
            bytes,
        });
    }
    ArtifactIntakeRegistry::load_with_builtin_compatibility(&sources, &builtin_compatibility_refs)
        .map_err(intake_admission_error)
}

fn intake_admission_error(_: ArtifactRegistrationErrorV1) -> ArtifactRepositoryErrorV1 {
    repository_error(
        ArtifactRepositoryErrorKindV1::IntakeAdmission,
        "the repository-bound intake registry was not admitted",
    )
}

fn operation_error(_: ArtifactOperationErrorV1) -> ArtifactRepositoryErrorV1 {
    repository_error(
        ArtifactRepositoryErrorKindV1::ArtifactOperation,
        "the artifact operation was refused",
    )
}

fn repository_error(
    kind: ArtifactRepositoryErrorKindV1,
    detail: &'static str,
) -> ArtifactRepositoryErrorV1 {
    ArtifactRepositoryErrorV1 { kind, detail }
}

fn descriptor_fingerprint(
    descriptor: &ArtifactInstanceDescriptor,
) -> Result<DefinitionFingerprint, ArtifactRepositoryErrorV1> {
    let dependencies = descriptor
        .dependencies()
        .iter()
        .map(|dependency| {
            serde_json::json!({
                "target_kind": dependency.target_kind(),
                "target_ref": dependency.target_ref().as_str(),
                "target_contract_ref": dependency.target_contract_ref().map(ExactDefinitionRef::as_str),
                "cardinality": dependency.cardinality(),
            })
        })
        .collect::<Vec<_>>();
    DefinitionFingerprint::from_json_value(&serde_json::json!({
        "schema_id": "handbook.artifact-instance-descriptor",
        "schema_version": "1.0",
        "id": descriptor.id().as_str(),
        "kind_ref": descriptor.kind_ref().as_str(),
        "role_ref": descriptor.role_ref(),
        "capability_refs": descriptor.capability_refs().iter().map(SymbolicId::as_str).collect::<Vec<_>>(),
        "label": descriptor.label(),
        "canonical_path": descriptor.canonical_path(),
        "requiredness": {
            "mode": descriptor.requiredness().mode(),
            "condition_ref": descriptor.requiredness().condition_ref().map(ExactDefinitionRef::as_str),
        },
        "depends_on": dependencies,
        "lifecycle_policy_ref": descriptor.lifecycle_policy_ref().map(ExactDefinitionRef::as_str),
        "intake_definition_ref": descriptor.intake_definition_ref().map(ExactDefinitionRef::as_str),
        "renderer_definition_refs": descriptor.renderer_definition_refs().iter().map(ExactDefinitionRef::as_str).collect::<Vec<_>>(),
        "projection_definition_refs": descriptor.projection_definition_refs().iter().map(ExactDefinitionRef::as_str).collect::<Vec<_>>(),
        "validation_overlay_refs": descriptor.validation_overlay_refs().iter().map(ExactDefinitionRef::as_str).collect::<Vec<_>>(),
        "extensions": descriptor.extensions(),
    }))
    .map_err(|_| repository_error(
        ArtifactRepositoryErrorKindV1::TargetResolution,
        "the selected descriptor fingerprint could not be derived",
    ))
}
