use semver::Version;
use serde::de::{MapAccess, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::{Map, Number, Value};
use sha2::{Digest, Sha256};
use std::fmt;

pub const MAX_SOURCE_DOCUMENT_BYTES: usize = 1024 * 1024;
pub const MAX_TOTAL_SOURCE_BYTES: usize = 8 * 1024 * 1024;
const MAX_REGISTRY_ERROR_LOCATION_BYTES: usize = 240;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RegistryLoadErrorKind {
    InvalidSourcePath,
    MissingSource,
    SymlinkSource,
    NonRegularSource,
    SourceReadFailure,
    SourceLimitExceeded,
    AggregateLimitExceeded,
    DocumentLimitExceeded,
    ReferenceDepthExceeded,
    DuplicateKey,
    SyntaxError,
    UnknownField,
    UnsupportedRecord,
    UnsupportedDialect,
    UnsupportedMediaType,
    UnsupportedCompatibility,
    UnsupportedStructuralValidationProfile,
    UnsupportedSchemaKeyword,
    UnsupportedSchemaIdentifier,
    InvalidJsonPointer,
    DuplicateResourceIdentity,
    ValidatorTargetMismatch,
    InvalidExactDefinitionRef,
    InvalidFingerprint,
    FingerprintMismatch,
    DuplicateIdentity,
    ConflictingIdentity,
    RemoteReferenceRefused,
    LocalReferenceMissing,
    LocalReferenceOutsideRoot,
    LocalReferenceCycle,
    StableRoleRegistryMismatch,
    UnknownStableRole,
    InvalidStableRoleCategory,
    MissingSchema,
    ConflictingSchema,
    UnsupportedDependency,
    StructuralValidationSetup,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RegistryLoadError {
    kind: RegistryLoadErrorKind,
    location: Option<String>,
    detail: String,
}

impl RegistryLoadError {
    pub fn new(kind: RegistryLoadErrorKind, detail: impl Into<String>) -> Self {
        Self {
            kind,
            location: None,
            detail: detail.into(),
        }
    }

    pub fn at(
        kind: RegistryLoadErrorKind,
        location: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        let location = bounded_registry_error_location(location.into());
        Self {
            kind,
            location: Some(location),
            detail: detail.into(),
        }
    }

    pub fn kind(&self) -> RegistryLoadErrorKind {
        self.kind
    }

    pub fn location(&self) -> Option<&str> {
        self.location.as_deref()
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

fn bounded_registry_error_location(location: String) -> String {
    let bytes = location.as_bytes();
    let looks_absolute = location.starts_with('/')
        || location.starts_with('\\')
        || matches!(bytes, [drive, b':', ..] if drive.is_ascii_alphabetic());
    if location.len() > MAX_REGISTRY_ERROR_LOCATION_BYTES
        || location.chars().any(char::is_control)
        || looks_absolute
    {
        "registry_location".to_string()
    } else {
        location
    }
}

impl fmt::Display for RegistryLoadError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(location) = &self.location {
            write!(formatter, "{location}: {}", self.detail)
        } else {
            formatter.write_str(&self.detail)
        }
    }
}

impl std::error::Error for RegistryLoadError {}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ExactDefinitionRef {
    identity: String,
    version: Version,
    exact_ref: String,
}

impl ExactDefinitionRef {
    pub fn new(identity: &str, version: &str) -> Result<Self, RegistryLoadError> {
        validate_definition_identity(identity)?;
        let parsed_version = validate_canonical_semver(version)?;
        Ok(Self {
            identity: identity.to_owned(),
            version: parsed_version,
            exact_ref: format!("{identity}@{version}"),
        })
    }

    pub fn parse(exact_ref: &str) -> Result<Self, RegistryLoadError> {
        let Some((identity, version)) = exact_ref.split_once('@') else {
            return Err(invalid_exact_ref(
                "exact definition ref must contain one '@' delimiter",
            ));
        };
        if version.contains('@') {
            return Err(invalid_exact_ref(
                "exact definition ref must contain one '@' delimiter",
            ));
        }
        let parsed = Self::new(identity, version)?;
        if parsed.exact_ref != exact_ref {
            return Err(invalid_exact_ref(
                "exact definition ref is not byte-canonical",
            ));
        }
        Ok(parsed)
    }

    pub fn identity(&self) -> &str {
        &self.identity
    }

    pub fn version(&self) -> &Version {
        &self.version
    }

    pub fn as_str(&self) -> &str {
        &self.exact_ref
    }
}

impl fmt::Display for ExactDefinitionRef {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.exact_ref)
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DefinitionFingerprint(String);

impl DefinitionFingerprint {
    pub fn parse(fingerprint: &str) -> Result<Self, RegistryLoadError> {
        let Some(hex) = fingerprint.strip_prefix("sha256:") else {
            return Err(RegistryLoadError::new(
                RegistryLoadErrorKind::InvalidFingerprint,
                "fingerprint must start with lowercase 'sha256:'",
            ));
        };
        if hex.len() != 64
            || !hex
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
        {
            return Err(RegistryLoadError::new(
                RegistryLoadErrorKind::InvalidFingerprint,
                "fingerprint must contain exactly 64 lowercase hexadecimal digits",
            ));
        }
        Ok(Self(fingerprint.to_owned()))
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        let digest = Sha256::digest(bytes);
        Self(format!("sha256:{digest:x}"))
    }

    pub fn from_json_value(value: &Value) -> Result<Self, RegistryLoadError> {
        Ok(Self::from_bytes(&canonical_json_bytes(value)?))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for DefinitionFingerprint {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SourceByteBudget {
    total_bytes: usize,
}

impl SourceByteBudget {
    pub fn admit(&mut self, bytes: usize) -> Result<(), RegistryLoadError> {
        if bytes > MAX_SOURCE_DOCUMENT_BYTES {
            return Err(RegistryLoadError::new(
                RegistryLoadErrorKind::SourceLimitExceeded,
                "source document exceeds the 1 MiB limit",
            ));
        }
        let next = self.total_bytes.checked_add(bytes).ok_or_else(|| {
            RegistryLoadError::new(
                RegistryLoadErrorKind::AggregateLimitExceeded,
                "aggregate source byte count overflowed",
            )
        })?;
        if next > MAX_TOTAL_SOURCE_BYTES {
            return Err(RegistryLoadError::new(
                RegistryLoadErrorKind::AggregateLimitExceeded,
                "aggregate source bytes exceed the 8 MiB limit",
            ));
        }
        self.total_bytes = next;
        Ok(())
    }

    pub fn total_bytes(&self) -> usize {
        self.total_bytes
    }

    pub fn remaining_bytes(&self) -> usize {
        MAX_TOTAL_SOURCE_BYTES - self.total_bytes
    }
}

pub fn parse_definition_yaml(bytes: &[u8]) -> Result<Value, RegistryLoadError> {
    enforce_document_limit(bytes)?;
    serde_yaml_bw::from_slice::<NoDuplicateValue>(bytes)
        .map(|value| value.0)
        .map_err(classify_parse_error)
}

pub fn parse_schema_json(bytes: &[u8]) -> Result<Value, RegistryLoadError> {
    enforce_document_limit(bytes)?;
    let mut deserializer = serde_json::Deserializer::from_slice(bytes);
    let value = NoDuplicateValue::deserialize(&mut deserializer).map_err(classify_parse_error)?;
    deserializer.end().map_err(classify_parse_error)?;
    Ok(value.0)
}

pub(crate) fn canonical_json_bytes(value: &Value) -> Result<Vec<u8>, RegistryLoadError> {
    serde_json_canonicalizer::to_vec(value).map_err(|_| {
        RegistryLoadError::new(
            RegistryLoadErrorKind::SyntaxError,
            "RFC 8785 canonicalization failed",
        )
    })
}

pub(crate) fn fingerprint_serializable<T: Serialize>(
    value: &T,
) -> Result<DefinitionFingerprint, RegistryLoadError> {
    let json = serde_json::to_value(value).map_err(|_| {
        RegistryLoadError::new(
            RegistryLoadErrorKind::SyntaxError,
            "typed JSON conversion failed",
        )
    })?;
    DefinitionFingerprint::from_json_value(&json)
}

fn enforce_document_limit(bytes: &[u8]) -> Result<(), RegistryLoadError> {
    if bytes.len() > MAX_SOURCE_DOCUMENT_BYTES {
        return Err(RegistryLoadError::new(
            RegistryLoadErrorKind::SourceLimitExceeded,
            "source document exceeds the 1 MiB limit",
        ));
    }
    Ok(())
}

fn validate_definition_identity(identity: &str) -> Result<(), RegistryLoadError> {
    if !(3..=255).contains(&identity.len()) || !identity.is_ascii() {
        return Err(invalid_exact_ref(
            "definition identity must contain 3-255 lowercase ASCII bytes",
        ));
    }
    let segments = identity.split('.').collect::<Vec<_>>();
    if segments.len() < 2 {
        return Err(invalid_exact_ref(
            "definition identity must contain at least two dot-separated segments",
        ));
    }
    for segment in segments {
        if segment.is_empty() || segment.len() > 63 {
            return Err(invalid_exact_ref(
                "definition identity segments must contain 1-63 bytes",
            ));
        }
        let bytes = segment.as_bytes();
        if !bytes[0].is_ascii_lowercase()
            || !bytes
                .last()
                .is_some_and(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit())
            || bytes.windows(2).any(|pair| pair == b"--")
            || !bytes
                .iter()
                .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || *byte == b'-')
        {
            return Err(invalid_exact_ref(
                "definition identity segment violates the lowercase ASCII grammar",
            ));
        }
    }
    Ok(())
}

fn validate_canonical_semver(version: &str) -> Result<Version, RegistryLoadError> {
    let parsed = Version::parse(version)
        .map_err(|_| invalid_exact_ref("definition version must be a full canonical SemVer"))?;
    if parsed.to_string() != version {
        return Err(invalid_exact_ref(
            "definition version must round-trip byte-identically through SemVer",
        ));
    }
    Ok(parsed)
}

fn invalid_exact_ref(detail: impl Into<String>) -> RegistryLoadError {
    RegistryLoadError::new(RegistryLoadErrorKind::InvalidExactDefinitionRef, detail)
}

fn classify_parse_error(error: impl fmt::Display) -> RegistryLoadError {
    let rendered = error.to_string();
    let (kind, detail) = if rendered.to_ascii_lowercase().contains("duplicate") {
        (
            RegistryLoadErrorKind::DuplicateKey,
            "source contains a duplicate mapping key",
        )
    } else {
        (
            RegistryLoadErrorKind::SyntaxError,
            "source document is syntactically invalid",
        )
    };
    RegistryLoadError::new(kind, detail)
}

struct NoDuplicateValue(Value);

impl<'de> Deserialize<'de> for NoDuplicateValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer
            .deserialize_any(NoDuplicateValueVisitor)
            .map(Self)
    }
}

struct NoDuplicateValueVisitor;

impl<'de> Visitor<'de> for NoDuplicateValueVisitor {
    type Value = Value;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a JSON-data-model value without duplicate mapping keys")
    }

    fn visit_bool<E>(self, value: bool) -> Result<Self::Value, E> {
        Ok(Value::Bool(value))
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E> {
        Ok(Value::Number(value.into()))
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E> {
        Ok(Value::Number(value.into()))
    }

    fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Number::from_f64(value)
            .map(Value::Number)
            .ok_or_else(|| E::custom("non-finite numbers are not part of the JSON data model"))
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E> {
        Ok(Value::String(value.to_owned()))
    }

    fn visit_string<E>(self, value: String) -> Result<Self::Value, E> {
        Ok(Value::String(value))
    }

    fn visit_none<E>(self) -> Result<Self::Value, E> {
        Ok(Value::Null)
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E> {
        Ok(Value::Null)
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        NoDuplicateValue::deserialize(deserializer).map(|value| value.0)
    }

    fn visit_seq<A>(self, mut sequence: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut values = Vec::with_capacity(sequence.size_hint().unwrap_or(0));
        while let Some(value) = sequence.next_element::<NoDuplicateValue>()? {
            values.push(value.0);
        }
        Ok(Value::Array(values))
    }

    fn visit_map<A>(self, mut mapping: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut values = Map::new();
        while let Some((key, value)) = mapping.next_entry::<String, NoDuplicateValue>()? {
            if values.insert(key.clone(), value.0).is_some() {
                return Err(serde::de::Error::custom("duplicate mapping key"));
            }
        }
        Ok(Value::Object(values))
    }
}
