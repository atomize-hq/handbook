use crate::artifact_kind_registry::ArtifactKindRegistry;
use crate::definition_identity::{
    fingerprint_serializable, DefinitionFingerprint, ExactDefinitionRef, RegistryLoadError,
    RegistryLoadErrorKind,
};
use crate::instance_profile::SymbolicId;
use crate::project_condition_registry::ProjectConditionDefinition;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RequirednessMode {
    Always,
    Conditional,
    Optional,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Requiredness {
    mode: RequirednessMode,
    condition_ref: Option<String>,
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
enum DependencyTargetKind {
    Instance,
    Capability,
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
enum DependencyCardinality {
    ExactlyOne,
    AtLeastOne,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Dependency {
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
    requiredness: Requiredness,
    depends_on: Vec<Dependency>,
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
    requiredness_mode: RequirednessMode,
    condition_ref: Option<ExactDefinitionRef>,
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
        self.requiredness_mode
    }
    pub fn condition_ref(&self) -> Option<&ExactDefinitionRef> {
        self.condition_ref.as_ref()
    }
}
#[derive(Clone, Debug)]
pub struct ArtifactInstanceRegistry {
    instances: BTreeMap<SymbolicId, ArtifactInstanceDescriptor>,
    fingerprint: DefinitionFingerprint,
}
impl ArtifactInstanceRegistry {
    pub fn resolve(
        values: &[Value],
        kinds: &ArtifactKindRegistry,
        conditions: &[&ProjectConditionDefinition],
    ) -> Result<Self, RegistryLoadError> {
        let mut authored = Vec::new();
        for value in values {
            authored.push(
                serde_json::from_value::<AuthoredDescriptor>(value.clone()).map_err(|e| {
                    RegistryLoadError::new(
                        if e.to_string().contains("unknown field") {
                            RegistryLoadErrorKind::UnknownField
                        } else {
                            RegistryLoadErrorKind::SyntaxError
                        },
                        "artifact instance descriptor does not match its closed record",
                    )
                })?,
            );
        }
        let condition_map = conditions
            .iter()
            .map(|c| (c.exact_ref().clone(), c.definition_fingerprint().as_str()))
            .collect::<BTreeMap<_, _>>();
        let mut instances = BTreeMap::new();
        let mut paths = BTreeSet::new();
        let mut constitutional = 0usize;
        for source in &authored {
            if source.schema_id != "handbook.artifact-instance-descriptor"
                || source.schema_version != "1.0"
                || source.lifecycle_policy_ref.is_some()
                || source.intake_definition_ref.is_some()
                || !source.renderer_definition_refs.is_empty()
                || !source.projection_definition_refs.is_empty()
                || !source.validation_overlay_refs.is_empty()
                || !source.extensions.is_empty()
            {
                return Err(RegistryLoadError::new(
                    RegistryLoadErrorKind::UnsupportedDependency,
                    "descriptor contains unsupported record or later-owned selection",
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
            if source.label.chars().count() == 0
                || source.label.chars().count() > 64
                || source.label.contains('\0')
            {
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
                        return Err(RegistryLoadError::new(
                            RegistryLoadErrorKind::UnsupportedDependency,
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
                        return Err(RegistryLoadError::new(
                            RegistryLoadErrorKind::UnsupportedDependency,
                            "descriptor condition is absent",
                        ));
                    }
                    Some(r)
                }
                (RequirednessMode::Always | RequirednessMode::Optional, None) => None,
                _ => {
                    return Err(RegistryLoadError::new(
                        RegistryLoadErrorKind::UnsupportedDependency,
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
                requiredness_mode: source.requiredness.mode,
                condition_ref,
            };
            if instances.insert(id, descriptor).is_some() {
                return Err(RegistryLoadError::new(
                    RegistryLoadErrorKind::DuplicateIdentity,
                    "descriptor ID is duplicated",
                ));
            }
        }
        if constitutional != 1 {
            return Err(RegistryLoadError::new(
                RegistryLoadErrorKind::UnsupportedDependency,
                "resolved profile must contain exactly one constitutional root",
            ));
        }
        validate_dependencies(&authored, &instances, kinds)?;
        let members = instances
            .values()
            .map(|d| DescriptorClosure {
                id: d.id.as_str(),
                kind_ref: d.kind_ref.as_str(),
                kind_fingerprint: kinds
                    .kind(&d.kind_ref)
                    .unwrap()
                    .definition_fingerprint()
                    .as_str(),
                role_ref: d.role_ref.as_deref(),
                capability_refs: d.capability_refs.iter().map(SymbolicId::as_str).collect(),
                label: &d.label,
                canonical_path: &d.canonical_path,
                requiredness_mode: d.requiredness_mode,
                condition_ref: d.condition_ref.as_ref().map(ExactDefinitionRef::as_str),
                condition_fingerprint: d
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
#[derive(Serialize)]
struct DescriptorClosure<'a> {
    id: &'a str,
    kind_ref: &'a str,
    kind_fingerprint: &'a str,
    role_ref: Option<&'a str>,
    capability_refs: Vec<&'a str>,
    label: &'a str,
    canonical_path: &'a str,
    requiredness_mode: RequirednessMode,
    condition_ref: Option<&'a str>,
    condition_fingerprint: Option<&'a str>,
}
fn validate_path(path: &str) -> Result<(), RegistryLoadError> {
    let valid = (1..=1024).contains(&path.len())
        && path.is_ascii()
        && !path.starts_with('/')
        && !path.ends_with('/')
        && !path.contains('\\')
        && !path.contains('\0')
        && path.split('/').count() <= 64
        && path
            .split('/')
            .all(|p| !p.is_empty() && p != "." && p != "..");
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
    for source in authored {
        for dependency in &source.depends_on {
            let target = SymbolicId::parse(&dependency.target_ref)
                .map_err(|_| invalid("dependency target"))?;
            match dependency.target_kind {
                DependencyTargetKind::Instance => {
                    if dependency.target_contract_ref.is_some()
                        || dependency.cardinality != DependencyCardinality::ExactlyOne
                        || !instances.contains_key(&target)
                    {
                        return Err(RegistryLoadError::new(
                            RegistryLoadErrorKind::UnsupportedDependency,
                            "instance dependency is invalid",
                        ));
                    }
                }
                DependencyTargetKind::Capability => {
                    let contract = dependency.target_contract_ref.as_deref().ok_or_else(|| {
                        RegistryLoadError::new(
                            RegistryLoadErrorKind::UnsupportedDependency,
                            "capability dependency lacks contract",
                        )
                    })?;
                    let contract = ExactDefinitionRef::parse(contract)?;
                    let providers = instances
                        .values()
                        .filter(|i| {
                            i.capability_refs.contains(&target)
                                && kinds
                                    .kind(&i.kind_ref)
                                    .and_then(|k| k.semantic_capabilities().get(&target))
                                    .is_some_and(|c| c.contract_ref() == &contract)
                        })
                        .count();
                    if providers == 0
                        || (dependency.cardinality == DependencyCardinality::ExactlyOne
                            && providers != 1)
                    {
                        return Err(RegistryLoadError::new(
                            RegistryLoadErrorKind::UnsupportedDependency,
                            "capability dependency provider cardinality is invalid",
                        ));
                    }
                }
            }
        }
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
