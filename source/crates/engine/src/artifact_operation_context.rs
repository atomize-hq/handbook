use crate::artifact_intake_registry::{
    ArtifactIntakeRegistry, ArtifactRegistrationErrorKindV1, ArtifactRegistrationErrorV1,
};
use crate::{DefinitionFingerprint, ExactDefinitionRef, ResolvedArtifactRegistry, SymbolicId};
use serde_json::{json, Value};
use std::collections::BTreeSet;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DefinitionBindingV1 {
    pub definition_ref: ExactDefinitionRef,
    pub definition_fingerprint: DefinitionFingerprint,
}

#[derive(Clone, Debug)]
pub struct ArtifactOperationContextInputV1 {
    pub repository_identity_fingerprint: DefinitionFingerprint,
    pub selection_fingerprint: DefinitionFingerprint,
    pub profile_ref: ExactDefinitionRef,
    pub resolved_profile_fingerprint: DefinitionFingerprint,
    pub kind_ref: ExactDefinitionRef,
    pub kind_fingerprint: DefinitionFingerprint,
    pub instance_id: SymbolicId,
    pub descriptor_fingerprint: DefinitionFingerprint,
    pub schema_ref: ExactDefinitionRef,
    pub schema_entry_fingerprint: DefinitionFingerprint,
    pub schema_document_fingerprint: DefinitionFingerprint,
    pub intake_definition_ref: Option<ExactDefinitionRef>,
    pub intake_definition_fingerprint: Option<DefinitionFingerprint>,
    pub resolved_definitions: Vec<DefinitionBindingV1>,
}

#[derive(Clone, Debug)]
pub struct ArtifactOperationAuthorityV1 {
    pub repository_identity_fingerprint: DefinitionFingerprint,
    pub selection_fingerprint: DefinitionFingerprint,
    pub selected_profile_definition_fingerprint: DefinitionFingerprint,
    pub descriptor_fingerprint: DefinitionFingerprint,
}

#[derive(Clone, Debug)]
pub struct ArtifactOperationContextV1 {
    input: ArtifactOperationContextInputV1,
    context_fingerprint: DefinitionFingerprint,
}

impl ArtifactOperationContextV1 {
    pub fn new(
        input: ArtifactOperationContextInputV1,
    ) -> Result<Self, ArtifactRegistrationErrorV1> {
        if input.intake_definition_ref.is_some() != input.intake_definition_fingerprint.is_some() {
            return Err(error(
                ArtifactRegistrationErrorKindV1::IncompatibleBinding,
                "intake_definition",
                "intake ref and fingerprint nullability must agree",
            ));
        }
        if input.resolved_definitions.is_empty() || input.resolved_definitions.len() > 64 {
            return Err(error(
                ArtifactRegistrationErrorKindV1::SourceLimitExceeded,
                "resolved_definitions",
                "resolved definition count is outside the closed limit",
            ));
        }
        let mut previous: Option<&ExactDefinitionRef> = None;
        let mut unique = BTreeSet::new();
        for binding in &input.resolved_definitions {
            if !unique.insert(binding.definition_ref.clone()) {
                return Err(error(
                    ArtifactRegistrationErrorKindV1::DuplicateIdentity,
                    "resolved_definitions",
                    "resolved definition ref is duplicated",
                ));
            }
            if previous.is_some_and(|value| value >= &binding.definition_ref) {
                return Err(error(
                    ArtifactRegistrationErrorKindV1::InvalidDefinitionOrder,
                    "resolved_definitions",
                    "resolved definitions are not in UTF-8 exact-ref order",
                ));
            }
            previous = Some(&binding.definition_ref);
        }
        let subject = context_subject(&input);
        let context_fingerprint =
            DefinitionFingerprint::from_json_value(&subject).map_err(|_| {
                error(
                    ArtifactRegistrationErrorKindV1::InvalidRecord,
                    "operation_context",
                    "operation context canonicalization failed",
                )
            })?;
        Ok(Self {
            input,
            context_fingerprint,
        })
    }

    pub fn resolve(
        registry: &ResolvedArtifactRegistry,
        intake_registry: &ArtifactIntakeRegistry,
        requested_kind_ref: &ExactDefinitionRef,
        requested_instance_id: &SymbolicId,
        authority: ArtifactOperationAuthorityV1,
    ) -> Result<Self, ArtifactRegistrationErrorV1> {
        let instance = registry.instance(requested_instance_id).ok_or_else(|| {
            error(
                ArtifactRegistrationErrorKindV1::MissingDependency,
                "instance_id",
                "requested artifact instance is not selected",
            )
        })?;
        if instance.kind_ref() != requested_kind_ref {
            return Err(error(
                ArtifactRegistrationErrorKindV1::IncompatibleBinding,
                "kind_ref",
                "requested kind does not match the selected instance",
            ));
        }
        let kind = registry.kind(requested_kind_ref).ok_or_else(|| {
            error(
                ArtifactRegistrationErrorKindV1::MissingDependency,
                "kind_ref",
                "requested artifact kind is not selected",
            )
        })?;

        let (intake_definition_ref, intake_definition_fingerprint) =
            match instance.intake_definition_ref() {
                Some(intake_ref) => {
                    let definition = intake_registry.definition(intake_ref).ok_or_else(|| {
                        error(
                            ArtifactRegistrationErrorKindV1::MissingDependency,
                            "intake_definition_ref",
                            "descriptor-selected intake definition is absent",
                        )
                    })?;
                    if definition.artifact_kind_ref() != requested_kind_ref
                        || definition.candidate_schema_ref() != kind.canonical_schema_ref()
                    {
                        return Err(error(
                            ArtifactRegistrationErrorKindV1::IncompatibleBinding,
                            "intake_definition_ref",
                            "intake kind/schema binding is incompatible with the selected artifact",
                        ));
                    }
                    (
                        Some(intake_ref.clone()),
                        Some(definition.definition_fingerprint().clone()),
                    )
                }
                None => (None, None),
            };

        let mut resolved_definitions = vec![
            DefinitionBindingV1 {
                definition_ref: kind.exact_ref().clone(),
                definition_fingerprint: kind.definition_fingerprint().clone(),
            },
            DefinitionBindingV1 {
                definition_ref: registry.profile_ref().clone(),
                definition_fingerprint: authority.selected_profile_definition_fingerprint,
            },
            DefinitionBindingV1 {
                definition_ref: kind.canonical_schema_ref().clone(),
                definition_fingerprint: kind.schema_entry_fingerprint().clone(),
            },
        ];
        if let (Some(reference), Some(fingerprint)) = (
            intake_definition_ref.as_ref(),
            intake_definition_fingerprint.as_ref(),
        ) {
            resolved_definitions.push(DefinitionBindingV1 {
                definition_ref: reference.clone(),
                definition_fingerprint: fingerprint.clone(),
            });
        }
        resolved_definitions.sort_by(|left, right| left.definition_ref.cmp(&right.definition_ref));

        Self::new(ArtifactOperationContextInputV1 {
            repository_identity_fingerprint: authority.repository_identity_fingerprint,
            selection_fingerprint: authority.selection_fingerprint,
            profile_ref: registry.profile_ref().clone(),
            resolved_profile_fingerprint: registry.profile_fingerprint().clone(),
            kind_ref: kind.exact_ref().clone(),
            kind_fingerprint: kind.definition_fingerprint().clone(),
            instance_id: requested_instance_id.clone(),
            descriptor_fingerprint: authority.descriptor_fingerprint,
            schema_ref: kind.canonical_schema_ref().clone(),
            schema_entry_fingerprint: kind.schema_entry_fingerprint().clone(),
            schema_document_fingerprint: kind.schema_document_fingerprint().clone(),
            intake_definition_ref,
            intake_definition_fingerprint,
            resolved_definitions,
        })
    }

    pub fn into_input(self) -> ArtifactOperationContextInputV1 {
        self.input
    }

    pub fn repository_identity_fingerprint(&self) -> &DefinitionFingerprint {
        &self.input.repository_identity_fingerprint
    }

    pub fn selection_fingerprint(&self) -> &DefinitionFingerprint {
        &self.input.selection_fingerprint
    }

    pub fn profile_ref(&self) -> &ExactDefinitionRef {
        &self.input.profile_ref
    }

    pub fn resolved_profile_fingerprint(&self) -> &DefinitionFingerprint {
        &self.input.resolved_profile_fingerprint
    }

    pub fn kind_ref(&self) -> &ExactDefinitionRef {
        &self.input.kind_ref
    }

    pub fn kind_fingerprint(&self) -> &DefinitionFingerprint {
        &self.input.kind_fingerprint
    }

    pub fn instance_id(&self) -> &SymbolicId {
        &self.input.instance_id
    }

    pub fn descriptor_fingerprint(&self) -> &DefinitionFingerprint {
        &self.input.descriptor_fingerprint
    }

    pub fn schema_ref(&self) -> &ExactDefinitionRef {
        &self.input.schema_ref
    }

    pub fn schema_entry_fingerprint(&self) -> &DefinitionFingerprint {
        &self.input.schema_entry_fingerprint
    }

    pub fn schema_document_fingerprint(&self) -> &DefinitionFingerprint {
        &self.input.schema_document_fingerprint
    }

    pub fn intake_definition_ref(&self) -> Option<&ExactDefinitionRef> {
        self.input.intake_definition_ref.as_ref()
    }

    pub fn intake_definition_fingerprint(&self) -> Option<&DefinitionFingerprint> {
        self.input.intake_definition_fingerprint.as_ref()
    }

    pub fn resolved_definitions(&self) -> &[DefinitionBindingV1] {
        &self.input.resolved_definitions
    }

    pub fn context_fingerprint(&self) -> &DefinitionFingerprint {
        &self.context_fingerprint
    }

    pub fn to_json_value(&self) -> Value {
        let mut value = context_subject(&self.input);
        value
            .as_object_mut()
            .expect("the operation context subject is always an object")
            .insert(
                "context_fingerprint".to_string(),
                Value::String(self.context_fingerprint.to_string()),
            );
        value
    }
}

fn context_subject(input: &ArtifactOperationContextInputV1) -> Value {
    json!({
        "schema_id": "handbook.artifact-operation-context",
        "schema_version": "1.0",
        "repository_identity_fingerprint": input.repository_identity_fingerprint.as_str(),
        "selection_fingerprint": input.selection_fingerprint.as_str(),
        "profile_ref": input.profile_ref.as_str(),
        "resolved_profile_fingerprint": input.resolved_profile_fingerprint.as_str(),
        "kind_ref": input.kind_ref.as_str(),
        "kind_fingerprint": input.kind_fingerprint.as_str(),
        "instance_id": input.instance_id.as_str(),
        "descriptor_fingerprint": input.descriptor_fingerprint.as_str(),
        "schema_ref": input.schema_ref.as_str(),
        "schema_entry_fingerprint": input.schema_entry_fingerprint.as_str(),
        "schema_document_fingerprint": input.schema_document_fingerprint.as_str(),
        "intake_definition_ref": input
            .intake_definition_ref
            .as_ref()
            .map(ExactDefinitionRef::as_str),
        "intake_definition_fingerprint": input
            .intake_definition_fingerprint
            .as_ref()
            .map(DefinitionFingerprint::as_str),
        "resolved_definitions": input
            .resolved_definitions
            .iter()
            .map(|binding| json!({
                "definition_ref": binding.definition_ref.as_str(),
                "definition_fingerprint": binding.definition_fingerprint.as_str(),
            }))
            .collect::<Vec<_>>(),
    })
}

fn error(
    kind: ArtifactRegistrationErrorKindV1,
    location: &'static str,
    detail: &'static str,
) -> ArtifactRegistrationErrorV1 {
    ArtifactRegistrationErrorV1::new_for_kernel(kind, location, detail)
}
