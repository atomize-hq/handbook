//! Owner-specific native governance records and exact-attempt publication.
//! Semantic interpretation remains in the v1/v2/v3 governance owners.
use super::*;
use handbook_engine::retained_database::{
    open_governance_database, Database, OriginalDisposition, OriginalResolution,
    RetainedDatabaseFs, Transaction, Version,
};
#[cfg(feature = "test-support")]
use handbook_engine::retained_database::{
    open_governance_database_with_creation_boundary, CreationBoundary, PublicationBoundary,
};
use handbook_engine::RepositoryExclusiveLockV1;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

const SCHEMA: &str = "handbook.project-governance-storage@1";
const ROOT_KEY: &str = "governance/root";
const CURRENT_KEY: &str = "governance/current/contracts.evidence-freshness-policy/root";
const PENDING: &str = ".handbook/admission-governance/v2/inflight.json";
const LIMIT: usize = 16 * 1024 * 1024;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct GovernanceOriginalRecord {
    pub schema: String,
    pub id: String,
    pub operation: String,
    pub request_key: String,
    pub claim_fingerprint: String,
    pub store: String,
    pub locator: Vec<u8>,
    pub outcome: Vec<u8>,
    pub noncommit: Vec<u8>,
    pub terminal: Vec<u8>,
    pub relation: Vec<u8>,
    pub predecessor: Option<Vec<u8>>,
    pub facts: Vec<Vec<u8>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct GovernanceCurrentRecord {
    pub schema: String,
    pub id: String,
    pub operation: String,
    pub claim_fingerprint: String,
    pub store: String,
    pub relation: Vec<u8>,
    pub predecessor: Option<Vec<u8>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct GovernanceAttemptEnvelope {
    schema: String,
    original: GovernanceOriginalRecord,
    current: GovernanceCurrentRecord,
    expected: Version,
    transaction: Transaction,
}

#[cfg(feature = "test-support")]
type TestBoundary<'a> = &'a dyn Fn(PublicationBoundary) -> Result<(), GovernanceRefusalV1>;

pub(super) struct GovernanceDatabase<'a> {
    guard: &'a RepositoryInitializationGuardV1,
    native: &'a RepositoryExclusiveLockV1,
    claim: &'a RepositoryInitializationClaimV1,
    database: Database<RetainedDatabaseFs<'a>>,
    #[cfg(feature = "test-support")]
    positive_boundary: Option<TestBoundary<'a>>,
    #[cfg(feature = "test-support")]
    closure_boundary: Option<TestBoundary<'a>>,
}

impl<'a> GovernanceDatabase<'a> {
    pub(super) fn open(
        guard: &'a RepositoryInitializationGuardV1,
        native: &'a RepositoryExclusiveLockV1,
        claim: &'a RepositoryInitializationClaimV1,
        mode: &str,
    ) -> Result<Self, GovernanceRefusalV1> {
        guard.validate().map_err(fs_refusal)?;
        native.validate_at(guard.root(), LOCK).map_err(fs_refusal)?;
        let (database, _) = open_governance_database(guard, claim, mode).map_err(database_error)?;
        // An interrupted selector replacement can leave no readable selected
        // version. Recover under the already-held root and owner locks before
        // any caller constructs an exact record from that version.
        if mode == "recover" {
            database.recover().map_err(database_error)?;
        }
        Ok(Self {
            guard,
            native,
            claim,
            database,
            #[cfg(feature = "test-support")]
            positive_boundary: None,
            #[cfg(feature = "test-support")]
            closure_boundary: None,
        })
    }

    #[cfg(feature = "test-support")]
    pub(super) fn open_with_creation_boundary(
        guard: &'a RepositoryInitializationGuardV1,
        native: &'a RepositoryExclusiveLockV1,
        claim: &'a RepositoryInitializationClaimV1,
        mut boundary: impl FnMut(CreationBoundary) -> Result<(), GovernanceRefusalV1>,
    ) -> Result<Self, GovernanceRefusalV1> {
        guard.validate().map_err(fs_refusal)?;
        native.validate_at(guard.root(), LOCK).map_err(fs_refusal)?;
        let (database, _) = open_governance_database_with_creation_boundary(
            guard, claim, |at| boundary(at).map_err(|error| format!("{error:?}").into()),
        ).map_err(database_error)?;
        Ok(Self { guard, native, claim, database, positive_boundary: None, closure_boundary: None })
    }

    #[cfg(feature = "test-support")]
    pub(super) fn with_test_boundaries(
        mut self,
        positive: Option<TestBoundary<'a>>,
        closure: Option<TestBoundary<'a>>,
    ) -> Self {
        self.positive_boundary = positive;
        self.closure_boundary = closure;
        self
    }

    pub(super) fn records(
        &self,
        operation: &str,
        request_key: &str,
        locator: Vec<u8>,
        outcome: Vec<u8>,
        noncommit: Vec<u8>,
        terminal: Vec<u8>,
        relation: Vec<u8>,
        predecessor: Option<Vec<u8>>,
        facts: Vec<Vec<u8>>,
    ) -> Result<(GovernanceOriginalRecord, GovernanceCurrentRecord), GovernanceRefusalV1> {
        self.validate()?;
        if !matches!(operation, "initial-g" | "initial-d" | "deferred-g") || request_key.is_empty()
        {
            return Err(refusal(
                "request_identity",
                "Supported original operation and key required",
            ));
        }
        let id = governance_fingerprint(
            "handbook.s06-request-key@1",
            &(operation, self.claim.claim_fingerprint(), request_key),
        )?
        .as_str()
        .strip_prefix("sha256:")
        .ok_or_else(|| refusal("request_identity", "Exact request fingerprint required"))?
        .to_owned();
        let store = store_of(&self.database.version().map_err(database_error)?)?;
        let original = GovernanceOriginalRecord {
            schema: SCHEMA.into(),
            id: id.clone(),
            operation: operation.into(),
            request_key: request_key.into(),
            claim_fingerprint: self.claim.claim_fingerprint().into(),
            store: store.clone(),
            locator,
            outcome,
            noncommit,
            terminal,
            relation: relation.clone(),
            predecessor: predecessor.clone(),
            facts,
        };
        let current = GovernanceCurrentRecord {
            schema: SCHEMA.into(),
            id,
            operation: operation.into(),
            claim_fingerprint: self.claim.claim_fingerprint().into(),
            store,
            relation,
            predecessor,
        };
        Ok((original, current))
    }

    fn validate(&self) -> Result<(), GovernanceRefusalV1> {
        self.guard.validate().map_err(fs_refusal)?;
        self.native
            .validate_at(self.guard.root(), LOCK)
            .map_err(fs_refusal)?;
        if !self
            .guard
            .observe_native_claim(self.claim)
            .map_err(fs_refusal)?
        {
            return Err(refusal(
                "claim_missing",
                "Original governance claim is unavailable",
            ));
        }
        Ok(())
    }
}

fn database_error(error: impl std::fmt::Display) -> GovernanceRefusalV1 {
    refusal("governance_database", &error.to_string())
}

fn bytes<T: Serialize>(value: &T) -> Result<Vec<u8>, GovernanceRefusalV1> {
    let result = serde_json_canonicalizer::to_vec(value).map_err(database_error)?;
    if result.len() > LIMIT {
        return Err(refusal(
            "record_bound",
            "Governance database record exceeds its bound",
        ));
    }
    Ok(result)
}

fn decode<T: for<'de> Deserialize<'de> + Serialize>(
    input: &[u8],
) -> Result<T, GovernanceRefusalV1> {
    if input.len() > LIMIT {
        return Err(refusal(
            "record_bound",
            "Governance database record exceeds its bound",
        ));
    }
    let value: T = serde_json::from_slice(input).map_err(database_error)?;
    if bytes(&value)? != input {
        return Err(refusal(
            "record_identity",
            "Exact canonical governance bytes required",
        ));
    }
    Ok(value)
}

fn store_of(version: &Version) -> Result<String, GovernanceRefusalV1> {
    serde_json::to_value(version)
        .map_err(database_error)?
        .get("store")
        .and_then(serde_json::Value::as_str)
        .filter(|value| value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit()))
        .map(str::to_owned)
        .ok_or_else(|| refusal("database_binding", "Exact native store identity required"))
}

fn original_key(id: &str) -> String {
    format!("governance/original/{id}")
}
fn record_key(id: &str) -> String {
    format!("governance/request/{id}")
}
fn closure_key(id: &str) -> String {
    format!("governance/closure/{id}")
}
fn closure_path(id: &str) -> String {
    format!(".handbook/admission-governance/v2/closure-{id}.json")
}
fn valid_id(id: &str) -> bool {
    id.len() == 64
        && id
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
}
fn validate_records(
    db: &GovernanceDatabase<'_>,
    original: &GovernanceOriginalRecord,
    current: &GovernanceCurrentRecord,
    version: &Version,
) -> Result<(), GovernanceRefusalV1> {
    let fingerprint = governance_fingerprint(
        "handbook.s06-request-key@1",
        &(
            &original.operation,
            db.claim.claim_fingerprint(),
            &original.request_key,
        ),
    )?;
    let expected_id = fingerprint
        .as_str()
        .strip_prefix("sha256:")
        .ok_or_else(|| refusal("request_identity", "Exact request fingerprint required"))?;
    if original.id != expected_id
        || original.request_key.is_empty()
        || original.schema != SCHEMA
        || current.schema != SCHEMA
        || !valid_id(&original.id)
        || original.id != current.id
        || original.operation != current.operation
        || original.claim_fingerprint != db.claim.claim_fingerprint()
        || current.claim_fingerprint != original.claim_fingerprint
        || original.store != store_of(version)?
        || current.store != original.store
        || current.relation != original.relation
        || current.predecessor != original.predecessor
        || original.locator.is_empty()
        || original.outcome.is_empty()
        || original.noncommit.is_empty()
        || original.terminal.is_empty()
        || original.relation.is_empty()
    {
        return Err(refusal(
            "database_binding",
            "Exact owner attempt and current relation required",
        ));
    }
    Ok(())
}

fn transaction_for(
    db: &GovernanceDatabase<'_>,
    expected: &Version,
    original: &GovernanceOriginalRecord,
    current: &GovernanceCurrentRecord,
) -> Result<Transaction, GovernanceRefusalV1> {
    validate_records(db, original, current, expected)?;
    let mut transaction = Transaction::new(
        expected.clone(),
        original_key(&original.id),
        original.outcome.clone(),
    );
    transaction
        .set_evidence(db.claim.claim_fingerprint(), bytes(original)?)
        .map_err(database_error)?;
    transaction
        .put(ROOT_KEY, bytes(&(SCHEMA, db.claim, &original.store))?)
        .map_err(database_error)?;
    transaction
        .put(record_key(&original.id), bytes(original)?)
        .map_err(database_error)?;
    transaction
        .put(CURRENT_KEY, bytes(current)?)
        .map_err(database_error)?;
    let sequence = expected
        .sequence
        .checked_add(1)
        .ok_or_else(|| refusal("history_bound", "Governance history sequence overflow"))?;
    transaction
        .put(
            format!("governance/history/{sequence:016x}"),
            bytes(&(&original.id, &original.operation, &original.terminal))?,
        )
        .map_err(database_error)?;
    Ok(transaction)
}

/// Persists the complete deterministic native attempt before submitting it.
/// Resolution never fabricates a terminal from an absent native original.
pub(super) fn publish_original(
    db: &GovernanceDatabase<'_>,
    original: GovernanceOriginalRecord,
    current: GovernanceCurrentRecord,
    before_submission: impl FnOnce() -> Result<(), GovernanceRefusalV1>,
) -> Result<OriginalDisposition, GovernanceRefusalV1> {
    db.validate()?;
    if !valid_id(&original.id) {
        return Err(refusal(
            "request_identity",
            "Exact request identity required",
        ));
    }
    if db
        .guard
        .root()
        .read_regular_bounded(&closure_path(&original.id), LIMIT)
        .map_err(fs_refusal)?
        != RepositoryReadV1::Absent
    {
        return Err(refusal(
            "positive_fenced",
            "Original positive submission is permanently fenced",
        ));
    }
    if !matches!(
        db.database
            .resolve(&closure_key(&original.id))
            .map_err(database_error)?,
        OriginalResolution::Absent
    ) {
        return Err(refusal(
            "positive_fenced",
            "Original closure already owns the request",
        ));
    }
    let root = bytes(&(SCHEMA, db.claim, &original.store))?;
    let expected = db
        .database
        .with_current(|view| -> Result<Version, GovernanceRefusalV1> {
            let version = view.version().clone();
            validate_records(db, &original, &current, &version)?;
            match view.get(ROOT_KEY).map_err(database_error)? {
                Some(existing) if existing == root => {}
                None if version.sequence == 0 => {}
                _ => {
                    return Err(refusal(
                        "database_binding",
                        "Original governance database root differs",
                    ))
                }
            }
            match (
                view.get(CURRENT_KEY).map_err(database_error)?,
                &original.predecessor,
            ) {
                (None, None) if version.sequence == 0 => {}
                (Some(selected), Some(predecessor)) => {
                    let selected: GovernanceCurrentRecord = decode(&selected)?;
                    if selected.relation != *predecessor {
                        return Err(refusal(
                            "basis_conflict",
                            "Exact current predecessor differs",
                        ));
                    }
                }
                _ => {
                    return Err(refusal(
                        "basis_conflict",
                        "Exact current predecessor is unavailable",
                    ))
                }
            }
            Ok(version)
        })
        .map_err(database_error)??;
    let transaction = transaction_for(db, &expected, &original, &current)?;
    let envelope = GovernanceAttemptEnvelope {
        schema: SCHEMA.into(),
        original,
        current,
        expected,
        transaction,
    };
    let saved = bytes(&envelope)?;
    match db
        .guard
        .root()
        .read_regular_bounded(PENDING, LIMIT)
        .map_err(fs_refusal)?
    {
        RepositoryReadV1::Absent => db
            .guard
            .root()
            .write_new_durable(PENDING, &saved)
            .map_err(fs_refusal)?,
        RepositoryReadV1::Found(_) => {
            return Err(refusal(
                "attempt_pending",
                "Resolve the existing original attempt before any later submission",
            ))
        }
    }
    db.validate()?;
    before_submission()?;
    #[cfg(feature = "test-support")]
    let result = if let Some(boundary) = db.positive_boundary {
        db.database.publish_guarded_with_boundary(envelope.transaction, || {
            db.guard.validate()?;
            db.native.validate_at(db.guard.root(), LOCK)?;
            Ok(())
        }, |at| boundary(at).map_err(|error| format!("{error:?}").into()))
    } else {
        db.database.publish_guarded(envelope.transaction, || {
            db.guard.validate()?;
            db.native.validate_at(db.guard.root(), LOCK)?;
            Ok(())
        })
    }.map_err(database_error)?;
    #[cfg(not(feature = "test-support"))]
    let result = db.database.publish_guarded(envelope.transaction, || {
        db.guard.validate()?;
        db.native.validate_at(db.guard.root(), LOCK)?;
        Ok(())
    }).map_err(database_error)?;
    db.validate()?;
    Ok(result.disposition)
}

/// Retrieves native evidence for one exact saved original, without changing
/// mutable current truth or treating absence as nonapplication proof.
pub(super) fn resolve_original(
    db: &GovernanceDatabase<'_>,
    id: &str,
    expected_locator: &[u8],
) -> Result<(OriginalDisposition, GovernanceOriginalRecord), GovernanceRefusalV1> {
    db.validate()?;
    if !valid_id(id) {
        return Err(refusal(
            "request_identity",
            "Exact request identity required",
        ));
    }
    db.database.recover().map_err(database_error)?;
    match db
        .database
        .resolve(&original_key(id))
        .map_err(database_error)?
    {
        OriginalResolution::Absent => {
            return match close_noncommit(db, id, expected_locator, false) {
                Ok(original) => Ok((OriginalDisposition::NotSelected, original)),
                Err(error) if error.condition == "closure_missing" => Err(refusal(
                    "original_absent",
                    "Native original is not positively closed",
                )),
                Err(error) => Err(error),
            };
        }
        OriginalResolution::Pending => {}
        OriginalResolution::Resolved(_) => {}
    }
    let observation = db
        .database
        .reconcile_evidence(db.claim.claim_fingerprint(), Some(&original_key(id)))
        .map_err(database_error)?;
    let evidence = observation.original.ok_or_else(|| {
        refusal(
            "original_missing",
            "Native original evidence is unavailable",
        )
    })?;
    let original: GovernanceOriginalRecord = decode(&evidence.evidence.payload)?;
    if original.id != id
        || original.locator != expected_locator
        || original.claim_fingerprint != db.claim.claim_fingerprint()
        || original.store != evidence.evidence.binding.store
    {
        return Err(refusal(
            "request_conflict",
            "Native original differs from the complete request",
        ));
    }
    let current = GovernanceCurrentRecord {
        schema: SCHEMA.into(),
        id: original.id.clone(),
        operation: original.operation.clone(),
        claim_fingerprint: original.claim_fingerprint.clone(),
        store: original.store.clone(),
        relation: original.relation.clone(),
        predecessor: original.predecessor.clone(),
    };
    let transaction =
        transaction_for(db, &evidence.evidence.binding.expected, &original, &current)?;
    let serialized = serde_json::to_vec(&transaction).map_err(database_error)?;
    if format!("{:x}", Sha256::digest(serialized)) != evidence.reference.fingerprint {
        return Err(refusal(
            "original_conflict",
            "Native original transaction differs from exact owner facts",
        ));
    }
    if evidence.outcome.disposition == OriginalDisposition::NotSelected
        && evidence.evidence.binding.expected != evidence.evidence.binding.predecessor
    {
        return Err(refusal(
            "original_conflict",
            "Native nonselection did not use the exact predecessor",
        ));
    }
    if evidence.outcome.disposition == OriginalDisposition::Selected {
        let saved = db
            .database
            .get(&record_key(id))
            .map_err(database_error)?
            .ok_or_else(|| refusal("original_missing", "Selected owner original is unavailable"))?;
        if saved != bytes(&original)? || evidence.outcome.value != original.outcome {
            return Err(refusal(
                "original_conflict",
                "Selected owner original differs",
            ));
        }
    }
    db.validate()?;
    Ok((evidence.outcome.disposition, original))
}

/// Reads root, current relation and its exact original in one selected view.
pub(super) fn resolve_current(
    db: &GovernanceDatabase<'_>,
) -> Result<(GovernanceCurrentRecord, GovernanceOriginalRecord), GovernanceRefusalV1> {
    db.validate()?;
    let result = db
        .database
        .with_current(|view| -> Result<_, GovernanceRefusalV1> {
            let root = view
                .get(ROOT_KEY)
                .map_err(database_error)?
                .ok_or_else(|| refusal("database_binding", "Governance root missing"))?;
            let current: GovernanceCurrentRecord = decode(
                &view
                    .get(CURRENT_KEY)
                    .map_err(database_error)?
                    .ok_or_else(|| {
                        refusal("governance_currentness", "Current governance missing")
                    })?,
            )?;
            let original: GovernanceOriginalRecord = decode(
                &view
                    .get(&record_key(&current.id))
                    .map_err(database_error)?
                    .ok_or_else(|| refusal("original_missing", "Current original missing"))?,
            )?;
            validate_records(db, &original, &current, view.version())?;
            if root != bytes(&(SCHEMA, db.claim, &original.store))? {
                return Err(refusal(
                    "database_binding",
                    "Current governance root differs",
                ));
            }
            Ok((current, original))
        })
        .map_err(database_error)??;
    let (disposition, retained) = resolve_original(db, &result.0.id, &result.1.locator)?;
    if disposition != OriginalDisposition::Selected || bytes(&retained)? != bytes(&result.1)? {
        return Err(refusal(
            "original_conflict",
            "Current governance lacks its selected original",
        ));
    }
    db.validate()?;
    Ok(result)
}

/// Retires only the exact saved attempt after the owner has independently
/// resolved its native original and any required initialization receipt.
pub(super) fn recover_original(
    db: &GovernanceDatabase<'_>,
    expected: &GovernanceOriginalRecord,
    disposition: OriginalDisposition,
    terminal: Option<(&str, &[u8], RepositoryInitializationDispositionV1)>,
) -> Result<(), GovernanceRefusalV1> {
    let (observed, original) = resolve_original(db, &expected.id, &expected.locator)?;
    if observed != disposition || bytes(&original)? != bytes(expected)? {
        return Err(refusal(
            "original_conflict",
            "Exact completed owner original differs",
        ));
    }
    if disposition == OriginalDisposition::NotSelected {
        let closed = close_noncommit(db, &expected.id, &expected.locator, false)?;
        if bytes(&closed)? != bytes(expected)? {
            return Err(refusal(
                "original_conflict",
                "Exact noncommit closure differs",
            ));
        }
    }
    if let Some((path, terminal_bytes, claim_disposition)) = terminal {
        db.guard
            .require_native_terminal(db.claim, claim_disposition, path, terminal_bytes)
            .map_err(fs_refusal)?;
    }
    let pending = match db
        .guard
        .root()
        .read_regular_bounded(PENDING, LIMIT)
        .map_err(fs_refusal)?
    {
        RepositoryReadV1::Absent => return Ok(()),
        RepositoryReadV1::Found(bytes) => bytes,
    };
    let envelope: GovernanceAttemptEnvelope = decode(&pending)?;
    if envelope.original.id != expected.id {
        return Ok(());
    }
    if envelope.schema != SCHEMA
        || bytes(&envelope.original)? != bytes(expected)?
        || bytes(&envelope.transaction)?
            != bytes(&transaction_for(
                db,
                &envelope.expected,
                &envelope.original,
                &envelope.current,
            )?)?
    {
        return Err(refusal("attempt_conflict", "Saved exact attempt differs"));
    }
    db.validate()?;
    db.guard
        .root()
        .remove_regular_durable(PENDING)
        .map_err(fs_refusal)?;
    db.validate()
}

fn closure_transaction(
    db: &GovernanceDatabase<'_>,
    expected: &Version,
    original: &GovernanceOriginalRecord,
) -> Result<Transaction, GovernanceRefusalV1> {
    let mut transaction = Transaction::new(
        expected.clone(),
        closure_key(&original.id),
        original.noncommit.clone(),
    );
    transaction
        .set_evidence(db.claim.claim_fingerprint(), bytes(original)?)
        .map_err(database_error)?;
    transaction
        .put(record_key(&original.id), bytes(original)?)
        .map_err(database_error)?;
    let sequence = expected
        .sequence
        .checked_add(1)
        .ok_or_else(|| refusal("history_bound", "Governance history sequence overflow"))?;
    transaction
        .put(
            format!("governance/history/{sequence:016x}"),
            bytes(&(
                &original.id,
                &original.operation,
                "not_committed",
                &original.noncommit,
            ))?,
        )
        .map_err(database_error)?;
    Ok(transaction)
}

/// Fences the original positive submission before attempting one exact native
/// noncommit closure. Both its selected and metadata-only negative branches
/// retain complete bound original evidence; an absent native key alone proves
/// nothing. A retained fence is never removed.
pub(super) fn close_noncommit(
    db: &GovernanceDatabase<'_>,
    id: &str,
    locator: &[u8],
    publish: bool,
) -> Result<GovernanceOriginalRecord, GovernanceRefusalV1> {
    db.validate()?;
    if !valid_id(id) {
        return Err(refusal(
            "request_identity",
            "Exact request identity required",
        ));
    }
    db.database.recover().map_err(database_error)?;
    let path = closure_path(id);
    let retained = db
        .guard
        .root()
        .read_regular_bounded(&path, LIMIT)
        .map_err(fs_refusal)?;
    let (schema, envelope, closure_expected, transaction): (
        String,
        GovernanceAttemptEnvelope,
        Version,
        Transaction,
    ) = match retained {
        RepositoryReadV1::Found(saved) => decode(&saved)?,
        RepositoryReadV1::Absent if !publish => {
            return Err(refusal(
                "closure_missing",
                "No retained positive-submission fence",
            ))
        }
        RepositoryReadV1::Absent => {
            let pending = match db
                .guard
                .root()
                .read_regular_bounded(PENDING, LIMIT)
                .map_err(fs_refusal)?
            {
                RepositoryReadV1::Found(value) => value,
                RepositoryReadV1::Absent => {
                    return Err(refusal(
                        "attempt_missing",
                        "Exact original attempt is unavailable",
                    ))
                }
            };
            let envelope: GovernanceAttemptEnvelope = decode(&pending)?;
            if envelope.schema != SCHEMA
                || envelope.original.id != id
                || envelope.original.locator != locator
                || bytes(&envelope.transaction)?
                    != bytes(&transaction_for(
                        db,
                        &envelope.expected,
                        &envelope.original,
                        &envelope.current,
                    )?)?
            {
                return Err(refusal(
                    "attempt_conflict",
                    "Exact retained original attempt differs",
                ));
            }
            let current = db.database.version().map_err(database_error)?;
            match db
                .database
                .resolve(&original_key(id))
                .map_err(database_error)?
            {
                OriginalResolution::Absent if current == envelope.expected => {}
                OriginalResolution::Resolved(outcome)
                    if outcome.disposition == OriginalDisposition::NotSelected =>
                {
                    let observation = db
                        .database
                        .reconcile_evidence(db.claim.claim_fingerprint(), Some(&original_key(id)))
                        .map_err(database_error)?;
                    let proof = observation.original.ok_or_else(|| {
                        refusal("original_missing", "Native original evidence missing")
                    })?;
                    let serialized =
                        serde_json::to_vec(&envelope.transaction).map_err(database_error)?;
                    if proof.evidence.payload != bytes(&envelope.original)?
                        || proof.evidence.binding.expected != envelope.expected
                        || proof.evidence.binding.predecessor != envelope.expected
                        || proof.reference.fingerprint
                            != format!("{:x}", Sha256::digest(serialized))
                        || observation.at.sequence != proof.evidence.binding.resolving.sequence
                        || observation.at != current
                    {
                        return Err(refusal(
                            "original_conflict",
                            "Exact native nonselection evidence differs",
                        ));
                    }
                }
                OriginalResolution::Resolved(outcome)
                    if outcome.disposition == OriginalDisposition::Selected =>
                {
                    return Err(refusal(
                        "original_selected",
                        "Selected positive original cannot be closed as noncommit",
                    ))
                }
                _ => {
                    return Err(refusal(
                        "original_uncertain",
                        "Native original does not positively permit closure",
                    ))
                }
            }
            let transaction = closure_transaction(db, &current, &envelope.original)?;
            let marker = (SCHEMA.to_owned(), envelope, current, transaction);
            let marker_bytes = bytes(&marker)?;
            db.validate()?;
            db.guard
                .root()
                .write_new_durable(&path, &marker_bytes)
                .map_err(fs_refusal)?;
            marker
        }
    };
    if schema != SCHEMA
        || envelope.original.id != id
        || envelope.original.locator != locator
        || bytes(&envelope.transaction)?
            != bytes(&transaction_for(
                db,
                &envelope.expected,
                &envelope.original,
                &envelope.current,
            )?)?
        || bytes(&transaction)?
            != bytes(&closure_transaction(
                db,
                &closure_expected,
                &envelope.original,
            )?)?
    {
        return Err(refusal(
            "closure_conflict",
            "Exact saved closure and original attempt differ",
        ));
    }
    match db
        .database
        .resolve(&original_key(id))
        .map_err(database_error)?
    {
        OriginalResolution::Absent if closure_expected == envelope.expected => {}
        OriginalResolution::Resolved(outcome)
            if outcome.disposition == OriginalDisposition::NotSelected =>
        {
            let positive = db
                .database
                .reconcile_evidence(db.claim.claim_fingerprint(), Some(&original_key(id)))
                .map_err(database_error)?
                .original
                .ok_or_else(|| {
                    refusal(
                        "original_missing",
                        "Exact positive original evidence is missing",
                    )
                })?;
            let positive_input =
                serde_json::to_vec(&envelope.transaction).map_err(database_error)?;
            if positive.evidence.payload != bytes(&envelope.original)?
                || positive.evidence.binding.expected != envelope.expected
                || positive.evidence.binding.predecessor != envelope.expected
                || positive.evidence.binding.resolving.sequence != closure_expected.sequence
                || positive.evidence.binding.store != store_of(&closure_expected)?
                || positive.reference.fingerprint != format!("{:x}", Sha256::digest(positive_input))
            {
                return Err(refusal(
                    "original_conflict",
                    "Positive nonselection proof differs",
                ));
            }
        }
        _ => {
            return Err(refusal(
                "original_uncertain",
                "Positive original lacks exact nonapplication proof",
            ))
        }
    }
    let closure_key = closure_key(id);
    let resolution = db.database.resolve(&closure_key).map_err(database_error)?;
    if matches!(resolution, OriginalResolution::Absent) && publish {
        #[cfg(feature = "test-support")]
        if let Some(boundary) = db.closure_boundary {
            db.database.publish_guarded_with_boundary(transaction.clone(), || {
                db.guard.validate()?;
                db.native.validate_at(db.guard.root(), LOCK)?;
                Ok(())
            }, |at| boundary(at).map_err(|error| format!("{error:?}").into()))
                .map_err(database_error)?;
        } else {
            db.database.publish_guarded(transaction.clone(), || {
                db.guard.validate()?;
                db.native.validate_at(db.guard.root(), LOCK)?;
                Ok(())
            }).map_err(database_error)?;
        }
        #[cfg(not(feature = "test-support"))]
        db.database.publish_guarded(transaction.clone(), || {
            db.guard.validate()?;
            db.native.validate_at(db.guard.root(), LOCK)?;
            Ok(())
        }).map_err(database_error)?;
    } else if matches!(
        resolution,
        OriginalResolution::Absent | OriginalResolution::Pending
    ) {
        return Err(refusal(
            "noncommit_closure_pending",
            "Exact closure has not completed",
        ));
    }
    db.database.recover().map_err(database_error)?;
    let observation = db
        .database
        .reconcile_evidence(db.claim.claim_fingerprint(), Some(&closure_key))
        .map_err(database_error)?;
    let proof = observation
        .original
        .ok_or_else(|| refusal("closure_missing", "Native closure evidence is unavailable"))?;
    let serialized = serde_json::to_vec(&transaction).map_err(database_error)?;
    if proof.evidence.payload != bytes(&envelope.original)?
        || proof.evidence.binding.expected != closure_expected
        || proof.evidence.binding.predecessor != closure_expected
        || proof.reference.fingerprint != format!("{:x}", Sha256::digest(serialized))
        || proof.evidence.binding.store != envelope.original.store
    {
        return Err(refusal(
            "closure_conflict",
            "Native closure evidence differs from the exact fence",
        ));
    }
    match proof.outcome.disposition {
        OriginalDisposition::Selected => {
            if proof.outcome.value != envelope.original.noncommit
                || db
                    .database
                    .get(&record_key(id))
                    .map_err(database_error)?
                    .as_deref()
                    != Some(bytes(&envelope.original)?.as_slice())
            {
                return Err(refusal(
                    "closure_conflict",
                    "Selected noncommit closure differs",
                ));
            }
        }
        OriginalDisposition::NotSelected => {}
        OriginalDisposition::Conflict => {
            return Err(refusal(
                "closure_conflict",
                "Closure predecessor conflicted",
            ))
        }
    }
    db.validate()?;
    Ok(envelope.original)
}
