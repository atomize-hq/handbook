use crate::canonical_repo_support::{CanonicalWorkspace, NormalizedRepoRelativePath};
use std::path::Path;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CanonicalLayoutContract {
    system_root_relative: &'static str,
}

impl CanonicalLayoutContract {
    const fn new(system_root_relative: &'static str) -> Self {
        Self {
            system_root_relative,
        }
    }

    pub const fn from_paths(
        system_root_relative: &'static str,
        _charter_namespace_dir: &'static str,
        _charter_relative_path: &'static str,
        _project_context_namespace_dir: &'static str,
        _project_context_relative_path: &'static str,
        _feature_spec_namespace_dir: &'static str,
        _feature_spec_relative_path: &'static str,
    ) -> Self {
        Self::new(system_root_relative)
    }

    pub const fn system_root_relative(self) -> &'static str {
        self.system_root_relative
    }
}

pub(crate) const DEFAULT_CANONICAL_LAYOUT_CONTRACT: CanonicalLayoutContract =
    CanonicalLayoutContract::new(".handbook");

pub(crate) const CANONICAL_CHARTER_RELATIVE_PATH: &str = ".handbook/charter/CHARTER.md";
pub(crate) const CANONICAL_PROJECT_CONTEXT_RELATIVE_PATH: &str =
    ".handbook/project_context/PROJECT_CONTEXT.md";
pub(crate) const CANONICAL_FEATURE_SPEC_RELATIVE_PATH: &str =
    ".handbook/feature_spec/FEATURE_SPEC.md";

pub(crate) const CANONICAL_CHARTER_NAMESPACE_DIR: &str = ".handbook/charter";
pub(crate) const CANONICAL_PROJECT_CONTEXT_NAMESPACE_DIR: &str = ".handbook/project_context";
pub(crate) const CANONICAL_FEATURE_SPEC_NAMESPACE_DIR: &str = ".handbook/feature_spec";

pub fn default_canonical_layout_contract() -> &'static CanonicalLayoutContract {
    &DEFAULT_CANONICAL_LAYOUT_CONTRACT
}

fn validate_canonical_layout_contract(contract: CanonicalLayoutContract) -> Result<(), String> {
    let _ = NormalizedRepoRelativePath::parse(contract.system_root_relative())?;
    Ok(())
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
}

#[cfg(test)]
mod tests {
    use super::{CanonicalLayout, CanonicalLayoutContract};
    use std::path::Path;

    #[test]
    fn canonical_layout_contract_retains_only_system_root_authority() {
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
    }
}
