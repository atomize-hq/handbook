use crate::artifact_instance::RequirednessMode;
use crate::artifact_registry::{ResolvedArtifactInstance, ResolvedArtifactRegistry};
use crate::charter_definition_registry::load_shipped_charter_definition_registry;
use crate::definition_identity::{DefinitionFingerprint, ExactDefinitionRef, RegistryLoadError};
use crate::instance_profile::{
    DefinitionSource, DefinitionSourceBinding, ProfileLoadError, ProfileSelectionRequest,
    SymbolicId,
};
use crate::profile_selection::{resolve_profile_selection, ResolvedInstanceProfile};
use serde::Serialize;
use std::collections::BTreeMap;
use std::path::Path;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ProjectConditionOutcome {
    True,
    False,
    Unknown,
    Unresolved,
    Stale,
    Refused,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ProjectConditionDecisionReason {
    EvidenceContractUnavailable,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectConditionEvaluation {
    condition_ref: ExactDefinitionRef,
    condition_definition_fingerprint: DefinitionFingerprint,
    outcome: ProjectConditionOutcome,
    reason: ProjectConditionDecisionReason,
    evidence_closure_fingerprint: Option<DefinitionFingerprint>,
}

impl ProjectConditionEvaluation {
    pub fn condition_ref(&self) -> &ExactDefinitionRef {
        &self.condition_ref
    }
    pub fn condition_definition_fingerprint(&self) -> &DefinitionFingerprint {
        &self.condition_definition_fingerprint
    }
    pub fn outcome(&self) -> ProjectConditionOutcome {
        self.outcome
    }
    pub fn reason(&self) -> ProjectConditionDecisionReason {
        self.reason
    }
    pub fn evidence_closure_fingerprint(&self) -> Option<&DefinitionFingerprint> {
        self.evidence_closure_fingerprint.as_ref()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactApplicability {
    Required,
    Optional,
    Indeterminate,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProfileCapabilityTruth {
    instance_id: SymbolicId,
    capability_id: SymbolicId,
    contract_ref: ExactDefinitionRef,
    contract_fingerprint: DefinitionFingerprint,
}

impl ProfileCapabilityTruth {
    pub fn instance_id(&self) -> &SymbolicId {
        &self.instance_id
    }
    pub fn capability_id(&self) -> &SymbolicId {
        &self.capability_id
    }
    pub fn contract_ref(&self) -> &ExactDefinitionRef {
        &self.contract_ref
    }
    pub fn contract_fingerprint(&self) -> &DefinitionFingerprint {
        &self.contract_fingerprint
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtifactProfileDecision {
    instance_id: SymbolicId,
    kind_ref: ExactDefinitionRef,
    role_id: Option<String>,
    canonical_path: String,
    requiredness_mode: RequirednessMode,
    condition_ref: Option<ExactDefinitionRef>,
    condition_outcome: Option<ProjectConditionOutcome>,
    condition_reason: Option<ProjectConditionDecisionReason>,
    evidence_closure_fingerprint: Option<DefinitionFingerprint>,
    applicability: ArtifactApplicability,
    capabilities: Vec<ProfileCapabilityTruth>,
}

impl ArtifactProfileDecision {
    pub fn instance_id(&self) -> &SymbolicId {
        &self.instance_id
    }
    pub fn kind_ref(&self) -> &ExactDefinitionRef {
        &self.kind_ref
    }
    pub fn role_id(&self) -> Option<&str> {
        self.role_id.as_deref()
    }
    pub fn canonical_path(&self) -> &str {
        &self.canonical_path
    }
    pub fn requiredness_mode(&self) -> RequirednessMode {
        self.requiredness_mode
    }
    pub fn condition_ref(&self) -> Option<&ExactDefinitionRef> {
        self.condition_ref.as_ref()
    }
    pub fn condition_outcome(&self) -> Option<ProjectConditionOutcome> {
        self.condition_outcome
    }
    pub fn condition_reason(&self) -> Option<ProjectConditionDecisionReason> {
        self.condition_reason
    }
    pub fn evidence_closure_fingerprint(&self) -> Option<&DefinitionFingerprint> {
        self.evidence_closure_fingerprint.as_ref()
    }
    pub fn applicability(&self) -> ArtifactApplicability {
        self.applicability
    }
    pub fn capabilities(&self) -> &[ProfileCapabilityTruth] {
        &self.capabilities
    }
}

#[derive(Clone, Debug)]
pub struct ResolvedProfileDecisions {
    registry: ResolvedArtifactRegistry,
    profile_definition_fingerprint: DefinitionFingerprint,
    condition_evaluations: Vec<ProjectConditionEvaluation>,
    artifact_decisions: Vec<ArtifactProfileDecision>,
    capability_truth: Vec<ProfileCapabilityTruth>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProfileDecisionError {
    Registry(RegistryLoadError),
    MissingConditionDefinition { condition_ref: ExactDefinitionRef },
}

#[derive(Debug)]
pub enum ShippedProfileDecisionError {
    Profile(ProfileLoadError),
    Decision(ProfileDecisionError),
}

pub fn resolve_shipped_profile_decisions(
    repo_root: impl AsRef<Path>,
) -> Result<ResolvedProfileDecisions, ShippedProfileDecisionError> {
    let profile = resolve_profile_selection(repo_root, shipped_profile_request())
        .map_err(ShippedProfileDecisionError::Profile)?;
    let decisions = ResolvedProfileDecisions::from_profile(&profile)
        .map_err(ShippedProfileDecisionError::Decision)?;
    load_shipped_charter_definition_registry()
        .and_then(|registry| registry.validate_selected_decisions(&decisions))
        .map_err(|error| {
            ShippedProfileDecisionError::Decision(ProfileDecisionError::Registry(error))
        })?;
    Ok(decisions)
}

impl ResolvedProfileDecisions {
    pub fn from_profile(profile: &ResolvedInstanceProfile) -> Result<Self, ProfileDecisionError> {
        let registry = ResolvedArtifactRegistry::from_profile(profile)
            .map_err(ProfileDecisionError::Registry)?;
        let mut evaluations = BTreeMap::new();
        let mut artifact_decisions = registry
            .instance_ids()
            .into_iter()
            .map(|instance_id| {
                let instance = registry
                    .instance(instance_id)
                    .expect("selected registry retains every selected instance");
                artifact_decision(profile, instance, &mut evaluations)
            })
            .collect::<Result<Vec<_>, _>>()?;
        artifact_decisions.sort_by(|left, right| left.instance_id.cmp(&right.instance_id));

        let condition_evaluations = evaluations.into_values().collect::<Vec<_>>();
        let mut capability_truth = artifact_decisions
            .iter()
            .flat_map(|decision| decision.capabilities.iter().cloned())
            .collect::<Vec<_>>();
        capability_truth.sort_by(|left, right| {
            (&left.instance_id, &left.capability_id, &left.contract_ref).cmp(&(
                &right.instance_id,
                &right.capability_id,
                &right.contract_ref,
            ))
        });

        Ok(Self {
            registry,
            profile_definition_fingerprint: profile
                .selected_profile_definition_fingerprint()
                .clone(),
            condition_evaluations,
            artifact_decisions,
            capability_truth,
        })
    }

    pub fn profile_ref(&self) -> &ExactDefinitionRef {
        self.registry.profile_ref()
    }
    pub fn profile_fingerprint(&self) -> &DefinitionFingerprint {
        self.registry.profile_fingerprint()
    }
    pub fn profile_definition_fingerprint(&self) -> &DefinitionFingerprint {
        &self.profile_definition_fingerprint
    }
    pub fn stable_role_registry_ref(&self) -> &ExactDefinitionRef {
        self.registry.stable_role_registry_ref()
    }
    pub fn stable_role_registry_fingerprint(&self) -> &DefinitionFingerprint {
        self.registry.stable_role_registry_fingerprint()
    }
    pub fn condition_evaluations(&self) -> &[ProjectConditionEvaluation] {
        &self.condition_evaluations
    }
    pub fn artifact_decisions(&self) -> &[ArtifactProfileDecision] {
        &self.artifact_decisions
    }
    pub fn capability_truth(&self) -> &[ProfileCapabilityTruth] {
        &self.capability_truth
    }
    pub fn registry(&self) -> &ResolvedArtifactRegistry {
        &self.registry
    }
}

fn artifact_decision(
    profile: &ResolvedInstanceProfile,
    instance: &ResolvedArtifactInstance,
    evaluations: &mut BTreeMap<ExactDefinitionRef, ProjectConditionEvaluation>,
) -> Result<ArtifactProfileDecision, ProfileDecisionError> {
    let (condition_outcome, condition_reason, applicability) = match instance.requiredness_mode() {
        RequirednessMode::Always => (None, None, ArtifactApplicability::Required),
        RequirednessMode::Optional => (None, None, ArtifactApplicability::Optional),
        RequirednessMode::Conditional => {
            let condition_ref = instance
                .condition_ref()
                .expect("resolved conditional instance retains its condition ref");
            let definition = profile
                .project_condition_registry()
                .definition(condition_ref)
                .ok_or_else(|| ProfileDecisionError::MissingConditionDefinition {
                    condition_ref: condition_ref.clone(),
                })?;
            evaluations.entry(condition_ref.clone()).or_insert_with(|| {
                ProjectConditionEvaluation {
                    condition_ref: condition_ref.clone(),
                    condition_definition_fingerprint: definition.definition_fingerprint().clone(),
                    outcome: ProjectConditionOutcome::Unresolved,
                    reason: ProjectConditionDecisionReason::EvidenceContractUnavailable,
                    evidence_closure_fingerprint: None,
                }
            });
            (
                Some(ProjectConditionOutcome::Unresolved),
                Some(ProjectConditionDecisionReason::EvidenceContractUnavailable),
                ArtifactApplicability::Indeterminate,
            )
        }
    };

    let mut capabilities = instance
        .capabilities()
        .iter()
        .map(|capability| ProfileCapabilityTruth {
            instance_id: instance.id().clone(),
            capability_id: capability.capability_id().clone(),
            contract_ref: capability.contract_ref().clone(),
            contract_fingerprint: capability.contract_fingerprint().clone(),
        })
        .collect::<Vec<_>>();
    capabilities.sort_by(|left, right| {
        (&left.capability_id, &left.contract_ref).cmp(&(&right.capability_id, &right.contract_ref))
    });

    Ok(ArtifactProfileDecision {
        instance_id: instance.id().clone(),
        kind_ref: instance.kind_ref().clone(),
        role_id: instance.role().map(|role| role.role_id().to_owned()),
        canonical_path: instance.canonical_path().to_owned(),
        requiredness_mode: instance.requiredness_mode(),
        condition_ref: instance.condition_ref().cloned(),
        condition_outcome,
        condition_reason,
        evidence_closure_fingerprint: None,
        applicability,
        capabilities,
    })
}

fn shipped_profile_request() -> ProfileSelectionRequest {
    let schema_refs = [
        "handbook.schemas.artifacts.project-authority@1.0.0",
        "handbook.schemas.artifacts.project-authority@1.1.0",
        "handbook.schemas.artifacts.project-context@1.0.0",
        "handbook.schemas.artifacts.environment-context@1.0.0",
        "handbook.schemas.artifacts.environment-context@1.1.0",
        "handbook.schemas.artifacts.work-specification@1.0.0",
        "handbook.schemas.artifacts.decision-record@1.0.0",
        "handbook.schemas.artifacts.risk-record@1.0.0",
        "handbook.schemas.lifecycle.trigger-evidence@1.0.0",
        "handbook.schemas.security.approver-registry@1.0.0",
        "handbook.schemas.security.approver-registry-transition@1.0.0",
        "handbook.schemas.security.authenticator-registration-request@1.0.0",
        "handbook.schemas.security.authenticator-challenge@1.0.0",
        "handbook.schemas.security.authenticator-make-credential-response@1.0.0",
        "handbook.schemas.security.authenticator-registration@1.0.0",
        "handbook.schemas.security.authenticator-get-assertion-response@1.0.0",
        "handbook.schemas.security.authenticator-assertion@1.0.0",
        "handbook.schemas.security.approver-admin-api@1.0.0",
    ];
    let kind_refs = [
        "handbook.artifact-kind.project-authority@1.0.0",
        "handbook.artifact-kind.project-authority@1.1.0",
        "handbook.artifact-kind.project-context@1.0.0",
        "handbook.artifact-kind.project-context@1.1.0",
        "handbook.artifact-kind.environment-context@1.0.0",
        "handbook.artifact-kind.environment-context@1.1.0",
        "handbook.artifact-kind.work-specification@1.0.0",
        "handbook.artifact-kind.work-specification@1.1.0",
        "handbook.artifact-kind.decision-record@1.0.0",
        "handbook.artifact-kind.decision-record@1.1.0",
        "handbook.artifact-kind.risk-record@1.0.0",
        "handbook.artifact-kind.risk-record@1.1.0",
    ];
    ProfileSelectionRequest {
        selected_profile_ref: exact("handbook.profile.shipped-root@1.2.0"),
        profile_sources: vec![builtin("handbook.profile.shipped-root@1.2.0")],
        stable_role_registry_sources: vec![builtin("handbook.roles.core@1.1.0")],
        schema_entry_sources: schema_refs
            .iter()
            .map(|reference| builtin(reference))
            .collect(),
        artifact_kind_sources: kind_refs
            .iter()
            .map(|reference| builtin(reference))
            .collect(),
        semantic_capability_sources: vec![builtin(
            "handbook.capabilities.constitutional-root@1.0.0",
        )],
        semantic_validator_sources: vec![
            builtin("handbook.semantic-validation.constitutional-root@1.0.0"),
            builtin("handbook.semantic-validation.constitutional-root@1.1.0"),
        ],
        project_condition_sources: Vec::new(),
        vocabulary_sources: vec![builtin("handbook.vocabulary.shipped-root@1.0.0")],
        context_resolution_sources: vec![builtin("handbook.context-resolution.shipped-root@1.0.0")],
        context_resolution_policy_sources: vec![
            builtin("handbook.mutation-matcher.core@1.0.0"),
            builtin("handbook.resolution-escalation.core@1.0.0"),
            builtin("handbook.memory-promotion.core@1.0.0"),
        ],
        intake_definition_sources: vec![],
        allowed_schema_roots: vec!["definitions/schemas".to_owned()],
    }
}

fn exact(value: &str) -> ExactDefinitionRef {
    ExactDefinitionRef::parse(value).expect("package-owned exact definition ref")
}

fn builtin(value: &str) -> DefinitionSourceBinding {
    let definition_ref = exact(value);
    DefinitionSourceBinding {
        definition_ref: definition_ref.clone(),
        source: DefinitionSource::BuiltIn(definition_ref),
    }
}
