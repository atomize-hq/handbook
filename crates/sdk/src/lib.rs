//! Transport-free Rust SDK for Handbook semantic owners.
//!
//! The SDK composes owner crates for ordinary use cases. CLI parsing, rendering,
//! streams, and exit codes remain outside this crate.

use std::path::{Path, PathBuf};

pub mod artifact;
mod author;
mod author_route_api;
mod blocker;
mod charter_product;
mod decision_log;
mod doctor;
mod error;
pub mod flow_api;
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
    ApproverOperationResult, AuthorOperationRefusal, AuthorOperationStatus,
    AuthorProjectContextRefusal, AuthorProjectContextRefusalKind, AuthorProjectContextResult,
    CharterAcquisitionMode, CharterCommandRequest, CharterCoverageSubmission, CharterInputValue,
    CharterIntakeConsumer, CharterIntakeEnvelope, CharterIntakeSourceKind, CharterOperation,
    CharterOperationResult, ProjectContextInput, RepositoryInvocationFailureView,
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

    /// Runs the existing repository setup composition for this SDK repository.
    pub fn run_setup(&self, request: &SetupRequest) -> Result<SetupOutcome, SetupError> {
        setup::run_setup(&self.repo_root, request)
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
}

pub fn workspace_contract_version() -> &'static str {
    handbook_engine::workspace_contract_version()
}
