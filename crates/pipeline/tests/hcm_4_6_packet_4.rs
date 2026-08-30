use handbook_flow::AdoptedSessionGroundingV1;
use handbook_pipeline::{
    include_session_grounding, SessionGroundingPipelineInclusionV1,
    SESSION_GROUNDING_PIPELINE_NAMESPACE_V1,
};

#[test]
fn session_grounding_pipeline_includes_only_the_typed_flow_adoption() {
    let _: fn(AdoptedSessionGroundingV1) -> SessionGroundingPipelineInclusionV1 =
        include_session_grounding;
    assert_eq!(
        SESSION_GROUNDING_PIPELINE_NAMESPACE_V1,
        "handbook.session-grounding-boundary@1.0.0"
    );
}
