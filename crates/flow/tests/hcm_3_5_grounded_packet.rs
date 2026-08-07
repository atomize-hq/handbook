use handbook_engine::grounding::GroundingOutcome;
use handbook_flow::{adopt_grounding_outcome, GroundedPacketOutcome};

#[test]
fn grounded_packet_path_accepts_only_the_typed_engine_outcome() {
    let _: fn(GroundingOutcome) -> GroundedPacketOutcome = adopt_grounding_outcome;
}
