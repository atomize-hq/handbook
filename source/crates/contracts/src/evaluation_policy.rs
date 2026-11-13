use crate::canonical::{canonical_bytes, canonical_fingerprint};
use crate::{ContractError, ContractIdV1, ExactBindingV1, ExactRefV1, FingerprintV1, VersionV1};
use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::BTreeMap;

pub const EVIDENCE_ADMISSION_POLICY_SCHEMA_REF_V1: &str =
    "handbook.contract-evidence-admission-policy@1.0.0";
pub const EVIDENCE_FRESHNESS_POLICY_SCHEMA_REF_V1: &str =
    "handbook.contract-evidence-freshness-policy@1.0.0";
pub const EVIDENCE_STABILITY_POLICY_SCHEMA_REF_V1: &str =
    "handbook.contract-evidence-stability-policy@1.0.0";
pub const GATE_POLICY_SCHEMA_REF_V1: &str = "handbook.contract-gate-policy@1.0.0";
pub const LOCAL_CLOSEOUT_POLICY_SCHEMA_REF_V1: &str =
    "handbook.contract-local-closeout-policy@1.0.0";

pub const EVIDENCE_ADMISSION_POLICY_REF_V1: &str =
    "handbook.contract-evidence-admission-policy.closed@1.0.0";
pub const EVIDENCE_STABILITY_POLICY_REF_V1: &str =
    "handbook.contract-evidence-stability-policy.outcome-consensus@1.0.0";
pub const LOCAL_CLOSEOUT_POLICY_REF_V1: &str =
    "handbook.contract-local-closeout-policy.gate-passed@1.0.0";

const JSON_SAFE_INTEGER_MAX: u64 = 9_007_199_254_740_991;
const MAX_REGISTERED_POLICIES: usize = 128;
const ADMISSION_SOURCE: &str =
    include_str!("../policies/contract-evidence-admission-policy-closed-1.0.0.json");
const STABILITY_SOURCE: &str =
    include_str!("../policies/contract-evidence-stability-policy-outcome-consensus-1.0.0.json");
const LOCAL_CLOSEOUT_SOURCE: &str =
    include_str!("../policies/contract-local-closeout-policy-gate-passed-1.0.0.json");

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct EvidenceAdmissionPolicyV1 {
    schema_ref: String,
    policy_ref: String,
    candidate_trust: String,
    credential_persistence: String,
    evidence_retention: String,
    require_current_evidence_source_authority_registry_head: bool,
    require_current_lifecycle_head: bool,
    require_exact_candidate_identity: bool,
    require_exact_source_revision: bool,
    require_independent_source_resolution: bool,
    require_sealed_source_admission: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct EvidenceStabilityPolicyV1 {
    schema_ref: String,
    policy_ref: String,
    algorithm: String,
    conflicting_outcome: String,
    differing_fact_detail_with_same_outcome: String,
    eligible_observations: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalCloseoutPolicyV1 {
    schema_ref: String,
    policy_ref: String,
    decision_on_indeterminate: String,
    require_complete_accounting: bool,
    require_current_evidence_source_authority_registry_heads: bool,
    require_current_lifecycle_head: bool,
    require_current_source_revisions: bool,
    require_gate_decision: String,
    require_score_basis_current: bool,
}

macro_rules! fixed_policy_deserialize {
    ($type:ident, $source:ident, $value:expr) => {
        impl<'de> Deserialize<'de> for $type {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let value = serde_json::Value::deserialize(deserializer)?;
                let canonical = serde_json_canonicalizer::to_vec(&value)
                    .map_err(|_| D::Error::custom(ContractError::Canonicalization))?;
                if canonical != canonical_source($source).as_bytes() {
                    return Err(D::Error::custom(ContractError::DerivedValueMismatch));
                }
                Ok($value)
            }
        }
    };
}

fixed_policy_deserialize!(
    EvidenceAdmissionPolicyV1,
    ADMISSION_SOURCE,
    admission_literal()
);
fixed_policy_deserialize!(
    EvidenceStabilityPolicyV1,
    STABILITY_SOURCE,
    stability_literal()
);
fixed_policy_deserialize!(
    LocalCloseoutPolicyV1,
    LOCAL_CLOSEOUT_SOURCE,
    local_closeout_literal()
);

fn admission_literal() -> EvidenceAdmissionPolicyV1 {
    EvidenceAdmissionPolicyV1 {
        schema_ref: EVIDENCE_ADMISSION_POLICY_SCHEMA_REF_V1.to_owned(),
        policy_ref: EVIDENCE_ADMISSION_POLICY_REF_V1.to_owned(),
        candidate_trust: "untrusted".to_owned(),
        credential_persistence: "forbidden".to_owned(),
        evidence_retention: "immutable_all".to_owned(),
        require_current_evidence_source_authority_registry_head: true,
        require_current_lifecycle_head: true,
        require_exact_candidate_identity: true,
        require_exact_source_revision: true,
        require_independent_source_resolution: true,
        require_sealed_source_admission: true,
    }
}

fn stability_literal() -> EvidenceStabilityPolicyV1 {
    EvidenceStabilityPolicyV1 {
        schema_ref: EVIDENCE_STABILITY_POLICY_SCHEMA_REF_V1.to_owned(),
        policy_ref: EVIDENCE_STABILITY_POLICY_REF_V1.to_owned(),
        algorithm: "outcome_consensus".to_owned(),
        conflicting_outcome: "flaky".to_owned(),
        differing_fact_detail_with_same_outcome: "consistent".to_owned(),
        eligible_observations: vec!["satisfied".to_owned(), "violated".to_owned()],
    }
}

fn local_closeout_literal() -> LocalCloseoutPolicyV1 {
    LocalCloseoutPolicyV1 {
        schema_ref: LOCAL_CLOSEOUT_POLICY_SCHEMA_REF_V1.to_owned(),
        policy_ref: LOCAL_CLOSEOUT_POLICY_REF_V1.to_owned(),
        decision_on_indeterminate: "blocked".to_owned(),
        require_complete_accounting: true,
        require_current_evidence_source_authority_registry_heads: true,
        require_current_lifecycle_head: true,
        require_current_source_revisions: true,
        require_gate_decision: "passed".to_owned(),
        require_score_basis_current: true,
    }
}

pub fn evidence_admission_policy_canonical_json_v1() -> &'static str {
    canonical_source(ADMISSION_SOURCE)
}
pub fn evidence_stability_policy_canonical_json_v1() -> &'static str {
    canonical_source(STABILITY_SOURCE)
}
pub fn local_closeout_policy_canonical_json_v1() -> &'static str {
    canonical_source(LOCAL_CLOSEOUT_SOURCE)
}

pub fn evidence_admission_policy_v1() -> Result<EvidenceAdmissionPolicyV1, ContractError> {
    let policy: EvidenceAdmissionPolicyV1 =
        serde_json::from_str(evidence_admission_policy_canonical_json_v1())
            .map_err(|_| ContractError::Canonicalization)?;
    if policy.schema_ref != EVIDENCE_ADMISSION_POLICY_SCHEMA_REF_V1
        || policy.policy_ref != EVIDENCE_ADMISSION_POLICY_REF_V1
        || canonical_bytes(&policy)? != evidence_admission_policy_canonical_json_v1().as_bytes()
    {
        return Err(ContractError::Canonicalization);
    }
    Ok(policy)
}

pub fn evidence_stability_policy_v1() -> Result<EvidenceStabilityPolicyV1, ContractError> {
    let policy: EvidenceStabilityPolicyV1 =
        serde_json::from_str(evidence_stability_policy_canonical_json_v1())
            .map_err(|_| ContractError::Canonicalization)?;
    if policy.schema_ref != EVIDENCE_STABILITY_POLICY_SCHEMA_REF_V1
        || policy.policy_ref != EVIDENCE_STABILITY_POLICY_REF_V1
        || canonical_bytes(&policy)? != evidence_stability_policy_canonical_json_v1().as_bytes()
    {
        return Err(ContractError::Canonicalization);
    }
    Ok(policy)
}

pub fn local_closeout_policy_v1() -> Result<LocalCloseoutPolicyV1, ContractError> {
    let policy: LocalCloseoutPolicyV1 =
        serde_json::from_str(local_closeout_policy_canonical_json_v1())
            .map_err(|_| ContractError::Canonicalization)?;
    if policy.schema_ref != LOCAL_CLOSEOUT_POLICY_SCHEMA_REF_V1
        || policy.policy_ref != LOCAL_CLOSEOUT_POLICY_REF_V1
        || canonical_bytes(&policy)? != local_closeout_policy_canonical_json_v1().as_bytes()
    {
        return Err(ContractError::Canonicalization);
    }
    Ok(policy)
}

pub fn evidence_admission_policy_binding_v1() -> Result<ExactBindingV1, ContractError> {
    fixed_binding(
        evidence_admission_policy_canonical_json_v1(),
        EVIDENCE_ADMISSION_POLICY_REF_V1,
    )
}
pub fn evidence_stability_policy_binding_v1() -> Result<ExactBindingV1, ContractError> {
    fixed_binding(
        evidence_stability_policy_canonical_json_v1(),
        EVIDENCE_STABILITY_POLICY_REF_V1,
    )
}
pub fn local_closeout_policy_binding_v1() -> Result<ExactBindingV1, ContractError> {
    fixed_binding(
        local_closeout_policy_canonical_json_v1(),
        LOCAL_CLOSEOUT_POLICY_REF_V1,
    )
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceFreshnessPolicyInputV1 {
    policy_id: ContractIdV1,
    policy_version: VersionV1,
    max_observation_age_seconds: u64,
    require_exact_source_revision: bool,
    future_skew_seconds: u64,
}

impl EvidenceFreshnessPolicyInputV1 {
    pub fn new(
        policy_id: ContractIdV1,
        policy_version: VersionV1,
        max_observation_age_seconds: u64,
        require_exact_source_revision: bool,
        future_skew_seconds: u64,
    ) -> Result<Self, ContractError> {
        if max_observation_age_seconds == 0
            || !require_exact_source_revision
            || future_skew_seconds != 0
        {
            return Err(ContractError::InvalidInvariant);
        }
        if max_observation_age_seconds > JSON_SAFE_INTEGER_MAX {
            return Err(ContractError::BoundExceeded);
        }
        Ok(Self {
            policy_id,
            policy_version,
            max_observation_age_seconds,
            require_exact_source_revision,
            future_skew_seconds,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct EvidenceFreshnessPolicyV1 {
    schema_ref: &'static str,
    policy_id: ContractIdV1,
    policy_version: VersionV1,
    policy_ref: ExactRefV1,
    max_observation_age_seconds: u64,
    require_exact_source_revision: bool,
    future_skew_seconds: u64,
    policy_fingerprint: FingerprintV1,
    #[serde(skip)]
    binding: ExactBindingV1,
}

impl EvidenceFreshnessPolicyV1 {
    pub fn new(input: EvidenceFreshnessPolicyInputV1) -> Result<Self, ContractError> {
        let policy_ref = ExactRefV1::derive(&input.policy_id, &input.policy_version);
        let preimage = FreshnessPreimage {
            schema_ref: EVIDENCE_FRESHNESS_POLICY_SCHEMA_REF_V1,
            policy_id: &input.policy_id,
            policy_version: &input.policy_version,
            policy_ref: &policy_ref,
            max_observation_age_seconds: input.max_observation_age_seconds,
            require_exact_source_revision: input.require_exact_source_revision,
            future_skew_seconds: input.future_skew_seconds,
        };
        let policy_fingerprint = canonical_fingerprint(&preimage)?;
        let binding = ExactBindingV1::new(policy_ref.clone(), policy_fingerprint.clone());
        Ok(Self {
            schema_ref: EVIDENCE_FRESHNESS_POLICY_SCHEMA_REF_V1,
            policy_id: input.policy_id,
            policy_version: input.policy_version,
            policy_ref,
            max_observation_age_seconds: input.max_observation_age_seconds,
            require_exact_source_revision: input.require_exact_source_revision,
            future_skew_seconds: input.future_skew_seconds,
            policy_fingerprint,
            binding,
        })
    }
    pub fn policy_version(&self) -> &VersionV1 {
        &self.policy_version
    }
    pub fn policy_ref(&self) -> &ExactRefV1 {
        &self.policy_ref
    }
    pub fn binding(&self) -> &ExactBindingV1 {
        &self.binding
    }
    pub fn exact_binding(&self) -> ExactBindingV1 {
        ExactBindingV1::new(self.policy_ref.clone(), self.policy_fingerprint.clone())
    }
    pub fn require_exact_source_revision(&self) -> bool {
        self.require_exact_source_revision
    }
    pub fn future_skew_seconds(&self) -> u64 {
        self.future_skew_seconds
    }
    pub fn max_observation_age_seconds(&self) -> u64 {
        self.max_observation_age_seconds
    }
    pub fn canonical_json_bytes(&self) -> Result<Vec<u8>, ContractError> {
        canonical_bytes(self)
    }
}

impl<'de> Deserialize<'de> for EvidenceFreshnessPolicyV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Raw {
            schema_ref: String,
            policy_id: ContractIdV1,
            policy_version: VersionV1,
            policy_ref: ExactRefV1,
            max_observation_age_seconds: u64,
            require_exact_source_revision: bool,
            future_skew_seconds: u64,
            policy_fingerprint: FingerprintV1,
        }
        let raw = Raw::deserialize(deserializer)?;
        if raw.schema_ref != EVIDENCE_FRESHNESS_POLICY_SCHEMA_REF_V1 {
            return Err(D::Error::custom(ContractError::InvalidInvariant));
        }
        let value = Self::new(
            EvidenceFreshnessPolicyInputV1::new(
                raw.policy_id,
                raw.policy_version,
                raw.max_observation_age_seconds,
                raw.require_exact_source_revision,
                raw.future_skew_seconds,
            )
            .map_err(D::Error::custom)?,
        )
        .map_err(D::Error::custom)?;
        if value.policy_ref != raw.policy_ref || value.policy_fingerprint != raw.policy_fingerprint
        {
            return Err(D::Error::custom(ContractError::DerivedValueMismatch));
        }
        Ok(value)
    }
}

#[derive(Serialize)]
struct FreshnessPreimage<'a> {
    schema_ref: &'static str,
    policy_id: &'a ContractIdV1,
    policy_version: &'a VersionV1,
    policy_ref: &'a ExactRefV1,
    max_observation_age_seconds: u64,
    require_exact_source_revision: bool,
    future_skew_seconds: u64,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GatePolicyAlgorithmV1 {
    ClosedAccounting,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ExactRationalThresholdV1 {
    numerator: u64,
    denominator: u64,
}

impl<'de> Deserialize<'de> for ExactRationalThresholdV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Raw {
            numerator: u64,
            denominator: u64,
        }
        let raw = Raw::deserialize(deserializer)?;
        Self::new(raw.numerator, raw.denominator).map_err(D::Error::custom)
    }
}

impl ExactRationalThresholdV1 {
    pub fn new(numerator: u64, denominator: u64) -> Result<Self, ContractError> {
        if numerator == 0 || numerator > denominator {
            return Err(ContractError::InvalidInvariant);
        }
        if denominator > JSON_SAFE_INTEGER_MAX {
            return Err(ContractError::BoundExceeded);
        }
        if greatest_common_divisor(numerator, denominator) != 1 {
            return Err(ContractError::InvalidInvariant);
        }
        Ok(Self {
            numerator,
            denominator,
        })
    }
    pub fn numerator(&self) -> u64 {
        self.numerator
    }
    pub fn denominator(&self) -> u64 {
        self.denominator
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GatePolicyInputV1 {
    policy_id: ContractIdV1,
    policy_version: VersionV1,
    algorithm: GatePolicyAlgorithmV1,
    score_threshold: Option<ExactRationalThresholdV1>,
    local_closeout_policy: ExactBindingV1,
}

impl GatePolicyInputV1 {
    pub fn new(
        policy_id: ContractIdV1,
        policy_version: VersionV1,
        algorithm: GatePolicyAlgorithmV1,
        score_threshold: Option<ExactRationalThresholdV1>,
        local_closeout_policy: ExactBindingV1,
    ) -> Result<Self, ContractError> {
        if local_closeout_policy != local_closeout_policy_binding_v1()? {
            return Err(ContractError::InvalidInvariant);
        }
        Ok(Self {
            policy_id,
            policy_version,
            algorithm,
            score_threshold,
            local_closeout_policy,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct GatePolicyV1 {
    schema_ref: &'static str,
    policy_id: ContractIdV1,
    policy_version: VersionV1,
    policy_ref: ExactRefV1,
    algorithm: GatePolicyAlgorithmV1,
    #[serde(skip_serializing_if = "Option::is_none")]
    score_threshold: Option<ExactRationalThresholdV1>,
    local_closeout_policy: ExactBindingV1,
    policy_fingerprint: FingerprintV1,
    #[serde(skip)]
    binding: ExactBindingV1,
}

impl GatePolicyV1 {
    pub fn new(input: GatePolicyInputV1) -> Result<Self, ContractError> {
        let policy_ref = ExactRefV1::derive(&input.policy_id, &input.policy_version);
        let preimage = GatePreimage {
            schema_ref: GATE_POLICY_SCHEMA_REF_V1,
            policy_id: &input.policy_id,
            policy_version: &input.policy_version,
            policy_ref: &policy_ref,
            algorithm: input.algorithm,
            score_threshold: input.score_threshold.as_ref(),
            local_closeout_policy: &input.local_closeout_policy,
        };
        let policy_fingerprint = canonical_fingerprint(&preimage)?;
        let binding = ExactBindingV1::new(policy_ref.clone(), policy_fingerprint.clone());
        Ok(Self {
            schema_ref: GATE_POLICY_SCHEMA_REF_V1,
            policy_id: input.policy_id,
            policy_version: input.policy_version,
            policy_ref,
            algorithm: input.algorithm,
            score_threshold: input.score_threshold,
            local_closeout_policy: input.local_closeout_policy,
            policy_fingerprint,
            binding,
        })
    }
    pub fn algorithm(&self) -> GatePolicyAlgorithmV1 {
        self.algorithm
    }
    pub fn score_threshold(&self) -> Option<&ExactRationalThresholdV1> {
        self.score_threshold.as_ref()
    }
    pub fn local_closeout_policy(&self) -> &ExactBindingV1 {
        &self.local_closeout_policy
    }
    pub fn exact_binding(&self) -> ExactBindingV1 {
        self.binding.clone()
    }
    pub fn binding(&self) -> &ExactBindingV1 {
        &self.binding
    }
    pub fn canonical_json_bytes(&self) -> Result<Vec<u8>, ContractError> {
        canonical_bytes(self)
    }
}

impl<'de> Deserialize<'de> for GatePolicyV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Raw {
            schema_ref: String,
            policy_id: ContractIdV1,
            policy_version: VersionV1,
            policy_ref: ExactRefV1,
            algorithm: GatePolicyAlgorithmV1,
            #[serde(default)]
            score_threshold: Option<ExactRationalThresholdV1>,
            local_closeout_policy: ExactBindingV1,
            policy_fingerprint: FingerprintV1,
        }
        let raw = Raw::deserialize(deserializer)?;
        if raw.schema_ref != GATE_POLICY_SCHEMA_REF_V1 {
            return Err(D::Error::custom(ContractError::InvalidInvariant));
        }
        let value = Self::new(
            GatePolicyInputV1::new(
                raw.policy_id,
                raw.policy_version,
                raw.algorithm,
                raw.score_threshold,
                raw.local_closeout_policy,
            )
            .map_err(D::Error::custom)?,
        )
        .map_err(D::Error::custom)?;
        if value.policy_ref != raw.policy_ref || value.policy_fingerprint != raw.policy_fingerprint
        {
            return Err(D::Error::custom(ContractError::DerivedValueMismatch));
        }
        Ok(value)
    }
}

#[derive(Serialize)]
struct GatePreimage<'a> {
    schema_ref: &'static str,
    policy_id: &'a ContractIdV1,
    policy_version: &'a VersionV1,
    policy_ref: &'a ExactRefV1,
    algorithm: GatePolicyAlgorithmV1,
    #[serde(skip_serializing_if = "Option::is_none")]
    score_threshold: Option<&'a ExactRationalThresholdV1>,
    local_closeout_policy: &'a ExactBindingV1,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvaluationPolicyKindV1 {
    Admission,
    Freshness,
    Stability,
    Gate,
    LocalCloseout,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RegisteredEvaluationPolicyV1 {
    kind: EvaluationPolicyKindV1,
    binding: ExactBindingV1,
    canonical_json: Vec<u8>,
}

impl RegisteredEvaluationPolicyV1 {
    pub fn kind(&self) -> EvaluationPolicyKindV1 {
        self.kind
    }
    pub fn binding(&self) -> &ExactBindingV1 {
        &self.binding
    }
    pub fn canonical_json(&self) -> Vec<u8> {
        self.canonical_json.clone()
    }
}

#[derive(Clone, Debug)]
pub struct EvaluationPolicyRegistryV1 {
    by_ref: BTreeMap<ExactRefV1, RegisteredEvaluationPolicyV1>,
}

impl EvaluationPolicyRegistryV1 {
    pub fn new() -> Result<Self, ContractError> {
        evidence_admission_policy_v1()?;
        evidence_stability_policy_v1()?;
        local_closeout_policy_v1()?;
        let mut registry = Self {
            by_ref: BTreeMap::new(),
        };
        registry.insert_fixed(
            EvaluationPolicyKindV1::Admission,
            evidence_admission_policy_binding_v1()?,
            evidence_admission_policy_canonical_json_v1()
                .as_bytes()
                .to_vec(),
        )?;
        registry.insert_fixed(
            EvaluationPolicyKindV1::Stability,
            evidence_stability_policy_binding_v1()?,
            evidence_stability_policy_canonical_json_v1()
                .as_bytes()
                .to_vec(),
        )?;
        registry.insert_fixed(
            EvaluationPolicyKindV1::LocalCloseout,
            local_closeout_policy_binding_v1()?,
            local_closeout_policy_canonical_json_v1()
                .as_bytes()
                .to_vec(),
        )?;
        Ok(registry)
    }
    pub fn len(&self) -> usize {
        self.by_ref.len()
    }
    pub fn is_empty(&self) -> bool {
        self.by_ref.is_empty()
    }
    pub fn register_freshness(
        &mut self,
        policy: EvidenceFreshnessPolicyV1,
    ) -> Result<(), ContractError> {
        self.insert_fixed(
            EvaluationPolicyKindV1::Freshness,
            policy.exact_binding(),
            policy.canonical_json_bytes()?,
        )
    }
    pub fn register_gate(&mut self, policy: GatePolicyV1) -> Result<(), ContractError> {
        self.insert_fixed(
            EvaluationPolicyKindV1::Gate,
            policy.exact_binding(),
            policy.canonical_json_bytes()?,
        )
    }
    pub fn resolve_exact(
        &self,
        binding: &ExactBindingV1,
    ) -> Result<&RegisteredEvaluationPolicyV1, ContractError> {
        self.by_ref
            .get(binding.reference())
            .filter(|registered| registered.binding == *binding)
            .ok_or(ContractError::InvalidInvariant)
    }
    fn insert_fixed(
        &mut self,
        kind: EvaluationPolicyKindV1,
        binding: ExactBindingV1,
        canonical_json: Vec<u8>,
    ) -> Result<(), ContractError> {
        if self.by_ref.len() >= MAX_REGISTERED_POLICIES {
            return Err(ContractError::BoundExceeded);
        }
        if self.by_ref.contains_key(binding.reference()) {
            return Err(ContractError::InvalidInvariant);
        }
        self.by_ref.insert(
            binding.reference().clone(),
            RegisteredEvaluationPolicyV1 {
                kind,
                binding,
                canonical_json,
            },
        );
        Ok(())
    }
}

fn canonical_source(source: &'static str) -> &'static str {
    source
        .strip_suffix('\n')
        .expect("checked-in policy ends in one LF")
}

fn fixed_binding(source: &str, reference: &str) -> Result<ExactBindingV1, ContractError> {
    let value: serde_json::Value =
        serde_json::from_str(source).map_err(|_| ContractError::Canonicalization)?;
    if serde_json_canonicalizer::to_string(&value).map_err(|_| ContractError::Canonicalization)?
        != source
        || value.get("policy_ref").and_then(|value| value.as_str()) != Some(reference)
    {
        return Err(ContractError::Canonicalization);
    }
    Ok(ExactBindingV1::new(
        ExactRefV1::parse(reference)?,
        canonical_fingerprint(&value)?,
    ))
}

const fn greatest_common_divisor(mut left: u64, mut right: u64) -> u64 {
    while right != 0 {
        let remainder = left % right;
        left = right;
        right = remainder;
    }
    left
}
