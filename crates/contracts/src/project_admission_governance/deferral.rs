//! One positive initial scoped deferral. All history is outside resettable state.
//! Lock order: shared Contracts process mutex, root initialization guard, owner lock.
use super::deferral_types::*;
use super::types::{decode_governance_json, governance_canonical_bytes, governance_fingerprint};
use super::{fs_refusal, refusal, GovernanceDeliveryV1, GovernanceRefusalV1, PROCESS_LOCK};
use crate::FingerprintV1;
use handbook_engine::{
    PreparedRepositoryIdentityV1, RepositoryDirectoryEntryKindV1, RepositoryInitializationClaimV1,
    RepositoryInitializationDispositionV1, RepositoryInitializationGuardV1,
    RepositoryInvocationIdentityServiceV1, RetainedRepositoryRootV1, REPOSITORY_IDENTITY_REPO_PATH,
};
use serde::{Deserialize, Serialize};
use std::path::Path;
#[cfg(feature = "test-support")]
use std::path::PathBuf;

pub const GOVERNANCE_ANCHOR_PATH_V2: &str = ".handbook/admission-governance-project.v2.json";
pub const GOVERNANCE_STORAGE_PATH_V2: &str = ".handbook/admission-governance/v2";
pub const DEFERRAL_NONCOMMIT_PATH_V2: &str = ".handbook-admission-deferral-noncommit.v2.json";
pub const GOVERNANCE_CURRENT_PATH_V2: &str = ".handbook/admission-governance/v2/scope-current.json";
const TARGETS: [&str; 6] = [
    "rule.json",
    "instruction.json",
    "deferral.json",
    "outcome.json",
    "consumed.json",
    "relation.json",
];

/// The owner anchor binds its retained lock; the portable request locator does not.
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct AnchorV2 {
    locator: DeferralRequestLocatorV2,
    governance_lock_fingerprint: String,
}

/// Single-use capability; its serializable locator can only resolve the request.
#[derive(Debug)]
pub struct PreparedAdmissionDeferralV2 {
    locator: DeferralRequestLocatorV2,
    identity: PreparedRepositoryIdentityV1,
}
impl PreparedAdmissionDeferralV2 {
    pub fn locator(&self) -> &DeferralRequestLocatorV2 {
        &self.locator
    }
    /// Pure prospective identity for a composed owner request, not current
    /// applicability. Execution and consuming use still require native resolution.
    pub fn prospective_deferred_basis(
        &self,
    ) -> Result<DeferredGovernanceBasisV2, GovernanceRefusalV1> {
        self.locator.prospective_deferred_basis()
    }
}
impl DeferralRequestLocatorV2 {
    /// Recomputes the exact initial relation through the existing deferral facts.
    /// It grants no applicability, qualification, permission or currentness.
    pub fn prospective_deferred_basis(
        &self,
    ) -> Result<DeferredGovernanceBasisV2, GovernanceRefusalV1> {
        Ok(Facts::new(self)?.basis)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeferralCommittedOutcomeV2 {
    pub locator: DeferralRequestLocatorV2,
    pub validated: ValidatedDeferralV2,
    pub validation_confirmed: bool,
    pub advisory_permission_accepted: bool,
    pub deferral_committed: bool,
    pub outcome_fingerprint: FingerprintV1,
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeferralNotCommittedOutcomeV2 {
    pub locator: DeferralRequestLocatorV2,
    pub condition: String,
    pub deferral_committed: bool,
    pub outcome_fingerprint: FingerprintV1,
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case", deny_unknown_fields)]
pub enum DeferralExecutionV2 {
    Committed {
        outcome: DeferralCommittedOutcomeV2,
        delivery: GovernanceDeliveryV1,
    },
    Refused {
        refusal: GovernanceRefusalV1,
    },
    NotCommitted {
        outcome: DeferralNotCommittedOutcomeV2,
        delivery: GovernanceDeliveryV1,
    },
    Uncertain {
        locator: DeferralRequestLocatorV2,
        condition: String,
    },
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExpectedDeferredBasisV2 {
    pub outcome_fingerprint: FingerprintV1,
    pub relation_fingerprint: FingerprintV1,
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeferredGovernanceBasisV2 {
    pub project: ProjectBindingV2,
    pub scope: String,
    pub never_established: bool,
    pub registration_identity: FingerprintV1,
    pub deferral_fingerprint: FingerprintV1,
    pub outcome_fingerprint: FingerprintV1,
    pub relation_fingerprint: FingerprintV1,
    pub establishment_rule: GovernanceRuleDescriptorV2,
}

impl DeferralExecutionV2 {
    /// Decodes bounded exact wire content. It does not attest owner delivery or
    /// current truth; use the restored locator with the native resolver.
    pub fn from_json_bytes(bytes: &[u8]) -> Result<Self, GovernanceRefusalV1> {
        let value: Self = decode_governance_json(bytes)?;
        value.canonical_json_bytes()?;
        Ok(value)
    }
    pub fn canonical_json_bytes(&self) -> Result<Vec<u8>, GovernanceRefusalV1> {
        let valid = match self {
            Self::Committed { outcome, .. } => *outcome == Facts::new(&outcome.locator)?.outcome,
            Self::NotCommitted { outcome, .. } => {
                *outcome == Facts::new(&outcome.locator)?.noncommit
            }
            Self::Uncertain { locator, condition } => {
                validate_deferral_locator(locator)?;
                !condition.is_empty() && condition.len() <= 4096
            }
            Self::Refused { refusal } => {
                !refusal.condition.is_empty() && !refusal.detail.is_empty()
            }
        };
        if !valid {
            return Err(refusal(
                "outcome_identity",
                "Exact outcome wire content is inconsistent",
            ));
        }
        governance_canonical_bytes(self)
    }
}

impl DeferredGovernanceBasisV2 {
    /// Restored wire content requires native current resolution; it grants no
    /// qualification, evaluation permission, delegation or action authority.
    pub fn from_json_bytes(bytes: &[u8]) -> Result<Self, GovernanceRefusalV1> {
        let value: Self = decode_governance_json(bytes)?;
        value.canonical_json_bytes()?;
        Ok(value)
    }
    pub fn canonical_json_bytes(&self) -> Result<Vec<u8>, GovernanceRefusalV1> {
        let relation = governance_fingerprint(
            "handbook.admission-governance-current-relation@2.0.0",
            &(
                &self.project,
                &self.scope,
                &self.deferral_fingerprint,
                &self.outcome_fingerprint,
                "deferred_never_established",
            ),
        )?;
        if self.scope != super::ADMISSION_GOVERNANCE_SCOPE_V1
            || !self.never_established
            || FingerprintV1::parse(&self.project.repository_identity_fingerprint).is_err()
            || FingerprintV1::parse(&self.project.physical_root_fingerprint).is_err()
            || self.project.retained_root.len() > 4096
            || self.project.retained_root.contains('\0')
            || !Path::new(&self.project.retained_root).is_absolute()
            || self.registration_identity != self.project.registration_identity
            || self.registration_identity
                != deferral_registration_fingerprint(
                    &self.project.repository_identity_fingerprint,
                    &self.project.retained_root,
                    &self.project.physical_root_fingerprint,
                )?
            || self.relation_fingerprint != relation
            || self.establishment_rule != establishment_rule_descriptor_v2()?
        {
            return Err(refusal(
                "basis_identity",
                "Exact deferred basis wire content is inconsistent",
            ));
        }
        governance_canonical_bytes(self)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[doc(hidden)]
pub enum DeferralFaultBoundaryV2 {
    Claim,
    Reservation,
    Anchor,
    Identity,
    Registration,
    Stage(usize),
    Intent,
    Decision,
    Installation(usize),
    CurrentRelation,
    Terminal,
    ClaimReceipt,
    NonCommitWitness,
    NonCommitReceipt,
    Reply,
}
#[derive(Default)]
pub struct ProjectAdmissionDeferralV2 {
    #[cfg(feature = "test-support")]
    fault: Option<DeferralFaultBoundaryV2>,
    #[cfg(feature = "test-support")]
    crash: bool,
    #[cfg(feature = "test-support")]
    pause: Option<(DeferralFaultBoundaryV2, PathBuf, PathBuf)>,
}
impl ProjectAdmissionDeferralV2 {
    pub fn new() -> Self {
        Self::default()
    }
    #[cfg(feature = "test-support")]
    #[doc(hidden)]
    pub fn with_fault_for_testing(mut self, fault: DeferralFaultBoundaryV2) -> Self {
        self.fault = Some(fault);
        self
    }
    #[cfg(feature = "test-support")]
    #[doc(hidden)]
    pub fn with_crash_for_testing(mut self, fault: DeferralFaultBoundaryV2) -> Self {
        self.fault = Some(fault);
        self.crash = true;
        self
    }
    #[cfg(feature = "test-support")]
    #[doc(hidden)]
    pub fn with_pause_for_testing(
        mut self,
        boundary: DeferralFaultBoundaryV2,
        ready: PathBuf,
        release: PathBuf,
    ) -> Self {
        self.pause = Some((boundary, ready, release));
        self
    }
    fn boundary(&self, boundary: DeferralFaultBoundaryV2) -> Result<(), GovernanceRefusalV1> {
        #[cfg(feature = "test-support")]
        {
            if let Some((at, ready, release)) = &self.pause {
                if *at == boundary {
                    std::fs::write(ready, b"ready")
                        .map_err(|_| refusal("pause_failed", "Pause rendezvous unavailable"))?;
                    let started = std::time::Instant::now();
                    while !release.exists() {
                        if started.elapsed() > std::time::Duration::from_secs(20) {
                            return Err(refusal("pause_timeout", "Pause release unavailable"));
                        }
                        std::thread::sleep(std::time::Duration::from_millis(10));
                    }
                }
            }
            if self.fault == Some(boundary) {
                if self.crash {
                    std::process::exit(71);
                }
                return Err(refusal(
                    "injected_interruption",
                    "Resolve the exact original locator",
                ));
            }
        }
        let _ = boundary;
        Ok(())
    }

    /// Nonmutating preparation. No guard/claim/root is created here.
    pub fn prepare(
        &self,
        root: impl AsRef<Path>,
        input: &DeferralRequestInputV2,
    ) -> Result<PreparedAdmissionDeferralV2, GovernanceRefusalV1> {
        validate_deferral_request(input.clone())?;
        let root = root.as_ref();
        let retained = RetainedRepositoryRootV1::open_exact(root).map_err(fs_refusal)?;
        let entries = retained
            .list_directory_bounded("", 4096)
            .map_err(fs_refusal)?;
        if entries.iter().any(|entry| {
            matches!(
                entry.name(),
                ".handbook"
                    | ".handbook-initialization.v1.json"
                    | ".handbook-initialization-receipt.v1.json"
                    | DEFERRAL_NONCOMMIT_PATH_V2
            )
        }) {
            return Err(refusal(
                "initial_registration_unavailable",
                "Existing roots or retained claims cannot produce a fresh creation capability",
            ));
        }
        let identity = RepositoryInvocationIdentityServiceV1::new()
            .allocate_prepared_repository_identity()
            .map_err(|_| {
                refusal(
                    "identity_entropy_unavailable",
                    "Proposed identity could not be allocated",
                )
            })?;
        let retained_root = root
            .to_str()
            .ok_or_else(|| refusal("invalid_root", "Root must be exact UTF-8"))?
            .to_owned();
        let physical = retained
            .governance_physical_root_fingerprint_v1()
            .map_err(fs_refusal)?;
        let project = ProjectBindingV2 {
            registration_identity: deferral_registration_fingerprint(
                identity.repository_identity_fingerprint(),
                &retained_root,
                &physical,
            )?,
            repository_identity_fingerprint: identity.repository_identity_fingerprint().into(),
            retained_root,
            physical_root_fingerprint: physical,
        };
        let request_fingerprint = deferral_request_fingerprint(&project, input)?;
        let initialization_claim = RepositoryInitializationClaimV1::prepare(
            root,
            &retained,
            DEFERRAL_NATIVE_OWNER_V2,
            request_fingerprint.as_str(),
            identity.repository_identity_fingerprint(),
        )
        .map_err(fs_refusal)?;
        let locator = DeferralRequestLocatorV2 {
            project,
            request: input.clone(),
            request_fingerprint,
            initialization_claim,
        };
        validate_deferral_locator(&locator)?;
        Ok(PreparedAdmissionDeferralV2 { locator, identity })
    }

    pub fn initialize(
        &self,
        root: impl AsRef<Path>,
        prepared: PreparedAdmissionDeferralV2,
    ) -> DeferralExecutionV2 {
        let PreparedAdmissionDeferralV2 { locator, identity } = prepared;
        let mut may_have_claim = false;
        let attempt = (|| {
            validate_deferral_locator(&locator)?;
            let _process = PROCESS_LOCK
                .lock()
                .map_err(|_| refusal("lock_unavailable", "Owner process lock unavailable"))?;
            let guard = exact_guard(root.as_ref(), &locator.project)?;
            guard
                .require_fresh()
                .map_err(|error| refusal("initial_registration_unavailable", error.detail()))?;
            may_have_claim = true;
            guard
                .retain_native_claim(&locator.initialization_claim)
                .map_err(fs_refusal)?;
            self.boundary(DeferralFaultBoundaryV2::Claim)?;
            let retained = guard.root();
            guard.validate().map_err(fs_refusal)?;
            retained
                .create_directory_new_durable(".handbook")
                .map_err(fs_refusal)?;
            self.boundary(DeferralFaultBoundaryV2::Reservation)?;
            guard.validate().map_err(fs_refusal)?;
            let native = retained
                .acquire_exclusive_lock(super::LOCK)
                .map_err(fs_refusal)?;
            native
                .validate_at(retained, super::LOCK)
                .map_err(fs_refusal)?;
            let anchor = AnchorV2 {
                locator: locator.clone(),
                governance_lock_fingerprint: native
                    .physical_fingerprint_v1()
                    .map_err(fs_refusal)?,
            };
            retained
                .write_new_durable(
                    GOVERNANCE_ANCHOR_PATH_V2,
                    &governance_canonical_bytes(&anchor)?,
                )
                .map_err(fs_refusal)?;
            self.boundary(DeferralFaultBoundaryV2::Anchor)?;
            RepositoryInvocationIdentityServiceV1::new()
                .persist_prepared_repository_identity_locked(&guard, identity)
                .map_err(fs_refusal)?;
            self.boundary(DeferralFaultBoundaryV2::Identity)?;
            let facts = Facts::new(&locator)?;
            guard.validate().map_err(fs_refusal)?;
            native
                .validate_at(retained, super::LOCK)
                .map_err(fs_refusal)?;
            retained
                .create_directories(GOVERNANCE_STORAGE_PATH_V2)
                .map_err(fs_refusal)?;
            retained
                .write_new_durable(&storage("registration.json"), &facts.registration)
                .map_err(fs_refusal)?;
            self.boundary(DeferralFaultBoundaryV2::Registration)?;
            guard.validate().map_err(fs_refusal)?;
            native
                .validate_at(retained, super::LOCK)
                .map_err(fs_refusal)?;
            retained
                .create_directories(&request_path(&locator, ""))
                .map_err(fs_refusal)?;
            for (i, bytes) in facts.staged.iter().enumerate() {
                guard.validate().map_err(fs_refusal)?;
                native
                    .validate_at(retained, super::LOCK)
                    .map_err(fs_refusal)?;
                write(retained, &locator, &format!("stage-{i}.json"), bytes)?;
                self.boundary(DeferralFaultBoundaryV2::Stage(i))?;
            }
            guard.validate().map_err(fs_refusal)?;
            native
                .validate_at(retained, super::LOCK)
                .map_err(fs_refusal)?;
            write(retained, &locator, "intent.json", &facts.intent)?;
            self.boundary(DeferralFaultBoundaryV2::Intent)?;
            let state = inspect(retained, &native, &locator, &facts, false)?;
            if !state.intent
                || state.decision
                || state.installed != 0
                || state.live
                || state.terminal
                || state.noncommit
            {
                return Err(refusal(
                    "initial_basis_consumed",
                    "Decision requires the exact positive unused initial choice",
                ));
            }
            guard.validate().map_err(fs_refusal)?;
            native
                .validate_at(retained, super::LOCK)
                .map_err(fs_refusal)?;
            guard
                .observe_native_claim(&locator.initialization_claim)
                .map_err(fs_refusal)?;
            write(retained, &locator, "decision.json", &facts.decision)?;
            self.boundary(DeferralFaultBoundaryV2::Decision)?;
            self.finish(&guard, &native, &locator, &facts, &state)?;
            inspect(retained, &native, &locator, &facts, true)?;
            self.boundary(DeferralFaultBoundaryV2::Reply)?;
            native.release().map_err(fs_refusal)?;
            guard.release().map_err(fs_refusal)?;
            Ok(DeferralExecutionV2::Committed {
                outcome: facts.outcome,
                delivery: GovernanceDeliveryV1::Original,
            })
        })();
        match attempt {
            Ok(result) => result,
            Err(refusal) if !may_have_claim => DeferralExecutionV2::Refused { refusal },
            Err(error) => DeferralExecutionV2::Uncertain {
                locator,
                condition: format!("{}: {}", error.condition, error.detail),
            },
        }
    }

    /// Resolves original history. A terminal replay never installs/restores current.
    pub fn resolve_request(
        &self,
        root: impl AsRef<Path>,
        locator: &DeferralRequestLocatorV2,
    ) -> DeferralExecutionV2 {
        if let Err(refusal) = validate_deferral_locator(locator) {
            return DeferralExecutionV2::Refused { refusal };
        }
        let attempt = (|| {
            let _process = PROCESS_LOCK
                .lock()
                .map_err(|_| refusal("lock_unavailable", "Owner process lock unavailable"))?;
            let guard = exact_guard(root.as_ref(), &locator.project)?;
            let observed = match guard.observe_native_claim(&locator.initialization_claim) {
                Ok(observed) => observed,
                Err(error) if error.kind() == handbook_engine::RepositoryDurabilityErrorKindV1::CurrentValueMismatch => return Ok(DeferralExecutionV2::Refused { refusal: refusal("request_conflict", "Retained initialization claim binds different complete original content").with_project(locator.project.refusal_project()) }),
                Err(error) => return Err(fs_refusal(error)),
            };
            if !observed {
                return Err(refusal(
                    "claim_missing",
                    "No retained original initialization claim",
                ));
            }
            let facts = Facts::new(locator)?;
            let root = guard.root();
            let early = inspect_early(root, locator)?;
            let native = if early {
                None
            } else {
                super::require_existing_lock(root)?;
                Some(
                    root.acquire_exclusive_lock(super::LOCK)
                        .map_err(fs_refusal)?,
                )
            };
            if early {
                close_noncommit(self, &guard, None, locator, &facts)?;
                guard.release().map_err(fs_refusal)?;
                return Ok(DeferralExecutionV2::NotCommitted {
                    outcome: facts.noncommit,
                    delivery: GovernanceDeliveryV1::Replay,
                });
            }
            let original = read_anchor(root, native.as_ref().unwrap())?.locator;
            if original != *locator {
                return Ok(DeferralExecutionV2::Refused {
                    refusal: refusal(
                        "request_conflict",
                        "Anchor binds different exact original content",
                    )
                    .with_project(original.project.refusal_project()),
                });
            }
            let state = inspect_records(
                root,
                Some(native.as_ref().unwrap()),
                locator,
                &facts,
                false,
                true,
            )?;
            native
                .as_ref()
                .unwrap()
                .validate_at(root, super::LOCK)
                .map_err(fs_refusal)?;
            let outcome = if state.terminal {
                super::transition::validate_original_replay(root, &facts.basis)?;
                guard
                    .complete_native_claim(
                        &locator.initialization_claim,
                        RepositoryInitializationDispositionV1::Committed,
                        &request_path(locator, "terminal.json"),
                        &facts.terminal,
                    )
                    .map_err(fs_refusal)?;
                DeferralExecutionV2::Committed {
                    outcome: facts.outcome,
                    delivery: GovernanceDeliveryV1::Replay,
                }
            } else if state.decision {
                self.finish(&guard, native.as_ref().unwrap(), locator, &facts, &state)?;
                inspect(root, native.as_ref().unwrap(), locator, &facts, true)?;
                DeferralExecutionV2::Committed {
                    outcome: facts.outcome,
                    delivery: GovernanceDeliveryV1::Replay,
                }
            } else {
                close_noncommit(self, &guard, native.as_ref(), locator, &facts)?;
                DeferralExecutionV2::NotCommitted {
                    outcome: facts.noncommit,
                    delivery: GovernanceDeliveryV1::Replay,
                }
            };
            if let Some(native) = native {
                native.release().map_err(fs_refusal)?;
            }
            guard.release().map_err(fs_refusal)?;
            Ok(outcome)
        })();
        match attempt {
            Ok(result) => result,
            Err(error) => DeferralExecutionV2::Uncertain {
                locator: locator.clone(),
                condition: error.condition,
            },
        }
    }

    /// Positive current deferral is distinct from delivering an original request.
    pub fn resolve_current(
        &self,
        root: impl AsRef<Path>,
        project: &ProjectBindingV2,
        scope: &str,
        expected: Option<&ExpectedDeferredBasisV2>,
    ) -> Result<DeferredGovernanceBasisV2, GovernanceRefusalV1> {
        if scope != super::ADMISSION_GOVERNANCE_SCOPE_V1 {
            return Err(refusal(
                "unsupported_scope",
                "Only the exact Contracts freshness root scope is supported",
            ));
        }
        let _process = PROCESS_LOCK
            .lock()
            .map_err(|_| refusal("lock_unavailable", "Owner process lock unavailable"))?;
        let guard = exact_guard(root.as_ref(), project)?;
        let root = guard.root();
        super::require_existing_lock(root)?;
        let native = root
            .acquire_exclusive_lock(super::LOCK)
            .map_err(fs_refusal)?;
        let locator = read_anchor(root, &native)?.locator;
        validate_deferral_locator(&locator)?;
        if locator.project != *project {
            return Err(refusal(
                "project_substituted",
                "Anchor binds a different exact project",
            ));
        }
        if !guard
            .observe_native_claim(&locator.initialization_claim)
            .map_err(fs_refusal)?
        {
            return Err(refusal("claim_missing", "Original claim unavailable"));
        }
        let facts = Facts::new(&locator)?;
        let state = inspect(root, &native, &locator, &facts, false)?;
        native.validate_at(root, super::LOCK).map_err(fs_refusal)?;
        if !state.terminal {
            if !state.decision {
                return Err(refusal(
                    "governance_not_committed",
                    "Initial choice has no committed deferral",
                ));
            }
            self.finish(&guard, &native, &locator, &facts, &state)?;
        }
        guard.validate().map_err(fs_refusal)?;
        native.validate_at(root, super::LOCK).map_err(fs_refusal)?;
        inspect(root, &native, &locator, &facts, true)?;
        guard
            .complete_native_claim(
                &locator.initialization_claim,
                RepositoryInitializationDispositionV1::Committed,
                &request_path(&locator, "terminal.json"),
                &facts.terminal,
            )
            .map_err(fs_refusal)?;
        if expected.is_some_and(|expected| {
            expected.outcome_fingerprint != facts.basis.outcome_fingerprint
                || expected.relation_fingerprint != facts.basis.relation_fingerprint
        }) {
            return Err(refusal(
                "basis_conflict",
                "Expected exact current outcome/relation differs",
            ));
        }
        native.release().map_err(fs_refusal)?;
        guard.release().map_err(fs_refusal)?;
        Ok(facts.basis)
    }

    fn finish(
        &self,
        guard: &RepositoryInitializationGuardV1,
        native: &handbook_engine::RepositoryExclusiveLockV1,
        locator: &DeferralRequestLocatorV2,
        facts: &Facts,
        state: &State,
    ) -> Result<(), GovernanceRefusalV1> {
        let root = guard.root();
        for (i, target) in TARGETS.iter().enumerate().skip(state.installed) {
            guard.validate().map_err(fs_refusal)?;
            native.validate_at(root, super::LOCK).map_err(fs_refusal)?;
            write(root, locator, target, &facts.staged[i])?;
            self.boundary(DeferralFaultBoundaryV2::Installation(i))?;
        }
        guard.validate().map_err(fs_refusal)?;
        native.validate_at(root, super::LOCK).map_err(fs_refusal)?;
        if !state.live {
            root.write_new_durable(GOVERNANCE_CURRENT_PATH_V2, &facts.staged[5])
                .map_err(fs_refusal)?;
            self.boundary(DeferralFaultBoundaryV2::CurrentRelation)?;
        }
        guard.validate().map_err(fs_refusal)?;
        native.validate_at(root, super::LOCK).map_err(fs_refusal)?;
        if !state.terminal {
            write(root, locator, "terminal.json", &facts.terminal)?;
            self.boundary(DeferralFaultBoundaryV2::Terminal)?;
        }
        guard
            .complete_native_claim(
                &locator.initialization_claim,
                RepositoryInitializationDispositionV1::Committed,
                &request_path(locator, "terminal.json"),
                &facts.terminal,
            )
            .map_err(fs_refusal)?;
        self.boundary(DeferralFaultBoundaryV2::ClaimReceipt)
    }
}

struct Facts {
    registration: Vec<u8>,
    staged: Vec<Vec<u8>>,
    intent: Vec<u8>,
    decision: Vec<u8>,
    terminal: Vec<u8>,
    noncommit_bytes: Vec<u8>,
    outcome: DeferralCommittedOutcomeV2,
    noncommit: DeferralNotCommittedOutcomeV2,
    basis: DeferredGovernanceBasisV2,
}
impl Facts {
    fn new(locator: &DeferralRequestLocatorV2) -> Result<Self, GovernanceRefusalV1> {
        let validated = validate_deferral_locator(locator)?;
        let deferral_fingerprint = governance_fingerprint(
            "handbook.admission-deferral@2.0.0",
            &(
                locator,
                &validated,
                "never_established",
                "initial_choice_consumed",
            ),
        )?;
        let outcome_fingerprint = governance_fingerprint(
            "handbook.admission-governance-outcome@2.0.0",
            &(
                locator,
                &validated,
                &deferral_fingerprint,
                "validated",
                "advisory_permission_accepted",
                "deferral_committed",
            ),
        )?;
        let outcome = DeferralCommittedOutcomeV2 {
            locator: locator.clone(),
            validated: validated.clone(),
            validation_confirmed: true,
            advisory_permission_accepted: true,
            deferral_committed: true,
            outcome_fingerprint: outcome_fingerprint.clone(),
        };
        let relation_fingerprint = governance_fingerprint(
            "handbook.admission-governance-current-relation@2.0.0",
            &(
                &locator.project,
                &locator.request.scope,
                &deferral_fingerprint,
                &outcome_fingerprint,
                "deferred_never_established",
            ),
        )?;
        let basis = DeferredGovernanceBasisV2 {
            project: locator.project.clone(),
            scope: locator.request.scope.clone(),
            never_established: true,
            registration_identity: locator.project.registration_identity.clone(),
            deferral_fingerprint: deferral_fingerprint.clone(),
            outcome_fingerprint: outcome_fingerprint.clone(),
            relation_fingerprint,
            establishment_rule: validated.establishment_rule.clone(),
        };
        let registration = governance_canonical_bytes(&(
            "handbook.admission-registration.v2",
            &locator.project,
            "stable_fresh_project_registration",
        ))?;
        let staged = vec![
            governance_canonical_bytes(&validated.establishment_rule)?,
            governance_canonical_bytes(&(
                &locator.request.instruction,
                &validated.instruction_fingerprint,
            ))?,
            governance_canonical_bytes(&(
                "handbook.admission-deferral.v2",
                locator,
                &deferral_fingerprint,
                "never_established",
            ))?,
            governance_canonical_bytes(&outcome)?,
            governance_canonical_bytes(&(
                "consumed_initial_choice",
                &locator.project.registration_identity,
                &locator.request_fingerprint,
                &outcome_fingerprint,
            ))?,
            governance_canonical_bytes(&basis)?,
        ];
        let intent = governance_canonical_bytes(&(
            "handbook.admission-intent.v2",
            &locator.request_fingerprint,
            governance_fingerprint("handbook.admission-registration-bytes@2.0.0", &registration)?,
            staged
                .iter()
                .map(|bytes| governance_fingerprint("handbook.admission-stage-bytes@2.0.0", bytes))
                .collect::<Result<Vec<_>, _>>()?,
        ))?;
        let decision = governance_canonical_bytes(&(
            "handbook.admission-commit-decision.v2",
            governance_fingerprint("handbook.admission-intent-bytes@2.0.0", &intent)?,
        ))?;
        let terminal = governance_canonical_bytes(&(
            "handbook.admission-terminal.v2",
            &outcome_fingerprint,
            governance_fingerprint("handbook.admission-decision-bytes@2.0.0", &decision)?,
            "exact_initial_current_relation_published",
        ))?;
        let condition = "verified_predecision_noncommit".to_owned();
        let noncommit = DeferralNotCommittedOutcomeV2 {
            locator: locator.clone(),
            condition: condition.clone(),
            deferral_committed: false,
            outcome_fingerprint: governance_fingerprint(
                "handbook.admission-noncommit@2.0.0",
                &(locator, &condition),
            )?,
        };
        let noncommit_bytes = governance_canonical_bytes(&noncommit)?;
        Ok(Self {
            registration,
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

#[derive(Default)]
struct State {
    intent: bool,
    decision: bool,
    installed: usize,
    live: bool,
    terminal: bool,
    noncommit: bool,
}

pub(super) fn exact_guard(
    path: &Path,
    project: &ProjectBindingV2,
) -> Result<RepositoryInitializationGuardV1, GovernanceRefusalV1> {
    if path.to_str() != Some(&project.retained_root) {
        return Err(refusal(
            "root_substituted",
            "Locator names a different retained root",
        ));
    }
    let retained = RetainedRepositoryRootV1::open_exact(path).map_err(fs_refusal)?;
    if retained
        .governance_physical_root_fingerprint_v1()
        .map_err(fs_refusal)?
        != project.physical_root_fingerprint
    {
        return Err(refusal(
            "physical_root_substituted",
            "Physical root differs before guard acquisition",
        ));
    }
    let guard = RepositoryInitializationGuardV1::acquire(path).map_err(fs_refusal)?;
    if guard
        .root()
        .governance_physical_root_fingerprint_v1()
        .map_err(fs_refusal)?
        != project.physical_root_fingerprint
    {
        return Err(refusal(
            "physical_root_substituted",
            "Physical root differs from original request",
        ));
    }
    Ok(guard)
}
fn storage(name: &str) -> String {
    format!("{GOVERNANCE_STORAGE_PATH_V2}/{name}")
}
fn read_anchor(
    root: &RetainedRepositoryRootV1,
    native: &handbook_engine::RepositoryExclusiveLockV1,
) -> Result<AnchorV2, GovernanceRefusalV1> {
    native.validate_at(root, super::LOCK).map_err(fs_refusal)?;
    let anchor = read_anchor_record(root)?;
    if anchor.governance_lock_fingerprint != native.physical_fingerprint_v1().map_err(fs_refusal)? {
        return Err(refusal(
            "governance_lock_substituted",
            "Original physical governance lock binding differs",
        ));
    }
    Ok(anchor)
}
fn read_anchor_record(root: &RetainedRepositoryRootV1) -> Result<AnchorV2, GovernanceRefusalV1> {
    let bytes = super::read(root, GOVERNANCE_ANCHOR_PATH_V2)?
        .ok_or_else(|| refusal("anchor_missing", "Original native anchor unavailable"))?;
    let anchor: AnchorV2 = decode_governance_json(&bytes)?;
    validate_deferral_locator(&anchor.locator)?;
    if bytes != governance_canonical_bytes(&anchor)? {
        return Err(refusal(
            "governance_lock_substituted",
            "Original canonical anchor and physical governance lock binding must agree",
        ));
    }
    Ok(anchor)
}
pub fn deferral_request_storage_path_v2(locator: &DeferralRequestLocatorV2) -> String {
    format!(
        "{GOVERNANCE_STORAGE_PATH_V2}/requests/{}",
        locator
            .request_fingerprint
            .as_str()
            .strip_prefix("sha256:")
            .unwrap_or("invalid")
    )
}
fn request_path(locator: &DeferralRequestLocatorV2, name: &str) -> String {
    if name.is_empty() {
        deferral_request_storage_path_v2(locator)
    } else {
        format!("{}/{name}", deferral_request_storage_path_v2(locator))
    }
}
fn write(
    root: &RetainedRepositoryRootV1,
    locator: &DeferralRequestLocatorV2,
    name: &str,
    bytes: &[u8],
) -> Result<(), GovernanceRefusalV1> {
    root.write_new_durable(&request_path(locator, name), bytes)
        .map_err(fs_refusal)
}
fn close_noncommit(
    owner: &ProjectAdmissionDeferralV2,
    guard: &RepositoryInitializationGuardV1,
    native: Option<&handbook_engine::RepositoryExclusiveLockV1>,
    locator: &DeferralRequestLocatorV2,
    facts: &Facts,
) -> Result<(), GovernanceRefusalV1> {
    guard.validate().map_err(fs_refusal)?;
    let root = guard.root();
    if let Some(native) = native {
        native.validate_at(root, super::LOCK).map_err(fs_refusal)?;
    }
    match super::read(root, DEFERRAL_NONCOMMIT_PATH_V2)? {
        Some(bytes) if bytes != facts.noncommit_bytes => {
            return Err(refusal(
                "noncommit_conflict",
                "Exact original noncommit witness differs",
            ))
        }
        Some(_) => {}
        None => root
            .write_new_durable(DEFERRAL_NONCOMMIT_PATH_V2, &facts.noncommit_bytes)
            .map_err(fs_refusal)?,
    }
    owner.boundary(DeferralFaultBoundaryV2::NonCommitWitness)?;
    guard
        .complete_native_claim(
            &locator.initialization_claim,
            RepositoryInitializationDispositionV1::NotCommitted,
            DEFERRAL_NONCOMMIT_PATH_V2,
            &facts.noncommit_bytes,
        )
        .map_err(fs_refusal)?;
    owner.boundary(DeferralFaultBoundaryV2::NonCommitReceipt)
}

/// Before the anchor exists only claim -> optional empty root -> optional lock is legal.
fn inspect_early(
    root: &RetainedRepositoryRootV1,
    locator: &DeferralRequestLocatorV2,
) -> Result<bool, GovernanceRefusalV1> {
    let entries = root.list_directory_bounded("", 4096).map_err(fs_refusal)?;
    let Some(handbook) = entries.iter().find(|entry| entry.name() == ".handbook") else {
        return Ok(true);
    };
    if handbook.kind() != RepositoryDirectoryEntryKindV1::Directory {
        return Err(refusal(
            "unsafe_inventory",
            "Reserved root is not a directory",
        ));
    }
    let entries = root
        .list_directory_bounded(".handbook", 4096)
        .map_err(fs_refusal)?;
    if entries
        .iter()
        .any(|entry| entry.name() == "admission-governance-project.v2.json")
    {
        return Ok(false);
    }
    for entry in entries {
        if entry.name() != "admission-governance.lock"
            || entry.kind() != RepositoryDirectoryEntryKindV1::RegularFile
        {
            return Err(refusal(
                "anchor_missing_nonprefix",
                "An absent anchor has later or unsupported root evidence",
            ));
        }
        super::require(root, super::LOCK, b"")?;
    }
    let _ = locator;
    Ok(true)
}

fn inspect(
    root: &RetainedRepositoryRootV1,
    native: &handbook_engine::RepositoryExclusiveLockV1,
    locator: &DeferralRequestLocatorV2,
    facts: &Facts,
    require_current: bool,
) -> Result<State, GovernanceRefusalV1> {
    inspect_records(root, Some(native), locator, facts, require_current, false)
}
fn inspect_records(
    root: &RetainedRepositoryRootV1,
    native: Option<&handbook_engine::RepositoryExclusiveLockV1>,
    locator: &DeferralRequestLocatorV2,
    facts: &Facts,
    require_current: bool,
    supports_v3: bool,
) -> Result<State, GovernanceRefusalV1> {
    let anchor = match native {
        Some(native) => read_anchor(root, native)?,
        None => read_anchor_record(root)?,
    };
    if anchor.locator != *locator {
        return Err(refusal(
            "anchor_conflict",
            "Original anchor binds different exact request content",
        ));
    }
    super::require_existing_lock(root)?;
    let identity = super::read(root, REPOSITORY_IDENTITY_REPO_PATH)?;
    let temporary = super::read(root, super::IDENTITY_TEMP)?;
    for bytes in [identity.as_ref(), temporary.as_ref()]
        .into_iter()
        .flatten()
    {
        if bytes != locator.project.repository_identity_fingerprint.as_bytes() {
            return Err(refusal(
                "identity_substituted",
                "Retained identity differs from original claim",
            ));
        }
    }
    if identity.is_some() && temporary.is_some() {
        return Err(refusal(
            "identity_nonprefix",
            "Identity publication has conflicting evidence",
        ));
    }
    let handbook = root
        .list_directory_bounded(".handbook", 4096)
        .map_err(fs_refusal)?;
    if handbook.iter().any(|entry| {
        entry.name().starts_with("admission-governance-project.")
            && entry.name() != "admission-governance-project.v2.json"
    }) {
        return Err(refusal(
            "unknown_inventory",
            "Unsupported governance anchor version",
        ));
    }
    let owner = handbook
        .iter()
        .find(|entry| entry.name() == "admission-governance");
    let mut root_names = Vec::new();
    if let Some(owner) = owner {
        if owner.kind() != RepositoryDirectoryEntryKindV1::Directory {
            return Err(refusal(
                "unsafe_inventory",
                "Owner storage is not a directory",
            ));
        }
        // Identity publication precedes every owner storage directory. An exact
        // temporary identity alone cannot prove this later causal prefix.
        if identity.is_none() {
            return Err(refusal(
                "owner_storage_without_identity",
                "Owner storage requires the exact installed repository identity",
            ));
        }
        let versions = root
            .list_directory_bounded(".handbook/admission-governance", 1)
            .map_err(fs_refusal)?;
        if versions.iter().any(|entry| {
            entry.name() != "v2" || entry.kind() != RepositoryDirectoryEntryKindV1::Directory
        }) {
            return Err(refusal(
                "unknown_inventory",
                "Unsupported owner inventory version",
            ));
        }
        if !versions.is_empty() {
            root_names = root
                .list_directory_bounded(GOVERNANCE_STORAGE_PATH_V2, if supports_v3 { 4 } else { 3 })
                .map_err(fs_refusal)?;
        }
    }
    for entry in &root_names {
        let expected_kind =
            if entry.name() == "requests" || (supports_v3 && entry.name() == "establishment-v3") {
                RepositoryDirectoryEntryKindV1::Directory
            } else {
                RepositoryDirectoryEntryKindV1::RegularFile
            };
        if !(matches!(
            entry.name(),
            "registration.json" | "requests" | "scope-current.json"
        ) || (supports_v3 && entry.name() == "establishment-v3"))
            || entry.kind() != expected_kind
        {
            return Err(refusal(
                "unknown_inventory",
                "Unsupported transition or unsafe v2 root inventory",
            ));
        }
    }
    let registration = root_names
        .iter()
        .any(|entry| entry.name() == "registration.json");
    if registration {
        super::require(root, &storage("registration.json"), &facts.registration)?;
        if identity.is_none() {
            return Err(refusal(
                "registration_without_identity",
                "Stable registration requires exact identity",
            ));
        }
    }
    let mut names = Vec::new();
    if root_names.iter().any(|entry| entry.name() == "requests") {
        if !registration {
            return Err(refusal(
                "history_without_registration",
                "Request history requires stable registration",
            ));
        }
        let requests = root
            .list_directory_bounded(&storage("requests"), 4096)
            .map_err(fs_refusal)?;
        let name = locator
            .request_fingerprint
            .as_str()
            .strip_prefix("sha256:")
            .unwrap_or("invalid");
        let transitions = if supports_v3 {
            super::transition::selected_request_names(root)?
        } else {
            vec![]
        };
        if requests.iter().any(|entry| {
            (entry.name() != name && !transitions.iter().any(|selected| selected == entry.name()))
                || entry.kind() != RepositoryDirectoryEntryKindV1::Directory
        }) {
            return Err(refusal(
                "unsupported_transition",
                "Only the one original request transition is supported",
            ));
        }
        if requests.iter().any(|entry| entry.name() == name) {
            names = root
                .list_directory_bounded(&request_path(locator, ""), 32)
                .map_err(fs_refusal)?;
        }
    }
    let allowed: Vec<String> = ["intent.json", "decision.json", "terminal.json"]
        .into_iter()
        .chain(TARGETS)
        .map(str::to_owned)
        .chain((0..6).map(|i| format!("stage-{i}.json")))
        .collect();
    for entry in &names {
        if entry.kind() != RepositoryDirectoryEntryKindV1::RegularFile
            || !allowed.iter().any(|name| name == entry.name())
        {
            return Err(refusal(
                "unknown_inventory",
                "Unsupported or unsafe request history",
            ));
        }
    }
    let check = |name: &str, bytes: &[u8]| -> Result<bool, GovernanceRefusalV1> {
        if names.iter().any(|entry| entry.name() == name) {
            super::require(root, &request_path(locator, name), bytes)?;
            Ok(true)
        } else {
            Ok(false)
        }
    };
    let mut staged = 0;
    for (i, bytes) in facts.staged.iter().enumerate() {
        if check(&format!("stage-{i}.json"), bytes)? {
            if !registration || i != staged {
                return Err(refusal(
                    "stage_nonprefix",
                    "Staging lacks complete causal prefix",
                ));
            }
            staged += 1;
        }
    }
    let intent = check("intent.json", &facts.intent)?;
    if intent && staged != 6 {
        return Err(refusal(
            "intent_without_staging",
            "Intent requires all exact staged facts",
        ));
    }
    let decision = check("decision.json", &facts.decision)?;
    if decision && !intent {
        return Err(refusal(
            "decision_without_intent",
            "Decision lacks exact intent",
        ));
    }
    let mut installed = 0;
    for (i, target) in TARGETS.iter().enumerate() {
        if check(target, &facts.staged[i])? {
            if !decision || installed != i {
                return Err(refusal(
                    "installation_nonprefix",
                    "Installed immutable effects lack an exact decision/prefix",
                ));
            }
            installed += 1;
        }
    }
    let terminal = check("terminal.json", &facts.terminal)?;
    let live = root_names
        .iter()
        .any(|entry| entry.name() == "scope-current.json");
    if terminal && installed != 6 {
        return Err(refusal(
            "terminal_incomplete",
            "Terminal does not close immutable history",
        ));
    }
    // Terminal historical replay deliberately does not use mutable current content.
    if !terminal || require_current {
        if live {
            super::require(root, GOVERNANCE_CURRENT_PATH_V2, &facts.staged[5])?;
            if installed != 6 || !decision {
                return Err(refusal(
                    "current_nonprefix",
                    "Current relation lacks exact installed history",
                ));
            }
        }
        if terminal && !live {
            return Err(refusal(
                "current_missing",
                "Current relation is unavailable",
            ));
        }
    }
    let noncommit = super::read(root, DEFERRAL_NONCOMMIT_PATH_V2)?;
    if let Some(bytes) = &noncommit {
        if *bytes != facts.noncommit_bytes || decision || installed != 0 || live || terminal {
            return Err(refusal(
                "terminal_conflict",
                "Noncommit witness conflicts with retained history",
            ));
        }
    }
    Ok(State {
        intent,
        decision,
        installed,
        live,
        terminal,
        noncommit: noncommit.is_some(),
    })
}

/// Record-only preparation view; execution supplies the retained native lock and
/// separately requires the exact Engine terminal receipt. No lock is created here.
pub(super) struct OriginalDeferredSnapshot {
    pub locator: DeferralRequestLocatorV2,
    pub basis: DeferredGovernanceBasisV2,
    pub current: Vec<u8>,
    pub terminal: Vec<u8>,
    pub terminal_path: String,
}
pub(super) fn original_deferred_snapshot(
    root: &RetainedRepositoryRootV1,
    project: &ProjectBindingV2,
    native: Option<&handbook_engine::RepositoryExclusiveLockV1>,
) -> Result<OriginalDeferredSnapshot, GovernanceRefusalV1> {
    let anchor = match native {
        Some(native) => read_anchor(root, native)?,
        None => read_anchor_record(root)?,
    };
    if anchor.locator.project != *project {
        return Err(refusal(
            "project_substituted",
            "Original registration differs",
        ));
    }
    let facts = Facts::new(&anchor.locator)?;
    let state = inspect_records(root, native, &anchor.locator, &facts, false, true)?;
    if !state.terminal || !state.decision || state.noncommit {
        return Err(refusal(
            "deferred_predecessor_unavailable",
            "Original D must be positively committed",
        ));
    }
    // Native locks are inspected by metadata: reading a held Windows byte-range
    // lock is not a supported way to establish its presence/generation.
    if !root
        .list_directory_bounded("", 4096)
        .map_err(fs_refusal)?
        .iter()
        .any(|entry| {
            entry.name() == handbook_engine::REPOSITORY_INITIALIZATION_LOCK_PATH_V1
                && entry.kind() == RepositoryDirectoryEntryKindV1::RegularFile
        })
    {
        return Err(refusal(
            "original_lock_unavailable",
            "Original root guard is absent",
        ));
    }
    for path in [
        handbook_engine::REPOSITORY_INITIALIZATION_CLAIM_PATH_V1,
        handbook_engine::REPOSITORY_INITIALIZATION_RECEIPT_PATH_V1,
    ] {
        if super::read(root, path)?.is_none() {
            return Err(refusal(
                "original_claim_unavailable",
                "Original claim, receipt and root guard must remain retained",
            ));
        }
    }
    let terminal_path = request_path(&anchor.locator, "terminal.json");
    Ok(OriginalDeferredSnapshot {
        locator: anchor.locator,
        basis: facts.basis,
        current: facts.staged[5].clone(),
        terminal: facts.terminal,
        terminal_path,
    })
}
