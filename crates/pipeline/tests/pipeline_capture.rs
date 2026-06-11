#[path = "support/pipeline_proof_corpus_support.rs"]
mod pipeline_proof_corpus_support;

use std::fs;
use std::path::Path;

use handbook_pipeline::{
    apply_pipeline_capture, capture_pipeline_output, compile_pipeline_stage_with_runtime,
    load_pipeline_capture_cache_entry, load_route_state_with_supported_variables,
    preview_pipeline_capture, render_pipeline_capture_apply_result,
    render_pipeline_capture_preview, render_pipeline_capture_refusal,
    render_pipeline_compile_explain, render_pipeline_compile_payload, set_route_state,
    PipelineCaptureCacheEntry, PipelineCapturePlan, PipelineCaptureRefusalClassification,
    PipelineCaptureRequest, PipelineCaptureStateUpdate, PipelineCaptureStateValue,
    PipelineCompileRuntimeContext, RouteState, RouteStateMutation, RouteStateMutationOutcome,
};
use sha2::{Digest, Sha256};

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

const PIPELINE_ID: &str = pipeline_proof_corpus_support::FOUNDATION_INPUTS_PIPELINE_ID;
const STAGE_04_ID: &str = pipeline_proof_corpus_support::STAGE_04_CHARTER_INPUTS_ID;
const STAGE_05_ID: &str = pipeline_proof_corpus_support::STAGE_05_CHARTER_SYNTHESIZE_ID;
const STAGE_06_ID: &str = pipeline_proof_corpus_support::STAGE_06_PROJECT_CONTEXT_INTERVIEW_ID;
const STAGE_07_ID: &str = pipeline_proof_corpus_support::STAGE_07_FOUNDATION_PACK_ID;
const STAGE_10_ID: &str = pipeline_proof_corpus_support::STAGE_10_FEATURE_SPEC_ID;
const FIXED_NOW_UTC: &str = "2026-01-28T18:35:10Z";
const STAGE_10_CAPTURE_PROVENANCE_PATH: &str =
    ".handbook/state/pipeline/stage_capture/pipeline.foundation_inputs.stage.10_feature_spec.json";

fn stage_04_request(input: String) -> PipelineCaptureRequest {
    PipelineCaptureRequest {
        pipeline_selector: PIPELINE_ID.to_string(),
        stage_selector: STAGE_04_ID.to_string(),
        input,
    }
}

fn stage_05_request(input: String) -> PipelineCaptureRequest {
    PipelineCaptureRequest {
        pipeline_selector: PIPELINE_ID.to_string(),
        stage_selector: STAGE_05_ID.to_string(),
        input,
    }
}

fn stage_06_request(input: String) -> PipelineCaptureRequest {
    PipelineCaptureRequest {
        pipeline_selector: PIPELINE_ID.to_string(),
        stage_selector: STAGE_06_ID.to_string(),
        input,
    }
}

fn stage_07_request(input: String) -> PipelineCaptureRequest {
    PipelineCaptureRequest {
        pipeline_selector: PIPELINE_ID.to_string(),
        stage_selector: STAGE_07_ID.to_string(),
        input,
    }
}

fn stage_10_request(input: String) -> PipelineCaptureRequest {
    PipelineCaptureRequest {
        pipeline_selector: PIPELINE_ID.to_string(),
        stage_selector: STAGE_10_ID.to_string(),
        input,
    }
}

fn stage_04_capture_input() -> String {
    pipeline_proof_corpus_support::read_committed_fixture("artifacts/charter/CHARTER_INPUTS.yaml")
}

fn stage_05_capture_input() -> String {
    pipeline_proof_corpus_support::read_committed_fixture("artifacts/charter/CHARTER.md")
}

fn stage_06_capture_input() -> String {
    pipeline_proof_corpus_support::read_committed_fixture(
        "artifacts/project_context/PROJECT_CONTEXT.md",
    )
}

fn stage_07_capture_input() -> String {
    let outputs = [
        "artifacts/foundation/FOUNDATION_STRATEGY.md",
        "artifacts/foundation/TECH_ARCH_BRIEF.md",
        "artifacts/foundation/TEST_STRATEGY_BRIEF.md",
        "artifacts/foundation/QUALITY_GATES_SPEC.md",
        "artifacts/foundation/quality_gates.yaml",
        "artifacts/foundation/ENVIRONMENT_INVENTORY.md",
    ];
    let mut out = String::new();
    for path in outputs {
        out.push_str(&format!("--- FILE: {path} ---\n"));
        out.push_str(&pipeline_proof_corpus_support::read_committed_fixture(path));
        out.push('\n');
    }
    out
}

fn fixed_runtime() -> PipelineCompileRuntimeContext {
    PipelineCompileRuntimeContext {
        now_utc_override: Some(FIXED_NOW_UTC.to_string()),
    }
}

fn stage_10_compile_payload(repo_root: &Path) -> String {
    let result =
        compile_pipeline_stage_with_runtime(repo_root, PIPELINE_ID, STAGE_10_ID, &fixed_runtime())
            .expect("compile result");
    render_pipeline_compile_payload(&result)
}

fn stage_10_compile_explain(repo_root: &Path) -> String {
    let result =
        compile_pipeline_stage_with_runtime(repo_root, PIPELINE_ID, STAGE_10_ID, &fixed_runtime())
            .expect("compile result");
    render_pipeline_compile_explain(&result)
}

fn stage_10_completed_feature_spec_input() -> String {
    pipeline_proof_corpus_support::read_committed_model_output("stage_10_feature_spec.md")
}

fn normalize_capture_id(output: &str, capture_id: &str) -> String {
    output.replace(capture_id, "{{CAPTURE_ID}}")
}

fn capture_next_safe_action(rendered: &str) -> &str {
    rendered
        .lines()
        .find_map(|line| line.strip_prefix("NEXT SAFE ACTION: "))
        .expect("rendered capture output should include next safe action")
}

fn apply_mutation(repo_root: &Path, mutation: RouteStateMutation) {
    let (definition, supported_variables) =
        pipeline_proof_corpus_support::load_foundation_inputs_definition(repo_root);
    let state = handbook_pipeline::load_route_state_with_supported_variables(
        repo_root,
        PIPELINE_ID,
        &supported_variables,
    )
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
            panic!("expected mutation to apply, got {refusal:?}")
        }
    }
    let _ = definition;
}

fn load_route_state(repo_root: &Path) -> RouteState {
    let (_, supported_variables) =
        pipeline_proof_corpus_support::load_foundation_inputs_definition(repo_root);
    load_route_state_with_supported_variables(repo_root, PIPELINE_ID, &supported_variables)
        .expect("load route state")
}

fn assert_no_capture_cache_entries(repo_root: &Path) {
    let capture_dir = repo_root
        .join(".handbook")
        .join("state")
        .join("pipeline")
        .join("capture");
    if !capture_dir.exists() {
        return;
    }
    assert!(
        fs::read_dir(&capture_dir)
            .expect("capture cache dir")
            .next()
            .is_none(),
        "refused capture should not leave cached preview entries"
    );
}

fn recompute_capture_id(plan: &PipelineCapturePlan) -> String {
    let mut identity_plan = plan.clone();
    identity_plan.capture_id.clear();
    let serialized = serde_yaml_bw::to_string(&identity_plan).expect("serialize capture plan");
    let mut hasher = Sha256::new();
    hasher.update(serialized.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn rewrite_tampered_capture_cache(
    repo_root: &Path,
    original_capture_id: &str,
    mutate: impl FnOnce(&mut PipelineCaptureCacheEntry),
) -> String {
    let mut cache_entry =
        load_pipeline_capture_cache_entry(repo_root, original_capture_id).expect("load cache");
    mutate(&mut cache_entry);
    cache_entry.plan.capture_id = recompute_capture_id(&cache_entry.plan);
    cache_entry.capture_id = cache_entry.plan.capture_id.clone();

    let original_path =
        pipeline_proof_corpus_support::pipeline_capture_cache_path(repo_root, original_capture_id);
    let next_path = pipeline_proof_corpus_support::pipeline_capture_cache_path(
        repo_root,
        &cache_entry.capture_id,
    );
    let serialized = serde_yaml_bw::to_string(&cache_entry).expect("serialize tampered cache");
    if next_path != original_path {
        fs::remove_file(&original_path).expect("remove original cache file");
    }
    fs::write(&next_path, serialized).expect("write tampered cache file");
    cache_entry.capture_id
}

fn route_basis_sha256(route_basis: &handbook_pipeline::RouteBasis) -> String {
    let bytes = serde_json::to_vec(route_basis).expect("serialize route basis");
    let mut hasher = Sha256::new();
    hasher.update(&bytes);
    format!("{:x}", hasher.finalize())
}

fn normalized_compile_payload_sha256(payload: &str) -> String {
    let mut normalized = payload
        .lines()
        .filter(|line| !line.trim_start().starts_with("- now_utc: "))
        .collect::<Vec<_>>()
        .join("\n");
    if payload.ends_with('\n') {
        normalized.push('\n');
    }
    let mut hasher = Sha256::new();
    hasher.update(normalized.as_bytes());
    format!("{:x}", hasher.finalize())
}

#[test]
fn capture_preview_stage_04_matches_shared_golden() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_stage_04_capture_ready_repo();
    let preview = preview_pipeline_capture(&repo_root, &stage_04_request(stage_04_capture_input()))
        .expect("preview");
    let rendered = render_pipeline_capture_preview(&preview);
    let normalized = normalize_capture_id(&rendered, &preview.plan.capture_id);

    pipeline_proof_corpus_support::assert_matches_golden_with_explicit_placeholders(
        &normalized,
        &[],
        "capture.preview.stage_04_charter_inputs.txt",
    );
    assert!(pipeline_proof_corpus_support::pipeline_capture_cache_path(
        &repo_root,
        &preview.plan.capture_id
    )
    .is_file());
}

#[test]
fn capture_preview_charter_matches_shared_golden() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_stage_05_capture_ready_repo();
    let preview = preview_pipeline_capture(&repo_root, &stage_05_request(stage_05_capture_input()))
        .expect("preview");
    let rendered = render_pipeline_capture_preview(&preview);
    let normalized = normalize_capture_id(&rendered, &preview.plan.capture_id);

    pipeline_proof_corpus_support::assert_matches_golden_with_explicit_placeholders(
        &normalized,
        &[],
        "capture.preview.stage_05_charter_synthesize.txt",
    );
    assert!(pipeline_proof_corpus_support::pipeline_capture_cache_path(
        &repo_root,
        &preview.plan.capture_id
    )
    .is_file());
}

#[test]
fn capture_preview_stage_06_matches_shared_golden() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_stage_06_capture_ready_repo();
    let preview = preview_pipeline_capture(&repo_root, &stage_06_request(stage_06_capture_input()))
        .expect("preview");
    let rendered = render_pipeline_capture_preview(&preview);
    let normalized = normalize_capture_id(&rendered, &preview.plan.capture_id);

    pipeline_proof_corpus_support::assert_matches_golden_with_explicit_placeholders(
        &normalized,
        &[],
        "capture.preview.stage_06_project_context_interview.txt",
    );
}

#[test]
fn capture_preview_foundation_pack_matches_shared_golden() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_stage_07_capture_ready_repo();
    let preview = preview_pipeline_capture(&repo_root, &stage_07_request(stage_07_capture_input()))
        .expect("preview");
    let rendered = render_pipeline_capture_preview(&preview);
    let normalized = normalize_capture_id(&rendered, &preview.plan.capture_id);

    pipeline_proof_corpus_support::assert_matches_golden_with_explicit_placeholders(
        &normalized,
        &[],
        "capture.preview.stage_07_foundation_pack.txt",
    );
}

#[test]
fn capture_preview_feature_spec_matches_shared_golden_from_completed_external_output() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_stage_10_capture_ready_repo();
    let compile_payload = stage_10_compile_payload(&repo_root);
    let completed_output = stage_10_completed_feature_spec_input();
    assert_ne!(
        compile_payload, completed_output,
        "stage-10 compile payload must stay distinct from completed external model output"
    );
    let preview =
        preview_pipeline_capture(&repo_root, &stage_10_request(completed_output)).expect("preview");
    let rendered = render_pipeline_capture_preview(&preview);
    let normalized = normalize_capture_id(&rendered, &preview.plan.capture_id);

    pipeline_proof_corpus_support::assert_matches_golden_with_explicit_placeholders(
        &normalized,
        &[],
        "capture.preview.stage_10_feature_spec.txt",
    );
}

#[test]
fn capture_apply_stage_04_matches_shared_golden() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_stage_04_capture_ready_repo();
    let result = capture_pipeline_output(&repo_root, &stage_04_request(stage_04_capture_input()))
        .expect("capture");
    let rendered = render_pipeline_capture_apply_result(&result);

    pipeline_proof_corpus_support::assert_matches_golden_with_explicit_placeholders(
        &rendered,
        &[],
        "capture.apply.stage_04_charter_inputs.txt",
    );
    assert_eq!(
        fs::read_to_string(repo_root.join("artifacts/charter/CHARTER_INPUTS.yaml"))
            .expect("artifact"),
        stage_04_capture_input()
    );
}

#[test]
fn capture_apply_charter_matches_shared_golden_and_writes_repo_mirror() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_stage_05_capture_ready_repo();
    let result = capture_pipeline_output(&repo_root, &stage_05_request(stage_05_capture_input()))
        .expect("capture");
    let rendered = render_pipeline_capture_apply_result(&result);
    let next_safe_action = capture_next_safe_action(&rendered);

    pipeline_proof_corpus_support::assert_matches_golden_with_explicit_placeholders(
        &rendered,
        &[],
        "capture.apply.stage_05_charter_synthesize.txt",
    );
    assert_eq!(
        fs::read_to_string(repo_root.join("artifacts/charter/CHARTER.md")).expect("artifact"),
        stage_05_capture_input()
    );
    assert_eq!(
        fs::read_to_string(repo_root.join("CHARTER.md")).expect("repo mirror"),
        stage_05_capture_input()
    );
    assert!(
        next_safe_action.contains(
            "handbook pipeline state set --id pipeline.foundation_inputs --var needs_project_context=<true|false>"
        ),
        "stage-05 apply should tell the operator to set needs_project_context"
    );
    assert!(
        next_safe_action.contains("handbook pipeline resolve --id pipeline.foundation_inputs"),
        "stage-05 apply should tell the operator to refresh route truth"
    );
}

#[test]
fn capture_apply_stage_06_matches_shared_golden_and_updates_project_context_ref() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_stage_06_capture_ready_repo();
    let result = capture_pipeline_output(&repo_root, &stage_06_request(stage_06_capture_input()))
        .expect("capture");
    let rendered = render_pipeline_capture_apply_result(&result);

    pipeline_proof_corpus_support::assert_matches_golden_with_explicit_placeholders(
        &rendered,
        &[],
        "capture.apply.stage_06_project_context_interview.txt",
    );
    assert_eq!(
        fs::read_to_string(repo_root.join("artifacts/project_context/PROJECT_CONTEXT.md"))
            .expect("artifact"),
        stage_06_capture_input()
    );
    assert_eq!(
        fs::read_to_string(repo_root.join("PROJECT_CONTEXT.md")).expect("repo mirror"),
        stage_06_capture_input()
    );
    let state = load_route_state(&repo_root);
    assert_eq!(
        state.refs.project_context_ref.as_deref(),
        Some("artifacts/project_context/PROJECT_CONTEXT.md")
    );
}

#[test]
fn capture_apply_foundation_pack_matches_shared_golden_and_uses_cached_preview() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_stage_07_capture_ready_repo();
    assert!(
        !repo_root.join("ENVIRONMENT_INVENTORY.md").exists(),
        "stage-07 capture-ready fixture should not pre-create the repo-root inventory"
    );
    let preview = preview_pipeline_capture(&repo_root, &stage_07_request(stage_07_capture_input()))
        .expect("preview");
    let cache_path = pipeline_proof_corpus_support::pipeline_capture_cache_path(
        &repo_root,
        &preview.plan.capture_id,
    );
    assert!(cache_path.is_file(), "preview should write capture cache");

    let result = apply_pipeline_capture(&repo_root, &preview.plan.capture_id).expect("apply");
    let rendered = render_pipeline_capture_apply_result(&result);

    pipeline_proof_corpus_support::assert_matches_golden_with_explicit_placeholders(
        &rendered,
        &[],
        "capture.apply.stage_07_foundation_pack.txt",
    );
    assert!(
        !cache_path.exists(),
        "successful apply should clear cached preview"
    );
    assert_eq!(
        fs::read_to_string(repo_root.join("artifacts/foundation/ENVIRONMENT_INVENTORY.md"))
            .expect("artifact"),
        pipeline_proof_corpus_support::read_committed_fixture(
            "artifacts/foundation/ENVIRONMENT_INVENTORY.md"
        )
    );
    assert!(
        !repo_root.join("ENVIRONMENT_INVENTORY.md").exists(),
        "stage-07 apply must not materialize a repo-root inventory mirror"
    );
}

#[test]
fn capture_apply_stage_10_matches_shared_golden_from_completed_external_output() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_stage_10_capture_ready_repo();
    let compile_payload = stage_10_compile_payload(&repo_root);
    let input = stage_10_completed_feature_spec_input();
    assert_ne!(
        compile_payload, input,
        "stage-10 compile payload must stay distinct from completed external model output"
    );
    let result =
        capture_pipeline_output(&repo_root, &stage_10_request(input.clone())).expect("capture");
    let rendered = render_pipeline_capture_apply_result(&result);

    pipeline_proof_corpus_support::assert_matches_golden_with_explicit_placeholders(
        &rendered,
        &[],
        "capture.apply.stage_10_feature_spec.txt",
    );
    assert_eq!(
        fs::read_to_string(repo_root.join("artifacts/feature_spec/FEATURE_SPEC.md"))
            .expect("artifact"),
        input
    );
    let provenance: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(repo_root.join(STAGE_10_CAPTURE_PROVENANCE_PATH))
            .expect("stage-10 capture provenance"),
    )
    .expect("parse stage-10 capture provenance");
    let expected_feature_spec_sha256 = {
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        format!("{:x}", hasher.finalize())
    };
    assert_eq!(
        provenance["schema_version"],
        "m5-stage-10-feature-spec-capture-provenance-v1"
    );
    assert_eq!(provenance["pipeline_id"], PIPELINE_ID);
    assert_eq!(provenance["stage_id"], STAGE_10_ID);
    assert_eq!(
        provenance["feature_spec_path"],
        "artifacts/feature_spec/FEATURE_SPEC.md"
    );
    assert_eq!(
        provenance["feature_spec_sha256"],
        expected_feature_spec_sha256
    );
    assert_eq!(
        provenance["route_basis_state_revision"],
        result.plan.basis.state_revision
    );
    assert_eq!(
        provenance["route_basis_fingerprint_sha256"],
        route_basis_sha256(&result.plan.basis)
    );
    assert_eq!(
        provenance["payload_sha256"],
        normalized_compile_payload_sha256(&compile_payload)
    );
}

#[test]
fn capture_refuses_stage_04_single_file_with_file_wrapper() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_stage_04_capture_ready_repo();
    let wrapped = format!(
        "--- FILE: artifacts/charter/CHARTER_INPUTS.yaml ---\n{}",
        stage_04_capture_input()
    );
    let refusal =
        preview_pipeline_capture(&repo_root, &stage_04_request(wrapped)).expect_err("refusal");
    assert_eq!(
        refusal.classification,
        PipelineCaptureRefusalClassification::InvalidCaptureInput
    );
    assert_eq!(
        refusal.summary,
        "single-file capture stages must receive plain body content and must not use `--- FILE:` wrappers"
    );
    assert_no_capture_cache_entries(&repo_root);
}

#[test]
fn capture_refuses_single_file_with_file_wrapper() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_stage_05_capture_ready_repo();
    let wrapped = format!(
        "--- FILE: artifacts/charter/CHARTER.md ---\n{}",
        stage_05_capture_input()
    );
    let refusal =
        preview_pipeline_capture(&repo_root, &stage_05_request(wrapped)).expect_err("refusal");
    assert_eq!(
        refusal.classification,
        PipelineCaptureRefusalClassification::InvalidCaptureInput
    );
    pipeline_proof_corpus_support::assert_matches_golden_with_explicit_placeholders(
        &render_pipeline_capture_refusal(&refusal, Some(PIPELINE_ID), Some(STAGE_05_ID)),
        &[],
        "capture.refused.single_file_with_file_wrapper.txt",
    );
}

#[test]
fn capture_refuses_stage_06_single_file_with_file_wrapper() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_stage_06_capture_ready_repo();
    let wrapped = format!(
        "--- FILE: artifacts/project_context/PROJECT_CONTEXT.md ---\n{}",
        stage_06_capture_input()
    );
    let refusal =
        preview_pipeline_capture(&repo_root, &stage_06_request(wrapped)).expect_err("refusal");
    assert_eq!(
        refusal.classification,
        PipelineCaptureRefusalClassification::InvalidCaptureInput
    );
    assert_eq!(
        refusal.summary,
        "single-file capture stages must receive plain body content and must not use `--- FILE:` wrappers"
    );
    assert_no_capture_cache_entries(&repo_root);
}

#[test]
fn capture_refuses_stage_10_single_file_with_file_wrapper() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_stage_10_capture_ready_repo();
    let input = stage_10_completed_feature_spec_input();
    let wrapped = format!("--- FILE: artifacts/feature_spec/FEATURE_SPEC.md ---\n{input}",);
    let refusal =
        preview_pipeline_capture(&repo_root, &stage_10_request(wrapped)).expect_err("refusal");
    assert_eq!(
        refusal.classification,
        PipelineCaptureRefusalClassification::InvalidCaptureInput
    );
    assert_eq!(
        refusal.summary,
        "single-file capture stages must receive plain body content and must not use `--- FILE:` wrappers"
    );
    assert_no_capture_cache_entries(&repo_root);
}

#[test]
fn capture_preview_stage_10_refuses_raw_compile_payload_without_side_effects() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_stage_10_capture_ready_repo();
    let initial_state = load_route_state(&repo_root);
    let compile_payload = stage_10_compile_payload(&repo_root);
    assert!(
        !repo_root
            .join("artifacts/feature_spec/FEATURE_SPEC.md")
            .exists(),
        "stage-10 capture-ready fixture should not pre-create the feature-spec artifact"
    );

    let refusal = preview_pipeline_capture(&repo_root, &stage_10_request(compile_payload))
        .expect_err("raw compile payload should refuse");

    assert_eq!(
        refusal.classification,
        PipelineCaptureRefusalClassification::InvalidCaptureInput
    );
    pipeline_proof_corpus_support::assert_matches_golden_with_explicit_placeholders(
        &render_pipeline_capture_refusal(&refusal, Some(PIPELINE_ID), Some(STAGE_10_ID)),
        &[],
        "capture.refused.stage_10_raw_compile_payload.txt",
    );
    assert!(
        !repo_root
            .join("artifacts/feature_spec/FEATURE_SPEC.md")
            .exists(),
        "raw compile payload refusal must not create the feature-spec artifact"
    );
    assert_eq!(load_route_state(&repo_root), initial_state);
    assert_no_capture_cache_entries(&repo_root);
}

#[test]
fn capture_preview_stage_10_refuses_compile_explain_output_without_side_effects() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_stage_10_capture_ready_repo();
    let initial_state = load_route_state(&repo_root);
    let explain_output = stage_10_compile_explain(&repo_root);
    assert!(
        !repo_root
            .join("artifacts/feature_spec/FEATURE_SPEC.md")
            .exists(),
        "stage-10 capture-ready fixture should not pre-create the feature-spec artifact"
    );

    let refusal = preview_pipeline_capture(&repo_root, &stage_10_request(explain_output))
        .expect_err("compile explain output should refuse");

    assert_eq!(
        refusal.classification,
        PipelineCaptureRefusalClassification::InvalidCaptureInput
    );
    assert_eq!(
        refusal.summary,
        "stage.10_feature_spec capture must receive a completed FEATURE_SPEC.md body that satisfies the shipped feature-spec contract"
    );
    assert!(
        !repo_root
            .join("artifacts/feature_spec/FEATURE_SPEC.md")
            .exists(),
        "compile explain refusal must not create the feature-spec artifact"
    );
    assert_eq!(load_route_state(&repo_root), initial_state);
    assert_no_capture_cache_entries(&repo_root);
}

#[test]
fn capture_preview_stage_04_refuses_empty_single_file_body_without_side_effects() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_stage_04_capture_ready_repo();
    let initial_artifact =
        fs::read_to_string(repo_root.join("artifacts/charter/CHARTER_INPUTS.yaml"))
            .expect("artifact");
    let initial_state = load_route_state(&repo_root);

    let refusal = preview_pipeline_capture(&repo_root, &stage_04_request("\n".to_string()))
        .expect_err("refusal");

    assert_eq!(
        refusal.classification,
        PipelineCaptureRefusalClassification::InvalidCaptureInput
    );
    assert_eq!(
        refusal.summary,
        "single-file capture stages must receive a non-empty body"
    );
    assert_eq!(
        fs::read_to_string(repo_root.join("artifacts/charter/CHARTER_INPUTS.yaml"))
            .expect("artifact"),
        initial_artifact
    );
    assert_eq!(load_route_state(&repo_root), initial_state);
    assert_no_capture_cache_entries(&repo_root);
}

#[test]
fn capture_preview_refuses_empty_single_file_body_without_side_effects() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_stage_05_capture_ready_repo();
    let initial_artifact =
        fs::read_to_string(repo_root.join("artifacts/charter/CHARTER.md")).expect("artifact");
    let initial_repo_mirror = fs::read_to_string(repo_root.join("CHARTER.md")).expect("mirror");
    let initial_state = load_route_state(&repo_root);

    let refusal = preview_pipeline_capture(&repo_root, &stage_05_request("\n".to_string()))
        .expect_err("refusal");

    assert_eq!(
        refusal.classification,
        PipelineCaptureRefusalClassification::InvalidCaptureInput
    );
    pipeline_proof_corpus_support::assert_matches_golden_with_explicit_placeholders(
        &render_pipeline_capture_refusal(&refusal, Some(PIPELINE_ID), Some(STAGE_05_ID)),
        &[],
        "capture.refused.empty_single_file_body.txt",
    );
    assert_eq!(
        fs::read_to_string(repo_root.join("artifacts/charter/CHARTER.md")).expect("artifact"),
        initial_artifact
    );
    assert_eq!(
        fs::read_to_string(repo_root.join("CHARTER.md")).expect("mirror"),
        initial_repo_mirror
    );
    assert_eq!(load_route_state(&repo_root), initial_state);
    assert_no_capture_cache_entries(&repo_root);
}

#[test]
fn capture_preview_stage_06_refuses_empty_single_file_body_without_side_effects() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_stage_06_capture_ready_repo();
    let initial_artifact =
        fs::read_to_string(repo_root.join("artifacts/project_context/PROJECT_CONTEXT.md"))
            .expect("artifact");
    let initial_state = load_route_state(&repo_root);
    assert!(
        !repo_root.join("PROJECT_CONTEXT.md").exists(),
        "stage-06 capture-ready fixture should not pre-create the repo mirror"
    );

    let refusal = preview_pipeline_capture(&repo_root, &stage_06_request("\n".to_string()))
        .expect_err("refusal");

    assert_eq!(
        refusal.classification,
        PipelineCaptureRefusalClassification::InvalidCaptureInput
    );
    assert_eq!(
        refusal.summary,
        "single-file capture stages must receive a non-empty body"
    );
    assert_eq!(
        fs::read_to_string(repo_root.join("artifacts/project_context/PROJECT_CONTEXT.md"))
            .expect("artifact"),
        initial_artifact
    );
    assert!(
        !repo_root.join("PROJECT_CONTEXT.md").exists(),
        "empty-body refusal must not create the project-context repo mirror"
    );
    assert_eq!(load_route_state(&repo_root), initial_state);
    assert_no_capture_cache_entries(&repo_root);
}

#[test]
fn capture_preview_stage_10_refuses_empty_single_file_body_without_side_effects() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_stage_10_capture_ready_repo();
    let initial_state = load_route_state(&repo_root);
    assert!(
        !repo_root
            .join("artifacts/feature_spec/FEATURE_SPEC.md")
            .exists(),
        "stage-10 capture-ready fixture should not pre-create the feature-spec artifact"
    );

    let refusal = preview_pipeline_capture(&repo_root, &stage_10_request("\n".to_string()))
        .expect_err("refusal");

    assert_eq!(
        refusal.classification,
        PipelineCaptureRefusalClassification::InvalidCaptureInput
    );
    assert_eq!(
        refusal.summary,
        "single-file capture stages must receive a non-empty body"
    );
    assert!(
        !repo_root
            .join("artifacts/feature_spec/FEATURE_SPEC.md")
            .exists(),
        "empty-body refusal must not create the feature-spec artifact"
    );
    assert_eq!(load_route_state(&repo_root), initial_state);
    assert_no_capture_cache_entries(&repo_root);
}

#[test]
fn capture_apply_refuses_empty_single_file_body_without_side_effects() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_stage_05_capture_ready_repo();
    let initial_artifact =
        fs::read_to_string(repo_root.join("artifacts/charter/CHARTER.md")).expect("artifact");
    let initial_repo_mirror = fs::read_to_string(repo_root.join("CHARTER.md")).expect("mirror");
    let initial_state = load_route_state(&repo_root);

    let refusal = capture_pipeline_output(&repo_root, &stage_05_request("\r\n".to_string()))
        .expect_err("refusal");

    assert_eq!(
        refusal.classification,
        PipelineCaptureRefusalClassification::InvalidCaptureInput
    );
    assert_eq!(
        refusal.summary,
        "single-file capture stages must receive a non-empty body"
    );
    assert_eq!(
        fs::read_to_string(repo_root.join("artifacts/charter/CHARTER.md")).expect("artifact"),
        initial_artifact
    );
    assert_eq!(
        fs::read_to_string(repo_root.join("CHARTER.md")).expect("mirror"),
        initial_repo_mirror
    );
    assert_eq!(load_route_state(&repo_root), initial_state);
    assert_no_capture_cache_entries(&repo_root);
}

#[test]
fn capture_apply_stage_10_refuses_raw_compile_payload_without_side_effects() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_stage_10_capture_ready_repo();
    let initial_state = load_route_state(&repo_root);
    let compile_payload = stage_10_compile_payload(&repo_root);
    assert!(
        !repo_root
            .join("artifacts/feature_spec/FEATURE_SPEC.md")
            .exists(),
        "stage-10 capture-ready fixture should not pre-create the feature-spec artifact"
    );

    let refusal = capture_pipeline_output(&repo_root, &stage_10_request(compile_payload))
        .expect_err("raw compile payload should refuse");

    assert_eq!(
        refusal.classification,
        PipelineCaptureRefusalClassification::InvalidCaptureInput
    );
    pipeline_proof_corpus_support::assert_matches_golden_with_explicit_placeholders(
        &render_pipeline_capture_refusal(&refusal, Some(PIPELINE_ID), Some(STAGE_10_ID)),
        &[],
        "capture.refused.stage_10_raw_compile_payload.txt",
    );
    assert!(
        !repo_root
            .join("artifacts/feature_spec/FEATURE_SPEC.md")
            .exists(),
        "raw compile payload refusal must not create the feature-spec artifact"
    );
    assert_eq!(load_route_state(&repo_root), initial_state);
    assert_no_capture_cache_entries(&repo_root);
}

#[test]
fn capture_apply_stage_10_refuses_invalid_feature_spec_body_without_side_effects() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_stage_10_capture_ready_repo();
    let initial_state = load_route_state(&repo_root);
    let refusal = capture_pipeline_output(&repo_root, &stage_10_request("hello\n".to_string()))
        .expect_err("invalid feature spec body should refuse");

    assert_eq!(
        refusal.classification,
        PipelineCaptureRefusalClassification::InvalidCaptureInput
    );
    assert_eq!(
        refusal.summary,
        "stage.10_feature_spec capture must receive a completed FEATURE_SPEC.md body that satisfies the shipped feature-spec contract"
    );
    assert!(
        !repo_root
            .join("artifacts/feature_spec/FEATURE_SPEC.md")
            .exists(),
        "invalid feature spec refusal must not create the feature-spec artifact"
    );
    assert_eq!(load_route_state(&repo_root), initial_state);
    assert_no_capture_cache_entries(&repo_root);
}

#[test]
fn capture_refuses_missing_declared_block() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_stage_07_capture_ready_repo();
    let missing = [
        "artifacts/foundation/FOUNDATION_STRATEGY.md",
        "artifacts/foundation/TECH_ARCH_BRIEF.md",
        "artifacts/foundation/TEST_STRATEGY_BRIEF.md",
        "artifacts/foundation/QUALITY_GATES_SPEC.md",
        "artifacts/foundation/quality_gates.yaml",
    ]
    .into_iter()
    .map(|path| {
        format!(
            "--- FILE: {path} ---\n{}",
            pipeline_proof_corpus_support::read_committed_fixture(path)
        )
    })
    .collect::<Vec<_>>()
    .join("\n");
    let refusal =
        preview_pipeline_capture(&repo_root, &stage_07_request(missing)).expect_err("refusal");
    pipeline_proof_corpus_support::assert_matches_golden_with_explicit_placeholders(
        &render_pipeline_capture_refusal(&refusal, Some(PIPELINE_ID), Some(STAGE_07_ID)),
        &[],
        "capture.refused.missing_declared_block.txt",
    );
}

#[test]
fn capture_preview_refuses_empty_declared_block_without_side_effects() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_stage_07_capture_ready_repo();
    let initial_artifact =
        fs::read_to_string(repo_root.join("artifacts/foundation/ENVIRONMENT_INVENTORY.md"))
            .expect("artifact");
    let initial_state = load_route_state(&repo_root);
    assert!(
        !repo_root.join("ENVIRONMENT_INVENTORY.md").exists(),
        "empty-body refusal fixture should not pre-create the repo-root inventory"
    );
    let mut input = stage_07_capture_input();
    let target_block = format!(
        "--- FILE: artifacts/foundation/ENVIRONMENT_INVENTORY.md ---\n{}",
        pipeline_proof_corpus_support::read_committed_fixture(
            "artifacts/foundation/ENVIRONMENT_INVENTORY.md"
        )
    );
    input = input.replace(
        &target_block,
        "--- FILE: artifacts/foundation/ENVIRONMENT_INVENTORY.md ---\n",
    );

    let refusal =
        preview_pipeline_capture(&repo_root, &stage_07_request(input)).expect_err("refusal");

    assert_eq!(
        refusal.classification,
        PipelineCaptureRefusalClassification::InvalidCaptureInput
    );
    pipeline_proof_corpus_support::assert_matches_golden_with_explicit_placeholders(
        &render_pipeline_capture_refusal(&refusal, Some(PIPELINE_ID), Some(STAGE_07_ID)),
        &[],
        "capture.refused.empty_declared_block.txt",
    );
    assert_eq!(
        fs::read_to_string(repo_root.join("artifacts/foundation/ENVIRONMENT_INVENTORY.md"))
            .expect("artifact"),
        initial_artifact
    );
    assert!(
        !repo_root.join("ENVIRONMENT_INVENTORY.md").exists(),
        "empty-body refusal must not create the repo-root inventory"
    );
    assert_eq!(load_route_state(&repo_root), initial_state);
    assert_no_capture_cache_entries(&repo_root);
}

#[test]
fn capture_apply_refuses_empty_declared_block_without_side_effects() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_stage_07_capture_ready_repo();
    let initial_artifact =
        fs::read_to_string(repo_root.join("artifacts/foundation/ENVIRONMENT_INVENTORY.md"))
            .expect("artifact");
    let initial_state = load_route_state(&repo_root);
    assert!(
        !repo_root.join("ENVIRONMENT_INVENTORY.md").exists(),
        "empty-body refusal fixture should not pre-create the repo-root inventory"
    );
    let mut input = stage_07_capture_input();
    let target_block = format!(
        "--- FILE: artifacts/foundation/ENVIRONMENT_INVENTORY.md ---\n{}",
        pipeline_proof_corpus_support::read_committed_fixture(
            "artifacts/foundation/ENVIRONMENT_INVENTORY.md"
        )
    );
    input = input.replace(
        &target_block,
        "--- FILE: artifacts/foundation/ENVIRONMENT_INVENTORY.md ---\r\n",
    );

    let refusal =
        capture_pipeline_output(&repo_root, &stage_07_request(input)).expect_err("refusal");

    assert_eq!(
        refusal.classification,
        PipelineCaptureRefusalClassification::InvalidCaptureInput
    );
    assert_eq!(
        refusal.summary,
        "declared artifact block `artifacts/foundation/ENVIRONMENT_INVENTORY.md` must contain a non-empty body"
    );
    assert_eq!(
        fs::read_to_string(repo_root.join("artifacts/foundation/ENVIRONMENT_INVENTORY.md"))
            .expect("artifact"),
        initial_artifact
    );
    assert!(
        !repo_root.join("ENVIRONMENT_INVENTORY.md").exists(),
        "empty-body refusal must not create the repo-root inventory"
    );
    assert_eq!(load_route_state(&repo_root), initial_state);
    assert_no_capture_cache_entries(&repo_root);
}

#[test]
fn capture_refuses_duplicate_declared_block() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_stage_07_capture_ready_repo();
    let mut input = stage_07_capture_input();
    input.push_str(&format!(
        "--- FILE: artifacts/foundation/ENVIRONMENT_INVENTORY.md ---\n{}",
        pipeline_proof_corpus_support::read_committed_fixture(
            "artifacts/foundation/ENVIRONMENT_INVENTORY.md"
        )
    ));
    let refusal =
        preview_pipeline_capture(&repo_root, &stage_07_request(input)).expect_err("refusal");
    pipeline_proof_corpus_support::assert_matches_golden_with_explicit_placeholders(
        &render_pipeline_capture_refusal(&refusal, Some(PIPELINE_ID), Some(STAGE_07_ID)),
        &[],
        "capture.refused.duplicate_declared_block.txt",
    );
}

#[test]
fn capture_refuses_undeclared_block() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_stage_07_capture_ready_repo();
    let mut input = stage_07_capture_input();
    input.push_str("--- FILE: artifacts/foundation/EXTRA_NOT_DECLARED.md ---\nextra\n");
    let refusal =
        preview_pipeline_capture(&repo_root, &stage_07_request(input)).expect_err("refusal");
    pipeline_proof_corpus_support::assert_matches_golden_with_explicit_placeholders(
        &render_pipeline_capture_refusal(&refusal, Some(PIPELINE_ID), Some(STAGE_07_ID)),
        &[],
        "capture.refused.undeclared_block.txt",
    );
}

#[test]
fn capture_refuses_stale_route_basis_after_preview() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_stage_05_capture_ready_repo();
    let preview = preview_pipeline_capture(&repo_root, &stage_05_request(stage_05_capture_input()))
        .expect("preview");
    apply_mutation(
        &repo_root,
        RouteStateMutation::RoutingVariable {
            variable: "charter_gaps_detected".to_string(),
            value: true,
        },
    );
    let refusal =
        apply_pipeline_capture(&repo_root, &preview.plan.capture_id).expect_err("refusal");
    let rendered = render_pipeline_capture_refusal(&refusal, None, None);
    pipeline_proof_corpus_support::assert_matches_golden_with_explicit_placeholders(
        &rendered,
        &[],
        "capture.refused.stale_route_basis.txt",
    );
    assert!(
        pipeline_proof_corpus_support::pipeline_capture_cache_path(
            &repo_root,
            &preview.plan.capture_id
        )
        .exists(),
        "refused apply should keep cached preview"
    );
}

#[test]
fn capture_refuses_inactive_stage() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_stage_05_capture_ready_repo();
    let refusal = preview_pipeline_capture(&repo_root, &stage_07_request(stage_07_capture_input()))
        .expect_err("refusal");
    pipeline_proof_corpus_support::assert_matches_golden_with_explicit_placeholders(
        &render_pipeline_capture_refusal(&refusal, Some(PIPELINE_ID), Some(STAGE_07_ID)),
        &[],
        "capture.refused.inactive_stage.txt",
    );
}

#[test]
fn capture_apply_refuses_missing_capture_id() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    let refusal = apply_pipeline_capture(
        &repo_root,
        "1111111111111111111111111111111111111111111111111111111111111111",
    )
    .expect_err("refusal");
    pipeline_proof_corpus_support::assert_matches_golden_with_explicit_placeholders(
        &render_pipeline_capture_refusal(&refusal, None, None),
        &[],
        "capture.refused.missing_capture_id.txt",
    );
}

#[test]
fn capture_apply_refuses_state_revision_conflict_before_writes() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_stage_05_capture_ready_repo();
    let preview = preview_pipeline_capture(&repo_root, &stage_05_request(stage_05_capture_input()))
        .expect("preview");
    let state_path = pipeline_proof_corpus_support::pipeline_state_path(&repo_root);
    let initial_artifact =
        fs::read_to_string(repo_root.join("artifacts/charter/CHARTER.md")).expect("artifact");
    let initial_repo_mirror = fs::read_to_string(repo_root.join("CHARTER.md")).expect("mirror");
    let initial_state = load_route_state(&repo_root);

    let mut persisted_state: RouteState =
        serde_yaml_bw::from_str(&fs::read_to_string(&state_path).expect("state file"))
            .expect("deserialize route state");
    persisted_state.revision += 1;
    if let Some(route_basis) = persisted_state.route_basis.as_mut() {
        route_basis.state_revision = persisted_state.revision;
    }
    fs::write(
        &state_path,
        serde_yaml_bw::to_string(&persisted_state).expect("serialize route state"),
    )
    .expect("write route state");

    let refusal =
        apply_pipeline_capture(&repo_root, &preview.plan.capture_id).expect_err("refusal");

    assert_eq!(
        refusal.classification,
        PipelineCaptureRefusalClassification::RevisionConflict
    );
    assert!(
        refusal
            .summary
            .contains("does not match previewed capture revision"),
        "expected revision-conflict summary, got: {}",
        refusal.summary
    );
    assert_eq!(
        fs::read_to_string(repo_root.join("artifacts/charter/CHARTER.md")).expect("artifact"),
        initial_artifact
    );
    assert_eq!(
        fs::read_to_string(repo_root.join("CHARTER.md")).expect("mirror"),
        initial_repo_mirror
    );
    assert_eq!(
        load_route_state(&repo_root).routing,
        initial_state.routing,
        "revision-conflict refusal should not mutate route state"
    );
    assert!(
        pipeline_proof_corpus_support::pipeline_capture_cache_path(
            &repo_root,
            &preview.plan.capture_id
        )
        .exists(),
        "refused apply should keep the cached preview in place"
    );
}

#[cfg(unix)]
#[test]
fn capture_apply_refuses_unreadable_cache_entry_without_side_effects() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_stage_05_capture_ready_repo();
    let preview = preview_pipeline_capture(&repo_root, &stage_05_request(stage_05_capture_input()))
        .expect("preview");
    let cache_path = pipeline_proof_corpus_support::pipeline_capture_cache_path(
        &repo_root,
        &preview.plan.capture_id,
    );
    let initial_artifact =
        fs::read_to_string(repo_root.join("artifacts/charter/CHARTER.md")).expect("artifact");
    let initial_repo_mirror = fs::read_to_string(repo_root.join("CHARTER.md")).expect("mirror");
    let initial_state = load_route_state(&repo_root);
    let original_mode = fs::metadata(&cache_path)
        .expect("cache metadata")
        .permissions()
        .mode();

    let mut unreadable_permissions = fs::metadata(&cache_path)
        .expect("cache metadata")
        .permissions();
    unreadable_permissions.set_mode(0o000);
    fs::set_permissions(&cache_path, unreadable_permissions).expect("make cache unreadable");

    let refusal =
        apply_pipeline_capture(&repo_root, &preview.plan.capture_id).expect_err("refusal");

    let mut restored_permissions = fs::metadata(&cache_path)
        .expect("cache metadata")
        .permissions();
    restored_permissions.set_mode(original_mode);
    fs::set_permissions(&cache_path, restored_permissions).expect("restore cache permissions");

    assert_eq!(
        refusal.classification,
        PipelineCaptureRefusalClassification::CacheFailure
    );
    assert!(
        refusal.summary.contains(&preview.plan.capture_id)
            || refusal
                .summary
                .contains(cache_path.to_string_lossy().as_ref()),
        "expected cache id or path in summary, got: {}",
        refusal.summary
    );
    assert!(
        refusal.summary.contains("Permission denied")
            || refusal.summary.contains("permission denied"),
        "expected read failure in summary, got: {}",
        refusal.summary
    );
    assert!(
        !refusal.recovery.to_ascii_lowercase().contains("not found"),
        "recovery should not claim the cached preview is missing: {}",
        refusal.recovery
    );
    assert_eq!(
        fs::read_to_string(repo_root.join("artifacts/charter/CHARTER.md")).expect("artifact"),
        initial_artifact
    );
    assert_eq!(
        fs::read_to_string(repo_root.join("CHARTER.md")).expect("mirror"),
        initial_repo_mirror
    );
    assert_eq!(load_route_state(&repo_root), initial_state);
    assert!(cache_path.is_file(), "cache file should remain present");
}

#[cfg(unix)]
#[test]
fn capture_apply_refuses_symlinked_cache_entry_without_side_effects() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_stage_05_capture_ready_repo();
    let preview = preview_pipeline_capture(&repo_root, &stage_05_request(stage_05_capture_input()))
        .expect("preview");
    let cache_path = pipeline_proof_corpus_support::pipeline_capture_cache_path(
        &repo_root,
        &preview.plan.capture_id,
    );
    let target_path = repo_root.join("CHARTER.md");
    let initial_artifact =
        fs::read_to_string(repo_root.join("artifacts/charter/CHARTER.md")).expect("artifact");
    let initial_repo_mirror = fs::read_to_string(&target_path).expect("mirror");
    let initial_state = load_route_state(&repo_root);

    fs::remove_file(&cache_path).expect("remove cached preview file");
    std::os::unix::fs::symlink(&target_path, &cache_path).expect("replace cache with symlink");

    let refusal =
        apply_pipeline_capture(&repo_root, &preview.plan.capture_id).expect_err("refusal");

    assert_eq!(
        refusal.classification,
        PipelineCaptureRefusalClassification::TamperedCaptureCache
    );
    assert!(
        refusal.summary.contains("capture cache path"),
        "expected cache-path summary, got: {}",
        refusal.summary
    );
    assert!(
        refusal
            .summary
            .contains("cannot be written through symlink"),
        "expected symlink refusal, got: {}",
        refusal.summary
    );
    assert_eq!(
        fs::read_to_string(repo_root.join("artifacts/charter/CHARTER.md")).expect("artifact"),
        initial_artifact
    );
    assert_eq!(
        fs::read_to_string(&target_path).expect("mirror"),
        initial_repo_mirror
    );
    assert_eq!(load_route_state(&repo_root), initial_state);
    assert!(
        fs::symlink_metadata(&cache_path)
            .expect("cache metadata")
            .file_type()
            .is_symlink(),
        "refused apply should leave the symlinked cache entry in place"
    );
}

#[cfg(unix)]
#[test]
fn capture_preview_refuses_cache_path_when_system_root_is_symlinked() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_stage_05_capture_ready_repo();
    let external_root = tempfile::tempdir().expect("external tempdir");
    let system_root = repo_root.join(".handbook");
    let redirected_system_root = external_root.path().join("redirected-system");
    let initial_state = load_route_state(&repo_root);

    fs::rename(&system_root, &redirected_system_root).expect("move .handbook");
    std::os::unix::fs::symlink(&redirected_system_root, &system_root)
        .expect("replace .handbook with symlink");

    let refusal = preview_pipeline_capture(&repo_root, &stage_05_request(stage_05_capture_input()))
        .expect_err("preview refusal");

    assert_eq!(
        refusal.classification,
        PipelineCaptureRefusalClassification::CacheFailure
    );
    assert!(
        refusal.summary.contains("capture cache path"),
        "expected cache-path summary, got: {}",
        refusal.summary
    );
    assert!(
        refusal
            .summary
            .contains("cannot be written through symlink"),
        "expected symlink refusal, got: {}",
        refusal.summary
    );
    assert_eq!(load_route_state(&repo_root), initial_state);
    assert_no_capture_cache_entries(&repo_root);
}

#[cfg(unix)]
#[test]
fn capture_preview_refuses_cache_path_when_capture_parent_is_symlinked() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_stage_05_capture_ready_repo();
    let external_root = tempfile::tempdir().expect("external tempdir");
    let capture_dir = repo_root
        .join(".handbook")
        .join("state")
        .join("pipeline")
        .join("capture");
    let redirected_capture_dir = external_root.path().join("redirected-capture");
    let initial_state = load_route_state(&repo_root);

    std::os::unix::fs::symlink(&redirected_capture_dir, &capture_dir).expect("symlink capture dir");

    let refusal = preview_pipeline_capture(&repo_root, &stage_05_request(stage_05_capture_input()))
        .expect_err("preview refusal");

    assert_eq!(
        refusal.classification,
        PipelineCaptureRefusalClassification::CacheFailure
    );
    assert!(
        refusal.summary.contains("capture cache path"),
        "expected cache-path summary, got: {}",
        refusal.summary
    );
    assert!(
        refusal
            .summary
            .contains("cannot be written through symlink"),
        "expected symlink refusal, got: {}",
        refusal.summary
    );
    assert_eq!(load_route_state(&repo_root), initial_state);
    assert_no_capture_cache_entries(&repo_root);
}

#[cfg(unix)]
#[test]
fn capture_preview_refuses_symlinked_repo_mirror_target_without_side_effects() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_stage_05_capture_ready_repo();
    let external_root = tempfile::tempdir().expect("external tempdir");
    let repo_mirror = repo_root.join("CHARTER.md");
    let redirected_target = external_root.path().join("CHARTER.md");
    let initial_artifact =
        fs::read_to_string(repo_root.join("artifacts/charter/CHARTER.md")).expect("artifact");
    let initial_state = load_route_state(&repo_root);

    fs::remove_file(&repo_mirror).expect("remove repo mirror");
    std::os::unix::fs::symlink(&redirected_target, &repo_mirror).expect("replace mirror");

    let refusal = preview_pipeline_capture(&repo_root, &stage_05_request(stage_05_capture_input()))
        .expect_err("preview refusal");

    assert_eq!(
        refusal.classification,
        PipelineCaptureRefusalClassification::InvalidWriteTarget
    );
    assert!(
        refusal
            .summary
            .contains("cannot be written through symlink"),
        "expected symlink refusal, got: {}",
        refusal.summary
    );
    assert_eq!(
        fs::read_to_string(repo_root.join("artifacts/charter/CHARTER.md")).expect("artifact"),
        initial_artifact
    );
    assert_eq!(load_route_state(&repo_root), initial_state);
    assert!(
        !redirected_target.exists(),
        "preview refusal should not write through the symlinked repo mirror"
    );
    assert_no_capture_cache_entries(&repo_root);
}

#[cfg(unix)]
#[test]
fn capture_apply_refuses_symlinked_cache_parent_chain_without_side_effects() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_stage_05_capture_ready_repo();
    let external_root = tempfile::tempdir().expect("external tempdir");
    let preview = preview_pipeline_capture(&repo_root, &stage_05_request(stage_05_capture_input()))
        .expect("preview");
    let cache_dir = repo_root
        .join(".handbook")
        .join("state")
        .join("pipeline")
        .join("capture");
    let redirected_capture_dir = external_root.path().join("redirected-capture");
    let cache_path = pipeline_proof_corpus_support::pipeline_capture_cache_path(
        &repo_root,
        &preview.plan.capture_id,
    );
    let target_path = repo_root.join("CHARTER.md");
    let initial_artifact =
        fs::read_to_string(repo_root.join("artifacts/charter/CHARTER.md")).expect("artifact");
    let initial_repo_mirror = fs::read_to_string(&target_path).expect("mirror");
    let initial_state = load_route_state(&repo_root);

    fs::rename(&cache_dir, &redirected_capture_dir).expect("move capture dir");
    std::os::unix::fs::symlink(&redirected_capture_dir, &cache_dir)
        .expect("replace capture dir with symlink");

    let refusal =
        apply_pipeline_capture(&repo_root, &preview.plan.capture_id).expect_err("refusal");

    assert_eq!(
        refusal.classification,
        PipelineCaptureRefusalClassification::TamperedCaptureCache
    );
    assert!(
        refusal.summary.contains("capture cache path"),
        "expected cache-path summary, got: {}",
        refusal.summary
    );
    assert!(
        refusal
            .summary
            .contains("cannot be written through symlink"),
        "expected symlink refusal, got: {}",
        refusal.summary
    );
    assert_eq!(
        fs::read_to_string(repo_root.join("artifacts/charter/CHARTER.md")).expect("artifact"),
        initial_artifact
    );
    assert_eq!(
        fs::read_to_string(&target_path).expect("mirror"),
        initial_repo_mirror
    );
    assert_eq!(load_route_state(&repo_root), initial_state);
    assert!(
        fs::symlink_metadata(&cache_dir)
            .expect("cache dir metadata")
            .file_type()
            .is_symlink(),
        "refused apply should leave the symlinked cache directory in place"
    );
    assert!(
        cache_path
            .parent()
            .expect("cache parent")
            .ends_with("capture"),
        "test should keep the redirected cache file under the capture directory"
    );
}

#[test]
fn capture_apply_stage_05_guidance_warns_about_resolve_before_follow_up_capture() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_stage_05_capture_ready_repo();
    let result = capture_pipeline_output(&repo_root, &stage_05_request(stage_05_capture_input()))
        .expect("capture");
    let rendered = render_pipeline_capture_apply_result(&result);
    let next_safe_action = capture_next_safe_action(&rendered);

    assert!(
        next_safe_action.contains(
            "handbook pipeline state set --id pipeline.foundation_inputs --var needs_project_context=<true|false>"
        ),
        "stage-05 apply guidance should include the manual route variable step"
    );
    assert!(
        next_safe_action.contains("handbook pipeline resolve --id pipeline.foundation_inputs"),
        "stage-05 apply guidance should include route refresh before follow-up work"
    );

    apply_mutation(
        &repo_root,
        RouteStateMutation::RoutingVariable {
            variable: "needs_project_context".to_string(),
            value: true,
        },
    );

    let refusal = preview_pipeline_capture(&repo_root, &stage_07_request(stage_07_capture_input()))
        .expect_err("stale route basis refusal");
    assert_eq!(
        refusal.classification,
        PipelineCaptureRefusalClassification::StaleRouteBasis
    );
    assert!(
        refusal
            .recovery
            .contains("handbook pipeline resolve --id pipeline.foundation_inputs"),
        "follow-up capture without resolve should still require route refresh"
    );
}

#[test]
fn capture_apply_stage_05_guidance_survives_preexisting_manual_route_value() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_stage_05_capture_ready_repo();
    apply_mutation(
        &repo_root,
        RouteStateMutation::RoutingVariable {
            variable: "needs_project_context".to_string(),
            value: true,
        },
    );
    let _ = pipeline_proof_corpus_support::persist_foundation_inputs_route_basis(&repo_root);

    let result = capture_pipeline_output(&repo_root, &stage_05_request(stage_05_capture_input()))
        .expect("capture");
    let rendered = render_pipeline_capture_apply_result(&result);
    let next_safe_action = capture_next_safe_action(&rendered);

    assert!(
        next_safe_action.contains(
            "handbook pipeline state set --id pipeline.foundation_inputs --var needs_project_context=<true|false>"
        ),
        "stage-05 apply guidance should still include the manual route variable step after re-capture"
    );
    assert!(
        next_safe_action.contains("handbook pipeline resolve --id pipeline.foundation_inputs"),
        "stage-05 apply guidance should still include route refresh after re-capture"
    );
}

#[test]
fn capture_preview_and_cached_apply_stage_05_guidance_survive_preexisting_manual_route_value() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_stage_05_capture_ready_repo();
    apply_mutation(
        &repo_root,
        RouteStateMutation::RoutingVariable {
            variable: "needs_project_context".to_string(),
            value: true,
        },
    );
    let _ = pipeline_proof_corpus_support::persist_foundation_inputs_route_basis(&repo_root);

    let preview = preview_pipeline_capture(&repo_root, &stage_05_request(stage_05_capture_input()))
        .expect("preview");
    assert_eq!(
        preview.plan.post_apply_next_safe_action.as_deref(),
        Some(
            "run `handbook pipeline state set --id pipeline.foundation_inputs --var needs_project_context=<true|false>`, then run `handbook pipeline resolve --id pipeline.foundation_inputs` before the next compile or capture"
        )
    );

    let result = apply_pipeline_capture(&repo_root, &preview.plan.capture_id).expect("apply");
    let rendered = render_pipeline_capture_apply_result(&result);
    let next_safe_action = capture_next_safe_action(&rendered);

    assert!(
        next_safe_action.contains(
            "handbook pipeline state set --id pipeline.foundation_inputs --var needs_project_context=<true|false>"
        ),
        "cached stage-05 apply guidance should still include the manual route variable step after re-capture"
    );
    assert!(
        next_safe_action.contains("handbook pipeline resolve --id pipeline.foundation_inputs"),
        "cached stage-05 apply guidance should still include route refresh after re-capture"
    );
}

#[test]
fn capture_apply_refuses_tampered_artifact_path_without_side_effects() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_stage_05_capture_ready_repo();
    let preview = preview_pipeline_capture(&repo_root, &stage_05_request(stage_05_capture_input()))
        .expect("preview");
    let initial_artifact =
        fs::read_to_string(repo_root.join("artifacts/charter/CHARTER.md")).expect("artifact");
    let initial_repo_mirror = fs::read_to_string(repo_root.join("CHARTER.md")).expect("mirror");
    let initial_state = load_route_state(&repo_root);

    let tampered_capture_id =
        rewrite_tampered_capture_cache(&repo_root, &preview.plan.capture_id, |cache_entry| {
            cache_entry.plan.artifact_writes[0].path = "docs/EVIL.md".to_string();
        });

    let refusal = apply_pipeline_capture(&repo_root, &tampered_capture_id).expect_err("refusal");

    assert_eq!(
        refusal.classification,
        PipelineCaptureRefusalClassification::TamperedCaptureCache
    );
    assert_eq!(
        fs::read_to_string(repo_root.join("artifacts/charter/CHARTER.md")).expect("artifact"),
        initial_artifact
    );
    assert_eq!(
        fs::read_to_string(repo_root.join("CHARTER.md")).expect("mirror"),
        initial_repo_mirror
    );
    assert_eq!(load_route_state(&repo_root), initial_state);
    assert!(
        pipeline_proof_corpus_support::pipeline_capture_cache_path(
            &repo_root,
            &tampered_capture_id
        )
        .exists(),
        "refused apply should keep the tampered cache entry for inspection"
    );
}

#[test]
fn capture_apply_refuses_tampered_repo_mirror_writes_without_side_effects() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_stage_05_capture_ready_repo();
    let preview = preview_pipeline_capture(&repo_root, &stage_05_request(stage_05_capture_input()))
        .expect("preview");
    let initial_artifact =
        fs::read_to_string(repo_root.join("artifacts/charter/CHARTER.md")).expect("artifact");
    let initial_repo_mirror = fs::read_to_string(repo_root.join("CHARTER.md")).expect("mirror");
    let initial_state = load_route_state(&repo_root);

    let tampered_capture_id =
        rewrite_tampered_capture_cache(&repo_root, &preview.plan.capture_id, |cache_entry| {
            cache_entry.plan.repo_mirror_writes[0].path = "README.md".to_string();
        });

    let refusal = apply_pipeline_capture(&repo_root, &tampered_capture_id).expect_err("refusal");

    assert_eq!(
        refusal.classification,
        PipelineCaptureRefusalClassification::TamperedCaptureCache
    );
    assert_eq!(
        fs::read_to_string(repo_root.join("artifacts/charter/CHARTER.md")).expect("artifact"),
        initial_artifact
    );
    assert_eq!(
        fs::read_to_string(repo_root.join("CHARTER.md")).expect("mirror"),
        initial_repo_mirror
    );
    assert_eq!(load_route_state(&repo_root), initial_state);
    assert!(
        pipeline_proof_corpus_support::pipeline_capture_cache_path(
            &repo_root,
            &tampered_capture_id
        )
        .exists(),
        "refused apply should keep the tampered cache entry for inspection"
    );
}

#[test]
fn capture_apply_refuses_tampered_state_updates_without_side_effects() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_stage_05_capture_ready_repo();
    let preview = preview_pipeline_capture(&repo_root, &stage_05_request(stage_05_capture_input()))
        .expect("preview");
    let initial_artifact =
        fs::read_to_string(repo_root.join("artifacts/charter/CHARTER.md")).expect("artifact");
    let initial_repo_mirror = fs::read_to_string(repo_root.join("CHARTER.md")).expect("mirror");
    let initial_state = load_route_state(&repo_root);

    let tampered_capture_id =
        rewrite_tampered_capture_cache(&repo_root, &preview.plan.capture_id, |cache_entry| {
            cache_entry
                .plan
                .state_updates
                .push(PipelineCaptureStateUpdate {
                    field_path: "refs.project_context_ref".to_string(),
                    value: PipelineCaptureStateValue::String(
                        "artifacts/project_context/PROJECT_CONTEXT.md".to_string(),
                    ),
                });
        });

    let refusal = apply_pipeline_capture(&repo_root, &tampered_capture_id).expect_err("refusal");

    assert_eq!(
        refusal.classification,
        PipelineCaptureRefusalClassification::TamperedCaptureCache
    );
    assert_eq!(
        fs::read_to_string(repo_root.join("artifacts/charter/CHARTER.md")).expect("artifact"),
        initial_artifact
    );
    assert_eq!(
        fs::read_to_string(repo_root.join("CHARTER.md")).expect("mirror"),
        initial_repo_mirror
    );
    assert_eq!(load_route_state(&repo_root), initial_state);
    assert!(
        pipeline_proof_corpus_support::pipeline_capture_cache_path(
            &repo_root,
            &tampered_capture_id
        )
        .exists(),
        "refused apply should keep the tampered cache entry for inspection"
    );
}

#[test]
fn capture_apply_refuses_unsupported_stage_id_from_tampered_cache_without_side_effects() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_stage_05_capture_ready_repo();
    let preview = preview_pipeline_capture(&repo_root, &stage_05_request(stage_05_capture_input()))
        .expect("preview");
    let initial_artifact =
        fs::read_to_string(repo_root.join("artifacts/charter/CHARTER.md")).expect("artifact");
    let initial_repo_mirror = fs::read_to_string(repo_root.join("CHARTER.md")).expect("mirror");
    let initial_state = load_route_state(&repo_root);

    let tampered_capture_id =
        rewrite_tampered_capture_cache(&repo_root, &preview.plan.capture_id, |cache_entry| {
            cache_entry.plan.target.stage_id = "stage.99_not_supported".to_string();
        });

    let refusal = apply_pipeline_capture(&repo_root, &tampered_capture_id).expect_err("refusal");

    assert_eq!(
        refusal.classification,
        PipelineCaptureRefusalClassification::UnsupportedTarget
    );
    assert_eq!(
        refusal.summary,
        "`pipeline capture` currently supports only stages `stage.04_charter_inputs`, `stage.05_charter_synthesize`, `stage.06_project_context_interview`, `stage.07_foundation_pack`, `stage.10_feature_spec` for pipeline `pipeline.foundation_inputs`"
    );
    assert!(
        !refusal.summary.contains("M3"),
        "unsupported-target summary should not mention milestones: {}",
        refusal.summary
    );
    assert_eq!(
        fs::read_to_string(repo_root.join("artifacts/charter/CHARTER.md")).expect("artifact"),
        initial_artifact
    );
    assert_eq!(
        fs::read_to_string(repo_root.join("CHARTER.md")).expect("mirror"),
        initial_repo_mirror
    );
    assert_eq!(load_route_state(&repo_root), initial_state);
    assert!(
        pipeline_proof_corpus_support::pipeline_capture_cache_path(
            &repo_root,
            &tampered_capture_id
        )
        .exists(),
        "refused apply should keep the tampered cache entry for inspection"
    );
}
