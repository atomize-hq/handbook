use handbook_engine::{CharterAuthorityTransactionServiceV1, CharterPromotionErrorKindV1};

#[test]
fn external_observer_rejects_loose_orphan_lineage_as_canonical_truth() {
    let repo = tempfile::tempdir().expect("repository");
    let state = repo.path().join(".handbook/state");
    std::fs::create_dir_all(state.join("promotions")).expect("orphan promotion directory");
    std::fs::create_dir_all(state.join("lifecycle-transitions"))
        .expect("orphan lifecycle directory");
    std::fs::write(state.join("promotions/orphan.json"), b"{}")
        .expect("loose orphan promotion record");
    std::fs::write(state.join("lifecycle-transitions/orphan.json"), b"{}")
        .expect("loose orphan lifecycle record");

    let observer = CharterAuthorityTransactionServiceV1::new(repo.path());
    assert!(observer
        .read_committed_charter()
        .expect("loose records are not authority")
        .is_none());
    assert!(!repo.path().join(".handbook/project/charter.yaml").exists());

    std::fs::create_dir_all(repo.path().join(".handbook/project")).expect("canonical parent");
    std::fs::write(
        repo.path().join(".handbook/project/charter.yaml"),
        b"forged canonical bytes",
    )
    .expect("forged canonical file");
    let error = observer
        .read_committed_charter()
        .expect_err("canonical bytes without a committed workflow journal must refuse");
    assert_eq!(
        error.kind(),
        CharterPromotionErrorKindV1::DurabilityViolation
    );
}
