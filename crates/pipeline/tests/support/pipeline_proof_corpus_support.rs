#![allow(dead_code)]

use std::fs;
use std::path::{Path, PathBuf};

use handbook_pipeline::{
    build_route_basis, load_pipeline_definition, load_route_state_with_supported_variables,
    persist_route_basis, resolve_pipeline_route, set_route_state, supported_route_state_variables,
    PipelineCompileRefusal, PipelineCompileRefusalClassification, ResolvedPipelineRoute,
    RouteBasisPersistOutcome, RouteStageReason, RouteState, RouteStateMutation,
    RouteStateMutationOutcome, RouteStateRun, RouteVariables,
};

pub const FOUNDATION_INPUTS_PIPELINE_ID: &str = "pipeline.foundation_inputs";
pub const STAGE_04_CHARTER_INPUTS_ID: &str = "stage.04_charter_inputs";
pub const STAGE_05_CHARTER_SYNTHESIZE_ID: &str = "stage.05_charter_synthesize";
pub const STAGE_06_PROJECT_CONTEXT_INTERVIEW_ID: &str = "stage.06_project_context_interview";
pub const STAGE_07_FOUNDATION_PACK_ID: &str = "stage.07_foundation_pack";
pub const STAGE_10_FEATURE_SPEC_ID: &str = "stage.10_feature_spec";
pub const CAPTURE_GOLDEN_NAMES: &[&str] = &[
    "capture.preview.stage_04_charter_inputs.txt",
    "capture.preview.stage_05_charter_synthesize.txt",
    "capture.preview.stage_06_project_context_interview.txt",
    "capture.preview.stage_07_foundation_pack.txt",
    "capture.preview.stage_10_feature_spec.txt",
    "capture.apply.stage_04_charter_inputs.txt",
    "capture.apply.stage_05_charter_synthesize.txt",
    "capture.apply.stage_06_project_context_interview.txt",
    "capture.apply.stage_07_foundation_pack.txt",
    "capture.apply.stage_10_feature_spec.txt",
    "capture.refused.empty_single_file_body.txt",
    "capture.refused.empty_declared_block.txt",
    "capture.refused.single_file_with_file_wrapper.txt",
    "capture.refused.missing_declared_block.txt",
    "capture.refused.duplicate_declared_block.txt",
    "capture.refused.undeclared_block.txt",
    "capture.refused.stale_route_basis.txt",
    "capture.refused.inactive_stage.txt",
    "capture.refused.missing_capture_id.txt",
];

pub fn committed_repo_root() -> PathBuf {
    committed_case_root().join("repo")
}

pub fn install_foundation_inputs_repo() -> (tempfile::TempDir, PathBuf) {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path().to_path_buf();
    copy_tree(&committed_repo_root(), &root);
    (dir, root)
}

pub fn pipeline_state_path(repo_root: &Path) -> PathBuf {
    repo_root
        .join(".handbook")
        .join("state")
        .join("pipeline")
        .join(format!("{FOUNDATION_INPUTS_PIPELINE_ID}.yaml"))
}

pub fn install_state_seed(repo_root: &Path, seed_name: &str) -> PathBuf {
    let source = committed_case_root().join("state_seeds").join(seed_name);
    let target = pipeline_state_path(repo_root);

    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent).expect("state seed parent");
    }

    fs::copy(&source, &target).unwrap_or_else(|err| {
        panic!(
            "copy state seed {} -> {}: {err}",
            source.display(),
            target.display()
        )
    });
    target
}

pub fn committed_fixture_path(relative_path: &str) -> PathBuf {
    committed_repo_root().join(relative_path)
}

pub fn read_committed_fixture(relative_path: &str) -> String {
    let path = committed_fixture_path(relative_path);
    normalize_newlines(
        &fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {}: {err}", path.display())),
    )
}

pub fn read_committed_model_output(relative_path: &str) -> String {
    let path = committed_case_root()
        .join("model_outputs")
        .join(relative_path);
    normalize_newlines(
        &fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {}: {err}", path.display())),
    )
}

pub fn read_golden_fixture(golden_name: &str) -> String {
    read_golden(golden_name)
}

pub fn assert_capture_golden_inventory_is_committed() {
    for golden_name in CAPTURE_GOLDEN_NAMES {
        let contents = read_golden(golden_name);
        assert!(
            !contents.trim().is_empty(),
            "capture golden {golden_name} must not be empty"
        );
    }
}

pub fn pipeline_capture_cache_path(repo_root: &Path, capture_id: &str) -> PathBuf {
    repo_root
        .join(".handbook")
        .join("state")
        .join("pipeline")
        .join("capture")
        .join(format!("{capture_id}.yaml"))
}

pub fn load_foundation_inputs_definition(
    repo_root: &Path,
) -> (
    handbook_pipeline::PipelineDefinition,
    std::collections::BTreeSet<String>,
) {
    let definition = load_pipeline_definition(repo_root, "core/pipelines/foundation_inputs.yaml")
        .expect("pipeline fixture");
    let supported_variables = supported_route_state_variables(&definition);
    (definition, supported_variables)
}

pub fn apply_foundation_inputs_state_mutation(
    repo_root: &Path,
    supported_variables: &std::collections::BTreeSet<String>,
    mutation: RouteStateMutation,
) {
    let state = load_route_state_with_supported_variables(
        repo_root,
        FOUNDATION_INPUTS_PIPELINE_ID,
        supported_variables,
    )
    .expect("load route state");
    let outcome = set_route_state(
        repo_root,
        FOUNDATION_INPUTS_PIPELINE_ID,
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

pub fn persist_foundation_inputs_route_basis(repo_root: &Path) -> RouteState {
    let (definition, supported_variables) = load_foundation_inputs_definition(repo_root);
    let state = load_route_state_with_supported_variables(
        repo_root,
        FOUNDATION_INPUTS_PIPELINE_ID,
        &supported_variables,
    )
    .expect("load route state");
    let route = resolve_pipeline_route(
        &definition,
        &RouteVariables::new(state.routing.clone()).expect("route variables"),
    )
    .expect("resolve route");
    let route_basis = build_route_basis(repo_root, &definition, &state, &route).expect("basis");
    let outcome = persist_route_basis(repo_root, FOUNDATION_INPUTS_PIPELINE_ID, route_basis)
        .expect("persist route basis");

    match outcome {
        RouteBasisPersistOutcome::Applied(state) => *state,
        RouteBasisPersistOutcome::Refused(refusal) => {
            panic!("expected route basis persist to apply, got {refusal:?}")
        }
    }
}

pub fn install_stage_04_capture_ready_repo() -> (tempfile::TempDir, PathBuf) {
    let (dir, repo_root) = install_foundation_inputs_repo();
    let _ = persist_foundation_inputs_route_basis(&repo_root);
    (dir, repo_root)
}

pub fn install_stage_05_capture_ready_repo() -> (tempfile::TempDir, PathBuf) {
    install_stage_04_capture_ready_repo()
}

pub fn install_stage_06_capture_ready_repo() -> (tempfile::TempDir, PathBuf) {
    let (dir, repo_root) = install_foundation_inputs_repo();
    let (_, supported_variables) = load_foundation_inputs_definition(&repo_root);
    apply_foundation_inputs_state_mutation(
        &repo_root,
        &supported_variables,
        RouteStateMutation::RefCharterRef {
            value: "artifacts/charter/CHARTER.md".to_string(),
        },
    );
    apply_foundation_inputs_state_mutation(
        &repo_root,
        &supported_variables,
        RouteStateMutation::RoutingVariable {
            variable: "needs_project_context".to_string(),
            value: true,
        },
    );
    let _ = persist_foundation_inputs_route_basis(&repo_root);
    (dir, repo_root)
}

pub fn install_stage_07_capture_ready_repo() -> (tempfile::TempDir, PathBuf) {
    let (dir, repo_root) = install_foundation_inputs_repo();
    let (_, supported_variables) = load_foundation_inputs_definition(&repo_root);
    apply_foundation_inputs_state_mutation(
        &repo_root,
        &supported_variables,
        RouteStateMutation::RefCharterRef {
            value: "artifacts/charter/CHARTER.md".to_string(),
        },
    );
    apply_foundation_inputs_state_mutation(
        &repo_root,
        &supported_variables,
        RouteStateMutation::RoutingVariable {
            variable: "needs_project_context".to_string(),
            value: false,
        },
    );
    apply_foundation_inputs_state_mutation(
        &repo_root,
        &supported_variables,
        RouteStateMutation::RoutingVariable {
            variable: "charter_gaps_detected".to_string(),
            value: false,
        },
    );
    let _ = persist_foundation_inputs_route_basis(&repo_root);
    (dir, repo_root)
}

pub fn install_stage_10_capture_ready_repo() -> (tempfile::TempDir, PathBuf) {
    let (dir, repo_root) = install_foundation_inputs_repo();
    let (_, supported_variables) = load_foundation_inputs_definition(&repo_root);
    apply_foundation_inputs_state_mutation(
        &repo_root,
        &supported_variables,
        RouteStateMutation::RunRunner {
            value: "codex-cli".to_string(),
        },
    );
    apply_foundation_inputs_state_mutation(
        &repo_root,
        &supported_variables,
        RouteStateMutation::RunProfile {
            value: "python-uv".to_string(),
        },
    );
    apply_foundation_inputs_state_mutation(
        &repo_root,
        &supported_variables,
        RouteStateMutation::RefCharterRef {
            value: "artifacts/charter/CHARTER.md".to_string(),
        },
    );
    apply_foundation_inputs_state_mutation(
        &repo_root,
        &supported_variables,
        RouteStateMutation::RefProjectContextRef {
            value: "artifacts/project_context/PROJECT_CONTEXT.md".to_string(),
        },
    );
    apply_foundation_inputs_state_mutation(
        &repo_root,
        &supported_variables,
        RouteStateMutation::RoutingVariable {
            variable: "needs_project_context".to_string(),
            value: false,
        },
    );
    apply_foundation_inputs_state_mutation(
        &repo_root,
        &supported_variables,
        RouteStateMutation::RoutingVariable {
            variable: "charter_gaps_detected".to_string(),
            value: false,
        },
    );
    let _ = persist_foundation_inputs_route_basis(&repo_root);
    (dir, repo_root)
}

pub fn assert_matches_golden(
    actual: &str,
    repo_root: &Path,
    state_path: Option<&Path>,
    golden_name: &str,
) {
    let mut placeholders = Vec::new();
    if let Some(state_path) = state_path {
        placeholders.push((state_path, "{{STATE_PATH}}"));
    }
    assert_matches_golden_with_placeholders(actual, repo_root, &placeholders, golden_name);
}

pub fn assert_matches_golden_with_placeholders(
    actual: &str,
    repo_root: &Path,
    placeholders: &[(&Path, &str)],
    golden_name: &str,
) {
    let normalized_actual = normalize_output(actual, repo_root, placeholders);
    assert_matches_normalized_output(&normalized_actual, golden_name);
}

#[allow(dead_code)]
pub fn assert_matches_golden_with_explicit_placeholders(
    actual: &str,
    placeholders: &[(&Path, &str)],
    golden_name: &str,
) {
    let normalized_actual = normalize_output_with_explicit_placeholders(actual, placeholders);
    assert_matches_normalized_output(&normalized_actual, golden_name);
}

fn assert_matches_normalized_output(actual: &str, golden_name: &str) {
    let expected = read_golden(golden_name);
    assert_eq!(
        actual,
        expected,
        "pipeline proof output drifted for {golden_name}; update the golden at {} if intentional",
        committed_case_root()
            .join("goldens")
            .join(golden_name)
            .display()
    );
}

#[allow(dead_code)]
pub fn assert_compile_refusal_matches_shared_golden(
    refusal: &PipelineCompileRefusal,
    pipeline_id: &str,
    stage_id: &str,
    golden_name: &str,
) {
    let expected = parse_compile_refusal_golden(golden_name);
    let actual_pipeline_id = refusal.pipeline_id.as_deref().unwrap_or(pipeline_id);
    let actual_stage_id = refusal.stage_id.as_deref().unwrap_or(stage_id);

    assert_eq!(
        actual_pipeline_id, expected.pipeline_id,
        "pipeline proof output drifted for {golden_name}; pipeline id no longer matches the shared golden"
    );
    assert_eq!(
        actual_stage_id, expected.stage_id,
        "pipeline proof output drifted for {golden_name}; stage id no longer matches the shared golden"
    );
    assert_eq!(
        refusal.classification.to_string(),
        expected.reason_classification,
        "pipeline proof output drifted for {golden_name}; refusal classification no longer matches the shared golden"
    );
    assert_compile_refusal_summary_matches_shared_golden(
        refusal,
        &expected.reason_summary,
        golden_name,
    );
    assert_compile_refusal_next_safe_action_matches_shared_golden(
        refusal,
        pipeline_id,
        stage_id,
        &expected.next_safe_action,
        golden_name,
    );
}

pub fn render_pipeline_resolve_output(
    pipeline_id: &str,
    state: &RouteState,
    effective_run: &RouteStateRun,
    route: &ResolvedPipelineRoute,
) -> String {
    let mut out = String::new();
    out.push_str("OUTCOME: RESOLVED\n");
    out.push_str(&format!("PIPELINE: {pipeline_id}\n"));
    out.push_str("ROUTE BASIS:\n");
    out.push_str(&format!("  revision = {}\n", state.revision));
    out.push_str("  routing:\n");
    if state.routing.is_empty() {
        out.push_str("    <empty>\n");
    } else {
        for (name, value) in &state.routing {
            out.push_str(&format!("    {} = {}\n", name, value));
        }
    }
    out.push_str("  refs:\n");
    render_optional_route_basis_field(&mut out, "charter_ref", state.refs.charter_ref.as_deref());
    render_optional_route_basis_field(
        &mut out,
        "project_context_ref",
        state.refs.project_context_ref.as_deref(),
    );
    out.push_str("  run:\n");
    render_optional_route_basis_field(&mut out, "runner", effective_run.runner.as_deref());
    render_optional_route_basis_field(&mut out, "profile", effective_run.profile.as_deref());
    render_optional_route_basis_field(&mut out, "repo_root", effective_run.repo_root.as_deref());
    out.push_str("ROUTE:\n");

    for (index, stage) in route.stages.iter().enumerate() {
        out.push_str(&format!(
            "  {}. {} | {}\n",
            index + 1,
            stage.stage_id,
            stage.status.as_str()
        ));
        if let Some(reason) = &stage.reason {
            out.push_str(&format!(
                "     REASON: {}\n",
                render_route_stage_reason(reason)
            ));
        }
    }

    out.trim_end().to_string()
}

pub fn render_pipeline_state_set_output(
    pipeline_id: &str,
    outcome: RouteStateMutationOutcome,
) -> String {
    let mut out = String::new();
    match outcome {
        RouteStateMutationOutcome::Applied(state) => {
            let state = *state;
            out.push_str("OUTCOME: APPLIED\n");
            out.push_str(&format!("PIPELINE: {pipeline_id}\n"));
            out.push_str(&format!("REVISION: {}\n", state.revision));
            out.push_str("ROUTING:\n");
            if state.routing.is_empty() {
                out.push_str("  <empty>\n");
            } else {
                for (name, value) in state.routing {
                    out.push_str(&format!("  {} = {}\n", name, value));
                }
            }
            out.push_str("REFS:\n");
            render_optional_state_field(&mut out, "charter_ref", state.refs.charter_ref.as_deref());
            render_optional_state_field(
                &mut out,
                "project_context_ref",
                state.refs.project_context_ref.as_deref(),
            );
            out.push_str("RUN:\n");
            render_optional_state_field(&mut out, "runner", state.run.runner.as_deref());
            render_optional_state_field(&mut out, "profile", state.run.profile.as_deref());
            render_optional_state_field(&mut out, "repo_root", state.run.repo_root.as_deref());
        }
        RouteStateMutationOutcome::Refused(refusal) => {
            out.push_str("OUTCOME: REFUSED\n");
            out.push_str(&format!("PIPELINE: {pipeline_id}\n"));
            out.push_str(&format!("REASON: {}\n", refusal));
        }
    }

    out.trim_end().to_string()
}

pub fn render_load_route_state_refusal(err: impl std::fmt::Display) -> String {
    format!("REFUSED: {err}")
}

fn render_optional_route_basis_field(out: &mut String, name: &str, value: Option<&str>) {
    match value {
        Some(value) => out.push_str(&format!("    {} = {}\n", name, value)),
        None => out.push_str(&format!("    {} = <unset>\n", name)),
    }
}

fn render_optional_state_field(out: &mut String, name: &str, value: Option<&str>) {
    match value {
        Some(value) => out.push_str(&format!("  {} = {}\n", name, value)),
        None => out.push_str(&format!("  {} = <unset>\n", name)),
    }
}

fn render_route_stage_reason(reason: &RouteStageReason) -> String {
    match reason {
        RouteStageReason::SkippedActivationFalse {
            unsatisfied_variables,
            ..
        } => format!(
            "activation evaluated false for variables: {}",
            unsatisfied_variables.join(", ")
        ),
        RouteStageReason::NextMissingRouteVariables {
            missing_variables, ..
        } => format!("missing route variables: {}", missing_variables.join(", ")),
        RouteStageReason::BlockedByUnresolvedStage {
            upstream_stage_id,
            upstream_status,
        } => format!(
            "blocked by unresolved stage {} ({})",
            upstream_stage_id,
            upstream_status.as_str()
        ),
    }
}

fn normalize_output(actual: &str, repo_root: &Path, placeholders: &[(&Path, &str)]) -> String {
    let mut normalized = normalize_output_with_explicit_placeholders(actual, placeholders);
    normalized = replace_path_candidates(&normalized, repo_root, "{{REPO_ROOT}}");
    normalized.trim_end().to_string()
}

fn normalize_output_with_explicit_placeholders(
    actual: &str,
    placeholders: &[(&Path, &str)],
) -> String {
    let mut normalized = normalize_newlines(actual);

    for (path, placeholder) in placeholders {
        normalized = replace_path_candidates(&normalized, path, placeholder);
    }

    normalized.trim_end().to_string()
}

#[allow(dead_code)]
struct ParsedCompileRefusalGolden {
    pipeline_id: String,
    stage_id: String,
    reason_classification: String,
    reason_summary: String,
    next_safe_action: String,
}

#[allow(dead_code)]
fn parse_compile_refusal_golden(golden_name: &str) -> ParsedCompileRefusalGolden {
    let golden = read_golden(golden_name);
    let pipeline_id = parse_golden_field(&golden, "PIPELINE: ", golden_name);
    let stage_id = parse_golden_field(&golden, "STAGE: ", golden_name);
    let reason = parse_golden_field(&golden, "REASON: ", golden_name);
    let (reason_classification, reason_summary) = reason.split_once(": ").unwrap_or_else(|| {
        panic!(
            "golden {golden_name} has malformed REASON field; expected `classification: summary`"
        )
    });
    let next_safe_action = parse_golden_field(&golden, "NEXT SAFE ACTION: ", golden_name);

    ParsedCompileRefusalGolden {
        pipeline_id,
        stage_id,
        reason_classification: reason_classification.to_string(),
        reason_summary: reason_summary.to_string(),
        next_safe_action,
    }
}

#[allow(dead_code)]
fn parse_golden_field(golden: &str, prefix: &str, golden_name: &str) -> String {
    golden
        .lines()
        .find_map(|line| line.strip_prefix(prefix))
        .map(str::to_string)
        .unwrap_or_else(|| panic!("golden {golden_name} is missing required field `{prefix}`"))
}

#[allow(dead_code)]
fn assert_compile_refusal_next_safe_action_matches_shared_golden(
    refusal: &PipelineCompileRefusal,
    pipeline_id: &str,
    stage_id: &str,
    next_safe_action: &str,
    golden_name: &str,
) {
    let expected_refresh_action = format!(
        "run `handbook pipeline resolve --id {pipeline_id}` and then retry `handbook pipeline compile --id {pipeline_id} --stage {stage_id}`"
    );

    match refusal.classification {
        PipelineCompileRefusalClassification::MissingRouteBasis
        | PipelineCompileRefusalClassification::MalformedRouteBasis
        | PipelineCompileRefusalClassification::StaleRouteBasis => {
            assert_eq!(
                next_safe_action, expected_refresh_action,
                "pipeline proof output drifted for {golden_name}; next-safe-action no longer matches the shared golden"
            );
            assert_eq!(
                refusal.recovery.trim(),
                "re-run `pipeline resolve` and retry `pipeline compile`",
                "pipeline proof output drifted for {golden_name}; compiler recovery no longer supports the shared next-safe-action contract"
            );
        }
        PipelineCompileRefusalClassification::InactiveStage => {
            let expected_inactive_stage_action = format!(
                "run `handbook pipeline resolve --id {pipeline_id}`, adjust route state if needed, and then retry `handbook pipeline compile --id {pipeline_id} --stage {stage_id}`"
            );
            assert_eq!(
                next_safe_action, expected_inactive_stage_action,
                "pipeline proof output drifted for {golden_name}; next-safe-action no longer matches the shared golden"
            );
            assert_eq!(
                refusal.recovery.trim(),
                "re-run `pipeline resolve`, adjust route state if needed, and retry `pipeline compile`",
                "pipeline proof output drifted for {golden_name}; compiler recovery no longer supports the shared next-safe-action contract"
            );
        }
        other => panic!(
            "assert_compile_refusal_matches_shared_golden does not support {:?} for {}",
            other, golden_name
        ),
    }
}

#[allow(dead_code)]
fn assert_compile_refusal_summary_matches_shared_golden(
    refusal: &PipelineCompileRefusal,
    expected_summary: &str,
    golden_name: &str,
) {
    match refusal.classification {
        PipelineCompileRefusalClassification::StaleRouteBasis => {
            parse_stale_route_basis_summary(expected_summary, golden_name);
            parse_stale_route_basis_summary(refusal.summary.trim(), golden_name);
        }
        _ => assert_eq!(
            refusal.summary.trim(),
            expected_summary,
            "pipeline proof output drifted for {golden_name}; refusal summary no longer matches the shared golden"
        ),
    }
}

#[allow(dead_code)]
fn parse_stale_route_basis_summary<'a>(summary: &'a str, golden_name: &str) -> (&'a str, &'a str) {
    let prefix = "route state revision ";
    let middle = " does not match persisted route_basis revision ";
    let revision_text = summary.strip_prefix(prefix).unwrap_or_else(|| {
        panic!(
            "golden {golden_name} has malformed stale-route-basis summary; expected prefix `{prefix}`"
        )
    });
    let (state_revision, basis_revision) =
        revision_text.split_once(middle).unwrap_or_else(|| {
            panic!(
                "golden {golden_name} has malformed stale-route-basis summary; expected separator `{middle}`"
            )
        });
    assert!(
        !state_revision.is_empty()
            && state_revision
                .chars()
                .all(|character| character.is_ascii_digit()),
        "golden {golden_name} has malformed stale-route-basis state revision"
    );
    assert!(
        !basis_revision.is_empty()
            && basis_revision
                .chars()
                .all(|character| character.is_ascii_digit()),
        "golden {golden_name} has malformed stale-route-basis basis revision"
    );

    (state_revision, basis_revision)
}

fn read_golden(golden_name: &str) -> String {
    let golden_path = committed_case_root().join("goldens").join(golden_name);
    normalize_newlines(
        &fs::read_to_string(&golden_path)
            .unwrap_or_else(|err| panic!("read {}: {err}", golden_path.display())),
    )
    .trim_end()
    .to_string()
}

fn normalize_newlines(value: &str) -> String {
    value.replace("\r\n", "\n")
}

fn path_candidates(path: &Path) -> Vec<String> {
    let mut candidates = vec![path.display().to_string()];
    if let Ok(canonical) = fs::canonicalize(path) {
        let canonical_display = canonical.display().to_string();
        if !candidates.contains(&canonical_display) {
            candidates.push(canonical_display);
        }
    }
    candidates.sort();
    candidates.dedup();
    candidates
}

fn replace_path_candidates(content: &str, path: &Path, placeholder: &str) -> String {
    let mut updated = content.to_string();
    for candidate in path_candidates(path) {
        updated = updated.replace(&candidate, placeholder);
    }
    updated
}

fn committed_case_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("tests")
        .join("fixtures")
        .join("pipeline_proof_corpus")
        .join("foundation_inputs")
}

fn copy_tree(source: &Path, target: &Path) {
    for entry in
        fs::read_dir(source).unwrap_or_else(|err| panic!("read {}: {err}", source.display()))
    {
        let entry = entry.unwrap_or_else(|err| panic!("dir entry {}: {err}", source.display()));
        let file_type = entry
            .file_type()
            .unwrap_or_else(|err| panic!("file_type {}: {err}", entry.path().display()));
        let from = entry.path();
        let to = target.join(entry.file_name());

        if file_type.is_dir() {
            fs::create_dir_all(&to).unwrap_or_else(|err| panic!("mkdir {}: {err}", to.display()));
            copy_tree(&from, &to);
        } else if file_type.is_file() {
            if let Some(parent) = to.parent() {
                fs::create_dir_all(parent)
                    .unwrap_or_else(|err| panic!("mkdir {}: {err}", parent.display()));
            }
            fs::copy(&from, &to)
                .unwrap_or_else(|err| panic!("copy {} -> {}: {err}", from.display(), to.display()));
        } else {
            panic!("unsupported proof corpus entry {}", from.display());
        }
    }
}
