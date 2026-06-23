mod declarative_roots;
mod layout;
pub mod pipeline;
pub mod pipeline_capture;
pub mod pipeline_compile;
pub mod pipeline_handoff;
pub mod pipeline_route;
mod repo_file_access;
pub mod route_state;
mod stage_10_feature_spec_provenance;

pub use declarative_roots::PipelineDeclarativeRootsContract;
pub use layout::PipelineStorageLayoutContract;

pub fn pipeline_contract_version() -> &'static str {
    handbook_engine::workspace_contract_version()
}
