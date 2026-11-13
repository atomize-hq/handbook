#![allow(
    dead_code,
    reason = "Packet 2 installs the private durable repository before Packet 3 consumes it"
)]

use crate::{
    DefinitionFingerprint, RepositoryDirectoryEntryKindV1, RepositoryDurabilityErrorKindV1,
    RepositoryDurabilityErrorV1, RepositoryReadV1, RetainedRepositoryRootV1,
    SnapshotBoundaryIdentityV1, SnapshotBoundaryKindV1, SnapshotGroundingOwnerV1,
    SnapshotRepositoryBindingV1, SnapshotStreamIdentityV1,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;
use std::sync::{Mutex, MutexGuard};

const STATE_ROOT: &str = ".handbook/state/snapshot-memory/v1";
const BINDING_PATH: &str = ".handbook/state/snapshot-memory/v1/repository-binding.json";
const TRANSACTION_ROOT: &str = ".handbook/state/snapshot-memory/v1/transactions";
const RECORD_ROOT: &str = ".handbook/state/snapshot-memory/v1/records";
const DERIVED_ROOT: &str = ".handbook/state/snapshot-memory/v1/derived";
const REPLAY_ROOT: &str = ".handbook/state/snapshot-memory/v1/derived/replay";
const AUDIT_ROOT: &str = ".handbook/state/snapshot-memory/v1/maintenance/audit";
const LOCK_ROOT: &str = ".handbook/state/locks";
const MAX_INPUT_BYTES: usize = 2 * 1024 * 1024;
const MAX_JOURNAL_BYTES: usize = 8 * 1024 * 1024;
const MAX_TRANSACTIONS: usize = 4096;
const MAX_RECORDS_PER_CLASS: usize = 4096;
const MAINTENANCE_BUDGET: usize = 64;
static PROCESS_SNAPSHOT_LOCK: Mutex<()> = Mutex::new(());

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SnapshotRepositoryErrorKindV1 {
    MalformedInput,
    RepositoryMismatch,
    SequenceConflict,
    IdempotencyConflict,
    RecordNotFound,
    CorruptStore,
    LockUnavailable,
    RecoveryFailed,
    DurabilityUncertain,
    MaintenanceDeferred,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SnapshotRepositoryErrorV1 {
    kind: SnapshotRepositoryErrorKindV1,
    detail: &'static str,
}

impl SnapshotRepositoryErrorV1 {
    pub fn kind(&self) -> SnapshotRepositoryErrorKindV1 {
        self.kind
    }

    pub fn detail(&self) -> &'static str {
        self.detail
    }
}

impl fmt::Display for SnapshotRepositoryErrorV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.detail)
    }
}

impl Error for SnapshotRepositoryErrorV1 {}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SnapshotRecordRefV1 {
    class: String,
    exact_ref: String,
    fingerprint: String,
}

impl SnapshotRecordRefV1 {
    pub fn class(&self) -> &str {
        &self.class
    }

    pub fn exact_ref(&self) -> &str {
        &self.exact_ref
    }

    pub fn fingerprint(&self) -> &str {
        &self.fingerprint
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "outcome", rename_all = "snake_case", deny_unknown_fields)]
pub enum SnapshotPredecessorV1 {
    NoApplicablePrior,
    ApplicablePrior {
        boundary_ref: String,
        boundary_fingerprint: String,
    },
}

pub struct SnapshotBoundaryCommitIntentV1 {
    idempotency_key: Vec<u8>,
    host_session_ref: String,
    kind: SnapshotBoundaryKindV1,
    accounting_bytes: Vec<u8>,
    delta_bytes: Option<Vec<u8>>,
    grounding_bytes: Vec<u8>,
    expected_sequence: Option<u64>,
    expected_predecessor: Option<SnapshotPredecessorV1>,
}

impl SnapshotBoundaryCommitIntentV1 {
    pub fn new(
        idempotency_key: impl AsRef<[u8]>,
        host_session_ref: impl Into<String>,
        kind: SnapshotBoundaryKindV1,
        accounting_bytes: Vec<u8>,
        delta_bytes: Option<Vec<u8>>,
        grounding_bytes: Vec<u8>,
    ) -> Self {
        Self {
            idempotency_key: idempotency_key.as_ref().to_vec(),
            host_session_ref: host_session_ref.into(),
            kind,
            accounting_bytes,
            delta_bytes,
            grounding_bytes,
            expected_sequence: None,
            expected_predecessor: None,
        }
    }

    pub(crate) fn with_owner_plan(
        mut self,
        sequence: u64,
        predecessor: SnapshotPredecessorV1,
    ) -> Self {
        self.expected_sequence = Some(sequence);
        self.expected_predecessor = Some(predecessor);
        self
    }
}

impl fmt::Debug for SnapshotBoundaryCommitIntentV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SnapshotBoundaryCommitIntentV1")
            .field("idempotency_key", &"<redacted>")
            .field("host_session_ref", &self.host_session_ref)
            .field("kind", &self.kind)
            .field("accounting_bytes", &self.accounting_bytes.len())
            .field("delta_bytes", &self.delta_bytes.as_ref().map(Vec::len))
            .field("grounding_bytes", &self.grounding_bytes.len())
            .finish()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SnapshotRepositoryHeadV1 {
    sequence: u64,
    kind: SnapshotBoundaryKindV1,
    host_session_ref: String,
    boundary_record: SnapshotRecordRefV1,
}

impl SnapshotRepositoryHeadV1 {
    pub fn sequence(&self) -> u64 {
        self.sequence
    }

    pub fn kind(&self) -> SnapshotBoundaryKindV1 {
        self.kind
    }

    pub fn host_session_ref(&self) -> &str {
        &self.host_session_ref
    }

    pub fn boundary_record(&self) -> &SnapshotRecordRefV1 {
        &self.boundary_record
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SnapshotRepositoryMaintenanceV1 {
    Complete {
        pruned_records: usize,
        active_holds: usize,
    },
    Deferred {
        pruned_records: usize,
        remaining_records: usize,
        active_holds: usize,
    },
}

#[cfg(any(test, feature = "test-support"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SnapshotRepositoryFaultPointV1 {
    AfterRecordsInstalled,
    AfterDerivedIndexes,
    AfterCommitMarker,
    AfterJournalFinalize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SnapshotRepositoryCommitV1 {
    sequence: u64,
    predecessor: SnapshotPredecessorV1,
    boundary_record: SnapshotRecordRefV1,
    accounting_record: SnapshotRecordRefV1,
    delta_record: Option<SnapshotRecordRefV1>,
    grounding_record: SnapshotRecordRefV1,
    receipt_record: SnapshotRecordRefV1,
    replayed: bool,
}

impl SnapshotRepositoryCommitV1 {
    pub fn sequence(&self) -> u64 {
        self.sequence
    }

    pub fn predecessor(&self) -> &SnapshotPredecessorV1 {
        &self.predecessor
    }

    pub fn boundary_record(&self) -> &SnapshotRecordRefV1 {
        &self.boundary_record
    }

    pub fn accounting_record(&self) -> &SnapshotRecordRefV1 {
        &self.accounting_record
    }

    pub fn delta_record(&self) -> Option<&SnapshotRecordRefV1> {
        self.delta_record.as_ref()
    }

    pub fn grounding_record(&self) -> &SnapshotRecordRefV1 {
        &self.grounding_record
    }

    pub fn receipt_record(&self) -> &SnapshotRecordRefV1 {
        &self.receipt_record
    }

    pub fn replayed(&self) -> bool {
        self.replayed
    }
}

pub struct SnapshotRepositoryV1<'binding> {
    owner: SnapshotGroundingOwnerV1,
    binding: &'binding SnapshotRepositoryBindingV1,
    stream: SnapshotStreamIdentityV1,
    maintenance: SnapshotRepositoryMaintenanceV1,
    active_holds: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum SnapshotRepositoryCapturePlanV1 {
    Replay(Box<SnapshotRepositoryCommitV1>),
    Fresh {
        sequence: u64,
        predecessor: SnapshotPredecessorV1,
        predecessor_grounding: Option<SnapshotRecordRefV1>,
    },
}

impl<'binding> SnapshotRepositoryV1<'binding> {
    pub(crate) fn read_record_binding(
        &self,
        class: &str,
        exact_ref: &str,
        fingerprint: &str,
    ) -> Result<Vec<u8>, SnapshotRepositoryErrorV1> {
        self.read_record(&SnapshotRecordRefV1 {
            class: class.to_owned(),
            exact_ref: exact_ref.to_owned(),
            fingerprint: fingerprint.to_owned(),
        })
    }

    pub(crate) fn open(
        owner: SnapshotGroundingOwnerV1,
        binding: &'binding SnapshotRepositoryBindingV1,
        stream: &SnapshotStreamIdentityV1,
    ) -> Result<Self, SnapshotRepositoryErrorV1> {
        validate_stream(binding, stream)?;
        let root = binding.retained_root();
        let _process_lock = acquire_process_lock()?;
        root.create_directories(LOCK_ROOT).map_err(map_durability)?;
        let lock = acquire_stream_lock(root, stream)?;
        ensure_layout(root)?;
        admit_repository_binding(root, binding)?;
        recover_locked(root)?;
        let history = load_history(root, &owner, binding, stream)?;
        rebuild_derived(root, stream, &history)?;
        let (maintenance, active_holds) = maintain_locked(root, stream, &history)?;
        lock.release().map_err(map_durability)?;
        Ok(Self {
            owner,
            binding,
            stream: stream.clone(),
            maintenance,
            active_holds,
        })
    }

    pub fn maintenance(&self) -> &SnapshotRepositoryMaintenanceV1 {
        &self.maintenance
    }

    pub fn active_holds(&self) -> &[String] {
        &self.active_holds
    }

    pub fn head(&self) -> Result<Option<SnapshotRepositoryHeadV1>, SnapshotRepositoryErrorV1> {
        let root = self.binding.retained_root();
        let _process_lock = acquire_process_lock()?;
        let lock = acquire_stream_lock(root, &self.stream)?;
        admit_repository_binding(root, self.binding)?;
        recover_locked(root)?;
        let history = load_history(root, &self.owner, self.binding, &self.stream)?;
        rebuild_derived(root, &self.stream, &history)?;
        let head = history.last().map(JournalIntent::head);
        lock.release().map_err(map_durability)?;
        Ok(head)
    }

    pub fn read_record(
        &self,
        record: &SnapshotRecordRefV1,
    ) -> Result<Vec<u8>, SnapshotRepositoryErrorV1> {
        let root = self.binding.retained_root();
        let _process_lock = acquire_process_lock()?;
        let lock = acquire_stream_lock(root, &self.stream)?;
        admit_repository_binding(root, self.binding)?;
        recover_locked(root)?;
        let history = load_history(root, &self.owner, self.binding, &self.stream)?;
        if !history.iter().any(|entry| {
            entry
                .records()
                .into_iter()
                .any(|candidate| candidate == record)
        }) {
            return Err(error(
                SnapshotRepositoryErrorKindV1::RecordNotFound,
                "snapshot record is not reachable from committed history",
            ));
        }
        let bytes = read_record_exact(root, record)?;
        lock.release().map_err(map_durability)?;
        Ok(bytes)
    }

    pub(crate) fn plan_capture(
        &self,
        idempotency_key: &[u8],
        host_session_ref: &str,
        kind: SnapshotBoundaryKindV1,
    ) -> Result<SnapshotRepositoryCapturePlanV1, SnapshotRepositoryErrorV1> {
        if idempotency_key.is_empty()
            || idempotency_key.len() > 256
            || host_session_ref.is_empty()
            || host_session_ref.len() > 256
            || host_session_ref.chars().any(char::is_control)
        {
            return Err(malformed());
        }
        let root = self.binding.retained_root();
        let _process_lock = acquire_process_lock()?;
        let lock = acquire_stream_lock(root, &self.stream)?;
        admit_repository_binding(root, self.binding)?;
        recover_locked(root)?;
        let history = load_history(root, &self.owner, self.binding, &self.stream)?;
        let key_digest = idempotency_digest(idempotency_key);
        let result =
            if let Some(existing) = history.iter().find(|entry| entry.key_digest == key_digest) {
                if existing.host_session_ref != host_session_ref || existing.kind()? != kind {
                    Err(error(
                        SnapshotRepositoryErrorKindV1::IdempotencyConflict,
                        "snapshot idempotency digest is already bound to another boundary request",
                    ))
                } else {
                    Ok(SnapshotRepositoryCapturePlanV1::Replay(Box::new(
                        existing.commit(true),
                    )))
                }
            } else {
                let (sequence, predecessor, predecessor_grounding) =
                    resolve_capture_plan(&history, host_session_ref, kind)?;
                Ok(SnapshotRepositoryCapturePlanV1::Fresh {
                    sequence,
                    predecessor,
                    predecessor_grounding,
                })
            };
        let released = lock.release().map_err(map_durability);
        match (result, released) {
            (Ok(plan), Ok(())) => Ok(plan),
            (Err(failure), Ok(())) => Err(failure),
            (_, Err(failure)) => Err(failure),
        }
    }

    pub fn commit_boundary(
        &self,
        intent: SnapshotBoundaryCommitIntentV1,
    ) -> Result<SnapshotRepositoryCommitV1, SnapshotRepositoryErrorV1> {
        let root = self.binding.retained_root();
        let _process_lock = acquire_process_lock()?;
        let lock = acquire_stream_lock(root, &self.stream)?;
        admit_repository_binding(root, self.binding)?;
        let result = self.commit_boundary_locked(root, intent, None);
        let released = lock.release().map_err(map_durability);
        match (result, released) {
            (Ok(commit), Ok(())) => Ok(commit),
            (Err(failure), Ok(())) => Err(failure),
            (_, Err(failure)) => Err(failure),
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn commit_boundary_with_fault_for_testing(
        &self,
        intent: SnapshotBoundaryCommitIntentV1,
        fault: SnapshotRepositoryFaultPointV1,
    ) -> Result<SnapshotRepositoryCommitV1, SnapshotRepositoryErrorV1> {
        let root = self.binding.retained_root();
        let _process_lock = acquire_process_lock()?;
        let lock = acquire_stream_lock(root, &self.stream)?;
        admit_repository_binding(root, self.binding)?;
        let result = self.commit_boundary_locked(root, intent, Some(fault));
        let released = lock.release().map_err(map_durability);
        match (result, released) {
            (Ok(commit), Ok(())) => Ok(commit),
            (Err(failure), Ok(())) => Err(failure),
            (_, Err(failure)) => Err(failure),
        }
    }

    fn commit_boundary_locked(
        &self,
        root: &RetainedRepositoryRootV1,
        intent: SnapshotBoundaryCommitIntentV1,
        #[cfg(any(test, feature = "test-support"))] fault: Option<SnapshotRepositoryFaultPointV1>,
        #[cfg(not(any(test, feature = "test-support")))] _fault: Option<()>,
    ) -> Result<SnapshotRepositoryCommitV1, SnapshotRepositoryErrorV1> {
        validate_intent(&intent)?;
        recover_locked(root)?;
        let history = load_history(root, &self.owner, self.binding, &self.stream)?;
        let key_digest = idempotency_digest(&intent.idempotency_key);
        let request_fingerprint = request_fingerprint(&intent)?;
        if let Some(existing) = history.iter().find(|entry| entry.key_digest == key_digest) {
            if existing.request_fingerprint != request_fingerprint {
                return Err(error(
                    SnapshotRepositoryErrorKindV1::IdempotencyConflict,
                    "snapshot idempotency digest is already bound to another canonical request",
                ));
            }
            return Ok(existing.commit(true));
        }
        if history.len() >= MAX_TRANSACTIONS {
            return Err(error(
                SnapshotRepositoryErrorKindV1::MaintenanceDeferred,
                "snapshot repository transaction bound requires retention maintenance",
            ));
        }

        let (sequence, predecessor) = resolve_next(&history, &intent)?;
        if intent
            .expected_sequence
            .is_some_and(|expected| expected != sequence)
            || intent
                .expected_predecessor
                .as_ref()
                .is_some_and(|expected| expected != &predecessor)
        {
            return Err(sequence_conflict());
        }
        let boundary_identity = self
            .owner
            .bind_boundary(
                self.binding,
                &self.stream,
                &intent.host_session_ref,
                intent.kind,
                sequence,
            )
            .map_err(|_| {
                error(
                    SnapshotRepositoryErrorKindV1::RepositoryMismatch,
                    "snapshot boundary does not match the retained owner binding",
                )
            })?;
        let mut staged = Vec::new();
        let accounting = record_from_input("accounting", &intent.accounting_bytes)?;
        staged.push(accounting.clone());
        let delta = intent
            .delta_bytes
            .as_deref()
            .map(|bytes| record_from_input("delta", bytes))
            .transpose()?;
        if let Some(record) = &delta {
            staged.push(record.clone());
        }
        let grounding = record_from_input("grounding", &intent.grounding_bytes)?;
        staged.push(grounding.clone());
        let boundary = boundary_record(
            &boundary_identity,
            &intent.host_session_ref,
            &predecessor,
            &accounting.reference,
            delta.as_ref().map(|record| &record.reference),
            &grounding.reference,
        )?;
        staged.push(boundary.clone());
        let receipt = receipt_record(
            self.binding,
            &self.stream,
            sequence,
            &predecessor,
            &boundary.reference,
            &accounting.reference,
            delta.as_ref().map(|record| &record.reference),
            &grounding.reference,
        )?;
        staged.push(receipt.clone());
        let replay = replay_record(&key_digest, &request_fingerprint, &receipt.reference)?;
        staged.push(replay.clone());

        let transaction_id =
            transaction_id(&self.stream, &key_digest, &request_fingerprint, sequence);
        let pending = format!("{TRANSACTION_ROOT}/{transaction_id}.pending");
        root.create_directory_new_durable(&pending)
            .map_err(map_durability)?;
        for (index, record) in staged.iter().enumerate() {
            root.write_new_durable(&format!("{pending}/record-{index}.new"), &record.bytes)
                .map_err(map_durability)?;
        }
        let journal = JournalIntent {
            schema: "handbook.snapshot-memory.transaction@1.0.0".to_owned(),
            stream_fingerprint: self.stream.fingerprint().to_owned(),
            key_digest,
            request_fingerprint,
            host_session_ref: intent.host_session_ref,
            kind: kind_name(intent.kind).to_owned(),
            sequence,
            predecessor,
            boundary: boundary.reference,
            accounting: accounting.reference,
            delta: delta.map(|record| record.reference),
            grounding: grounding.reference,
            receipt: receipt.reference,
            replay: replay.reference,
        };
        let journal_bytes =
            canonical_json(&serde_json::to_value(&journal).map_err(|_| malformed())?)?;
        root.write_new_durable(&format!("{pending}/intent.json"), &journal_bytes)
            .map_err(map_durability)?;
        for (index, record) in staged.iter().enumerate() {
            install_staged_record(root, &pending, index, record)?;
        }
        #[cfg(any(test, feature = "test-support"))]
        inject_fault(fault, SnapshotRepositoryFaultPointV1::AfterRecordsInstalled)?;
        let mut prospective = history;
        prospective.push(journal.clone());
        rebuild_derived(root, &self.stream, &prospective)?;
        #[cfg(any(test, feature = "test-support"))]
        inject_fault(fault, SnapshotRepositoryFaultPointV1::AfterDerivedIndexes)?;
        let marker = commit_marker(&journal_bytes);
        root.write_new_durable(&format!("{pending}/committed"), marker.as_bytes())
            .map_err(map_durability)?;
        #[cfg(any(test, feature = "test-support"))]
        inject_fault(fault, SnapshotRepositoryFaultPointV1::AfterCommitMarker)?;
        root.rename_directory_noreplace_durable(
            &pending,
            &format!("{TRANSACTION_ROOT}/{transaction_id}.committed"),
        )
        .map_err(map_durability)?;
        #[cfg(any(test, feature = "test-support"))]
        inject_fault(fault, SnapshotRepositoryFaultPointV1::AfterJournalFinalize)?;
        sync_holds_locked(root, &self.stream, &prospective)?;
        Ok(journal.commit(false))
    }
}

#[cfg(any(test, feature = "test-support"))]
fn inject_fault(
    selected: Option<SnapshotRepositoryFaultPointV1>,
    point: SnapshotRepositoryFaultPointV1,
) -> Result<(), SnapshotRepositoryErrorV1> {
    if selected == Some(point) {
        Err(error(
            SnapshotRepositoryErrorKindV1::DurabilityUncertain,
            "snapshot repository test fault makes durability uncertain",
        ))
    } else {
        Ok(())
    }
}

#[derive(Clone)]
struct StagedRecord {
    reference: SnapshotRecordRefV1,
    bytes: Vec<u8>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct JournalIntent {
    schema: String,
    stream_fingerprint: String,
    key_digest: String,
    request_fingerprint: String,
    host_session_ref: String,
    kind: String,
    sequence: u64,
    predecessor: SnapshotPredecessorV1,
    boundary: SnapshotRecordRefV1,
    accounting: SnapshotRecordRefV1,
    delta: Option<SnapshotRecordRefV1>,
    grounding: SnapshotRecordRefV1,
    receipt: SnapshotRecordRefV1,
    replay: SnapshotRecordRefV1,
}

impl JournalIntent {
    fn kind(&self) -> Result<SnapshotBoundaryKindV1, SnapshotRepositoryErrorV1> {
        match self.kind.as_str() {
            "start" => Ok(SnapshotBoundaryKindV1::Start),
            "end" => Ok(SnapshotBoundaryKindV1::End),
            _ => Err(corrupt()),
        }
    }

    fn records(&self) -> Vec<&SnapshotRecordRefV1> {
        let mut records = vec![
            &self.boundary,
            &self.accounting,
            &self.grounding,
            &self.receipt,
            &self.replay,
        ];
        if let Some(delta) = &self.delta {
            records.push(delta);
        }
        records
    }

    fn head(&self) -> SnapshotRepositoryHeadV1 {
        SnapshotRepositoryHeadV1 {
            sequence: self.sequence,
            kind: self.kind().expect("validated history has a closed kind"),
            host_session_ref: self.host_session_ref.clone(),
            boundary_record: self.boundary.clone(),
        }
    }

    fn commit(&self, replayed: bool) -> SnapshotRepositoryCommitV1 {
        SnapshotRepositoryCommitV1 {
            sequence: self.sequence,
            predecessor: self.predecessor.clone(),
            boundary_record: self.boundary.clone(),
            accounting_record: self.accounting.clone(),
            delta_record: self.delta.clone(),
            grounding_record: self.grounding.clone(),
            receipt_record: self.receipt.clone(),
            replayed,
        }
    }
}

fn ensure_layout(root: &RetainedRepositoryRootV1) -> Result<(), SnapshotRepositoryErrorV1> {
    for path in [
        STATE_ROOT,
        TRANSACTION_ROOT,
        RECORD_ROOT,
        DERIVED_ROOT,
        REPLAY_ROOT,
        AUDIT_ROOT,
        LOCK_ROOT,
    ] {
        root.create_directories(path).map_err(map_durability)?;
    }
    for class in [
        "accounting",
        "boundary",
        "delta",
        "grounding",
        "receipt",
        "replay",
    ] {
        root.create_directories(&format!("{RECORD_ROOT}/{class}"))
            .map_err(map_durability)?;
    }
    Ok(())
}

fn admit_repository_binding(
    root: &RetainedRepositoryRootV1,
    binding: &SnapshotRepositoryBindingV1,
) -> Result<(), SnapshotRepositoryErrorV1> {
    let expected = canonical_json(&json!({
        "engine_authority_fingerprint": binding.engine_authority_fingerprint(),
        "engine_authority_ref": binding.engine_authority_ref(),
        "policy_fingerprint": binding.policy_fingerprint(),
        "policy_ref": binding.policy_ref(),
        "repository_binding_fingerprint": binding.fingerprint(),
        "repository_format_ref": binding.format_ref(),
        "repository_state_root": binding.state_root(),
        "schema": "handbook.snapshot-memory.repository-binding@1.0.0",
        "workspace_fingerprint": binding.workspace_fingerprint(),
        "workspace_ref": binding.workspace_ref(),
    }))?;
    match root
        .read_regular_bounded(BINDING_PATH, MAX_JOURNAL_BYTES)
        .map_err(map_durability)?
    {
        RepositoryReadV1::Found(actual) if actual == expected => Ok(()),
        RepositoryReadV1::Found(_) => Err(error(
            SnapshotRepositoryErrorKindV1::RepositoryMismatch,
            "snapshot repository is bound to another exact owner identity",
        )),
        RepositoryReadV1::Absent => match root.write_new_durable(BINDING_PATH, &expected) {
            Ok(()) => Ok(()),
            Err(failure) if failure.kind() == RepositoryDurabilityErrorKindV1::AlreadyExists => {
                match root
                    .read_regular_bounded(BINDING_PATH, MAX_JOURNAL_BYTES)
                    .map_err(map_durability)?
                {
                    RepositoryReadV1::Found(actual) if actual == expected => Ok(()),
                    RepositoryReadV1::Found(_) => Err(error(
                        SnapshotRepositoryErrorKindV1::RepositoryMismatch,
                        "snapshot repository binding initialization conflicted",
                    )),
                    RepositoryReadV1::Absent => Err(error(
                        SnapshotRepositoryErrorKindV1::DurabilityUncertain,
                        "snapshot repository binding publication is uncertain",
                    )),
                }
            }
            Err(failure) => Err(map_durability(failure)),
        },
    }
}

fn acquire_process_lock() -> Result<MutexGuard<'static, ()>, SnapshotRepositoryErrorV1> {
    PROCESS_SNAPSHOT_LOCK.lock().map_err(|_| {
        error(
            SnapshotRepositoryErrorKindV1::LockUnavailable,
            "snapshot repository process lock is unavailable",
        )
    })
}

fn acquire_stream_lock(
    root: &RetainedRepositoryRootV1,
    stream: &SnapshotStreamIdentityV1,
) -> Result<crate::RepositoryExclusiveLockV1, SnapshotRepositoryErrorV1> {
    let path = format!(
        "{LOCK_ROOT}/snapshot-memory-{}.lock",
        fingerprint_hex(stream.fingerprint())?
    );
    match root.write_new_durable(&path, &[]) {
        Ok(()) => {}
        Err(failure) if failure.kind() == RepositoryDurabilityErrorKindV1::AlreadyExists => {
            let mut stable = false;
            for _ in 0..64 {
                match root.read_regular_bounded(&path, 0) {
                    Ok(RepositoryReadV1::Found(bytes)) if bytes.is_empty() => {
                        stable = true;
                        break;
                    }
                    Ok(RepositoryReadV1::Absent) => std::thread::yield_now(),
                    Err(failure)
                        if failure.kind() == RepositoryDurabilityErrorKindV1::UnsafeFilesystem =>
                    {
                        std::thread::yield_now();
                    }
                    Ok(RepositoryReadV1::Found(_)) => return Err(corrupt()),
                    Err(failure) => return Err(map_durability(failure)),
                }
            }
            if !stable {
                return Err(error(
                    SnapshotRepositoryErrorKindV1::LockUnavailable,
                    "snapshot repository stream lock initialization did not stabilize",
                ));
            }
        }
        Err(failure) => return Err(map_durability(failure)),
    }
    root.acquire_exclusive_lock(&path).map_err(map_durability)
}

fn validate_stream(
    binding: &SnapshotRepositoryBindingV1,
    stream: &SnapshotStreamIdentityV1,
) -> Result<(), SnapshotRepositoryErrorV1> {
    if stream.repository_binding_fingerprint() != binding.fingerprint()
        || stream.workspace_ref() != binding.workspace_ref()
        || stream.policy_fingerprint() != binding.policy_fingerprint()
    {
        return Err(error(
            SnapshotRepositoryErrorKindV1::RepositoryMismatch,
            "snapshot stream does not match the retained repository binding",
        ));
    }
    Ok(())
}

fn validate_intent(
    intent: &SnapshotBoundaryCommitIntentV1,
) -> Result<(), SnapshotRepositoryErrorV1> {
    if intent.idempotency_key.is_empty()
        || intent.idempotency_key.len() > 256
        || intent.host_session_ref.is_empty()
        || intent.host_session_ref.len() > 256
        || intent.host_session_ref.chars().any(char::is_control)
        || intent.accounting_bytes.is_empty()
        || intent.accounting_bytes.len() > MAX_INPUT_BYTES
        || intent.grounding_bytes.is_empty()
        || intent.grounding_bytes.len() > MAX_INPUT_BYTES
        || intent
            .delta_bytes
            .as_ref()
            .is_some_and(|bytes| bytes.is_empty() || bytes.len() > MAX_INPUT_BYTES)
    {
        return Err(malformed());
    }
    Ok(())
}

fn resolve_next(
    history: &[JournalIntent],
    intent: &SnapshotBoundaryCommitIntentV1,
) -> Result<(u64, SnapshotPredecessorV1), SnapshotRepositoryErrorV1> {
    let Some(head) = history.last() else {
        if intent.kind != SnapshotBoundaryKindV1::Start || intent.delta_bytes.is_some() {
            return Err(sequence_conflict());
        }
        return Ok((1, SnapshotPredecessorV1::NoApplicablePrior));
    };
    let head_kind = head.kind()?;
    match (head_kind, intent.kind) {
        (SnapshotBoundaryKindV1::Start, SnapshotBoundaryKindV1::End)
            if head.host_session_ref == intent.host_session_ref && intent.delta_bytes.is_some() => {
        }
        (SnapshotBoundaryKindV1::End, SnapshotBoundaryKindV1::Start)
            if intent.delta_bytes.is_some() => {}
        _ => return Err(sequence_conflict()),
    }
    let sequence = head.sequence.checked_add(1).ok_or_else(sequence_conflict)?;
    Ok((
        sequence,
        SnapshotPredecessorV1::ApplicablePrior {
            boundary_ref: head.boundary.exact_ref.clone(),
            boundary_fingerprint: head.boundary.fingerprint.clone(),
        },
    ))
}

fn resolve_capture_plan(
    history: &[JournalIntent],
    host_session_ref: &str,
    kind: SnapshotBoundaryKindV1,
) -> Result<(u64, SnapshotPredecessorV1, Option<SnapshotRecordRefV1>), SnapshotRepositoryErrorV1> {
    let Some(head) = history.last() else {
        if kind != SnapshotBoundaryKindV1::Start {
            return Err(sequence_conflict());
        }
        return Ok((1, SnapshotPredecessorV1::NoApplicablePrior, None));
    };
    match (head.kind()?, kind) {
        (SnapshotBoundaryKindV1::Start, SnapshotBoundaryKindV1::End)
            if head.host_session_ref == host_session_ref => {}
        (SnapshotBoundaryKindV1::End, SnapshotBoundaryKindV1::Start) => {}
        _ => return Err(sequence_conflict()),
    }
    let sequence = head.sequence.checked_add(1).ok_or_else(sequence_conflict)?;
    Ok((
        sequence,
        SnapshotPredecessorV1::ApplicablePrior {
            boundary_ref: head.boundary.exact_ref.clone(),
            boundary_fingerprint: head.boundary.fingerprint.clone(),
        },
        Some(head.grounding.clone()),
    ))
}

fn recover_locked(root: &RetainedRepositoryRootV1) -> Result<(), SnapshotRepositoryErrorV1> {
    for entry in root
        .list_directory_bounded(TRANSACTION_ROOT, MAX_TRANSACTIONS)
        .map_err(map_recovery)?
    {
        if entry.kind() != RepositoryDirectoryEntryKindV1::Directory {
            return Err(corrupt());
        }
        let name = entry.name();
        if name.ends_with(".committed") {
            continue;
        }
        if !name.ends_with(".pending") {
            return Err(corrupt());
        }
        let pending = format!("{TRANSACTION_ROOT}/{name}");
        let has_marker = matches!(
            root.read_regular_bounded(&format!("{pending}/committed"), 128)
                .map_err(map_recovery)?,
            RepositoryReadV1::Found(_)
        );
        if has_marker {
            let committed = format!(
                "{TRANSACTION_ROOT}/{}.committed",
                name.trim_end_matches(".pending")
            );
            root.rename_directory_noreplace_durable(&pending, &committed)
                .map_err(map_recovery)?;
        } else {
            remove_directory_tree(root, &pending)?;
        }
    }
    Ok(())
}

fn load_history(
    root: &RetainedRepositoryRootV1,
    owner: &SnapshotGroundingOwnerV1,
    binding: &SnapshotRepositoryBindingV1,
    stream: &SnapshotStreamIdentityV1,
) -> Result<Vec<JournalIntent>, SnapshotRepositoryErrorV1> {
    let mut history = Vec::new();
    let mut key_digests = BTreeSet::new();
    for entry in root
        .list_directory_bounded(TRANSACTION_ROOT, MAX_TRANSACTIONS)
        .map_err(map_recovery)?
    {
        if entry.kind() != RepositoryDirectoryEntryKindV1::Directory
            || !entry.name().ends_with(".committed")
        {
            return Err(corrupt());
        }
        let directory = format!("{TRANSACTION_ROOT}/{}", entry.name());
        let bytes = read_found(root, &format!("{directory}/intent.json"), MAX_JOURNAL_BYTES)?;
        let marker = read_found(root, &format!("{directory}/committed"), 128)?;
        if marker != commit_marker(&bytes).as_bytes() {
            return Err(corrupt());
        }
        let journal: JournalIntent = serde_json::from_slice(&bytes).map_err(|_| corrupt())?;
        if journal.schema != "handbook.snapshot-memory.transaction@1.0.0"
            || journal.stream_fingerprint != stream.fingerprint()
            || !key_digests.insert(journal.key_digest.clone())
        {
            return Err(corrupt());
        }
        for record in journal.records() {
            read_record_exact(root, record)?;
        }
        validate_journal_records(root, owner, binding, stream, &journal)?;
        history.push(journal);
    }
    history.sort_by_key(|entry| entry.sequence);
    validate_history(&history)?;
    Ok(history)
}

fn validate_journal_records(
    root: &RetainedRepositoryRootV1,
    owner: &SnapshotGroundingOwnerV1,
    binding: &SnapshotRepositoryBindingV1,
    stream: &SnapshotStreamIdentityV1,
    journal: &JournalIntent,
) -> Result<(), SnapshotRepositoryErrorV1> {
    let accounting_bytes = read_record_exact(root, &journal.accounting)?;
    let delta_bytes = journal
        .delta
        .as_ref()
        .map(|record| read_record_exact(root, record))
        .transpose()?;
    let grounding_bytes = read_record_exact(root, &journal.grounding)?;
    let expected_request = request_fingerprint_from_parts(
        &journal.host_session_ref,
        journal.kind()?,
        &accounting_bytes,
        delta_bytes.as_deref(),
        &grounding_bytes,
    )?;
    if expected_request != journal.request_fingerprint {
        return Err(corrupt());
    }

    let identity = owner
        .bind_boundary(
            binding,
            stream,
            &journal.host_session_ref,
            journal.kind()?,
            journal.sequence,
        )
        .map_err(|_| corrupt())?;
    let expected_boundary = boundary_record(
        &identity,
        &journal.host_session_ref,
        &journal.predecessor,
        &journal.accounting,
        journal.delta.as_ref(),
        &journal.grounding,
    )?;
    if expected_boundary.reference != journal.boundary
        || expected_boundary.bytes != read_record_exact(root, &journal.boundary)?
    {
        return Err(corrupt());
    }

    let expected_receipt = receipt_record(
        binding,
        stream,
        journal.sequence,
        &journal.predecessor,
        &journal.boundary,
        &journal.accounting,
        journal.delta.as_ref(),
        &journal.grounding,
    )?;
    if expected_receipt.reference != journal.receipt
        || expected_receipt.bytes != read_record_exact(root, &journal.receipt)?
    {
        return Err(corrupt());
    }

    let expected_replay = replay_record(
        &journal.key_digest,
        &journal.request_fingerprint,
        &journal.receipt,
    )?;
    if expected_replay.reference != journal.replay
        || expected_replay.bytes != read_record_exact(root, &journal.replay)?
    {
        return Err(corrupt());
    }
    Ok(())
}

fn validate_history(history: &[JournalIntent]) -> Result<(), SnapshotRepositoryErrorV1> {
    let mut prior: Option<&JournalIntent> = None;
    for (index, current) in history.iter().enumerate() {
        if current.sequence != index as u64 + 1 {
            return Err(corrupt());
        }
        let kind = current.kind()?;
        match prior {
            None => {
                if kind != SnapshotBoundaryKindV1::Start
                    || current.predecessor != SnapshotPredecessorV1::NoApplicablePrior
                    || current.delta.is_some()
                {
                    return Err(corrupt());
                }
            }
            Some(previous) => {
                let previous_kind = previous.kind()?;
                let alternates = matches!(
                    (previous_kind, kind),
                    (SnapshotBoundaryKindV1::Start, SnapshotBoundaryKindV1::End)
                        | (SnapshotBoundaryKindV1::End, SnapshotBoundaryKindV1::Start)
                );
                if !alternates
                    || current.delta.is_none()
                    || (kind == SnapshotBoundaryKindV1::End
                        && current.host_session_ref != previous.host_session_ref)
                    || current.predecessor
                        != (SnapshotPredecessorV1::ApplicablePrior {
                            boundary_ref: previous.boundary.exact_ref.clone(),
                            boundary_fingerprint: previous.boundary.fingerprint.clone(),
                        })
                {
                    return Err(corrupt());
                }
            }
        }
        prior = Some(current);
    }
    Ok(())
}

fn rebuild_derived(
    root: &RetainedRepositoryRootV1,
    stream: &SnapshotStreamIdentityV1,
    history: &[JournalIntent],
) -> Result<(), SnapshotRepositoryErrorV1> {
    replace_optional(
        root,
        &format!("{DERIVED_ROOT}/head.json"),
        history
            .last()
            .map(|head| {
                canonical_json(&json!({
                    "boundary": head.boundary,
                    "host_session_ref": head.host_session_ref,
                    "kind": head.kind,
                    "sequence": head.sequence,
                    "stream_fingerprint": stream.fingerprint(),
                }))
            })
            .transpose()?,
    )?;
    clear_regular_files(root, REPLAY_ROOT)?;
    for entry in history {
        let bytes = canonical_json(&json!({
            "key_digest": entry.key_digest,
            "receipt": entry.receipt,
            "request_fingerprint": entry.request_fingerprint,
            "sequence": entry.sequence,
            "stream_fingerprint": stream.fingerprint(),
        }))?;
        root.write_new_durable(
            &format!("{REPLAY_ROOT}/{}.json", fingerprint_hex(&entry.key_digest)?),
            &bytes,
        )
        .map_err(map_durability)?;
    }
    let compaction = canonical_json(&json!({
        "record_refs": history.iter().flat_map(JournalIntent::records).map(|record| record.exact_ref()).collect::<BTreeSet<_>>(),
        "schema": "handbook.snapshot-memory.compaction-index@1.0.0",
        "stream_fingerprint": stream.fingerprint(),
    }))?;
    replace_optional(
        root,
        &format!("{DERIVED_ROOT}/compaction.json"),
        Some(compaction),
    )
}

fn maintain_locked(
    root: &RetainedRepositoryRootV1,
    stream: &SnapshotStreamIdentityV1,
    history: &[JournalIntent],
) -> Result<(SnapshotRepositoryMaintenanceV1, Vec<String>), SnapshotRepositoryErrorV1> {
    let active_holds = sync_holds_locked(root, stream, history)?;
    let referenced = history
        .iter()
        .flat_map(JournalIntent::records)
        .map(|record| (record.class.clone(), fingerprint_hex(&record.fingerprint)))
        .map(|(class, fingerprint)| fingerprint.map(|fingerprint| (class, fingerprint)))
        .collect::<Result<BTreeSet<_>, _>>()?;
    let mut orphans = Vec::new();
    for class in [
        "accounting",
        "boundary",
        "delta",
        "grounding",
        "receipt",
        "replay",
    ] {
        let directory = format!("{RECORD_ROOT}/{class}");
        for entry in root
            .list_directory_bounded(&directory, MAX_RECORDS_PER_CLASS)
            .map_err(map_durability)?
        {
            if entry.kind() != RepositoryDirectoryEntryKindV1::RegularFile
                || !entry.name().ends_with(".json")
            {
                return Err(corrupt());
            }
            let fingerprint = entry.name().trim_end_matches(".json").to_owned();
            if !referenced.contains(&(class.to_owned(), fingerprint)) {
                orphans.push(format!("{directory}/{}", entry.name()));
            }
        }
    }
    orphans.sort();
    let mut pruned = 0;
    for path in orphans.iter().take(MAINTENANCE_BUDGET) {
        let bytes = read_found(root, path, MAX_INPUT_BYTES)?;
        let expected = path
            .rsplit('/')
            .next()
            .and_then(|name| name.strip_suffix(".json"))
            .ok_or_else(corrupt)?;
        let actual = fingerprint_hex(&DefinitionFingerprint::from_bytes(&bytes).to_string())?;
        if actual != expected {
            return Err(corrupt());
        }
        root.remove_regular_durable(path).map_err(map_durability)?;
        pruned += 1;
    }
    let maintenance = if orphans.len() > MAINTENANCE_BUDGET {
        SnapshotRepositoryMaintenanceV1::Deferred {
            pruned_records: pruned,
            remaining_records: orphans.len() - pruned,
            active_holds: active_holds.len(),
        }
    } else {
        SnapshotRepositoryMaintenanceV1::Complete {
            pruned_records: pruned,
            active_holds: active_holds.len(),
        }
    };
    Ok((maintenance, active_holds))
}

fn sync_holds_locked(
    root: &RetainedRepositoryRootV1,
    stream: &SnapshotStreamIdentityV1,
    history: &[JournalIntent],
) -> Result<Vec<String>, SnapshotRepositoryErrorV1> {
    let active_holds = derive_holds(history)?;
    let previous_holds = match root
        .read_regular_bounded(&format!("{DERIVED_ROOT}/holds.json"), MAX_JOURNAL_BYTES)
        .map_err(map_durability)?
    {
        RepositoryReadV1::Absent => Vec::new(),
        RepositoryReadV1::Found(bytes) => {
            let value: Value = serde_json::from_slice(&bytes).map_err(|_| corrupt())?;
            value
                .get("active_holds")
                .and_then(Value::as_array)
                .ok_or_else(corrupt)?
                .iter()
                .map(|value| value.as_str().map(ToOwned::to_owned).ok_or_else(corrupt))
                .collect::<Result<Vec<_>, _>>()?
        }
    };
    let released = previous_holds
        .iter()
        .filter(|hold| !active_holds.contains(hold))
        .cloned()
        .collect::<Vec<_>>();
    if previous_holds != active_holds || !released.is_empty() {
        let audit = canonical_json(&json!({
            "active_holds": active_holds,
            "head_boundary_ref": history.last().map(|entry| entry.boundary.exact_ref()),
            "head_sequence": history.last().map(|entry| entry.sequence),
            "released_holds": released,
            "schema": "handbook.snapshot-memory.maintenance-audit@1.0.0",
            "stream_fingerprint": stream.fingerprint(),
        }))?;
        let fingerprint = DefinitionFingerprint::from_bytes(&audit).to_string();
        let path = format!("{AUDIT_ROOT}/{}.json", fingerprint_hex(&fingerprint)?);
        match root
            .read_regular_bounded(&path, MAX_JOURNAL_BYTES)
            .map_err(map_durability)?
        {
            RepositoryReadV1::Absent => root
                .write_new_durable(&path, &audit)
                .map_err(map_durability)?,
            RepositoryReadV1::Found(existing) if existing == audit => {}
            RepositoryReadV1::Found(_) => return Err(corrupt()),
        }
    }
    let holds_bytes = canonical_json(&json!({
        "active_holds": active_holds,
        "schema": "handbook.snapshot-memory.implicit-holds@1.0.0",
        "stream_fingerprint": stream.fingerprint(),
    }))?;
    replace_optional(
        root,
        &format!("{DERIVED_ROOT}/holds.json"),
        Some(holds_bytes),
    )?;
    Ok(active_holds)
}

fn derive_holds(history: &[JournalIntent]) -> Result<Vec<String>, SnapshotRepositoryErrorV1> {
    let Some(head) = history.last() else {
        return Ok(Vec::new());
    };
    let mut holds = Vec::new();
    match head.kind()? {
        SnapshotBoundaryKindV1::Start => {
            holds.push(format!("open-session:{}", head.boundary.exact_ref))
        }
        SnapshotBoundaryKindV1::End => holds.push(format!(
            "applicable-predecessor:{}",
            head.boundary.exact_ref
        )),
    }
    if let Some(delta) = &head.delta {
        holds.push(format!("retained-derived:{}", delta.exact_ref));
    }
    holds.push(format!("retained-derived:{}", head.grounding.exact_ref));
    holds.sort();
    Ok(holds)
}

fn record_from_input(class: &str, bytes: &[u8]) -> Result<StagedRecord, SnapshotRepositoryErrorV1> {
    let value: Value = serde_json::from_slice(bytes).map_err(|_| malformed())?;
    let canonical = canonical_json(&value)?;
    Ok(staged_record(class, canonical))
}

fn boundary_record(
    identity: &SnapshotBoundaryIdentityV1,
    session_ref: &str,
    predecessor: &SnapshotPredecessorV1,
    accounting: &SnapshotRecordRefV1,
    delta: Option<&SnapshotRecordRefV1>,
    grounding: &SnapshotRecordRefV1,
) -> Result<StagedRecord, SnapshotRepositoryErrorV1> {
    let bytes = canonical_json(&json!({
        "accounting": accounting,
        "boundary_fingerprint": identity.fingerprint(),
        "boundary_kind": kind_name(identity.kind()),
        "delta": delta,
        "grounding": grounding,
        "host_session_ref": session_ref,
        "predecessor": predecessor,
        "schema": "handbook.snapshot-memory.boundary@1.0.0",
        "sequence": identity.sequence(),
        "stream_fingerprint": identity.stream_fingerprint(),
    }))?;
    Ok(staged_record("boundary", bytes))
}

#[allow(clippy::too_many_arguments)]
fn receipt_record(
    binding: &SnapshotRepositoryBindingV1,
    stream: &SnapshotStreamIdentityV1,
    sequence: u64,
    predecessor: &SnapshotPredecessorV1,
    boundary: &SnapshotRecordRefV1,
    accounting: &SnapshotRecordRefV1,
    delta: Option<&SnapshotRecordRefV1>,
    grounding: &SnapshotRecordRefV1,
) -> Result<StagedRecord, SnapshotRepositoryErrorV1> {
    let bytes = canonical_json(&json!({
        "accounting": accounting,
        "boundary": boundary,
        "delta": delta,
        "grounding": grounding,
        "policy_fingerprint": binding.policy_fingerprint(),
        "predecessor": predecessor,
        "repository_binding_fingerprint": binding.fingerprint(),
        "schema": "handbook.snapshot-memory.owner-receipt@1.0.0",
        "sequence": sequence,
        "stream_fingerprint": stream.fingerprint(),
    }))?;
    Ok(staged_record("receipt", bytes))
}

fn replay_record(
    key_digest: &str,
    request_fingerprint: &str,
    receipt: &SnapshotRecordRefV1,
) -> Result<StagedRecord, SnapshotRepositoryErrorV1> {
    let bytes = canonical_json(&json!({
        "idempotency_digest": key_digest,
        "receipt": receipt,
        "request_fingerprint": request_fingerprint,
        "schema": "handbook.snapshot-memory.replay@1.0.0",
    }))?;
    Ok(staged_record("replay", bytes))
}

fn staged_record(class: &str, bytes: Vec<u8>) -> StagedRecord {
    let fingerprint = DefinitionFingerprint::from_bytes(&bytes).to_string();
    StagedRecord {
        reference: SnapshotRecordRefV1 {
            class: class.to_owned(),
            exact_ref: format!("handbook.snapshot-memory.record.{class}@{fingerprint}"),
            fingerprint,
        },
        bytes,
    }
}

fn install_staged_record(
    root: &RetainedRepositoryRootV1,
    pending: &str,
    index: usize,
    record: &StagedRecord,
) -> Result<(), SnapshotRepositoryErrorV1> {
    let target = record_path(&record.reference)?;
    match root
        .read_regular_bounded(&target, MAX_INPUT_BYTES)
        .map_err(map_durability)?
    {
        RepositoryReadV1::Found(existing) if existing == record.bytes => root
            .remove_regular_durable(&format!("{pending}/record-{index}.new"))
            .map_err(map_durability),
        RepositoryReadV1::Found(_) => Err(corrupt()),
        RepositoryReadV1::Absent => root
            .rename_noreplace_durable(&format!("{pending}/record-{index}.new"), &target)
            .map_err(map_durability),
    }
}

fn read_record_exact(
    root: &RetainedRepositoryRootV1,
    record: &SnapshotRecordRefV1,
) -> Result<Vec<u8>, SnapshotRepositoryErrorV1> {
    if record.exact_ref
        != format!(
            "handbook.snapshot-memory.record.{}@{}",
            record.class, record.fingerprint
        )
    {
        return Err(corrupt());
    }
    let bytes = read_found(root, &record_path(record)?, MAX_INPUT_BYTES)?;
    if DefinitionFingerprint::from_bytes(&bytes).to_string() != record.fingerprint {
        return Err(corrupt());
    }
    Ok(bytes)
}

fn record_path(record: &SnapshotRecordRefV1) -> Result<String, SnapshotRepositoryErrorV1> {
    if !matches!(
        record.class.as_str(),
        "accounting" | "boundary" | "delta" | "grounding" | "receipt" | "replay"
    ) {
        return Err(corrupt());
    }
    Ok(format!(
        "{RECORD_ROOT}/{}/{}.json",
        record.class,
        fingerprint_hex(&record.fingerprint)?
    ))
}

fn request_fingerprint(
    intent: &SnapshotBoundaryCommitIntentV1,
) -> Result<String, SnapshotRepositoryErrorV1> {
    request_fingerprint_from_parts(
        &intent.host_session_ref,
        intent.kind,
        &intent.accounting_bytes,
        intent.delta_bytes.as_deref(),
        &intent.grounding_bytes,
    )
}

fn request_fingerprint_from_parts(
    host_session_ref: &str,
    kind: SnapshotBoundaryKindV1,
    accounting_bytes: &[u8],
    delta_bytes: Option<&[u8]>,
    grounding_bytes: &[u8],
) -> Result<String, SnapshotRepositoryErrorV1> {
    let value = json!({
        "accounting_fingerprint": DefinitionFingerprint::from_bytes(&canonical_input(accounting_bytes)?).to_string(),
        "boundary_kind": kind_name(kind),
        "delta_fingerprint": delta_bytes.map(canonical_input).transpose()?.as_deref().map(DefinitionFingerprint::from_bytes).map(|value| value.to_string()),
        "grounding_fingerprint": DefinitionFingerprint::from_bytes(&canonical_input(grounding_bytes)?).to_string(),
        "host_session_ref": host_session_ref,
        "schema": "handbook.snapshot-memory.canonical-request@1.0.0",
    });
    Ok(DefinitionFingerprint::from_bytes(&canonical_json(&value)?).to_string())
}

fn canonical_input(bytes: &[u8]) -> Result<Vec<u8>, SnapshotRepositoryErrorV1> {
    let value: Value = serde_json::from_slice(bytes).map_err(|_| malformed())?;
    canonical_json(&value)
}

fn idempotency_digest(key: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(b"handbook.snapshot-memory.idempotency-key@1.0\0");
    hasher.update(key);
    format!("sha256:{:x}", hasher.finalize())
}

fn transaction_id(
    stream: &SnapshotStreamIdentityV1,
    key_digest: &str,
    request_fingerprint: &str,
    sequence: u64,
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(b"handbook.snapshot-memory.transaction-id@1.0\0");
    hasher.update(stream.fingerprint().as_bytes());
    hasher.update(key_digest.as_bytes());
    hasher.update(request_fingerprint.as_bytes());
    hasher.update(sequence.to_be_bytes());
    format!("{:x}", hasher.finalize())
}

fn commit_marker(intent_bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(b"handbook.snapshot-memory.commit-marker@1.0\0");
    hasher.update(intent_bytes);
    format!("sha256:{:x}", hasher.finalize())
}

fn replace_optional(
    root: &RetainedRepositoryRootV1,
    path: &str,
    bytes: Option<Vec<u8>>,
) -> Result<(), SnapshotRepositoryErrorV1> {
    match root
        .read_regular_bounded(path, MAX_JOURNAL_BYTES)
        .map_err(map_durability)?
    {
        RepositoryReadV1::Absent => {
            if let Some(bytes) = bytes {
                root.write_new_durable(path, &bytes)
                    .map_err(map_durability)?;
            }
        }
        RepositoryReadV1::Found(existing) => match bytes {
            Some(bytes) if bytes == existing => {}
            Some(bytes) => {
                root.remove_regular_durable(path).map_err(map_durability)?;
                root.write_new_durable(path, &bytes)
                    .map_err(map_durability)?;
            }
            None => root.remove_regular_durable(path).map_err(map_durability)?,
        },
    }
    Ok(())
}

fn clear_regular_files(
    root: &RetainedRepositoryRootV1,
    directory: &str,
) -> Result<(), SnapshotRepositoryErrorV1> {
    for entry in root
        .list_directory_bounded(directory, MAX_TRANSACTIONS)
        .map_err(map_durability)?
    {
        if entry.kind() != RepositoryDirectoryEntryKindV1::RegularFile {
            return Err(corrupt());
        }
        root.remove_regular_durable(&format!("{directory}/{}", entry.name()))
            .map_err(map_durability)?;
    }
    Ok(())
}

fn remove_directory_tree(
    root: &RetainedRepositoryRootV1,
    directory: &str,
) -> Result<(), SnapshotRepositoryErrorV1> {
    for entry in root
        .list_directory_bounded(directory, 32)
        .map_err(map_recovery)?
    {
        let path = format!("{directory}/{}", entry.name());
        match entry.kind() {
            RepositoryDirectoryEntryKindV1::RegularFile => {
                root.remove_regular_durable(&path).map_err(map_recovery)?
            }
            RepositoryDirectoryEntryKindV1::Directory => return Err(corrupt()),
        }
    }
    root.remove_empty_directory_durable(directory)
        .map_err(map_recovery)
}

fn read_found(
    root: &RetainedRepositoryRootV1,
    path: &str,
    limit: usize,
) -> Result<Vec<u8>, SnapshotRepositoryErrorV1> {
    match root
        .read_regular_bounded(path, limit)
        .map_err(map_recovery)?
    {
        RepositoryReadV1::Found(bytes) => Ok(bytes),
        RepositoryReadV1::Absent => Err(corrupt()),
    }
}

fn canonical_json(value: &Value) -> Result<Vec<u8>, SnapshotRepositoryErrorV1> {
    serde_json_canonicalizer::to_vec(value).map_err(|_| malformed())
}

fn fingerprint_hex(fingerprint: &str) -> Result<String, SnapshotRepositoryErrorV1> {
    let Some(hex) = fingerprint.strip_prefix("sha256:") else {
        return Err(corrupt());
    };
    if hex.len() != 64 || !hex.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(corrupt());
    }
    Ok(hex.to_ascii_lowercase())
}

fn kind_name(kind: SnapshotBoundaryKindV1) -> &'static str {
    match kind {
        SnapshotBoundaryKindV1::Start => "start",
        SnapshotBoundaryKindV1::End => "end",
    }
}

fn map_durability(failure: RepositoryDurabilityErrorV1) -> SnapshotRepositoryErrorV1 {
    match failure.kind() {
        RepositoryDurabilityErrorKindV1::LockUnavailable => error(
            SnapshotRepositoryErrorKindV1::LockUnavailable,
            "snapshot repository stream lock is unavailable",
        ),
        RepositoryDurabilityErrorKindV1::DurabilityUncertain => error(
            SnapshotRepositoryErrorKindV1::DurabilityUncertain,
            "snapshot repository durability is uncertain",
        ),
        RepositoryDurabilityErrorKindV1::AlreadyExists => error(
            SnapshotRepositoryErrorKindV1::CorruptStore,
            "snapshot repository encountered conflicting existing state",
        ),
        RepositoryDurabilityErrorKindV1::NotFound => error(
            SnapshotRepositoryErrorKindV1::CorruptStore,
            "snapshot repository required state is absent",
        ),
        RepositoryDurabilityErrorKindV1::UnsafeFilesystem => error(
            SnapshotRepositoryErrorKindV1::CorruptStore,
            "snapshot repository filesystem identity is unsafe",
        ),
        _ => error(
            SnapshotRepositoryErrorKindV1::CorruptStore,
            "snapshot repository durability operation failed closed",
        ),
    }
}

fn map_recovery(failure: RepositoryDurabilityErrorV1) -> SnapshotRepositoryErrorV1 {
    let mapped = map_durability(failure);
    if mapped.kind == SnapshotRepositoryErrorKindV1::DurabilityUncertain {
        mapped
    } else {
        error(
            SnapshotRepositoryErrorKindV1::RecoveryFailed,
            "snapshot repository recovery could not establish exact committed state",
        )
    }
}

fn malformed() -> SnapshotRepositoryErrorV1 {
    error(
        SnapshotRepositoryErrorKindV1::MalformedInput,
        "snapshot repository input is malformed or exceeds its bound",
    )
}

fn sequence_conflict() -> SnapshotRepositoryErrorV1 {
    error(
        SnapshotRepositoryErrorKindV1::SequenceConflict,
        "snapshot boundary violates strict start/end stream sequence",
    )
}

fn corrupt() -> SnapshotRepositoryErrorV1 {
    error(
        SnapshotRepositoryErrorKindV1::CorruptStore,
        "snapshot repository contains conflicting or unverifiable state",
    )
}

fn error(kind: SnapshotRepositoryErrorKindV1, detail: &'static str) -> SnapshotRepositoryErrorV1 {
    SnapshotRepositoryErrorV1 { kind, detail }
}
