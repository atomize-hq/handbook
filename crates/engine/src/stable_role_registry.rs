use crate::canonical_repo_support::{CanonicalWorkspace, RepoRelativeFileAccessError};
use crate::definition_identity::{
    fingerprint_serializable, parse_definition_yaml, DefinitionFingerprint, ExactDefinitionRef,
    RegistryLoadError, RegistryLoadErrorKind, SourceByteBudget, MAX_SOURCE_DOCUMENT_BYTES,
};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::Path;

const CORE_1_0_REF: &str = "handbook.roles.core@1.0.0";
const CORE_1_1_REF: &str = "handbook.roles.core@1.1.0";
const CORE_1_0_BYTES: &[u8] =
    include_bytes!("../definitions/stable-roles/handbook.roles.core/1.0.0.yaml");
const CORE_1_1_BYTES: &[u8] =
    include_bytes!("../definitions/stable-roles/handbook.roles.core/1.1.0.yaml");

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StableRoleCategory {
    Artifact,
    Workflow,
    Governance,
    Evidence,
    Organizational,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StableRoleDefinition {
    role_id: String,
    canonical_display_label: String,
    category: StableRoleCategory,
}

impl StableRoleDefinition {
    pub fn role_id(&self) -> &str {
        &self.role_id
    }

    pub fn canonical_display_label(&self) -> &str {
        &self.canonical_display_label
    }

    pub fn category(&self) -> StableRoleCategory {
        self.category
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StableRoleRegistry {
    exact_ref: ExactDefinitionRef,
    roles: Vec<StableRoleDefinition>,
    fingerprint: DefinitionFingerprint,
}

impl StableRoleRegistry {
    pub fn load_builtin(exact_ref: &ExactDefinitionRef) -> Result<Self, RegistryLoadError> {
        let bytes = match exact_ref.as_str() {
            CORE_1_0_REF => CORE_1_0_BYTES,
            CORE_1_1_REF => CORE_1_1_BYTES,
            _ => {
                return Err(RegistryLoadError::new(
                    RegistryLoadErrorKind::StableRoleRegistryMismatch,
                    "the selected stable-role registry is not a package-owned core registry",
                ))
            }
        };
        let registry = Self::parse_yaml(bytes)?;
        if registry.exact_ref != *exact_ref {
            return Err(RegistryLoadError::new(
                RegistryLoadErrorKind::StableRoleRegistryMismatch,
                "the package-owned stable-role registry identity does not match the selection",
            ));
        }
        Ok(registry)
    }

    pub fn parse_yaml(bytes: &[u8]) -> Result<Self, RegistryLoadError> {
        let value = parse_definition_yaml(bytes)?;
        let authored: AuthoredStableRoleRegistry =
            serde_json::from_value(value).map_err(classify_typed_decode_error)?;
        authored.validate()
    }

    pub fn exact_ref(&self) -> &ExactDefinitionRef {
        &self.exact_ref
    }

    pub fn fingerprint(&self) -> &DefinitionFingerprint {
        &self.fingerprint
    }

    pub fn roles(&self) -> &[StableRoleDefinition] {
        &self.roles
    }

    pub fn role(&self, role_id: &str) -> Option<&StableRoleDefinition> {
        self.roles.iter().find(|role| role.role_id == role_id)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct AuthoredStableRoleRegistry {
    schema_id: String,
    schema_version: String,
    registry_id: String,
    registry_version: String,
    roles: Vec<StableRoleDefinition>,
    #[serde(skip_serializing)]
    registry_fingerprint: String,
}

impl AuthoredStableRoleRegistry {
    fn validate(self) -> Result<StableRoleRegistry, RegistryLoadError> {
        if self.schema_id != "handbook.stable-role-registry" || self.schema_version != "1.0" {
            return Err(RegistryLoadError::new(
                RegistryLoadErrorKind::UnsupportedRecord,
                "stable-role registry must use handbook.stable-role-registry / 1.0",
            ));
        }
        let exact_ref = ExactDefinitionRef::new(&self.registry_id, &self.registry_version)?;
        if self.roles.is_empty() {
            return Err(RegistryLoadError::new(
                RegistryLoadErrorKind::UnknownStableRole,
                "stable-role registry must contain at least one role",
            ));
        }

        let mut role_ids = HashSet::with_capacity(self.roles.len());
        for role in &self.roles {
            validate_role_id(&role.role_id)?;
            validate_display_label(&role.canonical_display_label)?;
            if !role_ids.insert(role.role_id.as_str()) {
                return Err(RegistryLoadError::at(
                    RegistryLoadErrorKind::DuplicateIdentity,
                    "roles",
                    "stable-role ID is duplicated",
                ));
            }
        }

        let supplied = DefinitionFingerprint::parse(&self.registry_fingerprint)?;
        let computed = fingerprint_serializable(&self)?;
        if supplied != computed {
            return Err(RegistryLoadError::new(
                RegistryLoadErrorKind::FingerprintMismatch,
                "stable-role registry fingerprint does not match normalized source bytes",
            ));
        }

        Ok(StableRoleRegistry {
            exact_ref,
            roles: self.roles,
            fingerprint: computed,
        })
    }
}

pub(crate) fn read_trusted_repo_source(
    repo_root: &Path,
    source_path: &str,
    budget: &mut SourceByteBudget,
) -> Result<(String, Vec<u8>), RegistryLoadError> {
    let workspace = CanonicalWorkspace::new(repo_root);
    let normalized = workspace
        .normalize_repo_relative(source_path)
        .map_err(|detail| {
            RegistryLoadError::at(
                RegistryLoadErrorKind::InvalidSourcePath,
                "source_path",
                detail,
            )
        })?;
    let normalized_path = normalized.as_str().to_owned();
    let trusted = workspace
        .trusted_read_strict(&normalized)
        .map_err(|error| {
            let (kind, detail) = match error {
                RepoRelativeFileAccessError::Missing(_) => (
                    RegistryLoadErrorKind::MissingSource,
                    "source file does not exist",
                ),
                RepoRelativeFileAccessError::InvalidPath(_) => (
                    RegistryLoadErrorKind::InvalidSourcePath,
                    "source path is invalid",
                ),
                RepoRelativeFileAccessError::SymlinkNotAllowed(_) => (
                    RegistryLoadErrorKind::SymlinkSource,
                    "source path contains a symlink",
                ),
                RepoRelativeFileAccessError::NotRegularFile(_) => (
                    RegistryLoadErrorKind::NonRegularSource,
                    "source path is not a regular file",
                ),
                RepoRelativeFileAccessError::ReadFailure { .. } => (
                    RegistryLoadErrorKind::SourceReadFailure,
                    "source metadata could not be read",
                ),
            };
            RegistryLoadError::at(kind, &normalized_path, detail)
        })?;
    let remaining = budget.remaining_bytes();
    let maximum = remaining.min(MAX_SOURCE_DOCUMENT_BYTES);
    let (bytes, exceeded) = trusted.read_bytes_bounded(maximum).map_err(|_| {
        RegistryLoadError::at(
            RegistryLoadErrorKind::SourceReadFailure,
            &normalized_path,
            "source bytes could not be read",
        )
    })?;
    if exceeded {
        let (kind, detail) = if remaining < MAX_SOURCE_DOCUMENT_BYTES {
            (
                RegistryLoadErrorKind::AggregateLimitExceeded,
                "aggregate source bytes exceed the 8 MiB limit",
            )
        } else {
            (
                RegistryLoadErrorKind::SourceLimitExceeded,
                "source document exceeds the 1 MiB limit",
            )
        };
        return Err(RegistryLoadError::at(kind, &normalized_path, detail));
    }
    budget.admit(bytes.len())?;
    Ok((normalized_path, bytes))
}

fn validate_role_id(role_id: &str) -> Result<(), RegistryLoadError> {
    let valid = !role_id.is_empty()
        && role_id.is_ascii()
        && role_id.split('_').all(|segment| {
            !segment.is_empty()
                && segment.as_bytes()[0].is_ascii_lowercase()
                && segment
                    .bytes()
                    .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit())
        });
    if !valid {
        return Err(RegistryLoadError::at(
            RegistryLoadErrorKind::UnknownStableRole,
            "roles",
            "stable-role ID violates the lowercase ASCII grammar",
        ));
    }
    Ok(())
}

fn validate_display_label(label: &str) -> Result<(), RegistryLoadError> {
    if label.is_empty() || label.trim() != label || label.chars().any(char::is_control) {
        return Err(RegistryLoadError::new(
            RegistryLoadErrorKind::UnknownStableRole,
            "stable-role display label must be non-empty, trimmed, and printable",
        ));
    }
    Ok(())
}

fn classify_typed_decode_error(error: serde_json::Error) -> RegistryLoadError {
    let rendered = error.to_string();
    let (kind, detail) = if rendered.contains("unknown field") {
        (
            RegistryLoadErrorKind::UnknownField,
            "stable-role registry contains an unknown field",
        )
    } else if rendered.contains("unknown variant") {
        (
            RegistryLoadErrorKind::InvalidStableRoleCategory,
            "stable-role registry contains an invalid role category",
        )
    } else {
        (
            RegistryLoadErrorKind::SyntaxError,
            "stable-role registry does not match its closed typed record",
        )
    };
    RegistryLoadError::new(kind, detail)
}

#[cfg(test)]
mod tests {
    use super::read_trusted_repo_source;
    use crate::{RegistryLoadErrorKind, SourceByteBudget};

    #[test]
    fn trusted_sources_reject_missing_directories_and_parent_traversal() {
        let repo = tempfile::tempdir().expect("repo");
        std::fs::create_dir(repo.path().join("directory")).expect("directory");

        for (path, expected) in [
            ("missing.yaml", RegistryLoadErrorKind::MissingSource),
            ("directory", RegistryLoadErrorKind::NonRegularSource),
            ("../escape.yaml", RegistryLoadErrorKind::InvalidSourcePath),
        ] {
            let error =
                read_trusted_repo_source(repo.path(), path, &mut SourceByteBudget::default())
                    .expect_err(path);
            assert_eq!(error.kind(), expected);
        }
    }

    #[cfg(unix)]
    #[test]
    fn trusted_sources_reject_symlinks() {
        use std::os::unix::fs::symlink;
        use std::sync::mpsc;
        use std::time::Duration;

        let repo = tempfile::tempdir().expect("repo");
        std::fs::write(repo.path().join("target.yaml"), b"value: true\n").expect("target");
        symlink("target.yaml", repo.path().join("link.yaml")).expect("symlink");

        let error =
            read_trusted_repo_source(repo.path(), "link.yaml", &mut SourceByteBudget::default())
                .expect_err("symlink must refuse");
        assert_eq!(error.kind(), RegistryLoadErrorKind::SymlinkSource);

        std::fs::create_dir_all(repo.path().join("target/sub")).expect("target directory");
        std::fs::write(repo.path().join("target/sub/value.yaml"), b"value: true\n")
            .expect("nested target");
        symlink("target", repo.path().join("link-directory")).expect("directory symlink");
        let error = read_trusted_repo_source(
            repo.path(),
            "link-directory/sub/value.yaml",
            &mut SourceByteBudget::default(),
        )
        .expect_err("intermediate symlink must refuse distinctly");
        assert_eq!(error.kind(), RegistryLoadErrorKind::SymlinkSource);

        std::fs::write(repo.path().join("ordinary"), b"not a directory\n")
            .expect("ordinary intermediate");
        let error = read_trusted_repo_source(
            repo.path(),
            "ordinary/value.yaml",
            &mut SourceByteBudget::default(),
        )
        .expect_err("ordinary intermediate must be non-regular");
        assert_eq!(error.kind(), RegistryLoadErrorKind::NonRegularSource);

        assert!(
            std::process::Command::new("mkfifo")
                .arg(repo.path().join("source.pipe"))
                .status()
                .expect("run mkfifo")
                .success(),
            "mkfifo must create the non-regular source fixture"
        );
        let repo_path = repo.path().to_path_buf();
        let (sender, receiver) = mpsc::channel();
        std::thread::spawn(move || {
            let result = read_trusted_repo_source(
                &repo_path,
                "source.pipe",
                &mut SourceByteBudget::default(),
            )
            .map(|_| None)
            .unwrap_or_else(|error| Some(error.kind()));
            sender.send(result).expect("send FIFO result");
        });
        match receiver.recv_timeout(Duration::from_millis(250)) {
            Ok(kind) => assert_eq!(kind, Some(RegistryLoadErrorKind::NonRegularSource)),
            Err(mpsc::RecvTimeoutError::Timeout) => {
                let _writer = std::fs::OpenOptions::new()
                    .write(true)
                    .open(repo.path().join("source.pipe"))
                    .expect("release blocked FIFO reader");
                let _ = receiver.recv_timeout(Duration::from_secs(1));
                panic!("FIFO source open blocked before non-regular refusal");
            }
            Err(error) => panic!("FIFO result channel failed: {error}"),
        }
    }
}
