use handbook_engine::RetainedRepositoryRootV1;
use tempfile::tempdir;

#[test]
fn read_only_lock_acquisition_never_recreates_lost_state() {
    let dir = tempdir().unwrap();
    let root = RetainedRepositoryRootV1::open_exact(dir.path()).unwrap();
    assert!(root
        .acquire_existing_exclusive_lock_v1("lost.lock")
        .is_err());
    assert!(!dir.path().join("lost.lock").exists());
    std::fs::write(dir.path().join("existing.lock"), []).unwrap();
    let lock = root
        .acquire_existing_exclusive_lock_v1("existing.lock")
        .unwrap();
    lock.validate_at(&root, "existing.lock").unwrap();
}

#[test]
fn exact_source_checkpoint_pins_content_and_refuses_absent_sources() {
    let dir = tempdir().unwrap();
    std::fs::write(dir.path().join("source.json"), b"captured input").unwrap();
    let root = RetainedRepositoryRootV1::open_exact(dir.path()).unwrap();
    assert!(root
        .retain_exact_files_checkpoint_v1(&["absent.json"])
        .is_err());
    let checkpoint = root
        .retain_exact_files_checkpoint_v1(&["source.json"])
        .unwrap();
    checkpoint.validate().unwrap();
    assert_eq!(
        checkpoint.pinned_file_paths().collect::<Vec<_>>(),
        vec!["source.json"]
    );
    checkpoint
        .require_file_membership(&root, &["source.json"])
        .unwrap();
    assert!(checkpoint
        .require_file_membership(&root, &["absent.json"])
        .is_err());
    assert!(checkpoint
        .require_file_membership(&root, &["source.json", "source.json"])
        .is_err());
    assert!(checkpoint.require_file_membership(&root, &[]).is_err());
    checkpoint
        .require_exact_file(&root, "source.json", b"captured input")
        .unwrap();
    assert!(checkpoint
        .require_exact_file(&root, "source.json", b"invented input")
        .is_err());
    assert!(checkpoint
        .require_exact_file(&root, "absent.json", b"captured input")
        .is_err());
    let foreign = tempdir().unwrap();
    std::fs::write(foreign.path().join("source.json"), b"captured input").unwrap();
    let foreign_root = RetainedRepositoryRootV1::open_exact(foreign.path()).unwrap();
    assert!(checkpoint
        .require_file_membership(&foreign_root, &["source.json"])
        .is_err());
    assert!(checkpoint
        .require_exact_file(&foreign_root, "source.json", b"captured input")
        .is_err());
    let changed = std::fs::write(dir.path().join("source.json"), b"different input");
    if changed.is_ok() {
        assert!(checkpoint.validate().is_err());
    } else {
        // Windows native retained read handles deny write while consumption runs.
        checkpoint.validate().unwrap();
    }
}
