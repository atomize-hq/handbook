use crate::repo_file_access::{
    read_repo_relative_string, write_repo_relative_bytes, RepoRelativeFileAccessError,
};
use crate::route_state::{acquire_advisory_lock, RouteStateLockGuard};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};

const JOURNAL_SCHEMA_VERSION: &str = "hcm-pipeline-mutation-journal-v1";

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PipelineMutationJournalErrorClassification {
    InvalidOperation,
    LockFailure,
    ReadFailure,
    MalformedEntry,
    WriteFailure,
}

#[derive(Debug)]
pub struct PipelineMutationJournalError {
    pub classification: PipelineMutationJournalErrorClassification,
    pub summary: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PipelineMutationJournalLookup<T> {
    Miss,
    Replay(T),
    Conflict {
        established_request_fingerprint: String,
        actual_request_fingerprint: String,
    },
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct PipelineMutationJournalEntry<T> {
    schema_version: String,
    operation_ref: String,
    idempotency_key_fingerprint: String,
    request_fingerprint: String,
    result: T,
}

pub struct PipelineMutationJournalSession {
    repo_root: PathBuf,
    relative_path: String,
    operation_ref: String,
    idempotency_key_fingerprint: String,
    _lock: RouteStateLockGuard,
}

impl PipelineMutationJournalSession {
    pub fn open(
        repo_root: impl AsRef<Path>,
        operation_ref: &str,
        raw_idempotency_key: &[u8],
    ) -> Result<Self, PipelineMutationJournalError> {
        let operation_identity = operation_ref
            .strip_suffix("@1.0.0")
            .filter(|identity| {
                !identity.is_empty()
                    && identity
                        .chars()
                        .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '.')
            })
            .ok_or_else(|| PipelineMutationJournalError {
                classification: PipelineMutationJournalErrorClassification::InvalidOperation,
                summary: "pipeline mutation operation reference is not journal-admissible"
                    .to_string(),
            })?;
        let idempotency_key_fingerprint =
            scoped_key_fingerprint(operation_ref, raw_idempotency_key);
        let key_segment = idempotency_key_fingerprint.trim_start_matches("sha256:");
        let relative_path = format!(
            ".handbook/state/pipeline/idempotency/{}/{}.json",
            operation_identity.replace('.', "-"),
            key_segment
        );
        let repo_root = repo_root.as_ref().to_path_buf();
        let lock_path = repo_root.join(&relative_path);
        let lock = acquire_advisory_lock(&lock_path).map_err(|_| PipelineMutationJournalError {
            classification: PipelineMutationJournalErrorClassification::LockFailure,
            summary: "pipeline mutation idempotency journal lock is unavailable".to_string(),
        })?;
        Ok(Self {
            repo_root,
            relative_path,
            operation_ref: operation_ref.to_owned(),
            idempotency_key_fingerprint,
            _lock: lock,
        })
    }

    pub fn lookup<T: DeserializeOwned>(
        &self,
        request_fingerprint: &str,
    ) -> Result<PipelineMutationJournalLookup<T>, PipelineMutationJournalError> {
        let body = match read_repo_relative_string(&self.repo_root, &self.relative_path) {
            Ok(body) => body,
            Err(RepoRelativeFileAccessError::Missing(_)) => {
                return Ok(PipelineMutationJournalLookup::Miss)
            }
            Err(_) => {
                return Err(PipelineMutationJournalError {
                    classification: PipelineMutationJournalErrorClassification::ReadFailure,
                    summary: "pipeline mutation idempotency journal cannot be read".to_string(),
                })
            }
        };
        let entry: PipelineMutationJournalEntry<T> =
            serde_json::from_str(&body).map_err(|_| PipelineMutationJournalError {
                classification: PipelineMutationJournalErrorClassification::MalformedEntry,
                summary: "pipeline mutation idempotency journal entry is malformed".to_string(),
            })?;
        if entry.schema_version != JOURNAL_SCHEMA_VERSION
            || entry.operation_ref != self.operation_ref
            || entry.idempotency_key_fingerprint != self.idempotency_key_fingerprint
        {
            return Err(PipelineMutationJournalError {
                classification: PipelineMutationJournalErrorClassification::MalformedEntry,
                summary:
                    "pipeline mutation idempotency journal entry does not match its owner binding"
                        .to_string(),
            });
        }
        if entry.request_fingerprint != request_fingerprint {
            return Ok(PipelineMutationJournalLookup::Conflict {
                established_request_fingerprint: entry.request_fingerprint,
                actual_request_fingerprint: request_fingerprint.to_owned(),
            });
        }
        Ok(PipelineMutationJournalLookup::Replay(entry.result))
    }

    pub fn persist<T: Serialize>(
        &self,
        request_fingerprint: &str,
        result: &T,
    ) -> Result<(), PipelineMutationJournalError> {
        let bytes = serde_json::to_vec(&PipelineMutationJournalEntry {
            schema_version: JOURNAL_SCHEMA_VERSION.to_owned(),
            operation_ref: self.operation_ref.clone(),
            idempotency_key_fingerprint: self.idempotency_key_fingerprint.clone(),
            request_fingerprint: request_fingerprint.to_owned(),
            result,
        })
        .map_err(|_| PipelineMutationJournalError {
            classification: PipelineMutationJournalErrorClassification::MalformedEntry,
            summary: "pipeline mutation idempotency result cannot be serialized".to_string(),
        })?;
        persist_journal_bytes(
            &self.repo_root,
            &self.relative_path,
            &bytes,
            write_repo_relative_bytes,
        )
    }
}

fn persist_journal_bytes<E>(
    repo_root: &Path,
    relative_path: &str,
    bytes: &[u8],
    write: impl FnOnce(&Path, &str, &[u8]) -> Result<(), E>,
) -> Result<(), PipelineMutationJournalError> {
    if write(repo_root, relative_path, bytes).is_ok() {
        return Ok(());
    }
    if read_repo_relative_string(repo_root, relative_path)
        .ok()
        .is_some_and(|body| body.as_bytes() == bytes)
    {
        return Ok(());
    }
    Err(PipelineMutationJournalError {
        classification: PipelineMutationJournalErrorClassification::WriteFailure,
        summary: "pipeline mutation idempotency journal cannot be persisted".to_string(),
    })
}

fn scoped_key_fingerprint(operation_ref: &str, raw_idempotency_key: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(b"handbook.pipeline-mutation-idempotency-v1\0");
    hasher.update(operation_ref.as_bytes());
    hasher.update(b"\0");
    hasher.update(raw_idempotency_key);
    format!("sha256:{:x}", hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::persist_journal_bytes;
    use crate::repo_file_access::write_repo_relative_bytes;

    #[test]
    fn post_commit_write_error_establishes_only_an_exact_journal_entry() {
        let repo_root = tempfile::tempdir().expect("tempdir");
        let relative_path = ".handbook/state/pipeline/idempotency/test/entry.json";
        let bytes = br#"{"entry":"exact"}"#;

        persist_journal_bytes(
            repo_root.path(),
            relative_path,
            bytes,
            |repo_root, relative_path, bytes| {
                write_repo_relative_bytes(repo_root, relative_path, bytes)
                    .expect("simulate committed journal write");
                Err::<(), ()>(())
            },
        )
        .expect("exact committed bytes establish the journal");

        let partial_path = ".handbook/state/pipeline/idempotency/test/partial.json";
        assert!(persist_journal_bytes(
            repo_root.path(),
            partial_path,
            bytes,
            |repo_root, relative_path, _bytes| {
                write_repo_relative_bytes(repo_root, relative_path, b"partial")
                    .expect("simulate partial journal write");
                Err::<(), ()>(())
            },
        )
        .is_err());
    }
}
