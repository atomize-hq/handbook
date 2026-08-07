use handbook_engine::grounding::SharedResolutionInclusion;
use handbook_pipeline::{
    include_grounded_shared_resolution, GroundedSharedResolutionInclusion,
    GROUNDED_SHARED_RESOLUTION_NAMESPACE,
};

#[test]
fn grounded_shared_inclusion_has_one_fixed_namespace_and_typed_input() {
    let _: fn(SharedResolutionInclusion) -> GroundedSharedResolutionInclusion =
        include_grounded_shared_resolution;
    assert_eq!(
        GROUNDED_SHARED_RESOLUTION_NAMESPACE,
        "handbook.hcm-3-5.shared-resolution"
    );
}
