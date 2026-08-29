//! Retained-root filesystem primitives for repository-owned durable state.
//!
//! Every operation starts from one retained physical repository directory and
//! accepts only a normalized repository-relative UTF-8 path. The module keeps
//! native identities and paths private so callers cannot turn them into
//! ambient authority.

use sha2::{Digest, Sha256};
use std::collections::{BTreeSet, VecDeque};
use std::error::Error;
use std::fmt;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Component, Path, PathBuf};

const MAX_RELATIVE_PATH_BYTES: usize = 4096;
const MAX_RELATIVE_COMPONENTS: usize = 32;
const MAX_COMPONENT_BYTES: usize = 255;
const MAX_STRICT_READ_BYTES: usize = 16 * 1024 * 1024;
const MAX_DIRECTORY_ENTRIES: usize = 4096;
const MAX_RETAINED_TREE_BYTES: u64 = 256 * 1024 * 1024;

/// Closed refusal categories for retained-root durability operations.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RepositoryDurabilityErrorKindV1 {
    InvalidRelativePath,
    UnsafeFilesystem,
    BoundExceeded,
    AlreadyExists,
    NotFound,
    CurrentValueMismatch,
    LockUnavailable,
    DurabilityUncertain,
    UnsupportedPlatform,
}

/// A bounded durability refusal that exposes neither native IDs nor raw paths.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RepositoryDurabilityErrorV1 {
    kind: RepositoryDurabilityErrorKindV1,
    detail: &'static str,
}

impl RepositoryDurabilityErrorV1 {
    pub fn kind(&self) -> RepositoryDurabilityErrorKindV1 {
        self.kind
    }

    pub fn detail(&self) -> &str {
        self.detail
    }
}

impl fmt::Display for RepositoryDurabilityErrorV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.detail)
    }
}

impl Error for RepositoryDurabilityErrorV1 {}

/// Typed result for an exact, bounded regular-file read.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RepositoryReadV1 {
    Absent,
    Found(Vec<u8>),
}

/// Admitted directory-entry kinds. Links and special files are refused.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum RepositoryDirectoryEntryKindV1 {
    RegularFile,
    Directory,
}

/// One bounded directory entry. Only its simple UTF-8 child name is exposed.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct RepositoryDirectoryEntryV1 {
    name: String,
    kind: RepositoryDirectoryEntryKindV1,
}

impl RepositoryDirectoryEntryV1 {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn kind(&self) -> RepositoryDirectoryEntryKindV1 {
        self.kind
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum PhysicalDirectoryIdentity {
    #[cfg(unix)]
    Unix { device: u64, inode: u64 },
    #[cfg(windows)]
    Windows { volume: u64, file_id: [u8; 16] },
}

/// Opaque retained authority over one exact physical repository directory.
pub struct RetainedRepositoryRootV1 {
    root: File,
    identity: PhysicalDirectoryIdentity,
    root_path: PathBuf,
    #[cfg(unix)]
    ancestor_epochs: Vec<RetainedDirectoryEpoch>,
    #[cfg(windows)]
    _ancestor_guards: Vec<File>,
}

#[cfg(unix)]
struct RetainedDirectoryEpoch {
    directory: File,
    identity: PhysicalDirectoryIdentity,
    modified_seconds: i64,
    modified_nanoseconds: i64,
    changed_seconds: i64,
    changed_nanoseconds: i64,
}

struct RetainedTreeDirectory {
    relative: String,
    directory: File,
    snapshot: DirectorySnapshot,
}

struct RetainedTreeFile {
    relative: String,
    file: File,
    snapshot: RegularFileSnapshot,
    content_fingerprint: [u8; 32],
}

pub(crate) struct RetainedRepositoryTreeGuardV1<'root> {
    root: &'root RetainedRepositoryRootV1,
    directories: Vec<RetainedTreeDirectory>,
    files: Vec<RetainedTreeFile>,
}

impl RetainedRepositoryTreeGuardV1<'_> {
    pub(crate) fn validate(&self) -> Result<(), RepositoryDurabilityErrorV1> {
        self.root.require_stable_path_epoch()?;
        for guard in &self.directories {
            let retained_now = directory_snapshot(&guard.directory)?;
            let relative = ValidatedRelativePath::parse(&guard.relative, false)?;
            let current = platform::open_retained_directory(self.root, &relative)?;
            let current_now = directory_snapshot(&current)?;
            let owner_mutable = guard.relative == ".handbook/state/contracts"
                || guard.relative.starts_with(".handbook/state/contracts/");
            if retained_now.identity != guard.snapshot.identity
                || current_now.identity != guard.snapshot.identity
                || (!owner_mutable
                    && (retained_now.version != guard.snapshot.version
                        || current_now.version != guard.snapshot.version))
            {
                return Err(unsafe_fs(
                    "retained repository state tree changed during authority use",
                ));
            }
        }
        for guard in &self.files {
            let retained_now = regular_file_snapshot(&guard.file)?;
            let relative = ValidatedRelativePath::parse(&guard.relative, false)?;
            let current = self
                .root
                .open_regular_optional(&relative, false)?
                .ok_or_else(|| unsafe_fs("retained state file disappeared during authority use"))?;
            let current_now = regular_file_snapshot(&current)?;
            if retained_now != guard.snapshot || current_now != guard.snapshot {
                return Err(unsafe_fs(
                    "retained state file identity or version changed during authority use",
                ));
            }
            if !guard.relative.ends_with(".lock")
                && (retained_file_fingerprint(&guard.file, guard.snapshot.length)?
                    != guard.content_fingerprint
                    || retained_file_fingerprint(&current, guard.snapshot.length)?
                        != guard.content_fingerprint)
            {
                return Err(unsafe_fs(
                    "retained state file content changed during authority use",
                ));
            }
        }
        Ok(())
    }

    pub(crate) fn with_stable_path<R>(
        &self,
        use_path: impl FnOnce(&Path) -> R,
    ) -> Result<R, RepositoryDurabilityErrorV1> {
        self.validate()?;
        let result = use_path(&self.root.root_path);
        self.validate()?;
        Ok(result)
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct DirectorySnapshot {
    identity: PhysicalDirectoryIdentity,
    version: NativeFileVersion,
}

/// A process/native exclusive lock held until this guard is dropped.
pub struct RepositoryExclusiveLockV1 {
    file: Option<File>,
    #[cfg(windows)]
    _parent_guards: Vec<File>,
}

impl Drop for RepositoryExclusiveLockV1 {
    fn drop(&mut self) {
        if let Some(file) = self.file.take() {
            let _ = file.unlock();
        }
    }
}

impl RepositoryExclusiveLockV1 {
    /// Explicitly releases the native lock so a caller can refuse its receipt
    /// when release certainty is required. Drop remains a best-effort fallback.
    pub fn release(mut self) -> Result<(), RepositoryDurabilityErrorV1> {
        let file = self
            .file
            .take()
            .ok_or_else(|| uncertain("repository lock was already released"))?;
        file.unlock()
            .map_err(|_| uncertain("exclusive repository lock release was uncertain"))
    }
}

impl RetainedRepositoryRootV1 {
    /// Opens an absolute, lexically normalized, link-free repository root and
    /// retains its native physical identity for the lifetime of this guard.
    pub fn open_exact(repo_root: impl AsRef<Path>) -> Result<Self, RepositoryDurabilityErrorV1> {
        ensure_supported_platform()?;
        let root_path = validate_root_path(repo_root.as_ref())?;

        #[cfg(unix)]
        {
            use rustix::fs::{open, Mode, OFlags};
            let root = open(
                &root_path,
                OFlags::RDONLY | OFlags::CLOEXEC | OFlags::DIRECTORY | OFlags::NOFOLLOW,
                Mode::empty(),
            )
            .map_err(|_| unsafe_fs("repository root no-follow open failed"))?;
            let root = File::from(root);
            let identity = directory_identity(&root)?;
            let ancestor_epochs = retain_unix_ancestor_epochs(&root_path)?;
            Ok(Self {
                root,
                identity,
                root_path,
                ancestor_epochs,
            })
        }

        #[cfg(windows)]
        {
            let mut guards = pin_windows_directory_chain(&root_path)?;
            let root = guards
                .pop()
                .ok_or_else(|| unsafe_fs("repository root could not be retained"))?;
            let identity = directory_identity(&root)?;
            Ok(Self {
                root,
                identity,
                root_path,
                _ancestor_guards: guards,
            })
        }

        #[cfg(all(not(unix), not(windows)))]
        unreachable!()
    }

    pub(crate) fn same_physical_root(&self, other: &Self) -> bool {
        self.identity == other.identity
            && directory_identity(&self.root).is_ok_and(|identity| identity == self.identity)
            && directory_identity(&other.root).is_ok_and(|identity| identity == other.identity)
    }

    pub(crate) fn opaque_physical_root_fingerprint(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(b"handbook.snapshot-physical-repository-root@1.0\0");
        match self.identity {
            #[cfg(unix)]
            PhysicalDirectoryIdentity::Unix { device, inode } => {
                hasher.update(b"unix\0");
                hasher.update(device.to_be_bytes());
                hasher.update(inode.to_be_bytes());
            }
            #[cfg(windows)]
            PhysicalDirectoryIdentity::Windows { volume, file_id } => {
                hasher.update(b"windows\0");
                hasher.update(volume.to_be_bytes());
                hasher.update(file_id);
            }
        }
        format!("sha256:{:x}", hasher.finalize())
    }

    pub(crate) fn path_still_names_same_physical_root(&self, path: &Path) -> bool {
        Self::open_exact(path).is_ok_and(|current| self.same_physical_root(&current))
    }

    #[cfg(test)]
    pub(crate) fn retain_tree_guard(
        &self,
        relative_root: &str,
        directory_limit: usize,
    ) -> Result<RetainedRepositoryTreeGuardV1<'_>, RepositoryDurabilityErrorV1> {
        if directory_limit == 0 || directory_limit > MAX_DIRECTORY_ENTRIES {
            return Err(bound(
                "retained directory-tree limit is outside the admitted bound",
            ));
        }
        self.require_stable_path_epoch()?;
        let (directories, files) = self.retain_directory_tree(relative_root, directory_limit)?;
        let guard = RetainedRepositoryTreeGuardV1 {
            root: self,
            directories,
            files,
        };
        guard.validate()?;
        Ok(guard)
    }

    pub(crate) fn retain_authority_tree_guard(
        &self,
        inventory_limit: usize,
    ) -> Result<RetainedRepositoryTreeGuardV1<'_>, RepositoryDurabilityErrorV1> {
        const RECURSIVE_ROOTS: &[&str] = &[
            ".handbook/state/transactions/registry",
            ".handbook/state/transactions/approvals",
            ".handbook/state/registry-states",
            ".handbook/state/registry-transitions",
            ".handbook/state/authenticator-registrations",
            ".handbook/state/authenticator-make-credential-responses",
            ".handbook/state/authenticator-use-heads",
        ];
        const EXACT_FILES: &[&str] = &[
            ".handbook/state/locks/promotion.lock",
            ".handbook/state/locks/registry.lock",
        ];
        if inventory_limit == 0 || inventory_limit > MAX_DIRECTORY_ENTRIES {
            return Err(bound(
                "retained authority inventory limit is outside the admitted bound",
            ));
        }
        self.require_stable_path_epoch()?;
        let (directories, files) =
            self.retain_selected_tree(RECURSIVE_ROOTS, EXACT_FILES, inventory_limit)?;
        let guard = RetainedRepositoryTreeGuardV1 {
            root: self,
            directories,
            files,
        };
        guard.validate()?;
        Ok(guard)
    }

    fn retain_selected_tree(
        &self,
        recursive_roots: &[&str],
        exact_files: &[&str],
        inventory_limit: usize,
    ) -> Result<(Vec<RetainedTreeDirectory>, Vec<RetainedTreeFile>), RepositoryDurabilityErrorV1>
    {
        let mut queue = VecDeque::from([(".handbook".to_owned(), false)]);
        let mut seen = BTreeSet::new();
        let mut directories = Vec::new();
        let mut files = Vec::new();
        let mut total_bytes = 0_u64;
        while let Some((relative, recursive)) = queue.pop_front() {
            if !seen.insert(relative.clone()) {
                continue;
            }
            if directories.len().saturating_add(files.len()) == inventory_limit {
                return Err(bound("retained authority inventory exceeds its bound"));
            }
            let validated = ValidatedRelativePath::parse(&relative, false)?;
            let directory = platform::open_retained_directory(self, &validated)?;
            let snapshot = directory_snapshot(&directory)?;
            directories.push(RetainedTreeDirectory {
                relative: relative.clone(),
                directory,
                snapshot,
            });
            for entry in self.list_directory_bounded(&relative, MAX_DIRECTORY_ENTRIES)? {
                let child = format!("{relative}/{}", entry.name());
                match entry.kind() {
                    RepositoryDirectoryEntryKindV1::Directory => {
                        let child_recursive =
                            recursive || recursive_roots.contains(&child.as_str());
                        let selected_prefix = recursive_roots
                            .iter()
                            .any(|root| root.starts_with(&format!("{child}/")))
                            || exact_files
                                .iter()
                                .any(|file| file.starts_with(&format!("{child}/")));
                        if child_recursive || selected_prefix {
                            queue.push_back((child, child_recursive));
                        }
                    }
                    RepositoryDirectoryEntryKindV1::RegularFile
                        if recursive || exact_files.contains(&child.as_str()) =>
                    {
                        if directories.len().saturating_add(files.len()) == inventory_limit {
                            return Err(bound("retained authority inventory exceeds its bound"));
                        }
                        let child = ValidatedRelativePath::parse(&child, false)?;
                        let file = self.open_regular_optional(&child, false)?.ok_or_else(|| {
                            unsafe_fs("authority file disappeared during retained inventory")
                        })?;
                        let snapshot = regular_file_snapshot(&file)?;
                        total_bytes = total_bytes
                            .checked_add(snapshot.length)
                            .ok_or_else(|| bound("retained authority byte inventory overflowed"))?;
                        if total_bytes > MAX_RETAINED_TREE_BYTES {
                            return Err(bound(
                                "retained authority closure exceeds its total byte bound",
                            ));
                        }
                        let content_fingerprint =
                            retained_file_fingerprint(&file, snapshot.length)?;
                        files.push(RetainedTreeFile {
                            relative: child.as_str().to_owned(),
                            file,
                            snapshot,
                            content_fingerprint,
                        });
                    }
                    RepositoryDirectoryEntryKindV1::RegularFile => {}
                }
            }
        }
        Ok((directories, files))
    }

    #[cfg(test)]
    fn retain_directory_tree(
        &self,
        relative_root: &str,
        directory_limit: usize,
    ) -> Result<(Vec<RetainedTreeDirectory>, Vec<RetainedTreeFile>), RepositoryDurabilityErrorV1>
    {
        let root = ValidatedRelativePath::parse(relative_root, false)?;
        let mut prefix = String::new();
        let mut queue = VecDeque::new();
        for component in root.as_str().split('/') {
            if !prefix.is_empty() {
                prefix.push('/');
            }
            prefix.push_str(component);
            queue.push_back(prefix.clone());
        }
        let mut seen = BTreeSet::new();
        let mut directories = Vec::new();
        let mut files = Vec::new();
        let mut total_bytes = 0_u64;
        while let Some(relative) = queue.pop_front() {
            if !seen.insert(relative.clone()) {
                continue;
            }
            if directories.len().saturating_add(files.len()) == directory_limit {
                return Err(bound(
                    "retained authority state inventory exceeds its bound",
                ));
            }
            let validated = ValidatedRelativePath::parse(&relative, false)?;
            let directory = platform::open_retained_directory(self, &validated)?;
            let snapshot = directory_snapshot(&directory)?;
            directories.push(RetainedTreeDirectory {
                relative: relative.clone(),
                directory,
                snapshot,
            });
            for entry in self.list_directory_bounded(&relative, MAX_DIRECTORY_ENTRIES)? {
                let child = format!("{relative}/{}", entry.name());
                match entry.kind() {
                    RepositoryDirectoryEntryKindV1::Directory => queue.push_back(child),
                    RepositoryDirectoryEntryKindV1::RegularFile => {
                        if directories.len().saturating_add(files.len()) == directory_limit {
                            return Err(bound(
                                "retained authority state inventory exceeds its bound",
                            ));
                        }
                        let child = ValidatedRelativePath::parse(&child, false)?;
                        let file = self.open_regular_optional(&child, false)?.ok_or_else(|| {
                            unsafe_fs("state file disappeared during retained inventory")
                        })?;
                        let snapshot = regular_file_snapshot(&file)?;
                        total_bytes =
                            total_bytes.checked_add(snapshot.length).ok_or_else(|| {
                                bound("retained authority state byte inventory overflowed")
                            })?;
                        if total_bytes > MAX_RETAINED_TREE_BYTES {
                            return Err(bound(
                                "retained authority state exceeds its total byte bound",
                            ));
                        }
                        let content_fingerprint =
                            retained_file_fingerprint(&file, snapshot.length)?;
                        files.push(RetainedTreeFile {
                            relative: child.as_str().to_owned(),
                            file,
                            snapshot,
                            content_fingerprint,
                        });
                    }
                }
            }
        }
        Ok((directories, files))
    }

    fn require_stable_path_epoch(&self) -> Result<(), RepositoryDurabilityErrorV1> {
        #[cfg(unix)]
        {
            for retained in &self.ancestor_epochs {
                let current = unix_directory_epoch(&retained.directory)?;
                if current.identity != retained.identity
                    || current.modified_seconds != retained.modified_seconds
                    || current.modified_nanoseconds != retained.modified_nanoseconds
                    || current.changed_seconds != retained.changed_seconds
                    || current.changed_nanoseconds != retained.changed_nanoseconds
                {
                    return Err(unsafe_fs(
                        "repository root ancestor changed during retained operation",
                    ));
                }
            }
            if !self.path_still_names_same_physical_root(&self.root_path) {
                return Err(unsafe_fs(
                    "repository path no longer names the retained physical root",
                ));
            }
        }
        #[cfg(windows)]
        require_root_unchanged(self)?;
        Ok(())
    }

    /// Reads one regular, single-link file without following links. Absence is
    /// distinct from an unsafe or uncertain observation.
    pub fn read_regular_bounded(
        &self,
        relative: &str,
        byte_limit: usize,
    ) -> Result<RepositoryReadV1, RepositoryDurabilityErrorV1> {
        if byte_limit > MAX_STRICT_READ_BYTES {
            return Err(bound("strict read limit exceeds the repository maximum"));
        }
        let relative = ValidatedRelativePath::parse(relative, false)?;
        let Some(mut file) = self.open_regular_optional(&relative, false)? else {
            return Ok(RepositoryReadV1::Absent);
        };
        let before = regular_file_snapshot(&file)?;
        let mut bytes = Vec::new();
        (&mut file)
            .take((byte_limit as u64).saturating_add(1))
            .read_to_end(&mut bytes)
            .map_err(|_| uncertain("strict retained read failed"))?;
        if bytes.len() > byte_limit {
            return Err(bound("strict retained read exceeded its byte limit"));
        }
        let after = regular_file_snapshot(&file)?;
        if before != after {
            return Err(unsafe_fs("regular file changed during retained read"));
        }
        Ok(RepositoryReadV1::Found(bytes))
    }

    /// Lists a directory in stable name order, refusing links, special files,
    /// non-UTF-8 children, and caller bounds outside the repository maximum.
    pub fn list_directory_bounded(
        &self,
        relative: &str,
        entry_limit: usize,
    ) -> Result<Vec<RepositoryDirectoryEntryV1>, RepositoryDurabilityErrorV1> {
        if entry_limit > MAX_DIRECTORY_ENTRIES {
            return Err(bound(
                "directory entry limit exceeds the repository maximum",
            ));
        }
        let relative = ValidatedRelativePath::parse(relative, true)?;
        platform::list_directory(self, &relative, entry_limit)
    }

    /// Creates every absent directory component, flushing each created parent.
    pub fn create_directories(&self, relative: &str) -> Result<(), RepositoryDurabilityErrorV1> {
        let relative = ValidatedRelativePath::parse(relative, false)?;
        platform::create_directories(self, &relative)
    }

    /// Creates exactly one new directory below an existing retained parent.
    pub fn create_directory_new_durable(
        &self,
        relative: &str,
    ) -> Result<(), RepositoryDurabilityErrorV1> {
        let relative = ValidatedRelativePath::parse(relative, false)?;
        platform::create_directory_new(self, &relative)
    }

    /// Creates and durably writes exactly one new regular file.
    pub fn write_new_durable(
        &self,
        relative: &str,
        bytes: &[u8],
    ) -> Result<(), RepositoryDurabilityErrorV1> {
        if bytes.len() > MAX_STRICT_READ_BYTES {
            return Err(bound("durable file exceeds the repository byte maximum"));
        }
        let relative = ValidatedRelativePath::parse(relative, false)?;
        platform::write_new(self, &relative, bytes)
    }

    /// Atomically renames one retained regular file without replacing a target.
    pub fn rename_noreplace_durable(
        &self,
        source: &str,
        target: &str,
    ) -> Result<(), RepositoryDurabilityErrorV1> {
        let source = ValidatedRelativePath::parse(source, false)?;
        let target = ValidatedRelativePath::parse(target, false)?;
        platform::rename_noreplace(self, &source, &target)
    }

    /// Atomically renames one retained directory without replacing a target.
    pub fn rename_directory_noreplace_durable(
        &self,
        source: &str,
        target: &str,
    ) -> Result<(), RepositoryDurabilityErrorV1> {
        let source = ValidatedRelativePath::parse(source, false)?;
        let target = ValidatedRelativePath::parse(target, false)?;
        platform::rename_directory_noreplace(self, &source, &target)
    }

    /// Replaces one exact current file using caller-owned prepared and
    /// displaced paths. Both publications are no-replace renames. If the
    /// second publication is interrupted or refused, the explicit displaced
    /// and prepared paths remain available for deterministic recovery.
    pub fn replace_exact_durable(
        &self,
        current: &str,
        expected_bytes: &[u8],
        prepared: &str,
        displaced: &str,
    ) -> Result<(), RepositoryDurabilityErrorV1> {
        if expected_bytes.len() > MAX_STRICT_READ_BYTES {
            return Err(bound(
                "expected current file exceeds the repository byte maximum",
            ));
        }
        let current = ValidatedRelativePath::parse(current, false)?;
        let prepared = ValidatedRelativePath::parse(prepared, false)?;
        let displaced = ValidatedRelativePath::parse(displaced, false)?;
        if current.as_str() == prepared.as_str()
            || current.as_str() == displaced.as_str()
            || prepared.as_str() == displaced.as_str()
        {
            return Err(invalid_path("exact replacement paths must be distinct"));
        }
        let mut retained_current = self
            .open_regular_optional(&current, false)?
            .ok_or_else(|| not_found("exact replacement current file is absent"))?;
        let current_snapshot = regular_file_snapshot(&retained_current)?;
        require_retained_bytes(&mut retained_current, expected_bytes)?;
        platform::rename_noreplace_expected(self, &current, &displaced, Some(current_snapshot))?;
        platform::rename_noreplace_expected(self, &prepared, &current, None)
    }

    /// Removes exactly one retained regular, single-link file and flushes its parent.
    pub fn remove_regular_durable(
        &self,
        relative: &str,
    ) -> Result<(), RepositoryDurabilityErrorV1> {
        let relative = ValidatedRelativePath::parse(relative, false)?;
        platform::remove_regular(self, &relative)
    }

    /// Removes exactly one empty retained directory and flushes its parent.
    pub fn remove_empty_directory_durable(
        &self,
        relative: &str,
    ) -> Result<(), RepositoryDurabilityErrorV1> {
        let relative = ValidatedRelativePath::parse(relative, false)?;
        platform::remove_empty_directory(self, &relative)
    }

    /// Acquires an exclusive native lock on an existing regular file, creating
    /// an empty durable lock file first when absent.
    pub fn acquire_exclusive_lock(
        &self,
        relative: &str,
    ) -> Result<RepositoryExclusiveLockV1, RepositoryDurabilityErrorV1> {
        let relative = ValidatedRelativePath::parse(relative, false)?;
        if self.open_regular_optional(&relative, true)?.is_none() {
            match platform::write_new(self, &relative, &[]) {
                Ok(()) => {}
                Err(error) if error.kind() == RepositoryDurabilityErrorKindV1::AlreadyExists => {}
                Err(error) => return Err(error),
            }
        }
        let file = self
            .open_regular_optional(&relative, true)?
            .ok_or_else(|| not_found("lock file disappeared before acquisition"))?;
        file.lock()
            .map_err(|_| lock_unavailable("exclusive repository lock acquisition failed"))?;
        #[cfg(windows)]
        let parent_guards = self.pin_windows_parent(&relative)?;
        Ok(RepositoryExclusiveLockV1 {
            file: Some(file),
            #[cfg(windows)]
            _parent_guards: parent_guards,
        })
    }

    fn open_regular_optional(
        &self,
        relative: &ValidatedRelativePath,
        writable: bool,
    ) -> Result<Option<File>, RepositoryDurabilityErrorV1> {
        platform::open_regular_optional(self, relative, writable)
    }

    #[cfg(windows)]
    fn absolute(&self, relative: &ValidatedRelativePath) -> PathBuf {
        self.root_path.join(relative.as_path())
    }

    #[cfg(windows)]
    fn pin_windows_parent(
        &self,
        relative: &ValidatedRelativePath,
    ) -> Result<Vec<File>, RepositoryDurabilityErrorV1> {
        require_root_unchanged(self)?;
        let mut guards = Vec::new();
        let mut current = self.root_path.clone();
        let parent = relative.as_path().parent().unwrap_or(Path::new(""));
        for component in parent.components() {
            let Component::Normal(component) = component else {
                return Err(invalid_path("repository path is not normalized"));
            };
            current.push(component);
            guards.push(open_windows_directory(&current)?);
        }
        require_root_unchanged(self)?;
        Ok(guards)
    }
}

#[derive(Clone)]
struct ValidatedRelativePath {
    text: String,
    path: PathBuf,
}

impl ValidatedRelativePath {
    fn parse(value: &str, allow_empty: bool) -> Result<Self, RepositoryDurabilityErrorV1> {
        if value.len() > MAX_RELATIVE_PATH_BYTES
            || value.as_bytes().contains(&0)
            || value.contains('\\')
            || value.starts_with('/')
            || value.ends_with('/')
            || (!allow_empty && value.is_empty())
        {
            return Err(invalid_path("repository-relative path is malformed"));
        }
        if value.is_empty() {
            return Ok(Self {
                text: String::new(),
                path: PathBuf::new(),
            });
        }
        let parts = value.split('/').collect::<Vec<_>>();
        if parts.len() > MAX_RELATIVE_COMPONENTS
            || parts.iter().any(|part| {
                part.is_empty() || *part == "." || *part == ".." || part.len() > MAX_COMPONENT_BYTES
            })
        {
            return Err(invalid_path(
                "repository-relative path is not lexically bounded and normalized",
            ));
        }
        let path = Path::new(value);
        if path.is_absolute()
            || path
                .components()
                .any(|component| !matches!(component, Component::Normal(_)))
        {
            return Err(invalid_path(
                "repository-relative path contains an unsafe component",
            ));
        }
        Ok(Self {
            text: value.to_owned(),
            path: path.to_path_buf(),
        })
    }

    fn as_str(&self) -> &str {
        &self.text
    }

    fn as_path(&self) -> &Path {
        &self.path
    }

    #[cfg(unix)]
    fn parent_and_name(&self) -> Result<(&Path, &Path), RepositoryDurabilityErrorV1> {
        Ok((
            self.path.parent().unwrap_or(Path::new("")),
            Path::new(
                self.path
                    .file_name()
                    .ok_or_else(|| invalid_path("repository-relative path has no child name"))?,
            ),
        ))
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct RegularFileSnapshot {
    identity: PhysicalFileIdentity,
    length: u64,
    version: NativeFileVersion,
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum NativeFileVersion {
    #[cfg(unix)]
    Unix {
        modified_seconds: i64,
        modified_nanoseconds: i64,
        changed_seconds: i64,
        changed_nanoseconds: i64,
    },
    #[cfg(windows)]
    Windows {
        creation_time: i64,
        last_write_time: i64,
        change_time: i64,
    },
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum PhysicalFileIdentity {
    #[cfg(unix)]
    Unix { device: u64, inode: u64, links: u64 },
    #[cfg(windows)]
    Windows {
        volume: u64,
        file_id: [u8; 16],
        links: u32,
    },
}

fn regular_file_snapshot(file: &File) -> Result<RegularFileSnapshot, RepositoryDurabilityErrorV1> {
    let metadata = file
        .metadata()
        .map_err(|_| uncertain("retained file metadata observation failed"))?;
    if !metadata.is_file() {
        return Err(unsafe_fs(
            "retained repository object is not a regular file",
        ));
    }
    let identity = regular_file_identity(file)?;
    Ok(RegularFileSnapshot {
        identity,
        length: metadata.len(),
        version: native_file_version(file)?,
    })
}

fn directory_snapshot(file: &File) -> Result<DirectorySnapshot, RepositoryDurabilityErrorV1> {
    Ok(DirectorySnapshot {
        identity: directory_identity(file)?,
        version: native_file_version(file)?,
    })
}

fn require_retained_bytes(
    file: &mut File,
    expected: &[u8],
) -> Result<(), RepositoryDurabilityErrorV1> {
    let before = regular_file_snapshot(file)?;
    let mut observed = Vec::new();
    (&mut *file)
        .take((expected.len() as u64).saturating_add(1))
        .read_to_end(&mut observed)
        .map_err(|_| uncertain("exact current retained read failed"))?;
    let after = regular_file_snapshot(file)?;
    if before != after {
        return Err(unsafe_fs("exact current file changed during retained read"));
    }
    if observed != expected {
        return Err(mismatch("exact current file bytes do not match"));
    }
    Ok(())
}

fn retained_file_fingerprint(
    file: &File,
    expected_length: u64,
) -> Result<[u8; 32], RepositoryDurabilityErrorV1> {
    if expected_length > MAX_STRICT_READ_BYTES as u64 {
        return Err(bound(
            "retained authority file exceeds the per-file byte bound",
        ));
    }
    let before = regular_file_snapshot(file)?;
    if before.length != expected_length {
        return Err(unsafe_fs(
            "retained authority file length changed before fingerprinting",
        ));
    }
    let mut reader = file;
    reader
        .seek(SeekFrom::Start(0))
        .map_err(|_| uncertain("retained authority file seek failed"))?;
    let mut remaining = expected_length;
    let mut digest = Sha256::new();
    let mut buffer = [0_u8; 8192];
    while remaining != 0 {
        let requested = usize::try_from(remaining.min(buffer.len() as u64))
            .expect("bounded read size fits usize");
        let read = reader
            .read(&mut buffer[..requested])
            .map_err(|_| uncertain("retained authority file read failed"))?;
        if read == 0 {
            return Err(unsafe_fs(
                "retained authority file ended before its admitted length",
            ));
        }
        digest.update(&buffer[..read]);
        remaining -= read as u64;
    }
    let mut extra = [0_u8; 1];
    if reader
        .read(&mut extra)
        .map_err(|_| uncertain("retained authority file terminal read failed"))?
        != 0
    {
        return Err(unsafe_fs(
            "retained authority file exceeds its admitted length",
        ));
    }
    reader
        .seek(SeekFrom::Start(0))
        .map_err(|_| uncertain("retained authority file reset failed"))?;
    if regular_file_snapshot(file)? != before {
        return Err(unsafe_fs(
            "retained authority file changed while fingerprinting",
        ));
    }
    Ok(digest.finalize().into())
}

fn validate_root_path(path: &Path) -> Result<PathBuf, RepositoryDurabilityErrorV1> {
    if !path.is_absolute()
        || path
            .components()
            .any(|component| matches!(component, Component::CurDir | Component::ParentDir))
    {
        return Err(invalid_path(
            "repository root must be absolute and lexically normalized",
        ));
    }
    for ancestor in path.ancestors() {
        let metadata = fs::symlink_metadata(ancestor)
            .map_err(|_| unsafe_fs("repository root ancestor metadata admission failed"))?;
        if !metadata.is_dir() || metadata.file_type().is_symlink() || metadata_is_reparse(&metadata)
        {
            return Err(unsafe_fs(
                "repository root contains a link, reparse point, or non-directory ancestor",
            ));
        }
    }
    Ok(path.to_path_buf())
}

fn ensure_supported_platform() -> Result<(), RepositoryDurabilityErrorV1> {
    if cfg!(any(unix, windows)) {
        Ok(())
    } else {
        Err(refusal(
            RepositoryDurabilityErrorKindV1::UnsupportedPlatform,
            "retained repository durability requires Unix or Windows primitives",
        ))
    }
}

#[cfg(unix)]
fn directory_identity(
    file: &File,
) -> Result<PhysicalDirectoryIdentity, RepositoryDurabilityErrorV1> {
    use std::os::unix::fs::MetadataExt;
    let metadata = file
        .metadata()
        .map_err(|_| uncertain("retained root metadata observation failed"))?;
    if !metadata.is_dir() {
        return Err(unsafe_fs(
            "retained repository root is no longer a directory",
        ));
    }
    Ok(PhysicalDirectoryIdentity::Unix {
        device: metadata.dev(),
        inode: metadata.ino(),
    })
}

#[cfg(unix)]
fn retain_unix_ancestor_epochs(
    root_path: &Path,
) -> Result<Vec<RetainedDirectoryEpoch>, RepositoryDurabilityErrorV1> {
    use rustix::fs::{open, Mode, OFlags};
    let mut ancestors = root_path.ancestors().skip(1).collect::<Vec<_>>();
    ancestors.reverse();
    ancestors
        .into_iter()
        .map(|path| {
            let directory = open(
                path,
                OFlags::RDONLY | OFlags::CLOEXEC | OFlags::DIRECTORY | OFlags::NOFOLLOW,
                Mode::empty(),
            )
            .map(File::from)
            .map_err(|_| unsafe_fs("repository root ancestor no-follow open failed"))?;
            unix_directory_epoch(&directory)
        })
        .collect()
}

#[cfg(unix)]
fn unix_directory_epoch(
    directory: &File,
) -> Result<RetainedDirectoryEpoch, RepositoryDurabilityErrorV1> {
    use std::os::unix::fs::MetadataExt;
    let metadata = directory
        .metadata()
        .map_err(|_| uncertain("retained ancestor metadata observation failed"))?;
    if !metadata.is_dir() {
        return Err(unsafe_fs("retained repository ancestor is not a directory"));
    }
    Ok(RetainedDirectoryEpoch {
        directory: directory
            .try_clone()
            .map_err(|_| uncertain("retained ancestor handle duplication failed"))?,
        identity: PhysicalDirectoryIdentity::Unix {
            device: metadata.dev(),
            inode: metadata.ino(),
        },
        modified_seconds: metadata.mtime(),
        modified_nanoseconds: metadata.mtime_nsec(),
        changed_seconds: metadata.ctime(),
        changed_nanoseconds: metadata.ctime_nsec(),
    })
}

#[cfg(unix)]
fn regular_file_identity(file: &File) -> Result<PhysicalFileIdentity, RepositoryDurabilityErrorV1> {
    use std::os::unix::fs::MetadataExt;
    let metadata = file
        .metadata()
        .map_err(|_| uncertain("retained regular-file identity observation failed"))?;
    if metadata.nlink() != 1 {
        return Err(unsafe_fs("retained regular file has a hard-link alias"));
    }
    Ok(PhysicalFileIdentity::Unix {
        device: metadata.dev(),
        inode: metadata.ino(),
        links: metadata.nlink(),
    })
}

#[cfg(unix)]
fn native_file_version(file: &File) -> Result<NativeFileVersion, RepositoryDurabilityErrorV1> {
    use std::os::unix::fs::MetadataExt;
    let metadata = file
        .metadata()
        .map_err(|_| uncertain("retained regular-file version observation failed"))?;
    Ok(NativeFileVersion::Unix {
        modified_seconds: metadata.mtime(),
        modified_nanoseconds: metadata.mtime_nsec(),
        changed_seconds: metadata.ctime(),
        changed_nanoseconds: metadata.ctime_nsec(),
    })
}

#[cfg(windows)]
fn metadata_is_reparse(metadata: &fs::Metadata) -> bool {
    use std::os::windows::fs::MetadataExt;
    metadata.file_attributes() & 0x0000_0400 != 0
}

#[cfg(not(windows))]
fn metadata_is_reparse(_metadata: &fs::Metadata) -> bool {
    false
}

#[cfg(windows)]
#[allow(
    unsafe_code,
    reason = "physical repository binding requires the exact retained Win32 FILE_ID_INFO"
)]
fn windows_handle_info(
    file: &File,
) -> Result<(PhysicalDirectoryIdentity, u32, bool, bool), RepositoryDurabilityErrorV1> {
    use std::ffi::c_void;
    use std::mem::size_of;
    use std::os::windows::io::AsRawHandle;
    use std::ptr;

    #[repr(C)]
    struct FileId128 {
        identifier: [u8; 16],
    }
    #[repr(C)]
    struct FileIdInfo {
        volume_serial_number: u64,
        file_id: FileId128,
    }
    #[repr(C)]
    struct FileStandardInfo {
        allocation_size: i64,
        end_of_file: i64,
        number_of_links: u32,
        delete_pending: u8,
        directory: u8,
    }
    #[repr(C)]
    struct FileAttributeTagInfo {
        file_attributes: u32,
        reparse_tag: u32,
    }
    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn GetFileInformationByHandleEx(
            file: *mut c_void,
            class: i32,
            information: *mut c_void,
            information_size: u32,
        ) -> i32;
    }
    const FILE_STANDARD_INFO_CLASS: i32 = 1;
    const FILE_ATTRIBUTE_TAG_INFO_CLASS: i32 = 9;
    const FILE_ID_INFO_CLASS: i32 = 18;
    const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x0000_0400;

    let mut id = FileIdInfo {
        volume_serial_number: 0,
        file_id: FileId128 {
            identifier: [0; 16],
        },
    };
    let mut standard = FileStandardInfo {
        allocation_size: 0,
        end_of_file: 0,
        number_of_links: 0,
        delete_pending: 0,
        directory: 0,
    };
    let mut tag = FileAttributeTagInfo {
        file_attributes: 0,
        reparse_tag: 0,
    };
    // SAFETY: `file` owns the handle for all calls and every output pointer
    // names one initialized, correctly sized `repr(C)` buffer.
    let ok = unsafe {
        GetFileInformationByHandleEx(
            file.as_raw_handle(),
            FILE_ID_INFO_CLASS,
            ptr::from_mut(&mut id).cast(),
            size_of::<FileIdInfo>() as u32,
        ) != 0
            && GetFileInformationByHandleEx(
                file.as_raw_handle(),
                FILE_STANDARD_INFO_CLASS,
                ptr::from_mut(&mut standard).cast(),
                size_of::<FileStandardInfo>() as u32,
            ) != 0
            && GetFileInformationByHandleEx(
                file.as_raw_handle(),
                FILE_ATTRIBUTE_TAG_INFO_CLASS,
                ptr::from_mut(&mut tag).cast(),
                size_of::<FileAttributeTagInfo>() as u32,
            ) != 0
    };
    if !ok {
        return Err(unsafe_fs("retained Windows information query failed"));
    }
    if id.file_id.identifier == [0; 16] || id.file_id.identifier == [u8::MAX; 16] {
        return Err(unsafe_fs("retained Windows file identity is unavailable"));
    }
    if standard.delete_pending != 0 {
        return Err(unsafe_fs("retained Windows object is pending deletion"));
    }
    Ok((
        PhysicalDirectoryIdentity::Windows {
            volume: id.volume_serial_number,
            file_id: id.file_id.identifier,
        },
        standard.number_of_links,
        standard.directory != 0,
        tag.file_attributes & FILE_ATTRIBUTE_REPARSE_POINT != 0 || tag.reparse_tag != 0,
    ))
}

#[cfg(windows)]
fn directory_identity(
    file: &File,
) -> Result<PhysicalDirectoryIdentity, RepositoryDurabilityErrorV1> {
    let (identity, _, directory, reparse) = windows_handle_info(file)?;
    if !directory || reparse {
        return Err(unsafe_fs("retained Windows directory is unsafe"));
    }
    Ok(identity)
}

#[cfg(windows)]
fn regular_file_identity(file: &File) -> Result<PhysicalFileIdentity, RepositoryDurabilityErrorV1> {
    let (identity, links, directory, reparse) = windows_handle_info(file)?;
    if directory || reparse || links != 1 {
        return Err(unsafe_fs("retained Windows regular file is unsafe"));
    }
    let PhysicalDirectoryIdentity::Windows { volume, file_id } = identity;
    Ok(PhysicalFileIdentity::Windows {
        volume,
        file_id,
        links,
    })
}

#[cfg(windows)]
#[allow(
    unsafe_code,
    reason = "stable retained reads require the exact Win32 FILE_BASIC_INFO change-time fields"
)]
fn native_file_version(file: &File) -> Result<NativeFileVersion, RepositoryDurabilityErrorV1> {
    use std::ffi::c_void;
    use std::mem::size_of;
    use std::os::windows::io::AsRawHandle;
    use std::ptr;

    #[repr(C)]
    struct FileBasicInfo {
        creation_time: i64,
        last_access_time: i64,
        last_write_time: i64,
        change_time: i64,
        file_attributes: u32,
    }
    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn GetFileInformationByHandleEx(
            file: *mut c_void,
            class: i32,
            information: *mut c_void,
            information_size: u32,
        ) -> i32;
    }
    const FILE_BASIC_INFO_CLASS: i32 = 0;
    let mut basic = FileBasicInfo {
        creation_time: 0,
        last_access_time: 0,
        last_write_time: 0,
        change_time: 0,
        file_attributes: 0,
    };
    // SAFETY: `file` retains the handle and `basic` is one initialized,
    // correctly sized `repr(C)` output buffer for `FileBasicInfo`.
    if unsafe {
        GetFileInformationByHandleEx(
            file.as_raw_handle(),
            FILE_BASIC_INFO_CLASS,
            ptr::from_mut(&mut basic).cast(),
            size_of::<FileBasicInfo>() as u32,
        )
    } == 0
    {
        return Err(uncertain(
            "retained Windows regular-file version observation failed",
        ));
    }
    Ok(NativeFileVersion::Windows {
        creation_time: basic.creation_time,
        last_write_time: basic.last_write_time,
        change_time: basic.change_time,
    })
}

#[cfg(windows)]
fn open_windows_directory(path: &Path) -> Result<File, RepositoryDurabilityErrorV1> {
    let file = open_windows_directory_guard(path)?;
    directory_identity(&file)?;
    Ok(file)
}

#[cfg(windows)]
fn open_windows_directory_guard(path: &Path) -> Result<File, RepositoryDurabilityErrorV1> {
    use std::os::windows::fs::OpenOptionsExt;
    const FILE_SHARE_READ: u32 = 0x0000_0001;
    const FILE_SHARE_WRITE: u32 = 0x0000_0002;
    const FILE_FLAG_BACKUP_SEMANTICS: u32 = 0x0200_0000;
    const FILE_FLAG_OPEN_REPARSE_POINT: u32 = 0x0020_0000;
    let file = OpenOptions::new()
        .read(true)
        .share_mode(FILE_SHARE_READ | FILE_SHARE_WRITE)
        .custom_flags(FILE_FLAG_BACKUP_SEMANTICS | FILE_FLAG_OPEN_REPARSE_POINT)
        .open(path)
        .map_err(|_| unsafe_fs("Windows directory could not be pinned"))?;
    let metadata = file
        .metadata()
        .map_err(|_| unsafe_fs("Windows pinned directory metadata failed"))?;
    if !metadata.is_dir() || metadata_is_reparse(&metadata) {
        return Err(unsafe_fs("Windows pinned directory is unsafe"));
    }
    Ok(file)
}

#[cfg(windows)]
fn open_windows_deletable_directory(path: &Path) -> Result<File, RepositoryDurabilityErrorV1> {
    use std::os::windows::fs::OpenOptionsExt;
    const FILE_SHARE_READ: u32 = 0x0000_0001;
    const FILE_SHARE_WRITE: u32 = 0x0000_0002;
    const FILE_SHARE_DELETE: u32 = 0x0000_0004;
    const FILE_FLAG_BACKUP_SEMANTICS: u32 = 0x0200_0000;
    const FILE_FLAG_OPEN_REPARSE_POINT: u32 = 0x0020_0000;
    let file = OpenOptions::new()
        .read(true)
        .share_mode(FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE)
        .custom_flags(FILE_FLAG_BACKUP_SEMANTICS | FILE_FLAG_OPEN_REPARSE_POINT)
        .open(path)
        .map_err(|_| unsafe_fs("Windows directory could not be retained for deletion"))?;
    directory_identity(&file)?;
    Ok(file)
}

#[cfg(windows)]
fn pin_windows_directory_chain(path: &Path) -> Result<Vec<File>, RepositoryDurabilityErrorV1> {
    let mut paths = path.ancestors().collect::<Vec<_>>();
    paths.reverse();
    let mut guards = Vec::with_capacity(paths.len());
    let final_index = paths.len().saturating_sub(1);
    for (index, ancestor) in paths.into_iter().enumerate() {
        guards.push(if index == final_index {
            open_windows_directory(ancestor)?
        } else {
            open_windows_directory_guard(ancestor)?
        });
    }
    Ok(guards)
}

#[cfg(windows)]
fn require_root_unchanged(
    root: &RetainedRepositoryRootV1,
) -> Result<(), RepositoryDurabilityErrorV1> {
    let current_path = open_windows_directory(&root.root_path)?;
    if directory_identity(&root.root)? != root.identity
        || directory_identity(&current_path)? != root.identity
    {
        return Err(unsafe_fs("retained repository root identity changed"));
    }
    Ok(())
}

fn refusal(
    kind: RepositoryDurabilityErrorKindV1,
    detail: &'static str,
) -> RepositoryDurabilityErrorV1 {
    RepositoryDurabilityErrorV1 { kind, detail }
}

fn invalid_path(detail: &'static str) -> RepositoryDurabilityErrorV1 {
    refusal(RepositoryDurabilityErrorKindV1::InvalidRelativePath, detail)
}

fn unsafe_fs(detail: &'static str) -> RepositoryDurabilityErrorV1 {
    refusal(RepositoryDurabilityErrorKindV1::UnsafeFilesystem, detail)
}

fn bound(detail: &'static str) -> RepositoryDurabilityErrorV1 {
    refusal(RepositoryDurabilityErrorKindV1::BoundExceeded, detail)
}

fn already_exists(detail: &'static str) -> RepositoryDurabilityErrorV1 {
    refusal(RepositoryDurabilityErrorKindV1::AlreadyExists, detail)
}

fn not_found(detail: &'static str) -> RepositoryDurabilityErrorV1 {
    refusal(RepositoryDurabilityErrorKindV1::NotFound, detail)
}

fn mismatch(detail: &'static str) -> RepositoryDurabilityErrorV1 {
    refusal(
        RepositoryDurabilityErrorKindV1::CurrentValueMismatch,
        detail,
    )
}

fn lock_unavailable(detail: &'static str) -> RepositoryDurabilityErrorV1 {
    refusal(RepositoryDurabilityErrorKindV1::LockUnavailable, detail)
}

fn uncertain(detail: &'static str) -> RepositoryDurabilityErrorV1 {
    refusal(RepositoryDurabilityErrorKindV1::DurabilityUncertain, detail)
}

#[cfg(unix)]
mod platform {
    use super::*;
    use rustix::fd::OwnedFd;
    use rustix::fs::{
        mkdirat, openat, renameat_with, statat, unlinkat, AtFlags, Dir, FileType, Mode, OFlags,
        RenameFlags,
    };

    const DIRECTORY_FLAGS: OFlags = OFlags::RDONLY
        .union(OFlags::CLOEXEC)
        .union(OFlags::DIRECTORY)
        .union(OFlags::NOFOLLOW);

    fn duplicate_root(
        root: &RetainedRepositoryRootV1,
    ) -> Result<OwnedFd, RepositoryDurabilityErrorV1> {
        rustix::io::dup(&root.root)
            .map_err(|_| unsafe_fs("retained repository root duplication failed"))
    }

    fn open_directory(
        root: &RetainedRepositoryRootV1,
        relative: &Path,
    ) -> Result<OwnedFd, RepositoryDurabilityErrorV1> {
        let mut current = duplicate_root(root)?;
        for component in relative.components() {
            let Component::Normal(component) = component else {
                return Err(invalid_path("repository directory path is not normalized"));
            };
            current = openat(
                &current,
                Path::new(component),
                DIRECTORY_FLAGS,
                Mode::empty(),
            )
            .map_err(|_| unsafe_fs("repository directory component failed no-follow admission"))?;
        }
        Ok(current)
    }

    pub(super) fn open_retained_directory(
        root: &RetainedRepositoryRootV1,
        relative: &ValidatedRelativePath,
    ) -> Result<File, RepositoryDurabilityErrorV1> {
        Ok(File::from(open_directory(root, relative.as_path())?))
    }

    fn open_parent(
        root: &RetainedRepositoryRootV1,
        relative: &ValidatedRelativePath,
    ) -> Result<(OwnedFd, &Path), RepositoryDurabilityErrorV1> {
        let (parent, name) = relative.parent_and_name()?;
        Ok((open_directory(root, parent)?, name))
    }

    pub(super) fn open_regular_optional(
        root: &RetainedRepositoryRootV1,
        relative: &ValidatedRelativePath,
        writable: bool,
    ) -> Result<Option<File>, RepositoryDurabilityErrorV1> {
        let (parent, name) = open_parent(root, relative)?;
        let access = if writable {
            OFlags::RDWR
        } else {
            OFlags::RDONLY
        };
        match openat(
            &parent,
            name,
            access | OFlags::CLOEXEC | OFlags::NOFOLLOW,
            Mode::empty(),
        ) {
            Ok(file) => {
                let file = File::from(file);
                regular_file_snapshot(&file)?;
                Ok(Some(file))
            }
            Err(rustix::io::Errno::NOENT) => Ok(None),
            Err(_) => Err(unsafe_fs(
                "repository regular file failed no-follow admission",
            )),
        }
    }

    pub(super) fn list_directory(
        root: &RetainedRepositoryRootV1,
        relative: &ValidatedRelativePath,
        entry_limit: usize,
    ) -> Result<Vec<RepositoryDirectoryEntryV1>, RepositoryDurabilityErrorV1> {
        let directory = open_directory(root, relative.as_path())?;
        let mut stream = Dir::read_from(&directory)
            .map_err(|_| uncertain("retained directory stream could not be opened"))?;
        let mut entries = Vec::new();
        while let Some(entry) = stream
            .read()
            .map_err(|_| uncertain("retained directory listing failed"))?
        {
            let name = entry.file_name().to_bytes();
            if name == b"." || name == b".." {
                continue;
            }
            if entries.len() == entry_limit {
                return Err(bound("retained directory listing exceeded its entry limit"));
            }
            let name = std::str::from_utf8(name)
                .map_err(|_| unsafe_fs("repository directory contains a non-UTF-8 child"))?;
            ValidatedRelativePath::parse(name, false)?;
            let stat = statat(&directory, Path::new(name), AtFlags::SYMLINK_NOFOLLOW)
                .map_err(|_| uncertain("directory child metadata observation failed"))?;
            let kind = match FileType::from_raw_mode(stat.st_mode) {
                FileType::RegularFile if stat.st_nlink == 1 => {
                    RepositoryDirectoryEntryKindV1::RegularFile
                }
                FileType::RegularFile => {
                    return Err(unsafe_fs(
                        "repository directory contains a hard-linked regular file",
                    ))
                }
                FileType::Directory => RepositoryDirectoryEntryKindV1::Directory,
                _ => {
                    return Err(unsafe_fs(
                        "repository directory contains a link or special file",
                    ))
                }
            };
            entries.push(RepositoryDirectoryEntryV1 {
                name: name.to_owned(),
                kind,
            });
        }
        entries.sort();
        Ok(entries)
    }

    pub(super) fn create_directories(
        root: &RetainedRepositoryRootV1,
        relative: &ValidatedRelativePath,
    ) -> Result<(), RepositoryDurabilityErrorV1> {
        let mut current = duplicate_root(root)?;
        for component in relative.as_path().components() {
            let Component::Normal(component) = component else {
                return Err(invalid_path("repository directory path is not normalized"));
            };
            let created = match mkdirat(
                &current,
                Path::new(component),
                Mode::RUSR | Mode::WUSR | Mode::XUSR,
            ) {
                Ok(()) => true,
                Err(rustix::io::Errno::EXIST) => false,
                Err(_) => return Err(uncertain("descriptor-relative directory creation failed")),
            };
            let next = openat(
                &current,
                Path::new(component),
                DIRECTORY_FLAGS,
                Mode::empty(),
            )
            .map_err(|_| unsafe_fs("created directory failed no-follow admission"))?;
            if created {
                rustix::fs::fsync(&current)
                    .map_err(|_| uncertain("created directory parent flush failed"))?;
                rustix::fs::fsync(&next)
                    .map_err(|_| uncertain("created directory flush failed"))?;
            }
            current = next;
        }
        Ok(())
    }

    pub(super) fn create_directory_new(
        root: &RetainedRepositoryRootV1,
        relative: &ValidatedRelativePath,
    ) -> Result<(), RepositoryDurabilityErrorV1> {
        let (parent, name) = open_parent(root, relative)?;
        match mkdirat(&parent, name, Mode::RUSR | Mode::WUSR | Mode::XUSR) {
            Ok(()) => {}
            Err(rustix::io::Errno::EXIST) => {
                return Err(already_exists("repository directory already exists"))
            }
            Err(_) => return Err(uncertain("descriptor-relative directory creation failed")),
        }
        let child = openat(&parent, name, DIRECTORY_FLAGS, Mode::empty())
            .map_err(|_| unsafe_fs("new repository directory failed no-follow admission"))?;
        rustix::fs::fsync(&child)
            .map_err(|_| uncertain("new repository directory flush failed"))?;
        rustix::fs::fsync(&parent)
            .map_err(|_| uncertain("new repository directory parent flush failed"))
    }

    pub(super) fn write_new(
        root: &RetainedRepositoryRootV1,
        relative: &ValidatedRelativePath,
        bytes: &[u8],
    ) -> Result<(), RepositoryDurabilityErrorV1> {
        let (parent, name) = open_parent(root, relative)?;
        let file = match openat(
            &parent,
            name,
            OFlags::WRONLY | OFlags::CLOEXEC | OFlags::NOFOLLOW | OFlags::CREATE | OFlags::EXCL,
            Mode::RUSR | Mode::WUSR,
        ) {
            Ok(file) => file,
            Err(rustix::io::Errno::EXIST) => {
                return Err(already_exists("repository file already exists"))
            }
            Err(_) => return Err(uncertain("descriptor-relative durable create-new failed")),
        };
        let mut file = File::from(file);
        file.write_all(bytes)
            .map_err(|_| uncertain("durable repository write failed"))?;
        file.sync_all()
            .map_err(|_| uncertain("durable repository file flush failed"))?;
        rustix::fs::fsync(&parent).map_err(|_| uncertain("durable repository parent flush failed"))
    }

    pub(super) fn rename_noreplace(
        root: &RetainedRepositoryRootV1,
        source: &ValidatedRelativePath,
        target: &ValidatedRelativePath,
    ) -> Result<(), RepositoryDurabilityErrorV1> {
        rename_noreplace_expected(root, source, target, None)
    }

    pub(super) fn rename_noreplace_expected(
        root: &RetainedRepositoryRootV1,
        source: &ValidatedRelativePath,
        target: &ValidatedRelativePath,
        expected_source: Option<RegularFileSnapshot>,
    ) -> Result<(), RepositoryDurabilityErrorV1> {
        let retained = open_regular_optional(root, source, false)?
            .ok_or_else(|| not_found("rename source is absent"))?;
        let expected = regular_file_snapshot(&retained)?;
        if expected_source.is_some_and(|identity| identity != expected) {
            return Err(unsafe_fs(
                "rename source changed after exact retained admission",
            ));
        }
        let (source_parent, source_name) = open_parent(root, source)?;
        let (target_parent, target_name) = open_parent(root, target)?;
        match renameat_with(
            &source_parent,
            source_name,
            &target_parent,
            target_name,
            RenameFlags::NOREPLACE,
        ) {
            Ok(()) => {}
            Err(rustix::io::Errno::EXIST) => {
                return Err(already_exists("rename target already exists"))
            }
            Err(rustix::io::Errno::NOENT) => {
                return Err(not_found("rename source or parent disappeared"))
            }
            Err(_) => return Err(uncertain("descriptor-relative no-replace rename failed")),
        }
        rustix::fs::fsync(&source_parent)
            .map_err(|_| uncertain("rename source parent flush failed"))?;
        if source_parent.as_fd() != target_parent.as_fd() {
            rustix::fs::fsync(&target_parent)
                .map_err(|_| uncertain("rename target parent flush failed"))?;
        }
        let installed = open_regular_optional(root, target, false)?
            .ok_or_else(|| uncertain("renamed target is absent after publication"))?;
        let installed = regular_file_snapshot(&installed)?;
        if installed.identity != expected.identity || installed.length != expected.length {
            return Err(unsafe_fs(
                "renamed target is not the retained source identity",
            ));
        }
        Ok(())
    }

    pub(super) fn rename_directory_noreplace(
        root: &RetainedRepositoryRootV1,
        source: &ValidatedRelativePath,
        target: &ValidatedRelativePath,
    ) -> Result<(), RepositoryDurabilityErrorV1> {
        let retained = File::from(open_directory(root, source.as_path())?);
        let expected = directory_identity(&retained)?;
        let (source_parent, source_name) = open_parent(root, source)?;
        let (target_parent, target_name) = open_parent(root, target)?;
        match renameat_with(
            &source_parent,
            source_name,
            &target_parent,
            target_name,
            RenameFlags::NOREPLACE,
        ) {
            Ok(()) => {}
            Err(rustix::io::Errno::EXIST) => {
                return Err(already_exists("directory rename target already exists"))
            }
            Err(rustix::io::Errno::NOENT) => {
                return Err(not_found("directory rename source or parent disappeared"))
            }
            Err(_) => return Err(uncertain("descriptor-relative directory rename failed")),
        }
        rustix::fs::fsync(&source_parent)
            .map_err(|_| uncertain("directory rename source parent flush failed"))?;
        rustix::fs::fsync(&target_parent)
            .map_err(|_| uncertain("directory rename target parent flush failed"))?;
        let installed = File::from(open_directory(root, target.as_path())?);
        if directory_identity(&installed)? != expected {
            return Err(unsafe_fs(
                "renamed directory is not the retained source identity",
            ));
        }
        Ok(())
    }

    pub(super) fn remove_regular(
        root: &RetainedRepositoryRootV1,
        relative: &ValidatedRelativePath,
    ) -> Result<(), RepositoryDurabilityErrorV1> {
        let retained = open_regular_optional(root, relative, false)?
            .ok_or_else(|| not_found("repository file is absent"))?;
        let expected = regular_file_snapshot(&retained)?;
        let (parent, name) = open_parent(root, relative)?;
        let current = openat(
            &parent,
            name,
            OFlags::RDONLY | OFlags::CLOEXEC | OFlags::NOFOLLOW,
            Mode::empty(),
        )
        .map(File::from)
        .map_err(|_| unsafe_fs("repository file changed before removal"))?;
        if regular_file_snapshot(&current)? != expected {
            return Err(unsafe_fs("repository file identity changed before removal"));
        }
        unlinkat(&parent, name, AtFlags::empty())
            .map_err(|_| uncertain("descriptor-relative regular-file removal failed"))?;
        rustix::fs::fsync(&parent).map_err(|_| uncertain("removed file parent flush failed"))
    }

    pub(super) fn remove_empty_directory(
        root: &RetainedRepositoryRootV1,
        relative: &ValidatedRelativePath,
    ) -> Result<(), RepositoryDurabilityErrorV1> {
        let (parent, name) = open_parent(root, relative)?;
        let retained = openat(&parent, name, DIRECTORY_FLAGS, Mode::empty())
            .map_err(|_| not_found("repository directory is absent or unsafe"))?;
        unlinkat(&parent, name, AtFlags::REMOVEDIR)
            .map_err(|_| uncertain("descriptor-relative empty-directory removal failed"))?;
        rustix::fs::fsync(&parent)
            .map_err(|_| uncertain("removed directory parent flush failed"))?;
        drop(retained);
        Ok(())
    }
}

#[cfg(windows)]
mod platform {
    use super::*;
    use std::os::windows::fs::OpenOptionsExt;

    const FILE_SHARE_READ: u32 = 0x0000_0001;
    const FILE_SHARE_WRITE: u32 = 0x0000_0002;
    const FILE_SHARE_DELETE: u32 = 0x0000_0004;
    const FILE_FLAG_BACKUP_SEMANTICS: u32 = 0x0200_0000;
    const FILE_FLAG_OPEN_REPARSE_POINT: u32 = 0x0020_0000;
    const FILE_FLAG_WRITE_THROUGH: u32 = 0x8000_0000;

    pub(super) fn open_retained_directory(
        root: &RetainedRepositoryRootV1,
        relative: &ValidatedRelativePath,
    ) -> Result<File, RepositoryDurabilityErrorV1> {
        let _parents = root.pin_windows_parent(relative)?;
        open_windows_directory(&root.absolute(relative))
    }

    pub(super) fn open_regular_optional(
        root: &RetainedRepositoryRootV1,
        relative: &ValidatedRelativePath,
        writable: bool,
    ) -> Result<Option<File>, RepositoryDurabilityErrorV1> {
        let _guards = root.pin_windows_parent(relative)?;
        let mut options = OpenOptions::new();
        options
            .read(true)
            .share_mode(FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE);
        if writable {
            options.write(true);
        }
        let file = match options
            .custom_flags(FILE_FLAG_OPEN_REPARSE_POINT)
            .open(root.absolute(relative))
        {
            Ok(file) => file,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(_) => return Err(unsafe_fs("Windows regular file retained open failed")),
        };
        regular_file_snapshot(&file)?;
        Ok(Some(file))
    }

    pub(super) fn list_directory(
        root: &RetainedRepositoryRootV1,
        relative: &ValidatedRelativePath,
        entry_limit: usize,
    ) -> Result<Vec<RepositoryDirectoryEntryV1>, RepositoryDurabilityErrorV1> {
        let _parents = root.pin_windows_parent(relative)?;
        let path = root.absolute(relative);
        let directory = open_windows_directory(&path)?;
        let expected = directory_identity(&directory)?;
        let mut entries = Vec::new();
        let listing = fs::read_dir(&path)
            .map_err(|_| uncertain("Windows retained directory listing failed"))?;
        for entry in listing {
            if entries.len() == entry_limit {
                return Err(bound("retained directory listing exceeded its entry limit"));
            }
            let entry =
                entry.map_err(|_| uncertain("Windows directory entry observation failed"))?;
            let name = entry
                .file_name()
                .into_string()
                .map_err(|_| unsafe_fs("repository directory contains a non-UTF-8 child"))?;
            ValidatedRelativePath::parse(&name, false)?;
            let metadata = fs::symlink_metadata(entry.path())
                .map_err(|_| uncertain("Windows directory child metadata observation failed"))?;
            if metadata.file_type().is_symlink() || metadata_is_reparse(&metadata) {
                return Err(unsafe_fs(
                    "repository directory contains a link or reparse point",
                ));
            }
            let kind = if metadata.is_file() {
                if !crate::canonical_repo_support::path_has_single_link(&entry.path()) {
                    return Err(unsafe_fs(
                        "repository directory contains a hard-linked regular file",
                    ));
                }
                RepositoryDirectoryEntryKindV1::RegularFile
            } else if metadata.is_dir() {
                RepositoryDirectoryEntryKindV1::Directory
            } else {
                return Err(unsafe_fs("repository directory contains a special file"));
            };
            entries.push(RepositoryDirectoryEntryV1 { name, kind });
        }
        if directory_identity(&directory)? != expected {
            return Err(unsafe_fs(
                "Windows directory changed identity during listing",
            ));
        }
        entries.sort();
        Ok(entries)
    }

    pub(super) fn create_directories(
        root: &RetainedRepositoryRootV1,
        relative: &ValidatedRelativePath,
    ) -> Result<(), RepositoryDurabilityErrorV1> {
        let mut current = root.root_path.clone();
        let mut guards = Vec::new();
        for component in relative.as_path().components() {
            let Component::Normal(component) = component else {
                return Err(invalid_path("repository directory path is not normalized"));
            };
            let parent = current.clone();
            current.push(component);
            let created = match fs::create_dir(&current) {
                Ok(()) => true,
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => false,
                Err(_) => return Err(uncertain("Windows repository directory creation failed")),
            };
            guards.push(open_windows_directory(&current)?);
            if created {
                sync_windows_directory(&current)?;
                sync_windows_directory(&parent)?;
            }
        }
        require_root_unchanged(root)
    }

    pub(super) fn create_directory_new(
        root: &RetainedRepositoryRootV1,
        relative: &ValidatedRelativePath,
    ) -> Result<(), RepositoryDurabilityErrorV1> {
        let _guards = root.pin_windows_parent(relative)?;
        let path = root.absolute(relative);
        match fs::create_dir(&path) {
            Ok(()) => {}
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
                return Err(already_exists("repository directory already exists"))
            }
            Err(_) => return Err(uncertain("Windows repository directory creation failed")),
        }
        open_windows_directory(&path)?;
        sync_windows_directory(&path)?;
        sync_windows_directory(
            path.parent()
                .ok_or_else(|| invalid_path("repository directory has no parent"))?,
        )
    }

    pub(super) fn write_new(
        root: &RetainedRepositoryRootV1,
        relative: &ValidatedRelativePath,
        bytes: &[u8],
    ) -> Result<(), RepositoryDurabilityErrorV1> {
        let _guards = root.pin_windows_parent(relative)?;
        let path = root.absolute(relative);
        let mut file = match OpenOptions::new()
            .read(true)
            .write(true)
            .create_new(true)
            .share_mode(FILE_SHARE_READ | FILE_SHARE_DELETE)
            .custom_flags(FILE_FLAG_OPEN_REPARSE_POINT | FILE_FLAG_WRITE_THROUGH)
            .open(&path)
        {
            Ok(file) => file,
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
                return Err(already_exists("repository file already exists"))
            }
            Err(_) => return Err(uncertain("Windows durable create-new failed")),
        };
        file.write_all(bytes)
            .map_err(|_| uncertain("Windows durable repository write failed"))?;
        file.sync_all()
            .map_err(|_| uncertain("Windows durable repository file flush failed"))?;
        regular_file_snapshot(&file)?;
        sync_windows_directory(
            path.parent()
                .ok_or_else(|| invalid_path("repository file has no parent"))?,
        )
    }

    pub(super) fn rename_noreplace(
        root: &RetainedRepositoryRootV1,
        source: &ValidatedRelativePath,
        target: &ValidatedRelativePath,
    ) -> Result<(), RepositoryDurabilityErrorV1> {
        rename_noreplace_expected(root, source, target, None)
    }

    pub(super) fn rename_noreplace_expected(
        root: &RetainedRepositoryRootV1,
        source: &ValidatedRelativePath,
        target: &ValidatedRelativePath,
        expected_source: Option<RegularFileSnapshot>,
    ) -> Result<(), RepositoryDurabilityErrorV1> {
        let source_guards = root.pin_windows_parent(source)?;
        let target_guards = root.pin_windows_parent(target)?;
        let retained = open_regular_optional(root, source, false)?
            .ok_or_else(|| not_found("rename source is absent"))?;
        let expected = regular_file_snapshot(&retained)?;
        if expected_source.is_some_and(|identity| identity != expected) {
            return Err(unsafe_fs(
                "rename source changed after exact retained admission",
            ));
        }
        let source_path = root.absolute(source);
        let target_path = root.absolute(target);
        if fs::symlink_metadata(&target_path).is_ok() {
            return Err(already_exists("rename target already exists"));
        }
        fs::rename(&source_path, &target_path)
            .map_err(|_| uncertain("Windows pinned no-replace rename failed"))?;
        let installed = open_regular_optional(root, target, false)?
            .ok_or_else(|| uncertain("renamed target is absent after publication"))?;
        let installed = regular_file_snapshot(&installed)?;
        if installed.identity != expected.identity || installed.length != expected.length {
            return Err(unsafe_fs(
                "renamed target is not the retained source identity",
            ));
        }
        sync_windows_directory(
            source_path
                .parent()
                .ok_or_else(|| invalid_path("rename source has no parent"))?,
        )?;
        sync_windows_directory(
            target_path
                .parent()
                .ok_or_else(|| invalid_path("rename target has no parent"))?,
        )?;
        drop((source_guards, target_guards));
        Ok(())
    }

    pub(super) fn rename_directory_noreplace(
        root: &RetainedRepositoryRootV1,
        source: &ValidatedRelativePath,
        target: &ValidatedRelativePath,
    ) -> Result<(), RepositoryDurabilityErrorV1> {
        let source_guards = root.pin_windows_parent(source)?;
        let target_guards = root.pin_windows_parent(target)?;
        let source_path = root.absolute(source);
        let target_path = root.absolute(target);
        let retained = open_windows_deletable_directory(&source_path)?;
        let expected = directory_identity(&retained)?;
        if fs::symlink_metadata(&target_path).is_ok() {
            return Err(already_exists("directory rename target already exists"));
        }
        fs::rename(&source_path, &target_path)
            .map_err(|_| uncertain("Windows pinned directory no-replace rename failed"))?;
        let installed = open_windows_directory(&target_path)?;
        if directory_identity(&installed)? != expected {
            return Err(unsafe_fs(
                "renamed directory is not the retained source identity",
            ));
        }
        sync_windows_directory(
            source_path
                .parent()
                .ok_or_else(|| invalid_path("directory rename source has no parent"))?,
        )?;
        sync_windows_directory(
            target_path
                .parent()
                .ok_or_else(|| invalid_path("directory rename target has no parent"))?,
        )?;
        drop((source_guards, target_guards, retained));
        Ok(())
    }

    pub(super) fn remove_regular(
        root: &RetainedRepositoryRootV1,
        relative: &ValidatedRelativePath,
    ) -> Result<(), RepositoryDurabilityErrorV1> {
        let _guards = root.pin_windows_parent(relative)?;
        let retained = open_regular_optional(root, relative, false)?
            .ok_or_else(|| not_found("repository file is absent"))?;
        let expected = regular_file_snapshot(&retained)?;
        let path = root.absolute(relative);
        let current = open_regular_optional(root, relative, false)?
            .ok_or_else(|| not_found("repository file disappeared before removal"))?;
        if regular_file_snapshot(&current)? != expected {
            return Err(unsafe_fs("repository file identity changed before removal"));
        }
        fs::remove_file(&path)
            .map_err(|_| uncertain("Windows retained regular-file removal failed"))?;
        sync_windows_directory(
            path.parent()
                .ok_or_else(|| invalid_path("repository file has no parent"))?,
        )
    }

    pub(super) fn remove_empty_directory(
        root: &RetainedRepositoryRootV1,
        relative: &ValidatedRelativePath,
    ) -> Result<(), RepositoryDurabilityErrorV1> {
        let _parents = root.pin_windows_parent(relative)?;
        let path = root.absolute(relative);
        let retained = open_windows_deletable_directory(&path)?;
        fs::remove_dir(&path)
            .map_err(|_| uncertain("Windows retained empty-directory removal failed"))?;
        sync_windows_directory(
            path.parent()
                .ok_or_else(|| invalid_path("repository directory has no parent"))?,
        )?;
        drop(retained);
        Ok(())
    }

    fn sync_windows_directory(path: &Path) -> Result<(), RepositoryDurabilityErrorV1> {
        let directory = OpenOptions::new()
            .write(true)
            .share_mode(FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE)
            .custom_flags(FILE_FLAG_BACKUP_SEMANTICS | FILE_FLAG_OPEN_REPARSE_POINT)
            .open(path)
            .map_err(|_| uncertain("Windows directory durable-open failed"))?;
        directory
            .sync_all()
            .map_err(|_| uncertain("Windows directory durable flush failed"))
    }
}

#[cfg(all(not(unix), not(windows)))]
mod platform {
    compile_error!("repository durability is supported only on Unix and Windows");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(unix)]
    #[test]
    fn stable_tree_epoch_refuses_intermediate_authority_directory_aba() {
        let repository = tempfile::tempdir().unwrap();
        let state = repository.path().join(".handbook/state");
        let authority = state.join("authority");
        let substitute = state.join("substitute");
        let parked = state.join("parked");
        std::fs::create_dir_all(&authority).unwrap();
        std::fs::create_dir(&substitute).unwrap();
        let root = RetainedRepositoryRootV1::open_exact(repository.path()).unwrap();

        let guard = root.retain_tree_guard(".handbook/state", 16).unwrap();
        let result = guard.with_stable_path(|_| {
            std::fs::rename(&authority, &parked).unwrap();
            std::fs::rename(&substitute, &authority).unwrap();
            std::fs::rename(&authority, &substitute).unwrap();
            std::fs::rename(&parked, &authority).unwrap();
        });

        assert_eq!(
            result.unwrap_err().kind(),
            RepositoryDurabilityErrorKindV1::UnsafeFilesystem
        );
    }

    #[cfg(windows)]
    #[test]
    fn retained_windows_root_cannot_be_renamed_while_guarded() {
        let parent = tempfile::tempdir().unwrap();
        let authority = parent.path().join("authority");
        let moved = parent.path().join("moved");
        std::fs::create_dir(&authority).unwrap();
        let root = RetainedRepositoryRootV1::open_exact(&authority).unwrap();

        assert!(std::fs::rename(&authority, &moved).is_err());
        root.require_stable_path_epoch().unwrap();
    }

    #[cfg(windows)]
    #[test]
    fn retained_windows_tree_prevents_intermediate_directory_substitution() {
        let repository = tempfile::tempdir().unwrap();
        let authority = repository.path().join(".handbook/state/authority");
        let moved = repository.path().join(".handbook/state/moved");
        std::fs::create_dir_all(&authority).unwrap();
        let root = RetainedRepositoryRootV1::open_exact(repository.path()).unwrap();

        root.retain_tree_guard(".handbook/state", 16)
            .unwrap()
            .with_stable_path(|_| {
                assert!(std::fs::rename(&authority, &moved).is_err());
            })
            .unwrap();
    }

    #[test]
    fn retained_tree_refuses_in_place_regular_file_mutation() {
        let repository = tempfile::tempdir().unwrap();
        let head = repository
            .path()
            .join(".handbook/state/authority/head.json");
        std::fs::create_dir_all(head.parent().unwrap()).unwrap();
        std::fs::write(&head, b"original").unwrap();
        let root = RetainedRepositoryRootV1::open_exact(repository.path()).unwrap();
        let guard = root.retain_tree_guard(".handbook/state", 16).unwrap();

        std::fs::write(&head, b"mutated!").unwrap();

        assert_eq!(
            guard.validate().unwrap_err().kind(),
            RepositoryDurabilityErrorKindV1::UnsafeFilesystem
        );
    }

    #[test]
    fn authority_guard_excludes_owner_mutable_contract_subtree() {
        let repository = tempfile::tempdir().unwrap();
        let contract = repository
            .path()
            .join(".handbook/state/contracts/v1/head.json");
        let locks = repository.path().join(".handbook/state/locks");
        std::fs::create_dir_all(contract.parent().unwrap()).unwrap();
        std::fs::create_dir_all(&locks).unwrap();
        std::fs::write(&contract, b"old-head").unwrap();
        std::fs::write(locks.join("promotion.lock"), b"").unwrap();
        std::fs::write(locks.join("registry.lock"), b"").unwrap();
        let root = RetainedRepositoryRootV1::open_exact(repository.path()).unwrap();
        let guard = root.retain_authority_tree_guard(32).unwrap();

        std::fs::write(&contract, b"new-head").unwrap();

        guard.validate().unwrap();
    }
}
