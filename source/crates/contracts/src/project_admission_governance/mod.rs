//! One native new-project establishment. No candidate admission or replacement API.
//!
//! Lock order: owner process mutex, Engine root initialization guard, retained
//! native governance lock, then Engine
//! prepared identity persistence (which takes no registry/Context Resolution lock).
//! The immutable reservation anchor is never reset. Its consumption is an append
//! in the same decided transaction as activation; usable truth requires terminal
//! closure of every installed fact. Recovery does not allocate an identity.

#[cfg(test)]
mod owner_tests;
mod semantic_types;
mod storage;
mod semantic;
pub use semantic_types::*;
pub use semantic::*;
mod types;
pub use types::*;
mod deferral;
mod deferral_types;
pub use deferral::*;
pub use deferral_types::*;
mod scope;
pub use scope::*;
mod scope_v3;
mod transition;
mod transition_types;
pub use scope_v3::*;
pub use transition::*;
pub use transition_types::*;

use crate::FingerprintV1;
use handbook_engine::{
    PreparedRepositoryIdentityV1, RepositoryDirectoryEntryKindV1, RepositoryDurabilityErrorKindV1,
    RepositoryInitializationClaimV1, RepositoryInitializationDispositionV1,
    RepositoryInitializationGuardV1, RepositoryInvocationIdentityServiceV1, RepositoryReadV1,
    RetainedRepositoryRootV1, REPOSITORY_IDENTITY_REPO_PATH,
};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::Mutex;

pub const GOVERNANCE_ANCHOR_PATH_V1: &str = ".handbook/admission-governance-project.v1.json";
pub const GOVERNANCE_STORAGE_PATH_V1: &str = ".handbook/admission-governance/v1";
pub const GOVERNANCE_SCOPE_V1: &str = ADMISSION_GOVERNANCE_SCOPE_V1;
const LOCK: &str = ".handbook/admission-governance.lock";
const IDENTITY_TEMP: &str = ".handbook/.repository-identity.v1.tmp";
const MAX_BYTES: usize = 1024 * 1024;
static PROCESS_LOCK: Mutex<()> = Mutex::new(());
const TARGETS: [&str; 5] = [
    "rules.json",
    "configuration.json",
    "outcome.json",
    "consumed.json",
    "active.json",
];

/// A creation capability is neither copyable nor restorable from its locator.
#[derive(Debug)]
pub struct PreparedAdmissionGovernanceV1 {
    locator: GovernanceRequestLocatorV1,
    identity: PreparedRepositoryIdentityV1,
}
impl PreparedAdmissionGovernanceV1 {
    pub fn locator(&self) -> &GovernanceRequestLocatorV1 {
        &self.locator
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceDeliveryV1 {
    Original,
    Replay,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GovernanceCommittedOutcomeV1 {
    pub locator: GovernanceRequestLocatorV1,
    pub validated: ValidatedGovernanceV1,
    pub validation_confirmed: bool,
    pub advisory_permission_accepted: bool,
    pub activation_committed: bool,
    pub replacement_basis: ReplacementAuthorityBasisV1,
    pub outcome_fingerprint: FingerprintV1,
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GovernanceNotCommittedOutcomeV1 {
    pub locator: GovernanceRequestLocatorV1,
    pub condition: String,
    pub activation_committed: bool,
    pub outcome_fingerprint: FingerprintV1,
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case", deny_unknown_fields)]
pub enum GovernanceExecutionV1 {
    Committed {
        outcome: GovernanceCommittedOutcomeV1,
        delivery: GovernanceDeliveryV1,
    },
    Refused {
        refusal: GovernanceRefusalV1,
    },
    NotCommitted {
        outcome: GovernanceNotCommittedOutcomeV1,
        delivery: GovernanceDeliveryV1,
    },
    Uncertain {
        locator: GovernanceRequestLocatorV1,
        condition: String,
    },
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExpectedGovernanceBasisV1 {
    pub configuration_fingerprint: FingerprintV1,
    pub outcome_fingerprint: FingerprintV1,
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ActiveGovernanceBasisV1 {
    pub project: ProjectBindingV1,
    pub scope: String,
    pub configuration: AdmissionConfigurationInputV1,
    pub configuration_fingerprint: FingerprintV1,
    pub establishment_rule: GovernanceRuleDescriptorV1,
    pub owner_rule: GovernanceRuleDescriptorV1,
    pub outcome_fingerprint: FingerprintV1,
    pub active_fingerprint: FingerprintV1,
}

/// Test boundaries correspond to durable product facts, including a lost reply.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[doc(hidden)]
pub enum GovernanceFaultBoundaryV1 {
    #[cfg(feature = "test-support")]
    NativeCreate(handbook_engine::retained_database::CreationBoundary),
    #[cfg(feature = "test-support")]
    NativePublish(handbook_engine::retained_database::PublicationBoundary),
    #[cfg(feature = "test-support")]
    NativeClosure(handbook_engine::retained_database::PublicationBoundary),
    Reservation,
    Anchor,
    Identity,
    Registration,
    Stage(usize),
    Intent,
    Decision,
    Installation(usize),
    Terminal,
    Reply,
}
#[derive(Default)]
pub(crate) struct ProjectAdmissionGovernanceV1 {
    #[cfg(feature = "test-support")]
    fault: Option<GovernanceFaultBoundaryV1>,
    #[cfg(feature = "test-support")]
    crash: bool,
    #[cfg(feature = "test-support")]
    pause: Option<(GovernanceFaultBoundaryV1, std::path::PathBuf, std::path::PathBuf)>,
}
impl ProjectAdmissionGovernanceV1 {
    pub fn new() -> Self {
        Self::default()
    }
    #[cfg(feature = "test-support")]
    #[doc(hidden)]
    pub fn with_fault_for_testing(mut self, fault: GovernanceFaultBoundaryV1) -> Self {
        self.fault = Some(fault);
        self
    }
    #[cfg(feature = "test-support")]
    #[doc(hidden)]
    pub fn with_crash_for_testing(mut self, fault: GovernanceFaultBoundaryV1) -> Self {
        self.fault = Some(fault);
        self.crash = true;
        self
    }
    #[cfg(feature = "test-support")]
    #[doc(hidden)]
    pub fn with_pause_for_testing(
        mut self, boundary: GovernanceFaultBoundaryV1,
        ready: std::path::PathBuf, release: std::path::PathBuf,
    ) -> Self {
        self.pause = Some((boundary, ready, release));
        self
    }
    fn boundary(&self, boundary: GovernanceFaultBoundaryV1) -> Result<(), GovernanceRefusalV1> {
        #[cfg(feature = "test-support")]
        if let Some((at, ready, release)) = &self.pause {
            if *at == boundary {
                std::fs::write(ready, b"ready").map_err(|_| refusal("pause_failed", "Pause rendezvous unavailable"))?;
                let start = std::time::Instant::now();
                while !release.exists() {
                    if start.elapsed() > std::time::Duration::from_secs(30) {
                        return Err(refusal("pause_timeout", "Pause release unavailable"));
                    }
                    std::thread::sleep(std::time::Duration::from_millis(10));
                }
            }
        }
        #[cfg(feature = "test-support")]
        if self.fault == Some(boundary) {
            if self.crash {
                std::process::exit(71);
            }
            return Err(refusal(
                "injected_interruption",
                "Resolve the exact original request",
            ));
        }
        let _ = boundary;
        Ok(())
    }

    /// Allocates a proposed identity and complete locator without changing files.
    pub fn prepare(
        &self,
        root: impl AsRef<Path>,
        input: &GovernanceRequestInputV1,
    ) -> Result<PreparedAdmissionGovernanceV1, GovernanceRefusalV1> {
        validate_request(input.clone())?;
        let root = root.as_ref();
        let retained = RetainedRepositoryRootV1::open_exact(root).map_err(fs_refusal)?;
        if retained
            .list_directory_bounded("", 4096)
            .map_err(fs_refusal)?
            .iter()
            .any(|e| {
                matches!(
                    e.name(),
                    ".handbook"
                        | ".handbook-initialization.v1.json"
                        | ".handbook-initialization-receipt.v1.json"
                        | DEFERRAL_NONCOMMIT_PATH_V2
                )
            })
        {
            return Err(refusal(
                "initial_registration_unavailable",
                "An existing or reserved root cannot produce a fresh creation capability",
            ));
        }
        let physical = retained
            .governance_physical_root_fingerprint_v1()
            .map_err(fs_refusal)?;
        let identity = RepositoryInvocationIdentityServiceV1::new()
            .allocate_prepared_repository_identity()
            .map_err(|_| {
                refusal(
                    "identity_entropy_unavailable",
                    "Proposed project identity could not be allocated",
                )
            })?;
        let retained_root = root
            .to_str()
            .ok_or_else(|| refusal("invalid_root", "Root must be exact UTF-8"))?
            .to_owned();
        let registration_identity = governance_fingerprint(
            "handbook.admission-registration@1.0.0",
            &(
                identity.repository_identity_fingerprint(),
                &retained_root,
                &physical,
                input,
            ),
        )?;
        let project = ProjectBindingV1 {
            repository_identity_fingerprint: identity.repository_identity_fingerprint().to_owned(),
            retained_root,
            physical_root_fingerprint: physical,
            registration_identity,
        };
        let locator = create_request_locator(project, input.clone())?;
        Ok(PreparedAdmissionGovernanceV1 { locator, identity })
    }

    /// Consumes the sealed creation capability once. No ordinary retry is exposed.
    pub fn initialize(
        &self,
        root: impl AsRef<Path>,
        prepared: PreparedAdmissionGovernanceV1,
    ) -> GovernanceExecutionV1 {
        let PreparedAdmissionGovernanceV1 { locator, identity } = prepared;
        let mut may_have_reserved = false;
        let attempt = (|| {
            validate_request_locator(&locator)?;
            let _process = PROCESS_LOCK
                .lock()
                .map_err(|_| refusal("lock_unavailable", "Owner process lock is unavailable"))?;
            let retained = exact_root(root.as_ref(), &locator.project)?;
            let guard =
                RepositoryInitializationGuardV1::acquire(root.as_ref()).map_err(fs_refusal)?;
            guard.require_fresh().map_err(|error| {
                if error.kind() == RepositoryDurabilityErrorKindV1::AlreadyExists {
                    refusal(
                        "initial_registration_unavailable",
                        "An existing or reserved root cannot receive new-project registration",
                    )
                    .with_project(locator.project.clone())
                } else {
                    fs_refusal(error)
                }
            })?;
            let claim = initialization_claim_v1(root.as_ref(), &retained, &locator)?;
            // Retain the reservation before any .handbook mutation. Neither a
            // pending nor a noncommitted native reservation is ordinary retry.
            may_have_reserved = true;
            guard.retain_native_claim(&claim).map_err(fs_refusal)?;
            // Once the claim exists, even a root-creation error requires exact
            // resolution; it cannot report this reservation as a clean retry.
            retained
                .create_directory_new_durable(".handbook")
                .map_err(fs_refusal)?;
            self.boundary(GovernanceFaultBoundaryV1::Reservation)?;
            let _native = retained.acquire_exclusive_lock(LOCK).map_err(fs_refusal)?;
            _native.validate_at(&retained, LOCK).map_err(fs_refusal)?;
            retained
                .write_new_durable(
                    GOVERNANCE_ANCHOR_PATH_V1,
                    &governance_canonical_bytes(&locator)?,
                )
                .map_err(fs_refusal)?;
            self.boundary(GovernanceFaultBoundaryV1::Anchor)?;
            let facts = Facts::new(&locator)?;
            retained.create_directories(GOVERNANCE_STORAGE_PATH_V1).map_err(fs_refusal)?;
            retained.write_new_durable(
                ".handbook/admission-governance/v1/initial-noncommit.prepared",
                &facts.not_committed,
            ).map_err(fs_refusal)?;
            RepositoryInvocationIdentityServiceV1::new()
                .persist_prepared_repository_identity_locked(&guard, identity)
                .map_err(fs_refusal)?;
            self.boundary(GovernanceFaultBoundaryV1::Identity)?;
            #[cfg(feature = "test-support")]
            for (index, _) in facts.staged.iter().enumerate() {
                self.boundary(GovernanceFaultBoundaryV1::Stage(index))?;
            }
            retained.write_new_durable(
                ".handbook/admission-governance/v1/initial-terminal.prepared",
                &facts.terminal,
            ).map_err(fs_refusal)?;
            #[cfg(feature = "test-support")]
            let database = if matches!(self.fault, Some(GovernanceFaultBoundaryV1::NativeCreate(_)))
                || matches!(&self.pause, Some((GovernanceFaultBoundaryV1::NativeCreate(_), _, _))) {
                storage::GovernanceDatabase::open_with_creation_boundary(&guard, &_native, &claim,
                    |at| self.boundary(GovernanceFaultBoundaryV1::NativeCreate(at)))?
            } else {
                storage::GovernanceDatabase::open(&guard, &_native, &claim, "create")?
            };
            #[cfg(not(feature = "test-support"))]
            let database = storage::GovernanceDatabase::open(&guard, &_native, &claim, "create")?;
            #[cfg(feature = "test-support")]
            let positive_boundary = |at| self.boundary(GovernanceFaultBoundaryV1::NativePublish(at));
            #[cfg(feature = "test-support")]
            let database = if matches!(self.fault, Some(GovernanceFaultBoundaryV1::NativePublish(_)))
                || matches!(&self.pause, Some((GovernanceFaultBoundaryV1::NativePublish(_), _, _))) {
                database.with_test_boundaries(Some(&positive_boundary), None)
            } else { database };
            let mut source_facts = vec![facts.registration.clone()];
            source_facts.extend(facts.staged.clone());
            source_facts.push(facts.intent.clone());
            source_facts.push(facts.decision.clone());
            let (original, current) = database.records(
                "initial-g", &locator.request.request_key, locator.canonical_json_bytes()?,
                governance_canonical_bytes(&facts.outcome)?, facts.not_committed.clone(),
                facts.terminal.clone(), facts.staged[4].clone(), None, source_facts,
            )?;
            if storage::publish_original(&database, original, current, || {
                retained.rename_noreplace_durable(
                    ".handbook/admission-governance/v1/initial-noncommit.prepared",
                    ".handbook/admission-governance/v1/initial-positive.started",
                ).map_err(fs_refusal)
            })?
                != handbook_engine::retained_database::OriginalDisposition::Selected
            {
                return Err(refusal("original_unselected", "Exact initial governance selection did not commit"));
            }
            #[cfg(feature = "test-support")]
            self.boundary(GovernanceFaultBoundaryV1::Decision)?;
            if self.finish(&guard, &database, &claim, &locator, &facts)?
                != handbook_engine::retained_database::OriginalDisposition::Selected
            {
                return Err(refusal("original_conflict", "Exact initial governance original differs"));
            }
            self.boundary(GovernanceFaultBoundaryV1::Reply)?;
            drop(database);
            _native.release().map_err(fs_refusal)?;
            guard.release().map_err(fs_refusal)?;
            Ok(GovernanceExecutionV1::Committed {
                outcome: facts.outcome,
                delivery: GovernanceDeliveryV1::Original,
            })
        })();
        match attempt {
            Ok(result) => result,
            Err(error) if !may_have_reserved => GovernanceExecutionV1::Refused { refusal: error },
            Err(error) => GovernanceExecutionV1::Uncertain {
                locator,
                condition: format!("{}: {}", error.condition, error.detail),
            },
        }
    }

    /// Historical request resolution may finish an already decided transaction.
    pub fn resolve_request(
        &self,
        root: impl AsRef<Path>,
        locator: &GovernanceRequestLocatorV1,
    ) -> GovernanceExecutionV1 {
        if let Err(refusal) = validate_request_locator(locator) {
            return GovernanceExecutionV1::Refused { refusal };
        }
        let result = (|| {
            let _process = PROCESS_LOCK.lock()
                .map_err(|_| refusal("lock_unavailable", "Owner process lock is unavailable"))?;
            let retained = exact_root(root.as_ref(), &locator.project)?;
            let guard = RepositoryInitializationGuardV1::acquire(root.as_ref()).map_err(fs_refusal)?;
            let claim = initialization_claim_v1(root.as_ref(), &retained, locator)?;
            require_existing_lock(&retained)?;
            let native = retained.acquire_exclusive_lock(LOCK).map_err(fs_refusal)?;
            native.validate_at(&retained, LOCK).map_err(fs_refusal)?;
            let original_bytes = read(&retained, GOVERNANCE_ANCHOR_PATH_V1)?
                .ok_or_else(|| refusal("registration_missing", "Original reservation anchor is unavailable"))?;
            let original: GovernanceRequestLocatorV1 = decode_governance_json(&original_bytes)?;
            validate_request_locator(&original)?;
            if original != *locator {
                return Ok(GovernanceExecutionV1::Refused {
                    refusal: refusal("request_or_configuration_conflict", "Original registration binds different exact request/configuration content")
                        .with_project(original.project),
                });
            }
            if !guard.observe_native_claim(&claim).map_err(fs_refusal)? {
                return Err(refusal("claim_missing", "Original initialization claim is unavailable"));
            }
            let facts = Facts::new(locator)?;
            const PRE: &str = ".handbook/admission-governance/v1/initial-noncommit.prepared";
            const STARTED: &str = ".handbook/admission-governance/v1/initial-positive.started";
            const CLOSED: &str = ".handbook/admission-governance/v1/initial-noncommit.json";
            let pre = retained.read_regular_bounded(PRE, MAX_BYTES).map_err(fs_refusal)?;
            let started = retained.read_regular_bounded(STARTED, MAX_BYTES).map_err(fs_refusal)?;
            let closed = retained.read_regular_bounded(CLOSED, MAX_BYTES).map_err(fs_refusal)?;
            match (pre, started, closed) {
                (RepositoryReadV1::Found(bytes), RepositoryReadV1::Absent, RepositoryReadV1::Absent)
                    if bytes == facts.not_committed => {
                    if retained.read_regular_bounded(handbook_engine::REPOSITORY_INITIALIZATION_RECEIPT_PATH_V1, MAX_BYTES)
                        .map_err(fs_refusal)? != RepositoryReadV1::Absent {
                        return Err(refusal("terminal_state", "Existing claim receipt prevents new noncommit closure"));
                    }
                    retained.rename_noreplace_durable(PRE, CLOSED).map_err(fs_refusal)?;
                    guard.complete_native_claim(&claim, RepositoryInitializationDispositionV1::NotCommitted,
                        CLOSED, &facts.not_committed).map_err(fs_refusal)?;
                    return Ok(GovernanceExecutionV1::NotCommitted { outcome: facts.noncommit, delivery: GovernanceDeliveryV1::Replay });
                }
                (RepositoryReadV1::Absent, RepositoryReadV1::Absent, RepositoryReadV1::Found(bytes))
                    if bytes == facts.not_committed => {
                    guard.complete_native_claim(&claim, RepositoryInitializationDispositionV1::NotCommitted,
                        CLOSED, &facts.not_committed).map_err(fs_refusal)?;
                    return Ok(GovernanceExecutionV1::NotCommitted { outcome: facts.noncommit, delivery: GovernanceDeliveryV1::Replay });
                }
                (RepositoryReadV1::Absent, RepositoryReadV1::Found(bytes), RepositoryReadV1::Absent)
                    if bytes == facts.not_committed => {}
                _ => return Err(refusal("terminal_state", "Exact initial submission fence is unavailable")),
            }
            #[cfg(feature = "test-support")]
            let closure_boundary = |at| self.boundary(GovernanceFaultBoundaryV1::NativeClosure(at));
            let database = storage::GovernanceDatabase::open(&guard, &native, &claim, "recover")?;
            #[cfg(feature = "test-support")]
            let database = if matches!(self.fault, Some(GovernanceFaultBoundaryV1::NativeClosure(_)))
                || matches!(&self.pause, Some((GovernanceFaultBoundaryV1::NativeClosure(_), _, _))) {
                database.with_test_boundaries(None, Some(&closure_boundary))
            } else { database };
            let disposition = self.finish(&guard, &database, &claim, locator, &facts)?;
            drop(database);
            native.release().map_err(fs_refusal)?;
            guard.release().map_err(fs_refusal)?;
            match disposition {
                handbook_engine::retained_database::OriginalDisposition::Selected =>
                    Ok(GovernanceExecutionV1::Committed { outcome: facts.outcome, delivery: GovernanceDeliveryV1::Replay }),
                handbook_engine::retained_database::OriginalDisposition::NotSelected =>
                    Ok(GovernanceExecutionV1::NotCommitted { outcome: facts.noncommit, delivery: GovernanceDeliveryV1::Replay }),
                _ => Err(refusal("request_conflict", "Native original conflicts with its predecessor")),
            }
        })();
        match result {
            Ok(result) => result,
            Err(error) => GovernanceExecutionV1::Uncertain { locator: locator.clone(), condition: error.condition },
        }
    }

    /// Current active resolution is independently checked against retained history.
    pub fn resolve_active(
        &self,
        root: impl AsRef<Path>,
        project: &ProjectBindingV1,
        scope: &str,
        expected: Option<&ExpectedGovernanceBasisV1>,
    ) -> Result<ActiveGovernanceBasisV1, GovernanceRefusalV1> {
        self.resolve_active_with_original(root, project, scope, expected)
            .map(|(basis, _)| basis)
    }

    /// Returns current owner-verified truth and its exact original request.
    /// The historical locator alone never establishes current applicability.
    pub fn resolve_active_with_original(
        &self,
        root: impl AsRef<Path>,
        project: &ProjectBindingV1,
        scope: &str,
        expected: Option<&ExpectedGovernanceBasisV1>,
    ) -> Result<(ActiveGovernanceBasisV1, GovernanceRequestLocatorV1), GovernanceRefusalV1> {
        if scope != GOVERNANCE_SCOPE_V1 {
            return Err(refusal("unsupported_scope", "Only the exact Contracts freshness root scope is supported"));
        }
        let _process = PROCESS_LOCK.lock()
            .map_err(|_| refusal("lock_unavailable", "Owner process lock is unavailable"))?;
        let retained = exact_root(root.as_ref(), project)?;
        let guard = RepositoryInitializationGuardV1::acquire(root.as_ref()).map_err(fs_refusal)?;
        require_existing_lock(&retained)?;
        let native = retained.acquire_exclusive_lock(LOCK).map_err(fs_refusal)?;
        let original_bytes = read(&retained, GOVERNANCE_ANCHOR_PATH_V1)?
            .ok_or_else(|| refusal("registration_missing", "No retained registration anchor"))?;
        let locator: GovernanceRequestLocatorV1 = decode_governance_json(&original_bytes)?;
        validate_request_locator(&locator)?;
        if locator.project != *project {
            return Err(refusal("project_substituted", "Anchor binds a different project"));
        }
        let claim = initialization_claim_v1(root.as_ref(), &retained, &locator)?;
        let facts = Facts::new(&locator)?;
        let database = storage::GovernanceDatabase::open(&guard, &native, &claim, "recover")?;
        let (current, selected) = storage::resolve_current(&database)?;
        if current.operation != "initial-g" || current.relation != facts.staged[4]
            || selected.locator != locator.canonical_json_bytes()?
        {
            return Err(refusal("active_unverifiable", "Exact initial governance is not current"));
        }
        if self.finish(&guard, &database, &claim, &locator, &facts)?
            != handbook_engine::retained_database::OriginalDisposition::Selected
        {
            return Err(refusal("governance_not_committed", "Registration did not activate governance"));
        }
        if expected.is_some_and(|expected| {
            expected.configuration_fingerprint != facts.active.configuration_fingerprint
                || expected.outcome_fingerprint != facts.outcome.outcome_fingerprint
        }) {
            return Err(refusal("basis_conflict", "Expected exact configuration/outcome differs"));
        }
        drop(database);
        native.release().map_err(fs_refusal)?;
        guard.release().map_err(fs_refusal)?;
        Ok((facts.active, locator))
    }

    fn finish(
        &self,
        guard: &RepositoryInitializationGuardV1,
        database: &storage::GovernanceDatabase<'_>,
        claim: &RepositoryInitializationClaimV1,
        locator: &GovernanceRequestLocatorV1,
        facts: &Facts,
    ) -> Result<handbook_engine::retained_database::OriginalDisposition, GovernanceRefusalV1> {
        use handbook_engine::retained_database::OriginalDisposition;
        let mut source_facts = vec![facts.registration.clone()];
        source_facts.extend(facts.staged.clone());
        source_facts.push(facts.intent.clone());
        source_facts.push(facts.decision.clone());
        let (expected, _) = database.records(
            "initial-g", &locator.request.request_key, locator.canonical_json_bytes()?,
            governance_canonical_bytes(&facts.outcome)?, facts.not_committed.clone(),
            facts.terminal.clone(), facts.staged[4].clone(), None, source_facts,
        )?;
        let (disposition, actual) = match storage::resolve_original(database, &expected.id, &expected.locator) {
            Ok(result) => result,
            Err(error) if matches!(error.condition.as_str(), "original_absent" | "noncommit_closure_pending") =>
                (OriginalDisposition::NotSelected, storage::close_noncommit(database, &expected.id, &expected.locator, true)?),
            Err(error) => return Err(error),
        };
        if governance_canonical_bytes(&actual)? != governance_canonical_bytes(&expected)? {
            return Err(refusal("original_conflict", "Exact initial owner facts differ"));
        }
        const PRE: &str = ".handbook/admission-governance/v1/initial-noncommit.prepared";
        const STARTED: &str = ".handbook/admission-governance/v1/initial-positive.started";
        const CLOSED: &str = ".handbook/admission-governance/v1/initial-noncommit.json";
        match disposition {
            OriginalDisposition::Selected => {
                if guard.root().read_regular_bounded(PRE, MAX_BYTES).map_err(fs_refusal)? != RepositoryReadV1::Absent
                    || guard.root().read_regular_bounded(STARTED, MAX_BYTES).map_err(fs_refusal)? != RepositoryReadV1::Found(facts.not_committed.clone())
                    || guard.root().read_regular_bounded(CLOSED, MAX_BYTES).map_err(fs_refusal)? != RepositoryReadV1::Absent
                { return Err(refusal("terminal_state", "Exact consumed initial submission preimage required")); }
            }
            OriginalDisposition::NotSelected => {
                let closed = storage::close_noncommit(database, &expected.id, &expected.locator, true)?;
                if governance_canonical_bytes(&closed)? != governance_canonical_bytes(&expected)? {
                    return Err(refusal("original_conflict", "Exact initial closure differs"));
                }
                match (guard.root().read_regular_bounded(STARTED, MAX_BYTES).map_err(fs_refusal)?,
                    guard.root().read_regular_bounded(CLOSED, MAX_BYTES).map_err(fs_refusal)?) {
                    (RepositoryReadV1::Found(bytes), RepositoryReadV1::Absent) if bytes == facts.not_committed =>
                        guard.root().rename_noreplace_durable(STARTED, CLOSED).map_err(fs_refusal)?,
                    (RepositoryReadV1::Absent, RepositoryReadV1::Found(bytes)) if bytes == facts.not_committed => {}
                    _ => return Err(refusal("terminal_state", "Exact noncommit preimage or witness required")),
                }
                guard.complete_native_claim(claim, RepositoryInitializationDispositionV1::NotCommitted,
                    CLOSED, &facts.not_committed).map_err(fs_refusal)?;
                storage::recover_original(database, &expected, disposition.clone(),
                    Some((CLOSED, &facts.not_committed, RepositoryInitializationDispositionV1::NotCommitted)))?;
                return Ok(disposition);
            }
            OriginalDisposition::Conflict =>
                return Err(refusal("request_conflict", "Native original has a predecessor conflict")),
        }
        const WITNESS: &str = ".handbook/admission-governance/v1/initial-terminal.json";
        const PREPARED: &str = ".handbook/admission-governance/v1/initial-terminal.prepared";
        let source = guard.root().read_regular_bounded(PREPARED, MAX_BYTES).map_err(fs_refusal)?;
        let witness = guard.root().read_regular_bounded(WITNESS, MAX_BYTES).map_err(fs_refusal)?;
        match (source, witness) {
            (RepositoryReadV1::Found(bytes), RepositoryReadV1::Absent) if bytes == facts.terminal => {
                guard.validate().map_err(fs_refusal)?;
                guard.root().rename_noreplace_durable(PREPARED, WITNESS).map_err(fs_refusal)?;
            }
            (RepositoryReadV1::Absent, RepositoryReadV1::Found(bytes)) if bytes == facts.terminal => {}
            _ => return Err(refusal("terminal_state", "Exact prepared or consumed initialization witness required")),
        }
        self.boundary(GovernanceFaultBoundaryV1::Terminal)?;
        guard.complete_native_claim(
            claim, RepositoryInitializationDispositionV1::Committed, WITNESS, &facts.terminal,
        ).map_err(fs_refusal)?;
        storage::recover_original(
            database, &expected, disposition.clone(),
            Some((WITNESS, &facts.terminal, RepositoryInitializationDispositionV1::Committed)),
        )?;
        Ok(disposition)
    }

}

fn initialization_claim_v1(
    root: &Path,
    retained: &RetainedRepositoryRootV1,
    locator: &GovernanceRequestLocatorV1,
) -> Result<RepositoryInitializationClaimV1, GovernanceRefusalV1> {
    RepositoryInitializationClaimV1::prepare(
        root,
        retained,
        "handbook-contracts.admission-establishment.v1",
        locator.request_fingerprint.as_str(),
        &locator.project.repository_identity_fingerprint,
    )
    .map_err(fs_refusal)
}

struct Facts {
    registration: Vec<u8>,
    staged: Vec<Vec<u8>>,
    intent: Vec<u8>,
    decision: Vec<u8>,
    terminal: Vec<u8>,
    not_committed: Vec<u8>,
    outcome: GovernanceCommittedOutcomeV1,
    noncommit: GovernanceNotCommittedOutcomeV1,
    active: ActiveGovernanceBasisV1,
}
impl Facts {
    fn new(locator: &GovernanceRequestLocatorV1) -> Result<Self, GovernanceRefusalV1> {
        let validated = validate_request_locator(locator)?;
        let outcome_fingerprint = governance_fingerprint(
            "handbook.admission-governance-outcome@1.0.0",
            &(
                locator,
                &validated,
                "validated",
                "advisory_permission_accepted",
                "activation_committed",
            ),
        )?;
        let outcome = GovernanceCommittedOutcomeV1 {
            locator: locator.clone(),
            replacement_basis: locator.request.replacement_basis.clone(),
            validated: validated.clone(),
            validation_confirmed: true,
            advisory_permission_accepted: true,
            activation_committed: true,
            outcome_fingerprint: outcome_fingerprint.clone(),
        };
        let active_fingerprint = governance_fingerprint(
            "handbook.admission-governance-active@1.0.0",
            &(
                &locator.project,
                &locator.request.scope,
                &validated.configuration_fingerprint,
                &outcome_fingerprint,
            ),
        )?;
        let active = ActiveGovernanceBasisV1 {
            project: locator.project.clone(),
            scope: locator.request.scope.clone(),
            configuration: locator.request.configuration.clone(),
            configuration_fingerprint: validated.configuration_fingerprint.clone(),
            establishment_rule: validated.establishment_rule.clone(),
            owner_rule: validated.owner_rule.clone(),
            outcome_fingerprint: outcome_fingerprint.clone(),
            active_fingerprint,
        };
        let registration = governance_canonical_bytes(&(
            "handbook.admission-registration.v1",
            locator,
            "single_unused_initial_establishment",
        ))?;
        let staged = vec![
            governance_canonical_bytes(&(&validated.establishment_rule, &validated.owner_rule))?,
            governance_canonical_bytes(&(
                &locator.request.configuration,
                &validated.configuration_fingerprint,
            ))?,
            governance_canonical_bytes(&outcome)?,
            governance_canonical_bytes(&(
                "consumed_initial_registration",
                &locator.project.registration_identity,
                &locator.request_fingerprint,
                &outcome_fingerprint,
            ))?,
            governance_canonical_bytes(&active)?,
        ];
        let intent = governance_canonical_bytes(&(
            "handbook.admission-intent.v1",
            &locator.request_fingerprint,
            governance_fingerprint("handbook.admission-registration-bytes@1.0.0", &registration)?,
            staged
                .iter()
                .map(|bytes| governance_fingerprint("handbook.admission-stage-bytes@1.0.0", bytes))
                .collect::<Result<Vec<_>, _>>()?,
        ))?;
        let decision = governance_canonical_bytes(&(
            "handbook.admission-commit-decision.v1",
            governance_fingerprint("handbook.admission-intent-bytes@1.0.0", &intent)?,
        ))?;
        let terminal = governance_canonical_bytes(&(
            "handbook.admission-terminal.v1",
            &outcome_fingerprint,
            governance_fingerprint("handbook.admission-decision-bytes@1.0.0", &decision)?,
        ))?;
        let condition = "verified_predecision_noncommit".to_owned();
        let noncommit = GovernanceNotCommittedOutcomeV1 {
            locator: locator.clone(),
            outcome_fingerprint: governance_fingerprint(
                "handbook.admission-noncommit@1.0.0",
                &(locator, &condition),
            )?,
            condition,
            activation_committed: false,
        };
        let not_committed = governance_canonical_bytes(&noncommit)?;
        Ok(Self {
            registration,
            staged,
            intent,
            decision,
            terminal,
            not_committed,
            outcome,
            noncommit,
            active,
        })
    }
}
struct State {
    intent: bool,
    decision: bool,
    installed: usize,
    terminal: bool,
    not_committed: bool,
}

/// Checks the entire owned inventory and causal ordering, including absent facts.
fn inspect(
    root: &RetainedRepositoryRootV1,
    locator: &GovernanceRequestLocatorV1,
    facts: &Facts,
) -> Result<State, GovernanceRefusalV1> {
    require(
        root,
        GOVERNANCE_ANCHOR_PATH_V1,
        &governance_canonical_bytes(locator)?,
    )?;
    let identity = read(root, REPOSITORY_IDENTITY_REPO_PATH)?;
    let temporary = read(root, IDENTITY_TEMP)?;
    for bytes in [identity.as_ref(), temporary.as_ref()]
        .into_iter()
        .flatten()
    {
        if bytes != locator.project.repository_identity_fingerprint.as_bytes() {
            return Err(refusal(
                "identity_substituted",
                "Retained identity differs from the original request",
            ));
        }
    }
    if identity.is_some() && temporary.is_some() {
        return Err(refusal(
            "identity_nonprefix",
            "Identity publication has conflicting evidence",
        ));
    }
    let owner_entries = root
        .list_directory_bounded(".handbook", 4096)
        .map_err(fs_refusal)?;
    if !owner_entries.iter().any(|e| {
        e.name() == "admission-governance.lock"
            && e.kind() == RepositoryDirectoryEntryKindV1::RegularFile
    }) {
        return Err(refusal(
            "lock_missing",
            "Retained project governance lock is missing",
        ));
    }
    let owner_dir = owner_entries
        .iter()
        .find(|e| e.name() == "admission-governance");
    let names = if let Some(entry) = owner_dir {
        if entry.kind() != RepositoryDirectoryEntryKindV1::Directory {
            return Err(refusal(
                "unsafe_inventory",
                "Governance storage is not a directory",
            ));
        }
        let versions = root
            .list_directory_bounded(".handbook/admission-governance", 1)
            .map_err(fs_refusal)?;
        if versions
            .iter()
            .any(|e| e.name() != "v1" || e.kind() != RepositoryDirectoryEntryKindV1::Directory)
        {
            return Err(refusal(
                "unknown_inventory",
                "Unknown governance storage version",
            ));
        }
        if versions.is_empty() {
            Vec::new()
        } else {
            root.list_directory_bounded(GOVERNANCE_STORAGE_PATH_V1, 32)
                .map_err(fs_refusal)?
        }
    } else {
        Vec::new()
    };
    let allowed: Vec<String> = vec![
        "registration.json",
        "intent.json",
        "decision.json",
        "terminal.json",
        "not-committed.json",
    ]
    .into_iter()
    .chain(TARGETS)
    .map(str::to_owned)
    .chain((0..5).map(|i| format!("stage-{i}.json")))
    .collect();
    for entry in &names {
        if entry.kind() != RepositoryDirectoryEntryKindV1::RegularFile
            || !allowed.iter().any(|n| n == entry.name())
        {
            return Err(refusal(
                "unknown_inventory",
                "Unknown entry in retained governance inventory",
            ));
        }
    }
    // Safe absent reads require storage parent existence; use enumerated membership.
    let has = |name: &str| names.iter().any(|e| e.name() == name);
    let check = |name: &str, bytes: &[u8]| -> Result<bool, GovernanceRefusalV1> {
        if has(name) {
            require(root, &path(name), bytes)?;
            Ok(true)
        } else {
            Ok(false)
        }
    };
    let registration = check("registration.json", &facts.registration)?;
    if registration && identity.is_none() {
        return Err(refusal(
            "registration_without_identity",
            "Positive registration requires the exact retained identity",
        ));
    }
    let mut stage_count = 0;
    for (i, bytes) in facts.staged.iter().enumerate() {
        if check(&format!("stage-{i}.json"), bytes)? {
            if !registration || i != stage_count {
                return Err(refusal(
                    "stage_nonprefix",
                    "Staging is not an exact causal prefix",
                ));
            }
            stage_count += 1;
        }
    }
    let intent = check("intent.json", &facts.intent)?;
    if intent && stage_count != 5 {
        return Err(refusal(
            "intent_without_staging",
            "Complete exact staged facts are required",
        ));
    }
    let decision = check("decision.json", &facts.decision)?;
    if decision && !intent {
        return Err(refusal(
            "decision_without_intent",
            "Exact transaction intent is missing",
        ));
    }
    let mut installed = 0;
    for (i, target) in TARGETS.iter().enumerate() {
        if check(target, &facts.staged[i])? {
            if !decision || i != installed {
                return Err(refusal(
                    "installation_nonprefix",
                    "Installed effects lack an exact decision/prefix",
                ));
            }
            installed += 1;
        }
    }
    let terminal = check("terminal.json", &facts.terminal)?;
    if terminal && installed != 5 {
        return Err(refusal(
            "terminal_incomplete",
            "Terminal outcome does not close all installed facts",
        ));
    }
    let not_committed = check("not-committed.json", &facts.not_committed)?;
    if not_committed && (decision || installed != 0 || terminal) {
        return Err(refusal(
            "terminal_conflict",
            "Noncommit conflicts with committed effects",
        ));
    }
    Ok(State {
        intent,
        decision,
        installed,
        terminal,
        not_committed,
    })
}
fn exact_root(
    root: &Path,
    project: &ProjectBindingV1,
) -> Result<RetainedRepositoryRootV1, GovernanceRefusalV1> {
    if root.to_str() != Some(project.retained_root.as_str()) {
        return Err(refusal(
            "root_substituted",
            "Locator names a different retained root",
        ));
    }
    let retained = RetainedRepositoryRootV1::open_exact(root).map_err(fs_refusal)?;
    if retained
        .governance_physical_root_fingerprint_v1()
        .map_err(fs_refusal)?
        != project.physical_root_fingerprint
    {
        return Err(refusal(
            "physical_root_substituted",
            "Physical root identity differs",
        ));
    }
    Ok(retained)
}
fn require_existing_lock(root: &RetainedRepositoryRootV1) -> Result<(), GovernanceRefusalV1> {
    if root
        .list_directory_bounded(".handbook", 4096)
        .map_err(fs_refusal)?
        .iter()
        .any(|e| {
            e.name() == "admission-governance.lock"
                && e.kind() == RepositoryDirectoryEntryKindV1::RegularFile
        })
    {
        Ok(())
    } else {
        Err(refusal(
            "lock_missing",
            "Exact request has no retained governance lock",
        ))
    }
}
fn path(name: &str) -> String {
    format!("{GOVERNANCE_STORAGE_PATH_V1}/{name}")
}
fn read(
    root: &RetainedRepositoryRootV1,
    path: &str,
) -> Result<Option<Vec<u8>>, GovernanceRefusalV1> {
    match root
        .read_regular_bounded(path, MAX_BYTES)
        .map_err(fs_refusal)?
    {
        RepositoryReadV1::Absent => Ok(None),
        RepositoryReadV1::Found(bytes) => Ok(Some(bytes)),
    }
}
fn require(
    root: &RetainedRepositoryRootV1,
    path: &str,
    expected: &[u8],
) -> Result<(), GovernanceRefusalV1> {
    if read(root, path)?.as_deref() != Some(expected) {
        Err(refusal(
            "record_missing_or_substituted",
            "Required exact retained record differs",
        ))
    } else {
        Ok(())
    }
}
fn write(
    root: &RetainedRepositoryRootV1,
    name: &str,
    bytes: &[u8],
) -> Result<(), GovernanceRefusalV1> {
    if bytes.len() > MAX_BYTES {
        return Err(refusal("bound_exceeded", "Record exceeds 1 MiB"));
    }
    root.write_new_durable(&path(name), bytes)
        .map_err(fs_refusal)
}
fn refusal(condition: impl Into<String>, detail: impl Into<String>) -> GovernanceRefusalV1 {
    GovernanceRefusalV1::new(condition, detail)
}
fn fs_refusal(error: handbook_engine::RepositoryDurabilityErrorV1) -> GovernanceRefusalV1 {
    refusal(format!("filesystem_{:?}", error.kind()), error.detail())
}
