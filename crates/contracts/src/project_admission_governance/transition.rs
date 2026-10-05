//! Same-project first establishment. Retained selection and request history are
//! outside runtime state; current truth and historical delivery are independent.
use super::*;
use handbook_engine::{
    RepositoryDirectoryEntryKindV1 as Kind, RepositoryExclusiveLockV1,
    RepositoryInitializationDispositionV1, RepositoryInitializationGuardV1,
    RetainedRepositoryRootV1,
};
use serde::{Deserialize, Serialize};
use std::path::Path;
#[cfg(feature = "test-support")]
use std::path::PathBuf;

pub const ESTABLISHMENT_SELECTION_PATH_V3: &str =
    ".handbook/admission-governance/v2/establishment-v3";
pub const MAX_ADMISSION_REQUESTS_V3: usize = 4096;
const TARGETS_V3: [&str; 7] = [
    "rules.json",
    "configuration.json",
    "instruction.json",
    "terms.json",
    "outcome.json",
    "predecessor.json",
    "relation.json",
];

#[derive(Debug)]
pub struct PreparedDeferredGovernanceV3 {
    locator: DeferredEstablishmentLocatorV3,
}
impl PreparedDeferredGovernanceV3 {
    pub fn locator(&self) -> &DeferredEstablishmentLocatorV3 {
        &self.locator
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeferredEstablishmentCommittedOutcomeV3 {
    pub locator: DeferredEstablishmentLocatorV3,
    pub validated: ValidatedDeferredEstablishmentV3,
    pub validation_confirmed: bool,
    pub advisory_permission_accepted: bool,
    pub activation_committed: bool,
    pub validation_fingerprint: FingerprintV1,
    pub advisory_permission_fingerprint: FingerprintV1,
    pub outcome_fingerprint: FingerprintV1,
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeferredEstablishmentNotCommittedOutcomeV3 {
    pub locator: DeferredEstablishmentLocatorV3,
    pub condition: String,
    pub activation_committed: bool,
    pub outcome_fingerprint: FingerprintV1,
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case", deny_unknown_fields)]
pub enum DeferredEstablishmentExecutionV3 {
    Committed {
        outcome: DeferredEstablishmentCommittedOutcomeV3,
        delivery: GovernanceDeliveryV1,
    },
    NotCommitted {
        outcome: DeferredEstablishmentNotCommittedOutcomeV3,
        delivery: GovernanceDeliveryV1,
    },
    Refused {
        refusal: GovernanceRefusalV1,
    },
    Uncertain {
        locator: DeferredEstablishmentLocatorV3,
        condition: String,
    },
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EstablishedFromDeferredBasisV3 {
    pub locator: DeferredEstablishmentLocatorV3,
    pub project: ProjectBindingV2,
    pub scope: String,
    pub predecessor: ExpectedDeferredBasisV2,
    pub configuration: AdmissionConfigurationInputV1,
    pub configuration_fingerprint: FingerprintV1,
    pub replacement_basis: ReplacementAuthorityBasisV1,
    pub establishment_rule: GovernanceRuleDescriptorV2,
    pub owner_rule: GovernanceRuleDescriptorV1,
    pub request_fingerprint: FingerprintV1,
    pub outcome_fingerprint: FingerprintV1,
    pub relation_fingerprint: FingerprintV1,
}
impl EstablishedFromDeferredBasisV3 {
    /// Wire validation grants no owner delivery or current authority.
    pub fn from_json_bytes(bytes: &[u8]) -> Result<Self, GovernanceRefusalV1> {
        let value: Self = decode_complete_v3(bytes)?;
        value.canonical_json_bytes()?;
        Ok(value)
    }
    pub fn canonical_json_bytes(&self) -> Result<Vec<u8>, GovernanceRefusalV1> {
        if *self != TransitionFacts::new(&self.locator)?.basis {
            return Err(refusal(
                "basis_identity",
                "Exact established-from-deferred basis is inconsistent",
            ));
        }
        governance_canonical_bytes(self)
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExpectedEstablishedFromDeferredBasisV3 {
    pub configuration_fingerprint: FingerprintV1,
    pub outcome_fingerprint: FingerprintV1,
    pub relation_fingerprint: FingerprintV1,
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(
    tag = "state",
    content = "basis",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum DeferredScopeBasisV3 {
    DeferredV2(DeferredGovernanceBasisV2),
    EstablishedFromDeferredV3(EstablishedFromDeferredBasisV3),
}

impl DeferredEstablishmentExecutionV3 {
    pub fn from_json_bytes(bytes: &[u8]) -> Result<Self, GovernanceRefusalV1> {
        let value: Self = decode_complete_v3(bytes)?;
        value.canonical_json_bytes()?;
        Ok(value)
    }
    pub fn canonical_json_bytes(&self) -> Result<Vec<u8>, GovernanceRefusalV1> {
        let valid = match self {
            Self::Committed { outcome, .. } => {
                *outcome == TransitionFacts::new(&outcome.locator)?.outcome
            }
            Self::NotCommitted { outcome, .. } => {
                *outcome == TransitionFacts::new(&outcome.locator)?.noncommit
            }
            Self::Refused { refusal } => {
                !refusal.condition.is_empty() && !refusal.detail.is_empty()
            }
            Self::Uncertain { locator, condition } => {
                validate_deferred_establishment_locator(locator)?;
                !condition.is_empty() && condition.len() <= 4096
            }
        };
        if !valid {
            return Err(refusal(
                "outcome_identity",
                "Exact v3 wire outcome is inconsistent",
            ));
        }
        governance_canonical_bytes(self)
    }
}
// New records require every serialized key, including nullable fields inside
// retained legacy descriptor types. This changes no v1/v2 decoder or fixture.
fn decode_complete_v3<T: serde::de::DeserializeOwned + Serialize>(
    bytes: &[u8],
) -> Result<T, GovernanceRefusalV1> {
    let value: serde_json::Value = decode_governance_json(bytes)?;
    let result: T = serde_json::from_value(value.clone())
        .map_err(|_| refusal("wire_shape", "Closed complete v3 record required"))?;
    if serde_json::to_value(&result)
        .map_err(|_| refusal("wire_shape", "V3 record cannot be represented"))?
        != value
    {
        return Err(refusal(
            "wire_shape",
            "Every complete v3 field, including nullable fields, is required",
        ));
    }
    Ok(result)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[doc(hidden)]
pub enum EstablishmentFaultBoundaryV3 {
    SelectionDirectory,
    RequestDirectory,
    Request,
    Selection,
    Stage(usize),
    Intent,
    Decision,
    Installation(usize),
    CurrentPrepared,
    CurrentDisplaced,
    CurrentPublished,
    Terminal,
    Closure,
    NonCommitTerminal,
    NonCommitClosure,
    Reply,
}
#[derive(Default)]
pub struct ProjectAdmissionEstablishmentV3 {
    #[cfg(feature = "test-support")]
    fault: Option<EstablishmentFaultBoundaryV3>,
    #[cfg(feature = "test-support")]
    crash: bool,
    #[cfg(feature = "test-support")]
    pause: Option<(EstablishmentFaultBoundaryV3, PathBuf, PathBuf)>,
}
impl ProjectAdmissionEstablishmentV3 {
    pub fn new() -> Self {
        Self::default()
    }
    #[cfg(feature = "test-support")]
    #[doc(hidden)]
    pub fn with_fault_for_testing(mut self, fault: EstablishmentFaultBoundaryV3) -> Self {
        self.fault = Some(fault);
        self
    }
    #[cfg(feature = "test-support")]
    #[doc(hidden)]
    pub fn with_crash_for_testing(mut self, fault: EstablishmentFaultBoundaryV3) -> Self {
        self.fault = Some(fault);
        self.crash = true;
        self
    }
    #[cfg(feature = "test-support")]
    #[doc(hidden)]
    pub fn with_pause_for_testing(
        mut self,
        boundary: EstablishmentFaultBoundaryV3,
        ready: PathBuf,
        release: PathBuf,
    ) -> Self {
        self.pause = Some((boundary, ready, release));
        self
    }
    fn boundary(&self, boundary: EstablishmentFaultBoundaryV3) -> Result<(), GovernanceRefusalV1> {
        #[cfg(feature = "test-support")]
        {
            if let Some((at, ready, release)) = &self.pause {
                if *at == boundary {
                    std::fs::write(ready, b"ready")
                        .map_err(|_| refusal("pause_unavailable", "Rendezvous failed"))?;
                    let start = std::time::Instant::now();
                    while !release.exists() {
                        if start.elapsed() > std::time::Duration::from_secs(30) {
                            return Err(refusal("pause_timeout", "Rendezvous release unavailable"));
                        }
                        std::thread::sleep(std::time::Duration::from_millis(10));
                    }
                }
            }
            if self.fault == Some(boundary) {
                if self.crash {
                    std::process::exit(73);
                }
                return Err(refusal(
                    "injected_interruption",
                    "Resolve the complete original locator",
                ));
            }
        }
        let _ = boundary;
        Ok(())
    }
    /// Pure snapshot preparation. No identity, claim, lock or directory is created.
    pub fn prepare(
        &self,
        root: impl AsRef<Path>,
        request: &DeferredEstablishmentRequestV3,
    ) -> Result<PreparedDeferredGovernanceV3, GovernanceRefusalV1> {
        let locator = create_deferred_establishment_locator(request.clone())?;
        let retained = exact_read_root(root.as_ref(), &request.project)?;
        let original = deferral::original_deferred_snapshot(&retained, &request.project, None)?;
        let history = inspect_history(&retained, &original.basis, true)?;
        require_eligible(&retained, &original, &history, &locator)?;
        Ok(PreparedDeferredGovernanceV3 { locator })
    }
    pub fn establish(
        &self,
        root: impl AsRef<Path>,
        prepared: PreparedDeferredGovernanceV3,
    ) -> DeferredEstablishmentExecutionV3 {
        let locator = prepared.locator;
        let mut effects = false;
        let attempt = (|| {
            validate_deferred_establishment_locator(&locator)?;
            let _process = PROCESS_LOCK
                .lock()
                .map_err(|_| refusal("lock_unavailable", "Owner mutex unavailable"))?;
            let guard = transition_guard(root.as_ref(), &locator.request.project)?;
            let native = owner_lock(guard.root())?;
            let original = require_original(&guard, &native, &locator.request.project)?;
            let history = inspect_history(guard.root(), &original.basis, true)?;
            if let Some(entry) = find_request(&history, &locator)? {
                effects = true;
                return self.resolve_locked(&guard, &native, &original, entry, &locator);
            }
            require_eligible(guard.root(), &original, &history, &locator)?;
            validate_locks(&guard, &native)?;
            effects = true;
            let retained = guard.root();
            retained
                .create_directories(ESTABLISHMENT_SELECTION_PATH_V3)
                .map_err(fs_refusal)?;
            self.boundary(EstablishmentFaultBoundaryV3::SelectionDirectory)?;
            retained
                .create_directory_new_durable(&transition_request_storage_path_v3(&locator))
                .map_err(fs_refusal)?;
            self.boundary(EstablishmentFaultBoundaryV3::RequestDirectory)?;
            t_write(
                retained,
                &locator,
                "request.json",
                &locator.canonical_json_bytes()?,
            )?;
            self.boundary(EstablishmentFaultBoundaryV3::Request)?;
            let (previous_selection, previous_terminal) = history
                .last()
                .map(|entry| {
                    (
                        entry.selection.fingerprint().unwrap(),
                        entry.closure.clone().unwrap(),
                    )
                })
                .unwrap_or((
                    original.basis.relation_fingerprint.clone(),
                    original.basis.outcome_fingerprint.clone(),
                ));
            let selection = SelectionV3 {
                sequence: history.len() + 1,
                locator: locator.clone(),
                previous_selection,
                previous_terminal,
            };
            retained
                .create_directory_new_durable(&selection_dir(selection.sequence))
                .map_err(fs_refusal)?;
            retained
                .write_new_durable(
                    &selection_path(selection.sequence, "selection.json"),
                    &governance_canonical_bytes(&selection)?,
                )
                .map_err(fs_refusal)?;
            self.boundary(EstablishmentFaultBoundaryV3::Selection)?;
            let facts = TransitionFacts::new(&locator)?;
            for (i, bytes) in facts.staged.iter().enumerate() {
                validate_locks(&guard, &native)?;
                t_write(retained, &locator, &format!("stage-{i}.json"), bytes)?;
                self.boundary(EstablishmentFaultBoundaryV3::Stage(i))?;
            }
            validate_locks(&guard, &native)?;
            t_write(retained, &locator, "intent.json", &facts.intent)?;
            self.boundary(EstablishmentFaultBoundaryV3::Intent)?;
            let history = inspect_history(retained, &original.basis, true)?;
            let entry = history
                .last()
                .ok_or_else(|| refusal("selection_missing", "Selected request disappeared"))?;
            if entry.selection != selection
                || entry.state.decision
                || entry.state.noncommit
                || entry.state.terminal
            {
                return Err(refusal(
                    "decision_conflict",
                    "Exact selected predecision prefix differs",
                ));
            }
            require_exact_predecessor(retained, &original, &locator)?;
            require_original(&guard, &native, &locator.request.project)?;
            validate_locks(&guard, &native)?;
            t_write(retained, &locator, "decision.json", &facts.decision)?;
            self.boundary(EstablishmentFaultBoundaryV3::Decision)?;
            self.finish(&guard, &native, &original, &selection, &facts, &entry.state)?;
            inspect_history(retained, &original.basis, true)?;
            self.boundary(EstablishmentFaultBoundaryV3::Reply)?;
            native.release().map_err(fs_refusal)?;
            guard.release().map_err(fs_refusal)?;
            Ok(DeferredEstablishmentExecutionV3::Committed {
                outcome: facts.outcome,
                delivery: GovernanceDeliveryV1::Original,
            })
        })();
        match attempt {
            Ok(result) => result,
            Err(error) if !effects => DeferredEstablishmentExecutionV3::Refused { refusal: error },
            Err(error) => uncertain(&locator, error),
        }
    }
    pub fn resolve_request(
        &self,
        root: impl AsRef<Path>,
        locator: &DeferredEstablishmentLocatorV3,
    ) -> DeferredEstablishmentExecutionV3 {
        if let Err(refusal) = validate_deferred_establishment_locator(locator) {
            return DeferredEstablishmentExecutionV3::Refused { refusal };
        }
        let attempt = (|| {
            let _process = PROCESS_LOCK
                .lock()
                .map_err(|_| refusal("lock_unavailable", "Owner mutex unavailable"))?;
            let guard = transition_guard(root.as_ref(), &locator.request.project)?;
            let native = owner_lock(guard.root())?;
            let original = require_original(&guard, &native, &locator.request.project)?;
            // Historical resolution deliberately does not inspect mutable current.
            let history = inspect_history(guard.root(), &original.basis, false)?;
            let Some(entry) = find_request(&history, locator)? else {
                return Err(refusal(
                    "request_not_selected",
                    "No complete retained selection for this exact request",
                ));
            };
            let result = self.resolve_locked(&guard, &native, &original, entry, locator)?;
            native.release().map_err(fs_refusal)?;
            guard.release().map_err(fs_refusal)?;
            Ok(result)
        })();
        match attempt {
            Ok(result) => result,
            Err(error) if error.condition == "request_conflict" => {
                DeferredEstablishmentExecutionV3::Refused { refusal: error }
            }
            Err(error) => uncertain(locator, error),
        }
    }
    fn resolve_locked(
        &self,
        guard: &RepositoryInitializationGuardV1,
        native: &RepositoryExclusiveLockV1,
        original: &deferral::OriginalDeferredSnapshot,
        entry: &HistoryEntry,
        locator: &DeferredEstablishmentLocatorV3,
    ) -> Result<DeferredEstablishmentExecutionV3, GovernanceRefusalV1> {
        let facts = TransitionFacts::new(locator)?;
        if entry.state.terminal || entry.state.noncommit {
            // A terminal is immutable history. Closure may be completed from that
            // exact history, but no current relation is touched during replay.
            self.close_selection(
                guard,
                native,
                &entry.selection,
                &facts,
                entry.state.noncommit,
            )?;
        } else if entry.state.decision {
            inspect_history(guard.root(), &original.basis, true)?;
            self.finish(
                guard,
                native,
                original,
                &entry.selection,
                &facts,
                &entry.state,
            )?;
        } else {
            require_exact_predecessor(guard.root(), original, locator)?;
            validate_locks(guard, native)?;
            t_write(
                guard.root(),
                locator,
                "noncommit.json",
                &facts.noncommit_bytes,
            )?;
            self.boundary(EstablishmentFaultBoundaryV3::NonCommitTerminal)?;
            self.close_selection(guard, native, &entry.selection, &facts, true)?;
        }
        inspect_history(guard.root(), &original.basis, false)?;
        if entry.state.noncommit || !entry.state.decision {
            Ok(DeferredEstablishmentExecutionV3::NotCommitted {
                outcome: facts.noncommit,
                delivery: GovernanceDeliveryV1::Replay,
            })
        } else {
            Ok(DeferredEstablishmentExecutionV3::Committed {
                outcome: facts.outcome,
                delivery: GovernanceDeliveryV1::Replay,
            })
        }
    }
    fn close_selection(
        &self,
        guard: &RepositoryInitializationGuardV1,
        native: &RepositoryExclusiveLockV1,
        selection: &SelectionV3,
        facts: &TransitionFacts,
        noncommit: bool,
    ) -> Result<(), GovernanceRefusalV1> {
        validate_locks(guard, native)?;
        let bytes = closure_bytes(selection, facts, noncommit)?;
        let path = selection_path(selection.sequence, "closure.json");
        match read(guard.root(), &path)? {
            Some(existing) if existing == bytes => {}
            Some(_) => {
                return Err(refusal(
                    "closure_conflict",
                    "Retained selected terminal differs",
                ))
            }
            None => guard
                .root()
                .write_new_durable(&path, &bytes)
                .map_err(fs_refusal)?,
        }
        self.boundary(if noncommit {
            EstablishmentFaultBoundaryV3::NonCommitClosure
        } else {
            EstablishmentFaultBoundaryV3::Closure
        })
    }
    fn finish(
        &self,
        guard: &RepositoryInitializationGuardV1,
        native: &RepositoryExclusiveLockV1,
        original: &deferral::OriginalDeferredSnapshot,
        selection: &SelectionV3,
        facts: &TransitionFacts,
        state: &TransitionState,
    ) -> Result<(), GovernanceRefusalV1> {
        let root = guard.root();
        let locator = &selection.locator;
        for (i, target) in TARGETS_V3.iter().enumerate().skip(state.installed) {
            validate_locks(guard, native)?;
            t_write(root, locator, target, &facts.staged[i])?;
            self.boundary(EstablishmentFaultBoundaryV3::Installation(i))?;
        }
        let prepared = t_path(locator, "current-prepared.json");
        let displaced = t_path(locator, "current-displaced.json");
        validate_locks(guard, native)?;
        let current = read(root, GOVERNANCE_CURRENT_PATH_V2)?;
        let staged = read(root, &prepared)?;
        let old = read(root, &displaced)?;
        match (current.as_deref(), staged.as_deref(), old.as_deref()) {
            (Some(d), None, None) if d == original.current => {
                root.write_new_durable(&prepared, &facts.staged[6])
                    .map_err(fs_refusal)?;
                self.boundary(EstablishmentFaultBoundaryV3::CurrentPrepared)?;
            }
            (Some(d), Some(g), None) if d == original.current && g == facts.staged[6] => {}
            (None, Some(g), Some(d)) if d == original.current && g == facts.staged[6] => {}
            (Some(g), None, Some(d)) if d == original.current && g == facts.staged[6] => {}
            _ => {
                return Err(refusal(
                    "current_replacement_nonprefix",
                    "Current replacement lacks exact decision-selected bytes",
                ))
            }
        }
        if read(root, &displaced)?.is_none() {
            validate_locks(guard, native)?;
            root.displace_exact_durable(GOVERNANCE_CURRENT_PATH_V2, &original.current, &displaced)
                .map_err(fs_refusal)?;
            self.boundary(EstablishmentFaultBoundaryV3::CurrentDisplaced)?;
        }
        if read(root, GOVERNANCE_CURRENT_PATH_V2)?.is_none() {
            validate_locks(guard, native)?;
            require(root, &prepared, &facts.staged[6])?;
            require(root, &displaced, &original.current)?;
            root.rename_noreplace_durable(&prepared, GOVERNANCE_CURRENT_PATH_V2)
                .map_err(fs_refusal)?;
            self.boundary(EstablishmentFaultBoundaryV3::CurrentPublished)?;
        }
        validate_locks(guard, native)?;
        require(root, GOVERNANCE_CURRENT_PATH_V2, &facts.staged[6])?;
        if !state.terminal {
            t_write(root, locator, "terminal.json", &facts.terminal)?;
            self.boundary(EstablishmentFaultBoundaryV3::Terminal)?;
        }
        self.close_selection(guard, native, selection, facts, false)
    }
    pub fn resolve_current(
        &self,
        root: impl AsRef<Path>,
        project: &ProjectBindingV2,
        scope: &str,
        expected: Option<&ExpectedProjectAdmissionBasisV3>,
    ) -> Result<DeferredScopeBasisV3, GovernanceRefusalV1> {
        if scope != ADMISSION_GOVERNANCE_SCOPE_V1 {
            return Err(refusal(
                "unsupported_scope",
                "Only exact root freshness scope resolves",
            ));
        }
        let _process = PROCESS_LOCK
            .lock()
            .map_err(|_| refusal("lock_unavailable", "Owner mutex unavailable"))?;
        let guard = transition_guard(root.as_ref(), project)?;
        let native = owner_lock(guard.root())?;
        let original = require_original(&guard, &native, project)?;
        let history = inspect_history(guard.root(), &original.basis, true)?;
        if let Some(last) = history.last() {
            if last.closure.is_none() {
                self.resolve_locked(&guard, &native, &original, last, &last.selection.locator)?;
            }
        }
        let history = inspect_history(guard.root(), &original.basis, true)?;
        let basis = if let Some(last) = history.last().filter(|e| e.state.terminal) {
            DeferredScopeBasisV3::EstablishedFromDeferredV3(
                TransitionFacts::new(&last.selection.locator)?.basis,
            )
        } else {
            DeferredScopeBasisV3::DeferredV2(original.basis)
        };
        if !expected_matches(&basis, expected) {
            return Err(refusal(
                "basis_conflict",
                "Expected exact current basis differs",
            ));
        }
        native.release().map_err(fs_refusal)?;
        guard.release().map_err(fs_refusal)?;
        Ok(basis)
    }
}

struct TransitionFacts {
    staged: Vec<Vec<u8>>,
    intent: Vec<u8>,
    decision: Vec<u8>,
    terminal: Vec<u8>,
    noncommit_bytes: Vec<u8>,
    outcome: DeferredEstablishmentCommittedOutcomeV3,
    noncommit: DeferredEstablishmentNotCommittedOutcomeV3,
    basis: EstablishedFromDeferredBasisV3,
}
impl TransitionFacts {
    fn new(locator: &DeferredEstablishmentLocatorV3) -> Result<Self, GovernanceRefusalV1> {
        let validated = validate_deferred_establishment_locator(locator)?;
        let validation_fingerprint = governance_fingerprint(
            "handbook.admission-governance-validation@3.0.0",
            &(locator, &validated, "exact_owner_conformance_confirmed"),
        )?;
        let advisory_permission_fingerprint = governance_fingerprint(
            "handbook.admission-governance-advisory-permission@3.0.0",
            &(
                locator,
                &validated.instruction_fingerprint,
                &validated.establishment_rule.binding,
                "accepted_standalone_local_advisory_assertion",
            ),
        )?;
        let outcome_fingerprint = governance_fingerprint(
            "handbook.admission-governance-outcome@3.0.0",
            &(
                locator,
                &validated,
                &validation_fingerprint,
                &advisory_permission_fingerprint,
                "validation_confirmed",
                "local_advisory_permission_accepted",
                "first_establishment_committed",
            ),
        )?;
        let outcome = DeferredEstablishmentCommittedOutcomeV3 {
            locator: locator.clone(),
            validated: validated.clone(),
            validation_confirmed: true,
            advisory_permission_accepted: true,
            activation_committed: true,
            validation_fingerprint,
            advisory_permission_fingerprint,
            outcome_fingerprint: outcome_fingerprint.clone(),
        };
        let relation_fingerprint = governance_fingerprint(
            "handbook.admission-governance-current-relation@3.0.0",
            &(
                locator,
                &validated,
                &outcome_fingerprint,
                "established_from_exact_deferred",
            ),
        )?;
        let input = &locator.request;
        let basis = EstablishedFromDeferredBasisV3 {
            locator: locator.clone(),
            project: input.project.clone(),
            scope: input.scope.clone(),
            predecessor: input.expected_deferred.clone(),
            configuration: input.configuration.clone(),
            configuration_fingerprint: validated.configuration_fingerprint.clone(),
            replacement_basis: input.replacement_basis.clone(),
            establishment_rule: validated.establishment_rule.clone(),
            owner_rule: validated.owner_rule.clone(),
            request_fingerprint: locator.request_fingerprint.clone(),
            outcome_fingerprint,
            relation_fingerprint,
        };
        let staged = vec![
            governance_canonical_bytes(&(&validated.establishment_rule, &validated.owner_rule))?,
            governance_canonical_bytes(&(
                &input.configuration,
                &validated.configuration_fingerprint,
            ))?,
            governance_canonical_bytes(&(&input.instruction, &validated.instruction_fingerprint))?,
            governance_canonical_bytes(&input.replacement_basis)?,
            governance_canonical_bytes(&outcome)?,
            governance_canonical_bytes(&(
                &input.project,
                &input.scope,
                &input.expected_deferred,
                &basis.relation_fingerprint,
            ))?,
            governance_canonical_bytes(&basis)?,
        ];
        let intent = governance_canonical_bytes(&(
            "handbook.admission-establishment-intent.v3",
            locator,
            staged
                .iter()
                .map(|b| governance_fingerprint("handbook.admission-stage-bytes@3.0.0", b))
                .collect::<Result<Vec<_>, _>>()?,
        ))?;
        let decision = governance_canonical_bytes(&(
            "handbook.admission-establishment-decision.v3",
            governance_fingerprint("handbook.admission-intent-bytes@3.0.0", &intent)?,
            &input.expected_deferred,
        ))?;
        let terminal = governance_canonical_bytes(&(
            "handbook.admission-establishment-terminal.v3",
            &basis.outcome_fingerprint,
            &basis.relation_fingerprint,
            governance_fingerprint("handbook.admission-decision-bytes@3.0.0", &decision)?,
        ))?;
        let condition = "verified_predecision_noncommit".to_owned();
        let noncommit = DeferredEstablishmentNotCommittedOutcomeV3 {
            locator: locator.clone(),
            activation_committed: false,
            outcome_fingerprint: governance_fingerprint(
                "handbook.admission-noncommit@3.0.0",
                &(locator, &condition),
            )?,
            condition,
        };
        let noncommit_bytes = governance_canonical_bytes(&noncommit)?;
        Ok(Self {
            staged,
            intent,
            decision,
            terminal,
            noncommit_bytes,
            outcome,
            noncommit,
            basis,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SelectionV3 {
    sequence: usize,
    locator: DeferredEstablishmentLocatorV3,
    previous_selection: FingerprintV1,
    previous_terminal: FingerprintV1,
}
impl SelectionV3 {
    fn fingerprint(&self) -> Result<FingerprintV1, GovernanceRefusalV1> {
        governance_fingerprint("handbook.admission-attempt-selection@3.0.0", self)
    }
}
#[derive(Default)]
struct TransitionState {
    decision: bool,
    installed: usize,
    terminal: bool,
    noncommit: bool,
}
struct HistoryEntry {
    selection: SelectionV3,
    state: TransitionState,
    closure: Option<FingerprintV1>,
}
fn selection_dir(seq: usize) -> String {
    format!("{ESTABLISHMENT_SELECTION_PATH_V3}/{seq:04}")
}
fn selection_path(seq: usize, name: &str) -> String {
    format!("{}/{name}", selection_dir(seq))
}
pub fn transition_request_storage_path_v3(locator: &DeferredEstablishmentLocatorV3) -> String {
    format!(
        "{GOVERNANCE_STORAGE_PATH_V2}/requests/{}",
        request_name(locator)
    )
}
fn request_name(locator: &DeferredEstablishmentLocatorV3) -> String {
    locator
        .request_fingerprint
        .as_str()
        .strip_prefix("sha256:")
        .unwrap_or("invalid")
        .into()
}
fn t_path(locator: &DeferredEstablishmentLocatorV3, name: &str) -> String {
    format!("{}/{name}", transition_request_storage_path_v3(locator))
}
fn t_write(
    root: &RetainedRepositoryRootV1,
    locator: &DeferredEstablishmentLocatorV3,
    name: &str,
    bytes: &[u8],
) -> Result<(), GovernanceRefusalV1> {
    root.write_new_durable(&t_path(locator, name), bytes)
        .map_err(fs_refusal)
}
fn closure_bytes(
    selection: &SelectionV3,
    facts: &TransitionFacts,
    noncommit: bool,
) -> Result<Vec<u8>, GovernanceRefusalV1> {
    governance_canonical_bytes(&(
        "handbook.admission-attempt-closure.v3",
        selection.fingerprint()?,
        governance_fingerprint(
            "handbook.admission-terminal-bytes@3.0.0",
            if noncommit {
                &facts.noncommit_bytes
            } else {
                &facts.terminal
            },
        )?,
        if noncommit {
            "not_committed"
        } else {
            "committed"
        },
    ))
}
fn selections(root: &RetainedRepositoryRootV1) -> Result<Vec<SelectionV3>, GovernanceRefusalV1> {
    let entries = root
        .list_directory_bounded(GOVERNANCE_STORAGE_PATH_V2, 4)
        .map_err(fs_refusal)?;
    if !entries.iter().any(|e| e.name() == "establishment-v3") {
        return Ok(vec![]);
    }
    let entries = root
        .list_directory_bounded(
            ESTABLISHMENT_SELECTION_PATH_V3,
            MAX_ADMISSION_REQUESTS_V3 - 1,
        )
        .map_err(fs_refusal)?;
    if entries.is_empty() {
        return Err(refusal(
            "selection_incomplete",
            "V3 footprint has no complete selection",
        ));
    }
    let mut result = Vec::new();
    for (i, e) in entries.iter().enumerate() {
        if e.kind() != Kind::Directory || e.name() != format!("{:04}", i + 1) {
            return Err(refusal(
                "selection_nonprefix",
                "Selected history has a missing or unsupported attempt",
            ));
        }
        let names = root
            .list_directory_bounded(&selection_dir(i + 1), 2)
            .map_err(fs_refusal)?;
        if names.iter().any(|e| {
            e.kind() != Kind::RegularFile || !matches!(e.name(), "selection.json" | "closure.json")
        }) {
            return Err(refusal(
                "selection_inventory",
                "Unsupported selection evidence",
            ));
        }
        let bytes = read(root, &selection_path(i + 1, "selection.json"))?.ok_or_else(|| {
            refusal(
                "selection_missing",
                "Complete locator selection unavailable",
            )
        })?;
        let selection: SelectionV3 = decode_governance_json(&bytes)?;
        validate_deferred_establishment_locator(&selection.locator)?;
        if bytes != governance_canonical_bytes(&selection)? || selection.sequence != i + 1 {
            return Err(refusal(
                "selection_identity",
                "Exact selected sequence differs",
            ));
        }
        result.push(selection);
    }
    Ok(result)
}
pub(super) fn selected_request_names(
    root: &RetainedRepositoryRootV1,
) -> Result<Vec<String>, GovernanceRefusalV1> {
    Ok(selections(root)?
        .iter()
        .map(|s| request_name(&s.locator))
        .collect())
}
fn inspect_history(
    root: &RetainedRepositoryRootV1,
    original: &DeferredGovernanceBasisV2,
    current: bool,
) -> Result<Vec<HistoryEntry>, GovernanceRefusalV1> {
    let mut history: Vec<HistoryEntry> = vec![];
    let mut previous_selection = original.relation_fingerprint.clone();
    let mut previous_terminal = original.outcome_fingerprint.clone();
    let selected = selections(root)?;
    let last_sequence = selected.len();
    for selection in selected {
        let request = &selection.locator.request;
        if request.project != original.project
            || request.scope != original.scope
            || request.expected_deferred.outcome_fingerprint != original.outcome_fingerprint
            || request.expected_deferred.relation_fingerprint != original.relation_fingerprint
            || selection.previous_selection != previous_selection
            || selection.previous_terminal != previous_terminal
            || history.iter().any(|e| {
                e.selection.locator.request.request_key == request.request_key
                    || e.state.terminal
                    || e.closure.is_none()
            })
        {
            return Err(refusal(
                "attempt_chain_conflict",
                "Every attempt must link exact original D and previous closed noncommit",
            ));
        }
        let facts = TransitionFacts::new(&selection.locator)?;
        let state = inspect_attempt(
            root,
            &selection.locator,
            &facts,
            original,
            current && selection.sequence == last_sequence,
        )?;
        let closure = match read(root, &selection_path(selection.sequence, "closure.json"))? {
            Some(bytes) => {
                if !(state.terminal || state.noncommit)
                    || bytes != closure_bytes(&selection, &facts, state.noncommit)?
                {
                    return Err(refusal(
                        "closure_incomplete",
                        "Selected terminal closure lacks exact required history",
                    ));
                }
                Some(governance_fingerprint(
                    "handbook.admission-selection-closure-bytes@3.0.0",
                    &bytes,
                )?)
            }
            None => None,
        };
        previous_selection = selection.fingerprint()?;
        if let Some(closure) = &closure {
            previous_terminal = closure.clone();
        }
        history.push(HistoryEntry {
            selection,
            state,
            closure,
        });
    }
    if current {
        let expected = if let Some(last) = history.last().filter(|e| e.state.terminal) {
            TransitionFacts::new(&last.selection.locator)?.staged[6].clone()
        } else {
            governance_canonical_bytes(original)?
        };
        if history.last().is_none_or(|e| !e.state.decision) {
            require(root, GOVERNANCE_CURRENT_PATH_V2, &expected)?;
        }
    }
    Ok(history)
}
fn inspect_attempt(
    root: &RetainedRepositoryRootV1,
    locator: &DeferredEstablishmentLocatorV3,
    facts: &TransitionFacts,
    original: &DeferredGovernanceBasisV2,
    current: bool,
) -> Result<TransitionState, GovernanceRefusalV1> {
    let names = root
        .list_directory_bounded(&transition_request_storage_path_v3(locator), 32)
        .map_err(fs_refusal)?;
    let allowed: Vec<String> = [
        "request.json",
        "intent.json",
        "decision.json",
        "terminal.json",
        "noncommit.json",
        "current-prepared.json",
        "current-displaced.json",
    ]
    .into_iter()
    .chain(TARGETS_V3)
    .map(str::to_owned)
    .chain((0..7).map(|i| format!("stage-{i}.json")))
    .collect();
    if names
        .iter()
        .any(|e| e.kind() != Kind::RegularFile || !allowed.iter().any(|name| name == e.name()))
    {
        return Err(refusal(
            "attempt_inventory",
            "Unsupported or unsafe request history",
        ));
    }
    require(
        root,
        &t_path(locator, "request.json"),
        &locator.canonical_json_bytes()?,
    )?;
    let check = |name: &str, bytes: &[u8]| -> Result<bool, GovernanceRefusalV1> {
        if names.iter().any(|e| e.name() == name) {
            require(root, &t_path(locator, name), bytes)?;
            Ok(true)
        } else {
            Ok(false)
        }
    };
    let mut staged = 0;
    for (i, b) in facts.staged.iter().enumerate() {
        if check(&format!("stage-{i}.json"), b)? {
            if staged != i {
                return Err(refusal("stage_nonprefix", "Missing exact staged prefix"));
            }
            staged += 1;
        }
    }
    let intent = check("intent.json", &facts.intent)?;
    let decision = check("decision.json", &facts.decision)?;
    if (intent && staged != 7) || (decision && !intent) {
        return Err(refusal(
            "decision_nonprefix",
            "Intent and decision require all exact staging",
        ));
    }
    let mut installed = 0;
    for (i, target) in TARGETS_V3.iter().enumerate() {
        if check(target, &facts.staged[i])? {
            if !decision || installed != i {
                return Err(refusal(
                    "installation_nonprefix",
                    "Installation lacks exact decision/prefix",
                ));
            }
            installed += 1;
        }
    }
    let terminal = check("terminal.json", &facts.terminal)?;
    let noncommit = check("noncommit.json", &facts.noncommit_bytes)?;
    let prepared = check("current-prepared.json", &facts.staged[6])?;
    let displaced = check(
        "current-displaced.json",
        &governance_canonical_bytes(original)?,
    )?;
    if (terminal && installed != 7)
        || (noncommit && (decision || installed != 0 || terminal || prepared || displaced))
        || ((prepared || displaced) && (installed != 7 || !decision))
    {
        return Err(refusal(
            "terminal_nonprefix",
            "Terminal/current replacement evidence conflicts",
        ));
    }
    if current {
        let live = read(root, GOVERNANCE_CURRENT_PATH_V2)?;
        let d = governance_canonical_bytes(original)?;
        let legal = match (live.as_deref(), prepared, displaced) {
            (Some(bytes), false, false) => bytes == d,
            (Some(bytes), true, false) => bytes == d,
            (None, true, true) => true,
            (Some(bytes), false, true) => bytes == facts.staged[6],
            _ => false,
        };
        if !legal
            || (terminal
                && (prepared || !displaced || live.as_deref() != Some(facts.staged[6].as_slice())))
        {
            return Err(refusal(
                "current_nonprefix",
                "Exact terminal-selected current relation unavailable",
            ));
        }
    } else if terminal && (prepared || !displaced) {
        return Err(refusal(
            "replacement_history_incomplete",
            "Terminal requires original displaced D and completed publication",
        ));
    }
    Ok(TransitionState {
        decision,
        installed,
        terminal,
        noncommit,
    })
}
pub(super) fn validate_original_replay(
    root: &RetainedRepositoryRootV1,
    original: &DeferredGovernanceBasisV2,
) -> Result<(), GovernanceRefusalV1> {
    inspect_history(root, original, false).map(|_| ())
}
fn find_request<'a>(
    history: &'a [HistoryEntry],
    locator: &DeferredEstablishmentLocatorV3,
) -> Result<Option<&'a HistoryEntry>, GovernanceRefusalV1> {
    if let Some(entry) = history
        .iter()
        .find(|e| e.selection.locator.request.request_key == locator.request.request_key)
    {
        if entry.selection.locator != *locator {
            return Err(refusal(
                "request_conflict",
                "Retained request key binds different complete content",
            ));
        }
        Ok(Some(entry))
    } else {
        Ok(None)
    }
}
fn require_eligible(
    root: &RetainedRepositoryRootV1,
    original: &deferral::OriginalDeferredSnapshot,
    history: &[HistoryEntry],
    locator: &DeferredEstablishmentLocatorV3,
) -> Result<(), GovernanceRefusalV1> {
    find_request(history, locator)?;
    if history
        .iter()
        .any(|e| e.state.terminal || !e.state.noncommit || e.closure.is_none())
    {
        return Err(refusal(
            "establishment_unavailable",
            "Earlier attempt must resolve; established governance cannot bootstrap",
        ));
    }
    if !history_capacity_available(history.len()) {
        return Err(refusal(
            "request_history_exhausted",
            "Lifetime retained request bound includes original D",
        ));
    }
    require_exact_predecessor(root, original, locator)
}
fn history_capacity_available(previous_attempts: usize) -> bool {
    previous_attempts < MAX_ADMISSION_REQUESTS_V3 - 1
}
#[cfg(test)]
mod capacity_tests {
    #[test]
    fn lifetime_bound_counts_original_d_and_every_retained_noncommit() {
        assert!(super::history_capacity_available(4094));
        assert!(!super::history_capacity_available(4095));
        assert!(!super::history_capacity_available(usize::MAX));
    }
}
fn require_exact_predecessor(
    root: &RetainedRepositoryRootV1,
    original: &deferral::OriginalDeferredSnapshot,
    locator: &DeferredEstablishmentLocatorV3,
) -> Result<(), GovernanceRefusalV1> {
    if locator.request.expected_deferred.outcome_fingerprint != original.basis.outcome_fingerprint
        || locator.request.expected_deferred.relation_fingerprint
            != original.basis.relation_fingerprint
    {
        return Err(refusal(
            "basis_conflict",
            "Expected current D differs from exact committed predecessor",
        ));
    }
    require(root, GOVERNANCE_CURRENT_PATH_V2, &original.current)
}
fn exact_read_root(
    path: &Path,
    project: &ProjectBindingV2,
) -> Result<RetainedRepositoryRootV1, GovernanceRefusalV1> {
    if path.to_str() != Some(&project.retained_root) {
        return Err(refusal("root_substituted", "Exact retained root differs"));
    }
    let root = RetainedRepositoryRootV1::open_exact(path).map_err(fs_refusal)?;
    if root
        .governance_physical_root_fingerprint_v1()
        .map_err(fs_refusal)?
        != project.physical_root_fingerprint
    {
        return Err(refusal(
            "physical_root_substituted",
            "Exact physical root differs",
        ));
    }
    Ok(root)
}
fn transition_guard(
    path: &Path,
    project: &ProjectBindingV2,
) -> Result<RepositoryInitializationGuardV1, GovernanceRefusalV1> {
    let root = exact_read_root(path, project)?;
    let entries = root.list_directory_bounded("", 4096).map_err(fs_refusal)?;
    if !entries.iter().any(|e| {
        e.name() == handbook_engine::REPOSITORY_INITIALIZATION_LOCK_PATH_V1
            && e.kind() == Kind::RegularFile
    }) {
        return Err(refusal(
            "lock_missing",
            "Original root guard cannot be recreated",
        ));
    }
    deferral::exact_guard(path, project)
}
fn owner_lock(
    root: &RetainedRepositoryRootV1,
) -> Result<RepositoryExclusiveLockV1, GovernanceRefusalV1> {
    require_existing_lock(root)?;
    root.acquire_exclusive_lock(LOCK).map_err(fs_refusal)
}
fn require_original(
    guard: &RepositoryInitializationGuardV1,
    native: &RepositoryExclusiveLockV1,
    project: &ProjectBindingV2,
) -> Result<deferral::OriginalDeferredSnapshot, GovernanceRefusalV1> {
    validate_locks(guard, native)?;
    let original = deferral::original_deferred_snapshot(guard.root(), project, Some(native))?;
    guard
        .require_native_terminal(
            &original.locator.initialization_claim,
            RepositoryInitializationDispositionV1::Committed,
            &original.terminal_path,
            &original.terminal,
        )
        .map_err(fs_refusal)?;
    Ok(original)
}
fn validate_locks(
    guard: &RepositoryInitializationGuardV1,
    native: &RepositoryExclusiveLockV1,
) -> Result<(), GovernanceRefusalV1> {
    guard.validate().map_err(fs_refusal)?;
    native.validate_at(guard.root(), LOCK).map_err(fs_refusal)
}
fn uncertain(
    locator: &DeferredEstablishmentLocatorV3,
    error: GovernanceRefusalV1,
) -> DeferredEstablishmentExecutionV3 {
    DeferredEstablishmentExecutionV3::Uncertain {
        locator: locator.clone(),
        condition: error.condition,
    }
}
fn expected_matches(
    basis: &DeferredScopeBasisV3,
    expected: Option<&ExpectedProjectAdmissionBasisV3>,
) -> bool {
    match (basis, expected) {
        (_, None) => true,
        (
            DeferredScopeBasisV3::DeferredV2(b),
            Some(ExpectedProjectAdmissionBasisV3::DeferredV2(e)),
        ) => {
            b.outcome_fingerprint == e.outcome_fingerprint
                && b.relation_fingerprint == e.relation_fingerprint
        }
        (
            DeferredScopeBasisV3::EstablishedFromDeferredV3(b),
            Some(ExpectedProjectAdmissionBasisV3::EstablishedFromDeferredV3(e)),
        ) => {
            b.configuration_fingerprint == e.configuration_fingerprint
                && b.outcome_fingerprint == e.outcome_fingerprint
                && b.relation_fingerprint == e.relation_fingerprint
        }
        _ => false,
    }
}
