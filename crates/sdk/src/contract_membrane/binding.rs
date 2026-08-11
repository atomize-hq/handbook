use super::ContractError;
use serde::de::Error as _;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

const MAX_SEMVER_BYTES: usize = 255;
const MAX_IDENTITY_BYTES: usize = 255;
const MAX_EXACT_REF_BYTES: usize = 511;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SemVer {
    canonical: String,
    major: u64,
    minor: u64,
    patch: u64,
    prerelease: Option<String>,
    build: Option<String>,
}

impl SemVer {
    pub fn parse(value: &str) -> Result<Self, ContractError> {
        if value.is_empty() || value.len() > MAX_SEMVER_BYTES || !value.is_ascii() {
            return Err(ContractError::InvalidSemVer);
        }

        let (without_build, build) = split_optional_once(value, '+')?;
        let (core, prerelease) = match without_build.split_once('-') {
            Some((core, prerelease)) if !prerelease.is_empty() => (core, Some(prerelease)),
            Some(_) => return Err(ContractError::InvalidSemVer),
            None => (without_build, None),
        };
        let mut components = core.split('.');
        let major = parse_core_component(components.next())?;
        let minor = parse_core_component(components.next())?;
        let patch = parse_core_component(components.next())?;
        if components.next().is_some() {
            return Err(ContractError::InvalidSemVer);
        }
        if let Some(prerelease) = prerelease {
            validate_identifiers(prerelease, true)?;
        }
        if let Some(build) = build {
            validate_identifiers(build, false)?;
        }

        Ok(Self {
            canonical: value.to_owned(),
            major,
            minor,
            patch,
            prerelease: prerelease.map(str::to_owned),
            build: build.map(str::to_owned),
        })
    }

    pub fn major(&self) -> u64 {
        self.major
    }

    pub fn minor(&self) -> u64 {
        self.minor
    }

    pub fn patch(&self) -> u64 {
        self.patch
    }

    pub fn prerelease(&self) -> Option<&str> {
        self.prerelease.as_deref()
    }

    pub fn build(&self) -> Option<&str> {
        self.build.as_deref()
    }

    pub fn as_str(&self) -> &str {
        &self.canonical
    }
}

impl fmt::Display for SemVer {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.canonical)
    }
}

impl Serialize for SemVer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.canonical)
    }
}

impl<'de> Deserialize<'de> for SemVer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer).map_err(sanitize_deserialize_error)?;
        Self::parse(&value).map_err(D::Error::custom)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExactRef {
    canonical: String,
    identity: String,
    version: SemVer,
}

impl ExactRef {
    pub fn parse(value: &str) -> Result<Self, ContractError> {
        if value.is_empty() || value.len() > MAX_EXACT_REF_BYTES || !value.is_ascii() {
            return Err(ContractError::InvalidExactRef);
        }
        let (identity, version) = value
            .rsplit_once('@')
            .ok_or(ContractError::InvalidExactRef)?;
        if !valid_identity(identity) {
            return Err(ContractError::InvalidExactRef);
        }
        let version = SemVer::parse(version).map_err(|_| ContractError::InvalidExactRef)?;
        Ok(Self {
            canonical: value.to_owned(),
            identity: identity.to_owned(),
            version,
        })
    }

    pub fn identity(&self) -> &str {
        &self.identity
    }

    pub fn version(&self) -> &SemVer {
        &self.version
    }

    pub fn as_str(&self) -> &str {
        &self.canonical
    }
}

impl fmt::Display for ExactRef {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.canonical)
    }
}

impl Serialize for ExactRef {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.canonical)
    }
}

impl<'de> Deserialize<'de> for ExactRef {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer).map_err(sanitize_deserialize_error)?;
        Self::parse(&value).map_err(D::Error::custom)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Fingerprint(String);

impl Fingerprint {
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

    pub fn from_sha256(value: [u8; 32]) -> Self {
        const HEX: &[u8; 16] = b"0123456789abcdef";
        let mut canonical = String::with_capacity(71);
        canonical.push_str("sha256:");
        for byte in value {
            canonical.push(HEX[(byte >> 4) as usize] as char);
            canonical.push(HEX[(byte & 0x0f) as usize] as char);
        }
        Self(canonical)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Serialize for Fingerprint {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for Fingerprint {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer).map_err(sanitize_deserialize_error)?;
        Self::parse(&value).map_err(D::Error::custom)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExactBinding {
    reference: ExactRef,
    fingerprint: Fingerprint,
}

impl ExactBinding {
    pub fn new(reference: ExactRef, fingerprint: Fingerprint) -> Self {
        Self {
            reference,
            fingerprint,
        }
    }

    pub fn reference(&self) -> &ExactRef {
        &self.reference
    }

    pub fn fingerprint(&self) -> &Fingerprint {
        &self.fingerprint
    }
}

impl Serialize for ExactBinding {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut object = serializer.serialize_struct("ExactBinding", 2)?;
        object.serialize_field("ref", &self.reference)?;
        object.serialize_field("fingerprint", &self.fingerprint)?;
        object.end()
    }
}

impl<'de> Deserialize<'de> for ExactBinding {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct RawBinding {
            #[serde(rename = "ref")]
            reference: ExactRef,
            fingerprint: Fingerprint,
        }

        let raw = RawBinding::deserialize(deserializer).map_err(sanitize_deserialize_error)?;
        Ok(Self::new(raw.reference, raw.fingerprint))
    }
}

fn split_optional_once(
    value: &str,
    separator: char,
) -> Result<(&str, Option<&str>), ContractError> {
    let mut parts = value.split(separator);
    let first = parts.next().ok_or(ContractError::InvalidSemVer)?;
    let second = parts.next();
    if parts.next().is_some() || second.is_some_and(str::is_empty) {
        return Err(ContractError::InvalidSemVer);
    }
    Ok((first, second))
}

fn parse_core_component(component: Option<&str>) -> Result<u64, ContractError> {
    let component = component.ok_or(ContractError::InvalidSemVer)?;
    if component.is_empty()
        || !component.bytes().all(|byte| byte.is_ascii_digit())
        || (component.len() > 1 && component.starts_with('0'))
    {
        return Err(ContractError::InvalidSemVer);
    }
    component.parse().map_err(|_| ContractError::InvalidSemVer)
}

fn validate_identifiers(value: &str, prerelease: bool) -> Result<(), ContractError> {
    for identifier in value.split('.') {
        if identifier.is_empty()
            || !identifier
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
            || (prerelease
                && identifier.bytes().all(|byte| byte.is_ascii_digit())
                && identifier.len() > 1
                && identifier.starts_with('0'))
        {
            return Err(ContractError::InvalidSemVer);
        }
    }
    Ok(())
}

fn valid_identity(value: &str) -> bool {
    let bytes = value.as_bytes();
    !bytes.is_empty()
        && bytes.len() <= MAX_IDENTITY_BYTES
        && bytes[0].is_ascii_lowercase()
        && bytes[1..].iter().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'.' | b'_' | b'-')
        })
}

fn sanitize_deserialize_error<E: serde::de::Error>(error: E) -> E {
    E::custom(super::serialization::classify_contract_error_message(
        &error.to_string(),
    ))
}
