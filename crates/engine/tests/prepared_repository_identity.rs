use handbook_engine::{
    RepositoryDurabilityErrorKindV1, RepositoryIdentityInitializationDispositionV1,
    RepositoryInvocationIdentityServiceV1, RepositoryReadV1, RetainedRepositoryRootV1,
    REPOSITORY_IDENTITY_REPO_PATH,
};

const TEMP_IDENTITY: &str = ".handbook/.repository-identity.v1.tmp";

#[test]
fn preparation_has_no_repository_effect_and_persists_the_exact_proposed_identity() {
    let repo = tempfile::tempdir().unwrap();
    let service = RepositoryInvocationIdentityServiceV1::new();
    let prepared = service.allocate_prepared_repository_identity().unwrap();
    let expected = prepared.repository_identity_fingerprint().to_owned();
    assert_eq!(expected.len(), 71);
    assert!(std::fs::read_dir(repo.path()).unwrap().next().is_none());

    let root = RetainedRepositoryRootV1::open_exact(repo.path()).unwrap();
    root.create_directory_new_durable(".handbook").unwrap();
    let persisted = service
        .persist_prepared_repository_identity(&root, prepared)
        .unwrap();
    assert_eq!(
        persisted.disposition(),
        RepositoryIdentityInitializationDispositionV1::Created
    );
    assert_eq!(persisted.repository_identity_fingerprint(), expected);
    assert_eq!(
        root.read_regular_bounded(REPOSITORY_IDENTITY_REPO_PATH, 71)
            .unwrap(),
        RepositoryReadV1::Found(expected.as_bytes().to_vec())
    );
    assert_eq!(
        root.read_regular_bounded(TEMP_IDENTITY, 71).unwrap(),
        RepositoryReadV1::Absent
    );
    drop(root);

    let reopened = RetainedRepositoryRootV1::open_exact(repo.path()).unwrap();
    assert_eq!(
        reopened
            .read_regular_bounded(REPOSITORY_IDENTITY_REPO_PATH, 71)
            .unwrap(),
        RepositoryReadV1::Found(expected.as_bytes().to_vec())
    );
    let ordinary_setup = service.initialize_for_setup(repo.path()).unwrap();
    assert_eq!(
        ordinary_setup.disposition(),
        RepositoryIdentityInitializationDispositionV1::Preserved
    );
    assert_eq!(ordinary_setup.repository_identity_fingerprint(), expected);
}

#[test]
fn prepared_persistence_refuses_missing_reservation_without_creating_it() {
    let repo = tempfile::tempdir().unwrap();
    let root = RetainedRepositoryRootV1::open_exact(repo.path()).unwrap();
    let service = RepositoryInvocationIdentityServiceV1::new();
    let prepared = service.allocate_prepared_repository_identity().unwrap();
    assert_eq!(
        service
            .persist_prepared_repository_identity(&root, prepared)
            .unwrap_err()
            .kind(),
        RepositoryDurabilityErrorKindV1::UnsafeFilesystem
    );
    assert!(std::fs::read_dir(repo.path()).unwrap().next().is_none());
}

#[test]
fn prepared_persistence_refuses_existing_identity_or_interrupted_temp_exactly() {
    for occupied in [REPOSITORY_IDENTITY_REPO_PATH, TEMP_IDENTITY] {
        let repo = tempfile::tempdir().unwrap();
        let root = RetainedRepositoryRootV1::open_exact(repo.path()).unwrap();
        root.create_directory_new_durable(".handbook").unwrap();
        let service = RepositoryInvocationIdentityServiceV1::new();
        let first = service.allocate_prepared_repository_identity().unwrap();
        let old_bytes = first.repository_identity_fingerprint().as_bytes().to_vec();
        root.write_new_durable(occupied, &old_bytes).unwrap();
        let replacement = service.allocate_prepared_repository_identity().unwrap();
        assert_eq!(
            service
                .persist_prepared_repository_identity(&root, replacement)
                .unwrap_err()
                .kind(),
            RepositoryDurabilityErrorKindV1::AlreadyExists
        );
        assert_eq!(
            root.read_regular_bounded(occupied, 71).unwrap(),
            RepositoryReadV1::Found(old_bytes)
        );
        let other = if occupied == TEMP_IDENTITY {
            REPOSITORY_IDENTITY_REPO_PATH
        } else {
            TEMP_IDENTITY
        };
        assert_eq!(
            root.read_regular_bounded(other, 71).unwrap(),
            RepositoryReadV1::Absent
        );
    }
}

#[test]
fn prepared_persistence_refuses_hardlinked_identity_without_touching_either_link() {
    let repo = tempfile::tempdir().unwrap();
    let root = RetainedRepositoryRootV1::open_exact(repo.path()).unwrap();
    root.create_directory_new_durable(".handbook").unwrap();
    std::fs::write(repo.path().join("external"), b"retained").unwrap();
    std::fs::hard_link(
        repo.path().join("external"),
        repo.path().join(REPOSITORY_IDENTITY_REPO_PATH),
    )
    .unwrap();
    let service = RepositoryInvocationIdentityServiceV1::new();
    let prepared = service.allocate_prepared_repository_identity().unwrap();
    assert_eq!(
        service
            .persist_prepared_repository_identity(&root, prepared)
            .unwrap_err()
            .kind(),
        RepositoryDurabilityErrorKindV1::UnsafeFilesystem
    );
    assert_eq!(
        std::fs::read(repo.path().join("external")).unwrap(),
        b"retained"
    );
    assert!(!repo.path().join(TEMP_IDENTITY).exists());
}

#[test]
fn governance_physical_binding_survives_reopen_but_refuses_a_copied_root() {
    let original = tempfile::tempdir().unwrap();
    let copied = tempfile::tempdir().unwrap();
    let root = RetainedRepositoryRootV1::open_exact(original.path()).unwrap();
    let expected = root.governance_physical_root_fingerprint_v1().unwrap();
    root.create_directory_new_durable(".handbook").unwrap();
    root.write_new_durable(".handbook/anchor", b"identical anchor")
        .unwrap();
    assert_eq!(
        root.governance_physical_root_fingerprint_v1().unwrap(),
        expected
    );
    drop(root);
    let reopened = RetainedRepositoryRootV1::open_exact(original.path()).unwrap();
    assert_eq!(
        reopened.governance_physical_root_fingerprint_v1().unwrap(),
        expected
    );

    let copied_root = RetainedRepositoryRootV1::open_exact(copied.path()).unwrap();
    copied_root
        .create_directory_new_durable(".handbook")
        .unwrap();
    copied_root
        .write_new_durable(".handbook/anchor", b"identical anchor")
        .unwrap();
    assert_ne!(
        copied_root
            .governance_physical_root_fingerprint_v1()
            .unwrap(),
        expected
    );
}

#[cfg(unix)]
#[test]
fn retained_governance_binding_and_prepared_persistence_refuse_root_substitution() {
    let base = tempfile::tempdir().unwrap();
    let original = base.path().join("original");
    std::fs::create_dir(&original).unwrap();
    let root = RetainedRepositoryRootV1::open_exact(&original).unwrap();
    root.create_directory_new_durable(".handbook").unwrap();
    let before = root.governance_physical_root_fingerprint_v1().unwrap();
    std::fs::rename(&original, base.path().join("displaced")).unwrap();
    std::fs::create_dir(&original).unwrap();
    std::fs::create_dir(original.join(".handbook")).unwrap();
    assert_eq!(
        root.governance_physical_root_fingerprint_v1()
            .unwrap_err()
            .kind(),
        RepositoryDurabilityErrorKindV1::UnsafeFilesystem
    );
    let service = RepositoryInvocationIdentityServiceV1::new();
    let prepared = service.allocate_prepared_repository_identity().unwrap();
    assert_eq!(
        service
            .persist_prepared_repository_identity(&root, prepared)
            .unwrap_err()
            .kind(),
        RepositoryDurabilityErrorKindV1::UnsafeFilesystem
    );
    assert!(!original.join(REPOSITORY_IDENTITY_REPO_PATH).exists());
    drop(root);
    let replacement = RetainedRepositoryRootV1::open_exact(&original).unwrap();
    assert_ne!(
        replacement
            .governance_physical_root_fingerprint_v1()
            .unwrap(),
        before
    );
}
