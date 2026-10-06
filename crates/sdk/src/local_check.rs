//! Typed local-check composition. Native owners retain authority and qualification.
use crate::admission_governance::{
    ProjectAdmissionBindingV3, ProjectAdmissionScopeBasisV3, ADMISSION_GOVERNANCE_SCOPE_V1,
};
use crate::local_setup::LocalInitialSetupLocatorV1;
use crate::HandbookSdkV1;
use handbook_contracts::{
    ContractOwnerRepositoryErrorV1, ContractOwnerRepositoryV1, ExactBindingV1, SemanticTimeV1,
};
pub use handbook_contracts::{
    LocalContractDefinitionInputV2, LocalContractDefinitionV2, LocalContractUseIntentV2,
    LocalContractUseV2, LocalDefinitionDependenciesV2, LocalDefinitionRecordV2,
    LocalLifecycleHeadV2, LocalLifecycleTransitionInputV2, LocalOwnerMutationReceiptV2,
    LocalPermittedUseV2, LocalReviewDecisionV2, LocalReviewInputV2,
    PreparedLocalDefinitionAppendV2, PreparedLocalLifecycleTransitionV2,
};
pub use handbook_dock::{DockCapturedInputCurrentnessV1, DockLocalEvidenceCandidateSourceV2};
use handbook_engine::local_authority::local_check::NativeLocalCheckAuthorityV1;
pub use handbook_engine::local_authority::local_check::{
    LocalCheckActionRequestV1, LocalCheckActionV1, LocalCheckAdmissionV1,
    LocalCheckAuthorityExecutionV1, LocalCheckAuthorityOutcomeV1, LocalCheckCurrentBasisV1,
    LocalCheckLocatorV1, LocalCheckTermsV1, PreparedLocalCheckAuthorityV1, LOCAL_CHECK_RULE_V1,
};
use handbook_engine::local_authority::{LocalAuthorityRefusalV1, LocalRevokeRequestV1};
use handbook_engine::ContextResolutionStackDefinition;

#[derive(Debug)]
pub enum LocalCheckSdkErrorV1 {
    Engine(LocalAuthorityRefusalV1),
    Contracts(ContractOwnerRepositoryErrorV1),
}
impl std::fmt::Display for LocalCheckSdkErrorV1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Engine(e) => write!(f, "{}: {}", e.code, e.detail),
            Self::Contracts(e) => e.fmt(f),
        }
    }
}
impl std::error::Error for LocalCheckSdkErrorV1 {}
impl From<LocalAuthorityRefusalV1> for LocalCheckSdkErrorV1 {
    fn from(e: LocalAuthorityRefusalV1) -> Self {
        Self::Engine(e)
    }
}
impl From<ContractOwnerRepositoryErrorV1> for LocalCheckSdkErrorV1 {
    fn from(e: ContractOwnerRepositoryErrorV1) -> Self {
        Self::Contracts(e)
    }
}

impl HandbookSdkV1 {
    /// Opens the native retained-capture source. Every resolution freshly checks
    /// owner qualification and live authority through the consuming callback.
    pub fn open_local_contract_evidence_source(
        &self,
        initial: &LocalInitialSetupLocatorV1,
        locator: &LocalCheckLocatorV1,
        definition: ExactBindingV1,
        intent: LocalContractUseIntentV2,
    ) -> Result<
        DockLocalEvidenceCandidateSourceV2,
        handbook_contracts::EvidenceCandidateSourceErrorV1,
    > {
        if locator.predecessor != initial.terms {
            return Err(
                handbook_contracts::EvidenceCandidateSourceErrorV1::repository_refusal(
                    handbook_contracts::EvidenceCandidateSourceErrorKindV1::AuthorityAmbiguous,
                ),
            );
        }
        DockLocalEvidenceCandidateSourceV2::open(
            &self.repo_root,
            locator.clone(),
            ProjectAdmissionBindingV3::ScopedChoiceV2(initial.deferral.project.clone()),
            definition,
            intent,
        )
    }
    /// Prepares only the reviewed local run basis needed to exercise Packet 2's
    /// source consumers. It supplies no Coordination or Evaluation Authorization.
    pub fn prepare_local_source_seam_run(
        &self,
        input: handbook_contracts::PrepareEvaluationRunInputV1,
    ) -> Result<handbook_contracts::ContractEvaluationRunV1, LocalCheckSdkErrorV1> {
        Ok(
            ContractOwnerRepositoryV1::open_existing_for_local_use_v2(&self.repo_root)?
                .prepare_local_source_seam_run_v2(input)?,
        )
    }
    pub fn prepare_local_contract_definition(
        &self,
        initial: &LocalInitialSetupLocatorV1,
        definition: LocalContractDefinitionV2,
        stack: &ContextResolutionStackDefinition,
    ) -> Result<PreparedLocalDefinitionAppendV2, LocalCheckSdkErrorV1> {
        initial
            .canonical_json_bytes()
            .map_err(|e| error(&e.code, &e.detail))?;
        let owner = ContractOwnerRepositoryV1::open(&self.repo_root)?;
        Ok(owner.prepare_local_definition_append_v2(
            &ProjectAdmissionBindingV3::ScopedChoiceV2(initial.deferral.project.clone()),
            definition,
            stack,
        )?)
    }
    pub fn append_local_contract_definition(
        &self,
        locator: &LocalCheckLocatorV1,
        prepared: PreparedLocalDefinitionAppendV2,
        semantic_time: SemanticTimeV1,
        key: &str,
    ) -> Result<LocalOwnerMutationReceiptV2, LocalCheckSdkErrorV1> {
        let owner = ContractOwnerRepositoryV1::open_existing_for_local_use_v2(&self.repo_root)?;
        let request = prepared.authority_request(
            locator.terms.agent.clone(),
            locator.terms.author_participant_ref.clone(),
            semantic_time,
        );
        let basis = prepared.current_basis().clone();
        Ok(NativeLocalCheckAuthorityV1::with_admission(
            &self.repo_root,
            locator,
            &basis,
            &request,
            |admission| Ok(owner.append_local_definition_v2(prepared, admission, key)),
        )??)
    }
    pub fn read_local_contract_definition(
        &self,
        binding: &ExactBindingV1,
    ) -> Result<LocalDefinitionRecordV2, LocalCheckSdkErrorV1> {
        Ok(
            ContractOwnerRepositoryV1::open_existing_for_local_use_v2(&self.repo_root)?
                .read_local_definition_v2(binding)?,
        )
    }
    pub fn current_local_contract_lifecycle(
        &self,
        binding: &ExactBindingV1,
    ) -> Result<LocalLifecycleHeadV2, LocalCheckSdkErrorV1> {
        Ok(
            ContractOwnerRepositoryV1::open_existing_for_local_use_v2(&self.repo_root)?
                .current_local_lifecycle_head_v2(binding)?,
        )
    }
    pub fn prepare_local_contract_lifecycle(
        &self,
        initial: &LocalInitialSetupLocatorV1,
        binding: &ExactBindingV1,
        input: LocalLifecycleTransitionInputV2,
    ) -> Result<PreparedLocalLifecycleTransitionV2, LocalCheckSdkErrorV1> {
        initial
            .canonical_json_bytes()
            .map_err(|e| error(&e.code, &e.detail))?;
        Ok(
            ContractOwnerRepositoryV1::open_existing_for_local_use_v2(&self.repo_root)?
                .prepare_local_lifecycle_transition_v2(
                    &ProjectAdmissionBindingV3::ScopedChoiceV2(initial.deferral.project.clone()),
                    binding,
                    input,
                )?,
        )
    }
    pub fn transition_local_contract_lifecycle(
        &self,
        locator: &LocalCheckLocatorV1,
        prepared: PreparedLocalLifecycleTransitionV2,
        actor_ref: String,
        participant_ref: String,
        semantic_time: SemanticTimeV1,
        key: &str,
    ) -> Result<LocalOwnerMutationReceiptV2, LocalCheckSdkErrorV1> {
        let owner = ContractOwnerRepositoryV1::open_existing_for_local_use_v2(&self.repo_root)?;
        let request = prepared.authority_request(actor_ref, participant_ref, semantic_time)?;
        let basis = prepared.current_basis().clone();
        Ok(NativeLocalCheckAuthorityV1::with_admission(
            &self.repo_root,
            locator,
            &basis,
            &request,
            |admission| Ok(owner.transition_local_lifecycle_v2(prepared, admission, key)),
        )??)
    }
    pub fn prepare_local_contract_use(
        &self,
        initial: &LocalInitialSetupLocatorV1,
        binding: &ExactBindingV1,
        intent: LocalContractUseIntentV2,
    ) -> Result<LocalContractUseV2, LocalCheckSdkErrorV1> {
        initial
            .canonical_json_bytes()
            .map_err(|e| error(&e.code, &e.detail))?;
        Ok(
            ContractOwnerRepositoryV1::open_existing_for_local_use_v2(&self.repo_root)?
                .prepare_local_contract_use_v2(
                    &ProjectAdmissionBindingV3::ScopedChoiceV2(initial.deferral.project.clone()),
                    binding,
                    intent,
                )?,
        )
    }
    pub fn prepare_local_check_terms(
        &self,
        initial: &LocalInitialSetupLocatorV1,
        terms: LocalCheckTermsV1,
    ) -> Result<PreparedLocalCheckAuthorityV1, LocalAuthorityRefusalV1> {
        initial
            .canonical_json_bytes()
            .map_err(|e| error(&e.code, &e.detail))?;
        let basis = self.observe_local_check_basis(initial)?;
        NativeLocalCheckAuthorityV1::prepare(&self.repo_root, &initial.terms, terms, basis)
    }
    pub fn adopt_local_check_terms(
        &self,
        prepared: PreparedLocalCheckAuthorityV1,
    ) -> LocalCheckAuthorityExecutionV1 {
        NativeLocalCheckAuthorityV1::adopt(&self.repo_root, prepared)
    }
    pub fn resolve_local_check_terms_request(
        &self,
        locator: &LocalCheckLocatorV1,
    ) -> LocalCheckAuthorityExecutionV1 {
        NativeLocalCheckAuthorityV1::resolve_request(&self.repo_root, locator)
    }
    pub fn resume_local_check_terms_request(
        &self,
        locator: &LocalCheckLocatorV1,
    ) -> LocalCheckAuthorityExecutionV1 {
        NativeLocalCheckAuthorityV1::resume_request(&self.repo_root, locator)
    }
    pub fn resolve_current_local_check_terms(
        &self,
        initial: &LocalInitialSetupLocatorV1,
        locator: &LocalCheckLocatorV1,
    ) -> Result<LocalCheckAuthorityOutcomeV1, LocalAuthorityRefusalV1> {
        if locator.predecessor != initial.terms {
            return Err(error(
                "project_binding",
                "successor and initial setup differ",
            ));
        }
        let basis = self.observe_local_check_basis(initial)?;
        NativeLocalCheckAuthorityV1::resolve_current(&self.repo_root, locator, &basis)
    }
    pub fn revoke_local_check_terms(
        &self,
        locator: &LocalCheckLocatorV1,
        request: &LocalRevokeRequestV1,
    ) -> Result<(), LocalAuthorityRefusalV1> {
        NativeLocalCheckAuthorityV1::revoke(&self.repo_root, locator, request)
    }
    /// A native byte observation alone establishes no definition qualification.
    pub fn observe_local_check_basis(
        &self,
        initial: &LocalInitialSetupLocatorV1,
    ) -> Result<LocalCheckCurrentBasisV1, LocalAuthorityRefusalV1> {
        initial
            .canonical_json_bytes()
            .map_err(|e| error(&e.code, &e.detail))?;
        let project = ProjectAdmissionBindingV3::ScopedChoiceV2(initial.deferral.project.clone());
        let before = self
            .resolve_project_admission_scope_v3(&project, ADMISSION_GOVERNANCE_SCOPE_V1, None)
            .map_err(|e| error(&e.condition, &e.detail))?;
        let bytes = match &before {
            ProjectAdmissionScopeBasisV3::DeferredV2(b) => b.canonical_json_bytes(),
            ProjectAdmissionScopeBasisV3::EstablishedFromDeferredV3(b) => b.canonical_json_bytes(),
            _ => {
                return Err(error(
                    "unsupported_project_contract",
                    "local-check terms require original defer-first project",
                ))
            }
        }
        .map_err(|e| error(&e.condition, &e.detail))?;
        let observation = NativeLocalCheckAuthorityV1::observe_basis(
            &self.repo_root,
            handbook_engine::DefinitionFingerprint::from_bytes(&bytes).to_string(),
        )?;
        let after = self
            .resolve_project_admission_scope_v3(&project, ADMISSION_GOVERNANCE_SCOPE_V1, None)
            .map_err(|e| error(&e.condition, &e.detail))?;
        if before != after {
            return Err(error(
                "governance_currentness",
                "native scope changed while observing applicability",
            ));
        }
        Ok(observation)
    }
}

fn error(code: &str, detail: &str) -> LocalAuthorityRefusalV1 {
    LocalAuthorityRefusalV1 {
        code: code.into(),
        detail: detail.into(),
        unresolved_request: None,
    }
}
