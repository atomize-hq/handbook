//! Direct typed Rust SDK for Handbook.
//!
//! This initial surface deliberately contains only posture-transition ingress.
//! It has no runtime operation catalog, discovery API, JSON DTO, schema, or
//! transport support; callers link it directly from Rust.

use std::path::Path;

pub use handbook_engine::{
    PostureCanonicalBindingV1, PostureDimensionChangeRequestV1, PostureReferenceV1,
    PostureTransitionApplyRequestV1, PostureTransitionApplyResultV1,
    PostureTransitionBlockedCodeV1, PostureTransitionErrorCodeV1, PostureTransitionReceiptV1,
    PostureTransitionRefusalCodeV1,
};

/// Rust-linkage-only SDK entry point for the initial posture ingress.
#[derive(Clone, Debug)]
pub struct HandbookSdkV1 {
    posture: handbook_engine::PostureTransitionEngineFacadeV1,
}

impl HandbookSdkV1 {
    /// Opens an SDK handle for one repository without mutating it.
    pub fn open(repo_root: impl AsRef<Path>) -> Self {
        Self {
            posture: handbook_engine::PostureTransitionEngineFacadeV1::open(repo_root),
        }
    }

    /// Applies a bounded posture transition through the public engine facade.
    pub fn apply_posture_transition(
        &self,
        request: PostureTransitionApplyRequestV1,
    ) -> PostureTransitionApplyResultV1 {
        self.posture.apply_posture_transition(request)
    }
}
