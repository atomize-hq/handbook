//! Route-specific Generate and Inspect SDK operations.
//!
//! Flow remains the semantic owner of resolution. The SDK projects its result
//! into closed values, while the CLI remains responsible for presentation and
//! exit policy.

pub use crate::resolver::{
    ArtifactPresence, BudgetByteDomain, BudgetDisposition, BudgetNextSafeAction, BudgetOutcome,
    BudgetReason, BudgetTarget, PacketBodyNote, PacketBodyNoteKind, PacketDecisionSummary,
    PacketFixtureContext, PacketResult, PacketSection, PacketSectionMode, PacketSelection,
    PacketSelectionStatus, PacketSourceSummary, PacketVariant, ReadyPacketNextSafeAction,
    ResolverResult as PacketResolutionView, C04_RESULT_VERSION,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GeneratePacketRequest {
    packet_id: String,
}

impl GeneratePacketRequest {
    pub fn new(packet_id: impl Into<String>) -> Self {
        Self {
            packet_id: packet_id.into(),
        }
    }

    pub(crate) fn into_inner(self) -> crate::resolver::PacketResolveRequest {
        crate::resolver::PacketResolveRequest::new(self.packet_id)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InspectPacketRequest {
    packet_id: String,
}

impl InspectPacketRequest {
    pub fn new(packet_id: impl Into<String>) -> Self {
        Self {
            packet_id: packet_id.into(),
        }
    }

    pub(crate) fn into_inner(self) -> crate::resolver::PacketResolveRequest {
        crate::resolver::PacketResolveRequest::new(self.packet_id)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GeneratePacketResult {
    resolution: PacketResolutionView,
}

impl GeneratePacketResult {
    pub fn resolution(&self) -> &PacketResolutionView {
        &self.resolution
    }

    pub fn into_resolution(self) -> PacketResolutionView {
        self.resolution
    }

    pub(crate) fn new(resolution: PacketResolutionView) -> Self {
        Self { resolution }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InspectPacketResult {
    resolution: PacketResolutionView,
}

impl InspectPacketResult {
    pub fn resolution(&self) -> &PacketResolutionView {
        &self.resolution
    }

    pub fn into_resolution(self) -> PacketResolutionView {
        self.resolution
    }

    pub(crate) fn new(resolution: PacketResolutionView) -> Self {
        Self { resolution }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FlowRouteError {
    detail: String,
}

impl FlowRouteError {
    pub(crate) fn from_compiler(error: crate::CompilerError) -> Self {
        Self {
            detail: format!("{error:?}"),
        }
    }
}

impl std::fmt::Display for FlowRouteError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.detail)
    }
}

impl std::error::Error for FlowRouteError {}
