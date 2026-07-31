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
        "artifact list-kinds --repository-root $GenericRepo --json",
        "inspect_reports_ready_when_required_artifacts_present",
        "generate_emits_real_packet_body_when_ready",
        "pipeline show --id pipeline.foundation_inputs",
        "pipeline resolve --id pipeline.foundation_inputs",
    ] {
        assert!(SMOKE_RUNBOOK.contains(required), "missing `{required}`");
    }
    assert!(!SMOKE_RUNBOOK.contains("AppData\\Local\\Temp"));
}
