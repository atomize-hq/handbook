use handbook_engine::SnapshotCaptureOutcomeV1;
use handbook_flow::{
    adopt_session_grounding_outcome, AdoptedSessionGroundingV1, SessionGroundingFlowStatusV1,
};

#[test]
fn session_grounding_flow_adopts_only_the_typed_committed_owner_outcome() {
    let _: fn(SnapshotCaptureOutcomeV1) -> AdoptedSessionGroundingV1 =
        adopt_session_grounding_outcome;
    let _ = SessionGroundingFlowStatusV1::Started;
    let _ = SessionGroundingFlowStatusV1::Finalized;
}
