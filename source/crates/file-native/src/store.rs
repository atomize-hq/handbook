use crate::{
    backend::{Backend, Storage},
    digest,
    index::DiskIndex,
    objects::{self, Objects},
    Result, MAX_CHANGES, MAX_OBJECT, MAX_READERS, MAX_TRANSACTION_BYTES,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
/// Operation-local diagnostic counts. These describe completed branch construction,
/// including the unselected negative alternative, not durable publication success.
/// Reset immediately before a synchronous operation and take on the same thread.
#[cfg(feature = "test-support")]
#[derive(Clone, Debug, Default, Serialize)]
pub struct BranchBuildCounts {
    pub positive_branches: u64,
    pub negative_branches: u64,
    pub same_key_equal_values: u64,
    pub reused_values: u64,
    pub value_allocations: u64,
    pub value_retirements: u64,
    pub pending_objects: u64,
    pub pending_framed_bytes: u64,
    pub retired_objects: u64,
}
#[cfg(feature = "test-support")]
std::thread_local! {
    static BRANCH_BUILD_COUNTS: std::cell::RefCell<BranchBuildCounts> =
        std::cell::RefCell::new(BranchBuildCounts::default());
}
#[cfg(feature = "test-support")]
pub fn take_branch_build_counts() -> BranchBuildCounts {
    BRANCH_BUILD_COUNTS.with(|counts| std::mem::take(&mut *counts.borrow_mut()))
}

const CONTROL: usize = 2 * 1024 * 1024;
const GATE: &str = "gate.lock";
fn nonce() -> Result<String> {
    let mut bytes = [0u8; 32];
    getrandom::fill(&mut bytes).map_err(|e| format!("native entropy unavailable: {e}"))?;
    Ok(digest(&bytes))
}
fn encode<T: Serialize>(v: &T) -> Result<Vec<u8>> {
    let b = serde_json::to_vec(v)?;
    if b.len() > CONTROL {
        return Err("bounded control record".into());
    }
    Ok(b)
}
fn required(io: &dyn Storage, p: &str, n: usize) -> Result<Vec<u8>> {
    io.read(p, n)?
        .ok_or_else(|| "required storage proof missing".into())
}
fn exact(io: &dyn Storage, p: &str, b: &[u8]) -> Result<()> {
    match io.read(p, b.len())? {
        Some(old) if old == b => Ok(()),
        Some(_) => Err("immutable record conflicts".into()),
        None => {
            if let Some((parent, _)) = p.rsplit_once('/') {
                io.mkdir(parent)?;
            }
            io.write_new(p, b)
        }
    }
}
// Exact names are reserved by initialized Ready or the current owned plan.
// Staging bytes are never adopted as proof; only complete final files are read.
fn atomic_exact(io: &dyn Storage, p: &str, b: &[u8]) -> Result<()> {
    match io.read(p, b.len())? {
        Some(old) if old == b => return Ok(()),
        Some(_) => return Err("immutable control/output conflicts".into()),
        None => {}
    }
    let writing = format!("{p}.writing");
    io.remove(&writing)?;
    if let Some((parent, _)) = p.rsplit_once('/') {
        io.mkdir(parent)?;
    }
    io.write_new(&writing, b)?;
    io.rename_new(&writing, p)
}
fn load<T: for<'de> Deserialize<'de>>(io: &dyn Storage, p: &str) -> Result<T> {
    Ok(serde_json::from_slice(&required(io, p, CONTROL)?)?)
}
fn replace(
    io: &dyn Storage,
    p: &str,
    b: &[u8],
    mut displaced: impl FnMut() -> Result<()>,
) -> Result<()> {
    let old = required(io, p, CONTROL)?;
    if old == b {
        return Ok(());
    }
    let prepared = format!("{p}.prepared");
    let before = format!("{p}.displaced");
    if io.read(&before, CONTROL)?.is_some() {
        return Err("replacement requires recovery".into());
    }
    atomic_exact(io, &prepared, b)?;
    io.displace(p, &old, &before)?;
    displaced()?;
    io.rename_new(&prepared, p)?;
    io.remove(&before)?;
    Ok(())
}
// Restoring an interrupted displaced predecessor never selects a new effect.
fn recover_replace(io: &dyn Storage, p: &str) -> Result<()> {
    let before = format!("{p}.displaced");
    let prepared = format!("{p}.prepared");
    io.remove(&format!("{prepared}.writing"))?;
    if io.read(p, CONTROL)?.is_none() && io.read(&before, CONTROL)?.is_some() {
        io.rename_new(&before, p)?;
    }
    if io.read(p, CONTROL)?.is_some() {
        io.remove(&before)?;
        io.remove(&prepared)?;
    }
    Ok(())
}
fn number_path(prefix: &str, n: u64) -> String {
    let h = format!("{n:016x}");
    format!(
        "{prefix}/{}/{}/{}/{}/{}/{}/{}/{}",
        &h[..2],
        &h[2..4],
        &h[4..6],
        &h[6..8],
        &h[8..10],
        &h[10..12],
        &h[12..14],
        h
    )
}
fn event_path(n: u64) -> String {
    number_path("retirements", n)
}
fn pin_path(n: usize) -> String {
    format!("readers/{n}.json")
}
fn pin_lock(n: usize) -> String {
    format!("readers/{n}.lock")
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Version {
    pub sequence: u64,
    root: Option<String>,
    store: String,
    pub records: u64,
    pub indexed_originals: u64,
    pub logical_bytes: u64,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Selected {
    version: Version,
    token: String,
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum OriginalDisposition {
    Selected,
    Conflict,
    NotSelected,
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OriginalOutcome {
    pub disposition: OriginalDisposition,
    pub sequence: u64,
    pub value: Vec<u8>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Original {
    key: String,
    fingerprint: String,
    outcome: OriginalOutcome,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    evidence: Option<RetainedEvidence>,
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CandidateIdentity {
    pub store: String,
    pub token: String,
    pub sequence: u64,
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EvidenceReference {
    pub key: String,
    pub fingerprint: String,
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EvidenceBinding {
    pub format: String,
    pub namespace: String,
    pub store: String,
    pub key: String,
    pub fingerprint: String,
    pub expected: Version,
    pub predecessor: Version,
    pub predecessor_token: String,
    pub candidate: CandidateIdentity,
    pub resolving: CandidateIdentity,
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RetainedEvidence {
    pub scope: String,
    pub payload: Vec<u8>,
    pub binding: EvidenceBinding,
    pub previous: Option<EvidenceReference>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ScopeHead {
    scope: String,
    reference: EvidenceReference,
}
#[derive(Clone, Debug)]
pub struct OriginalEvidence {
    pub reference: EvidenceReference,
    pub outcome: OriginalOutcome,
    pub evidence: RetainedEvidence,
}
#[derive(Clone, Debug)]
pub struct EvidenceObservation {
    pub at: Version,
    pub original: Option<OriginalEvidence>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct OpaqueEvidence {
    scope: String,
    payload: Vec<u8>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OriginalResolution {
    Absent,
    Pending,
    Resolved(OriginalOutcome),
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Event {
    sequence: u64,
    token: String,
    retired: Vec<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Branch {
    candidate: Selected,
    original: Original,
    owned: Vec<String>,
    event: Event,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Plan {
    prior: Selected,
    positive: Branch,
    negative: Branch,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(
    tag = "state",
    content = "value",
    rename_all = "snake_case",
    deny_unknown_fields
)]
enum ActiveCell {
    Idle(Selected),
    Owned(Plan),
}
struct BuiltBranch {
    branch: Branch,
    pending: BTreeMap<String, Vec<u8>>,
}
fn same_selected(a: &Selected, b: &Selected) -> bool {
    a.version == b.version && a.token == b.token
}
fn same_attempt(a: &Option<RetainedEvidence>, b: &Option<RetainedEvidence>) -> bool {
    match (a, b) {
        (None, None) => true,
        (Some(a), Some(b)) => {
            let mut binding = a.binding.clone();
            binding.resolving = b.binding.resolving.clone();
            a.scope == b.scope
                && a.payload == b.payload
                && a.previous == b.previous
                && binding == b.binding
        }
        _ => false,
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Cursor {
    sequence: u64,
    item: usize,
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Binding {
    format: String,
    namespace: String,
    store: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Ready {
    binding: Binding,
    locks: Vec<String>,
    control_format: String,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CreationBoundary {
    Binding,
    Store,
    Locks,
    Selector,
    Ready,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Transaction {
    expected: Version,
    key: String,
    outcome: Vec<u8>,
    changes: BTreeMap<String, Option<Vec<u8>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    evidence: Option<OpaqueEvidence>,
}
impl Transaction {
    pub fn new(expected: Version, original_key: impl Into<String>, outcome: Vec<u8>) -> Self {
        Self {
            expected,
            key: original_key.into(),
            outcome,
            changes: BTreeMap::new(),
            evidence: None,
        }
    }
    /// Opaque caller evidence grants no domain permission.
    pub fn set_evidence(&mut self, scope: impl Into<String>, payload: Vec<u8>) -> Result<()> {
        let scope = scope.into();
        if self.evidence.is_some()
            || scope.is_empty()
            || scope.len() > 1000
            || payload.len() > MAX_OBJECT
        {
            return Err("evidence admission bound/duplicate".into());
        }
        self.evidence = Some(OpaqueEvidence { scope, payload });
        Ok(())
    }
    pub fn put(&mut self, key: impl Into<String>, value: Vec<u8>) -> Result<()> {
        self.change(key.into(), Some(value))
    }
    pub fn delete(&mut self, key: impl Into<String>) -> Result<()> {
        self.change(key.into(), None)
    }
    fn change(&mut self, key: String, value: Option<Vec<u8>>) -> Result<()> {
        if key.is_empty()
            || key.len() > 1000
            || value.as_ref().is_some_and(|v| v.len() > MAX_OBJECT)
        {
            return Err("per-record bound".into());
        }
        if !self.changes.contains_key(&key) && self.changes.len() >= MAX_CHANGES {
            return Err("transaction change bound".into());
        }
        self.changes.insert(key, value);
        Ok(())
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PublicationBoundary {
    BeforeOwnership,
    Owned,
    Outputs,
    BeforeSelection,
    SelectorDisplaced,
    Selected,
    Reply,
}
#[derive(Clone, Copy, Debug)]
pub struct MaintenanceBudget {
    pub items: usize,
    pub bytes: u64,
}
impl Default for MaintenanceBudget {
    fn default() -> Self {
        Self {
            items: 128,
            bytes: 16 * 1024 * 1024,
        }
    }
}
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct MaintenanceProgress {
    pub sequence: u64,
    pub item: usize,
    pub visited: usize,
    pub removed_files: usize,
    pub removed_bytes: u64,
    pub blocked_by_reader: bool,
    pub complete: bool,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Statistics {
    pub version: Version,
    pub maintenance_sequence: u64,
    pub directory_sync_supported: bool,
}

/// Read-only selected data. A current view is valid only inside with_current;
/// a historical Snapshot additionally retains its independent native lease.
pub trait ReadView {
    fn version(&self) -> &Version;
    fn get(&self, key: &str) -> Result<Option<Vec<u8>>>;
}
struct CurrentView<'a, B: Backend> {
    db: &'a Database<B>,
    version: Version,
}
impl<B: Backend> ReadView for CurrentView<'_, B> {
    fn version(&self) -> &Version {
        &self.version
    }
    fn get(&self, key: &str) -> Result<Option<Vec<u8>>> {
        self.db.indexed(&self.version, &format!("d/{key}"))
    }
}

#[derive(Clone)]
pub struct Database<B: Backend> {
    io: B,
    binding: Binding,
    ready: Ready,
}
impl<B: Backend> Database<B> {
    pub fn create(io: B, namespace: &str) -> Result<Self> {
        Self::create_inner(io, namespace, &mut |_| Ok(()))
    }
    #[cfg(feature = "test-support")]
    pub fn create_with_boundary(
        io: B,
        namespace: &str,
        mut boundary: impl FnMut(CreationBoundary) -> Result<()>,
    ) -> Result<Self> {
        Self::create_inner(io, namespace, &mut boundary)
    }
    fn create_inner(
        io: B,
        namespace: &str,
        boundary: &mut dyn FnMut(CreationBoundary) -> Result<()>,
    ) -> Result<Self> {
        if namespace.is_empty() || namespace.len() > 4096 {
            return Err("namespace bound".into());
        }
        io.validate()?;
        if io.read("STORE.json", CONTROL)?.is_some()
            || io.read("CREATING.json", CONTROL)?.is_some()
            || io.read("READY.json", CONTROL)?.is_some()
        {
            return Err("database already initialized or reserved".into());
        }
        let binding = Binding {
            format: "file-native@1".into(),
            namespace: namespace.into(),
            store: nonce()?,
        };
        io.write_new("CREATING.json", &encode(&binding)?)?;
        boundary(CreationBoundary::Binding)?;
        Self::finish_creation(io, binding, boundary)
    }
    fn finish_creation(
        io: B,
        binding: Binding,
        boundary: &mut dyn FnMut(CreationBoundary) -> Result<()>,
    ) -> Result<Self> {
        exact(&io, "STORE.json", &encode(&binding)?)?;
        boundary(CreationBoundary::Store)?;
        exact(&io, GATE, &[])?;
        io.mkdir("readers")?;
        for n in 0..MAX_READERS {
            exact(&io, &pin_lock(n), &[])?;
            exact(&io, &pin_path(n), b"null")?;
        }
        boundary(CreationBoundary::Locks)?;
        exact(
            &io,
            "maintenance.json",
            &encode(&Cursor {
                sequence: 1,
                item: 0,
            })?,
        )?;
        let version = Version {
            sequence: 0,
            root: None,
            store: binding.store.clone(),
            records: 0,
            indexed_originals: 0,
            logical_bytes: 0,
        };
        let initial = Selected {
            version,
            token: binding.store.clone(),
        };
        exact(&io, "SELECTED.json", &encode(&initial)?)?;
        exact(&io, "active.json", &encode(&ActiveCell::Idle(initial))?)?;
        boundary(CreationBoundary::Selector)?;
        let mut locks = Vec::new();
        let gate = io.lock(GATE)?;
        locks.push(io.lock_identity(&gate)?);
        for n in 0..MAX_READERS {
            let guard = io.lock(&pin_lock(n))?;
            locks.push(io.lock_identity(&guard)?);
        }
        let ready = Ready {
            binding: binding.clone(),
            locks,
            control_format: "atomic-control@1".into(),
        };
        exact(&io, "READY.json", &encode(&ready)?)?;
        boundary(CreationBoundary::Ready)?;
        io.remove("CREATING.json")?;
        io.validate_lock(GATE, &gate)?;
        Ok(Self { io, binding, ready })
    }
    /// Resumes only the positively reserved empty creation. Once creation is
    /// complete the witness is removed; lost initialized state cannot use this.
    pub fn resume_creation(io: B, namespace: &str) -> Result<Self> {
        io.validate()?;
        let binding: Binding = load(&io, "CREATING.json")?;
        if binding.format != "file-native@1" || binding.namespace != namespace {
            return Err("original creation binding required".into());
        }
        Self::finish_creation(io, binding, &mut |_| Ok(()))
    }
    pub fn open(io: B, namespace: &str) -> Result<Self> {
        io.validate()?;
        let binding: Binding = load(&io, "STORE.json")?;
        let ready: Ready = load(&io, "READY.json")?;
        if binding.format != "file-native@1"
            || binding.namespace != namespace
            || binding != ready.binding
            || ready.locks.len() != MAX_READERS + 1
            || ready.control_format != "atomic-control@1"
            || io.read("CREATING.json", CONTROL)?.is_some()
        {
            return Err("database binding/ready proof differs".into());
        }
        let db = Self { io, binding, ready };
        let gate = db.gate()?;
        db.selected()?;
        db.io.validate_lock(GATE, &gate)?;
        drop(gate);
        Ok(db)
    }
    fn gate(&self) -> Result<B::Guard> {
        let guard = self.io.lock(GATE)?;
        if self.io.lock_identity(&guard)? != self.ready.locks[0] {
            return Err("original native publication lock required".into());
        }
        Ok(guard)
    }
    fn reader_identity(&self, n: usize, guard: &B::Guard) -> Result<()> {
        if self.io.lock_identity(guard)? != self.ready.locks[n + 1] {
            return Err("original native reader lock required".into());
        }
        Ok(())
    }
    fn selected(&self) -> Result<Selected> {
        let s: Selected = load(&self.io, "SELECTED.json")?;
        if s.version.store != self.binding.store {
            return Err("selected store binding differs".into());
        }
        Ok(s)
    }
    pub fn version(&self) -> Result<Version> {
        let g = self.gate()?;
        let v = self.selected()?.version;
        self.io.validate_lock(GATE, &g)?;
        Ok(v)
    }
    fn indexed(&self, version: &Version, key: &str) -> Result<Option<Vec<u8>>> {
        let mut objects = Objects::new(&self.io, "");
        match DiskIndex::lookup_once(&mut objects, version.root.clone(), key)? {
            Some(id) if !id.is_empty() => Ok(Some(objects.read(std::str::from_utf8(&id)?)?)),
            _ => Ok(None),
        }
    }
    pub fn get(&self, key: &str) -> Result<Option<Vec<u8>>> {
        let g = self.gate()?;
        let v = self.selected()?.version;
        let out = self.indexed(&v, &format!("d/{key}"))?;
        self.io.validate_lock(GATE, &g)?;
        Ok(out)
    }
    /// Consumes bounded current reads while the original publication gate is
    /// retained for the entire callback. No historical pin is written. Do not
    /// reenter gate-acquiring operations from the callback. Copy read inputs out
    /// and exit this scope before publishing or maintaining.
    pub fn with_current<R>(&self, read: impl FnOnce(&dyn ReadView) -> R) -> Result<R> {
        let gate = self.gate()?;
        let view = CurrentView {
            db: self,
            version: self.selected()?.version,
        };
        let result = read(&view);
        self.io.validate_lock(GATE, &gate)?;
        Ok(result)
    }
    pub fn snapshot(&self) -> Result<Snapshot<B>> {
        let gate = self.gate()?;
        let v = self.selected()?.version;
        for n in 0..MAX_READERS {
            if let Some(guard) = self.io.try_lock(&pin_lock(n))? {
                self.reader_identity(n, &guard)?;
                recover_replace(&self.io, &pin_path(n))?;
                replace(
                    &self.io,
                    &pin_path(n),
                    &encode(&Some(v.clone()))?,
                    || Ok(()),
                )?;
                self.io.validate_lock(GATE, &gate)?;
                return Ok(Snapshot {
                    db: self.clone(),
                    version: v,
                    slot: n,
                    _guard: guard,
                });
            }
        }
        Err("active reader capacity; release a snapshot".into())
    }
    fn original_at(&self, v: &Version, key: &str) -> Result<Option<Original>> {
        self.indexed(v, &format!("o/{}", digest(key.as_bytes())))?
            .map(|b| Ok(serde_json::from_slice(&b)?))
            .transpose()
    }
    pub fn resolve(&self, key: &str) -> Result<OriginalResolution> {
        let gate = self.gate()?;
        let selected = self.selected()?;
        let cell: ActiveCell = load(&self.io, "active.json")?;
        let pending = match &cell {
            ActiveCell::Idle(idle) => {
                if !same_selected(idle, &selected) {
                    return Err("idle selector differs".into());
                }
                None
            }
            ActiveCell::Owned(plan) => {
                self.validate_plan(plan)?;
                Some(plan)
            }
        };
        let result = if let Some(original) = self.original_at(&selected.version, key)? {
            if original.key != key {
                return Err("original key collision".into());
            }
            OriginalResolution::Resolved(original.outcome)
        } else if pending.is_some_and(|p| p.positive.original.key == key) {
            OriginalResolution::Pending
        } else {
            OriginalResolution::Absent
        };
        self.io.validate_lock(GATE, &gate)?;
        Ok(result)
    }
    fn scope_head(&self, v: &Version, scope: &str) -> Result<Option<EvidenceReference>> {
        Ok(self
            .verified_scope_original(v, scope)?
            .map(|original| original.reference))
    }
    fn verified_scope_original(&self, v: &Version, scope: &str) -> Result<Option<OriginalEvidence>> {
        let bytes = self.indexed(v, &format!("e/{}", digest(scope.as_bytes())))?;
        let Some(bytes) = bytes else { return Ok(None) };
        let head: ScopeHead = serde_json::from_slice(&bytes)?;
        if head.scope != scope {
            return Err("scope identity collision".into());
        }
        let mut original = self
            .original_at(v, &head.reference.key)?
            .ok_or("scope original proof missing")?;
        if original.key != head.reference.key || original.fingerprint != head.reference.fingerprint
        {
            return Err("scope native original differs".into());
        }
        let evidence = original.evidence.take().ok_or("scope evidence missing")?;
        self.validate_evidence(&original, &evidence)?;
        if evidence.scope != scope {
            return Err("scope evidence differs".into());
        }
        Ok(Some(OriginalEvidence {
            reference: head.reference,
            outcome: original.outcome,
            evidence,
        }))
    }
    fn validate_evidence(&self, o: &Original, e: &RetainedEvidence) -> Result<()> {
        let b = &e.binding;
        if e.scope.is_empty()
            || e.scope.len() > 1000
            || e.payload.len() > MAX_OBJECT
            || b.format != self.binding.format
            || b.namespace != self.binding.namespace
            || b.store != self.binding.store
            || b.predecessor.store != self.binding.store
            || b.key != o.key
            || b.fingerprint != o.fingerprint
            || b.candidate.store != self.binding.store
            || b.resolving.store != self.binding.store
            || b.candidate.sequence
                != b.predecessor
                    .sequence
                    .checked_add(1)
                    .ok_or("evidence sequence")?
            || b.resolving.sequence != b.candidate.sequence
        {
            return Err("native evidence binding differs".into());
        }
        objects::path(&b.candidate.token)?;
        objects::path(&b.resolving.token)?;
        match o.outcome.disposition {
            OriginalDisposition::NotSelected
                if o.outcome.sequence == b.predecessor.sequence && o.outcome.value.is_empty() => {}
            OriginalDisposition::Selected | OriginalDisposition::Conflict
                if o.outcome.sequence == b.resolving.sequence => {}
            _ => return Err("evidence outcome differs".into()),
        }
        Ok(())
    }
    /// Reconciles only original ownership/outcome metadata, then makes bounded
    /// scope/exact-original lookups. Historical evidence grants no permission.
    pub fn reconcile_evidence(
        &self,
        scope: &str,
        original_key: Option<&str>,
    ) -> Result<EvidenceObservation> {
        if scope.is_empty() || scope.len() > 1000 {
            return Err("evidence scope bound".into());
        }
        let gate = self.gate()?;
        self.recover_held(&gate)?;
        let selected = self.selected()?;
        let original = if let Some(key) = original_key {
            self.scope_head(&selected.version, scope)?;
            let o = self
                .original_at(&selected.version, key)?
                .ok_or("required scoped original missing")?;
            let evidence = o
                .evidence
                .clone()
                .ok_or("required original evidence missing")?;
            self.validate_evidence(&o, &evidence)?;
            if evidence.scope != scope || o.key != key {
                return Err("original evidence scope/key differs".into());
            }
            Some(OriginalEvidence {
                reference: EvidenceReference {
                    key: o.key,
                    fingerprint: o.fingerprint,
                },
                outcome: o.outcome,
                evidence,
            })
        } else {
            self.verified_scope_original(&selected.version, scope)?
        };
        self.io.validate_lock(GATE, &gate)?;
        Ok(EvidenceObservation {
            at: selected.version,
            original,
        })
    }
    pub fn publish(&self, tx: Transaction) -> Result<OriginalOutcome> {
        self.publish_inner(tx, &mut |_| Ok(()), &mut || Ok(()))
    }
    /// Publishes while borrowing the caller's live selection guard. The guard
    /// runs before ownership, after outputs, immediately before native selector
    /// installation, and after selection. It is never stored. Replayed originals need no new
    /// selection and do not call it. A post-selection error remains uncertain
    /// until the original is resolved. The callback must not reenter this
    /// database: its native publication gate is held throughout.
    pub fn publish_guarded(
        &self,
        tx: Transaction,
        mut guard: impl FnMut() -> Result<()>,
    ) -> Result<OriginalOutcome> {
        self.publish_inner(tx, &mut |_| Ok(()), &mut guard)
    }
    #[cfg(feature = "test-support")]
    pub fn publish_with_boundary(
        &self,
        tx: Transaction,
        mut boundary: impl FnMut(PublicationBoundary) -> Result<()>,
    ) -> Result<OriginalOutcome> {
        self.publish_inner(tx, &mut boundary, &mut || Ok(()))
    }
    #[cfg(feature = "test-support")]
    pub fn publish_guarded_with_boundary(
        &self,
        tx: Transaction,
        mut guard: impl FnMut() -> Result<()>,
        mut boundary: impl FnMut(PublicationBoundary) -> Result<()>,
    ) -> Result<OriginalOutcome> {
        self.publish_inner(tx, &mut boundary, &mut guard)
    }
    fn build_branch(
        &self,
        prior: &Selected,
        changes: &BTreeMap<String, Option<Vec<u8>>>,
        original: Original,
        token: &str,
    ) -> Result<BuiltBranch> {
        let mut objects = Objects::new(&self.io, token);
        let mut index = DiskIndex::open(&mut objects, prior.version.root.clone())?;
        let mut version = prior.version.clone();
        version.sequence = version
            .sequence
            .checked_add(1)
            .ok_or("publication sequence overflow")?;
        version.indexed_originals = version
            .indexed_originals
            .checked_add(1)
            .ok_or("outcome count overflow")?;
        #[cfg(feature = "test-support")]
        let mut counts = BranchBuildCounts::default();
        let mut updates = Vec::new();
        for (key, value) in changes {
            let key = format!("d/{key}");
            let mut reused = None;
            if let Some(old) = index.get(&mut objects, &key)?.filter(|id| !id.is_empty()) {
                let id = std::str::from_utf8(&old)?;
                let bytes = objects.read(id)?;
                #[cfg(feature = "test-support")]
                if value.as_ref().is_some_and(|value| value == &bytes) {
                    counts.same_key_equal_values += 1;
                }
                version.logical_bytes = version
                    .logical_bytes
                    .checked_sub(bytes.len() as u64)
                    .ok_or("logical accounting")?;
                version.records = version.records.checked_sub(1).ok_or("record accounting")?;
                // Reuse only this key's authenticated value. Keep subtraction
                // and the addition below even when its payload is identical.
                if value.as_ref().is_some_and(|value| value == &bytes) {
                    reused = Some(old.clone());
                    #[cfg(feature = "test-support")]
                    { counts.reused_values += 1; }
                } else {
                    objects.retire(id);
                    #[cfg(feature = "test-support")]
                    { counts.value_retirements += 1; }
                }
            }
            let value = match value {
                Some(bytes) => {
                    version.records = version.records.checked_add(1).ok_or("record count")?;
                    version.logical_bytes = version
                        .logical_bytes
                        .checked_add(bytes.len() as u64)
                        .ok_or("logical bytes")?;
                    match reused {
                        Some(id) => id,
                        None => {
                            let id = objects.write(bytes)?.into_bytes();
                            #[cfg(feature = "test-support")]
                            { counts.value_allocations += 1; }
                            id
                        }
                    }
                }
                None => Vec::new(),
            };
            updates.push((key, value));
        }
        let original_bytes = encode(&original)?;
        version.logical_bytes = version
            .logical_bytes
            .checked_add(original_bytes.len() as u64)
            .ok_or("outcome bytes")?;
        updates.push((
            format!("o/{}", digest(original.key.as_bytes())),
            objects.write(&original_bytes)?.into_bytes(),
        ));
        if let Some(e) = &original.evidence {
            let key = format!("e/{}", digest(e.scope.as_bytes()));
            if let Some(old) = index.get(&mut objects, &key)?.filter(|id| !id.is_empty()) {
                let id = std::str::from_utf8(&old)?;
                let bytes = objects.read(id)?;
                version.logical_bytes = version
                    .logical_bytes
                    .checked_sub(bytes.len() as u64)
                    .ok_or("scope accounting")?;
                objects.retire(id);
            }
            let head = encode(&ScopeHead {
                scope: e.scope.clone(),
                reference: EvidenceReference {
                    key: original.key.clone(),
                    fingerprint: original.fingerprint.clone(),
                },
            })?;
            version.logical_bytes = version
                .logical_bytes
                .checked_add(head.len() as u64)
                .ok_or("scope bytes")?;
            updates.push((key, objects.write(&head)?.into_bytes()));
        }
        for chunk in updates.chunks(MAX_CHANGES) {
            index.put_batch(&mut objects, chunk.to_vec())?;
        }
        version.root = index.persist(&mut objects)?;
        let candidate = Selected {
            version,
            token: token.into(),
        };
        // New bytes remain in the pending map, so this equality holds even
        // before output. The complete exact checkpoint is also in the plan.
        let mut check = DiskIndex::open(&mut objects, candidate.version.root.clone())?;
        let id = check
            .get(
                &mut objects,
                &format!("o/{}", digest(original.key.as_bytes())),
            )?
            .ok_or("candidate original missing")?;
        if objects.read(std::str::from_utf8(&id)?)? != original_bytes {
            return Err("candidate original equality".into());
        }
        if let Some(e) = &original.evidence {
            let id = check
                .get(&mut objects, &format!("e/{}", digest(e.scope.as_bytes())))?
                .ok_or("candidate scope missing")?;
            let expected = encode(&ScopeHead {
                scope: e.scope.clone(),
                reference: EvidenceReference {
                    key: original.key.clone(),
                    fingerprint: original.fingerprint.clone(),
                },
            })?;
            if objects.read(std::str::from_utf8(&id)?)? != expected {
                return Err("candidate scope equality".into());
            }
        }
        let owned = objects.pending.keys().cloned().collect();
        let event = Event {
            sequence: candidate.version.sequence,
            token: token.into(),
            retired: objects.retired.iter().cloned().collect(),
        };
        #[cfg(feature = "test-support")]
        BRANCH_BUILD_COUNTS.with(|total| {
            let mut total = total.borrow_mut();
            if original.outcome.disposition == OriginalDisposition::NotSelected {
                total.negative_branches += 1;
            } else {
                total.positive_branches += 1;
            }
            total.same_key_equal_values += counts.same_key_equal_values;
            total.reused_values += counts.reused_values;
            total.value_allocations += counts.value_allocations;
            total.value_retirements += counts.value_retirements;
            total.pending_objects += objects.pending.len() as u64;
            total.pending_framed_bytes += objects.bytes as u64;
            total.retired_objects += objects.retired.len() as u64;
        });
        Ok(BuiltBranch {
            branch: Branch {
                candidate,
                original,
                owned,
                event,
            },
            pending: objects.pending,
        })
    }
    fn validate_plan(&self, p: &Plan) -> Result<()> {
        let a = &p.positive;
        let b = &p.negative;
        let seq = p
            .prior
            .version
            .sequence
            .checked_add(1)
            .ok_or("plan sequence")?;
        if a.candidate.version.sequence != seq
            || b.candidate.version.sequence != seq
            || a.candidate.version.store != self.binding.store
            || b.candidate.version.store != self.binding.store
            || p.prior.version.store != self.binding.store
            || a.original.key != b.original.key
            || a.original.fingerprint != b.original.fingerprint
            || !same_attempt(&a.original.evidence, &b.original.evidence)
            || b.original.outcome.disposition != OriginalDisposition::NotSelected
            || b.original.outcome.sequence != p.prior.version.sequence
            || !b.original.outcome.value.is_empty()
            || b.candidate.version.records != p.prior.version.records
            || a.owned.len() + b.owned.len() > 4096
            || a.event.retired.len() > 4096
            || b.event.retired.len() > 4096
        {
            return Err("closed publication plan differs".into());
        }
        for (branch, negative) in [(a, false), (b, true)] {
            if branch.event.sequence != seq || branch.event.token != branch.candidate.token {
                return Err("plan retirement binding".into());
            }
            for id in branch.owned.iter().chain(branch.event.retired.iter()) {
                objects::path(id)?;
            }
            if let Some(e) = &branch.original.evidence {
                self.validate_evidence(&branch.original, e)?;
                if e.binding.predecessor != p.prior.version
                    || e.binding.predecessor_token != p.prior.token
                    || e.binding.candidate.token != a.candidate.token
                    || e.binding.candidate.sequence != seq
                    || e.binding.resolving.token != branch.candidate.token
                {
                    return Err("plan checkpoint/candidate binding".into());
                }
            }
            if !negative
                && (!matches!(
                    branch.original.outcome.disposition,
                    OriginalDisposition::Selected | OriginalDisposition::Conflict
                ) || branch.original.outcome.sequence != seq)
            {
                return Err("positive outcome binding".into());
            }
        }
        let negative: std::collections::BTreeSet<_> = b.owned.iter().collect();
        if a.owned.iter().any(|id| negative.contains(id)) {
            return Err("alternative allocation alias".into());
        }
        Ok(())
    }
    fn publish_inner(
        &self,
        tx: Transaction,
        boundary: &mut dyn FnMut(PublicationBoundary) -> Result<()>,
        guard: &mut dyn FnMut() -> Result<()>,
    ) -> Result<OriginalOutcome> {
        if tx.key.is_empty()
            || tx.key.len() > 1024
            || tx.outcome.len() > MAX_OBJECT
            || tx.changes.len() > MAX_CHANGES
            || tx.evidence.as_ref().is_some_and(|e| {
                e.scope.is_empty() || e.scope.len() > 1000 || e.payload.len() > MAX_OBJECT
            })
        {
            return Err("publication bound".into());
        }
        let input = serde_json::to_vec(&tx)?;
        if input.len() > MAX_TRANSACTION_BYTES {
            return Err("publication input bytes".into());
        }
        let fingerprint = digest(&input);
        let gate = self.gate()?;
        self.recover_held(&gate)?;
        let prior = self.selected()?;
        if let Some(o) = self.original_at(&prior.version, &tx.key)? {
            if o.key != tx.key || o.fingerprint != fingerprint {
                return Err("original key content conflict".into());
            }
            return Ok(o.outcome);
        }
        let seq = prior
            .version
            .sequence
            .checked_add(1)
            .ok_or("publication sequence overflow")?;
        let token = nonce()?;
        let negative_token = nonce()?;
        let identity = CandidateIdentity {
            store: self.binding.store.clone(),
            token: token.clone(),
            sequence: seq,
        };
        let negative_identity = CandidateIdentity {
            store: self.binding.store.clone(),
            token: negative_token.clone(),
            sequence: seq,
        };
        let conflict = tx.expected != prior.version;
        let outcome = OriginalOutcome {
            disposition: if conflict {
                OriginalDisposition::Conflict
            } else {
                OriginalDisposition::Selected
            },
            sequence: seq,
            value: if conflict { Vec::new() } else { tx.outcome },
        };
        let evidence = if let Some(e) = tx.evidence {
            let previous = self.scope_head(&prior.version, &e.scope)?;
            Some(RetainedEvidence {
                scope: e.scope,
                payload: e.payload,
                previous,
                binding: EvidenceBinding {
                    format: self.binding.format.clone(),
                    namespace: self.binding.namespace.clone(),
                    store: self.binding.store.clone(),
                    key: tx.key.clone(),
                    fingerprint: fingerprint.clone(),
                    expected: tx.expected,
                    predecessor: prior.version.clone(),
                    predecessor_token: prior.token.clone(),
                    candidate: identity.clone(),
                    resolving: identity,
                },
            })
        } else {
            None
        };
        let original = Original {
            key: tx.key,
            fingerprint,
            outcome: outcome.clone(),
            evidence,
        };
        let mut negative_original = original.clone();
        negative_original.outcome = OriginalOutcome {
            disposition: OriginalDisposition::NotSelected,
            sequence: prior.version.sequence,
            value: Vec::new(),
        };
        if let Some(e) = &mut negative_original.evidence {
            e.binding.resolving = negative_identity;
        }
        let empty = BTreeMap::new();
        let positive = self.build_branch(
            &prior,
            if conflict { &empty } else { &tx.changes },
            original,
            &token,
        )?;
        let negative = self.build_branch(&prior, &empty, negative_original, &negative_token)?;
        let bytes: usize = positive
            .pending
            .values()
            .chain(negative.pending.values())
            .map(Vec::len)
            .sum();
        let retired: std::collections::BTreeSet<_> = positive
            .branch
            .event
            .retired
            .iter()
            .chain(negative.branch.event.retired.iter())
            .collect();
        if bytes > MAX_TRANSACTION_BYTES || retired.len() > 4096 {
            return Err("union publication admission bound".into());
        }
        let plan = Plan {
            prior,
            positive: positive.branch,
            negative: negative.branch,
        };
        self.validate_plan(&plan)?;
        let owned = encode(&ActiveCell::Owned(plan.clone()))?;
        boundary(PublicationBoundary::BeforeOwnership)?;
        guard()?;
        self.io.validate_lock(GATE, &gate)?;
        replace(&self.io, "active.json", &owned, || {
            self.io.validate_lock(GATE, &gate)
        })?;
        boundary(PublicationBoundary::Owned)?;
        for (id, bytes) in &positive.pending {
            exact(&self.io, &objects::path(id)?, bytes)?;
        }
        boundary(PublicationBoundary::Outputs)?;
        guard()?;
        self.io.validate_lock(GATE, &gate)?;
        boundary(PublicationBoundary::BeforeSelection)?;
        replace(
            &self.io,
            "SELECTED.json",
            &encode(&plan.positive.candidate)?,
            || {
                boundary(PublicationBoundary::SelectorDisplaced)?;
                guard()?;
                self.io.validate_lock(GATE, &gate)
            },
        )?;
        boundary(PublicationBoundary::Selected)?;
        guard()?;
        self.finish_selected(&plan, false, &gate)?;
        self.io.validate_lock(GATE, &gate)?;
        boundary(PublicationBoundary::Reply)?;
        Ok(outcome)
    }
    fn finish_selected(&self, p: &Plan, negative: bool, gate: &B::Guard) -> Result<()> {
        let branch = if negative { &p.negative } else { &p.positive };
        if !same_selected(&self.selected()?, &branch.candidate) {
            return Err("exact completed selector required".into());
        }
        match load::<ActiveCell>(&self.io, "active.json")? {
            ActiveCell::Owned(current) if encode(&current)? == encode(p)? => {}
            _ => return Err("exact original active owner required".into()),
        }
        let original = self
            .original_at(&branch.candidate.version, &branch.original.key)?
            .ok_or("selected original missing")?;
        if encode(&original)? != encode(&branch.original)? {
            return Err("selected original proof differs".into());
        }
        if let Some(e) = &original.evidence {
            self.validate_evidence(&original, e)?;
            let head = self
                .scope_head(&branch.candidate.version, &e.scope)?
                .ok_or("selected scope proof missing")?;
            if head.key != original.key || head.fingerprint != original.fingerprint {
                return Err("selected scope equality".into());
            }
        }
        if negative {
            // These exact plan-owned outputs never entered a selected root.
            // Reap partial writes here, not in hash-validated normal retirement.
            for id in &p.positive.owned {
                self.io.remove(&objects::path(id)?)?;
            }
        }
        atomic_exact(
            &self.io,
            &event_path(branch.event.sequence),
            &encode(&branch.event)?,
        )?;
        self.io.validate_lock(GATE, gate)?;
        replace(
            &self.io,
            "active.json",
            &encode(&ActiveCell::Idle(branch.candidate.clone()))?,
            || self.io.validate_lock(GATE, gate),
        )?;
        self.io.validate_lock(GATE, gate)
    }
    fn recover_held(&self, gate: &B::Guard) -> Result<()> {
        recover_replace(&self.io, "active.json")?;
        recover_replace(&self.io, "SELECTED.json")?;
        recover_replace(&self.io, "maintenance.json")?;
        let selected = self.selected()?;
        let cell: ActiveCell = load(&self.io, "active.json")?;
        let p = match cell {
            ActiveCell::Idle(idle) => {
                if !same_selected(&idle, &selected) {
                    return Err("idle selector proof differs".into());
                }
                return self.io.validate_lock(GATE, gate);
            }
            ActiveCell::Owned(p) => p,
        };
        self.validate_plan(&p)?;
        if same_selected(&selected, &p.positive.candidate) {
            return self.finish_selected(&p, false, gate);
        }
        if same_selected(&selected, &p.negative.candidate) {
            return self.finish_selected(&p, true, gate);
        }
        if !same_selected(&selected, &p.prior) {
            return Err("unresolved original ownership/selector".into());
        }
        let rebuilt = self.build_branch(
            &p.prior,
            &BTreeMap::new(),
            p.negative.original.clone(),
            &p.negative.candidate.token,
        )?;
        if encode(&rebuilt.branch)? != encode(&p.negative)? {
            return Err("negative allocation reconstruction differs".into());
        }
        for (id, bytes) in &rebuilt.pending {
            atomic_exact(&self.io, &objects::path(id)?, bytes)?;
        }
        self.io.validate_lock(GATE, gate)?;
        // Metadata-only root: never applies the unselected caller changes.
        replace(
            &self.io,
            "SELECTED.json",
            &encode(&p.negative.candidate)?,
            || self.io.validate_lock(GATE, gate),
        )?;
        self.finish_selected(&p, true, gate)
    }
    /// Restores interrupted control replacement or completes ownership transfer.
    /// An unselected caller attempt can select only its preadmitted metadata-only
    /// negative alternative, preserving every predecessor user byte and exact
    /// original nonapplication proof. It never installs the caller's user changes.
    pub fn recover(&self) -> Result<()> {
        let g = self.gate()?;
        self.recover_held(&g)?;
        self.io.validate_lock(GATE, &g)
    }
    /// Opens incomplete control state only for explicit storage recovery.
    pub fn open_for_recovery(io: B, namespace: &str) -> Result<Self> {
        io.validate()?;
        let binding: Binding = load(&io, "STORE.json")?;
        let ready: Ready = load(&io, "READY.json")?;
        if binding.format != "file-native@1"
            || binding.namespace != namespace
            || binding != ready.binding
            || ready.locks.len() != MAX_READERS + 1
            || ready.control_format != "atomic-control@1"
        {
            return Err("recovery binding differs".into());
        }
        Ok(Self { io, binding, ready })
    }
    pub fn maintain(&self, budget: MaintenanceBudget) -> Result<MaintenanceProgress> {
        if budget.items == 0
            || budget.items > 4096
            || budget.bytes < MAX_OBJECT as u64
            || budget.bytes > 1024 * 1024 * 1024
        {
            return Err("maintenance batch bound".into());
        }
        let g = self.gate()?;
        self.recover_held(&g)?;
        let selected = self.selected()?;
        let mut oldest = selected.version.sequence;
        for n in 0..MAX_READERS {
            match self.io.try_lock(&pin_lock(n))? {
                Some(guard) => {
                    self.reader_identity(n, &guard)?;
                }
                None => {
                    let v: Option<Version> = load(&self.io, &pin_path(n))?;
                    let v = v.ok_or("live reader has no positive pin")?;
                    if v.store != self.binding.store {
                        return Err("reader store differs".into());
                    }
                    oldest = oldest.min(v.sequence);
                }
            }
        }
        let mut cursor: Cursor = load(&self.io, "maintenance.json")?;
        let mut progress = MaintenanceProgress::default();
        while cursor.sequence <= selected.version.sequence
            && progress.visited < budget.items
            && progress.removed_bytes < budget.bytes
        {
            // Conservative sequence protection retains everything retired after
            // the oldest lease. It cannot skip a hole and needs no deferred set.
            if cursor.sequence > oldest {
                progress.blocked_by_reader = true;
                break;
            }
            let event: Event = load(&self.io, &event_path(cursor.sequence))?;
            if event.sequence != cursor.sequence
                || event.retired.len() > 4096
                || cursor.item > event.retired.len()
            {
                return Err("retirement coverage proof".into());
            }
            if cursor.item == event.retired.len() {
                cursor.sequence = cursor
                    .sequence
                    .checked_add(1)
                    .ok_or("maintenance sequence overflow")?;
                cursor.item = 0;
                progress.visited += 1;
                continue;
            }
            let path = objects::path(&event.retired[cursor.item])?;
            if let Some(b) = self.io.read(&path, MAX_OBJECT + 76)? {
                if digest(&b) != event.retired[cursor.item] {
                    return Err("retirement identity differs".into());
                }
                self.io.remove(&path)?;
                progress.removed_files += 1;
                progress.removed_bytes += b.len() as u64;
            }
            cursor.item += 1;
            progress.visited += 1;
        }
        replace(&self.io, "maintenance.json", &encode(&cursor)?, || Ok(()))?;
        progress.sequence = cursor.sequence;
        progress.item = cursor.item;
        progress.complete = cursor.sequence > selected.version.sequence;
        self.io.validate_lock(GATE, &g)?;
        Ok(progress)
    }
    pub fn statistics(&self) -> Result<Statistics> {
        let g = self.gate()?;
        let version = self.selected()?.version;
        let c: Cursor = load(&self.io, "maintenance.json")?;
        self.io.validate_lock(GATE, &g)?;
        Ok(Statistics {
            version,
            maintenance_sequence: c.sequence,
            directory_sync_supported: self.io.directory_sync_supported(),
        })
    }
}

#[derive(Clone, Debug)]
pub struct RecordPage {
    pub records: Vec<(String, Vec<u8>)>,
    pub next: Option<String>,
    pub complete: bool,
}
pub struct Snapshot<B: Backend> {
    db: Database<B>,
    version: Version,
    slot: usize,
    _guard: B::Guard,
}
impl<B: Backend> Snapshot<B> {
    pub fn version(&self) -> &Version {
        &self.version
    }
    pub fn get(&self, key: &str) -> Result<Option<Vec<u8>>> {
        self.db
            .io
            .validate_lock(&pin_lock(self.slot), &self._guard)?;
        let value = self.db.indexed(&self.version, &format!("d/{key}"))?;
        self.db
            .io
            .validate_lock(&pin_lock(self.slot), &self._guard)?;
        Ok(value)
    }
    pub fn page(&self, after: Option<&str>, limit: usize) -> Result<RecordPage> {
        if limit == 0 || limit > 256 {
            return Err("iterator page bound".into());
        }
        self.db
            .io
            .validate_lock(&pin_lock(self.slot), &self._guard)?;
        let mut objects = Objects::new(&self.db.io, "");
        let mut index = DiskIndex::open(&mut objects, self.version.root.clone())?;
        let after = after
            .map(|s| format!("d/{s}"))
            .unwrap_or_else(|| "d/".into());
        let rows = index.page(&mut objects, &after, limit)?;
        let mut complete = rows.len() < limit;
        let mut next = None;
        let mut records = Vec::new();
        for (key, id) in rows {
            if let Some(key) = key.strip_prefix("d/") {
                next = Some(key.to_owned());
                if !id.is_empty() {
                    records.push((key.into(), objects.read(std::str::from_utf8(&id)?)?));
                }
            } else {
                complete = true;
                break;
            }
        }
        self.db
            .io
            .validate_lock(&pin_lock(self.slot), &self._guard)?;
        Ok(RecordPage {
            records,
            next: if complete { None } else { next },
            complete,
        })
    }
}

impl<B: Backend> ReadView for Snapshot<B> {
    fn version(&self) -> &Version {
        Snapshot::version(self)
    }
    fn get(&self, key: &str) -> Result<Option<Vec<u8>>> {
        Snapshot::get(self, key)
    }
}
