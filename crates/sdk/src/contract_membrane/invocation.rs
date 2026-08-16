use super::serialization::classify_contract_error_message;
use super::{
    canonical_fingerprint, canonical_json_bytes, parse_canonical_json, sha256_fingerprint,
    BoundedString, BoundedVec, ContractError, ContractLimits, ContractValue, CorrelationRef,
    Diagnostic, ExactBinding, ExactRef, Fingerprint, NextAction, NextActionKind,
    OperationAdmissionRefusalV1, OperationDefinition, OperationResponseV1,
    OperationSchemaValidator, Problem, ProblemCategory, RecheckCondition, RetryDirective, SafeU64,
    TerminalFields, TerminalStatus, WriteReceipt,
};
use crate::pipeline_route_api::{
    PipelineResolveOutcome, PipelineResolvedStageView, PipelineRouteRunView,
    PipelineRouteStageReason, PipelineRouteStageStatus, PipelineRouteStateView,
};
use crate::{
    ApproverOperation, ApproverOperationProjection, ApproverOperationResult, ArtifactApplicability,
    ArtifactInspectionReason, ArtifactInspectionStatus, AuthorOperationStatus,
    AuthorProjectContextResult, CharterOperation, CharterOperationResult,
    DoctorCharterDefinitionClosureStatus, DoctorCharterDefinitionRow, DoctorCharterLifecycleState,
    DoctorCharterNextAction, DoctorCharterRow, DoctorError, DoctorErrorKind, DoctorErrorReasonCode,
    DoctorProjectContextRow, DoctorReport, ProfileArtifactRow, ProfileCapabilityRow,
    ProfileConditionOutcome, ProfileConditionReason, ProfileConditionRow, ProfileRequirednessMode,
    RepositoryReadinessStatus, SetupArtifactActionKind, SetupMode, SetupOutcome, SetupRootAction,
};
use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize};

const INVOCATION_RESPONSE_REF: &str = "handbook.invocation-response@1.0.0";
const USE_CASE_RESPONSE_REF: &str = "handbook.use-case-response@1.0.0";
const TRANSPORT_FAILURE_REF: &str = "handbook.transport-failure@1.0.0";
const DOCTOR_USE_CASE_REF: &str = "handbook.use-case.repository-doctor@1.0.0";
const DOCTOR_RESULT_REF: &str = "handbook.use-case.repository-doctor-result@1.0.0";
const DOCTOR_DETAILS_REF: &str = "handbook.use-case.repository-doctor-details@1.0.0";
const TRANSPORT_DETAILS_REF: &str = "handbook.transport-failure-details@1.0.0";

fn binding(reference: &str) -> Result<ExactBinding, ContractError> {
    if matches!(
        reference,
        INVOCATION_RESPONSE_REF
            | USE_CASE_RESPONSE_REF
            | TRANSPORT_FAILURE_REF
            | DOCTOR_RESULT_REF
            | DOCTOR_DETAILS_REF
            | TRANSPORT_DETAILS_REF
            | "handbook.use-case.repository-setup-result@1.0.0"
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
}

map_enum!(SetupMode => RepositorySetupModeV1 { Auto, Init, Refresh });
map_enum!(SetupRootAction => RepositorySetupRootActionV1 { Preserve, Create });
map_enum!(SetupArtifactActionKind => RepositorySetupArtifactActionKindV1 {
    Preserve, AuthorRequired, OptionalAbsent, ConditionIndeterminate, Invalid
});

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
}

map_enum!(CharterOperation => AuthorCharterOperationV1 {
    Author, Approve, Promote, Validate
});

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
}

map_enum!(ApproverOperation => ApproverAdministrationOperationV1 {
    Bootstrap, AddCredential, RevokeCredential, UpdateMapping
});

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

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TransportFailureDetailsV1 {
    failure_kind: TransportFailureKind,
    reason_code: BoundedString<255>,
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
            _ => Err(ContractError::InvalidInvariant),
        }
    }
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

impl UseCaseResponseV1 {
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
        let details_schema = binding(TRANSPORT_DETAILS_REF)?;
        let details = contract_value(
            &TransportFailureDetailsV1 {
                failure_kind,
                reason_code: bounded(reason_code)?,
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
            if details.failure_kind != failure_kind {
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

    fn fingerprint(label: &str) -> String {
        sha256_fingerprint(label.as_bytes()).as_str().to_owned()
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
        let setup =
            serde_json::to_value(RepositorySetupResultV1::from_outcome(&setup).unwrap()).unwrap();
        assert_eq!(setup["requested_mode"], "refresh");
        assert_eq!(setup["reset_paths"][0], ".handbook/state/route.json");
        assert_eq!(setup["reset_applied"], true);

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
}
