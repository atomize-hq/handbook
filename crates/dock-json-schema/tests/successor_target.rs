#[path = "../package-support/staging.rs"]
#[allow(dead_code)]
mod staging;

use handbook_contracts::{native_local_evaluation_capability_binding_v3, LocalValidatorDependencyV2};
use handbook_dock::production_manifest_bytes_v1;
use handbook_engine::repository_durability::admit_native_production_manifest_v1;
use std::fs;

#[test]
fn staging_selects_successor_only_for_the_exact_windows_target() {
    let source = tempfile::tempdir().unwrap();
    let executable = source.path().join("handbook-dock-json-schema.exe");
    fs::write(&executable, b"distinct test member").unwrap();
    for (target, version) in [
        ("x86_64-pc-windows-msvc", "1.0.2"),
        ("x86_64-pc-windows-gnu", "1.0.2"),
        ("x86_64-unknown-linux-gnu", "1.0.2"),
        ("aarch64-apple-darwin", "1.0.2"),
    ] {
        let host = tempfile::tempdir().unwrap();
        let staged = staging::stage_native_package_v1(&executable, host.path(), target, ".exe").unwrap();
        let manifest: serde_json::Value = serde_json::from_slice(&staged.manifest_bytes).unwrap();
        assert_eq!(manifest["dock_version"], version);
        assert_eq!(manifest["dock_ref"], format!("handbook.dock.json-schema@{version}"));
        assert_eq!(manifest["implementation_bundle"]["bundle_ref"], format!("handbook.dock.bundle.json-schema-native@{version}"));
        assert_eq!(manifest["implementation_bundle"]["target"], target);
        assert_eq!(staged.package_root, host.path().join("handbook-docks/handbook.dock.json-schema").join(version).join(target));
        assert_eq!(fs::read(staged.executable_path).unwrap(), b"distinct test member");
    }
}

#[test]
fn compiled_manifest_is_the_only_admitted_successor_descriptor() {
    let bytes = production_manifest_bytes_v1();
    let target = env!("HANDBOOK_DOCK_JSON_SCHEMA_TARGET");
    let version = "1.0.2";
    let descriptor = LocalValidatorDependencyV2::from_manifest_json(bytes).unwrap();
    assert_eq!(descriptor.manifest.reference().as_str(), format!("handbook.dock.json-schema@{version}"));
    assert!(admit_native_production_manifest_v1(bytes).is_ok());
    let mut changed = bytes.to_vec();
    changed.push(b' ');
    assert!(admit_native_production_manifest_v1(&changed).is_err());
    let capability = native_local_evaluation_capability_binding_v3().unwrap();
    assert_eq!(capability.reference().as_str(), format!("handbook.dock.json-schema.semantic-capability@{version}"));
    if target == "x86_64-pc-windows-msvc" {
        let historical = include_bytes!("../../dock/production-manifests/x86_64-pc-windows-msvc/manifest-1.0.0.json");
        let historical = historical.strip_suffix(b"\n").unwrap();
        assert!(admit_native_production_manifest_v1(historical).is_err());
        assert!(LocalValidatorDependencyV2::from_manifest_json(historical).is_err());
    }
}
