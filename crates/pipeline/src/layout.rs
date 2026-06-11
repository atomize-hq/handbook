use crate::repo_file_access::{CompilerWorkspace, NormalizedRepoRelativePath};
use std::path::Path;

pub(crate) const RUNTIME_STATE_ROOT_RELATIVE: &str = ".handbook/state";
const RUNTIME_STATE_PIPELINE_DIR_RELATIVE: &str = ".handbook/state/pipeline";
const CAPTURE_PROVENANCE_DIR_RELATIVE: &str = ".handbook/state/pipeline/stage_capture";
#[cfg_attr(not(test), allow(dead_code))]
const CAPTURE_CACHE_DIR_RELATIVE: &str = ".handbook/state/pipeline/capture";
const HANDOFF_FEATURE_SLICE_DIR_RELATIVE: &str = "artifacts/handoff/feature_slice";

#[derive(Clone, Copy, Debug)]
pub(crate) struct RepoLayoutRoot<'a> {
    workspace: CompilerWorkspace<'a>,
}

impl<'a> RepoLayoutRoot<'a> {
    pub(crate) fn new(repo_root: &'a Path) -> Self {
        Self {
            workspace: CompilerWorkspace::new(repo_root),
        }
    }

    pub(crate) fn runtime_state(self) -> RuntimeStateLayout<'a> {
        RuntimeStateLayout {
            workspace: self.workspace,
        }
    }

    pub(crate) fn capture_provenance(self) -> CaptureProvenanceLayout<'a> {
        CaptureProvenanceLayout {
            workspace: self.workspace,
        }
    }

    pub(crate) fn handoff_bundle(self) -> HandoffBundleLayout<'a> {
        HandoffBundleLayout {
            workspace: self.workspace,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct RuntimeStateLayout<'a> {
    workspace: CompilerWorkspace<'a>,
}

impl<'a> RuntimeStateLayout<'a> {
    pub(crate) fn state_root_relative(self) -> &'static str {
        RUNTIME_STATE_ROOT_RELATIVE
    }

    pub(crate) fn state_root(self) -> NormalizedRepoRelativePath {
        self.workspace
            .normalize_repo_relative(self.state_root_relative())
            .expect("runtime-state root should stay repo-relative")
    }

    pub(crate) fn route_state_relative_path(self, pipeline_id: &str) -> NormalizedRepoRelativePath {
        self.workspace
            .normalize_repo_relative(&format!(
                "{RUNTIME_STATE_PIPELINE_DIR_RELATIVE}/{pipeline_id}.yaml"
            ))
            .expect("runtime-state route-state path should stay repo-relative")
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct CaptureProvenanceLayout<'a> {
    workspace: CompilerWorkspace<'a>,
}

impl<'a> CaptureProvenanceLayout<'a> {
    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn stage_capture_root_relative(self) -> &'static str {
        CAPTURE_PROVENANCE_DIR_RELATIVE
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn stage_capture_root(self) -> NormalizedRepoRelativePath {
        self.workspace
            .normalize_repo_relative(self.stage_capture_root_relative())
            .expect("capture-provenance root should stay repo-relative")
    }

    pub(crate) fn stage_capture_provenance_relative_path(
        self,
        pipeline_id: &str,
        stage_id: &str,
    ) -> NormalizedRepoRelativePath {
        self.workspace
            .normalize_repo_relative(&format!(
                "{CAPTURE_PROVENANCE_DIR_RELATIVE}/{pipeline_id}.{stage_id}.json"
            ))
            .expect("capture-provenance path should stay repo-relative")
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn capture_cache_root_relative(self) -> &'static str {
        CAPTURE_CACHE_DIR_RELATIVE
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn capture_cache_root(self) -> NormalizedRepoRelativePath {
        self.workspace
            .normalize_repo_relative(self.capture_cache_root_relative())
            .expect("capture-cache root should stay repo-relative")
    }

    pub(crate) fn capture_cache_relative_path(
        self,
        capture_id: &str,
    ) -> NormalizedRepoRelativePath {
        self.workspace
            .normalize_repo_relative(&format!("{CAPTURE_CACHE_DIR_RELATIVE}/{capture_id}.yaml"))
            .expect("capture-cache path should stay repo-relative")
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct HandoffBundleLayout<'a> {
    workspace: CompilerWorkspace<'a>,
}

impl<'a> HandoffBundleLayout<'a> {
    #[cfg_attr(not(test), allow(dead_code))]
    pub(crate) fn feature_slice_root_relative(self) -> &'static str {
        HANDOFF_FEATURE_SLICE_DIR_RELATIVE
    }

    pub(crate) fn feature_slice_bundle_root(self, feature_id: &str) -> NormalizedRepoRelativePath {
        self.workspace
            .normalize_repo_relative(&format!(
                "{HANDOFF_FEATURE_SLICE_DIR_RELATIVE}/{feature_id}"
            ))
            .expect("handoff bundle root should stay repo-relative")
    }
}
