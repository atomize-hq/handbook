//! Closed, transport-free requests and results for the Author and Approvers
//! routes. YAML decoding and legacy JSON rendering deliberately live in the
//! CLI; engine record conversion remains private to the SDK.

use crate::{
    author::project_context,
    charter_product::{self, ApproverAdapterResult, ApproverAdminIntent, CharterCommandIntent},
    HandbookSdkV1,
};
use handbook_engine::{
    CharterAcquisitionMode as EngineCharterAcquisitionMode,
    CharterCoverageSubmission as EngineCharterCoverageSubmission,
    CharterIntakeConsumer as EngineCharterIntakeConsumer,
    CharterIntakeEnvelope as EngineCharterIntakeEnvelope,
    CharterIntakeSourceKind as EngineCharterIntakeSourceKind,
};
use serde_json::{Map, Number, Value};

pub use crate::author::project_context::{
    AuthorProjectContextRefusal, AuthorProjectContextRefusalKind, AuthorProjectContextResult,
};
pub use crate::charter_product::{
    AdapterOperationStatus as AuthorOperationStatus, AdapterRefusal as AuthorOperationRefusal,
    ApproverAdapterResult as ApproverOperationResult, ApproverAdminOperation as ApproverOperation,
    ApproverLegacyProjection as ApproverOperationProjection, CharterOperation,
    CharterOperationResult, RepositoryInvocationFailureView,
};

/// Acquisition modes accepted by the closed Charter Author request.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CharterAcquisitionMode {
    GuidedAdaptive,
    Express,
    AgentAssisted,
}

impl From<CharterAcquisitionMode> for EngineCharterAcquisitionMode {
    fn from(value: CharterAcquisitionMode) -> Self {
        match value {
            CharterAcquisitionMode::GuidedAdaptive => Self::GuidedAdaptive,
            CharterAcquisitionMode::Express => Self::Express,
            CharterAcquisitionMode::AgentAssisted => Self::AgentAssisted,
        }
    }
}

/// A recursive, closed Charter content value. This is intentionally not a
/// serde or JSON value: transports stay outside the SDK boundary.
#[derive(Clone, Debug, PartialEq)]
pub enum CharterInputValue {
    Null,
    Boolean(bool),
    Number(String),
    String(String),
    Sequence(Vec<CharterInputValue>),
    Mapping(Vec<(String, CharterInputValue)>),
}

impl CharterInputValue {
    fn into_engine(self) -> Result<Value, ()> {
        match self {
            Self::Null => Ok(Value::Null),
            Self::Boolean(value) => Ok(Value::Bool(value)),
            Self::Number(value) => value.parse::<Number>().map(Value::Number).map_err(|_| ()),
            Self::String(value) => Ok(Value::String(value)),
            Self::Sequence(values) => values
                .into_iter()
                .map(Self::into_engine)
                .collect::<Result<Vec<_>, _>>()
                .map(Value::Array),
            Self::Mapping(entries) => {
                let mut document = Map::new();
                for (key, value) in entries {
                    if document.insert(key, value.into_engine()?).is_some() {
                        return Err(());
                    }
                }
                Ok(Value::Object(document))
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CharterIntakeSourceKind {
    UserDeclaration,
    EvidencedInference,
    DeterministicDefault,
    KnownUnknown,
    Contradiction,
    Waiver,
}

impl From<CharterIntakeSourceKind> for EngineCharterIntakeSourceKind {
    fn from(value: CharterIntakeSourceKind) -> Self {
        match value {
            CharterIntakeSourceKind::UserDeclaration => Self::UserDeclaration,
            CharterIntakeSourceKind::EvidencedInference => Self::EvidencedInference,
            CharterIntakeSourceKind::DeterministicDefault => Self::DeterministicDefault,
            CharterIntakeSourceKind::KnownUnknown => Self::KnownUnknown,
            CharterIntakeSourceKind::Contradiction => Self::Contradiction,
            CharterIntakeSourceKind::Waiver => Self::Waiver,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CharterCoverageSubmission {
    pub coverage_id: String,
    pub source_kind: CharterIntakeSourceKind,
    pub value_ref: String,
    pub evidence_refs: Vec<String>,
    pub confidence: String,
    pub freshness: Option<String>,
    pub sensitivity: String,
    pub contradiction_refs: Vec<String>,
    pub waiver_ref: Option<String>,
}

impl From<CharterCoverageSubmission> for EngineCharterCoverageSubmission {
    fn from(value: CharterCoverageSubmission) -> Self {
        Self {
            coverage_id: value.coverage_id,
            source_kind: value.source_kind.into(),
            value_ref: value.value_ref,
            evidence_refs: value.evidence_refs,
            confidence: value.confidence,
            freshness: value.freshness,
            sensitivity: value.sensitivity,
            contradiction_refs: value.contradiction_refs,
            waiver_ref: value.waiver_ref,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CharterIntakeConsumer {
    pub kind: String,
    pub id: String,
    pub version: String,
}

impl From<CharterIntakeConsumer> for EngineCharterIntakeConsumer {
    fn from(value: CharterIntakeConsumer) -> Self {
        Self {
            kind: value.kind,
            id: value.id,
            version: value.version,
        }
    }
}

/// Fully typed Charter intake. The CLI is responsible for converting its
/// selected input format into this transport-free value.
#[derive(Clone, Debug, PartialEq)]
pub struct CharterIntakeEnvelope {
    pub mode: CharterAcquisitionMode,
    pub content: CharterInputValue,
    pub coverage: Vec<CharterCoverageSubmission>,
    pub consumer: CharterIntakeConsumer,
    pub prompt_event_refs: Vec<String>,
    pub finalized_at_utc: String,
    pub expected_current_fingerprint: Option<String>,
}

impl CharterIntakeEnvelope {
    fn into_engine(self) -> Result<EngineCharterIntakeEnvelope, ()> {
        Ok(EngineCharterIntakeEnvelope {
            mode: self.mode.into(),
            content: self.content.into_engine()?,
            coverage: self.coverage.into_iter().map(Into::into).collect(),
            consumer: self.consumer.into(),
            prompt_event_refs: self.prompt_event_refs,
            finalized_at_utc: self.finalized_at_utc,
            expected_current_fingerprint: self.expected_current_fingerprint,
        })
    }
}

/// Closed command variants for the Charter route.
#[derive(Clone, Debug, PartialEq)]
pub enum CharterCommandRequest {
    Author {
        mode: CharterAcquisitionMode,
        intake: CharterIntakeEnvelope,
    },
    Approve {
        candidate_ref: String,
        approval_class: String,
        authority_ref: String,
        accepted_waiver_refs: Vec<String>,
    },
    Promote {
        candidate_ref: String,
        approval_ref: String,
        expected_current_fingerprint: Option<String>,
    },
    Validate,
}

impl CharterCommandRequest {
    fn into_internal(self) -> Result<CharterCommandIntent, ()> {
        match self {
            Self::Author { mode, intake } => intake
                .into_engine()
                .map(|envelope| CharterCommandIntent::Author {
                    mode: mode.into(),
                    envelope,
                })
                .map_err(|_| ()),
            Self::Approve {
                candidate_ref,
                approval_class,
                authority_ref,
                accepted_waiver_refs,
            } => Ok(CharterCommandIntent::Approve {
                candidate_ref,
                approval_class,
                authority_ref,
                accepted_waiver_refs,
            }),
            Self::Promote {
                candidate_ref,
                approval_ref,
                expected_current_fingerprint,
            } => Ok(CharterCommandIntent::Promote {
                candidate_ref,
                approval_ref,
                expected_current_fingerprint,
            }),
            Self::Validate => Ok(CharterCommandIntent::Validate),
        }
    }
}

/// Typed request for the Project Context authoring operation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectContextInput {
    pub schema_id: String,
    pub schema_version: String,
    pub record_id: String,
    pub summary: String,
    pub system_boundaries: Vec<String>,
    pub ownership: Vec<String>,
    pub authoritative_references: Vec<String>,
    pub known_unknowns: Vec<String>,
}

/// Closed approver-administration requests.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ApproverCommandRequest {
    Bootstrap {
        initial_charter_quorum: Vec<ApproverMapping>,
    },
    AddCredential {
        approval_mappings: Vec<ApproverMapping>,
    },
    RevokeCredential {
        credential_id_hash: String,
    },
    UpdateMapping {
        credential_id_hash: String,
        approval_mappings: Vec<ApproverMapping>,
    },
}

/// One closed approval-class to authority-reference assignment.
///
/// CLI `class=authority` grammar is decoded before constructing this value.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ApproverMapping {
    approval_class: String,
    authority_ref: String,
}

impl ApproverMapping {
    pub fn new(approval_class: impl Into<String>, authority_ref: impl Into<String>) -> Self {
        Self {
            approval_class: approval_class.into(),
            authority_ref: authority_ref.into(),
        }
    }

    pub fn approval_class(&self) -> &str {
        &self.approval_class
    }

    pub fn authority_ref(&self) -> &str {
        &self.authority_ref
    }
}

impl From<ApproverCommandRequest> for ApproverAdminIntent {
    fn from(value: ApproverCommandRequest) -> Self {
        match value {
            ApproverCommandRequest::Bootstrap {
                initial_charter_quorum,
            } => Self::Bootstrap {
                initial_charter_quorum,
            },
            ApproverCommandRequest::AddCredential { approval_mappings } => {
                Self::AddCredential { approval_mappings }
            }
            ApproverCommandRequest::RevokeCredential { credential_id_hash } => {
                Self::RevokeCredential { credential_id_hash }
            }
            ApproverCommandRequest::UpdateMapping {
                credential_id_hash,
                approval_mappings,
            } => Self::UpdateMapping {
                credential_id_hash,
                approval_mappings,
            },
        }
    }
}

impl CharterOperationResult {
    /// Produces the fixed refusal for the legacy unselected `--from-inputs`
    /// grammar. Parsing and command selection remain CLI responsibilities.
    pub fn legacy_input_refused() -> Self {
        charter_product::legacy_charter_input_refusal()
    }

    /// Produces the fixed refusal for an invalid selected command grammar.
    pub fn invalid_request(operation: CharterOperation) -> Self {
        charter_product::invalid_charter_command_refusal(operation)
    }

    /// Produces the fixed refusal for a CLI-decoded intake that cannot become
    /// the engine's closed intake record.
    pub fn invalid_intake_envelope() -> Self {
        charter_product::invalid_intake_envelope_refusal()
    }
}

impl HandbookSdkV1 {
    /// Executes one closed Charter operation for this repository.
    pub fn author_charter(&self, request: CharterCommandRequest) -> CharterOperationResult {
        match request.into_internal() {
            Ok(intent) => charter_product::execute_charter_command(&self.repo_root, intent),
            Err(()) => CharterOperationResult::invalid_intake_envelope(),
        }
    }

    /// Validates a typed Project Context input without mutating repository
    /// state.
    pub fn validate_project_context(
        &self,
        input: &ProjectContextInput,
    ) -> Result<(), AuthorProjectContextRefusal> {
        project_context::validate_project_context_input(input)
    }

    /// Performs only the Project Context mutation preflight for this
    /// repository.
    pub fn preflight_project_context_authoring(&self) -> Result<(), AuthorProjectContextRefusal> {
        project_context::preflight_author_project_context(&self.repo_root)
    }

    /// Authors a typed Project Context input for this repository.
    pub fn author_project_context(
        &self,
        input: &ProjectContextInput,
    ) -> Result<AuthorProjectContextResult, AuthorProjectContextRefusal> {
        project_context::author_project_context_from_input(&self.repo_root, input)
    }

    /// Executes a closed approver-administration operation for this
    /// repository.
    pub fn manage_approvers(&self, request: ApproverCommandRequest) -> ApproverAdapterResult {
        charter_product::execute_approver_admin_intent(&self.repo_root, request.into())
    }
}
