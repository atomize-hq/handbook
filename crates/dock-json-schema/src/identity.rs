use crate::canonical::canonical_fingerprint;
use crate::{ValidatorLimitsV1, ValidatorRefusalV1};
use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

/// A canonical lowercase `sha256:<64-hex>` RFC-8785 value fingerprint.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CanonicalFingerprintV1(String);

impl CanonicalFingerprintV1 {
    pub fn parse(value: &str) -> Result<Self, ValidatorRefusalV1> {
        let bytes = value.as_bytes();
        if bytes.len() != 71
            || !value.starts_with("sha256:")
            || !bytes[7..]
                .iter()
                .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
        {
            return Err(ValidatorRefusalV1::InputMalformed);
        }
        Ok(Self(value.to_owned()))
    }

    pub fn for_value<T: Serialize>(value: &T) -> Result<Self, ValidatorRefusalV1> {
        canonical_fingerprint(value)
    }

    pub fn verify_value<T: Serialize>(&self, value: &T) -> Result<(), ValidatorRefusalV1> {
        if &Self::for_value(value)? != self {
            return Err(ValidatorRefusalV1::InputSubstituted);
        }
        Ok(())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub(crate) fn from_digest(digest: [u8; 32]) -> Self {
        const HEX: &[u8; 16] = b"0123456789abcdef";
        let mut value = String::with_capacity(71);
        value.push_str("sha256:");
        for byte in digest {
            value.push(HEX[(byte >> 4) as usize] as char);
            value.push(HEX[(byte & 0x0f) as usize] as char);
        }
        Self(value)
    }
}

impl fmt::Display for CanonicalFingerprintV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl Serialize for CanonicalFingerprintV1 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for CanonicalFingerprintV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Self::parse(&String::deserialize(deserializer)?).map_err(D::Error::custom)
    }
}

/// A closed logical schema reference and its canonical root-document binding.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExactSchemaBindingV1 {
    #[serde(rename = "ref")]
    reference: String,
    fingerprint: CanonicalFingerprintV1,
}

impl<'de> Deserialize<'de> for ExactSchemaBindingV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Raw {
            #[serde(rename = "ref")]
            reference: String,
            fingerprint: CanonicalFingerprintV1,
        }

        let raw = Raw::deserialize(deserializer)?;
        Self::parse(&raw.reference, raw.fingerprint).map_err(D::Error::custom)
    }
}

impl ExactSchemaBindingV1 {
    pub fn parse(
        reference: &str,
        fingerprint: CanonicalFingerprintV1,
    ) -> Result<Self, ValidatorRefusalV1> {
        if !valid_exact_ref(reference) {
            return Err(ValidatorRefusalV1::InputMalformed);
        }
        Ok(Self {
            reference: reference.to_owned(),
            fingerprint,
        })
    }

    pub fn reference(&self) -> &str {
        &self.reference
    }
    pub fn fingerprint(&self) -> &CanonicalFingerprintV1 {
        &self.fingerprint
    }
}

/// A normalized safe-relative logical path under `definitions/schemas/`.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct LogicalSchemaPathV1(String);

impl LogicalSchemaPathV1 {
    pub fn parse(value: &str) -> Result<Self, ValidatorRefusalV1> {
        let limits = ValidatorLimitsV1::v1();
        if value.len() > limits.max_path_bytes() {
            return Err(ValidatorRefusalV1::LimitExceeded);
        }
        let components = value.split('/').collect::<Vec<_>>();
        if components.len() > limits.max_path_components() {
            return Err(ValidatorRefusalV1::LimitExceeded);
        }
        let valid = !value.is_empty()
            && value.trim() == value
            && value.starts_with("definitions/schemas/")
            && !value.ends_with('/')
            && !value.contains('\\')
            && !value.contains('\0')
            && !value.contains(':')
            && components.len() >= 3
            && components.iter().all(|component| {
                !component.is_empty()
                    && *component != "."
                    && *component != ".."
                    && component.trim() == *component
                    && !component.chars().any(|character| {
                        character.is_control()
                            || matches!(character, '<' | '>' | '"' | '|' | '?' | '*')
                    })
            });
        if !valid {
            return Err(ValidatorRefusalV1::InputMalformed);
        }
        Ok(Self(value.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for LogicalSchemaPathV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl Serialize for LogicalSchemaPathV1 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> Deserialize<'de> for LogicalSchemaPathV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Self::parse(&String::deserialize(deserializer)?).map_err(D::Error::custom)
    }
}

fn valid_exact_ref(value: &str) -> bool {
    if !(7..=511).contains(&value.len()) || !value.is_ascii() {
        return false;
    }
    let Some((identity, version)) = value.rsplit_once('@') else {
        return false;
    };
    if identity.is_empty()
        || identity.len() > 255
        || !identity.as_bytes()[0].is_ascii_lowercase()
        || !identity.bytes().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || b"._-".contains(&byte)
        })
    {
        return false;
    }
    valid_semver(version)
}

fn valid_semver(value: &str) -> bool {
    let (without_build, build) = value
        .split_once('+')
        .map_or((value, None), |(left, right)| (left, Some(right)));
    if build.is_some_and(|part| !valid_identifiers(part, false)) {
        return false;
    }
    let (core, prerelease) = without_build
        .split_once('-')
        .map_or((without_build, None), |(left, right)| (left, Some(right)));
    if prerelease.is_some_and(|part| !valid_identifiers(part, true)) {
        return false;
    }
    let mut parts = core.split('.');
    let valid_core = (0..3).all(|_| parts.next().is_some_and(valid_numeric_identifier));
    valid_core && parts.next().is_none()
}

fn valid_identifiers(value: &str, reject_numeric_leading_zero: bool) -> bool {
    !value.is_empty()
        && value.split('.').all(|part| {
            !part.is_empty()
                && part
                    .bytes()
                    .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
                && (!reject_numeric_leading_zero
                    || !part.bytes().all(|byte| byte.is_ascii_digit())
                    || valid_numeric_identifier(part))
        })
}

fn valid_numeric_identifier(value: &str) -> bool {
    !value.is_empty()
        && value.bytes().all(|byte| byte.is_ascii_digit())
        && (value == "0" || !value.starts_with('0'))
}
