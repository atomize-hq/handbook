use super::*;
use super::database::{WorkFs,db_error};
use crate::{RepositoryExclusiveLockV1, RepositoryReadV1, RetainedRepositoryRootV1};
use std::cell::Cell;


pub(super) const LOCK: &str = ".handbook/work-state/work.lock";
const ANCHOR: &str = ".handbook-work-state.v1.json";
const BOOT: &str = ".handbook-work-state-bootstrap.v1.json";
const STAGE: &str = ".handbook/work-state-bootstrap";
const OWNER: &str = ".handbook/work-state/v1/owner.json";
/// Historical grant origins retain their exact raw representation. Only the new
/// kind origin has a closed, purpose-specific discriminant and owner schema.
#[derive(Clone, Debug, PartialEq)]
pub(super) enum BootstrapOrigin {
    Grant(WorkGrantLocatorV1),
    Kind(WorkKindLocatorV1),
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct KindOrigin {
    schema_ref: String,
    kind: WorkKindLocatorV1,
}
impl Serialize for BootstrapOrigin {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Grant(g) => g.serialize(serializer),
            Self::Kind(kind) => KindOrigin {
                schema_ref: "handbook.work-state-bootstrap-kind@2.0.0".into(),
                kind: kind.clone(),
            }
            .serialize(serializer),
        }
    }
}
impl<'de> Deserialize<'de> for BootstrapOrigin {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = serde_json::Value::deserialize(deserializer)?;
        if value
            .as_object()
            .is_some_and(|o| o.contains_key("schema_ref"))
        {
            let origin: KindOrigin =
                serde_json::from_value(value).map_err(serde::de::Error::custom)?;
            if origin.schema_ref != "handbook.work-state-bootstrap-kind@2.0.0" {
                return Err(serde::de::Error::custom("unknown bootstrap origin"));
            }
            Ok(Self::Kind(origin.kind))
        } else {
            serde_json::from_value(value)
                .map(Self::Grant)
                .map_err(serde::de::Error::custom)
        }
    }
}
impl BootstrapOrigin {
    fn validate(&self) -> Result<(), LocalAuthorityRefusalV1> {
        match self {
            Self::Grant(v) => v.validate(),
            Self::Kind(v) => v.validate(),
        }
    }
    fn check(&self) -> &crate::local_authority::local_check::LocalCheckLocatorV1 {
        match self {
            Self::Grant(v) => &v.check,
            Self::Kind(v) => &v.check,
        }
    }
    fn project(&self) -> &str {
        self.check()
            .predecessor
            .charter
            .binding
            .initialization_claim
            .proposed_repository_identity()
    }
    fn schema(&self) -> &'static str {
        match self {
            Self::Grant(_) => "handbook.work-state-owner@1.0.0",
            Self::Kind(_) => "handbook.work-state-owner@2.0.0",
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Owner {
    schema: String,
    pub project: String,
    physical: String,
    initialization: crate::RepositoryInitializationClaimV1,
    bootstrap: BootstrapOrigin,
    lock_fingerprint: String,
    pub identity: String,
}
pub(super) struct WorkLock<'a> {
    pub root: &'a RetainedRepositoryRootV1,
    lock: RepositoryExclusiveLockV1,
    pub owner: Owner,
    pub db: file_native::Database<WorkFs<'a>>,
    proof:crate::RepositoryExactFilesCheckpointV1<'a>,
}
impl WorkLock<'_> {
    pub fn validate(&self) -> Result<(), LocalAuthorityRefusalV1> {
        self.proof.validate().map_err(fs_error)?;
        let entries=self.root.list_directory_bounded(WORK_STATE_ROOT_V1,2).map_err(fs_error)?;
        if entries.len()!=2 || entries.iter().any(|entry| !matches!((entry.name(),entry.kind()),("owner.json",crate::RepositoryDirectoryEntryKindV1::RegularFile)|("database",crate::RepositoryDirectoryEntryKindV1::Directory))) {
            return Err(refuse("work_inventory","closed current-format owner root required"));
        }
        self.lock.validate_at(self.root, LOCK).map_err(fs_error)?;
        if self.lock.physical_fingerprint_v1().map_err(fs_error)? != self.owner.lock_fingerprint
            || required(self.root, OWNER)? != encode(&self.owner)?
            || required(self.root, ANCHOR)? != encode(&self.owner)?
            || required(self.root, BOOT)? != encode(&self.owner.bootstrap)?
            || self
                .root
                .governance_physical_root_fingerprint_v1()
                .map_err(fs_error)?
                != self.owner.physical
        {
            return Err(refuse(
                "work_owner_substituted",
                "original retained physical root/owner/lock required",
            ));
        }
        Ok(())
    }
}
pub(super) fn open<'a>(
    root: &'a RetainedRepositoryRootV1,
) -> Result<WorkLock<'a>, LocalAuthorityRefusalV1> {
    let owner: Owner = decode(&required(root, ANCHOR)?)?;
    owner.initialization.validate().map_err(fs_error)?;
    owner.bootstrap.validate()?;
    if owner.schema != owner.bootstrap.schema()
        || owner.project != owner.bootstrap.project()
        || owner.physical != owner.initialization.physical_root_fingerprint()
        || owner.identity != owner_identity(&owner.bootstrap)?
        || required(root, BOOT)? != encode(&owner.bootstrap)?
    {
        return Err(refuse(
            "work_owner_proof",
            "positive exact initialized owner proof required",
        ));
    }
    let lock = root
        .acquire_existing_exclusive_lock_v1(LOCK)
        .map_err(fs_error)?;
    let db = file_native::Database::open_for_recovery(WorkFs {root,prefix:format!("{WORK_STATE_ROOT_V1}/database"),physical:owner.physical.clone()},&owner.identity).map_err(db_error)?;
    let proof=owner_proof(root)?;
    let result = WorkLock { root, lock, owner, db, proof };
    result.validate()?;
    Ok(result)
}
pub(super) fn owner_identity(origin: &BootstrapOrigin) -> Result<String, LocalAuthorityRefusalV1> {
    fp(
        "handbook.work-state-owner-identity@1.0.0",
        &(
            origin.project(),
            origin
                .check()
                .predecessor
                .charter
                .binding
                .initialization_claim
                .claim_fingerprint(),
        ),
    )
}

// Called only under original live initialization/operating/check admission. The
// external bootstrap intent reserves the original activation across every prefix.
pub(super) fn establish<'a>(
    root: &'a RetainedRepositoryRootV1,
    origin: &BootstrapOrigin,
    mutation_attempted: &Cell<bool>,
    boundary: &mut dyn FnMut(super::WorkWriteBoundaryV1) -> Result<(), LocalAuthorityRefusalV1>,
) -> Result<WorkLock<'a>, LocalAuthorityRefusalV1> {
    if optional(root, ANCHOR)?.is_some() {
        return open(root);
    }
    origin.validate()?;
    let boot = encode(origin)?;
    // Check complete bootstrap/owner preimages before retaining the first
    // external reservation. Native lock fingerprints have this fixed length.
    let candidate = Owner {
        schema: origin.schema().into(),
        project: origin.project().into(),
        physical: root
            .governance_physical_root_fingerprint_v1()
            .map_err(fs_error)?,
        initialization: origin
            .check()
            .predecessor
            .charter
            .binding
            .initialization_claim
            .clone(),
        bootstrap: origin.clone(),
        lock_fingerprint: format!("sha256:{}", "0".repeat(64)),
        identity: owner_identity(origin)?,
    };
    encode(&candidate)?;
    match optional(root, BOOT)? {
        Some(b) if b == boot => {
            if root
                .list_directory_bounded(".handbook/work-state", 64)
                .is_ok()
                || optional(root, &format!("{STAGE}/v1/owner.json"))?.is_none()
            {
                return Err(refuse("work_bootstrap_uncertain","establishment already began but exact original unpublished owner proof is unavailable; no replacement lock or owner"));
            }
        }
        Some(_) => {
            return Err(refuse(
                "work_bootstrap_reserved",
                "only original activation may recover pending owner establishment",
            ))
        }
        None => {
            if root
                .list_directory_bounded(".handbook/work-state", 64)
                .is_ok()
                || root.list_directory_bounded(STAGE, 64).is_ok()
            {
                return Err(refuse(
                    "work_owner_lost",
                    "owner footprint without retained establishment proof is not fresh",
                ));
            }
            mutation_attempted.set(true);
            root.write_new_durable(BOOT, &boot).map_err(fs_error)?;
            boundary(super::WorkWriteBoundaryV1::BootstrapIntent)?;
        }
    }
    mutation_attempted.set(true);
    root.create_directories(&format!("{STAGE}/v1"))
        .map_err(fs_error)?;
    let stage_lock = format!("{STAGE}/work.lock");
    let stage_owner = format!("{STAGE}/v1/owner.json");
    let stage_owner_present = optional(root, &stage_owner)?.is_some();
    let retained = if stage_owner_present {
        root.acquire_existing_exclusive_lock_v1(&stage_lock)
    } else {
        root.acquire_exclusive_lock(&stage_lock)
    }
    .map_err(fs_error)?;
    let owner = Owner {
        lock_fingerprint: retained.physical_fingerprint_v1().map_err(fs_error)?,
        ..candidate
    };
    write_exact(root, &stage_owner, &encode(&owner)?)?;
    let database_path=format!("{STAGE}/v1/database");
    let io=WorkFs {root,prefix:database_path.clone(),physical:owner.physical.clone()};
    if stage_owner_present && optional(root,&format!("{database_path}/CREATING.json"))?.is_some() {
        file_native::Database::resume_creation(io,&owner.identity).map_err(db_error)?;
    } else if stage_owner_present {
        file_native::Database::open(io,&owner.identity).map_err(db_error)?;
    } else {
        root.create_directories(&database_path).map_err(fs_error)?;
        file_native::Database::create(io,&owner.identity).map_err(db_error)?;
    }
    write_exact(root, &stage_owner, &encode(&owner)?)?;
    retained.validate_at(root, &stage_lock).map_err(fs_error)?;
    drop(retained);
    root.rename_directory_noreplace_durable(STAGE, ".handbook/work-state")
        .map_err(fs_error)?;
    write_exact(root, ANCHOR, &encode(&owner)?)?;
    open(root)
}
pub(super) fn optional(
    root: &RetainedRepositoryRootV1,
    p: &str,
) -> Result<Option<Vec<u8>>, LocalAuthorityRefusalV1> {
    match root
        .read_local_owner_optional_bounded(p, MAX)
        .map_err(fs_error)?
    {
        RepositoryReadV1::Absent => Ok(None),
        RepositoryReadV1::Found(v) => Ok(Some(v)),
    }
}
pub(super) fn required(
    root: &RetainedRepositoryRootV1,
    p: &str,
) -> Result<Vec<u8>, LocalAuthorityRefusalV1> {
    optional(root, p)?.ok_or_else(|| {
        refuse(
            "work_proof_missing",
            "required retained proof missing; loss is not absence",
        )
    })
}
pub(super) fn write_exact(
    root: &RetainedRepositoryRootV1,
    p: &str,
    b: &[u8],
) -> Result<(), LocalAuthorityRefusalV1> {
    match optional(root, p)? {
        Some(old) if old == b => Ok(()),
        Some(_) => Err(refuse(
            "work_conflict",
            "exact retained request or owner output conflicts",
        )),
        None => root.write_new_durable(p, b).map_err(fs_error),
    }
}

pub(super) fn validate_reader_owner(root:&RetainedRepositoryRootV1,owner:&Owner)->Result<(),LocalAuthorityRefusalV1>{
    if required(root,OWNER)?!=encode(owner)? || required(root,ANCHOR)?!=encode(owner)? || required(root,BOOT)?!=encode(&owner.bootstrap)? || root.governance_physical_root_fingerprint_v1().map_err(fs_error)?!=owner.physical || root.existing_lock_physical_fingerprint_v1(LOCK).map_err(fs_error)?!=owner.lock_fingerprint {
        return Err(refuse("work_owner_substituted","original retained reader root/owner/lock required"));
    }
    Ok(())
}

pub(super) fn owner_proof(root:&RetainedRepositoryRootV1)->Result<crate::RepositoryExactFilesCheckpointV1<'_>,LocalAuthorityRefusalV1>{
    root.retain_exact_files_checkpoint_v1(&[BOOT,ANCHOR,OWNER]).map_err(fs_error)
}
