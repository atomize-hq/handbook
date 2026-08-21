#[path = "../../sdk/tests/packet5_support/mod.rs"]
#[allow(dead_code)]
mod sdk_support;
#[path = "../package-support/staging.rs"]
#[allow(dead_code)]
mod staging;

use handbook_contracts::{
    ContentAddressedBindingV1, EvaluationCaseV1, EvaluationSubjectV1, ExactRefV1, FingerprintV1,
    MatcherInputValueV1, SemanticTimeV1, StableIdV1,
};
use handbook_dock::*;
use serde_json::json;
use std::fs;
use std::path::Path;

type PackageMutation = (&'static str, fn(&staging::StagedPackageV1));

fn fingerprint(hex: char) -> FingerprintV1 {
    FingerprintV1::parse(&format!("sha256:{}", hex.to_string().repeat(64))).unwrap()
}

fn stable_id(value: &str) -> StableIdV1 {
    StableIdV1::parse(value).unwrap()
}

fn production_manifest() -> DockManifestV1 {
    let registry = DockManifestRegistryV1::production();
    let page = registry
        .list(DockManifestListRequestV1::first(10).unwrap())
        .unwrap();
    assert_eq!(page.entries().len(), 1);
    match registry
        .read(
            page.entries()[0].manifest_ref(),
            page.entries()[0].manifest_fingerprint(),
        )
        .unwrap()
    {
        DockManifestReadOutcomeV1::Found(manifest) => *manifest,
        DockManifestReadOutcomeV1::Absent => panic!("embedded production manifest was absent"),
    }
}

fn stage_package(host_directory: &Path) -> staging::StagedPackageV1 {
    staging::stage_native_package_v1(
        Path::new(env!("CARGO_BIN_EXE_handbook-dock-json-schema")),
        host_directory,
        env!("HANDBOOK_DOCK_JSON_SCHEMA_TARGET"),
        std::env::consts::EXE_SUFFIX,
    )
    .unwrap()
}

fn validation_content() -> serde_json::Value {
    let schema = json!({"$schema":"https://json-schema.org/draft/2020-12/schema","type":"string"});
    let document_fingerprint =
        handbook_dock_json_schema::CanonicalFingerprintV1::for_value(&schema).unwrap();
    let instance = json!("valid");
    let instance_fingerprint =
        handbook_dock_json_schema::CanonicalFingerprintV1::for_value(&instance).unwrap();
    let closure_members = json!([{
        "path":"definitions/schemas/test/1.0.0.schema.json",
        "document_fingerprint":document_fingerprint.as_str()
    }]);
    let closure_fingerprint =
        handbook_dock_json_schema::CanonicalFingerprintV1::for_value(&closure_members).unwrap();
    json!({
        "root_schema":{"ref":"handbook.schema.test@1.0.0","fingerprint":document_fingerprint.as_str()},
        "root_document":"definitions/schemas/test/1.0.0.schema.json",
        "root_document_fingerprint":document_fingerprint.as_str(),
        "closure_fingerprint":closure_fingerprint.as_str(),
        "documents":[{"path":"definitions/schemas/test/1.0.0.schema.json","schema":schema,"document_fingerprint":document_fingerprint.as_str()}],
        "instance":{"value":instance,"instance_fingerprint":instance_fingerprint.as_str()}
    })
}

fn symlink_file(source: &Path, destination: &Path) {
    #[cfg(windows)]
    std::os::windows::fs::symlink_file(source, destination).unwrap();
    #[cfg(unix)]
    std::os::unix::fs::symlink(source, destination).unwrap();
}

#[test]
fn production_registry_embeds_exact_one_entry_without_package_material() {
    let registry = DockManifestRegistryV1::production();
    assert_eq!(registry.production_len(), 1);
    let page = registry
        .list(DockManifestListRequestV1::first(10).unwrap())
        .unwrap();
    assert_eq!(page.entries().len(), 1);
    let manifest = production_manifest();
    assert_eq!(manifest.dock_ref().as_str(), staging::DOCK_REF_V1);
    assert_eq!(manifest.implementation_bundle().files().len(), 1);
    assert!(manifest
        .implementation_bundle()
        .fixed_arguments()
        .is_empty());
    assert_eq!(
        manifest.implementation_bundle().target().as_str(),
        host_target_tuple_v1()
    );
    assert_eq!(
        manifest.canonical_json_bytes().unwrap(),
        production_manifest_bytes_v1()
    );
    assert!(!String::from_utf8_lossy(production_manifest_bytes_v1()).contains("latest"));
}

#[test]
fn production_package_requires_exact_manifest_and_one_regular_member() {
    let valid_host = tempfile::tempdir().unwrap();
    let staged = stage_package(valid_host.path());
    assert_eq!(
        staged.manifest_bytes,
        production_manifest_bytes_v1(),
        "actual staged manifest: {}",
        String::from_utf8_lossy(&staged.manifest_bytes)
    );
    DockResolvedFirstPartyBundleV1::resolve_from_host_directory_for_testing(valid_host.path())
        .unwrap();

    let cases: &[PackageMutation] = &[
        ("missing manifest", |staged| {
            fs::remove_file(staged.package_root.join("manifest.json")).unwrap();
        }),
        ("unreadable manifest shape", |staged| {
            fs::remove_file(staged.package_root.join("manifest.json")).unwrap();
            fs::create_dir(staged.package_root.join("manifest.json")).unwrap();
        }),
        ("substituted manifest", |staged| {
            fs::write(staged.package_root.join("manifest.json"), b"{}").unwrap();
        }),
        ("missing binary", |staged| {
            fs::remove_file(&staged.executable_path).unwrap();
        }),
        ("substituted binary", |staged| {
            fs::write(&staged.executable_path, b"substituted").unwrap();
        }),
        ("extra member", |staged| {
            fs::write(staged.package_root.join("bundle/extra"), b"extra").unwrap();
        }),
        ("non-regular member", |staged| {
            fs::remove_file(&staged.executable_path).unwrap();
            fs::create_dir(&staged.executable_path).unwrap();
        }),
        ("symlink member", |staged| {
            let replacement = staged.package_root.join("replacement.exe");
            fs::write(&replacement, b"replacement").unwrap();
            fs::remove_file(&staged.executable_path).unwrap();
            symlink_file(&replacement, &staged.executable_path);
        }),
    ];
    for (name, mutate) in cases {
        let host = tempfile::tempdir().unwrap();
        let staged = stage_package(host.path());
        mutate(&staged);
        assert!(
            DockResolvedFirstPartyBundleV1::resolve_from_host_directory_for_testing(host.path())
                .is_err(),
            "{name}"
        );
    }

    let wrong_target_host = tempfile::tempdir().unwrap();
    let staged = stage_package(wrong_target_host.path());
    let wrong = wrong_target_host
        .path()
        .join("handbook-docks/handbook.dock.json-schema/1.0.0/wrong-target-tuple");
    fs::rename(&staged.package_root, wrong).unwrap();
    assert!(
        DockResolvedFirstPartyBundleV1::resolve_from_host_directory_for_testing(
            wrong_target_host.path()
        )
        .is_err()
    );
}

#[test]
fn frozen_manifest_rejects_changed_derived_fields_and_non_exact_installed_bytes() {
    let mutations = [
        (
            "entry length",
            "/implementation_bundle/files/0/length",
            json!(1),
        ),
        (
            "entry digest",
            "/implementation_bundle/files/0/sha256",
            json!(format!("sha256:{}", "0".repeat(64))),
        ),
        (
            "entrypoint digest",
            "/implementation_bundle/entrypoint_fingerprint",
            json!(format!("sha256:{}", "0".repeat(64))),
        ),
        (
            "arguments",
            "/implementation_bundle/fixed_arguments",
            json!(["--replacement"]),
        ),
        (
            "target",
            "/implementation_bundle/target",
            json!("wrong-target-tuple"),
        ),
        (
            "bundle fingerprint",
            "/implementation_bundle/bundle_fingerprint",
            json!(format!("sha256:{}", "0".repeat(64))),
        ),
        (
            "manifest fingerprint",
            "/manifest_fingerprint",
            json!(format!("sha256:{}", "0".repeat(64))),
        ),
    ];
    for (name, pointer, replacement) in mutations {
        let mut value: serde_json::Value =
            serde_json::from_slice(production_manifest_bytes_v1()).unwrap();
        *value.pointer_mut(pointer).unwrap() = replacement;
        let bytes = serde_json_canonicalizer::to_vec(&value).unwrap();
        assert!(
            DockManifestV1::admit_canonical_json(&bytes).is_err(),
            "{name}"
        );

        let host = tempfile::tempdir().unwrap();
        let staged = stage_package(host.path());
        fs::write(staged.package_root.join("manifest.json"), bytes).unwrap();
        assert!(
            DockResolvedFirstPartyBundleV1::resolve_from_host_directory_for_testing(host.path())
                .is_err(),
            "installed {name}"
        );
    }
}

#[test]
fn exact_package_launches_directly_and_replays_without_a_second_spawn() {
    let host = tempfile::tempdir().unwrap();
    stage_package(host.path());
    let resolved =
        DockResolvedFirstPartyBundleV1::resolve_from_host_directory_for_testing(host.path())
            .unwrap();
    let manifest = production_manifest();
    let subject =
        ContentAddressedBindingV1::new("subject.fixture#revision/exact", fingerprint('6')).unwrap();
    let run = DockEvaluationRunBindingV1::new(DockEvaluationRunBindingInputV1 {
        evaluation_run: ContentAddressedBindingV1::new(
            "handbook.contract.fixture@1.0.0#evaluation/exact",
            fingerprint('7'),
        )
        .unwrap(),
        evaluation_lifecycle_basis: ContentAddressedBindingV1::new(
            "handbook.contract.fixture@1.0.0#evaluation-lifecycle-basis/exact",
            fingerprint('8'),
        )
        .unwrap(),
        contract: handbook_contracts::ExactBindingV1::new(
            ExactRefV1::parse("handbook.contract.fixture@1.0.0").unwrap(),
            fingerprint('6'),
        ),
        claim_ids: vec![stable_id("payload.schema-conformant")],
        subject: EvaluationSubjectV1::new(
            subject.clone(),
            vec![MatcherInputValueV1::new(stable_id("artifact.kind"), "json-document").unwrap()],
        )
        .unwrap(),
        cases: vec![EvaluationCaseV1::new(
            stable_id("case.schema-conformance"),
            subject,
            vec![MatcherInputValueV1::new(stable_id("validation.case"), "primary").unwrap()],
        )
        .unwrap()],
        requested_resolution: manifest.resolution_ceiling().clone(),
        semantic_time_utc: SemanticTimeV1::parse("2026-08-20T12:00:00Z").unwrap(),
        lifecycle_head_fingerprint: fingerprint('8'),
    })
    .unwrap();
    let content = validation_content();
    let request = DockProcessRequestV1::new(
        &manifest,
        DockProcessRequestInputV1 {
            run,
            requested_evidence_kinds: vec![stable_id(staging::EVIDENCE_KIND_V1)],
            inputs: vec![DockInlineJsonInputV1::new(
                stable_id("handbook.input.json-schema-validation"),
                "application/json",
                manifest.supported_inputs()[0].schema().clone(),
                content,
            )
            .unwrap()],
            effective_limits: manifest.limits().clone(),
        },
    )
    .unwrap();
    let repo = tempfile::tempdir().unwrap();
    let store = DockOperationalStoreV1::open(repo.path()).unwrap();
    let cancellation = DockCancellationV1::new();
    let evaluation_run_jcs = serde_json_canonicalizer::to_vec(&json!({
        "evaluation_run_fingerprint": request.run().evaluation_run().fingerprint(),
        "evaluation_run_ref": request.run().evaluation_run().reference(),
        "schema_ref": "handbook.contract-evaluation-run@1.0.0",
    }))
    .unwrap();
    let execute = || {
        DockProcessExecutorV1::new()
            .execute(DockExecutionInputV1 {
                store: &store,
                resolved_bundle: &resolved,
                request: &request,
                evaluation_run_jcs: &evaluation_run_jcs,
                idempotency_key: "packet4-production-replay",
                cancellation: &cancellation,
            })
            .unwrap()
    };
    let first = execute();
    let replay = execute();
    assert_eq!(first.disposition(), DockStoreDispositionV1::Committed);
    assert_eq!(replay.disposition(), DockStoreDispositionV1::Replayed);
    assert_eq!(first.execution_record(), replay.execution_record());
    assert_eq!(first.candidates(), replay.candidates());
    assert_eq!(first.candidates().len(), 1);
}

struct InstalledProductionPackage {
    root: std::path::PathBuf,
}

impl InstalledProductionPackage {
    fn install() -> Self {
        let host = std::env::current_exe().unwrap();
        let host = host.parent().unwrap();
        let package_root = host
            .join("handbook-docks/handbook.dock.json-schema/1.0.0")
            .join(host_target_tuple_v1());
        assert!(
            !package_root.exists(),
            "test package path must start absent"
        );
        let staged = stage_package(host);
        assert_eq!(staged.package_root, package_root);
        Self { root: package_root }
    }
}

impl Drop for InstalledProductionPackage {
    fn drop(&mut self) {
        if self.root.exists() {
            fs::remove_dir_all(&self.root).unwrap();
            let version = self.root.parent().unwrap();
            let dock = version.parent().unwrap();
            let docks = dock.parent().unwrap();
            fs::remove_dir(version).unwrap();
            fs::remove_dir(dock).unwrap();
            fs::remove_dir(docks).unwrap();
        }
    }
}

#[test]
fn default_sdk_run_uses_only_the_fixed_production_package_and_preserves_replay_and_conflict() {
    use handbook_sdk::contract_membrane::{
        canonical_json_bytes, execute_dock_operation_with_context, parse_canonical_json,
        ContractLimits, DockRunResultV1, IdempotencyStateRef, TerminalStatus,
    };

    let _installed = InstalledProductionPackage::install();
    let manifest = production_manifest();
    let fixture = sdk_support::ProductionDockFixture::new();
    let raw_key = "packet-four-production-key-must-not-leak";
    let request = fixture.request(&manifest, raw_key, validation_content());
    let execute = |request| {
        execute_dock_operation_with_context(fixture.repository_path(), request, fixture.policies())
            .unwrap()
    };
    let committed = execute(&request);
    assert_eq!(committed.terminal().status(), TerminalStatus::Ok);
    assert_eq!(committed.receipts().len(), 1);
    let result: DockRunResultV1 = parse_canonical_json(
        &canonical_json_bytes(
            committed.terminal().data().unwrap(),
            &ContractLimits::packet0(),
        )
        .unwrap(),
        &ContractLimits::packet0(),
    )
    .unwrap();
    assert_eq!(result.candidates().len(), 1);

    let replay_request = fixture.request(&manifest, raw_key, validation_content());
    let replay = execute(&replay_request);
    assert_eq!(replay.terminal().status(), TerminalStatus::Ok);
    assert_eq!(replay.receipts(), committed.receipts());
    assert!(matches!(
        replay.idempotency().as_ref(),
        IdempotencyStateRef::Established { replayed: true, .. }
    ));

    let mut changed = validation_content();
    changed["instance"]["value"] = json!(17);
    let conflict_request = fixture.request(&manifest, raw_key, changed);
    let conflict = execute(&conflict_request);
    assert_eq!(conflict.terminal().status(), TerminalStatus::Refused);
    assert!(conflict.receipts().is_empty());
    assert_eq!(
        serde_json::to_value(&conflict).unwrap()["refusals"][0]["details"]["cause"],
        "conflict"
    );

    for response in [&committed, &replay, &conflict] {
        let bytes = canonical_json_bytes(response, &ContractLimits::packet0()).unwrap();
        assert!(!bytes
            .windows(raw_key.len())
            .any(|window| window == raw_key.as_bytes()));
        assert!(!String::from_utf8(bytes)
            .unwrap()
            .contains(fixture.repository_path().to_string_lossy().as_ref()));
    }
}

#[test]
fn production_resolution_and_launch_sources_expose_no_mapping_or_ambient_search() {
    let source = include_str!("../../dock/src/executor.rs");
    let start = source.find("pub fn resolve_production()").unwrap();
    let end = source
        .find("pub fn for_testing(")
        .expect("the explicit test seam follows production resolution");
    let production = &source[start..end];
    assert!(production.contains("std::env::current_exe()"));
    for forbidden in [
        "std::env::var",
        "current_dir",
        "PATH",
        "HOME",
        "repository",
        "package-manager",
        "canonicalize",
    ] {
        assert!(!production.contains(forbidden), "{forbidden}");
    }
    let launch_start = source
        .find("let mut command = Command::new(&entrypoint)")
        .unwrap();
    let launch_end = source.find("let spawn = command.spawn();").unwrap();
    let launch = &source[launch_start..launch_end];
    assert!(launch.contains("Command::new("));
    assert!(launch.contains(".env_clear()"));
    assert!(!launch.contains("cmd.exe"));
    assert!(!launch.contains("powershell"));
    assert!(!launch.contains("sh -c"));
}
