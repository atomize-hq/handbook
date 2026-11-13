//! Physical initialization reservation. Native owners retain their own meaning;
//! this module binds only exact root, identity, opaque request and terminal bytes.

use crate::repository_durability::{
    RepositoryDurabilityErrorKindV1, RepositoryDurabilityErrorV1, RepositoryExclusiveLockV1,
    RepositoryReadV1, RetainedRepositoryRootV1,
};
use crate::REPOSITORY_IDENTITY_REPO_PATH;
use serde::{Deserialize, Deserializer, Serialize};
use sha2::{Digest, Sha256};
use std::fmt;
use std::path::Path;

pub const REPOSITORY_INITIALIZATION_LOCK_PATH_V1: &str = ".handbook-initialization.lock";
pub const REPOSITORY_INITIALIZATION_CLAIM_PATH_V1: &str = ".handbook-initialization.v1.json";
pub const REPOSITORY_INITIALIZATION_RECEIPT_PATH_V1: &str =
    ".handbook-initialization-receipt.v1.json";
const MAX_RECORD_BYTES: usize = 1024 * 1024;
const V1_ANCHOR: &str = ".handbook/admission-governance-project.v1.json";
const V2_ANCHOR: &str = ".handbook/admission-governance-project.v2.json";
const V2_NONCOMMIT_WITNESS: &str = ".handbook-admission-deferral-noncommit.v2.json";

/// Serializable observation locator, never a fresh identity-creation token.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct RepositoryInitializationClaimV1 {
    schema_id: String,
    schema_version: String,
    root_path_digest: String,
    physical_root_fingerprint: String,
    proposed_repository_identity: String,
    native_owner: String,
    request_digest: String,
    claim_fingerprint: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ClaimWire {
    schema_id: String,
    schema_version: String,
    root_path_digest: String,
    physical_root_fingerprint: String,
    proposed_repository_identity: String,
    native_owner: String,
    request_digest: String,
    claim_fingerprint: String,
}

impl<'de> Deserialize<'de> for RepositoryInitializationClaimV1 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let wire = ClaimWire::deserialize(deserializer)?;
        let claim = Self {
            schema_id: wire.schema_id,
            schema_version: wire.schema_version,
            root_path_digest: wire.root_path_digest,
            physical_root_fingerprint: wire.physical_root_fingerprint,
            proposed_repository_identity: wire.proposed_repository_identity,
            native_owner: wire.native_owner,
            request_digest: wire.request_digest,
            claim_fingerprint: wire.claim_fingerprint,
        };
        claim.validate().map_err(serde::de::Error::custom)?;
        Ok(claim)
    }
}

impl RepositoryInitializationClaimV1 {
    /// Computes exact locator bytes without acquiring a lock or writing a file.
    /// The semantic owner separately decides fresh-project eligibility.
    pub fn prepare(
        repo_root: &Path,
        root: &RetainedRepositoryRootV1,
        native_owner: &str,
        request_digest: &str,
        proposed_repository_identity: &str,
    ) -> Result<Self, RepositoryDurabilityErrorV1> {
        if root.initialization_root_path() != repo_root
            || !root.path_still_names_same_physical_root(repo_root)
        {
            return Err(unsafe_state(
                "initialization claim root differs from its retained root",
            ));
        }
        let mut claim = Self {
            schema_id: "handbook.repository-initialization-claim".to_owned(),
            schema_version: "1.0".to_owned(),
            root_path_digest: path_digest(repo_root)?,
            physical_root_fingerprint: root.governance_physical_root_fingerprint_v1()?,
            proposed_repository_identity: proposed_repository_identity.to_owned(),
            native_owner: native_owner.to_owned(),
            request_digest: request_digest.to_owned(),
            claim_fingerprint: String::new(),
        };
        claim.claim_fingerprint = claim.compute_fingerprint()?;
        claim.validate()?;
        Ok(claim)
    }

    pub fn root_path_digest(&self) -> &str {
        &self.root_path_digest
    }
    pub fn physical_root_fingerprint(&self) -> &str {
        &self.physical_root_fingerprint
    }
    pub fn proposed_repository_identity(&self) -> &str {
        &self.proposed_repository_identity
    }
    pub fn native_owner(&self) -> &str {
        &self.native_owner
    }
    pub fn request_digest(&self) -> &str {
        &self.request_digest
    }
    pub fn claim_fingerprint(&self) -> &str {
        &self.claim_fingerprint
    }

    pub fn validate(&self) -> Result<(), RepositoryDurabilityErrorV1> {
        if self.schema_id != "handbook.repository-initialization-claim"
            || self.schema_version != "1.0"
            || self.native_owner.is_empty()
            || self.native_owner.len() > 128
            || !self.native_owner.bytes().all(|byte| {
                byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'-' | b'_' | b'@')
            })
            || [
                &self.root_path_digest,
                &self.physical_root_fingerprint,
                &self.proposed_repository_identity,
                &self.request_digest,
                &self.claim_fingerprint,
            ]
            .into_iter()
            .any(|value| !is_fingerprint(value))
            || self.compute_fingerprint()? != self.claim_fingerprint
        {
            return Err(unsafe_state(
                "initialization claim is not the exact supported record",
            ));
        }
        Ok(())
    }

    pub fn canonical_bytes(&self) -> Result<Vec<u8>, RepositoryDurabilityErrorV1> {
        self.validate()?;
        canonical(self)
    }

    fn compute_fingerprint(&self) -> Result<String, RepositoryDurabilityErrorV1> {
        #[derive(Serialize)]
        struct Binding<'a> {
            schema_id: &'a str,
            schema_version: &'a str,
            root_path_digest: &'a str,
            physical_root_fingerprint: &'a str,
            proposed_repository_identity: &'a str,
            native_owner: &'a str,
            request_digest: &'a str,
        }
        let bytes = canonical(&Binding {
            schema_id: &self.schema_id,
            schema_version: &self.schema_version,
            root_path_digest: &self.root_path_digest,
            physical_root_fingerprint: &self.physical_root_fingerprint,
            proposed_repository_identity: &self.proposed_repository_identity,
            native_owner: &self.native_owner,
            request_digest: &self.request_digest,
        })?;
        Ok(fingerprint(
            "handbook.repository-initialization-claim@1.0",
            &bytes,
        ))
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RepositoryInitializationDispositionV1 {
    Committed,
    NotCommitted,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct Receipt {
    schema_id: String,
    schema_version: String,
    claim_fingerprint: String,
    disposition: RepositoryInitializationDispositionV1,
    terminal_path: String,
    terminal_digest: String,
}

/// Physical bootstrap binding is retained after the lock exists; it is not
/// part of the caller's pre-effect claim identity or restored locator authority.
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct ClaimRecord {
    schema_id: String,
    schema_version: String,
    claim: RepositoryInitializationClaimV1,
    retained_lock_fingerprint: String,
}

/// Sealed lock held before any `.handbook` inspection/mutation. It must precede
/// native-owner or registry locks; callers pass it to locked identity helpers.
pub struct RepositoryInitializationGuardV1 {
    root: RetainedRepositoryRootV1,
    lock: Option<RepositoryExclusiveLockV1>,
}

impl fmt::Debug for RepositoryInitializationGuardV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("RepositoryInitializationGuardV1")
            .finish_non_exhaustive()
    }
}

impl RepositoryInitializationGuardV1 {
    /// Strict current/history acquisition never creates a lost native lock.
    pub fn acquire_existing_v1(
        repo_root: impl AsRef<Path>,
    ) -> Result<Self, RepositoryDurabilityErrorV1> {
        let root = RetainedRepositoryRootV1::open_exact(repo_root)?;
        let lock =
            root.acquire_existing_exclusive_lock_v1(REPOSITORY_INITIALIZATION_LOCK_PATH_V1)?;
        let guard = Self {
            root,
            lock: Some(lock),
        };
        guard.validate()?;
        Ok(guard)
    }

    pub fn acquire(repo_root: impl AsRef<Path>) -> Result<Self, RepositoryDurabilityErrorV1> {
        let root = RetainedRepositoryRootV1::open_exact(repo_root)?;
        let claim_present = root.initialization_regular_file_exists(
            REPOSITORY_INITIALIZATION_CLAIM_PATH_V1,
            MAX_RECORD_BYTES,
        )?;
        let receipt_present = root.initialization_regular_file_exists(
            REPOSITORY_INITIALIZATION_RECEIPT_PATH_V1,
            MAX_RECORD_BYTES,
        )?;
        let witness_present =
            root.initialization_regular_file_exists(V2_NONCOMMIT_WITNESS, MAX_RECORD_BYTES)?;
        let lock_present =
            root.initialization_regular_file_exists(REPOSITORY_INITIALIZATION_LOCK_PATH_V1, 0)?;
        if (claim_present || receipt_present || witness_present) && !lock_present {
            return Err(unsafe_state(
                "native initialization reservation lost its retained lock",
            ));
        }
        let lock = root.acquire_exclusive_lock(REPOSITORY_INITIALIZATION_LOCK_PATH_V1)?;
        lock.confirm_initialization_durable_at(&root, REPOSITORY_INITIALIZATION_LOCK_PATH_V1)?;
        let guard = Self {
            root,
            lock: Some(lock),
        };
        guard.validate()?;
        Ok(guard)
    }

    pub fn root(&self) -> &RetainedRepositoryRootV1 {
        &self.root
    }

    pub fn validate(&self) -> Result<(), RepositoryDurabilityErrorV1> {
        self.lock
            .as_ref()
            .ok_or_else(|| unsafe_state("initialization guard was released"))?
            .validate_at(&self.root, REPOSITORY_INITIALIZATION_LOCK_PATH_V1)
    }

    pub fn require_fresh(&self) -> Result<(), RepositoryDurabilityErrorV1> {
        self.validate()?;
        if self
            .root
            .read_regular_bounded(REPOSITORY_INITIALIZATION_CLAIM_PATH_V1, MAX_RECORD_BYTES)?
            != RepositoryReadV1::Absent
            || self
                .root
                .read_regular_bounded(REPOSITORY_INITIALIZATION_RECEIPT_PATH_V1, MAX_RECORD_BYTES)?
                != RepositoryReadV1::Absent
            || self
                .root
                .initialization_regular_file_exists(V2_NONCOMMIT_WITNESS, MAX_RECORD_BYTES)?
            || self.root.initialization_handbook_exists()?
        {
            return Err(error(
                RepositoryDurabilityErrorKindV1::AlreadyExists,
                "initialization is already reserved",
            ));
        }
        self.validate()
    }

    pub fn retain_native_claim(
        &self,
        claim: &RepositoryInitializationClaimV1,
    ) -> Result<(), RepositoryDurabilityErrorV1> {
        self.require_fresh()?;
        self.validate_claim_root(claim)?;
        let record = ClaimRecord {
            schema_id: "handbook.repository-initialization-claim-record".to_owned(),
            schema_version: "1.0".to_owned(),
            claim: claim.clone(),
            retained_lock_fingerprint: self.lock_fingerprint()?,
        };
        self.root.write_new_durable(
            REPOSITORY_INITIALIZATION_CLAIM_PATH_V1,
            &canonical(&record)?,
        )?;
        if !self.observe_native_claim(claim)? {
            return Err(unsafe_state(
                "native initialization claim publication is unverified",
            ));
        }
        Ok(())
    }

    pub fn observe_native_claim(
        &self,
        expected: &RepositoryInitializationClaimV1,
    ) -> Result<bool, RepositoryDurabilityErrorV1> {
        self.validate()?;
        self.validate_claim_root(expected)?;
        let Some(claim) = self.read_claim()? else {
            if self
                .root
                .read_regular_bounded(REPOSITORY_INITIALIZATION_RECEIPT_PATH_V1, MAX_RECORD_BYTES)?
                != RepositoryReadV1::Absent
            {
                return Err(unsafe_state("initialization receipt has no retained claim"));
            }
            return Ok(false);
        };
        if claim != *expected {
            return Err(error(
                RepositoryDurabilityErrorKindV1::CurrentValueMismatch,
                "native initialization claim differs from the exact request",
            ));
        }
        self.inspect_existing_receipt(&claim)?;
        self.validate()?;
        Ok(true)
    }

    /// Owner calls this only after its complete terminal inspection. Engine
    /// checks byte closure and identity; it does not determine governance truth.
    pub fn complete_native_claim(
        &self,
        claim: &RepositoryInitializationClaimV1,
        disposition: RepositoryInitializationDispositionV1,
        terminal_path: &str,
        terminal_bytes: &[u8],
    ) -> Result<(), RepositoryDurabilityErrorV1> {
        if !self.observe_native_claim(claim)? {
            return Err(unsafe_state(
                "native terminal has no exact initialization claim",
            ));
        }
        self.verify_terminal(claim, disposition, terminal_path, terminal_bytes)?;
        let receipt = Self::receipt(claim, disposition, terminal_path, terminal_bytes)?;
        let bytes = canonical(&receipt)?;
        match self
            .root
            .read_regular_bounded(REPOSITORY_INITIALIZATION_RECEIPT_PATH_V1, MAX_RECORD_BYTES)?
        {
            RepositoryReadV1::Absent => self
                .root
                .write_new_durable(REPOSITORY_INITIALIZATION_RECEIPT_PATH_V1, &bytes)?,
            RepositoryReadV1::Found(actual) if actual == bytes => {}
            RepositoryReadV1::Found(_) => {
                return Err(unsafe_state(
                    "initialization receipt conflicts with the exact owner terminal",
                ))
            }
        }
        self.require_native_terminal(claim, disposition, terminal_path, terminal_bytes)
    }

    pub fn require_native_terminal(
        &self,
        claim: &RepositoryInitializationClaimV1,
        disposition: RepositoryInitializationDispositionV1,
        terminal_path: &str,
        terminal_bytes: &[u8],
    ) -> Result<(), RepositoryDurabilityErrorV1> {
        if !self.observe_native_claim(claim)? {
            return Err(unsafe_state("initialization terminal claim is absent"));
        }
        self.verify_terminal(claim, disposition, terminal_path, terminal_bytes)?;
        let expected = canonical(&Self::receipt(
            claim,
            disposition,
            terminal_path,
            terminal_bytes,
        )?)?;
        if self
            .root
            .read_regular_bounded(REPOSITORY_INITIALIZATION_RECEIPT_PATH_V1, MAX_RECORD_BYTES)?
            != RepositoryReadV1::Found(expected)
        {
            return Err(unsafe_state(
                "exact initialization terminal receipt is unavailable",
            ));
        }
        self.validate()
    }

    /// Native claims are never repaired by ordinary setup. Claimless v1 roots
    /// stay readable only with their original exact physical/identity binding.
    pub fn validate_ordinary_setup(&self) -> Result<(), RepositoryDurabilityErrorV1> {
        self.validate()?;
        let Some(claim) = self.read_claim()? else {
            if self
                .root
                .read_regular_bounded(REPOSITORY_INITIALIZATION_RECEIPT_PATH_V1, MAX_RECORD_BYTES)?
                != RepositoryReadV1::Absent
            {
                return Err(unsafe_state("initialization receipt has no retained claim"));
            }
            return self.validate_claimless_native_anchor();
        };
        self.validate_claim_root(&claim)?;
        let RepositoryReadV1::Found(bytes) = self
            .root
            .read_regular_bounded(REPOSITORY_INITIALIZATION_RECEIPT_PATH_V1, MAX_RECORD_BYTES)?
        else {
            return Err(unsafe_state(
                "native initialization claim remains unresolved",
            ));
        };
        let receipt: Receipt = decode_canonical(&bytes)?;
        if receipt.schema_id != "handbook.repository-initialization-receipt"
            || receipt.schema_version != "1.0"
            || receipt.claim_fingerprint != claim.claim_fingerprint
            || receipt.disposition != RepositoryInitializationDispositionV1::Committed
            || !is_fingerprint(&receipt.terminal_digest)
        {
            return Err(unsafe_state(
                "native initialization did not retain exact committed closure",
            ));
        }
        let RepositoryReadV1::Found(terminal) = self
            .root
            .read_regular_bounded(&receipt.terminal_path, MAX_RECORD_BYTES)?
        else {
            return Err(unsafe_state(
                "native initialization terminal witness is unavailable",
            ));
        };
        if fingerprint(
            "handbook.repository-initialization-terminal-bytes@1.0",
            &terminal,
        ) != receipt.terminal_digest
        {
            return Err(unsafe_state(
                "native initialization terminal witness differs",
            ));
        }
        self.verify_terminal(
            &claim,
            receipt.disposition,
            &receipt.terminal_path,
            &terminal,
        )?;
        self.validate()
    }

    pub fn release(mut self) -> Result<(), RepositoryDurabilityErrorV1> {
        self.validate()?;
        self.lock
            .take()
            .ok_or_else(|| unsafe_state("initialization guard was released"))?
            .release()
    }

    pub(crate) fn require_prepared_identity(
        &self,
        proposed: &str,
    ) -> Result<(), RepositoryDurabilityErrorV1> {
        self.validate()?;
        match self.read_claim()? {
            Some(claim) => {
                self.validate_claim_root(&claim)?;
                if claim.proposed_repository_identity != proposed
                    || self.root.read_regular_bounded(
                        REPOSITORY_INITIALIZATION_RECEIPT_PATH_V1,
                        MAX_RECORD_BYTES,
                    )? != RepositoryReadV1::Absent
                {
                    return Err(unsafe_state(
                        "prepared identity does not match the pending native claim",
                    ));
                }
            }
            None => {
                self.validate_ordinary_setup()?;
                if self.root.initialization_handbook_exists()?
                    && [V1_ANCHOR, V2_ANCHOR].into_iter().any(|path| {
                        self.root.read_regular_bounded(path, MAX_RECORD_BYTES)
                            != Ok(RepositoryReadV1::Absent)
                    })
                {
                    return Err(unsafe_state(
                        "native initialization anchor cannot accept a new identity",
                    ));
                }
            }
        }
        self.validate()
    }

    fn read_claim(
        &self,
    ) -> Result<Option<RepositoryInitializationClaimV1>, RepositoryDurabilityErrorV1> {
        match self
            .root
            .read_regular_bounded(REPOSITORY_INITIALIZATION_CLAIM_PATH_V1, MAX_RECORD_BYTES)?
        {
            RepositoryReadV1::Absent => Ok(None),
            RepositoryReadV1::Found(bytes) => {
                let record: ClaimRecord = decode_canonical(&bytes)?;
                if record.schema_id != "handbook.repository-initialization-claim-record"
                    || record.schema_version != "1.0"
                    || record.retained_lock_fingerprint != self.lock_fingerprint()?
                {
                    return Err(unsafe_state(
                        "native initialization retained lock binding differs",
                    ));
                }
                Ok(Some(record.claim))
            }
        }
    }

    fn lock_fingerprint(&self) -> Result<String, RepositoryDurabilityErrorV1> {
        self.validate()?;
        self.lock
            .as_ref()
            .ok_or_else(|| unsafe_state("initialization guard was released"))?
            .physical_fingerprint_v1()
    }

    fn inspect_existing_receipt(
        &self,
        claim: &RepositoryInitializationClaimV1,
    ) -> Result<Option<Receipt>, RepositoryDurabilityErrorV1> {
        let RepositoryReadV1::Found(bytes) = self
            .root
            .read_regular_bounded(REPOSITORY_INITIALIZATION_RECEIPT_PATH_V1, MAX_RECORD_BYTES)?
        else {
            return Ok(None);
        };
        let receipt: Receipt = decode_canonical(&bytes)?;
        if receipt.schema_id != "handbook.repository-initialization-receipt"
            || receipt.schema_version != "1.0"
            || receipt.claim_fingerprint != claim.claim_fingerprint
            || !is_fingerprint(&receipt.terminal_digest)
        {
            return Err(unsafe_state(
                "initialization receipt does not close its exact claim",
            ));
        }
        let RepositoryReadV1::Found(terminal) = self
            .root
            .read_regular_bounded(&receipt.terminal_path, MAX_RECORD_BYTES)?
        else {
            return Err(unsafe_state(
                "initialization receipt terminal witness is absent",
            ));
        };
        if receipt.terminal_digest
            != fingerprint(
                "handbook.repository-initialization-terminal-bytes@1.0",
                &terminal,
            )
        {
            return Err(unsafe_state(
                "initialization receipt terminal witness digest differs",
            ));
        }
        self.verify_terminal(
            claim,
            receipt.disposition,
            &receipt.terminal_path,
            &terminal,
        )?;
        Ok(Some(receipt))
    }

    fn validate_claim_root(
        &self,
        claim: &RepositoryInitializationClaimV1,
    ) -> Result<(), RepositoryDurabilityErrorV1> {
        claim.validate()?;
        if claim.root_path_digest != path_digest(self.root.initialization_root_path())?
            || claim.physical_root_fingerprint
                != self.root.governance_physical_root_fingerprint_v1()?
        {
            return Err(unsafe_state(
                "initialization claim names another retained or physical root",
            ));
        }
        Ok(())
    }

    fn verify_terminal(
        &self,
        claim: &RepositoryInitializationClaimV1,
        disposition: RepositoryInitializationDispositionV1,
        path: &str,
        bytes: &[u8],
    ) -> Result<(), RepositoryDurabilityErrorV1> {
        self.validate()?;
        if path == REPOSITORY_INITIALIZATION_CLAIM_PATH_V1
            || path == REPOSITORY_INITIALIZATION_RECEIPT_PATH_V1
            || path == REPOSITORY_INITIALIZATION_LOCK_PATH_V1
            || path == REPOSITORY_IDENTITY_REPO_PATH
            || path == ".handbook/state"
            || path.starts_with(".handbook/state/")
            || bytes.is_empty()
            || bytes.len() > MAX_RECORD_BYTES
            || self.root.read_regular_bounded(path, MAX_RECORD_BYTES)?
                != RepositoryReadV1::Found(bytes.to_vec())
        {
            return Err(unsafe_state(
                "initialization terminal witness is not exact retained owner bytes",
            ));
        }
        if disposition == RepositoryInitializationDispositionV1::Committed
            && self
                .root
                .read_regular_bounded(REPOSITORY_IDENTITY_REPO_PATH, 71)?
                != RepositoryReadV1::Found(claim.proposed_repository_identity.as_bytes().to_vec())
        {
            return Err(unsafe_state(
                "native committed identity differs from its initialization claim",
            ));
        }
        if self.root.initialization_handbook_exists()? {
            for identity_path in [
                REPOSITORY_IDENTITY_REPO_PATH,
                ".handbook/.repository-identity.v1.tmp",
            ] {
                match self.root.read_regular_bounded(identity_path, 71)? {
                    RepositoryReadV1::Found(bytes)
                        if bytes != claim.proposed_repository_identity.as_bytes() =>
                    {
                        return Err(unsafe_state(
                            "native initialization identity prefix was substituted",
                        ));
                    }
                    RepositoryReadV1::Found(_)
                        if disposition == RepositoryInitializationDispositionV1::Committed
                            && identity_path != REPOSITORY_IDENTITY_REPO_PATH =>
                    {
                        return Err(unsafe_state(
                            "native committed identity retains an unresolved temporary",
                        ));
                    }
                    _ => {}
                }
            }
        }
        self.validate()
    }

    fn receipt(
        claim: &RepositoryInitializationClaimV1,
        disposition: RepositoryInitializationDispositionV1,
        path: &str,
        bytes: &[u8],
    ) -> Result<Receipt, RepositoryDurabilityErrorV1> {
        claim.validate()?;
        Ok(Receipt {
            schema_id: "handbook.repository-initialization-receipt".to_owned(),
            schema_version: "1.0".to_owned(),
            claim_fingerprint: claim.claim_fingerprint.clone(),
            disposition,
            terminal_path: path.to_owned(),
            terminal_digest: fingerprint(
                "handbook.repository-initialization-terminal-bytes@1.0",
                bytes,
            ),
        })
    }

    fn validate_claimless_native_anchor(&self) -> Result<(), RepositoryDurabilityErrorV1> {
        if self
            .root
            .initialization_regular_file_exists(V2_NONCOMMIT_WITNESS, MAX_RECORD_BYTES)?
        {
            return Err(unsafe_state(
                "native deferral noncommit witness lost its initialization claim",
            ));
        }
        if !self.root.initialization_handbook_exists()? {
            return self.validate();
        }
        if self
            .root
            .read_regular_bounded(V2_ANCHOR, MAX_RECORD_BYTES)?
            != RepositoryReadV1::Absent
        {
            return Err(unsafe_state(
                "v2 native initialization anchor lost its claim",
            ));
        }
        if let RepositoryReadV1::Found(bytes) = self
            .root
            .read_regular_bounded(V1_ANCHOR, MAX_RECORD_BYTES)?
        {
            let anchor = crate::parse_schema_json(&bytes)
                .map_err(|_| unsafe_state("legacy native initialization anchor is malformed"))?;
            let project = anchor.get("project").ok_or_else(|| {
                unsafe_state("legacy native initialization binding is unavailable")
            })?;
            let identity = project
                .get("repository_identity_fingerprint")
                .and_then(serde_json::Value::as_str)
                .filter(|value| is_fingerprint(value))
                .ok_or_else(|| unsafe_state("legacy native identity binding is malformed"))?;
            if project
                .get("retained_root")
                .and_then(serde_json::Value::as_str)
                != self.root.initialization_root_path().to_str()
                || project
                    .get("physical_root_fingerprint")
                    .and_then(serde_json::Value::as_str)
                    != Some(
                        self.root
                            .governance_physical_root_fingerprint_v1()?
                            .as_str(),
                    )
                || self
                    .root
                    .read_regular_bounded(REPOSITORY_IDENTITY_REPO_PATH, 71)?
                    != RepositoryReadV1::Found(identity.as_bytes().to_vec())
            {
                return Err(unsafe_state(
                    "legacy native initialization identity or root binding differs",
                ));
            }
        }
        self.validate()
    }
}

fn canonical<T: Serialize>(value: &T) -> Result<Vec<u8>, RepositoryDurabilityErrorV1> {
    let bytes = serde_json_canonicalizer::to_vec(value)
        .map_err(|_| unsafe_state("initialization record cannot be canonicalized"))?;
    if bytes.len() > MAX_RECORD_BYTES {
        return Err(error(
            RepositoryDurabilityErrorKindV1::BoundExceeded,
            "initialization record exceeds its byte bound",
        ));
    }
    Ok(bytes)
}

fn decode_canonical<T: serde::de::DeserializeOwned + Serialize>(
    bytes: &[u8],
) -> Result<T, RepositoryDurabilityErrorV1> {
    if bytes.is_empty() || bytes.len() > MAX_RECORD_BYTES {
        return Err(unsafe_state(
            "initialization record is empty or exceeds its byte bound",
        ));
    }
    let value: T = serde_json::from_slice(bytes)
        .map_err(|_| unsafe_state("initialization record is not closed typed JSON"))?;
    if canonical(&value)? != bytes {
        return Err(unsafe_state(
            "initialization record is not exact canonical JSON",
        ));
    }
    Ok(value)
}

fn path_digest(path: &Path) -> Result<String, RepositoryDurabilityErrorV1> {
    let path = path
        .to_str()
        .filter(|value| value.len() <= 4096)
        .ok_or_else(|| unsafe_state("initialization root path is not bounded UTF-8"))?;
    Ok(fingerprint(
        "handbook.repository-initialization-root-path@1.0",
        path.as_bytes(),
    ))
}

fn fingerprint(domain: &str, bytes: &[u8]) -> String {
    let mut digest = Sha256::new();
    digest.update(domain.as_bytes());
    digest.update([0]);
    digest.update(bytes);
    format!("sha256:{:x}", digest.finalize())
}

fn is_fingerprint(value: &str) -> bool {
    value.len() == 71
        && value.starts_with("sha256:")
        && value.as_bytes()[7..]
            .iter()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(byte))
}

fn error(
    kind: RepositoryDurabilityErrorKindV1,
    detail: &'static str,
) -> RepositoryDurabilityErrorV1 {
    RepositoryDurabilityErrorV1::initialization_refusal(kind, detail)
}

fn unsafe_state(detail: &'static str) -> RepositoryDurabilityErrorV1 {
    error(RepositoryDurabilityErrorKindV1::UnsafeFilesystem, detail)
}
