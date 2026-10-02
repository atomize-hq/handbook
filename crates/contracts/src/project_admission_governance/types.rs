//! Closed inputs and exact owner identities for the selected standalone scope.

use crate::canonical::canonical_bytes;
use crate::{ExactBindingV1, ExactRefV1, FingerprintV1};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::{fmt, path::Path};

pub const ADMISSION_GOVERNANCE_SCOPE_V1: &str = "contracts.evidence-freshness-policy/root";
pub const ADMISSION_GOVERNANCE_OWNER_V1: &str = "handbook-contracts";
pub const ADMISSION_CONFIGURATION_NAME_V1: &str = "project.admission.freshness";
pub const ADMISSION_CONFIGURATION_VERSION_V1: &str = "1.0.0";
pub const ESTABLISHMENT_RULE_NAME_V1: &str = "handbook.admission-establishment.standalone";
pub const FRESHNESS_ADMISSION_RULE_NAME_V1: &str = "handbook.contracts.freshness-policy-admission";
pub const GOVERNANCE_RULE_VERSION_V1: &str = "1.0.0";
pub const FRESHNESS_CANDIDATE_KIND_V1: &str = "EvidenceFreshnessPolicyV1";
pub const JSON_SAFE_INTEGER_MAX: u64 = 9_007_199_254_740_991;
pub const MAX_GOVERNANCE_RECORD_BYTES_V1: usize = 1024 * 1024;
const MAX_TEXT_BYTES: usize = 4096;
const MAX_REQUEST_KEY_BYTES: usize = 128;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityModeV1 {
    StandaloneLocalAdvisory,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ExpectedInitialStateV1 {
    FreshRegistrationUnused,
}

/// These asserted names are advisory claims, never verified credentials.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalAdvisoryInstructionV1 {
    pub mode: AuthorityModeV1,
    pub text: String,
    pub asserted_principal: Option<String>,
    pub asserted_agent: Option<String>,
    pub asserted_delegation: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReplacementAuthorityBasisV1 {
    pub mode: AuthorityModeV1,
    pub explicit_local_instruction_required: bool,
    pub same_project_and_scope_only: bool,
    pub preserve_fixed_constraints: bool,
    pub require_exact_predecessor: bool,
    pub require_confirmed_cutover: bool,
    pub terms_immutable: bool,
}

impl ReplacementAuthorityBasisV1 {
    /// Immutable prior terms; this packet provides no successor operation.
    pub fn standalone() -> Self {
        Self {
            mode: AuthorityModeV1::StandaloneLocalAdvisory,
            explicit_local_instruction_required: true,
            same_project_and_scope_only: true,
            preserve_fixed_constraints: true,
            require_exact_predecessor: true,
            require_confirmed_cutover: true,
            terms_immutable: true,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FreshnessRuleBindingInputV1 {
    pub owner: String,
    pub rule_name: String,
    pub rule_version: String,
    pub candidate_kind: String,
    /// Required caller input in seconds; no implicit ceiling is supplied.
    pub max_observation_age_seconds: u64,
    pub require_exact_source_revision: bool,
    pub future_skew_seconds: u64,
    pub required_authored_sources: Vec<String>,
    pub approval_sources: Vec<String>,
    pub exception_sources: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AdmissionConfigurationInputV1 {
    pub owner: String,
    pub name: String,
    pub version: String,
    pub bindings: Vec<FreshnessRuleBindingInputV1>,
}

/// Untrusted operation input, validated again by the native owner.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GovernanceRequestInputV1 {
    pub scope: String,
    pub configuration: AdmissionConfigurationInputV1,
    pub instruction: LocalAdvisoryInstructionV1,
    pub replacement_basis: ReplacementAuthorityBasisV1,
    pub expected_initial_state: ExpectedInitialStateV1,
    pub request_key: String,
}

impl GovernanceRequestInputV1 {
    pub fn standalone_freshness(
        max_observation_age_seconds: u64,
        request_key: impl Into<String>,
        instruction_text: impl Into<String>,
    ) -> Self {
        Self::standalone(request_key, max_observation_age_seconds, instruction_text)
    }

    /// Builds the selected fixed vocabulary; all varying authority input remains
    /// caller supplied and this constructor grants no eligibility or permission.
    pub fn standalone(
        request_key: impl Into<String>,
        max_observation_age_seconds: u64,
        instruction_text: impl Into<String>,
    ) -> Self {
        Self {
            scope: ADMISSION_GOVERNANCE_SCOPE_V1.into(),
            configuration: AdmissionConfigurationInputV1 {
                owner: ADMISSION_GOVERNANCE_OWNER_V1.into(),
                name: ADMISSION_CONFIGURATION_NAME_V1.into(),
                version: ADMISSION_CONFIGURATION_VERSION_V1.into(),
                bindings: vec![FreshnessRuleBindingInputV1 {
                    owner: ADMISSION_GOVERNANCE_OWNER_V1.into(),
                    rule_name: FRESHNESS_ADMISSION_RULE_NAME_V1.into(),
                    rule_version: GOVERNANCE_RULE_VERSION_V1.into(),
                    candidate_kind: FRESHNESS_CANDIDATE_KIND_V1.into(),
                    max_observation_age_seconds,
                    require_exact_source_revision: true,
                    future_skew_seconds: 0,
                    required_authored_sources: vec![],
                    approval_sources: vec![],
                    exception_sources: vec![],
                }],
            },
            instruction: LocalAdvisoryInstructionV1 {
                mode: AuthorityModeV1::StandaloneLocalAdvisory,
                text: instruction_text.into(),
                asserted_principal: None,
                asserted_agent: None,
                asserted_delegation: None,
            },
            replacement_basis: ReplacementAuthorityBasisV1::standalone(),
            expected_initial_state: ExpectedInitialStateV1::FreshRegistrationUnused,
            request_key: request_key.into(),
        }
    }

    pub fn from_json_bytes(bytes: &[u8]) -> Result<Self, GovernanceRefusalV1> {
        parse_governance_request_json(bytes)
    }

    pub fn canonical_json_bytes(&self) -> Result<Vec<u8>, GovernanceRefusalV1> {
        validate_request(self.clone())?;
        governance_canonical_bytes(self)
    }
}

/// The retained path is a locator. Exact identity and native registration supply
/// project authority; the physical-root binding detects replacement of that path.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectBindingV1 {
    pub repository_identity_fingerprint: String,
    pub retained_root: String,
    pub physical_root_fingerprint: String,
    pub registration_identity: FingerprintV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GovernanceRequestLocatorV1 {
    pub project: ProjectBindingV1,
    pub request: GovernanceRequestInputV1,
    pub request_fingerprint: FingerprintV1,
}

impl GovernanceRequestLocatorV1 {
    /// Restoring a locator grants resolution only, never a fresh-creation token.
    pub fn from_json_bytes(bytes: &[u8]) -> Result<Self, GovernanceRefusalV1> {
        let locator: Self = decode_governance_json(bytes)?;
        validate_request_locator(&locator)?;
        Ok(locator)
    }

    pub fn canonical_json_bytes(&self) -> Result<Vec<u8>, GovernanceRefusalV1> {
        validate_request_locator(self)?;
        governance_canonical_bytes(self)
    }
}

/// Exact canonical shipped rule content, including its derived content binding.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GovernanceRuleDescriptorV1 {
    pub owner: String,
    pub name: String,
    pub version: String,
    pub candidate_kind: Option<String>,
    pub candidate_schema_ref: Option<String>,
    pub required_authored_sources: Vec<String>,
    pub approval_sources: Vec<String>,
    pub exception_sources: Vec<String>,
    pub constraints: Vec<String>,
    pub binding: ExactBindingV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ValidatedGovernanceV1 {
    pub input: GovernanceRequestInputV1,
    pub configuration_fingerprint: FingerprintV1,
    pub instruction_fingerprint: FingerprintV1,
    pub establishment_rule: GovernanceRuleDescriptorV1,
    pub owner_rule: GovernanceRuleDescriptorV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GovernanceRefusalV1 {
    pub condition: String,
    pub detail: String,
    pub project: Option<ProjectBindingV1>,
    pub configuration_fingerprint: Option<FingerprintV1>,
}

impl GovernanceRefusalV1 {
    pub fn new(condition: impl Into<String>, detail: impl Into<String>) -> Self {
        Self {
            condition: condition.into(),
            detail: detail.into(),
            project: None,
            configuration_fingerprint: None,
        }
    }

    pub fn with_project(mut self, project: ProjectBindingV1) -> Self {
        self.project = Some(project);
        self
    }
}

impl fmt::Display for GovernanceRefusalV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.condition, self.detail)
    }
}

impl std::error::Error for GovernanceRefusalV1 {}

pub fn validate_request(
    input: GovernanceRequestInputV1,
) -> Result<ValidatedGovernanceV1, GovernanceRefusalV1> {
    if input.scope != ADMISSION_GOVERNANCE_SCOPE_V1 {
        return Err(GovernanceRefusalV1::new(
            "scope",
            "only the exact Contracts root freshness scope is supported",
        ));
    }
    let configuration = &input.configuration;
    if configuration.owner != ADMISSION_GOVERNANCE_OWNER_V1
        || configuration.name != ADMISSION_CONFIGURATION_NAME_V1
        || configuration.version != ADMISSION_CONFIGURATION_VERSION_V1
    {
        return Err(GovernanceRefusalV1::new(
            "configuration_identity",
            "unsupported configuration owner, name or version",
        ));
    }
    if configuration.bindings.len() != 1 {
        return Err(GovernanceRefusalV1::new(
            "binding_population",
            "exactly one owner-rule binding is required",
        ));
    }
    let binding = &configuration.bindings[0];
    if binding.owner != ADMISSION_GOVERNANCE_OWNER_V1
        || binding.rule_name != FRESHNESS_ADMISSION_RULE_NAME_V1
        || binding.rule_version != GOVERNANCE_RULE_VERSION_V1
        || binding.candidate_kind != FRESHNESS_CANDIDATE_KIND_V1
    {
        return Err(GovernanceRefusalV1::new(
            "owner_rule",
            "the exact shipped freshness owner rule and candidate kind are required",
        ));
    }
    if binding.max_observation_age_seconds == 0
        || binding.max_observation_age_seconds > JSON_SAFE_INTEGER_MAX
    {
        return Err(GovernanceRefusalV1::new("freshness_ceiling", "the caller must supply an integer ceiling from 1 through JSON_SAFE_INTEGER_MAX seconds"));
    }
    if !binding.require_exact_source_revision || binding.future_skew_seconds != 0 {
        return Err(GovernanceRefusalV1::new(
            "freshness_floor",
            "exact source revision is required and future skew must be zero",
        ));
    }
    if !binding.required_authored_sources.is_empty()
        || !binding.approval_sources.is_empty()
        || !binding.exception_sources.is_empty()
    {
        return Err(GovernanceRefusalV1::new("governance_sources", "the selected standalone rule declares an exact empty authored, approval and exception source population"));
    }
    if input.replacement_basis != ReplacementAuthorityBasisV1::standalone() {
        return Err(GovernanceRefusalV1::new(
            "replacement_basis",
            "prior replacement terms must preserve the exact fixed standalone basis",
        ));
    }
    if !bounded_nonempty_text(&input.instruction.text, MAX_TEXT_BYTES)
        || [
            &input.instruction.asserted_principal,
            &input.instruction.asserted_agent,
            &input.instruction.asserted_delegation,
        ]
        .iter()
        .any(|claim| {
            claim
                .as_ref()
                .is_some_and(|text| !bounded_nonempty_text(text, MAX_TEXT_BYTES))
        })
    {
        return Err(GovernanceRefusalV1::new(
            "explicit_instruction",
            "explicit advisory instruction and any asserted claims must be nonempty bounded text",
        ));
    }
    if !bounded_nonempty_text(&input.request_key, MAX_REQUEST_KEY_BYTES)
        || !input
            .request_key
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.'))
    {
        return Err(GovernanceRefusalV1::new(
            "request_key",
            "request key must be 1..128 ASCII letters, digits, hyphens, underscores or periods",
        ));
    }
    // Encode only after numeric and finite-population validation, avoiding lossy
    // canonicalization of out-of-range JSON numbers.
    governance_canonical_bytes(&input)?;
    Ok(ValidatedGovernanceV1 {
        configuration_fingerprint: governance_fingerprint(
            "handbook.project-admission-governance.configuration@1.0.0",
            configuration,
        )?,
        instruction_fingerprint: governance_fingerprint(
            "handbook.project-admission-governance.local-advisory-instruction@1.0.0",
            &input.instruction,
        )?,
        establishment_rule: establishment_rule_descriptor_v1()?,
        owner_rule: freshness_admission_rule_descriptor_v1()?,
        input,
    })
}

pub fn parse_governance_request_json(
    bytes: &[u8],
) -> Result<GovernanceRequestInputV1, GovernanceRefusalV1> {
    let input: GovernanceRequestInputV1 = decode_governance_json(bytes)?;
    Ok(validate_request(input)?.input)
}

pub fn create_request_locator(
    project: ProjectBindingV1,
    input: GovernanceRequestInputV1,
) -> Result<GovernanceRequestLocatorV1, GovernanceRefusalV1> {
    validate_project_binding(&project)?;
    let validated =
        validate_request(input).map_err(|refusal| refusal.with_project(project.clone()))?;
    let request_fingerprint = request_fingerprint(&project, &validated.input)?;
    let locator = GovernanceRequestLocatorV1 {
        project,
        request: validated.input,
        request_fingerprint,
    };
    governance_canonical_bytes(&locator)?;
    Ok(locator)
}

pub fn validate_request_locator(
    locator: &GovernanceRequestLocatorV1,
) -> Result<ValidatedGovernanceV1, GovernanceRefusalV1> {
    validate_project_binding(&locator.project)?;
    let validated = validate_request(locator.request.clone())
        .map_err(|refusal| refusal.with_project(locator.project.clone()))?;
    if request_fingerprint(&locator.project, &locator.request)? != locator.request_fingerprint {
        return Err(GovernanceRefusalV1::new(
            "request_identity",
            "original project and complete request do not match the exact request fingerprint",
        )
        .with_project(locator.project.clone()));
    }
    governance_canonical_bytes(locator)?;
    Ok(validated)
}

pub fn establishment_rule_descriptor_v1() -> Result<GovernanceRuleDescriptorV1, GovernanceRefusalV1>
{
    shipped_rule_descriptor(
        "handbook",
        ESTABLISHMENT_RULE_NAME_V1,
        None,
        &[
            "positive_fresh_project_registration",
            "exact_single_project_and_scope",
            "immutable_configuration_content",
            "exact_supported_owner_rule",
            "explicit_local_advisory_instruction",
            "immutable_prior_replacement_basis",
            "separate_validation_and_permission",
            "confirmed_activation_cutover",
            "exact_conflict_refusal_and_historical_replay",
            "no_circular_candidate_authority",
        ],
    )
}

pub fn freshness_admission_rule_descriptor_v1(
) -> Result<GovernanceRuleDescriptorV1, GovernanceRefusalV1> {
    shipped_rule_descriptor(
        ADMISSION_GOVERNANCE_OWNER_V1,
        FRESHNESS_ADMISSION_RULE_NAME_V1,
        Some(FRESHNESS_CANDIDATE_KIND_V1),
        &[
            "caller_supplied_max_observation_age_seconds_1_through_9007199254740991",
            "candidate_max_observation_age_seconds_not_exceed_configured_ceiling",
            "require_exact_source_revision_true",
            "future_skew_seconds_zero",
            "exact_single_binding",
            "exact_empty_governance_source_population",
        ],
    )
}

pub(crate) fn governance_canonical_bytes<T: Serialize>(
    value: &T,
) -> Result<Vec<u8>, GovernanceRefusalV1> {
    let bytes = canonical_bytes(value).map_err(|_| {
        GovernanceRefusalV1::new(
            "canonicalization",
            "governance content could not be canonically encoded",
        )
    })?;
    if bytes.len() > MAX_GOVERNANCE_RECORD_BYTES_V1 {
        return Err(GovernanceRefusalV1::new(
            "record_bound",
            "governance request or record exceeds one MiB",
        ));
    }
    Ok(bytes)
}

pub(crate) fn governance_fingerprint<T: Serialize>(
    domain: &str,
    value: &T,
) -> Result<FingerprintV1, GovernanceRefusalV1> {
    #[derive(Serialize)]
    struct Preimage<'a, T> {
        domain: &'a str,
        content: &'a T,
    }
    let bytes = governance_canonical_bytes(&Preimage {
        domain,
        content: value,
    })?;
    Ok(FingerprintV1::from_bytes(&bytes))
}

/// Decode a bounded complete document into a closed typed record. The owner's
/// record types all deny unknown fields; derive rejects duplicate known keys at
/// every nesting level. This helper is deliberately not a general Value parser.
pub(crate) fn decode_governance_json<T: DeserializeOwned>(
    bytes: &[u8],
) -> Result<T, GovernanceRefusalV1> {
    if bytes.len() > MAX_GOVERNANCE_RECORD_BYTES_V1 {
        return Err(GovernanceRefusalV1::new(
            "record_bound",
            "governance request or record exceeds one MiB",
        ));
    }
    serde_json::from_slice(bytes).map_err(|_| GovernanceRefusalV1::new("closed_json", "governance requires one complete typed JSON document with no unknown or duplicate fields"))
}

fn request_fingerprint(
    project: &ProjectBindingV1,
    input: &GovernanceRequestInputV1,
) -> Result<FingerprintV1, GovernanceRefusalV1> {
    #[derive(Serialize)]
    struct RequestPreimage<'a> {
        project: &'a ProjectBindingV1,
        request: &'a GovernanceRequestInputV1,
    }
    governance_fingerprint(
        "handbook.project-admission-governance.request@1.0.0",
        &RequestPreimage {
            project,
            request: input,
        },
    )
}

fn validate_project_binding(project: &ProjectBindingV1) -> Result<(), GovernanceRefusalV1> {
    if FingerprintV1::parse(&project.repository_identity_fingerprint).is_err()
        || FingerprintV1::parse(&project.physical_root_fingerprint).is_err()
        || !bounded_nonempty_text(&project.retained_root, MAX_TEXT_BYTES)
        || !Path::new(&project.retained_root).is_absolute()
    {
        return Err(GovernanceRefusalV1::new("project_binding", "exact Engine and physical-root fingerprints and an absolute retained root locator are required"));
    }
    Ok(())
}

fn bounded_nonempty_text(text: &str, max_bytes: usize) -> bool {
    !text.trim().is_empty() && text.len() <= max_bytes && !text.contains('\0')
}

fn shipped_rule_descriptor(
    owner: &str,
    name: &str,
    candidate_kind: Option<&str>,
    constraints: &[&str],
) -> Result<GovernanceRuleDescriptorV1, GovernanceRefusalV1> {
    #[derive(Serialize)]
    struct RuleContent<'a> {
        owner: &'a str,
        name: &'a str,
        version: &'a str,
        candidate_kind: Option<&'a str>,
        candidate_schema_ref: Option<&'a str>,
        required_authored_sources: Vec<String>,
        approval_sources: Vec<String>,
        exception_sources: Vec<String>,
        constraints: &'a [&'a str],
    }
    let content = RuleContent {
        owner,
        name,
        version: GOVERNANCE_RULE_VERSION_V1,
        candidate_kind,
        candidate_schema_ref: candidate_kind
            .map(|_| crate::EVIDENCE_FRESHNESS_POLICY_SCHEMA_REF_V1),
        required_authored_sources: vec![],
        approval_sources: vec![],
        exception_sources: vec![],
        constraints,
    };
    let reference =
        ExactRefV1::parse(&format!("{name}@{GOVERNANCE_RULE_VERSION_V1}")).map_err(|_| {
            GovernanceRefusalV1::new("owner_rule", "shipped governance rule reference is invalid")
        })?;
    let fingerprint = governance_fingerprint(
        "handbook.project-admission-governance.shipped-rule@1.0.0",
        &content,
    )?;
    Ok(GovernanceRuleDescriptorV1 {
        owner: owner.into(),
        name: name.into(),
        version: GOVERNANCE_RULE_VERSION_V1.into(),
        candidate_kind: candidate_kind.map(String::from),
        candidate_schema_ref: content.candidate_schema_ref.map(String::from),
        required_authored_sources: vec![],
        approval_sources: vec![],
        exception_sources: vec![],
        constraints: constraints.iter().map(|item| (*item).into()).collect(),
        binding: ExactBindingV1::new(reference, fingerprint),
    })
}
