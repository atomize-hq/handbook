use crate::approver_registry_observation::{
    observe_committed_approver_registry_if_present_locked,
    observe_committed_approver_registry_locked, ApproverRegistryObservationErrorKindV1,
    ApproverRegistryObservationErrorV1, CommittedApproverCredentialV1,
    RetainedApproverRegistryObservationV1,
};
use crate::charter_authenticator::{
    decode_and_verify_get_assertion_response, encode_get_assertion_request,
    AuthenticatorCredentialV1, NativeAuthenticatorPortErrorV1, NativeAuthenticatorPortV1,
};
use crate::charter_authority_workflow::RegistryAuthorityLocks;
use crate::{DefinitionFingerprint, ExactDefinitionRef};
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::error::Error;
use std::fmt;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

const MAX_APPROVAL_CLASS_BYTES: usize = 256;
const MAX_CHALLENGE_BYTES: usize = 16_384;

/// Untrusted semantic scope for one credential-backed authority decision.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CredentialAuthorityAdmissionRequestV1 {
    action_ref: ExactDefinitionRef,
    subject_ref: ExactDefinitionRef,
    approval_class: String,
    semantic_scope_fingerprint: DefinitionFingerprint,
}

impl CredentialAuthorityAdmissionRequestV1 {
    /// Validates an exact owner action, subject, approval class, and semantic
    /// scope. Actor, authority, current head, and time are deliberately absent.
    pub fn new(
        action_ref: &str,
        subject_ref: &str,
        approval_class: &str,
        semantic_scope_fingerprint: &str,
    ) -> Result<Self, CredentialAuthorityAdmissionErrorV1> {
        let action_ref = ExactDefinitionRef::parse(action_ref)
            .map_err(|_| invalid("authority action ref is not exact"))?;
        let subject_ref = ExactDefinitionRef::parse(subject_ref)
            .map_err(|_| invalid("authority subject ref is not exact"))?;
        if !valid_approval_class(approval_class) {
            return Err(invalid(
                "approval class is not a bounded lowercase selector",
            ));
        }
        let semantic_scope_fingerprint =
            DefinitionFingerprint::parse(semantic_scope_fingerprint)
                .map_err(|_| invalid("semantic scope fingerprint is not exact"))?;
        Ok(Self {
            action_ref,
            subject_ref,
            approval_class: approval_class.to_owned(),
            semantic_scope_fingerprint,
        })
    }

    pub fn action_ref(&self) -> &str {
        self.action_ref.as_str()
    }

    pub fn subject_ref(&self) -> &str {
        self.subject_ref.as_str()
    }

    pub fn approval_class(&self) -> &str {
        &self.approval_class
    }

    pub fn semantic_scope_fingerprint(&self) -> &str {
        self.semantic_scope_fingerprint.as_str()
    }
}

/// Closed refusal categories for generic credential authority admission.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CredentialAuthorityAdmissionErrorKindV1 {
    InvalidRequest,
    AuthorityAbsent,
    AuthorityAmbiguous,
    StaleAuthority,
    AuthenticationRefused,
    AuthenticatorUnavailable,
    DurabilityViolation,
}

/// A bounded public refusal that never contains credential or assertion bytes.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CredentialAuthorityAdmissionErrorV1 {
    kind: CredentialAuthorityAdmissionErrorKindV1,
    detail: &'static str,
}

impl CredentialAuthorityAdmissionErrorV1 {
    pub fn kind(&self) -> CredentialAuthorityAdmissionErrorKindV1 {
        self.kind
    }

    pub fn detail(&self) -> &str {
        self.detail
    }
}

impl fmt::Display for CredentialAuthorityAdmissionErrorV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.detail)
    }
}

impl Error for CredentialAuthorityAdmissionErrorV1 {}

/// A sealed, lifetime-bounded admission. It can only exist while the exact
/// committed registry authority remains locked by Engine.
pub struct CredentialAuthorityAdmissionV1<'authority> {
    actor_ref: String,
    approval_class: String,
    authority_ref: String,
    repository_identity_fingerprint: String,
    registry_state_ref: String,
    registry_state_fingerprint: String,
    registry_head_transition_ref: String,
    registry_head_transition_fingerprint: String,
    credential_registration_ref: String,
    credential_registration_fingerprint: String,
    authenticator_assertion_fingerprint: String,
    action_ref: String,
    subject_ref: String,
    semantic_scope_fingerprint: String,
    semantic_decision_time_utc: String,
    _authority_lock: &'authority RegistryAuthorityLocks,
}

impl CredentialAuthorityAdmissionV1<'_> {
    pub fn actor_ref(&self) -> &str {
        &self.actor_ref
    }

    pub fn approval_class(&self) -> &str {
        &self.approval_class
    }

    pub fn authority_ref(&self) -> &str {
        &self.authority_ref
    }

    pub fn repository_identity_fingerprint(&self) -> &str {
        &self.repository_identity_fingerprint
    }

    pub fn registry_state_ref(&self) -> &str {
        &self.registry_state_ref
    }

    pub fn registry_state_fingerprint(&self) -> &str {
        &self.registry_state_fingerprint
    }

    pub fn registry_head_transition_ref(&self) -> &str {
        &self.registry_head_transition_ref
    }

    pub fn registry_head_transition_fingerprint(&self) -> &str {
        &self.registry_head_transition_fingerprint
    }

    pub fn credential_registration_ref(&self) -> &str {
        &self.credential_registration_ref
    }

    pub fn credential_registration_fingerprint(&self) -> &str {
        &self.credential_registration_fingerprint
    }

    pub fn authenticator_assertion_fingerprint(&self) -> &str {
        &self.authenticator_assertion_fingerprint
    }

    pub fn action_ref(&self) -> &str {
        &self.action_ref
    }

    pub fn subject_ref(&self) -> &str {
        &self.subject_ref
    }

    pub fn semantic_scope_fingerprint(&self) -> &str {
        &self.semantic_scope_fingerprint
    }

    pub fn semantic_decision_time_utc(&self) -> &str {
        &self.semantic_decision_time_utc
    }
}

/// Engine-owned generic authority admission over the committed approver root.
pub struct CredentialAuthorityAdmissionServiceV1<P> {
    repo_root: PathBuf,
    port: P,
}

impl<P: NativeAuthenticatorPortV1> CredentialAuthorityAdmissionServiceV1<P> {
    pub fn new(repo_root: impl AsRef<Path>, port: P) -> Self {
        Self {
            repo_root: repo_root.as_ref().to_path_buf(),
            port,
        }
    }

    pub fn into_port(self) -> P {
        self.port
    }

    /// Authenticates exactly one resolved credential and invokes `consume`
    /// while the committed registry lock and rechecked current head are held.
    /// The higher-ranked lifetime prevents the sealed admission from escaping.
    pub fn admit<R>(
        &mut self,
        request: CredentialAuthorityAdmissionRequestV1,
        consume: impl for<'authority> FnOnce(CredentialAuthorityAdmissionV1<'authority>) -> R,
    ) -> Result<R, CredentialAuthorityAdmissionErrorV1> {
        let locks = RegistryAuthorityLocks::acquire(&self.repo_root).map_err(|_| {
            refusal(
                CredentialAuthorityAdmissionErrorKindV1::DurabilityViolation,
                "committed authority locks could not be acquired safely",
            )
        })?;
        let authority = observe_committed_approver_registry_if_present_locked(&self.repo_root)
            .map_err(map_initial_observation_error)?
            .ok_or_else(authority_absent)?;
        let (credential, authority_ref) = resolve_exact_credential(&authority, &request)?;
        let credential = credential.clone();

        let mut nonce = [0_u8; 32];
        getrandom::fill(&mut nonce).map_err(|_| {
            refusal(
                CredentialAuthorityAdmissionErrorKindV1::DurabilityViolation,
                "operating-system randomness was unavailable",
            )
        })?;
        let nonce_fingerprint = DefinitionFingerprint::from_bytes(&nonce);
        let challenge = AuthorityChallengeV1 {
            schema_ref: "handbook.engine.credential-authority-challenge@1.0.0",
            action_ref: request.action_ref(),
            subject_ref: request.subject_ref(),
            approval_class: request.approval_class(),
            authority_ref: &authority_ref,
            repository_identity_fingerprint: required_state_string(
                &authority,
                "repository_identity_fingerprint",
            )?,
            registry_state_ref: &authority.state_ref,
            registry_state_fingerprint: &authority.state_fingerprint,
            registry_head_transition_ref: &authority.head_transition_ref,
            registry_head_transition_fingerprint: &authority.head_transition_fingerprint,
            semantic_scope_fingerprint: request.semantic_scope_fingerprint(),
            nonce_fingerprint: nonce_fingerprint.as_str(),
        };
        let challenge_bytes = serde_json_canonicalizer::to_vec(&challenge).map_err(|_| {
            refusal(
                CredentialAuthorityAdmissionErrorKindV1::InvalidRequest,
                "authority challenge could not be canonicalized",
            )
        })?;
        if challenge_bytes.len() > MAX_CHALLENGE_BYTES {
            return Err(invalid("authority challenge exceeds the admitted bound"));
        }
        let client_data_hash: [u8; 32] = Sha256::digest(&challenge_bytes).into();
        let request_cbor = encode_get_assertion_request(
            client_data_hash,
            std::slice::from_ref(&credential.credential_id),
        )
        .map_err(|_| authentication_refused())?;
        let raw_response = self
            .port
            .get_assertion(&request_cbor)
            .map_err(|error| match error {
                NativeAuthenticatorPortErrorV1::Unavailable => refusal(
                    CredentialAuthorityAdmissionErrorKindV1::AuthenticatorUnavailable,
                    "native authenticator API is unavailable",
                ),
                NativeAuthenticatorPortErrorV1::Transport => refusal(
                    CredentialAuthorityAdmissionErrorKindV1::DurabilityViolation,
                    "authenticator transport ended without a complete decision",
                ),
            })?;
        let decoded = decode_and_verify_get_assertion_response(
            &raw_response,
            &[AuthenticatorCredentialV1 {
                credential_id: credential.credential_id.clone(),
                cose_public_key: credential.cose_public_key.clone(),
            }],
            client_data_hash,
        )
        .map_err(|_| authentication_refused())?;
        if decoded.credential_id != credential.credential_id
            || !valid_counter(credential.sign_count, decoded.sign_count)
        {
            return Err(authentication_refused());
        }

        let current = observe_committed_approver_registry_locked(&self.repo_root)
            .map_err(map_recheck_observation_error)?;
        if !same_authority(&authority, &current)
            || current
                .credentials
                .iter()
                .find(|candidate| candidate.credential_id_hash == credential.credential_id_hash)
                != Some(&credential)
        {
            return Err(stale_authority());
        }

        let repository_identity_fingerprint =
            required_state_string(&authority, "repository_identity_fingerprint")?.to_owned();
        let actor_ref = format!(
            "credential.sha256-{}",
            credential
                .credential_id_hash
                .strip_prefix("sha256:")
                .expect("observed credential hash is normalized")
        );
        let admission = CredentialAuthorityAdmissionV1 {
            actor_ref,
            approval_class: request.approval_class,
            authority_ref,
            repository_identity_fingerprint,
            registry_state_ref: authority.state_ref,
            registry_state_fingerprint: authority.state_fingerprint,
            registry_head_transition_ref: authority.head_transition_ref,
            registry_head_transition_fingerprint: authority.head_transition_fingerprint,
            credential_registration_ref: credential.registration_ref.clone(),
            credential_registration_fingerprint: credential.registration_fingerprint.clone(),
            authenticator_assertion_fingerprint: DefinitionFingerprint::from_bytes(&raw_response)
                .to_string(),
            action_ref: request.action_ref.to_string(),
            subject_ref: request.subject_ref.to_string(),
            semantic_scope_fingerprint: request.semantic_scope_fingerprint.to_string(),
            semantic_decision_time_utc: current_semantic_utc()?,
            _authority_lock: &locks,
        };
        Ok(consume(admission))
    }
}

#[derive(Serialize)]
struct AuthorityChallengeV1<'a> {
    schema_ref: &'static str,
    action_ref: &'a str,
    subject_ref: &'a str,
    approval_class: &'a str,
    authority_ref: &'a str,
    repository_identity_fingerprint: &'a str,
    registry_state_ref: &'a str,
    registry_state_fingerprint: &'a str,
    registry_head_transition_ref: &'a str,
    registry_head_transition_fingerprint: &'a str,
    semantic_scope_fingerprint: &'a str,
    nonce_fingerprint: &'a str,
}

fn resolve_exact_credential<'a>(
    authority: &'a RetainedApproverRegistryObservationV1,
    request: &CredentialAuthorityAdmissionRequestV1,
) -> Result<(&'a CommittedApproverCredentialV1, String), CredentialAuthorityAdmissionErrorV1> {
    let mut matches = Vec::new();
    for credential in &authority.credentials {
        if !credential.active || credential.use_sequence >= 4096 {
            continue;
        }
        for mapping in &credential.approval_mappings {
            if mapping.approval_class == request.approval_class {
                ExactDefinitionRef::parse(&mapping.authority_ref).map_err(|_| {
                    refusal(
                        CredentialAuthorityAdmissionErrorKindV1::AuthorityAmbiguous,
                        "resolved authority ref is not one exact definition ref",
                    )
                })?;
                matches.push((credential, mapping.authority_ref.clone()));
            }
        }
    }
    match matches.as_slice() {
        [] => Err(authority_absent()),
        [(credential, authority_ref)] => Ok((*credential, authority_ref.clone())),
        _ => Err(refusal(
            CredentialAuthorityAdmissionErrorKindV1::AuthorityAmbiguous,
            "approval class resolves to more than one active credential authority",
        )),
    }
}

fn same_authority(
    expected: &RetainedApproverRegistryObservationV1,
    current: &RetainedApproverRegistryObservationV1,
) -> bool {
    expected.state_ref == current.state_ref
        && expected.state_fingerprint == current.state_fingerprint
        && expected.state_bytes == current.state_bytes
        && expected.head_transition_ref == current.head_transition_ref
        && expected.head_transition_fingerprint == current.head_transition_fingerprint
        && expected.head_transition_bytes == current.head_transition_bytes
}

fn required_state_string<'a>(
    authority: &'a RetainedApproverRegistryObservationV1,
    field: &str,
) -> Result<&'a str, CredentialAuthorityAdmissionErrorV1> {
    authority
        .state
        .get(field)
        .and_then(serde_json::Value::as_str)
        .filter(|value| DefinitionFingerprint::parse(value).is_ok())
        .ok_or_else(stale_authority)
}

fn map_initial_observation_error(
    error: ApproverRegistryObservationErrorV1,
) -> CredentialAuthorityAdmissionErrorV1 {
    match error.kind() {
        ApproverRegistryObservationErrorKindV1::AuthorityAbsent => authority_absent(),
        ApproverRegistryObservationErrorKindV1::DurabilityViolation
        | ApproverRegistryObservationErrorKindV1::UnsafeFilesystem
        | ApproverRegistryObservationErrorKindV1::LineageViolation => durability_violation(),
    }
}

fn map_recheck_observation_error(
    error: ApproverRegistryObservationErrorV1,
) -> CredentialAuthorityAdmissionErrorV1 {
    match error.kind() {
        ApproverRegistryObservationErrorKindV1::AuthorityAbsent
        | ApproverRegistryObservationErrorKindV1::LineageViolation => stale_authority(),
        ApproverRegistryObservationErrorKindV1::DurabilityViolation
        | ApproverRegistryObservationErrorKindV1::UnsafeFilesystem => durability_violation(),
    }
}

fn valid_counter(prior: u32, current: u32) -> bool {
    (prior == 0 && current == 0) || current > prior
}

fn valid_approval_class(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= MAX_APPROVAL_CLASS_BYTES
        && value.is_ascii()
        && value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
        && value.as_bytes()[0].is_ascii_lowercase()
}

fn current_semantic_utc() -> Result<String, CredentialAuthorityAdmissionErrorV1> {
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| {
            refusal(
                CredentialAuthorityAdmissionErrorKindV1::DurabilityViolation,
                "system time is before the Unix epoch",
            )
        })?
        .as_secs();
    let days = (seconds / 86_400) as i64;
    let seconds_of_day = seconds % 86_400;
    let (year, month, day) = civil_date_from_unix_days(days);
    let hour = seconds_of_day / 3_600;
    let minute = (seconds_of_day % 3_600) / 60;
    let second = seconds_of_day % 60;
    Ok(format!(
        "{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}Z"
    ))
}

fn civil_date_from_unix_days(days: i64) -> (i64, i64, i64) {
    let shifted = days + 719_468;
    let era = if shifted >= 0 {
        shifted / 146_097
    } else {
        (shifted - 146_096) / 146_097
    };
    let day_of_era = shifted - era * 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let mut year = year_of_era + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_prime = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_prime + 2) / 5 + 1;
    let month = month_prime + if month_prime < 10 { 3 } else { -9 };
    year += i64::from(month <= 2);
    (year, month, day)
}

fn invalid(detail: &'static str) -> CredentialAuthorityAdmissionErrorV1 {
    refusal(
        CredentialAuthorityAdmissionErrorKindV1::InvalidRequest,
        detail,
    )
}

fn authority_absent() -> CredentialAuthorityAdmissionErrorV1 {
    refusal(
        CredentialAuthorityAdmissionErrorKindV1::AuthorityAbsent,
        "approval class has no exact active committed credential authority",
    )
}

fn stale_authority() -> CredentialAuthorityAdmissionErrorV1 {
    refusal(
        CredentialAuthorityAdmissionErrorKindV1::StaleAuthority,
        "committed credential authority changed during admission",
    )
}

fn durability_violation() -> CredentialAuthorityAdmissionErrorV1 {
    refusal(
        CredentialAuthorityAdmissionErrorKindV1::DurabilityViolation,
        "committed credential authority could not be observed safely",
    )
}

fn authentication_refused() -> CredentialAuthorityAdmissionErrorV1 {
    refusal(
        CredentialAuthorityAdmissionErrorKindV1::AuthenticationRefused,
        "credential authentication was refused",
    )
}

const fn refusal(
    kind: CredentialAuthorityAdmissionErrorKindV1,
    detail: &'static str,
) -> CredentialAuthorityAdmissionErrorV1 {
    CredentialAuthorityAdmissionErrorV1 { kind, detail }
}
