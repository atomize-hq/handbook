use crate::{ContentAddressedBindingV1, EvidenceCandidateV1, FingerprintV1, ResolutionEnvelopeV1};
use handbook_engine::{
    EvidenceSourceAdmissionErrorKindV1, EvidenceSourceAdmissionRequestV1,
    EvidenceSourceAdmissionServiceV1, EvidenceSourceAdmissionV1, NativeAuthenticatorPortV1,
    RepositoryReadV1, RetainedRepositoryRootV1,
};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use std::path::Path;

const CANDIDATE_OBSERVATION_RELATIVE_ROOT: &str =
    ".handbook/state/contracts/v1/candidate-observations";
const OBSERVATION_SCHEMA_REF: &str =
    "handbook.contracts.internal-evidence-candidate-observation@1.0.0";
const MAX_OBSERVATION_BYTES: usize = 1024 * 1024;
const MAX_CANDIDATE_BYTES: usize = 512 * 1024;
const MAX_RUN_BYTES: usize = 512 * 1024;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceCandidateSourceErrorKindV1 {
    InvalidRequest,
    Missing,
    Malformed,
    OverLimit,
    CandidateSubstituted,
    AuthorityAbsent,
    AuthorityAmbiguous,
    StaleAuthority,
    AuthenticationRefused,
    Unavailable,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceCandidateSourceErrorV1 {
    kind: EvidenceCandidateSourceErrorKindV1,
    detail: &'static str,
}

impl EvidenceCandidateSourceErrorV1 {
    pub fn kind(&self) -> EvidenceCandidateSourceErrorKindV1 {
        self.kind
    }
    pub fn detail(&self) -> &str {
        self.detail
    }

    /// Constructs a bounded repository-adapter refusal without exposing raw
    /// paths, credentials, or source-specific diagnostics.
    pub fn repository_refusal(kind: EvidenceCandidateSourceErrorKindV1) -> Self {
        Self {
            kind,
            detail: "repository evidence candidate source refused",
        }
    }

    #[cfg(feature = "test-support")]
    #[doc(hidden)]
    pub fn for_testing(kind: EvidenceCandidateSourceErrorKindV1) -> Self {
        Self {
            kind,
            detail: "injected candidate-source failure",
        }
    }
}

impl fmt::Display for EvidenceCandidateSourceErrorV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.detail)
    }
}

impl Error for EvidenceCandidateSourceErrorV1 {}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct CommittedCandidateObservationV1 {
    schema_ref: String,
    candidate: ContentAddressedBindingV1,
    candidate_jcs: String,
    evaluation_run_jcs: String,
    producer: ContentAddressedBindingV1,
    request_record: ContentAddressedBindingV1,
    execution_record: ContentAddressedBindingV1,
    producer_resolution_ceiling: Option<ResolutionEnvelopeV1>,
}

pub struct ResolvedEvidenceCandidateObservationV1<'authority> {
    candidate_jcs: Vec<u8>,
    evaluation_run_jcs: Vec<u8>,
    producer: ContentAddressedBindingV1,
    request_record: ContentAddressedBindingV1,
    execution_record: ContentAddressedBindingV1,
    producer_resolution_ceiling: Option<ResolutionEnvelopeV1>,
    admission: EvidenceSourceAdmissionV1<'authority>,
}

#[derive(Clone, Debug)]
pub struct ResolvedEvidenceCandidateObservationInputV1 {
    pub candidate_jcs: Vec<u8>,
    pub evaluation_run_jcs: Vec<u8>,
    pub producer: ContentAddressedBindingV1,
    pub request_record: ContentAddressedBindingV1,
    pub execution_record: ContentAddressedBindingV1,
    pub producer_resolution_ceiling: Option<ResolutionEnvelopeV1>,
}

impl ResolvedEvidenceCandidateObservationV1<'_> {
    pub fn candidate_jcs(&self) -> &[u8] {
        &self.candidate_jcs
    }
    pub fn evaluation_run_jcs(&self) -> &[u8] {
        &self.evaluation_run_jcs
    }
    pub fn producer(&self) -> &ContentAddressedBindingV1 {
        &self.producer
    }
    pub fn request_record(&self) -> &ContentAddressedBindingV1 {
        &self.request_record
    }
    pub fn execution_record(&self) -> &ContentAddressedBindingV1 {
        &self.execution_record
    }
    pub fn producer_resolution_ceiling(&self) -> Option<&ResolutionEnvelopeV1> {
        self.producer_resolution_ceiling.as_ref()
    }
    pub(crate) fn admission(&self) -> &EvidenceSourceAdmissionV1<'_> {
        &self.admission
    }
}

/// Seals a repository-owned observation behind the existing candidate-source
/// admission. The retained root must equal the root authenticated by Engine;
/// candidate identity and canonical bytes are recomputed before admission.
pub fn resolve_repository_candidate_observation_v1<P, R>(
    repository_root: &RetainedRepositoryRootV1,
    admission: &mut EvidenceSourceAdmissionServiceV1<P>,
    candidate: &ContentAddressedBindingV1,
    input: ResolvedEvidenceCandidateObservationInputV1,
    consume: impl for<'authority> FnOnce(ResolvedEvidenceCandidateObservationV1<'authority>) -> R,
) -> Result<R, EvidenceCandidateSourceErrorV1>
where
    P: NativeAuthenticatorPortV1,
{
    let candidate_jcs = validate_embedded_jcs(
        String::from_utf8(input.candidate_jcs)
            .map_err(|_| malformed("candidate bytes are not UTF-8"))?,
        MAX_CANDIDATE_BYTES,
        "candidate bytes are malformed or over limit",
    )?;
    let evaluation_run_jcs = validate_embedded_jcs(
        String::from_utf8(input.evaluation_run_jcs)
            .map_err(|_| malformed("evaluation-run bytes are not UTF-8"))?,
        MAX_RUN_BYTES,
        "evaluation-run bytes are malformed or over limit",
    )?;
    let request = EvidenceSourceAdmissionRequestV1::new(
        candidate.reference(),
        candidate.fingerprint().as_str(),
        input.producer.reference(),
        input.producer.fingerprint().as_str(),
        input.request_record.reference(),
        input.request_record.fingerprint().as_str(),
        input.execution_record.reference(),
        input.execution_record.fingerprint().as_str(),
    )
    .map_err(map_admission_error)?;
    admission
        .admit(request, |source_admission| {
            source_admission
                .require_repository_root(repository_root)
                .map_err(map_admission_error)?;
            let parsed_candidate: EvidenceCandidateV1 = serde_json::from_slice(&candidate_jcs)
                .map_err(|_| malformed("candidate bytes do not encode an exact candidate"))?;
            if parsed_candidate.candidate_ref() != candidate.reference()
                || parsed_candidate.candidate_fingerprint() != candidate.fingerprint()
            {
                return Err(refusal(
                    EvidenceCandidateSourceErrorKindV1::CandidateSubstituted,
                    "candidate bytes and requested binding disagree",
                ));
            }
            Ok(consume(ResolvedEvidenceCandidateObservationV1 {
                candidate_jcs,
                evaluation_run_jcs,
                producer: input.producer,
                request_record: input.request_record,
                execution_record: input.execution_record,
                producer_resolution_ceiling: input.producer_resolution_ceiling,
                admission: source_admission,
            }))
        })
        .map_err(map_admission_error)?
}

pub trait EvidenceCandidateSourceV1 {
    fn resolve_exact<R>(
        &mut self,
        candidate: &ContentAddressedBindingV1,
        consume: impl for<'authority> FnOnce(ResolvedEvidenceCandidateObservationV1<'authority>) -> R,
    ) -> Result<R, EvidenceCandidateSourceErrorV1>;
}

pub struct RepositoryEvidenceCandidateSourceV1<P> {
    repository_root: RetainedRepositoryRootV1,
    admission: EvidenceSourceAdmissionServiceV1<P>,
}

impl<P: NativeAuthenticatorPortV1> RepositoryEvidenceCandidateSourceV1<P> {
    pub fn open(
        repo_root: impl AsRef<Path>,
        port: P,
    ) -> Result<Self, EvidenceCandidateSourceErrorV1> {
        let root = repo_root.as_ref();
        let repository_root = RetainedRepositoryRootV1::open_exact(root)
            .map_err(|_| unavailable("candidate repository root is unavailable"))?;
        Ok(Self {
            repository_root,
            admission: EvidenceSourceAdmissionServiceV1::new(root, port),
        })
    }

    pub fn into_port(self) -> P {
        self.admission.into_port()
    }
}

impl<P: NativeAuthenticatorPortV1> EvidenceCandidateSourceV1
    for RepositoryEvidenceCandidateSourceV1<P>
{
    fn resolve_exact<R>(
        &mut self,
        candidate: &ContentAddressedBindingV1,
        consume: impl for<'authority> FnOnce(ResolvedEvidenceCandidateObservationV1<'authority>) -> R,
    ) -> Result<R, EvidenceCandidateSourceErrorV1> {
        let expected_suffix = candidate
            .fingerprint()
            .as_str()
            .strip_prefix("sha256:")
            .expect("normalized fingerprint");
        if !candidate.reference().ends_with(expected_suffix) {
            return Err(refusal(
                EvidenceCandidateSourceErrorKindV1::CandidateSubstituted,
                "candidate reference and fingerprint are incoherent",
            ));
        }
        let relative = observation_relative_path(candidate.fingerprint())?;
        let bytes = match self
            .repository_root
            .read_regular_bounded(&relative, MAX_OBSERVATION_BYTES)
            .map_err(|error| match error.kind() {
                handbook_engine::RepositoryDurabilityErrorKindV1::BoundExceeded => refusal(
                    EvidenceCandidateSourceErrorKindV1::OverLimit,
                    "candidate observation exceeds its byte bound",
                ),
                handbook_engine::RepositoryDurabilityErrorKindV1::NotFound => refusal(
                    EvidenceCandidateSourceErrorKindV1::Missing,
                    "exact committed candidate observation is absent",
                ),
                _ => unavailable("candidate observation repository is unavailable"),
            })? {
            RepositoryReadV1::Found(bytes) => bytes,
            RepositoryReadV1::Absent => {
                return Err(refusal(
                    EvidenceCandidateSourceErrorKindV1::Missing,
                    "exact committed candidate observation is absent",
                ))
            }
        };
        let value: serde_json::Value = serde_json::from_slice(&bytes)
            .map_err(|_| malformed("candidate observation is not strict JSON"))?;
        let canonical = serde_json_canonicalizer::to_vec(&value)
            .map_err(|_| malformed("candidate observation cannot be canonicalized"))?;
        if canonical != bytes {
            return Err(malformed("candidate observation is not canonical JSON"));
        }
        let record: CommittedCandidateObservationV1 = serde_json::from_value(value)
            .map_err(|_| malformed("candidate observation shape is not closed"))?;
        if record.schema_ref != OBSERVATION_SCHEMA_REF || &record.candidate != candidate {
            return Err(refusal(
                EvidenceCandidateSourceErrorKindV1::CandidateSubstituted,
                "candidate observation binding is substituted",
            ));
        }
        let candidate_jcs = validate_embedded_jcs(
            record.candidate_jcs,
            MAX_CANDIDATE_BYTES,
            "candidate bytes are malformed or over limit",
        )?;
        let evaluation_run_jcs = validate_embedded_jcs(
            record.evaluation_run_jcs,
            MAX_RUN_BYTES,
            "evaluation-run bytes are malformed or over limit",
        )?;
        resolve_repository_candidate_observation_v1(
            &self.repository_root,
            &mut self.admission,
            candidate,
            ResolvedEvidenceCandidateObservationInputV1 {
                candidate_jcs,
                evaluation_run_jcs,
                producer: record.producer,
                request_record: record.request_record,
                execution_record: record.execution_record,
                producer_resolution_ceiling: record.producer_resolution_ceiling,
            },
            consume,
        )
    }
}

fn observation_relative_path(
    fingerprint: &FingerprintV1,
) -> Result<String, EvidenceCandidateSourceErrorV1> {
    let hex = fingerprint
        .as_str()
        .strip_prefix("sha256:")
        .ok_or_else(|| malformed("candidate fingerprint is not normalized"))?;
    Ok(format!("{CANDIDATE_OBSERVATION_RELATIVE_ROOT}/{hex}.json"))
}

fn validate_embedded_jcs(
    value: String,
    maximum: usize,
    detail: &'static str,
) -> Result<Vec<u8>, EvidenceCandidateSourceErrorV1> {
    let bytes = value.into_bytes();
    if bytes.len() > maximum {
        return Err(refusal(
            EvidenceCandidateSourceErrorKindV1::OverLimit,
            detail,
        ));
    }
    if bytes.is_empty() || std::str::from_utf8(&bytes).is_err() {
        return Err(malformed(detail));
    }
    let parsed: serde_json::Value =
        serde_json::from_slice(&bytes).map_err(|_| malformed(detail))?;
    if serde_json_canonicalizer::to_vec(&parsed).map_err(|_| malformed(detail))? != bytes {
        return Err(malformed(detail));
    }
    Ok(bytes)
}

fn map_admission_error(
    error: handbook_engine::EvidenceSourceAdmissionErrorV1,
) -> EvidenceCandidateSourceErrorV1 {
    let kind = match error.kind() {
        EvidenceSourceAdmissionErrorKindV1::InvalidRequest => {
            EvidenceCandidateSourceErrorKindV1::InvalidRequest
        }
        EvidenceSourceAdmissionErrorKindV1::AuthorityAbsent => {
            EvidenceCandidateSourceErrorKindV1::AuthorityAbsent
        }
        EvidenceSourceAdmissionErrorKindV1::AuthorityAmbiguous => {
            EvidenceCandidateSourceErrorKindV1::AuthorityAmbiguous
        }
        EvidenceSourceAdmissionErrorKindV1::StaleAuthority => {
            EvidenceCandidateSourceErrorKindV1::StaleAuthority
        }
        EvidenceSourceAdmissionErrorKindV1::AuthenticationRefused => {
            EvidenceCandidateSourceErrorKindV1::AuthenticationRefused
        }
        EvidenceSourceAdmissionErrorKindV1::AuthenticatorUnavailable
        | EvidenceSourceAdmissionErrorKindV1::DurabilityViolation
        | EvidenceSourceAdmissionErrorKindV1::ResolutionRefused => {
            EvidenceCandidateSourceErrorKindV1::Unavailable
        }
    };
    refusal(kind, "sealed evidence candidate source admission refused")
}

fn malformed(detail: &'static str) -> EvidenceCandidateSourceErrorV1 {
    refusal(EvidenceCandidateSourceErrorKindV1::Malformed, detail)
}

fn unavailable(detail: &'static str) -> EvidenceCandidateSourceErrorV1 {
    refusal(EvidenceCandidateSourceErrorKindV1::Unavailable, detail)
}

const fn refusal(
    kind: EvidenceCandidateSourceErrorKindV1,
    detail: &'static str,
) -> EvidenceCandidateSourceErrorV1 {
    EvidenceCandidateSourceErrorV1 { kind, detail }
}

#[cfg(feature = "test-support")]
#[doc(hidden)]
#[derive(Clone, Debug)]
pub struct CandidateObservationFixtureV1(CommittedCandidateObservationV1);

#[cfg(feature = "test-support")]
impl CandidateObservationFixtureV1 {
    pub fn new(
        candidate: ContentAddressedBindingV1,
        candidate_jcs: Vec<u8>,
        evaluation_run_jcs: Vec<u8>,
        producer: ContentAddressedBindingV1,
        request_record: ContentAddressedBindingV1,
        execution_record: ContentAddressedBindingV1,
        producer_resolution_ceiling: Option<ResolutionEnvelopeV1>,
    ) -> Result<Self, EvidenceCandidateSourceErrorV1> {
        let candidate_jcs = String::from_utf8(validate_embedded_jcs(
            String::from_utf8(candidate_jcs)
                .map_err(|_| malformed("candidate fixture is not UTF-8"))?,
            MAX_CANDIDATE_BYTES,
            "candidate fixture is malformed",
        )?)
        .expect("validated UTF-8");
        let evaluation_run_jcs = String::from_utf8(validate_embedded_jcs(
            String::from_utf8(evaluation_run_jcs)
                .map_err(|_| malformed("evaluation-run fixture is not UTF-8"))?,
            MAX_RUN_BYTES,
            "evaluation-run fixture is malformed",
        )?)
        .expect("validated UTF-8");
        Ok(Self(CommittedCandidateObservationV1 {
            schema_ref: OBSERVATION_SCHEMA_REF.to_owned(),
            candidate,
            candidate_jcs,
            evaluation_run_jcs,
            producer,
            request_record,
            execution_record,
            producer_resolution_ceiling,
        }))
    }
}

#[cfg(feature = "test-support")]
#[doc(hidden)]
pub fn seed_committed_candidate_observation_for_testing_v1(
    repo_root: impl AsRef<Path>,
    fixture: &CandidateObservationFixtureV1,
) -> Result<(), EvidenceCandidateSourceErrorV1> {
    let relative = observation_relative_path(fixture.0.candidate.fingerprint())?;
    let path = repo_root.as_ref().join(relative);
    std::fs::create_dir_all(path.parent().expect("observation path has a parent"))
        .map_err(|_| unavailable("candidate fixture directory could not be created"))?;
    let bytes = serde_json_canonicalizer::to_vec(&fixture.0)
        .map_err(|_| malformed("candidate fixture could not be canonicalized"))?;
    std::fs::write(path, bytes).map_err(|_| unavailable("candidate fixture could not be committed"))
}
