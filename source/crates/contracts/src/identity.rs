use semver::Version;
use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sha2::{Digest, Sha256};
use std::error::Error;
use std::fmt;

const MAX_ID_BYTES: usize = 255;
const MAX_VERSION_BYTES: usize = 255;
const MAX_EXACT_REF_BYTES: usize = MAX_ID_BYTES + MAX_VERSION_BYTES + 1;

/// A closed validation or exact-identity failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ContractError {
    /// A stable lowercase dot-separated identifier was invalid.
    InvalidId,
    /// A full canonical SemVer was invalid.
    InvalidVersion,
    /// An exact `id@version` reference was invalid.
    InvalidExactRef,
    /// A lowercase SHA-256 fingerprint was invalid.
    InvalidFingerprint,
    /// A bounded string or collection exceeded its limit.
    BoundExceeded,
    /// A required collection or string was empty.
    Empty,
    /// A closed invariant was not satisfied.
    InvalidInvariant,
    /// A supplied derived value did not match its recomputed value.
    DerivedValueMismatch,
    /// A matcher binding was not exactly registered by the owner.
    UnknownMatcher,
    /// Canonical serialization failed.
    Canonicalization,
}

impl fmt::Display for ContractError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::InvalidId => "invalid stable identifier",
            Self::InvalidVersion => "invalid full semantic version",
            Self::InvalidExactRef => "invalid exact reference",
            Self::InvalidFingerprint => "invalid SHA-256 fingerprint",
            Self::BoundExceeded => "contract bound exceeded",
            Self::Empty => "required contract value is empty",
            Self::InvalidInvariant => "contract invariant is invalid",
            Self::DerivedValueMismatch => "derived contract value does not match",
            Self::UnknownMatcher => "matcher binding is not exactly registered",
            Self::Canonicalization => "contract canonicalization failed",
        })
    }
}

impl Error for ContractError {}

/// A stable lowercase dot-separated machine identifier.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ContractIdV1(String);

impl ContractIdV1 {
    /// Parses a bounded identifier with at least two dot-separated segments.
    pub fn parse(value: &str) -> Result<Self, ContractError> {
        if value.is_empty()
            || value.len() > MAX_ID_BYTES
            || !value.is_ascii()
            || !value.contains('.')
            || !value.split('.').all(valid_id_segment)
        {
            return Err(ContractError::InvalidId);
        }
        Ok(Self(value.to_owned()))
    }

    /// Returns the canonical identifier.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ContractIdV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl Serialize for ContractIdV1 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for ContractIdV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::parse(&value).map_err(D::Error::custom)
    }
}

/// A canonical full semantic version.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct VersionV1 {
    canonical: String,
    parsed: Version,
}

impl VersionV1 {
    /// Parses a complete `major.minor.patch` SemVer, including optional
    /// prerelease and build identifiers.
    pub fn parse(value: &str) -> Result<Self, ContractError> {
        if value.is_empty() || value.len() > MAX_VERSION_BYTES || !value.is_ascii() {
            return Err(ContractError::InvalidVersion);
        }
        let parsed = Version::parse(value).map_err(|_| ContractError::InvalidVersion)?;
        if parsed.to_string() != value {
            return Err(ContractError::InvalidVersion);
        }
        Ok(Self {
            canonical: value.to_owned(),
            parsed,
        })
    }

    /// Returns the major component.
    pub fn major(&self) -> u64 {
        self.parsed.major
    }

    /// Returns the minor component.
    pub fn minor(&self) -> u64 {
        self.parsed.minor
    }

    /// Returns the patch component.
    pub fn patch(&self) -> u64 {
        self.parsed.patch
    }

    /// Returns the canonical full SemVer.
    pub fn as_str(&self) -> &str {
        &self.canonical
    }
}

impl fmt::Display for VersionV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.canonical)
    }
}

impl Serialize for VersionV1 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for VersionV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::parse(&value).map_err(D::Error::custom)
    }
}

/// An exact definition reference derived from a stable ID and full SemVer.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ExactRefV1 {
    canonical: String,
    identity: ContractIdV1,
    version: VersionV1,
}

impl ExactRefV1 {
    /// Derives the exact reference mechanically as `id@version`.
    pub fn derive(identity: &ContractIdV1, version: &VersionV1) -> Self {
        Self {
            canonical: format!("{identity}@{version}"),
            identity: identity.clone(),
            version: version.clone(),
        }
    }

    /// Parses and rederives an exact reference.
    pub fn parse(value: &str) -> Result<Self, ContractError> {
        if value.is_empty() || value.len() > MAX_EXACT_REF_BYTES || !value.is_ascii() {
            return Err(ContractError::InvalidExactRef);
        }
        let (identity, version) = value
            .rsplit_once('@')
            .ok_or(ContractError::InvalidExactRef)?;
        let identity = ContractIdV1::parse(identity).map_err(|_| ContractError::InvalidExactRef)?;
        let version = VersionV1::parse(version).map_err(|_| ContractError::InvalidExactRef)?;
        let derived = Self::derive(&identity, &version);
        if derived.as_str() != value {
            return Err(ContractError::InvalidExactRef);
        }
        Ok(derived)
    }

    /// Returns the stable identity component.
    pub fn identity(&self) -> &ContractIdV1 {
        &self.identity
    }

    /// Returns the exact version component.
    pub fn version(&self) -> &VersionV1 {
        &self.version
    }

    /// Returns the canonical exact reference.
    pub fn as_str(&self) -> &str {
        &self.canonical
    }
}

impl fmt::Display for ExactRefV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.canonical)
    }
}

impl Serialize for ExactRefV1 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.canonical)
    }
}

impl<'de> Deserialize<'de> for ExactRefV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::parse(&value).map_err(D::Error::custom)
    }
}

/// A lowercase `sha256:<64-hex>` semantic fingerprint.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FingerprintV1(String);

impl FingerprintV1 {
    /// Parses a canonical lowercase SHA-256 fingerprint.
    pub fn parse(value: &str) -> Result<Self, ContractError> {
        let bytes = value.as_bytes();
        if bytes.len() != 71
            || !value.starts_with("sha256:")
            || !bytes[7..]
                .iter()
                .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
        {
            return Err(ContractError::InvalidFingerprint);
        }
        Ok(Self(value.to_owned()))
    }

    pub(crate) fn from_bytes(bytes: &[u8]) -> Self {
        let digest: [u8; 32] = Sha256::digest(bytes).into();
        const HEX: &[u8; 16] = b"0123456789abcdef";
        let mut value = String::with_capacity(71);
        value.push_str("sha256:");
        for byte in digest {
            value.push(HEX[(byte >> 4) as usize] as char);
            value.push(HEX[(byte & 0x0f) as usize] as char);
        }
        Self(value)
    }

    /// Returns the canonical fingerprint.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Serialize for FingerprintV1 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for FingerprintV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::parse(&value).map_err(D::Error::custom)
    }
}

/// One exact reference and fingerprint pair.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExactBindingV1 {
    #[serde(rename = "ref")]
    reference: ExactRefV1,
    fingerprint: FingerprintV1,
}

impl ExactBindingV1 {
    /// Creates an exact binding from validated parts.
    pub fn new(reference: ExactRefV1, fingerprint: FingerprintV1) -> Self {
        Self {
            reference,
            fingerprint,
        }
    }

    /// Returns the exact definition reference.
    pub fn reference(&self) -> &ExactRefV1 {
        &self.reference
    }

    /// Returns the exact definition fingerprint.
    pub fn fingerprint(&self) -> &FingerprintV1 {
        &self.fingerprint
    }

    /// Returns true only when both the exact ref and fingerprint match.
    pub fn matches(&self, other: &Self) -> bool {
        self == other
    }
}

fn valid_id_segment(segment: &str) -> bool {
    let bytes = segment.as_bytes();
    !bytes.is_empty()
        && bytes[0].is_ascii_lowercase()
        && bytes
            .last()
            .is_some_and(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit())
        && bytes
            .iter()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || *byte == b'-')
}
