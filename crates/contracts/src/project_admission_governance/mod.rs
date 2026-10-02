//! One native new-project establishment. No candidate admission or replacement API.
//!
//! Lock order: owner process mutex, retained native governance lock, then Engine
//! prepared identity persistence (which takes no registry/Context Resolution lock).
//! The immutable reservation anchor is never reset. Its consumption is an append
//! in the same decided transaction as activation; usable truth requires terminal
//! closure of every installed fact. Recovery does not allocate an identity.

mod types;
pub use types::*;

use crate::FingerprintV1;
use handbook_engine::{
    PreparedRepositoryIdentityV1, RepositoryDirectoryEntryKindV1, RepositoryDurabilityErrorKindV1,
    RepositoryInvocationIdentityServiceV1, RepositoryReadV1, RetainedRepositoryRootV1,
    REPOSITORY_IDENTITY_REPO_PATH,
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
pub struct ProjectAdmissionGovernanceV1 {
    #[cfg(feature = "test-support")]
    fault: Option<GovernanceFaultBoundaryV1>,
    #[cfg(feature = "test-support")]
    crash: bool,
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
    fn boundary(&self, boundary: GovernanceFaultBoundaryV1) -> Result<(), GovernanceRefusalV1> {
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
            .any(|e| e.name() == ".handbook")
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
            // An AlreadyExists error is positive evidence this call did not reserve.
            may_have_reserved = true;
            match retained.create_directory_new_durable(".handbook") {
                Ok(()) => {}
                Err(error) if error.kind() == RepositoryDurabilityErrorKindV1::AlreadyExists => {
                    return Ok(GovernanceExecutionV1::Refused {
                        refusal: refusal(
                            "initial_registration_unavailable",
                            "An existing root cannot receive new-project registration",
                        )
                        .with_project(locator.project.clone()),
                    })
                }
                Err(error) => return Err(fs_refusal(error)),
            }
            self.boundary(GovernanceFaultBoundaryV1::Reservation)?;
            let _native = retained.acquire_exclusive_lock(LOCK).map_err(fs_refusal)?;
            retained
                .write_new_durable(
                    GOVERNANCE_ANCHOR_PATH_V1,
                    &governance_canonical_bytes(&locator)?,
                )
                .map_err(fs_refusal)?;
            self.boundary(GovernanceFaultBoundaryV1::Anchor)?;
            RepositoryInvocationIdentityServiceV1::new()
                .persist_prepared_repository_identity(&retained, identity)
                .map_err(fs_refusal)?;
            self.boundary(GovernanceFaultBoundaryV1::Identity)?;
            retained
                .create_directories(GOVERNANCE_STORAGE_PATH_V1)
                .map_err(fs_refusal)?;
            let facts = Facts::new(&locator)?;
            write(&retained, "registration.json", &facts.registration)?;
            self.boundary(GovernanceFaultBoundaryV1::Registration)?;
            for (i, bytes) in facts.staged.iter().enumerate() {
                write(&retained, &format!("stage-{i}.json"), bytes)?;
                self.boundary(GovernanceFaultBoundaryV1::Stage(i))?;
            }
            write(&retained, "intent.json", &facts.intent)?;
            self.boundary(GovernanceFaultBoundaryV1::Intent)?;
            // Compare exact positive unused registration, request and no predecessor
            // immediately before the decision. Every current fact must be a prefix.
            let state = inspect(&retained, &locator, &facts)?;
            if !state.intent
                || state.decision
                || state.installed != 0
                || state.not_committed
                || state.terminal
            {
                return Err(refusal(
                    "initial_basis_consumed",
                    "Commit requires exact unused registration and no active predecessor",
                ));
            }
            write(&retained, "decision.json", &facts.decision)?;
            self.boundary(GovernanceFaultBoundaryV1::Decision)?;
            self.finish(&retained, &facts, 0)?;
            inspect(&retained, &locator, &facts)?;
            self.boundary(GovernanceFaultBoundaryV1::Reply)?;
            _native.release().map_err(fs_refusal)?;
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
            let _process = PROCESS_LOCK
                .lock()
                .map_err(|_| refusal("lock_unavailable", "Owner process lock is unavailable"))?;
            let retained = exact_root(root.as_ref(), &locator.project)?;
            require_existing_lock(&retained)?;
            let native = retained.acquire_exclusive_lock(LOCK).map_err(fs_refusal)?;
            let original_bytes = read(&retained, GOVERNANCE_ANCHOR_PATH_V1)?.ok_or_else(|| {
                refusal(
                    "registration_missing",
                    "Original reservation anchor is unavailable",
                )
            })?;
            let original: GovernanceRequestLocatorV1 = decode_governance_json(&original_bytes)?;
            validate_request_locator(&original)?;
            if original != *locator {
                native.release().map_err(fs_refusal)?;
                return Ok(GovernanceExecutionV1::Refused {
                    refusal: refusal(
                        "request_or_configuration_conflict",
                        "Original registration binds different exact request/configuration content",
                    )
                    .with_project(original.project),
                });
            }
            let facts = Facts::new(locator)?;
            let state = inspect(&retained, locator, &facts)?;
            if state.terminal {
                native.release().map_err(fs_refusal)?;
                return Ok(GovernanceExecutionV1::Committed {
                    outcome: facts.outcome,
                    delivery: GovernanceDeliveryV1::Replay,
                });
            }
            if state.decision {
                self.finish(&retained, &facts, state.installed)?;
                let verified = inspect(&retained, locator, &facts)?;
                if !verified.terminal {
                    return Err(refusal("terminal_missing", "Commit has not closed"));
                }
                native.release().map_err(fs_refusal)?;
                return Ok(GovernanceExecutionV1::Committed {
                    outcome: facts.outcome,
                    delivery: GovernanceDeliveryV1::Replay,
                });
            }
            // Only an exact verified predecision prefix with no installation can
            // close as noncommit. A consumed reservation cannot be reinitialized.
            if !state.not_committed {
                retained
                    .create_directories(GOVERNANCE_STORAGE_PATH_V1)
                    .map_err(fs_refusal)?;
                write(&retained, "not-committed.json", &facts.not_committed)?;
            }
            inspect(&retained, locator, &facts)?;
            native.release().map_err(fs_refusal)?;
            Ok(GovernanceExecutionV1::NotCommitted {
                outcome: facts.noncommit,
                delivery: GovernanceDeliveryV1::Replay,
            })
        })();
        match result {
            Ok(result) => result,
            Err(error) => GovernanceExecutionV1::Uncertain {
                locator: locator.clone(),
                condition: error.condition,
            },
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
        if scope != GOVERNANCE_SCOPE_V1 {
            return Err(refusal(
                "unsupported_scope",
                "Only the exact Contracts freshness root scope is supported",
            ));
        }
        let locator = {
            let retained = exact_root(root.as_ref(), project)?;
            let bytes = read(&retained, GOVERNANCE_ANCHOR_PATH_V1)?.ok_or_else(|| {
                refusal("registration_missing", "No retained registration anchor")
            })?;
            let locator: GovernanceRequestLocatorV1 = decode_governance_json(&bytes)?;
            if locator.project != *project {
                return Err(refusal(
                    "project_substituted",
                    "Anchor binds a different project",
                ));
            }
            locator
        };
        match self.resolve_request(root.as_ref(), &locator) {
            GovernanceExecutionV1::Committed { outcome, .. } => {
                // Reacquire the same locks and recheck, keeping replay and current
                // selection separate. No replay result alone proves active truth.
                let _process = PROCESS_LOCK.lock().map_err(|_| {
                    refusal("lock_unavailable", "Owner process lock is unavailable")
                })?;
                let retained = exact_root(root.as_ref(), project)?;
                require_existing_lock(&retained)?;
                let native = retained.acquire_exclusive_lock(LOCK).map_err(fs_refusal)?;
                let facts = Facts::new(&locator)?;
                let state = inspect(&retained, &locator, &facts)?;
                if !state.terminal {
                    return Err(refusal(
                        "active_unverifiable",
                        "Complete terminal governance is required",
                    ));
                }
                if let Some(expected) = expected {
                    if expected.configuration_fingerprint != facts.active.configuration_fingerprint
                        || expected.outcome_fingerprint != outcome.outcome_fingerprint
                    {
                        return Err(refusal(
                            "basis_conflict",
                            "Expected exact configuration/outcome differs",
                        ));
                    }
                }
                native.release().map_err(fs_refusal)?;
                Ok(facts.active)
            }
            GovernanceExecutionV1::Refused { refusal } => Err(refusal),
            GovernanceExecutionV1::NotCommitted { .. } => Err(refusal(
                "governance_not_committed",
                "Registration did not activate governance",
            )),
            GovernanceExecutionV1::Uncertain { condition, .. } => {
                Err(refusal("active_unverifiable", condition))
            }
        }
    }

    fn finish(
        &self,
        retained: &RetainedRepositoryRootV1,
        facts: &Facts,
        installed: usize,
    ) -> Result<(), GovernanceRefusalV1> {
        for (i, target) in TARGETS.iter().enumerate().skip(installed) {
            write(retained, target, &facts.staged[i])?;
            self.boundary(GovernanceFaultBoundaryV1::Installation(i))?;
        }
        write(retained, "terminal.json", &facts.terminal)?;
        self.boundary(GovernanceFaultBoundaryV1::Terminal)
    }
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
