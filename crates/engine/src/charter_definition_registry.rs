use crate::definition_identity::{
    parse_definition_yaml, parse_schema_json, DefinitionFingerprint, ExactDefinitionRef,
    RegistryLoadError, RegistryLoadErrorKind,
};
use crate::instance_profile::SymbolicId;
use crate::profile_builtins;
use crate::profile_decision::ResolvedProfileDecisions;
use crate::profile_selection::ResolvedInstanceProfile;
use serde_json::Value;
use std::collections::BTreeMap;

const DEFINITION_VECTOR_BYTES: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/specs/handbook-contract-membrane/slices/HCM-2.2/contracts/definition-closure-vectors-v1.0.json"
));

const CHARTER_DEFINITION_REFS: [&str; 8] = [
    "handbook.approval.constitutional-candidate@1.0.0",
    "handbook.waiver.constitutional-intake@1.0.0",
    "handbook.intake-trigger.production-posture-changed@1.0.0",
    "handbook.intake-trigger.trust-boundary-changed@1.0.0",
    "handbook.lifecycle-trigger.charter-amendment-proposed@1.0.0",
    "handbook.renderer.charter-review-markdown@1.0.0",
    "handbook.lifecycle.constitutional-review-lock@1.0.0",
    "handbook.intake.charter@1.0.0",
];

#[derive(Clone, Debug)]
pub struct CharterDefinitionRecord {
    exact_ref: ExactDefinitionRef,
    definition_fingerprint: DefinitionFingerprint,
    package_path: &'static str,
    definition: Value,
}

impl CharterDefinitionRecord {
    pub fn exact_ref(&self) -> &ExactDefinitionRef {
        &self.exact_ref
    }

    pub fn definition_fingerprint(&self) -> &DefinitionFingerprint {
        &self.definition_fingerprint
    }

    pub fn package_path(&self) -> &str {
        self.package_path
    }

    pub fn definition(&self) -> &Value {
        &self.definition
    }
}

#[derive(Clone, Debug)]
pub struct CharterDefinitionRegistry {
    records: BTreeMap<ExactDefinitionRef, CharterDefinitionRecord>,
}

impl CharterDefinitionRegistry {
    pub fn refs(&self) -> Vec<&ExactDefinitionRef> {
        self.records.keys().collect()
    }

    pub fn record(&self, reference: &ExactDefinitionRef) -> Option<&CharterDefinitionRecord> {
        self.records.get(reference)
    }

    pub fn validate_selected_profile(
        &self,
        profile: &ResolvedInstanceProfile,
    ) -> Result<(), RegistryLoadError> {
        if profile.exact_ref().as_str() != "handbook.profile.shipped-root@1.1.0" {
            return Err(RegistryLoadError::new(
                RegistryLoadErrorKind::UnsupportedDependency,
                "Charter definition closure requires the shipped profile 1.1 selection",
            ));
        }
        let charter_id = SymbolicId::parse("project_authority").map_err(|_| {
            RegistryLoadError::new(
                RegistryLoadErrorKind::InvalidExactDefinitionRef,
                "selected Charter instance ID is invalid",
            )
        })?;
        let charter = profile
            .artifact_instances()
            .instance(&charter_id)
            .ok_or_else(|| {
                RegistryLoadError::new(
                    RegistryLoadErrorKind::UnsupportedDependency,
                    "selected Charter instance is absent",
                )
            })?;
        let exact = charter.kind_ref().as_str() == "handbook.artifact-kind.project-authority@1.1.0"
            && charter.intake_definition_ref().is_some_and(|reference| {
                reference.as_str() == "handbook.intake.charter@1.0.0"
                    && self.records.contains_key(reference)
            })
            && charter.lifecycle_policy_ref().is_some_and(|reference| {
                reference.as_str() == "handbook.lifecycle.constitutional-review-lock@1.0.0"
                    && self.records.contains_key(reference)
            })
            && charter.renderer_definition_refs().len() == 1
            && charter.renderer_definition_refs()[0].as_str()
                == "handbook.renderer.charter-review-markdown@1.0.0"
            && self
                .records
                .contains_key(&charter.renderer_definition_refs()[0]);
        if exact {
            Ok(())
        } else {
            Err(RegistryLoadError::new(
                RegistryLoadErrorKind::UnsupportedDependency,
                "selected Charter instance does not bind the exact subordinate definition closure",
            ))
        }
    }

    pub fn validate_selected_decisions(
        &self,
        decisions: &ResolvedProfileDecisions,
    ) -> Result<(), RegistryLoadError> {
        let profile_tuple_is_exact = matches!(
            (
                decisions.profile_ref().as_str(),
                decisions.profile_definition_fingerprint().as_str(),
            ),
            (
                "handbook.profile.shipped-root@1.1.0",
                "sha256:6a7b41befa77b999b9ee20f513636051726a8401a81bf2f369501e8f3dd4fa74",
            ) | (
                "handbook.profile.shipped-root@1.2.0",
                "sha256:40c5fdb8a6ea42cf0f5f2c5cac8306ec7ad3a238c341653947f85abc93d72c40",
            )
        );
        if !profile_tuple_is_exact {
            return Err(RegistryLoadError::new(
                RegistryLoadErrorKind::UnsupportedDependency,
                "Charter definition closure requires an exact compatible shipped profile tuple",
            ));
        }
        let charter_id = SymbolicId::parse("project_authority").map_err(|_| {
            RegistryLoadError::new(
                RegistryLoadErrorKind::InvalidExactDefinitionRef,
                "selected Charter instance ID is invalid",
            )
        })?;
        let charter = decisions.registry().instance(&charter_id).ok_or_else(|| {
            RegistryLoadError::new(
                RegistryLoadErrorKind::UnsupportedDependency,
                "selected Charter decision instance is absent",
            )
        })?;
        let exact = charter.id().as_str() == "project_authority"
            && charter.kind_ref().as_str() == "handbook.artifact-kind.project-authority@1.1.0"
            && charter
                .role()
                .is_some_and(|role| role.role_id() == "constitutional_authority")
            && charter.capabilities().len() == 1
            && charter.capabilities()[0].capability_id().as_str() == "constitutional_root"
            && charter.capabilities()[0].contract_ref().as_str()
                == "handbook.capabilities.constitutional-root@1.0.0"
            && charter.label() == "Charter"
            && charter.canonical_path() == ".handbook/project/charter.yaml"
            && charter.requiredness_mode() == crate::artifact_instance::RequirednessMode::Always
            && charter.condition_ref().is_none()
            && charter.dependencies().is_empty()
            && charter.intake_definition_ref().is_some_and(|reference| {
                reference.as_str() == "handbook.intake.charter@1.0.0"
                    && self.records.contains_key(reference)
            })
            && charter.lifecycle_policy_ref().is_some_and(|reference| {
                reference.as_str() == "handbook.lifecycle.constitutional-review-lock@1.0.0"
                    && self.records.contains_key(reference)
            })
            && charter.renderer_definition_refs().len() == 1
            && charter.renderer_definition_refs()[0].as_str()
                == "handbook.renderer.charter-review-markdown@1.0.0"
            && self
                .records
                .contains_key(&charter.renderer_definition_refs()[0])
            && charter.projection_definition_refs().is_empty()
            && charter.validation_overlay_refs().is_empty()
            && charter.extensions().is_empty();
        if exact {
            Ok(())
        } else {
            Err(RegistryLoadError::new(
                RegistryLoadErrorKind::UnsupportedDependency,
                "selected Charter decisions do not bind the exact subordinate definition closure",
            ))
        }
    }
}

pub fn load_shipped_charter_definition_registry(
) -> Result<CharterDefinitionRegistry, RegistryLoadError> {
    let vectors = parse_schema_json(DEFINITION_VECTOR_BYTES)?;
    if vectors.get("schema_id").and_then(Value::as_str)
        != Some("hcm-2.2-definition-closure-vectors")
        || vectors.get("schema_version").and_then(Value::as_str) != Some("1.0")
    {
        return Err(RegistryLoadError::new(
            RegistryLoadErrorKind::UnsupportedRecord,
            "Charter definition vector identity is unsupported",
        ));
    }
    let definitions = vectors
        .get("definition_vectors")
        .and_then(Value::as_object)
        .ok_or_else(|| {
            RegistryLoadError::new(
                RegistryLoadErrorKind::SyntaxError,
                "Charter definition vectors are absent",
            )
        })?;
    let mut records = BTreeMap::new();
    for reference in CHARTER_DEFINITION_REFS {
        let exact_ref = ExactDefinitionRef::parse(reference)?;
        let vector = definitions.get(reference).ok_or_else(|| {
            RegistryLoadError::new(
                RegistryLoadErrorKind::UnsupportedDependency,
                "Charter definition vector is absent",
            )
        })?;
        let fingerprint_field = vector
            .get("own_fingerprint_field")
            .and_then(Value::as_str)
            .ok_or_else(|| {
                RegistryLoadError::new(
                    RegistryLoadErrorKind::SyntaxError,
                    "Charter definition fingerprint field is absent",
                )
            })?;
        let expected_fingerprint = vector
            .get("expected_fingerprint")
            .and_then(Value::as_str)
            .ok_or_else(|| {
                RegistryLoadError::new(
                    RegistryLoadErrorKind::SyntaxError,
                    "Charter definition expected fingerprint is absent",
                )
            })?;
        let preimage = vector.get("fingerprint_preimage").ok_or_else(|| {
            RegistryLoadError::new(
                RegistryLoadErrorKind::SyntaxError,
                "Charter definition fingerprint preimage is absent",
            )
        })?;
        let expected_definition = preimage.get("definition").ok_or_else(|| {
            RegistryLoadError::new(
                RegistryLoadErrorKind::SyntaxError,
                "Charter definition normalized value is absent",
            )
        })?;
        let source = profile_builtins::definition(&exact_ref).ok_or_else(|| {
            RegistryLoadError::new(
                RegistryLoadErrorKind::UnsupportedDependency,
                "Charter package definition source is absent",
            )
        })?;
        let authored_definition = parse_definition_yaml(source.bytes)?;
        let mut normalized_definition = authored_definition.clone();
        let supplied_fingerprint = normalized_definition
            .as_object_mut()
            .and_then(|object| object.remove(fingerprint_field))
            .and_then(|value| value.as_str().map(str::to_owned))
            .ok_or_else(|| {
                RegistryLoadError::new(
                    RegistryLoadErrorKind::SyntaxError,
                    "Charter definition own fingerprint is absent",
                )
            })?;
        if &normalized_definition != expected_definition {
            return Err(RegistryLoadError::new(
                RegistryLoadErrorKind::ConflictingIdentity,
                "Charter definition bytes differ from the exact normalized record",
            ));
        }
        let computed = DefinitionFingerprint::from_json_value(preimage)?;
        if supplied_fingerprint != expected_fingerprint || computed.as_str() != expected_fingerprint
        {
            return Err(RegistryLoadError::new(
                RegistryLoadErrorKind::FingerprintMismatch,
                "Charter definition fingerprint differs from its exact dependency closure",
            ));
        }
        records.insert(
            exact_ref.clone(),
            CharterDefinitionRecord {
                exact_ref,
                definition_fingerprint: computed,
                package_path: source.package_path,
                definition: authored_definition,
            },
        );
    }
    Ok(CharterDefinitionRegistry { records })
}
