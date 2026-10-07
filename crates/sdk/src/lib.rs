//! Transport-free Rust SDK for Handbook semantic owners.
//!
//! The SDK composes owner crates for ordinary use cases. CLI parsing, rendering,
//! streams, and exit codes remain outside this crate.

use std::path::{Path, PathBuf};

pub mod admission_governance;
pub mod artifact;
mod author;
mod author_route_api;
mod blocker;
mod charter_product;
pub mod contract_membrane;
mod decision_log;
mod doctor;
mod error;
pub mod flow_api;
pub mod local_check;
pub mod local_evaluation;
pub mod local_setup;
pub mod pipeline_route_api;
mod profile_readiness;
mod refusal;
#[cfg(unix)]
mod repo_file_access;
mod resolver;
mod route_state;
mod setup;

pub use author_route_api::{
    ApproverCommandRequest, ApproverMapping, ApproverOperation, ApproverOperationProjection,
    ApproverOperationResult, AuthorCausalDisposition, AuthorOperationRefusal,
    AuthorOperationStatus, AuthorProjectContextRefusal, AuthorProjectContextRefusalKind,
    AuthorProjectContextResult, AuthorPublicEffect, AuthorPublicEffectDisposition,
    CharterAcquisitionMode, CharterCommandRequest, CharterCoverageSubmission, CharterInputValue,
    CharterIntakeConsumer, CharterIntakeEnvelope, CharterIntakeSourceKind, CharterOperation,
    CharterOperationResult, ProjectContextCommandOutcome, ProjectContextCommandRequest,
    ProjectContextInput, RepositoryInvocationFailureView,
};
pub use blocker::{Blocker, BlockerCategory};
pub use decision_log::DecisionLog;
pub use doctor::{
    DoctorCharterDefinitionClosureStatus, DoctorCharterDefinitionRow, DoctorCharterLifecycleState,
    DoctorCharterNextAction, DoctorCharterRow, DoctorError, DoctorErrorKind, DoctorErrorReasonCode,
    DoctorProjectContextRow, DoctorReport,
};
use error::CompilerError;
pub use profile_readiness::{
    ArtifactApplicability, ArtifactInspectionReason, ArtifactInspectionStatus, ProfileArtifactRow,
    ProfileCapabilityRow, ProfileConditionOutcome, ProfileConditionReason, ProfileConditionRow,
    ProfileRequirednessMode, RepositoryReadinessStatus,
};
pub use refusal::{NextSafeAction, Refusal, RefusalCategory, SubjectRef};
pub use resolver::{
    ArtifactPresence, BudgetByteDomain, BudgetDisposition, BudgetNextSafeAction, BudgetOutcome,
    BudgetReason, BudgetTarget, PacketBodyNote, PacketBodyNoteKind, PacketDecisionSummary,
    PacketFixtureContext, PacketResult, PacketSection, PacketSectionMode, PacketSelection,
    PacketSelectionStatus, PacketSourceSummary, PacketVariant, ReadyPacketNextSafeAction,
};
pub use setup::{
    SetupArtifactAction, SetupArtifactActionKind, SetupError, SetupErrorCode, SetupErrorKind,
    SetupErrorReasonCode, SetupMode, SetupOutcome, SetupPlan, SetupRequest, SetupRootAction,
};

pub use handbook_engine::{
    PostureCanonicalBindingV1, PostureDimensionChangeRequestV1, PostureReferenceV1,
    PostureTransitionApplyRequestV1, PostureTransitionApplyResultV1,
    PostureTransitionBlockedCodeV1, PostureTransitionErrorCodeV1, PostureTransitionReceiptV1,
    PostureTransitionRefusalCodeV1,
};

/// Repository-bound transport-free SDK coordinator.
#[derive(Clone, Debug)]
pub struct HandbookSdkV1 {
    repo_root: PathBuf,
    posture: handbook_engine::PostureTransitionEngineFacadeV1,
}

impl HandbookSdkV1 {
    /// Opens an SDK handle for one repository without mutating it.
    pub fn open(repo_root: impl AsRef<Path>) -> Self {
        let repo_root = repo_root.as_ref().to_path_buf();
        Self {
            posture: handbook_engine::PostureTransitionEngineFacadeV1::open(&repo_root),
            repo_root,
        }
    }

    /// Applies a bounded posture transition through the public engine facade.
    pub fn apply_posture_transition(
        &self,
        request: PostureTransitionApplyRequestV1,
    ) -> PostureTransitionApplyResultV1 {
        self.posture.apply_posture_transition(request)
    }

    /// Applies the exact public Packet-4 operation envelope through the same
    /// engine-owned posture transition authority as the direct typed method.
    pub fn apply_posture_transition_operation(
        &self,
        request: &contract_membrane::OperationRequestV1,
    ) -> Result<contract_membrane::OperationResponseV1, contract_membrane::ContractError> {
        contract_membrane::apply_posture_transition_operation(&self.posture, request)
    }

    /// Runs the existing repository setup composition for this SDK repository.
    pub fn run_setup(&self, request: &SetupRequest) -> Result<SetupOutcome, SetupError> {
        setup::run_setup(&self.repo_root, request)
    }

    /// Runs repository setup once and closes its owner result into the public use-case response.
    ///
    /// Setup writes use the frozen Packet-0 projection bound, so limits other
    /// than [`contract_membrane::ContractLimits::packet0`] are rejected before
    /// setup planning or mutation.
    pub fn run_setup_use_case(
        &self,
        request: &SetupRequest,
        limits: &contract_membrane::ContractLimits,
    ) -> Result<contract_membrane::UseCaseResponseV1, contract_membrane::ContractError> {
        if limits != &contract_membrane::ContractLimits::packet0() {
            return Err(contract_membrane::ContractError::LimitOutOfRange);
        }
        let execution = match setup::prepare_setup_execution(&self.repo_root, request) {
            Ok(plan) => execute_preflighted_setup(&self.repo_root, request, plan, limits)?,
            Err(error) => setup::SetupExecutionResult::new(
                Err(error),
                setup::SetupExecutionEffects::default(),
            ),
        };
        contract_membrane::UseCaseResponseV1::repository_setup_execution(&execution, limits)
    }

    /// Runs the existing repository doctor composition for this SDK repository.
    pub fn doctor(&self) -> Result<DoctorReport, DoctorError> {
        doctor::doctor(&self.repo_root)
    }

    /// Resolves one packet through the SDK-owned ordinary Flow composition.
    pub fn generate_packet(
        &self,
        request: flow_api::GeneratePacketRequest,
    ) -> Result<flow_api::GeneratePacketResult, flow_api::FlowRouteError> {
        resolver::resolve_packet(&self.repo_root, request.into_inner())
            .map(flow_api::GeneratePacketResult::new)
            .map_err(flow_api::FlowRouteError::from_compiler)
    }

    /// Resolves one packet for the Inspect route through the bounded Flow composition.
    pub fn inspect_packet(
        &self,
        request: flow_api::InspectPacketRequest,
    ) -> Result<flow_api::InspectPacketResult, flow_api::FlowRouteError> {
        resolver::resolve_packet(&self.repo_root, request.into_inner())
            .map(flow_api::InspectPacketResult::new)
            .map_err(flow_api::FlowRouteError::from_compiler)
    }

    /// Executes the exact Flow resolve ordinary operation once.
    pub fn execute_flow_operation(
        &self,
        request: &contract_membrane::OperationRequestV1,
    ) -> Result<contract_membrane::OperationResponseV1, contract_membrane::ContractError> {
        contract_membrane::execute_flow_operation(&self.repo_root, request)
    }

    /// Executes one exact Pipeline ordinary operation once.
    pub fn execute_pipeline_operation(
        &self,
        request: &contract_membrane::OperationRequestV1,
    ) -> Result<contract_membrane::OperationResponseV1, contract_membrane::ContractError> {
        contract_membrane::execute_pipeline_operation(&self.repo_root, request)
    }

    /// Executes one exact Contracts ordinary operation once.
    pub fn execute_contract_operation(
        &self,
        request: &contract_membrane::OperationRequestV1,
    ) -> Result<contract_membrane::OperationResponseV1, contract_membrane::ContractError> {
        contract_membrane::execute_contract_operation(&self.repo_root, request)
    }

    /// Executes one exact evidence/verdict/gate ordinary operation once.
    pub fn execute_contract_evidence_operation(
        &self,
        request: &contract_membrane::OperationRequestV1,
    ) -> Result<contract_membrane::OperationResponseV1, contract_membrane::ContractError> {
        contract_membrane::execute_contract_evidence_operation(&self.repo_root, request)
    }

    /// Executes one exact Dock manifest or run ordinary operation once.
    pub fn execute_dock_operation(
        &self,
        request: &contract_membrane::OperationRequestV1,
    ) -> Result<contract_membrane::OperationResponseV1, contract_membrane::ContractError> {
        contract_membrane::execute_dock_operation(&self.repo_root, request)
    }

    /// Executes `dock.run` with owner-resolved runtime capabilities after
    /// Contracts run preparation and independent Dock re-admission.
    pub fn execute_dock_operation_with_runtime(
        &self,
        request: &contract_membrane::OperationRequestV1,
        context: contract_membrane::DockRunExecutionContextV1<'_>,
    ) -> Result<contract_membrane::OperationResponseV1, contract_membrane::ContractError> {
        contract_membrane::execute_dock_operation_with_runtime(&self.repo_root, request, context)
    }

    /// Executes an evidence append or owner-derived evaluation with explicit runtime facts.
    #[allow(clippy::too_many_arguments)]
    pub fn execute_contract_evidence_operation_with_context<S, C>(
        &self,
        request: &contract_membrane::OperationRequestV1,
        source: &mut S,
        source_currentness: &mut C,
        resolution_stack: &handbook_engine::ContextResolutionStackDefinition,
        policy_registry: &handbook_contracts::EvaluationPolicyRegistryV1,
    ) -> Result<contract_membrane::OperationResponseV1, contract_membrane::ContractError>
    where
        S: handbook_contracts::EvidenceCandidateSourceV1,
        C: handbook_engine::SourceCurrentnessPortV1,
    {
        contract_membrane::execute_contract_evidence_operation_with_context(
            &self.repo_root,
            request,
            source,
            source_currentness,
            resolution_stack,
            policy_registry,
        )
    }

    /// Executes one exact Contracts mutation with an injected native authenticator.
    pub fn execute_contract_operation_with_authenticator<P>(
        &self,
        request: &contract_membrane::OperationRequestV1,
        authenticator: &mut P,
    ) -> Result<contract_membrane::OperationResponseV1, contract_membrane::ContractError>
    where
        P: handbook_engine::NativeAuthenticatorPortV1,
    {
        contract_membrane::execute_contract_operation_with_authenticator(
            &self.repo_root,
            request,
            authenticator,
        )
    }

    /// Dispatches one admitted read-only ordinary invocation through its existing owner route.
    ///
    /// HCM-4.4 Packet 3 default-denies mutations and use cases as typed transport failures.
    pub fn invoke(
        &self,
        request: &contract_membrane::InvocationRequestV1,
        limits: &contract_membrane::ContractLimits,
    ) -> Result<contract_membrane::InvocationResponseV1, contract_membrane::ContractError> {
        contract_membrane::dispatch_invocation(&self.repo_root, request, limits)
    }

    /// Dispatches one Rust SDK invocation with a trusted snapshot-source port.
    pub fn invoke_with_snapshot_source(
        &self,
        request: &contract_membrane::InvocationRequestV1,
        limits: &contract_membrane::ContractLimits,
        source_port: &mut dyn handbook_engine::SnapshotSourcePortV1,
    ) -> Result<contract_membrane::InvocationResponseV1, contract_membrane::ContractError> {
        contract_membrane::dispatch_invocation_with_snapshot_source(
            &self.repo_root,
            request,
            limits,
            source_port,
        )
    }

    /// Dispatches one Tauri invocation with fixed transport-owned presentation facts.
    ///
    /// The Tauri presentation value never enters the request contract or ordinary capability
    /// evidence and cannot be selected by the caller.
    pub fn invoke_tauri(
        &self,
        request: &contract_membrane::InvocationRequestV1,
        limits: &contract_membrane::ContractLimits,
    ) -> Result<contract_membrane::InvocationResponseV1, contract_membrane::ContractError> {
        contract_membrane::dispatch_tauri_invocation(&self.repo_root, request, limits)
    }

    /// Dispatches the same generic Tauri invocation with a trusted snapshot-source port.
    pub fn invoke_tauri_with_snapshot_source(
        &self,
        request: &contract_membrane::InvocationRequestV1,
        limits: &contract_membrane::ContractLimits,
        source_port: &mut dyn handbook_engine::SnapshotSourcePortV1,
    ) -> Result<contract_membrane::InvocationResponseV1, contract_membrane::ContractError> {
        contract_membrane::dispatch_tauri_invocation_with_snapshot_source(
            &self.repo_root,
            request,
            limits,
            source_port,
        )
    }

    /// Re-admits one LF-free invocation response against its exact originating request.
    pub fn readmit_invocation_response(
        &self,
        request: &contract_membrane::InvocationRequestV1,
        response: &contract_membrane::InvocationResponseV1,
        limits: &contract_membrane::ContractLimits,
    ) -> Result<contract_membrane::InvocationResponseV1, contract_membrane::ContractError> {
        contract_membrane::readmit_invocation_response(request, response, limits)
    }

    /// Resolves and persists one Pipeline route exactly once through the SDK composition.
    pub fn resolve_pipeline_use_case(
        &self,
        selector: &str,
        limits: &contract_membrane::ContractLimits,
    ) -> Result<contract_membrane::UseCaseResponseV1, contract_membrane::ContractError> {
        if limits != &contract_membrane::ContractLimits::packet0() {
            return Err(contract_membrane::ContractError::LimitOutOfRange);
        }
        let execution = pipeline_route_api::resolve_pipeline_execution(
            &pipeline_route_api::PipelineResolveRequest {
                repository_root: self.repo_root.clone(),
                selector: selector.to_owned(),
            },
        );
        contract_membrane::UseCaseResponseV1::pipeline_route_resolve_and_persist_execution(
            &execution, limits,
        )
    }

    /// Captures one explicit session boundary once and composes its committed owner result.
    pub fn run_session_grounding_boundary_use_case<P: handbook_engine::SnapshotSourcePortV1>(
        &self,
        request: &contract_membrane::SessionGroundingBoundaryRequestV1,
        source_port: &mut P,
        limits: &contract_membrane::ContractLimits,
    ) -> Result<contract_membrane::UseCaseResponseV1, contract_membrane::ContractError> {
        if limits != &contract_membrane::ContractLimits::packet0() {
            return Err(contract_membrane::ContractError::LimitOutOfRange);
        }
        let execution = contract_membrane::execute_session_grounding_boundary(
            &self.repo_root,
            request,
            source_port,
        )?;
        contract_membrane::UseCaseResponseV1::session_grounding_boundary_execution(
            execution, limits,
        )
    }

    fn run_session_grounding_boundary_use_case_dyn(
        &self,
        request: &contract_membrane::SessionGroundingBoundaryRequestV1,
        source_port: &mut dyn handbook_engine::SnapshotSourcePortV1,
        limits: &contract_membrane::ContractLimits,
    ) -> Result<contract_membrane::UseCaseResponseV1, contract_membrane::ContractError> {
        if limits != &contract_membrane::ContractLimits::packet0() {
            return Err(contract_membrane::ContractError::LimitOutOfRange);
        }
        let execution = contract_membrane::execute_session_grounding_boundary_dyn(
            &self.repo_root,
            request,
            source_port,
        )?;
        contract_membrane::UseCaseResponseV1::session_grounding_boundary_execution(
            execution, limits,
        )
    }

    /// Recomposes one already-committed owner result without invoking Engine capture.
    pub fn recompose_session_grounding_boundary_use_case(
        &self,
        committed: handbook_engine::SnapshotCaptureOutcomeV1,
        limits: &contract_membrane::ContractLimits,
    ) -> Result<contract_membrane::UseCaseResponseV1, contract_membrane::ContractError> {
        if limits != &contract_membrane::ContractLimits::packet0() {
            return Err(contract_membrane::ContractError::LimitOutOfRange);
        }
        let execution = contract_membrane::recompose_session_grounding_boundary(committed)?;
        contract_membrane::UseCaseResponseV1::session_grounding_boundary_execution(
            execution, limits,
        )
    }
}

fn execute_preflighted_setup(
    repo_root: &Path,
    request: &SetupRequest,
    plan: setup::SetupExecutionPlan,
    limits: &contract_membrane::ContractLimits,
) -> Result<setup::SetupExecutionResult, contract_membrane::ContractError> {
    let prospective = setup::prospective_setup_execution(&plan);
    let observed_bytes = contract_membrane::measure_repository_setup_invocation(&prospective)?;
    if observed_bytes <= limits.document_bytes() {
        return Ok(setup::apply_setup_execution(repo_root, plan));
    }
    if !request.reset_state {
        return Err(contract_membrane::ContractError::BoundExceeded);
    }
    Ok(setup::SetupExecutionResult::new(
        Err(setup::SetupError::runtime_state_reset_projection_too_large(
            observed_bytes,
        )),
        setup::SetupExecutionEffects::default(),
    ))
}

pub fn workspace_contract_version() -> &'static str {
    handbook_engine::workspace_contract_version()
}

#[cfg(test)]
mod setup_projection_tests {
    use super::*;
    use crate::contract_membrane::{canonical_json_bytes, InvocationResponseV1};
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn oversized_prospective_reset_projection_refuses_before_mutation() {
        let repo = tempdir().unwrap();
        fs::create_dir_all(repo.path().join(".handbook/state")).unwrap();
        let sentinel = repo.path().join(".handbook/state/sentinel.json");
        fs::write(&sentinel, b"{}\n").unwrap();
        let request = SetupRequest {
            mode: SetupMode::Refresh,
            rewrite: false,
            reset_state: true,
        };
        let limits = contract_membrane::ContractLimits::packet0();
        let mut plan = setup::prepare_setup_execution(repo.path(), &request).unwrap();
        let reset_paths = (0..128)
            .map(|index| {
                let prefix = format!(".handbook/state/{index:03}/");
                format!("{prefix}{}", "x".repeat(4096 - prefix.len()))
            })
            .collect::<Vec<_>>();
        setup::replace_reset_paths_for_projection_test(&mut plan, reset_paths);
        let observed_bytes = contract_membrane::measure_repository_setup_invocation(
            &setup::prospective_setup_execution(&plan),
        )
        .unwrap();
        assert!(observed_bytes > limits.document_bytes());

        let execution = execute_preflighted_setup(repo.path(), &request, plan, &limits).unwrap();

        let error = execution.result().as_ref().unwrap_err();
        assert_eq!(
            error.code(),
            SetupErrorCode::RuntimeStateResetProjectionTooLarge
        );
        assert_eq!(error.observed_bytes(), Some(observed_bytes));
        assert!(sentinel.exists());
        assert!(!repo
            .path()
            .join(handbook_engine::REPOSITORY_IDENTITY_REPO_PATH)
            .exists());
    }

    #[test]
    fn realized_in_bound_setup_outer_length_matches_the_preflight_candidate() {
        let repo = tempdir().unwrap();
        let request = SetupRequest::default();
        let limits = contract_membrane::ContractLimits::packet0();
        let plan = setup::prepare_setup_execution(repo.path(), &request).unwrap();
        let prospective_bytes = contract_membrane::measure_repository_setup_invocation(
            &setup::prospective_setup_execution(&plan),
        )
        .unwrap();

        let execution = execute_preflighted_setup(repo.path(), &request, plan, &limits).unwrap();
        assert!(execution.effects().root_created());
        assert!(execution.effects().repository_identity_created().is_some());
        let use_case =
            contract_membrane::UseCaseResponseV1::repository_setup_execution(&execution, &limits)
                .unwrap();
        let invocation = InvocationResponseV1::use_case(use_case, &limits).unwrap();
        assert_eq!(
            canonical_json_bytes(&invocation, &limits).unwrap().len(),
            prospective_bytes
        );
    }
}
