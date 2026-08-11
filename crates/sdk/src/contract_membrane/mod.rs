mod artifact;
mod binding;
mod bounds;
mod catalog;
mod dto;
mod raw_key;
mod response;
mod serialization;
mod verification;

pub use artifact::{
    ArtifactLocator, ArtifactLocatorRef, ArtifactRef, DigestAlgorithm, Sensitivity,
};
pub use binding::{ExactBinding, ExactRef, Fingerprint, SemVer};
pub use bounds::{
    BoundedString, BoundedVec, ContractLimits, ContractMember, ContractValue, ContractValueRef,
    SafeI64, SafeU64, UtcTimestamp,
};
pub use catalog::{CatalogCursor, CatalogPage, CatalogRoot};
pub use dto::{
    CorrelationRef, Diagnostic, DiagnosticSeverity, DraftVersion, NextAction, NextActionKind,
    Omission, OmissionReason, Problem, ProblemBinding, ProblemCategory, ProofEffect,
    RecheckCondition, RetryDirective, SchemaManifestEntry, SchemaMediaType, SourceBinding,
    WriteReceipt,
};
pub use raw_key::{
    with_owner_idempotency_key, DefinitionPinnedKeyPointer, IdempotencyKeyConsumer,
    RawIdempotencyKey,
};
pub use response::{
    IdempotencyStage, IdempotencyState, IdempotencyStateRef, TerminalFields, TerminalStatus,
};
pub use serialization::{
    canonical_fingerprint, canonical_json_bytes, parse_canonical_json, sha256_fingerprint,
};
pub use verification::{verify, ContractWitness, VerificationError, Verified, VerifyContract};

#[doc(hidden)]
#[allow(unused_imports)]
pub(crate) use raw_key::IdempotencyKeyConsumerSealed;
#[doc(hidden)]
#[allow(unused_imports)]
pub(crate) use verification::ContractWitnessSealed;

use std::fmt;

/// Closed error classification for structurally admitted contract values.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ContractError {
    Empty,
    LimitOutOfRange,
    BoundExceeded,
    InvalidString,
    InvalidNumber,
    InvalidTimestamp,
    InvalidSemVer,
    InvalidExactRef,
    InvalidFingerprint,
    InvalidJson,
    NonCanonicalJson,
    DuplicateMember,
    NonCanonicalOrder,
    InvalidInvariant,
    InvalidArtifactLocator,
    FingerprintMismatch,
}

impl fmt::Display for ContractError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::Empty => "required value is empty",
            Self::LimitOutOfRange => "configured limit is out of range",
            Self::BoundExceeded => "contract bound was exceeded",
            Self::InvalidString => "contract string is invalid",
            Self::InvalidNumber => "contract number is invalid",
            Self::InvalidTimestamp => "UTC timestamp is invalid",
            Self::InvalidSemVer => "semantic version is invalid",
            Self::InvalidExactRef => "exact reference is invalid",
            Self::InvalidFingerprint => "fingerprint is invalid",
            Self::InvalidJson => "JSON document is invalid",
            Self::NonCanonicalJson => "JSON document is not canonical",
            Self::DuplicateMember => "contract member is duplicated",
            Self::NonCanonicalOrder => "contract members are not canonically ordered",
            Self::InvalidInvariant => "contract invariant is invalid",
            Self::InvalidArtifactLocator => "artifact locator is invalid",
            Self::FingerprintMismatch => "fingerprint does not match",
        };
        formatter.write_str(message)
    }
}

impl std::error::Error for ContractError {}
