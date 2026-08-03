use crate::definition_identity::{
    fingerprint_serializable, parse_definition_yaml, DefinitionFingerprint, ExactDefinitionRef,
    RegistryLoadError, RegistryLoadErrorKind, SourceByteBudget,
};
use crate::stable_role_registry::read_trusted_repo_source;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct SelectorGrammar {
    grammar_id: String,
    encoding: String,
    min_bytes: u16,
    max_bytes: u16,
    min_segments: u8,
    max_segments: u8,
    separator: String,
    normal_segment_character_class: String,
    single_segment_wildcard: String,
    recursive_wildcard: String,
    recursive_position: String,
    disallowed: Vec<String>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Matcher {
    schema_id: String,
    schema_version: String,
    matcher_id: String,
    matcher_version: String,
    target_kinds: Vec<String>,
    selector_grammar: SelectorGrammar,
    case_mode: String,
    deny_precedence: bool,
    extensions: BTreeMap<String, Value>,
    #[serde(skip_serializing)]
    definition_fingerprint: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Escalation {
    schema_id: String,
    schema_version: String,
    policy_id: String,
    policy_version: String,
    trigger_classes: Vec<String>,
    proposal_relation: String,
    required_request_bindings: Vec<String>,
    terminal_outcomes: Vec<String>,
    terminal_cardinality: String,
    preapproval_effect: String,
    extensions: BTreeMap<String, Value>,
    #[serde(skip_serializing)]
    policy_fingerprint: String,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Promotion {
    schema_id: String,
    schema_version: String,
    policy_id: String,
    policy_version: String,
    source_requirement: String,
    target_authority: String,
    horizon_relation: String,
    write_precondition: String,
    required_request_bindings: Vec<String>,
    terminal_outcomes: Vec<String>,
    terminal_cardinality: String,
    forbidden_authorities: Vec<String>,
    extensions: BTreeMap<String, Value>,
    #[serde(skip_serializing)]
    policy_fingerprint: String,
}
#[derive(Clone, Debug, Default)]
pub struct ContextResolutionPolicyRegistry {
    policies: BTreeMap<ExactDefinitionRef, DefinitionFingerprint>,
}
impl ContextResolutionPolicyRegistry {
    pub fn load(repo: impl AsRef<Path>, paths: &[String]) -> Result<Self, RegistryLoadError> {
        let mut b = SourceByteBudget::default();
        let mut policies = BTreeMap::new();
        for path in paths {
            let (_, bytes) = read_trusted_repo_source(repo.as_ref(), path, &mut b)?;
            let value = parse_definition_yaml(&bytes)?;
            let schema = value.get("schema_id").and_then(Value::as_str).unwrap_or("");
            let (r, f) = match schema {
                "handbook.mutation-matcher-definition" => resolve_matcher(decode(value)?)?,
                "handbook.resolution-escalation-policy-definition" => {
                    resolve_escalation(decode(value)?)?
                }
                "handbook.memory-promotion-policy-definition" => resolve_promotion(decode(value)?)?,
                _ => {
                    return Err(RegistryLoadError::new(
                        RegistryLoadErrorKind::UnsupportedRecord,
                        "unsupported Context Resolution policy record",
                    ))
                }
            };
            if policies.insert(r, f).is_some() {
                return Err(RegistryLoadError::new(
                    RegistryLoadErrorKind::DuplicateIdentity,
                    "Context Resolution policy identity is duplicated",
                ));
            }
        }
        Ok(Self { policies })
    }
    pub(crate) fn load_admitted(
        sources: &[(&ExactDefinitionRef, &[u8])],
    ) -> Result<Self, RegistryLoadError> {
        let mut policies = BTreeMap::new();
        for (declared_ref, bytes) in sources {
            let value = parse_definition_yaml(bytes)?;
            let schema = value.get("schema_id").and_then(Value::as_str).unwrap_or("");
            let (reference, fingerprint) = match schema {
                "handbook.mutation-matcher-definition" => resolve_matcher(decode(value)?)?,
                "handbook.resolution-escalation-policy-definition" => {
                    resolve_escalation(decode(value)?)?
                }
                "handbook.memory-promotion-policy-definition" => resolve_promotion(decode(value)?)?,
                _ => {
                    return Err(RegistryLoadError::new(
                        RegistryLoadErrorKind::UnsupportedRecord,
                        "unsupported Context Resolution policy record",
                    ))
                }
            };
            if &reference != *declared_ref {
                return Err(RegistryLoadError::new(
                    RegistryLoadErrorKind::ConflictingIdentity,
                    "Context Resolution producer derived exact ref does not match its typed source binding",
                ));
            }
            if policies.insert(reference, fingerprint).is_some() {
                return Err(RegistryLoadError::new(
                    RegistryLoadErrorKind::DuplicateIdentity,
                    "Context Resolution policy identity is duplicated",
                ));
            }
        }
        Ok(Self { policies })
    }
    pub fn fingerprint(&self, r: &ExactDefinitionRef) -> Option<&DefinitionFingerprint> {
        self.policies.get(r)
    }
    pub fn refs(&self) -> BTreeSet<ExactDefinitionRef> {
        self.policies.keys().cloned().collect()
    }
}

pub(crate) fn admitted_context_resolution_policy_exact_ref(
    bytes: &[u8],
) -> Result<ExactDefinitionRef, RegistryLoadError> {
    let value = parse_definition_yaml(bytes)?;
    match value.get("schema_id").and_then(Value::as_str).unwrap_or("") {
        "handbook.mutation-matcher-definition" => {
            let authored: Matcher = decode(value)?;
            if authored.schema_version != "1.0" {
                return Err(RegistryLoadError::new(
                    RegistryLoadErrorKind::UnsupportedRecord,
                    "unsupported Context Resolution matcher record",
                ));
            }
            ExactDefinitionRef::new(&authored.matcher_id, &authored.matcher_version)
        }
        "handbook.resolution-escalation-policy-definition" => {
            let authored: Escalation = decode(value)?;
            if authored.schema_version != "1.0" {
                return Err(RegistryLoadError::new(
                    RegistryLoadErrorKind::UnsupportedRecord,
                    "unsupported Context Resolution escalation record",
                ));
            }
            ExactDefinitionRef::new(&authored.policy_id, &authored.policy_version)
        }
        "handbook.memory-promotion-policy-definition" => {
            let authored: Promotion = decode(value)?;
            if authored.schema_version != "1.0" {
                return Err(RegistryLoadError::new(
                    RegistryLoadErrorKind::UnsupportedRecord,
                    "unsupported Context Resolution promotion record",
                ));
            }
            ExactDefinitionRef::new(&authored.policy_id, &authored.policy_version)
        }
        _ => Err(RegistryLoadError::new(
            RegistryLoadErrorKind::UnsupportedRecord,
            "unsupported Context Resolution policy record",
        )),
    }
}
fn decode<T: for<'de> Deserialize<'de>>(v: Value) -> Result<T, RegistryLoadError> {
    serde_json::from_value(v).map_err(|e| {
        RegistryLoadError::new(
            if e.to_string().contains("unknown field") {
                RegistryLoadErrorKind::UnknownField
            } else {
                RegistryLoadErrorKind::SyntaxError
            },
            "Context Resolution producer does not match its closed record",
        )
    })
}
fn finish<T: Serialize>(
    value: &T,
    supplied: &str,
    id: &str,
    version: &str,
    expected: Value,
) -> Result<(ExactDefinitionRef, DefinitionFingerprint), RegistryLoadError> {
    let actual = serde_json::to_value(value).map_err(|_| {
        RegistryLoadError::new(
            RegistryLoadErrorKind::SyntaxError,
            "producer serialization failed",
        )
    })?;
    if actual != expected {
        return Err(RegistryLoadError::new(
            RegistryLoadErrorKind::UnsupportedRecord,
            "Context Resolution producer differs from the exact shipped definition",
        ));
    }
    let r = ExactDefinitionRef::new(id, version)?;
    let supplied = DefinitionFingerprint::parse(supplied)?;
    let computed = fingerprint_serializable(value)?;
    if supplied != computed {
        return Err(RegistryLoadError::new(
            RegistryLoadErrorKind::FingerprintMismatch,
            "Context Resolution producer fingerprint mismatch",
        ));
    }
    Ok((r, computed))
}
fn resolve_matcher(
    v: Matcher,
) -> Result<(ExactDefinitionRef, DefinitionFingerprint), RegistryLoadError> {
    let expected = json!({"schema_id":"handbook.mutation-matcher-definition","schema_version":"1.0","matcher_id":"handbook.mutation-matcher.core","matcher_version":"1.0.0","target_kinds":["repository_path"],"selector_grammar":{"grammar_id":"normalized_repo_relative_glob_v1","encoding":"ascii","min_bytes":1,"max_bytes":1024,"min_segments":1,"max_segments":64,"separator":"/","normal_segment_character_class":"[A-Za-z0-9._*-]","single_segment_wildcard":"*","recursive_wildcard":"**","recursive_position":"terminal_segment_only","disallowed":["leading_slash","trailing_slash","empty_segment","dot_segment","dotdot_segment","backslash","nul","uri_prefix","adjacent_double_star_in_normal_segment"]},"case_mode":"sensitive","deny_precedence":true,"extensions":{}});
    finish(
        &v,
        &v.definition_fingerprint,
        &v.matcher_id,
        &v.matcher_version,
        expected,
    )
}
fn resolve_escalation(
    v: Escalation,
) -> Result<(ExactDefinitionRef, DefinitionFingerprint), RegistryLoadError> {
    let expected = json!({"schema_id":"handbook.resolution-escalation-policy-definition","schema_version":"1.0","policy_id":"handbook.resolution-escalation.core","policy_version":"1.0.0","trigger_classes":["dimension_rank_increase","mutation_allow_expansion","missing_context","missing_authority"],"proposal_relation":"same_profile_stack_strict_widening","required_request_bindings":["current_envelope_ref_fingerprint","proposed_envelope_ref_fingerprint","trigger_ref_fingerprint","missing_condition","requested_authority_ref","evidence_refs"],"terminal_outcomes":["approved","refused","superseded"],"terminal_cardinality":"exactly_one","preapproval_effect":"request_only_no_authority_change","extensions":{}});
    finish(
        &v,
        &v.policy_fingerprint,
        &v.policy_id,
        &v.policy_version,
        expected,
    )
}
fn resolve_promotion(
    v: Promotion,
) -> Result<(ExactDefinitionRef, DefinitionFingerprint), RegistryLoadError> {
    let expected = json!({"schema_id":"handbook.memory-promotion-policy-definition","schema_version":"1.0","policy_id":"handbook.memory-promotion.core","policy_version":"1.0.0","source_requirement":"nonempty_exact_ref_fingerprint_pairs","target_authority":"semantic_memory","horizon_relation":"strictly_higher_memory_rank","write_precondition":"expected_target_fingerprint_compare_and_write","required_request_bindings":["source_inputs","source_envelope_ref_fingerprint","target_memory_horizon","target_record_ref","expected_target_fingerprint","requested_authority_ref"],"terminal_outcomes":["applied","refused","stale"],"terminal_cardinality":"exactly_one","forbidden_authorities":["canonical_artifact","contract","posture"],"extensions":{}});
    finish(
        &v,
        &v.policy_fingerprint,
        &v.policy_id,
        &v.policy_version,
        expected,
    )
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct RefFingerprint {
    #[serde(rename = "ref")]
    reference: String,
    fingerprint: String,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct Defaults {
    scope_horizon: String,
    detail_resolution: String,
    temporal_horizon: String,
    authority_horizon: String,
    memory_horizon: String,
    validation_horizon: String,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct Level {
    level_id: String,
    display_label: String,
    defaults: Defaults,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct RankedValue {
    value_id: String,
    rank: u8,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct DimensionDomains {
    scope_horizon: Vec<RankedValue>,
    detail_resolution: Vec<RankedValue>,
    temporal_horizon: Vec<RankedValue>,
    authority_horizon: Vec<RankedValue>,
    memory_horizon: Vec<RankedValue>,
    validation_horizon: Vec<RankedValue>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct AuthoredStack {
    schema_id: String,
    schema_version: String,
    stack_id: String,
    stack_version: String,
    levels: Vec<Level>,
    dimension_domains: DimensionDomains,
    mutation_matcher: RefFingerprint,
    escalation_policy: RefFingerprint,
    memory_promotion_policy: RefFingerprint,
    extensions: BTreeMap<String, Value>,
    #[serde(skip_serializing)]
    definition_fingerprint: String,
}
#[derive(Clone, Debug)]
pub struct ContextResolutionStackDefinition {
    exact_ref: ExactDefinitionRef,
    definition_fingerprint: DefinitionFingerprint,
    levels: Vec<Level>,
    dimension_domains: DimensionDomains,
}
impl ContextResolutionStackDefinition {
    pub fn exact_ref(&self) -> &ExactDefinitionRef {
        &self.exact_ref
    }
    pub fn definition_fingerprint(&self) -> &DefinitionFingerprint {
        &self.definition_fingerprint
    }
    pub fn load(
        repo: impl AsRef<Path>,
        path: &str,
        policies: &ContextResolutionPolicyRegistry,
    ) -> Result<Self, RegistryLoadError> {
        let mut b = SourceByteBudget::default();
        let (_, bytes) = read_trusted_repo_source(repo.as_ref(), path, &mut b)?;
        Self::load_bytes(&bytes, policies)
    }
    pub(crate) fn load_bytes(
        bytes: &[u8],
        policies: &ContextResolutionPolicyRegistry,
    ) -> Result<Self, RegistryLoadError> {
        let value = parse_definition_yaml(bytes)?;
        let authored: AuthoredStack = decode(value)?;
        authored.resolve(policies)
    }

    #[rustfmt::skip]
    pub(crate) fn ranks(&self, level_id: &str, values: [&str; 6]) -> Option<[u8; 6]> {
        self.levels.iter().any(|level| level.level_id == level_id).then_some(())?;
        let domains = [&self.dimension_domains.scope_horizon, &self.dimension_domains.detail_resolution, &self.dimension_domains.temporal_horizon, &self.dimension_domains.authority_horizon, &self.dimension_domains.memory_horizon, &self.dimension_domains.validation_horizon];
        let mut ranks = [0; 6];
        for (index, (domain, value_id)) in domains.into_iter().zip(values).enumerate() {
            ranks[index] = domain.iter().find(|value| value.value_id == value_id)?.rank;
        }
        Some(ranks)
    }
}
pub(crate) fn admitted_context_resolution_stack_exact_ref(
    bytes: &[u8],
) -> Result<ExactDefinitionRef, RegistryLoadError> {
    let value = parse_definition_yaml(bytes)?;
    let authored: AuthoredStack = decode(value)?;
    if authored.schema_id != "handbook.context-resolution-stack-definition"
        || authored.schema_version != "1.0"
    {
        return Err(RegistryLoadError::new(
            RegistryLoadErrorKind::UnsupportedRecord,
            "unsupported Context Resolution stack record",
        ));
    }
    ExactDefinitionRef::new(&authored.stack_id, &authored.stack_version)
}
impl AuthoredStack {
    fn resolve(
        self,
        policies: &ContextResolutionPolicyRegistry,
    ) -> Result<ContextResolutionStackDefinition, RegistryLoadError> {
        if self.schema_id != "handbook.context-resolution-stack-definition"
            || self.schema_version != "1.0"
            || !self.extensions.is_empty()
        {
            return Err(RegistryLoadError::new(
                RegistryLoadErrorKind::UnsupportedRecord,
                "unsupported Context Resolution stack record",
            ));
        }
        if self.levels.is_empty() || self.levels.len() > 64 {
            return Err(stack_drift());
        }
        let mut level_ids = BTreeSet::new();
        for level in &self.levels {
            if level.level_id.is_empty()
                || level.display_label.is_empty()
                || !level_ids.insert(level.level_id.as_str())
            {
                return Err(stack_drift());
            }
        }
        let domains = [
            &self.dimension_domains.scope_horizon,
            &self.dimension_domains.detail_resolution,
            &self.dimension_domains.temporal_horizon,
            &self.dimension_domains.authority_horizon,
            &self.dimension_domains.memory_horizon,
            &self.dimension_domains.validation_horizon,
        ];
        for domain in domains {
            let mut ids = BTreeSet::new();
            if domain.is_empty()
                || domain.len() > 256
                || domain.iter().enumerate().any(|(rank, value)| {
                    value.value_id.is_empty()
                        || !ids.insert(value.value_id.as_str())
                        || usize::from(value.rank) != rank
                })
            {
                return Err(stack_drift());
            }
        }
        let ranks = |level: &Level| {
            let values = [
                level.defaults.scope_horizon.as_str(),
                level.defaults.detail_resolution.as_str(),
                level.defaults.temporal_horizon.as_str(),
                level.defaults.authority_horizon.as_str(),
                level.defaults.memory_horizon.as_str(),
                level.defaults.validation_horizon.as_str(),
            ];
            let mut resolved = [0u8; 6];
            for (index, (domain, value)) in domains.iter().zip(values).enumerate() {
                resolved[index] = domain
                    .iter()
                    .find(|entry| entry.value_id == value)
                    .map(|entry| entry.rank)
                    .ok_or_else(stack_drift)?;
            }
            Ok::<_, RegistryLoadError>(resolved)
        };
        let level_ranks = self
            .levels
            .iter()
            .map(ranks)
            .collect::<Result<Vec<_>, _>>()?;
        if level_ranks
            .windows(2)
            .any(|pair| (0..6).any(|index| pair[1][index] > pair[0][index]))
        {
            return Err(stack_drift());
        }
        let selections = [
            (
                &self.mutation_matcher,
                "handbook.mutation-matcher.core@1.0.0",
            ),
            (
                &self.escalation_policy,
                "handbook.resolution-escalation.core@1.0.0",
            ),
            (
                &self.memory_promotion_policy,
                "handbook.memory-promotion.core@1.0.0",
            ),
        ];
        let mut fingerprints = Vec::new();
        for (selection, expected) in selections {
            if selection.reference != expected {
                return Err(stack_drift());
            }
            let r = ExactDefinitionRef::parse(&selection.reference)?;
            let f = policies.fingerprint(&r).ok_or_else(|| {
                RegistryLoadError::new(
                    RegistryLoadErrorKind::UnsupportedDependency,
                    "Context Resolution stack policy is absent",
                )
            })?;
            if selection.fingerprint != f.as_str() {
                return Err(RegistryLoadError::new(
                    RegistryLoadErrorKind::FingerprintMismatch,
                    "Context Resolution stack policy fingerprint mismatch",
                ));
            }
            fingerprints.push(f.as_str());
        }
        let exact_ref = ExactDefinitionRef::new(&self.stack_id, &self.stack_version)?;
        let supplied = DefinitionFingerprint::parse(&self.definition_fingerprint)?;
        let computed = fingerprint_serializable(&StackClosure {
            definition: &self,
            policy_fingerprints: fingerprints,
        })?;
        if supplied != computed {
            return Err(RegistryLoadError::new(
                RegistryLoadErrorKind::FingerprintMismatch,
                "Context Resolution stack fingerprint mismatch",
            ));
        }
        Ok(ContextResolutionStackDefinition {
            exact_ref,
            definition_fingerprint: computed,
            levels: self.levels,
            dimension_domains: self.dimension_domains,
        })
    }
}
#[derive(Serialize)]
struct StackClosure<'a> {
    definition: &'a AuthoredStack,
    policy_fingerprints: Vec<&'a str>,
}
fn stack_drift() -> RegistryLoadError {
    RegistryLoadError::new(
        RegistryLoadErrorKind::UnsupportedRecord,
        "Context Resolution stack differs from the exact shipped definition",
    )
}
