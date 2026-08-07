mod delta;
mod policy;
mod record;
mod redaction;

#[cfg(test)]
mod tests;

use crate::{DefinitionFingerprint, ExactDefinitionRef};
use delta::{derive_snapshot_delta, load_drift_catalog, SnapshotEndpoint};
use policy::{load_policy, validate_capture_input};
use record::build_snapshot;
use serde::Deserialize;
use serde_json::Value;

#[derive(Clone, Debug)]
pub(crate) struct GroundingSourcePair {
    current_snapshot: Value,
    delta: Value,
    current_snapshot_fingerprint: DefinitionFingerprint,
    delta_route_fingerprint: DefinitionFingerprint,
    delta_ref: String,
}

#[derive(Clone, Debug)]
pub(crate) struct GroundingTransition {
    prior_end_snapshot_fingerprint: DefinitionFingerprint,
    session_start_snapshot_fingerprint: DefinitionFingerprint,
    session_end_snapshot_fingerprint: DefinitionFingerprint,
    grounding_delta_route_fingerprint: DefinitionFingerprint,
    grounding_delta_ref: String,
    session_delta_route_fingerprint: DefinitionFingerprint,
    session_delta_ref: String,
}

impl GroundingTransition {
    pub(crate) fn prior_end_snapshot_fingerprint(&self) -> &DefinitionFingerprint {
        &self.prior_end_snapshot_fingerprint
    }

    pub(crate) fn session_start_snapshot_fingerprint(&self) -> &DefinitionFingerprint {
        &self.session_start_snapshot_fingerprint
    }

    pub(crate) fn session_end_snapshot_fingerprint(&self) -> &DefinitionFingerprint {
        &self.session_end_snapshot_fingerprint
    }

    pub(crate) fn grounding_delta_route_fingerprint(&self) -> &DefinitionFingerprint {
        &self.grounding_delta_route_fingerprint
    }

    pub(crate) fn grounding_delta_ref(&self) -> &str {
        &self.grounding_delta_ref
    }

    pub(crate) fn session_delta_route_fingerprint(&self) -> &DefinitionFingerprint {
        &self.session_delta_route_fingerprint
    }

    pub(crate) fn session_delta_ref(&self) -> &str {
        &self.session_delta_ref
    }
}

impl GroundingSourcePair {
    pub(crate) fn current_snapshot(&self) -> &Value {
        &self.current_snapshot
    }

    pub(crate) fn delta(&self) -> &Value {
        &self.delta
    }

    pub(crate) fn current_snapshot_fingerprint(&self) -> &DefinitionFingerprint {
        &self.current_snapshot_fingerprint
    }

    pub(crate) fn delta_route_fingerprint(&self) -> &DefinitionFingerprint {
        &self.delta_route_fingerprint
    }

    pub(crate) fn delta_ref(&self) -> &str {
        &self.delta_ref
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum GroundingSourceError {
    InvalidSource,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct GroundingDeltaRoute {
    schema_id: String,
    schema_version: String,
    delta_ref: String,
    prior_snapshot_fingerprint: String,
    current_snapshot_fingerprint: String,
    delta_catalog_fingerprint: String,
}

pub(crate) fn derive_grounding_source_pair(
    policy_bytes: &[u8],
    prior_capture_bytes: &[u8],
    prior_record_bytes: &[u8],
    current_capture_bytes: &[u8],
    current_record_bytes: &[u8],
    catalog_bytes: &[u8],
    delta_route_bytes: &[u8],
) -> Result<GroundingSourcePair, GroundingSourceError> {
    let policy = load_policy(policy_bytes).map_err(|_| GroundingSourceError::InvalidSource)?;
    let prior_capture = validate_capture_input(&policy, prior_capture_bytes)
        .map_err(|_| GroundingSourceError::InvalidSource)?;
    let current_capture = validate_capture_input(&policy, current_capture_bytes)
        .map_err(|_| GroundingSourceError::InvalidSource)?;
    let prior = build_snapshot(&policy, &prior_capture, &[], prior_record_bytes)
        .map_err(|_| GroundingSourceError::InvalidSource)?;
    let current = build_snapshot(&policy, &current_capture, &[], current_record_bytes)
        .or_else(|_| build_snapshot(&policy, &current_capture, &[&prior], current_record_bytes))
        .map_err(|_| GroundingSourceError::InvalidSource)?;
    let catalog =
        load_drift_catalog(catalog_bytes).map_err(|_| GroundingSourceError::InvalidSource)?;
    let delta = derive_snapshot_delta(
        SnapshotEndpoint::new(&prior),
        SnapshotEndpoint::new(&current),
        &catalog,
        &Default::default(),
    )
    .map_err(|_| GroundingSourceError::InvalidSource)?;

    let prior_record = json_value(prior_record_bytes)?;
    let current_record = json_value(current_record_bytes)?;
    let catalog_value = json_value(catalog_bytes)?;
    let route_value = json_value(delta_route_bytes)?;
    let route: GroundingDeltaRoute = serde_json::from_value(route_value.clone())
        .map_err(|_| GroundingSourceError::InvalidSource)?;
    if route.schema_id != "handbook.grounding-delta-route"
        || route.schema_version != "1.0"
        || route.delta_ref.is_empty()
        || fingerprint_of(&prior_record)?.as_str() != route.prior_snapshot_fingerprint
        || fingerprint_of(&current_record)?.as_str() != route.current_snapshot_fingerprint
        || fingerprint_of(&catalog_value)?.as_str() != route.delta_catalog_fingerprint
    {
        return Err(GroundingSourceError::InvalidSource);
    }

    Ok(GroundingSourcePair {
        current_snapshot: current.normalized_value().clone(),
        delta: delta.normalized_value().clone(),
        current_snapshot_fingerprint: fingerprint_of(&current_record)?,
        delta_route_fingerprint: fingerprint_of(&route_value)?,
        delta_ref: route.delta_ref,
    })
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn derive_grounding_transition(
    policy_bytes: &[u8],
    prior_end_capture_bytes: &[u8],
    prior_end_record_bytes: &[u8],
    session_start_capture_bytes: &[u8],
    session_start_record_bytes: &[u8],
    session_end_capture_bytes: &[u8],
    session_end_record_bytes: &[u8],
    catalog_bytes: &[u8],
    grounding_delta_route_bytes: &[u8],
    session_delta_route_bytes: &[u8],
) -> Result<GroundingTransition, GroundingSourceError> {
    let policy = load_policy(policy_bytes).map_err(|_| GroundingSourceError::InvalidSource)?;
    let prior_end_capture = validate_capture_input(&policy, prior_end_capture_bytes)
        .map_err(|_| GroundingSourceError::InvalidSource)?;
    let session_start_capture = validate_capture_input(&policy, session_start_capture_bytes)
        .map_err(|_| GroundingSourceError::InvalidSource)?;
    let session_end_capture = validate_capture_input(&policy, session_end_capture_bytes)
        .map_err(|_| GroundingSourceError::InvalidSource)?;
    let prior_end = build_snapshot(&policy, &prior_end_capture, &[], prior_end_record_bytes)
        .map_err(|_| GroundingSourceError::InvalidSource)?;
    let session_start = build_snapshot(
        &policy,
        &session_start_capture,
        &[&prior_end],
        session_start_record_bytes,
    )
    .map_err(|_| GroundingSourceError::InvalidSource)?;
    let session_end = build_snapshot(
        &policy,
        &session_end_capture,
        &[&session_start],
        session_end_record_bytes,
    )
    .map_err(|_| GroundingSourceError::InvalidSource)?;
    let catalog =
        load_drift_catalog(catalog_bytes).map_err(|_| GroundingSourceError::InvalidSource)?;
    derive_snapshot_delta(
        SnapshotEndpoint::new(&prior_end),
        SnapshotEndpoint::new(&session_start),
        &catalog,
        &Default::default(),
    )
    .map_err(|_| GroundingSourceError::InvalidSource)?;
    derive_snapshot_delta(
        SnapshotEndpoint::new(&session_start),
        SnapshotEndpoint::new(&session_end),
        &catalog,
        &Default::default(),
    )
    .map_err(|_| GroundingSourceError::InvalidSource)?;

    let catalog_value = json_value(catalog_bytes)?;
    let grounding_route = validate_grounding_route(
        grounding_delta_route_bytes,
        prior_end_record_bytes,
        session_start_record_bytes,
        &catalog_value,
    )?;
    let session_route = validate_grounding_route(
        session_delta_route_bytes,
        session_start_record_bytes,
        session_end_record_bytes,
        &catalog_value,
    )?;

    Ok(GroundingTransition {
        prior_end_snapshot_fingerprint: fingerprint_of(&json_value(prior_end_record_bytes)?)?,
        session_start_snapshot_fingerprint: fingerprint_of(&json_value(
            session_start_record_bytes,
        )?)?,
        session_end_snapshot_fingerprint: fingerprint_of(&json_value(session_end_record_bytes)?)?,
        grounding_delta_route_fingerprint: grounding_route.1,
        grounding_delta_ref: grounding_route.0.delta_ref,
        session_delta_route_fingerprint: session_route.1,
        session_delta_ref: session_route.0.delta_ref,
    })
}

fn validate_grounding_route(
    route_bytes: &[u8],
    prior_record_bytes: &[u8],
    current_record_bytes: &[u8],
    catalog_value: &Value,
) -> Result<(GroundingDeltaRoute, DefinitionFingerprint), GroundingSourceError> {
    let route_value = json_value(route_bytes)?;
    let route: GroundingDeltaRoute = serde_json::from_value(route_value.clone())
        .map_err(|_| GroundingSourceError::InvalidSource)?;
    if route.schema_id != "handbook.grounding-delta-route"
        || route.schema_version != "1.0"
        || ExactDefinitionRef::parse(&route.delta_ref).is_err()
        || fingerprint_of(&json_value(prior_record_bytes)?)?.as_str()
            != route.prior_snapshot_fingerprint
        || fingerprint_of(&json_value(current_record_bytes)?)?.as_str()
            != route.current_snapshot_fingerprint
        || fingerprint_of(catalog_value)?.as_str() != route.delta_catalog_fingerprint
    {
        return Err(GroundingSourceError::InvalidSource);
    }
    Ok((route, fingerprint_of(&route_value)?))
}

fn json_value(bytes: &[u8]) -> Result<Value, GroundingSourceError> {
    serde_json::from_slice(bytes).map_err(|_| GroundingSourceError::InvalidSource)
}

fn fingerprint_of(value: &Value) -> Result<DefinitionFingerprint, GroundingSourceError> {
    DefinitionFingerprint::from_json_value(value).map_err(|_| GroundingSourceError::InvalidSource)
}
