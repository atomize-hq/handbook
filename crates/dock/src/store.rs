use crate::canonical::{canonical_bytes, canonical_fingerprint, fingerprint_hex};
use crate::{
    DockCleanupDispositionV1, DockDurabilityDispositionV1, DockErrorV1, DockExecutionRecordV1,
    DockHostOutcomeV1, DockProcessRequestV1, DockProcessResultStatusV1, DockProcessResultV1,
};
use handbook_contracts::{
    resolve_repository_candidate_observation_v1, ContentAddressedBindingV1,
    EvidenceCandidateInputV1, EvidenceCandidateSourceErrorKindV1, EvidenceCandidateSourceErrorV1,
    EvidenceCandidateSourceV1, EvidenceCandidateV1, EvidenceClaimExclusionInputV1,
    EvidenceExclusionKindV1, EvidenceSourceBindingInputV1, ExactBindingV1, FingerprintV1,
    NativeDockCandidateObservationV1 as StoredCandidateObservationV1,
    ResolvedEvidenceCandidateObservationInputV1, ResolvedEvidenceCandidateObservationV1,
};
use handbook_engine::{
    EvidenceSourceAdmissionErrorKindV1, EvidenceSourceAdmissionErrorV1,
    EvidenceSourceAdmissionServiceV1, NativeAuthenticatorPortV1, RepositoryDirectoryEntryKindV1,
    RepositoryDurabilityErrorKindV1, RepositoryDurabilityErrorV1, RepositoryExclusiveLockV1,
    RepositoryReadV1, RetainedRepositoryRootV1, SourceCurrentnessObservationV1,
    SourceCurrentnessPortV1, SourceCurrentnessRequestV1,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;
use std::path::Path;

pub const DOCK_OPERATIONAL_STORE_RELATIVE_ROOT_V1: &str = ".handbook/state/dock/v1";
const LOCK_RELATIVE: &str = ".handbook/state/locks/dock-operational-store.lock";
const REQUESTS: &str = ".handbook/state/dock/v1/requests";
const RESULTS: &str = ".handbook/state/dock/v1/results";
const EXECUTIONS: &str = ".handbook/state/dock/v1/executions";
const CANDIDATES: &str = ".handbook/state/dock/v1/candidates";
const REPLAYS: &str = ".handbook/state/dock/v1/replays";
const TRANSACTIONS: &str = ".handbook/state/dock/v1/transactions";
const MAX_RECORD_BYTES: usize = 8 * 1024 * 1024;
const MAX_RECORDS: usize = 4096;
const CANDIDATE_RECORD_SCHEMA: &str = "handbook.dock.internal-candidate-observation@1.0.0";
const REPLAY_SCHEMA: &str = "handbook.dock.internal-run-replay@1.0.0";
const INTENT_SCHEMA: &str = "handbook.dock.internal-store-intent@1.0.0";
const MARKER_SCHEMA: &str = "handbook.dock.internal-store-commit@1.0.0";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DockStoreErrorKindV1 {
    InvalidRequest,
    Missing,
    BoundExceeded,
    IdempotencyConflict,
    RecordSubstituted,
    InventoryAmbiguous,
    DurabilityUncertain,
    UnsafeFilesystem,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DockStoreErrorV1 {
    kind: DockStoreErrorKindV1,
    detail: &'static str,
}

impl DockStoreErrorV1 {
    pub fn kind(&self) -> DockStoreErrorKindV1 {
        self.kind
    }
    pub fn detail(&self) -> &str {
        self.detail
    }
}

impl fmt::Display for DockStoreErrorV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.detail)
    }
}

impl Error for DockStoreErrorV1 {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DockStoreDispositionV1 {
    Committed,
    Replayed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DockStoreReceiptV1 {
    disposition: DockStoreDispositionV1,
    root: FingerprintV1,
    execution_record: DockExecutionRecordV1,
    candidates: Vec<ContentAddressedBindingV1>,
    evaluation_run_jcs: Vec<u8>,
}

impl DockStoreReceiptV1 {
    pub fn disposition(&self) -> DockStoreDispositionV1 {
        self.disposition
    }
    pub fn root(&self) -> &FingerprintV1 {
        &self.root
    }
    pub fn execution_record(&self) -> &DockExecutionRecordV1 {
        &self.execution_record
    }
    pub fn candidates(&self) -> &[ContentAddressedBindingV1] {
        &self.candidates
    }
    pub fn evaluation_run_jcs(&self) -> &[u8] {
        &self.evaluation_run_jcs
    }
}

#[cfg(feature = "test-support")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DockStoreFaultBoundaryV1 {
    AfterIntent,
    AfterFirstInstall,
    BeforeCommitMarker,
}

#[derive(Serialize)]
struct RootPreimage<'a> {
    schema_ref: &'static str,
    key_fingerprint: &'a FingerprintV1,
    idempotency_request_fingerprint: &'a FingerprintV1,
    request: &'a ContentAddressedBindingV1,
    result: &'a Option<ContentAddressedBindingV1>,
    execution: &'a ContentAddressedBindingV1,
    candidates: &'a [ContentAddressedBindingV1],
    evaluation_run_jcs: &'a str,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct ReplayLedgerV1 {
    schema_ref: String,
    key_fingerprint: FingerprintV1,
    idempotency_request_fingerprint: FingerprintV1,
    request: ContentAddressedBindingV1,
    result: Option<ContentAddressedBindingV1>,
    execution: ContentAddressedBindingV1,
    candidates: Vec<ContentAddressedBindingV1>,
    evaluation_run_jcs: String,
    root: FingerprintV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct OutputIntentV1 {
    stage: String,
    target: String,
    byte_length: usize,
    content_fingerprint: FingerprintV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct StoreIntentV1 {
    schema_ref: String,
    key_fingerprint: FingerprintV1,
    idempotency_request_fingerprint: FingerprintV1,
    request: ContentAddressedBindingV1,
    result: Option<ContentAddressedBindingV1>,
    execution: ContentAddressedBindingV1,
    candidates: Vec<ContentAddressedBindingV1>,
    evaluation_run_jcs: String,
    root: FingerprintV1,
    outputs: Vec<OutputIntentV1>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct CommitMarkerV1 {
    schema_ref: String,
    root: FingerprintV1,
}

#[derive(Deserialize)]
struct CandidateEnvelopeBindingsV1 {
    body: CandidateBodyBindingsV1,
}

#[derive(Deserialize)]
struct CandidateBodyBindingsV1 {
    evaluation_run: ContentAddressedBindingV1,
    producer: ContentAddressedBindingV1,
    request_record: ContentAddressedBindingV1,
    execution_record: ContentAddressedBindingV1,
}

pub struct DockOperationalStoreV1 {
    pub(crate) repository_root: RetainedRepositoryRootV1,
}

pub(crate) struct LocalCandidateStoreObservationV2 {
    pub(crate) stored: StoredCandidateObservationV1,
    pub(crate) request: DockProcessRequestV1,
    pub(crate) exact_paths: Vec<String>,
}

pub(crate) enum DockExecutionStartV1<'store> {
    Replay(Box<DockStoreReceiptV1>),
    Lease(DockExecutionLeaseV1<'store>),
}

pub(crate) struct DockExecutionLeaseV1<'store> {
    store: &'store DockOperationalStoreV1,
    lock: Option<RepositoryExclusiveLockV1>,
    request: ContentAddressedBindingV1,
    key_fingerprint: FingerprintV1,
    idempotency_request_fingerprint: FingerprintV1,
}

impl DockExecutionLeaseV1<'_> {
    pub(crate) fn commit(
        mut self,
        request: &DockProcessRequestV1,
        result: Option<&DockProcessResultV1>,
        execution: &DockExecutionRecordV1,
        evaluation_run_jcs: &[u8],
        #[cfg(feature = "test-support")] fault: Option<DockStoreFaultBoundaryV1>,
        #[cfg(not(feature = "test-support"))] _fault: Option<()>,
    ) -> Result<DockStoreReceiptV1, DockStoreErrorV1> {
        if request.binding().map_err(map_dock_error)? != self.request {
            return Err(conflict("execution lease request was substituted"));
        }
        let prepared = self.store.prepare_commit(
            request,
            result,
            execution,
            evaluation_run_jcs,
            self.key_fingerprint.clone(),
            self.idempotency_request_fingerprint.clone(),
        )?;
        if self
            .store
            .read_optional(
                &request_path(request.request_fingerprint()),
                MAX_RECORD_BYTES,
            )?
            .is_some()
            || self
                .store
                .read_optional(
                    &execution_path(execution.record_fingerprint()),
                    MAX_RECORD_BYTES,
                )?
                .is_some()
        {
            return Err(conflict(
                "request or execution identity is already committed",
            ));
        }
        self.store.stage_transaction(&prepared)?;
        #[cfg(feature = "test-support")]
        if fault == Some(DockStoreFaultBoundaryV1::AfterIntent) {
            return Err(durability("injected uncertainty after store intent"));
        }
        #[cfg(feature = "test-support")]
        self.store.install_transaction(&prepared, fault)?;
        #[cfg(not(feature = "test-support"))]
        self.store.install_transaction(&prepared, _fault)?;
        self.store.validate_inventory_locked()?;
        let receipt = self
            .store
            .receipt_from_ledger(prepared.ledger, DockStoreDispositionV1::Committed)?;
        self.lock
            .take()
            .expect("execution lease retains its lock")
            .release()
            .map_err(map_repository_error)?;
        Ok(receipt)
    }
}

impl DockOperationalStoreV1 {
    pub(crate) fn open_read_only(repo_root: impl AsRef<Path>) -> Result<Self, DockStoreErrorV1> {
        let repository_root =
            RetainedRepositoryRootV1::open_exact(repo_root).map_err(map_repository_error)?;
        let store = Self { repository_root };
        let lock = store
            .repository_root
            .acquire_existing_exclusive_lock_v1(LOCK_RELATIVE)
            .map_err(map_repository_error)?;
        store.validate_inventory_locked()?;
        lock.release().map_err(map_repository_error)?;
        Ok(store)
    }

    pub fn open(repo_root: impl AsRef<Path>) -> Result<Self, DockStoreErrorV1> {
        let repository_root =
            RetainedRepositoryRootV1::open_exact(repo_root).map_err(map_repository_error)?;
        ensure_layout(&repository_root)?;
        let store = Self { repository_root };
        let lock = store
            .repository_root
            .acquire_exclusive_lock(LOCK_RELATIVE)
            .map_err(map_repository_error)?;
        store.recover_locked()?;
        store.validate_inventory_locked()?;
        lock.release().map_err(map_repository_error)?;
        Ok(store)
    }

    pub fn commit(
        &self,
        request: &DockProcessRequestV1,
        result: Option<&DockProcessResultV1>,
        execution: &DockExecutionRecordV1,
        evaluation_run_jcs: &[u8],
        idempotency_key: &str,
    ) -> Result<DockStoreReceiptV1, DockStoreErrorV1> {
        self.commit_inner(
            request,
            result,
            execution,
            evaluation_run_jcs,
            idempotency_key,
            None,
        )
    }

    pub(crate) fn begin_execution_with_idempotency_request(
        &self,
        request: &DockProcessRequestV1,
        idempotency_key: &str,
        idempotency_request_fingerprint: &FingerprintV1,
    ) -> Result<DockExecutionStartV1<'_>, DockStoreErrorV1> {
        validate_key(idempotency_key)?;
        let key_fingerprint = key_fingerprint(idempotency_key)?;
        let request_binding = request.binding().map_err(map_dock_error)?;
        let lock = self
            .repository_root
            .acquire_exclusive_lock(LOCK_RELATIVE)
            .map_err(map_repository_error)?;
        self.recover_locked()?;
        self.validate_inventory_locked()?;
        if let Some(replay) = self.read_replay(&key_fingerprint)? {
            if &replay.idempotency_request_fingerprint != idempotency_request_fingerprint {
                return Err(conflict(
                    "idempotency key is bound to another exact request",
                ));
            }
            let receipt = self.receipt_from_ledger(replay, DockStoreDispositionV1::Replayed)?;
            lock.release().map_err(map_repository_error)?;
            return Ok(DockExecutionStartV1::Replay(Box::new(receipt)));
        }
        Ok(DockExecutionStartV1::Lease(DockExecutionLeaseV1 {
            store: self,
            lock: Some(lock),
            request: request_binding,
            key_fingerprint,
            idempotency_request_fingerprint: idempotency_request_fingerprint.clone(),
        }))
    }

    /// Resolve an already committed exact key/request replay without launching again.
    pub fn replay_for_request(
        &self,
        request: &DockProcessRequestV1,
        idempotency_key: &str,
    ) -> Result<Option<DockStoreReceiptV1>, DockStoreErrorV1> {
        validate_key(idempotency_key)?;
        let key_fingerprint = key_fingerprint(idempotency_key)?;
        let lock = self
            .repository_root
            .acquire_exclusive_lock(LOCK_RELATIVE)
            .map_err(map_repository_error)?;
        self.recover_locked()?;
        self.validate_inventory_locked()?;
        let Some(replay) = self.read_replay(&key_fingerprint)? else {
            lock.release().map_err(map_repository_error)?;
            return Ok(None);
        };
        if replay.request != request.binding().map_err(map_dock_error)? {
            return Err(conflict(
                "idempotency key is bound to another exact request",
            ));
        }
        let receipt = self.receipt_from_ledger(replay, DockStoreDispositionV1::Replayed)?;
        lock.release().map_err(map_repository_error)?;
        Ok(Some(receipt))
    }

    #[cfg(feature = "test-support")]
    pub fn commit_with_fault_for_testing(
        &self,
        request: &DockProcessRequestV1,
        result: Option<&DockProcessResultV1>,
        execution: &DockExecutionRecordV1,
        evaluation_run_jcs: &[u8],
        idempotency_key: &str,
        fault: DockStoreFaultBoundaryV1,
    ) -> Result<DockStoreReceiptV1, DockStoreErrorV1> {
        self.commit_inner(
            request,
            result,
            execution,
            evaluation_run_jcs,
            idempotency_key,
            Some(fault),
        )
    }

    fn commit_inner(
        &self,
        request: &DockProcessRequestV1,
        result: Option<&DockProcessResultV1>,
        execution: &DockExecutionRecordV1,
        evaluation_run_jcs: &[u8],
        idempotency_key: &str,
        #[cfg(feature = "test-support")] fault: Option<DockStoreFaultBoundaryV1>,
        #[cfg(not(feature = "test-support"))] _fault: Option<()>,
    ) -> Result<DockStoreReceiptV1, DockStoreErrorV1> {
        validate_key(idempotency_key)?;
        let key_fingerprint = key_fingerprint(idempotency_key)?;
        let lock = self
            .repository_root
            .acquire_exclusive_lock(LOCK_RELATIVE)
            .map_err(map_repository_error)?;
        self.recover_locked()?;
        self.validate_inventory_locked()?;

        if let Some(replay) = self.read_replay(&key_fingerprint)? {
            let request_binding = request.binding().map_err(map_dock_error)?;
            let execution_binding = execution.binding().map_err(map_dock_error)?;
            let result_binding = result
                .map(DockProcessResultV1::binding)
                .transpose()
                .map_err(map_dock_error)?;
            if replay.request != request_binding
                || replay.result != result_binding
                || replay.execution != execution_binding
            {
                return Err(conflict("idempotency key is bound to another exact run"));
            }
            let receipt = self.receipt_from_ledger(replay, DockStoreDispositionV1::Replayed)?;
            lock.release().map_err(map_repository_error)?;
            return Ok(receipt);
        }

        let prepared = self.prepare_commit(
            request,
            result,
            execution,
            evaluation_run_jcs,
            key_fingerprint,
            request.request_fingerprint().clone(),
        )?;
        if self
            .read_optional(
                &request_path(request.request_fingerprint()),
                MAX_RECORD_BYTES,
            )?
            .is_some()
            || self
                .read_optional(
                    &execution_path(execution.record_fingerprint()),
                    MAX_RECORD_BYTES,
                )?
                .is_some()
        {
            return Err(conflict(
                "request or execution identity is already committed",
            ));
        }
        self.stage_transaction(&prepared)?;
        #[cfg(feature = "test-support")]
        if fault == Some(DockStoreFaultBoundaryV1::AfterIntent) {
            return Err(durability("injected uncertainty after store intent"));
        }
        #[cfg(feature = "test-support")]
        self.install_transaction(&prepared, fault)?;
        #[cfg(not(feature = "test-support"))]
        self.install_transaction(&prepared, _fault)?;
        self.validate_inventory_locked()?;
        let receipt =
            self.receipt_from_ledger(prepared.ledger, DockStoreDispositionV1::Committed)?;
        lock.release().map_err(map_repository_error)?;
        Ok(receipt)
    }

    fn prepare_commit(
        &self,
        request: &DockProcessRequestV1,
        result: Option<&DockProcessResultV1>,
        execution: &DockExecutionRecordV1,
        evaluation_run_jcs: &[u8],
        key_fingerprint: FingerprintV1,
        idempotency_request_fingerprint: FingerprintV1,
    ) -> Result<PreparedTransaction, DockStoreErrorV1> {
        let request_binding = request.binding().map_err(map_dock_error)?;
        let result_binding = result
            .map(DockProcessResultV1::binding)
            .transpose()
            .map_err(map_dock_error)?;
        let execution_binding = execution.binding().map_err(map_dock_error)?;
        if execution.request() != &request_binding
            || execution.result() != result_binding.as_ref()
            || execution.manifest() != request.manifest()
            || execution.implementation_bundle() != request.implementation_bundle()
            || execution.evaluation_run() != request.run().evaluation_run()
        {
            return Err(substituted(
                "execution record is not bound to the exact request",
            ));
        }
        if let Some(result) = result {
            if result.request() != &request_binding
                || result.manifest() != request.manifest()
                || result.implementation_bundle() != request.implementation_bundle()
                || result.evaluation_run() != request.run().evaluation_run()
            {
                return Err(substituted(
                    "process result is not bound to the exact request",
                ));
            }
        }
        let run_jcs = validate_run_jcs(evaluation_run_jcs, request.run().evaluation_run())?;
        let observations = if execution.host_outcome() == DockHostOutcomeV1::Completed {
            if execution.cleanup() != DockCleanupDispositionV1::Confirmed
                || execution.durability() != DockDurabilityDispositionV1::Confirmed
            {
                return Err(substituted(
                    "completed execution lacks clean durable dispositions",
                ));
            }
            let result = result.ok_or_else(|| substituted("completed execution has no result"))?;
            if result.status() != DockProcessResultStatusV1::Completed {
                return Err(substituted(
                    "completed execution has a non-completed result",
                ));
            }
            let expected_bundle =
                canonical_fingerprint(&result.candidates()).map_err(map_dock_error)?;
            if execution.candidate_observation_count() != result.candidates().len()
                || execution.candidate_observation_bundle_fingerprint() != &expected_bundle
            {
                return Err(substituted("execution candidate bundle is substituted"));
            }
            result.candidates()
        } else {
            if execution.candidate_observation_count() != 0 {
                return Err(substituted("non-completed execution exposes candidates"));
            }
            &[]
        };

        let mut stored_candidates = Vec::with_capacity(observations.len());
        let mut candidate_bindings = Vec::with_capacity(observations.len());
        if !observations.is_empty() {
            let result = result.expect("completed observations require a result");
            let partition = result
                .claim_partition()
                .ok_or_else(|| substituted("completed result has no claim partition"))?;
            let actual_resolution = result
                .actual_resolution()
                .ok_or_else(|| substituted("completed result has no actual resolution"))?;
            let producer = content_from_exact(request.manifest())?;
            let sources = request
                .inputs()
                .iter()
                .map(|input| {
                    Ok(EvidenceSourceBindingInputV1 {
                        source: ContentAddressedBindingV1::new(
                            &format!("{}#input/{}", request.request_ref(), input.role().as_str()),
                            input.content_fingerprint().clone(),
                        )
                        .map_err(|_| invalid("request input source identity is invalid"))?,
                        revision: input.content_fingerprint().as_str().to_owned(),
                        collected_at: request.run().semantic_time_utc().clone(),
                    })
                })
                .collect::<Result<Vec<_>, DockStoreErrorV1>>()?;
            for observation in observations {
                let case = request
                    .run()
                    .cases()
                    .iter()
                    .find(|case| case.case_id() == observation.case_id())
                    .cloned()
                    .ok_or_else(|| substituted("candidate case is absent from the request"))?;
                let candidate = EvidenceCandidateV1::new(EvidenceCandidateInputV1 {
                    evaluation_run: request.run().evaluation_run().clone(),
                    evaluation_lifecycle_basis: request.run().evaluation_lifecycle_basis().clone(),
                    contract: request.run().contract().clone(),
                    subject: request.run().subject().clone(),
                    case,
                    observed_claim_ids: vec![observation.claim_id().clone()],
                    evidence_kind: observation.evidence_kind().clone(),
                    fact_schema: observation.payload_schema().clone(),
                    fact_payload: observation.payload().clone(),
                    observation: observation.observation(),
                    producer: producer.clone(),
                    request_record: request_binding.clone(),
                    execution_record: execution_binding.clone(),
                    sources: sources.clone(),
                    observed_claim_partition: partition.observed().to_vec(),
                    unobserved_claim_partition: partition.unobserved().to_vec(),
                    excluded_claim_partition: partition
                        .typed_excluded()
                        .iter()
                        .cloned()
                        .map(|claim_id| EvidenceClaimExclusionInputV1 {
                            claim_id,
                            kind: EvidenceExclusionKindV1::UnsupportedKind,
                        })
                        .collect(),
                    request_resolution: request.run().requested_resolution().clone(),
                    actual_resolution: actual_resolution.clone(),
                    observed_at: request.run().semantic_time_utc().clone(),
                })
                .map_err(|_| substituted("candidate materialization was refused by Contracts"))?;
                let candidate_binding = ContentAddressedBindingV1::new(
                    candidate.candidate_ref(),
                    candidate.candidate_fingerprint().clone(),
                )
                .map_err(|_| invalid("materialized candidate binding is invalid"))?;
                let stored = StoredCandidateObservationV1 {
                    schema_ref: CANDIDATE_RECORD_SCHEMA.to_owned(),
                    candidate: candidate_binding.clone(),
                    candidate_jcs: String::from_utf8(candidate.canonical_json_bytes().map_err(
                        |_| invalid("materialized candidate canonical bytes are unavailable"),
                    )?)
                    .expect("canonical JSON is UTF-8"),
                    evaluation_run_jcs: String::from_utf8(run_jcs.clone())
                        .expect("validated run JSON is UTF-8"),
                    producer: producer.clone(),
                    request_record: request_binding.clone(),
                    execution_record: execution_binding.clone(),
                    producer_resolution_ceiling: Some(request.run().requested_resolution().clone()),
                };
                validate_stored_candidate(&stored)?;
                candidate_bindings.push(candidate_binding);
                stored_candidates.push(stored);
            }
        }

        let evaluation_run_jcs = String::from_utf8(run_jcs).expect("validated run JSON is UTF-8");
        let root = canonical_fingerprint(&RootPreimage {
            schema_ref: REPLAY_SCHEMA,
            key_fingerprint: &key_fingerprint,
            idempotency_request_fingerprint: &idempotency_request_fingerprint,
            request: &request_binding,
            result: &result_binding,
            execution: &execution_binding,
            candidates: &candidate_bindings,
            evaluation_run_jcs: &evaluation_run_jcs,
        })
        .map_err(map_dock_error)?;
        let ledger = ReplayLedgerV1 {
            schema_ref: REPLAY_SCHEMA.to_owned(),
            key_fingerprint: key_fingerprint.clone(),
            idempotency_request_fingerprint,
            request: request_binding,
            result: result_binding,
            execution: execution_binding,
            candidates: candidate_bindings,
            evaluation_run_jcs,
            root,
        };
        validate_ledger(&ledger)?;

        let mut records = vec![(
            request_path(request.request_fingerprint()),
            request.canonical_json_bytes().map_err(map_dock_error)?,
        )];
        if let Some(result) = result {
            records.push((
                result_path(result.result_fingerprint()),
                result.canonical_json_bytes().map_err(map_dock_error)?,
            ));
        }
        records.push((
            execution_path(execution.record_fingerprint()),
            execution.canonical_json_bytes().map_err(map_dock_error)?,
        ));
        for candidate in &stored_candidates {
            records.push((
                candidate_path(candidate.candidate.fingerprint()),
                canonical_bytes(candidate).map_err(map_dock_error)?,
            ));
        }
        records.push((
            replay_path(&key_fingerprint),
            canonical_bytes(&ledger).map_err(map_dock_error)?,
        ));
        if records.len() > MAX_RECORDS {
            return Err(bound("store transaction exceeds its record bound"));
        }
        let transaction = transaction_root(&key_fingerprint);
        let mut outputs = Vec::with_capacity(records.len());
        for (index, (target, bytes)) in records.iter().enumerate() {
            outputs.push(OutputIntentV1 {
                stage: format!("{transaction}/{index:04}.stage"),
                target: target.clone(),
                byte_length: bytes.len(),
                content_fingerprint: bytes_fingerprint(bytes)?,
            });
        }
        let intent = StoreIntentV1 {
            schema_ref: INTENT_SCHEMA.to_owned(),
            key_fingerprint,
            idempotency_request_fingerprint: ledger.idempotency_request_fingerprint.clone(),
            request: ledger.request.clone(),
            result: ledger.result.clone(),
            execution: ledger.execution.clone(),
            candidates: ledger.candidates.clone(),
            evaluation_run_jcs: ledger.evaluation_run_jcs.clone(),
            root: ledger.root.clone(),
            outputs,
        };
        validate_intent(&intent, &transaction)?;
        Ok(PreparedTransaction {
            transaction,
            records,
            intent,
            ledger,
        })
    }

    fn stage_transaction(&self, prepared: &PreparedTransaction) -> Result<(), DockStoreErrorV1> {
        self.repository_root
            .create_directory_new_durable(&prepared.transaction)
            .map_err(map_repository_error)?;
        for (output, (_, bytes)) in prepared.intent.outputs.iter().zip(&prepared.records) {
            self.repository_root
                .write_new_durable(&output.stage, bytes)
                .map_err(map_repository_error)?;
        }
        self.repository_root
            .write_new_durable(
                &format!("{}/intent.json", prepared.transaction),
                &canonical_bytes(&prepared.intent).map_err(map_dock_error)?,
            )
            .map_err(map_repository_error)
    }

    fn install_transaction(
        &self,
        prepared: &PreparedTransaction,
        #[cfg(feature = "test-support")] fault: Option<DockStoreFaultBoundaryV1>,
        #[cfg(not(feature = "test-support"))] _fault: Option<()>,
    ) -> Result<(), DockStoreErrorV1> {
        #[cfg(feature = "test-support")]
        let mut installed = 0_usize;
        for output in &prepared.intent.outputs {
            self.repository_root
                .rename_noreplace_durable(&output.stage, &output.target)
                .map_err(map_repository_error)?;
            #[cfg(feature = "test-support")]
            {
                installed += 1;
                if installed == 1 && fault == Some(DockStoreFaultBoundaryV1::AfterFirstInstall) {
                    return Err(durability("injected uncertainty after first store install"));
                }
            }
        }
        #[cfg(feature = "test-support")]
        if fault == Some(DockStoreFaultBoundaryV1::BeforeCommitMarker) {
            return Err(durability(
                "injected uncertainty before store commit marker",
            ));
        }
        let marker = CommitMarkerV1 {
            schema_ref: MARKER_SCHEMA.to_owned(),
            root: prepared.intent.root.clone(),
        };
        self.repository_root
            .write_new_durable(
                &format!("{}/committed.json", prepared.transaction),
                &canonical_bytes(&marker).map_err(map_dock_error)?,
            )
            .map_err(map_repository_error)
    }

    fn recover_locked(&self) -> Result<(), DockStoreErrorV1> {
        for entry in self
            .repository_root
            .list_directory_bounded(TRANSACTIONS, MAX_RECORDS)
            .map_err(map_repository_error)?
        {
            if entry.kind() != RepositoryDirectoryEntryKindV1::Directory || !is_hex(entry.name()) {
                return Err(ambiguous("transaction inventory contains an invalid entry"));
            }
            let transaction = format!("{TRANSACTIONS}/{}", entry.name());
            let intent_path = format!("{transaction}/intent.json");
            let Some(intent_bytes) = self.read_optional(&intent_path, MAX_RECORD_BYTES)? else {
                let entries = self
                    .repository_root
                    .list_directory_bounded(&transaction, MAX_RECORDS)
                    .map_err(map_repository_error)?;
                if entries.iter().any(|entry| {
                    entry.kind() != RepositoryDirectoryEntryKindV1::RegularFile
                        || !entry.name().ends_with(".stage")
                }) {
                    return Err(ambiguous("pre-intent transaction inventory is ambiguous"));
                }
                for entry in entries {
                    self.repository_root
                        .remove_regular_durable(&format!("{transaction}/{}", entry.name()))
                        .map_err(map_repository_error)?;
                }
                self.repository_root
                    .remove_empty_directory_durable(&transaction)
                    .map_err(map_repository_error)?;
                continue;
            };
            let intent: StoreIntentV1 = parse_canonical(&intent_bytes)?;
            validate_intent(&intent, &transaction)?;
            for output in &intent.outputs {
                let stage = self.read_optional(&output.stage, MAX_RECORD_BYTES)?;
                let target = self.read_optional(&output.target, MAX_RECORD_BYTES)?;
                match (stage, target) {
                    (Some(_), Some(_)) => {
                        return Err(ambiguous(
                            "store output exists in staged and installed form",
                        ))
                    }
                    (Some(bytes), None) => {
                        require_output_bytes(output, &bytes)?;
                        self.repository_root
                            .rename_noreplace_durable(&output.stage, &output.target)
                            .map_err(map_repository_error)?;
                    }
                    (None, Some(bytes)) => require_output_bytes(output, &bytes)?,
                    (None, None) => {
                        return Err(ambiguous(
                            "recoverable store output is missing in both forms",
                        ))
                    }
                }
            }
            let marker_path = format!("{transaction}/committed.json");
            match self.read_optional(&marker_path, MAX_RECORD_BYTES)? {
                Some(bytes) => {
                    let marker: CommitMarkerV1 = parse_canonical(&bytes)?;
                    if marker.schema_ref != MARKER_SCHEMA || marker.root != intent.root {
                        return Err(substituted("store commit marker is substituted"));
                    }
                }
                None => {
                    let marker = CommitMarkerV1 {
                        schema_ref: MARKER_SCHEMA.to_owned(),
                        root: intent.root.clone(),
                    };
                    self.repository_root
                        .write_new_durable(
                            &marker_path,
                            &canonical_bytes(&marker).map_err(map_dock_error)?,
                        )
                        .map_err(map_repository_error)?;
                }
            }
        }
        Ok(())
    }

    fn validate_inventory_locked(&self) -> Result<(), DockStoreErrorV1> {
        require_directory_names(
            &self.repository_root,
            DOCK_OPERATIONAL_STORE_RELATIVE_ROOT_V1,
            &[
                "candidates",
                "executions",
                "replays",
                "requests",
                "results",
                "transactions",
            ],
        )?;
        let mut expected = BTreeSet::new();
        let mut seen_roots = BTreeSet::new();
        for entry in self
            .repository_root
            .list_directory_bounded(TRANSACTIONS, MAX_RECORDS)
            .map_err(map_repository_error)?
        {
            if entry.kind() != RepositoryDirectoryEntryKindV1::Directory || !is_hex(entry.name()) {
                return Err(ambiguous("transaction directory identity is invalid"));
            }
            let transaction = format!("{TRANSACTIONS}/{}", entry.name());
            require_directory_names(
                &self.repository_root,
                &transaction,
                &["committed.json", "intent.json"],
            )?;
            let intent: StoreIntentV1 = parse_canonical(
                &self.read_required(&format!("{transaction}/intent.json"), MAX_RECORD_BYTES)?,
            )?;
            validate_intent(&intent, &transaction)?;
            let marker: CommitMarkerV1 = parse_canonical(
                &self.read_required(&format!("{transaction}/committed.json"), MAX_RECORD_BYTES)?,
            )?;
            if marker.schema_ref != MARKER_SCHEMA || marker.root != intent.root {
                return Err(substituted("transaction marker does not match its intent"));
            }
            if !seen_roots.insert(intent.root.as_str().to_owned()) {
                return Err(ambiguous("duplicate transaction root is present"));
            }
            for output in &intent.outputs {
                if !expected.insert(output.target.clone()) {
                    return Err(ambiguous("multiple transactions claim one exact output"));
                }
                let bytes = self.read_required(&output.target, MAX_RECORD_BYTES)?;
                require_output_bytes(output, &bytes)?;
                validate_output_record(&output.target, &bytes)?;
            }
        }
        for directory in [REQUESTS, RESULTS, EXECUTIONS, CANDIDATES, REPLAYS] {
            let actual = self
                .repository_root
                .list_directory_bounded(directory, MAX_RECORDS)
                .map_err(map_repository_error)?;
            let expected_names: BTreeSet<_> = expected
                .iter()
                .filter_map(|path| path.strip_prefix(&format!("{directory}/")))
                .map(str::to_owned)
                .collect();
            let actual_names: BTreeSet<_> = actual
                .iter()
                .map(|entry| {
                    if entry.kind() != RepositoryDirectoryEntryKindV1::RegularFile
                        || !entry.name().ends_with(".json")
                    {
                        return Err(ambiguous(
                            "operational record inventory is not regular JSON",
                        ));
                    }
                    Ok(entry.name().to_owned())
                })
                .collect::<Result<_, DockStoreErrorV1>>()?;
            if actual_names != expected_names {
                return Err(ambiguous(
                    "operational record inventory is missing or surplus",
                ));
            }
        }
        Ok(())
    }

    fn read_replay(
        &self,
        fingerprint: &FingerprintV1,
    ) -> Result<Option<ReplayLedgerV1>, DockStoreErrorV1> {
        self.read_optional(&replay_path(fingerprint), MAX_RECORD_BYTES)?
            .map(|bytes| {
                let ledger: ReplayLedgerV1 = parse_canonical(&bytes)?;
                validate_ledger(&ledger)?;
                Ok(ledger)
            })
            .transpose()
    }

    fn receipt_from_ledger(
        &self,
        ledger: ReplayLedgerV1,
        disposition: DockStoreDispositionV1,
    ) -> Result<DockStoreReceiptV1, DockStoreErrorV1> {
        validate_ledger(&ledger)?;
        let execution: DockExecutionRecordV1 = parse_canonical(&self.read_required(
            &execution_path(ledger.execution.fingerprint()),
            MAX_RECORD_BYTES,
        )?)?;
        if execution.binding().map_err(map_dock_error)? != ledger.execution {
            return Err(substituted("replay execution binding is substituted"));
        }
        validate_run_jcs(
            ledger.evaluation_run_jcs.as_bytes(),
            execution.evaluation_run(),
        )?;
        for candidate in &ledger.candidates {
            let stored: StoredCandidateObservationV1 = parse_canonical(
                &self.read_required(&candidate_path(candidate.fingerprint()), MAX_RECORD_BYTES)?,
            )?;
            validate_stored_candidate(&stored)?;
            if &stored.candidate != candidate {
                return Err(substituted("replay candidate binding is substituted"));
            }
        }
        Ok(DockStoreReceiptV1 {
            disposition,
            root: ledger.root,
            execution_record: execution,
            candidates: ledger.candidates,
            evaluation_run_jcs: ledger.evaluation_run_jcs.into_bytes(),
        })
    }

    fn read_candidate(
        &self,
        candidate: &ContentAddressedBindingV1,
    ) -> Result<StoredCandidateObservationV1, DockStoreErrorV1> {
        let lock = self
            .repository_root
            .acquire_exclusive_lock(LOCK_RELATIVE)
            .map_err(map_repository_error)?;
        self.recover_locked()?;
        self.validate_inventory_locked()?;
        let bytes = self
            .read_optional(&candidate_path(candidate.fingerprint()), MAX_RECORD_BYTES)?
            .ok_or_else(|| missing("exact candidate observation is absent"))?;
        let stored: StoredCandidateObservationV1 = parse_canonical(&bytes)?;
        validate_stored_candidate(&stored)?;
        if &stored.candidate != candidate {
            return Err(substituted("candidate observation binding is substituted"));
        }
        lock.release().map_err(map_repository_error)?;
        Ok(stored)
    }

    pub(crate) fn read_local_candidate_v2(
        &self,
        candidate: &ContentAddressedBindingV1,
    ) -> Result<LocalCandidateStoreObservationV2, DockStoreErrorV1> {
        let lock = self
            .repository_root
            .acquire_existing_exclusive_lock_v1(LOCK_RELATIVE)
            .map_err(map_repository_error)?;
        self.validate_inventory_locked()?;
        let candidate_relative = candidate_path(candidate.fingerprint());
        let stored: StoredCandidateObservationV1 =
            parse_canonical(&self.read_required(&candidate_relative, MAX_RECORD_BYTES)?)?;
        validate_stored_candidate(&stored)?;
        if &stored.candidate != candidate {
            return Err(substituted("candidate observation binding is substituted"));
        }
        let mut ledgers = Vec::new();
        for entry in self
            .repository_root
            .list_directory_bounded(REPLAYS, MAX_RECORDS)
            .map_err(map_repository_error)?
        {
            let relative = format!("{REPLAYS}/{}", entry.name());
            let ledger: ReplayLedgerV1 =
                parse_canonical(&self.read_required(&relative, MAX_RECORD_BYTES)?)?;
            validate_ledger(&ledger)?;
            if ledger.candidates.contains(candidate) {
                ledgers.push((ledger, relative));
            }
        }
        if ledgers.len() != 1 {
            return Err(ambiguous(
                "candidate must have one exact completed native transaction",
            ));
        }
        let (ledger, replay_relative) = ledgers.pop().unwrap();
        let request_relative = request_path(ledger.request.fingerprint());
        let request: DockProcessRequestV1 =
            parse_canonical(&self.read_required(&request_relative, MAX_RECORD_BYTES)?)?;
        let execution_relative = execution_path(ledger.execution.fingerprint());
        let execution: DockExecutionRecordV1 =
            parse_canonical(&self.read_required(&execution_relative, MAX_RECORD_BYTES)?)?;
        let result_binding = ledger
            .result
            .as_ref()
            .ok_or_else(|| substituted("candidate native result is absent"))?;
        let result_relative = result_path(result_binding.fingerprint());
        let result: DockProcessResultV1 =
            parse_canonical(&self.read_required(&result_relative, MAX_RECORD_BYTES)?)?;
        if request.binding().map_err(map_dock_error)? != ledger.request
            || execution.binding().map_err(map_dock_error)? != ledger.execution
            || result.binding().map_err(map_dock_error)? != *result_binding
            || stored.request_record != ledger.request
            || stored.execution_record != ledger.execution
        {
            return Err(substituted(
                "candidate native request or execution closure is substituted",
            ));
        }
        // Reuse the actual writer's closed derivation to check the retained
        // candidate, input roles, full content and result against one another.
        let expected = self.prepare_commit(
            &request,
            Some(&result),
            &execution,
            ledger.evaluation_run_jcs.as_bytes(),
            ledger.key_fingerprint.clone(),
            ledger.idempotency_request_fingerprint.clone(),
        )?;
        if expected.ledger != ledger
            || expected
                .records
                .iter()
                .find(|(path, _)| path == &candidate_relative)
                .map(|(_, bytes)| bytes.as_slice())
                != Some(canonical_bytes(&stored).map_err(map_dock_error)?.as_slice())
        {
            return Err(substituted(
                "candidate is not the exact native completed input derivation",
            ));
        }
        let transaction = transaction_root(&ledger.key_fingerprint);
        let mut exact_paths = vec![
            LOCK_RELATIVE.into(),
            candidate_relative,
            request_relative,
            execution_relative,
            result_relative,
            replay_relative,
            format!("{transaction}/intent.json"),
            format!("{transaction}/committed.json"),
        ];
        exact_paths.sort();
        lock.release().map_err(map_repository_error)?;
        Ok(LocalCandidateStoreObservationV2 {
            stored,
            request,
            exact_paths,
        })
    }

    fn read_optional(
        &self,
        relative: &str,
        bound: usize,
    ) -> Result<Option<Vec<u8>>, DockStoreErrorV1> {
        match self
            .repository_root
            .read_regular_bounded(relative, bound)
            .map_err(map_repository_error)?
        {
            RepositoryReadV1::Found(bytes) => Ok(Some(bytes)),
            RepositoryReadV1::Absent => Ok(None),
        }
    }

    fn read_required(&self, relative: &str, bound: usize) -> Result<Vec<u8>, DockStoreErrorV1> {
        self.read_optional(relative, bound)?
            .ok_or_else(|| ambiguous("required operational record is absent"))
    }
}

/// Currentness of immutable native Dock captures. The expected source identity
/// is checked against the retained request and actual role content each time.
/// This reader neither initializes a missing store nor recovers pending writes.
pub struct DockCapturedInputCurrentnessV1 {
    store: DockOperationalStoreV1,
}

impl DockCapturedInputCurrentnessV1 {
    pub fn open(repo_root: impl AsRef<Path>) -> Result<Self, DockStoreErrorV1> {
        Ok(Self {
            store: DockOperationalStoreV1::open_read_only(repo_root)?,
        })
    }
}

impl SourceCurrentnessPortV1 for DockCapturedInputCurrentnessV1 {
    fn observe_current(
        &mut self,
        expected: &SourceCurrentnessRequestV1,
    ) -> Result<SourceCurrentnessObservationV1, EvidenceSourceAdmissionErrorV1> {
        let checked = (|| -> Result<SourceCurrentnessObservationV1, DockStoreErrorV1> {
            let (request_ref, role) = expected
                .source_ref()
                .split_once("#input/")
                .ok_or_else(|| invalid("source is not a native retained input"))?;
            if role.is_empty() || role.contains('#') || role.contains('/') {
                return Err(invalid("retained input role is malformed"));
            }
            let (_, hex) = request_ref
                .rsplit_once("#request/")
                .ok_or_else(|| invalid("retained request reference is malformed"))?;
            if !is_hex(hex) {
                return Err(invalid("retained request digest is malformed"));
            }
            let fingerprint = FingerprintV1::parse(&format!("sha256:{hex}"))
                .map_err(|_| invalid("retained request fingerprint is malformed"))?;
            let lock = self
                .store
                .repository_root
                .acquire_existing_exclusive_lock_v1(LOCK_RELATIVE)
                .map_err(map_repository_error)?;
            self.store.validate_inventory_locked()?;
            let bytes = self
                .store
                .read_required(&request_path(&fingerprint), MAX_RECORD_BYTES)?;
            let request: DockProcessRequestV1 = parse_canonical(&bytes)?;
            if request.request_ref() != request_ref || request.request_fingerprint() != &fingerprint
            {
                return Err(substituted("retained request identity is substituted"));
            }
            let input = request
                .inputs()
                .iter()
                .find(|input| input.role().as_str() == role)
                .ok_or_else(|| missing("retained input role is absent"))?;
            let actual = canonical_fingerprint(input.content()).map_err(map_dock_error)?;
            if &actual != input.content_fingerprint()
                || actual.as_str() != expected.source_fingerprint()
                || actual.as_str() != expected.revision()
            {
                return Err(substituted(
                    "retained input content or revision is substituted",
                ));
            }
            let observed_at = handbook_contracts::SemanticTimeV1::owner_observation_now_v1()
                .map_err(|_| invalid("native source observation clock is unavailable"))?;
            let result = SourceCurrentnessObservationV1::new(
                expected,
                request.request_ref(),
                request.request_fingerprint().as_str(),
                observed_at.as_str(),
            )
            .map_err(|_| invalid("retained source observation is invalid"))?;
            lock.release().map_err(map_repository_error)?;
            Ok(result)
        })();
        checked.map_err(|_| {
            EvidenceSourceAdmissionErrorV1::repository_observation_refusal_v1(
                EvidenceSourceAdmissionErrorKindV1::ResolutionRefused,
            )
        })
    }
}

pub struct DockEvidenceCandidateSourceV1<P> {
    store: DockOperationalStoreV1,
    admission: EvidenceSourceAdmissionServiceV1<P>,
}

impl<P: NativeAuthenticatorPortV1> DockEvidenceCandidateSourceV1<P> {
    pub fn open(
        repo_root: impl AsRef<Path>,
        port: P,
    ) -> Result<Self, EvidenceCandidateSourceErrorV1> {
        let root = repo_root.as_ref();
        let store = DockOperationalStoreV1::open(root).map_err(map_candidate_store_error)?;
        Ok(Self {
            store,
            admission: EvidenceSourceAdmissionServiceV1::new(root, port),
        })
    }

    pub fn into_port(self) -> P {
        self.admission.into_port()
    }
}

impl<P: NativeAuthenticatorPortV1> EvidenceCandidateSourceV1 for DockEvidenceCandidateSourceV1<P> {
    fn resolve_exact<R>(
        &mut self,
        candidate: &ContentAddressedBindingV1,
        consume: impl for<'authority> FnOnce(ResolvedEvidenceCandidateObservationV1<'authority>) -> R,
    ) -> Result<R, EvidenceCandidateSourceErrorV1> {
        let stored = self
            .store
            .read_candidate(candidate)
            .map_err(map_candidate_store_error)?;
        resolve_repository_candidate_observation_v1(
            &self.store.repository_root,
            &mut self.admission,
            candidate,
            ResolvedEvidenceCandidateObservationInputV1 {
                candidate_jcs: stored.candidate_jcs.into_bytes(),
                evaluation_run_jcs: stored.evaluation_run_jcs.into_bytes(),
                producer: stored.producer,
                request_record: stored.request_record,
                execution_record: stored.execution_record,
                producer_resolution_ceiling: stored.producer_resolution_ceiling,
            },
            consume,
        )
    }
}

struct PreparedTransaction {
    transaction: String,
    records: Vec<(String, Vec<u8>)>,
    intent: StoreIntentV1,
    ledger: ReplayLedgerV1,
}

fn ensure_layout(root: &RetainedRepositoryRootV1) -> Result<(), DockStoreErrorV1> {
    root.create_directories(".handbook/state/locks")
        .map_err(map_repository_error)?;
    for directory in [
        REQUESTS,
        RESULTS,
        EXECUTIONS,
        CANDIDATES,
        REPLAYS,
        TRANSACTIONS,
    ] {
        root.create_directories(directory)
            .map_err(map_repository_error)?;
    }
    Ok(())
}

fn validate_key(key: &str) -> Result<(), DockStoreErrorV1> {
    if !(16..=256).contains(&key.len())
        || !key
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.'))
    {
        return Err(invalid(
            "idempotency key is outside its admitted syntax and bound",
        ));
    }
    Ok(())
}

fn key_fingerprint(key: &str) -> Result<FingerprintV1, DockStoreErrorV1> {
    let mut hasher = Sha256::new();
    hasher.update(b"handbook.dock-run-idempotency-key@1.0.0\0");
    hasher.update(key.as_bytes());
    FingerprintV1::parse(&format!("sha256:{:x}", hasher.finalize()))
        .map_err(|_| invalid("idempotency key fingerprint is invalid"))
}

fn bytes_fingerprint(bytes: &[u8]) -> Result<FingerprintV1, DockStoreErrorV1> {
    FingerprintV1::parse(&format!("sha256:{:x}", Sha256::digest(bytes)))
        .map_err(|_| invalid("record fingerprint is invalid"))
}

fn validate_run_jcs(
    bytes: &[u8],
    binding: &ContentAddressedBindingV1,
) -> Result<Vec<u8>, DockStoreErrorV1> {
    if bytes.is_empty() || bytes.len() > MAX_RECORD_BYTES || std::str::from_utf8(bytes).is_err() {
        return Err(bound(
            "evaluation-run bytes are empty, invalid, or over limit",
        ));
    }
    let value: serde_json::Value =
        serde_json::from_slice(bytes).map_err(|_| invalid("evaluation-run bytes are not JSON"))?;
    if serde_json_canonicalizer::to_vec(&value)
        .map_err(|_| invalid("evaluation-run bytes cannot be canonicalized"))?
        != bytes
    {
        return Err(substituted("evaluation-run bytes are not canonical"));
    }
    if value
        .get("evaluation_run_ref")
        .and_then(serde_json::Value::as_str)
        != Some(binding.reference())
        || value
            .get("evaluation_run_fingerprint")
            .and_then(serde_json::Value::as_str)
            != Some(binding.fingerprint().as_str())
    {
        return Err(substituted(
            "evaluation-run bytes disagree with the request binding",
        ));
    }
    Ok(bytes.to_vec())
}

fn content_from_exact(
    value: &ExactBindingV1,
) -> Result<ContentAddressedBindingV1, DockStoreErrorV1> {
    ContentAddressedBindingV1::new(value.reference().as_str(), value.fingerprint().clone())
        .map_err(|_| invalid("exact binding cannot be used as content identity"))
}

fn validate_stored_candidate(value: &StoredCandidateObservationV1) -> Result<(), DockStoreErrorV1> {
    if value.schema_ref != CANDIDATE_RECORD_SCHEMA {
        return Err(substituted("candidate observation schema is substituted"));
    }
    let candidate_bytes = value.candidate_jcs.as_bytes();
    let candidate: EvidenceCandidateV1 = parse_canonical(candidate_bytes)?;
    if candidate.candidate_ref() != value.candidate.reference()
        || candidate.candidate_fingerprint() != value.candidate.fingerprint()
    {
        return Err(substituted(
            "candidate observation bytes disagree with their binding",
        ));
    }
    let bindings: CandidateEnvelopeBindingsV1 = serde_json::from_slice(candidate_bytes)
        .map_err(|_| substituted("candidate exact bindings are malformed"))?;
    if bindings.body.producer != value.producer
        || bindings.body.request_record != value.request_record
        || bindings.body.execution_record != value.execution_record
    {
        return Err(substituted(
            "candidate observation bindings disagree with its exact body",
        ));
    }
    validate_run_jcs(
        value.evaluation_run_jcs.as_bytes(),
        &bindings.body.evaluation_run,
    )?;
    Ok(())
}

fn validate_ledger(value: &ReplayLedgerV1) -> Result<(), DockStoreErrorV1> {
    if value.schema_ref != REPLAY_SCHEMA {
        return Err(substituted("replay ledger schema is substituted"));
    }
    let expected = canonical_fingerprint(&RootPreimage {
        schema_ref: REPLAY_SCHEMA,
        key_fingerprint: &value.key_fingerprint,
        idempotency_request_fingerprint: &value.idempotency_request_fingerprint,
        request: &value.request,
        result: &value.result,
        execution: &value.execution,
        candidates: &value.candidates,
        evaluation_run_jcs: &value.evaluation_run_jcs,
    })
    .map_err(map_dock_error)?;
    if value.root != expected {
        return Err(substituted("replay ledger root is substituted"));
    }
    Ok(())
}

fn validate_intent(value: &StoreIntentV1, transaction: &str) -> Result<(), DockStoreErrorV1> {
    if value.schema_ref != INTENT_SCHEMA
        || transaction != transaction_root(&value.key_fingerprint)
        || value.outputs.is_empty()
        || value.outputs.len() > MAX_RECORDS
    {
        return Err(ambiguous("store transaction intent identity is invalid"));
    }
    let ledger = ReplayLedgerV1 {
        schema_ref: REPLAY_SCHEMA.to_owned(),
        key_fingerprint: value.key_fingerprint.clone(),
        idempotency_request_fingerprint: value.idempotency_request_fingerprint.clone(),
        request: value.request.clone(),
        result: value.result.clone(),
        execution: value.execution.clone(),
        candidates: value.candidates.clone(),
        evaluation_run_jcs: value.evaluation_run_jcs.clone(),
        root: value.root.clone(),
    };
    validate_ledger(&ledger)?;
    let expected_targets = expected_targets(&ledger);
    let actual_targets: Vec<_> = value
        .outputs
        .iter()
        .map(|output| output.target.clone())
        .collect();
    if actual_targets != expected_targets {
        return Err(ambiguous(
            "store transaction outputs do not match their exact bindings",
        ));
    }
    for (index, output) in value.outputs.iter().enumerate() {
        if output.stage != format!("{transaction}/{index:04}.stage")
            || output.byte_length > MAX_RECORD_BYTES
        {
            return Err(ambiguous("store transaction staging inventory is invalid"));
        }
    }
    Ok(())
}

fn expected_targets(ledger: &ReplayLedgerV1) -> Vec<String> {
    let mut targets = vec![request_path(ledger.request.fingerprint())];
    if let Some(result) = &ledger.result {
        targets.push(result_path(result.fingerprint()));
    }
    targets.push(execution_path(ledger.execution.fingerprint()));
    targets.extend(
        ledger
            .candidates
            .iter()
            .map(|candidate| candidate_path(candidate.fingerprint())),
    );
    targets.push(replay_path(&ledger.key_fingerprint));
    targets
}

fn require_output_bytes(output: &OutputIntentV1, bytes: &[u8]) -> Result<(), DockStoreErrorV1> {
    if bytes.len() != output.byte_length || bytes_fingerprint(bytes)? != output.content_fingerprint
    {
        return Err(substituted("operational output bytes are substituted"));
    }
    Ok(())
}

fn validate_output_record(path: &str, bytes: &[u8]) -> Result<(), DockStoreErrorV1> {
    if path.starts_with(&format!("{REQUESTS}/")) {
        let request: DockProcessRequestV1 = parse_canonical(bytes)?;
        if path != request_path(request.request_fingerprint()) {
            return Err(substituted("request record path is substituted"));
        }
    } else if path.starts_with(&format!("{RESULTS}/")) {
        let result: DockProcessResultV1 = parse_canonical(bytes)?;
        if path != result_path(result.result_fingerprint()) {
            return Err(substituted("result record path is substituted"));
        }
    } else if path.starts_with(&format!("{EXECUTIONS}/")) {
        let execution: DockExecutionRecordV1 = parse_canonical(bytes)?;
        if path != execution_path(execution.record_fingerprint()) {
            return Err(substituted("execution record path is substituted"));
        }
    } else if path.starts_with(&format!("{CANDIDATES}/")) {
        let candidate: StoredCandidateObservationV1 = parse_canonical(bytes)?;
        validate_stored_candidate(&candidate)?;
        if path != candidate_path(candidate.candidate.fingerprint()) {
            return Err(substituted("candidate record path is substituted"));
        }
    } else if path.starts_with(&format!("{REPLAYS}/")) {
        let ledger: ReplayLedgerV1 = parse_canonical(bytes)?;
        validate_ledger(&ledger)?;
        if path != replay_path(&ledger.key_fingerprint) {
            return Err(substituted("replay ledger path is substituted"));
        }
    } else {
        return Err(ambiguous(
            "operational output target is outside the fixed inventory",
        ));
    }
    Ok(())
}

fn parse_canonical<T: for<'de> Deserialize<'de>>(bytes: &[u8]) -> Result<T, DockStoreErrorV1> {
    let value: serde_json::Value =
        serde_json::from_slice(bytes).map_err(|_| substituted("record is not strict JSON"))?;
    if serde_json_canonicalizer::to_vec(&value)
        .map_err(|_| substituted("record cannot be canonicalized"))?
        != bytes
    {
        return Err(substituted("record is not canonical JSON"));
    }
    serde_json::from_value(value).map_err(|_| substituted("record shape or identity is invalid"))
}

fn require_directory_names(
    root: &RetainedRepositoryRootV1,
    relative: &str,
    expected: &[&str],
) -> Result<(), DockStoreErrorV1> {
    let entries = root
        .list_directory_bounded(relative, MAX_RECORDS)
        .map_err(map_repository_error)?;
    let actual: BTreeMap<_, _> = entries
        .into_iter()
        .map(|entry| (entry.name().to_owned(), entry.kind()))
        .collect();
    let expected: BTreeSet<_> = expected.iter().map(|value| (*value).to_owned()).collect();
    if actual.keys().cloned().collect::<BTreeSet<_>>() != expected {
        return Err(ambiguous(
            "directory inventory is missing, surplus, or duplicated",
        ));
    }
    Ok(())
}

fn request_path(fingerprint: &FingerprintV1) -> String {
    format!("{REQUESTS}/{}.json", fingerprint_hex(fingerprint))
}
fn result_path(fingerprint: &FingerprintV1) -> String {
    format!("{RESULTS}/{}.json", fingerprint_hex(fingerprint))
}
fn execution_path(fingerprint: &FingerprintV1) -> String {
    format!("{EXECUTIONS}/{}.json", fingerprint_hex(fingerprint))
}
fn candidate_path(fingerprint: &FingerprintV1) -> String {
    format!("{CANDIDATES}/{}.json", fingerprint_hex(fingerprint))
}
fn replay_path(fingerprint: &FingerprintV1) -> String {
    format!("{REPLAYS}/{}.json", fingerprint_hex(fingerprint))
}
fn transaction_root(fingerprint: &FingerprintV1) -> String {
    format!("{TRANSACTIONS}/{}", fingerprint_hex(fingerprint))
}

fn is_hex(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
}

fn map_dock_error(_: DockErrorV1) -> DockStoreErrorV1 {
    substituted("dock identity or canonicalization was refused")
}

fn map_repository_error(error: RepositoryDurabilityErrorV1) -> DockStoreErrorV1 {
    match error.kind() {
        RepositoryDurabilityErrorKindV1::InvalidRelativePath => {
            invalid("operational repository path is invalid")
        }
        RepositoryDurabilityErrorKindV1::BoundExceeded => {
            bound("operational repository bound exceeded")
        }
        RepositoryDurabilityErrorKindV1::UnsafeFilesystem => {
            unsafe_fs("operational repository filesystem is unsafe")
        }
        RepositoryDurabilityErrorKindV1::AlreadyExists
        | RepositoryDurabilityErrorKindV1::NotFound
        | RepositoryDurabilityErrorKindV1::CurrentValueMismatch => {
            ambiguous("operational repository inventory changed unexpectedly")
        }
        RepositoryDurabilityErrorKindV1::LockUnavailable
        | RepositoryDurabilityErrorKindV1::DurabilityUncertain
        | RepositoryDurabilityErrorKindV1::UnsupportedPlatform => {
            durability("operational repository durability is uncertain")
        }
    }
}

pub(crate) fn map_candidate_store_error(error: DockStoreErrorV1) -> EvidenceCandidateSourceErrorV1 {
    let kind = match error.kind {
        DockStoreErrorKindV1::BoundExceeded => EvidenceCandidateSourceErrorKindV1::OverLimit,
        DockStoreErrorKindV1::RecordSubstituted => {
            EvidenceCandidateSourceErrorKindV1::CandidateSubstituted
        }
        DockStoreErrorKindV1::InvalidRequest => EvidenceCandidateSourceErrorKindV1::Malformed,
        DockStoreErrorKindV1::Missing => EvidenceCandidateSourceErrorKindV1::Missing,
        DockStoreErrorKindV1::IdempotencyConflict
        | DockStoreErrorKindV1::InventoryAmbiguous
        | DockStoreErrorKindV1::DurabilityUncertain
        | DockStoreErrorKindV1::UnsafeFilesystem => EvidenceCandidateSourceErrorKindV1::Unavailable,
    };
    EvidenceCandidateSourceErrorV1::repository_refusal(kind)
}

const fn invalid(detail: &'static str) -> DockStoreErrorV1 {
    DockStoreErrorV1 {
        kind: DockStoreErrorKindV1::InvalidRequest,
        detail,
    }
}
const fn bound(detail: &'static str) -> DockStoreErrorV1 {
    DockStoreErrorV1 {
        kind: DockStoreErrorKindV1::BoundExceeded,
        detail,
    }
}
const fn conflict(detail: &'static str) -> DockStoreErrorV1 {
    DockStoreErrorV1 {
        kind: DockStoreErrorKindV1::IdempotencyConflict,
        detail,
    }
}
const fn substituted(detail: &'static str) -> DockStoreErrorV1 {
    DockStoreErrorV1 {
        kind: DockStoreErrorKindV1::RecordSubstituted,
        detail,
    }
}
const fn ambiguous(detail: &'static str) -> DockStoreErrorV1 {
    DockStoreErrorV1 {
        kind: DockStoreErrorKindV1::InventoryAmbiguous,
        detail,
    }
}
const fn durability(detail: &'static str) -> DockStoreErrorV1 {
    DockStoreErrorV1 {
        kind: DockStoreErrorKindV1::DurabilityUncertain,
        detail,
    }
}
const fn unsafe_fs(detail: &'static str) -> DockStoreErrorV1 {
    DockStoreErrorV1 {
        kind: DockStoreErrorKindV1::UnsafeFilesystem,
        detail,
    }
}
const fn missing(detail: &'static str) -> DockStoreErrorV1 {
    DockStoreErrorV1 {
        kind: DockStoreErrorKindV1::Missing,
        detail,
    }
}
