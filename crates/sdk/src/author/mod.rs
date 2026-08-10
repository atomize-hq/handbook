pub mod project_context;
#[cfg(unix)]
mod project_context_shell;

#[cfg(unix)]
use crate::repo_file_access::{
    resolve_repo_relative_write_path, RepoRelativeMutationError, RepoRelativeWritePathError,
};
#[cfg(unix)]
use std::fs::{File, OpenOptions};
#[cfg(unix)]
use std::path::{Path, PathBuf};

#[cfg(unix)]
#[derive(Debug)]
enum AuthoringLockError {
    WritePath(RepoRelativeWritePathError),
    Io {
        lock_path: PathBuf,
        source: std::io::Error,
    },
}

#[cfg(unix)]
fn acquire_authoring_lock(
    repo_root: &Path,
    lock_repo_relative_path: &str,
) -> Result<AuthoringLockGuard, AuthoringLockError> {
    let lock_path = resolve_repo_relative_write_path(repo_root, lock_repo_relative_path)
        .map_err(AuthoringLockError::WritePath)?;

    if let Some(parent) = lock_path.parent() {
        std::fs::create_dir_all(parent).map_err(|source| AuthoringLockError::Io {
            lock_path: lock_path.clone(),
            source,
        })?;
    }

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(&lock_path)
        .map_err(|source| AuthoringLockError::Io {
            lock_path: lock_path.clone(),
            source,
        })?;

    let lock_operation = libc::LOCK_EX;

    lock_authoring_file(&file, lock_operation).map_err(|source| AuthoringLockError::Io {
        lock_path: lock_path.clone(),
        source,
    })?;

    Ok(AuthoringLockGuard { file, lock_path })
}

#[cfg(unix)]
fn lock_authoring_file(file: &File, operation: libc::c_int) -> Result<(), std::io::Error> {
    use std::os::unix::io::AsRawFd;

    loop {
        let result = unsafe { libc::flock(file.as_raw_fd(), operation) };
        if result == 0 {
            return Ok(());
        }

        let error = std::io::Error::last_os_error();
        if error.kind() == std::io::ErrorKind::Interrupted {
            continue;
        }

        return Err(error);
    }
}

#[cfg(unix)]
struct AuthoringLockGuard {
    file: File,
    lock_path: PathBuf,
}

#[cfg(unix)]
impl Drop for AuthoringLockGuard {
    fn drop(&mut self) {
        let _ = lock_authoring_file(&self.file, libc::LOCK_UN);
        let _ = &self.lock_path;
    }
}

#[cfg(unix)]
fn format_repo_mutation_error(path: &str, err: RepoRelativeMutationError) -> String {
    match err {
        RepoRelativeMutationError::InvalidPath(reason) => {
            format!("write target `{path}` is invalid: {reason}")
        }
        RepoRelativeMutationError::ParentNotDirectory(found) => {
            format!(
                "write target `{path}` cannot be written because {} is not a directory",
                found.display()
            )
        }
        RepoRelativeMutationError::NotRegularFile(found) => {
            format!(
                "write target `{path}` cannot be written because {} is not a regular file target",
                found.display()
            )
        }
        RepoRelativeMutationError::SymlinkNotAllowed(found) => {
            format!(
                "write target `{path}` cannot be written through symlink {}",
                found.display()
            )
        }
        RepoRelativeMutationError::ReadFailure {
            path: found,
            source,
        }
        | RepoRelativeMutationError::WriteFailure {
            path: found,
            source,
        } => {
            format!(
                "failed to mutate write target `{path}` at {}: {source}",
                found.display()
            )
        }
    }
}

#[cfg(unix)]
fn format_repo_write_path_error(path: &str, err: RepoRelativeWritePathError) -> String {
    match err {
        RepoRelativeWritePathError::InvalidPath(reason) => {
            format!("write target `{path}` is invalid: {reason}")
        }
        RepoRelativeWritePathError::ParentNotDirectory(found) => {
            format!(
                "write target `{path}` cannot be written because {} is not a directory",
                found.display()
            )
        }
        RepoRelativeWritePathError::NotRegularFile(found) => {
            format!(
                "write target `{path}` cannot be written because {} is not a regular file target",
                found.display()
            )
        }
        RepoRelativeWritePathError::SymlinkNotAllowed(found) => {
            format!(
                "write target `{path}` cannot be written through symlink {}",
                found.display()
            )
        }
        RepoRelativeWritePathError::ReadFailure {
            path: found,
            source,
        } => {
            format!(
                "failed to inspect write target `{path}` at {}: {source}",
                found.display()
            )
        }
    }
}
