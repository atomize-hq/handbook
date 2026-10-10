use super::*;

/// New portable inputs are closed tagged choices. Historical explicit JSON has
/// one separate deterministic schema-dispatch path, preserving original bytes.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorkCompositionInputV1 {
    Explicit(WorkCompositionV1),
    Kind {
        binding: WorkKindBindingV1,
        role: Option<WorkRoleValueV1>,
    },
    #[doc(hidden)]
    RetainedExplicit(WorkCompositionV1),
}
#[derive(Serialize, Deserialize)]
#[serde(tag = "form", rename_all = "snake_case", deny_unknown_fields)]
enum Submitted {
    Explicit {
        descriptor: WorkDescriptorV1,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        role: Option<WorkRoleValueV1>,
    },
    Kind {
        binding: WorkKindBindingV1,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        role: Option<WorkRoleValueV1>,
    },
}
impl Serialize for WorkCompositionInputV1 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::RetainedExplicit(value) => value.serialize(serializer),
            Self::Explicit(value) => {
                if value.kind.is_some() {
                    return Err(serde::ser::Error::custom(
                        "explicit descriptor cannot override a kind",
                    ));
                }
                Submitted::Explicit {
                    descriptor: value.descriptor.clone(),
                    role: value.role.clone(),
                }
                .serialize(serializer)
            }
            Self::Kind { binding, role } => Submitted::Kind {
                binding: binding.clone(),
                role: role.clone(),
            }
            .serialize(serializer),
        }
    }
}
impl<'de> Deserialize<'de> for WorkCompositionInputV1 {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = serde_json::Value::deserialize(deserializer)?;
        if value.as_object().is_some_and(|o| o.contains_key("form")) {
            Ok(
                match serde_json::from_value(value).map_err(serde::de::Error::custom)? {
                    Submitted::Explicit { descriptor, role } => Self::Explicit(WorkCompositionV1 {
                        descriptor,
                        kind: None,
                        role,
                    }),
                    Submitted::Kind { binding, role } => Self::Kind { binding, role },
                },
            )
        } else {
            serde_json::from_value(value)
                .map(Self::RetainedExplicit)
                .map_err(serde::de::Error::custom)
        }
    }
}
impl From<WorkCompositionV1> for WorkCompositionInputV1 {
    fn from(value: WorkCompositionV1) -> Self {
        Self::Explicit(value)
    }
}
impl WorkCompositionInputV1 {
    pub fn explicit(value: WorkCompositionV1) -> Self {
        Self::Explicit(value)
    }
    pub fn kind(binding: WorkKindBindingV1, role: Option<WorkRoleValueV1>) -> Self {
        Self::Kind { binding, role }
    }
    pub fn explicit_composition(&self) -> Option<&WorkCompositionV1> {
        match self {
            Self::Explicit(v) | Self::RetainedExplicit(v) => Some(v),
            Self::Kind { .. } => None,
        }
    }
    pub fn explicit_mut(&mut self) -> Option<&mut WorkCompositionV1> {
        match self {
            Self::Explicit(v) | Self::RetainedExplicit(v) => Some(v),
            Self::Kind { .. } => None,
        }
    }
    pub fn role(&self) -> Option<&WorkRoleValueV1> {
        match self {
            Self::Explicit(v) | Self::RetainedExplicit(v) => v.role.as_ref(),
            Self::Kind { role, .. } => role.as_ref(),
        }
    }
    pub fn submitted_kind(&self) -> Option<&WorkKindBindingV1> {
        match self {
            Self::Kind { binding, .. } => Some(binding),
            _ => None,
        }
    }
    pub(crate) fn retained(&self) -> bool {
        matches!(self, Self::RetainedExplicit(_))
    }
    pub(crate) fn validate(&self) -> Result<(), LocalAuthorityRefusalV1> {
        match self {
            Self::Explicit(v) | Self::RetainedExplicit(v) => v.validate(),
            Self::Kind { binding, role } => {
                binding.validate()?;
                match role {
                    None => Ok(()),
                    Some(WorkRoleValueV1::PlanningOutput { question }) => text(question),
                    Some(WorkRoleValueV1::RequirementsInput { definition }) => {
                        definition.validate()
                    }
                }
            }
        }
    }
}
