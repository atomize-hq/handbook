use crate::DockErrorV1;
use handbook_contracts::FingerprintV1;
use serde::de::{MapAccess, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::fmt;

pub(crate) fn canonical_bytes<T: Serialize>(value: &T) -> Result<Vec<u8>, DockErrorV1> {
    serde_json_canonicalizer::to_vec(value).map_err(|_| DockErrorV1::Canonicalization)
}

pub(crate) fn canonical_fingerprint<T: Serialize>(value: &T) -> Result<FingerprintV1, DockErrorV1> {
    let bytes = canonical_bytes(value)?;
    let digest = Sha256::digest(bytes);
    FingerprintV1::parse(&format!("sha256:{digest:x}")).map_err(|_| DockErrorV1::Canonicalization)
}

pub(crate) fn fingerprint_hex(fingerprint: &FingerprintV1) -> &str {
    fingerprint.as_str().strip_prefix("sha256:").unwrap_or("")
}

pub(crate) fn admit_canonical_json<T>(bytes: &[u8], max_bytes: usize) -> Result<T, DockErrorV1>
where
    T: for<'de> Deserialize<'de> + Serialize,
{
    if bytes.is_empty() || bytes.len() > max_bytes {
        return Err(DockErrorV1::BoundExceeded);
    }
    let _: DuplicateCheckedValue =
        serde_json::from_slice(bytes).map_err(|_| DockErrorV1::Canonicalization)?;
    let value: T = serde_json::from_slice(bytes).map_err(|_| DockErrorV1::InvalidInvariant)?;
    if canonical_bytes(&value)? != bytes {
        return Err(DockErrorV1::Canonicalization);
    }
    Ok(value)
}

pub(crate) fn admit_framed_json<T>(bytes: &[u8], max_bytes: usize) -> Result<T, DockErrorV1>
where
    T: for<'de> Deserialize<'de> + Serialize,
{
    if bytes.len() < 2 || bytes.last() != Some(&b'\n') || bytes[..bytes.len() - 1].contains(&b'\n')
    {
        return Err(DockErrorV1::Canonicalization);
    }
    admit_canonical_json(&bytes[..bytes.len() - 1], max_bytes)
}

pub(crate) fn framed_json<T: Serialize>(
    value: &T,
    max_bytes: usize,
) -> Result<Vec<u8>, DockErrorV1> {
    let mut bytes = canonical_bytes(value)?;
    if bytes.is_empty() || bytes.len() > max_bytes {
        return Err(DockErrorV1::BoundExceeded);
    }
    bytes.push(b'\n');
    Ok(bytes)
}

struct DuplicateCheckedValue;

impl<'de> Deserialize<'de> for DuplicateCheckedValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CheckedVisitor;
        impl<'de> Visitor<'de> for CheckedVisitor {
            type Value = DuplicateCheckedValue;
            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("JSON value without duplicate object members")
            }
            fn visit_bool<E>(self, _: bool) -> Result<Self::Value, E> {
                Ok(DuplicateCheckedValue)
            }
            fn visit_i64<E>(self, _: i64) -> Result<Self::Value, E> {
                Ok(DuplicateCheckedValue)
            }
            fn visit_u64<E>(self, _: u64) -> Result<Self::Value, E> {
                Ok(DuplicateCheckedValue)
            }
            fn visit_f64<E>(self, _: f64) -> Result<Self::Value, E> {
                Ok(DuplicateCheckedValue)
            }
            fn visit_str<E>(self, _: &str) -> Result<Self::Value, E> {
                Ok(DuplicateCheckedValue)
            }
            fn visit_string<E>(self, _: String) -> Result<Self::Value, E> {
                Ok(DuplicateCheckedValue)
            }
            fn visit_none<E>(self) -> Result<Self::Value, E> {
                Ok(DuplicateCheckedValue)
            }
            fn visit_unit<E>(self) -> Result<Self::Value, E> {
                Ok(DuplicateCheckedValue)
            }
            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                while seq.next_element::<DuplicateCheckedValue>()?.is_some() {}
                Ok(DuplicateCheckedValue)
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut keys = BTreeSet::new();
                while let Some(key) = map.next_key::<String>()? {
                    if !keys.insert(key) {
                        return Err(serde::de::Error::custom("duplicate JSON member"));
                    }
                    map.next_value::<DuplicateCheckedValue>()?;
                }
                Ok(DuplicateCheckedValue)
            }
        }
        deserializer.deserialize_any(CheckedVisitor)
    }
}
