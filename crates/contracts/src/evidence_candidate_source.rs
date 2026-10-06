use crate::{ContentAddressedBindingV1, EvidenceCandidateV1, FingerprintV1, ResolutionEnvelopeV1};
use handbook_engine::local_authority::local_check::{LocalCheckActionV1, LocalCheckAdmissionV1};
use handbook_engine::{
    EvidenceSourceAdmissionErrorKindV1, EvidenceSourceAdmissionErrorV1,
    EvidenceSourceAdmissionRequestV1, EvidenceSourceAdmissionServiceV1, EvidenceSourceAdmissionV1,
    NativeAuthenticatorPortV1, RepositoryExactFilesCheckpointV1, RepositoryReadV1,
    RetainedRepositoryRootV1,
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
    admission: EvidenceCandidateAuthorityV1<'authority>,
    source_checkpoint: Option<&'authority RepositoryExactFilesCheckpointV1<'authority>>,
}

/// Internal consuming authority remains explicitly selected. Local advisory
/// provenance never populates credential registry fields.
pub(crate) enum EvidenceCandidateAuthorityV1<'authority> {
    Credential(EvidenceSourceAdmissionV1<'authority>),
    Local {
        admission: &'authority LocalCheckAdmissionV1<'authority>,
        request: EvidenceSourceAdmissionRequestV1,
        definition: crate::ExactBindingV1,
        closure: FingerprintV1,
        applicability: crate::LocalContractApplicabilityV2,
        intent: crate::LocalContractUseIntentV2,
    },
}

impl EvidenceCandidateAuthorityV1<'_> {
    pub(crate) fn is_local(&self) -> bool {
        matches!(self, Self::Local { .. })
    }
    fn request(&self) -> Option<&EvidenceSourceAdmissionRequestV1> {
        match self {
            Self::Local { request, .. } => Some(request),
            Self::Credential(_) => None,
        }
    }
    pub(crate) fn candidate_ref(&self) -> &str {
        match self {
            Self::Credential(a) => a.candidate_ref(),
            _ => self.request().unwrap().candidate_ref(),
        }
    }
    pub(crate) fn candidate_fingerprint(&self) -> &str {
        match self {
            Self::Credential(a) => a.candidate_fingerprint(),
            _ => self.request().unwrap().candidate_fingerprint(),
        }
    }
    pub(crate) fn producer_ref(&self) -> &str {
        match self {
            Self::Credential(a) => a.producer_ref(),
            _ => self.request().unwrap().producer_ref(),
        }
    }
    pub(crate) fn producer_fingerprint(&self) -> &str {
        match self {
            Self::Credential(a) => a.producer_fingerprint(),
            _ => self.request().unwrap().producer_fingerprint(),
        }
    }
    pub(crate) fn request_ref(&self) -> &str {
        match self {
            Self::Credential(a) => a.request_ref(),
            _ => self.request().unwrap().request_ref(),
        }
    }
    pub(crate) fn request_fingerprint(&self) -> &str {
        match self {
            Self::Credential(a) => a.request_fingerprint(),
            _ => self.request().unwrap().request_fingerprint(),
        }
    }
    pub(crate) fn execution_ref(&self) -> &str {
        match self {
            Self::Credential(a) => a.execution_ref(),
            _ => self.request().unwrap().execution_ref(),
        }
    }
    pub(crate) fn execution_fingerprint(&self) -> &str {
        match self {
            Self::Credential(a) => a.execution_fingerprint(),
            _ => self.request().unwrap().execution_fingerprint(),
        }
    }
    pub(crate) fn semantic_time_utc(&self) -> &str {
        match self {
            Self::Credential(a) => a.semantic_time_utc(),
            Self::Local { admission, .. } => admission.observed_at_utc(),
        }
    }
    pub(crate) fn require_repository_root(
        &self,
        root: &RetainedRepositoryRootV1,
    ) -> Result<(), EvidenceSourceAdmissionErrorV1> {
        match self {
            Self::Credential(a) => a.require_repository_root(root),
            Self::Local { admission, .. } => admission
                .require_repository_root(root)
                .map_err(|_| local_authority_error()),
        }
    }
}

fn local_authority_error() -> EvidenceSourceAdmissionErrorV1 {
    EvidenceSourceAdmissionErrorV1::repository_observation_refusal_v1(
        EvidenceSourceAdmissionErrorKindV1::StaleAuthority,
    )
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
    pub(crate) fn admission(&self) -> &EvidenceCandidateAuthorityV1<'_> {
        &self.admission
    }
    /// Revalidate the live native local authority and immutable source files at
    /// the consuming boundary. Credential admission keeps its existing guard.
    pub fn validate_current(&self) -> Result<(), EvidenceCandidateSourceErrorV1> {
        if let EvidenceCandidateAuthorityV1::Local { admission, .. } = &self.admission {
            admission
                .validate_current()
                .map_err(|_| map_admission_error(local_authority_error()))?;
        }
        if let Some(checkpoint) = self.source_checkpoint {
            checkpoint
                .validate()
                .map_err(|_| unavailable("retained input source generation changed"))?;
        }
        Ok(())
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
                admission: EvidenceCandidateAuthorityV1::Credential(source_admission),
                source_checkpoint: None,
            }))
        })
        .map_err(map_admission_error)?
}

/// Normalize an actual native candidate under lifetime-sealed local authority.
/// The exact Contracts use outcome is owner-derived afresh by the source; its
/// closure and candidate binding must be the content admitted by Engine.
pub fn resolve_repository_local_candidate_observation_v2<'authority, R>(
    repository_root: &RetainedRepositoryRootV1,
    owner: &crate::ContractOwnerRepositoryV1,
    admission: &'authority LocalCheckAdmissionV1<'authority>,
    local_use: &crate::LocalContractUseV2,
    checkpoint: &'authority RepositoryExactFilesCheckpointV1<'authority>,
    candidate: &ContentAddressedBindingV1,
    input: ResolvedEvidenceCandidateObservationInputV1,
    consume: impl FnOnce(ResolvedEvidenceCandidateObservationV1<'authority>) -> R,
) -> Result<R, EvidenceCandidateSourceErrorV1> {
    admission
        .require_repository_root(repository_root)
        .map_err(|_| map_admission_error(local_authority_error()))?;
    admission
        .require_repository_root(owner.retained_repository_root())
        .map_err(|_| map_admission_error(local_authority_error()))?;
    if checkpoint.pinned_file_paths().any(|relative| {
        !admission
            .request()
            .input_paths
            .iter()
            .any(|declared| declared == relative)
    }) {
        return Err(refusal(
            EvidenceCandidateSourceErrorKindV1::AuthorityAmbiguous,
            "local source admission omits a pinned source input path",
        ));
    }
    if !admission
        .request()
        .owner_write_paths
        .iter()
        .any(|relative| relative == ".handbook/state/contracts/v1/evidence-v1")
    {
        return Err(refusal(
            EvidenceCandidateSourceErrorKindV1::AuthorityAmbiguous,
            "local source admission omits the Contracts evidence write path",
        ));
    }
    checkpoint
        .require_file_membership(
            repository_root,
            &local_use
                .required_state_paths()
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>(),
        )
        .map_err(|_| map_admission_error(local_authority_error()))?;
    owner
        .validate_local_contract_use_current_v2(local_use)
        .map_err(|_| map_admission_error(local_authority_error()))?;
    if admission.request().action != LocalCheckActionV1::EvidenceSource
        || admission.request().subject_ref != candidate.reference()
        || admission.request().content_fingerprint != candidate.fingerprint().as_str()
        || admission.request().closure_fingerprint
            != local_use
                .applicability()
                .policy_admission_closure_fingerprint
                .as_str()
        || admission.current_basis() != local_use.current_basis()
    {
        return Err(refusal(
            EvidenceCandidateSourceErrorKindV1::AuthorityAmbiguous,
            "local source action or exact owner applicability is substituted",
        ));
    }
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
    let parsed: EvidenceCandidateV1 = serde_json::from_slice(&candidate_jcs)
        .map_err(|_| malformed("candidate bytes do not encode an exact candidate"))?;
    if parsed.candidate_ref() != candidate.reference()
        || parsed.candidate_fingerprint() != candidate.fingerprint()
    {
        return Err(refusal(
            EvidenceCandidateSourceErrorKindV1::CandidateSubstituted,
            "candidate bytes and requested binding disagree",
        ));
    }
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
    let body = parsed.admission_body();
    let run: crate::ContractEvaluationRunV1 = serde_json::from_slice(&evaluation_run_jcs)
        .map_err(|_| malformed("local evaluation-run bytes are malformed"))?;
    let definition = local_use.definition();
    let dependencies = definition.dependencies();
    if !run.is_local_source_seam_v2()
        || run.contract() != definition.binding()
        || body.contract() != &definition.binding()
        || run.claims() != definition.semantic_definition().claims()
        || definition
            .semantic_definition()
            .evaluation_closure()
            .matcher_bindings
            .as_slice()
            != std::slice::from_ref(run.matcher())
        || run.freshness_policies()
            != dependencies
                .freshness_policies
                .iter()
                .map(crate::EvidenceFreshnessPolicyV1::exact_binding)
                .collect::<Vec<_>>()
        || run.stability_policies() != [dependencies.stability_policy.clone()]
        || run.gate_policy() != &dependencies.gate_policy.exact_binding()
        || run.local_closeout_policy() != dependencies.gate_policy.local_closeout_policy()
        || run.owner_currentness().lifecycle_head_fingerprint()
            != local_use.lifecycle_head_fingerprint()
        || body.evaluation_run().reference() != run.evaluation_run_ref()
        || body.evaluation_run().fingerprint() != run.evaluation_run_fingerprint()
        || body.evaluation_lifecycle_basis() != run.evaluation_lifecycle_basis()
        || body.producer().reference()
            != definition
                .dependencies()
                .validator
                .manifest
                .reference()
                .as_str()
        || body.producer().fingerprint()
            != definition.dependencies().validator.manifest.fingerprint()
    {
        return Err(refusal(
            EvidenceCandidateSourceErrorKindV1::CandidateSubstituted,
            "local candidate is outside the exact reviewed current definition/use closure",
        ));
    }
    if body.producer() != &input.producer
        || body.request_record() != &input.request_record
        || body.execution_record() != &input.execution_record
    {
        return Err(refusal(
            EvidenceCandidateSourceErrorKindV1::CandidateSubstituted,
            "local candidate provenance is substituted",
        ));
    }
    for resolution in [
        run.requested_resolution(),
        body.request_resolution(),
        body.actual_resolution(),
    ]
    .into_iter()
    .chain(input.producer_resolution_ceiling.as_ref())
    {
        let dimensions =
            crate::evidence_admission::engine_resolution_values(resolution.dimension_values())
                .map_err(|_| {
                    refusal(
                        EvidenceCandidateSourceErrorKindV1::CandidateSubstituted,
                        "local source Resolution namespace is substituted",
                    )
                })?;
        admission
            .require_resolution_ceiling(
                resolution.resolution_stack().reference().as_str(),
                resolution.resolution_stack().fingerprint().as_str(),
                dimensions.each_ref().map(String::as_str),
            )
            .map_err(|_| {
                refusal(
                    EvidenceCandidateSourceErrorKindV1::StaleAuthority,
                    "actual local source Resolution exceeds selected terms",
                )
            })?;
    }
    crate::native_dock_source::require_completed_native_dock_source(
        repository_root,
        checkpoint,
        &crate::NativeDockCandidateObservationV1 {
            schema_ref: "handbook.dock.internal-candidate-observation@1.0.0".into(),
            candidate: candidate.clone(),
            candidate_jcs: String::from_utf8(candidate_jcs.clone()).expect("validated UTF-8"),
            evaluation_run_jcs: String::from_utf8(evaluation_run_jcs.clone())
                .expect("validated UTF-8"),
            producer: input.producer.clone(),
            request_record: input.request_record.clone(),
            execution_record: input.execution_record.clone(),
            producer_resolution_ceiling: input.producer_resolution_ceiling.clone(),
        },
        &parsed,
        &run,
        dependencies,
    )?;
    let resolved = ResolvedEvidenceCandidateObservationV1 {
        candidate_jcs,
        evaluation_run_jcs,
        producer: input.producer,
        request_record: input.request_record,
        execution_record: input.execution_record,
        producer_resolution_ceiling: input.producer_resolution_ceiling,
        admission: EvidenceCandidateAuthorityV1::Local {
            admission,
            request,
            definition: local_use.definition().binding(),
            closure: local_use.closure_fingerprint().clone(),
            applicability: local_use.applicability().clone(),
            intent: local_use.intent(),
        },
        source_checkpoint: Some(checkpoint),
    };
    resolved.validate_current()?;
    let result = consume(resolved);
    checkpoint
        .validate()
        .map_err(|_| unavailable("retained input source generation changed"))?;
    admission
        .validate_current()
        .map_err(|_| map_admission_error(local_authority_error()))?;
    Ok(result)
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
