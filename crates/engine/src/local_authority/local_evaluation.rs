//! Native local-advisory Evaluation Authorization. Portable projections are
//! untrusted input or historical data; only a live guarded admission permits use.
use super::local_check::{LocalCheckCurrentBasisV1, LocalCheckLocatorV1};
use super::*;
use crate::repository_durability::{
    RepositoryExactFilesCheckpointV1, RetainedRepositoryTreeGuardV1,
};

pub const LOCAL_EVALUATION_RULE_V1: &str = "handbook.operating.local-evaluation@1.0.0";
pub const LOCAL_EVALUATION_ROOT_V1: &str = ".handbook/local-operating/local-evaluation-v1";
pub const LOCAL_EVALUATION_PURPOSE_SCHEMA_V1: &str = "handbook.local-evaluation-purpose@1.0.0";
pub const LOCAL_EVALUATION_PLAN_SCHEMA_V1: &str = "handbook.local-evaluation-plan@1.0.0";
pub const LOCAL_EVALUATION_ROLE_V1: &str = "schema_conformance";
pub const LOCAL_EVALUATION_PRODUCER_V1: &str = "handbook.dock.json-schema@1.0.0";

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalEvaluationUseIntentV1 {
    DecisionSeeking,
    Diagnostic,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalEvaluationCheckpointV1 {
    Preparation,
    CollectionCommit,
    Launch,
    SourceAdmission,
    EvidenceAppend,
    FinalContext,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalEvaluationReachV1 {
    pub profile: LocalBindingV1,
    pub stack: LocalBindingV1,
    pub active_level_id: String,
    pub dimensions: [String; 6],
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalEvaluationSelectionV1 {
    pub selection_id: String,
    pub role_ref: String,
    pub producer_ref: String,
    pub configuration_fingerprint: String,
    pub capability_fingerprint: String,
    pub operating_ceiling_fingerprint: String,
    pub host_prerequisites_fingerprint: String,
    pub requested_reach: LocalEvaluationReachV1,
}
impl LocalEvaluationSelectionV1 {
    pub fn fingerprint(&self) -> Result<String, LocalAuthorityRefusalV1> {
        self.validate()?;
        fingerprint("handbook.local-evaluation-selection@1.0.0", self)
    }
    fn validate(&self) -> Result<(), LocalAuthorityRefusalV1> {
        if !token(&self.selection_id)
            || self.role_ref != LOCAL_EVALUATION_ROLE_V1
            || self.producer_ref != LOCAL_EVALUATION_PRODUCER_V1
        {
            return Err(refuse(
                "selection",
                "one exact supported native schema-conformance selection required",
            ));
        }
        for value in [
            &self.configuration_fingerprint,
            &self.capability_fingerprint,
            &self.operating_ceiling_fingerprint,
            &self.host_prerequisites_fingerprint,
        ] {
            check_fp(value)?;
        }
        self.requested_reach.validate()
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalEvaluationPlanV1 {
    pub schema_ref: String,
    pub requirements_fingerprint: String,
    pub selections: Vec<LocalEvaluationSelectionV1>,
    pub composition: String,
    pub max_attempts: u32,
}
impl LocalEvaluationPlanV1 {
    pub fn fingerprint(&self) -> Result<String, LocalAuthorityRefusalV1> {
        self.validate()?;
        fingerprint("handbook.local-evaluation-plan-identity@1.0.0", self)
    }
    pub fn validate(&self) -> Result<(), LocalAuthorityRefusalV1> {
        if self.schema_ref != LOCAL_EVALUATION_PLAN_SCHEMA_V1
            || self.selections.len() != 1
            || self.composition != "single"
            || self.max_attempts != 1
        {
            return Err(refuse(
                "plan",
                "complete single-selection plan with at most one attempt required",
            ));
        }
        check_fp(&self.requirements_fingerprint)?;
        self.selections[0].validate()
    }
}

/// Complete semantic projection supplied by Contracts after durable formation.
/// It grants no authority and Engine independently checks every closed relation.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalEvaluationPurposeV1 {
    pub schema_ref: String,
    pub owner_identity: String,
    pub project_identity: String,
    pub occurrence_identity: String,
    pub requirements_fingerprint: String,
    pub requirements_source: String,
    pub contract: LocalBindingV1,
    pub subject_ref: String,
    pub subject_fingerprint: String,
    pub scope_fingerprint: String,
    pub objective: String,
    pub permitted_use: String,
    pub use_intent: LocalEvaluationUseIntentV1,
    pub plan_fingerprint: String,
    pub formation_policy_fingerprint: String,
    pub evaluation_rule_fingerprint: String,
    pub time_rules_fingerprint: String,
    pub retention_until_utc: String,
    pub content_fingerprint: String,
    pub formed_at_utc: String,
}
impl LocalEvaluationPurposeV1 {
    pub fn fingerprint_semantic_content(&self) -> Result<String, LocalAuthorityRefusalV1> {
        let mut value = serde_json::to_value(self).map_err(codec)?;
        let object = value
            .as_object_mut()
            .ok_or_else(|| refuse("purpose", "purpose object required"))?;
        for name in [
            "occurrence_identity",
            "content_fingerprint",
            "formed_at_utc",
        ] {
            object.remove(name);
        }
        fingerprint("handbook.local-evaluation-purpose-content@1.0.0", &value)
    }
    pub fn fingerprint(&self) -> Result<String, LocalAuthorityRefusalV1> {
        self.validate()?;
        fingerprint("handbook.local-evaluation-purpose-identity@1.0.0", self)
    }
    pub fn validate(&self) -> Result<(), LocalAuthorityRefusalV1> {
        if self.schema_ref != LOCAL_EVALUATION_PURPOSE_SCHEMA_V1
            || self.requirements_source != "direct_contract"
            || !["diagnostic_only", "decision_seeking_eligible"]
                .contains(&self.permitted_use.as_str())
            || (self.permitted_use == "diagnostic_only"
                && self.use_intent == LocalEvaluationUseIntentV1::DecisionSeeking)
            || self.content_fingerprint != self.fingerprint_semantic_content()?
        {
            return Err(refuse("purpose", "exact direct requirements, compatible intent and complete Purpose content required"));
        }
        for value in [
            &self.owner_identity,
            &self.project_identity,
            &self.subject_ref,
            &self.objective,
        ] {
            bounded_text(value)?;
        }
        for value in [
            &self.occurrence_identity,
            &self.requirements_fingerprint,
            &self.subject_fingerprint,
            &self.scope_fingerprint,
            &self.plan_fingerprint,
            &self.formation_policy_fingerprint,
            &self.evaluation_rule_fingerprint,
            &self.time_rules_fingerprint,
        ] {
            check_fp(value)?;
        }
        exact(&self.contract)?;
        utc(&self.formed_at_utc)?;
        horizon(&self.retention_until_utc, &self.formed_at_utc)
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalEvaluationMutationRuleV1 {
    pub rule_id: String,
    pub effect: String,
    pub target_kind: String,
    pub selector: String,
}

/// The complete private envelope, retained only in Engine issuance history.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalEvaluationEnvelopeV1 {
    pub envelope_id: String,
    pub objective_ref: String,
    pub reach: LocalEvaluationReachV1,
    pub parent: Option<LocalBindingV1>,
    pub constraint_inputs: Vec<LocalBindingV1>,
    pub mutation_rules: Vec<LocalEvaluationMutationRuleV1>,
    pub escalation_triggers: Vec<LocalBindingV1>,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalEvaluationPathReachV1 {
    /// Literal mutation roots, each covering its complete owner subtree.
    pub owner_write_paths: Vec<String>,
    /// Declared proof-read selectors; actual paths are checked before opening.
    pub input_paths: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalEvaluationAuthorizationRequestV1 {
    pub request_key: String,
    pub purpose: LocalEvaluationPurposeV1,
    pub plan: LocalEvaluationPlanV1,
    pub envelope: LocalEvaluationEnvelopeV1,
    pub current_basis: LocalCheckCurrentBasisV1,
    pub reach: LocalEvaluationPathReachV1,
    pub retention_until_utc: String,
}
pub type LocalEvaluationRequestV1 = LocalEvaluationAuthorizationRequestV1;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalEvaluationTermsV1 {
    pub request_key: String,
    pub rule_ref: String,
    pub principal: String,
    pub agent: String,
    pub delegation: String,
    pub instruction: String,
    pub approval_source_ref: String,
    pub human_approval_asserted: bool,
    pub approved_content_fingerprint: String,
    pub expected_predecessor_fingerprint: String,
    pub expected_charter_fingerprint: String,
    pub ceiling: LocalEnvelopeCeilingV1,
    pub constraint_inputs: Vec<LocalBindingV1>,
    pub allowed_paths: Vec<String>,
    pub allowed_input_paths: Vec<String>,
    pub allowed_use_intents: Vec<LocalEvaluationUseIntentV1>,
    pub required_checkpoints: Vec<LocalEvaluationCheckpointV1>,
    pub retention_until_utc: String,
    pub revocation_principal: String,
}
impl LocalEvaluationTermsV1 {
    pub fn fingerprint_approved_content(&self) -> Result<String, LocalAuthorityRefusalV1> {
        let mut value = serde_json::to_value(self).map_err(codec)?;
        let object = value
            .as_object_mut()
            .ok_or_else(|| refuse("terms", "terms object required"))?;
        object.remove("human_approval_asserted");
        object.remove("approved_content_fingerprint");
        fingerprint("handbook.local-evaluation-approved-content@1.0.0", &value)
    }
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
        if self.rule_ref != LOCAL_EVALUATION_RULE_V1
            || !token(&self.request_key)
            || !self.human_approval_asserted
            || self.approved_content_fingerprint != self.fingerprint_approved_content()?
            || self.allowed_use_intents.is_empty()
            || self.allowed_use_intents.len() > 2
            || duplicate(&self.allowed_use_intents)
            || self.required_checkpoints != required_checkpoints()
        {
            return Err(refuse("evaluation_terms", "exact supplied human-approved supported evaluation terms and checkpoint rule required"));
        }
        for value in [
            &self.principal,
            &self.agent,
            &self.delegation,
            &self.instruction,
            &self.approval_source_ref,
            &self.revocation_principal,
        ] {
            bounded_text(value)?;
        }
        for value in [
            &self.expected_predecessor_fingerprint,
            &self.expected_charter_fingerprint,
        ] {
            check_fp(value)?;
        }
        utc(&self.retention_until_utc)?;
        validate_paths(&self.allowed_paths)?;
        validate_paths(&self.allowed_input_paths)?;
        if self.allowed_paths.is_empty()
            || self.allowed_input_paths.is_empty()
            || self.constraint_inputs.len() > 64
            || self
                .constraint_inputs
                .windows(2)
                .any(|p| p[0].reference >= p[1].reference)
        {
            return Err(refuse(
                "evaluation_terms",
                "bounded explicit complete owner-write and proof-read reach required",
            ));
        }
        for binding in &self.constraint_inputs {
            exact(binding)?;
        }
        let stack = profile()?.context_resolution().clone();
        if stack
            .ranks(
                &self.ceiling.active_level_id,
                self.ceiling.dimensions.each_ref().map(String::as_str),
            )
            .is_none()
        {
            return Err(refuse(
                "ceiling",
                "supplied evaluation ceiling must resolve in the selected shipped stack",
            ));
        }
        encode(self)?;
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalEvaluationAuthorityLocatorV1 {
    pub predecessor: LocalCheckLocatorV1,
    pub terms: LocalEvaluationTermsV1,
    pub adoption_basis: LocalCheckCurrentBasisV1,
    pub operating_lock_fingerprint: String,
    pub lock_generations: Vec<LocalBindingV1>,
    pub prepared_at_utc: String,
    pub request_fingerprint: String,
}
pub struct PreparedLocalEvaluationAuthorityV1 {
    locator: LocalEvaluationAuthorityLocatorV1,
}
impl PreparedLocalEvaluationAuthorityV1 {
    pub fn locator(&self) -> &LocalEvaluationAuthorityLocatorV1 {
        &self.locator
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalEvaluationAuthorityOutcomeV1 {
    pub schema_ref: String,
    pub locator: LocalEvaluationAuthorityLocatorV1,
    pub outcome_fingerprint: String,
    pub assurance: String,
}
#[derive(Debug, Serialize)]
pub struct LocalEvaluationAuthorityExecutionV1 {
    pub disposition: LocalAuthorityDispositionV1,
    pub outcome: Option<LocalEvaluationAuthorityOutcomeV1>,
    pub refusal: Option<LocalAuthorityRefusalV1>,
}

/// Opaque recovery key. It carries neither the private envelope nor permission.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalEvaluationBindingLocatorV1 {
    pub request_key: String,
    pub request_fingerprint: String,
    pub authority_fingerprint: String,
}

/// Stable public issuance projection; private envelope and paths are excluded.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalEvaluationAuthorizationBindingV1 {
    pub schema_ref: String,
    pub authority_owner_identity: String,
    pub issuance_fingerprint: String,
    pub opaque_lineage: String,
    pub purpose_fingerprint: String,
    pub plan_fingerprint: String,
    pub occurrence_identity: String,
    pub subject_fingerprint: String,
    pub scope_fingerprint: String,
    pub objective_fingerprint: String,
    pub use_intent: LocalEvaluationUseIntentV1,
    pub selection_fingerprint: String,
    pub requested_reach: LocalEvaluationReachV1,
    pub envelope_fingerprint: String,
    pub checkpoint_rule_ref: String,
    pub issuance_time_utc: String,
    pub retention_until_utc: String,
    pub request_within_envelope: bool,
}
#[derive(Debug, Serialize)]
pub struct LocalEvaluationAuthorizationExecutionV1 {
    pub disposition: LocalAuthorityDispositionV1,
    pub locator: LocalEvaluationBindingLocatorV1,
    pub binding: Option<LocalEvaluationAuthorizationBindingV1>,
    pub refusal: Option<LocalAuthorityRefusalV1>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalEvaluationCurrentStatusV1 {
    Current,
    Superseded,
    NotCurrent,
    Unverifiable,
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalEvaluationHistoricalStatusV1 {
    HistoricallyValid,
    HistoricallyInvalid,
    Unverifiable,
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalEvaluationVerificationObservationV1 {
    pub schema_ref: String,
    pub observation_fingerprint: String,
    pub issuance_fingerprint: String,
    pub purpose_fingerprint: String,
    pub checkpoint: LocalEvaluationCheckpointV1,
    pub checkpoint_target: Option<String>,
    pub checkpoint_source_fingerprint: Option<String>,
    pub current_basis: LocalCheckCurrentBasisV1,
    pub status: LocalEvaluationCurrentStatusV1,
    pub observed_at_utc: String,
    pub reason_code: Option<String>,
    pub reason: Option<String>,
}
#[derive(Clone, Debug, Serialize)]
pub struct LocalEvaluationHistoricalObservationV1 {
    pub status: LocalEvaluationHistoricalStatusV1,
    pub binding: Option<LocalEvaluationAuthorizationBindingV1>,
    pub reason: Option<LocalAuthorityRefusalV1>,
}

pub struct NativeLocalEvaluationAuthorityV1;

/// Exact Contracts-owned, already closed set snapshot. The public DTO alone
/// proves nothing; Engine pins the actual native record through final use.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalEvaluationFinalSetSourceV1 {
    pub relative_path: String,
    pub canonical_record_bytes: Vec<u8>,
    pub record_fingerprint: String,
    pub set_fingerprint: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalEvaluationFormationRequestV1 {
    pub subject_ref: String,
    pub subject_fingerprint: String,
    pub scope_fingerprint: String,
    pub objective: String,
    pub use_intent: LocalEvaluationUseIntentV1,
    pub requested_reach: LocalEvaluationReachV1,
    pub reach: LocalEvaluationPathReachV1,
    pub retention_until_utc: String,
}

/// Local terms permission for the preauthorization Contracts formation only.
/// This token is distinct from an Evaluation Authorization admission.
pub struct LocalEvaluationTermsAdmissionV1<'a> {
    locator: &'a LocalEvaluationAuthorityLocatorV1,
    basis: &'a LocalCheckCurrentBasisV1,
    request: &'a LocalEvaluationFormationRequestV1,
    guard: &'a OperatingGuard,
    tree: &'a RetainedRepositoryTreeGuardV1<'a>,
    authority_files: &'a RepositoryExactFilesCheckpointV1<'a>,
    authority_fingerprint: String,
}
impl LocalEvaluationTermsAdmissionV1<'_> {
    pub fn terms(&self) -> &LocalEvaluationTermsV1 {
        &self.locator.terms
    }
    pub fn request(&self) -> &LocalEvaluationFormationRequestV1 {
        self.request
    }
    pub fn current_basis(&self) -> &LocalCheckCurrentBasisV1 {
        self.basis
    }
    pub fn authority_fingerprint(&self) -> &str {
        &self.authority_fingerprint
    }
    /// Exact native project identity established by the original initialization
    /// claim. Applicability generation fingerprints are separate identities.
    pub fn project_identity(&self) -> &str {
        self.locator
            .predecessor
            .predecessor
            .charter
            .binding
            .initialization_claim
            .proposed_repository_identity()
    }
    pub fn require_repository_root(
        &self,
        root: &RetainedRepositoryRootV1,
    ) -> Result<(), LocalAuthorityRefusalV1> {
        require_root(root, self.locator)?;
        self.validate_current()
    }
    pub fn require_owner_write_subtree(&self, root: &str) -> Result<(), LocalAuthorityRefusalV1> {
        require_declared_write(&self.request.reach, root)?;
        self.validate_current()
    }
    pub fn require_input_path(&self, path: &str) -> Result<(), LocalAuthorityRefusalV1> {
        require_read(&self.request.reach.input_paths, path)?;
        self.validate_current()
    }
    /// Require coverage of every possible file in an owner's proof-read selector.
    /// This does not reinterpret the selector as an exact file path.
    pub fn require_input_selector(&self, selector: &str) -> Result<(), LocalAuthorityRefusalV1> {
        require_input_selector_coverage(
            &self.request.reach.input_paths,
            &self.locator.terms.allowed_input_paths,
            selector,
        )?;
        self.validate_current()
    }
    pub fn validate_current(&self) -> Result<(), LocalAuthorityRefusalV1> {
        self.tree.validate().map_err(fs_error)?;
        self.authority_files.validate().map_err(fs_error)?;
        self.guard.validate()?;
        current_authority(self.guard.root(), self.locator, self.basis)?;
        horizon(&self.request.retention_until_utc, &now()?)?;
        Ok(())
    }
}

/// Exact issuance and current verification guarded through one consuming owner
/// callback. It cannot be cloned, serialized or manufactured by a caller.
pub struct LocalEvaluationAdmissionV1<'a> {
    locator: &'a LocalEvaluationAuthorityLocatorV1,
    issuance: &'a LocalEvaluationBindingLocatorV1,
    basis: &'a LocalCheckCurrentBasisV1,
    request: &'a LocalEvaluationAuthorizationRequestV1,
    binding: &'a LocalEvaluationAuthorizationBindingV1,
    observation: &'a LocalEvaluationVerificationObservationV1,
    guard: &'a OperatingGuard,
    tree: &'a RetainedRepositoryTreeGuardV1<'a>,
    files: &'a RepositoryExactFilesCheckpointV1<'a>,
    reach: &'a LocalEvaluationPathReachV1,
    final_set_source: Option<&'a LocalEvaluationFinalSetSourceV1>,
    set_files: Option<&'a RepositoryExactFilesCheckpointV1<'a>>,
}
impl LocalEvaluationAdmissionV1<'_> {
    pub fn binding(&self) -> &LocalEvaluationAuthorizationBindingV1 {
        self.binding
    }
    pub fn request(&self) -> &LocalEvaluationAuthorizationRequestV1 {
        self.request
    }
    pub fn checkpoint(&self) -> LocalEvaluationCheckpointV1 {
        self.observation.checkpoint
    }
    pub fn checkpoint_target(&self) -> Option<&str> {
        self.observation.checkpoint_target.as_deref()
    }
    pub fn observation(&self) -> &LocalEvaluationVerificationObservationV1 {
        self.observation
    }
    pub fn current_basis(&self) -> &LocalCheckCurrentBasisV1 {
        self.basis
    }
    pub fn final_set_source(&self) -> Option<&LocalEvaluationFinalSetSourceV1> {
        self.final_set_source
    }
    pub fn issuance_locator(&self) -> &LocalEvaluationBindingLocatorV1 {
        self.issuance
    }
    /// Require the producer's actual parsed request to carry the exact stack and
    /// six canonical resolution IDs authorized by the original closed Plan.
    pub fn require_requested_resolution(
        &self,
        stack: &LocalBindingV1,
        dimension_ids: [&str; 6],
    ) -> Result<(), LocalAuthorityRefusalV1> {
        self.validate_current()?;
        require_exact_resolution_projection(&self.binding.requested_reach, stack, dimension_ids)
    }
    /// Domain coordinate conversion over the admitted exact stack. It grants no
    /// reach and transfers none of Engine's envelope or currentness decisions.
    pub fn resolution_dimension_ranks(
        &self,
        values: [&str; 6],
    ) -> Result<[u8; 6], LocalAuthorityRefusalV1> {
        self.validate_current()?;
        let selected = profile()?;
        let stack = selected.context_resolution();
        if stack.exact_ref().as_str() != self.binding.requested_reach.stack.reference
            || stack.definition_fingerprint().as_str()
                != self.binding.requested_reach.stack.fingerprint
        {
            return Err(refuse(
                "resolution_stack",
                "only the exact admitted stack can interpret capability coordinates",
            ));
        }
        stack.dimension_ranks(values).ok_or_else(|| {
            refuse(
                "resolution_stack",
                "capability coordinates do not resolve in the exact admitted stack",
            )
        })
    }
    /// Exact immutable dependency locations for the inward consuming owner.
    /// These relative owner paths are excluded from the public Binding projection.
    pub fn historical_dependency_paths(&self) -> Result<Vec<String>, LocalAuthorityRefusalV1> {
        self.validate_current()?;
        let mut paths = local_charter::local_evaluation_historical_dependencies(
            self.guard.root(),
            &self.locator.predecessor.predecessor.charter,
        )?;
        for (root, key) in [
            (
                BASE,
                self.locator
                    .predecessor
                    .predecessor
                    .terms
                    .request_key
                    .as_str(),
            ),
            (
                local_check::LOCAL_CHECK_ROOT_V1,
                self.locator.predecessor.terms.request_key.as_str(),
            ),
        ] {
            for file in ["request.json", "outcome.json", "terminal.json"] {
                paths.push(format!("{root}/requests/{key}/{file}"));
            }
        }
        paths.extend(
            authority_paths(self.locator)
                .into_iter()
                .filter(|p| !p.ends_with("/current.json")),
        );
        paths.extend(issuance_paths(self.issuance));
        let checks = format!("{LOCAL_EVALUATION_ROOT_V1}/checks");
        require_read(&self.reach.input_paths, &checks)?;
        for entry in self
            .guard
            .root()
            .list_directory_bounded(&checks, 4096)
            .map_err(fs_error)?
        {
            let path = format!("{checks}/{}", entry.name());
            require_read(&self.reach.input_paths, &path)?;
            if entry.kind()
                != crate::repository_durability::RepositoryDirectoryEntryKindV1::RegularFile
            {
                return Err(refuse(
                    "check_inventory",
                    "immutable Engine check inventory contains an unexpected directory",
                ));
            }
            let bytes = found(self.guard.root(), &path)?;
            let record: LocalEvaluationVerificationObservationV1 = decode(&bytes)?;
            let mut value = serde_json::to_value(&record).map_err(codec)?;
            value
                .as_object_mut()
                .ok_or_else(|| refuse("check_inventory", "closed check object required"))?
                .insert(
                    "observation_fingerprint".into(),
                    serde_json::Value::String(String::new()),
                );
            let expected = fingerprint(
                "handbook.local-evaluation-current-observation@1.0.0",
                &value,
            )?;
            let key = expected
                .strip_prefix("sha256:")
                .ok_or_else(|| refuse("fingerprint", "exact check identity required"))?;
            if record.observation_fingerprint != expected
                || path != format!("{checks}/{key}.json")
                || encode(&record)? != bytes
            {
                return Err(refuse(
                    "check_inventory",
                    "retained immutable Engine check identity or bytes are substituted",
                ));
            }
            if record.issuance_fingerprint == self.binding.issuance_fingerprint {
                paths.push(path);
            }
        }
        paths.sort();
        paths.dedup();
        for path in &paths {
            require_read(&self.reach.input_paths, path)?;
        }
        self.validate_current()?;
        Ok(paths)
    }
    pub fn require_repository_root(
        &self,
        root: &RetainedRepositoryRootV1,
    ) -> Result<(), LocalAuthorityRefusalV1> {
        require_root(root, self.locator)?;
        self.validate_current()
    }
    pub fn require_owner_write_subtree(&self, root: &str) -> Result<(), LocalAuthorityRefusalV1> {
        require_declared_write(self.reach, root)?;
        self.validate_current()
    }
    pub fn require_input_path(&self, path: &str) -> Result<(), LocalAuthorityRefusalV1> {
        require_read(&self.reach.input_paths, path)?;
        self.validate_current()
    }
    pub fn require_input_selector(&self, selector: &str) -> Result<(), LocalAuthorityRefusalV1> {
        require_input_selector_coverage(
            &self.reach.input_paths,
            &self.locator.terms.allowed_input_paths,
            selector,
        )?;
        self.validate_current()
    }
    pub fn validate_current(&self) -> Result<(), LocalAuthorityRefusalV1> {
        self.tree.validate().map_err(fs_error)?;
        self.files.validate().map_err(fs_error)?;
        self.guard.validate()?;
        if let Some(files) = self.set_files {
            files.validate().map_err(fs_error)?;
        }
        if let Some(source) = self.final_set_source {
            source.validate(self.request)?;
            if self.observation.checkpoint != LocalEvaluationCheckpointV1::FinalContext
                || self.observation.checkpoint_target.as_deref()
                    != Some(source.set_fingerprint.as_str())
                || self.observation.checkpoint_source_fingerprint.as_deref()
                    != Some(source.record_fingerprint.as_str())
            {
                return Err(refuse(
                    "final_set_congruence",
                    "final authorization check must bind the exact already fixed native set source",
                ));
            }
        }
        current_authority(self.guard.root(), self.locator, self.basis)?;
        // The private tuple was fully reread and checked after these exact
        // immutable file pins were established. Native identity/version/byte
        // validation above preserves that tuple through every consuming read.
        if self.basis != &self.request.current_basis {
            return Err(refuse(
                "issuance_congruence",
                "exact original issuance, Purpose and applicability required",
            ));
        }
        require_unsuperseded(self.guard.root(), self.issuance)?;
        horizon(&self.request.retention_until_utc, &now()?)?;
        Ok(())
    }
}

impl NativeLocalEvaluationAuthorityV1 {
    pub fn prepare(
        repo: impl AsRef<Path>,
        predecessor: &LocalCheckLocatorV1,
        terms: LocalEvaluationTermsV1,
        basis: LocalCheckCurrentBasisV1,
    ) -> Result<PreparedLocalEvaluationAuthorityV1, LocalAuthorityRefusalV1> {
        validate_terms_predecessor(predecessor, &terms)?;
        let reach = terms_adoption_reach(&terms);
        validate_declared_reach(&terms, &reach, true)?;
        let guard = OperatingGuard::acquire(repo.as_ref(), &predecessor.predecessor, false)?;
        let original =
            local_check::evaluation_predecessor_current(guard.root(), predecessor, &basis)?;
        if original.outcome_fingerprint != terms.expected_predecessor_fingerprint {
            return Err(refuse(
                "predecessor_terms",
                "exact current predecessor selection required",
            ));
        }
        let prepared_at_utc = now()?;
        horizon(&terms.retention_until_utc, &prepared_at_utc)?;
        let mut locator = LocalEvaluationAuthorityLocatorV1 {
            predecessor: predecessor.clone(),
            terms,
            adoption_basis: basis,
            operating_lock_fingerprint: guard
                .root()
                .existing_lock_physical_fingerprint_v1(".handbook/state/locks/local-operating.lock")
                .map_err(fs_error)?,
            lock_generations: evaluation_lock_paths()
                .iter()
                .map(|path| {
                    Ok(LocalBindingV1 {
                        reference: (*path).into(),
                        fingerprint: guard
                            .root()
                            .existing_lock_physical_fingerprint_v1(path)
                            .map_err(fs_error)?,
                    })
                })
                .collect::<Result<Vec<_>, LocalAuthorityRefusalV1>>()?,
            prepared_at_utc,
            request_fingerprint: String::new(),
        };
        locator.request_fingerprint = locator.fingerprint_request()?;
        locator.validate()?;
        if guard
            .root()
            .list_directory_bounded(LOCAL_EVALUATION_ROOT_V1, 4096)
            .is_ok()
        {
            return Err(refuse(
                "already_selected",
                "retained evaluation footprint requires exact original recovery",
            ));
        }
        guard.validate()?;
        Ok(PreparedLocalEvaluationAuthorityV1 { locator })
    }
    pub fn adopt(
        repo: impl AsRef<Path>,
        prepared: PreparedLocalEvaluationAuthorityV1,
    ) -> LocalEvaluationAuthorityExecutionV1 {
        adopt_evaluation(repo.as_ref(), &prepared.locator, false, || {})
    }
    pub fn resume_request(
        repo: impl AsRef<Path>,
        locator: &LocalEvaluationAuthorityLocatorV1,
    ) -> LocalEvaluationAuthorityExecutionV1 {
        adopt_evaluation(repo.as_ref(), locator, true, || {})
    }
    pub fn resolve_request(
        repo: impl AsRef<Path>,
        locator: &LocalEvaluationAuthorityLocatorV1,
    ) -> LocalEvaluationAuthorityExecutionV1 {
        let result = (|| {
            locator.validate()?;
            validate_declared_reach(&locator.terms, &terms_adoption_reach(&locator.terms), false)?;
            let guard =
                OperatingGuard::acquire(repo.as_ref(), &locator.predecessor.predecessor, false)?;
            require_evaluation_lock(guard.root(), locator)?;
            let value = history_authority(guard.root(), locator)?;
            guard.validate()?;
            Ok(value)
        })();
        authority_execution(result, LocalAuthorityDispositionV1::Replayed)
    }
    pub fn with_terms_admission<T>(
        repo: impl AsRef<Path>,
        locator: &LocalEvaluationAuthorityLocatorV1,
        basis: &LocalCheckCurrentBasisV1,
        request: &LocalEvaluationFormationRequestV1,
        consume: impl FnOnce(&LocalEvaluationTermsAdmissionV1<'_>) -> Result<T, LocalAuthorityRefusalV1>,
    ) -> Result<T, LocalAuthorityRefusalV1> {
        locator.validate()?;
        validate_formation(request, &locator.terms)?;
        let guard =
            OperatingGuard::acquire(repo.as_ref(), &locator.predecessor.predecessor, false)?;
        let authority = current_authority(guard.root(), locator, basis)?;
        let tree = guard
            .root()
            .retain_local_check_authority_tree_guard()
            .map_err(fs_error)?;
        let paths = authority_paths(locator);
        let refs = paths.iter().map(String::as_str).collect::<Vec<_>>();
        let files = guard
            .root()
            .retain_exact_files_checkpoint_v1(&refs)
            .map_err(fs_error)?;
        let admission = LocalEvaluationTermsAdmissionV1 {
            locator,
            basis,
            request,
            guard: &guard,
            tree: &tree,
            authority_files: &files,
            authority_fingerprint: authority.outcome_fingerprint,
        };
        admission.validate_current()?;
        let value = consume(&admission)?;
        admission.validate_current()?;
        Ok(value)
    }
    pub fn issuance_locator(
        authority: &LocalEvaluationAuthorityLocatorV1,
        request: &LocalEvaluationAuthorizationRequestV1,
    ) -> Result<LocalEvaluationBindingLocatorV1, LocalAuthorityRefusalV1> {
        authority.validate()?;
        request.validate_under(&authority.terms)?;
        Ok(LocalEvaluationBindingLocatorV1 {
            request_key: request.request_key.clone(),
            request_fingerprint: request.fingerprint()?,
            authority_fingerprint: expected_authority(authority)?.outcome_fingerprint,
        })
    }
    pub fn issue(
        repo: impl AsRef<Path>,
        authority: &LocalEvaluationAuthorityLocatorV1,
        request: &LocalEvaluationAuthorizationRequestV1,
    ) -> LocalEvaluationAuthorizationExecutionV1 {
        issue_evaluation(repo.as_ref(), authority, request, false, || {})
    }
    pub fn recover_issuance(
        repo: impl AsRef<Path>,
        authority: &LocalEvaluationAuthorityLocatorV1,
        original: &LocalEvaluationBindingLocatorV1,
        reach: &LocalEvaluationPathReachV1,
    ) -> LocalEvaluationAuthorizationExecutionV1 {
        let result = (|| {
            authority.validate()?;
            original.validate()?;
            validate_declared_reach(&authority.terms, reach, false)?;
            let guard =
                OperatingGuard::acquire(repo.as_ref(), &authority.predecessor.predecessor, false)?;
            require_evaluation_lock(guard.root(), authority)?;
            let (_, binding) = read_issuance(guard.root(), authority, original, reach)?;
            guard.validate()?;
            Ok(binding)
        })();
        issuance_execution(
            original.clone(),
            result,
            LocalAuthorityDispositionV1::Replayed,
        )
    }
    pub fn check_history(
        repo: impl AsRef<Path>,
        authority: &LocalEvaluationAuthorityLocatorV1,
        original: &LocalEvaluationBindingLocatorV1,
        reach: &LocalEvaluationPathReachV1,
    ) -> LocalEvaluationHistoricalObservationV1 {
        let outcome = Self::recover_issuance(repo, authority, original, reach);
        match outcome.binding {
            Some(binding) => LocalEvaluationHistoricalObservationV1 {
                status: LocalEvaluationHistoricalStatusV1::HistoricallyValid,
                binding: Some(binding),
                reason: None,
            },
            None => {
                LocalEvaluationHistoricalObservationV1 {
                    status: if outcome.refusal.as_ref().is_some_and(|r| {
                        r.code == "issuance_conflict" || r.code == "record_conflict"
                    }) {
                        LocalEvaluationHistoricalStatusV1::HistoricallyInvalid
                    } else {
                        LocalEvaluationHistoricalStatusV1::Unverifiable
                    },
                    binding: None,
                    reason: outcome.refusal,
                }
            }
        }
    }
    pub fn with_verification<T>(
        repo: impl AsRef<Path>,
        authority: &LocalEvaluationAuthorityLocatorV1,
        original: &LocalEvaluationBindingLocatorV1,
        expected_request: &LocalEvaluationAuthorizationRequestV1,
        checkpoint: LocalEvaluationCheckpointV1,
        checkpoint_target: Option<&str>,
        basis: &LocalCheckCurrentBasisV1,
        reach: &LocalEvaluationPathReachV1,
        consume: impl FnOnce(&LocalEvaluationAdmissionV1<'_>) -> Result<T, LocalAuthorityRefusalV1>,
    ) -> Result<T, LocalAuthorityRefusalV1> {
        if checkpoint == LocalEvaluationCheckpointV1::FinalContext {
            return Err(refuse(
                "final_set_source_required",
                "final verification requires the already closed exact native Contracts set source",
            ));
        }
        with_verification_source(
            repo.as_ref(),
            authority,
            original,
            expected_request,
            checkpoint,
            checkpoint_target,
            None,
            basis,
            reach,
            consume,
        )
    }
    pub fn with_final_verification<T>(
        repo: impl AsRef<Path>,
        authority: &LocalEvaluationAuthorityLocatorV1,
        original: &LocalEvaluationBindingLocatorV1,
        expected_request: &LocalEvaluationAuthorizationRequestV1,
        source: &LocalEvaluationFinalSetSourceV1,
        basis: &LocalCheckCurrentBasisV1,
        reach: &LocalEvaluationPathReachV1,
        consume: impl FnOnce(&LocalEvaluationAdmissionV1<'_>) -> Result<T, LocalAuthorityRefusalV1>,
    ) -> Result<T, LocalAuthorityRefusalV1> {
        with_verification_source(
            repo.as_ref(),
            authority,
            original,
            expected_request,
            LocalEvaluationCheckpointV1::FinalContext,
            Some(&source.set_fingerprint),
            Some(source),
            basis,
            reach,
            consume,
        )
    }

    /// Resume only an original owner-retained pending intent. Inspection and
    /// historical resolution never call this mutation route automatically.
    pub fn resume_issuance_request(
        repo: impl AsRef<Path>,
        authority: &LocalEvaluationAuthorityLocatorV1,
        original: &LocalEvaluationBindingLocatorV1,
        reach: &LocalEvaluationPathReachV1,
    ) -> LocalEvaluationAuthorizationExecutionV1 {
        let request = (|| {
            authority.validate()?;
            original.validate()?;
            validate_declared_reach(&authority.terms, reach, true)?;
            let guard =
                OperatingGuard::acquire(repo.as_ref(), &authority.predecessor.predecessor, false)?;
            require_evaluation_lock(guard.root(), authority)?;
            let intent = read_intent(guard.root(), authority, original, reach)?;
            if &intent.request.reach != reach {
                return Err(refuse(
                    "request_reach",
                    "resume requires original exact declared reach",
                ));
            }
            guard.validate()?;
            Ok(intent.request)
        })();
        match request {
            Ok(request) => issue_evaluation(repo.as_ref(), authority, &request, true, || {}),
            Err(error) => issuance_execution(
                original.clone(),
                Err(error),
                LocalAuthorityDispositionV1::Refused,
            ),
        }
    }

    #[cfg(feature = "test-support")]
    #[doc(hidden)]
    pub fn adopt_with_interruption_for_test(
        repo: impl AsRef<Path>,
        prepared: PreparedLocalEvaluationAuthorityV1,
        interrupt: impl FnOnce(),
    ) -> LocalEvaluationAuthorityExecutionV1 {
        adopt_evaluation(repo.as_ref(), &prepared.locator, false, interrupt)
    }
    #[cfg(feature = "test-support")]
    #[doc(hidden)]
    pub fn issue_with_interruption_for_test(
        repo: impl AsRef<Path>,
        authority: &LocalEvaluationAuthorityLocatorV1,
        request: &LocalEvaluationAuthorizationRequestV1,
        interrupt: impl FnOnce(),
    ) -> LocalEvaluationAuthorizationExecutionV1 {
        issue_evaluation(repo.as_ref(), authority, request, false, interrupt)
    }

    pub fn revoke(
        repo: impl AsRef<Path>,
        authority: &LocalEvaluationAuthorityLocatorV1,
        request: &LocalRevokeRequestV1,
        reach: &LocalEvaluationPathReachV1,
    ) -> Result<(), LocalAuthorityRefusalV1> {
        authority.validate()?;
        validate_declared_reach(&authority.terms, reach, true)?;
        if !token(&request.request_key) || request.principal != authority.terms.revocation_principal
        {
            return Err(refuse(
                "revocation_authority",
                "exact supplied named revocation principal required",
            ));
        }
        bounded_text(&request.source_ref)?;
        bounded_text(&request.instruction)?;
        let guard =
            OperatingGuard::acquire(repo.as_ref(), &authority.predecessor.predecessor, false)?;
        let root = guard.root();
        require_evaluation_lock(root, authority)?;
        let outcome = history_authority(root, authority)?;
        if request.expected_current_fingerprint != outcome.outcome_fingerprint {
            return Err(refuse(
                "revocation_predecessor",
                "exact selected evaluation authority required",
            ));
        }
        let current = format!("{LOCAL_EVALUATION_ROOT_V1}/current.json");
        let revoke = format!("{LOCAL_EVALUATION_ROOT_V1}/revoke.json");
        let before = encode(&EvaluationHead {
            schema_ref: LOCAL_EVALUATION_RULE_V1.into(),
            selected: outcome.outcome_fingerprint.clone(),
            revoked: None,
        })?;
        let after = encode(&EvaluationHead {
            schema_ref: LOCAL_EVALUATION_RULE_V1.into(),
            selected: outcome.outcome_fingerprint,
            revoked: Some(fingerprint(
                "handbook.local-evaluation-revocation@1.0.0",
                request,
            )?),
        })?;
        let retained = root
            .read_local_owner_optional_bounded(&current, MAX)
            .map_err(fs_error)?;
        match root
            .read_local_owner_optional_bounded(&revoke, MAX)
            .map_err(fs_error)?
        {
            RepositoryReadV1::Absent => {
                if retained != RepositoryReadV1::Found(before.clone()) {
                    return Err(refuse(
                        "revocation_state",
                        "missing or substituted selection cannot authorize revocation",
                    ));
                }
                write_exact(root, &revoke, &encode(request)?)?;
            }
            RepositoryReadV1::Found(bytes) if bytes == encode(request)? => {}
            _ => {
                return Err(refuse(
                    "revocation_conflict",
                    "another original revocation is retained",
                ))
            }
        }
        let staged = format!("{LOCAL_EVALUATION_ROOT_V1}/revoked-head.json");
        let displaced = format!("{LOCAL_EVALUATION_ROOT_V1}/displaced-active-head.json");
        if retained != RepositoryReadV1::Found(after.clone()) {
            write_exact(root, &staged, &after)?;
            if retained == RepositoryReadV1::Found(before.clone()) {
                root.replace_exact_durable(&current, &before, &staged, &displaced)
                    .map_err(fs_error)?;
            } else if retained == RepositoryReadV1::Absent
                && root
                    .read_local_owner_optional_bounded(&displaced, MAX)
                    .map_err(fs_error)?
                    == RepositoryReadV1::Found(before)
            {
                root.rename_noreplace_durable(&staged, &current)
                    .map_err(fs_error)?;
            } else {
                return Err(refuse(
                    "revocation_state",
                    "only exact retained revocation intent can restore its committed head",
                ));
            }
        }
        if found(root, &current)? != after {
            return Err(refuse(
                "uncertain",
                "exact revocation publication is unverified",
            ));
        }
        guard.validate()
    }

    /// Resolve an immutable recorded check as historical data. The returned DTO
    /// cannot recreate current permission or the live guarded admission token.
    pub fn resolve_verification(
        repo: impl AsRef<Path>,
        authority: &LocalEvaluationAuthorityLocatorV1,
        original: &LocalEvaluationBindingLocatorV1,
        observation_fingerprint: &str,
        reach: &LocalEvaluationPathReachV1,
    ) -> Result<LocalEvaluationVerificationObservationV1, LocalAuthorityRefusalV1> {
        authority.validate()?;
        original.validate()?;
        check_fp(observation_fingerprint)?;
        validate_declared_reach(&authority.terms, reach, false)?;
        let key = observation_fingerprint
            .strip_prefix("sha256:")
            .ok_or_else(|| refuse("fingerprint", "exact check fingerprint required"))?;
        let path = format!("{LOCAL_EVALUATION_ROOT_V1}/checks/{key}.json");
        require_read(&reach.input_paths, &path)?;
        let guard =
            OperatingGuard::acquire(repo.as_ref(), &authority.predecessor.predecessor, false)?;
        let (_, binding) = read_issuance(guard.root(), authority, original, reach)?;
        let bytes = found(guard.root(), &path)?;
        let record: LocalEvaluationVerificationObservationV1 = decode(&bytes)?;
        let mut value = record.clone();
        value.observation_fingerprint.clear();
        if record.schema_ref != LOCAL_EVALUATION_RULE_V1
            || record.observation_fingerprint != observation_fingerprint
            || fingerprint(
                "handbook.local-evaluation-current-observation@1.0.0",
                &value,
            )? != observation_fingerprint
            || record.issuance_fingerprint != binding.issuance_fingerprint
            || record.purpose_fingerprint != binding.purpose_fingerprint
            || encode(&record)? != bytes
        {
            return Err(refuse(
                "check_history",
                "exact immutable owner verification record is substituted",
            ));
        }
        guard.validate()?;
        Ok(record)
    }
}

#[allow(clippy::too_many_arguments)]
fn with_verification_source<T>(
    repo: &Path,
    authority: &LocalEvaluationAuthorityLocatorV1,
    original: &LocalEvaluationBindingLocatorV1,
    expected_request: &LocalEvaluationAuthorizationRequestV1,
    checkpoint: LocalEvaluationCheckpointV1,
    checkpoint_target: Option<&str>,
    final_set_source: Option<&LocalEvaluationFinalSetSourceV1>,
    basis: &LocalCheckCurrentBasisV1,
    reach: &LocalEvaluationPathReachV1,
    consume: impl FnOnce(&LocalEvaluationAdmissionV1<'_>) -> Result<T, LocalAuthorityRefusalV1>,
) -> Result<T, LocalAuthorityRefusalV1> {
    authority.validate()?;
    expected_request.validate_under(&authority.terms)?;
    original.validate()?;
    validate_declared_reach(&authority.terms, reach, true)?;
    validate_checkpoint(checkpoint, checkpoint_target)?;
    if reach != &expected_request.reach {
        return Err(refuse(
            "request_reach",
            "consuming declared reach differs from exact authorization request",
        ));
    }
    if let Some(source) = final_set_source {
        source.validate(expected_request)?;
        require_read(&reach.input_paths, &source.relative_path)?;
    }
    let guard = OperatingGuard::acquire(repo, &authority.predecessor.predecessor, false)?;
    require_evaluation_lock(guard.root(), authority)?;
    // A copied future set fingerprint is insufficient. Its already retained
    // exact native bytes are pinned before the final Engine observation.
    let set_files = final_set_source
        .map(|source| pin_final_set_source(guard.root(), source, expected_request, reach))
        .transpose()?;
    let (request, binding) = read_issuance(guard.root(), authority, original, reach)?;
    if &request != expected_request {
        return Err(refuse(
            "issuance_congruence",
            "copied binding cannot transfer the original exact request",
        ));
    }
    let current = (|| {
        current_authority(guard.root(), authority, basis)?;
        if basis != &request.current_basis {
            return Err(refuse(
                "purpose_currentness",
                "new applicability cannot be substituted into original Purpose",
            ));
        }
        require_unsuperseded(guard.root(), original)?;
        horizon(&request.retention_until_utc, &now()?)?;
        Ok(())
    })();
    let observation = retain_verification(
        guard.root(),
        &binding,
        checkpoint,
        checkpoint_target,
        final_set_source.map(|s| s.record_fingerprint.as_str()),
        basis,
        &current,
    )?;
    current?;
    let tree = guard
        .root()
        .retain_local_check_authority_tree_guard()
        .map_err(fs_error)?;
    let mut paths = authority_paths(authority);
    paths.extend(issuance_paths(original));
    paths.sort();
    paths.dedup();
    let refs = paths.iter().map(String::as_str).collect::<Vec<_>>();
    let files = guard
        .root()
        .retain_exact_files_checkpoint_v1(&refs)
        .map_err(fs_error)?;
    // Close the read-to-pin race before retaining the tuple in the sealed token.
    // Every subsequent use still checks native pins and all mutable authority.
    let reread = read_issuance(guard.root(), authority, original, reach)?;
    let (request, binding) =
        require_pinned_issuance_tuple(guard.root(), &files, original, &request, &binding, reread)?;
    let admission = LocalEvaluationAdmissionV1 {
        locator: authority,
        issuance: original,
        basis,
        request: &request,
        binding: &binding,
        observation: &observation,
        guard: &guard,
        tree: &tree,
        files: &files,
        reach,
        final_set_source,
        set_files: set_files.as_ref(),
    };
    admission.validate_current()?;
    let value = consume(&admission)?;
    admission.validate_current()?;
    Ok(value)
}

fn require_pinned_issuance_tuple(
    root: &RetainedRepositoryRootV1,
    files: &RepositoryExactFilesCheckpointV1<'_>,
    original: &LocalEvaluationBindingLocatorV1,
    expected_request: &LocalEvaluationAuthorizationRequestV1,
    expected_binding: &LocalEvaluationAuthorizationBindingV1,
    reread: (
        LocalEvaluationAuthorizationRequestV1,
        LocalEvaluationAuthorizationBindingV1,
    ),
) -> Result<
    (
        LocalEvaluationAuthorizationRequestV1,
        LocalEvaluationAuthorizationBindingV1,
    ),
    LocalAuthorityRefusalV1,
> {
    let paths = issuance_paths(original);
    let refs = paths.iter().map(String::as_str).collect::<Vec<_>>();
    files
        .require_file_membership(root, &refs)
        .map_err(fs_error)?;
    if &reread.0 != expected_request || &reread.1 != expected_binding {
        return Err(refuse(
            "issuance_congruence",
            "post-pin native issuance differs from the exact original request and binding",
        ));
    }
    let bytes = encode(&reread.1)?;
    for file in ["outcome.json", "terminal.json"] {
        files
            .require_exact_file(root, &issuance_path(original, file), &bytes)
            .map_err(fs_error)?;
    }
    files.validate().map_err(fs_error)?;
    Ok(reread)
}

impl LocalEvaluationReachV1 {
    pub fn selected(ceiling: &LocalEnvelopeCeilingV1) -> Result<Self, LocalAuthorityRefusalV1> {
        let selected = profile()?;
        Ok(Self {
            profile: LocalBindingV1 {
                reference: selected.exact_ref().to_string(),
                fingerprint: selected.resolved_profile_fingerprint().to_string(),
            },
            stack: LocalBindingV1 {
                reference: selected.context_resolution().exact_ref().to_string(),
                fingerprint: selected
                    .context_resolution()
                    .definition_fingerprint()
                    .to_string(),
            },
            active_level_id: ceiling.active_level_id.clone(),
            dimensions: ceiling.dimensions.clone(),
        })
    }
    pub fn validate(&self) -> Result<(), LocalAuthorityRefusalV1> {
        exact(&self.profile)?;
        exact(&self.stack)?;
        let selected = profile()?;
        if self.profile.reference != selected.exact_ref().as_str()
            || self.profile.fingerprint != selected.resolved_profile_fingerprint().as_str()
            || self.stack.reference != selected.context_resolution().exact_ref().as_str()
            || self.stack.fingerprint
                != selected
                    .context_resolution()
                    .definition_fingerprint()
                    .as_str()
            || selected
                .context_resolution()
                .ranks(
                    &self.active_level_id,
                    self.dimensions.each_ref().map(String::as_str),
                )
                .is_none()
        {
            return Err(refuse(
                "resolution_stack",
                "complete exact selected profile/stack and supported dimension values required",
            ));
        }
        Ok(())
    }
}
impl LocalEvaluationEnvelopeV1 {
    pub fn local(
        subject_ref: &str,
        reach: LocalEvaluationReachV1,
        terms: &LocalEvaluationTermsV1,
    ) -> Self {
        Self {
            envelope_id: terms.request_key.clone(),
            objective_ref: subject_ref.into(),
            reach,
            parent: None,
            constraint_inputs: terms.constraint_inputs.clone(),
            mutation_rules: terms
                .allowed_paths
                .iter()
                .enumerate()
                .map(|(index, path)| LocalEvaluationMutationRuleV1 {
                    rule_id: format!("local-evaluation-{index}"),
                    effect: "allow".into(),
                    target_kind: "repository_path".into(),
                    selector: path.clone(),
                })
                .collect(),
            escalation_triggers: Vec::new(),
        }
    }
    pub fn fingerprint(&self) -> Result<String, LocalAuthorityRefusalV1> {
        fingerprint("handbook.local-evaluation-private-envelope@1.0.0", self)
    }
    fn validate(
        &self,
        terms: &LocalEvaluationTermsV1,
        purpose: &LocalEvaluationPurposeV1,
    ) -> Result<(), LocalAuthorityRefusalV1> {
        self.reach.validate()?;
        if !token(&self.envelope_id)
            || self.objective_ref != purpose.subject_ref
            || self.parent.is_some()
            || !self.escalation_triggers.is_empty()
            || self.constraint_inputs != terms.constraint_inputs
            || self.reach.active_level_id != terms.ceiling.active_level_id
        {
            return Err(refuse("private_envelope", "complete exact local envelope subject, root, constraints and active level required"));
        }
        require_subject_reach(
            &terms.allowed_input_paths,
            &self.objective_ref,
            &purpose.subject_fingerprint,
        )?;
        require_reach_within(
            &self.reach,
            &LocalEvaluationReachV1::selected(&terms.ceiling)?,
        )?;
        let mut ids = std::collections::BTreeSet::new();
        for rule in &self.mutation_rules {
            if !token(&rule.rule_id)
                || !ids.insert(&rule.rule_id)
                || rule.target_kind != "repository_path"
                || !["allow", "deny"].contains(&rule.effect.as_str())
            {
                return Err(refuse(
                    "private_envelope",
                    "bounded unique supported envelope mutation rules required",
                ));
            }
            validate_selector(&rule.selector)?;
            if rule.effect == "allow"
                && !terms
                    .allowed_paths
                    .iter()
                    .any(|parent| selector_contains(parent, &rule.selector))
            {
                return Err(refuse(
                    "path_expansion",
                    "private envelope mutation selector exceeds current supplied terms",
                ));
            }
        }
        if self.mutation_rules.len() > 64 {
            return Err(refuse(
                "private_envelope",
                "envelope mutation population exceeds its bound",
            ));
        }
        Ok(())
    }
}
impl LocalEvaluationAuthorizationRequestV1 {
    pub fn fingerprint(&self) -> Result<String, LocalAuthorityRefusalV1> {
        self.validate()?;
        fingerprint(
            "handbook.local-evaluation-authorization-request@1.0.0",
            self,
        )
    }
    pub fn canonical_json_bytes(&self) -> Result<Vec<u8>, LocalAuthorityRefusalV1> {
        self.validate()?;
        encode(self)
    }
    pub fn from_json_bytes(bytes: &[u8]) -> Result<Self, LocalAuthorityRefusalV1> {
        let value: Self = decode(bytes)?;
        value.validate()?;
        Ok(value)
    }
    pub fn validate(&self) -> Result<(), LocalAuthorityRefusalV1> {
        self.purpose.validate()?;
        self.plan.validate()?;
        if !token(&self.request_key)
            || self.purpose.plan_fingerprint != self.plan.fingerprint()?
            || self.purpose.requirements_fingerprint != self.plan.requirements_fingerprint
            || self.retention_until_utc != self.purpose.retention_until_utc
        {
            return Err(refuse(
                "request_congruence",
                "exact Purpose, Plan, requirements and retention closure required",
            ));
        }
        check_fp(&self.current_basis.relation_fingerprint)?;
        check_fp(&self.current_basis.governance_tree_fingerprint)?;
        validate_paths(&self.reach.owner_write_paths)?;
        validate_paths(&self.reach.input_paths)?;
        horizon(&self.retention_until_utc, &self.purpose.formed_at_utc)?;
        encode(self)?;
        Ok(())
    }
    fn validate_under(
        &self,
        terms: &LocalEvaluationTermsV1,
    ) -> Result<(), LocalAuthorityRefusalV1> {
        self.validate()?;
        if !terms.allowed_use_intents.contains(&self.purpose.use_intent)
            || self.retention_until_utc > terms.retention_until_utc
        {
            return Err(refuse(
                "purpose_terms",
                "exact supplied intent and supported retention horizon required",
            ));
        }
        self.envelope.validate(terms, &self.purpose)?;
        require_reach_within(
            &self.plan.selections[0].requested_reach,
            &self.envelope.reach,
        )?;
        validate_declared_reach(terms, &self.reach, true)?;
        require_subject_reach(
            &self.reach.input_paths,
            &self.purpose.subject_ref,
            &self.purpose.subject_fingerprint,
        )?;
        for root in &self.reach.owner_write_paths {
            let allowed = self
                .envelope
                .mutation_rules
                .iter()
                .filter(|r| r.effect == "allow")
                .any(|rule| {
                    crate::context_resolution::local_owner_subtree_within_selector(
                        &rule.selector,
                        root,
                    )
                    .unwrap_or(false)
                });
            let denied = self
                .envelope
                .mutation_rules
                .iter()
                .filter(|r| r.effect == "deny")
                .any(|rule| {
                    selector_intersects_owner_subtree(&rule.selector, root).unwrap_or(true)
                });
            if !allowed || denied {
                return Err(refuse("envelope_mutation","requested complete owner subtree is not permitted by the exact private envelope"));
            }
        }
        Ok(())
    }
}

pub fn local_evaluation_required_checkpoints_v1() -> Vec<LocalEvaluationCheckpointV1> {
    required_checkpoints()
}
fn required_checkpoints() -> Vec<LocalEvaluationCheckpointV1> {
    vec![
        LocalEvaluationCheckpointV1::Preparation,
        LocalEvaluationCheckpointV1::CollectionCommit,
        LocalEvaluationCheckpointV1::Launch,
        LocalEvaluationCheckpointV1::SourceAdmission,
        LocalEvaluationCheckpointV1::EvidenceAppend,
        LocalEvaluationCheckpointV1::FinalContext,
    ]
}
fn validate_selector(value: &str) -> Result<(), LocalAuthorityRefusalV1> {
    crate::ContextResolutionMutationRule::new(
        "evaluation-path",
        crate::ContextResolutionMutationEffect::Allow,
        "repository_path",
        value,
    )
    .map(|_| ())
    .map_err(kernel)
}
fn validate_paths(values: &[String]) -> Result<(), LocalAuthorityRefusalV1> {
    if values.len() > 64 || values.windows(2).any(|p| p[0] >= p[1]) {
        return Err(refuse(
            "path_reach",
            "sorted unique bounded declared paths required",
        ));
    }
    for value in values {
        validate_selector(value)?;
    }
    Ok(())
}
fn selector_contains(parent: &str, child: &str) -> bool {
    if parent == child {
        return true;
    }
    if !child.contains('*') {
        return crate::context_resolution::local_path_within_selector(parent, child)
            .unwrap_or(false);
    }
    parent.strip_suffix("/**").is_some_and(|prefix| {
        !prefix.contains('*') && (child == prefix || child.starts_with(&format!("{prefix}/")))
    })
}
fn require_read(selectors: &[String], path: &str) -> Result<(), LocalAuthorityRefusalV1> {
    if !selectors.iter().any(|selector| {
        crate::context_resolution::local_path_within_selector(selector, path).unwrap_or(false)
    }) {
        return Err(refuse(
            "required_input_reach",
            "actual owner proof-read path was omitted from supplied reach",
        ));
    }
    Ok(())
}
fn require_input_selector_coverage(
    declared: &[String],
    approved: &[String],
    selector: &str,
) -> Result<(), LocalAuthorityRefusalV1> {
    validate_selector(selector)?;
    if !declared
        .iter()
        .any(|parent| selector_contains(parent, selector))
    {
        return Err(refuse(
            "required_input_reach",
            "complete native proof-read selector was omitted from caller-declared reach",
        ));
    }
    if !approved
        .iter()
        .any(|parent| selector_contains(parent, selector))
    {
        return Err(refuse(
            "path_expansion",
            "complete native proof-read selector exceeds exact current approved read reach",
        ));
    }
    Ok(())
}
fn require_subtree(selectors: &[String], root: &str) -> Result<(), LocalAuthorityRefusalV1> {
    if !selectors.iter().any(|selector| {
        crate::context_resolution::local_owner_subtree_within_selector(selector, root)
            .unwrap_or(false)
    }) {
        return Err(refuse(
            "required_owner_subtree",
            "explicit complete consuming owner subtree required",
        ));
    }
    Ok(())
}
fn require_reach_within(
    requested: &LocalEvaluationReachV1,
    ceiling: &LocalEvaluationReachV1,
) -> Result<(), LocalAuthorityRefusalV1> {
    requested.validate()?;
    ceiling.validate()?;
    if requested.profile != ceiling.profile
        || requested.stack != ceiling.stack
        || requested.active_level_id != ceiling.active_level_id
    {
        return Err(refuse(
            "resolution_congruence",
            "exact profile/stack/level required for each selection",
        ));
    }
    let stack = profile()?.context_resolution().clone();
    let request = stack
        .ranks(
            &requested.active_level_id,
            requested.dimensions.each_ref().map(String::as_str),
        )
        .ok_or_else(|| refuse("resolution_stack", "request does not resolve"))?;
    let limit = stack
        .ranks(
            &ceiling.active_level_id,
            ceiling.dimensions.each_ref().map(String::as_str),
        )
        .ok_or_else(|| refuse("resolution_stack", "ceiling does not resolve"))?;
    if request.iter().zip(limit).any(|(a, b)| *a > b) {
        return Err(refuse(
            "resolution_expansion",
            "selection request exceeds exact admitted envelope",
        ));
    }
    Ok(())
}
fn require_exact_resolution_projection(
    requested: &LocalEvaluationReachV1,
    stack: &LocalBindingV1,
    dimension_ids: [&str; 6],
) -> Result<(), LocalAuthorityRefusalV1> {
    exact(stack)?;
    let prefixes = [
        "resolution.scope.",
        "resolution.detail.",
        "resolution.time.",
        "resolution.authority.",
        "resolution.memory.",
        "resolution.validation.",
    ];
    if stack != &requested.stack
        || dimension_ids.iter().enumerate().any(|(index, actual)| {
            *actual
                != format!(
                    "{}{}",
                    prefixes[index],
                    requested.dimensions[index].replace('_', "-")
                )
        })
    {
        return Err(refuse(
            "requested_resolution_congruence",
            "producer request stack and every dimension must equal the exact native admitted selection",
        ));
    }
    Ok(())
}
/// A complete literal owner root includes its own path and every descendant.
/// Any deny selector with a possible member in that population prevents whole
/// subtree admission, including one exact denied file beneath an allowed root.
fn selector_intersects_owner_subtree(
    selector: &str,
    root: &str,
) -> Result<bool, LocalAuthorityRefusalV1> {
    validate_selector(selector)?;
    validate_selector(root)?;
    if root.contains('*') {
        return Err(refuse(
            "required_owner_subtree",
            "literal owner root required",
        ));
    }
    let segments = selector.split('/').collect::<Vec<_>>();
    let root_segments = root.split('/').collect::<Vec<_>>();
    for (index, target) in root_segments.iter().enumerate() {
        let Some(pattern) = segments.get(index) else {
            return Ok(false);
        };
        if *pattern == "**" {
            return Ok(true);
        }
        if !crate::context_resolution::local_path_within_selector(pattern, target)
            .map_err(kernel)?
        {
            return Ok(false);
        }
    }
    Ok(true)
}
fn validate_declared_reach(
    terms: &LocalEvaluationTermsV1,
    reach: &LocalEvaluationPathReachV1,
    mutation: bool,
) -> Result<(), LocalAuthorityRefusalV1> {
    validate_paths(&reach.owner_write_paths)?;
    validate_paths(&reach.input_paths)?;
    if mutation
        && !reach
            .owner_write_paths
            .iter()
            .any(|r| r == LOCAL_EVALUATION_ROOT_V1)
    {
        return Err(refuse(
            "required_owner_subtree",
            "caller omitted complete Engine evaluation mutation root",
        ));
    }
    for root in &reach.owner_write_paths {
        require_subtree(&terms.allowed_paths, root)?;
    }
    for path in &reach.input_paths {
        if !terms
            .allowed_input_paths
            .iter()
            .any(|selector| selector_contains(selector, path))
        {
            return Err(refuse(
                "path_expansion",
                "caller declared proof-read reach exceeds supplied terms",
            ));
        }
    }
    for root in [
        ".handbook/local-charter/v1",
        ".handbook/local-operating/v1",
        ".handbook/local-operating/local-check-v1",
        ".handbook/admission-governance/v2",
    ] {
        require_subtree(&reach.input_paths, root)?;
    }
    for path in [
        ".handbook/project/charter.yaml",
        ".handbook/repository-identity.v1",
        ".handbook-initialization.v1.json",
        ".handbook-initialization-receipt.v1.json",
        crate::REPOSITORY_INITIALIZATION_LOCK_PATH_V1,
        ".handbook/state/locks/promotion.lock",
        ".handbook/state/locks/registry.lock",
        ".handbook/state/locks/lifecycle.lock",
        ".handbook/state/locks/local-operating.lock",
    ] {
        require_read(&reach.input_paths, path)?;
    }
    require_subtree(&reach.input_paths, LOCAL_EVALUATION_ROOT_V1)?;
    Ok(())
}

fn bounded_text(value: &str) -> Result<(), LocalAuthorityRefusalV1> {
    if value.trim().is_empty() || value.len() > 4096 || value.chars().any(char::is_control) {
        return Err(refuse("bounded_text", "bounded explicit text required"));
    }
    Ok(())
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct EvaluationHead {
    schema_ref: String,
    selected: String,
    revoked: Option<String>,
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct IssuanceIntent {
    schema_ref: String,
    locator: LocalEvaluationBindingLocatorV1,
    request: LocalEvaluationAuthorizationRequestV1,
    issuance_time_utc: String,
    operating_lock_fingerprint: String,
    authority_owner_identity: String,
}

impl LocalEvaluationAuthorityLocatorV1 {
    pub fn canonical_json_bytes(&self) -> Result<Vec<u8>, LocalAuthorityRefusalV1> {
        self.validate()?;
        encode(self)
    }
    pub fn from_json_bytes(bytes: &[u8]) -> Result<Self, LocalAuthorityRefusalV1> {
        let value: Self = decode(bytes)?;
        value.validate()?;
        Ok(value)
    }
    fn fingerprint_request(&self) -> Result<String, LocalAuthorityRefusalV1> {
        fingerprint(
            "handbook.local-evaluation-terms-request@1.0.0",
            &(
                &self.predecessor,
                &self.terms,
                &self.adoption_basis,
                &self.operating_lock_fingerprint,
                &self.lock_generations,
                &self.prepared_at_utc,
            ),
        )
    }
    fn validate(&self) -> Result<(), LocalAuthorityRefusalV1> {
        self.predecessor.canonical_json_bytes()?;
        self.terms.validate()?;
        check_fp(&self.operating_lock_fingerprint)?;
        check_fp(&self.adoption_basis.relation_fingerprint)?;
        check_fp(&self.adoption_basis.governance_tree_fingerprint)?;
        if self
            .lock_generations
            .iter()
            .map(|b| b.reference.as_str())
            .collect::<Vec<_>>()
            != evaluation_lock_paths()
        {
            return Err(refuse(
                "lock_generations",
                "complete exact original Engine lock-generation population required",
            ));
        }
        for binding in &self.lock_generations {
            check_fp(&binding.fingerprint)?;
        }
        horizon(&self.terms.retention_until_utc, &self.prepared_at_utc)?;
        if self.request_fingerprint != self.fingerprint_request()? {
            return Err(refuse(
                "terms_locator",
                "exact retained authority request required",
            ));
        }
        validate_terms_predecessor(&self.predecessor, &self.terms)
    }
}
impl LocalEvaluationBindingLocatorV1 {
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
        if !token(&self.request_key) {
            return Err(refuse(
                "issuance_locator",
                "exact original request key required",
            ));
        }
        check_fp(&self.request_fingerprint)?;
        check_fp(&self.authority_fingerprint)
    }
}
fn now() -> Result<String, LocalAuthorityRefusalV1> {
    crate::authority_admission::current_semantic_utc()
        .map_err(|_| refuse("semantic_time", "native semantic UTC unavailable"))
}
fn terms_adoption_reach(terms: &LocalEvaluationTermsV1) -> LocalEvaluationPathReachV1 {
    LocalEvaluationPathReachV1 {
        owner_write_paths: vec![LOCAL_EVALUATION_ROOT_V1.into()],
        input_paths: terms.allowed_input_paths.clone(),
    }
}
fn validate_terms_predecessor(
    predecessor: &LocalCheckLocatorV1,
    terms: &LocalEvaluationTermsV1,
) -> Result<(), LocalAuthorityRefusalV1> {
    predecessor.canonical_json_bytes()?;
    terms.validate()?;
    let prior = &predecessor.terms;
    if terms.principal != prior.principal
        || terms.agent != prior.agent
        || terms.revocation_principal != prior.revocation_principal
        || terms.expected_charter_fingerprint != prior.expected_charter_fingerprint
        || terms.constraint_inputs != prior.constraint_inputs
    {
        return Err(refuse("predecessor_terms","evaluation terms must preserve the exact supplied original Charter, actors and constraint lineage"));
    }
    require_reach_within(
        &LocalEvaluationReachV1::selected(&terms.ceiling)?,
        &LocalEvaluationReachV1::selected(&prior.ceiling)?,
    )?;
    for selector in &terms.allowed_paths {
        if !prior
            .allowed_paths
            .iter()
            .any(|parent| selector_contains(parent, selector))
        {
            return Err(refuse(
                "predecessor_path_expansion",
                "new evaluation terms cannot grant an owner subtree the predecessor omitted",
            ));
        }
    }
    for selector in &terms.allowed_input_paths {
        if !prior
            .allowed_input_paths
            .iter()
            .any(|parent| selector_contains(parent, selector))
        {
            return Err(refuse(
                "predecessor_input_expansion",
                "new evaluation terms cannot infer missing predecessor proof-read reach",
            ));
        }
    }
    require_subtree(&prior.allowed_paths, LOCAL_EVALUATION_ROOT_V1)?;
    require_subtree(&terms.allowed_paths, LOCAL_EVALUATION_ROOT_V1)
}
fn validate_formation(
    request: &LocalEvaluationFormationRequestV1,
    terms: &LocalEvaluationTermsV1,
) -> Result<(), LocalAuthorityRefusalV1> {
    bounded_text(&request.subject_ref)?;
    bounded_text(&request.objective)?;
    check_fp(&request.subject_fingerprint)?;
    check_fp(&request.scope_fingerprint)?;
    if !terms.allowed_use_intents.contains(&request.use_intent)
        || request.retention_until_utc > terms.retention_until_utc
    {
        return Err(refuse(
            "formation_terms",
            "exact permitted use intent and supplied supported retention horizon required",
        ));
    }
    horizon(&request.retention_until_utc, &now()?)?;
    require_reach_within(
        &request.requested_reach,
        &LocalEvaluationReachV1::selected(&terms.ceiling)?,
    )?;
    validate_declared_reach(terms, &request.reach, true)?;
    require_subject_reach(
        &request.reach.input_paths,
        &request.subject_ref,
        &request.subject_fingerprint,
    )
}
fn require_subject_reach(
    selectors: &[String],
    subject_ref: &str,
    subject_fingerprint: &str,
) -> Result<(), LocalAuthorityRefusalV1> {
    const CAPTURED_PREFIX: &str =
        "handbook.dock.captured-input@1.0.0#role/handbook.input.json-schema-validation/";
    if let Some(key) = subject_ref.strip_prefix(CAPTURED_PREFIX) {
        check_fp(subject_fingerprint)?;
        if subject_fingerprint.strip_prefix("sha256:") != Some(key) {
            return Err(refuse(
                "captured_subject",
                "captured subject ref and complete input fingerprint differ",
            ));
        }
        // A logical subject binding is not a repository path. Permission concerns
        // the actual native capture/source owner roots; Dock proves their exact
        // retained bytes independently at each consuming checkpoint.
        for root in [
            ".handbook/state/dock/v1",
            ".handbook/state/dock/local-evaluation-v1",
        ] {
            require_subtree(selectors, root)?;
        }
        Ok(())
    } else {
        require_read(selectors, subject_ref)
    }
}
fn require_declared_write(
    reach: &LocalEvaluationPathReachV1,
    root: &str,
) -> Result<(), LocalAuthorityRefusalV1> {
    validate_selector(root)?;
    if root.contains('*')
        || !reach
            .owner_write_paths
            .iter()
            .any(|declared| root == declared || root.starts_with(&format!("{declared}/")))
    {
        return Err(refuse(
            "required_owner_subtree",
            "consuming owner subtree was omitted from caller-declared mutation reach",
        ));
    }
    Ok(())
}
fn require_root(
    root: &RetainedRepositoryRootV1,
    locator: &LocalEvaluationAuthorityLocatorV1,
) -> Result<(), LocalAuthorityRefusalV1> {
    if root
        .governance_physical_root_fingerprint_v1()
        .map_err(fs_error)?
        != locator
            .predecessor
            .predecessor
            .charter
            .binding
            .initialization_claim
            .physical_root_fingerprint()
    {
        return Err(refuse(
            "wrong_root",
            "evaluation admission is bound to its original actual native repository root",
        ));
    }
    Ok(())
}
fn require_evaluation_lock(
    root: &RetainedRepositoryRootV1,
    locator: &LocalEvaluationAuthorityLocatorV1,
) -> Result<(), LocalAuthorityRefusalV1> {
    require_root(root, locator)?;
    if root
        .existing_lock_physical_fingerprint_v1(".handbook/state/locks/local-operating.lock")
        .map_err(fs_error)?
        != locator.operating_lock_fingerprint
    {
        return Err(refuse(
            "operating_lock_substituted",
            "original exact operating lock generation is required",
        ));
    }
    for binding in &locator.lock_generations {
        if root
            .existing_lock_physical_fingerprint_v1(&binding.reference)
            .map_err(fs_error)?
            != binding.fingerprint
        {
            return Err(refuse(
                "engine_lock_substituted",
                "an original exact Engine lock generation was substituted",
            ));
        }
    }
    Ok(())
}
fn evaluation_lock_paths() -> Vec<&'static str> {
    vec![
        crate::REPOSITORY_INITIALIZATION_LOCK_PATH_V1,
        ".handbook/state/locks/lifecycle.lock",
        ".handbook/state/locks/local-operating.lock",
        ".handbook/state/locks/promotion.lock",
        ".handbook/state/locks/registry.lock",
    ]
}
fn authority_path(locator: &LocalEvaluationAuthorityLocatorV1, file: &str) -> String {
    format!(
        "{LOCAL_EVALUATION_ROOT_V1}/authority/{}/{file}",
        locator.terms.request_key
    )
}
fn issuance_path(locator: &LocalEvaluationBindingLocatorV1, file: &str) -> String {
    format!(
        "{LOCAL_EVALUATION_ROOT_V1}/issuances/{}/{file}",
        locator.request_key
    )
}
fn authority_paths(locator: &LocalEvaluationAuthorityLocatorV1) -> Vec<String> {
    let mut values = vec![
        format!("{LOCAL_EVALUATION_ROOT_V1}/current.json"),
        authority_path(locator, "request.json"),
        authority_path(locator, "outcome.json"),
        authority_path(locator, "terminal.json"),
    ];
    values.sort();
    values
}
fn issuance_paths(locator: &LocalEvaluationBindingLocatorV1) -> Vec<String> {
    let mut values = vec![
        issuance_path(locator, "request.json"),
        issuance_path(locator, "outcome.json"),
        issuance_path(locator, "terminal.json"),
    ];
    values.sort();
    values
}
fn expected_authority(
    locator: &LocalEvaluationAuthorityLocatorV1,
) -> Result<LocalEvaluationAuthorityOutcomeV1, LocalAuthorityRefusalV1> {
    Ok(LocalEvaluationAuthorityOutcomeV1 {
        schema_ref: LOCAL_EVALUATION_RULE_V1.into(),
        locator: locator.clone(),
        outcome_fingerprint: fingerprint(
            "handbook.local-evaluation-terms-outcome@1.0.0",
            &(locator, "local_advisory"),
        )?,
        assurance: "local_advisory".into(),
    })
}
fn history_authority(
    root: &RetainedRepositoryRootV1,
    locator: &LocalEvaluationAuthorityLocatorV1,
) -> Result<LocalEvaluationAuthorityOutcomeV1, LocalAuthorityRefusalV1> {
    require_evaluation_lock(root, locator)?;
    local_charter::local_evaluation_historical_dependencies(
        root,
        &locator.predecessor.predecessor.charter,
    )?;
    read_history(root, &locator.predecessor.predecessor)?;
    let predecessor = local_check::evaluation_predecessor_history(root, &locator.predecessor)?;
    if predecessor.outcome_fingerprint != locator.terms.expected_predecessor_fingerprint {
        return Err(refuse(
            "owner_history",
            "exact original predecessor history required",
        ));
    }
    let expected = expected_authority(locator)?;
    if found(root, &authority_path(locator, "request.json"))? != encode(locator)?
        || found(root, &authority_path(locator, "outcome.json"))? != encode(&expected)?
        || found(root, &authority_path(locator, "terminal.json"))? != encode(&expected)?
    {
        return Err(refuse(
            "owner_history",
            "exact retained evaluation terms history is incomplete or substituted",
        ));
    }
    Ok(expected)
}
fn current_authority(
    root: &RetainedRepositoryRootV1,
    locator: &LocalEvaluationAuthorityLocatorV1,
    basis: &LocalCheckCurrentBasisV1,
) -> Result<LocalEvaluationAuthorityOutcomeV1, LocalAuthorityRefusalV1> {
    require_evaluation_lock(root, locator)?;
    local_check::evaluation_predecessor_current(root, &locator.predecessor, basis)?;
    let value = history_authority(root, locator)?;
    let head: EvaluationHead = decode(&found(
        root,
        &format!("{LOCAL_EVALUATION_ROOT_V1}/current.json"),
    )?)?;
    if head.schema_ref != LOCAL_EVALUATION_RULE_V1
        || head.selected != value.outcome_fingerprint
        || head.revoked.is_some()
        || root
            .read_local_owner_optional_bounded(
                &format!("{LOCAL_EVALUATION_ROOT_V1}/revoke.json"),
                MAX,
            )
            .map_err(fs_error)?
            != RepositoryReadV1::Absent
    {
        return Err(refuse(
            "evaluation_not_current",
            "evaluation authority selection is missing, revoked or changed",
        ));
    }
    Ok(value)
}
fn authority_execution(
    result: Result<LocalEvaluationAuthorityOutcomeV1, LocalAuthorityRefusalV1>,
    disposition: LocalAuthorityDispositionV1,
) -> LocalEvaluationAuthorityExecutionV1 {
    match result {
        Ok(outcome) => LocalEvaluationAuthorityExecutionV1 {
            disposition,
            outcome: Some(outcome),
            refusal: None,
        },
        Err(refusal) => LocalEvaluationAuthorityExecutionV1 {
            disposition: if disposition == LocalAuthorityDispositionV1::Committed {
                LocalAuthorityDispositionV1::Uncertain
            } else {
                LocalAuthorityDispositionV1::Refused
            },
            outcome: None,
            refusal: Some(refusal),
        },
    }
}
fn issuance_execution(
    locator: LocalEvaluationBindingLocatorV1,
    result: Result<LocalEvaluationAuthorizationBindingV1, LocalAuthorityRefusalV1>,
    disposition: LocalAuthorityDispositionV1,
) -> LocalEvaluationAuthorizationExecutionV1 {
    match result {
        Ok(binding) => LocalEvaluationAuthorizationExecutionV1 {
            disposition,
            locator,
            binding: Some(binding),
            refusal: None,
        },
        Err(refusal) => LocalEvaluationAuthorizationExecutionV1 {
            disposition: if disposition == LocalAuthorityDispositionV1::Committed
                || refusal.code == "issuance_pending"
            {
                LocalAuthorityDispositionV1::Uncertain
            } else {
                LocalAuthorityDispositionV1::Refused
            },
            locator,
            binding: None,
            refusal: Some(refusal),
        },
    }
}
fn adopt_evaluation(
    repo: &Path,
    locator: &LocalEvaluationAuthorityLocatorV1,
    resume: bool,
    interrupt: impl FnOnce(),
) -> LocalEvaluationAuthorityExecutionV1 {
    let mut effects = false;
    let mut replay = false;
    let result = (|| {
        locator.validate()?;
        validate_declared_reach(&locator.terms, &terms_adoption_reach(&locator.terms), true)?;
        let guard = OperatingGuard::acquire(repo, &locator.predecessor.predecessor, false)?;
        let root = guard.root();
        require_evaluation_lock(root, locator)?;
        let request = authority_path(locator, "request.json");
        let terminal = authority_path(locator, "terminal.json");
        let pending = authority_path(locator, "pending-terminal.json");
        if root
            .read_local_owner_optional_bounded(&terminal, MAX)
            .map_err(fs_error)?
            != RepositoryReadV1::Absent
        {
            replay = true;
            return history_authority(root, locator);
        }
        let before = local_check::evaluation_predecessor_current(
            root,
            &locator.predecessor,
            &locator.adoption_basis,
        )?;
        if before.outcome_fingerprint != locator.terms.expected_predecessor_fingerprint {
            return Err(refuse(
                "predecessor_terms",
                "exact live predecessor required",
            ));
        }
        horizon(&locator.terms.retention_until_utc, &now()?)?;
        match root
            .read_local_owner_optional_bounded(&request, MAX)
            .map_err(fs_error)?
        {
            RepositoryReadV1::Found(bytes) if bytes == encode(locator)? => {
                if !resume {
                    return Err(refuse(
                        "original_request_pending",
                        "resume only the exact original retained terms intent",
                    ));
                }
            }
            RepositoryReadV1::Found(_) => {
                return Err(refuse(
                    "request_conflict",
                    "same request key has different retained terms content",
                ))
            }
            RepositoryReadV1::Absent if resume => {
                return Err(refuse(
                    "original_request_absent",
                    "portable locator cannot create absent authority intent",
                ))
            }
            RepositoryReadV1::Absent => {
                if root
                    .list_directory_bounded(LOCAL_EVALUATION_ROOT_V1, 4096)
                    .is_ok()
                {
                    return Err(refuse(
                        "already_selected",
                        "only one explicit compatible evaluation terms selection is supported",
                    ));
                }
                root.create_directories(&format!(
                    "{LOCAL_EVALUATION_ROOT_V1}/authority/{}",
                    locator.terms.request_key
                ))
                .map_err(fs_error)?;
                effects = true;
                write_exact(root, &request, &encode(locator)?)?;
            }
        }
        let outcome = expected_authority(locator)?;
        let bytes = encode(&outcome)?;
        if root
            .read_local_owner_optional_bounded(&authority_path(locator, "outcome.json"), MAX)
            .map_err(fs_error)?
            != RepositoryReadV1::Absent
            && root
                .read_local_owner_optional_bounded(&pending, MAX)
                .map_err(fs_error)?
                == RepositoryReadV1::Absent
        {
            return Err(refuse(
                "required_terminal_state_missing",
                "missing terminal cannot be invented from an outcome",
            ));
        }
        effects = true;
        write_exact(root, &authority_path(locator, "outcome.json"), &bytes)?;
        write_exact(root, &pending, &bytes)?;
        interrupt();
        guard.validate()?;
        local_check::evaluation_predecessor_current(
            root,
            &locator.predecessor,
            &locator.adoption_basis,
        )?;
        write_exact(
            root,
            &format!("{LOCAL_EVALUATION_ROOT_V1}/current.json"),
            &encode(&EvaluationHead {
                schema_ref: LOCAL_EVALUATION_RULE_V1.into(),
                selected: outcome.outcome_fingerprint.clone(),
                revoked: None,
            })?,
        )?;
        root.rename_noreplace_durable(&pending, &terminal)
            .map_err(fs_error)?;
        guard.validate()?;
        current_authority(root, locator, &locator.adoption_basis)
    })();
    authority_execution(
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
fn expected_binding(
    intent: &IssuanceIntent,
) -> Result<LocalEvaluationAuthorizationBindingV1, LocalAuthorityRefusalV1> {
    let request = &intent.request;
    let mut value = LocalEvaluationAuthorizationBindingV1 {
        schema_ref: LOCAL_EVALUATION_RULE_V1.into(),
        authority_owner_identity: intent.authority_owner_identity.clone(),
        issuance_fingerprint: String::new(),
        opaque_lineage: fingerprint("handbook.local-evaluation-private-issuance@1.0.0", intent)?,
        purpose_fingerprint: request.purpose.fingerprint()?,
        plan_fingerprint: request.plan.fingerprint()?,
        occurrence_identity: request.purpose.occurrence_identity.clone(),
        subject_fingerprint: request.purpose.subject_fingerprint.clone(),
        scope_fingerprint: request.purpose.scope_fingerprint.clone(),
        objective_fingerprint: fingerprint(
            "handbook.local-evaluation-objective@1.0.0",
            &request.purpose.objective,
        )?,
        use_intent: request.purpose.use_intent,
        selection_fingerprint: request.plan.selections[0].fingerprint()?,
        requested_reach: request.plan.selections[0].requested_reach.clone(),
        envelope_fingerprint: request.envelope.fingerprint()?,
        checkpoint_rule_ref: LOCAL_EVALUATION_RULE_V1.into(),
        issuance_time_utc: intent.issuance_time_utc.clone(),
        retention_until_utc: request.retention_until_utc.clone(),
        request_within_envelope: true,
    };
    value.issuance_fingerprint = fingerprint("handbook.local-evaluation-binding@1.0.0", &value)?;
    Ok(value)
}
fn read_intent(
    root: &RetainedRepositoryRootV1,
    authority: &LocalEvaluationAuthorityLocatorV1,
    locator: &LocalEvaluationBindingLocatorV1,
    reach: &LocalEvaluationPathReachV1,
) -> Result<IssuanceIntent, LocalAuthorityRefusalV1> {
    require_read(&reach.input_paths, &issuance_path(locator, "request.json"))?;
    let bytes = found(root, &issuance_path(locator, "request.json"))?;
    let value: IssuanceIntent = decode(&bytes)?;
    value.request.validate_under(&authority.terms)?;
    horizon(&value.request.retention_until_utc, &value.issuance_time_utc)?;
    let selected = expected_authority(authority)?;
    if value.schema_ref != LOCAL_EVALUATION_RULE_V1
        || &value.locator != locator
        || value.locator.authority_fingerprint != selected.outcome_fingerprint
        || value.locator.request_fingerprint != value.request.fingerprint()?
        || value.operating_lock_fingerprint != authority.operating_lock_fingerprint
        || value.authority_owner_identity != expected_owner_identity(authority)?
        || value.issuance_time_utc < value.request.purpose.formed_at_utc
        || value.issuance_time_utc < authority.prepared_at_utc
        || value.request.purpose.project_identity
            != authority
                .predecessor
                .predecessor
                .charter
                .binding
                .initialization_claim
                .proposed_repository_identity()
        || bytes != encode(&value)?
    {
        return Err(refuse(
            "issuance_conflict",
            "immutable native issuance intent disagrees with exact original authority/request",
        ));
    }
    Ok(value)
}
fn expected_owner_identity(
    authority: &LocalEvaluationAuthorityLocatorV1,
) -> Result<String, LocalAuthorityRefusalV1> {
    fingerprint(
        "handbook.engine-local-evaluation-owner@1.0.0",
        &(
            authority
                .predecessor
                .predecessor
                .charter
                .binding
                .initialization_claim
                .proposed_repository_identity(),
            &authority.operating_lock_fingerprint,
            LOCAL_EVALUATION_RULE_V1,
        ),
    )
}
fn read_issuance(
    root: &RetainedRepositoryRootV1,
    authority: &LocalEvaluationAuthorityLocatorV1,
    locator: &LocalEvaluationBindingLocatorV1,
    reach: &LocalEvaluationPathReachV1,
) -> Result<
    (
        LocalEvaluationAuthorizationRequestV1,
        LocalEvaluationAuthorizationBindingV1,
    ),
    LocalAuthorityRefusalV1,
> {
    history_authority(root, authority)?;
    let value = read_intent(root, authority, locator, reach)?;
    let expected = expected_binding(&value)?;
    for file in ["outcome.json", "terminal.json"] {
        let path = issuance_path(locator, file);
        require_read(&reach.input_paths, &path)?;
        match root.read_local_owner_optional_bounded(&path,MAX).map_err(fs_error)? {
            RepositoryReadV1::Found(bytes) if bytes==encode(&expected)?=>{},
            RepositoryReadV1::Absent=>return Err(refuse("issuance_pending","retained original issuance has no complete terminal; exact recovery cannot claim success")),
            _=>return Err(refuse("issuance_conflict","original immutable issuance output or terminal was substituted")),
        }
    }
    Ok((value.request, expected))
}
fn issue_evaluation(
    repo: &Path,
    authority: &LocalEvaluationAuthorityLocatorV1,
    request: &LocalEvaluationAuthorizationRequestV1,
    resume: bool,
    interrupt: impl FnOnce(),
) -> LocalEvaluationAuthorizationExecutionV1 {
    let locator = match NativeLocalEvaluationAuthorityV1::issuance_locator(authority, request) {
        Ok(value) => value,
        Err(error) => {
            return issuance_execution(
                LocalEvaluationBindingLocatorV1 {
                    request_key: request.request_key.clone(),
                    request_fingerprint: String::new(),
                    authority_fingerprint: String::new(),
                },
                Err(error),
                LocalAuthorityDispositionV1::Refused,
            )
        }
    };
    let mut effects = false;
    let mut replay = false;
    let result = (|| {
        let guard = OperatingGuard::acquire(repo, &authority.predecessor.predecessor, false)?;
        let root = guard.root();
        require_evaluation_lock(root, authority)?;
        let intent_path = issuance_path(&locator, "request.json");
        let terminal = issuance_path(&locator, "terminal.json");
        let pending = issuance_path(&locator, "pending-terminal.json");
        if root
            .read_local_owner_optional_bounded(&terminal, MAX)
            .map_err(fs_error)?
            != RepositoryReadV1::Absent
        {
            replay = true;
            return read_issuance(root, authority, &locator, &request.reach)
                .map(|(_, binding)| binding);
        }
        current_authority(root, authority, &request.current_basis)?;
        request.validate_under(&authority.terms)?;
        let issuance_now = now()?;
        horizon(&request.retention_until_utc, &issuance_now)?;
        if request.purpose.formed_at_utc > issuance_now {
            return Err(refuse(
                "purpose_time",
                "durable Purpose formation must precede native issuance",
            ));
        }
        if request.purpose.project_identity
            != authority
                .predecessor
                .predecessor
                .charter
                .binding
                .initialization_claim
                .proposed_repository_identity()
        {
            return Err(refuse(
                "project_identity",
                "Purpose belongs to a different exact native project identity",
            ));
        }
        let intent = match root
            .read_local_owner_optional_bounded(&intent_path, MAX)
            .map_err(fs_error)?
        {
            RepositoryReadV1::Found(_) => {
                let intent = read_intent(root, authority, &locator, &request.reach)?;
                if &intent.request != request {
                    return Err(refuse(
                        "request_conflict",
                        "request key conflicts with immutable original Purpose closure",
                    ));
                }
                if !resume {
                    return Err(refuse(
                        "issuance_pending",
                        "retained issuance intent must use its exact recovery route",
                    ));
                }
                intent
            }
            RepositoryReadV1::Absent if resume => {
                return Err(refuse(
                    "original_request_absent",
                    "exact recovery cannot form missing issuance intent",
                ))
            }
            RepositoryReadV1::Absent => {
                let issuance_time_utc = issuance_now;
                horizon(&request.retention_until_utc, &issuance_time_utc)?;
                let value = IssuanceIntent {
                    schema_ref: LOCAL_EVALUATION_RULE_V1.into(),
                    locator: locator.clone(),
                    request: request.clone(),
                    issuance_time_utc,
                    operating_lock_fingerprint: authority.operating_lock_fingerprint.clone(),
                    authority_owner_identity: expected_owner_identity(authority)?,
                };
                root.create_directories(&format!(
                    "{LOCAL_EVALUATION_ROOT_V1}/issuances/{}",
                    locator.request_key
                ))
                .map_err(fs_error)?;
                effects = true;
                write_exact(root, &intent_path, &encode(&value)?)?;
                value
            }
        };
        let value = expected_binding(&intent)?;
        let bytes = encode(&value)?;
        if root
            .read_local_owner_optional_bounded(&issuance_path(&locator, "outcome.json"), MAX)
            .map_err(fs_error)?
            != RepositoryReadV1::Absent
            && root
                .read_local_owner_optional_bounded(&pending, MAX)
                .map_err(fs_error)?
                == RepositoryReadV1::Absent
        {
            return Err(refuse(
                "required_terminal_state_missing",
                "lost original issuance terminal cannot be recreated",
            ));
        }
        effects = true;
        write_exact(root, &issuance_path(&locator, "outcome.json"), &bytes)?;
        write_exact(root, &pending, &bytes)?;
        interrupt();
        guard.validate()?;
        current_authority(root, authority, &request.current_basis)?;
        root.rename_noreplace_durable(&pending, &terminal)
            .map_err(fs_error)?;
        guard.validate()?;
        read_issuance(root, authority, &locator, &request.reach).map(|(_, binding)| binding)
    })();
    issuance_execution(
        locator,
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
fn validate_checkpoint(
    checkpoint: LocalEvaluationCheckpointV1,
    target: Option<&str>,
) -> Result<(), LocalAuthorityRefusalV1> {
    if checkpoint != LocalEvaluationCheckpointV1::Preparation && target.is_none() {
        return Err(refuse(
            "checkpoint_target",
            "named consuming checkpoint requires its exact basis or fixed set target",
        ));
    }
    if let Some(target) = target {
        check_fp(target)?;
    }
    Ok(())
}
fn require_unsuperseded(
    root: &RetainedRepositoryRootV1,
    locator: &LocalEvaluationBindingLocatorV1,
) -> Result<(), LocalAuthorityRefusalV1> {
    if root
        .read_local_owner_optional_bounded(&issuance_path(locator, "supersession.json"), MAX)
        .map_err(fs_error)?
        != RepositoryReadV1::Absent
    {
        return Err(refuse(
            "supersession_unverifiable",
            "an unrecognized supersession footprint is not a supported immutable Engine relation",
        ));
    }
    Ok(())
}
fn retain_verification(
    root: &RetainedRepositoryRootV1,
    binding: &LocalEvaluationAuthorizationBindingV1,
    checkpoint: LocalEvaluationCheckpointV1,
    target: Option<&str>,
    source_fingerprint: Option<&str>,
    basis: &LocalCheckCurrentBasisV1,
    current: &Result<(), LocalAuthorityRefusalV1>,
) -> Result<LocalEvaluationVerificationObservationV1, LocalAuthorityRefusalV1> {
    let mut value = LocalEvaluationVerificationObservationV1 {
        schema_ref: LOCAL_EVALUATION_RULE_V1.into(),
        observation_fingerprint: String::new(),
        issuance_fingerprint: binding.issuance_fingerprint.clone(),
        purpose_fingerprint: binding.purpose_fingerprint.clone(),
        checkpoint,
        checkpoint_target: target.map(str::to_owned),
        checkpoint_source_fingerprint: source_fingerprint.map(str::to_owned),
        current_basis: basis.clone(),
        status: match current {
            Ok(()) => LocalEvaluationCurrentStatusV1::Current,
            Err(error) if error.code == "authorization_superseded" => {
                LocalEvaluationCurrentStatusV1::Superseded
            }
            Err(error)
                if [
                    "evaluation_not_current",
                    "revoked_or_stale_head",
                    "predecessor_revoked_or_lost",
                    "governance_currentness",
                    "purpose_currentness",
                ]
                .contains(&error.code.as_str()) =>
            {
                LocalEvaluationCurrentStatusV1::NotCurrent
            }
            Err(_) => LocalEvaluationCurrentStatusV1::Unverifiable,
        },
        observed_at_utc: now()?,
        reason_code: current.as_ref().err().map(|e| e.code.clone()),
        reason: current.as_ref().err().map(|e| e.detail.clone()),
    };
    value.observation_fingerprint = fingerprint(
        "handbook.local-evaluation-current-observation@1.0.0",
        &value,
    )?;
    let key = value
        .observation_fingerprint
        .strip_prefix("sha256:")
        .ok_or_else(|| refuse("fingerprint", "exact observation fingerprint required"))?;
    root.create_directories(&format!("{LOCAL_EVALUATION_ROOT_V1}/checks"))
        .map_err(fs_error)?;
    write_exact(
        root,
        &format!("{LOCAL_EVALUATION_ROOT_V1}/checks/{key}.json"),
        &encode(&value)?,
    )?;
    Ok(value)
}

impl LocalEvaluationFinalSetSourceV1 {
    pub fn validate(
        &self,
        request: &LocalEvaluationAuthorizationRequestV1,
    ) -> Result<(), LocalAuthorityRefusalV1> {
        check_fp(&self.record_fingerprint)?;
        check_fp(&self.set_fingerprint)?;
        let value = crate::parse_schema_json(&self.canonical_record_bytes).map_err(codec)?;
        if encode(&value)? != self.canonical_record_bytes
            || DefinitionFingerprint::from_bytes(&self.canonical_record_bytes).as_str()
                != self.record_fingerprint
        {
            return Err(refuse(
                "final_set_source",
                "exact byte-complete canonical duplicate-safe native set snapshot required",
            ));
        }
        let purpose: LocalEvaluationPurposeV1 =
            serde_json::from_value(value.get("purpose").cloned().ok_or_else(|| {
                refuse("final_set_source", "Purpose missing from native set record")
            })?)
            .map_err(codec)?;
        let plan: LocalEvaluationPlanV1 =
            serde_json::from_value(value.get("plan").cloned().ok_or_else(|| {
                refuse("final_set_source", "Plan missing from native set record")
            })?)
            .map_err(codec)?;
        let at = |pointer: &str| value.pointer(pointer).and_then(serde_json::Value::as_str);
        let identity = at("/identity/fingerprint").ok_or_else(|| {
            refuse(
                "final_set_source",
                "native immutable set-state identity missing",
            )
        })?;
        check_fp(identity)?;
        let key = identity
            .strip_prefix("sha256:")
            .ok_or_else(|| refuse("fingerprint", "native state fingerprint required"))?;
        if self.relative_path
            != format!(".handbook/state/contracts/local-evaluation-v3/records/{key}.json")
            || at("/stage") != Some("set_closed")
            || purpose != request.purpose
            || plan != request.plan
            || at("/locator/occurrence/fingerprint")
                != Some(request.purpose.occurrence_identity.as_str())
            || at("/evidence_set/identity/fingerprint") != Some(self.set_fingerprint.as_str())
            || at("/evidence_set/occurrence/fingerprint")
                != Some(request.purpose.occurrence_identity.as_str())
            || at("/committed_basis/fingerprint") != at("/evidence_set/basis/fingerprint")
            || at("/committed_basis/fingerprint").is_none()
            || at("/evidence_set/formation_policy/fingerprint")
                != Some(request.purpose.formation_policy_fingerprint.as_str())
            || at("/original_input/formation/retention_until_utc")
                != Some(request.retention_until_utc.as_str())
        {
            return Err(refuse("final_set_congruence","final verification requires the exact original Purpose, Plan, occurrence, committed basis, policy and already closed native set"));
        }
        check_fp(
            at("/committed_basis/fingerprint")
                .ok_or_else(|| refuse("final_set_source", "committed basis absent"))?,
        )?;
        let closed = at("/evidence_set/closed_at")
            .ok_or_else(|| refuse("final_set_source", "set closure semantic time absent"))?;
        utc(closed)?;
        if closed < request.purpose.formed_at_utc.as_str() || closed > now()?.as_str() {
            return Err(refuse(
                "final_set_time",
                "set closure must precede the exact final Engine verification",
            ));
        }
        Ok(())
    }
}
fn pin_final_set_source<'a>(
    root: &'a RetainedRepositoryRootV1,
    source: &LocalEvaluationFinalSetSourceV1,
    request: &LocalEvaluationAuthorizationRequestV1,
    reach: &LocalEvaluationPathReachV1,
) -> Result<RepositoryExactFilesCheckpointV1<'a>, LocalAuthorityRefusalV1> {
    source.validate(request)?;
    require_read(&reach.input_paths, &source.relative_path)?;
    let files = root
        .retain_exact_files_checkpoint_v1(&[source.relative_path.as_str()])
        .map_err(fs_error)?;
    files
        .require_exact_file(root, &source.relative_path, &source.canonical_record_bytes)
        .map_err(fs_error)?;
    files.validate().map_err(fs_error)?;
    Ok(files)
}
fn check_fp(value: &str) -> Result<(), LocalAuthorityRefusalV1> {
    DefinitionFingerprint::parse(value)
        .map(|_| ())
        .map_err(|_| refuse("fingerprint", "exact fingerprint required"))
}
fn utc(value: &str) -> Result<(), LocalAuthorityRefusalV1> {
    crate::charter_posture_transaction_intent_v1::validate_utc_second_v1(
        value,
        "local evaluation time",
    )
    .map_err(|_| {
        refuse(
            "semantic_time",
            "canonical UTC-second semantic time required",
        )
    })
}
fn horizon(value: &str, basis: &str) -> Result<(), LocalAuthorityRefusalV1> {
    utc(value)?;
    utc(basis)?;
    if value < basis {
        return Err(refuse(
            "retention_horizon",
            "supplied retention horizon precedes its semantic basis",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests;
