use crate::{
    ContextResolutionStackDefinition, CredentialAuthorityAdmissionErrorKindV1,
    CredentialAuthorityAdmissionRequestV1, CredentialAuthorityAdmissionServiceV1,
    CredentialAuthorityAdmissionV1, DefinitionFingerprint, NativeAuthenticatorPortV1,
    RetainedRepositoryRootV1,
};
use serde::Serialize;
use std::error::Error;
use std::fmt;
use std::path::Path;

pub const CONTRACT_EVIDENCE_SOURCE_APPROVAL_CLASS_V1: &str = "contract_evidence_source";
const EVIDENCE_ADMISSION_ACTION_REF: &str = "handbook.contract-evidence-admit@1.0.0";
const EVIDENCE_SOURCE_SUBJECT_REF: &str = "handbook.contract-evidence-source@1.0.0";
const MAX_REF_BYTES: usize = 2048;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceSourceAdmissionErrorKindV1 {
    InvalidRequest,
    AuthorityAbsent,
    AuthorityAmbiguous,
    StaleAuthority,
    AuthenticationRefused,
    AuthenticatorUnavailable,
    DurabilityViolation,
    ResolutionRefused,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceSourceAdmissionErrorV1 {
    kind: EvidenceSourceAdmissionErrorKindV1,
    detail: &'static str,
}

impl EvidenceSourceAdmissionErrorV1 {
    /// Bounded refusal for a read-only native source adapter. This does not
    /// construct an admission or expose retained paths or authority state.
    pub fn repository_observation_refusal_v1(kind: EvidenceSourceAdmissionErrorKindV1) -> Self {
        Self {
            kind,
            detail: "native retained source observation refused",
        }
    }

    pub fn kind(&self) -> EvidenceSourceAdmissionErrorKindV1 {
        self.kind
    }

    pub fn detail(&self) -> &str {
        self.detail
    }
}

impl fmt::Display for EvidenceSourceAdmissionErrorV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.detail)
    }
}

impl Error for EvidenceSourceAdmissionErrorV1 {}

/// Exact untrusted bindings that must be authenticated as one evidence-source
/// observation. Actor, authority, registry heads, repository root, and time
/// remain Engine-derived and cannot be supplied here.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceSourceAdmissionRequestV1 {
    candidate_ref: String,
    candidate_fingerprint: DefinitionFingerprint,
    producer_ref: String,
    producer_fingerprint: DefinitionFingerprint,
    request_ref: String,
    request_fingerprint: DefinitionFingerprint,
    execution_ref: String,
    execution_fingerprint: DefinitionFingerprint,
}

impl EvidenceSourceAdmissionRequestV1 {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        candidate_ref: &str,
        candidate_fingerprint: &str,
        producer_ref: &str,
        producer_fingerprint: &str,
        request_ref: &str,
        request_fingerprint: &str,
        execution_ref: &str,
        execution_fingerprint: &str,
    ) -> Result<Self, EvidenceSourceAdmissionErrorV1> {
        for reference in [candidate_ref, producer_ref, request_ref, execution_ref] {
            validate_ref(reference)?;
        }
        Ok(Self {
            candidate_ref: candidate_ref.to_owned(),
            candidate_fingerprint: parse_fingerprint(candidate_fingerprint)?,
            producer_ref: producer_ref.to_owned(),
            producer_fingerprint: parse_fingerprint(producer_fingerprint)?,
            request_ref: request_ref.to_owned(),
            request_fingerprint: parse_fingerprint(request_fingerprint)?,
            execution_ref: execution_ref.to_owned(),
            execution_fingerprint: parse_fingerprint(execution_fingerprint)?,
        })
    }

    pub fn candidate_ref(&self) -> &str {
        &self.candidate_ref
    }
    pub fn candidate_fingerprint(&self) -> &str {
        self.candidate_fingerprint.as_str()
    }
    pub fn producer_ref(&self) -> &str {
        &self.producer_ref
    }
    pub fn producer_fingerprint(&self) -> &str {
        self.producer_fingerprint.as_str()
    }
    pub fn request_ref(&self) -> &str {
        &self.request_ref
    }
    pub fn request_fingerprint(&self) -> &str {
        self.request_fingerprint.as_str()
    }
    pub fn execution_ref(&self) -> &str {
        &self.execution_ref
    }
    pub fn execution_fingerprint(&self) -> &str {
        self.execution_fingerprint.as_str()
    }

    fn semantic_scope_fingerprint(
        &self,
    ) -> Result<DefinitionFingerprint, EvidenceSourceAdmissionErrorV1> {
        #[derive(Serialize)]
        struct Scope<'a> {
            schema_ref: &'static str,
            candidate_ref: &'a str,
            candidate_fingerprint: &'a str,
            producer_ref: &'a str,
            producer_fingerprint: &'a str,
            request_ref: &'a str,
            request_fingerprint: &'a str,
            execution_ref: &'a str,
            execution_fingerprint: &'a str,
        }
        DefinitionFingerprint::from_json_value(
            &serde_json::to_value(Scope {
                schema_ref: "handbook.engine.evidence-source-admission-scope@1.0.0",
                candidate_ref: self.candidate_ref(),
                candidate_fingerprint: self.candidate_fingerprint(),
                producer_ref: self.producer_ref(),
                producer_fingerprint: self.producer_fingerprint(),
                request_ref: self.request_ref(),
                request_fingerprint: self.request_fingerprint(),
                execution_ref: self.execution_ref(),
                execution_fingerprint: self.execution_fingerprint(),
            })
            .map_err(|_| invalid("evidence-source scope could not be represented"))?,
        )
        .map_err(|_| invalid("evidence-source scope could not be fingerprinted"))
    }
}

/// Lifetime-sealed evidence-source authority. It deliberately implements
/// neither serialization nor cloning and cannot escape the Engine callback.
pub struct EvidenceSourceAdmissionV1<'authority> {
    request: EvidenceSourceAdmissionRequestV1,
    authority_fingerprint: DefinitionFingerprint,
    credential: CredentialAuthorityAdmissionV1<'authority>,
}

impl EvidenceSourceAdmissionV1<'_> {
    pub fn candidate_ref(&self) -> &str {
        self.request.candidate_ref()
    }
    pub fn candidate_fingerprint(&self) -> &str {
        self.request.candidate_fingerprint()
    }
    pub fn producer_ref(&self) -> &str {
        self.request.producer_ref()
    }
    pub fn producer_fingerprint(&self) -> &str {
        self.request.producer_fingerprint()
    }
    pub fn request_ref(&self) -> &str {
        self.request.request_ref()
    }
    pub fn request_fingerprint(&self) -> &str {
        self.request.request_fingerprint()
    }
    pub fn execution_ref(&self) -> &str {
        self.request.execution_ref()
    }
    pub fn execution_fingerprint(&self) -> &str {
        self.request.execution_fingerprint()
    }
    pub fn actor_ref(&self) -> &str {
        self.credential.actor_ref()
    }
    pub fn authority_ref(&self) -> &str {
        self.credential.authority_ref()
    }
    pub fn authority_fingerprint(&self) -> &str {
        self.authority_fingerprint.as_str()
    }
    pub fn registry_state_ref(&self) -> &str {
        self.credential.registry_state_ref()
    }
    pub fn registry_state_fingerprint(&self) -> &str {
        self.credential.registry_state_fingerprint()
    }
    pub fn registry_head_ref(&self) -> &str {
        self.credential.registry_head_transition_ref()
    }
    pub fn registry_head_fingerprint(&self) -> &str {
        self.credential.registry_head_transition_fingerprint()
    }
    pub fn semantic_time_utc(&self) -> &str {
        self.credential.semantic_decision_time_utc()
    }
    pub fn repository_identity_fingerprint(&self) -> &str {
        self.credential.repository_identity_fingerprint()
    }
    pub fn require_repository_root(
        &self,
        root: &RetainedRepositoryRootV1,
    ) -> Result<(), EvidenceSourceAdmissionErrorV1> {
        self.credential
            .require_repository_root(root)
            .map_err(map_credential_error)
    }
}

pub struct EvidenceSourceAdmissionServiceV1<P> {
    credential: CredentialAuthorityAdmissionServiceV1<P>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SourceCurrentnessRequestV1 {
    source_ref: String,
    source_fingerprint: DefinitionFingerprint,
    revision: String,
}

impl SourceCurrentnessRequestV1 {
    pub fn new(
        source_ref: &str,
        source_fingerprint: &str,
        revision: &str,
    ) -> Result<Self, EvidenceSourceAdmissionErrorV1> {
        validate_ref(source_ref)?;
        if revision.is_empty() || revision.len() > 1024 || revision.chars().any(char::is_control) {
            return Err(invalid("source revision is malformed"));
        }
        Ok(Self {
            source_ref: source_ref.to_owned(),
            source_fingerprint: parse_fingerprint(source_fingerprint)?,
            revision: revision.to_owned(),
        })
    }

    pub fn source_ref(&self) -> &str {
        &self.source_ref
    }
    pub fn source_fingerprint(&self) -> &str {
        self.source_fingerprint.as_str()
    }
    pub fn revision(&self) -> &str {
        &self.revision
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SourceCurrentnessObservationV1 {
    request: SourceCurrentnessRequestV1,
    current_head_ref: String,
    current_head_fingerprint: DefinitionFingerprint,
    observed_at_utc: String,
}

impl SourceCurrentnessObservationV1 {
    pub fn new(
        request: &SourceCurrentnessRequestV1,
        current_head_ref: &str,
        current_head_fingerprint: &str,
        observed_at_utc: &str,
    ) -> Result<Self, EvidenceSourceAdmissionErrorV1> {
        validate_ref(current_head_ref)?;
        if !valid_utc_second(observed_at_utc) {
            return Err(invalid("source currentness time is malformed"));
        }
        Ok(Self {
            request: request.clone(),
            current_head_ref: current_head_ref.to_owned(),
            current_head_fingerprint: parse_fingerprint(current_head_fingerprint)?,
            observed_at_utc: observed_at_utc.to_owned(),
        })
    }

    pub fn source_ref(&self) -> &str {
        self.request.source_ref()
    }
    pub fn source_fingerprint(&self) -> &str {
        self.request.source_fingerprint()
    }
    pub fn revision(&self) -> &str {
        self.request.revision()
    }
    pub fn current_head_ref(&self) -> &str {
        &self.current_head_ref
    }
    pub fn current_head_fingerprint(&self) -> &str {
        self.current_head_fingerprint.as_str()
    }
    pub fn observed_at_utc(&self) -> &str {
        &self.observed_at_utc
    }
}

/// Generic read-only current-source observation seam. Implementations may
/// consult a repository, registry, or other retained authority but cannot
/// decide contract lifecycle, evidence eligibility, or mutation.
pub trait SourceCurrentnessPortV1 {
    fn observe_current(
        &mut self,
        request: &SourceCurrentnessRequestV1,
    ) -> Result<SourceCurrentnessObservationV1, EvidenceSourceAdmissionErrorV1>;
}

impl<P: NativeAuthenticatorPortV1> EvidenceSourceAdmissionServiceV1<P> {
    pub fn new(repo_root: impl AsRef<Path>, port: P) -> Self {
        Self {
            credential: CredentialAuthorityAdmissionServiceV1::new(repo_root, port),
        }
    }

    pub fn into_port(self) -> P {
        self.credential.into_port()
    }

    pub fn admit<R>(
        &mut self,
        request: EvidenceSourceAdmissionRequestV1,
        consume: impl for<'authority> FnOnce(EvidenceSourceAdmissionV1<'authority>) -> R,
    ) -> Result<R, EvidenceSourceAdmissionErrorV1> {
        let scope = request.semantic_scope_fingerprint()?;
        let credential_request = CredentialAuthorityAdmissionRequestV1::new(
            EVIDENCE_ADMISSION_ACTION_REF,
            EVIDENCE_SOURCE_SUBJECT_REF,
            CONTRACT_EVIDENCE_SOURCE_APPROVAL_CLASS_V1,
            scope.as_str(),
        )
        .map_err(map_credential_error)?;
        self.credential
            .admit(credential_request, |credential| {
                let authority_fingerprint =
                    evidence_source_authority_fingerprint(credential.authority_ref())?;
                Ok(consume(EvidenceSourceAdmissionV1 {
                    request,
                    authority_fingerprint,
                    credential,
                }))
            })
            .map_err(map_credential_error)?
    }
}

fn evidence_source_authority_fingerprint(
    authority_ref: &str,
) -> Result<DefinitionFingerprint, EvidenceSourceAdmissionErrorV1> {
    #[derive(Serialize)]
    struct AuthorityBinding<'a> {
        schema_ref: &'static str,
        authority_ref: &'a str,
    }
    DefinitionFingerprint::from_json_value(
        &serde_json::to_value(AuthorityBinding {
            schema_ref: "handbook.engine.evidence-source-authority-binding@1.0.0",
            authority_ref,
        })
        .map_err(|_| invalid("evidence-source authority binding could not be represented"))?,
    )
    .map_err(|_| invalid("evidence-source authority binding could not be fingerprinted"))
}

/// Generic six-dimension Resolution observation. It carries no contract or
/// mutation semantics.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolutionObservationV1 {
    stack_ref: String,
    stack_fingerprint: DefinitionFingerprint,
    values: [String; 6],
}

impl ResolutionObservationV1 {
    pub fn new(
        stack_ref: &str,
        stack_fingerprint: &str,
        values: [&str; 6],
    ) -> Result<Self, EvidenceSourceAdmissionErrorV1> {
        validate_ref(stack_ref)?;
        if values.iter().any(|value| {
            value.is_empty()
                || value.len() > 128
                || value
                    .bytes()
                    .any(|byte| byte.is_ascii_control() || byte.is_ascii_whitespace())
        }) {
            return Err(invalid("Resolution dimension value is malformed"));
        }
        Ok(Self {
            stack_ref: stack_ref.to_owned(),
            stack_fingerprint: parse_fingerprint(stack_fingerprint)?,
            values: values.map(str::to_owned),
        })
    }

    pub fn stack_ref(&self) -> &str {
        &self.stack_ref
    }
    pub fn stack_fingerprint(&self) -> &str {
        self.stack_fingerprint.as_str()
    }
    pub fn values(&self) -> [&str; 6] {
        self.values.each_ref().map(String::as_str)
    }
}

pub fn effective_resolution_minimum_v1(
    stack: &ContextResolutionStackDefinition,
    observations: &[ResolutionObservationV1],
) -> Result<ResolutionObservationV1, EvidenceSourceAdmissionErrorV1> {
    let first = observations
        .first()
        .ok_or_else(|| resolution_refused("Resolution observation set is empty"))?;
    let stack_ref = stack.exact_ref().as_str();
    let stack_fingerprint = stack.definition_fingerprint().as_str();
    if observations.iter().any(|observation| {
        observation.stack_ref() != stack_ref || observation.stack_fingerprint() != stack_fingerprint
    }) {
        return Err(resolution_refused(
            "Resolution observation stack binding is substituted",
        ));
    }
    let mut selected = first.values().map(str::to_owned);
    let mut selected_ranks = stack
        .dimension_ranks(first.values())
        .ok_or_else(|| resolution_refused("Resolution dimension is unknown"))?;
    for observation in &observations[1..] {
        let ranks = stack
            .dimension_ranks(observation.values())
            .ok_or_else(|| resolution_refused("Resolution dimension is unknown"))?;
        let values = observation.values();
        for index in 0..6 {
            if ranks[index] < selected_ranks[index] {
                selected_ranks[index] = ranks[index];
                selected[index] = values[index].to_owned();
            }
        }
    }
    ResolutionObservationV1::new(
        stack_ref,
        stack_fingerprint,
        selected.each_ref().map(String::as_str),
    )
}

pub fn resolution_meets_minimum_v1(
    stack: &ContextResolutionStackDefinition,
    actual: &ResolutionObservationV1,
    minimum: &ResolutionObservationV1,
) -> Result<bool, EvidenceSourceAdmissionErrorV1> {
    if actual.stack_ref() != minimum.stack_ref()
        || actual.stack_fingerprint() != minimum.stack_fingerprint()
        || actual.stack_ref() != stack.exact_ref().as_str()
        || actual.stack_fingerprint() != stack.definition_fingerprint().as_str()
    {
        return Err(resolution_refused(
            "Resolution minimum stack is substituted",
        ));
    }
    let actual = stack
        .dimension_ranks(actual.values())
        .ok_or_else(|| resolution_refused("actual Resolution dimension is unknown"))?;
    let minimum = stack
        .dimension_ranks(minimum.values())
        .ok_or_else(|| resolution_refused("minimum Resolution dimension is unknown"))?;
    Ok((0..6).all(|index| actual[index] >= minimum[index]))
}

fn validate_ref(value: &str) -> Result<(), EvidenceSourceAdmissionErrorV1> {
    if value.is_empty()
        || value.len() > MAX_REF_BYTES
        || !value.is_ascii()
        || value.starts_with('/')
        || value.ends_with('/')
        || value.contains("//")
        || value.contains('\\')
        || value
            .split('/')
            .any(|segment| matches!(segment, "." | ".."))
        || value
            .bytes()
            .any(|byte| byte.is_ascii_control() || byte.is_ascii_whitespace())
    {
        return Err(invalid("evidence-source reference is malformed"));
    }
    Ok(())
}

fn parse_fingerprint(value: &str) -> Result<DefinitionFingerprint, EvidenceSourceAdmissionErrorV1> {
    DefinitionFingerprint::parse(value)
        .map_err(|_| invalid("evidence-source fingerprint is malformed"))
}

fn valid_utc_second(value: &str) -> bool {
    let bytes = value.as_bytes();
    if bytes.len() != 20
        || bytes[4] != b'-'
        || bytes[7] != b'-'
        || bytes[10] != b'T'
        || bytes[13] != b':'
        || bytes[16] != b':'
        || bytes[19] != b'Z'
        || bytes.iter().enumerate().any(|(index, byte)| {
            !matches!(index, 4 | 7 | 10 | 13 | 16 | 19) && !byte.is_ascii_digit()
        })
    {
        return false;
    }
    let number = |start: usize, end: usize| value[start..end].parse::<u32>().ok();
    let (Some(year), Some(month), Some(day), Some(hour), Some(minute), Some(second)) = (
        number(0, 4),
        number(5, 7),
        number(8, 10),
        number(11, 13),
        number(14, 16),
        number(17, 19),
    ) else {
        return false;
    };
    let leap = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
    let days = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if leap => 29,
        2 => 28,
        _ => return false,
    };
    (1..=days).contains(&day) && hour < 24 && minute < 60 && second < 60
}

fn map_credential_error(
    error: crate::CredentialAuthorityAdmissionErrorV1,
) -> EvidenceSourceAdmissionErrorV1 {
    let kind = match error.kind() {
        CredentialAuthorityAdmissionErrorKindV1::InvalidRequest => {
            EvidenceSourceAdmissionErrorKindV1::InvalidRequest
        }
        CredentialAuthorityAdmissionErrorKindV1::AuthorityAbsent => {
            EvidenceSourceAdmissionErrorKindV1::AuthorityAbsent
        }
        CredentialAuthorityAdmissionErrorKindV1::AuthorityAmbiguous => {
            EvidenceSourceAdmissionErrorKindV1::AuthorityAmbiguous
        }
        CredentialAuthorityAdmissionErrorKindV1::StaleAuthority => {
            EvidenceSourceAdmissionErrorKindV1::StaleAuthority
        }
        CredentialAuthorityAdmissionErrorKindV1::AuthenticationRefused => {
            EvidenceSourceAdmissionErrorKindV1::AuthenticationRefused
        }
        CredentialAuthorityAdmissionErrorKindV1::AuthenticatorUnavailable => {
            EvidenceSourceAdmissionErrorKindV1::AuthenticatorUnavailable
        }
        CredentialAuthorityAdmissionErrorKindV1::DurabilityViolation => {
            EvidenceSourceAdmissionErrorKindV1::DurabilityViolation
        }
    };
    refusal(kind, "credential-backed evidence-source admission refused")
}

fn invalid(detail: &'static str) -> EvidenceSourceAdmissionErrorV1 {
    refusal(EvidenceSourceAdmissionErrorKindV1::InvalidRequest, detail)
}

fn resolution_refused(detail: &'static str) -> EvidenceSourceAdmissionErrorV1 {
    refusal(
        EvidenceSourceAdmissionErrorKindV1::ResolutionRefused,
        detail,
    )
}

const fn refusal(
    kind: EvidenceSourceAdmissionErrorKindV1,
    detail: &'static str,
) -> EvidenceSourceAdmissionErrorV1 {
    EvidenceSourceAdmissionErrorV1 { kind, detail }
}
