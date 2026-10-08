//! Bounded caller-carried InputCorrection decisions. Exact native records are
//! historical data; only the lifetime-sealed admission permits dependent use.
use super::*;

pub const LOCAL_INPUT_CORRECTION_RULE_V1: &str = "handbook.local-input-correction@1.0.0";
pub const LOCAL_INPUT_CORRECTION_ROOT_V1: &str =
    ".handbook/local-operating/local-evaluation-v1/input-correction-v1";
pub const LOCAL_INPUT_CORRECTION_CANDIDATE_SCHEMA_V1: &str =
    "handbook.local-input-correction-candidate@1.0.0";
const CANDIDATE_ROOT: &str =
    ".handbook/state/contracts/local-evaluation-v3/input-correction-candidates";
const ORIGINAL_STATE_DOMAIN: &str = "handbook.contract-local-evaluation-state@3.0.0";
const ORIGINAL_STATE_ROOT: &str = ".handbook/state/contracts/local-evaluation-v3/records";

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalInputCorrectionDelegationV1 {
    pub request_key: String,
    pub principal: String,
    pub decision_agent: String,
    pub executor: String,
    pub approval_source_ref: String,
    pub human_approval_asserted: bool,
    pub approved_content_fingerprint: String,
    pub expected_initial_fingerprint: String,
    pub expected_local_check_fingerprint: String,
    pub expected_evaluation_fingerprint: String,
    pub expected_charter_fingerprint: String,
    pub decision_class: LocalDecisionClassV1,
    pub assurance: String,
    pub requested_reach: LocalEvaluationReachV1,
    pub constraint_inputs: Vec<LocalBindingV1>,
    pub reach: LocalEvaluationPathReachV1,
    pub retention_until_utc: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalInputCorrectionDelegationLocatorV1 {
    pub delegation: LocalInputCorrectionDelegationV1,
    pub authority_fingerprint: String,
    pub adoption_basis: LocalCheckCurrentBasisV1,
    pub prepared_at_utc: String,
    pub request_fingerprint: String,
}
pub struct PreparedLocalInputCorrectionDelegationV1 {
    locator: LocalInputCorrectionDelegationLocatorV1,
}
impl PreparedLocalInputCorrectionDelegationV1 {
    pub fn locator(&self) -> &LocalInputCorrectionDelegationLocatorV1 {
        &self.locator
    }
}
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalInputCorrectionDelegationOutcomeV1 {
    pub schema_ref: String,
    pub locator: LocalInputCorrectionDelegationLocatorV1,
    pub delegation_fingerprint: String,
}
#[derive(Debug, Serialize)]
pub struct LocalInputCorrectionExecutionV1<T> {
    pub disposition: LocalAuthorityDispositionV1,
    pub outcome: Option<T>,
    pub refusal: Option<LocalAuthorityRefusalV1>,
}

/// Contracts retains this projection only after reproducing the actual rejected
/// old-subject/new-input candidate and preparing its consistent correction.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalInputCorrectionCandidateV1 {
    pub schema_ref: String,
    pub owner_identity: String,
    pub project_identity: String,
    pub original_result: LocalBindingV1,
    pub rejected_candidate_fingerprint: String,
    pub missing_condition: String,
    pub original_input_fingerprint: String,
    pub proposed_input_fingerprint: String,
    pub formation_request_key: String,
    pub formation_request_fingerprint: String,
    pub original_state_path: String,
    pub original_state_bytes_fingerprint: String,
    pub original_state_identity: LocalBindingV1,
    pub rejected_candidate: serde_json::Value,
    pub prospective_candidate: serde_json::Value,
    pub original_capture: serde_json::Value,
    pub proposed_capture: serde_json::Value,
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalInputCorrectionCandidateSourceV1 {
    pub relative_path: String,
    pub canonical_record_bytes: Vec<u8>,
    pub record_fingerprint: String,
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalInputCorrectionDependencySourceV1 {
    pub relative_path: String,
    pub canonical_record_bytes: Vec<u8>,
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalInputCorrectionRequestV1 {
    pub request_key: String,
    pub submitter: String,
    pub context_source_ref: String,
    pub additional_context: String,
    pub candidate_source: LocalInputCorrectionCandidateSourceV1,
    pub formation: LocalEvaluationFormationRequestV1,
    pub current_basis: LocalCheckCurrentBasisV1,
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalInputCorrectionRequestLocatorV1 {
    pub request_key: String,
    pub request_fingerprint: String,
    pub delegation_fingerprint: String,
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalInputCorrectionRequestRecordV1 {
    pub schema_ref: String,
    pub locator: LocalInputCorrectionRequestLocatorV1,
    pub request: LocalInputCorrectionRequestV1,
    pub head_fingerprint: String,
    pub retained_at_utc: String,
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalInputCorrectionDecisionV1 {
    Approve,
    Deny,
    Clarify,
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalInputCorrectionSubmissionV1 {
    pub submission_key: String,
    pub expected_head: String,
    pub candidate_choice_fingerprint: String,
    pub deciding_actor: String,
    pub submitter: String,
    pub decision_source_ref: String,
    pub assurance: String,
    pub outcome: LocalInputCorrectionDecisionV1,
    pub reason: String,
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalInputCorrectionDecisionLocatorV1 {
    pub request_key: String,
    pub request_fingerprint: String,
    pub submission_key: String,
    pub submission_fingerprint: String,
}
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LocalInputCorrectionDecisionRecordV1 {
    pub schema_ref: String,
    pub locator: LocalInputCorrectionDecisionLocatorV1,
    pub submission: LocalInputCorrectionSubmissionV1,
    pub request_fingerprint: String,
    pub decision_fingerprint: String,
    pub recorded_at_utc: String,
}

impl LocalInputCorrectionCandidateV1 {
    pub fn canonical_json_bytes(&self) -> Result<Vec<u8>, LocalAuthorityRefusalV1> {
        if self.schema_ref != LOCAL_INPUT_CORRECTION_CANDIDATE_SCHEMA_V1
            || self.missing_condition != "subject_case_input_congruence"
            || self.original_input_fingerprint == self.proposed_input_fingerprint
            || !token(&self.formation_request_key)
        {
            return Err(refuse("correction_candidate", "actual changed-capture subject/case inconsistency and one fixed correction request required"));
        }
        bounded_text(&self.owner_identity)?;
        bounded_text(&self.project_identity)?;
        exact(&self.original_result)?;
        exact(&self.original_state_identity)?;
        check_fp(&self.original_state_bytes_fingerprint)?;
        if self.original_state_path
            != format!(
                "{ORIGINAL_STATE_ROOT}/{}.json",
                &self.original_state_identity.fingerprint[7..]
            )
        {
            return Err(refuse(
                "correction_original_source",
                "exact immutable original closed Contracts state path required",
            ));
        }
        for value in [
            &self.rejected_candidate_fingerprint,
            &self.original_input_fingerprint,
            &self.proposed_input_fingerprint,
            &self.formation_request_fingerprint,
        ] {
            check_fp(value)?;
        }
        if !self.rejected_candidate.is_object()
            || !self.prospective_candidate.is_object()
            || !self.original_capture.is_object()
            || !self.proposed_capture.is_object()
            || DefinitionFingerprint::from_bytes(&encode(&self.rejected_candidate)?).as_str()
                != self.rejected_candidate_fingerprint
            || DefinitionFingerprint::from_bytes(&encode(&self.original_capture)?).as_str()
                != self.original_input_fingerprint
            || DefinitionFingerprint::from_bytes(&encode(&self.proposed_capture)?).as_str()
                != self.proposed_input_fingerprint
        {
            return Err(refuse("correction_candidate_content", "bounded complete original/proposed captures and actual rejected candidate must match their canonical fingerprints"));
        }
        encode(self)
    }
}
impl LocalInputCorrectionCandidateSourceV1 {
    pub fn candidate(&self) -> Result<LocalInputCorrectionCandidateV1, LocalAuthorityRefusalV1> {
        check_fp(&self.record_fingerprint)?;
        let candidate: LocalInputCorrectionCandidateV1 = decode(&self.canonical_record_bytes)?;
        if candidate.canonical_json_bytes()? != self.canonical_record_bytes
            || DefinitionFingerprint::from_bytes(&self.canonical_record_bytes).as_str()
                != self.record_fingerprint
            || self.relative_path
                != format!("{CANDIDATE_ROOT}/{}.json", &self.record_fingerprint[7..])
        {
            return Err(refuse(
                "correction_candidate_source",
                "exact bounded native Contracts candidate path and canonical bytes required",
            ));
        }
        Ok(candidate)
    }
}
fn pin_candidate_source<'a>(
    root: &'a RetainedRepositoryRootV1,
    source: &LocalInputCorrectionCandidateSourceV1,
    reach: &LocalEvaluationPathReachV1,
) -> Result<RepositoryExactFilesCheckpointV1<'a>, LocalAuthorityRefusalV1> {
    let candidate = source.candidate()?;
    require_read(&reach.input_paths, &source.relative_path)?;
    require_read(&reach.input_paths, &candidate.original_state_path)?;
    let files = root
        .retain_exact_files_checkpoint_v1(&[&source.relative_path, &candidate.original_state_path])
        .map_err(fs_error)?;
    files
        .require_exact_file(root, &source.relative_path, &source.canonical_record_bytes)
        .map_err(fs_error)?;
    let original_bytes = found(root, &candidate.original_state_path)?;
    files
        .require_exact_file(root, &candidate.original_state_path, &original_bytes)
        .map_err(fs_error)?;
    validate_original_candidate_state(&candidate, &original_bytes)?;
    files.validate().map_err(fs_error)?;
    Ok(files)
}

fn validate_original_candidate_state(
    candidate: &LocalInputCorrectionCandidateV1,
    bytes: &[u8],
) -> Result<(), LocalAuthorityRefusalV1> {
    let state: serde_json::Value = decode(bytes)?;
    let mut content = state.clone();
    content
        .as_object_mut()
        .ok_or_else(|| {
            refuse(
                "correction_original_source",
                "original native state object required",
            )
        })?
        .remove("identity");
    let identity =
        DefinitionFingerprint::from_bytes(&encode(&(ORIGINAL_STATE_DOMAIN, &content))?).to_string();
    let expected_ref = format!("{ORIGINAL_STATE_DOMAIN}#{}", &identity[7..]);
    let original = &state["original_input"];
    let rejected = &candidate.rejected_candidate;
    let proposed = &candidate.prospective_candidate;
    let mut rejected_content = rejected.clone();
    let mut proposed_content = proposed.clone();
    for value in [&mut rejected_content, &mut proposed_content] {
        let object = value.as_object_mut().ok_or_else(|| {
            refuse(
                "correction_original_source",
                "complete rejected and prospective candidates required",
            )
        })?;
        object.remove("subject");
        object.remove("case");
    }
    let capture_request = state["origin_intent"]["request_jcs"]
        .as_str()
        .ok_or_else(|| {
            refuse(
                "correction_original_source",
                "native original captured request must be retained",
            )
        })?;
    let capture: serde_json::Value = decode(capture_request.as_bytes())?;
    if encode(&state)? != bytes
        || DefinitionFingerprint::from_bytes(bytes).as_str()
            != candidate.original_state_bytes_fingerprint
        || identity != candidate.original_state_identity.fingerprint
        || expected_ref != candidate.original_state_identity.reference
        || state["identity"]["fingerprint"] != identity
        || state["identity"]["ref"] != expected_ref
        || state["stage"] != "result_closed"
        || state["locator"]["owner_identity"] != candidate.owner_identity
        || state["purpose"]["project_identity"] != candidate.project_identity
        || state["result"]["identity"]["fingerprint"] != candidate.original_result.fingerprint
        || state["result"]["identity"]["ref"] != candidate.original_result.reference
        || capture["inputs"].as_array().map(Vec::len) != Some(1)
        || capture["inputs"][0] != candidate.original_capture
        || !original["subject"].is_object()
        || !original["case"].is_object()
        || rejected["subject"] != original["subject"]
        || rejected["case"] != original["case"]
        || rejected_content != proposed_content
        || proposed["subject"]["subject"]["fingerprint"] != candidate.proposed_input_fingerprint
        || proposed["case"]["case"]["fingerprint"] != candidate.proposed_input_fingerprint
        || original["subject"]["subject"]["fingerprint"] != candidate.original_input_fingerprint
        || original["case"]["case"]["fingerprint"] != candidate.original_input_fingerprint
        || rejected["subject"] == proposed["subject"]
        || rejected["case"] == proposed["case"]
        || proposed["request_key"] != candidate.formation_request_key
        || proposed["formation"]["subject_fingerprint"] != candidate.proposed_input_fingerprint
        || proposed["contract"] != original["contract"]
        || proposed["case"]["case_id"] != original["case"]["case_id"]
        || proposed["requested_resolution"] != original["requested_resolution"]
    {
        return Err(refuse("correction_original_source", "exact native closed original, retained capture, actual old-subject/new-input inconsistency and consistent proposed correction required"));
    }
    Ok(())
}
impl LocalInputCorrectionDelegationV1 {
    pub fn fingerprint_approved_content(&self) -> Result<String, LocalAuthorityRefusalV1> {
        let mut value = serde_json::to_value(self).map_err(codec)?;
        let object = value
            .as_object_mut()
            .ok_or_else(|| refuse("correction_delegation", "exact delegation object required"))?;
        object.remove("human_approval_asserted");
        object.remove("approved_content_fingerprint");
        fingerprint(
            "handbook.local-input-correction-approved-content@1.0.0",
            &value,
        )
    }
    fn validate_under(
        &self,
        authority: &LocalEvaluationAuthorityLocatorV1,
    ) -> Result<(), LocalAuthorityRefusalV1> {
        authority.validate()?;
        let prior = &authority.predecessor.terms;
        let initial = &authority.predecessor.predecessor.terms;
        if !token(&self.request_key)
            || !self.human_approval_asserted
            || self.approved_content_fingerprint != self.fingerprint_approved_content()?
            || self.principal != authority.terms.principal
            || self.principal != initial.principal
            || self.decision_agent == self.principal
            || self.executor != authority.terms.agent
            || self.expected_initial_fingerprint != prior.expected_predecessor_fingerprint
            || self.expected_local_check_fingerprint
                != authority.terms.expected_predecessor_fingerprint
            || self.expected_evaluation_fingerprint
                != expected_authority(authority)?.outcome_fingerprint
            || self.expected_charter_fingerprint != authority.terms.expected_charter_fingerprint
            || self.decision_class != LocalDecisionClassV1::InputCorrection
            || self.assurance != "local_advisory"
            || self.constraint_inputs != authority.terms.constraint_inputs
            || self.retention_until_utc > authority.terms.retention_until_utc
        {
            return Err(refuse("correction_delegation", "initial human approval must bind the exact compatible executor, predecessors, Charter, class, assurance and constraints"));
        }
        for terms in [
            (
                initial.reserved_decisions.as_slice(),
                initial.delegated_decisions.as_slice(),
            ),
            (
                prior.reserved_decisions.as_slice(),
                prior.delegated_decisions.as_slice(),
            ),
        ] {
            if terms.0.contains(&LocalDecisionClassV1::InputCorrection)
                || !terms.1.contains(&LocalDecisionClassV1::InputCorrection)
            {
                return Err(refuse("correction_not_delegated", "InputCorrection must be explicitly delegated and unreserved through the exact prior terms; human resolution required"));
            }
        }
        for text in [
            &self.principal,
            &self.decision_agent,
            &self.executor,
            &self.approval_source_ref,
        ] {
            bounded_text(text)?;
        }
        for value in [
            &self.expected_initial_fingerprint,
            &self.expected_local_check_fingerprint,
            &self.expected_evaluation_fingerprint,
            &self.expected_charter_fingerprint,
        ] {
            check_fp(value)?;
        }
        utc(&self.retention_until_utc)?;
        require_reach_within(
            &self.requested_reach,
            &LocalEvaluationReachV1::selected(&authority.terms.ceiling)?,
        )?;
        validate_declared_reach(&authority.terms, &self.reach, true)?;
        encode(self)?;
        Ok(())
    }
}

pub(super) fn require_input_correction_class(
    authority: &LocalEvaluationAuthorityLocatorV1,
) -> Result<(), LocalAuthorityRefusalV1> {
    let initial = &authority.predecessor.predecessor.terms;
    let prior = &authority.predecessor.terms;
    for (reserved, delegated) in [
        (&initial.reserved_decisions, &initial.delegated_decisions),
        (&prior.reserved_decisions, &prior.delegated_decisions),
    ] {
        if reserved.contains(&LocalDecisionClassV1::InputCorrection)
            || !delegated.contains(&LocalDecisionClassV1::InputCorrection)
        {
            return Err(refuse("correction_not_delegated", "InputCorrection is reserved or absent from the exact prior delegation; human resolution required"));
        }
    }
    Ok(())
}

/// A consuming permission held through Contracts' exact comparison and atomic
/// formation. Neither a serialized decision nor this type's projections grant it.
pub struct LocalInputCorrectionAdmissionV1<'a> {
    terms: LocalEvaluationTermsAdmissionV1<'a>,
    delegation_locator: &'a LocalInputCorrectionDelegationLocatorV1,
    delegation: &'a LocalInputCorrectionDelegationOutcomeV1,
    request: &'a LocalInputCorrectionRequestRecordV1,
    decision: &'a LocalInputCorrectionDecisionRecordV1,
    candidate: &'a LocalInputCorrectionCandidateV1,
    files: &'a RepositoryExactFilesCheckpointV1<'a>,
    dependencies: &'a [LocalInputCorrectionDependencySourceV1],
}
impl LocalInputCorrectionAdmissionV1<'_> {
    pub fn terms_admission(&self) -> &LocalEvaluationTermsAdmissionV1<'_> {
        &self.terms
    }
    pub fn delegation(&self) -> &LocalInputCorrectionDelegationOutcomeV1 {
        self.delegation
    }
    pub fn request(&self) -> &LocalInputCorrectionRequestRecordV1 {
        self.request
    }
    pub fn decision(&self) -> &LocalInputCorrectionDecisionRecordV1 {
        self.decision
    }
    pub fn candidate(&self) -> &LocalInputCorrectionCandidateV1 {
        self.candidate
    }
    pub fn candidate_source(&self) -> &LocalInputCorrectionCandidateSourceV1 {
        &self.request.request.candidate_source
    }
    pub fn dependency_sources(&self) -> &[LocalInputCorrectionDependencySourceV1] {
        self.dependencies
    }
    pub fn validate_current(&self) -> Result<(), LocalAuthorityRefusalV1> {
        self.terms.require_input_correction()?;
        self.files.validate().map_err(fs_error)?;
        let root = self.terms.guard.root();
        current_delegation(
            root,
            self.terms.locator,
            self.delegation_locator,
            self.terms.basis,
        )?;
        if self.terms.basis != &self.request.request.current_basis
            || self.terms.request != &self.request.request.formation
        {
            return Err(refuse("correction_currentness", "consumption cannot replace the exact pending request applicability or prospective formation"));
        }
        let (head, terminal) = request_head(root, self.request)?;
        if head != self.decision.decision_fingerprint
            || terminal.as_ref() != Some(self.decision)
            || self.decision.submission.outcome != LocalInputCorrectionDecisionV1::Approve
        {
            return Err(refuse(
                "correction_not_approved",
                "only the exact current terminal approval permits one fixed dependent formation",
            ));
        }
        for source in self.dependencies {
            self.files
                .require_exact_file(root, &source.relative_path, &source.canonical_record_bytes)
                .map_err(fs_error)?;
        }
        self.files.validate().map_err(fs_error)
    }
}

fn native_dependencies(
    root: &RetainedRepositoryRootV1,
    authority: &LocalEvaluationAuthorityLocatorV1,
    delegation: &LocalInputCorrectionDelegationLocatorV1,
    request: Option<&LocalInputCorrectionRequestRecordV1>,
    decision: Option<&LocalInputCorrectionDecisionRecordV1>,
) -> Result<Vec<LocalInputCorrectionDependencySourceV1>, LocalAuthorityRefusalV1> {
    let mut paths = authority_paths(authority);
    paths.push(selected_path());
    paths.extend(
        ["request.json", "outcome.json", "terminal.json"].map(|f| delegation_path(delegation, f)),
    );
    if let Some(request) = request {
        paths.extend(["request.json", "terminal.json"].map(|f| request_path(&request.locator, f)));
        paths.push(request.request.candidate_source.relative_path.clone());
        paths.push(
            request
                .request
                .candidate_source
                .candidate()?
                .original_state_path,
        );
        // Every consumed clarification remains in the dependency closure.
        let mut head = request.head_fingerprint.clone();
        for _ in 0..64 {
            let path = head_path(&request.locator, &head)?;
            let RepositoryReadV1::Found(bytes) = optional(root, &path)? else {
                break;
            };
            let locator: LocalInputCorrectionDecisionLocatorV1 = decode(&bytes)?;
            let record = read_decision(root, request, &locator)?;
            paths.push(path);
            paths.extend(
                ["request.json", "outcome.json", "terminal.json"]
                    .map(|f| decision_path(&locator, f)),
            );
            head = record.decision_fingerprint;
            if record.submission.outcome != LocalInputCorrectionDecisionV1::Clarify {
                break;
            }
        }
    }
    if let Some(decision) = decision {
        paths.extend(
            ["request.json", "outcome.json", "terminal.json"]
                .map(|f| decision_path(&decision.locator, f)),
        );
    }
    paths.sort();
    paths.dedup();
    paths
        .into_iter()
        .map(|path| {
            require_read(&delegation.delegation.reach.input_paths, &path)?;
            Ok(LocalInputCorrectionDependencySourceV1 {
                canonical_record_bytes: found(root, &path)?,
                relative_path: path,
            })
        })
        .collect()
}
fn pin_dependencies<'a>(
    root: &'a RetainedRepositoryRootV1,
    dependencies: &[LocalInputCorrectionDependencySourceV1],
) -> Result<RepositoryExactFilesCheckpointV1<'a>, LocalAuthorityRefusalV1> {
    let mut refs = dependencies
        .iter()
        .map(|s| s.relative_path.as_str())
        .collect::<Vec<_>>();
    refs.sort();
    refs.dedup();
    let files = root
        .retain_exact_files_checkpoint_v1(&refs)
        .map_err(fs_error)?;
    for source in dependencies {
        files
            .require_exact_file(root, &source.relative_path, &source.canonical_record_bytes)
            .map_err(fs_error)?;
    }
    files.validate().map_err(fs_error)?;
    Ok(files)
}

impl NativeLocalInputCorrectionV1 {
    #[allow(clippy::too_many_arguments)]
    pub fn with_admission<T>(
        repo: impl AsRef<Path>,
        authority: &LocalEvaluationAuthorityLocatorV1,
        delegation: &LocalInputCorrectionDelegationLocatorV1,
        request_locator: &LocalInputCorrectionRequestLocatorV1,
        decision_locator: &LocalInputCorrectionDecisionLocatorV1,
        basis: &LocalCheckCurrentBasisV1,
        formation: &LocalEvaluationFormationRequestV1,
        executor: &str,
        consume: impl FnOnce(&LocalInputCorrectionAdmissionV1<'_>) -> Result<T, LocalAuthorityRefusalV1>,
    ) -> Result<T, LocalAuthorityRefusalV1> {
        delegation.validate_under(authority)?;
        validate_formation(formation, &authority.terms)?;
        // The source locator is learned from the retained Engine request. Cover
        // its complete bounded native source namespace before reading that proof.
        require_subtree(&formation.reach.input_paths, CANDIDATE_ROOT)?;
        require_subtree(&formation.reach.input_paths, ORIGINAL_STATE_ROOT)?;
        require_subtree(&formation.reach.input_paths, LOCAL_INPUT_CORRECTION_ROOT_V1)?;
        require_input_correction_class(authority)?;
        if executor != authority.terms.agent || executor != delegation.delegation.executor {
            return Err(refuse("correction_executor", "dependent execution must match the unchanged approved executor; a locator is no execution grant"));
        }
        let guard =
            OperatingGuard::acquire(repo.as_ref(), &authority.predecessor.predecessor, false)?;
        let root = guard.root();
        let adopted = current_delegation(root, authority, delegation, basis)?;
        let request = read_request(root, authority, delegation, request_locator)?;
        request.request.validate_under(authority, delegation)?;
        let decision = read_decision(root, &request, decision_locator)?;
        if decision.submission.deciding_actor != delegation.delegation.decision_agent
            || decision.submission.candidate_choice_fingerprint
                != request.request.candidate_source.record_fingerprint
            || decision.submission.outcome != LocalInputCorrectionDecisionV1::Approve
            || formation != &request.request.formation
            || basis != &request.request.current_basis
        {
            return Err(refuse("correction_admission", "exact approved choice, original fresh basis, prospective formation and deciding actor required"));
        }
        let candidate = request.request.candidate_source.candidate()?;
        let dependencies =
            native_dependencies(root, authority, delegation, Some(&request), Some(&decision))?;
        for source in &dependencies {
            require_read(&formation.reach.input_paths, &source.relative_path)?;
        }
        let files = pin_dependencies(root, &dependencies)?;
        let retained_dependencies = dependencies
            .iter()
            .filter(|source| {
                source.relative_path != format!("{LOCAL_EVALUATION_ROOT_V1}/current.json")
            })
            .cloned()
            .collect::<Vec<_>>();
        // The predecessor guard excludes the evaluation subtree, hence the
        // explicit pins above plus exact semantic rereads close the read/pin gap.
        if read_delegation(root, authority, delegation)? != adopted
            || read_request(root, authority, delegation, request_locator)? != request
            || read_decision(root, &request, decision_locator)? != decision
        {
            return Err(refuse(
                "correction_source_substituted",
                "native correction sources changed before admission",
            ));
        }
        let tree = root
            .retain_local_check_authority_tree_guard()
            .map_err(fs_error)?;
        let terms = LocalEvaluationTermsAdmissionV1 {
            locator: authority,
            basis,
            request: formation,
            guard: &guard,
            tree: &tree,
            authority_files: &files,
            authority_fingerprint: adopted.locator.authority_fingerprint.clone(),
        };
        let admission = LocalInputCorrectionAdmissionV1 {
            terms,
            delegation_locator: delegation,
            delegation: &adopted,
            request: &request,
            decision: &decision,
            candidate: &candidate,
            files: &files,
            dependencies: &retained_dependencies,
        };
        admission.validate_current()?;
        let value = consume(&admission)?;
        admission.validate_current()?;
        Ok(value)
    }

    #[cfg(feature = "test-support")]
    pub fn retain_request_interrupted(
        repo: impl AsRef<Path>,
        authority: &LocalEvaluationAuthorityLocatorV1,
        delegation: &LocalInputCorrectionDelegationLocatorV1,
        basis: &LocalCheckCurrentBasisV1,
        request: &LocalInputCorrectionRequestV1,
        interrupt: impl FnOnce(),
    ) -> LocalInputCorrectionExecutionV1<LocalInputCorrectionRequestRecordV1> {
        retain_request(
            repo.as_ref(),
            authority,
            delegation,
            basis,
            request,
            interrupt,
        )
    }
    #[cfg(feature = "test-support")]
    pub fn submit_decision_interrupted(
        repo: impl AsRef<Path>,
        authority: &LocalEvaluationAuthorityLocatorV1,
        delegation: &LocalInputCorrectionDelegationLocatorV1,
        request: &LocalInputCorrectionRequestLocatorV1,
        basis: &LocalCheckCurrentBasisV1,
        submission: &LocalInputCorrectionSubmissionV1,
        interrupt: impl FnOnce(),
    ) -> LocalInputCorrectionExecutionV1<LocalInputCorrectionDecisionRecordV1> {
        submit_decision(
            repo.as_ref(),
            authority,
            delegation,
            request,
            basis,
            submission,
            interrupt,
        )
    }
}

macro_rules! correction_codec {
    ($($record:ty),+ $(,)?) => { $(impl $record {
        /// Bounded duplicate-safe closed decoding. No portable record is admission.
        pub fn from_json_bytes(bytes: &[u8]) -> Result<Self, LocalAuthorityRefusalV1> { decode(bytes) }
        pub fn canonical_json_bytes(&self) -> Result<Vec<u8>, LocalAuthorityRefusalV1> { encode(self) }
    })+ };
}
correction_codec!(
    LocalInputCorrectionDelegationV1,
    LocalInputCorrectionRequestV1,
    LocalInputCorrectionRequestLocatorV1,
    LocalInputCorrectionRequestRecordV1,
    LocalInputCorrectionSubmissionV1,
    LocalInputCorrectionDecisionLocatorV1,
    LocalInputCorrectionDecisionRecordV1,
    LocalInputCorrectionCandidateSourceV1,
    LocalInputCorrectionDelegationOutcomeV1
);
impl LocalInputCorrectionDelegationLocatorV1 {
    fn fingerprint_request(&self) -> Result<String, LocalAuthorityRefusalV1> {
        fingerprint(
            "handbook.local-input-correction-delegation-request@1.0.0",
            &(
                &self.delegation,
                &self.authority_fingerprint,
                &self.adoption_basis,
                &self.prepared_at_utc,
            ),
        )
    }
    fn validate_under(
        &self,
        authority: &LocalEvaluationAuthorityLocatorV1,
    ) -> Result<(), LocalAuthorityRefusalV1> {
        self.delegation.validate_under(authority)?;
        if self.authority_fingerprint != expected_authority(authority)?.outcome_fingerprint
            || self.request_fingerprint != self.fingerprint_request()?
        {
            return Err(refuse(
                "correction_delegation_locator",
                "exact original delegation locator required",
            ));
        }
        validate_basis(&self.adoption_basis)?;
        horizon(&self.delegation.retention_until_utc, &self.prepared_at_utc)
    }
    pub fn canonical_json_bytes(&self) -> Result<Vec<u8>, LocalAuthorityRefusalV1> {
        encode(self)
    }
    pub fn from_json_bytes(bytes: &[u8]) -> Result<Self, LocalAuthorityRefusalV1> {
        decode(bytes)
    }
}
impl LocalInputCorrectionRequestV1 {
    fn validate_under(
        &self,
        authority: &LocalEvaluationAuthorityLocatorV1,
        delegation: &LocalInputCorrectionDelegationLocatorV1,
    ) -> Result<(), LocalAuthorityRefusalV1> {
        delegation.validate_under(authority)?;
        if !token(&self.request_key) {
            return Err(refuse(
                "correction_request",
                "bounded original request key required",
            ));
        }
        bounded_text(&self.submitter)?;
        bounded_text(&self.context_source_ref)?;
        if self.additional_context.len() > 4096 || self.additional_context.contains('\0') {
            return Err(refuse(
                "correction_context",
                "bounded attributed context required",
            ));
        }
        validate_formation(&self.formation, &authority.terms)?;
        validate_basis(&self.current_basis)?;
        let candidate = self.candidate_source.candidate()?;
        if candidate.project_identity != project_identity(authority)
            || candidate.owner_identity
                != authority
                    .predecessor
                    .predecessor
                    .charter
                    .binding
                    .initialization_claim
                    .physical_root_fingerprint()
            || candidate.proposed_input_fingerprint != self.formation.subject_fingerprint
            || candidate.prospective_candidate["formation"]
                != serde_json::to_value(&self.formation).map_err(codec)?
            || self.formation.retention_until_utc > delegation.delegation.retention_until_utc
        {
            return Err(refuse(
                "correction_request_congruence",
                "exact proposed native project/capture and bounded retention required",
            ));
        }
        require_reach_within(
            &self.formation.requested_reach,
            &delegation.delegation.requested_reach,
        )?;
        for root in &self.formation.reach.owner_write_paths {
            require_declared_write(&delegation.delegation.reach, root)?;
        }
        for selector in &self.formation.reach.input_paths {
            require_input_selector_coverage(
                &delegation.delegation.reach.input_paths,
                &authority.terms.allowed_input_paths,
                selector,
            )?;
        }
        require_read(
            &self.formation.reach.input_paths,
            &self.candidate_source.relative_path,
        )?;
        encode(self)?;
        Ok(())
    }
    pub fn locator(
        &self,
        delegation_fingerprint: &str,
    ) -> Result<LocalInputCorrectionRequestLocatorV1, LocalAuthorityRefusalV1> {
        if !token(&self.request_key) {
            return Err(refuse(
                "correction_request",
                "bounded original request key required",
            ));
        }
        check_fp(delegation_fingerprint)?;
        Ok(LocalInputCorrectionRequestLocatorV1 {
            request_key: self.request_key.clone(),
            request_fingerprint: fingerprint(
                "handbook.local-input-correction-request@1.0.0",
                &(self, delegation_fingerprint),
            )?,
            delegation_fingerprint: delegation_fingerprint.into(),
        })
    }
}
impl LocalInputCorrectionRequestLocatorV1 {
    fn validate(&self) -> Result<(), LocalAuthorityRefusalV1> {
        if !token(&self.request_key) {
            return Err(refuse(
                "correction_request_locator",
                "original request key required",
            ));
        }
        check_fp(&self.request_fingerprint)?;
        check_fp(&self.delegation_fingerprint)
    }
}
impl LocalInputCorrectionSubmissionV1 {
    pub fn locator(
        &self,
        request: &LocalInputCorrectionRequestLocatorV1,
    ) -> Result<LocalInputCorrectionDecisionLocatorV1, LocalAuthorityRefusalV1> {
        request.validate()?;
        self.validate()?;
        Ok(LocalInputCorrectionDecisionLocatorV1 {
            request_key: request.request_key.clone(),
            request_fingerprint: request.request_fingerprint.clone(),
            submission_key: self.submission_key.clone(),
            submission_fingerprint: fingerprint(
                "handbook.local-input-correction-submission@1.0.0",
                &(request, self),
            )?,
        })
    }
    fn validate(&self) -> Result<(), LocalAuthorityRefusalV1> {
        if !token(&self.submission_key) || self.assurance != "local_advisory" {
            return Err(refuse(
                "correction_submission",
                "one supported keyed local-advisory decision required",
            ));
        }
        check_fp(&self.expected_head)?;
        check_fp(&self.candidate_choice_fingerprint)?;
        for text in [
            &self.deciding_actor,
            &self.submitter,
            &self.decision_source_ref,
            &self.reason,
        ] {
            bounded_text(text)?;
        }
        Ok(())
    }
}
impl LocalInputCorrectionDecisionLocatorV1 {
    fn validate(&self) -> Result<(), LocalAuthorityRefusalV1> {
        if !token(&self.request_key) || !token(&self.submission_key) {
            return Err(refuse(
                "correction_decision_locator",
                "exact original decision keys required",
            ));
        }
        check_fp(&self.request_fingerprint)?;
        check_fp(&self.submission_fingerprint)
    }
}
fn validate_basis(basis: &LocalCheckCurrentBasisV1) -> Result<(), LocalAuthorityRefusalV1> {
    check_fp(&basis.relation_fingerprint)?;
    check_fp(&basis.governance_tree_fingerprint)
}
fn project_identity(authority: &LocalEvaluationAuthorityLocatorV1) -> &str {
    authority
        .predecessor
        .predecessor
        .charter
        .binding
        .initialization_claim
        .proposed_repository_identity()
}
fn selected_path() -> String {
    format!("{LOCAL_INPUT_CORRECTION_ROOT_V1}/selected.json")
}
fn delegation_path(locator: &LocalInputCorrectionDelegationLocatorV1, file: &str) -> String {
    format!(
        "{LOCAL_INPUT_CORRECTION_ROOT_V1}/delegation/{}/{file}",
        locator.delegation.request_key
    )
}
fn request_path(locator: &LocalInputCorrectionRequestLocatorV1, file: &str) -> String {
    format!(
        "{LOCAL_INPUT_CORRECTION_ROOT_V1}/requests/{}/{file}",
        locator.request_key
    )
}
fn decision_path(locator: &LocalInputCorrectionDecisionLocatorV1, file: &str) -> String {
    format!(
        "{LOCAL_INPUT_CORRECTION_ROOT_V1}/requests/{}/decisions/{}/{file}",
        locator.request_key, locator.submission_key
    )
}
fn head_path(
    request: &LocalInputCorrectionRequestLocatorV1,
    head: &str,
) -> Result<String, LocalAuthorityRefusalV1> {
    request.validate()?;
    check_fp(head)?;
    Ok(format!(
        "{LOCAL_INPUT_CORRECTION_ROOT_V1}/requests/{}/heads/{}.json",
        request.request_key,
        &head[7..]
    ))
}
fn delegation_outcome(
    locator: &LocalInputCorrectionDelegationLocatorV1,
) -> Result<LocalInputCorrectionDelegationOutcomeV1, LocalAuthorityRefusalV1> {
    Ok(LocalInputCorrectionDelegationOutcomeV1 {
        schema_ref: LOCAL_INPUT_CORRECTION_RULE_V1.into(),
        locator: locator.clone(),
        delegation_fingerprint: fingerprint(
            "handbook.local-input-correction-delegation-outcome@1.0.0",
            locator,
        )?,
    })
}
fn read_delegation(
    root: &RetainedRepositoryRootV1,
    authority: &LocalEvaluationAuthorityLocatorV1,
    locator: &LocalInputCorrectionDelegationLocatorV1,
) -> Result<LocalInputCorrectionDelegationOutcomeV1, LocalAuthorityRefusalV1> {
    locator.validate_under(authority)?;
    history_authority(root, authority)?;
    let expected = delegation_outcome(locator)?;
    for (path, bytes) in [
        (delegation_path(locator, "request.json"), encode(locator)?),
        (delegation_path(locator, "outcome.json"), encode(&expected)?),
        (
            delegation_path(locator, "terminal.json"),
            encode(&expected)?,
        ),
        (selected_path(), encode(&expected)?),
    ] {
        require_read(&locator.delegation.reach.input_paths, &path)?;
        if found(root, &path)? != bytes {
            return Err(refuse(
                "correction_delegation_conflict",
                "exact original immutable delegation selection/history required",
            ));
        }
    }
    Ok(expected)
}
fn current_delegation(
    root: &RetainedRepositoryRootV1,
    authority: &LocalEvaluationAuthorityLocatorV1,
    locator: &LocalInputCorrectionDelegationLocatorV1,
    basis: &LocalCheckCurrentBasisV1,
) -> Result<LocalInputCorrectionDelegationOutcomeV1, LocalAuthorityRefusalV1> {
    current_authority(root, authority, basis)?;
    horizon(&locator.delegation.retention_until_utc, &now()?)?;
    read_delegation(root, authority, locator)
}
fn optional(
    root: &RetainedRepositoryRootV1,
    path: &str,
) -> Result<RepositoryReadV1, LocalAuthorityRefusalV1> {
    root.read_local_owner_optional_bounded(path, MAX)
        .map_err(fs_error)
}
fn require_recovery_intent(
    root: &RetainedRepositoryRootV1,
    intent: &str,
    terminal: &str,
    kind: &str,
) -> Result<(), LocalAuthorityRefusalV1> {
    if optional(root, intent)? == RepositoryReadV1::Absent {
        return Err(refuse(
            &format!("{kind}_absent"),
            "original durable intent is absent; recovery creates no operation",
        ));
    }
    if optional(root, terminal)? == RepositoryReadV1::Absent {
        return Err(refuse(
            &format!("{kind}_pending"),
            "original durable intent remains pending; resume only the exact original request",
        ));
    }
    Ok(())
}
fn adopt_delegation(
    repo: &Path,
    authority: &LocalEvaluationAuthorityLocatorV1,
    locator: &LocalInputCorrectionDelegationLocatorV1,
    allow_create: bool,
    interrupt: impl FnOnce(),
) -> LocalInputCorrectionExecutionV1<LocalInputCorrectionDelegationOutcomeV1> {
    let mut effects = false;
    let mut replay = false;
    let result = (|| {
        locator.validate_under(authority)?;
        let guard = OperatingGuard::acquire(repo, &authority.predecessor.predecessor, false)?;
        let root = guard.root();
        if optional(root, &delegation_path(locator, "terminal.json"))? != RepositoryReadV1::Absent {
            replay = true;
            return read_delegation(root, authority, locator);
        }
        let intent = optional(root, &delegation_path(locator, "request.json"))?;
        if !allow_create && intent == RepositoryReadV1::Absent {
            return Err(refuse(
                "correction_delegation_absent",
                "portable recovery locator cannot create an absent delegation intent",
            ));
        }
        if let RepositoryReadV1::Found(bytes) = intent {
            if bytes != encode(locator)? {
                return Err(refuse(
                    "correction_delegation_conflict",
                    "original delegation key retains different content",
                ));
            }
        }
        current_authority(root, authority, &locator.adoption_basis)?;
        horizon(&locator.delegation.retention_until_utc, &now()?)?;
        let tree = root
            .retain_local_check_authority_tree_guard()
            .map_err(fs_error)?;
        let paths = authority_paths(authority);
        let dependencies = paths
            .into_iter()
            .map(|p| {
                Ok(LocalInputCorrectionDependencySourceV1 {
                    canonical_record_bytes: found(root, &p)?,
                    relative_path: p,
                })
            })
            .collect::<Result<Vec<_>, LocalAuthorityRefusalV1>>()?;
        for source in &dependencies {
            require_read(&locator.delegation.reach.input_paths, &source.relative_path)?;
        }
        let files = pin_dependencies(root, &dependencies)?;
        current_authority(root, authority, &locator.adoption_basis)?;
        let outcome = delegation_outcome(locator)?;
        if let RepositoryReadV1::Found(bytes) = optional(root, &selected_path())? {
            if bytes != encode(&outcome)? {
                return Err(refuse(
                    "correction_delegation_selected",
                    "another immutable delegation selected; no replacement authority",
                ));
            }
        }
        let outcome_bytes = encode(&outcome)?;
        let locator_bytes = encode(locator)?;
        effects = true;
        root.create_directories(&format!(
            "{LOCAL_INPUT_CORRECTION_ROOT_V1}/delegation/{}",
            locator.delegation.request_key
        ))
        .map_err(fs_error)?;
        write_exact(
            root,
            &delegation_path(locator, "request.json"),
            &locator_bytes,
        )?;
        interrupt();
        tree.validate().map_err(fs_error)?;
        files.validate().map_err(fs_error)?;
        current_authority(root, authority, &locator.adoption_basis)?;
        write_exact(
            root,
            &delegation_path(locator, "outcome.json"),
            &outcome_bytes,
        )?;
        write_exact(root, &selected_path(), &outcome_bytes)?;
        guard.validate()?;
        write_exact(
            root,
            &delegation_path(locator, "terminal.json"),
            &outcome_bytes,
        )?;
        read_delegation(root, authority, locator)
    })();
    execute(result, effects, replay)
}
fn request_record_identity(
    record: &LocalInputCorrectionRequestRecordV1,
) -> Result<String, LocalAuthorityRefusalV1> {
    fingerprint(
        "handbook.local-input-correction-request-head@1.0.0",
        &(&record.locator, &record.request, &record.retained_at_utc),
    )
}
fn read_request(
    root: &RetainedRepositoryRootV1,
    authority: &LocalEvaluationAuthorityLocatorV1,
    delegation: &LocalInputCorrectionDelegationLocatorV1,
    locator: &LocalInputCorrectionRequestLocatorV1,
) -> Result<LocalInputCorrectionRequestRecordV1, LocalAuthorityRefusalV1> {
    locator.validate()?;
    let adopted = read_delegation(root, authority, delegation)?;
    if locator.delegation_fingerprint != adopted.delegation_fingerprint {
        return Err(refuse(
            "correction_request_conflict",
            "request belongs to another immutable delegation",
        ));
    }
    let bytes = found(root, &request_path(locator, "request.json"))?;
    let record: LocalInputCorrectionRequestRecordV1 = decode(&bytes)?;
    if record.schema_ref != LOCAL_INPUT_CORRECTION_RULE_V1
        || record.locator != *locator
        || record.request.locator(&adopted.delegation_fingerprint)? != *locator
        || record.head_fingerprint != request_record_identity(&record)?
        || encode(&record)? != bytes
        || found(root, &request_path(locator, "terminal.json"))? != bytes
    {
        return Err(refuse(
            "correction_request_conflict",
            "exact original native request history is incomplete or substituted",
        ));
    }
    utc(&record.retained_at_utc)?;
    // Historical replay checks immutable bytes and content, not a fresh horizon.
    record.request.candidate_source.candidate()?;
    require_read(
        &delegation.delegation.reach.input_paths,
        &record.request.candidate_source.relative_path,
    )?;
    let candidate = record.request.candidate_source.candidate()?;
    require_read(
        &delegation.delegation.reach.input_paths,
        &candidate.original_state_path,
    )?;
    validate_original_candidate_state(&candidate, &found(root, &candidate.original_state_path)?)?;
    if found(root, &record.request.candidate_source.relative_path)?
        != record.request.candidate_source.canonical_record_bytes
    {
        return Err(refuse(
            "correction_candidate_source",
            "native Contracts correction proof was substituted",
        ));
    }
    record.request.candidate_source.candidate()?;
    Ok(record)
}
fn decision_identity(
    record: &LocalInputCorrectionDecisionRecordV1,
) -> Result<String, LocalAuthorityRefusalV1> {
    fingerprint(
        "handbook.local-input-correction-decision@1.0.0",
        &(
            &record.locator,
            &record.submission,
            &record.request_fingerprint,
            &record.recorded_at_utc,
        ),
    )
}
fn read_decision(
    root: &RetainedRepositoryRootV1,
    request: &LocalInputCorrectionRequestRecordV1,
    locator: &LocalInputCorrectionDecisionLocatorV1,
) -> Result<LocalInputCorrectionDecisionRecordV1, LocalAuthorityRefusalV1> {
    locator.validate()?;
    let bytes = found(root, &decision_path(locator, "request.json"))?;
    let record: LocalInputCorrectionDecisionRecordV1 = decode(&bytes)?;
    if record.schema_ref != LOCAL_INPUT_CORRECTION_RULE_V1
        || record.locator != *locator
        || record.request_fingerprint != request.locator.request_fingerprint
        || record.submission.locator(&request.locator)? != *locator
        || record.decision_fingerprint != decision_identity(&record)?
        || encode(&record)? != bytes
        || found(root, &decision_path(locator, "outcome.json"))? != bytes
        || found(root, &decision_path(locator, "terminal.json"))? != bytes
        || found(
            root,
            &head_path(&request.locator, &record.submission.expected_head)?,
        )? != encode(locator)?
    {
        return Err(refuse(
            "correction_decision_conflict",
            "exact original decision content, retained request and serialized head required",
        ));
    }
    utc(&record.recorded_at_utc)?;
    Ok(record)
}
fn request_head(
    root: &RetainedRepositoryRootV1,
    request: &LocalInputCorrectionRequestRecordV1,
) -> Result<(String, Option<LocalInputCorrectionDecisionRecordV1>), LocalAuthorityRefusalV1> {
    let mut head = request.head_fingerprint.clone();
    let mut last = None;
    for _ in 0..64 {
        match optional(root, &head_path(&request.locator, &head)?)? {
            RepositoryReadV1::Absent => return Ok((head, last)),
            RepositoryReadV1::Found(bytes) => {
                let locator: LocalInputCorrectionDecisionLocatorV1 = decode(&bytes)?;
                let decision = read_decision(root, request, &locator)?;
                if decision.submission.expected_head != head {
                    return Err(refuse(
                        "correction_head",
                        "native request decision chain is substituted",
                    ));
                }
                head = decision.decision_fingerprint.clone();
                if decision.submission.outcome != LocalInputCorrectionDecisionV1::Clarify {
                    return Ok((head, Some(decision)));
                }
                last = Some(decision);
            }
        }
    }
    Err(refuse(
        "correction_decision_bound",
        "bounded clarification decision population exhausted",
    ))
}

fn predecessor_decision_dependencies(
    root: &RetainedRepositoryRootV1,
    request: &LocalInputCorrectionRequestRecordV1,
    expected_head: &str,
    reach: &LocalEvaluationPathReachV1,
) -> Result<Vec<LocalInputCorrectionDependencySourceV1>, LocalAuthorityRefusalV1> {
    let mut head = request.head_fingerprint.clone();
    let mut dependencies = Vec::new();
    for _ in 0..64 {
        if head == expected_head {
            return Ok(dependencies);
        }
        let path = head_path(&request.locator, &head)?;
        require_read(&reach.input_paths, &path)?;
        let bytes = match optional(root, &path)? {
            RepositoryReadV1::Found(bytes) => bytes,
            RepositoryReadV1::Absent => {
                return Err(refuse(
                    "correction_stale_head",
                    "expected head is not a retained unresolved clarification descendant",
                ))
            }
        };
        let locator: LocalInputCorrectionDecisionLocatorV1 = decode(&bytes)?;
        for file in ["request.json", "outcome.json", "terminal.json"] {
            require_read(&reach.input_paths, &decision_path(&locator, file))?;
        }
        let decision = read_decision(root, request, &locator)?;
        if decision.submission.expected_head != head
            || decision.submission.outcome != LocalInputCorrectionDecisionV1::Clarify
        {
            return Err(refuse(
                "correction_stale_head",
                "expected head must preserve the exact unresolved clarification chain",
            ));
        }
        dependencies.push(LocalInputCorrectionDependencySourceV1 {
            relative_path: path,
            canonical_record_bytes: bytes,
        });
        for file in ["request.json", "outcome.json", "terminal.json"] {
            dependencies.push(LocalInputCorrectionDependencySourceV1 {
                relative_path: decision_path(&locator, file),
                canonical_record_bytes: encode(&decision)?,
            });
        }
        head = decision.decision_fingerprint;
    }
    Err(refuse(
        "correction_decision_bound",
        "bounded clarification decision population exhausted",
    ))
}
fn execute<T>(
    result: Result<T, LocalAuthorityRefusalV1>,
    effects: bool,
    replay: bool,
) -> LocalInputCorrectionExecutionV1<T> {
    match result {
        Ok(outcome) => LocalInputCorrectionExecutionV1 {
            disposition: if replay {
                LocalAuthorityDispositionV1::Replayed
            } else {
                LocalAuthorityDispositionV1::Committed
            },
            outcome: Some(outcome),
            refusal: None,
        },
        Err(refusal) => LocalInputCorrectionExecutionV1 {
            disposition: if effects || refusal.code.ends_with("_pending") {
                LocalAuthorityDispositionV1::Uncertain
            } else {
                LocalAuthorityDispositionV1::Refused
            },
            outcome: None,
            refusal: Some(refusal),
        },
    }
}

pub struct NativeLocalInputCorrectionV1;
impl NativeLocalInputCorrectionV1 {
    #[cfg(feature = "test-support")]
    pub fn adopt_delegation_interrupted(
        repo: impl AsRef<Path>,
        authority: &LocalEvaluationAuthorityLocatorV1,
        prepared: PreparedLocalInputCorrectionDelegationV1,
        interrupt: impl FnOnce(),
    ) -> LocalInputCorrectionExecutionV1<LocalInputCorrectionDelegationOutcomeV1> {
        adopt_delegation(repo.as_ref(), authority, &prepared.locator, true, interrupt)
    }
    pub fn prepare_delegation(
        repo: impl AsRef<Path>,
        authority: &LocalEvaluationAuthorityLocatorV1,
        basis: &LocalCheckCurrentBasisV1,
        delegation: LocalInputCorrectionDelegationV1,
    ) -> Result<PreparedLocalInputCorrectionDelegationV1, LocalAuthorityRefusalV1> {
        delegation.validate_under(authority)?;
        let guard =
            OperatingGuard::acquire(repo.as_ref(), &authority.predecessor.predecessor, false)?;
        let current = current_authority(guard.root(), authority, basis)?;
        horizon(&delegation.retention_until_utc, &now()?)?;
        if optional(guard.root(), &selected_path())? != RepositoryReadV1::Absent {
            return Err(refuse("correction_delegation_selected", "one immutable delegation selected; recover its original locator, replacement unsupported"));
        }
        let mut locator = LocalInputCorrectionDelegationLocatorV1 {
            delegation,
            authority_fingerprint: current.outcome_fingerprint,
            adoption_basis: basis.clone(),
            prepared_at_utc: now()?,
            request_fingerprint: String::new(),
        };
        locator.request_fingerprint = locator.fingerprint_request()?;
        locator.validate_under(authority)?;
        guard.validate()?;
        Ok(PreparedLocalInputCorrectionDelegationV1 { locator })
    }
    pub fn adopt_delegation(
        repo: impl AsRef<Path>,
        authority: &LocalEvaluationAuthorityLocatorV1,
        prepared: PreparedLocalInputCorrectionDelegationV1,
    ) -> LocalInputCorrectionExecutionV1<LocalInputCorrectionDelegationOutcomeV1> {
        adopt_delegation(repo.as_ref(), authority, &prepared.locator, true, || {})
    }
    pub fn resume_delegation(
        repo: impl AsRef<Path>,
        authority: &LocalEvaluationAuthorityLocatorV1,
        locator: &LocalInputCorrectionDelegationLocatorV1,
    ) -> LocalInputCorrectionExecutionV1<LocalInputCorrectionDelegationOutcomeV1> {
        adopt_delegation(repo.as_ref(), authority, locator, false, || {})
    }
    pub fn recover_delegation(
        repo: impl AsRef<Path>,
        authority: &LocalEvaluationAuthorityLocatorV1,
        locator: &LocalInputCorrectionDelegationLocatorV1,
    ) -> LocalInputCorrectionExecutionV1<LocalInputCorrectionDelegationOutcomeV1> {
        let result = (|| {
            locator.validate_under(authority)?;
            let guard =
                OperatingGuard::acquire(repo.as_ref(), &authority.predecessor.predecessor, false)?;
            require_recovery_intent(
                guard.root(),
                &delegation_path(locator, "request.json"),
                &delegation_path(locator, "terminal.json"),
                "correction_delegation",
            )?;
            let record = read_delegation(guard.root(), authority, locator)?;
            guard.validate()?;
            Ok(record)
        })();
        execute(result, false, true)
    }
    pub fn retain_request(
        repo: impl AsRef<Path>,
        authority: &LocalEvaluationAuthorityLocatorV1,
        delegation: &LocalInputCorrectionDelegationLocatorV1,
        basis: &LocalCheckCurrentBasisV1,
        request: &LocalInputCorrectionRequestV1,
    ) -> LocalInputCorrectionExecutionV1<LocalInputCorrectionRequestRecordV1> {
        retain_request(repo.as_ref(), authority, delegation, basis, request, || {})
    }
    pub fn recover_request(
        repo: impl AsRef<Path>,
        authority: &LocalEvaluationAuthorityLocatorV1,
        delegation: &LocalInputCorrectionDelegationLocatorV1,
        locator: &LocalInputCorrectionRequestLocatorV1,
    ) -> LocalInputCorrectionExecutionV1<LocalInputCorrectionRequestRecordV1> {
        let result = (|| {
            delegation.validate_under(authority)?;
            locator.validate()?;
            let guard =
                OperatingGuard::acquire(repo.as_ref(), &authority.predecessor.predecessor, false)?;
            require_recovery_intent(
                guard.root(),
                &request_path(locator, "request.json"),
                &request_path(locator, "terminal.json"),
                "correction_request",
            )?;
            let record = read_request(guard.root(), authority, delegation, locator)?;
            guard.validate()?;
            Ok(record)
        })();
        execute(result, false, true)
    }
    pub fn submit_decision(
        repo: impl AsRef<Path>,
        authority: &LocalEvaluationAuthorityLocatorV1,
        delegation: &LocalInputCorrectionDelegationLocatorV1,
        request: &LocalInputCorrectionRequestLocatorV1,
        basis: &LocalCheckCurrentBasisV1,
        submission: &LocalInputCorrectionSubmissionV1,
    ) -> LocalInputCorrectionExecutionV1<LocalInputCorrectionDecisionRecordV1> {
        submit_decision(
            repo.as_ref(),
            authority,
            delegation,
            request,
            basis,
            submission,
            || {},
        )
    }
    pub fn recover_decision(
        repo: impl AsRef<Path>,
        authority: &LocalEvaluationAuthorityLocatorV1,
        delegation: &LocalInputCorrectionDelegationLocatorV1,
        request: &LocalInputCorrectionRequestLocatorV1,
        locator: &LocalInputCorrectionDecisionLocatorV1,
    ) -> LocalInputCorrectionExecutionV1<LocalInputCorrectionDecisionRecordV1> {
        let result = (|| {
            delegation.validate_under(authority)?;
            locator.validate()?;
            let guard =
                OperatingGuard::acquire(repo.as_ref(), &authority.predecessor.predecessor, false)?;
            require_recovery_intent(
                guard.root(),
                &decision_path(locator, "request.json"),
                &decision_path(locator, "terminal.json"),
                "correction_decision",
            )?;
            let request = read_request(guard.root(), authority, delegation, request)?;
            let decision = read_decision(guard.root(), &request, locator)?;
            guard.validate()?;
            Ok(decision)
        })();
        execute(result, false, true)
    }
}

fn retain_request(
    repo: &Path,
    authority: &LocalEvaluationAuthorityLocatorV1,
    delegation: &LocalInputCorrectionDelegationLocatorV1,
    basis: &LocalCheckCurrentBasisV1,
    request: &LocalInputCorrectionRequestV1,
    interrupt: impl FnOnce(),
) -> LocalInputCorrectionExecutionV1<LocalInputCorrectionRequestRecordV1> {
    let mut effects = false;
    let mut replay = false;
    let result = (|| {
        delegation.validate_under(authority)?;
        let adopted = delegation_outcome(delegation)?;
        let locator = request.locator(&adopted.delegation_fingerprint)?;
        let guard = OperatingGuard::acquire(repo, &authority.predecessor.predecessor, false)?;
        let root = guard.root();
        if optional(root, &request_path(&locator, "terminal.json"))? != RepositoryReadV1::Absent {
            replay = true;
            return read_request(root, authority, delegation, &locator);
        }
        request.validate_under(authority, delegation)?;
        current_delegation(root, authority, delegation, basis)?;
        if basis != &request.current_basis {
            return Err(refuse(
                "correction_currentness",
                "fresh basis differs from the original correction request; prepare a new request",
            ));
        }
        let candidate_files =
            pin_candidate_source(root, &request.candidate_source, &request.formation.reach)?;
        let dependencies = native_dependencies(root, authority, delegation, None, None)?;
        let files = pin_dependencies(root, &dependencies)?;
        let tree = root
            .retain_local_check_authority_tree_guard()
            .map_err(fs_error)?;
        current_delegation(root, authority, delegation, basis)?;
        let path = request_path(&locator, "request.json");
        let record = match optional(root, &path)? {
            RepositoryReadV1::Found(bytes) => {
                let record: LocalInputCorrectionRequestRecordV1 = decode(&bytes)?;
                if record.locator != locator
                    || record.request != *request
                    || record.head_fingerprint != request_record_identity(&record)?
                    || encode(&record)? != bytes
                {
                    return Err(refuse(
                        "correction_request_conflict",
                        "request key already retains different original content",
                    ));
                }
                record
            }
            RepositoryReadV1::Absent => {
                let mut record = LocalInputCorrectionRequestRecordV1 {
                    schema_ref: LOCAL_INPUT_CORRECTION_RULE_V1.into(),
                    locator: locator.clone(),
                    request: request.clone(),
                    head_fingerprint: String::new(),
                    retained_at_utc: now()?,
                };
                record.head_fingerprint = request_record_identity(&record)?;
                record
            }
        };
        let record_bytes = encode(&record)?;
        effects = true;
        root.create_directories(&format!(
            "{LOCAL_INPUT_CORRECTION_ROOT_V1}/requests/{}",
            locator.request_key
        ))
        .map_err(fs_error)?;
        write_exact(root, &path, &record_bytes)?;
        interrupt();
        tree.validate().map_err(fs_error)?;
        files.validate().map_err(fs_error)?;
        candidate_files.validate().map_err(fs_error)?;
        current_delegation(root, authority, delegation, basis)?;
        guard.validate()?;
        write_exact(
            root,
            &request_path(&locator, "terminal.json"),
            &record_bytes,
        )?;
        read_request(root, authority, delegation, &locator)
    })();
    execute(result, effects, replay)
}

fn submit_decision(
    repo: &Path,
    authority: &LocalEvaluationAuthorityLocatorV1,
    delegation: &LocalInputCorrectionDelegationLocatorV1,
    request_locator: &LocalInputCorrectionRequestLocatorV1,
    basis: &LocalCheckCurrentBasisV1,
    submission: &LocalInputCorrectionSubmissionV1,
    interrupt: impl FnOnce(),
) -> LocalInputCorrectionExecutionV1<LocalInputCorrectionDecisionRecordV1> {
    let mut effects = false;
    let mut replay = false;
    let result = (|| {
        delegation.validate_under(authority)?;
        let locator = submission.locator(request_locator)?;
        let guard = OperatingGuard::acquire(repo, &authority.predecessor.predecessor, false)?;
        let root = guard.root();
        let request = read_request(root, authority, delegation, request_locator)?;
        if optional(root, &decision_path(&locator, "terminal.json"))? != RepositoryReadV1::Absent {
            replay = true;
            return read_decision(root, &request, &locator);
        }
        current_delegation(root, authority, delegation, basis)?;
        request.request.validate_under(authority, delegation)?;
        if basis != &request.request.current_basis {
            return Err(refuse(
                "correction_currentness",
                "pending request binds another applicability basis; a fresh request is required",
            ));
        }
        if submission.deciding_actor != delegation.delegation.decision_agent
            || submission.candidate_choice_fingerprint
                != request.request.candidate_source.record_fingerprint
        {
            return Err(refuse("correction_decision_authority", "deciding assertion and exact proposed choice must match the original approved delegation and native request"));
        }
        let candidate_files = pin_candidate_source(
            root,
            &request.request.candidate_source,
            &request.request.formation.reach,
        )?;
        let mut dependencies = native_dependencies(root, authority, delegation, None, None)?;
        let predecessor_dependencies = predecessor_decision_dependencies(
            root,
            &request,
            &submission.expected_head,
            &delegation.delegation.reach,
        )?;
        dependencies.extend(predecessor_dependencies.iter().cloned());
        for path in [
            request_path(&request.locator, "request.json"),
            request_path(&request.locator, "terminal.json"),
        ] {
            dependencies.push(LocalInputCorrectionDependencySourceV1 {
                canonical_record_bytes: found(root, &path)?,
                relative_path: path,
            });
        }
        let files = pin_dependencies(root, &dependencies)?;
        if predecessor_decision_dependencies(
            root,
            &request,
            &submission.expected_head,
            &delegation.delegation.reach,
        )? != predecessor_dependencies
        {
            return Err(refuse(
                "correction_source_substituted",
                "clarification predecessor sources changed before decision admission",
            ));
        }
        let tree = root
            .retain_local_check_authority_tree_guard()
            .map_err(fs_error)?;
        if read_request(root, authority, delegation, request_locator)? != request {
            return Err(refuse(
                "correction_source_substituted",
                "native request changed before decision admission",
            ));
        }
        let slot = head_path(&request.locator, &submission.expected_head)?;
        match optional(root, &slot)? {
            RepositoryReadV1::Found(bytes) if bytes == encode(&locator)? => {}
            RepositoryReadV1::Found(_) => {
                return Err(refuse(
                    "correction_stale_head",
                    "another decision already consumes the expected request head",
                ))
            }
            RepositoryReadV1::Absent => {
                let (head, last) = request_head(root, &request)?;
                if head != submission.expected_head
                    || last.is_some_and(|d| {
                        d.submission.outcome != LocalInputCorrectionDecisionV1::Clarify
                    })
                {
                    return Err(refuse(
                        "correction_stale_head",
                        "exact current unresolved request head required",
                    ));
                }
            }
        }
        let path = decision_path(&locator, "request.json");
        let record = match optional(root, &path)? {
            RepositoryReadV1::Found(bytes) => {
                let record: LocalInputCorrectionDecisionRecordV1 = decode(&bytes)?;
                if record.locator != locator
                    || record.submission != *submission
                    || record.request_fingerprint != request.locator.request_fingerprint
                    || record.decision_fingerprint != decision_identity(&record)?
                    || encode(&record)? != bytes
                {
                    return Err(refuse(
                        "correction_decision_conflict",
                        "original submission key retains different content",
                    ));
                }
                record
            }
            RepositoryReadV1::Absent => {
                let mut record = LocalInputCorrectionDecisionRecordV1 {
                    schema_ref: LOCAL_INPUT_CORRECTION_RULE_V1.into(),
                    locator: locator.clone(),
                    submission: submission.clone(),
                    request_fingerprint: request.locator.request_fingerprint.clone(),
                    decision_fingerprint: String::new(),
                    recorded_at_utc: now()?,
                };
                record.decision_fingerprint = decision_identity(&record)?;
                record
            }
        };
        let record_bytes = encode(&record)?;
        let locator_bytes = encode(&locator)?;
        effects = true;
        root.create_directories(&format!(
            "{LOCAL_INPUT_CORRECTION_ROOT_V1}/requests/{}/decisions/{}",
            locator.request_key, locator.submission_key
        ))
        .map_err(fs_error)?;
        root.create_directories(&format!(
            "{LOCAL_INPUT_CORRECTION_ROOT_V1}/requests/{}/heads",
            locator.request_key
        ))
        .map_err(fs_error)?;
        write_exact(root, &path, &record_bytes)?;
        // One immutable head slot serializes incompatible submissions. Exact
        // original-key recovery completes a reserved interrupted submission.
        write_exact(root, &slot, &locator_bytes)?;
        interrupt();
        tree.validate().map_err(fs_error)?;
        files.validate().map_err(fs_error)?;
        candidate_files.validate().map_err(fs_error)?;
        current_delegation(root, authority, delegation, basis)?;
        guard.validate()?;
        write_exact(
            root,
            &decision_path(&locator, "outcome.json"),
            &encode(&record)?,
        )?;
        write_exact(
            root,
            &decision_path(&locator, "terminal.json"),
            &encode(&record)?,
        )?;
        read_decision(root, &request, &locator)
    })();
    execute(result, effects, replay)
}

#[cfg(test)]
mod tests;
