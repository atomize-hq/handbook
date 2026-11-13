use crate::repo_file_access::NormalizedRepoRelativePath;
use std::path::Path;

pub(crate) const DECLARATIVE_ROOT: &str = "core";
pub(crate) const PIPELINES_ROOT: &str = "core/pipelines";
pub(crate) const PROFILES_ROOT: &str = "core/profiles";
pub(crate) const RUNNERS_ROOT: &str = "core/runners";
pub(crate) const STAGES_ROOT: &str = "core/stages";
pub(crate) const PROFILES_ROOT_DISPLAY: &str = "core/profiles/";
pub(crate) const RUNNERS_ROOT_DISPLAY: &str = "core/runners/";

/// Reviewed public owner for declarative repo-relative path roots.
///
/// Handbook-product default helpers remain private; this contract exposes only
/// the typed root ownership that downstream consumers must construct and read.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PipelineDeclarativeRootsContract {
    pipeline_root_relative: &'static str,
    profile_root_relative: &'static str,
    runner_root_relative: &'static str,
    stage_root_relative: &'static str,
}

impl PipelineDeclarativeRootsContract {
    const fn from_paths(
        pipeline_root_relative: &'static str,
        profile_root_relative: &'static str,
        runner_root_relative: &'static str,
        stage_root_relative: &'static str,
    ) -> Self {
        Self {
            pipeline_root_relative,
            profile_root_relative,
            runner_root_relative,
            stage_root_relative,
        }
    }

    pub fn try_from_paths(
        pipeline_root_relative: &'static str,
        profile_root_relative: &'static str,
        runner_root_relative: &'static str,
        stage_root_relative: &'static str,
    ) -> Result<Self, String> {
        let contract = Self::from_paths(
            pipeline_root_relative,
            profile_root_relative,
            runner_root_relative,
            stage_root_relative,
        );
        validate_pipeline_declarative_roots_contract(contract)?;
        Ok(contract)
    }

    pub const fn pipeline_root_relative(self) -> &'static str {
        self.pipeline_root_relative
    }

    pub const fn profile_root_relative(self) -> &'static str {
        self.profile_root_relative
    }

    pub const fn runner_root_relative(self) -> &'static str {
        self.runner_root_relative
    }

    pub const fn stage_root_relative(self) -> &'static str {
        self.stage_root_relative
    }

    pub(crate) fn pipeline_root(self) -> &'static Path {
        Path::new(self.pipeline_root_relative())
    }

    pub(crate) fn profile_root(self) -> &'static Path {
        Path::new(self.profile_root_relative())
    }

    pub(crate) fn runner_root(self) -> &'static Path {
        Path::new(self.runner_root_relative())
    }

    pub(crate) fn stage_root(self) -> &'static Path {
        Path::new(self.stage_root_relative())
    }

    #[allow(dead_code)]
    pub(crate) fn pipeline_file(self, file_name: &str) -> String {
        format!("{}/{file_name}", self.pipeline_root_relative())
    }

    pub(crate) fn stage_file(self, file_name: &str) -> String {
        format!("{}/{file_name}", self.stage_root_relative())
    }

    pub(crate) fn runner_file(self, runner_id: &str) -> String {
        format!("{}/{runner_id}.md", self.runner_root_relative())
    }

    pub(crate) fn profile_file(self, profile_id: &str, file_name: &str) -> String {
        format!("{}/{profile_id}/{file_name}", self.profile_root_relative())
    }

    pub(crate) fn is_profile_file(self, path: &str, profile_id: &str) -> bool {
        path == self.profile_file(profile_id, "profile.yaml")
            || path == self.profile_file(profile_id, "commands.yaml")
            || path == self.profile_file(profile_id, "conventions.md")
    }
}

pub(crate) const HANDBOOK_PRODUCT_PIPELINE_DECLARATIVE_ROOTS: PipelineDeclarativeRootsContract =
    PipelineDeclarativeRootsContract::from_paths(
        PIPELINES_ROOT,
        PROFILES_ROOT,
        RUNNERS_ROOT,
        STAGES_ROOT,
    );

/// Explicit handbook-product defaults for the current supported loader/catalog
/// boundary.
pub(crate) fn handbook_product_declarative_roots() -> &'static PipelineDeclarativeRootsContract {
    &HANDBOOK_PRODUCT_PIPELINE_DECLARATIVE_ROOTS
}

pub(crate) fn pipeline_root() -> &'static Path {
    handbook_product_declarative_roots().pipeline_root()
}

pub(crate) fn runner_root() -> &'static Path {
    handbook_product_declarative_roots().runner_root()
}

pub(crate) fn profile_root() -> &'static Path {
    handbook_product_declarative_roots().profile_root()
}

#[allow(dead_code)]
pub(crate) fn stage_root() -> &'static Path {
    handbook_product_declarative_roots().stage_root()
}

pub(crate) fn runner_file(runner_id: &str) -> String {
    handbook_product_declarative_roots().runner_file(runner_id)
}

pub(crate) fn profile_file(profile_id: &str, file_name: &str) -> String {
    handbook_product_declarative_roots().profile_file(profile_id, file_name)
}

pub(crate) fn is_profile_file(path: &str, profile_id: &str) -> bool {
    handbook_product_declarative_roots().is_profile_file(path, profile_id)
}

pub(crate) fn is_canonical_declarative_path(path: &str) -> bool {
    path == DECLARATIVE_ROOT || path.starts_with("core/")
}

fn validate_pipeline_declarative_roots_contract(
    contract: PipelineDeclarativeRootsContract,
) -> Result<(), String> {
    let _ = NormalizedRepoRelativePath::parse(contract.pipeline_root_relative())?;
    let _ = NormalizedRepoRelativePath::parse(contract.profile_root_relative())?;
    let _ = NormalizedRepoRelativePath::parse(contract.runner_root_relative())?;
    let _ = NormalizedRepoRelativePath::parse(contract.stage_root_relative())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::PipelineDeclarativeRootsContract;

    #[test]
    fn public_constructor_accepts_non_default_repo_relative_roots() {
        let contract = PipelineDeclarativeRootsContract::try_from_paths(
            "custom/core/pipelines",
            "custom/core/profiles",
            "custom/core/runners",
            "custom/core/stages",
        )
        .expect("public constructor should accept repo-relative declarative roots");

        assert_eq!(contract.pipeline_root_relative(), "custom/core/pipelines");
        assert_eq!(contract.profile_root_relative(), "custom/core/profiles");
        assert_eq!(contract.runner_root_relative(), "custom/core/runners");
        assert_eq!(contract.stage_root_relative(), "custom/core/stages");
    }

    #[test]
    fn public_constructor_rejects_paths_that_escape_repo_root() {
        let err = PipelineDeclarativeRootsContract::try_from_paths(
            "../core/pipelines",
            "core/profiles",
            "core/runners",
            "core/stages",
        )
        .expect_err("public constructor should reject non-repo-relative roots");

        assert!(
            err.contains("repo root"),
            "expected repo-root validation error, got: {err}"
        );
    }
}
