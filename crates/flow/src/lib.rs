#![forbid(unsafe_code)]

pub mod budget;
pub mod grounded_packet;
pub mod packet_result;
pub mod resolver;

pub use budget::{
    evaluate_budget, evaluate_budget_with_effective_bytes, BudgetByteDomain, BudgetDisposition,
    BudgetEffectiveBytes, BudgetOutcome, BudgetPolicy, BudgetReason, BudgetTarget, NextSafeAction,
};
pub use grounded_packet::{
    adopt_grounding_outcome, GroundedPacket, GroundedPacketOutcome, GroundedPacketStatus,
    RefusedGroundedPacket,
};
pub use packet_result::{
    PacketBodyNote, PacketBodyNoteKind, PacketDecisionSummary, PacketFixtureContext, PacketResult,
    PacketSection, PacketSectionMode, PacketSourceSummary, PacketVariant,
    ReadyPacketNextSafeAction,
};
pub use resolver::{
    resolve, resolve_with_contract, PacketSelection, PacketSelectionStatus, ResolveRequest,
    ResolverBlocker, ResolverBlockerCategory, ResolverNextSafeAction, ResolverRefusal,
    ResolverRefusalCategory, ResolverResult, ResolverSubjectRef, C04_RESULT_VERSION,
};

pub fn flow_contract_version() -> &'static str {
    handbook_engine::workspace_contract_version()
}
