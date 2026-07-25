use crate::artifact_kind_registry::ArtifactKindRegistry;
use crate::canonical_repo_support::NormalizedRepoRelativePath;
use crate::definition_identity::{
    fingerprint_serializable, DefinitionFingerprint, ExactDefinitionRef, RegistryLoadError,
    RegistryLoadErrorKind,
};
use crate::instance_profile::SymbolicId;
use crate::project_condition_registry::ProjectConditionDefinition;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RequirednessMode {
    Always,
    Conditional,
    Optional,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct AuthoredRequiredness {
    mode: RequirednessMode,
    condition_ref: Option<String>,
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DependencyTargetKind {
    Instance,
    Capability,
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DependencyCardinality {
    ExactlyOne,
    AtLeastOne,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct AuthoredDependency {
    target_kind: DependencyTargetKind,
    target_ref: String,
    target_contract_ref: Option<String>,
    cardinality: DependencyCardinality,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct AuthoredDescriptor {
    schema_id: String,
    schema_version: String,
    id: String,
    kind_ref: String,
    role_ref: Option<String>,
    capability_refs: Vec<String>,
    label: String,
    canonical_path: String,
    requiredness: AuthoredRequiredness,
    depends_on: Vec<AuthoredDependency>,
    lifecycle_policy_ref: Option<String>,
    intake_definition_ref: Option<String>,
    renderer_definition_refs: Vec<String>,
    projection_definition_refs: Vec<String>,
    validation_overlay_refs: Vec<String>,
    extensions: BTreeMap<String, Value>,
}
#[derive(Clone, Debug)]
pub struct ArtifactInstanceDescriptor {
    id: SymbolicId,
    kind_ref: ExactDefinitionRef,
    role_ref: Option<String>,
    capability_refs: Vec<SymbolicId>,
    label: String,
    canonical_path: String,
    requiredness: ArtifactRequiredness,
    dependencies: Vec<ArtifactDependency>,
    lifecycle_policy_ref: Option<ExactDefinitionRef>,
    intake_definition_ref: Option<ExactDefinitionRef>,
    renderer_definition_refs: Vec<ExactDefinitionRef>,
    projection_definition_refs: Vec<ExactDefinitionRef>,
    validation_overlay_refs: Vec<ExactDefinitionRef>,
    extensions: BTreeMap<String, Value>,
}

#[derive(Clone, Debug)]
pub struct ArtifactRequiredness {
    mode: RequirednessMode,
    condition_ref: Option<ExactDefinitionRef>,
}

impl ArtifactRequiredness {
    pub fn mode(&self) -> RequirednessMode {
        self.mode
    }
    pub fn condition_ref(&self) -> Option<&ExactDefinitionRef> {
        self.condition_ref.as_ref()
    }
}

#[derive(Clone, Debug)]
pub struct ArtifactDependency {
    target_kind: DependencyTargetKind,
    target_ref: SymbolicId,
    target_contract_ref: Option<ExactDefinitionRef>,
    cardinality: DependencyCardinality,
}

impl ArtifactDependency {
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
}
impl ArtifactInstanceDescriptor {
    pub fn id(&self) -> &SymbolicId {
        &self.id
    }
    pub fn kind_ref(&self) -> &ExactDefinitionRef {
        &self.kind_ref
    }
    pub fn role_ref(&self) -> Option<&str> {
        self.role_ref.as_deref()
    }
    pub fn capability_refs(&self) -> &[SymbolicId] {
        &self.capability_refs
    }
    pub fn label(&self) -> &str {
        &self.label
    }
    pub fn canonical_path(&self) -> &str {
        &self.canonical_path
    }
    pub fn requiredness_mode(&self) -> RequirednessMode {
        self.requiredness.mode
    }
    pub fn condition_ref(&self) -> Option<&ExactDefinitionRef> {
        self.requiredness.condition_ref.as_ref()
    }
    pub fn requiredness(&self) -> &ArtifactRequiredness {
        &self.requiredness
    }
    pub fn dependencies(&self) -> &[ArtifactDependency] {
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
#[derive(Clone, Debug)]
pub struct ArtifactInstanceRegistry {
    instances: BTreeMap<SymbolicId, ArtifactInstanceDescriptor>,
    fingerprint: DefinitionFingerprint,
}

fn insert_condition_fingerprint<'a>(
    condition_map: &mut BTreeMap<ExactDefinitionRef, &'a str>,
    exact_ref: ExactDefinitionRef,
    fingerprint: &'a str,
) -> Result<(), RegistryLoadError> {
    if let Some(existing) = condition_map.insert(exact_ref, fingerprint) {
        let (kind, detail) = if existing == fingerprint {
            (
                RegistryLoadErrorKind::DuplicateIdentity,
                "project condition identity is duplicated",
            )
        } else {
            (
                RegistryLoadErrorKind::ConflictingIdentity,
                "project condition identity has conflicting fingerprints",
            )
        };
        return Err(RegistryLoadError::new(kind, detail));
    }
    Ok(())
}

impl ArtifactInstanceRegistry {
    pub fn resolve(
        values: &[Value],
        kinds: &ArtifactKindRegistry,
        conditions: &[&ProjectConditionDefinition],
    ) -> Result<Self, RegistryLoadError> {
        let authored = decode_authored_descriptors(values)?;
        let mut condition_map = BTreeMap::new();
        for condition in conditions {
            insert_condition_fingerprint(
                &mut condition_map,
                condition.exact_ref().clone(),
                condition.definition_fingerprint().as_str(),
            )?;
        }
        let mut instances = BTreeMap::new();
        let mut paths = BTreeSet::new();
        let mut constitutional = 0usize;
        for (index, source) in authored.iter().enumerate() {
            if source.schema_id != "handbook.artifact-instance-descriptor" {
                return Err(RegistryLoadError::at(
                    RegistryLoadErrorKind::UnsupportedRecord,
                    format!("artifact_instances/{index}/schema_id"),
                    "descriptor schema id is unsupported",
                ));
            }
            if source.schema_version != "1.0" {
                return Err(RegistryLoadError::at(
                    RegistryLoadErrorKind::UnsupportedRecord,
                    format!("artifact_instances/{index}/schema_version"),
                    "descriptor schema version is unsupported",
                ));
            }
            let is_frozen_charter_1_1 = source.id == "project_authority"
                && source.kind_ref == "handbook.artifact-kind.project-authority@1.1.0";
            if is_frozen_charter_1_1 {
                let exact = source.lifecycle_policy_ref.as_deref()
                    == Some("handbook.lifecycle.constitutional-review-lock@1.0.0")
                    && source.intake_definition_ref.as_deref()
                        == Some("handbook.intake.charter@1.0.0")
                    && source.renderer_definition_refs
                        == ["handbook.renderer.charter-review-markdown@1.0.0"]
                    && source.projection_definition_refs.is_empty()
                    && source.validation_overlay_refs.is_empty();
                if !exact {
                    return Err(RegistryLoadError::at(
                        RegistryLoadErrorKind::UnsupportedDependency,
                        format!("artifact_instances/{index}"),
                        "Project Authority 1.1 descriptor differs from its frozen dependency closure",
                    ));
                }
            } else {
                for (selected, field) in [
                    (
                        source.lifecycle_policy_ref.is_some(),
                        "lifecycle_policy_ref",
                    ),
                    (
                        !source.renderer_definition_refs.is_empty(),
                        "renderer_definition_refs",
                    ),
                    (
                        !source.projection_definition_refs.is_empty(),
                        "projection_definition_refs",
                    ),
                    (
                        !source.validation_overlay_refs.is_empty(),
                        "validation_overlay_refs",
                    ),
                ] {
                    if selected {
                        return Err(RegistryLoadError::at(
                            RegistryLoadErrorKind::UnsupportedDependency,
                            format!("artifact_instances/{index}/{field}"),
                            "descriptor selects a later-owned dependency",
                        ));
                    }
                }
            }
            if !source.extensions.is_empty() {
                return Err(RegistryLoadError::at(
                    RegistryLoadErrorKind::UnsupportedRecord,
                    format!("artifact_instances/{index}/extensions"),
                    "descriptor extensions must be empty",
                ));
            }
            let id = SymbolicId::parse(&source.id).map_err(|_| invalid("descriptor id"))?;
            let kind_ref = ExactDefinitionRef::parse(&source.kind_ref)?;
            let kind = kinds.kind(&kind_ref).ok_or_else(|| {
                RegistryLoadError::new(
                    RegistryLoadErrorKind::UnsupportedDependency,
                    "descriptor kind is absent",
                )
            })?;
            if source.label.is_empty() || source.label.contains('\0') {
                return Err(invalid("descriptor label"));
            }
            validate_path(&source.canonical_path)?;
            if !paths.insert(source.canonical_path.clone()) {
                return Err(RegistryLoadError::new(
                    RegistryLoadErrorKind::DuplicateIdentity,
                    "descriptor canonical path is duplicated",
                ));
            }
            if let Some(role) = &source.role_ref {
                if !kind.supported_role_refs().iter().any(|r| r == role) {
                    return Err(RegistryLoadError::new(
                        RegistryLoadErrorKind::UnsupportedDependency,
                        "descriptor role is unsupported by kind",
                    ));
                }
            }
            let mut caps = BTreeSet::new();
            let mut capability_refs = Vec::new();
            for value in &source.capability_refs {
                let cap = SymbolicId::parse(value).map_err(|_| invalid("descriptor capability"))?;
                if !caps.insert(cap.clone()) || !kind.semantic_capabilities().contains_key(&cap) {
                    return Err(RegistryLoadError::new(
                        RegistryLoadErrorKind::UnsupportedDependency,
                        "descriptor capability is duplicate or unsupported",
                    ));
                }
                if cap.as_str() == "constitutional_root" {
                    constitutional += 1;
                    if source.requiredness.mode != RequirednessMode::Always {
                        return Err(RegistryLoadError::at(
                            RegistryLoadErrorKind::InvalidConstitutionalRoot,
                            format!("artifact_instances/{index}/requiredness"),
                            "constitutional root must always be required",
                        ));
                    }
                }
                capability_refs.push(cap);
            }
            let condition_ref = match (source.requiredness.mode, &source.requiredness.condition_ref)
            {
                (RequirednessMode::Conditional, Some(r)) => {
                    let r = ExactDefinitionRef::parse(r)?;
                    if !condition_map.contains_key(&r) {
                        return Err(RegistryLoadError::at(
                            RegistryLoadErrorKind::InvalidRequiredness,
                            format!("artifact_instances/{index}/requiredness/condition_ref"),
                            "descriptor condition is absent",
                        ));
                    }
                    Some(r)
                }
                (RequirednessMode::Always | RequirednessMode::Optional, None) => None,
                _ => {
                    return Err(RegistryLoadError::at(
                        RegistryLoadErrorKind::InvalidRequiredness,
                        format!("artifact_instances/{index}/requiredness"),
                        "descriptor requiredness/condition combination is invalid",
                    ))
                }
            };
            let descriptor = ArtifactInstanceDescriptor {
                id: id.clone(),
                kind_ref,
                role_ref: source.role_ref.clone(),
                capability_refs,
                label: source.label.clone(),
                canonical_path: source.canonical_path.clone(),
                requiredness: ArtifactRequiredness {
                    mode: source.requiredness.mode,
                    condition_ref,
                },
                dependencies: source
                    .depends_on
                    .iter()
                    .enumerate()
                    .map(|(dependency_index, dependency)| {
                        let location =
                            format!("artifact_instances/{index}/depends_on/{dependency_index}");
                        Ok(ArtifactDependency {
                            target_kind: dependency.target_kind,
                            target_ref: SymbolicId::parse(&dependency.target_ref).map_err(
                                |_| {
                                    RegistryLoadError::at(
                                        RegistryLoadErrorKind::InvalidDependencyTarget,
                                        format!("{location}/target_ref"),
                                        "descriptor dependency target is invalid",
                                    )
                                },
                            )?,
                            target_contract_ref: dependency
                                .target_contract_ref
                                .as_deref()
                                .map(ExactDefinitionRef::parse)
                                .transpose()
                                .map_err(|_| {
                                    RegistryLoadError::at(
                                        RegistryLoadErrorKind::InvalidDependencyContract,
                                        format!("{location}/target_contract_ref"),
                                        "descriptor dependency contract ref is invalid",
                                    )
                                })?,
                            cardinality: dependency.cardinality,
                        })
                    })
                    .collect::<Result<Vec<_>, RegistryLoadError>>()?,
                lifecycle_policy_ref: source
                    .lifecycle_policy_ref
                    .as_deref()
                    .map(ExactDefinitionRef::parse)
                    .transpose()?,
                intake_definition_ref: source
                    .intake_definition_ref
                    .as_deref()
                    .map(ExactDefinitionRef::parse)
                    .transpose()?,
                renderer_definition_refs: source
                    .renderer_definition_refs
                    .iter()
                    .map(|reference| ExactDefinitionRef::parse(reference))
                    .collect::<Result<Vec<_>, _>>()?,
                projection_definition_refs: source
                    .projection_definition_refs
                    .iter()
                    .map(|reference| ExactDefinitionRef::parse(reference))
                    .collect::<Result<Vec<_>, _>>()?,
                validation_overlay_refs: source
                    .validation_overlay_refs
                    .iter()
                    .map(|reference| ExactDefinitionRef::parse(reference))
                    .collect::<Result<Vec<_>, _>>()?,
                extensions: BTreeMap::new(),
            };
            if instances.insert(id, descriptor).is_some() {
                return Err(RegistryLoadError::new(
                    RegistryLoadErrorKind::DuplicateIdentity,
                    "descriptor ID is duplicated",
                ));
            }
        }
        if constitutional != 1 {
            return Err(RegistryLoadError::at(
                RegistryLoadErrorKind::InvalidConstitutionalRoot,
                "artifact_instances",
                "resolved profile must contain exactly one constitutional root",
            ));
        }
        validate_dependencies(&authored, &instances, kinds)?;
        let authored_by_id = authored
            .iter()
            .map(|source| (source.id.as_str(), source))
            .collect::<BTreeMap<_, _>>();
        let members = instances
            .values()
            .map(|descriptor| DescriptorClosure {
                definition: authored_by_id[descriptor.id.as_str()],
                kind_fingerprint: kinds
                    .kind(&descriptor.kind_ref)
                    .unwrap()
                    .definition_fingerprint()
                    .as_str(),
                condition_fingerprint: descriptor
                    .requiredness
                    .condition_ref
                    .as_ref()
                    .and_then(|r| condition_map.get(r).copied()),
            })
            .collect::<Vec<_>>();
        let fingerprint = fingerprint_serializable(&members)?;
        Ok(Self {
            instances,
            fingerprint,
        })
    }
    pub fn ids(&self) -> Vec<&str> {
        self.instances.keys().map(SymbolicId::as_str).collect()
    }
    pub fn instance(&self, id: &SymbolicId) -> Option<&ArtifactInstanceDescriptor> {
        self.instances.get(id)
    }
    pub fn fingerprint(&self) -> &DefinitionFingerprint {
        &self.fingerprint
    }
}

pub(crate) fn validate_authored_descriptor_record_shapes(
    value: &Value,
) -> Result<(), RegistryLoadError> {
    let values = value.as_array().ok_or_else(|| {
        RegistryLoadError::new(
            RegistryLoadErrorKind::SyntaxError,
            "profile artifact_instances must be an array of closed descriptor records",
        )
    })?;
    decode_authored_descriptors(values).map(|_| ())
}

fn decode_authored_descriptors(
    values: &[Value],
) -> Result<Vec<AuthoredDescriptor>, RegistryLoadError> {
    values
        .iter()
        .enumerate()
        .map(|(index, value)| {
            validate_authored_error_categories(value, index)?;
            serde_json::from_value::<AuthoredDescriptor>(value.clone()).map_err(|error| {
                RegistryLoadError::new(
                    if error.to_string().contains("unknown field") {
                        RegistryLoadErrorKind::UnknownField
                    } else {
                        RegistryLoadErrorKind::SyntaxError
                    },
                    "artifact instance descriptor does not match its closed record",
                )
            })
        })
        .collect()
}

fn validate_authored_error_categories(
    value: &Value,
    descriptor_index: usize,
) -> Result<(), RegistryLoadError> {
    let descriptor = value.as_object().ok_or_else(|| {
        RegistryLoadError::new(
            RegistryLoadErrorKind::SyntaxError,
            "artifact instance descriptor must be an object",
        )
    })?;
    if ["role_ref", "lifecycle_policy_ref", "intake_definition_ref"]
        .iter()
        .any(|key| !descriptor.contains_key(*key))
    {
        return Err(RegistryLoadError::new(
            RegistryLoadErrorKind::SyntaxError,
            "artifact instance descriptor omits an explicitly nullable member",
        ));
    }
    let requiredness = value.get("requiredness").and_then(Value::as_object);
    if requiredness.is_some_and(|object| !object.contains_key("condition_ref")) {
        return Err(RegistryLoadError::new(
            RegistryLoadErrorKind::SyntaxError,
            "descriptor requiredness omits explicit condition_ref",
        ));
    }
    if requiredness
        .and_then(|object| object.get("mode"))
        .and_then(Value::as_str)
        .is_none_or(|mode| !matches!(mode, "always" | "conditional" | "optional"))
    {
        return Err(RegistryLoadError::at(
            RegistryLoadErrorKind::InvalidRequiredness,
            format!("artifact_instances/{descriptor_index}/requiredness/mode"),
            "descriptor requiredness mode is invalid",
        ));
    }
    if let Some(dependencies) = value.get("depends_on").and_then(Value::as_array) {
        for (dependency_index, dependency) in dependencies.iter().enumerate() {
            let location =
                || format!("artifact_instances/{descriptor_index}/depends_on/{dependency_index}");
            if dependency
                .as_object()
                .is_some_and(|object| !object.contains_key("target_contract_ref"))
            {
                return Err(RegistryLoadError::new(
                    RegistryLoadErrorKind::SyntaxError,
                    "descriptor dependency omits explicit target_contract_ref",
                ));
            }
            if dependency
                .get("target_kind")
                .and_then(Value::as_str)
                .is_none_or(|kind| !matches!(kind, "instance" | "capability"))
            {
                return Err(RegistryLoadError::at(
                    RegistryLoadErrorKind::InvalidDependencyNamespace,
                    format!("{}/target_kind", location()),
                    "descriptor dependency target namespace is invalid",
                ));
            }
            if dependency
                .get("cardinality")
                .and_then(Value::as_str)
                .is_none_or(|cardinality| !matches!(cardinality, "exactly_one" | "at_least_one"))
            {
                return Err(RegistryLoadError::at(
                    RegistryLoadErrorKind::InvalidDependencyCardinality,
                    format!("{}/cardinality", location()),
                    "descriptor dependency cardinality is invalid",
                ));
            }
        }
    }
    Ok(())
}
#[derive(Serialize)]
struct DescriptorClosure<'a> {
    definition: &'a AuthoredDescriptor,
    kind_fingerprint: &'a str,
    condition_fingerprint: Option<&'a str>,
}
fn validate_path(path: &str) -> Result<(), RegistryLoadError> {
    let normalized = NormalizedRepoRelativePath::parse(path);
    let valid = path.trim() == path
        && (1..=1024).contains(&path.len())
        && (1..=64).contains(&path.split('/').count())
        && !crate::instance_profile::has_uri_scheme_or_drive_prefix(path)
        && normalized
            .as_ref()
            .is_ok_and(|normalized| normalized.as_str() == path);
    if valid {
        Ok(())
    } else {
        Err(invalid("descriptor canonical path"))
    }
}
fn validate_dependencies(
    authored: &[AuthoredDescriptor],
    instances: &BTreeMap<SymbolicId, ArtifactInstanceDescriptor>,
    kinds: &ArtifactKindRegistry,
) -> Result<(), RegistryLoadError> {
    let mut graph = instances
        .keys()
        .cloned()
        .map(|id| (id, BTreeSet::new()))
        .collect::<BTreeMap<_, _>>();
    for (source_index, source) in authored.iter().enumerate() {
        let source_id = SymbolicId::parse(&source.id).map_err(|_| invalid("descriptor id"))?;
        let mut seen_dependencies = BTreeSet::new();
        for (dependency_index, dependency) in source.depends_on.iter().enumerate() {
            let location =
                format!("artifact_instances/{source_index}/depends_on/{dependency_index}");
            let target = SymbolicId::parse(&dependency.target_ref).map_err(|_| {
                RegistryLoadError::at(
                    RegistryLoadErrorKind::InvalidDependencyTarget,
                    format!("{location}/target_ref"),
                    "descriptor dependency target is invalid",
                )
            })?;
            let duplicate_key = (
                dependency.target_kind,
                target.clone(),
                dependency.target_contract_ref.clone(),
            );
            if !seen_dependencies.insert(duplicate_key) {
                return Err(RegistryLoadError::at(
                    RegistryLoadErrorKind::DuplicateIdentity,
                    location,
                    "descriptor dependency is duplicated",
                ));
            }
            match dependency.target_kind {
                DependencyTargetKind::Instance => {
                    if dependency.target_contract_ref.is_some() {
                        return Err(RegistryLoadError::at(
                            RegistryLoadErrorKind::InvalidDependencyContract,
                            format!("{location}/target_contract_ref"),
                            "instance dependency cannot carry a contract ref",
                        ));
                    }
                    if dependency.cardinality != DependencyCardinality::ExactlyOne {
                        return Err(RegistryLoadError::at(
                            RegistryLoadErrorKind::InvalidDependencyCardinality,
                            format!("{location}/cardinality"),
                            "instance dependency cardinality must be exactly_one",
                        ));
                    }
                    if !instances.contains_key(&target) {
                        return Err(RegistryLoadError::at(
                            RegistryLoadErrorKind::InvalidDependencyTarget,
                            format!("{location}/target_ref"),
                            "instance dependency target is absent",
                        ));
                    }
                    graph.get_mut(&source_id).unwrap().insert(target);
                }
                DependencyTargetKind::Capability => {
                    let contract = dependency.target_contract_ref.as_deref().ok_or_else(|| {
                        RegistryLoadError::at(
                            RegistryLoadErrorKind::InvalidDependencyContract,
                            format!("{location}/target_contract_ref"),
                            "capability dependency lacks contract",
                        )
                    })?;
                    let contract = ExactDefinitionRef::parse(contract).map_err(|_| {
                        RegistryLoadError::at(
                            RegistryLoadErrorKind::InvalidDependencyContract,
                            format!("{location}/target_contract_ref"),
                            "capability dependency contract ref is invalid",
                        )
                    })?;
                    let contract_definition = kinds
                        .semantic_capability_registry()
                        .capability(&contract)
                        .ok_or_else(|| {
                            RegistryLoadError::at(
                                RegistryLoadErrorKind::InvalidDependencyContract,
                                format!("{location}/target_contract_ref"),
                                "capability dependency contract is absent",
                            )
                        })?;
                    if contract_definition.capability_id() != &target {
                        return Err(RegistryLoadError::at(
                            RegistryLoadErrorKind::InvalidDependencyContract,
                            format!("{location}/target_contract_ref"),
                            "capability dependency contract targets another capability",
                        ));
                    }
                    let providers = instances
                        .values()
                        .filter(|i| {
                            i.capability_refs.contains(&target)
                                && kinds
                                    .kind(&i.kind_ref)
                                    .and_then(|k| k.semantic_capabilities().get(&target))
                                    .is_some_and(|c| c.contract_ref() == &contract)
                        })
                        .map(|instance| instance.id.clone())
                        .collect::<Vec<_>>();
                    if providers.is_empty()
                        || (dependency.cardinality == DependencyCardinality::ExactlyOne
                            && providers.len() != 1)
                    {
                        return Err(RegistryLoadError::at(
                            RegistryLoadErrorKind::InvalidDependencyProviderCount,
                            location,
                            "capability dependency provider cardinality is invalid",
                        ));
                    }
                    graph.get_mut(&source_id).unwrap().extend(providers);
                }
            }
        }
    }
    let mut indegree = graph
        .keys()
        .cloned()
        .map(|id| (id, 0usize))
        .collect::<BTreeMap<_, _>>();
    for targets in graph.values() {
        for target in targets {
            *indegree
                .get_mut(target)
                .expect("validated dependency target") += 1;
        }
    }
    let mut ready = indegree
        .iter()
        .filter_map(|(id, degree)| (*degree == 0).then_some(id.clone()))
        .collect::<BTreeSet<_>>();
    let mut visited = 0usize;
    while let Some(id) = ready.pop_first() {
        visited += 1;
        for target in &graph[&id] {
            let degree = indegree.get_mut(target).unwrap();
            *degree -= 1;
            if *degree == 0 {
                ready.insert(target.clone());
            }
        }
    }
    if visited != graph.len() {
        return Err(RegistryLoadError::at(
            RegistryLoadErrorKind::DependencyCycle,
            "artifact_instances/depends_on",
            "artifact instance dependency graph contains a cycle",
        ));
    }
    Ok(())
}
fn invalid(label: &str) -> RegistryLoadError {
    RegistryLoadError::new(
        RegistryLoadErrorKind::UnsupportedRecord,
        format!("invalid {label}"),
    )
}
pub fn shipped_root_artifact_instance_values() -> Vec<Value> {
    vec![
        json!({"schema_id":"handbook.artifact-instance-descriptor","schema_version":"1.0","id":"project_authority","kind_ref":"handbook.artifact-kind.project-authority@1.0.0","role_ref":"constitutional_authority","capability_refs":["constitutional_root"],"label":"Charter","canonical_path":".handbook/project/charter.yaml","requiredness":{"mode":"always","condition_ref":null},"depends_on":[],"lifecycle_policy_ref":null,"intake_definition_ref":null,"renderer_definition_refs":[],"projection_definition_refs":[],"validation_overlay_refs":[],"extensions":{}}),
        json!({"schema_id":"handbook.artifact-instance-descriptor","schema_version":"1.0","id":"project_context","kind_ref":"handbook.artifact-kind.project-context@1.0.0","role_ref":"project_context","capability_refs":[],"label":"Project Context","canonical_path":".handbook/project/context.yaml","requiredness":{"mode":"always","condition_ref":null},"depends_on":[],"lifecycle_policy_ref":null,"intake_definition_ref":null,"renderer_definition_refs":[],"projection_definition_refs":[],"validation_overlay_refs":[],"extensions":{}}),
        json!({"schema_id":"handbook.artifact-instance-descriptor","schema_version":"1.0","id":"environment_context","kind_ref":"handbook.artifact-kind.environment-context@1.0.0","role_ref":"environment_context","capability_refs":[],"label":"Environment Context","canonical_path":".handbook/project/environment.yaml","requiredness":{"mode":"conditional","condition_ref":"handbook.condition.project.managed-operational-surface@1.0.0"},"depends_on":[],"lifecycle_policy_ref":null,"intake_definition_ref":null,"renderer_definition_refs":[],"projection_definition_refs":[],"validation_overlay_refs":[],"extensions":{}}),
    ]
}

#[cfg(test)]
mod condition_identity_tests {
    use super::*;

    fn collision(first: &'static str, second: &'static str) -> RegistryLoadErrorKind {
        let exact_ref =
            ExactDefinitionRef::parse("handbook.condition.project.example@1.0.0").unwrap();
        let mut map = BTreeMap::new();
        insert_condition_fingerprint(&mut map, exact_ref.clone(), first).unwrap();
        insert_condition_fingerprint(&mut map, exact_ref, second)
            .unwrap_err()
            .kind()
    }

    #[test]
    fn condition_identity_collisions_distinguish_duplicates_and_conflicts_in_both_orders() {
        const FIRST: &str =
            "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
        const SECOND: &str =
            "sha256:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";

        assert_eq!(
            collision(FIRST, FIRST),
            RegistryLoadErrorKind::DuplicateIdentity
        );
        assert_eq!(
            collision(SECOND, SECOND),
            RegistryLoadErrorKind::DuplicateIdentity
        );
        assert_eq!(
            collision(FIRST, SECOND),
            RegistryLoadErrorKind::ConflictingIdentity
        );
        assert_eq!(
            collision(SECOND, FIRST),
            RegistryLoadErrorKind::ConflictingIdentity
        );
    }
}
