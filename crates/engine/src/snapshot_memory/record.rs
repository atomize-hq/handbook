use super::policy::{CaptureInput, SnapshotCapturePolicy};
use crate::definition_identity::canonical_json_bytes;
use crate::{parse_schema_json, DefinitionFingerprint, ExactDefinitionRef};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};

const SNAPSHOT_SCHEMA_ID: &str = "handbook.context-memory-snapshot";
const SNAPSHOT_SCHEMA_VERSION: &str = "1.0";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum RecordErrorKind {
    MalformedRecord,
    InvalidExactPair,
    InvalidSnapshotIdentity,
    UnknownFamily,
    DuplicateFamily,
    DuplicateExclusion,
    MissingFamily,
    InvalidFamilyProvenance,
    InvalidWindow,
    InvalidCaptureProvenance,
    InvalidDerivedConsistency,
    InvalidPredecessor,
    FingerprintMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct RecordError {
    kind: RecordErrorKind,
}

impl RecordError {
    fn new(kind: RecordErrorKind) -> Self {
        Self { kind }
    }

    pub(super) fn kind(&self) -> RecordErrorKind {
        self.kind
    }
}

#[derive(Clone, Debug)]
pub(super) struct ContextMemorySnapshot {
    value: Value,
    state_fingerprint: DefinitionFingerprint,
    record_fingerprint: DefinitionFingerprint,
}

impl ContextMemorySnapshot {
    pub(super) fn state_fingerprint(&self) -> &DefinitionFingerprint {
        &self.state_fingerprint
    }

    pub(super) fn record_fingerprint(&self) -> &DefinitionFingerprint {
        &self.record_fingerprint
    }

    pub(super) fn normalized_value(&self) -> &Value {
        &self.value
    }

    pub(super) fn normalized_bytes(&self) -> Result<Vec<u8>, RecordError> {
        canonical_json_bytes(&self.value)
            .map_err(|_| RecordError::new(RecordErrorKind::MalformedRecord))
    }
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
struct AuthoredSnapshot {
    schema_id: String,
    schema_version: String,
    snapshot_id: String,
    capture: CaptureMetadata,
    repository_identity: RepositoryIdentity,
    boundary_stream_ref: String,
    boundary_sequence: u64,
    resolved_profile: ExactPair,
    context_resolution_envelope: ExactPair,
    family_observations: Vec<AuthoredFamilyObservation>,
    excluded_families: Vec<AuthoredFamilyExclusion>,
    previous_snapshot: Option<AuthoredPreviousSnapshot>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct CaptureMetadata {
    trigger: CaptureTrigger,
    started_at: String,
    completed_at: String,
    retry_count: u8,
    input: Value,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
enum CaptureTrigger {
    SessionEnd,
    SessionStart,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
enum CaptureMemoryHorizon {
    Execution,
    Operation,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct BoundCaptureInput {
    policy: ExactPair,
    trigger: CaptureTrigger,
    memory_horizon: CaptureMemoryHorizon,
    families: BTreeMap<String, BoundCaptureFamily>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct BoundCaptureFamily {
    slots: BTreeMap<String, BoundCaptureSlot>,
    windows: BTreeMap<String, BoundCaptureWindow>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct BoundCaptureSlot {
    slot: String,
    revision: ExactPair,
    cursor: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct BoundCaptureWindow {
    window_id: String,
    slot: String,
    revision: ExactPair,
    cursor: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct AuthoredPreviousSnapshot {
    #[serde(rename = "ref")]
    reference: String,
    record_fingerprint: String,
    boundary_sequence: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct RepositoryIdentity {
    repository_id: String,
    workspace_id: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct AuthoredFamilyObservation {
    family: String,
    source_adapter: ExactPair,
    pre_revision: ExactPair,
    captured_revision: ExactPair,
    post_revision: ExactPair,
    source_slot_revisions: Vec<SourceSlotRevision>,
    windows: Vec<WindowCapture>,
    bound_evaluation: Option<AuthoredBoundedEvidence>,
    payload_fingerprint: String,
    payload: Value,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct AuthoredBoundedEvidence {
    rule: ExactPair,
    evaluated_revisions: AuthoredEvaluatedRevisions,
    outcome: BoundedOutcome,
    evidence_fingerprint: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct AuthoredEvaluatedRevisions {
    family_revision: ExactPair,
    source_slot_revisions: BTreeMap<String, ExactPair>,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
enum BoundedOutcome {
    WithinBound,
    OutOfBound,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct SourceSlotRevision {
    source_slot: String,
    pre_revision: ExactPair,
    captured_revision: ExactPair,
    post_revision: ExactPair,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct WindowCapture {
    window_id: String,
    source_slot: String,
    source_revision: ExactPair,
    cursor: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct AuthoredFamilyExclusion {
    family: String,
    reason: ExclusionReason,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
enum ExclusionReason {
    Unavailable,
    Unsupported,
    Redacted,
    Unstable,
}

#[derive(Debug, Deserialize)]
struct PolicyView {
    policy_id: String,
    policy_version: String,
    policy_fingerprint: String,
    state_families: BTreeMap<String, PolicyFamily>,
    consistency: PolicyConsistency,
}

#[derive(Debug, Deserialize)]
struct PolicyConsistency {
    bounded_skew_rule: ExactPair,
    unstable_action: PolicyUnstableAction,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum PolicyUnstableAction {
    PersistNonPromotable,
    Refuse,
}

#[derive(Debug, Deserialize)]
struct PolicyFamily {
    source_adapter: ExactPair,
    #[serde(default)]
    source_slots: Vec<String>,
    #[serde(default)]
    windows: Vec<PolicyWindow>,
}

#[derive(Debug, Deserialize)]
struct PolicyWindow {
    window_id: String,
    source_slot: String,
}

pub(super) fn build_snapshot(
    policy: &SnapshotCapturePolicy,
    capture_input: &CaptureInput,
    history: &[&ContextMemorySnapshot],
    bytes: &[u8],
) -> Result<ContextMemorySnapshot, RecordError> {
    let value =
        parse_schema_json(bytes).map_err(|_| RecordError::new(RecordErrorKind::MalformedRecord))?;
    let authored: AuthoredSnapshot = serde_json::from_value(value)
        .map_err(|_| RecordError::new(RecordErrorKind::MalformedRecord))?;
    let policy_view = policy_view(policy)?;
    validate_snapshot_header(&authored)?;
    let previous_snapshot = bind_predecessor(history, &authored)?;

    let mut observations = BTreeMap::new();
    for observation in authored.family_observations {
        let expected = policy_view
            .state_families
            .get(&observation.family)
            .ok_or_else(|| RecordError::new(RecordErrorKind::UnknownFamily))?;
        let observation = normalize_observation(expected, observation)?;
        if observations
            .insert(observation.family.clone(), observation)
            .is_some()
        {
            return Err(RecordError::new(RecordErrorKind::DuplicateFamily));
        }
    }

    let mut exclusions = BTreeMap::new();
    for exclusion in authored.excluded_families {
        if !policy_view.state_families.contains_key(&exclusion.family) {
            return Err(RecordError::new(RecordErrorKind::UnknownFamily));
        }
        if observations.contains_key(&exclusion.family) {
            return Err(RecordError::new(RecordErrorKind::DuplicateFamily));
        }
        if exclusions
            .insert(exclusion.family.clone(), exclusion)
            .is_some()
        {
            return Err(RecordError::new(RecordErrorKind::DuplicateExclusion));
        }
    }
    if observations.len() + exclusions.len() != policy_view.state_families.len()
        || policy_view
            .state_families
            .keys()
            .any(|family| !observations.contains_key(family) && !exclusions.contains_key(family))
    {
        return Err(RecordError::new(RecordErrorKind::MissingFamily));
    }

    let policy_pair = ExactPair {
        reference: policy.reference().to_owned(),
        fingerprint: policy.fingerprint().to_string(),
    };
    let observations: Vec<_> = observations.into_values().collect();
    let exclusions: Vec<_> = exclusions.into_values().collect();
    let capture_provenance = bind_capture_provenance(
        &authored.capture,
        capture_input,
        &policy_pair,
        &observations,
        &exclusions,
    )?;
    let derived_consistency =
        derive_snapshot_consistency(&policy_view, &observations, &exclusions)?;
    let state_value = json!({
        "schema_id": authored.schema_id,
        "schema_version": authored.schema_version,
        "repository_identity": authored.repository_identity,
        "policy": policy_pair,
        "capture_input": capture_provenance.state_input,
        "resolved_profile": authored.resolved_profile,
        "context_resolution_envelope": authored.context_resolution_envelope,
        "family_observations": observations,
        "excluded_families": exclusions,
    });
    let state_fingerprint = DefinitionFingerprint::from_json_value(&state_value)
        .map_err(|_| RecordError::new(RecordErrorKind::MalformedRecord))?;

    let record_value = json!({
        "schema_id": authored.schema_id,
        "schema_version": authored.schema_version,
        "snapshot_id": authored.snapshot_id,
        "capture": {
            "trigger": authored.capture.trigger,
            "started_at": authored.capture.started_at,
            "completed_at": authored.capture.completed_at,
            "retry_count": authored.capture.retry_count,
            "policy": policy_pair,
            "memory_horizon": capture_provenance.memory_horizon,
            "input": capture_provenance.input,
            "capture_input_fingerprint": capture_input.capture_input_fingerprint().to_string(),
            "consistency": snapshot_consistency_name(derived_consistency.consistency),
        },
        "repository_identity": authored.repository_identity,
        "boundary_stream_ref": authored.boundary_stream_ref,
        "boundary_sequence": authored.boundary_sequence,
        "resolved_profile": authored.resolved_profile,
        "context_resolution_envelope": authored.context_resolution_envelope,
        "family_observations": observations,
        "excluded_families": exclusions,
        "previous_snapshot": previous_snapshot,
        "admissibility": admissibility_name(derived_consistency.admissibility),
        "state_fingerprint": state_fingerprint.to_string(),
    });
    let record_fingerprint = DefinitionFingerprint::from_json_value(&record_value)
        .map_err(|_| RecordError::new(RecordErrorKind::MalformedRecord))?;
    let mut value = record_value;
    value
        .as_object_mut()
        .ok_or_else(|| RecordError::new(RecordErrorKind::MalformedRecord))?
        .insert(
            "record_fingerprint".to_owned(),
            Value::String(record_fingerprint.to_string()),
        );

    Ok(ContextMemorySnapshot {
        value,
        state_fingerprint,
        record_fingerprint,
    })
}

struct CaptureProvenance {
    input: Value,
    state_input: Value,
    memory_horizon: CaptureMemoryHorizon,
}

fn bind_capture_provenance(
    capture: &CaptureMetadata,
    capture_input: &CaptureInput,
    policy: &ExactPair,
    observations: &[AuthoredFamilyObservation],
    exclusions: &[AuthoredFamilyExclusion],
) -> Result<CaptureProvenance, RecordError> {
    let input: BoundCaptureInput = serde_json::from_value(capture.input.clone())
        .map_err(|_| RecordError::new(RecordErrorKind::InvalidCaptureProvenance))?;
    validate_definition_pair(&input.policy)?;
    if input.policy != *policy
        || input.trigger != capture.trigger
        || input.families.len() != observations.len() + exclusions.len()
    {
        return Err(RecordError::new(RecordErrorKind::InvalidCaptureProvenance));
    }

    for observation in observations {
        let family = input
            .families
            .get(&observation.family)
            .ok_or_else(|| RecordError::new(RecordErrorKind::InvalidCaptureProvenance))?;
        bind_capture_family(family, observation)?;
    }
    if input.families.keys().any(|family| {
        !observations
            .iter()
            .any(|observation| &observation.family == family)
            && !exclusions
                .iter()
                .any(|exclusion| &exclusion.family == family)
    }) {
        return Err(RecordError::new(RecordErrorKind::InvalidCaptureProvenance));
    }

    let input = serde_json::to_value(input)
        .map_err(|_| RecordError::new(RecordErrorKind::InvalidCaptureProvenance))?;
    let fingerprint = DefinitionFingerprint::from_json_value(&input)
        .map_err(|_| RecordError::new(RecordErrorKind::InvalidCaptureProvenance))?;
    if &fingerprint != capture_input.capture_input_fingerprint() {
        return Err(RecordError::new(RecordErrorKind::InvalidCaptureProvenance));
    }
    let mut state_input = input.clone();
    state_input
        .as_object_mut()
        .ok_or_else(|| RecordError::new(RecordErrorKind::InvalidCaptureProvenance))?
        .remove("trigger");
    let memory_horizon = serde_json::from_value(
        input
            .get("memory_horizon")
            .cloned()
            .ok_or_else(|| RecordError::new(RecordErrorKind::InvalidCaptureProvenance))?,
    )
    .map_err(|_| RecordError::new(RecordErrorKind::InvalidCaptureProvenance))?;
    Ok(CaptureProvenance {
        input,
        state_input,
        memory_horizon,
    })
}

fn bind_capture_family(
    capture: &BoundCaptureFamily,
    observation: &AuthoredFamilyObservation,
) -> Result<(), RecordError> {
    if capture.windows.len() != observation.windows.len() {
        return Err(RecordError::new(RecordErrorKind::InvalidCaptureProvenance));
    }
    if observation.source_slot_revisions.is_empty() {
        let Some((slot_name, slot)) = capture.slots.iter().next() else {
            return Err(RecordError::new(RecordErrorKind::InvalidCaptureProvenance));
        };
        if capture.slots.len() != 1 || slot.slot != *slot_name {
            return Err(RecordError::new(RecordErrorKind::InvalidCaptureProvenance));
        }
        validate_bound_capture_slot(slot)?;
        if slot.revision != observation.captured_revision {
            return Err(RecordError::new(RecordErrorKind::InvalidCaptureProvenance));
        }
    } else {
        if capture.slots.len() != observation.source_slot_revisions.len() {
            return Err(RecordError::new(RecordErrorKind::InvalidCaptureProvenance));
        }
        for source_slot in &observation.source_slot_revisions {
            let slot = capture
                .slots
                .get(&source_slot.source_slot)
                .ok_or_else(|| RecordError::new(RecordErrorKind::InvalidCaptureProvenance))?;
            if slot.slot != source_slot.source_slot {
                return Err(RecordError::new(RecordErrorKind::InvalidCaptureProvenance));
            }
            validate_bound_capture_slot(slot)?;
            if slot.revision != source_slot.captured_revision {
                return Err(RecordError::new(RecordErrorKind::InvalidCaptureProvenance));
            }
        }
    }

    for window in &observation.windows {
        let captured = capture
            .windows
            .get(&window.window_id)
            .ok_or_else(|| RecordError::new(RecordErrorKind::InvalidCaptureProvenance))?;
        if captured.window_id != window.window_id
            || captured.slot != window.source_slot
            || captured.revision != window.source_revision
            || captured.cursor != window.cursor
        {
            return Err(RecordError::new(RecordErrorKind::InvalidCaptureProvenance));
        }
        validate_bound_capture_window(captured)?;
    }
    Ok(())
}

fn validate_bound_capture_slot(slot: &BoundCaptureSlot) -> Result<(), RecordError> {
    if !is_identifier(&slot.slot) || !slot.cursor.as_deref().map_or(true, is_safe_text) {
        return Err(RecordError::new(RecordErrorKind::InvalidCaptureProvenance));
    }
    validate_live_pair(&slot.revision)
        .map_err(|_| RecordError::new(RecordErrorKind::InvalidCaptureProvenance))
}

fn validate_bound_capture_window(window: &BoundCaptureWindow) -> Result<(), RecordError> {
    if !is_identifier(&window.window_id)
        || !is_identifier(&window.slot)
        || !window.cursor.as_deref().map_or(true, is_safe_text)
    {
        return Err(RecordError::new(RecordErrorKind::InvalidCaptureProvenance));
    }
    validate_live_pair(&window.revision)
        .map_err(|_| RecordError::new(RecordErrorKind::InvalidCaptureProvenance))
}

fn derive_snapshot_consistency(
    policy: &PolicyView,
    observations: &[AuthoredFamilyObservation],
    exclusions: &[AuthoredFamilyExclusion],
) -> Result<consistency::DerivedConsistency, RecordError> {
    validate_definition_pair(&policy.consistency.bounded_skew_rule)?;
    let bounded_rule = consistency_revision(&policy.consistency.bounded_skew_rule)?;
    let consistency_policy = consistency::ConsistencyPolicy {
        selected_families: policy.state_families.keys().cloned().collect(),
        bounded_rule,
        unstable_action: match policy.consistency.unstable_action {
            PolicyUnstableAction::PersistNonPromotable => {
                consistency::UnstableAction::PersistNonPromotable
            }
            PolicyUnstableAction::Refuse => consistency::UnstableAction::Refuse,
        },
    };
    let observations = observations
        .iter()
        .map(consistency_observation)
        .collect::<Result<Vec<_>, _>>()?;
    let exclusions = exclusions
        .iter()
        .map(|exclusion| consistency::FamilyExclusion {
            family: exclusion.family.clone(),
        })
        .collect();
    consistency::derive_consistency(&consistency_policy, observations, exclusions)
        .map_err(|_| RecordError::new(RecordErrorKind::InvalidDerivedConsistency))
}

fn consistency_observation(
    observation: &AuthoredFamilyObservation,
) -> Result<consistency::FamilyObservation, RecordError> {
    let source_slot_revisions = observation
        .source_slot_revisions
        .iter()
        .map(|slot| {
            Ok(consistency::SourceSlotObservation {
                source_slot: slot.source_slot.clone(),
                pre_revision: consistency_revision(&slot.pre_revision)?,
                captured_revision: consistency_revision(&slot.captured_revision)?,
                post_revision: consistency_revision(&slot.post_revision)?,
            })
        })
        .collect::<Result<Vec<_>, RecordError>>()?;
    Ok(consistency::FamilyObservation {
        family: observation.family.clone(),
        pre_revision: consistency_revision(&observation.pre_revision)?,
        captured_revision: consistency_revision(&observation.captured_revision)?,
        post_revision: consistency_revision(&observation.post_revision)?,
        source_slot_revisions,
        bound_evaluation: observation
            .bound_evaluation
            .as_ref()
            .map(consistency_bounded_evidence)
            .transpose()?,
    })
}

fn consistency_bounded_evidence(
    evidence: &AuthoredBoundedEvidence,
) -> Result<consistency::BoundedEvidence, RecordError> {
    validate_definition_pair(&evidence.rule)?;
    let mut source_slot_revisions = BTreeMap::new();
    for (slot, revision) in &evidence.evaluated_revisions.source_slot_revisions {
        if !is_identifier(slot)
            || source_slot_revisions
                .insert(slot.clone(), consistency_revision(revision)?)
                .is_some()
        {
            return Err(RecordError::new(RecordErrorKind::InvalidDerivedConsistency));
        }
    }
    Ok(consistency::BoundedEvidence {
        rule: consistency_revision(&evidence.rule)?,
        family_revision: consistency_revision(&evidence.evaluated_revisions.family_revision)?,
        source_slot_revisions,
        evidence_fingerprint: DefinitionFingerprint::parse(&evidence.evidence_fingerprint)
            .map_err(|_| RecordError::new(RecordErrorKind::InvalidDerivedConsistency))?,
        within_bound: matches!(evidence.outcome, BoundedOutcome::WithinBound),
    })
}

fn consistency_revision(pair: &ExactPair) -> Result<consistency::Revision, RecordError> {
    consistency::Revision::new(&pair.reference, &pair.fingerprint)
        .map_err(|_| RecordError::new(RecordErrorKind::InvalidDerivedConsistency))
}

fn snapshot_consistency_name(value: consistency::SnapshotConsistency) -> &'static str {
    match value {
        consistency::SnapshotConsistency::Stable => "stable",
        consistency::SnapshotConsistency::Bounded => "bounded",
        consistency::SnapshotConsistency::Unstable => "unstable",
    }
}

fn admissibility_name(value: consistency::Admissibility) -> &'static str {
    match value {
        consistency::Admissibility::GroundingAndEvidence => "grounding_and_evidence",
        consistency::Admissibility::DiagnosticOnly => "diagnostic_only",
    }
}

#[derive(Deserialize)]
struct HistorySnapshot {
    snapshot_id: String,
    capture: HistoryCapture,
    repository_identity: RepositoryIdentity,
    boundary_stream_ref: String,
    boundary_sequence: u64,
    previous_snapshot: Option<HistoryPreviousSnapshot>,
}

#[derive(Deserialize)]
struct HistoryCapture {
    trigger: CaptureTrigger,
}

#[derive(Deserialize)]
struct HistoryPreviousSnapshot {
    #[serde(rename = "ref")]
    reference: String,
}

fn bind_predecessor(
    history: &[&ContextMemorySnapshot],
    snapshot: &AuthoredSnapshot,
) -> Result<Option<AuthoredPreviousSnapshot>, RecordError> {
    let current = consistency::BoundaryRecord {
        snapshot_id: snapshot.snapshot_id.clone(),
        repository_id: snapshot.repository_identity.repository_id.clone(),
        workspace_id: snapshot.repository_identity.workspace_id.clone(),
        boundary_stream_ref: snapshot.boundary_stream_ref.clone(),
        boundary_sequence: snapshot.boundary_sequence,
        trigger: boundary_trigger(snapshot.capture.trigger),
        previous_snapshot_id: snapshot
            .previous_snapshot
            .as_ref()
            .map(|previous| previous.reference.clone()),
    };
    let history = history_views(history)?;
    let relevant = history
        .iter()
        .filter(|(_, record)| {
            record.repository_identity.repository_id == current.repository_id
                && record.repository_identity.workspace_id == current.workspace_id
                && record.boundary_stream_ref == current.boundary_stream_ref
        })
        .map(|(_, record)| consistency::BoundaryRecord {
            snapshot_id: record.snapshot_id.clone(),
            repository_id: record.repository_identity.repository_id.clone(),
            workspace_id: record.repository_identity.workspace_id.clone(),
            boundary_stream_ref: record.boundary_stream_ref.clone(),
            boundary_sequence: record.boundary_sequence,
            trigger: boundary_trigger(record.capture.trigger),
            previous_snapshot_id: record
                .previous_snapshot
                .as_ref()
                .map(|previous| previous.reference.clone()),
        })
        .collect::<Vec<_>>();
    if relevant
        .iter()
        .any(|record| record.boundary_sequence == current.boundary_sequence)
    {
        return Err(RecordError::new(RecordErrorKind::InvalidPredecessor));
    }

    if snapshot.capture.trigger == CaptureTrigger::SessionEnd {
        if snapshot.previous_snapshot.is_some() {
            return Err(RecordError::new(RecordErrorKind::InvalidPredecessor));
        }
        return Ok(None);
    }

    let predecessor = consistency::validate_prior_end(&relevant, &current)
        .map_err(|_| RecordError::new(RecordErrorKind::InvalidPredecessor))?;
    let Some(predecessor) = predecessor else {
        return snapshot
            .previous_snapshot
            .is_none()
            .then_some(None)
            .ok_or_else(|| RecordError::new(RecordErrorKind::InvalidPredecessor));
    };
    let claimed = snapshot
        .previous_snapshot
        .as_ref()
        .ok_or_else(|| RecordError::new(RecordErrorKind::InvalidPredecessor))?;
    if claimed.reference != predecessor.snapshot_id
        || claimed.boundary_sequence != predecessor.boundary_sequence
        || !is_safe_text(&claimed.reference)
    {
        return Err(RecordError::new(RecordErrorKind::InvalidPredecessor));
    }
    let fingerprint = DefinitionFingerprint::parse(&claimed.record_fingerprint)
        .map_err(|_| RecordError::new(RecordErrorKind::InvalidPredecessor))?;
    let (predecessor_snapshot, _) = history
        .iter()
        .find(|(_, record)| record.snapshot_id == predecessor.snapshot_id)
        .ok_or_else(|| RecordError::new(RecordErrorKind::InvalidPredecessor))?;
    if &fingerprint != predecessor_snapshot.record_fingerprint() {
        return Err(RecordError::new(RecordErrorKind::InvalidPredecessor));
    }
    Ok(Some(claimed.clone()))
}

fn history_views<'a>(
    history: &'a [&'a ContextMemorySnapshot],
) -> Result<Vec<(&'a ContextMemorySnapshot, HistorySnapshot)>, RecordError> {
    let mut snapshot_ids = BTreeSet::new();
    let mut views = Vec::new();
    for snapshot in history {
        let record: HistorySnapshot =
            serde_json::from_value(snapshot.normalized_value().clone())
                .map_err(|_| RecordError::new(RecordErrorKind::InvalidPredecessor))?;
        if !is_safe_text(&record.snapshot_id)
            || !is_safe_text(&record.repository_identity.repository_id)
            || !is_safe_text(&record.repository_identity.workspace_id)
            || !is_safe_text(&record.boundary_stream_ref)
            || record.boundary_sequence == 0
            || !snapshot_ids.insert(record.snapshot_id.clone())
        {
            return Err(RecordError::new(RecordErrorKind::InvalidPredecessor));
        }
        views.push((*snapshot, record));
    }
    Ok(views)
}

fn boundary_trigger(trigger: CaptureTrigger) -> consistency::BoundaryTrigger {
    match trigger {
        CaptureTrigger::SessionEnd => consistency::BoundaryTrigger::SessionEnd,
        CaptureTrigger::SessionStart => consistency::BoundaryTrigger::SessionStart,
    }
}

fn policy_view(policy: &SnapshotCapturePolicy) -> Result<PolicyView, RecordError> {
    let bytes = policy
        .canonical_bytes()
        .map_err(|_| RecordError::new(RecordErrorKind::MalformedRecord))?;
    let value = parse_schema_json(&bytes)
        .map_err(|_| RecordError::new(RecordErrorKind::MalformedRecord))?;
    let view: PolicyView = serde_json::from_value(value)
        .map_err(|_| RecordError::new(RecordErrorKind::MalformedRecord))?;
    let reference = ExactDefinitionRef::new(&view.policy_id, &view.policy_version)
        .map_err(|_| RecordError::new(RecordErrorKind::InvalidExactPair))?;
    let fingerprint = DefinitionFingerprint::parse(&view.policy_fingerprint)
        .map_err(|_| RecordError::new(RecordErrorKind::InvalidExactPair))?;
    if reference.as_str() != policy.reference() || &fingerprint != policy.fingerprint() {
        return Err(RecordError::new(RecordErrorKind::FingerprintMismatch));
    }
    if view.state_families.is_empty() {
        return Err(RecordError::new(RecordErrorKind::MissingFamily));
    }
    for (family, expected) in &view.state_families {
        if !is_identifier(family) {
            return Err(RecordError::new(RecordErrorKind::MalformedRecord));
        }
        validate_definition_pair(&expected.source_adapter)?;
        let slots = expected.source_slots.iter().collect::<BTreeSet<_>>();
        if slots.len() != expected.source_slots.len()
            || expected
                .source_slots
                .iter()
                .any(|slot| !is_identifier(slot))
        {
            return Err(RecordError::new(RecordErrorKind::MalformedRecord));
        }
        let mut windows = BTreeSet::new();
        for window in &expected.windows {
            if !is_identifier(&window.window_id)
                || !slots.contains(&window.source_slot)
                || !windows.insert(&window.window_id)
            {
                return Err(RecordError::new(RecordErrorKind::MalformedRecord));
            }
        }
    }
    Ok(view)
}

fn validate_snapshot_header(snapshot: &AuthoredSnapshot) -> Result<(), RecordError> {
    if snapshot.schema_id != SNAPSHOT_SCHEMA_ID
        || snapshot.schema_version != SNAPSHOT_SCHEMA_VERSION
    {
        return Err(RecordError::new(RecordErrorKind::MalformedRecord));
    }
    if !is_safe_text(&snapshot.snapshot_id)
        || !is_safe_text(&snapshot.capture.started_at)
        || !is_safe_text(&snapshot.capture.completed_at)
        || !is_safe_text(&snapshot.repository_identity.repository_id)
        || !is_safe_text(&snapshot.repository_identity.workspace_id)
        || !is_safe_text(&snapshot.boundary_stream_ref)
        || snapshot.boundary_sequence == 0
    {
        return Err(RecordError::new(RecordErrorKind::InvalidSnapshotIdentity));
    }
    validate_definition_pair(&snapshot.resolved_profile)?;
    validate_definition_pair(&snapshot.context_resolution_envelope)?;
    Ok(())
}

fn normalize_observation(
    expected: &PolicyFamily,
    mut observation: AuthoredFamilyObservation,
) -> Result<AuthoredFamilyObservation, RecordError> {
    validate_definition_pair(&observation.source_adapter)?;
    if observation.source_adapter != expected.source_adapter {
        return Err(RecordError::new(RecordErrorKind::InvalidFamilyProvenance));
    }
    validate_live_pair(&observation.pre_revision)?;
    validate_live_pair(&observation.captured_revision)?;
    validate_live_pair(&observation.post_revision)?;
    if !observation.payload.is_object() {
        return Err(RecordError::new(RecordErrorKind::MalformedRecord));
    }

    let expected_slots = expected.source_slots.iter().collect::<BTreeSet<_>>();
    let mut slots = BTreeMap::new();
    for slot in observation.source_slot_revisions {
        if !is_identifier(&slot.source_slot) || !expected_slots.contains(&slot.source_slot) {
            return Err(RecordError::new(RecordErrorKind::InvalidFamilyProvenance));
        }
        validate_live_pair(&slot.pre_revision)?;
        validate_live_pair(&slot.captured_revision)?;
        validate_live_pair(&slot.post_revision)?;
        if slots.insert(slot.source_slot.clone(), slot).is_some() {
            return Err(RecordError::new(RecordErrorKind::DuplicateFamily));
        }
    }
    if slots.len() != expected_slots.len()
        || expected_slots.iter().any(|slot| !slots.contains_key(*slot))
    {
        return Err(RecordError::new(RecordErrorKind::InvalidFamilyProvenance));
    }

    let expected_windows = expected
        .windows
        .iter()
        .map(|window| (&window.window_id, &window.source_slot))
        .collect::<BTreeMap<_, _>>();
    let mut windows = BTreeMap::new();
    for window in observation.windows {
        let expected_slot = expected_windows
            .get(&window.window_id)
            .ok_or_else(|| RecordError::new(RecordErrorKind::InvalidWindow))?;
        if &window.source_slot != *expected_slot {
            return Err(RecordError::new(RecordErrorKind::InvalidWindow));
        }
        validate_live_pair(&window.source_revision)?;
        if !window.cursor.as_deref().map_or(true, is_safe_text)
            || slots
                .get(&window.source_slot)
                .ok_or_else(|| RecordError::new(RecordErrorKind::InvalidWindow))?
                .captured_revision
                != window.source_revision
        {
            return Err(RecordError::new(RecordErrorKind::InvalidWindow));
        }
        if windows.insert(window.window_id.clone(), window).is_some() {
            return Err(RecordError::new(RecordErrorKind::InvalidWindow));
        }
    }
    if windows.len() != expected_windows.len()
        || expected_windows
            .keys()
            .any(|window| !windows.contains_key(*window))
    {
        return Err(RecordError::new(RecordErrorKind::InvalidWindow));
    }

    normalize_payload(&mut observation.payload, None)?;
    let supplied = DefinitionFingerprint::parse(&observation.payload_fingerprint)
        .map_err(|_| RecordError::new(RecordErrorKind::InvalidExactPair))?;
    let computed = DefinitionFingerprint::from_json_value(&observation.payload)
        .map_err(|_| RecordError::new(RecordErrorKind::MalformedRecord))?;
    if supplied != computed {
        return Err(RecordError::new(RecordErrorKind::FingerprintMismatch));
    }
    observation.source_slot_revisions = slots.into_values().collect();
    observation.windows = windows.into_values().collect();
    Ok(observation)
}

fn normalize_payload(value: &mut Value, field_name: Option<&str>) -> Result<(), RecordError> {
    match value {
        Value::Array(values) => {
            for value in values.iter_mut() {
                normalize_payload(value, None)?;
            }
            if field_name == Some("paths")
                || field_name == Some("refs")
                || field_name.is_some_and(|field| field.ends_with("_refs"))
            {
                let mut ordered = values
                    .drain(..)
                    .map(|value| {
                        canonical_json_bytes(&value)
                            .map(|bytes| (bytes, value))
                            .map_err(|_| RecordError::new(RecordErrorKind::MalformedRecord))
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                ordered.sort_by(|left, right| left.0.cmp(&right.0));
                *values = ordered.into_iter().map(|(_, value)| value).collect();
            }
        }
        Value::Object(values) => {
            for (field, value) in values {
                normalize_payload(value, Some(field))?;
            }
        }
        _ => {}
    }
    Ok(())
}

fn validate_definition_pair(pair: &ExactPair) -> Result<(), RecordError> {
    ExactDefinitionRef::parse(&pair.reference)
        .map_err(|_| RecordError::new(RecordErrorKind::InvalidExactPair))?;
    DefinitionFingerprint::parse(&pair.fingerprint)
        .map_err(|_| RecordError::new(RecordErrorKind::InvalidExactPair))?;
    Ok(())
}

fn validate_live_pair(pair: &ExactPair) -> Result<(), RecordError> {
    if !is_safe_text(&pair.reference) {
        return Err(RecordError::new(RecordErrorKind::InvalidExactPair));
    }
    DefinitionFingerprint::parse(&pair.fingerprint)
        .map_err(|_| RecordError::new(RecordErrorKind::InvalidExactPair))?;
    Ok(())
}

fn is_safe_text(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 1_024
        && value.is_ascii()
        && value
            .bytes()
            .all(|byte| !byte.is_ascii_control() && !byte.is_ascii_whitespace())
}

fn is_identifier(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 64
        && value.as_bytes()[0].is_ascii_lowercase()
        && value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
}

pub(super) mod consistency {
    use crate::DefinitionFingerprint;
    use std::collections::{BTreeMap, BTreeSet};

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub(in crate::snapshot_memory) struct Revision {
        pub(in crate::snapshot_memory) reference: String,
        pub(in crate::snapshot_memory) fingerprint: DefinitionFingerprint,
    }

    impl Revision {
        pub(in crate::snapshot_memory) fn new(
            reference: &str,
            fingerprint: &str,
        ) -> Result<Self, ConsistencyError> {
            if !super::is_safe_text(reference) {
                return Err(ConsistencyError::new(ConsistencyErrorKind::InvalidRevision));
            }
            let fingerprint = DefinitionFingerprint::parse(fingerprint)
                .map_err(|_| ConsistencyError::new(ConsistencyErrorKind::InvalidRevision))?;
            Ok(Self {
                reference: reference.to_owned(),
                fingerprint,
            })
        }
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub(in crate::snapshot_memory) enum SnapshotConsistency {
        Stable,
        Bounded,
        Unstable,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub(in crate::snapshot_memory) enum Admissibility {
        GroundingAndEvidence,
        DiagnosticOnly,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub(in crate::snapshot_memory) enum UnstableAction {
        PersistNonPromotable,
        Refuse,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub(in crate::snapshot_memory) struct ConsistencyPolicy {
        pub(in crate::snapshot_memory) selected_families: BTreeSet<String>,
        pub(in crate::snapshot_memory) bounded_rule: Revision,
        pub(in crate::snapshot_memory) unstable_action: UnstableAction,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub(in crate::snapshot_memory) struct SourceSlotObservation {
        pub(in crate::snapshot_memory) source_slot: String,
        pub(in crate::snapshot_memory) pre_revision: Revision,
        pub(in crate::snapshot_memory) captured_revision: Revision,
        pub(in crate::snapshot_memory) post_revision: Revision,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub(in crate::snapshot_memory) struct BoundedEvidence {
        pub(in crate::snapshot_memory) rule: Revision,
        pub(in crate::snapshot_memory) family_revision: Revision,
        pub(in crate::snapshot_memory) source_slot_revisions: BTreeMap<String, Revision>,
        pub(in crate::snapshot_memory) evidence_fingerprint: DefinitionFingerprint,
        pub(in crate::snapshot_memory) within_bound: bool,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub(in crate::snapshot_memory) struct FamilyObservation {
        pub(in crate::snapshot_memory) family: String,
        pub(in crate::snapshot_memory) pre_revision: Revision,
        pub(in crate::snapshot_memory) captured_revision: Revision,
        pub(in crate::snapshot_memory) post_revision: Revision,
        pub(in crate::snapshot_memory) source_slot_revisions: Vec<SourceSlotObservation>,
        pub(in crate::snapshot_memory) bound_evaluation: Option<BoundedEvidence>,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub(in crate::snapshot_memory) struct FamilyExclusion {
        pub(in crate::snapshot_memory) family: String,
    }

    impl FamilyExclusion {
        pub(in crate::snapshot_memory) fn unavailable(family: &str) -> Self {
            Self {
                family: family.to_owned(),
            }
        }
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub(in crate::snapshot_memory) struct DerivedConsistency {
        pub(in crate::snapshot_memory) consistency: SnapshotConsistency,
        pub(in crate::snapshot_memory) admissibility: Admissibility,
        pub(in crate::snapshot_memory) bounded_evidence: BTreeMap<String, BoundedEvidence>,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub(in crate::snapshot_memory) enum BoundaryTrigger {
        SessionEnd,
        SessionStart,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub(in crate::snapshot_memory) struct BoundaryRecord {
        pub(in crate::snapshot_memory) snapshot_id: String,
        pub(in crate::snapshot_memory) repository_id: String,
        pub(in crate::snapshot_memory) workspace_id: String,
        pub(in crate::snapshot_memory) boundary_stream_ref: String,
        pub(in crate::snapshot_memory) boundary_sequence: u64,
        pub(in crate::snapshot_memory) trigger: BoundaryTrigger,
        pub(in crate::snapshot_memory) previous_snapshot_id: Option<String>,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub(in crate::snapshot_memory) struct Predecessor {
        pub(in crate::snapshot_memory) snapshot_id: String,
        pub(in crate::snapshot_memory) boundary_sequence: u64,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub(in crate::snapshot_memory) enum ConsistencyErrorKind {
        InvalidRevision,
        UnknownFamily,
        DuplicateFamily,
        DuplicateSourceSlot,
        IncompleteFamilyAggregation,
        InvalidBoundedEvidence,
        UnstableRefused,
        InvalidBoundarySequence,
        DuplicateBoundarySequence,
        DuplicateSnapshotIdentity,
        MissingPredecessor,
        UnknownPredecessor,
        SelfPredecessor,
        FuturePredecessor,
        CyclicPredecessor,
        SkippedPredecessor,
        WrongPredecessorRepository,
        WrongPredecessorWorkspace,
        WrongPredecessorStream,
        WrongPredecessorTrigger,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub(in crate::snapshot_memory) struct ConsistencyError {
        kind: ConsistencyErrorKind,
    }

    impl ConsistencyError {
        fn new(kind: ConsistencyErrorKind) -> Self {
            Self { kind }
        }

        pub(in crate::snapshot_memory) fn kind(&self) -> ConsistencyErrorKind {
            self.kind
        }
    }

    enum FamilyClassification {
        Stable,
        Bounded(BoundedEvidence),
        Unstable,
    }

    pub(in crate::snapshot_memory) fn derive_consistency(
        policy: &ConsistencyPolicy,
        observations: Vec<FamilyObservation>,
        exclusions: Vec<FamilyExclusion>,
    ) -> Result<DerivedConsistency, ConsistencyError> {
        let mut observed = BTreeMap::new();
        for observation in observations {
            if !policy.selected_families.contains(&observation.family) {
                return Err(ConsistencyError::new(ConsistencyErrorKind::UnknownFamily));
            }
            if observed
                .insert(observation.family.clone(), observation)
                .is_some()
            {
                return Err(ConsistencyError::new(ConsistencyErrorKind::DuplicateFamily));
            }
        }

        let mut excluded = BTreeSet::new();
        for exclusion in exclusions {
            if !policy.selected_families.contains(&exclusion.family) {
                return Err(ConsistencyError::new(ConsistencyErrorKind::UnknownFamily));
            }
            if observed.contains_key(&exclusion.family) || !excluded.insert(exclusion.family) {
                return Err(ConsistencyError::new(ConsistencyErrorKind::DuplicateFamily));
            }
        }
        if observed.len() + excluded.len() != policy.selected_families.len()
            || policy
                .selected_families
                .iter()
                .any(|family| !observed.contains_key(family) && !excluded.contains(family))
        {
            return Err(ConsistencyError::new(
                ConsistencyErrorKind::IncompleteFamilyAggregation,
            ));
        }

        let mut consistency = if excluded.is_empty() {
            SnapshotConsistency::Stable
        } else {
            SnapshotConsistency::Unstable
        };
        let mut bounded_evidence = BTreeMap::new();
        for (family, observation) in observed {
            match classify_family(policy, &observation)? {
                FamilyClassification::Stable => {}
                FamilyClassification::Bounded(evidence) => {
                    if consistency == SnapshotConsistency::Stable {
                        consistency = SnapshotConsistency::Bounded;
                    }
                    bounded_evidence.insert(family, evidence);
                }
                FamilyClassification::Unstable => consistency = SnapshotConsistency::Unstable,
            }
        }

        if consistency == SnapshotConsistency::Unstable {
            if policy.unstable_action == UnstableAction::Refuse {
                return Err(ConsistencyError::new(ConsistencyErrorKind::UnstableRefused));
            }
            return Ok(DerivedConsistency {
                consistency,
                admissibility: Admissibility::DiagnosticOnly,
                bounded_evidence,
            });
        }

        Ok(DerivedConsistency {
            consistency,
            admissibility: Admissibility::GroundingAndEvidence,
            bounded_evidence,
        })
    }

    fn classify_family(
        policy: &ConsistencyPolicy,
        observation: &FamilyObservation,
    ) -> Result<FamilyClassification, ConsistencyError> {
        let captured_slots = captured_slot_revisions(observation)?;
        let revisions_stable = observation.pre_revision == observation.captured_revision
            && observation.captured_revision == observation.post_revision
            && observation.source_slot_revisions.iter().all(|slot| {
                slot.pre_revision == slot.captured_revision
                    && slot.captured_revision == slot.post_revision
            });
        match &observation.bound_evaluation {
            None if revisions_stable => Ok(FamilyClassification::Stable),
            None => Ok(FamilyClassification::Unstable),
            Some(evidence) if !evidence.within_bound => Ok(FamilyClassification::Unstable),
            Some(evidence)
                if evidence.rule != policy.bounded_rule
                    || evidence.family_revision != observation.captured_revision
                    || evidence.source_slot_revisions != captured_slots =>
            {
                Err(ConsistencyError::new(
                    ConsistencyErrorKind::InvalidBoundedEvidence,
                ))
            }
            Some(evidence) => Ok(FamilyClassification::Bounded(evidence.clone())),
        }
    }

    fn captured_slot_revisions(
        observation: &FamilyObservation,
    ) -> Result<BTreeMap<String, Revision>, ConsistencyError> {
        let mut slots = BTreeMap::new();
        for slot in &observation.source_slot_revisions {
            if slots
                .insert(slot.source_slot.clone(), slot.captured_revision.clone())
                .is_some()
            {
                return Err(ConsistencyError::new(
                    ConsistencyErrorKind::DuplicateSourceSlot,
                ));
            }
        }
        Ok(slots)
    }

    pub(in crate::snapshot_memory) fn validate_prior_end(
        history: &[BoundaryRecord],
        current: &BoundaryRecord,
    ) -> Result<Option<Predecessor>, ConsistencyError> {
        validate_boundary_identities(history, current)?;
        if current
            .previous_snapshot_id
            .as_deref()
            .is_some_and(|snapshot_id| snapshot_id == current.snapshot_id)
        {
            return Err(ConsistencyError::new(ConsistencyErrorKind::SelfPredecessor));
        }
        if topology_is_cyclic(history, current) {
            return Err(ConsistencyError::new(
                ConsistencyErrorKind::CyclicPredecessor,
            ));
        }
        if current.trigger != BoundaryTrigger::SessionStart {
            return Err(ConsistencyError::new(
                ConsistencyErrorKind::WrongPredecessorTrigger,
            ));
        }

        let expected = select_prior_end(history, current);
        let Some(claimed_id) = current.previous_snapshot_id.as_deref() else {
            return expected
                .map(|_| {
                    Err(ConsistencyError::new(
                        ConsistencyErrorKind::MissingPredecessor,
                    ))
                })
                .unwrap_or(Ok(None));
        };
        let claimed = history
            .iter()
            .find(|record| record.snapshot_id == claimed_id)
            .ok_or_else(|| ConsistencyError::new(ConsistencyErrorKind::UnknownPredecessor))?;
        if claimed.boundary_sequence >= current.boundary_sequence {
            return Err(ConsistencyError::new(
                ConsistencyErrorKind::FuturePredecessor,
            ));
        }
        if claimed.repository_id != current.repository_id {
            return Err(ConsistencyError::new(
                ConsistencyErrorKind::WrongPredecessorRepository,
            ));
        }
        if claimed.workspace_id != current.workspace_id {
            return Err(ConsistencyError::new(
                ConsistencyErrorKind::WrongPredecessorWorkspace,
            ));
        }
        if claimed.boundary_stream_ref != current.boundary_stream_ref {
            return Err(ConsistencyError::new(
                ConsistencyErrorKind::WrongPredecessorStream,
            ));
        }
        if claimed.trigger != BoundaryTrigger::SessionEnd {
            return Err(ConsistencyError::new(
                ConsistencyErrorKind::WrongPredecessorTrigger,
            ));
        }
        let expected = expected
            .ok_or_else(|| ConsistencyError::new(ConsistencyErrorKind::WrongPredecessorTrigger))?;
        if expected.snapshot_id != claimed.snapshot_id {
            return Err(ConsistencyError::new(
                ConsistencyErrorKind::SkippedPredecessor,
            ));
        }
        Ok(Some(Predecessor {
            snapshot_id: claimed.snapshot_id.clone(),
            boundary_sequence: claimed.boundary_sequence,
        }))
    }

    fn validate_boundary_identities(
        history: &[BoundaryRecord],
        current: &BoundaryRecord,
    ) -> Result<(), ConsistencyError> {
        let mut snapshot_ids = BTreeSet::new();
        let mut sequences = BTreeSet::new();
        for record in history.iter().chain(std::iter::once(current)) {
            if record.boundary_sequence == 0 {
                return Err(ConsistencyError::new(
                    ConsistencyErrorKind::InvalidBoundarySequence,
                ));
            }
            if !snapshot_ids.insert(record.snapshot_id.as_str()) {
                return Err(ConsistencyError::new(
                    ConsistencyErrorKind::DuplicateSnapshotIdentity,
                ));
            }
            let key = (
                record.repository_id.as_str(),
                record.workspace_id.as_str(),
                record.boundary_stream_ref.as_str(),
                record.boundary_sequence,
            );
            if !sequences.insert(key) {
                return Err(ConsistencyError::new(
                    ConsistencyErrorKind::DuplicateBoundarySequence,
                ));
            }
        }
        Ok(())
    }

    fn topology_is_cyclic(history: &[BoundaryRecord], current: &BoundaryRecord) -> bool {
        let records = history
            .iter()
            .chain(std::iter::once(current))
            .map(|record| (record.snapshot_id.as_str(), record))
            .collect::<BTreeMap<_, _>>();
        records.keys().any(|start| {
            let mut seen = BTreeSet::new();
            let mut cursor = Some(*start);
            while let Some(snapshot_id) = cursor {
                if !seen.insert(snapshot_id) {
                    return true;
                }
                cursor = records
                    .get(snapshot_id)
                    .and_then(|record| record.previous_snapshot_id.as_deref());
            }
            false
        })
    }

    fn select_prior_end<'a>(
        history: &'a [BoundaryRecord],
        current: &BoundaryRecord,
    ) -> Option<&'a BoundaryRecord> {
        history
            .iter()
            .filter(|candidate| {
                candidate.repository_id == current.repository_id
                    && candidate.workspace_id == current.workspace_id
                    && candidate.boundary_stream_ref == current.boundary_stream_ref
                    && candidate.boundary_sequence < current.boundary_sequence
                    && candidate.trigger == BoundaryTrigger::SessionEnd
            })
            .max_by_key(|candidate| candidate.boundary_sequence)
    }
}
