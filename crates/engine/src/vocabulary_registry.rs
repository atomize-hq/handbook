use crate::definition_identity::{
    fingerprint_serializable, parse_definition_yaml, DefinitionFingerprint, ExactDefinitionRef,
    RegistryLoadError, RegistryLoadErrorKind, SourceByteBudget,
};
use crate::stable_role_registry::read_trusted_repo_source;
use crate::{StableRoleCategory, StableRoleDefinition, StableRoleRegistry, SymbolicId};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Selection {
    #[serde(rename = "ref")]
    reference: String,
    fingerprint: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct VocabularyAbsorption {
    unit_id: String,
    absorbs: Vec<String>,
}

impl VocabularyAbsorption {
    pub fn unit_id(&self) -> &str {
        &self.unit_id
    }

    pub fn absorbs(&self) -> &[String] {
        &self.absorbs
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct AuthoredVocabulary {
    schema_id: String,
    schema_version: String,
    vocabulary_id: String,
    vocabulary_version: String,
    stable_role_registry: Selection,
    labels: BTreeMap<String, String>,
    aliases: BTreeMap<String, Vec<String>>,
    absorptions: Vec<VocabularyAbsorption>,
    extensions: BTreeMap<String, Value>,
    #[serde(skip_serializing)]
    vocabulary_fingerprint: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum VocabularyResolution {
    Unique(String),
    Ambiguous(Vec<String>),
    Unknown,
}

#[derive(Clone, Debug)]
pub struct VocabularyDefinition {
    exact_ref: ExactDefinitionRef,
    stable_role_registry: StableRoleRegistry,
    vocabulary_fingerprint: DefinitionFingerprint,
    labels: BTreeMap<String, String>,
    aliases: BTreeMap<String, Vec<String>>,
    lookup: BTreeMap<String, BTreeSet<String>>,
    absorptions: Vec<VocabularyAbsorption>,
}

impl VocabularyDefinition {
    pub fn exact_ref(&self) -> &ExactDefinitionRef {
        &self.exact_ref
    }

    pub fn vocabulary_fingerprint(&self) -> &DefinitionFingerprint {
        &self.vocabulary_fingerprint
    }

    pub fn stable_role_registry_ref(&self) -> &ExactDefinitionRef {
        self.stable_role_registry.exact_ref()
    }

    pub fn stable_role_registry_fingerprint(&self) -> &DefinitionFingerprint {
        self.stable_role_registry.fingerprint()
    }

    pub fn explicit_label(&self, role_id: &str) -> Option<&str> {
        self.labels.get(role_id).map(String::as_str)
    }

    pub fn display_label(&self, role_id: &str) -> Option<&str> {
        self.explicit_label(role_id).or_else(|| {
            self.stable_role_registry
                .role(role_id)
                .map(StableRoleDefinition::canonical_display_label)
        })
    }

    pub fn resolve_typed_role(&self, role_id: &str) -> Option<&StableRoleDefinition> {
        self.stable_role_registry.role(role_id)
    }

    pub fn resolve_untyped(&self, input: &str) -> VocabularyResolution {
        let normalized = normalize_lookup_text(input);
        match self.lookup.get(&normalized) {
            Some(candidates) if candidates.len() == 1 => VocabularyResolution::Unique(
                candidates
                    .iter()
                    .next()
                    .expect("one candidate was checked")
                    .clone(),
            ),
            Some(candidates) => {
                VocabularyResolution::Ambiguous(candidates.iter().cloned().collect())
            }
            None => VocabularyResolution::Unknown,
        }
    }

    pub fn absorptions(&self) -> &[VocabularyAbsorption] {
        &self.absorptions
    }

    pub fn is_empty(&self) -> bool {
        self.labels.is_empty() && self.aliases.is_empty() && self.absorptions.is_empty()
    }

    pub fn load(repo: impl AsRef<Path>, path: &str) -> Result<Self, RegistryLoadError> {
        let mut budget = SourceByteBudget::default();
        let (_, bytes) = read_trusted_repo_source(repo.as_ref(), path, &mut budget)?;
        Self::load_bytes(&bytes)
    }

    pub(crate) fn load_bytes(bytes: &[u8]) -> Result<Self, RegistryLoadError> {
        let (registry_ref, supplied_fingerprint) =
            admitted_vocabulary_stable_role_selection(bytes)?;
        let registry = StableRoleRegistry::load_builtin(&registry_ref)?;
        if registry.fingerprint() != &supplied_fingerprint {
            return Err(RegistryLoadError::new(
                RegistryLoadErrorKind::StableRoleRegistryMismatch,
                "vocabulary stable-role fingerprint does not match its selected registry",
            ));
        }
        Self::load_bytes_with_registry(bytes, &registry)
    }

    pub(crate) fn load_bytes_with_registry(
        bytes: &[u8],
        registry: &StableRoleRegistry,
    ) -> Result<Self, RegistryLoadError> {
        decode_authored(bytes)?.resolve(registry)
    }
}

pub(crate) fn admitted_vocabulary_exact_ref(
    bytes: &[u8],
) -> Result<ExactDefinitionRef, RegistryLoadError> {
    let authored = decode_authored(bytes)?;
    validate_record_version(&authored)?;
    ExactDefinitionRef::new(&authored.vocabulary_id, &authored.vocabulary_version)
}

pub(crate) fn admitted_vocabulary_stable_role_selection(
    bytes: &[u8],
) -> Result<(ExactDefinitionRef, DefinitionFingerprint), RegistryLoadError> {
    let authored = decode_authored(bytes)?;
    validate_record_version(&authored)?;
    Ok((
        ExactDefinitionRef::parse(&authored.stable_role_registry.reference)?,
        DefinitionFingerprint::parse(&authored.stable_role_registry.fingerprint)?,
    ))
}

impl AuthoredVocabulary {
    fn resolve(
        mut self,
        stable_role_registry: &StableRoleRegistry,
    ) -> Result<VocabularyDefinition, RegistryLoadError> {
        validate_record_version(&self)?;
        if !self.extensions.is_empty() {
            return Err(RegistryLoadError::new(
                RegistryLoadErrorKind::UnsupportedRecord,
                "vocabulary extensions require a declared extension namespace",
            ));
        }

        let exact_ref = ExactDefinitionRef::new(&self.vocabulary_id, &self.vocabulary_version)?;
        let registry_ref = ExactDefinitionRef::parse(&self.stable_role_registry.reference)?;
        let registry_fingerprint =
            DefinitionFingerprint::parse(&self.stable_role_registry.fingerprint)?;
        if &registry_ref != stable_role_registry.exact_ref()
            || &registry_fingerprint != stable_role_registry.fingerprint()
        {
            return Err(RegistryLoadError::new(
                RegistryLoadErrorKind::StableRoleRegistryMismatch,
                "vocabulary stable-role selection does not match the resolving registry",
            ));
        }

        validate_labels(&self.labels, stable_role_registry)?;
        self.aliases = normalize_aliases(self.aliases, stable_role_registry)?;
        canonicalize_absorptions(&mut self.absorptions, stable_role_registry)?;

        let supplied = DefinitionFingerprint::parse(&self.vocabulary_fingerprint)?;
        let computed = fingerprint_serializable(&self)?;
        if supplied != computed {
            return Err(RegistryLoadError::new(
                RegistryLoadErrorKind::FingerprintMismatch,
                "vocabulary fingerprint does not match its canonical semantic fields",
            ));
        }

        let lookup = build_lookup(stable_role_registry, &self.labels, &self.aliases);
        Ok(VocabularyDefinition {
            exact_ref,
            stable_role_registry: stable_role_registry.clone(),
            vocabulary_fingerprint: computed,
            labels: self.labels,
            aliases: self.aliases,
            lookup,
            absorptions: self.absorptions,
        })
    }
}

fn decode_authored(bytes: &[u8]) -> Result<AuthoredVocabulary, RegistryLoadError> {
    let value = parse_definition_yaml(bytes)?;
    serde_json::from_value(value).map_err(|error| {
        RegistryLoadError::new(
            if error.to_string().contains("unknown field") {
                RegistryLoadErrorKind::UnknownField
            } else {
                RegistryLoadErrorKind::SyntaxError
            },
            "vocabulary does not match its closed typed record",
        )
    })
}

fn validate_record_version(authored: &AuthoredVocabulary) -> Result<(), RegistryLoadError> {
    if authored.schema_id != "handbook.vocabulary-profile" || authored.schema_version != "1.0" {
        return Err(RegistryLoadError::new(
            RegistryLoadErrorKind::UnsupportedRecord,
            "unsupported vocabulary record",
        ));
    }
    Ok(())
}

fn validate_labels(
    labels: &BTreeMap<String, String>,
    registry: &StableRoleRegistry,
) -> Result<(), RegistryLoadError> {
    for (role_id, label) in labels {
        require_role(registry, role_id)?;
        validate_display_text(label, "vocabulary label")?;
    }
    Ok(())
}

fn normalize_aliases(
    aliases: BTreeMap<String, Vec<String>>,
    registry: &StableRoleRegistry,
) -> Result<BTreeMap<String, Vec<String>>, RegistryLoadError> {
    let mut normalized = BTreeMap::new();
    for (role_id, role_aliases) in aliases {
        require_role(registry, &role_id)?;
        let mut tokens = BTreeSet::new();
        for alias in role_aliases {
            validate_display_text(&alias, "vocabulary alias")?;
            tokens.insert(normalize_lookup_text(&alias));
        }
        normalized.insert(role_id, tokens.into_iter().collect());
    }
    Ok(normalized)
}

fn canonicalize_absorptions(
    absorptions: &mut [VocabularyAbsorption],
    registry: &StableRoleRegistry,
) -> Result<(), RegistryLoadError> {
    let mut unit_ids = BTreeSet::new();
    let mut owners = BTreeMap::new();
    for absorption in absorptions.iter_mut() {
        SymbolicId::parse(&absorption.unit_id).map_err(|_| {
            RegistryLoadError::new(
                RegistryLoadErrorKind::UnknownStableRole,
                "absorption unit ID violates the lowercase symbolic ID grammar",
            )
        })?;
        if !unit_ids.insert(absorption.unit_id.clone()) {
            return Err(RegistryLoadError::new(
                RegistryLoadErrorKind::DuplicateIdentity,
                "absorption unit ID is duplicated",
            ));
        }
        if registry
            .role(&absorption.unit_id)
            .is_some_and(|role| role.category() != StableRoleCategory::Workflow)
        {
            return Err(RegistryLoadError::new(
                RegistryLoadErrorKind::InvalidStableRoleCategory,
                "a registered absorption unit ID must be a workflow role",
            ));
        }
        if absorption.absorbs.is_empty() {
            return Err(RegistryLoadError::new(
                RegistryLoadErrorKind::InvalidDependencyCardinality,
                "an absorption must name at least one workflow role",
            ));
        }

        let mut targets = BTreeSet::new();
        for role_id in &absorption.absorbs {
            let role = require_role(registry, role_id)?;
            if role_id == "constitutional_authority"
                || role.category() != StableRoleCategory::Workflow
            {
                return Err(RegistryLoadError::new(
                    RegistryLoadErrorKind::InvalidStableRoleCategory,
                    "absorption targets must be admissible workflow roles",
                ));
            }
            if !targets.insert(role_id.clone()) {
                return Err(RegistryLoadError::new(
                    RegistryLoadErrorKind::DuplicateIdentity,
                    "an absorption target is duplicated",
                ));
            }
            if owners
                .insert(role_id.clone(), absorption.unit_id.clone())
                .is_some()
            {
                return Err(RegistryLoadError::new(
                    RegistryLoadErrorKind::DuplicateIdentity,
                    "a workflow role is owned by more than one absorption",
                ));
            }
        }
        absorption.absorbs = targets.into_iter().collect();
    }
    absorptions.sort_by(|left, right| left.unit_id.cmp(&right.unit_id));
    validate_absorption_graph(absorptions)
}

fn validate_absorption_graph(
    absorptions: &[VocabularyAbsorption],
) -> Result<(), RegistryLoadError> {
    let mut nodes = BTreeSet::new();
    let mut indegrees = BTreeMap::<String, usize>::new();
    let mut edges = BTreeMap::<String, BTreeSet<String>>::new();
    for absorption in absorptions {
        nodes.insert(absorption.unit_id.clone());
        indegrees.entry(absorption.unit_id.clone()).or_default();
        for target in &absorption.absorbs {
            nodes.insert(target.clone());
            *indegrees.entry(target.clone()).or_default() += 1;
            edges
                .entry(absorption.unit_id.clone())
                .or_default()
                .insert(target.clone());
        }
    }

    let mut available = nodes
        .iter()
        .filter(|node| indegrees.get(*node).copied().unwrap_or(0) == 0)
        .cloned()
        .collect::<BTreeSet<_>>();
    let mut visited = 0usize;
    while let Some(node) = available.pop_first() {
        visited += 1;
        if let Some(targets) = edges.get(&node) {
            for target in targets {
                let indegree = indegrees
                    .get_mut(target)
                    .expect("every absorption target is a graph node");
                *indegree -= 1;
                if *indegree == 0 {
                    available.insert(target.clone());
                }
            }
        }
    }
    if visited != nodes.len() {
        return Err(RegistryLoadError::new(
            RegistryLoadErrorKind::DependencyCycle,
            "vocabulary absorption graph contains a directed cycle",
        ));
    }
    Ok(())
}

fn build_lookup(
    registry: &StableRoleRegistry,
    labels: &BTreeMap<String, String>,
    aliases: &BTreeMap<String, Vec<String>>,
) -> BTreeMap<String, BTreeSet<String>> {
    let mut lookup = BTreeMap::<String, BTreeSet<String>>::new();
    for role in registry.roles() {
        let label = labels
            .get(role.role_id())
            .map(String::as_str)
            .unwrap_or_else(|| role.canonical_display_label());
        lookup
            .entry(normalize_lookup_text(label))
            .or_default()
            .insert(role.role_id().to_string());
    }
    for (role_id, role_aliases) in aliases {
        for alias in role_aliases {
            lookup
                .entry(alias.clone())
                .or_default()
                .insert(role_id.clone());
        }
    }
    lookup
}

fn require_role<'a>(
    registry: &'a StableRoleRegistry,
    role_id: &str,
) -> Result<&'a StableRoleDefinition, RegistryLoadError> {
    registry.role(role_id).ok_or_else(|| {
        RegistryLoadError::new(
            RegistryLoadErrorKind::UnknownStableRole,
            "vocabulary references an unregistered stable role",
        )
    })
}

fn validate_display_text(value: &str, field: &str) -> Result<(), RegistryLoadError> {
    if value.is_empty() || value.trim() != value || value.chars().any(char::is_control) {
        return Err(RegistryLoadError::new(
            RegistryLoadErrorKind::UnknownStableRole,
            format!("{field} must be non-empty, trimmed, and printable"),
        ));
    }
    Ok(())
}

fn normalize_lookup_text(value: &str) -> String {
    let mut normalized = String::new();
    let mut in_whitespace = false;
    for character in value.chars() {
        if character.is_whitespace() {
            if !in_whitespace {
                normalized.push(' ');
            }
            in_whitespace = true;
        } else {
            normalized.extend(character.to_lowercase());
            in_whitespace = false;
        }
    }
    normalized
}
