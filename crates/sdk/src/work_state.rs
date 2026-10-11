//! Typed native work definitions and registration. Locators identify original
//! requests and grant no operating permission. Retain them before invoking effects.
use crate::local_check::LocalCheckLocatorV1;
use crate::local_setup::LocalInitialSetupLocatorV1;
use crate::HandbookSdkV1;
pub use handbook_engine::local_authority::LocalAuthorityRefusalV1;
use handbook_engine::work_state::NativeWorkStateV1;
pub use handbook_engine::work_state::{
    work_item_identity_v1, PreparedWorkGrantV1, PreparedWorkKindV1, PreparedWorkRegistrationV1,
    WorkAuthorityChangeV1, WorkCompletionSupportV1, WorkCompositionClauseV1,
    WorkCompositionInputV1, WorkCompositionV1, WorkCurrentItemV1, WorkCurrentKindV1,
    WorkDefinitionFamilyV1, WorkDescriptorV1, WorkDispositionV1, WorkExecutionV1,
    WorkExplicitScopeV1, WorkGrantLocatorV1, WorkGrantV1, WorkKindApprovalV1, WorkKindBindingV1,
    WorkKindDefinitionV1, WorkKindLocatorV1, WorkKindProvenanceV1, WorkKindReceiptV1,
    WorkRegistrationInputV1, WorkRegistrationReceiptV1, WorkRegistrationScopeV1,
    WorkRegistrationTermsV1, WorkRequestLocatorV1, WorkRequirementsDefinitionV1,
    WorkRevocationReceiptV1, WorkRevokeLocatorV1, WorkRolePermissionV1, WorkRoleValueV1,
    WorkSnapshotV1, WorkTermsTransitionV1, ORDINARY_LEAF_COMPOSITION_V1,
    PLANNING_OUTPUT_COMPOSITION_V1, REQUIREMENTS_INPUT_COMPOSITION_V1, WORK_EXPLICIT_SCOPE_V1,
    WORK_IDENTITY_SCHEME_V1, WORK_KIND_BINDING_SCHEMA_V1, WORK_KIND_DEFINITION_SCHEMA_V1,
    WORK_KIND_ESTABLISHMENT_RULE_V1, WORK_REGISTRATION_RULE_V1, WORK_STATE_FULL_OWNER_REACH_V1,
};

impl HandbookSdkV1 {
    /// Prepares the original human's approval of one exact immutable definition.
    /// Supply the observed native basis and check fingerprint in that approval;
    /// they are validated inward, never replaced by this observation. Definition
    /// establishment needs no work grant and supplies no work permission.
    pub fn prepare_work_kind_establishment(
        &self,
        initial: &LocalInitialSetupLocatorV1,
        check: &LocalCheckLocatorV1,
        definition: WorkKindDefinitionV1,
        approval: WorkKindApprovalV1,
    ) -> Result<PreparedWorkKindV1, LocalAuthorityRefusalV1> {
        require_initial(initial, check)?;
        let basis = self.observe_local_check_basis(initial)?;
        NativeWorkStateV1::prepare_kind(&self.repo_root, check, basis, definition, approval)
    }

    /// Save `prepared.locator()` first. A candidate is never a committed kind;
    /// inspect the returned disposition, outcome and refusal before using it.
    pub fn establish_work_kind(
        &self,
        prepared: PreparedWorkKindV1,
    ) -> WorkExecutionV1<WorkKindReceiptV1> {
        NativeWorkStateV1::establish_kind(&self.repo_root, prepared)
    }

    /// Reads original definition-establishment truth without live permission.
    pub fn resolve_work_kind_request(
        &self,
        locator: &WorkKindLocatorV1,
    ) -> WorkExecutionV1<WorkKindReceiptV1> {
        NativeWorkStateV1::resolve_kind_request(&self.repo_root, locator)
    }

    /// Recovers only the saved original basis, approval and version reservation.
    /// A successor basis or fresh request key never repairs uncertain history.
    pub fn recover_work_kind_request(
        &self,
        locator: &WorkKindLocatorV1,
    ) -> WorkExecutionV1<WorkKindReceiptV1> {
        NativeWorkStateV1::recover_kind_request(&self.repo_root, locator)
    }

    /// Resolves an exact same-project definition independently of work permission.
    /// Pending definitions cannot be used. Missing proof refuses rather than
    /// substituting another version or inferring absence.
    pub fn resolve_work_kind(
        &self,
        binding: &WorkKindBindingV1,
    ) -> Result<WorkCurrentKindV1, LocalAuthorityRefusalV1> {
        NativeWorkStateV1::current_kind(&self.repo_root, binding)
    }

    /// Prepares exact supplied human-approved terms under the fixed Engine
    /// establishment rule. Existing local-check terms alone supply no work grant.
    /// Select initial establishment, an exact same-basis scope successor, or
    /// unchanged-scope D to G re-establishment. Each requires supplied approval.
    pub fn prepare_work_registration_terms(
        &self,
        initial: &LocalInitialSetupLocatorV1,
        check: &LocalCheckLocatorV1,
        terms: WorkRegistrationTermsV1,
        transition: WorkTermsTransitionV1<'_>,
    ) -> Result<PreparedWorkGrantV1, LocalAuthorityRefusalV1> {
        require_initial(initial, check)?;
        let basis = self.observe_local_check_basis(initial)?;
        NativeWorkStateV1::prepare_terms(&self.repo_root, check, basis, terms, transition)
    }

    /// Retain `prepared.locator()` before invocation. Activation's native
    /// terminal establishes the grant; a pending candidate supplies no permission.
    pub fn activate_work_registration_terms(
        &self,
        prepared: PreparedWorkGrantV1,
    ) -> WorkExecutionV1<WorkGrantV1> {
        NativeWorkStateV1::activate_terms(&self.repo_root, prepared)
    }

    /// Resolves original activation truth without restoring current permission.
    pub fn resolve_work_registration_terms_request(
        &self,
        locator: &WorkGrantLocatorV1,
    ) -> WorkExecutionV1<WorkGrantV1> {
        NativeWorkStateV1::resolve_terms_request(&self.repo_root, locator)
    }

    /// Uses the original establishment basis; never substitutes current terms.
    pub fn recover_work_registration_terms_request(
        &self,
        locator: &WorkGrantLocatorV1,
    ) -> WorkExecutionV1<WorkGrantV1> {
        NativeWorkStateV1::recover_terms_request(&self.repo_root, locator)
    }

    /// Resolves permission against fresh supported Contracts applicability
    /// observed outside the native consuming locks and validated inward.
    pub fn resolve_current_work_registration_terms(
        &self,
        initial: &LocalInitialSetupLocatorV1,
        locator: &WorkGrantLocatorV1,
    ) -> Result<WorkGrantV1, LocalAuthorityRefusalV1> {
        require_initial(initial, &locator.check)?;
        let basis = self.observe_local_check_basis(initial)?;
        NativeWorkStateV1::resolve_current_terms(&self.repo_root, locator, &basis)
    }

    /// Binds one same-project absent to open leaf and its original recovery locator.
    /// Explicit input uses supplied values only; kind input verifies its exact
    /// retained committed definition. Preparation reserves nothing and promises
    /// no current grant usability, item absence, capacity or eventual commit.
    /// Save the complete portable `locator()` before invocation.
    pub fn prepare_work_registration(
        &self,
        initial: &LocalInitialSetupLocatorV1,
        grant: &WorkGrantLocatorV1,
        input: WorkRegistrationInputV1,
    ) -> Result<PreparedWorkRegistrationV1, LocalAuthorityRefusalV1> {
        require_initial(initial, &grant.check)?;
        NativeWorkStateV1::prepare_registration(&self.repo_root, grant, &grant.basis, input)
    }

    /// Checks current authority, conflicts and capacity and invokes the owner.
    /// An exact committed original replays even after its grant becomes unusable.
    /// Inspect disposition, outcome and refusal; an uncertain response requires
    /// original-key recovery, never a blind new key.
    pub fn register_work_item(
        &self,
        prepared: PreparedWorkRegistrationV1,
    ) -> WorkExecutionV1<WorkRegistrationReceiptV1> {
        NativeWorkStateV1::register(&self.repo_root, prepared)
    }

    /// Returns original registration history under retained owner dependencies,
    /// including after revocation, governance change or runtime-state reset.
    pub fn resolve_work_registration_request(
        &self,
        locator: &WorkRequestLocatorV1,
    ) -> WorkExecutionV1<WorkRegistrationReceiptV1> {
        NativeWorkStateV1::resolve_request(&self.repo_root, locator)
    }

    /// Recovers only this original request and authority. Native proof can return
    /// committed/replayed, pending, noncommit or uncertainty without guessing.
    pub fn recover_work_registration_request(
        &self,
        locator: &WorkRequestLocatorV1,
    ) -> WorkExecutionV1<WorkRegistrationReceiptV1> {
        NativeWorkStateV1::recover_request(&self.repo_root, locator)
    }

    /// Reads current immutable item truth, independently of live permission.
    /// Snapshot bytes without their exact terminal are pending, never open.
    pub fn resolve_current_work_item(
        &self,
        grant: &WorkGrantLocatorV1,
        item_key: &str,
    ) -> Result<WorkCurrentItemV1, LocalAuthorityRefusalV1> {
        NativeWorkStateV1::current_item(&self.repo_root, grant, item_key)
    }

    /// Prepares revocation by the exact declared principal. Retain the returned
    /// locator before invoking revocation; it includes the original grant.
    pub fn prepare_work_registration_revocation(
        &self,
        grant: &WorkGrantLocatorV1,
        request_key: &str,
        principal: &str,
        source_ref: &str,
        instruction: &str,
    ) -> Result<WorkRevokeLocatorV1, LocalAuthorityRefusalV1> {
        NativeWorkStateV1::prepare_revocation(
            &self.repo_root,
            grant,
            request_key,
            principal,
            source_ref,
            instruction,
        )
    }

    pub fn revoke_work_registration_terms(
        &self,
        locator: &WorkRevokeLocatorV1,
    ) -> WorkExecutionV1<WorkRevocationReceiptV1> {
        NativeWorkStateV1::revoke_terms(&self.repo_root, locator)
    }

    pub fn resolve_work_registration_revocation_request(
        &self,
        locator: &WorkRevokeLocatorV1,
    ) -> WorkExecutionV1<WorkRevocationReceiptV1> {
        NativeWorkStateV1::resolve_revocation_request(&self.repo_root, locator)
    }

    pub fn recover_work_registration_revocation_request(
        &self,
        locator: &WorkRevokeLocatorV1,
    ) -> WorkExecutionV1<WorkRevocationReceiptV1> {
        NativeWorkStateV1::recover_revocation_request(&self.repo_root, locator)
    }

    /// Inspects the resolved item's unsupported/not-assessed completion boundary.
    /// It forms no eligibility, disposition, Determination or closing effect.
    pub fn inspect_work_completion_support(
        &self,
        locator: &WorkRequestLocatorV1,
    ) -> Result<WorkCompletionSupportV1, LocalAuthorityRefusalV1> {
        NativeWorkStateV1::completion_support(&self.repo_root, locator)
    }
}

/// Copies one exact intrinsic Contracts definition. This declaration does not
/// resolve source availability, qualification, governance or assessment.
pub fn work_requirements_definition_from_intrinsic_v1(
    definition: &handbook_contracts::ContractDefinitionV1,
) -> WorkRequirementsDefinitionV1 {
    let family = WorkDefinitionFamilyV1::IntrinsicV1;
    WorkRequirementsDefinitionV1 {
        owner: "handbook-contracts".into(),
        family,
        schema_ref: family.schema_ref().into(),
        reference: definition.contract_ref().as_str().into(),
        fingerprint: definition.definition_fingerprint().as_str().into(),
    }
}

/// Copies the complete local V2 definition binding, including its dependencies.
/// The embedded intrinsic fingerprint is not the complete definition identity.
pub fn work_requirements_definition_from_local_v2(
    definition: &handbook_contracts::LocalContractDefinitionV2,
) -> WorkRequirementsDefinitionV1 {
    requirements_definition_from_binding(
        WorkDefinitionFamilyV1::CompleteLocalV2,
        definition.binding(),
    )
}

/// Copies the full local V3 definition binding, including evaluation dependencies.
pub fn work_requirements_definition_from_full_local_v3(
    definition: &handbook_contracts::LocalContractDefinitionV3,
) -> WorkRequirementsDefinitionV1 {
    requirements_definition_from_binding(WorkDefinitionFamilyV1::FullLocalV3, definition.binding())
}

fn requirements_definition_from_binding(
    family: WorkDefinitionFamilyV1,
    binding: handbook_contracts::ExactBindingV1,
) -> WorkRequirementsDefinitionV1 {
    WorkRequirementsDefinitionV1 {
        owner: "handbook-contracts".into(),
        family,
        schema_ref: family.schema_ref().into(),
        reference: binding.reference().as_str().into(),
        fingerprint: binding.fingerprint().as_str().into(),
    }
}

fn require_initial(
    initial: &LocalInitialSetupLocatorV1,
    check: &LocalCheckLocatorV1,
) -> Result<(), LocalAuthorityRefusalV1> {
    initial
        .canonical_json_bytes()
        .map_err(|error| LocalAuthorityRefusalV1 {
            code: error.code,
            detail: error.detail,
            unresolved_request: None,
        })?;
    if check.predecessor != initial.terms {
        return Err(LocalAuthorityRefusalV1 {
            code: "project_binding".into(),
            detail: "work authority and initial setup differ".into(),
            unresolved_request: None,
        });
    }
    Ok(())
}
