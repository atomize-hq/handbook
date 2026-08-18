use super::{
    identity_binding, parse_canonical_wire, parse_contract_value, require_schema_binding,
    schema_binding, RequiredOption, INVOCATION_REQUEST_REF, USE_CASE_REQUEST_REF,
};
use crate::contract_membrane::serialization::classify_contract_error_message;
use crate::contract_membrane::{
    canonical_fingerprint, canonical_json_bytes, BoundedString, BoundedVec, ContractError,
    ContractLimits, ExactBinding, Fingerprint, OperationRequestV1, RepositorySetupModeV1,
    UtcTimestamp,
};
use serde::de::Error as _;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;
use std::collections::BTreeSet;

const SETUP_USE_CASE_REF: &str = "handbook.use-case.repository-setup@1.0.0";
const DOCTOR_USE_CASE_REF: &str = "handbook.use-case.repository-doctor@1.0.0";
const AUTHOR_CHARTER_USE_CASE_REF: &str = "handbook.use-case.author-charter@1.0.0";
const AUTHOR_PROJECT_CONTEXT_USE_CASE_REF: &str = "handbook.use-case.author-project-context@1.0.0";
const APPROVER_ADMINISTRATION_USE_CASE_REF: &str =
    "handbook.use-case.approver-administration@1.0.0";
const PIPELINE_ROUTE_USE_CASE_REF: &str =
    "handbook.use-case.pipeline-route-resolve-and-persist@1.0.0";

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RepositorySetupRequestV1 {
    mode: RepositorySetupModeV1,
    rewrite: bool,
    reset_state: bool,
}

impl RepositorySetupRequestV1 {
    pub fn new(mode: RepositorySetupModeV1, rewrite: bool, reset_state: bool) -> Self {
        Self {
            mode,
            rewrite,
            reset_state,
        }
    }

    pub fn mode(&self) -> RepositorySetupModeV1 {
        self.mode
    }

    pub fn rewrite(&self) -> bool {
        self.rewrite
    }

    pub fn reset_state(&self) -> bool {
        self.reset_state
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RepositoryDoctorRequestV1 {}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CharterAcquisitionModeV1 {
    GuidedAdaptive,
    Express,
    AgentAssisted,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CharterIntakeSourceKindV1 {
    UserDeclaration,
    EvidencedInference,
    DeterministicDefault,
    KnownUnknown,
    Contradiction,
    Waiver,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CharterInputEntryV1 {
    key: String,
    value: CharterInputValueV1,
}

impl CharterInputEntryV1 {
    pub fn new(key: &str, value: CharterInputValueV1) -> Result<Self, ContractError> {
        check_charter_string(key)?;
        Ok(Self {
            key: key.to_owned(),
            value,
        })
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn value(&self) -> &CharterInputValueV1 {
        &self.value
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "value_kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum CharterInputValueV1 {
    Null,
    Boolean {
        value: bool,
    },
    Number {
        value: String,
    },
    String {
        value: String,
    },
    Sequence {
        values: BoundedVec<CharterInputValueV1, 128>,
    },
    Mapping {
        entries: BoundedVec<CharterInputEntryV1, 128>,
    },
}

impl CharterInputValueV1 {
    pub fn null() -> Self {
        Self::Null
    }

    pub fn boolean(value: bool) -> Self {
        Self::Boolean { value }
    }

    pub fn number(value: &str) -> Result<Self, ContractError> {
        let value = Self::Number {
            value: value.to_owned(),
        };
        value.validate()?;
        Ok(value)
    }

    pub fn string(value: &str) -> Result<Self, ContractError> {
        check_charter_string(value)?;
        Ok(Self::String {
            value: value.to_owned(),
        })
    }

    pub fn sequence(values: Vec<Self>) -> Result<Self, ContractError> {
        let value = Self::Sequence {
            values: BoundedVec::new(values)?,
        };
        value.validate()?;
        Ok(value)
    }

    pub fn mapping(entries: Vec<CharterInputEntryV1>) -> Result<Self, ContractError> {
        let value = Self::Mapping {
            entries: BoundedVec::new(entries)?,
        };
        value.validate()?;
        Ok(value)
    }

    fn validate(&self) -> Result<(), ContractError> {
        match self {
            Self::Number { value } => {
                check_charter_string(value)?;
                value
                    .parse::<serde_json::Number>()
                    .map_err(|_| ContractError::InvalidNumber)?;
            }
            Self::String { value } => check_charter_string(value)?,
            Self::Sequence { values } => {
                for value in values.as_slice() {
                    value.validate()?;
                }
            }
            Self::Mapping { entries } => {
                let mut keys = BTreeSet::new();
                for entry in entries.as_slice() {
                    check_charter_string(&entry.key)?;
                    if !keys.insert(entry.key.as_str()) {
                        return Err(ContractError::DuplicateMember);
                    }
                    entry.value.validate()?;
                }
            }
            Self::Null | Self::Boolean { .. } => {}
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CharterCoverageSubmissionV1 {
    coverage_id: BoundedString<255>,
    source_kind: CharterIntakeSourceKindV1,
    value_ref: BoundedString<4096>,
    evidence_refs: BoundedVec<BoundedString<4096>, 128>,
    confidence: BoundedString<255>,
    freshness: Option<BoundedString<255>>,
    sensitivity: BoundedString<255>,
    contradiction_refs: BoundedVec<BoundedString<4096>, 128>,
    waiver_ref: Option<BoundedString<4096>>,
}

impl CharterCoverageSubmissionV1 {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        coverage_id: &str,
        source_kind: CharterIntakeSourceKindV1,
        value_ref: &str,
        evidence_refs: Vec<String>,
        confidence: &str,
        freshness: Option<&str>,
        sensitivity: &str,
        contradiction_refs: Vec<String>,
        waiver_ref: Option<&str>,
    ) -> Result<Self, ContractError> {
        Ok(Self {
            coverage_id: bounded(coverage_id)?,
            source_kind,
            value_ref: bounded(value_ref)?,
            evidence_refs: bounded_vec(evidence_refs)?,
            confidence: bounded(confidence)?,
            freshness: optional_bounded(freshness)?,
            sensitivity: bounded(sensitivity)?,
            contradiction_refs: bounded_vec(contradiction_refs)?,
            waiver_ref: optional_bounded(waiver_ref)?,
        })
    }
}

impl<'de> Deserialize<'de> for CharterCoverageSubmissionV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Raw {
            coverage_id: BoundedString<255>,
            source_kind: CharterIntakeSourceKindV1,
            value_ref: BoundedString<4096>,
            evidence_refs: BoundedVec<BoundedString<4096>, 128>,
            confidence: BoundedString<255>,
            #[serde(default)]
            freshness: RequiredOption<BoundedString<255>>,
            sensitivity: BoundedString<255>,
            contradiction_refs: BoundedVec<BoundedString<4096>, 128>,
            #[serde(default)]
            waiver_ref: RequiredOption<BoundedString<4096>>,
        }
        let raw = Raw::deserialize(deserializer).map_err(sanitize_deserialize_error)?;
        Ok(Self {
            coverage_id: raw.coverage_id,
            source_kind: raw.source_kind,
            value_ref: raw.value_ref,
            evidence_refs: raw.evidence_refs,
            confidence: raw.confidence,
            freshness: raw.freshness.into_option().map_err(D::Error::custom)?,
            sensitivity: raw.sensitivity,
            contradiction_refs: raw.contradiction_refs,
            waiver_ref: raw.waiver_ref.into_option().map_err(D::Error::custom)?,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CharterIntakeConsumerV1 {
    kind: BoundedString<255>,
    id: BoundedString<255>,
    version: BoundedString<255>,
}

impl CharterIntakeConsumerV1 {
    pub fn new(kind: &str, id: &str, version: &str) -> Result<Self, ContractError> {
        Ok(Self {
            kind: bounded(kind)?,
            id: bounded(id)?,
            version: bounded(version)?,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CharterIntakeEnvelopeV1 {
    mode: CharterAcquisitionModeV1,
    content: CharterInputValueV1,
    coverage: BoundedVec<CharterCoverageSubmissionV1, 128>,
    consumer: CharterIntakeConsumerV1,
    prompt_event_refs: BoundedVec<BoundedString<4096>, 128>,
    finalized_at_utc: UtcTimestamp,
    expected_current_fingerprint: Option<Fingerprint>,
}

impl CharterIntakeEnvelopeV1 {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        mode: CharterAcquisitionModeV1,
        content: CharterInputValueV1,
        coverage: Vec<CharterCoverageSubmissionV1>,
        consumer: CharterIntakeConsumerV1,
        prompt_event_refs: Vec<String>,
        finalized_at_utc: UtcTimestamp,
        expected_current_fingerprint: Option<Fingerprint>,
    ) -> Result<Self, ContractError> {
        let value = Self {
            mode,
            content,
            coverage: BoundedVec::new(coverage)?,
            consumer,
            prompt_event_refs: bounded_vec(prompt_event_refs)?,
            finalized_at_utc,
            expected_current_fingerprint,
        };
        value.validate()?;
        Ok(value)
    }

    fn validate(&self) -> Result<(), ContractError> {
        self.content.validate()
    }
}

impl<'de> Deserialize<'de> for CharterIntakeEnvelopeV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Raw {
            mode: CharterAcquisitionModeV1,
            content: CharterInputValueV1,
            coverage: BoundedVec<CharterCoverageSubmissionV1, 128>,
            consumer: CharterIntakeConsumerV1,
            prompt_event_refs: BoundedVec<BoundedString<4096>, 128>,
            finalized_at_utc: UtcTimestamp,
            #[serde(default)]
            expected_current_fingerprint: RequiredOption<Fingerprint>,
        }
        let raw = Raw::deserialize(deserializer).map_err(sanitize_deserialize_error)?;
        let value = Self {
            mode: raw.mode,
            content: raw.content,
            coverage: raw.coverage,
            consumer: raw.consumer,
            prompt_event_refs: raw.prompt_event_refs,
            finalized_at_utc: raw.finalized_at_utc,
            expected_current_fingerprint: raw
                .expected_current_fingerprint
                .into_option()
                .map_err(D::Error::custom)?,
        };
        value.validate().map_err(D::Error::custom)?;
        Ok(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "operation", rename_all = "snake_case")]
pub enum AuthorCharterRequestV1 {
    Author {
        mode: CharterAcquisitionModeV1,
        intake: CharterIntakeEnvelopeV1,
    },
    Approve {
        candidate_ref: BoundedString<4096>,
        approval_class: BoundedString<255>,
        authority_ref: BoundedString<4096>,
        accepted_waiver_refs: BoundedVec<BoundedString<4096>, 128>,
    },
    Promote {
        candidate_ref: BoundedString<4096>,
        approval_ref: BoundedString<4096>,
        expected_current_fingerprint: Option<Fingerprint>,
    },
    Validate,
}

impl AuthorCharterRequestV1 {
    fn validate(&self) -> Result<(), ContractError> {
        if let Self::Author { intake, .. } = self {
            intake.validate()?;
        }
        Ok(())
    }
}

impl<'de> Deserialize<'de> for AuthorCharterRequestV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(tag = "operation", rename_all = "snake_case", deny_unknown_fields)]
        enum Raw {
            Author {
                mode: CharterAcquisitionModeV1,
                intake: CharterIntakeEnvelopeV1,
            },
            Approve {
                candidate_ref: BoundedString<4096>,
                approval_class: BoundedString<255>,
                authority_ref: BoundedString<4096>,
                accepted_waiver_refs: BoundedVec<BoundedString<4096>, 128>,
            },
            Promote {
                candidate_ref: BoundedString<4096>,
                approval_ref: BoundedString<4096>,
                #[serde(default)]
                expected_current_fingerprint: RequiredOption<Fingerprint>,
            },
            Validate,
        }
        let value = match Raw::deserialize(deserializer).map_err(sanitize_deserialize_error)? {
            Raw::Author { mode, intake } => Self::Author { mode, intake },
            Raw::Approve {
                candidate_ref,
                approval_class,
                authority_ref,
                accepted_waiver_refs,
            } => Self::Approve {
                candidate_ref,
                approval_class,
                authority_ref,
                accepted_waiver_refs,
            },
            Raw::Promote {
                candidate_ref,
                approval_ref,
                expected_current_fingerprint,
            } => Self::Promote {
                candidate_ref,
                approval_ref,
                expected_current_fingerprint: expected_current_fingerprint
                    .into_option()
                    .map_err(D::Error::custom)?,
            },
            Raw::Validate => Self::Validate,
        };
        value.validate().map_err(D::Error::custom)?;
        Ok(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectContextInputV1 {
    schema_id: BoundedString<255>,
    schema_version: BoundedString<255>,
    record_id: BoundedString<255>,
    summary: BoundedString<4096>,
    system_boundaries: BoundedVec<BoundedString<4096>, 128>,
    ownership: BoundedVec<BoundedString<4096>, 128>,
    authoritative_references: BoundedVec<BoundedString<4096>, 128>,
    known_unknowns: BoundedVec<BoundedString<4096>, 128>,
}

impl ProjectContextInputV1 {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        schema_id: &str,
        schema_version: &str,
        record_id: &str,
        summary: &str,
        system_boundaries: Vec<String>,
        ownership: Vec<String>,
        authoritative_references: Vec<String>,
        known_unknowns: Vec<String>,
    ) -> Result<Self, ContractError> {
        Ok(Self {
            schema_id: bounded(schema_id)?,
            schema_version: bounded(schema_version)?,
            record_id: bounded(record_id)?,
            summary: bounded(summary)?,
            system_boundaries: bounded_vec(system_boundaries)?,
            ownership: bounded_vec(ownership)?,
            authoritative_references: bounded_vec(authoritative_references)?,
            known_unknowns: bounded_vec(known_unknowns)?,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(
    tag = "operation",
    content = "input",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum AuthorProjectContextRequestV1 {
    Author(ProjectContextInputV1),
    Validate(ProjectContextInputV1),
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ApproverMappingRequestV1 {
    approval_class: BoundedString<255>,
    authority_ref: BoundedString<4096>,
}

impl ApproverMappingRequestV1 {
    pub fn new(approval_class: &str, authority_ref: &str) -> Result<Self, ContractError> {
        Ok(Self {
            approval_class: bounded(approval_class)?,
            authority_ref: bounded(authority_ref)?,
        })
    }

    pub fn approval_class(&self) -> &str {
        self.approval_class.as_str()
    }

    pub fn authority_ref(&self) -> &str {
        self.authority_ref.as_str()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "operation", rename_all = "snake_case", deny_unknown_fields)]
pub enum ApproverAdministrationRequestV1 {
    Bootstrap {
        initial_charter_quorum: BoundedVec<ApproverMappingRequestV1, 128>,
    },
    AddCredential {
        approval_mappings: BoundedVec<ApproverMappingRequestV1, 128>,
    },
    RevokeCredential {
        credential_id_hash: BoundedString<4096>,
    },
    UpdateMapping {
        credential_id_hash: BoundedString<4096>,
        approval_mappings: BoundedVec<ApproverMappingRequestV1, 128>,
    },
}

impl ApproverAdministrationRequestV1 {
    pub fn revoke_credential(credential_id_hash: &str) -> Result<Self, ContractError> {
        Ok(Self::RevokeCredential {
            credential_id_hash: bounded(credential_id_hash)?,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PipelineRouteResolveAndPersistRequestV1 {
    selector: BoundedString<4096>,
}

impl PipelineRouteResolveAndPersistRequestV1 {
    pub fn new(selector: &str) -> Result<Self, ContractError> {
        Ok(Self {
            selector: bounded(selector)?,
        })
    }

    pub fn selector(&self) -> &str {
        self.selector.as_str()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UseCaseRequestBodyV1 {
    RepositorySetup(RepositorySetupRequestV1),
    RepositoryDoctor(RepositoryDoctorRequestV1),
    AuthorCharter(AuthorCharterRequestV1),
    AuthorProjectContext(AuthorProjectContextRequestV1),
    ApproverAdministration(ApproverAdministrationRequestV1),
    PipelineRouteResolveAndPersist(PipelineRouteResolveAndPersistRequestV1),
}

pub(crate) enum SdkUseCaseRequest {
    RepositorySetup(crate::SetupRequest),
    RepositoryDoctor,
    AuthorCharter(crate::CharterCommandRequest),
    AuthorProjectContext(crate::ProjectContextCommandRequest),
    ApproverAdministration(crate::ApproverCommandRequest),
    PipelineRouteResolveAndPersist(String),
}

impl UseCaseRequestBodyV1 {
    fn use_case_ref(&self) -> &'static str {
        match self {
            Self::RepositorySetup(_) => SETUP_USE_CASE_REF,
            Self::RepositoryDoctor(_) => DOCTOR_USE_CASE_REF,
            Self::AuthorCharter(_) => AUTHOR_CHARTER_USE_CASE_REF,
            Self::AuthorProjectContext(_) => AUTHOR_PROJECT_CONTEXT_USE_CASE_REF,
            Self::ApproverAdministration(_) => APPROVER_ADMINISTRATION_USE_CASE_REF,
            Self::PipelineRouteResolveAndPersist(_) => PIPELINE_ROUTE_USE_CASE_REF,
        }
    }

    fn validate(&self) -> Result<(), ContractError> {
        if let Self::AuthorCharter(request) = self {
            request.validate()?;
        }
        Ok(())
    }

    pub(crate) fn sdk_request(&self) -> SdkUseCaseRequest {
        match self {
            Self::RepositorySetup(request) => {
                SdkUseCaseRequest::RepositorySetup(crate::SetupRequest {
                    mode: match request.mode {
                        RepositorySetupModeV1::Auto => crate::SetupMode::Auto,
                        RepositorySetupModeV1::Init => crate::SetupMode::Init,
                        RepositorySetupModeV1::Refresh => crate::SetupMode::Refresh,
                    },
                    rewrite: request.rewrite,
                    reset_state: request.reset_state,
                })
            }
            Self::RepositoryDoctor(_) => SdkUseCaseRequest::RepositoryDoctor,
            Self::AuthorCharter(request) => {
                SdkUseCaseRequest::AuthorCharter(sdk_charter_request(request))
            }
            Self::AuthorProjectContext(request) => {
                SdkUseCaseRequest::AuthorProjectContext(sdk_project_context_request(request))
            }
            Self::ApproverAdministration(request) => {
                SdkUseCaseRequest::ApproverAdministration(sdk_approver_request(request))
            }
            Self::PipelineRouteResolveAndPersist(request) => {
                SdkUseCaseRequest::PipelineRouteResolveAndPersist(
                    request.selector.as_str().to_owned(),
                )
            }
        }
    }
}

fn sdk_charter_request(request: &AuthorCharterRequestV1) -> crate::CharterCommandRequest {
    match request {
        AuthorCharterRequestV1::Author { mode, intake } => crate::CharterCommandRequest::Author {
            mode: sdk_charter_mode(*mode),
            intake: crate::CharterIntakeEnvelope {
                mode: sdk_charter_mode(intake.mode),
                content: sdk_charter_value(&intake.content),
                coverage: intake
                    .coverage
                    .as_slice()
                    .iter()
                    .map(|submission| crate::CharterCoverageSubmission {
                        coverage_id: submission.coverage_id.as_str().to_owned(),
                        source_kind: sdk_charter_source_kind(submission.source_kind),
                        value_ref: submission.value_ref.as_str().to_owned(),
                        evidence_refs: bounded_strings(&submission.evidence_refs),
                        confidence: submission.confidence.as_str().to_owned(),
                        freshness: submission
                            .freshness
                            .as_ref()
                            .map(|value| value.as_str().to_owned()),
                        sensitivity: submission.sensitivity.as_str().to_owned(),
                        contradiction_refs: bounded_strings(&submission.contradiction_refs),
                        waiver_ref: submission
                            .waiver_ref
                            .as_ref()
                            .map(|value| value.as_str().to_owned()),
                    })
                    .collect(),
                consumer: crate::CharterIntakeConsumer {
                    kind: intake.consumer.kind.as_str().to_owned(),
                    id: intake.consumer.id.as_str().to_owned(),
                    version: intake.consumer.version.as_str().to_owned(),
                },
                prompt_event_refs: bounded_strings(&intake.prompt_event_refs),
                finalized_at_utc: intake.finalized_at_utc.as_str().to_owned(),
                expected_current_fingerprint: intake
                    .expected_current_fingerprint
                    .as_ref()
                    .map(|value| value.as_str().to_owned()),
            },
        },
        AuthorCharterRequestV1::Approve {
            candidate_ref,
            approval_class,
            authority_ref,
            accepted_waiver_refs,
        } => crate::CharterCommandRequest::Approve {
            candidate_ref: candidate_ref.as_str().to_owned(),
            approval_class: approval_class.as_str().to_owned(),
            authority_ref: authority_ref.as_str().to_owned(),
            accepted_waiver_refs: bounded_strings(accepted_waiver_refs),
        },
        AuthorCharterRequestV1::Promote {
            candidate_ref,
            approval_ref,
            expected_current_fingerprint,
        } => crate::CharterCommandRequest::Promote {
            candidate_ref: candidate_ref.as_str().to_owned(),
            approval_ref: approval_ref.as_str().to_owned(),
            expected_current_fingerprint: expected_current_fingerprint
                .as_ref()
                .map(|value| value.as_str().to_owned()),
        },
        AuthorCharterRequestV1::Validate => crate::CharterCommandRequest::Validate,
    }
}

fn sdk_charter_value(value: &CharterInputValueV1) -> crate::CharterInputValue {
    match value {
        CharterInputValueV1::Null => crate::CharterInputValue::Null,
        CharterInputValueV1::Boolean { value } => crate::CharterInputValue::Boolean(*value),
        CharterInputValueV1::Number { value } => crate::CharterInputValue::Number(value.clone()),
        CharterInputValueV1::String { value } => crate::CharterInputValue::String(value.clone()),
        CharterInputValueV1::Sequence { values } => crate::CharterInputValue::Sequence(
            values.as_slice().iter().map(sdk_charter_value).collect(),
        ),
        CharterInputValueV1::Mapping { entries } => crate::CharterInputValue::Mapping(
            entries
                .as_slice()
                .iter()
                .map(|entry| (entry.key.clone(), sdk_charter_value(&entry.value)))
                .collect(),
        ),
    }
}

fn sdk_charter_mode(mode: CharterAcquisitionModeV1) -> crate::CharterAcquisitionMode {
    match mode {
        CharterAcquisitionModeV1::GuidedAdaptive => crate::CharterAcquisitionMode::GuidedAdaptive,
        CharterAcquisitionModeV1::Express => crate::CharterAcquisitionMode::Express,
        CharterAcquisitionModeV1::AgentAssisted => crate::CharterAcquisitionMode::AgentAssisted,
    }
}

fn sdk_charter_source_kind(
    source_kind: CharterIntakeSourceKindV1,
) -> crate::CharterIntakeSourceKind {
    match source_kind {
        CharterIntakeSourceKindV1::UserDeclaration => {
            crate::CharterIntakeSourceKind::UserDeclaration
        }
        CharterIntakeSourceKindV1::EvidencedInference => {
            crate::CharterIntakeSourceKind::EvidencedInference
        }
        CharterIntakeSourceKindV1::DeterministicDefault => {
            crate::CharterIntakeSourceKind::DeterministicDefault
        }
        CharterIntakeSourceKindV1::KnownUnknown => crate::CharterIntakeSourceKind::KnownUnknown,
        CharterIntakeSourceKindV1::Contradiction => crate::CharterIntakeSourceKind::Contradiction,
        CharterIntakeSourceKindV1::Waiver => crate::CharterIntakeSourceKind::Waiver,
    }
}

fn sdk_project_context_request(
    request: &AuthorProjectContextRequestV1,
) -> crate::ProjectContextCommandRequest {
    match request {
        AuthorProjectContextRequestV1::Author(input) => {
            crate::ProjectContextCommandRequest::Author(sdk_project_context_input(input))
        }
        AuthorProjectContextRequestV1::Validate(input) => {
            crate::ProjectContextCommandRequest::Validate(sdk_project_context_input(input))
        }
    }
}

fn sdk_project_context_input(input: &ProjectContextInputV1) -> crate::ProjectContextInput {
    crate::ProjectContextInput {
        schema_id: input.schema_id.as_str().to_owned(),
        schema_version: input.schema_version.as_str().to_owned(),
        record_id: input.record_id.as_str().to_owned(),
        summary: input.summary.as_str().to_owned(),
        system_boundaries: bounded_strings(&input.system_boundaries),
        ownership: bounded_strings(&input.ownership),
        authoritative_references: bounded_strings(&input.authoritative_references),
        known_unknowns: bounded_strings(&input.known_unknowns),
    }
}

fn sdk_approver_request(
    request: &ApproverAdministrationRequestV1,
) -> crate::ApproverCommandRequest {
    let mappings = |values: &BoundedVec<ApproverMappingRequestV1, 128>| {
        values
            .as_slice()
            .iter()
            .map(|mapping| {
                crate::ApproverMapping::new(mapping.approval_class(), mapping.authority_ref())
            })
            .collect()
    };
    match request {
        ApproverAdministrationRequestV1::Bootstrap {
            initial_charter_quorum,
        } => crate::ApproverCommandRequest::Bootstrap {
            initial_charter_quorum: mappings(initial_charter_quorum),
        },
        ApproverAdministrationRequestV1::AddCredential { approval_mappings } => {
            crate::ApproverCommandRequest::AddCredential {
                approval_mappings: mappings(approval_mappings),
            }
        }
        ApproverAdministrationRequestV1::RevokeCredential { credential_id_hash } => {
            crate::ApproverCommandRequest::RevokeCredential {
                credential_id_hash: credential_id_hash.as_str().to_owned(),
            }
        }
        ApproverAdministrationRequestV1::UpdateMapping {
            credential_id_hash,
            approval_mappings,
        } => crate::ApproverCommandRequest::UpdateMapping {
            credential_id_hash: credential_id_hash.as_str().to_owned(),
            approval_mappings: mappings(approval_mappings),
        },
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

impl Serialize for UseCaseRequestBodyV1 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::RepositorySetup(value) => value.serialize(serializer),
            Self::RepositoryDoctor(value) => value.serialize(serializer),
            Self::AuthorCharter(value) => value.serialize(serializer),
            Self::AuthorProjectContext(value) => value.serialize(serializer),
            Self::ApproverAdministration(value) => value.serialize(serializer),
            Self::PipelineRouteResolveAndPersist(value) => value.serialize(serializer),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct UseCaseRequestV1 {
    schema: ExactBinding,
    use_case: ExactBinding,
    request: UseCaseRequestBodyV1,
    request_fingerprint: Fingerprint,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawUseCaseRequestV1 {
    schema: ExactBinding,
    use_case: ExactBinding,
    request: Value,
    request_fingerprint: Fingerprint,
}

impl UseCaseRequestV1 {
    pub fn repository_setup(
        request: RepositorySetupRequestV1,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        Self::from_body(UseCaseRequestBodyV1::RepositorySetup(request), limits)
    }

    pub fn repository_doctor(limits: &ContractLimits) -> Result<Self, ContractError> {
        Self::from_body(
            UseCaseRequestBodyV1::RepositoryDoctor(RepositoryDoctorRequestV1 {}),
            limits,
        )
    }

    pub fn author_charter(
        request: AuthorCharterRequestV1,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        Self::from_body(UseCaseRequestBodyV1::AuthorCharter(request), limits)
    }

    pub fn author_project_context(
        request: AuthorProjectContextRequestV1,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        Self::from_body(UseCaseRequestBodyV1::AuthorProjectContext(request), limits)
    }

    pub fn approver_administration(
        request: ApproverAdministrationRequestV1,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        Self::from_body(
            UseCaseRequestBodyV1::ApproverAdministration(request),
            limits,
        )
    }

    pub fn pipeline_route_resolve_and_persist(
        request: PipelineRouteResolveAndPersistRequestV1,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        Self::from_body(
            UseCaseRequestBodyV1::PipelineRouteResolveAndPersist(request),
            limits,
        )
    }

    fn from_body(
        request: UseCaseRequestBodyV1,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        let use_case = identity_binding(request.use_case_ref())?;
        Self::from_parts(
            schema_binding(USE_CASE_REQUEST_REF)?,
            use_case,
            request,
            limits,
        )
    }

    fn from_parts(
        schema: ExactBinding,
        use_case: ExactBinding,
        request: UseCaseRequestBodyV1,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        require_schema_binding(&schema, USE_CASE_REQUEST_REF)?;
        if use_case != identity_binding(request.use_case_ref())? {
            return Err(ContractError::InvalidInvariant);
        }
        request.validate()?;
        let request_fingerprint = canonical_fingerprint(
            &UseCaseRequestPreimage {
                schema: &schema,
                use_case: &use_case,
                request: &request,
            },
            limits,
        )?;
        let value = Self {
            schema,
            use_case,
            request,
            request_fingerprint,
        };
        canonical_json_bytes(&value, limits)?;
        Ok(value)
    }

    fn from_raw(raw: RawUseCaseRequestV1, limits: &ContractLimits) -> Result<Self, ContractError> {
        let request = match raw.use_case.reference().as_str() {
            SETUP_USE_CASE_REF => {
                UseCaseRequestBodyV1::RepositorySetup(parse_contract_value(&raw.request, limits)?)
            }
            DOCTOR_USE_CASE_REF => {
                UseCaseRequestBodyV1::RepositoryDoctor(parse_contract_value(&raw.request, limits)?)
            }
            AUTHOR_CHARTER_USE_CASE_REF => {
                UseCaseRequestBodyV1::AuthorCharter(parse_contract_value(&raw.request, limits)?)
            }
            AUTHOR_PROJECT_CONTEXT_USE_CASE_REF => UseCaseRequestBodyV1::AuthorProjectContext(
                parse_contract_value(&raw.request, limits)?,
            ),
            APPROVER_ADMINISTRATION_USE_CASE_REF => UseCaseRequestBodyV1::ApproverAdministration(
                parse_contract_value(&raw.request, limits)?,
            ),
            PIPELINE_ROUTE_USE_CASE_REF => UseCaseRequestBodyV1::PipelineRouteResolveAndPersist(
                parse_contract_value(&raw.request, limits)?,
            ),
            _ => return Err(ContractError::InvalidInvariant),
        };
        let value = Self::from_parts(raw.schema, raw.use_case, request, limits)?;
        if value.request_fingerprint != raw.request_fingerprint {
            return Err(ContractError::FingerprintMismatch);
        }
        Ok(value)
    }

    pub fn parse_and_admit(bytes: &[u8], limits: &ContractLimits) -> Result<Self, ContractError> {
        let value = parse_canonical_wire(bytes, limits)?;
        Self::from_raw(parse_contract_value(&value, limits)?, limits)
    }

    pub fn schema(&self) -> &ExactBinding {
        &self.schema
    }

    pub fn use_case(&self) -> &ExactBinding {
        &self.use_case
    }

    pub fn request(&self) -> &UseCaseRequestBodyV1 {
        &self.request
    }

    pub fn request_fingerprint(&self) -> &Fingerprint {
        &self.request_fingerprint
    }
}

impl<'de> Deserialize<'de> for UseCaseRequestV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw =
            RawUseCaseRequestV1::deserialize(deserializer).map_err(sanitize_deserialize_error)?;
        Self::from_raw(raw, &ContractLimits::packet0()).map_err(D::Error::custom)
    }
}

#[derive(Serialize)]
struct UseCaseRequestPreimage<'a> {
    schema: &'a ExactBinding,
    use_case: &'a ExactBinding,
    request: &'a UseCaseRequestBodyV1,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InvocationRequestKindV1 {
    Operation,
    UseCase,
}

#[derive(Debug)]
pub enum InvocationRequestBodyV1 {
    Operation(OperationRequestV1),
    UseCase(UseCaseRequestV1),
}

impl Serialize for InvocationRequestBodyV1 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Operation(value) => value.serialize(serializer),
            Self::UseCase(value) => value.serialize(serializer),
        }
    }
}

#[derive(Debug)]
pub struct InvocationRequestV1 {
    schema: ExactBinding,
    request_kind: InvocationRequestKindV1,
    request: InvocationRequestBodyV1,
    request_fingerprint: Fingerprint,
}

impl Serialize for InvocationRequestV1 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut object = serializer.serialize_struct("InvocationRequestV1", 4)?;
        object.serialize_field("schema", &self.schema)?;
        object.serialize_field("request_kind", &self.request_kind)?;
        object.serialize_field("request", &self.request)?;
        object.serialize_field("request_fingerprint", &self.request_fingerprint)?;
        object.end()
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawInvocationRequestV1 {
    schema: ExactBinding,
    request_kind: InvocationRequestKindV1,
    request: Value,
    request_fingerprint: Fingerprint,
}

impl InvocationRequestV1 {
    pub fn operation(
        request: OperationRequestV1,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        let admitted = admit_operation_bytes(&canonical_json_bytes(&request, limits)?, limits)?;
        Self::from_parts(
            schema_binding(INVOCATION_REQUEST_REF)?,
            InvocationRequestKindV1::Operation,
            InvocationRequestBodyV1::Operation(admitted),
            limits,
        )
    }

    pub fn use_case(
        request: UseCaseRequestV1,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        Self::from_parts(
            schema_binding(INVOCATION_REQUEST_REF)?,
            InvocationRequestKindV1::UseCase,
            InvocationRequestBodyV1::UseCase(request),
            limits,
        )
    }

    fn from_parts(
        schema: ExactBinding,
        request_kind: InvocationRequestKindV1,
        request: InvocationRequestBodyV1,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        require_schema_binding(&schema, INVOCATION_REQUEST_REF)?;
        if !matches!(
            (request_kind, &request),
            (
                InvocationRequestKindV1::Operation,
                InvocationRequestBodyV1::Operation(_)
            ) | (
                InvocationRequestKindV1::UseCase,
                InvocationRequestBodyV1::UseCase(_)
            )
        ) {
            return Err(ContractError::InvalidInvariant);
        }
        let request_fingerprint = canonical_fingerprint(
            &InvocationRequestPreimage {
                schema: &schema,
                request_kind,
                request: &request,
            },
            limits,
        )?;
        let value = Self {
            schema,
            request_kind,
            request,
            request_fingerprint,
        };
        canonical_json_bytes(&value, limits)?;
        Ok(value)
    }

    fn from_raw(
        raw: RawInvocationRequestV1,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        let request_bytes = canonical_json_bytes(&raw.request, limits)?;
        let request = match raw.request_kind {
            InvocationRequestKindV1::Operation => {
                InvocationRequestBodyV1::Operation(admit_operation_bytes(&request_bytes, limits)?)
            }
            InvocationRequestKindV1::UseCase => InvocationRequestBodyV1::UseCase(
                UseCaseRequestV1::parse_and_admit(&request_bytes, limits)?,
            ),
        };
        let value = Self::from_parts(raw.schema, raw.request_kind, request, limits)?;
        if value.request_fingerprint != raw.request_fingerprint {
            return Err(ContractError::FingerprintMismatch);
        }
        Ok(value)
    }

    pub fn parse_and_admit(bytes: &[u8], limits: &ContractLimits) -> Result<Self, ContractError> {
        let value = parse_canonical_wire(bytes, limits)?;
        Self::from_raw(parse_contract_value(&value, limits)?, limits)
    }

    pub fn schema(&self) -> &ExactBinding {
        &self.schema
    }

    pub fn request_kind(&self) -> InvocationRequestKindV1 {
        self.request_kind
    }

    pub fn request(&self) -> &InvocationRequestBodyV1 {
        &self.request
    }

    pub fn request_fingerprint(&self) -> &Fingerprint {
        &self.request_fingerprint
    }
}

impl<'de> Deserialize<'de> for InvocationRequestV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = RawInvocationRequestV1::deserialize(deserializer)
            .map_err(sanitize_deserialize_error)?;
        Self::from_raw(raw, &ContractLimits::packet0()).map_err(D::Error::custom)
    }
}

#[derive(Serialize)]
struct InvocationRequestPreimage<'a> {
    schema: &'a ExactBinding,
    request_kind: InvocationRequestKindV1,
    request: &'a InvocationRequestBodyV1,
}

fn admit_operation_bytes(
    bytes: &[u8],
    limits: &ContractLimits,
) -> Result<OperationRequestV1, ContractError> {
    crate::contract_membrane::invocation_dispatch::admit_operation_request_bytes(bytes, limits)
}

fn bounded<const N: usize>(value: &str) -> Result<BoundedString<N>, ContractError> {
    BoundedString::new(value.to_owned())
}

fn optional_bounded<const N: usize>(
    value: Option<&str>,
) -> Result<Option<BoundedString<N>>, ContractError> {
    value.map(bounded).transpose()
}

fn check_charter_string(value: &str) -> Result<(), ContractError> {
    if value.len() > 65_536 {
        return Err(ContractError::BoundExceeded);
    }
    Ok(())
}

fn bounded_vec<const N: usize, const M: usize>(
    values: Vec<String>,
) -> Result<BoundedVec<BoundedString<N>, M>, ContractError> {
    BoundedVec::new(
        values
            .into_iter()
            .map(BoundedString::new)
            .collect::<Result<Vec<_>, _>>()?,
    )
}

fn sanitize_deserialize_error<E: serde::de::Error>(error: E) -> E {
    E::custom(classify_contract_error_message(&error.to_string()))
}
