use crate::definition_identity::canonical_json_bytes;
use crate::{parse_schema_json, DefinitionFingerprint, ExactDefinitionRef};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

const POLICY_SCHEMA_ID: &str = "handbook.snapshot-capture-policy";
const POLICY_SCHEMA_VERSION: &str = "1.0";
const MAX_CAPTURE_RETRIES: u8 = 3;
const MAX_WINDOW_COUNT: u32 = 1_000;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum PolicyErrorKind {
    MalformedPolicy,
    MalformedCaptureInput,
    InvalidExactPair,
    FingerprintMismatch,
    UnknownFamily,
    DuplicateTrigger,
    DuplicateHorizon,
    DuplicateSourceSlot,
    DuplicateWindow,
    UnknownTrigger,
    InvalidHorizon,
    DuplicateFamily,
    UnknownSourceSlot,
    UnknownWindow,
    MissingFamily,
    MissingSourceSlot,
    MissingWindow,
    ScopeExpansion,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct PolicyError {
    kind: PolicyErrorKind,
}

impl PolicyError {
    fn new(kind: PolicyErrorKind) -> Self {
        Self { kind }
    }

    pub(super) fn kind(&self) -> PolicyErrorKind {
        self.kind
    }
}

#[derive(Clone, Debug)]
pub(super) struct SnapshotCapturePolicy {
    reference: ExactDefinitionRef,
    fingerprint: DefinitionFingerprint,
    triggers: BTreeSet<Trigger>,
    horizons: BTreeSet<MemoryHorizon>,
    families: BTreeMap<String, SourceFamily>,
    canonical_value: Value,
}

impl SnapshotCapturePolicy {
    pub(super) fn reference(&self) -> &str {
        self.reference.as_str()
    }

    pub(super) fn fingerprint(&self) -> &DefinitionFingerprint {
        &self.fingerprint
    }

    pub(super) fn canonical_bytes(&self) -> Result<Vec<u8>, PolicyError> {
        canonical_json_bytes(&self.canonical_value)
            .map_err(|_| PolicyError::new(PolicyErrorKind::MalformedPolicy))
    }
}

#[derive(Clone, Debug)]
pub(super) struct CaptureInput {
    fingerprint: DefinitionFingerprint,
}

impl CaptureInput {
    pub(super) fn capture_input_fingerprint(&self) -> &DefinitionFingerprint {
        &self.fingerprint
    }
}

#[derive(Clone, Debug)]
struct SourceFamily {
    slots: BTreeSet<String>,
    windows: BTreeMap<String, StaticWindow>,
}

#[derive(Clone, Debug)]
struct StaticWindow {
    source_slot: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
enum Trigger {
    SessionEnd,
    SessionStart,
}

impl Trigger {
    fn parse(value: &str) -> Result<Self, PolicyError> {
        match value {
            "session_end" => Ok(Self::SessionEnd),
            "session_start" => Ok(Self::SessionStart),
            _ => Err(PolicyError::new(PolicyErrorKind::UnknownTrigger)),
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
enum MemoryHorizon {
    Execution,
    Operation,
}

impl MemoryHorizon {
    fn parse(value: &str) -> Result<Self, PolicyError> {
        match value {
            "execution" => Ok(Self::Execution),
            "operation" => Ok(Self::Operation),
            _ => Err(PolicyError::new(PolicyErrorKind::InvalidHorizon)),
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
enum CursorMode {
    Exclusive,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
enum WindowOrdering {
    CanonicalQueueOrder,
    CompletedAtThenId,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
enum UnstableAction {
    PersistNonPromotable,
    Refuse,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct ExactPair {
    #[serde(rename = "ref")]
    reference: String,
    fingerprint: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct AuthoredPolicy {
    schema_id: String,
    schema_version: String,
    policy_id: String,
    policy_version: String,
    triggers: Vec<Trigger>,
    allowed_memory_horizons: Vec<MemoryHorizon>,
    state_families: BTreeMap<String, AuthoredSourceFamily>,
    comparison_contract: ExactPair,
    drift_rule_catalog: ExactPair,
    predecessor_rule: ExactPair,
    redaction_policy: ExactPair,
    consistency: AuthoredConsistency,
    retention_policy: ExactPair,
    policy_fingerprint: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct AuthoredSourceFamily {
    source_adapter: ExactPair,
    #[serde(default)]
    source_slots: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    composite_revision_rule: Option<ExactPair>,
    selected_fields: Vec<String>,
    #[serde(default)]
    windows: Vec<AuthoredStaticWindow>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct AuthoredStaticWindow {
    window_id: String,
    count: u32,
    source_slot: String,
    cursor_mode: CursorMode,
    ordering: WindowOrdering,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct AuthoredConsistency {
    retries: u8,
    bounded_skew_rule: ExactPair,
    unstable_action: UnstableAction,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct AuthoredCaptureInput {
    policy: ExactPair,
    trigger: String,
    memory_horizon: String,
    families: Vec<AuthoredCaptureFamily>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct AuthoredCaptureFamily {
    family: String,
    slots: Vec<AuthoredLiveSlot>,
    windows: Vec<AuthoredLiveWindow>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct AuthoredLiveSlot {
    slot: String,
    revision: ExactPair,
    cursor: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct AuthoredLiveWindow {
    window_id: String,
    slot: String,
    revision: ExactPair,
    cursor: Option<String>,
}

#[derive(Serialize)]
struct NormalizedCaptureInput {
    policy: ExactPair,
    trigger: Trigger,
    memory_horizon: MemoryHorizon,
    families: BTreeMap<String, NormalizedCaptureFamily>,
}

#[derive(Serialize)]
struct NormalizedCaptureFamily {
    slots: BTreeMap<String, AuthoredLiveSlot>,
    windows: BTreeMap<String, AuthoredLiveWindow>,
}

pub(super) fn load_policy(bytes: &[u8]) -> Result<SnapshotCapturePolicy, PolicyError> {
    let value =
        parse_schema_json(bytes).map_err(|_| PolicyError::new(PolicyErrorKind::MalformedPolicy))?;
    let mut authored: AuthoredPolicy = serde_json::from_value(value)
        .map_err(|_| PolicyError::new(PolicyErrorKind::MalformedPolicy))?;

    let reference = ExactDefinitionRef::new(&authored.policy_id, &authored.policy_version)
        .map_err(|_| PolicyError::new(PolicyErrorKind::InvalidExactPair))?;
    if authored.schema_id != POLICY_SCHEMA_ID || authored.schema_version != POLICY_SCHEMA_VERSION {
        return Err(PolicyError::new(PolicyErrorKind::MalformedPolicy));
    }

    let triggers = normalize_triggers(&mut authored.triggers)?;
    let horizons = normalize_horizons(&mut authored.allowed_memory_horizons)?;
    let families = normalize_families(&mut authored.state_families)?;
    validate_definition_pair(&authored.comparison_contract)?;
    validate_definition_pair(&authored.drift_rule_catalog)?;
    validate_definition_pair(&authored.predecessor_rule)?;
    validate_definition_pair(&authored.redaction_policy)?;
    validate_definition_pair(&authored.retention_policy)?;
    validate_definition_pair(&authored.consistency.bounded_skew_rule)?;
    if authored.consistency.retries > MAX_CAPTURE_RETRIES {
        return Err(PolicyError::new(PolicyErrorKind::ScopeExpansion));
    }

    let computed = fingerprint_without_policy_fingerprint(&authored)?;
    let supplied = DefinitionFingerprint::parse(&authored.policy_fingerprint)
        .map_err(|_| PolicyError::new(PolicyErrorKind::InvalidExactPair))?;
    if supplied != computed {
        return Err(PolicyError::new(PolicyErrorKind::FingerprintMismatch));
    }
    authored.policy_fingerprint = computed.to_string();
    let canonical_value = serde_json::to_value(authored)
        .map_err(|_| PolicyError::new(PolicyErrorKind::MalformedPolicy))?;

    Ok(SnapshotCapturePolicy {
        reference,
        fingerprint: computed,
        triggers,
        horizons,
        families,
        canonical_value,
    })
}

pub(super) fn validate_capture_input(
    policy: &SnapshotCapturePolicy,
    bytes: &[u8],
) -> Result<CaptureInput, PolicyError> {
    let value = parse_schema_json(bytes)
        .map_err(|_| PolicyError::new(PolicyErrorKind::MalformedCaptureInput))?;
    let capture: AuthoredCaptureInput = serde_json::from_value(value)
        .map_err(|_| PolicyError::new(PolicyErrorKind::MalformedCaptureInput))?;

    validate_definition_pair(&capture.policy)?;
    if capture.policy.reference != policy.reference()
        || capture.policy.fingerprint != policy.fingerprint().as_str()
    {
        return Err(PolicyError::new(PolicyErrorKind::ScopeExpansion));
    }
    let trigger = Trigger::parse(&capture.trigger)?;
    if !policy.triggers.contains(&trigger) {
        return Err(PolicyError::new(PolicyErrorKind::UnknownTrigger));
    }
    let horizon = MemoryHorizon::parse(&capture.memory_horizon)?;
    if !policy.horizons.contains(&horizon) {
        return Err(PolicyError::new(PolicyErrorKind::InvalidHorizon));
    }

    let mut families = BTreeMap::new();
    for family in capture.families {
        if families.contains_key(&family.family) {
            return Err(PolicyError::new(PolicyErrorKind::DuplicateFamily));
        }
        let definition = policy
            .families
            .get(&family.family)
            .ok_or_else(|| PolicyError::new(PolicyErrorKind::UnknownFamily))?;
        let normalized = validate_capture_family(definition, family)?;
        families.insert(normalized.0, normalized.1);
    }
    if families.len() != policy.families.len()
        || policy
            .families
            .keys()
            .any(|family| !families.contains_key(family))
    {
        return Err(PolicyError::new(PolicyErrorKind::MissingFamily));
    }

    let normalized = NormalizedCaptureInput {
        policy: capture.policy,
        trigger,
        memory_horizon: horizon,
        families,
    };
    let value = serde_json::to_value(normalized)
        .map_err(|_| PolicyError::new(PolicyErrorKind::MalformedCaptureInput))?;
    let fingerprint = DefinitionFingerprint::from_json_value(&value)
        .map_err(|_| PolicyError::new(PolicyErrorKind::MalformedCaptureInput))?;
    Ok(CaptureInput { fingerprint })
}

fn normalize_triggers(triggers: &mut Vec<Trigger>) -> Result<BTreeSet<Trigger>, PolicyError> {
    let values = triggers.iter().copied().collect::<BTreeSet<_>>();
    if values.len() != triggers.len() {
        return Err(PolicyError::new(PolicyErrorKind::DuplicateTrigger));
    }
    if values.is_empty() {
        return Err(PolicyError::new(PolicyErrorKind::MalformedPolicy));
    }
    *triggers = values.iter().copied().collect();
    Ok(values)
}

fn normalize_horizons(
    horizons: &mut Vec<MemoryHorizon>,
) -> Result<BTreeSet<MemoryHorizon>, PolicyError> {
    let values = horizons.iter().copied().collect::<BTreeSet<_>>();
    if values.len() != horizons.len() {
        return Err(PolicyError::new(PolicyErrorKind::DuplicateHorizon));
    }
    if values.is_empty() {
        return Err(PolicyError::new(PolicyErrorKind::MalformedPolicy));
    }
    *horizons = values.iter().copied().collect();
    Ok(values)
}

fn normalize_families(
    families: &mut BTreeMap<String, AuthoredSourceFamily>,
) -> Result<BTreeMap<String, SourceFamily>, PolicyError> {
    if families.is_empty() {
        return Err(PolicyError::new(PolicyErrorKind::MalformedPolicy));
    }
    let mut normalized = BTreeMap::new();
    for (family_id, family) in families.iter_mut() {
        let allowed_fields = allowed_fields(family_id)?;
        validate_definition_pair(&family.source_adapter)?;
        if family.source_adapter.reference != format!("handbook.snapshot-source.{family_id}@1.0.0")
        {
            return Err(PolicyError::new(PolicyErrorKind::ScopeExpansion));
        }

        let selected_fields =
            normalize_identifiers(&mut family.selected_fields, PolicyErrorKind::ScopeExpansion)?;
        if selected_fields
            .iter()
            .any(|field| !allowed_fields.contains(&field.as_str()))
        {
            return Err(PolicyError::new(PolicyErrorKind::ScopeExpansion));
        }
        if (family_id == "work") != selected_fields.is_empty() {
            return Err(PolicyError::new(PolicyErrorKind::ScopeExpansion));
        }

        let source_slots = normalize_identifiers(
            &mut family.source_slots,
            PolicyErrorKind::DuplicateSourceSlot,
        )?;
        let (slots, windows) = if family_id == "work" {
            if source_slots.len() < 2 {
                return Err(PolicyError::new(PolicyErrorKind::MissingSourceSlot));
            }
            let composite = family
                .composite_revision_rule
                .as_ref()
                .ok_or_else(|| PolicyError::new(PolicyErrorKind::MalformedPolicy))?;
            validate_definition_pair(composite)?;
            let windows = normalize_windows(&source_slots, &mut family.windows)?;
            (source_slots, windows)
        } else {
            if !source_slots.is_empty()
                || family.composite_revision_rule.is_some()
                || !family.windows.is_empty()
            {
                return Err(PolicyError::new(PolicyErrorKind::ScopeExpansion));
            }
            (BTreeSet::from([family_id.clone()]), BTreeMap::new())
        };
        normalized.insert(family_id.clone(), SourceFamily { slots, windows });
    }
    Ok(normalized)
}

fn normalize_windows(
    source_slots: &BTreeSet<String>,
    windows: &mut Vec<AuthoredStaticWindow>,
) -> Result<BTreeMap<String, StaticWindow>, PolicyError> {
    let mut normalized = BTreeMap::new();
    for window in windows.iter() {
        if !is_identifier(&window.window_id)
            || !source_slots.contains(&window.source_slot)
            || window.count == 0
            || window.count > MAX_WINDOW_COUNT
            || window.cursor_mode != CursorMode::Exclusive
        {
            return Err(PolicyError::new(PolicyErrorKind::ScopeExpansion));
        }
        if normalized
            .insert(
                window.window_id.clone(),
                StaticWindow {
                    source_slot: window.source_slot.clone(),
                },
            )
            .is_some()
        {
            return Err(PolicyError::new(PolicyErrorKind::DuplicateWindow));
        }
    }
    windows.sort_by(|left, right| left.window_id.cmp(&right.window_id));
    Ok(normalized)
}

fn validate_capture_family(
    definition: &SourceFamily,
    family: AuthoredCaptureFamily,
) -> Result<(String, NormalizedCaptureFamily), PolicyError> {
    let mut slots = BTreeMap::new();
    for slot in family.slots {
        if !is_identifier(&slot.slot) {
            return Err(PolicyError::new(PolicyErrorKind::UnknownSourceSlot));
        }
        validate_live_pair(&slot.revision)?;
        validate_cursor(slot.cursor.as_deref())?;
        if !definition.slots.contains(&slot.slot) {
            return Err(PolicyError::new(PolicyErrorKind::UnknownSourceSlot));
        }
        if slots.insert(slot.slot.clone(), slot).is_some() {
            return Err(PolicyError::new(PolicyErrorKind::DuplicateSourceSlot));
        }
    }
    if slots.len() != definition.slots.len()
        || definition
            .slots
            .iter()
            .any(|slot| !slots.contains_key(slot))
    {
        return Err(PolicyError::new(PolicyErrorKind::MissingSourceSlot));
    }

    let mut windows = BTreeMap::new();
    for window in family.windows {
        let expected = definition
            .windows
            .get(&window.window_id)
            .ok_or_else(|| PolicyError::new(PolicyErrorKind::UnknownWindow))?;
        if window.slot != expected.source_slot {
            return Err(PolicyError::new(PolicyErrorKind::ScopeExpansion));
        }
        validate_live_pair(&window.revision)?;
        validate_cursor(window.cursor.as_deref())?;
        let slot = slots
            .get(&window.slot)
            .ok_or_else(|| PolicyError::new(PolicyErrorKind::UnknownSourceSlot))?;
        if slot.revision != window.revision || slot.cursor != window.cursor {
            return Err(PolicyError::new(PolicyErrorKind::ScopeExpansion));
        }
        if windows.insert(window.window_id.clone(), window).is_some() {
            return Err(PolicyError::new(PolicyErrorKind::DuplicateWindow));
        }
    }
    if windows.len() != definition.windows.len()
        || definition
            .windows
            .keys()
            .any(|window| !windows.contains_key(window))
    {
        return Err(PolicyError::new(PolicyErrorKind::MissingWindow));
    }

    Ok((family.family, NormalizedCaptureFamily { slots, windows }))
}

fn fingerprint_without_policy_fingerprint(
    policy: &AuthoredPolicy,
) -> Result<DefinitionFingerprint, PolicyError> {
    let mut value = serde_json::to_value(policy)
        .map_err(|_| PolicyError::new(PolicyErrorKind::MalformedPolicy))?;
    value
        .as_object_mut()
        .ok_or_else(|| PolicyError::new(PolicyErrorKind::MalformedPolicy))?
        .remove("policy_fingerprint")
        .ok_or_else(|| PolicyError::new(PolicyErrorKind::MalformedPolicy))?;
    DefinitionFingerprint::from_json_value(&value)
        .map_err(|_| PolicyError::new(PolicyErrorKind::MalformedPolicy))
}

fn validate_definition_pair(pair: &ExactPair) -> Result<(), PolicyError> {
    ExactDefinitionRef::parse(&pair.reference)
        .map_err(|_| PolicyError::new(PolicyErrorKind::InvalidExactPair))?;
    DefinitionFingerprint::parse(&pair.fingerprint)
        .map_err(|_| PolicyError::new(PolicyErrorKind::InvalidExactPair))?;
    Ok(())
}

fn validate_live_pair(pair: &ExactPair) -> Result<(), PolicyError> {
    if pair.reference.is_empty()
        || pair.reference.len() > 255
        || !pair.reference.is_ascii()
        || pair
            .reference
            .bytes()
            .any(|byte| byte.is_ascii_control() || byte.is_ascii_whitespace())
    {
        return Err(PolicyError::new(PolicyErrorKind::InvalidExactPair));
    }
    DefinitionFingerprint::parse(&pair.fingerprint)
        .map_err(|_| PolicyError::new(PolicyErrorKind::InvalidExactPair))?;
    Ok(())
}

fn validate_cursor(cursor: Option<&str>) -> Result<(), PolicyError> {
    if let Some(cursor) = cursor {
        if cursor.is_empty()
            || cursor.len() > 1_024
            || !cursor.is_ascii()
            || cursor.bytes().any(|byte| byte.is_ascii_control())
        {
            return Err(PolicyError::new(PolicyErrorKind::ScopeExpansion));
        }
    }
    Ok(())
}

fn normalize_identifiers(
    values: &mut Vec<String>,
    duplicate_kind: PolicyErrorKind,
) -> Result<BTreeSet<String>, PolicyError> {
    let mut normalized = BTreeSet::new();
    for value in values.iter() {
        if !is_identifier(value) {
            return Err(PolicyError::new(PolicyErrorKind::ScopeExpansion));
        }
        if !normalized.insert(value.clone()) {
            return Err(PolicyError::new(duplicate_kind));
        }
    }
    *values = normalized.iter().cloned().collect();
    Ok(normalized)
}

fn allowed_fields(family: &str) -> Result<&'static [&'static str], PolicyError> {
    match family {
        "evidence" => Ok(&["latest_gate_refs"]),
        "git" => Ok(&["diff_stats", "full_diff_artifact_ref_only", "paths"]),
        "handbook" => Ok(&["artifact_fingerprints", "contract_state", "profile"]),
        "session" => Ok(&["orchestration_state"]),
        "work" => Ok(&[]),
        _ => Err(PolicyError::new(PolicyErrorKind::UnknownFamily)),
    }
}

fn is_identifier(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 64
        && value.as_bytes()[0].is_ascii_lowercase()
        && value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
}
