use crate::canonical::{canonical_bytes, canonical_fingerprint};
use crate::{ApplicabilityOutcomeV1, ContractError, ExactBindingV1, FingerprintV1, StableIdV1};
use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;
use std::time::{SystemTime, UNIX_EPOCH};

pub const EVIDENCE_CANDIDATE_SCHEMA_REF_V1: &str = "handbook.contract-evidence-candidate@1.0.0";
pub const EVIDENCE_RECORD_SCHEMA_REF_V1: &str = "handbook.contract-evidence-record@1.0.0";
pub const CLAIM_VERDICT_SCHEMA_REF_V1: &str = "handbook.contract-claim-verdict@1.0.0";
pub const GATE_RESULT_SCHEMA_REF_V1: &str = "handbook.contract-gate-result@1.0.0";
pub const LOCAL_CLOSEOUT_RESULT_SCHEMA_REF_V1: &str =
    "handbook.contract-local-closeout-result@1.0.0";
pub const EVIDENCE_OPERATION_DETAILS_SCHEMA_REF_V1: &str =
    "handbook.contract-evidence-operation-details@1.0.0";

const MAX_REF_BYTES: usize = 2048;
const MAX_TEXT_BYTES: usize = 4096;
const MAX_REVISION_BYTES: usize = 1024;
const MAX_ITEMS: usize = 128;
const MAX_FACT_BYTES: usize = 256 * 1024;

#[allow(dead_code)] // Packet 1 fixes the owner identity kernel before later sealed minting paths.
trait IdentityBodyValidation {
    fn validate_identity_body(&self) -> Result<(), ContractError>;
}

macro_rules! identity_record {
    ($name:ident, $body:ident, $schema:ident, $ref_field:ident, $fp_field:ident, $derive_ref:expr) => {
        #[derive(Clone, Debug, PartialEq, Serialize)]
        pub struct $name {
            schema_ref: &'static str,
            body: $body,
            $ref_field: String,
            $fp_field: FingerprintV1,
        }

        impl $name {
            #[allow(dead_code)] // Reserved for contracts-owner construction in later admitted packets.
            pub(crate) fn from_body(body: $body) -> Result<Self, ContractError> {
                body.validate_identity_body()?;
                #[derive(Serialize)]
                struct Preimage<'a> {
                    schema_ref: &'static str,
                    body: &'a $body,
                }
                let fingerprint = canonical_fingerprint(&Preimage {
                    schema_ref: $schema,
                    body: &body,
                })?;
                let derive_ref = $derive_ref;
                let reference = derive_ref(&body, &fingerprint);
                Ok(Self {
                    schema_ref: $schema,
                    body,
                    $ref_field: reference,
                    $fp_field: fingerprint,
                })
            }

            pub fn record_ref(&self) -> &str {
                &self.$ref_field
            }
            pub fn record_fingerprint(&self) -> &FingerprintV1 {
                &self.$fp_field
            }
            pub fn canonical_json_bytes(&self) -> Result<Vec<u8>, ContractError> {
                canonical_bytes(self)
            }
        }
    };
}

/// Exact owner-established UTC semantic time, restricted to whole seconds.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SemanticTimeV1(String);

impl SemanticTimeV1 {
    pub fn parse(value: &str) -> Result<Self, ContractError> {
        if !valid_utc_second(value) {
            return Err(ContractError::InvalidInvariant);
        }
        Ok(Self(value.to_owned()))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub(crate) fn owner_now() -> Result<Self, ContractError> {
        let seconds = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| ContractError::InvalidInvariant)?
            .as_secs();
        let days = i64::try_from(seconds / 86_400).map_err(|_| ContractError::BoundExceeded)?;
        let seconds_of_day = seconds % 86_400;
        let (year, month, day) = civil_from_days(days);
        let hour = seconds_of_day / 3_600;
        let minute = (seconds_of_day % 3_600) / 60;
        let second = seconds_of_day % 60;
        Self::parse(&format!(
            "{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}Z"
        ))
    }
}

impl Serialize for SemanticTimeV1 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for SemanticTimeV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::parse(&value).map_err(D::Error::custom)
    }
}

/// A bounded exact ref/fingerprint pair whose identity formula remains owned by
/// the referenced domain.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ContentAddressedBindingV1 {
    #[serde(rename = "ref")]
    reference: String,
    fingerprint: FingerprintV1,
}

impl ContentAddressedBindingV1 {
    pub fn new(reference: &str, fingerprint: FingerprintV1) -> Result<Self, ContractError> {
        validate_ref(reference)?;
        Ok(Self {
            reference: reference.to_owned(),
            fingerprint,
        })
    }
    pub fn reference(&self) -> &str {
        &self.reference
    }
    pub fn fingerprint(&self) -> &FingerprintV1 {
        &self.fingerprint
    }
}

impl<'de> Deserialize<'de> for ContentAddressedBindingV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Raw {
            #[serde(rename = "ref")]
            reference: String,
            fingerprint: FingerprintV1,
        }
        let raw = Raw::deserialize(deserializer)?;
        Self::new(&raw.reference, raw.fingerprint).map_err(D::Error::custom)
    }
}

/// One bounded matcher input value used for owner-side applicability later.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct MatcherInputValueV1 {
    field: StableIdV1,
    value: String,
}

impl MatcherInputValueV1 {
    pub fn new(field: StableIdV1, value: &str) -> Result<Self, ContractError> {
        validate_text(value, MAX_TEXT_BYTES)?;
        Ok(Self {
            field,
            value: value.to_owned(),
        })
    }
    pub fn field(&self) -> &StableIdV1 {
        &self.field
    }
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl<'de> Deserialize<'de> for MatcherInputValueV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Raw {
            field: StableIdV1,
            value: String,
        }
        let raw = Raw::deserialize(deserializer)?;
        Self::new(raw.field, &raw.value).map_err(D::Error::custom)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct EvaluationSubjectV1 {
    subject: ContentAddressedBindingV1,
    matcher_inputs: Vec<MatcherInputValueV1>,
}

impl EvaluationSubjectV1 {
    pub fn new(
        subject: ContentAddressedBindingV1,
        mut matcher_inputs: Vec<MatcherInputValueV1>,
    ) -> Result<Self, ContractError> {
        normalize_matcher_inputs(&mut matcher_inputs)?;
        Ok(Self {
            subject,
            matcher_inputs,
        })
    }
    pub fn subject(&self) -> &ContentAddressedBindingV1 {
        &self.subject
    }
    pub fn matcher_inputs(&self) -> &[MatcherInputValueV1] {
        &self.matcher_inputs
    }
}

impl<'de> Deserialize<'de> for EvaluationSubjectV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Raw {
            subject: ContentAddressedBindingV1,
            matcher_inputs: Vec<MatcherInputValueV1>,
        }
        let raw = Raw::deserialize(deserializer)?;
        Self::new(raw.subject, raw.matcher_inputs).map_err(D::Error::custom)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct EvaluationCaseV1 {
    case_id: StableIdV1,
    case: ContentAddressedBindingV1,
    matcher_inputs: Vec<MatcherInputValueV1>,
}

impl EvaluationCaseV1 {
    pub fn new(
        case_id: StableIdV1,
        case: ContentAddressedBindingV1,
        mut matcher_inputs: Vec<MatcherInputValueV1>,
    ) -> Result<Self, ContractError> {
        normalize_matcher_inputs(&mut matcher_inputs)?;
        Ok(Self {
            case_id,
            case,
            matcher_inputs,
        })
    }
    pub fn case_id(&self) -> &StableIdV1 {
        &self.case_id
    }
    pub fn case_binding(&self) -> &ContentAddressedBindingV1 {
        &self.case
    }
    pub fn matcher_inputs(&self) -> &[MatcherInputValueV1] {
        &self.matcher_inputs
    }
}

impl<'de> Deserialize<'de> for EvaluationCaseV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Raw {
            case_id: StableIdV1,
            case: ContentAddressedBindingV1,
            matcher_inputs: Vec<MatcherInputValueV1>,
        }
        let raw = Raw::deserialize(deserializer)?;
        Self::new(raw.case_id, raw.case, raw.matcher_inputs).map_err(D::Error::custom)
    }
}

/// A complete six-dimension owner-native Resolution envelope.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ResolutionEnvelopeV1 {
    resolution_stack: ExactBindingV1,
    scope_horizon: StableIdV1,
    detail_resolution: StableIdV1,
    temporal_horizon: StableIdV1,
    authority_horizon: StableIdV1,
    memory_horizon: StableIdV1,
    validation_horizon: StableIdV1,
}

impl ResolutionEnvelopeV1 {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        resolution_stack: ExactBindingV1,
        scope_horizon: StableIdV1,
        detail_resolution: StableIdV1,
        temporal_horizon: StableIdV1,
        authority_horizon: StableIdV1,
        memory_horizon: StableIdV1,
        validation_horizon: StableIdV1,
    ) -> Self {
        Self {
            resolution_stack,
            scope_horizon,
            detail_resolution,
            temporal_horizon,
            authority_horizon,
            memory_horizon,
            validation_horizon,
        }
    }
    pub fn resolution_stack(&self) -> &ExactBindingV1 {
        &self.resolution_stack
    }
    pub fn scope_horizon(&self) -> &StableIdV1 {
        &self.scope_horizon
    }

    pub(crate) fn dimension_values(&self) -> [&StableIdV1; 6] {
        [
            &self.scope_horizon,
            &self.detail_resolution,
            &self.temporal_horizon,
            &self.authority_horizon,
            &self.memory_horizon,
            &self.validation_horizon,
        ]
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OwnerCurrentnessObservationV1 {
    semantic_time_utc: SemanticTimeV1,
    lifecycle_head_fingerprint: FingerprintV1,
}

impl OwnerCurrentnessObservationV1 {
    pub(crate) fn new(
        semantic_time_utc: SemanticTimeV1,
        lifecycle_head_fingerprint: FingerprintV1,
    ) -> Self {
        Self {
            semantic_time_utc,
            lifecycle_head_fingerprint,
        }
    }
    pub fn semantic_time_utc(&self) -> &SemanticTimeV1 {
        &self.semantic_time_utc
    }
    pub fn lifecycle_head_fingerprint(&self) -> &FingerprintV1 {
        &self.lifecycle_head_fingerprint
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct SourceCurrentnessObservationV1 {
    source: ContentAddressedBindingV1,
    revision: String,
    collected_at: SemanticTimeV1,
    observed_at: SemanticTimeV1,
    current_head: ContentAddressedBindingV1,
}

impl SourceCurrentnessObservationV1 {
    pub(crate) fn new(
        source: ContentAddressedBindingV1,
        revision: String,
        collected_at: SemanticTimeV1,
        observed_at: SemanticTimeV1,
        current_head: ContentAddressedBindingV1,
    ) -> Self {
        Self {
            source,
            revision,
            collected_at,
            observed_at,
            current_head,
        }
    }

    pub(crate) fn source(&self) -> &ContentAddressedBindingV1 {
        &self.source
    }
    pub(crate) fn revision(&self) -> &str {
        &self.revision
    }
    pub(crate) fn current_head(&self) -> &ContentAddressedBindingV1 {
        &self.current_head
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct EvidenceSourceAuthorityObservationV1 {
    actor_ref: StableIdV1,
    authority: ExactBindingV1,
    registry_state: ContentAddressedBindingV1,
    registry_head: ContentAddressedBindingV1,
    observed_at: SemanticTimeV1,
}

impl EvidenceSourceAuthorityObservationV1 {
    pub(crate) fn new(
        actor_ref: StableIdV1,
        authority: ExactBindingV1,
        registry_state: ContentAddressedBindingV1,
        registry_head: ContentAddressedBindingV1,
        observed_at: SemanticTimeV1,
    ) -> Self {
        Self {
            actor_ref,
            authority,
            registry_state,
            registry_head,
            observed_at,
        }
    }

    pub(crate) fn actor_ref(&self) -> &StableIdV1 {
        &self.actor_ref
    }

    pub(crate) fn authority(&self) -> &ExactBindingV1 {
        &self.authority
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceObservationV1 {
    Satisfied,
    Violated,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceExclusionKindV1 {
    NotRequested,
    UnsupportedKind,
    SourceRefused,
    InsufficientResolution,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ClaimExclusionV1 {
    claim_id: StableIdV1,
    kind: EvidenceExclusionKindV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ClaimPartitionV1 {
    observed: Vec<StableIdV1>,
    unobserved: Vec<StableIdV1>,
    excluded: Vec<ClaimExclusionV1>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct EvidenceSourceBindingV1 {
    source: ContentAddressedBindingV1,
    revision: String,
    collected_at: SemanticTimeV1,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct EvidenceCandidateBodyV1 {
    evaluation_run: ContentAddressedBindingV1,
    evaluation_lifecycle_basis: ContentAddressedBindingV1,
    contract: ExactBindingV1,
    subject: EvaluationSubjectV1,
    case: EvaluationCaseV1,
    observed_claim_ids: Vec<StableIdV1>,
    evidence_kind: StableIdV1,
    fact_schema: ExactBindingV1,
    fact_payload: Value,
    observation: EvidenceObservationV1,
    producer: ContentAddressedBindingV1,
    request_record: ContentAddressedBindingV1,
    execution_record: ContentAddressedBindingV1,
    sources: Vec<EvidenceSourceBindingV1>,
    claim_partition: ClaimPartitionV1,
    request_resolution: ResolutionEnvelopeV1,
    actual_resolution: ResolutionEnvelopeV1,
    observed_at: SemanticTimeV1,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct EvidenceCandidateV1 {
    schema_ref: &'static str,
    body: EvidenceCandidateBodyV1,
    candidate_ref: String,
    candidate_fingerprint: FingerprintV1,
}

impl EvidenceCandidateV1 {
    pub fn candidate_ref(&self) -> &str {
        &self.candidate_ref
    }
    pub fn candidate_fingerprint(&self) -> &FingerprintV1 {
        &self.candidate_fingerprint
    }
    pub fn canonical_json_bytes(&self) -> Result<Vec<u8>, ContractError> {
        canonical_bytes(self)
    }

    pub(crate) fn admission_body(&self) -> &EvidenceCandidateBodyV1 {
        &self.body
    }
}

impl EvidenceCandidateBodyV1 {
    pub(crate) fn evaluation_run(&self) -> &ContentAddressedBindingV1 {
        &self.evaluation_run
    }
    pub(crate) fn evaluation_lifecycle_basis(&self) -> &ContentAddressedBindingV1 {
        &self.evaluation_lifecycle_basis
    }
    pub(crate) fn contract(&self) -> &ExactBindingV1 {
        &self.contract
    }
    pub(crate) fn subject(&self) -> &EvaluationSubjectV1 {
        &self.subject
    }
    pub(crate) fn case(&self) -> &EvaluationCaseV1 {
        &self.case
    }
    pub(crate) fn observed_claim_ids(&self) -> &[StableIdV1] {
        &self.observed_claim_ids
    }
    pub(crate) fn evidence_kind(&self) -> &StableIdV1 {
        &self.evidence_kind
    }
    pub(crate) fn observation(&self) -> EvidenceObservationV1 {
        self.observation
    }
    pub(crate) fn producer(&self) -> &ContentAddressedBindingV1 {
        &self.producer
    }
    pub(crate) fn request_record(&self) -> &ContentAddressedBindingV1 {
        &self.request_record
    }
    pub(crate) fn execution_record(&self) -> &ContentAddressedBindingV1 {
        &self.execution_record
    }
    pub(crate) fn sources(&self) -> &[EvidenceSourceBindingV1] {
        &self.sources
    }
    pub(crate) fn claim_partition(&self) -> &ClaimPartitionV1 {
        &self.claim_partition
    }
    pub(crate) fn request_resolution(&self) -> &ResolutionEnvelopeV1 {
        &self.request_resolution
    }
    pub(crate) fn actual_resolution(&self) -> &ResolutionEnvelopeV1 {
        &self.actual_resolution
    }
    pub(crate) fn observed_at(&self) -> &SemanticTimeV1 {
        &self.observed_at
    }
}

impl EvidenceSourceBindingV1 {
    pub(crate) fn source(&self) -> &ContentAddressedBindingV1 {
        &self.source
    }
    pub(crate) fn revision(&self) -> &str {
        &self.revision
    }
    pub(crate) fn collected_at(&self) -> &SemanticTimeV1 {
        &self.collected_at
    }
}

impl ClaimPartitionV1 {
    pub(crate) fn observed(&self) -> &[StableIdV1] {
        &self.observed
    }
    pub(crate) fn unobserved(&self) -> &[StableIdV1] {
        &self.unobserved
    }
    pub(crate) fn excluded_ids(&self) -> impl Iterator<Item = &StableIdV1> {
        self.excluded.iter().map(|entry| &entry.claim_id)
    }
}

impl<'de> Deserialize<'de> for EvidenceCandidateV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Raw {
            schema_ref: String,
            body: EvidenceCandidateBodyV1,
            candidate_ref: String,
            candidate_fingerprint: FingerprintV1,
        }
        let raw = Raw::deserialize(deserializer)?;
        if raw.schema_ref != EVIDENCE_CANDIDATE_SCHEMA_REF_V1 {
            return Err(D::Error::custom(ContractError::InvalidInvariant));
        }
        validate_candidate_body(&raw.body).map_err(D::Error::custom)?;
        let expected = canonical_fingerprint(&CandidatePreimage {
            schema_ref: EVIDENCE_CANDIDATE_SCHEMA_REF_V1,
            body: &raw.body,
        })
        .map_err(D::Error::custom)?;
        if expected != raw.candidate_fingerprint {
            return Err(D::Error::custom(ContractError::DerivedValueMismatch));
        }
        validate_candidate_ref(&raw.candidate_ref, &expected).map_err(D::Error::custom)?;
        Ok(Self {
            schema_ref: EVIDENCE_CANDIDATE_SCHEMA_REF_V1,
            body: raw.body,
            candidate_ref: raw.candidate_ref,
            candidate_fingerprint: expected,
        })
    }
}

#[derive(Serialize)]
struct CandidatePreimage<'a> {
    schema_ref: &'static str,
    body: &'a EvidenceCandidateBodyV1,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct EvidenceRecordBodyV1 {
    observation: EvidenceCandidateBodyV1,
    effective_resolution: ResolutionEnvelopeV1,
    admission_currentness: OwnerCurrentnessObservationV1,
    source_currentness: Vec<SourceCurrentnessObservationV1>,
    evidence_source_authority: EvidenceSourceAuthorityObservationV1,
    admission_policy: ExactBindingV1,
    freshness_policies: Vec<ExactBindingV1>,
    stability_policies: Vec<ExactBindingV1>,
    admitted_candidate: ContentAddressedBindingV1,
}

impl IdentityBodyValidation for EvidenceRecordBodyV1 {
    fn validate_identity_body(&self) -> Result<(), ContractError> {
        validate_candidate_body(&self.observation)?;
        validate_source_currentness(&self.source_currentness)?;
        validate_ordered_unique(&self.freshness_policies)?;
        validate_ordered_unique(&self.stability_policies)?;
        let candidate_fingerprint = canonical_fingerprint(&CandidatePreimage {
            schema_ref: EVIDENCE_CANDIDATE_SCHEMA_REF_V1,
            body: &self.observation,
        })?;
        if self.admitted_candidate.fingerprint() != &candidate_fingerprint {
            return Err(ContractError::DerivedValueMismatch);
        }
        validate_candidate_ref(self.admitted_candidate.reference(), &candidate_fingerprint)?;
        if self.admission_policy != crate::evidence_admission_policy_binding_v1()?
            || self.stability_policies.is_empty()
            || self.freshness_policies.is_empty()
        {
            return Err(ContractError::InvalidInvariant);
        }
        Ok(())
    }
}

identity_record!(
    EvidenceRecordV1,
    EvidenceRecordBodyV1,
    EVIDENCE_RECORD_SCHEMA_REF_V1,
    evidence_ref,
    evidence_fingerprint,
    |body: &EvidenceRecordBodyV1, fingerprint: &FingerprintV1| format!(
        "{}#evidence/{}",
        body.observation.contract.reference(),
        fingerprint_hex(fingerprint)
    )
);

impl EvidenceRecordV1 {
    pub fn evaluation_run(&self) -> &ContentAddressedBindingV1 {
        &self.body.observation.evaluation_run
    }

    pub fn admitted_candidate(&self) -> &ContentAddressedBindingV1 {
        &self.body.admitted_candidate
    }

    pub(crate) fn observation(&self) -> &EvidenceCandidateBodyV1 {
        &self.body.observation
    }

    pub(crate) fn effective_resolution(&self) -> &ResolutionEnvelopeV1 {
        &self.body.effective_resolution
    }

    pub(crate) fn source_currentness(&self) -> &[SourceCurrentnessObservationV1] {
        &self.body.source_currentness
    }

    pub(crate) fn evidence_source_authority(&self) -> &EvidenceSourceAuthorityObservationV1 {
        &self.body.evidence_source_authority
    }
}

impl<'de> Deserialize<'de> for EvidenceRecordV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Raw {
            schema_ref: String,
            body: EvidenceRecordBodyV1,
            evidence_ref: String,
            evidence_fingerprint: FingerprintV1,
        }
        let raw = Raw::deserialize(deserializer)?;
        if raw.schema_ref != EVIDENCE_RECORD_SCHEMA_REF_V1 {
            return Err(D::Error::custom(ContractError::InvalidInvariant));
        }
        let expected = Self::from_body(raw.body).map_err(D::Error::custom)?;
        if expected.evidence_ref != raw.evidence_ref
            || expected.evidence_fingerprint != raw.evidence_fingerprint
        {
            return Err(D::Error::custom(ContractError::DerivedValueMismatch));
        }
        Ok(expected)
    }
}

pub(crate) struct AdmittedEvidenceWitnessV1(EvidenceRecordBodyV1);

impl AdmittedEvidenceWitnessV1 {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        observation: EvidenceCandidateBodyV1,
        effective_resolution: ResolutionEnvelopeV1,
        admission_currentness: OwnerCurrentnessObservationV1,
        source_currentness: Vec<SourceCurrentnessObservationV1>,
        evidence_source_authority: EvidenceSourceAuthorityObservationV1,
        admission_policy: ExactBindingV1,
        freshness_policies: Vec<ExactBindingV1>,
        stability_policies: Vec<ExactBindingV1>,
        admitted_candidate: ContentAddressedBindingV1,
    ) -> Self {
        Self(EvidenceRecordBodyV1 {
            observation,
            effective_resolution,
            admission_currentness,
            source_currentness,
            evidence_source_authority,
            admission_policy,
            freshness_policies,
            stability_policies,
            admitted_candidate,
        })
    }

    pub(crate) fn into_record(self) -> Result<EvidenceRecordV1, ContractError> {
        EvidenceRecordV1::from_body(self.0)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimVerdictDecisionV1 {
    Pass,
    Fail,
    Blocked,
    Warning,
    NotObserved,
    NotApplicable,
    Flaky,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ReasonV1 {
    code: StableIdV1,
    related: Vec<ContentAddressedBindingV1>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct FreshnessAccountingV1 {
    policy: ExactBindingV1,
    fresh: Vec<ContentAddressedBindingV1>,
    stale: Vec<ContentAddressedBindingV1>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ResolutionAccountingV1 {
    requested: ResolutionEnvelopeV1,
    eligible: Vec<ContentAddressedBindingV1>,
    insufficient: Vec<ContentAddressedBindingV1>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ClaimVerdictBodyV1 {
    evaluation_run: ContentAddressedBindingV1,
    evaluation_lifecycle_basis: ContentAddressedBindingV1,
    contract: ExactBindingV1,
    claim_id: StableIdV1,
    applicability: ApplicabilityOutcomeV1,
    evidence_repository_root: FingerprintV1,
    supporting_evidence: Vec<ContentAddressedBindingV1>,
    disqualifying_evidence: Vec<ContentAddressedBindingV1>,
    freshness_accounting: FreshnessAccountingV1,
    resolution_accounting: ResolutionAccountingV1,
    evidence_source_authority: Vec<EvidenceSourceAuthorityObservationV1>,
    verdict: ClaimVerdictDecisionV1,
    reasons: Vec<ReasonV1>,
}

impl IdentityBodyValidation for ClaimVerdictBodyV1 {
    fn validate_identity_body(&self) -> Result<(), ContractError> {
        validate_bounded_unique(&self.supporting_evidence)?;
        validate_bounded_unique(&self.disqualifying_evidence)?;
        validate_disjoint(&self.supporting_evidence, &self.disqualifying_evidence)?;
        validate_bounded_unique(&self.evidence_source_authority)?;
        validate_bounded(&self.reasons)?;
        validate_bounded_unique(&self.freshness_accounting.fresh)?;
        validate_bounded_unique(&self.freshness_accounting.stale)?;
        validate_disjoint(
            &self.freshness_accounting.fresh,
            &self.freshness_accounting.stale,
        )?;
        validate_bounded_unique(&self.resolution_accounting.eligible)?;
        validate_bounded_unique(&self.resolution_accounting.insufficient)?;
        validate_disjoint(
            &self.resolution_accounting.eligible,
            &self.resolution_accounting.insufficient,
        )?;
        for reason in &self.reasons {
            validate_bounded_unique(&reason.related)?;
        }
        Ok(())
    }
}

identity_record!(
    ClaimVerdictV1,
    ClaimVerdictBodyV1,
    CLAIM_VERDICT_SCHEMA_REF_V1,
    verdict_ref,
    verdict_fingerprint,
    |body: &ClaimVerdictBodyV1, fingerprint: &FingerprintV1| format!(
        "{}#verdict/{}/{}",
        body.evaluation_run.reference(),
        body.claim_id.as_str(),
        fingerprint_hex(fingerprint)
    )
);

impl ClaimVerdictV1 {
    pub fn claim_id(&self) -> &StableIdV1 {
        &self.body.claim_id
    }

    pub fn applicability(&self) -> ApplicabilityOutcomeV1 {
        self.body.applicability
    }

    pub fn evidence_repository_root(&self) -> &FingerprintV1 {
        &self.body.evidence_repository_root
    }

    pub fn supporting_evidence(&self) -> &[ContentAddressedBindingV1] {
        &self.body.supporting_evidence
    }

    pub fn disqualifying_evidence(&self) -> &[ContentAddressedBindingV1] {
        &self.body.disqualifying_evidence
    }

    pub fn decision(&self) -> ClaimVerdictDecisionV1 {
        self.body.verdict
    }

    pub fn reason_codes(&self) -> impl ExactSizeIterator<Item = &StableIdV1> {
        self.body.reasons.iter().map(|reason| &reason.code)
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn claim_verdict_from_accounting_v1(
    evaluation_run: ContentAddressedBindingV1,
    evaluation_lifecycle_basis: ContentAddressedBindingV1,
    contract: ExactBindingV1,
    claim_id: StableIdV1,
    applicability: ApplicabilityOutcomeV1,
    evidence_repository_root: FingerprintV1,
    supporting_evidence: Vec<ContentAddressedBindingV1>,
    disqualifying_evidence: Vec<ContentAddressedBindingV1>,
    freshness_policy: ExactBindingV1,
    fresh: Vec<ContentAddressedBindingV1>,
    stale: Vec<ContentAddressedBindingV1>,
    requested_resolution: ResolutionEnvelopeV1,
    resolution_eligible: Vec<ContentAddressedBindingV1>,
    resolution_insufficient: Vec<ContentAddressedBindingV1>,
    evidence_source_authority: Vec<EvidenceSourceAuthorityObservationV1>,
    verdict: ClaimVerdictDecisionV1,
    reasons: Vec<(StableIdV1, Vec<ContentAddressedBindingV1>)>,
) -> Result<ClaimVerdictV1, ContractError> {
    ClaimVerdictV1::from_body(ClaimVerdictBodyV1 {
        evaluation_run,
        evaluation_lifecycle_basis,
        contract,
        claim_id,
        applicability,
        evidence_repository_root,
        supporting_evidence,
        disqualifying_evidence,
        freshness_accounting: FreshnessAccountingV1 {
            policy: freshness_policy,
            fresh,
            stale,
        },
        resolution_accounting: ResolutionAccountingV1 {
            requested: requested_resolution,
            eligible: resolution_eligible,
            insufficient: resolution_insufficient,
        },
        evidence_source_authority,
        verdict,
        reasons: reasons
            .into_iter()
            .map(|(code, related)| ReasonV1 { code, related })
            .collect(),
    })
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GateDecisionV1 {
    Passed,
    Blocked,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "state", rename_all = "snake_case", deny_unknown_fields)]
pub enum ScoreResultV1 {
    NotComputed,
    Computed {
        passed_weight: u64,
        total_weight: u64,
    },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct BlockerV1 {
    code: StableIdV1,
    related: Vec<ContentAddressedBindingV1>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct NextActionV1 {
    code: StableIdV1,
    related: Vec<ContentAddressedBindingV1>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ClaimVerdictBindingV1 {
    claim_id: StableIdV1,
    verdict: ContentAddressedBindingV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct GateResultBodyV1 {
    evaluation_run: ContentAddressedBindingV1,
    evaluation_lifecycle_basis: ContentAddressedBindingV1,
    evidence_repository_root: FingerprintV1,
    gate_policy: ExactBindingV1,
    evidence_source_authority: Vec<EvidenceSourceAuthorityObservationV1>,
    verdicts: Vec<ClaimVerdictBindingV1>,
    score: ScoreResultV1,
    blockers: Vec<BlockerV1>,
    next_actions: Vec<NextActionV1>,
    decision: GateDecisionV1,
}

impl IdentityBodyValidation for GateResultBodyV1 {
    fn validate_identity_body(&self) -> Result<(), ContractError> {
        validate_bounded_unique(&self.evidence_source_authority)?;
        validate_bounded_unique(&self.verdicts)?;
        validate_bounded_unique(&self.blockers)?;
        validate_bounded_unique(&self.next_actions)?;
        validate_unique_by(&self.verdicts, |value| &value.claim_id)?;
        for blocker in &self.blockers {
            validate_bounded_unique(&blocker.related)?;
        }
        for action in &self.next_actions {
            validate_bounded_unique(&action.related)?;
        }
        Ok(())
    }
}

identity_record!(
    GateResultV1,
    GateResultBodyV1,
    GATE_RESULT_SCHEMA_REF_V1,
    gate_ref,
    gate_fingerprint,
    |body: &GateResultBodyV1, fingerprint: &FingerprintV1| format!(
        "{}#gate/{}",
        body.evaluation_run.reference(),
        fingerprint_hex(fingerprint)
    )
);

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalCloseoutDecisionV1 {
    Eligible,
    Blocked,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LocalCloseoutResultBodyV1 {
    evaluation_run: ContentAddressedBindingV1,
    gate: ContentAddressedBindingV1,
    local_closeout_policy: ExactBindingV1,
    final_lifecycle_head: FingerprintV1,
    source_currentness: Vec<SourceCurrentnessObservationV1>,
    evidence_source_authority: Vec<EvidenceSourceAuthorityObservationV1>,
    decision: LocalCloseoutDecisionV1,
    blockers: Vec<BlockerV1>,
    next_actions: Vec<NextActionV1>,
}

impl IdentityBodyValidation for LocalCloseoutResultBodyV1 {
    fn validate_identity_body(&self) -> Result<(), ContractError> {
        validate_source_currentness(&self.source_currentness)?;
        validate_bounded_unique(&self.evidence_source_authority)?;
        validate_bounded_unique(&self.blockers)?;
        validate_bounded_unique(&self.next_actions)?;
        for blocker in &self.blockers {
            validate_bounded_unique(&blocker.related)?;
        }
        for action in &self.next_actions {
            validate_bounded_unique(&action.related)?;
        }
        Ok(())
    }
}

identity_record!(
    LocalCloseoutResultV1,
    LocalCloseoutResultBodyV1,
    LOCAL_CLOSEOUT_RESULT_SCHEMA_REF_V1,
    local_closeout_ref,
    local_closeout_fingerprint,
    |body: &LocalCloseoutResultBodyV1, fingerprint: &FingerprintV1| format!(
        "{}#local-closeout/{}",
        body.evaluation_run.reference(),
        fingerprint_hex(fingerprint)
    )
);

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceOperationProblemCauseV1 {
    Malformed,
    OverLimit,
    Missing,
    Unknown,
    Stale,
    Substituted,
    Duplicate,
    Conflict,
    IneligibleCandidate,
    InsufficientResolution,
    RepositoryUnavailable,
    CandidateSourceUnavailable,
    DurabilityUncertain,
    InternalInvariant,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ContractEvidenceOperationDetailsV1 {
    schema_ref: &'static str,
    cause: EvidenceOperationProblemCauseV1,
    related_refs: Vec<String>,
    diagnostics: Vec<String>,
}

impl<'de> Deserialize<'de> for ContractEvidenceOperationDetailsV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Raw {
            schema_ref: String,
            cause: EvidenceOperationProblemCauseV1,
            related_refs: Vec<String>,
            diagnostics: Vec<String>,
        }
        let raw = Raw::deserialize(deserializer)?;
        if raw.schema_ref != EVIDENCE_OPERATION_DETAILS_SCHEMA_REF_V1
            || raw.related_refs.len() > MAX_ITEMS
            || raw.diagnostics.len() > MAX_ITEMS
            || raw
                .related_refs
                .iter()
                .any(|value| validate_ref(value).is_err())
            || raw
                .diagnostics
                .iter()
                .any(|value| validate_text(value, MAX_TEXT_BYTES).is_err())
        {
            return Err(D::Error::custom(ContractError::InvalidInvariant));
        }
        Ok(Self {
            schema_ref: EVIDENCE_OPERATION_DETAILS_SCHEMA_REF_V1,
            cause: raw.cause,
            related_refs: raw.related_refs,
            diagnostics: raw.diagnostics,
        })
    }
}

fn normalize_matcher_inputs(values: &mut [MatcherInputValueV1]) -> Result<(), ContractError> {
    if values.is_empty() {
        return Err(ContractError::Empty);
    }
    if values.len() > MAX_ITEMS {
        return Err(ContractError::BoundExceeded);
    }
    values.sort_by(|left, right| left.field.cmp(&right.field));
    if values
        .windows(2)
        .any(|window| window[0].field == window[1].field)
    {
        return Err(ContractError::InvalidInvariant);
    }
    Ok(())
}

fn validate_candidate_body(body: &EvidenceCandidateBodyV1) -> Result<(), ContractError> {
    if body.observed_claim_ids.is_empty() || body.sources.is_empty() {
        return Err(ContractError::Empty);
    }
    if body.observed_claim_ids.len() > MAX_ITEMS || body.sources.len() > MAX_ITEMS {
        return Err(ContractError::BoundExceeded);
    }
    if canonical_bytes(&body.fact_payload)?.len() > MAX_FACT_BYTES {
        return Err(ContractError::BoundExceeded);
    }
    validate_ordered_unique(&body.observed_claim_ids)?;
    validate_ordered_unique(&body.claim_partition.observed)?;
    validate_ordered_unique(&body.claim_partition.unobserved)?;
    if body.observed_claim_ids != body.claim_partition.observed {
        return Err(ContractError::InvalidInvariant);
    }
    let excluded_ids = body
        .claim_partition
        .excluded
        .iter()
        .map(|entry| entry.claim_id.clone())
        .collect::<Vec<_>>();
    validate_ordered_unique(&excluded_ids)?;
    validate_disjoint(
        &body.claim_partition.observed,
        &body.claim_partition.unobserved,
    )?;
    validate_disjoint(&body.claim_partition.observed, &excluded_ids)?;
    validate_disjoint(&body.claim_partition.unobserved, &excluded_ids)?;
    validate_bounded_unique(&body.sources)?;
    validate_unique_by(&body.sources, |value| &value.source)?;
    for source in &body.sources {
        validate_text(&source.revision, MAX_REVISION_BYTES)?;
    }
    Ok(())
}

fn validate_bounded<T>(values: &[T]) -> Result<(), ContractError> {
    if values.len() > MAX_ITEMS {
        return Err(ContractError::BoundExceeded);
    }
    Ok(())
}

fn validate_bounded_unique<T: PartialEq>(values: &[T]) -> Result<(), ContractError> {
    validate_bounded(values)?;
    if values
        .iter()
        .enumerate()
        .any(|(index, value)| values[..index].contains(value))
    {
        return Err(ContractError::InvalidInvariant);
    }
    Ok(())
}

fn validate_unique_by<T, K: PartialEq>(
    values: &[T],
    key: impl Fn(&T) -> &K,
) -> Result<(), ContractError> {
    if values.iter().enumerate().any(|(index, value)| {
        values[..index]
            .iter()
            .any(|previous| key(previous) == key(value))
    }) {
        return Err(ContractError::InvalidInvariant);
    }
    Ok(())
}

fn validate_source_currentness(
    values: &[SourceCurrentnessObservationV1],
) -> Result<(), ContractError> {
    validate_bounded_unique(values)?;
    validate_unique_by(values, |value| &value.source)?;
    for value in values {
        validate_text(&value.revision, MAX_REVISION_BYTES)?;
    }
    Ok(())
}

fn validate_ordered_unique<T: Ord>(values: &[T]) -> Result<(), ContractError> {
    validate_bounded(values)?;
    if values.windows(2).any(|window| window[0] >= window[1]) {
        return Err(ContractError::InvalidInvariant);
    }
    Ok(())
}

fn validate_disjoint<T: PartialEq>(left: &[T], right: &[T]) -> Result<(), ContractError> {
    if left.iter().any(|value| right.contains(value)) {
        return Err(ContractError::InvalidInvariant);
    }
    Ok(())
}

fn validate_ref(value: &str) -> Result<(), ContractError> {
    if value.is_empty()
        || value.len() > MAX_REF_BYTES
        || !value.is_ascii()
        || value
            .bytes()
            .any(|byte| byte.is_ascii_whitespace() || byte.is_ascii_control())
    {
        return Err(ContractError::InvalidInvariant);
    }
    Ok(())
}

fn validate_candidate_ref(
    reference: &str,
    fingerprint: &FingerprintV1,
) -> Result<(), ContractError> {
    validate_ref(reference)?;
    let hex = fingerprint_hex(fingerprint);
    if !reference.ends_with(hex)
        || reference
            .as_bytes()
            .get(reference.len().saturating_sub(hex.len() + 1))
            != Some(&b'/')
    {
        return Err(ContractError::DerivedValueMismatch);
    }
    Ok(())
}

fn validate_text(value: &str, max: usize) -> Result<(), ContractError> {
    if value.is_empty() {
        return Err(ContractError::Empty);
    }
    if value.len() > max || value.chars().any(char::is_control) {
        return Err(ContractError::BoundExceeded);
    }
    Ok(())
}

fn fingerprint_hex(value: &FingerprintV1) -> &str {
    value.as_str().strip_prefix("sha256:").expect("normalized")
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

fn civil_from_days(days_since_epoch: i64) -> (i64, i64, i64) {
    let z = days_since_epoch + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let day_of_era = z - era * 146_097;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        evidence_admission_policy_binding_v1, evidence_stability_policy_binding_v1,
        local_closeout_policy_binding_v1, ContractIdV1, ExactRefV1, VersionV1,
    };

    #[test]
    fn candidate_identity_is_source_owned_but_semantically_content_addressed() {
        let body = candidate_body();
        let fingerprint = canonical_fingerprint(&CandidatePreimage {
            schema_ref: EVIDENCE_CANDIDATE_SCHEMA_REF_V1,
            body: &body,
        })
        .unwrap();
        let hex = fingerprint_hex(&fingerprint);
        let value = serde_json::json!({
            "schema_ref": EVIDENCE_CANDIDATE_SCHEMA_REF_V1,
            "body": body,
            "candidate_ref": format!("producer.one/candidate/{hex}"),
            "candidate_fingerprint": fingerprint,
        });
        let candidate: EvidenceCandidateV1 = serde_json::from_value(value.clone()).unwrap();
        assert_eq!(
            serde_json_canonicalizer::to_vec(&serde_json::to_value(&candidate).unwrap()).unwrap(),
            candidate.canonical_json_bytes().unwrap()
        );

        let mut another_source = value.clone();
        another_source["candidate_ref"] = Value::String(format!("another.source/opaque/{hex}"));
        assert!(serde_json::from_value::<EvidenceCandidateV1>(another_source).is_ok());

        let mut changed = value;
        changed["body"]["observation"] = Value::String("violated".to_owned());
        assert!(serde_json::from_value::<EvidenceCandidateV1>(changed).is_err());
    }

    #[test]
    fn canonical_owner_records_bind_every_body_and_exact_ref_formula() {
        let candidate = candidate_body();
        let admitted_candidate = candidate_binding(&candidate);
        let record = EvidenceRecordV1::from_body(EvidenceRecordBodyV1 {
            observation: candidate.clone(),
            effective_resolution: resolution(),
            admission_currentness: OwnerCurrentnessObservationV1::new(time(), fingerprint('a')),
            source_currentness: vec![source_currentness()],
            evidence_source_authority: source_authority(),
            admission_policy: evidence_admission_policy_binding_v1().unwrap(),
            freshness_policies: vec![exact(
                "handbook.contract-evidence-freshness-policy.fixture@1.2.3",
            )],
            stability_policies: vec![evidence_stability_policy_binding_v1().unwrap()],
            admitted_candidate,
        })
        .unwrap();
        assert!(record
            .record_ref()
            .starts_with("handbook.contract.fixture@1.2.3#evidence/"));

        let verdict = ClaimVerdictV1::from_body(ClaimVerdictBodyV1 {
            evaluation_run: content("run/ref", 'c'),
            evaluation_lifecycle_basis: content("basis/ref", 'd'),
            contract: exact("handbook.contract.fixture@1.2.3"),
            claim_id: id("claim.one"),
            applicability: ApplicabilityOutcomeV1::Applicable,
            evidence_repository_root: fingerprint('e'),
            supporting_evidence: vec![content(record.record_ref(), 'f')],
            disqualifying_evidence: vec![],
            freshness_accounting: FreshnessAccountingV1 {
                policy: exact("handbook.contract-evidence-freshness-policy.fixture@1.2.3"),
                fresh: vec![content("evidence/fresh", '1')],
                stale: vec![],
            },
            resolution_accounting: ResolutionAccountingV1 {
                requested: resolution(),
                eligible: vec![content("evidence/eligible", '2')],
                insufficient: vec![],
            },
            evidence_source_authority: vec![source_authority()],
            verdict: ClaimVerdictDecisionV1::Pass,
            reasons: vec![],
        })
        .unwrap();
        assert!(verdict.record_ref().contains("#verdict/claim.one/"));

        let gate = GateResultV1::from_body(GateResultBodyV1 {
            evaluation_run: content("run/ref", 'c'),
            evaluation_lifecycle_basis: content("basis/ref", 'd'),
            evidence_repository_root: fingerprint('e'),
            gate_policy: exact("handbook.contract-gate-policy.fixture@1.2.3"),
            evidence_source_authority: vec![source_authority()],
            verdicts: vec![ClaimVerdictBindingV1 {
                claim_id: id("claim.one"),
                verdict: content(verdict.record_ref(), '3'),
            }],
            score: ScoreResultV1::NotComputed,
            blockers: vec![],
            next_actions: vec![],
            decision: GateDecisionV1::Passed,
        })
        .unwrap();
        assert!(gate.record_ref().contains("#gate/"));

        let closeout = LocalCloseoutResultV1::from_body(LocalCloseoutResultBodyV1 {
            evaluation_run: content("run/ref", 'c'),
            gate: content(gate.record_ref(), '4'),
            local_closeout_policy: local_closeout_policy_binding_v1().unwrap(),
            final_lifecycle_head: fingerprint('5'),
            source_currentness: vec![source_currentness()],
            evidence_source_authority: vec![source_authority()],
            decision: LocalCloseoutDecisionV1::Eligible,
            blockers: vec![],
            next_actions: vec![],
        })
        .unwrap();
        assert!(closeout.record_ref().contains("#local-closeout/"));

        for bytes in [
            record.canonical_json_bytes().unwrap(),
            verdict.canonical_json_bytes().unwrap(),
            gate.canonical_json_bytes().unwrap(),
            closeout.canonical_json_bytes().unwrap(),
        ] {
            let value: Value = serde_json::from_slice(&bytes).unwrap();
            assert_eq!(serde_json_canonicalizer::to_vec(&value).unwrap(), bytes);
        }
    }

    #[test]
    fn object_roots_refuse_unknown_fields_and_candidate_bounds() {
        let candidate = candidate_body();
        let mut record_body = serde_json::to_value(EvidenceRecordBodyV1 {
            observation: candidate.clone(),
            effective_resolution: resolution(),
            admission_currentness: OwnerCurrentnessObservationV1::new(time(), fingerprint('a')),
            source_currentness: vec![source_currentness()],
            evidence_source_authority: source_authority(),
            admission_policy: evidence_admission_policy_binding_v1().unwrap(),
            freshness_policies: vec![exact(
                "handbook.contract-evidence-freshness-policy.fixture@1.2.3",
            )],
            stability_policies: vec![evidence_stability_policy_binding_v1().unwrap()],
            admitted_candidate: candidate_binding(&candidate),
        })
        .unwrap();
        record_body["unexpected"] = Value::Bool(true);
        assert!(serde_json::from_value::<EvidenceRecordBodyV1>(record_body).is_err());

        let mut oversized = candidate_body();
        oversized.fact_payload = Value::String("x".repeat(MAX_FACT_BYTES + 1));
        let fingerprint = fingerprint('9');
        let value = serde_json::json!({
            "schema_ref": EVIDENCE_CANDIDATE_SCHEMA_REF_V1,
            "body": oversized,
            "candidate_ref": format!("producer/candidate/{}", fingerprint_hex(&fingerprint)),
            "candidate_fingerprint": fingerprint,
        });
        assert!(serde_json::from_value::<EvidenceCandidateV1>(value).is_err());
    }

    fn candidate_body() -> EvidenceCandidateBodyV1 {
        let observed = vec![id("claim.one")];
        EvidenceCandidateBodyV1 {
            evaluation_run: content("run/ref", '1'),
            evaluation_lifecycle_basis: content("basis/ref", '2'),
            contract: exact("handbook.contract.fixture@1.2.3"),
            subject: EvaluationSubjectV1::new(
                content("subject/ref", '3'),
                vec![MatcherInputValueV1::new(id("field.one"), "value").unwrap()],
            )
            .unwrap(),
            case: EvaluationCaseV1::new(
                id("case.one"),
                content("case/ref", '4'),
                vec![MatcherInputValueV1::new(id("field.one"), "value").unwrap()],
            )
            .unwrap(),
            observed_claim_ids: observed.clone(),
            evidence_kind: id("evidence.kind"),
            fact_schema: exact("handbook.fact.fixture@1.2.3"),
            fact_payload: serde_json::json!({"result": "exact"}),
            observation: EvidenceObservationV1::Satisfied,
            producer: content("producer/ref", '5'),
            request_record: content("request/ref", '6'),
            execution_record: content("execution/ref", '7'),
            sources: vec![EvidenceSourceBindingV1 {
                source: content("source/ref", '8'),
                revision: "revision-1".to_owned(),
                collected_at: time(),
            }],
            claim_partition: ClaimPartitionV1 {
                observed,
                unobserved: vec![id("claim.two")],
                excluded: vec![],
            },
            request_resolution: resolution(),
            actual_resolution: resolution(),
            observed_at: time(),
        }
    }

    fn candidate_binding(body: &EvidenceCandidateBodyV1) -> ContentAddressedBindingV1 {
        let fingerprint = canonical_fingerprint(&CandidatePreimage {
            schema_ref: EVIDENCE_CANDIDATE_SCHEMA_REF_V1,
            body,
        })
        .unwrap();
        ContentAddressedBindingV1::new(
            &format!("producer/candidate/{}", fingerprint_hex(&fingerprint)),
            fingerprint,
        )
        .unwrap()
    }

    fn source_currentness() -> SourceCurrentnessObservationV1 {
        SourceCurrentnessObservationV1 {
            source: content("source/ref", '8'),
            revision: "revision-1".to_owned(),
            collected_at: time(),
            observed_at: time(),
            current_head: content("source/head", '9'),
        }
    }

    fn source_authority() -> EvidenceSourceAuthorityObservationV1 {
        EvidenceSourceAuthorityObservationV1 {
            actor_ref: id("actor.one"),
            authority: exact("handbook.authority.evidence-source@1.2.3"),
            registry_state: content("registry/state", 'a'),
            registry_head: content("registry/head", 'b'),
            observed_at: time(),
        }
    }

    fn resolution() -> ResolutionEnvelopeV1 {
        ResolutionEnvelopeV1::new(
            exact("handbook.resolution.stack@1.2.3"),
            id("resolution.scope"),
            id("resolution.detail"),
            id("resolution.temporal"),
            id("resolution.authority"),
            id("resolution.memory"),
            id("resolution.validation"),
        )
    }

    fn exact(reference: &str) -> ExactBindingV1 {
        let parsed = ExactRefV1::parse(reference).unwrap();
        ExactBindingV1::new(parsed, fingerprint('0'))
    }

    fn content(reference: &str, byte: char) -> ContentAddressedBindingV1 {
        ContentAddressedBindingV1::new(reference, fingerprint(byte)).unwrap()
    }

    fn fingerprint(byte: char) -> FingerprintV1 {
        FingerprintV1::parse(&format!("sha256:{}", byte.to_string().repeat(64))).unwrap()
    }

    fn id(value: &str) -> StableIdV1 {
        StableIdV1::parse(value).unwrap()
    }

    fn time() -> SemanticTimeV1 {
        SemanticTimeV1::parse("2026-08-19T12:34:56Z").unwrap()
    }

    #[test]
    fn full_semver_exact_bindings_remain_distinct() {
        let stable = exact("handbook.policy.fixture@1.2.3");
        let prerelease = exact("handbook.policy.fixture@1.2.3-rc.1+build.7");
        assert_ne!(stable.reference(), prerelease.reference());
        assert!(ExactRefV1::parse("handbook.policy.fixture@latest").is_err());
        assert!(VersionV1::parse("1.2").is_err());
        assert!(ContractIdV1::parse("Handbook.Policy").is_err());
    }
}
