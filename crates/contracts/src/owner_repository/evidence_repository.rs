use super::*;
use crate::evidence_admission::{prepare_admitted_evidence_candidate_v1, PreparedEvidenceAppendV1};
use crate::{ContentAddressedBindingV1, EvidenceCandidateAdmissionErrorKindV1, EvidenceRecordV1};
use handbook_engine::{ContextResolutionStackDefinition, SourceCurrentnessPortV1};

const EVIDENCE_ROOT: &str = ".handbook/state/contracts/v1/evidence-v1";
const MAX_EVIDENCE_RECORDS: usize = 4096;

#[derive(Clone, Debug)]
pub struct EvidenceListRequestV1 {
    limit: usize,
    root: Option<FingerprintV1>,
    cursor: Option<String>,
}

impl EvidenceListRequestV1 {
    pub fn first(limit: usize) -> Result<Self, ContractOwnerRepositoryErrorV1> {
        if !(1..=MAX_PAGE_SIZE).contains(&limit) {
            return Err(bound("evidence page limit is outside the admitted bound"));
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
    ) -> Result<Self, ContractOwnerRepositoryErrorV1> {
        if !(1..=MAX_PAGE_SIZE).contains(&limit) {
            return Err(bound("evidence page limit is outside the admitted bound"));
        }
        let cursor = cursor.into();
        if cursor.len() > 4096 {
            return Err(bound("evidence cursor is outside the admitted bound"));
        }
        Ok(Self {
            limit,
            root: Some(root),
            cursor: Some(cursor),
        })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceListRetentionV1 {
    ImmutableAll,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct EvidenceListEntryV1 {
    sequence: u64,
    evidence_ref: String,
    evidence_fingerprint: FingerprintV1,
    evaluation_run: ContentAddressedBindingV1,
}

impl EvidenceListEntryV1 {
    pub fn sequence(&self) -> u64 {
        self.sequence
    }
    pub fn evidence_ref(&self) -> &str {
        &self.evidence_ref
    }
    pub fn evidence_fingerprint(&self) -> &FingerprintV1 {
        &self.evidence_fingerprint
    }
    pub fn evaluation_run(&self) -> &ContentAddressedBindingV1 {
        &self.evaluation_run
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceListPageV1 {
    root: FingerprintV1,
    entries: Vec<EvidenceListEntryV1>,
    next_cursor: Option<String>,
}

impl EvidenceListPageV1 {
    pub fn root(&self) -> &FingerprintV1 {
        &self.root
    }
    pub fn entries(&self) -> &[EvidenceListEntryV1] {
        &self.entries
    }
    pub fn next_cursor(&self) -> Option<&str> {
        self.next_cursor.as_deref()
    }
    pub fn retention(&self) -> EvidenceListRetentionV1 {
        EvidenceListRetentionV1::ImmutableAll
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct StoredEvidenceRecordV1 {
    record: EvidenceRecordV1,
    record_bytes: Vec<u8>,
}

impl StoredEvidenceRecordV1 {
    pub fn record(&self) -> &EvidenceRecordV1 {
        &self.record
    }
    pub fn record_bytes(&self) -> &[u8] {
        &self.record_bytes
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum EvidenceReadOutcomeV1 {
    Found(Box<StoredEvidenceRecordV1>),
    Absent,
}

#[derive(Clone, Debug)]
pub(crate) struct EvidenceEvaluationEntryV1 {
    pub(crate) sequence: u64,
    pub(crate) record: EvidenceRecordV1,
}

#[derive(Clone, Debug)]
pub(crate) struct EvidenceEvaluationSnapshotV1 {
    pub(crate) root: FingerprintV1,
    pub(crate) entries: Vec<EvidenceEvaluationEntryV1>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct EvidenceAppendReceiptV1 {
    schema_ref: String,
    sequence: u64,
    request_fingerprint: FingerprintV1,
    candidate: ContentAddressedBindingV1,
    evidence_ref: String,
    evidence_fingerprint: FingerprintV1,
    receipt_fingerprint: FingerprintV1,
}

impl EvidenceAppendReceiptV1 {
    pub fn sequence(&self) -> u64 {
        self.sequence
    }
    pub fn request_fingerprint(&self) -> &FingerprintV1 {
        &self.request_fingerprint
    }
    pub fn candidate(&self) -> &ContentAddressedBindingV1 {
        &self.candidate
    }
    pub fn evidence_ref(&self) -> &str {
        &self.evidence_ref
    }
    pub fn evidence_fingerprint(&self) -> &FingerprintV1 {
        &self.evidence_fingerprint
    }
    pub fn receipt_fingerprint(&self) -> &FingerprintV1 {
        &self.receipt_fingerprint
    }

    fn new(
        sequence: u64,
        request: &FingerprintV1,
        candidate: &ContentAddressedBindingV1,
        evidence: &EvidenceRecordV1,
    ) -> Result<Self, ContractOwnerRepositoryErrorV1> {
        let receipt_fingerprint = canonical_fingerprint(&EvidenceReceiptPreimage {
            schema_ref: "handbook.contract-evidence-append-receipt@1.0.0",
            sequence,
            request_fingerprint: request,
            candidate,
            evidence_ref: evidence.record_ref(),
            evidence_fingerprint: evidence.record_fingerprint(),
        })
        .map_err(|_| durability("evidence receipt identity failed"))?;
        Ok(Self {
            schema_ref: "handbook.contract-evidence-append-receipt@1.0.0".to_owned(),
            sequence,
            request_fingerprint: request.clone(),
            candidate: candidate.clone(),
            evidence_ref: evidence.record_ref().to_owned(),
            evidence_fingerprint: evidence.record_fingerprint().clone(),
            receipt_fingerprint,
        })
    }

    fn validate(&self) -> Result<(), ContractOwnerRepositoryErrorV1> {
        if self.schema_ref != "handbook.contract-evidence-append-receipt@1.0.0"
            || self.sequence == 0
            || self != &Self::new_from_parts(self)?
        {
            return Err(recovery("evidence receipt is substituted"));
        }
        Ok(())
    }

    fn new_from_parts(value: &Self) -> Result<Self, ContractOwnerRepositoryErrorV1> {
        let receipt_fingerprint = canonical_fingerprint(&EvidenceReceiptPreimage {
            schema_ref: "handbook.contract-evidence-append-receipt@1.0.0",
            sequence: value.sequence,
            request_fingerprint: &value.request_fingerprint,
            candidate: &value.candidate,
            evidence_ref: &value.evidence_ref,
            evidence_fingerprint: &value.evidence_fingerprint,
        })
        .map_err(|_| recovery("evidence receipt identity cannot be rederived"))?;
        Ok(Self {
            receipt_fingerprint,
            ..value.clone()
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct EvidenceAppendExecutionV1 {
    disposition: OwnerMutationDispositionV1,
    receipt: EvidenceAppendReceiptV1,
    record: EvidenceRecordV1,
}

impl EvidenceAppendExecutionV1 {
    pub fn disposition(&self) -> OwnerMutationDispositionV1 {
        self.disposition
    }
    pub fn receipt(&self) -> &EvidenceAppendReceiptV1 {
        &self.receipt
    }
    pub fn record(&self) -> &EvidenceRecordV1 {
        &self.record
    }
}

impl ContractOwnerRepositoryV1 {
    pub(crate) fn evidence_evaluation_snapshot_v1(
        &self,
    ) -> Result<EvidenceEvaluationSnapshotV1, ContractOwnerRepositoryErrorV1> {
        self.with_lock(|owner| {
            owner.recover_locked()?;
            owner.validate_inventory()?;
            let ledgers = owner.load_evidence_ledgers()?;
            let snapshot_sequence = ledgers.last().map_or(0, |ledger| ledger.receipt.sequence);
            let inventory = owner.evidence_entries(&ledgers, snapshot_sequence)?;
            let root = evidence_list_root(snapshot_sequence, &inventory)?;
            let entries = inventory
                .iter()
                .map(|entry| {
                    let stored = owner.load_evidence_record(
                        &owner.evidence_record_path(&entry.evidence_fingerprint),
                    )?;
                    if stored.record.record_ref() != entry.evidence_ref
                        || stored.record.record_fingerprint() != &entry.evidence_fingerprint
                        || stored.record.evaluation_run() != &entry.evaluation_run
                    {
                        return Err(recovery("evaluation snapshot entry is substituted"));
                    }
                    Ok(EvidenceEvaluationEntryV1 {
                        sequence: entry.sequence,
                        record: stored.record,
                    })
                })
                .collect::<Result<Vec<_>, ContractOwnerRepositoryErrorV1>>()?;
            Ok(EvidenceEvaluationSnapshotV1 { root, entries })
        })
    }

    pub fn list_evidence(
        &self,
        request: EvidenceListRequestV1,
    ) -> Result<EvidenceListPageV1, ContractOwnerRepositoryErrorV1> {
        self.with_lock(|owner| {
            owner.recover_locked()?;
            owner.validate_inventory()?;
            let ledgers = owner.load_evidence_ledgers()?;
            let current_sequence = ledgers.last().map_or(0, |ledger| ledger.receipt.sequence);
            let (snapshot_sequence, offset) = match (&request.root, &request.cursor) {
                (None, None) => (current_sequence, 0),
                (Some(root), Some(cursor)) => {
                    let decoded = decode_evidence_cursor(cursor, root, request.limit)?;
                    if decoded.snapshot_sequence > current_sequence {
                        return Err(substituted(
                            "evidence cursor snapshot is beyond the committed high-water",
                        ));
                    }
                    (decoded.snapshot_sequence, decoded.offset)
                }
                _ => return Err(invalid("evidence continuation requires root and cursor")),
            };
            let entries = owner.evidence_entries(&ledgers, snapshot_sequence)?;
            let root = evidence_list_root(snapshot_sequence, &entries)?;
            if request
                .root
                .as_ref()
                .is_some_and(|expected| expected != &root)
            {
                return Err(substituted("evidence list root is substituted"));
            }
            if offset > entries.len() {
                return Err(substituted(
                    "evidence cursor offset is outside its snapshot",
                ));
            }
            let end = offset.saturating_add(request.limit).min(entries.len());
            let next_cursor = (end < entries.len())
                .then(|| encode_evidence_cursor(&root, snapshot_sequence, end, request.limit))
                .transpose()?;
            Ok(EvidenceListPageV1 {
                root,
                entries: entries[offset..end].to_vec(),
                next_cursor,
            })
        })
    }

    pub fn read_evidence(
        &self,
        evidence_ref: &str,
        fingerprint: &FingerprintV1,
    ) -> Result<EvidenceReadOutcomeV1, ContractOwnerRepositoryErrorV1> {
        self.with_lock(|owner| {
            owner.recover_locked()?;
            owner.validate_inventory()?;
            let path = owner.evidence_record_path(fingerprint);
            if !owner.exists(&path)? {
                if owner
                    .load_evidence_ledgers()?
                    .iter()
                    .any(|ledger| ledger.receipt.evidence_ref == evidence_ref)
                {
                    return Err(substituted(
                        "exact evidence ref exists under another fingerprint",
                    ));
                }
                return Ok(EvidenceReadOutcomeV1::Absent);
            }
            let stored = owner.load_evidence_record(&path)?;
            if stored.record.record_ref() != evidence_ref
                || stored.record.record_fingerprint() != fingerprint
            {
                return Err(substituted("evidence identity is substituted"));
            }
            Ok(EvidenceReadOutcomeV1::Found(Box::new(stored)))
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn append_evidence_candidate_v1<S, C>(
        &self,
        source: &mut S,
        source_currentness: &mut C,
        resolution_stack: &ContextResolutionStackDefinition,
        policy_registry: &EvaluationPolicyRegistryV1,
        candidate: &ContentAddressedBindingV1,
        key: &str,
    ) -> Result<EvidenceAppendExecutionV1, ContractOwnerRepositoryErrorV1>
    where
        S: crate::EvidenceCandidateSourceV1,
        C: SourceCurrentnessPortV1,
    {
        validate_key(key)?;
        let request_fingerprint = evidence_request_fingerprint(candidate)?;
        let key_fingerprint = evidence_key_fingerprint(key)?;
        if let Some(replay) = self.with_lock(|owner| {
            owner.recover_locked()?;
            owner.validate_inventory()?;
            owner.replay_evidence(&key_fingerprint, &request_fingerprint)
        })? {
            return Ok(replay);
        }
        let prepared = prepare_admitted_evidence_candidate_v1(
            self,
            source,
            source_currentness,
            resolution_stack,
            policy_registry,
            candidate,
        )
        .map_err(map_candidate_admission)?;
        prepared
            .recheck_source_currentness(source_currentness)
            .map_err(map_candidate_admission)?;
        source
            .resolve_exact(candidate, |resolved| {
                prepared
                    .validate_final_resolved(&resolved)
                    .map_err(map_candidate_admission)?;
                resolved
                    .admission()
                    .require_repository_root(&self.repository_root)
                    .map_err(|_| {
                        unsafe_fs("final evidence authority belongs to another repository")
                    })?;
                self.with_lock(|owner| {
                    owner.recover_locked()?;
                    owner.validate_inventory()?;
                    if let Some(replay) =
                        owner.replay_evidence(&key_fingerprint, &request_fingerprint)?
                    {
                        return Ok(replay);
                    }
                    prepared
                        .validate_final_resolved(&resolved)
                        .map_err(map_candidate_admission)?;
                    owner.readmit_evaluation_run_locked_v1(prepared.run(), policy_registry)?;
                    owner.commit_evidence(prepared, candidate, request_fingerprint, key_fingerprint)
                })
            })
            .map_err(map_candidate_source)?
    }

    pub(super) fn ensure_evidence_layout(&self) -> Result<(), ContractOwnerRepositoryErrorV1> {
        for relative in [
            EVIDENCE_ROOT,
            ".handbook/state/contracts/v1/evidence-v1/records",
            ".handbook/state/contracts/v1/evidence-v1/idempotency",
            ".handbook/state/contracts/v1/evidence-v1/transactions",
        ] {
            self.repository_root
                .create_directories(relative)
                .map_err(map_durability)?;
        }
        Ok(())
    }

    pub(super) fn recover_evidence_locked(&self) -> Result<(), ContractOwnerRepositoryErrorV1> {
        let transactions =
            self.read_dir_bounded(&self.evidence_transactions_root(), MAX_EVIDENCE_RECORDS)?;
        let mut identities = BTreeSet::new();
        let mut pending = 0usize;
        for path in &transactions {
            self.reject_path(path, true)?;
            let name = file_name(path)?;
            let (identity, is_pending) = name
                .strip_suffix(".pending")
                .map(|v| (v, true))
                .or_else(|| name.strip_suffix(".committed").map(|v| (v, false)))
                .ok_or_else(|| recovery("evidence transaction inventory has an unknown entry"))?;
            if !identities.insert(identity.to_owned()) {
                return Err(recovery(
                    "evidence transaction identity has duplicate suffixes",
                ));
            }
            pending += usize::from(is_pending);
        }
        if pending > 1 {
            return Err(recovery(
                "more than one pending evidence transaction exists",
            ));
        }
        for path in &transactions {
            if file_name(path)?.ends_with(".committed") {
                self.validate_evidence_transaction(path)?;
            } else {
                self.preflight_evidence_pending(path)?;
            }
        }
        for path in transactions {
            if file_name(&path)?.ends_with(".pending") {
                self.recover_evidence_pending(&path)?;
            }
        }
        Ok(())
    }

    pub(super) fn validate_evidence_inventory(&self) -> Result<(), ContractOwnerRepositoryErrorV1> {
        let root_entries = self.read_dir_bounded(&self.evidence_root(), 4)?;
        let expected_children = BTreeSet::from([
            "idempotency".to_owned(),
            "records".to_owned(),
            "transactions".to_owned(),
        ]);
        let observed_children = root_entries
            .into_iter()
            .map(|path| {
                self.reject_path(&path, true)?;
                file_name(&path)
            })
            .collect::<Result<BTreeSet<_>, _>>()?;
        if observed_children != expected_children {
            return Err(recovery("evidence root inventory is not exact"));
        }

        let mut records = BTreeSet::new();
        let mut ledgers = BTreeSet::new();
        for transaction in
            self.read_dir_bounded(&self.evidence_transactions_root(), MAX_EVIDENCE_RECORDS)?
        {
            if !file_name(&transaction)?.ends_with(".committed") {
                return Err(recovery("pending evidence transaction remains"));
            }
            let intent = self.validate_evidence_transaction(&transaction)?;
            if !records.insert(intent.outputs[0].relative_ref.clone())
                || !ledgers.insert(intent.outputs[1].relative_ref.clone())
            {
                return Err(recovery(
                    "evidence output is claimed by multiple transactions",
                ));
            }
        }
        self.require_inventory(&self.evidence_records_root(), &records)?;
        self.require_inventory(&self.evidence_idempotency_root(), &ledgers)?;
        let loaded = self.load_evidence_ledgers()?;
        for (index, ledger) in loaded.iter().enumerate() {
            if ledger.receipt.sequence
                != u64::try_from(index + 1).map_err(|_| bound("evidence sequence overflow"))?
            {
                return Err(recovery("evidence sequences are not exact and contiguous"));
            }
            self.replay_evidence(&ledger.key_fingerprint, &ledger.receipt.request_fingerprint)?
                .ok_or_else(|| recovery("evidence replay ledger is not reachable"))?;
        }
        if loaded.iter().enumerate().any(|(i, left)| {
            loaded[..i].iter().any(|right| {
                right.receipt.candidate == left.receipt.candidate
                    || right.receipt.request_fingerprint == left.receipt.request_fingerprint
                    || right.receipt.evidence_ref == left.receipt.evidence_ref
            })
        }) {
            return Err(recovery("evidence semantic inventory is duplicated"));
        }
        Ok(())
    }

    fn commit_evidence(
        &self,
        prepared: PreparedEvidenceAppendV1,
        candidate: &ContentAddressedBindingV1,
        request_fingerprint: FingerprintV1,
        key_fingerprint: FingerprintV1,
    ) -> Result<EvidenceAppendExecutionV1, ContractOwnerRepositoryErrorV1> {
        let ledgers = self.load_evidence_ledgers()?;
        if ledgers.iter().any(|ledger| {
            ledger.receipt.candidate == *candidate
                || ledger.receipt.request_fingerprint == request_fingerprint
                || ledger.receipt.evidence_ref == prepared.record().record_ref()
        }) {
            return Err(repository_error(
                ContractOwnerRepositoryErrorKindV1::IdempotencyConflict,
                "evidence candidate or request is already bound",
            ));
        }
        if ledgers.len() >= MAX_EVIDENCE_RECORDS {
            return Err(bound("evidence inventory exceeds its admitted bound"));
        }
        let sequence =
            u64::try_from(ledgers.len() + 1).map_err(|_| bound("evidence sequence overflow"))?;
        let record_bytes = prepared
            .record()
            .canonical_json_bytes()
            .map_err(|_| substituted("admitted evidence cannot be canonicalized"))?;
        if record_bytes.len() > MAX_RECORD_BYTES {
            return Err(bound("evidence record is too large"));
        }
        let receipt = EvidenceAppendReceiptV1::new(
            sequence,
            &request_fingerprint,
            candidate,
            prepared.record(),
        )?;
        let ledger = EvidenceReplayLedger::new(&key_fingerprint, receipt.clone())?;
        let ledger_bytes =
            canonical_bytes(&ledger).map_err(|_| durability("evidence ledger encoding failed"))?;
        let transaction_id = fingerprint_hex(
            &canonical_fingerprint(&EvidenceTransactionIdPreimage {
                schema_ref: "handbook.contract-evidence-transaction-identity@1.0.0",
                key_fingerprint: &key_fingerprint,
                request_fingerprint: &request_fingerprint,
            })
            .map_err(|_| durability("evidence transaction identity failed"))?,
        )
        .to_owned();
        let pending = self
            .evidence_transactions_root()
            .join(format!("{transaction_id}.pending"));
        let committed = self
            .evidence_transactions_root()
            .join(format!("{transaction_id}.committed"));
        if self.exists(&pending)? || self.exists(&committed)? {
            return Err(recovery("evidence transaction identity is occupied"));
        }
        self.create_dir_durable(&pending)?;
        self.maybe_fault(OwnerFaultBoundary::PendingDirectory)?;
        let record_ref = self.evidence_record_ref(prepared.record().record_fingerprint());
        let ledger_ref = self.evidence_ledger_ref(&key_fingerprint);
        let outputs = vec![
            StagedOutput::new("00-evidence.json", &record_ref, &record_bytes)?,
            StagedOutput::new("01-ledger.json", &ledger_ref, &ledger_bytes)?,
        ];
        for (index, (output, bytes)) in outputs
            .iter()
            .zip([record_bytes.as_slice(), ledger_bytes.as_slice()])
            .enumerate()
        {
            self.write_new_durable(&pending.join(&output.staged_name), bytes)?;
            self.maybe_fault(if index == 0 {
                OwnerFaultBoundary::StagedPrimary
            } else {
                OwnerFaultBoundary::StagedLedger
            })?;
        }
        let intent = EvidenceTransactionIntent::new(
            transaction_id,
            &key_fingerprint,
            &request_fingerprint,
            outputs,
        )?;
        self.write_new_durable(
            &pending.join("intent.json"),
            &canonical_bytes(&intent).map_err(|_| durability("evidence intent encoding failed"))?,
        )?;
        self.maybe_fault(OwnerFaultBoundary::Intent)?;
        let marker = EvidenceCommitMarker::new(&intent)?;
        self.write_new_durable(
            &pending.join("commit-marker.json"),
            &canonical_bytes(&marker).map_err(|_| durability("evidence marker encoding failed"))?,
        )?;
        self.maybe_fault(OwnerFaultBoundary::Marker)?;
        self.finish_evidence_transaction(&pending, &intent, &marker)?;
        self.rename_dir_durable(&pending, &committed)?;
        self.maybe_fault(OwnerFaultBoundary::CommittedRename)?;
        self.validate_evidence_inventory()?;
        Ok(EvidenceAppendExecutionV1 {
            disposition: OwnerMutationDispositionV1::Committed,
            receipt,
            record: prepared.into_record(),
        })
    }

    fn replay_evidence(
        &self,
        key: &FingerprintV1,
        request: &FingerprintV1,
    ) -> Result<Option<EvidenceAppendExecutionV1>, ContractOwnerRepositoryErrorV1> {
        let path = self.evidence_ledger_path(key);
        if !self.exists(&path)? {
            return Ok(None);
        }
        let ledger: EvidenceReplayLedger = parse_canonical(&self.read_regular(&path)?)?;
        ledger.validate()?;
        if ledger.key_fingerprint != *key || ledger.receipt.request_fingerprint != *request {
            return Err(repository_error(
                ContractOwnerRepositoryErrorKindV1::IdempotencyConflict,
                "evidence idempotency key is bound to another request",
            ));
        }
        let stored = self.load_evidence_record(
            &self.evidence_record_path(&ledger.receipt.evidence_fingerprint),
        )?;
        if stored.record.record_ref() != ledger.receipt.evidence_ref
            || stored.record.admitted_candidate() != &ledger.receipt.candidate
        {
            return Err(recovery("evidence replay closure is substituted"));
        }
        Ok(Some(EvidenceAppendExecutionV1 {
            disposition: OwnerMutationDispositionV1::Replayed,
            receipt: ledger.receipt,
            record: stored.record,
        }))
    }

    fn load_evidence_ledgers(
        &self,
    ) -> Result<Vec<EvidenceReplayLedger>, ContractOwnerRepositoryErrorV1> {
        let mut values = self
            .read_dir_bounded(&self.evidence_idempotency_root(), MAX_EVIDENCE_RECORDS)?
            .into_iter()
            .map(|path| {
                if path.extension().and_then(|v| v.to_str()) != Some("json") {
                    return Err(recovery("evidence ledger inventory has an unknown entry"));
                }
                let ledger: EvidenceReplayLedger = parse_canonical(&self.read_regular(&path)?)?;
                if file_stem(&path)? != fingerprint_hex(&ledger.key_fingerprint) {
                    return Err(substituted("evidence ledger filename is substituted"));
                }
                ledger.validate()?;
                Ok(ledger)
            })
            .collect::<Result<Vec<_>, _>>()?;
        values.sort_by_key(|ledger| ledger.receipt.sequence);
        Ok(values)
    }

    fn evidence_entries(
        &self,
        ledgers: &[EvidenceReplayLedger],
        snapshot: u64,
    ) -> Result<Vec<EvidenceListEntryV1>, ContractOwnerRepositoryErrorV1> {
        ledgers
            .iter()
            .filter(|ledger| ledger.receipt.sequence <= snapshot)
            .map(|ledger| {
                let stored = self.load_evidence_record(
                    &self.evidence_record_path(&ledger.receipt.evidence_fingerprint),
                )?;
                Ok(EvidenceListEntryV1 {
                    sequence: ledger.receipt.sequence,
                    evidence_ref: ledger.receipt.evidence_ref.clone(),
                    evidence_fingerprint: ledger.receipt.evidence_fingerprint.clone(),
                    evaluation_run: stored.record.evaluation_run().clone(),
                })
            })
            .collect()
    }

    fn load_evidence_record(
        &self,
        path: &Path,
    ) -> Result<StoredEvidenceRecordV1, ContractOwnerRepositoryErrorV1> {
        let bytes = self.read_regular(path)?;
        let record: EvidenceRecordV1 = parse_canonical(&bytes)?;
        if file_stem(path)? != fingerprint_hex(record.record_fingerprint()) {
            return Err(substituted("evidence filename is substituted"));
        }
        Ok(StoredEvidenceRecordV1 {
            record,
            record_bytes: bytes,
        })
    }

    fn validate_evidence_transaction(
        &self,
        path: &Path,
    ) -> Result<EvidenceTransactionIntent, ContractOwnerRepositoryErrorV1> {
        let intent: EvidenceTransactionIntent =
            parse_canonical(&self.read_regular(&path.join("intent.json"))?)?;
        intent.validate()?;
        self.require_evidence_transaction_inventory(path, &intent, true)?;
        if file_name(path)?.strip_suffix(".committed") != Some(intent.transaction_id.as_str()) {
            return Err(recovery(
                "evidence committed transaction identity is substituted",
            ));
        }
        let marker: EvidenceCommitMarker =
            parse_canonical(&self.read_regular(&path.join("commit-marker.json"))?)?;
        marker.validate(&intent)?;
        let record: EvidenceRecordV1 =
            parse_canonical(&self.read_regular(&path.join("00-evidence.json"))?)?;
        let ledger: EvidenceReplayLedger =
            parse_canonical(&self.read_regular(&path.join("01-ledger.json"))?)?;
        ledger.validate()?;
        if ledger.key_fingerprint != intent.key_fingerprint
            || ledger.receipt.request_fingerprint != intent.request_fingerprint
            || ledger.receipt.evidence_ref != record.record_ref()
            || ledger.receipt.evidence_fingerprint != *record.record_fingerprint()
            || ledger.receipt.candidate != *record.admitted_candidate()
        {
            return Err(recovery("evidence transaction closure is substituted"));
        }
        for output in &intent.outputs {
            let staged = self.read_regular(&path.join(&output.staged_name))?;
            if FingerprintV1::from_bytes(&staged) != output.bytes_fingerprint
                || staged != self.read_regular(&self.repo_root.join(&output.relative_ref))?
            {
                return Err(recovery(
                    "evidence committed output is not exactly reachable",
                ));
            }
        }
        Ok(intent)
    }

    fn preflight_evidence_pending(
        &self,
        path: &Path,
    ) -> Result<(), ContractOwnerRepositoryErrorV1> {
        let intent_path = path.join("intent.json");
        let marker_path = path.join("commit-marker.json");
        if !self.exists(&intent_path)? {
            if self.exists(&marker_path)? {
                return Err(recovery("evidence marker exists without intent"));
            }
            for entry in self.read_dir_bounded(path, 6)? {
                let name = file_name(&entry)?;
                if !matches!(name.as_str(), "00-evidence.json" | "01-ledger.json") {
                    return Err(recovery(
                        "unestablished evidence transaction has unknown content",
                    ));
                }
                self.reject_path(&entry, false)?;
            }
            return Ok(());
        }
        let intent: EvidenceTransactionIntent = parse_canonical(&self.read_regular(&intent_path)?)?;
        intent.validate()?;
        if file_name(path)?.strip_suffix(".pending") != Some(intent.transaction_id.as_str()) {
            return Err(recovery(
                "pending evidence transaction identity is substituted",
            ));
        }
        let marker_present = self.exists(&marker_path)?;
        self.require_evidence_transaction_inventory(path, &intent, marker_present)?;
        if marker_present {
            let marker: EvidenceCommitMarker = parse_canonical(&self.read_regular(&marker_path)?)?;
            marker.validate(&intent)?;
        } else if self.evidence_installed_prefix(&intent)? != 0 {
            return Err(recovery("markerless evidence transaction installed output"));
        }
        Ok(())
    }

    fn recover_evidence_pending(&self, path: &Path) -> Result<(), ContractOwnerRepositoryErrorV1> {
        let intent_path = path.join("intent.json");
        let marker_path = path.join("commit-marker.json");
        if !self.exists(&intent_path)? {
            self.remove_pending(path)?;
            return Ok(());
        }
        let intent: EvidenceTransactionIntent = parse_canonical(&self.read_regular(&intent_path)?)?;
        intent.validate()?;
        self.require_evidence_transaction_inventory(path, &intent, self.exists(&marker_path)?)?;
        if !self.exists(&marker_path)? {
            self.remove_pending(path)?;
            return Ok(());
        }
        let marker: EvidenceCommitMarker = parse_canonical(&self.read_regular(&marker_path)?)?;
        self.finish_evidence_transaction(path, &intent, &marker)?;
        let committed = path.with_file_name(format!("{}.committed", intent.transaction_id));
        self.rename_dir_durable(path, &committed)
    }

    fn finish_evidence_transaction(
        &self,
        path: &Path,
        intent: &EvidenceTransactionIntent,
        marker: &EvidenceCommitMarker,
    ) -> Result<(), ContractOwnerRepositoryErrorV1> {
        marker.validate(intent)?;
        for (index, output) in intent.outputs.iter().enumerate() {
            let bytes = self.read_regular(&path.join(&output.staged_name))?;
            if FingerprintV1::from_bytes(&bytes) != output.bytes_fingerprint {
                return Err(recovery("staged evidence output is substituted"));
            }
            let final_path = self.repo_root.join(&output.relative_ref);
            if self.exists(&final_path)? {
                if self.read_regular(&final_path)? != bytes {
                    return Err(recovery("immutable evidence output is unequal"));
                }
            } else {
                self.write_new_durable(&final_path, &bytes)?;
            }
            self.maybe_fault(if index == 0 {
                OwnerFaultBoundary::PrimaryInstalled
            } else {
                OwnerFaultBoundary::LedgerInstalled
            })?;
        }
        Ok(())
    }

    fn evidence_installed_prefix(
        &self,
        intent: &EvidenceTransactionIntent,
    ) -> Result<usize, ContractOwnerRepositoryErrorV1> {
        let mut prefix = 0;
        let mut missing = false;
        for output in &intent.outputs {
            let path = self.repo_root.join(&output.relative_ref);
            let installed = self.exists(&path)?;
            if installed
                && FingerprintV1::from_bytes(&self.read_regular(&path)?) != output.bytes_fingerprint
            {
                return Err(recovery("installed evidence output is unequal"));
            }
            if installed {
                if missing {
                    return Err(recovery("installed evidence outputs are not a prefix"));
                }
                prefix += 1;
            } else {
                missing = true;
            }
        }
        Ok(prefix)
    }

    fn require_evidence_transaction_inventory(
        &self,
        path: &Path,
        intent: &EvidenceTransactionIntent,
        marker: bool,
    ) -> Result<(), ContractOwnerRepositoryErrorV1> {
        let mut expected = intent
            .outputs
            .iter()
            .map(|output| output.staged_name.clone())
            .collect::<BTreeSet<_>>();
        expected.insert("intent.json".to_owned());
        if marker {
            expected.insert("commit-marker.json".to_owned());
        }
        let observed = self
            .read_dir_bounded(path, 6)?
            .into_iter()
            .map(|entry| {
                self.reject_path(&entry, false)?;
                file_name(&entry)
            })
            .collect::<Result<BTreeSet<_>, _>>()?;
        if observed != expected {
            return Err(recovery("evidence transaction inventory is not exact"));
        }
        Ok(())
    }

    fn evidence_root(&self) -> PathBuf {
        self.repo_root.join(EVIDENCE_ROOT)
    }
    fn evidence_records_root(&self) -> PathBuf {
        self.evidence_root().join("records")
    }
    fn evidence_idempotency_root(&self) -> PathBuf {
        self.evidence_root().join("idempotency")
    }
    fn evidence_transactions_root(&self) -> PathBuf {
        self.evidence_root().join("transactions")
    }
    fn evidence_record_ref(&self, fp: &FingerprintV1) -> String {
        format!("{EVIDENCE_ROOT}/records/{}.json", fingerprint_hex(fp))
    }
    fn evidence_ledger_ref(&self, fp: &FingerprintV1) -> String {
        format!("{EVIDENCE_ROOT}/idempotency/{}.json", fingerprint_hex(fp))
    }
    fn evidence_record_path(&self, fp: &FingerprintV1) -> PathBuf {
        self.repo_root.join(self.evidence_record_ref(fp))
    }
    fn evidence_ledger_path(&self, fp: &FingerprintV1) -> PathBuf {
        self.repo_root.join(self.evidence_ledger_ref(fp))
    }
}

#[derive(Serialize)]
struct EvidenceRequestPreimage<'a> {
    schema_ref: &'static str,
    candidate: &'a ContentAddressedBindingV1,
}
#[derive(Serialize)]
struct EvidenceKeyPreimage<'a> {
    schema_ref: &'static str,
    key: &'a str,
}
#[derive(Serialize)]
struct EvidenceReceiptPreimage<'a> {
    schema_ref: &'static str,
    sequence: u64,
    request_fingerprint: &'a FingerprintV1,
    candidate: &'a ContentAddressedBindingV1,
    evidence_ref: &'a str,
    evidence_fingerprint: &'a FingerprintV1,
}
#[derive(Serialize)]
struct EvidenceTransactionIdPreimage<'a> {
    schema_ref: &'static str,
    key_fingerprint: &'a FingerprintV1,
    request_fingerprint: &'a FingerprintV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct EvidenceReplayLedger {
    schema_ref: String,
    operation: String,
    key_fingerprint: FingerprintV1,
    receipt: EvidenceAppendReceiptV1,
    ledger_fingerprint: FingerprintV1,
}
impl EvidenceReplayLedger {
    fn new(
        key: &FingerprintV1,
        receipt: EvidenceAppendReceiptV1,
    ) -> Result<Self, ContractOwnerRepositoryErrorV1> {
        let ledger_fingerprint = canonical_fingerprint(&EvidenceLedgerPreimage {
            schema_ref: "handbook.contract-evidence-append-replay@1.0.0",
            operation: "contract.evidence.append",
            key_fingerprint: key,
            receipt: &receipt,
        })
        .map_err(|_| durability("evidence ledger identity failed"))?;
        Ok(Self {
            schema_ref: "handbook.contract-evidence-append-replay@1.0.0".to_owned(),
            operation: "contract.evidence.append".to_owned(),
            key_fingerprint: key.clone(),
            receipt,
            ledger_fingerprint,
        })
    }
    fn validate(&self) -> Result<(), ContractOwnerRepositoryErrorV1> {
        self.receipt.validate()?;
        if self != &Self::new(&self.key_fingerprint, self.receipt.clone())? {
            return Err(recovery("evidence replay ledger is substituted"));
        }
        Ok(())
    }
}
#[derive(Serialize)]
struct EvidenceLedgerPreimage<'a> {
    schema_ref: &'static str,
    operation: &'static str,
    key_fingerprint: &'a FingerprintV1,
    receipt: &'a EvidenceAppendReceiptV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct EvidenceTransactionIntent {
    schema_ref: String,
    transaction_id: String,
    operation: String,
    key_fingerprint: FingerprintV1,
    request_fingerprint: FingerprintV1,
    outputs: Vec<StagedOutput>,
    intent_fingerprint: FingerprintV1,
}
impl EvidenceTransactionIntent {
    fn new(
        id: String,
        key: &FingerprintV1,
        request: &FingerprintV1,
        outputs: Vec<StagedOutput>,
    ) -> Result<Self, ContractOwnerRepositoryErrorV1> {
        let intent_fingerprint = canonical_fingerprint(&EvidenceIntentPreimage {
            schema_ref: "handbook.contract-evidence-transaction-intent@1.0.0",
            transaction_id: &id,
            operation: "contract.evidence.append",
            key_fingerprint: key,
            request_fingerprint: request,
            outputs: &outputs,
        })
        .map_err(|_| durability("evidence intent identity failed"))?;
        Ok(Self {
            schema_ref: "handbook.contract-evidence-transaction-intent@1.0.0".to_owned(),
            transaction_id: id,
            operation: "contract.evidence.append".to_owned(),
            key_fingerprint: key.clone(),
            request_fingerprint: request.clone(),
            outputs,
            intent_fingerprint,
        })
    }
    fn validate(&self) -> Result<(), ContractOwnerRepositoryErrorV1> {
        if self.schema_ref != "handbook.contract-evidence-transaction-intent@1.0.0"
            || self.operation != "contract.evidence.append"
            || self.outputs.len() != 2
            || self.transaction_id.len() != 64
            || !self
                .transaction_id
                .bytes()
                .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
        {
            return Err(recovery("evidence transaction intent shape is invalid"));
        }
        if self.outputs[0].staged_name != "00-evidence.json"
            || !is_exact_evidence_output_ref(&self.outputs[0].relative_ref, "records")
            || self.outputs[1].staged_name != "01-ledger.json"
            || !is_exact_evidence_output_ref(&self.outputs[1].relative_ref, "idempotency")
            || self.outputs[0].relative_ref == self.outputs[1].relative_ref
            || self.outputs[0].bytes_fingerprint == self.outputs[1].bytes_fingerprint
        {
            return Err(recovery("evidence transaction output class is invalid"));
        }
        let expected_id = fingerprint_hex(
            &canonical_fingerprint(&EvidenceTransactionIdPreimage {
                schema_ref: "handbook.contract-evidence-transaction-identity@1.0.0",
                key_fingerprint: &self.key_fingerprint,
                request_fingerprint: &self.request_fingerprint,
            })
            .map_err(|_| recovery("evidence transaction identity cannot be rederived"))?,
        )
        .to_owned();
        if expected_id != self.transaction_id
            || self
                != &Self::new(
                    self.transaction_id.clone(),
                    &self.key_fingerprint,
                    &self.request_fingerprint,
                    self.outputs.clone(),
                )?
        {
            return Err(recovery("evidence transaction intent is substituted"));
        }
        Ok(())
    }
}

fn is_exact_evidence_output_ref(value: &str, class: &str) -> bool {
    let Some(name) = value.strip_prefix(&format!("{EVIDENCE_ROOT}/{class}/")) else {
        return false;
    };
    let Some(hex) = name.strip_suffix(".json") else {
        return false;
    };
    hex.len() == 64
        && hex
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
}
#[derive(Serialize)]
struct EvidenceIntentPreimage<'a> {
    schema_ref: &'static str,
    transaction_id: &'a str,
    operation: &'static str,
    key_fingerprint: &'a FingerprintV1,
    request_fingerprint: &'a FingerprintV1,
    outputs: &'a [StagedOutput],
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct EvidenceCommitMarker {
    schema_ref: String,
    intent_fingerprint: FingerprintV1,
    marker_fingerprint: FingerprintV1,
}
impl EvidenceCommitMarker {
    fn new(intent: &EvidenceTransactionIntent) -> Result<Self, ContractOwnerRepositoryErrorV1> {
        let marker_fingerprint = canonical_fingerprint(&EvidenceMarkerPreimage {
            schema_ref: "handbook.contract-evidence-commit-marker@1.0.0",
            intent_fingerprint: &intent.intent_fingerprint,
        })
        .map_err(|_| durability("evidence marker identity failed"))?;
        Ok(Self {
            schema_ref: "handbook.contract-evidence-commit-marker@1.0.0".to_owned(),
            intent_fingerprint: intent.intent_fingerprint.clone(),
            marker_fingerprint,
        })
    }
    fn validate(
        &self,
        intent: &EvidenceTransactionIntent,
    ) -> Result<(), ContractOwnerRepositoryErrorV1> {
        if self != &Self::new(intent)? {
            return Err(recovery("evidence commit marker is substituted"));
        }
        Ok(())
    }
}
#[derive(Serialize)]
struct EvidenceMarkerPreimage<'a> {
    schema_ref: &'static str,
    intent_fingerprint: &'a FingerprintV1,
}

#[derive(Serialize)]
struct EvidenceListRootPreimage<'a> {
    schema_ref: &'static str,
    retention: &'static str,
    snapshot_sequence: u64,
    entries: &'a [EvidenceListEntryV1],
}
fn evidence_list_root(
    snapshot: u64,
    entries: &[EvidenceListEntryV1],
) -> Result<FingerprintV1, ContractOwnerRepositoryErrorV1> {
    canonical_fingerprint(&EvidenceListRootPreimage {
        schema_ref: "handbook.contract-evidence-list-root@1.0.0",
        retention: "immutable_all",
        snapshot_sequence: snapshot,
        entries,
    })
    .map_err(|_| durability("evidence list root could not be derived"))
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct EvidenceCursorV1 {
    schema_ref: String,
    root: FingerprintV1,
    snapshot_sequence: u64,
    offset: usize,
    page_size: usize,
    cursor_fingerprint: FingerprintV1,
}
#[derive(Serialize)]
struct EvidenceCursorPreimage<'a> {
    schema_ref: &'static str,
    domain: &'static str,
    root: &'a FingerprintV1,
    snapshot_sequence: u64,
    offset: usize,
    page_size: usize,
}
fn encode_evidence_cursor(
    root: &FingerprintV1,
    snapshot: u64,
    offset: usize,
    page_size: usize,
) -> Result<String, ContractOwnerRepositoryErrorV1> {
    let cursor_fingerprint = canonical_fingerprint(&EvidenceCursorPreimage {
        schema_ref: "handbook.contract-evidence-list-cursor@1.0.0",
        domain: "contract-evidence-list-cursor",
        root,
        snapshot_sequence: snapshot,
        offset,
        page_size,
    })
    .map_err(|_| durability("evidence cursor identity failed"))?;
    let bytes = canonical_bytes(&EvidenceCursorV1 {
        schema_ref: "handbook.contract-evidence-list-cursor@1.0.0".to_owned(),
        root: root.clone(),
        snapshot_sequence: snapshot,
        offset,
        page_size,
        cursor_fingerprint,
    })
    .map_err(|_| durability("evidence cursor encoding failed"))?;
    Ok(format!("hcm-evidence-v1.{}", hex_encode(&bytes)))
}
fn decode_evidence_cursor(
    token: &str,
    root: &FingerprintV1,
    page_size: usize,
) -> Result<EvidenceCursorV1, ContractOwnerRepositoryErrorV1> {
    let bytes = hex_decode(
        token
            .strip_prefix("hcm-evidence-v1.")
            .ok_or_else(|| substituted("evidence cursor envelope is malformed"))?,
    )?;
    if bytes.len() > 2048 {
        return Err(bound("evidence cursor payload exceeds its bound"));
    }
    let cursor: EvidenceCursorV1 =
        parse_canonical(&bytes).map_err(|_| substituted("evidence cursor is malformed"))?;
    let expected = canonical_fingerprint(&EvidenceCursorPreimage {
        schema_ref: "handbook.contract-evidence-list-cursor@1.0.0",
        domain: "contract-evidence-list-cursor",
        root: &cursor.root,
        snapshot_sequence: cursor.snapshot_sequence,
        offset: cursor.offset,
        page_size: cursor.page_size,
    })
    .map_err(|_| durability("evidence cursor identity failed"))?;
    if cursor.schema_ref != "handbook.contract-evidence-list-cursor@1.0.0"
        || cursor.root != *root
        || cursor.page_size != page_size
        || cursor.cursor_fingerprint != expected
    {
        return Err(substituted("evidence cursor is substituted"));
    }
    Ok(cursor)
}

fn evidence_request_fingerprint(
    candidate: &ContentAddressedBindingV1,
) -> Result<FingerprintV1, ContractOwnerRepositoryErrorV1> {
    canonical_fingerprint(&EvidenceRequestPreimage {
        schema_ref: "handbook.contract-evidence-append-request-fingerprint@1.0.0",
        candidate,
    })
    .map_err(|_| durability("evidence request identity failed"))
}
fn evidence_key_fingerprint(key: &str) -> Result<FingerprintV1, ContractOwnerRepositoryErrorV1> {
    canonical_fingerprint(&EvidenceKeyPreimage {
        schema_ref: "handbook.contract-evidence-idempotency-key@1.0.0",
        key,
    })
    .map_err(|_| durability("evidence key identity failed"))
}
fn map_candidate_source(
    _: crate::EvidenceCandidateSourceErrorV1,
) -> ContractOwnerRepositoryErrorV1 {
    durability("evidence candidate source is unavailable or refused")
}
fn map_candidate_admission(
    error: crate::EvidenceCandidateAdmissionErrorV1,
) -> ContractOwnerRepositoryErrorV1 {
    match error.kind() {
        EvidenceCandidateAdmissionErrorKindV1::StaleLifecycle => repository_error(
            ContractOwnerRepositoryErrorKindV1::LifecycleRefused,
            "evidence lifecycle is stale",
        ),
        EvidenceCandidateAdmissionErrorKindV1::SourceUnavailable => {
            durability("evidence source is unavailable")
        }
        EvidenceCandidateAdmissionErrorKindV1::CandidateOverLimit => {
            bound("evidence candidate exceeds its bound")
        }
        EvidenceCandidateAdmissionErrorKindV1::CandidateSubstituted
        | EvidenceCandidateAdmissionErrorKindV1::StaleSource
        | EvidenceCandidateAdmissionErrorKindV1::SourceAuthorityRefused
        | EvidenceCandidateAdmissionErrorKindV1::PolicySubstituted => {
            substituted("evidence admission currentness or identity was substituted")
        }
        _ => invalid("evidence candidate was not admitted"),
    }
}
