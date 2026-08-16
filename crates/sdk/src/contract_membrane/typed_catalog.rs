use super::serialization::classify_contract_error_message;
use super::{
    canonical_fingerprint, canonical_json_bytes, sha256_fingerprint, BoundedVec, CatalogRoot,
    ContractError, ContractLimits, ExactBinding, ExactRef, Fingerprint, SafeU64,
    SchemaManifestEntry, UtcTimestamp,
};
use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize};
use std::cmp::Ordering;

const API_MAJOR_V1: u64 = 1;
const PAGE_SIZE_MAX: u64 = 128;
const CATALOG_TOTAL_MAX: usize = 128;
const CATALOG_ENTRY_MAX_BYTES: usize = 65_536;
const CURSOR_SCHEMA_REF: &str = "handbook.typed-catalog-cursor@1.0.0";
const SORT_REF: &str = "handbook.catalog-sort.exact-ref-byte-order@1.0.0";
const SORT_PREIMAGE: &[u8] = b"handbook.catalog-sort.exact-ref-byte-order-v1";

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub struct CatalogPageSize(u8);

impl CatalogPageSize {
    pub fn new(value: u64) -> Result<Self, ContractError> {
        if value == 0 {
            return Err(ContractError::Empty);
        }
        if value > PAGE_SIZE_MAX {
            return Err(ContractError::BoundExceeded);
        }
        Ok(Self(value as u8))
    }

    pub fn get(self) -> u64 {
        u64::from(self.0)
    }
}

impl<'de> Deserialize<'de> for CatalogPageSize {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = SafeU64::deserialize(deserializer).map_err(|error| {
            D::Error::custom(classify_contract_error_message(&error.to_string()))
        })?;
        Self::new(value.get()).map_err(D::Error::custom)
    }
}

pub trait TypedCatalogEntry {
    const CATALOG_REF: &'static str;
    const ENTRY_SCHEMA_REF: &'static str;
    const PAGE_SCHEMA_REF: &'static str;

    fn catalog_sort_key(&self) -> &ExactRef;
}

impl TypedCatalogEntry for SchemaManifestEntry {
    const CATALOG_REF: &'static str = "handbook.catalog.schemas@1.0.0";
    const ENTRY_SCHEMA_REF: &'static str = "handbook.schema-manifest-entry@1.0.0";
    const PAGE_SCHEMA_REF: &'static str = "handbook.schema-catalog-page@1.0.0";

    fn catalog_sort_key(&self) -> &ExactRef {
        self.schema().reference()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct TypedCatalogCursor {
    schema: ExactBinding,
    api_major: SafeU64,
    catalog: ExactBinding,
    page_size: CatalogPageSize,
    next_index: SafeU64,
    next_sort_key: ExactRef,
    expires_at_utc: UtcTimestamp,
    cursor_fingerprint: Fingerprint,
}

impl TypedCatalogCursor {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        schema: ExactBinding,
        api_major: SafeU64,
        catalog: ExactBinding,
        page_size: CatalogPageSize,
        next_index: SafeU64,
        next_sort_key: ExactRef,
        expires_at_utc: UtcTimestamp,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        require_binding_ref(&schema, CURSOR_SCHEMA_REF)?;
        if api_major.get() != API_MAJOR_V1 || next_index.get() == 0 {
            return Err(ContractError::InvalidInvariant);
        }
        let cursor_fingerprint = canonical_fingerprint(
            &CursorPreimage {
                schema: &schema,
                api_major,
                catalog: &catalog,
                page_size,
                next_index,
                next_sort_key: &next_sort_key,
                expires_at_utc: &expires_at_utc,
            },
            limits,
        )?;
        let value = Self {
            schema,
            api_major,
            catalog,
            page_size,
            next_index,
            next_sort_key,
            expires_at_utc,
            cursor_fingerprint,
        };
        canonical_json_bytes(&value, limits)?;
        Ok(value)
    }

    pub fn schema(&self) -> &ExactBinding {
        &self.schema
    }

    pub fn api_major(&self) -> SafeU64 {
        self.api_major
    }

    pub fn catalog(&self) -> &ExactBinding {
        &self.catalog
    }

    pub fn page_size(&self) -> CatalogPageSize {
        self.page_size
    }

    pub fn next_index(&self) -> SafeU64 {
        self.next_index
    }

    pub fn next_sort_key(&self) -> &ExactRef {
        &self.next_sort_key
    }

    pub fn expires_at_utc(&self) -> &UtcTimestamp {
        &self.expires_at_utc
    }

    pub fn cursor_fingerprint(&self) -> &Fingerprint {
        &self.cursor_fingerprint
    }
}

impl<'de> Deserialize<'de> for TypedCatalogCursor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Raw {
            schema: ExactBinding,
            api_major: SafeU64,
            catalog: ExactBinding,
            page_size: CatalogPageSize,
            next_index: SafeU64,
            next_sort_key: ExactRef,
            expires_at_utc: UtcTimestamp,
            cursor_fingerprint: Fingerprint,
        }

        let raw = Raw::deserialize(deserializer).map_err(|error| {
            D::Error::custom(classify_contract_error_message(&error.to_string()))
        })?;
        let value = Self::new(
            raw.schema,
            raw.api_major,
            raw.catalog,
            raw.page_size,
            raw.next_index,
            raw.next_sort_key,
            raw.expires_at_utc,
            &ContractLimits::packet0(),
        )
        .map_err(D::Error::custom)?;
        if raw.cursor_fingerprint != value.cursor_fingerprint {
            return Err(D::Error::custom(ContractError::FingerprintMismatch));
        }
        Ok(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct TypedCatalogPage<T> {
    schema: ExactBinding,
    api_major: SafeU64,
    root: CatalogRoot,
    page_size: CatalogPageSize,
    start_index: SafeU64,
    entries: BoundedVec<T, 128>,
    next_cursor: Option<TypedCatalogCursor>,
    page_fingerprint: Fingerprint,
}

impl<T> TypedCatalogPage<T>
where
    T: TypedCatalogEntry + Serialize,
{
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        schema: ExactBinding,
        api_major: SafeU64,
        root: CatalogRoot,
        page_size: CatalogPageSize,
        start_index: SafeU64,
        entries: BoundedVec<T, 128>,
        next_cursor: Option<TypedCatalogCursor>,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        require_binding_ref(&schema, T::PAGE_SCHEMA_REF)?;
        require_binding_ref(root.catalog(), T::CATALOG_REF)?;
        require_binding_ref(root.entry_schema(), T::ENTRY_SCHEMA_REF)?;
        if api_major.get() != API_MAJOR_V1 || entries.as_slice().len() > page_size.get() as usize {
            return Err(ContractError::InvalidInvariant);
        }
        validate_strict_entry_order(entries.as_slice())?;
        validate_page_span(
            &root,
            page_size,
            start_index,
            entries.as_slice(),
            next_cursor.as_ref(),
        )?;
        let page_fingerprint = canonical_fingerprint(
            &PagePreimage {
                schema: &schema,
                api_major,
                root: &root,
                page_size,
                start_index,
                entries: &entries,
                next_cursor: next_cursor.as_ref(),
            },
            limits,
        )?;
        let value = Self {
            schema,
            api_major,
            root,
            page_size,
            start_index,
            entries,
            next_cursor,
            page_fingerprint,
        };
        canonical_json_bytes(&value, limits)?;
        Ok(value)
    }

    pub fn schema(&self) -> &ExactBinding {
        &self.schema
    }

    pub fn api_major(&self) -> SafeU64 {
        self.api_major
    }

    pub fn root(&self) -> &CatalogRoot {
        &self.root
    }

    pub fn page_size(&self) -> CatalogPageSize {
        self.page_size
    }

    pub fn start_index(&self) -> SafeU64 {
        self.start_index
    }

    pub fn entries(&self) -> &[T] {
        self.entries.as_slice()
    }

    pub fn next_cursor(&self) -> Option<&TypedCatalogCursor> {
        self.next_cursor.as_ref()
    }

    pub fn page_fingerprint(&self) -> &Fingerprint {
        &self.page_fingerprint
    }
}

impl<'de, T> Deserialize<'de> for TypedCatalogPage<T>
where
    T: TypedCatalogEntry + Serialize + Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields, bound(deserialize = "T: Deserialize<'de>"))]
        struct Raw<T> {
            schema: ExactBinding,
            api_major: SafeU64,
            root: CatalogRoot,
            page_size: CatalogPageSize,
            start_index: SafeU64,
            entries: BoundedVec<T, 128>,
            #[serde(default)]
            next_cursor: RequiredOption<TypedCatalogCursor>,
            page_fingerprint: Fingerprint,
        }

        let raw = Raw::<T>::deserialize(deserializer).map_err(|error| {
            D::Error::custom(classify_contract_error_message(&error.to_string()))
        })?;
        let value = Self::new(
            raw.schema,
            raw.api_major,
            raw.root,
            raw.page_size,
            raw.start_index,
            raw.entries,
            raw.next_cursor.into_option().map_err(D::Error::custom)?,
            &ContractLimits::packet0(),
        )
        .map_err(D::Error::custom)?;
        if raw.page_fingerprint != value.page_fingerprint {
            return Err(D::Error::custom(ContractError::FingerprintMismatch));
        }
        Ok(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CatalogReadError {
    CursorMismatch,
    Expired { root: Box<CatalogRoot> },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImmutableCatalog<T> {
    page_schema: ExactBinding,
    cursor_schema: ExactBinding,
    root: CatalogRoot,
    entries: Vec<T>,
}

impl<T> ImmutableCatalog<T>
where
    T: Clone + Eq + TypedCatalogEntry + Serialize,
{
    pub fn new(
        page_schema: ExactBinding,
        cursor_schema: ExactBinding,
        entry_schema: ExactBinding,
        mut entries: Vec<T>,
        retention_until_utc: UtcTimestamp,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        require_binding_ref(&page_schema, T::PAGE_SCHEMA_REF)?;
        require_binding_ref(&cursor_schema, CURSOR_SCHEMA_REF)?;
        require_binding_ref(&entry_schema, T::ENTRY_SCHEMA_REF)?;
        if entries.len() > CATALOG_TOTAL_MAX {
            return Err(ContractError::BoundExceeded);
        }
        entries.sort_by(|left, right| {
            left.catalog_sort_key()
                .as_str()
                .cmp(right.catalog_sort_key().as_str())
        });
        validate_strict_entry_order(&entries)?;
        for entry in &entries {
            if canonical_json_bytes(entry, limits)?.len() > CATALOG_ENTRY_MAX_BYTES {
                return Err(ContractError::BoundExceeded);
            }
        }

        let catalog_ref = ExactRef::parse(T::CATALOG_REF)?;
        let canonical_sort = ExactBinding::new(
            ExactRef::parse(SORT_REF)?,
            sha256_fingerprint(SORT_PREIMAGE),
        );
        let catalog_fingerprint = canonical_fingerprint(
            &CatalogPreimage {
                catalog_ref: &catalog_ref,
                entry_schema: &entry_schema,
                canonical_sort: &canonical_sort,
                retention_until_utc: &retention_until_utc,
                entries: &entries,
            },
            limits,
        )?;
        let root = CatalogRoot::new(
            ExactBinding::new(catalog_ref, catalog_fingerprint),
            entry_schema,
            SafeU64::new(entries.len() as u64)?,
            canonical_sort,
            retention_until_utc,
        )?;
        canonical_json_bytes(&root, limits)?;
        Ok(Self {
            page_schema,
            cursor_schema,
            root,
            entries,
        })
    }

    pub fn root(&self) -> &CatalogRoot {
        &self.root
    }

    pub fn entries(&self) -> &[T] {
        &self.entries
    }

    pub fn page(
        &self,
        page_size: CatalogPageSize,
        cursor: Option<&TypedCatalogCursor>,
        now_utc: &UtcTimestamp,
        limits: &ContractLimits,
    ) -> Result<TypedCatalogPage<T>, CatalogReadError> {
        if now_utc.as_str() > self.root.retention_until_utc().as_str() {
            return Err(CatalogReadError::Expired {
                root: Box::new(self.root.clone()),
            });
        }

        let start = match cursor {
            None => 0,
            Some(cursor) => {
                let index = cursor.next_index().get() as usize;
                if cursor.schema() != &self.cursor_schema
                    || cursor.api_major().get() != API_MAJOR_V1
                    || cursor.catalog() != self.root.catalog()
                    || cursor.page_size() != page_size
                    || cursor.expires_at_utc() != self.root.retention_until_utc()
                    || index >= self.entries.len()
                    || self.entries[index].catalog_sort_key() != cursor.next_sort_key()
                {
                    return Err(CatalogReadError::CursorMismatch);
                }
                index
            }
        };
        let end = (start + page_size.get() as usize).min(self.entries.len());
        let entries = self.entries[start..end].to_vec();
        let next_cursor = if end < self.entries.len() {
            Some(
                TypedCatalogCursor::new(
                    self.cursor_schema.clone(),
                    SafeU64::new(API_MAJOR_V1).map_err(|_| CatalogReadError::CursorMismatch)?,
                    self.root.catalog().clone(),
                    page_size,
                    SafeU64::new(end as u64).map_err(|_| CatalogReadError::CursorMismatch)?,
                    self.entries[end].catalog_sort_key().clone(),
                    self.root.retention_until_utc().clone(),
                    limits,
                )
                .map_err(|_| CatalogReadError::CursorMismatch)?,
            )
        } else {
            None
        };
        TypedCatalogPage::new(
            self.page_schema.clone(),
            SafeU64::new(API_MAJOR_V1).map_err(|_| CatalogReadError::CursorMismatch)?,
            self.root.clone(),
            page_size,
            SafeU64::new(start as u64).map_err(|_| CatalogReadError::CursorMismatch)?,
            BoundedVec::new(entries).map_err(|_| CatalogReadError::CursorMismatch)?,
            next_cursor,
            limits,
        )
        .map_err(|_| CatalogReadError::CursorMismatch)
    }
}

#[derive(Serialize)]
struct CursorPreimage<'a> {
    schema: &'a ExactBinding,
    api_major: SafeU64,
    catalog: &'a ExactBinding,
    page_size: CatalogPageSize,
    next_index: SafeU64,
    next_sort_key: &'a ExactRef,
    expires_at_utc: &'a UtcTimestamp,
}

#[derive(Serialize)]
struct PagePreimage<'a, T> {
    schema: &'a ExactBinding,
    api_major: SafeU64,
    root: &'a CatalogRoot,
    page_size: CatalogPageSize,
    start_index: SafeU64,
    entries: &'a BoundedVec<T, 128>,
    next_cursor: Option<&'a TypedCatalogCursor>,
}

#[derive(Serialize)]
struct CatalogPreimage<'a, T> {
    catalog_ref: &'a ExactRef,
    entry_schema: &'a ExactBinding,
    canonical_sort: &'a ExactBinding,
    retention_until_utc: &'a UtcTimestamp,
    entries: &'a [T],
}

fn validate_strict_entry_order<T: TypedCatalogEntry>(entries: &[T]) -> Result<(), ContractError> {
    for pair in entries.windows(2) {
        match pair[0]
            .catalog_sort_key()
            .as_str()
            .cmp(pair[1].catalog_sort_key().as_str())
        {
            Ordering::Less => {}
            Ordering::Equal => return Err(ContractError::DuplicateMember),
            Ordering::Greater => return Err(ContractError::NonCanonicalOrder),
        }
    }
    Ok(())
}

fn validate_page_span<T: TypedCatalogEntry>(
    root: &CatalogRoot,
    page_size: CatalogPageSize,
    start_index: SafeU64,
    entries: &[T],
    next_cursor: Option<&TypedCatalogCursor>,
) -> Result<(), ContractError> {
    let start = start_index.get();
    let end = start
        .checked_add(entries.len() as u64)
        .ok_or(ContractError::InvalidInvariant)?;
    if end > root.total_entries().get() {
        return Err(ContractError::InvalidInvariant);
    }
    if root.total_entries().get() > 0 && entries.is_empty() {
        return Err(ContractError::InvalidInvariant);
    }
    match next_cursor {
        Some(cursor) => {
            if end >= root.total_entries().get()
                || cursor.catalog() != root.catalog()
                || cursor.api_major().get() != API_MAJOR_V1
                || cursor.page_size() != page_size
                || cursor.next_index().get() != end
                || cursor.expires_at_utc() != root.retention_until_utc()
                || entries.last().is_some_and(|last| {
                    last.catalog_sort_key().as_str() >= cursor.next_sort_key().as_str()
                })
            {
                return Err(ContractError::InvalidInvariant);
            }
        }
        None if end != root.total_entries().get() => return Err(ContractError::InvalidInvariant),
        None => {}
    }
    Ok(())
}

fn require_binding_ref(binding: &ExactBinding, expected: &str) -> Result<(), ContractError> {
    if binding.reference().as_str() == expected {
        Ok(())
    } else {
        Err(ContractError::InvalidInvariant)
    }
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
