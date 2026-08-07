mod declarative_roots;
mod layout;
pub mod pipeline;
pub mod pipeline_capture;
pub mod pipeline_compile;
pub mod pipeline_handoff;
pub mod pipeline_route;
mod repo_file_access;
pub mod route_state;
pub mod shared_grounding_inclusion;
mod stage_10_feature_spec_provenance;

pub use declarative_roots::PipelineDeclarativeRootsContract;
pub use layout::PipelineStorageLayoutContract;
pub use shared_grounding_inclusion::{
    include_grounded_shared_resolution, GroundedSharedResolutionInclusion,
    GROUNDED_SHARED_RESOLUTION_NAMESPACE,
};

pub fn pipeline_contract_version() -> &'static str {
    handbook_engine::workspace_contract_version()
}
