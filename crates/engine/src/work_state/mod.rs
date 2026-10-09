//! Immutable same-project open leaf registration. Retained history is distinct
//! from current operating permission; terminal publication is the commit point.
mod store;
mod transaction;
mod types;
pub use transaction::{NativeWorkStateV1, WorkWriteBoundaryV1};
pub use types::*;

use crate::local_authority::{refuse, LocalAuthorityRefusalV1};
use serde::{Deserialize, Serialize};
const MAX: usize = 1024 * 1024;
pub const WORK_STATE_ROOT_V1: &str = ".handbook/work-state/v1";
pub const WORK_REGISTRATION_RULE_V1: &str = "handbook.operating.work-registration@1.0.0";
pub const WORK_IDENTITY_SCHEME_V1: &str = "handbook.work-item-identity@1.0.0";
pub const ORDINARY_LEAF_COMPOSITION_V1: &str = "handbook.work-composition.ordinary-leaf@1.0.0";
/// Closed native reach: work-state subtree, exact retained bootstrap/owner
/// anchor files, and the one fixed unpublished bootstrap directory. This grants
/// no arbitrary repository path or other owner's mutation.
pub const WORK_STATE_FULL_OWNER_REACH_V1: &str = "handbook.work-state.full-owner@1.0.0";

fn encode<T: Serialize>(v: &T) -> Result<Vec<u8>, LocalAuthorityRefusalV1> {
    let bytes =
        crate::definition_identity::canonical_json_bytes(&serde_json::to_value(v).map_err(codec)?)
            .map_err(codec)?;
    if bytes.len() > MAX {
        return Err(codec("record bound"));
    }
    Ok(bytes)
}
fn decode<T: for<'de> Deserialize<'de>>(bytes: &[u8]) -> Result<T, LocalAuthorityRefusalV1> {
    if bytes.len() > MAX {
        return Err(codec("record bound"));
    }
    serde_json::from_value(crate::parse_schema_json(bytes).map_err(codec)?).map_err(codec)
}
fn fp<T: Serialize>(domain: &str, v: &T) -> Result<String, LocalAuthorityRefusalV1> {
    Ok(crate::DefinitionFingerprint::from_bytes(&encode(
        &serde_json::json!({"domain":domain,"content":v}),
    )?)
    .to_string())
}
fn codec(_: impl std::fmt::Display) -> LocalAuthorityRefusalV1 {
    refuse(
        "work_codec",
        "bounded complete duplicate-safe closed JSON required",
    )
}
fn fs_error(e: crate::RepositoryDurabilityErrorV1) -> LocalAuthorityRefusalV1 {
    refuse("work_retained_state", e.detail())
}
fn token(v: &str) -> Result<(), LocalAuthorityRefusalV1> {
    if v.is_empty()
        || v.len() > 128
        || !v
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.'))
        || matches!(v, "." | "..")
    {
        Err(refuse("work_key", "bounded ASCII key required"))
    } else {
        Ok(())
    }
}
fn text(v: &str) -> Result<(), LocalAuthorityRefusalV1> {
    if v.trim().is_empty() || v.len() > 4096 || v.contains('\0') {
        Err(refuse("work_text", "explicit bounded text required"))
    } else {
        Ok(())
    }
}
