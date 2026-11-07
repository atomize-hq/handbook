//! Exact SDK-boundary refusal must preserve existing native originals.
use super::*;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::io::Read;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

// Inspect only the actual Contracts and Dock owner trees used by this test.
// File-native physical indexes, frames, packs and matchers are not traversed.
fn snapshot_native_tree_walk_v1(root: &Path, path: &Path, out: &mut BTreeMap<String, String>) {
    let relative = path.strip_prefix(root).unwrap().to_str().unwrap().to_owned();
    let metadata = match std::fs::symlink_metadata(path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            out.insert(relative, "absent".into());
            return;
        }
        Err(error) => panic!("owner artifact metadata unavailable: {error}"),
    };
    assert!(!metadata.file_type().is_symlink());
    if metadata.is_dir() {
        out.insert(relative, format!("directory:{:o}", metadata.permissions().mode()));
        for child in std::fs::read_dir(path).unwrap() {
            snapshot_native_tree_walk_v1(root, &child.unwrap().path(), out);
        }
    } else {
        assert!(metadata.is_file());
        let mut file = std::fs::File::open(path).unwrap();
        let mut digest = Sha256::new();
        let mut buffer = [0_u8; 64 * 1024];
        loop {
            let count = file.read(&mut buffer).unwrap();
            if count == 0 { break; }
            digest.update(&buffer[..count]);
        }
        out.insert(relative, format!("file:{}:{:o}:{:x}", metadata.len(), metadata.permissions().mode(), digest.finalize()));
    }
}

fn snapshot_native_tree_v1(root: &Path) -> BTreeMap<String, String> {
    let mut snapshot = BTreeMap::new();
    for relative in [
        LOCAL_EVALUATION_OWNER_ROOT_V3,
        DOCK_LOCAL_EVALUATION_ROOT_V1,
        DOCK_OPERATIONAL_STORE_RELATIVE_ROOT_V1,
    ] {
        snapshot_native_tree_walk_v1(root, &root.join(relative), &mut snapshot);
    }
    snapshot
}

fn assert_manifest_refusal(
    result: Result<DockLocalEvaluationAttemptOutcomeV1, LocalEvaluationSdkErrorV1>,
) {
    match result {
        Err(LocalEvaluationSdkErrorV1::Engine(refusal)) => assert_eq!(refusal.code, "native_manifest"),
        other => panic!("expected explicit native-manifest refusal, got {other:?}"),
    }
}

#[test]
fn native_manifest_rejection_preserves_originals() {
    on_stack(|| {
        let _package = InstalledProductionPackage::install();
        let manifest = production_manifest();
        let staged = std::env::current_exe().unwrap().parent().unwrap()
            .join("handbook-docks/handbook.dock.json-schema/1.0.2")
            .join(handbook_dock::host_target_tuple_v1())
            .join("bundle")
            .join(manifest.implementation_bundle().entrypoint().as_str());
        let source_validator = std::env::var_os("HANDBOOK_PACKET2_VALIDATOR_EXE")
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|| Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../../target/debug")
                .join(format!("handbook-dock-json-schema{}", std::env::consts::EXE_SUFFIX)));
        let staged_mode = std::fs::metadata(staged).unwrap().permissions().mode() & 0o7777;
        let source_mode = std::fs::metadata(source_validator).unwrap().permissions().mode() & 0o7777;
        assert_eq!(staged_mode, source_mode, "staged validator permissions changed");
        assert_ne!(staged_mode & 0o111, 0, "the exact staged validator must be executable");
        let fixture = Fixture::new();
        let original = fixture.form(
            "native-manifest-original-attempt-0001", supplied_validation_content(),
            manifest.resolution_ceiling().clone(), LocalEvaluationUseIntentV1::DecisionSeeking,
        );
        assert_eq!(fixture.assess(&original).stage, LocalEvaluationStageV3::Attemptable);
        fixture.sdk.commit_local_evaluation_collection(&fixture.session(&original)).unwrap();
        let original_outcome = fixture.sdk.execute_local_evaluation_once(
            &fixture.session(&original), original.process_request.as_ref().unwrap(),
            &dock_reach(), &DockCancellationV1::new(),
        ).unwrap();
        let DockLocalEvaluationAttemptOutcomeV1::Terminal { intent, terminal, replayed: false } = original_outcome else {
            panic!("real original validator execution did not complete");
        };
        assert_eq!(terminal.kind, "completed");
        let saved_attempt = intent.binding().unwrap();
        let saved_terminal = serde_json_canonicalizer::to_vec(&terminal).unwrap();
        finish(&fixture, &original);
        let saved_original = fixture.sdk.recover_local_evaluation(&original.snapshot.locator).unwrap();
        assert_eq!(saved_original.stage, LocalEvaluationStageV3::ResultClosed);
        let saved_result_locator = saved_original.result_locator().unwrap();
        let saved_history = fixture.sdk.read_local_evaluation_result_history(&saved_result_locator).unwrap();
        assert!(saved_history.exact_replay_available);
        let saved_result_bytes = saved_history.result.canonical_json_bytes().unwrap();

        // Preserve a genuine pre-allocation request locator from a separate
        // real formation, so request recovery is not reconstructed afterward.
        let mut saved_input = original.snapshot.original_input.clone();
        saved_input.request_key = "native-manifest-saved-request-0002".into();
        let prepared = fixture.sdk.prepare_local_evaluation(&fixture.initial, &fixture.authority, saved_input).unwrap();
        let saved_request = prepared.request_locator();
        let saved_request_snapshot = fixture.sdk.form_local_evaluation(&fixture.authority, prepared).unwrap();

        let pending = fixture.form(
            "native-manifest-new-attempt-0003", supplied_validation_content(),
            manifest.resolution_ceiling().clone(), LocalEvaluationUseIntentV1::DecisionSeeking,
        );
        assert_eq!(fixture.assess(&pending).stage, LocalEvaluationStageV3::Attemptable);
        fixture.sdk.commit_local_evaluation_collection(&fixture.session(&pending)).unwrap();
        let baseline = snapshot_native_tree_v1(fixture.root.path());
        let foreign = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"),
            "/../dock/production-manifests/x86_64-pc-windows-msvc/manifest.json"))
            .strip_suffix(b"\n").unwrap();
        // Derive a stale fingerprint from the actual Linux descriptor while
        // retaining its complete canonical document shape; do not edit assets.
        let mut stale_manifest: Value = serde_json::from_slice(
            handbook_dock::production_manifest_bytes_v1(),
        ).unwrap();
        let timeout = stale_manifest["limits"]["wall_timeout_ms"].as_u64().unwrap();
        stale_manifest["limits"]["wall_timeout_ms"] = json!(timeout + 1);
        let stale_bytes = serde_json_canonicalizer::to_vec(&stale_manifest).unwrap();
        assert!(stale_bytes.len() <= 1024 * 1024);
        let stale: &'static [u8] = Box::leak(stale_bytes.into_boxed_slice());
        let call = || fixture.sdk.execute_local_evaluation_once(
            &fixture.session(&pending), pending.process_request.as_ref().unwrap(),
            &dock_reach(), &DockCancellationV1::new(),
        );
        with_additional_native_manifest_check_for_testing_v1(foreign, || {
            assert_manifest_refusal(call());
            assert_eq!(snapshot_native_tree_v1(fixture.root.path()), baseline);
            with_additional_native_manifest_check_for_testing_v1(stale, || {
                assert_manifest_refusal(call());
                assert_eq!(snapshot_native_tree_v1(fixture.root.path()), baseline);
            });
            let unwind = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                with_additional_native_manifest_check_for_testing_v1(
                    handbook_dock::production_manifest_bytes_v1(), || panic!("test guard unwind"),
                )
            }));
            assert!(unwind.is_err());
            assert_manifest_refusal(call());
            assert_eq!(snapshot_native_tree_v1(fixture.root.path()), baseline);

            // Recovery is intentionally after every immediate no-effect check.
            assert_eq!(fixture.sdk.recover_local_evaluation(&original.snapshot.locator).unwrap(), saved_original);
            assert_eq!(fixture.sdk.recover_local_evaluation_request(&saved_request).unwrap(), Some(saved_request_snapshot.clone()));
            let history = fixture.sdk.read_local_evaluation_result_history(&saved_result_locator).unwrap();
            assert!(history.exact_replay_available);
            assert_eq!(history.result.canonical_json_bytes().unwrap(), saved_result_bytes);
            assert_eq!(history.dependencies, saved_history.dependencies);
            let recovered = fixture.sdk.recover_local_evaluation_attempt(&saved_attempt, &dock_reach()).unwrap();
            let DockLocalEvaluationAttemptOutcomeV1::Terminal { terminal, replayed: true, .. } = recovered else {
                panic!("saved original attempt was not replayed");
            };
            assert_eq!(serde_json_canonicalizer::to_vec(&terminal).unwrap(), saved_terminal);
            let after_recovery = snapshot_native_tree_v1(fixture.root.path());
            let execution_prefix = format!("{DOCK_OPERATIONAL_STORE_RELATIVE_ROOT_V1}/executions");
            let executions = |map: &BTreeMap<String, String>| map.iter()
                .filter(|(path, _)| path.starts_with(&execution_prefix))
                .map(|(path, value)| (path.clone(), value.clone())).collect::<BTreeMap<_, _>>();
            assert_eq!(executions(&baseline), executions(&after_recovery), "recovery must not relaunch");
        });
        let outcome = call().unwrap();
        let DockLocalEvaluationAttemptOutcomeV1::Terminal { terminal, replayed: false, .. } = outcome else {
            panic!("valid post-scope validator execution did not complete");
        };
        assert_eq!(terminal.kind, "completed");
    });
}
