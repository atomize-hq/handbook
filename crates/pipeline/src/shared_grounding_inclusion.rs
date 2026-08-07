use handbook_engine::grounding::SharedResolutionInclusion;

pub const GROUNDED_SHARED_RESOLUTION_NAMESPACE: &str = "handbook.hcm-3-5.shared-resolution";

#[derive(Clone, Debug)]
pub struct GroundedSharedResolutionInclusion {
    namespace: &'static str,
    inclusion: SharedResolutionInclusion,
}

impl GroundedSharedResolutionInclusion {
    pub fn namespace(&self) -> &'static str {
        self.namespace
    }

    pub fn inclusion(&self) -> &SharedResolutionInclusion {
        &self.inclusion
    }
}

pub fn include_grounded_shared_resolution(
    inclusion: SharedResolutionInclusion,
) -> GroundedSharedResolutionInclusion {
    GroundedSharedResolutionInclusion {
        namespace: GROUNDED_SHARED_RESOLUTION_NAMESPACE,
        inclusion,
    }
}
