use super::serialization::classify_contract_error_message;
use super::{
    BoundedVec, ContractError, Fingerprint, Problem, ProblemBinding, ProblemCategory, UtcTimestamp,
};
use serde::de::Error as _;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::cmp::Ordering;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TerminalStatus {
    Ok,
    Blocked,
    Refused,
    Error,
}

impl<'de> Deserialize<'de> for TerminalStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)
            .map_err(|_| D::Error::custom(ContractError::InvalidJson))?;
        match value.as_str() {
            "ok" => Ok(Self::Ok),
            "blocked" => Ok(Self::Blocked),
            "refused" => Ok(Self::Refused),
            "error" => Ok(Self::Error),
            _ => Err(D::Error::custom(ContractError::InvalidJson)),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct TerminalFields<T> {
    status: TerminalStatus,
    data: Option<T>,
    blockers: BoundedVec<Problem, 128>,
    refusals: BoundedVec<Problem, 128>,
    errors: BoundedVec<Problem, 128>,
}

impl<T> TerminalFields<T> {
    pub fn new(
        status: TerminalStatus,
        data: Option<T>,
        blockers: BoundedVec<Problem, 128>,
        refusals: BoundedVec<Problem, 128>,
        errors: BoundedVec<Problem, 128>,
    ) -> Result<Self, ContractError> {
        validate_terminal_shape(
            status,
            data.is_some(),
            blockers.as_slice(),
            refusals.as_slice(),
            errors.as_slice(),
        )?;
        Ok(Self {
            status,
            data,
            blockers,
            refusals,
            errors,
        })
    }

    pub fn status(&self) -> TerminalStatus {
        self.status
    }

    pub fn data(&self) -> Option<&T> {
        self.data.as_ref()
    }

    pub fn blockers(&self) -> &[Problem] {
        self.blockers.as_slice()
    }

    pub fn refusals(&self) -> &[Problem] {
        self.refusals.as_slice()
    }

    pub fn errors(&self) -> &[Problem] {
        self.errors.as_slice()
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for TerminalFields<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields, bound(deserialize = "T: Deserialize<'de>"))]
        struct RawTerminalFields<T> {
            status: TerminalStatus,
            #[serde(default)]
            data: RequiredOption<T>,
            blockers: BoundedVec<Problem, 128>,
            refusals: BoundedVec<Problem, 128>,
            errors: BoundedVec<Problem, 128>,
        }

        let raw = RawTerminalFields::<T>::deserialize(deserializer).map_err(|error| {
            D::Error::custom(classify_contract_error_message(&error.to_string()))
        })?;
        let data = raw.data.into_option().map_err(D::Error::custom)?;
        Self::new(raw.status, data, raw.blockers, raw.refusals, raw.errors)
            .map_err(D::Error::custom)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum IdempotencyStage {
    CapabilityValidation,
    RequestValidation,
    ResolutionValidation,
    AuthorityValidation,
    PreconditionValidation,
    SafetyValidation,
    IdempotencyValidation,
    ExecutionStart,
}

impl<'de> Deserialize<'de> for IdempotencyStage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)
            .map_err(|_| D::Error::custom(ContractError::InvalidJson))?;
        match value.as_str() {
            "capability_validation" => Ok(Self::CapabilityValidation),
            "request_validation" => Ok(Self::RequestValidation),
            "resolution_validation" => Ok(Self::ResolutionValidation),
            "authority_validation" => Ok(Self::AuthorityValidation),
            "precondition_validation" => Ok(Self::PreconditionValidation),
            "safety_validation" => Ok(Self::SafetyValidation),
            "idempotency_validation" => Ok(Self::IdempotencyValidation),
            "execution_start" => Ok(Self::ExecutionStart),
            _ => Err(D::Error::custom(ContractError::InvalidJson)),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IdempotencyState {
    kind: IdempotencyStateKind,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum IdempotencyStateKind {
    NotApplicable,
    NotEstablished {
        stage: IdempotencyStage,
        terminal_problem: ProblemBinding,
    },
    Established {
        idempotency_key_fingerprint: Fingerprint,
        request_fingerprint: Fingerprint,
        replayed: bool,
        original_result_fingerprint: Option<Fingerprint>,
        result_retention_until_utc: UtcTimestamp,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IdempotencyStateRef<'a> {
    NotApplicable,
    NotEstablished {
        stage: IdempotencyStage,
        terminal_problem: &'a ProblemBinding,
    },
    Established {
        idempotency_key_fingerprint: &'a Fingerprint,
        request_fingerprint: &'a Fingerprint,
        replayed: bool,
        original_result_fingerprint: Option<&'a Fingerprint>,
        result_retention_until_utc: &'a UtcTimestamp,
    },
}

impl IdempotencyState {
    pub fn not_applicable() -> Result<Self, ContractError> {
        Ok(Self {
            kind: IdempotencyStateKind::NotApplicable,
        })
    }

    pub fn not_established(
        stage: IdempotencyStage,
        terminal_problem: ProblemBinding,
    ) -> Result<Self, ContractError> {
        Ok(Self {
            kind: IdempotencyStateKind::NotEstablished {
                stage,
                terminal_problem,
            },
        })
    }

    pub fn established(
        idempotency_key_fingerprint: Fingerprint,
        request_fingerprint: Fingerprint,
        replayed: bool,
        original_result_fingerprint: Option<Fingerprint>,
        result_retention_until_utc: UtcTimestamp,
    ) -> Result<Self, ContractError> {
        if replayed != original_result_fingerprint.is_some() {
            return Err(ContractError::InvalidInvariant);
        }
        Ok(Self {
            kind: IdempotencyStateKind::Established {
                idempotency_key_fingerprint,
                request_fingerprint,
                replayed,
                original_result_fingerprint,
                result_retention_until_utc,
            },
        })
    }

    pub fn as_ref(&self) -> IdempotencyStateRef<'_> {
        match &self.kind {
            IdempotencyStateKind::NotApplicable => IdempotencyStateRef::NotApplicable,
            IdempotencyStateKind::NotEstablished {
                stage,
                terminal_problem,
            } => IdempotencyStateRef::NotEstablished {
                stage: *stage,
                terminal_problem,
            },
            IdempotencyStateKind::Established {
                idempotency_key_fingerprint,
                request_fingerprint,
                replayed,
                original_result_fingerprint,
                result_retention_until_utc,
            } => IdempotencyStateRef::Established {
                idempotency_key_fingerprint,
                request_fingerprint,
                replayed: *replayed,
                original_result_fingerprint: original_result_fingerprint.as_ref(),
                result_retention_until_utc,
            },
        }
    }
}

impl Serialize for IdempotencyState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match &self.kind {
            IdempotencyStateKind::NotApplicable => {
                let mut object = serializer.serialize_struct("IdempotencyState", 1)?;
                object.serialize_field("state", "not_applicable")?;
                object.end()
            }
            IdempotencyStateKind::NotEstablished {
                stage,
                terminal_problem,
            } => {
                let mut object = serializer.serialize_struct("IdempotencyState", 3)?;
                object.serialize_field("state", "not_established")?;
                object.serialize_field("stage", stage)?;
                object.serialize_field("terminal_problem", terminal_problem)?;
                object.end()
            }
            IdempotencyStateKind::Established {
                idempotency_key_fingerprint,
                request_fingerprint,
                replayed,
                original_result_fingerprint,
                result_retention_until_utc,
            } => {
                let mut object = serializer.serialize_struct("IdempotencyState", 6)?;
                object.serialize_field("state", "established")?;
                object
                    .serialize_field("idempotency_key_fingerprint", idempotency_key_fingerprint)?;
                object.serialize_field("request_fingerprint", request_fingerprint)?;
                object.serialize_field("replayed", replayed)?;
                object
                    .serialize_field("original_result_fingerprint", original_result_fingerprint)?;
                object.serialize_field("result_retention_until_utc", result_retention_until_utc)?;
                object.end()
            }
        }
    }
}

impl<'de> Deserialize<'de> for IdempotencyState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(tag = "state", rename_all = "snake_case", deny_unknown_fields)]
        enum RawIdempotencyState {
            NotApplicable {},
            NotEstablished {
                stage: IdempotencyStage,
                terminal_problem: ProblemBinding,
            },
            Established {
                idempotency_key_fingerprint: Fingerprint,
                request_fingerprint: Fingerprint,
                replayed: bool,
                #[serde(default)]
                original_result_fingerprint: RequiredOption<Fingerprint>,
                result_retention_until_utc: UtcTimestamp,
            },
        }

        match RawIdempotencyState::deserialize(deserializer).map_err(|error| {
            D::Error::custom(classify_contract_error_message(&error.to_string()))
        })? {
            RawIdempotencyState::NotApplicable {} => Self::not_applicable(),
            RawIdempotencyState::NotEstablished {
                stage,
                terminal_problem,
            } => Self::not_established(stage, terminal_problem),
            RawIdempotencyState::Established {
                idempotency_key_fingerprint,
                request_fingerprint,
                replayed,
                original_result_fingerprint,
                result_retention_until_utc,
            } => Self::established(
                idempotency_key_fingerprint,
                request_fingerprint,
                replayed,
                original_result_fingerprint
                    .into_option()
                    .map_err(D::Error::custom)?,
                result_retention_until_utc,
            ),
        }
        .map_err(D::Error::custom)
    }
}

fn validate_terminal_shape(
    status: TerminalStatus,
    has_data: bool,
    blockers: &[Problem],
    refusals: &[Problem],
    errors: &[Problem],
) -> Result<(), ContractError> {
    let shape_is_valid = match status {
        TerminalStatus::Ok => {
            has_data && blockers.is_empty() && refusals.is_empty() && errors.is_empty()
        }
        TerminalStatus::Blocked => {
            !has_data && !blockers.is_empty() && refusals.is_empty() && errors.is_empty()
        }
        TerminalStatus::Refused => {
            !has_data && blockers.is_empty() && !refusals.is_empty() && errors.is_empty()
        }
        TerminalStatus::Error => {
            !has_data && blockers.is_empty() && refusals.is_empty() && !errors.is_empty()
        }
    };
    if !shape_is_valid {
        return Err(ContractError::InvalidInvariant);
    }

    validate_problem_collection(blockers, |category| {
        category == ProblemCategory::Prerequisite
    })?;
    validate_problem_collection(refusals, |category| {
        matches!(
            category,
            ProblemCategory::Schema
                | ProblemCategory::Compatibility
                | ProblemCategory::Capability
                | ProblemCategory::Resolution
                | ProblemCategory::Authority
                | ProblemCategory::Precondition
                | ProblemCategory::Safety
                | ProblemCategory::Idempotency
        )
    })?;
    validate_problem_collection(errors, |category| {
        matches!(
            category,
            ProblemCategory::Implementation | ProblemCategory::Adapter
        )
    })
}

fn validate_problem_collection(
    problems: &[Problem],
    category_is_valid: impl Fn(ProblemCategory) -> bool,
) -> Result<(), ContractError> {
    if problems
        .iter()
        .any(|problem| !category_is_valid(problem.category()))
    {
        return Err(ContractError::InvalidInvariant);
    }

    for (index, problem) in problems.iter().enumerate() {
        if problems[..index]
            .iter()
            .any(|prior| prior.problem_id() == problem.problem_id())
        {
            return Err(ContractError::DuplicateMember);
        }
    }

    for pair in problems.windows(2) {
        match problem_sort_key(&pair[0]).cmp(&problem_sort_key(&pair[1])) {
            Ordering::Less => {}
            Ordering::Equal => return Err(ContractError::DuplicateMember),
            Ordering::Greater => return Err(ContractError::NonCanonicalOrder),
        }
    }
    Ok(())
}

fn problem_sort_key(problem: &Problem) -> (&str, &str, &str, &str) {
    (
        problem.code().as_str(),
        problem.subject().reference().as_str(),
        problem.details_schema().reference().as_str(),
        problem.details_fingerprint().as_str(),
    )
}

enum RequiredOption<T> {
    Missing,
    Present(Option<T>),
}

impl<T> RequiredOption<T> {
    fn into_option(self) -> Result<Option<T>, ContractError> {
        match self {
            Self::Missing => Err(ContractError::InvalidJson),
            Self::Present(value) => Ok(value),
        }
    }
}

impl<T> Default for RequiredOption<T> {
    fn default() -> Self {
        Self::Missing
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for RequiredOption<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Option::deserialize(deserializer).map(Self::Present)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contract_membrane::{
        canonical_json_bytes, parse_canonical_json, sha256_fingerprint, BoundedString,
        ContractLimits, ContractValue,
    };

    const SENTINEL: &str = "raw-key-sentinel";

    fn with_unknown_field<T: Serialize>(value: &T) -> String {
        let mut wire = serde_json::to_string(value).unwrap();
        wire.pop();
        wire.push_str(&format!(",\"{SENTINEL}\":null}}"));
        wire
    }

    fn assert_sanitized<T>(wire: &str)
    where
        T: for<'de> Deserialize<'de>,
    {
        let error = match serde_json::from_str::<T>(wire) {
            Ok(_) => panic!("rejected response input was accepted"),
            Err(error) => error.to_string(),
        };
        assert!(error.starts_with(&ContractError::InvalidJson.to_string()));
        assert!(!error.contains(SENTINEL));
    }

    fn replace_once(bytes: &[u8], old: &str, new: &str) -> Vec<u8> {
        let wire = String::from_utf8(bytes.to_vec()).unwrap();
        assert_eq!(wire.matches(old).count(), 1);
        wire.replacen(old, new, 1).into_bytes()
    }

    #[test]
    fn public_response_deserializers_and_closed_enums_redact_rejected_input() {
        assert_sanitized::<TerminalStatus>(&format!(r#""{SENTINEL}""#));
        assert_sanitized::<IdempotencyStage>(&format!(r#""{SENTINEL}""#));

        let terminal = TerminalFields::new(
            TerminalStatus::Ok,
            Some(ContractValue::null()),
            BoundedVec::new(Vec::new()).unwrap(),
            BoundedVec::new(Vec::new()).unwrap(),
            BoundedVec::new(Vec::new()).unwrap(),
        )
        .unwrap();
        assert_sanitized::<TerminalFields<ContractValue>>(&with_unknown_field(&terminal));

        let idempotency = IdempotencyState::not_applicable().unwrap();
        assert_sanitized::<IdempotencyState>(&with_unknown_field(&idempotency));
        assert_sanitized::<IdempotencyState>(&format!(r#"{{"state":"{SENTINEL}"}}"#));
    }

    #[test]
    fn bounded_parser_preserves_nested_response_errors() {
        let limits = ContractLimits::packet0();
        let terminal = TerminalFields::new(
            TerminalStatus::Ok,
            Some(BoundedString::<4>::new("four".to_owned()).unwrap()),
            BoundedVec::new(Vec::new()).unwrap(),
            BoundedVec::new(Vec::new()).unwrap(),
            BoundedVec::new(Vec::new()).unwrap(),
        )
        .unwrap();
        let bound_exceeded = replace_once(
            &canonical_json_bytes(&terminal, &limits).unwrap(),
            "\"four\"",
            "\"five!\"",
        );
        assert_eq!(
            parse_canonical_json::<TerminalFields<BoundedString<4>>>(&bound_exceeded, &limits,),
            Err(ContractError::BoundExceeded)
        );

        let key_fingerprint = sha256_fingerprint(b"idempotency-key");
        let key_fingerprint_wire = key_fingerprint.as_str().to_owned();
        let idempotency = IdempotencyState::established(
            key_fingerprint,
            sha256_fingerprint(b"request"),
            false,
            None,
            UtcTimestamp::parse("2030-01-01T00:00:00Z").unwrap(),
        )
        .unwrap();
        let invalid_fingerprint = replace_once(
            &canonical_json_bytes(&idempotency, &limits).unwrap(),
            &format!("\"{key_fingerprint_wire}\""),
            "\"sha256:bad\"",
        );
        assert_eq!(
            parse_canonical_json::<IdempotencyState>(&invalid_fingerprint, &limits),
            Err(ContractError::InvalidFingerprint)
        );
    }
}
