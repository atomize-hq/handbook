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
    /// Complete native Contracts inventory reads are an additive v3 action
    /// declaration. V1/V2 action input paths retain their literal semantics.
    pub fn require_owner_input_subtree_v3(
        &self,
        root: &str,
    ) -> Result<(), LocalAuthorityRefusalV1> {
        self.require_full_definition_action_v3()?;
        if root != ".handbook/state/contracts"
            || !self.request.input_paths.iter().any(|path| path == root)
            || !self.terms().allowed_input_paths.iter().any(|selector| {
                crate::context_resolution::local_owner_subtree_within_selector(selector, root)
                    .unwrap_or(false)
            })
        {
            return Err(refuse("definition_read_subtree","v3 definition inventory needs literal root declaration and explicit complete approved read subtree"));
        }
        self.validate_current()
    }
    pub fn require_input_path_v3(&self, path: &str) -> Result<(), LocalAuthorityRefusalV1> {
        self.require_full_definition_action_v3()?;
        let literal = self
            .request
            .input_paths
            .iter()
            .any(|declared| declared == path);
        let owned = path.starts_with(".handbook/state/contracts/")
            && self
                .require_owner_input_subtree_v3(".handbook/state/contracts")
                .is_ok();
        if (!literal && !owned)
            || !self.terms().allowed_input_paths.iter().any(|selector| {
                crate::context_resolution::local_path_within_selector(selector, path)
                    .unwrap_or(false)
            })
        {
            return Err(refuse(
                "definition_read_path",
                "actual v3 definition proof read exceeds exact declaration or approved terms",
            ));
        }
        self.validate_current()
    }
    fn require_full_definition_action_v3(&self) -> Result<(), LocalAuthorityRefusalV1> {
        if !matches!(
            (self.request.action, self.request.action_ref.as_str()),
            (
                LocalCheckActionV1::DefinitionAuthor,
                "handbook.contract-local-definition-append@3.0.0"
            ) | (
                LocalCheckActionV1::Review,
                "handbook.contract-local-review@3.0.0"
            ) | (
                LocalCheckActionV1::Lifecycle,
                "handbook.contract-local-lifecycle-transition@3.0.0"
            )
        ) {
            return Err(refuse(
                "definition_action",
                "complete Contracts read declarations apply only to exact v3 definition actions",
            ));
        }
        Ok(())
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
        check_current(self.guard, self.locator, self.basis)?;
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

/// The sole purpose admitted by the Field Notes local-advisory Charter read.
/// This exact owner-approved instruction is not a general purpose parser.
pub const FIELD_NOTES_PLANNING_INSTRUCTION_V1: &str =
    "Inspect the Field Notes Charter and index.html as local evidence for planning the demo.";

/// Bytes and fingerprint from one guarded local-advisory observation. The
/// named parties are assertions in the selected terms, not authenticated identities.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalPlanningCharterReadV1 {
    pub canonical_bytes: Vec<u8>,
    pub canonical_fingerprint: String,
    pub unverified_asserted_agent: String,
    pub unverified_asserted_principal: String,
}

pub struct NativeLocalCheckAuthorityV1;
impl NativeLocalCheckAuthorityV1 {
    /// Dedicated Charter read for the exact selected Field Notes instruction.
    /// The caller-supplied agent is an unverified advisory assertion.
    pub fn read_planning_charter(
        repo: impl AsRef<Path>,
        locator: &LocalCheckLocatorV1,
        basis: &LocalCheckCurrentBasisV1,
        unverified_asserted_agent: &str,
        asserted_instruction: &str,
    ) -> Result<LocalPlanningCharterReadV1, LocalAuthorityRefusalV1> {
        const CHARTER: &str = ".handbook/project/charter.yaml";
        locator.validate()?;
        let guard = OperatingGuard::acquire(repo.as_ref(), &locator.predecessor, false)?;
        let tree = guard
            .root()
            .retain_local_check_authority_tree_guard()
            .map_err(fs_error)?;
        check_current(&guard, locator, basis)?;
        // This consuming rule is authorized for the exact approved Field Notes
        // successor, whose validated terms bind its original predecessor and Charter.
        if locator.terms.approved_content_fingerprint
            != "sha256:6d2f7bae73225ea0fbb3000a2e6857d1fbf80e064d81188f11398b28baa39ec6"
            && locator.terms.approved_content_fingerprint
                != "sha256:cf25d67d8b5addce25a6d9edaac3412a48d01caee0856dd24ef965bc4a304046"
            && locator.terms.approved_content_fingerprint
                != "sha256:fde572fa8066c4a7f3b74092f1e76ee33ce4ae856e4d773c260ba2a9830586eb"
        {
            return Err(refuse(
                "planning_charter_authority",
                "the exact approved Field Notes successor terms are required",
            ));
        }
        if asserted_instruction != FIELD_NOTES_PLANNING_INSTRUCTION_V1
            || locator.terms.instruction != FIELD_NOTES_PLANNING_INSTRUCTION_V1
            || unverified_asserted_agent != locator.terms.agent
            || !locator.terms.allowed_input_paths.iter().any(|selector| {
                crate::context_resolution::local_path_within_selector(selector, CHARTER)
                    .unwrap_or(false)
            })
        {
            return Err(refuse("planning_charter_scope", "exact advisory agent assertion, selected Field Notes instruction and approved Charter input are required"));
        }
        let bytes = match guard
            .root()
            .read_local_owner_optional_bounded(CHARTER, 1_048_576)
            .map_err(fs_error)?
        {
            RepositoryReadV1::Found(bytes) => bytes,
            RepositoryReadV1::Absent => {
                return Err(refuse(
                    "planning_charter_missing",
                    "selected Charter bytes are absent",
                ))
            }
        };
        let digest = DefinitionFingerprint::from_bytes(&bytes).to_string();
        if digest != locator.predecessor.charter.canonical_fingerprint {
            return Err(refuse(
                "planning_charter_changed",
                "observed Charter bytes differ from the exact approved source",
            ));
        }
        tree.validate().map_err(fs_error)?;
        guard.validate()?;
        check_current(&guard, locator, basis)?;
        Ok(LocalPlanningCharterReadV1 {
            canonical_bytes: bytes,
            canonical_fingerprint: digest,
            unverified_asserted_agent: unverified_asserted_agent.to_owned(),
            unverified_asserted_principal: locator.terms.principal.clone(),
        })
    }
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
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct ClaimRecord {
            schema_id: String,
            schema_version: String,
            claim: crate::RepositoryInitializationClaimV1,
            retained_lock_fingerprint: String,
        }
        let record: ClaimRecord = decode(&found(root, crate::REPOSITORY_INITIALIZATION_CLAIM_PATH_V1)?)?;
        if record.schema_id != "handbook.repository-initialization-claim-record"
            || record.schema_version != "1.0"
            || record.retained_lock_fingerprint != root.initialization_lock_physical_fingerprint_v1().map_err(fs_error)?
        {
            return Err(refuse("governance_capture", "Exact initialization claim and retained lock required"));
        }
        let (_, relation, identity) = capture_governance_current_guarded(&guard, &record.claim)?;
        let basis = LocalCheckCurrentBasisV1 {
            relation_fingerprint: relation_fingerprint.into(),
            governance_tree_fingerprint: identity,
        };
        if DefinitionFingerprint::from_bytes(&relation).as_str() != basis.relation_fingerprint {
            return Err(refuse("governance_currentness", "Exact current relation required"));
        }
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
        require_basis(&guard, &locator.adoption_basis)?;
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
        let outcome = check_current(&guard, locator, basis)?;
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
        let outcome = check_current(&guard, locator, basis)?;
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
        require_basis(&guard, &locator.adoption_basis)?;
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
        require_basis(&guard, &locator.adoption_basis)?;
        root.rename_noreplace_durable(&pending, &terminal)
            .map_err(fs_error)?;
        check_current(&guard, locator, &locator.adoption_basis)
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
    guard: &OperatingGuard,
    locator: &LocalCheckLocatorV1,
    basis: &LocalCheckCurrentBasisV1,
) -> Result<LocalCheckAuthorityOutcomeV1, LocalAuthorityRefusalV1> {
    let root = guard.root();
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
    require_basis(guard, basis)?;
    Ok(outcome)
}

// The additive full-evaluation owner uses these checks under the same native
// OperatingGuard. It must not recursively reacquire that guard through a public
// local-check action or manufacture an old action as evaluation permission.
pub(super) fn evaluation_predecessor_current(
    guard: &OperatingGuard,
    locator: &LocalCheckLocatorV1,
    basis: &LocalCheckCurrentBasisV1,
) -> Result<LocalCheckAuthorityOutcomeV1, LocalAuthorityRefusalV1> {
    locator.validate()?;
    check_current(guard, locator, basis)
}

pub(super) fn evaluation_predecessor_history(
    root: &RetainedRepositoryRootV1,
    locator: &LocalCheckLocatorV1,
) -> Result<LocalCheckAuthorityOutcomeV1, LocalAuthorityRefusalV1> {
    locator.validate()?;
    check_history(root, locator)
}

// The work owner has its own fixed establishment rule and permission. These
// crate-private checks reuse the native lineage/currentness mechanics without
// manufacturing one of the four local-check action classes as work permission.
pub(super) fn work_registration_current(
    guard: &OperatingGuard,
    locator: &LocalCheckLocatorV1,
    basis: &LocalCheckCurrentBasisV1,
) -> Result<LocalCheckAuthorityOutcomeV1, LocalAuthorityRefusalV1> {
    locator.validate()?;
    check_current(guard, locator, basis)
}

pub(super) fn work_registration_expected_outcome(
    locator: &LocalCheckLocatorV1,
) -> Result<LocalCheckAuthorityOutcomeV1, LocalAuthorityRefusalV1> {
    locator.validate()?;
    expected_outcome(locator)
}
fn require_basis(
    guard: &OperatingGuard,
    basis: &LocalCheckCurrentBasisV1,
) -> Result<(), LocalAuthorityRefusalV1> {
    check_fingerprint(&basis.relation_fingerprint)?;
    check_fingerprint(&basis.governance_tree_fingerprint)?;
    let (_, relation, identity) = guard.governance.get_or_init(|| {
        capture_governance_current_guarded(&guard.initialization, &guard.governance_claim)
    }).as_ref().map_err(Clone::clone)?;
    if DefinitionFingerprint::from_bytes(relation).as_str() != basis.relation_fingerprint
        || identity != &basis.governance_tree_fingerprint
    {
        return Err(refuse(
            "governance_currentness",
            "complete selected native governance evidence changed; freshly resolve dependent use",
        ));
    }
    Ok(())
}
pub(super) fn governance_tree_identity(
    capture: &GovernanceAuthorityCaptureV1,
) -> Result<String, LocalAuthorityRefusalV1> {
    fingerprint("handbook.local-check-governance-observation@2.0.0", capture)
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

/// Owner-independent, bounded selected-governance evidence. A live capture is
/// retained only alongside the original initialization guard; its serialized
/// bytes support historical Work proof without consulting current storage.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct GovernanceAuthorityCaptureV1 {
    pub claim: crate::RepositoryInitializationClaimV1,
    pub root_path: String,
    pub governance_lock_fingerprint: String,
    pub version: file_native::Version,
    pub anchor: String,
    pub root: String,
    pub current: String,
    pub original: String,
    pub evidence: serde_json::Value,
    pub transition: Option<(String, serde_json::Value)>,
}

/// Checks fixed storage identities and the original native transaction against
/// canonical owner bytes. This never accepts a caller-supplied current head.
pub(super) fn validate_captured_governance(
    capture: &GovernanceAuthorityCaptureV1,
    claim: &crate::RepositoryInitializationClaimV1,
) -> Result<Vec<u8>, LocalAuthorityRefusalV1> {
    fn bad() -> LocalAuthorityRefusalV1 {
        refuse("governance_capture", "Exact selected governance and original evidence required")
    }
    fn object<'a>(value: &'a serde_json::Value, keys: &[&str]) -> Result<&'a serde_json::Map<String, serde_json::Value>, LocalAuthorityRefusalV1> {
        let object = value.as_object().ok_or_else(bad)?;
        if object.len() != keys.len() || keys.iter().any(|key| !object.contains_key(*key)) {
            return Err(bad());
        }
        Ok(object)
    }
    fn field_bytes(value: &serde_json::Value, key: &str) -> Result<Vec<u8>, LocalAuthorityRefusalV1> {
        serde_json::from_value(value.get(key).cloned().ok_or_else(bad)?).map_err(codec)
    }
    fn field_str<'a>(value: &'a serde_json::Value, key: &str) -> Result<&'a str, LocalAuthorityRefusalV1> {
        value.get(key).and_then(serde_json::Value::as_str).ok_or_else(bad)
    }
    fn exact(raw: &[u8]) -> Result<serde_json::Value, LocalAuthorityRefusalV1> {
        let value: serde_json::Value = decode(raw)?;
        if encode(&value)? != raw { return Err(bad()); }
        Ok(value)
    }

    fn validate_transition(
        original_text: &str,
        native: &serde_json::Value,
        current: &serde_json::Value,
        deferred_bytes: &[u8],
        deferred: &serde_json::Value,
        original_rule: &serde_json::Value,
        capture: &GovernanceAuthorityCaptureV1,
        claim: &crate::RepositoryInitializationClaimV1,
    ) -> Result<Vec<u8>, LocalAuthorityRefusalV1> {
        let original_bytes = original_text.as_bytes();
        let original = exact(original_bytes)?;
        object(&original, &["schema","id","operation","request_key","claim_fingerprint","store",
            "locator","outcome","noncommit","terminal","relation","predecessor","facts"])?;
        let id = field_str(&original, "id")?;
        let request_key = field_str(&original, "request_key")?;
        let store = field_str(&original, "store")?;
        let expected_id = fingerprint("handbook.s06-request-key@1", &("deferred-g", claim.claim_fingerprint(), request_key))?;
        if field_str(&original, "schema")? != "handbook.project-governance-storage@1"
            || field_str(&original, "operation")? != "deferred-g"
            || field_str(&original, "claim_fingerprint")? != claim.claim_fingerprint()
            || expected_id.strip_prefix("sha256:") != Some(id)
            || current.get("schema") != original.get("schema")
            || current.get("id") != original.get("id")
            || current.get("operation") != original.get("operation")
            || current.get("claim_fingerprint") != original.get("claim_fingerprint")
            || current.get("store") != original.get("store")
            || current.get("relation") != original.get("relation")
            || current.get("predecessor") != original.get("predecessor")
            || field_bytes(&original, "predecessor")? != deferred_bytes
        { return Err(bad()); }
        let locator_bytes = field_bytes(&original, "locator")?;
        let outcome_bytes = field_bytes(&original, "outcome")?;
        let relation_bytes = field_bytes(&original, "relation")?;
        let terminal = field_bytes(&original, "terminal")?;
        let locator = exact(&locator_bytes)?;
        let outcome = exact(&outcome_bytes)?;
        let relation = exact(&relation_bytes)?;
        object(&locator, &["request","request_fingerprint"])?;
        object(&outcome, &["locator","validated","validation_confirmed","advisory_permission_accepted",
            "activation_committed","validation_fingerprint","advisory_permission_fingerprint","outcome_fingerprint"])?;
        object(&relation, &["locator","project","scope","predecessor","configuration","configuration_fingerprint",
            "replacement_basis","establishment_rule","owner_rule","request_fingerprint","outcome_fingerprint","relation_fingerprint"])?;
        let request = locator.get("request").ok_or_else(bad)?;
        let validated = outcome.get("validated").ok_or_else(bad)?;
        object(request, &["project","scope","expected_deferred","configuration","replacement_basis","instruction","request_key"])?;
        object(validated, &["input","configuration_fingerprint","instruction_fingerprint","establishment_rule","owner_rule"])?;
        let instruction = request.get("instruction").ok_or_else(bad)?;
        object(instruction, &["mode","human_approval_asserted","approved_content_fingerprint","text",
            "asserted_principal","asserted_agent","asserted_delegation"])?;
        let expected_deferred = request.get("expected_deferred").ok_or_else(bad)?;
        object(expected_deferred, &["outcome_fingerprint","relation_fingerprint"])?;
        let configuration = request.get("configuration").ok_or_else(bad)?;
        object(configuration, &["owner","name","version","bindings"])?;
        let bindings = configuration.get("bindings").and_then(serde_json::Value::as_array).ok_or_else(bad)?;
        if bindings.len() != 1 { return Err(bad()); }
        let binding = &bindings[0];
        object(binding, &["owner","rule_name","rule_version","candidate_kind","max_observation_age_seconds",
            "require_exact_source_revision","future_skew_seconds","required_authored_sources","approval_sources","exception_sources"])?;
        let replacement = request.get("replacement_basis").ok_or_else(bad)?;
        let fixed_replacement = serde_json::json!({"mode":"standalone_local_advisory",
            "explicit_local_instruction_required":true,"same_project_and_scope_only":true,"preserve_fixed_constraints":true,
            "require_exact_predecessor":true,"require_confirmed_cutover":true,"terms_immutable":true});
        let valid_text = |v: &serde_json::Value| v.as_str().is_some_and(|t|
            !t.trim().is_empty() && t.len() <= 4096 && !t.contains('\0'));
        let age = binding.get("max_observation_age_seconds").and_then(serde_json::Value::as_u64).ok_or_else(bad)?;
        if request.get("project") != deferred.get("project")
            || request.get("scope").and_then(serde_json::Value::as_str) != Some("contracts.evidence-freshness-policy/root")
            || expected_deferred.get("outcome_fingerprint") != deferred.get("outcome_fingerprint")
            || expected_deferred.get("relation_fingerprint") != deferred.get("relation_fingerprint")
            || request.get("request_key").and_then(serde_json::Value::as_str) != Some(request_key)
            || request_key.is_empty() || request_key.len() > 128
            || !request_key.bytes().all(|b| b.is_ascii_alphanumeric() || matches!(b,b'-'|b'_'|b'.'))
            || configuration.get("owner").and_then(serde_json::Value::as_str) != Some("handbook-contracts")
            || configuration.get("name").and_then(serde_json::Value::as_str) != Some("project.admission.freshness")
            || configuration.get("version").and_then(serde_json::Value::as_str) != Some("1.0.0")
            || binding.get("owner").and_then(serde_json::Value::as_str) != Some("handbook-contracts")
            || binding.get("rule_name").and_then(serde_json::Value::as_str) != Some("handbook.contracts.freshness-policy-admission")
            || binding.get("rule_version").and_then(serde_json::Value::as_str) != Some("1.0.0")
            || binding.get("candidate_kind").and_then(serde_json::Value::as_str) != Some("EvidenceFreshnessPolicyV1")
            || age == 0 || age > 9_007_199_254_740_991
            || binding.get("require_exact_source_revision") != Some(&serde_json::Value::Bool(true))
            || binding.get("future_skew_seconds").and_then(serde_json::Value::as_u64) != Some(0)
            || ["required_authored_sources","approval_sources","exception_sources"].iter()
                .any(|key| binding.get(*key).and_then(serde_json::Value::as_array).is_none_or(|v| !v.is_empty()))
            || replacement != &fixed_replacement
            || instruction.get("mode").and_then(serde_json::Value::as_str) != Some("standalone_local_advisory")
            || instruction.get("human_approval_asserted") != Some(&serde_json::Value::Bool(true))
            || instruction.get("text").is_none_or(|v| !valid_text(v))
            || ["asserted_principal","asserted_agent","asserted_delegation"].iter()
                .any(|key| instruction.get(*key).is_none_or(|v| !v.is_null() && !valid_text(v)))
            || validated.get("input") != Some(request)
            || outcome.get("locator") != Some(&locator)
            || ["validation_confirmed","advisory_permission_accepted","activation_committed"].iter()
                .any(|key| outcome.get(*key) != Some(&serde_json::Value::Bool(true)))
        { return Err(bad()); }

        let owner_constraints = serde_json::json!([
            "caller_supplied_max_observation_age_seconds_1_through_9007199254740991",
            "candidate_max_observation_age_seconds_not_exceed_configured_ceiling",
            "require_exact_source_revision_true","future_skew_seconds_zero","exact_single_binding",
            "exact_empty_governance_source_population"]);
        let owner_content = serde_json::json!({"owner":"handbook-contracts",
            "name":"handbook.contracts.freshness-policy-admission","version":"1.0.0",
            "candidate_kind":"EvidenceFreshnessPolicyV1",
            "candidate_schema_ref":"handbook.contract-evidence-freshness-policy@1.0.0",
            "required_authored_sources":[],"approval_sources":[],"exception_sources":[],"constraints":owner_constraints});
        let owner_fp = fingerprint("handbook.project-admission-governance.shipped-rule@1.0.0", &owner_content)?;
        let owner_binding = serde_json::json!({"ref":"handbook.contracts.freshness-policy-admission@1.0.0","fingerprint":owner_fp});
        let mut owner_rule = owner_content.clone();
        owner_rule.as_object_mut().ok_or_else(bad)?.insert("binding".into(), owner_binding.clone());
        let original_binding = original_rule.get("binding").ok_or_else(bad)?;
        let mut constraints = vec![
            "positive_exact_current_deferred_never_established_lifecycle",
            "same_physical_project_identity_registration_and_scope",
            "preserve_original_deferred_request_claim_receipt_and_consumed_initial_choice",
            "new_exact_human_approved_configuration_and_immutable_terms",
            "explicit_standalone_local_advisory_instruction","immutable_configuration_content_and_exact_supported_owner_rule",
            "preserve_fixed_owner_conformance_floors","separate_validation_accepted_permission_and_commit",
            "single_durable_decision_exact_predecessor_and_confirmed_cutover",
            "complete_linked_request_history_and_verified_predecision_noncommit",
            "decided_recovery_finishes_only_original_governance","historical_replay_never_restores_or_repairs_current_relation",
            "unsupported_inventory_and_uncertain_history_refuse_fresh_attempts","no_circular_candidate_authority",
            "no_scope_to_qualification_evaluation_or_action_authority"
        ].into_iter().map(str::to_owned).collect::<Vec<_>>();
        constraints.push(format!("original_establishment_rule:{}:{}", field_str(original_binding,"ref")?,field_str(original_binding,"fingerprint")?));
        let content = serde_json::json!({"owner":"handbook","name":"handbook.admission-establishment.standalone",
            "version":"3.0.0","candidate_kind":null,"candidate_schema_ref":null,"required_authored_sources":[],
            "approval_sources":[],"exception_sources":[],"constraints":constraints,"original_establishment_rule":original_binding});
        let rule_fp = fingerprint("handbook.project-admission-governance.shipped-rule@3.0.0", &content)?;
        let rule_binding = serde_json::json!({"ref":"handbook.admission-establishment.standalone@3.0.0","fingerprint":rule_fp});
        let mut rule = content.clone();
        rule.as_object_mut().ok_or_else(bad)?.remove("original_establishment_rule");
        rule.as_object_mut().ok_or_else(bad)?.insert("binding".into(),rule_binding.clone());
        let configuration_fp = fingerprint("handbook.project-admission-governance.configuration@1.0.0", configuration)?;
        let instruction_fp = fingerprint("handbook.project-admission-governance.local-advisory-instruction@3.0.0", instruction)?;
        let approved = fingerprint("handbook.project-admission-governance.approved-content@3.0.0",
            &serde_json::json!({"project":request.get("project"),"scope":request.get("scope"),
                "expected_deferred":expected_deferred,"configuration":configuration,"replacement_basis":replacement,
                "establishment_rule":rule_binding,"owner_rule":owner_binding}))?;
        let request_fp = fingerprint("handbook.project-admission-governance.request@3.0.0",
            &serde_json::json!({"request":request,"establishment_rule":rule_binding,"owner_rule":owner_binding}))?;
        let validation_fp = fingerprint("handbook.admission-governance-validation@3.0.0",
            &serde_json::json!([locator,validated,"exact_owner_conformance_confirmed"]))?;
        let permission_fp = fingerprint("handbook.admission-governance-advisory-permission@3.0.0",
            &serde_json::json!([locator,instruction_fp,rule_binding,"accepted_standalone_local_advisory_assertion"]))?;
        let outcome_fp = fingerprint("handbook.admission-governance-outcome@3.0.0",
            &serde_json::json!([locator,validated,validation_fp,permission_fp,"validation_confirmed",
                "local_advisory_permission_accepted","first_establishment_committed"]))?;
        let relation_fp = fingerprint("handbook.admission-governance-current-relation@3.0.0",
            &serde_json::json!([locator,validated,outcome_fp,"established_from_exact_deferred"]))?;
        if validated.get("establishment_rule") != Some(&rule) || validated.get("owner_rule") != Some(&owner_rule)
            || field_str(validated,"configuration_fingerprint")? != configuration_fp
            || field_str(validated,"instruction_fingerprint")? != instruction_fp
            || field_str(instruction,"approved_content_fingerprint")? != approved
            || field_str(&locator,"request_fingerprint")? != request_fp
            || field_str(&outcome,"validation_fingerprint")? != validation_fp
            || field_str(&outcome,"advisory_permission_fingerprint")? != permission_fp
            || field_str(&outcome,"outcome_fingerprint")? != outcome_fp
            || relation.get("locator") != Some(&locator) || relation.get("project") != request.get("project")
            || relation.get("scope") != request.get("scope") || relation.get("predecessor") != Some(expected_deferred)
            || relation.get("configuration") != Some(configuration) || relation.get("replacement_basis") != Some(replacement)
            || relation.get("establishment_rule") != Some(&rule) || relation.get("owner_rule") != Some(&owner_rule)
            || field_str(&relation,"configuration_fingerprint")? != configuration_fp
            || field_str(&relation,"request_fingerprint")? != request_fp
            || field_str(&relation,"outcome_fingerprint")? != outcome_fp
            || field_str(&relation,"relation_fingerprint")? != relation_fp
        { return Err(bad()); }
        let staged = vec![
            encode(&(&rule,&owner_rule))?, encode(&(configuration,&configuration_fp))?,
            encode(&(instruction,&instruction_fp))?, encode(replacement)?, outcome_bytes.clone(),
            encode(&(request.get("project").ok_or_else(bad)?,request.get("scope").ok_or_else(bad)?,
                expected_deferred,&relation_fp))?, relation_bytes.clone(),
        ];
        let stage_fp = staged.iter().map(|v| fingerprint("handbook.admission-stage-bytes@3.0.0",v))
            .collect::<Result<Vec<_>,_>>()?;
        let intent = encode(&("handbook.admission-establishment-intent.v3",&locator,stage_fp))?;
        let decision = encode(&("handbook.admission-establishment-decision.v3",
            fingerprint("handbook.admission-intent-bytes@3.0.0",&intent)?,expected_deferred))?;
        let expected_terminal = encode(&("handbook.admission-establishment-terminal.v3",&outcome_fp,&relation_fp,
            fingerprint("handbook.admission-decision-bytes@3.0.0",&decision)?))?;
        let condition = "verified_predecision_noncommit";
        let noncommit = encode(&serde_json::json!({"locator":locator,"condition":condition,"activation_committed":false,
            "outcome_fingerprint":fingerprint("handbook.admission-noncommit@3.0.0",&(&locator,condition))?}))?;
        let facts: Vec<Vec<u8>> = serde_json::from_value(original.get("facts").cloned().ok_or_else(bad)?).map_err(codec)?;
        if facts.len()!=9 || facts[..7]!=staged || facts[7]!=intent || facts[8]!=decision
            || terminal!=expected_terminal || field_bytes(&original,"noncommit")?!=noncommit
        { return Err(bad()); }
        let reference: file_native::EvidenceReference = serde_json::from_value(native.get("reference").cloned().ok_or_else(bad)?).map_err(codec)?;
        let observed: file_native::OriginalOutcome = serde_json::from_value(native.get("outcome").cloned().ok_or_else(bad)?).map_err(codec)?;
        let metadata = native.get("evidence").ok_or_else(bad)?;
        object(metadata, &["scope","binding","previous"])?;
        let mut complete_evidence = metadata.clone();
        complete_evidence.as_object_mut().ok_or_else(bad)?.insert(
            "payload".into(), serde_json::to_value(original_bytes).map_err(codec)?);
        let evidence: file_native::RetainedEvidence = serde_json::from_value(complete_evidence).map_err(codec)?;
        let binding = &evidence.binding;
        let version = serde_json::to_value(&capture.version).map_err(codec)?;
        let valid_token = |v: &str| v.len()==64 && v.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b));
        let key = format!("governance/original/{id}");
        if observed.disposition != file_native::OriginalDisposition::Selected || observed.value != outcome_bytes
            || reference.key != key || binding.key != key || binding.fingerprint != reference.fingerprint
            || binding.format != "file-native@1" || binding.namespace != "handbook.project-governance@1"
            || binding.store != store || version.get("store").and_then(serde_json::Value::as_str) != Some(store)
            || evidence.scope != claim.claim_fingerprint() || evidence.payload != original_bytes
            || binding.expected != binding.predecessor || binding.expected.sequence == 0
            || binding.candidate != binding.resolving || binding.candidate.store != store
            || binding.candidate.sequence != binding.expected.sequence.checked_add(1).ok_or_else(bad)?
            || observed.sequence != binding.candidate.sequence || capture.version.sequence < observed.sequence
            || !valid_token(&binding.predecessor_token) || !valid_token(&binding.candidate.token)
            || !valid_token(&reference.fingerprint)
        { return Err(bad()); }
        let mut tx = file_native::Transaction::new(binding.expected.clone(),&key,outcome_bytes);
        tx.set_evidence(claim.claim_fingerprint(),original_bytes.to_vec()).map_err(codec)?;
        tx.put("governance/root",capture.root.as_bytes().to_vec()).map_err(codec)?;
        tx.put(format!("governance/request/{id}"),original_bytes.to_vec()).map_err(codec)?;
        tx.put("governance/current/contracts.evidence-freshness-policy/root",capture.current.as_bytes().to_vec()).map_err(codec)?;
        tx.put(format!("governance/history/{:016x}",binding.candidate.sequence),
            encode(&(id,"deferred-g",&terminal))?).map_err(codec)?;
        let tx_fp = DefinitionFingerprint::from_bytes(&serde_json::to_vec(&tx).map_err(codec)?).to_string();
        if tx_fp.strip_prefix("sha256:") != Some(reference.fingerprint.as_str()) { return Err(bad()); }
        Ok(relation_bytes)
    }
    const SCHEMA: &str = "handbook.project-governance-storage@1";
    const SCOPE: &str = "contracts.evidence-freshness-policy/root";
    claim.validate().map_err(fs_error)?;
    if &capture.claim != claim || claim.native_owner() != "handbook-contracts.admission-deferral@2.0.0" {
        return Err(bad());
    }
    use sha2::{Digest, Sha256};
    let mut root_digest = Sha256::new();
    root_digest.update(b"handbook.repository-initialization-root-path@1.0\0");
    root_digest.update(capture.root_path.as_bytes());
    if format!("sha256:{:x}", root_digest.finalize()) != claim.root_path_digest() {
        return Err(bad());
    }
    let anchor = exact(capture.anchor.as_bytes())?;
    object(&anchor, &["locator", "governance_lock_fingerprint"])?;
    if field_str(&anchor, "governance_lock_fingerprint")? != capture.governance_lock_fingerprint {
        return Err(bad());
    }
    let selected_current = exact(capture.current.as_bytes())?;
    object(&selected_current, &["schema", "id", "operation", "claim_fingerprint", "store", "relation", "predecessor"])?;
    let original = exact(capture.original.as_bytes())?;
    object(&original, &["schema", "id", "operation", "request_key", "claim_fingerprint", "store",
        "locator", "outcome", "noncommit", "terminal", "relation", "predecessor", "facts"])?;
    let deferred_current = serde_json::json!({
        "schema": original.get("schema").ok_or_else(bad)?,
        "id": original.get("id").ok_or_else(bad)?,
        "operation": original.get("operation").ok_or_else(bad)?,
        "claim_fingerprint": original.get("claim_fingerprint").ok_or_else(bad)?,
        "store": original.get("store").ok_or_else(bad)?,
        "relation": original.get("relation").ok_or_else(bad)?,
        "predecessor": original.get("predecessor").ok_or_else(bad)?,
    });
    let current = if capture.transition.is_some() { &deferred_current } else { &selected_current };
    let id = field_str(&original, "id")?;
    let operation = field_str(&original, "operation")?;
    let store = field_str(&original, "store")?;
    let request_key = field_str(&original, "request_key")?;
    let version = serde_json::to_value(&capture.version).map_err(codec)?;
    let expected_id = fingerprint("handbook.s06-request-key@1", &(operation, claim.claim_fingerprint(), request_key))?;
    if field_str(&original, "schema")? != SCHEMA || field_str(&current, "schema")? != SCHEMA
        || operation != "initial-d" || field_str(&current, "operation")? != operation
        || expected_id.strip_prefix("sha256:") != Some(id)
        || field_str(&current, "id")? != id
        || field_str(&original, "claim_fingerprint")? != claim.claim_fingerprint()
        || field_str(&current, "claim_fingerprint")? != claim.claim_fingerprint()
        || field_str(&current, "store")? != store || version.get("store").and_then(serde_json::Value::as_str) != Some(store)
        || original.get("predecessor") != Some(&serde_json::Value::Null)
        || current.get("predecessor") != Some(&serde_json::Value::Null)
        || current.get("relation") != original.get("relation")
        || capture.root.as_bytes() != encode(&(SCHEMA, claim, store))?
    {
        return Err(bad());
    }
    let locator_bytes = field_bytes(&original, "locator")?;
    let outcome_bytes = field_bytes(&original, "outcome")?;
    let relation_bytes = field_bytes(&original, "relation")?;
    let terminal_bytes = field_bytes(&original, "terminal")?;
    let locator = exact(&locator_bytes)?;
    let outcome = exact(&outcome_bytes)?;
    let relation = exact(&relation_bytes)?;
    if anchor.get("locator") != Some(&locator)
        || locator.get("initialization_claim") != Some(&serde_json::to_value(claim).map_err(codec)?)
        || outcome.get("locator") != Some(&locator)
        || outcome.get("validation_confirmed") != Some(&serde_json::Value::Bool(true))
        || outcome.get("advisory_permission_accepted") != Some(&serde_json::Value::Bool(true))
        || outcome.get("deferral_committed") != Some(&serde_json::Value::Bool(true))
        || relation.get("never_established") != Some(&serde_json::Value::Bool(true))
        || relation.get("scope").and_then(serde_json::Value::as_str) != Some(SCOPE)
    {
        return Err(bad());
    }
    object(&locator, &["project", "request", "request_fingerprint", "initialization_claim"])?;
    object(&outcome, &["locator", "validated", "validation_confirmed", "advisory_permission_accepted", "deferral_committed", "outcome_fingerprint"])?;
    object(&relation, &["project", "scope", "never_established", "registration_identity", "deferral_fingerprint", "outcome_fingerprint", "relation_fingerprint", "establishment_rule"])?;
    let project = locator.get("project").ok_or_else(bad)?;
    let request = locator.get("request").ok_or_else(bad)?;
    let validated = outcome.get("validated").ok_or_else(bad)?;
    let instruction = request.get("instruction").ok_or_else(bad)?;
    object(project, &["repository_identity_fingerprint", "retained_root", "physical_root_fingerprint", "registration_identity"])?;
    object(request, &["scope", "instruction", "expected_initial_state", "request_key"])?;
    object(validated, &["input", "instruction_fingerprint", "establishment_rule"])?;
    object(instruction, &["mode", "choice", "human_approval_asserted", "text", "asserted_principal", "asserted_agent", "asserted_delegation"])?;
    let bounded = |value: &serde_json::Value| value.as_str()
        .is_some_and(|text| !text.trim().is_empty() && text.len() <= 4096 && !text.contains('\0'));
    if !bounded(instruction.get("text").ok_or_else(bad)?)
        || ["asserted_principal", "asserted_agent", "asserted_delegation"].iter()
            .any(|key| instruction.get(*key).is_none_or(|value| !value.is_null() && !bounded(value)))
        || instruction.get("mode").and_then(serde_json::Value::as_str) != Some("standalone_local_advisory")
        || instruction.get("choice").and_then(serde_json::Value::as_str) != Some("defer_initial_governance")
        || request.get("expected_initial_state").and_then(serde_json::Value::as_str) != Some("fresh_registration_unused")
        || request_key.len() > 128 || request_key.is_empty()
        || !request_key.bytes().all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.'))
    { return Err(bad()); }
    let constraints = serde_json::json!([
        "positive_fresh_project_registration", "exact_single_project_and_scope",
        "explicit_human_approved_initial_local_advisory_instruction", "separate_validation_permission_and_commit",
        "deferral_has_no_configuration_or_owner_rule_qualification", "positive_complete_never_established_lifecycle",
        "stable_registration_distinct_from_consumed_initial_choice", "immutable_original_request_instruction_deferral_and_outcome",
        "exact_terminal_closed_current_relation", "historical_replay_never_restores_current_relation",
        "same_identity_later_first_establishment_requires_exact_current_deferred_predecessor_and_new_human_approved_terms",
        "unsupported_transitions_refuse", "no_scope_to_evaluation_or_action_authority"
    ]);
    let rule_content = serde_json::json!({
        "owner":"handbook", "name":"handbook.admission-establishment.standalone", "version":"2.0.0",
        "candidate_kind":null, "candidate_schema_ref":null,
        "required_authored_sources":[], "approval_sources":[], "exception_sources":[], "constraints":constraints,
    });
    let rule_fingerprint = fingerprint("handbook.project-admission-governance.shipped-rule@2.0.0", &rule_content)?;
    let mut exact_rule = rule_content.clone();
    exact_rule.as_object_mut().ok_or_else(bad)?.insert(
        "binding".into(),
        serde_json::json!({"ref":"handbook.admission-establishment.standalone@2.0.0","fingerprint":rule_fingerprint}),
    );
    if validated.get("establishment_rule") != Some(&exact_rule) { return Err(bad()); }
    let request_identity = fingerprint(
        "handbook.project-admission-governance.request@2.0.0",
        &serde_json::json!({"project":project,"request":request}),
    )?;
    let registration = fingerprint("handbook.admission-registration@2.0.0", &serde_json::json!([
        project.get("repository_identity_fingerprint").ok_or_else(bad)?,
        project.get("retained_root").ok_or_else(bad)?,
        project.get("physical_root_fingerprint").ok_or_else(bad)?,
    ]))?;
    let deferral = fingerprint("handbook.admission-deferral@2.0.0", &serde_json::json!([
        locator, validated, "never_established", "initial_choice_consumed"
    ]))?;
    let outcome_fp = fingerprint("handbook.admission-governance-outcome@2.0.0", &serde_json::json!([
        locator, validated, deferral, "validated", "advisory_permission_accepted", "deferral_committed"
    ]))?;
    let relation_fp = fingerprint("handbook.admission-governance-current-relation@2.0.0", &serde_json::json!([
        project, SCOPE, deferral, outcome_fp, "deferred_never_established"
    ]))?;
    if project.get("repository_identity_fingerprint").and_then(serde_json::Value::as_str) != Some(claim.proposed_repository_identity())
        || project.get("physical_root_fingerprint").and_then(serde_json::Value::as_str) != Some(claim.physical_root_fingerprint())
        || project.get("retained_root").and_then(serde_json::Value::as_str) != Some(capture.root_path.as_str())
        || project.get("registration_identity").and_then(serde_json::Value::as_str) != Some(registration.as_str())
        || locator.get("request_fingerprint").and_then(serde_json::Value::as_str) != Some(claim.request_digest())
        || request_identity != claim.request_digest()
        || request.get("request_key").and_then(serde_json::Value::as_str) != Some(request_key)
        || request.get("scope").and_then(serde_json::Value::as_str) != Some(SCOPE)
        || instruction.get("human_approval_asserted") != Some(&serde_json::Value::Bool(true))
        || validated.get("input") != Some(request)
        || validated.get("instruction_fingerprint").and_then(serde_json::Value::as_str)
            != Some(fingerprint("handbook.project-admission-governance.local-advisory-instruction@2.0.0", instruction)?.as_str())
        || relation.get("project") != Some(project)
        || relation.get("registration_identity").and_then(serde_json::Value::as_str) != Some(registration.as_str())
        || relation.get("deferral_fingerprint").and_then(serde_json::Value::as_str) != Some(deferral.as_str())
        || relation.get("outcome_fingerprint").and_then(serde_json::Value::as_str) != Some(outcome_fp.as_str())
        || relation.get("relation_fingerprint").and_then(serde_json::Value::as_str) != Some(relation_fp.as_str())
        || outcome.get("outcome_fingerprint").and_then(serde_json::Value::as_str) != Some(outcome_fp.as_str())
        || relation.get("establishment_rule") != validated.get("establishment_rule")
    {
        return Err(bad());
    }
    let facts: Vec<Vec<u8>> = serde_json::from_value(original.get("facts").cloned().ok_or_else(bad)?).map_err(codec)?;
    if facts.len() != 9 || facts[4] != outcome_bytes || facts[6] != relation_bytes {
        return Err(bad());
    }
    let instruction_fp = field_str(validated, "instruction_fingerprint")?;
    let expected_stages = vec![
        encode(&exact_rule)?,
        encode(&(instruction, instruction_fp))?,
        encode(&("handbook.admission-deferral.v2", &locator, &deferral, "never_established"))?,
        outcome_bytes.clone(),
        encode(&("consumed_initial_choice", &registration, claim.request_digest(), &outcome_fp))?,
        relation_bytes.clone(),
    ];
    let registration_bytes = encode(&("handbook.admission-registration.v2", project, "stable_fresh_project_registration"))?;
    let stage_fingerprints = expected_stages.iter()
        .map(|value| fingerprint("handbook.admission-stage-bytes@2.0.0", value))
        .collect::<Result<Vec<_>, _>>()?;
    let expected_intent = encode(&(
        "handbook.admission-intent.v2", claim.request_digest(),
        fingerprint("handbook.admission-registration-bytes@2.0.0", &registration_bytes)?,
        stage_fingerprints,
    ))?;
    let expected_decision = encode(&(
        "handbook.admission-commit-decision.v2",
        fingerprint("handbook.admission-intent-bytes@2.0.0", &expected_intent)?,
    ))?;
    let expected_terminal = encode(&(
        "handbook.admission-terminal.v2", &outcome_fp,
        fingerprint("handbook.admission-decision-bytes@2.0.0", &expected_decision)?,
        "exact_initial_current_relation_published",
    ))?;
    let condition = "verified_predecision_noncommit";
    let expected_noncommit = encode(&serde_json::json!({
        "locator":locator, "condition":condition, "deferral_committed":false,
        "outcome_fingerprint":fingerprint("handbook.admission-noncommit@2.0.0", &(&locator, condition))?,
    }))?;
    if facts[0] != registration_bytes || facts[1..7] != expected_stages
        || facts[7] != expected_intent || facts[8] != expected_decision
        || terminal_bytes != expected_terminal || field_bytes(&original, "noncommit")? != expected_noncommit
    { return Err(bad()); }

    let proof = &capture.evidence;
    object(proof, &["reference","outcome","evidence"])?;
    let reference: file_native::EvidenceReference = serde_json::from_value(
        proof.get("reference").cloned().ok_or_else(bad)?).map_err(codec)?;
    let native_outcome: file_native::OriginalOutcome = serde_json::from_value(proof.get("outcome").cloned().ok_or_else(bad)?).map_err(codec)?;
    let evidence = proof.get("evidence").ok_or_else(bad)?;
    object(evidence, &["scope","binding","previous"])?;
    let binding: file_native::EvidenceBinding = serde_json::from_value(
        evidence.get("binding").cloned().ok_or_else(bad)?).map_err(codec)?;
    let native_key = format!("governance/original/{id}");
    let valid_token = |value: &str| value.len() == 64
        && value.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b));
    if native_outcome.disposition != file_native::OriginalDisposition::Selected
        || native_outcome.value != outcome_bytes
        || reference.key != native_key || binding.key != native_key
        || binding.fingerprint != reference.fingerprint
        || field_str(evidence, "scope")? != claim.claim_fingerprint()
        || evidence.get("previous") != Some(&serde_json::Value::Null)
        || binding.format != "file-native@1" || binding.namespace != "handbook.project-governance@1"
        || binding.store != store || binding.expected != binding.predecessor
        || binding.expected.sequence != 0 || binding.candidate != binding.resolving
        || binding.candidate.store != store || binding.candidate.sequence != 1
        || native_outcome.sequence != binding.candidate.sequence
        || capture.version.sequence < native_outcome.sequence
        || !valid_token(&binding.predecessor_token) || !valid_token(&binding.candidate.token)
        || !valid_token(&reference.fingerprint)
    {
        return Err(bad());
    }
    let expected = &binding.expected;
    let mut tx = file_native::Transaction::new(expected.clone(), &native_key, outcome_bytes.clone());
    tx.set_evidence(claim.claim_fingerprint(), capture.original.as_bytes().to_vec()).map_err(codec)?;
    tx.put("governance/root", capture.root.as_bytes().to_vec()).map_err(codec)?;
    tx.put(format!("governance/request/{id}"), capture.original.as_bytes().to_vec()).map_err(codec)?;
    tx.put("governance/current/contracts.evidence-freshness-policy/root", encode(current)?).map_err(codec)?;
    tx.put(format!("governance/history/{:016x}", expected.sequence.checked_add(1).ok_or_else(bad)?),
        encode(&(id, operation, &terminal_bytes))?).map_err(codec)?;
    let native_fingerprint = DefinitionFingerprint::from_bytes(&serde_json::to_vec(&tx).map_err(codec)?).to_string();
    if native_fingerprint.strip_prefix("sha256:") != Some(reference.fingerprint.as_str()) {
        return Err(bad());
    }
    if let Some((transition_bytes, transition_evidence)) = &capture.transition {
        return validate_transition(transition_bytes, transition_evidence, &selected_current,
            &relation_bytes, &relation, &exact_rule, capture, claim);
    }
    Ok(relation_bytes)
}
