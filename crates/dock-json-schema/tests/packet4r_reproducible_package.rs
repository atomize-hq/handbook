#[path = "../package-support/production.rs"]
mod production;
#[path = "../package-support/staging.rs"]
#[allow(dead_code)]
mod staging;

use handbook_contracts::FingerprintV1;
use handbook_dock::{
    production_manifest_bytes_v1, DockErrorV1, DockManifestListRequestV1, DockManifestRegistryV1,
    DockResolvedFirstPartyBundleV1,
};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;

const OLD_EXECUTABLE_FINGERPRINT: &str =
    "sha256:13b8f78780e540439c30c5e805111e1acce19895c4cce83b4d5563f77e5f09b9";
const OLD_BUNDLE_FINGERPRINT: &str =
    "sha256:fd12372df6a2006ee311a43d505ad32e0ecb7c3f5c9999b42a1152c7c6c77855";
const OLD_MANIFEST_FINGERPRINT: &str =
    "sha256:8367b798dd56c2f374b7a5c321f86d2f69461529a254329b4bb46a6fb12a9296";

fn fingerprint(bytes: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(bytes))
}

fn manifest(bytes: &[u8]) -> Value {
    serde_json::from_slice(bytes).unwrap()
}

#[test]
fn isolated_normal_package_builds_reproduce_the_complete_frozen_identity() {
    assert_eq!(
        production::CARGO_ARGUMENTS,
        [
            "rustc",
            "--offline",
            "--locked",
            "-p",
            "handbook-dock-json-schema",
            "--bin",
            "handbook-dock-json-schema",
            "--",
            "-C",
            "link-arg=/Brepro",
        ]
    );
    assert_eq!(
        env!("HANDBOOK_DOCK_JSON_SCHEMA_TARGET"),
        production::SUPPORTED_TARGET
    );

    let workspace = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let first_target = tempfile::tempdir().unwrap();
    let second_target = tempfile::tempdir().unwrap();
    let first = production::produce_current_package_v1(&workspace, first_target.path()).unwrap();
    let second = production::produce_current_package_v1(&workspace, second_target.path()).unwrap();

    let first_executable = fs::read(&first.executable_path).unwrap();
    let second_executable = fs::read(&second.executable_path).unwrap();
    assert_eq!(first_executable, second_executable);

    let first_host = tempfile::tempdir().unwrap();
    let second_host = tempfile::tempdir().unwrap();
    let first_staged = staging::stage_native_package_v1(
        &first.executable_path,
        first_host.path(),
        production::SUPPORTED_TARGET,
        ".exe",
    )
    .unwrap();
    let second_staged = staging::stage_native_package_v1(
        &second.executable_path,
        second_host.path(),
        production::SUPPORTED_TARGET,
        ".exe",
    )
    .unwrap();
    assert_eq!(first_staged.manifest_bytes, second_staged.manifest_bytes);
    assert_eq!(
        first_staged.manifest_bytes,
        production_manifest_bytes_v1(),
        "the reproducible normal package must equal frozen host truth; derived manifest: {}",
        String::from_utf8_lossy(&first_staged.manifest_bytes)
    );

    let frozen = manifest(production_manifest_bytes_v1());
    let executable_fingerprint = fingerprint(&first_executable);
    assert_eq!(
        frozen["implementation_bundle"]["files"],
        json!([{
            "path": "bin/handbook-dock-json-schema.exe",
            "length": first_executable.len() as u64,
            "sha256": executable_fingerprint,
        }])
    );
    assert_eq!(
        frozen["implementation_bundle"]["entrypoint"],
        "bin/handbook-dock-json-schema.exe"
    );
    assert_eq!(
        frozen["implementation_bundle"]["entrypoint_fingerprint"],
        executable_fingerprint
    );
    assert_eq!(
        frozen["implementation_bundle"]["fixed_arguments"],
        json!([])
    );
    assert_eq!(
        frozen["implementation_bundle"]["arguments_fingerprint"],
        "sha256:4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945"
    );
    assert_eq!(
        frozen["implementation_bundle"]["target"],
        production::SUPPORTED_TARGET
    );

    let mut bundle_preimage = frozen["implementation_bundle"].clone();
    let bundle_fingerprint = bundle_preimage
        .as_object_mut()
        .unwrap()
        .remove("bundle_fingerprint")
        .unwrap();
    assert_eq!(
        bundle_fingerprint,
        fingerprint(&serde_json_canonicalizer::to_vec(&bundle_preimage).unwrap())
    );
    let mut manifest_preimage = frozen.clone();
    let manifest_fingerprint = manifest_preimage
        .as_object_mut()
        .unwrap()
        .remove("manifest_fingerprint")
        .unwrap();
    assert_eq!(
        manifest_fingerprint,
        fingerprint(&serde_json_canonicalizer::to_vec(&manifest_preimage).unwrap())
    );

    DockResolvedFirstPartyBundleV1::resolve_from_host_directory_for_testing(first_host.path())
        .unwrap();

    let test_binary = fs::read(env!("CARGO_BIN_EXE_handbook-dock-json-schema")).unwrap();
    assert_ne!(test_binary, first_executable);
    let test_host = tempfile::tempdir().unwrap();
    let test_staged = staging::stage_native_package_v1(
        Path::new(env!("CARGO_BIN_EXE_handbook-dock-json-schema")),
        test_host.path(),
        production::SUPPORTED_TARGET,
        ".exe",
    )
    .unwrap();
    assert_ne!(test_staged.manifest_bytes, first_staged.manifest_bytes);

    let frozen_text = String::from_utf8(production_manifest_bytes_v1().to_vec()).unwrap();
    for superseded in [
        OLD_EXECUTABLE_FINGERPRINT,
        OLD_BUNDLE_FINGERPRINT,
        OLD_MANIFEST_FINGERPRINT,
    ] {
        assert!(!frozen_text.contains(superseded));
    }
    let registry = DockManifestRegistryV1::production();
    let page = registry
        .list(DockManifestListRequestV1::first(1).unwrap())
        .unwrap();
    let old_manifest = FingerprintV1::parse(OLD_MANIFEST_FINGERPRINT).unwrap();
    assert!(matches!(
        registry.read(page.entries()[0].manifest_ref(), &old_manifest),
        Err(DockErrorV1::Substituted)
    ));
}
