mod declarative_roots;
mod layout;
pub mod operation_admission;
pub mod pipeline;
pub mod pipeline_capture;
pub mod pipeline_compile;
pub mod pipeline_handoff;
mod pipeline_mutation_admission;
pub mod pipeline_mutation_effect;
pub mod pipeline_mutation_journal;
mod pipeline_read_admission;
pub mod pipeline_route;
mod repo_file_access;
pub mod route_state;
mod session_grounding;
pub mod shared_grounding_inclusion;
mod stage_10_feature_spec_provenance;

pub use declarative_roots::PipelineDeclarativeRootsContract;
pub use layout::PipelineStorageLayoutContract;
pub use session_grounding::{
    include_session_grounding, SessionGroundingPipelineInclusionV1,
    SESSION_GROUNDING_PIPELINE_NAMESPACE_V1,
};
pub use shared_grounding_inclusion::{
    include_grounded_shared_resolution, GroundedSharedResolutionInclusion,
    GROUNDED_SHARED_RESOLUTION_NAMESPACE,
};

pub fn pipeline_contract_version() -> &'static str {
    handbook_engine::workspace_contract_version()
}
