#![cfg(feature = "test-support")]

use handbook_engine::{
    SnapshotBoundaryCommitIntentV1, SnapshotBoundaryKindV1, SnapshotGroundingOwnerV1,
    SnapshotPredecessorV1, SnapshotRepositoryErrorKindV1, SnapshotRepositoryMaintenanceV1,
};
use std::sync::{Arc, Barrier};
use tempfile::TempDir;

use handbook_engine::SnapshotRepositoryFaultPointV1;

const WORKSPACE_REF: &str = "handbook.workspace.packet-2-test@1.0.0";
const WORKSPACE_FINGERPRINT: &str =
    "sha256:2222222222222222222222222222222222222222222222222222222222222222";

#[test]
fn first_start_replay_and_reopen_preserve_exact_bytes_and_receipt() {
    let repo = TempDir::new().expect("temporary repository");
    let owner = SnapshotGroundingOwnerV1::new();
    let binding = owner
        .bind_repository(repo.path(), WORKSPACE_REF, WORKSPACE_FINGERPRINT)
        .expect("repository binding");
    let stream = owner.bind_session_stream(&binding);
    let repository = owner
        .open_repository(&binding, &stream)
        .expect("owner repository open");

    let first = repository
        .commit_boundary(intent(
            "key-1",
            "session-1",
            SnapshotBoundaryKindV1::Start,
            None,
        ))
        .expect("first start commits");
    assert_eq!(first.sequence(), 1);
    assert_eq!(
        first.predecessor(),
        &SnapshotPredecessorV1::NoApplicablePrior
    );
    assert!(!first.replayed());
    let boundary_bytes = repository
        .read_record(first.boundary_record())
        .expect("boundary record read");
    let receipt_bytes = repository
        .read_record(first.receipt_record())
        .expect("receipt record read");

    let replay = repository
        .commit_boundary(intent(
            "key-1",
            "session-1",
            SnapshotBoundaryKindV1::Start,
            None,
        ))
        .expect("exact replay");
    assert!(replay.replayed());
    assert_eq!(replay.receipt_record(), first.receipt_record());
    assert_eq!(replay.boundary_record(), first.boundary_record());

    drop(repository);
    let reopened = owner
        .open_repository(&binding, &stream)
        .expect("repository reopen");
    let replay_after_reopen = reopened
        .commit_boundary(intent(
            "key-1",
            "session-1",
            SnapshotBoundaryKindV1::Start,
            None,
        ))
        .expect("stable replay after reopen");
    assert!(replay_after_reopen.replayed());
    assert_eq!(
        reopened
            .read_record(replay_after_reopen.boundary_record())
            .expect("stable boundary bytes"),
        boundary_bytes
    );
    assert_eq!(
        reopened
            .read_record(replay_after_reopen.receipt_record())
            .expect("stable receipt bytes"),
        receipt_bytes
    );
}

#[test]
fn strict_alternation_and_changed_request_conflict_write_nothing() {
    let repo = TempDir::new().expect("temporary repository");
    let owner = SnapshotGroundingOwnerV1::new();
    let binding = owner
        .bind_repository(repo.path(), WORKSPACE_REF, WORKSPACE_FINGERPRINT)
        .expect("repository binding");
    let stream = owner.bind_session_stream(&binding);
    let repository = owner
        .open_repository(&binding, &stream)
        .expect("repository open");

    repository
        .commit_boundary(intent(
            "key-1",
            "session-1",
            SnapshotBoundaryKindV1::Start,
            None,
        ))
        .expect("start commits");
    let before = repository.head().expect("head read").expect("head exists");

    let second_start = repository
        .commit_boundary(intent(
            "key-2",
            "session-2",
            SnapshotBoundaryKindV1::Start,
            None,
        ))
        .expect_err("second open start must fail");
    assert_eq!(
        second_start.kind(),
        SnapshotRepositoryErrorKindV1::SequenceConflict
    );

    let changed_key = repository
        .commit_boundary(intent(
            "key-1",
            "session-1",
            SnapshotBoundaryKindV1::End,
            Some(br#"{"delta":"changed"}"#),
        ))
        .expect_err("changed request with same key must conflict");
    assert_eq!(
        changed_key.kind(),
        SnapshotRepositoryErrorKindV1::IdempotencyConflict
    );
    assert_eq!(repository.head().expect("head read"), Some(before.clone()));

    let wrong_session_end = repository
        .commit_boundary(intent(
            "key-3",
            "session-2",
            SnapshotBoundaryKindV1::End,
            Some(br#"{"delta":"wrong-session"}"#),
        ))
        .expect_err("end must bind exact open session");
    assert_eq!(
        wrong_session_end.kind(),
        SnapshotRepositoryErrorKindV1::SequenceConflict
    );
    assert_eq!(repository.head().expect("head read"), Some(before));
}

#[test]
fn end_and_next_start_bind_exact_applicable_predecessors() {
    let repo = TempDir::new().expect("temporary repository");
    let owner = SnapshotGroundingOwnerV1::new();
    let binding = owner
        .bind_repository(repo.path(), WORKSPACE_REF, WORKSPACE_FINGERPRINT)
        .expect("repository binding");
    let stream = owner.bind_session_stream(&binding);
    let repository = owner
        .open_repository(&binding, &stream)
        .expect("repository open");

    let start = repository
        .commit_boundary(intent(
            "key-1",
            "session-1",
            SnapshotBoundaryKindV1::Start,
            None,
        ))
        .expect("start commits");
    let end = repository
        .commit_boundary(intent(
            "key-2",
            "session-1",
            SnapshotBoundaryKindV1::End,
            Some(br#"{"delta":"end"}"#),
        ))
        .expect("end commits");
    assert_eq!(end.sequence(), 2);
    assert_eq!(
        end.predecessor(),
        &SnapshotPredecessorV1::ApplicablePrior {
            boundary_ref: start.boundary_record().exact_ref().to_owned(),
            boundary_fingerprint: start.boundary_record().fingerprint().to_owned(),
        }
    );

    let next_start = repository
        .commit_boundary(intent(
            "key-3",
            "session-2",
            SnapshotBoundaryKindV1::Start,
            Some(br#"{"delta":"between"}"#),
        ))
        .expect("next start commits");
    assert_eq!(next_start.sequence(), 3);
    assert_eq!(
        next_start.predecessor(),
        &SnapshotPredecessorV1::ApplicablePrior {
            boundary_ref: end.boundary_record().exact_ref().to_owned(),
            boundary_fingerprint: end.boundary_record().fingerprint().to_owned(),
        }
    );
}

#[test]
fn concurrent_same_stream_requests_serialize_without_skipping() {
    let repo = Arc::new(TempDir::new().expect("temporary repository"));
    let barrier = Arc::new(Barrier::new(2));
    let mut threads = Vec::new();
    for index in 0..2 {
        let repo = Arc::clone(&repo);
        let barrier = Arc::clone(&barrier);
        threads.push(std::thread::spawn(move || {
            let owner = SnapshotGroundingOwnerV1::new();
            let binding = owner
                .bind_repository(repo.path(), WORKSPACE_REF, WORKSPACE_FINGERPRINT)
                .expect("repository binding");
            let stream = owner.bind_session_stream(&binding);
            barrier.wait();
            let repository = owner
                .open_repository(&binding, &stream)
                .expect("repository open");
            repository.commit_boundary(intent(
                &format!("key-{index}"),
                &format!("session-{index}"),
                SnapshotBoundaryKindV1::Start,
                None,
            ))
        }));
    }
    let results = threads
        .into_iter()
        .map(|thread| thread.join().expect("thread joins"))
        .collect::<Vec<_>>();
    assert_eq!(
        results.iter().filter(|result| result.is_ok()).count(),
        1,
        "concurrent results: {results:?}"
    );
    assert_eq!(
        results
            .iter()
            .filter_map(|result| result.as_ref().err())
            .filter(|error| error.kind() == SnapshotRepositoryErrorKindV1::SequenceConflict)
            .count(),
        1,
        "concurrent results: {results:?}"
    );
}

#[test]
fn owner_open_derives_holds_and_compaction_without_rewriting_records() {
    let repo = TempDir::new().expect("temporary repository");
    let owner = SnapshotGroundingOwnerV1::new();
    let binding = owner
        .bind_repository(repo.path(), WORKSPACE_REF, WORKSPACE_FINGERPRINT)
        .expect("repository binding");
    let stream = owner.bind_session_stream(&binding);
    let repository = owner
        .open_repository(&binding, &stream)
        .expect("repository open");
    let start = repository
        .commit_boundary(intent(
            "key-1",
            "session-1",
            SnapshotBoundaryKindV1::Start,
            None,
        ))
        .expect("start commits");
    let start_bytes = repository
        .read_record(start.boundary_record())
        .expect("start bytes");
    drop(repository);

    let reopened = owner
        .open_repository(&binding, &stream)
        .expect("maintenance open");
    assert!(reopened
        .active_holds()
        .iter()
        .any(|hold| hold.starts_with("open-session:")));
    assert_eq!(
        reopened
            .read_record(start.boundary_record())
            .expect("stable record"),
        start_bytes
    );
    assert!(matches!(
        reopened.maintenance(),
        SnapshotRepositoryMaintenanceV1::Complete { .. }
            | SnapshotRepositoryMaintenanceV1::Deferred { .. }
    ));
}

#[test]
fn crash_boundaries_roll_back_or_roll_forward_without_false_success() {
    use SnapshotRepositoryFaultPointV1::{
        AfterCommitMarker, AfterDerivedIndexes, AfterJournalFinalize, AfterRecordsInstalled,
    };

    for (fault, committed) in [
        (AfterRecordsInstalled, false),
        (AfterDerivedIndexes, false),
        (AfterCommitMarker, true),
        (AfterJournalFinalize, true),
    ] {
        let repo = TempDir::new().expect("temporary repository");
        let owner = SnapshotGroundingOwnerV1::new();
        let binding = owner
            .bind_repository(repo.path(), WORKSPACE_REF, WORKSPACE_FINGERPRINT)
            .expect("repository binding");
        let stream = owner.bind_session_stream(&binding);
        let repository = owner
            .open_repository(&binding, &stream)
            .expect("repository open");
        let failure = repository
            .commit_boundary_with_fault_for_testing(
                intent(
                    "crash-key",
                    "session-1",
                    SnapshotBoundaryKindV1::Start,
                    None,
                ),
                fault,
            )
            .expect_err("fault boundary cannot return success");
        assert_eq!(
            failure.kind(),
            SnapshotRepositoryErrorKindV1::DurabilityUncertain
        );
        drop(repository);

        let reopened = owner
            .open_repository(&binding, &stream)
            .expect("recovery open");
        if committed {
            let replay = reopened
                .commit_boundary(intent(
                    "crash-key",
                    "session-1",
                    SnapshotBoundaryKindV1::Start,
                    None,
                ))
                .expect("committed fault replays");
            assert!(replay.replayed());
            assert_eq!(replay.sequence(), 1);
        } else {
            assert_eq!(reopened.head().expect("head read"), None);
            let retried = reopened
                .commit_boundary(intent(
                    "crash-key",
                    "session-1",
                    SnapshotBoundaryKindV1::Start,
                    None,
                ))
                .expect("rolled-back fault permits a fresh commit");
            assert!(!retried.replayed());
            assert_eq!(retried.sequence(), 1);
        }
    }
}

#[test]
fn raw_idempotency_key_never_enters_repository_bytes() {
    let repo = TempDir::new().expect("temporary repository");
    let owner = SnapshotGroundingOwnerV1::new();
    let binding = owner
        .bind_repository(repo.path(), WORKSPACE_REF, WORKSPACE_FINGERPRINT)
        .expect("repository binding");
    let stream = owner.bind_session_stream(&binding);
    let repository = owner
        .open_repository(&binding, &stream)
        .expect("repository open");
    let raw_key = "opaque-key-material-that-must-not-persist";
    repository
        .commit_boundary(intent(
            raw_key,
            "session-1",
            SnapshotBoundaryKindV1::Start,
            None,
        ))
        .expect("start commits");
    drop(repository);
    drop(binding);

    let state_root = repo.path().join(".handbook/state/snapshot-memory/v1");
    let mut pending = vec![state_root];
    while let Some(path) = pending.pop() {
        for entry in std::fs::read_dir(path).expect("state directory") {
            let path = entry.expect("state entry").path();
            if path.is_dir() {
                pending.push(path);
            } else {
                let bytes = std::fs::read(path).expect("state file bytes");
                assert!(!String::from_utf8_lossy(&bytes).contains(raw_key));
            }
        }
    }
}

#[test]
fn owner_open_records_exact_implicit_hold_release_audit() {
    let repo = TempDir::new().expect("temporary repository");
    let owner = SnapshotGroundingOwnerV1::new();
    let binding = owner
        .bind_repository(repo.path(), WORKSPACE_REF, WORKSPACE_FINGERPRINT)
        .expect("repository binding");
    let stream = owner.bind_session_stream(&binding);
    let repository = owner
        .open_repository(&binding, &stream)
        .expect("repository open");
    let start = repository
        .commit_boundary(intent(
            "key-1",
            "session-1",
            SnapshotBoundaryKindV1::Start,
            None,
        ))
        .expect("start commits");
    let released_hold = format!("open-session:{}", start.boundary_record().exact_ref());
    repository
        .commit_boundary(intent(
            "key-2",
            "session-1",
            SnapshotBoundaryKindV1::End,
            Some(br#"{"delta":"end"}"#),
        ))
        .expect("end commits");
    drop(repository);

    let reopened = owner
        .open_repository(&binding, &stream)
        .expect("release maintenance open");
    assert!(!reopened.active_holds().contains(&released_hold));
    let audit_root = repo
        .path()
        .join(".handbook/state/snapshot-memory/v1/maintenance/audit");
    let audits = std::fs::read_dir(audit_root)
        .expect("audit directory")
        .map(|entry| std::fs::read(entry.expect("audit entry").path()).expect("audit bytes"))
        .collect::<Vec<_>>();
    assert!(audits.iter().any(|bytes| {
        String::from_utf8_lossy(bytes)
            .contains(&format!("\"released_holds\":[\"{released_hold}\"]"))
    }));
}

#[test]
fn owner_open_prunes_only_verified_orphans_and_reports_deferred_work() {
    use handbook_engine::DefinitionFingerprint;

    let repo = TempDir::new().expect("temporary repository");
    let owner = SnapshotGroundingOwnerV1::new();
    let binding = owner
        .bind_repository(repo.path(), WORKSPACE_REF, WORKSPACE_FINGERPRINT)
        .expect("repository binding");
    let stream = owner.bind_session_stream(&binding);
    drop(
        owner
            .open_repository(&binding, &stream)
            .expect("layout open"),
    );
    drop(binding);

    let delta_root = repo
        .path()
        .join(".handbook/state/snapshot-memory/v1/records/delta");
    for index in 0..70 {
        let bytes = format!("{{\"orphan\":{index}}}").into_bytes();
        let fingerprint = DefinitionFingerprint::from_bytes(&bytes).to_string();
        let name = fingerprint
            .strip_prefix("sha256:")
            .expect("sha256 fingerprint");
        std::fs::write(delta_root.join(format!("{name}.json")), bytes)
            .expect("orphan fixture write");
    }

    let binding = owner
        .bind_repository(repo.path(), WORKSPACE_REF, WORKSPACE_FINGERPRINT)
        .expect("reopened binding");
    let stream = owner.bind_session_stream(&binding);
    let reopened = owner
        .open_repository(&binding, &stream)
        .expect("bounded maintenance open");
    assert_eq!(
        reopened.maintenance(),
        &SnapshotRepositoryMaintenanceV1::Deferred {
            pruned_records: 64,
            remaining_records: 6,
            active_holds: 0,
        }
    );
    assert_eq!(
        std::fs::read_dir(delta_root)
            .expect("delta record directory")
            .count(),
        6
    );
}

#[test]
fn durable_repository_binding_refuses_workspace_substitution_before_first_start() {
    let repo = TempDir::new().expect("temporary repository");
    let owner = SnapshotGroundingOwnerV1::new();
    let first_binding = owner
        .bind_repository(repo.path(), WORKSPACE_REF, WORKSPACE_FINGERPRINT)
        .expect("first repository binding");
    let first_stream = owner.bind_session_stream(&first_binding);
    drop(
        owner
            .open_repository(&first_binding, &first_stream)
            .expect("first binding initializes repository identity"),
    );

    let substituted_binding = owner
        .bind_repository(
            repo.path(),
            "handbook.workspace.substituted@1.0.0",
            "sha256:3333333333333333333333333333333333333333333333333333333333333333",
        )
        .expect("logical substituted binding");
    let substituted_stream = owner.bind_session_stream(&substituted_binding);
    let refusal = owner
        .open_repository(&substituted_binding, &substituted_stream)
        .err()
        .expect("physical repository cannot be rebound");
    assert_eq!(
        refusal.kind(),
        SnapshotRepositoryErrorKindV1::RepositoryMismatch
    );
    assert_eq!(
        owner
            .open_repository(&first_binding, &first_stream)
            .expect("original binding still opens")
            .head()
            .expect("head read"),
        None
    );
}

fn intent(
    key: &str,
    session: &str,
    kind: SnapshotBoundaryKindV1,
    delta: Option<&[u8]>,
) -> SnapshotBoundaryCommitIntentV1 {
    SnapshotBoundaryCommitIntentV1::new(
        key,
        session,
        kind,
        br#"{"accounted":true}"#.to_vec(),
        delta.map(ToOwned::to_owned),
        br#"{"grounding":"bounded"}"#.to_vec(),
    )
}
