use crate::context_resolution::ProjectionAuthorityView;
use crate::definition_identity::{canonical_json_bytes, DefinitionFingerprint, RegistryLoadError};
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use std::cell::Cell;
use std::collections::{BTreeMap, BTreeSet};

const DECISION_INPUT_FIELDS: [&str; 11] = [
    "source_kind",
    "source_schema_ref",
    "source_schema_fingerprint",
    "source_pointer",
    "source_pointer_schema",
    "derivation_ref",
    "derivation_fingerprint",
    "target_schema_ref",
    "target_schema_fingerprint",
    "target_pointer",
    "target_pointer_schema",
];
const SUPPORT_REASON_ORDER: [SupportReason; 9] = [
    SupportReason::SourceKindUnsupported,
    SupportReason::SourceSchemaUnregistered,
    SupportReason::SourcePointerMissing,
    SupportReason::SourcePointerTypeIncompatible,
    SupportReason::TargetSchemaUnregistered,
    SupportReason::TargetPointerMissing,
    SupportReason::TargetPointerTypeIncompatible,
    SupportReason::DerivationUnregistered,
    SupportReason::DerivationIoIncompatible,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ProjectionErrorKind {
    InvalidJson,
    NonCanonicalJson,
    InvalidPair,
    StaleBinding,
    DefinitionUnlisted,
    InvalidProfile,
    InvalidVocabulary,
    InvalidDefinition,
    InvalidPolicy,
    InvalidEvaluator,
    ForbiddenDefinition,
    Cardinality,
    UnsupportedOperation,
    UnsupportedSurface,
    InvalidPurpose,
    InvalidCurrentness,
    ResolutionEscalationRequired,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ProjectionError {
    kind: ProjectionErrorKind,
    detail: String,
}

impl ProjectionError {
    fn new(kind: ProjectionErrorKind, detail: impl Into<String>) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }
}

type ProjectionResultValue<T> = Result<T, ProjectionError>;

#[derive(Clone, Debug, Default, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
struct ExactPair {
    #[serde(rename = "ref")]
    reference: String,
    fingerprint: String,
}

impl ExactPair {
    fn new(reference: &str, fingerprint: &str) -> ProjectionResultValue<Self> {
        if reference.is_empty()
            || reference.len() > 1024
            || reference
                .bytes()
                .any(|byte| byte.is_ascii_control() || byte.is_ascii_whitespace())
        {
            return Err(ProjectionError::new(
                ProjectionErrorKind::InvalidPair,
                "exact reference is invalid",
            ));
        }
        DefinitionFingerprint::parse(fingerprint).map_err(|_| {
            ProjectionError::new(
                ProjectionErrorKind::InvalidPair,
                "exact fingerprint is invalid",
            )
        })?;
        Ok(Self {
            reference: reference.to_owned(),
            fingerprint: fingerprint.to_owned(),
        })
    }
}

impl From<&crate::context_resolution::ContextResolutionExactBinding> for ExactPair {
    fn from(value: &crate::context_resolution::ContextResolutionExactBinding) -> Self {
        Self {
            reference: value.reference().to_owned(),
            fingerprint: value.fingerprint().to_owned(),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct AuthoredProfile {
    schema_id: String,
    schema_version: String,
    profile_id: String,
    profile_version: String,
    resolved_profile: ExactPair,
    configured_kinds: Vec<ConfiguredKind>,
    projection_catalog: Vec<ExactPair>,
    purposes: Vec<String>,
    resolution_stack: ExactPair,
    surfaces: Vec<String>,
    vocabulary: ExactPair,
    profile_fingerprint: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ConfiguredKind {
    kind_ref: String,
    source_kind: String,
    capability: ExactPair,
    source_schema: ExactPair,
}

#[derive(Clone, Debug)]
struct ValidatedProfile {
    exact_pair: ExactPair,
    configuration_pair: ExactPair,
    configured_kinds: Vec<ConfiguredKind>,
    projection_catalog: Vec<ExactPair>,
    purposes: Vec<String>,
    resolution_stack: ExactPair,
    surfaces: Vec<String>,
    vocabulary: ExactPair,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct AuthoredVocabulary {
    schema_id: String,
    schema_version: String,
    vocabulary_id: String,
    vocabulary_version: String,
    registered_custom_kinds: Vec<String>,
    vocabulary_fingerprint: String,
}

#[derive(Clone, Debug)]
struct ValidatedVocabulary {
    exact_pair: ExactPair,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct AuthoredDefinition {
    projection_definition_id: String,
    projection_definition_version: String,
    source_selectors: Vec<SourceSelector>,
    allowed_surfaces: Vec<String>,
    allowed_operations: Vec<ProjectionOperation>,
    target_schema: TargetSchema,
    disclosure_policy: DisclosurePolicy,
    support_evaluator: SupportEvaluator,
    currentness_requirements: CurrentnessRequirements,
    #[serde(default)]
    source_pair_requirements: Vec<SourcePairRequirement>,
    field_rules: Vec<FieldRule>,
    derivations: Vec<DerivationDefinition>,
    definition_fingerprint: String,
}

#[derive(Clone, Debug)]
struct ValidatedDefinition {
    exact_pair: ExactPair,
    source_selectors: Vec<SourceSelector>,
    allowed_surfaces: Vec<String>,
    allowed_operations: Vec<ProjectionOperation>,
    target_schema: TargetSchema,
    disclosure_policy: DisclosurePolicy,
    support_evaluator: SupportEvaluator,
    currentness_requirements: CurrentnessRequirements,
    source_pair_requirements: Vec<SourcePairRequirement>,
    field_rules: Vec<FieldRule>,
    derivations: Vec<DerivationDefinition>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct SourceSelector {
    selector_id: String,
    source_kind: String,
    configured_kind_ref: String,
    capability: ExactPair,
    source_schema: ExactPair,
    cardinality: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct SourcePairRequirement {
    current_selector_id: String,
    derived_selector_id: String,
    dependency_role: String,
    require_state_identity: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct TargetSchema {
    #[serde(rename = "ref")]
    reference: String,
    fingerprint: String,
    pointer_types: BTreeMap<String, String>,
    optional_pointers: Vec<String>,
}

impl TargetSchema {
    fn exact_pair(&self) -> ExactPair {
        ExactPair {
            reference: self.reference.clone(),
            fingerprint: self.fingerprint.clone(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct DisclosurePolicy {
    policy_id: String,
    policy_version: String,
    classification_registry: ClassificationRegistry,
    matcher_definition: ExactPair,
    unmatched_action: PolicyAction,
    indeterminate_match_action: String,
    overlap_precedence: String,
    rules: Vec<DisclosureRule>,
    policy_fingerprint: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct ClassificationRegistry {
    #[serde(rename = "ref")]
    reference: String,
    fingerprint: String,
    values: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct DisclosureRule {
    rule_id: String,
    disclosure_classifications: Vec<String>,
    source_kinds: Vec<String>,
    source_pointer_selector: String,
    action: PolicyAction,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
enum PolicyAction {
    Allow,
    Redact,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct SupportEvaluator {
    support_evaluator_id: String,
    support_evaluator_version: String,
    schema_registry_contract: ExactPair,
    pointer_semantics: ExactPair,
    derivation_compatibility: ExactPair,
    supported_source_kinds: Vec<String>,
    decision_input_fields: Vec<String>,
    unsupported_reason_precedence: Vec<SupportReason>,
    evaluator_fingerprint: String,
    #[serde(skip)]
    exact_pair: ExactPair,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
enum SupportReason {
    SourceKindUnsupported,
    SourceSchemaUnregistered,
    SourcePointerMissing,
    SourcePointerTypeIncompatible,
    TargetSchemaUnregistered,
    TargetPointerMissing,
    TargetPointerTypeIncompatible,
    DerivationUnregistered,
    DerivationIoIncompatible,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct CurrentnessRequirements {
    mode: CurrentnessMode,
    revision_basis: Option<String>,
    families: Vec<CurrentnessFamilyRequirement>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct CurrentnessFamilyRequirement {
    selector_id: String,
    family: String,
    adapter: ExactPair,
    slots: Vec<String>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
enum CurrentnessMode {
    None,
    ExactRevisionCheck,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct FieldRule {
    rule_id: String,
    operation: ProjectionOperation,
    source_selector_id: String,
    source_pointer: String,
    source_pointer_schema: String,
    target_pointer: String,
    target_pointer_schema: String,
    minimum_resolution: MinimumResolution,
    disclosure_classification: String,
    required_for_result: bool,
    claim_refs: Vec<String>,
    derivation: Option<ExactPair>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct MinimumResolution {
    scope_horizon: u8,
    detail_resolution: u8,
    temporal_horizon: u8,
    authority_horizon: u8,
    memory_horizon: u8,
    validation_horizon: u8,
}

impl MinimumResolution {
    fn ranks(&self) -> [u8; 6] {
        [
            self.scope_horizon,
            self.detail_resolution,
            self.temporal_horizon,
            self.authority_horizon,
            self.memory_horizon,
            self.validation_horizon,
        ]
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct DerivationDefinition {
    #[serde(rename = "ref")]
    reference: String,
    fingerprint: String,
    algorithm: DerivationAlgorithm,
    input_type: String,
    output_type: String,
    lossy: bool,
}

impl DerivationDefinition {
    fn exact_pair(&self) -> ExactPair {
        ExactPair {
            reference: self.reference.clone(),
            fingerprint: self.fingerprint.clone(),
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
enum DerivationAlgorithm {
    CountArray,
    NormalizedSummary,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
enum ProjectionOperation {
    Reveal,
    Derive,
    SynthesizeCandidate,
}

#[derive(Clone, Debug)]
struct ProjectionConfiguration {
    profile: ValidatedProfile,
    vocabulary: ValidatedVocabulary,
    definition: ValidatedDefinition,
}

impl ProjectionConfiguration {
    fn load(
        profile_bytes: &[u8],
        vocabulary_bytes: &[u8],
        definition_bytes: &[u8],
    ) -> ProjectionResultValue<Self> {
        let profile_value = strict_jcs(profile_bytes, "resolved profile")?;
        let vocabulary_value = strict_jcs(vocabulary_bytes, "vocabulary")?;
        let definition_value = strict_jcs(definition_bytes, "Projection definition")?;
        if contains_forbidden_definition_content(&definition_value) {
            return Err(ProjectionError::new(
                ProjectionErrorKind::ForbiddenDefinition,
                "Projection definition contains forbidden executable, remote, prompt, command, transport, or synthesis content",
            ));
        }

        let authored_profile: AuthoredProfile = serde_json::from_value(profile_value.clone())
            .map_err(|error| {
                ProjectionError::new(
                    ProjectionErrorKind::InvalidProfile,
                    format!("resolved profile is not exact: {error}"),
                )
            })?;
        let authored_vocabulary: AuthoredVocabulary =
            serde_json::from_value(vocabulary_value.clone()).map_err(|error| {
                ProjectionError::new(
                    ProjectionErrorKind::InvalidVocabulary,
                    format!("vocabulary is not exact: {error}"),
                )
            })?;
        let mut authored_definition: AuthoredDefinition =
            serde_json::from_value(definition_value.clone()).map_err(|error| {
                ProjectionError::new(
                    ProjectionErrorKind::InvalidDefinition,
                    format!("Projection definition is not exact: {error}"),
                )
            })?;

        let profile_fingerprint = require_fingerprint(
            &profile_value,
            "profile_fingerprint",
            &authored_profile.profile_fingerprint,
            ProjectionErrorKind::InvalidProfile,
        )?;
        let vocabulary_fingerprint = require_fingerprint(
            &vocabulary_value,
            "vocabulary_fingerprint",
            &authored_vocabulary.vocabulary_fingerprint,
            ProjectionErrorKind::InvalidVocabulary,
        )?;
        require_fingerprint(
            definition_value.get("disclosure_policy").ok_or_else(|| {
                ProjectionError::new(
                    ProjectionErrorKind::InvalidPolicy,
                    "disclosure policy is absent",
                )
            })?,
            "policy_fingerprint",
            &authored_definition.disclosure_policy.policy_fingerprint,
            ProjectionErrorKind::InvalidPolicy,
        )?;
        let evaluator_fingerprint = require_fingerprint(
            definition_value.get("support_evaluator").ok_or_else(|| {
                ProjectionError::new(
                    ProjectionErrorKind::InvalidEvaluator,
                    "support evaluator is absent",
                )
            })?,
            "evaluator_fingerprint",
            &authored_definition.support_evaluator.evaluator_fingerprint,
            ProjectionErrorKind::InvalidEvaluator,
        )?;
        let definition_fingerprint = require_fingerprint(
            &definition_value,
            "definition_fingerprint",
            &authored_definition.definition_fingerprint,
            ProjectionErrorKind::InvalidDefinition,
        )?;

        let profile_configuration_pair = ExactPair::new(
            &format!(
                "{}@{}",
                authored_profile.profile_id, authored_profile.profile_version
            ),
            profile_fingerprint.as_str(),
        )?;
        let resolved_profile_pair = ExactPair::new(
            &authored_profile.resolved_profile.reference,
            &authored_profile.resolved_profile.fingerprint,
        )?;
        let vocabulary_pair = ExactPair::new(
            &format!(
                "{}@{}",
                authored_vocabulary.vocabulary_id, authored_vocabulary.vocabulary_version
            ),
            vocabulary_fingerprint.as_str(),
        )?;
        let definition_pair = ExactPair::new(
            &format!(
                "{}@{}",
                authored_definition.projection_definition_id,
                authored_definition.projection_definition_version
            ),
            definition_fingerprint.as_str(),
        )?;
        authored_definition.support_evaluator.exact_pair = ExactPair::new(
            &format!(
                "{}@{}",
                authored_definition.support_evaluator.support_evaluator_id,
                authored_definition
                    .support_evaluator
                    .support_evaluator_version
            ),
            evaluator_fingerprint.as_str(),
        )?;

        validate_profile(&authored_profile)?;
        validate_vocabulary(&authored_vocabulary)?;
        validate_policy(&authored_definition.disclosure_policy)?;
        validate_evaluator(&authored_definition.support_evaluator)?;
        validate_definition(&authored_definition)?;
        validate_trusted_semantic_dependencies(&authored_profile, &authored_definition)?;

        if authored_profile.vocabulary != vocabulary_pair {
            return Err(ProjectionError::new(
                ProjectionErrorKind::StaleBinding,
                "profile vocabulary pair is stale",
            ));
        }
        if !authored_profile
            .projection_catalog
            .contains(&definition_pair)
        {
            return Err(ProjectionError::new(
                ProjectionErrorKind::DefinitionUnlisted,
                "Projection definition is not listed by the exact profile",
            ));
        }
        for selector in &authored_definition.source_selectors {
            let kind = authored_profile
                .configured_kinds
                .iter()
                .find(|kind| kind.kind_ref == selector.configured_kind_ref)
                .ok_or_else(|| {
                    ProjectionError::new(
                        ProjectionErrorKind::InvalidProfile,
                        "configured custom kind is absent from the exact profile",
                    )
                })?;
            if !authored_vocabulary
                .registered_custom_kinds
                .contains(&selector.configured_kind_ref)
                || kind.source_kind != selector.source_kind
                || kind.capability != selector.capability
                || kind.source_schema != selector.source_schema
            {
                return Err(ProjectionError::new(
                    ProjectionErrorKind::StaleBinding,
                    "custom kind capability or schema closure is incompatible",
                ));
            }
        }

        Ok(Self {
            profile: ValidatedProfile {
                exact_pair: resolved_profile_pair,
                configuration_pair: profile_configuration_pair,
                configured_kinds: authored_profile.configured_kinds,
                projection_catalog: authored_profile.projection_catalog,
                purposes: authored_profile.purposes,
                resolution_stack: authored_profile.resolution_stack,
                surfaces: authored_profile.surfaces,
                vocabulary: authored_profile.vocabulary,
            },
            vocabulary: ValidatedVocabulary {
                exact_pair: vocabulary_pair,
            },
            definition: ValidatedDefinition {
                exact_pair: definition_pair,
                source_selectors: authored_definition.source_selectors,
                allowed_surfaces: authored_definition.allowed_surfaces,
                allowed_operations: authored_definition.allowed_operations,
                target_schema: authored_definition.target_schema,
                disclosure_policy: authored_definition.disclosure_policy,
                support_evaluator: authored_definition.support_evaluator,
                currentness_requirements: authored_definition.currentness_requirements,
                source_pair_requirements: authored_definition.source_pair_requirements,
                field_rules: authored_definition.field_rules,
                derivations: authored_definition.derivations,
            },
        })
    }

    #[cfg(test)]
    fn only_rule(&self, rule_id: &str) -> ProjectionResultValue<Self> {
        let mut changed = self.clone();
        changed
            .definition
            .field_rules
            .retain(|rule| rule.rule_id == rule_id);
        if changed.definition.field_rules.len() != 1 {
            return Err(ProjectionError::new(
                ProjectionErrorKind::InvalidDefinition,
                "requested fixture rule is absent",
            ));
        }
        changed.rebind_test_closure()?;
        Ok(changed)
    }

    #[cfg(test)]
    fn with_exact_currentness(
        &self,
        selector_id: &str,
        family: &str,
        adapter: &str,
        slots: &[&str],
    ) -> ProjectionResultValue<Self> {
        let mut changed = self.clone();
        let selector = changed
            .definition
            .source_selectors
            .iter_mut()
            .find(|selector| selector.selector_id == selector_id)
            .ok_or_else(|| {
                ProjectionError::new(
                    ProjectionErrorKind::InvalidCurrentness,
                    "exact-currentness fixture selector is absent",
                )
            })?;
        selector.source_kind = "snapshot".to_owned();
        let configured_kind = changed
            .profile
            .configured_kinds
            .iter_mut()
            .find(|kind| kind.kind_ref == selector.configured_kind_ref)
            .expect("validated configured kind");
        configured_kind.source_kind = "snapshot".to_owned();
        changed.definition.currentness_requirements = CurrentnessRequirements {
            mode: CurrentnessMode::ExactRevisionCheck,
            revision_basis: Some("captured_revision".to_owned()),
            families: vec![CurrentnessFamilyRequirement {
                selector_id: selector_id.to_owned(),
                family: family.to_owned(),
                adapter: ExactPair::new(
                    adapter,
                    DefinitionFingerprint::from_bytes(adapter.as_bytes()).as_str(),
                )?,
                slots: slots.iter().map(|slot| (*slot).to_owned()).collect(),
            }],
        };
        changed.rebind_test_closure()?;
        Ok(changed)
    }

    #[cfg(test)]
    fn with_rule_source_schema(
        &self,
        rule_id: &str,
        source_schema: &str,
    ) -> ProjectionResultValue<Self> {
        let mut changed = self.clone();
        let rule = changed
            .definition
            .field_rules
            .iter_mut()
            .find(|rule| rule.rule_id == rule_id)
            .ok_or_else(|| {
                ProjectionError::new(
                    ProjectionErrorKind::InvalidDefinition,
                    "fixture rule is absent",
                )
            })?;
        rule.source_pointer_schema = source_schema.to_owned();
        changed.rebind_test_closure()?;
        Ok(changed)
    }

    #[cfg(test)]
    fn with_evaluator_reason_order_reversed(&self) -> ProjectionResultValue<Self> {
        let mut changed = self.clone();
        changed
            .definition
            .support_evaluator
            .unsupported_reason_precedence
            .reverse();
        changed.rebind_test_closure()?;
        Ok(changed)
    }

    #[cfg(test)]
    fn rebind_test_closure(&mut self) -> ProjectionResultValue<()> {
        let evaluator_value =
            serde_json::to_value(&self.definition.support_evaluator).map_err(|_| {
                ProjectionError::new(
                    ProjectionErrorKind::InvalidEvaluator,
                    "evaluator cannot be fingerprinted",
                )
            })?;
        let evaluator_fingerprint =
            fingerprint_without_field(&evaluator_value, "evaluator_fingerprint")?;
        self.definition.support_evaluator.evaluator_fingerprint = evaluator_fingerprint.to_string();
        self.definition.support_evaluator.exact_pair.fingerprint =
            evaluator_fingerprint.to_string();
        let definition_fingerprint = fingerprint_validated_definition(&self.definition)?;
        self.definition.exact_pair.fingerprint = definition_fingerprint.to_string();
        for pair in &mut self.profile.projection_catalog {
            if pair.reference == self.definition.exact_pair.reference {
                pair.fingerprint = definition_fingerprint.to_string();
            }
        }
        let profile_fingerprint = fingerprint_validated_profile(&self.profile)?;
        self.profile.configuration_pair.fingerprint = profile_fingerprint.to_string();
        Ok(())
    }
}

fn validate_profile(profile: &AuthoredProfile) -> ProjectionResultValue<()> {
    if profile.schema_id != "handbook.private-resolved-projection-profile"
        || profile.schema_version != "1.0"
        || profile.configured_kinds.is_empty()
        || profile.projection_catalog.is_empty()
        || profile.surfaces.is_empty()
        || profile.purposes.is_empty()
    {
        return Err(ProjectionError::new(
            ProjectionErrorKind::InvalidProfile,
            "resolved profile closure is incomplete",
        ));
    }
    require_unique(
        profile
            .configured_kinds
            .iter()
            .map(|kind| kind.kind_ref.as_str()),
        "custom kind",
    )?;
    require_unique(
        profile
            .projection_catalog
            .iter()
            .map(|pair| pair.reference.as_str()),
        "projection catalog",
    )?;
    require_unique(
        profile.surfaces.iter().map(String::as_str),
        "profile surface",
    )?;
    require_unique(
        profile.purposes.iter().map(String::as_str),
        "profile purpose",
    )?;
    validate_pair(&profile.resolution_stack)?;
    validate_pair(&profile.vocabulary)?;
    for pair in &profile.projection_catalog {
        validate_pair(pair)?;
    }
    for kind in &profile.configured_kinds {
        validate_pair(&kind.capability)?;
        validate_pair(&kind.source_schema)?;
    }
    Ok(())
}

fn validate_vocabulary(vocabulary: &AuthoredVocabulary) -> ProjectionResultValue<()> {
    if vocabulary.schema_id != "handbook.private-projection-vocabulary"
        || vocabulary.schema_version != "1.0"
        || vocabulary.registered_custom_kinds.is_empty()
    {
        return Err(ProjectionError::new(
            ProjectionErrorKind::InvalidVocabulary,
            "projection vocabulary closure is incomplete",
        ));
    }
    require_unique(
        vocabulary
            .registered_custom_kinds
            .iter()
            .map(String::as_str),
        "registered custom kind",
    )
}

fn validate_policy(policy: &DisclosurePolicy) -> ProjectionResultValue<()> {
    if policy.policy_id != "handbook.projection-disclosure.core"
        || policy.policy_version != "1.0.0"
        || policy.classification_registry.reference
            != "handbook.projection-classification.core@1.0.0"
        || policy.matcher_definition.reference
            != "handbook.projection-disclosure-matcher.core@1.0.0"
        || policy.unmatched_action != PolicyAction::Redact
        || policy.indeterminate_match_action != "refuse"
        || policy.overlap_precedence != "redact_wins"
        || policy.rules.is_empty()
    {
        return Err(ProjectionError::new(
            ProjectionErrorKind::InvalidPolicy,
            "disclosure policy is not the exact fail-closed metadata-only contract",
        ));
    }
    ExactPair::new(
        &policy.classification_registry.reference,
        &policy.classification_registry.fingerprint,
    )?;
    validate_pair(&policy.matcher_definition)?;
    require_unique(
        policy.rules.iter().map(|rule| rule.rule_id.as_str()),
        "policy rule",
    )?;
    for rule in &policy.rules {
        if rule.disclosure_classifications.is_empty()
            || rule.source_kinds.is_empty()
            || rule.source_pointer_selector != "*"
            || rule
                .disclosure_classifications
                .iter()
                .any(|classification| {
                    !policy
                        .classification_registry
                        .values
                        .contains(classification)
                })
        {
            return Err(ProjectionError::new(
                ProjectionErrorKind::InvalidPolicy,
                "disclosure policy rule is invalid",
            ));
        }
    }
    Ok(())
}

fn validate_evaluator(evaluator: &SupportEvaluator) -> ProjectionResultValue<()> {
    if evaluator.support_evaluator_id != "handbook.projection-support.core"
        || evaluator.support_evaluator_version != "1.0.0"
        || evaluator.schema_registry_contract.reference != "handbook.schema-registry.core@1.0.0"
        || evaluator.pointer_semantics.reference != "handbook.json-pointer.rfc6901@1.0.0"
        || evaluator.derivation_compatibility.reference
            != "handbook.projection-derivation-compatibility.core@1.0.0"
        || evaluator.decision_input_fields
            != DECISION_INPUT_FIELDS
                .into_iter()
                .map(str::to_owned)
                .collect::<Vec<_>>()
        || evaluator.unsupported_reason_precedence != SUPPORT_REASON_ORDER
        || evaluator.supported_source_kinds.is_empty()
    {
        return Err(ProjectionError::new(
            ProjectionErrorKind::InvalidEvaluator,
            "support evaluator or dependency closure is not exact",
        ));
    }
    validate_pair(&evaluator.schema_registry_contract)?;
    validate_pair(&evaluator.pointer_semantics)?;
    validate_pair(&evaluator.derivation_compatibility)?;
    Ok(())
}

fn validate_definition(definition: &AuthoredDefinition) -> ProjectionResultValue<()> {
    if definition.source_selectors.is_empty()
        || definition.allowed_surfaces.is_empty()
        || definition.allowed_operations.is_empty()
        || definition.field_rules.is_empty()
        || definition.allowed_operations.iter().any(|operation| {
            !matches!(
                operation,
                ProjectionOperation::Reveal | ProjectionOperation::Derive
            )
        })
    {
        return Err(ProjectionError::new(
            ProjectionErrorKind::ForbiddenDefinition,
            "Projection definition operation set is not deterministic reveal/derive",
        ));
    }
    validate_currentness_requirements(
        &definition.currentness_requirements,
        &definition.source_selectors,
    )?;
    validate_source_pair_requirements(
        &definition.source_pair_requirements,
        &definition.source_selectors,
    )?;
    require_unique(
        definition
            .source_selectors
            .iter()
            .map(|selector| selector.selector_id.as_str()),
        "source selector",
    )?;
    require_unique(
        definition
            .field_rules
            .iter()
            .map(|rule| rule.rule_id.as_str()),
        "field rule",
    )?;
    require_unique(
        definition
            .field_rules
            .iter()
            .map(|rule| rule.target_pointer.as_str()),
        "target producer",
    )?;
    require_unique(
        definition
            .derivations
            .iter()
            .map(|derivation| derivation.reference.as_str()),
        "derivation",
    )?;
    let selectors = definition
        .source_selectors
        .iter()
        .map(|selector| selector.selector_id.as_str())
        .collect::<BTreeSet<_>>();
    let classifications = &definition.disclosure_policy.classification_registry.values;
    let optional = definition
        .target_schema
        .optional_pointers
        .iter()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    let derivations = definition
        .derivations
        .iter()
        .map(|derivation| (derivation.reference.as_str(), derivation))
        .collect::<BTreeMap<_, _>>();
    ExactPair::new(
        &definition.target_schema.reference,
        &definition.target_schema.fingerprint,
    )?;
    for derivation in &definition.derivations {
        validate_pair(&derivation.exact_pair())?;
    }
    for selector in &definition.source_selectors {
        validate_pair(&selector.capability)?;
        validate_pair(&selector.source_schema)?;
        let is_pair_derived = definition
            .source_pair_requirements
            .iter()
            .any(|requirement| requirement.derived_selector_id == selector.selector_id);
        if selector.cardinality != "exactly_one"
            || selector.configured_kind_ref.is_empty()
            || (!definition
                .support_evaluator
                .supported_source_kinds
                .contains(&selector.source_kind)
                && !is_pair_derived)
        {
            return Err(ProjectionError::new(
                ProjectionErrorKind::InvalidDefinition,
                "source selector is not an exact configured-kind selector",
            ));
        }
    }
    for rule in &definition.field_rules {
        if let Some(derivation) = &rule.derivation {
            validate_pair(derivation)?;
        }
        if !definition.allowed_operations.contains(&rule.operation)
            || !selectors.contains(rule.source_selector_id.as_str())
            || !valid_json_pointer(&rule.source_pointer)
            || !valid_json_pointer(&rule.target_pointer)
            || !classifications.contains(&rule.disclosure_classification)
            || !optional.contains(rule.target_pointer.as_str())
            || definition
                .source_selectors
                .iter()
                .find(|selector| selector.selector_id == rule.source_selector_id)
                .is_none_or(|selector| selector.source_kind == "snapshot_delta")
            || definition
                .target_schema
                .pointer_types
                .get(&rule.target_pointer)
                != Some(&rule.target_pointer_schema)
        {
            return Err(ProjectionError::new(
                ProjectionErrorKind::InvalidDefinition,
                "field rule closure is invalid",
            ));
        }
        match (rule.operation, &rule.derivation) {
            (ProjectionOperation::Reveal, None) => {}
            (ProjectionOperation::Derive, Some(pair)) => {
                let derivation = derivations.get(pair.reference.as_str()).ok_or_else(|| {
                    ProjectionError::new(
                        ProjectionErrorKind::InvalidDefinition,
                        "field rule derivation is unregistered",
                    )
                })?;
                if derivation.fingerprint != pair.fingerprint
                    || derivation.input_type != rule.source_pointer_schema
                    || derivation.output_type != rule.target_pointer_schema
                {
                    return Err(ProjectionError::new(
                        ProjectionErrorKind::InvalidDefinition,
                        "field rule derivation I/O closure is invalid",
                    ));
                }
            }
            _ => {
                return Err(ProjectionError::new(
                    ProjectionErrorKind::InvalidDefinition,
                    "reveal/derive rule has an invalid derivation shape",
                ));
            }
        }
    }
    validate_acyclic_rules(&definition.field_rules)
}

fn validate_pair(pair: &ExactPair) -> ProjectionResultValue<()> {
    ExactPair::new(&pair.reference, &pair.fingerprint).map(|_| ())
}

fn validate_currentness_requirements(
    requirements: &CurrentnessRequirements,
    selectors: &[SourceSelector],
) -> ProjectionResultValue<()> {
    match requirements.mode {
        CurrentnessMode::None => {
            if requirements.revision_basis.is_some() || !requirements.families.is_empty() {
                return Err(ProjectionError::new(
                    ProjectionErrorKind::InvalidCurrentness,
                    "none currentness must have null basis and no families",
                ));
            }
        }
        CurrentnessMode::ExactRevisionCheck => {
            if requirements.revision_basis.as_deref() != Some("captured_revision")
                || requirements.families.is_empty()
            {
                return Err(ProjectionError::new(
                    ProjectionErrorKind::InvalidCurrentness,
                    "exact currentness must use captured_revision and non-empty families",
                ));
            }
            require_unique(
                requirements
                    .families
                    .iter()
                    .map(|family| family.family.as_str()),
                "currentness family",
            )?;
            if requirements
                .families
                .windows(2)
                .any(|pair| pair[0].family >= pair[1].family)
            {
                return Err(ProjectionError::new(
                    ProjectionErrorKind::InvalidCurrentness,
                    "exact currentness families are not canonically sorted",
                ));
            }
            for family in &requirements.families {
                let selector = selectors
                    .iter()
                    .find(|selector| selector.selector_id == family.selector_id);
                if validate_pair(&family.adapter).is_err()
                    || selector.is_none_or(|selector| selector.source_kind != "snapshot")
                    || family.slots.windows(2).any(|pair| pair[0] >= pair[1])
                {
                    return Err(ProjectionError::new(
                        ProjectionErrorKind::InvalidCurrentness,
                        "exact currentness requires a bound snapshot selector and complete family closure",
                    ));
                }
                require_unique(family.slots.iter().map(String::as_str), "currentness slot")?;
            }
        }
    }
    Ok(())
}

fn validate_source_pair_requirements(
    requirements: &[SourcePairRequirement],
    selectors: &[SourceSelector],
) -> ProjectionResultValue<()> {
    require_unique(
        requirements
            .iter()
            .map(|requirement| requirement.current_selector_id.as_str()),
        "source-pair current selector",
    )?;
    require_unique(
        requirements
            .iter()
            .map(|requirement| requirement.derived_selector_id.as_str()),
        "source-pair derived selector",
    )?;
    let paired_derived_selectors = requirements
        .iter()
        .map(|requirement| requirement.derived_selector_id.as_str())
        .collect::<BTreeSet<_>>();
    if selectors.iter().any(|selector| {
        selector.source_kind == "snapshot_delta"
            && !paired_derived_selectors.contains(selector.selector_id.as_str())
    }) {
        return Err(ProjectionError::new(
            ProjectionErrorKind::InvalidDefinition,
            "snapshot-delta selectors require one declared source-pair relation",
        ));
    }
    for requirement in requirements {
        let current_selector = selectors
            .iter()
            .find(|selector| selector.selector_id == requirement.current_selector_id);
        let derived_selector = selectors
            .iter()
            .find(|selector| selector.selector_id == requirement.derived_selector_id);
        if requirement.current_selector_id.is_empty()
            || requirement.derived_selector_id.is_empty()
            || requirement.dependency_role.is_empty()
            || !requirement.require_state_identity
            || requirement.current_selector_id == requirement.derived_selector_id
            || current_selector.is_none_or(|selector| selector.source_kind != "snapshot")
            || derived_selector.is_none_or(|selector| selector.source_kind != "snapshot_delta")
        {
            return Err(ProjectionError::new(
                ProjectionErrorKind::InvalidDefinition,
                "source-pair requirement is not an exact selector relation",
            ));
        }
    }
    Ok(())
}

fn validate_acyclic_rules(rules: &[FieldRule]) -> ProjectionResultValue<()> {
    // V1 derivations read only exact source-selector payloads and can never
    // name another rule's output namespace. The graph is therefore acyclic
    // by construction once every rule has one validated source selector.
    if rules.iter().any(|rule| rule.source_selector_id.is_empty()) {
        return Err(ProjectionError::new(
            ProjectionErrorKind::InvalidDefinition,
            "field rule graph contains an unbound source node",
        ));
    }
    Ok(())
}

fn require_unique<'a>(
    values: impl Iterator<Item = &'a str>,
    label: &str,
) -> ProjectionResultValue<()> {
    let mut seen = BTreeSet::new();
    for value in values {
        if value.is_empty() || !seen.insert(value) {
            return Err(ProjectionError::new(
                ProjectionErrorKind::InvalidDefinition,
                format!("{label} identity is empty or duplicated"),
            ));
        }
    }
    Ok(())
}

fn valid_json_pointer(pointer: &str) -> bool {
    pointer.starts_with('/')
        && !pointer.contains("//")
        && !pointer.bytes().any(|byte| byte.is_ascii_control())
}

fn strict_jcs(bytes: &[u8], label: &str) -> ProjectionResultValue<Value> {
    if bytes.is_empty()
        || bytes.len() > crate::definition_identity::MAX_SOURCE_DOCUMENT_BYTES
        || bytes.starts_with(&[0xef, 0xbb, 0xbf])
        || bytes.ends_with(b"\r\n")
    {
        return Err(ProjectionError::new(
            ProjectionErrorKind::InvalidJson,
            format!("{label} is not bounded strict UTF-8 JSON"),
        ));
    }
    // The repository text convention contributes one terminal LF; the JSON
    // document itself must otherwise be byte-identical RFC 8785 output.
    let document = bytes.strip_suffix(b"\n").unwrap_or(bytes);
    let value = crate::parse_schema_json(document).map_err(|_| {
        ProjectionError::new(
            ProjectionErrorKind::InvalidJson,
            format!("{label} is not duplicate-safe I-JSON"),
        )
    })?;
    let canonical = canonical_json_bytes(&value).map_err(registry_fingerprint_error)?;
    if canonical != document {
        return Err(ProjectionError::new(
            ProjectionErrorKind::NonCanonicalJson,
            format!("{label} is not byte-identical JCS"),
        ));
    }
    Ok(value)
}

fn contains_forbidden_definition_content(value: &Value) -> bool {
    const FORBIDDEN_KEYS: [&str; 12] = [
        "command",
        "executable",
        "hook",
        "model",
        "prompt",
        "remote",
        "remote_code",
        "script",
        "synthesis",
        "synthesize",
        "transport_rule",
        "value_matcher",
    ];
    match value {
        Value::Object(object) => object.iter().any(|(key, child)| {
            FORBIDDEN_KEYS.iter().any(|forbidden| key == forbidden)
                || contains_forbidden_definition_content(child)
        }),
        Value::Array(values) => values.iter().any(contains_forbidden_definition_content),
        Value::String(value) => {
            value.starts_with("http://")
                || value.starts_with("https://")
                || value == "synthesize_candidate"
        }
        _ => false,
    }
}

fn require_fingerprint(
    value: &Value,
    field: &str,
    supplied: &str,
    kind: ProjectionErrorKind,
) -> ProjectionResultValue<DefinitionFingerprint> {
    let supplied = DefinitionFingerprint::parse(supplied)
        .map_err(|_| ProjectionError::new(kind, format!("{field} is malformed")))?;
    let computed = fingerprint_without_field(value, field)?;
    if supplied != computed {
        return Err(ProjectionError::new(
            kind,
            format!("{field} does not bind the exact semantic closure"),
        ));
    }
    Ok(computed)
}

fn fingerprint_without_field(
    value: &Value,
    field: &str,
) -> ProjectionResultValue<DefinitionFingerprint> {
    let mut preimage = value.clone();
    preimage
        .as_object_mut()
        .ok_or_else(|| {
            ProjectionError::new(
                ProjectionErrorKind::InvalidJson,
                "fingerprint preimage is not an object",
            )
        })?
        .remove(field)
        .ok_or_else(|| {
            ProjectionError::new(
                ProjectionErrorKind::InvalidJson,
                format!("fingerprint field {field} is absent"),
            )
        })?;
    DefinitionFingerprint::from_json_value(&preimage).map_err(registry_fingerprint_error)
}

fn validate_trusted_semantic_dependencies(
    profile: &AuthoredProfile,
    definition: &AuthoredDefinition,
) -> ProjectionResultValue<()> {
    for kind in &profile.configured_kinds {
        require_trusted_pair(&kind.capability, "configured-kind capability")?;
        require_trusted_pair(&kind.source_schema, "configured-kind source schema")?;
    }
    for selector in &definition.source_selectors {
        require_trusted_pair(&selector.capability, "selector capability")?;
        require_trusted_pair(&selector.source_schema, "selector source schema")?;
    }

    let target_schema = json!({
        "optional_pointers": definition.target_schema.optional_pointers,
        "pointer_types": definition.target_schema.pointer_types,
        "ref": definition.target_schema.reference,
    });
    require_recomputed_trusted_pair(
        &definition.target_schema.exact_pair(),
        &target_schema,
        "target schema",
    )?;

    let classification_registry = json!({
        "ref": definition.disclosure_policy.classification_registry.reference,
        "values": definition.disclosure_policy.classification_registry.values,
    });
    let classification_pair = ExactPair {
        reference: definition
            .disclosure_policy
            .classification_registry
            .reference
            .clone(),
        fingerprint: definition
            .disclosure_policy
            .classification_registry
            .fingerprint
            .clone(),
    };
    require_recomputed_trusted_pair(
        &classification_pair,
        &classification_registry,
        "classification registry",
    )?;
    require_trusted_pair(
        &definition.disclosure_policy.matcher_definition,
        "disclosure matcher",
    )?;
    let trusted_policy_rule_shapes = json!([
        {
            "action": "allow",
            "disclosure_classifications": ["public"],
            "rule_id": "public_allow",
            "source_pointer_selector": "*",
        },
        {
            "action": "allow",
            "disclosure_classifications": ["internal"],
            "rule_id": "internal_allow",
            "source_pointer_selector": "*",
        },
        {
            "action": "redact",
            "disclosure_classifications": ["sensitive", "secret"],
            "rule_id": "sensitive_redact",
            "source_pointer_selector": "*",
        },
    ]);
    let policy_rule_shapes = Value::Array(
        definition
            .disclosure_policy
            .rules
            .iter()
            .map(|rule| {
                json!({
                    "action": match rule.action { PolicyAction::Allow => "allow", PolicyAction::Redact => "redact" },
                    "disclosure_classifications": rule.disclosure_classifications,
                    "rule_id": rule.rule_id,
                    "source_pointer_selector": rule.source_pointer_selector,
                })
            })
            .collect(),
    );
    let allowed_policy_source_kinds = ["semantic_record".to_owned(), "snapshot".to_owned()];
    if !definition
        .disclosure_policy
        .rules
        .iter()
        .all(|rule| rule.source_kinds == allowed_policy_source_kinds)
        || policy_rule_shapes != trusted_policy_rule_shapes
    {
        return Err(ProjectionError::new(
            ProjectionErrorKind::StaleBinding,
            "disclosure policy rules drift from the bounded private definition",
        ));
    }

    require_trusted_pair(
        &definition.support_evaluator.schema_registry_contract,
        "schema registry contract",
    )?;
    require_trusted_pair(
        &definition.support_evaluator.pointer_semantics,
        "pointer semantics",
    )?;
    require_trusted_pair(
        &definition.support_evaluator.derivation_compatibility,
        "derivation compatibility",
    )?;
    if definition.support_evaluator.supported_source_kinds
        != ["semantic_record".to_owned(), "snapshot".to_owned()]
    {
        return Err(ProjectionError::new(
            ProjectionErrorKind::StaleBinding,
            "support evaluator source-kind semantics drift from the bounded private definition",
        ));
    }

    for derivation in &definition.derivations {
        let semantic_definition = json!({
            "algorithm": match derivation.algorithm {
                DerivationAlgorithm::CountArray => "count_array",
                DerivationAlgorithm::NormalizedSummary => "normalized_summary",
            },
            "input_type": derivation.input_type,
            "lossy": derivation.lossy,
            "output_type": derivation.output_type,
            "ref": derivation.reference,
        });
        require_recomputed_trusted_pair(
            &derivation.exact_pair(),
            &semantic_definition,
            "derivation definition",
        )?;
    }
    Ok(())
}

fn require_recomputed_trusted_pair(
    pair: &ExactPair,
    semantic_definition: &Value,
    label: &str,
) -> ProjectionResultValue<()> {
    let recomputed = DefinitionFingerprint::from_json_value(semantic_definition)
        .map_err(registry_fingerprint_error)?;
    if pair.fingerprint != recomputed.as_str() {
        return Err(ProjectionError::new(
            ProjectionErrorKind::StaleBinding,
            format!("{label} fingerprint does not bind its selected semantic definition"),
        ));
    }
    require_trusted_pair(pair, label)
}

fn require_trusted_pair(pair: &ExactPair, label: &str) -> ProjectionResultValue<()> {
    let trusted = trusted_semantic_definition(&pair.reference).ok_or_else(|| {
        ProjectionError::new(
            ProjectionErrorKind::StaleBinding,
            format!("{label} is absent from the bounded private semantic registry"),
        )
    })?;
    let fingerprint =
        DefinitionFingerprint::from_json_value(&trusted).map_err(registry_fingerprint_error)?;
    if pair.fingerprint != fingerprint.as_str() {
        return Err(ProjectionError::new(
            ProjectionErrorKind::StaleBinding,
            format!("{label} does not resolve to the trusted semantic definition"),
        ));
    }
    Ok(())
}

fn trusted_semantic_definition(reference: &str) -> Option<Value> {
    match reference {
        "handbook.semantic-capability.projection-source@1.0.0" => Some(json!({
            "allowed_source_kinds": ["semantic_record", "snapshot"],
            "authority_effect": "none",
            "capability_id": "handbook.semantic-capability.projection-source",
            "capability_version": "1.0.0",
        })),
        "handbook.schemas.custom-roadmap@1.0.0" => Some(json!({
            "pointer_types": {
                "/constraints": "array",
                "/internal_notes": "string",
                "/objective": "string",
                "/optional_detail": "string",
                "/secret_notes": "string",
                "/upstream_hidden": "string",
            },
            "schema_id": "handbook.schemas.custom-roadmap",
            "schema_version": "1.0.0",
        })),
        "handbook.schemas.projection.custom-roadmap-agent-packet@1.0.0" => Some(json!({
            "optional_pointers": [
                "/constraint_count",
                "/constraint_summary",
                "/internal_notes",
                "/objective",
                "/optional_detail",
                "/secret_notes",
                "/upstream_hidden",
            ],
            "pointer_types": {
                "/constraint_count": "number",
                "/constraint_summary": "string",
                "/internal_notes": "string",
                "/objective": "string",
                "/optional_detail": "string",
                "/secret_notes": "string",
                "/upstream_hidden": "string",
            },
            "ref": "handbook.schemas.projection.custom-roadmap-agent-packet@1.0.0",
        })),
        "handbook.derivation.count-array@1.0.0" => Some(json!({
            "algorithm": "count_array",
            "input_type": "array",
            "lossy": false,
            "output_type": "number",
            "ref": "handbook.derivation.count-array@1.0.0",
        })),
        "handbook.derivation.normalized-summary@1.0.0" => Some(json!({
            "algorithm": "normalized_summary",
            "input_type": "array",
            "lossy": true,
            "output_type": "string",
            "ref": "handbook.derivation.normalized-summary@1.0.0",
        })),
        "handbook.projection-classification.core@1.0.0" => Some(json!({
            "ref": "handbook.projection-classification.core@1.0.0",
            "values": ["public", "internal", "sensitive", "secret"],
        })),
        "handbook.projection-disclosure-matcher.core@1.0.0" => Some(json!({
            "inputs": ["disclosure_classification", "source_kind", "source_pointer"],
            "matching": "ordered_exact_metadata_only",
            "ref": "handbook.projection-disclosure-matcher.core@1.0.0",
        })),
        "handbook.schema-registry.core@1.0.0" => Some(json!({
            "binding": "exact_local_schema_ref_fingerprint",
            "ref": "handbook.schema-registry.core@1.0.0",
            "remote_refs": "forbidden",
        })),
        "handbook.json-pointer.rfc6901@1.0.0" => Some(json!({
            "ref": "handbook.json-pointer.rfc6901@1.0.0",
            "root_pointer": "forbidden",
            "syntax": "rfc6901",
        })),
        "handbook.projection-derivation-compatibility.core@1.0.0" => Some(json!({
            "algorithms": ["count_array", "normalized_summary"],
            "matching": "exact_input_output_types",
            "ref": "handbook.projection-derivation-compatibility.core@1.0.0",
        })),
        _ => None,
    }
}

fn registry_fingerprint_error(error: RegistryLoadError) -> ProjectionError {
    ProjectionError::new(
        ProjectionErrorKind::InvalidJson,
        format!("canonical fingerprint failed: {error}"),
    )
}

#[cfg(test)]
fn fingerprint_validated_definition(
    definition: &ValidatedDefinition,
) -> ProjectionResultValue<DefinitionFingerprint> {
    let value = json!({
        "allowed_operations": definition.allowed_operations,
        "allowed_surfaces": definition.allowed_surfaces,
        "currentness_requirements": definition.currentness_requirements,
        "definition_fingerprint": definition.exact_pair.fingerprint,
        "derivations": definition.derivations.iter().map(|derivation| json!({
            "algorithm": match derivation.algorithm { DerivationAlgorithm::CountArray => "count_array", DerivationAlgorithm::NormalizedSummary => "normalized_summary" },
            "fingerprint": derivation.fingerprint,
            "input_type": derivation.input_type,
            "lossy": derivation.lossy,
            "output_type": derivation.output_type,
            "ref": derivation.reference,
        })).collect::<Vec<_>>(),
        "disclosure_policy": definition.disclosure_policy,
        "field_rules": definition.field_rules.iter().map(field_rule_value).collect::<Vec<_>>(),
        "projection_definition_id": definition.exact_pair.reference.split('@').next().unwrap_or_default(),
        "projection_definition_version": definition.exact_pair.reference.split('@').nth(1).unwrap_or_default(),
        "source_selectors": definition.source_selectors.iter().map(source_selector_value).collect::<Vec<_>>(),
        "source_pair_requirements": definition.source_pair_requirements,
        "support_evaluator": definition.support_evaluator,
        "target_schema": definition.target_schema,
    });
    fingerprint_without_field(&value, "definition_fingerprint")
}

#[cfg(test)]
fn fingerprint_validated_profile(
    profile: &ValidatedProfile,
) -> ProjectionResultValue<DefinitionFingerprint> {
    let value = json!({
        "configured_kinds": profile.configured_kinds.iter().map(configured_kind_value).collect::<Vec<_>>(),
        "profile_fingerprint": profile.configuration_pair.fingerprint,
        "profile_id": profile.configuration_pair.reference.split('@').next().unwrap_or_default(),
        "profile_version": profile.configuration_pair.reference.split('@').nth(1).unwrap_or_default(),
        "projection_catalog": profile.projection_catalog,
        "purposes": profile.purposes,
        "resolved_profile": profile.exact_pair,
        "resolution_stack": profile.resolution_stack,
        "schema_id": "handbook.private-resolved-projection-profile",
        "schema_version": "1.0",
        "surfaces": profile.surfaces,
        "vocabulary": profile.vocabulary,
    });
    fingerprint_without_field(&value, "profile_fingerprint")
}

#[cfg(test)]
fn configured_kind_value(kind: &ConfiguredKind) -> Value {
    json!({
        "capability": kind.capability,
        "kind_ref": kind.kind_ref,
        "source_kind": kind.source_kind,
        "source_schema": kind.source_schema,
    })
}

#[cfg(test)]
fn source_selector_value(selector: &SourceSelector) -> Value {
    json!({
        "capability": selector.capability,
        "cardinality": selector.cardinality,
        "configured_kind_ref": selector.configured_kind_ref,
        "selector_id": selector.selector_id,
        "source_kind": selector.source_kind,
        "source_schema": selector.source_schema,
    })
}

#[cfg(test)]
fn field_rule_value(rule: &FieldRule) -> Value {
    json!({
        "claim_refs": rule.claim_refs,
        "derivation": rule.derivation,
        "disclosure_classification": rule.disclosure_classification,
        "minimum_resolution": {
            "authority_horizon": rule.minimum_resolution.authority_horizon,
            "detail_resolution": rule.minimum_resolution.detail_resolution,
            "memory_horizon": rule.minimum_resolution.memory_horizon,
            "scope_horizon": rule.minimum_resolution.scope_horizon,
            "temporal_horizon": rule.minimum_resolution.temporal_horizon,
            "validation_horizon": rule.minimum_resolution.validation_horizon,
        },
        "operation": rule.operation,
        "required_for_result": rule.required_for_result,
        "rule_id": rule.rule_id,
        "source_pointer": rule.source_pointer,
        "source_pointer_schema": rule.source_pointer_schema,
        "source_selector_id": rule.source_selector_id,
        "target_pointer": rule.target_pointer,
        "target_pointer_schema": rule.target_pointer_schema,
    })
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawSourceDocument {
    authority_class: String,
    capability: ExactPair,
    captured_revisions: Option<CapturedRevision>,
    captured_family_revisions: Option<Vec<CapturedRevision>>,
    configured_kind_ref: String,
    declared_pointers: BTreeMap<String, String>,
    payload_jcs: String,
    #[serde(rename = "ref")]
    reference: String,
    schema: ExactPair,
    source_kind: String,
    #[serde(default)]
    source_dependencies: Vec<SourceDependency>,
    #[serde(default)]
    state_fingerprint: Option<String>,
    upstream_redactions: Vec<UpstreamRedaction>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct CapturedRevision {
    family: String,
    adapter: ExactPair,
    family_revision: String,
    slots: BTreeMap<String, String>,
}

#[derive(Clone, Debug)]
struct SourceDocument {
    authority_class: String,
    capability: ExactPair,
    captured_family_revisions: Vec<CapturedRevision>,
    configured_kind_ref: String,
    declared_pointers: BTreeMap<String, String>,
    payload_jcs: String,
    reference: String,
    schema: ExactPair,
    source_kind: String,
    source_dependencies: Vec<SourceDependency>,
    state_fingerprint: Option<String>,
    upstream_redactions: Vec<UpstreamRedaction>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct SourceDependency {
    role: String,
    source: ExactPair,
    state_fingerprint: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct UpstreamRedaction {
    original_pointer: String,
    action: String,
    disposition: ExactPair,
    retained_pointer: Option<String>,
}

#[derive(Debug)]
struct ProjectionSource {
    exact_bytes: Vec<u8>,
    exact_pair: ExactPair,
    document: SourceDocument,
    payload_reads: Cell<usize>,
}

impl ProjectionSource {
    fn load(bytes: &[u8]) -> ProjectionResultValue<Self> {
        let value = strict_jcs(bytes, "Projection source")?;
        let raw: RawSourceDocument = serde_json::from_value(value).map_err(|error| {
            ProjectionError::new(
                ProjectionErrorKind::InvalidDefinition,
                format!("Projection source envelope is not exact: {error}"),
            )
        })?;
        let document = SourceDocument {
            authority_class: raw.authority_class,
            capability: raw.capability,
            captured_family_revisions: normalize_captured_family_revisions(
                raw.captured_revisions,
                raw.captured_family_revisions,
            )?,
            configured_kind_ref: raw.configured_kind_ref,
            declared_pointers: raw.declared_pointers,
            payload_jcs: raw.payload_jcs,
            reference: raw.reference,
            schema: raw.schema,
            source_kind: raw.source_kind,
            source_dependencies: raw.source_dependencies,
            state_fingerprint: raw.state_fingerprint,
            upstream_redactions: raw.upstream_redactions,
        };
        if document.reference.is_empty()
            || document.authority_class.is_empty()
            || document.configured_kind_ref.is_empty()
            || document.declared_pointers.is_empty()
            || document.payload_jcs.is_empty()
        {
            return Err(ProjectionError::new(
                ProjectionErrorKind::InvalidDefinition,
                "Projection source metadata closure is incomplete",
            ));
        }
        if document
            .state_fingerprint
            .as_deref()
            .is_some_and(|fingerprint| DefinitionFingerprint::parse(fingerprint).is_err())
        {
            return Err(ProjectionError::new(
                ProjectionErrorKind::InvalidDefinition,
                "source state fingerprint is invalid",
            ));
        }
        for dependency in &document.source_dependencies {
            if dependency.role.is_empty()
                || DefinitionFingerprint::parse(&dependency.state_fingerprint).is_err()
                || validate_pair(&dependency.source).is_err()
            {
                return Err(ProjectionError::new(
                    ProjectionErrorKind::InvalidDefinition,
                    "source dependency closure is invalid",
                ));
            }
        }
        require_unique(
            document
                .source_dependencies
                .iter()
                .map(|dependency| dependency.role.as_str()),
            "source dependency role",
        )?;
        let fingerprint = DefinitionFingerprint::from_bytes(bytes).to_string();
        let exact_pair = ExactPair::new(&document.reference, &fingerprint)?;
        Ok(Self {
            exact_bytes: bytes.to_vec(),
            exact_pair,
            document,
            payload_reads: Cell::new(0),
        })
    }

    fn exact_bytes(&self) -> &[u8] {
        &self.exact_bytes
    }

    fn exact_pair(&self) -> ExactPair {
        self.exact_pair.clone()
    }

    fn payload_read_count(&self) -> usize {
        self.payload_reads.get()
    }

    fn read_payload_pointer(&self, pointer: &str) -> ProjectionResultValue<Option<Value>> {
        self.payload_reads.set(self.payload_reads.get() + 1);
        let payload = strict_jcs(
            self.document.payload_jcs.as_bytes(),
            "protected source payload",
        )?;
        Ok(payload.pointer(pointer).cloned())
    }
}

fn normalize_captured_family_revisions(
    legacy: Option<CapturedRevision>,
    multi_family: Option<Vec<CapturedRevision>>,
) -> ProjectionResultValue<Vec<CapturedRevision>> {
    let (captured, requires_sorted_input) = match (legacy, multi_family) {
        (Some(legacy), None) => (vec![legacy], false),
        (None, Some(multi_family)) => (multi_family, true),
        _ => {
            return Err(ProjectionError::new(
                ProjectionErrorKind::InvalidDefinition,
                "source captured revisions must use exactly one legacy or multi-family form",
            ));
        }
    };
    if captured.is_empty()
        || captured.iter().any(|revision| {
            revision.family.is_empty()
                || revision.family_revision.is_empty()
                || revision
                    .slots
                    .iter()
                    .any(|(slot, value)| slot.is_empty() || value.is_empty())
                || validate_pair(&revision.adapter).is_err()
        })
        || (requires_sorted_input
            && captured
                .windows(2)
                .any(|pair| pair[0].family >= pair[1].family))
    {
        return Err(ProjectionError::new(
            ProjectionErrorKind::InvalidDefinition,
            "source captured-family closure is malformed, unsorted, or duplicate",
        ));
    }
    Ok(captured)
}

trait ProjectionAuthorityAccess {
    fn envelope_pair(&self) -> ExactPair;
    fn resolved_profile_pair(&self) -> ExactPair;
    fn resolution_stack_pair(&self) -> ExactPair;
    fn active_level(&self) -> &str;
    fn dimension_values(&self) -> [&str; 6];
    fn dimension_ranks(&self) -> [u8; 6];
}

impl ProjectionAuthorityAccess for ProjectionAuthorityView {
    fn envelope_pair(&self) -> ExactPair {
        self.envelope().into()
    }

    fn resolved_profile_pair(&self) -> ExactPair {
        self.resolved_profile().into()
    }

    fn resolution_stack_pair(&self) -> ExactPair {
        self.resolution_stack().into()
    }

    fn active_level(&self) -> &str {
        self.active_level()
    }

    fn dimension_values(&self) -> [&str; 6] {
        self.dimension_values()
    }

    fn dimension_ranks(&self) -> [u8; 6] {
        self.dimension_ranks()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
struct ProjectionSourceSelection {
    selector_id: String,
    exact_pair: ExactPair,
    state_fingerprint: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
struct ProjectionCurrentnessRequest {
    mode: CurrentnessMode,
    revision_basis: Option<String>,
    expected_family_revisions: Vec<ExpectedFamilyRevision>,
}

impl ProjectionCurrentnessRequest {
    fn none() -> Self {
        Self {
            mode: CurrentnessMode::None,
            revision_basis: None,
            expected_family_revisions: Vec::new(),
        }
    }

    fn captured(source: &ProjectionSource, selector_id: &str) -> ProjectionResultValue<Self> {
        let captured = &source.document.captured_family_revisions;
        Ok(Self {
            mode: CurrentnessMode::ExactRevisionCheck,
            revision_basis: Some("captured_revision".to_owned()),
            expected_family_revisions: captured
                .iter()
                .map(|captured| ExpectedFamilyRevision {
                    selector_id: selector_id.to_owned(),
                    family: captured.family.clone(),
                    adapter: captured.adapter.clone(),
                    family_revision: captured.family_revision.clone(),
                    slots: captured
                        .slots
                        .iter()
                        .map(|(slot, revision)| RevisionSlot {
                            slot: slot.clone(),
                            revision: revision.clone(),
                        })
                        .collect(),
                })
                .collect(),
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
struct ExpectedFamilyRevision {
    selector_id: String,
    family: String,
    adapter: ExactPair,
    family_revision: String,
    slots: Vec<RevisionSlot>,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize)]
struct RevisionSlot {
    slot: String,
    revision: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
struct ProjectionRequest {
    request_id: String,
    sources: Vec<ProjectionSourceSelection>,
    profile_configuration: ExactPair,
    resolved_profile: ExactPair,
    vocabulary: ExactPair,
    projection_definition: ExactPair,
    resolution_envelope: ExactPair,
    resolution_stack: ExactPair,
    operation: ProjectionOperation,
    surface: String,
    purpose: String,
    currentness: ProjectionCurrentnessRequest,
    target_resolution_ranks: [u8; 6],
}

impl ProjectionRequest {
    #[allow(clippy::too_many_arguments)]
    fn new(
        request_id: &str,
        sources: Vec<ProjectionSourceSelection>,
        profile_configuration: ExactPair,
        resolved_profile: ExactPair,
        vocabulary: ExactPair,
        projection_definition: ExactPair,
        resolution_envelope: ExactPair,
        resolution_stack: ExactPair,
        operation: ProjectionOperation,
        surface: &str,
        purpose: &str,
        currentness: ProjectionCurrentnessRequest,
        target_resolution_ranks: [u8; 6],
    ) -> Self {
        Self {
            request_id: request_id.to_owned(),
            sources,
            profile_configuration,
            resolved_profile,
            vocabulary,
            projection_definition,
            resolution_envelope,
            resolution_stack,
            operation,
            surface: surface.to_owned(),
            purpose: purpose.to_owned(),
            currentness,
            target_resolution_ranks,
        }
    }

    fn non_envelope_fingerprint(&self) -> ProjectionResultValue<DefinitionFingerprint> {
        let mut value = serde_json::to_value(self).map_err(|_| {
            ProjectionError::new(
                ProjectionErrorKind::InvalidJson,
                "request cannot be normalized",
            )
        })?;
        let object = value
            .as_object_mut()
            .expect("Projection request serializes as object");
        object.remove("resolution_envelope");
        object.remove("target_resolution_ranks");
        DefinitionFingerprint::from_json_value(&value).map_err(registry_fingerprint_error)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
enum MinimumResolutionOutcome {
    Sufficient,
    Insufficient,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
enum UpstreamRedactionCheck {
    None,
    Covered,
    NotEvaluated,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
enum PolicyEvaluation {
    Matched,
    UnmatchedDefault,
    NotEvaluated,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
enum EvaluatedPolicyAction {
    Allow,
    Redact,
    NotEvaluated,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
enum SupportStatus {
    Supported,
    Unsupported,
    NotEvaluated,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
enum PayloadAccess {
    Read,
    AttemptedUnavailable,
    NotAttempted,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
enum FinalDisposition {
    Included,
    OutOfResolution,
    Redacted,
    Unavailable,
    Unsupported,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
struct DisclosureEvaluation {
    rule_id: String,
    definition_rule_ordinal: usize,
    minimum_resolution_outcome: MinimumResolutionOutcome,
    upstream_redaction_check: UpstreamRedactionCheck,
    upstream_redaction_disposition: Option<ExactPair>,
    policy_evaluation: PolicyEvaluation,
    matched_policy_rule_ids: Vec<String>,
    policy_action: EvaluatedPolicyAction,
    support_evaluator: ExactPair,
    support_status: SupportStatus,
    support_reason: Option<SupportReason>,
    payload_access: PayloadAccess,
    final_disposition: FinalDisposition,
    evaluation_fingerprint: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
enum OmissionReason {
    OutOfResolution,
    Redacted,
    Unavailable,
    Unsupported,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
enum ProofEffect {
    NotObserved,
    None,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
struct IncludedRule {
    rule_id: String,
    source_selector_id: String,
    source_pointer: String,
    target_pointer: String,
    claim_refs: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
struct OmittedRule {
    rule_id: String,
    source_selector_id: String,
    source_pointer: String,
    target_pointer: String,
    claim_refs: Vec<String>,
    reason: OmissionReason,
    proof_effect: ProofEffect,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
enum NotApplicableReason {
    OperationMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
struct NotApplicableRule {
    rule_id: String,
    reason: NotApplicableReason,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
struct DerivationRecord {
    rule_id: String,
    derivation: ExactPair,
    input_fingerprints: Vec<String>,
    output_fingerprint: String,
    lossy: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
enum ProjectionLossiness {
    Redacted,
    Partial,
    Collapsed,
    Lossless,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
enum AuthorityEffect {
    None,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
struct CurrentnessValidation {
    mode: CurrentnessMode,
    revision_basis: Option<String>,
    checks: Vec<CurrentnessCheck>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
struct CurrentnessCheck {
    selector_id: String,
    family: String,
    adapter: ExactPair,
    expected_revision: String,
    captured_revision: String,
    observed_revision: String,
    expected_slots: BTreeMap<String, String>,
    captured_slots: BTreeMap<String, String>,
    observed_slots: BTreeMap<String, String>,
    passed: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct LiveCurrentnessQuery {
    selector_id: String,
    source: ExactPair,
    family: String,
    adapter: ExactPair,
    slots: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct LiveCurrentnessObservation {
    selector_id: String,
    source: ExactPair,
    family: String,
    adapter: ExactPair,
    family_revision: String,
    slots: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
struct ProjectionOutput {
    schema: ExactPair,
    content_ref: String,
    content: Value,
    content_fingerprint: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
struct ProjectionResult {
    result_id: String,
    request_ref: String,
    sources: Vec<ProjectionSourceSelection>,
    profile_configuration: ExactPair,
    resolved_profile: ExactPair,
    vocabulary: ExactPair,
    projection_definition: ExactPair,
    resolution_envelope: ExactPair,
    resolution_stack: ExactPair,
    active_level: String,
    dimension_values: [String; 6],
    dimension_ranks: [u8; 6],
    target_resolution_ranks: [u8; 6],
    purpose: String,
    surface: String,
    operation: ProjectionOperation,
    currentness_validation: CurrentnessValidation,
    disclosure_evaluations: Vec<DisclosureEvaluation>,
    lossiness: ProjectionLossiness,
    included: Vec<IncludedRule>,
    omissions: Vec<OmittedRule>,
    not_applicable: Vec<NotApplicableRule>,
    derivations: Vec<DerivationRecord>,
    output: ProjectionOutput,
    authority_effect: AuthorityEffect,
    result_fingerprint: String,
}

impl ProjectionResult {
    fn canonical_bytes(&self) -> ProjectionResultValue<Vec<u8>> {
        let value = serde_json::to_value(self).map_err(|_| {
            ProjectionError::new(
                ProjectionErrorKind::InvalidJson,
                "Projection result cannot be normalized",
            )
        })?;
        canonical_json_bytes(&value).map_err(registry_fingerprint_error)
    }

    fn disposition_for(&self, rule_id: &str) -> Option<FinalDisposition> {
        self.disclosure_evaluations
            .iter()
            .find(|evaluation| evaluation.rule_id == rule_id)
            .map(|evaluation| evaluation.final_disposition)
    }

    fn omission(&self, rule_id: &str) -> Option<&OmittedRule> {
        self.omissions
            .iter()
            .find(|omission| omission.rule_id == rule_id)
    }

    fn evaluation(&self, rule_id: &str) -> Option<&DisclosureEvaluation> {
        self.disclosure_evaluations
            .iter()
            .find(|evaluation| evaluation.rule_id == rule_id)
    }
}

fn execute_projection<A: ProjectionAuthorityAccess>(
    configuration: &ProjectionConfiguration,
    authority: &A,
    request: &ProjectionRequest,
    sources: &[&ProjectionSource],
) -> ProjectionResultValue<ProjectionResult> {
    execute_projection_with_live_observer(configuration, authority, request, sources, &|_| {
        Err(ProjectionError::new(
            ProjectionErrorKind::InvalidCurrentness,
            "exact currentness requires an independent live observation",
        ))
    })
}

fn execute_projection_with_live_observer<A, O>(
    configuration: &ProjectionConfiguration,
    authority: &A,
    request: &ProjectionRequest,
    sources: &[&ProjectionSource],
    observe_live: &O,
) -> ProjectionResultValue<ProjectionResult>
where
    A: ProjectionAuthorityAccess,
    O: Fn(&LiveCurrentnessQuery) -> ProjectionResultValue<LiveCurrentnessObservation>,
{
    validate_request(configuration, authority, request)?;
    let selected = bind_sources(configuration, request, sources)?;
    validate_selected_source_pair_relations(configuration, request, &selected)?;
    validate_selected_source_semantics(configuration, &selected)?;
    let currentness_validation =
        validate_request_currentness(configuration, request, &selected, observe_live)?;

    let authority_ranks = authority.dimension_ranks();
    let effective_ranks = request.target_resolution_ranks;
    let mut output = Map::new();
    let mut evaluations = Vec::new();
    let mut included = Vec::new();
    let mut omissions = Vec::new();
    let mut not_applicable = Vec::new();
    let mut derivations = Vec::new();

    for (index, rule) in configuration.definition.field_rules.iter().enumerate() {
        if rule.operation != request.operation {
            not_applicable.push(NotApplicableRule {
                rule_id: rule.rule_id.clone(),
                reason: NotApplicableReason::OperationMismatch,
            });
            continue;
        }
        let selector = configuration
            .definition
            .source_selectors
            .iter()
            .find(|selector| selector.selector_id == rule.source_selector_id)
            .expect("validated rule selector");
        let source = selected
            .get(selector.selector_id.as_str())
            .expect("exact source binding");
        let mut evaluation = empty_evaluation(configuration, rule, index + 1);

        if effective_ranks
            .iter()
            .zip(rule.minimum_resolution.ranks())
            .any(|(actual, minimum)| *actual < minimum)
        {
            evaluation.minimum_resolution_outcome = MinimumResolutionOutcome::Insufficient;
            evaluation.final_disposition = FinalDisposition::OutOfResolution;
            record_omission(rule, OmissionReason::OutOfResolution, &mut omissions);
            finish_evaluation(configuration, request, source, &mut evaluation)?;
            evaluations.push(evaluation);
            continue;
        }

        evaluation.minimum_resolution_outcome = MinimumResolutionOutcome::Sufficient;
        if let Some(redaction) = covering_upstream_redaction(source, &rule.source_pointer) {
            evaluation.upstream_redaction_check = UpstreamRedactionCheck::Covered;
            evaluation.upstream_redaction_disposition = Some(redaction.disposition.clone());
            evaluation.final_disposition = FinalDisposition::Redacted;
            record_omission(rule, OmissionReason::Redacted, &mut omissions);
            finish_evaluation(configuration, request, source, &mut evaluation)?;
            evaluations.push(evaluation);
            continue;
        }

        evaluation.upstream_redaction_check = UpstreamRedactionCheck::None;
        let (policy_evaluation, matched, action) = evaluate_disclosure_policy(
            &configuration.definition.disclosure_policy,
            &source.document.source_kind,
            &rule.source_pointer,
            &rule.disclosure_classification,
        );
        evaluation.policy_evaluation = policy_evaluation;
        evaluation.matched_policy_rule_ids = matched;
        evaluation.policy_action = action;
        if action == EvaluatedPolicyAction::Redact {
            evaluation.final_disposition = FinalDisposition::Redacted;
            record_omission(rule, OmissionReason::Redacted, &mut omissions);
            finish_evaluation(configuration, request, source, &mut evaluation)?;
            evaluations.push(evaluation);
            continue;
        }

        let support_reason = evaluate_support(configuration, selector, source, rule);
        if let Some(reason) = support_reason {
            evaluation.support_status = SupportStatus::Unsupported;
            evaluation.support_reason = Some(reason);
            evaluation.final_disposition = FinalDisposition::Unsupported;
            record_omission(rule, OmissionReason::Unsupported, &mut omissions);
            finish_evaluation(configuration, request, source, &mut evaluation)?;
            evaluations.push(evaluation);
            continue;
        }

        evaluation.support_status = SupportStatus::Supported;
        match source.read_payload_pointer(&rule.source_pointer)? {
            None => {
                evaluation.payload_access = PayloadAccess::AttemptedUnavailable;
                evaluation.final_disposition = FinalDisposition::Unavailable;
                record_omission(rule, OmissionReason::Unavailable, &mut omissions);
            }
            Some(value) => {
                let (value, derivation) = execute_rule(configuration, rule, value)?;
                insert_pointer(&mut output, &rule.target_pointer, value.clone())?;
                evaluation.payload_access = PayloadAccess::Read;
                evaluation.final_disposition = FinalDisposition::Included;
                included.push(IncludedRule {
                    rule_id: rule.rule_id.clone(),
                    source_selector_id: rule.source_selector_id.clone(),
                    source_pointer: rule.source_pointer.clone(),
                    target_pointer: rule.target_pointer.clone(),
                    claim_refs: rule.claim_refs.clone(),
                });
                if let Some(derivation) = derivation {
                    derivations.push(derivation);
                }
            }
        }
        finish_evaluation(configuration, request, source, &mut evaluation)?;
        evaluations.push(evaluation);
    }

    let lossiness = compute_lossiness(&omissions, &derivations);
    let content = Value::Object(output);
    let content_bytes = canonical_json_bytes(&content).map_err(registry_fingerprint_error)?;
    let output = ProjectionOutput {
        schema: configuration.definition.target_schema.exact_pair(),
        content_ref: format!("projection-output/{}", request.request_id),
        content,
        content_fingerprint: DefinitionFingerprint::from_bytes(&content_bytes).to_string(),
    };
    let mut result = ProjectionResult {
        result_id: format!("projection-result/{}", request.request_id),
        request_ref: request.request_id.clone(),
        sources: request.sources.clone(),
        profile_configuration: request.profile_configuration.clone(),
        resolved_profile: request.resolved_profile.clone(),
        vocabulary: request.vocabulary.clone(),
        projection_definition: request.projection_definition.clone(),
        resolution_envelope: request.resolution_envelope.clone(),
        resolution_stack: request.resolution_stack.clone(),
        active_level: authority.active_level().to_owned(),
        dimension_values: authority.dimension_values().map(str::to_owned),
        dimension_ranks: authority_ranks,
        target_resolution_ranks: effective_ranks,
        purpose: request.purpose.clone(),
        surface: request.surface.clone(),
        operation: request.operation,
        currentness_validation,
        disclosure_evaluations: evaluations,
        lossiness,
        included,
        omissions,
        not_applicable,
        derivations,
        output,
        authority_effect: AuthorityEffect::None,
        result_fingerprint: String::new(),
    };
    let value = serde_json::to_value(&result).map_err(|_| {
        ProjectionError::new(
            ProjectionErrorKind::InvalidJson,
            "Projection result cannot be fingerprinted",
        )
    })?;
    result.result_fingerprint =
        fingerprint_without_field(&value, "result_fingerprint")?.to_string();
    Ok(result)
}

fn validate_request<A: ProjectionAuthorityAccess>(
    configuration: &ProjectionConfiguration,
    authority: &A,
    request: &ProjectionRequest,
) -> ProjectionResultValue<()> {
    if request.profile_configuration != configuration.profile.configuration_pair
        || request.resolved_profile != authority.resolved_profile_pair()
        || request.resolved_profile != configuration.profile.exact_pair
        || request.vocabulary != configuration.vocabulary.exact_pair
        || request.projection_definition != configuration.definition.exact_pair
        || request.resolution_envelope != authority.envelope_pair()
        || request.resolution_stack != authority.resolution_stack_pair()
        || request.resolution_stack != configuration.profile.resolution_stack
    {
        return Err(ProjectionError::new(
            ProjectionErrorKind::StaleBinding,
            "request authored profile, resolved profile, vocabulary, definition, stack, or envelope pair is stale",
        ));
    }
    if !matches!(
        request.operation,
        ProjectionOperation::Reveal | ProjectionOperation::Derive
    ) || !configuration
        .definition
        .allowed_operations
        .contains(&request.operation)
    {
        return Err(ProjectionError::new(
            ProjectionErrorKind::UnsupportedOperation,
            "request operation is unsupported",
        ));
    }
    if !configuration
        .definition
        .allowed_surfaces
        .contains(&request.surface)
        || !configuration.profile.surfaces.contains(&request.surface)
    {
        return Err(ProjectionError::new(
            ProjectionErrorKind::UnsupportedSurface,
            "request surface is unsupported",
        ));
    }
    if !configuration.profile.purposes.contains(&request.purpose) {
        return Err(ProjectionError::new(
            ProjectionErrorKind::InvalidPurpose,
            "request purpose is not registered",
        ));
    }
    if request
        .target_resolution_ranks
        .iter()
        .zip(authority.dimension_ranks())
        .any(|(target, authorized)| *target > authorized)
    {
        return Err(ProjectionError::new(
            ProjectionErrorKind::ResolutionEscalationRequired,
            "Projection cannot expand beyond the authorized Resolution envelope",
        ));
    }
    Ok(())
}

fn bind_sources<'a>(
    configuration: &ProjectionConfiguration,
    request: &ProjectionRequest,
    sources: &[&'a ProjectionSource],
) -> ProjectionResultValue<BTreeMap<String, &'a ProjectionSource>> {
    let mut selected = BTreeMap::new();
    for selector in &configuration.definition.source_selectors {
        let request_matches = request
            .sources
            .iter()
            .filter(|source| source.selector_id == selector.selector_id)
            .collect::<Vec<_>>();
        if request_matches.len() != 1 {
            return Err(ProjectionError::new(
                ProjectionErrorKind::Cardinality,
                "request source selector cardinality is not exactly one",
            ));
        }
        let source_matches = sources
            .iter()
            .copied()
            .filter(|source| source.exact_pair == request_matches[0].exact_pair)
            .collect::<Vec<_>>();
        if source_matches.len() != 1 {
            return Err(ProjectionError::new(
                ProjectionErrorKind::Cardinality,
                "source identity cardinality is not exactly one",
            ));
        }
        let source = source_matches[0];
        if source.document.source_kind != selector.source_kind
            || source.document.configured_kind_ref != selector.configured_kind_ref
            || source.document.capability != selector.capability
            || source.document.schema != selector.source_schema
        {
            return Err(ProjectionError::new(
                ProjectionErrorKind::StaleBinding,
                "source configured-kind capability or schema pair is stale",
            ));
        }
        if let Some(state_fingerprint) = &request_matches[0].state_fingerprint {
            if DefinitionFingerprint::parse(state_fingerprint).is_err()
                || source.document.state_fingerprint.as_deref() != Some(state_fingerprint)
            {
                return Err(ProjectionError::new(
                    ProjectionErrorKind::StaleBinding,
                    "selected source state identity is stale",
                ));
            }
        }
        selected.insert(selector.selector_id.clone(), source);
    }
    if request.sources.len() != selected.len() || sources.len() != selected.len() {
        return Err(ProjectionError::new(
            ProjectionErrorKind::Cardinality,
            "request contains an unbound or duplicate source",
        ));
    }
    Ok(selected)
}

fn validate_selected_source_pair_relations(
    configuration: &ProjectionConfiguration,
    request: &ProjectionRequest,
    selected: &BTreeMap<String, &ProjectionSource>,
) -> ProjectionResultValue<()> {
    for requirement in &configuration.definition.source_pair_requirements {
        let current = selected
            .get(requirement.current_selector_id.as_str())
            .expect("validated source-pair current selector");
        let derived = selected
            .get(requirement.derived_selector_id.as_str())
            .expect("validated source-pair derived selector");
        let current_selection = request
            .sources
            .iter()
            .find(|selection| selection.selector_id == requirement.current_selector_id)
            .expect("validated source-pair current request selection");
        if current.document.state_fingerprint.is_none()
            || current_selection.state_fingerprint != current.document.state_fingerprint
        {
            return Err(ProjectionError::new(
                ProjectionErrorKind::StaleBinding,
                "source-pair current state identity is absent or stale",
            ));
        }
        let dependencies = derived
            .document
            .source_dependencies
            .iter()
            .filter(|dependency| dependency.role == requirement.dependency_role)
            .collect::<Vec<_>>();
        if dependencies.len() != 1 {
            return Err(ProjectionError::new(
                ProjectionErrorKind::Cardinality,
                "source-pair dependency role cardinality is not exactly one",
            ));
        }
        let dependency = dependencies[0];
        if dependency.source != current.exact_pair
            || current.document.state_fingerprint.as_deref()
                != Some(dependency.state_fingerprint.as_str())
        {
            return Err(ProjectionError::new(
                ProjectionErrorKind::StaleBinding,
                "source-pair dependency does not bind the selected current source identity",
            ));
        }
    }
    Ok(())
}

fn validate_selected_source_semantics(
    configuration: &ProjectionConfiguration,
    selected: &BTreeMap<String, &ProjectionSource>,
) -> ProjectionResultValue<()> {
    for selector in &configuration.definition.source_selectors {
        let source = selected
            .get(selector.selector_id.as_str())
            .expect("exact source binding");
        require_trusted_pair(&source.document.capability, "selected source capability")?;
        require_trusted_pair(&source.document.schema, "selected source schema")?;
        let trusted_schema = trusted_semantic_definition(&source.document.schema.reference)
            .expect("validated trusted source schema");
        let trusted_pointers: BTreeMap<String, String> = serde_json::from_value(
            trusted_schema
                .get("pointer_types")
                .cloned()
                .expect("trusted source schema pointer definitions"),
        )
        .map_err(|_| {
            ProjectionError::new(
                ProjectionErrorKind::StaleBinding,
                "trusted source schema pointer definitions are invalid",
            )
        })?;
        if source.document.declared_pointers != trusted_pointers {
            return Err(ProjectionError::new(
                ProjectionErrorKind::StaleBinding,
                "selected source pointer definitions drift from the trusted source schema",
            ));
        }
    }
    Ok(())
}

fn validate_request_currentness<O>(
    configuration: &ProjectionConfiguration,
    request: &ProjectionRequest,
    selected: &BTreeMap<String, &ProjectionSource>,
    observe_live: &O,
) -> ProjectionResultValue<CurrentnessValidation>
where
    O: Fn(&LiveCurrentnessQuery) -> ProjectionResultValue<LiveCurrentnessObservation>,
{
    let requirements = &configuration.definition.currentness_requirements;
    if request.currentness.mode != requirements.mode
        || request.currentness.revision_basis != requirements.revision_basis
    {
        return Err(ProjectionError::new(
            ProjectionErrorKind::InvalidCurrentness,
            "request currentness mode or basis differs from the definition",
        ));
    }
    match requirements.mode {
        CurrentnessMode::None => {
            if !request.currentness.expected_family_revisions.is_empty() {
                return Err(ProjectionError::new(
                    ProjectionErrorKind::InvalidCurrentness,
                    "none currentness contains expected revisions",
                ));
            }
            Ok(CurrentnessValidation {
                mode: CurrentnessMode::None,
                revision_basis: None,
                checks: Vec::new(),
            })
        }
        CurrentnessMode::ExactRevisionCheck => {
            if request.currentness.expected_family_revisions.len() != requirements.families.len() {
                return Err(ProjectionError::new(
                    ProjectionErrorKind::InvalidCurrentness,
                    "captured currentness family cardinality is not exact",
                ));
            }
            for (selector_id, source) in selected {
                let requirement_count = requirements
                    .families
                    .iter()
                    .filter(|requirement| requirement.selector_id == *selector_id)
                    .count();
                if requirement_count > 0
                    && source.document.captured_family_revisions.len() != requirement_count
                {
                    return Err(ProjectionError::new(
                        ProjectionErrorKind::InvalidCurrentness,
                        "selected source captured-family closure is incomplete or contains an extra family",
                    ));
                }
            }
            let mut checks = Vec::new();
            for requirement in &requirements.families {
                let expected = request
                    .currentness
                    .expected_family_revisions
                    .iter()
                    .find(|expected| expected.family == requirement.family)
                    .ok_or_else(|| {
                        ProjectionError::new(
                            ProjectionErrorKind::InvalidCurrentness,
                            "captured currentness family is absent",
                        )
                    })?;
                let source = selected
                    .get(requirement.selector_id.as_str())
                    .ok_or_else(|| {
                        ProjectionError::new(
                            ProjectionErrorKind::InvalidCurrentness,
                            "captured currentness selector is absent",
                        )
                    })?;
                let captured = source
                    .document
                    .captured_family_revisions
                    .iter()
                    .find(|captured| captured.family == requirement.family)
                    .ok_or_else(|| {
                        ProjectionError::new(
                            ProjectionErrorKind::InvalidCurrentness,
                            "source captured currentness family is absent",
                        )
                    })?;
                if expected.selector_id != requirement.selector_id
                    || expected.family != requirement.family
                    || expected.adapter != requirement.adapter
                    || captured.family != requirement.family
                    || captured.adapter != requirement.adapter
                    || expected.family_revision != captured.family_revision
                    || expected.slots.len() != requirement.slots.len()
                    || captured.slots.len() != requirement.slots.len()
                {
                    return Err(ProjectionError::new(
                        ProjectionErrorKind::InvalidCurrentness,
                        "captured currentness family/adapter/value closure is stale",
                    ));
                }
                let live = observe_live(&LiveCurrentnessQuery {
                    selector_id: requirement.selector_id.clone(),
                    source: source.exact_pair(),
                    family: requirement.family.clone(),
                    adapter: requirement.adapter.clone(),
                    slots: requirement.slots.clone(),
                })?;
                if live.selector_id != requirement.selector_id
                    || live.source != source.exact_pair()
                    || live.family != requirement.family
                    || live.adapter != requirement.adapter
                    || live.family_revision != expected.family_revision
                    || live.family_revision != captured.family_revision
                    || live.slots.len() != requirement.slots.len()
                {
                    return Err(ProjectionError::new(
                        ProjectionErrorKind::InvalidCurrentness,
                        "independent live family observation does not equal the bound captured tuple",
                    ));
                }
                let expected_slots = expected
                    .slots
                    .iter()
                    .map(|slot| (slot.slot.clone(), slot.revision.clone()))
                    .collect::<BTreeMap<_, _>>();
                for slot in &requirement.slots {
                    let expected_revision = expected_slots.get(slot.as_str()).ok_or_else(|| {
                        ProjectionError::new(
                            ProjectionErrorKind::InvalidCurrentness,
                            "captured currentness slot is absent or duplicated",
                        )
                    })?;
                    let captured_revision = captured.slots.get(slot).ok_or_else(|| {
                        ProjectionError::new(
                            ProjectionErrorKind::InvalidCurrentness,
                            "source captured currentness slot is absent",
                        )
                    })?;
                    if expected_revision != captured_revision {
                        return Err(ProjectionError::new(
                            ProjectionErrorKind::InvalidCurrentness,
                            "captured currentness slot value is stale",
                        ));
                    }
                    let observed_revision = live.slots.get(slot).ok_or_else(|| {
                        ProjectionError::new(
                            ProjectionErrorKind::InvalidCurrentness,
                            "independent live currentness slot is absent",
                        )
                    })?;
                    if observed_revision != expected_revision
                        || observed_revision != captured_revision
                    {
                        return Err(ProjectionError::new(
                            ProjectionErrorKind::InvalidCurrentness,
                            "independent live slot observation does not equal the bound captured tuple",
                        ));
                    }
                }
                checks.push(CurrentnessCheck {
                    selector_id: requirement.selector_id.clone(),
                    family: requirement.family.clone(),
                    adapter: requirement.adapter.clone(),
                    expected_revision: expected.family_revision.clone(),
                    captured_revision: captured.family_revision.clone(),
                    observed_revision: live.family_revision.clone(),
                    expected_slots,
                    captured_slots: captured.slots.clone(),
                    observed_slots: live.slots.clone(),
                    passed: true,
                });
            }
            Ok(CurrentnessValidation {
                mode: CurrentnessMode::ExactRevisionCheck,
                revision_basis: Some("captured_revision".to_owned()),
                checks,
            })
        }
    }
}

fn empty_evaluation(
    configuration: &ProjectionConfiguration,
    rule: &FieldRule,
    ordinal: usize,
) -> DisclosureEvaluation {
    DisclosureEvaluation {
        rule_id: rule.rule_id.clone(),
        definition_rule_ordinal: ordinal,
        minimum_resolution_outcome: MinimumResolutionOutcome::Sufficient,
        upstream_redaction_check: UpstreamRedactionCheck::NotEvaluated,
        upstream_redaction_disposition: None,
        policy_evaluation: PolicyEvaluation::NotEvaluated,
        matched_policy_rule_ids: Vec::new(),
        policy_action: EvaluatedPolicyAction::NotEvaluated,
        support_evaluator: configuration
            .definition
            .support_evaluator
            .exact_pair
            .clone(),
        support_status: SupportStatus::NotEvaluated,
        support_reason: None,
        payload_access: PayloadAccess::NotAttempted,
        final_disposition: FinalDisposition::OutOfResolution,
        evaluation_fingerprint: String::new(),
    }
}

fn covering_upstream_redaction<'a>(
    source: &'a ProjectionSource,
    pointer: &str,
) -> Option<&'a UpstreamRedaction> {
    source
        .document
        .upstream_redactions
        .iter()
        .find(|redaction| {
            matches!(
                redaction.action.as_str(),
                "omit" | "fingerprint_only" | "artifact_ref_only" | "redacted_summary"
            ) && pointer_is_same_or_descendant(pointer, &redaction.original_pointer)
                && redaction.retained_pointer.as_deref() != Some(pointer)
        })
}

fn pointer_is_same_or_descendant(pointer: &str, ancestor: &str) -> bool {
    pointer == ancestor
        || pointer
            .strip_prefix(ancestor)
            .is_some_and(|suffix| suffix.starts_with('/'))
}

fn evaluate_disclosure_policy(
    policy: &DisclosurePolicy,
    source_kind: &str,
    _source_pointer: &str,
    classification: &str,
) -> (PolicyEvaluation, Vec<String>, EvaluatedPolicyAction) {
    let matched = policy
        .rules
        .iter()
        .filter(|rule| {
            rule.disclosure_classifications
                .iter()
                .any(|candidate| candidate == classification)
                && rule
                    .source_kinds
                    .iter()
                    .any(|candidate| candidate == source_kind)
        })
        .collect::<Vec<_>>();
    if matched.is_empty() {
        return (
            PolicyEvaluation::UnmatchedDefault,
            Vec::new(),
            EvaluatedPolicyAction::Redact,
        );
    }
    let action = if matched
        .iter()
        .any(|rule| rule.action == PolicyAction::Redact)
    {
        EvaluatedPolicyAction::Redact
    } else {
        EvaluatedPolicyAction::Allow
    };
    (
        PolicyEvaluation::Matched,
        matched.iter().map(|rule| rule.rule_id.clone()).collect(),
        action,
    )
}

fn evaluate_support(
    configuration: &ProjectionConfiguration,
    selector: &SourceSelector,
    source: &ProjectionSource,
    rule: &FieldRule,
) -> Option<SupportReason> {
    let evaluator = &configuration.definition.support_evaluator;
    let derivation = rule.derivation.as_ref().and_then(|pair| {
        configuration
            .definition
            .derivations
            .iter()
            .find(|definition| definition.reference == pair.reference)
    });
    let failures = [
        (
            SupportReason::SourceKindUnsupported,
            !evaluator
                .supported_source_kinds
                .contains(&source.document.source_kind),
        ),
        (
            SupportReason::SourceSchemaUnregistered,
            source.document.schema != selector.source_schema,
        ),
        (
            SupportReason::SourcePointerMissing,
            !source
                .document
                .declared_pointers
                .contains_key(&rule.source_pointer),
        ),
        (
            SupportReason::SourcePointerTypeIncompatible,
            source.document.declared_pointers.get(&rule.source_pointer)
                != Some(&rule.source_pointer_schema),
        ),
        (
            SupportReason::TargetSchemaUnregistered,
            configuration.definition.target_schema.reference.is_empty(),
        ),
        (
            SupportReason::TargetPointerMissing,
            !configuration
                .definition
                .target_schema
                .pointer_types
                .contains_key(&rule.target_pointer),
        ),
        (
            SupportReason::TargetPointerTypeIncompatible,
            configuration
                .definition
                .target_schema
                .pointer_types
                .get(&rule.target_pointer)
                != Some(&rule.target_pointer_schema),
        ),
        (
            SupportReason::DerivationUnregistered,
            rule.operation == ProjectionOperation::Derive && derivation.is_none(),
        ),
        (
            SupportReason::DerivationIoIncompatible,
            derivation.is_some_and(|derivation| {
                derivation.input_type != rule.source_pointer_schema
                    || derivation.output_type != rule.target_pointer_schema
            }),
        ),
    ];
    evaluator
        .unsupported_reason_precedence
        .iter()
        .find(|reason| {
            failures
                .iter()
                .any(|(candidate, failed)| candidate == *reason && *failed)
        })
        .copied()
}

fn execute_rule(
    configuration: &ProjectionConfiguration,
    rule: &FieldRule,
    input: Value,
) -> ProjectionResultValue<(Value, Option<DerivationRecord>)> {
    if rule.operation == ProjectionOperation::Reveal {
        return Ok((input, None));
    }
    let pair = rule.derivation.as_ref().ok_or_else(|| {
        ProjectionError::new(
            ProjectionErrorKind::InvalidDefinition,
            "derive rule has no exact derivation",
        )
    })?;
    let definition = configuration
        .definition
        .derivations
        .iter()
        .find(|definition| definition.exact_pair() == *pair)
        .ok_or_else(|| {
            ProjectionError::new(
                ProjectionErrorKind::InvalidDefinition,
                "derive rule derivation is not allowlisted",
            )
        })?;
    let input_fingerprint = DefinitionFingerprint::from_json_value(&input)
        .map_err(registry_fingerprint_error)?
        .to_string();
    let output = match definition.algorithm {
        DerivationAlgorithm::CountArray => Value::from(
            input
                .as_array()
                .ok_or_else(|| {
                    ProjectionError::new(
                        ProjectionErrorKind::InvalidDefinition,
                        "count_array input is not an array",
                    )
                })?
                .len() as u64,
        ),
        DerivationAlgorithm::NormalizedSummary => Value::String(
            input
                .as_array()
                .ok_or_else(|| {
                    ProjectionError::new(
                        ProjectionErrorKind::InvalidDefinition,
                        "normalized_summary input is not an array",
                    )
                })?
                .iter()
                .map(|value| value.as_str().unwrap_or_default().trim().to_lowercase())
                .collect::<Vec<_>>()
                .join("; "),
        ),
    };
    let output_fingerprint = DefinitionFingerprint::from_json_value(&output)
        .map_err(registry_fingerprint_error)?
        .to_string();
    Ok((
        output,
        Some(DerivationRecord {
            rule_id: rule.rule_id.clone(),
            derivation: pair.clone(),
            input_fingerprints: vec![input_fingerprint],
            output_fingerprint,
            lossy: definition.lossy,
        }),
    ))
}

fn insert_pointer(
    output: &mut Map<String, Value>,
    pointer: &str,
    value: Value,
) -> ProjectionResultValue<()> {
    let segments = pointer
        .strip_prefix('/')
        .ok_or_else(|| {
            ProjectionError::new(
                ProjectionErrorKind::InvalidDefinition,
                "target pointer is not absolute",
            )
        })?
        .split('/')
        .map(|segment| segment.replace("~1", "/").replace("~0", "~"))
        .collect::<Vec<_>>();
    if segments.len() != 1 || segments[0].is_empty() || output.contains_key(&segments[0]) {
        return Err(ProjectionError::new(
            ProjectionErrorKind::InvalidDefinition,
            "private v1 target pointer is duplicated or not one exact object member",
        ));
    }
    output.insert(segments[0].clone(), value);
    Ok(())
}

fn record_omission(rule: &FieldRule, reason: OmissionReason, omissions: &mut Vec<OmittedRule>) {
    let proof_effect = if rule.required_for_result || !rule.claim_refs.is_empty() {
        ProofEffect::NotObserved
    } else {
        ProofEffect::None
    };
    omissions.push(OmittedRule {
        rule_id: rule.rule_id.clone(),
        source_selector_id: rule.source_selector_id.clone(),
        source_pointer: rule.source_pointer.clone(),
        target_pointer: rule.target_pointer.clone(),
        claim_refs: rule.claim_refs.clone(),
        reason,
        proof_effect,
    });
}

fn finish_evaluation(
    configuration: &ProjectionConfiguration,
    request: &ProjectionRequest,
    source: &ProjectionSource,
    evaluation: &mut DisclosureEvaluation,
) -> ProjectionResultValue<()> {
    let value = serde_json::to_value(&*evaluation).map_err(|_| {
        ProjectionError::new(
            ProjectionErrorKind::InvalidJson,
            "disclosure evaluation cannot be fingerprinted",
        )
    })?;
    let preimage = json!({
        "definition": configuration.definition.exact_pair,
        "envelope": request.resolution_envelope,
        "evaluation": value.as_object().map(|object| {
            let mut object = object.clone();
            object.remove("evaluation_fingerprint");
            Value::Object(object)
        }).unwrap_or(Value::Null),
        "source": source.exact_pair,
    });
    evaluation.evaluation_fingerprint = DefinitionFingerprint::from_json_value(&preimage)
        .map_err(registry_fingerprint_error)?
        .to_string();
    Ok(())
}

fn compute_lossiness(
    omissions: &[OmittedRule],
    derivations: &[DerivationRecord],
) -> ProjectionLossiness {
    if omissions
        .iter()
        .any(|omission| omission.reason == OmissionReason::Redacted)
    {
        ProjectionLossiness::Redacted
    } else if omissions.iter().any(|omission| {
        matches!(
            omission.reason,
            OmissionReason::Unavailable | OmissionReason::Unsupported
        )
    }) {
        ProjectionLossiness::Partial
    } else if omissions
        .iter()
        .any(|omission| omission.reason == OmissionReason::OutOfResolution)
        || derivations.iter().any(|derivation| derivation.lossy)
    {
        ProjectionLossiness::Collapsed
    } else {
        ProjectionLossiness::Lossless
    }
}

#[cfg(test)]
mod tests;
