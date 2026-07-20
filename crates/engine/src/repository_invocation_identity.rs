use crate::approver_registry_mutation::recover_complete_registry_approval_authority_locked;
use crate::approver_registry_observation::observe_committed_approver_registry_if_present_locked;
use crate::canonical_repo_support::CanonicalWorkspace;
use crate::charter_authority_workflow::RegistryAuthorityLocks;
use crate::charter_lineage_store::{create_new_file, sync_directory};
use serde::{Deserialize, Deserializer, Serialize};
use sha2::{Digest, Sha256};
use std::fmt;
use std::fs;
use std::io::Write;
use std::path::Path;

const REPOSITORY_IDENTITY_DOMAIN: &[u8] = b"handbook.repository-identity@1.0";
const OPERATION_ID_DOMAIN: &[u8] = b"handbook.operation-id@1.0";
const ENTROPY_BYTES: usize = 32;

pub const REPOSITORY_IDENTITY_REPO_PATH: &str = ".handbook/repository-identity.v1";
const REPOSITORY_IDENTITY_TEMP_REPO_PATH: &str = ".handbook/.repository-identity.v1.tmp";

const PREFLIGHT_SCHEMA_ID: &str = "handbook.repository-invocation-preflight-result";
const RECOVERY_REFUSAL_SCHEMA_ID: &str = "handbook.repository-invocation-recovery-refusal";
const SCHEMA_VERSION: &str = "1.0";
const REFUSED_STATUS: &str = "refused";

const IDENTITY_STAGE: &str = "repository_identity";
const IDENTITY_UNAVAILABLE_CODE: &str = "repository_identity_unavailable";
const IDENTITY_UNAVAILABLE_MESSAGE: &str = "repository invocation identity is unavailable";
const IDENTITY_UNAVAILABLE_ACTION: &str =
    "run or repair handbook setup, then retry the complete operation";

const OPERATION_ID_STAGE: &str = "operation_id";
const OPERATION_ID_ENTROPY_CODE: &str = "operation_id_entropy_unavailable";
const OPERATION_ID_ENTROPY_MESSAGE: &str =
    "operating-system randomness was unavailable for operation ID allocation";
const OPERATION_ID_ENTROPY_ACTION: &str =
    "retry the complete operation with fresh operating-system randomness";

const AUTHORITY_RECOVERY_CODE: &str = "authority_recovery_blocked";
const AUTHORITY_RECOVERY_MESSAGE: &str = "repository authority recovery could not complete safely";
const AUTHORITY_RECOVERY_ACTION: &str =
    "repair retained registry/approval recovery evidence, then retry the complete operation";

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum RepositoryInvocationOperationV1 {
    #[serde(rename = "bootstrap")]
    ApproverBootstrap,
    #[serde(rename = "add_credential")]
    ApproverAddCredential,
    #[serde(rename = "revoke_credential")]
    ApproverRevokeCredential,
    #[serde(rename = "update_mapping")]
    ApproverUpdateMapping,
    #[serde(rename = "charter_approval")]
    CharterApproval,
}

impl RepositoryInvocationOperationV1 {
    fn allocator_token(self) -> &'static str {
        match self {
            Self::ApproverBootstrap => "bootstrap",
            Self::ApproverAddCredential => "add-credential",
            Self::ApproverRevokeCredential => "revoke-credential",
            Self::ApproverUpdateMapping => "update-mapping",
            Self::CharterApproval => "charter-approval",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RepositoryIdentityAllocationV1 {
    repository_identity_fingerprint: String,
}

impl RepositoryIdentityAllocationV1 {
    pub fn repository_identity_fingerprint(&self) -> &str {
        &self.repository_identity_fingerprint
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RepositoryIdentityAllocationErrorV1 {
    EntropyUnavailable,
}

impl fmt::Display for RepositoryIdentityAllocationErrorV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("operating-system randomness was unavailable for repository identity")
    }
}

impl std::error::Error for RepositoryIdentityAllocationErrorV1 {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RepositoryInvocationIdentityV1 {
    repository_identity_fingerprint: String,
    operation_id: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RepositoryIdentityInitializationDispositionV1 {
    Created,
    Preserved,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RepositoryIdentityInitializationV1 {
    disposition: RepositoryIdentityInitializationDispositionV1,
    repository_identity_fingerprint: String,
}

impl RepositoryIdentityInitializationV1 {
    pub fn disposition(&self) -> RepositoryIdentityInitializationDispositionV1 {
        self.disposition
    }

    pub fn repository_identity_fingerprint(&self) -> &str {
        &self.repository_identity_fingerprint
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RepositoryIdentitySetupErrorKindV1 {
    AuthorityRecoveryBlocked,
    UnsafeIdentityState,
    RegistryIdentityMismatch,
    EntropyUnavailable,
    PersistenceFailed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RepositoryIdentitySetupErrorV1 {
    kind: RepositoryIdentitySetupErrorKindV1,
    detail: &'static str,
}

impl RepositoryIdentitySetupErrorV1 {
    pub fn kind(&self) -> RepositoryIdentitySetupErrorKindV1 {
        self.kind
    }

    pub fn detail(&self) -> &str {
        self.detail
    }
}

impl fmt::Display for RepositoryIdentitySetupErrorV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.detail)
    }
}

impl std::error::Error for RepositoryIdentitySetupErrorV1 {}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(untagged)]
pub enum RepositoryInvocationPreparationFailureV1 {
    Recovery(RepositoryInvocationRecoveryRefusalV1),
    Preflight(RepositoryInvocationPreflightResultV1),
}

impl RepositoryInvocationPreparationFailureV1 {
    pub fn schema_id(&self) -> &str {
        match self {
            Self::Recovery(refusal) => refusal.schema_id(),
            Self::Preflight(refusal) => refusal.schema_id(),
        }
    }

    pub fn schema_version(&self) -> &str {
        match self {
            Self::Recovery(refusal) => refusal.schema_version(),
            Self::Preflight(refusal) => refusal.schema_version(),
        }
    }

    pub fn operation(&self) -> RepositoryInvocationOperationV1 {
        match self {
            Self::Recovery(refusal) => refusal.operation(),
            Self::Preflight(refusal) => refusal.operation(),
        }
    }

    pub fn stage(&self) -> Option<&str> {
        match self {
            Self::Recovery(_) => None,
            Self::Preflight(refusal) => Some(refusal.stage()),
        }
    }

    pub fn status(&self) -> &str {
        match self {
            Self::Recovery(refusal) => refusal.status(),
            Self::Preflight(refusal) => refusal.status(),
        }
    }

    pub fn changed_paths(&self) -> &[String] {
        match self {
            Self::Recovery(refusal) => refusal.changed_paths(),
            Self::Preflight(refusal) => refusal.changed_paths(),
        }
    }

    pub fn refusal(&self) -> &RepositoryInvocationRefusalV1 {
        match self {
            Self::Recovery(refusal) => refusal.refusal(),
            Self::Preflight(refusal) => refusal.refusal(),
        }
    }

    pub fn next_actions(&self) -> &[String] {
        match self {
            Self::Recovery(refusal) => refusal.next_actions(),
            Self::Preflight(refusal) => refusal.next_actions(),
        }
    }
}

impl RepositoryInvocationIdentityV1 {
    pub fn repository_identity_fingerprint(&self) -> &str {
        &self.repository_identity_fingerprint
    }

    pub fn operation_id(&self) -> &str {
        &self.operation_id
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RepositoryInvocationIdentityServiceV1;

impl RepositoryInvocationIdentityServiceV1 {
    pub fn new() -> Self {
        Self
    }

    pub fn allocate_repository_identity(
        &self,
    ) -> Result<RepositoryIdentityAllocationV1, RepositoryIdentityAllocationErrorV1> {
        self.allocate_repository_identity_with(|entropy| getrandom::fill(entropy).map_err(|_| ()))
    }

    #[allow(
        clippy::result_large_err,
        reason = "the exact reviewed preflight refusal DTO is serialized directly by product adapters"
    )]
    pub fn allocate_operation_id(
        &self,
        operation: RepositoryInvocationOperationV1,
        repository_identity_fingerprint: &str,
    ) -> Result<RepositoryInvocationIdentityV1, RepositoryInvocationPreflightResultV1> {
        self.allocate_operation_id_with(operation, repository_identity_fingerprint, |entropy| {
            getrandom::fill(entropy).map_err(|_| ())
        })
    }

    pub fn initialize_for_setup(
        &self,
        repo_root: impl AsRef<Path>,
    ) -> Result<RepositoryIdentityInitializationV1, RepositoryIdentitySetupErrorV1> {
        self.initialize_for_setup_with(repo_root.as_ref(), |entropy| {
            getrandom::fill(entropy).map_err(|_| ())
        })
    }

    #[allow(
        clippy::result_large_err,
        reason = "the exact reviewed preparation refusal union is serialized directly by product adapters"
    )]
    pub fn prepare_operation(
        &self,
        repo_root: impl AsRef<Path>,
        operation: RepositoryInvocationOperationV1,
    ) -> Result<RepositoryInvocationIdentityV1, RepositoryInvocationPreparationFailureV1> {
        self.prepare_operation_with(repo_root.as_ref(), operation, |entropy| {
            getrandom::fill(entropy).map_err(|_| ())
        })
    }

    fn allocate_repository_identity_with<F>(
        &self,
        fill_entropy: F,
    ) -> Result<RepositoryIdentityAllocationV1, RepositoryIdentityAllocationErrorV1>
    where
        F: FnOnce(&mut [u8]) -> Result<(), ()>,
    {
        let mut entropy = [0u8; ENTROPY_BYTES];
        fill_entropy(&mut entropy)
            .map_err(|_| RepositoryIdentityAllocationErrorV1::EntropyUnavailable)?;
        Ok(RepositoryIdentityAllocationV1 {
            repository_identity_fingerprint: derive_repository_identity(&entropy),
        })
    }

    #[allow(
        clippy::result_large_err,
        reason = "the injected-entropy seam preserves the same exact reviewed preflight DTO"
    )]
    fn allocate_operation_id_with<F>(
        &self,
        operation: RepositoryInvocationOperationV1,
        repository_identity_fingerprint: &str,
        fill_entropy: F,
    ) -> Result<RepositoryInvocationIdentityV1, RepositoryInvocationPreflightResultV1>
    where
        F: FnOnce(&mut [u8]) -> Result<(), ()>,
    {
        if !is_repository_identity_fingerprint(repository_identity_fingerprint) {
            return Err(
                RepositoryInvocationPreflightResultV1::repository_identity_unavailable(operation),
            );
        }
        let mut entropy = [0u8; ENTROPY_BYTES];
        if fill_entropy(&mut entropy).is_err() {
            return Err(
                RepositoryInvocationPreflightResultV1::operation_id_entropy_unavailable(
                    operation,
                    repository_identity_fingerprint,
                )
                .expect("validated repository identity"),
            );
        }
        Ok(RepositoryInvocationIdentityV1 {
            repository_identity_fingerprint: repository_identity_fingerprint.to_owned(),
            operation_id: derive_operation_id(repository_identity_fingerprint, operation, &entropy),
        })
    }

    fn initialize_for_setup_with<F>(
        &self,
        repo_root: &Path,
        fill_entropy: F,
    ) -> Result<RepositoryIdentityInitializationV1, RepositoryIdentitySetupErrorV1>
    where
        F: FnOnce(&mut [u8]) -> Result<(), ()>,
    {
        let _locks = RegistryAuthorityLocks::acquire(repo_root).map_err(|_| {
            setup_error(
                RepositoryIdentitySetupErrorKindV1::AuthorityRecoveryBlocked,
                "repository authority locks could not be acquired safely",
            )
        })?;
        recover_complete_registry_approval_authority_locked(repo_root).map_err(|_| {
            setup_error(
                RepositoryIdentitySetupErrorKindV1::AuthorityRecoveryBlocked,
                "repository authority recovery could not complete safely",
            )
        })?;
        let registry =
            observe_committed_approver_registry_if_present_locked(repo_root).map_err(|_| {
                setup_error(
                    RepositoryIdentitySetupErrorKindV1::AuthorityRecoveryBlocked,
                    "committed repository authority could not be observed safely",
                )
            })?;
        let registry_identity =
            validated_registry_identity(registry.as_ref().map(|observation| &observation.state))
                .map_err(|_| {
                    setup_error(
                        RepositoryIdentitySetupErrorKindV1::AuthorityRecoveryBlocked,
                        "committed registry repository identity is absent or malformed",
                    )
                })?;

        match read_repository_identity(repo_root)? {
            Some(repository_identity_fingerprint) => {
                if !registry_identity_matches(&repository_identity_fingerprint, registry_identity) {
                    return Err(setup_error(
                        RepositoryIdentitySetupErrorKindV1::RegistryIdentityMismatch,
                        "repository identity disagrees with committed registry authority",
                    ));
                }
                Ok(RepositoryIdentityInitializationV1 {
                    disposition: RepositoryIdentityInitializationDispositionV1::Preserved,
                    repository_identity_fingerprint,
                })
            }
            None if registry_identity.is_some() => Err(setup_error(
                RepositoryIdentitySetupErrorKindV1::RegistryIdentityMismatch,
                "repository identity is absent after registry bootstrap",
            )),
            None => {
                let mut entropy = [0u8; ENTROPY_BYTES];
                fill_entropy(&mut entropy).map_err(|_| {
                    setup_error(
                        RepositoryIdentitySetupErrorKindV1::EntropyUnavailable,
                        "operating-system randomness was unavailable for repository identity",
                    )
                })?;
                let repository_identity_fingerprint = derive_repository_identity(&entropy);
                persist_repository_identity(repo_root, &repository_identity_fingerprint)?;
                let observed = read_repository_identity(repo_root)?.ok_or_else(|| {
                    setup_error(
                        RepositoryIdentitySetupErrorKindV1::PersistenceFailed,
                        "created repository identity could not be re-observed",
                    )
                })?;
                if observed != repository_identity_fingerprint {
                    return Err(setup_error(
                        RepositoryIdentitySetupErrorKindV1::PersistenceFailed,
                        "created repository identity changed during verification",
                    ));
                }
                Ok(RepositoryIdentityInitializationV1 {
                    disposition: RepositoryIdentityInitializationDispositionV1::Created,
                    repository_identity_fingerprint,
                })
            }
        }
    }

    #[allow(
        clippy::result_large_err,
        reason = "the injected-entropy seam preserves the same exact reviewed preparation refusal union"
    )]
    fn prepare_operation_with<F>(
        &self,
        repo_root: &Path,
        operation: RepositoryInvocationOperationV1,
        fill_entropy: F,
    ) -> Result<RepositoryInvocationIdentityV1, RepositoryInvocationPreparationFailureV1>
    where
        F: FnOnce(&mut [u8]) -> Result<(), ()>,
    {
        match fs::symlink_metadata(repo_root.join(".handbook")) {
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                return Err(RepositoryInvocationPreparationFailureV1::Preflight(
                    RepositoryInvocationPreflightResultV1::repository_identity_unavailable(
                        operation,
                    ),
                ));
            }
            _ => {}
        }
        let _locks = RegistryAuthorityLocks::acquire(repo_root).map_err(|_| {
            RepositoryInvocationPreparationFailureV1::Recovery(
                RepositoryInvocationRecoveryRefusalV1::authority_recovery_blocked(operation),
            )
        })?;
        recover_complete_registry_approval_authority_locked(repo_root).map_err(|_| {
            RepositoryInvocationPreparationFailureV1::Recovery(
                RepositoryInvocationRecoveryRefusalV1::authority_recovery_blocked(operation),
            )
        })?;
        let registry =
            observe_committed_approver_registry_if_present_locked(repo_root).map_err(|_| {
                RepositoryInvocationPreparationFailureV1::Recovery(
                    RepositoryInvocationRecoveryRefusalV1::authority_recovery_blocked(operation),
                )
            })?;
        let registry_identity =
            validated_registry_identity(registry.as_ref().map(|observation| &observation.state))
                .map_err(|_| {
                    RepositoryInvocationPreparationFailureV1::Recovery(
                        RepositoryInvocationRecoveryRefusalV1::authority_recovery_blocked(
                            operation,
                        ),
                    )
                })?;
        let repository_identity_fingerprint = read_repository_identity(repo_root)
            .ok()
            .flatten()
            .filter(|identity| registry_identity_matches(identity, registry_identity))
            .ok_or_else(|| {
                RepositoryInvocationPreparationFailureV1::Preflight(
                    RepositoryInvocationPreflightResultV1::repository_identity_unavailable(
                        operation,
                    ),
                )
            })?;

        self.allocate_operation_id_with(operation, &repository_identity_fingerprint, fill_entropy)
            .map_err(RepositoryInvocationPreparationFailureV1::Preflight)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RepositoryInvocationContractErrorV1 {
    detail: &'static str,
}

impl RepositoryInvocationContractErrorV1 {
    pub fn detail(&self) -> &str {
        self.detail
    }
}

impl fmt::Display for RepositoryInvocationContractErrorV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.detail)
    }
}

impl std::error::Error for RepositoryInvocationContractErrorV1 {}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RepositoryInvocationRefusalV1 {
    code: String,
    message: String,
    retryable: bool,
}

impl RepositoryInvocationRefusalV1 {
    pub fn code(&self) -> &str {
        &self.code
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn retryable(&self) -> bool {
        self.retryable
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct RepositoryInvocationPreflightResultV1 {
    schema_id: String,
    schema_version: String,
    operation: RepositoryInvocationOperationV1,
    stage: String,
    status: String,
    repository_identity_fingerprint: Option<String>,
    operation_id: Option<String>,
    changed_paths: Vec<String>,
    refusal: RepositoryInvocationRefusalV1,
    next_actions: Vec<String>,
}

impl RepositoryInvocationPreflightResultV1 {
    pub fn repository_identity_unavailable(operation: RepositoryInvocationOperationV1) -> Self {
        Self {
            schema_id: PREFLIGHT_SCHEMA_ID.to_owned(),
            schema_version: SCHEMA_VERSION.to_owned(),
            operation,
            stage: IDENTITY_STAGE.to_owned(),
            status: REFUSED_STATUS.to_owned(),
            repository_identity_fingerprint: None,
            operation_id: None,
            changed_paths: Vec::new(),
            refusal: RepositoryInvocationRefusalV1 {
                code: IDENTITY_UNAVAILABLE_CODE.to_owned(),
                message: IDENTITY_UNAVAILABLE_MESSAGE.to_owned(),
                retryable: false,
            },
            next_actions: vec![IDENTITY_UNAVAILABLE_ACTION.to_owned()],
        }
    }

    pub fn operation_id_entropy_unavailable(
        operation: RepositoryInvocationOperationV1,
        repository_identity_fingerprint: impl Into<String>,
    ) -> Result<Self, RepositoryInvocationContractErrorV1> {
        let repository_identity_fingerprint = repository_identity_fingerprint.into();
        if !is_repository_identity_fingerprint(&repository_identity_fingerprint) {
            return Err(contract_error(
                "operation-ID entropy refusal requires an exact repository identity fingerprint",
            ));
        }
        Ok(Self {
            schema_id: PREFLIGHT_SCHEMA_ID.to_owned(),
            schema_version: SCHEMA_VERSION.to_owned(),
            operation,
            stage: OPERATION_ID_STAGE.to_owned(),
            status: REFUSED_STATUS.to_owned(),
            repository_identity_fingerprint: Some(repository_identity_fingerprint),
            operation_id: None,
            changed_paths: Vec::new(),
            refusal: RepositoryInvocationRefusalV1 {
                code: OPERATION_ID_ENTROPY_CODE.to_owned(),
                message: OPERATION_ID_ENTROPY_MESSAGE.to_owned(),
                retryable: true,
            },
            next_actions: vec![OPERATION_ID_ENTROPY_ACTION.to_owned()],
        })
    }

    pub fn operation(&self) -> RepositoryInvocationOperationV1 {
        self.operation
    }

    pub fn schema_id(&self) -> &str {
        &self.schema_id
    }

    pub fn schema_version(&self) -> &str {
        &self.schema_version
    }

    pub fn stage(&self) -> &str {
        &self.stage
    }

    pub fn status(&self) -> &str {
        &self.status
    }

    pub fn repository_identity_fingerprint(&self) -> Option<&str> {
        self.repository_identity_fingerprint.as_deref()
    }

    pub fn changed_paths(&self) -> &[String] {
        &self.changed_paths
    }

    pub fn refusal(&self) -> &RepositoryInvocationRefusalV1 {
        &self.refusal
    }

    pub fn next_actions(&self) -> &[String] {
        &self.next_actions
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RepositoryInvocationPreflightWireV1 {
    schema_id: String,
    schema_version: String,
    operation: RepositoryInvocationOperationV1,
    stage: String,
    status: String,
    repository_identity_fingerprint: Option<String>,
    operation_id: Option<String>,
    changed_paths: Vec<String>,
    refusal: RepositoryInvocationRefusalV1,
    next_actions: Vec<String>,
}

impl<'de> Deserialize<'de> for RepositoryInvocationPreflightResultV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = RepositoryInvocationPreflightWireV1::deserialize(deserializer)?;
        let expected = match wire.stage.as_str() {
            IDENTITY_STAGE => Self::repository_identity_unavailable(wire.operation),
            OPERATION_ID_STAGE => Self::operation_id_entropy_unavailable(
                wire.operation,
                wire.repository_identity_fingerprint
                    .as_deref()
                    .unwrap_or_default(),
            )
            .map_err(serde::de::Error::custom)?,
            _ => return Err(serde::de::Error::custom("unknown preflight stage")),
        };
        let actual = Self {
            schema_id: wire.schema_id,
            schema_version: wire.schema_version,
            operation: wire.operation,
            stage: wire.stage,
            status: wire.status,
            repository_identity_fingerprint: wire.repository_identity_fingerprint,
            operation_id: wire.operation_id,
            changed_paths: wire.changed_paths,
            refusal: wire.refusal,
            next_actions: wire.next_actions,
        };
        if actual != expected {
            return Err(serde::de::Error::custom(
                "preflight result does not match an exact closed branch",
            ));
        }
        Ok(actual)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct RepositoryInvocationRecoveryRefusalV1 {
    schema_id: String,
    schema_version: String,
    operation: RepositoryInvocationOperationV1,
    status: String,
    repository_identity_fingerprint: Option<String>,
    operation_id: Option<String>,
    changed_paths: Vec<String>,
    refusal: RepositoryInvocationRefusalV1,
    next_actions: Vec<String>,
}

impl RepositoryInvocationRecoveryRefusalV1 {
    pub fn authority_recovery_blocked(operation: RepositoryInvocationOperationV1) -> Self {
        Self {
            schema_id: RECOVERY_REFUSAL_SCHEMA_ID.to_owned(),
            schema_version: SCHEMA_VERSION.to_owned(),
            operation,
            status: REFUSED_STATUS.to_owned(),
            repository_identity_fingerprint: None,
            operation_id: None,
            changed_paths: Vec::new(),
            refusal: RepositoryInvocationRefusalV1 {
                code: AUTHORITY_RECOVERY_CODE.to_owned(),
                message: AUTHORITY_RECOVERY_MESSAGE.to_owned(),
                retryable: false,
            },
            next_actions: vec![AUTHORITY_RECOVERY_ACTION.to_owned()],
        }
    }

    pub fn operation(&self) -> RepositoryInvocationOperationV1 {
        self.operation
    }

    pub fn schema_id(&self) -> &str {
        &self.schema_id
    }

    pub fn schema_version(&self) -> &str {
        &self.schema_version
    }

    pub fn status(&self) -> &str {
        &self.status
    }

    pub fn changed_paths(&self) -> &[String] {
        &self.changed_paths
    }

    pub fn refusal(&self) -> &RepositoryInvocationRefusalV1 {
        &self.refusal
    }

    pub fn next_actions(&self) -> &[String] {
        &self.next_actions
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RepositoryInvocationRecoveryRefusalWireV1 {
    schema_id: String,
    schema_version: String,
    operation: RepositoryInvocationOperationV1,
    status: String,
    repository_identity_fingerprint: Option<String>,
    operation_id: Option<String>,
    changed_paths: Vec<String>,
    refusal: RepositoryInvocationRefusalV1,
    next_actions: Vec<String>,
}

impl<'de> Deserialize<'de> for RepositoryInvocationRecoveryRefusalV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = RepositoryInvocationRecoveryRefusalWireV1::deserialize(deserializer)?;
        let expected = Self::authority_recovery_blocked(wire.operation);
        let actual = Self {
            schema_id: wire.schema_id,
            schema_version: wire.schema_version,
            operation: wire.operation,
            status: wire.status,
            repository_identity_fingerprint: wire.repository_identity_fingerprint,
            operation_id: wire.operation_id,
            changed_paths: wire.changed_paths,
            refusal: wire.refusal,
            next_actions: wire.next_actions,
        };
        if actual != expected {
            return Err(serde::de::Error::custom(
                "recovery refusal does not match the exact closed envelope",
            ));
        }
        Ok(actual)
    }
}

fn derive_repository_identity(entropy: &[u8; ENTROPY_BYTES]) -> String {
    let mut digest = Sha256::new();
    digest.update(REPOSITORY_IDENTITY_DOMAIN);
    digest.update([0]);
    digest.update(entropy);
    format!("sha256:{:x}", digest.finalize())
}

fn derive_operation_id(
    repository_identity_fingerprint: &str,
    operation: RepositoryInvocationOperationV1,
    entropy: &[u8; ENTROPY_BYTES],
) -> String {
    let token = operation.allocator_token();
    let mut digest = Sha256::new();
    digest.update(OPERATION_ID_DOMAIN);
    digest.update([0]);
    digest.update(repository_identity_fingerprint.as_bytes());
    digest.update([0]);
    digest.update(token.as_bytes());
    digest.update([0]);
    digest.update(entropy);
    format!("{token}-{:x}", digest.finalize())
}

fn is_repository_identity_fingerprint(value: &str) -> bool {
    value.len() == 71
        && value.starts_with("sha256:")
        && value.as_bytes()[7..]
            .iter()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(byte))
}

fn read_repository_identity(
    repo_root: &Path,
) -> Result<Option<String>, RepositoryIdentitySetupErrorV1> {
    let workspace = CanonicalWorkspace::new(repo_root);
    let relative = workspace
        .normalize_repo_relative(REPOSITORY_IDENTITY_REPO_PATH)
        .map_err(|_| {
            setup_error(
                RepositoryIdentitySetupErrorKindV1::UnsafeIdentityState,
                "repository identity path is not normalized",
            )
        })?;
    let metadata = workspace.metadata_no_follow(&relative).map_err(|_| {
        setup_error(
            RepositoryIdentitySetupErrorKindV1::UnsafeIdentityState,
            "repository identity metadata could not be observed safely",
        )
    })?;
    if metadata.is_none() {
        return Ok(None);
    }
    let file = workspace.trusted_read_strict(&relative).map_err(|_| {
        setup_error(
            RepositoryIdentitySetupErrorKindV1::UnsafeIdentityState,
            "repository identity is not a safe regular file",
        )
    })?;
    let (bytes, exceeded) = file.read_bytes_bounded(71).map_err(|_| {
        setup_error(
            RepositoryIdentitySetupErrorKindV1::UnsafeIdentityState,
            "repository identity could not be read",
        )
    })?;
    if exceeded || bytes.len() != 71 {
        return Err(setup_error(
            RepositoryIdentitySetupErrorKindV1::UnsafeIdentityState,
            "repository identity does not have the exact byte length",
        ));
    }
    let value = std::str::from_utf8(&bytes).map_err(|_| {
        setup_error(
            RepositoryIdentitySetupErrorKindV1::UnsafeIdentityState,
            "repository identity is not exact ASCII",
        )
    })?;
    if !is_repository_identity_fingerprint(value) {
        return Err(setup_error(
            RepositoryIdentitySetupErrorKindV1::UnsafeIdentityState,
            "repository identity does not match the exact fingerprint grammar",
        ));
    }
    Ok(Some(value.to_owned()))
}

fn persist_repository_identity(
    repo_root: &Path,
    repository_identity_fingerprint: &str,
) -> Result<(), RepositoryIdentitySetupErrorV1> {
    let final_path = repo_root.join(REPOSITORY_IDENTITY_REPO_PATH);
    let temporary_path = repo_root.join(REPOSITORY_IDENTITY_TEMP_REPO_PATH);
    if fs::symlink_metadata(&final_path).is_ok() || fs::symlink_metadata(&temporary_path).is_ok() {
        return Err(setup_error(
            RepositoryIdentitySetupErrorKindV1::UnsafeIdentityState,
            "repository identity create-new path is already occupied",
        ));
    }
    let mut temporary = create_new_file(&temporary_path).map_err(|_| {
        setup_error(
            RepositoryIdentitySetupErrorKindV1::PersistenceFailed,
            "repository identity temporary file could not be created",
        )
    })?;
    if temporary
        .write_all(repository_identity_fingerprint.as_bytes())
        .and_then(|()| temporary.sync_all())
        .is_err()
    {
        drop(temporary);
        let _ = fs::remove_file(&temporary_path);
        return Err(setup_error(
            RepositoryIdentitySetupErrorKindV1::PersistenceFailed,
            "repository identity temporary file could not be written durably",
        ));
    }
    drop(temporary);
    if rename_create_new(&temporary_path, &final_path).is_err() {
        let _ = fs::remove_file(&temporary_path);
        return Err(setup_error(
            RepositoryIdentitySetupErrorKindV1::PersistenceFailed,
            "repository identity could not be installed atomically",
        ));
    }
    sync_directory(
        final_path
            .parent()
            .expect("repository identity has a fixed parent"),
    )
    .map_err(|_| {
        setup_error(
            RepositoryIdentitySetupErrorKindV1::PersistenceFailed,
            "repository identity parent could not be flushed durably",
        )
    })
}

#[cfg(any(
    target_os = "android",
    target_os = "linux",
    target_os = "macos",
    target_os = "ios",
    target_os = "tvos",
    target_os = "visionos",
    target_os = "watchos",
    target_os = "redox",
))]
fn rename_create_new(source: &Path, target: &Path) -> Result<(), std::io::Error> {
    use rustix::fs::{renameat_with, RenameFlags, CWD};

    renameat_with(CWD, source, CWD, target, RenameFlags::NOREPLACE).map_err(std::io::Error::from)
}

#[cfg(windows)]
fn rename_create_new(source: &Path, target: &Path) -> Result<(), std::io::Error> {
    fs::rename(source, target)
}

#[cfg(all(
    unix,
    not(any(
        target_os = "android",
        target_os = "linux",
        target_os = "macos",
        target_os = "ios",
        target_os = "tvos",
        target_os = "visionos",
        target_os = "watchos",
        target_os = "redox",
    ))
))]
fn rename_create_new(_source: &Path, _target: &Path) -> Result<(), std::io::Error> {
    Err(std::io::Error::new(
        std::io::ErrorKind::Unsupported,
        "atomic create-new rename is unavailable on this platform",
    ))
}

#[cfg(all(not(unix), not(windows)))]
fn rename_create_new(_source: &Path, _target: &Path) -> Result<(), std::io::Error> {
    Err(std::io::Error::new(
        std::io::ErrorKind::Unsupported,
        "atomic create-new rename is unavailable on this platform",
    ))
}

fn registry_identity_matches(identity: &str, committed_registry_identity: Option<&str>) -> bool {
    committed_registry_identity
        .map(|committed| committed == identity)
        .unwrap_or(true)
}

fn validated_registry_identity(
    committed_registry_state: Option<&serde_json::Value>,
) -> Result<Option<&str>, ()> {
    let Some(state) = committed_registry_state else {
        return Ok(None);
    };
    state
        .get("repository_identity_fingerprint")
        .and_then(serde_json::Value::as_str)
        .filter(|identity| is_repository_identity_fingerprint(identity))
        .map(Some)
        .ok_or(())
}

fn setup_error(
    kind: RepositoryIdentitySetupErrorKindV1,
    detail: &'static str,
) -> RepositoryIdentitySetupErrorV1 {
    RepositoryIdentitySetupErrorV1 { kind, detail }
}

fn contract_error(detail: &'static str) -> RepositoryInvocationContractErrorV1 {
    RepositoryInvocationContractErrorV1 { detail }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, Value};
    use std::fs;
    use std::sync::{Arc, Barrier};
    use std::thread;
    use tempfile::tempdir;

    const VECTOR_REPOSITORY_IDENTITY: &str =
        "sha256:ee7147baa7ae6a6d858d9e253e503bc8c90c2ba23a5f1096b5717d1c3231e1db";

    #[test]
    fn reproduces_repository_identity_vector() {
        let entropy: [u8; 32] = std::array::from_fn(|index| index as u8);
        assert_eq!(
            derive_repository_identity(&entropy),
            VECTOR_REPOSITORY_IDENTITY
        );
    }

    #[test]
    fn independently_reproduces_all_five_operation_id_vectors() {
        let entropy: [u8; 32] = std::array::from_fn(|index| (index + 32) as u8);
        let vectors = [
            (
                RepositoryInvocationOperationV1::ApproverBootstrap,
                "bootstrap-2dba41bdd163fdba35657795f59cfe92268a8bdd2aa488ece31c9128d6050c32",
            ),
            (
                RepositoryInvocationOperationV1::ApproverAddCredential,
                "add-credential-12ff3d066b614295e6b4fbc8c99a4471f875da3cb21f76d1405c8a7acefd27aa",
            ),
            (
                RepositoryInvocationOperationV1::ApproverRevokeCredential,
                "revoke-credential-b30d32bcdafdf2a19daf711c7e8b8b4a7abf92c8fc99510e64ae58ab67ebb860",
            ),
            (
                RepositoryInvocationOperationV1::ApproverUpdateMapping,
                "update-mapping-a2e1e8c941078be301bbce5543f1cab33528988c955d778667fb1fb7cb0cc23a",
            ),
            (
                RepositoryInvocationOperationV1::CharterApproval,
                "charter-approval-6170a1a98ad62de3bcf9ed7c4cb53eb810b5245bb210a9ae7f36b840a3464aae",
            ),
        ];
        for (operation, expected) in vectors {
            assert_eq!(
                derive_operation_id(VECTOR_REPOSITORY_IDENTITY, operation, &entropy),
                expected
            );
        }
    }

    #[test]
    fn operation_id_lengths_cover_the_exact_token_range() {
        let entropy = [0u8; 32];
        let lengths = [
            (RepositoryInvocationOperationV1::ApproverBootstrap, 74),
            (RepositoryInvocationOperationV1::ApproverAddCredential, 79),
            (
                RepositoryInvocationOperationV1::ApproverRevokeCredential,
                82,
            ),
            (RepositoryInvocationOperationV1::ApproverUpdateMapping, 79),
            (RepositoryInvocationOperationV1::CharterApproval, 81),
        ];
        for (operation, expected) in lengths {
            assert_eq!(
                derive_operation_id(VECTOR_REPOSITORY_IDENTITY, operation, &entropy).len(),
                expected
            );
        }
    }

    #[test]
    fn deterministic_test_entropy_can_reproduce_failure_without_public_injection() {
        let service = RepositoryInvocationIdentityServiceV1::new();
        assert_eq!(
            service.allocate_repository_identity_with(|_| Err(())),
            Err(RepositoryIdentityAllocationErrorV1::EntropyUnavailable)
        );
        let failure = service
            .allocate_operation_id_with(
                RepositoryInvocationOperationV1::CharterApproval,
                VECTOR_REPOSITORY_IDENTITY,
                |_| Err(()),
            )
            .unwrap_err();
        assert_eq!(
            serde_json::to_value(failure).unwrap(),
            json!({
                "schema_id": PREFLIGHT_SCHEMA_ID,
                "schema_version": SCHEMA_VERSION,
                "operation": "charter_approval",
                "stage": OPERATION_ID_STAGE,
                "status": REFUSED_STATUS,
                "repository_identity_fingerprint": VECTOR_REPOSITORY_IDENTITY,
                "operation_id": null,
                "changed_paths": [],
                "refusal": {
                    "code": OPERATION_ID_ENTROPY_CODE,
                    "message": OPERATION_ID_ENTROPY_MESSAGE,
                    "retryable": true
                },
                "next_actions": [OPERATION_ID_ENTROPY_ACTION]
            })
        );
    }

    #[test]
    fn closed_deserializers_reject_all_mandatory_negative_shapes() {
        let preflight = serde_json::to_value(
            RepositoryInvocationPreflightResultV1::operation_id_entropy_unavailable(
                RepositoryInvocationOperationV1::CharterApproval,
                VECTOR_REPOSITORY_IDENTITY,
            )
            .unwrap(),
        )
        .unwrap();
        for mutation in [
            ("operation_id", json!("charter-approval-forged")),
            ("stage", json!(IDENTITY_STAGE)),
            ("changed_paths", json!(["forged"])),
        ] {
            let mut rejected = preflight.clone();
            rejected[mutation.0] = mutation.1;
            assert!(
                serde_json::from_value::<RepositoryInvocationPreflightResultV1>(rejected).is_err()
            );
        }
        let mut malformed_identity = preflight;
        malformed_identity["repository_identity_fingerprint"] =
            json!(VECTOR_REPOSITORY_IDENTITY.to_uppercase());
        assert!(
            serde_json::from_value::<RepositoryInvocationPreflightResultV1>(malformed_identity)
                .is_err()
        );

        let recovery = serde_json::to_value(
            RepositoryInvocationRecoveryRefusalV1::authority_recovery_blocked(
                RepositoryInvocationOperationV1::ApproverBootstrap,
            ),
        )
        .unwrap();
        for mutation in [
            (
                "repository_identity_fingerprint",
                json!(VECTOR_REPOSITORY_IDENTITY),
            ),
            ("operation_id", json!("bootstrap-forged")),
            ("changed_paths", json!(["forged"])),
            ("schema_id", json!(PREFLIGHT_SCHEMA_ID)),
        ] {
            let mut rejected = recovery.clone();
            rejected[mutation.0] = mutation.1;
            assert!(
                serde_json::from_value::<RepositoryInvocationRecoveryRefusalV1>(rejected).is_err()
            );
        }
        let mut unknown = recovery;
        unknown["detail"] = Value::String("forged".to_owned());
        assert!(serde_json::from_value::<RepositoryInvocationRecoveryRefusalV1>(unknown).is_err());
    }

    #[test]
    fn setup_initializes_exact_state_and_repeated_setup_preserves_bytes() {
        let repo = tempdir().unwrap();
        fs::create_dir(repo.path().join(".handbook")).unwrap();
        let service = RepositoryInvocationIdentityServiceV1::new();

        let created = service.initialize_for_setup(repo.path()).unwrap();
        assert_eq!(
            created.disposition(),
            RepositoryIdentityInitializationDispositionV1::Created
        );
        let path = repo.path().join(REPOSITORY_IDENTITY_REPO_PATH);
        let before = fs::read(&path).unwrap();
        assert_eq!(before.len(), 71);
        assert_eq!(
            std::str::from_utf8(&before).unwrap(),
            created.repository_identity_fingerprint()
        );

        let preserved = service.initialize_for_setup(repo.path()).unwrap();
        assert_eq!(
            preserved.disposition(),
            RepositoryIdentityInitializationDispositionV1::Preserved
        );
        assert_eq!(fs::read(path).unwrap(), before);
        assert_eq!(
            preserved.repository_identity_fingerprint(),
            created.repository_identity_fingerprint()
        );
    }

    #[test]
    fn concurrent_setup_converges_on_one_exact_identity() {
        let repo = tempdir().unwrap();
        fs::create_dir(repo.path().join(".handbook")).unwrap();
        let repo = Arc::new(repo);
        let barrier = Arc::new(Barrier::new(8));
        let mut workers = Vec::new();
        for _ in 0..8 {
            let repo = Arc::clone(&repo);
            let barrier = Arc::clone(&barrier);
            workers.push(thread::spawn(move || {
                barrier.wait();
                RepositoryInvocationIdentityServiceV1::new()
                    .initialize_for_setup(repo.path())
                    .unwrap()
                    .repository_identity_fingerprint()
                    .to_owned()
            }));
        }
        let identities = workers
            .into_iter()
            .map(|worker| worker.join().unwrap())
            .collect::<Vec<_>>();
        assert!(identities.iter().all(|value| value == &identities[0]));
        assert_eq!(
            fs::read(repo.path().join(REPOSITORY_IDENTITY_REPO_PATH)).unwrap(),
            identities[0].as_bytes()
        );
    }

    #[test]
    fn setup_entropy_failure_creates_no_identity_or_temporary_file() {
        let repo = tempdir().unwrap();
        fs::create_dir(repo.path().join(".handbook")).unwrap();
        let failure = RepositoryInvocationIdentityServiceV1::new()
            .initialize_for_setup_with(repo.path(), |_| Err(()))
            .unwrap_err();
        assert_eq!(
            failure.kind(),
            RepositoryIdentitySetupErrorKindV1::EntropyUnavailable
        );
        assert!(!repo.path().join(REPOSITORY_IDENTITY_REPO_PATH).exists());
        assert!(!repo
            .path()
            .join(REPOSITORY_IDENTITY_TEMP_REPO_PATH)
            .exists());
    }

    #[test]
    fn malformed_identity_refuses_without_replacement() {
        let repo = tempdir().unwrap();
        fs::create_dir(repo.path().join(".handbook")).unwrap();
        let identity_path = repo.path().join(REPOSITORY_IDENTITY_REPO_PATH);
        fs::write(&identity_path, format!("{VECTOR_REPOSITORY_IDENTITY}\n")).unwrap();
        let before = fs::read(&identity_path).unwrap();

        let failure = RepositoryInvocationIdentityServiceV1::new()
            .initialize_for_setup(repo.path())
            .unwrap_err();
        assert_eq!(
            failure.kind(),
            RepositoryIdentitySetupErrorKindV1::UnsafeIdentityState
        );
        assert_eq!(fs::read(identity_path).unwrap(), before);
    }

    #[test]
    fn missing_identity_after_setup_returns_exact_preflight_without_new_paths() {
        let repo = tempdir().unwrap();
        fs::create_dir(repo.path().join(".handbook")).unwrap();
        RepositoryInvocationIdentityServiceV1::new()
            .initialize_for_setup(repo.path())
            .unwrap();
        fs::remove_file(repo.path().join(REPOSITORY_IDENTITY_REPO_PATH)).unwrap();
        let before = relative_paths(repo.path());

        let failure = RepositoryInvocationIdentityServiceV1::new()
            .prepare_operation(
                repo.path(),
                RepositoryInvocationOperationV1::ApproverBootstrap,
            )
            .unwrap_err();
        assert!(matches!(
            failure,
            RepositoryInvocationPreparationFailureV1::Preflight(_)
        ));
        assert_eq!(relative_paths(repo.path()), before);
    }

    #[test]
    fn unsafe_shared_recovery_wins_before_identity_preflight() {
        let repo = tempdir().unwrap();
        fs::create_dir_all(
            repo.path()
                .join(".handbook/state/transactions/registry/unknown-evidence"),
        )
        .unwrap();

        let failure = RepositoryInvocationIdentityServiceV1::new()
            .prepare_operation(
                repo.path(),
                RepositoryInvocationOperationV1::ApproverBootstrap,
            )
            .unwrap_err();
        let RepositoryInvocationPreparationFailureV1::Recovery(refusal) = failure else {
            panic!("unsafe predecessor recovery must have precedence");
        };
        assert_eq!(
            serde_json::to_value(refusal).unwrap()["schema_id"],
            RECOVERY_REFUSAL_SCHEMA_ID
        );
        assert!(!repo.path().join(REPOSITORY_IDENTITY_REPO_PATH).exists());
    }

    #[test]
    fn committed_registry_identity_comparison_is_exact() {
        let valid_state = json!({"repository_identity_fingerprint": VECTOR_REPOSITORY_IDENTITY});
        assert_eq!(
            validated_registry_identity(Some(&valid_state)).unwrap(),
            Some(VECTOR_REPOSITORY_IDENTITY)
        );
        assert!(validated_registry_identity(Some(&json!({}))).is_err());
        assert!(validated_registry_identity(Some(&json!({
            "repository_identity_fingerprint": VECTOR_REPOSITORY_IDENTITY.to_uppercase()
        })))
        .is_err());
        assert_eq!(validated_registry_identity(None).unwrap(), None);
        assert!(registry_identity_matches(
            VECTOR_REPOSITORY_IDENTITY,
            Some(VECTOR_REPOSITORY_IDENTITY)
        ));
        assert!(!registry_identity_matches(
            VECTOR_REPOSITORY_IDENTITY,
            Some("sha256:1111111111111111111111111111111111111111111111111111111111111111")
        ));
        assert!(registry_identity_matches(VECTOR_REPOSITORY_IDENTITY, None));
    }

    #[cfg(unix)]
    #[test]
    fn identity_symlink_is_refused_and_preserved() {
        use std::os::unix::fs::symlink;

        let repo = tempdir().unwrap();
        fs::create_dir(repo.path().join(".handbook")).unwrap();
        fs::write(repo.path().join("outside"), VECTOR_REPOSITORY_IDENTITY).unwrap();
        symlink(
            repo.path().join("outside"),
            repo.path().join(REPOSITORY_IDENTITY_REPO_PATH),
        )
        .unwrap();

        let failure = RepositoryInvocationIdentityServiceV1::new()
            .initialize_for_setup(repo.path())
            .unwrap_err();
        assert_eq!(
            failure.kind(),
            RepositoryIdentitySetupErrorKindV1::UnsafeIdentityState
        );
        assert!(
            fs::symlink_metadata(repo.path().join(REPOSITORY_IDENTITY_REPO_PATH))
                .unwrap()
                .file_type()
                .is_symlink()
        );
    }

    fn relative_paths(root: &std::path::Path) -> Vec<String> {
        fn visit(root: &std::path::Path, current: &std::path::Path, paths: &mut Vec<String>) {
            let mut entries = fs::read_dir(current)
                .unwrap()
                .map(|entry| entry.unwrap())
                .collect::<Vec<_>>();
            entries.sort_by_key(|entry| entry.file_name());
            for entry in entries {
                let path = entry.path();
                paths.push(
                    path.strip_prefix(root)
                        .unwrap()
                        .to_string_lossy()
                        .replace('\\', "/"),
                );
                if entry.file_type().unwrap().is_dir() {
                    visit(root, &path, paths);
                }
            }
        }
        let mut paths = Vec::new();
        visit(root, root, &mut paths);
        paths
    }
}
