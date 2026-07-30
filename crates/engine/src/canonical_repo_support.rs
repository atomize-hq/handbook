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
        Ok(TrustedRepoFile {
            file,
            target_path: self.absolute_path(relative_path),
            _directory_guards: Vec::new(),
        })
    }

    pub(crate) fn trusted_read_strict(
        &self,
        relative_path: &NormalizedRepoRelativePath,
    ) -> Result<TrustedRepoFile, RepoRelativeFileAccessError> {
        open_repo_relative_regular_file_strict(self.repo_root, relative_path.as_path())
    }

    fn absolute_path(&self, relative_path: &NormalizedRepoRelativePath) -> PathBuf {
        self.repo_root.join(relative_path.as_path())
    }
}

#[derive(Debug)]
pub(crate) struct TrustedRepoFile {
    file: fs::File,
    target_path: PathBuf,
    _directory_guards: Vec<fs::File>,
}

#[cfg(unix)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct TrustedRepoFileIdentity {
    device: u64,
    inode: u64,
}

impl TrustedRepoFile {
    pub(crate) fn metadata(&self) -> Result<fs::Metadata, std::io::Error> {
        self.file.metadata()
    }

    #[cfg(unix)]
    pub(crate) fn identity(&self) -> Result<TrustedRepoFileIdentity, std::io::Error> {
        use std::os::unix::fs::MetadataExt;

        let metadata = self.file.metadata()?;
        Ok(TrustedRepoFileIdentity {
            device: metadata.dev(),
            inode: metadata.ino(),
        })
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

    pub(crate) fn read_bytes_bounded_stable(
        &self,
        maximum_bytes: usize,
    ) -> Result<(Vec<u8>, bool), std::io::Error> {
        self.read_bytes_bounded_stable_with(maximum_bytes, || {})
    }

    #[cfg(test)]
    pub(crate) fn read_bytes_bounded_stable_with_hook(
        &self,
        maximum_bytes: usize,
        hook: impl FnOnce(),
    ) -> Result<(Vec<u8>, bool), std::io::Error> {
        self.read_bytes_bounded_stable_with(maximum_bytes, hook)
    }

    fn read_bytes_bounded_stable_with(
        &self,
        maximum_bytes: usize,
        hook: impl FnOnce(),
    ) -> Result<(Vec<u8>, bool), std::io::Error> {
        let before = self.file.metadata()?;
        if !metadata_has_single_stable_identity(&before) || !path_has_single_link(&self.target_path)
        {
            return Err(std::io::Error::other(
                "repository file does not have one stable regular-file identity",
            ));
        }
        hook();
        let observed = self.read_bytes_bounded(maximum_bytes)?;
        let after = self.file.metadata()?;
        let length_matches_observation = if observed.1 {
            after.len() > observed.0.len() as u64
        } else {
            after.len() == observed.0.len() as u64
        };
        if !same_stable_file_observation(&before, &after) || !length_matches_observation {
            return Err(std::io::Error::other(
                "repository file changed during retained-handle observation",
            ));
        }
        Ok(observed)
    }
}

#[cfg(unix)]
fn metadata_has_single_stable_identity(metadata: &fs::Metadata) -> bool {
    use std::os::unix::fs::MetadataExt;

    metadata.is_file() && metadata.nlink() == 1
}

#[cfg(unix)]
fn path_has_single_link(_path: &Path) -> bool {
    true
}

#[cfg(windows)]
fn metadata_has_single_stable_identity(metadata: &fs::Metadata) -> bool {
    metadata.is_file()
}

#[cfg(windows)]
pub(crate) fn path_has_single_link(path: &Path) -> bool {
    let Some(system_root) = std::env::var_os("SystemRoot") else {
        return false;
    };
    let system_root = PathBuf::from(system_root);
    let fsutil = system_root.join("System32/fsutil.exe");
    let mut extended = std::ffi::OsString::from(r"\\?\");
    extended.push(path.as_os_str());
    if let Some(single_link) = [path.as_os_str(), extended.as_os_str()]
        .into_iter()
        .find_map(|candidate| {
            std::process::Command::new(&fsutil)
                .arg("hardlink")
                .arg("list")
                .arg(candidate)
                .output()
                .ok()
                .filter(|output| output.status.success())
                .map(|output| {
                    String::from_utf8_lossy(&output.stdout)
                        .lines()
                        .filter(|line| !line.trim().is_empty())
                        .count()
                        == 1
                })
        })
    {
        return single_link;
    }

    // `fsutil hardlink list` does not accept extended-length paths. The
    // Windows PowerShell file-system provider obtains the same native link
    // identity for those paths. Any missing provider, ambiguous link type, or
    // command failure remains fail-closed.
    let powershell = system_root.join("System32/WindowsPowerShell/v1.0/powershell.exe");
    std::process::Command::new(powershell)
        .args([
            "-NoLogo",
            "-NoProfile",
            "-NonInteractive",
            "-Command",
            "try { $item = Get-Item -LiteralPath $env:HANDBOOK_STRICT_READ_PATH -Force -ErrorAction Stop; if ([string]::IsNullOrEmpty([string]$item.LinkType)) { exit 0 }; exit 1 } catch { exit 2 }",
        ])
        .env("HANDBOOK_STRICT_READ_PATH", extended)
        .status()
        .is_ok_and(|status| status.success())
}

#[cfg(all(not(unix), not(windows)))]
fn metadata_has_single_stable_identity(_metadata: &fs::Metadata) -> bool {
    false
}

#[cfg(all(not(unix), not(windows)))]
fn path_has_single_link(_path: &Path) -> bool {
    false
}

#[cfg(unix)]
fn same_stable_file_observation(before: &fs::Metadata, after: &fs::Metadata) -> bool {
    use std::os::unix::fs::MetadataExt;

    before.dev() == after.dev()
        && before.ino() == after.ino()
        && before.nlink() == after.nlink()
        && before.len() == after.len()
        && before.mtime() == after.mtime()
        && before.mtime_nsec() == after.mtime_nsec()
        && before.ctime() == after.ctime()
        && before.ctime_nsec() == after.ctime_nsec()
}

#[cfg(windows)]
fn same_stable_file_observation(before: &fs::Metadata, after: &fs::Metadata) -> bool {
    use std::os::windows::fs::MetadataExt;

    before.file_size() == after.file_size()
        && before.creation_time() == after.creation_time()
        && before.last_write_time() == after.last_write_time()
}

#[cfg(all(not(unix), not(windows)))]
fn same_stable_file_observation(_before: &fs::Metadata, _after: &fs::Metadata) -> bool {
    false
}

#[cfg(unix)]
fn open_repo_relative_regular_file(
    repo_root: &Path,
    relative_path: &Path,
) -> Result<fs::File, RepoRelativeFileAccessError> {
    open_repo_relative_regular_file_with_hook(repo_root, relative_path, |_, _| {})
}

#[cfg(unix)]
fn open_repo_relative_regular_file_strict(
    repo_root: &Path,
    relative_path: &Path,
) -> Result<TrustedRepoFile, RepoRelativeFileAccessError> {
    let file = open_repo_relative_regular_file_with_hook(repo_root, relative_path, |_, _| {})?;
    Ok(TrustedRepoFile {
        file,
        target_path: repo_root.join(relative_path),
        _directory_guards: Vec::new(),
    })
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

#[cfg(windows)]
fn open_repo_relative_regular_file_strict(
    repo_root: &Path,
    relative_path: &Path,
) -> Result<TrustedRepoFile, RepoRelativeFileAccessError> {
    use std::os::windows::fs::{MetadataExt, OpenOptionsExt};

    const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x0000_0400;
    const FILE_SHARE_READ: u32 = 0x0000_0001;
    const FILE_SHARE_WRITE: u32 = 0x0000_0002;
    const FILE_FLAG_BACKUP_SEMANTICS: u32 = 0x0200_0000;
    const FILE_FLAG_OPEN_REPARSE_POINT: u32 = 0x0020_0000;

    let root_metadata = fs::symlink_metadata(repo_root).map_err(|source| {
        RepoRelativeFileAccessError::ReadFailure {
            path: repo_root.to_path_buf(),
            source,
        }
    })?;
    if root_metadata.file_type().is_symlink()
        || root_metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0
    {
        return Err(RepoRelativeFileAccessError::SymlinkNotAllowed(
            repo_root.to_path_buf(),
        ));
    }

    let absolute_path = repo_root.join(relative_path);
    let mut directory_guards = Vec::new();
    let root_handle = fs::OpenOptions::new()
        .read(true)
        .share_mode(FILE_SHARE_READ | FILE_SHARE_WRITE)
        .custom_flags(FILE_FLAG_BACKUP_SEMANTICS | FILE_FLAG_OPEN_REPARSE_POINT)
        .open(repo_root)
        .map_err(|source| RepoRelativeFileAccessError::ReadFailure {
            path: repo_root.to_path_buf(),
            source,
        })?;
    if !root_handle
        .metadata()
        .map_err(|source| RepoRelativeFileAccessError::ReadFailure {
            path: repo_root.to_path_buf(),
            source,
        })?
        .is_dir()
    {
        return Err(RepoRelativeFileAccessError::NotRegularFile(
            repo_root.to_path_buf(),
        ));
    }
    directory_guards.push(root_handle);
    let mut current = repo_root.to_path_buf();
    let components = relative_path
        .components()
        .filter_map(|component| match component {
            Component::Normal(part) => Some(part),
            _ => None,
        })
        .collect::<Vec<_>>();
    for part in components.iter().take(components.len().saturating_sub(1)) {
        current.push(part);
        let directory = fs::OpenOptions::new()
            .read(true)
            .share_mode(FILE_SHARE_READ | FILE_SHARE_WRITE)
            .custom_flags(FILE_FLAG_BACKUP_SEMANTICS | FILE_FLAG_OPEN_REPARSE_POINT)
            .open(&current)
            .map_err(|source| {
                if source.kind() == std::io::ErrorKind::NotFound {
                    RepoRelativeFileAccessError::Missing(current.clone())
                } else {
                    RepoRelativeFileAccessError::ReadFailure {
                        path: current.clone(),
                        source,
                    }
                }
            })?;
        let metadata =
            directory
                .metadata()
                .map_err(|source| RepoRelativeFileAccessError::ReadFailure {
                    path: current.clone(),
                    source,
                })?;
        if !metadata.is_dir() || metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0 {
            return Err(RepoRelativeFileAccessError::SymlinkNotAllowed(current));
        }
        directory_guards.push(directory);
    }

    let target_path_metadata = fs::symlink_metadata(&absolute_path).map_err(|source| {
        if source.kind() == std::io::ErrorKind::NotFound {
            RepoRelativeFileAccessError::Missing(absolute_path.clone())
        } else {
            RepoRelativeFileAccessError::ReadFailure {
                path: absolute_path.clone(),
                source,
            }
        }
    })?;
    if target_path_metadata.file_type().is_symlink()
        || target_path_metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0
    {
        return Err(RepoRelativeFileAccessError::SymlinkNotAllowed(
            absolute_path,
        ));
    }
    if !target_path_metadata.is_file() {
        return Err(RepoRelativeFileAccessError::NotRegularFile(absolute_path));
    }

    let canonical_root =
        fs::canonicalize(repo_root).map_err(|source| RepoRelativeFileAccessError::ReadFailure {
            path: repo_root.to_path_buf(),
            source,
        })?;
    let canonical_target = fs::canonicalize(&absolute_path).map_err(|source| {
        if source.kind() == std::io::ErrorKind::NotFound {
            RepoRelativeFileAccessError::Missing(absolute_path.clone())
        } else {
            RepoRelativeFileAccessError::ReadFailure {
                path: absolute_path.clone(),
                source,
            }
        }
    })?;
    if canonical_target.strip_prefix(&canonical_root).is_err() {
        return Err(RepoRelativeFileAccessError::InvalidPath(
            "repository file resolves outside the repository root".to_string(),
        ));
    }

    let file = fs::OpenOptions::new()
        .read(true)
        .share_mode(FILE_SHARE_READ)
        .open(&absolute_path)
        .map_err(|source| {
            if source.kind() == std::io::ErrorKind::NotFound {
                RepoRelativeFileAccessError::Missing(absolute_path.clone())
            } else {
                RepoRelativeFileAccessError::ReadFailure {
                    path: absolute_path.clone(),
                    source,
                }
            }
        })?;
    let handle_metadata =
        file.metadata()
            .map_err(|source| RepoRelativeFileAccessError::ReadFailure {
                path: absolute_path.clone(),
                source,
            })?;
    let path_metadata = fs::symlink_metadata(&absolute_path).map_err(|source| {
        RepoRelativeFileAccessError::ReadFailure {
            path: absolute_path.clone(),
            source,
        }
    })?;
    if !handle_metadata.is_file() || !path_metadata.is_file() {
        return Err(RepoRelativeFileAccessError::NotRegularFile(absolute_path));
    }
    if path_metadata.file_type().is_symlink()
        || path_metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0
        || handle_metadata.file_size() != path_metadata.file_size()
        || handle_metadata.creation_time() != path_metadata.creation_time()
        || handle_metadata.last_write_time() != path_metadata.last_write_time()
    {
        return Err(RepoRelativeFileAccessError::InvalidPath(
            "repository file changed during strict read admission".to_string(),
        ));
    }
    Ok(TrustedRepoFile {
        file,
        target_path: absolute_path,
        _directory_guards: directory_guards,
    })
}

#[cfg(all(not(unix), not(windows)))]
fn open_repo_relative_regular_file_strict(
    _repo_root: &Path,
    _relative_path: &Path,
) -> Result<TrustedRepoFile, RepoRelativeFileAccessError> {
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

    #[cfg(windows)]
    #[test]
    fn strict_read_accepts_a_regular_repo_relative_file_on_windows() {
        let repo = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(repo.path().join("definitions/schemas")).unwrap();
        std::fs::write(
            repo.path().join("definitions/schemas/value.json"),
            b"{\"value\":true}\n",
        )
        .unwrap();
        let relative = NormalizedRepoRelativePath::parse("definitions/schemas/value.json").unwrap();

        let file = super::open_repo_relative_regular_file_strict(repo.path(), relative.as_path())
            .expect("regular read-only repository file should be admitted on Windows");

        assert!(file.file.metadata().unwrap().is_file());
        assert!(std::fs::OpenOptions::new()
            .write(true)
            .open(repo.path().join("definitions/schemas/value.json"))
            .is_err());
        assert!(std::fs::rename(
            repo.path().join("definitions"),
            repo.path().join("definitions-renamed")
        )
        .is_err());
    }

    #[cfg(windows)]
    #[test]
    fn retained_windows_handle_blocks_same_size_mutation_during_stable_read() {
        let repo = tempfile::tempdir().unwrap();
        let path = repo.path().join("value.yaml");
        std::fs::write(&path, b"before\n").unwrap();
        let relative = NormalizedRepoRelativePath::parse("value.yaml").unwrap();
        let file = super::open_repo_relative_regular_file_strict(repo.path(), relative.as_path())
            .expect("strict handle");

        let (bytes, exceeded) = file
            .read_bytes_bounded_stable_with_hook(1024, || {
                assert!(std::fs::write(&path, b"after!\n").is_err());
            })
            .expect("retained handle keeps one stable observation");
        assert!(!exceeded);
        assert_eq!(bytes, b"before\n");
    }

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
        let trusted = TrustedRepoFile {
            file,
            target_path: repo.path().join("original/sub/value.yaml"),
            _directory_guards: Vec::new(),
        };

        std::fs::rename(
            repo.path().join("original/sub/value.yaml"),
            repo.path().join("original/sub/retained.yaml"),
        )
        .unwrap();
        std::fs::write(repo.path().join("original/sub/value.yaml"), b"replaced\n").unwrap();

        assert_eq!(
            trusted.read_bytes_bounded_stable(1024).unwrap().0,
            b"inside\n"
        );
    }

    #[cfg(unix)]
    #[test]
    fn stable_read_rejects_same_size_mutation_with_restored_mtime() {
        let repo = tempfile::tempdir().unwrap();
        let path = repo.path().join("value.yaml");
        let timestamp = repo.path().join("timestamp.reference");
        std::fs::write(&path, b"before\n").unwrap();
        std::fs::write(&timestamp, b"reference\n").unwrap();
        assert!(std::process::Command::new("touch")
            .args(["-r"])
            .arg(&path)
            .arg(&timestamp)
            .status()
            .unwrap()
            .success());
        let relative = NormalizedRepoRelativePath::parse("value.yaml").unwrap();
        let file = super::open_repo_relative_regular_file_strict(repo.path(), relative.as_path())
            .expect("strict handle");

        let error = file
            .read_bytes_bounded_stable_with_hook(1024, || {
                std::fs::write(&path, b"after!\n").unwrap();
                assert!(std::process::Command::new("touch")
                    .args(["-r"])
                    .arg(&timestamp)
                    .arg(&path)
                    .status()
                    .unwrap()
                    .success());
            })
            .expect_err("ctime identity must expose restored-mtime mutation");
        assert_eq!(error.kind(), std::io::ErrorKind::Other);
    }

    #[cfg(all(not(unix), not(windows)))]
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
                .read_bytes_bounded(1024)
                .unwrap()
                .0,
            b"legacy\n"
        );
        assert!(workspace.trusted_read_strict(&relative).is_err());
    }
}
