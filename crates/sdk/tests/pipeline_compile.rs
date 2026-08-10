#[path = "support/pipeline_proof_corpus_support.rs"]
mod pipeline_proof_corpus_support;

use std::fs;
use std::path::Path;

use handbook_pipeline::{
    pipeline::{load_pipeline_definition, supported_route_state_variables, PipelineDefinition},
    pipeline_compile::{
        compile_pipeline_stage, compile_pipeline_stage_with_runtime,
        render_pipeline_compile_explain, render_pipeline_compile_payload,
        PipelineCompileDocumentStatus, PipelineCompileRefusalClassification,
        PipelineCompileRuntimeContext,
    },
    pipeline_route::{resolve_pipeline_route, RouteVariables},
    route_state::{
        build_route_basis, load_route_state_with_supported_variables, persist_route_basis,
        set_route_state, RouteBasisPersistOutcome, RouteStateMutation, RouteStateMutationOutcome,
    },
};

const PIPELINE_ID: &str = "pipeline.foundation_inputs";
const STAGE_ID: &str = "stage.10_feature_spec";
const FIXED_NOW_UTC: &str = "2026-01-28T18:35:10Z";

fn pipeline_definition(repo_root: &Path) -> PipelineDefinition {
    load_pipeline_definition(repo_root, "core/pipelines/foundation_inputs.yaml")
        .expect("pipeline fixture")
}

fn supported_variables(
    repo_root: &Path,
) -> (PipelineDefinition, std::collections::BTreeSet<String>) {
    let definition = pipeline_definition(repo_root);
    let supported_variables = supported_route_state_variables(&definition);
    (definition, supported_variables)
}

fn apply_state_mutation(
    repo_root: &Path,
    supported_variables: &std::collections::BTreeSet<String>,
    mutation: RouteStateMutation,
) {
    let state =
        load_route_state_with_supported_variables(repo_root, PIPELINE_ID, supported_variables)
            .expect("load route state");
    let outcome = set_route_state(
        repo_root,
        PIPELINE_ID,
        supported_variables.iter().map(String::as_str),
        mutation,
        state.revision,
    )
    .expect("set route state");
    match outcome {
        RouteStateMutationOutcome::Applied(_) => {}
        RouteStateMutationOutcome::Refused(refusal) => {
            panic!("expected route-state mutation to apply, got {refusal:?}")
        }
    }
}

fn persist_route_basis_for_current_state(repo_root: &Path) {
    let (definition, supported_variables) = supported_variables(repo_root);
    let state =
        load_route_state_with_supported_variables(repo_root, PIPELINE_ID, &supported_variables)
            .expect("load route state");
    let route_variables = RouteVariables::new(state.routing.clone()).expect("route variables");
    let route = resolve_pipeline_route(&definition, &route_variables).expect("resolved route");
    let route_basis =
        build_route_basis(repo_root, &definition, &state, &route).expect("route basis");
    let outcome =
        persist_route_basis(repo_root, PIPELINE_ID, route_basis).expect("persist route basis");
    match outcome {
        RouteBasisPersistOutcome::Applied(_) => {}
        RouteBasisPersistOutcome::Refused(refusal) => {
            panic!("expected route basis persist to apply, got {refusal:?}")
        }
    }
}

fn prepare_compile_ready_repo() -> (tempfile::TempDir, std::path::PathBuf) {
    let (dir, repo_root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    let (_, supported_variables) = supported_variables(&repo_root);
    apply_state_mutation(
        &repo_root,
        &supported_variables,
        RouteStateMutation::RunRunner {
            value: "codex-cli".to_string(),
        },
    );
    apply_state_mutation(
        &repo_root,
        &supported_variables,
        RouteStateMutation::RunProfile {
            value: "python-uv".to_string(),
        },
    );
    apply_state_mutation(
        &repo_root,
        &supported_variables,
        RouteStateMutation::RefCharterRef {
            value: "artifacts/charter/CHARTER.md".to_string(),
        },
    );
    apply_state_mutation(
        &repo_root,
        &supported_variables,
        RouteStateMutation::RefProjectContextRef {
            value: "artifacts/project_context/PROJECT_CONTEXT.md".to_string(),
        },
    );
    apply_state_mutation(
        &repo_root,
        &supported_variables,
        RouteStateMutation::RoutingVariable {
            variable: "needs_project_context".to_string(),
            value: false,
        },
    );
    apply_state_mutation(
        &repo_root,
        &supported_variables,
        RouteStateMutation::RoutingVariable {
            variable: "charter_gaps_detected".to_string(),
            value: false,
        },
    );
    persist_route_basis_for_current_state(&repo_root);
    (dir, repo_root)
}

fn prepare_compile_ready_repo_with_default_run() -> (tempfile::TempDir, std::path::PathBuf) {
    let (dir, repo_root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    let (_, supported_variables) = supported_variables(&repo_root);
    apply_state_mutation(
        &repo_root,
        &supported_variables,
        RouteStateMutation::RefCharterRef {
            value: "artifacts/charter/CHARTER.md".to_string(),
        },
    );
    apply_state_mutation(
        &repo_root,
        &supported_variables,
        RouteStateMutation::RefProjectContextRef {
            value: "artifacts/project_context/PROJECT_CONTEXT.md".to_string(),
        },
    );
    apply_state_mutation(
        &repo_root,
        &supported_variables,
        RouteStateMutation::RoutingVariable {
            variable: "needs_project_context".to_string(),
            value: false,
        },
    );
    apply_state_mutation(
        &repo_root,
        &supported_variables,
        RouteStateMutation::RoutingVariable {
            variable: "charter_gaps_detected".to_string(),
            value: false,
        },
    );
    persist_route_basis_for_current_state(&repo_root);
    (dir, repo_root)
}

fn fixed_runtime() -> PipelineCompileRuntimeContext {
    PipelineCompileRuntimeContext {
        now_utc_override: Some(FIXED_NOW_UTC.to_string()),
    }
}

fn set_stage_work_level(repo_root: &Path, work_level: &str) {
    let stage_path = repo_root.join("core/stages/10_feature_spec.md");
    let stage = fs::read_to_string(&stage_path).expect("read stage");
    let updated_stage = stage.replace("work_level: L1", &format!("work_level: {work_level}"));
    assert_ne!(stage, updated_stage, "stage fixture should be updated");
    fs::write(&stage_path, updated_stage).expect("write stage");
}

#[test]
fn compile_feature_spec_payload_matches_shared_golden() {
    let (_dir, repo_root) = prepare_compile_ready_repo();
    let result =
        compile_pipeline_stage_with_runtime(&repo_root, PIPELINE_ID, STAGE_ID, &fixed_runtime())
            .expect("compile result");
    let payload = render_pipeline_compile_payload(&result);

    pipeline_proof_corpus_support::assert_matches_golden_with_explicit_placeholders(
        &payload,
        &[],
        "compile.stage_10_feature_spec.payload.full_context.txt",
    );
    assert!(!payload.contains(repo_root.to_string_lossy().as_ref()));
    assert!(payload.contains("- repo_root: ${repo_root}"));
}

#[test]
fn compile_feature_spec_explain_matches_shared_golden() {
    let (_dir, repo_root) = prepare_compile_ready_repo();
    let result =
        compile_pipeline_stage_with_runtime(&repo_root, PIPELINE_ID, STAGE_ID, &fixed_runtime())
            .expect("compile result");
    let explain = render_pipeline_compile_explain(&result);

    pipeline_proof_corpus_support::assert_matches_golden_with_explicit_placeholders(
        &explain,
        &[],
        "compile.stage_10_feature_spec.explain.full_context.txt",
    );
    assert!(!explain.contains(repo_root.to_string_lossy().as_ref()));
    assert!(explain.contains("repo_root = ${repo_root}"));
}

#[test]
fn compile_feature_spec_ignores_unrelated_malformed_stage_files() {
    let (_dir, repo_root) = prepare_compile_ready_repo();
    fs::write(
        repo_root.join("core/stages/99_bad.md"),
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

    let result =
        compile_pipeline_stage_with_runtime(&repo_root, PIPELINE_ID, STAGE_ID, &fixed_runtime())
            .expect("compile result");
    let payload = render_pipeline_compile_payload(&result);

    pipeline_proof_corpus_support::assert_matches_golden_with_explicit_placeholders(
        &payload,
        &[],
        "compile.stage_10_feature_spec.payload.full_context.txt",
    );
}

#[test]
fn compile_payload_and_explain_share_one_typed_result() {
    let (_dir, repo_root) = prepare_compile_ready_repo();
    let result =
        compile_pipeline_stage_with_runtime(&repo_root, PIPELINE_ID, STAGE_ID, &fixed_runtime())
            .expect("compile result");
    let payload = render_pipeline_compile_payload(&result);
    let explain = render_pipeline_compile_explain(&result);

    assert_eq!(result.target.stage_id, STAGE_ID);
    pipeline_proof_corpus_support::assert_matches_golden_with_explicit_placeholders(
        &payload,
        &[],
        "compile.stage_10_feature_spec.payload.full_context.txt",
    );
    pipeline_proof_corpus_support::assert_matches_golden_with_explicit_placeholders(
        &explain,
        &[],
        "compile.stage_10_feature_spec.explain.full_context.txt",
    );
}

#[test]
fn compile_sanitizes_legacy_absolute_route_basis_repo_root() {
    let (_dir, repo_root) = prepare_compile_ready_repo();
    let state_path = pipeline_proof_corpus_support::pipeline_state_path(&repo_root);
    let state = fs::read_to_string(&state_path).expect("state file");
    let updated_state = state.replacen(
        "    repo_root: ${repo_root}\n",
        &format!("    repo_root: {}\n", repo_root.display()),
        1,
    );
    assert_ne!(
        state, updated_state,
        "route_basis repo_root should be rewritten for the test"
    );
    fs::write(&state_path, updated_state).expect("write legacy route_basis repo_root");

    let result =
        compile_pipeline_stage_with_runtime(&repo_root, PIPELINE_ID, STAGE_ID, &fixed_runtime())
            .expect("compile result");
    let payload = render_pipeline_compile_payload(&result);
    let explain = render_pipeline_compile_explain(&result);

    assert!(!payload.contains(repo_root.to_string_lossy().as_ref()));
    assert!(!explain.contains(repo_root.to_string_lossy().as_ref()));
    assert!(payload.contains("- repo_root: ${repo_root}"));
    assert!(explain.contains("repo_root = ${repo_root}"));
}

#[test]
fn compile_refuses_missing_route_basis() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    let err =
        compile_pipeline_stage(&repo_root, PIPELINE_ID, STAGE_ID).expect_err("compile refusal");

    pipeline_proof_corpus_support::assert_compile_refusal_matches_shared_golden(
        &err,
        PIPELINE_ID,
        STAGE_ID,
        "compile.refused.missing_route_basis.txt",
    );
}

#[test]
fn compile_refuses_malformed_selected_pipeline_definition() {
    let (_dir, repo_root) = prepare_compile_ready_repo();
    let pipeline_path = repo_root.join("core/pipelines/foundation_inputs.yaml");
    let pipeline = fs::read_to_string(&pipeline_path).expect("read pipeline");
    let updated_pipeline = pipeline.replace("  runner: codex-cli", "  runner:");
    assert_ne!(
        pipeline, updated_pipeline,
        "pipeline fixture should be updated"
    );
    fs::write(&pipeline_path, updated_pipeline).expect("write malformed pipeline");

    let err =
        compile_pipeline_stage(&repo_root, PIPELINE_ID, STAGE_ID).expect_err("compile refusal");

    assert_eq!(
        err.classification,
        PipelineCompileRefusalClassification::InvalidDefinition
    );
    assert!(err.summary.contains("selected pipeline definition"));
}

#[test]
fn compile_refuses_malformed_selected_stage_definition() {
    let (_dir, repo_root) = prepare_compile_ready_repo();
    let stage_path = repo_root.join("core/stages/10_feature_spec.md");
    let stage = fs::read_to_string(&stage_path).expect("read stage");
    let updated_stage = stage.replacen("kind: stage", "kind: nonsense", 1);
    assert_ne!(stage, updated_stage, "stage fixture should be updated");
    fs::write(&stage_path, updated_stage).expect("write malformed stage");

    let err =
        compile_pipeline_stage(&repo_root, PIPELINE_ID, STAGE_ID).expect_err("compile refusal");

    assert_eq!(
        err.classification,
        PipelineCompileRefusalClassification::InvalidDefinition
    );
    assert!(err.summary.contains("must declare kind `stage`"));
}

#[test]
fn compile_refuses_missing_required_artifact() {
    let (_dir, repo_root) = prepare_compile_ready_repo();
    fs::remove_file(repo_root.join("artifacts/base/BASE_CONTEXT.md"))
        .expect("remove required artifact");

    let err =
        compile_pipeline_stage(&repo_root, PIPELINE_ID, STAGE_ID).expect_err("compile refusal");

    assert_eq!(
        err.classification,
        PipelineCompileRefusalClassification::MissingRequiredInput
    );
    assert!(err.summary.contains("artifacts/base/BASE_CONTEXT.md"));
}

#[cfg(unix)]
#[test]
fn compile_refuses_symlinked_required_artifact() {
    use std::os::unix::fs::symlink;

    let (_dir, repo_root) = prepare_compile_ready_repo();
    let outside_dir = tempfile::tempdir().expect("outside tempdir");
    let outside_secret = outside_dir.path().join("system-review-secret.txt");
    fs::write(&outside_secret, "outside-secret").expect("write secret");

    let target = repo_root.join("artifacts/base/BASE_CONTEXT.md");
    fs::remove_file(&target).expect("remove artifact");
    symlink(&outside_secret, &target).expect("symlink artifact");

    let err =
        compile_pipeline_stage(&repo_root, PIPELINE_ID, STAGE_ID).expect_err("compile refusal");

    assert_eq!(
        err.classification,
        PipelineCompileRefusalClassification::InvalidState
    );
    assert!(err.summary.contains("artifacts/base/BASE_CONTEXT.md"));
    assert!(!err.summary.contains("outside-secret"));
}

#[test]
fn compile_refuses_when_required_variable_is_missing() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    let stage_path = repo_root.join("core/stages/10_feature_spec.md");
    let stage = fs::read_to_string(&stage_path).expect("read stage");
    let updated_stage = stage.replace("    - project_name?\n", "    - project_name\n");
    assert_ne!(
        stage, updated_stage,
        "stage fixture should be updated for the test"
    );
    fs::write(&stage_path, updated_stage).expect("write stage");

    let (_, supported_variables) = supported_variables(&repo_root);
    apply_state_mutation(
        &repo_root,
        &supported_variables,
        RouteStateMutation::RunRunner {
            value: "codex-cli".to_string(),
        },
    );
    apply_state_mutation(
        &repo_root,
        &supported_variables,
        RouteStateMutation::RunProfile {
            value: "python-uv".to_string(),
        },
    );
    apply_state_mutation(
        &repo_root,
        &supported_variables,
        RouteStateMutation::RefCharterRef {
            value: "artifacts/charter/CHARTER.md".to_string(),
        },
    );
    apply_state_mutation(
        &repo_root,
        &supported_variables,
        RouteStateMutation::RefProjectContextRef {
            value: "artifacts/project_context/PROJECT_CONTEXT.md".to_string(),
        },
    );
    apply_state_mutation(
        &repo_root,
        &supported_variables,
        RouteStateMutation::RoutingVariable {
            variable: "needs_project_context".to_string(),
            value: false,
        },
    );
    apply_state_mutation(
        &repo_root,
        &supported_variables,
        RouteStateMutation::RoutingVariable {
            variable: "charter_gaps_detected".to_string(),
            value: false,
        },
    );
    persist_route_basis_for_current_state(&repo_root);

    let err =
        compile_pipeline_stage_with_runtime(&repo_root, PIPELINE_ID, STAGE_ID, &fixed_runtime())
            .expect_err("compile refusal");

    assert_eq!(
        err.classification,
        PipelineCompileRefusalClassification::MissingRequiredInput
    );
    assert!(err
        .summary
        .contains("required compile variable `project_name`"));
}

#[test]
fn compile_succeeds_when_optional_artifacts_are_absent() {
    let (_dir, repo_root) = prepare_compile_ready_repo();
    for path in [
        "artifacts/project_context/PROJECT_CONTEXT.md",
        "artifacts/foundation/FOUNDATION_STRATEGY.md",
        "artifacts/foundation/TECH_ARCH_BRIEF.md",
    ] {
        fs::remove_file(repo_root.join(path)).expect("remove optional artifact");
    }

    let result =
        compile_pipeline_stage_with_runtime(&repo_root, PIPELINE_ID, STAGE_ID, &fixed_runtime())
            .expect("compile success");

    assert!(
        result.documents.iter().any(|document| {
            document.path == "artifacts/project_context/PROJECT_CONTEXT.md"
                && document.status == PipelineCompileDocumentStatus::MissingOptional
        }),
        "expected optional project-context artifact to be marked missing"
    );
}

#[test]
fn compile_succeeds_with_route_basis_backed_by_default_runner_and_profile() {
    let (_dir, repo_root) = prepare_compile_ready_repo_with_default_run();

    let result =
        compile_pipeline_stage_with_runtime(&repo_root, PIPELINE_ID, STAGE_ID, &fixed_runtime())
            .expect("compile success");

    assert!(result.variables.iter().any(
        |variable| variable.name == "runner" && variable.value.as_deref() == Some("codex-cli")
    ));
    assert!(result.variables.iter().any(
        |variable| variable.name == "profile" && variable.value.as_deref() == Some("python-uv")
    ));
}

#[test]
fn compile_refuses_stale_route_basis_after_route_state_mutation() {
    let (_dir, repo_root) = prepare_compile_ready_repo();
    let (_, supported_variables) = supported_variables(&repo_root);
    apply_state_mutation(
        &repo_root,
        &supported_variables,
        RouteStateMutation::RoutingVariable {
            variable: "needs_project_context".to_string(),
            value: true,
        },
    );

    let err =
        compile_pipeline_stage(&repo_root, PIPELINE_ID, STAGE_ID).expect_err("compile refusal");

    pipeline_proof_corpus_support::assert_compile_refusal_matches_shared_golden(
        &err,
        PIPELINE_ID,
        STAGE_ID,
        "compile.refused.stale_route_basis.txt",
    );
}

#[test]
fn compile_refuses_inactive_stage() {
    let (dir, repo_root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    let (_, supported_variables) = supported_variables(&repo_root);
    apply_state_mutation(
        &repo_root,
        &supported_variables,
        RouteStateMutation::RunRunner {
            value: "codex-cli".to_string(),
        },
    );
    apply_state_mutation(
        &repo_root,
        &supported_variables,
        RouteStateMutation::RunProfile {
            value: "python-uv".to_string(),
        },
    );
    persist_route_basis_for_current_state(&repo_root);

    let err =
        compile_pipeline_stage(&repo_root, PIPELINE_ID, STAGE_ID).expect_err("compile refusal");

    pipeline_proof_corpus_support::assert_compile_refusal_matches_shared_golden(
        &err,
        PIPELINE_ID,
        STAGE_ID,
        "compile.refused.inactive_stage.txt",
    );
    drop(dir);
}

#[test]
fn compile_refuses_stage_not_declared_in_pipeline() {
    let (_dir, repo_root) = prepare_compile_ready_repo();
    let pipeline_path = repo_root.join("core/pipelines/foundation_inputs.yaml");
    let pipeline = fs::read_to_string(&pipeline_path).expect("read pipeline");
    let updated_pipeline = pipeline.replace(
        "  - id: stage.10_feature_spec\n    file: core/stages/10_feature_spec.md\n",
        "",
    );
    assert_ne!(
        pipeline, updated_pipeline,
        "pipeline fixture should be updated for the test"
    );
    fs::write(&pipeline_path, updated_pipeline).expect("write pipeline");

    let err =
        compile_pipeline_stage(&repo_root, PIPELINE_ID, STAGE_ID).expect_err("compile refusal");

    assert_eq!(
        err.classification,
        PipelineCompileRefusalClassification::UnsupportedTarget
    );
    assert!(err.summary.contains("unknown stage selector"));
    assert!(err.summary.contains("stage.10_feature_spec"));
    assert_eq!(
        err.recovery,
        "re-run `pipeline resolve` and confirm the selected stage is declared in the pipeline"
    );
}

#[test]
fn compile_uses_selected_stage_work_level_for_scoped_rule_filtering() {
    let (_dir, repo_root) = prepare_compile_ready_repo();
    set_stage_work_level(&repo_root, "L2");
    persist_route_basis_for_current_state(&repo_root);

    let result =
        compile_pipeline_stage_with_runtime(&repo_root, PIPELINE_ID, STAGE_ID, &fixed_runtime())
            .expect("compile result");
    let payload = render_pipeline_compile_payload(&result);

    assert_eq!(result.target.work_level, "L2");
    assert!(payload.contains("- work_level: L2"));
    assert!(payload.contains("One slice implementation in flight per worktree/agent context."));
    assert!(!payload.contains(
        "Do not merge multiple slices simultaneously into the same target branch unless:"
    ));
}

#[test]
fn compile_filters_scoped_library_inputs_by_selected_stage_work_level() {
    let (_dir, repo_root) = prepare_compile_ready_repo();
    set_stage_work_level(&repo_root, "L2");
    persist_route_basis_for_current_state(&repo_root);

    let library_path =
        repo_root.join("core/library/feature_spec/feature_spec_architect_directive.md");
    let library = fs::read_to_string(&library_path).expect("read library input");
    let scoped_suffix = "\n<!-- SCOPE: L2 -->\nLibrary scoped L2 content.\n<!-- END_SCOPE -->\n<!-- SCOPE: L3 -->\nLibrary scoped L3 content.\n<!-- END_SCOPE -->\n";
    fs::write(&library_path, format!("{library}{scoped_suffix}")).expect("write library input");

    let result =
        compile_pipeline_stage_with_runtime(&repo_root, PIPELINE_ID, STAGE_ID, &fixed_runtime())
            .expect("compile result");
    let library_document = result
        .documents
        .iter()
        .find(|document| {
            document.path == "core/library/feature_spec/feature_spec_architect_directive.md"
        })
        .expect("library document");
    let content = library_document
        .content
        .as_deref()
        .expect("library document content");

    assert!(content.contains("Library scoped L2 content."));
    assert!(!content.contains("Library scoped L3 content."));
}

#[test]
fn compile_refuses_malformed_route_basis() {
    let (_dir, repo_root) = prepare_compile_ready_repo();
    let state_path = pipeline_proof_corpus_support::pipeline_state_path(&repo_root);
    let state = fs::read_to_string(&state_path).expect("state file");
    let malformed = state.replace(
        "schema_version: m2-route-basis-v1",
        "schema_version: bad-route-basis",
    );
    fs::write(&state_path, malformed).expect("write malformed state");

    let err =
        compile_pipeline_stage(&repo_root, PIPELINE_ID, STAGE_ID).expect_err("compile refusal");

    assert_eq!(
        err.classification,
        PipelineCompileRefusalClassification::MalformedRouteBasis
    );
    assert!(err.summary.contains("route_basis"));
    assert!(err.recovery.contains("pipeline resolve"));
}

#[test]
fn compile_keeps_non_route_basis_unknown_top_level_keys_as_invalid_state() {
    let (_dir, repo_root) = prepare_compile_ready_repo();
    let state_path = pipeline_proof_corpus_support::pipeline_state_path(&repo_root);
    let state = fs::read_to_string(&state_path).expect("state file");
    fs::write(&state_path, format!("{state}\nroute_basis_extra: true\n"))
        .expect("write malformed state");

    let err =
        compile_pipeline_stage(&repo_root, PIPELINE_ID, STAGE_ID).expect_err("compile refusal");

    assert_eq!(
        err.classification,
        PipelineCompileRefusalClassification::InvalidState
    );
    assert_eq!(err.pipeline_id.as_deref(), Some(PIPELINE_ID));
    assert_eq!(err.stage_id.as_deref(), Some(STAGE_ID));
    assert!(err.summary.contains("trusted pipeline session state at"));
    assert!(err.summary.contains("unknown field"), "{err:?}");
    assert!(err.summary.contains("route_basis_extra"), "{err:?}");
    assert_eq!(
        err.recovery,
        "fix the persisted route state and retry `pipeline compile`"
    );
}

#[test]
fn compile_refuses_forged_route_basis_status() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    persist_route_basis_for_current_state(&repo_root);

    let state_path = pipeline_proof_corpus_support::pipeline_state_path(&repo_root);
    let state = fs::read_to_string(&state_path).expect("state file");
    let forged = state.replace(
        "  - stage_id: stage.10_feature_spec\n    file: core/stages/10_feature_spec.md\n    status: blocked\n    reason:\n      kind: blocked_by_unresolved_stage\n      upstream_stage_id: stage.06_project_context_interview\n      upstream_status: next\n",
        "  - stage_id: stage.10_feature_spec\n    file: core/stages/10_feature_spec.md\n    status: active\n    reason: null\n",
    );
    assert_ne!(
        state, forged,
        "route_basis fixture should be updated for the test"
    );
    fs::write(&state_path, forged).expect("write forged state");

    let err =
        compile_pipeline_stage(&repo_root, PIPELINE_ID, STAGE_ID).expect_err("compile refusal");

    assert_eq!(
        err.classification,
        PipelineCompileRefusalClassification::MalformedRouteBasis
    );
    pipeline_proof_corpus_support::assert_compile_refusal_matches_shared_golden(
        &err,
        PIPELINE_ID,
        STAGE_ID,
        "compile.refused.malformed_route_basis_forged_status.txt",
    );
}
