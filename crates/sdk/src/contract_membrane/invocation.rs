use super::serialization::classify_contract_error_message;
use super::{
    canonical_fingerprint, canonical_json_bytes, parse_canonical_json, sha256_fingerprint,
    BoundedString, BoundedVec, ContractError, ContractLimits, ContractValue, CorrelationRef,
    Diagnostic, ExactBinding, ExactRef, Fingerprint, NextAction, NextActionKind,
    OperationAdmissionRefusalV1, OperationDefinition, OperationResponseV1,
    OperationSchemaValidator, Problem, ProblemCategory, RecheckCondition, RequestId,
    RetryDirective, SafeU64, TerminalFields, TerminalStatus, WriteReceipt,
};
use crate::charter_product::{
    AdapterCausalDisposition, AdapterPublicEffect, AdapterPublicEffectDisposition,
    ApproverAdminLegacyFields,
};
use crate::pipeline_route_api::{
    PipelineResolveEffect, PipelineResolveExecution, PipelineResolveExecutionFailure,
    PipelineResolveOutcome, PipelineResolvedStageView, PipelineRouteBasisPersistenceRefusal,
    PipelineRouteRunView, PipelineRouteStageReason, PipelineRouteStageStatus,
    PipelineRouteStateView, PipelineSelectedDefinitionFailure, PipelineSelectorRefusal,
};
use crate::setup::{SetupExecutionEffects, SetupExecutionResult};
use crate::{
    ApproverOperation, ApproverOperationProjection, ApproverOperationResult, ArtifactApplicability,
    ArtifactInspectionReason, ArtifactInspectionStatus, AuthorOperationRefusal,
    AuthorOperationStatus, AuthorProjectContextRefusal, AuthorProjectContextRefusalKind,
    AuthorProjectContextResult, CharterOperation, CharterOperationResult,
    DoctorCharterDefinitionClosureStatus, DoctorCharterDefinitionRow, DoctorCharterLifecycleState,
    DoctorCharterNextAction, DoctorCharterRow, DoctorError, DoctorErrorKind, DoctorErrorReasonCode,
    DoctorProjectContextRow, DoctorReport, ProfileArtifactRow, ProfileCapabilityRow,
    ProfileConditionOutcome, ProfileConditionReason, ProfileConditionRow, ProfileRequirednessMode,
    ProjectContextCommandOutcome, RepositoryInvocationFailureView, RepositoryReadinessStatus,
    SetupArtifactAction, SetupArtifactActionKind, SetupError, SetupErrorCode, SetupErrorKind,
    SetupErrorReasonCode, SetupMode, SetupOutcome, SetupPlan, SetupRootAction,
};
use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize};
use std::io;

const INVOCATION_RESPONSE_REF: &str = "handbook.invocation-response@1.0.0";
const USE_CASE_RESPONSE_REF: &str = "handbook.use-case-response@1.0.0";
const TRANSPORT_FAILURE_REF: &str = "handbook.transport-failure@1.0.0";
const SETUP_USE_CASE_REF: &str = "handbook.use-case.repository-setup@1.0.0";
const SETUP_RESULT_REF: &str = "handbook.use-case.repository-setup-result@1.0.0";
const SETUP_DETAILS_REF: &str = "handbook.use-case.repository-setup-details@1.0.0";
const SETUP_RULE_REF: &str = "handbook.use-case.repository-setup-rule@1.0.0";
const SETUP_ROOT_RECORD_REF: &str = "handbook.use-case.repository-setup-root@1.0.0";
const SETUP_RESET_RECORD_REF: &str = "handbook.use-case.repository-setup-runtime-state-reset@1.0.0";
const SETUP_IDENTITY_RECORD_REF: &str =
    "handbook.use-case.repository-setup-repository-identity@1.0.0";
const DOCTOR_USE_CASE_REF: &str = "handbook.use-case.repository-doctor@1.0.0";
const DOCTOR_RESULT_REF: &str = "handbook.use-case.repository-doctor-result@1.0.0";
const DOCTOR_DETAILS_REF: &str = "handbook.use-case.repository-doctor-details@1.0.0";
const AUTHOR_CHARTER_DETAILS_REF: &str = "handbook.use-case.author-charter-details@1.0.0";
const AUTHOR_PROJECT_CONTEXT_DETAILS_REF: &str =
    "handbook.use-case.author-project-context-details@1.0.0";
const APPROVER_ADMINISTRATION_DETAILS_REF: &str =
    "handbook.use-case.approver-administration-details@1.0.0";
const AUTHOR_CHARTER_EFFECT_REF: &str = "handbook.use-case.author-charter-effect@1.0.0";
const AUTHOR_PROJECT_CONTEXT_EFFECT_REF: &str =
    "handbook.use-case.author-project-context-effect@1.0.0";
const APPROVER_ADMINISTRATION_EFFECT_REF: &str =
    "handbook.use-case.approver-administration-effect@1.0.0";
const PIPELINE_ROUTE_USE_CASE_REF: &str =
    "handbook.use-case.pipeline-route-resolve-and-persist@1.0.0";
const PIPELINE_ROUTE_DETAILS_REF: &str =
    "handbook.use-case.pipeline-route-resolve-and-persist-details@1.0.0";
const PIPELINE_ROUTE_BASIS_RECORD_REF: &str = "handbook.pipeline-record.route-basis@1.0.0";
const TRANSPORT_DETAILS_REF: &str = "handbook.transport-failure-details@1.0.0";

fn binding(reference: &str) -> Result<ExactBinding, ContractError> {
    if matches!(
        reference,
        INVOCATION_RESPONSE_REF
            | USE_CASE_RESPONSE_REF
            | TRANSPORT_FAILURE_REF
            | SETUP_RESULT_REF
            | SETUP_DETAILS_REF
            | DOCTOR_RESULT_REF
            | DOCTOR_DETAILS_REF
            | AUTHOR_CHARTER_DETAILS_REF
            | AUTHOR_PROJECT_CONTEXT_DETAILS_REF
            | APPROVER_ADMINISTRATION_DETAILS_REF
            | PIPELINE_ROUTE_DETAILS_REF
            | TRANSPORT_DETAILS_REF
            | "handbook.use-case.author-charter-result@1.0.0"
            | "handbook.use-case.author-project-context-result@1.0.0"
            | "handbook.use-case.approver-administration-result@1.0.0"
            | "handbook.use-case.pipeline-route-resolve-and-persist-result@1.0.0"
    ) {
        return super::production_witness::current_schema_binding(reference);
    }
    Ok(ExactBinding::new(
        ExactRef::parse(reference)?,
        sha256_fingerprint(reference.as_bytes()),
    ))
}

fn bounded<const N: usize>(value: &str) -> Result<BoundedString<N>, ContractError> {
    BoundedString::new(value.to_owned())
}

fn optional_bounded<const N: usize>(
    value: Option<&str>,
) -> Result<Option<BoundedString<N>>, ContractError> {
    value.map(bounded).transpose()
}

fn bounded_vec<const N: usize, const M: usize>(
    values: &[String],
) -> Result<BoundedVec<BoundedString<N>, M>, ContractError> {
    BoundedVec::new(
        values
            .iter()
            .map(|value| bounded(value))
            .collect::<Result<Vec<_>, _>>()?,
    )
}

fn optional_reference_fingerprint(
    reference: Option<&str>,
    fingerprint: Option<&str>,
) -> Result<Option<ReferenceFingerprintV1>, ContractError> {
    match (reference, fingerprint) {
        (Some(reference), Some(fingerprint)) => Ok(Some(ReferenceFingerprintV1 {
            reference: bounded(reference)?,
            fingerprint: Fingerprint::parse(fingerprint)?,
        })),
        (None, None) => Ok(None),
        _ => Err(ContractError::InvalidInvariant),
    }
}

fn contract_value<T: Serialize>(
    value: &T,
    limits: &ContractLimits,
) -> Result<ContractValue, ContractError> {
    let bytes = canonical_json_bytes(value, limits)?;
    parse_canonical_json(&bytes, limits)
}

fn admit_contract_value<T>(
    value: &ContractValue,
    limits: &ContractLimits,
) -> Result<T, ContractError>
where
    T: for<'de> Deserialize<'de> + Serialize,
{
    let bytes = canonical_json_bytes(value, limits)?;
    parse_canonical_json(&bytes, limits)
}

macro_rules! closed_enum {
    ($name:ident { $($variant:ident),+ $(,)? }) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
        #[serde(rename_all = "snake_case")]
        pub enum $name {
            $($variant),+
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                #[derive(Deserialize)]
                #[serde(rename_all = "snake_case")]
                enum Raw {
                    $($variant),+
                }
                let raw = Raw::deserialize(deserializer).map_err(|error| {
                    D::Error::custom(classify_contract_error_message(&error.to_string()))
                })?;
                Ok(match raw {
                    $(Raw::$variant => Self::$variant),+
                })
            }
        }
    };
}

closed_enum!(RepositoryReadinessStatusV1 {
    Ready,
    ActionRequired,
    Indeterminate,
    Invalid,
});
closed_enum!(ProfileConditionOutcomeV1 {
    True,
    False,
    Unknown,
    Unresolved,
    Stale,
    Refused,
});
closed_enum!(ProfileConditionReasonV1 {
    EvidenceContractUnavailable,
});
closed_enum!(ProfileRequirednessModeV1 {
    Always,
    Conditional,
    Optional,
});
closed_enum!(ArtifactApplicabilityV1 {
    Required,
    Optional,
    Indeterminate,
});
closed_enum!(ArtifactInspectionStatusV1 {
    Missing,
    StructurallyValid,
    StructurallyInvalid,
    UnsafePath,
    Unreadable,
    NotInspected,
});
closed_enum!(ArtifactInspectionReasonV1 {
    PresentAndStructurallyValid,
    RequiredPathMissing,
    OptionalPathMissing,
    ConditionalEvidenceUnavailablePathMissing,
    ConditionalEvidenceUnavailablePathPresent,
    YamlSyntaxInvalid,
    DuplicateYamlKey,
    DocumentNotObject,
    StructuralValidationFailed,
    DocumentLimitExceeded,
    AggregateReadLimitExceeded,
    SymlinkRefused,
    NonRegularFileRefused,
    UnsafeRepositoryPath,
    UnsupportedPlatformStrictRead,
    RepositoryReadFailed,
    TypedDecodeFailed,
    RenderedViewRefused,
    ObservationChangedDuringInspection,
});
closed_enum!(DoctorCharterDefinitionClosureStatusV1 {
    Resolved,
    Unavailable,
});
closed_enum!(DoctorCharterLifecycleStateV1 { Unobserved });
closed_enum!(DoctorCharterNextActionV1 {
    RunCharterAuthor,
    RepairCanonicalCharter,
    RepairDefinitionClosure,
    ObserveLifecycle,
});

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RepositoryDoctorConditionV1 {
    condition_ref: ExactRef,
    condition_definition_fingerprint: Fingerprint,
    outcome: ProfileConditionOutcomeV1,
    reason: ProfileConditionReasonV1,
    evidence_closure_fingerprint: Option<Fingerprint>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RepositoryDoctorCapabilityV1 {
    instance_id: BoundedString<255>,
    capability_id: BoundedString<255>,
    contract_ref: ExactRef,
    contract_fingerprint: Fingerprint,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RepositoryDoctorArtifactV1 {
    instance_id: BoundedString<255>,
    kind_ref: ExactRef,
    role_id: Option<BoundedString<255>>,
    capability_ids: BoundedVec<BoundedString<255>, 128>,
    canonical_path: BoundedString<4096>,
    requiredness: ProfileRequirednessModeV1,
    condition_ref: Option<ExactRef>,
    condition_outcome: Option<ProfileConditionOutcomeV1>,
    condition_reason: Option<ProfileConditionReasonV1>,
    evidence_closure_fingerprint: Option<Fingerprint>,
    applicability: ArtifactApplicabilityV1,
    inspection_status: ArtifactInspectionStatusV1,
    inspection_reason: ArtifactInspectionReasonV1,
}

impl RepositoryDoctorArtifactV1 {
    fn from_row(row: &ProfileArtifactRow) -> Result<Self, ContractError> {
        Ok(Self {
            instance_id: bounded(&row.instance_id)?,
            kind_ref: ExactRef::parse(&row.kind_ref)?,
            role_id: optional_bounded(row.role_id.as_deref())?,
            capability_ids: bounded_vec(&row.capability_ids)?,
            canonical_path: bounded(&row.canonical_path)?,
            requiredness: row.requiredness.into(),
            condition_ref: row
                .condition_ref
                .as_deref()
                .map(ExactRef::parse)
                .transpose()?,
            condition_outcome: row.condition_outcome.map(Into::into),
            condition_reason: row.condition_reason.map(Into::into),
            evidence_closure_fingerprint: row
                .evidence_closure_fingerprint
                .as_deref()
                .map(Fingerprint::parse)
                .transpose()?,
            applicability: row.applicability.into(),
            inspection_status: row.inspection_status.into(),
            inspection_reason: row.inspection_reason.into(),
        })
    }

    fn to_row(&self) -> ProfileArtifactRow {
        ProfileArtifactRow {
            instance_id: self.instance_id.as_str().to_owned(),
            kind_ref: self.kind_ref.as_str().to_owned(),
            role_id: self.role_id.as_ref().map(|value| value.as_str().to_owned()),
            capability_ids: self
                .capability_ids
                .as_slice()
                .iter()
                .map(|value| value.as_str().to_owned())
                .collect(),
            canonical_path: self.canonical_path.as_str().to_owned(),
            requiredness: self.requiredness.into(),
            condition_ref: self
                .condition_ref
                .as_ref()
                .map(|value| value.as_str().to_owned()),
            condition_outcome: self.condition_outcome.map(Into::into),
            condition_reason: self.condition_reason.map(Into::into),
            evidence_closure_fingerprint: self
                .evidence_closure_fingerprint
                .as_ref()
                .map(|value| value.as_str().to_owned()),
            applicability: self.applicability.into(),
            inspection_status: self.inspection_status.into(),
            inspection_reason: self.inspection_reason.into(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RepositoryDoctorProjectContextV1 {
    instance_id: BoundedString<255>,
    kind_ref: ExactRef,
    canonical_path: BoundedString<4096>,
    source_fingerprint: Fingerprint,
    rendered_output_fingerprint: Fingerprint,
    rendered_media_type: BoundedString<255>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RepositoryDoctorCharterDefinitionV1 {
    exact_ref: ExactRef,
    definition_fingerprint: Fingerprint,
    package_path: BoundedString<4096>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RepositoryDoctorCharterV1 {
    instance_id: BoundedString<255>,
    kind_ref: ExactRef,
    canonical_path: BoundedString<4096>,
    definition_closure_status: DoctorCharterDefinitionClosureStatusV1,
    definition_closure: BoundedVec<RepositoryDoctorCharterDefinitionV1, 128>,
    canonical_status: ArtifactInspectionStatusV1,
    canonical_reason: ArtifactInspectionReasonV1,
    lifecycle_state: DoctorCharterLifecycleStateV1,
    source_fingerprint: Option<Fingerprint>,
    rendered_output_fingerprint: Option<Fingerprint>,
    rendered_media_type: Option<BoundedString<255>>,
    next_actions: BoundedVec<DoctorCharterNextActionV1, 128>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RepositoryDoctorResultV1 {
    profile: ExactBinding,
    stable_role_registry: ExactBinding,
    conditions: BoundedVec<RepositoryDoctorConditionV1, 128>,
    capabilities: BoundedVec<RepositoryDoctorCapabilityV1, 128>,
    artifacts: BoundedVec<RepositoryDoctorArtifactV1, 128>,
    project_context: Option<RepositoryDoctorProjectContextV1>,
    charter: Option<RepositoryDoctorCharterV1>,
    readiness: RepositoryReadinessStatusV1,
}

impl RepositoryDoctorResultV1 {
    pub fn from_report(report: &DoctorReport) -> Result<Self, ContractError> {
        Ok(Self {
            profile: ExactBinding::new(
                ExactRef::parse(&report.profile_ref)?,
                Fingerprint::parse(&report.profile_fingerprint)?,
            ),
            stable_role_registry: ExactBinding::new(
                ExactRef::parse(&report.stable_role_registry_ref)?,
                Fingerprint::parse(&report.stable_role_registry_fingerprint)?,
            ),
            conditions: BoundedVec::new(
                report
                    .conditions
                    .iter()
                    .map(|row| {
                        Ok(RepositoryDoctorConditionV1 {
                            condition_ref: ExactRef::parse(&row.condition_ref)?,
                            condition_definition_fingerprint: Fingerprint::parse(
                                &row.condition_definition_fingerprint,
                            )?,
                            outcome: row.outcome.into(),
                            reason: row.reason.into(),
                            evidence_closure_fingerprint: row
                                .evidence_closure_fingerprint
                                .as_deref()
                                .map(Fingerprint::parse)
                                .transpose()?,
                        })
                    })
                    .collect::<Result<Vec<_>, ContractError>>()?,
            )?,
            capabilities: BoundedVec::new(
                report
                    .capabilities
                    .iter()
                    .map(|row| {
                        Ok(RepositoryDoctorCapabilityV1 {
                            instance_id: bounded(&row.instance_id)?,
                            capability_id: bounded(&row.capability_id)?,
                            contract_ref: ExactRef::parse(&row.contract_ref)?,
                            contract_fingerprint: Fingerprint::parse(&row.contract_fingerprint)?,
                        })
                    })
                    .collect::<Result<Vec<_>, ContractError>>()?,
            )?,
            artifacts: BoundedVec::new(
                report
                    .artifacts
                    .iter()
                    .map(RepositoryDoctorArtifactV1::from_row)
                    .collect::<Result<Vec<_>, ContractError>>()?,
            )?,
            project_context: report
                .project_context
                .as_ref()
                .map(|row| {
                    Ok(RepositoryDoctorProjectContextV1 {
                        instance_id: bounded(&row.instance_id)?,
                        kind_ref: ExactRef::parse(&row.kind_ref)?,
                        canonical_path: bounded(&row.canonical_path)?,
                        source_fingerprint: Fingerprint::parse(&row.source_fingerprint)?,
                        rendered_output_fingerprint: Fingerprint::parse(
                            &row.rendered_output_fingerprint,
                        )?,
                        rendered_media_type: bounded(&row.rendered_media_type)?,
                    })
                })
                .transpose()?,
            charter: report
                .charter
                .as_ref()
                .map(|row| {
                    Ok(RepositoryDoctorCharterV1 {
                        instance_id: bounded(&row.instance_id)?,
                        kind_ref: ExactRef::parse(&row.kind_ref)?,
                        canonical_path: bounded(&row.canonical_path)?,
                        definition_closure_status: row.definition_closure_status.into(),
                        definition_closure: BoundedVec::new(
                            row.definition_closure
                                .iter()
                                .map(|definition| {
                                    Ok(RepositoryDoctorCharterDefinitionV1 {
                                        exact_ref: ExactRef::parse(&definition.exact_ref)?,
                                        definition_fingerprint: Fingerprint::parse(
                                            &definition.definition_fingerprint,
                                        )?,
                                        package_path: bounded(&definition.package_path)?,
                                    })
                                })
                                .collect::<Result<Vec<_>, ContractError>>()?,
                        )?,
                        canonical_status: row.canonical_status.into(),
                        canonical_reason: row.canonical_reason.into(),
                        lifecycle_state: row.lifecycle_state.into(),
                        source_fingerprint: row
                            .source_fingerprint
                            .as_deref()
                            .map(Fingerprint::parse)
                            .transpose()?,
                        rendered_output_fingerprint: row
                            .rendered_output_fingerprint
                            .as_deref()
                            .map(Fingerprint::parse)
                            .transpose()?,
                        rendered_media_type: optional_bounded(row.rendered_media_type.as_deref())?,
                        next_actions: BoundedVec::new(
                            row.next_actions.iter().copied().map(Into::into).collect(),
                        )?,
                    })
                })
                .transpose()?,
            readiness: report.status.into(),
        })
    }

    pub fn readiness(&self) -> RepositoryReadinessStatusV1 {
        self.readiness
    }

    pub fn to_report(&self) -> DoctorReport {
        DoctorReport {
            profile_ref: self.profile.reference().as_str().to_owned(),
            profile_fingerprint: self.profile.fingerprint().as_str().to_owned(),
            stable_role_registry_ref: self.stable_role_registry.reference().as_str().to_owned(),
            stable_role_registry_fingerprint: self
                .stable_role_registry
                .fingerprint()
                .as_str()
                .to_owned(),
            conditions: self
                .conditions
                .as_slice()
                .iter()
                .map(|row| ProfileConditionRow {
                    condition_ref: row.condition_ref.as_str().to_owned(),
                    condition_definition_fingerprint: row
                        .condition_definition_fingerprint
                        .as_str()
                        .to_owned(),
                    outcome: row.outcome.into(),
                    reason: row.reason.into(),
                    evidence_closure_fingerprint: row
                        .evidence_closure_fingerprint
                        .as_ref()
                        .map(|value| value.as_str().to_owned()),
                })
                .collect(),
            capabilities: self
                .capabilities
                .as_slice()
                .iter()
                .map(|row| ProfileCapabilityRow {
                    instance_id: row.instance_id.as_str().to_owned(),
                    capability_id: row.capability_id.as_str().to_owned(),
                    contract_ref: row.contract_ref.as_str().to_owned(),
                    contract_fingerprint: row.contract_fingerprint.as_str().to_owned(),
                })
                .collect(),
            artifacts: self
                .artifacts
                .as_slice()
                .iter()
                .map(RepositoryDoctorArtifactV1::to_row)
                .collect(),
            project_context: self
                .project_context
                .as_ref()
                .map(|row| DoctorProjectContextRow {
                    instance_id: row.instance_id.as_str().to_owned(),
                    kind_ref: row.kind_ref.as_str().to_owned(),
                    canonical_path: row.canonical_path.as_str().to_owned(),
                    source_fingerprint: row.source_fingerprint.as_str().to_owned(),
                    rendered_output_fingerprint: row
                        .rendered_output_fingerprint
                        .as_str()
                        .to_owned(),
                    rendered_media_type: row.rendered_media_type.as_str().to_owned(),
                }),
            charter: self.charter.as_ref().map(|row| DoctorCharterRow {
                instance_id: row.instance_id.as_str().to_owned(),
                kind_ref: row.kind_ref.as_str().to_owned(),
                canonical_path: row.canonical_path.as_str().to_owned(),
                definition_closure_status: row.definition_closure_status.into(),
                definition_closure: row
                    .definition_closure
                    .as_slice()
                    .iter()
                    .map(|definition| DoctorCharterDefinitionRow {
                        exact_ref: definition.exact_ref.as_str().to_owned(),
                        definition_fingerprint: definition
                            .definition_fingerprint
                            .as_str()
                            .to_owned(),
                        package_path: definition.package_path.as_str().to_owned(),
                    })
                    .collect(),
                canonical_status: row.canonical_status.into(),
                canonical_reason: row.canonical_reason.into(),
                lifecycle_state: row.lifecycle_state.into(),
                source_fingerprint: row
                    .source_fingerprint
                    .as_ref()
                    .map(|value| value.as_str().to_owned()),
                rendered_output_fingerprint: row
                    .rendered_output_fingerprint
                    .as_ref()
                    .map(|value| value.as_str().to_owned()),
                rendered_media_type: row
                    .rendered_media_type
                    .as_ref()
                    .map(|value| value.as_str().to_owned()),
                next_actions: row
                    .next_actions
                    .as_slice()
                    .iter()
                    .copied()
                    .map(Into::into)
                    .collect(),
            }),
            status: self.readiness.into(),
        }
    }
}

macro_rules! map_enum {
    ($source:ty => $target:ty { $($variant:ident),+ $(,)? }) => {
        impl From<$source> for $target {
            fn from(value: $source) -> Self {
                match value {
                    $(<$source>::$variant => <$target>::$variant),+
                }
            }
        }
    };
}

map_enum!(RepositoryReadinessStatus => RepositoryReadinessStatusV1 {
    Ready, ActionRequired, Indeterminate, Invalid
});
map_enum!(ProfileConditionOutcome => ProfileConditionOutcomeV1 {
    True, False, Unknown, Unresolved, Stale, Refused
});
map_enum!(ProfileConditionReason => ProfileConditionReasonV1 {
    EvidenceContractUnavailable
});
map_enum!(ProfileRequirednessMode => ProfileRequirednessModeV1 {
    Always, Conditional, Optional
});
map_enum!(ArtifactApplicability => ArtifactApplicabilityV1 {
    Required, Optional, Indeterminate
});
map_enum!(ArtifactInspectionStatus => ArtifactInspectionStatusV1 {
    Missing, StructurallyValid, StructurallyInvalid, UnsafePath, Unreadable, NotInspected
});
map_enum!(ArtifactInspectionReason => ArtifactInspectionReasonV1 {
    PresentAndStructurallyValid,
    RequiredPathMissing,
    OptionalPathMissing,
    ConditionalEvidenceUnavailablePathMissing,
    ConditionalEvidenceUnavailablePathPresent,
    YamlSyntaxInvalid,
    DuplicateYamlKey,
    DocumentNotObject,
    StructuralValidationFailed,
    DocumentLimitExceeded,
    AggregateReadLimitExceeded,
    SymlinkRefused,
    NonRegularFileRefused,
    UnsafeRepositoryPath,
    UnsupportedPlatformStrictRead,
    RepositoryReadFailed,
    TypedDecodeFailed,
    RenderedViewRefused,
    ObservationChangedDuringInspection
});
map_enum!(DoctorCharterDefinitionClosureStatus => DoctorCharterDefinitionClosureStatusV1 {
    Resolved, Unavailable
});
map_enum!(DoctorCharterLifecycleState => DoctorCharterLifecycleStateV1 { Unobserved });
map_enum!(DoctorCharterNextAction => DoctorCharterNextActionV1 {
    RunCharterAuthor, RepairCanonicalCharter, RepairDefinitionClosure, ObserveLifecycle
});

map_enum!(RepositoryReadinessStatusV1 => RepositoryReadinessStatus {
    Ready, ActionRequired, Indeterminate, Invalid
});
map_enum!(ProfileConditionOutcomeV1 => ProfileConditionOutcome {
    True, False, Unknown, Unresolved, Stale, Refused
});
map_enum!(ProfileConditionReasonV1 => ProfileConditionReason {
    EvidenceContractUnavailable
});
map_enum!(ProfileRequirednessModeV1 => ProfileRequirednessMode {
    Always, Conditional, Optional
});
map_enum!(ArtifactApplicabilityV1 => ArtifactApplicability {
    Required, Optional, Indeterminate
});
map_enum!(ArtifactInspectionStatusV1 => ArtifactInspectionStatus {
    Missing, StructurallyValid, StructurallyInvalid, UnsafePath, Unreadable, NotInspected
});
map_enum!(ArtifactInspectionReasonV1 => ArtifactInspectionReason {
    PresentAndStructurallyValid,
    RequiredPathMissing,
    OptionalPathMissing,
    ConditionalEvidenceUnavailablePathMissing,
    ConditionalEvidenceUnavailablePathPresent,
    YamlSyntaxInvalid,
    DuplicateYamlKey,
    DocumentNotObject,
    StructuralValidationFailed,
    DocumentLimitExceeded,
    AggregateReadLimitExceeded,
    SymlinkRefused,
    NonRegularFileRefused,
    UnsafeRepositoryPath,
    UnsupportedPlatformStrictRead,
    RepositoryReadFailed,
    TypedDecodeFailed,
    RenderedViewRefused,
    ObservationChangedDuringInspection
});
map_enum!(DoctorCharterDefinitionClosureStatusV1 => DoctorCharterDefinitionClosureStatus {
    Resolved, Unavailable
});
map_enum!(DoctorCharterLifecycleStateV1 => DoctorCharterLifecycleState { Unobserved });
map_enum!(DoctorCharterNextActionV1 => DoctorCharterNextAction {
    RunCharterAuthor, RepairCanonicalCharter, RepairDefinitionClosure, ObserveLifecycle
});

closed_enum!(RepositorySetupModeV1 {
    Auto,
    Init,
    Refresh
});
closed_enum!(RepositorySetupRootActionV1 { Preserve, Create });
closed_enum!(RepositorySetupArtifactActionKindV1 {
    Preserve,
    AuthorRequired,
    OptionalAbsent,
    ConditionIndeterminate,
    Invalid,
});

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RepositorySetupArtifactV1 {
    artifact: RepositoryDoctorArtifactV1,
    action: RepositorySetupArtifactActionKindV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RepositorySetupResultV1 {
    requested_mode: RepositorySetupModeV1,
    resolved_mode: RepositorySetupModeV1,
    root_action: RepositorySetupRootActionV1,
    profile: ExactBinding,
    stable_role_registry: ExactBinding,
    conditions: BoundedVec<RepositoryDoctorConditionV1, 128>,
    capabilities: BoundedVec<RepositoryDoctorCapabilityV1, 128>,
    artifacts: BoundedVec<RepositorySetupArtifactV1, 128>,
    reset_paths: BoundedVec<BoundedString<4096>, 128>,
    readiness: RepositoryReadinessStatusV1,
    reset_applied: bool,
}

impl RepositorySetupResultV1 {
    pub fn from_outcome(outcome: &SetupOutcome) -> Result<Self, ContractError> {
        let plan = &outcome.plan;
        Ok(Self {
            requested_mode: plan.requested_mode.into(),
            resolved_mode: plan.resolved_mode.into(),
            root_action: plan.root_action.into(),
            profile: ExactBinding::new(
                ExactRef::parse(&plan.profile_ref)?,
                Fingerprint::parse(&plan.profile_fingerprint)?,
            ),
            stable_role_registry: ExactBinding::new(
                ExactRef::parse(&plan.stable_role_registry_ref)?,
                Fingerprint::parse(&plan.stable_role_registry_fingerprint)?,
            ),
            conditions: BoundedVec::new(
                plan.conditions
                    .iter()
                    .map(|row| {
                        Ok(RepositoryDoctorConditionV1 {
                            condition_ref: ExactRef::parse(&row.condition_ref)?,
                            condition_definition_fingerprint: Fingerprint::parse(
                                &row.condition_definition_fingerprint,
                            )?,
                            outcome: row.outcome.into(),
                            reason: row.reason.into(),
                            evidence_closure_fingerprint: row
                                .evidence_closure_fingerprint
                                .as_deref()
                                .map(Fingerprint::parse)
                                .transpose()?,
                        })
                    })
                    .collect::<Result<Vec<_>, ContractError>>()?,
            )?,
            capabilities: BoundedVec::new(
                plan.capabilities
                    .iter()
                    .map(|row| {
                        Ok(RepositoryDoctorCapabilityV1 {
                            instance_id: bounded(&row.instance_id)?,
                            capability_id: bounded(&row.capability_id)?,
                            contract_ref: ExactRef::parse(&row.contract_ref)?,
                            contract_fingerprint: Fingerprint::parse(&row.contract_fingerprint)?,
                        })
                    })
                    .collect::<Result<Vec<_>, ContractError>>()?,
            )?,
            artifacts: BoundedVec::new(
                plan.artifacts
                    .iter()
                    .map(|row| {
                        Ok(RepositorySetupArtifactV1 {
                            artifact: RepositoryDoctorArtifactV1::from_row(&row.artifact)?,
                            action: row.action.into(),
                        })
                    })
                    .collect::<Result<Vec<_>, ContractError>>()?,
            )?,
            reset_paths: bounded_vec(&plan.reset_paths)?,
            readiness: outcome.status.into(),
            reset_applied: outcome.reset_applied,
        })
    }

    pub fn to_outcome(&self) -> SetupOutcome {
        SetupOutcome {
            plan: SetupPlan {
                requested_mode: self.requested_mode.into(),
                resolved_mode: self.resolved_mode.into(),
                root_action: self.root_action.into(),
                profile_ref: self.profile.reference().as_str().to_owned(),
                profile_fingerprint: self.profile.fingerprint().as_str().to_owned(),
                stable_role_registry_ref: self.stable_role_registry.reference().as_str().to_owned(),
                stable_role_registry_fingerprint: self
                    .stable_role_registry
                    .fingerprint()
                    .as_str()
                    .to_owned(),
                conditions: self
                    .conditions
                    .as_slice()
                    .iter()
                    .map(|row| ProfileConditionRow {
                        condition_ref: row.condition_ref.as_str().to_owned(),
                        condition_definition_fingerprint: row
                            .condition_definition_fingerprint
                            .as_str()
                            .to_owned(),
                        outcome: row.outcome.into(),
                        reason: row.reason.into(),
                        evidence_closure_fingerprint: row
                            .evidence_closure_fingerprint
                            .as_ref()
                            .map(|value| value.as_str().to_owned()),
                    })
                    .collect(),
                capabilities: self
                    .capabilities
                    .as_slice()
                    .iter()
                    .map(|row| ProfileCapabilityRow {
                        instance_id: row.instance_id.as_str().to_owned(),
                        capability_id: row.capability_id.as_str().to_owned(),
                        contract_ref: row.contract_ref.as_str().to_owned(),
                        contract_fingerprint: row.contract_fingerprint.as_str().to_owned(),
                    })
                    .collect(),
                artifacts: self
                    .artifacts
                    .as_slice()
                    .iter()
                    .map(|row| SetupArtifactAction {
                        artifact: row.artifact.to_row(),
                        action: row.action.into(),
                    })
                    .collect(),
                reset_paths: self
                    .reset_paths
                    .as_slice()
                    .iter()
                    .map(|path| path.as_str().to_owned())
                    .collect(),
            },
            status: self.readiness.into(),
            reset_applied: self.reset_applied,
        }
    }
}

map_enum!(SetupMode => RepositorySetupModeV1 { Auto, Init, Refresh });
map_enum!(SetupRootAction => RepositorySetupRootActionV1 { Preserve, Create });
map_enum!(SetupArtifactActionKind => RepositorySetupArtifactActionKindV1 {
    Preserve, AuthorRequired, OptionalAbsent, ConditionIndeterminate, Invalid
});
map_enum!(RepositorySetupModeV1 => SetupMode { Auto, Init, Refresh });
map_enum!(RepositorySetupRootActionV1 => SetupRootAction { Preserve, Create });
map_enum!(RepositorySetupArtifactActionKindV1 => SetupArtifactActionKind {
    Preserve, AuthorRequired, OptionalAbsent, ConditionIndeterminate, Invalid
});

closed_enum!(RepositorySetupErrorKindV1 {
    ProfileResolution,
    ProfileDecision,
    AlreadyInitialized,
    MissingCanonicalRoot,
    InvalidCanonicalRoot,
    InvalidRequest,
    MaterializerUnavailable,
    RuntimeStatePlan,
    RuntimeStateApply,
    RepositoryIdentity,
});
closed_enum!(RepositorySetupErrorReasonV1 {
    ShippedProfileUnavailable,
    SelectedProfileDecisionInvalid,
    UnresolvedMode,
    InitRejectsRefreshFlags,
    RootAlreadyInitialized,
    RefreshRootMissing,
    RootNotDirectory,
    RootSymlinkRefused,
    CanonicalRootInspectFailed,
    CanonicalRootCreateFailed,
    RewriteHasNoMaterializer,
    RuntimeStateTargetUnsafe,
    RuntimeStateResetPlanTooLarge,
    RuntimeStateResetProjectionTooLarge,
    RuntimeStateMutationFailed,
    RepositoryAuthorityRecoveryBlocked,
    RepositoryIdentityUnsafe,
    RepositoryIdentityMismatch,
    RepositoryIdentityEntropyUnavailable,
    RepositoryIdentityPersistenceFailed,
});
closed_enum!(RepositorySetupErrorCodeV1 {
    ShippedProfileUnavailable,
    SelectedProfileDecisionInvalid,
    UnresolvedMode,
    InitRejectsRefreshFlags,
    RootAlreadyInitialized,
    RefreshRootMissing,
    RootNotDirectory,
    RootSymlinkRefused,
    CanonicalRootInspectFailed,
    CanonicalRootCreateFailed,
    RewriteHasNoMaterializer,
    RuntimeStateTargetUnsafe,
    RuntimeStateResetPlanTooLarge,
    RuntimeStateResetProjectionTooLarge,
    RuntimeStateMutationFailed,
    RepositoryAuthorityRecoveryBlocked,
    RepositoryIdentityUnsafe,
    RepositoryIdentityMismatch,
    RepositoryIdentityEntropyUnavailable,
    RepositoryIdentityPersistenceFailed,
});

map_enum!(SetupErrorKind => RepositorySetupErrorKindV1 {
    ProfileResolution,
    ProfileDecision,
    AlreadyInitialized,
    MissingCanonicalRoot,
    InvalidCanonicalRoot,
    InvalidRequest,
    MaterializerUnavailable,
    RuntimeStatePlan,
    RuntimeStateApply,
    RepositoryIdentity
});
map_enum!(RepositorySetupErrorKindV1 => SetupErrorKind {
    ProfileResolution,
    ProfileDecision,
    AlreadyInitialized,
    MissingCanonicalRoot,
    InvalidCanonicalRoot,
    InvalidRequest,
    MaterializerUnavailable,
    RuntimeStatePlan,
    RuntimeStateApply,
    RepositoryIdentity
});
map_enum!(SetupErrorReasonCode => RepositorySetupErrorReasonV1 {
    ShippedProfileUnavailable,
    SelectedProfileDecisionInvalid,
    UnresolvedMode,
    InitRejectsRefreshFlags,
    RootAlreadyInitialized,
    RefreshRootMissing,
    RootNotDirectory,
    RootSymlinkRefused,
    CanonicalRootInspectFailed,
    CanonicalRootCreateFailed,
    RewriteHasNoMaterializer,
    RuntimeStateTargetUnsafe,
    RuntimeStateResetPlanTooLarge,
    RuntimeStateResetProjectionTooLarge,
    RuntimeStateMutationFailed,
    RepositoryAuthorityRecoveryBlocked,
    RepositoryIdentityUnsafe,
    RepositoryIdentityMismatch,
    RepositoryIdentityEntropyUnavailable,
    RepositoryIdentityPersistenceFailed
});
map_enum!(RepositorySetupErrorReasonV1 => SetupErrorReasonCode {
    ShippedProfileUnavailable,
    SelectedProfileDecisionInvalid,
    UnresolvedMode,
    InitRejectsRefreshFlags,
    RootAlreadyInitialized,
    RefreshRootMissing,
    RootNotDirectory,
    RootSymlinkRefused,
    CanonicalRootInspectFailed,
    CanonicalRootCreateFailed,
    RewriteHasNoMaterializer,
    RuntimeStateTargetUnsafe,
    RuntimeStateResetPlanTooLarge,
    RuntimeStateResetProjectionTooLarge,
    RuntimeStateMutationFailed,
    RepositoryAuthorityRecoveryBlocked,
    RepositoryIdentityUnsafe,
    RepositoryIdentityMismatch,
    RepositoryIdentityEntropyUnavailable,
    RepositoryIdentityPersistenceFailed
});
map_enum!(SetupErrorCode => RepositorySetupErrorCodeV1 {
    ShippedProfileUnavailable,
    SelectedProfileDecisionInvalid,
    UnresolvedMode,
    InitRejectsRefreshFlags,
    RootAlreadyInitialized,
    RefreshRootMissing,
    RootNotDirectory,
    RootSymlinkRefused,
    CanonicalRootInspectFailed,
    CanonicalRootCreateFailed,
    RewriteHasNoMaterializer,
    RuntimeStateTargetUnsafe,
    RuntimeStateResetPlanTooLarge,
    RuntimeStateResetProjectionTooLarge,
    RuntimeStateMutationFailed,
    RepositoryAuthorityRecoveryBlocked,
    RepositoryIdentityUnsafe,
    RepositoryIdentityMismatch,
    RepositoryIdentityEntropyUnavailable,
    RepositoryIdentityPersistenceFailed
});
map_enum!(RepositorySetupErrorCodeV1 => SetupErrorCode {
    ShippedProfileUnavailable,
    SelectedProfileDecisionInvalid,
    UnresolvedMode,
    InitRejectsRefreshFlags,
    RootAlreadyInitialized,
    RefreshRootMissing,
    RootNotDirectory,
    RootSymlinkRefused,
    CanonicalRootInspectFailed,
    CanonicalRootCreateFailed,
    RewriteHasNoMaterializer,
    RuntimeStateTargetUnsafe,
    RuntimeStateResetPlanTooLarge,
    RuntimeStateResetProjectionTooLarge,
    RuntimeStateMutationFailed,
    RepositoryAuthorityRecoveryBlocked,
    RepositoryIdentityUnsafe,
    RepositoryIdentityMismatch,
    RepositoryIdentityEntropyUnavailable,
    RepositoryIdentityPersistenceFailed
});

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RepositorySetupBoundsV1 {
    observed_count: SafeU64,
    limit: SafeU64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RepositorySetupSizeBoundsV1 {
    observed_bytes: SafeU64,
    limit_bytes: SafeU64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "details_kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum RepositorySetupDetailsV1 {
    Readiness {
        result: Box<RepositorySetupResultV1>,
    },
    SetupFailure {
        code: RepositorySetupErrorCodeV1,
        failure_kind: RepositorySetupErrorKindV1,
        reason: RepositorySetupErrorReasonV1,
        repo_relative_path: Option<BoundedString<4096>>,
        bounds: Option<RepositorySetupBoundsV1>,
        size_bounds: Option<RepositorySetupSizeBoundsV1>,
    },
}

impl RepositorySetupDetailsV1 {
    fn from_error(error: &SetupError) -> Result<Self, ContractError> {
        let bounds = match (error.observed_count(), error.limit()) {
            (Some(observed_count), Some(limit)) => Some(RepositorySetupBoundsV1 {
                observed_count: SafeU64::new(
                    u64::try_from(observed_count).map_err(|_| ContractError::InvalidNumber)?,
                )?,
                limit: SafeU64::new(
                    u64::try_from(limit).map_err(|_| ContractError::InvalidNumber)?,
                )?,
            }),
            (None, None) => None,
            _ => return Err(ContractError::InvalidInvariant),
        };
        let size_bounds = match (error.observed_bytes(), error.limit_bytes()) {
            (Some(observed_bytes), Some(limit_bytes)) => Some(RepositorySetupSizeBoundsV1 {
                observed_bytes: SafeU64::new(
                    u64::try_from(observed_bytes).map_err(|_| ContractError::InvalidNumber)?,
                )?,
                limit_bytes: SafeU64::new(
                    u64::try_from(limit_bytes).map_err(|_| ContractError::InvalidNumber)?,
                )?,
            }),
            (None, None) => None,
            _ => return Err(ContractError::InvalidInvariant),
        };
        Ok(Self::SetupFailure {
            code: error.code().into(),
            failure_kind: error.kind().into(),
            reason: error.reason_code().into(),
            repo_relative_path: optional_bounded(error.repo_relative_path())?,
            bounds,
            size_bounds,
        })
    }

    fn to_error(&self) -> Result<Option<SetupError>, ContractError> {
        let Self::SetupFailure {
            code,
            failure_kind,
            reason,
            repo_relative_path,
            bounds,
            size_bounds,
        } = self
        else {
            return Ok(None);
        };
        let code: SetupErrorCode = (*code).into();
        let error = if code == SetupErrorCode::RuntimeStateResetPlanTooLarge {
            let bounds = bounds.as_ref().ok_or(ContractError::InvalidInvariant)?;
            if size_bounds.is_some()
                || bounds.limit.get() != 128
                || bounds.observed_count.get() <= bounds.limit.get()
            {
                return Err(ContractError::InvalidInvariant);
            }
            SetupError::runtime_state_reset_plan_too_large(
                usize::try_from(bounds.observed_count.get())
                    .map_err(|_| ContractError::InvalidNumber)?,
            )
        } else if code == SetupErrorCode::RuntimeStateResetProjectionTooLarge {
            let size_bounds = size_bounds
                .as_ref()
                .ok_or(ContractError::InvalidInvariant)?;
            if bounds.is_some()
                || size_bounds.limit_bytes.get() != 1_048_576
                || size_bounds.observed_bytes.get() <= size_bounds.limit_bytes.get()
            {
                return Err(ContractError::InvalidInvariant);
            }
            SetupError::runtime_state_reset_projection_too_large(
                usize::try_from(size_bounds.observed_bytes.get())
                    .map_err(|_| ContractError::InvalidNumber)?,
            )
        } else {
            if bounds.is_some() || size_bounds.is_some() {
                return Err(ContractError::InvalidInvariant);
            }
            SetupError::from_code(code)
        };
        if error.kind() != (*failure_kind).into()
            || error.reason_code() != (*reason).into()
            || error.repo_relative_path() != repo_relative_path.as_ref().map(BoundedString::as_str)
        {
            return Err(ContractError::InvalidInvariant);
        }
        Ok(Some(error))
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
enum RepositorySetupResetApplicationV1 {
    Sequential,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "effect_kind", rename_all = "snake_case", deny_unknown_fields)]
enum RepositorySetupReceiptConditionV1 {
    RootCreated {
        repo_relative_path: BoundedString<4096>,
    },
    RuntimeStateReset {
        application: RepositorySetupResetApplicationV1,
        removed_paths: BoundedVec<BoundedString<4096>, 128>,
    },
    RepositoryIdentityCreated {
        repo_relative_path: BoundedString<4096>,
        repository_identity_fingerprint: Fingerprint,
    },
}

closed_enum!(AuthorCharterOperationV1 {
    Author,
    Approve,
    Promote,
    Validate,
});

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReferenceFingerprintV1 {
    reference: BoundedString<4096>,
    fingerprint: Fingerprint,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AuthorCharterResultV1 {
    operation: AuthorCharterOperationV1,
    canonical_path: Option<BoundedString<4096>>,
    source_fingerprint: Option<Fingerprint>,
    rendered_output_fingerprint: Option<Fingerprint>,
    intake: Option<ReferenceFingerprintV1>,
    candidate: Option<ReferenceFingerprintV1>,
    approval: Option<ReferenceFingerprintV1>,
    promotion: Option<ReferenceFingerprintV1>,
    changed_paths: BoundedVec<BoundedString<4096>, 128>,
    next_actions: BoundedVec<BoundedString<1024>, 128>,
}

impl AuthorCharterResultV1 {
    pub fn from_result(result: &CharterOperationResult) -> Result<Self, ContractError> {
        if result.status != AuthorOperationStatus::Succeeded
            || result.refusal.is_some()
            || result.invocation_failure.is_some()
        {
            return Err(ContractError::InvalidInvariant);
        }
        Ok(Self {
            operation: result.operation.into(),
            canonical_path: optional_bounded(result.canonical_path.as_deref())?,
            source_fingerprint: result
                .source_fingerprint
                .as_deref()
                .map(Fingerprint::parse)
                .transpose()?,
            rendered_output_fingerprint: result
                .rendered_output_fingerprint
                .as_deref()
                .map(Fingerprint::parse)
                .transpose()?,
            intake: optional_reference_fingerprint(
                result.intake_ref.as_deref(),
                result.intake_fingerprint.as_deref(),
            )?,
            candidate: optional_reference_fingerprint(
                result.candidate_ref.as_deref(),
                result.candidate_fingerprint.as_deref(),
            )?,
            approval: optional_reference_fingerprint(
                result.approval_ref.as_deref(),
                result.approval_fingerprint.as_deref(),
            )?,
            promotion: optional_reference_fingerprint(
                result.promotion_ref.as_deref(),
                result.promotion_fingerprint.as_deref(),
            )?,
            changed_paths: bounded_vec(&result.changed_paths)?,
            next_actions: bounded_vec(&result.next_actions)?,
        })
    }

    fn to_result(&self) -> CharterOperationResult {
        CharterOperationResult {
            schema_id: "handbook.charter-operation-result".to_owned(),
            schema_version: "1.0".to_owned(),
            operation: self.operation.into(),
            status: AuthorOperationStatus::Succeeded,
            canonical_path: optional_string(&self.canonical_path),
            source_fingerprint: optional_fingerprint(&self.source_fingerprint),
            rendered_output_fingerprint: optional_fingerprint(&self.rendered_output_fingerprint),
            intake_ref: optional_reference(&self.intake),
            intake_fingerprint: optional_reference_fingerprint_value(&self.intake),
            candidate_ref: optional_reference(&self.candidate),
            candidate_fingerprint: optional_reference_fingerprint_value(&self.candidate),
            approval_ref: optional_reference(&self.approval),
            approval_fingerprint: optional_reference_fingerprint_value(&self.approval),
            promotion_ref: optional_reference(&self.promotion),
            promotion_fingerprint: optional_reference_fingerprint_value(&self.promotion),
            changed_paths: bounded_strings(&self.changed_paths),
            refusal: None,
            next_actions: bounded_strings(&self.next_actions),
            invocation_failure: None,
            causal_disposition: Some(
                crate::charter_product::AdapterCausalDisposition::CallerRefusal,
            ),
            public_effects: Vec::new(),
        }
    }
}

map_enum!(CharterOperation => AuthorCharterOperationV1 {
    Author, Approve, Promote, Validate
});

impl From<AuthorCharterOperationV1> for CharterOperation {
    fn from(value: AuthorCharterOperationV1) -> Self {
        match value {
            AuthorCharterOperationV1::Author => Self::Author,
            AuthorCharterOperationV1::Approve => Self::Approve,
            AuthorCharterOperationV1::Promote => Self::Promote,
            AuthorCharterOperationV1::Validate => Self::Validate,
        }
    }
}

closed_enum!(UseCaseCauseV1 {
    Prerequisite,
    CallerRefusal,
    ImplementationFailure,
});

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct AuthorOperationRefusalV1 {
    code: BoundedString<255>,
    message: BoundedString<4096>,
    retryable: bool,
}

impl AuthorOperationRefusalV1 {
    fn from_refusal(refusal: &AuthorOperationRefusal) -> Result<Self, ContractError> {
        Ok(Self {
            code: bounded(&refusal.code)?,
            message: bounded(&refusal.message)?,
            retryable: refusal.retryable,
        })
    }

    fn to_refusal(&self) -> AuthorOperationRefusal {
        AuthorOperationRefusal {
            code: self.code.as_str().to_owned(),
            message: self.message.as_str().to_owned(),
            retryable: self.retryable,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct RepositoryInvocationFailureDetailsV1 {
    schema_id: BoundedString<255>,
    schema_version: BoundedString<255>,
    operation: BoundedString<255>,
    stage: Option<BoundedString<255>>,
    status: BoundedString<255>,
    repository_identity_fingerprint: Option<Fingerprint>,
    operation_id: Option<BoundedString<255>>,
    changed_paths: BoundedVec<BoundedString<4096>, 128>,
    refusal: AuthorOperationRefusalV1,
    next_actions: BoundedVec<BoundedString<1024>, 128>,
}

impl RepositoryInvocationFailureDetailsV1 {
    fn from_view(view: &RepositoryInvocationFailureView) -> Result<Self, ContractError> {
        let details = Self {
            schema_id: bounded(&view.schema_id)?,
            schema_version: bounded(&view.schema_version)?,
            operation: bounded(&view.operation)?,
            stage: optional_bounded(view.stage.as_deref())?,
            status: bounded(&view.status)?,
            repository_identity_fingerprint: view
                .repository_identity_fingerprint
                .as_deref()
                .map(Fingerprint::parse)
                .transpose()?,
            operation_id: optional_bounded(view.operation_id.as_deref())?,
            changed_paths: bounded_vec(&view.changed_paths)?,
            refusal: AuthorOperationRefusalV1::from_refusal(&view.refusal)?,
            next_actions: bounded_vec(&view.next_actions)?,
        };
        if details
            .exact_preflight_cause()
            .is_some_and(|cause| cause != view.causal_disposition)
        {
            return Err(ContractError::InvalidInvariant);
        }
        Ok(details)
    }

    fn to_view(
        &self,
        causal_disposition: AdapterCausalDisposition,
    ) -> RepositoryInvocationFailureView {
        RepositoryInvocationFailureView {
            schema_id: self.schema_id.as_str().to_owned(),
            schema_version: self.schema_version.as_str().to_owned(),
            operation: self.operation.as_str().to_owned(),
            stage: self.stage.as_ref().map(|value| value.as_str().to_owned()),
            status: self.status.as_str().to_owned(),
            repository_identity_fingerprint: self
                .repository_identity_fingerprint
                .as_ref()
                .map(|value| value.as_str().to_owned()),
            operation_id: self
                .operation_id
                .as_ref()
                .map(|value| value.as_str().to_owned()),
            changed_paths: bounded_strings(&self.changed_paths),
            refusal: self.refusal.to_refusal(),
            next_actions: bounded_strings(&self.next_actions),
            causal_disposition,
        }
    }

    fn exact_preflight_cause(&self) -> Option<AdapterCausalDisposition> {
        match (
            self.schema_id.as_str(),
            self.stage.as_ref().map(BoundedString::as_str),
        ) {
            ("handbook.repository-invocation-preflight-result", Some("repository_identity")) => {
                None
            }
            ("handbook.repository-invocation-preflight-result", Some("operation_id")) => {
                Some(AdapterCausalDisposition::ImplementationFailure)
            }
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AuthorCharterDetailsV1 {
    cause: UseCaseCauseV1,
    operation: AuthorCharterOperationV1,
    canonical_path: Option<BoundedString<4096>>,
    source_fingerprint: Option<Fingerprint>,
    rendered_output_fingerprint: Option<Fingerprint>,
    intake: Option<ReferenceFingerprintV1>,
    candidate: Option<ReferenceFingerprintV1>,
    approval: Option<ReferenceFingerprintV1>,
    promotion: Option<ReferenceFingerprintV1>,
    changed_paths: BoundedVec<BoundedString<4096>, 128>,
    refusal: AuthorOperationRefusalV1,
    next_actions: BoundedVec<BoundedString<1024>, 128>,
}

impl AuthorCharterDetailsV1 {
    fn from_result(result: &CharterOperationResult) -> Result<Self, ContractError> {
        if result.status != AuthorOperationStatus::Refused || result.invocation_failure.is_some() {
            return Err(ContractError::InvalidInvariant);
        }
        let refusal = result
            .refusal
            .as_ref()
            .ok_or(ContractError::InvalidInvariant)?;
        Ok(Self {
            cause: result
                .causal_disposition
                .map(adapter_cause)
                .ok_or(ContractError::InvalidInvariant)?,
            operation: result.operation.into(),
            canonical_path: optional_bounded(result.canonical_path.as_deref())?,
            source_fingerprint: result
                .source_fingerprint
                .as_deref()
                .map(Fingerprint::parse)
                .transpose()?,
            rendered_output_fingerprint: result
                .rendered_output_fingerprint
                .as_deref()
                .map(Fingerprint::parse)
                .transpose()?,
            intake: optional_reference_fingerprint(
                result.intake_ref.as_deref(),
                result.intake_fingerprint.as_deref(),
            )?,
            candidate: optional_reference_fingerprint(
                result.candidate_ref.as_deref(),
                result.candidate_fingerprint.as_deref(),
            )?,
            approval: optional_reference_fingerprint(
                result.approval_ref.as_deref(),
                result.approval_fingerprint.as_deref(),
            )?,
            promotion: optional_reference_fingerprint(
                result.promotion_ref.as_deref(),
                result.promotion_fingerprint.as_deref(),
            )?,
            changed_paths: bounded_vec(&result.changed_paths)?,
            refusal: AuthorOperationRefusalV1::from_refusal(refusal)?,
            next_actions: bounded_vec(&result.next_actions)?,
        })
    }

    fn to_result(&self) -> CharterOperationResult {
        CharterOperationResult {
            schema_id: "handbook.charter-operation-result".to_owned(),
            schema_version: "1.0".to_owned(),
            operation: self.operation.into(),
            status: AuthorOperationStatus::Refused,
            canonical_path: optional_string(&self.canonical_path),
            source_fingerprint: optional_fingerprint(&self.source_fingerprint),
            rendered_output_fingerprint: optional_fingerprint(&self.rendered_output_fingerprint),
            intake_ref: optional_reference(&self.intake),
            intake_fingerprint: optional_reference_fingerprint_value(&self.intake),
            candidate_ref: optional_reference(&self.candidate),
            candidate_fingerprint: optional_reference_fingerprint_value(&self.candidate),
            approval_ref: optional_reference(&self.approval),
            approval_fingerprint: optional_reference_fingerprint_value(&self.approval),
            promotion_ref: optional_reference(&self.promotion),
            promotion_fingerprint: optional_reference_fingerprint_value(&self.promotion),
            changed_paths: bounded_strings(&self.changed_paths),
            refusal: Some(self.refusal.to_refusal()),
            next_actions: bounded_strings(&self.next_actions),
            invocation_failure: None,
            causal_disposition: Some(adapter_cause_from_use_case(self.cause)),
            public_effects: Vec::new(),
        }
    }
}

fn bounded_strings<const N: usize, const M: usize>(
    values: &BoundedVec<BoundedString<N>, M>,
) -> Vec<String> {
    values
        .as_slice()
        .iter()
        .map(|value| value.as_str().to_owned())
        .collect()
}

fn optional_string<const N: usize>(value: &Option<BoundedString<N>>) -> Option<String> {
    value.as_ref().map(|value| value.as_str().to_owned())
}

fn optional_fingerprint(value: &Option<Fingerprint>) -> Option<String> {
    value.as_ref().map(|value| value.as_str().to_owned())
}

fn optional_reference(value: &Option<ReferenceFingerprintV1>) -> Option<String> {
    value
        .as_ref()
        .map(|value| value.reference.as_str().to_owned())
}

fn optional_reference_fingerprint_value(value: &Option<ReferenceFingerprintV1>) -> Option<String> {
    value
        .as_ref()
        .map(|value| value.fingerprint.as_str().to_owned())
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "result_kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum AuthorProjectContextResultV1 {
    Author {
        input_mode: BoundedString<255>,
        input_source: BoundedString<4096>,
        canonical_path: BoundedString<4096>,
        bytes_written: SafeU64,
        source_fingerprint: Fingerprint,
        rendered_output_fingerprint: Fingerprint,
        rendered_media_type: BoundedString<255>,
    },
    Validate {
        input_mode: BoundedString<255>,
        input_source: BoundedString<4096>,
    },
}

impl AuthorProjectContextResultV1 {
    pub fn author(
        input_mode: &str,
        input_source: &str,
        result: &AuthorProjectContextResult,
    ) -> Result<Self, ContractError> {
        Ok(Self::Author {
            input_mode: bounded(input_mode)?,
            input_source: bounded(input_source)?,
            canonical_path: bounded(&result.canonical_repo_relative_path)?,
            bytes_written: SafeU64::new(
                u64::try_from(result.bytes_written).map_err(|_| ContractError::InvalidNumber)?,
            )?,
            source_fingerprint: Fingerprint::parse(&result.source_fingerprint)?,
            rendered_output_fingerprint: Fingerprint::parse(&result.rendered_output_fingerprint)?,
            rendered_media_type: bounded(result.rendered_media_type)?,
        })
    }

    pub fn validate(input_mode: &str, input_source: &str) -> Result<Self, ContractError> {
        Ok(Self::Validate {
            input_mode: bounded(input_mode)?,
            input_source: bounded(input_source)?,
        })
    }

    fn to_outcome(&self) -> ProjectContextCommandOutcome {
        match self {
            Self::Author {
                input_mode,
                input_source,
                canonical_path,
                bytes_written,
                source_fingerprint,
                rendered_output_fingerprint,
                rendered_media_type,
            } => ProjectContextCommandOutcome::Author {
                input_mode: input_mode.as_str().to_owned(),
                input_source: input_source.as_str().to_owned(),
                result: Ok(AuthorProjectContextResult {
                    canonical_repo_relative_path: canonical_path.as_str().to_owned(),
                    bytes_written: usize::try_from(bytes_written.get()).unwrap_or(usize::MAX),
                    source_fingerprint: source_fingerprint.as_str().to_owned(),
                    rendered_output_fingerprint: rendered_output_fingerprint.as_str().to_owned(),
                    rendered_media_type: match rendered_media_type.as_str() {
                        "text/markdown" => "text/markdown",
                        _ => "application/octet-stream",
                    },
                    public_effects: Vec::new(),
                }),
            },
            Self::Validate {
                input_mode,
                input_source,
            } => ProjectContextCommandOutcome::Validate {
                input_mode: input_mode.as_str().to_owned(),
                input_source: input_source.as_str().to_owned(),
                result: Ok(()),
            },
        }
    }
}

closed_enum!(AuthorProjectContextOperationV1 { Author, Validate });
closed_enum!(Packet3EffectDispositionV1 {
    CreatedIncomplete,
    InstalledDurabilityUnconfirmed,
    DurableCreated,
});

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct Packet3EffectConditionV1 {
    target_ref: BoundedString<4096>,
    content_fingerprint: Option<Fingerprint>,
    disposition: Packet3EffectDispositionV1,
}
closed_enum!(AuthorProjectContextRefusalKindV1 {
    InvalidRequest,
    MissingSystemRoot,
    InvalidSystemRoot,
    ShippedContractUnavailable,
    SelectedDecisionMissing,
    SystemRootInspectionFailed,
    UnsafeSystemRoot,
    CanonicalInspectionFailed,
    MalformedStructuredInput,
    IncompleteStructuredInput,
    ExistingCanonicalTruth,
    MutationRefused,
    MutationSafetyRefused,
    MutationIoFailed,
    UnsupportedPlatformStrictMutation,
});

impl From<AuthorProjectContextRefusalKind> for AuthorProjectContextRefusalKindV1 {
    fn from(value: AuthorProjectContextRefusalKind) -> Self {
        match value {
            AuthorProjectContextRefusalKind::InvalidRequest => Self::InvalidRequest,
            AuthorProjectContextRefusalKind::MissingSystemRoot => Self::MissingSystemRoot,
            AuthorProjectContextRefusalKind::InvalidSystemRoot => Self::InvalidSystemRoot,
            AuthorProjectContextRefusalKind::ShippedContractUnavailable => {
                Self::ShippedContractUnavailable
            }
            AuthorProjectContextRefusalKind::SelectedDecisionMissing => {
                Self::SelectedDecisionMissing
            }
            AuthorProjectContextRefusalKind::SystemRootInspectionFailed => {
                Self::SystemRootInspectionFailed
            }
            AuthorProjectContextRefusalKind::UnsafeSystemRoot => Self::UnsafeSystemRoot,
            AuthorProjectContextRefusalKind::CanonicalInspectionFailed => {
                Self::CanonicalInspectionFailed
            }
            AuthorProjectContextRefusalKind::MalformedStructuredInput => {
                Self::MalformedStructuredInput
            }
            AuthorProjectContextRefusalKind::IncompleteStructuredInput => {
                Self::IncompleteStructuredInput
            }
            AuthorProjectContextRefusalKind::ExistingCanonicalTruth => Self::ExistingCanonicalTruth,
            AuthorProjectContextRefusalKind::MutationRefused => Self::MutationRefused,
            AuthorProjectContextRefusalKind::MutationSafetyRefused => Self::MutationSafetyRefused,
            AuthorProjectContextRefusalKind::MutationIoFailed => Self::MutationIoFailed,
            AuthorProjectContextRefusalKind::UnsupportedPlatformStrictMutation => {
                Self::UnsupportedPlatformStrictMutation
            }
        }
    }
}

impl From<AuthorProjectContextRefusalKindV1> for AuthorProjectContextRefusalKind {
    fn from(value: AuthorProjectContextRefusalKindV1) -> Self {
        match value {
            AuthorProjectContextRefusalKindV1::InvalidRequest => Self::InvalidRequest,
            AuthorProjectContextRefusalKindV1::MissingSystemRoot => Self::MissingSystemRoot,
            AuthorProjectContextRefusalKindV1::InvalidSystemRoot => Self::InvalidSystemRoot,
            AuthorProjectContextRefusalKindV1::ShippedContractUnavailable => {
                Self::ShippedContractUnavailable
            }
            AuthorProjectContextRefusalKindV1::SelectedDecisionMissing => {
                Self::SelectedDecisionMissing
            }
            AuthorProjectContextRefusalKindV1::SystemRootInspectionFailed => {
                Self::SystemRootInspectionFailed
            }
            AuthorProjectContextRefusalKindV1::UnsafeSystemRoot => Self::UnsafeSystemRoot,
            AuthorProjectContextRefusalKindV1::CanonicalInspectionFailed => {
                Self::CanonicalInspectionFailed
            }
            AuthorProjectContextRefusalKindV1::MalformedStructuredInput => {
                Self::MalformedStructuredInput
            }
            AuthorProjectContextRefusalKindV1::IncompleteStructuredInput => {
                Self::IncompleteStructuredInput
            }
            AuthorProjectContextRefusalKindV1::ExistingCanonicalTruth => {
                Self::ExistingCanonicalTruth
            }
            AuthorProjectContextRefusalKindV1::MutationRefused => Self::MutationRefused,
            AuthorProjectContextRefusalKindV1::MutationSafetyRefused => Self::MutationSafetyRefused,
            AuthorProjectContextRefusalKindV1::MutationIoFailed => Self::MutationIoFailed,
            AuthorProjectContextRefusalKindV1::UnsupportedPlatformStrictMutation => {
                Self::UnsupportedPlatformStrictMutation
            }
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AuthorProjectContextDetailsV1 {
    cause: UseCaseCauseV1,
    operation: AuthorProjectContextOperationV1,
    input_mode: BoundedString<255>,
    input_source: BoundedString<4096>,
    refusal_kind: AuthorProjectContextRefusalKindV1,
    summary: BoundedString<4096>,
    broken_subject: BoundedString<4096>,
    next_safe_action: BoundedString<4096>,
}

impl AuthorProjectContextDetailsV1 {
    fn from_outcome(outcome: &ProjectContextCommandOutcome) -> Result<Self, ContractError> {
        let (operation, input_mode, input_source, refusal) = match outcome {
            ProjectContextCommandOutcome::Author {
                input_mode,
                input_source,
                result: Err(refusal),
            } => (
                AuthorProjectContextOperationV1::Author,
                input_mode,
                input_source,
                refusal,
            ),
            ProjectContextCommandOutcome::Validate {
                input_mode,
                input_source,
                result: Err(refusal),
            } => (
                AuthorProjectContextOperationV1::Validate,
                input_mode,
                input_source,
                refusal,
            ),
            _ => return Err(ContractError::InvalidInvariant),
        };
        Ok(Self {
            cause: adapter_cause(refusal.causal_disposition),
            operation,
            input_mode: bounded(input_mode)?,
            input_source: bounded(input_source)?,
            refusal_kind: refusal.kind.into(),
            summary: bounded(&refusal.summary)?,
            broken_subject: bounded(&refusal.broken_subject)?,
            next_safe_action: bounded(&refusal.next_safe_action)?,
        })
    }

    fn to_outcome(&self) -> ProjectContextCommandOutcome {
        let refusal = AuthorProjectContextRefusal {
            kind: self.refusal_kind.into(),
            summary: self.summary.as_str().to_owned(),
            broken_subject: self.broken_subject.as_str().to_owned(),
            next_safe_action: self.next_safe_action.as_str().to_owned(),
            causal_disposition: adapter_cause_from_use_case(self.cause),
            public_effects: Vec::new(),
        };
        match self.operation {
            AuthorProjectContextOperationV1::Author => ProjectContextCommandOutcome::Author {
                input_mode: self.input_mode.as_str().to_owned(),
                input_source: self.input_source.as_str().to_owned(),
                result: Err(refusal),
            },
            AuthorProjectContextOperationV1::Validate => ProjectContextCommandOutcome::Validate {
                input_mode: self.input_mode.as_str().to_owned(),
                input_source: self.input_source.as_str().to_owned(),
                result: Err(refusal),
            },
        }
    }
}

fn adapter_cause(value: crate::charter_product::AdapterCausalDisposition) -> UseCaseCauseV1 {
    match value {
        crate::charter_product::AdapterCausalDisposition::Prerequisite => {
            UseCaseCauseV1::Prerequisite
        }
        crate::charter_product::AdapterCausalDisposition::CallerRefusal => {
            UseCaseCauseV1::CallerRefusal
        }
        crate::charter_product::AdapterCausalDisposition::ImplementationFailure => {
            UseCaseCauseV1::ImplementationFailure
        }
    }
}

fn adapter_cause_from_use_case(
    value: UseCaseCauseV1,
) -> crate::charter_product::AdapterCausalDisposition {
    match value {
        UseCaseCauseV1::Prerequisite => {
            crate::charter_product::AdapterCausalDisposition::Prerequisite
        }
        UseCaseCauseV1::CallerRefusal => {
            crate::charter_product::AdapterCausalDisposition::CallerRefusal
        }
        UseCaseCauseV1::ImplementationFailure => {
            crate::charter_product::AdapterCausalDisposition::ImplementationFailure
        }
    }
}

closed_enum!(ApproverAdministrationOperationV1 {
    Bootstrap,
    AddCredential,
    RevokeCredential,
    UpdateMapping,
});

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ApproverAdministrationResultV1 {
    operation: ApproverAdministrationOperationV1,
    operation_id: BoundedString<255>,
    repository_identity_fingerprint: Fingerprint,
    prior_registry_state: Option<ReferenceFingerprintV1>,
    result_registry_state: Option<ReferenceFingerprintV1>,
    transition: Option<ReferenceFingerprintV1>,
    registration: Option<ReferenceFingerprintV1>,
    assertion: Option<ReferenceFingerprintV1>,
    changed_paths: BoundedVec<BoundedString<4096>, 128>,
    next_actions: BoundedVec<BoundedString<1024>, 128>,
}

impl ApproverAdministrationResultV1 {
    pub fn from_result(result: &ApproverOperationResult) -> Result<Self, ContractError> {
        if result.status != AuthorOperationStatus::Succeeded || result.refusal.is_some() {
            return Err(ContractError::InvalidInvariant);
        }
        let fields = match result.legacy_projection.as_ref() {
            Some(ApproverOperationProjection::Admin(fields)) => fields,
            _ => return Err(ContractError::InvalidInvariant),
        };
        Ok(Self {
            operation: result.operation.into(),
            operation_id: bounded(&fields.operation_id)?,
            repository_identity_fingerprint: Fingerprint::parse(
                &fields.repository_identity_fingerprint,
            )?,
            prior_registry_state: optional_reference_fingerprint(
                fields.prior_registry_state_ref.as_deref(),
                fields.prior_registry_state_fingerprint.as_deref(),
            )?,
            result_registry_state: optional_reference_fingerprint(
                fields.result_registry_state_ref.as_deref(),
                fields.result_registry_state_fingerprint.as_deref(),
            )?,
            transition: optional_reference_fingerprint(
                fields.transition_ref.as_deref(),
                fields.transition_fingerprint.as_deref(),
            )?,
            registration: optional_reference_fingerprint(
                fields.registration_ref.as_deref(),
                fields.registration_fingerprint.as_deref(),
            )?,
            assertion: optional_reference_fingerprint(
                fields.assertion_ref.as_deref(),
                fields.assertion_fingerprint.as_deref(),
            )?,
            changed_paths: bounded_vec(&result.changed_paths)?,
            next_actions: bounded_vec(&result.next_actions)?,
        })
    }

    fn to_result(&self) -> ApproverOperationResult {
        let fields = ApproverAdminLegacyFields {
            operation_id: self.operation_id.as_str().to_owned(),
            repository_identity_fingerprint: self
                .repository_identity_fingerprint
                .as_str()
                .to_owned(),
            prior_registry_state_ref: optional_reference(&self.prior_registry_state),
            prior_registry_state_fingerprint: optional_reference_fingerprint_value(
                &self.prior_registry_state,
            ),
            result_registry_state_ref: optional_reference(&self.result_registry_state),
            result_registry_state_fingerprint: optional_reference_fingerprint_value(
                &self.result_registry_state,
            ),
            transition_ref: optional_reference(&self.transition),
            transition_fingerprint: optional_reference_fingerprint_value(&self.transition),
            registration_ref: optional_reference(&self.registration),
            registration_fingerprint: optional_reference_fingerprint_value(&self.registration),
            assertion_ref: optional_reference(&self.assertion),
            assertion_fingerprint: optional_reference_fingerprint_value(&self.assertion),
        };
        ApproverOperationResult {
            schema_id: "handbook.approver-admin-result".to_owned(),
            schema_version: "1.0".to_owned(),
            operation: self.operation.into(),
            status: AuthorOperationStatus::Succeeded,
            changed_paths: bounded_strings(&self.changed_paths),
            refusal: None,
            next_actions: bounded_strings(&self.next_actions),
            legacy_projection: Some(ApproverOperationProjection::Admin(fields)),
            causal_disposition: None,
            public_effects: Vec::new(),
        }
    }
}

map_enum!(ApproverOperation => ApproverAdministrationOperationV1 {
    Bootstrap, AddCredential, RevokeCredential, UpdateMapping
});

impl From<ApproverAdministrationOperationV1> for ApproverOperation {
    fn from(value: ApproverAdministrationOperationV1) -> Self {
        match value {
            ApproverAdministrationOperationV1::Bootstrap => Self::Bootstrap,
            ApproverAdministrationOperationV1::AddCredential => Self::AddCredential,
            ApproverAdministrationOperationV1::RevokeCredential => Self::RevokeCredential,
            ApproverAdministrationOperationV1::UpdateMapping => Self::UpdateMapping,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct ApproverAdminLegacyDetailsV1 {
    operation_id: BoundedString<255>,
    repository_identity_fingerprint: Fingerprint,
    prior_registry_state: Option<ReferenceFingerprintV1>,
    result_registry_state: Option<ReferenceFingerprintV1>,
    transition: Option<ReferenceFingerprintV1>,
    registration: Option<ReferenceFingerprintV1>,
    assertion: Option<ReferenceFingerprintV1>,
}

impl ApproverAdminLegacyDetailsV1 {
    fn from_fields(fields: &ApproverAdminLegacyFields) -> Result<Self, ContractError> {
        Ok(Self {
            operation_id: bounded(&fields.operation_id)?,
            repository_identity_fingerprint: Fingerprint::parse(
                &fields.repository_identity_fingerprint,
            )?,
            prior_registry_state: optional_reference_fingerprint(
                fields.prior_registry_state_ref.as_deref(),
                fields.prior_registry_state_fingerprint.as_deref(),
            )?,
            result_registry_state: optional_reference_fingerprint(
                fields.result_registry_state_ref.as_deref(),
                fields.result_registry_state_fingerprint.as_deref(),
            )?,
            transition: optional_reference_fingerprint(
                fields.transition_ref.as_deref(),
                fields.transition_fingerprint.as_deref(),
            )?,
            registration: optional_reference_fingerprint(
                fields.registration_ref.as_deref(),
                fields.registration_fingerprint.as_deref(),
            )?,
            assertion: optional_reference_fingerprint(
                fields.assertion_ref.as_deref(),
                fields.assertion_fingerprint.as_deref(),
            )?,
        })
    }

    fn to_fields(&self) -> ApproverAdminLegacyFields {
        ApproverAdminLegacyFields {
            operation_id: self.operation_id.as_str().to_owned(),
            repository_identity_fingerprint: self
                .repository_identity_fingerprint
                .as_str()
                .to_owned(),
            prior_registry_state_ref: optional_reference(&self.prior_registry_state),
            prior_registry_state_fingerprint: optional_reference_fingerprint_value(
                &self.prior_registry_state,
            ),
            result_registry_state_ref: optional_reference(&self.result_registry_state),
            result_registry_state_fingerprint: optional_reference_fingerprint_value(
                &self.result_registry_state,
            ),
            transition_ref: optional_reference(&self.transition),
            transition_fingerprint: optional_reference_fingerprint_value(&self.transition),
            registration_ref: optional_reference(&self.registration),
            registration_fingerprint: optional_reference_fingerprint_value(&self.registration),
            assertion_ref: optional_reference(&self.assertion),
            assertion_fingerprint: optional_reference_fingerprint_value(&self.assertion),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(
    tag = "projection_kind",
    rename_all = "snake_case",
    deny_unknown_fields
)]
enum ApproverLegacyDetailsV1 {
    Admin {
        fields: ApproverAdminLegacyDetailsV1,
    },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ApproverAdministrationDetailsV1 {
    cause: UseCaseCauseV1,
    operation: ApproverAdministrationOperationV1,
    changed_paths: BoundedVec<BoundedString<4096>, 128>,
    refusal: AuthorOperationRefusalV1,
    next_actions: BoundedVec<BoundedString<1024>, 128>,
    legacy_projection: Option<ApproverLegacyDetailsV1>,
}

impl ApproverAdministrationDetailsV1 {
    fn from_result(result: &ApproverOperationResult) -> Result<Self, ContractError> {
        if result.status != AuthorOperationStatus::Refused {
            return Err(ContractError::InvalidInvariant);
        }
        let refusal = result
            .refusal
            .as_ref()
            .ok_or(ContractError::InvalidInvariant)?;
        let legacy_projection = match result.legacy_projection.as_ref() {
            Some(ApproverOperationProjection::Admin(fields)) => {
                Some(ApproverLegacyDetailsV1::Admin {
                    fields: ApproverAdminLegacyDetailsV1::from_fields(fields)?,
                })
            }
            Some(ApproverOperationProjection::InvocationFailure(_)) => {
                return Err(ContractError::InvalidInvariant);
            }
            None => None,
        };
        Ok(Self {
            cause: result
                .causal_disposition
                .map(adapter_cause)
                .ok_or(ContractError::InvalidInvariant)?,
            operation: result.operation.into(),
            changed_paths: bounded_vec(&result.changed_paths)?,
            refusal: AuthorOperationRefusalV1::from_refusal(refusal)?,
            next_actions: bounded_vec(&result.next_actions)?,
            legacy_projection,
        })
    }

    fn to_result(&self) -> ApproverOperationResult {
        let legacy_projection =
            self.legacy_projection
                .as_ref()
                .map(|projection| match projection {
                    ApproverLegacyDetailsV1::Admin { fields } => {
                        ApproverOperationProjection::Admin(fields.to_fields())
                    }
                });
        let (schema_id, schema_version) = match &legacy_projection {
            Some(ApproverOperationProjection::Admin(_)) => (
                "handbook.approver-admin-result".to_owned(),
                "1.0".to_owned(),
            ),
            Some(ApproverOperationProjection::InvocationFailure(failure)) => {
                (failure.schema_id.clone(), failure.schema_version.clone())
            }
            None => (
                "handbook.approver-admin-adapter-result".to_owned(),
                "1.0".to_owned(),
            ),
        };
        ApproverOperationResult {
            schema_id,
            schema_version,
            operation: self.operation.into(),
            status: AuthorOperationStatus::Refused,
            changed_paths: bounded_strings(&self.changed_paths),
            refusal: Some(self.refusal.to_refusal()),
            next_actions: bounded_strings(&self.next_actions),
            legacy_projection,
            causal_disposition: Some(match self.cause {
                UseCaseCauseV1::Prerequisite => {
                    crate::charter_product::AdapterCausalDisposition::Prerequisite
                }
                UseCaseCauseV1::CallerRefusal => {
                    crate::charter_product::AdapterCausalDisposition::CallerRefusal
                }
                UseCaseCauseV1::ImplementationFailure => {
                    crate::charter_product::AdapterCausalDisposition::ImplementationFailure
                }
            }),
            public_effects: Vec::new(),
        }
    }
}

fn packet3_disposition(cause: UseCaseCauseV1) -> (TerminalStatus, ProblemCategory, RetryDirective) {
    match cause {
        UseCaseCauseV1::Prerequisite => (
            TerminalStatus::Blocked,
            ProblemCategory::Prerequisite,
            RetryDirective::AfterRecheck,
        ),
        UseCaseCauseV1::CallerRefusal => (
            TerminalStatus::Refused,
            ProblemCategory::Precondition,
            RetryDirective::AfterRequestChange,
        ),
        UseCaseCauseV1::ImplementationFailure => (
            TerminalStatus::Error,
            ProblemCategory::Implementation,
            RetryDirective::Transient,
        ),
    }
}

fn admit_packet3_problem(problem: &Problem, cause: UseCaseCauseV1) -> Result<(), ContractError> {
    let (status, category, retry) = packet3_disposition(cause);
    if problem.category() != category
        || problem.retry() != retry
        || (status == TerminalStatus::Blocked) != problem.recheck().is_some()
        || (status == TerminalStatus::Refused) != problem.rule().is_some()
        || (status == TerminalStatus::Error) != problem.correlation_ref().is_some()
    {
        return Err(ContractError::InvalidInvariant);
    }
    Ok(())
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PipelineRouteRunV1 {
    runner: Option<BoundedString<4096>>,
    profile: Option<BoundedString<4096>>,
    repository_root: Option<BoundedString<4096>>,
}

impl PipelineRouteRunV1 {
    fn from_view(view: &PipelineRouteRunView) -> Result<Self, ContractError> {
        Ok(Self {
            runner: optional_bounded(view.runner.as_deref())?,
            profile: optional_bounded(view.profile.as_deref())?,
            repository_root: optional_bounded(view.repository_root.as_deref())?,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PipelineRoutingEntryV1 {
    variable: BoundedString<255>,
    value: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PipelineRouteStateV1 {
    revision: SafeU64,
    routing: BoundedVec<PipelineRoutingEntryV1, 128>,
    charter_ref: Option<BoundedString<4096>>,
    project_context_ref: Option<BoundedString<4096>>,
    run: PipelineRouteRunV1,
}

impl PipelineRouteStateV1 {
    fn from_view(view: &PipelineRouteStateView) -> Result<Self, ContractError> {
        Ok(Self {
            revision: SafeU64::new(view.revision)?,
            routing: BoundedVec::new(
                view.routing
                    .iter()
                    .map(|(variable, value)| {
                        Ok(PipelineRoutingEntryV1 {
                            variable: bounded(variable)?,
                            value: *value,
                        })
                    })
                    .collect::<Result<Vec<_>, ContractError>>()?,
            )?,
            charter_ref: optional_bounded(view.charter_ref.as_deref())?,
            project_context_ref: optional_bounded(view.project_context_ref.as_deref())?,
            run: PipelineRouteRunV1::from_view(&view.run)?,
        })
    }
}

closed_enum!(PipelineRouteStageStatusV1 {
    Active,
    Skipped,
    Blocked,
    Next
});

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "reason_kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum PipelineRouteStageReasonV1 {
    SkippedActivationFalse {
        unsatisfied_variables: BoundedVec<BoundedString<255>, 128>,
    },
    NextMissingRouteVariables {
        missing_variables: BoundedVec<BoundedString<255>, 128>,
    },
    BlockedByUnresolvedStage {
        upstream_stage_id: BoundedString<255>,
        upstream_status: PipelineRouteStageStatusV1,
    },
}

impl PipelineRouteStageReasonV1 {
    fn from_reason(reason: &PipelineRouteStageReason) -> Result<Self, ContractError> {
        Ok(match reason {
            PipelineRouteStageReason::SkippedActivationFalse {
                unsatisfied_variables,
            } => Self::SkippedActivationFalse {
                unsatisfied_variables: bounded_vec(unsatisfied_variables)?,
            },
            PipelineRouteStageReason::NextMissingRouteVariables { missing_variables } => {
                Self::NextMissingRouteVariables {
                    missing_variables: bounded_vec(missing_variables)?,
                }
            }
            PipelineRouteStageReason::BlockedByUnresolvedStage {
                upstream_stage_id,
                upstream_status,
            } => Self::BlockedByUnresolvedStage {
                upstream_stage_id: bounded(upstream_stage_id)?,
                upstream_status: (*upstream_status).into(),
            },
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PipelineResolvedStageV1 {
    stage_id: BoundedString<255>,
    status: PipelineRouteStageStatusV1,
    reason: Option<PipelineRouteStageReasonV1>,
}

impl PipelineResolvedStageV1 {
    fn from_view(view: &PipelineResolvedStageView) -> Result<Self, ContractError> {
        Ok(Self {
            stage_id: bounded(&view.stage_id)?,
            status: view.status.into(),
            reason: view
                .reason
                .as_ref()
                .map(PipelineRouteStageReasonV1::from_reason)
                .transpose()?,
        })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PipelineRouteResolveAndPersistResultV1 {
    pipeline_id: BoundedString<255>,
    state: PipelineRouteStateV1,
    effective_run: PipelineRouteRunV1,
    stages: BoundedVec<PipelineResolvedStageV1, 128>,
}

impl PipelineRouteResolveAndPersistResultV1 {
    pub fn from_outcome(outcome: &PipelineResolveOutcome) -> Result<Self, ContractError> {
        Ok(Self {
            pipeline_id: bounded(&outcome.pipeline_id)?,
            state: PipelineRouteStateV1::from_view(&outcome.state)?,
            effective_run: PipelineRouteRunV1::from_view(&outcome.effective_run)?,
            stages: BoundedVec::new(
                outcome
                    .stages
                    .iter()
                    .map(PipelineResolvedStageV1::from_view)
                    .collect::<Result<Vec<_>, ContractError>>()?,
            )?,
        })
    }

    fn to_outcome(&self) -> PipelineResolveOutcome {
        PipelineResolveOutcome {
            pipeline_id: self.pipeline_id.as_str().to_owned(),
            state: self.state.to_view(),
            effective_run: self.effective_run.to_view(),
            stages: self
                .stages
                .as_slice()
                .iter()
                .map(PipelineResolvedStageV1::to_view)
                .collect(),
        }
    }
}

impl PipelineRouteRunV1 {
    fn to_view(&self) -> PipelineRouteRunView {
        PipelineRouteRunView {
            runner: self.runner.as_ref().map(|value| value.as_str().to_owned()),
            profile: self.profile.as_ref().map(|value| value.as_str().to_owned()),
            repository_root: self
                .repository_root
                .as_ref()
                .map(|value| value.as_str().to_owned()),
        }
    }
}

impl PipelineRouteStateV1 {
    fn to_view(&self) -> PipelineRouteStateView {
        PipelineRouteStateView {
            revision: self.revision.get(),
            routing: self
                .routing
                .as_slice()
                .iter()
                .map(|entry| (entry.variable.as_str().to_owned(), entry.value))
                .collect(),
            charter_ref: self
                .charter_ref
                .as_ref()
                .map(|value| value.as_str().to_owned()),
            project_context_ref: self
                .project_context_ref
                .as_ref()
                .map(|value| value.as_str().to_owned()),
            run: self.run.to_view(),
        }
    }
}

impl PipelineResolvedStageV1 {
    fn to_view(&self) -> PipelineResolvedStageView {
        PipelineResolvedStageView {
            stage_id: self.stage_id.as_str().to_owned(),
            status: self.status.into(),
            reason: self
                .reason
                .as_ref()
                .map(PipelineRouteStageReasonV1::to_reason),
        }
    }
}

impl PipelineRouteStageReasonV1 {
    fn to_reason(&self) -> PipelineRouteStageReason {
        match self {
            Self::SkippedActivationFalse {
                unsatisfied_variables,
            } => PipelineRouteStageReason::SkippedActivationFalse {
                unsatisfied_variables: unsatisfied_variables
                    .as_slice()
                    .iter()
                    .map(|value| value.as_str().to_owned())
                    .collect(),
            },
            Self::NextMissingRouteVariables { missing_variables } => {
                PipelineRouteStageReason::NextMissingRouteVariables {
                    missing_variables: missing_variables
                        .as_slice()
                        .iter()
                        .map(|value| value.as_str().to_owned())
                        .collect(),
                }
            }
            Self::BlockedByUnresolvedStage {
                upstream_stage_id,
                upstream_status,
            } => PipelineRouteStageReason::BlockedByUnresolvedStage {
                upstream_stage_id: upstream_stage_id.as_str().to_owned(),
                upstream_status: (*upstream_status).into(),
            },
        }
    }
}

map_enum!(PipelineRouteStageStatusV1 => PipelineRouteStageStatus {
    Active, Skipped, Blocked, Next
});

closed_enum!(PipelineRouteResolveCauseV1 {
    Catalog,
    UnsupportedSelector,
    AmbiguousSelector,
    UnknownSelector,
    Definition,
    RouteStateRead,
    RouteVariables,
    RouteResolution,
    RouteBasisBuild,
    RouteBasisPersistenceMalformed,
    RouteBasisRevisionConflict,
    RouteBasisPersistence
});

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PipelineRouteResolveDetailsV1 {
    cause: PipelineRouteResolveCauseV1,
    reason: BoundedString<8192>,
    selector: Option<BoundedString<4096>>,
    matches: BoundedVec<BoundedString<4096>, 128>,
    expected_revision: Option<SafeU64>,
    actual_revision: Option<SafeU64>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PipelineRouteBasisWriteConditionV1 {
    pipeline_id: BoundedString<255>,
    state_revision: SafeU64,
    route_basis_fingerprint: Fingerprint,
}

impl PipelineRouteResolveDetailsV1 {
    fn validate(&self) -> Result<(), ContractError> {
        let selector_cause = matches!(
            self.cause,
            PipelineRouteResolveCauseV1::UnsupportedSelector
                | PipelineRouteResolveCauseV1::AmbiguousSelector
                | PipelineRouteResolveCauseV1::UnknownSelector
        );
        if selector_cause != self.selector.is_some()
            || (!matches!(self.cause, PipelineRouteResolveCauseV1::AmbiguousSelector)
                && !self.matches.as_slice().is_empty())
            || (matches!(self.cause, PipelineRouteResolveCauseV1::AmbiguousSelector)
                && self.matches.as_slice().is_empty())
        {
            return Err(ContractError::InvalidInvariant);
        }
        let revision_conflict = matches!(
            self.cause,
            PipelineRouteResolveCauseV1::RouteBasisRevisionConflict
        );
        if revision_conflict != (self.expected_revision.is_some() && self.actual_revision.is_some())
            || (!revision_conflict
                && (self.expected_revision.is_some() || self.actual_revision.is_some()))
        {
            return Err(ContractError::InvalidInvariant);
        }
        Ok(())
    }
}

map_enum!(PipelineRouteStageStatus => PipelineRouteStageStatusV1 {
    Active, Skipped, Blocked, Next
});

closed_enum!(RepositoryDoctorFailureKindV1 {
    ProfileResolution,
    ProfileDecision,
});

closed_enum!(RepositoryDoctorFailureReasonV1 {
    ShippedProfileUnavailable,
    SelectedProfileDecisionInvalid,
});

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "details_kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum RepositoryDoctorDetailsV1 {
    Readiness {
        result: Box<RepositoryDoctorResultV1>,
    },
    DoctorFailure {
        failure_kind: RepositoryDoctorFailureKindV1,
        reason: RepositoryDoctorFailureReasonV1,
    },
}

closed_enum!(TransportFailureKind {
    Repository,
    Acquisition,
    Decoding,
    Framing,
    Serialization,
    Emission,
    Adapter,
});

closed_enum!(AuthorInputTransportRouteV1 {
    Charter,
    ProjectContext,
});

closed_enum!(TransportIoErrorKindV1 {
    NotFound,
    PermissionDenied,
    ConnectionRefused,
    ConnectionReset,
    ConnectionAborted,
    NotConnected,
    AddrInUse,
    AddrNotAvailable,
    BrokenPipe,
    AlreadyExists,
    WouldBlock,
    InvalidInput,
    InvalidData,
    TimedOut,
    WriteZero,
    Interrupted,
    Unsupported,
    UnexpectedEof,
    OutOfMemory,
    Other,
});

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TransportIoErrorV1 {
    kind: TransportIoErrorKindV1,
    raw_os_error: Option<i32>,
}

impl TransportIoErrorV1 {
    pub fn from_io_error(error: &io::Error) -> Self {
        Self {
            kind: TransportIoErrorKindV1::from(error.kind()),
            raw_os_error: error.raw_os_error(),
        }
    }

    pub fn kind(&self) -> TransportIoErrorKindV1 {
        self.kind
    }

    pub fn raw_os_error(&self) -> Option<i32> {
        self.raw_os_error
    }

    pub fn display_message(&self) -> String {
        match self.raw_os_error {
            Some(code) => io::Error::from_raw_os_error(code).to_string(),
            None => io::Error::from(io::ErrorKind::from(self.kind)).to_string(),
        }
    }

    pub fn is_caller_acquisition(&self) -> bool {
        matches!(
            self.kind,
            TransportIoErrorKindV1::NotFound
                | TransportIoErrorKindV1::InvalidInput
                | TransportIoErrorKindV1::InvalidData
        )
    }
}

impl From<io::ErrorKind> for TransportIoErrorKindV1 {
    fn from(kind: io::ErrorKind) -> Self {
        match kind {
            io::ErrorKind::NotFound => Self::NotFound,
            io::ErrorKind::PermissionDenied => Self::PermissionDenied,
            io::ErrorKind::ConnectionRefused => Self::ConnectionRefused,
            io::ErrorKind::ConnectionReset => Self::ConnectionReset,
            io::ErrorKind::ConnectionAborted => Self::ConnectionAborted,
            io::ErrorKind::NotConnected => Self::NotConnected,
            io::ErrorKind::AddrInUse => Self::AddrInUse,
            io::ErrorKind::AddrNotAvailable => Self::AddrNotAvailable,
            io::ErrorKind::BrokenPipe => Self::BrokenPipe,
            io::ErrorKind::AlreadyExists => Self::AlreadyExists,
            io::ErrorKind::WouldBlock => Self::WouldBlock,
            io::ErrorKind::InvalidInput => Self::InvalidInput,
            io::ErrorKind::InvalidData => Self::InvalidData,
            io::ErrorKind::TimedOut => Self::TimedOut,
            io::ErrorKind::WriteZero => Self::WriteZero,
            io::ErrorKind::Interrupted => Self::Interrupted,
            io::ErrorKind::Unsupported => Self::Unsupported,
            io::ErrorKind::UnexpectedEof => Self::UnexpectedEof,
            io::ErrorKind::OutOfMemory => Self::OutOfMemory,
            _ => Self::Other,
        }
    }
}

impl From<TransportIoErrorKindV1> for io::ErrorKind {
    fn from(kind: TransportIoErrorKindV1) -> Self {
        match kind {
            TransportIoErrorKindV1::NotFound => Self::NotFound,
            TransportIoErrorKindV1::PermissionDenied => Self::PermissionDenied,
            TransportIoErrorKindV1::ConnectionRefused => Self::ConnectionRefused,
            TransportIoErrorKindV1::ConnectionReset => Self::ConnectionReset,
            TransportIoErrorKindV1::ConnectionAborted => Self::ConnectionAborted,
            TransportIoErrorKindV1::NotConnected => Self::NotConnected,
            TransportIoErrorKindV1::AddrInUse => Self::AddrInUse,
            TransportIoErrorKindV1::AddrNotAvailable => Self::AddrNotAvailable,
            TransportIoErrorKindV1::BrokenPipe => Self::BrokenPipe,
            TransportIoErrorKindV1::AlreadyExists => Self::AlreadyExists,
            TransportIoErrorKindV1::WouldBlock => Self::WouldBlock,
            TransportIoErrorKindV1::InvalidInput => Self::InvalidInput,
            TransportIoErrorKindV1::InvalidData => Self::InvalidData,
            TransportIoErrorKindV1::TimedOut => Self::TimedOut,
            TransportIoErrorKindV1::WriteZero => Self::WriteZero,
            TransportIoErrorKindV1::Interrupted => Self::Interrupted,
            TransportIoErrorKindV1::Unsupported => Self::Unsupported,
            TransportIoErrorKindV1::UnexpectedEof => Self::UnexpectedEof,
            TransportIoErrorKindV1::OutOfMemory => Self::OutOfMemory,
            TransportIoErrorKindV1::Other => Self::Other,
        }
    }
}

closed_enum!(AuthorWorkingDirectoryCommandV1 {
    CharterAuthor,
    CharterApprove,
    CharterPromote,
    CharterValidate,
    ProjectContextAuthor,
    ProjectContextValidate,
});

closed_enum!(RepositoryInvocationCommandV1 {
    CharterAuthor,
    CharterApprove,
    CharterPromote,
    CharterValidate,
    ApproverBootstrap,
    ApproverAddCredential,
    ApproverRevokeCredential,
    ApproverUpdateMapping,
});

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(
    tag = "presentation_kind",
    rename_all = "snake_case",
    deny_unknown_fields
)]
enum TransportPresentationV1 {
    AuthorInput {
        route: AuthorInputTransportRouteV1,
        input_source: BoundedString<65536>,
        error: Option<TransportIoErrorV1>,
    },
    WorkingDirectory {
        command: AuthorWorkingDirectoryCommandV1,
        error: TransportIoErrorV1,
    },
    RepositoryInvocation {
        command: RepositoryInvocationCommandV1,
        failure: Box<RepositoryInvocationFailureDetailsV1>,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TransportPresentationViewV1 {
    AuthorInput {
        route: AuthorInputTransportRouteV1,
        input_source: String,
        error: Option<TransportIoErrorV1>,
    },
    WorkingDirectory {
        command: AuthorWorkingDirectoryCommandV1,
        error: TransportIoErrorV1,
    },
    RepositoryInvocation {
        command: RepositoryInvocationCommandV1,
        failure: Box<RepositoryInvocationFailureView>,
    },
}

impl TransportPresentationV1 {
    fn to_view(&self, failure_kind: TransportFailureKind) -> TransportPresentationViewV1 {
        match self {
            Self::AuthorInput {
                route,
                input_source,
                error,
            } => TransportPresentationViewV1::AuthorInput {
                route: *route,
                input_source: input_source.as_str().to_owned(),
                error: error.clone(),
            },
            Self::WorkingDirectory { command, error } => {
                TransportPresentationViewV1::WorkingDirectory {
                    command: *command,
                    error: error.clone(),
                }
            }
            Self::RepositoryInvocation { command, failure } => {
                TransportPresentationViewV1::RepositoryInvocation {
                    command: *command,
                    failure: Box::new(failure.to_view(transport_cause(failure_kind))),
                }
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TransportFailureDetailsV1 {
    failure_kind: TransportFailureKind,
    reason_code: BoundedString<255>,
    #[serde(skip_serializing_if = "Option::is_none")]
    presentation: Option<TransportPresentationV1>,
}

impl<'de> Deserialize<'de> for TransportFailureDetailsV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Raw {
            failure_kind: TransportFailureKind,
            reason_code: BoundedString<255>,
            presentation: Option<TransportPresentationV1>,
        }
        let raw = Raw::deserialize(deserializer).map_err(|error| {
            D::Error::custom(classify_contract_error_message(&error.to_string()))
        })?;
        let bytes = raw.reason_code.as_str().as_bytes();
        if !bytes.first().is_some_and(u8::is_ascii_lowercase)
            || !bytes[1..].iter().all(|byte| {
                byte.is_ascii_lowercase()
                    || byte.is_ascii_digit()
                    || matches!(byte, b'.' | b'_' | b'-')
            })
        {
            return Err(D::Error::custom(ContractError::InvalidString));
        }
        Ok(Self {
            failure_kind: raw.failure_kind,
            reason_code: raw.reason_code,
            presentation: raw.presentation,
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum UseCaseKind {
    RepositorySetup,
    RepositoryDoctor,
    AuthorCharter,
    AuthorProjectContext,
    ApproverAdministration,
    PipelineRouteResolveAndPersist,
}

impl UseCaseKind {
    fn from_binding(value: &ExactBinding) -> Result<Self, ContractError> {
        match value.reference().as_str() {
            "handbook.use-case.repository-setup@1.0.0" => Ok(Self::RepositorySetup),
            DOCTOR_USE_CASE_REF => Ok(Self::RepositoryDoctor),
            "handbook.use-case.author-charter@1.0.0" => Ok(Self::AuthorCharter),
            "handbook.use-case.author-project-context@1.0.0" => Ok(Self::AuthorProjectContext),
            "handbook.use-case.approver-administration@1.0.0" => Ok(Self::ApproverAdministration),
            "handbook.use-case.pipeline-route-resolve-and-persist@1.0.0" => {
                Ok(Self::PipelineRouteResolveAndPersist)
            }
            _ => Err(ContractError::InvalidInvariant),
        }
    }

    fn result_ref(self) -> &'static str {
        match self {
            Self::RepositorySetup => "handbook.use-case.repository-setup-result@1.0.0",
            Self::RepositoryDoctor => DOCTOR_RESULT_REF,
            Self::AuthorCharter => "handbook.use-case.author-charter-result@1.0.0",
            Self::AuthorProjectContext => "handbook.use-case.author-project-context-result@1.0.0",
            Self::ApproverAdministration => {
                "handbook.use-case.approver-administration-result@1.0.0"
            }
            Self::PipelineRouteResolveAndPersist => {
                "handbook.use-case.pipeline-route-resolve-and-persist-result@1.0.0"
            }
        }
    }

    fn use_case_ref(self) -> &'static str {
        match self {
            Self::RepositorySetup => "handbook.use-case.repository-setup@1.0.0",
            Self::RepositoryDoctor => DOCTOR_USE_CASE_REF,
            Self::AuthorCharter => "handbook.use-case.author-charter@1.0.0",
            Self::AuthorProjectContext => "handbook.use-case.author-project-context@1.0.0",
            Self::ApproverAdministration => "handbook.use-case.approver-administration@1.0.0",
            Self::PipelineRouteResolveAndPersist => {
                "handbook.use-case.pipeline-route-resolve-and-persist@1.0.0"
            }
        }
    }

    fn admit_success(
        self,
        value: &ContractValue,
        limits: &ContractLimits,
    ) -> Result<(), ContractError> {
        match self {
            Self::RepositorySetup => {
                let result: RepositorySetupResultV1 = admit_contract_value(value, limits)?;
                if result.readiness != RepositoryReadinessStatusV1::Ready {
                    return Err(ContractError::InvalidInvariant);
                }
            }
            Self::RepositoryDoctor => {
                let result: RepositoryDoctorResultV1 = admit_contract_value(value, limits)?;
                if result.readiness != RepositoryReadinessStatusV1::Ready {
                    return Err(ContractError::InvalidInvariant);
                }
            }
            Self::AuthorCharter => {
                let _: AuthorCharterResultV1 = admit_contract_value(value, limits)?;
            }
            Self::AuthorProjectContext => {
                let _: AuthorProjectContextResultV1 = admit_contract_value(value, limits)?;
            }
            Self::ApproverAdministration => {
                let _: ApproverAdministrationResultV1 = admit_contract_value(value, limits)?;
            }
            Self::PipelineRouteResolveAndPersist => {
                let _: PipelineRouteResolveAndPersistResultV1 =
                    admit_contract_value(value, limits)?;
            }
        }
        Ok(())
    }

    fn admit_problem_details(
        self,
        problem: &Problem,
        limits: &ContractLimits,
    ) -> Result<(), ContractError> {
        match self {
            Self::RepositorySetup
                if problem.subject() == &binding(SETUP_USE_CASE_REF)?
                    && problem.details_schema() == &binding(SETUP_DETAILS_REF)?
                    && problem.recheck().is_none_or(|condition| {
                        condition.condition_schema() == problem.details_schema()
                            && condition.condition() == problem.details()
                    }) =>
            {
                let details: RepositorySetupDetailsV1 =
                    admit_contract_value(problem.details(), limits)?;
                match details {
                    RepositorySetupDetailsV1::Readiness { result } => {
                        if result.readiness == RepositoryReadinessStatusV1::Ready
                            || problem.problem_id() != problem.code()
                            || problem.code().as_str() != setup_readiness_code(result.readiness)?
                            || problem.category() != ProblemCategory::Prerequisite
                            || problem.retry() != RetryDirective::AfterRecheck
                            || problem.recheck().is_none()
                            || problem.rule().is_some()
                            || problem.correlation_ref().is_some()
                            || !problem.evidence().as_slice().is_empty()
                        {
                            return Err(ContractError::InvalidInvariant);
                        }
                    }
                    details @ RepositorySetupDetailsV1::SetupFailure { .. } => {
                        let error = details.to_error()?.ok_or(ContractError::InvalidInvariant)?;
                        let (status, category, retry, code) = setup_error_disposition(error.code());
                        if problem.problem_id() != problem.code()
                            || problem.code().as_str() != code
                            || problem.category() != category
                            || problem.retry() != retry
                            || !problem.evidence().as_slice().is_empty()
                            || (status == TerminalStatus::Blocked) != problem.recheck().is_some()
                            || (status == TerminalStatus::Refused)
                                != problem.rule().is_some_and(|rule| {
                                    binding(SETUP_RULE_REF).is_ok_and(|expected| rule == &expected)
                                })
                            || (status == TerminalStatus::Error)
                                != problem.correlation_ref().is_some_and(|correlation| {
                                    setup_correlation_ref(code).is_ok_and(|expected| {
                                        correlation.value() == expected.value()
                                    })
                                })
                        {
                            return Err(ContractError::InvalidInvariant);
                        }
                    }
                }
                Ok(())
            }
            Self::RepositoryDoctor
                if problem.subject() == &binding(DOCTOR_USE_CASE_REF)?
                    && problem.details_schema() == &binding(DOCTOR_DETAILS_REF)?
                    && problem.recheck().is_none_or(|condition| {
                        condition.condition_schema() == problem.details_schema()
                    }) =>
            {
                let _: RepositoryDoctorDetailsV1 = admit_contract_value(problem.details(), limits)?;
                Ok(())
            }
            Self::AuthorCharter
                if problem.subject() == &binding(self.use_case_ref())?
                    && problem.details_schema() == &binding(AUTHOR_CHARTER_DETAILS_REF)? =>
            {
                let details: AuthorCharterDetailsV1 =
                    admit_contract_value(problem.details(), limits)?;
                admit_packet3_problem(problem, details.cause)
            }
            Self::AuthorProjectContext
                if problem.subject() == &binding(self.use_case_ref())?
                    && problem.details_schema()
                        == &binding(AUTHOR_PROJECT_CONTEXT_DETAILS_REF)? =>
            {
                let details: AuthorProjectContextDetailsV1 =
                    admit_contract_value(problem.details(), limits)?;
                admit_packet3_problem(problem, details.cause)
            }
            Self::ApproverAdministration
                if problem.subject() == &binding(self.use_case_ref())?
                    && problem.details_schema()
                        == &binding(APPROVER_ADMINISTRATION_DETAILS_REF)? =>
            {
                let details: ApproverAdministrationDetailsV1 =
                    admit_contract_value(problem.details(), limits)?;
                admit_packet3_problem(problem, details.cause)
            }
            Self::PipelineRouteResolveAndPersist
                if problem.subject() == &binding(PIPELINE_ROUTE_USE_CASE_REF)?
                    && problem.details_schema() == &binding(PIPELINE_ROUTE_DETAILS_REF)? =>
            {
                let details: PipelineRouteResolveDetailsV1 =
                    admit_contract_value(problem.details(), limits)?;
                details.validate()?;
                let (status, category, retry) = pipeline_route_disposition(details.cause);
                let cause = pipeline_route_cause_name(details.cause);
                if problem.problem_id() != problem.code()
                    || problem.code().as_str() != format!("pipeline.route.{cause}")
                    || problem.category() != category
                    || problem.retry() != retry
                    || !problem.evidence().as_slice().is_empty()
                    || (status == TerminalStatus::Blocked)
                        != problem.recheck().is_some_and(|condition| {
                            condition.condition_schema() == problem.details_schema()
                                && condition.condition() == problem.details()
                        })
                    || (status == TerminalStatus::Refused) != problem.rule().is_some()
                    || (status == TerminalStatus::Error) != problem.correlation_ref().is_some()
                {
                    return Err(ContractError::InvalidInvariant);
                }
                Ok(())
            }
            _ => Err(ContractError::InvalidInvariant),
        }
    }

    fn admit_terminal(
        self,
        terminal: &TerminalFields<ContractValue>,
        next_actions: &BoundedVec<NextAction, 128>,
        limits: &ContractLimits,
    ) -> Result<(), ContractError> {
        if self == Self::PipelineRouteResolveAndPersist {
            return next_actions
                .as_slice()
                .is_empty()
                .then_some(())
                .ok_or(ContractError::InvalidInvariant);
        }
        if self != Self::RepositorySetup {
            return Ok(());
        }
        if terminal.status() == TerminalStatus::Ok {
            return next_actions
                .as_slice()
                .is_empty()
                .then_some(())
                .ok_or(ContractError::InvalidInvariant);
        }
        let problems = match terminal.status() {
            TerminalStatus::Blocked => terminal.blockers(),
            TerminalStatus::Refused => terminal.refusals(),
            TerminalStatus::Error => terminal.errors(),
            TerminalStatus::Ok => return Ok(()),
        };
        let [problem] = problems else {
            return Err(ContractError::InvalidInvariant);
        };
        let details: RepositorySetupDetailsV1 = admit_contract_value(problem.details(), limits)?;
        match details {
            RepositorySetupDetailsV1::Readiness { result } => {
                if terminal.status() != TerminalStatus::Blocked {
                    return Err(ContractError::InvalidInvariant);
                }
                let expected = setup_readiness_next_actions(
                    result.readiness,
                    &binding(SETUP_USE_CASE_REF)?,
                    &binding(SETUP_DETAILS_REF)?,
                    problem.details(),
                    limits,
                )?;
                if next_actions != &expected {
                    return Err(ContractError::InvalidInvariant);
                }
            }
            details @ RepositorySetupDetailsV1::SetupFailure { .. } => {
                let error = details.to_error()?.ok_or(ContractError::InvalidInvariant)?;
                let (status, _, _, _) = setup_error_disposition(error.code());
                if terminal.status() != status || !next_actions.as_slice().is_empty() {
                    return Err(ContractError::InvalidInvariant);
                }
            }
        }
        Ok(())
    }

    fn admit_receipts(
        self,
        terminal: &TerminalFields<ContractValue>,
        receipts: &BoundedVec<WriteReceipt, 128>,
        limits: &ContractLimits,
    ) -> Result<(), ContractError> {
        if matches!(
            self,
            Self::AuthorCharter | Self::AuthorProjectContext | Self::ApproverAdministration
        ) {
            return self.admit_packet3_receipts(terminal, receipts, limits);
        }
        if self == Self::PipelineRouteResolveAndPersist {
            let success = if terminal.status() == TerminalStatus::Ok {
                Some(admit_contract_value::<
                    PipelineRouteResolveAndPersistResultV1,
                >(
                    terminal.data().ok_or(ContractError::InvalidInvariant)?,
                    limits,
                )?)
            } else {
                None
            };
            if success.is_some() && receipts.as_slice().len() != 1 {
                return Err(ContractError::InvalidInvariant);
            }
            for receipt in receipts.as_slice() {
                if receipt.record().reference().as_str() != PIPELINE_ROUTE_BASIS_RECORD_REF
                    || receipt.record_kind().as_str() != "pipeline_route_basis"
                    || receipt.authority_class().as_str() != "pipeline_owner"
                    || receipt.atomic_group().as_str() != "pipeline_route_resolve_and_persist"
                {
                    return Err(ContractError::InvalidInvariant);
                }
                let condition: PipelineRouteBasisWriteConditionV1 =
                    admit_contract_value(receipt.condition(), limits)?;
                if receipt.record().fingerprint() != &condition.route_basis_fingerprint {
                    return Err(ContractError::InvalidInvariant);
                }
                if let Some(result) = &success {
                    if condition.pipeline_id.as_str() != result.pipeline_id.as_str()
                        || condition.state_revision != result.state.revision
                    {
                        return Err(ContractError::InvalidInvariant);
                    }
                }
            }
            return Ok(());
        }
        if self != Self::RepositorySetup {
            return Ok(());
        }
        let (result, error) = if terminal.status() == TerminalStatus::Ok {
            let data = terminal.data().ok_or(ContractError::InvalidInvariant)?;
            (
                Some(admit_contract_value::<RepositorySetupResultV1>(
                    data, limits,
                )?),
                None,
            )
        } else {
            let problems = match terminal.status() {
                TerminalStatus::Blocked => terminal.blockers(),
                TerminalStatus::Refused => terminal.refusals(),
                TerminalStatus::Error => terminal.errors(),
                TerminalStatus::Ok => unreachable!(),
            };
            let [problem] = problems else {
                return Err(ContractError::InvalidInvariant);
            };
            match admit_contract_value::<RepositorySetupDetailsV1>(problem.details(), limits)? {
                RepositorySetupDetailsV1::Readiness { result } => (Some(*result), None),
                details @ RepositorySetupDetailsV1::SetupFailure { .. } => (
                    None,
                    Some(details.to_error()?.ok_or(ContractError::InvalidInvariant)?),
                ),
            }
        };

        let mut previous_ordinal = None;
        let mut saw_root_receipt = false;
        let mut saw_reset_receipt = false;
        for receipt in receipts.as_slice() {
            let reference = receipt.record().reference().as_str();
            let (ordinal, record_kind, atomic_group) = match reference {
                SETUP_ROOT_RECORD_REF => {
                    (0, "repository_setup_root", "repository_setup_root_create")
                }
                SETUP_RESET_RECORD_REF => (
                    1,
                    "repository_setup_runtime_state_reset",
                    "repository_setup_sequential_reset",
                ),
                SETUP_IDENTITY_RECORD_REF => (
                    2,
                    "repository_setup_identity",
                    "repository_setup_identity_create",
                ),
                _ => return Err(ContractError::InvalidInvariant),
            };
            if previous_ordinal.is_some_and(|previous| ordinal <= previous)
                || receipt.record() != &binding(reference)?
                || receipt.record_kind().as_str() != record_kind
                || receipt.authority_class().as_str() != "sdk_composition"
                || receipt.atomic_group().as_str() != atomic_group
            {
                return Err(ContractError::InvalidInvariant);
            }
            previous_ordinal = Some(ordinal);

            let condition: RepositorySetupReceiptConditionV1 =
                admit_contract_value(receipt.condition(), limits)?;
            match (reference, condition) {
                (
                    SETUP_ROOT_RECORD_REF,
                    RepositorySetupReceiptConditionV1::RootCreated { repo_relative_path },
                ) if repo_relative_path.as_str() == ".handbook"
                    && setup_root_receipt_is_compatible(result.as_ref(), error.as_ref()) =>
                {
                    saw_root_receipt = true;
                }
                (
                    SETUP_RESET_RECORD_REF,
                    RepositorySetupReceiptConditionV1::RuntimeStateReset {
                        application: RepositorySetupResetApplicationV1::Sequential,
                        removed_paths,
                    },
                ) if setup_reset_receipt_is_compatible(
                    removed_paths.as_slice(),
                    result.as_ref(),
                    error.as_ref(),
                ) =>
                {
                    saw_reset_receipt = true;
                }
                (
                    SETUP_IDENTITY_RECORD_REF,
                    RepositorySetupReceiptConditionV1::RepositoryIdentityCreated {
                        repo_relative_path,
                        ..
                    },
                ) if repo_relative_path.as_str()
                    == handbook_engine::REPOSITORY_IDENTITY_REPO_PATH
                    && setup_identity_receipt_is_compatible(result.as_ref(), error.as_ref()) => {}
                _ => return Err(ContractError::InvalidInvariant),
            }
        }
        if let Some(result) = result {
            let root_receipt_required = result.root_action == RepositorySetupRootActionV1::Create;
            let reset_receipt_required =
                result.reset_applied && !result.reset_paths.as_slice().is_empty();
            if saw_root_receipt != root_receipt_required
                || saw_reset_receipt != reset_receipt_required
            {
                return Err(ContractError::InvalidInvariant);
            }
        }
        Ok(())
    }

    fn admit_packet3_receipts(
        self,
        terminal: &TerminalFields<ContractValue>,
        receipts: &BoundedVec<WriteReceipt, 128>,
        limits: &ContractLimits,
    ) -> Result<(), ContractError> {
        let (record_ref, record_kind, atomic_group) = match self {
            Self::AuthorCharter => (
                AUTHOR_CHARTER_EFFECT_REF,
                "author_charter_effect",
                "author_charter_owner_installs",
            ),
            Self::AuthorProjectContext => (
                AUTHOR_PROJECT_CONTEXT_EFFECT_REF,
                "author_project_context_effect",
                "author_project_context_owner_install",
            ),
            Self::ApproverAdministration => (
                APPROVER_ADMINISTRATION_EFFECT_REF,
                "approver_administration_effect",
                "approver_administration_owner_installs",
            ),
            _ => return Err(ContractError::InvalidInvariant),
        };
        let mut targets = std::collections::BTreeSet::new();
        let mut conditions = Vec::with_capacity(receipts.as_slice().len());
        for receipt in receipts.as_slice() {
            if receipt.record() != &binding(record_ref)?
                || receipt.record_kind().as_str() != record_kind
                || receipt.authority_class().as_str() != "sdk_composition"
                || receipt.atomic_group().as_str() != atomic_group
            {
                return Err(ContractError::InvalidInvariant);
            }
            let condition: Packet3EffectConditionV1 =
                admit_contract_value(receipt.condition(), limits)?;
            let target = condition.target_ref.as_str();
            if !targets.insert(target.to_owned())
                || !packet3_effect_target_is_exact(
                    self,
                    target,
                    condition.content_fingerprint.as_ref(),
                )
            {
                return Err(ContractError::InvalidInvariant);
            }
            match condition.disposition {
                Packet3EffectDispositionV1::CreatedIncomplete => {
                    if condition.content_fingerprint.is_some() {
                        return Err(ContractError::InvalidInvariant);
                    }
                }
                Packet3EffectDispositionV1::InstalledDurabilityUnconfirmed
                | Packet3EffectDispositionV1::DurableCreated => {
                    if condition.content_fingerprint.is_none() {
                        return Err(ContractError::InvalidInvariant);
                    }
                }
            }
            conditions.push(condition);
        }

        match self {
            Self::AuthorCharter => {
                let operation = if terminal.status() == TerminalStatus::Ok {
                    let result: AuthorCharterResultV1 = admit_contract_value(
                        terminal.data().ok_or(ContractError::InvalidInvariant)?,
                        limits,
                    )?;
                    result.operation
                } else {
                    let [problem] = terminal_problems(terminal) else {
                        return Err(ContractError::InvalidInvariant);
                    };
                    let details: AuthorCharterDetailsV1 =
                        admit_contract_value(problem.details(), limits)?;
                    details.operation
                };
                if operation == AuthorCharterOperationV1::Validate && !conditions.is_empty() {
                    return Err(ContractError::InvalidInvariant);
                }
            }
            Self::AuthorProjectContext => {
                if terminal.status() == TerminalStatus::Ok {
                    let result: AuthorProjectContextResultV1 = admit_contract_value(
                        terminal.data().ok_or(ContractError::InvalidInvariant)?,
                        limits,
                    )?;
                    match result {
                        AuthorProjectContextResultV1::Validate { .. } if conditions.is_empty() => {}
                        AuthorProjectContextResultV1::Author {
                            source_fingerprint, ..
                        } if conditions.as_slice()
                            == [Packet3EffectConditionV1 {
                                target_ref: bounded(".handbook/project/context.yaml")?,
                                content_fingerprint: Some(source_fingerprint.clone()),
                                disposition: Packet3EffectDispositionV1::DurableCreated,
                            }] => {}
                        _ => return Err(ContractError::InvalidInvariant),
                    }
                } else {
                    let [problem] = terminal_problems(terminal) else {
                        return Err(ContractError::InvalidInvariant);
                    };
                    let details: AuthorProjectContextDetailsV1 =
                        admit_contract_value(problem.details(), limits)?;
                    match details.operation {
                        AuthorProjectContextOperationV1::Validate if conditions.is_empty() => {}
                        AuthorProjectContextOperationV1::Author
                            if details.refusal_kind
                                == AuthorProjectContextRefusalKindV1::MutationIoFailed
                                && (conditions.is_empty()
                                    || conditions.as_slice()
                                        == [Packet3EffectConditionV1 {
                                            target_ref: bounded(
                                                ".handbook/project/context.yaml",
                                            )?,
                                            content_fingerprint: conditions[0]
                                                .content_fingerprint
                                                .clone(),
                                            disposition: Packet3EffectDispositionV1::InstalledDurabilityUnconfirmed,
                                        }]) => {}
                        AuthorProjectContextOperationV1::Author if conditions.is_empty() => {}
                        _ => return Err(ContractError::InvalidInvariant),
                    }
                }
            }
            Self::ApproverAdministration => {}
            _ => return Err(ContractError::InvalidInvariant),
        }
        Ok(())
    }
}

fn terminal_problems<T>(terminal: &TerminalFields<T>) -> &[Problem] {
    match terminal.status() {
        TerminalStatus::Blocked => terminal.blockers(),
        TerminalStatus::Refused => terminal.refusals(),
        TerminalStatus::Error => terminal.errors(),
        TerminalStatus::Ok => &[],
    }
}

fn packet3_effect_target_is_exact(
    kind: UseCaseKind,
    target: &str,
    fingerprint: Option<&Fingerprint>,
) -> bool {
    if kind == UseCaseKind::AuthorProjectContext {
        return target == ".handbook/project/context.yaml";
    }
    if kind == UseCaseKind::AuthorCharter && target == ".handbook/project/charter.yaml" {
        return true;
    }

    const AUTHOR_TARGETS: [(&str, &str); 15] = [
        (".handbook/state/intake-records/intake_", ".json"),
        (".handbook/evidence/charter/candidates/candidate_", ".json"),
        (".handbook/state/candidate-content/charter_", ".yaml"),
        (
            ".handbook/state/lifecycle-validation-results/lifecycle-validation-result_",
            ".json",
        ),
        (".handbook/state/promotions/promotion_", ".json"),
        (
            ".handbook/state/lifecycle-transitions/lifecycle-transition_",
            ".json",
        ),
        (
            ".handbook/state/lifecycle-observations/lifecycle-observation_",
            ".json",
        ),
        (".handbook/state/approvals/approval_", ".json"),
        (".handbook/state/registry-states/registry-state_", ".json"),
        (
            ".handbook/state/registry-transitions/registry-transition_",
            ".json",
        ),
        (
            ".handbook/state/authenticator-make-credential-responses/authenticator-make-credential-response_",
            ".json",
        ),
        (
            ".handbook/state/authenticator-registrations/authenticator-registration_",
            ".json",
        ),
        (
            ".handbook/state/authenticator-get-assertion-responses/authenticator-get-assertion-response_",
            ".json",
        ),
        (
            ".handbook/state/authenticator-assertions/authenticator-assertion_",
            ".json",
        ),
        (
            ".handbook/state/authenticator-use-transitions/authenticator-use-transition_",
            ".json",
        ),
    ];
    const APPROVER_TARGETS: [(&str, &str); 8] = [
        (".handbook/state/approvals/approval_", ".json"),
        (".handbook/state/registry-states/registry-state_", ".json"),
        (
            ".handbook/state/registry-transitions/registry-transition_",
            ".json",
        ),
        (
            ".handbook/state/authenticator-make-credential-responses/authenticator-make-credential-response_",
            ".json",
        ),
        (
            ".handbook/state/authenticator-registrations/authenticator-registration_",
            ".json",
        ),
        (
            ".handbook/state/authenticator-get-assertion-responses/authenticator-get-assertion-response_",
            ".json",
        ),
        (
            ".handbook/state/authenticator-assertions/authenticator-assertion_",
            ".json",
        ),
        (
            ".handbook/state/authenticator-use-transitions/authenticator-use-transition_",
            ".json",
        ),
    ];
    let targets = match kind {
        UseCaseKind::AuthorCharter => AUTHOR_TARGETS.as_slice(),
        UseCaseKind::ApproverAdministration => APPROVER_TARGETS.as_slice(),
        _ => return false,
    };
    targets.iter().any(|(prefix, suffix)| {
        let Some(hex) = target
            .strip_prefix(prefix)
            .and_then(|rest| rest.strip_suffix(suffix))
        else {
            return false;
        };
        hex.len() == 64
            && hex
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
            && (*prefix
                == ".handbook/state/lifecycle-validation-results/lifecycle-validation-result_"
                || fingerprint.is_none_or(|fingerprint| {
                    fingerprint.as_str().strip_prefix("sha256:") == Some(hex)
                }))
    })
}

fn setup_root_receipt_is_compatible(
    result: Option<&RepositorySetupResultV1>,
    error: Option<&SetupError>,
) -> bool {
    result.is_some_and(|result| result.root_action == RepositorySetupRootActionV1::Create)
        || error.is_some_and(|error| {
            matches!(
                error.code(),
                SetupErrorCode::RepositoryAuthorityRecoveryBlocked
                    | SetupErrorCode::RepositoryIdentityUnsafe
                    | SetupErrorCode::RepositoryIdentityMismatch
                    | SetupErrorCode::RepositoryIdentityEntropyUnavailable
                    | SetupErrorCode::RepositoryIdentityPersistenceFailed
            )
        })
}

fn setup_reset_receipt_is_compatible(
    removed_paths: &[BoundedString<4096>],
    result: Option<&RepositorySetupResultV1>,
    error: Option<&SetupError>,
) -> bool {
    if removed_paths.is_empty()
        || removed_paths.iter().any(|path| {
            !setup_reset_path_is_normalized(path.as_str())
                || removed_paths
                    .iter()
                    .filter(|candidate| candidate == &path)
                    .count()
                    != 1
        })
    {
        return false;
    }
    if let Some(result) = result {
        if !result.reset_applied || result.reset_paths.as_slice().len() != removed_paths.len() {
            return false;
        }
        let mut expected = result
            .reset_paths
            .as_slice()
            .iter()
            .map(BoundedString::as_str)
            .collect::<Vec<_>>();
        let mut observed = removed_paths
            .iter()
            .map(BoundedString::as_str)
            .collect::<Vec<_>>();
        expected.sort_unstable();
        observed.sort_unstable();
        return expected == observed;
    }
    error.is_some_and(|error| {
        matches!(
            error.code(),
            SetupErrorCode::RuntimeStateMutationFailed
                | SetupErrorCode::RepositoryAuthorityRecoveryBlocked
                | SetupErrorCode::RepositoryIdentityUnsafe
                | SetupErrorCode::RepositoryIdentityMismatch
                | SetupErrorCode::RepositoryIdentityEntropyUnavailable
                | SetupErrorCode::RepositoryIdentityPersistenceFailed
        )
    })
}

fn setup_reset_path_is_normalized(path: &str) -> bool {
    path.strip_prefix(".handbook/state/").is_some_and(|tail| {
        !tail.is_empty()
            && !tail.contains('\\')
            && !tail.chars().any(char::is_control)
            && tail
                .split('/')
                .all(|segment| !segment.is_empty() && !matches!(segment, "." | ".."))
    })
}

fn setup_identity_receipt_is_compatible(
    result: Option<&RepositorySetupResultV1>,
    error: Option<&SetupError>,
) -> bool {
    result.is_some()
        || error.is_some_and(|error| {
            error.code() == SetupErrorCode::RepositoryIdentityPersistenceFailed
        })
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct UseCaseResponseV1 {
    schema: ExactBinding,
    use_case: ExactBinding,
    result_schema: ExactBinding,
    #[serde(flatten)]
    terminal: TerminalFields<ContractValue>,
    receipts: BoundedVec<WriteReceipt, 128>,
    diagnostics: BoundedVec<Diagnostic, 128>,
    next_actions: BoundedVec<NextAction, 128>,
    result_fingerprint: Fingerprint,
    response_fingerprint: Fingerprint,
}

fn repository_setup_receipts(
    effects: &SetupExecutionEffects,
    limits: &ContractLimits,
) -> Result<BoundedVec<WriteReceipt, 128>, ContractError> {
    let mut receipts = Vec::new();
    if effects.root_created() {
        receipts.push(repository_setup_receipt(
            "repository_setup_root",
            SETUP_ROOT_RECORD_REF,
            &RepositorySetupReceiptConditionV1::RootCreated {
                repo_relative_path: bounded(".handbook")?,
            },
            "repository_setup_root_create",
            limits,
        )?);
    }
    if !effects.removed_runtime_state_paths().is_empty() {
        receipts.push(repository_setup_receipt(
            "repository_setup_runtime_state_reset",
            SETUP_RESET_RECORD_REF,
            &RepositorySetupReceiptConditionV1::RuntimeStateReset {
                application: RepositorySetupResetApplicationV1::Sequential,
                removed_paths: bounded_vec(effects.removed_runtime_state_paths())?,
            },
            "repository_setup_sequential_reset",
            limits,
        )?);
    }
    if let Some(repository_identity_fingerprint) = effects.repository_identity_created() {
        receipts.push(repository_setup_receipt(
            "repository_setup_identity",
            SETUP_IDENTITY_RECORD_REF,
            &RepositorySetupReceiptConditionV1::RepositoryIdentityCreated {
                repo_relative_path: bounded(handbook_engine::REPOSITORY_IDENTITY_REPO_PATH)?,
                repository_identity_fingerprint: Fingerprint::parse(
                    repository_identity_fingerprint,
                )?,
            },
            "repository_setup_identity_create",
            limits,
        )?);
    }
    BoundedVec::new(receipts)
}

fn repository_setup_receipt(
    record_kind: &str,
    record_ref: &str,
    condition: &RepositorySetupReceiptConditionV1,
    atomic_group: &str,
    limits: &ContractLimits,
) -> Result<WriteReceipt, ContractError> {
    WriteReceipt::new(
        bounded(record_kind)?,
        binding(record_ref)?,
        bounded("sdk_composition")?,
        contract_value(condition, limits)?,
        bounded(atomic_group)?,
        limits,
    )
}

fn packet3_receipts(
    kind: UseCaseKind,
    effects: &[AdapterPublicEffect],
    limits: &ContractLimits,
) -> Result<BoundedVec<WriteReceipt, 128>, ContractError> {
    let (record_ref, record_kind, atomic_group) = match kind {
        UseCaseKind::AuthorCharter => (
            AUTHOR_CHARTER_EFFECT_REF,
            "author_charter_effect",
            "author_charter_owner_installs",
        ),
        UseCaseKind::AuthorProjectContext => (
            AUTHOR_PROJECT_CONTEXT_EFFECT_REF,
            "author_project_context_effect",
            "author_project_context_owner_install",
        ),
        UseCaseKind::ApproverAdministration => (
            APPROVER_ADMINISTRATION_EFFECT_REF,
            "approver_administration_effect",
            "approver_administration_owner_installs",
        ),
        _ => return Err(ContractError::InvalidInvariant),
    };
    BoundedVec::new(
        effects
            .iter()
            .map(|effect| {
                let disposition = match effect.disposition {
                    AdapterPublicEffectDisposition::CreatedIncomplete => {
                        Packet3EffectDispositionV1::CreatedIncomplete
                    }
                    AdapterPublicEffectDisposition::InstalledDurabilityUnconfirmed => {
                        Packet3EffectDispositionV1::InstalledDurabilityUnconfirmed
                    }
                    AdapterPublicEffectDisposition::DurableCreated => {
                        Packet3EffectDispositionV1::DurableCreated
                    }
                };
                let content_fingerprint = effect
                    .content_fingerprint
                    .as_deref()
                    .map(Fingerprint::parse)
                    .transpose()?;
                if (disposition == Packet3EffectDispositionV1::CreatedIncomplete)
                    != content_fingerprint.is_none()
                {
                    return Err(ContractError::InvalidInvariant);
                }
                WriteReceipt::new(
                    bounded(record_kind)?,
                    binding(record_ref)?,
                    bounded("sdk_composition")?,
                    contract_value(
                        &Packet3EffectConditionV1 {
                            target_ref: bounded(&effect.target_ref)?,
                            content_fingerprint,
                            disposition,
                        },
                        limits,
                    )?,
                    bounded(atomic_group)?,
                    limits,
                )
            })
            .collect::<Result<Vec<_>, _>>()?,
    )
}

fn pipeline_route_receipts(
    effects: &[PipelineResolveEffect],
    limits: &ContractLimits,
) -> Result<BoundedVec<WriteReceipt, 128>, ContractError> {
    BoundedVec::new(
        effects
            .iter()
            .map(|effect| {
                let route_basis_fingerprint = canonical_fingerprint(&effect.route_basis, limits)?;
                WriteReceipt::new(
                    bounded("pipeline_route_basis")?,
                    ExactBinding::new(
                        ExactRef::parse(PIPELINE_ROUTE_BASIS_RECORD_REF)?,
                        route_basis_fingerprint.clone(),
                    ),
                    bounded("pipeline_owner")?,
                    contract_value(
                        &PipelineRouteBasisWriteConditionV1 {
                            pipeline_id: bounded(&effect.pipeline_id)?,
                            state_revision: SafeU64::new(effect.state_revision)?,
                            route_basis_fingerprint,
                        },
                        limits,
                    )?,
                    bounded("pipeline_route_resolve_and_persist")?,
                    limits,
                )
            })
            .collect::<Result<Vec<_>, ContractError>>()?,
    )
}

fn pipeline_route_details(
    failure: &PipelineResolveExecutionFailure,
) -> Result<PipelineRouteResolveDetailsV1, ContractError> {
    let mut selector = None;
    let mut matches = Vec::new();
    let mut expected_revision = None;
    let mut actual_revision = None;
    let (cause, reason) = match failure {
        PipelineResolveExecutionFailure::Selection(PipelineSelectedDefinitionFailure::Catalog(
            reason,
        )) => (PipelineRouteResolveCauseV1::Catalog, reason.clone()),
        PipelineResolveExecutionFailure::Selection(
            PipelineSelectedDefinitionFailure::Selector(PipelineSelectorRefusal::Unsupported {
                selector: value,
                reason,
            }),
        ) => {
            selector = Some(bounded(value)?);
            (
                PipelineRouteResolveCauseV1::UnsupportedSelector,
                reason.clone(),
            )
        }
        PipelineResolveExecutionFailure::Selection(
            PipelineSelectedDefinitionFailure::Selector(PipelineSelectorRefusal::Ambiguous {
                selector: value,
                matches: values,
            }),
        ) => {
            selector = Some(bounded(value)?);
            matches = values
                .iter()
                .map(|value| bounded(value))
                .collect::<Result<Vec<_>, ContractError>>()?;
            (
                PipelineRouteResolveCauseV1::AmbiguousSelector,
                "selector matched more than one pipeline or stage".to_owned(),
            )
        }
        PipelineResolveExecutionFailure::Selection(
            PipelineSelectedDefinitionFailure::Selector(PipelineSelectorRefusal::Unknown {
                selector: value,
            }),
        ) => {
            selector = Some(bounded(value)?);
            (
                PipelineRouteResolveCauseV1::UnknownSelector,
                "selector did not match a pipeline or stage".to_owned(),
            )
        }
        PipelineResolveExecutionFailure::Selection(
            PipelineSelectedDefinitionFailure::Definition(reason),
        ) => (PipelineRouteResolveCauseV1::Definition, reason.clone()),
        PipelineResolveExecutionFailure::RouteStateRead(reason) => {
            (PipelineRouteResolveCauseV1::RouteStateRead, reason.clone())
        }
        PipelineResolveExecutionFailure::RouteVariables(reason) => {
            (PipelineRouteResolveCauseV1::RouteVariables, reason.clone())
        }
        PipelineResolveExecutionFailure::RouteResolution(reason) => {
            (PipelineRouteResolveCauseV1::RouteResolution, reason.clone())
        }
        PipelineResolveExecutionFailure::RouteBasisBuild(reason) => {
            (PipelineRouteResolveCauseV1::RouteBasisBuild, reason.clone())
        }
        PipelineResolveExecutionFailure::RouteBasisPersistenceRefused(
            PipelineRouteBasisPersistenceRefusal::MalformedState { reason },
        ) => (
            PipelineRouteResolveCauseV1::RouteBasisPersistenceMalformed,
            reason.clone(),
        ),
        PipelineResolveExecutionFailure::RouteBasisPersistenceRefused(
            PipelineRouteBasisPersistenceRefusal::RevisionConflict {
                expected_revision: expected,
                actual_revision: actual,
            },
        ) => {
            expected_revision = Some(SafeU64::new(*expected)?);
            actual_revision = Some(SafeU64::new(*actual)?);
            (
                PipelineRouteResolveCauseV1::RouteBasisRevisionConflict,
                "route state changed before route_basis persistence".to_owned(),
            )
        }
        PipelineResolveExecutionFailure::RouteBasisPersistence(reason) => (
            PipelineRouteResolveCauseV1::RouteBasisPersistence,
            reason.clone(),
        ),
    };
    Ok(PipelineRouteResolveDetailsV1 {
        cause,
        reason: bounded(&reason)?,
        selector,
        matches: BoundedVec::new(matches)?,
        expected_revision,
        actual_revision,
    })
}

fn pipeline_route_disposition(
    cause: PipelineRouteResolveCauseV1,
) -> (TerminalStatus, ProblemCategory, RetryDirective) {
    match cause {
        PipelineRouteResolveCauseV1::UnsupportedSelector
        | PipelineRouteResolveCauseV1::AmbiguousSelector
        | PipelineRouteResolveCauseV1::UnknownSelector => (
            TerminalStatus::Refused,
            ProblemCategory::Resolution,
            RetryDirective::AfterRequestChange,
        ),
        PipelineRouteResolveCauseV1::Catalog
        | PipelineRouteResolveCauseV1::RouteStateRead
        | PipelineRouteResolveCauseV1::RouteBasisPersistence => (
            TerminalStatus::Error,
            ProblemCategory::Implementation,
            RetryDirective::Transient,
        ),
        _ => (
            TerminalStatus::Blocked,
            ProblemCategory::Prerequisite,
            RetryDirective::AfterRecheck,
        ),
    }
}

fn pipeline_route_cause_name(cause: PipelineRouteResolveCauseV1) -> &'static str {
    match cause {
        PipelineRouteResolveCauseV1::Catalog => "catalog",
        PipelineRouteResolveCauseV1::UnsupportedSelector => "unsupported_selector",
        PipelineRouteResolveCauseV1::AmbiguousSelector => "ambiguous_selector",
        PipelineRouteResolveCauseV1::UnknownSelector => "unknown_selector",
        PipelineRouteResolveCauseV1::Definition => "definition",
        PipelineRouteResolveCauseV1::RouteStateRead => "route_state_read",
        PipelineRouteResolveCauseV1::RouteVariables => "route_variables",
        PipelineRouteResolveCauseV1::RouteResolution => "route_resolution",
        PipelineRouteResolveCauseV1::RouteBasisBuild => "route_basis_build",
        PipelineRouteResolveCauseV1::RouteBasisPersistenceMalformed => {
            "route_basis_persistence_malformed"
        }
        PipelineRouteResolveCauseV1::RouteBasisRevisionConflict => "route_basis_revision_conflict",
        PipelineRouteResolveCauseV1::RouteBasisPersistence => "route_basis_persistence",
    }
}

fn pipeline_route_failure(
    details: &PipelineRouteResolveDetailsV1,
) -> Result<PipelineResolveExecutionFailure, ContractError> {
    let reason = details.reason.as_str().to_owned();
    let selector = || {
        details
            .selector
            .as_ref()
            .map(|value| value.as_str().to_owned())
            .ok_or(ContractError::InvalidInvariant)
    };
    Ok(match details.cause {
        PipelineRouteResolveCauseV1::Catalog => PipelineResolveExecutionFailure::Selection(
            PipelineSelectedDefinitionFailure::Catalog(reason),
        ),
        PipelineRouteResolveCauseV1::UnsupportedSelector => {
            PipelineResolveExecutionFailure::Selection(PipelineSelectedDefinitionFailure::Selector(
                PipelineSelectorRefusal::Unsupported {
                    selector: selector()?,
                    reason,
                },
            ))
        }
        PipelineRouteResolveCauseV1::AmbiguousSelector => {
            PipelineResolveExecutionFailure::Selection(PipelineSelectedDefinitionFailure::Selector(
                PipelineSelectorRefusal::Ambiguous {
                    selector: selector()?,
                    matches: details
                        .matches
                        .as_slice()
                        .iter()
                        .map(|value| value.as_str().to_owned())
                        .collect(),
                },
            ))
        }
        PipelineRouteResolveCauseV1::UnknownSelector => PipelineResolveExecutionFailure::Selection(
            PipelineSelectedDefinitionFailure::Selector(PipelineSelectorRefusal::Unknown {
                selector: selector()?,
            }),
        ),
        PipelineRouteResolveCauseV1::Definition => PipelineResolveExecutionFailure::Selection(
            PipelineSelectedDefinitionFailure::Definition(reason),
        ),
        PipelineRouteResolveCauseV1::RouteStateRead => {
            PipelineResolveExecutionFailure::RouteStateRead(reason)
        }
        PipelineRouteResolveCauseV1::RouteVariables => {
            PipelineResolveExecutionFailure::RouteVariables(reason)
        }
        PipelineRouteResolveCauseV1::RouteResolution => {
            PipelineResolveExecutionFailure::RouteResolution(reason)
        }
        PipelineRouteResolveCauseV1::RouteBasisBuild => {
            PipelineResolveExecutionFailure::RouteBasisBuild(reason)
        }
        PipelineRouteResolveCauseV1::RouteBasisPersistenceMalformed => {
            PipelineResolveExecutionFailure::RouteBasisPersistenceRefused(
                PipelineRouteBasisPersistenceRefusal::MalformedState { reason },
            )
        }
        PipelineRouteResolveCauseV1::RouteBasisRevisionConflict => {
            PipelineResolveExecutionFailure::RouteBasisPersistenceRefused(
                PipelineRouteBasisPersistenceRefusal::RevisionConflict {
                    expected_revision: details
                        .expected_revision
                        .ok_or(ContractError::InvalidInvariant)?
                        .get(),
                    actual_revision: details
                        .actual_revision
                        .ok_or(ContractError::InvalidInvariant)?
                        .get(),
                },
            )
        }
        PipelineRouteResolveCauseV1::RouteBasisPersistence => {
            PipelineResolveExecutionFailure::RouteBasisPersistence(reason)
        }
    })
}

fn empty_bounded<T>() -> Result<BoundedVec<T, 128>, ContractError> {
    BoundedVec::new(Vec::new())
}

fn setup_readiness_code(
    status: RepositoryReadinessStatusV1,
) -> Result<&'static str, ContractError> {
    match status {
        RepositoryReadinessStatusV1::ActionRequired => Ok("repository.action_required"),
        RepositoryReadinessStatusV1::Indeterminate => Ok("repository.readiness_indeterminate"),
        RepositoryReadinessStatusV1::Invalid => Ok("repository.readiness_invalid"),
        RepositoryReadinessStatusV1::Ready => Err(ContractError::InvalidInvariant),
    }
}

fn setup_readiness_next_actions(
    status: RepositoryReadinessStatusV1,
    use_case: &ExactBinding,
    details_schema: &ExactBinding,
    details: &ContractValue,
    limits: &ContractLimits,
) -> Result<BoundedVec<NextAction, 128>, ContractError> {
    let action = match status {
        RepositoryReadinessStatusV1::Ready | RepositoryReadinessStatusV1::Indeterminate => None,
        RepositoryReadinessStatusV1::ActionRequired => Some((
            "repository.resolve_setup_actions",
            "Resolve repository readiness actions and rerun setup",
        )),
        RepositoryReadinessStatusV1::Invalid => Some((
            "repository.repair_readiness",
            "Repair invalid repository inputs and rerun setup",
        )),
    };
    BoundedVec::new(
        action
            .map(|(action_id, display_label)| {
                NextAction::new(
                    bounded(action_id)?,
                    NextActionKind::ChangeRequest,
                    use_case.clone(),
                    details_schema.clone(),
                    details.clone(),
                    Some(bounded(display_label)?),
                    limits,
                )
            })
            .into_iter()
            .collect::<Result<Vec<_>, ContractError>>()?,
    )
}

fn setup_error_disposition(
    code: SetupErrorCode,
) -> (
    TerminalStatus,
    ProblemCategory,
    RetryDirective,
    &'static str,
) {
    match code {
        SetupErrorCode::ShippedProfileUnavailable => (
            TerminalStatus::Blocked,
            ProblemCategory::Prerequisite,
            RetryDirective::AfterRecheck,
            "repository.shipped_profile_unavailable",
        ),
        SetupErrorCode::SelectedProfileDecisionInvalid => (
            TerminalStatus::Refused,
            ProblemCategory::Precondition,
            RetryDirective::AfterRequestChange,
            "repository.selected_profile_decision_invalid",
        ),
        SetupErrorCode::UnresolvedMode => (
            TerminalStatus::Refused,
            ProblemCategory::Schema,
            RetryDirective::AfterRequestChange,
            "setup.unresolved_mode",
        ),
        SetupErrorCode::InitRejectsRefreshFlags => (
            TerminalStatus::Refused,
            ProblemCategory::Schema,
            RetryDirective::AfterRequestChange,
            "setup.init_rejects_refresh_flags",
        ),
        SetupErrorCode::RootAlreadyInitialized => (
            TerminalStatus::Refused,
            ProblemCategory::Precondition,
            RetryDirective::AfterRequestChange,
            "repository.root_already_initialized",
        ),
        SetupErrorCode::RefreshRootMissing => (
            TerminalStatus::Blocked,
            ProblemCategory::Prerequisite,
            RetryDirective::AfterRecheck,
            "repository.root_missing",
        ),
        SetupErrorCode::RootNotDirectory => (
            TerminalStatus::Refused,
            ProblemCategory::Safety,
            RetryDirective::Never,
            "repository.root_not_directory",
        ),
        SetupErrorCode::RootSymlinkRefused => (
            TerminalStatus::Refused,
            ProblemCategory::Safety,
            RetryDirective::Never,
            "repository.root_symlink_refused",
        ),
        SetupErrorCode::CanonicalRootInspectFailed => (
            TerminalStatus::Error,
            ProblemCategory::Implementation,
            RetryDirective::Never,
            "repository.root_inspect_failed",
        ),
        SetupErrorCode::CanonicalRootCreateFailed => (
            TerminalStatus::Error,
            ProblemCategory::Implementation,
            RetryDirective::Never,
            "repository.root_create_failed",
        ),
        SetupErrorCode::RewriteHasNoMaterializer => (
            TerminalStatus::Refused,
            ProblemCategory::Capability,
            RetryDirective::AfterRequestChange,
            "setup.rewrite_materializer_unavailable",
        ),
        SetupErrorCode::RuntimeStateTargetUnsafe => (
            TerminalStatus::Refused,
            ProblemCategory::Safety,
            RetryDirective::Never,
            "repository.runtime_state_target_unsafe",
        ),
        SetupErrorCode::RuntimeStateResetPlanTooLarge => (
            TerminalStatus::Refused,
            ProblemCategory::Schema,
            RetryDirective::AfterRequestChange,
            "repository.runtime_state_reset_plan_too_large",
        ),
        SetupErrorCode::RuntimeStateResetProjectionTooLarge => (
            TerminalStatus::Refused,
            ProblemCategory::Schema,
            RetryDirective::AfterRequestChange,
            "repository.runtime_state_reset_projection_too_large",
        ),
        SetupErrorCode::RuntimeStateMutationFailed => (
            TerminalStatus::Error,
            ProblemCategory::Implementation,
            RetryDirective::Transient,
            "repository.runtime_state_mutation_failed",
        ),
        SetupErrorCode::RepositoryAuthorityRecoveryBlocked => (
            TerminalStatus::Blocked,
            ProblemCategory::Prerequisite,
            RetryDirective::AfterRecheck,
            "repository.authority_recovery_blocked",
        ),
        SetupErrorCode::RepositoryIdentityUnsafe => (
            TerminalStatus::Refused,
            ProblemCategory::Safety,
            RetryDirective::Never,
            "repository.identity_unsafe",
        ),
        SetupErrorCode::RepositoryIdentityMismatch => (
            TerminalStatus::Refused,
            ProblemCategory::Authority,
            RetryDirective::AfterAuthorityChange,
            "repository.identity_mismatch",
        ),
        SetupErrorCode::RepositoryIdentityEntropyUnavailable => (
            TerminalStatus::Error,
            ProblemCategory::Implementation,
            RetryDirective::Transient,
            "repository.identity_entropy_unavailable",
        ),
        SetupErrorCode::RepositoryIdentityPersistenceFailed => (
            TerminalStatus::Error,
            ProblemCategory::Implementation,
            RetryDirective::Transient,
            "repository.identity_persistence_failed",
        ),
    }
}

fn setup_correlation_ref(code: &str) -> Result<CorrelationRef, ContractError> {
    let fingerprint = sha256_fingerprint(code.as_bytes());
    CorrelationRef::new(bounded(&format!("corr_{}", &fingerprint.as_str()[7..]))?)
}

pub(crate) fn measure_repository_setup_invocation(
    execution: &SetupExecutionResult,
) -> Result<usize, ContractError> {
    let measurement_limits = ContractLimits::setup_measurement();
    let use_case = UseCaseResponseV1::repository_setup_execution(execution, &measurement_limits)?;
    let invocation = InvocationResponseV1::use_case(use_case, &measurement_limits)?;
    Ok(canonical_json_bytes(&invocation, &measurement_limits)?.len())
}

impl UseCaseResponseV1 {
    pub fn repository_setup(
        outcome: &SetupOutcome,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        let execution =
            SetupExecutionResult::new(Ok(outcome.clone()), SetupExecutionEffects::default());
        Self::repository_setup_execution(&execution, limits)
    }

    pub(crate) fn repository_setup_execution(
        execution: &SetupExecutionResult,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        let use_case = binding(SETUP_USE_CASE_REF)?;
        let result_schema = binding(SETUP_RESULT_REF)?;
        let details_schema = binding(SETUP_DETAILS_REF)?;
        let receipts = repository_setup_receipts(execution.effects(), limits)?;
        match execution.result() {
            Ok(outcome) => {
                let result = RepositorySetupResultV1::from_outcome(outcome)?;
                if result.readiness == RepositoryReadinessStatusV1::Ready {
                    return Self::success(
                        UseCaseKind::RepositorySetup,
                        &result,
                        receipts,
                        empty_bounded()?,
                        empty_bounded()?,
                        limits,
                    );
                }
                let details = contract_value(
                    &RepositorySetupDetailsV1::Readiness {
                        result: Box::new(result.clone()),
                    },
                    limits,
                )?;
                let code = setup_readiness_code(result.readiness)?;
                let recheck =
                    RecheckCondition::new(details_schema.clone(), details.clone(), limits)?;
                let problem = Problem::new(
                    bounded(code)?,
                    bounded(code)?,
                    ProblemCategory::Prerequisite,
                    use_case.clone(),
                    None,
                    details_schema.clone(),
                    details.clone(),
                    empty_bounded()?,
                    RetryDirective::AfterRecheck,
                    Some(recheck),
                    None,
                    limits,
                )?;
                let next_actions = setup_readiness_next_actions(
                    result.readiness,
                    &use_case,
                    &details_schema,
                    &details,
                    limits,
                )?;
                Self::from_parts(
                    binding(USE_CASE_RESPONSE_REF)?,
                    use_case,
                    result_schema,
                    TerminalFields::new(
                        TerminalStatus::Blocked,
                        None,
                        BoundedVec::new(vec![problem])?,
                        empty_bounded()?,
                        empty_bounded()?,
                    )?,
                    receipts,
                    empty_bounded()?,
                    next_actions,
                    limits,
                )
            }
            Err(error) => {
                let details =
                    contract_value(&RepositorySetupDetailsV1::from_error(error)?, limits)?;
                let (status, category, retry, code) = setup_error_disposition(error.code());
                let recheck = (status == TerminalStatus::Blocked)
                    .then(|| RecheckCondition::new(details_schema.clone(), details.clone(), limits))
                    .transpose()?;
                let rule = (status == TerminalStatus::Refused)
                    .then(|| binding(SETUP_RULE_REF))
                    .transpose()?;
                let correlation_ref = if status == TerminalStatus::Error {
                    Some(setup_correlation_ref(code)?)
                } else {
                    None
                };
                let problem = Problem::new(
                    bounded(code)?,
                    bounded(code)?,
                    category,
                    use_case.clone(),
                    rule,
                    details_schema,
                    details,
                    empty_bounded()?,
                    retry,
                    recheck,
                    correlation_ref,
                    limits,
                )?;
                let terminal = match status {
                    TerminalStatus::Blocked => TerminalFields::new(
                        status,
                        None,
                        BoundedVec::new(vec![problem])?,
                        empty_bounded()?,
                        empty_bounded()?,
                    )?,
                    TerminalStatus::Refused => TerminalFields::new(
                        status,
                        None,
                        empty_bounded()?,
                        BoundedVec::new(vec![problem])?,
                        empty_bounded()?,
                    )?,
                    TerminalStatus::Error => TerminalFields::new(
                        status,
                        None,
                        empty_bounded()?,
                        empty_bounded()?,
                        BoundedVec::new(vec![problem])?,
                    )?,
                    TerminalStatus::Ok => return Err(ContractError::InvalidInvariant),
                };
                Self::from_parts(
                    binding(USE_CASE_RESPONSE_REF)?,
                    use_case,
                    result_schema,
                    terminal,
                    receipts,
                    empty_bounded()?,
                    empty_bounded()?,
                    limits,
                )
            }
        }
    }

    pub fn repository_setup_success(
        result: RepositorySetupResultV1,
        receipts: BoundedVec<WriteReceipt, 128>,
        diagnostics: BoundedVec<Diagnostic, 128>,
        next_actions: BoundedVec<NextAction, 128>,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        if result.readiness != RepositoryReadinessStatusV1::Ready {
            return Err(ContractError::InvalidInvariant);
        }
        Self::success(
            UseCaseKind::RepositorySetup,
            &result,
            receipts,
            diagnostics,
            next_actions,
            limits,
        )
    }

    pub fn author_charter_success(
        result: AuthorCharterResultV1,
        receipts: BoundedVec<WriteReceipt, 128>,
        diagnostics: BoundedVec<Diagnostic, 128>,
        next_actions: BoundedVec<NextAction, 128>,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        Self::success(
            UseCaseKind::AuthorCharter,
            &result,
            receipts,
            diagnostics,
            next_actions,
            limits,
        )
    }

    pub fn author_charter(
        result: &CharterOperationResult,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        let receipts =
            packet3_receipts(UseCaseKind::AuthorCharter, &result.public_effects, limits)?;
        match result.status {
            AuthorOperationStatus::Succeeded => Self::author_charter_success(
                AuthorCharterResultV1::from_result(result)?,
                receipts,
                empty_bounded()?,
                empty_bounded()?,
                limits,
            ),
            AuthorOperationStatus::Refused => {
                let details = AuthorCharterDetailsV1::from_result(result)?;
                Self::packet3_problem(
                    UseCaseKind::AuthorCharter,
                    AUTHOR_CHARTER_DETAILS_REF,
                    "author_charter",
                    details.cause,
                    &details,
                    receipts,
                    limits,
                )
            }
        }
    }

    pub fn author_project_context_success(
        result: AuthorProjectContextResultV1,
        receipts: BoundedVec<WriteReceipt, 128>,
        diagnostics: BoundedVec<Diagnostic, 128>,
        next_actions: BoundedVec<NextAction, 128>,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        Self::success(
            UseCaseKind::AuthorProjectContext,
            &result,
            receipts,
            diagnostics,
            next_actions,
            limits,
        )
    }

    pub fn author_project_context(
        outcome: &ProjectContextCommandOutcome,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        let effects = match outcome {
            ProjectContextCommandOutcome::Author {
                result: Ok(result), ..
            } => result.public_effects.as_slice(),
            ProjectContextCommandOutcome::Author {
                result: Err(refusal),
                ..
            }
            | ProjectContextCommandOutcome::Validate {
                result: Err(refusal),
                ..
            } => refusal.public_effects.as_slice(),
            ProjectContextCommandOutcome::Validate { result: Ok(()), .. } => &[],
        };
        let receipts = packet3_receipts(UseCaseKind::AuthorProjectContext, effects, limits)?;
        match outcome {
            ProjectContextCommandOutcome::Author {
                input_mode,
                input_source,
                result: Ok(result),
            } => Self::author_project_context_success(
                AuthorProjectContextResultV1::author(input_mode, input_source, result)?,
                receipts,
                empty_bounded()?,
                empty_bounded()?,
                limits,
            ),
            ProjectContextCommandOutcome::Validate {
                input_mode,
                input_source,
                result: Ok(()),
            } => Self::author_project_context_success(
                AuthorProjectContextResultV1::validate(input_mode, input_source)?,
                empty_bounded()?,
                empty_bounded()?,
                empty_bounded()?,
                limits,
            ),
            _ => {
                let details = AuthorProjectContextDetailsV1::from_outcome(outcome)?;
                Self::packet3_problem(
                    UseCaseKind::AuthorProjectContext,
                    AUTHOR_PROJECT_CONTEXT_DETAILS_REF,
                    "author_project_context",
                    details.cause,
                    &details,
                    receipts,
                    limits,
                )
            }
        }
    }

    pub fn approver_administration_success(
        result: ApproverAdministrationResultV1,
        receipts: BoundedVec<WriteReceipt, 128>,
        diagnostics: BoundedVec<Diagnostic, 128>,
        next_actions: BoundedVec<NextAction, 128>,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        Self::success(
            UseCaseKind::ApproverAdministration,
            &result,
            receipts,
            diagnostics,
            next_actions,
            limits,
        )
    }

    pub fn approver_administration(
        result: &ApproverOperationResult,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        let receipts = packet3_receipts(
            UseCaseKind::ApproverAdministration,
            &result.public_effects,
            limits,
        )?;
        match result.status {
            AuthorOperationStatus::Succeeded => Self::approver_administration_success(
                ApproverAdministrationResultV1::from_result(result)?,
                receipts,
                empty_bounded()?,
                empty_bounded()?,
                limits,
            ),
            AuthorOperationStatus::Refused => {
                let details = ApproverAdministrationDetailsV1::from_result(result)?;
                Self::packet3_problem(
                    UseCaseKind::ApproverAdministration,
                    APPROVER_ADMINISTRATION_DETAILS_REF,
                    "approver_administration",
                    details.cause,
                    &details,
                    receipts,
                    limits,
                )
            }
        }
    }

    fn packet3_problem<T: Serialize>(
        kind: UseCaseKind,
        details_ref: &str,
        code_prefix: &str,
        cause: UseCaseCauseV1,
        typed_details: &T,
        receipts: BoundedVec<WriteReceipt, 128>,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        let use_case = binding(kind.use_case_ref())?;
        let result_schema = binding(kind.result_ref())?;
        let details_schema = binding(details_ref)?;
        let details = contract_value(typed_details, limits)?;
        let (status, category, retry) = packet3_disposition(cause);
        let suffix = match cause {
            UseCaseCauseV1::Prerequisite => "blocked",
            UseCaseCauseV1::CallerRefusal => "refused",
            UseCaseCauseV1::ImplementationFailure => "error",
        };
        let code = format!("{code_prefix}.{suffix}");
        let recheck = (status == TerminalStatus::Blocked)
            .then(|| RecheckCondition::new(details_schema.clone(), details.clone(), limits))
            .transpose()?;
        let rule = (status == TerminalStatus::Refused)
            .then(|| binding(&format!("handbook.use-case.{code_prefix}-rule@1.0.0")))
            .transpose()?;
        let correlation_ref = (status == TerminalStatus::Error)
            .then(|| setup_correlation_ref(&code))
            .transpose()?;
        let problem = Problem::new(
            bounded(&code)?,
            bounded(&code)?,
            category,
            use_case.clone(),
            rule,
            details_schema,
            details,
            empty_bounded()?,
            retry,
            recheck,
            correlation_ref,
            limits,
        )?;
        let terminal = match status {
            TerminalStatus::Blocked => TerminalFields::new(
                status,
                None,
                BoundedVec::new(vec![problem])?,
                empty_bounded()?,
                empty_bounded()?,
            )?,
            TerminalStatus::Refused => TerminalFields::new(
                status,
                None,
                empty_bounded()?,
                BoundedVec::new(vec![problem])?,
                empty_bounded()?,
            )?,
            TerminalStatus::Error => TerminalFields::new(
                status,
                None,
                empty_bounded()?,
                empty_bounded()?,
                BoundedVec::new(vec![problem])?,
            )?,
            TerminalStatus::Ok => return Err(ContractError::InvalidInvariant),
        };
        Self::from_parts(
            binding(USE_CASE_RESPONSE_REF)?,
            use_case,
            result_schema,
            terminal,
            receipts,
            empty_bounded()?,
            empty_bounded()?,
            limits,
        )
    }

    pub fn pipeline_route_resolve_and_persist_success(
        result: PipelineRouteResolveAndPersistResultV1,
        receipts: BoundedVec<WriteReceipt, 128>,
        diagnostics: BoundedVec<Diagnostic, 128>,
        next_actions: BoundedVec<NextAction, 128>,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        Self::success(
            UseCaseKind::PipelineRouteResolveAndPersist,
            &result,
            receipts,
            diagnostics,
            next_actions,
            limits,
        )
    }

    pub fn pipeline_route_resolve_and_persist_execution(
        execution: &PipelineResolveExecution,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        let receipts = pipeline_route_receipts(&execution.effects, limits)?;
        match &execution.result {
            Ok(outcome) => {
                let [effect] = execution.effects.as_slice() else {
                    return Err(ContractError::InvalidInvariant);
                };
                if effect.pipeline_id != outcome.pipeline_id
                    || effect.state_revision != outcome.state.revision
                    || effect.route_basis.pipeline_id != outcome.pipeline_id
                    || effect.route_basis.state_revision != outcome.state.revision
                {
                    return Err(ContractError::InvalidInvariant);
                }
                Self::pipeline_route_resolve_and_persist_success(
                    PipelineRouteResolveAndPersistResultV1::from_outcome(outcome)?,
                    receipts,
                    empty_bounded()?,
                    empty_bounded()?,
                    limits,
                )
            }
            Err(failure) => {
                let details = pipeline_route_details(failure)?;
                details.validate()?;
                let (status, category, retry) = pipeline_route_disposition(details.cause);
                let cause = pipeline_route_cause_name(details.cause);
                let details_value = contract_value(&details, limits)?;
                let details_schema = binding(PIPELINE_ROUTE_DETAILS_REF)?;
                let recheck = (status == TerminalStatus::Blocked)
                    .then(|| {
                        RecheckCondition::new(details_schema.clone(), details_value.clone(), limits)
                    })
                    .transpose()?;
                let rule = (status == TerminalStatus::Refused)
                    .then(|| binding(&format!("handbook.pipeline-rule.{cause}@1.0.0")))
                    .transpose()?;
                let correlation_ref = (status == TerminalStatus::Error)
                    .then(|| {
                        let fingerprint = canonical_fingerprint(&details, limits)?;
                        CorrelationRef::new(bounded(&format!(
                            "corr_{}",
                            fingerprint.as_str().trim_start_matches("sha256:")
                        ))?)
                    })
                    .transpose()?;
                let problem = Problem::new(
                    bounded(&format!("pipeline.route.{cause}"))?,
                    bounded(&format!("pipeline.route.{cause}"))?,
                    category,
                    binding(PIPELINE_ROUTE_USE_CASE_REF)?,
                    rule,
                    details_schema,
                    details_value,
                    empty_bounded()?,
                    retry,
                    recheck,
                    correlation_ref,
                    limits,
                )?;
                let terminal = match status {
                    TerminalStatus::Blocked => TerminalFields::new(
                        status,
                        None,
                        BoundedVec::new(vec![problem])?,
                        empty_bounded()?,
                        empty_bounded()?,
                    )?,
                    TerminalStatus::Refused => TerminalFields::new(
                        status,
                        None,
                        empty_bounded()?,
                        BoundedVec::new(vec![problem])?,
                        empty_bounded()?,
                    )?,
                    TerminalStatus::Error => TerminalFields::new(
                        status,
                        None,
                        empty_bounded()?,
                        empty_bounded()?,
                        BoundedVec::new(vec![problem])?,
                    )?,
                    TerminalStatus::Ok => return Err(ContractError::InvalidInvariant),
                };
                Self::from_parts(
                    binding(USE_CASE_RESPONSE_REF)?,
                    binding(PIPELINE_ROUTE_USE_CASE_REF)?,
                    binding(UseCaseKind::PipelineRouteResolveAndPersist.result_ref())?,
                    terminal,
                    receipts,
                    empty_bounded()?,
                    empty_bounded()?,
                    limits,
                )
            }
        }
    }

    fn success<T: Serialize>(
        kind: UseCaseKind,
        result: &T,
        receipts: BoundedVec<WriteReceipt, 128>,
        diagnostics: BoundedVec<Diagnostic, 128>,
        next_actions: BoundedVec<NextAction, 128>,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        let terminal = TerminalFields::new(
            TerminalStatus::Ok,
            Some(contract_value(result, limits)?),
            BoundedVec::new(Vec::new())?,
            BoundedVec::new(Vec::new())?,
            BoundedVec::new(Vec::new())?,
        )?;
        Self::from_parts(
            binding(USE_CASE_RESPONSE_REF)?,
            binding(kind.use_case_ref())?,
            binding(kind.result_ref())?,
            terminal,
            receipts,
            diagnostics,
            next_actions,
            limits,
        )
    }

    pub fn repository_doctor(
        report: &DoctorReport,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        let result = RepositoryDoctorResultV1::from_report(report)?;
        let use_case = binding(DOCTOR_USE_CASE_REF)?;
        let result_schema = binding(DOCTOR_RESULT_REF)?;
        let empty_problems = || BoundedVec::new(Vec::new());
        let (terminal, next_actions) = match report.status {
            RepositoryReadinessStatus::Ready => (
                TerminalFields::new(
                    TerminalStatus::Ok,
                    Some(contract_value(&result, limits)?),
                    empty_problems()?,
                    empty_problems()?,
                    empty_problems()?,
                )?,
                BoundedVec::new(Vec::new())?,
            ),
            status => {
                let (code, label) = match status {
                    RepositoryReadinessStatus::ActionRequired => (
                        "repository.action_required",
                        "Resolve repository readiness actions and rerun doctor",
                    ),
                    RepositoryReadinessStatus::Indeterminate => (
                        "repository.readiness_indeterminate",
                        "Resolve indeterminate repository evidence and rerun doctor",
                    ),
                    RepositoryReadinessStatus::Invalid => (
                        "repository.readiness_invalid",
                        "Repair invalid repository inputs and rerun doctor",
                    ),
                    RepositoryReadinessStatus::Ready => unreachable!(),
                };
                let details = contract_value(
                    &RepositoryDoctorDetailsV1::Readiness {
                        result: Box::new(result.clone()),
                    },
                    limits,
                )?;
                let details_schema = binding(DOCTOR_DETAILS_REF)?;
                let recheck =
                    RecheckCondition::new(details_schema.clone(), details.clone(), limits)?;
                let problem = Problem::new(
                    bounded(code)?,
                    bounded(code)?,
                    ProblemCategory::Prerequisite,
                    use_case.clone(),
                    None,
                    details_schema.clone(),
                    details.clone(),
                    BoundedVec::new(Vec::new())?,
                    RetryDirective::AfterRecheck,
                    Some(recheck),
                    None,
                    limits,
                )?;
                let action = NextAction::new(
                    bounded("repository.rerun_doctor")?,
                    NextActionKind::Recheck,
                    use_case.clone(),
                    details_schema,
                    details,
                    Some(bounded(label)?),
                    limits,
                )?;
                (
                    TerminalFields::new(
                        TerminalStatus::Blocked,
                        None,
                        BoundedVec::new(vec![problem])?,
                        empty_problems()?,
                        empty_problems()?,
                    )?,
                    BoundedVec::new(vec![action])?,
                )
            }
        };
        Self::from_parts(
            binding(USE_CASE_RESPONSE_REF)?,
            use_case,
            result_schema,
            terminal,
            BoundedVec::new(Vec::new())?,
            BoundedVec::new(Vec::new())?,
            next_actions,
            limits,
        )
    }

    pub fn repository_doctor_error(
        error: DoctorError,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        let use_case = binding(DOCTOR_USE_CASE_REF)?;
        let details_schema = binding(DOCTOR_DETAILS_REF)?;
        let (failure_kind, reason, status, category, retry) = match error {
            DoctorError::ShippedProfileUnavailable => (
                RepositoryDoctorFailureKindV1::ProfileResolution,
                RepositoryDoctorFailureReasonV1::ShippedProfileUnavailable,
                TerminalStatus::Blocked,
                ProblemCategory::Prerequisite,
                RetryDirective::AfterRecheck,
            ),
            DoctorError::SelectedProfileDecisionInvalid => (
                RepositoryDoctorFailureKindV1::ProfileDecision,
                RepositoryDoctorFailureReasonV1::SelectedProfileDecisionInvalid,
                TerminalStatus::Refused,
                ProblemCategory::Precondition,
                RetryDirective::AfterRequestChange,
            ),
        };
        let details = contract_value(
            &RepositoryDoctorDetailsV1::DoctorFailure {
                failure_kind,
                reason,
            },
            limits,
        )?;
        let recheck = if status == TerminalStatus::Blocked {
            Some(RecheckCondition::new(
                details_schema.clone(),
                details.clone(),
                limits,
            )?)
        } else {
            None
        };
        let rule = if status == TerminalStatus::Refused {
            Some(binding("handbook.use-case.repository-doctor-rule@1.0.0")?)
        } else {
            None
        };
        let reason_code = match error.reason_code() {
            DoctorErrorReasonCode::ShippedProfileUnavailable => {
                "repository.shipped_profile_unavailable"
            }
            DoctorErrorReasonCode::SelectedProfileDecisionInvalid => {
                "repository.selected_profile_decision_invalid"
            }
        };
        let problem = Problem::new(
            bounded(reason_code)?,
            bounded(reason_code)?,
            category,
            use_case.clone(),
            rule,
            details_schema,
            details,
            BoundedVec::new(Vec::new())?,
            retry,
            recheck,
            None,
            limits,
        )?;
        let empty = || BoundedVec::new(Vec::new());
        let terminal = match status {
            TerminalStatus::Blocked => TerminalFields::new(
                status,
                None,
                BoundedVec::new(vec![problem])?,
                empty()?,
                empty()?,
            )?,
            TerminalStatus::Refused => TerminalFields::new(
                status,
                None,
                empty()?,
                BoundedVec::new(vec![problem])?,
                empty()?,
            )?,
            _ => return Err(ContractError::InvalidInvariant),
        };
        Self::from_parts(
            binding(USE_CASE_RESPONSE_REF)?,
            use_case,
            binding(DOCTOR_RESULT_REF)?,
            terminal,
            BoundedVec::new(Vec::new())?,
            BoundedVec::new(Vec::new())?,
            BoundedVec::new(Vec::new())?,
            limits,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn from_parts(
        schema: ExactBinding,
        use_case: ExactBinding,
        result_schema: ExactBinding,
        terminal: TerminalFields<ContractValue>,
        receipts: BoundedVec<WriteReceipt, 128>,
        diagnostics: BoundedVec<Diagnostic, 128>,
        next_actions: BoundedVec<NextAction, 128>,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        if schema != binding(USE_CASE_RESPONSE_REF)? {
            return Err(ContractError::InvalidInvariant);
        }
        let kind = UseCaseKind::from_binding(&use_case)?;
        if use_case != binding(use_case.reference().as_str())?
            || result_schema != binding(kind.result_ref())?
        {
            return Err(ContractError::InvalidInvariant);
        }
        if kind == UseCaseKind::PipelineRouteResolveAndPersist && !diagnostics.as_slice().is_empty()
        {
            return Err(ContractError::InvalidInvariant);
        }
        if let Some(data) = terminal.data() {
            kind.admit_success(data, limits)?;
        }
        for problem in terminal
            .blockers()
            .iter()
            .chain(terminal.refusals())
            .chain(terminal.errors())
        {
            kind.admit_problem_details(problem, limits)?;
        }
        kind.admit_terminal(&terminal, &next_actions, limits)?;
        kind.admit_receipts(&terminal, &receipts, limits)?;
        let result_fingerprint = canonical_fingerprint(
            &UseCaseResultPreimage {
                use_case: &use_case,
                result_schema: &result_schema,
                terminal: &terminal,
                receipts: &receipts,
            },
            limits,
        )?;
        let response_fingerprint = canonical_fingerprint(
            &UseCaseResponsePreimage {
                schema: &schema,
                use_case: &use_case,
                result_schema: &result_schema,
                terminal: &terminal,
                receipts: &receipts,
                diagnostics: &diagnostics,
                next_actions: &next_actions,
                result_fingerprint: &result_fingerprint,
            },
            limits,
        )?;
        let value = Self {
            schema,
            use_case,
            result_schema,
            terminal,
            receipts,
            diagnostics,
            next_actions,
            result_fingerprint,
            response_fingerprint,
        };
        canonical_json_bytes(&value, limits)?;
        Ok(value)
    }

    pub fn schema(&self) -> &ExactBinding {
        &self.schema
    }

    pub fn use_case(&self) -> &ExactBinding {
        &self.use_case
    }

    pub fn result_schema(&self) -> &ExactBinding {
        &self.result_schema
    }

    pub fn status(&self) -> TerminalStatus {
        self.terminal.status()
    }

    pub fn terminal(&self) -> &TerminalFields<ContractValue> {
        &self.terminal
    }

    pub fn terminal_problems(&self) -> &[Problem] {
        match self.status() {
            TerminalStatus::Ok => &[],
            TerminalStatus::Blocked => self.terminal.blockers(),
            TerminalStatus::Refused => self.terminal.refusals(),
            TerminalStatus::Error => self.terminal.errors(),
        }
    }

    pub fn repository_doctor_outcome(
        &self,
        limits: &ContractLimits,
    ) -> Result<Option<Result<DoctorReport, DoctorError>>, ContractError> {
        if self.use_case.reference().as_str() != DOCTOR_USE_CASE_REF {
            return Ok(None);
        }
        if self.status() == TerminalStatus::Ok {
            let data = self
                .terminal
                .data()
                .ok_or(ContractError::InvalidInvariant)?;
            let result: RepositoryDoctorResultV1 = admit_contract_value(data, limits)?;
            return Ok(Some(Ok(result.to_report())));
        }
        let [problem] = self.terminal_problems() else {
            return Err(ContractError::InvalidInvariant);
        };
        let details: RepositoryDoctorDetailsV1 = admit_contract_value(problem.details(), limits)?;
        match details {
            RepositoryDoctorDetailsV1::Readiness { result } => Ok(Some(Ok(result.to_report()))),
            RepositoryDoctorDetailsV1::DoctorFailure {
                failure_kind,
                reason,
            } => Ok(Some(Err(doctor_error(failure_kind, reason)?))),
        }
    }

    pub fn pipeline_route_resolve_and_persist_outcome(
        &self,
        limits: &ContractLimits,
    ) -> Result<
        Option<Result<PipelineResolveOutcome, PipelineResolveExecutionFailure>>,
        ContractError,
    > {
        if self.use_case.reference().as_str() != PIPELINE_ROUTE_USE_CASE_REF {
            return Ok(None);
        }
        if self.status() == TerminalStatus::Ok {
            let result: PipelineRouteResolveAndPersistResultV1 = admit_contract_value(
                self.terminal
                    .data()
                    .ok_or(ContractError::InvalidInvariant)?,
                limits,
            )?;
            return Ok(Some(Ok(result.to_outcome())));
        }
        let [problem] = self.terminal_problems() else {
            return Err(ContractError::InvalidInvariant);
        };
        let details: PipelineRouteResolveDetailsV1 =
            admit_contract_value(problem.details(), limits)?;
        details.validate()?;
        Ok(Some(Err(pipeline_route_failure(&details)?)))
    }

    pub fn repository_setup_outcome(
        &self,
        limits: &ContractLimits,
    ) -> Result<Option<Result<SetupOutcome, SetupError>>, ContractError> {
        if self.use_case.reference().as_str() != SETUP_USE_CASE_REF {
            return Ok(None);
        }
        if self.status() == TerminalStatus::Ok {
            let data = self
                .terminal
                .data()
                .ok_or(ContractError::InvalidInvariant)?;
            let result: RepositorySetupResultV1 = admit_contract_value(data, limits)?;
            return Ok(Some(Ok(result.to_outcome())));
        }
        let [problem] = self.terminal_problems() else {
            return Err(ContractError::InvalidInvariant);
        };
        let details: RepositorySetupDetailsV1 = admit_contract_value(problem.details(), limits)?;
        match details {
            RepositorySetupDetailsV1::Readiness { result } => Ok(Some(Ok(result.to_outcome()))),
            details @ RepositorySetupDetailsV1::SetupFailure { .. } => Ok(Some(Err(details
                .to_error()?
                .ok_or(ContractError::InvalidInvariant)?))),
        }
    }

    pub fn author_charter_outcome(
        &self,
        limits: &ContractLimits,
    ) -> Result<Option<CharterOperationResult>, ContractError> {
        if self.use_case.reference().as_str() != "handbook.use-case.author-charter@1.0.0" {
            return Ok(None);
        }
        if self.status() == TerminalStatus::Ok {
            let data = self
                .terminal
                .data()
                .ok_or(ContractError::InvalidInvariant)?;
            let result: AuthorCharterResultV1 = admit_contract_value(data, limits)?;
            return Ok(Some(result.to_result()));
        }
        let [problem] = self.terminal_problems() else {
            return Err(ContractError::InvalidInvariant);
        };
        let details: AuthorCharterDetailsV1 = admit_contract_value(problem.details(), limits)?;
        Ok(Some(details.to_result()))
    }

    pub fn author_project_context_outcome(
        &self,
        limits: &ContractLimits,
    ) -> Result<Option<ProjectContextCommandOutcome>, ContractError> {
        if self.use_case.reference().as_str() != "handbook.use-case.author-project-context@1.0.0" {
            return Ok(None);
        }
        if self.status() == TerminalStatus::Ok {
            let data = self
                .terminal
                .data()
                .ok_or(ContractError::InvalidInvariant)?;
            let result: AuthorProjectContextResultV1 = admit_contract_value(data, limits)?;
            return Ok(Some(result.to_outcome()));
        }
        let [problem] = self.terminal_problems() else {
            return Err(ContractError::InvalidInvariant);
        };
        let details: AuthorProjectContextDetailsV1 =
            admit_contract_value(problem.details(), limits)?;
        Ok(Some(details.to_outcome()))
    }

    pub fn approver_administration_outcome(
        &self,
        limits: &ContractLimits,
    ) -> Result<Option<ApproverOperationResult>, ContractError> {
        if self.use_case.reference().as_str() != "handbook.use-case.approver-administration@1.0.0" {
            return Ok(None);
        }
        if self.status() == TerminalStatus::Ok {
            let data = self
                .terminal
                .data()
                .ok_or(ContractError::InvalidInvariant)?;
            let result: ApproverAdministrationResultV1 = admit_contract_value(data, limits)?;
            return Ok(Some(result.to_result()));
        }
        let [problem] = self.terminal_problems() else {
            return Err(ContractError::InvalidInvariant);
        };
        let details: ApproverAdministrationDetailsV1 =
            admit_contract_value(problem.details(), limits)?;
        Ok(Some(details.to_result()))
    }

    pub fn receipts(&self) -> &[WriteReceipt] {
        self.receipts.as_slice()
    }

    pub fn diagnostics(&self) -> &[Diagnostic] {
        self.diagnostics.as_slice()
    }

    pub fn next_actions(&self) -> &[NextAction] {
        self.next_actions.as_slice()
    }

    pub fn result_fingerprint(&self) -> &Fingerprint {
        &self.result_fingerprint
    }

    pub fn response_fingerprint(&self) -> &Fingerprint {
        &self.response_fingerprint
    }
}

impl<'de> Deserialize<'de> for UseCaseResponseV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize, Serialize)]
        #[serde(deny_unknown_fields)]
        struct Raw {
            schema: ExactBinding,
            use_case: ExactBinding,
            result_schema: ExactBinding,
            #[serde(flatten)]
            terminal: TerminalFields<ContractValue>,
            receipts: BoundedVec<WriteReceipt, 128>,
            diagnostics: BoundedVec<Diagnostic, 128>,
            next_actions: BoundedVec<NextAction, 128>,
            result_fingerprint: Fingerprint,
            response_fingerprint: Fingerprint,
        }
        let raw = Raw::deserialize(deserializer).map_err(|error| {
            D::Error::custom(classify_contract_error_message(&error.to_string()))
        })?;
        let value = Self::from_parts(
            raw.schema,
            raw.use_case,
            raw.result_schema,
            raw.terminal,
            raw.receipts,
            raw.diagnostics,
            raw.next_actions,
            &ContractLimits::packet0(),
        )
        .map_err(D::Error::custom)?;
        if value.result_fingerprint != raw.result_fingerprint
            || value.response_fingerprint != raw.response_fingerprint
        {
            return Err(D::Error::custom(ContractError::FingerprintMismatch));
        }
        Ok(value)
    }
}

#[derive(Serialize)]
struct UseCaseResultPreimage<'a> {
    use_case: &'a ExactBinding,
    result_schema: &'a ExactBinding,
    #[serde(flatten)]
    terminal: &'a TerminalFields<ContractValue>,
    receipts: &'a BoundedVec<WriteReceipt, 128>,
}

#[derive(Serialize)]
struct UseCaseResponsePreimage<'a> {
    schema: &'a ExactBinding,
    use_case: &'a ExactBinding,
    result_schema: &'a ExactBinding,
    #[serde(flatten)]
    terminal: &'a TerminalFields<ContractValue>,
    receipts: &'a BoundedVec<WriteReceipt, 128>,
    diagnostics: &'a BoundedVec<Diagnostic, 128>,
    next_actions: &'a BoundedVec<NextAction, 128>,
    result_fingerprint: &'a Fingerprint,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct TransportFailureV1 {
    schema: ExactBinding,
    failure_kind: TransportFailureKind,
    #[serde(flatten)]
    terminal: TerminalFields<ContractValue>,
    diagnostics: BoundedVec<Diagnostic, 128>,
    next_actions: BoundedVec<NextAction, 128>,
    response_fingerprint: Fingerprint,
}

impl TransportFailureV1 {
    pub fn adapter_error(
        failure_kind: TransportFailureKind,
        reason_code: &str,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        Self::adapter_error_with_presentation(failure_kind, reason_code, None, limits)
    }

    pub fn author_input_error(
        failure_kind: TransportFailureKind,
        reason_code: &str,
        route: AuthorInputTransportRouteV1,
        input_source: &str,
        error: Option<TransportIoErrorV1>,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        let exact_kind = match (&error, failure_kind) {
            (None, TransportFailureKind::Decoding) => true,
            (Some(error), TransportFailureKind::Acquisition) => error.is_caller_acquisition(),
            (Some(error), TransportFailureKind::Adapter) => !error.is_caller_acquisition(),
            _ => false,
        };
        if !exact_kind {
            return Err(ContractError::InvalidInvariant);
        }
        Self::adapter_error_with_presentation(
            failure_kind,
            reason_code,
            Some(TransportPresentationV1::AuthorInput {
                route,
                input_source: bounded(input_source)?,
                error,
            }),
            limits,
        )
    }

    pub fn repository_invocation_error(
        reason_code: &str,
        command: RepositoryInvocationCommandV1,
        failure: &RepositoryInvocationFailureView,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        let failure_kind = match failure.causal_disposition {
            AdapterCausalDisposition::Prerequisite => TransportFailureKind::Repository,
            AdapterCausalDisposition::CallerRefusal => TransportFailureKind::Acquisition,
            AdapterCausalDisposition::ImplementationFailure => TransportFailureKind::Adapter,
        };
        Self::adapter_error_with_presentation(
            failure_kind,
            reason_code,
            Some(TransportPresentationV1::RepositoryInvocation {
                command,
                failure: Box::new(RepositoryInvocationFailureDetailsV1::from_view(failure)?),
            }),
            limits,
        )
    }

    pub fn working_directory_error(
        failure_kind: TransportFailureKind,
        reason_code: &str,
        command: AuthorWorkingDirectoryCommandV1,
        error: TransportIoErrorV1,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        if failure_kind != TransportFailureKind::Adapter {
            return Err(ContractError::InvalidInvariant);
        }
        Self::adapter_error_with_presentation(
            failure_kind,
            reason_code,
            Some(TransportPresentationV1::WorkingDirectory { command, error }),
            limits,
        )
    }

    fn adapter_error_with_presentation(
        failure_kind: TransportFailureKind,
        reason_code: &str,
        presentation: Option<TransportPresentationV1>,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        let details_schema = binding(TRANSPORT_DETAILS_REF)?;
        let details = contract_value(
            &TransportFailureDetailsV1 {
                failure_kind,
                reason_code: bounded(reason_code)?,
                presentation,
            },
            limits,
        )?;
        let digest = sha256_fingerprint(reason_code.as_bytes());
        let (status, category, retry) = transport_disposition(failure_kind);
        let rule = if status == TerminalStatus::Refused {
            Some(binding("handbook.transport-failure-rule@1.0.0")?)
        } else {
            None
        };
        let correlation = if status == TerminalStatus::Error {
            Some(CorrelationRef::new(BoundedString::new(format!(
                "corr_{}",
                digest.as_str().trim_start_matches("sha256:")
            ))?)?)
        } else {
            None
        };
        let recheck = if status == TerminalStatus::Blocked {
            Some(RecheckCondition::new(
                details_schema.clone(),
                details.clone(),
                limits,
            )?)
        } else {
            None
        };
        let problem = Problem::new(
            bounded(reason_code)?,
            bounded(reason_code)?,
            category,
            binding(TRANSPORT_FAILURE_REF)?,
            rule,
            details_schema,
            details,
            BoundedVec::new(Vec::new())?,
            retry,
            recheck,
            correlation,
            limits,
        )?;
        let (blockers, refusals, errors) = match status {
            TerminalStatus::Blocked => (
                BoundedVec::new(vec![problem])?,
                BoundedVec::new(Vec::new())?,
                BoundedVec::new(Vec::new())?,
            ),
            TerminalStatus::Refused => (
                BoundedVec::new(Vec::new())?,
                BoundedVec::new(vec![problem])?,
                BoundedVec::new(Vec::new())?,
            ),
            TerminalStatus::Error => (
                BoundedVec::new(Vec::new())?,
                BoundedVec::new(Vec::new())?,
                BoundedVec::new(vec![problem])?,
            ),
            TerminalStatus::Ok => return Err(ContractError::InvalidInvariant),
        };
        Self::new(
            failure_kind,
            status,
            blockers,
            refusals,
            errors,
            BoundedVec::new(Vec::new())?,
            limits,
        )
    }

    pub fn new(
        failure_kind: TransportFailureKind,
        status: TerminalStatus,
        blockers: BoundedVec<Problem, 128>,
        refusals: BoundedVec<Problem, 128>,
        errors: BoundedVec<Problem, 128>,
        diagnostics: BoundedVec<Diagnostic, 128>,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        if status == TerminalStatus::Ok {
            return Err(ContractError::InvalidInvariant);
        }
        let terminal = TerminalFields::new(status, None, blockers, refusals, errors)?;
        Self::from_parts(
            binding(TRANSPORT_FAILURE_REF)?,
            failure_kind,
            terminal,
            diagnostics,
            BoundedVec::new(Vec::new())?,
            limits,
        )
    }

    fn from_parts(
        schema: ExactBinding,
        failure_kind: TransportFailureKind,
        terminal: TerminalFields<ContractValue>,
        diagnostics: BoundedVec<Diagnostic, 128>,
        next_actions: BoundedVec<NextAction, 128>,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        let (expected_status, expected_category, expected_retry) =
            transport_disposition(failure_kind);
        if schema != binding(TRANSPORT_FAILURE_REF)?
            || terminal.status() != expected_status
            || terminal.blockers().len() + terminal.refusals().len() + terminal.errors().len() != 1
        {
            return Err(ContractError::InvalidInvariant);
        }
        for problem in terminal
            .blockers()
            .iter()
            .chain(terminal.refusals())
            .chain(terminal.errors())
        {
            if problem.subject() != &binding(TRANSPORT_FAILURE_REF)?
                || problem.details_schema() != &binding(TRANSPORT_DETAILS_REF)?
                || problem.category() != expected_category
                || problem.retry() != expected_retry
            {
                return Err(ContractError::InvalidInvariant);
            }
            let details: TransportFailureDetailsV1 =
                admit_contract_value(problem.details(), limits)?;
            let presentation_matches = match &details.presentation {
                None => true,
                Some(TransportPresentationV1::AuthorInput {
                    route,
                    input_source: _,
                    error,
                }) => author_input_presentation_is_exact(
                    failure_kind,
                    details.reason_code.as_str(),
                    *route,
                    error.as_ref(),
                ),
                Some(TransportPresentationV1::WorkingDirectory { .. }) => {
                    failure_kind == TransportFailureKind::Adapter
                        && details.reason_code.as_str() == "transport.repository_root_unavailable"
                }
                Some(TransportPresentationV1::RepositoryInvocation { command, failure }) => {
                    repository_invocation_presentation_is_exact(
                        failure_kind,
                        details.reason_code.as_str(),
                        *command,
                        failure,
                    )
                }
            };
            if details.failure_kind != failure_kind || !presentation_matches {
                return Err(ContractError::InvalidInvariant);
            }
        }
        let response_fingerprint = canonical_fingerprint(
            &TransportFailurePreimage {
                schema: &schema,
                failure_kind,
                terminal: &terminal,
                diagnostics: &diagnostics,
                next_actions: &next_actions,
            },
            limits,
        )?;
        Ok(Self {
            schema,
            failure_kind,
            terminal,
            diagnostics,
            next_actions,
            response_fingerprint,
        })
    }

    pub fn status(&self) -> TerminalStatus {
        self.terminal.status()
    }

    pub fn failure_kind(&self) -> TransportFailureKind {
        self.failure_kind
    }

    pub fn reason_code(&self, limits: &ContractLimits) -> Result<String, ContractError> {
        let problems = match self.status() {
            TerminalStatus::Blocked => self.terminal.blockers(),
            TerminalStatus::Refused => self.terminal.refusals(),
            TerminalStatus::Error => self.terminal.errors(),
            TerminalStatus::Ok => return Err(ContractError::InvalidInvariant),
        };
        let [problem] = problems else {
            return Err(ContractError::InvalidInvariant);
        };
        let details: TransportFailureDetailsV1 = admit_contract_value(problem.details(), limits)?;
        Ok(details.reason_code.as_str().to_owned())
    }

    pub fn presentation(
        &self,
        limits: &ContractLimits,
    ) -> Result<Option<TransportPresentationViewV1>, ContractError> {
        let problems = match self.status() {
            TerminalStatus::Blocked => self.terminal.blockers(),
            TerminalStatus::Refused => self.terminal.refusals(),
            TerminalStatus::Error => self.terminal.errors(),
            TerminalStatus::Ok => return Err(ContractError::InvalidInvariant),
        };
        let [problem] = problems else {
            return Err(ContractError::InvalidInvariant);
        };
        let details: TransportFailureDetailsV1 = admit_contract_value(problem.details(), limits)?;
        Ok(details
            .presentation
            .as_ref()
            .map(|presentation| presentation.to_view(self.failure_kind)))
    }
}

fn transport_cause(failure_kind: TransportFailureKind) -> AdapterCausalDisposition {
    match failure_kind {
        TransportFailureKind::Repository => AdapterCausalDisposition::Prerequisite,
        TransportFailureKind::Acquisition | TransportFailureKind::Decoding => {
            AdapterCausalDisposition::CallerRefusal
        }
        TransportFailureKind::Framing
        | TransportFailureKind::Serialization
        | TransportFailureKind::Emission
        | TransportFailureKind::Adapter => AdapterCausalDisposition::ImplementationFailure,
    }
}

fn author_input_presentation_is_exact(
    failure_kind: TransportFailureKind,
    reason_code: &str,
    route: AuthorInputTransportRouteV1,
    error: Option<&TransportIoErrorV1>,
) -> bool {
    let expected_reason = match (route, error.is_some()) {
        (AuthorInputTransportRouteV1::Charter, true) => {
            "transport.author_charter_input_unavailable"
        }
        (AuthorInputTransportRouteV1::Charter, false) => "transport.author_charter_input_invalid",
        (AuthorInputTransportRouteV1::ProjectContext, true) => {
            "transport.author_project_context_input_unavailable"
        }
        (AuthorInputTransportRouteV1::ProjectContext, false) => {
            "transport.author_project_context_input_invalid"
        }
    };
    reason_code == expected_reason
        && match (failure_kind, error) {
            (TransportFailureKind::Decoding, None) => true,
            (TransportFailureKind::Acquisition, Some(error)) => error.is_caller_acquisition(),
            (TransportFailureKind::Adapter, Some(error)) => !error.is_caller_acquisition(),
            _ => false,
        }
}

fn repository_invocation_presentation_is_exact(
    failure_kind: TransportFailureKind,
    reason_code: &str,
    command: RepositoryInvocationCommandV1,
    failure: &RepositoryInvocationFailureDetailsV1,
) -> bool {
    let operation_matches = matches!(
        (command, failure.operation.as_str()),
        (
            RepositoryInvocationCommandV1::CharterApprove,
            "charter_approval"
        ) | (
            RepositoryInvocationCommandV1::ApproverBootstrap,
            "bootstrap"
        ) | (
            RepositoryInvocationCommandV1::ApproverAddCredential,
            "add_credential"
        ) | (
            RepositoryInvocationCommandV1::ApproverRevokeCredential,
            "revoke_credential"
        ) | (
            RepositoryInvocationCommandV1::ApproverUpdateMapping,
            "update_mapping"
        )
    );
    if reason_code != "transport.repository_invocation_unavailable"
        || !operation_matches
        || failure.schema_version.as_str() != "1.0"
        || failure.status.as_str() != "refused"
        || !failure.changed_paths.as_slice().is_empty()
        || failure.operation_id.is_some()
        || failure.next_actions.as_slice().len() != 1
    {
        return false;
    }

    let (stage, code, message, retryable, action) = match (
        failure.schema_id.as_str(),
        failure.stage.as_ref().map(BoundedString::as_str),
    ) {
        (
            "handbook.repository-invocation-preflight-result",
            Some("repository_identity"),
        ) if matches!(
            failure_kind,
            TransportFailureKind::Repository
                | TransportFailureKind::Acquisition
                | TransportFailureKind::Adapter
        )
            && failure.repository_identity_fingerprint.is_none() => (
            "repository_identity",
            "repository_identity_unavailable",
            "repository invocation identity is unavailable",
            false,
            "run or repair handbook setup, then retry the complete operation",
        ),
        ("handbook.repository-invocation-preflight-result", Some("operation_id"))
            if failure_kind == TransportFailureKind::Adapter
                && failure.repository_identity_fingerprint.is_some() => (
            "operation_id",
            "operation_id_entropy_unavailable",
            "operating-system randomness was unavailable for operation ID allocation",
            true,
            "retry the complete operation with fresh operating-system randomness",
        ),
        ("handbook.repository-invocation-recovery-refusal", None)
            if matches!(
                failure_kind,
                TransportFailureKind::Repository
                    | TransportFailureKind::Acquisition
                    | TransportFailureKind::Adapter
            ) && failure.repository_identity_fingerprint.is_none() => (
            "",
            "authority_recovery_blocked",
            "repository authority recovery could not complete safely",
            false,
            "repair retained registry/approval recovery evidence, then retry the complete operation",
        ),
        _ => return false,
    };
    let observed_stage = failure
        .stage
        .as_ref()
        .map(BoundedString::as_str)
        .unwrap_or_default();
    observed_stage == stage
        && failure.refusal.code.as_str() == code
        && failure.refusal.message.as_str() == message
        && failure.refusal.retryable == retryable
        && failure.next_actions.as_slice()[0].as_str() == action
}

fn transport_disposition(
    failure_kind: TransportFailureKind,
) -> (TerminalStatus, ProblemCategory, RetryDirective) {
    match failure_kind {
        TransportFailureKind::Repository => (
            TerminalStatus::Blocked,
            ProblemCategory::Prerequisite,
            RetryDirective::AfterRecheck,
        ),
        TransportFailureKind::Acquisition | TransportFailureKind::Decoding => (
            TerminalStatus::Refused,
            ProblemCategory::Precondition,
            RetryDirective::AfterRequestChange,
        ),
        TransportFailureKind::Framing
        | TransportFailureKind::Serialization
        | TransportFailureKind::Emission
        | TransportFailureKind::Adapter => (
            TerminalStatus::Error,
            ProblemCategory::Adapter,
            RetryDirective::Never,
        ),
    }
}

impl<'de> Deserialize<'de> for TransportFailureV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Raw {
            schema: ExactBinding,
            failure_kind: TransportFailureKind,
            #[serde(flatten)]
            terminal: TerminalFields<ContractValue>,
            diagnostics: BoundedVec<Diagnostic, 128>,
            next_actions: BoundedVec<NextAction, 128>,
            response_fingerprint: Fingerprint,
        }
        let raw = Raw::deserialize(deserializer).map_err(|error| {
            D::Error::custom(classify_contract_error_message(&error.to_string()))
        })?;
        let value = Self::from_parts(
            raw.schema,
            raw.failure_kind,
            raw.terminal,
            raw.diagnostics,
            raw.next_actions,
            &ContractLimits::packet0(),
        )
        .map_err(D::Error::custom)?;
        if value.response_fingerprint != raw.response_fingerprint {
            return Err(D::Error::custom(ContractError::FingerprintMismatch));
        }
        Ok(value)
    }
}

#[derive(Serialize)]
struct TransportFailurePreimage<'a> {
    schema: &'a ExactBinding,
    failure_kind: TransportFailureKind,
    #[serde(flatten)]
    terminal: &'a TerminalFields<ContractValue>,
    diagnostics: &'a BoundedVec<Diagnostic, 128>,
    next_actions: &'a BoundedVec<NextAction, 128>,
}

closed_enum!(InvocationResponseKind {
    Operation,
    UseCase,
    TransportFailure,
});

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(untagged)]
enum InvocationResponseBodyV1 {
    Operation(OperationBodyV1),
    UseCase(Box<UseCaseResponseV1>),
    TransportFailure(Box<TransportFailureV1>),
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(untagged)]
enum OperationBodyV1 {
    Response(Box<OperationResponseV1>),
    AdmissionRefusal(Box<OperationAdmissionRefusalV1>),
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct InvocationResponseV1 {
    schema: ExactBinding,
    response_kind: InvocationResponseKind,
    response: InvocationResponseBodyV1,
    response_fingerprint: Fingerprint,
}

impl InvocationResponseV1 {
    pub fn use_case(
        response: UseCaseResponseV1,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        Self::from_body(
            binding(INVOCATION_RESPONSE_REF)?,
            InvocationResponseKind::UseCase,
            InvocationResponseBodyV1::UseCase(Box::new(response)),
            limits,
        )
    }

    pub fn transport_failure(
        response: TransportFailureV1,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        Self::from_body(
            binding(INVOCATION_RESPONSE_REF)?,
            InvocationResponseKind::TransportFailure,
            InvocationResponseBodyV1::TransportFailure(Box::new(response)),
            limits,
        )
    }

    pub fn operation_response(
        response: OperationResponseV1,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        Self::from_body(
            binding(INVOCATION_RESPONSE_REF)?,
            InvocationResponseKind::Operation,
            InvocationResponseBodyV1::Operation(OperationBodyV1::Response(Box::new(response))),
            limits,
        )
    }

    pub fn operation_admission_refusal(
        response: OperationAdmissionRefusalV1,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        Self::from_body(
            binding(INVOCATION_RESPONSE_REF)?,
            InvocationResponseKind::Operation,
            InvocationResponseBodyV1::Operation(OperationBodyV1::AdmissionRefusal(Box::new(
                response,
            ))),
            limits,
        )
    }

    fn from_body(
        schema: ExactBinding,
        response_kind: InvocationResponseKind,
        response: InvocationResponseBodyV1,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        if schema != binding(INVOCATION_RESPONSE_REF)?
            || !matches!(
                (response_kind, &response),
                (
                    InvocationResponseKind::Operation,
                    InvocationResponseBodyV1::Operation(_)
                ) | (
                    InvocationResponseKind::UseCase,
                    InvocationResponseBodyV1::UseCase(_)
                ) | (
                    InvocationResponseKind::TransportFailure,
                    InvocationResponseBodyV1::TransportFailure(_)
                )
            )
        {
            return Err(ContractError::InvalidInvariant);
        }
        let response_fingerprint = canonical_fingerprint(
            &InvocationResponsePreimage {
                schema: &schema,
                response_kind,
                response: &response,
            },
            limits,
        )?;
        let value = Self {
            schema,
            response_kind,
            response,
            response_fingerprint,
        };
        canonical_json_bytes(&value, limits)?;
        Ok(value)
    }

    pub fn parse_and_admit(
        bytes: &[u8],
        ordinary: Option<(&OperationDefinition, &dyn OperationSchemaValidator)>,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        #[derive(Deserialize, Serialize)]
        #[serde(deny_unknown_fields)]
        struct Raw {
            schema: ExactBinding,
            response_kind: InvocationResponseKind,
            response: ContractValue,
            response_fingerprint: Fingerprint,
        }
        let raw = parse_canonical_json::<Raw>(bytes, limits)?;
        let body_bytes = canonical_json_bytes(&raw.response, limits)?;
        let response = match raw.response_kind {
            InvocationResponseKind::UseCase => {
                InvocationResponseBodyV1::UseCase(Box::new(parse_canonical_json::<
                    UseCaseResponseV1,
                >(&body_bytes, limits)?))
            }
            InvocationResponseKind::TransportFailure => {
                InvocationResponseBodyV1::TransportFailure(Box::new(parse_canonical_json::<
                    TransportFailureV1,
                >(
                    &body_bytes, limits
                )?))
            }
            InvocationResponseKind::Operation => {
                if let Some((definition, validator)) = ordinary {
                    if let Ok(response) = OperationResponseV1::parse_and_admit(
                        &body_bytes,
                        definition,
                        validator,
                        limits,
                    ) {
                        InvocationResponseBodyV1::Operation(OperationBodyV1::Response(Box::new(
                            response,
                        )))
                    } else {
                        InvocationResponseBodyV1::Operation(OperationBodyV1::AdmissionRefusal(
                            Box::new(parse_canonical_json(&body_bytes, limits)?),
                        ))
                    }
                } else {
                    InvocationResponseBodyV1::Operation(OperationBodyV1::AdmissionRefusal(
                        Box::new(parse_canonical_json(&body_bytes, limits)?),
                    ))
                }
            }
        };
        let value = Self::from_body(raw.schema, raw.response_kind, response, limits)?;
        if value.response_fingerprint != raw.response_fingerprint {
            return Err(ContractError::FingerprintMismatch);
        }
        Ok(value)
    }

    pub fn response_kind(&self) -> InvocationResponseKind {
        self.response_kind
    }

    pub(crate) fn use_case_binding(&self) -> Option<&ExactBinding> {
        match &self.response {
            InvocationResponseBodyV1::UseCase(response) => Some(response.use_case()),
            _ => None,
        }
    }

    pub(crate) fn operation_request_binding(
        &self,
    ) -> Option<(&ExactBinding, &Fingerprint, Option<&RequestId>)> {
        match &self.response {
            InvocationResponseBodyV1::Operation(OperationBodyV1::Response(response)) => Some((
                response.operation(),
                response.request_fingerprint(),
                response.request_id(),
            )),
            _ => None,
        }
    }

    pub fn schema(&self) -> &ExactBinding {
        &self.schema
    }

    pub fn response_fingerprint(&self) -> &Fingerprint {
        &self.response_fingerprint
    }

    pub fn terminal_status(&self) -> TerminalStatus {
        match &self.response {
            InvocationResponseBodyV1::Operation(OperationBodyV1::Response(response)) => {
                response.terminal().status()
            }
            InvocationResponseBodyV1::Operation(OperationBodyV1::AdmissionRefusal(response)) => {
                response.terminal().status()
            }
            InvocationResponseBodyV1::UseCase(response) => response.status(),
            InvocationResponseBodyV1::TransportFailure(response) => response.status(),
        }
    }

    pub fn repository_doctor_outcome(
        &self,
        limits: &ContractLimits,
    ) -> Result<Option<Result<DoctorReport, DoctorError>>, ContractError> {
        match &self.response {
            InvocationResponseBodyV1::UseCase(response) => {
                response.repository_doctor_outcome(limits)
            }
            _ => Ok(None),
        }
    }

    pub fn repository_setup_outcome(
        &self,
        limits: &ContractLimits,
    ) -> Result<Option<Result<SetupOutcome, SetupError>>, ContractError> {
        match &self.response {
            InvocationResponseBodyV1::UseCase(response) => {
                response.repository_setup_outcome(limits)
            }
            _ => Ok(None),
        }
    }

    pub fn pipeline_route_resolve_and_persist_outcome(
        &self,
        limits: &ContractLimits,
    ) -> Result<
        Option<Result<PipelineResolveOutcome, PipelineResolveExecutionFailure>>,
        ContractError,
    > {
        match &self.response {
            InvocationResponseBodyV1::UseCase(response) => {
                response.pipeline_route_resolve_and_persist_outcome(limits)
            }
            _ => Ok(None),
        }
    }

    pub fn author_charter_outcome(
        &self,
        limits: &ContractLimits,
    ) -> Result<Option<CharterOperationResult>, ContractError> {
        match &self.response {
            InvocationResponseBodyV1::UseCase(response) => response.author_charter_outcome(limits),
            _ => Ok(None),
        }
    }

    pub fn author_project_context_outcome(
        &self,
        limits: &ContractLimits,
    ) -> Result<Option<ProjectContextCommandOutcome>, ContractError> {
        match &self.response {
            InvocationResponseBodyV1::UseCase(response) => {
                response.author_project_context_outcome(limits)
            }
            _ => Ok(None),
        }
    }

    pub fn approver_administration_outcome(
        &self,
        limits: &ContractLimits,
    ) -> Result<Option<ApproverOperationResult>, ContractError> {
        match &self.response {
            InvocationResponseBodyV1::UseCase(response) => {
                response.approver_administration_outcome(limits)
            }
            _ => Ok(None),
        }
    }

    pub fn transport_failure_reason_code(
        &self,
        limits: &ContractLimits,
    ) -> Result<Option<String>, ContractError> {
        match &self.response {
            InvocationResponseBodyV1::TransportFailure(response) => {
                response.reason_code(limits).map(Some)
            }
            _ => Ok(None),
        }
    }

    pub fn transport_failure_kind(&self) -> Option<TransportFailureKind> {
        match &self.response {
            InvocationResponseBodyV1::TransportFailure(response) => Some(response.failure_kind()),
            _ => None,
        }
    }

    pub fn transport_presentation(
        &self,
        limits: &ContractLimits,
    ) -> Result<Option<TransportPresentationViewV1>, ContractError> {
        match &self.response {
            InvocationResponseBodyV1::TransportFailure(response) => response.presentation(limits),
            _ => Ok(None),
        }
    }
}

#[derive(Serialize)]
struct InvocationResponsePreimage<'a> {
    schema: &'a ExactBinding,
    response_kind: InvocationResponseKind,
    response: &'a InvocationResponseBodyV1,
}

impl From<DoctorErrorKind> for RepositoryDoctorFailureKindV1 {
    fn from(value: DoctorErrorKind) -> Self {
        match value {
            DoctorErrorKind::ProfileResolution => Self::ProfileResolution,
            DoctorErrorKind::ProfileDecision => Self::ProfileDecision,
        }
    }
}

fn doctor_error(
    failure_kind: RepositoryDoctorFailureKindV1,
    reason: RepositoryDoctorFailureReasonV1,
) -> Result<DoctorError, ContractError> {
    match (failure_kind, reason) {
        (
            RepositoryDoctorFailureKindV1::ProfileResolution,
            RepositoryDoctorFailureReasonV1::ShippedProfileUnavailable,
        ) => Ok(DoctorError::ShippedProfileUnavailable),
        (
            RepositoryDoctorFailureKindV1::ProfileDecision,
            RepositoryDoctorFailureReasonV1::SelectedProfileDecisionInvalid,
        ) => Ok(DoctorError::SelectedProfileDecisionInvalid),
        _ => Err(ContractError::InvalidInvariant),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::charter_product::{ApproverAdminLegacyFields, ApproverLegacyProjection};
    use std::collections::BTreeMap;
    use std::fs;
    use tempfile::tempdir;

    fn fingerprint(label: &str) -> String {
        sha256_fingerprint(label.as_bytes()).as_str().to_owned()
    }

    fn repository_failure(operation: &str) -> RepositoryInvocationFailureView {
        RepositoryInvocationFailureView {
            schema_id: "handbook.repository-invocation-preflight-result".to_owned(),
            schema_version: "1.0".to_owned(),
            operation: operation.to_owned(),
            stage: Some("repository_identity".to_owned()),
            status: "refused".to_owned(),
            repository_identity_fingerprint: None,
            operation_id: None,
            changed_paths: Vec::new(),
            refusal: AuthorOperationRefusal {
                code: "repository_identity_unavailable".to_owned(),
                message: "repository invocation identity is unavailable".to_owned(),
                retryable: false,
            },
            next_actions: vec![
                "run or repair handbook setup, then retry the complete operation".to_owned(),
            ],
            causal_disposition: AdapterCausalDisposition::Prerequisite,
        }
    }

    fn effect(
        target_ref: &str,
        content_fingerprint: Option<String>,
        disposition: AdapterPublicEffectDisposition,
    ) -> AdapterPublicEffect {
        AdapterPublicEffect {
            target_ref: target_ref.to_owned(),
            content_fingerprint,
            disposition,
        }
    }

    #[test]
    fn packet_three_semantic_details_reject_repository_invocation_failures() {
        let limits = ContractLimits::packet0();
        let charter = CharterOperationResult {
            schema_id: "handbook.charter-operation-result".to_owned(),
            schema_version: "1.0".to_owned(),
            operation: CharterOperation::Approve,
            status: AuthorOperationStatus::Refused,
            canonical_path: None,
            source_fingerprint: None,
            rendered_output_fingerprint: None,
            intake_ref: None,
            intake_fingerprint: None,
            candidate_ref: None,
            candidate_fingerprint: None,
            approval_ref: None,
            approval_fingerprint: None,
            promotion_ref: None,
            promotion_fingerprint: None,
            changed_paths: Vec::new(),
            refusal: Some(repository_failure("charter_approval").refusal.clone()),
            next_actions: Vec::new(),
            invocation_failure: Some(repository_failure("charter_approval")),
            causal_disposition: Some(AdapterCausalDisposition::Prerequisite),
            public_effects: Vec::new(),
        };
        assert!(UseCaseResponseV1::author_charter(&charter, &limits).is_err());

        let approver = ApproverOperationResult {
            schema_id: "handbook.repository-invocation-preflight-result".to_owned(),
            schema_version: "1.0".to_owned(),
            operation: ApproverOperation::Bootstrap,
            status: AuthorOperationStatus::Refused,
            changed_paths: Vec::new(),
            refusal: Some(repository_failure("bootstrap").refusal.clone()),
            next_actions: Vec::new(),
            legacy_projection: Some(ApproverOperationProjection::InvocationFailure(
                repository_failure("bootstrap"),
            )),
            causal_disposition: Some(AdapterCausalDisposition::Prerequisite),
            public_effects: Vec::new(),
        };
        assert!(UseCaseResponseV1::approver_administration(&approver, &limits).is_err());
    }

    #[test]
    fn packet_three_validate_terminals_reject_every_receipt() {
        let limits = ContractLimits::packet0();
        let canonical_effect = effect(
            ".handbook/project/charter.yaml",
            Some(fingerprint("charter")),
            AdapterPublicEffectDisposition::DurableCreated,
        );
        let success = CharterOperationResult {
            schema_id: "handbook.charter-operation-result".to_owned(),
            schema_version: "1.0".to_owned(),
            operation: CharterOperation::Validate,
            status: AuthorOperationStatus::Succeeded,
            canonical_path: Some(".handbook/project/charter.yaml".to_owned()),
            source_fingerprint: Some(fingerprint("charter")),
            rendered_output_fingerprint: None,
            intake_ref: None,
            intake_fingerprint: None,
            candidate_ref: None,
            candidate_fingerprint: None,
            approval_ref: None,
            approval_fingerprint: None,
            promotion_ref: None,
            promotion_fingerprint: None,
            changed_paths: Vec::new(),
            refusal: None,
            next_actions: Vec::new(),
            invocation_failure: None,
            causal_disposition: Some(AdapterCausalDisposition::CallerRefusal),
            public_effects: vec![canonical_effect.clone()],
        };
        assert!(UseCaseResponseV1::author_charter(&success, &limits).is_err());

        let refused = CharterOperationResult {
            status: AuthorOperationStatus::Refused,
            refusal: Some(AuthorOperationRefusal {
                code: "candidate_refused".to_owned(),
                message: "candidate refused".to_owned(),
                retryable: false,
            }),
            public_effects: vec![canonical_effect],
            ..success
        };
        assert!(UseCaseResponseV1::author_charter(&refused, &limits).is_err());

        let validate_success = ProjectContextCommandOutcome::Validate {
            input_mode: "stdin".to_owned(),
            input_source: "standard-input".to_owned(),
            result: Ok(()),
        };
        let result = AuthorProjectContextResultV1::validate("stdin", "standard-input").unwrap();
        let receipts = packet3_receipts(
            UseCaseKind::AuthorProjectContext,
            &[effect(
                ".handbook/project/context.yaml",
                Some(fingerprint("context")),
                AdapterPublicEffectDisposition::DurableCreated,
            )],
            &limits,
        )
        .unwrap();
        assert!(UseCaseResponseV1::author_project_context_success(
            result,
            receipts,
            empty_bounded().unwrap(),
            empty_bounded().unwrap(),
            &limits,
        )
        .is_err());
        assert!(UseCaseResponseV1::author_project_context(&validate_success, &limits).is_ok());

        let validate_refused = ProjectContextCommandOutcome::Validate {
            input_mode: "file".to_owned(),
            input_source: "context.yaml".to_owned(),
            result: Err(AuthorProjectContextRefusal {
                kind: AuthorProjectContextRefusalKind::MalformedStructuredInput,
                summary: "invalid input".to_owned(),
                broken_subject: "context.yaml".to_owned(),
                next_safe_action: "repair input".to_owned(),
                causal_disposition: AdapterCausalDisposition::CallerRefusal,
                public_effects: vec![effect(
                    ".handbook/project/context.yaml",
                    Some(fingerprint("context")),
                    AdapterPublicEffectDisposition::DurableCreated,
                )],
            }),
        };
        assert!(UseCaseResponseV1::author_project_context(&validate_refused, &limits).is_err());
    }

    #[test]
    fn project_context_author_receipts_are_exactly_correlated_to_the_outcome() {
        let limits = ContractLimits::packet0();
        let source = fingerprint("context-source");
        let authored = |public_effects| ProjectContextCommandOutcome::Author {
            input_mode: "file".to_owned(),
            input_source: "context.yaml".to_owned(),
            result: Ok(AuthorProjectContextResult {
                canonical_repo_relative_path: ".handbook/project/context.yaml".to_owned(),
                bytes_written: 42,
                source_fingerprint: source.clone(),
                rendered_output_fingerprint: fingerprint("context-rendered"),
                rendered_media_type: "text/markdown",
                public_effects,
            }),
        };
        assert!(UseCaseResponseV1::author_project_context(&authored(Vec::new()), &limits).is_err());
        assert!(UseCaseResponseV1::author_project_context(
            &authored(vec![effect(
                ".handbook/project/context.yaml",
                Some(fingerprint("wrong")),
                AdapterPublicEffectDisposition::DurableCreated,
            )]),
            &limits,
        )
        .is_err());
        assert!(UseCaseResponseV1::author_project_context(
            &authored(vec![effect(
                ".handbook/project/context.yaml",
                Some(source.clone()),
                AdapterPublicEffectDisposition::DurableCreated,
            )]),
            &limits,
        )
        .is_ok());
    }

    fn setup_projection_execution_with_fill(path_bytes: usize, fill: char) -> SetupExecutionResult {
        let reset_paths = (0..128)
            .map(|index| {
                let prefix = format!(".handbook/state/{index:03}/");
                assert!(path_bytes >= prefix.len());
                format!(
                    "{prefix}{}",
                    fill.to_string().repeat(path_bytes - prefix.len())
                )
            })
            .collect::<Vec<_>>();
        SetupExecutionResult::new(
            Ok(SetupOutcome {
                plan: SetupPlan {
                    requested_mode: SetupMode::Refresh,
                    resolved_mode: SetupMode::Refresh,
                    root_action: SetupRootAction::Create,
                    profile_ref: "example.profile@1.0.0".to_owned(),
                    profile_fingerprint: fingerprint("profile"),
                    stable_role_registry_ref: "example.roles@1.0.0".to_owned(),
                    stable_role_registry_fingerprint: fingerprint("roles"),
                    conditions: Vec::new(),
                    capabilities: Vec::new(),
                    artifacts: Vec::new(),
                    reset_paths: reset_paths.clone(),
                },
                status: RepositoryReadinessStatus::ActionRequired,
                reset_applied: true,
            }),
            SetupExecutionEffects::for_projection_test(reset_paths),
        )
    }

    fn setup_projection_execution(path_bytes: usize) -> SetupExecutionResult {
        setup_projection_execution_with_fill(path_bytes, 'x')
    }

    #[test]
    fn setup_outer_projection_measurement_closes_the_one_mib_boundary() {
        let public_limits = ContractLimits::packet0();
        let mut lower: usize = 24;
        let mut upper: usize = 4096;
        while lower < upper {
            let candidate = lower + (upper - lower).div_ceil(2);
            if measure_repository_setup_invocation(&setup_projection_execution(candidate)).unwrap()
                <= public_limits.document_bytes()
            {
                lower = candidate;
            } else {
                upper = candidate - 1;
            }
        }

        let in_bound_execution = setup_projection_execution(lower);
        let in_bound_bytes = measure_repository_setup_invocation(&in_bound_execution).unwrap();
        let out_of_bound_execution = setup_projection_execution(lower + 1);
        let out_of_bound_bytes =
            measure_repository_setup_invocation(&out_of_bound_execution).unwrap();
        assert!(in_bound_bytes <= public_limits.document_bytes());
        assert!(out_of_bound_bytes > public_limits.document_bytes());

        let use_case =
            UseCaseResponseV1::repository_setup_execution(&in_bound_execution, &public_limits)
                .unwrap();
        let invocation = InvocationResponseV1::use_case(use_case, &public_limits).unwrap();
        assert_eq!(
            canonical_json_bytes(&invocation, &public_limits)
                .unwrap()
                .len(),
            in_bound_bytes
        );
    }

    #[test]
    fn setup_measurement_ceiling_admits_maximum_quote_heavy_public_paths() {
        let observed =
            measure_repository_setup_invocation(&setup_projection_execution_with_fill(4096, '"'))
                .unwrap();

        assert!(observed > ContractLimits::packet0().document_bytes());
    }

    #[test]
    fn setup_error_codes_have_one_exact_terminal_disposition() {
        let limits = ContractLimits::packet0();
        for (code, status, category, retry) in [
            (
                crate::SetupErrorCode::ShippedProfileUnavailable,
                TerminalStatus::Blocked,
                ProblemCategory::Prerequisite,
                RetryDirective::AfterRecheck,
            ),
            (
                crate::SetupErrorCode::SelectedProfileDecisionInvalid,
                TerminalStatus::Refused,
                ProblemCategory::Precondition,
                RetryDirective::AfterRequestChange,
            ),
            (
                crate::SetupErrorCode::UnresolvedMode,
                TerminalStatus::Refused,
                ProblemCategory::Schema,
                RetryDirective::AfterRequestChange,
            ),
            (
                crate::SetupErrorCode::InitRejectsRefreshFlags,
                TerminalStatus::Refused,
                ProblemCategory::Schema,
                RetryDirective::AfterRequestChange,
            ),
            (
                crate::SetupErrorCode::RootAlreadyInitialized,
                TerminalStatus::Refused,
                ProblemCategory::Precondition,
                RetryDirective::AfterRequestChange,
            ),
            (
                crate::SetupErrorCode::RefreshRootMissing,
                TerminalStatus::Blocked,
                ProblemCategory::Prerequisite,
                RetryDirective::AfterRecheck,
            ),
            (
                crate::SetupErrorCode::RootNotDirectory,
                TerminalStatus::Refused,
                ProblemCategory::Safety,
                RetryDirective::Never,
            ),
            (
                crate::SetupErrorCode::RootSymlinkRefused,
                TerminalStatus::Refused,
                ProblemCategory::Safety,
                RetryDirective::Never,
            ),
            (
                crate::SetupErrorCode::CanonicalRootInspectFailed,
                TerminalStatus::Error,
                ProblemCategory::Implementation,
                RetryDirective::Never,
            ),
            (
                crate::SetupErrorCode::CanonicalRootCreateFailed,
                TerminalStatus::Error,
                ProblemCategory::Implementation,
                RetryDirective::Never,
            ),
            (
                crate::SetupErrorCode::RewriteHasNoMaterializer,
                TerminalStatus::Refused,
                ProblemCategory::Capability,
                RetryDirective::AfterRequestChange,
            ),
            (
                crate::SetupErrorCode::RuntimeStateTargetUnsafe,
                TerminalStatus::Refused,
                ProblemCategory::Safety,
                RetryDirective::Never,
            ),
            (
                crate::SetupErrorCode::RuntimeStateResetPlanTooLarge,
                TerminalStatus::Refused,
                ProblemCategory::Schema,
                RetryDirective::AfterRequestChange,
            ),
            (
                crate::SetupErrorCode::RuntimeStateResetProjectionTooLarge,
                TerminalStatus::Refused,
                ProblemCategory::Schema,
                RetryDirective::AfterRequestChange,
            ),
            (
                crate::SetupErrorCode::RuntimeStateMutationFailed,
                TerminalStatus::Error,
                ProblemCategory::Implementation,
                RetryDirective::Transient,
            ),
            (
                crate::SetupErrorCode::RepositoryAuthorityRecoveryBlocked,
                TerminalStatus::Blocked,
                ProblemCategory::Prerequisite,
                RetryDirective::AfterRecheck,
            ),
            (
                crate::SetupErrorCode::RepositoryIdentityUnsafe,
                TerminalStatus::Refused,
                ProblemCategory::Safety,
                RetryDirective::Never,
            ),
            (
                crate::SetupErrorCode::RepositoryIdentityMismatch,
                TerminalStatus::Refused,
                ProblemCategory::Authority,
                RetryDirective::AfterAuthorityChange,
            ),
            (
                crate::SetupErrorCode::RepositoryIdentityEntropyUnavailable,
                TerminalStatus::Error,
                ProblemCategory::Implementation,
                RetryDirective::Transient,
            ),
            (
                crate::SetupErrorCode::RepositoryIdentityPersistenceFailed,
                TerminalStatus::Error,
                ProblemCategory::Implementation,
                RetryDirective::Transient,
            ),
        ] {
            let error = match code {
                crate::SetupErrorCode::RuntimeStateResetPlanTooLarge => {
                    crate::setup::SetupError::runtime_state_reset_plan_too_large(129)
                }
                crate::SetupErrorCode::RuntimeStateResetProjectionTooLarge => {
                    crate::setup::SetupError::runtime_state_reset_projection_too_large(1_048_577)
                }
                _ => crate::setup::SetupError::from_code(code),
            };
            let expected_error = error.clone();
            let execution = crate::setup::SetupExecutionResult::new(
                Err(error),
                crate::setup::SetupExecutionEffects::default(),
            );
            let response = UseCaseResponseV1::repository_setup_execution(&execution, &limits)
                .unwrap_or_else(|error| panic!("{code:?}: {error:?}"));
            let [problem] = response.terminal_problems() else {
                panic!("{code:?}: expected one terminal problem");
            };
            assert_eq!(response.status(), status, "{code:?}");
            assert_eq!(problem.category(), category, "{code:?}");
            assert_eq!(problem.retry(), retry, "{code:?}");
            assert_eq!(problem.rule().is_some(), status == TerminalStatus::Refused);
            assert_eq!(
                problem.recheck().is_some(),
                status == TerminalStatus::Blocked
            );
            assert_eq!(
                problem.correlation_ref().is_some(),
                status == TerminalStatus::Error
            );
            let invocation = InvocationResponseV1::use_case(response, &limits).unwrap();
            let bytes = canonical_json_bytes(&invocation, &limits).unwrap();
            let parsed = InvocationResponseV1::parse_and_admit(&bytes, None, &limits).unwrap();
            assert_eq!(
                parsed.repository_setup_outcome(&limits).unwrap(),
                Some(Err(expected_error)),
                "{code:?}"
            );
        }
    }

    #[test]
    fn setup_error_after_reset_carries_only_the_exact_realized_reset_paths() {
        let limits = ContractLimits::packet0();
        let repo = tempdir().unwrap();
        fs::create_dir_all(repo.path().join(".handbook/state/child")).unwrap();
        fs::write(
            repo.path()
                .join(handbook_engine::REPOSITORY_IDENTITY_REPO_PATH),
            b"not-a-valid-fingerprint\n",
        )
        .unwrap();
        fs::write(
            repo.path().join(".handbook/state/child/realized.json"),
            b"{}",
        )
        .unwrap();
        let request = crate::SetupRequest {
            mode: crate::SetupMode::Refresh,
            rewrite: false,
            reset_state: true,
        };

        let execution = crate::setup::run_setup_with_effects(repo.path(), &request);
        let response = UseCaseResponseV1::repository_setup_execution(&execution, &limits).unwrap();

        assert_eq!(response.status(), TerminalStatus::Refused);
        let [receipt] = response.receipts() else {
            panic!("expected exactly one reset receipt");
        };
        assert_eq!(
            receipt.record().reference().as_str(),
            SETUP_RESET_RECORD_REF
        );
        let condition: RepositorySetupReceiptConditionV1 =
            admit_contract_value(receipt.condition(), &limits).unwrap();
        let RepositorySetupReceiptConditionV1::RuntimeStateReset {
            application,
            removed_paths,
        } = condition
        else {
            panic!("expected reset receipt condition");
        };
        assert_eq!(application, RepositorySetupResetApplicationV1::Sequential);
        assert!(removed_paths
            .as_slice()
            .iter()
            .any(|path| path.as_str() == ".handbook/state/child/realized.json"));
        assert!(!repo
            .path()
            .join(".handbook/state/child/realized.json")
            .exists());
    }

    #[test]
    fn setup_admission_rejects_fabricated_receipts_and_ready_actions() {
        let limits = ContractLimits::packet0();
        let ready = SetupOutcome {
            plan: SetupPlan {
                requested_mode: SetupMode::Refresh,
                resolved_mode: SetupMode::Refresh,
                root_action: SetupRootAction::Preserve,
                profile_ref: "example.profile@1.0.0".to_owned(),
                profile_fingerprint: fingerprint("profile"),
                stable_role_registry_ref: "example.roles@1.0.0".to_owned(),
                stable_role_registry_fingerprint: fingerprint("roles"),
                conditions: Vec::new(),
                capabilities: Vec::new(),
                artifacts: Vec::new(),
                reset_paths: Vec::new(),
            },
            status: RepositoryReadinessStatus::Ready,
            reset_applied: false,
        };
        let result = RepositorySetupResultV1::from_outcome(&ready).unwrap();
        let fabricated_root = repository_setup_receipt(
            "repository_setup_root",
            SETUP_ROOT_RECORD_REF,
            &RepositorySetupReceiptConditionV1::RootCreated {
                repo_relative_path: bounded(".handbook").unwrap(),
            },
            "repository_setup_root_create",
            &limits,
        )
        .unwrap();
        assert!(UseCaseResponseV1::repository_setup_success(
            result.clone(),
            BoundedVec::new(vec![fabricated_root]).unwrap(),
            BoundedVec::new(Vec::new()).unwrap(),
            BoundedVec::new(Vec::new()).unwrap(),
            &limits,
        )
        .is_err());

        let details = contract_value(
            &RepositorySetupDetailsV1::Readiness {
                result: Box::new(result.clone()),
            },
            &limits,
        )
        .unwrap();
        let fabricated_action = NextAction::new(
            bounded("repository.resolve_setup_actions").unwrap(),
            NextActionKind::ChangeRequest,
            binding(SETUP_USE_CASE_REF).unwrap(),
            binding(SETUP_DETAILS_REF).unwrap(),
            details,
            Some(bounded("Resolve repository readiness actions and rerun setup").unwrap()),
            &limits,
        )
        .unwrap();
        assert!(UseCaseResponseV1::repository_setup_success(
            result.clone(),
            BoundedVec::new(Vec::new()).unwrap(),
            BoundedVec::new(Vec::new()).unwrap(),
            BoundedVec::new(vec![fabricated_action]).unwrap(),
            &limits,
        )
        .is_err());

        let mut effectful = ready;
        effectful.plan.root_action = SetupRootAction::Create;
        effectful.plan.reset_paths = vec![".handbook/state/route.json".to_owned()];
        effectful.reset_applied = true;
        let effectful_result = RepositorySetupResultV1::from_outcome(&effectful).unwrap();
        let root = repository_setup_receipt(
            "repository_setup_root",
            SETUP_ROOT_RECORD_REF,
            &RepositorySetupReceiptConditionV1::RootCreated {
                repo_relative_path: bounded(".handbook").unwrap(),
            },
            "repository_setup_root_create",
            &limits,
        )
        .unwrap();
        let reset = repository_setup_receipt(
            "repository_setup_runtime_state_reset",
            SETUP_RESET_RECORD_REF,
            &RepositorySetupReceiptConditionV1::RuntimeStateReset {
                application: RepositorySetupResetApplicationV1::Sequential,
                removed_paths: BoundedVec::new(
                    vec![bounded(".handbook/state/route.json").unwrap()],
                )
                .unwrap(),
            },
            "repository_setup_sequential_reset",
            &limits,
        )
        .unwrap();
        for receipts in [
            vec![root.clone(), root.clone()],
            vec![reset.clone(), root.clone()],
        ] {
            assert!(UseCaseResponseV1::repository_setup_success(
                effectful_result.clone(),
                BoundedVec::new(receipts).unwrap(),
                BoundedVec::new(Vec::<Diagnostic>::new()).unwrap(),
                BoundedVec::new(Vec::<NextAction>::new()).unwrap(),
                &limits,
            )
            .is_err());
        }

        let mismatched_reset = repository_setup_receipt(
            "repository_setup_runtime_state_reset",
            SETUP_RESET_RECORD_REF,
            &RepositorySetupReceiptConditionV1::RuntimeStateReset {
                application: RepositorySetupResetApplicationV1::Sequential,
                removed_paths: BoundedVec::new(
                    vec![bounded(".handbook/state/other.json").unwrap()],
                )
                .unwrap(),
            },
            "repository_setup_sequential_reset",
            &limits,
        )
        .unwrap();
        let wrong_metadata = WriteReceipt::new(
            bounded("repository_setup_root_wrong").unwrap(),
            binding(SETUP_ROOT_RECORD_REF).unwrap(),
            bounded("sdk_composition").unwrap(),
            contract_value(
                &RepositorySetupReceiptConditionV1::RootCreated {
                    repo_relative_path: bounded(".handbook").unwrap(),
                },
                &limits,
            )
            .unwrap(),
            bounded("repository_setup_root_create").unwrap(),
            &limits,
        )
        .unwrap();
        for receipt in [mismatched_reset, wrong_metadata] {
            assert!(UseCaseResponseV1::repository_setup_success(
                effectful_result.clone(),
                BoundedVec::new(vec![receipt]).unwrap(),
                BoundedVec::new(Vec::<Diagnostic>::new()).unwrap(),
                BoundedVec::new(Vec::<NextAction>::new()).unwrap(),
                &limits,
            )
            .is_err());
        }

        for receipts in [Vec::new(), vec![root]] {
            assert!(UseCaseResponseV1::repository_setup_success(
                effectful_result.clone(),
                BoundedVec::new(receipts).unwrap(),
                BoundedVec::new(Vec::<Diagnostic>::new()).unwrap(),
                BoundedVec::new(Vec::<NextAction>::new()).unwrap(),
                &limits,
            )
            .is_err());
        }
    }

    #[test]
    fn current_public_results_project_without_dropping_packet_one_facts() {
        let setup = SetupOutcome {
            plan: crate::SetupPlan {
                requested_mode: SetupMode::Refresh,
                resolved_mode: SetupMode::Refresh,
                root_action: SetupRootAction::Preserve,
                profile_ref: "example.profile@1.0.0".to_owned(),
                profile_fingerprint: fingerprint("profile"),
                stable_role_registry_ref: "example.roles@1.0.0".to_owned(),
                stable_role_registry_fingerprint: fingerprint("roles"),
                conditions: Vec::new(),
                capabilities: Vec::new(),
                artifacts: Vec::new(),
                reset_paths: vec![".handbook/state/route.json".to_owned()],
            },
            status: RepositoryReadinessStatus::ActionRequired,
            reset_applied: true,
        };
        let projected_setup = RepositorySetupResultV1::from_outcome(&setup).unwrap();
        assert_eq!(projected_setup.to_outcome(), setup);
        let setup_wire = serde_json::to_value(projected_setup).unwrap();
        assert_eq!(setup_wire["requested_mode"], "refresh");
        assert_eq!(setup_wire["reset_paths"][0], ".handbook/state/route.json");
        assert_eq!(setup_wire["reset_applied"], true);

        let charter = CharterOperationResult {
            schema_id: "handbook.charter-operation-result".to_owned(),
            schema_version: "1.0".to_owned(),
            operation: CharterOperation::Author,
            status: AuthorOperationStatus::Succeeded,
            canonical_path: Some(".handbook/project/charter.yaml".to_owned()),
            source_fingerprint: Some(fingerprint("source")),
            rendered_output_fingerprint: Some(fingerprint("rendered")),
            intake_ref: Some("intake.json".to_owned()),
            intake_fingerprint: Some(fingerprint("intake")),
            candidate_ref: Some("candidate.json".to_owned()),
            candidate_fingerprint: Some(fingerprint("candidate")),
            approval_ref: None,
            approval_fingerprint: None,
            promotion_ref: None,
            promotion_fingerprint: None,
            changed_paths: vec![".handbook/project/charter.yaml".to_owned()],
            refusal: None,
            next_actions: vec!["review charter".to_owned()],
            invocation_failure: None,
            causal_disposition: Some(
                crate::charter_product::AdapterCausalDisposition::CallerRefusal,
            ),
            public_effects: Vec::new(),
        };
        let charter =
            serde_json::to_value(AuthorCharterResultV1::from_result(&charter).unwrap()).unwrap();
        assert_eq!(charter["intake"]["reference"], "intake.json");
        assert_eq!(charter["candidate"]["reference"], "candidate.json");
        assert_eq!(charter["next_actions"][0], "review charter");

        let authored = AuthorProjectContextResult {
            canonical_repo_relative_path: ".handbook/project/context.yaml".to_owned(),
            bytes_written: 27,
            source_fingerprint: fingerprint("context-source"),
            rendered_output_fingerprint: fingerprint("context-rendered"),
            rendered_media_type: "text/markdown",
            public_effects: Vec::new(),
        };
        let authored = serde_json::to_value(
            AuthorProjectContextResultV1::author("file", "context.yaml", &authored).unwrap(),
        )
        .unwrap();
        assert_eq!(authored["result_kind"], "author");
        assert_eq!(authored["bytes_written"], 27);
        let validated_result =
            AuthorProjectContextResultV1::validate("stdin", "standard-input").unwrap();
        let validated = serde_json::to_value(&validated_result).unwrap();
        assert_eq!(validated["result_kind"], "validate");
        assert!(validated.get("canonical_path").is_none());
        let limits = ContractLimits::packet0();
        let response = UseCaseResponseV1::author_project_context_success(
            validated_result,
            BoundedVec::new(Vec::new()).unwrap(),
            BoundedVec::new(Vec::new()).unwrap(),
            BoundedVec::new(Vec::new()).unwrap(),
            &limits,
        )
        .unwrap();
        let invocation = InvocationResponseV1::use_case(response, &limits).unwrap();
        assert_eq!(invocation.terminal_status(), TerminalStatus::Ok);

        let approver = ApproverOperationResult {
            schema_id: "handbook.approver-admin-adapter-result".to_owned(),
            schema_version: "1.0".to_owned(),
            operation: ApproverOperation::Bootstrap,
            status: AuthorOperationStatus::Succeeded,
            changed_paths: vec![".handbook/approvers/registry.json".to_owned()],
            refusal: None,
            next_actions: vec!["add credential".to_owned()],
            legacy_projection: Some(ApproverLegacyProjection::Admin(ApproverAdminLegacyFields {
                operation_id: "op_bootstrap".to_owned(),
                repository_identity_fingerprint: fingerprint("repository"),
                prior_registry_state_ref: None,
                prior_registry_state_fingerprint: None,
                result_registry_state_ref: Some("registry.json".to_owned()),
                result_registry_state_fingerprint: Some(fingerprint("registry")),
                transition_ref: Some("transition.json".to_owned()),
                transition_fingerprint: Some(fingerprint("transition")),
                registration_ref: None,
                registration_fingerprint: None,
                assertion_ref: None,
                assertion_fingerprint: None,
            })),
            causal_disposition: Some(
                crate::charter_product::AdapterCausalDisposition::CallerRefusal,
            ),
            public_effects: Vec::new(),
        };
        let approver =
            serde_json::to_value(ApproverAdministrationResultV1::from_result(&approver).unwrap())
                .unwrap();
        assert_eq!(
            approver["result_registry_state"]["reference"],
            "registry.json"
        );
        assert_eq!(approver["transition"]["reference"], "transition.json");

        let pipeline = PipelineResolveOutcome {
            pipeline_id: "planning".to_owned(),
            state: PipelineRouteStateView {
                revision: 4,
                routing: BTreeMap::from([("charter_ready".to_owned(), true)]),
                charter_ref: Some("charter@current".to_owned()),
                project_context_ref: Some("context@current".to_owned()),
                run: PipelineRouteRunView {
                    runner: Some("local".to_owned()),
                    profile: Some("default".to_owned()),
                    repository_root: Some(".".to_owned()),
                },
            },
            effective_run: PipelineRouteRunView {
                runner: Some("local".to_owned()),
                profile: Some("default".to_owned()),
                repository_root: Some(".".to_owned()),
            },
            stages: vec![PipelineResolvedStageView {
                stage_id: "stage-01".to_owned(),
                status: PipelineRouteStageStatus::Next,
                reason: Some(PipelineRouteStageReason::NextMissingRouteVariables {
                    missing_variables: vec!["approval_ready".to_owned()],
                }),
            }],
        };
        let pipeline = serde_json::to_value(
            PipelineRouteResolveAndPersistResultV1::from_outcome(&pipeline).unwrap(),
        )
        .unwrap();
        assert_eq!(pipeline["state"]["revision"], 4);
        assert_eq!(pipeline["effective_run"]["runner"], "local");
        assert_eq!(pipeline["stages"][0]["status"], "next");
        assert_eq!(
            pipeline["stages"][0]["reason"]["missing_variables"][0],
            "approval_ready"
        );
    }

    #[test]
    fn pipeline_route_non_ok_preserves_receipts_for_prior_writes() {
        use handbook_pipeline::route_state::{
            RouteBasis, RouteBasisProfilePack, RouteBasisRunner, RouteStateRefs, RouteStateRun,
        };

        let limits = ContractLimits::packet0();
        let execution = PipelineResolveExecution {
            result: Err(PipelineResolveExecutionFailure::RouteResolution(
                "route resolution stopped after persistence".to_owned(),
            )),
            effects: vec![PipelineResolveEffect {
                pipeline_id: "planning".to_owned(),
                state_revision: 4,
                route_basis: RouteBasis {
                    schema_version: "route_basis.v1".to_owned(),
                    pipeline_id: "planning".to_owned(),
                    pipeline_file: "core/pipelines/planning.yaml".to_owned(),
                    pipeline_file_sha256: fingerprint("pipeline"),
                    state_revision: 4,
                    routing: BTreeMap::new(),
                    refs: RouteStateRefs::default(),
                    run: RouteStateRun::default(),
                    route: Vec::new(),
                    runner: RouteBasisRunner {
                        id: "local".to_owned(),
                        file: "core/runners/local.md".to_owned(),
                        file_sha256: fingerprint("runner"),
                    },
                    profile: RouteBasisProfilePack {
                        id: "default".to_owned(),
                        profile_yaml_sha256: fingerprint("profile"),
                        commands_yaml_sha256: fingerprint("commands"),
                        conventions_md_sha256: fingerprint("conventions"),
                    },
                },
            }],
        };

        let response =
            UseCaseResponseV1::pipeline_route_resolve_and_persist_execution(&execution, &limits)
                .expect("adapt non-ok execution");
        assert_eq!(response.status(), TerminalStatus::Blocked);
        assert_eq!(response.receipts().len(), 1);

        let bytes = canonical_json_bytes(&response, &limits).expect("encode response");
        let admitted: UseCaseResponseV1 =
            parse_canonical_json(&bytes, &limits).expect("admit response with prior-write receipt");
        assert_eq!(admitted.receipts().len(), 1);
    }
}
