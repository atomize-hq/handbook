use crate::canonical_artifacts::CanonicalArtifactKind;
use crate::canonical_repo_support::{CanonicalWorkspace, NormalizedRepoRelativePath};
use std::path::Path;

pub(crate) const SYSTEM_ROOT_RELATIVE: &str = ".handbook";

pub(crate) const CANONICAL_CHARTER_RELATIVE_PATH: &str = ".handbook/charter/CHARTER.md";
pub(crate) const CANONICAL_PROJECT_CONTEXT_RELATIVE_PATH: &str =
    ".handbook/project_context/PROJECT_CONTEXT.md";
pub(crate) const CANONICAL_ENVIRONMENT_INVENTORY_RELATIVE_PATH: &str =
    ".handbook/environment_inventory/ENVIRONMENT_INVENTORY.md";
pub(crate) const CANONICAL_FEATURE_SPEC_RELATIVE_PATH: &str =
    ".handbook/feature_spec/FEATURE_SPEC.md";

pub(crate) const CANONICAL_CHARTER_NAMESPACE_DIR: &str = ".handbook/charter";
pub(crate) const CANONICAL_PROJECT_CONTEXT_NAMESPACE_DIR: &str = ".handbook/project_context";
pub(crate) const CANONICAL_ENVIRONMENT_INVENTORY_NAMESPACE_DIR: &str =
    ".handbook/environment_inventory";
pub(crate) const CANONICAL_FEATURE_SPEC_NAMESPACE_DIR: &str = ".handbook/feature_spec";

pub(crate) fn canonical_artifact_relative_path(kind: CanonicalArtifactKind) -> &'static str {
    match kind {
        CanonicalArtifactKind::Charter => CANONICAL_CHARTER_RELATIVE_PATH,
        CanonicalArtifactKind::ProjectContext => CANONICAL_PROJECT_CONTEXT_RELATIVE_PATH,
        CanonicalArtifactKind::EnvironmentInventory => {
            CANONICAL_ENVIRONMENT_INVENTORY_RELATIVE_PATH
        }
        CanonicalArtifactKind::FeatureSpec => CANONICAL_FEATURE_SPEC_RELATIVE_PATH,
    }
}

pub(crate) fn canonical_artifact_namespace_dir(kind: CanonicalArtifactKind) -> &'static str {
    match kind {
        CanonicalArtifactKind::Charter => CANONICAL_CHARTER_NAMESPACE_DIR,
        CanonicalArtifactKind::ProjectContext => CANONICAL_PROJECT_CONTEXT_NAMESPACE_DIR,
        CanonicalArtifactKind::EnvironmentInventory => {
            CANONICAL_ENVIRONMENT_INVENTORY_NAMESPACE_DIR
        }
        CanonicalArtifactKind::FeatureSpec => CANONICAL_FEATURE_SPEC_NAMESPACE_DIR,
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct CanonicalLayout<'a> {
    workspace: CanonicalWorkspace<'a>,
}

impl<'a> CanonicalLayout<'a> {
    pub(crate) fn new(repo_root: &'a Path) -> Self {
        Self {
            workspace: CanonicalWorkspace::new(repo_root),
        }
    }

    pub(crate) fn workspace(self) -> CanonicalWorkspace<'a> {
        self.workspace
    }

    pub(crate) fn system_root_relative(self) -> &'static str {
        SYSTEM_ROOT_RELATIVE
    }

    pub(crate) fn system_root(self) -> NormalizedRepoRelativePath {
        self.workspace()
            .normalize_repo_relative(self.system_root_relative())
            .expect("canonical .handbook root should stay repo-relative")
    }

    pub(crate) fn artifact_relative_path(self, kind: CanonicalArtifactKind) -> &'static str {
        canonical_artifact_relative_path(kind)
    }

    pub(crate) fn namespace_dir(self, kind: CanonicalArtifactKind) -> &'static str {
        canonical_artifact_namespace_dir(kind)
    }

    pub(crate) fn artifact_path(self, kind: CanonicalArtifactKind) -> NormalizedRepoRelativePath {
        self.workspace()
            .normalize_repo_relative(self.artifact_relative_path(kind))
            .expect("canonical artifact path should stay repo-relative")
    }

    pub(crate) fn namespace_dir_path(
        self,
        kind: CanonicalArtifactKind,
    ) -> NormalizedRepoRelativePath {
        self.workspace()
            .normalize_repo_relative(self.namespace_dir(kind))
            .expect("canonical namespace path should stay repo-relative")
    }
}
