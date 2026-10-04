use handbook_engine::{
    RepositoryDurabilityErrorKindV1, RepositoryInitializationClaimV1,
    RepositoryInitializationDispositionV1, RepositoryInitializationGuardV1,
    RepositoryInvocationIdentityServiceV1, RepositoryReadV1, RetainedRepositoryRootV1,
    REPOSITORY_IDENTITY_REPO_PATH, REPOSITORY_INITIALIZATION_CLAIM_PATH_V1,
    REPOSITORY_INITIALIZATION_LOCK_PATH_V1, REPOSITORY_INITIALIZATION_RECEIPT_PATH_V1,
};
use std::sync::mpsc;
use std::time::Duration;

const OWNER: &str = "handbook-contracts.admission-deferral.v2";
const TERMINAL: &str = ".handbook/test-native-terminal.json";
const NONCOMMIT: &str = ".handbook-admission-deferral-noncommit.v2.json";

fn prepare(
    repo: &std::path::Path,
) -> (
    handbook_engine::PreparedRepositoryIdentityV1,
    RepositoryInitializationClaimV1,
) {
    let identity = RepositoryInvocationIdentityServiceV1::new()
        .allocate_prepared_repository_identity()
        .unwrap();
    let root = RetainedRepositoryRootV1::open_exact(repo).unwrap();
    let claim = RepositoryInitializationClaimV1::prepare(
        repo,
        &root,
        OWNER,
        &format!("sha256:{}", "1".repeat(64)),
        identity.repository_identity_fingerprint(),
    )
    .unwrap();
    (identity, claim)
}

#[test]
fn locator_claim_preparation_is_pure_closed_and_exact() {
    let repo = tempfile::tempdir().unwrap();
    let (_, claim) = prepare(repo.path());
    assert!(std::fs::read_dir(repo.path()).unwrap().next().is_none());
    let bytes = claim.canonical_bytes().unwrap();
    let reopened: RepositoryInitializationClaimV1 = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(reopened, claim);
    assert_eq!(reopened.canonical_bytes().unwrap(), bytes);
    for field in [
        "claim_fingerprint",
        "native_owner",
        "root_path_digest",
        "request_digest",
    ] {
        let mut changed = serde_json::to_value(&claim).unwrap();
        changed[field] = serde_json::json!("substituted");
        assert!(serde_json::from_value::<RepositoryInitializationClaimV1>(changed).is_err());
    }
    let mut unknown = serde_json::to_value(&claim).unwrap();
    unknown["permission"] = serde_json::json!("approved");
    assert!(serde_json::from_value::<RepositoryInitializationClaimV1>(unknown).is_err());
    let original = String::from_utf8(bytes).unwrap();
    let duplicate = format!("{{\"schema_id\":\"ignored\",{}", &original[1..]);
    assert!(serde_json::from_str::<RepositoryInitializationClaimV1>(&duplicate).is_err());
}

#[test]
fn pending_claim_blocks_ordinary_and_changed_prepared_identity() {
    let repo = tempfile::tempdir().unwrap();
    let (identity, claim) = prepare(repo.path());
    let guard = RepositoryInitializationGuardV1::acquire(repo.path()).unwrap();
    guard.retain_native_claim(&claim).unwrap();
    assert!(guard.validate_ordinary_setup().is_err());
    guard
        .root()
        .create_directory_new_durable(".handbook")
        .unwrap();
    let replacement = RepositoryInvocationIdentityServiceV1::new()
        .allocate_prepared_repository_identity()
        .unwrap();
    assert!(RepositoryInvocationIdentityServiceV1::new()
        .persist_prepared_repository_identity_locked(&guard, replacement)
        .is_err());
    assert_eq!(
        guard
            .root()
            .read_regular_bounded(REPOSITORY_IDENTITY_REPO_PATH, 71)
            .unwrap(),
        RepositoryReadV1::Absent
    );
    RepositoryInvocationIdentityServiceV1::new()
        .persist_prepared_repository_identity_locked(&guard, identity)
        .unwrap();
    guard.release().unwrap();
    assert!(RepositoryInvocationIdentityServiceV1::new()
        .initialize_for_setup(repo.path())
        .is_err());
    assert_eq!(
        std::fs::read_to_string(repo.path().join(REPOSITORY_IDENTITY_REPO_PATH)).unwrap(),
        claim.proposed_repository_identity()
    );
}

#[test]
fn exact_native_terminal_receipt_allows_refresh_and_rejects_identity_healing() {
    let repo = tempfile::tempdir().unwrap();
    let (identity, claim) = prepare(repo.path());
    let guard = RepositoryInitializationGuardV1::acquire(repo.path()).unwrap();
    guard.retain_native_claim(&claim).unwrap();
    guard
        .root()
        .create_directory_new_durable(".handbook")
        .unwrap();
    guard
        .root()
        .write_new_durable(TERMINAL, b"{\"owner_terminal\":true}")
        .unwrap();
    assert!(guard
        .complete_native_claim(
            &claim,
            RepositoryInitializationDispositionV1::Committed,
            TERMINAL,
            b"{\"owner_terminal\":true}"
        )
        .is_err());
    RepositoryInvocationIdentityServiceV1::new()
        .persist_prepared_repository_identity_locked(&guard, identity)
        .unwrap();
    for _ in 0..2 {
        guard
            .complete_native_claim(
                &claim,
                RepositoryInitializationDispositionV1::Committed,
                TERMINAL,
                b"{\"owner_terminal\":true}",
            )
            .unwrap();
    }
    guard.validate_ordinary_setup().unwrap();
    let receipt =
        std::fs::read(repo.path().join(REPOSITORY_INITIALIZATION_RECEIPT_PATH_V1)).unwrap();
    guard.release().unwrap();
    let ordinary = RepositoryInvocationIdentityServiceV1::new()
        .initialize_for_setup(repo.path())
        .unwrap();
    assert_eq!(
        ordinary.repository_identity_fingerprint(),
        claim.proposed_repository_identity()
    );
    assert_eq!(
        std::fs::read(repo.path().join(REPOSITORY_INITIALIZATION_RECEIPT_PATH_V1)).unwrap(),
        receipt
    );
    std::fs::remove_file(repo.path().join(REPOSITORY_IDENTITY_REPO_PATH)).unwrap();
    assert!(RepositoryInvocationIdentityServiceV1::new()
        .initialize_for_setup(repo.path())
        .is_err());
    assert!(!repo.path().join(REPOSITORY_IDENTITY_REPO_PATH).exists());
}

#[test]
fn claim_only_noncommit_retains_closed_reservation_without_creating_handbook() {
    let repo = tempfile::tempdir().unwrap();
    let (_, claim) = prepare(repo.path());
    let guard = RepositoryInitializationGuardV1::acquire(repo.path()).unwrap();
    guard.retain_native_claim(&claim).unwrap();
    guard
        .root()
        .write_new_durable(NONCOMMIT, b"{\"owner_noncommit\":true}")
        .unwrap();
    guard
        .complete_native_claim(
            &claim,
            RepositoryInitializationDispositionV1::NotCommitted,
            NONCOMMIT,
            b"{\"owner_noncommit\":true}",
        )
        .unwrap();
    guard
        .require_native_terminal(
            &claim,
            RepositoryInitializationDispositionV1::NotCommitted,
            NONCOMMIT,
            b"{\"owner_noncommit\":true}",
        )
        .unwrap();
    assert!(guard.require_fresh().is_err());
    assert!(guard.validate_ordinary_setup().is_err());
    guard.release().unwrap();
    assert!(RepositoryInvocationIdentityServiceV1::new()
        .initialize_for_setup(repo.path())
        .is_err());
    assert!(!repo.path().join(".handbook").exists());
}

#[test]
fn orphaned_native_noncommit_witness_freezes_fresh_claim_and_ordinary_identity_setup() {
    let repo = tempfile::tempdir().unwrap();
    let (_, claim) = prepare(repo.path());
    let guard = RepositoryInitializationGuardV1::acquire(repo.path()).unwrap();
    guard.retain_native_claim(&claim).unwrap();
    guard
        .root()
        .write_new_durable(NONCOMMIT, b"opaque owner noncommit witness")
        .unwrap();
    guard
        .complete_native_claim(
            &claim,
            RepositoryInitializationDispositionV1::NotCommitted,
            NONCOMMIT,
            b"opaque owner noncommit witness",
        )
        .unwrap();
    guard.release().unwrap();
    for path in [
        REPOSITORY_INITIALIZATION_CLAIM_PATH_V1,
        REPOSITORY_INITIALIZATION_RECEIPT_PATH_V1,
    ] {
        std::fs::remove_file(repo.path().join(path)).unwrap();
    }
    let (_, another_claim) = prepare(repo.path());
    let guard = RepositoryInitializationGuardV1::acquire(repo.path()).unwrap();
    assert!(
        guard.require_fresh().is_err(),
        "retained witness is reservation evidence even without .handbook"
    );
    assert!(guard.validate_ordinary_setup().is_err());
    assert!(guard.retain_native_claim(&another_claim).is_err());
    guard.release().unwrap();
    assert!(RepositoryInvocationIdentityServiceV1::new()
        .initialize_for_setup(repo.path())
        .is_err());
    assert!(!repo.path().join(".handbook").exists());
    assert!(!repo
        .path()
        .join(REPOSITORY_INITIALIZATION_CLAIM_PATH_V1)
        .exists());
    assert!(!repo
        .path()
        .join(REPOSITORY_INITIALIZATION_RECEIPT_PATH_V1)
        .exists());
    assert_eq!(
        std::fs::read(repo.path().join(NONCOMMIT)).unwrap(),
        b"opaque owner noncommit witness"
    );
    assert_eq!(std::fs::read_dir(repo.path()).unwrap().count(), 2);
}

#[test]
fn orphaned_native_noncommit_witness_refuses_missing_lock_without_bootstrap_effect() {
    let repo = tempfile::tempdir().unwrap();
    let (_, claim) = prepare(repo.path());
    let guard = RepositoryInitializationGuardV1::acquire(repo.path()).unwrap();
    guard.retain_native_claim(&claim).unwrap();
    guard
        .root()
        .write_new_durable(NONCOMMIT, b"opaque owner noncommit witness")
        .unwrap();
    guard
        .complete_native_claim(
            &claim,
            RepositoryInitializationDispositionV1::NotCommitted,
            NONCOMMIT,
            b"opaque owner noncommit witness",
        )
        .unwrap();
    guard.release().unwrap();
    for path in [
        REPOSITORY_INITIALIZATION_CLAIM_PATH_V1,
        REPOSITORY_INITIALIZATION_RECEIPT_PATH_V1,
        REPOSITORY_INITIALIZATION_LOCK_PATH_V1,
    ] {
        std::fs::remove_file(repo.path().join(path)).unwrap();
    }
    assert!(
        RepositoryInitializationGuardV1::acquire(repo.path()).is_err(),
        "witness-only history cannot bootstrap a replacement initialization lock"
    );
    assert!(RepositoryInvocationIdentityServiceV1::new()
        .initialize_for_setup(repo.path())
        .is_err());
    assert!(!repo.path().join(".handbook").exists());
    assert!(!repo
        .path()
        .join(REPOSITORY_INITIALIZATION_LOCK_PATH_V1)
        .exists());
    assert!(!repo
        .path()
        .join(REPOSITORY_INITIALIZATION_CLAIM_PATH_V1)
        .exists());
    assert!(!repo
        .path()
        .join(REPOSITORY_INITIALIZATION_RECEIPT_PATH_V1)
        .exists());
    assert_eq!(
        std::fs::read(repo.path().join(NONCOMMIT)).unwrap(),
        b"opaque owner noncommit witness"
    );
    assert_eq!(std::fs::read_dir(repo.path()).unwrap().count(), 1);
}

#[test]
fn a_retained_receipt_is_checked_before_recovery_can_add_other_closure() {
    for damaged in ["receipt", "terminal"] {
        let repo = tempfile::tempdir().unwrap();
        let (_, claim) = prepare(repo.path());
        let guard = RepositoryInitializationGuardV1::acquire(repo.path()).unwrap();
        guard.retain_native_claim(&claim).unwrap();
        guard
            .root()
            .write_new_durable(NONCOMMIT, b"noncommit")
            .unwrap();
        guard
            .complete_native_claim(
                &claim,
                RepositoryInitializationDispositionV1::NotCommitted,
                NONCOMMIT,
                b"noncommit",
            )
            .unwrap();
        guard.release().unwrap();
        let path = if damaged == "receipt" {
            REPOSITORY_INITIALIZATION_RECEIPT_PATH_V1
        } else {
            NONCOMMIT
        };
        std::fs::write(repo.path().join(path), b"damaged").unwrap();
        let guard = RepositoryInitializationGuardV1::acquire(repo.path()).unwrap();
        assert!(guard.observe_native_claim(&claim).is_err());
        assert!(guard
            .complete_native_claim(
                &claim,
                RepositoryInitializationDispositionV1::NotCommitted,
                NONCOMMIT,
                b"noncommit"
            )
            .is_err());
        assert_eq!(std::fs::read(repo.path().join(path)).unwrap(), b"damaged");
    }
}

#[test]
fn missing_or_substituted_initialization_lock_is_never_healed() {
    let repo = tempfile::tempdir().unwrap();
    let (_, claim) = prepare(repo.path());
    let guard = RepositoryInitializationGuardV1::acquire(repo.path()).unwrap();
    guard.retain_native_claim(&claim).unwrap();
    guard.release().unwrap();
    std::fs::remove_file(repo.path().join(REPOSITORY_INITIALIZATION_LOCK_PATH_V1)).unwrap();
    assert!(RepositoryInitializationGuardV1::acquire(repo.path()).is_err());
    assert!(!repo
        .path()
        .join(REPOSITORY_INITIALIZATION_LOCK_PATH_V1)
        .exists());

    let other = tempfile::tempdir().unwrap();
    let guard = RepositoryInitializationGuardV1::acquire(other.path()).unwrap();
    let lock = other.path().join(REPOSITORY_INITIALIZATION_LOCK_PATH_V1);
    std::fs::rename(&lock, other.path().join("removed-lock")).unwrap();
    std::fs::write(&lock, b"").unwrap();
    assert!(guard.validate().is_err());
}

#[test]
fn native_claim_cannot_be_copied_to_another_physical_root() {
    let repo = tempfile::tempdir().unwrap();
    let (_, claim) = prepare(repo.path());
    let guard = RepositoryInitializationGuardV1::acquire(repo.path()).unwrap();
    guard.retain_native_claim(&claim).unwrap();
    guard.release().unwrap();
    let other = tempfile::tempdir().unwrap();
    for path in [
        REPOSITORY_INITIALIZATION_CLAIM_PATH_V1,
        REPOSITORY_INITIALIZATION_LOCK_PATH_V1,
    ] {
        std::fs::copy(repo.path().join(path), other.path().join(path)).unwrap();
    }
    let guard = RepositoryInitializationGuardV1::acquire(other.path()).unwrap();
    assert!(guard.observe_native_claim(&claim).is_err());
    assert!(guard.validate_ordinary_setup().is_err());
}

#[test]
fn retained_native_claim_refuses_a_replacement_lock_after_restart() {
    let repo = tempfile::tempdir().unwrap();
    let (_, claim) = prepare(repo.path());
    let guard = RepositoryInitializationGuardV1::acquire(repo.path()).unwrap();
    guard.retain_native_claim(&claim).unwrap();
    guard.release().unwrap();
    let lock = repo.path().join(REPOSITORY_INITIALIZATION_LOCK_PATH_V1);
    std::fs::rename(&lock, repo.path().join("original-lock")).unwrap();
    std::fs::write(&lock, b"").unwrap();
    let guard = RepositoryInitializationGuardV1::acquire(repo.path()).unwrap();
    assert!(
        guard.observe_native_claim(&claim).is_err(),
        "a replaced lock cannot resume native history"
    );
}

#[test]
fn held_root_guard_refuses_changed_and_restored_empty_lock_generation() {
    let repo = tempfile::tempdir().unwrap();
    let guard = RepositoryInitializationGuardV1::acquire(repo.path()).unwrap();
    let lock_path = repo.path().join(REPOSITORY_INITIALIZATION_LOCK_PATH_V1);
    let original_modified = std::fs::metadata(&lock_path).unwrap().modified().unwrap();
    let lock = std::fs::OpenOptions::new()
        .write(true)
        .open(&lock_path)
        .unwrap();
    lock.set_modified(original_modified + Duration::from_secs(1))
        .unwrap();
    lock.set_modified(original_modified).unwrap();
    assert_eq!(std::fs::metadata(&lock_path).unwrap().len(), 0);
    assert!(
        guard.validate().is_err(),
        "same-file changed/restored empty lock must not regain the retained generation"
    );
}

#[test]
fn empty_root_lock_bootstrap_converges_under_simultaneous_creators() {
    let repo = std::sync::Arc::new(tempfile::tempdir().unwrap());
    let barrier = std::sync::Arc::new(std::sync::Barrier::new(8));
    let mut threads = Vec::new();
    for _ in 0..8 {
        let repo = std::sync::Arc::clone(&repo);
        let barrier = std::sync::Arc::clone(&barrier);
        threads.push(std::thread::spawn(move || {
            barrier.wait();
            let guard = RepositoryInitializationGuardV1::acquire(repo.path()).unwrap();
            guard.validate().unwrap();
            guard.release().unwrap();
        }));
    }
    for thread in threads {
        thread.join().unwrap();
    }
    assert_eq!(
        std::fs::read(repo.path().join(REPOSITORY_INITIALIZATION_LOCK_PATH_V1)).unwrap(),
        b""
    );
    assert!(!repo.path().join(".handbook").exists());
}

#[test]
fn direct_identity_initializer_waits_for_root_guard_then_refuses_paused_claim() {
    let repo = tempfile::tempdir().unwrap();
    let (_, claim) = prepare(repo.path());
    let guard = RepositoryInitializationGuardV1::acquire(repo.path()).unwrap();
    guard.retain_native_claim(&claim).unwrap();
    let (started_tx, started_rx) = mpsc::channel();
    let (done_tx, done_rx) = mpsc::channel();
    let path = repo.path().to_owned();
    let child = std::thread::spawn(move || {
        started_tx.send(()).unwrap();
        let result = RepositoryInvocationIdentityServiceV1::new().initialize_for_setup(path);
        done_tx.send(result.is_err()).unwrap();
    });
    started_rx.recv_timeout(Duration::from_secs(5)).unwrap();
    assert!(done_rx.recv_timeout(Duration::from_millis(75)).is_err());
    assert!(!repo.path().join(".handbook").exists());
    guard.release().unwrap();
    assert!(done_rx.recv_timeout(Duration::from_secs(5)).unwrap());
    child.join().unwrap();
    assert!(!repo.path().join(".handbook").exists());
}

#[test]
fn unsafe_native_claim_and_lock_entries_refuse_without_changing_external_bytes() {
    for path in [
        REPOSITORY_INITIALIZATION_CLAIM_PATH_V1,
        REPOSITORY_INITIALIZATION_LOCK_PATH_V1,
    ] {
        let repo = tempfile::tempdir().unwrap();
        let external = repo.path().join("external");
        std::fs::write(&external, b"").unwrap();
        std::fs::hard_link(&external, repo.path().join(path)).unwrap();
        let error = RepositoryInitializationGuardV1::acquire(repo.path()).unwrap_err();
        assert_eq!(
            error.kind(),
            RepositoryDurabilityErrorKindV1::UnsafeFilesystem
        );
        assert_eq!(std::fs::read(external).unwrap(), b"");
    }
}

#[test]
fn legacy_native_anchor_requires_original_identity_and_physical_binding() {
    for damage in [
        "missing_identity",
        "changed_identity",
        "changed_physical_root",
    ] {
        let repo = tempfile::tempdir().unwrap();
        let (identity, claim) = prepare(repo.path());
        let root = RetainedRepositoryRootV1::open_exact(repo.path()).unwrap();
        root.create_directory_new_durable(".handbook").unwrap();
        RepositoryInvocationIdentityServiceV1::new()
            .persist_prepared_repository_identity(&root, identity)
            .unwrap();
        let mut anchor = serde_json::json!({"project":{
            "retained_root":repo.path().to_str().unwrap(),
            "physical_root_fingerprint":claim.physical_root_fingerprint(),
            "repository_identity_fingerprint":claim.proposed_repository_identity()
        }});
        if damage == "changed_physical_root" {
            anchor["project"]["physical_root_fingerprint"] =
                serde_json::json!(format!("sha256:{}", "f".repeat(64)));
        }
        root.write_new_durable(
            ".handbook/admission-governance-project.v1.json",
            &serde_json::to_vec(&anchor).unwrap(),
        )
        .unwrap();
        if damage == "missing_identity" {
            std::fs::remove_file(repo.path().join(REPOSITORY_IDENTITY_REPO_PATH)).unwrap();
        }
        if damage == "changed_identity" {
            std::fs::write(
                repo.path().join(REPOSITORY_IDENTITY_REPO_PATH),
                format!("sha256:{}", "e".repeat(64)),
            )
            .unwrap();
        }
        assert!(RepositoryInvocationIdentityServiceV1::new()
            .initialize_for_setup(repo.path())
            .is_err());
        if damage == "missing_identity" {
            assert!(!repo.path().join(REPOSITORY_IDENTITY_REPO_PATH).exists());
        }
    }
}
