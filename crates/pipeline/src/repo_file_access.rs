use sha2::{Digest, Sha256};
use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
use std::path::{Component, Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

static TEMP_FILE_COUNTER: AtomicU64 = AtomicU64::new(0);

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

#[derive(Debug)]
pub(crate) enum RepoRelativeWritePathError {
    InvalidPath(String),
    ParentNotDirectory(PathBuf),
    NotRegularFile(PathBuf),
    SymlinkNotAllowed(PathBuf),
    ReadFailure {
        path: PathBuf,
        source: std::io::Error,
    },
}

#[derive(Debug)]
pub(crate) enum RepoRelativeMutationError {
    InvalidPath(String),
    ParentNotDirectory(PathBuf),
    NotRegularFile(PathBuf),
    SymlinkNotAllowed(PathBuf),
    ReadFailure {
        path: PathBuf,
        source: std::io::Error,
    },
    WriteFailure {
        path: PathBuf,
        source: std::io::Error,
    },
}

#[derive(Debug)]
pub(crate) enum RepoRelativeDirectoryAccessError {
    Missing(PathBuf),
    SymlinkNotAllowed(PathBuf),
    NotDirectory(PathBuf),
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

    pub(crate) fn parse_path(path: &Path) -> Result<Self, String> {
        let path = validate_repo_relative_path_from_path(path)?;
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
pub(crate) struct CompilerWorkspace<'a> {
    repo_root: &'a Path,
}

impl<'a> CompilerWorkspace<'a> {
    pub(crate) fn new(repo_root: &'a Path) -> Self {
        Self { repo_root }
    }

    pub(crate) fn normalize_repo_relative(
        &self,
        path: &str,
    ) -> Result<NormalizedRepoRelativePath, String> {
        NormalizedRepoRelativePath::parse(path)
    }

    pub(crate) fn normalize_repo_relative_path(
        &self,
        path: &Path,
    ) -> Result<NormalizedRepoRelativePath, String> {
        NormalizedRepoRelativePath::parse_path(path)
    }

    pub(crate) fn trusted_read(
        &self,
        relative_path: &NormalizedRepoRelativePath,
    ) -> Result<TrustedRepoFile, RepoRelativeFileAccessError> {
        let absolute_path =
            resolve_repo_relative_regular_file_path(self.repo_root, relative_path.as_path())?;
        Ok(TrustedRepoFile {
            repo_relative: relative_path.clone(),
            absolute_path,
        })
    }

    pub(crate) fn trusted_directory(
        &self,
        relative_path: &NormalizedRepoRelativePath,
    ) -> Result<TrustedRepoDirectory, RepoRelativeDirectoryAccessError> {
        let absolute_path =
            resolve_repo_relative_directory_path(self.repo_root, relative_path.as_path())?;
        Ok(TrustedRepoDirectory { absolute_path })
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

    pub(crate) fn read_string(
        &self,
        relative_path: &NormalizedRepoRelativePath,
    ) -> Result<String, RepoRelativeFileAccessError> {
        let trusted_file = self.trusted_read(relative_path)?;
        trusted_file
            .read_string()
            .map_err(|source| RepoRelativeFileAccessError::ReadFailure {
                path: trusted_file.absolute_path.clone(),
                source,
            })
    }

    pub(crate) fn sha256_file(
        &self,
        relative_path: &NormalizedRepoRelativePath,
    ) -> Result<String, RepoRelativeFileAccessError> {
        let trusted_file = self.trusted_read(relative_path)?;
        trusted_file
            .sha256_hex()
            .map_err(|source| RepoRelativeFileAccessError::ReadFailure {
                path: trusted_file.absolute_path.clone(),
                source,
            })
    }

    pub(crate) fn resolve_write_path(
        &self,
        relative_path: &NormalizedRepoRelativePath,
    ) -> Result<PathBuf, RepoRelativeWritePathError> {
        resolve_repo_relative_write_path_from_validated(self.repo_root, relative_path.as_path())
    }

    pub(crate) fn write_bytes(
        &self,
        relative_path: &NormalizedRepoRelativePath,
        bytes: &[u8],
    ) -> Result<(), RepoRelativeMutationError> {
        write_repo_relative_bytes_from_validated(self.repo_root, relative_path.as_path(), bytes)
    }

    pub(crate) fn delete_file(
        &self,
        relative_path: &NormalizedRepoRelativePath,
    ) -> Result<(), RepoRelativeMutationError> {
        delete_repo_relative_file_from_validated(self.repo_root, relative_path.as_path())
    }

    fn absolute_path(&self, relative_path: &NormalizedRepoRelativePath) -> PathBuf {
        self.repo_root.join(relative_path.as_path())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct TrustedRepoDirectory {
    absolute_path: PathBuf,
}

impl TrustedRepoDirectory {
    pub(crate) fn absolute_path(&self) -> &Path {
        &self.absolute_path
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct TrustedRepoFile {
    repo_relative: NormalizedRepoRelativePath,
    absolute_path: PathBuf,
}

impl TrustedRepoFile {
    #[cfg(test)]
    pub(crate) fn repo_relative(&self) -> &NormalizedRepoRelativePath {
        &self.repo_relative
    }

    pub(crate) fn absolute_path(&self) -> &Path {
        &self.absolute_path
    }

    pub(crate) fn read_string(&self) -> Result<String, std::io::Error> {
        read_string_no_follow_path(&self.absolute_path)
    }

    pub(crate) fn read_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        read_bytes_no_follow_path(&self.absolute_path)
    }

    pub(crate) fn sha256_hex(&self) -> Result<String, std::io::Error> {
        let mut hasher = Sha256::new();
        hasher.update(self.read_bytes()?);
        Ok(format!("{:x}", hasher.finalize()))
    }
}

pub(crate) fn read_repo_relative_string(
    repo_root: &Path,
    relative_path: &str,
) -> Result<String, RepoRelativeFileAccessError> {
    let workspace = CompilerWorkspace::new(repo_root);
    let relative_path = workspace
        .normalize_repo_relative(relative_path)
        .map_err(RepoRelativeFileAccessError::InvalidPath)?;
    workspace.read_string(&relative_path)
}

pub(crate) fn sha256_repo_relative_file(
    repo_root: &Path,
    relative_path: &str,
) -> Result<String, RepoRelativeFileAccessError> {
    let workspace = CompilerWorkspace::new(repo_root);
    let relative_path = workspace
        .normalize_repo_relative(relative_path)
        .map_err(RepoRelativeFileAccessError::InvalidPath)?;
    workspace.sha256_file(&relative_path)
}

pub(crate) fn resolve_repo_relative_write_path(
    repo_root: &Path,
    relative_path: &str,
) -> Result<PathBuf, RepoRelativeWritePathError> {
    let workspace = CompilerWorkspace::new(repo_root);
    let relative_path = workspace
        .normalize_repo_relative(relative_path)
        .map_err(RepoRelativeWritePathError::InvalidPath)?;
    workspace.resolve_write_path(&relative_path)
}

fn resolve_repo_relative_write_path_from_validated(
    repo_root: &Path,
    relative_path: &Path,
) -> Result<PathBuf, RepoRelativeWritePathError> {
    let mut current = repo_root.to_path_buf();
    let mut components = relative_path.components().peekable();

    while let Some(component) = components.next() {
        let Component::Normal(part) = component else {
            continue;
        };
        current.push(part);
        let is_last = components.peek().is_none();

        match fs::symlink_metadata(&current) {
            Ok(metadata) => {
                if metadata.file_type().is_symlink() {
                    return Err(RepoRelativeWritePathError::SymlinkNotAllowed(
                        current.clone(),
                    ));
                }

                if is_last {
                    if metadata.is_dir() {
                        return Err(RepoRelativeWritePathError::NotRegularFile(current.clone()));
                    }
                    if !metadata.is_file() {
                        return Err(RepoRelativeWritePathError::NotRegularFile(current.clone()));
                    }
                } else if !metadata.is_dir() {
                    return Err(RepoRelativeWritePathError::ParentNotDirectory(
                        current.clone(),
                    ));
                }
            }
            Err(source) if source.kind() == std::io::ErrorKind::NotFound => {
                continue;
            }
            Err(source) => {
                return Err(RepoRelativeWritePathError::ReadFailure {
                    path: current.clone(),
                    source,
                });
            }
        }
    }

    Ok(current)
}

pub(crate) fn read_bytes_no_follow_path(path: &Path) -> Result<Vec<u8>, std::io::Error> {
    read_bytes_no_follow(path)
}

pub(crate) fn read_string_no_follow_path(path: &Path) -> Result<String, std::io::Error> {
    read_string_no_follow(path)
}

pub(crate) fn write_repo_relative_bytes(
    repo_root: &Path,
    relative_path: &str,
    bytes: &[u8],
) -> Result<(), RepoRelativeMutationError> {
    let workspace = CompilerWorkspace::new(repo_root);
    let relative_path = workspace
        .normalize_repo_relative(relative_path)
        .map_err(RepoRelativeMutationError::InvalidPath)?;
    workspace.write_bytes(&relative_path, bytes)
}

fn write_repo_relative_bytes_from_validated(
    repo_root: &Path,
    relative_path: &Path,
    bytes: &[u8],
) -> Result<(), RepoRelativeMutationError> {
    #[cfg(unix)]
    {
        write_repo_relative_bytes_unix(repo_root, relative_path, bytes)
    }

    #[cfg(not(unix))]
    {
        write_repo_relative_bytes_fallback(repo_root, relative_path, bytes)
    }
}

pub(crate) fn delete_repo_relative_file(
    repo_root: &Path,
    relative_path: &str,
) -> Result<(), RepoRelativeMutationError> {
    let workspace = CompilerWorkspace::new(repo_root);
    let relative_path = workspace
        .normalize_repo_relative(relative_path)
        .map_err(RepoRelativeMutationError::InvalidPath)?;
    workspace.delete_file(&relative_path)
}

fn delete_repo_relative_file_from_validated(
    repo_root: &Path,
    relative_path: &Path,
) -> Result<(), RepoRelativeMutationError> {
    #[cfg(unix)]
    {
        delete_repo_relative_file_unix(repo_root, relative_path)
    }

    #[cfg(not(unix))]
    {
        delete_repo_relative_file_fallback(repo_root, relative_path)
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

fn resolve_repo_relative_directory_path(
    repo_root: &Path,
    relative_path: &Path,
) -> Result<PathBuf, RepoRelativeDirectoryAccessError> {
    let mut current = repo_root.to_path_buf();

    for component in relative_path.components() {
        let Component::Normal(part) = component else {
            continue;
        };
        current.push(part);

        let metadata = match fs::symlink_metadata(&current) {
            Ok(metadata) => metadata,
            Err(source) if source.kind() == std::io::ErrorKind::NotFound => {
                return Err(RepoRelativeDirectoryAccessError::Missing(current.clone()))
            }
            Err(source) => {
                return Err(RepoRelativeDirectoryAccessError::ReadFailure {
                    path: current.clone(),
                    source,
                })
            }
        };

        if metadata.file_type().is_symlink() {
            return Err(RepoRelativeDirectoryAccessError::SymlinkNotAllowed(
                current.clone(),
            ));
        }

        if !metadata.is_dir() {
            return Err(RepoRelativeDirectoryAccessError::NotDirectory(
                current.clone(),
            ));
        }
    }

    Ok(current)
}

pub(crate) fn validate_repo_relative_path(path: &str) -> Result<&Path, String> {
    let trimmed = path.trim();
    validate_repo_relative_path_from_path(Path::new(trimmed))
}

pub(crate) fn validate_repo_relative_path_from_path(path: &Path) -> Result<&Path, String> {
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

pub(crate) fn normalize_repo_relative_path(path: &str) -> Result<String, String> {
    NormalizedRepoRelativePath::parse(path).map(|path| path.0)
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

#[cfg(unix)]
fn write_repo_relative_bytes_unix(
    repo_root: &Path,
    relative_path: &Path,
    bytes: &[u8],
) -> Result<(), RepoRelativeMutationError> {
    use std::ffi::{CString, OsStr};
    use std::fs::File;
    use std::os::fd::{AsRawFd, FromRawFd, IntoRawFd, OwnedFd};
    use std::os::unix::ffi::OsStrExt;

    let components = normal_components(relative_path);
    let (target_name, parent_parts) = components.split_last().ok_or_else(|| {
        RepoRelativeMutationError::InvalidPath("path must not be empty".to_string())
    })?;
    let mut parent_fd = open_directory_path(repo_root).map_err(|source| {
        RepoRelativeMutationError::WriteFailure {
            path: repo_root.to_path_buf(),
            source,
        }
    })?;
    let mut parent_path = repo_root.to_path_buf();

    for part in parent_parts {
        parent_path.push(part);
        parent_fd = open_or_create_directory_at(parent_fd.as_raw_fd(), part, &parent_path)
            .map_err(|source| classify_dir_traversal_error(source, &parent_path))?;
    }

    let target_path = parent_path.join(target_name);
    reject_non_regular_target(parent_fd.as_raw_fd(), target_name, &target_path)?;

    let (temp_name, temp_path) = next_temp_name(target_name, &parent_path)?;
    let temp_fd = open_temp_file_at(parent_fd.as_raw_fd(), &temp_name).map_err(|source| {
        RepoRelativeMutationError::WriteFailure {
            path: temp_path.clone(),
            source,
        }
    })?;
    let mut temp_file = unsafe { File::from_raw_fd(temp_fd.into_raw_fd()) };

    let result = (|| -> Result<(), RepoRelativeMutationError> {
        temp_file
            .write_all(bytes)
            .map_err(|source| RepoRelativeMutationError::WriteFailure {
                path: temp_path.clone(),
                source,
            })?;
        temp_file
            .sync_all()
            .map_err(|source| RepoRelativeMutationError::WriteFailure {
                path: temp_path.clone(),
                source,
            })?;
        drop(temp_file);

        rename_at(parent_fd.as_raw_fd(), &temp_name, target_name).map_err(|source| {
            RepoRelativeMutationError::WriteFailure {
                path: target_path.clone(),
                source,
            }
        })?;
        sync_directory_fd(parent_fd.as_raw_fd()).map_err(|source| {
            RepoRelativeMutationError::WriteFailure {
                path: parent_path.clone(),
                source,
            }
        })?;
        Ok(())
    })();

    if result.is_err() {
        let _ = unlink_at(parent_fd.as_raw_fd(), &temp_name);
    }

    fn normal_components(path: &Path) -> Vec<&OsStr> {
        path.components()
            .filter_map(|component| match component {
                Component::Normal(part) => Some(part),
                _ => None,
            })
            .collect()
    }

    fn classify_dir_traversal_error(
        source: std::io::Error,
        path: &Path,
    ) -> RepoRelativeMutationError {
        if matches!(fs::symlink_metadata(path), Ok(metadata) if metadata.file_type().is_symlink()) {
            return RepoRelativeMutationError::SymlinkNotAllowed(path.to_path_buf());
        }
        match source.raw_os_error() {
            Some(code) if code == libc::ELOOP => {
                RepoRelativeMutationError::SymlinkNotAllowed(path.to_path_buf())
            }
            Some(code) if code == libc::ENOTDIR => {
                RepoRelativeMutationError::ParentNotDirectory(path.to_path_buf())
            }
            _ => RepoRelativeMutationError::ReadFailure {
                path: path.to_path_buf(),
                source,
            },
        }
    }

    fn reject_non_regular_target(
        parent_fd: libc::c_int,
        target_name: &OsStr,
        target_path: &Path,
    ) -> Result<(), RepoRelativeMutationError> {
        match lstat_at(parent_fd, target_name) {
            Ok(Some(stat)) => {
                let mode = stat.st_mode & libc::S_IFMT;
                if mode == libc::S_IFLNK {
                    Err(RepoRelativeMutationError::SymlinkNotAllowed(
                        target_path.to_path_buf(),
                    ))
                } else if mode != libc::S_IFREG {
                    Err(RepoRelativeMutationError::NotRegularFile(
                        target_path.to_path_buf(),
                    ))
                } else {
                    Ok(())
                }
            }
            Ok(None) => Ok(()),
            Err(source) => Err(RepoRelativeMutationError::ReadFailure {
                path: target_path.to_path_buf(),
                source,
            }),
        }
    }

    fn next_temp_name(
        target_name: &OsStr,
        parent_path: &Path,
    ) -> Result<(CString, PathBuf), RepoRelativeMutationError> {
        let counter = TEMP_FILE_COUNTER.fetch_add(1, Ordering::Relaxed);
        let pid = std::process::id();
        let temp = format!(".{}.tmp-{pid}-{counter}", target_name.to_string_lossy());
        let c_temp = CString::new(temp.clone()).map_err(|_| {
            RepoRelativeMutationError::InvalidPath("path must not contain NUL bytes".to_string())
        })?;
        Ok((c_temp, parent_path.join(temp)))
    }

    fn open_directory_path(path: &Path) -> std::io::Result<OwnedFd> {
        let c_path = std::ffi::CString::new(path.as_os_str().as_bytes())
            .map_err(|_| std::io::Error::from_raw_os_error(libc::EINVAL))?;
        open_fd(
            c_path.as_ptr(),
            libc::O_RDONLY | libc::O_DIRECTORY | libc::O_NOFOLLOW,
        )
    }

    fn open_or_create_directory_at(
        parent_fd: libc::c_int,
        component: &OsStr,
        full_path: &Path,
    ) -> std::io::Result<OwnedFd> {
        let c_component = std::ffi::CString::new(component.as_bytes())
            .map_err(|_| std::io::Error::from_raw_os_error(libc::EINVAL))?;
        match openat_fd(
            parent_fd,
            c_component.as_ptr(),
            libc::O_RDONLY | libc::O_DIRECTORY | libc::O_NOFOLLOW,
            None,
        ) {
            Ok(fd) => Ok(fd),
            Err(source) if source.kind() == std::io::ErrorKind::NotFound => {
                mkdir_at(parent_fd, c_component.as_ptr(), 0o755)?;
                openat_fd(
                    parent_fd,
                    c_component.as_ptr(),
                    libc::O_RDONLY | libc::O_DIRECTORY | libc::O_NOFOLLOW,
                    None,
                )
            }
            Err(source) => Err(adjust_dir_open_error(source, full_path)),
        }
    }

    fn adjust_dir_open_error(source: std::io::Error, full_path: &Path) -> std::io::Error {
        let _ = full_path;
        source
    }

    fn open_temp_file_at(parent_fd: libc::c_int, temp_name: &CString) -> std::io::Result<OwnedFd> {
        openat_fd(
            parent_fd,
            temp_name.as_ptr(),
            libc::O_RDWR | libc::O_CREAT | libc::O_EXCL | libc::O_NOFOLLOW,
            Some(0o600),
        )
    }

    fn rename_at(parent_fd: libc::c_int, from: &CString, to: &OsStr) -> std::io::Result<()> {
        let c_to = std::ffi::CString::new(to.as_bytes())
            .map_err(|_| std::io::Error::from_raw_os_error(libc::EINVAL))?;
        retry_on_eintr(|| unsafe {
            libc::renameat(parent_fd, from.as_ptr(), parent_fd, c_to.as_ptr())
        })
        .map(|_| ())
    }

    fn unlink_at(parent_fd: libc::c_int, target_name: &CString) -> std::io::Result<()> {
        retry_on_eintr(|| unsafe { libc::unlinkat(parent_fd, target_name.as_ptr(), 0) }).map(|_| ())
    }

    fn sync_directory_fd(fd: libc::c_int) -> std::io::Result<()> {
        retry_on_eintr(|| unsafe { libc::fsync(fd) }).map(|_| ())
    }

    fn lstat_at(
        parent_fd: libc::c_int,
        target_name: &OsStr,
    ) -> std::io::Result<Option<libc::stat>> {
        let c_target = std::ffi::CString::new(target_name.as_bytes())
            .map_err(|_| std::io::Error::from_raw_os_error(libc::EINVAL))?;
        let mut stat = std::mem::MaybeUninit::<libc::stat>::uninit();
        match retry_on_eintr(|| unsafe {
            libc::fstatat(
                parent_fd,
                c_target.as_ptr(),
                stat.as_mut_ptr(),
                libc::AT_SYMLINK_NOFOLLOW,
            )
        }) {
            Ok(_) => Ok(Some(unsafe { stat.assume_init() })),
            Err(source) if source.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(source) => Err(source),
        }
    }

    fn mkdir_at(
        parent_fd: libc::c_int,
        path: *const libc::c_char,
        mode: libc::mode_t,
    ) -> std::io::Result<()> {
        match retry_on_eintr(|| unsafe { libc::mkdirat(parent_fd, path, mode) }) {
            Ok(_) => Ok(()),
            Err(source) if source.raw_os_error() == Some(libc::EEXIST) => Ok(()),
            Err(source) => Err(source),
        }
    }

    fn open_fd(path: *const libc::c_char, flags: libc::c_int) -> std::io::Result<OwnedFd> {
        let fd = retry_on_eintr(|| unsafe { libc::open(path, flags | libc::O_CLOEXEC) })?;
        Ok(unsafe { OwnedFd::from_raw_fd(fd) })
    }

    fn openat_fd(
        parent_fd: libc::c_int,
        path: *const libc::c_char,
        flags: libc::c_int,
        mode: Option<libc::mode_t>,
    ) -> std::io::Result<OwnedFd> {
        let fd = match mode {
            Some(mode) => retry_on_eintr(|| unsafe {
                libc::openat(
                    parent_fd,
                    path,
                    flags | libc::O_CLOEXEC,
                    mode as libc::c_uint,
                )
            })?,
            None => retry_on_eintr(|| unsafe {
                libc::openat(parent_fd, path, flags | libc::O_CLOEXEC)
            })?,
        };
        Ok(unsafe { OwnedFd::from_raw_fd(fd) })
    }

    fn retry_on_eintr(mut operation: impl FnMut() -> libc::c_int) -> std::io::Result<libc::c_int> {
        loop {
            let result = operation();
            if result >= 0 {
                return Ok(result);
            }
            let source = std::io::Error::last_os_error();
            if source.kind() == std::io::ErrorKind::Interrupted {
                continue;
            }
            return Err(source);
        }
    }

    result
}

#[cfg(unix)]
fn delete_repo_relative_file_unix(
    repo_root: &Path,
    relative_path: &Path,
) -> Result<(), RepoRelativeMutationError> {
    use std::ffi::{CString, OsStr};
    use std::os::fd::{AsRawFd, FromRawFd, OwnedFd};
    use std::os::unix::ffi::OsStrExt;

    let components: Vec<&OsStr> = relative_path
        .components()
        .filter_map(|component| match component {
            Component::Normal(part) => Some(part),
            _ => None,
        })
        .collect();
    let (target_name, parent_parts) = components.split_last().ok_or_else(|| {
        RepoRelativeMutationError::InvalidPath("path must not be empty".to_string())
    })?;
    let c_target = CString::new(target_name.as_bytes()).map_err(|_| {
        RepoRelativeMutationError::InvalidPath("path must not contain NUL bytes".to_string())
    })?;
    let mut parent_fd = {
        let c_path = CString::new(repo_root.as_os_str().as_bytes()).map_err(|_| {
            RepoRelativeMutationError::InvalidPath("path must not contain NUL bytes".to_string())
        })?;
        let fd = retry_on_eintr(|| unsafe {
            libc::open(
                c_path.as_ptr(),
                libc::O_RDONLY | libc::O_DIRECTORY | libc::O_NOFOLLOW | libc::O_CLOEXEC,
            )
        })
        .map_err(|source| RepoRelativeMutationError::WriteFailure {
            path: repo_root.to_path_buf(),
            source,
        })?;
        unsafe { OwnedFd::from_raw_fd(fd) }
    };
    let mut parent_path = repo_root.to_path_buf();

    for part in parent_parts {
        parent_path.push(part);
        let c_part = CString::new(part.as_bytes()).map_err(|_| {
            RepoRelativeMutationError::InvalidPath("path must not contain NUL bytes".to_string())
        })?;
        let next_fd = match retry_on_eintr(|| unsafe {
            libc::openat(
                parent_fd.as_raw_fd(),
                c_part.as_ptr(),
                libc::O_RDONLY | libc::O_DIRECTORY | libc::O_NOFOLLOW | libc::O_CLOEXEC,
            )
        }) {
            Ok(fd) => unsafe { OwnedFd::from_raw_fd(fd) },
            Err(source) if source.kind() == std::io::ErrorKind::NotFound => return Ok(()),
            Err(_source) if matches!(fs::symlink_metadata(&parent_path), Ok(metadata) if metadata.file_type().is_symlink()) => {
                return Err(RepoRelativeMutationError::SymlinkNotAllowed(parent_path))
            }
            Err(source) if source.raw_os_error() == Some(libc::ELOOP) => {
                return Err(RepoRelativeMutationError::SymlinkNotAllowed(parent_path))
            }
            Err(source) if source.raw_os_error() == Some(libc::ENOTDIR) => {
                return Err(RepoRelativeMutationError::ParentNotDirectory(parent_path))
            }
            Err(source) => {
                return Err(RepoRelativeMutationError::ReadFailure {
                    path: parent_path,
                    source,
                })
            }
        };
        parent_fd = next_fd;
    }

    let target_path = parent_path.join(target_name);
    let mut stat = std::mem::MaybeUninit::<libc::stat>::uninit();
    match retry_on_eintr(|| unsafe {
        libc::fstatat(
            parent_fd.as_raw_fd(),
            c_target.as_ptr(),
            stat.as_mut_ptr(),
            libc::AT_SYMLINK_NOFOLLOW,
        )
    }) {
        Ok(_) => {
            let stat = unsafe { stat.assume_init() };
            let mode = stat.st_mode & libc::S_IFMT;
            if mode == libc::S_IFLNK {
                return Err(RepoRelativeMutationError::SymlinkNotAllowed(target_path));
            }
            if mode != libc::S_IFREG {
                return Err(RepoRelativeMutationError::NotRegularFile(target_path));
            }
        }
        Err(source) if source.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(source) => {
            return Err(RepoRelativeMutationError::ReadFailure {
                path: target_path,
                source,
            })
        }
    }

    match retry_on_eintr(|| unsafe { libc::unlinkat(parent_fd.as_raw_fd(), c_target.as_ptr(), 0) })
    {
        Ok(_) => {}
        Err(source) if source.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(source) => {
            return Err(RepoRelativeMutationError::WriteFailure {
                path: target_path,
                source,
            })
        }
    }
    retry_on_eintr(|| unsafe { libc::fsync(parent_fd.as_raw_fd()) }).map_err(|source| {
        RepoRelativeMutationError::WriteFailure {
            path: parent_path,
            source,
        }
    })?;

    fn retry_on_eintr(mut operation: impl FnMut() -> libc::c_int) -> std::io::Result<libc::c_int> {
        loop {
            let result = operation();
            if result >= 0 {
                return Ok(result);
            }
            let source = std::io::Error::last_os_error();
            if source.kind() == std::io::ErrorKind::Interrupted {
                continue;
            }
            return Err(source);
        }
    }

    Ok(())
}

#[cfg(not(unix))]
fn write_repo_relative_bytes_fallback(
    repo_root: &Path,
    relative_path: &Path,
    bytes: &[u8],
) -> Result<(), RepoRelativeMutationError> {
    let path = resolve_repo_relative_write_path_from_validated(repo_root, relative_path)
        .map_err(map_write_path_error_to_mutation_error)?;
    let parent = path.parent().ok_or_else(|| {
        RepoRelativeMutationError::InvalidPath("path must have a parent directory".to_string())
    })?;
    fs::create_dir_all(parent).map_err(|source| RepoRelativeMutationError::WriteFailure {
        path: parent.to_path_buf(),
        source,
    })?;
    fs::write(&path, bytes)
        .map_err(|source| RepoRelativeMutationError::WriteFailure { path, source })?;
    Ok(())
}

#[cfg(not(unix))]
fn delete_repo_relative_file_fallback(
    repo_root: &Path,
    relative_path: &Path,
) -> Result<(), RepoRelativeMutationError> {
    let path = resolve_repo_relative_write_path_from_validated(repo_root, relative_path)
        .map_err(map_write_path_error_to_mutation_error)?;
    match fs::remove_file(&path) {
        Ok(()) => Ok(()),
        Err(source) if source.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(source) => Err(RepoRelativeMutationError::WriteFailure { path, source }),
    }
}

#[cfg(not(unix))]
fn map_write_path_error_to_mutation_error(
    err: RepoRelativeWritePathError,
) -> RepoRelativeMutationError {
    match err {
        RepoRelativeWritePathError::InvalidPath(reason) => {
            RepoRelativeMutationError::InvalidPath(reason)
        }
        RepoRelativeWritePathError::ParentNotDirectory(path) => {
            RepoRelativeMutationError::ParentNotDirectory(path)
        }
        RepoRelativeWritePathError::NotRegularFile(path) => {
            RepoRelativeMutationError::NotRegularFile(path)
        }
        RepoRelativeWritePathError::SymlinkNotAllowed(path) => {
            RepoRelativeMutationError::SymlinkNotAllowed(path)
        }
        RepoRelativeWritePathError::ReadFailure { path, source } => {
            RepoRelativeMutationError::ReadFailure { path, source }
        }
    }
}

fn read_string_no_follow(path: &Path) -> Result<String, std::io::Error> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;

        let mut file = OpenOptions::new()
            .read(true)
            .custom_flags(libc::O_NOFOLLOW)
            .open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }

    #[cfg(not(unix))]
    {
        fs::read_to_string(path)
    }
}

pub(crate) fn read_bytes_no_follow(path: &Path) -> Result<Vec<u8>, std::io::Error> {
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

#[cfg(test)]
mod tests {
    use super::{CompilerWorkspace, RepoRelativeFileAccessError, RepoRelativeMutationError};
    use std::fs;

    #[test]
    fn workspace_normalizes_repo_relative_paths() {
        let repo_root = tempfile::tempdir().expect("tempdir");
        let workspace = CompilerWorkspace::new(repo_root.path());

        assert_eq!(
            workspace
                .normalize_repo_relative("./nested/./output.txt")
                .expect("normalize repo-relative path")
                .as_str(),
            "nested/output.txt"
        );
    }

    #[test]
    fn workspace_write_bytes_creates_missing_parent_chain() {
        let repo_root = tempfile::tempdir().expect("tempdir");
        let workspace = CompilerWorkspace::new(repo_root.path());
        let path = workspace
            .normalize_repo_relative("nested/path/output.txt")
            .expect("normalize repo-relative path");

        workspace
            .write_bytes(&path, b"hello\n")
            .expect("write bytes");

        assert_eq!(
            fs::read_to_string(repo_root.path().join("nested/path/output.txt"))
                .expect("read written file"),
            "hello\n"
        );
    }

    #[cfg(unix)]
    #[test]
    fn workspace_trusted_read_refuses_symlink_targets() {
        let repo_root = tempfile::tempdir().expect("tempdir");
        let external_root = tempfile::tempdir().expect("external tempdir");
        let link_path = repo_root.path().join("nested/output.txt");
        fs::create_dir_all(link_path.parent().expect("link parent")).expect("create parent");
        fs::write(external_root.path().join("output.txt"), "outside\n").expect("seed outside");
        std::os::unix::fs::symlink(external_root.path().join("output.txt"), &link_path)
            .expect("symlink file");
        let workspace = CompilerWorkspace::new(repo_root.path());
        let path = workspace
            .normalize_repo_relative("nested/output.txt")
            .expect("normalize repo-relative path");

        let err = workspace.trusted_read(&path).expect_err("symlink refusal");

        assert!(
            matches!(err, RepoRelativeFileAccessError::SymlinkNotAllowed(ref found) if found == &link_path),
            "expected symlink refusal, got: {err:?}"
        );
    }

    #[cfg(unix)]
    #[test]
    fn trusted_repo_file_read_uses_no_follow_after_resolution() {
        let repo_root = tempfile::tempdir().expect("tempdir");
        let external_root = tempfile::tempdir().expect("external tempdir");
        let file_path = repo_root.path().join("nested/output.txt");
        fs::create_dir_all(file_path.parent().expect("file parent")).expect("create parent");
        fs::write(&file_path, "inside\n").expect("seed repo file");
        fs::write(external_root.path().join("output.txt"), "outside\n").expect("seed outside");

        let workspace = CompilerWorkspace::new(repo_root.path());
        let path = workspace
            .normalize_repo_relative("nested/output.txt")
            .expect("normalize repo-relative path");
        let trusted_file = workspace.trusted_read(&path).expect("resolve regular file");

        fs::remove_file(&file_path).expect("remove repo file");
        std::os::unix::fs::symlink(external_root.path().join("output.txt"), &file_path)
            .expect("replace repo file with symlink");

        let err = trusted_file.read_string().expect_err("no-follow refusal");

        assert_eq!(trusted_file.repo_relative().as_str(), "nested/output.txt");
        assert_eq!(trusted_file.absolute_path(), file_path.as_path());
        assert_eq!(
            err.raw_os_error(),
            Some(libc::ELOOP),
            "expected O_NOFOLLOW refusal, got: {err}"
        );
        assert_eq!(
            fs::read_to_string(external_root.path().join("output.txt")).expect("read outside"),
            "outside\n"
        );
    }

    #[cfg(unix)]
    #[test]
    fn workspace_write_bytes_rejects_symlinked_parent_chain() {
        let repo_root = tempfile::tempdir().expect("tempdir");
        let external_root = tempfile::tempdir().expect("external tempdir");
        std::os::unix::fs::symlink(external_root.path(), repo_root.path().join("nested"))
            .expect("symlink parent");
        let workspace = CompilerWorkspace::new(repo_root.path());
        let path = workspace
            .normalize_repo_relative("nested/output.txt")
            .expect("normalize repo-relative path");

        let err = workspace
            .write_bytes(&path, b"hello\n")
            .expect_err("symlink refusal");

        assert!(
            matches!(err, RepoRelativeMutationError::SymlinkNotAllowed(_)),
            "expected symlink refusal, got: {err:?}"
        );
        assert!(
            !external_root.path().join("output.txt").exists(),
            "helper must not follow the symlink outside the repo"
        );
    }

    #[cfg(unix)]
    #[test]
    fn workspace_write_bytes_replaces_target_without_temp_leftovers() {
        let repo_root = tempfile::tempdir().expect("tempdir");
        let parent = repo_root.path().join("nested");
        fs::create_dir_all(&parent).expect("create parent");
        fs::write(parent.join("output.txt"), "before\n").expect("seed target");
        let workspace = CompilerWorkspace::new(repo_root.path());
        let path = workspace
            .normalize_repo_relative("nested/output.txt")
            .expect("normalize repo-relative path");

        workspace
            .write_bytes(&path, b"after\n")
            .expect("overwrite file");

        assert_eq!(
            fs::read_to_string(parent.join("output.txt")).expect("read overwritten file"),
            "after\n"
        );
        let entries = fs::read_dir(&parent)
            .expect("read parent dir")
            .map(|entry| entry.expect("dir entry").file_name())
            .collect::<Vec<_>>();
        assert_eq!(
            entries.len(),
            1,
            "same-directory commit should clean temp files"
        );
        assert_eq!(entries[0].to_string_lossy(), "output.txt");
    }

    #[cfg(unix)]
    #[test]
    fn workspace_delete_file_rejects_symlinked_parent_chain() {
        let repo_root = tempfile::tempdir().expect("tempdir");
        let external_root = tempfile::tempdir().expect("external tempdir");
        fs::write(external_root.path().join("output.txt"), "outside\n").expect("seed outside");
        std::os::unix::fs::symlink(external_root.path(), repo_root.path().join("nested"))
            .expect("symlink parent");
        let workspace = CompilerWorkspace::new(repo_root.path());
        let path = workspace
            .normalize_repo_relative("nested/output.txt")
            .expect("normalize repo-relative path");

        let err = workspace.delete_file(&path).expect_err("symlink refusal");

        assert!(
            matches!(err, RepoRelativeMutationError::SymlinkNotAllowed(_)),
            "expected symlink refusal, got: {err:?}"
        );
        assert_eq!(
            fs::read_to_string(external_root.path().join("output.txt")).expect("read outside"),
            "outside\n"
        );
    }
}
