//! Fixed retained-root capability for the project-governance owner.
//!
//! Contracts owns record meanings. A writer retains the original initialization
//! guard. A separate read/pin handle can outlive that guard but cannot mutate
//! publication or maintenance state.
use crate::{
    RepositoryExclusiveLockV1, RepositoryInitializationClaimV1, RepositoryInitializationGuardV1,
    RepositoryReadV1, RetainedRepositoryRootV1, REPOSITORY_INITIALIZATION_CLAIM_PATH_V1,
};
use file_native::{Backend, Storage};
use std::sync::Arc;

#[cfg(feature = "test-support")]
pub use file_native::{CreationBoundary, PublicationBoundary};
pub use file_native::{
    Database, EvidenceObservation, MaintenanceBudget, MaintenanceProgress, OriginalDisposition,
    OriginalEvidence, OriginalOutcome, OriginalResolution, ReadView, Transaction, Version,
};

const PREFIX: &str = ".handbook/admission-governance/v2/database";
const NAMESPACE: &str = "handbook.project-governance@1";

/// The writer borrows the original guard. The detached reader has no guard and
/// can mutate only native reader-pin files, never selection or owner records.
#[derive(Clone)]
pub struct RetainedDatabaseFs<'a> {
    root: Arc<RetainedRepositoryRootV1>,
    guard: Option<&'a RepositoryInitializationGuardV1>,
    claim: RepositoryInitializationClaimV1,
    claim_record: Vec<u8>,
    physical: String,
}

impl RetainedDatabaseFs<'_> {
    fn path(&self, path: &str) -> String {
        format!("{PREFIX}/{path}")
    }

    fn allow_write(&self, path: &str) -> file_native::Result<()> {
        if self.guard.is_some() {
            return Ok(());
        }
        let valid = path
            .strip_prefix("readers/")
            .and_then(|rest| {
                let (slot, suffix) = rest.split_once(".json")?;
                let number = slot.parse::<usize>().ok()?;
                Some(
                    number < file_native::MAX_READERS
                        && number.to_string() == slot
                        && matches!(
                            suffix,
                            "" | ".prepared" | ".displaced" | ".prepared.writing"
                        ),
                )
            })
            .unwrap_or(false);
        if !valid {
            return Err("detached governance reader cannot mutate publication state".into());
        }
        Ok(())
    }
}

impl Storage for RetainedDatabaseFs<'_> {
    fn validate(&self) -> file_native::Result<()> {
        if self.root.governance_physical_root_fingerprint_v1()? != self.physical
            || self.claim.physical_root_fingerprint() != self.physical
            || self
                .root
                .read_regular_bounded(REPOSITORY_INITIALIZATION_CLAIM_PATH_V1, 1024 * 1024)?
                != RepositoryReadV1::Found(self.claim_record.clone())
        {
            return Err("original governance claim and physical root required".into());
        }
        if let Some(guard) = self.guard {
            guard.validate()?;
            if !guard.root().same_physical_root(&self.root)
                || !guard.observe_native_claim(&self.claim)?
            {
                return Err("original held governance initialization guard required".into());
            }
        }
        Ok(())
    }

    fn read(&self, path: &str, limit: usize) -> file_native::Result<Option<Vec<u8>>> {
        self.validate()?;
        Ok(
            match self
                .root
                .read_local_owner_optional_bounded(&self.path(path), limit)?
            {
                RepositoryReadV1::Absent => None,
                RepositoryReadV1::Found(bytes) => Some(bytes),
            },
        )
    }

    fn mkdir(&self, path: &str) -> file_native::Result<()> {
        self.validate()?;
        if self.guard.is_none() {
            if path != "readers" {
                return Err("detached governance reader cannot create directories".into());
            }
            self.root.list_directory_bounded(&self.path(path), 4096)?;
        } else {
            self.root.create_directories(&self.path(path))?;
        }
        self.validate()
    }

    fn write_new(&self, path: &str, bytes: &[u8]) -> file_native::Result<()> {
        self.validate()?;
        self.allow_write(path)?;
        self.root.write_new_durable(&self.path(path), bytes)?;
        self.validate()
    }

    fn rename_new(&self, from: &str, to: &str) -> file_native::Result<()> {
        self.validate()?;
        self.allow_write(from)?;
        self.allow_write(to)?;
        self.root
            .rename_noreplace_durable(&self.path(from), &self.path(to))?;
        self.validate()
    }

    fn displace(&self, from: &str, expected: &[u8], to: &str) -> file_native::Result<()> {
        self.validate()?;
        self.allow_write(from)?;
        self.allow_write(to)?;
        self.root
            .displace_exact_durable(&self.path(from), expected, &self.path(to))?;
        self.validate()
    }

    fn remove(&self, path: &str) -> file_native::Result<()> {
        self.validate()?;
        self.allow_write(path)?;
        if self.read(path, 16 * 1024 * 1024)?.is_some() {
            self.root.remove_regular_durable(&self.path(path))?;
        }
        self.validate()
    }

    fn directory_sync_supported(&self) -> bool {
        cfg!(unix)
    }
}

impl Backend for RetainedDatabaseFs<'_> {
    type Guard = RepositoryExclusiveLockV1;
    fn lock(&self, path: &str) -> file_native::Result<Self::Guard> {
        self.validate()?;
        Ok(self
            .root
            .acquire_existing_exclusive_lock_v1(&self.path(path))?)
    }
    fn try_lock(&self, path: &str) -> file_native::Result<Option<Self::Guard>> {
        self.validate()?;
        Ok(self
            .root
            .try_acquire_existing_exclusive_lock_v1(&self.path(path))?)
    }
    fn lock_identity(&self, guard: &Self::Guard) -> file_native::Result<String> {
        Ok(guard.physical_fingerprint_v1()?)
    }
    fn validate_lock(&self, path: &str, guard: &Self::Guard) -> file_native::Result<()> {
        guard.validate_at(&self.root, &self.path(path))?;
        self.validate()
    }
}

/// Opens one fixed database under the exact initialization claim. The first
/// handle cannot outlive its guard. The second supports detached reads and pins
/// but refuses all native selection-changing operations. The owner must prove
/// its creation/recovery prefix before selecting a mode.
pub fn open_governance_database<'a>(
    guard: &'a RepositoryInitializationGuardV1,
    claim: &RepositoryInitializationClaimV1,
    mode: &str,
) -> file_native::Result<(
    Database<RetainedDatabaseFs<'a>>,
    Database<RetainedDatabaseFs<'static>>,
)> {
    guard.validate()?;
    if !guard.observe_native_claim(claim)? {
        return Err("original governance claim required".into());
    }
    let root = Arc::new(RetainedRepositoryRootV1::open_exact(
        guard.root().initialization_root_path(),
    )?);
    if !guard.root().same_physical_root(&root) {
        return Err("retained governance root differs".into());
    }
    let physical = root.governance_physical_root_fingerprint_v1()?;
    let claim_record =
        match root.read_regular_bounded(REPOSITORY_INITIALIZATION_CLAIM_PATH_V1, 1024 * 1024)? {
            RepositoryReadV1::Found(bytes) => bytes,
            RepositoryReadV1::Absent => return Err("original governance claim missing".into()),
        };
    let writer = RetainedDatabaseFs {
        root: root.clone(),
        guard: Some(guard),
        claim: claim.clone(),
        claim_record: claim_record.clone(),
        physical: physical.clone(),
    };
    let reader = RetainedDatabaseFs {
        root,
        guard: None,
        claim: claim.clone(),
        claim_record,
        physical,
    };
    writer.validate()?;
    let database = match mode {
        "create" => {
            writer.root.create_directories(PREFIX)?;
            Database::create(writer, NAMESPACE)?
        }
        "resume_creation" => Database::resume_creation(writer, NAMESPACE)?,
        "open" => Database::open(writer, NAMESPACE)?,
        "recover" => Database::open_for_recovery(writer, NAMESPACE)?,
        _ => return Err("unsupported governance database open mode".into()),
    };
    let detached = if mode == "recover" {
        Database::open_for_recovery(reader, NAMESPACE)?
    } else {
        Database::open(reader, NAMESPACE)?
    };
    guard.validate()?;
    Ok((database, detached))
}

/// Test-only native creation cut using the same fixed root, claim and namespace.
#[cfg(feature = "test-support")]
pub fn open_governance_database_with_creation_boundary<'a>(
    guard: &'a RepositoryInitializationGuardV1,
    claim: &RepositoryInitializationClaimV1,
    boundary: impl FnMut(CreationBoundary) -> file_native::Result<()>,
) -> file_native::Result<(
    Database<RetainedDatabaseFs<'a>>,
    Database<RetainedDatabaseFs<'static>>,
)> {
    guard.validate()?;
    if !guard.observe_native_claim(claim)? {
        return Err("original governance claim required".into());
    }
    let root = Arc::new(RetainedRepositoryRootV1::open_exact(
        guard.root().initialization_root_path(),
    )?);
    if !guard.root().same_physical_root(&root) {
        return Err("retained governance root differs".into());
    }
    let physical = root.governance_physical_root_fingerprint_v1()?;
    let claim_record = match root.read_regular_bounded(
        REPOSITORY_INITIALIZATION_CLAIM_PATH_V1, 1024 * 1024,
    )? {
        RepositoryReadV1::Found(bytes) => bytes,
        RepositoryReadV1::Absent => return Err("original governance claim missing".into()),
    };
    let writer = RetainedDatabaseFs {
        root: root.clone(),
        guard: Some(guard),
        claim: claim.clone(),
        claim_record: claim_record.clone(),
        physical: physical.clone(),
    };
    let reader = RetainedDatabaseFs {
        root, guard: None, claim: claim.clone(), claim_record, physical,
    };
    writer.validate()?;
    writer.root.create_directories(PREFIX)?;
    let database = Database::create_with_boundary(writer, NAMESPACE, boundary)?;
    let detached = Database::open(reader, NAMESPACE)?;
    guard.validate()?;
    Ok((database, detached))
}
