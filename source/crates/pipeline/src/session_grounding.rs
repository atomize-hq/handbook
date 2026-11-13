use handbook_flow::{AdoptedSessionGroundingV1, SessionGroundingFlowStatusV1};

/// Fixed namespace for one session-boundary grounding inclusion.
pub const SESSION_GROUNDING_PIPELINE_NAMESPACE_V1: &str =
    "handbook.session-grounding-boundary@1.0.0";

/// Pipeline inclusion of one already-adopted Flow result.
#[derive(Clone, Debug)]
pub struct SessionGroundingPipelineInclusionV1 {
    namespace: &'static str,
    adoption: AdoptedSessionGroundingV1,
}

impl SessionGroundingPipelineInclusionV1 {
    /// Returns the fixed non-semantic inclusion namespace.
    pub fn namespace(&self) -> &'static str {
        self.namespace
    }

    /// Preserves the exact Flow terminal selected from the owner result.
    pub fn status(&self) -> SessionGroundingFlowStatusV1 {
        self.adoption.status()
    }

    /// Borrows the exact Flow adoption.
    pub fn adoption(&self) -> &AdoptedSessionGroundingV1 {
        &self.adoption
    }

    /// Returns the exact Flow adoption for pure downstream recovery.
    pub fn into_adoption(self) -> AdoptedSessionGroundingV1 {
        self.adoption
    }
}

/// Includes one typed Flow adoption without semantic recomputation.
pub fn include_session_grounding(
    adoption: AdoptedSessionGroundingV1,
) -> SessionGroundingPipelineInclusionV1 {
    SessionGroundingPipelineInclusionV1 {
        namespace: SESSION_GROUNDING_PIPELINE_NAMESPACE_V1,
        adoption,
    }
}
