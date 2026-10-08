//! Additive lineage and exact source preparation share the original owner lock.
use super::*;
use crate::local_evaluation_followup::validate_followup_capture;
use handbook_engine::local_authority::local_evaluation::correction::{
    LocalInputCorrectionAdmissionV1, LocalInputCorrectionCandidateSourceV1,
    LocalInputCorrectionCandidateV1, LocalInputCorrectionDecisionV1,
    LOCAL_INPUT_CORRECTION_CANDIDATE_SCHEMA_V1,
};
use serde_json::Value;

const CANDIDATE_ROOT: &str =
    ".handbook/state/contracts/local-evaluation-v3/input-correction-candidates";

pub(super) struct FormationFollowup {
    pub input: LocalEvaluationFollowupInputV1,
    pub decision_provenance: Option<Value>,
    pub dependencies: Vec<LocalEvaluationRetentionDependencyV3>,
}

impl ContractOwnerRepositoryV1 {
    /// Reproduce a selected actual refusal outside the sealed Engine guard.
    pub fn prepare_local_input_correction_candidate_v1(
        &self,
        project: &crate::project_admission_governance::ProjectAdmissionBindingV3,
        prepared: &PreparedLocalEvaluationFollowupV1,
        rejected: LocalEvaluationPrepareInputV3,
    ) -> Result<PreparedLocalInputCorrectionCandidateV1, ContractOwnerRepositoryErrorV1> {
        if prepared.followup.action != LocalEvaluationFollowupActionV1::CorrectedInput {
            return Err(invalid(
                "selected correction candidate differs from exact prepared authority",
            ));
        }
        let original = self.with_existing_full_definition_lock(|owner| {
            owner.recover_locked()?;
            owner.validate_inventory()?;
            owner.validate_followup_original_locked(&prepared.followup)
        })?;
        let mut expected_rejected = prepared.followup.input.clone();
        expected_rejected.subject = original.original_input.subject.clone();
        expected_rejected.case = original.original_input.case.clone();
        if rejected != expected_rejected {
            return Err(invalid(
            "unresolved candidate must name old exact subject/case and proposed corrected input"));
        }
        let refusal = self
            .prepare_local_evaluation_v3(project, rejected.clone())
            .err()
            .ok_or_else(|| invalid("selected unresolved candidate was not actually refused"))?;
        if refusal.detail()
            != "exact captured subject/case/lifecycle/Resolution must close formation"
        {
            return Err(invalid(
                "selected unresolved candidate failed a different owner condition",
            ));
        }
        let current = self.prepare_local_evaluation_v3(project, prepared.followup.input.clone())?;
        if current.request_locator() != prepared.request_locator() {
            return Err(substituted(
                "prospective correction changed during refusal reproduction",
            ));
        }
        Ok(PreparedLocalInputCorrectionCandidateV1 {
            followup: prepared.followup.clone(),
            rejected,
            request: prepared.request_locator(),
            basis: prepared.current_basis().clone(),
            original_state: original,
        })
    }

    /// Retain owner-established proof under the held Engine admission and lock.
    pub fn retain_local_input_correction_candidate_v1(
        &self,
        prepared: &PreparedLocalEvaluationFollowupV1,
        candidate: PreparedLocalInputCorrectionCandidateV1,
        admission: &LocalEvaluationTermsAdmissionV1<'_>,
    ) -> Result<LocalInputCorrectionCandidateSourceV1, ContractOwnerRepositoryErrorV1> {
        admission
            .require_repository_root(&self.repository_root)
            .map_err(|_| {
                invalid("correction candidate authority belongs to another native owner root")
            })?;
        admission
            .require_input_correction()
            .map_err(|_| invalid("InputCorrection is not delegated under exact prior terms"))?;
        admission
            .require_owner_write_subtree(CANDIDATE_ROOT)
            .map_err(|_| invalid("correction candidate write subtree was not declared"))?;
        for required in [".handbook/state/contracts", LOCK_RELATIVE_PATH] {
            admission
                .require_input_path(required)
                .map_err(|_| invalid("correction candidate omits native original proof reach"))?;
        }
        if candidate.followup != prepared.followup
            || candidate.request != prepared.request_locator()
            || candidate.basis != *admission.current_basis()
            || admission.request() != prepared.formation_request()
            || admission.current_basis() != prepared.current_basis()
        {
            return Err(invalid(
                "sealed refused candidate does not match exact prospective correction",
            ));
        }
        let rejected = candidate.rejected;
        let original_state = candidate.original_state;
        let original_state_bytes = canonical_bytes(&original_state).map_err(map_contract_input)?;
        let original_state_path = eval_ref("records", original_state.identity.fingerprint());
        admission
            .require_input_path(&original_state_path)
            .map_err(|_| {
                invalid("correction candidate omitted exact original immutable source reach")
            })?;
        let candidate = LocalInputCorrectionCandidateV1 {
            schema_ref: LOCAL_INPUT_CORRECTION_CANDIDATE_SCHEMA_V1.into(),
            owner_identity: prepared
                .followup
                .original
                .evaluation
                .owner_identity
                .as_str()
                .into(),
            project_identity: admission.project_identity().into(),
            original_result: handbook_engine::local_authority::LocalBindingV1 {
                reference: prepared.followup.original.result.reference().into(),
                fingerprint: prepared
                    .followup
                    .original
                    .result
                    .fingerprint()
                    .as_str()
                    .into(),
            },
            rejected_candidate_fingerprint: canonical_fingerprint(&rejected)
                .map_err(map_contract_input)?
                .as_str()
                .into(),
            missing_condition: "subject_case_input_congruence".into(),
            original_input_fingerprint: canonical_fingerprint(&prepared.followup.original_capture)
                .map_err(map_contract_input)?
                .as_str()
                .into(),
            proposed_input_fingerprint: canonical_fingerprint(&prepared.followup.capture)
                .map_err(map_contract_input)?
                .as_str()
                .into(),
            formation_request_key: prepared.request_locator().request_key,
            formation_request_fingerprint: prepared
                .request_locator()
                .request_fingerprint
                .as_str()
                .into(),
            rejected_candidate: serde_json::to_value(&rejected)
                .map_err(|_| invalid("rejected candidate cannot be represented"))?,
            prospective_candidate: serde_json::to_value(&prepared.followup.input)
                .map_err(|_| invalid("prospective candidate cannot be represented"))?,
            original_capture: prepared.followup.original_capture.clone(),
            proposed_capture: prepared.followup.capture.clone(),
            original_state_path: original_state_path.clone(),
            original_state_bytes_fingerprint: FingerprintV1::from_bytes(&original_state_bytes)
                .as_str()
                .into(),
            original_state_identity: handbook_engine::local_authority::LocalBindingV1 {
                reference: original_state.identity.reference().into(),
                fingerprint: original_state.identity.fingerprint().as_str().into(),
            },
        };
        let bytes = candidate.canonical_json_bytes().map_err(|_| {
            invalid("complete native correction candidate violates its bounded source format")
        })?;
        if bytes.len() > MAX_RECORD_BYTES {
            return Err(bound(
                "retained correction candidate exceeds owner record bound",
            ));
        }
        let fingerprint = FingerprintV1::from_bytes(&bytes);
        let path = format!("{CANDIDATE_ROOT}/{}.json", fingerprint_hex(&fingerprint));
        self.with_existing_full_definition_lock(|owner| {
            owner.recover_locked()?;
            owner.validate_inventory()?;
            let current_original = owner.validate_followup_original_locked(&prepared.followup)?;
            if current_original.identity != original_state.identity
                || owner.read_regular(&owner.repo_root.join(&original_state_path))?
                    != original_state_bytes
            {
                return Err(substituted(
                    "actual original closed result source changed before candidate retention",
                ));
            }
            let (definition, head) = owner.full_local_bundle(&prepared.followup.input.contract)?;
            if definition.definition() != prepared.prepared.local_use.definition()
                || head.lifecycle_fingerprint()
                    != prepared.prepared.local_use.lifecycle_head_fingerprint()
            {
                return Err(substituted(
                    "exact correction definition changed before source retention",
                ));
            }
            admission
                .validate_current()
                .map_err(|_| invalid("correction candidate authority changed before retention"))?;
            owner
                .repository_root
                .create_directories(CANDIDATE_ROOT)
                .map_err(map_durability)?;
            match owner
                .repository_root
                .read_regular_bounded(&path, MAX_RECORD_BYTES)
                .map_err(map_durability)?
            {
                RepositoryReadV1::Found(existing) if existing == bytes => {}
                RepositoryReadV1::Found(_) => {
                    return Err(substituted("retained correction candidate substituted"))
                }
                RepositoryReadV1::Absent => owner
                    .repository_root
                    .write_new_durable(&path, &bytes)
                    .map_err(map_durability)?,
            }
            admission
                .validate_current()
                .map_err(|_| invalid("correction candidate authority changed after retention"))?;
            Ok(())
        })?;
        Ok(LocalInputCorrectionCandidateSourceV1 {
            relative_path: path,
            canonical_record_bytes: bytes,
            record_fingerprint: fingerprint.as_str().into(),
        })
    }
    pub fn prepare_local_evaluation_followup_v1(
        &self,
        project: &crate::project_admission_governance::ProjectAdmissionBindingV3,
        input: LocalEvaluationFollowupInputV1,
    ) -> Result<PreparedLocalEvaluationFollowupV1, ContractOwnerRepositoryErrorV1> {
        input.canonical_json_bytes().map_err(map_contract_input)?;
        self.with_existing_full_definition_lock(|owner| {
            owner.recover_locked()?;
            owner.validate_inventory()?;
            owner.validate_followup_original_locked(&input)?;
            Ok(())
        })?;
        let prepared = self.prepare_local_evaluation_v3(project, input.input.clone())?;
        Ok(PreparedLocalEvaluationFollowupV1 {
            followup: input,
            prepared,
        })
    }

    pub fn form_local_evaluation_followup_v1(
        &self,
        prepared: PreparedLocalEvaluationFollowupV1,
        admission: &LocalEvaluationTermsAdmissionV1<'_>,
    ) -> Result<LocalEvaluationSnapshotV3, ContractOwnerRepositoryErrorV1> {
        self.form_local_evaluation_with_followup_v3(
            prepared.prepared,
            admission,
            Some(FormationFollowup {
                input: prepared.followup,
                decision_provenance: None,
                dependencies: vec![],
            }),
            None,
        )
    }

    /// Deliver an already committed exact followup before attempting fresh use.
    /// This creates no absent intent and validates no current execution authority.
    pub fn recover_local_evaluation_followup_v1(
        &self,
        prepared: &PreparedLocalEvaluationFollowupV1,
    ) -> Result<
        Option<(LocalEvaluationSnapshotV3, LocalEvaluationLineageV1)>,
        ContractOwnerRepositoryErrorV1,
    > {
        self.with_existing_full_definition_lock(|owner| {
            owner.recover_locked()?;
            owner.validate_inventory()?;
            let request = prepared.request_locator();
            let Some(ledger) = owner.eval_ledger(&eval_key(&request.request_key)?)? else {
                return Ok(None);
            };
            if ledger.request_fingerprint != request.request_fingerprint {
                return Err(repository_error(
                    ContractOwnerRepositoryErrorKindV1::IdempotencyConflict,
                    "historical followup key conflicts with original exact formation",
                ));
            }
            let state = owner.eval_current(&ledger.locator)?;
            let lineage = owner.eval_lineage_locked(&ledger.locator)?.ok_or_else(|| {
                repository_error(
                    ContractOwnerRepositoryErrorKindV1::IdempotencyConflict,
                    "historical key names ordinary formation without selected followup lineage",
                )
            })?;
            if state.original_input != prepared.followup.input
                || lineage.action != prepared.followup.action
                || lineage.original != prepared.followup.original
                || canonical_fingerprint(&lineage.original_capture).map_err(map_contract_input)?
                    != canonical_fingerprint(&prepared.followup.original_capture)
                        .map_err(map_contract_input)?
                || canonical_fingerprint(&lineage.capture).map_err(map_contract_input)?
                    != canonical_fingerprint(&prepared.followup.capture)
                        .map_err(map_contract_input)?
            {
                return Err(repository_error(
                    ContractOwnerRepositoryErrorKindV1::IdempotencyConflict,
                    "historical key conflicts with original selected followup lineage",
                ));
            }
            Ok(Some((state, lineage)))
        })
    }

    /// The sealed Engine permission remains held through owner comparison/commit.
    pub fn form_decided_local_evaluation_followup_v1(
        &self,
        prepared: PreparedLocalEvaluationFollowupV1,
        admission: &LocalInputCorrectionAdmissionV1<'_>,
    ) -> Result<LocalEvaluationSnapshotV3, ContractOwnerRepositoryErrorV1> {
        admission
            .validate_current()
            .map_err(|_| invalid("correction decision is no longer current"))?;
        let candidate = admission.candidate();
        let request = &admission.request().request;
        let followup = &prepared.followup;
        if followup.action != LocalEvaluationFollowupActionV1::CorrectedInput
            || candidate.owner_identity != followup.original.evaluation.owner_identity.as_str()
            || candidate.original_result.reference != followup.original.result.reference()
            || candidate.original_result.fingerprint
                != followup.original.result.fingerprint().as_str()
            || candidate.original_input_fingerprint
                != canonical_fingerprint(&followup.original_capture)
                    .map_err(map_contract_input)?
                    .as_str()
            || candidate.proposed_input_fingerprint
                != canonical_fingerprint(&followup.capture)
                    .map_err(map_contract_input)?
                    .as_str()
            || candidate.formation_request_key != prepared.request_locator().request_key
            || candidate.formation_request_fingerprint
                != prepared.request_locator().request_fingerprint.as_str()
            || candidate.prospective_candidate
                != serde_json::to_value(&followup.input)
                    .map_err(|_| invalid("prospective candidate cannot be represented"))?
            || candidate.original_capture != followup.original_capture
            || candidate.proposed_capture != followup.capture
            || request.current_basis != *prepared.current_basis()
            || request.formation != *prepared.formation_request()
            || admission.decision().submission.outcome != LocalInputCorrectionDecisionV1::Approve
        {
            return Err(invalid("accepted decision does not name exact semantic correction and prospective formation"));
        }
        let source = admission.candidate_source();
        let retained: LocalInputCorrectionCandidateV1 =
            parse_canonical(&source.canonical_record_bytes)?;
        if retained != *candidate
            || source.record_fingerprint
                != FingerprintV1::from_bytes(&source.canonical_record_bytes).as_str()
        {
            return Err(substituted(
                "decision candidate is not exact native retained correction source",
            ));
        }
        let until = SemanticTimeV1::parse(&followup.input.formation.retention_until_utc)
            .map_err(map_contract_input)?;
        let dependencies = admission
            .dependency_sources()
            .iter()
            .map(|dependency| LocalEvaluationRetentionDependencyV3 {
                owner: if dependency.relative_path.contains("/contracts/") {
                    "handbook-contracts"
                } else {
                    "handbook-engine"
                }
                .into(),
                path: dependency.relative_path.clone(),
                bytes_fingerprint: FingerprintV1::from_bytes(&dependency.canonical_record_bytes),
                retained_until: until.clone(),
            })
            .collect();
        let provenance = json!({"delegation": admission.delegation(),
            "request": {"locator": admission.request().locator, "head_fingerprint": admission.request().head_fingerprint,
                "retained_at_utc": admission.request().retained_at_utc, "submitter": request.submitter,
                "context_source_ref": request.context_source_ref, "additional_context": request.additional_context,
                "current_basis": request.current_basis, "formation": request.formation},
            "decision": admission.decision(), "candidate_source": {
                "relative_path": source.relative_path, "record_fingerprint": source.record_fingerprint}});
        self.form_local_evaluation_with_followup_v3(
            prepared.prepared,
            admission.terms_admission(),
            Some(FormationFollowup {
                input: prepared.followup,
                decision_provenance: Some(provenance),
                dependencies,
            }),
            Some(admission),
        )
    }

    /// Immutable additive lineage can be delivered after authority revocation.
    pub fn read_local_evaluation_lineage_v1(
        &self,
        evaluation: &LocalEvaluationLocatorV3,
    ) -> Result<Option<LocalEvaluationLineageV1>, ContractOwnerRepositoryErrorV1> {
        self.with_existing_full_definition_lock(|owner| {
            owner.recover_locked()?;
            owner.validate_inventory()?;
            owner.eval_current(evaluation)?;
            owner.eval_lineage_locked(evaluation)
        })
    }

    pub(super) fn eval_lineage_locked(
        &self,
        evaluation: &LocalEvaluationLocatorV3,
    ) -> Result<Option<LocalEvaluationLineageV1>, ContractOwnerRepositoryErrorV1> {
        if !self.exists(
            &self
                .repo_root
                .join(format!("{LOCAL_EVALUATION_OWNER_ROOT_V3}/lineage")),
        )? {
            return Ok(None);
        }
        let path = self
            .repo_root
            .join(eval_ref("lineage", evaluation.occurrence.fingerprint()));
        if !self.exists(&path)? {
            return Ok(None);
        }
        let lineage = LocalEvaluationLineageV1::from_json_bytes(&self.read_regular(&path)?)
            .map_err(map_contract_input)?;
        lineage.validate_identity().map_err(map_contract_input)?;
        if lineage.evaluation != *evaluation {
            return Err(substituted("lineage evaluation changed"));
        }
        Ok(Some(lineage))
    }

    pub(super) fn validate_followup_original_locked(
        &self,
        input: &LocalEvaluationFollowupInputV1,
    ) -> Result<LocalEvaluationSnapshotV3, ContractOwnerRepositoryErrorV1> {
        validate_key(&input.input.request_key)?;
        let original = self.eval_current(&input.original.evaluation)?;
        let result = original.result.as_ref().ok_or_else(|| invalid(
            "fresh followup requires an exact closed result; uncertain attempts use original recovery"))?;
        if original.stage != LocalEvaluationStageV3::ResultClosed
            || result.identity != input.original.result
            || input.input.request_key == original.locator.request_key
            || input.input.contract != original.definition.binding()
            || input.input.requested_resolution != original.original_input.requested_resolution
            || input.input.case.case_id() != original.original_input.case.case_id()
            || input.input.formation.scope_fingerprint
                != original.original_input.formation.scope_fingerprint
            || input.input.formation.objective != original.original_input.formation.objective
            || input.input.formation.use_intent != original.original_input.formation.use_intent
            || input.input.formation.requested_reach
                != original.original_input.formation.requested_reach
            || input.input.formation.retention_until_utc > original.purpose.retention_until_utc
        {
            return Err(invalid(
                "followup must preserve exact closed original and independently reviewed criteria",
            ));
        }
        validate_followup_capture(
            &input.original_capture,
            &original.original_input.subject,
            &original.original_input.case,
            original.definition.dependencies(),
        )
        .map_err(|_| {
            invalid("original captured input does not match exact retained result question")
        })?;
        validate_followup_capture(
            &input.capture,
            &input.input.subject,
            &input.input.case,
            original.definition.dependencies(),
        )
        .map_err(|_| {
            invalid(
                "proposed capture does not match corrected subject/case and exact original schema",
            )
        })?;
        if let Some(intent) = &original.origin_intent {
            let bytes = intent["request_jcs"]
                .as_str()
                .ok_or_else(|| recovery("original native capture missing"))?;
            let request: Value = parse_canonical(bytes.as_bytes())?;
            if request["inputs"].as_array().map(Vec::len) != Some(1)
                || request["inputs"][0] != input.original_capture
            {
                return Err(substituted(
                    "original capture differs from retained native request",
                ));
            }
        }
        if (input.action == LocalEvaluationFollowupActionV1::FreshRepeat)
            != (canonical_fingerprint(&input.capture).map_err(map_contract_input)?
                == canonical_fingerprint(&input.original_capture).map_err(map_contract_input)?)
        {
            return Err(invalid(
                "fresh repeat requires equal capture; correction requires changed captured input",
            ));
        }
        Ok(original)
    }

    pub(super) fn form_followup_lineage_locked(
        &self,
        followup: &FormationFollowup,
        state: &LocalEvaluationSnapshotV3,
        admission: &LocalEvaluationTermsAdmissionV1<'_>,
    ) -> Result<LocalEvaluationLineageV1, ContractOwnerRepositoryErrorV1> {
        let original = self.validate_followup_original_locked(&followup.input)?;
        let retained_until = SemanticTimeV1::parse(&state.purpose.retention_until_utc)
            .map_err(map_contract_input)?;
        let mut dependencies = followup.dependencies.clone();
        let mut paths = self.eval_immutable_history_paths(&original.locator)?;
        paths.push(eval_ref("records", original.identity.fingerprint()));
        if let Some(lineage) = self.eval_lineage_locked(&original.locator)? {
            paths.push(eval_ref(
                "lineage",
                original.locator.occurrence.fingerprint(),
            ));
            dependencies.extend(lineage.dependencies);
        }
        for path in paths {
            admission
                .require_input_path(&path)
                .map_err(|_| invalid("new lineage omitted actual original history proof reach"))?;
            dependencies.push(LocalEvaluationRetentionDependencyV3 {
                owner: "handbook-contracts".into(),
                bytes_fingerprint: FingerprintV1::from_bytes(
                    &self.read_regular(&self.repo_root.join(&path))?,
                ),
                path,
                retained_until: retained_until.clone(),
            });
        }
        for dependency in &original.result.as_ref().unwrap().dependencies {
            admission
                .require_input_path(&dependency.path)
                .map_err(|_| {
                    invalid("new lineage omitted exact original dependency proof reach")
                })?;
            if retained_evaluation_dependency_fingerprint(
                &self.repository_root,
                &dependency.path,
                &original.retained_lock_generations,
            )? != Some(dependency.bytes_fingerprint.clone())
            {
                return Err(substituted(
                    "original result dependency unavailable or substituted before new lineage",
                ));
            }
            dependencies.push(dependency.clone());
        }
        dependencies.sort_by(|a, b| a.path.cmp(&b.path));
        dependencies
            .dedup_by(|a, b| a.path == b.path && a.bytes_fingerprint == b.bytes_fingerprint);
        let mut lineage = LocalEvaluationLineageV1 {
            schema_ref: "handbook.contract-local-evaluation-lineage@1.0.0".into(),
            action: followup.input.action,
            original: followup.input.original.clone(),
            original_capture: followup.input.original_capture.clone(),
            capture: followup.input.capture.clone(),
            evaluation: state.locator.clone(),
            decision_provenance: followup.decision_provenance.clone(),
            dependencies,
            retained_until,
            identity: identified("handbook.contract-local-evaluation-lineage@1.0.0", &())
                .map_err(map_contract_input)?,
        };
        lineage.identity = identified(
            "handbook.contract-local-evaluation-lineage@1.0.0",
            &without_identity(&lineage).map_err(map_contract_input)?,
        )
        .map_err(map_contract_input)?;
        lineage.validate_identity().map_err(map_contract_input)?;
        Ok(lineage)
    }

    pub(super) fn validate_lineage_state_locked(
        &self,
        lineage: &LocalEvaluationLineageV1,
        state: &LocalEvaluationSnapshotV3,
    ) -> Result<(), ContractOwnerRepositoryErrorV1> {
        lineage.validate_identity().map_err(map_contract_input)?;
        if lineage.evaluation != state.locator
            || lineage.retained_until.as_str() != state.purpose.retention_until_utc
        {
            return Err(recovery(
                "lineage does not name exact committed occurrence and horizon",
            ));
        }
        self.validate_followup_original_locked(&LocalEvaluationFollowupInputV1 {
            action: lineage.action,
            original: lineage.original.clone(),
            original_capture: lineage.original_capture.clone(),
            capture: lineage.capture.clone(),
            input: state.original_input.clone(),
        })?;
        Ok(())
    }

    pub(super) fn validate_correction_candidate_inventory(
        &self,
    ) -> Result<(), ContractOwnerRepositoryErrorV1> {
        // Native listing requires an existing parent. An absent evaluation
        // namespace has no candidate inventory and must stay read-only.
        if !self.exists(&self.repo_root.join(LOCAL_EVALUATION_OWNER_ROOT_V3))? {
            return Ok(());
        }
        let directory = self.repo_root.join(CANDIDATE_ROOT);
        if !self.exists(&directory)? {
            return Ok(());
        }
        for path in self.read_dir_bounded(&directory, MAX_RECORDS)? {
            let bytes = self.read_regular(&path)?;
            let candidate: LocalInputCorrectionCandidateV1 = parse_canonical(&bytes)?;
            if candidate.canonical_json_bytes().map_err(|_| {
                recovery("retained correction candidate violates native closed source format")
            })? != bytes
            {
                return Err(recovery(
                    "retained correction candidate canonical source changed",
                ));
            }
            let fingerprint = FingerprintV1::from_bytes(&bytes);
            if candidate.schema_ref != LOCAL_INPUT_CORRECTION_CANDIDATE_SCHEMA_V1
                || candidate.owner_identity
                    != self
                        .repository_root
                        .governance_physical_root_fingerprint_v1()
                        .map_err(map_durability)?
                || candidate.missing_condition != "subject_case_input_congruence"
                || file_name(&path)? != format!("{}.json", fingerprint_hex(&fingerprint))
                || FingerprintV1::parse(&candidate.rejected_candidate_fingerprint).is_err()
                || FingerprintV1::parse(&candidate.original_input_fingerprint).is_err()
                || FingerprintV1::parse(&candidate.proposed_input_fingerprint).is_err()
                || FingerprintV1::parse(&candidate.formation_request_fingerprint).is_err()
                || candidate.original_input_fingerprint == candidate.proposed_input_fingerprint
            {
                return Err(recovery(
                    "retained correction candidate identity or owner changed",
                ));
            }
            validate_key(&candidate.formation_request_key)?;
        }
        Ok(())
    }
}
