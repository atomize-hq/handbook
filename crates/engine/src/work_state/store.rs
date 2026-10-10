use super::*;
use crate::repository_durability::RepositoryDirectoryEntryKindV1 as Kind;
use crate::{RepositoryExclusiveLockV1, RepositoryReadV1, RetainedRepositoryRootV1};
use std::cell::Cell;
use std::collections::BTreeMap;

pub(super) const LOCK: &str = ".handbook/work-state/work.lock";
const ANCHOR: &str = ".handbook-work-state.v1.json";
const BOOT: &str = ".handbook-work-state-bootstrap.v1.json";
const STAGE: &str = ".handbook/work-state-bootstrap";
const OWNER: &str = ".handbook/work-state/v1/owner.json";
pub(super) const CAPACITY: usize = 32;
const INVENTORY_BYTES: usize = 128 * 1024 * 1024;
const SLOT_SCHEMA: &str = "handbook.work-state-reservation-slot@1.0.0";
const ROOT_INVENTORY_ENTRIES: usize = 4 * CAPACITY + 1;

/// Historical grant origins retain their exact raw representation. Only the new
/// kind origin has a closed, purpose-specific discriminant and owner schema.
#[derive(Clone, Debug, PartialEq)]
pub(super) enum BootstrapOrigin {
    Grant(WorkGrantLocatorV1),
    Kind(WorkKindLocatorV1),
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct KindOrigin {
    schema_ref: String,
    kind: WorkKindLocatorV1,
}
impl Serialize for BootstrapOrigin {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Grant(g) => g.serialize(serializer),
            Self::Kind(kind) => KindOrigin {
                schema_ref: "handbook.work-state-bootstrap-kind@2.0.0".into(),
                kind: kind.clone(),
            }
            .serialize(serializer),
        }
    }
}
impl<'de> Deserialize<'de> for BootstrapOrigin {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = serde_json::Value::deserialize(deserializer)?;
        if value
            .as_object()
            .is_some_and(|o| o.contains_key("schema_ref"))
        {
            let origin: KindOrigin =
                serde_json::from_value(value).map_err(serde::de::Error::custom)?;
            if origin.schema_ref != "handbook.work-state-bootstrap-kind@2.0.0" {
                return Err(serde::de::Error::custom("unknown bootstrap origin"));
            }
            Ok(Self::Kind(origin.kind))
        } else {
            serde_json::from_value(value)
                .map(Self::Grant)
                .map_err(serde::de::Error::custom)
        }
    }
}
impl BootstrapOrigin {
    fn validate(&self) -> Result<(), LocalAuthorityRefusalV1> {
        match self {
            Self::Grant(v) => v.validate(),
            Self::Kind(v) => v.validate(),
        }
    }
    fn check(&self) -> &crate::local_authority::local_check::LocalCheckLocatorV1 {
        match self {
            Self::Grant(v) => &v.check,
            Self::Kind(v) => &v.check,
        }
    }
    fn project(&self) -> &str {
        self.check()
            .predecessor
            .charter
            .binding
            .initialization_claim
            .proposed_repository_identity()
    }
    fn schema(&self) -> &'static str {
        match self {
            Self::Grant(_) => "handbook.work-state-owner@1.0.0",
            Self::Kind(_) => "handbook.work-state-owner@2.0.0",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct SlotEntry {
    name: String,
    intent_byte_fingerprint: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct Slot {
    schema: String,
    owner_identity: String,
    slot: usize,
    entry: Option<SlotEntry>,
}

struct Manifest {
    slots: Vec<Slot>,
    paths: Vec<String>,
    bytes: usize,
}

fn empty_slot(owner: &Owner, index: usize) -> Slot {
    Slot {
        schema: SLOT_SCHEMA.into(),
        owner_identity: owner.identity.clone(),
        slot: index,
        entry: None,
    }
}

fn slot_name(index: usize) -> String {
    format!("slot-{index:02}.json")
}
fn displaced_slot_name(index: usize) -> String {
    format!("slot-{index:02}.displaced.json")
}
fn prepared_slot_name(index: usize) -> String {
    format!("slot-{index:02}.prepared.json")
}
fn slot_path(index: usize) -> String {
    format!("{WORK_STATE_ROOT_V1}/{}", slot_name(index))
}
fn displaced_slot_path(index: usize) -> String {
    format!("{WORK_STATE_ROOT_V1}/{}", displaced_slot_name(index))
}
fn prepared_slot_path(index: usize) -> String {
    format!("{WORK_STATE_ROOT_V1}/{}", prepared_slot_name(index))
}
fn slot_artifact(name: &str) -> bool {
    (0..CAPACITY).any(|index| {
        [
            slot_name(index),
            displaced_slot_name(index),
            prepared_slot_name(index),
        ]
        .iter()
        .any(|expected| expected == name)
    })
}

fn valid_intent_fingerprint(value: &str) -> bool {
    value.strip_prefix("sha256:").is_some_and(|digest| {
        digest.len() == 64
            && digest
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    })
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Owner {
    schema: String,
    pub project: String,
    physical: String,
    initialization: crate::RepositoryInitializationClaimV1,
    bootstrap: BootstrapOrigin,
    lock_fingerprint: String,
    pub identity: String,
}
pub(super) struct WorkLock<'a> {
    pub root: &'a RetainedRepositoryRootV1,
    lock: RepositoryExclusiveLockV1,
    pub owner: Owner,
}
impl WorkLock<'_> {
    pub fn validate(&self) -> Result<(), LocalAuthorityRefusalV1> {
        self.lock.validate_at(self.root, LOCK).map_err(fs_error)?;
        if self.lock.physical_fingerprint_v1().map_err(fs_error)? != self.owner.lock_fingerprint
            || required(self.root, OWNER)? != encode(&self.owner)?
            || required(self.root, ANCHOR)? != encode(&self.owner)?
            || required(self.root, BOOT)? != encode(&self.owner.bootstrap)?
            || self
                .root
                .governance_physical_root_fingerprint_v1()
                .map_err(fs_error)?
                != self.owner.physical
        {
            return Err(refuse(
                "work_owner_substituted",
                "original retained physical root/owner/lock required",
            ));
        }
        Ok(())
    }
    pub fn inventory(&self) -> Result<Inventory, LocalAuthorityRefusalV1> {
        self.validate()?;
        let parent = self
            .root
            .list_directory_bounded(".handbook/work-state", 2)
            .map_err(fs_error)?;
        if parent.len() != 2
            || parent.iter().any(|entry| {
                !matches!(
                    (entry.name(), entry.kind()),
                    ("work.lock", Kind::RegularFile) | ("v1", Kind::Directory)
                )
            })
        {
            return Err(refuse(
                "work_inventory",
                "complete closed owner root inventory required",
            ));
        }
        if self
            .root
            .list_directory_bounded(".handbook", 4096)
            .map_err(fs_error)?
            .iter()
            .any(|e| e.name() == "work-state-bootstrap")
        {
            return Err(refuse(
                "work_bootstrap_conflict",
                "published owner cannot have a substituted/pending bootstrap candidate",
            ));
        }
        let manifest = self.manifest()?;
        let mut transactions = BTreeMap::new();
        let mut bytes = manifest
            .bytes
            .checked_add(required(self.root, OWNER)?.len())
            .and_then(|bytes| bytes.checked_add(required(self.root, ANCHOR).ok()?.len()))
            .and_then(|bytes| bytes.checked_add(required(self.root, BOOT).ok()?.len()))
            .ok_or_else(|| codec("overflow"))?;
        if bytes > INVENTORY_BYTES {
            return Err(refuse(
                "work_inventory_bound",
                "complete owner records exceed byte bound",
            ));
        }
        for entry in self
            .root
            .list_directory_bounded(WORK_STATE_ROOT_V1, ROOT_INVENTORY_ENTRIES)
            .map_err(fs_error)?
        {
            if entry.name() == "owner.json" && entry.kind() == Kind::RegularFile {
                continue;
            }
            if slot_artifact(entry.name()) && entry.kind() == Kind::RegularFile {
                continue;
            }
            if entry.kind() != Kind::Directory || !valid_transaction_name(entry.name()) {
                return Err(refuse(
                    "work_inventory",
                    "unknown or unsupported owner inventory entry",
                ));
            }
            let base = format!("{WORK_STATE_ROOT_V1}/{}", entry.name());
            let mut files = BTreeMap::new();
            for file in self
                .root
                .list_directory_bounded(&base, 8)
                .map_err(fs_error)?
            {
                if file.kind() != Kind::RegularFile
                    || ![
                        "intent.json",
                        "not-attempted.json",
                        "attempted.json",
                        "decision.json",
                        "snapshot.json",
                        "terminal.pending",
                        "terminal.json",
                        "noncommit.json",
                    ]
                    .contains(&file.name())
                {
                    return Err(refuse(
                        "work_inventory",
                        "unknown transaction target or nonregular entry",
                    ));
                }
                let b = required(self.root, &format!("{base}/{}", file.name()))?;
                bytes = bytes
                    .checked_add(b.len())
                    .ok_or_else(|| codec("overflow"))?;
                if bytes > INVENTORY_BYTES {
                    return Err(refuse(
                        "work_inventory_bound",
                        "complete owner inventory exceeds byte bound",
                    ));
                }
                files.insert(file.name().to_owned(), b);
            }
            transactions.insert(entry.name().to_owned(), files);
        }
        let mut declared = BTreeMap::new();
        for slot in &manifest.slots {
            if let Some(entry) = &slot.entry {
                if declared
                    .insert(entry.name.clone(), entry.intent_byte_fingerprint.clone())
                    .is_some()
                {
                    return Err(refuse(
                        "work_manifest_conflict",
                        "one request cannot occupy multiple retained slots",
                    ));
                }
                let accepted = transactions.get(&entry.name);
                let candidate = transactions.get(&format!("accept-{}", entry.name));
                let files = match (accepted, candidate) {
                    (Some(files), None) | (None, Some(files)) => files,
                    _ => return Err(refuse("work_manifest_missing", "declared original transaction is missing or conflicts; loss is not absence")),
                };
                let intent = files.get("intent.json").ok_or_else(|| refuse("work_manifest_missing", "declared transaction has no exact original intent; reservation remains uncertain"))?;
                if crate::DefinitionFingerprint::from_bytes(intent).as_str()
                    != entry.intent_byte_fingerprint
                {
                    return Err(refuse(
                        "work_manifest_intent",
                        "retained transaction intent differs from its positive slot declaration",
                    ));
                }
            }
        }
        if transactions.len() > CAPACITY
            || transactions
                .keys()
                .any(|name| !declared.contains_key(name.strip_prefix("accept-").unwrap_or(name)))
        {
            return Err(refuse(
                "work_manifest_undeclared",
                "every transaction must have exactly one retained positive slot declaration",
            ));
        }
        self.validate()?;
        Ok(Inventory {
            transactions,
            bytes,
            manifest_paths: manifest.paths,
        })
    }

    // Independent required-slot reads also work immediately after a durable
    // reservation but before its directory exists. Normal inventory refuses
    // that prefix rather than treating the missing declared effect as absence.
    fn manifest(&self) -> Result<Manifest, LocalAuthorityRefusalV1> {
        self.validate()?;
        let mut slots = Vec::with_capacity(CAPACITY);
        let mut paths = Vec::new();
        let mut bytes = 0usize;
        for index in 0..CAPACITY {
            let path = slot_path(index);
            let content = required(self.root, &path)?;
            let slot: Slot = decode(&content)?;
            if slot.schema != SLOT_SCHEMA
                || slot.owner_identity != self.owner.identity
                || slot.slot != index
                || encode(&slot)? != content
            {
                return Err(refuse(
                    "work_manifest_slot",
                    "required exact canonical owner/index slot record differs",
                ));
            }
            bytes = bytes
                .checked_add(content.len())
                .ok_or_else(|| codec("overflow"))?;
            let displaced_path = displaced_slot_path(index);
            let displaced = optional(self.root, &displaced_path)?;
            let prepared_path = prepared_slot_path(index);
            if let Some(content) = optional(self.root, &prepared_path)? {
                let pending: Slot = decode(&content)?;
                if pending.schema != SLOT_SCHEMA
                    || pending.owner_identity != self.owner.identity
                    || pending.slot != index
                    || pending.entry.is_none()
                    || encode(&pending)? != content
                {
                    return Err(refuse(
                        "work_manifest_slot",
                        "pending replacement slot is substituted",
                    ));
                }
                return Err(refuse(
                    "work_manifest_pending",
                    "slot publication is pending or uncertain; no empty-slot fallback",
                ));
            }
            match &slot.entry {
                None if displaced.is_some() => {
                    return Err(refuse(
                        "work_manifest_reset",
                        "a consumed slot cannot be reset to empty",
                    ))
                }
                None => {}
                Some(entry) => {
                    if entry.name.starts_with("accept-")
                        || !valid_transaction_name(&entry.name)
                        || !valid_intent_fingerprint(&entry.intent_byte_fingerprint)
                    {
                        return Err(refuse(
                            "work_manifest_slot",
                            "slot must declare one exact bounded request and intent byte identity",
                        ));
                    }
                    let expected = encode(&empty_slot(&self.owner, index))?;
                    if displaced.as_deref() != Some(expected.as_slice()) {
                        return Err(refuse(
                            "work_manifest_displaced",
                            "occupied slot requires its exact retained displaced empty predecessor",
                        ));
                    }
                    bytes = bytes
                        .checked_add(expected.len())
                        .ok_or_else(|| codec("overflow"))?;
                    paths.push(displaced_path);
                }
            }
            paths.push(path);
            slots.push(slot);
        }
        paths.sort();
        self.validate()?;
        Ok(Manifest {
            slots,
            paths,
            bytes,
        })
    }

    /// Durably consumes exactly one positive required empty slot before the
    /// caller creates an acceptance directory. Mutation is marked attempted
    /// before the first native write, after all effect-free capacity checks.
    /// Slots and displaced predecessors are never reused.
    pub fn reserve_intent(
        &self,
        name: &str,
        intent: &[u8],
        mutation_attempted: &Cell<bool>,
    ) -> Result<(), LocalAuthorityRefusalV1> {
        if name.starts_with("accept-")
            || !valid_transaction_name(name)
            || intent.is_empty()
            || intent.len() > MAX
        {
            return Err(refuse(
                "work_manifest_request",
                "one exact bounded transaction intent required",
            ));
        }
        let entry = SlotEntry {
            name: name.into(),
            intent_byte_fingerprint: crate::DefinitionFingerprint::from_bytes(intent).to_string(),
        };
        let inventory = self.inventory()?;
        let manifest = self.manifest()?;
        if let Some(slot) = manifest.slots.iter().find(|slot| {
            slot.entry
                .as_ref()
                .is_some_and(|retained| retained.name == name)
        }) {
            if slot.entry.as_ref() != Some(&entry) {
                return Err(refuse(
                    "work_manifest_conflict",
                    "original slot request or intent bytes conflict",
                ));
            }
            let accepted = optional(
                self.root,
                &format!("{WORK_STATE_ROOT_V1}/{name}/intent.json"),
            )?;
            let candidate = optional(
                self.root,
                &format!("{WORK_STATE_ROOT_V1}/accept-{name}/intent.json"),
            )?;
            return match (accepted, candidate) {
                (Some(bytes), None) | (None, Some(bytes)) if bytes == intent => Ok(()),
                _ => Err(refuse("work_manifest_uncertain", "occupied slot has no exact singular original transaction prefix; reservation cannot be replaced")),
            };
        }
        let index = manifest
            .slots
            .iter()
            .find(|slot| slot.entry.is_none())
            .map(|slot| slot.slot)
            .ok_or_else(|| {
                refuse(
                    "work_capacity",
                    "all positive retained transaction slots are consumed",
                )
            })?;
        let mut slot = empty_slot(&self.owner, index);
        let before = encode(&slot)?;
        slot.entry = Some(entry);
        let after = encode(&slot)?;
        if inventory
            .bytes
            .checked_add(8 * MAX)
            .and_then(|bytes| bytes.checked_add(after.len()))
            .and_then(|bytes| bytes.checked_add(before.len()))
            .is_none_or(|bytes| bytes > INVENTORY_BYTES)
        {
            return Err(refuse(
                "work_capacity",
                "reserve complete peak transaction and slot publication bytes before acceptance",
            ));
        }
        let prepared = prepared_slot_path(index);
        mutation_attempted.set(true);
        self.root
            .write_new_durable(&prepared, &after)
            .map_err(fs_error)?;
        self.lock.validate_at(self.root, LOCK).map_err(fs_error)?;
        self.root
            .replace_exact_durable(
                &slot_path(index),
                &before,
                &prepared,
                &displaced_slot_path(index),
            )
            .map_err(fs_error)?;
        if required(self.root, &slot_path(index))? != after
            || required(self.root, &displaced_slot_path(index))? != before
            || optional(self.root, &prepared)?.is_some()
        {
            return Err(refuse(
                "work_manifest_uncertain",
                "exact slot replacement publication is unverifiable",
            ));
        }
        self.validate()
    }
}
pub(super) struct Inventory {
    pub transactions: BTreeMap<String, BTreeMap<String, Vec<u8>>>,
    pub bytes: usize,
    pub manifest_paths: Vec<String>,
}
fn valid_transaction_name(v: &str) -> bool {
    let v = v.strip_prefix("accept-").unwrap_or(v);
    ["grant-", "item-", "revoke-", "kind-"]
        .iter()
        .any(|p| v.strip_prefix(p).is_some_and(|s| token(s).is_ok()))
}

pub(super) fn open<'a>(
    root: &'a RetainedRepositoryRootV1,
) -> Result<WorkLock<'a>, LocalAuthorityRefusalV1> {
    let owner: Owner = decode(&required(root, ANCHOR)?)?;
    owner.initialization.validate().map_err(fs_error)?;
    owner.bootstrap.validate()?;
    if owner.schema != owner.bootstrap.schema()
        || owner.project != owner.bootstrap.project()
        || owner.physical != owner.initialization.physical_root_fingerprint()
        || owner.identity != owner_identity(&owner.bootstrap)?
        || required(root, BOOT)? != encode(&owner.bootstrap)?
    {
        return Err(refuse(
            "work_owner_proof",
            "positive exact initialized owner proof required",
        ));
    }
    let lock = root
        .acquire_existing_exclusive_lock_v1(LOCK)
        .map_err(fs_error)?;
    let result = WorkLock { root, lock, owner };
    result.validate()?;
    Ok(result)
}
pub(super) fn owner_identity(origin: &BootstrapOrigin) -> Result<String, LocalAuthorityRefusalV1> {
    fp(
        "handbook.work-state-owner-identity@1.0.0",
        &(
            origin.project(),
            origin
                .check()
                .predecessor
                .charter
                .binding
                .initialization_claim
                .claim_fingerprint(),
        ),
    )
}

// Called only under original live initialization/operating/check admission. The
// external bootstrap intent reserves the original activation across every prefix.
pub(super) fn establish<'a>(
    root: &'a RetainedRepositoryRootV1,
    origin: &BootstrapOrigin,
    mutation_attempted: &Cell<bool>,
    boundary: &mut dyn FnMut(super::WorkWriteBoundaryV1) -> Result<(), LocalAuthorityRefusalV1>,
) -> Result<WorkLock<'a>, LocalAuthorityRefusalV1> {
    if optional(root, ANCHOR)?.is_some() {
        return open(root);
    }
    origin.validate()?;
    let boot = encode(origin)?;
    // Check complete bootstrap/owner preimages before retaining the first
    // external reservation. Native lock fingerprints have this fixed length.
    let candidate = Owner {
        schema: origin.schema().into(),
        project: origin.project().into(),
        physical: root
            .governance_physical_root_fingerprint_v1()
            .map_err(fs_error)?,
        initialization: origin
            .check()
            .predecessor
            .charter
            .binding
            .initialization_claim
            .clone(),
        bootstrap: origin.clone(),
        lock_fingerprint: format!("sha256:{}", "0".repeat(64)),
        identity: owner_identity(origin)?,
    };
    encode(&candidate)?;
    match optional(root, BOOT)? {
        Some(b) if b == boot => {
            if root
                .list_directory_bounded(".handbook/work-state", 64)
                .is_ok()
                || optional(root, &format!("{STAGE}/v1/owner.json"))?.is_none()
            {
                return Err(refuse("work_bootstrap_uncertain","establishment already began but exact original unpublished owner proof is unavailable; no replacement lock or owner"));
            }
        }
        Some(_) => {
            return Err(refuse(
                "work_bootstrap_reserved",
                "only original activation may recover pending owner establishment",
            ))
        }
        None => {
            if root
                .list_directory_bounded(".handbook/work-state", 64)
                .is_ok()
                || root.list_directory_bounded(STAGE, 64).is_ok()
            {
                return Err(refuse(
                    "work_owner_lost",
                    "owner footprint without retained establishment proof is not fresh",
                ));
            }
            mutation_attempted.set(true);
            root.write_new_durable(BOOT, &boot).map_err(fs_error)?;
            boundary(super::WorkWriteBoundaryV1::BootstrapIntent)?;
        }
    }
    mutation_attempted.set(true);
    root.create_directories(&format!("{STAGE}/v1"))
        .map_err(fs_error)?;
    let stage_lock = format!("{STAGE}/work.lock");
    let stage_owner = format!("{STAGE}/v1/owner.json");
    let stage_owner_present = optional(root, &stage_owner)?.is_some();
    let retained = if stage_owner_present {
        root.acquire_existing_exclusive_lock_v1(&stage_lock)
    } else {
        root.acquire_exclusive_lock(&stage_lock)
    }
    .map_err(fs_error)?;
    let owner = Owner {
        lock_fingerprint: retained.physical_fingerprint_v1().map_err(fs_error)?,
        ..candidate
    };
    for index in 0..CAPACITY {
        let path = format!("{STAGE}/v1/{}", slot_name(index));
        let expected = encode(&empty_slot(&owner, index))?;
        if stage_owner_present {
            if required(root, &path)? != expected {
                return Err(refuse(
                    "work_bootstrap_manifest",
                    "original unpublished owner slots are missing or changed; no regeneration",
                ));
            }
        } else {
            write_exact(root, &path, &expected)?;
        }
    }
    write_exact(root, &stage_owner, &encode(&owner)?)?;
    retained.validate_at(root, &stage_lock).map_err(fs_error)?;
    drop(retained);
    root.rename_directory_noreplace_durable(STAGE, ".handbook/work-state")
        .map_err(fs_error)?;
    write_exact(root, ANCHOR, &encode(&owner)?)?;
    open(root)
}
pub(super) fn optional(
    root: &RetainedRepositoryRootV1,
    p: &str,
) -> Result<Option<Vec<u8>>, LocalAuthorityRefusalV1> {
    match root
        .read_local_owner_optional_bounded(p, MAX)
        .map_err(fs_error)?
    {
        RepositoryReadV1::Absent => Ok(None),
        RepositoryReadV1::Found(v) => Ok(Some(v)),
    }
}
pub(super) fn required(
    root: &RetainedRepositoryRootV1,
    p: &str,
) -> Result<Vec<u8>, LocalAuthorityRefusalV1> {
    optional(root, p)?.ok_or_else(|| {
        refuse(
            "work_proof_missing",
            "required retained proof missing; loss is not absence",
        )
    })
}
pub(super) fn write_exact(
    root: &RetainedRepositoryRootV1,
    p: &str,
    b: &[u8],
) -> Result<(), LocalAuthorityRefusalV1> {
    match optional(root, p)? {
        Some(old) if old == b => Ok(()),
        Some(_) => Err(refuse(
            "work_conflict",
            "exact retained request or owner output conflicts",
        )),
        None => root.write_new_durable(p, b).map_err(fs_error),
    }
}
