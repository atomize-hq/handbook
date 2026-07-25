use crate::artifact_instance::{DependencyCardinality, DependencyTargetKind, RequirednessMode};
use crate::artifact_kind_registry::ArtifactKindRegistry;
use crate::definition_identity::{
    DefinitionFingerprint, ExactDefinitionRef, RegistryLoadError, RegistryLoadErrorKind,
};
use crate::instance_profile::SymbolicId;
use crate::profile_selection::ResolvedInstanceProfile;
use crate::schema_registry::{ResolvedBindingJsonType, StructuralValidationError};
use crate::semantic_capability_registry::{
    AllowedInstanceCardinality, SemanticValidationProfileDefinition,
};
use crate::stable_role_registry::StableRoleDefinition;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug)]
pub struct ResolvedArtifactRegistry {
    profile_ref: ExactDefinitionRef,
    profile_fingerprint: DefinitionFingerprint,
    stable_role_registry_ref: ExactDefinitionRef,
    stable_role_registry_fingerprint: DefinitionFingerprint,
    kind_refs: Vec<ExactDefinitionRef>,
    instance_ids: Vec<SymbolicId>,
    kinds: BTreeMap<ExactDefinitionRef, ResolvedArtifactKind>,
    instances: BTreeMap<SymbolicId, ResolvedArtifactInstance>,
    dependency_order: Vec<SymbolicId>,
    kind_registry: ArtifactKindRegistry,
}

#[derive(Clone, Debug)]
pub struct ResolvedArtifactKind {
    exact_ref: ExactDefinitionRef,
    definition_fingerprint: DefinitionFingerprint,
    canonical_schema_ref: ExactDefinitionRef,
    schema_entry_fingerprint: DefinitionFingerprint,
    schema_document_fingerprint: DefinitionFingerprint,
    schema_closure_fingerprint: DefinitionFingerprint,
    supported_role_ids: Vec<String>,
    capabilities: Vec<ResolvedArtifactCapability>,
}

#[derive(Clone, Debug)]
pub struct ResolvedArtifactInstance {
    id: SymbolicId,
    kind_ref: ExactDefinitionRef,
    role: Option<StableRoleDefinition>,
    capabilities: Vec<ResolvedArtifactCapability>,
    label: String,
    canonical_path: String,
    requiredness_mode: RequirednessMode,
    condition_ref: Option<ExactDefinitionRef>,
    dependencies: Vec<ResolvedArtifactDependency>,
    lifecycle_policy_ref: Option<ExactDefinitionRef>,
    intake_definition_ref: Option<ExactDefinitionRef>,
    renderer_definition_refs: Vec<ExactDefinitionRef>,
    projection_definition_refs: Vec<ExactDefinitionRef>,
    validation_overlay_refs: Vec<ExactDefinitionRef>,
    extensions: BTreeMap<String, Value>,
}

#[derive(Clone, Debug)]
pub struct ResolvedArtifactCapability {
    capability_id: SymbolicId,
    contract_ref: ExactDefinitionRef,
    contract_fingerprint: DefinitionFingerprint,
    required_bindings: Vec<SymbolicId>,
    allowed_instance_cardinality: AllowedInstanceCardinality,
    bindings: BTreeMap<SymbolicId, String>,
    semantic_validators: Vec<SemanticValidationProfileDefinition>,
}

#[derive(Clone, Debug)]
pub struct ResolvedArtifactDependency {
    target_kind: DependencyTargetKind,
    target_ref: SymbolicId,
    target_contract_ref: Option<ExactDefinitionRef>,
    cardinality: DependencyCardinality,
    provider_ids: Vec<SymbolicId>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ArtifactRegistryValidationError {
    UnknownArtifactInstance,
    Structural(Vec<StructuralValidationError>),
}

impl ResolvedArtifactRegistry {
    pub fn from_profile(profile: &ResolvedInstanceProfile) -> Result<Self, RegistryLoadError> {
        let kind_registry = profile.artifact_kind_registry().clone();
        let kind_refs = kind_registry.kind_refs();
        let mut kinds = BTreeMap::new();
        for exact_ref in &kind_refs {
            let source_kind = kind_registry.kind(exact_ref).ok_or_else(|| {
                RegistryLoadError::at(
                    RegistryLoadErrorKind::ConflictingIdentity,
                    "artifact_registry/kinds",
                    "selected kind ref is absent from the resolved kind registry",
                )
            })?;
            let schema_entry = kind_registry
                .schema_registry()
                .entry(source_kind.canonical_schema_ref())
                .ok_or_else(|| {
                    RegistryLoadError::at(
                        RegistryLoadErrorKind::UnsupportedDependency,
                        exact_ref.as_str(),
                        "selected kind schema entry is absent from the resolved schema registry",
                    )
                })?;
            let capabilities = source_kind
                .semantic_capabilities()
                .iter()
                .map(|(capability_id, selected)| {
                    resolved_capability(
                        &kind_registry,
                        capability_id,
                        selected.contract_ref(),
                        selected.bindings(),
                    )
                })
                .collect::<Result<Vec<_>, _>>()?;
            kinds.insert(
                exact_ref.clone(),
                ResolvedArtifactKind {
                    exact_ref: exact_ref.clone(),
                    definition_fingerprint: source_kind.definition_fingerprint().clone(),
                    canonical_schema_ref: source_kind.canonical_schema_ref().clone(),
                    schema_entry_fingerprint: schema_entry.entry_fingerprint().clone(),
                    schema_document_fingerprint: schema_entry.document_fingerprint().clone(),
                    schema_closure_fingerprint: schema_entry.closure_fingerprint().clone(),
                    supported_role_ids: source_kind.supported_role_refs().to_vec(),
                    capabilities,
                },
            );
        }
        let instance_ids = profile
            .artifact_instances()
            .ids()
            .into_iter()
            .map(SymbolicId::parse)
            .map(|id| id.expect("profile instance IDs remain valid symbolic IDs"))
            .collect::<Vec<_>>();
        let mut providers_by_consumer = BTreeMap::new();
        let mut instances = BTreeMap::new();
        for id in &instance_ids {
            let descriptor = profile
                .artifact_instances()
                .instance(id)
                .expect("profile instance IDs remain present");
            let kind = kinds.get(descriptor.kind_ref()).ok_or_else(|| {
                RegistryLoadError::at(
                    RegistryLoadErrorKind::UnsupportedDependency,
                    id.as_str(),
                    "descriptor kind is absent from the selected registry",
                )
            })?;
            let role = descriptor
                .role_ref()
                .map(|role_id| {
                    profile
                        .stable_role_registry()
                        .role(role_id)
                        .cloned()
                        .ok_or_else(|| {
                            RegistryLoadError::at(
                                RegistryLoadErrorKind::UnsupportedDependency,
                                id.as_str(),
                                "descriptor role is absent from the selected stable-role registry",
                            )
                        })
                })
                .transpose()?;
            let capabilities = descriptor
                .capability_refs()
                .iter()
                .map(|capability_id| {
                    kind.capabilities
                        .iter()
                        .find(|capability| &capability.capability_id == capability_id)
                        .cloned()
                        .ok_or_else(|| {
                            RegistryLoadError::at(
                                RegistryLoadErrorKind::UnsupportedDependency,
                                id.as_str(),
                                "descriptor capability is absent from its selected kind",
                            )
                        })
                })
                .collect::<Result<Vec<_>, _>>()?;
            let mut provider_set = BTreeSet::new();
            let dependencies = descriptor
                .dependencies()
                .iter()
                .map(|dependency| {
                    let resolved =
                        resolve_dependency(profile, &kind_registry, &instance_ids, dependency)?;
                    provider_set.extend(resolved.provider_ids.iter().cloned());
                    Ok(resolved)
                })
                .collect::<Result<Vec<_>, RegistryLoadError>>()?;
            providers_by_consumer.insert(id.clone(), provider_set);
            instances.insert(
                id.clone(),
                ResolvedArtifactInstance {
                    id: id.clone(),
                    kind_ref: descriptor.kind_ref().clone(),
                    role,
                    capabilities,
                    label: descriptor.label().to_string(),
                    canonical_path: descriptor.canonical_path().to_string(),
                    requiredness_mode: descriptor.requiredness_mode(),
                    condition_ref: descriptor.condition_ref().cloned(),
                    dependencies,
                    lifecycle_policy_ref: descriptor.lifecycle_policy_ref().cloned(),
                    intake_definition_ref: descriptor.intake_definition_ref().cloned(),
                    renderer_definition_refs: descriptor.renderer_definition_refs().to_vec(),
                    projection_definition_refs: descriptor.projection_definition_refs().to_vec(),
                    validation_overlay_refs: descriptor.validation_overlay_refs().to_vec(),
                    extensions: descriptor.extensions().clone(),
                },
            );
        }
        let dependency_order = dependency_order(&instance_ids, &providers_by_consumer)?;
        Ok(Self {
            profile_ref: profile.exact_ref().clone(),
            profile_fingerprint: profile.resolved_profile_fingerprint().clone(),
            stable_role_registry_ref: profile.stable_role_registry().exact_ref().clone(),
            stable_role_registry_fingerprint: profile.stable_role_registry().fingerprint().clone(),
            kind_refs,
            dependency_order,
            instance_ids,
            kinds,
            instances,
            kind_registry,
        })
    }

    pub fn profile_ref(&self) -> &ExactDefinitionRef {
        &self.profile_ref
    }

    pub fn profile_fingerprint(&self) -> &DefinitionFingerprint {
        &self.profile_fingerprint
    }

    pub fn stable_role_registry_ref(&self) -> &ExactDefinitionRef {
        &self.stable_role_registry_ref
    }

    pub fn stable_role_registry_fingerprint(&self) -> &DefinitionFingerprint {
        &self.stable_role_registry_fingerprint
    }

    pub fn kind_refs(&self) -> Vec<&ExactDefinitionRef> {
        self.kind_refs.iter().collect()
    }

    pub fn instance_ids(&self) -> Vec<&SymbolicId> {
        self.instance_ids.iter().collect()
    }

    pub fn kind(&self, exact_ref: &ExactDefinitionRef) -> Option<&ResolvedArtifactKind> {
        self.kinds.get(exact_ref)
    }

    pub fn instance(&self, id: &SymbolicId) -> Option<&ResolvedArtifactInstance> {
        self.instances.get(id)
    }

    pub fn dependency_order(&self) -> &[SymbolicId] {
        &self.dependency_order
    }

    pub fn validate_json(
        &self,
        id: &SymbolicId,
        value: &Value,
    ) -> Result<(), ArtifactRegistryValidationError> {
        let Some(instance) = self.instances.get(id) else {
            return Err(ArtifactRegistryValidationError::UnknownArtifactInstance);
        };
        self.kind_registry
            .validate_json(&instance.kind_ref, value)
            .map_err(ArtifactRegistryValidationError::Structural)
    }

    #[doc(hidden)]
    pub fn intake_schema_leaf_shapes(
        &self,
        schema_ref: &ExactDefinitionRef,
    ) -> Result<BTreeMap<String, ResolvedBindingJsonType>, RegistryLoadError> {
        self.kind_registry
            .schema_registry()
            .resolved(schema_ref)
            .ok_or_else(|| {
                RegistryLoadError::at(
                    RegistryLoadErrorKind::UnsupportedDependency,
                    schema_ref.as_str(),
                    "intake candidate schema is absent from the resolved registry",
                )
            })?
            .coverage_leaf_shapes()
    }
}

impl ResolvedArtifactKind {
    pub fn exact_ref(&self) -> &ExactDefinitionRef {
        &self.exact_ref
    }

    pub fn definition_fingerprint(&self) -> &DefinitionFingerprint {
        &self.definition_fingerprint
    }

    pub fn canonical_schema_ref(&self) -> &ExactDefinitionRef {
        &self.canonical_schema_ref
    }

    pub fn schema_entry_fingerprint(&self) -> &DefinitionFingerprint {
        &self.schema_entry_fingerprint
    }

    pub fn schema_document_fingerprint(&self) -> &DefinitionFingerprint {
        &self.schema_document_fingerprint
    }

    pub fn schema_closure_fingerprint(&self) -> &DefinitionFingerprint {
        &self.schema_closure_fingerprint
    }

    pub fn supported_role_ids(&self) -> &[String] {
        &self.supported_role_ids
    }

    pub fn capabilities(&self) -> &[ResolvedArtifactCapability] {
        &self.capabilities
    }
}

impl ResolvedArtifactInstance {
    pub fn id(&self) -> &SymbolicId {
        &self.id
    }

    pub fn kind_ref(&self) -> &ExactDefinitionRef {
        &self.kind_ref
    }

    pub fn role(&self) -> Option<&StableRoleDefinition> {
        self.role.as_ref()
    }

    pub fn capabilities(&self) -> &[ResolvedArtifactCapability] {
        &self.capabilities
    }

    pub fn label(&self) -> &str {
        &self.label
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

    pub fn dependencies(&self) -> &[ResolvedArtifactDependency] {
        &self.dependencies
    }

    pub fn lifecycle_policy_ref(&self) -> Option<&ExactDefinitionRef> {
        self.lifecycle_policy_ref.as_ref()
    }

    pub fn intake_definition_ref(&self) -> Option<&ExactDefinitionRef> {
        self.intake_definition_ref.as_ref()
    }

    pub fn renderer_definition_refs(&self) -> &[ExactDefinitionRef] {
        &self.renderer_definition_refs
    }

    pub fn projection_definition_refs(&self) -> &[ExactDefinitionRef] {
        &self.projection_definition_refs
    }

    pub fn validation_overlay_refs(&self) -> &[ExactDefinitionRef] {
        &self.validation_overlay_refs
    }

    pub fn extensions(&self) -> &BTreeMap<String, Value> {
        &self.extensions
    }
}

impl ResolvedArtifactCapability {
    pub fn capability_id(&self) -> &SymbolicId {
        &self.capability_id
    }

    pub fn contract_ref(&self) -> &ExactDefinitionRef {
        &self.contract_ref
    }

    pub fn contract_fingerprint(&self) -> &DefinitionFingerprint {
        &self.contract_fingerprint
    }

    pub fn required_bindings(&self) -> &[SymbolicId] {
        &self.required_bindings
    }

    pub fn allowed_instance_cardinality(&self) -> AllowedInstanceCardinality {
        self.allowed_instance_cardinality
    }

    pub fn bindings(&self) -> &BTreeMap<SymbolicId, String> {
        &self.bindings
    }

    pub fn semantic_validators(&self) -> &[SemanticValidationProfileDefinition] {
        &self.semantic_validators
    }
}

impl ResolvedArtifactDependency {
    pub fn target_kind(&self) -> DependencyTargetKind {
        self.target_kind
    }

    pub fn target_ref(&self) -> &SymbolicId {
        &self.target_ref
    }

    pub fn target_contract_ref(&self) -> Option<&ExactDefinitionRef> {
        self.target_contract_ref.as_ref()
    }

    pub fn cardinality(&self) -> DependencyCardinality {
        self.cardinality
    }

    pub fn provider_ids(&self) -> &[SymbolicId] {
        &self.provider_ids
    }
}

fn resolved_capability(
    kind_registry: &ArtifactKindRegistry,
    capability_id: &SymbolicId,
    contract_ref: &ExactDefinitionRef,
    bindings: &BTreeMap<SymbolicId, String>,
) -> Result<ResolvedArtifactCapability, RegistryLoadError> {
    let contract = kind_registry
        .semantic_capability_registry()
        .capability(contract_ref)
        .ok_or_else(|| {
            RegistryLoadError::at(
                RegistryLoadErrorKind::UnsupportedDependency,
                contract_ref.as_str(),
                "capability contract is absent from the selected semantic registry",
            )
        })?;
    if contract.capability_id() != capability_id {
        return Err(RegistryLoadError::at(
            RegistryLoadErrorKind::InvalidDependencyContract,
            contract_ref.as_str(),
            "capability contract targets another capability ID",
        ));
    }
    let semantic_validators = contract
        .semantic_validation_profile_refs()
        .iter()
        .map(|reference| {
            kind_registry
                .semantic_capability_registry()
                .validator(reference)
                .cloned()
                .ok_or_else(|| {
                    RegistryLoadError::at(
                        RegistryLoadErrorKind::UnsupportedDependency,
                        reference.as_str(),
                        "semantic validator is absent from the selected semantic registry",
                    )
                })
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(ResolvedArtifactCapability {
        capability_id: capability_id.clone(),
        contract_ref: contract_ref.clone(),
        contract_fingerprint: contract.capability_fingerprint().clone(),
        required_bindings: contract.required_bindings().to_vec(),
        allowed_instance_cardinality: contract.allowed_instance_cardinality(),
        bindings: bindings.clone(),
        semantic_validators,
    })
}

fn resolve_dependency(
    profile: &ResolvedInstanceProfile,
    kind_registry: &ArtifactKindRegistry,
    instance_ids: &[SymbolicId],
    dependency: &crate::artifact_instance::ArtifactDependency,
) -> Result<ResolvedArtifactDependency, RegistryLoadError> {
    let provider_ids = match dependency.target_kind() {
        DependencyTargetKind::Instance => {
            if !instance_ids.contains(dependency.target_ref()) {
                return Err(RegistryLoadError::at(
                    RegistryLoadErrorKind::InvalidDependencyTarget,
                    dependency.target_ref().as_str(),
                    "instance dependency target is absent from the selected registry",
                ));
            }
            vec![dependency.target_ref().clone()]
        }
        DependencyTargetKind::Capability => {
            let contract_ref = dependency.target_contract_ref().ok_or_else(|| {
                RegistryLoadError::at(
                    RegistryLoadErrorKind::InvalidDependencyContract,
                    "artifact_registry/dependencies/target_contract_ref",
                    "capability dependency lacks an exact contract ref",
                )
            })?;
            let contract = kind_registry
                .semantic_capability_registry()
                .capability(contract_ref)
                .ok_or_else(|| {
                    RegistryLoadError::at(
                        RegistryLoadErrorKind::InvalidDependencyContract,
                        contract_ref.as_str(),
                        "capability dependency contract is absent",
                    )
                })?;
            if contract.capability_id() != dependency.target_ref() {
                return Err(RegistryLoadError::at(
                    RegistryLoadErrorKind::InvalidDependencyContract,
                    contract_ref.as_str(),
                    "capability dependency contract targets another capability",
                ));
            }
            let providers = instance_ids
                .iter()
                .filter_map(|id| {
                    let descriptor = profile
                        .artifact_instances()
                        .instance(id)
                        .expect("profile instance IDs remain present");
                    let selected = descriptor
                        .capability_refs()
                        .contains(dependency.target_ref())
                        && kind_registry
                            .kind(descriptor.kind_ref())
                            .and_then(|kind| {
                                kind.semantic_capabilities().get(dependency.target_ref())
                            })
                            .is_some_and(|selected| selected.contract_ref() == contract_ref);
                    selected.then_some(id.clone())
                })
                .collect::<Vec<_>>();
            if providers.is_empty()
                || (dependency.cardinality() == DependencyCardinality::ExactlyOne
                    && providers.len() != 1)
            {
                return Err(RegistryLoadError::at(
                    RegistryLoadErrorKind::InvalidDependencyProviderCount,
                    "artifact_registry/dependencies",
                    "capability dependency provider cardinality is invalid",
                ));
            }
            providers
        }
    };
    Ok(ResolvedArtifactDependency {
        target_kind: dependency.target_kind(),
        target_ref: dependency.target_ref().clone(),
        target_contract_ref: dependency.target_contract_ref().cloned(),
        cardinality: dependency.cardinality(),
        provider_ids,
    })
}

fn dependency_order(
    instance_ids: &[SymbolicId],
    providers_by_consumer: &BTreeMap<SymbolicId, BTreeSet<SymbolicId>>,
) -> Result<Vec<SymbolicId>, RegistryLoadError> {
    let mut remaining_provider_count = instance_ids
        .iter()
        .cloned()
        .map(|id| {
            let count = providers_by_consumer.get(&id).map_or(0, BTreeSet::len);
            (id, count)
        })
        .collect::<BTreeMap<_, _>>();
    let mut dependents_by_provider = BTreeMap::<SymbolicId, BTreeSet<SymbolicId>>::new();
    for (consumer, providers) in providers_by_consumer {
        for provider in providers {
            dependents_by_provider
                .entry(provider.clone())
                .or_default()
                .insert(consumer.clone());
        }
    }
    let mut ready = remaining_provider_count
        .iter()
        .filter_map(|(id, count)| (*count == 0).then_some(id.clone()))
        .collect::<BTreeSet<_>>();
    let mut ordered = Vec::with_capacity(instance_ids.len());
    while let Some(provider) = ready.pop_first() {
        ordered.push(provider.clone());
        if let Some(dependents) = dependents_by_provider.get(&provider) {
            for dependent in dependents {
                let count = remaining_provider_count
                    .get_mut(dependent)
                    .expect("dependent is a selected instance");
                *count -= 1;
                if *count == 0 {
                    ready.insert(dependent.clone());
                }
            }
        }
    }
    if ordered.len() != instance_ids.len() {
        return Err(RegistryLoadError::at(
            RegistryLoadErrorKind::DependencyCycle,
            "artifact_registry/dependencies",
            "artifact registry dependency graph contains a cycle",
        ));
    }
    Ok(ordered)
}
