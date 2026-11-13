use crate::artifact_intake_registry::{ArtifactIntakeDefinitionV1, ArtifactIntakeRegistry};
use crate::artifact_operation_context::ArtifactOperationContextV1;
use crate::canonical_yaml::parse_canonical_yaml;
use crate::{
    ArtifactRegistryValidationError, DefinitionFingerprint, ExactDefinitionRef,
    ResolvedArtifactRegistry, StructuralValidationError, SymbolicId,
};
use serde_json::Value;
use std::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ArtifactOperationIdV1 {
    KindList,
    InstanceList,
    Read,
    Validate,
    IntakeDefinitionRead,
    CoverageEvaluate,
    IntakeRecordAppend,
    CandidateValidate,
    CandidateAppend,
    ApprovalAppend,
    CandidatePromote,
}

impl ArtifactOperationIdV1 {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::KindList => "artifact.kind.list",
            Self::InstanceList => "artifact.instance.list",
            Self::Read => "artifact.read",
            Self::Validate => "artifact.validate",
            Self::IntakeDefinitionRead => "intake.definition.read",
            Self::CoverageEvaluate => "intake.coverage.evaluate",
            Self::IntakeRecordAppend => "intake.record.append",
            Self::CandidateValidate => "artifact.candidate.validate",
            Self::CandidateAppend => "artifact.candidate.append",
            Self::ApprovalAppend => "artifact.approval.append",
            Self::CandidatePromote => "artifact.candidate.promote",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtifactKindListItemV1 {
    pub kind_ref: ExactDefinitionRef,
    pub definition_fingerprint: DefinitionFingerprint,
    pub schema_ref: ExactDefinitionRef,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtifactInstanceListItemV1 {
    pub instance_id: SymbolicId,
    pub kind_ref: ExactDefinitionRef,
    pub canonical_path: String,
    pub intake_definition_ref: Option<ExactDefinitionRef>,
}

#[derive(Clone, Debug)]
pub struct ArtifactReadResultV1 {
    pub operation_id: ArtifactOperationIdV1,
    pub operation_context_fingerprint: DefinitionFingerprint,
    pub kind_ref: ExactDefinitionRef,
    pub instance_id: SymbolicId,
    pub canonical_path: String,
    pub artifact_fingerprint: DefinitionFingerprint,
    pub content: Value,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ArtifactValidationLayerStatusV1 {
    Pass,
    NotApplicable,
}

#[derive(Clone, Debug)]
pub struct ArtifactValidationResultV1 {
    pub operation_id: ArtifactOperationIdV1,
    pub operation_context_fingerprint: DefinitionFingerprint,
    pub artifact_fingerprint: DefinitionFingerprint,
    pub content: Value,
    pub structural_errors: Vec<StructuralValidationError>,
    pub semantic_status: ArtifactValidationLayerStatusV1,
    pub intake_status: ArtifactValidationLayerStatusV1,
    pub approval_status: ArtifactValidationLayerStatusV1,
    pub external_evidence_status: ArtifactValidationLayerStatusV1,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ArtifactOperationErrorKindV1 {
    TargetMismatch,
    CanonicalSyntax,
    Structural,
    NotApplicable,
    StaleOperationContext,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtifactOperationErrorV1 {
    kind: ArtifactOperationErrorKindV1,
    detail: &'static str,
    structural_errors: Vec<StructuralValidationError>,
}

impl ArtifactOperationErrorV1 {
    pub fn kind(&self) -> ArtifactOperationErrorKindV1 {
        self.kind
    }

    pub fn detail(&self) -> &'static str {
        self.detail
    }

    pub fn structural_errors(&self) -> &[StructuralValidationError] {
        &self.structural_errors
    }
}

impl fmt::Display for ArtifactOperationErrorV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.detail)
    }
}

impl std::error::Error for ArtifactOperationErrorV1 {}

pub struct ArtifactOperationServiceV1<'a> {
    registry: &'a ResolvedArtifactRegistry,
    intake_registry: &'a ArtifactIntakeRegistry,
    context: &'a ArtifactOperationContextV1,
}

impl<'a> ArtifactOperationServiceV1<'a> {
    pub fn new(
        registry: &'a ResolvedArtifactRegistry,
        intake_registry: &'a ArtifactIntakeRegistry,
        context: &'a ArtifactOperationContextV1,
    ) -> Result<Self, ArtifactOperationErrorV1> {
        let instance = registry.instance(context.instance_id()).ok_or_else(|| {
            operation_error(
                ArtifactOperationErrorKindV1::StaleOperationContext,
                "operation context instance is no longer selected",
            )
        })?;
        let kind = registry.kind(context.kind_ref()).ok_or_else(|| {
            operation_error(
                ArtifactOperationErrorKindV1::StaleOperationContext,
                "operation context kind is no longer selected",
            )
        })?;
        if instance.kind_ref() != context.kind_ref()
            || registry.profile_ref() != context.profile_ref()
            || registry.profile_fingerprint() != context.resolved_profile_fingerprint()
            || kind.definition_fingerprint() != context.kind_fingerprint()
            || kind.canonical_schema_ref() != context.schema_ref()
            || kind.schema_entry_fingerprint() != context.schema_entry_fingerprint()
            || kind.schema_document_fingerprint() != context.schema_document_fingerprint()
            || instance.intake_definition_ref() != context.intake_definition_ref()
        {
            return Err(operation_error(
                ArtifactOperationErrorKindV1::StaleOperationContext,
                "operation context no longer matches the resolved artifact registry",
            ));
        }
        match (
            context.intake_definition_ref(),
            context.intake_definition_fingerprint(),
        ) {
            (Some(reference), Some(fingerprint)) => {
                if intake_registry
                    .definition(reference)
                    .is_none_or(|definition| definition.definition_fingerprint() != fingerprint)
                {
                    return Err(operation_error(
                        ArtifactOperationErrorKindV1::StaleOperationContext,
                        "operation context no longer matches the intake registry",
                    ));
                }
            }
            (None, None) => {}
            _ => {
                return Err(operation_error(
                    ArtifactOperationErrorKindV1::StaleOperationContext,
                    "operation context intake binding is incomplete",
                ))
            }
        }
        Ok(Self {
            registry,
            intake_registry,
            context,
        })
    }

    pub fn read_from_bytes(
        &self,
        bytes: &[u8],
    ) -> Result<ArtifactReadResultV1, ArtifactOperationErrorV1> {
        let content = parse_canonical_yaml(bytes).map_err(|_| {
            operation_error(
                ArtifactOperationErrorKindV1::CanonicalSyntax,
                "canonical artifact is not duplicate-free single-document YAML",
            )
        })?;
        let instance = self
            .registry
            .instance(self.context.instance_id())
            .expect("service construction closed the selected instance");
        Ok(ArtifactReadResultV1 {
            operation_id: ArtifactOperationIdV1::Read,
            operation_context_fingerprint: self.context.context_fingerprint().clone(),
            kind_ref: self.context.kind_ref().clone(),
            instance_id: self.context.instance_id().clone(),
            canonical_path: instance.canonical_path().to_string(),
            artifact_fingerprint: DefinitionFingerprint::from_bytes(bytes),
            content,
        })
    }

    pub fn validate_from_bytes(
        &self,
        bytes: &[u8],
    ) -> Result<ArtifactValidationResultV1, ArtifactOperationErrorV1> {
        let read = self.read_from_bytes(bytes)?;
        match self
            .registry
            .validate_json(self.context.instance_id(), &read.content)
        {
            Ok(()) => Ok(ArtifactValidationResultV1 {
                operation_id: ArtifactOperationIdV1::Validate,
                operation_context_fingerprint: read.operation_context_fingerprint,
                artifact_fingerprint: read.artifact_fingerprint,
                content: read.content,
                structural_errors: Vec::new(),
                semantic_status: ArtifactValidationLayerStatusV1::NotApplicable,
                intake_status: if self.context.intake_definition_ref().is_some() {
                    ArtifactValidationLayerStatusV1::Pass
                } else {
                    ArtifactValidationLayerStatusV1::NotApplicable
                },
                approval_status: ArtifactValidationLayerStatusV1::NotApplicable,
                external_evidence_status: ArtifactValidationLayerStatusV1::NotApplicable,
            }),
            Err(ArtifactRegistryValidationError::Structural(errors)) => {
                Err(ArtifactOperationErrorV1 {
                    kind: ArtifactOperationErrorKindV1::Structural,
                    detail: "canonical artifact failed the selected exact structural schema",
                    structural_errors: errors,
                })
            }
            Err(ArtifactRegistryValidationError::UnknownArtifactInstance) => Err(operation_error(
                ArtifactOperationErrorKindV1::StaleOperationContext,
                "operation context instance is no longer selected",
            )),
        }
    }

    pub fn intake_definition(
        &self,
    ) -> Result<&'a ArtifactIntakeDefinitionV1, ArtifactOperationErrorV1> {
        let reference = self.context.intake_definition_ref().ok_or_else(|| {
            operation_error(
                ArtifactOperationErrorKindV1::NotApplicable,
                "selected artifact instance has no intake definition",
            )
        })?;
        self.intake_registry.definition(reference).ok_or_else(|| {
            operation_error(
                ArtifactOperationErrorKindV1::StaleOperationContext,
                "descriptor-selected intake definition is no longer registered",
            )
        })
    }
}

pub fn list_artifact_kinds(registry: &ResolvedArtifactRegistry) -> Vec<ArtifactKindListItemV1> {
    registry
        .kind_refs()
        .into_iter()
        .filter_map(|reference| registry.kind(reference))
        .map(|kind| ArtifactKindListItemV1 {
            kind_ref: kind.exact_ref().clone(),
            definition_fingerprint: kind.definition_fingerprint().clone(),
            schema_ref: kind.canonical_schema_ref().clone(),
        })
        .collect()
}

pub fn list_artifact_instances(
    registry: &ResolvedArtifactRegistry,
) -> Vec<ArtifactInstanceListItemV1> {
    registry
        .instance_ids()
        .into_iter()
        .filter_map(|id| registry.instance(id))
        .map(|instance| ArtifactInstanceListItemV1 {
            instance_id: instance.id().clone(),
            kind_ref: instance.kind_ref().clone(),
            canonical_path: instance.canonical_path().to_string(),
            intake_definition_ref: instance.intake_definition_ref().cloned(),
        })
        .collect()
}

fn operation_error(
    kind: ArtifactOperationErrorKindV1,
    detail: &'static str,
) -> ArtifactOperationErrorV1 {
    ArtifactOperationErrorV1 {
        kind,
        detail,
        structural_errors: Vec::new(),
    }
}
