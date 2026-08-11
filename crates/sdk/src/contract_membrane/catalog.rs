use super::serialization::classify_contract_error_message;
use super::{
    canonical_fingerprint, canonical_json_bytes, BoundedVec, ContractError, ContractLimits,
    ContractValue, ExactBinding, Fingerprint, SafeU64, UtcTimestamp,
};
use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::BTreeSet;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CatalogRoot {
    catalog: ExactBinding,
    entry_schema: ExactBinding,
    total_entries: SafeU64,
    canonical_sort: ExactBinding,
    retention_until_utc: UtcTimestamp,
}

impl CatalogRoot {
    pub fn new(
        catalog: ExactBinding,
        entry_schema: ExactBinding,
        total_entries: SafeU64,
        canonical_sort: ExactBinding,
        retention_until_utc: UtcTimestamp,
    ) -> Result<Self, ContractError> {
        Ok(Self {
            catalog,
            entry_schema,
            total_entries,
            canonical_sort,
            retention_until_utc,
        })
    }

    pub fn catalog(&self) -> &ExactBinding {
        &self.catalog
    }

    pub fn entry_schema(&self) -> &ExactBinding {
        &self.entry_schema
    }

    pub fn total_entries(&self) -> SafeU64 {
        self.total_entries
    }

    pub fn canonical_sort(&self) -> &ExactBinding {
        &self.canonical_sort
    }

    pub fn retention_until_utc(&self) -> &UtcTimestamp {
        &self.retention_until_utc
    }
}

impl<'de> Deserialize<'de> for CatalogRoot {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct RawCatalogRoot {
            catalog: ExactBinding,
            entry_schema: ExactBinding,
            total_entries: SafeU64,
            canonical_sort: ExactBinding,
            retention_until_utc: UtcTimestamp,
        }

        let raw = RawCatalogRoot::deserialize(deserializer).map_err(|error| {
            D::Error::custom(classify_contract_error_message(&error.to_string()))
        })?;
        Self::new(
            raw.catalog,
            raw.entry_schema,
            raw.total_entries,
            raw.canonical_sort,
            raw.retention_until_utc,
        )
        .map_err(D::Error::custom)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CatalogCursor {
    catalog: ExactBinding,
    next_sort_key: ContractValue,
    cursor_fingerprint: Fingerprint,
}

impl CatalogCursor {
    pub fn new(
        catalog: ExactBinding,
        next_sort_key: ContractValue,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        let cursor_fingerprint = cursor_fingerprint(&catalog, &next_sort_key, limits)?;
        let value = Self {
            catalog,
            next_sort_key,
            cursor_fingerprint,
        };
        canonical_json_bytes(&value, limits)?;
        Ok(value)
    }

    pub fn catalog(&self) -> &ExactBinding {
        &self.catalog
    }

    pub fn next_sort_key(&self) -> &ContractValue {
        &self.next_sort_key
    }

    pub fn cursor_fingerprint(&self) -> &Fingerprint {
        &self.cursor_fingerprint
    }
}

impl<'de> Deserialize<'de> for CatalogCursor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct RawCatalogCursor {
            catalog: ExactBinding,
            next_sort_key: ContractValue,
            cursor_fingerprint: Fingerprint,
        }

        let raw = RawCatalogCursor::deserialize(deserializer).map_err(|error| {
            D::Error::custom(classify_contract_error_message(&error.to_string()))
        })?;
        let value = Self::new(raw.catalog, raw.next_sort_key, &ContractLimits::packet0())
            .map_err(D::Error::custom)?;
        if raw.cursor_fingerprint != value.cursor_fingerprint {
            return Err(D::Error::custom(ContractError::FingerprintMismatch));
        }
        Ok(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CatalogPage {
    root: CatalogRoot,
    entries: BoundedVec<ContractValue, 128>,
    next_cursor: Option<CatalogCursor>,
    page_fingerprint: Fingerprint,
}

impl CatalogPage {
    pub fn new(
        root: CatalogRoot,
        entries: BoundedVec<ContractValue, 128>,
        next_cursor: Option<CatalogCursor>,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        validate_page_local(&root, entries.as_slice(), next_cursor.as_ref(), limits)?;
        let page_fingerprint = page_fingerprint(&root, &entries, next_cursor.as_ref(), limits)?;
        let value = Self {
            root,
            entries,
            next_cursor,
            page_fingerprint,
        };
        canonical_json_bytes(&value, limits)?;
        Ok(value)
    }

    pub fn root(&self) -> &CatalogRoot {
        &self.root
    }

    pub fn entries(&self) -> &BoundedVec<ContractValue, 128> {
        &self.entries
    }

    pub fn next_cursor(&self) -> Option<&CatalogCursor> {
        self.next_cursor.as_ref()
    }

    pub fn page_fingerprint(&self) -> &Fingerprint {
        &self.page_fingerprint
    }
}

impl<'de> Deserialize<'de> for CatalogPage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct RawCatalogPage {
            root: CatalogRoot,
            entries: BoundedVec<ContractValue, 128>,
            #[serde(default)]
            next_cursor: RequiredOption<CatalogCursor>,
            page_fingerprint: Fingerprint,
        }

        let raw = RawCatalogPage::deserialize(deserializer).map_err(|error| {
            D::Error::custom(classify_contract_error_message(&error.to_string()))
        })?;
        let next_cursor = raw.next_cursor.into_option().map_err(D::Error::custom)?;
        let value = Self::new(
            raw.root,
            raw.entries,
            next_cursor,
            &ContractLimits::packet0(),
        )
        .map_err(D::Error::custom)?;
        if raw.page_fingerprint != value.page_fingerprint {
            return Err(D::Error::custom(ContractError::FingerprintMismatch));
        }
        Ok(value)
    }
}

#[derive(Serialize)]
struct CursorPreimage<'a> {
    catalog: &'a ExactBinding,
    next_sort_key: &'a ContractValue,
}

#[derive(Serialize)]
struct PagePreimage<'a> {
    root: &'a CatalogRoot,
    entries: &'a BoundedVec<ContractValue, 128>,
    next_cursor: Option<&'a CatalogCursor>,
}

pub(super) fn cursor_fingerprint(
    catalog: &ExactBinding,
    next_sort_key: &ContractValue,
    limits: &ContractLimits,
) -> Result<Fingerprint, ContractError> {
    canonical_fingerprint(
        &CursorPreimage {
            catalog,
            next_sort_key,
        },
        limits,
    )
}

pub(super) fn page_fingerprint(
    root: &CatalogRoot,
    entries: &BoundedVec<ContractValue, 128>,
    next_cursor: Option<&CatalogCursor>,
    limits: &ContractLimits,
) -> Result<Fingerprint, ContractError> {
    canonical_fingerprint(
        &PagePreimage {
            root,
            entries,
            next_cursor,
        },
        limits,
    )
}

pub(super) fn validate_page_local(
    root: &CatalogRoot,
    entries: &[ContractValue],
    next_cursor: Option<&CatalogCursor>,
    limits: &ContractLimits,
) -> Result<(), ContractError> {
    if entries.len() as u64 > root.total_entries().get() {
        return Err(ContractError::InvalidInvariant);
    }
    if next_cursor.is_some_and(|cursor| cursor.catalog() != root.catalog()) {
        return Err(ContractError::InvalidInvariant);
    }

    let mut unique_entries = BTreeSet::new();
    for entry in entries {
        let bytes = canonical_json_bytes(entry, limits)?;
        if !unique_entries.insert(bytes) {
            return Err(ContractError::DuplicateMember);
        }
    }
    Ok(())
}

enum RequiredOption<T> {
    Missing,
    Present(Option<T>),
}

impl<T> RequiredOption<T> {
    fn into_option(self) -> Result<Option<T>, ContractError> {
        match self {
            Self::Missing => Err(ContractError::InvalidJson),
            Self::Present(value) => Ok(value),
        }
    }
}

impl<T> Default for RequiredOption<T> {
    fn default() -> Self {
        Self::Missing
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for RequiredOption<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Option::deserialize(deserializer).map(Self::Present)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contract_membrane::{parse_canonical_json, sha256_fingerprint, ExactRef};

    const SENTINEL: &str = "raw-key-sentinel";

    fn binding(identity: &str) -> ExactBinding {
        ExactBinding::new(
            ExactRef::parse(&format!("{identity}@1.0.0")).unwrap(),
            sha256_fingerprint(identity.as_bytes()),
        )
    }

    fn with_unknown_field<T: Serialize>(value: &T) -> String {
        let mut wire = serde_json::to_string(value).unwrap();
        wire.pop();
        wire.push_str(&format!(",\"{SENTINEL}\":null}}"));
        wire
    }

    fn assert_sanitized<T>(wire: &str)
    where
        T: for<'de> Deserialize<'de>,
    {
        let error = match serde_json::from_str::<T>(wire) {
            Ok(_) => panic!("rejected catalog input was accepted"),
            Err(error) => error.to_string(),
        };
        assert!(error.starts_with(&ContractError::InvalidJson.to_string()));
        assert!(!error.contains(SENTINEL));
    }

    fn replace_once(bytes: &[u8], old: &str, new: &str) -> Vec<u8> {
        let wire = String::from_utf8(bytes.to_vec()).unwrap();
        assert_eq!(wire.matches(old).count(), 1);
        wire.replacen(old, new, 1).into_bytes()
    }

    #[test]
    fn public_catalog_deserializers_redact_rejected_fields() {
        let limits = ContractLimits::packet0();
        let root = CatalogRoot::new(
            binding("catalog.root"),
            binding("entry.schema"),
            SafeU64::new(0).unwrap(),
            binding("sort.definition"),
            UtcTimestamp::parse("2030-01-01T00:00:00Z").unwrap(),
        )
        .unwrap();
        let cursor =
            CatalogCursor::new(root.catalog().clone(), ContractValue::null(), &limits).unwrap();
        let page = CatalogPage::new(
            root.clone(),
            BoundedVec::new(Vec::new()).unwrap(),
            None,
            &limits,
        )
        .unwrap();

        assert_sanitized::<CatalogRoot>(&with_unknown_field(&root));
        assert_sanitized::<CatalogCursor>(&with_unknown_field(&cursor));
        assert_sanitized::<CatalogPage>(&with_unknown_field(&page));
    }

    #[test]
    fn bounded_parser_preserves_nested_catalog_errors() {
        let limits = ContractLimits::packet0();
        let root = CatalogRoot::new(
            binding("catalog.root"),
            binding("entry.schema"),
            SafeU64::new(0).unwrap(),
            binding("sort.definition"),
            UtcTimestamp::parse("2030-01-01T00:00:00Z").unwrap(),
        )
        .unwrap();

        let invalid_timestamp = replace_once(
            &canonical_json_bytes(&root, &limits).unwrap(),
            "2030-01-01T00:00:00Z",
            "2030-02-30T00:00:00Z",
        );
        assert_eq!(
            parse_canonical_json::<CatalogRoot>(&invalid_timestamp, &limits),
            Err(ContractError::InvalidTimestamp)
        );

        let cursor =
            CatalogCursor::new(root.catalog().clone(), ContractValue::null(), &limits).unwrap();
        let invalid_exact_ref = replace_once(
            &canonical_json_bytes(&cursor, &limits).unwrap(),
            "catalog.root@1.0.0",
            "catalog.root@latest",
        );
        assert_eq!(
            parse_canonical_json::<CatalogCursor>(&invalid_exact_ref, &limits),
            Err(ContractError::InvalidExactRef)
        );

        let page =
            CatalogPage::new(root, BoundedVec::new(Vec::new()).unwrap(), None, &limits).unwrap();
        let oversized_entries = std::iter::repeat_n("null", 129)
            .collect::<Vec<_>>()
            .join(",");
        let bound_exceeded = replace_once(
            &canonical_json_bytes(&page, &limits).unwrap(),
            "\"entries\":[]",
            &format!("\"entries\":[{oversized_entries}]"),
        );
        assert_eq!(
            parse_canonical_json::<CatalogPage>(&bound_exceeded, &limits),
            Err(ContractError::BoundExceeded)
        );
    }
}
