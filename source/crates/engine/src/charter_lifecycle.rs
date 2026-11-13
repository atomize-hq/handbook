use crate::definition_identity::canonical_json_bytes;
use crate::DefinitionFingerprint;
use serde::Serialize;
use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CharterLifecycleState {
    Current,
    ReviewRequired,
    ReassessmentRequired,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CharterLifecycleEventKind {
    Review,
    ProductionPostureChanged,
    TrustBoundaryChanged,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CharterLifecycleEvent {
    pub event_fingerprint: String,
    pub basis_artifact_fingerprint: String,
    pub kind: CharterLifecycleEventKind,
    pub evidence_ref: String,
    pub evidence_fingerprint: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CharterLifecycleObservation {
    pub event_fingerprint: String,
    pub basis_artifact_fingerprint: String,
    pub kind: CharterLifecycleEventKind,
    pub evidence_ref: String,
    pub evidence_fingerprint: String,
    pub reopened_coverage_ids: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CharterLifecycleTransition {
    pub prior_state: CharterLifecycleState,
    pub result_state: CharterLifecycleState,
    pub new_observations: Vec<CharterLifecycleObservation>,
    pub active_observations: Vec<CharterLifecycleObservation>,
    pub reopened_coverage_ids: Vec<String>,
    pub state_fingerprint: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CharterLifecycleErrorKind {
    InvalidEvent,
    StaleBasis,
    ClearanceIncomplete,
    FingerprintFailed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CharterLifecycleError {
    kind: CharterLifecycleErrorKind,
    detail: String,
}

impl CharterLifecycleError {
    fn new(kind: CharterLifecycleErrorKind, detail: impl Into<String>) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }

    pub fn kind(&self) -> CharterLifecycleErrorKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

pub fn apply_charter_lifecycle_events(
    prior_state: CharterLifecycleState,
    active_observations: &[CharterLifecycleObservation],
    mut events: Vec<CharterLifecycleEvent>,
) -> Result<CharterLifecycleTransition, CharterLifecycleError> {
    let active_fingerprints = active_observations
        .iter()
        .map(|observation| observation.event_fingerprint.as_str())
        .collect::<BTreeSet<_>>();
    let active_basis = active_observations
        .first()
        .map(|observation| observation.basis_artifact_fingerprint.as_str());
    for event in &events {
        validate_event(event)?;
        if active_basis.is_some_and(|basis| basis != event.basis_artifact_fingerprint) {
            return Err(CharterLifecycleError::new(
                CharterLifecycleErrorKind::StaleBasis,
                "lifecycle event basis does not match the active canonical basis",
            ));
        }
    }
    events.sort_by(|left, right| {
        event_precedence(left.kind)
            .cmp(&event_precedence(right.kind))
            .then_with(|| left.event_fingerprint.cmp(&right.event_fingerprint))
    });

    let mut seen_in_batch = BTreeSet::new();
    let mut new_observations = Vec::new();
    for event in events {
        if active_fingerprints.contains(event.event_fingerprint.as_str())
            || !seen_in_batch.insert(event.event_fingerprint.clone())
        {
            continue;
        }
        new_observations.push(CharterLifecycleObservation {
            event_fingerprint: event.event_fingerprint,
            basis_artifact_fingerprint: event.basis_artifact_fingerprint,
            kind: event.kind,
            evidence_ref: event.evidence_ref,
            evidence_fingerprint: event.evidence_fingerprint,
            reopened_coverage_ids: reopened_coverage(event.kind)
                .iter()
                .map(|value| (*value).to_owned())
                .collect(),
        });
    }

    let mut active = active_observations.to_vec();
    active.extend(new_observations.iter().cloned());
    let result_state = visible_state(&active).unwrap_or(prior_state);
    let reopened = active
        .iter()
        .flat_map(|observation| observation.reopened_coverage_ids.iter().cloned())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let mut active_fingerprints = active
        .iter()
        .map(|observation| observation.event_fingerprint.clone())
        .collect::<Vec<_>>();
    active_fingerprints.sort();
    let state_fingerprint = fingerprint_value(serde_json::json!({
        "prior_state": prior_state,
        "result_state": result_state,
        "active_observation_fingerprints": active_fingerprints,
        "reopened_coverage_ids": reopened,
    }))?;

    Ok(CharterLifecycleTransition {
        prior_state,
        result_state,
        new_observations,
        active_observations: active,
        reopened_coverage_ids: reopened,
        state_fingerprint,
    })
}

pub fn address_charter_lifecycle(
    active_observations: &[CharterLifecycleObservation],
    addressed_event_fingerprints: &[String],
    reevaluated_coverage_ids: &[String],
) -> Result<CharterLifecycleState, CharterLifecycleError> {
    let required_events = active_observations
        .iter()
        .map(|observation| observation.event_fingerprint.as_str())
        .collect::<BTreeSet<_>>();
    let addressed_events = addressed_event_fingerprints
        .iter()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    let required_coverage = active_observations
        .iter()
        .flat_map(|observation| observation.reopened_coverage_ids.iter().map(String::as_str))
        .collect::<BTreeSet<_>>();
    let reevaluated_coverage = reevaluated_coverage_ids
        .iter()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    if required_events != addressed_events || required_coverage != reevaluated_coverage {
        return Err(CharterLifecycleError::new(
            CharterLifecycleErrorKind::ClearanceIncomplete,
            "addressing promotion must cover every active observation and exactly the reopened coverage IDs",
        ));
    }
    Ok(CharterLifecycleState::Current)
}

fn validate_event(event: &CharterLifecycleEvent) -> Result<(), CharterLifecycleError> {
    if DefinitionFingerprint::parse(&event.event_fingerprint).is_err()
        || DefinitionFingerprint::parse(&event.basis_artifact_fingerprint).is_err()
        || DefinitionFingerprint::parse(&event.evidence_fingerprint).is_err()
        || event.evidence_ref.trim().is_empty()
    {
        return Err(CharterLifecycleError::new(
            CharterLifecycleErrorKind::InvalidEvent,
            "lifecycle event identity, basis, evidence ref, or evidence fingerprint is invalid",
        ));
    }
    Ok(())
}

fn event_precedence(kind: CharterLifecycleEventKind) -> u8 {
    match kind {
        CharterLifecycleEventKind::ProductionPostureChanged
        | CharterLifecycleEventKind::TrustBoundaryChanged => 0,
        CharterLifecycleEventKind::Review => 1,
    }
}

fn reopened_coverage(kind: CharterLifecycleEventKind) -> &'static [&'static str] {
    match kind {
        CharterLifecycleEventKind::Review => &[],
        CharterLifecycleEventKind::ProductionPostureChanged => {
            &["operational_reality.production_state"]
        }
        CharterLifecycleEventKind::TrustBoundaryChanged => &["governance.exception_policy"],
    }
}

fn visible_state(
    active_observations: &[CharterLifecycleObservation],
) -> Option<CharterLifecycleState> {
    if active_observations.iter().any(|observation| {
        matches!(
            observation.kind,
            CharterLifecycleEventKind::ProductionPostureChanged
                | CharterLifecycleEventKind::TrustBoundaryChanged
        )
    }) {
        Some(CharterLifecycleState::ReassessmentRequired)
    } else if active_observations
        .iter()
        .any(|observation| observation.kind == CharterLifecycleEventKind::Review)
    {
        Some(CharterLifecycleState::ReviewRequired)
    } else {
        None
    }
}

fn fingerprint_value(value: serde_json::Value) -> Result<String, CharterLifecycleError> {
    let bytes = canonical_json_bytes(&value).map_err(|_| {
        CharterLifecycleError::new(
            CharterLifecycleErrorKind::FingerprintFailed,
            "lifecycle state fingerprint input could not be canonicalized",
        )
    })?;
    Ok(DefinitionFingerprint::from_bytes(&bytes).to_string())
}
