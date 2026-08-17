use std::fs;
use std::path::Path;
use std::process::{Command, Output};

const SMOKE_RUNBOOK: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/specs/handbook-contract-membrane/slices/HCM-2.4/proof/implementation/post-phase-2-smoke-runbook.md"
));

fn run_in(root: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_handbook"))
        .current_dir(root)
        .args(args)
        .output()
        .unwrap_or_else(|error| panic!("run `{}`: {error}", args.join(" ")))
}

fn stdout(output: &Output) -> String {
    String::from_utf8(output.stdout.clone()).expect("stdout UTF-8")
}

fn assert_setup_invocation(output: &Output) -> serde_json::Value {
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stderr.is_empty());
    assert!(output.stdout.ends_with(b"\n"));
    assert!(!output.stdout.ends_with(b"\n\n"));
    let value: serde_json::Value = serde_json::from_slice(&output.stdout).expect("setup JSON");
    assert_eq!(value["schema"]["ref"], "handbook.invocation-response@1.0.0");
    assert_eq!(value["response_kind"], "use_case");
    assert_eq!(
        value["response"]["use_case"]["ref"],
        "handbook.use-case.repository-setup@1.0.0"
    );
    assert_eq!(value["response"]["status"], "blocked");
    value
}

#[test]
fn all_setup_invocations_emit_one_new_envelope_and_never_offer_legacy_json() {
    for args in [
        ["setup", "--json"].as_slice(),
        ["setup", "init", "--json"].as_slice(),
    ] {
        let repo = tempfile::tempdir().expect("tempdir");
        fs::create_dir(repo.path().join(".git")).expect("git root");
        let value = assert_setup_invocation(&run_in(repo.path(), args));
        let receipts = value["response"]["receipts"].as_array().unwrap();
        assert_eq!(receipts.len(), 2, "root and repository identity writes");
    }

    let repo = tempfile::tempdir().expect("tempdir");
    fs::create_dir(repo.path().join(".git")).expect("git root");
    let _ = run_in(repo.path(), &["setup"]);
    fs::create_dir_all(repo.path().join(".handbook/state/a")).unwrap();
    fs::write(repo.path().join(".handbook/state/a/one.json"), "{}").unwrap();
    fs::write(repo.path().join(".handbook/state/two.json"), "{}").unwrap();
    let value = assert_setup_invocation(&run_in(
        repo.path(),
        &["setup", "refresh", "--reset-state", "--json"],
    ));
    let receipts = value["response"]["receipts"].as_array().unwrap();
    assert_eq!(receipts.len(), 1);
    assert_eq!(receipts[0]["condition"]["application"], "sequential");
    let removed = receipts[0]["condition"]["removed_paths"]
        .as_array()
        .unwrap();
    assert!(removed.len() <= 128);
    assert!(removed
        .iter()
        .any(|path| path == ".handbook/state/a/one.json"));
    assert!(removed
        .iter()
        .any(|path| path == ".handbook/state/two.json"));

    let conflict = run_in(repo.path(), &["setup", "--legacy-json"]);
    assert_eq!(conflict.status.code(), Some(2));
}

#[test]
fn setup_human_and_new_json_have_semantic_and_exit_parity_for_all_invocations() {
    for invocation in ["auto", "init", "refresh"] {
        let human_repo = tempfile::tempdir().unwrap();
        let json_repo = tempfile::tempdir().unwrap();
        for repo in [human_repo.path(), json_repo.path()] {
            fs::create_dir(repo.join(".git")).unwrap();
            if invocation == "refresh" {
                let setup = run_in(repo, &["setup"]);
                assert_eq!(setup.status.code(), Some(1));
            }
        }
        let human_args = match invocation {
            "auto" => vec!["setup"],
            "init" => vec!["setup", "init"],
            "refresh" => vec!["setup", "refresh"],
            _ => unreachable!(),
        };
        let mut json_args = human_args.clone();
        json_args.push("--json");

        let human = run_in(human_repo.path(), &human_args);
        let machine = run_in(json_repo.path(), &json_args);
        assert_eq!(human.status.code(), machine.status.code(), "{invocation}");
        assert!(human.stderr.is_empty(), "{invocation}");
        assert!(machine.stderr.is_empty(), "{invocation}");
        let human = stdout(&human);
        let machine: serde_json::Value = serde_json::from_slice(&machine.stdout).unwrap();
        let readiness = machine["response"]["blockers"][0]["details"]["result"]["readiness"]
            .as_str()
            .unwrap();
        assert_eq!(
            human.lines().next(),
            Some(match readiness {
                "action_required" => "OUTCOME: ACTION_REQUIRED",
                "indeterminate" => "OUTCOME: INDETERMINATE",
                "invalid" => "OUTCOME: INVALID",
                other => panic!("unexpected blocked readiness {other}"),
            }),
            "{invocation}"
        );
    }
}

#[test]
fn setup_created_root_reports_missing_artifacts_without_setup_retry() {
    let repo = tempfile::tempdir().expect("tempdir");
    fs::create_dir(repo.path().join(".git")).expect("git root");

    let setup = run_in(repo.path(), &["setup"]);
    let setup_stdout = stdout(&setup);
    assert!(!setup_stdout.contains("OUTCOME: ERROR"), "{setup_stdout}");
    assert!(repo
        .path()
        .join(".handbook/repository-identity.v1")
        .is_file());
    for forbidden in [
        ".handbook/project/charter.yaml",
        ".handbook/project/context.yaml",
        ".handbook/project/environment.yaml",
        ".handbook/profile-selection.json",
    ] {
        assert!(
            !repo.path().join(forbidden).exists(),
            "unexpected {forbidden}"
        );
    }

    for command in ["inspect", "generate"] {
        let output = run_in(repo.path(), &[command]);
        assert!(!output.status.success(), "{command} unexpectedly succeeded");
        let text = stdout(&output);
        assert!(text.contains("required_path_missing"), "{text}");
        assert!(text.contains("handbook author charter"), "{text}");
        if command == "inspect" {
            assert!(text.contains("handbook author project-context"), "{text}");
        }
        assert!(!text.contains("CATEGORY: SystemRootMissing"), "{text}");
        assert!(!text.contains("handbook setup"), "{text}");
    }
}

#[test]
fn legacy_markdown_tree_still_requires_setup() {
    let repo = tempfile::tempdir().expect("tempdir");
    fs::create_dir_all(repo.path().join(".git")).expect("git root");
    fs::create_dir_all(repo.path().join(".handbook/charter")).expect("charter namespace");
    fs::create_dir_all(repo.path().join(".handbook/project_context"))
        .expect("project context namespace");
    fs::write(
        repo.path().join(".handbook/charter/CHARTER.md"),
        "legacy Charter truth",
    )
    .expect("legacy Charter");
    fs::write(
        repo.path()
            .join(".handbook/project_context/PROJECT_CONTEXT.md"),
        "retired Project Context truth",
    )
    .expect("retired Project Context");

    let inspect = run_in(repo.path(), &["inspect"]);
    assert!(!inspect.status.success(), "inspect unexpectedly succeeded");
    let text = stdout(&inspect);
    assert!(text.contains("CATEGORY: SystemRootMissing"), "{text}");
    assert!(text.contains("handbook setup"), "{text}");
    assert!(!text.contains("legacy Charter truth"), "{text}");
    assert!(!text.contains("retired Project Context truth"), "{text}");
}

#[test]
fn post_phase_2_runbook_binds_supported_disposable_journeys() {
    for required in [
        "$env:CARGO_TARGET_DIR",
        "$Handbook setup",
        "$Handbook doctor --json",
        "runtime_smoke_valid.yaml",
        "AUTHENTICATOR_UNAVAILABLE",
        "hcm_2_3_generic_custom_kind/.handbook",
        "artifact list-kinds --repository-root $GenericRepo --legacy-json",
        "inspect_reports_ready_when_required_artifacts_present",
        "generate_emits_real_packet_body_when_ready",
        "pipeline show --id pipeline.foundation_inputs",
        "pipeline resolve --id pipeline.foundation_inputs",
    ] {
        assert!(SMOKE_RUNBOOK.contains(required), "missing `{required}`");
    }
    assert!(!SMOKE_RUNBOOK.contains("AppData\\Local\\Temp"));
}
