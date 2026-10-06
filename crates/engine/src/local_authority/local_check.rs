//! Explicit local-check successor terms and lifetime-sealed owner actions.
//! Portable records carry history; they cannot recreate current permission.
use super::*;
use crate::repository_durability::{RepositoryDirectoryEntryKindV1, RetainedRepositoryTreeGuardV1};
use std::collections::VecDeque;

pub const LOCAL_CHECK_RULE_V1: &str = "handbook.operating.local-check@1.0.0";
pub const LOCAL_CHECK_ROOT_V1: &str = ".handbook/local-operating/local-check-v1";
const CURRENT: &str = ".handbook/local-operating/local-check-v1/current.json";
const REVOKE: &str = ".handbook/local-operating/local-check-v1/revoke.json";

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalCheckActionV1 {
    DefinitionAuthor,
    Review,
    Lifecycle,
    EvidenceSource,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalCheckTermsV1 {
    pub request_key: String,
    pub mode: String,
    pub rule_ref: String,
    pub instruction: String,
    pub principal: String,
    pub agent: String,
    pub delegation: String,
    pub approval_source_ref: String,
    pub human_approval_asserted: bool,
    pub approved_content_fingerprint: String,
    pub expected_predecessor_fingerprint: String,
    pub expected_charter_fingerprint: String,
    pub ceiling: LocalEnvelopeCeilingV1,
    pub constraint_inputs: Vec<LocalBindingV1>,
    pub allowed_paths: Vec<String>,
    pub allowed_input_paths: Vec<String>,
    pub action_classes: Vec<LocalCheckActionV1>,
    pub author_participant_ref: String,
    pub review_participant_refs: Vec<String>,
    pub reserved_decisions: Vec<LocalDecisionClassV1>,
    pub delegated_decisions: Vec<LocalDecisionClassV1>,
    pub revocation_principal: String,
}
impl LocalCheckTermsV1 {
    pub fn fingerprint_approved_content(&self) -> Result<String, LocalAuthorityRefusalV1> {
        let mut value = serde_json::to_value(self).map_err(codec)?;
        let object = value
            .as_object_mut()
            .ok_or_else(|| refuse("shape", "terms are not an object"))?;
        object.remove("human_approval_asserted");
        object.remove("approved_content_fingerprint");
        fingerprint("handbook.local-check-approved-content@1.0.0", &value)
    }
    pub fn canonical_json_bytes(&self) -> Result<Vec<u8>, LocalAuthorityRefusalV1> {
        self.validate()?;
        encode(self)
    }
    pub fn from_json_bytes(bytes: &[u8]) -> Result<Self, LocalAuthorityRefusalV1> {
        let result: Self = decode(bytes)?;
        result.validate()?;
        Ok(result)
    }
    fn validate(&self) -> Result<(), LocalAuthorityRefusalV1> {
        if self.rule_ref != LOCAL_CHECK_RULE_V1
            || self.mode != "local_advisory"
            || !self.human_approval_asserted
            || self.approved_content_fingerprint != self.fingerprint_approved_content()?
        {
            return Err(refuse(
                "successor_approval",
                "explicit human approval of exact compatible local-check terms required",
            ));
        }
        for value in [
            &self.expected_predecessor_fingerprint,
            &self.expected_charter_fingerprint,
        ] {
            check_fingerprint(value)?;
        }
        if self.action_classes.is_empty()
            || self.action_classes.len() > 4
            || duplicate(&self.action_classes)
            || self.review_participant_refs.is_empty()
            || self.review_participant_refs.len() > 16
            || self
                .review_participant_refs
                .windows(2)
                .any(|p| p[0] >= p[1])
            || self
                .review_participant_refs
                .contains(&self.author_participant_ref)
        {
            return Err(refuse(
                "participant_terms",
                "actual distinct stable author/reviewer participants and closed actions required",
            ));
        }
        for participant in self
            .review_participant_refs
            .iter()
            .chain([&self.author_participant_ref])
        {
            bounded_text(participant)?;
        }
        // Reuse the selected path/ceiling/provenance mechanics without creating
        // or reinterpreting any V1 owner record or identity.
        let mut base = LocalOperatingTermsV1 {
            request_key: self.request_key.clone(),
            mode: self.mode.clone(),
            rule_ref: LOCAL_OPERATING_RULE_V1.into(),
            instruction: self.instruction.clone(),
            principal: self.principal.clone(),
            agent: self.agent.clone(),
            delegation: self.delegation.clone(),
            approval_source_ref: self.approval_source_ref.clone(),
            human_approval_asserted: self.human_approval_asserted,
            approved_content_fingerprint: String::new(),
            ceiling: self.ceiling.clone(),
            constraint_inputs: self.constraint_inputs.clone(),
            allowed_paths: self.allowed_paths.clone(),
            allowed_input_paths: self.allowed_input_paths.clone(),
            operation_classes: if self.allowed_paths.is_empty() {
                vec![LocalOperationClassV1::LocalValidation]
            } else {
                vec![
                    LocalOperationClassV1::LocalValidation,
                    LocalOperationClassV1::OwnerWrite,
                ]
            },
            reserved_decisions: self.reserved_decisions.clone(),
            delegated_decisions: self.delegated_decisions.clone(),
            revocation_principal: self.revocation_principal.clone(),
        };
        base.approved_content_fingerprint = base.fingerprint_approved_content()?;
        base.validate()
    }
}

/// A byte-complete observation, independently checked by Engine. Only the
/// Contracts owner can seal its separate qualification outcome around this value.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalCheckCurrentBasisV1 {
    pub relation_fingerprint: String,
    pub governance_tree_fingerprint: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalCheckLocatorV1 {
    pub predecessor: LocalAuthorityLocatorV1,
    pub terms: LocalCheckTermsV1,
    pub adoption_basis: LocalCheckCurrentBasisV1,
    pub operating_lock_fingerprint: String,
    pub request_fingerprint: String,
}
impl LocalCheckLocatorV1 {
    pub fn canonical_json_bytes(&self) -> Result<Vec<u8>, LocalAuthorityRefusalV1> {
        self.validate()?;
        encode(self)
    }
    pub fn from_json_bytes(bytes: &[u8]) -> Result<Self, LocalAuthorityRefusalV1> {
        let value: Self = decode(bytes)?;
        value.validate()?;
        Ok(value)
    }
    fn validate(&self) -> Result<(), LocalAuthorityRefusalV1> {
        self.predecessor.validate()?;
        self.terms.validate()?;
        check_fingerprint(&self.adoption_basis.relation_fingerprint)?;
        check_fingerprint(&self.adoption_basis.governance_tree_fingerprint)?;
        check_fingerprint(&self.operating_lock_fingerprint)?;
        if self.terms.principal != self.predecessor.terms.principal
            || self.terms.agent != self.predecessor.terms.agent
            || self.terms.expected_charter_fingerprint
                != self.predecessor.charter.canonical_fingerprint
            || self.request_fingerprint
                != fingerprint(
                    "handbook.local-check-request@1.0.0",
                    &(
                        &self.predecessor,
                        &self.terms,
                        &self.adoption_basis,
                        &self.operating_lock_fingerprint,
                    ),
                )?
        {
            return Err(refuse(
                "successor_binding",
                "successor must bind original project, Charter, actors and approved exact lineage",
            ));
        }
        Ok(())
    }
}
#[derive(Debug)]
pub struct PreparedLocalCheckAuthorityV1 {
    locator: LocalCheckLocatorV1,
}
impl PreparedLocalCheckAuthorityV1 {
    pub fn locator(&self) -> &LocalCheckLocatorV1 {
        &self.locator
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalCheckAuthorityOutcomeV1 {
    pub schema_ref: String,
    pub locator: LocalCheckLocatorV1,
    pub outcome_fingerprint: String,
    pub assurance: String,
}
#[derive(Debug, Serialize)]
pub struct LocalCheckAuthorityExecutionV1 {
    pub disposition: LocalAuthorityDispositionV1,
    pub outcome: Option<LocalCheckAuthorityOutcomeV1>,
    pub refusal: Option<LocalAuthorityRefusalV1>,
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct CheckHead {
    schema_ref: String,
    selected: String,
    revoked: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalCheckActionRequestV1 {
    pub action: LocalCheckActionV1,
    pub action_ref: String,
    pub subject_ref: String,
    pub content_fingerprint: String,
    pub closure_fingerprint: String,
    pub actor_ref: String,
    pub participant_ref: String,
    pub semantic_time_utc: String,
    /// Literal owner directory roots; approved selectors must cover each full
    /// subtree, including the owner's transaction and output files.
    pub owner_write_paths: Vec<String>,
    pub input_paths: Vec<String>,
}

/// Cannot be serialized, cloned, or constructed by a consumer. Its borrowed
/// native guards remain live through the owner mutation/source callback.
pub struct LocalCheckAdmissionV1<'authority> {
    locator: &'authority LocalCheckLocatorV1,
    basis: &'authority LocalCheckCurrentBasisV1,
    request: &'authority LocalCheckActionRequestV1,
    guard: &'authority OperatingGuard,
    tree: &'authority RetainedRepositoryTreeGuardV1<'authority>,
    authority_fingerprint: String,
    observed_at_utc: String,
}
impl LocalCheckAdmissionV1<'_> {
    pub fn request(&self) -> &LocalCheckActionRequestV1 {
        self.request
    }
    pub fn terms(&self) -> &LocalCheckTermsV1 {
        &self.locator.terms
    }
    pub fn locator(&self) -> &LocalCheckLocatorV1 {
        self.locator
    }
    pub fn current_basis(&self) -> &LocalCheckCurrentBasisV1 {
        self.basis
    }
    pub fn authority_fingerprint(&self) -> &str {
        &self.authority_fingerprint
    }
    pub fn observed_at_utc(&self) -> &str {
        &self.observed_at_utc
    }
    pub fn require_repository_root(
        &self,
        root: &RetainedRepositoryRootV1,
    ) -> Result<(), LocalAuthorityRefusalV1> {
        if root
            .governance_physical_root_fingerprint_v1()
            .map_err(fs_error)?
            != self
                .locator
                .predecessor
                .charter
                .binding
                .initialization_claim
                .physical_root_fingerprint()
        {
            return Err(refuse(
                "wrong_root",
                "admission is bound to its actual original physical root",
            ));
        }
        self.validate_current()
    }
    pub fn validate_current(&self) -> Result<(), LocalAuthorityRefusalV1> {
        self.tree.validate().map_err(fs_error)?;
        self.guard.validate()?;
        check_current(self.guard.root(), self.locator, self.basis)?;
        Ok(())
    }
    /// Checks the actual consuming Resolution against the exact shipped stack
    /// and supplied local ceiling; it creates no reusable envelope permission.
    pub fn require_resolution_ceiling(
        &self,
        stack_ref: &str,
        stack_fingerprint: &str,
        dimensions: [&str; 6],
    ) -> Result<(), LocalAuthorityRefusalV1> {
        self.validate_current()?;
        let selected = profile()?;
        let stack = selected.context_resolution();
        if stack_ref != stack.exact_ref().to_string()
            || stack_fingerprint != stack.definition_fingerprint().to_string()
        {
            return Err(refuse(
                "resolution_ceiling",
                "actual Resolution must use the selected local stack",
            ));
        }
        let requested = stack
            .dimension_ranks(dimensions)
            .ok_or_else(|| refuse("resolution_ceiling", "actual Resolution does not resolve"))?;
        let ceiling = stack
            .ranks(
                &self.terms().ceiling.active_level_id,
                self.terms()
                    .ceiling
                    .dimensions
                    .each_ref()
                    .map(String::as_str),
            )
            .ok_or_else(|| refuse("resolution_ceiling", "selected ceiling does not resolve"))?;
        if requested
            .iter()
            .zip(ceiling)
            .any(|(actual, limit)| *actual > limit)
        {
            return Err(refuse(
                "resolution_ceiling",
                "actual Resolution exceeds the supplied local ceiling",
            ));
        }
        Ok(())
    }
}

pub struct NativeLocalCheckAuthorityV1;
impl NativeLocalCheckAuthorityV1 {
    pub fn observe_basis(
        repo: impl AsRef<Path>,
        relation_fingerprint: impl Into<String>,
    ) -> Result<LocalCheckCurrentBasisV1, LocalAuthorityRefusalV1> {
        let root = RetainedRepositoryRootV1::open_exact(repo.as_ref()).map_err(fs_error)?;
        if root
            .read_local_owner_optional_bounded(crate::REPOSITORY_INITIALIZATION_LOCK_PATH_V1, MAX)
            .map_err(fs_error)?
            != RepositoryReadV1::Found(Vec::new())
        {
            return Err(refuse(
                "required_lock_state_missing",
                "observation cannot recreate the original initialization lock",
            ));
        }
        let guard = RepositoryInitializationGuardV1::acquire_existing_v1(repo).map_err(fs_error)?;
        let root = guard.root();
        let tree = root
            .retain_local_check_authority_tree_guard()
            .map_err(fs_error)?;
        let basis = LocalCheckCurrentBasisV1 {
            relation_fingerprint: relation_fingerprint.into(),
            governance_tree_fingerprint: governance_tree_identity(root)?,
        };
        require_basis(root, &basis)?;
        tree.validate().map_err(fs_error)?;
        guard.validate().map_err(fs_error)?;
        Ok(basis)
    }
    pub fn prepare(
        repo: impl AsRef<Path>,
        predecessor: &LocalAuthorityLocatorV1,
        terms: LocalCheckTermsV1,
        adoption_basis: LocalCheckCurrentBasisV1,
    ) -> Result<PreparedLocalCheckAuthorityV1, LocalAuthorityRefusalV1> {
        let guard = OperatingGuard::acquire(repo.as_ref(), predecessor, false)?;
        let operating_lock_fingerprint = guard
            .root()
            .existing_lock_physical_fingerprint_v1(".handbook/state/locks/local-operating.lock")
            .map_err(fs_error)?;
        let locator = LocalCheckLocatorV1 {
            request_fingerprint: fingerprint(
                "handbook.local-check-request@1.0.0",
                &(
                    predecessor,
                    &terms,
                    &adoption_basis,
                    &operating_lock_fingerprint,
                ),
            )?,
            predecessor: predecessor.clone(),
            terms,
            adoption_basis,
            operating_lock_fingerprint,
        };
        locator.validate()?;
        require_predecessor(guard.root(), &locator)?;
        require_basis(guard.root(), &locator.adoption_basis)?;
        if guard
            .root()
            .list_directory_bounded(LOCAL_CHECK_ROOT_V1, 128)
            .is_ok()
        {
            return Err(refuse(
                "already_selected",
                "existing successor footprint requires exact original recovery",
            ));
        }
        Ok(PreparedLocalCheckAuthorityV1 { locator })
    }
    pub fn adopt(
        repo: impl AsRef<Path>,
        prepared: PreparedLocalCheckAuthorityV1,
    ) -> LocalCheckAuthorityExecutionV1 {
        adopt(repo.as_ref(), &prepared.locator, false)
    }
    #[cfg(feature = "test-support")]
    #[doc(hidden)]
    pub fn adopt_with_interruption_for_test(
        repo: impl AsRef<Path>,
        prepared: PreparedLocalCheckAuthorityV1,
        interrupt: impl FnOnce(),
    ) -> LocalCheckAuthorityExecutionV1 {
        adopt_with_interrupt(repo.as_ref(), &prepared.locator, false, interrupt)
    }
    pub fn resume_request(
        repo: impl AsRef<Path>,
        locator: &LocalCheckLocatorV1,
    ) -> LocalCheckAuthorityExecutionV1 {
        adopt(repo.as_ref(), locator, true)
    }
    pub fn resolve_request(
        repo: impl AsRef<Path>,
        locator: &LocalCheckLocatorV1,
    ) -> LocalCheckAuthorityExecutionV1 {
        let result = (|| {
            locator.validate()?;
            let guard = OperatingGuard::acquire(repo.as_ref(), &locator.predecessor, false)?;
            check_history(guard.root(), locator)
        })();
        execution(result, LocalAuthorityDispositionV1::Replayed)
    }
    pub fn resolve_current(
        repo: impl AsRef<Path>,
        locator: &LocalCheckLocatorV1,
        basis: &LocalCheckCurrentBasisV1,
    ) -> Result<LocalCheckAuthorityOutcomeV1, LocalAuthorityRefusalV1> {
        locator.validate()?;
        let guard = OperatingGuard::acquire(repo.as_ref(), &locator.predecessor, false)?;
        let tree = guard
            .root()
            .retain_local_check_authority_tree_guard()
            .map_err(fs_error)?;
        let outcome = check_current(guard.root(), locator, basis)?;
        tree.validate().map_err(fs_error)?;
        guard.validate()?;
        Ok(outcome)
    }
    pub fn with_admission<T>(
        repo: impl AsRef<Path>,
        locator: &LocalCheckLocatorV1,
        basis: &LocalCheckCurrentBasisV1,
        request: &LocalCheckActionRequestV1,
        consume: impl FnOnce(&LocalCheckAdmissionV1<'_>) -> Result<T, LocalAuthorityRefusalV1>,
    ) -> Result<T, LocalAuthorityRefusalV1> {
        locator.validate()?;
        let guard = OperatingGuard::acquire(repo.as_ref(), &locator.predecessor, false)?;
        let outcome = check_current(guard.root(), locator, basis)?;
        validate_action(&locator.terms, request)?;
        let tree = guard
            .root()
            .retain_local_check_authority_tree_guard()
            .map_err(fs_error)?;
        let admission = LocalCheckAdmissionV1 {
            locator,
            basis,
            request,
            guard: &guard,
            tree: &tree,
            authority_fingerprint: outcome.outcome_fingerprint,
            observed_at_utc: crate::authority_admission::current_semantic_utc()
                .map_err(|_| refuse("semantic_time", "native time unavailable"))?,
        };
        admission.validate_current()?;
        let result = consume(&admission)?;
        admission.validate_current()?;
        Ok(result)
    }
    pub fn revoke(
        repo: impl AsRef<Path>,
        locator: &LocalCheckLocatorV1,
        request: &LocalRevokeRequestV1,
    ) -> Result<(), LocalAuthorityRefusalV1> {
        locator.validate()?;
        if !token(&request.request_key) || request.principal != locator.terms.revocation_principal {
            return Err(refuse(
                "revocation_authority",
                "declared revocation principal required",
            ));
        }
        bounded_text(&request.source_ref)?;
        bounded_text(&request.instruction)?;
        let guard = OperatingGuard::acquire(repo.as_ref(), &locator.predecessor, false)?;
        let root = guard.root();
        require_predecessor(root, locator)?;
        let outcome = check_history(root, locator)?;
        let before_head = CheckHead {
            schema_ref: LOCAL_CHECK_RULE_V1.into(),
            selected: outcome.outcome_fingerprint.clone(),
            revoked: None,
        };
        if request.expected_current_fingerprint != before_head.selected {
            return Err(refuse(
                "revocation_predecessor",
                "exact selected successor required",
            ));
        }
        let bytes = encode(request)?;
        let revocation = fingerprint("handbook.local-check-revocation@1.0.0", request)?;
        let before = encode(&before_head)?;
        let after = encode(&CheckHead {
            revoked: Some(revocation),
            ..before_head
        })?;
        let stage = format!("{LOCAL_CHECK_ROOT_V1}/revoked-head.json");
        let displaced = format!("{LOCAL_CHECK_ROOT_V1}/displaced-active-head.json");
        let current = root
            .read_local_owner_optional_bounded(CURRENT, MAX)
            .map_err(fs_error)?;
        match root
            .read_local_owner_optional_bounded(REVOKE, MAX)
            .map_err(fs_error)?
        {
            RepositoryReadV1::Absent => {
                if current != RepositoryReadV1::Found(before.clone()) {
                    return Err(refuse(
                        "revocation_state",
                        "missing or substituted head cannot authorize revocation",
                    ));
                }
                write_exact(root, REVOKE, &bytes)?;
            }
            RepositoryReadV1::Found(retained) if retained == bytes => {}
            _ => {
                return Err(refuse(
                    "revocation_conflict",
                    "another revocation was retained",
                ))
            }
        }
        if current != RepositoryReadV1::Found(after.clone()) {
            write_exact(root, &stage, &after)?;
            if current == RepositoryReadV1::Found(before.clone()) {
                root.replace_exact_durable(CURRENT, &before, &stage, &displaced)
                    .map_err(fs_error)?;
            } else if current == RepositoryReadV1::Absent
                && root
                    .read_local_owner_optional_bounded(&displaced, MAX)
                    .map_err(fs_error)?
                    == RepositoryReadV1::Found(before)
            {
                root.rename_noreplace_durable(&stage, CURRENT)
                    .map_err(fs_error)?;
            } else {
                return Err(refuse(
                    "revocation_state",
                    "only exact retained revocation intent can recover its head publication",
                ));
            }
        }
        if found(root, CURRENT)? != after {
            return Err(refuse("uncertain", "revocation publication unverified"));
        }
        guard.validate()
    }
}

fn adopt(
    repo: &Path,
    locator: &LocalCheckLocatorV1,
    resume: bool,
) -> LocalCheckAuthorityExecutionV1 {
    adopt_with_interrupt(repo, locator, resume, || {})
}
fn adopt_with_interrupt(
    repo: &Path,
    locator: &LocalCheckLocatorV1,
    resume: bool,
    interrupt: impl FnOnce(),
) -> LocalCheckAuthorityExecutionV1 {
    let mut effects = false;
    let mut replay = false;
    let result = (|| {
        locator.validate()?;
        let guard = OperatingGuard::acquire(repo, &locator.predecessor, false)?;
        let root = guard.root();
        let request_path = path(locator, "request.json");
        let terminal = path(locator, "terminal.json");
        let pending = path(locator, "pending-terminal.json");
        if root
            .read_local_owner_optional_bounded(&terminal, MAX)
            .map_err(fs_error)?
            != RepositoryReadV1::Absent
        {
            replay = true;
            return check_history(root, locator);
        }
        require_predecessor(root, locator)?;
        require_basis(root, &locator.adoption_basis)?;
        if root
            .read_local_owner_optional_bounded(REVOKE, MAX)
            .map_err(fs_error)?
            != RepositoryReadV1::Absent
        {
            return Err(refuse("revoked", "recovery cannot restore revoked terms"));
        }
        match root
            .read_local_owner_optional_bounded(&request_path, MAX)
            .map_err(fs_error)?
        {
            RepositoryReadV1::Found(bytes) if bytes == encode(locator)? => {
                if !resume {
                    return Err(refuse(
                        "original_request_pending",
                        "resume exact retained successor intent",
                    ));
                }
            }
            RepositoryReadV1::Found(_) => {
                return Err(refuse(
                    "request_conflict",
                    "request key has different retained content",
                ))
            }
            RepositoryReadV1::Absent if resume => {
                return Err(refuse(
                    "original_request_absent",
                    "a portable locator cannot create missing intent",
                ))
            }
            RepositoryReadV1::Absent => {
                if root
                    .list_directory_bounded(LOCAL_CHECK_ROOT_V1, 128)
                    .is_ok()
                {
                    return Err(refuse(
                        "already_selected",
                        "one explicit successor adoption only",
                    ));
                }
                root.create_directories(&format!(
                    "{LOCAL_CHECK_ROOT_V1}/requests/{}",
                    locator.terms.request_key
                ))
                .map_err(fs_error)?;
                effects = true;
                write_exact(root, &request_path, &encode(locator)?)?;
            }
        }
        let outcome_path = path(locator, "outcome.json");
        if root
            .read_local_owner_optional_bounded(&outcome_path, MAX)
            .map_err(fs_error)?
            != RepositoryReadV1::Absent
            && root
                .read_local_owner_optional_bounded(&pending, MAX)
                .map_err(fs_error)?
                == RepositoryReadV1::Absent
        {
            return Err(refuse(
                "required_terminal_state_missing",
                "lost terminal cannot be recreated",
            ));
        }
        effects = true;
        let outcome = expected_outcome(locator)?;
        let bytes = encode(&outcome)?;
        write_exact(root, &outcome_path, &bytes)?;
        write_exact(root, &pending, &bytes)?;
        interrupt();
        let head = encode(&CheckHead {
            schema_ref: LOCAL_CHECK_RULE_V1.into(),
            selected: outcome.outcome_fingerprint.clone(),
            revoked: None,
        })?;
        write_exact(root, CURRENT, &head)?;
        guard.validate()?;
        require_predecessor(root, locator)?;
        require_basis(root, &locator.adoption_basis)?;
        root.rename_noreplace_durable(&pending, &terminal)
            .map_err(fs_error)?;
        check_current(root, locator, &locator.adoption_basis)
    })();
    execution(
        result,
        if replay {
            LocalAuthorityDispositionV1::Replayed
        } else if effects {
            LocalAuthorityDispositionV1::Committed
        } else {
            LocalAuthorityDispositionV1::Refused
        },
    )
}
fn execution(
    result: Result<LocalCheckAuthorityOutcomeV1, LocalAuthorityRefusalV1>,
    disposition: LocalAuthorityDispositionV1,
) -> LocalCheckAuthorityExecutionV1 {
    match result {
        Ok(outcome) => LocalCheckAuthorityExecutionV1 {
            disposition,
            outcome: Some(outcome),
            refusal: None,
        },
        Err(error) => LocalCheckAuthorityExecutionV1 {
            disposition: if disposition == LocalAuthorityDispositionV1::Committed {
                LocalAuthorityDispositionV1::Uncertain
            } else {
                LocalAuthorityDispositionV1::Refused
            },
            outcome: None,
            refusal: Some(error),
        },
    }
}
fn expected_outcome(
    locator: &LocalCheckLocatorV1,
) -> Result<LocalCheckAuthorityOutcomeV1, LocalAuthorityRefusalV1> {
    Ok(LocalCheckAuthorityOutcomeV1 {
        schema_ref: LOCAL_CHECK_RULE_V1.into(),
        locator: locator.clone(),
        outcome_fingerprint: fingerprint(
            "handbook.local-check-outcome@1.0.0",
            &(locator, "local_advisory"),
        )?,
        assurance: "local_advisory".into(),
    })
}
fn check_history(
    root: &RetainedRepositoryRootV1,
    locator: &LocalCheckLocatorV1,
) -> Result<LocalCheckAuthorityOutcomeV1, LocalAuthorityRefusalV1> {
    let expected = expected_outcome(locator)?;
    if found(root, &path(locator, "request.json"))? != encode(locator)?
        || found(root, &path(locator, "outcome.json"))? != encode(&expected)?
        || found(root, &path(locator, "terminal.json"))? != encode(&expected)?
    {
        return Err(refuse(
            "owner_history",
            "successor history is incomplete or substituted",
        ));
    }
    Ok(expected)
}
fn require_predecessor(
    root: &RetainedRepositoryRootV1,
    locator: &LocalCheckLocatorV1,
) -> Result<(), LocalAuthorityRefusalV1> {
    if root
        .existing_lock_physical_fingerprint_v1(".handbook/state/locks/local-operating.lock")
        .map_err(fs_error)?
        != locator.operating_lock_fingerprint
    {
        return Err(refuse(
            "operating_lock_substituted",
            "the original operating lock is required",
        ));
    }
    let original = read_history(root, &locator.predecessor)?;
    let head: Head = decode(&found(root, HEAD)?)?;
    if original.outcome_fingerprint != locator.terms.expected_predecessor_fingerprint
        || head.schema != LOCAL_OPERATING_RULE_V1
        || head.initial != original.outcome_fingerprint
        || head.revoked.is_some()
        || root
            .read_local_owner_optional_bounded(&format!("{BASE}/revoke.json"), MAX)
            .map_err(fs_error)?
            != RepositoryReadV1::Absent
    {
        return Err(refuse(
            "predecessor_revoked_or_lost",
            "exact unrevoked original predecessor lineage required",
        ));
    }
    local_charter::resolve_current_retained(root, &locator.predecessor.charter)?;
    Ok(())
}
fn check_current(
    root: &RetainedRepositoryRootV1,
    locator: &LocalCheckLocatorV1,
    basis: &LocalCheckCurrentBasisV1,
) -> Result<LocalCheckAuthorityOutcomeV1, LocalAuthorityRefusalV1> {
    require_predecessor(root, locator)?;
    let outcome = check_history(root, locator)?;
    let head: CheckHead = decode(&found(root, CURRENT)?)?;
    if head.schema_ref != LOCAL_CHECK_RULE_V1
        || head.selected != outcome.outcome_fingerprint
        || head.revoked.is_some()
        || root
            .read_local_owner_optional_bounded(REVOKE, MAX)
            .map_err(fs_error)?
            != RepositoryReadV1::Absent
    {
        return Err(refuse(
            "revoked_or_stale_head",
            "local-check head revoked, missing or changed",
        ));
    }
    require_basis(root, basis)?;
    Ok(outcome)
}
fn require_basis(
    root: &RetainedRepositoryRootV1,
    basis: &LocalCheckCurrentBasisV1,
) -> Result<(), LocalAuthorityRefusalV1> {
    check_fingerprint(&basis.relation_fingerprint)?;
    check_fingerprint(&basis.governance_tree_fingerprint)?;
    if DefinitionFingerprint::from_bytes(&found(root, SCOPE_CURRENT)?).as_str()
        != basis.relation_fingerprint
        || governance_tree_identity(root)? != basis.governance_tree_fingerprint
    {
        return Err(refuse(
            "governance_currentness",
            "complete native applicability generations changed; freshly resolve dependent use",
        ));
    }
    Ok(())
}
fn governance_tree_identity(
    root: &RetainedRepositoryRootV1,
) -> Result<String, LocalAuthorityRefusalV1> {
    let mut queue = VecDeque::from([".handbook/admission-governance/v2".to_owned()]);
    let mut inventory = Vec::new();
    let mut bytes = 0_usize;
    while let Some(directory) = queue.pop_front() {
        inventory.push((directory.clone(), "directory".to_owned(), String::new()));
        for entry in root
            .list_directory_bounded(&directory, 4096)
            .map_err(fs_error)?
        {
            if inventory.len() + queue.len() >= 4096 {
                return Err(refuse(
                    "governance_bound",
                    "complete governance inventory exceeds supported bounds",
                ));
            }
            let child = format!("{directory}/{}", entry.name());
            match entry.kind() {
                RepositoryDirectoryEntryKindV1::Directory => queue.push_back(child),
                RepositoryDirectoryEntryKindV1::RegularFile => {
                    let content = found(root, &child)?;
                    bytes = bytes
                        .checked_add(content.len())
                        .ok_or_else(|| refuse("governance_bound", "governance bytes overflow"))?;
                    if bytes > 256 * 1024 * 1024 {
                        return Err(refuse(
                            "governance_bound",
                            "governance bytes exceed supported bounds",
                        ));
                    }
                    inventory.push((
                        child,
                        "file".into(),
                        DefinitionFingerprint::from_bytes(&content).to_string(),
                    ));
                }
            }
        }
    }
    inventory.sort();
    fingerprint(
        "handbook.local-check-governance-observation@1.0.0",
        &inventory,
    )
}
fn validate_action(
    terms: &LocalCheckTermsV1,
    request: &LocalCheckActionRequestV1,
) -> Result<(), LocalAuthorityRefusalV1> {
    for value in [
        &request.action_ref,
        &request.subject_ref,
        &request.actor_ref,
        &request.participant_ref,
    ] {
        bounded_text(value)?;
    }
    check_fingerprint(&request.content_fingerprint)?;
    check_fingerprint(&request.closure_fingerprint)?;
    crate::charter_posture_transaction_intent_v1::validate_utc_second_v1(
        &request.semantic_time_utc,
        "local action time",
    )
    .map_err(|_| {
        refuse(
            "semantic_time",
            "valid canonical UTC-second action time required",
        )
    })?;
    if !terms.action_classes.contains(&request.action) {
        return Err(refuse(
            "action_class",
            "exact permitted local action and semantic time required",
        ));
    }
    let review = request.action == LocalCheckActionV1::Review;
    if (review
        && (!terms
            .review_participant_refs
            .contains(&request.participant_ref)
            || request.participant_ref == terms.author_participant_ref
            || request.actor_ref == terms.agent
            || (request.actor_ref != terms.principal
                && request.actor_ref != request.participant_ref)))
        || (!review
            && (request.actor_ref != terms.agent
                || request.participant_ref != terms.author_participant_ref))
    {
        return Err(refuse(
            "action_participant",
            "actual supplied participant and action role differ from selected terms",
        ));
    }
    for (paths, selectors, owner_mutation) in [
        (&request.owner_write_paths, &terms.allowed_paths, true),
        (&request.input_paths, &terms.allowed_input_paths, false),
    ] {
        if paths.len() > 64 || paths.windows(2).any(|p| p[0] >= p[1]) {
            return Err(refuse(
                "action_paths",
                "action reach must be sorted, unique and bounded",
            ));
        }
        for path in paths {
            if !selectors.iter().any(|selector| {
                if owner_mutation {
                    crate::context_resolution::local_owner_subtree_within_selector(selector, path)
                        .unwrap_or(false)
                } else {
                    crate::context_resolution::local_path_within_selector(selector, path)
                        .unwrap_or(false)
                }
            }) {
                return Err(refuse(
                    "path_expansion",
                    if owner_mutation {
                        "actual consuming owner-write subtree exceeds supplied approved terms"
                    } else {
                        "actual consuming reach exceeds supplied approved terms"
                    },
                ));
            }
        }
    }
    Ok(())
}
fn check_fingerprint(value: &str) -> Result<(), LocalAuthorityRefusalV1> {
    DefinitionFingerprint::parse(value)
        .map(|_| ())
        .map_err(|_| refuse("fingerprint", "exact fingerprint required"))
}
fn bounded_text(value: &str) -> Result<(), LocalAuthorityRefusalV1> {
    if value.trim().is_empty() || value.len() > 4096 || value.chars().any(char::is_control) {
        return Err(refuse("bounded_text", "bounded explicit text required"));
    }
    Ok(())
}
fn path(locator: &LocalCheckLocatorV1, file: &str) -> String {
    format!(
        "{LOCAL_CHECK_ROOT_V1}/requests/{}/{file}",
        locator.terms.request_key
    )
}

#[cfg(test)]
mod action_reach_tests {
    use super::*;

    fn selected(root: &str) -> (LocalCheckTermsV1, LocalCheckActionRequestV1) {
        let terms = LocalCheckTermsV1 {
            request_key: "action-reach".into(),
            mode: "local_advisory".into(),
            rule_ref: LOCAL_CHECK_RULE_V1.into(),
            instruction: "validate bounded owner mutation reach".into(),
            principal: "human".into(),
            agent: "author-agent".into(),
            delegation: "local advisory".into(),
            approval_source_ref: "human-supplied-terms".into(),
            human_approval_asserted: true,
            approved_content_fingerprint: String::new(),
            expected_predecessor_fingerprint: format!("sha256:{}", "1".repeat(64)),
            expected_charter_fingerprint: format!("sha256:{}", "2".repeat(64)),
            ceiling: LocalEnvelopeCeilingV1 {
                active_level_id: "local_operation".into(),
                dimensions: [
                    "local_observation",
                    "identifier_only",
                    "current_operation",
                    "read_only",
                    "operation",
                    "observation_only",
                ]
                .map(str::to_owned),
            },
            constraint_inputs: vec![],
            allowed_paths: vec![root.into()],
            allowed_input_paths: vec!["inputs/order.json".into()],
            action_classes: vec![LocalCheckActionV1::EvidenceSource],
            author_participant_ref: "author-participant".into(),
            review_participant_refs: vec!["human".into()],
            reserved_decisions: vec![],
            delegated_decisions: vec![],
            revocation_principal: "human".into(),
        };
        let request = LocalCheckActionRequestV1 {
            action: LocalCheckActionV1::EvidenceSource,
            action_ref: "handbook.local-source-reach-probe@1.0.0".into(),
            subject_ref: "handbook.local-source-reach-probe@1.0.0".into(),
            content_fingerprint: format!("sha256:{}", "3".repeat(64)),
            closure_fingerprint: format!("sha256:{}", "4".repeat(64)),
            actor_ref: terms.agent.clone(),
            participant_ref: terms.author_participant_ref.clone(),
            semantic_time_utc: "2026-10-05T12:00:00Z".into(),
            owner_write_paths: vec![root.into()],
            input_paths: vec!["inputs/order.json".into()],
        };
        (terms, request)
    }

    #[test]
    fn owner_mutation_roots_require_explicit_complete_subtree_authority() {
        for root in [
            ".handbook/state/contracts/v1/evidence-v1",
            ".handbook/state/contracts/v1/transactions",
            ".handbook/state/contracts/local-v2",
        ] {
            let (mut terms, request) = selected(root);
            assert_eq!(
                validate_action(&terms, &request).unwrap_err().code,
                "path_expansion"
            );
            terms.allowed_paths = vec![format!("{root}/**")];
            validate_action(&terms, &request).unwrap();
            terms.allowed_paths = vec![".handbook/state/contracts/**".into()];
            validate_action(&terms, &request).unwrap();
            terms.allowed_paths = vec![format!("{root}/records/**")];
            assert_eq!(
                validate_action(&terms, &request).unwrap_err().code,
                "path_expansion"
            );
        }
    }

    #[test]
    fn owner_subtree_authority_does_not_expand_literal_input_reach() {
        let root = ".handbook/state/contracts/v1/evidence-v1";
        let (mut terms, mut request) = selected(root);
        terms.allowed_paths = vec![format!("{root}/**")];
        validate_action(&terms, &request).unwrap();
        for outside in ["inputs/order.json/child", "inputs/other.json"] {
            request.input_paths = vec![outside.into()];
            assert_eq!(
                validate_action(&terms, &request).unwrap_err().code,
                "path_expansion"
            );
        }
    }
}
