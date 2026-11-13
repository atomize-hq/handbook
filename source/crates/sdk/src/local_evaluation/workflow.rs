//! Explicit fresh followup and caller-carried correction decisions.
//!
//! These operations create new truth only after native admission. Original
//! request recovery and historical delivery remain the separate parent APIs.
use super::*;
use handbook_engine::local_authority::local_evaluation::correction::NativeLocalInputCorrectionV1;

pub use handbook_contracts::{
    LocalEvaluationFollowupActionV1, LocalEvaluationFollowupInputV1, LocalEvaluationLineageV1,
    PreparedLocalEvaluationFollowupV1,
};
pub use handbook_engine::local_authority::local_evaluation::correction::{
    LocalInputCorrectionCandidateSourceV1, LocalInputCorrectionDecisionLocatorV1,
    LocalInputCorrectionDecisionRecordV1, LocalInputCorrectionDecisionV1,
    LocalInputCorrectionDelegationLocatorV1, LocalInputCorrectionDelegationOutcomeV1,
    LocalInputCorrectionDelegationV1, LocalInputCorrectionExecutionV1,
    LocalInputCorrectionRequestLocatorV1, LocalInputCorrectionRequestRecordV1,
    LocalInputCorrectionRequestV1, LocalInputCorrectionSubmissionV1,
    PreparedLocalInputCorrectionDelegationV1,
};

impl HandbookSdkV1 {
    /// Prepares a deliberately new repeat or corrected capture from an exact
    /// closed result. It neither recovers an uncertain attempt nor allocates.
    pub fn prepare_local_evaluation_followup(
        &self,
        initial: &LocalInitialSetupLocatorV1,
        authority: &LocalEvaluationAuthorityLocatorV1,
        input: LocalEvaluationFollowupInputV1,
    ) -> Result<PreparedLocalEvaluationFollowupV1, LocalEvaluationSdkErrorV1> {
        require_initial(initial, authority)?;
        let basis = self.observe_local_check_basis(initial)?;
        NativeLocalEvaluationAuthorityV1::with_terms_admission(
            &self.repo_root,
            authority,
            &basis,
            &input.input.formation,
            |admission| {
                admission.require_owner_write_subtree(".handbook/state/contracts")?;
                admission.require_input_path(".handbook/state/contracts")?;
                admission.require_input_path(".handbook/state/locks/contract-owner.lock")?;
                Ok(())
            },
        )?;
        Ok(
            ContractOwnerRepositoryV1::open_existing_for_local_use_v3(&self.repo_root)?
                .prepare_local_evaluation_followup_v1(&project(initial), input)?,
        )
    }

    /// Routine permitted correction and repetition need only the original live
    /// executor terms. A decision agent is optional for an unresolved choice.
    pub fn form_local_evaluation_followup(
        &self,
        authority: &LocalEvaluationAuthorityLocatorV1,
        prepared: PreparedLocalEvaluationFollowupV1,
    ) -> Result<LocalEvaluationSnapshotV3, LocalEvaluationSdkErrorV1> {
        let history = ContractOwnerRepositoryV1::open_existing_for_local_evaluation_history_v3(
            &self.repo_root,
        )?;
        if let Some((snapshot, lineage)) =
            history.recover_local_evaluation_followup_v1(&prepared)?
        {
            if lineage.decision_provenance.is_some() {
                return Err(engine_error(
                    "correction_replay_conflict",
                    "original formation consumed a decision; use its exact decision locators",
                )
                .into());
            }
            return Ok(snapshot);
        }
        let basis = prepared.current_basis().clone();
        let formation = prepared.formation_request().clone();
        Ok(NativeLocalEvaluationAuthorityV1::with_terms_admission(
            &self.repo_root,
            authority,
            &basis,
            &formation,
            |admission| {
                Ok(
                    ContractOwnerRepositoryV1::open_existing_for_local_use_v3(&self.repo_root)
                        .and_then(|owner| {
                            owner.form_local_evaluation_followup_v1(prepared, admission)
                        }),
                )
            },
        )??)
    }

    pub fn read_local_evaluation_lineage(
        &self,
        original: &LocalEvaluationLocatorV3,
    ) -> Result<Option<LocalEvaluationLineageV1>, LocalEvaluationSdkErrorV1> {
        Ok(
            ContractOwnerRepositoryV1::open_existing_for_local_evaluation_history_v3(
                &self.repo_root,
            )?
            .read_local_evaluation_lineage_v1(original)?,
        )
    }

    /// Retains Contracts' actual inconsistent candidate and its exact consistent
    /// correction. A caller-authored refusal or context DTO cannot replace it.
    pub fn retain_local_input_correction_candidate(
        &self,
        initial: &LocalInitialSetupLocatorV1,
        authority: &LocalEvaluationAuthorityLocatorV1,
        prepared: &PreparedLocalEvaluationFollowupV1,
        rejected: LocalEvaluationPrepareInputV3,
    ) -> Result<LocalInputCorrectionCandidateSourceV1, LocalEvaluationSdkErrorV1> {
        require_initial(initial, authority)?;
        // Contracts reproduces the rejected candidate and resolves fresh
        // applicability before the consuming Engine guard is acquired.
        let owner = ContractOwnerRepositoryV1::open_existing_for_local_use_v3(&self.repo_root)?;
        let candidate = owner.prepare_local_input_correction_candidate_v1(
            &project(initial),
            prepared,
            rejected,
        )?;
        let basis = prepared.current_basis().clone();
        let formation = prepared.formation_request().clone();
        Ok(NativeLocalEvaluationAuthorityV1::with_terms_admission(
            &self.repo_root,
            authority,
            &basis,
            &formation,
            |admission| {
                Ok(
                    owner
                        .retain_local_input_correction_candidate_v1(prepared, candidate, admission),
                )
            },
        )??)
    }

    /// Approval may be supplied upfront. Adoption still follows establishment
    /// of the exact native evaluation predecessors and preserves their executor.
    pub fn prepare_local_input_correction_delegation(
        &self,
        initial: &LocalInitialSetupLocatorV1,
        authority: &LocalEvaluationAuthorityLocatorV1,
        delegation: LocalInputCorrectionDelegationV1,
    ) -> Result<PreparedLocalInputCorrectionDelegationV1, LocalAuthorityRefusalV1> {
        require_initial(initial, authority)?;
        let basis = self.observe_local_check_basis(initial)?;
        NativeLocalInputCorrectionV1::prepare_delegation(
            &self.repo_root,
            authority,
            &basis,
            delegation,
        )
    }

    pub fn adopt_local_input_correction_delegation(
        &self,
        authority: &LocalEvaluationAuthorityLocatorV1,
        prepared: PreparedLocalInputCorrectionDelegationV1,
    ) -> LocalInputCorrectionExecutionV1<LocalInputCorrectionDelegationOutcomeV1> {
        NativeLocalInputCorrectionV1::adopt_delegation(&self.repo_root, authority, prepared)
    }

    pub fn recover_local_input_correction_delegation(
        &self,
        authority: &LocalEvaluationAuthorityLocatorV1,
        original: &LocalInputCorrectionDelegationLocatorV1,
    ) -> LocalInputCorrectionExecutionV1<LocalInputCorrectionDelegationOutcomeV1> {
        NativeLocalInputCorrectionV1::recover_delegation(&self.repo_root, authority, original)
    }

    /// Completes only an existing original adoption intent. An absent intent
    /// cannot be created by presenting a portable locator.
    pub fn resume_local_input_correction_delegation(
        &self,
        authority: &LocalEvaluationAuthorityLocatorV1,
        original: &LocalInputCorrectionDelegationLocatorV1,
    ) -> LocalInputCorrectionExecutionV1<LocalInputCorrectionDelegationOutcomeV1> {
        NativeLocalInputCorrectionV1::resume_delegation(&self.repo_root, authority, original)
    }

    pub fn retain_local_input_correction_request(
        &self,
        initial: &LocalInitialSetupLocatorV1,
        authority: &LocalEvaluationAuthorityLocatorV1,
        delegation: &LocalInputCorrectionDelegationLocatorV1,
        input: &LocalInputCorrectionRequestV1,
    ) -> Result<
        LocalInputCorrectionExecutionV1<LocalInputCorrectionRequestRecordV1>,
        LocalAuthorityRefusalV1,
    > {
        require_initial(initial, authority)?;
        let adopted = NativeLocalInputCorrectionV1::recover_delegation(
            &self.repo_root,
            authority,
            delegation,
        );
        let Some(adopted_record) = adopted.outcome else {
            return Ok(LocalInputCorrectionExecutionV1 {
                disposition: adopted.disposition,
                outcome: None,
                refusal: adopted.refusal,
            });
        };
        let original = input.locator(&adopted_record.delegation_fingerprint)?;
        let retained = NativeLocalInputCorrectionV1::recover_request(
            &self.repo_root,
            authority,
            delegation,
            &original,
        );
        if retained.outcome.is_some()
            || !retained.refusal.as_ref().is_some_and(|refusal| {
                matches!(
                    refusal.code.as_str(),
                    "correction_request_absent" | "correction_request_pending",
                )
            })
        {
            return Ok(retained);
        }
        let basis = self.observe_local_check_basis(initial)?;
        Ok(NativeLocalInputCorrectionV1::retain_request(
            &self.repo_root,
            authority,
            delegation,
            &basis,
            input,
        ))
    }

    pub fn submit_local_input_correction_decision(
        &self,
        initial: &LocalInitialSetupLocatorV1,
        authority: &LocalEvaluationAuthorityLocatorV1,
        delegation: &LocalInputCorrectionDelegationLocatorV1,
        request: &LocalInputCorrectionRequestLocatorV1,
        input: &LocalInputCorrectionSubmissionV1,
    ) -> Result<
        LocalInputCorrectionExecutionV1<LocalInputCorrectionDecisionRecordV1>,
        LocalAuthorityRefusalV1,
    > {
        require_initial(initial, authority)?;
        let original = input.locator(request)?;
        let retained = NativeLocalInputCorrectionV1::recover_decision(
            &self.repo_root,
            authority,
            delegation,
            request,
            &original,
        );
        if retained.outcome.is_some()
            || !retained.refusal.as_ref().is_some_and(|refusal| {
                matches!(
                    refusal.code.as_str(),
                    "correction_decision_absent" | "correction_decision_pending",
                )
            })
        {
            return Ok(retained);
        }
        // An exact completed decision is historical delivery. Only a proven
        // absent submission or this same pending intent needs live resumption.
        let basis = self.observe_local_check_basis(initial)?;
        Ok(NativeLocalInputCorrectionV1::submit_decision(
            &self.repo_root,
            authority,
            delegation,
            request,
            &basis,
            input,
        ))
    }

    pub fn recover_local_input_correction_request(
        &self,
        authority: &LocalEvaluationAuthorityLocatorV1,
        delegation: &LocalInputCorrectionDelegationLocatorV1,
        original: &LocalInputCorrectionRequestLocatorV1,
    ) -> LocalInputCorrectionExecutionV1<LocalInputCorrectionRequestRecordV1> {
        NativeLocalInputCorrectionV1::recover_request(
            &self.repo_root,
            authority,
            delegation,
            original,
        )
    }

    pub fn recover_local_input_correction_decision(
        &self,
        authority: &LocalEvaluationAuthorityLocatorV1,
        delegation: &LocalInputCorrectionDelegationLocatorV1,
        request: &LocalInputCorrectionRequestLocatorV1,
        original: &LocalInputCorrectionDecisionLocatorV1,
    ) -> LocalInputCorrectionExecutionV1<LocalInputCorrectionDecisionRecordV1> {
        NativeLocalInputCorrectionV1::recover_decision(
            &self.repo_root,
            authority,
            delegation,
            request,
            original,
        )
    }

    /// Consumes only the exact approved correction under a freshly resolved
    /// basis. Engine pins remain held through Contracts' atomic comparison and
    /// lineage/formation commit; original request recovery handles lost delivery.
    pub fn form_approved_local_input_correction(
        &self,
        initial: &LocalInitialSetupLocatorV1,
        authority: &LocalEvaluationAuthorityLocatorV1,
        delegation: &LocalInputCorrectionDelegationLocatorV1,
        request: &LocalInputCorrectionRequestLocatorV1,
        decision: &LocalInputCorrectionDecisionLocatorV1,
        prepared: PreparedLocalEvaluationFollowupV1,
    ) -> Result<LocalEvaluationSnapshotV3, LocalEvaluationSdkErrorV1> {
        require_initial(initial, authority)?;
        let history = ContractOwnerRepositoryV1::open_existing_for_local_evaluation_history_v3(
            &self.repo_root,
        )?;
        if let Some((snapshot, lineage)) =
            history.recover_local_evaluation_followup_v1(&prepared)?
        {
            let retained = NativeLocalInputCorrectionV1::recover_delegation(
                &self.repo_root,
                authority,
                delegation,
            );
            let adopted = retained.outcome.ok_or_else(|| {
                retained.refusal.unwrap_or_else(|| {
                    engine_error(
                        "correction_replay_conflict",
                        "original delegation is unavailable",
                    )
                })
            })?;
            let grant = serde_json::to_value(adopted).map_err(|_| {
                engine_error(
                    "correction_replay_conflict",
                    "original delegation cannot be represented",
                )
            })?;
            let request_value = serde_json::to_value(request).map_err(|_| {
                engine_error(
                    "correction_replay_conflict",
                    "original request cannot be represented",
                )
            })?;
            let decision_value = serde_json::to_value(decision).map_err(|_| {
                engine_error(
                    "correction_replay_conflict",
                    "original decision cannot be represented",
                )
            })?;
            let provenance = lineage.decision_provenance.as_ref().ok_or_else(|| {
                engine_error(
                    "correction_replay_conflict",
                    "original formation did not consume a decision",
                )
            })?;
            if provenance["delegation"] != grant
                || provenance["request"]["locator"] != request_value
                || provenance["decision"]["locator"] != decision_value
            {
                return Err(engine_error("correction_replay_conflict", "original formation requires its exact retained delegation, request and decision locators").into());
            }
            return Ok(snapshot);
        }
        let basis = self.observe_local_check_basis(initial)?;
        let formation = prepared.formation_request().clone();
        Ok(NativeLocalInputCorrectionV1::with_admission(
            &self.repo_root,
            authority,
            delegation,
            request,
            decision,
            &basis,
            &formation,
            &authority.terms.agent,
            |admission| {
                Ok(
                    ContractOwnerRepositoryV1::open_existing_for_local_use_v3(&self.repo_root)
                        .and_then(|owner| {
                            owner.form_decided_local_evaluation_followup_v1(prepared, admission)
                        }),
                )
            },
        )??)
    }
}
