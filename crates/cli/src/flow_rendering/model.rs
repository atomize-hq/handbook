use super::error::RenderError;
use handbook_sdk::flow_api::{PacketResolutionView as ResolverResult, C04_RESULT_VERSION};
use handbook_sdk::{Blocker, Refusal};
use handbook_sdk::{BudgetOutcome, PacketResult, PacketSelectionStatus};

#[cfg(all(test, unix))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderSurface {
    Markdown,
    Json,
    Inspect,
}

#[cfg(all(test, unix))]
impl RenderSurface {
    pub const fn order(self) -> usize {
        match self {
            RenderSurface::Markdown => 0,
            RenderSurface::Json => 1,
            RenderSurface::Inspect => 2,
        }
    }
}

#[cfg(all(test, unix))]
pub const fn ordered_surfaces() -> [RenderSurface; 3] {
    [
        RenderSurface::Markdown,
        RenderSurface::Json,
        RenderSurface::Inspect,
    ]
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderOutputModel {
    pub c04_result_version: String,
    pub c03_schema_version: String,
    pub c03_manifest_generation_version: u32,
    pub c03_fingerprint_sha256: String,
    pub packet_id: String,
    pub packet_result: PacketResult,
    pub packet_status: PacketSelectionStatus,
    pub budget_outcome: BudgetOutcome,
    pub decision_log_entries: Vec<String>,
    pub refusal: Option<Refusal>,
    pub blockers: Vec<Blocker>,
}

pub fn build_output_model(result: &ResolverResult) -> Result<RenderOutputModel, RenderError> {
    if result.c04_result_version != C04_RESULT_VERSION {
        return Err(RenderError::UnsupportedResultVersion {
            expected: C04_RESULT_VERSION,
            actual: result.c04_result_version.clone(),
        });
    }

    if result.selection.packet_id.trim().is_empty() {
        return Err(RenderError::EmptyPacketId);
    }

    if result.decision_log.entries.is_empty() {
        return Err(RenderError::EmptyDecisionLog);
    }

    if result.selection.status == PacketSelectionStatus::Selected
        && result.packet_result.sections.is_empty()
    {
        return Err(RenderError::EmptyPacketBody);
    }

    Ok(RenderOutputModel {
        c04_result_version: result.c04_result_version.clone(),
        c03_schema_version: result.c03_schema_version.clone(),
        c03_manifest_generation_version: result.c03_manifest_generation_version,
        c03_fingerprint_sha256: result.c03_fingerprint_sha256.clone(),
        packet_id: result.selection.packet_id.clone(),
        packet_result: result.packet_result.clone(),
        packet_status: result.selection.status,
        budget_outcome: result.budget_outcome.clone(),
        decision_log_entries: result.decision_log.entries.clone(),
        refusal: result.refusal.clone(),
        blockers: result.blockers.clone(),
    })
}
