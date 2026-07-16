use crate::definition_identity::{
    fingerprint_serializable, parse_definition_yaml, DefinitionFingerprint, ExactDefinitionRef,
    RegistryLoadError, RegistryLoadErrorKind, SourceByteBudget,
};
use crate::schema_registry::{SchemaRegistry, StructuralValidationError};
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
        }
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
}

#[derive(Clone, Debug)]
pub struct ArtifactKindRegistry {
    stable_role_registry: StableRoleRegistry,
    schema_registry: SchemaRegistry,
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
        let definition = authored.validate(&stable_role_registry, &schema_registry)?;
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
    semantic_capabilities: Vec<Value>,
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

impl AuthoredArtifactKindDefinition {
    fn parse(bytes: &[u8]) -> Result<Self, RegistryLoadError> {
        let value = parse_definition_yaml(bytes)?;
        serde_json::from_value(value).map_err(classify_kind_decode_error)
    }

    fn validate(
        mut self,
        stable_role_registry: &StableRoleRegistry,
        schema_registry: &SchemaRegistry,
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
        self.refuse_later_owned_dependencies()?;

        let exact_ref = ExactDefinitionRef::new(&self.kind_id, &self.kind_version)?;
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

        let supplied = DefinitionFingerprint::parse(&self.definition_fingerprint)?;
        let computed = fingerprint_serializable(&ArtifactKindFingerprintClosure {
            definition: &self,
            stable_role_registry_fingerprint: stable_role_registry.fingerprint().as_str(),
            schema_entry_fingerprint: schema_entry.entry_fingerprint().as_str(),
            schema_closure_fingerprint: schema_entry.closure_fingerprint().as_str(),
        })?;
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
            definition_fingerprint: computed,
        })
    }

    fn refuse_later_owned_dependencies(&self) -> Result<(), RegistryLoadError> {
        let refused = !self.semantic_capabilities.is_empty()
            || !self.semantic_validation_profile_refs.is_empty()
            || !self.renderer_definition_refs.is_empty()
            || !self.projection_definition_refs.is_empty()
            || !self.lifecycle_policy_ref.is_null()
            || !self.review_triggers.is_empty()
            || !self.required_capabilities.is_empty()
            || !self.extensions.is_empty();
        if refused {
            return Err(RegistryLoadError::new(
                RegistryLoadErrorKind::UnsupportedDependency,
                "HCM-1.1 refuses every non-empty later-owned artifact-kind dependency",
            ));
        }
        Ok(())
    }
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
