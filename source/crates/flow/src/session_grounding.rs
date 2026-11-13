use handbook_engine::SnapshotCaptureOutcomeV1;

/// Exact terminal variant selected by the Engine snapshot-grounding owner.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SessionGroundingFlowStatusV1 {
    /// The owner committed a start-boundary grounding result.
    Started,
    /// The owner committed an end-boundary final grounding result.
    Finalized,
}

/// Flow adoption of one already-committed Engine owner result.
///
/// The owner result remains intact so Flow cannot substitute or recompute any
/// predecessor, delta, projection, receipt, replay, or handoff fact.
#[derive(Clone, Debug)]
pub struct AdoptedSessionGroundingV1 {
    owner_outcome: SnapshotCaptureOutcomeV1,
}

impl AdoptedSessionGroundingV1 {
    /// Returns the exact Engine terminal variant without changing it.
    pub fn status(&self) -> SessionGroundingFlowStatusV1 {
        match self.owner_outcome {
            SnapshotCaptureOutcomeV1::Started(_) => SessionGroundingFlowStatusV1::Started,
            SnapshotCaptureOutcomeV1::Finalized(_) => SessionGroundingFlowStatusV1::Finalized,
        }
    }

    /// Borrows the exact committed Engine owner result.
    pub fn owner_outcome(&self) -> &SnapshotCaptureOutcomeV1 {
        &self.owner_outcome
    }

    /// Returns the exact committed Engine owner result for downstream recovery.
    pub fn into_owner_outcome(self) -> SnapshotCaptureOutcomeV1 {
        self.owner_outcome
    }
}

/// Adopts one typed committed owner result without semantic recomputation.
pub fn adopt_session_grounding_outcome(
    owner_outcome: SnapshotCaptureOutcomeV1,
) -> AdoptedSessionGroundingV1 {
    AdoptedSessionGroundingV1 { owner_outcome }
}
