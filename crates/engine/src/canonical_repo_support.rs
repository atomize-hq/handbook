use std::fs;
use std::io::Read;
use std::path::{Component, Path, PathBuf};

#[allow(dead_code)]
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
        let file = open_repo_relative_regular_file(self.repo_root, relative_path.as_path())?;
        Ok(TrustedRepoFile { file })
    }

    pub(crate) fn trusted_read_strict(
        &self,
        relative_path: &NormalizedRepoRelativePath,
    ) -> Result<TrustedRepoFile, RepoRelativeFileAccessError> {
        let file = open_repo_relative_regular_file_strict(self.repo_root, relative_path.as_path())?;
        Ok(TrustedRepoFile { file })
    }

    fn absolute_path(&self, relative_path: &NormalizedRepoRelativePath) -> PathBuf {
        self.repo_root.join(relative_path.as_path())
    }
}

#[derive(Debug)]
pub(crate) struct TrustedRepoFile {
    file: fs::File,
}

impl TrustedRepoFile {
    pub(crate) fn read_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut bytes = Vec::new();
        (&self.file).read_to_end(&mut bytes)?;
        Ok(bytes)
    }

    pub(crate) fn read_bytes_bounded(
        &self,
        maximum_bytes: usize,
    ) -> Result<(Vec<u8>, bool), std::io::Error> {
        let sentinel_limit = maximum_bytes.saturating_add(1) as u64;
        let mut bytes = Vec::with_capacity(maximum_bytes.saturating_add(1));
        (&self.file).take(sentinel_limit).read_to_end(&mut bytes)?;
        let exceeded = bytes.len() > maximum_bytes;
        if exceeded {
            bytes.truncate(maximum_bytes);
        }
        Ok((bytes, exceeded))
    }
}

#[cfg(unix)]
fn open_repo_relative_regular_file(
    repo_root: &Path,
    relative_path: &Path,
) -> Result<fs::File, RepoRelativeFileAccessError> {
    open_repo_relative_regular_file_strict(repo_root, relative_path)
}

#[cfg(unix)]
fn open_repo_relative_regular_file_strict(
    repo_root: &Path,
    relative_path: &Path,
) -> Result<fs::File, RepoRelativeFileAccessError> {
    open_repo_relative_regular_file_with_hook(repo_root, relative_path, |_, _| {})
}

#[cfg(unix)]
fn open_repo_relative_regular_file_with_hook(
    repo_root: &Path,
    relative_path: &Path,
    mut after_component_open: impl FnMut(&Path, bool),
) -> Result<fs::File, RepoRelativeFileAccessError> {
    use rustix::fs::{open, openat, Mode, OFlags};

    let root_directory_flags = OFlags::RDONLY | OFlags::CLOEXEC | OFlags::DIRECTORY;
    let directory_flags = root_directory_flags | OFlags::NOFOLLOW;
    let file_flags = OFlags::RDONLY | OFlags::CLOEXEC | OFlags::NOFOLLOW | OFlags::NONBLOCK;
    let mut current = open(repo_root, root_directory_flags, Mode::empty())
        .map_err(|error| classify_descriptor_open_error(repo_root.to_path_buf(), error))?;
    let mut current_path = repo_root.to_path_buf();
    let components = relative_path
        .components()
        .filter_map(|component| match component {
            Component::Normal(part) => Some(part),
            _ => None,
        })
        .collect::<Vec<_>>();

    for (index, part) in components.iter().enumerate() {
        current_path.push(part);
        let flags = if index + 1 == components.len() {
            file_flags
        } else {
            directory_flags
        };
        current = openat(&current, Path::new(part), flags, Mode::empty()).map_err(|error| {
            classify_descriptor_component_open_error(
                &current,
                Path::new(part),
                current_path.clone(),
                error,
            )
        })?;
        after_component_open(&current_path, index + 1 == components.len());
    }

    let file = fs::File::from(current);
    let metadata = file
        .metadata()
        .map_err(|source| RepoRelativeFileAccessError::ReadFailure {
            path: current_path.clone(),
            source,
        })?;
    if !metadata.is_file() {
        return Err(RepoRelativeFileAccessError::NotRegularFile(current_path));
    }
    Ok(file)
}

#[cfg(unix)]
fn classify_descriptor_open_error(
    path: PathBuf,
    error: rustix::io::Errno,
) -> RepoRelativeFileAccessError {
    use rustix::io::Errno;

    match error {
        Errno::NOENT => RepoRelativeFileAccessError::Missing(path),
        Errno::LOOP => RepoRelativeFileAccessError::SymlinkNotAllowed(path),
        Errno::NOTDIR | Errno::ISDIR => RepoRelativeFileAccessError::NotRegularFile(path),
        _ => RepoRelativeFileAccessError::ReadFailure {
            path,
            source: std::io::Error::from_raw_os_error(error.raw_os_error()),
        },
    }
}

#[cfg(unix)]
fn classify_descriptor_component_open_error(
    parent: &impl std::os::fd::AsFd,
    component: &Path,
    path: PathBuf,
    error: rustix::io::Errno,
) -> RepoRelativeFileAccessError {
    use rustix::fs::{statat, AtFlags, FileType};
    use rustix::io::Errno;

    if error == Errno::NOTDIR
        && statat(parent, component, AtFlags::SYMLINK_NOFOLLOW)
            .map(|metadata| FileType::from_raw_mode(metadata.st_mode) == FileType::Symlink)
            .unwrap_or(false)
    {
        return RepoRelativeFileAccessError::SymlinkNotAllowed(path);
    }
    classify_descriptor_open_error(path, error)
}

#[cfg(not(unix))]
fn open_repo_relative_regular_file(
    repo_root: &Path,
    relative_path: &Path,
) -> Result<fs::File, RepoRelativeFileAccessError> {
    let absolute_path = resolve_repo_relative_regular_file_path(repo_root, relative_path)?;
    fs::File::open(&absolute_path).map_err(|source| RepoRelativeFileAccessError::ReadFailure {
        path: absolute_path,
        source,
    })
}

#[cfg(not(unix))]
fn open_repo_relative_regular_file_strict(
    _repo_root: &Path,
    _relative_path: &Path,
) -> Result<fs::File, RepoRelativeFileAccessError> {
    Err(RepoRelativeFileAccessError::InvalidPath(
        "descriptor-relative no-follow access is unavailable on this platform".to_string(),
    ))
}

#[cfg(not(unix))]
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
                return Err(RepoRelativeFileAccessError::Missing(current));
            }
            Err(source) => {
                return Err(RepoRelativeFileAccessError::ReadFailure {
                    path: current,
                    source,
                });
            }
        };
        if metadata.file_type().is_symlink() {
            return Err(RepoRelativeFileAccessError::SymlinkNotAllowed(current));
        }
        let is_last = components.peek().is_none();
        if (is_last && !metadata.is_file()) || (!is_last && !metadata.is_dir()) {
            return Err(RepoRelativeFileAccessError::NotRegularFile(current));
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

#[cfg(test)]
mod trusted_read_race_tests {
    use super::NormalizedRepoRelativePath;
    #[cfg(unix)]
    use super::{open_repo_relative_regular_file_with_hook, TrustedRepoFile};

    #[cfg(unix)]
    #[test]
    fn trusted_handle_survives_intermediate_and_final_path_substitution() {
        use std::os::unix::fs::symlink;

        let repo = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(repo.path().join("inside/sub")).unwrap();
        std::fs::create_dir_all(repo.path().join("outside/sub")).unwrap();
        std::fs::write(repo.path().join("inside/sub/value.yaml"), b"inside\n").unwrap();
        std::fs::write(repo.path().join("outside/sub/value.yaml"), b"outside\n").unwrap();
        let relative = NormalizedRepoRelativePath::parse("inside/sub/value.yaml").unwrap();
        let file = open_repo_relative_regular_file_with_hook(
            repo.path(),
            relative.as_path(),
            |opened_path, is_last| {
                if !is_last && opened_path == repo.path().join("inside") {
                    std::fs::rename(repo.path().join("inside"), repo.path().join("original"))
                        .unwrap();
                    symlink("outside", repo.path().join("inside")).unwrap();
                }
            },
        )
        .unwrap();
        let trusted = TrustedRepoFile { file };

        std::fs::rename(
            repo.path().join("original/sub/value.yaml"),
            repo.path().join("original/sub/retained.yaml"),
        )
        .unwrap();
        std::fs::write(repo.path().join("original/sub/value.yaml"), b"replaced\n").unwrap();

        assert_eq!(trusted.read_bytes().unwrap(), b"inside\n");
    }

    #[cfg(not(unix))]
    #[test]
    fn legacy_canonical_reads_remain_available_while_strict_registry_reads_fail_closed() {
        use super::CanonicalWorkspace;

        let repo = tempfile::tempdir().unwrap();
        std::fs::write(repo.path().join("value.yaml"), b"legacy\n").unwrap();
        let relative = NormalizedRepoRelativePath::parse("value.yaml").unwrap();
        let workspace = CanonicalWorkspace::new(repo.path());

        assert_eq!(
            workspace
                .trusted_read(&relative)
                .unwrap()
                .read_bytes()
                .unwrap(),
            b"legacy\n"
        );
        assert!(workspace.trusted_read_strict(&relative).is_err());
    }
}
