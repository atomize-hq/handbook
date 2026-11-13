//! Linux focused source-observation tests; synthetic members do not stand in
//! for the real validator or constitute native qualification.
use super::*;
use std::os::unix::fs::{symlink, PermissionsExt};

struct NativeSourceFixture {
    _temporary: tempfile::TempDir,
    bundle: PathBuf,
    root: RepositoryNativeSourceRootV1,
    admission: RepositoryNativeManifestAdmissionV1,
}

fn synthetic_inventory_admission_v1(relative: &str, bytes: &[u8]) -> RepositoryNativeManifestAdmissionV1 {
    RepositoryNativeManifestAdmissionV1 { members: vec![RepositoryNativeManifestMemberV1 {
        path: format!(".handbook/state/dock/local-evaluation-v1/packages/{}/{relative}", "0".repeat(64)),
        length: bytes.len() as u64,
        sha256: Sha256::digest(bytes).into(),
    }] }
}

fn source_fixture_v1() -> NativeSourceFixture {
    let temporary = tempfile::Builder::new().prefix("task19-source-inventory-").tempdir().unwrap();
    let bundle = temporary.path().join("bundle");
    fs::create_dir_all(bundle.join("bin")).unwrap();
    fs::write(bundle.join("bin/validator"), b"test-source-member").unwrap();
    let root = open_production_source_root_v1(&bundle).unwrap();
    let admission = synthetic_inventory_admission_v1("bin/validator", b"test-source-member");
    NativeSourceFixture { _temporary: temporary, bundle, root, admission }
}

struct RestoreDirectoryPermissionsV1(PathBuf, fs::Permissions);
impl Drop for RestoreDirectoryPermissionsV1 {
    fn drop(&mut self) { let _ = fs::set_permissions(&self.0, self.1.clone()); }
}

#[test]
fn source_missing_unobservable_and_changes_are_classified() {
    let f = source_fixture_v1();
    let path = f.bundle.join("bin/validator");
    fs::remove_file(&path).unwrap();
    assert!(matches!(f.root.read_production_source_member_v1(&f.admission, "bin/validator"),
        Err(RepositoryNativeSourceFailureV1::Missing)));
    fs::write(&path, b"test-source-member").unwrap();
    let (_, file) = f.root.read_production_source_member_v1(&f.admission, "bin/validator").unwrap();
    fs::remove_file(&path).unwrap();
    assert!(matches!(file.validate_production_source_file_v1(), Err(RepositoryNativeSourceFailureV1::Unsafe)));

    let f = source_fixture_v1();
    let directory = f.bundle.join("bin");
    let _restore = RestoreDirectoryPermissionsV1(directory.clone(), fs::metadata(&directory).unwrap().permissions());
    fs::set_permissions(&directory, fs::Permissions::from_mode(0)).unwrap();
    assert_eq!(fs::read(directory.join("validator")).unwrap_err().kind(), std::io::ErrorKind::PermissionDenied,
        "this test requires an actually unprivileged access denial");
    assert!(matches!(f.root.read_production_source_member_v1(&f.admission, "bin/validator"),
        Err(RepositoryNativeSourceFailureV1::Unavailable)));
}

#[test]
fn source_rejects_links_and_bad_contents() {
    let f = source_fixture_v1();
    let path = f.bundle.join("bin/validator");
    let target = f._temporary.path().join("outside");
    fs::write(&target, b"test-source-member").unwrap();
    fs::remove_file(&path).unwrap();
    symlink(&target, &path).unwrap();
    assert!(matches!(f.root.read_production_source_member_v1(&f.admission, "bin/validator"),
        Err(RepositoryNativeSourceFailureV1::Unsafe)));
    fs::remove_file(&path).unwrap();
    fs::hard_link(&target, &path).unwrap();
    assert!(matches!(f.root.read_production_source_member_v1(&f.admission, "bin/validator"),
        Err(RepositoryNativeSourceFailureV1::Unsafe)));
    fs::remove_file(&path).unwrap();
    fs::write(&path, b"wrong-size").unwrap();
    assert!(matches!(f.root.read_production_source_member_v1(&f.admission, "bin/validator"),
        Err(RepositoryNativeSourceFailureV1::Unsafe)));
    fs::write(&path, b"test-source-membes").unwrap();
    assert!(matches!(f.root.read_production_source_member_v1(&f.admission, "bin/validator"),
        Err(RepositoryNativeSourceFailureV1::Unsafe)));
}

#[test]
fn inventory_refuses_missing_extra_and_type_conflicts() {
    let f = source_fixture_v1();
    let guard = f.root.retain_production_source_inventory_v1(&f.admission).unwrap();
    guard.validate_production_source_inventory_v1().unwrap();
    drop(guard);
    fs::remove_file(f.bundle.join("bin/validator")).unwrap();
    assert!(f.root.retain_production_source_inventory_v1(&f.admission).is_err());
    fs::remove_dir(f.bundle.join("bin")).unwrap();
    assert!(f.root.retain_production_source_inventory_v1(&f.admission).is_err());

    let f = source_fixture_v1();
    fs::write(f.bundle.join("bin/extra"), b"unexpected").unwrap();
    assert!(f.root.retain_production_source_inventory_v1(&f.admission).is_err());
    fs::remove_file(f.bundle.join("bin/extra")).unwrap();
    fs::create_dir(f.bundle.join("extra-dir")).unwrap();
    assert!(f.root.retain_production_source_inventory_v1(&f.admission).is_err());

    let f = source_fixture_v1();
    let member = f.bundle.join("bin/validator");
    let outside = f._temporary.path().join("outside");
    fs::write(&outside, b"test-source-member").unwrap();
    fs::remove_file(&member).unwrap();
    symlink(&outside, &member).unwrap();
    assert!(f.root.retain_production_source_inventory_v1(&f.admission).is_err());
    fs::remove_file(&member).unwrap();
    fs::hard_link(&outside, &member).unwrap();
    assert!(f.root.retain_production_source_inventory_v1(&f.admission).is_err());

    let f = source_fixture_v1();
    fs::remove_file(f.bundle.join("bin/validator")).unwrap();
    fs::create_dir(f.bundle.join("bin/validator")).unwrap();
    assert!(f.root.retain_production_source_inventory_v1(&f.admission).is_err());
    let mut conflict = synthetic_inventory_admission_v1("bin", b"x");
    conflict.members.extend(synthetic_inventory_admission_v1("bin/validator", b"x").members);
    assert!(f.root.retain_production_source_inventory_v1(&conflict).is_err());
}

#[test]
fn inventory_guard_detects_later_changes_and_root_replacement() {
    let f = source_fixture_v1();
    let inventory = f.root.retain_production_source_inventory_v1(&f.admission).unwrap();
    let (_, file) = f.root.read_production_source_member_v1(&f.admission, "bin/validator").unwrap();
    fs::write(f.bundle.join("bin/new-child"), b"changed").unwrap();
    assert!(inventory.validate_production_source_inventory_v1().is_err());
    assert!(matches!(file.validate_production_source_file_v1(), Err(RepositoryNativeSourceFailureV1::Unsafe)));

    let f = source_fixture_v1();
    let (_, file) = f.root.read_production_source_member_v1(&f.admission, "bin/validator").unwrap();
    fs::rename(f.bundle.join("bin"), f.bundle.join("old-bin")).unwrap();
    fs::create_dir(f.bundle.join("bin")).unwrap();
    fs::write(f.bundle.join("bin/validator"), b"test-source-member").unwrap();
    assert!(matches!(file.validate_production_source_file_v1(), Err(RepositoryNativeSourceFailureV1::Unsafe)));

    let f = source_fixture_v1();
    let inventory = f.root.retain_production_source_inventory_v1(&f.admission).unwrap();
    fs::rename(&f.bundle, f._temporary.path().join("old-bundle")).unwrap();
    fs::create_dir_all(f.bundle.join("bin")).unwrap();
    fs::write(f.bundle.join("bin/validator"), b"test-source-member").unwrap();
    assert!(inventory.validate_production_source_inventory_v1().is_err());
}

#[test]
fn ordinary_error_identity_and_size_limit_are_preserved() {
    let plain = unsafe_fs("stable detail");
    let tagged = with_native_source_evidence_v1(unsafe_fs("stable detail"), NativeSourceObservationEvidenceV1::Unobservable);
    assert_eq!(plain, tagged);
    assert_eq!(format!("{plain:?}"), format!("{tagged:?}"));
    assert_eq!(plain.kind(), tagged.kind());
    assert_eq!(plain.detail(), tagged.detail());
    assert_eq!(plain.to_string(), tagged.to_string());
    assert_eq!(classify_native_source_error_v1(tagged, false), RepositoryNativeSourceFailureV1::Unavailable);
    let missing = with_native_source_evidence_v1(not_found("missing"), NativeSourceObservationEvidenceV1::Missing);
    assert_eq!(classify_native_source_error_v1(missing.clone(), false), RepositoryNativeSourceFailureV1::Missing);
    assert_eq!(classify_native_source_error_v1(missing, true), RepositoryNativeSourceFailureV1::Unsafe);

    let temporary = tempfile::tempdir().unwrap();
    let file = File::create(temporary.path().join("metadata")).unwrap();
    file.set_len(MAX_STRICT_READ_BYTES as u64 + 1).unwrap();
    let root = RetainedRepositoryRootV1::open_exact(temporary.path()).unwrap();
    assert_eq!(root.read_regular_bounded("metadata", MAX_STRICT_READ_BYTES).unwrap_err().kind(),
        RepositoryDurabilityErrorKindV1::BoundExceeded);
    assert_eq!(root.read_regular_bounded("metadata", MAX_STRICT_READ_BYTES + 1).unwrap_err().kind(),
        RepositoryDurabilityErrorKindV1::BoundExceeded);
}

#[test]
fn checkpoint_aggregate_counts_nonbundle_bytes() {
    let temporary = tempfile::Builder::new().prefix("task19-aggregate-bound-").tempdir().unwrap();
    fs::write(temporary.path().join("0-ordinary"), [1_u8]).unwrap();
    let zeroes = [0_u8; 64 * 1024];
    let mut hasher = Sha256::new();
    for _ in 0..(MAX_NATIVE_MEMBER_BYTES as usize / zeroes.len()) { hasher.update(zeroes); }
    let digest: [u8; 32] = hasher.finalize().into();
    let mut members = Vec::new();
    for index in 0..4 {
        let path = format!("native-{index}");
        File::create(temporary.path().join(&path)).unwrap().set_len(MAX_NATIVE_MEMBER_BYTES).unwrap();
        members.push(RepositoryNativeManifestMemberV1 { path, length: MAX_NATIVE_MEMBER_BYTES, sha256: digest });
    }
    let admission = RepositoryNativeManifestAdmissionV1 { members };
    let root = RetainedRepositoryRootV1::open_exact(temporary.path()).unwrap();
    let paths = ["0-ordinary", "native-0", "native-1", "native-2", "native-3"];
    let error = match root.retain_exact_files_with_native_manifest_v1(&paths, &admission) {
        Ok(_) => panic!("ordinary byte must count against the aggregate"),
        Err(error) => error,
    };
    assert_eq!(error.kind(), RepositoryDurabilityErrorKindV1::BoundExceeded);
    assert_eq!(error.detail(), "source file population exceeds the retained byte bound");
}
