//! Root-relative database capability. The reusable crate never receives an
//! ambient repository path or a Handbook permission/authority token.
use crate::{RepositoryExclusiveLockV1, RepositoryReadV1, RetainedRepositoryRootV1};
use file_native::{Backend, Storage};
#[derive(Clone)]
pub(super) struct WorkFs<'a> {
    pub root: &'a RetainedRepositoryRootV1,
    pub prefix: String,
    pub physical: String,
}
impl WorkFs<'_> {
    fn path(&self, p: &str) -> String {
        format!("{}/{p}", self.prefix)
    }
}
impl Storage for WorkFs<'_> {
    fn validate(&self) -> file_native::Result<()> {
        if self.root.governance_physical_root_fingerprint_v1()? != self.physical {
            return Err("original work physical root required".into());
        }
        Ok(())
    }
    fn read(&self, p: &str, n: usize) -> file_native::Result<Option<Vec<u8>>> {
        self.validate()?;
        Ok(
            match self
                .root
                .read_local_owner_optional_bounded(&self.path(p), n)?
            {
                RepositoryReadV1::Absent => None,
                RepositoryReadV1::Found(b) => Some(b),
            },
        )
    }
    fn mkdir(&self, p: &str) -> file_native::Result<()> {
        self.validate()?;
        self.root.create_directories(&self.path(p))?;
        Ok(())
    }
    fn write_new(&self, p: &str, b: &[u8]) -> file_native::Result<()> {
        self.validate()?;
        self.root.write_new_durable(&self.path(p), b)?;
        self.validate()
    }
    fn rename_new(&self, a: &str, b: &str) -> file_native::Result<()> {
        self.validate()?;
        self.root
            .rename_noreplace_durable(&self.path(a), &self.path(b))?;
        self.validate()
    }
    fn displace(&self, a: &str, expected: &[u8], b: &str) -> file_native::Result<()> {
        self.validate()?;
        self.root
            .displace_exact_durable(&self.path(a), expected, &self.path(b))?;
        self.validate()
    }
    fn remove(&self, p: &str) -> file_native::Result<()> {
        self.validate()?;
        if self.read(p, 16 * 1024 * 1024)?.is_some() {
            self.root.remove_regular_durable(&self.path(p))?;
        }
        self.validate()
    }
    fn directory_sync_supported(&self) -> bool {
        cfg!(unix)
    }
}
impl Backend for WorkFs<'_> {
    type Guard = RepositoryExclusiveLockV1;
    fn lock(&self, p: &str) -> file_native::Result<Self::Guard> {
        self.validate()?;
        Ok(self
            .root
            .acquire_existing_exclusive_lock_v1(&self.path(p))?)
    }
    fn try_lock(&self, p: &str) -> file_native::Result<Option<Self::Guard>> {
        self.validate()?;
        Ok(self
            .root
            .try_acquire_existing_exclusive_lock_v1(&self.path(p))?)
    }
    fn lock_identity(&self, g: &Self::Guard) -> file_native::Result<String> {
        Ok(g.physical_fingerprint_v1()?)
    }
    fn validate_lock(&self, p: &str, g: &Self::Guard) -> file_native::Result<()> {
        g.validate_at(self.root, &self.path(p))?;
        self.validate()
    }
}
pub(super) fn db_error(e: impl std::fmt::Display) -> super::LocalAuthorityRefusalV1 {
    crate::local_authority::refuse("work_database", &e.to_string())
}
