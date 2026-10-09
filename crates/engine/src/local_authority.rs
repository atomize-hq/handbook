//! Bounded local advisory operating authority. Portable locators resolve owner
//! history; every consuming check reacquires the native current authority guards.
pub use crate::local_charter::LocalAuthorityRefusalV1;
use crate::local_charter::{self, LocalCharterLocatorV1};
use crate::{
    DefinitionFingerprint, RepositoryInitializationGuardV1, RepositoryReadV1,
    RetainedRepositoryRootV1,
};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

pub mod local_check;
pub mod local_evaluation;
pub(crate) mod work_registration;

#[cfg(test)]
mod fixed_profile_tests;

pub const LOCAL_OPERATING_RULE_V1: &str = "handbook.operating.local-initial@1.0.0";
const BASE: &str = ".handbook/local-operating/v1";
const HEAD: &str = ".handbook/local-operating/v1/current.json";
const SCOPE_CURRENT: &str = ".handbook/admission-governance/v2/scope-current.json";
const MAX: usize = 1024 * 1024;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalOperationClassV1 {
    LocalValidation,
    OwnerWrite,
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalDecisionClassV1 {
    PathExpansion,
    HorizonExpansion,
    CharterReplacement,
    TermsReplacement,
    InputCorrection,
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalBindingV1 {
    pub reference: String,
    pub fingerprint: String,
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalEnvelopeCeilingV1 {
    pub active_level_id: String,
    pub dimensions: [String; 6],
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalOperatingTermsV1 {
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
    pub ceiling: LocalEnvelopeCeilingV1,
    pub constraint_inputs: Vec<LocalBindingV1>,
    pub allowed_paths: Vec<String>,
    pub allowed_input_paths: Vec<String>,
    pub operation_classes: Vec<LocalOperationClassV1>,
    pub reserved_decisions: Vec<LocalDecisionClassV1>,
    pub delegated_decisions: Vec<LocalDecisionClassV1>,
    pub revocation_principal: String,
}
impl LocalOperatingTermsV1 {
    pub fn fingerprint_approved_content(&self) -> Result<String, LocalAuthorityRefusalV1> {
        let mut value = serde_json::to_value(self).map_err(codec)?;
        let object = value
            .as_object_mut()
            .ok_or_else(|| refuse("shape", "terms are not an object"))?;
        object.remove("human_approval_asserted");
        object.remove("approved_content_fingerprint");
        fingerprint("handbook.local-operating-approved-content@1.0.0", &value)
    }
    pub fn from_json_bytes(bytes: &[u8]) -> Result<Self, LocalAuthorityRefusalV1> {
        let terms: Self = decode(bytes)?;
        terms.validate()?;
        Ok(terms)
    }
    pub fn canonical_json_bytes(&self) -> Result<Vec<u8>, LocalAuthorityRefusalV1> {
        self.validate()?;
        encode(self)
    }
    fn validate(&self) -> Result<(), LocalAuthorityRefusalV1> {
        if self.mode != "local_advisory" || self.rule_ref != LOCAL_OPERATING_RULE_V1 {
            return Err(refuse(
                "unsupported_rule",
                "the exact local advisory operating rule is required",
            ));
        }
        if !token(&self.request_key)
            || !self.human_approval_asserted
            || self.approved_content_fingerprint != self.fingerprint_approved_content()?
        {
            return Err(refuse(
                "initial_approval",
                "exact supplied human approval of operating terms is required",
            ));
        }
        for text in [
            &self.instruction,
            &self.principal,
            &self.agent,
            &self.delegation,
            &self.approval_source_ref,
            &self.revocation_principal,
        ] {
            if text.trim().is_empty() || text.len() > 4096 || text.contains('\0') {
                return Err(refuse(
                    "incomplete_terms",
                    "bounded explicit provenance and revocation terms are required",
                ));
            }
        }
        if self.operation_classes.is_empty()
            || self.operation_classes.len() > 2
            || duplicate(&self.operation_classes)
            || self.allowed_paths.len() > 64
            || self.allowed_paths.windows(2).any(|p| p[0] >= p[1])
            || self.allowed_input_paths.is_empty()
            || self.allowed_input_paths.len() > 64
            || self.allowed_input_paths.windows(2).any(|p| p[0] >= p[1])
            || self.constraint_inputs.len() > 64
            || self
                .constraint_inputs
                .windows(2)
                .any(|p| p[0].reference >= p[1].reference)
            || self.reserved_decisions.len() > 5
            || self.delegated_decisions.len() > 5
            || duplicate(&self.reserved_decisions)
            || duplicate(&self.delegated_decisions)
            || self
                .reserved_decisions
                .iter()
                .any(|d| self.delegated_decisions.contains(d))
        {
            return Err(refuse(
                "closed_terms",
                "terms collections are conflicting, duplicate or exceed the closed bounds",
            ));
        }
        if self
            .operation_classes
            .contains(&LocalOperationClassV1::OwnerWrite)
            != !self.allowed_paths.is_empty()
        {
            return Err(refuse(
                "owner_write_reach",
                "owner writes require explicit paths and paths require the owner-write class",
            ));
        }
        for path in self.allowed_paths.iter().chain(&self.allowed_input_paths) {
            crate::ContextResolutionMutationRule::new(
                "local-bound",
                crate::ContextResolutionMutationEffect::Allow,
                "repository_path",
                path,
            )
            .map_err(kernel)?;
        }
        for binding in &self.constraint_inputs {
            exact(binding)?;
        }
        let profile = profile()?;
        if profile
            .context_resolution()
            .ranks(
                &self.ceiling.active_level_id,
                self.ceiling.dimensions.each_ref().map(String::as_str),
            )
            .is_none()
        {
            return Err(refuse(
                "ceiling",
                "ceiling must name exact values in the retained shipped stack",
            ));
        }
        encode(self)?;
        Ok(())
    }
}
fn duplicate<T: PartialEq>(items: &[T]) -> bool {
    items
        .iter()
        .enumerate()
        .any(|(i, x)| items[..i].contains(x))
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalAuthorityLocatorV1 {
    pub charter: LocalCharterLocatorV1,
    pub terms: LocalOperatingTermsV1,
    pub request_fingerprint: String,
    pub expected_governance_relation_fingerprint: String,
}
impl LocalAuthorityLocatorV1 {
    pub fn canonical_json_bytes(&self) -> Result<Vec<u8>, LocalAuthorityRefusalV1> {
        self.validate()?;
        encode(self)
    }
    pub fn from_json_bytes(bytes: &[u8]) -> Result<Self, LocalAuthorityRefusalV1> {
        let locator: Self = decode(bytes)?;
        locator.validate()?;
        Ok(locator)
    }
    fn validate(&self) -> Result<(), LocalAuthorityRefusalV1> {
        self.terms.validate()?;
        self.charter.canonical_json_bytes()?;
        if !self
            .expected_governance_relation_fingerprint
            .strip_prefix("sha256:")
            .is_some_and(|digest| {
                digest.len() == 64
                    && digest
                        .bytes()
                        .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
            })
        {
            return Err(refuse(
                "governance_relation_identity",
                "complete prospective native deferred relation fingerprint is required",
            ));
        }
        if self.terms.principal != self.charter.request.human_approval.principal_claim
            || self.terms.agent != self.charter.request.human_approval.agent_claim
        {
            return Err(refuse("conflicting_actor_claims", "initial Charter and operating instruction carry conflicting principal or agent claims"));
        }
        if self.request_fingerprint
            != fingerprint(
                "handbook.local-operating-request@1.0.0",
                &(
                    &self.charter,
                    &self.terms,
                    &self.expected_governance_relation_fingerprint,
                ),
            )?
        {
            return Err(refuse(
                "request_identity",
                "original operating request identity changed",
            ));
        }
        Ok(())
    }
}
#[derive(Debug)]
pub struct PreparedLocalAuthorityV1 {
    locator: LocalAuthorityLocatorV1,
}
impl PreparedLocalAuthorityV1 {
    pub fn locator(&self) -> &LocalAuthorityLocatorV1 {
        &self.locator
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalAuthorityOutcomeV1 {
    pub locator: LocalAuthorityLocatorV1,
    pub outcome_fingerprint: String,
    pub governance_relation_fingerprint: String,
    pub validation_confirmed: bool,
    pub advisory_permission_accepted: bool,
    pub terms_committed: bool,
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalAuthorityDispositionV1 {
    Committed,
    Replayed,
    Refused,
    NonCommitted,
    Uncertain,
}
#[derive(Clone, Debug, Serialize)]
pub struct LocalAuthorityExecutionV1 {
    pub disposition: LocalAuthorityDispositionV1,
    pub outcome: Option<LocalAuthorityOutcomeV1>,
    pub refusal: Option<LocalAuthorityRefusalV1>,
    pub revocation_committed: bool,
    pub revocation_request: Option<LocalRevokeRequestV1>,
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct Head {
    schema: String,
    initial: String,
    revoked: Option<String>,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalRevokeRequestV1 {
    pub request_key: String,
    pub expected_current_fingerprint: String,
    pub principal: String,
    pub source_ref: String,
    pub instruction: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalUnresolvedRequestV1 {
    pub original_operation: String,
    pub original_basis: String,
    pub missing_condition: String,
    pub requested_choice: LocalDecisionClassV1,
    pub permitted_resolver_class: String,
    pub source_references: Vec<String>,
    pub original_candidate: serde_json::Value,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct NativeLocalAuthorityServiceV1;
impl NativeLocalAuthorityServiceV1 {
    pub fn prepare(
        repo: impl AsRef<Path>,
        charter: &LocalCharterLocatorV1,
        terms: LocalOperatingTermsV1,
        expected_governance_relation_fingerprint: String,
    ) -> Result<PreparedLocalAuthorityV1, LocalAuthorityRefusalV1> {
        let locator = LocalAuthorityLocatorV1 {
            request_fingerprint: fingerprint(
                "handbook.local-operating-request@1.0.0",
                &(charter, &terms, &expected_governance_relation_fingerprint),
            )?,
            charter: charter.clone(),
            terms,
            expected_governance_relation_fingerprint,
        };
        locator.validate()?;
        let root = RetainedRepositoryRootV1::open_exact(repo.as_ref()).map_err(fs_error)?;
        if root
            .governance_physical_root_fingerprint_v1()
            .map_err(fs_error)?
            != charter
                .binding
                .initialization_claim
                .physical_root_fingerprint()
        {
            return Err(refuse(
                "wrong_root",
                "physical root differs from the original registration",
            ));
        }
        if root
            .read_local_owner_optional_bounded(HEAD, MAX)
            .map_err(fs_error)?
            != RepositoryReadV1::Absent
            || root
                .read_local_owner_optional_bounded(
                    ".handbook/state/locks/local-operating.lock",
                    MAX,
                )
                .map_err(fs_error)?
                != RepositoryReadV1::Absent
        {
            return Err(refuse("already_selected", "fresh initial operating authority only; retained owner footprint requires original request recovery"));
        }
        Ok(PreparedLocalAuthorityV1 { locator })
    }
    pub fn establish(
        repo: impl AsRef<Path>,
        prepared: PreparedLocalAuthorityV1,
    ) -> LocalAuthorityExecutionV1 {
        execute(repo.as_ref(), &prepared.locator, false)
    }
    #[cfg(any(test, feature = "test-support"))]
    pub fn establish_with_interruption_for_test(
        repo: impl AsRef<Path>,
        prepared: PreparedLocalAuthorityV1,
        interrupt: impl FnOnce(),
    ) -> LocalAuthorityExecutionV1 {
        execute_with_interrupt(repo.as_ref(), &prepared.locator, false, interrupt)
    }
    pub fn resume_request(
        repo: impl AsRef<Path>,
        locator: &LocalAuthorityLocatorV1,
    ) -> LocalAuthorityExecutionV1 {
        execute(repo.as_ref(), locator, true)
    }
    pub fn resolve_request(
        repo: impl AsRef<Path>,
        locator: &LocalAuthorityLocatorV1,
    ) -> LocalAuthorityExecutionV1 {
        let result = (|| {
            locator.validate()?;
            let guard = OperatingGuard::acquire(repo.as_ref(), locator, false)?;
            read_history(guard.root(), locator)
        })();
        execution(result, LocalAuthorityDispositionV1::Replayed)
    }
    pub fn resolve_current(
        repo: impl AsRef<Path>,
        locator: &LocalAuthorityLocatorV1,
    ) -> Result<LocalAuthorityOutcomeV1, LocalAuthorityRefusalV1> {
        LocalCurrentAuthorityWitnessV1 {
            repo: repo.as_ref().to_owned(),
            locator: locator.clone(),
        }
        .with_current(|_| Ok(()))?;
        let guard = OperatingGuard::acquire(repo.as_ref(), locator, false)?;
        read_current(guard.root(), locator)
    }
    pub fn admit_envelope(
        repo: impl AsRef<Path>,
        locator: &LocalAuthorityLocatorV1,
        input: crate::ContextResolutionEnvelopeInput,
    ) -> Result<crate::ContextResolutionEnvelope, LocalAuthorityRefusalV1> {
        let witness = LocalCurrentAuthorityWitnessV1 {
            repo: repo.as_ref().to_owned(),
            locator: locator.clone(),
        };
        let candidate = serde_json::json!({"binding":input.candidate_binding(), "input": &input});
        witness
            .with_current(|_| {
                crate::context_resolution::admit_local_envelope(input, witness.clone())
            })
            .map_err(|mut error| {
                if error.unresolved_request.is_none() {
                    // A failed currentness check cannot certify the portable terms'
                    // claimed resolver. Preserve the request without granting a role.
                    error.unresolved_request = Some(Box::new(unresolved_request(
                        locator,
                        "admit_local_envelope",
                        &error.detail,
                        LocalDecisionClassV1::InputCorrection,
                        "missing_authority",
                        candidate,
                    )));
                }
                error
            })
    }
    /// Request context only: this helper certifies no portable term or resolver.
    /// Used when an outer owner refuses before the native current guard is reached.
    pub fn missing_authority_context(
        locator: &LocalAuthorityLocatorV1,
        input: &crate::ContextResolutionEnvelopeInput,
        condition: &str,
    ) -> LocalUnresolvedRequestV1 {
        unresolved_request(
            locator,
            "admit_local_envelope",
            condition,
            LocalDecisionClassV1::InputCorrection,
            "missing_authority",
            serde_json::json!({"binding":input.candidate_binding(),"input":input}),
        )
    }
    pub fn revoke(
        repo: impl AsRef<Path>,
        locator: &LocalAuthorityLocatorV1,
        request: LocalRevokeRequestV1,
    ) -> LocalAuthorityExecutionV1 {
        let mut effects = false;
        let mut replayed = false;
        let result = (|| {
            locator.validate()?;
            if !token(&request.request_key)
                || request.principal != locator.terms.revocation_principal
                || request.source_ref.trim().is_empty()
                || request.source_ref.len() > 4096
                || request.instruction.trim().is_empty()
                || request.instruction.len() > 4096
            {
                return Err(refuse("revocation_authority", "exact initially declared revocation principal, source and instruction required"));
            }
            let guard = OperatingGuard::acquire(repo.as_ref(), locator, false)?;
            let root = guard.root();
            let outcome = read_history(root, locator)?;
            if request.expected_current_fingerprint != outcome.outcome_fingerprint {
                return Err(refuse(
                    "revocation_predecessor",
                    "revocation does not cite the exact active operating predecessor",
                ));
            }
            let revoke_ref = format!("{BASE}/revoke.json");
            let request_bytes = encode(&request)?;
            match root
                .read_local_owner_optional_bounded(&revoke_ref, MAX)
                .map_err(fs_error)?
            {
                RepositoryReadV1::Found(bytes) if bytes == request_bytes => {
                    replayed = true;
                }
                RepositoryReadV1::Found(_) => {
                    return Err(refuse(
                        "revocation_conflict",
                        "another exact revocation was already retained",
                    ))
                }
                RepositoryReadV1::Absent => {
                    read_current(root, locator)?;
                    effects = true;
                    root.write_new_durable(&revoke_ref, &request_bytes)
                        .map_err(fs_error)?;
                }
            }
            let old = encode(&Head {
                schema: LOCAL_OPERATING_RULE_V1.into(),
                initial: outcome.outcome_fingerprint.clone(),
                revoked: None,
            })?;
            let new = encode(&Head {
                schema: LOCAL_OPERATING_RULE_V1.into(),
                initial: outcome.outcome_fingerprint.clone(),
                revoked: Some(fingerprint("handbook.local-revocation@1.0.0", &request)?),
            })?;
            let stage = format!("{BASE}/revoked-head.json");
            let displaced = format!("{BASE}/displaced-active-head.json");
            let current = root
                .read_local_owner_optional_bounded(HEAD, MAX)
                .map_err(fs_error)?;
            if current == RepositoryReadV1::Found(new.clone()) {
                return Ok(outcome);
            }
            write_exact(root, &stage, &new)?;
            if current == RepositoryReadV1::Found(old.clone()) {
                root.replace_exact_durable(HEAD, &old, &stage, &displaced)
                    .map_err(fs_error)?;
            } else if current == RepositoryReadV1::Absent
                && root
                    .read_local_owner_optional_bounded(&displaced, MAX)
                    .map_err(fs_error)?
                    == RepositoryReadV1::Found(old)
            {
                root.rename_noreplace_durable(&stage, HEAD)
                    .map_err(fs_error)?;
            } else {
                return Err(refuse(
                    "revocation_state",
                    "revocation head cannot be recovered from the retained original relation",
                ));
            }
            if root
                .read_local_owner_optional_bounded(HEAD, MAX)
                .map_err(fs_error)?
                != RepositoryReadV1::Found(new)
            {
                return Err(refuse(
                    "uncertain",
                    "revoked relation publication is unverified",
                ));
            }
            Ok(outcome)
        })();
        let success = result.is_ok();
        let mut execution = execution(
            result,
            if replayed {
                LocalAuthorityDispositionV1::Replayed
            } else {
                LocalAuthorityDispositionV1::Committed
            },
        );
        execution.revocation_committed = success;
        execution.revocation_request = Some(request);
        if !success && effects {
            execution.disposition = LocalAuthorityDispositionV1::Uncertain;
        }
        execution
    }
}

fn execute(
    repo: &Path,
    locator: &LocalAuthorityLocatorV1,
    resume: bool,
) -> LocalAuthorityExecutionV1 {
    execute_with_interrupt(repo, locator, resume, || {})
}
fn execute_with_interrupt(
    repo: &Path,
    locator: &LocalAuthorityLocatorV1,
    resume: bool,
    interrupt: impl FnOnce(),
) -> LocalAuthorityExecutionV1 {
    let mut effects = false;
    let mut replayed = false;
    let result = (|| {
        locator.validate()?;
        let guard = OperatingGuard::acquire(repo, locator, !resume)?;
        let root = guard.root();
        let request_ref = request_path(locator, "request.json");
        let terminal = request_path(locator, "terminal.json");
        let pending_terminal = request_path(locator, "pending-terminal.json");
        let request_bytes = encode(locator)?;
        if root
            .read_local_owner_optional_bounded(&terminal, MAX)
            .map_err(fs_error)?
            != RepositoryReadV1::Absent
        {
            replayed = true;
            return read_history(root, locator);
        }
        local_charter::resolve_current_retained(root, &locator.charter)?;
        let relation = require_initial_relation(root, locator)?;
        match root
            .read_local_owner_optional_bounded(&request_ref, MAX)
            .map_err(fs_error)?
        {
            RepositoryReadV1::Found(bytes) if bytes == request_bytes => {
                if !resume {
                    return Err(refuse(
                        "original_request_pending",
                        "resume the exact retained original request",
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
                    "no retained owner intent; locator alone cannot create authority",
                ))
            }
            RepositoryReadV1::Absent => {
                if guard.operating_already_started
                    || root
                        .read_local_owner_optional_bounded(HEAD, MAX)
                        .map_err(fs_error)?
                        != RepositoryReadV1::Absent
                    || root
                        .list_directory_bounded(BASE, 128)
                        .is_ok_and(|v| !v.is_empty())
                {
                    return Err(refuse(
                        "already_selected",
                        "existing operating owner footprint cannot be initialized again",
                    ));
                }
                root.create_directories(&format!("{BASE}/requests/{}", locator.terms.request_key))
                    .map_err(fs_error)?;
                effects = true;
                root.write_new_durable(&request_ref, &request_bytes)
                    .map_err(fs_error)?;
            }
        }
        effects = true;
        let retained_outcome = root
            .read_local_owner_optional_bounded(&request_path(locator, "outcome.json"), MAX)
            .map_err(fs_error)?;
        if retained_outcome != RepositoryReadV1::Absent
            && root
                .read_local_owner_optional_bounded(&pending_terminal, MAX)
                .map_err(fs_error)?
                == RepositoryReadV1::Absent
        {
            return Err(refuse("required_terminal_state_missing", "original outcome has no retained pending or final terminal; state loss cannot be repaired by replay"));
        }
        if root
            .read_local_owner_optional_bounded(&format!("{BASE}/revoke.json"), MAX)
            .map_err(fs_error)?
            != RepositoryReadV1::Absent
        {
            return Err(refuse(
                "revoked",
                "initial request cannot restore revoked authority",
            ));
        }
        let mut outcome = LocalAuthorityOutcomeV1 {
            locator: locator.clone(),
            outcome_fingerprint: String::new(),
            governance_relation_fingerprint: DefinitionFingerprint::from_bytes(&relation)
                .to_string(),
            validation_confirmed: true,
            advisory_permission_accepted: true,
            terms_committed: true,
        };
        outcome.outcome_fingerprint = outcome_identity(&outcome)?;
        let outcome_bytes = encode(&outcome)?;
        write_exact(root, &request_path(locator, "outcome.json"), &outcome_bytes)?;
        write_exact(root, &pending_terminal, &outcome_bytes)?;
        interrupt();
        let head = encode(&Head {
            schema: LOCAL_OPERATING_RULE_V1.into(),
            initial: outcome.outcome_fingerprint.clone(),
            revoked: None,
        })?;
        write_exact(root, HEAD, &head)?;
        guard.validate()?;
        root.rename_noreplace_durable(&pending_terminal, &terminal)
            .map_err(fs_error)?;
        read_current(root, locator)
    })();
    match result {
        Ok(outcome) => LocalAuthorityExecutionV1 {
            disposition: if replayed {
                LocalAuthorityDispositionV1::Replayed
            } else {
                LocalAuthorityDispositionV1::Committed
            },
            outcome: Some(outcome),
            refusal: None,
            revocation_committed: false,
            revocation_request: None,
        },
        Err(refusal) => LocalAuthorityExecutionV1 {
            disposition: if effects {
                LocalAuthorityDispositionV1::Uncertain
            } else {
                LocalAuthorityDispositionV1::Refused
            },
            outcome: None,
            refusal: Some(refusal),
            revocation_committed: false,
            revocation_request: None,
        },
    }
}
fn execution(
    result: Result<LocalAuthorityOutcomeV1, LocalAuthorityRefusalV1>,
    disposition: LocalAuthorityDispositionV1,
) -> LocalAuthorityExecutionV1 {
    match result {
        Ok(outcome) => LocalAuthorityExecutionV1 {
            disposition,
            outcome: Some(outcome),
            refusal: None,
            revocation_committed: false,
            revocation_request: None,
        },
        Err(refusal) => LocalAuthorityExecutionV1 {
            disposition: LocalAuthorityDispositionV1::Refused,
            outcome: None,
            refusal: Some(refusal),
            revocation_committed: false,
            revocation_request: None,
        },
    }
}
fn read_history(
    root: &RetainedRepositoryRootV1,
    locator: &LocalAuthorityLocatorV1,
) -> Result<LocalAuthorityOutcomeV1, LocalAuthorityRefusalV1> {
    if found(root, &request_path(locator, "request.json"))? != encode(locator)? {
        return Err(refuse(
            "request_conflict",
            "retained original request differs",
        ));
    }
    let bytes = found(root, &request_path(locator, "outcome.json"))?;
    if found(root, &request_path(locator, "terminal.json"))? != bytes {
        return Err(refuse(
            "original_request_pending",
            "original request has no complete terminal",
        ));
    }
    let outcome: LocalAuthorityOutcomeV1 = decode(&bytes)?;
    if outcome.locator != *locator
        || outcome.governance_relation_fingerprint
            != locator.expected_governance_relation_fingerprint
        || !outcome.validation_confirmed
        || !outcome.advisory_permission_accepted
        || !outcome.terms_committed
        || outcome.outcome_fingerprint != outcome_identity(&outcome)?
        || encode(&outcome)? != bytes
    {
        return Err(refuse(
            "owner_history",
            "immutable owner outcome is not exact",
        ));
    }
    Ok(outcome)
}
fn read_current(
    root: &RetainedRepositoryRootV1,
    locator: &LocalAuthorityLocatorV1,
) -> Result<LocalAuthorityOutcomeV1, LocalAuthorityRefusalV1> {
    if root
        .list_directory_bounded(local_check::LOCAL_CHECK_ROOT_V1, 128)
        .is_ok()
    {
        return Err(refuse("successor_terms_selected", "local-check successor footprint requires its exact current-use path; initial terms remain history"));
    }
    let outcome = read_history(root, locator)?;
    let head: Head = decode(&found(root, HEAD)?)?;
    if head.schema != LOCAL_OPERATING_RULE_V1
        || head.initial != outcome.outcome_fingerprint
        || head.revoked.is_some()
        || root
            .read_local_owner_optional_bounded(&format!("{BASE}/revoke.json"), MAX)
            .map_err(fs_error)?
            != RepositoryReadV1::Absent
    {
        return Err(refuse(
            "revoked_or_stale_head",
            "operating authority is revoked or its exact head changed",
        ));
    }
    local_charter::resolve_current_retained(root, &locator.charter)?;
    if DefinitionFingerprint::from_bytes(&found(root, SCOPE_CURRENT)?).as_str()
        != outcome.governance_relation_fingerprint
    {
        return Err(refuse("governance_currentness", "current admission relation differs from the retained initial basis; reprepare dependent use"));
    }
    Ok(outcome)
}
fn outcome_identity(outcome: &LocalAuthorityOutcomeV1) -> Result<String, LocalAuthorityRefusalV1> {
    fingerprint(
        "handbook.local-operating-outcome@1.0.0",
        &(
            &outcome.locator,
            &outcome.governance_relation_fingerprint,
            outcome.validation_confirmed,
            outcome.advisory_permission_accepted,
            outcome.terms_committed,
        ),
    )
}
fn require_initial_relation(
    root: &RetainedRepositoryRootV1,
    locator: &LocalAuthorityLocatorV1,
) -> Result<Vec<u8>, LocalAuthorityRefusalV1> {
    let relation = found(root, SCOPE_CURRENT)?;
    if DefinitionFingerprint::from_bytes(&relation).as_str()
        != locator.expected_governance_relation_fingerprint
    {
        return Err(refuse("governance_relation_substituted", "complete current native relation differs from the exact original prospective deferred basis; reprepare dependent use"));
    }
    Ok(relation)
}
fn request_path(locator: &LocalAuthorityLocatorV1, name: &str) -> String {
    format!("{BASE}/requests/{}/{name}", locator.terms.request_key)
}

struct OperatingGuard {
    locks: Vec<crate::RepositoryExclusiveLockV1>,
    initialization: RepositoryInitializationGuardV1,
    operating_already_started: bool,
}
impl Drop for OperatingGuard {
    fn drop(&mut self) {
        while self.locks.pop().is_some() {}
    }
}
impl OperatingGuard {
    fn acquire(
        repo: &Path,
        locator: &LocalAuthorityLocatorV1,
        allow_initial_lock: bool,
    ) -> Result<Self, LocalAuthorityRefusalV1> {
        locator.validate()?;
        let initialization =
            RepositoryInitializationGuardV1::acquire_existing_v1(repo).map_err(fs_error)?;
        local_charter::validate_binding_guarded(&initialization, &locator.charter.binding)?;
        let root = initialization.root();
        let operating_already_started = root
            .read_local_owner_optional_bounded(".handbook/state/locks/local-operating.lock", MAX)
            .map_err(fs_error)?
            != RepositoryReadV1::Absent;
        let operating_started = root.list_directory_bounded(BASE, 128).is_ok();
        if !operating_already_started && !operating_started {
            if !allow_initial_lock {
                return Err(refuse("required_lock_state_missing", "original operating lock is absent; read or recovery cannot create an owner footprint"));
            }
            require_initial_relation(root, locator)?;
        }
        let mut locks = Vec::new();
        for path in [
            ".handbook/state/locks/promotion.lock",
            ".handbook/state/locks/registry.lock",
            ".handbook/state/locks/lifecycle.lock",
            ".handbook/state/locks/local-operating.lock",
        ] {
            if (operating_started || !path.ends_with("local-operating.lock"))
                && root
                    .read_local_owner_optional_bounded(path, MAX)
                    .map_err(fs_error)?
                    != RepositoryReadV1::Found(Vec::new())
            {
                return Err(refuse(
                    "required_lock_state_missing",
                    "required owner lock state was lost; no reinitialization is permitted",
                ));
            }
            locks.push(
                if allow_initial_lock
                    && !operating_started
                    && path.ends_with("local-operating.lock")
                {
                    root.acquire_exclusive_lock(path).map_err(fs_error)?
                } else {
                    root.acquire_existing_exclusive_lock_v1(path)
                        .map_err(fs_error)?
                },
            );
        }
        Ok(Self {
            locks,
            initialization,
            operating_already_started,
        })
    }
    fn root(&self) -> &RetainedRepositoryRootV1 {
        self.initialization.root()
    }
    fn validate(&self) -> Result<(), LocalAuthorityRefusalV1> {
        self.initialization.validate().map_err(fs_error)?;
        for (lock, path) in self.locks.iter().zip([
            ".handbook/state/locks/promotion.lock",
            ".handbook/state/locks/registry.lock",
            ".handbook/state/locks/lifecycle.lock",
            ".handbook/state/locks/local-operating.lock",
        ]) {
            lock.validate_at(self.root(), path).map_err(fs_error)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub(crate) struct LocalCurrentAuthorityWitnessV1 {
    pub(crate) repo: PathBuf,
    pub(crate) locator: LocalAuthorityLocatorV1,
}
impl LocalCurrentAuthorityWitnessV1 {
    pub(crate) fn with_current<T>(
        &self,
        consume: impl FnOnce(&LocalOperatingTermsV1) -> Result<T, LocalAuthorityRefusalV1>,
    ) -> Result<T, LocalAuthorityRefusalV1> {
        let guard = OperatingGuard::acquire(&self.repo, &self.locator, false)?;
        read_current(guard.root(), &self.locator)?;
        let tree = guard
            .root()
            .retain_local_authority_tree_guard()
            .map_err(fs_error)?;
        guard.validate()?;
        let value = consume(&self.locator.terms)?;
        tree.validate().map_err(fs_error)?;
        guard.validate()?;
        read_current(guard.root(), &self.locator)?;
        Ok(value)
    }
}
pub fn resolved_local_operating_profile(
) -> Result<crate::ResolvedInstanceProfile, LocalAuthorityRefusalV1> {
    crate::resolve_profile_selection(
        Path::new(env!("CARGO_MANIFEST_DIR")),
        crate::profile_decision::local_operating_profile_request(),
    )
    .map_err(|_| refuse("profile", "exact shipped mechanics could not resolve"))
}
pub(crate) fn profile() -> Result<crate::ResolvedInstanceProfile, LocalAuthorityRefusalV1> {
    static PROFILE: OnceLock<Result<crate::ResolvedInstanceProfile, LocalAuthorityRefusalV1>> =
        OnceLock::new();
    PROFILE
        .get_or_init(resolve_fixed_operating_profile)
        .as_ref()
        .cloned()
        .map_err(Clone::clone)
}

fn resolve_fixed_operating_profile(
) -> Result<crate::ResolvedInstanceProfile, LocalAuthorityRefusalV1> {
    let request = crate::profile_decision::local_operating_profile_request();
    require_fixed_builtin_profile_request(&request)?;
    let selected = crate::resolve_profile_selection(Path::new(env!("CARGO_MANIFEST_DIR")), request)
        .map_err(|_| refuse("profile", "exact shipped mechanics could not resolve"))?;
    require_fixed_builtin_profile(&selected)?;
    Ok(selected)
}

// Only this package-owned closure may be retained for the process lifetime.
// No repository bytes, supplied content or authority observations enter it.
pub(crate) fn require_fixed_builtin_profile_request(
    request: &crate::ProfileSelectionRequest,
) -> Result<(), LocalAuthorityRefusalV1> {
    use crate::DefinitionSource;
    if request.selected_profile_ref.as_str() != "handbook.profile.shipped-root@1.2.0"
        || request.allowed_schema_roots != ["definitions/schemas"]
        || [
            &request.profile_sources,
            &request.stable_role_registry_sources,
            &request.schema_entry_sources,
            &request.artifact_kind_sources,
            &request.semantic_capability_sources,
            &request.semantic_validator_sources,
            &request.project_condition_sources,
            &request.vocabulary_sources,
            &request.context_resolution_sources,
            &request.context_resolution_policy_sources,
            &request.intake_definition_sources,
        ]
        .into_iter()
        .flatten()
        .any(|binding| !matches!(&binding.source, DefinitionSource::BuiltIn(reference) if reference == &binding.definition_ref))
    {
        return Err(refuse("profile", "exact shipped mechanics could not resolve"));
    }
    Ok(())
}

pub(crate) fn require_fixed_builtin_profile(
    profile: &crate::ResolvedInstanceProfile,
) -> Result<(), LocalAuthorityRefusalV1> {
    if profile.exact_ref().as_str() != "handbook.profile.shipped-root@1.2.0"
        || profile.selected_profile_definition_fingerprint().as_str()
            != "sha256:40c5fdb8a6ea42cf0f5f2c5cac8306ec7ad3a238c341653947f85abc93d72c40"
    {
        return Err(refuse(
            "profile",
            "exact shipped mechanics could not resolve",
        ));
    }
    Ok(())
}
fn exact(
    binding: &LocalBindingV1,
) -> Result<crate::ContextResolutionExactBinding, LocalAuthorityRefusalV1> {
    crate::ContextResolutionExactBinding::new(&binding.reference, &binding.fingerprint)
        .map_err(kernel)
}
fn token(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 128
        && value
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'.' | b'_' | b'-'))
}
pub(crate) fn refuse(code: &str, detail: &str) -> LocalAuthorityRefusalV1 {
    LocalAuthorityRefusalV1 {
        code: code.into(),
        detail: detail.into(),
        unresolved_request: None,
    }
}
pub(crate) fn admission_refusal(
    locator: &LocalAuthorityLocatorV1,
    candidate: serde_json::Value,
    error: crate::ContextResolutionKernelError,
    choice: LocalDecisionClassV1,
) -> LocalAuthorityRefusalV1 {
    let resolver = if locator.terms.reserved_decisions.contains(&choice) {
        "reserved_human_decision"
    } else if locator.terms.delegated_decisions.contains(&choice) {
        "delegated_project_agent_delivery_unimplemented"
    } else {
        "missing_authority"
    };
    let mut refusal = kernel(error);
    refusal.unresolved_request = Some(Box::new(unresolved_request(
        locator,
        "admit_local_envelope",
        &refusal.detail,
        choice,
        resolver,
        candidate,
    )));
    refusal
}
fn unresolved_request(
    locator: &LocalAuthorityLocatorV1,
    operation: &str,
    condition: &str,
    choice: LocalDecisionClassV1,
    resolver: &str,
    candidate: serde_json::Value,
) -> LocalUnresolvedRequestV1 {
    LocalUnresolvedRequestV1 {
        original_operation: operation.into(),
        original_basis: locator.request_fingerprint.clone(),
        missing_condition: condition.into(),
        requested_choice: choice,
        permitted_resolver_class: resolver.into(),
        source_references: vec![
            locator.terms.approval_source_ref.clone(),
            locator.charter.request_fingerprint.clone(),
            locator.charter.canonical_fingerprint.clone(),
            locator.charter.profile_binding.closure_fingerprint.clone(),
            locator.request_fingerprint.clone(),
            LOCAL_OPERATING_RULE_V1.into(),
        ],
        original_candidate: candidate,
    }
}
fn codec(_: impl std::fmt::Display) -> LocalAuthorityRefusalV1 {
    refuse(
        "closed_json",
        "record exceeds bounds or is not exact closed duplicate-safe JSON",
    )
}
fn kernel(error: crate::ContextResolutionKernelError) -> LocalAuthorityRefusalV1 {
    refuse("envelope_refused", error.detail())
}
fn fs_error(error: crate::RepositoryDurabilityErrorV1) -> LocalAuthorityRefusalV1 {
    refuse("retained_state", error.detail())
}
fn encode<T: Serialize>(record: &T) -> Result<Vec<u8>, LocalAuthorityRefusalV1> {
    let value = serde_json::to_value(record).map_err(codec)?;
    let bytes = crate::definition_identity::canonical_json_bytes(&value).map_err(codec)?;
    if bytes.len() > MAX {
        return Err(codec("bound"));
    }
    Ok(bytes)
}
fn decode<T: for<'de> Deserialize<'de>>(bytes: &[u8]) -> Result<T, LocalAuthorityRefusalV1> {
    if bytes.len() > MAX {
        return Err(codec("bound"));
    }
    serde_json::from_value(crate::parse_schema_json(bytes).map_err(codec)?).map_err(codec)
}
fn fingerprint<T: Serialize>(domain: &str, record: &T) -> Result<String, LocalAuthorityRefusalV1> {
    Ok(DefinitionFingerprint::from_bytes(&encode(
        &serde_json::json!({"domain":domain,"content":record}),
    )?)
    .to_string())
}
fn found(root: &RetainedRepositoryRootV1, path: &str) -> Result<Vec<u8>, LocalAuthorityRefusalV1> {
    match root
        .read_local_owner_optional_bounded(path, MAX)
        .map_err(fs_error)?
    {
        RepositoryReadV1::Found(bytes) => Ok(bytes),
        RepositoryReadV1::Absent => Err(refuse(
            "required_state_missing",
            "required retained authority state is absent; no bootstrap or reset is permitted",
        )),
    }
}
fn write_exact(
    root: &RetainedRepositoryRootV1,
    path: &str,
    bytes: &[u8],
) -> Result<(), LocalAuthorityRefusalV1> {
    match root
        .read_local_owner_optional_bounded(path, MAX)
        .map_err(fs_error)?
    {
        RepositoryReadV1::Absent => root.write_new_durable(path, bytes).map_err(fs_error),
        RepositoryReadV1::Found(existing) if existing == bytes => Ok(()),
        RepositoryReadV1::Found(_) => Err(refuse(
            "record_conflict",
            "retained record differs from original exact owner output",
        )),
    }
}
