use crate::definition_identity::{
    fingerprint_serializable, parse_definition_yaml, DefinitionFingerprint, ExactDefinitionRef,
    RegistryLoadError, RegistryLoadErrorKind, SourceByteBudget,
};
use crate::stable_role_registry::read_trusted_repo_source;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct AuthoredCondition {
    schema_id: String,
    schema_version: String,
    condition_id: String,
    condition_version: String,
    outcomes: Vec<String>,
    accepted_input_classes: Vec<String>,
    freshness_requirement: String,
    minimum_independent_current_bases: u8,
    self_reference_exclusions: Vec<String>,
    outcome_precedence: Vec<String>,
    effects: Vec<String>,
    extensions: BTreeMap<String, Value>,
    #[serde(skip_serializing)]
    definition_fingerprint: String,
}
#[derive(Clone, Debug)]
pub struct ProjectConditionDefinition {
    exact_ref: ExactDefinitionRef,
    definition_fingerprint: DefinitionFingerprint,
}

#[derive(Clone, Debug, Default)]
pub struct ProjectConditionRegistry {
    definitions: BTreeMap<ExactDefinitionRef, ProjectConditionDefinition>,
}

impl ProjectConditionRegistry {
    pub fn load(repo: impl AsRef<Path>, paths: &[String]) -> Result<Self, RegistryLoadError> {
        let mut budget = SourceByteBudget::default();
        let mut definitions = BTreeMap::new();
        for path in paths {
            let (_, bytes) = read_trusted_repo_source(repo.as_ref(), path, &mut budget)?;
            let definition = ProjectConditionDefinition::load_bytes(&bytes)?;
            Self::insert(&mut definitions, definition)?;
        }
        Ok(Self { definitions })
    }

    pub(crate) fn load_admitted(
        sources: &[(&ExactDefinitionRef, &[u8])],
    ) -> Result<Self, RegistryLoadError> {
        let mut definitions = BTreeMap::new();
        for (declared_ref, bytes) in sources {
            let definition = ProjectConditionDefinition::load_bytes(bytes)?;
            if definition.exact_ref() != *declared_ref {
                return Err(RegistryLoadError::new(
                    RegistryLoadErrorKind::ConflictingIdentity,
                    "project condition producer derived exact ref does not match its typed source binding",
                ));
            }
            Self::insert(&mut definitions, definition)?;
        }
        Ok(Self { definitions })
    }

    fn insert(
        definitions: &mut BTreeMap<ExactDefinitionRef, ProjectConditionDefinition>,
        definition: ProjectConditionDefinition,
    ) -> Result<(), RegistryLoadError> {
        if definitions
            .insert(definition.exact_ref().clone(), definition)
            .is_some()
        {
            return Err(RegistryLoadError::new(
                RegistryLoadErrorKind::DuplicateIdentity,
                "project condition identity is duplicated",
            ));
        }
        Ok(())
    }

    pub fn definition(
        &self,
        exact_ref: &ExactDefinitionRef,
    ) -> Option<&ProjectConditionDefinition> {
        self.definitions.get(exact_ref)
    }

    pub fn refs(&self) -> BTreeSet<ExactDefinitionRef> {
        self.definitions.keys().cloned().collect()
    }

    pub(crate) fn values(&self) -> impl Iterator<Item = &ProjectConditionDefinition> {
        self.definitions.values()
    }
}

impl ProjectConditionDefinition {
    pub fn exact_ref(&self) -> &ExactDefinitionRef {
        &self.exact_ref
    }
    pub fn definition_fingerprint(&self) -> &DefinitionFingerprint {
        &self.definition_fingerprint
    }
    pub fn load(repo: impl AsRef<Path>, path: &str) -> Result<Self, RegistryLoadError> {
        let mut budget = SourceByteBudget::default();
        let (_, bytes) = read_trusted_repo_source(repo.as_ref(), path, &mut budget)?;
        Self::load_bytes(&bytes)
    }
    pub(crate) fn load_bytes(bytes: &[u8]) -> Result<Self, RegistryLoadError> {
        let value = parse_definition_yaml(bytes)?;
        let authored: AuthoredCondition = serde_json::from_value(value).map_err(|e| {
            RegistryLoadError::new(
                if e.to_string().contains("unknown field") {
                    RegistryLoadErrorKind::UnknownField
                } else {
                    RegistryLoadErrorKind::SyntaxError
                },
                "project condition does not match its closed record",
            )
        })?;
        authored.resolve()
    }
}
pub(crate) fn admitted_project_condition_exact_ref(
    bytes: &[u8],
) -> Result<ExactDefinitionRef, RegistryLoadError> {
    let value = parse_definition_yaml(bytes)?;
    let authored: AuthoredCondition = serde_json::from_value(value).map_err(|error| {
        RegistryLoadError::new(
            if error.to_string().contains("unknown field") {
                RegistryLoadErrorKind::UnknownField
            } else {
                RegistryLoadErrorKind::SyntaxError
            },
            "project condition does not match its closed record",
        )
    })?;
    if authored.schema_id != "handbook.project-condition-definition"
        || authored.schema_version != "1.0"
    {
        return Err(RegistryLoadError::new(
            RegistryLoadErrorKind::UnsupportedRecord,
            "unsupported project condition record",
        ));
    }
    ExactDefinitionRef::new(&authored.condition_id, &authored.condition_version)
}
impl AuthoredCondition {
    fn resolve(self) -> Result<ProjectConditionDefinition, RegistryLoadError> {
        let literal = self.schema_id == "handbook.project-condition-definition"
            && self.schema_version == "1.0"
            && self.outcomes == ["true", "false", "unknown", "unresolved", "stale", "refused"]
            && self.accepted_input_classes == ["authoritative_fact_ref", "admitted_evidence_ref"]
            && self.freshness_requirement == "explicit_current_basis_required"
            && self.minimum_independent_current_bases == 1
            && self.self_reference_exclusions == ["environment_context"]
            && self.outcome_precedence
                == [
                    "refused_on_contradiction_or_disallowed_input",
                    "unresolved_on_missing_definition_or_required_input",
                    "stale_on_expired_basis",
                    "unknown_on_insufficient_proof",
                    "false_on_current_affirmative_no_responsibility",
                    "true_on_current_affirmative_qualifying_responsibility",
                ]
            && self.effects
                == [
                    "metadata_only",
                    "no_boolean_coercion",
                    "no_create",
                    "no_scaffold",
                    "no_delete",
                ]
            && self.extensions.is_empty();
        if !literal {
            return Err(RegistryLoadError::new(
                RegistryLoadErrorKind::UnsupportedRecord,
                "project condition differs from the exact managed-operational-surface definition",
            ));
        }
        let exact_ref = ExactDefinitionRef::new(&self.condition_id, &self.condition_version)?;
        if exact_ref.as_str() != "handbook.condition.project.managed-operational-surface@1.0.0" {
            return Err(RegistryLoadError::new(
                RegistryLoadErrorKind::UnsupportedRecord,
                "unsupported project condition identity",
            ));
        }
        let supplied = DefinitionFingerprint::parse(&self.definition_fingerprint)?;
        let computed = fingerprint_serializable(&self)?;
        if supplied != computed {
            return Err(RegistryLoadError::new(
                RegistryLoadErrorKind::FingerprintMismatch,
                "project condition fingerprint mismatch",
            ));
        }
        Ok(ProjectConditionDefinition {
            exact_ref,
            definition_fingerprint: computed,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CONDITION_BYTES: &[u8] = include_bytes!(
        "../definitions/project-conditions/handbook.condition.project.managed-operational-surface/1.0.0.yaml"
    );

    #[test]
    fn admitted_registry_refuses_conflicting_typed_bindings_in_both_source_orders() {
        let exact = ExactDefinitionRef::parse(
            "handbook.condition.project.managed-operational-surface@1.0.0",
        )
        .unwrap();
        let conflicting =
            ExactDefinitionRef::parse("handbook.condition.project.other@1.0.0").unwrap();

        for sources in [
            vec![(&exact, CONDITION_BYTES), (&conflicting, CONDITION_BYTES)],
            vec![(&conflicting, CONDITION_BYTES), (&exact, CONDITION_BYTES)],
        ] {
            let error = ProjectConditionRegistry::load_admitted(&sources).unwrap_err();
            assert_eq!(error.kind(), RegistryLoadErrorKind::ConflictingIdentity);
        }
    }
}
