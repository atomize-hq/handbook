use crate::canonical_artifacts::CanonicalArtifactKind;
use crate::canonical_repo_support::{CanonicalWorkspace, NormalizedRepoRelativePath};
use std::path::Path;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct CanonicalArtifactPathContract {
    namespace_dir: &'static str,
    relative_path: &'static str,
}

impl CanonicalArtifactPathContract {
    pub(crate) const fn new(namespace_dir: &'static str, relative_path: &'static str) -> Self {
        Self {
            namespace_dir,
            relative_path,
        }
    }

    pub(crate) const fn namespace_dir(self) -> &'static str {
        self.namespace_dir
    }

    pub(crate) const fn relative_path(self) -> &'static str {
        self.relative_path
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CanonicalLayoutContract {
    system_root_relative: &'static str,
    charter: CanonicalArtifactPathContract,
    project_context: CanonicalArtifactPathContract,
    feature_spec: CanonicalArtifactPathContract,
}

impl CanonicalLayoutContract {
    const fn new(
        system_root_relative: &'static str,
        charter: CanonicalArtifactPathContract,
        project_context: CanonicalArtifactPathContract,
        feature_spec: CanonicalArtifactPathContract,
    ) -> Self {
        Self {
            system_root_relative,
            charter,
            project_context,
            feature_spec,
        }
    }

    pub const fn from_paths(
        system_root_relative: &'static str,
        charter_namespace_dir: &'static str,
        charter_relative_path: &'static str,
        project_context_namespace_dir: &'static str,
        project_context_relative_path: &'static str,
        feature_spec_namespace_dir: &'static str,
        feature_spec_relative_path: &'static str,
    ) -> Self {
        Self::new(
            system_root_relative,
            CanonicalArtifactPathContract::new(charter_namespace_dir, charter_relative_path),
            CanonicalArtifactPathContract::new(
                project_context_namespace_dir,
                project_context_relative_path,
            ),
            CanonicalArtifactPathContract::new(
                feature_spec_namespace_dir,
                feature_spec_relative_path,
            ),
        )
    }

    pub const fn system_root_relative(self) -> &'static str {
        self.system_root_relative
    }

    const fn artifact(self, kind: CanonicalArtifactKind) -> CanonicalArtifactPathContract {
        match kind {
            CanonicalArtifactKind::Charter => self.charter,
            CanonicalArtifactKind::ProjectContext => self.project_context,
            CanonicalArtifactKind::EnvironmentContext => CanonicalArtifactPathContract {
                namespace_dir: ".handbook/project",
                relative_path:
                    crate::environment_context_artifact::ENVIRONMENT_CONTEXT_CANONICAL_PATH,
            },
            CanonicalArtifactKind::FeatureSpec => self.feature_spec,
        }
    }

    pub const fn namespace_dir(self, kind: CanonicalArtifactKind) -> &'static str {
        self.artifact(kind).namespace_dir()
    }

    pub const fn artifact_relative_path(self, kind: CanonicalArtifactKind) -> &'static str {
        self.artifact(kind).relative_path()
    }
}

pub(crate) const DEFAULT_CANONICAL_LAYOUT_CONTRACT: CanonicalLayoutContract =
    CanonicalLayoutContract::new(
        ".handbook",
        CanonicalArtifactPathContract::new(".handbook/charter", ".handbook/charter/CHARTER.md"),
        CanonicalArtifactPathContract::new(
            ".handbook/project_context",
            ".handbook/project_context/PROJECT_CONTEXT.md",
        ),
        CanonicalArtifactPathContract::new(
            ".handbook/feature_spec",
            ".handbook/feature_spec/FEATURE_SPEC.md",
        ),
    );

pub(crate) const CANONICAL_CHARTER_RELATIVE_PATH: &str =
    DEFAULT_CANONICAL_LAYOUT_CONTRACT.artifact_relative_path(CanonicalArtifactKind::Charter);
pub(crate) const CANONICAL_PROJECT_CONTEXT_RELATIVE_PATH: &str =
    DEFAULT_CANONICAL_LAYOUT_CONTRACT.artifact_relative_path(CanonicalArtifactKind::ProjectContext);
pub(crate) const CANONICAL_FEATURE_SPEC_RELATIVE_PATH: &str =
    DEFAULT_CANONICAL_LAYOUT_CONTRACT.artifact_relative_path(CanonicalArtifactKind::FeatureSpec);

pub(crate) const CANONICAL_CHARTER_NAMESPACE_DIR: &str =
    DEFAULT_CANONICAL_LAYOUT_CONTRACT.namespace_dir(CanonicalArtifactKind::Charter);
pub(crate) const CANONICAL_PROJECT_CONTEXT_NAMESPACE_DIR: &str =
    DEFAULT_CANONICAL_LAYOUT_CONTRACT.namespace_dir(CanonicalArtifactKind::ProjectContext);
pub(crate) const CANONICAL_FEATURE_SPEC_NAMESPACE_DIR: &str =
    DEFAULT_CANONICAL_LAYOUT_CONTRACT.namespace_dir(CanonicalArtifactKind::FeatureSpec);

pub fn default_canonical_layout_contract() -> &'static CanonicalLayoutContract {
    &DEFAULT_CANONICAL_LAYOUT_CONTRACT
}

fn validate_canonical_layout_contract(contract: CanonicalLayoutContract) -> Result<(), String> {
    let _ = NormalizedRepoRelativePath::parse(contract.system_root_relative())?;

    for kind in [
        CanonicalArtifactKind::Charter,
        CanonicalArtifactKind::ProjectContext,
        CanonicalArtifactKind::FeatureSpec,
    ] {
        let artifact = contract.artifact(kind);
        let _ = NormalizedRepoRelativePath::parse(artifact.namespace_dir())?;
        let _ = NormalizedRepoRelativePath::parse(artifact.relative_path())?;
    }

    Ok(())
}

pub(crate) fn canonical_artifact_relative_path(kind: CanonicalArtifactKind) -> &'static str {
    default_canonical_layout_contract().artifact_relative_path(kind)
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct CanonicalLayout<'repo> {
    workspace: CanonicalWorkspace<'repo>,
    contract: CanonicalLayoutContract,
}

impl<'repo> CanonicalLayout<'repo> {
    pub(crate) fn new(repo_root: &'repo Path) -> Self {
        Self::with_contract(repo_root, *default_canonical_layout_contract())
    }

    pub(crate) fn with_contract(repo_root: &'repo Path, contract: CanonicalLayoutContract) -> Self {
        validate_canonical_layout_contract(contract)
            .expect("canonical layout contract should stay repo-relative");
        Self {
            workspace: CanonicalWorkspace::new(repo_root),
            contract,
        }
    }

    pub(crate) fn workspace(self) -> CanonicalWorkspace<'repo> {
        self.workspace
    }

    pub(crate) fn contract(self) -> CanonicalLayoutContract {
        self.contract
    }

    pub(crate) fn system_root_relative(self) -> &'static str {
        self.contract().system_root_relative()
    }

    pub(crate) fn system_root(self) -> NormalizedRepoRelativePath {
        self.workspace()
            .normalize_repo_relative(self.system_root_relative())
            .expect("canonical system root should stay repo-relative")
    }

    pub(crate) fn artifact_relative_path(self, kind: CanonicalArtifactKind) -> &'static str {
        self.contract().artifact_relative_path(kind)
    }

    pub(crate) fn namespace_dir(self, kind: CanonicalArtifactKind) -> &'static str {
        self.contract().namespace_dir(kind)
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

#[cfg(test)]
mod tests {
    use super::{CanonicalLayout, CanonicalLayoutContract};
    use crate::canonical_artifacts::CanonicalArtifactKind;
    use std::path::Path;

    #[test]
    fn canonical_layout_contract_can_drive_non_default_paths() {
        let contract = CanonicalLayoutContract::from_paths(
            ".custom_handbook",
            ".custom_handbook/charter",
            ".custom_handbook/charter/CHARTER.md",
            ".custom_handbook/project_context",
            ".custom_handbook/project_context/PROJECT_CONTEXT.md",
            ".custom_handbook/feature_spec",
            ".custom_handbook/feature_spec/FEATURE_SPEC.md",
        );

        let layout = CanonicalLayout::with_contract(Path::new("."), contract);

        assert_eq!(layout.system_root_relative(), ".custom_handbook");
        assert_eq!(
            layout.artifact_relative_path(CanonicalArtifactKind::Charter),
            ".custom_handbook/charter/CHARTER.md"
        );
        assert_eq!(
            layout.namespace_dir(CanonicalArtifactKind::FeatureSpec),
            ".custom_handbook/feature_spec"
        );
    }
}
