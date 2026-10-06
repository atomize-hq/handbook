use crate::canonical::{canonical_bytes, canonical_fingerprint};
use crate::{
    ContentAddressedBindingV1, ContractClaimV1, ContractError, ContractOwnerRepositoryErrorV1,
    ContractOwnerRepositoryV1, EvaluationCaseV1, EvaluationLifecycleAdmissionBasisV1,
    EvaluationPolicyRegistryV1, EvaluationSubjectV1, ExactBindingV1, ExactRefV1, FingerprintV1,
    MatcherBindingV1, OwnerCurrentnessObservationV1, ResolutionEnvelopeV1,
};
use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

const MAX_RUN_NONCE_BYTES: usize = 256;
const MAX_EVALUATION_CASES: usize = 128;
const MAX_EVALUATION_POLICIES: usize = 128;

/// Exact independently retrievable owner schema identity for evaluation runs.
pub const CONTRACT_EVALUATION_RUN_SCHEMA_REF_V1: &str = "handbook.contract-evaluation-run@1.0.0";
pub const LOCAL_SOURCE_SEAM_RUN_SCHEMA_REF_V2: &str =
    "handbook.contract-local-source-seam-run@2.0.0";

/// A bounded caller-selected nonce that distinguishes intentional observations
/// without granting authority or selecting compatible identities.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct EvaluationRunNonceV1(String);

impl EvaluationRunNonceV1 {
    /// Parses a non-empty bounded printable ASCII nonce exactly as supplied.
    pub fn parse(value: &str) -> Result<Self, ContractError> {
        if value.is_empty() {
            return Err(ContractError::Empty);
        }
        if value.len() > MAX_RUN_NONCE_BYTES {
            return Err(ContractError::BoundExceeded);
        }
        if !value
            .bytes()
            .all(|byte| byte.is_ascii_graphic() && !matches!(byte, b'"' | b'\\'))
        {
            return Err(ContractError::InvalidInvariant);
        }
        Ok(Self(value.to_owned()))
    }

    /// Returns the exact nonce.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for EvaluationRunNonceV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl Serialize for EvaluationRunNonceV1 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for EvaluationRunNonceV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::parse(&value).map_err(D::Error::custom)
    }
}

/// Bounded caller observations admitted by owner-side run preparation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrepareEvaluationRunInputV1 {
    contract: ExactBindingV1,
    expected_lifecycle_fingerprint: FingerprintV1,
    subject: EvaluationSubjectV1,
    cases: Vec<EvaluationCaseV1>,
    requested_resolution: ResolutionEnvelopeV1,
    matcher: MatcherBindingV1,
    freshness_policies: Vec<ExactBindingV1>,
    stability_policies: Vec<ExactBindingV1>,
    gate_policy: ExactBindingV1,
    local_closeout_policy: ExactBindingV1,
    nonce: EvaluationRunNonceV1,
}

impl PrepareEvaluationRunInputV1 {
    /// Creates the bounded untrusted portion of a run-preparation request.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        contract: ExactBindingV1,
        expected_lifecycle_fingerprint: FingerprintV1,
        subject: EvaluationSubjectV1,
        mut cases: Vec<EvaluationCaseV1>,
        requested_resolution: ResolutionEnvelopeV1,
        matcher: MatcherBindingV1,
        mut freshness_policies: Vec<ExactBindingV1>,
        mut stability_policies: Vec<ExactBindingV1>,
        gate_policy: ExactBindingV1,
        local_closeout_policy: ExactBindingV1,
        nonce: EvaluationRunNonceV1,
    ) -> Result<Self, ContractError> {
        if cases.is_empty() {
            return Err(ContractError::Empty);
        }
        if cases.len() > MAX_EVALUATION_CASES {
            return Err(ContractError::BoundExceeded);
        }
        if !matcher.is_registered() {
            return Err(ContractError::UnknownMatcher);
        }
        normalize_policies(&mut freshness_policies)?;
        normalize_policies(&mut stability_policies)?;
        cases.sort_by(|left, right| left.case_id().cmp(right.case_id()));
        if cases
            .windows(2)
            .any(|window| window[0].case_id() == window[1].case_id())
        {
            return Err(ContractError::InvalidInvariant);
        }
        Ok(Self {
            contract,
            expected_lifecycle_fingerprint,
            subject,
            cases,
            requested_resolution,
            matcher,
            freshness_policies,
            stability_policies,
            gate_policy,
            local_closeout_policy,
            nonce,
        })
    }

    pub(crate) fn contract(&self) -> &ExactBindingV1 {
        &self.contract
    }

    pub(crate) fn expected_lifecycle_fingerprint(&self) -> &FingerprintV1 {
        &self.expected_lifecycle_fingerprint
    }

    pub(crate) fn freshness_policies(&self) -> &[ExactBindingV1] {
        &self.freshness_policies
    }

    pub(crate) fn stability_policies(&self) -> &[ExactBindingV1] {
        &self.stability_policies
    }

    pub(crate) fn gate_policy(&self) -> &ExactBindingV1 {
        &self.gate_policy
    }

    pub(crate) fn local_closeout_policy(&self) -> &ExactBindingV1 {
        &self.local_closeout_policy
    }

    pub(crate) fn matcher(&self) -> &MatcherBindingV1 {
        &self.matcher
    }
}

/// One exact read-only owner-prepared evaluation run.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ContractEvaluationRunV1 {
    schema_ref: &'static str,
    contract_ref: ExactRefV1,
    definition_fingerprint: FingerprintV1,
    evaluation_lifecycle_basis: ContentAddressedBindingV1,
    subject: EvaluationSubjectV1,
    cases: Vec<EvaluationCaseV1>,
    claims: Vec<ContractClaimV1>,
    requested_resolution: ResolutionEnvelopeV1,
    matcher: MatcherBindingV1,
    freshness_policies: Vec<ExactBindingV1>,
    stability_policies: Vec<ExactBindingV1>,
    gate_policy: ExactBindingV1,
    local_closeout_policy: ExactBindingV1,
    nonce: EvaluationRunNonceV1,
    owner_currentness: OwnerCurrentnessObservationV1,
    evaluation_run_ref: String,
    evaluation_run_fingerprint: FingerprintV1,
}

impl ContractEvaluationRunV1 {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn from_owner_snapshot(
        contract_ref: ExactRefV1,
        definition_fingerprint: FingerprintV1,
        basis: &EvaluationLifecycleAdmissionBasisV1,
        input: PrepareEvaluationRunInputV1,
        claims: Vec<ContractClaimV1>,
        owner_currentness: OwnerCurrentnessObservationV1,
    ) -> Result<Self, ContractError> {
        if claims.is_empty() {
            return Err(ContractError::Empty);
        }
        if input.contract.reference() != &contract_ref
            || input.contract.fingerprint() != &definition_fingerprint
            || input.expected_lifecycle_fingerprint
                != *basis.active_resulting_lifecycle_fingerprint()
            || owner_currentness.lifecycle_head_fingerprint()
                != basis.active_resulting_lifecycle_fingerprint()
            || claims.iter().any(|claim| {
                claim.subject_selector().matcher() != &input.matcher
                    || claim.case_selector().matcher() != &input.matcher
            })
        {
            return Err(ContractError::DerivedValueMismatch);
        }
        let evaluation_lifecycle_basis =
            ContentAddressedBindingV1::new(basis.basis_ref(), basis.basis_fingerprint().clone())?;
        let preimage = EvaluationRunPreimage {
            schema_ref: CONTRACT_EVALUATION_RUN_SCHEMA_REF_V1,
            contract_ref: &contract_ref,
            definition_fingerprint: &definition_fingerprint,
            evaluation_lifecycle_basis: &evaluation_lifecycle_basis,
            subject: &input.subject,
            cases: &input.cases,
            claims: &claims,
            requested_resolution: &input.requested_resolution,
            matcher: &input.matcher,
            freshness_policies: &input.freshness_policies,
            stability_policies: &input.stability_policies,
            gate_policy: &input.gate_policy,
            local_closeout_policy: &input.local_closeout_policy,
            nonce: &input.nonce,
            owner_currentness: &owner_currentness,
        };
        let evaluation_run_fingerprint = canonical_fingerprint(&preimage)?;
        let evaluation_run_ref = format!(
            "{}#evaluation/{}",
            contract_ref,
            fingerprint_hex(&evaluation_run_fingerprint)
        );
        Ok(Self {
            schema_ref: CONTRACT_EVALUATION_RUN_SCHEMA_REF_V1,
            contract_ref,
            definition_fingerprint,
            evaluation_lifecycle_basis,
            subject: input.subject,
            cases: input.cases,
            claims,
            requested_resolution: input.requested_resolution,
            matcher: input.matcher,
            freshness_policies: input.freshness_policies,
            stability_policies: input.stability_policies,
            gate_policy: input.gate_policy,
            local_closeout_policy: input.local_closeout_policy,
            nonce: input.nonce,
            owner_currentness,
            evaluation_run_ref,
            evaluation_run_fingerprint,
        })
    }

    /// Returns the exact owner-derived run reference.
    pub fn evaluation_run_ref(&self) -> &str {
        &self.evaluation_run_ref
    }

    /// Returns the exact contract definition binding.
    pub fn contract(&self) -> ExactBindingV1 {
        ExactBindingV1::new(
            self.contract_ref.clone(),
            self.definition_fingerprint.clone(),
        )
    }

    /// Returns the RFC8785/SHA-256 run fingerprint.
    pub fn evaluation_run_fingerprint(&self) -> &FingerprintV1 {
        &self.evaluation_run_fingerprint
    }

    /// Returns the exact active-after-independent-lock basis binding.
    pub fn evaluation_lifecycle_basis(&self) -> &ContentAddressedBindingV1 {
        &self.evaluation_lifecycle_basis
    }

    /// Returns the definition-order complete claim set.
    pub fn claims(&self) -> &[ContractClaimV1] {
        &self.claims
    }

    /// Returns the exact subject observation binding and matcher input.
    pub fn subject(&self) -> &EvaluationSubjectV1 {
        &self.subject
    }

    /// Returns the normalized duplicate-free case set.
    pub fn cases(&self) -> &[EvaluationCaseV1] {
        &self.cases
    }

    /// Returns the requested complete six-dimension Resolution envelope.
    pub fn requested_resolution(&self) -> &ResolutionEnvelopeV1 {
        &self.requested_resolution
    }

    /// Returns the exact owner-registered matcher.
    pub fn matcher(&self) -> &MatcherBindingV1 {
        &self.matcher
    }

    /// Returns the normalized exact freshness-policy closure.
    pub fn freshness_policies(&self) -> &[ExactBindingV1] {
        &self.freshness_policies
    }

    /// Returns the normalized exact stability-policy closure.
    pub fn stability_policies(&self) -> &[ExactBindingV1] {
        &self.stability_policies
    }

    /// Returns the exact contract-selected gate policy.
    pub fn gate_policy(&self) -> &ExactBindingV1 {
        &self.gate_policy
    }

    /// Returns the exact local-closeout policy selected by the gate policy.
    pub fn local_closeout_policy(&self) -> &ExactBindingV1 {
        &self.local_closeout_policy
    }

    /// Returns the non-authorizing observation nonce.
    pub fn nonce(&self) -> &EvaluationRunNonceV1 {
        &self.nonce
    }

    /// Returns the owner-established semantic-time/current-head observation.
    pub fn owner_currentness(&self) -> &OwnerCurrentnessObservationV1 {
        &self.owner_currentness
    }

    /// Returns canonical JSON bytes for the complete run.
    pub fn canonical_json_bytes(&self) -> Result<Vec<u8>, ContractError> {
        canonical_bytes(self)
    }
}

impl<'de> Deserialize<'de> for ContractEvaluationRunV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Raw {
            schema_ref: String,
            contract_ref: ExactRefV1,
            definition_fingerprint: FingerprintV1,
            evaluation_lifecycle_basis: ContentAddressedBindingV1,
            subject: EvaluationSubjectV1,
            cases: Vec<EvaluationCaseV1>,
            claims: Vec<ContractClaimV1>,
            requested_resolution: ResolutionEnvelopeV1,
            matcher: MatcherBindingV1,
            freshness_policies: Vec<ExactBindingV1>,
            stability_policies: Vec<ExactBindingV1>,
            gate_policy: ExactBindingV1,
            local_closeout_policy: ExactBindingV1,
            nonce: EvaluationRunNonceV1,
            owner_currentness: OwnerCurrentnessObservationV1,
            evaluation_run_ref: String,
            evaluation_run_fingerprint: FingerprintV1,
        }
        let raw = Raw::deserialize(deserializer)?;
        let schema_ref = match raw.schema_ref.as_str() {
            CONTRACT_EVALUATION_RUN_SCHEMA_REF_V1 => CONTRACT_EVALUATION_RUN_SCHEMA_REF_V1,
            LOCAL_SOURCE_SEAM_RUN_SCHEMA_REF_V2 => LOCAL_SOURCE_SEAM_RUN_SCHEMA_REF_V2,
            _ => return Err(D::Error::custom(ContractError::InvalidInvariant)),
        };
        let input = PrepareEvaluationRunInputV1::new(
            ExactBindingV1::new(raw.contract_ref.clone(), raw.definition_fingerprint.clone()),
            raw.owner_currentness.lifecycle_head_fingerprint().clone(),
            raw.subject,
            raw.cases,
            raw.requested_resolution,
            raw.matcher,
            raw.freshness_policies,
            raw.stability_policies,
            raw.gate_policy,
            raw.local_closeout_policy,
            raw.nonce,
        )
        .map_err(D::Error::custom)?;
        let value = Self::from_prevalidated_parts(
            schema_ref,
            raw.contract_ref,
            raw.definition_fingerprint,
            raw.evaluation_lifecycle_basis,
            input,
            raw.claims,
            raw.owner_currentness,
        )
        .map_err(D::Error::custom)?;
        if value.evaluation_run_ref != raw.evaluation_run_ref
            || value.evaluation_run_fingerprint != raw.evaluation_run_fingerprint
        {
            return Err(D::Error::custom(ContractError::DerivedValueMismatch));
        }
        Ok(value)
    }
}

impl ContractEvaluationRunV1 {
    #[allow(clippy::too_many_arguments)]
    fn from_prevalidated_parts(
        schema_ref: &'static str,
        contract_ref: ExactRefV1,
        definition_fingerprint: FingerprintV1,
        evaluation_lifecycle_basis: ContentAddressedBindingV1,
        input: PrepareEvaluationRunInputV1,
        claims: Vec<ContractClaimV1>,
        owner_currentness: OwnerCurrentnessObservationV1,
    ) -> Result<Self, ContractError> {
        if claims.is_empty() {
            return Err(ContractError::Empty);
        }
        let preimage = EvaluationRunPreimage {
            schema_ref,
            contract_ref: &contract_ref,
            definition_fingerprint: &definition_fingerprint,
            evaluation_lifecycle_basis: &evaluation_lifecycle_basis,
            subject: &input.subject,
            cases: &input.cases,
            claims: &claims,
            requested_resolution: &input.requested_resolution,
            matcher: &input.matcher,
            freshness_policies: &input.freshness_policies,
            stability_policies: &input.stability_policies,
            gate_policy: &input.gate_policy,
            local_closeout_policy: &input.local_closeout_policy,
            nonce: &input.nonce,
            owner_currentness: &owner_currentness,
        };
        let evaluation_run_fingerprint = canonical_fingerprint(&preimage)?;
        let evaluation_run_ref = format!(
            "{}#evaluation/{}",
            contract_ref,
            fingerprint_hex(&evaluation_run_fingerprint)
        );
        Ok(Self {
            schema_ref,
            contract_ref,
            definition_fingerprint,
            evaluation_lifecycle_basis,
            subject: input.subject,
            cases: input.cases,
            claims,
            requested_resolution: input.requested_resolution,
            matcher: input.matcher,
            freshness_policies: input.freshness_policies,
            stability_policies: input.stability_policies,
            gate_policy: input.gate_policy,
            local_closeout_policy: input.local_closeout_policy,
            nonce: input.nonce,
            owner_currentness,
            evaluation_run_ref,
            evaluation_run_fingerprint,
        })
    }
    pub fn is_local_source_seam_v2(&self) -> bool {
        self.schema_ref == LOCAL_SOURCE_SEAM_RUN_SCHEMA_REF_V2
    }
    pub(crate) fn from_local_source_seam_v2(
        input: PrepareEvaluationRunInputV1,
        claims: Vec<ContractClaimV1>,
        basis: ContentAddressedBindingV1,
        currentness: OwnerCurrentnessObservationV1,
    ) -> Result<Self, ContractError> {
        Self::from_prevalidated_parts(
            LOCAL_SOURCE_SEAM_RUN_SCHEMA_REF_V2,
            input.contract.reference().clone(),
            input.contract.fingerprint().clone(),
            basis,
            input,
            claims,
            currentness,
        )
    }
}

#[derive(Serialize)]
struct EvaluationRunPreimage<'a> {
    schema_ref: &'static str,
    contract_ref: &'a ExactRefV1,
    definition_fingerprint: &'a FingerprintV1,
    evaluation_lifecycle_basis: &'a ContentAddressedBindingV1,
    subject: &'a EvaluationSubjectV1,
    cases: &'a [EvaluationCaseV1],
    claims: &'a [ContractClaimV1],
    requested_resolution: &'a ResolutionEnvelopeV1,
    matcher: &'a MatcherBindingV1,
    freshness_policies: &'a [ExactBindingV1],
    stability_policies: &'a [ExactBindingV1],
    gate_policy: &'a ExactBindingV1,
    local_closeout_policy: &'a ExactBindingV1,
    nonce: &'a EvaluationRunNonceV1,
    owner_currentness: &'a OwnerCurrentnessObservationV1,
}

fn normalize_policies(values: &mut [ExactBindingV1]) -> Result<(), ContractError> {
    if values.is_empty() {
        return Err(ContractError::Empty);
    }
    if values.len() > MAX_EVALUATION_POLICIES {
        return Err(ContractError::BoundExceeded);
    }
    values.sort();
    if values.windows(2).any(|window| window[0] == window[1]) {
        return Err(ContractError::InvalidInvariant);
    }
    Ok(())
}

fn fingerprint_hex(value: &FingerprintV1) -> &str {
    value
        .as_str()
        .strip_prefix("sha256:")
        .expect("fingerprints are normalized")
}

/// Prepares one evaluation run through the contracts owner repository's single
/// recovered read lock. This is owner-native and intentionally has no SDK or
/// ordinary-operation surface.
pub fn prepare_evaluation_run_v1(
    owner: &ContractOwnerRepositoryV1,
    input: PrepareEvaluationRunInputV1,
    policy_registry: &EvaluationPolicyRegistryV1,
) -> Result<ContractEvaluationRunV1, ContractOwnerRepositoryErrorV1> {
    owner.prepare_evaluation_run_v1(input, policy_registry)
}
