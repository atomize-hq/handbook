//! Private held native basis for the purpose-bound work owner. This seam grants
//! no work permission and never reinterprets a local-check action class.
use super::local_check::{self, LocalCheckCurrentBasisV1, LocalCheckLocatorV1};
use super::*;
use crate::repository_durability::{RepositoryDirectoryEntryKindV1, RetainedRepositoryTreeGuardV1};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet, VecDeque};

const PROOF_SCHEMA: &str = "handbook.work-registration-authority-proof@1.0.0";
const ROOTS: [&str; 4] = [
    ".handbook/local-charter/v1",
    ".handbook/local-operating/v1",
    ".handbook/local-operating/local-check-v1",
    ".handbook/admission-governance/v2",
];
const EXACT: [&str; 6] = [
    ".handbook/project/charter.yaml",
    ".handbook/repository-identity.v1",
    ".handbook-initialization.v1.json",
    ".handbook-initialization-receipt.v1.json",
    ".handbook/admission-governance-project.v2.json",
    ".handbook/admission-governance.lock",
];
const INVENTORY_MAX: usize = 4096;

/// Borrows the original native guards. It cannot escape the consuming callback,
/// be cloned, be serialized, or be minted by an SDK consumer.
pub(crate) struct WorkRegistrationCurrentV1<'authority> {
    locator: &'authority LocalCheckLocatorV1,
    basis: &'authority LocalCheckCurrentBasisV1,
    guard: &'authority OperatingGuard,
    tree: &'authority RetainedRepositoryTreeGuardV1<'authority>,
    authority_fingerprint: String,
}

impl WorkRegistrationCurrentV1<'_> {
    pub(crate) fn root(&self) -> &RetainedRepositoryRootV1 {
        self.guard.root()
    }

    pub(crate) fn validate_current(&self) -> Result<(), LocalAuthorityRefusalV1> {
        self.tree.validate().map_err(fs_error)?;
        self.guard.validate()?;
        let current =
            local_check::work_registration_current(self.root(), self.locator, self.basis)?;
        if current.outcome_fingerprint != self.authority_fingerprint {
            return Err(refuse(
                "work_authority_changed",
                "exact held predecessor changed",
            ));
        }
        Ok(())
    }

    /// Copies exact admitted dependencies into a bounded closed proof. Live
    /// current heads are captured evidence only; history never consults them.
    pub(crate) fn capture_proof(
        &self,
    ) -> Result<WorkRegistrationAuthorityProofV1, LocalAuthorityRefusalV1> {
        self.validate_current()?;
        let root = self.root();
        let mut directories = BTreeSet::new();
        let mut paths = BTreeSet::new();
        let mut queue = ROOTS
            .into_iter()
            .map(str::to_owned)
            .collect::<VecDeque<_>>();
        while let Some(directory) = queue.pop_front() {
            directories.insert(directory.clone());
            for entry in root
                .list_directory_bounded(&directory, INVENTORY_MAX)
                .map_err(fs_error)?
            {
                if directories.len() + paths.len() + queue.len() >= INVENTORY_MAX {
                    return Err(refuse(
                        "work_proof_bound",
                        "complete authority inventory exceeds bound",
                    ));
                }
                let path = format!("{directory}/{}", entry.name());
                match entry.kind() {
                    RepositoryDirectoryEntryKindV1::Directory => queue.push_back(path),
                    RepositoryDirectoryEntryKindV1::RegularFile => {
                        paths.insert(path);
                    }
                }
            }
        }
        paths.extend(EXACT.into_iter().map(str::to_owned));
        paths.extend(local_charter::local_evaluation_historical_dependencies(
            root,
            &self.locator.predecessor.charter,
        )?);
        let mut files = Vec::new();
        for path in paths {
            let bytes = found(root, &path)?;
            let byte_fingerprint = DefinitionFingerprint::from_bytes(&bytes).to_string();
            files.push(WorkRegistrationAuthorityFileV1 {
                path,
                content: String::from_utf8(bytes).map_err(codec)?,
                byte_fingerprint,
            });
        }
        let proof = WorkRegistrationAuthorityProofV1 {
            schema_ref: PROOF_SCHEMA.into(),
            locator: self.locator.clone(),
            basis: self.basis.clone(),
            authority_fingerprint: self.authority_fingerprint.clone(),
            initialization_lock_fingerprint: root
                .initialization_lock_physical_fingerprint_v1()
                .map_err(fs_error)?,
            directories: directories.into_iter().collect(),
            files,
        };
        proof.validate_historical(self.locator, self.basis)?;
        self.validate_current()?;
        Ok(proof)
    }
}

/// The work owner must validate its separate fixed establishment/registration
/// permission inside this callback and acquire its own lock after these guards.
/// Terminal publication must finish before returning from `consume`.
pub(crate) fn with_work_registration_current<T>(
    repo: impl AsRef<Path>,
    locator: &LocalCheckLocatorV1,
    basis: &LocalCheckCurrentBasisV1,
    consume: impl FnOnce(&WorkRegistrationCurrentV1<'_>) -> Result<T, LocalAuthorityRefusalV1>,
) -> Result<T, LocalAuthorityRefusalV1> {
    let guard = OperatingGuard::acquire(repo.as_ref(), &locator.predecessor, false)?;
    let outcome = local_check::work_registration_current(guard.root(), locator, basis)?;
    let tree = guard
        .root()
        .retain_local_check_authority_tree_guard()
        .map_err(fs_error)?;
    let admission = WorkRegistrationCurrentV1 {
        locator,
        basis,
        guard: &guard,
        tree: &tree,
        authority_fingerprint: outcome.outcome_fingerprint,
    };
    admission.validate_current()?;
    let value = consume(&admission)?;
    admission.validate_current()?;
    Ok(value)
}

/// The work owner's positive bootstrap creates its sibling partition before
/// retaining authority-tree directory versions. Original native authority locks
/// stay held across bootstrap, tree capture, work-lock acquisition by the
/// consumer, and terminal publication. The work owner validates its fixed human
/// establishment request before invoking this private seam; bootstrap is not an
/// action authorized by LocalCheck terms or by the candidate work grant.
pub(crate) fn with_work_registration_establishment<T>(
    repo: impl AsRef<Path>,
    locator: &LocalCheckLocatorV1,
    basis: &LocalCheckCurrentBasisV1,
    establish_under_guard: impl FnOnce(&RetainedRepositoryRootV1) -> Result<(), LocalAuthorityRefusalV1>,
    consume: impl FnOnce(&WorkRegistrationCurrentV1<'_>) -> Result<T, LocalAuthorityRefusalV1>,
) -> Result<T, LocalAuthorityRefusalV1> {
    let guard = OperatingGuard::acquire(repo.as_ref(), &locator.predecessor, false)?;
    let before = local_check::work_registration_current(guard.root(), locator, basis)?;
    guard.validate()?;
    establish_under_guard(guard.root())?;
    guard.validate()?;
    let after = local_check::work_registration_current(guard.root(), locator, basis)?;
    if after.outcome_fingerprint != before.outcome_fingerprint {
        return Err(refuse(
            "work_authority_changed",
            "exact original authority changed across work-owner bootstrap",
        ));
    }
    let tree = guard
        .root()
        .retain_local_check_authority_tree_guard()
        .map_err(fs_error)?;
    let admission = WorkRegistrationCurrentV1 {
        locator,
        basis,
        guard: &guard,
        tree: &tree,
        authority_fingerprint: after.outcome_fingerprint,
    };
    admission.validate_current()?;
    let value = consume(&admission)?;
    admission.validate_current()?;
    Ok(value)
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct WorkRegistrationAuthorityProofV1 {
    schema_ref: String,
    locator: LocalCheckLocatorV1,
    basis: LocalCheckCurrentBasisV1,
    authority_fingerprint: String,
    initialization_lock_fingerprint: String,
    directories: Vec<String>,
    files: Vec<WorkRegistrationAuthorityFileV1>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum WorkRegistrationRelationKindV1 {
    Deferred,
    EstablishedFromDeferred,
}

impl WorkRegistrationRelationKindV1 {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Deferred => "deferred",
            Self::EstablishedFromDeferred => "established_from_deferred",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct WorkRegistrationAuthorityFileV1 {
    path: String,
    content: String,
    byte_fingerprint: String,
}

impl WorkRegistrationAuthorityProofV1 {
    pub(crate) fn locator(&self) -> &LocalCheckLocatorV1 {
        &self.locator
    }

    pub(crate) fn current_basis(&self) -> &LocalCheckCurrentBasisV1 {
        &self.basis
    }

    /// Checks exact original copied owner history. The work owner independently
    /// checks its retained physical root, original initialization claim and work
    /// lock. This method consults no live Charter/terms or filesystem at all.
    pub(crate) fn validate_historical(
        &self,
        locator: &LocalCheckLocatorV1,
        basis: &LocalCheckCurrentBasisV1,
    ) -> Result<(), LocalAuthorityRefusalV1> {
        self.validate_shape()?;
        if &self.locator != locator || &self.basis != basis {
            return Err(refuse(
                "work_proof_binding",
                "historical proof differs from original exact admission",
            ));
        }
        for (path, expected) in
            local_charter::work_registration_historical_bytes(&locator.predecessor.charter)?
        {
            if self.file(&path)?.as_bytes() != expected {
                return Err(refuse(
                    "work_proof_charter",
                    "original exact Charter owner dependency differs",
                ));
            }
        }
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct ClaimRecord {
            schema_id: String,
            schema_version: String,
            claim: crate::RepositoryInitializationClaimV1,
            retained_lock_fingerprint: String,
        }
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Receipt {
            schema_id: String,
            schema_version: String,
            claim_fingerprint: String,
            disposition: crate::RepositoryInitializationDispositionV1,
            terminal_path: String,
            terminal_digest: String,
        }
        let claim = &locator.predecessor.charter.binding.initialization_claim;
        let record: ClaimRecord = decode(
            self.file(crate::REPOSITORY_INITIALIZATION_CLAIM_PATH_V1)?
                .as_bytes(),
        )?;
        let receipt: Receipt = decode(
            self.file(crate::REPOSITORY_INITIALIZATION_RECEIPT_PATH_V1)?
                .as_bytes(),
        )?;
        if record.schema_id != "handbook.repository-initialization-claim-record"
            || record.schema_version != "1.0"
            || &record.claim != claim
            || record.retained_lock_fingerprint != self.initialization_lock_fingerprint
            || receipt.schema_id != "handbook.repository-initialization-receipt"
            || receipt.schema_version != "1.0"
            || receipt.claim_fingerprint != claim.claim_fingerprint()
            || receipt.disposition != crate::RepositoryInitializationDispositionV1::Committed
            || receipt.terminal_path.starts_with(".handbook/state/")
            || initialization_fingerprint(
                "handbook.repository-initialization-terminal-bytes@1.0",
                self.file(&receipt.terminal_path)?.as_bytes(),
            ) != receipt.terminal_digest
            || self.file(crate::REPOSITORY_IDENTITY_REPO_PATH)?
                != claim.proposed_repository_identity()
        {
            return Err(refuse(
                "work_proof_initialization",
                "original positive initialization claim and terminal closure differ",
            ));
        }
        let predecessor = &locator.predecessor;
        let original_path =
            |name| format!("{BASE}/requests/{}/{name}", predecessor.terms.request_key);
        if self.file(&original_path("request.json"))?.as_bytes() != encode(predecessor)? {
            return Err(refuse(
                "work_proof_history",
                "original initial operating request differs",
            ));
        }
        let bytes = self.file(&original_path("outcome.json"))?.as_bytes();
        let original: LocalAuthorityOutcomeV1 = decode(bytes)?;
        if original.locator != *predecessor
            || original.outcome_fingerprint != locator.terms.expected_predecessor_fingerprint
            || original.governance_relation_fingerprint
                != predecessor.expected_governance_relation_fingerprint
            || !original.validation_confirmed
            || !original.advisory_permission_accepted
            || !original.terms_committed
            || original.outcome_fingerprint != outcome_identity(&original)?
            || encode(&original)? != bytes
            || self.file(&original_path("terminal.json"))?.as_bytes() != bytes
        {
            return Err(refuse(
                "work_proof_history",
                "original initial operating outcome/terminal differs",
            ));
        }
        let outcome = local_check::work_registration_expected_outcome(locator)?;
        let check_path = |name| {
            format!(
                "{}/requests/{}/{name}",
                local_check::LOCAL_CHECK_ROOT_V1,
                locator.terms.request_key
            )
        };
        if self.file(&check_path("request.json"))?.as_bytes() != encode(locator)?
            || self.file(&check_path("outcome.json"))?.as_bytes() != encode(&outcome)?
            || self.file(&check_path("terminal.json"))?.as_bytes() != encode(&outcome)?
        {
            return Err(refuse(
                "work_proof_history",
                "original exact local-check history differs",
            ));
        }
        if outcome.outcome_fingerprint != self.authority_fingerprint {
            return Err(refuse(
                "work_proof_history",
                "original authority outcome differs",
            ));
        }
        let initial_head: Head = decode(self.file(HEAD)?.as_bytes())?;
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct CheckHead {
            schema_ref: String,
            selected: String,
            revoked: Option<String>,
        }
        let check_head: CheckHead = decode(
            self.file(&format!(
                "{}/current.json",
                local_check::LOCAL_CHECK_ROOT_V1
            ))?
            .as_bytes(),
        )?;
        if initial_head.schema != LOCAL_OPERATING_RULE_V1
            || initial_head.initial != original.outcome_fingerprint
            || initial_head.revoked.is_some()
            || check_head.schema_ref != local_check::LOCAL_CHECK_RULE_V1
            || check_head.selected != outcome.outcome_fingerprint
            || check_head.revoked.is_some()
            || self.files.iter().any(|file| {
                [
                    format!("{BASE}/revoke.json"),
                    format!("{}/revoke.json", local_check::LOCAL_CHECK_ROOT_V1),
                    ".handbook/local-charter/v1/pending.json".into(),
                ]
                .contains(&file.path)
            })
        {
            return Err(refuse(
                "work_proof_history",
                "captured original admission was not positively current",
            ));
        }
        self.relation_kind()?;
        Ok(())
    }

    pub(crate) fn relation_kind(
        &self,
    ) -> Result<WorkRegistrationRelationKindV1, LocalAuthorityRefusalV1> {
        let value =
            crate::parse_schema_json(self.file(SCOPE_CURRENT)?.as_bytes()).map_err(codec)?;
        let object = value.as_object().ok_or_else(|| {
            refuse(
                "work_relation",
                "exact current D or established-from-D relation required",
            )
        })?;
        let deferred = [
            "project",
            "scope",
            "never_established",
            "registration_identity",
            "deferral_fingerprint",
            "outcome_fingerprint",
            "relation_fingerprint",
            "establishment_rule",
        ];
        let established = [
            "locator",
            "project",
            "scope",
            "predecessor",
            "configuration",
            "configuration_fingerprint",
            "replacement_basis",
            "establishment_rule",
            "owner_rule",
            "request_fingerprint",
            "outcome_fingerprint",
            "relation_fingerprint",
        ];
        let kind = if object.len() == deferred.len()
            && deferred.iter().all(|key| object.contains_key(*key))
            && object.get("never_established") == Some(&serde_json::Value::Bool(true))
        {
            WorkRegistrationRelationKindV1::Deferred
        } else if object.len() == established.len()
            && established.iter().all(|key| object.contains_key(*key))
        {
            WorkRegistrationRelationKindV1::EstablishedFromDeferred
        } else {
            return Err(refuse(
                "work_relation",
                "only exact current D or established-from-D G is supported",
            ));
        };
        let claim = &self
            .locator
            .predecessor
            .charter
            .binding
            .initialization_claim;
        let project = object
            .get("project")
            .and_then(serde_json::Value::as_object)
            .ok_or_else(|| refuse("work_relation", "exact same-project relation required"))?;
        if object.get("scope").and_then(serde_json::Value::as_str)
            != Some("contracts.evidence-freshness-policy/root")
            || project.len() != 4
            || project
                .get("repository_identity_fingerprint")
                .and_then(serde_json::Value::as_str)
                != Some(claim.proposed_repository_identity())
            || project
                .get("physical_root_fingerprint")
                .and_then(serde_json::Value::as_str)
                != Some(claim.physical_root_fingerprint())
            || project
                .get("retained_root")
                .and_then(serde_json::Value::as_str)
                .map(|path| {
                    initialization_fingerprint(
                        "handbook.repository-initialization-root-path@1.0",
                        path.as_bytes(),
                    )
                })
                .as_deref()
                != Some(claim.root_path_digest())
        {
            return Err(refuse(
                "work_relation",
                "current D/G scope differs from the exact initialized project",
            ));
        }
        Ok(kind)
    }

    fn file(&self, path: &str) -> Result<&str, LocalAuthorityRefusalV1> {
        self.files
            .binary_search_by(|entry| entry.path.as_str().cmp(path))
            .map(|index| self.files[index].content.as_str())
            .map_err(|_| {
                refuse(
                    "work_proof_missing",
                    "required captured authority dependency is absent",
                )
            })
    }

    fn validate_shape(&self) -> Result<(), LocalAuthorityRefusalV1> {
        self.locator.canonical_json_bytes()?;
        if self.schema_ref != PROOF_SCHEMA
            || self.directories.len() + self.files.len() > INVENTORY_MAX
            || self.directories.windows(2).any(|p| p[0] >= p[1])
            || self.files.windows(2).any(|p| p[0].path >= p[1].path)
            || ROOTS
                .iter()
                .any(|root| self.directories.binary_search(&root.to_string()).is_err())
            || EXACT.iter().any(|path| self.file(path).is_err())
        {
            return Err(refuse(
                "work_proof_shape",
                "closed complete sorted authority proof required",
            ));
        }
        let mut inventory = BTreeMap::new();
        for directory in &self.directories {
            validate_proof_path(directory)?;
            if !ROOTS
                .iter()
                .any(|root| directory == root || directory.starts_with(&format!("{root}/")))
            {
                return Err(refuse("work_proof_shape", "unexpected authority directory"));
            }
            if directory.starts_with(".handbook/admission-governance/v2") {
                inventory.insert(directory.clone(), ("directory", String::new()));
            }
        }
        for file in &self.files {
            validate_proof_path(&file.path)?;
            if !EXACT.contains(&file.path.as_str())
                && !ROOTS
                    .iter()
                    .any(|root| file.path.starts_with(&format!("{root}/")))
            {
                return Err(refuse(
                    "work_proof_inventory",
                    "unexpected captured authority file",
                ));
            }
            if file.content.len() > MAX
                || DefinitionFingerprint::from_bytes(file.content.as_bytes()).as_str()
                    != file.byte_fingerprint
            {
                return Err(refuse(
                    "work_proof_bytes",
                    "captured exact authority bytes exceed bound or changed",
                ));
            }
            let Some((parent, _)) = file.path.rsplit_once('/') else {
                continue;
            };
            if ROOTS
                .iter()
                .any(|root| file.path.starts_with(&format!("{root}/")))
                && self.directories.binary_search(&parent.to_string()).is_err()
            {
                return Err(refuse(
                    "work_proof_inventory",
                    "captured authority file has no declared directory",
                ));
            }
            if file.path.starts_with(".handbook/admission-governance/v2/") {
                inventory.insert(file.path.clone(), ("file", file.byte_fingerprint.clone()));
            }
        }
        let entries = inventory
            .into_iter()
            .map(|(path, (kind, fingerprint))| (path, kind, fingerprint))
            .collect::<Vec<_>>();
        if fingerprint(
            "handbook.local-check-governance-observation@1.0.0",
            &entries,
        )? != self.basis.governance_tree_fingerprint
            || DefinitionFingerprint::from_bytes(self.file(SCOPE_CURRENT)?.as_bytes()).as_str()
                != self.basis.relation_fingerprint
        {
            return Err(refuse(
                "work_proof_basis",
                "captured complete governance generation differs from original admitted basis",
            ));
        }
        encode(self)?;
        Ok(())
    }
}

fn initialization_fingerprint(domain: &str, bytes: &[u8]) -> String {
    let mut digest = Sha256::new();
    digest.update(domain.as_bytes());
    digest.update([0]);
    digest.update(bytes);
    format!("sha256:{:x}", digest.finalize())
}

fn validate_proof_path(path: &str) -> Result<(), LocalAuthorityRefusalV1> {
    if path.len() > 4096
        || path.contains(['\\', '\0'])
        || path
            .split('/')
            .any(|component| component.is_empty() || matches!(component, "." | ".."))
    {
        return Err(refuse(
            "work_proof_path",
            "exact bounded native owner path required",
        ));
    }
    Ok(())
}
