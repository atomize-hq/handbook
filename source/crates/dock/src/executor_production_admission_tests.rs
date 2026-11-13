use super::*;
use handbook_engine::repository_durability::admit_native_production_manifest_v1;

#[test]
fn exact_manifest_admission_rejects_foreign_and_altered_bytes() {
    let actual = crate::registry::production_manifest_bytes_v1();
    assert!(admit_native_production_manifest_v1(actual).is_ok());
    let known = [
        include_bytes!("../production-manifests/x86_64-unknown-linux-gnu/manifest.json").as_slice(),
        include_bytes!("../production-manifests/x86_64-pc-windows-msvc/manifest.json").as_slice(),
        include_bytes!("../production-manifests/aarch64-apple-darwin/manifest.json").as_slice(),
    ];
    let mut foreign_count = 0;
    for manifest in known {
        let manifest = manifest.strip_suffix(b"\n").unwrap();
        if manifest != actual {
            foreign_count += 1;
            assert!(admit_native_production_manifest_v1(manifest).is_err());
        }
    }
    assert_eq!(foreign_count, 2);
    let mut changed = actual.to_vec();
    changed[0] ^= 1;
    assert!(admit_native_production_manifest_v1(&changed).is_err());

    let base: serde_json::Value = serde_json::from_slice(actual).unwrap();
    let mut foreign_target = base.clone();
    foreign_target["implementation_bundle"]["target"] = serde_json::json!("foreign-target");
    let mut foreign_path = base.clone();
    foreign_path["implementation_bundle"]["files"][0]["path"] = serde_json::json!("../foreign");
    let mut oversized_member = base.clone();
    oversized_member["implementation_bundle"]["files"][0]["length"] = serde_json::json!(64_u64 * 1024 * 1024 + 1);
    let mut aggregate = base.clone();
    let mut many = Vec::new();
    for index in 0..5 {
        let mut entry = base["implementation_bundle"]["files"][0].clone();
        entry["path"] = serde_json::json!(format!("bin/member-{index}"));
        entry["length"] = serde_json::json!(if index == 4 { 1_u64 } else { 64_u64 * 1024 * 1024 });
        many.push(entry);
    }
    aggregate["implementation_bundle"]["files"] = serde_json::json!(many);
    let mut excessive_count = base.clone();
    let mut files = Vec::new();
    for index in 0..257 {
        let mut entry = base["implementation_bundle"]["files"][0].clone();
        entry["path"] = serde_json::json!(format!("bin/member-{index:03}"));
        files.push(entry);
    }
    excessive_count["implementation_bundle"]["files"] = serde_json::json!(files);
    // Each uncommitted document must fail the exact-byte gate before parse;
    // this does not claim independent parser-boundary coverage.
    for changed in [foreign_target, foreign_path, oversized_member, aggregate, excessive_count] {
        assert!(admit_native_production_manifest_v1(&serde_json::to_vec(&changed).unwrap()).is_err());
    }
}

#[test]
fn sealed_member_admission_refuses_foreign_paths() {
    let admission = admit_native_production_manifest_v1(crate::registry::production_manifest_bytes_v1()).unwrap();
    let manifest = crate::registry::production_manifest_v1();
    let bundle = manifest.implementation_bundle();
    let fingerprint = bundle.bundle_fingerprint().as_str().strip_prefix("sha256:").unwrap();
    let actual = format!(".handbook/state/dock/local-evaluation-v1/packages/{fingerprint}/{}", bundle.files()[0].path().as_str());
    let root_dir = tempfile::tempdir().unwrap();
    let root = handbook_engine::RetainedRepositoryRootV1::open_exact(root_dir.path()).unwrap();
    for foreign in [format!("{actual}.other"), "unrelated.bin".to_owned()] {
        assert!(root.read_native_manifest_member_v1(&admission, &foreign).is_err());
        assert!(root.write_native_manifest_member_new_durable_v1(&admission, &foreign, b"untrusted").is_err());
    }
    assert_eq!(fs::read_dir(root_dir.path()).unwrap().count(), 0);
}
