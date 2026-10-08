//! Typed composition for one local advisory evaluation.
//!
//! Supplied approval and review records are inputs. Contracts, Engine and Dock
//! independently qualify them and retain the resulting owner observations.
use crate::admission_governance::ProjectAdmissionBindingV3;
use crate::local_check::LocalCheckLocatorV1;
use crate::local_setup::LocalInitialSetupLocatorV1;
use crate::HandbookSdkV1;
use handbook_contracts::{
    ContractOwnerRepositoryErrorV1, ContractOwnerRepositoryV1, ExactBindingV1, SemanticTimeV1,
};
use handbook_engine::local_authority::local_check::NativeLocalCheckAuthorityV1;
use handbook_engine::local_authority::local_evaluation::NativeLocalEvaluationAuthorityV1;
use handbook_engine::local_authority::LocalAuthorityRefusalV1;
use handbook_engine::ContextResolutionStackDefinition;

mod workflow;
pub use workflow::*;

pub use handbook_contracts::{
    LocalContractDefinitionInputV3, LocalContractDefinitionV3, LocalContractUseV3,
    LocalDefinitionDependenciesV3, LocalDefinitionRecordV3, LocalEvaluationGateV3,
    LocalEvaluationHistoryV3, LocalEvaluationLocatorV3, LocalEvaluationPrepareInputV3,
    LocalEvaluationReadinessV3, LocalEvaluationRequestLocatorV3, LocalEvaluationResultLocatorV3,
    LocalEvaluationResultV3, LocalEvaluationSnapshotV3, LocalEvaluationStageV3,
    LocalLifecycleHeadV3, LocalLifecycleTransitionInputV3, LocalOwnerMutationReceiptV3,
    LocalReviewInputV3, NativeLocalEvaluationCapabilityV3, PreparedLocalDefinitionAppendV3,
    PreparedLocalEvaluationV3, PreparedLocalLifecycleTransitionV3,
};
pub use handbook_dock::{DockLocalEvaluationAttemptOutcomeV1, DockLocalEvaluationReachV1};
pub use handbook_engine::local_authority::local_evaluation::{
    LocalEvaluationAuthorityExecutionV1, LocalEvaluationAuthorityLocatorV1,
    LocalEvaluationAuthorizationBindingV1, LocalEvaluationAuthorizationExecutionV1,
    LocalEvaluationAuthorizationRequestV1, LocalEvaluationBindingLocatorV1,
    LocalEvaluationCheckpointV1, LocalEvaluationCurrentStatusV1, LocalEvaluationEnvelopeV1,
    LocalEvaluationFormationRequestV1, LocalEvaluationHistoricalObservationV1,
    LocalEvaluationHistoricalStatusV1, LocalEvaluationMutationRuleV1, LocalEvaluationPathReachV1,
    LocalEvaluationPlanV1, LocalEvaluationPurposeV1, LocalEvaluationReachV1,
    LocalEvaluationSelectionV1, LocalEvaluationTermsV1, LocalEvaluationUseIntentV1,
    PreparedLocalEvaluationAuthorityV1, LOCAL_EVALUATION_RULE_V1,
};

/// Caller-carried exact locators and the original private request. This value
/// grants nothing; every consuming operation obtains a new live owner admission.
pub struct LocalEvaluationSessionV1<'a> {
    pub initial: &'a LocalInitialSetupLocatorV1,
    pub authority: &'a LocalEvaluationAuthorityLocatorV1,
    pub original_issuance: &'a LocalEvaluationBindingLocatorV1,
    pub authorization_request: &'a LocalEvaluationAuthorizationRequestV1,
    pub evaluation: &'a LocalEvaluationLocatorV3,
}

#[derive(Debug)]
pub enum LocalEvaluationSdkErrorV1 {
    Engine(LocalAuthorityRefusalV1),
    Contracts(ContractOwnerRepositoryErrorV1),
    Dock(handbook_dock::DockLocalEvaluationErrorV1),
}
impl std::fmt::Display for LocalEvaluationSdkErrorV1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Engine(e) => write!(f, "{}: {}", e.code, e.detail),
            Self::Contracts(e) => e.fmt(f),
            Self::Dock(e) => e.fmt(f),
        }
    }
}
impl std::error::Error for LocalEvaluationSdkErrorV1 {}
impl From<LocalAuthorityRefusalV1> for LocalEvaluationSdkErrorV1 {
    fn from(e: LocalAuthorityRefusalV1) -> Self {
        Self::Engine(e)
    }
}
impl From<ContractOwnerRepositoryErrorV1> for LocalEvaluationSdkErrorV1 {
    fn from(e: ContractOwnerRepositoryErrorV1) -> Self {
        Self::Contracts(e)
    }
}
impl From<handbook_dock::DockLocalEvaluationErrorV1> for LocalEvaluationSdkErrorV1 {
    fn from(e: handbook_dock::DockLocalEvaluationErrorV1) -> Self {
        Self::Dock(e)
    }
}

impl HandbookSdkV1 {
    pub fn prepare_local_evaluation_definition(
        &self,
        initial: &LocalInitialSetupLocatorV1,
        check: &LocalCheckLocatorV1,
        definition: LocalContractDefinitionV3,
        stack: &ContextResolutionStackDefinition,
    ) -> Result<PreparedLocalDefinitionAppendV3, LocalEvaluationSdkErrorV1> {
        initial
            .canonical_json_bytes()
            .map_err(|e| engine_error(&e.code, &e.detail))?;
        if check.predecessor != initial.terms {
            return Err(engine_error(
                "project_binding",
                "definition authority differs from initial setup",
            )
            .into());
        }
        let current = self.observe_local_check_basis(initial)?;
        let request = ContractOwnerRepositoryV1::local_definition_authority_request_v3(
            &definition,
            check.terms.agent.clone(),
            check.terms.author_participant_ref.clone(),
            SemanticTimeV1::owner_observation_now_v1()
                .map_err(|_| engine_error("time", "owner time unavailable"))?,
        );
        let owner = NativeLocalCheckAuthorityV1::with_admission(
            &self.repo_root,
            check,
            &current,
            &request,
            |admission| {
                Ok(ContractOwnerRepositoryV1::open_for_local_definition_v3(
                    &self.repo_root,
                    admission,
                ))
            },
        )??;
        // Resolve governance only after the Engine initializer guard is released.
        Ok(owner.prepare_local_definition_append_v3(&project(initial), definition, stack)?)
    }

    pub fn append_local_evaluation_definition(
        &self,
        locator: &LocalCheckLocatorV1,
        prepared: PreparedLocalDefinitionAppendV3,
        time: SemanticTimeV1,
        key: &str,
    ) -> Result<LocalOwnerMutationReceiptV3, LocalEvaluationSdkErrorV1> {
        let owner = ContractOwnerRepositoryV1::open_existing_for_local_use_v3(&self.repo_root)?;
        let request = prepared.authority_request(
            locator.terms.agent.clone(),
            locator.terms.author_participant_ref.clone(),
            time,
        );
        let basis = prepared.current_basis().clone();
        Ok(NativeLocalCheckAuthorityV1::with_admission(
            &self.repo_root,
            locator,
            &basis,
            &request,
            |admission| Ok(owner.append_local_definition_v3(prepared, admission, key)),
        )??)
    }

    pub fn read_local_evaluation_definition(
        &self,
        binding: &ExactBindingV1,
    ) -> Result<LocalDefinitionRecordV3, LocalEvaluationSdkErrorV1> {
        Ok(
            ContractOwnerRepositoryV1::open_existing_for_local_use_v3(&self.repo_root)?
                .read_local_definition_v3(binding)?,
        )
    }

    pub fn current_local_evaluation_lifecycle(
        &self,
        binding: &ExactBindingV1,
    ) -> Result<LocalLifecycleHeadV3, LocalEvaluationSdkErrorV1> {
        Ok(
            ContractOwnerRepositoryV1::open_existing_for_local_use_v3(&self.repo_root)?
                .current_local_lifecycle_head_v3(binding)?,
        )
    }

    pub fn prepare_local_evaluation_lifecycle(
        &self,
        initial: &LocalInitialSetupLocatorV1,
        binding: &ExactBindingV1,
        input: LocalLifecycleTransitionInputV3,
    ) -> Result<PreparedLocalLifecycleTransitionV3, LocalEvaluationSdkErrorV1> {
        initial
            .canonical_json_bytes()
            .map_err(|e| engine_error(&e.code, &e.detail))?;
        Ok(
            ContractOwnerRepositoryV1::open_existing_for_local_use_v3(&self.repo_root)?
                .prepare_local_lifecycle_transition_v3(&project(initial), binding, input)?,
        )
    }

    pub fn transition_local_evaluation_lifecycle(
        &self,
        locator: &LocalCheckLocatorV1,
        prepared: PreparedLocalLifecycleTransitionV3,
        actor: String,
        participant: String,
        time: SemanticTimeV1,
        key: &str,
    ) -> Result<LocalOwnerMutationReceiptV3, LocalEvaluationSdkErrorV1> {
        let owner = ContractOwnerRepositoryV1::open_existing_for_local_use_v3(&self.repo_root)?;
        let request = prepared.authority_request(actor, participant, time)?;
        let basis = prepared.current_basis().clone();
        Ok(NativeLocalCheckAuthorityV1::with_admission(
            &self.repo_root,
            locator,
            &basis,
            &request,
            |admission| Ok(owner.transition_local_lifecycle_v3(prepared, admission, key)),
        )??)
    }

    pub fn prepare_local_evaluation_use(
        &self,
        initial: &LocalInitialSetupLocatorV1,
        binding: &ExactBindingV1,
        intent: handbook_contracts::LocalContractUseIntentV2,
    ) -> Result<LocalContractUseV3, LocalEvaluationSdkErrorV1> {
        initial
            .canonical_json_bytes()
            .map_err(|e| engine_error(&e.code, &e.detail))?;
        Ok(
            ContractOwnerRepositoryV1::open_existing_for_local_use_v3(&self.repo_root)?
                .prepare_local_contract_use_v3(&project(initial), binding, intent)?,
        )
    }

    pub fn prepare_local_evaluation_terms(
        &self,
        predecessor: &LocalCheckLocatorV1,
        terms: LocalEvaluationTermsV1,
        basis: crate::local_check::LocalCheckCurrentBasisV1,
    ) -> Result<PreparedLocalEvaluationAuthorityV1, LocalAuthorityRefusalV1> {
        NativeLocalEvaluationAuthorityV1::prepare(&self.repo_root, predecessor, terms, basis)
    }

    pub fn adopt_local_evaluation_terms(
        &self,
        prepared: PreparedLocalEvaluationAuthorityV1,
    ) -> LocalEvaluationAuthorityExecutionV1 {
        NativeLocalEvaluationAuthorityV1::adopt(&self.repo_root, prepared)
    }

    pub fn issue_local_evaluation_authorization(
        &self,
        authority: &LocalEvaluationAuthorityLocatorV1,
        request: &LocalEvaluationAuthorizationRequestV1,
    ) -> LocalEvaluationAuthorizationExecutionV1 {
        NativeLocalEvaluationAuthorityV1::issue(&self.repo_root, authority, request)
    }

    pub fn local_evaluation_authorization_locator(
        &self,
        authority: &LocalEvaluationAuthorityLocatorV1,
        request: &LocalEvaluationAuthorizationRequestV1,
    ) -> Result<LocalEvaluationBindingLocatorV1, LocalAuthorityRefusalV1> {
        NativeLocalEvaluationAuthorityV1::issuance_locator(authority, request)
    }

    /// Resolves the original retained issuance; it does not issue a replacement.
    pub fn recover_local_evaluation_authorization(
        &self,
        authority: &LocalEvaluationAuthorityLocatorV1,
        original: &LocalEvaluationBindingLocatorV1,
        reach: &LocalEvaluationPathReachV1,
    ) -> LocalEvaluationAuthorizationExecutionV1 {
        NativeLocalEvaluationAuthorityV1::recover_issuance(
            &self.repo_root,
            authority,
            original,
            reach,
        )
    }

    /// Finishes only an original retained pending issuance under current scope.
    /// An absent intent cannot be created from the portable locator.
    pub fn resume_local_evaluation_authorization(
        &self,
        authority: &LocalEvaluationAuthorityLocatorV1,
        original: &LocalEvaluationBindingLocatorV1,
        reach: &LocalEvaluationPathReachV1,
    ) -> LocalEvaluationAuthorizationExecutionV1 {
        NativeLocalEvaluationAuthorityV1::resume_issuance_request(
            &self.repo_root,
            authority,
            original,
            reach,
        )
    }

    pub fn revoke_local_evaluation_terms(
        &self,
        authority: &LocalEvaluationAuthorityLocatorV1,
        request: &handbook_engine::local_authority::LocalRevokeRequestV1,
        reach: &LocalEvaluationPathReachV1,
    ) -> Result<(), LocalAuthorityRefusalV1> {
        NativeLocalEvaluationAuthorityV1::revoke(&self.repo_root, authority, request, reach)
    }

    /// Reads immutable issuance history without evaluating or refreshing it.
    pub fn read_local_evaluation_authorization_history(
        &self,
        authority: &LocalEvaluationAuthorityLocatorV1,
        original: &LocalEvaluationBindingLocatorV1,
        reach: &LocalEvaluationPathReachV1,
    ) -> LocalEvaluationHistoricalObservationV1 {
        NativeLocalEvaluationAuthorityV1::check_history(&self.repo_root, authority, original, reach)
    }

    pub fn recover_local_evaluation_terms(
        &self,
        original: &LocalEvaluationAuthorityLocatorV1,
    ) -> LocalEvaluationAuthorityExecutionV1 {
        NativeLocalEvaluationAuthorityV1::resume_request(&self.repo_root, original)
    }

    pub fn read_local_evaluation_terms_history(
        &self,
        original: &LocalEvaluationAuthorityLocatorV1,
    ) -> LocalEvaluationAuthorityExecutionV1 {
        NativeLocalEvaluationAuthorityV1::resolve_request(&self.repo_root, original)
    }

    pub fn prepare_local_evaluation(
        &self,
        initial: &LocalInitialSetupLocatorV1,
        authority: &LocalEvaluationAuthorityLocatorV1,
        input: LocalEvaluationPrepareInputV3,
    ) -> Result<PreparedLocalEvaluationV3, LocalEvaluationSdkErrorV1> {
        require_initial(initial, authority)?;
        let basis = self.observe_local_check_basis(initial)?;
        let formation = input.formation.clone();
        NativeLocalEvaluationAuthorityV1::with_terms_admission(
            &self.repo_root,
            authority,
            &basis,
            &formation,
            |admission| {
                admission.require_owner_write_subtree(".handbook/state/contracts")?;
                admission.require_input_path(".handbook/state/contracts")?;
                admission.require_input_path(".handbook/state/locks/contract-owner.lock")?;
                Ok(())
            },
        )?;
        // This preparation grants nothing. Resolve fresh governance after the
        // Engine preflight guard is released; formation rechecks the exact basis.
        Ok(
            ContractOwnerRepositoryV1::open_existing_for_local_use_v3(&self.repo_root)?
                .prepare_local_evaluation_v3(&project(initial), input)?,
        )
    }

    pub fn form_local_evaluation(
        &self,
        authority: &LocalEvaluationAuthorityLocatorV1,
        prepared: PreparedLocalEvaluationV3,
    ) -> Result<LocalEvaluationSnapshotV3, LocalEvaluationSdkErrorV1> {
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
                        .and_then(|owner| owner.form_local_evaluation_v3(prepared, admission)),
                )
            },
        )??)
    }

    /// Initializes only a genuinely absent Dock namespace under live terms.
    /// Existing missing or substituted locks remain an owner refusal.
    pub fn observe_local_evaluation_capability(
        &self,
        initial: &LocalInitialSetupLocatorV1,
        authority: &LocalEvaluationAuthorityLocatorV1,
        formation: &LocalEvaluationFormationRequestV1,
        reach: &DockLocalEvaluationReachV1,
    ) -> Result<NativeLocalEvaluationCapabilityV3, LocalEvaluationSdkErrorV1> {
        require_initial(initial, authority)?;
        let basis = self.observe_local_check_basis(initial)?;
        let until = SemanticTimeV1::parse(&formation.retention_until_utc)
            .map_err(|_| engine_error("retention", "invalid supplied horizon"))?;
        Ok(NativeLocalEvaluationAuthorityV1::with_terms_admission(
            &self.repo_root,
            authority,
            &basis,
            formation,
            |admission| {
                Ok(handbook_dock::DockLocalEvaluationOwnerV1::initialize(
                    &self.repo_root,
                    admission,
                    reach,
                )
                .and_then(|owner| owner.observe_capability(admission, reach, until)))
            },
        )??)
    }

    pub fn assess_local_evaluation(
        &self,
        session: &LocalEvaluationSessionV1<'_>,
        capability: &NativeLocalEvaluationCapabilityV3,
        dock_reach: &DockLocalEvaluationReachV1,
    ) -> Result<LocalEvaluationSnapshotV3, LocalEvaluationSdkErrorV1> {
        let snapshot = self.recover_local_evaluation(session.evaluation)?;
        self.with_evaluation_checkpoint(
            session,
            LocalEvaluationCheckpointV1::Preparation,
            None,
            |admission| {
                let owner =
                    ContractOwnerRepositoryV1::open_existing_for_local_use_v3(&self.repo_root)?;
                let dock =
                    handbook_dock::DockLocalEvaluationOwnerV1::open_existing(&self.repo_root)?;
                Ok(dock.with_capability(
                    capability,
                    snapshot.definition.dependencies(),
                    dock_reach,
                    |resolved| {
                        owner.assess_local_evaluation_v3(session.evaluation, admission, resolved)
                    },
                )??)
            },
        )
    }

    pub fn commit_local_evaluation_collection(
        &self,
        session: &LocalEvaluationSessionV1<'_>,
    ) -> Result<LocalEvaluationSnapshotV3, LocalEvaluationSdkErrorV1> {
        let target = ContractOwnerRepositoryV1::open_existing_for_local_use_v3(&self.repo_root)?
            .prospective_local_evaluation_basis_v3(session.evaluation)?;
        self.with_evaluation_checkpoint(
            session,
            LocalEvaluationCheckpointV1::CollectionCommit,
            Some(target.fingerprint().as_str()),
            |admission| {
                Ok(
                    ContractOwnerRepositoryV1::open_existing_for_local_use_v3(&self.repo_root)?
                        .commit_local_evaluation_v3(session.evaluation, admission)?,
                )
            },
        )
    }

    /// Contracts records the original attempt locator before Dock reserves or
    /// launches. An unresolved retained attempt never becomes a new launch.
    pub fn execute_local_evaluation_once(
        &self,
        session: &LocalEvaluationSessionV1<'_>,
        request: &handbook_dock::DockProcessRequestV1,
        dock_reach: &DockLocalEvaluationReachV1,
        cancellation: &handbook_dock::DockCancellationV1,
    ) -> Result<DockLocalEvaluationAttemptOutcomeV1, LocalEvaluationSdkErrorV1> {
        let snapshot = self.recover_local_evaluation(session.evaluation)?;
        let coordinates = coordinates(&snapshot)?;
        let run = snapshot
            .run
            .canonical_json_bytes()
            .map_err(|_| engine_error("run", "invalid retained full run"))?;
        let until = SemanticTimeV1::parse(&snapshot.purpose.retention_until_utc)
            .map_err(|_| engine_error("retention", "invalid retained horizon"))?;
        let dock = handbook_dock::DockLocalEvaluationOwnerV1::open_existing(&self.repo_root)?;
        let original =
            dock.prepare_attempt_locator(&coordinates, request, &run, dock_reach, until.clone())?;
        self.with_evaluation_checkpoint(
            session,
            LocalEvaluationCheckpointV1::Launch,
            Some(coordinates.basis.fingerprint().as_str()),
            |admission| {
                ContractOwnerRepositoryV1::open_existing_for_local_use_v3(&self.repo_root)?
                    .bind_local_evaluation_attempt_v3(session.evaluation, admission, original)?;
                Ok(dock.execute_once(
                    admission,
                    &coordinates,
                    request,
                    &run,
                    dock_reach,
                    until,
                    cancellation,
                )?)
            },
        )
    }

    pub fn admit_local_evaluation_origin(
        &self,
        session: &LocalEvaluationSessionV1<'_>,
        dock_reach: &DockLocalEvaluationReachV1,
        operation_key: &str,
    ) -> Result<LocalEvaluationSnapshotV3, LocalEvaluationSdkErrorV1> {
        let snapshot = self.recover_local_evaluation(session.evaluation)?;
        let coordinates = coordinates(&snapshot)?;
        let attempt = snapshot
            .attempt
            .as_ref()
            .ok_or_else(|| engine_error("attempt", "original attempt was not committed"))?;
        self.with_evaluation_checkpoint(
            session,
            LocalEvaluationCheckpointV1::EvidenceAppend,
            Some(coordinates.basis.fingerprint().as_str()),
            |admission| {
                let owner =
                    ContractOwnerRepositoryV1::open_existing_for_local_use_v3(&self.repo_root)?;
                let dock =
                    handbook_dock::DockLocalEvaluationOwnerV1::open_existing(&self.repo_root)?;
                Ok(dock.with_origin(
                    attempt,
                    &coordinates,
                    &snapshot.run,
                    snapshot.definition.dependencies(),
                    dock_reach,
                    |origin| {
                        owner.admit_local_evaluation_origin_v3(
                            session.evaluation,
                            admission,
                            origin,
                            operation_key,
                        )
                    },
                )??)
            },
        )
    }

    pub fn seal_local_evaluation_frontier(
        &self,
        session: &LocalEvaluationSessionV1<'_>,
    ) -> Result<LocalEvaluationSnapshotV3, LocalEvaluationSdkErrorV1> {
        let snapshot = self.recover_local_evaluation(session.evaluation)?;
        let basis = snapshot
            .basis()
            .ok_or_else(|| engine_error("basis", "collection is not committed"))?;
        self.with_evaluation_checkpoint(
            session,
            LocalEvaluationCheckpointV1::SourceAdmission,
            Some(basis.fingerprint().as_str()),
            |admission| {
                Ok(
                    ContractOwnerRepositoryV1::open_existing_for_local_use_v3(&self.repo_root)?
                        .seal_local_evaluation_frontier_v3(session.evaluation, admission)?,
                )
            },
        )
    }

    pub fn close_local_evaluation_set(
        &self,
        session: &LocalEvaluationSessionV1<'_>,
    ) -> Result<LocalEvaluationSnapshotV3, LocalEvaluationSdkErrorV1> {
        let snapshot = self.recover_local_evaluation(session.evaluation)?;
        let basis = snapshot
            .basis()
            .ok_or_else(|| engine_error("basis", "collection is not committed"))?;
        self.with_evaluation_checkpoint(
            session,
            LocalEvaluationCheckpointV1::SourceAdmission,
            Some(basis.fingerprint().as_str()),
            |admission| {
                Ok(
                    ContractOwnerRepositoryV1::open_existing_for_local_use_v3(&self.repo_root)?
                        .close_local_evaluation_set_v3(session.evaluation, admission)?,
                )
            },
        )
    }

    /// Engine pins the fixed set and Dock re-resolves the original native chain;
    /// both remain pinned through Contracts' final comparison and commit.
    pub fn close_local_evaluation_result(
        &self,
        session: &LocalEvaluationSessionV1<'_>,
    ) -> Result<LocalEvaluationSnapshotV3, LocalEvaluationSdkErrorV1> {
        require_initial(session.initial, session.authority)?;
        let owner = ContractOwnerRepositoryV1::open_existing_for_local_use_v3(&self.repo_root)?;
        let retained = owner.recover_local_evaluation_v3(session.evaluation)?;
        if retained.stage == LocalEvaluationStageV3::ResultClosed {
            let expected_original = self.local_evaluation_authorization_locator(
                session.authority,
                session.authorization_request,
            )?;
            if retained.issuance_locator.as_ref() != Some(session.original_issuance)
                || expected_original != *session.original_issuance
            {
                return Err(engine_error(
                    "result_replay_identity",
                    "closed result belongs to another original authorization request",
                )
                .into());
            }
            // Delivery of an already retained result grants no current use.
            return Ok(retained);
        }
        let current = self.observe_local_check_basis(session.initial)?;
        let source = owner.local_evaluation_final_set_source_v3(session.evaluation)?;
        let coordinates = coordinates(&retained)?;
        let attempt = retained
            .attempt
            .as_ref()
            .ok_or_else(|| engine_error("attempt", "original attempt was not committed"))?;
        // Engine also admits glob selectors that Dock does not interpret.
        // Retain only the original selectors valid for Dock; filtering narrows
        // reach and never turns a glob into a wider native subtree permission.
        let mut dock_read_paths = session
            .authorization_request
            .reach
            .input_paths
            .iter()
            .filter(|path| {
                DockLocalEvaluationReachV1 {
                    read_paths: vec![(*path).clone()],
                    write_subtrees: vec![],
                }
                .validate()
                .is_ok()
            })
            .cloned()
            .collect::<Vec<_>>();
        dock_read_paths.sort();
        dock_read_paths.dedup();
        let dock_reach = DockLocalEvaluationReachV1 {
            read_paths: dock_read_paths,
            write_subtrees: vec![],
        };
        Ok(NativeLocalEvaluationAuthorityV1::with_final_verification(
            &self.repo_root,
            session.authority,
            session.original_issuance,
            session.authorization_request,
            &source,
            &current,
            &session.authorization_request.reach,
            |admission| {
                let closed: Result<_, LocalEvaluationSdkErrorV1> = (|| {
                    let dock =
                        handbook_dock::DockLocalEvaluationOwnerV1::open_existing(&self.repo_root)?;
                    Ok(dock.with_origin(
                        attempt,
                        &coordinates,
                        &retained.run,
                        retained.definition.dependencies(),
                        &dock_reach,
                        |origin| {
                            owner.close_local_evaluation_result_v3(
                                session.evaluation,
                                admission,
                                origin,
                            )
                        },
                    )??)
                })();
                Ok(closed)
            },
        )??)
    }

    /// Exact retained recovery, including an uncertain stage, never executes.
    pub fn recover_local_evaluation(
        &self,
        original: &LocalEvaluationLocatorV3,
    ) -> Result<LocalEvaluationSnapshotV3, LocalEvaluationSdkErrorV1> {
        Ok(
            // Open the retained root and original lock without rejecting the
            // pending transaction that this exact recovery must resolve.
            ContractOwnerRepositoryV1::open_existing_for_local_evaluation_history_v3(
                &self.repo_root,
            )?
            .recover_local_evaluation_v3(original)?,
        )
    }

    /// Uses the request locator saved before allocation, including when the
    /// caller never received the allocated occurrence locator.
    pub fn recover_local_evaluation_request(
        &self,
        original: &LocalEvaluationRequestLocatorV3,
    ) -> Result<Option<LocalEvaluationSnapshotV3>, LocalEvaluationSdkErrorV1> {
        Ok(
            ContractOwnerRepositoryV1::open_existing_for_local_evaluation_history_v3(
                &self.repo_root,
            )?
            .recover_local_evaluation_request_v3(original)?,
        )
    }

    /// Returns the original closed result with dependency availability separately.
    pub fn read_local_evaluation_history(
        &self,
        original: &LocalEvaluationLocatorV3,
    ) -> Result<LocalEvaluationHistoryV3, LocalEvaluationSdkErrorV1> {
        Ok(
            ContractOwnerRepositoryV1::open_existing_for_local_evaluation_history_v3(
                &self.repo_root,
            )?
            .read_local_evaluation_history_v3(original)?,
        )
    }

    /// Identifies the original immutable result, without selecting a later head.
    pub fn read_local_evaluation_result_history(
        &self,
        original: &LocalEvaluationResultLocatorV3,
    ) -> Result<LocalEvaluationHistoryV3, LocalEvaluationSdkErrorV1> {
        Ok(
            ContractOwnerRepositoryV1::open_existing_for_local_evaluation_history_v3(
                &self.repo_root,
            )?
            .read_local_evaluation_result_history_v3(original)?,
        )
    }

    pub fn recover_local_evaluation_attempt(
        &self,
        original: &handbook_contracts::ContentAddressedBindingV1,
        reach: &DockLocalEvaluationReachV1,
    ) -> Result<DockLocalEvaluationAttemptOutcomeV1, LocalEvaluationSdkErrorV1> {
        Ok(
            handbook_dock::DockLocalEvaluationOwnerV1::open_existing(&self.repo_root)?
                .resolve_exact_attempt(original, reach, true)?,
        )
    }

    fn with_evaluation_checkpoint<T>(
        &self,
        session: &LocalEvaluationSessionV1<'_>,
        checkpoint: LocalEvaluationCheckpointV1,
        target: Option<&str>,
        consume: impl FnOnce(
            &handbook_engine::local_authority::local_evaluation::LocalEvaluationAdmissionV1<'_>,
        ) -> Result<T, LocalEvaluationSdkErrorV1>,
    ) -> Result<T, LocalEvaluationSdkErrorV1> {
        require_initial(session.initial, session.authority)?;
        let current = self.observe_local_check_basis(session.initial)?;
        Ok(NativeLocalEvaluationAuthorityV1::with_verification(
            &self.repo_root,
            session.authority,
            session.original_issuance,
            session.authorization_request,
            checkpoint,
            target,
            &current,
            &session.authorization_request.reach,
            |admission| Ok(consume(admission)),
        )??)
    }
}

fn project(initial: &LocalInitialSetupLocatorV1) -> ProjectAdmissionBindingV3 {
    ProjectAdmissionBindingV3::ScopedChoiceV2(initial.deferral.project.clone())
}
fn engine_error(code: &str, detail: &str) -> LocalAuthorityRefusalV1 {
    LocalAuthorityRefusalV1 {
        code: code.into(),
        detail: detail.into(),
        unresolved_request: None,
    }
}
fn require_initial(
    initial: &LocalInitialSetupLocatorV1,
    authority: &LocalEvaluationAuthorityLocatorV1,
) -> Result<(), LocalAuthorityRefusalV1> {
    if authority.predecessor.predecessor != initial.terms {
        return Err(engine_error(
            "project_binding",
            "evaluation authority differs from original initial setup",
        ));
    }
    Ok(())
}
fn coordinates(
    snapshot: &LocalEvaluationSnapshotV3,
) -> Result<handbook_dock::DockLocalEvaluationCoordinatesV1, LocalEvaluationSdkErrorV1> {
    Ok(handbook_dock::DockLocalEvaluationCoordinatesV1 {
        occurrence: snapshot.occurrence().clone(),
        basis: snapshot
            .basis()
            .ok_or_else(|| engine_error("basis", "one committed collection basis required"))?
            .clone(),
        selection: snapshot.selection().clone(),
        authorization: snapshot
            .authorization_binding()
            .map_err(|_| engine_error("authorization", "retained issuance absent"))?,
        capability_observation: snapshot
            .readiness_bases
            .last()
            .ok_or_else(|| engine_error("readiness", "no fixed readiness observation"))?
            .capability
            .binding()
            .map_err(|_| engine_error("capability", "invalid fixed capability observation"))?,
    })
}
