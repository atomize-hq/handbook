use crate::canonical::canonical_fingerprint;
use crate::{ContractError, ContractIdV1, ExactBindingV1, ExactRefV1};
use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::BTreeSet;

const MAX_CLAIMS: usize = 128;
const MAX_REQUIREMENTS: usize = 64;
const MAX_CRITERIA: usize = 64;
const MAX_MATCHER_VALUES: usize = 128;
const MAX_STATEMENT_BYTES: usize = 4096;
const MAX_VALUE_BYTES: usize = 1024;
const MAX_SAFE_INTEGER: u64 = 9_007_199_254_740_991;

/// A stable lowercase dot-separated identifier used inside owner records.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct StableIdV1(ContractIdV1);

impl StableIdV1 {
    /// Parses a stable bounded identifier.
    pub fn parse(value: &str) -> Result<Self, ContractError> {
        ContractIdV1::parse(value).map(Self)
    }

    /// Returns the canonical identifier.
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl Serialize for StableIdV1 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for StableIdV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::parse(&value).map_err(D::Error::custom)
    }
}

/// The exact checked-in matcher implementation and its exact input schema.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MatcherBindingV1 {
    matcher: ExactBindingV1,
    input_schema: ExactBindingV1,
}

impl MatcherBindingV1 {
    /// Creates a matcher binding. `SelectorV1` admits it only when it exactly
    /// equals a checked-in owner registration.
    pub fn new(matcher: ExactBindingV1, input_schema: ExactBindingV1) -> Self {
        Self {
            matcher,
            input_schema,
        }
    }

    /// Returns the sole V1 checked-in declarative matcher registration.
    pub fn exact_fields_v1() -> Self {
        #[derive(Serialize)]
        struct InputSchemaDefinition<'a> {
            schema_ref: &'a str,
            representation: &'a str,
            field_id: &'a str,
            field_value: &'a str,
            duplicate_fields: &'a str,
            unknown_object_members: &'a str,
        }

        #[derive(Serialize)]
        struct MatcherDefinition<'a> {
            matcher_ref: &'a str,
            input_schema_ref: &'a str,
            input_schema_fingerprint: &'a crate::FingerprintV1,
            algorithm: &'a str,
            comparison: &'a str,
            missing_input: &'a str,
            executable: bool,
            ambient_selection: bool,
        }

        let matcher_ref = ExactRefV1::parse("handbook.contract-matcher.exact-fields@1.0.0")
            .expect("checked-in matcher ref is valid");
        let input_schema_ref =
            ExactRefV1::parse("handbook.contract-matcher.exact-fields-input@1.0.0")
                .expect("checked-in matcher input ref is valid");
        let input_schema_fingerprint = canonical_fingerprint(&InputSchemaDefinition {
            schema_ref: input_schema_ref.as_str(),
            representation: "non_empty_ordered_field_value_pairs",
            field_id: "stable_lowercase_dot_separated_id",
            field_value: "bounded_non_empty_string",
            duplicate_fields: "refuse",
            unknown_object_members: "refuse",
        })
        .expect("checked-in matcher input definition is canonicalizable");
        let matcher_fingerprint = canonical_fingerprint(&MatcherDefinition {
            matcher_ref: matcher_ref.as_str(),
            input_schema_ref: input_schema_ref.as_str(),
            input_schema_fingerprint: &input_schema_fingerprint,
            algorithm: "ordered_field_exact_string_equality",
            comparison: "all_criteria_must_match",
            missing_input: "blocked",
            executable: false,
            ambient_selection: false,
        })
        .expect("checked-in matcher definition is canonicalizable");

        Self::new(
            ExactBindingV1::new(matcher_ref, matcher_fingerprint),
            ExactBindingV1::new(input_schema_ref, input_schema_fingerprint),
        )
    }

    /// Returns the matcher definition binding.
    pub fn matcher(&self) -> &ExactBindingV1 {
        &self.matcher
    }

    /// Returns the exact matcher-input schema binding.
    pub fn input_schema(&self) -> &ExactBindingV1 {
        &self.input_schema
    }

    pub(crate) fn is_registered(&self) -> bool {
        self == &Self::exact_fields_v1()
    }
}

/// One exact-field equality criterion in a declarative selector.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct SelectorCriterionV1 {
    field: StableIdV1,
    expected: String,
}

impl SelectorCriterionV1 {
    /// Creates a bounded, non-empty exact-field criterion.
    pub fn new(field: StableIdV1, expected: &str) -> Result<Self, ContractError> {
        validate_text(expected, MAX_VALUE_BYTES)?;
        Ok(Self {
            field,
            expected: expected.to_owned(),
        })
    }

    /// Returns the declarative input field.
    pub fn field(&self) -> &StableIdV1 {
        &self.field
    }

    /// Returns the exact expected value.
    pub fn expected(&self) -> &str {
        &self.expected
    }
}

impl<'de> Deserialize<'de> for SelectorCriterionV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Raw {
            field: StableIdV1,
            expected: String,
        }
        let raw = Raw::deserialize(deserializer)?;
        Self::new(raw.field, &raw.expected).map_err(D::Error::custom)
    }
}

/// A closed declarative selector interpreted by an exact registered matcher.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct SelectorV1 {
    matcher: MatcherBindingV1,
    criteria: Vec<SelectorCriterionV1>,
}

impl SelectorV1 {
    /// Creates a normalized selector and refuses unknown matcher semantics.
    pub fn new(
        matcher: MatcherBindingV1,
        mut criteria: Vec<SelectorCriterionV1>,
    ) -> Result<Self, ContractError> {
        if !matcher.is_registered() {
            return Err(ContractError::UnknownMatcher);
        }
        if criteria.is_empty() {
            return Err(ContractError::Empty);
        }
        if criteria.len() > MAX_CRITERIA {
            return Err(ContractError::BoundExceeded);
        }
        criteria.sort_by(|left, right| left.field.cmp(&right.field));
        if criteria
            .windows(2)
            .any(|window| window[0].field == window[1].field)
        {
            return Err(ContractError::InvalidInvariant);
        }
        Ok(Self { matcher, criteria })
    }

    /// Returns the exact registered matcher binding.
    pub fn matcher(&self) -> &MatcherBindingV1 {
        &self.matcher
    }

    /// Returns normalized exact-field criteria.
    pub fn criteria(&self) -> &[SelectorCriterionV1] {
        &self.criteria
    }
}

impl<'de> Deserialize<'de> for SelectorV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Raw {
            matcher: MatcherBindingV1,
            criteria: Vec<SelectorCriterionV1>,
        }
        let raw = Raw::deserialize(deserializer)?;
        Self::new(raw.matcher, raw.criteria).map_err(D::Error::custom)
    }
}

/// The sole V1 declarative applicability rule.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ApplicabilityRuleV1 {
    /// Both exact subject and case selectors must match.
    SubjectAndCaseMatch,
}

/// The only V1 gate-effect declarations.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GateEffectV1 {
    /// A failing or unobserved applicable claim blocks composition later.
    HardFail,
    /// A passing applicable claim is required later.
    Required,
    /// The claim is advisory and cannot become hard by transport defaulting.
    Advisory,
}

/// The only V1 evidence cardinalities.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceCardinalityV1 {
    /// Exactly one distinct eligible canonical record.
    ExactlyOne,
    /// One or more distinct eligible canonical records.
    AtLeastOne,
    /// One or more eligible records for every exact declared case.
    AllDeclaredCases,
}

/// The case closure for one all-of evidence clause.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum EvidenceCaseSelectionV1 {
    /// Use the claim's exact case selector.
    ClaimCaseSelector,
    /// Use a normalized exact non-empty case set.
    ExactCases { case_ids: Vec<StableIdV1> },
}

impl EvidenceCaseSelectionV1 {
    /// Creates a sorted, duplicate-free exact case set.
    pub fn exact_cases(mut case_ids: Vec<StableIdV1>) -> Result<Self, ContractError> {
        if case_ids.is_empty() {
            return Err(ContractError::Empty);
        }
        if case_ids.len() > MAX_MATCHER_VALUES {
            return Err(ContractError::BoundExceeded);
        }
        case_ids.sort();
        if case_ids.windows(2).any(|window| window[0] == window[1]) {
            return Err(ContractError::InvalidInvariant);
        }
        Ok(Self::ExactCases { case_ids })
    }
}

impl<'de> Deserialize<'de> for EvidenceCaseSelectionV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
        enum Raw {
            ClaimCaseSelector,
            ExactCases { case_ids: Vec<StableIdV1> },
        }
        match Raw::deserialize(deserializer)? {
            Raw::ClaimCaseSelector => Ok(Self::ClaimCaseSelector),
            Raw::ExactCases { case_ids } => Self::exact_cases(case_ids).map_err(D::Error::custom),
        }
    }
}

/// One required all-of evidence clause.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct EvidenceRequirementV1 {
    evidence_kind: StableIdV1,
    cardinality: EvidenceCardinalityV1,
    cases: EvidenceCaseSelectionV1,
    stability_policy: ExactBindingV1,
}

impl EvidenceRequirementV1 {
    /// Creates one closed all-of evidence clause.
    pub fn new(
        evidence_kind: StableIdV1,
        cardinality: EvidenceCardinalityV1,
        cases: EvidenceCaseSelectionV1,
        stability_policy: ExactBindingV1,
    ) -> Result<Self, ContractError> {
        if cardinality == EvidenceCardinalityV1::AllDeclaredCases
            && !matches!(cases, EvidenceCaseSelectionV1::ExactCases { .. })
        {
            return Err(ContractError::InvalidInvariant);
        }
        Ok(Self {
            evidence_kind,
            cardinality,
            cases,
            stability_policy,
        })
    }

    pub(crate) fn evidence_kind(&self) -> &StableIdV1 {
        &self.evidence_kind
    }

    pub(crate) fn cardinality(&self) -> EvidenceCardinalityV1 {
        self.cardinality
    }

    pub(crate) fn cases(&self) -> &EvidenceCaseSelectionV1 {
        &self.cases
    }

    pub(crate) fn stability_policy(&self) -> &ExactBindingV1 {
        &self.stability_policy
    }
}

impl<'de> Deserialize<'de> for EvidenceRequirementV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Raw {
            evidence_kind: StableIdV1,
            cardinality: EvidenceCardinalityV1,
            cases: EvidenceCaseSelectionV1,
            stability_policy: ExactBindingV1,
        }
        let raw = Raw::deserialize(deserializer)?;
        Self::new(
            raw.evidence_kind,
            raw.cardinality,
            raw.cases,
            raw.stability_policy,
        )
        .map_err(D::Error::custom)
    }
}

/// A complete six-dimension minimum Context Resolution envelope.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ResolutionMinimumV1 {
    resolution_stack: ExactBindingV1,
    scope_horizon: StableIdV1,
    detail_resolution: StableIdV1,
    temporal_horizon: StableIdV1,
    authority_horizon: StableIdV1,
    memory_horizon: StableIdV1,
    validation_horizon: StableIdV1,
}

impl ResolutionMinimumV1 {
    /// Creates a complete six-dimension minimum.
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

    pub(crate) fn resolution_stack(&self) -> &ExactBindingV1 {
        &self.resolution_stack
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

/// A positive JSON-safe integer score weight.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(transparent)]
pub struct ScoreWeightV1(u64);

impl ScoreWeightV1 {
    /// Creates a weight in `1..=9_007_199_254_740_991`.
    pub fn new(value: u64) -> Result<Self, ContractError> {
        if value == 0 || value > MAX_SAFE_INTEGER {
            return Err(ContractError::InvalidInvariant);
        }
        Ok(Self(value))
    }

    /// Returns the integer weight.
    pub fn get(self) -> u64 {
        self.0
    }
}

impl<'de> Deserialize<'de> for ScoreWeightV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u64::deserialize(deserializer)?;
        Self::new(value).map_err(D::Error::custom)
    }
}

/// Authored inputs for immutable claim construction.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContractClaimInputV1 {
    pub claim_id: StableIdV1,
    pub statement: String,
    pub subject_selector: SelectorV1,
    pub case_selector: SelectorV1,
    pub applicability_rule: ApplicabilityRuleV1,
    pub gate_effect: GateEffectV1,
    pub evidence_requirements: Vec<EvidenceRequirementV1>,
    pub freshness_policy: ExactBindingV1,
    pub minimum_resolution: ResolutionMinimumV1,
    pub score_weight: Option<ScoreWeightV1>,
}

/// One immutable owner-native contract claim.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ContractClaimV1 {
    claim_id: StableIdV1,
    statement: String,
    subject_selector: SelectorV1,
    case_selector: SelectorV1,
    applicability_rule: ApplicabilityRuleV1,
    gate_effect: GateEffectV1,
    evidence_requirements: Vec<EvidenceRequirementV1>,
    freshness_policy: ExactBindingV1,
    minimum_resolution: ResolutionMinimumV1,
    #[serde(skip_serializing_if = "Option::is_none")]
    score_weight: Option<ScoreWeightV1>,
}

impl ContractClaimV1 {
    /// Validates and immutably constructs a complete claim.
    pub fn new(input: ContractClaimInputV1) -> Result<Self, ContractError> {
        validate_text(&input.statement, MAX_STATEMENT_BYTES)?;
        if input.evidence_requirements.is_empty() {
            return Err(ContractError::Empty);
        }
        if input.evidence_requirements.len() > MAX_REQUIREMENTS {
            return Err(ContractError::BoundExceeded);
        }
        if input
            .evidence_requirements
            .iter()
            .enumerate()
            .any(|(index, value)| input.evidence_requirements[..index].contains(value))
        {
            return Err(ContractError::InvalidInvariant);
        }
        Ok(Self {
            claim_id: input.claim_id,
            statement: input.statement,
            subject_selector: input.subject_selector,
            case_selector: input.case_selector,
            applicability_rule: input.applicability_rule,
            gate_effect: input.gate_effect,
            evidence_requirements: input.evidence_requirements,
            freshness_policy: input.freshness_policy,
            minimum_resolution: input.minimum_resolution,
            score_weight: input.score_weight,
        })
    }

    /// Returns the contract-local claim ID.
    pub fn claim_id(&self) -> &StableIdV1 {
        &self.claim_id
    }

    /// Returns the normative statement.
    pub fn statement(&self) -> &str {
        &self.statement
    }

    /// Returns the subject selector.
    pub fn subject_selector(&self) -> &SelectorV1 {
        &self.subject_selector
    }

    /// Returns the case selector.
    pub fn case_selector(&self) -> &SelectorV1 {
        &self.case_selector
    }

    /// Returns the declarative applicability rule.
    pub fn applicability_rule(&self) -> ApplicabilityRuleV1 {
        self.applicability_rule
    }

    /// Returns the gate-effect declaration.
    pub fn gate_effect(&self) -> GateEffectV1 {
        self.gate_effect
    }

    /// Returns the ordered all-of evidence requirements.
    pub fn evidence_requirements(&self) -> &[EvidenceRequirementV1] {
        &self.evidence_requirements
    }

    pub(crate) fn freshness_policy(&self) -> &ExactBindingV1 {
        &self.freshness_policy
    }

    pub(crate) fn minimum_resolution(&self) -> &ResolutionMinimumV1 {
        &self.minimum_resolution
    }

    /// Returns the optional positive JSON-safe score weight.
    pub fn score_weight(&self) -> Option<ScoreWeightV1> {
        self.score_weight
    }
}

impl<'de> Deserialize<'de> for ContractClaimV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Raw {
            claim_id: StableIdV1,
            statement: String,
            subject_selector: SelectorV1,
            case_selector: SelectorV1,
            applicability_rule: ApplicabilityRuleV1,
            gate_effect: GateEffectV1,
            evidence_requirements: Vec<EvidenceRequirementV1>,
            freshness_policy: ExactBindingV1,
            minimum_resolution: ResolutionMinimumV1,
            #[serde(default, deserialize_with = "deserialize_optional_non_null")]
            score_weight: Option<ScoreWeightV1>,
        }
        let raw = Raw::deserialize(deserializer)?;
        Self::new(ContractClaimInputV1 {
            claim_id: raw.claim_id,
            statement: raw.statement,
            subject_selector: raw.subject_selector,
            case_selector: raw.case_selector,
            applicability_rule: raw.applicability_rule,
            gate_effect: raw.gate_effect,
            evidence_requirements: raw.evidence_requirements,
            freshness_policy: raw.freshness_policy,
            minimum_resolution: raw.minimum_resolution,
            score_weight: raw.score_weight,
        })
        .map_err(D::Error::custom)
    }
}

pub(crate) fn evaluation_policy_closure(
    claims: &[ContractClaimV1],
) -> (Vec<ExactBindingV1>, Vec<ExactBindingV1>) {
    let mut freshness = claims
        .iter()
        .map(|claim| claim.freshness_policy.clone())
        .collect::<Vec<_>>();
    let mut stability = claims
        .iter()
        .flat_map(|claim| claim.evidence_requirements.iter())
        .map(|requirement| requirement.stability_policy.clone())
        .collect::<Vec<_>>();
    freshness.sort();
    freshness.dedup();
    stability.sort();
    stability.dedup();
    (freshness, stability)
}

/// The only owner-native applicability outcomes.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ApplicabilityOutcomeV1 {
    Applicable,
    NotApplicable,
    Blocked,
}

/// The declared origin of matcher input. Only the owner registry is admitted.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ApplicabilityInputSourceV1 {
    OwnerRegistry,
    Dock,
    Ambient,
    Executable,
}

/// One exact matcher input value.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct MatcherValueV1 {
    field: StableIdV1,
    value: String,
}

impl MatcherValueV1 {
    /// Creates a bounded exact matcher input value.
    pub fn new(field: StableIdV1, value: &str) -> Result<Self, ContractError> {
        validate_text(value, MAX_VALUE_BYTES)?;
        Ok(Self {
            field,
            value: value.to_owned(),
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "state", rename_all = "snake_case")]
enum MatcherInputStateV1 {
    Current { values: Vec<MatcherValueV1> },
    Stale,
    Unavailable,
    Indeterminate,
}

/// An untrusted applicability-input candidate evaluated fail closed.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct MatcherInputCandidateV1 {
    source: ApplicabilityInputSourceV1,
    input_schema: ExactBindingV1,
    input: MatcherInputStateV1,
}

impl MatcherInputCandidateV1 {
    /// Creates a normalized current candidate.
    pub fn current(
        source: ApplicabilityInputSourceV1,
        input_schema: ExactBindingV1,
        mut values: Vec<MatcherValueV1>,
    ) -> Result<Self, ContractError> {
        if values.is_empty() {
            return Err(ContractError::Empty);
        }
        if values.len() > MAX_MATCHER_VALUES {
            return Err(ContractError::BoundExceeded);
        }
        values.sort_by(|left, right| left.field.cmp(&right.field));
        if values
            .windows(2)
            .any(|window| window[0].field == window[1].field)
        {
            return Err(ContractError::InvalidInvariant);
        }
        Ok(Self {
            source,
            input_schema,
            input: MatcherInputStateV1::Current { values },
        })
    }

    pub fn stale(source: ApplicabilityInputSourceV1, input_schema: ExactBindingV1) -> Self {
        Self {
            source,
            input_schema,
            input: MatcherInputStateV1::Stale,
        }
    }

    pub fn unavailable(source: ApplicabilityInputSourceV1, input_schema: ExactBindingV1) -> Self {
        Self {
            source,
            input_schema,
            input: MatcherInputStateV1::Unavailable,
        }
    }

    pub fn indeterminate(source: ApplicabilityInputSourceV1, input_schema: ExactBindingV1) -> Self {
        Self {
            source,
            input_schema,
            input: MatcherInputStateV1::Indeterminate,
        }
    }
}

/// Evaluates claim applicability once, before any evidence binding.
pub fn evaluate_applicability(
    claim: &ContractClaimV1,
    subject: &MatcherInputCandidateV1,
    case: &MatcherInputCandidateV1,
) -> ApplicabilityOutcomeV1 {
    let subject = evaluate_selector(claim.subject_selector(), subject);
    let case = evaluate_selector(claim.case_selector(), case);
    if subject == SelectorOutcome::Blocked || case == SelectorOutcome::Blocked {
        ApplicabilityOutcomeV1::Blocked
    } else if subject == SelectorOutcome::NoMatch || case == SelectorOutcome::NoMatch {
        ApplicabilityOutcomeV1::NotApplicable
    } else {
        ApplicabilityOutcomeV1::Applicable
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum SelectorOutcome {
    Match,
    NoMatch,
    Blocked,
}

fn evaluate_selector(
    selector: &SelectorV1,
    candidate: &MatcherInputCandidateV1,
) -> SelectorOutcome {
    if candidate.source != ApplicabilityInputSourceV1::OwnerRegistry
        || !selector.matcher.is_registered()
        || !candidate
            .input_schema
            .matches(selector.matcher.input_schema())
    {
        return SelectorOutcome::Blocked;
    }
    let MatcherInputStateV1::Current { values } = &candidate.input else {
        return SelectorOutcome::Blocked;
    };
    for criterion in &selector.criteria {
        let Some(value) = values.iter().find(|value| value.field == criterion.field) else {
            return SelectorOutcome::Blocked;
        };
        if value.value != criterion.expected {
            return SelectorOutcome::NoMatch;
        }
    }
    SelectorOutcome::Match
}

fn validate_text(value: &str, max_bytes: usize) -> Result<(), ContractError> {
    if value.trim().is_empty() {
        return Err(ContractError::Empty);
    }
    if value.len() > max_bytes {
        return Err(ContractError::BoundExceeded);
    }
    Ok(())
}

fn deserialize_optional_non_null<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    T::deserialize(deserializer).map(Some)
}

pub(crate) fn validate_claims(claims: &[ContractClaimV1]) -> Result<(), ContractError> {
    if claims.is_empty() {
        return Err(ContractError::Empty);
    }
    if claims.len() > MAX_CLAIMS {
        return Err(ContractError::BoundExceeded);
    }
    let mut ids = BTreeSet::new();
    if claims
        .iter()
        .any(|claim| !ids.insert(claim.claim_id.clone()))
    {
        return Err(ContractError::InvalidInvariant);
    }
    Ok(())
}
