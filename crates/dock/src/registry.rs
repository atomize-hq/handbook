use crate::canonical::canonical_fingerprint;
use crate::{DockErrorV1, DockManifestV1};
use handbook_contracts::{ExactRefV1, FingerprintV1};
use serde::Serialize;

const MAX_MANIFEST_PAGE_SIZE: usize = 100;
const MAX_CURSOR_BYTES: usize = 4_096;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DockManifestListRequestV1 {
    limit: usize,
    root: Option<FingerprintV1>,
    cursor: Option<String>,
}

impl DockManifestListRequestV1 {
    pub fn first(limit: usize) -> Result<Self, DockErrorV1> {
        if !(1..=MAX_MANIFEST_PAGE_SIZE).contains(&limit) {
            return Err(DockErrorV1::BoundExceeded);
        }
        Ok(Self {
            limit,
            root: None,
            cursor: None,
        })
    }

    pub fn continue_from(
        limit: usize,
        root: FingerprintV1,
        cursor: impl Into<String>,
    ) -> Result<Self, DockErrorV1> {
        let cursor = cursor.into();
        if !(1..=MAX_MANIFEST_PAGE_SIZE).contains(&limit)
            || cursor.is_empty()
            || cursor.len() > MAX_CURSOR_BYTES
        {
            return Err(DockErrorV1::BoundExceeded);
        }
        Ok(Self {
            limit,
            root: Some(root),
            cursor: Some(cursor),
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DockManifestRetentionV1 {
    ImmutableExact,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct DockManifestListEntryV1 {
    manifest_ref: ExactRefV1,
    manifest_fingerprint: FingerprintV1,
}

impl DockManifestListEntryV1 {
    pub fn manifest_ref(&self) -> &ExactRefV1 {
        &self.manifest_ref
    }
    pub fn manifest_fingerprint(&self) -> &FingerprintV1 {
        &self.manifest_fingerprint
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DockManifestListPageV1 {
    root: FingerprintV1,
    entries: Vec<DockManifestListEntryV1>,
    next_cursor: Option<String>,
}

impl DockManifestListPageV1 {
    pub fn root(&self) -> &FingerprintV1 {
        &self.root
    }
    pub fn entries(&self) -> &[DockManifestListEntryV1] {
        &self.entries
    }
    pub fn next_cursor(&self) -> Option<&str> {
        self.next_cursor.as_deref()
    }
    pub fn retention(&self) -> DockManifestRetentionV1 {
        DockManifestRetentionV1::ImmutableExact
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DockManifestReadOutcomeV1 {
    Found(Box<DockManifestV1>),
    Absent,
}

/// Immutable owner registry closed over the host-embedded production truth.
#[derive(Clone, Debug)]
pub struct DockManifestRegistryV1 {
    manifests: Vec<DockManifestV1>,
}

impl Default for DockManifestRegistryV1 {
    fn default() -> Self {
        Self::production()
    }
}

impl DockManifestRegistryV1 {
    pub fn production() -> Self {
        Self {
            manifests: vec![production_manifest_v1()],
        }
    }
    pub fn production_len(&self) -> usize {
        self.manifests.len()
    }

    pub fn list(
        &self,
        request: DockManifestListRequestV1,
    ) -> Result<DockManifestListPageV1, DockErrorV1> {
        let entries: Vec<_> = self
            .manifests
            .iter()
            .map(|manifest| DockManifestListEntryV1 {
                manifest_ref: manifest.dock_ref().clone(),
                manifest_fingerprint: manifest.manifest_fingerprint().clone(),
            })
            .collect();
        let root = canonical_fingerprint(&ManifestRootPreimage {
            schema_ref: "handbook.dock-manifest-list-root@1.0.0",
            retention: "immutable_exact",
            manifests: &entries,
        })?;
        let offset = match (&request.root, &request.cursor) {
            (None, None) => 0,
            (Some(expected), Some(cursor)) => {
                if expected != &root {
                    return Err(DockErrorV1::Substituted);
                }
                decode_cursor(cursor, expected, request.limit)?
            }
            _ => return Err(DockErrorV1::InvalidInvariant),
        };
        if offset > entries.len() {
            return Err(DockErrorV1::Substituted);
        }
        let end = offset.saturating_add(request.limit).min(entries.len());
        let next_cursor = (end < entries.len()).then(|| encode_cursor(&root, end, request.limit));
        Ok(DockManifestListPageV1 {
            root,
            entries: entries[offset..end].to_vec(),
            next_cursor,
        })
    }

    pub fn read(
        &self,
        manifest_ref: &ExactRefV1,
        fingerprint: &FingerprintV1,
    ) -> Result<DockManifestReadOutcomeV1, DockErrorV1> {
        if let Some(manifest) = self.manifests.iter().find(|manifest| {
            manifest.dock_ref() == manifest_ref && manifest.manifest_fingerprint() == fingerprint
        }) {
            return Ok(DockManifestReadOutcomeV1::Found(Box::new(manifest.clone())));
        }
        if self
            .manifests
            .iter()
            .any(|manifest| manifest.dock_ref() == manifest_ref)
        {
            return Err(DockErrorV1::Substituted);
        }
        Ok(DockManifestReadOutcomeV1::Absent)
    }
}

/// Exact canonical manifest bytes frozen into this target's host build.
pub fn production_manifest_bytes_v1() -> &'static [u8] {
    include_bytes!(env!("HANDBOOK_DOCK_PRODUCTION_MANIFEST"))
        .strip_suffix(b"\n")
        .expect("the checked-in production manifest must end with one text newline")
}

pub(crate) fn production_manifest_v1() -> DockManifestV1 {
    DockManifestV1::admit_canonical_json(production_manifest_bytes_v1())
        .expect("the build-selected production manifest must be exact and canonical")
}

#[derive(Serialize)]
struct ManifestRootPreimage<'a> {
    schema_ref: &'static str,
    retention: &'static str,
    manifests: &'a [DockManifestListEntryV1],
}

fn encode_cursor(root: &FingerprintV1, offset: usize, limit: usize) -> String {
    format!("{}:{offset}:{limit}", root.as_str())
}

fn decode_cursor(cursor: &str, root: &FingerprintV1, limit: usize) -> Result<usize, DockErrorV1> {
    if cursor.len() > MAX_CURSOR_BYTES {
        return Err(DockErrorV1::BoundExceeded);
    }
    let prefix = format!("{}:", root.as_str());
    let remainder = cursor
        .strip_prefix(&prefix)
        .ok_or(DockErrorV1::Substituted)?;
    let (offset, cursor_limit) = remainder.split_once(':').ok_or(DockErrorV1::Substituted)?;
    let offset = offset
        .parse::<usize>()
        .map_err(|_| DockErrorV1::Substituted)?;
    let cursor_limit = cursor_limit
        .parse::<usize>()
        .map_err(|_| DockErrorV1::Substituted)?;
    if cursor_limit != limit {
        return Err(DockErrorV1::Substituted);
    }
    Ok(offset)
}
