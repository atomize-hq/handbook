//! Handbook's operational dock owner and one-shot process protocol.
//!
//! This crate owns operational identities and records only. Contract evidence,
//! verdict, gate, lifecycle, local-closeout, posture, and promotion semantics
//! remain owned by `handbook-contracts`.

mod bounds;
mod canonical;
mod executor;
mod local_source;
mod manifest;
pub mod operation_admission;
mod operation_claims;
mod protocol;
mod registry;
mod run_admission;
mod store;

pub use bounds::*;
pub use executor::*;
pub use local_source::*;
pub use manifest::*;
pub use protocol::*;
pub use registry::*;
pub use run_admission::*;
pub use store::*;

use std::error::Error;
use std::fmt;

/// A closed dock identity, admission, or bound failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum DockErrorV1 {
    InvalidIdentity,
    InvalidInvariant,
    BoundExceeded,
    Canonicalization,
    DerivedValueMismatch,
    Substituted,
    Unknown,
}

impl fmt::Display for DockErrorV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::InvalidIdentity => "invalid dock identity",
            Self::InvalidInvariant => "dock invariant is invalid",
            Self::BoundExceeded => "dock bound exceeded",
            Self::Canonicalization => "dock canonicalization failed",
            Self::DerivedValueMismatch => "derived dock value does not match",
            Self::Substituted => "exact dock binding was substituted",
            Self::Unknown => "exact dock binding is unknown",
        })
    }
}

impl Error for DockErrorV1 {}
