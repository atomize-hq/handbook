// This integration test only uses a subset of the shared proof corpus helpers.
#[allow(dead_code)]
mod pipeline_proof_corpus_support;

use std::path::Path;
use std::process::{Command, Output, Stdio};

fn binary() -> Command {
    Command::new(env!("CARGO_BIN_EXE_handbook"))
}

fn binary_in(dir: &Path) -> Command {
    let mut cmd = binary();
    cmd.current_dir(dir);
    cmd
}

fn run_in(dir: &Path, args: &[&str]) -> Output {
    binary_in(dir)
        .args(args)
        .output()
        .unwrap_or_else(|err| panic!("run `{}`: {err}", args.join(" ")))
}

fn run_in_with_input(dir: &Path, args: &[&str], input: &str) -> Output {
    let mut child = binary_in(dir)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap_or_else(|err| panic!("spawn `{}`: {err}", args.join(" ")));
    {
        let stdin = child.stdin.as_mut().expect("child stdin");
        use std::io::Write as _;
        stdin
            .write_all(input.as_bytes())
            .unwrap_or_else(|err| panic!("write stdin for `{}`: {err}", args.join(" ")));
    }
    child
        .wait_with_output()
        .unwrap_or_else(|err| panic!("wait for `{}`: {err}", args.join(" ")))
}

fn write_file(path: &Path, contents: &[u8]) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("mkdirs");
    }
    std::fs::write(path, contents).expect("write file");
}

fn planning_ready_repo() -> (tempfile::TempDir, std::path::PathBuf) {
    pipeline_proof_corpus_support::install_committed_fixture_repo(
        "tests/fixtures/planning_ready_repo",
    )
}

fn foundation_inputs_repo() -> (tempfile::TempDir, std::path::PathBuf) {
    pipeline_proof_corpus_support::install_foundation_inputs_repo()
}

fn prepare_foundation_inputs_full_context_route_basis(root: &Path) {
    for args in [
        vec![
            "pipeline",
            "state",
            "set",
            "--id",
            "foundation_inputs",
            "--field",
            "run.runner=codex-cli",
        ],
        vec![
            "pipeline",
            "state",
            "set",
            "--id",
            "foundation_inputs",
            "--field",
            "run.profile=python-uv",
        ],
        vec![
            "pipeline",
            "state",
            "set",
            "--id",
            "foundation_inputs",
            "--field",
            "refs.charter_ref=artifacts/charter/CHARTER.md",
        ],
        vec![
            "pipeline",
            "state",
            "set",
            "--id",
            "foundation_inputs",
            "--field",
            "refs.project_context_ref=artifacts/project_context/PROJECT_CONTEXT.md",
        ],
        vec![
            "pipeline",
            "state",
            "set",
            "--id",
            "foundation_inputs",
            "--var",
            "needs_project_context=false",
        ],
        vec![
            "pipeline",
            "state",
            "set",
            "--id",
            "foundation_inputs",
            "--var",
            "charter_gaps_detected=false",
        ],
        vec!["pipeline", "resolve", "--id", "foundation_inputs"],
    ] {
        let output = run_in(root, &args);
        assert!(
            output.status.success(),
            "command should succeed: {:?}",
            args
        );
    }
}

fn install_canonical_inputs(root: &Path) {
    write_file(
        &root.join(".handbook/charter/CHARTER.md"),
        &std::fs::read(root.join("artifacts/charter/CHARTER.md")).expect("read charter fixture"),
    );
    write_file(
        &root.join(".handbook/project_context/PROJECT_CONTEXT.md"),
        &std::fs::read(root.join("artifacts/project_context/PROJECT_CONTEXT.md"))
            .expect("read project context fixture"),
    );
    write_file(
        &root.join(".handbook/feature_spec/FEATURE_SPEC.md"),
        pipeline_proof_corpus_support::read_committed_model_output("stage_10_feature_spec.md")
            .as_bytes(),
    );
}

fn seed_non_canonical_boundary_noise(root: &Path) {
    write_file(
        &root.join("artifacts/feature_spec/FEATURE_SPEC.md"),
        b"POISON derived feature spec outside canonical boundary\n",
    );
    write_file(
        &root.join("artifacts/handoff/feature_slice/poison/handoff_manifest.json"),
        br#"{"schema_version":"poison","bundle_root":"artifacts/handoff/feature_slice/poison"}"#,
    );
    write_file(
        &root.join("artifacts/handoff/feature_slice/poison/read_allowlist.json"),
        br#"{"allow_read_paths":["../../../../core/stages/10_feature_spec.md"]}"#,
    );
    write_file(
        &root.join("core/library/feature_spec/FEATURE_SPEC.md.tmpl"),
        b"POISON canonical-looking template outside planning packet trust surface\n",
    );
    write_file(
        &root.join("core/runners/codex-cli.md"),
        b"POISON runner outside planning packet trust surface\n",
    );
}

#[test]
fn pipeline_handoff_emit_refuses_when_feature_spec_artifact_is_missing() {
    let (_dir, root) = foundation_inputs_repo();
    prepare_foundation_inputs_full_context_route_basis(root.as_path());

    let output = run_in(
        root.as_path(),
        &[
            "pipeline",
            "handoff",
            "emit",
            "--id",
            "foundation_inputs",
            "--consumer",
            "feature-slice-decomposer",
        ],
    );
    assert!(!output.status.success(), "handoff emit should refuse");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert!(stdout.contains("OUTCOME: REFUSED"), "{stdout}");
    assert!(
        stdout.contains("PIPELINE: pipeline.foundation_inputs"),
        "{stdout}"
    );
    assert!(
        stdout.contains("CONSUMER: feature-slice-decomposer"),
        "{stdout}"
    );
    assert!(
        stdout.contains(
            "REASON: missing_required_input: required handoff source `artifacts/feature_spec/FEATURE_SPEC.md` is unavailable"
        ),
        "{stdout}"
    );
    assert!(
        stdout.contains(
            "NEXT SAFE ACTION: capture `stage.10_feature_spec` output before retrying `pipeline handoff emit`"
        ),
        "{stdout}"
    );
}

#[test]
fn pipeline_handoff_emit_refuses_when_stage_10_capture_provenance_is_stale() {
    let (_dir, root) = foundation_inputs_repo();
    prepare_foundation_inputs_full_context_route_basis(root.as_path());
    install_canonical_inputs(root.as_path());

    let capture = run_in_with_input(
        root.as_path(),
        &[
            "pipeline",
            "capture",
            "--id",
            "foundation_inputs",
            "--stage",
            "stage.10_feature_spec",
        ],
        &pipeline_proof_corpus_support::read_committed_model_output("stage_10_feature_spec.md"),
    );
    assert!(capture.status.success(), "stage 10 capture should succeed");

    write_file(
        &root.join("artifacts/foundation/FOUNDATION_STRATEGY.md"),
        b"# drifted foundation strategy\n",
    );

    let output = run_in(
        root.as_path(),
        &[
            "pipeline",
            "handoff",
            "emit",
            "--id",
            "foundation_inputs",
            "--consumer",
            "feature-slice-decomposer",
        ],
    );
    assert!(!output.status.success(), "handoff emit should refuse");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert!(stdout.contains("OUTCOME: REFUSED"), "{stdout}");
    assert!(stdout.contains("REASON: invalid_provenance:"), "{stdout}");
    assert!(
        stdout.contains("stage-10 capture provenance") || stdout.contains("payload_sha256"),
        "{stdout}"
    );
    assert!(
        stdout.contains(
            "NEXT SAFE ACTION: recapture `stage.10_feature_spec` from the current compile payload before retrying `pipeline handoff emit`"
        ),
        "{stdout}"
    );
}

#[test]
fn generate_ignores_non_system_handoff_and_derived_files() {
    let (_dir, root) = planning_ready_repo();

    let baseline = run_in(root.as_path(), &["generate"]);
    assert!(
        !baseline.status.success(),
        "legacy-only baseline generate should refuse"
    );
    let baseline_stdout = String::from_utf8(baseline.stdout).expect("baseline stdout is utf-8");

    seed_non_canonical_boundary_noise(root.as_path());

    let output = run_in(root.as_path(), &["generate"]);
    assert!(
        !output.status.success(),
        "generate should retain the selected-Charter refusal"
    );
    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");

    assert_eq!(stdout, baseline_stdout);
    assert!(
        stdout.contains(".handbook/project/charter.yaml"),
        "{stdout}"
    );
    assert!(!stdout.contains(".handbook/charter/CHARTER.md"), "{stdout}");
    assert!(
        !stdout.contains("artifacts/feature_spec/FEATURE_SPEC.md"),
        "{stdout}"
    );
    assert!(
        !stdout.contains("artifacts/handoff/feature_slice/poison"),
        "{stdout}"
    );
    assert!(!stdout.contains("POISON"), "{stdout}");
}

#[test]
fn inspect_ignores_non_system_handoff_and_derived_files() {
    let (_dir, root) = planning_ready_repo();

    let baseline = run_in(root.as_path(), &["inspect"]);
    assert!(
        !baseline.status.success(),
        "legacy-only baseline inspect should refuse"
    );
    let baseline_stdout = String::from_utf8(baseline.stdout).expect("baseline stdout is utf-8");

    seed_non_canonical_boundary_noise(root.as_path());

    let output = run_in(root.as_path(), &["inspect"]);
    assert!(
        !output.status.success(),
        "inspect should retain the selected-Charter refusal"
    );
    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");

    assert_eq!(stdout, baseline_stdout);
    assert!(
        stdout.contains(".handbook/project/charter.yaml"),
        "{stdout}"
    );
    assert!(!stdout.contains(".handbook/charter/CHARTER.md"), "{stdout}");
    assert!(
        stdout.contains(".handbook/feature_spec/FEATURE_SPEC.md"),
        "{stdout}"
    );
    assert!(
        !stdout.contains("artifacts/feature_spec/FEATURE_SPEC.md"),
        "{stdout}"
    );
    assert!(
        !stdout.contains("artifacts/handoff/feature_slice/poison"),
        "{stdout}"
    );
    assert!(!stdout.contains("POISON"), "{stdout}");
}
