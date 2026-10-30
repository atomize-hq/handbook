use crate::Result;
use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::{Path, PathBuf},
    sync::Arc,
};

/// A retained, owner-scoped native IO capability. Implementations must reject
/// symlinks/special files and preserve the physical root and lock identities.
/// Paths are generated internally, relative, and never caller record keys.
pub trait Storage {
    fn validate(&self) -> Result<()>;
    fn read(&self, path: &str, limit: usize) -> Result<Option<Vec<u8>>>;
    fn mkdir(&self, path: &str) -> Result<()>;
    fn write_new(&self, path: &str, bytes: &[u8]) -> Result<()>;
    fn rename_new(&self, from: &str, to: &str) -> Result<()>;
    fn displace(&self, from: &str, expected: &[u8], to: &str) -> Result<()>;
    fn remove(&self, path: &str) -> Result<()>;
    fn directory_sync_supported(&self) -> bool;
}
/// Native locks must be released when their guard is dropped or process exits.
pub trait Backend: Storage + Clone {
    type Guard;
    fn lock(&self, path: &str) -> Result<Self::Guard>;
    fn try_lock(&self, path: &str) -> Result<Option<Self::Guard>>;
    fn validate_lock(&self, path: &str, guard: &Self::Guard) -> Result<()>;
    fn lock_identity(&self, guard: &Self::Guard) -> Result<String>;
}

#[cfg(unix)]
fn identity(file: &File) -> Result<String> {
    use std::os::unix::fs::MetadataExt;
    let m = file.metadata()?;
    Ok(format!("{}:{}", m.dev(), m.ino()))
}
#[cfg(windows)]
fn identity(file: &File) -> Result<String> {
    use std::os::windows::io::AsRawHandle;
    #[repr(C)]
    struct Info {
        attributes: u32,
        creation: [u32; 2],
        access: [u32; 2],
        write: [u32; 2],
        volume: u32,
        size_high: u32,
        size_low: u32,
        links: u32,
        index_high: u32,
        index_low: u32,
    }
    #[link(name = "kernel32")]
    extern "system" {
        fn GetFileInformationByHandle(file: *mut std::ffi::c_void, info: *mut Info) -> i32;
    }
    let mut info = std::mem::MaybeUninit::<Info>::uninit();
    if unsafe { GetFileInformationByHandle(file.as_raw_handle(), info.as_mut_ptr()) } == 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    let i = unsafe { info.assume_init() };
    Ok(format!("{}:{}:{}", i.volume, i.index_high, i.index_low))
}

fn lock_generation(file: &File) -> Result<String> {
    let metadata = file.metadata()?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        return Ok(format!(
            "{}:{}:{}:{}:{}:{}",
            identity(file)?,
            metadata.len(),
            metadata.ctime(),
            metadata.ctime_nsec(),
            metadata.mtime(),
            metadata.mtime_nsec()
        ));
    }
    #[cfg(windows)]
    {
        Ok(format!(
            "{}:{}:{:?}:{:?}",
            identity(file)?,
            metadata.len(),
            metadata.created()?,
            metadata.modified()?
        ))
    }
}
struct Root {
    path: PathBuf,
    file: File,
    identity: String,
}
/// Default native backend. The Engine can supply its stronger retained-root
/// capability through Backend without passing an ambient repository path.
#[derive(Clone)]
pub struct NativeBackend {
    root: Arc<Root>,
    #[cfg(feature = "test-support")]
    after_create: Option<Arc<dyn Fn(&str) -> Result<()> + Send + Sync>>,
}
pub struct NativeGuard {
    file: File,
    identity: String,
    #[cfg(windows)]
    _parents: Vec<File>,
}
impl NativeBackend {
    /// Opens an existing directory, never initializes a database implicitly.
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let path = fs::canonicalize(path)?;
        let file = open_directory(&path)?;
        let id = identity(&file)?;
        Ok(Self {
            root: Arc::new(Root {
                path,
                file,
                identity: id,
            }),
            #[cfg(feature = "test-support")]
            after_create: None,
        })
    }
    /// Observes the actual exclusive-create prefix for valid process-death
    /// tests. Default production construction has no observer.
    #[cfg(feature = "test-support")]
    pub fn with_create_observer(
        mut self,
        observer: impl Fn(&str) -> Result<()> + Send + Sync + 'static,
    ) -> Self {
        self.after_create = Some(Arc::new(observer));
        self
    }
    fn observe_created(&self, path: &str) -> Result<()> {
        #[cfg(feature = "test-support")]
        if let Some(observer) = &self.after_create {
            observer(path)?;
        }
        let _ = path;
        Ok(())
    }
    #[cfg(windows)]
    fn path(&self, relative: &str) -> Result<PathBuf> {
        self.validate()?;
        if relative.is_empty()
            || relative.len() > 2048
            || relative
                .split('/')
                .any(|s| s.is_empty() || s == "." || s == "..")
        {
            return Err("invalid storage path".into());
        }
        let mut path = self.root.path.clone();
        for part in Path::new(relative).components() {
            let std::path::Component::Normal(part) = part else {
                return Err("invalid storage component".into());
            };
            path.push(part);
            match fs::symlink_metadata(&path) {
                Ok(m) if m.file_type().is_symlink() || !(m.is_dir() || m.is_file()) => {
                    return Err("unsafe storage entry".into())
                }
                Ok(_) => {}
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
                Err(e) => return Err(e.into()),
            }
        }
        Ok(path)
    }
    #[cfg(windows)]
    fn sync_parent(&self, path: &Path) -> Result<()> {
        sync_directory(path.parent().ok_or("missing parent")?)?;
        self.validate()
    }
    fn validate_root(&self) -> Result<()> {
        if identity(&self.root.file)? != self.root.identity
            || identity(&open_directory(&self.root.path)?)? != self.root.identity
        {
            return Err("storage root substituted".into());
        }
        Ok(())
    }
    #[cfg(windows)]
    fn guarded_path(&self, relative: &str) -> Result<(Vec<File>, PathBuf)> {
        let path = self.path(relative)?;
        let mut current = self.root.path.clone();
        let mut guards = Vec::new();
        let parts: Vec<_> = Path::new(relative).components().collect();
        for part in &parts[..parts.len() - 1] {
            current.push(part.as_os_str());
            guards.push(open_directory(&current)?);
        }
        Ok((guards, path))
    }
    fn open_lock(&self, path: &str) -> Result<NativeGuard> {
        #[cfg(unix)]
        let f = unix::open_file(self, path, true)?;
        #[cfg(windows)]
        let (parents, path) = self.guarded_path(path)?;
        #[cfg(windows)]
        let f = open_regular(&path, true)?;
        if f.metadata()?.len() != 0 {
            return Err("lock must be empty".into());
        }
        let id = lock_generation(&f)?;
        Ok(NativeGuard {
            file: f,
            identity: id,
            #[cfg(windows)]
            _parents: parents,
        })
    }
}
fn open_directory(path: &Path) -> Result<File> {
    let mut o = OpenOptions::new();
    o.read(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        o.custom_flags(libc::O_DIRECTORY | libc::O_NOFOLLOW);
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs::OpenOptionsExt;
        o.custom_flags(0x02000000 | 0x00200000).share_mode(3);
    }
    let file = o.open(path)?;
    if !file.metadata()?.is_dir() {
        return Err("expected directory".into());
    }
    Ok(file)
}
#[cfg(windows)]
fn open_regular(path: &Path, write: bool) -> Result<File> {
    let mut o = OpenOptions::new();
    o.read(true).write(write);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        o.custom_flags(libc::O_NOFOLLOW);
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs::OpenOptionsExt;
        o.custom_flags(0x00200000);
    }
    let f = o.open(path)?;
    let m = f.metadata()?;
    if !m.is_file() {
        return Err("expected regular storage file".into());
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        if m.nlink() != 1 {
            return Err("linked storage file".into());
        }
    }
    Ok(f)
}
#[cfg(windows)]
fn sync_directory(_path: &Path) -> Result<()> {
    Ok(())
}
#[cfg(windows)]
impl Storage for NativeBackend {
    fn validate(&self) -> Result<()> {
        self.validate_root()
    }
    fn read(&self, path: &str, limit: usize) -> Result<Option<Vec<u8>>> {
        let (_parents, path) = match self.guarded_path(path) {
            Ok(p) => p,
            Err(e)
                if e.downcast_ref::<std::io::Error>()
                    .is_some_and(|e| e.kind() == std::io::ErrorKind::NotFound) =>
            {
                return Ok(None)
            }
            Err(e) => return Err(e),
        };
        let mut f = match open_regular(&path, false) {
            Ok(f) => f,
            Err(e)
                if e.downcast_ref::<std::io::Error>()
                    .is_some_and(|e| e.kind() == std::io::ErrorKind::NotFound) =>
            {
                return Ok(None)
            }
            Err(e) => return Err(e),
        };
        let n = f.metadata()?.len();
        if n > limit as u64 {
            return Err("file read bound".into());
        }
        let before = identity(&f)?;
        let mut b = Vec::with_capacity(n as usize);
        std::io::Read::by_ref(&mut f)
            .take(limit as u64 + 1)
            .read_to_end(&mut b)?;
        if b.len() > limit
            || b.len() as u64 != n
            || identity(&open_regular(&path, false)?)? != before
        {
            return Err("storage file changed during read".into());
        }
        self.validate()?;
        Ok(Some(b))
    }
    fn mkdir(&self, path: &str) -> Result<()> {
        let path = self.path(path)?;
        let mut guards = Vec::new();
        let relative = path.strip_prefix(&self.root.path)?;
        let mut current = self.root.path.clone();
        for component in relative.components() {
            current.push(component);
            match fs::create_dir(&current) {
                Ok(()) => sync_directory(current.parent().unwrap())?,
                Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {}
                Err(e) => return Err(e.into()),
            }
            guards.push(open_directory(&current)?);
        }
        self.validate()
    }
    fn write_new(&self, path: &str, bytes: &[u8]) -> Result<()> {
        let relative = path;
        let (_parents, path) = self.guarded_path(path)?;
        let mut o = OpenOptions::new();
        o.write(true).create_new(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            o.custom_flags(libc::O_NOFOLLOW);
        }
        let mut f = o.open(&path)?;
        self.observe_created(relative)?;
        f.write_all(bytes)?;
        f.sync_all()?;
        drop(f);
        self.sync_parent(&path)
    }
    fn rename_new(&self, from: &str, to: &str) -> Result<()> {
        let (_source_parents, source) = self.guarded_path(from)?;
        let (_target_parents, target) = self.guarded_path(to)?;
        use std::os::windows::ffi::OsStrExt;
        #[link(name = "kernel32")]
        extern "system" {
            fn MoveFileExW(a: *const u16, b: *const u16, flags: u32) -> i32;
        }
        let a: Vec<u16> = source.as_os_str().encode_wide().chain(Some(0)).collect();
        let b: Vec<u16> = target.as_os_str().encode_wide().chain(Some(0)).collect();
        if unsafe { MoveFileExW(a.as_ptr(), b.as_ptr(), 0) } == 0 {
            return Err(std::io::Error::last_os_error().into());
        }
        self.sync_parent(&source)?;
        self.sync_parent(&target)
    }
    fn displace(&self, from: &str, expected: &[u8], to: &str) -> Result<()> {
        if self.read(from, expected.len())?.as_deref() != Some(expected) {
            return Err("exact replacement mismatch".into());
        }
        self.rename_new(from, to)
    }
    fn remove(&self, path: &str) -> Result<()> {
        let (_parents, path) = match self.guarded_path(path) {
            Ok(p) => p,
            Err(e)
                if e.downcast_ref::<std::io::Error>()
                    .is_some_and(|e| e.kind() == std::io::ErrorKind::NotFound) =>
            {
                return Ok(())
            }
            Err(e) => return Err(e),
        };
        match fs::remove_file(&path) {
            Ok(()) => self.sync_parent(&path),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(e) => Err(e.into()),
        }
    }
    fn directory_sync_supported(&self) -> bool {
        cfg!(unix)
    }
}
impl Backend for NativeBackend {
    type Guard = NativeGuard;
    fn lock(&self, path: &str) -> Result<Self::Guard> {
        let g = self.open_lock(path)?;
        g.file.lock()?;
        self.validate_lock(path, &g)?;
        Ok(g)
    }
    fn try_lock(&self, path: &str) -> Result<Option<Self::Guard>> {
        let g = self.open_lock(path)?;
        match g.file.try_lock() {
            Ok(()) => {
                self.validate_lock(path, &g)?;
                Ok(Some(g))
            }
            Err(std::fs::TryLockError::WouldBlock) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }
    fn lock_identity(&self, guard: &Self::Guard) -> Result<String> {
        Ok(guard.identity.clone())
    }
    fn validate_lock(&self, path: &str, guard: &Self::Guard) -> Result<()> {
        if lock_generation(&guard.file)? != guard.identity
            || lock_generation(&{
                #[cfg(unix)]
                {
                    unix::open_file(self, path, true)?
                }
                #[cfg(windows)]
                {
                    let (_parents, path) = self.guarded_path(path)?;
                    open_regular(&path, true)?
                }
            })? != guard.identity
        {
            return Err("native lock substituted".into());
        }
        self.validate()
    }
}

#[cfg(unix)]
mod unix {
    use super::*;
    use std::{
        ffi::CString,
        os::fd::{AsRawFd, FromRawFd},
    };
    fn c(s: &str) -> Result<CString> {
        Ok(CString::new(s)?)
    }
    fn admitted_file(fd: i32) -> Result<File> {
        if fd < 0 {
            return Err(std::io::Error::last_os_error().into());
        }
        let f = unsafe { File::from_raw_fd(fd) };
        let m = f.metadata()?;
        use std::os::unix::fs::MetadataExt;
        if !m.is_file() || m.nlink() != 1 {
            return Err("regular single-link object required".into());
        }
        Ok(f)
    }
    fn directory_at(parent: &File, name: &str) -> Result<File> {
        let name = c(name)?;
        let fd = unsafe {
            libc::openat(
                parent.as_raw_fd(),
                name.as_ptr(),
                libc::O_RDONLY | libc::O_DIRECTORY | libc::O_NOFOLLOW | libc::O_CLOEXEC,
            )
        };
        if fd < 0 {
            return Err(std::io::Error::last_os_error().into());
        }
        Ok(unsafe { File::from_raw_fd(fd) })
    }
    fn components(path: &str) -> Result<Vec<&str>> {
        if path.is_empty() || path.len() > 2048 {
            return Err("storage relative path bound".into());
        }
        let parts: Vec<_> = path.split('/').collect();
        if parts
            .iter()
            .any(|s| s.is_empty() || *s == "." || *s == ".." || s.contains('\0'))
        {
            return Err("storage relative path".into());
        }
        Ok(parts)
    }
    fn parent(io: &NativeBackend, path: &str) -> Result<(File, String)> {
        io.validate_root()?;
        let parts = components(path)?;
        let mut dir = io.root.file.try_clone()?;
        for name in &parts[..parts.len() - 1] {
            dir = directory_at(&dir, name)?;
        }
        Ok((dir, parts.last().unwrap().to_string()))
    }
    fn validate_parent(io: &NativeBackend, path: &str, retained: &File) -> Result<()> {
        let (current, _) = parent(io, path)?;
        if identity(&current)? != identity(retained)? {
            return Err("storage parent substituted".into());
        }
        io.validate_root()
    }
    pub(super) fn open_file(io: &NativeBackend, path: &str, write: bool) -> Result<File> {
        let (parent, name) = parent(io, path)?;
        let name = c(&name)?;
        let f = admitted_file(unsafe {
            libc::openat(
                parent.as_raw_fd(),
                name.as_ptr(),
                if write { libc::O_RDWR } else { libc::O_RDONLY }
                    | libc::O_NOFOLLOW
                    | libc::O_CLOEXEC,
            )
        })?;
        validate_parent(io, path, &parent)?;
        Ok(f)
    }
    impl Storage for NativeBackend {
        fn validate(&self) -> Result<()> {
            self.validate_root()
        }
        fn read(&self, path: &str, limit: usize) -> Result<Option<Vec<u8>>> {
            let mut f = match open_file(self, path, false) {
                Ok(f) => f,
                Err(e)
                    if e.downcast_ref::<std::io::Error>()
                        .is_some_and(|e| e.kind() == std::io::ErrorKind::NotFound) =>
                {
                    return Ok(None)
                }
                Err(e) => return Err(e),
            };
            let before = f.metadata()?;
            let n = before.len();
            if n > limit as u64 {
                return Err("bounded native file read".into());
            }
            let mut bytes = Vec::with_capacity(n as usize);
            std::io::Read::by_ref(&mut f)
                .take(limit as u64 + 1)
                .read_to_end(&mut bytes)?;
            let current = open_file(self, path, false)?;
            if bytes.len() as u64 != n
                || identity(&f)? != identity(&current)?
                || current.metadata()?.len() != n
            {
                return Err("native file changed while reading".into());
            }
            self.validate_root()?;
            Ok(Some(bytes))
        }
        fn mkdir(&self, path: &str) -> Result<()> {
            self.validate_root()?;
            let mut dir = self.root.file.try_clone()?;
            for name in components(path)? {
                let next = match directory_at(&dir, name) {
                    Ok(f) => f,
                    Err(e)
                        if e.downcast_ref::<std::io::Error>()
                            .is_some_and(|e| e.kind() == std::io::ErrorKind::NotFound) =>
                    {
                        let namec = c(name)?;
                        if unsafe { libc::mkdirat(dir.as_raw_fd(), namec.as_ptr(), 0o700) } < 0 {
                            let e = std::io::Error::last_os_error();
                            if e.kind() != std::io::ErrorKind::AlreadyExists {
                                return Err(e.into());
                            }
                        }
                        dir.sync_all()?;
                        directory_at(&dir, name)?
                    }
                    Err(e) => return Err(e),
                };
                dir = next;
            }
            let mut current = self.root.file.try_clone()?;
            for name in components(path)? {
                current = directory_at(&current, name)?;
            }
            if identity(&dir)? != identity(&current)? {
                return Err("created parent substituted".into());
            }
            self.validate_root()
        }
        fn write_new(&self, path: &str, bytes: &[u8]) -> Result<()> {
            let (parent, name) = parent(self, path)?;
            let name = c(&name)?;
            let mut f = admitted_file(unsafe {
                libc::openat(
                    parent.as_raw_fd(),
                    name.as_ptr(),
                    libc::O_WRONLY
                        | libc::O_CREAT
                        | libc::O_EXCL
                        | libc::O_NOFOLLOW
                        | libc::O_CLOEXEC,
                    0o600,
                )
            })?;
            self.observe_created(path)?;
            f.write_all(bytes)?;
            f.sync_all()?;
            drop(f);
            parent.sync_all()?;
            validate_parent(self, path, &parent)
        }
        fn rename_new(&self, from: &str, to: &str) -> Result<()> {
            let (source, a) = parent(self, from)?;
            let (target, b) = parent(self, to)?;
            let a = c(&a)?;
            let b = c(&b)?;
            #[cfg(target_os = "linux")]
            let result = unsafe {
                libc::renameat2(
                    source.as_raw_fd(),
                    a.as_ptr(),
                    target.as_raw_fd(),
                    b.as_ptr(),
                    libc::RENAME_NOREPLACE,
                )
            };
            #[cfg(target_os = "macos")]
            let result = unsafe {
                libc::renameatx_np(
                    source.as_raw_fd(),
                    a.as_ptr(),
                    target.as_raw_fd(),
                    b.as_ptr(),
                    libc::RENAME_EXCL,
                )
            };
            #[cfg(not(any(target_os = "linux", target_os = "macos")))]
            return Err("native no-replace platform unsupported".into());
            if result < 0 {
                return Err(std::io::Error::last_os_error().into());
            }
            source.sync_all()?;
            target.sync_all()?;
            validate_parent(self, from, &source)?;
            validate_parent(self, to, &target)
        }
        fn displace(&self, from: &str, expected: &[u8], to: &str) -> Result<()> {
            if self.read(from, expected.len())?.as_deref() != Some(expected) {
                return Err("native exact displacement differs".into());
            }
            self.rename_new(from, to)?;
            if self.read(to, expected.len())?.as_deref() != Some(expected) {
                return Err("native displaced bytes differ".into());
            }
            Ok(())
        }
        fn remove(&self, path: &str) -> Result<()> {
            let (parent, name) = match parent(self, path) {
                Ok(p) => p,
                Err(e)
                    if e.downcast_ref::<std::io::Error>()
                        .is_some_and(|e| e.kind() == std::io::ErrorKind::NotFound) =>
                {
                    return Ok(())
                }
                Err(e) => return Err(e),
            };
            match open_file(self, path, false) {
                Ok(_) => {}
                Err(e)
                    if e.downcast_ref::<std::io::Error>()
                        .is_some_and(|e| e.kind() == std::io::ErrorKind::NotFound) =>
                {
                    return Ok(())
                }
                Err(e) => return Err(e),
            }
            let name = c(&name)?;
            if unsafe { libc::unlinkat(parent.as_raw_fd(), name.as_ptr(), 0) } < 0 {
                return Err(std::io::Error::last_os_error().into());
            }
            parent.sync_all()?;
            validate_parent(self, path, &parent)
        }
        fn directory_sync_supported(&self) -> bool {
            true
        }
    }
}
