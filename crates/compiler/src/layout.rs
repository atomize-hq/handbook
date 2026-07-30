use crate::repo_file_access::{CompilerWorkspace, NormalizedRepoRelativePath};
use std::path::Path;

pub(crate) const SYSTEM_ROOT_RELATIVE: &str = ".handbook";

pub(crate) const CANONICAL_CHARTER_RELATIVE_PATH: &str = ".handbook/charter/CHARTER.md";
pub(crate) const CANONICAL_PROJECT_CONTEXT_RELATIVE_PATH: &str =
    ".handbook/project_context/PROJECT_CONTEXT.md";
#[allow(dead_code)]
const AUTHORING_LOCK_ROOT_RELATIVE: &str = ".handbook/state/authoring";
const CHARTER_AUTHORING_LOCK_RELATIVE_PATH: &str = ".handbook/state/authoring/charter.lock";
const PROJECT_CONTEXT_AUTHORING_LOCK_RELATIVE_PATH: &str =
    ".handbook/state/authoring/project_context.lock";

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

    pub(crate) fn authoring(self) -> AuthoringLayout<'a> {
        AuthoringLayout { repo_root: self }
    }

    pub(crate) fn workspace(self) -> CompilerWorkspace<'a> {
        self.workspace
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct AuthoringLayout<'a> {
    repo_root: RepoLayoutRoot<'a>,
}

impl<'a> AuthoringLayout<'a> {
    pub(crate) fn workspace(self) -> CompilerWorkspace<'a> {
        self.repo_root.workspace()
    }

    #[allow(dead_code)]
    pub(crate) fn lock_root_relative(self) -> &'static str {
        AUTHORING_LOCK_ROOT_RELATIVE
    }

    #[allow(dead_code)]
    pub(crate) fn lock_root(self) -> NormalizedRepoRelativePath {
        self.workspace()
            .normalize_repo_relative(self.lock_root_relative())
            .expect("authoring lock root should stay repo-relative")
    }

    pub(crate) fn charter(self) -> AuthoringArtifactLayout<'a> {
        AuthoringArtifactLayout {
            authoring: self,
            canonical_target_relative_path: CANONICAL_CHARTER_RELATIVE_PATH,
            lock_relative_path: CHARTER_AUTHORING_LOCK_RELATIVE_PATH,
        }
    }

    pub(crate) fn project_context(self) -> AuthoringArtifactLayout<'a> {
        AuthoringArtifactLayout {
            authoring: self,
            canonical_target_relative_path: CANONICAL_PROJECT_CONTEXT_RELATIVE_PATH,
            lock_relative_path: PROJECT_CONTEXT_AUTHORING_LOCK_RELATIVE_PATH,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct AuthoringArtifactLayout<'a> {
    authoring: AuthoringLayout<'a>,
    canonical_target_relative_path: &'static str,
    lock_relative_path: &'static str,
}

impl<'a> AuthoringArtifactLayout<'a> {
    pub(crate) fn canonical_target_relative(self) -> &'static str {
        self.canonical_target_relative_path
    }

    pub(crate) fn canonical_target(self) -> NormalizedRepoRelativePath {
        self.authoring
            .workspace()
            .normalize_repo_relative(self.canonical_target_relative_path)
            .expect("authoring target should stay repo-relative")
    }

    pub(crate) fn lock_relative_path(self) -> &'static str {
        self.lock_relative_path
    }

    pub(crate) fn lock_path(self) -> NormalizedRepoRelativePath {
        self.authoring
            .workspace()
            .normalize_repo_relative(self.lock_relative_path())
            .expect("authoring lock path should stay repo-relative")
    }
}
