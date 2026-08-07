mod delta;
mod policy;
mod record;
mod redaction;

#[cfg(test)]
mod tests;

use crate::DefinitionFingerprint;
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

fn json_value(bytes: &[u8]) -> Result<Value, GroundingSourceError> {
    serde_json::from_slice(bytes).map_err(|_| GroundingSourceError::InvalidSource)
}

fn fingerprint_of(value: &Value) -> Result<DefinitionFingerprint, GroundingSourceError> {
    DefinitionFingerprint::from_json_value(value).map_err(|_| GroundingSourceError::InvalidSource)
}
