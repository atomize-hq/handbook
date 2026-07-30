#[path = "../../engine/tests/support/hcm_2_2_committed_charter.rs"]
mod hcm_2_2_committed_charter;
mod pipeline_proof_corpus_support;

use std::collections::BTreeSet;
use std::fs;
use std::process::{Command, Output};

const FIXED_NOW_UTC: &str = "2026-01-28T18:35:10Z";
const FOUNDATION_FLOW_DEMO_HAPPY_PATH_FEATURE_ID: &str = "fs-m4-foundation-journey-2026-04";
const CANONICAL_CHARTER_YAML: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/specs/handbook-contract-membrane/slices/HCM-2.2/contracts/canonical-charter-boundary-v1.1.yaml"
));
#[derive(Debug)]
struct ConsumerHarnessRun {
    feature_id: String,
    bundle_root: String,
    plan_path: String,
    plan_body: String,
    bundle_reads: Vec<String>,
    repo_rereads: Vec<String>,
}

#[derive(Debug)]
struct PlanningInputs {
    feature_id: String,
    bundle_root: String,
    pipeline_id: String,
    consumer_id: String,
    repo_reread_fallback: String,
    summary: String,
    goals: Vec<String>,
    acceptance_criteria: Vec<String>,
    strategy_pillars: Vec<String>,
    journey_flow: Vec<String>,
    mandatory_gates: Vec<String>,
}

fn binary() -> Command {
    Command::new(env!("CARGO_BIN_EXE_handbook"))
}

fn binary_in(dir: &std::path::Path) -> Command {
    let mut cmd = binary();
    cmd.current_dir(dir);
    cmd
}

fn run_in(dir: &std::path::Path, args: &[&str]) -> Output {
    binary_in(dir)
        .args(args)
        .output()
        .unwrap_or_else(|err| panic!("run `{}`: {err}", args.join(" ")))
}

fn run_in_with_env(dir: &std::path::Path, args: &[&str], envs: &[(&str, &str)]) -> Output {
    let mut cmd = binary_in(dir);
    cmd.args(args);
    for (key, value) in envs {
        cmd.env(key, value);
    }
    cmd.output()
        .unwrap_or_else(|err| panic!("run `{}` with env: {err}", args.join(" ")))
}

fn run_in_with_input(dir: &std::path::Path, args: &[&str], input: &str) -> Output {
    use std::io::Write;
    use std::process::Stdio;

    let mut child = binary_in(dir)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap_or_else(|err| panic!("spawn `{}`: {err}", args.join(" ")));

    {
        let stdin = child.stdin.as_mut().expect("stdin");
        stdin
            .write_all(input.as_bytes())
            .unwrap_or_else(|err| panic!("write stdin for `{}`: {err}", args.join(" ")));
    }

    child
        .wait_with_output()
        .unwrap_or_else(|err| panic!("wait `{}`: {err}", args.join(" ")))
}

fn workspace_root() -> std::path::PathBuf {
    let start = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    for ancestor in start.ancestors() {
        let cargo_toml = ancestor.join("Cargo.toml");
        if !cargo_toml.is_file() {
            continue;
        }
        let Ok(contents) = std::fs::read_to_string(&cargo_toml) else {
            continue;
        };
        if contents.contains("[workspace]") {
            return ancestor.to_path_buf();
        }
    }

    panic!(
        "failed to locate workspace root from CARGO_MANIFEST_DIR={}",
        env!("CARGO_MANIFEST_DIR")
    );
}

fn copy_tree(source: &std::path::Path, target: &std::path::Path) {
    std::fs::create_dir_all(target)
        .unwrap_or_else(|err| panic!("mkdir {}: {err}", target.display()));

    for entry in std::fs::read_dir(source)
        .unwrap_or_else(|err| panic!("read_dir {}: {err}", source.display()))
    {
        let entry =
            entry.unwrap_or_else(|err| panic!("dir entry under {}: {err}", source.display()));
        let source_path = entry.path();
        let target_path = target.join(entry.file_name());
        let file_type = entry
            .file_type()
            .unwrap_or_else(|err| panic!("file_type {}: {err}", source_path.display()));

        if file_type.is_dir() {
            copy_tree(&source_path, &target_path);
        } else if file_type.is_file() {
            if let Some(parent) = target_path.parent() {
                std::fs::create_dir_all(parent)
                    .unwrap_or_else(|err| panic!("mkdir {}: {err}", parent.display()));
            }
            std::fs::copy(&source_path, &target_path).unwrap_or_else(|err| {
                panic!(
                    "copy {} -> {}: {err}",
                    source_path.display(),
                    target_path.display()
                )
            });
        }
    }
}

#[cfg(any())]
fn tracked_workspace_checkout() -> (tempfile::TempDir, std::path::PathBuf) {
    let dir = tempfile::tempdir().expect("tempdir");
    let checkout_root = dir.path().to_path_buf();
    let workspace = workspace_root();

    let output = Command::new("git")
        .arg("-C")
        .arg(&workspace)
        .arg("ls-files")
        .arg("-z")
        .output()
        .unwrap_or_else(|err| panic!("git ls-files: {err}"));
    assert!(
        output.status.success(),
        "git ls-files failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    for entry in output.stdout.split(|byte| *byte == 0) {
        if entry.is_empty() {
            continue;
        }
        let relative = std::str::from_utf8(entry).expect("git ls-files output must be utf-8");
        let source = workspace.join(relative);
        if !source.is_file() {
            continue;
        }
        let target = checkout_root.join(relative);
        if let Some(parent) = target.parent() {
            std::fs::create_dir_all(parent)
                .unwrap_or_else(|err| panic!("mkdir {}: {err}", parent.display()));
        }
        std::fs::copy(&source, &target).unwrap_or_else(|err| {
            panic!(
                "copy tracked file {} -> {}: {err}",
                source.display(),
                target.display()
            )
        });
    }

    (dir, checkout_root)
}

fn foundation_flow_demo_root() -> std::path::PathBuf {
    workspace_root().join("tests/fixtures/foundation_flow_demo")
}

fn foundation_flow_demo_evidence_root() -> std::path::PathBuf {
    foundation_flow_demo_root().join("evidence")
}

fn remove_stage_10_capture_outputs(root: &std::path::Path) {
    for relative_path in [
        "artifacts/work-specification/work-specification.yaml",
        "artifacts/feature_spec/FEATURE_SPEC.md",
    ] {
        match std::fs::remove_file(root.join(relative_path)) {
            Ok(()) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => panic!("remove seeded stage-10 output {relative_path}: {error}"),
        }
    }
}

fn initialize_stage_10_test_repository_identity(root: &std::path::Path) {
    handbook_engine::RepositoryInvocationIdentityServiceV1::new()
        .initialize_for_setup(root)
        .expect("initialize fresh Stage 10 test repository identity");
}

fn install_foundation_flow_demo_repo() -> (tempfile::TempDir, std::path::PathBuf) {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path().to_path_buf();
    copy_tree(&foundation_flow_demo_root().join("repo"), &root);
    initialize_stage_10_test_repository_identity(root.as_path());
    remove_stage_10_capture_outputs(root.as_path());
    (dir, root)
}

fn read_foundation_flow_demo_model_output(case: &str, filename: &str) -> String {
    std::fs::read_to_string(
        foundation_flow_demo_root()
            .join("model_outputs")
            .join(case)
            .join(filename),
    )
    .unwrap_or_else(|err| panic!("read demo model output {case}/{filename}: {err}"))
}

fn read_foundation_flow_demo_expected(case: &str, filename: &str) -> String {
    std::fs::read_to_string(
        foundation_flow_demo_root()
            .join("expected")
            .join(case)
            .join(filename),
    )
    .unwrap_or_else(|err| panic!("read demo expected output {case}/{filename}: {err}"))
}

fn read_foundation_flow_demo_evidence(filename: &str) -> String {
    std::fs::read_to_string(foundation_flow_demo_evidence_root().join(filename))
        .unwrap_or_else(|err| panic!("read demo evidence {filename}: {err}"))
}

fn foundation_flow_demo_happy_path_bundle_root() -> String {
    format!("artifacts/handoff/feature_slice/{FOUNDATION_FLOW_DEMO_HAPPY_PATH_FEATURE_ID}")
}

fn canonical_repo_root(path: &std::path::Path) -> std::path::PathBuf {
    std::fs::canonicalize(path)
        .unwrap_or_else(|err| panic!("canonicalize {}: {err}", path.display()))
}

fn write_file(path: &std::path::Path, contents: &[u8]) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("mkdirs");
    }
    std::fs::write(path, contents).expect("write");
}

fn write_minimal_runner_and_profile(root: &std::path::Path) {
    write_file(
        &root.join("core/runners/codex-cli.md"),
        b"# Runner: codex-cli\n\nThis runner assumes the agent can run commands.\n",
    );
    write_file(
        &root.join("core/profiles/python-uv/profile.yaml"),
        b"kind: profile\nid: python-uv\nversion: 0.1.0\ntitle: \"Python (uv)\"\ndescription: test profile\ncompatibility:\n  languages: [\"python\"]\n  package_managers: [\"uv\"]\n  runners: [\"codex-cli\"]\nproject_defaults:\n  python_requires: \">=3.11\"\n  code_dirs: [\"src\"]\n  test_dirs: [\"tests\"]\n  config_files: []\ngates:\n  required: [\"tests\"]\n  optional: []\nevidence:\n  required_fields: [\"cmd\", \"exit\", \"tail\", \"timestamp\"]\n  tail_lines: 80\n",
    );
    write_file(
        &root.join("core/profiles/python-uv/commands.yaml"),
        b"commands:\n  tests: \"pytest -q\"\n",
    );
    write_file(
        &root.join("core/profiles/python-uv/conventions.md"),
        b"# Test profile conventions\n",
    );
}

fn planning_ready_repo() -> (tempfile::TempDir, std::path::PathBuf) {
    let (dir, root) = pipeline_proof_corpus_support::install_committed_fixture_repo(
        "tests/fixtures/planning_ready_repo",
    );
    write_valid_selected_charter(&root);
    write_valid_selected_project_context(&root);
    (dir, root)
}

fn planning_ready_repo_with_nested_cwd() -> (tempfile::TempDir, std::path::PathBuf) {
    let (dir, root, nested) =
        pipeline_proof_corpus_support::install_committed_fixture_checkout_with_nested_cwd(
            "tests/fixtures/planning_ready_repo",
        );
    write_valid_selected_charter(&root);
    write_valid_selected_project_context(&root);
    (dir, nested)
}

fn nested_git_repo_with_nested_cwd() -> (tempfile::TempDir, std::path::PathBuf) {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    std::fs::create_dir_all(root.join(".git")).expect("git root");
    let nested = root.join("work/nested");
    std::fs::create_dir_all(&nested).expect("nested cwd");

    (dir, nested)
}

fn execution_demo_repo_with_nested_cwd() -> (tempfile::TempDir, std::path::PathBuf) {
    let (dir, root, nested) =
        pipeline_proof_corpus_support::install_committed_fixture_under_repo_with_nested_cwd(
            "tests/fixtures/execution_demo/basic",
            "tests/fixtures/execution_demo/basic",
        );
    let fixture_root = root.join("tests/fixtures/execution_demo/basic");
    write_valid_selected_charter(&fixture_root);
    write_valid_selected_project_context(&fixture_root);
    (dir, nested)
}

fn execution_demo_repo() -> (tempfile::TempDir, std::path::PathBuf) {
    let (dir, root, _nested) =
        pipeline_proof_corpus_support::install_committed_fixture_under_repo_with_nested_cwd(
            "tests/fixtures/execution_demo/basic",
            "tests/fixtures/execution_demo/basic",
        );
    let fixture_root = root.join("tests/fixtures/execution_demo/basic");
    write_valid_selected_charter(&fixture_root);
    write_valid_selected_project_context(&fixture_root);
    (dir, root)
}

fn committed_execution_demo_fixture_dir_under_temp_git_root(
) -> (tempfile::TempDir, std::path::PathBuf) {
    let (dir, root, _nested) =
        pipeline_proof_corpus_support::install_committed_fixture_under_repo_with_nested_cwd(
            "tests/fixtures/execution_demo/basic",
            "tests/fixtures/execution_demo/basic",
        );
    let fixture_dir = root.join("tests/fixtures/execution_demo/basic");
    write_valid_selected_charter(&fixture_dir);
    write_valid_selected_project_context(&fixture_dir);
    (dir, fixture_dir)
}

fn repair_to_ready(root: &std::path::Path) {
    write_valid_selected_charter(root);
    write_valid_selected_project_context(root);
    write_file(
        &root.join(".handbook/feature_spec/FEATURE_SPEC.md"),
        b"feature",
    );
}

#[cfg(any())]
fn starter_template_bytes_for_path(path: &str) -> &'static [u8] {
    unreachable!("retired pre-membrane setup fixture: {path}")
}

fn partial_system_repo() -> tempfile::TempDir {
    let dir = tempfile::tempdir().expect("tempdir");
    write_valid_selected_charter(dir.path());
    write_valid_selected_project_context(dir.path());
    dir
}

#[cfg(any())]
fn valid_charter_markdown() -> &'static str {
    "# Engineering Charter — Handbook

## What this is
Body.

## How to use this charter
Use it.

## Rubric: 1–5 rigor levels
Levels.

## Project baseline posture
Baseline.

## Domains / areas (optional overrides)
None.

## Posture at a glance (quick scan)
Snapshot.

## Dimensions (details + guardrails)
Details.

## Cross-cutting red lines (global non-negotiables)
- Keep trust boundaries intact.

## Exceptions / overrides process
- **Approvers:** project_owner
- **Record location:** docs/exceptions.md
- **Minimum required fields:**
  - what
  - why
  - scope
  - risk
  - owner
  - expiry_or_revisit_date

## Debt tracking expectations
Tracked in issues.

## Decision Records (ADRs): how to use this charter
Use ADRs.

## Review & updates
Review monthly.
"
}

fn valid_project_context_markdown() -> &'static str {
    "# Project Context — Handbook

> **File:** `PROJECT_CONTEXT.md`
> **Created (UTC):** 2026-04-21T00:00:00Z
> **Owner:** project-owner
> **Team:** handbook-team
> **Repo / Project:** /tmp/handbook
> **Charter Ref:** .handbook/charter/CHARTER.md

## What this is
Project reality.

## How to use this
Use this document to ground planning in reality.

## 0) Project Summary (factual, 3–6 bullets)
- Summary.

## 1) Operational Reality (the most important section)
- Operations.

## 2) Project Classification Implications (planning guardrails)
- Guardrails.

## 3) System Boundaries (what we own vs integrate with)
### What we own
- Canonical `.handbook/` truth.
### What we do NOT own (but may depend on)
- External delivery systems.

## 4) Integrations & Contracts (top 1–5)
- Integrations.

## 5) Environments & Delivery
- Delivery.

## 6) Data Reality
- Data.

## 7) Repo / Codebase Reality (brownfield-friendly, but safe for greenfield)
- Codebase.

## 8) Constraints
- Constraints.

## 9) Known Unknowns (explicitly tracked)
- Unknowns.

## 10) Update Triggers
- Update when reality changes.
"
}

fn valid_selected_project_context_yaml() -> &'static str {
    concat!(
        "schema_id: \"handbook.artifact.project-context\"\n",
        "schema_version: \"1.0\"\n",
        "record_id: \"handbook.project-context\"\n",
        "summary: \"Project reality.\"\n",
        "system_boundaries:\n",
        "  - \"Canonical handbook truth\"\n",
        "ownership:\n",
        "  - \"handbook-team\"\n",
        "authoritative_references:\n",
        "  - \"handbook.charter@1.0.0\"\n",
        "known_unknowns:\n",
        "  - \"None\"\n",
    )
}

fn write_valid_selected_project_context(root: &std::path::Path) {
    write_file(
        &root.join(".handbook/project/context.yaml"),
        valid_selected_project_context_yaml().as_bytes(),
    );
}

fn write_valid_selected_charter(root: &std::path::Path) {
    hcm_2_2_committed_charter::promote_committed_charter(
        root,
        CANONICAL_CHARTER_YAML.as_bytes(),
        "cli-surface-ready-charter",
    );
}

fn profile_ready_repo() -> tempfile::TempDir {
    let dir = tempfile::tempdir().expect("tempdir");
    std::fs::create_dir_all(dir.path().join(".git")).expect("git root");
    write_valid_selected_charter(dir.path());
    write_valid_selected_project_context(dir.path());
    write_file(
        &dir.path().join(".handbook/project/environment.yaml"),
        br#"authoritative_references: []
environments:
  -
    capabilities:
      - "rust.stable"
      - "filesystem.workspace-write"
    description: "Local development and focused tests."
    environment_id: "local-dev"
known_unknowns: []
record_id: "handbook.environment-context"
schema_id: "handbook.artifact.environment-context"
schema_version: "1.1"
"#,
    );
    dir
}

fn legacy_placeholder_project_context_markdown() -> String {
    valid_project_context_markdown()
        .replace("> **Owner:** project-owner", "> **Owner:** unknown-owner")
        .replace("> **Team:** handbook-team", "> **Team:** project-team")
        .replace(
            "- Operations.",
            "- Unknown from local repo inspection; confirm before planning live changes.",
        )
}

fn malformed_optional_project_context_repo() -> tempfile::TempDir {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    repair_to_ready(root);
    std::fs::remove_file(root.join(".handbook/project/context.yaml"))
        .expect("remove project_context file");
    std::fs::create_dir_all(root.join(".handbook/project/context.yaml"))
        .expect("project_context directory");

    dir
}

#[cfg(any())]
fn nested_git_repo_inside_managed_parent_with_nested_cwd() -> (tempfile::TempDir, std::path::PathBuf)
{
    let dir = tempfile::tempdir().expect("tempdir");
    let parent = dir.path().join("parent");
    let child = parent.join("child");
    let nested = child.join("work/nested");

    write_file(
        &parent.join(".handbook/charter/CHARTER.md"),
        b"parent charter",
    );
    write_file(
        &parent.join(".handbook/feature_spec/FEATURE_SPEC.md"),
        b"parent feature",
    );
    std::fs::create_dir_all(child.join(".git")).expect("child git root");
    std::fs::create_dir_all(&nested).expect("nested cwd");

    (dir, nested)
}

fn activation_drift_pipeline_repo() -> (tempfile::TempDir, std::path::PathBuf) {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path().to_path_buf();

    write_minimal_runner_and_profile(&root);
    write_file(
        &root.join("core/stages/00_base.md"),
        b"---\nkind: stage\nid: stage.00_base\nversion: 0.1.0\ntitle: Base\ndescription: base\nactivation:\n  when:\n    any:\n      - variables.needs_project_context == true\n---\n# base\n",
    );
    write_file(
        &root.join("core/pipelines/drift.yaml"),
        b"---\nkind: pipeline\nid: pipeline.drift\nversion: 0.1.0\ntitle: Drift\ndescription: drift\n---\ndefaults:\n  runner: codex-cli\n  profile: python-uv\n  enable_complexity: false\nstages:\n  - id: stage.00_base\n    file: core/stages/00_base.md\n",
    );

    (dir, root)
}

fn invalid_pipeline_id_repo() -> (tempfile::TempDir, std::path::PathBuf) {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path().to_path_buf();

    write_minimal_runner_and_profile(&root);
    write_file(
        &root.join("core/stages/00_base.md"),
        b"---\nkind: stage\nid: stage.00_base\nversion: 0.1.0\ntitle: Base\ndescription: base\n---\n# base\n",
    );
    write_file(
        &root.join("core/pipelines/foundation.yaml"),
        b"---\nkind: pipeline\nid: pipeline.foundation\nversion: 0.1.0\ntitle: Foundation\ndescription: foundation\n---\ndefaults:\n  runner: codex-cli\n  profile: python-uv\n  enable_complexity: false\nstages:\n  - id: stage.00_base\n    file: core/stages/00_base.md\n",
    );
    write_file(
        &root.join("core/pipelines/bad-id.yaml"),
        b"---\nkind: pipeline\nid: pipeline.bad/path\nversion: 0.1.0\ntitle: Bad Id\ndescription: bad\n---\ndefaults:\n  runner: codex-cli\n  profile: python-uv\n  enable_complexity: false\nstages:\n  - id: stage.00_base\n    file: core/stages/00_base.md\n",
    );

    (dir, root)
}

fn unused_bad_stage_repo() -> (tempfile::TempDir, std::path::PathBuf) {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path().to_path_buf();

    write_file(
        &root.join("core/stages/00_base.md"),
        b"---\nkind: stage\nid: stage.00_base\nversion: 0.1.0\ntitle: Base\ndescription: base\n---\n# base\n",
    );
    write_file(
        &root.join("core/stages/99_bad_unused.md"),
        b"---\nkind: nonsense\nid: stage.bad_unused\nversion: 0.1.0\ntitle: Bad Unused Stage\ndescription: bad\n---\n# bad\n",
    );
    write_file(
        &root.join("core/pipelines/foundation.yaml"),
        b"---\nkind: pipeline\nid: pipeline.foundation\nversion: 0.1.0\ntitle: Foundation\ndescription: foundation\n---\ndefaults:\n  runner: codex-cli\n  profile: python-uv\n  enable_complexity: false\nstages:\n  - id: stage.00_base\n    file: core/stages/00_base.md\n",
    );

    (dir, root)
}

fn selected_broken_pipeline_repo() -> (tempfile::TempDir, std::path::PathBuf) {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path().to_path_buf();

    write_file(
        &root.join("core/stages/00_base.md"),
        b"---\nkind: stage\nid: stage.00_base\nversion: 0.1.0\ntitle: Base\ndescription: base\n---\n# base\n",
    );
    write_file(
        &root.join("core/stages/bad.md"),
        b"---\nkind: nonsense\nid: stage.bad\nversion: 0.1.0\ntitle: Bad\ndescription: bad\n---\n# bad\n",
    );
    write_file(
        &root.join("core/pipelines/foundation.yaml"),
        b"---\nkind: pipeline\nid: pipeline.foundation\nversion: 0.1.0\ntitle: Foundation\ndescription: foundation\n---\ndefaults:\n  runner: codex-cli\n  profile: python-uv\n  enable_complexity: false\nstages:\n  - id: stage.00_base\n    file: core/stages/00_base.md\n",
    );
    write_file(
        &root.join("core/pipelines/broken.yaml"),
        b"---\nkind: pipeline\nid: pipeline.broken\nversion: 0.1.0\ntitle: Broken\ndescription: broken\n---\ndefaults:\n  runner: codex-cli\n  profile: python-uv\n  enable_complexity: false\nstages:\n  - id: stage.bad\n    file: core/stages/bad.md\n",
    );

    (dir, root)
}

fn write_incomplete_profile_pack(root: &std::path::Path, profile_id: &str) {
    write_file(
        &root.join(format!("core/profiles/{profile_id}/profile.yaml")),
        format!("kind: profile\nid: {profile_id}\n").as_bytes(),
    );
}

fn set_foundation_inputs_default_profile(root: &std::path::Path, profile_id: &str) {
    let pipeline_path = root.join("core/pipelines/foundation_inputs.yaml");
    let contents = std::fs::read_to_string(&pipeline_path).expect("read foundation pipeline");
    std::fs::write(
        &pipeline_path,
        contents.replace(
            "defaults:\n  runner: codex-cli\n  profile: python-uv\n",
            &format!("defaults:\n  runner: codex-cli\n  profile: {profile_id}\n"),
        ),
    )
    .expect("write foundation pipeline");
}

fn prepare_foundation_inputs_compile_ready_route_basis(root: &std::path::Path) {
    for args in [
        vec![
            "pipeline",
            "state",
            "set",
            "--id",
            "foundation_inputs",
            "--var",
            "needs_project_context=true",
        ],
        vec![
            "pipeline",
            "state",
            "set",
            "--id",
            "foundation_inputs",
            "--var",
            "charter_gaps_detected=true",
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

fn prepare_foundation_inputs_full_context_route_basis(root: &std::path::Path) {
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

fn prepare_stage_05_capture_ready_route_basis(root: &std::path::Path) {
    let output = run_in(root, &["pipeline", "resolve", "--id", "foundation_inputs"]);
    assert!(output.status.success(), "resolve should succeed");
}

fn prepare_stage_04_capture_ready_route_basis(root: &std::path::Path) {
    prepare_stage_05_capture_ready_route_basis(root);
}

fn prepare_stage_06_capture_ready_route_basis(root: &std::path::Path) {
    for args in [
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
            "--var",
            "needs_project_context=true",
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

fn prepare_stage_07_capture_ready_route_basis(root: &std::path::Path) {
    for args in [
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

fn prepare_stage_10_capture_ready_route_basis(root: &std::path::Path) {
    prepare_foundation_inputs_full_context_route_basis(root);
    initialize_stage_10_test_repository_identity(root);
    remove_stage_10_capture_outputs(root);
}

fn stage_04_capture_input(root: &std::path::Path) -> String {
    std::fs::read_to_string(root.join("artifacts/charter/CHARTER_INPUTS.yaml"))
        .expect("stage 04 input")
}

fn stage_05_capture_input(root: &std::path::Path) -> String {
    std::fs::read_to_string(root.join("artifacts/charter/CHARTER.md")).expect("stage 05 input")
}

fn stage_06_capture_input(root: &std::path::Path) -> String {
    std::fs::read_to_string(root.join("artifacts/project_context/PROJECT_CONTEXT.md"))
        .expect("stage 06 input")
}

fn stage_07_capture_input(root: &std::path::Path) -> String {
    let outputs = [
        "artifacts/foundation/FOUNDATION_STRATEGY.md",
        "artifacts/foundation/TECH_ARCH_BRIEF.md",
        "artifacts/foundation/TEST_STRATEGY_BRIEF.md",
        "artifacts/foundation/QUALITY_GATES_SPEC.md",
        "artifacts/foundation/quality_gates.yaml",
    ];
    let mut out = String::new();
    for path in outputs {
        out.push_str(&format!("--- FILE: {path} ---\n"));
        out.push_str(&std::fs::read_to_string(root.join(path)).expect("stage 07 input"));
        out.push('\n');
    }
    out
}

fn stage_10_compile_payload(root: &std::path::Path) -> String {
    let output = run_in_with_env(
        root,
        &[
            "pipeline",
            "compile",
            "--id",
            "foundation_inputs",
            "--stage",
            "stage.10_feature_spec",
        ],
        &[(
            handbook_pipeline::pipeline_compile::PIPELINE_COMPILE_NOW_UTC_ENV_VAR,
            FIXED_NOW_UTC,
        )],
    );
    assert!(
        output.status.success(),
        "stage-10 compile should succeed before capture"
    );
    let stdout = String::from_utf8(output.stdout).expect("compile stdout is utf-8");
    let trimmed = stdout.trim_end_matches('\n');
    format!("{trimmed}\n")
}

fn stage_10_completed_feature_spec_input() -> String {
    pipeline_proof_corpus_support::read_committed_model_output("stage_10_feature_spec.md")
}

fn normalize_capture_id(output: &str) -> String {
    output
        .lines()
        .map(|line| {
            if line.starts_with("CAPTURE ID: ") {
                "CAPTURE ID: {{CAPTURE_ID}}".to_string()
            } else if let Some(prefix) = line
                .strip_prefix("NEXT SAFE ACTION: run `handbook pipeline capture apply --capture-id ")
            {
                let _ = prefix;
                "NEXT SAFE ACTION: run `handbook pipeline capture apply --capture-id {{CAPTURE_ID}}`"
                    .to_string()
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn normalize_evidence_output(output: &str, repo_root: &std::path::Path) -> String {
    pipeline_proof_corpus_support::normalize_output_for_proof(
        &normalize_capture_id(output),
        repo_root,
        &[],
    )
}

fn append_evidence_step(transcript: &mut String, command: &str, stdout: &str) {
    if !transcript.is_empty() {
        transcript.push('\n');
    }
    transcript.push_str("$ ");
    transcript.push_str(command);
    transcript.push('\n');
    transcript.push_str(stdout.trim_end());
    transcript.push('\n');
}

fn record_evidence_step(
    transcript: &mut String,
    repo_root: &std::path::Path,
    command: &str,
    output: Output,
) {
    assert!(output.status.success(), "command should succeed: {command}");
    let stdout = String::from_utf8(output.stdout).expect("command stdout is utf-8");
    append_evidence_step(
        transcript,
        command,
        &normalize_evidence_output(&stdout, repo_root),
    );
}

fn record_refused_evidence_step(
    transcript: &mut String,
    repo_root: &std::path::Path,
    command: &str,
    output: Output,
) {
    assert!(!output.status.success(), "command should refuse: {command}");
    let stdout = String::from_utf8(output.stdout).expect("command stdout is utf-8");
    append_evidence_step(
        transcript,
        command,
        &normalize_evidence_output(&stdout, repo_root),
    );
}

fn record_virtual_evidence_step(transcript: &mut String, command: &str, stdout: &str) {
    append_evidence_step(transcript, command, stdout);
}

fn read_bundle_manifest_from_disk(
    repo_root: &std::path::Path,
    bundle_root: &str,
) -> handbook_pipeline::pipeline_handoff::PipelineHandoffManifest {
    handbook_pipeline::pipeline_handoff::validate_pipeline_handoff_bundle(repo_root, bundle_root)
        .unwrap_or_else(|err| panic!("validate handoff bundle `{bundle_root}`: {}", err.summary))
        .manifest
}

fn read_bundle_allowlist_from_disk(
    repo_root: &std::path::Path,
    bundle_root: &str,
) -> handbook_pipeline::pipeline_handoff::PipelineHandoffReadAllowlist {
    handbook_pipeline::pipeline_handoff::validate_pipeline_handoff_bundle(repo_root, bundle_root)
        .unwrap_or_else(|err| panic!("validate handoff bundle `{bundle_root}`: {}", err.summary))
        .read_allowlist
}

fn read_bundle_text(
    repo_root: &std::path::Path,
    bundle_root: &str,
    allow_read_paths: &BTreeSet<String>,
    bundle_reads: &mut Vec<String>,
    relative_path: &str,
) -> String {
    assert!(
        allow_read_paths.contains(relative_path),
        "bundle path `{relative_path}` is not allowlisted"
    );
    let path = repo_root.join(bundle_root).join(relative_path);
    let body = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("read bundle file {}: {err}", path.display()));
    bundle_reads.push(relative_path.to_string());
    body
}

fn read_repo_text(
    repo_root: &std::path::Path,
    repo_rereads: &mut Vec<String>,
    relative_path: &str,
) -> String {
    let path = repo_root.join(relative_path);
    let body = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("read repo file {}: {err}", path.display()));
    repo_rereads.push(relative_path.to_string());
    body
}

fn bundle_path_for_source_path(
    manifest: &handbook_pipeline::pipeline_handoff::PipelineHandoffManifest,
    source_path: &str,
) -> String {
    manifest
        .inputs
        .iter()
        .find(|input| input.source_path == source_path)
        .map(|input| input.bundle_path.clone())
        .unwrap_or_else(|| panic!("missing handoff input for `{source_path}`"))
}

fn markdown_section_body(document: &str, heading: &str) -> String {
    let lines = document.lines().collect::<Vec<_>>();
    let start = lines
        .iter()
        .position(|line| *line == heading)
        .unwrap_or_else(|| panic!("missing heading `{heading}`"));
    let level = heading.chars().take_while(|ch| *ch == '#').count();
    let end = lines[start + 1..]
        .iter()
        .position(|line| {
            let trimmed = line.trim_start();
            if !trimmed.starts_with('#') {
                return false;
            }
            trimmed.chars().take_while(|ch| *ch == '#').count() <= level
        })
        .map(|offset| start + 1 + offset)
        .unwrap_or(lines.len());
    lines[start + 1..end].join("\n")
}

fn markdown_bullet_lines(section: &str) -> Vec<String> {
    section
        .lines()
        .filter_map(|line| line.trim_start().strip_prefix("- "))
        .map(|line| line.trim().to_string())
        .collect()
}

fn markdown_numbered_lines(section: &str) -> Vec<String> {
    section
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim_start();
            let split = trimmed.split_once(". ")?;
            if split.0.chars().all(|ch| ch.is_ascii_digit()) {
                Some(split.1.trim().to_string())
            } else {
                None
            }
        })
        .collect()
}

fn planning_input_line<'a>(lines: &'a [String], prefix: &str) -> &'a str {
    lines
        .iter()
        .find(|line| line.starts_with(prefix))
        .map(String::as_str)
        .unwrap_or_else(|| panic!("missing line with prefix `{prefix}`"))
}

fn collect_repo_reread_planning_inputs(
    repo_root: &std::path::Path,
    bundle_root: &str,
    bundle_reads: &mut Vec<String>,
    repo_rereads: &mut Vec<String>,
) -> PlanningInputs {
    let manifest = read_bundle_manifest_from_disk(repo_root, bundle_root);
    let allowlist = read_bundle_allowlist_from_disk(repo_root, bundle_root);
    let allow_read_paths = allowlist
        .allow_read_paths
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();

    let _ = read_bundle_text(
        repo_root,
        bundle_root,
        &allow_read_paths,
        bundle_reads,
        "handoff_manifest.json",
    );
    let allowlist_body = read_bundle_text(
        repo_root,
        bundle_root,
        &allow_read_paths,
        bundle_reads,
        "read_allowlist.json",
    );
    assert!(
        allowlist_body.contains(&format!(r#""consumer_id": "{}""#, allowlist.consumer_id)),
        "allowlist bundle text should name the consumer id"
    );
    let trust_matrix = read_bundle_text(
        repo_root,
        bundle_root,
        &allow_read_paths,
        bundle_reads,
        "trust_matrix.md",
    );
    assert!(
        trust_matrix.contains("- repo reread fallback: disabled"),
        "trust matrix should keep repo reread fallback disabled"
    );

    let work_specification = read_repo_text(
        repo_root,
        repo_rereads,
        "artifacts/work-specification/work-specification.yaml",
    );
    let foundation_strategy = read_repo_text(
        repo_root,
        repo_rereads,
        "artifacts/foundation/FOUNDATION_STRATEGY.md",
    );
    let tech_arch_brief = read_repo_text(
        repo_root,
        repo_rereads,
        "artifacts/foundation/TECH_ARCH_BRIEF.md",
    );
    let quality_gates_spec = read_repo_text(
        repo_root,
        repo_rereads,
        "artifacts/foundation/QUALITY_GATES_SPEC.md",
    );
    let _ = read_repo_text(
        repo_root,
        repo_rereads,
        "core/pipelines/foundation_inputs.yaml",
    );
    let _ = read_repo_text(repo_root, repo_rereads, "core/stages/10_feature_spec.md");

    build_planning_inputs(
        &manifest,
        &work_specification,
        &foundation_strategy,
        &tech_arch_brief,
        &quality_gates_spec,
    )
}

fn build_planning_inputs(
    manifest: &handbook_pipeline::pipeline_handoff::PipelineHandoffManifest,
    work_specification: &str,
    foundation_strategy: &str,
    tech_arch_brief: &str,
    quality_gates_spec: &str,
) -> PlanningInputs {
    let work_specification =
        handbook_engine::canonical_yaml::parse_canonical_yaml(work_specification.as_bytes())
            .expect("canonical Work Specification YAML");
    let work_specification = work_specification
        .as_object()
        .expect("Work Specification object");
    let summary = work_specification
        .get("objective")
        .and_then(serde_json::Value::as_str)
        .expect("Work Specification objective")
        .to_string();
    let goals = work_specification
        .get("scope")
        .and_then(serde_json::Value::as_array)
        .expect("Work Specification scope")
        .iter()
        .map(|item| {
            item.as_str()
                .expect("Work Specification scope item")
                .to_string()
        })
        .collect();
    let acceptance_criteria = work_specification
        .get("acceptance_criteria")
        .and_then(serde_json::Value::as_array)
        .expect("Work Specification acceptance criteria")
        .iter()
        .map(|item| {
            item.as_str()
                .expect("Work Specification acceptance criterion")
                .to_string()
        })
        .collect();
    let strategy_pillars = markdown_numbered_lines(&markdown_section_body(
        foundation_strategy,
        "## Strategy Pillars",
    ));
    let journey_flow =
        markdown_numbered_lines(&markdown_section_body(tech_arch_brief, "## Journey Flow"));
    let mandatory_gates = markdown_bullet_lines(&markdown_section_body(
        quality_gates_spec,
        "## Mandatory Gates",
    ));

    PlanningInputs {
        feature_id: manifest.feature_id.clone(),
        bundle_root: manifest.bundle_root.clone(),
        pipeline_id: manifest.pipeline_id.clone(),
        consumer_id: manifest.consumer_id.clone(),
        repo_reread_fallback: if manifest.fallback.repo_reread_allowed {
            "enabled".to_string()
        } else {
            "disabled".to_string()
        },
        summary,
        goals,
        acceptance_criteria,
        strategy_pillars,
        journey_flow,
        mandatory_gates,
    }
}

fn render_slice_plan(inputs: &PlanningInputs) -> String {
    let g1 = planning_input_line(&inputs.goals, "G1:");
    let g2 = planning_input_line(&inputs.goals, "G2:");
    let g3 = planning_input_line(&inputs.goals, "G3:");
    let ac001 = planning_input_line(&inputs.acceptance_criteria, "AC-001:");
    let ac002 = planning_input_line(&inputs.acceptance_criteria, "AC-002:");
    let ac003 = planning_input_line(&inputs.acceptance_criteria, "AC-003:");
    let ac004 = planning_input_line(&inputs.acceptance_criteria, "AC-004:");
    let ac005 = planning_input_line(&inputs.acceptance_criteria, "AC-005:");
    let pillar_one = inputs.strategy_pillars.first().expect("strategy pillar 1");
    let pillar_three = inputs.strategy_pillars.get(2).expect("strategy pillar 3");
    let pillar_four = inputs.strategy_pillars.get(3).expect("strategy pillar 4");
    let journey_stage_06 = inputs.journey_flow.get(3).expect("journey flow step 4");
    let journey_stage_10_compile = inputs.journey_flow.get(6).expect("journey flow step 7");
    let journey_stage_10_capture = inputs.journey_flow.get(7).expect("journey flow step 8");
    let gate_happy = inputs.mandatory_gates.first().expect("mandatory gate 1");
    let gate_skip = inputs.mandatory_gates.get(1).expect("mandatory gate 2");
    let gate_handoff = inputs.mandatory_gates.get(2).expect("mandatory gate 3");
    let gate_determinism = inputs.mandatory_gates.get(3).expect("mandatory gate 4");

    format!(
        "\
# Slice Plan
- Feature ID: `{feature_id}`
- Handoff Target: `{pipeline_id}` -> `{consumer_id}`
- Bundle Root: `{bundle_root}`
- Repo Reread Fallback: {repo_reread_fallback}

## Planning Intent
{summary}

## Evidence Pack
- Goals:
  - {g1}
  - {g2}
  - {g3}
- Strategy pillars:
  - {pillar_one}
  - {pillar_three}
  - {pillar_four}
- Journey anchors:
  - {journey_stage_06}
  - {journey_stage_10_compile}
  - {journey_stage_10_capture}
- Mandatory gates:
  - {gate_happy}
  - {gate_skip}
  - {gate_handoff}
  - {gate_determinism}

## Proposed Slices
### Slice 1: Route Journey Proof
- Objective: {g1}
- Acceptance:
  - {ac001}
  - {ac003}
- Grounding:
  - {journey_stage_06}
  - {gate_happy}
  - {gate_skip}
- Deliverable: keep the happy path and skip path evidence truthful through stage 07 before the external-model boundary.

### Slice 2: Stage-10 Handoff Boundary
- Objective: {g3}
- Acceptance:
  - {ac002}
  - {ac004}
- Grounding:
  - {journey_stage_10_compile}
  - {journey_stage_10_capture}
  - {gate_handoff}
- Deliverable: preserve payload-only compile and completed-output capture with no raw compile payload success path.

### Slice 3: Deterministic Downstream Adoption
- Objective: turn the emitted bundle into a bounded planning artifact without repo rereads.
- Acceptance:
  - {ac005}
- Grounding:
  - {pillar_three}
  - {pillar_four}
  - {gate_determinism}
- Deliverable: validate the emitted bundle, write `artifacts/planning/feature_slice/{feature_id}/SLICE_PLAN.md`, and keep bundle-only reads sufficient for the same planning job.

## Sequence
1. Reconfirm the route-journey proof so the happy path and skip path remain believable.
2. Lock the stage-10 handoff boundary so compile stays payload-only and capture consumes completed external output.
3. Emit, validate, and consume the handoff bundle to produce the downstream slice plan without repo rereads.
",
        feature_id = inputs.feature_id,
        pipeline_id = inputs.pipeline_id,
        consumer_id = inputs.consumer_id,
        bundle_root = inputs.bundle_root,
        repo_reread_fallback = inputs.repo_reread_fallback,
        summary = inputs.summary,
        g1 = g1,
        g2 = g2,
        g3 = g3,
        pillar_one = pillar_one,
        pillar_three = pillar_three,
        pillar_four = pillar_four,
        journey_stage_06 = journey_stage_06,
        journey_stage_10_compile = journey_stage_10_compile,
        journey_stage_10_capture = journey_stage_10_capture,
        gate_happy = gate_happy,
        gate_skip = gate_skip,
        gate_handoff = gate_handoff,
        gate_determinism = gate_determinism,
        ac001 = ac001,
        ac002 = ac002,
        ac003 = ac003,
        ac004 = ac004,
        ac005 = ac005,
    )
}

fn run_bundle_only_feature_slice_consumer_harness(
    repo_root: &std::path::Path,
    bundle_root: &str,
) -> ConsumerHarnessRun {
    let manifest = read_bundle_manifest_from_disk(repo_root, bundle_root);
    let allowlist = read_bundle_allowlist_from_disk(repo_root, bundle_root);
    let allow_read_paths = allowlist
        .allow_read_paths
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    let mut bundle_reads = Vec::new();

    let _ = read_bundle_text(
        repo_root,
        bundle_root,
        &allow_read_paths,
        &mut bundle_reads,
        "handoff_manifest.json",
    );
    let _ = read_bundle_text(
        repo_root,
        bundle_root,
        &allow_read_paths,
        &mut bundle_reads,
        "read_allowlist.json",
    );
    let trust_matrix = read_bundle_text(
        repo_root,
        bundle_root,
        &allow_read_paths,
        &mut bundle_reads,
        "trust_matrix.md",
    );
    assert!(
        trust_matrix.contains("- repo reread fallback: disabled"),
        "bundle-only harness requires reread fallback disabled"
    );

    let work_specification = read_bundle_text(
        repo_root,
        bundle_root,
        &allow_read_paths,
        &mut bundle_reads,
        &bundle_path_for_source_path(
            &manifest,
            "artifacts/work-specification/work-specification.yaml",
        ),
    );
    let foundation_strategy = read_bundle_text(
        repo_root,
        bundle_root,
        &allow_read_paths,
        &mut bundle_reads,
        &bundle_path_for_source_path(&manifest, "artifacts/foundation/FOUNDATION_STRATEGY.md"),
    );
    let tech_arch_brief = read_bundle_text(
        repo_root,
        bundle_root,
        &allow_read_paths,
        &mut bundle_reads,
        &bundle_path_for_source_path(&manifest, "artifacts/foundation/TECH_ARCH_BRIEF.md"),
    );
    let quality_gates_spec = read_bundle_text(
        repo_root,
        bundle_root,
        &allow_read_paths,
        &mut bundle_reads,
        &bundle_path_for_source_path(&manifest, "artifacts/foundation/QUALITY_GATES_SPEC.md"),
    );

    let inputs = build_planning_inputs(
        &manifest,
        &work_specification,
        &foundation_strategy,
        &tech_arch_brief,
        &quality_gates_spec,
    );
    let plan_body = render_slice_plan(&inputs);
    let plan_path = format!(
        "artifacts/planning/feature_slice/{}/SLICE_PLAN.md",
        manifest.feature_id
    );
    write_file(&repo_root.join(&plan_path), plan_body.as_bytes());

    ConsumerHarnessRun {
        feature_id: manifest.feature_id,
        bundle_root: manifest.bundle_root,
        plan_path,
        plan_body,
        bundle_reads,
        repo_rereads: Vec::new(),
    }
}

fn run_repo_reread_feature_slice_consumer_baseline(
    repo_root: &std::path::Path,
    bundle_root: &str,
) -> ConsumerHarnessRun {
    let mut bundle_reads = Vec::new();
    let mut repo_rereads = Vec::new();
    let inputs = collect_repo_reread_planning_inputs(
        repo_root,
        bundle_root,
        &mut bundle_reads,
        &mut repo_rereads,
    );
    let plan_body = render_slice_plan(&inputs);
    ConsumerHarnessRun {
        feature_id: inputs.feature_id.clone(),
        bundle_root: inputs.bundle_root,
        plan_path: format!(
            "artifacts/planning/feature_slice/{}/SLICE_PLAN.md",
            inputs.feature_id
        ),
        plan_body,
        bundle_reads,
        repo_rereads,
    }
}

fn render_handoff_validation_output(
    validated: &handbook_pipeline::pipeline_handoff::PipelineHandoffValidatedBundle,
) -> String {
    format!(
        "\
OUTCOME: VALID
PIPELINE: {pipeline_id}
CONSUMER: {consumer_id}
FEATURE ID: {feature_id}
BUNDLE ROOT: {bundle_root}
INPUT COUNT: {input_count}
ALLOW READ PATH COUNT: {allow_count}
REPO REREAD FALLBACK: disabled
",
        pipeline_id = validated.manifest.pipeline_id,
        consumer_id = validated.manifest.consumer_id,
        feature_id = validated.manifest.feature_id,
        bundle_root = validated.manifest.bundle_root,
        input_count = validated.manifest.inputs.len(),
        allow_count = validated.read_allowlist.allow_read_paths.len(),
    )
}

fn render_consumer_harness_output(run: &ConsumerHarnessRun) -> String {
    let mut out = String::new();
    out.push_str("OUTCOME: WROTE\n");
    out.push_str(&format!("FEATURE ID: {}\n", run.feature_id));
    out.push_str(&format!("BUNDLE ROOT: {}\n", run.bundle_root));
    out.push_str(&format!("PLAN PATH: {}\n", run.plan_path));
    out.push_str("BUNDLE READS:\n");
    for path in &run.bundle_reads {
        out.push_str(&format!("  - {path}\n"));
    }
    out.push_str(&format!("REPO REREADS: {}\n", run.repo_rereads.len()));
    out.push_str("READ ONLY EMITTED BUNDLE: yes\n");
    out
}

fn render_m5_handoff_scorecard(
    baseline: &ConsumerHarnessRun,
    bundle_only: &ConsumerHarnessRun,
) -> String {
    assert_eq!(
        baseline.plan_body, bundle_only.plan_body,
        "scorecard must compare the same planning output"
    );
    format!(
        "\
# M5 Handoff Scorecard

## Same Job
- Job: derive a bounded slice plan for the M4 happy-path feature from the emitted handoff bundle.
- Feature ID: `{feature_id}`
- Output parity: identical `SLICE_PLAN.md`
- Bundle Root: `{bundle_root}`

## Access Comparison
| Metric | Repo-reread baseline | Bundle-only consumer | Delta |
| --- | ---: | ---: | ---: |
| Repo rereads | {baseline_repo_rereads} | {bundle_repo_rereads} | {delta_repo_rereads} |
| Total grounding reads | {baseline_total_reads} | {bundle_total_reads} | {delta_total_reads} |
| Bundle reads | {baseline_bundle_reads} | {bundle_bundle_reads} | +{delta_bundle_reads} |

## Before: Repo-Reread Baseline
- Bundle reads:
  - {baseline_bundle_read_1}
  - {baseline_bundle_read_2}
  - {baseline_bundle_read_3}
- Repo rereads:
  - {baseline_repo_read_1}
  - {baseline_repo_read_2}
  - {baseline_repo_read_3}
  - {baseline_repo_read_4}
  - {baseline_repo_read_5}
  - {baseline_repo_read_6}

## After: Bundle-Only Consumer
- Bundle reads:
  - {bundle_read_1}
  - {bundle_read_2}
  - {bundle_read_3}
  - {bundle_read_4}
  - {bundle_read_5}
  - {bundle_read_6}
  - {bundle_read_7}
- Repo rereads:
  - none

## Conclusion
- The same planning job now completes with zero repo rereads.
- Grounding moved from canonical repo surfaces to emitted bundle copies, reducing total reads while keeping the output identical.
",
        feature_id = bundle_only.feature_id,
        bundle_root = bundle_only.bundle_root,
        baseline_repo_rereads = baseline.repo_rereads.len(),
        bundle_repo_rereads = bundle_only.repo_rereads.len(),
        delta_repo_rereads = bundle_only.repo_rereads.len() as isize
            - baseline.repo_rereads.len() as isize,
        baseline_total_reads = baseline.bundle_reads.len() + baseline.repo_rereads.len(),
        bundle_total_reads = bundle_only.bundle_reads.len() + bundle_only.repo_rereads.len(),
        delta_total_reads = (bundle_only.bundle_reads.len() + bundle_only.repo_rereads.len())
            as isize
            - (baseline.bundle_reads.len() + baseline.repo_rereads.len()) as isize,
        baseline_bundle_reads = baseline.bundle_reads.len(),
        bundle_bundle_reads = bundle_only.bundle_reads.len(),
        delta_bundle_reads = bundle_only.bundle_reads.len() - baseline.bundle_reads.len(),
        baseline_bundle_read_1 = baseline.bundle_reads[0],
        baseline_bundle_read_2 = baseline.bundle_reads[1],
        baseline_bundle_read_3 = baseline.bundle_reads[2],
        baseline_repo_read_1 = baseline.repo_rereads[0],
        baseline_repo_read_2 = baseline.repo_rereads[1],
        baseline_repo_read_3 = baseline.repo_rereads[2],
        baseline_repo_read_4 = baseline.repo_rereads[3],
        baseline_repo_read_5 = baseline.repo_rereads[4],
        baseline_repo_read_6 = baseline.repo_rereads[5],
        bundle_read_1 = bundle_only.bundle_reads[0],
        bundle_read_2 = bundle_only.bundle_reads[1],
        bundle_read_3 = bundle_only.bundle_reads[2],
        bundle_read_4 = bundle_only.bundle_reads[3],
        bundle_read_5 = bundle_only.bundle_reads[4],
        bundle_read_6 = bundle_only.bundle_reads[5],
        bundle_read_7 = bundle_only.bundle_reads[6],
    )
}

fn happy_path_evidence_transcript() -> String {
    let (_dir, root) = install_foundation_flow_demo_repo();
    let mut transcript = String::new();

    record_evidence_step(
        &mut transcript,
        root.as_path(),
        "handbook pipeline resolve --id foundation_inputs",
        run_in(
            root.as_path(),
            &["pipeline", "resolve", "--id", "foundation_inputs"],
        ),
    );
    record_evidence_step(
        &mut transcript,
        root.as_path(),
        "handbook pipeline capture --id foundation_inputs --stage stage.04_charter_inputs < tests/fixtures/foundation_flow_demo/model_outputs/happy_path/stage_04_charter_inputs.txt",
        run_in_with_input(
            root.as_path(),
            &[
                "pipeline",
                "capture",
                "--id",
                "foundation_inputs",
                "--stage",
                "stage.04_charter_inputs",
            ],
            &read_foundation_flow_demo_model_output("happy_path", "stage_04_charter_inputs.txt"),
        ),
    );
    record_evidence_step(
        &mut transcript,
        root.as_path(),
        "handbook pipeline capture --id foundation_inputs --stage stage.05_charter_synthesize < tests/fixtures/foundation_flow_demo/model_outputs/happy_path/stage_05_charter_synthesize.md",
        run_in_with_input(
            root.as_path(),
            &[
                "pipeline",
                "capture",
                "--id",
                "foundation_inputs",
                "--stage",
                "stage.05_charter_synthesize",
            ],
            &read_foundation_flow_demo_model_output("happy_path", "stage_05_charter_synthesize.md"),
        ),
    );
    record_evidence_step(
        &mut transcript,
        root.as_path(),
        "handbook pipeline state set --id foundation_inputs --var needs_project_context=true",
        run_in(
            root.as_path(),
            &[
                "pipeline",
                "state",
                "set",
                "--id",
                "foundation_inputs",
                "--var",
                "needs_project_context=true",
            ],
        ),
    );
    record_evidence_step(
        &mut transcript,
        root.as_path(),
        "handbook pipeline resolve --id foundation_inputs",
        run_in(
            root.as_path(),
            &["pipeline", "resolve", "--id", "foundation_inputs"],
        ),
    );
    record_evidence_step(
        &mut transcript,
        root.as_path(),
        "handbook pipeline capture --id foundation_inputs --stage stage.06_project_context_interview < tests/fixtures/foundation_flow_demo/model_outputs/happy_path/stage_06_project_context_interview.md",
        run_in_with_input(
            root.as_path(),
            &[
                "pipeline",
                "capture",
                "--id",
                "foundation_inputs",
                "--stage",
                "stage.06_project_context_interview",
            ],
            &read_foundation_flow_demo_model_output(
                "happy_path",
                "stage_06_project_context_interview.md",
            ),
        ),
    );
    record_evidence_step(
        &mut transcript,
        root.as_path(),
        "handbook pipeline resolve --id foundation_inputs",
        run_in(
            root.as_path(),
            &["pipeline", "resolve", "--id", "foundation_inputs"],
        ),
    );
    record_evidence_step(
        &mut transcript,
        root.as_path(),
        "handbook pipeline capture --id foundation_inputs --stage stage.07_foundation_pack < tests/fixtures/foundation_flow_demo/model_outputs/happy_path/stage_07_foundation_pack.txt",
        run_in_with_input(
            root.as_path(),
            &[
                "pipeline",
                "capture",
                "--id",
                "foundation_inputs",
                "--stage",
                "stage.07_foundation_pack",
            ],
            &read_foundation_flow_demo_model_output("happy_path", "stage_07_foundation_pack.txt"),
        ),
    );
    record_evidence_step(
        &mut transcript,
        root.as_path(),
        "HANDBOOK_PIPELINE_COMPILE_NOW_UTC=2026-01-28T18:35:10Z handbook pipeline compile --id foundation_inputs --stage stage.10_feature_spec",
        run_in_with_env(
            root.as_path(),
            &[
                "pipeline",
                "compile",
                "--id",
                "foundation_inputs",
                "--stage",
                "stage.10_feature_spec",
            ],
            &[(
                handbook_pipeline::pipeline_compile::PIPELINE_COMPILE_NOW_UTC_ENV_VAR,
                FIXED_NOW_UTC,
            )],
        ),
    );
    record_refused_evidence_step(
        &mut transcript,
        root.as_path(),
        "handbook pipeline capture --id foundation_inputs --stage stage.10_feature_spec < <compile-payload>",
        run_in_with_input(
            root.as_path(),
            &[
                "pipeline",
                "capture",
                "--id",
                "foundation_inputs",
                "--stage",
                "stage.10_feature_spec",
            ],
            &stage_10_compile_payload(root.as_path()),
        ),
    );
    record_evidence_step(
        &mut transcript,
        root.as_path(),
        "handbook pipeline capture --id foundation_inputs --stage stage.10_feature_spec < tests/fixtures/foundation_flow_demo/model_outputs/happy_path/stage_10_feature_spec.md",
        run_in_with_input(
            root.as_path(),
            &[
                "pipeline",
                "capture",
                "--id",
                "foundation_inputs",
                "--stage",
                "stage.10_feature_spec",
            ],
            &read_foundation_flow_demo_model_output("happy_path", "stage_10_feature_spec.md"),
        ),
    );
    record_evidence_step(
        &mut transcript,
        root.as_path(),
        "handbook pipeline handoff emit --id foundation_inputs --consumer feature-slice-decomposer",
        run_in(
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
        ),
    );

    let validated = handbook_pipeline::pipeline_handoff::validate_pipeline_handoff_bundle(
        root.as_path(),
        &foundation_flow_demo_happy_path_bundle_root(),
    )
    .expect("freshly emitted handoff bundle should validate");
    record_virtual_evidence_step(
        &mut transcript,
        "test-only: validate emitted handoff bundle",
        &render_handoff_validation_output(&validated),
    );

    let consumer_run = run_bundle_only_feature_slice_consumer_harness(
        root.as_path(),
        &foundation_flow_demo_happy_path_bundle_root(),
    );
    record_virtual_evidence_step(
        &mut transcript,
        "test-only: consume emitted bundle into artifacts/planning/feature_slice/fs-m4-foundation-journey-2026-04/SLICE_PLAN.md",
        &render_consumer_harness_output(&consumer_run),
    );

    transcript
}

fn skip_path_evidence_transcript() -> String {
    let (_dir, root) = install_foundation_flow_demo_repo();
    let mut transcript = String::new();

    record_evidence_step(
        &mut transcript,
        root.as_path(),
        "handbook pipeline resolve --id foundation_inputs",
        run_in(
            root.as_path(),
            &["pipeline", "resolve", "--id", "foundation_inputs"],
        ),
    );
    record_evidence_step(
        &mut transcript,
        root.as_path(),
        "handbook pipeline capture --id foundation_inputs --stage stage.04_charter_inputs < tests/fixtures/foundation_flow_demo/model_outputs/skip_path/stage_04_charter_inputs.txt",
        run_in_with_input(
            root.as_path(),
            &[
                "pipeline",
                "capture",
                "--id",
                "foundation_inputs",
                "--stage",
                "stage.04_charter_inputs",
            ],
            &read_foundation_flow_demo_model_output("skip_path", "stage_04_charter_inputs.txt"),
        ),
    );
    record_evidence_step(
        &mut transcript,
        root.as_path(),
        "handbook pipeline capture --id foundation_inputs --stage stage.05_charter_synthesize < tests/fixtures/foundation_flow_demo/model_outputs/skip_path/stage_05_charter_synthesize.md",
        run_in_with_input(
            root.as_path(),
            &[
                "pipeline",
                "capture",
                "--id",
                "foundation_inputs",
                "--stage",
                "stage.05_charter_synthesize",
            ],
            &read_foundation_flow_demo_model_output("skip_path", "stage_05_charter_synthesize.md"),
        ),
    );
    record_evidence_step(
        &mut transcript,
        root.as_path(),
        "handbook pipeline state set --id foundation_inputs --var needs_project_context=false",
        run_in(
            root.as_path(),
            &[
                "pipeline",
                "state",
                "set",
                "--id",
                "foundation_inputs",
                "--var",
                "needs_project_context=false",
            ],
        ),
    );
    record_evidence_step(
        &mut transcript,
        root.as_path(),
        "handbook pipeline resolve --id foundation_inputs",
        run_in(
            root.as_path(),
            &["pipeline", "resolve", "--id", "foundation_inputs"],
        ),
    );
    record_evidence_step(
        &mut transcript,
        root.as_path(),
        "handbook pipeline capture --id foundation_inputs --stage stage.07_foundation_pack < tests/fixtures/foundation_flow_demo/model_outputs/skip_path/stage_07_foundation_pack.txt",
        run_in_with_input(
            root.as_path(),
            &[
                "pipeline",
                "capture",
                "--id",
                "foundation_inputs",
                "--stage",
                "stage.07_foundation_pack",
            ],
            &read_foundation_flow_demo_model_output("skip_path", "stage_07_foundation_pack.txt"),
        ),
    );
    record_evidence_step(
        &mut transcript,
        root.as_path(),
        "HANDBOOK_PIPELINE_COMPILE_NOW_UTC=2026-01-28T18:35:10Z handbook pipeline compile --id foundation_inputs --stage stage.10_feature_spec",
        run_in_with_env(
            root.as_path(),
            &[
                "pipeline",
                "compile",
                "--id",
                "foundation_inputs",
                "--stage",
                "stage.10_feature_spec",
            ],
            &[(
                handbook_pipeline::pipeline_compile::PIPELINE_COMPILE_NOW_UTC_ENV_VAR,
                FIXED_NOW_UTC,
            )],
        ),
    );
    record_refused_evidence_step(
        &mut transcript,
        root.as_path(),
        "handbook pipeline capture --id foundation_inputs --stage stage.10_feature_spec < <compile-payload>",
        run_in_with_input(
            root.as_path(),
            &[
                "pipeline",
                "capture",
                "--id",
                "foundation_inputs",
                "--stage",
                "stage.10_feature_spec",
            ],
            &stage_10_compile_payload(root.as_path()),
        ),
    );
    record_evidence_step(
        &mut transcript,
        root.as_path(),
        "handbook pipeline capture --id foundation_inputs --stage stage.10_feature_spec < tests/fixtures/foundation_flow_demo/model_outputs/skip_path/stage_10_feature_spec.md",
        run_in_with_input(
            root.as_path(),
            &[
                "pipeline",
                "capture",
                "--id",
                "foundation_inputs",
                "--stage",
                "stage.10_feature_spec",
            ],
            &read_foundation_flow_demo_model_output("skip_path", "stage_10_feature_spec.md"),
        ),
    );

    transcript
}

fn assert_pipeline_capture_refusal(
    stdout: &str,
    stage: &str,
    reason: &str,
    next_safe_action: &str,
) {
    assert!(stdout.contains("OUTCOME: REFUSED"), "{stdout}");
    assert!(
        stdout.contains("PIPELINE: pipeline.foundation_inputs"),
        "{stdout}"
    );
    assert!(stdout.contains(&format!("STAGE: {stage}")), "{stdout}");
    assert!(stdout.contains(&format!("REASON: {reason}")), "{stdout}");
    assert!(
        stdout.contains(&format!("NEXT SAFE ACTION: {next_safe_action}")),
        "{stdout}"
    );
}

#[test]
fn help_lists_setup_first() {
    let output = binary().arg("--help").output().expect("help should run");

    assert!(output.status.success(), "help should succeed");

    let stdout = String::from_utf8(output.stdout).expect("help is utf-8");
    let command_lines = command_section_lines(&stdout);

    assert_eq!(
        command_lines.len(),
        8,
        "expected eight command lines in help"
    );
    assert!(
        command_lines[0].starts_with("setup "),
        "setup should be first: {command_lines:?}"
    );
    assert!(
        command_lines[1].starts_with("author "),
        "author should be second: {command_lines:?}"
    );
    assert!(
        command_lines[2].starts_with("approvers "),
        "approvers should be third: {command_lines:?}"
    );
    assert!(
        command_lines[3].starts_with("artifact "),
        "artifact should be fourth: {command_lines:?}"
    );
    assert!(
        command_lines[4].starts_with("pipeline "),
        "pipeline should be fifth: {command_lines:?}"
    );
    assert!(
        command_lines[5].starts_with("generate "),
        "generate should be sixth: {command_lines:?}"
    );
    assert!(
        command_lines[6].starts_with("inspect "),
        "inspect should be seventh: {command_lines:?}"
    );
    assert!(
        command_lines[7].starts_with("doctor "),
        "doctor should be eighth: {command_lines:?}"
    );
}

#[test]
fn version_matches_release_metadata() {
    let root = workspace_root();
    let output = run_in(root.as_path(), &["--version"]);

    assert!(output.status.success(), "version should succeed");

    let stdout = String::from_utf8(output.stdout).expect("version stdout is utf-8");
    let expected_version =
        std::fs::read_to_string(root.join("VERSION")).expect("read workspace VERSION");
    let expected = format!("handbook {}\n", expected_version.trim());

    assert_eq!(
        stdout, expected,
        "CLI version should match release metadata"
    );
}

#[test]
fn pipeline_help_lists_supported_surface() {
    let root = workspace_root();

    let output = run_in(root.as_path(), &["pipeline", "--help"]);
    assert!(output.status.success(), "pipeline help should succeed");

    let stdout = String::from_utf8(output.stdout).expect("help stdout is utf-8");
    let command_lines = command_section_lines(&stdout);

    assert_eq!(
        command_lines.len(),
        7,
        "expected seven pipeline command lines"
    );
    assert!(
        command_lines[0].starts_with("list "),
        "list should be first: {command_lines:?}"
    );
    assert!(
        command_lines[1].starts_with("show "),
        "show should be second: {command_lines:?}"
    );
    assert!(
        command_lines[2].starts_with("resolve "),
        "resolve should be third: {command_lines:?}"
    );
    assert!(
        command_lines[3].starts_with("compile "),
        "compile should be fourth: {command_lines:?}"
    );
    assert!(
        command_lines[4].starts_with("capture "),
        "capture should be fifth: {command_lines:?}"
    );
    assert!(
        command_lines[5].starts_with("handoff "),
        "handoff should be sixth: {command_lines:?}"
    );
    assert!(
        command_lines[6].starts_with("state "),
        "state should be seventh: {command_lines:?}"
    );
    assert!(
        stdout.contains("explicit stage-output capture"),
        "expected pipeline help title: {stdout}"
    );
}

#[test]
fn pipeline_state_help_lists_set() {
    let root = workspace_root();

    let output = run_in(root.as_path(), &["pipeline", "state", "--help"]);
    assert!(
        output.status.success(),
        "pipeline state help should succeed"
    );

    let stdout = String::from_utf8(output.stdout).expect("help stdout is utf-8");
    let command_lines = command_section_lines(&stdout);

    assert_eq!(
        command_lines.len(),
        1,
        "expected one pipeline state command line"
    );
    assert!(
        command_lines[0].starts_with("set "),
        "set should be the only state subcommand: {command_lines:?}"
    );
    assert!(
        stdout.contains("Set one supported route-state field"),
        "expected pipeline state help text: {stdout}"
    );
}

#[test]
fn pipeline_compile_help_lists_exact_m2_surface() {
    let root = workspace_root();

    let output = run_in(root.as_path(), &["pipeline", "compile", "--help"]);
    assert!(
        output.status.success(),
        "pipeline compile help should succeed"
    );

    let stdout = String::from_utf8(output.stdout).expect("help stdout is utf-8");
    assert!(
        stdout.contains("Compile one supported stage payload from persisted route basis"),
        "expected compile help summary: {stdout}"
    );
    assert!(stdout.contains("--id <ID>"), "expected --id: {stdout}");
    assert!(
        stdout.contains("--stage <STAGE>"),
        "expected --stage: {stdout}"
    );
    assert!(stdout.contains("--explain"), "expected --explain: {stdout}");
}

#[test]
fn pipeline_capture_help_lists_preview_and_apply_surface() {
    let root = workspace_root();

    let output = run_in(root.as_path(), &["pipeline", "capture", "--help"]);
    assert!(
        output.status.success(),
        "pipeline capture help should succeed"
    );

    let stdout = String::from_utf8(output.stdout).expect("help stdout is utf-8");
    assert!(
        stdout.contains(
            "Capture one supported stage output and materialize declared artifact and repo-mirror files"
        ),
        "expected capture help summary: {stdout}"
    );
    assert!(stdout.contains("--id <ID>"), "expected --id: {stdout}");
    assert!(
        stdout.contains("--stage <STAGE>"),
        "expected --stage: {stdout}"
    );
    assert!(stdout.contains("--preview"), "expected --preview: {stdout}");
    assert!(
        stdout.contains("apply"),
        "expected apply subcommand: {stdout}"
    );

    let apply = run_in(root.as_path(), &["pipeline", "capture", "apply", "--help"]);
    assert!(
        apply.status.success(),
        "pipeline capture apply help should succeed"
    );
    let apply_stdout = String::from_utf8(apply.stdout).expect("apply help stdout is utf-8");
    assert!(
        apply_stdout.contains("--capture-id <CAPTURE_ID>"),
        "expected --capture-id: {apply_stdout}"
    );
}

#[test]
fn pipeline_capture_preview_stage_04_matches_shared_golden() {
    let (_dir, root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    prepare_stage_04_capture_ready_route_basis(root.as_path());

    let output = run_in_with_input(
        root.as_path(),
        &[
            "pipeline",
            "capture",
            "--id",
            "foundation_inputs",
            "--stage",
            "stage.04_charter_inputs",
            "--preview",
        ],
        &stage_04_capture_input(root.as_path()),
    );
    assert!(output.status.success(), "preview should succeed");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    pipeline_proof_corpus_support::assert_matches_golden_with_explicit_placeholders(
        &normalize_capture_id(&stdout),
        &[],
        "capture.preview.stage_04_charter_inputs.txt",
    );
}

#[test]
fn pipeline_capture_preview_charter_matches_shared_golden() {
    let (_dir, root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    prepare_stage_05_capture_ready_route_basis(root.as_path());

    let output = run_in_with_input(
        root.as_path(),
        &[
            "pipeline",
            "capture",
            "--id",
            "foundation_inputs",
            "--stage",
            "stage.05_charter_synthesize",
            "--preview",
        ],
        &stage_05_capture_input(root.as_path()),
    );
    assert!(output.status.success(), "preview should succeed");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    pipeline_proof_corpus_support::assert_matches_golden_with_explicit_placeholders(
        &normalize_capture_id(&stdout),
        &[],
        "capture.preview.stage_05_charter_synthesize.txt",
    );
}

#[test]
fn pipeline_capture_preview_stage_06_matches_shared_golden() {
    let (_dir, root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    prepare_stage_06_capture_ready_route_basis(root.as_path());

    let output = run_in_with_input(
        root.as_path(),
        &[
            "pipeline",
            "capture",
            "--id",
            "foundation_inputs",
            "--stage",
            "stage.06_project_context_interview",
            "--preview",
        ],
        &stage_06_capture_input(root.as_path()),
    );
    assert!(output.status.success(), "preview should succeed");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    pipeline_proof_corpus_support::assert_matches_golden_with_explicit_placeholders(
        &normalize_capture_id(&stdout),
        &[],
        "capture.preview.stage_06_project_context_interview.txt",
    );
}

#[test]
fn pipeline_capture_preview_foundation_pack_matches_shared_golden() {
    let (_dir, root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    prepare_stage_07_capture_ready_route_basis(root.as_path());

    let output = run_in_with_input(
        root.as_path(),
        &[
            "pipeline",
            "capture",
            "--id",
            "foundation_inputs",
            "--stage",
            "stage.07_foundation_pack",
            "--preview",
        ],
        &stage_07_capture_input(root.as_path()),
    );
    assert!(output.status.success(), "preview should succeed");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    pipeline_proof_corpus_support::assert_matches_golden_with_explicit_placeholders(
        &normalize_capture_id(&stdout),
        &[],
        "capture.preview.stage_07_foundation_pack.txt",
    );
}

#[test]
fn pipeline_capture_preview_stage_10_matches_shared_golden() {
    let (_dir, root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    prepare_stage_10_capture_ready_route_basis(root.as_path());

    let output = run_in_with_input(
        root.as_path(),
        &[
            "pipeline",
            "capture",
            "--id",
            "foundation_inputs",
            "--stage",
            "stage.10_feature_spec",
            "--preview",
        ],
        &stage_10_completed_feature_spec_input(),
    );
    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert!(output.status.success(), "preview should succeed: {stdout}");
    pipeline_proof_corpus_support::assert_matches_golden_with_explicit_placeholders(
        &normalize_capture_id(&stdout),
        &[],
        "capture.preview.stage_10_feature_spec.txt",
    );
}

#[test]
fn pipeline_capture_preview_stage_04_refuses_file_wrapper() {
    let (_dir, root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    prepare_stage_04_capture_ready_route_basis(root.as_path());
    let wrapped = format!(
        "--- FILE: artifacts/charter/CHARTER_INPUTS.yaml ---\n{}",
        stage_04_capture_input(root.as_path())
    );

    let output = run_in_with_input(
        root.as_path(),
        &[
            "pipeline",
            "capture",
            "--id",
            "foundation_inputs",
            "--stage",
            "stage.04_charter_inputs",
            "--preview",
        ],
        &wrapped,
    );
    assert!(!output.status.success(), "preview should refuse");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert_pipeline_capture_refusal(
        &stdout,
        "stage.04_charter_inputs",
        "invalid_capture_input: single-file capture stages must receive plain body content and must not use `--- FILE:` wrappers",
        "paste only the stage body and retry `pipeline capture`",
    );
}

#[test]
fn pipeline_capture_preview_stage_04_refuses_empty_body() {
    let (_dir, root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    prepare_stage_04_capture_ready_route_basis(root.as_path());

    let output = run_in_with_input(
        root.as_path(),
        &[
            "pipeline",
            "capture",
            "--id",
            "foundation_inputs",
            "--stage",
            "stage.04_charter_inputs",
            "--preview",
        ],
        "\n",
    );
    assert!(!output.status.success(), "preview should refuse");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert_pipeline_capture_refusal(
        &stdout,
        "stage.04_charter_inputs",
        "invalid_capture_input: single-file capture stages must receive a non-empty body",
        "paste the generated stage body and retry `pipeline capture`",
    );
}

#[test]
fn pipeline_capture_preview_stage_06_refuses_file_wrapper() {
    let (_dir, root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    prepare_stage_06_capture_ready_route_basis(root.as_path());
    let wrapped = format!(
        "--- FILE: artifacts/project_context/PROJECT_CONTEXT.md ---\n{}",
        stage_06_capture_input(root.as_path())
    );

    let output = run_in_with_input(
        root.as_path(),
        &[
            "pipeline",
            "capture",
            "--id",
            "foundation_inputs",
            "--stage",
            "stage.06_project_context_interview",
            "--preview",
        ],
        &wrapped,
    );
    assert!(!output.status.success(), "preview should refuse");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert_pipeline_capture_refusal(
        &stdout,
        "stage.06_project_context_interview",
        "invalid_capture_input: single-file capture stages must receive plain body content and must not use `--- FILE:` wrappers",
        "paste only the stage body and retry `pipeline capture`",
    );
}

#[test]
fn pipeline_capture_preview_stage_06_refuses_empty_body() {
    let (_dir, root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    prepare_stage_06_capture_ready_route_basis(root.as_path());

    let output = run_in_with_input(
        root.as_path(),
        &[
            "pipeline",
            "capture",
            "--id",
            "foundation_inputs",
            "--stage",
            "stage.06_project_context_interview",
            "--preview",
        ],
        "\n",
    );
    assert!(!output.status.success(), "preview should refuse");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert_pipeline_capture_refusal(
        &stdout,
        "stage.06_project_context_interview",
        "invalid_capture_input: single-file capture stages must receive a non-empty body",
        "paste the generated stage body and retry `pipeline capture`",
    );
}

#[test]
fn pipeline_capture_preview_stage_10_refuses_file_wrapper() {
    let (_dir, root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    prepare_stage_10_capture_ready_route_basis(root.as_path());
    let wrapped = format!(
        "--- FILE: artifacts/work-specification/work-specification.yaml ---\n{}",
        stage_10_completed_feature_spec_input()
    );

    let output = run_in_with_input(
        root.as_path(),
        &[
            "pipeline",
            "capture",
            "--id",
            "foundation_inputs",
            "--stage",
            "stage.10_feature_spec",
            "--preview",
        ],
        &wrapped,
    );
    assert!(!output.status.success(), "preview should refuse");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert_pipeline_capture_refusal(
        &stdout,
        "stage.10_feature_spec",
        "invalid_capture_input: single-file capture stages must receive plain body content and must not use `--- FILE:` wrappers",
        "paste only the stage body and retry `pipeline capture`",
    );
}

#[test]
fn pipeline_capture_preview_stage_10_refuses_empty_body() {
    let (_dir, root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    prepare_stage_10_capture_ready_route_basis(root.as_path());

    let output = run_in_with_input(
        root.as_path(),
        &[
            "pipeline",
            "capture",
            "--id",
            "foundation_inputs",
            "--stage",
            "stage.10_feature_spec",
            "--preview",
        ],
        "\n",
    );
    assert!(!output.status.success(), "preview should refuse");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert_pipeline_capture_refusal(
        &stdout,
        "stage.10_feature_spec",
        "invalid_capture_input: single-file capture stages must receive a non-empty body",
        "paste the generated stage body and retry `pipeline capture`",
    );
}

#[test]
fn pipeline_capture_apply_stage_04_matches_shared_golden() {
    let (_dir, root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    prepare_stage_04_capture_ready_route_basis(root.as_path());

    let output = run_in_with_input(
        root.as_path(),
        &[
            "pipeline",
            "capture",
            "--id",
            "foundation_inputs",
            "--stage",
            "stage.04_charter_inputs",
        ],
        &stage_04_capture_input(root.as_path()),
    );
    assert!(output.status.success(), "capture should succeed");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    pipeline_proof_corpus_support::assert_matches_golden_with_explicit_placeholders(
        &stdout,
        &[],
        "capture.apply.stage_04_charter_inputs.txt",
    );
}

#[test]
fn pipeline_capture_apply_charter_matches_shared_golden() {
    let (_dir, root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    prepare_stage_05_capture_ready_route_basis(root.as_path());

    let output = run_in_with_input(
        root.as_path(),
        &[
            "pipeline",
            "capture",
            "--id",
            "foundation_inputs",
            "--stage",
            "stage.05_charter_synthesize",
        ],
        &stage_05_capture_input(root.as_path()),
    );
    assert!(output.status.success(), "capture should succeed");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    pipeline_proof_corpus_support::assert_matches_golden_with_explicit_placeholders(
        &stdout,
        &[],
        "capture.apply.stage_05_charter_synthesize.txt",
    );
}

#[test]
fn pipeline_capture_apply_stage_06_matches_shared_golden() {
    let (_dir, root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    prepare_stage_06_capture_ready_route_basis(root.as_path());

    let output = run_in_with_input(
        root.as_path(),
        &[
            "pipeline",
            "capture",
            "--id",
            "foundation_inputs",
            "--stage",
            "stage.06_project_context_interview",
        ],
        &stage_06_capture_input(root.as_path()),
    );
    assert!(output.status.success(), "capture should succeed");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    pipeline_proof_corpus_support::assert_matches_golden_with_explicit_placeholders(
        &stdout,
        &[],
        "capture.apply.stage_06_project_context_interview.txt",
    );
}

#[test]
fn pipeline_capture_apply_foundation_pack_matches_shared_golden() {
    let (_dir, root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    prepare_stage_07_capture_ready_route_basis(root.as_path());

    let preview = run_in_with_input(
        root.as_path(),
        &[
            "pipeline",
            "capture",
            "--id",
            "foundation_inputs",
            "--stage",
            "stage.07_foundation_pack",
            "--preview",
        ],
        &stage_07_capture_input(root.as_path()),
    );
    assert!(preview.status.success(), "preview should succeed");
    let preview_stdout = String::from_utf8(preview.stdout).expect("preview stdout is utf-8");
    let capture_id = preview_stdout
        .lines()
        .find_map(|line| line.strip_prefix("CAPTURE ID: "))
        .expect("capture id");

    let apply = run_in(
        root.as_path(),
        &["pipeline", "capture", "apply", "--capture-id", capture_id],
    );
    assert!(apply.status.success(), "apply should succeed");

    let stdout = String::from_utf8(apply.stdout).expect("stdout is utf-8");
    pipeline_proof_corpus_support::assert_matches_golden_with_explicit_placeholders(
        &stdout,
        &[],
        "capture.apply.stage_07_foundation_pack.txt",
    );
}

#[test]
fn pipeline_capture_apply_stage_10_matches_shared_golden() {
    let (_dir, root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    prepare_stage_10_capture_ready_route_basis(root.as_path());

    let output = run_in_with_input(
        root.as_path(),
        &[
            "pipeline",
            "capture",
            "--id",
            "foundation_inputs",
            "--stage",
            "stage.10_feature_spec",
        ],
        &stage_10_completed_feature_spec_input(),
    );
    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert!(output.status.success(), "capture should succeed: {stdout}");
    pipeline_proof_corpus_support::assert_matches_golden_with_explicit_placeholders(
        &stdout,
        &[],
        "capture.apply.stage_10_feature_spec.txt",
    );
}

#[test]
fn pipeline_capture_stage_10_refuses_raw_compile_payload() {
    let (_dir, root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    prepare_stage_10_capture_ready_route_basis(root.as_path());

    let output = run_in_with_input(
        root.as_path(),
        &[
            "pipeline",
            "capture",
            "--id",
            "foundation_inputs",
            "--stage",
            "stage.10_feature_spec",
        ],
        &stage_10_compile_payload(root.as_path()),
    );
    assert!(
        !output.status.success(),
        "raw compile payload should refuse"
    );

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert_pipeline_capture_refusal(
        &stdout,
        "stage.10_feature_spec",
        "invalid_capture_input: stage.10_feature_spec capture must receive Work Specification YAML, not raw `pipeline compile` payload",
        "run the stage-10 compile payload through an external operator or model runner, then retry `pipeline capture` with one duplicate-free Work Specification YAML document",
    );
    assert!(
        !root
            .join("artifacts/work-specification/work-specification.yaml")
            .exists(),
        "raw compile payload refusal must not create the canonical Work Specification"
    );
    assert!(
        !root.join("artifacts/feature_spec/FEATURE_SPEC.md").exists(),
        "raw compile payload refusal must not create the generated Markdown view"
    );
}

#[test]
fn pipeline_capture_apply_refuses_missing_capture_id() {
    let (_dir, root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    let output = run_in(
        root.as_path(),
        &[
            "pipeline",
            "capture",
            "apply",
            "--capture-id",
            "1111111111111111111111111111111111111111111111111111111111111111",
        ],
    );
    assert!(!output.status.success(), "apply should refuse");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    pipeline_proof_corpus_support::assert_matches_golden_with_explicit_placeholders(
        &stdout,
        &[],
        "capture.refused.missing_capture_id.txt",
    );
}

#[test]
fn pipeline_foundation_inputs_m4_happy_path_proves_real_stage_10_handoff() {
    let (_dir, root) = install_foundation_flow_demo_repo();

    let resolve = run_in(
        root.as_path(),
        &["pipeline", "resolve", "--id", "foundation_inputs"],
    );
    assert!(resolve.status.success(), "initial resolve should succeed");

    let stage_04 = run_in_with_input(
        root.as_path(),
        &[
            "pipeline",
            "capture",
            "--id",
            "foundation_inputs",
            "--stage",
            "stage.04_charter_inputs",
        ],
        &read_foundation_flow_demo_model_output("happy_path", "stage_04_charter_inputs.txt"),
    );
    assert!(stage_04.status.success(), "stage 04 capture should succeed");

    let stage_05 = run_in_with_input(
        root.as_path(),
        &[
            "pipeline",
            "capture",
            "--id",
            "foundation_inputs",
            "--stage",
            "stage.05_charter_synthesize",
        ],
        &read_foundation_flow_demo_model_output("happy_path", "stage_05_charter_synthesize.md"),
    );
    assert!(stage_05.status.success(), "stage 05 capture should succeed");

    let state_set = run_in(
        root.as_path(),
        &[
            "pipeline",
            "state",
            "set",
            "--id",
            "foundation_inputs",
            "--var",
            "needs_project_context=true",
        ],
    );
    assert!(
        state_set.status.success(),
        "needs_project_context state set should succeed"
    );

    let second_resolve = run_in(
        root.as_path(),
        &["pipeline", "resolve", "--id", "foundation_inputs"],
    );
    assert!(
        second_resolve.status.success(),
        "second resolve should succeed"
    );
    let second_resolve_stdout =
        String::from_utf8(second_resolve.stdout).expect("resolve stdout is utf-8");
    assert!(
        second_resolve_stdout.contains("stage.06_project_context_interview | active"),
        "stage 06 should be active after the manual handoff: {second_resolve_stdout}"
    );
    assert!(
        second_resolve_stdout.contains("needs_project_context = true"),
        "route basis should record the explicit stage-05 handoff: {second_resolve_stdout}"
    );

    let stage_06 = run_in_with_input(
        root.as_path(),
        &[
            "pipeline",
            "capture",
            "--id",
            "foundation_inputs",
            "--stage",
            "stage.06_project_context_interview",
        ],
        &read_foundation_flow_demo_model_output(
            "happy_path",
            "stage_06_project_context_interview.md",
        ),
    );
    assert!(stage_06.status.success(), "stage 06 capture should succeed");

    let third_resolve = run_in(
        root.as_path(),
        &["pipeline", "resolve", "--id", "foundation_inputs"],
    );
    assert!(
        third_resolve.status.success(),
        "post-stage-06 resolve should succeed"
    );

    let stage_07 = run_in_with_input(
        root.as_path(),
        &[
            "pipeline",
            "capture",
            "--id",
            "foundation_inputs",
            "--stage",
            "stage.07_foundation_pack",
        ],
        &read_foundation_flow_demo_model_output("happy_path", "stage_07_foundation_pack.txt"),
    );
    assert!(stage_07.status.success(), "stage 07 capture should succeed");

    let stage_10_payload = stage_10_compile_payload(root.as_path());
    assert!(
        stage_10_payload.starts_with("# stage.10_feature_spec - Work Specification"),
        "stage-10 compile should remain payload-only stage input: {stage_10_payload}"
    );
    let work_specification =
        read_foundation_flow_demo_model_output("happy_path", "stage_10_feature_spec.md");
    assert_ne!(
        stage_10_payload, work_specification,
        "stage-10 compile payload must stay distinct from completed external model output"
    );
    let raw_stage_10 = run_in_with_input(
        root.as_path(),
        &[
            "pipeline",
            "capture",
            "--id",
            "foundation_inputs",
            "--stage",
            "stage.10_feature_spec",
        ],
        &stage_10_payload,
    );
    assert!(
        !raw_stage_10.status.success(),
        "raw stage-10 compile payload capture must refuse"
    );
    let raw_stage_10_stdout =
        String::from_utf8(raw_stage_10.stdout).expect("refusal stdout is utf-8");
    assert_pipeline_capture_refusal(
        &raw_stage_10_stdout,
        "stage.10_feature_spec",
        "invalid_capture_input: stage.10_feature_spec capture must receive Work Specification YAML, not raw `pipeline compile` payload",
        "run the stage-10 compile payload through an external operator or model runner, then retry `pipeline capture` with one duplicate-free Work Specification YAML document",
    );
    assert!(
        !root
            .join("artifacts/work-specification/work-specification.yaml")
            .exists(),
        "raw stage-10 compile payload refusal must not create the canonical Work Specification"
    );
    assert!(
        !root.join("artifacts/feature_spec/FEATURE_SPEC.md").exists(),
        "raw stage-10 compile payload refusal must not create the generated Markdown view"
    );
    let stage_10 = run_in_with_input(
        root.as_path(),
        &[
            "pipeline",
            "capture",
            "--id",
            "foundation_inputs",
            "--stage",
            "stage.10_feature_spec",
        ],
        &work_specification,
    );
    assert!(stage_10.status.success(), "stage 10 capture should succeed");
    assert_eq!(
        std::fs::read_to_string(root.join("artifacts/work-specification/work-specification.yaml"))
            .expect("canonical Work Specification"),
        work_specification
    );
    let generated_view =
        std::fs::read_to_string(root.join("artifacts/feature_spec/FEATURE_SPEC.md"))
            .expect("generated Markdown view");
    assert_eq!(
        generated_view,
        read_foundation_flow_demo_expected("happy_path", "final_feature_spec.md")
    );
}

#[test]
fn pipeline_foundation_inputs_m5_happy_path_emits_valid_bundle_and_produces_slice_plan() {
    let (_dir, root) = install_foundation_flow_demo_repo();

    let resolve = run_in(
        root.as_path(),
        &["pipeline", "resolve", "--id", "foundation_inputs"],
    );
    assert!(resolve.status.success(), "initial resolve should succeed");

    let stage_04 = run_in_with_input(
        root.as_path(),
        &[
            "pipeline",
            "capture",
            "--id",
            "foundation_inputs",
            "--stage",
            "stage.04_charter_inputs",
        ],
        &read_foundation_flow_demo_model_output("happy_path", "stage_04_charter_inputs.txt"),
    );
    assert!(stage_04.status.success(), "stage 04 capture should succeed");

    let stage_05 = run_in_with_input(
        root.as_path(),
        &[
            "pipeline",
            "capture",
            "--id",
            "foundation_inputs",
            "--stage",
            "stage.05_charter_synthesize",
        ],
        &read_foundation_flow_demo_model_output("happy_path", "stage_05_charter_synthesize.md"),
    );
    assert!(stage_05.status.success(), "stage 05 capture should succeed");

    let state_set = run_in(
        root.as_path(),
        &[
            "pipeline",
            "state",
            "set",
            "--id",
            "foundation_inputs",
            "--var",
            "needs_project_context=true",
        ],
    );
    assert!(
        state_set.status.success(),
        "needs_project_context state set should succeed"
    );

    let second_resolve = run_in(
        root.as_path(),
        &["pipeline", "resolve", "--id", "foundation_inputs"],
    );
    assert!(
        second_resolve.status.success(),
        "second resolve should succeed"
    );

    let stage_06 = run_in_with_input(
        root.as_path(),
        &[
            "pipeline",
            "capture",
            "--id",
            "foundation_inputs",
            "--stage",
            "stage.06_project_context_interview",
        ],
        &read_foundation_flow_demo_model_output(
            "happy_path",
            "stage_06_project_context_interview.md",
        ),
    );
    assert!(stage_06.status.success(), "stage 06 capture should succeed");

    let third_resolve = run_in(
        root.as_path(),
        &["pipeline", "resolve", "--id", "foundation_inputs"],
    );
    assert!(
        third_resolve.status.success(),
        "post-stage-06 resolve should succeed"
    );

    let stage_07 = run_in_with_input(
        root.as_path(),
        &[
            "pipeline",
            "capture",
            "--id",
            "foundation_inputs",
            "--stage",
            "stage.07_foundation_pack",
        ],
        &read_foundation_flow_demo_model_output("happy_path", "stage_07_foundation_pack.txt"),
    );
    assert!(stage_07.status.success(), "stage 07 capture should succeed");

    let stage_10 = run_in_with_input(
        root.as_path(),
        &[
            "pipeline",
            "capture",
            "--id",
            "foundation_inputs",
            "--stage",
            "stage.10_feature_spec",
        ],
        &read_foundation_flow_demo_model_output("happy_path", "stage_10_feature_spec.md"),
    );
    assert!(stage_10.status.success(), "stage 10 capture should succeed");

    let emit = run_in(
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
    assert!(emit.status.success(), "handoff emit should succeed");

    let emit_stdout = String::from_utf8(emit.stdout).expect("emit stdout is utf-8");
    assert!(
        emit_stdout.contains("OUTCOME: EMITTED"),
        "emit output should report success: {emit_stdout}"
    );
    assert!(
        emit_stdout.contains(&format!(
            "FEATURE ID: {FOUNDATION_FLOW_DEMO_HAPPY_PATH_FEATURE_ID}"
        )),
        "emit output should keep the stable feature id: {emit_stdout}"
    );
    assert!(
        emit_stdout.contains(&format!(
            "BUNDLE ROOT: {}",
            foundation_flow_demo_happy_path_bundle_root()
        )),
        "emit output should keep the stable bundle root: {emit_stdout}"
    );

    let validated = handbook_pipeline::pipeline_handoff::validate_pipeline_handoff_bundle(
        root.as_path(),
        &foundation_flow_demo_happy_path_bundle_root(),
    )
    .expect("freshly emitted bundle should validate");
    assert_eq!(
        validated.manifest.feature_id,
        FOUNDATION_FLOW_DEMO_HAPPY_PATH_FEATURE_ID
    );
    assert_eq!(
        validated.manifest.bundle_root,
        foundation_flow_demo_happy_path_bundle_root()
    );
    assert!(
        !validated.read_allowlist.repo_reread_allowed,
        "handoff allowlist must keep repo reread fallback disabled"
    );

    let bundle_only_run = run_bundle_only_feature_slice_consumer_harness(
        root.as_path(),
        &foundation_flow_demo_happy_path_bundle_root(),
    );
    assert!(
        bundle_only_run.repo_rereads.is_empty(),
        "happy-path consumer harness must not reread the repo"
    );
    assert_eq!(
        bundle_only_run.plan_body,
        read_foundation_flow_demo_expected("happy_path", "SLICE_PLAN.md")
    );
    assert_eq!(
        std::fs::read_to_string(root.join(&bundle_only_run.plan_path))
            .expect("slice plan artifact"),
        bundle_only_run.plan_body
    );

    let baseline_run = run_repo_reread_feature_slice_consumer_baseline(
        root.as_path(),
        &foundation_flow_demo_happy_path_bundle_root(),
    );
    assert_eq!(
        baseline_run.plan_body, bundle_only_run.plan_body,
        "scorecard should compare the same downstream plan"
    );
    let scorecard = render_m5_handoff_scorecard(&baseline_run, &bundle_only_run);
    assert_eq!(
        scorecard,
        read_foundation_flow_demo_evidence("m5_handoff_scorecard.md")
    );
}

#[test]
fn pipeline_foundation_inputs_m4_skip_path_skips_stage_06_when_both_route_predicates_are_false() {
    let (_dir, root) = install_foundation_flow_demo_repo();

    let resolve = run_in(
        root.as_path(),
        &["pipeline", "resolve", "--id", "foundation_inputs"],
    );
    assert!(resolve.status.success(), "initial resolve should succeed");

    let stage_04 = run_in_with_input(
        root.as_path(),
        &[
            "pipeline",
            "capture",
            "--id",
            "foundation_inputs",
            "--stage",
            "stage.04_charter_inputs",
        ],
        &read_foundation_flow_demo_model_output("skip_path", "stage_04_charter_inputs.txt"),
    );
    assert!(stage_04.status.success(), "stage 04 capture should succeed");

    let stage_05 = run_in_with_input(
        root.as_path(),
        &[
            "pipeline",
            "capture",
            "--id",
            "foundation_inputs",
            "--stage",
            "stage.05_charter_synthesize",
        ],
        &read_foundation_flow_demo_model_output("skip_path", "stage_05_charter_synthesize.md"),
    );
    assert!(stage_05.status.success(), "stage 05 capture should succeed");

    let state_set = run_in(
        root.as_path(),
        &[
            "pipeline",
            "state",
            "set",
            "--id",
            "foundation_inputs",
            "--var",
            "needs_project_context=false",
        ],
    );
    assert!(
        state_set.status.success(),
        "needs_project_context state set should succeed"
    );

    let second_resolve = run_in(
        root.as_path(),
        &["pipeline", "resolve", "--id", "foundation_inputs"],
    );
    assert!(
        second_resolve.status.success(),
        "second resolve should succeed"
    );
    let second_resolve_stdout =
        String::from_utf8(second_resolve.stdout).expect("resolve stdout is utf-8");
    assert!(
        second_resolve_stdout.contains("charter_gaps_detected = false"),
        "skip path should keep charter gaps false by content: {second_resolve_stdout}"
    );
    assert!(
        second_resolve_stdout.contains("needs_project_context = false"),
        "skip path should preserve explicit operator decision: {second_resolve_stdout}"
    );
    assert!(
        second_resolve_stdout.contains("stage.06_project_context_interview | skipped"),
        "stage 06 should stay skipped: {second_resolve_stdout}"
    );
    assert!(
        second_resolve_stdout.contains(
            "REASON: activation evaluated false for variables: charter_gaps_detected, needs_project_context"
        ),
        "stage 06 skip reason must stay explicit: {second_resolve_stdout}"
    );
    assert!(
        second_resolve_stdout.contains("stage.07_foundation_pack | active"),
        "stage 07 should remain active on the skip path: {second_resolve_stdout}"
    );
    assert!(
        second_resolve_stdout.contains("stage.10_feature_spec | active"),
        "stage 10 should remain active on the skip path: {second_resolve_stdout}"
    );

    let stage_07 = run_in_with_input(
        root.as_path(),
        &[
            "pipeline",
            "capture",
            "--id",
            "foundation_inputs",
            "--stage",
            "stage.07_foundation_pack",
        ],
        &read_foundation_flow_demo_model_output("skip_path", "stage_07_foundation_pack.txt"),
    );
    assert!(stage_07.status.success(), "stage 07 capture should succeed");

    let stage_10_payload = stage_10_compile_payload(root.as_path());
    assert!(
        stage_10_payload.starts_with("# stage.10_feature_spec - Work Specification"),
        "stage-10 compile should remain payload-only stage input: {stage_10_payload}"
    );
    let work_specification =
        read_foundation_flow_demo_model_output("skip_path", "stage_10_feature_spec.md");
    assert_ne!(
        stage_10_payload, work_specification,
        "stage-10 compile payload must stay distinct from completed external model output"
    );
    let raw_stage_10 = run_in_with_input(
        root.as_path(),
        &[
            "pipeline",
            "capture",
            "--id",
            "foundation_inputs",
            "--stage",
            "stage.10_feature_spec",
        ],
        &stage_10_payload,
    );
    assert!(
        !raw_stage_10.status.success(),
        "raw stage-10 compile payload capture must refuse"
    );
    let raw_stage_10_stdout =
        String::from_utf8(raw_stage_10.stdout).expect("refusal stdout is utf-8");
    assert_pipeline_capture_refusal(
        &raw_stage_10_stdout,
        "stage.10_feature_spec",
        "invalid_capture_input: stage.10_feature_spec capture must receive Work Specification YAML, not raw `pipeline compile` payload",
        "run the stage-10 compile payload through an external operator or model runner, then retry `pipeline capture` with one duplicate-free Work Specification YAML document",
    );
    assert!(
        !root
            .join("artifacts/work-specification/work-specification.yaml")
            .exists(),
        "raw stage-10 compile payload refusal must not create the canonical Work Specification"
    );
    assert!(
        !root.join("artifacts/feature_spec/FEATURE_SPEC.md").exists(),
        "raw stage-10 compile payload refusal must not create the generated Markdown view"
    );
    let stage_10 = run_in_with_input(
        root.as_path(),
        &[
            "pipeline",
            "capture",
            "--id",
            "foundation_inputs",
            "--stage",
            "stage.10_feature_spec",
        ],
        &work_specification,
    );
    assert!(stage_10.status.success(), "stage 10 capture should succeed");
    assert_eq!(
        std::fs::read_to_string(root.join("artifacts/work-specification/work-specification.yaml"))
            .expect("canonical Work Specification"),
        work_specification
    );
    let generated_view =
        std::fs::read_to_string(root.join("artifacts/feature_spec/FEATURE_SPEC.md"))
            .expect("generated Markdown view");
    assert_eq!(
        generated_view,
        read_foundation_flow_demo_expected("skip_path", "final_feature_spec.md")
    );
}

#[test]
fn pipeline_foundation_inputs_m4_happy_path_matches_committed_evidence_bundle() {
    let transcript = happy_path_evidence_transcript();
    assert_eq!(
        transcript,
        read_foundation_flow_demo_evidence("happy_path.transcript.txt"),
        "happy-path evidence transcript drifted; update the committed bundle under tests/fixtures/foundation_flow_demo/evidence/ if intentional"
    );
}

#[test]
fn pipeline_foundation_inputs_m4_skip_path_matches_committed_evidence_bundle() {
    let transcript = skip_path_evidence_transcript();
    assert_eq!(
        transcript,
        read_foundation_flow_demo_evidence("skip_path.transcript.txt"),
        "skip-path evidence transcript drifted; update the committed bundle under tests/fixtures/foundation_flow_demo/evidence/ if intentional"
    );
}

#[test]
fn pipeline_foundation_inputs_m4_evidence_bundle_manifest_declares_normalization_rules() {
    let manifest = read_foundation_flow_demo_evidence("README.md");

    for phrase in [
        "HANDBOOK_PIPELINE_COMPILE_NOW_UTC=2026-01-28T18:35:10Z",
        "`{{CAPTURE_ID}}`",
        "`{{REPO_ROOT}}`",
        "`{{STATE_PATH}}`",
        "Shared stage-10 contract regressions read their completed external output from `tests/fixtures/pipeline_proof_corpus/foundation_inputs/model_outputs/stage_10_feature_spec.md`.",
    ] {
        assert!(
            manifest.contains(phrase),
            "evidence manifest missing phrase `{phrase}`"
        );
    }
}

#[cfg(unix)]
#[test]
fn pipeline_capture_preview_refuses_invalid_write_target() {
    let (_dir, root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    prepare_stage_05_capture_ready_route_basis(root.as_path());
    let external = tempfile::tempdir().expect("tempdir");
    let repo_mirror = root.join("CHARTER.md");

    std::fs::remove_file(&repo_mirror).expect("remove repo mirror");
    std::os::unix::fs::symlink(external.path().join("CHARTER.md"), &repo_mirror)
        .expect("replace repo mirror with symlink");

    let output = run_in_with_input(
        root.as_path(),
        &[
            "pipeline",
            "capture",
            "--id",
            "foundation_inputs",
            "--stage",
            "stage.05_charter_synthesize",
            "--preview",
        ],
        &stage_05_capture_input(root.as_path()),
    );
    assert!(!output.status.success(), "preview should refuse");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert!(stdout.contains("OUTCOME: REFUSED"), "{stdout}");
    assert!(
        stdout.contains("PIPELINE: pipeline.foundation_inputs"),
        "{stdout}"
    );
    assert!(
        stdout.contains("STAGE: stage.05_charter_synthesize"),
        "{stdout}"
    );
    assert!(stdout.contains("invalid_write_target"), "{stdout}");
    assert!(
        stdout.contains("cannot be written through symlink"),
        "{stdout}"
    );
}

#[test]
fn pipeline_capture_preview_refuses_unsupported_pipeline_with_current_capture_contract() {
    let root = workspace_root();
    let output = run_in_with_input(
        root.as_path(),
        &[
            "pipeline",
            "capture",
            "--id",
            "pipeline.foundation",
            "--stage",
            "stage.00_base",
            "--preview",
        ],
        "ignored input",
    );
    assert!(!output.status.success(), "preview should refuse");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert!(stdout.contains("OUTCOME: REFUSED"), "{stdout}");
    assert!(stdout.contains("PIPELINE: pipeline.foundation"), "{stdout}");
    assert!(stdout.contains("STAGE: stage.00_base"), "{stdout}");
    assert!(
        stdout.contains(
            "REASON: unsupported_target: `pipeline capture` currently supports only pipeline `pipeline.foundation_inputs`"
        ),
        "{stdout}"
    );
    assert!(
        stdout.contains(
            "NEXT SAFE ACTION: retry with `pipeline capture --id pipeline.foundation_inputs --stage stage.04_charter_inputs`"
        ),
        "{stdout}"
    );
    assert!(
        !stdout.contains("M3"),
        "unsupported-target refusal should not mention milestones: {stdout}"
    );
}

#[test]
fn pipeline_list_and_show_use_canonical_id_discovery() {
    let root = workspace_root();

    let list = run_in(root.as_path(), &["pipeline", "list"]);
    assert!(list.status.success(), "pipeline list should succeed");
    let list_stdout = String::from_utf8(list.stdout).expect("list stdout is utf-8");
    assert!(list_stdout.contains("PIPELINE INVENTORY"));
    assert!(list_stdout.contains("PIPELINE COUNT: 4"));
    assert!(list_stdout.contains("PIPELINE: pipeline.foundation"));
    assert!(list_stdout.contains("SOURCE: core/pipelines/foundation.yaml"));

    let show = run_in(
        root.as_path(),
        &["pipeline", "show", "--id", "pipeline.foundation_inputs"],
    );
    assert!(show.status.success(), "pipeline show should succeed");
    let show_stdout = String::from_utf8(show.stdout).expect("show stdout is utf-8");
    assert!(show_stdout.contains("PIPELINE: pipeline.foundation_inputs"));
    assert!(show_stdout.contains("DEFAULTS:"));
    assert!(show_stdout.contains("SOURCE: core/pipelines/foundation_inputs.yaml"));
    assert!(show_stdout.contains("stage.05_charter_synthesize"));
    assert!(show_stdout.contains("core/stages/05_charter_synthesize.md"));
    assert!(show_stdout.contains("sets: [needs_project_context]"));
    assert!(show_stdout.contains(
        "activation: activation.when.any [variables.charter_gaps_detected == true, variables.needs_project_context == true]"
    ));

    let shorthand = run_in(root.as_path(), &["pipeline", "show", "--id", "00_base"]);
    assert!(shorthand.status.success(), "stage shorthand should resolve");
    let shorthand_stdout = String::from_utf8(shorthand.stdout).expect("stdout is utf-8");
    assert!(shorthand_stdout.contains("STAGE: stage.00_base"));
    assert!(shorthand_stdout.contains("pipeline.foundation"));

    let ambiguous_repo = tempfile::tempdir().expect("tempdir");
    let ambiguous_root = ambiguous_repo.path();
    write_file(
        &ambiguous_root.join("core/stages/alpha.md"),
        b"---\nkind: stage\nid: stage.alpha\nversion: 0.1.0\ntitle: Alpha Stage\ndescription: alpha\n---\n# alpha\n",
    );
    write_file(
        &ambiguous_root.join("core/pipelines/alpha.yaml"),
        b"---\nkind: pipeline\nid: pipeline.alpha\nversion: 0.1.0\ntitle: Alpha Pipeline\ndescription: alpha\n---\ndefaults:\n  runner: codex-cli\n  profile: python-uv\n  enable_complexity: false\nstages:\n  - id: stage.alpha\n    file: core/stages/alpha.md\n",
    );

    let ambiguous = run_in(ambiguous_root, &["pipeline", "show", "--id", "alpha"]);
    assert!(
        !ambiguous.status.success(),
        "ambiguous shorthand should refuse"
    );
    let ambiguous_stdout = String::from_utf8(ambiguous.stdout).expect("stdout is utf-8");
    assert!(ambiguous_stdout.contains("ambiguous selector `alpha`"));
    assert!(ambiguous_stdout.contains("pipeline.alpha"));
    assert!(ambiguous_stdout.contains("stage.alpha"));
    assert!(ambiguous_stdout.contains("NEXT SAFE ACTION"));
    assert!(ambiguous_stdout.contains("rename the conflicting ids"));

    let unknown = run_in(root.as_path(), &["pipeline", "show", "--id", "missing-id"]);
    assert!(!unknown.status.success(), "unknown selector should refuse");
    let unknown_stdout = String::from_utf8(unknown.stdout).expect("stdout is utf-8");
    assert!(unknown_stdout.contains("unknown pipeline selector `missing-id`"));
    assert!(unknown_stdout.contains("pipeline list"));
    assert!(unknown_stdout.contains("NEXT SAFE ACTION"));
    assert!(unknown_stdout.contains("full canonical id"));

    let path_like = run_in(
        root.as_path(),
        &["pipeline", "show", "--id", "core/pipelines/foundation.yaml"],
    );
    assert!(
        !path_like.status.success(),
        "path-like selector should refuse"
    );
    let path_like_stdout = String::from_utf8(path_like.stdout).expect("stdout is utf-8");
    assert!(path_like_stdout.contains("raw file paths are evidence only"));
    assert!(path_like_stdout.contains("NEXT SAFE ACTION"));
    assert!(path_like_stdout.contains("canonical pipeline or stage id"));
}

#[test]
fn pipeline_resolve_and_state_set_use_compiler_route_state_handoff() {
    let (_dir, root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();

    let first_resolve = run_in(
        root.as_path(),
        &["pipeline", "resolve", "--id", "foundation_inputs"],
    );
    assert!(
        first_resolve.status.success(),
        "pipeline resolve should succeed"
    );
    let first_resolve_stdout = String::from_utf8(first_resolve.stdout).expect("stdout is utf-8");
    pipeline_proof_corpus_support::assert_matches_golden(
        &first_resolve_stdout,
        root.as_path(),
        None,
        "resolve.initial.txt",
    );

    let applied = run_in(
        root.as_path(),
        &[
            "pipeline",
            "state",
            "set",
            "--id",
            "foundation_inputs",
            "--var",
            "needs_project_context=true",
        ],
    );
    assert!(
        applied.status.success(),
        "pipeline state set should succeed"
    );
    let applied_stdout = String::from_utf8(applied.stdout).expect("stdout is utf-8");
    pipeline_proof_corpus_support::assert_matches_golden(
        &applied_stdout,
        root.as_path(),
        None,
        "state_set.var.needs_project_context.applied.txt",
    );

    let activation_applied = run_in(
        root.as_path(),
        &[
            "pipeline",
            "state",
            "set",
            "--id",
            "foundation_inputs",
            "--var",
            "charter_gaps_detected=true",
        ],
    );
    assert!(
        activation_applied.status.success(),
        "activation-only route variable should be supported"
    );
    let activation_applied_stdout =
        String::from_utf8(activation_applied.stdout).expect("stdout is utf-8");
    pipeline_proof_corpus_support::assert_matches_golden(
        &activation_applied_stdout,
        root.as_path(),
        None,
        "state_set.var.charter_gaps_detected.applied.txt",
    );

    let second_resolve = run_in(
        root.as_path(),
        &["pipeline", "resolve", "--id", "foundation_inputs"],
    );
    assert!(
        second_resolve.status.success(),
        "pipeline resolve should succeed after mutation"
    );
    let second_resolve_stdout = String::from_utf8(second_resolve.stdout).expect("stdout is utf-8");
    pipeline_proof_corpus_support::assert_matches_golden(
        &second_resolve_stdout,
        root.as_path(),
        None,
        "resolve.after_full_activation.txt",
    );
}

#[test]
fn pipeline_resolve_refuses_incomplete_default_profile_pack() {
    let (_dir, root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    write_incomplete_profile_pack(root.as_path(), "incomplete");
    set_foundation_inputs_default_profile(root.as_path(), "incomplete");

    let output = run_in(
        root.as_path(),
        &["pipeline", "resolve", "--id", "foundation_inputs"],
    );
    assert!(
        !output.status.success(),
        "pipeline resolve should refuse incomplete default profile pack"
    );

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert!(
        stdout.contains("REFUSED: route basis build error:"),
        "{stdout}"
    );
    assert!(stdout.contains("core/profiles/incomplete/"), "{stdout}");
    assert!(stdout.contains("commands.yaml"), "{stdout}");
    assert!(stdout.contains("conventions.md"), "{stdout}");
    assert!(
        !stdout.contains("failed to read route_basis input"),
        "{stdout}"
    );
}

#[test]
fn pipeline_state_set_field_surfaces_accept_run_and_refs() {
    let (_dir, root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();

    let runner_applied = run_in(
        root.as_path(),
        &[
            "pipeline",
            "state",
            "set",
            "--id",
            "foundation_inputs",
            "--field",
            "run.runner=codex-cli",
        ],
    );
    assert!(
        runner_applied.status.success(),
        "run.runner field should succeed"
    );
    let runner_stdout = String::from_utf8(runner_applied.stdout).expect("stdout is utf-8");
    pipeline_proof_corpus_support::assert_matches_golden(
        &runner_stdout,
        root.as_path(),
        None,
        "state_set.field.run_runner.applied.txt",
    );

    let ref_applied = run_in(
        root.as_path(),
        &[
            "pipeline",
            "state",
            "set",
            "--id",
            "foundation_inputs",
            "--field",
            "refs.charter_ref=artifacts/charter/CHARTER.md",
        ],
    );
    assert!(
        ref_applied.status.success(),
        "refs.charter_ref field should succeed"
    );
    let ref_stdout = String::from_utf8(ref_applied.stdout).expect("stdout is utf-8");
    pipeline_proof_corpus_support::assert_matches_golden(
        &ref_stdout,
        root.as_path(),
        None,
        "state_set.field.refs_charter_ref.applied.txt",
    );

    let resolve = run_in(
        root.as_path(),
        &["pipeline", "resolve", "--id", "foundation_inputs"],
    );
    assert!(
        resolve.status.success(),
        "pipeline resolve should surface refs and run fields in route basis"
    );
    let resolve_stdout = String::from_utf8(resolve.stdout).expect("stdout is utf-8");
    pipeline_proof_corpus_support::assert_matches_golden(
        &resolve_stdout,
        root.as_path(),
        None,
        "resolve.after_run_and_refs.txt",
    );
}

#[test]
fn pipeline_compile_feature_spec_payload_matches_shared_golden() {
    let (_dir, root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    prepare_foundation_inputs_full_context_route_basis(root.as_path());

    let output = run_in_with_env(
        root.as_path(),
        &[
            "pipeline",
            "compile",
            "--id",
            "foundation_inputs",
            "--stage",
            "10_feature_spec",
        ],
        &[(
            handbook_pipeline::pipeline_compile::PIPELINE_COMPILE_NOW_UTC_ENV_VAR,
            FIXED_NOW_UTC,
        )],
    );
    assert!(output.status.success(), "pipeline compile should succeed");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    pipeline_proof_corpus_support::assert_matches_golden(
        &stdout,
        root.as_path(),
        None,
        "compile.stage_10_feature_spec.payload.full_context.txt",
    );
    assert!(
        !stdout.contains("OUTCOME:"),
        "plain compile must stay payload-only: {stdout}"
    );
    assert!(
        !stdout.contains("NEXT SAFE ACTION:"),
        "plain compile must not include refusal framing: {stdout}"
    );
    assert!(
        !stdout.contains("ROUTE BASIS:"),
        "plain compile must not include proof sections: {stdout}"
    );
}

#[test]
fn pipeline_compile_ignores_unrelated_malformed_stage_files() {
    let (_dir, root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    prepare_foundation_inputs_full_context_route_basis(root.as_path());
    std::fs::write(
        root.join("core/stages/99_bad.md"),
        r#"---
kind: nonsense
id: stage.99_bad
version: 0.1.0
title: Bad Stage
description: malformed and unrelated
---
# bad
"#,
    )
    .expect("write unrelated malformed stage");

    let output = run_in_with_env(
        root.as_path(),
        &[
            "pipeline",
            "compile",
            "--id",
            "foundation_inputs",
            "--stage",
            "10_feature_spec",
        ],
        &[(
            handbook_pipeline::pipeline_compile::PIPELINE_COMPILE_NOW_UTC_ENV_VAR,
            FIXED_NOW_UTC,
        )],
    );
    assert!(
        output.status.success(),
        "pipeline compile should ignore unrelated malformed stages"
    );

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    pipeline_proof_corpus_support::assert_matches_golden(
        &stdout,
        root.as_path(),
        None,
        "compile.stage_10_feature_spec.payload.full_context.txt",
    );
}

#[test]
fn pipeline_compile_feature_spec_explain_matches_shared_golden() {
    let (_dir, root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    prepare_foundation_inputs_full_context_route_basis(root.as_path());

    let output = run_in_with_env(
        root.as_path(),
        &[
            "pipeline",
            "compile",
            "--id",
            "pipeline.foundation_inputs",
            "--stage",
            "stage.10_feature_spec",
            "--explain",
        ],
        &[(
            handbook_pipeline::pipeline_compile::PIPELINE_COMPILE_NOW_UTC_ENV_VAR,
            FIXED_NOW_UTC,
        )],
    );
    assert!(
        output.status.success(),
        "pipeline compile --explain should succeed"
    );

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    pipeline_proof_corpus_support::assert_matches_golden(
        &stdout,
        root.as_path(),
        None,
        "compile.stage_10_feature_spec.explain.full_context.txt",
    );
    assert!(
        stdout.contains("OUTCOME: COMPILED"),
        "explain mode should render proof: {stdout}"
    );
    assert!(
        !stdout.starts_with("# stage.10_feature_spec"),
        "explain mode must not include the payload header: {stdout}"
    );
}

#[test]
fn pipeline_compile_refuses_when_required_variable_is_missing() {
    let (_dir, root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    let stage_path = root.join("core/stages/10_feature_spec.md");
    let stage = std::fs::read_to_string(&stage_path).expect("read stage");
    let updated_stage = stage.replace("    - project_name?\n", "    - project_name\n");
    assert_ne!(
        stage, updated_stage,
        "stage fixture should be updated for the test"
    );
    std::fs::write(&stage_path, updated_stage).expect("write stage");
    prepare_foundation_inputs_compile_ready_route_basis(root.as_path());

    let output = run_in_with_env(
        root.as_path(),
        &[
            "pipeline",
            "compile",
            "--id",
            "foundation_inputs",
            "--stage",
            "10_feature_spec",
        ],
        &[(
            handbook_pipeline::pipeline_compile::PIPELINE_COMPILE_NOW_UTC_ENV_VAR,
            FIXED_NOW_UTC,
        )],
    );
    assert!(
        !output.status.success(),
        "missing required variable should refuse"
    );

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    pipeline_proof_corpus_support::assert_matches_golden(
        &stdout,
        root.as_path(),
        None,
        "compile.refused.missing_required_variable.txt",
    );
}

#[cfg(unix)]
#[test]
fn pipeline_compile_refuses_symlinked_required_artifact_input() {
    use std::os::unix::fs::symlink;

    let (_dir, root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    prepare_foundation_inputs_full_context_route_basis(root.as_path());

    let outside_dir = tempfile::tempdir().expect("outside tempdir");
    let outside_secret = outside_dir.path().join("system-review-secret.txt");
    std::fs::write(&outside_secret, "outside-secret").expect("write secret");

    let artifact = root.join("artifacts/base/BASE_CONTEXT.md");
    std::fs::remove_file(&artifact).expect("remove artifact");
    symlink(&outside_secret, &artifact).expect("symlink artifact");

    let output = run_in_with_env(
        root.as_path(),
        &[
            "pipeline",
            "compile",
            "--id",
            "foundation_inputs",
            "--stage",
            "10_feature_spec",
        ],
        &[(
            handbook_pipeline::pipeline_compile::PIPELINE_COMPILE_NOW_UTC_ENV_VAR,
            FIXED_NOW_UTC,
        )],
    );
    assert!(
        !output.status.success(),
        "symlinked required artifact should refuse"
    );

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert!(stdout.contains("OUTCOME: REFUSED"));
    assert!(!stdout.contains("# stage.10_feature_spec"));
    assert!(stdout.contains("artifacts/base/BASE_CONTEXT.md"));
    assert!(!stdout.contains("outside-secret"));
}

#[test]
fn pipeline_compile_refuses_missing_route_basis() {
    let (_dir, root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();

    let output = run_in(
        root.as_path(),
        &[
            "pipeline",
            "compile",
            "--id",
            "foundation_inputs",
            "--stage",
            "10_feature_spec",
        ],
    );
    assert!(
        !output.status.success(),
        "pipeline compile should refuse without a persisted route basis"
    );

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    pipeline_proof_corpus_support::assert_matches_golden(
        &stdout,
        root.as_path(),
        None,
        "compile.refused.missing_route_basis.txt",
    );
}

#[test]
fn pipeline_compile_refuses_inactive_stage() {
    let (_dir, root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();

    let resolve = run_in(
        root.as_path(),
        &["pipeline", "resolve", "--id", "foundation_inputs"],
    );
    assert!(resolve.status.success(), "pipeline resolve should succeed");

    let output = run_in(
        root.as_path(),
        &[
            "pipeline",
            "compile",
            "--id",
            "pipeline.foundation_inputs",
            "--stage",
            "stage.10_feature_spec",
        ],
    );
    assert!(
        !output.status.success(),
        "inactive-stage compile should refuse"
    );

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    pipeline_proof_corpus_support::assert_matches_golden(
        &stdout,
        root.as_path(),
        None,
        "compile.refused.inactive_stage.txt",
    );
}

#[test]
fn pipeline_compile_refuses_stale_route_basis_after_state_set() {
    let (_dir, root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    prepare_foundation_inputs_compile_ready_route_basis(root.as_path());

    let mutation = run_in(
        root.as_path(),
        &[
            "pipeline",
            "state",
            "set",
            "--id",
            "foundation_inputs",
            "--field",
            "refs.charter_ref=artifacts/charter/CHARTER.md",
        ],
    );
    assert!(mutation.status.success(), "state mutation should succeed");

    let output = run_in(
        root.as_path(),
        &[
            "pipeline",
            "compile",
            "--id",
            "foundation_inputs",
            "--stage",
            "10_feature_spec",
        ],
    );
    assert!(!output.status.success(), "stale route basis should refuse");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    pipeline_proof_corpus_support::assert_matches_golden(
        &stdout,
        root.as_path(),
        None,
        "compile.refused.stale_route_basis.txt",
    );
}

#[test]
fn pipeline_compile_refuses_malformed_route_basis() {
    let (_dir, root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    pipeline_proof_corpus_support::install_state_seed(root.as_path(), "malformed_route_basis.yaml");
    let state_path = root.join(".handbook/state/pipeline/pipeline.foundation_inputs.yaml");
    let state = std::fs::read_to_string(&state_path).expect("malformed route-basis state");
    let absolute_root = root.to_string_lossy().replace('\\', "/");
    std::fs::write(
        &state_path,
        state.replace("/tmp/foundation-inputs", &absolute_root),
    )
    .expect("normalize seeded repo root for native platform");

    let output = run_in(
        root.as_path(),
        &[
            "pipeline",
            "compile",
            "--id",
            "pipeline.foundation_inputs",
            "--stage",
            "stage.10_feature_spec",
        ],
    );
    assert!(
        !output.status.success(),
        "malformed route basis should refuse"
    );

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    pipeline_proof_corpus_support::assert_matches_golden(
        &stdout,
        root.as_path(),
        None,
        "compile.refused.malformed_route_basis.txt",
    );
}

#[test]
fn pipeline_compile_keeps_non_route_basis_unknown_top_level_keys_as_invalid_state() {
    let (_dir, root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    prepare_foundation_inputs_compile_ready_route_basis(root.as_path());

    let state_path = root
        .join(".handbook")
        .join("state")
        .join("pipeline")
        .join("pipeline.foundation_inputs.yaml");
    let state = std::fs::read_to_string(&state_path).expect("state file");
    std::fs::write(&state_path, format!("{state}\nroute_basis_extra: true\n"))
        .expect("write malformed state");

    let output = run_in(
        root.as_path(),
        &[
            "pipeline",
            "compile",
            "--id",
            "pipeline.foundation_inputs",
            "--stage",
            "stage.10_feature_spec",
        ],
    );
    assert!(
        !output.status.success(),
        "unknown top-level key should refuse as invalid state"
    );

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert!(stdout.contains("OUTCOME: REFUSED"), "{stdout}");
    assert!(stdout.contains("REASON: invalid_state:"), "{stdout}");
    assert!(
        stdout.contains("trusted pipeline session state at "),
        "{stdout}"
    );
    assert!(stdout.contains("unknown field"), "{stdout}");
    assert!(stdout.contains("route_basis_extra"), "{stdout}");
    assert!(
        stdout.contains(
            "NEXT SAFE ACTION: fix the persisted route state and retry `pipeline compile`; then retry `handbook pipeline compile --id pipeline.foundation_inputs --stage stage.10_feature_spec`"
        ),
        "{stdout}"
    );
}

#[test]
fn pipeline_compile_refuses_forged_route_basis_status() {
    let (_dir, root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();

    let resolve = run_in(
        root.as_path(),
        &["pipeline", "resolve", "--id", "foundation_inputs"],
    );
    assert!(resolve.status.success(), "pipeline resolve should succeed");

    let state_path = root
        .join(".handbook")
        .join("state")
        .join("pipeline")
        .join("pipeline.foundation_inputs.yaml");
    let state = std::fs::read_to_string(&state_path).expect("state file");
    let forged = state.replace(
        "  - stage_id: stage.10_feature_spec\n    file: core/stages/10_feature_spec.md\n    status: blocked\n    reason:\n      kind: blocked_by_unresolved_stage\n      upstream_stage_id: stage.06_project_context_interview\n      upstream_status: next\n",
        "  - stage_id: stage.10_feature_spec\n    file: core/stages/10_feature_spec.md\n    status: active\n    reason: null\n",
    );
    assert_ne!(
        state, forged,
        "route_basis fixture should be updated for the test"
    );
    std::fs::write(&state_path, forged).expect("write forged state");

    let output = run_in(
        root.as_path(),
        &[
            "pipeline",
            "compile",
            "--id",
            "pipeline.foundation_inputs",
            "--stage",
            "stage.10_feature_spec",
        ],
    );
    assert!(!output.status.success(), "forged route basis should refuse");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    pipeline_proof_corpus_support::assert_matches_golden(
        &stdout,
        root.as_path(),
        None,
        "compile.refused.malformed_route_basis_forged_status.txt",
    );
}

#[test]
fn pipeline_compile_refuses_stage_not_in_pipeline() {
    let (_dir, root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    prepare_foundation_inputs_compile_ready_route_basis(root.as_path());

    let pipeline_path = root.join("core/pipelines/foundation_inputs.yaml");
    let pipeline = std::fs::read_to_string(&pipeline_path).expect("read pipeline");
    let updated_pipeline = pipeline.replace(
        "  - id: stage.10_feature_spec\n    file: core/stages/10_feature_spec.md\n",
        "",
    );
    assert_ne!(
        pipeline, updated_pipeline,
        "pipeline fixture should be updated for the test"
    );
    std::fs::write(&pipeline_path, updated_pipeline).expect("write pipeline");

    let output = run_in(
        root.as_path(),
        &[
            "pipeline",
            "compile",
            "--id",
            "pipeline.foundation_inputs",
            "--stage",
            "stage.10_feature_spec",
        ],
    );
    assert!(
        !output.status.success(),
        "missing declared stage should refuse"
    );

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    pipeline_proof_corpus_support::assert_matches_golden(
        &stdout,
        root.as_path(),
        None,
        "compile.refused.stage_not_in_pipeline.txt",
    );
}

#[test]
fn pipeline_compile_refuses_missing_required_artifact() {
    let (_dir, root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    prepare_foundation_inputs_compile_ready_route_basis(root.as_path());
    std::fs::remove_file(root.join("artifacts/base/BASE_CONTEXT.md"))
        .expect("remove required artifact");

    let output = run_in_with_env(
        root.as_path(),
        &[
            "pipeline",
            "compile",
            "--id",
            "pipeline.foundation_inputs",
            "--stage",
            "stage.10_feature_spec",
        ],
        &[(
            handbook_pipeline::pipeline_compile::PIPELINE_COMPILE_NOW_UTC_ENV_VAR,
            FIXED_NOW_UTC,
        )],
    );
    assert!(
        !output.status.success(),
        "missing required artifact should refuse"
    );

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    pipeline_proof_corpus_support::assert_matches_golden(
        &stdout,
        root.as_path(),
        None,
        "compile.refused.missing_required_artifact.txt",
    );
}

#[test]
fn pipeline_compile_allows_optional_artifacts_to_be_absent() {
    let (_dir, root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    prepare_foundation_inputs_compile_ready_route_basis(root.as_path());
    for path in [
        "artifacts/project_context/PROJECT_CONTEXT.md",
        "artifacts/foundation/FOUNDATION_STRATEGY.md",
        "artifacts/foundation/TECH_ARCH_BRIEF.md",
    ] {
        std::fs::remove_file(root.join(path)).expect("remove optional artifact");
    }

    let canonical_root = canonical_repo_root(root.as_path());
    let expected = handbook_pipeline::pipeline_compile::compile_pipeline_stage_with_runtime(
        canonical_root.as_path(),
        "pipeline.foundation_inputs",
        "stage.10_feature_spec",
        &handbook_pipeline::pipeline_compile::PipelineCompileRuntimeContext {
            now_utc_override: Some(FIXED_NOW_UTC.to_string()),
        },
    )
    .map(|result| handbook_pipeline::pipeline_compile::render_pipeline_compile_payload(&result))
    .expect("compile should succeed");

    let output = run_in_with_env(
        root.as_path(),
        &[
            "pipeline",
            "compile",
            "--id",
            "pipeline.foundation_inputs",
            "--stage",
            "stage.10_feature_spec",
        ],
        &[(
            handbook_pipeline::pipeline_compile::PIPELINE_COMPILE_NOW_UTC_ENV_VAR,
            FIXED_NOW_UTC,
        )],
    );
    assert!(
        output.status.success(),
        "optional artifacts should remain a success path"
    );

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert_eq!(stdout, format!("{expected}\n"));
    assert!(
        !stdout.contains("OUTCOME:"),
        "plain compile must stay payload-only: {stdout}"
    );
}

#[test]
fn pipeline_state_set_field_rejects_invalid_paths_and_values() {
    let (_dir, root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();

    let invalid_path = run_in(
        root.as_path(),
        &[
            "pipeline",
            "state",
            "set",
            "--id",
            "foundation_inputs",
            "--field",
            "refs.unknown=artifacts/charter/CHARTER.md",
        ],
    );
    assert!(
        !invalid_path.status.success(),
        "invalid field path should refuse"
    );
    let invalid_path_stdout = String::from_utf8(invalid_path.stdout).expect("stdout is utf-8");
    assert_eq!(
        invalid_path_stdout.trim_end(),
        "REFUSED: unsupported --field path `refs.unknown`; expected one of `run.runner`, `run.profile`, `refs.charter_ref`, or `refs.project_context_ref`"
    );

    let derived_path = run_in(
        root.as_path(),
        &[
            "pipeline",
            "state",
            "set",
            "--id",
            "foundation_inputs",
            "--field",
            "run.repo_root=/tmp/repo",
        ],
    );
    assert!(
        !derived_path.status.success(),
        "derived run.repo_root field should refuse"
    );
    let derived_path_stdout = String::from_utf8(derived_path.stdout).expect("stdout is utf-8");
    assert_eq!(
        derived_path_stdout.trim_end(),
        "REFUSED: unsupported --field path `run.repo_root`; expected one of `run.runner`, `run.profile`, `refs.charter_ref`, or `refs.project_context_ref`"
    );

    let invalid_value = run_in(
        root.as_path(),
        &[
            "pipeline",
            "state",
            "set",
            "--id",
            "foundation_inputs",
            "--field",
            "refs.charter_ref=/tmp/CHARTER.md",
        ],
    );
    assert!(
        !invalid_value.status.success(),
        "invalid field value should refuse"
    );
    let invalid_value_stdout = String::from_utf8(invalid_value.stdout).expect("stdout is utf-8");
    assert_eq!(
        invalid_value_stdout.trim_end(),
        "REFUSED: route state mutation error: route state mutation is invalid: repo-relative ref `/tmp/CHARTER.md` must be a clean repo-relative path"
    );
}

#[test]
fn pipeline_state_set_field_rejects_incomplete_profile_pack() {
    let (_dir, root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    write_incomplete_profile_pack(root.as_path(), "incomplete");

    let output = run_in(
        root.as_path(),
        &[
            "pipeline",
            "state",
            "set",
            "--id",
            "foundation_inputs",
            "--field",
            "run.profile=incomplete",
        ],
    );
    assert!(
        !output.status.success(),
        "incomplete profile pack should refuse"
    );

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert!(
        stdout.contains("REFUSED: route state mutation error:"),
        "{stdout}"
    );
    assert!(stdout.contains("run.profile `incomplete`"), "{stdout}");
    assert!(stdout.contains("core/profiles/incomplete/"), "{stdout}");
    assert!(stdout.contains("commands.yaml"), "{stdout}");
    assert!(stdout.contains("conventions.md"), "{stdout}");
}

#[test]
fn pipeline_list_and_show_ignore_activation_drift_during_inventory_inspection() {
    let (_dir, root) = activation_drift_pipeline_repo();

    for args in [
        vec!["pipeline", "list"],
        vec!["pipeline", "show", "--id", "pipeline.drift"],
        vec!["pipeline", "show", "--id", "stage.00_base"],
    ] {
        let output = run_in(root.as_path(), &args);
        assert!(
            output.status.success(),
            "command should succeed: {:?}",
            args
        );

        let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
        assert!(
            !stdout.contains("has activation drift"),
            "inventory command should not surface activation drift for {:?}: {stdout}",
            args
        );
    }
}

#[test]
fn pipeline_list_and_show_ignore_unrelated_broken_stage_files_during_inventory_inspection() {
    let (_dir, root) = unused_bad_stage_repo();

    for args in [
        vec!["pipeline", "list"],
        vec!["pipeline", "show", "--id", "pipeline.foundation"],
        vec!["pipeline", "show", "--id", "stage.00_base"],
    ] {
        let output = run_in(root.as_path(), &args);
        assert!(
            output.status.success(),
            "command should succeed: {:?}",
            args
        );

        let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
        assert!(
            !stdout.contains("pipeline catalog error"),
            "unexpected refusal: {stdout}"
        );
        assert!(stdout.contains("pipeline.foundation") || stdout.contains("stage.00_base"));
    }
}

#[test]
fn pipeline_resolve_and_state_set_still_refuse_activation_drift_before_route_evaluation() {
    let (_dir, root) = activation_drift_pipeline_repo();

    for args in [
        vec!["pipeline", "resolve", "--id", "pipeline.drift"],
        vec![
            "pipeline",
            "state",
            "set",
            "--id",
            "pipeline.drift",
            "--var",
            "needs_project_context=true",
        ],
    ] {
        let output = run_in(root.as_path(), &args);
        assert!(
            !output.status.success(),
            "route-aware command should refuse: {:?}",
            args
        );

        let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
        assert!(
            stdout.contains("REFUSED: pipeline definition error: invalid pipeline definition"),
            "expected pipeline definition refusal for {:?}: {stdout}",
            args
        );
        assert!(
            stdout.contains("has activation drift"),
            "expected activation drift for {:?}: {stdout}",
            args
        );
        assert!(
            stdout.contains("stage `stage.00_base` file `core/stages/00_base.md`"),
            "expected stage identification for {:?}: {stdout}",
            args
        );
    }
}

#[test]
fn pipeline_list_and_resolve_omit_unrelated_invalid_pipeline_ids() {
    let (_dir, root) = invalid_pipeline_id_repo();

    let output = run_in(root.as_path(), &["pipeline", "list"]);
    assert!(output.status.success(), "pipeline list should succeed");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert!(stdout.contains("PIPELINE: pipeline.foundation"));
    assert!(
        !stdout.contains("PIPELINE: pipeline.bad/path"),
        "invalid pipeline id must not be advertised: {stdout}"
    );

    let resolve = run_in(
        root.as_path(),
        &["pipeline", "resolve", "--id", "pipeline.foundation"],
    );
    assert!(resolve.status.success(), "pipeline resolve should succeed");
    let resolve_stdout = String::from_utf8(resolve.stdout).expect("stdout is utf-8");
    assert!(
        resolve_stdout.contains("OUTCOME: RESOLVED")
            && resolve_stdout.contains("PIPELINE: pipeline.foundation"),
        "expected successful resolve output: {resolve_stdout}"
    );
}

#[test]
fn pipeline_show_still_refuses_when_selected_pipeline_has_broken_stage_metadata() {
    let (_dir, root) = selected_broken_pipeline_repo();

    let healthy = run_in(
        root.as_path(),
        &["pipeline", "show", "--id", "pipeline.foundation"],
    );
    assert!(
        healthy.status.success(),
        "healthy pipeline show should succeed"
    );

    let broken = run_in(
        root.as_path(),
        &["pipeline", "show", "--id", "pipeline.broken"],
    );
    assert!(
        !broken.status.success(),
        "selected broken pipeline should refuse"
    );
    let stdout = String::from_utf8(broken.stdout).expect("stdout is utf-8");
    assert!(
        stdout.contains("REFUSED: pipeline catalog error: stage front matter"),
        "expected stage metadata refusal: {stdout}"
    );
    assert!(
        stdout.contains("must declare kind `stage`, got `nonsense`"),
        "expected stage kind detail: {stdout}"
    );
}

#[test]
fn pipeline_state_set_preserves_distinct_refusals() {
    let (_dir, root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    let state_path = pipeline_proof_corpus_support::install_state_seed(
        root.as_path(),
        "malformed_route_state.yaml",
    );

    let malformed = run_in(
        root.as_path(),
        &[
            "pipeline",
            "state",
            "set",
            "--id",
            "foundation_inputs",
            "--var",
            "needs_project_context=true",
        ],
    );
    assert!(
        !malformed.status.success(),
        "malformed route state should refuse"
    );
    let malformed_stdout = String::from_utf8(malformed.stdout).expect("stdout is utf-8");
    pipeline_proof_corpus_support::assert_matches_golden(
        &malformed_stdout,
        root.as_path(),
        Some(&state_path),
        "state_set.refused.malformed_route_state.txt",
    );

    pipeline_proof_corpus_support::install_state_seed(
        root.as_path(),
        "revision_conflict_state.yaml",
    );

    let revision_conflict = run_in(
        root.as_path(),
        &[
            "pipeline",
            "state",
            "set",
            "--id",
            "foundation_inputs",
            "--var",
            "needs_project_context=false",
            "--expected-revision",
            "0",
        ],
    );
    let revision_conflict_stdout =
        String::from_utf8(revision_conflict.stdout).expect("stdout is utf-8");
    assert!(
        !revision_conflict.status.success(),
        "revision conflict should refuse: {revision_conflict_stdout}"
    );
    pipeline_proof_corpus_support::assert_matches_golden(
        &revision_conflict_stdout,
        root.as_path(),
        None,
        "state_set.refused.revision_conflict.txt",
    );

    let unsupported = run_in(
        root.as_path(),
        &[
            "pipeline",
            "state",
            "set",
            "--id",
            "foundation_inputs",
            "--var",
            "unsupported_flag=true",
        ],
    );
    assert!(
        !unsupported.status.success(),
        "unsupported variable should refuse"
    );
    let unsupported_stdout = String::from_utf8(unsupported.stdout).expect("stdout is utf-8");
    pipeline_proof_corpus_support::assert_matches_golden(
        &unsupported_stdout,
        root.as_path(),
        None,
        "state_set.refused.unsupported_variable.txt",
    );
}

#[test]
fn setup_help_matches_snapshot() {
    assert_help_snapshot(&["setup", "--help"], "handbook-setup-help.txt");
}

#[test]
fn setup_init_help_matches_snapshot() {
    assert_help_snapshot(&["setup", "init", "--help"], "handbook-setup-init-help.txt");
}

#[test]
fn setup_refresh_help_matches_snapshot() {
    assert_help_snapshot(
        &["setup", "refresh", "--help"],
        "handbook-setup-refresh-help.txt",
    );
}

#[test]
fn author_help_matches_snapshot() {
    let output = run_in(workspace_root().as_path(), &["author", "--help"]);
    assert!(output.status.success());
    let rendered = String::from_utf8(output.stdout).expect("help is utf-8");
    let rendered = rendered.replace("handbook.exe", "handbook");
    assert_in_order(
        &rendered,
        &[
            "Usage: handbook author [COMMAND]",
            "charter          Create, approve, promote, or validate the selected canonical Charter",
            "project-context  Deterministically author canonical `.handbook/project/context.yaml`",
        ],
    );
    assert!(!rendered.contains(".handbook/charter/CHARTER.md"));
}

#[test]
fn author_charter_help_matches_snapshot() {
    let output = run_in(workspace_root().as_path(), &["author", "charter", "--help"]);
    assert!(output.status.success());
    let rendered = String::from_utf8(output.stdout).expect("help is utf-8");
    let rendered = rendered.replace("handbook.exe", "handbook");
    assert!(rendered.contains("Usage: handbook author charter [OPTIONS]"));
    assert_in_order(
        &rendered,
        &[
            "--mode <MODE>",
            "--from-inputs <path|->",
            "--expected-current-fingerprint <EXPECTED_CURRENT_FINGERPRINT>",
            "--approve-candidate <APPROVE_CANDIDATE>",
            "--approval-class <APPROVAL_CLASS>",
            "--authority-ref <AUTHORITY_REF>",
            "--accept-waiver-ref <ACCEPT_WAIVER_REFS>",
            "--promote-candidate <PROMOTE_CANDIDATE>",
            "--approval-ref <APPROVAL_REF>",
            "--validate",
            "--json",
        ],
    );
    assert!(!rendered.contains(".handbook/charter/CHARTER.md"));
    assert!(!rendered.contains("--interactive"));
    assert!(!rendered.contains("--yes"));
}

#[test]
fn author_project_context_help_matches_snapshot() {
    assert_help_snapshot(
        &["author", "project-context", "--help"],
        "handbook-author-project-context-help.txt",
    );
}

#[test]
fn profile_setup_and_doctor_use_typed_rows_json_and_exit_policy() {
    let (outcome, reason, json_status) = (
        "OUTCOME: ACTION_REQUIRED",
        "environment_context [.handbook/project/environment.yaml] ACTION: optional_absent STATUS: missing REASON: optional_path_missing",
        "action_required",
    );

    let setup_repo = tempfile::tempdir().expect("tempdir");
    let setup = run_in(setup_repo.path(), &["setup"]);
    assert!(!setup.status.success());
    let setup_stdout = String::from_utf8(setup.stdout).expect("setup stdout utf-8");
    assert!(setup_stdout.contains(outcome), "{setup_stdout}");
    assert!(setup_stdout.contains(reason), "{setup_stdout}");
    assert!(setup_repo.path().join(".handbook").is_dir());
    let identity = fs::read(setup_repo.path().join(".handbook/repository-identity.v1"))
        .expect("setup repository identity");
    assert_eq!(identity.len(), 71);
    assert!(identity.starts_with(b"sha256:"));
    assert!(!setup_repo
        .path()
        .join(".handbook/project/charter.yaml")
        .exists());
    assert!(!setup_repo
        .path()
        .join(".handbook/charter/CHARTER.md")
        .exists());

    let doctor_repo = tempfile::tempdir().expect("tempdir");
    let text = run_in(doctor_repo.path(), &["doctor"]);
    assert!(!text.status.success());
    let text_stdout = String::from_utf8(text.stdout).expect("doctor stdout utf-8");
    assert!(text_stdout.contains("environment_context [.handbook/project/environment.yaml] APPLICABILITY: optional STATUS: missing REASON: optional_path_missing"), "{text_stdout}");

    let json = run_in(doctor_repo.path(), &["doctor", "--json"]);
    assert!(!json.status.success());
    let json_stdout = String::from_utf8(json.stdout).expect("doctor json utf-8");
    assert!(json_stdout.ends_with('\n'));
    assert!(!json_stdout.ends_with("\n\n"));
    let value: serde_json::Value = serde_json::from_str(&json_stdout).expect("doctor json");
    assert_eq!(value["schema_id"], "handbook.repository-doctor-report");
    assert_eq!(value["schema_version"], "1.2.0");
    assert_eq!(value["status"], json_status);
}

#[cfg(unix)]
#[test]
fn profile_doctor_discovers_repository_root_from_nested_directory() {
    let repo = tempfile::tempdir().expect("tempdir");
    std::fs::create_dir(repo.path().join(".git")).expect("git root");
    std::fs::create_dir_all(repo.path().join(".handbook/project/charter.yaml"))
        .expect("non-regular selected path");
    let nested = repo.path().join("nested/work");
    std::fs::create_dir_all(&nested).expect("nested cwd");

    let output = binary_in(&nested)
        .arg("doctor")
        .output()
        .expect("doctor should run");

    assert!(!output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout utf-8");
    assert!(stdout.contains("OUTCOME: INVALID"), "{stdout}");
    assert!(
        stdout.contains("project_authority [.handbook/project/charter.yaml] APPLICABILITY: required STATUS: unsafe_path REASON: non_regular_file_refused"),
        "{stdout}"
    );
}

#[cfg(unix)]
#[test]
fn profile_doctor_does_not_cross_nested_git_repository_boundary() {
    let outer = tempfile::tempdir().expect("tempdir");
    std::fs::create_dir(outer.path().join(".git")).expect("outer git root");
    std::fs::create_dir_all(outer.path().join(".handbook/project/charter.yaml"))
        .expect("outer non-regular selected path");
    let inner = outer.path().join("inner");
    std::fs::create_dir_all(inner.join(".git")).expect("inner git root");
    let nested = inner.join("nested/work");
    std::fs::create_dir_all(&nested).expect("nested cwd");

    let output = binary_in(&nested)
        .arg("doctor")
        .output()
        .expect("doctor should run");

    assert!(!output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout utf-8");
    assert!(
        stdout.contains("project_authority [.handbook/project/charter.yaml] APPLICABILITY: required STATUS: missing REASON: required_path_missing"),
        "{stdout}"
    );
    assert!(!stdout.contains("non_regular_file_refused"), "{stdout}");
}

#[test]
fn profile_setup_auto_refresh_preserves_existing_root_without_artifact_writes() {
    let repo = tempfile::tempdir().expect("tempdir");
    std::fs::create_dir_all(repo.path().join(".handbook")).expect("handbook root");
    std::fs::write(repo.path().join(".handbook/KEEP"), b"keep\n").expect("sentinel");

    let output = run_in(repo.path(), &["setup"]);

    assert!(!output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout utf-8");
    assert!(stdout.contains("OUTCOME: ACTION_REQUIRED"), "{stdout}");
    assert!(stdout.contains("MODE: refresh"), "{stdout}");
    assert!(stdout.contains("ROOT ACTION: preserve"), "{stdout}");
    assert_eq!(
        std::fs::read(repo.path().join(".handbook/KEEP")).expect("sentinel"),
        b"keep\n"
    );
    for selected in [
        ".handbook/project/charter.yaml",
        ".handbook/project/context.yaml",
        ".handbook/project/environment.yaml",
    ] {
        assert!(!repo.path().join(selected).exists(), "{selected}");
    }
}

#[test]
fn shipped_indeterminate_reset_request_mutates_no_runtime_state() {
    let repo = tempfile::tempdir().expect("tempdir");
    std::fs::create_dir_all(repo.path().join(".handbook/state")).expect("state root");
    std::fs::write(repo.path().join(".handbook/state/a.yaml"), b"a: 1\n").expect("state file");

    let output = run_in(repo.path(), &["setup", "refresh", "--reset-state"]);

    assert!(!output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout utf-8");
    assert!(stdout.contains("OUTCOME: ACTION_REQUIRED"), "{stdout}");
    assert!(stdout.contains("RESET APPLIED: yes"), "{stdout}");
    assert!(!repo.path().join(".handbook/state/a.yaml").exists());
}

#[cfg(unix)]
#[test]
fn profile_reset_symlink_refusal_is_fail_safe() {
    use std::os::unix::fs::symlink;

    let repo = tempfile::tempdir().expect("tempdir");
    std::fs::create_dir_all(repo.path().join(".handbook/state")).expect("state root");
    std::fs::write(repo.path().join(".handbook/state/a.yaml"), b"a: 1\n").expect("state file");
    let outside = tempfile::tempdir().expect("outside");
    symlink(outside.path(), repo.path().join(".handbook/state/z_link")).expect("state symlink");

    let output = run_in(repo.path(), &["setup", "refresh", "--reset-state"]);

    assert!(!output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout utf-8");
    assert!(stdout.contains("OUTCOME: ERROR"), "{stdout}");
    assert!(stdout.contains("CATEGORY: runtime_state_plan"), "{stdout}");
    assert!(
        stdout.contains("REASON: runtime_state_target_unsafe"),
        "{stdout}"
    );
    assert_eq!(
        std::fs::read(repo.path().join(".handbook/state/a.yaml")).expect("state file"),
        b"a: 1\n"
    );
    assert!(repo.path().join(".handbook/state/z_link").exists());
}

#[test]
#[cfg(any())]
fn setup_init_refuses_when_canonical_system_already_exists() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();
    write_file(&root.join(".handbook/charter/CHARTER.md"), b"charter\n");
    write_file(
        &root.join(".handbook/feature_spec/FEATURE_SPEC.md"),
        b"feature\n",
    );

    let output = run_in(root, &["setup", "init"]);
    assert!(!output.status.success(), "setup init should refuse");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert_first_three_lines(
        &stdout,
        [
            "OUTCOME: REFUSED",
            "OBJECT: setup",
            "NEXT SAFE ACTION: run `handbook setup refresh`",
        ],
    );
    assert!(stdout.contains("## REFUSAL"), "{stdout}");
    assert!(stdout.contains("CATEGORY: AlreadyInitialized"), "{stdout}");
    assert!(
        stdout.contains(
            "SUMMARY: canonical .handbook root already exists; use `handbook setup refresh` instead"
        ),
        "{stdout}"
    );
    assert!(
        stdout.contains("BROKEN SUBJECT: canonical `.handbook` root"),
        "{stdout}"
    );
}

#[test]
fn generate_retry_after_repair_clears_missing_root_refusal() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    let first = run_in(root, &["generate"]);
    assert!(!first.status.success(), "initial generate should fail");
    let first_stdout = String::from_utf8(first.stdout).expect("stdout is utf-8");
    assert_first_three_lines(
        &first_stdout,
        [
            "OUTCOME: REFUSED",
            "OBJECT: planning.packet",
            "NEXT SAFE ACTION: run `handbook setup`",
        ],
    );

    repair_to_ready(root);

    let second = run_in(root, &["generate"]);
    assert!(
        second.status.success(),
        "generate should succeed after repair"
    );
    let second_stdout = String::from_utf8(second.stdout).expect("stdout is utf-8");
    assert_first_three_lines(
        &second_stdout,
        [
            "OUTCOME: READY",
            "OBJECT: planning.packet",
            "NEXT SAFE ACTION: run `handbook inspect --packet planning.packet` for proof",
        ],
    );
    assert!(second_stdout.contains("## PACKET BODY"));
    assert!(second_stdout.contains("### CHARTER"));
    assert!(second_stdout.contains("### PROJECT_CONTEXT"));
    assert!(!second_stdout.contains("### FEATURE_SPEC"));
}

#[test]
fn inspect_retry_after_repair_clears_missing_root_refusal() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    let first = run_in(root, &["inspect"]);
    assert!(!first.status.success(), "initial inspect should fail");
    let first_stdout = String::from_utf8(first.stdout).expect("stdout is utf-8");
    assert_first_three_lines(
        &first_stdout,
        [
            "OUTCOME: REFUSED",
            "OBJECT: planning.packet",
            "NEXT SAFE ACTION: run `handbook setup`",
        ],
    );

    repair_to_ready(root);

    let second = run_in(root, &["inspect"]);
    assert!(
        second.status.success(),
        "inspect should succeed after repair"
    );
    let second_stdout = String::from_utf8(second.stdout).expect("stdout is utf-8");
    assert_first_three_lines(
        &second_stdout,
        [
            "OUTCOME: READY",
            "OBJECT: planning.packet",
            "NEXT SAFE ACTION: run `handbook generate --packet planning.packet`",
        ],
    );
    assert!(second_stdout.contains("## JSON FALLBACK"));
    assert!(
        !second_stdout.contains("run `handbook inspect --packet planning.packet` for proof"),
        "inspect ready path should not loop back into inspect: {second_stdout}"
    );
    assert!(second_stdout.contains("## PACKET BODY"));
    assert!(second_stdout.contains("### CHARTER"));
    assert!(second_stdout.contains("### PROJECT_CONTEXT"));
    assert!(!second_stdout.contains("### FEATURE_SPEC"));
}

#[test]
#[cfg(any())]
fn doctor_marks_empty_charter_as_invalid_baseline() {
    assert_doctor_empty_baseline_invalid(
        ".handbook/charter/CHARTER.md",
        "NEXT SAFE ACTION: run `handbook author charter --from-inputs <path|->`",
        "CHARTER [.handbook/charter/CHARTER.md] STATUS: EMPTY ACTION: run `handbook author charter --from-inputs <path|->`",
    );
}

#[test]
#[cfg(any())]
fn doctor_marks_empty_project_context_as_invalid_baseline() {
    assert_doctor_empty_baseline_invalid(
        ".handbook/project_context/PROJECT_CONTEXT.md",
        "NEXT SAFE ACTION: run `handbook author project-context --from-inputs <path|->`",
        "PROJECT_CONTEXT [.handbook/project_context/PROJECT_CONTEXT.md] STATUS: EMPTY ACTION: run `handbook author project-context --from-inputs <path|->`",
    );
}

#[test]
fn generate_refuses_semantically_invalid_required_project_context() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    write_valid_selected_charter(root);
    write_file(
        &root.join(".handbook/project/context.yaml"),
        legacy_placeholder_project_context_markdown().as_bytes(),
    );
    write_file(
        &root.join(".handbook/feature_spec/FEATURE_SPEC.md"),
        b"feature-body",
    );

    let output = run_in(root, &["generate"]);
    assert!(!output.status.success(), "generate should refuse");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert_first_three_lines(
        &stdout,
        [
            "OUTCOME: REFUSED",
            "OBJECT: planning.packet",
            "NEXT SAFE ACTION: run `handbook author project-context --from-inputs <path|->`",
        ],
    );
    assert!(stdout.contains("CATEGORY: RequiredArtifactInvalid"));
    assert!(stdout.contains("required canonical artifact is invalid"));
    assert!(!stdout.contains("## PACKET BODY"));
}

#[test]
fn generate_blocks_invalid_required_charter_with_required_artifact_invalid() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    write_file(&root.join(".handbook/charter/CHARTER.md"), b"charter-body");
    write_file(
        &root.join(".handbook/feature_spec/FEATURE_SPEC.md"),
        b"feature-body",
    );

    let output = run_in(root, &["generate"]);
    assert!(!output.status.success(), "generate should fail");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert_first_three_lines(
        &stdout,
        [
            "OUTCOME: REFUSED",
            "OBJECT: planning.packet",
            "NEXT SAFE ACTION: run `handbook author charter --from-inputs <path|->`",
        ],
    );
    assert!(stdout.contains("CATEGORY: RequiredArtifactInvalid"));
    assert!(stdout.contains("required canonical artifact is invalid"));
}

#[test]
fn generate_refuses_when_system_root_missing() {
    let dir = tempfile::tempdir().expect("tempdir");
    let output = binary_in(dir.path())
        .arg("generate")
        .output()
        .expect("generate should run");

    assert!(!output.status.success(), "generate should return nonzero");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert_first_three_lines(
        &stdout,
        [
            "OUTCOME: REFUSED",
            "OBJECT: planning.packet",
            "NEXT SAFE ACTION: run `handbook setup`",
        ],
    );
    assert!(
        stdout.contains("## REFUSAL"),
        "expected refusal section: {stdout}"
    );
    assert!(
        stdout.contains("CATEGORY: SystemRootMissing"),
        "expected SystemRootMissing category: {stdout}"
    );
    assert!(
        stdout.contains("BROKEN SUBJECT: policy system_root"),
        "expected broken subject line: {stdout}"
    );
}

#[test]
fn generate_succeeds_when_feature_spec_is_missing_in_partial_system_tree() {
    let dir = partial_system_repo();

    let output = run_in(dir.path(), &["generate"]);
    assert!(output.status.success(), "generate should succeed");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert_first_three_lines(
        &stdout,
        [
            "OUTCOME: READY",
            "OBJECT: planning.packet",
            "NEXT SAFE ACTION: run `handbook inspect --packet planning.packet` for proof",
        ],
    );
    assert!(stdout.contains("### CHARTER (.handbook/project/charter.yaml)"));
    assert!(stdout.contains("### PROJECT_CONTEXT (.handbook/project/context.yaml)"));
    assert!(!stdout.contains("### FEATURE_SPEC (.handbook/feature_spec/FEATURE_SPEC.md)"));
    assert!(!stdout.contains(".handbook/feature_spec/FEATURE_SPEC.md"));
    assert!(stdout.contains("optional source omitted: .handbook/project/environment.yaml"));
}

#[test]
fn inspect_refuses_when_system_root_missing() {
    let dir = tempfile::tempdir().expect("tempdir");
    let output = binary_in(dir.path())
        .arg("inspect")
        .output()
        .expect("inspect should run");

    assert!(!output.status.success(), "inspect should return nonzero");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert_first_three_lines(
        &stdout,
        [
            "OUTCOME: REFUSED",
            "OBJECT: planning.packet",
            "NEXT SAFE ACTION: run `handbook setup`",
        ],
    );
    assert!(
        stdout.contains("## JSON FALLBACK"),
        "expected JSON fallback: {stdout}"
    );
}

#[test]
fn inspect_succeeds_when_feature_spec_is_missing_in_partial_system_tree() {
    let dir = partial_system_repo();

    let output = run_in(dir.path(), &["inspect"]);
    assert!(output.status.success(), "inspect should succeed");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert_first_three_lines(
        &stdout,
        [
            "OUTCOME: READY",
            "OBJECT: planning.packet",
            "NEXT SAFE ACTION: run `handbook generate --packet planning.packet`",
        ],
    );
    assert!(stdout.contains("## JSON FALLBACK"));
    assert!(!stdout.contains(".handbook/feature_spec/FEATURE_SPEC.md"));
    assert!(stdout.contains("optional source omitted: .handbook/project/environment.yaml"));
}

#[test]
#[cfg(any())]
fn doctor_blocks_when_system_root_missing() {
    let dir = tempfile::tempdir().expect("tempdir");
    let output = binary_in(dir.path())
        .arg("doctor")
        .output()
        .expect("doctor should run");

    assert!(
        !output.status.success(),
        "doctor should return nonzero when blocked"
    );

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert!(
        stdout.contains("SCAFFOLDED"),
        "expected scaffolded header: {stdout}"
    );
    assert!(stdout.contains("ROOT STATUS: MISSING"), "{stdout}");
    assert!(
        stdout.contains("NEXT SAFE ACTION: run `handbook setup`"),
        "expected human-facing next action: {stdout}"
    );
    assert!(
        !stdout.contains("NEXT ACTION:"),
        "doctor should use NEXT SAFE ACTION phrasing: {stdout}"
    );
}

#[test]
#[cfg(any())]
fn doctor_reports_ready_when_feature_spec_is_missing_in_partial_system_tree() {
    let dir = partial_system_repo();

    let output = run_in(dir.path(), &["doctor"]);
    assert!(output.status.success(), "doctor should return success");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert!(stdout.contains("BASELINE_COMPLETE"), "{stdout}");
}

#[test]
fn generate_refuses_against_repo_root_when_nested_git_repo_has_invalid_system_root() {
    let (_dir, nested) = nested_git_repo_with_nested_cwd();
    write_file(&nested.join("../../.handbook"), b"not a directory");

    let output = binary_in(&nested)
        .arg("generate")
        .output()
        .expect("generate should run");

    assert!(!output.status.success(), "generate should return nonzero");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert_first_three_lines(
        &stdout,
        [
            "OUTCOME: REFUSED",
            "OBJECT: planning.packet",
            "NEXT SAFE ACTION: run `handbook setup`",
        ],
    );
    assert!(
        stdout.contains("CATEGORY: SystemRootNotDir"),
        "expected SystemRootNotDir category: {stdout}"
    );
}

#[test]
fn inspect_refuses_against_repo_root_when_nested_git_repo_has_invalid_system_root() {
    let (_dir, nested) = nested_git_repo_with_nested_cwd();
    write_file(&nested.join("../../.handbook"), b"not a directory");

    let output = binary_in(&nested)
        .arg("inspect")
        .output()
        .expect("inspect should run");

    assert!(!output.status.success(), "inspect should return nonzero");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert_first_three_lines(
        &stdout,
        [
            "OUTCOME: REFUSED",
            "OBJECT: planning.packet",
            "NEXT SAFE ACTION: run `handbook setup`",
        ],
    );
    assert!(
        stdout.contains("CATEGORY: SystemRootNotDir"),
        "expected SystemRootNotDir category: {stdout}"
    );
}

#[test]
#[cfg(any())]
fn doctor_blocks_against_repo_root_when_nested_git_repo_has_invalid_system_root() {
    let (_dir, nested) = nested_git_repo_with_nested_cwd();
    write_file(&nested.join("../../.handbook"), b"not a directory");

    let output = binary_in(&nested)
        .arg("doctor")
        .output()
        .expect("doctor should run");

    assert!(
        !output.status.success(),
        "doctor should return nonzero when blocked"
    );

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert!(stdout.contains("INVALID_BASELINE"), "{stdout}");
    assert!(stdout.contains("ROOT STATUS: NOT_DIR"), "{stdout}");
}

#[test]
#[cfg(any())]
fn doctor_does_not_cross_nested_git_repo_boundary_into_parent_system_root() {
    let (_dir, nested) = nested_git_repo_inside_managed_parent_with_nested_cwd();

    let output = binary_in(&nested)
        .arg("doctor")
        .output()
        .expect("doctor should run");

    assert!(
        !output.status.success(),
        "doctor should block against the child repo root"
    );

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert!(stdout.contains("SCAFFOLDED"), "{stdout}");
    assert!(stdout.contains("ROOT STATUS: MISSING"), "{stdout}");
}

#[test]
fn generate_emits_real_packet_body_when_ready() {
    let (_dir, root) = planning_ready_repo();

    let output = binary_in(&root)
        .arg("generate")
        .output()
        .expect("generate should run");

    assert!(output.status.success(), "generate should return zero");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert_first_three_lines(
        &stdout,
        [
            "OUTCOME: READY",
            "OBJECT: planning.packet",
            "NEXT SAFE ACTION: run `handbook inspect --packet planning.packet` for proof",
        ],
    );
    assert!(
        stdout.contains("## INCLUDED SOURCES"),
        "expected included-sources section: {stdout}"
    );
    assert!(
        stdout.contains("## OMISSIONS AND BUDGET"),
        "expected omissions section: {stdout}"
    );
    assert!(
        stdout.contains("## DECISION SUMMARY"),
        "expected decision-summary section: {stdout}"
    );
    assert!(
        stdout.contains("## PACKET OVERVIEW"),
        "expected packet overview section: {stdout}"
    );
    assert!(
        stdout.contains("PACKET VARIANT: planning.packet"),
        "expected packet variant line: {stdout}"
    );
    assert!(
        stdout.contains("STATUS: Selected"),
        "expected selected status: {stdout}"
    );
    assert!(
        stdout.contains("SUMMARY: READY planning.packet:"),
        "expected ready summary line: {stdout}"
    );
    assert!(
        stdout.contains("## PACKET BODY"),
        "expected packet-body section: {stdout}"
    );
    assert!(
        stdout.contains("### CHARTER"),
        "expected charter body section: {stdout}"
    );
    assert!(
        stdout.contains("# Engineering Charter — Boundary Project"),
        "expected committed charter fixture contents: {stdout}"
    );
    assert!(
        stdout.contains("### PROJECT_CONTEXT"),
        "expected selected Project Context body section: {stdout}"
    );
    assert!(!stdout.contains("### FEATURE_SPEC"), "{stdout}");
    assert!(
        !stdout.contains("# Canonical planning-ready feature spec fixture"),
        "unadmitted fixed FeatureSpec must not influence the packet: {stdout}"
    );
}

#[test]
fn generate_succeeds_from_nested_directory_inside_ready_repo() {
    let (_dir, nested) = planning_ready_repo_with_nested_cwd();

    let output = binary_in(&nested)
        .arg("generate")
        .output()
        .expect("generate should run");

    assert!(output.status.success(), "generate should return zero");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert_first_three_lines(
        &stdout,
        [
            "OUTCOME: READY",
            "OBJECT: planning.packet",
            "NEXT SAFE ACTION: run `handbook inspect --packet planning.packet` for proof",
        ],
    );
    assert!(
        stdout.contains("## PACKET BODY"),
        "expected packet body: {stdout}"
    );
    assert!(
        stdout.contains("# Engineering Charter — Boundary Project"),
        "expected committed charter fixture contents: {stdout}"
    );
    assert!(stdout.contains("### PROJECT_CONTEXT"), "{stdout}");
    assert!(!stdout.contains("### FEATURE_SPEC"), "{stdout}");
}

#[test]
fn doctor_reports_ready_when_required_artifacts_present() {
    let dir = profile_ready_repo();
    let root = dir.path();

    let output = run_in(root, &["doctor"]);

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert!(stdout.starts_with("OUTCOME: READY\n"), "{stdout}");
    assert!(stdout.contains("## PROJECT CONTEXT\n"), "{stdout}");
    assert!(stdout.contains("PATH: .handbook/project/context.yaml SOURCE FINGERPRINT: sha256:c8646d5821d80fe8e0eeade2713fc63f6e91b2cbb64f6832b5a1f178b810d02f RENDERED OUTPUT FINGERPRINT: sha256:9502d897dc9542a492fdc50ca9ebb2340ac59be25b37a615ea6cb842387641fd MEDIA TYPE: text/markdown\n"), "{stdout}");

    let json_output = run_in(root, &["doctor", "--json"]);
    assert!(json_output.status.success());
    let json_stdout = String::from_utf8(json_output.stdout).expect("doctor JSON is utf-8");
    assert!(json_stdout.ends_with('\n'));
    assert!(!json_stdout.ends_with("\n\n"));
    let value: serde_json::Value = serde_json::from_str(&json_stdout).expect("doctor JSON");
    assert_eq!(value["schema_id"], "handbook.repository-doctor-report");
    assert_eq!(value["schema_version"], "1.2.0");
    assert_eq!(value["status"], "ready");
    assert_eq!(value["project_context"]["instance_id"], "project_context");
    assert_eq!(
        value["project_context"]["kind_ref"],
        "handbook.artifact-kind.project-context@1.1.0"
    );
    assert_eq!(
        value["project_context"]["canonical_path"],
        ".handbook/project/context.yaml"
    );
    assert_eq!(
        value["project_context"]["source_fingerprint"],
        "sha256:c8646d5821d80fe8e0eeade2713fc63f6e91b2cbb64f6832b5a1f178b810d02f"
    );
    assert_eq!(
        value["project_context"]["rendered_output_fingerprint"],
        "sha256:9502d897dc9542a492fdc50ca9ebb2340ac59be25b37a615ea6cb842387641fd"
    );
    assert_eq!(
        value["project_context"]["rendered_media_type"],
        "text/markdown"
    );
}

#[test]
#[cfg(any())]
fn doctor_succeeds_from_nested_directory_inside_ready_repo() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();
    std::fs::create_dir_all(root.join(".git")).expect("git root");
    let nested = root.join("nested/work");
    std::fs::create_dir_all(&nested).expect("nested cwd");
    repair_to_ready(root);

    let output = binary_in(&nested)
        .arg("doctor")
        .output()
        .expect("doctor should run");

    assert!(output.status.success(), "doctor should succeed when ready");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert!(
        stdout.contains("BASELINE_COMPLETE"),
        "expected baseline header: {stdout}"
    );
}

#[test]
fn inspect_reports_ready_when_required_artifacts_present() {
    let (_dir, root) = planning_ready_repo();

    let output = binary_in(&root)
        .arg("inspect")
        .output()
        .expect("inspect should run");

    assert!(output.status.success(), "inspect should succeed when ready");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert_first_three_lines(
        &stdout,
        [
            "OUTCOME: READY",
            "OBJECT: planning.packet",
            "NEXT SAFE ACTION: run `handbook generate --packet planning.packet`",
        ],
    );
    assert!(
        !stdout.contains("run `handbook inspect --packet planning.packet` for proof"),
        "inspect ready path should not loop back into inspect: {stdout}"
    );
    assert!(
        stdout.contains("## JSON FALLBACK"),
        "expected JSON fallback: {stdout}"
    );
    assert!(
        stdout.contains("## DECISION LOG"),
        "expected decision log section: {stdout}"
    );
    assert!(
        stdout.contains("## BUDGET OUTCOME"),
        "expected budget outcome section: {stdout}"
    );
    assert!(
        stdout.contains("## PACKET BODY"),
        "expected packet body section: {stdout}"
    );
    assert!(
        stdout.contains("### CHARTER"),
        "expected charter body section: {stdout}"
    );
    assert!(
        stdout.contains("# Engineering Charter — Boundary Project"),
        "expected committed charter fixture contents: {stdout}"
    );
    assert!(stdout.contains("### PROJECT_CONTEXT"), "{stdout}");
    assert!(!stdout.contains("### FEATURE_SPEC"), "{stdout}");
    assert!(
        stdout.contains("selection packet_id=planning.packet status=Selected"),
        "expected selected decision summary: {stdout}"
    );
}

#[test]
fn inspect_succeeds_from_nested_directory_inside_ready_repo() {
    let (_dir, nested) = planning_ready_repo_with_nested_cwd();

    let output = binary_in(&nested)
        .arg("inspect")
        .output()
        .expect("inspect should run");

    assert!(output.status.success(), "inspect should return zero");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert_first_three_lines(
        &stdout,
        [
            "OUTCOME: READY",
            "OBJECT: planning.packet",
            "NEXT SAFE ACTION: run `handbook generate --packet planning.packet`",
        ],
    );
    assert!(
        stdout.contains("## JSON FALLBACK"),
        "expected JSON fallback: {stdout}"
    );
    assert!(
        stdout.contains("## PACKET BODY"),
        "expected packet body: {stdout}"
    );
    assert!(
        stdout.contains("# Engineering Charter — Boundary Project"),
        "expected committed charter fixture contents: {stdout}"
    );
    assert!(stdout.contains("### PROJECT_CONTEXT"), "{stdout}");
    assert!(!stdout.contains("### FEATURE_SPEC"), "{stdout}");
}

#[test]
fn generate_refuses_when_demo_packet_selected_without_fixture_set() {
    let dir = tempfile::tempdir().expect("tempdir");
    let output = binary_in(dir.path())
        .args(["generate", "--packet", "execution.demo.packet"])
        .output()
        .expect("generate should run");

    assert!(!output.status.success(), "generate should return nonzero");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert!(
        stdout.contains("--fixture-set is required"),
        "expected explicit missing fixture-set refusal: {stdout}"
    );
}

#[test]
fn inspect_blocks_when_demo_packet_selected_without_fixture_set() {
    let dir = tempfile::tempdir().expect("tempdir");
    let output = binary_in(dir.path())
        .args(["inspect", "--packet", "execution.demo.packet"])
        .output()
        .expect("inspect should run");

    assert!(!output.status.success(), "inspect should return nonzero");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert!(
        stdout.contains("--fixture-set is required"),
        "expected explicit missing fixture-set refusal: {stdout}"
    );
}

#[test]
fn generate_resolves_execution_demo_packet_from_fixture_set() {
    let (_dir, root) = execution_demo_repo();
    assert!(
        root.join("tests/fixtures/execution_demo/basic/.handbook/project/charter.yaml")
            .is_file(),
        "expected committed execution demo fixtures to exist under tests/fixtures/execution_demo/basic"
    );
    assert!(
        root.join("tests/fixtures/execution_demo/basic/.handbook/feature_spec/FEATURE_SPEC.md")
            .is_file(),
        "expected committed execution demo fixtures to exist under tests/fixtures/execution_demo/basic"
    );

    let output = binary_in(&root)
        .args([
            "generate",
            "--packet",
            "execution.demo.packet",
            "--fixture-set",
            "basic",
        ])
        .output()
        .expect("generate should run");

    assert!(output.status.success(), "generate should return zero");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert_first_three_lines(
        &stdout,
        [
            "OUTCOME: READY",
            "OBJECT: execution.demo.packet",
            "NEXT SAFE ACTION: run `handbook inspect --packet execution.demo.packet --fixture-set basic` for proof",
        ],
    );
    assert!(
        stdout.contains("## FIXTURE DEMO"),
        "expected fixture demo section: {stdout}"
    );
    assert!(
        stdout.contains("MODE: fixture-backed execution demo"),
        "expected fixture-backed label near top: {stdout}"
    );
    assert!(
        stdout.contains("FIXTURE SET: basic"),
        "expected fixture set id: {stdout}"
    );
    assert!(
        stdout.contains("## INCLUDED SOURCES"),
        "expected included-sources section: {stdout}"
    );
    assert!(
        stdout.contains("FIXTURE LINEAGE:"),
        "expected fixture lineage block: {stdout}"
    );
    assert!(
        stdout.contains("## PACKET BODY"),
        "expected packet body section: {stdout}"
    );
    assert!(
        stdout.contains("### CHARTER"),
        "expected charter body section: {stdout}"
    );
    assert!(stdout.contains("### PROJECT_CONTEXT"), "{stdout}");
    assert!(!stdout.contains("### FEATURE_SPEC"), "{stdout}");
    assert!(
        stdout.contains("## PACKET OVERVIEW"),
        "expected packet overview section: {stdout}"
    );
    assert!(
        stdout.contains("PACKET VARIANT: execution.demo.packet"),
        "expected packet variant line: {stdout}"
    );
    assert!(
        stdout.contains("STATUS: Selected"),
        "expected selected status: {stdout}"
    );
    assert!(
        stdout.contains("SUMMARY: READY execution.demo.packet"),
        "expected ready summary line: {stdout}"
    );
}

#[test]
fn generate_resolves_execution_demo_packet_from_nested_directory_inside_repo() {
    let (_dir, nested) = execution_demo_repo_with_nested_cwd();

    let output = binary_in(&nested)
        .args([
            "generate",
            "--packet",
            "execution.demo.packet",
            "--fixture-set",
            "basic",
        ])
        .output()
        .expect("generate should run");

    assert!(output.status.success(), "generate should return zero");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert_first_three_lines(
        &stdout,
        [
            "OUTCOME: READY",
            "OBJECT: execution.demo.packet",
            "NEXT SAFE ACTION: run `handbook inspect --packet execution.demo.packet --fixture-set basic` for proof",
        ],
    );
    assert!(
        stdout.contains("## FIXTURE DEMO"),
        "expected fixture demo section: {stdout}"
    );
    assert!(
        stdout.contains("MODE: fixture-backed execution demo"),
        "expected fixture-backed label near top: {stdout}"
    );
    assert!(
        stdout.contains("FIXTURE SET: basic"),
        "expected fixture set id: {stdout}"
    );
    assert!(
        stdout.contains("FIXTURE LINEAGE:"),
        "expected fixture lineage block: {stdout}"
    );
    assert!(
        stdout.contains("## PACKET BODY"),
        "expected packet body section: {stdout}"
    );
    assert!(
        stdout.contains("### CHARTER"),
        "expected charter body section: {stdout}"
    );
    assert!(stdout.contains("### PROJECT_CONTEXT"), "{stdout}");
    assert!(!stdout.contains("### FEATURE_SPEC"), "{stdout}");
}

#[test]
fn inspect_includes_fixture_section_for_execution_demo_packet() {
    let (_dir, root) = execution_demo_repo();
    assert!(
        root.join("tests/fixtures/execution_demo/basic/.handbook/project/charter.yaml")
            .is_file(),
        "expected committed execution demo fixtures to exist under tests/fixtures/execution_demo/basic"
    );
    assert!(
        root.join("tests/fixtures/execution_demo/basic/.handbook/feature_spec/FEATURE_SPEC.md")
            .is_file(),
        "expected committed execution demo fixtures to exist under tests/fixtures/execution_demo/basic"
    );

    let output = binary_in(&root)
        .args([
            "inspect",
            "--packet",
            "execution.demo.packet",
            "--fixture-set",
            "basic",
        ])
        .output()
        .expect("inspect should run");

    assert!(output.status.success(), "inspect should succeed when ready");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert_first_three_lines(
        &stdout,
        [
            "OUTCOME: READY",
            "OBJECT: execution.demo.packet",
            "NEXT SAFE ACTION: run `handbook generate --packet execution.demo.packet --fixture-set basic`",
        ],
    );
    assert!(
        !stdout.contains(
            "run `handbook inspect --packet execution.demo.packet --fixture-set basic` for proof"
        ),
        "inspect ready path should not loop back into inspect: {stdout}"
    );
    assert!(
        stdout.contains("## FIXTURE DEMO"),
        "expected fixture section: {stdout}"
    );
    assert!(
        stdout.contains("## DECISION LOG"),
        "expected decision log section: {stdout}"
    );
    assert!(
        stdout.contains("## BUDGET OUTCOME"),
        "expected budget outcome section: {stdout}"
    );
    assert!(
        stdout.contains("MODE: fixture-backed execution demo"),
        "expected fixture-backed label near top: {stdout}"
    );
    assert!(
        stdout.contains("FIXTURE SET: basic"),
        "expected fixture set id: {stdout}"
    );
    assert!(
        stdout.contains("FIXTURE LINEAGE:"),
        "expected fixture lineage list: {stdout}"
    );
    assert!(
        stdout.contains("## INCLUDED SOURCES"),
        "expected included-sources section: {stdout}"
    );
    assert!(
        stdout.contains("## PACKET BODY"),
        "expected packet body section: {stdout}"
    );
    assert!(
        stdout.contains("### CHARTER"),
        "expected charter body section: {stdout}"
    );
    assert!(stdout.contains("### PROJECT_CONTEXT"), "{stdout}");
    assert!(!stdout.contains("### FEATURE_SPEC"), "{stdout}");
    assert!(
        stdout.contains("## PACKET OVERVIEW"),
        "expected packet overview section: {stdout}"
    );
    assert!(
        stdout.contains("PACKET VARIANT: execution.demo.packet"),
        "expected packet variant line: {stdout}"
    );
    assert!(
        stdout.contains("STATUS: Selected"),
        "expected selected status: {stdout}"
    );
    assert!(
        stdout.contains("SUMMARY: READY execution.demo.packet"),
        "expected ready summary line: {stdout}"
    );
    let pos_charter = stdout
        .find("Charter [.handbook/project/charter.yaml]")
        .expect("charter should be listed");
    let pos_project_context = stdout
        .find("ProjectContext [.handbook/project/context.yaml]")
        .expect("Project Context should be listed");
    assert!(
        pos_charter < pos_project_context,
        "expected descriptor-selected ordering (Charter before Project Context): {stdout}"
    );
    assert!(!stdout.contains("FeatureSpec [.handbook/feature_spec/FEATURE_SPEC.md]"));
}

#[test]
fn inspect_preserves_full_execution_demo_fixture_lineage_order() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();
    let fixture_repo = root.join("tests/fixtures/execution_demo/full-lineage");
    let fixture_root = fixture_repo.join(".handbook");

    std::fs::create_dir_all(&fixture_repo).expect("fixture repository root");
    write_valid_selected_charter(&fixture_repo);
    write_file(
        &fixture_root.join("project/context.yaml"),
        valid_selected_project_context_yaml().as_bytes(),
    );
    write_file(
        &fixture_root.join("feature_spec/FEATURE_SPEC.md"),
        b"demo feature",
    );

    let output = binary_in(root)
        .args([
            "inspect",
            "--packet",
            "execution.demo.packet",
            "--fixture-set",
            "full-lineage",
        ])
        .output()
        .expect("inspect should run");

    assert!(output.status.success(), "inspect should succeed when ready");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    let fixture_section_start = stdout.find("## FIXTURE DEMO").expect("fixture section");
    let json_section_start = stdout.find("## JSON FALLBACK").expect("json fallback");
    let fixture_section = &stdout[fixture_section_start..json_section_start];
    assert_in_order(
        fixture_section,
        &[
            "1. Charter [.handbook/project/charter.yaml]",
            "2. ProjectContext [.handbook/project/context.yaml]",
        ],
    );

    let json_section = &stdout[json_section_start..];
    assert_in_order(
        json_section,
        &[
            "\"canonical_repo_relative_path\": \".handbook/project/charter.yaml\"",
            "\"canonical_repo_relative_path\": \".handbook/project/context.yaml\"",
        ],
    );
    assert!(!stdout.contains(".handbook/feature_spec/FEATURE_SPEC.md"));
}

#[test]
fn generate_non_ready_execution_demo_preserves_fixture_backed_labeling() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();
    let fixture_root = root.join("tests/fixtures/execution_demo/non-ready/.handbook");

    write_file(&fixture_root.join("project/charter.yaml"), b"");
    write_file(
        &fixture_root.join("project/context.yaml"),
        valid_selected_project_context_yaml().as_bytes(),
    );
    write_file(
        &fixture_root.join("feature_spec/FEATURE_SPEC.md"),
        b"demo feature",
    );

    let output = binary_in(root)
        .args([
            "generate",
            "--packet",
            "execution.demo.packet",
            "--fixture-set",
            "non-ready",
        ])
        .output()
        .expect("generate should run");

    assert!(
        !output.status.success(),
        "generate should be non-zero when the execution demo fixture is not ready: {}",
        String::from_utf8_lossy(&output.stdout)
    );

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    let fixture_section_start = stdout
        .find("MODE: fixture-backed execution demo")
        .expect("fixture-backed label should be injected near the top");
    let body_section_start = stdout
        .find("## REFUSAL")
        .or_else(|| stdout.find("## BLOCKERS"))
        .expect("non-ready generate body section");
    assert!(
        fixture_section_start < body_section_start,
        "expected fixture section before the non-ready body section for generate output: {stdout}"
    );
    assert!(
        stdout.contains("FIXTURE SET: non-ready"),
        "expected fixture set id: {stdout}"
    );
    assert!(
        stdout.contains("FIXTURE LINEAGE:"),
        "expected fixture lineage list: {stdout}"
    );
    assert_in_order(
        &stdout,
        &["1. ProjectContext [.handbook/project/context.yaml]"],
    );
    assert!(!stdout.contains("FeatureSpec [.handbook/feature_spec/FEATURE_SPEC.md]"));
    assert!(stdout.contains("document_not_object"), "{stdout}");
}

#[test]
fn inspect_resolves_execution_demo_packet_from_nested_directory_inside_repo() {
    let (_dir, nested) = execution_demo_repo_with_nested_cwd();

    let output = binary_in(&nested)
        .args([
            "inspect",
            "--packet",
            "execution.demo.packet",
            "--fixture-set",
            "basic",
        ])
        .output()
        .expect("inspect should run");

    assert!(output.status.success(), "inspect should return zero");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert_first_three_lines(
        &stdout,
        [
            "OUTCOME: READY",
            "OBJECT: execution.demo.packet",
            "NEXT SAFE ACTION: run `handbook generate --packet execution.demo.packet --fixture-set basic`",
        ],
    );
    assert!(
        stdout.contains("## FIXTURE DEMO"),
        "expected fixture section: {stdout}"
    );
    assert!(
        stdout.contains("MODE: fixture-backed execution demo"),
        "expected fixture-backed label near top: {stdout}"
    );
    assert!(
        stdout.contains("FIXTURE SET: basic"),
        "expected fixture set id: {stdout}"
    );
    assert!(
        stdout.contains("FIXTURE LINEAGE:"),
        "expected fixture lineage list: {stdout}"
    );
    assert!(
        stdout.contains("## JSON FALLBACK"),
        "expected JSON fallback: {stdout}"
    );
    assert!(
        stdout.contains("## PACKET BODY"),
        "expected packet body section: {stdout}"
    );
    assert!(
        stdout.contains("### CHARTER"),
        "expected charter body section: {stdout}"
    );
    assert!(stdout.contains("### PROJECT_CONTEXT"), "{stdout}");
    assert!(!stdout.contains("### FEATURE_SPEC"), "{stdout}");
}

#[test]
fn generate_from_committed_fixture_dir_refuses_against_workspace_git_root() {
    let (_dir, fixture_dir) = committed_execution_demo_fixture_dir_under_temp_git_root();

    let output = binary_in(&fixture_dir)
        .arg("generate")
        .output()
        .expect("generate should run");

    assert!(!output.status.success(), "generate should return nonzero");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert_first_three_lines(
        &stdout,
        [
            "OUTCOME: REFUSED",
            "OBJECT: planning.packet",
            "NEXT SAFE ACTION: run `handbook setup`",
        ],
    );
    assert!(
        stdout.contains("CATEGORY: SystemRootMissing"),
        "expected refusal against workspace git root: {stdout}"
    );
    assert!(
        stdout.contains("BROKEN SUBJECT: policy system_root"),
        "expected handbook root refusal subject: {stdout}"
    );
}

#[test]
fn inspect_from_committed_fixture_dir_refuses_against_workspace_git_root() {
    let (_dir, fixture_dir) = committed_execution_demo_fixture_dir_under_temp_git_root();

    let output = binary_in(&fixture_dir)
        .arg("inspect")
        .output()
        .expect("inspect should run");

    assert!(!output.status.success(), "inspect should return nonzero");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert_first_three_lines(
        &stdout,
        [
            "OUTCOME: REFUSED",
            "OBJECT: planning.packet",
            "NEXT SAFE ACTION: run `handbook setup`",
        ],
    );
    assert!(
        stdout.contains("CATEGORY: SystemRootMissing"),
        "expected refusal against workspace git root: {stdout}"
    );
    assert!(
        stdout.contains("## JSON FALLBACK"),
        "expected inspect refusal output with json fallback: {stdout}"
    );
}

#[test]
#[cfg(any())]
fn doctor_from_committed_fixture_dir_blocks_against_workspace_git_root() {
    let (_dir, fixture_dir) = committed_execution_demo_fixture_dir_under_temp_git_root();

    let output = binary_in(&fixture_dir)
        .arg("doctor")
        .output()
        .expect("doctor should run");

    assert!(
        !output.status.success(),
        "doctor should return nonzero when blocked"
    );

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert!(stdout.contains("SCAFFOLDED"), "{stdout}");
    assert!(stdout.contains("ROOT STATUS: MISSING"), "{stdout}");
    assert!(
        stdout.contains("NEXT SAFE ACTION: run `handbook setup`"),
        "expected setup-family guidance: {stdout}"
    );
}

#[test]
fn generate_resolves_execution_demo_packet_from_committed_fixture_dir() {
    let (_dir, fixture_dir) = committed_execution_demo_fixture_dir_under_temp_git_root();

    let output = binary_in(&fixture_dir)
        .args([
            "generate",
            "--packet",
            "execution.demo.packet",
            "--fixture-set",
            "basic",
        ])
        .output()
        .expect("generate should run");

    assert!(output.status.success(), "generate should return zero");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert_first_three_lines(
        &stdout,
        [
            "OUTCOME: READY",
            "OBJECT: execution.demo.packet",
            "NEXT SAFE ACTION: run `handbook inspect --packet execution.demo.packet --fixture-set basic` for proof",
        ],
    );
    assert!(
        stdout.contains("## FIXTURE DEMO"),
        "expected fixture demo section: {stdout}"
    );
    assert!(
        stdout.contains("FIXTURE BASIS ROOT: tests/fixtures/execution_demo/basic/.handbook/"),
        "expected fixture basis root to resolve exactly once: {stdout}"
    );
}

#[test]
fn inspect_resolves_execution_demo_packet_from_committed_fixture_dir() {
    let (_dir, fixture_dir) = committed_execution_demo_fixture_dir_under_temp_git_root();

    let output = binary_in(&fixture_dir)
        .args([
            "inspect",
            "--packet",
            "execution.demo.packet",
            "--fixture-set",
            "basic",
        ])
        .output()
        .expect("inspect should run");

    assert!(output.status.success(), "inspect should return zero");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert_first_three_lines(
        &stdout,
        [
            "OUTCOME: READY",
            "OBJECT: execution.demo.packet",
            "NEXT SAFE ACTION: run `handbook generate --packet execution.demo.packet --fixture-set basic`",
        ],
    );
    assert!(
        stdout.contains("## FIXTURE DEMO"),
        "expected fixture section: {stdout}"
    );
    assert!(
        stdout.contains("FIXTURE BASIS ROOT: tests/fixtures/execution_demo/basic/.handbook/"),
        "expected fixture basis root to resolve exactly once: {stdout}"
    );
}

#[test]
fn generate_refuses_for_live_execution_packet_when_other_inputs_ok() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    write_valid_selected_charter(root);
    write_valid_selected_project_context(root);
    write_file(
        &root.join(".handbook/feature_spec/FEATURE_SPEC.md"),
        b"feature",
    );

    let output = binary_in(root)
        .args(["generate", "--packet", "execution.live.packet"])
        .output()
        .expect("generate should run");

    assert!(!output.status.success(), "generate should return nonzero");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert_first_three_lines(
        &stdout,
        [
            "OUTCOME: REFUSED",
            "OBJECT: execution.live.packet",
            "NEXT SAFE ACTION: run `handbook generate --packet planning.packet`",
        ],
    );
    assert!(
        stdout.contains("CATEGORY: UnsupportedRequest"),
        "expected UnsupportedRequest category: {stdout}"
    );
    assert!(
        stdout.contains("fixture-backed execution demos"),
        "expected boundary statement to mention fixture-backed demos: {stdout}"
    );
}

#[test]
fn inspect_redacts_packet_body_for_live_execution_refusal() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    write_valid_selected_charter(root);
    write_valid_selected_project_context(root);
    write_file(
        &root.join(".handbook/feature_spec/FEATURE_SPEC.md"),
        b"feature-body",
    );

    let output = binary_in(root)
        .args(["inspect", "--packet", "execution.live.packet"])
        .output()
        .expect("inspect should run");

    assert!(!output.status.success(), "inspect should return nonzero");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert_first_three_lines(
        &stdout,
        [
            "OUTCOME: REFUSED",
            "OBJECT: execution.live.packet",
            "NEXT SAFE ACTION: run `handbook generate --packet planning.packet`",
        ],
    );
    assert!(stdout.contains("## JSON FALLBACK"));
    assert!(stdout.contains("\"packet_result\""));
    assert!(stdout.contains("\"packet body omitted because request is not ready\""));
    assert!(!stdout.contains("# Engineering Charter — Boundary Project"));
    assert!(!stdout.contains("feature-body"));
}

#[test]
fn generate_refuses_when_required_project_context_path_is_malformed() {
    let dir = malformed_optional_project_context_repo();

    let output = run_in(dir.path(), &["generate"]);
    assert!(!output.status.success(), "generate should return nonzero");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert_first_three_lines(
        &stdout,
        [
            "OUTCOME: REFUSED",
            "OBJECT: planning.packet",
            "NEXT SAFE ACTION: run `handbook setup refresh`",
        ],
    );
    assert!(stdout.contains("CATEGORY: ArtifactReadError"));
    assert!(stdout.contains("non_regular_file_refused"));
    assert!(!stdout.contains("## PACKET BODY"));
}

#[test]
fn inspect_refuses_when_required_project_context_path_is_malformed() {
    let dir = malformed_optional_project_context_repo();

    let output = run_in(dir.path(), &["inspect"]);
    assert!(!output.status.success(), "inspect should return nonzero");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert_first_three_lines(
        &stdout,
        [
            "OUTCOME: REFUSED",
            "OBJECT: planning.packet",
            "NEXT SAFE ACTION: run `handbook setup refresh`",
        ],
    );
    assert!(stdout.contains("CATEGORY: ArtifactReadError"));
    assert!(stdout.contains("non_regular_file_refused"));
    assert!(stdout.contains("## JSON FALLBACK"));
    assert!(!stdout.contains("## PACKET BODY"));
}

fn command_section_lines(help: &str) -> Vec<&str> {
    let mut in_commands = false;
    let mut lines = Vec::new();

    for line in help.lines() {
        if !in_commands {
            if line.trim() == "Commands:" {
                in_commands = true;
            }
            continue;
        }

        if line.trim().is_empty() {
            break;
        }

        if line.starts_with("  ") {
            lines.push(line.trim_start());
        }
    }

    lines
}

fn assert_first_three_lines(stdout: &str, expected: [&str; 3]) {
    let lines: Vec<&str> = stdout.lines().take(3).collect();
    assert_eq!(lines, expected, "unexpected trust header: {stdout}");
}

fn assert_in_order(haystack: &str, needles: &[&str]) {
    let mut last = 0;
    for needle in needles {
        let offset = haystack[last..]
            .find(needle)
            .unwrap_or_else(|| panic!("missing `{needle}` in output:\n{haystack}"));
        last += offset + needle.len();
    }
}

fn assert_help_snapshot(args: &[&str], snapshot_filename: &str) {
    let stdout = run_in(workspace_root().as_path(), args).stdout;
    let rendered = String::from_utf8(stdout).expect("help is utf-8");
    assert_eq!(
        rendered.replace("handbook.exe", "handbook"),
        read_snapshot(snapshot_filename)
    );
}

fn read_snapshot(filename: &str) -> String {
    let path = workspace_root()
        .join("crates/cli/tests/snapshots")
        .join(filename);
    fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {}: {err}", path.display()))
}

#[cfg(any())]
struct SetupSuccessExpectation<'a> {
    outcome: &'a str,
    object: &'a str,
    next_safe_action: &'a str,
    root_status: &'a str,
    starter_actions: &'a [&'a str],
    state_updates: &'a [&'a str],
    scaffold_guidance: bool,
    routed_command: Option<&'a str>,
}

#[cfg(any())]
fn assert_setup_success(stdout: &str, expected: SetupSuccessExpectation<'_>) {
    let lines: Vec<&str> = stdout.lines().collect();
    let mut index = 0;

    assert_eq!(
        lines.get(index).copied(),
        Some(expected.outcome),
        "{stdout}"
    );
    index += 1;
    let expected_object_line = format!("OBJECT: {}", expected.object);
    assert_eq!(
        lines.get(index).copied(),
        Some(expected_object_line.as_str()),
        "{stdout}"
    );
    index += 1;
    assert_eq!(
        lines.get(index).copied(),
        Some(expected.next_safe_action),
        "{stdout}"
    );
    index += 1;
    assert_eq!(
        lines.get(index).copied(),
        Some("## CANONICAL ROOT"),
        "{stdout}"
    );
    index += 1;
    assert_eq!(
        lines.get(index).copied(),
        Some(expected.root_status),
        "{stdout}"
    );
    index += 1;
    assert_eq!(
        lines.get(index).copied(),
        Some("## STARTER FILES"),
        "{stdout}"
    );
    index += 1;

    for expected_action in expected.starter_actions {
        assert_eq!(
            lines.get(index).copied(),
            Some(*expected_action),
            "{stdout}"
        );
        index += 1;
    }

    assert_eq!(
        lines.get(index).copied(),
        Some("## STATE UPDATES"),
        "{stdout}"
    );
    index += 1;
    if expected.state_updates.is_empty() {
        assert_eq!(lines.get(index).copied(), Some("<none>"), "{stdout}");
        index += 1;
    } else {
        for expected_update in expected.state_updates {
            assert_eq!(
                lines.get(index).copied(),
                Some(*expected_update),
                "{stdout}"
            );
            index += 1;
        }
    }

    assert_eq!(lines.get(index).copied(), Some("## MODE NOTES"), "{stdout}");
    index += 1;
    if let Some(routed_command) = expected.routed_command {
        let expected = format!("ROUTED FROM: handbook setup -> {routed_command}");
        assert_eq!(
            lines.get(index).copied(),
            Some(expected.as_str()),
            "{stdout}"
        );
        index += 1;
    }
    if expected.scaffold_guidance {
        assert_eq!(
            lines.get(index).copied(),
            Some(
                "Required starter files still contain shipped scaffold text; replace canonical truth before running `handbook doctor` or packet work."
            ),
            "{stdout}"
        );
        index += 1;
    }
    assert_eq!(
        index,
        lines.len(),
        "unexpected trailing setup output: {stdout}"
    );
}
