use crate::repo_file_access::{CompilerWorkspace, NormalizedRepoRelativePath};
use std::path::Path;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct RuntimeStateLayoutContract {
    state_root_relative: &'static str,
    pipeline_dir_relative: &'static str,
}

impl RuntimeStateLayoutContract {
    pub(crate) const fn new(
        state_root_relative: &'static str,
        pipeline_dir_relative: &'static str,
    ) -> Self {
        Self {
            state_root_relative,
            pipeline_dir_relative,
        }
    }

    pub(crate) const fn state_root_relative(self) -> &'static str {
        self.state_root_relative
    }

    pub(crate) const fn pipeline_dir_relative(self) -> &'static str {
        self.pipeline_dir_relative
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct CaptureStorageLayoutContract {
    stage_capture_root_relative: &'static str,
    capture_cache_root_relative: &'static str,
}

impl CaptureStorageLayoutContract {
    pub(crate) const fn new(
        stage_capture_root_relative: &'static str,
        capture_cache_root_relative: &'static str,
    ) -> Self {
        Self {
            stage_capture_root_relative,
            capture_cache_root_relative,
        }
    }

    pub(crate) const fn stage_capture_root_relative(self) -> &'static str {
        self.stage_capture_root_relative
    }

    pub(crate) const fn capture_cache_root_relative(self) -> &'static str {
        self.capture_cache_root_relative
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct HandoffBundleLayoutContract {
    feature_slice_root_relative: &'static str,
}

impl HandoffBundleLayoutContract {
    pub(crate) const fn new(feature_slice_root_relative: &'static str) -> Self {
        Self {
            feature_slice_root_relative,
        }
    }

    pub(crate) const fn feature_slice_root_relative(self) -> &'static str {
        self.feature_slice_root_relative
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PipelineStorageLayoutContract {
    runtime_state: RuntimeStateLayoutContract,
    capture_storage: CaptureStorageLayoutContract,
    handoff_bundle: HandoffBundleLayoutContract,
}

impl PipelineStorageLayoutContract {
    const fn new(
        runtime_state: RuntimeStateLayoutContract,
        capture_storage: CaptureStorageLayoutContract,
        handoff_bundle: HandoffBundleLayoutContract,
    ) -> Self {
        Self {
            runtime_state,
            capture_storage,
            handoff_bundle,
        }
    }

    #[allow(dead_code)]
    const fn from_paths(
        state_root_relative: &'static str,
        pipeline_dir_relative: &'static str,
        stage_capture_root_relative: &'static str,
        capture_cache_root_relative: &'static str,
        feature_slice_root_relative: &'static str,
    ) -> Self {
        Self::new(
            RuntimeStateLayoutContract::new(state_root_relative, pipeline_dir_relative),
            CaptureStorageLayoutContract::new(
                stage_capture_root_relative,
                capture_cache_root_relative,
            ),
            HandoffBundleLayoutContract::new(feature_slice_root_relative),
        )
    }

    #[allow(dead_code)]
    pub fn try_from_paths(
        state_root_relative: &'static str,
        pipeline_dir_relative: &'static str,
        stage_capture_root_relative: &'static str,
        capture_cache_root_relative: &'static str,
        feature_slice_root_relative: &'static str,
    ) -> Result<Self, String> {
        let contract = Self::from_paths(
            state_root_relative,
            pipeline_dir_relative,
            stage_capture_root_relative,
            capture_cache_root_relative,
            feature_slice_root_relative,
        );
        validate_pipeline_storage_layout_contract(contract)?;
        Ok(contract)
    }

    pub const fn state_root_relative(self) -> &'static str {
        self.runtime_state.state_root_relative
    }

    pub const fn pipeline_dir_relative(self) -> &'static str {
        self.runtime_state.pipeline_dir_relative
    }

    pub const fn stage_capture_root_relative(self) -> &'static str {
        self.capture_storage.stage_capture_root_relative
    }

    pub const fn capture_cache_root_relative(self) -> &'static str {
        self.capture_storage.capture_cache_root_relative
    }

    pub const fn feature_slice_root_relative(self) -> &'static str {
        self.handoff_bundle.feature_slice_root_relative
    }

    const fn runtime_state(self) -> RuntimeStateLayoutContract {
        self.runtime_state
    }

    const fn capture_storage(self) -> CaptureStorageLayoutContract {
        self.capture_storage
    }

    const fn handoff_bundle(self) -> HandoffBundleLayoutContract {
        self.handoff_bundle
    }
}

pub(crate) const HANDBOOK_PRODUCT_PIPELINE_STORAGE_LAYOUT: PipelineStorageLayoutContract =
    PipelineStorageLayoutContract::new(
        RuntimeStateLayoutContract::new(".handbook/state", ".handbook/state/pipeline"),
        CaptureStorageLayoutContract::new(
            ".handbook/state/pipeline/stage_capture",
            ".handbook/state/pipeline/capture",
        ),
        HandoffBundleLayoutContract::new("artifacts/handoff/feature_slice"),
    );

pub(crate) fn handbook_product_pipeline_storage_layout_contract(
) -> &'static PipelineStorageLayoutContract {
    &HANDBOOK_PRODUCT_PIPELINE_STORAGE_LAYOUT
}

fn validate_pipeline_storage_layout_contract(
    contract: PipelineStorageLayoutContract,
) -> Result<(), String> {
    let runtime_state = contract.runtime_state();
    let capture_storage = contract.capture_storage();
    let handoff_bundle = contract.handoff_bundle();

    let state_root = NormalizedRepoRelativePath::parse(runtime_state.state_root_relative())?;
    let pipeline_dir = NormalizedRepoRelativePath::parse(runtime_state.pipeline_dir_relative())?;
    let stage_capture_root =
        NormalizedRepoRelativePath::parse(capture_storage.stage_capture_root_relative())?;
    let capture_cache_root =
        NormalizedRepoRelativePath::parse(capture_storage.capture_cache_root_relative())?;
    let _ = NormalizedRepoRelativePath::parse(handoff_bundle.feature_slice_root_relative())?;

    validate_repo_relative_containment(
        "state_root_relative",
        &state_root,
        "pipeline_dir_relative",
        &pipeline_dir,
    )?;
    validate_repo_relative_containment(
        "state_root_relative",
        &state_root,
        "stage_capture_root_relative",
        &stage_capture_root,
    )?;
    validate_repo_relative_containment(
        "state_root_relative",
        &state_root,
        "capture_cache_root_relative",
        &capture_cache_root,
    )?;

    Ok(())
}

fn validate_repo_relative_containment(
    parent_label: &str,
    parent: &NormalizedRepoRelativePath,
    child_label: &str,
    child: &NormalizedRepoRelativePath,
) -> Result<(), String> {
    if Path::new(child.as_str()).starts_with(Path::new(parent.as_str())) {
        return Ok(());
    }

    Err(format!(
        "{child_label} (`{}`) must stay within {parent_label} (`{}`)",
        child.as_str(),
        parent.as_str()
    ))
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct RepoLayoutRoot<'a> {
    workspace: CompilerWorkspace<'a>,
    contract: PipelineStorageLayoutContract,
}

impl<'a> RepoLayoutRoot<'a> {
    pub(crate) fn new(repo_root: &'a Path) -> Self {
        Self::with_contract(
            repo_root,
            *handbook_product_pipeline_storage_layout_contract(),
        )
    }

    pub(crate) fn with_contract(
        repo_root: &'a Path,
        contract: PipelineStorageLayoutContract,
    ) -> Self {
        validate_pipeline_storage_layout_contract(contract)
            .expect("pipeline storage layout contract should stay repo-relative");
        Self {
            workspace: CompilerWorkspace::new(repo_root),
            contract,
        }
    }

    pub(crate) fn runtime_state(self) -> RuntimeStateLayout<'a> {
        RuntimeStateLayout {
            workspace: self.workspace,
            contract: self.contract.runtime_state(),
        }
    }

    pub(crate) fn capture_provenance(self) -> CaptureProvenanceLayout<'a> {
        CaptureProvenanceLayout {
            workspace: self.workspace,
            contract: self.contract.capture_storage(),
        }
    }

    pub(crate) fn handoff_bundle(self) -> HandoffBundleLayout<'a> {
        HandoffBundleLayout {
            workspace: self.workspace,
            contract: self.contract.handoff_bundle(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct RuntimeStateLayout<'a> {
    workspace: CompilerWorkspace<'a>,
    contract: RuntimeStateLayoutContract,
}

impl<'a> RuntimeStateLayout<'a> {
    pub(crate) fn state_root_relative(self) -> &'static str {
        self.contract.state_root_relative()
    }

    pub(crate) fn state_root(self) -> NormalizedRepoRelativePath {
        self.workspace
            .normalize_repo_relative(self.state_root_relative())
            .expect("runtime-state root should stay repo-relative")
    }

    pub(crate) fn route_state_relative_path(self, pipeline_id: &str) -> NormalizedRepoRelativePath {
        self.workspace
            .normalize_repo_relative(&format!(
                "{}/{pipeline_id}.yaml",
                self.contract.pipeline_dir_relative()
            ))
            .expect("runtime-state route-state path should stay repo-relative")
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct CaptureProvenanceLayout<'a> {
    workspace: CompilerWorkspace<'a>,
    contract: CaptureStorageLayoutContract,
}

impl<'a> CaptureProvenanceLayout<'a> {
    #[allow(dead_code)]
    pub(crate) fn stage_capture_root_relative(self) -> &'static str {
        self.contract.stage_capture_root_relative()
    }

    #[allow(dead_code)]
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
                "{}/{}.{}.json",
                self.contract.stage_capture_root_relative(),
                pipeline_id,
                stage_id
            ))
            .expect("capture-provenance path should stay repo-relative")
    }

    #[allow(dead_code)]
    pub(crate) fn capture_cache_root_relative(self) -> &'static str {
        self.contract.capture_cache_root_relative()
    }

    #[allow(dead_code)]
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
            .normalize_repo_relative(&format!(
                "{}/{capture_id}.yaml",
                self.contract.capture_cache_root_relative()
            ))
            .expect("capture-cache path should stay repo-relative")
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct HandoffBundleLayout<'a> {
    workspace: CompilerWorkspace<'a>,
    contract: HandoffBundleLayoutContract,
}

impl<'a> HandoffBundleLayout<'a> {
    #[allow(dead_code)]
    pub(crate) fn feature_slice_root_relative(self) -> &'static str {
        self.contract.feature_slice_root_relative()
    }

    pub(crate) fn feature_slice_bundle_root(self, feature_id: &str) -> NormalizedRepoRelativePath {
        self.workspace
            .normalize_repo_relative(&format!(
                "{}/{}",
                self.feature_slice_root_relative(),
                feature_id
            ))
            .expect("handoff bundle root should stay repo-relative")
    }
}

#[cfg(test)]
mod tests {
    use super::{
        validate_pipeline_storage_layout_contract, PipelineStorageLayoutContract, RepoLayoutRoot,
    };
    use std::path::Path;

    #[test]
    fn pipeline_storage_layout_contract_can_drive_non_default_paths() {
        let contract = PipelineStorageLayoutContract::from_paths(
            ".custom_handbook/state",
            ".custom_handbook/state/pipelines",
            ".custom_handbook/state/pipelines/stage_capture",
            ".custom_handbook/state/pipelines/capture_cache",
            "custom_artifacts/handoff/feature_slice",
        );

        assert_eq!(contract.state_root_relative(), ".custom_handbook/state");
        assert_eq!(
            contract.pipeline_dir_relative(),
            ".custom_handbook/state/pipelines"
        );
        assert_eq!(
            contract.stage_capture_root_relative(),
            ".custom_handbook/state/pipelines/stage_capture"
        );
        assert_eq!(
            contract.capture_cache_root_relative(),
            ".custom_handbook/state/pipelines/capture_cache"
        );
        assert_eq!(
            contract.feature_slice_root_relative(),
            "custom_artifacts/handoff/feature_slice"
        );

        let layout = RepoLayoutRoot::with_contract(Path::new("."), contract);

        assert_eq!(
            layout.runtime_state().state_root_relative(),
            ".custom_handbook/state"
        );
        assert_eq!(
            layout
                .runtime_state()
                .route_state_relative_path("foundation")
                .as_str(),
            ".custom_handbook/state/pipelines/foundation.yaml"
        );
        assert_eq!(
            layout
                .capture_provenance()
                .stage_capture_provenance_relative_path("foundation", "stage.10_feature_spec")
                .as_str(),
            ".custom_handbook/state/pipelines/stage_capture/foundation.stage.10_feature_spec.json"
        );
        assert_eq!(
            layout
                .capture_provenance()
                .capture_cache_relative_path("abc123")
                .as_str(),
            ".custom_handbook/state/pipelines/capture_cache/abc123.yaml"
        );
        assert_eq!(
            layout
                .handoff_bundle()
                .feature_slice_bundle_root("feature-123")
                .as_str(),
            "custom_artifacts/handoff/feature_slice/feature-123"
        );
    }

    #[test]
    fn pipeline_storage_layout_contract_rejects_runtime_state_paths_outside_state_root() {
        for (label, contract) in [
            (
                "pipeline_dir_relative",
                PipelineStorageLayoutContract::from_paths(
                    ".custom_handbook/state",
                    ".custom_handbook/pipelines",
                    ".custom_handbook/state/pipelines/stage_capture",
                    ".custom_handbook/state/pipelines/capture_cache",
                    "custom_artifacts/handoff/feature_slice",
                ),
            ),
            (
                "stage_capture_root_relative",
                PipelineStorageLayoutContract::from_paths(
                    ".custom_handbook/state",
                    ".custom_handbook/state/pipelines",
                    ".custom_handbook/captures",
                    ".custom_handbook/state/pipelines/capture_cache",
                    "custom_artifacts/handoff/feature_slice",
                ),
            ),
            (
                "capture_cache_root_relative",
                PipelineStorageLayoutContract::from_paths(
                    ".custom_handbook/state",
                    ".custom_handbook/state/pipelines",
                    ".custom_handbook/state/pipelines/stage_capture",
                    ".custom_handbook/capture_cache",
                    "custom_artifacts/handoff/feature_slice",
                ),
            ),
        ] {
            let err = validate_pipeline_storage_layout_contract(contract)
                .expect_err("contract should reject runtime-state paths outside state root");
            assert!(
                err.contains(label),
                "expected `{label}` in error, got: {err}"
            );
        }
    }

    #[test]
    fn public_constructor_rejects_runtime_state_paths_outside_state_root() {
        let err = PipelineStorageLayoutContract::try_from_paths(
            ".custom_handbook/state",
            ".custom_handbook/pipelines",
            ".custom_handbook/state/pipelines/stage_capture",
            ".custom_handbook/state/pipelines/capture_cache",
            "custom_artifacts/handoff/feature_slice",
        )
        .expect_err("public constructor should reject invalid storage layout");
        assert!(
            err.contains("pipeline_dir_relative"),
            "expected pipeline_dir_relative in error, got: {err}"
        );
    }
}
