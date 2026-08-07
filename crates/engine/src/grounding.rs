use crate::context_resolution::ContextResolutionEnvelope;
use crate::snapshot_memory::{derive_grounding_source_pair, GroundingSourcePair};
use crate::{DefinitionFingerprint, ExactDefinitionRef};
use serde::Deserialize;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

const SOURCE_DIRECTORY: [&str; 4] = [".handbook", "grounding", "hcm-3.5", "v1"];
const CURRENT_SNAPSHOT_REF: &str = "handbook.grounding.snapshot.current@1.0.0";
const DEFINITION_SCHEMA_ID: &str = "handbook.grounding-summary-definition";
const DISCLOSURE_SCHEMA_ID: &str = "handbook.grounding-disclosure-policy";
const REQUIRED_FAMILIES: [&str; 5] = ["evidence", "git", "handbook", "session", "work"];
const ELIGIBLE_KINDS: [&str; 3] = ["expected_progress", "proof_drift", "scope_expansion"];
const PERMITTED_METADATA: [&str; 3] = [
    "affected_after_fingerprint",
    "evidence_ref",
    "justification_ref",
];

#[derive(Clone, Debug, Eq, PartialEq)]
struct ExactGroundingRef {
    reference: String,
    fingerprint: DefinitionFingerprint,
}

impl ExactGroundingRef {
    fn parse(value: &str) -> Result<Self, GroundingReferenceError> {
        let (reference, fingerprint) = value
            .rsplit_once('#')
            .filter(|(reference, _)| !reference.contains('#'))
            .ok_or(GroundingReferenceError::InvalidExactReference)?;
        ExactDefinitionRef::parse(reference)
            .map_err(|_| GroundingReferenceError::InvalidExactReference)?;
        let fingerprint = DefinitionFingerprint::parse(fingerprint)
            .map_err(|_| GroundingReferenceError::InvalidExactReference)?;
        Ok(Self {
            reference: reference.to_owned(),
            fingerprint,
        })
    }

    fn token(&self) -> String {
        format!("{}#{}", self.reference, self.fingerprint)
    }
}

macro_rules! grounding_ref {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub struct $name(ExactGroundingRef);

        impl $name {
            pub fn parse_exact(value: &str) -> Result<Self, GroundingReferenceError> {
                ExactGroundingRef::parse(value).map(Self)
            }
        }
    };
}

grounding_ref!(GroundingSnapshotRef);
grounding_ref!(GroundingDeltaRef);
grounding_ref!(GroundingDefinitionRef);
grounding_ref!(GroundingDisclosureRef);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GroundingReferenceError {
    InvalidExactReference,
}

#[derive(Debug)]
pub struct GroundingOperationError;

#[derive(Debug)]
pub struct GroundingRequest {
    snapshot_ref: GroundingSnapshotRef,
    delta_ref: GroundingDeltaRef,
    definition_ref: GroundingDefinitionRef,
    disclosure_ref: GroundingDisclosureRef,
    envelope: ContextResolutionEnvelope,
}

impl GroundingRequest {
    pub fn new(
        snapshot_ref: GroundingSnapshotRef,
        delta_ref: GroundingDeltaRef,
        definition_ref: GroundingDefinitionRef,
        disclosure_ref: GroundingDisclosureRef,
        envelope: ContextResolutionEnvelope,
    ) -> Self {
        Self {
            snapshot_ref,
            delta_ref,
            definition_ref,
            disclosure_ref,
            envelope,
        }
    }
}

#[derive(Debug)]
pub enum GroundingOutcome {
    Grounded(GroundedResolution),
    Refused(GroundingRefusal),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GroundingRefusalKind {
    MissingSource,
    MalformedSource,
    SourceMismatch,
    DefinitionMismatch,
    DisclosureMismatch,
    IncompatibleDelta,
    StaleCurrentness,
    InsufficientResolution,
    UnsupportedDefinition,
    UnaccountedSignal,
}

#[derive(Clone, Debug)]
pub struct GroundingRefusal {
    kind: GroundingRefusalKind,
    omissions: Vec<GroundingOmission>,
    provenance: GroundingProvenance,
}

impl GroundingRefusal {
    pub fn kind(&self) -> GroundingRefusalKind {
        self.kind
    }

    pub fn omissions(&self) -> &[GroundingOmission] {
        &self.omissions
    }

    pub fn provenance(&self) -> &GroundingProvenance {
        &self.provenance
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GroundingOmission {
    signal_id: Option<String>,
    kind: Option<String>,
    reason: &'static str,
}

impl GroundingOmission {
    pub fn signal_id(&self) -> Option<&str> {
        self.signal_id.as_deref()
    }

    pub fn classification(&self) -> Option<&str> {
        self.kind.as_deref()
    }

    pub fn reason(&self) -> &str {
        self.reason
    }
}

#[derive(Clone, Debug)]
pub struct GroundingProvenance {
    snapshot: String,
    delta: String,
    definition: String,
    disclosure: String,
    envelope: String,
}

impl GroundingProvenance {
    pub fn snapshot(&self) -> &str {
        &self.snapshot
    }

    pub fn delta(&self) -> &str {
        &self.delta
    }

    pub fn definition(&self) -> &str {
        &self.definition
    }

    pub fn disclosure(&self) -> &str {
        &self.disclosure
    }

    pub fn envelope(&self) -> &str {
        &self.envelope
    }
}

#[derive(Clone, Debug)]
pub struct DeltaSignalSummary {
    entries: Vec<DeltaSignalSummaryEntry>,
    omissions: Vec<GroundingOmission>,
    provenance: GroundingProvenance,
}

impl DeltaSignalSummary {
    pub fn entries(&self) -> &[DeltaSignalSummaryEntry] {
        &self.entries
    }

    pub fn omissions(&self) -> &[GroundingOmission] {
        &self.omissions
    }

    pub fn provenance(&self) -> &GroundingProvenance {
        &self.provenance
    }
}

#[derive(Clone, Debug)]
pub struct DeltaSignalSummaryEntry {
    signal_id: String,
    classification: String,
    rule_reference: String,
    rule_fingerprint: String,
    affected_after_fingerprints: Vec<String>,
    evidence_refs: Vec<String>,
    justification_refs: Vec<String>,
}

impl DeltaSignalSummaryEntry {
    pub fn signal_id(&self) -> &str {
        &self.signal_id
    }

    pub fn classification(&self) -> &str {
        &self.classification
    }

    pub fn rule_reference(&self) -> &str {
        &self.rule_reference
    }

    pub fn rule_fingerprint(&self) -> &str {
        &self.rule_fingerprint
    }

    pub fn affected_after_fingerprints(&self) -> &[String] {
        &self.affected_after_fingerprints
    }

    pub fn evidence_refs(&self) -> &[String] {
        &self.evidence_refs
    }

    pub fn justification_refs(&self) -> &[String] {
        &self.justification_refs
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceAvailability {
    Unavailable,
    False,
}

#[derive(Clone, Debug)]
pub struct GroundingEvidence;

impl GroundingEvidence {
    pub fn local_closeout(&self) -> EvidenceAvailability {
        EvidenceAvailability::Unavailable
    }

    pub fn parent_promotion(&self) -> EvidenceAvailability {
        EvidenceAvailability::Unavailable
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct FlowPacketGrounding {
    provenance: GroundingProvenance,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct SharedResolutionInclusion {
    provenance: GroundingProvenance,
}

#[derive(Debug)]
pub struct GroundedResolution {
    flow_packet_grounding: FlowPacketGrounding,
    shared_resolution_inclusion: SharedResolutionInclusion,
    delta_signal_summary: DeltaSignalSummary,
    provenance: GroundingProvenance,
    omissions: Vec<GroundingOmission>,
    evidence: GroundingEvidence,
}

impl GroundedResolution {
    pub fn flow_packet_grounding(&self) -> &FlowPacketGrounding {
        &self.flow_packet_grounding
    }

    pub fn shared_resolution_inclusion(&self) -> &SharedResolutionInclusion {
        &self.shared_resolution_inclusion
    }

    pub fn delta_signal_summary(&self) -> &DeltaSignalSummary {
        &self.delta_signal_summary
    }

    pub fn provenance(&self) -> &GroundingProvenance {
        &self.provenance
    }

    pub fn omissions(&self) -> &[GroundingOmission] {
        &self.omissions
    }

    pub fn evidence(&self) -> &GroundingEvidence {
        &self.evidence
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct SummaryDefinition {
    schema_id: String,
    schema_version: String,
    definition_ref: String,
    maximum_cardinality: usize,
    eligible_signal_kinds: Vec<String>,
    stable_order: String,
    overflow_disposition: String,
    permitted_metadata: Vec<String>,
    required_currentness_families: Vec<String>,
    minimum_resolution_ranks: Vec<u8>,
    insufficient_resolution_disposition: String,
    unchecked_family_disposition: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct DisclosurePolicy {
    schema_id: String,
    schema_version: String,
    disclosure_ref: String,
    redacted_signal_kinds: Vec<String>,
    permitted_evidence_ref_prefixes: Vec<String>,
    permitted_justification_ref_prefixes: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct CurrentnessWitness {
    schema_id: String,
    schema_version: String,
    families: Vec<CurrentnessFamily>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct CurrentnessFamily {
    family: String,
    captured_revision_fingerprint: String,
    slots: Vec<CurrentnessSlot>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct CurrentnessSlot {
    source_slot: String,
    captured_revision_fingerprint: String,
}

pub fn ground_resolution(
    repo_root: &Path,
    request: GroundingRequest,
) -> Result<GroundingOutcome, GroundingOperationError> {
    if !repo_root.is_dir() {
        return Err(GroundingOperationError);
    }
    let provenance = provenance_for(&request);
    let result = resolve(repo_root, &request, provenance.clone());
    Ok(match result {
        Ok(grounded) => GroundingOutcome::Grounded(grounded),
        Err((kind, omissions)) => GroundingOutcome::Refused(GroundingRefusal {
            kind,
            omissions,
            provenance,
        }),
    })
}

fn resolve(
    repo_root: &Path,
    request: &GroundingRequest,
    provenance: GroundingProvenance,
) -> Result<GroundedResolution, (GroundingRefusalKind, Vec<GroundingOmission>)> {
    let authority_view = request
        .envelope
        .projection_authority_view()
        .map_err(|_| (GroundingRefusalKind::StaleCurrentness, Vec::new()))?;
    resolve_with_authority(
        repo_root,
        &request.snapshot_ref.0,
        &request.delta_ref.0,
        &request.definition_ref.0,
        &request.disclosure_ref.0,
        authority_view.dimension_ranks(),
        provenance,
    )
}

fn resolve_with_authority(
    repo_root: &Path,
    snapshot_ref: &ExactGroundingRef,
    delta_ref: &ExactGroundingRef,
    definition_ref: &ExactGroundingRef,
    disclosure_ref: &ExactGroundingRef,
    resolution_ranks: [u8; 6],
    provenance: GroundingProvenance,
) -> Result<GroundedResolution, (GroundingRefusalKind, Vec<GroundingOmission>)> {
    let source_root = source_root(repo_root);
    let definition_bytes = read_source(&source_root, "definition.json")?;
    let definition_value = parse_json(&definition_bytes)?;
    if fingerprint(&definition_value)? != definition_ref.fingerprint {
        return Err((GroundingRefusalKind::DefinitionMismatch, Vec::new()));
    }
    let definition: SummaryDefinition = serde_json::from_value(definition_value)
        .map_err(|_| (GroundingRefusalKind::MalformedSource, Vec::new()))?;
    validate_definition(&definition, definition_ref)?;

    let disclosure_bytes = read_source(&source_root, "disclosure.json")?;
    let disclosure_value = parse_json(&disclosure_bytes)?;
    if fingerprint(&disclosure_value)? != disclosure_ref.fingerprint {
        return Err((GroundingRefusalKind::DisclosureMismatch, Vec::new()));
    }
    let disclosure: DisclosurePolicy = serde_json::from_value(disclosure_value)
        .map_err(|_| (GroundingRefusalKind::MalformedSource, Vec::new()))?;
    validate_disclosure(&disclosure, disclosure_ref)?;

    let source_pair = load_source_pair(&source_root)?;
    if snapshot_ref.reference != CURRENT_SNAPSHOT_REF
        || snapshot_ref.fingerprint != *source_pair.current_snapshot_fingerprint()
        || delta_ref.reference != source_pair.delta_ref()
        || delta_ref.fingerprint != *source_pair.delta_route_fingerprint()
    {
        return Err((GroundingRefusalKind::SourceMismatch, Vec::new()));
    }

    if resolution_ranks
        .iter()
        .zip(&definition.minimum_resolution_ranks)
        .any(|(actual, required)| actual < required)
    {
        return Err((GroundingRefusalKind::InsufficientResolution, Vec::new()));
    }

    let witness_bytes = read_source(&source_root, "currentness.json")?;
    let witness: CurrentnessWitness = serde_json::from_slice(&witness_bytes)
        .map_err(|_| (GroundingRefusalKind::MalformedSource, Vec::new()))?;
    validate_currentness(&definition, &witness, source_pair.current_snapshot())?;

    let (entries, omissions) =
        summarize(&definition, &disclosure, source_pair.delta(), &provenance)?;
    let summary = DeltaSignalSummary {
        entries,
        omissions: omissions.clone(),
        provenance: provenance.clone(),
    };
    Ok(GroundedResolution {
        flow_packet_grounding: FlowPacketGrounding {
            provenance: provenance.clone(),
        },
        shared_resolution_inclusion: SharedResolutionInclusion {
            provenance: provenance.clone(),
        },
        delta_signal_summary: summary,
        provenance,
        omissions,
        evidence: GroundingEvidence,
    })
}

fn source_root(repo_root: &Path) -> PathBuf {
    SOURCE_DIRECTORY
        .iter()
        .fold(repo_root.to_path_buf(), |path, segment| path.join(segment))
}

fn read_source(
    source_root: &Path,
    name: &str,
) -> Result<Vec<u8>, (GroundingRefusalKind, Vec<GroundingOmission>)> {
    fs::read(source_root.join(name)).map_err(|_| (GroundingRefusalKind::MissingSource, Vec::new()))
}

fn load_source_pair(
    source_root: &Path,
) -> Result<GroundingSourcePair, (GroundingRefusalKind, Vec<GroundingOmission>)> {
    let policy = read_source(source_root, "snapshot-policy.json")?;
    let prior_capture = read_source(source_root, "prior-capture.json")?;
    let prior_snapshot = read_source(source_root, "prior-snapshot.json")?;
    let current_capture = read_source(source_root, "current-capture.json")?;
    let current_snapshot = read_source(source_root, "current-snapshot.json")?;
    let catalog = read_source(source_root, "delta-catalog.json")?;
    let delta_route = read_source(source_root, "delta-route.json")?;
    derive_grounding_source_pair(
        &policy,
        &prior_capture,
        &prior_snapshot,
        &current_capture,
        &current_snapshot,
        &catalog,
        &delta_route,
    )
    .map_err(|_| (GroundingRefusalKind::IncompatibleDelta, Vec::new()))
}

fn parse_json(bytes: &[u8]) -> Result<Value, (GroundingRefusalKind, Vec<GroundingOmission>)> {
    serde_json::from_slice(bytes).map_err(|_| (GroundingRefusalKind::MalformedSource, Vec::new()))
}

fn fingerprint(
    value: &Value,
) -> Result<DefinitionFingerprint, (GroundingRefusalKind, Vec<GroundingOmission>)> {
    DefinitionFingerprint::from_json_value(value)
        .map_err(|_| (GroundingRefusalKind::MalformedSource, Vec::new()))
}

fn validate_definition(
    definition: &SummaryDefinition,
    requested: &ExactGroundingRef,
) -> Result<(), (GroundingRefusalKind, Vec<GroundingOmission>)> {
    if definition.schema_id != DEFINITION_SCHEMA_ID
        || definition.schema_version != "1.0"
        || definition.definition_ref != requested.reference
        || definition.maximum_cardinality != 2
        || definition.eligible_signal_kinds != ELIGIBLE_KINDS
        || definition.stable_order != "kind_then_signal_id"
        || definition.overflow_disposition != "omit"
        || definition.permitted_metadata != PERMITTED_METADATA
        || definition.required_currentness_families != REQUIRED_FAMILIES
        || definition.minimum_resolution_ranks.len() != 6
        || definition.insufficient_resolution_disposition != "refuse"
        || definition.unchecked_family_disposition != "refuse"
    {
        return Err((GroundingRefusalKind::UnsupportedDefinition, Vec::new()));
    }
    Ok(())
}

fn validate_disclosure(
    disclosure: &DisclosurePolicy,
    requested: &ExactGroundingRef,
) -> Result<(), (GroundingRefusalKind, Vec<GroundingOmission>)> {
    let eligible = ELIGIBLE_KINDS.into_iter().collect::<BTreeSet<_>>();
    if disclosure.schema_id != DISCLOSURE_SCHEMA_ID
        || disclosure.schema_version != "1.0"
        || disclosure.disclosure_ref != requested.reference
        || disclosure
            .redacted_signal_kinds
            .iter()
            .any(|kind| !eligible.contains(kind.as_str()))
        || !strictly_ordered(&disclosure.redacted_signal_kinds)
        || disclosure.permitted_evidence_ref_prefixes != ["evidence.", "decision."]
        || disclosure.permitted_justification_ref_prefixes != ["decision."]
    {
        return Err((GroundingRefusalKind::UnsupportedDefinition, Vec::new()));
    }
    Ok(())
}

fn validate_currentness(
    definition: &SummaryDefinition,
    witness: &CurrentnessWitness,
    snapshot: &Value,
) -> Result<(), (GroundingRefusalKind, Vec<GroundingOmission>)> {
    if witness.schema_id != "handbook.grounding-currentness" || witness.schema_version != "1.0" {
        return Err((GroundingRefusalKind::MalformedSource, Vec::new()));
    }
    let expected = currentness_from_snapshot(snapshot)?;
    if witness.families.len() != definition.required_currentness_families.len()
        || witness
            .families
            .iter()
            .map(|family| family.family.as_str())
            .ne(definition
                .required_currentness_families
                .iter()
                .map(String::as_str))
    {
        return Err((GroundingRefusalKind::StaleCurrentness, Vec::new()));
    }
    for family in &witness.families {
        let Some((fingerprint, slots)) = expected.get(&family.family) else {
            return Err((GroundingRefusalKind::StaleCurrentness, Vec::new()));
        };
        if &family.captured_revision_fingerprint != fingerprint
            || !strictly_ordered_by(&family.slots, |slot| slot.source_slot.as_str())
            || family.slots.len() != slots.len()
            || family.slots.iter().any(|slot| {
                slots
                    .get(&slot.source_slot)
                    .is_none_or(|fingerprint| fingerprint != &slot.captured_revision_fingerprint)
            })
        {
            return Err((GroundingRefusalKind::StaleCurrentness, Vec::new()));
        }
    }
    Ok(())
}

fn currentness_from_snapshot(
    snapshot: &Value,
) -> Result<
    BTreeMap<String, (String, BTreeMap<String, String>)>,
    (GroundingRefusalKind, Vec<GroundingOmission>),
> {
    let observations = snapshot
        .get("family_observations")
        .and_then(Value::as_array)
        .ok_or((GroundingRefusalKind::MalformedSource, Vec::new()))?;
    let mut result = BTreeMap::new();
    for observation in observations {
        let object = observation
            .as_object()
            .ok_or((GroundingRefusalKind::MalformedSource, Vec::new()))?;
        let family = required_string(object, "family")?.to_owned();
        let captured = exact_fingerprint(
            object
                .get("captured_revision")
                .ok_or((GroundingRefusalKind::MalformedSource, Vec::new()))?,
        )?;
        let mut slots = BTreeMap::new();
        for slot in object
            .get("source_slot_revisions")
            .and_then(Value::as_array)
            .ok_or((GroundingRefusalKind::MalformedSource, Vec::new()))?
        {
            let slot = slot
                .as_object()
                .ok_or((GroundingRefusalKind::MalformedSource, Vec::new()))?;
            let name = required_string(slot, "source_slot")?.to_owned();
            let fingerprint = exact_fingerprint(
                slot.get("captured_revision")
                    .ok_or((GroundingRefusalKind::MalformedSource, Vec::new()))?,
            )?;
            if slots.insert(name, fingerprint).is_some() {
                return Err((GroundingRefusalKind::MalformedSource, Vec::new()));
            }
        }
        if result.insert(family, (captured, slots)).is_some() {
            return Err((GroundingRefusalKind::MalformedSource, Vec::new()));
        }
    }
    if result.keys().map(String::as_str).ne(REQUIRED_FAMILIES) {
        return Err((GroundingRefusalKind::StaleCurrentness, Vec::new()));
    }
    Ok(result)
}

fn summarize(
    definition: &SummaryDefinition,
    disclosure: &DisclosurePolicy,
    delta: &Value,
    provenance: &GroundingProvenance,
) -> Result<
    (Vec<DeltaSignalSummaryEntry>, Vec<GroundingOmission>),
    (GroundingRefusalKind, Vec<GroundingOmission>),
> {
    let changes = delta
        .get("changes")
        .and_then(Value::as_array)
        .ok_or((GroundingRefusalKind::MalformedSource, Vec::new()))?;
    let mut after_by_change = BTreeMap::new();
    for change in changes {
        let change = change
            .as_object()
            .ok_or((GroundingRefusalKind::MalformedSource, Vec::new()))?;
        let id = required_string(change, "change_id")?.to_owned();
        let after = required_string(change, "after_fingerprint")?.to_owned();
        DefinitionFingerprint::parse(&after)
            .map_err(|_| (GroundingRefusalKind::MalformedSource, Vec::new()))?;
        if after_by_change.insert(id, after).is_some() {
            return Err((GroundingRefusalKind::UnaccountedSignal, Vec::new()));
        }
    }

    let signals = delta
        .get("signals")
        .and_then(Value::as_array)
        .ok_or((GroundingRefusalKind::MalformedSource, Vec::new()))?;
    let mut included = Vec::new();
    let mut omissions = Vec::new();
    let mut seen = BTreeSet::new();
    for signal in signals {
        let signal = signal
            .as_object()
            .ok_or((GroundingRefusalKind::MalformedSource, Vec::new()))?;
        let signal_id = required_string(signal, "signal_id")?.to_owned();
        let kind = required_string(signal, "kind")?.to_owned();
        if !seen.insert(signal_id.clone()) {
            return Err((GroundingRefusalKind::UnaccountedSignal, Vec::new()));
        }
        if !definition.eligible_signal_kinds.contains(&kind) {
            omissions.push(omission(&signal_id, &kind, "ineligible"));
            continue;
        }
        if disclosure.redacted_signal_kinds.contains(&kind) {
            omissions.push(omission(&signal_id, &kind, "redacted"));
            continue;
        }
        included.push(summary_entry(signal, &after_by_change)?);
    }
    included.sort_by(|left, right| {
        (&left.classification, &left.signal_id).cmp(&(&right.classification, &right.signal_id))
    });
    while included.len() > definition.maximum_cardinality {
        let entry = included.pop().expect("bounded length is nonzero");
        omissions.push(omission(
            &entry.signal_id,
            &entry.classification,
            "overflow",
        ));
    }
    omissions.sort_by(|left, right| {
        (&left.kind, &left.signal_id, left.reason).cmp(&(
            &right.kind,
            &right.signal_id,
            right.reason,
        ))
    });
    if included.len() + omissions.len() != signals.len() {
        return Err((GroundingRefusalKind::UnaccountedSignal, Vec::new()));
    }
    let _ = provenance;
    Ok((included, omissions))
}

fn summary_entry(
    signal: &serde_json::Map<String, Value>,
    after_by_change: &BTreeMap<String, String>,
) -> Result<DeltaSignalSummaryEntry, (GroundingRefusalKind, Vec<GroundingOmission>)> {
    let signal_id = required_string(signal, "signal_id")?.to_owned();
    let classification = required_string(signal, "kind")?.to_owned();
    let rule = signal
        .get("rule")
        .and_then(Value::as_object)
        .ok_or((GroundingRefusalKind::MalformedSource, Vec::new()))?;
    let rule_reference = required_string(rule, "ref")?.to_owned();
    ExactDefinitionRef::parse(&rule_reference)
        .map_err(|_| (GroundingRefusalKind::MalformedSource, Vec::new()))?;
    let rule_fingerprint = required_string(rule, "fingerprint")?.to_owned();
    DefinitionFingerprint::parse(&rule_fingerprint)
        .map_err(|_| (GroundingRefusalKind::MalformedSource, Vec::new()))?;
    let change_ids = string_array(signal.get("change_ids"))?;
    if change_ids.is_empty() || !strictly_ordered(&change_ids) {
        return Err((GroundingRefusalKind::MalformedSource, Vec::new()));
    }
    let affected_after_fingerprints = change_ids
        .iter()
        .map(|id| {
            after_by_change
                .get(id)
                .cloned()
                .ok_or((GroundingRefusalKind::UnaccountedSignal, Vec::new()))
        })
        .collect::<Result<BTreeSet<_>, _>>()?
        .into_iter()
        .collect();
    let evidence_refs = admitted_refs(
        string_array(signal.get("evidence_refs"))?,
        &["evidence.", "decision."],
    )?;
    let justification_refs = admitted_refs(
        string_array(signal.get("justification_refs"))?,
        &["decision."],
    )?;
    Ok(DeltaSignalSummaryEntry {
        signal_id,
        classification,
        rule_reference,
        rule_fingerprint,
        affected_after_fingerprints,
        evidence_refs,
        justification_refs,
    })
}

fn admitted_refs(
    refs: Vec<String>,
    prefixes: &[&str],
) -> Result<Vec<String>, (GroundingRefusalKind, Vec<GroundingOmission>)> {
    if !strictly_ordered(&refs)
        || refs
            .iter()
            .any(|reference| !prefixes.iter().any(|prefix| reference.starts_with(prefix)))
    {
        return Err((GroundingRefusalKind::UnaccountedSignal, Vec::new()));
    }
    Ok(refs)
}

fn string_array(
    value: Option<&Value>,
) -> Result<Vec<String>, (GroundingRefusalKind, Vec<GroundingOmission>)> {
    value
        .and_then(Value::as_array)
        .ok_or((GroundingRefusalKind::MalformedSource, Vec::new()))?
        .iter()
        .map(|value| {
            value
                .as_str()
                .map(str::to_owned)
                .ok_or((GroundingRefusalKind::MalformedSource, Vec::new()))
        })
        .collect()
}

fn omission(signal_id: &str, kind: &str, reason: &'static str) -> GroundingOmission {
    GroundingOmission {
        signal_id: Some(signal_id.to_owned()),
        kind: Some(kind.to_owned()),
        reason,
    }
}

fn provenance_for(request: &GroundingRequest) -> GroundingProvenance {
    GroundingProvenance {
        snapshot: request.snapshot_ref.0.token(),
        delta: request.delta_ref.0.token(),
        definition: request.definition_ref.0.token(),
        disclosure: request.disclosure_ref.0.token(),
        envelope: format!(
            "{}#{}",
            request.envelope.exact_binding().reference(),
            request.envelope.exact_binding().fingerprint()
        ),
    }
}

fn required_string<'a>(
    object: &'a serde_json::Map<String, Value>,
    field: &str,
) -> Result<&'a str, (GroundingRefusalKind, Vec<GroundingOmission>)> {
    object
        .get(field)
        .and_then(Value::as_str)
        .ok_or((GroundingRefusalKind::MalformedSource, Vec::new()))
}

fn exact_fingerprint(
    value: &Value,
) -> Result<String, (GroundingRefusalKind, Vec<GroundingOmission>)> {
    let value = value
        .as_object()
        .ok_or((GroundingRefusalKind::MalformedSource, Vec::new()))?;
    let reference = required_string(value, "ref")?;
    if reference.is_empty()
        || reference
            .bytes()
            .any(|byte| byte.is_ascii_control() || byte.is_ascii_whitespace())
    {
        return Err((GroundingRefusalKind::MalformedSource, Vec::new()));
    }
    let fingerprint = required_string(value, "fingerprint")?.to_owned();
    DefinitionFingerprint::parse(&fingerprint)
        .map_err(|_| (GroundingRefusalKind::MalformedSource, Vec::new()))?;
    Ok(fingerprint)
}

fn strictly_ordered(values: &[String]) -> bool {
    values.windows(2).all(|pair| pair[0] < pair[1])
}

fn strictly_ordered_by<T>(values: &[T], key: impl Fn(&T) -> &str) -> bool {
    values.windows(2).all(|pair| key(&pair[0]) < key(&pair[1]))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::fs;
    use std::path::Path;
    use tempfile::TempDir;

    fn definition() -> SummaryDefinition {
        SummaryDefinition {
            schema_id: DEFINITION_SCHEMA_ID.to_owned(),
            schema_version: "1.0".to_owned(),
            definition_ref: "handbook.grounding.summary.hcm-3-5-p1@1.0.0".to_owned(),
            maximum_cardinality: 2,
            eligible_signal_kinds: ELIGIBLE_KINDS.into_iter().map(str::to_owned).collect(),
            stable_order: "kind_then_signal_id".to_owned(),
            overflow_disposition: "omit".to_owned(),
            permitted_metadata: PERMITTED_METADATA.into_iter().map(str::to_owned).collect(),
            required_currentness_families: REQUIRED_FAMILIES
                .into_iter()
                .map(str::to_owned)
                .collect(),
            minimum_resolution_ranks: vec![0; 6],
            insufficient_resolution_disposition: "refuse".to_owned(),
            unchecked_family_disposition: "refuse".to_owned(),
        }
    }

    fn disclosure(redacted_signal_kinds: Vec<&str>) -> DisclosurePolicy {
        DisclosurePolicy {
            schema_id: DISCLOSURE_SCHEMA_ID.to_owned(),
            schema_version: "1.0".to_owned(),
            disclosure_ref: "handbook.grounding.disclosure.hcm-3-5-p1@1.0.0".to_owned(),
            redacted_signal_kinds: redacted_signal_kinds
                .into_iter()
                .map(str::to_owned)
                .collect(),
            permitted_evidence_ref_prefixes: vec!["evidence.".to_owned(), "decision.".to_owned()],
            permitted_justification_ref_prefixes: vec!["decision.".to_owned()],
        }
    }

    fn provenance() -> GroundingProvenance {
        GroundingProvenance {
            snapshot: "snapshot".to_owned(),
            delta: "delta".to_owned(),
            definition: "definition".to_owned(),
            disclosure: "disclosure".to_owned(),
            envelope: "envelope".to_owned(),
        }
    }

    fn fingerprint(fill: char) -> String {
        format!("sha256:{}", fill.to_string().repeat(64))
    }

    fn signal(id: &str, kind: &str, change_id: &str) -> Value {
        json!({
            "signal_id": id,
            "kind": kind,
            "rule": {"ref": format!("handbook.rule.{}@1.0.0", kind.replace('_', "-")), "fingerprint": fingerprint('a')},
            "change_ids": [change_id],
            "evidence_refs": [],
            "justification_refs": [],
        })
    }

    #[test]
    fn summary_is_stably_bounded_and_partitions_every_source_signal() {
        let delta = json!({
            "changes": [
                {"change_id": "change.evidence", "after_fingerprint": fingerprint('e')},
                {"change_id": "change.git", "after_fingerprint": fingerprint('b')},
                {"change_id": "change.work", "after_fingerprint": fingerprint('c')},
                {"change_id": "change.handbook", "after_fingerprint": fingerprint('d')}
            ],
            "signals": [
                signal("signal.scope", "scope_expansion", "change.git"),
                signal("signal.expected", "expected_progress", "change.work"),
                signal("signal.proof", "proof_drift", "change.evidence"),
                signal("signal.semantic", "semantic_drift", "change.handbook")
            ]
        });
        let (entries, omissions) = summarize(
            &definition(),
            &disclosure(Vec::new()),
            &delta,
            &provenance(),
        )
        .expect("valid bounded summary");

        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].classification(), "expected_progress");
        assert_eq!(entries[1].classification(), "proof_drift");
        assert!(entries.iter().all(|entry| entry.evidence_refs().is_empty()));
        assert!(omissions
            .iter()
            .any(|omission| omission.reason() == "overflow"));
        assert!(omissions
            .iter()
            .any(|omission| omission.reason() == "ineligible"));
        assert_eq!(entries.len() + omissions.len(), 4);
    }

    #[test]
    fn redaction_precedes_entry_materialization_and_currentness_is_exact() {
        let delta = json!({
            "changes": [{"change_id": "change.git", "after_fingerprint": fingerprint('b')}],
            "signals": [signal("signal.scope", "scope_expansion", "change.git")]
        });
        let (entries, omissions) = summarize(
            &definition(),
            &disclosure(vec!["scope_expansion"]),
            &delta,
            &provenance(),
        )
        .expect("redaction is an omission rather than an error");
        assert!(entries.is_empty());
        assert_eq!(omissions[0].reason(), "redacted");

        let snapshot = json!({"family_observations": [
            {"family": "evidence", "captured_revision": {"ref": "evidence.revision.1", "fingerprint": fingerprint('a')}, "source_slot_revisions": []},
            {"family": "git", "captured_revision": {"ref": "git.revision.1", "fingerprint": fingerprint('b')}, "source_slot_revisions": []},
            {"family": "handbook", "captured_revision": {"ref": "handbook.revision.1", "fingerprint": fingerprint('c')}, "source_slot_revisions": []},
            {"family": "session", "captured_revision": {"ref": "session.revision.1", "fingerprint": fingerprint('d')}, "source_slot_revisions": []},
            {"family": "work", "captured_revision": {"ref": "work.composite.1", "fingerprint": fingerprint('e')}, "source_slot_revisions": [
                {"source_slot": "active_plan", "captured_revision": {"ref": "work.active-plan.1", "fingerprint": fingerprint('f')}},
                {"source_slot": "work_ledger", "captured_revision": {"ref": "work.ledger.1", "fingerprint": fingerprint('a')}}
            ]}
        ]});
        let witness = CurrentnessWitness {
            schema_id: "handbook.grounding-currentness".to_owned(),
            schema_version: "1.0".to_owned(),
            families: vec![
                CurrentnessFamily {
                    family: "evidence".to_owned(),
                    captured_revision_fingerprint: fingerprint('a'),
                    slots: Vec::new(),
                },
                CurrentnessFamily {
                    family: "git".to_owned(),
                    captured_revision_fingerprint: fingerprint('b'),
                    slots: Vec::new(),
                },
                CurrentnessFamily {
                    family: "handbook".to_owned(),
                    captured_revision_fingerprint: fingerprint('c'),
                    slots: Vec::new(),
                },
                CurrentnessFamily {
                    family: "session".to_owned(),
                    captured_revision_fingerprint: fingerprint('d'),
                    slots: Vec::new(),
                },
                CurrentnessFamily {
                    family: "work".to_owned(),
                    captured_revision_fingerprint: fingerprint('e'),
                    slots: vec![
                        CurrentnessSlot {
                            source_slot: "active_plan".to_owned(),
                            captured_revision_fingerprint: fingerprint('f'),
                        },
                        CurrentnessSlot {
                            source_slot: "work_ledger".to_owned(),
                            captured_revision_fingerprint: fingerprint('a'),
                        },
                    ],
                },
            ],
        };
        assert!(validate_currentness(&definition(), &witness, &snapshot).is_ok());
        let mut incomplete = witness;
        incomplete.families.pop();
        assert_eq!(
            validate_currentness(&definition(), &incomplete, &snapshot)
                .unwrap_err()
                .0,
            GroundingRefusalKind::StaleCurrentness
        );
    }

    #[test]
    fn persisted_source_route_derives_the_compatible_delta_before_grounding() {
        let (repo, refs) = persisted_source_fixture();
        let grounded = resolve_with_authority(
            repo.path(),
            &refs.snapshot,
            &refs.delta,
            &refs.definition,
            &refs.disclosure,
            [0; 6],
            provenance(),
        )
        .expect("exact fixture source route grounds");

        assert_eq!(grounded.delta_signal_summary().entries().len(), 2);
        assert_eq!(grounded.delta_signal_summary().omissions().len(), 3);
        assert_eq!(
            grounded.evidence().local_closeout(),
            EvidenceAvailability::Unavailable
        );
    }

    struct FixtureRefs {
        snapshot: ExactGroundingRef,
        delta: ExactGroundingRef,
        definition: ExactGroundingRef,
        disclosure: ExactGroundingRef,
    }

    fn persisted_source_fixture() -> (TempDir, FixtureRefs) {
        let repo = tempfile::tempdir().expect("temporary repository");
        let source_root = source_root(repo.path());
        fs::create_dir_all(&source_root).expect("fixture source directory");

        let mut policy: Value = serde_json::from_slice(include_bytes!(
            "snapshot_memory/fixtures/policy-family.json"
        ))
        .expect("policy fixture");
        repair_policy_fingerprint(&mut policy);
        let prior_capture = capture_input(&policy, "session_start");
        let current_capture = capture_input(&policy, "session_end");

        let mut prior: Value = serde_json::from_slice(include_bytes!(
            "snapshot_memory/fixtures/snapshot-record.json"
        ))
        .expect("snapshot fixture");
        repair_snapshot_payload_fingerprints(&mut prior);
        bind_snapshot_capture(&mut prior, &policy, "session_start");
        let mut current = prior.clone();
        current["snapshot_id"] = json!("snap_hcm_3_5_current_0002");
        current["capture"]["trigger"] = json!("session_end");
        current["capture"]["started_at"] = json!("2026-08-07T15:35:00Z");
        current["capture"]["completed_at"] = json!("2026-08-07T15:35:05Z");
        current["boundary_stream_ref"] = json!("orchestration.hcm_3_5.current");
        current["boundary_sequence"] = json!(2);
        bind_snapshot_capture(&mut current, &policy, "session_end");
        for family in ["evidence", "git", "work"] {
            current["family_observations"]
                .as_array_mut()
                .expect("observations")
                .iter_mut()
                .find(|observation| observation["family"] == family)
                .expect("selected family")["payload"]["hcm_3_5_delta_marker"] =
                json!(format!("{family}_changed"));
        }
        repair_snapshot_payload_fingerprints(&mut current);

        let mut catalog: Value = serde_json::from_slice(include_bytes!(
            "snapshot_memory/fixtures/delta-catalog.json"
        ))
        .expect("catalog fixture");
        catalog["admitted_policies"] = json!([exact_pair(
            "handbook.snapshot-policy.session-boundary@1.0.0",
            policy["policy_fingerprint"]
                .as_str()
                .expect("policy fingerprint"),
        )]);
        repair_catalog_fingerprint(&mut catalog);

        let definition_value: Value = serde_json::from_slice(include_bytes!(
            "../tests/fixtures/hcm_3_5_grounding/definition.json"
        ))
        .expect("definition fixture");
        let disclosure_value: Value = serde_json::from_slice(include_bytes!(
            "../tests/fixtures/hcm_3_5_grounding/disclosure.json"
        ))
        .expect("disclosure fixture");
        let route = json!({
            "schema_id": "handbook.grounding-delta-route",
            "schema_version": "1.0",
            "delta_ref": "handbook.grounding.delta.prior-current@1.0.0",
            "prior_snapshot_fingerprint": json_fingerprint(&prior),
            "current_snapshot_fingerprint": json_fingerprint(&current),
            "delta_catalog_fingerprint": json_fingerprint(&catalog),
        });
        let witness = currentness_witness(&current);

        write_json(&source_root.join("snapshot-policy.json"), &policy);
        write_json(&source_root.join("prior-capture.json"), &prior_capture);
        write_json(&source_root.join("prior-snapshot.json"), &prior);
        write_json(&source_root.join("current-capture.json"), &current_capture);
        write_json(&source_root.join("current-snapshot.json"), &current);
        write_json(&source_root.join("delta-catalog.json"), &catalog);
        write_json(&source_root.join("delta-route.json"), &route);
        write_json(&source_root.join("currentness.json"), &witness);
        write_json(&source_root.join("definition.json"), &definition_value);
        write_json(&source_root.join("disclosure.json"), &disclosure_value);

        (
            repo,
            FixtureRefs {
                snapshot: ExactGroundingRef::parse(&format!(
                    "{CURRENT_SNAPSHOT_REF}#{}",
                    json_fingerprint(&current)
                ))
                .expect("snapshot ref"),
                delta: ExactGroundingRef::parse(&format!(
                    "handbook.grounding.delta.prior-current@1.0.0#{}",
                    json_fingerprint(&route)
                ))
                .expect("delta ref"),
                definition: ExactGroundingRef::parse(&format!(
                    "{}#{}",
                    definition_value["definition_ref"]
                        .as_str()
                        .expect("definition ref"),
                    json_fingerprint(&definition_value)
                ))
                .expect("definition ref"),
                disclosure: ExactGroundingRef::parse(&format!(
                    "{}#{}",
                    disclosure_value["disclosure_ref"]
                        .as_str()
                        .expect("disclosure ref"),
                    json_fingerprint(&disclosure_value)
                ))
                .expect("disclosure ref"),
            },
        )
    }

    fn capture_input(policy: &Value, trigger: &str) -> Value {
        json!({
            "policy": exact_pair("handbook.snapshot-policy.session-boundary@1.0.0", policy["policy_fingerprint"].as_str().expect("policy fingerprint")),
            "trigger": trigger,
            "memory_horizon": "execution",
            "families": [
                single_slot_capture("evidence", "evidence", "evidence.revision.1", None),
                single_slot_capture("git", "git", "git.revision.1", None),
                single_slot_capture("handbook", "handbook", "handbook.revision.1", None),
                single_slot_capture("session", "session", "session.revision.1", Some("session-cursor")),
                {"family": "work", "slots": [
                    {"slot": "active_plan", "revision": exact_pair("work.active-plan.1", &fingerprint('b')), "cursor": null},
                    {"slot": "work_ledger", "revision": exact_pair("work.ledger.1", &fingerprint('c')), "cursor": "ledger-cursor"}
                ], "windows": [
                    {"window_id": "queued_next", "slot": "active_plan", "revision": exact_pair("work.active-plan.1", &fingerprint('b')), "cursor": null},
                    {"window_id": "recent_completed", "slot": "work_ledger", "revision": exact_pair("work.ledger.1", &fingerprint('c')), "cursor": "ledger-cursor"}
                ]}
            ]
        })
    }

    fn single_slot_capture(
        family: &str,
        slot: &str,
        revision: &str,
        cursor: Option<&str>,
    ) -> Value {
        json!({"family": family, "slots": [{"slot": slot, "revision": exact_pair(revision, &fingerprint('d')), "cursor": cursor}], "windows": []})
    }

    fn bind_snapshot_capture(record: &mut Value, policy: &Value, trigger: &str) {
        record["capture"]["input"]["policy"] = exact_pair(
            "handbook.snapshot-policy.session-boundary@1.0.0",
            policy["policy_fingerprint"]
                .as_str()
                .expect("policy fingerprint"),
        );
        record["capture"]["input"]["trigger"] = json!(trigger);
    }

    fn currentness_witness(record: &Value) -> Value {
        let families = record["family_observations"]
            .as_array()
            .expect("observations")
            .iter()
            .map(|observation| {
                let slots = observation["source_slot_revisions"]
                    .as_array()
                    .expect("slots")
                    .iter()
                    .map(|slot| json!({
                        "source_slot": slot["source_slot"],
                        "captured_revision_fingerprint": slot["captured_revision"]["fingerprint"],
                    }))
                    .collect::<Vec<_>>();
                json!({
                    "family": observation["family"],
                    "captured_revision_fingerprint": observation["captured_revision"]["fingerprint"],
                    "slots": slots,
                })
            })
            .collect::<Vec<_>>();
        json!({"schema_id": "handbook.grounding-currentness", "schema_version": "1.0", "families": families})
    }

    fn repair_policy_fingerprint(value: &mut Value) {
        let mut preimage = value.clone();
        preimage
            .as_object_mut()
            .unwrap()
            .remove("policy_fingerprint");
        value["policy_fingerprint"] = json!(json_fingerprint(&preimage));
    }

    fn repair_catalog_fingerprint(value: &mut Value) {
        let mut preimage = value.clone();
        preimage
            .as_object_mut()
            .unwrap()
            .remove("catalog_fingerprint");
        value["catalog_fingerprint"] = json!(json_fingerprint(&preimage));
    }

    fn repair_snapshot_payload_fingerprints(value: &mut Value) {
        for observation in value["family_observations"]
            .as_array_mut()
            .expect("observations")
        {
            let mut payload = observation["payload"].clone();
            normalize_payload(&mut payload, None);
            observation["payload_fingerprint"] = json!(json_fingerprint(&payload));
        }
    }

    fn normalize_payload(value: &mut Value, field_name: Option<&str>) {
        match value {
            Value::Array(values) => {
                for value in values.iter_mut() {
                    normalize_payload(value, None);
                }
                if field_name == Some("paths")
                    || field_name == Some("refs")
                    || field_name.is_some_and(|field| field.ends_with("_refs"))
                {
                    values.sort_by_key(canonical_json);
                }
            }
            Value::Object(values) => {
                for (field, value) in values {
                    normalize_payload(value, Some(field));
                }
            }
            _ => {}
        }
    }

    fn exact_pair(reference: &str, fingerprint: &str) -> Value {
        json!({"ref": reference, "fingerprint": fingerprint})
    }

    fn json_fingerprint(value: &Value) -> String {
        DefinitionFingerprint::from_json_value(value)
            .expect("canonical fingerprint")
            .to_string()
    }

    fn canonical_json(value: &Value) -> Vec<u8> {
        serde_json_canonicalizer::to_vec(value).expect("canonical JSON")
    }

    fn write_json(path: &Path, value: &Value) {
        fs::write(path, canonical_json(value)).expect("write fixture JSON");
    }
}
