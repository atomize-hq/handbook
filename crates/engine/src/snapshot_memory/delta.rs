use super::record::ContextMemorySnapshot;
use crate::{parse_schema_json, DefinitionFingerprint, ExactDefinitionRef};
use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};

const CATALOG_SCHEMA_ID: &str = "handbook.snapshot-drift-catalog";
const DELTA_SCHEMA_ID: &str = "handbook.snapshot-delta";
const SCHEMA_VERSION: &str = "1.0";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum DeltaErrorKind {
    MalformedCatalog,
    StaleCatalog,
    MissingCatalogRule,
    DuplicateCatalogRule,
    DuplicateSignalKind,
    InvalidCatalogRule,
    MalformedSnapshot,
    UnstableEndpoint,
    IncompatibleRepository,
    IncompatibleWorkspace,
    IncompatiblePolicy,
    IncompatibleFamily,
    IncompleteFamilyCoverage,
    ExcludedFamily,
    InvalidJustification,
    MissingJustification,
    MalformedDelta,
    IncompleteEvaluation,
    DuplicateEvaluation,
    ContradictoryEvaluation,
    SignalBijection,
    UncatalogedSignal,
    FingerprintMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct DeltaError {
    kind: DeltaErrorKind,
}

impl DeltaError {
    fn new(kind: DeltaErrorKind) -> Self {
        Self { kind }
    }

    pub(super) fn kind(&self) -> DeltaErrorKind {
        self.kind
    }
}

pub(super) struct SnapshotEndpoint<'a> {
    snapshot: &'a ContextMemorySnapshot,
}

impl<'a> SnapshotEndpoint<'a> {
    pub(super) fn new(snapshot: &'a ContextMemorySnapshot) -> Self {
        Self { snapshot }
    }
}

#[derive(Clone, Debug)]
pub(super) struct DriftCatalog {
    comparison_contract: ExactPair,
    drift_rule_catalog: ExactPair,
    admitted_policies: BTreeSet<ExactPair>,
    rules: Vec<CatalogRule>,
}

#[derive(Clone, Debug)]
pub(super) struct SnapshotDelta {
    value: Value,
    fingerprint: DefinitionFingerprint,
}

impl SnapshotDelta {
    pub(super) fn fingerprint(&self) -> &DefinitionFingerprint {
        &self.fingerprint
    }

    pub(super) fn normalized_value(&self) -> &Value {
        &self.value
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd)]
#[serde(deny_unknown_fields)]
struct ExactPair {
    #[serde(rename = "ref")]
    reference: String,
    fingerprint: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct AuthoredCatalog {
    schema_id: String,
    schema_version: String,
    comparison_contract: ExactPair,
    drift_rule_catalog: ExactPair,
    admitted_policies: Vec<ExactPair>,
    rules: Vec<AuthoredRule>,
    catalog_fingerprint: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct AuthoredRule {
    rule_id: String,
    rule: ExactPair,
    signal_kind: String,
    matches_family: String,
    requires_justification: bool,
    admitted_justification_refs: Vec<String>,
}

#[derive(Clone, Debug)]
struct CatalogRule {
    rule_id: String,
    rule: ExactPair,
    signal_kind: SignalKind,
    matches_family: String,
    requires_justification: bool,
    admitted_justification_refs: BTreeSet<String>,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum SignalKind {
    ExpectedProgress,
    JustifiedDivergence,
    UnexplainedDrift,
    ScopeExpansion,
    ExecutionInefficiencySignal,
    PlanningInaccuracySignal,
    ProofDrift,
    SemanticDrift,
    StaleHandoff,
}

impl SignalKind {
    fn parse(value: &str) -> Option<Self> {
        match value {
            "expected_progress" => Some(Self::ExpectedProgress),
            "justified_divergence" => Some(Self::JustifiedDivergence),
            "unexplained_drift" => Some(Self::UnexplainedDrift),
            "scope_expansion" => Some(Self::ScopeExpansion),
            "execution_inefficiency_signal" => Some(Self::ExecutionInefficiencySignal),
            "planning_inaccuracy_signal" => Some(Self::PlanningInaccuracySignal),
            "proof_drift" => Some(Self::ProofDrift),
            "semantic_drift" => Some(Self::SemanticDrift),
            "stale_handoff" => Some(Self::StaleHandoff),
            _ => None,
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            Self::ExpectedProgress => "expected_progress",
            Self::JustifiedDivergence => "justified_divergence",
            Self::UnexplainedDrift => "unexplained_drift",
            Self::ScopeExpansion => "scope_expansion",
            Self::ExecutionInefficiencySignal => "execution_inefficiency_signal",
            Self::PlanningInaccuracySignal => "planning_inaccuracy_signal",
            Self::ProofDrift => "proof_drift",
            Self::SemanticDrift => "semantic_drift",
            Self::StaleHandoff => "stale_handoff",
        }
    }

    fn required_family(self) -> &'static str {
        match self {
            Self::ExpectedProgress | Self::PlanningInaccuracySignal => "work",
            Self::JustifiedDivergence | Self::SemanticDrift => "handbook",
            Self::UnexplainedDrift | Self::StaleHandoff => "session",
            Self::ScopeExpansion => "git",
            Self::ExecutionInefficiencySignal | Self::ProofDrift => "evidence",
        }
    }
}

pub(super) fn load_drift_catalog(bytes: &[u8]) -> Result<DriftCatalog, DeltaError> {
    let value =
        parse_schema_json(bytes).map_err(|_| DeltaError::new(DeltaErrorKind::MalformedCatalog))?;
    let authored: AuthoredCatalog = serde_json::from_value(value.clone())
        .map_err(|_| DeltaError::new(DeltaErrorKind::MalformedCatalog))?;
    if authored.schema_id != CATALOG_SCHEMA_ID || authored.schema_version != SCHEMA_VERSION {
        return Err(DeltaError::new(DeltaErrorKind::MalformedCatalog));
    }
    let mut preimage = value;
    preimage
        .as_object_mut()
        .ok_or_else(|| DeltaError::new(DeltaErrorKind::MalformedCatalog))?
        .remove("catalog_fingerprint");
    let supplied = DefinitionFingerprint::parse(&authored.catalog_fingerprint)
        .map_err(|_| DeltaError::new(DeltaErrorKind::StaleCatalog))?;
    let computed = DefinitionFingerprint::from_json_value(&preimage)
        .map_err(|_| DeltaError::new(DeltaErrorKind::MalformedCatalog))?;
    if supplied != computed {
        return Err(DeltaError::new(DeltaErrorKind::StaleCatalog));
    }
    validate_exact_pair(
        &authored.comparison_contract,
        DeltaErrorKind::MalformedCatalog,
    )?;
    validate_exact_pair(
        &authored.drift_rule_catalog,
        DeltaErrorKind::MalformedCatalog,
    )?;

    let mut admitted_policies = BTreeSet::new();
    for policy in authored.admitted_policies {
        validate_exact_pair(&policy, DeltaErrorKind::MalformedCatalog)?;
        if !admitted_policies.insert(policy) {
            return Err(DeltaError::new(DeltaErrorKind::MalformedCatalog));
        }
    }
    if admitted_policies.is_empty() {
        return Err(DeltaError::new(DeltaErrorKind::MalformedCatalog));
    }

    let mut rule_ids = BTreeSet::new();
    let mut rule_pairs = BTreeSet::new();
    let mut signal_kinds = BTreeSet::new();
    let mut rules = Vec::new();
    for authored_rule in authored.rules {
        if !is_identifier(&authored_rule.rule_id) || !is_identifier(&authored_rule.matches_family) {
            return Err(DeltaError::new(DeltaErrorKind::InvalidCatalogRule));
        }
        validate_exact_pair(&authored_rule.rule, DeltaErrorKind::InvalidCatalogRule)?;
        if !rule_ids.insert(authored_rule.rule_id.clone())
            || !rule_pairs.insert(authored_rule.rule.clone())
        {
            return Err(DeltaError::new(DeltaErrorKind::DuplicateCatalogRule));
        }
        let signal_kind = SignalKind::parse(&authored_rule.signal_kind)
            .ok_or_else(|| DeltaError::new(DeltaErrorKind::InvalidCatalogRule))?;
        if !signal_kinds.insert(signal_kind) {
            return Err(DeltaError::new(DeltaErrorKind::DuplicateSignalKind));
        }
        if authored_rule.matches_family != signal_kind.required_family()
            || authored_rule.requires_justification
                != (signal_kind == SignalKind::JustifiedDivergence)
        {
            return Err(DeltaError::new(DeltaErrorKind::InvalidCatalogRule));
        }
        let admitted_justification_refs = authored_rule
            .admitted_justification_refs
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>();
        if admitted_justification_refs.len() != authored_rule.admitted_justification_refs.len()
            || admitted_justification_refs
                .iter()
                .any(|reference| !is_safe_text(reference))
            || (!authored_rule.requires_justification && !admitted_justification_refs.is_empty())
            || (authored_rule.requires_justification && admitted_justification_refs.is_empty())
        {
            return Err(DeltaError::new(DeltaErrorKind::InvalidCatalogRule));
        }
        rules.push(CatalogRule {
            rule_id: authored_rule.rule_id,
            rule: authored_rule.rule,
            signal_kind,
            matches_family: authored_rule.matches_family,
            requires_justification: authored_rule.requires_justification,
            admitted_justification_refs,
        });
    }
    if signal_kinds.len() != 9 {
        return Err(DeltaError::new(DeltaErrorKind::MissingCatalogRule));
    }

    Ok(DriftCatalog {
        comparison_contract: authored.comparison_contract,
        drift_rule_catalog: authored.drift_rule_catalog,
        admitted_policies,
        rules,
    })
}

pub(super) fn derive_snapshot_delta(
    from: SnapshotEndpoint<'_>,
    to: SnapshotEndpoint<'_>,
    catalog: &DriftCatalog,
    justifications: &BTreeMap<String, Vec<String>>,
) -> Result<SnapshotDelta, DeltaError> {
    let from = SnapshotView::from_snapshot(from.snapshot)?;
    let to = SnapshotView::from_snapshot(to.snapshot)?;
    if !from.is_delta_admissible() || !to.is_delta_admissible() {
        return Err(DeltaError::new(DeltaErrorKind::UnstableEndpoint));
    }
    if from.repository_id != to.repository_id {
        return Err(DeltaError::new(DeltaErrorKind::IncompatibleRepository));
    }
    if from.workspace_id != to.workspace_id {
        return Err(DeltaError::new(DeltaErrorKind::IncompatibleWorkspace));
    }
    if from.policy != to.policy
        || !catalog.admitted_policies.contains(&from.policy)
        || !catalog.admitted_policies.contains(&to.policy)
    {
        return Err(DeltaError::new(DeltaErrorKind::IncompatiblePolicy));
    }
    if !from.excluded_families.is_empty() || !to.excluded_families.is_empty() {
        return Err(DeltaError::new(DeltaErrorKind::ExcludedFamily));
    }
    if from.observations.keys().collect::<Vec<_>>() != to.observations.keys().collect::<Vec<_>>() {
        return Err(DeltaError::new(DeltaErrorKind::IncompleteFamilyCoverage));
    }
    validate_justifications(catalog, justifications)?;

    let mut changes = Vec::new();
    for (family, before) in &from.observations {
        let after = to
            .observations
            .get(family)
            .ok_or_else(|| DeltaError::new(DeltaErrorKind::IncompleteFamilyCoverage))?;
        if before.source_adapter != after.source_adapter {
            return Err(DeltaError::new(DeltaErrorKind::IncompatibleFamily));
        }
        if before.payload_fingerprint != after.payload_fingerprint {
            changes.push(DeltaChange {
                change_id: format!("change.{family}"),
                family: family.clone(),
                stable_key: family.clone(),
                before_fingerprint: before.payload_fingerprint.to_string(),
                after_fingerprint: after.payload_fingerprint.to_string(),
            });
        }
    }

    let mut rule_evaluations = Vec::new();
    let mut signals = Vec::new();
    for rule in &catalog.rules {
        let change_ids = changes
            .iter()
            .filter(|change| change.family == rule.matches_family)
            .map(|change| change.change_id.clone())
            .collect::<Vec<_>>();
        let matched = !change_ids.is_empty();
        let signal_id = matched.then(|| format!("signal.{}", rule.rule_id));
        if matched {
            let justification_refs = if rule.requires_justification {
                let refs = justifications
                    .get(&rule.rule_id)
                    .filter(|refs| !refs.is_empty())
                    .ok_or_else(|| DeltaError::new(DeltaErrorKind::MissingJustification))?;
                refs.clone()
            } else {
                Vec::new()
            };
            signals.push(json!({
                "signal_id": signal_id.as_ref().expect("matched signal id"),
                "kind": rule.signal_kind.as_str(),
                "rule": exact_pair_value(&rule.rule),
                "change_ids": change_ids,
                "evidence_refs": [],
                "justification_refs": justification_refs,
            }));
        } else if justifications.contains_key(&rule.rule_id) {
            return Err(DeltaError::new(DeltaErrorKind::InvalidJustification));
        }
        rule_evaluations.push(json!({
            "rule": exact_pair_value(&rule.rule),
            "outcome": if matched { "matched" } else { "not_matched" },
            "signal_id": signal_id,
        }));
    }

    let compared_state_families = from.observations.keys().cloned().collect::<Vec<_>>();
    let changes = changes
        .iter()
        .map(DeltaChange::as_value)
        .collect::<Vec<_>>();
    let mut value = json!({
        "schema_id": DELTA_SCHEMA_ID,
        "schema_version": SCHEMA_VERSION,
        "delta_id": format!("delta.{}.{}", from.record_fingerprint, to.record_fingerprint),
        "from_snapshot": from.as_value(),
        "to_snapshot": to.as_value(),
        "compatibility": {
            "comparison_contract": exact_pair_value(&catalog.comparison_contract),
            "drift_rule_catalog": exact_pair_value(&catalog.drift_rule_catalog),
            "compared_state_families": compared_state_families,
            "excluded_state_families": [],
        },
        "changes": changes,
        "rule_evaluations": rule_evaluations,
        "signals": signals,
    });
    let fingerprint = DefinitionFingerprint::from_json_value(&value)
        .map_err(|_| DeltaError::new(DeltaErrorKind::MalformedDelta))?;
    value
        .as_object_mut()
        .ok_or_else(|| DeltaError::new(DeltaErrorKind::MalformedDelta))?
        .insert(
            "delta_fingerprint".to_owned(),
            Value::String(fingerprint.to_string()),
        );
    validate_delta(catalog, &value)?;
    Ok(SnapshotDelta { value, fingerprint })
}

pub(super) fn validate_delta(catalog: &DriftCatalog, value: &Value) -> Result<(), DeltaError> {
    let delta: AuthoredDelta = serde_json::from_value(value.clone())
        .map_err(|_| DeltaError::new(DeltaErrorKind::MalformedDelta))?;
    if delta.schema_id != DELTA_SCHEMA_ID
        || delta.schema_version != SCHEMA_VERSION
        || !is_safe_text(&delta.delta_id)
    {
        return Err(DeltaError::new(DeltaErrorKind::MalformedDelta));
    }
    validate_snapshot_ref(&delta.from_snapshot)?;
    validate_snapshot_ref(&delta.to_snapshot)?;
    if delta.compatibility.comparison_contract != catalog.comparison_contract
        || delta.compatibility.drift_rule_catalog != catalog.drift_rule_catalog
        || delta.compatibility.compared_state_families.is_empty()
        || !delta.compatibility.excluded_state_families.is_empty()
        || !is_strictly_ordered_identifiers(&delta.compatibility.compared_state_families)
    {
        return Err(DeltaError::new(DeltaErrorKind::IncompleteFamilyCoverage));
    }

    let mut change_by_id = BTreeMap::new();
    let mut previous_family = None;
    for change in &delta.changes {
        if !is_safe_text(&change.change_id)
            || !is_identifier(&change.family)
            || change.stable_key != change.family
            || change.change_kind != "updated"
            || change.before_fingerprint.is_none()
            || change.after_fingerprint.is_none()
            || !delta
                .compatibility
                .compared_state_families
                .contains(&change.family)
            || previous_family
                .as_ref()
                .is_some_and(|family: &String| family >= &change.family)
        {
            return Err(DeltaError::new(DeltaErrorKind::MalformedDelta));
        }
        let before = DefinitionFingerprint::parse(change.before_fingerprint.as_deref().unwrap())
            .map_err(|_| DeltaError::new(DeltaErrorKind::MalformedDelta))?;
        let after = DefinitionFingerprint::parse(change.after_fingerprint.as_deref().unwrap())
            .map_err(|_| DeltaError::new(DeltaErrorKind::MalformedDelta))?;
        if before == after
            || change_by_id
                .insert(change.change_id.clone(), change)
                .is_some()
        {
            return Err(DeltaError::new(DeltaErrorKind::MalformedDelta));
        }
        previous_family = Some(change.family.clone());
    }

    let mut seen_evaluations = BTreeSet::new();
    for evaluation in &delta.rule_evaluations {
        validate_exact_pair(&evaluation.rule, DeltaErrorKind::MalformedDelta)?;
        if !seen_evaluations.insert(evaluation.rule.clone()) {
            return Err(DeltaError::new(DeltaErrorKind::DuplicateEvaluation));
        }
    }
    if delta.rule_evaluations.len() != catalog.rules.len()
        || delta
            .rule_evaluations
            .iter()
            .zip(&catalog.rules)
            .any(|(evaluation, rule)| evaluation.rule != rule.rule)
    {
        return Err(DeltaError::new(DeltaErrorKind::IncompleteEvaluation));
    }

    let mut signal_by_id = BTreeMap::new();
    for signal in &delta.signals {
        validate_exact_pair(&signal.rule, DeltaErrorKind::MalformedDelta)?;
        let Some(rule) = catalog.rules.iter().find(|rule| rule.rule == signal.rule) else {
            return Err(DeltaError::new(DeltaErrorKind::UncatalogedSignal));
        };
        if signal.signal_id != format!("signal.{}", rule.rule_id)
            || SignalKind::parse(&signal.kind) != Some(rule.signal_kind)
            || signal.change_ids.is_empty()
            || !unique(&signal.change_ids)
            || signal
                .change_ids
                .iter()
                .any(|id| !change_by_id.contains_key(id))
            || signal.change_ids
                != change_by_id
                    .values()
                    .filter(|change| change.family == rule.matches_family)
                    .map(|change| change.change_id.clone())
                    .collect::<Vec<_>>()
        {
            return Err(DeltaError::new(DeltaErrorKind::UncatalogedSignal));
        }
        if rule.requires_justification {
            if signal.justification_refs.is_empty() {
                return Err(DeltaError::new(DeltaErrorKind::MissingJustification));
            }
            if !unique(&signal.justification_refs)
                || signal
                    .justification_refs
                    .iter()
                    .any(|reference| !rule.admitted_justification_refs.contains(reference))
            {
                return Err(DeltaError::new(DeltaErrorKind::InvalidJustification));
            }
        } else if !signal.justification_refs.is_empty() {
            return Err(DeltaError::new(DeltaErrorKind::InvalidJustification));
        }
        if signal_by_id
            .insert(signal.signal_id.clone(), signal)
            .is_some()
        {
            return Err(DeltaError::new(DeltaErrorKind::SignalBijection));
        }
    }

    let mut matched_count = 0;
    for (evaluation, rule) in delta.rule_evaluations.iter().zip(&catalog.rules) {
        match evaluation.outcome.as_str() {
            "matched" => {
                matched_count += 1;
                let expected_id = format!("signal.{}", rule.rule_id);
                if evaluation.signal_id.as_deref() != Some(&expected_id)
                    || signal_by_id
                        .get(&expected_id)
                        .is_none_or(|signal| signal.rule != rule.rule)
                {
                    return Err(DeltaError::new(DeltaErrorKind::SignalBijection));
                }
            }
            "not_matched" if evaluation.signal_id.is_none() => {}
            _ => return Err(DeltaError::new(DeltaErrorKind::ContradictoryEvaluation)),
        }
    }
    if matched_count != signal_by_id.len() {
        return Err(DeltaError::new(DeltaErrorKind::SignalBijection));
    }

    let mut preimage = value.clone();
    preimage
        .as_object_mut()
        .ok_or_else(|| DeltaError::new(DeltaErrorKind::MalformedDelta))?
        .remove("delta_fingerprint");
    let supplied = DefinitionFingerprint::parse(&delta.delta_fingerprint)
        .map_err(|_| DeltaError::new(DeltaErrorKind::FingerprintMismatch))?;
    let computed = DefinitionFingerprint::from_json_value(&preimage)
        .map_err(|_| DeltaError::new(DeltaErrorKind::MalformedDelta))?;
    if supplied != computed {
        return Err(DeltaError::new(DeltaErrorKind::FingerprintMismatch));
    }
    Ok(())
}

#[derive(Clone, Debug)]
struct SnapshotView {
    snapshot_id: String,
    repository_id: String,
    workspace_id: String,
    policy: ExactPair,
    record_fingerprint: DefinitionFingerprint,
    state_fingerprint: DefinitionFingerprint,
    consistency: SnapshotConsistency,
    admissibility: SnapshotAdmissibility,
    observations: BTreeMap<String, SnapshotObservation>,
    excluded_families: BTreeSet<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SnapshotConsistency {
    Stable,
    Bounded,
    Unstable,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SnapshotAdmissibility {
    GroundingAndEvidence,
    DiagnosticOnly,
}

impl SnapshotView {
    fn from_snapshot(snapshot: &ContextMemorySnapshot) -> Result<Self, DeltaError> {
        let value = snapshot.normalized_value();
        let object = value
            .as_object()
            .ok_or_else(|| DeltaError::new(DeltaErrorKind::MalformedSnapshot))?;
        if required_string(object, "schema_id")? != "handbook.context-memory-snapshot"
            || required_string(object, "schema_version")? != SCHEMA_VERSION
        {
            return Err(DeltaError::new(DeltaErrorKind::MalformedSnapshot));
        }
        let snapshot_id = required_string(object, "snapshot_id")?.to_owned();
        if !is_safe_text(&snapshot_id) {
            return Err(DeltaError::new(DeltaErrorKind::MalformedSnapshot));
        }
        let identity = object
            .get("repository_identity")
            .and_then(Value::as_object)
            .ok_or_else(|| DeltaError::new(DeltaErrorKind::MalformedSnapshot))?;
        let repository_id = required_string(identity, "repository_id")?.to_owned();
        let workspace_id = required_string(identity, "workspace_id")?.to_owned();
        if !is_safe_text(&repository_id) || !is_safe_text(&workspace_id) {
            return Err(DeltaError::new(DeltaErrorKind::MalformedSnapshot));
        }
        let policy = parse_exact_pair(
            object
                .get("capture")
                .and_then(Value::as_object)
                .and_then(|capture| capture.get("policy"))
                .ok_or_else(|| DeltaError::new(DeltaErrorKind::MalformedSnapshot))?,
            DeltaErrorKind::MalformedSnapshot,
        )?;
        let capture = object
            .get("capture")
            .and_then(Value::as_object)
            .ok_or_else(|| DeltaError::new(DeltaErrorKind::MalformedSnapshot))?;
        let consistency = match required_string(capture, "consistency")? {
            "stable" => SnapshotConsistency::Stable,
            "bounded" => SnapshotConsistency::Bounded,
            "unstable" => SnapshotConsistency::Unstable,
            _ => return Err(DeltaError::new(DeltaErrorKind::MalformedSnapshot)),
        };
        let admissibility = match required_string(object, "admissibility")? {
            "grounding_and_evidence" => SnapshotAdmissibility::GroundingAndEvidence,
            "diagnostic_only" => SnapshotAdmissibility::DiagnosticOnly,
            _ => return Err(DeltaError::new(DeltaErrorKind::MalformedSnapshot)),
        };
        if matches!(consistency, SnapshotConsistency::Unstable)
            != matches!(admissibility, SnapshotAdmissibility::DiagnosticOnly)
        {
            return Err(DeltaError::new(DeltaErrorKind::MalformedSnapshot));
        }
        let record_fingerprint = parse_fingerprint(
            object
                .get("record_fingerprint")
                .and_then(Value::as_str)
                .ok_or_else(|| DeltaError::new(DeltaErrorKind::MalformedSnapshot))?,
            DeltaErrorKind::MalformedSnapshot,
        )?;
        let state_fingerprint = parse_fingerprint(
            object
                .get("state_fingerprint")
                .and_then(Value::as_str)
                .ok_or_else(|| DeltaError::new(DeltaErrorKind::MalformedSnapshot))?,
            DeltaErrorKind::MalformedSnapshot,
        )?;
        if &record_fingerprint != snapshot.record_fingerprint()
            || &state_fingerprint != snapshot.state_fingerprint()
        {
            return Err(DeltaError::new(DeltaErrorKind::MalformedSnapshot));
        }

        let mut observations = BTreeMap::new();
        for observation in object
            .get("family_observations")
            .and_then(Value::as_array)
            .ok_or_else(|| DeltaError::new(DeltaErrorKind::MalformedSnapshot))?
        {
            let observation = SnapshotObservation::parse(observation)?;
            if observations
                .insert(observation.family.clone(), observation)
                .is_some()
            {
                return Err(DeltaError::new(DeltaErrorKind::MalformedSnapshot));
            }
        }
        let mut excluded_families = BTreeSet::new();
        for exclusion in object
            .get("excluded_families")
            .and_then(Value::as_array)
            .ok_or_else(|| DeltaError::new(DeltaErrorKind::MalformedSnapshot))?
        {
            let family = exclusion
                .as_object()
                .and_then(|value| value.get("family"))
                .and_then(Value::as_str)
                .filter(|family| is_identifier(family))
                .ok_or_else(|| DeltaError::new(DeltaErrorKind::MalformedSnapshot))?;
            if observations.contains_key(family) || !excluded_families.insert(family.to_owned()) {
                return Err(DeltaError::new(DeltaErrorKind::MalformedSnapshot));
            }
        }
        if observations.is_empty() {
            return Err(DeltaError::new(DeltaErrorKind::MalformedSnapshot));
        }
        Ok(Self {
            snapshot_id,
            repository_id,
            workspace_id,
            policy,
            record_fingerprint,
            state_fingerprint,
            consistency,
            admissibility,
            observations,
            excluded_families,
        })
    }

    fn is_delta_admissible(&self) -> bool {
        matches!(
            (self.consistency, self.admissibility),
            (
                SnapshotConsistency::Stable | SnapshotConsistency::Bounded,
                SnapshotAdmissibility::GroundingAndEvidence
            )
        )
    }

    fn as_value(&self) -> Value {
        json!({
            "ref": self.snapshot_id,
            "record_fingerprint": self.record_fingerprint.to_string(),
            "state_fingerprint": self.state_fingerprint.to_string(),
        })
    }
}

#[derive(Clone, Debug)]
struct SnapshotObservation {
    family: String,
    source_adapter: ExactPair,
    payload_fingerprint: DefinitionFingerprint,
}

impl SnapshotObservation {
    fn parse(value: &Value) -> Result<Self, DeltaError> {
        let object = value
            .as_object()
            .ok_or_else(|| DeltaError::new(DeltaErrorKind::MalformedSnapshot))?;
        let family = required_string(object, "family")?.to_owned();
        if !is_identifier(&family) {
            return Err(DeltaError::new(DeltaErrorKind::MalformedSnapshot));
        }
        let source_adapter = parse_exact_pair(
            object
                .get("source_adapter")
                .ok_or_else(|| DeltaError::new(DeltaErrorKind::MalformedSnapshot))?,
            DeltaErrorKind::MalformedSnapshot,
        )?;
        let payload = object
            .get("payload")
            .ok_or_else(|| DeltaError::new(DeltaErrorKind::MalformedSnapshot))?;
        let supplied = parse_fingerprint(
            required_string(object, "payload_fingerprint")?,
            DeltaErrorKind::MalformedSnapshot,
        )?;
        let computed = DefinitionFingerprint::from_json_value(payload)
            .map_err(|_| DeltaError::new(DeltaErrorKind::MalformedSnapshot))?;
        if supplied != computed {
            return Err(DeltaError::new(DeltaErrorKind::MalformedSnapshot));
        }
        Ok(Self {
            family,
            source_adapter,
            payload_fingerprint: supplied,
        })
    }
}

#[derive(Clone, Debug)]
struct DeltaChange {
    change_id: String,
    family: String,
    stable_key: String,
    before_fingerprint: String,
    after_fingerprint: String,
}

impl DeltaChange {
    fn as_value(&self) -> Value {
        json!({
            "change_id": self.change_id,
            "family": self.family,
            "stable_key": self.stable_key,
            "change_kind": "updated",
            "before_fingerprint": self.before_fingerprint,
            "after_fingerprint": self.after_fingerprint,
        })
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct AuthoredDelta {
    schema_id: String,
    schema_version: String,
    delta_id: String,
    from_snapshot: AuthoredSnapshotRef,
    to_snapshot: AuthoredSnapshotRef,
    compatibility: AuthoredCompatibility,
    changes: Vec<AuthoredChange>,
    rule_evaluations: Vec<AuthoredEvaluation>,
    signals: Vec<AuthoredSignal>,
    delta_fingerprint: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct AuthoredSnapshotRef {
    #[serde(rename = "ref")]
    reference: String,
    record_fingerprint: String,
    state_fingerprint: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct AuthoredCompatibility {
    comparison_contract: ExactPair,
    drift_rule_catalog: ExactPair,
    compared_state_families: Vec<String>,
    excluded_state_families: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct AuthoredChange {
    change_id: String,
    family: String,
    stable_key: String,
    change_kind: String,
    before_fingerprint: Option<String>,
    after_fingerprint: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct AuthoredEvaluation {
    rule: ExactPair,
    outcome: String,
    signal_id: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct AuthoredSignal {
    signal_id: String,
    kind: String,
    rule: ExactPair,
    change_ids: Vec<String>,
    evidence_refs: Vec<String>,
    justification_refs: Vec<String>,
}

fn validate_justifications(
    catalog: &DriftCatalog,
    justifications: &BTreeMap<String, Vec<String>>,
) -> Result<(), DeltaError> {
    for (rule_id, refs) in justifications {
        let rule = catalog
            .rules
            .iter()
            .find(|rule| &rule.rule_id == rule_id)
            .ok_or_else(|| DeltaError::new(DeltaErrorKind::InvalidJustification))?;
        if !rule.requires_justification
            || !unique(refs)
            || refs
                .iter()
                .any(|reference| !rule.admitted_justification_refs.contains(reference))
        {
            return Err(DeltaError::new(DeltaErrorKind::InvalidJustification));
        }
    }
    Ok(())
}

fn validate_snapshot_ref(snapshot: &AuthoredSnapshotRef) -> Result<(), DeltaError> {
    if !is_safe_text(&snapshot.reference) {
        return Err(DeltaError::new(DeltaErrorKind::MalformedDelta));
    }
    parse_fingerprint(&snapshot.record_fingerprint, DeltaErrorKind::MalformedDelta)?;
    parse_fingerprint(&snapshot.state_fingerprint, DeltaErrorKind::MalformedDelta)?;
    Ok(())
}

fn validate_exact_pair(pair: &ExactPair, kind: DeltaErrorKind) -> Result<(), DeltaError> {
    ExactDefinitionRef::parse(&pair.reference).map_err(|_| DeltaError::new(kind))?;
    DefinitionFingerprint::parse(&pair.fingerprint).map_err(|_| DeltaError::new(kind))?;
    Ok(())
}

fn parse_exact_pair(value: &Value, kind: DeltaErrorKind) -> Result<ExactPair, DeltaError> {
    let pair = serde_json::from_value(value.clone()).map_err(|_| DeltaError::new(kind))?;
    validate_exact_pair(&pair, kind)?;
    Ok(pair)
}

fn parse_fingerprint(
    value: &str,
    kind: DeltaErrorKind,
) -> Result<DefinitionFingerprint, DeltaError> {
    DefinitionFingerprint::parse(value).map_err(|_| DeltaError::new(kind))
}

fn exact_pair_value(pair: &ExactPair) -> Value {
    json!({"ref": pair.reference, "fingerprint": pair.fingerprint})
}

fn required_string<'a>(
    object: &'a serde_json::Map<String, Value>,
    field: &str,
) -> Result<&'a str, DeltaError> {
    object
        .get(field)
        .and_then(Value::as_str)
        .ok_or_else(|| DeltaError::new(DeltaErrorKind::MalformedSnapshot))
}

fn is_strictly_ordered_identifiers(values: &[String]) -> bool {
    !values.is_empty()
        && values.iter().all(|value| is_identifier(value))
        && values.windows(2).all(|pair| pair[0] < pair[1])
}

fn unique(values: &[String]) -> bool {
    values.iter().collect::<BTreeSet<_>>().len() == values.len()
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
