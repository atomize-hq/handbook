use crate::definition_identity::{
    fingerprint_serializable, parse_definition_yaml, DefinitionFingerprint, ExactDefinitionRef,
    RegistryLoadError, RegistryLoadErrorKind, SourceByteBudget,
};
use crate::instance_profile::SymbolicId;
use crate::schema_registry::{
    ResolvedBindingCardinality, ResolvedBindingEmptyPolicy, ResolvedBindingJsonType,
    SchemaRegistry, StructuralValidationError,
};
use crate::semantic_capability_registry::{
    BindingCardinality, BindingEmptyPolicy, BindingJsonType, SemanticCapabilityRegistry,
};
use crate::stable_role_registry::{read_trusted_repo_source, StableRoleRegistry};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

const KIND_SCHEMA_ID: &str = "handbook.artifact-kind-definition";
const KIND_SCHEMA_VERSION: &str = "1.0";
const STRUCTURAL_VALIDATION_PROFILE: &str = "json-schema.draft-2020-12";

#[derive(Clone, Debug)]
pub struct ArtifactKindRegistryLoadRequest {
    stable_role_registry_ref: ExactDefinitionRef,
    schema_entry_source_paths: Vec<String>,
    allowed_schema_roots: Vec<String>,
    artifact_kind_source_paths: Vec<String>,
    semantic_capability_source_paths: Vec<String>,
    semantic_validator_source_paths: Vec<String>,
}

impl ArtifactKindRegistryLoadRequest {
    pub fn new(
        stable_role_registry_ref: ExactDefinitionRef,
        schema_entry_source_paths: Vec<String>,
        allowed_schema_roots: Vec<String>,
        artifact_kind_source_paths: Vec<String>,
    ) -> Self {
        Self {
            stable_role_registry_ref,
            schema_entry_source_paths,
            allowed_schema_roots,
            artifact_kind_source_paths,
            semantic_capability_source_paths: Vec::new(),
            semantic_validator_source_paths: Vec::new(),
        }
    }

    pub fn with_semantic_sources(
        mut self,
        semantic_capability_source_paths: Vec<String>,
        semantic_validator_source_paths: Vec<String>,
    ) -> Self {
        self.semantic_capability_source_paths = semantic_capability_source_paths;
        self.semantic_validator_source_paths = semantic_validator_source_paths;
        self
    }

    pub fn stable_role_registry_ref(&self) -> &ExactDefinitionRef {
        &self.stable_role_registry_ref
    }

    pub fn schema_entry_source_paths(&self) -> &[String] {
        &self.schema_entry_source_paths
    }

    pub fn allowed_schema_roots(&self) -> &[String] {
        &self.allowed_schema_roots
    }

    pub fn artifact_kind_source_paths(&self) -> &[String] {
        &self.artifact_kind_source_paths
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtifactKindDefinition {
    exact_ref: ExactDefinitionRef,
    canonical_schema_ref: ExactDefinitionRef,
    supported_role_refs: Vec<String>,
    semantic_capabilities: BTreeMap<SymbolicId, ArtifactKindCapability>,
    definition_fingerprint: DefinitionFingerprint,
}

impl ArtifactKindDefinition {
    pub fn exact_ref(&self) -> &ExactDefinitionRef {
        &self.exact_ref
    }

    pub fn canonical_schema_ref(&self) -> &ExactDefinitionRef {
        &self.canonical_schema_ref
    }

    pub fn supported_role_refs(&self) -> &[String] {
        &self.supported_role_refs
    }

    pub fn definition_fingerprint(&self) -> &DefinitionFingerprint {
        &self.definition_fingerprint
    }

    pub fn semantic_capabilities(&self) -> &BTreeMap<SymbolicId, ArtifactKindCapability> {
        &self.semantic_capabilities
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtifactKindCapability {
    contract_ref: ExactDefinitionRef,
    bindings: BTreeMap<SymbolicId, String>,
}

impl ArtifactKindCapability {
    pub fn contract_ref(&self) -> &ExactDefinitionRef {
        &self.contract_ref
    }
    pub fn bindings(&self) -> &BTreeMap<SymbolicId, String> {
        &self.bindings
    }
}

#[derive(Clone, Debug)]
pub struct ArtifactKindRegistry {
    stable_role_registry: StableRoleRegistry,
    schema_registry: SchemaRegistry,
    semantic_capability_registry: SemanticCapabilityRegistry,
    kinds: BTreeMap<ExactDefinitionRef, ArtifactKindDefinition>,
    fingerprint: DefinitionFingerprint,
}

impl ArtifactKindRegistry {
    pub fn fingerprint(&self) -> &DefinitionFingerprint {
        &self.fingerprint
    }

    pub fn stable_role_registry(&self) -> &StableRoleRegistry {
        &self.stable_role_registry
    }

    pub fn schema_registry(&self) -> &SchemaRegistry {
        &self.schema_registry
    }

    pub fn semantic_capability_registry(&self) -> &SemanticCapabilityRegistry {
        &self.semantic_capability_registry
    }

    pub fn kind_refs(&self) -> Vec<ExactDefinitionRef> {
        self.kinds.keys().cloned().collect()
    }

    pub fn kind(&self, exact_ref: &ExactDefinitionRef) -> Option<&ArtifactKindDefinition> {
        self.kinds.get(exact_ref)
    }

    pub fn validate_json(
        &self,
        kind_ref: &ExactDefinitionRef,
        instance: &Value,
    ) -> Result<(), Vec<StructuralValidationError>> {
        let Some(kind) = self.kind(kind_ref) else {
            return Err(Vec::new());
        };
        self.schema_registry
            .resolved(kind.canonical_schema_ref())
            .expect("admitted kind schemas remain in the owned registry")
            .validate_json(instance)
    }
}

pub fn load_artifact_kind_registry(
    repo_root: impl AsRef<Path>,
    request: ArtifactKindRegistryLoadRequest,
) -> Result<ArtifactKindRegistry, RegistryLoadError> {
    if request.artifact_kind_source_paths.is_empty() {
        return Err(RegistryLoadError::new(
            RegistryLoadErrorKind::UnsupportedRecord,
            "at least one artifact-kind definition source is required",
        ));
    }

    let stable_role_registry = StableRoleRegistry::load_builtin(&request.stable_role_registry_ref)?;
    let mut source_budget = SourceByteBudget::default();
    let schema_registry = SchemaRegistry::load_with_budget(
        repo_root.as_ref(),
        &request.schema_entry_source_paths,
        &request.allowed_schema_roots,
        &mut source_budget,
    )?;
    let semantic_capability_registry = SemanticCapabilityRegistry::load(
        repo_root.as_ref(),
        &request.semantic_capability_source_paths,
        &request.semantic_validator_source_paths,
    )?;
    let mut kinds = BTreeMap::new();

    for source_path in &request.artifact_kind_source_paths {
        let (normalized_source, bytes) =
            read_trusted_repo_source(repo_root.as_ref(), source_path, &mut source_budget)?;
        if normalized_source != *source_path {
            return Err(RegistryLoadError::at(
                RegistryLoadErrorKind::InvalidSourcePath,
                "artifact_kind_source",
                "artifact-kind source path must already be normalized",
            ));
        }
        let authored = AuthoredArtifactKindDefinition::parse(&bytes)?;
        let definition = authored.validate(
            &stable_role_registry,
            &schema_registry,
            &semantic_capability_registry,
        )?;
        if let Some(existing) = kinds.get(definition.exact_ref()) {
            let kind = if existing == &definition {
                RegistryLoadErrorKind::DuplicateIdentity
            } else {
                RegistryLoadErrorKind::ConflictingIdentity
            };
            return Err(RegistryLoadError::at(
                kind,
                "artifact_kind_definitions",
                "artifact-kind exact identity appears more than once",
            ));
        }
        kinds.insert(definition.exact_ref.clone(), definition);
    }

    let members = kinds
        .values()
        .map(|definition| ArtifactKindRegistryFingerprintMember {
            kind_ref: definition.exact_ref.as_str(),
            definition_fingerprint: definition.definition_fingerprint.as_str(),
        })
        .collect::<Vec<_>>();
    let fingerprint = fingerprint_serializable(&members)?;

    Ok(ArtifactKindRegistry {
        stable_role_registry,
        schema_registry,
        semantic_capability_registry,
        kinds,
        fingerprint,
    })
}

pub(crate) fn load_artifact_kind_registry_admitted(
    stable_role_registry: StableRoleRegistry,
    schema_registry: SchemaRegistry,
    semantic_capability_registry: SemanticCapabilityRegistry,
    sources: &[(&ExactDefinitionRef, &[u8])],
) -> Result<ArtifactKindRegistry, RegistryLoadError> {
    if sources.is_empty() {
        return Err(RegistryLoadError::new(
            RegistryLoadErrorKind::UnsupportedRecord,
            "at least one artifact-kind definition source is required",
        ));
    }
    let mut kinds = BTreeMap::new();
    for (declared_ref, bytes) in sources {
        let definition = AuthoredArtifactKindDefinition::parse(bytes)?.validate(
            &stable_role_registry,
            &schema_registry,
            &semantic_capability_registry,
        )?;
        if definition.exact_ref() != *declared_ref {
            return Err(RegistryLoadError::at(
                RegistryLoadErrorKind::ConflictingIdentity,
                "artifact_kind_source",
                "artifact-kind derived exact ref does not match its typed source binding",
            ));
        }
        if let Some(existing) = kinds.get(definition.exact_ref()) {
            let kind = if existing == &definition {
                RegistryLoadErrorKind::DuplicateIdentity
            } else {
                RegistryLoadErrorKind::ConflictingIdentity
            };
            return Err(RegistryLoadError::at(
                kind,
                "artifact_kind_definitions",
                "artifact-kind exact identity appears more than once",
            ));
        }
        kinds.insert(definition.exact_ref.clone(), definition);
    }
    let members = kinds
        .values()
        .map(|definition| ArtifactKindRegistryFingerprintMember {
            kind_ref: definition.exact_ref.as_str(),
            definition_fingerprint: definition.definition_fingerprint.as_str(),
        })
        .collect::<Vec<_>>();
    let fingerprint = fingerprint_serializable(&members)?;
    Ok(ArtifactKindRegistry {
        stable_role_registry,
        schema_registry,
        semantic_capability_registry,
        kinds,
        fingerprint,
    })
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct AuthoredArtifactKindDefinition {
    schema_id: String,
    schema_version: String,
    kind_id: String,
    kind_version: String,
    compatibility: String,
    stable_role_registry: AuthoredStableRoleRegistrySelection,
    canonical_schema_ref: String,
    supported_role_refs: Vec<String>,
    semantic_capabilities: Vec<AuthoredKindCapability>,
    structural_validation_profile_ref: String,
    semantic_validation_profile_refs: Vec<String>,
    renderer_definition_refs: Vec<String>,
    projection_definition_refs: Vec<String>,
    lifecycle_policy_ref: Value,
    review_triggers: Vec<String>,
    required_capabilities: Vec<Value>,
    extensions: BTreeMap<String, Value>,
    #[serde(skip_serializing)]
    definition_fingerprint: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct AuthoredKindCapability {
    capability_id: String,
    contract_ref: String,
    bindings: BTreeMap<String, String>,
}

impl AuthoredArtifactKindDefinition {
    fn parse(bytes: &[u8]) -> Result<Self, RegistryLoadError> {
        let value = parse_definition_yaml(bytes)?;
        serde_json::from_value(value).map_err(classify_kind_decode_error)
    }

    fn validate(
        mut self,
        stable_role_registry: &StableRoleRegistry,
        schema_registry: &SchemaRegistry,
        semantic_registry: &SemanticCapabilityRegistry,
    ) -> Result<ArtifactKindDefinition, RegistryLoadError> {
        if self.schema_id != KIND_SCHEMA_ID || self.schema_version != KIND_SCHEMA_VERSION {
            return Err(RegistryLoadError::new(
                RegistryLoadErrorKind::UnsupportedRecord,
                "artifact-kind definition must use handbook.artifact-kind-definition / 1.0",
            ));
        }
        if self.compatibility != "exact" {
            return Err(RegistryLoadError::new(
                RegistryLoadErrorKind::UnsupportedCompatibility,
                "artifact-kind compatibility must be exact",
            ));
        }
        if self.structural_validation_profile_ref != STRUCTURAL_VALIDATION_PROFILE {
            return Err(RegistryLoadError::new(
                RegistryLoadErrorKind::UnsupportedStructuralValidationProfile,
                "artifact-kind structural validation profile must be json-schema.draft-2020-12",
            ));
        }
        let exact_ref = ExactDefinitionRef::new(&self.kind_id, &self.kind_version)?;
        self.validate_later_owned_dependencies(&exact_ref)?;
        let selected_registry_ref =
            ExactDefinitionRef::parse(&self.stable_role_registry.reference)?;
        let selected_registry_fingerprint =
            DefinitionFingerprint::parse(&self.stable_role_registry.fingerprint)?;
        if selected_registry_ref != *stable_role_registry.exact_ref()
            || selected_registry_fingerprint != *stable_role_registry.fingerprint()
        {
            return Err(RegistryLoadError::new(
                RegistryLoadErrorKind::StableRoleRegistryMismatch,
                "artifact-kind stable-role registry selection does not match the exact loaded registry",
            ));
        }

        let mut roles = BTreeSet::new();
        for role_ref in &self.supported_role_refs {
            if stable_role_registry.role(role_ref).is_none() {
                return Err(RegistryLoadError::at(
                    RegistryLoadErrorKind::UnknownStableRole,
                    "supported_role_refs",
                    "artifact-kind role is absent from the selected stable-role registry",
                ));
            }
            if !roles.insert(role_ref.clone()) {
                return Err(RegistryLoadError::at(
                    RegistryLoadErrorKind::DuplicateIdentity,
                    "supported_role_refs",
                    "artifact-kind role is duplicated",
                ));
            }
        }
        self.supported_role_refs = roles.into_iter().collect();

        let canonical_schema_ref = ExactDefinitionRef::parse(&self.canonical_schema_ref)?;
        let Some(schema_entry) = schema_registry.entry(&canonical_schema_ref) else {
            return Err(RegistryLoadError::at(
                RegistryLoadErrorKind::MissingSchema,
                "canonical_schema_ref",
                "artifact-kind canonical schema is absent from the exact schema registry",
            ));
        };

        let semantic_capabilities = self.validate_semantic_capabilities(
            schema_registry,
            &canonical_schema_ref,
            semantic_registry,
        )?;

        let mut validator_refs = Vec::new();
        for value in &self.semantic_validation_profile_refs {
            let reference = ExactDefinitionRef::parse(value)?;
            if semantic_registry.validator(&reference).is_none() {
                return Err(RegistryLoadError::at(
                    RegistryLoadErrorKind::UnsupportedDependency,
                    "semantic_validation_profile_refs",
                    "semantic validator is absent from the exact typed registry",
                ));
            }
            validator_refs.push(reference);
        }
        let mut expected_validator_refs = semantic_capabilities
            .values()
            .flat_map(|selected| {
                semantic_registry
                    .capability(selected.contract_ref())
                    .expect("validated capability remains present")
                    .semantic_validation_profile_refs()
                    .iter()
                    .cloned()
            })
            .collect::<Vec<_>>();
        if exact_ref.as_str() == "handbook.artifact-kind.project-authority@1.1.0" {
            expected_validator_refs.push(ExactDefinitionRef::parse(
                "handbook.semantic-validation.constitutional-root@1.1.0",
            )?);
        }
        if validator_refs != expected_validator_refs {
            return Err(RegistryLoadError::at(
                RegistryLoadErrorKind::UnsupportedDependency,
                "semantic_validation_profile_refs",
                "kind semantic validator refs must exactly match its capability contracts",
            ));
        }

        let supplied = DefinitionFingerprint::parse(&self.definition_fingerprint)?;
        let computed = if exact_ref.as_str() == "handbook.artifact-kind.project-authority@1.1.0" {
            fingerprint_serializable(&ArtifactKindUniformClosure {
                definition: &self,
                resolved_dependencies: vec![
                    ArtifactKindUniformDependency {
                        definition_fingerprint: "sha256:7420efe464c45e17319c56a233f9a54960c52f81b502ed4ffb59a479474f9836",
                        definition_ref: "handbook.schemas.artifacts.project-authority@1.1.0",
                        dependency_role: "canonical_schema",
                    },
                    ArtifactKindUniformDependency {
                        definition_fingerprint: "sha256:1d4a1c2f85158c14524559e6846bf805eff4e531d8a78ac5c4890e2a4c0b0998",
                        definition_ref: "handbook.capabilities.constitutional-root@1.0.0",
                        dependency_role: "capability_contract",
                    },
                    ArtifactKindUniformDependency {
                        definition_fingerprint: "sha256:88caafb9caaf137647c42a91cd2762ac0871e0a20e2a1844c2c0076d5fb43cc3",
                        definition_ref: "handbook.lifecycle.constitutional-review-lock@1.0.0",
                        dependency_role: "lifecycle",
                    },
                    ArtifactKindUniformDependency {
                        definition_fingerprint: "sha256:68f6fceedaab6133364d18a2b474694d1b44b91d73c05d0af65db1aa58e80fa7",
                        definition_ref: "handbook.renderer.charter-review-markdown@1.0.0",
                        dependency_role: "renderer",
                    },
                    ArtifactKindUniformDependency {
                        definition_fingerprint: "sha256:9672246337ff266fc07f67053ca053736cb0650d2ad5e053e4a19304acd48ed8",
                        definition_ref: "handbook.lifecycle-trigger.charter-amendment-proposed@1.0.0",
                        dependency_role: "review_trigger",
                    },
                    ArtifactKindUniformDependency {
                        definition_fingerprint: "sha256:be0fb9fd4ee98e9fc1c384b710d61198e103f2bca6ac6ef2bbe14957808c9738",
                        definition_ref: "handbook.semantic-validation.constitutional-root@1.0.0",
                        dependency_role: "semantic_validator",
                    },
                    ArtifactKindUniformDependency {
                        definition_fingerprint: "sha256:10703119fbb0a4cfcab3fcdc4c618df269933591caf653786fdd8fee8cfc3c10",
                        definition_ref: "handbook.semantic-validation.constitutional-root@1.1.0",
                        dependency_role: "semantic_validator",
                    },
                    ArtifactKindUniformDependency {
                        definition_fingerprint: "sha256:0c85b1b53786e7980c4fd0d7975cd9cde1a3eae2bc8daceb23be1a1731263029",
                        definition_ref: "handbook.roles.core@1.1.0",
                        dependency_role: "stable_role_registry",
                    },
                ],
            })?
        } else if semantic_capabilities.is_empty() {
            fingerprint_serializable(&ArtifactKindFingerprintClosure {
                definition: &self,
                stable_role_registry_fingerprint: stable_role_registry.fingerprint().as_str(),
                schema_entry_fingerprint: schema_entry.entry_fingerprint().as_str(),
                schema_closure_fingerprint: schema_entry.closure_fingerprint().as_str(),
            })?
        } else {
            let capability_fingerprints = semantic_capabilities
                .values()
                .map(|selected| {
                    semantic_registry
                        .capability(selected.contract_ref())
                        .expect("validated")
                        .capability_fingerprint()
                        .as_str()
                })
                .collect::<Vec<_>>();
            let semantic_validator_fingerprints = validator_refs
                .iter()
                .map(|reference| {
                    semantic_registry
                        .validator(reference)
                        .expect("validated")
                        .profile_fingerprint()
                        .as_str()
                })
                .collect::<Vec<_>>();
            fingerprint_serializable(&ArtifactKindSemanticFingerprintClosure {
                definition: &self,
                stable_role_registry_fingerprint: stable_role_registry.fingerprint().as_str(),
                schema_entry_fingerprint: schema_entry.entry_fingerprint().as_str(),
                schema_closure_fingerprint: schema_entry.closure_fingerprint().as_str(),
                capability_fingerprints,
                semantic_validator_fingerprints,
            })?
        };
        if supplied != computed {
            return Err(RegistryLoadError::new(
                RegistryLoadErrorKind::FingerprintMismatch,
                "artifact-kind definition fingerprint does not match its exact typed closure",
            ));
        }

        Ok(ArtifactKindDefinition {
            exact_ref,
            canonical_schema_ref,
            supported_role_refs: self.supported_role_refs,
            semantic_capabilities,
            definition_fingerprint: computed,
        })
    }

    fn validate_later_owned_dependencies(
        &self,
        exact_ref: &ExactDefinitionRef,
    ) -> Result<(), RegistryLoadError> {
        if exact_ref.as_str() == "handbook.artifact-kind.project-authority@1.1.0" {
            let exact = self.renderer_definition_refs
                == ["handbook.renderer.charter-review-markdown@1.0.0"]
                && self.projection_definition_refs.is_empty()
                && self.lifecycle_policy_ref.as_str()
                    == Some("handbook.lifecycle.constitutional-review-lock@1.0.0")
                && self.review_triggers
                    == ["handbook.lifecycle-trigger.charter-amendment-proposed@1.0.0"]
                && self.required_capabilities.is_empty()
                && self.extensions.is_empty();
            if exact {
                return Ok(());
            }
            return Err(RegistryLoadError::new(
                RegistryLoadErrorKind::UnsupportedDependency,
                "Project Authority 1.1 later-owned dependencies differ from the frozen closure",
            ));
        }
        let refused = !self.renderer_definition_refs.is_empty()
            || !self.projection_definition_refs.is_empty()
            || !self.lifecycle_policy_ref.is_null()
            || !self.review_triggers.is_empty()
            || !self.required_capabilities.is_empty()
            || !self.extensions.is_empty();
        if refused {
            return Err(RegistryLoadError::new(
                RegistryLoadErrorKind::UnsupportedDependency,
                "non-Project-Authority-1.1 kinds refuse every non-empty later-owned dependency",
            ));
        }
        Ok(())
    }

    fn validate_semantic_capabilities(
        &self,
        schema_registry: &SchemaRegistry,
        canonical_schema_ref: &ExactDefinitionRef,
        semantic_registry: &SemanticCapabilityRegistry,
    ) -> Result<BTreeMap<SymbolicId, ArtifactKindCapability>, RegistryLoadError> {
        let schema = schema_registry
            .resolved(canonical_schema_ref)
            .expect("schema entry resolved");
        let mut selected = BTreeMap::new();
        for authored in &self.semantic_capabilities {
            let capability_id = SymbolicId::parse(&authored.capability_id).map_err(|_| {
                RegistryLoadError::new(
                    RegistryLoadErrorKind::InvalidExactDefinitionRef,
                    "kind capability ID is invalid",
                )
            })?;
            let contract_ref = ExactDefinitionRef::parse(&authored.contract_ref)?;
            let contract = semantic_registry.capability(&contract_ref).ok_or_else(|| {
                RegistryLoadError::new(
                    RegistryLoadErrorKind::UnsupportedDependency,
                    "kind capability contract is absent",
                )
            })?;
            if contract.capability_id() != &capability_id {
                return Err(RegistryLoadError::new(
                    RegistryLoadErrorKind::UnsupportedDependency,
                    "kind capability ID does not match contract",
                ));
            }
            let validator = contract
                .semantic_validation_profile_refs()
                .first()
                .and_then(|r| semantic_registry.validator(r))
                .ok_or_else(|| {
                    RegistryLoadError::new(
                        RegistryLoadErrorKind::UnsupportedDependency,
                        "kind capability validator is absent",
                    )
                })?;
            if authored.bindings.len() != contract.required_bindings().len() {
                return Err(RegistryLoadError::new(
                    RegistryLoadErrorKind::UnsupportedDependency,
                    "kind capability binding set is incomplete",
                ));
            }
            let mut bindings = BTreeMap::new();
            for (index, key) in contract.required_bindings().iter().enumerate() {
                let pointer = authored.bindings.get(key.as_str()).ok_or_else(|| {
                    RegistryLoadError::new(
                        RegistryLoadErrorKind::UnsupportedDependency,
                        "kind capability binding is missing",
                    )
                })?;
                let shape = schema.binding_shape(pointer)?;
                let rule = &validator.binding_rules()[index];
                let compatible = matches!(
                    (rule.json_type(), shape.json_type()),
                    (BindingJsonType::Object, ResolvedBindingJsonType::Object)
                        | (BindingJsonType::Array, ResolvedBindingJsonType::Array)
                        | (BindingJsonType::String, ResolvedBindingJsonType::String)
                ) && matches!(
                    (rule.cardinality(), shape.cardinality()),
                    (
                        BindingCardinality::Singular,
                        ResolvedBindingCardinality::Singular
                    ) | (
                        BindingCardinality::Plural,
                        ResolvedBindingCardinality::Plural
                    )
                ) && matches!(
                    (rule.empty_policy(), shape.empty_policy()),
                    (
                        BindingEmptyPolicy::Forbidden,
                        ResolvedBindingEmptyPolicy::Forbidden
                    ) | (
                        BindingEmptyPolicy::Allowed,
                        ResolvedBindingEmptyPolicy::Allowed
                    )
                );
                if !compatible {
                    return Err(RegistryLoadError::new(
                        RegistryLoadErrorKind::UnsupportedDependency,
                        "kind capability binding shape is incompatible",
                    ));
                }
                bindings.insert(key.clone(), pointer.clone());
            }
            if selected
                .insert(
                    capability_id,
                    ArtifactKindCapability {
                        contract_ref,
                        bindings,
                    },
                )
                .is_some()
            {
                return Err(RegistryLoadError::new(
                    RegistryLoadErrorKind::DuplicateIdentity,
                    "kind capability ID is duplicated",
                ));
            }
        }
        Ok(selected)
    }
}

pub(crate) fn admitted_artifact_kind_exact_ref(
    bytes: &[u8],
) -> Result<ExactDefinitionRef, RegistryLoadError> {
    let authored = AuthoredArtifactKindDefinition::parse(bytes)?;
    if authored.schema_id != KIND_SCHEMA_ID || authored.schema_version != KIND_SCHEMA_VERSION {
        return Err(RegistryLoadError::new(
            RegistryLoadErrorKind::UnsupportedRecord,
            "artifact-kind definition must use handbook.artifact-kind-definition / 1.0",
        ));
    }
    ExactDefinitionRef::new(&authored.kind_id, &authored.kind_version)
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct AuthoredStableRoleRegistrySelection {
    #[serde(rename = "ref")]
    reference: String,
    fingerprint: String,
}

#[derive(Serialize)]
struct ArtifactKindFingerprintClosure<'a> {
    definition: &'a AuthoredArtifactKindDefinition,
    stable_role_registry_fingerprint: &'a str,
    schema_entry_fingerprint: &'a str,
    schema_closure_fingerprint: &'a str,
}

#[derive(Serialize)]
struct ArtifactKindSemanticFingerprintClosure<'a> {
    definition: &'a AuthoredArtifactKindDefinition,
    stable_role_registry_fingerprint: &'a str,
    schema_entry_fingerprint: &'a str,
    schema_closure_fingerprint: &'a str,
    capability_fingerprints: Vec<&'a str>,
    semantic_validator_fingerprints: Vec<&'a str>,
}

#[derive(Serialize)]
struct ArtifactKindUniformClosure<'a> {
    definition: &'a AuthoredArtifactKindDefinition,
    resolved_dependencies: Vec<ArtifactKindUniformDependency<'a>>,
}

#[derive(Serialize)]
struct ArtifactKindUniformDependency<'a> {
    definition_fingerprint: &'a str,
    definition_ref: &'a str,
    dependency_role: &'a str,
}

#[derive(Serialize)]
struct ArtifactKindRegistryFingerprintMember<'a> {
    kind_ref: &'a str,
    definition_fingerprint: &'a str,
}

fn classify_kind_decode_error(error: serde_json::Error) -> RegistryLoadError {
    let rendered = error.to_string();
    let (kind, detail) = if rendered.contains("unknown field") {
        (
            RegistryLoadErrorKind::UnknownField,
            "artifact-kind definition contains an unknown field",
        )
    } else {
        (
            RegistryLoadErrorKind::SyntaxError,
            "artifact-kind definition does not match its closed typed record",
        )
    };
    RegistryLoadError::new(kind, detail)
}
