use crate::{NextSafeAction, SubjectRef};

/// Closed blocker categories returned by the Generate and Inspect operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockerCategory {
    SystemRootMissing,
    SystemRootNotDir,
    SystemRootSymlinkNotAllowed,
    RequiredArtifactMissing,
    RequiredArtifactEmpty,
    RequiredArtifactStarterTemplate,
    RequiredArtifactInvalid,
    ArtifactReadError,
    FreshnessInvalid,
    BudgetRefused,
    UnsupportedRequest,
}

/// A closed Flow blocker projection. Rendering remains a CLI concern.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Blocker {
    pub category: BlockerCategory,
    pub subject: SubjectRef,
    pub summary: String,
    pub next_safe_action: NextSafeAction,
}
