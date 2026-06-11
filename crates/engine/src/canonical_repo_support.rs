use std::fs::{self, OpenOptions};
use std::io::Read;
use std::path::{Component, Path, PathBuf};

#[derive(Debug)]
pub(crate) enum RepoRelativeFileAccessError {
    Missing(PathBuf),
    InvalidPath(String),
    SymlinkNotAllowed(PathBuf),
    NotRegularFile(PathBuf),
    ReadFailure {
        path: PathBuf,
        source: std::io::Error,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct NormalizedRepoRelativePath(String);

impl NormalizedRepoRelativePath {
    pub(crate) fn parse(path: &str) -> Result<Self, String> {
        let path = validate_repo_relative_path(path)?;
        let normalized = normalize_validated_repo_relative_path(path)?;
        Ok(Self(normalized))
    }

    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }

    fn as_path(&self) -> &Path {
        Path::new(self.as_str())
    }
}

#[derive(Debug)]
pub(crate) struct RepoRelativeMetadataReadError {
    pub(crate) path: PathBuf,
    pub(crate) source: std::io::Error,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct CanonicalWorkspace<'a> {
    repo_root: &'a Path,
}

impl<'a> CanonicalWorkspace<'a> {
    pub(crate) fn new(repo_root: &'a Path) -> Self {
        Self { repo_root }
    }

    pub(crate) fn normalize_repo_relative(
        &self,
        path: &str,
    ) -> Result<NormalizedRepoRelativePath, String> {
        NormalizedRepoRelativePath::parse(path)
    }

    pub(crate) fn metadata_no_follow(
        &self,
        relative_path: &NormalizedRepoRelativePath,
    ) -> Result<Option<fs::Metadata>, RepoRelativeMetadataReadError> {
        let absolute_path = self.absolute_path(relative_path);
        match fs::symlink_metadata(&absolute_path) {
            Ok(metadata) => Ok(Some(metadata)),
            Err(source) if source.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(source) => Err(RepoRelativeMetadataReadError {
                path: absolute_path,
                source,
            }),
        }
    }

    pub(crate) fn trusted_read(
        &self,
        relative_path: &NormalizedRepoRelativePath,
    ) -> Result<TrustedRepoFile, RepoRelativeFileAccessError> {
        let absolute_path =
            resolve_repo_relative_regular_file_path(self.repo_root, relative_path.as_path())?;
        Ok(TrustedRepoFile { absolute_path })
    }

    fn absolute_path(&self, relative_path: &NormalizedRepoRelativePath) -> PathBuf {
        self.repo_root.join(relative_path.as_path())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct TrustedRepoFile {
    absolute_path: PathBuf,
}

impl TrustedRepoFile {
    pub(crate) fn read_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        read_bytes_no_follow(&self.absolute_path)
    }
}

fn resolve_repo_relative_regular_file_path(
    repo_root: &Path,
    relative_path: &Path,
) -> Result<PathBuf, RepoRelativeFileAccessError> {
    let mut current = repo_root.to_path_buf();
    let mut components = relative_path.components().peekable();

    while let Some(component) = components.next() {
        let Component::Normal(part) = component else {
            continue;
        };
        current.push(part);

        let metadata = match fs::symlink_metadata(&current) {
            Ok(metadata) => metadata,
            Err(source) if source.kind() == std::io::ErrorKind::NotFound => {
                return Err(RepoRelativeFileAccessError::Missing(current.clone()))
            }
            Err(source) => {
                return Err(RepoRelativeFileAccessError::ReadFailure {
                    path: current.clone(),
                    source,
                })
            }
        };

        if metadata.file_type().is_symlink() {
            return Err(RepoRelativeFileAccessError::SymlinkNotAllowed(
                current.clone(),
            ));
        }

        let is_last = components.peek().is_none();
        if is_last {
            if !metadata.is_file() {
                return Err(RepoRelativeFileAccessError::NotRegularFile(current.clone()));
            }
        } else if !metadata.is_dir() {
            return Err(RepoRelativeFileAccessError::NotRegularFile(current.clone()));
        }
    }

    Ok(current)
}

fn validate_repo_relative_path(path: &str) -> Result<&Path, String> {
    let trimmed = path.trim();
    let path = Path::new(trimmed);

    if path.as_os_str().is_empty() {
        return Err("path must not be empty".to_string());
    }

    if path.is_absolute() {
        return Err("path must be repo-relative".to_string());
    }

    for component in path.components() {
        match component {
            Component::Normal(_) => {}
            Component::CurDir => {}
            Component::ParentDir => return Err("path must not escape the repo root".to_string()),
            Component::RootDir | Component::Prefix(_) => {
                return Err("path must be repo-relative".to_string())
            }
        }
    }

    Ok(path)
}

fn normalize_validated_repo_relative_path(path: &Path) -> Result<String, String> {
    let normalized = path
        .components()
        .filter_map(|component| match component {
            Component::Normal(part) => Some(part.to_string_lossy().into_owned()),
            Component::CurDir => None,
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("/");
    if normalized.is_empty() {
        return Err("path must not be empty".to_string());
    }
    Ok(normalized)
}

fn read_bytes_no_follow(path: &Path) -> Result<Vec<u8>, std::io::Error> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;

        let mut file = OpenOptions::new()
            .read(true)
            .custom_flags(libc::O_NOFOLLOW)
            .open(path)?;
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)?;
        Ok(bytes)
    }

    #[cfg(not(unix))]
    {
        fs::read(path)
    }
}
