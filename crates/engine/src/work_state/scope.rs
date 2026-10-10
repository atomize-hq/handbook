use super::*;

pub const PLANNING_OUTPUT_COMPOSITION_V1: &str =
    "handbook.work-composition.planning-output-leaf@1.0.0";
pub const REQUIREMENTS_INPUT_COMPOSITION_V1: &str =
    "handbook.work-composition.requirements-input-leaf@1.0.0";
pub const WORK_EXPLICIT_SCOPE_V1: &str = "handbook.work-registration-scope@1.0.0";

/// A definition declaration, never a result, source path or qualification claim.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkDefinitionFamilyV1 {
    IntrinsicV1,
    CompleteLocalV2,
    FullLocalV3,
}
impl WorkDefinitionFamilyV1 {
    pub fn schema_ref(self) -> &'static str {
        match self {
            Self::IntrinsicV1 => "handbook.contract-definition@1.0.0",
            Self::CompleteLocalV2 => "handbook.contract-definition.local@2.0.0",
            Self::FullLocalV3 => "handbook.contract-definition.local@3.0.0",
        }
    }
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WorkRequirementsDefinitionV1 {
    pub owner: String,
    pub family: WorkDefinitionFamilyV1,
    pub schema_ref: String,
    pub reference: String,
    /// Full owner definition fingerprint, including the V2/V3 dependency closure.
    pub fingerprint: String,
}
impl WorkRequirementsDefinitionV1 {
    pub fn validate(&self) -> Result<(), LocalAuthorityRefusalV1> {
        let (id, version) = self
            .reference
            .split_once('@')
            .ok_or_else(|| codec("exact reference"))?;
        let valid_segment = |s: &str| {
            !s.is_empty()
                && s.as_bytes()[0].is_ascii_lowercase()
                && s.as_bytes()
                    .last()
                    .is_some_and(|b| b.is_ascii_lowercase() || b.is_ascii_digit())
                && s.bytes()
                    .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
        };
        let version_valid = version.len() <= 255
            && semver::Version::parse(version).is_ok_and(|v| v.to_string() == version);
        if self.owner != "handbook-contracts"
            || self.schema_ref != self.family.schema_ref()
            || id.len() > 255
            || !id.contains('.')
            || !id.split('.').all(valid_segment)
            || !version_valid
            || self.fingerprint.len() != 71
            || !self.fingerprint.starts_with("sha256:")
            || !self.fingerprint.as_bytes()[7..]
                .iter()
                .all(|b| b.is_ascii_digit() || matches!(b, b'a'..=b'f'))
        {
            return Err(refuse(
                "work_requirements_definition",
                "exact supported Contracts family/schema/reference/full fingerprint required",
            ));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "role", rename_all = "snake_case", deny_unknown_fields)]
pub enum WorkRoleValueV1 {
    PlanningOutput {
        question: String,
    },
    RequirementsInput {
        definition: WorkRequirementsDefinitionV1,
    },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "role", rename_all = "snake_case", deny_unknown_fields)]
pub enum WorkRolePermissionV1 {
    Ordinary {},
    PlanningOutput {
        question_max_bytes: u32,
    },
    RequirementsInput {
        definitions: Vec<WorkRequirementsDefinitionV1>,
    },
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WorkCompositionClauseV1 {
    pub descriptor: WorkDescriptorV1,
    pub label_max_bytes: u32,
    pub work_question_max_bytes: u32,
    pub allow_omitted_display_level: bool,
    pub display_levels: Vec<String>,
    pub role_permission: WorkRolePermissionV1,
}
impl WorkCompositionClauseV1 {
    pub(super) fn validate(&self) -> Result<(), LocalAuthorityRefusalV1> {
        self.descriptor.validate()?;
        maximum(self.label_max_bytes)?;
        maximum(self.work_question_max_bytes)?;
        if !self.allow_omitted_display_level && self.display_levels.is_empty() {
            return Err(refuse(
                "work_display_scope",
                "explicit omission permission or finite tokens required",
            ));
        }
        for (i, level) in self.display_levels.iter().enumerate() {
            token(level)?;
            if self.display_levels[..i].contains(level) {
                return Err(codec("duplicate display token"));
            }
        }
        match &self.role_permission {
            WorkRolePermissionV1::Ordinary {}
                if self.descriptor == WorkDescriptorV1::ordinary_leaf() => {}
            WorkRolePermissionV1::PlanningOutput { question_max_bytes }
                if self.descriptor == WorkDescriptorV1::planning_output() =>
            {
                maximum(*question_max_bytes)?
            }
            WorkRolePermissionV1::RequirementsInput { definitions }
                if self.descriptor == WorkDescriptorV1::requirements_input() =>
            {
                if definitions.is_empty() {
                    return Err(codec("finite definitions required"));
                }
                for (i, definition) in definitions.iter().enumerate() {
                    definition.validate()?;
                    if definitions[..i].contains(definition) {
                        return Err(codec("duplicate definition"));
                    }
                }
            }
            _ => {
                return Err(refuse(
                    "work_role_scope",
                    "descriptor and exact role permission differ",
                ))
            }
        }
        Ok(())
    }
    pub(super) fn permits(&self, input: &WorkRegistrationInputV1) -> bool {
        let Some(composition) = input.composition.explicit_composition() else {
            return false;
        };
        self.descriptor == composition.descriptor
            && input.label.len() <= self.label_max_bytes as usize
            && input.question.len() <= self.work_question_max_bytes as usize
            && match &input.display_level {
                None => self.allow_omitted_display_level,
                Some(v) => self.display_levels.contains(v),
            }
            && match (&self.role_permission, &composition.role) {
                (WorkRolePermissionV1::Ordinary {}, None) => true,
                (
                    WorkRolePermissionV1::PlanningOutput { question_max_bytes },
                    Some(WorkRoleValueV1::PlanningOutput { question }),
                ) => question.len() <= *question_max_bytes as usize,
                (
                    WorkRolePermissionV1::RequirementsInput { definitions },
                    Some(WorkRoleValueV1::RequirementsInput { definition }),
                ) => definitions.contains(definition),
                _ => false,
            }
    }
}
fn maximum(v: u32) -> Result<(), LocalAuthorityRefusalV1> {
    if v == 0 || v > 4096 {
        Err(refuse(
            "work_scope_bound",
            "explicit positive maximum within 4096 bytes required",
        ))
    } else {
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WorkExplicitScopeV1 {
    pub schema_ref: String,
    pub clauses: Vec<WorkCompositionClauseV1>,
}
/// Wire dispatch preserves the original singleton's exact approved bytes. The
/// retained variant is only historical decoding; new preparation requires Explicit.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub enum WorkRegistrationScopeV1 {
    Explicit(WorkExplicitScopeV1),
    RetainedOrdinary(WorkCompositionV1),
}
impl WorkRegistrationScopeV1 {
    pub fn explicit(clauses: Vec<WorkCompositionClauseV1>) -> Self {
        Self::Explicit(WorkExplicitScopeV1 {
            schema_ref: WORK_EXPLICIT_SCOPE_V1.into(),
            clauses,
        })
    }
    pub(crate) fn validate(&self) -> Result<(), LocalAuthorityRefusalV1> {
        match self {
            Self::RetainedOrdinary(v) if v == &WorkCompositionV1::ordinary_leaf() => Ok(()),
            Self::Explicit(v) => {
                if v.schema_ref != WORK_EXPLICIT_SCOPE_V1
                    || v.clauses.is_empty()
                    || v.clauses.len() > 3
                {
                    return Err(codec("closed nonempty scope"));
                }
                for (i, clause) in v.clauses.iter().enumerate() {
                    clause.validate()?;
                    if v.clauses[..i]
                        .iter()
                        .any(|p| p.descriptor == clause.descriptor)
                    {
                        return Err(codec("duplicate composition"));
                    }
                }
                encode(self)?;
                Ok(())
            }
            _ => Err(codec("retained singleton")),
        }
    }
    pub(crate) fn permits(
        &self,
        input: &WorkRegistrationInputV1,
    ) -> Result<(), LocalAuthorityRefusalV1> {
        self.validate()?;
        let allowed = match self {
            Self::RetainedOrdinary(v) => {
                input.composition.explicit_composition() == Some(v) && input.display_level.is_none()
            }
            Self::Explicit(v) => v.clauses.iter().any(|c| c.permits(input)),
        };
        if allowed {
            Ok(())
        } else {
            Err(refuse(
                "work_scope",
                "composition/token/text/definition exceeds exact approved scope",
            ))
        }
    }
    pub(crate) fn equivalent(&self, other: &Self) -> bool {
        let clauses = |scope: &Self| match scope {
            Self::Explicit(v) => v.clauses.clone(),
            Self::RetainedOrdinary(_) => vec![WorkCompositionClauseV1 {
                descriptor: WorkDescriptorV1::ordinary_leaf(),
                label_max_bytes: 4096,
                work_question_max_bytes: 4096,
                allow_omitted_display_level: true,
                display_levels: vec![],
                role_permission: WorkRolePermissionV1::Ordinary {},
            }],
        };
        // Scope is exact approved content; list order is retained as part of it.
        clauses(self) == clauses(other)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkAuthorityChangeV1 {
    Initial,
    ScopeChange,
    EstablishFromDeferred,
}
/// Exact predecessor and operation are chosen explicitly by the caller. A
/// transition never infers expansion, restoration, or a newer authority.
pub enum WorkTermsTransitionV1<'a> {
    Initial,
    ScopeChange(&'a WorkGrantLocatorV1),
    EstablishFromDeferred(&'a WorkGrantLocatorV1),
}
