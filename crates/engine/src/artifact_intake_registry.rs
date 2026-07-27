use crate::artifact_registry::ResolvedArtifactRegistry;
use crate::{
    parse_definition_yaml, parse_schema_json, DefinitionFingerprint, DefinitionSource,
    DefinitionSourceBinding, ExactDefinitionRef, ProfileSelectionRequest, SourceByteBudget,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

pub const REPOSITORY_PROFILE_SELECTION_PATH: &str = ".handbook/profile-selection.json";
const MAX_EXACT_REF_BYTES: usize = 512;
const MAX_REPOSITORY_PATH_BYTES: usize = 1024;
const MAX_REPOSITORY_PATH_COMPONENTS: usize = 64;
const MAX_PROFILE_SOURCES: usize = 64;
const MAX_DEFINITION_SOURCES: usize = 512;
const MAX_SCHEMA_ROOTS: usize = 32;
const MAX_COVERAGE_ROWS: usize = 256;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ArtifactRegistrationErrorKindV1 {
    InvalidRecord,
    InvalidExactRef,
    InvalidRepositoryPath,
    SourceLimitExceeded,
    DuplicateSourceIdentity,
    DefinitionClassMismatch,
    BoundRefMismatch,
    FingerprintMismatch,
    DuplicateIdentity,
    ConflictingIdentity,
    MissingDependency,
    IncompatibleBinding,
    InvalidCoverage,
    InvalidDefinitionOrder,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtifactRegistrationErrorV1 {
    kind: ArtifactRegistrationErrorKindV1,
    location: &'static str,
    detail: &'static str,
}

impl ArtifactRegistrationErrorV1 {
    pub(crate) fn new_for_kernel(
        kind: ArtifactRegistrationErrorKindV1,
        location: &'static str,
        detail: &'static str,
    ) -> Self {
        Self {
            kind,
            location,
            detail,
        }
    }

    pub fn kind(&self) -> ArtifactRegistrationErrorKindV1 {
        self.kind
    }

    pub fn location(&self) -> &'static str {
        self.location
    }

    pub fn detail(&self) -> &'static str {
        self.detail
    }
}

impl fmt::Display for ArtifactRegistrationErrorV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.location, self.detail)
    }
}

impl std::error::Error for ArtifactRegistrationErrorV1 {}

fn registration_error(
    kind: ArtifactRegistrationErrorKindV1,
    location: &'static str,
    detail: &'static str,
) -> ArtifactRegistrationErrorV1 {
    ArtifactRegistrationErrorV1::new_for_kernel(kind, location, detail)
}

#[derive(Clone, Debug)]
pub struct RepositoryProfileSelectionV1 {
    profile_request: ProfileSelectionRequest,
    selection_fingerprint: DefinitionFingerprint,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct AuthoredRepositoryProfileSelectionV1 {
    schema_id: String,
    schema_version: String,
    selected_profile_ref: String,
    profile_sources: Vec<AuthoredSourceBindingV1>,
    stable_role_registry_sources: Vec<AuthoredSourceBindingV1>,
    schema_entry_sources: Vec<AuthoredSourceBindingV1>,
    artifact_kind_sources: Vec<AuthoredSourceBindingV1>,
    semantic_capability_sources: Vec<AuthoredSourceBindingV1>,
    semantic_validator_sources: Vec<AuthoredSourceBindingV1>,
    project_condition_sources: Vec<AuthoredSourceBindingV1>,
    vocabulary_sources: Vec<AuthoredSourceBindingV1>,
    context_resolution_sources: Vec<AuthoredSourceBindingV1>,
    context_resolution_policy_sources: Vec<AuthoredSourceBindingV1>,
    intake_definition_sources: Vec<AuthoredSourceBindingV1>,
    allowed_schema_roots: Vec<String>,
    extensions: BTreeMap<String, Value>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct AuthoredSourceBindingV1 {
    exact_ref: String,
    source: AuthoredSourceV1,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
enum AuthoredSourceV1 {
    BuiltIn,
    RepositoryPath { path: String },
}

impl RepositoryProfileSelectionV1 {
    pub fn from_json_bytes(bytes: &[u8]) -> Result<Self, ArtifactRegistrationErrorV1> {
        let mut value = parse_schema_json(bytes).map_err(|_| {
            registration_error(
                ArtifactRegistrationErrorKindV1::InvalidRecord,
                "profile_selection",
                "selection record is not duplicate-safe closed JSON",
            )
        })?;
        let authored: AuthoredRepositoryProfileSelectionV1 = serde_json::from_value(value.clone())
            .map_err(|_| {
                registration_error(
                    ArtifactRegistrationErrorKindV1::InvalidRecord,
                    "profile_selection",
                    "selection record shape is unsupported",
                )
            })?;
        if authored.schema_id != "handbook.repository-profile-selection"
            || authored.schema_version != "1.0"
            || !authored.extensions.is_empty()
        {
            return Err(registration_error(
                ArtifactRegistrationErrorKindV1::InvalidRecord,
                "profile_selection",
                "selection record identity or extensions are unsupported",
            ));
        }
        if authored.profile_sources.is_empty()
            || authored.profile_sources.len() > MAX_PROFILE_SOURCES
        {
            return Err(registration_error(
                ArtifactRegistrationErrorKindV1::SourceLimitExceeded,
                "profile_sources",
                "profile source count is outside the closed limit",
            ));
        }

        let selected_profile_ref = parse_exact_ref(&authored.selected_profile_ref)?;
        let collections = [
            ("profile", &authored.profile_sources),
            (
                "stable_role_registry",
                &authored.stable_role_registry_sources,
            ),
            ("schema_entry", &authored.schema_entry_sources),
            ("artifact_kind", &authored.artifact_kind_sources),
            ("semantic_capability", &authored.semantic_capability_sources),
            ("semantic_validator", &authored.semantic_validator_sources),
            ("project_condition", &authored.project_condition_sources),
            ("vocabulary", &authored.vocabulary_sources),
            ("context_resolution", &authored.context_resolution_sources),
            (
                "context_resolution_policy",
                &authored.context_resolution_policy_sources,
            ),
            ("intake_definition", &authored.intake_definition_sources),
        ];
        let total_sources = collections
            .iter()
            .map(|(_, bindings)| bindings.len())
            .sum::<usize>();
        if total_sources > MAX_DEFINITION_SOURCES {
            return Err(registration_error(
                ArtifactRegistrationErrorKindV1::SourceLimitExceeded,
                "definition_sources",
                "definition source count exceeds the closed limit",
            ));
        }

        let mut exact_ref_classes = BTreeMap::<String, &'static str>::new();
        for (class, bindings) in collections {
            for binding in bindings {
                parse_exact_ref(&binding.exact_ref)?;
                if let Some(previous) = exact_ref_classes.insert(binding.exact_ref.clone(), class) {
                    let kind = if previous == class {
                        ArtifactRegistrationErrorKindV1::DuplicateSourceIdentity
                    } else {
                        ArtifactRegistrationErrorKindV1::DefinitionClassMismatch
                    };
                    return Err(registration_error(
                        kind,
                        "definition_sources",
                        "an exact ref is rebound within or across definition classes",
                    ));
                }
                if let AuthoredSourceV1::RepositoryPath { path } = &binding.source {
                    validate_repository_path(path)?;
                }
            }
        }
        if !exact_ref_classes.contains_key(selected_profile_ref.as_str()) {
            return Err(registration_error(
                ArtifactRegistrationErrorKindV1::MissingDependency,
                "selected_profile_ref",
                "selected profile ref has no explicit source binding",
            ));
        }

        if authored.allowed_schema_roots.is_empty()
            || authored.allowed_schema_roots.len() > MAX_SCHEMA_ROOTS
        {
            return Err(registration_error(
                ArtifactRegistrationErrorKindV1::SourceLimitExceeded,
                "allowed_schema_roots",
                "allowed schema root count is outside the closed limit",
            ));
        }
        let mut roots = BTreeSet::new();
        for root in &authored.allowed_schema_roots {
            validate_repository_path(root)?;
            if !roots.insert(root) {
                return Err(registration_error(
                    ArtifactRegistrationErrorKindV1::DuplicateSourceIdentity,
                    "allowed_schema_roots",
                    "allowed schema root is duplicated",
                ));
            }
        }

        normalize_selection_value(&mut value)?;
        let selection_fingerprint =
            DefinitionFingerprint::from_json_value(&value).map_err(|_| {
                registration_error(
                    ArtifactRegistrationErrorKindV1::InvalidRecord,
                    "profile_selection",
                    "selection record canonicalization failed",
                )
            })?;

        let profile_request = ProfileSelectionRequest {
            selected_profile_ref,
            profile_sources: convert_bindings(authored.profile_sources)?,
            stable_role_registry_sources: convert_bindings(authored.stable_role_registry_sources)?,
            schema_entry_sources: convert_bindings(authored.schema_entry_sources)?,
            artifact_kind_sources: convert_bindings(authored.artifact_kind_sources)?,
            semantic_capability_sources: convert_bindings(authored.semantic_capability_sources)?,
            semantic_validator_sources: convert_bindings(authored.semantic_validator_sources)?,
            project_condition_sources: convert_bindings(authored.project_condition_sources)?,
            vocabulary_sources: convert_bindings(authored.vocabulary_sources)?,
            context_resolution_sources: convert_bindings(authored.context_resolution_sources)?,
            context_resolution_policy_sources: convert_bindings(
                authored.context_resolution_policy_sources,
            )?,
            intake_definition_sources: convert_bindings(authored.intake_definition_sources)?,
            allowed_schema_roots: authored.allowed_schema_roots,
        };

        Ok(Self {
            profile_request,
            selection_fingerprint,
        })
    }

    pub fn profile_request(&self) -> ProfileSelectionRequest {
        self.profile_request.clone()
    }

    pub fn selection_fingerprint(&self) -> &DefinitionFingerprint {
        &self.selection_fingerprint
    }

    pub fn intake_definition_sources(&self) -> &[DefinitionSourceBinding] {
        &self.profile_request.intake_definition_sources
    }
}

fn normalize_selection_value(value: &mut Value) -> Result<(), ArtifactRegistrationErrorV1> {
    let object = value.as_object_mut().ok_or_else(|| {
        registration_error(
            ArtifactRegistrationErrorKindV1::InvalidRecord,
            "profile_selection",
            "selection record must be an object",
        )
    })?;
    for (name, field) in object {
        if name.ends_with("_sources") {
            let bindings = field.as_array_mut().ok_or_else(|| {
                registration_error(
                    ArtifactRegistrationErrorKindV1::InvalidRecord,
                    "definition_sources",
                    "source field must be an array",
                )
            })?;
            bindings.sort_by(|left, right| {
                left.get("exact_ref")
                    .and_then(Value::as_str)
                    .cmp(&right.get("exact_ref").and_then(Value::as_str))
            });
        } else if name == "allowed_schema_roots" {
            let roots = field.as_array_mut().ok_or_else(|| {
                registration_error(
                    ArtifactRegistrationErrorKindV1::InvalidRecord,
                    "allowed_schema_roots",
                    "schema roots must be an array",
                )
            })?;
            roots.sort_by(|left, right| left.as_str().cmp(&right.as_str()));
        }
    }
    Ok(())
}

fn convert_bindings(
    bindings: Vec<AuthoredSourceBindingV1>,
) -> Result<Vec<DefinitionSourceBinding>, ArtifactRegistrationErrorV1> {
    bindings
        .into_iter()
        .map(|binding| {
            let definition_ref = parse_exact_ref(&binding.exact_ref)?;
            let source = match binding.source {
                AuthoredSourceV1::BuiltIn => DefinitionSource::BuiltIn(definition_ref.clone()),
                AuthoredSourceV1::RepositoryPath { path } => DefinitionSource::RepositoryPath(path),
            };
            Ok(DefinitionSourceBinding {
                definition_ref,
                source,
            })
        })
        .collect()
}

fn parse_exact_ref(value: &str) -> Result<ExactDefinitionRef, ArtifactRegistrationErrorV1> {
    if value.len() > MAX_EXACT_REF_BYTES {
        return Err(registration_error(
            ArtifactRegistrationErrorKindV1::InvalidExactRef,
            "exact_ref",
            "exact ref exceeds the closed byte limit",
        ));
    }
    ExactDefinitionRef::parse(value).map_err(|_| {
        registration_error(
            ArtifactRegistrationErrorKindV1::InvalidExactRef,
            "exact_ref",
            "exact ref violates the inherited identity contract",
        )
    })
}

pub fn validate_repository_path(path: &str) -> Result<(), ArtifactRegistrationErrorV1> {
    let components = path.split('/').collect::<Vec<_>>();
    let valid = !path.is_empty()
        && path.len() <= MAX_REPOSITORY_PATH_BYTES
        && path.is_ascii()
        && components.len() <= MAX_REPOSITORY_PATH_COMPONENTS
        && components.iter().all(|component| {
            !component.is_empty()
                && *component != "."
                && *component != ".."
                && component
                    .bytes()
                    .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-'))
        });
    if !valid {
        return Err(registration_error(
            ArtifactRegistrationErrorKindV1::InvalidRepositoryPath,
            "repository_path",
            "repository path violates the normalized relative-path contract",
        ));
    }
    Ok(())
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AcquisitionModeV1 {
    GuidedAdaptive,
    Express,
    AgentAssisted,
}

#[derive(Clone, Debug)]
pub struct ArtifactIntakeSourceV1 {
    pub definition_ref: ExactDefinitionRef,
    pub bytes: Vec<u8>,
}

#[derive(Clone, Debug)]
pub struct ArtifactIntakeRegistry {
    definitions: BTreeMap<ExactDefinitionRef, ArtifactIntakeDefinitionV1>,
    fingerprint: DefinitionFingerprint,
}

#[derive(Clone, Debug)]
pub struct ArtifactIntakeDefinitionV1 {
    exact_ref: ExactDefinitionRef,
    artifact_kind_ref: ExactDefinitionRef,
    candidate_schema_ref: ExactDefinitionRef,
    supported_modes: Vec<AcquisitionModeV1>,
    coverage: Vec<ArtifactCoverageDefinitionV1>,
    definition_fingerprint: DefinitionFingerprint,
    generic_eligible: bool,
}

#[derive(Clone, Debug)]
pub struct ArtifactCoverageDefinitionV1 {
    coverage_id: String,
    target_paths: Vec<String>,
    minimum_specificity: CoverageMinimumSpecificityV1,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CoverageMinimumSpecificityV1 {
    Concrete,
    Exact,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct AuthoredArtifactIntakeDefinitionV1 {
    schema_id: String,
    schema_version: String,
    intake_id: String,
    intake_version: String,
    artifact_kind_ref: String,
    candidate_schema_ref: String,
    supported_modes: Vec<AcquisitionModeV1>,
    coverage: Vec<AuthoredCoverageDefinitionV1>,
    approval_policy_ref: Option<String>,
    reassessment_triggers: Vec<String>,
    extensions: BTreeMap<String, Value>,
    intake_definition_fingerprint: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct AuthoredCoverageDefinitionV1 {
    coverage_id: String,
    target_paths: Vec<String>,
    applicability: String,
    authority_class: String,
    acquisition: AuthoredCoverageAcquisitionV1,
    evaluation: AuthoredCoverageEvaluationV1,
    prompt_guidance_refs: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct AuthoredCoverageAcquisitionV1 {
    inferable: bool,
    user_declaration_required: bool,
    evidence_kinds: Vec<String>,
    freshness: Option<Value>,
    sensitivity: String,
    deterministic_default: Option<Value>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct AuthoredCoverageEvaluationV1 {
    required: bool,
    minimum_specificity: CoverageMinimumSpecificityV1,
    minimum_confidence: String,
    unknown_policy: String,
    contradiction_policy: String,
    waiver_policy_ref: Option<String>,
}

impl ArtifactIntakeRegistry {
    pub fn empty() -> Self {
        Self {
            definitions: BTreeMap::new(),
            fingerprint: DefinitionFingerprint::from_json_value(&Value::Array(Vec::new()))
                .expect("an empty JSON array is canonicalizable"),
        }
    }

    pub fn load(sources: &[ArtifactIntakeSourceV1]) -> Result<Self, ArtifactRegistrationErrorV1> {
        Self::load_with_builtin_compatibility(sources, &BTreeSet::new())
    }

    pub(crate) fn load_with_builtin_compatibility(
        sources: &[ArtifactIntakeSourceV1],
        builtin_compatibility_refs: &BTreeSet<ExactDefinitionRef>,
    ) -> Result<Self, ArtifactRegistrationErrorV1> {
        let mut budget = SourceByteBudget::default();
        let mut definitions = BTreeMap::<ExactDefinitionRef, ArtifactIntakeDefinitionV1>::new();
        for source in sources {
            budget.admit(source.bytes.len()).map_err(|_| {
                registration_error(
                    ArtifactRegistrationErrorKindV1::SourceLimitExceeded,
                    "intake_definition_sources",
                    "intake source bytes exceed the shared source budget",
                )
            })?;
            let definition = if builtin_compatibility_refs.contains(&source.definition_ref) {
                ArtifactIntakeDefinitionV1::parse_builtin_compatibility(&source.bytes)?
            } else {
                ArtifactIntakeDefinitionV1::parse(&source.bytes)?
            };
            if definition.exact_ref != source.definition_ref {
                return Err(registration_error(
                    ArtifactRegistrationErrorKindV1::BoundRefMismatch,
                    "intake_definition_ref",
                    "bound exact ref does not match the admitted definition",
                ));
            }
            if let Some(existing) = definitions.get(&source.definition_ref) {
                let kind = if existing.definition_fingerprint == definition.definition_fingerprint {
                    ArtifactRegistrationErrorKindV1::DuplicateIdentity
                } else {
                    ArtifactRegistrationErrorKindV1::ConflictingIdentity
                };
                return Err(registration_error(
                    kind,
                    "intake_definition_ref",
                    "intake exact ref is duplicated or conflicting",
                ));
            }
            definitions.insert(source.definition_ref.clone(), definition);
        }
        let members = definitions
            .values()
            .map(|definition| {
                serde_json::json!({
                    "definition_ref": definition.exact_ref.as_str(),
                    "definition_fingerprint": definition.definition_fingerprint.as_str(),
                })
            })
            .collect::<Vec<_>>();
        let fingerprint =
            DefinitionFingerprint::from_json_value(&Value::Array(members)).map_err(|_| {
                registration_error(
                    ArtifactRegistrationErrorKindV1::InvalidRecord,
                    "intake_registry",
                    "intake registry fingerprint canonicalization failed",
                )
            })?;
        Ok(Self {
            definitions,
            fingerprint,
        })
    }

    pub fn definition(
        &self,
        exact_ref: &ExactDefinitionRef,
    ) -> Option<&ArtifactIntakeDefinitionV1> {
        self.definitions.get(exact_ref)
    }

    pub fn definition_refs(&self) -> Vec<&ExactDefinitionRef> {
        self.definitions.keys().collect()
    }

    pub fn fingerprint(&self) -> &DefinitionFingerprint {
        &self.fingerprint
    }

    pub(crate) fn validate_against_registry(
        &self,
        registry: &ResolvedArtifactRegistry,
    ) -> Result<(), ArtifactRegistrationErrorV1> {
        for definition in self
            .definitions
            .values()
            .filter(|definition| definition.generic_eligible)
        {
            let kind = registry
                .kind(definition.artifact_kind_ref())
                .ok_or_else(|| {
                    registration_error(
                        ArtifactRegistrationErrorKindV1::MissingDependency,
                        "artifact_kind_ref",
                        "intake artifact kind is absent from the selected registry",
                    )
                })?;
            if kind.canonical_schema_ref() != definition.candidate_schema_ref() {
                return Err(registration_error(
                    ArtifactRegistrationErrorKindV1::IncompatibleBinding,
                    "candidate_schema_ref",
                    "intake candidate schema does not equal the selected kind schema",
                ));
            }
            let leaves = registry
                .intake_schema_leaf_shapes(definition.candidate_schema_ref())
                .map_err(|_| {
                    registration_error(
                        ArtifactRegistrationErrorKindV1::InvalidCoverage,
                        "coverage",
                        "candidate schema coverage leaves are indeterminate",
                    )
                })?;
            let mut covered = BTreeSet::new();
            for row in &definition.coverage {
                let mut row_type = None;
                for target in &row.target_paths {
                    let target_type = leaves.get(target).copied().ok_or_else(|| {
                        registration_error(
                            ArtifactRegistrationErrorKindV1::InvalidCoverage,
                            "coverage",
                            "coverage target is not an exact candidate schema leaf",
                        )
                    })?;
                    if row_type
                        .replace(target_type)
                        .is_some_and(|prior| prior != target_type)
                    {
                        return Err(registration_error(
                            ArtifactRegistrationErrorKindV1::InvalidCoverage,
                            "coverage",
                            "one coverage value cannot target incompatible JSON types",
                        ));
                    }
                    if definition.coverage.iter().any(|other| {
                        other.target_paths.iter().any(|candidate| {
                            candidate != target
                                && (candidate
                                    .strip_prefix(target)
                                    .is_some_and(|suffix| suffix.starts_with('/'))
                                    || target
                                        .strip_prefix(candidate)
                                        .is_some_and(|suffix| suffix.starts_with('/')))
                        })
                    }) {
                        return Err(registration_error(
                            ArtifactRegistrationErrorKindV1::InvalidCoverage,
                            "coverage",
                            "coverage targets overlap by ancestor and descendant",
                        ));
                    }
                    covered.insert(target.clone());
                }
            }
            if covered != leaves.keys().cloned().collect() {
                return Err(registration_error(
                    ArtifactRegistrationErrorKindV1::InvalidCoverage,
                    "coverage",
                    "coverage does not cover every candidate schema leaf exactly once",
                ));
            }
        }
        Ok(())
    }
}

impl ArtifactIntakeDefinitionV1 {
    fn parse_builtin_compatibility(bytes: &[u8]) -> Result<Self, ArtifactRegistrationErrorV1> {
        let mut value = parse_definition_yaml(bytes).map_err(|_| {
            registration_error(
                ArtifactRegistrationErrorKindV1::InvalidRecord,
                "intake_definition",
                "built-in intake definition is not duplicate-safe closed YAML",
            )
        })?;
        let object = value.as_object_mut().ok_or_else(|| {
            registration_error(
                ArtifactRegistrationErrorKindV1::InvalidRecord,
                "intake_definition",
                "built-in intake definition must be an object",
            )
        })?;
        let supplied_fingerprint = object
            .remove("intake_definition_fingerprint")
            .and_then(|fingerprint| fingerprint.as_str().map(ToOwned::to_owned))
            .ok_or_else(|| {
                registration_error(
                    ArtifactRegistrationErrorKindV1::InvalidRecord,
                    "intake_definition_fingerprint",
                    "built-in intake definition fingerprint is required",
                )
            })?;
        let definition_fingerprint =
            DefinitionFingerprint::parse(&supplied_fingerprint).map_err(|_| {
                registration_error(
                    ArtifactRegistrationErrorKindV1::InvalidRecord,
                    "intake_definition_fingerprint",
                    "built-in intake definition fingerprint grammar is invalid",
                )
            })?;
        let string = |name: &'static str| {
            value.get(name).and_then(Value::as_str).ok_or_else(|| {
                registration_error(
                    ArtifactRegistrationErrorKindV1::InvalidRecord,
                    name,
                    "built-in intake identity field is absent",
                )
            })
        };
        if string("schema_id")? != "handbook.artifact-intake-definition"
            || string("schema_version")? != "1.0"
        {
            return Err(registration_error(
                ArtifactRegistrationErrorKindV1::InvalidRecord,
                "intake_definition",
                "built-in intake definition identity is unsupported",
            ));
        }
        let exact_ref = ExactDefinitionRef::new(string("intake_id")?, string("intake_version")?)
            .map_err(|_| {
                registration_error(
                    ArtifactRegistrationErrorKindV1::InvalidExactRef,
                    "intake_definition_ref",
                    "built-in intake identity/version is not exact",
                )
            })?;
        let artifact_kind_ref = parse_exact_ref(string("artifact_kind_ref")?)?;
        let candidate_schema_ref = parse_exact_ref(string("candidate_schema_ref")?)?;
        Ok(Self {
            exact_ref,
            artifact_kind_ref,
            candidate_schema_ref,
            supported_modes: Vec::new(),
            coverage: Vec::new(),
            definition_fingerprint,
            generic_eligible: false,
        })
    }

    fn parse(bytes: &[u8]) -> Result<Self, ArtifactRegistrationErrorV1> {
        let mut value = parse_definition_yaml(bytes).map_err(|_| {
            registration_error(
                ArtifactRegistrationErrorKindV1::InvalidRecord,
                "intake_definition",
                "intake definition is not duplicate-safe closed YAML",
            )
        })?;
        let supplied_fingerprint = value
            .as_object_mut()
            .and_then(|object| object.remove("intake_definition_fingerprint"))
            .and_then(|fingerprint| fingerprint.as_str().map(ToOwned::to_owned))
            .ok_or_else(|| {
                registration_error(
                    ArtifactRegistrationErrorKindV1::InvalidRecord,
                    "intake_definition_fingerprint",
                    "intake definition fingerprint is required",
                )
            })?;
        let string = |name: &'static str| {
            value.get(name).and_then(Value::as_str).ok_or_else(|| {
                registration_error(
                    ArtifactRegistrationErrorKindV1::InvalidRecord,
                    name,
                    "intake definition identity field is absent",
                )
            })
        };
        let p1a_ref = ExactDefinitionRef::new(string("intake_id")?, string("intake_version")?)
            .map_err(|_| {
                registration_error(
                    ArtifactRegistrationErrorKindV1::InvalidExactRef,
                    "intake_definition_ref",
                    "intake identity/version is not an exact definition ref",
                )
            })?;
        let p1a_dependencies = match p1a_ref.as_str() {
            "handbook.intake.project-context@1.0.0" => Some((
                "handbook.artifact-kind.project-context@1.1.0",
                "handbook.schemas.artifacts.project-context@1.0.0",
                include_bytes!(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/definitions/artifact-kinds/handbook.artifact-kind.project-context/1.1.0.yaml"
                ))
                .as_slice(),
                include_bytes!(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/definitions/schemas/handbook.schemas.artifacts.project-context/1.0.0.entry.yaml"
                ))
                .as_slice(),
            )),
            "handbook.intake.environment-context@1.0.0" => Some((
                "handbook.artifact-kind.environment-context@1.1.0",
                "handbook.schemas.artifacts.environment-context@1.0.0",
                include_bytes!(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/definitions/artifact-kinds/handbook.artifact-kind.environment-context/1.1.0.yaml"
                ))
                .as_slice(),
                include_bytes!(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/definitions/schemas/handbook.schemas.artifacts.environment-context/1.0.0.entry.yaml"
                ))
                .as_slice(),
            )),
            "handbook.intake.work-specification@1.0.0" => Some((
                "handbook.artifact-kind.work-specification@1.1.0",
                "handbook.schemas.artifacts.work-specification@1.0.0",
                include_bytes!(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/definitions/artifact-kinds/handbook.artifact-kind.work-specification/1.1.0.yaml"
                ))
                .as_slice(),
                include_bytes!(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/definitions/schemas/handbook.schemas.artifacts.work-specification/1.0.0.entry.yaml"
                ))
                .as_slice(),
            )),
            "handbook.intake.decision-record@1.0.0" => Some((
                "handbook.artifact-kind.decision-record@1.1.0",
                "handbook.schemas.artifacts.decision-record@1.0.0",
                include_bytes!(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/definitions/artifact-kinds/handbook.artifact-kind.decision-record/1.1.0.yaml"
                ))
                .as_slice(),
                include_bytes!(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/definitions/schemas/handbook.schemas.artifacts.decision-record/1.0.0.entry.yaml"
                ))
                .as_slice(),
            )),
            "handbook.intake.risk-record@1.0.0" => Some((
                "handbook.artifact-kind.risk-record@1.1.0",
                "handbook.schemas.artifacts.risk-record@1.0.0",
                include_bytes!(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/definitions/artifact-kinds/handbook.artifact-kind.risk-record/1.1.0.yaml"
                ))
                .as_slice(),
                include_bytes!(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/definitions/schemas/handbook.schemas.artifacts.risk-record/1.0.0.entry.yaml"
                ))
                .as_slice(),
            )),
            _ => None,
        };
        let computed_fingerprint =
            if let Some((kind_ref, schema_ref, kind_bytes, schema_bytes)) = p1a_dependencies {
                if string("artifact_kind_ref")? != kind_ref
                    || string("candidate_schema_ref")? != schema_ref
                {
                    return Err(registration_error(
                        ArtifactRegistrationErrorKindV1::IncompatibleBinding,
                        "intake_definition",
                        "P1A intake kind/schema binding differs from its frozen closure",
                    ));
                }
                let dependency_fingerprint =
                    |bytes: &[u8],
                     field: &'static str|
                     -> Result<String, ArtifactRegistrationErrorV1> {
                        let dependency = parse_definition_yaml(bytes).map_err(|_| {
                            registration_error(
                                ArtifactRegistrationErrorKindV1::InvalidRecord,
                                "intake_definition",
                                "P1A intake dependency is not duplicate-safe closed YAML",
                            )
                        })?;
                        let fingerprint = dependency
                            .get(field)
                            .and_then(Value::as_str)
                            .ok_or_else(|| {
                                registration_error(
                                    ArtifactRegistrationErrorKindV1::MissingDependency,
                                    "intake_definition",
                                    "P1A intake dependency fingerprint is absent",
                                )
                            })?;
                        DefinitionFingerprint::parse(fingerprint).map_err(|_| {
                            registration_error(
                                ArtifactRegistrationErrorKindV1::InvalidRecord,
                                "intake_definition",
                                "P1A intake dependency fingerprint grammar is invalid",
                            )
                        })?;
                        Ok(fingerprint.to_owned())
                    };
                let kind_fingerprint =
                    dependency_fingerprint(kind_bytes, "definition_fingerprint")?;
                let schema_fingerprint = dependency_fingerprint(schema_bytes, "entry_fingerprint")?;
                DefinitionFingerprint::from_json_value(&serde_json::json!({
                    "definition": value,
                    "resolved_dependencies": [
                        {
                            "definition_fingerprint": kind_fingerprint,
                            "definition_ref": kind_ref,
                            "dependency_role": "artifact_kind",
                        },
                        {
                            "definition_fingerprint": schema_fingerprint,
                            "definition_ref": schema_ref,
                            "dependency_role": "candidate_schema",
                        },
                    ],
                }))
            } else {
                DefinitionFingerprint::from_json_value(&value)
            }
            .map_err(|_| {
                registration_error(
                    ArtifactRegistrationErrorKindV1::InvalidRecord,
                    "intake_definition",
                    "intake definition canonicalization failed",
                )
            })?;
        if supplied_fingerprint != computed_fingerprint.as_str() {
            return Err(registration_error(
                ArtifactRegistrationErrorKindV1::FingerprintMismatch,
                "intake_definition_fingerprint",
                "intake definition fingerprint does not match normalized content",
            ));
        }
        value
            .as_object_mut()
            .expect("intake definition was previously an object")
            .insert(
                "intake_definition_fingerprint".to_string(),
                Value::String(supplied_fingerprint),
            );
        let authored: AuthoredArtifactIntakeDefinitionV1 =
            serde_json::from_value(value).map_err(|_| {
                registration_error(
                    ArtifactRegistrationErrorKindV1::InvalidRecord,
                    "intake_definition",
                    "intake definition shape is unsupported",
                )
            })?;
        if authored.schema_id != "handbook.artifact-intake-definition"
            || authored.schema_version != "1.0"
            || authored.approval_policy_ref.is_some()
            || !authored.reassessment_triggers.is_empty()
            || !authored.extensions.is_empty()
        {
            return Err(registration_error(
                ArtifactRegistrationErrorKindV1::InvalidRecord,
                "intake_definition",
                "intake definition selects unsupported authority or extensions",
            ));
        }
        let exact_ref = ExactDefinitionRef::new(&authored.intake_id, &authored.intake_version)
            .map_err(|_| {
                registration_error(
                    ArtifactRegistrationErrorKindV1::InvalidExactRef,
                    "intake_definition_ref",
                    "intake identity/version is not an exact definition ref",
                )
            })?;
        let artifact_kind_ref = parse_exact_ref(&authored.artifact_kind_ref)?;
        let candidate_schema_ref = parse_exact_ref(&authored.candidate_schema_ref)?;
        let definition_fingerprint = DefinitionFingerprint::parse(
            &authored.intake_definition_fingerprint,
        )
        .map_err(|_| {
            registration_error(
                ArtifactRegistrationErrorKindV1::InvalidRecord,
                "intake_definition_fingerprint",
                "intake definition fingerprint grammar is invalid",
            )
        })?;
        let modes = authored
            .supported_modes
            .iter()
            .copied()
            .collect::<BTreeSet<_>>();
        if authored.supported_modes.is_empty() || modes.len() != authored.supported_modes.len() {
            return Err(registration_error(
                ArtifactRegistrationErrorKindV1::InvalidRecord,
                "supported_modes",
                "supported acquisition modes must be nonempty and unique",
            ));
        }
        if authored.coverage.is_empty() || authored.coverage.len() > MAX_COVERAGE_ROWS {
            return Err(registration_error(
                ArtifactRegistrationErrorKindV1::InvalidCoverage,
                "coverage",
                "coverage count is outside the closed limit",
            ));
        }
        let mut coverage_ids = BTreeSet::new();
        let mut target_paths = BTreeSet::new();
        let mut coverage = Vec::with_capacity(authored.coverage.len());
        for row in authored.coverage {
            if !validate_coverage_id(&row.coverage_id)
                || !coverage_ids.insert(row.coverage_id.clone())
                || row.target_paths.is_empty()
                || row
                    .target_paths
                    .iter()
                    .any(|path| !validate_json_pointer(path) || !target_paths.insert(path.clone()))
            {
                return Err(registration_error(
                    ArtifactRegistrationErrorKindV1::InvalidCoverage,
                    "coverage",
                    "coverage IDs and target paths must be valid and unique",
                ));
            }
            let acquisition = &row.acquisition;
            let evaluation = &row.evaluation;
            if row.applicability != "always"
                || row.authority_class != "observational"
                || acquisition.inferable
                || !acquisition.user_declaration_required
                || !acquisition.evidence_kinds.is_empty()
                || acquisition.freshness.is_some()
                || acquisition.sensitivity != "internal"
                || acquisition.deterministic_default.is_some()
                || !evaluation.required
                || evaluation.minimum_confidence != "high"
                || evaluation.unknown_policy != "block"
                || evaluation.contradiction_policy != "block"
                || evaluation.waiver_policy_ref.is_some()
                || !row.prompt_guidance_refs.is_empty()
            {
                return Err(registration_error(
                    ArtifactRegistrationErrorKindV1::InvalidCoverage,
                    "coverage",
                    "coverage row attempts unsupported inference, evidence, default, waiver, or authority",
                ));
            }
            coverage.push(ArtifactCoverageDefinitionV1 {
                coverage_id: row.coverage_id,
                target_paths: row.target_paths,
                minimum_specificity: row.evaluation.minimum_specificity,
            });
        }
        Ok(Self {
            exact_ref,
            artifact_kind_ref,
            candidate_schema_ref,
            supported_modes: authored.supported_modes,
            coverage,
            definition_fingerprint,
            generic_eligible: true,
        })
    }

    pub fn exact_ref(&self) -> &ExactDefinitionRef {
        &self.exact_ref
    }

    pub fn artifact_kind_ref(&self) -> &ExactDefinitionRef {
        &self.artifact_kind_ref
    }

    pub fn candidate_schema_ref(&self) -> &ExactDefinitionRef {
        &self.candidate_schema_ref
    }

    pub fn supported_modes(&self) -> &[AcquisitionModeV1] {
        &self.supported_modes
    }

    pub fn coverage(&self) -> &[ArtifactCoverageDefinitionV1] {
        &self.coverage
    }

    pub fn definition_fingerprint(&self) -> &DefinitionFingerprint {
        &self.definition_fingerprint
    }
}

impl ArtifactCoverageDefinitionV1 {
    pub fn coverage_id(&self) -> &str {
        &self.coverage_id
    }

    pub fn target_paths(&self) -> &[String] {
        &self.target_paths
    }

    pub fn minimum_specificity(&self) -> CoverageMinimumSpecificityV1 {
        self.minimum_specificity
    }
}

fn validate_coverage_id(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 128
        && value.is_ascii()
        && value.split('.').all(|segment| {
            let bytes = segment.as_bytes();
            !bytes.is_empty()
                && bytes[0].is_ascii_lowercase()
                && bytes
                    .iter()
                    .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || *byte == b'_')
        })
}

fn validate_json_pointer(value: &str) -> bool {
    value.starts_with('/')
        && value.len() <= MAX_REPOSITORY_PATH_BYTES
        && !value.contains("//")
        && value.split('/').skip(1).all(|token| {
            !token.is_empty()
                && !token.chars().any(char::is_control)
                && valid_pointer_escapes(token)
        })
}

fn valid_pointer_escapes(token: &str) -> bool {
    let bytes = token.as_bytes();
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'~' {
            if index + 1 >= bytes.len() || !matches!(bytes[index + 1], b'0' | b'1') {
                return false;
            }
            index += 2;
        } else {
            index += 1;
        }
    }
    true
}
