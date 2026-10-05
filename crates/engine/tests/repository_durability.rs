use handbook_engine::{
    RepositoryDirectoryEntryKindV1, RepositoryDurabilityErrorKindV1, RepositoryReadV1,
    RetainedRepositoryRootV1,
};

#[test]
fn retained_root_performs_bounded_create_read_list_rename_and_remove() {
    let repo = tempfile::tempdir().unwrap();
    let root = RetainedRepositoryRootV1::open_exact(repo.path()).unwrap();

    root.create_directories(".handbook/state/contracts")
        .unwrap();
    root.create_directory_new_durable(".handbook/state/contracts/staged")
        .unwrap();
    root.create_directory_new_durable(".handbook/state/contracts/transaction.pending")
        .unwrap();
    root.rename_directory_noreplace_durable(
        ".handbook/state/contracts/transaction.pending",
        ".handbook/state/contracts/transaction.committed",
    )
    .unwrap();
    root.write_new_durable(".handbook/state/contracts/staged/b.json", b"bravo")
        .unwrap();
    root.write_new_durable(".handbook/state/contracts/staged/a.json", b"alpha")
        .unwrap();

    assert_eq!(
        root.read_regular_bounded(".handbook/state/contracts/staged/a.json", 5)
            .unwrap(),
        RepositoryReadV1::Found(b"alpha".to_vec())
    );
    assert_eq!(
        root.read_regular_bounded(".handbook/state/contracts/staged/absent.json", 5)
            .unwrap(),
        RepositoryReadV1::Absent
    );
    assert_eq!(
        root.read_regular_bounded(".handbook/state/contracts/staged/a.json", 4)
            .unwrap_err()
            .kind(),
        RepositoryDurabilityErrorKindV1::BoundExceeded
    );

    let listed = root
        .list_directory_bounded(".handbook/state/contracts/staged", 2)
        .unwrap();
    assert_eq!(
        listed
            .iter()
            .map(|entry| (entry.name(), entry.kind()))
            .collect::<Vec<_>>(),
        vec![
            ("a.json", RepositoryDirectoryEntryKindV1::RegularFile),
            ("b.json", RepositoryDirectoryEntryKindV1::RegularFile),
        ]
    );

    root.rename_noreplace_durable(
        ".handbook/state/contracts/staged/a.json",
        ".handbook/state/contracts/a.json",
    )
    .unwrap();
    assert_eq!(
        root.rename_noreplace_durable(
            ".handbook/state/contracts/staged/b.json",
            ".handbook/state/contracts/a.json",
        )
        .unwrap_err()
        .kind(),
        RepositoryDurabilityErrorKindV1::AlreadyExists
    );
    root.remove_regular_durable(".handbook/state/contracts/a.json")
        .unwrap();
    root.remove_regular_durable(".handbook/state/contracts/staged/b.json")
        .unwrap();
    root.remove_empty_directory_durable(".handbook/state/contracts/staged")
        .unwrap();
    root.remove_empty_directory_durable(".handbook/state/contracts/transaction.committed")
        .unwrap();
}

#[test]
fn retained_root_refuses_escaping_absolute_and_link_paths() {
    let repo = tempfile::tempdir().unwrap();
    let root = RetainedRepositoryRootV1::open_exact(repo.path()).unwrap();
    for path in ["../escape", "/absolute", "a//b", "a\\b", "./a"] {
        assert_eq!(
            root.read_regular_bounded(path, 1).unwrap_err().kind(),
            RepositoryDurabilityErrorKindV1::InvalidRelativePath,
            "{path}"
        );
    }

    root.create_directories("safe").unwrap();
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(repo.path().join("safe"), repo.path().join("linked")).unwrap();
        assert_eq!(
            root.list_directory_bounded("linked", 1).unwrap_err().kind(),
            RepositoryDurabilityErrorKindV1::UnsafeFilesystem
        );
    }
}

#[test]
fn retained_root_creates_and_holds_an_exclusive_lock() {
    let repo = tempfile::tempdir().unwrap();
    let root = RetainedRepositoryRootV1::open_exact(repo.path()).unwrap();
    root.create_directories(".handbook/state/locks").unwrap();
    let lock = root
        .acquire_exclusive_lock(".handbook/state/locks/contracts.lock")
        .unwrap();
    lock.release().unwrap();
    assert_eq!(
        root.read_regular_bounded(".handbook/state/locks/contracts.lock", 0)
            .unwrap(),
        RepositoryReadV1::Found(Vec::new())
    );
}

#[test]
fn exact_replace_requires_current_bytes_and_leaves_recoverable_displaced_state() {
    let repo = tempfile::tempdir().unwrap();
    let root = RetainedRepositoryRootV1::open_exact(repo.path()).unwrap();
    root.create_directories("state").unwrap();
    root.write_new_durable("state/head", b"old").unwrap();
    root.write_new_durable("state/prepared", b"new").unwrap();

    assert_eq!(
        root.replace_exact_durable("state/head", b"wrong", "state/prepared", "state/displaced")
            .unwrap_err()
            .kind(),
        RepositoryDurabilityErrorKindV1::CurrentValueMismatch
    );
    root.replace_exact_durable("state/head", b"old", "state/prepared", "state/displaced")
        .unwrap();
    assert_eq!(
        root.read_regular_bounded("state/head", 3).unwrap(),
        RepositoryReadV1::Found(b"new".to_vec())
    );
    assert_eq!(
        root.read_regular_bounded("state/displaced", 3).unwrap(),
        RepositoryReadV1::Found(b"old".to_vec())
    );
}

#[test]
fn exact_displacement_retains_the_actual_gap_until_prepared_publication() {
    let repo = tempfile::tempdir().unwrap();
    let root = RetainedRepositoryRootV1::open_exact(repo.path()).unwrap();
    root.create_directories("state").unwrap();
    root.write_new_durable("state/head", b"old").unwrap();
    root.write_new_durable("state/prepared", b"new").unwrap();

    assert_eq!(
        root.displace_exact_durable("state/head", b"wrong", "state/displaced")
            .unwrap_err()
            .kind(),
        RepositoryDurabilityErrorKindV1::CurrentValueMismatch
    );
    assert_eq!(
        root.read_regular_bounded("state/head", 3).unwrap(),
        RepositoryReadV1::Found(b"old".to_vec())
    );
    assert_eq!(
        root.read_regular_bounded("state/displaced", 3).unwrap(),
        RepositoryReadV1::Absent
    );

    root.displace_exact_durable("state/head", b"old", "state/displaced")
        .unwrap();
    let reopened = RetainedRepositoryRootV1::open_exact(repo.path()).unwrap();
    assert_eq!(
        reopened.read_regular_bounded("state/head", 3).unwrap(),
        RepositoryReadV1::Absent
    );
    assert_eq!(
        reopened.read_regular_bounded("state/prepared", 3).unwrap(),
        RepositoryReadV1::Found(b"new".to_vec())
    );
    assert_eq!(
        reopened.read_regular_bounded("state/displaced", 3).unwrap(),
        RepositoryReadV1::Found(b"old".to_vec())
    );
    reopened
        .rename_noreplace_durable("state/prepared", "state/head")
        .unwrap();
    assert_eq!(
        reopened.read_regular_bounded("state/head", 3).unwrap(),
        RepositoryReadV1::Found(b"new".to_vec())
    );
    assert_eq!(
        reopened.read_regular_bounded("state/prepared", 3).unwrap(),
        RepositoryReadV1::Absent
    );
    assert_eq!(
        reopened.read_regular_bounded("state/displaced", 3).unwrap(),
        RepositoryReadV1::Found(b"old".to_vec())
    );
}

#[test]
fn exact_displacement_and_replacement_refuse_aliases_and_hard_linked_current() {
    let repo = tempfile::tempdir().unwrap();
    let root = RetainedRepositoryRootV1::open_exact(repo.path()).unwrap();
    root.create_directories("state").unwrap();
    root.write_new_durable("state/head", b"old").unwrap();
    root.write_new_durable("state/prepared", b"new").unwrap();

    assert_eq!(
        root.displace_exact_durable("state/head", b"old", "state/head")
            .unwrap_err()
            .kind(),
        RepositoryDurabilityErrorKindV1::InvalidRelativePath
    );
    for (prepared, displaced) in [
        ("state/head", "state/displaced"),
        ("state/prepared", "state/head"),
        ("state/prepared", "state/prepared"),
    ] {
        assert_eq!(
            root.replace_exact_durable("state/head", b"old", prepared, displaced)
                .unwrap_err()
                .kind(),
            RepositoryDurabilityErrorKindV1::InvalidRelativePath
        );
    }
    assert_eq!(
        root.read_regular_bounded("state/head", 3).unwrap(),
        RepositoryReadV1::Found(b"old".to_vec())
    );
    std::fs::hard_link(
        repo.path().join("state/head"),
        repo.path().join("state/linked-head"),
    )
    .unwrap();
    assert_eq!(
        root.displace_exact_durable("state/head", b"old", "state/displaced")
            .unwrap_err()
            .kind(),
        RepositoryDurabilityErrorKindV1::UnsafeFilesystem
    );
    assert_eq!(
        std::fs::read(repo.path().join("state/head")).unwrap(),
        b"old"
    );
    assert!(!repo.path().join("state/displaced").exists());
    assert_eq!(
        root.read_regular_bounded("state/prepared", 3).unwrap(),
        RepositoryReadV1::Found(b"new".to_vec())
    );
}
