use crate::layout::RepoLayoutRoot;
use crate::pipeline::{
    load_selected_pipeline_definition, load_stage_compile_definition,
    supported_route_state_variables, CompileStageDefinition, PipelineDefinition,
    SelectedPipelineLoadError, SupportedTargetRegistry,
};
use crate::pipeline_compile::{compile_pipeline_stage, PipelineCompileRefusal};
use crate::repo_file_access::{
    delete_repo_relative_file, read_bytes_no_follow_path, read_string_no_follow_path,
    resolve_repo_relative_write_path, validate_repo_relative_path, write_repo_relative_bytes,
    NormalizedRepoRelativePath, RepoRelativeMutationError, RepoRelativeWritePathError,
};
use crate::route_state::{
    acquire_advisory_lock, effective_route_basis_run, load_route_state_with_supported_variables,
    normalize_route_basis_run, normalized_state_for_persistence, persist_route_state,
    rebuild_canonical_route_basis, route_basis_mismatch_reason, route_state_path, RouteBasis,
    RouteBasisStageStatus, RouteState, RouteStateAuditEntry, RouteStateReadError, RouteStateValue,
    ROUTE_STATE_AUDIT_LIMIT, ROUTE_STATE_SCHEMA_VERSION,
};
use crate::stage_10_feature_spec_provenance::{
    build_stage_10_feature_spec_capture_provenance,
    persist_stage_10_feature_spec_capture_provenance, sha256_hex, FEATURE_SPEC_ARTIFACT_PATH,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fs;
use std::io::Write;
use std::path::{Component, Path, PathBuf};

const SUPPORTED_STAGE_FEATURE_SPEC_ID: &str = "stage.10_feature_spec";
const REQUIRED_STAGE_10_FEATURE_SPEC_HEADINGS: [&str; 23] = [
    "## 0) Charter Alignment",
    "### Dimension alignment",
    "### Charter red lines check",
    "## 1) Summary",
    "## 2) Problem & Context",
    "## 3) Goals",
    "## 4) Non-Goals",
    "## 5) Users & Stakeholders",
    "## 6) Scope",
    "### In Scope",
    "### Out of Scope",
    "## 7) Requirements",
    "### Functional Requirements",
    "### Non-Functional Requirements (NFRs)",
    "## 8) Acceptance Criteria (testable)",
    "## 9) Technical Design",
    "### Proposed Approach (recommended)",
    "## 10) Alternatives Considered",
    "## 11) Testing Strategy",
    "## 12) Rollout Plan",
    "## 13) Risks & Mitigations",
    "## 15) Traceability Map",
    "## Y) Debt Tracking (Charter-aligned)",
];
const CAPTURE_CACHE_SCHEMA_VERSION: &str = "m3-capture-cache-v1";
const CAPTURE_UNKNOWN_MARKERS: [&str; 5] = ["TBD", "UNKNOWN", "Unknown", "TODO", "??"];
pub const PIPELINE_CAPTURE_CACHE_SCHEMA_VERSION: &str = CAPTURE_CACHE_SCHEMA_VERSION;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineCaptureRequest {
    pub pipeline_selector: String,
    pub stage_selector: String,
    pub input: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineCapturePreview {
    pub plan: PipelineCapturePlan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineCaptureApplyResult {
    pub plan: PipelineCapturePlan,
    pub written_files: Vec<String>,
    pub persisted_state_revision: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PipelineCapturePlan {
    pub target: PipelineCaptureTarget,
    pub basis: RouteBasis,
    pub artifact_writes: Vec<PipelineCaptureWriteIntent>,
    pub repo_mirror_writes: Vec<PipelineCaptureWriteIntent>,
    pub state_updates: Vec<PipelineCaptureStateUpdate>,
    pub capture_id: String,
    pub post_apply_next_safe_action: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PipelineCaptureTarget {
    pub pipeline_id: String,
    pub stage_id: String,
    pub stage_file: String,
    pub title: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PipelineCaptureWriteIntent {
    pub path: String,
    pub content: String,
}

pub type PipelineCaptureWrite = PipelineCaptureWriteIntent;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PipelineCaptureStateUpdate {
    pub field_path: String,
    pub value: PipelineCaptureStateValue,
}

pub type PipelineCaptureStateEffect = PipelineCaptureStateUpdate;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PipelineCaptureStateValue {
    Bool(bool),
    String(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PipelineCaptureCacheEntry {
    pub schema_version: String,
    pub capture_id: String,
    pub plan: PipelineCapturePlan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineCaptureRefusal {
    pub classification: PipelineCaptureRefusalClassification,
    pub summary: String,
    pub pipeline_id: Option<String>,
    pub stage_id: Option<String>,
    pub recovery: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PipelineCaptureRefusalClassification {
    UnsupportedTarget,
    InvalidDefinition,
    InvalidState,
    MissingRouteBasis,
    MalformedRouteBasis,
    StaleRouteBasis,
    InactiveStage,
    InvalidCaptureInput,
    InvalidWriteTarget,
    MissingCaptureId,
    TamperedCaptureCache,
    RevisionConflict,
    WriteFailure,
    StatePersistenceFailure,
    CacheFailure,
}

#[derive(Debug)]
enum CaptureInputParseError {
    InvalidSingleFileWrapper,
    EmptySingleFileBody,
    RawStage10CompilePayload,
    InvalidStage10FeatureSpecBody,
    InvalidMultifilePrefix,
    EmptyDeclaredBlock(String),
    DuplicateBlock(String),
    MissingDeclaredBlock(String),
    UndeclaredBlock(String),
}

#[derive(Debug)]
struct SnapshotEntry {
    repo_relative_path: String,
    prior_bytes: Option<Vec<u8>>,
}

pub fn preview_pipeline_capture(
    repo_root: impl AsRef<Path>,
    request: &PipelineCaptureRequest,
) -> Result<PipelineCapturePreview, PipelineCaptureRefusal> {
    let repo_root = repo_root.as_ref();
    let plan = build_capture_plan(
        repo_root,
        &request.pipeline_selector,
        &request.stage_selector,
        &request.input,
    )?;
    persist_capture_cache(repo_root, &plan)?;
    Ok(PipelineCapturePreview { plan })
}

pub fn capture_pipeline_output(
    repo_root: impl AsRef<Path>,
    request: &PipelineCaptureRequest,
) -> Result<PipelineCaptureApplyResult, PipelineCaptureRefusal> {
    let repo_root = repo_root.as_ref();
    let plan = build_capture_plan(
        repo_root,
        &request.pipeline_selector,
        &request.stage_selector,
        &request.input,
    )?;
    apply_capture_plan(repo_root, &plan)
}

pub fn apply_pipeline_capture(
    repo_root: impl AsRef<Path>,
    capture_id: &str,
) -> Result<PipelineCaptureApplyResult, PipelineCaptureRefusal> {
    let repo_root = repo_root.as_ref();
    let cache_entry = load_capture_cache(repo_root, capture_id)?;
    apply_capture_plan(repo_root, &cache_entry.plan).inspect(|_| {
        let _ = delete_capture_cache(repo_root, capture_id);
    })
}

pub fn apply_cached_pipeline_capture(
    repo_root: impl AsRef<Path>,
    capture_id: &str,
) -> Result<PipelineCaptureApplyResult, PipelineCaptureRefusal> {
    apply_pipeline_capture(repo_root, capture_id)
}

pub fn load_pipeline_capture_cache_entry(
    repo_root: impl AsRef<Path>,
    capture_id: &str,
) -> Result<PipelineCaptureCacheEntry, PipelineCaptureRefusal> {
    load_capture_cache(repo_root.as_ref(), capture_id)
}

pub fn render_pipeline_capture_preview(preview: &PipelineCapturePreview) -> String {
    let mut out = String::new();
    out.push_str("OUTCOME: PREVIEW\n");
    out.push_str(&format!("PIPELINE: {}\n", preview.plan.target.pipeline_id));
    out.push_str(&format!("STAGE: {}\n", preview.plan.target.stage_id));
    out.push_str(&format!("CAPTURE ID: {}\n", preview.plan.capture_id));
    out.push_str(&format!(
        "ROUTE BASIS REVISION: {}\n",
        preview.plan.basis.state_revision
    ));
    out.push_str("WRITE PLAN:\n");
    render_write_plan(&mut out, &preview.plan);
    out.push_str("POST-CAPTURE STATE UPDATES:\n");
    render_state_updates(&mut out, &preview.plan.state_updates);
    out.push_str(&format!(
        "NEXT SAFE ACTION: run `handbook pipeline capture apply --capture-id {}`",
        preview.plan.capture_id
    ));
    out
}

pub fn render_pipeline_capture_apply_result(result: &PipelineCaptureApplyResult) -> String {
    let mut out = String::new();
    out.push_str("OUTCOME: CAPTURED\n");
    out.push_str(&format!("PIPELINE: {}\n", result.plan.target.pipeline_id));
    out.push_str(&format!("STAGE: {}\n", result.plan.target.stage_id));
    out.push_str("WRITTEN FILES:\n");
    if result.written_files.is_empty() {
        out.push_str("  <none>\n");
    } else {
        for path in &result.written_files {
            out.push_str(&format!("  - {path}\n"));
        }
    }
    out.push_str("STATE UPDATES:\n");
    render_state_updates(&mut out, &result.plan.state_updates);
    out.push_str("NEXT SAFE ACTION: ");
    out.push_str(
        result
            .plan
            .post_apply_next_safe_action
            .as_deref()
            .unwrap_or("<none>"),
    );
    out
}

pub fn render_pipeline_capture_refusal(
    refusal: &PipelineCaptureRefusal,
    requested_pipeline_id: Option<&str>,
    requested_stage_id: Option<&str>,
) -> String {
    let mut out = String::new();
    out.push_str("OUTCOME: REFUSED\n");
    out.push_str(&format!(
        "PIPELINE: {}\n",
        refusal
            .pipeline_id
            .as_deref()
            .or(requested_pipeline_id.map(str::trim))
            .unwrap_or("<unknown>")
    ));
    out.push_str(&format!(
        "STAGE: {}\n",
        refusal
            .stage_id
            .as_deref()
            .or(requested_stage_id.map(str::trim))
            .unwrap_or("<unknown>")
    ));
    out.push_str(&format!(
        "REASON: {}: {}\n",
        refusal.classification,
        refusal.summary.trim()
    ));
    out.push_str("NEXT SAFE ACTION: ");
    out.push_str(refusal.recovery.trim());
    out
}

fn render_write_plan(out: &mut String, plan: &PipelineCapturePlan) {
    if plan.artifact_writes.is_empty() && plan.repo_mirror_writes.is_empty() {
        out.push_str("  <none>\n");
        return;
    }

    for write in &plan.artifact_writes {
        out.push_str(&format!("  - artifact: {}\n", write.path));
    }
    for write in &plan.repo_mirror_writes {
        out.push_str(&format!("  - repo_file: {}\n", write.path));
    }
}

fn render_state_updates(out: &mut String, state_updates: &[PipelineCaptureStateUpdate]) {
    if state_updates.is_empty() {
        out.push_str("  <none>\n");
        return;
    }

    for update in state_updates {
        out.push_str(&format!("  - {} = {}\n", update.field_path, update.value));
    }
}

fn build_capture_plan(
    repo_root: &Path,
    pipeline_selector: &str,
    stage_selector: &str,
    input: &str,
) -> Result<PipelineCapturePlan, PipelineCaptureRefusal> {
    let pipeline = load_selected_pipeline_definition(repo_root, pipeline_selector).map_err(
        |err| match err {
            SelectedPipelineLoadError::Lookup(err) => PipelineCaptureRefusal {
                classification: PipelineCaptureRefusalClassification::UnsupportedTarget,
                summary: err.to_string(),
                pipeline_id: None,
                stage_id: None,
                recovery: "retry with the canonical pipeline id `pipeline.foundation_inputs`"
                    .to_string(),
            },
            SelectedPipelineLoadError::Catalog(err) => PipelineCaptureRefusal {
                classification: PipelineCaptureRefusalClassification::InvalidDefinition,
                summary: format!("failed to load selected pipeline definition: {err}"),
                pipeline_id: None,
                stage_id: None,
                recovery: "fix the pipeline/stage definitions and retry `pipeline capture`"
                    .to_string(),
            },
            SelectedPipelineLoadError::Load(err) => PipelineCaptureRefusal {
                classification: PipelineCaptureRefusalClassification::InvalidDefinition,
                summary: format!("failed to load selected pipeline definition: {err}"),
                pipeline_id: None,
                stage_id: None,
                recovery: "fix the pipeline/stage definitions and retry `pipeline capture`"
                    .to_string(),
            },
        },
    )?;

    let stage_id = match resolve_stage_selector(&pipeline, stage_selector) {
        Ok(stage_id) => stage_id,
        Err(summary) => {
            let registry =
                load_capture_target_registry(repo_root, Some(&pipeline.header.id), None)?;
            return Err(PipelineCaptureRefusal {
                classification: PipelineCaptureRefusalClassification::UnsupportedTarget,
                summary,
                pipeline_id: Some(pipeline.header.id.clone()),
                stage_id: Some(stage_selector.trim().to_string()),
                recovery: format!(
                    "retry with one of {}",
                    render_supported_capture_stage_list(&registry)
                ),
            });
        }
    };
    let registry =
        load_capture_target_registry(repo_root, Some(&pipeline.header.id), Some(&stage_id))?;
    validate_supported_capture_target(&registry, &pipeline.header.id, &stage_id)?;

    let supported_variables = supported_route_state_variables(&pipeline);
    let state = load_route_state_with_supported_variables(
        repo_root,
        &pipeline.header.id,
        &supported_variables,
    )
    .map_err(|err| classify_state_read_refusal(err, &pipeline.header.id, &stage_id))?;
    let route_basis = state
        .route_basis
        .clone()
        .ok_or_else(|| PipelineCaptureRefusal {
            classification: PipelineCaptureRefusalClassification::MissingRouteBasis,
            summary: "persisted route_basis is missing for the selected pipeline".to_string(),
            pipeline_id: Some(pipeline.header.id.clone()),
            stage_id: Some(stage_id.clone()),
            recovery: format!(
                "run `handbook pipeline resolve --id {}` and retry `handbook pipeline capture`",
                pipeline.header.id
            ),
        })?;
    let canonical_route_basis = rebuild_canonical_route_basis(repo_root, &pipeline, &state)
        .map_err(|reason| stale_basis_refusal(&pipeline.header.id, &stage_id, reason))?;
    check_route_basis_freshness(
        repo_root,
        &pipeline,
        &state,
        &route_basis,
        &canonical_route_basis,
        &stage_id,
    )?;
    let stage_definition = load_stage_compile_definition(repo_root, &pipeline, &stage_id)
        .map_err(|err| classify_compile_stage_load_refusal(err, &pipeline.header.id, &stage_id))?;
    let basis_stage = route_basis
        .route
        .iter()
        .find(|stage| stage.stage_id == stage_id)
        .ok_or_else(|| PipelineCaptureRefusal {
            classification: PipelineCaptureRefusalClassification::MalformedRouteBasis,
            summary: format!(
                "persisted route_basis is malformed: stage `{stage_id}` is missing from the route snapshot"
            ),
            pipeline_id: Some(pipeline.header.id.clone()),
            stage_id: Some(stage_id.clone()),
            recovery: format!(
                "run `handbook pipeline resolve --id {}` and retry `handbook pipeline capture`",
                pipeline.header.id
            ),
        })?;

    if basis_stage.status != RouteBasisStageStatus::Active {
        return Err(PipelineCaptureRefusal {
            classification: PipelineCaptureRefusalClassification::InactiveStage,
            summary: format!(
                "selected stage `{stage_id}` is not active in the persisted route_basis (current status: {}{})",
                render_route_basis_status(basis_stage.status),
                render_route_basis_reason_suffix(basis_stage.reason.as_ref())
            ),
            pipeline_id: Some(pipeline.header.id.clone()),
            stage_id: Some(stage_id.clone()),
            recovery: format!(
                "run `handbook pipeline resolve --id {}`, adjust route state if needed, and retry `handbook pipeline capture --id {} --stage {stage_id}`",
                pipeline.header.id, pipeline.header.id
            ),
        });
    }

    let (artifact_paths, repo_paths) =
        derive_capture_output_paths(&stage_definition, &route_basis, &state).map_err(|reason| {
            PipelineCaptureRefusal {
                classification: PipelineCaptureRefusalClassification::InvalidWriteTarget,
                summary: reason,
                pipeline_id: Some(pipeline.header.id.clone()),
                stage_id: Some(stage_id.clone()),
                recovery: "fix the declared output targets and retry `pipeline capture`"
                    .to_string(),
            }
        })?;

    let artifact_contents = if artifact_paths.len() == 1 {
        parse_single_file_capture_input(&stage_definition.id, input).map(|content| {
            vec![PipelineCaptureWriteIntent {
                path: artifact_paths[0].clone(),
                content,
            }]
        })
    } else {
        parse_multi_file_capture_input(input, &artifact_paths).map(|contents| {
            artifact_paths
                .iter()
                .map(|path| PipelineCaptureWriteIntent {
                    path: path.clone(),
                    content: contents
                        .get(path)
                        .cloned()
                        .expect("declared artifact path should be present"),
                })
                .collect()
        })
    }
    .map_err(|err| classify_capture_input_refusal(err, &pipeline.header.id, &stage_id))?;

    let repo_mirror_writes = build_repo_mirror_writes(&artifact_contents, &repo_paths).map_err(
        |summary| PipelineCaptureRefusal {
            classification: PipelineCaptureRefusalClassification::InvalidCaptureInput,
            summary,
            pipeline_id: Some(pipeline.header.id.clone()),
            stage_id: Some(stage_id.clone()),
            recovery:
                "emit the declared artifact outputs only and let `pipeline capture` derive repo-file mirrors"
                    .to_string(),
        },
    )?;
    validate_write_targets(
        repo_root,
        &artifact_contents,
        &repo_mirror_writes,
        &pipeline.header.id,
        &stage_id,
    )?;
    let state_updates = derive_state_updates(&artifact_contents);
    let post_apply_next_safe_action =
        build_post_apply_next_safe_action(&pipeline.header.id, &stage_definition, &state_updates);

    let mut plan = PipelineCapturePlan {
        target: PipelineCaptureTarget {
            pipeline_id: pipeline.header.id.clone(),
            stage_id: stage_definition.id.clone(),
            stage_file: stage_definition.stage.file.clone(),
            title: stage_definition.title.clone(),
        },
        basis: route_basis,
        artifact_writes: artifact_contents,
        repo_mirror_writes,
        state_updates,
        capture_id: String::new(),
        post_apply_next_safe_action,
    };
    plan.capture_id = compute_capture_id(&plan).map_err(|reason| PipelineCaptureRefusal {
        classification: PipelineCaptureRefusalClassification::InvalidState,
        summary: reason,
        pipeline_id: Some(plan.target.pipeline_id.clone()),
        stage_id: Some(plan.target.stage_id.clone()),
        recovery: "retry `pipeline capture` after the capture identity material is stable"
            .to_string(),
    })?;
    Ok(plan)
}

fn apply_capture_plan(
    repo_root: &Path,
    plan: &PipelineCapturePlan,
) -> Result<PipelineCaptureApplyResult, PipelineCaptureRefusal> {
    let supported_variables =
        supported_route_state_variables_for_plan(repo_root, &plan.target.pipeline_id)?;
    let state_path = route_state_path(repo_root, &plan.target.pipeline_id).map_err(|reason| {
        PipelineCaptureRefusal {
            classification: PipelineCaptureRefusalClassification::InvalidState,
            summary: format!(
                "invalid route state pipeline id `{}`: {reason}",
                plan.target.pipeline_id
            ),
            pipeline_id: Some(plan.target.pipeline_id.clone()),
            stage_id: Some(plan.target.stage_id.clone()),
            recovery: "fix the pipeline id and retry `pipeline capture`".to_string(),
        }
    })?;
    let _lock = acquire_advisory_lock(&state_path).map_err(|err| PipelineCaptureRefusal {
        classification: PipelineCaptureRefusalClassification::InvalidState,
        summary: format!("{err}"),
        pipeline_id: Some(plan.target.pipeline_id.clone()),
        stage_id: Some(plan.target.stage_id.clone()),
        recovery: "retry `pipeline capture` after the route state lock is available".to_string(),
    })?;

    let pipeline =
        load_selected_pipeline_definition(repo_root, &plan.target.pipeline_id).map_err(|err| {
            PipelineCaptureRefusal {
                classification: PipelineCaptureRefusalClassification::InvalidDefinition,
                summary: format!("failed to reload selected pipeline definition: {err}"),
                pipeline_id: Some(plan.target.pipeline_id.clone()),
                stage_id: Some(plan.target.stage_id.clone()),
                recovery: "fix the pipeline/stage definitions and retry `pipeline capture`"
                    .to_string(),
            }
        })?;
    let state = load_route_state_with_supported_variables(
        repo_root,
        &plan.target.pipeline_id,
        &supported_variables,
    )
    .map_err(|err| {
        classify_state_read_refusal(err, &plan.target.pipeline_id, &plan.target.stage_id)
    })?;
    let current_basis = state
        .route_basis
        .clone()
        .ok_or_else(|| PipelineCaptureRefusal {
            classification: PipelineCaptureRefusalClassification::MissingRouteBasis,
            summary: "persisted route_basis is missing for the selected pipeline".to_string(),
            pipeline_id: Some(plan.target.pipeline_id.clone()),
            stage_id: Some(plan.target.stage_id.clone()),
            recovery: format!(
                "run `handbook pipeline resolve --id {}` and retry `handbook pipeline capture`",
                plan.target.pipeline_id
            ),
        })?;
    let canonical_basis =
        rebuild_canonical_route_basis(repo_root, &pipeline, &state).map_err(|reason| {
            stale_basis_refusal(&plan.target.pipeline_id, &plan.target.stage_id, reason)
        })?;
    check_route_basis_freshness(
        repo_root,
        &pipeline,
        &state,
        &current_basis,
        &canonical_basis,
        &plan.target.stage_id,
    )?;
    if state.revision != plan.basis.state_revision {
        return Err(PipelineCaptureRefusal {
            classification: PipelineCaptureRefusalClassification::RevisionConflict,
            summary: format!(
                "route state revision {} does not match previewed capture revision {}",
                state.revision, plan.basis.state_revision
            ),
            pipeline_id: Some(plan.target.pipeline_id.clone()),
            stage_id: Some(plan.target.stage_id.clone()),
            recovery: format!(
                "run `handbook pipeline resolve --id {}` and rebuild the capture preview before retrying apply",
                plan.target.pipeline_id
            ),
        });
    }
    if let Some(reason) = route_basis_mismatch_reason(&plan.basis, &current_basis) {
        return Err(PipelineCaptureRefusal {
            classification: PipelineCaptureRefusalClassification::StaleRouteBasis,
            summary: format!(
                "cached capture plan no longer matches the persisted route_basis: {reason}"
            ),
            pipeline_id: Some(plan.target.pipeline_id.clone()),
            stage_id: Some(plan.target.stage_id.clone()),
            recovery: format!(
                "run `handbook pipeline resolve --id {}` and rebuild the capture preview before retrying apply",
                plan.target.pipeline_id
            ),
        });
    }
    let registry = load_capture_target_registry(
        repo_root,
        Some(&plan.target.pipeline_id),
        Some(&plan.target.stage_id),
    )?;
    validate_supported_capture_target(&registry, &plan.target.pipeline_id, &plan.target.stage_id)?;
    let stage_definition = load_stage_compile_definition(
        repo_root,
        &pipeline,
        &plan.target.stage_id,
    )
    .map_err(|err| {
        classify_compile_stage_load_refusal(err, &plan.target.pipeline_id, &plan.target.stage_id)
    })?;
    let canonical_plan = canonicalize_capture_plan_for_apply(
        repo_root,
        plan,
        &stage_definition,
        &current_basis,
        &state,
    )?;
    let stage_10_capture_provenance =
        build_stage_10_capture_provenance_for_apply(repo_root, &canonical_plan)?;

    let mut snapshots =
        snapshot_targets(repo_root, &canonical_plan).map_err(|summary| PipelineCaptureRefusal {
            classification: PipelineCaptureRefusalClassification::InvalidWriteTarget,
            summary,
            pipeline_id: Some(canonical_plan.target.pipeline_id.clone()),
            stage_id: Some(canonical_plan.target.stage_id.clone()),
            recovery: "fix the output targets and retry `pipeline capture`".to_string(),
        })?;
    let written_files =
        write_capture_targets(repo_root, &canonical_plan, &mut snapshots).map_err(|summary| {
            rollback_snapshots(repo_root, &snapshots);
            PipelineCaptureRefusal {
                classification: PipelineCaptureRefusalClassification::WriteFailure,
                summary,
                pipeline_id: Some(canonical_plan.target.pipeline_id.clone()),
                stage_id: Some(canonical_plan.target.stage_id.clone()),
                recovery: "fix the file-write failure and retry `pipeline capture`".to_string(),
            }
        })?;

    let persisted_state_revision = if canonical_plan.state_updates.is_empty() {
        None
    } else {
        let mut next_state = normalized_state_for_persistence(&state, repo_root);
        next_state.schema_version = ROUTE_STATE_SCHEMA_VERSION.to_string();
        next_state.revision = next_state.revision.saturating_add(1);
        for update in &canonical_plan.state_updates {
            apply_state_update(&mut next_state, update);
        }
        next_state.run.repo_root = Some(repo_root.display().to_string());
        for update in &canonical_plan.state_updates {
            next_state.audit.push(RouteStateAuditEntry {
                revision: next_state.revision,
                field_path: update.field_path.clone(),
                value: update.value.clone().into(),
            });
        }
        trim_audit_history(&mut next_state.audit);
        if let Err(err) = persist_route_state(&state_path, &next_state) {
            rollback_snapshots(repo_root, &snapshots);
            return Err(PipelineCaptureRefusal {
                classification: PipelineCaptureRefusalClassification::StatePersistenceFailure,
                summary: format!("failed to persist post-capture state: {err}"),
                pipeline_id: Some(canonical_plan.target.pipeline_id.clone()),
                stage_id: Some(canonical_plan.target.stage_id.clone()),
                recovery: "fix the route-state persistence failure and retry `pipeline capture`"
                    .to_string(),
            });
        }
        Some(next_state.revision)
    };
    if let Some(provenance) = stage_10_capture_provenance.as_ref() {
        if let Err(summary) =
            persist_stage_10_feature_spec_capture_provenance(repo_root, provenance)
        {
            rollback_snapshots(repo_root, &snapshots);
            return Err(PipelineCaptureRefusal {
                classification: PipelineCaptureRefusalClassification::StatePersistenceFailure,
                summary: format!("failed to persist stage-10 capture provenance: {summary}"),
                pipeline_id: Some(canonical_plan.target.pipeline_id.clone()),
                stage_id: Some(canonical_plan.target.stage_id.clone()),
                recovery:
                    "fix the stage-10 capture provenance persistence failure and retry `pipeline capture`"
                        .to_string(),
            });
        }
    }

    Ok(PipelineCaptureApplyResult {
        plan: canonical_plan,
        written_files,
        persisted_state_revision,
    })
}

fn build_stage_10_capture_provenance_for_apply(
    repo_root: &Path,
    plan: &PipelineCapturePlan,
) -> Result<
    Option<crate::stage_10_feature_spec_provenance::Stage10FeatureSpecCaptureProvenance>,
    PipelineCaptureRefusal,
> {
    if plan.target.stage_id != SUPPORTED_STAGE_FEATURE_SPEC_ID {
        return Ok(None);
    }

    let feature_spec_write = plan
        .artifact_writes
        .iter()
        .find(|write| write.path == FEATURE_SPEC_ARTIFACT_PATH)
        .ok_or_else(|| PipelineCaptureRefusal {
            classification: PipelineCaptureRefusalClassification::InvalidState,
            summary: format!(
                "stage `{}` capture is missing required artifact write `{FEATURE_SPEC_ARTIFACT_PATH}`",
                plan.target.stage_id
            ),
            pipeline_id: Some(plan.target.pipeline_id.clone()),
            stage_id: Some(plan.target.stage_id.clone()),
            recovery:
                "repair the declared stage outputs and retry `pipeline capture`".to_string(),
        })?;

    let compile_result =
        compile_pipeline_stage(repo_root, &plan.target.pipeline_id, &plan.target.stage_id)
            .map_err(classify_stage_10_provenance_compile_refusal)?;
    let feature_spec_sha256 = sha256_hex(feature_spec_write.content.as_bytes());
    let provenance = build_stage_10_feature_spec_capture_provenance(
        repo_root,
        &compile_result,
        &feature_spec_sha256,
    )
    .map_err(|reason| PipelineCaptureRefusal {
        classification: PipelineCaptureRefusalClassification::InvalidState,
        summary: format!("failed to build stage-10 capture provenance: {reason}"),
        pipeline_id: Some(plan.target.pipeline_id.clone()),
        stage_id: Some(plan.target.stage_id.clone()),
        recovery: "repair the stage-10 compile provenance inputs and retry `pipeline capture`"
            .to_string(),
    })?;
    Ok(Some(provenance))
}

fn classify_stage_10_provenance_compile_refusal(
    refusal: PipelineCompileRefusal,
) -> PipelineCaptureRefusal {
    let classification = match refusal.classification {
        crate::pipeline_compile::PipelineCompileRefusalClassification::UnsupportedTarget => {
            PipelineCaptureRefusalClassification::UnsupportedTarget
        }
        crate::pipeline_compile::PipelineCompileRefusalClassification::InvalidDefinition => {
            PipelineCaptureRefusalClassification::InvalidDefinition
        }
        crate::pipeline_compile::PipelineCompileRefusalClassification::MissingRequiredInput
        | crate::pipeline_compile::PipelineCompileRefusalClassification::EmptyRequiredInput
        | crate::pipeline_compile::PipelineCompileRefusalClassification::InvalidState
        | crate::pipeline_compile::PipelineCompileRefusalClassification::MissingRouteBasis
        | crate::pipeline_compile::PipelineCompileRefusalClassification::MalformedRouteBasis
        | crate::pipeline_compile::PipelineCompileRefusalClassification::StaleRouteBasis
        | crate::pipeline_compile::PipelineCompileRefusalClassification::InactiveStage => {
            PipelineCaptureRefusalClassification::InvalidState
        }
    };
    PipelineCaptureRefusal {
        classification,
        summary: format!(
            "failed to rebuild stage-10 compile provenance during capture apply: {}",
            refusal.summary
        ),
        pipeline_id: refusal.pipeline_id,
        stage_id: refusal.stage_id,
        recovery: refusal.recovery,
    }
}

fn canonicalize_capture_plan_for_apply(
    repo_root: &Path,
    plan: &PipelineCapturePlan,
    stage_definition: &CompileStageDefinition,
    basis: &RouteBasis,
    state: &RouteState,
) -> Result<PipelineCapturePlan, PipelineCaptureRefusal> {
    let (artifact_paths, repo_paths) = derive_capture_output_paths(stage_definition, basis, state)
        .map_err(|reason| PipelineCaptureRefusal {
            classification: PipelineCaptureRefusalClassification::InvalidWriteTarget,
            summary: reason,
            pipeline_id: Some(plan.target.pipeline_id.clone()),
            stage_id: Some(plan.target.stage_id.clone()),
            recovery: "fix the declared output targets and retry `pipeline capture`".to_string(),
        })?;
    let artifact_writes =
        canonicalize_cached_artifact_writes(plan, &artifact_paths).map_err(|summary| {
            PipelineCaptureRefusal {
                classification: PipelineCaptureRefusalClassification::TamperedCaptureCache,
                summary,
                pipeline_id: Some(plan.target.pipeline_id.clone()),
                stage_id: Some(plan.target.stage_id.clone()),
                recovery:
                    "re-run `handbook pipeline capture --preview` to rebuild the cached capture"
                        .to_string(),
            }
        })?;
    let repo_mirror_writes =
        build_repo_mirror_writes(&artifact_writes, &repo_paths).map_err(|summary| {
            PipelineCaptureRefusal {
                classification: PipelineCaptureRefusalClassification::InvalidWriteTarget,
                summary,
                pipeline_id: Some(plan.target.pipeline_id.clone()),
                stage_id: Some(plan.target.stage_id.clone()),
                recovery: "fix the declared output targets and retry `pipeline capture`"
                    .to_string(),
            }
        })?;
    validate_write_targets(
        repo_root,
        &artifact_writes,
        &repo_mirror_writes,
        &plan.target.pipeline_id,
        &plan.target.stage_id,
    )?;
    let state_updates = derive_state_updates(&artifact_writes);
    let post_apply_next_safe_action = build_post_apply_next_safe_action(
        &plan.target.pipeline_id,
        stage_definition,
        &state_updates,
    );

    if artifact_writes != plan.artifact_writes {
        return Err(tampered_capture_cache_refusal(
            plan,
            "cached preview artifact writes do not match the canonical stage outputs",
        ));
    }
    if repo_mirror_writes != plan.repo_mirror_writes {
        return Err(tampered_capture_cache_refusal(
            plan,
            "cached preview repo mirror writes do not match the canonical derivation",
        ));
    }
    if state_updates != plan.state_updates {
        return Err(tampered_capture_cache_refusal(
            plan,
            "cached preview state updates do not match the canonical derivation",
        ));
    }
    if post_apply_next_safe_action != plan.post_apply_next_safe_action {
        return Err(tampered_capture_cache_refusal(
            plan,
            "cached preview next safe action does not match the canonical derivation",
        ));
    }

    Ok(PipelineCapturePlan {
        target: PipelineCaptureTarget {
            pipeline_id: plan.target.pipeline_id.clone(),
            stage_id: stage_definition.id.clone(),
            stage_file: stage_definition.stage.file.clone(),
            title: stage_definition.title.clone(),
        },
        basis: basis.clone(),
        artifact_writes,
        repo_mirror_writes,
        state_updates,
        capture_id: plan.capture_id.clone(),
        post_apply_next_safe_action,
    })
}

fn persist_capture_cache(
    repo_root: &Path,
    plan: &PipelineCapturePlan,
) -> Result<(), PipelineCaptureRefusal> {
    let cache_path = capture_cache_path(repo_root, &plan.capture_id).map_err(|reason| {
        PipelineCaptureRefusal {
            classification: PipelineCaptureRefusalClassification::CacheFailure,
            summary: reason,
            pipeline_id: Some(plan.target.pipeline_id.clone()),
            stage_id: Some(plan.target.stage_id.clone()),
            recovery: "fix the capture cache path and retry preview".to_string(),
        }
    })?;
    if let Some(parent) = cache_path.parent() {
        fs::create_dir_all(parent).map_err(|err| PipelineCaptureRefusal {
            classification: PipelineCaptureRefusalClassification::CacheFailure,
            summary: format!(
                "failed to create capture cache directory {}: {err}",
                parent.display()
            ),
            pipeline_id: Some(plan.target.pipeline_id.clone()),
            stage_id: Some(plan.target.stage_id.clone()),
            recovery: "fix the cache directory and retry preview".to_string(),
        })?;
    }
    let cache_entry = PipelineCaptureCacheEntry {
        schema_version: CAPTURE_CACHE_SCHEMA_VERSION.to_string(),
        capture_id: plan.capture_id.clone(),
        plan: plan.clone(),
    };
    let bytes = serde_yaml_bw::to_string(&cache_entry)
        .map_err(|err| PipelineCaptureRefusal {
            classification: PipelineCaptureRefusalClassification::CacheFailure,
            summary: format!("failed to serialize capture cache entry: {err}"),
            pipeline_id: Some(plan.target.pipeline_id.clone()),
            stage_id: Some(plan.target.stage_id.clone()),
            recovery: "retry preview after the cache serialization failure is fixed".to_string(),
        })?
        .into_bytes();
    atomic_write_bytes(&cache_path, &bytes).map_err(|err| PipelineCaptureRefusal {
        classification: PipelineCaptureRefusalClassification::CacheFailure,
        summary: err,
        pipeline_id: Some(plan.target.pipeline_id.clone()),
        stage_id: Some(plan.target.stage_id.clone()),
        recovery: "fix the cache write failure and retry preview".to_string(),
    })?;
    Ok(())
}

fn load_capture_cache(
    repo_root: &Path,
    capture_id: &str,
) -> Result<PipelineCaptureCacheEntry, PipelineCaptureRefusal> {
    validate_capture_id(capture_id).map_err(|reason| PipelineCaptureRefusal {
        classification: PipelineCaptureRefusalClassification::MissingCaptureId,
        summary: reason,
        pipeline_id: None,
        stage_id: None,
        recovery: "retry with the capture id printed by `pipeline capture --preview`".to_string(),
    })?;
    let cache_relative_path =
        capture_cache_repo_relative_path(repo_root, capture_id).map_err(|reason| {
            PipelineCaptureRefusal {
                classification: PipelineCaptureRefusalClassification::MissingCaptureId,
                summary: reason,
                pipeline_id: None,
                stage_id: None,
                recovery: "retry with the capture id printed by `pipeline capture --preview`"
                    .to_string(),
            }
        })?;
    let cache_path = resolve_repo_relative_write_path(repo_root, cache_relative_path.as_str())
        .map_err(|source| {
            classify_capture_cache_path_failure(cache_relative_path.as_str(), source)
        })?;
    let contents = read_string_no_follow_path(&cache_path)
        .map_err(|source| classify_capture_cache_read_failure(&cache_path, capture_id, source))?;
    let cache_entry: PipelineCaptureCacheEntry =
        serde_yaml_bw::from_str(&contents).map_err(|err| PipelineCaptureRefusal {
            classification: PipelineCaptureRefusalClassification::TamperedCaptureCache,
            summary: format!("cached preview `{capture_id}` is malformed: {err}"),
            pipeline_id: None,
            stage_id: None,
            recovery: "re-run `handbook pipeline capture --preview` to rebuild the cached capture"
                .to_string(),
        })?;
    if cache_entry.schema_version != CAPTURE_CACHE_SCHEMA_VERSION {
        return Err(PipelineCaptureRefusal {
            classification: PipelineCaptureRefusalClassification::TamperedCaptureCache,
            summary: format!(
                "cached preview `{capture_id}` schema_version `{}` does not match expected `{}`",
                cache_entry.schema_version, CAPTURE_CACHE_SCHEMA_VERSION
            ),
            pipeline_id: Some(cache_entry.plan.target.pipeline_id.clone()),
            stage_id: Some(cache_entry.plan.target.stage_id.clone()),
            recovery: "re-run `handbook pipeline capture --preview` to rebuild the cached capture"
                .to_string(),
        });
    }
    let expected_capture_id =
        compute_capture_id(&cache_entry.plan).map_err(|reason| PipelineCaptureRefusal {
            classification: PipelineCaptureRefusalClassification::TamperedCaptureCache,
            summary: reason,
            pipeline_id: Some(cache_entry.plan.target.pipeline_id.clone()),
            stage_id: Some(cache_entry.plan.target.stage_id.clone()),
            recovery: "re-run `handbook pipeline capture --preview` to rebuild the cached capture"
                .to_string(),
        })?;
    if expected_capture_id != capture_id || cache_entry.capture_id != capture_id {
        return Err(PipelineCaptureRefusal {
            classification: PipelineCaptureRefusalClassification::TamperedCaptureCache,
            summary: format!(
                "cached preview `{capture_id}` no longer matches its deterministic capture_id"
            ),
            pipeline_id: Some(cache_entry.plan.target.pipeline_id.clone()),
            stage_id: Some(cache_entry.plan.target.stage_id.clone()),
            recovery: "re-run `handbook pipeline capture --preview` to rebuild the cached capture"
                .to_string(),
        });
    }
    Ok(cache_entry)
}

fn classify_capture_cache_path_failure(
    cache_relative_path: &str,
    source: RepoRelativeWritePathError,
) -> PipelineCaptureRefusal {
    let (classification, recovery) = match &source {
        RepoRelativeWritePathError::ParentNotDirectory(_)
        | RepoRelativeWritePathError::NotRegularFile(_)
        | RepoRelativeWritePathError::SymlinkNotAllowed(_) => (
            PipelineCaptureRefusalClassification::TamperedCaptureCache,
            "re-run `handbook pipeline capture --preview` to rebuild the cached capture".to_string(),
        ),
        RepoRelativeWritePathError::InvalidPath(_)
        | RepoRelativeWritePathError::ReadFailure { .. } => (
            PipelineCaptureRefusalClassification::CacheFailure,
            "fix permissions or repair the cache path, or delete/rebuild the cached preview before retrying apply"
                .to_string(),
        ),
    };

    PipelineCaptureRefusal {
        classification,
        summary: format_cache_path_error(cache_relative_path, source),
        pipeline_id: None,
        stage_id: None,
        recovery,
    }
}

fn classify_capture_cache_read_failure(
    cache_path: &Path,
    capture_id: &str,
    source: std::io::Error,
) -> PipelineCaptureRefusal {
    let not_found_refusal = || PipelineCaptureRefusal {
        classification: PipelineCaptureRefusalClassification::MissingCaptureId,
        summary: format!("cached preview `{capture_id}` was not found"),
        pipeline_id: None,
        stage_id: None,
        recovery: "re-run `handbook pipeline capture --preview` to generate a fresh capture id"
            .to_string(),
    };

    if source.kind() == std::io::ErrorKind::NotFound {
        return not_found_refusal();
    }

    match fs::symlink_metadata(cache_path) {
        Ok(metadata) if !metadata.is_file() || metadata.file_type().is_symlink() => {
            PipelineCaptureRefusal {
                classification: PipelineCaptureRefusalClassification::TamperedCaptureCache,
                summary: format!(
                    "cached preview `{capture_id}` must be a regular non-symlink file"
                ),
                pipeline_id: None,
                stage_id: None,
                recovery:
                    "re-run `handbook pipeline capture --preview` to rebuild the cached capture"
                        .to_string(),
            }
        }
        Ok(_) => PipelineCaptureRefusal {
            classification: PipelineCaptureRefusalClassification::CacheFailure,
            summary: format!(
                "failed to read cached preview `{capture_id}` at {}: {source}",
                cache_path.display()
            ),
            pipeline_id: None,
            stage_id: None,
            recovery:
                "fix permissions or delete/rebuild the cached preview, then retry apply"
                    .to_string(),
        },
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => not_found_refusal(),
        Err(err) => PipelineCaptureRefusal {
            classification: PipelineCaptureRefusalClassification::CacheFailure,
            summary: format!(
                "failed to read cached preview `{capture_id}` at {}: {source}; metadata inspection also failed: {err}",
                cache_path.display()
            ),
            pipeline_id: None,
            stage_id: None,
            recovery:
                "fix permissions or delete/rebuild the cached preview, then retry apply"
                    .to_string(),
        },
    }
}

fn delete_capture_cache(repo_root: &Path, capture_id: &str) -> Result<(), String> {
    let cache_path = capture_cache_path(repo_root, capture_id)?;
    match fs::remove_file(&cache_path) {
        Ok(()) => Ok(()),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(err) => Err(format!(
            "failed to delete cached preview {}: {err}",
            cache_path.display()
        )),
    }
}

fn capture_cache_path(repo_root: &Path, capture_id: &str) -> Result<PathBuf, String> {
    let relative_path = capture_cache_repo_relative_path(repo_root, capture_id)?;
    resolve_repo_relative_write_path(repo_root, relative_path.as_str())
        .map_err(|err| format_cache_path_error(relative_path.as_str(), err))
}

fn capture_cache_repo_relative_path(
    repo_root: &Path,
    capture_id: &str,
) -> Result<NormalizedRepoRelativePath, String> {
    validate_capture_id(capture_id)?;
    Ok(RepoLayoutRoot::new(repo_root)
        .capture_provenance()
        .capture_cache_relative_path(capture_id))
}

fn validate_capture_id(capture_id: &str) -> Result<(), String> {
    let capture_id = capture_id.trim();
    if capture_id.is_empty() {
        return Err("capture id must not be empty".to_string());
    }
    if capture_id.len() != 64 || !capture_id.chars().all(|ch| ch.is_ascii_hexdigit()) {
        return Err(format!(
            "capture id `{capture_id}` is invalid; expected a 64-character lowercase hex identifier"
        ));
    }
    Ok(())
}

fn compute_capture_id(plan: &PipelineCapturePlan) -> Result<String, String> {
    let mut identity_plan = plan.clone();
    identity_plan.capture_id.clear();
    let serialized = serde_yaml_bw::to_string(&identity_plan)
        .map_err(|err| format!("failed to serialize capture identity material: {err}"))?;
    let mut hasher = Sha256::new();
    hasher.update(serialized.as_bytes());
    Ok(format!("{:x}", hasher.finalize()))
}

fn format_cache_path_error(path: &str, err: RepoRelativeWritePathError) -> String {
    match err {
        RepoRelativeWritePathError::InvalidPath(reason) => {
            format!("capture cache path `{path}` is invalid: {reason}")
        }
        RepoRelativeWritePathError::ParentNotDirectory(found) => {
            format!(
                "capture cache path `{path}` cannot be written because parent {} is not a directory",
                found.display()
            )
        }
        RepoRelativeWritePathError::NotRegularFile(found) => {
            format!(
                "capture cache path `{path}` cannot be written because {} is not a regular file target",
                found.display()
            )
        }
        RepoRelativeWritePathError::SymlinkNotAllowed(found) => {
            format!(
                "capture cache path `{path}` cannot be written through symlink {}",
                found.display()
            )
        }
        RepoRelativeWritePathError::ReadFailure {
            path: found,
            source,
        } => {
            format!(
                "failed to inspect capture cache path `{path}` at {}: {source}",
                found.display()
            )
        }
    }
}

fn derive_capture_output_paths(
    stage_definition: &CompileStageDefinition,
    basis: &RouteBasis,
    state: &RouteState,
) -> Result<(Vec<String>, Vec<String>), String> {
    let output_variables = build_output_variables(basis, state);
    let artifact_paths = stage_definition
        .outputs
        .artifacts
        .iter()
        .map(|output| {
            normalize_output_relative_path(&substitute_variables(&output.path, &output_variables))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let repo_paths = stage_definition
        .outputs
        .repo_files
        .iter()
        .map(|output| {
            normalize_output_relative_path(&substitute_variables(&output.path, &output_variables))
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok((artifact_paths, repo_paths))
}

fn canonicalize_cached_artifact_writes(
    plan: &PipelineCapturePlan,
    artifact_paths: &[String],
) -> Result<Vec<PipelineCaptureWriteIntent>, String> {
    if artifact_paths.len() == 1 {
        let write = plan
            .artifact_writes
            .first()
            .ok_or_else(|| "cached preview is missing the declared artifact output".to_string())?;
        if plan.artifact_writes.len() != 1 {
            return Err(
                "cached preview emitted an unexpected number of artifact outputs".to_string(),
            );
        }
        if write.path != artifact_paths[0] {
            return Err(format!(
                "cached preview artifact `{}` does not match the declared output `{}`",
                write.path, artifact_paths[0]
            ));
        }
        let content = normalize_nonempty_capture_content(&write.content).ok_or_else(|| {
            format!(
                "cached preview artifact `{}` must contain a non-empty canonical body",
                write.path
            )
        })?;
        return Ok(vec![PipelineCaptureWriteIntent {
            path: write.path.clone(),
            content,
        }]);
    }

    let declared = artifact_paths.iter().cloned().collect::<BTreeSet<_>>();
    let mut writes_by_path = BTreeMap::new();
    for write in &plan.artifact_writes {
        if !declared.contains(&write.path) {
            return Err(format!(
                "cached preview emitted undeclared artifact `{}` for the selected stage",
                write.path
            ));
        }
        let content = normalize_nonempty_capture_content(&write.content).ok_or_else(|| {
            format!(
                "cached preview artifact `{}` must contain a non-empty canonical body",
                write.path
            )
        })?;
        if writes_by_path
            .insert(
                write.path.clone(),
                PipelineCaptureWriteIntent {
                    path: write.path.clone(),
                    content,
                },
            )
            .is_some()
        {
            return Err(format!(
                "cached preview emitted artifact `{}` more than once",
                write.path
            ));
        }
    }

    artifact_paths
        .iter()
        .map(|path| {
            writes_by_path
                .get(path)
                .cloned()
                .ok_or_else(|| format!("cached preview is missing declared artifact `{path}`"))
        })
        .collect()
}

fn parse_single_file_capture_input(
    stage_id: &str,
    input: &str,
) -> Result<String, CaptureInputParseError> {
    let normalized = normalize_capture_line_endings(input);
    if normalized
        .lines()
        .any(|line| line.starts_with("--- FILE: ") && line.ends_with(" ---"))
    {
        return Err(CaptureInputParseError::InvalidSingleFileWrapper);
    }
    let content = normalize_nonempty_capture_content(&normalized)
        .ok_or(CaptureInputParseError::EmptySingleFileBody)?;
    validate_single_file_capture_content(stage_id, &content)?;
    Ok(content)
}

fn parse_multi_file_capture_input(
    input: &str,
    declared_artifacts: &[String],
) -> Result<BTreeMap<String, String>, CaptureInputParseError> {
    let normalized = input.replace("\r\n", "\n");
    let declared = declared_artifacts
        .iter()
        .cloned()
        .collect::<std::collections::BTreeSet<_>>();
    let mut blocks = BTreeMap::new();
    let mut current_path: Option<String> = None;
    let mut current_content = String::new();
    let mut saw_header = false;

    for raw_line in normalized.split('\n') {
        if let Some(path) = parse_file_header_line(raw_line) {
            saw_header = true;
            if !declared.contains(path) {
                return Err(CaptureInputParseError::UndeclaredBlock(path.to_string()));
            }
            if let Some(previous_path) = current_path.take() {
                if blocks
                    .insert(
                        previous_path.clone(),
                        normalize_declared_block_content(&previous_path, &current_content)?,
                    )
                    .is_some()
                {
                    return Err(CaptureInputParseError::DuplicateBlock(previous_path));
                }
                current_content.clear();
            }
            current_path = Some(path.to_string());
            continue;
        }

        if current_path.is_none() {
            if raw_line.trim().is_empty() {
                continue;
            }
            if !saw_header {
                return Err(CaptureInputParseError::InvalidMultifilePrefix);
            }
        } else {
            current_content.push_str(raw_line);
            current_content.push('\n');
        }
    }

    if let Some(previous_path) = current_path.take() {
        if blocks
            .insert(
                previous_path.clone(),
                normalize_declared_block_content(&previous_path, &current_content)?,
            )
            .is_some()
        {
            return Err(CaptureInputParseError::DuplicateBlock(previous_path));
        }
    }

    for declared_path in declared_artifacts {
        if !blocks.contains_key(declared_path) {
            return Err(CaptureInputParseError::MissingDeclaredBlock(
                declared_path.clone(),
            ));
        }
    }

    Ok(blocks)
}

fn parse_file_header_line(line: &str) -> Option<&str> {
    if !line.starts_with("--- FILE: ") || !line.ends_with(" ---") {
        return None;
    }
    let path = &line["--- FILE: ".len()..line.len() - " ---".len()];
    if path.trim().is_empty() {
        return None;
    }
    Some(path)
}

fn normalize_capture_line_endings(input: &str) -> String {
    input.replace("\r\n", "\n")
}

fn normalize_nonempty_capture_content(input: &str) -> Option<String> {
    let normalized = normalize_capture_line_endings(input);
    let trimmed = normalized.trim_end_matches('\n');
    if trimmed.is_empty() {
        None
    } else {
        Some(format!("{trimmed}\n"))
    }
}

fn validate_single_file_capture_content(
    stage_id: &str,
    content: &str,
) -> Result<(), CaptureInputParseError> {
    if stage_id == SUPPORTED_STAGE_FEATURE_SPEC_ID {
        if is_raw_stage_10_compile_payload(content) {
            return Err(CaptureInputParseError::RawStage10CompilePayload);
        }
        if !is_completed_stage_10_feature_spec_body(content) {
            return Err(CaptureInputParseError::InvalidStage10FeatureSpecBody);
        }
    }
    Ok(())
}

fn is_raw_stage_10_compile_payload(content: &str) -> bool {
    let mut lines = content.lines();
    if lines.next() != Some("# stage.10_feature_spec - Feature Specification") {
        return false;
    }

    [
        "## Run Variables",
        "## Selected Runner",
        "## Selected Profile",
        "## Outputs",
    ]
    .into_iter()
    .all(|marker| content.contains(marker))
}

fn is_completed_stage_10_feature_spec_body(content: &str) -> bool {
    let Some(first_line) = content.lines().next() else {
        return false;
    };
    if !first_line.starts_with("# ") || !first_line.contains("Feature Specification") {
        return false;
    }
    if REQUIRED_STAGE_10_FEATURE_SPEC_HEADINGS
        .iter()
        .any(|heading| !content.contains(heading))
    {
        return false;
    }
    if content.contains("{{") || content.contains("}}") || content.contains('\u{2026}') {
        return false;
    }
    if !content
        .lines()
        .any(|line| line.trim_start().starts_with("- G") && line.contains(':'))
    {
        return false;
    }
    if !content
        .lines()
        .any(|line| line.trim_start().starts_with("- AC-") && line.contains(':'))
    {
        return false;
    }
    if !content
        .lines()
        .any(|line| line.trim_start().starts_with("- Alt "))
    {
        return false;
    }
    for prefix in ["- Security", "- Performance", "- Reliability"] {
        if !content.lines().any(|line| {
            let trimmed = line.trim_start();
            trimmed.starts_with(prefix) && trimmed.contains(':')
        }) {
            return false;
        }
    }
    content.lines().any(|line| {
        let trimmed = line.trim_start();
        trimmed.starts_with("- R") && trimmed.contains("-> AC-")
    })
}

fn normalize_declared_block_content(
    path: &str,
    input: &str,
) -> Result<String, CaptureInputParseError> {
    normalize_nonempty_capture_content(input)
        .ok_or_else(|| CaptureInputParseError::EmptyDeclaredBlock(path.to_string()))
}

fn build_repo_mirror_writes(
    artifact_writes: &[PipelineCaptureWriteIntent],
    repo_paths: &[String],
) -> Result<Vec<PipelineCaptureWriteIntent>, String> {
    let mut writes = Vec::new();
    for repo_path in repo_paths {
        let basename = Path::new(repo_path)
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| format!("repo-file target `{repo_path}` has no basename"))?;
        let matches = artifact_writes
            .iter()
            .filter(|artifact| {
                Path::new(&artifact.path)
                    .file_name()
                    .and_then(|name| name.to_str())
                    == Some(basename)
            })
            .collect::<Vec<_>>();
        if matches.is_empty() {
            return Err(format!(
                "required repo-file mirror `{repo_path}` could not be derived from the declared artifact outputs"
            ));
        }
        if matches.len() > 1 {
            return Err(format!(
                "required repo-file mirror `{repo_path}` matches multiple artifact outputs by basename"
            ));
        }
        writes.push(PipelineCaptureWriteIntent {
            path: repo_path.clone(),
            content: matches[0].content.clone(),
        });
    }
    Ok(writes)
}

fn validate_write_targets(
    repo_root: &Path,
    artifact_writes: &[PipelineCaptureWriteIntent],
    repo_mirror_writes: &[PipelineCaptureWriteIntent],
    pipeline_id: &str,
    stage_id: &str,
) -> Result<(), PipelineCaptureRefusal> {
    for write in artifact_writes.iter().chain(repo_mirror_writes.iter()) {
        resolve_repo_relative_write_path(repo_root, &write.path).map_err(|err| {
            PipelineCaptureRefusal {
                classification: PipelineCaptureRefusalClassification::InvalidWriteTarget,
                summary: format_write_target_error(&write.path, err),
                pipeline_id: Some(pipeline_id.to_string()),
                stage_id: Some(stage_id.to_string()),
                recovery: "fix the declared output targets and retry `pipeline capture`"
                    .to_string(),
            }
        })?;
    }
    Ok(())
}

fn derive_state_updates(
    artifact_writes: &[PipelineCaptureWriteIntent],
) -> Vec<PipelineCaptureStateUpdate> {
    let mut updates = Vec::new();
    for write in artifact_writes {
        match write.path.as_str() {
            "artifacts/charter/CHARTER.md" => {
                updates.push(PipelineCaptureStateUpdate {
                    field_path: "refs.charter_ref".to_string(),
                    value: PipelineCaptureStateValue::String(write.path.clone()),
                });
                updates.push(PipelineCaptureStateUpdate {
                    field_path: "routing.charter_gaps_detected".to_string(),
                    value: PipelineCaptureStateValue::Bool(
                        CAPTURE_UNKNOWN_MARKERS
                            .iter()
                            .any(|marker| write.content.contains(marker)),
                    ),
                });
            }
            "artifacts/project_context/PROJECT_CONTEXT.md" => {
                updates.push(PipelineCaptureStateUpdate {
                    field_path: "refs.project_context_ref".to_string(),
                    value: PipelineCaptureStateValue::String(write.path.clone()),
                });
            }
            _ => {}
        }
    }
    dedupe_state_updates(updates)
}

fn build_post_apply_next_safe_action(
    pipeline_id: &str,
    stage_definition: &CompileStageDefinition,
    state_updates: &[PipelineCaptureStateUpdate],
) -> Option<String> {
    let unresolved_variables = unresolved_manual_set_variables(stage_definition, state_updates);
    if !unresolved_variables.is_empty() {
        let mut action_steps = unresolved_variables
            .into_iter()
            .map(|variable| {
                format!(
                    "run `handbook pipeline state set --id {pipeline_id} --var {variable}=<true|false>`"
                )
            })
            .collect::<Vec<_>>();
        action_steps.push(format!(
            "run `handbook pipeline resolve --id {pipeline_id}` before the next compile or capture"
        ));
        return Some(action_steps.join(", then "));
    }

    if state_updates.is_empty() {
        None
    } else {
        Some(format!(
            "run `handbook pipeline resolve --id {pipeline_id}` before the next compile or capture"
        ))
    }
}

fn unresolved_manual_set_variables<'a>(
    stage_definition: &'a CompileStageDefinition,
    state_updates: &[PipelineCaptureStateUpdate],
) -> Vec<&'a str> {
    let deterministic_routing_updates = state_updates
        .iter()
        .filter_map(|update| update.field_path.strip_prefix("routing."))
        .collect::<BTreeSet<_>>();

    stage_definition
        .stage
        .sets
        .as_ref()
        .map(|sets| {
            sets.iter()
                .map(String::as_str)
                .filter(|variable| !deterministic_routing_updates.contains(variable))
                .collect()
        })
        .unwrap_or_default()
}

fn dedupe_state_updates(
    updates: Vec<PipelineCaptureStateUpdate>,
) -> Vec<PipelineCaptureStateUpdate> {
    let mut deduped = BTreeMap::new();
    for update in updates {
        deduped.insert(update.field_path.clone(), update);
    }
    deduped.into_values().collect()
}

fn apply_state_update(state: &mut RouteState, update: &PipelineCaptureStateUpdate) {
    match (update.field_path.as_str(), &update.value) {
        ("refs.charter_ref", PipelineCaptureStateValue::String(value)) => {
            state.refs.charter_ref = Some(value.clone());
        }
        ("refs.project_context_ref", PipelineCaptureStateValue::String(value)) => {
            state.refs.project_context_ref = Some(value.clone());
        }
        ("routing.charter_gaps_detected", PipelineCaptureStateValue::Bool(value)) => {
            state
                .routing
                .insert("charter_gaps_detected".to_string(), *value);
        }
        _ => {}
    }
}

fn trim_audit_history(audit: &mut Vec<RouteStateAuditEntry>) {
    if audit.len() <= ROUTE_STATE_AUDIT_LIMIT {
        return;
    }
    let overflow = audit.len() - ROUTE_STATE_AUDIT_LIMIT;
    audit.drain(0..overflow);
}

fn supported_route_state_variables_for_plan(
    repo_root: &Path,
    pipeline_id: &str,
) -> Result<std::collections::BTreeSet<String>, PipelineCaptureRefusal> {
    let pipeline = load_selected_pipeline_definition(repo_root, pipeline_id).map_err(|err| {
        PipelineCaptureRefusal {
            classification: PipelineCaptureRefusalClassification::InvalidDefinition,
            summary: format!("failed to load selected pipeline definition: {err}"),
            pipeline_id: Some(pipeline_id.to_string()),
            stage_id: None,
            recovery: "fix the pipeline definition and retry `pipeline capture`".to_string(),
        }
    })?;
    Ok(supported_route_state_variables(&pipeline))
}

fn snapshot_targets(
    repo_root: &Path,
    plan: &PipelineCapturePlan,
) -> Result<Vec<SnapshotEntry>, String> {
    let mut snapshots = Vec::new();
    for write in plan
        .artifact_writes
        .iter()
        .chain(plan.repo_mirror_writes.iter())
    {
        let absolute_path = resolve_repo_relative_write_path(repo_root, &write.path)
            .map_err(|err| format_write_target_error(&write.path, err))?;
        let prior_bytes = match fs::symlink_metadata(&absolute_path) {
            Ok(metadata) => {
                if metadata.file_type().is_symlink() {
                    return Err(format!(
                        "write target `{}` resolves through a symlink",
                        write.path
                    ));
                }
                if metadata.is_dir() {
                    return Err(format!("write target `{}` is a directory", write.path));
                }
                Some(read_bytes_no_follow_path(&absolute_path).map_err(|err| {
                    format!(
                        "failed to snapshot write target `{}` at {}: {err}",
                        write.path,
                        absolute_path.display()
                    )
                })?)
            }
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => None,
            Err(err) => {
                return Err(format!(
                    "failed to inspect write target `{}` at {}: {err}",
                    write.path,
                    absolute_path.display()
                ))
            }
        };
        snapshots.push(SnapshotEntry {
            repo_relative_path: write.path.clone(),
            prior_bytes,
        });
    }
    Ok(snapshots)
}

fn write_capture_targets(
    repo_root: &Path,
    plan: &PipelineCapturePlan,
    snapshots: &mut [SnapshotEntry],
) -> Result<Vec<String>, String> {
    let mut written = Vec::new();
    let writes = plan
        .artifact_writes
        .iter()
        .chain(plan.repo_mirror_writes.iter())
        .collect::<Vec<_>>();
    for (index, write) in writes.iter().enumerate() {
        let snapshot = snapshots
            .get(index)
            .ok_or_else(|| "internal capture snapshot mismatch".to_string())?;
        let bytes = write.content.as_bytes().to_vec();
        write_repo_relative_bytes(repo_root, &snapshot.repo_relative_path, &bytes).map_err(
            |err| {
                format!(
                    "failed to commit `{}`: {}",
                    write.path,
                    format_repo_relative_mutation_error(&snapshot.repo_relative_path, err)
                )
            },
        )?;
        written.push(write.path.clone());
    }
    Ok(written)
}

fn rollback_snapshots(repo_root: &Path, snapshots: &[SnapshotEntry]) {
    for snapshot in snapshots.iter().rev() {
        match &snapshot.prior_bytes {
            Some(bytes) => {
                let _ = write_repo_relative_bytes(repo_root, &snapshot.repo_relative_path, bytes);
            }
            None => {
                let _ = delete_repo_relative_file(repo_root, &snapshot.repo_relative_path);
            }
        }
    }
}

fn atomic_write_bytes(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let parent = path
        .parent()
        .ok_or_else(|| format!("write target {} has no parent directory", path.display()))?;
    fs::create_dir_all(parent)
        .map_err(|err| format!("failed to create {}: {err}", parent.display()))?;

    let temp_path = crate::route_state::temp_route_state_path(path);
    let result = (|| -> Result<(), String> {
        let mut file =
            crate::route_state::open_new_temp_file(&temp_path).map_err(|err| format!("{err}"))?;
        file.write_all(bytes)
            .map_err(|err| format!("failed to write {}: {err}", temp_path.display()))?;
        file.sync_all()
            .map_err(|err| format!("failed to fsync {}: {err}", temp_path.display()))?;
        drop(file);
        fs::rename(&temp_path, path).map_err(|err| {
            format!(
                "failed to rename {} -> {}: {err}",
                temp_path.display(),
                path.display()
            )
        })?;
        crate::route_state::sync_parent_dir(path).map_err(|err| format!("{err}"))?;
        Ok(())
    })();
    if result.is_err() {
        let _ = fs::remove_file(&temp_path);
    }
    result
}

fn classify_capture_input_refusal(
    err: CaptureInputParseError,
    pipeline_id: &str,
    stage_id: &str,
) -> PipelineCaptureRefusal {
    match err {
        CaptureInputParseError::InvalidSingleFileWrapper => PipelineCaptureRefusal {
            classification: PipelineCaptureRefusalClassification::InvalidCaptureInput,
            summary: "single-file capture stages must receive plain body content and must not use `--- FILE:` wrappers".to_string(),
            pipeline_id: Some(pipeline_id.to_string()),
            stage_id: Some(stage_id.to_string()),
            recovery: "paste only the stage body and retry `pipeline capture`".to_string(),
        },
        CaptureInputParseError::EmptySingleFileBody => PipelineCaptureRefusal {
            classification: PipelineCaptureRefusalClassification::InvalidCaptureInput,
            summary: "single-file capture stages must receive a non-empty body".to_string(),
            pipeline_id: Some(pipeline_id.to_string()),
            stage_id: Some(stage_id.to_string()),
            recovery: "paste the generated stage body and retry `pipeline capture`".to_string(),
        },
        CaptureInputParseError::RawStage10CompilePayload => PipelineCaptureRefusal {
            classification: PipelineCaptureRefusalClassification::InvalidCaptureInput,
            summary: "stage.10_feature_spec capture must receive a completed FEATURE_SPEC.md body, not raw `pipeline compile` payload".to_string(),
            pipeline_id: Some(pipeline_id.to_string()),
            stage_id: Some(stage_id.to_string()),
            recovery: "run the stage-10 compile payload through an external operator or model runner, then retry `pipeline capture` with the completed `FEATURE_SPEC.md`".to_string(),
        },
        CaptureInputParseError::InvalidStage10FeatureSpecBody => PipelineCaptureRefusal {
            classification: PipelineCaptureRefusalClassification::InvalidCaptureInput,
            summary: "stage.10_feature_spec capture must receive a completed FEATURE_SPEC.md body that satisfies the shipped feature-spec contract".to_string(),
            pipeline_id: Some(pipeline_id.to_string()),
            stage_id: Some(stage_id.to_string()),
            recovery: "provide only the completed `FEATURE_SPEC.md` body, with the shipped headings and filled content, then retry `pipeline capture`".to_string(),
        },
        CaptureInputParseError::InvalidMultifilePrefix => PipelineCaptureRefusal {
            classification: PipelineCaptureRefusalClassification::InvalidCaptureInput,
            summary: "multi-file capture stages must start with declared `--- FILE: <path> ---` blocks".to_string(),
            pipeline_id: Some(pipeline_id.to_string()),
            stage_id: Some(stage_id.to_string()),
            recovery: "emit the declared artifact FILE blocks exactly once and retry `pipeline capture`".to_string(),
        },
        CaptureInputParseError::EmptyDeclaredBlock(path) => PipelineCaptureRefusal {
            classification: PipelineCaptureRefusalClassification::InvalidCaptureInput,
            summary: format!("declared artifact block `{path}` must contain a non-empty body"),
            pipeline_id: Some(pipeline_id.to_string()),
            stage_id: Some(stage_id.to_string()),
            recovery:
                "emit every declared artifact FILE block with a non-empty body and retry `pipeline capture`"
                    .to_string(),
        },
        CaptureInputParseError::DuplicateBlock(path) => PipelineCaptureRefusal {
            classification: PipelineCaptureRefusalClassification::InvalidCaptureInput,
            summary: format!("declared artifact block `{path}` was emitted more than once"),
            pipeline_id: Some(pipeline_id.to_string()),
            stage_id: Some(stage_id.to_string()),
            recovery: "emit each declared artifact FILE block exactly once and retry `pipeline capture`".to_string(),
        },
        CaptureInputParseError::MissingDeclaredBlock(path) => PipelineCaptureRefusal {
            classification: PipelineCaptureRefusalClassification::InvalidCaptureInput,
            summary: format!("declared artifact block `{path}` is missing from the capture input"),
            pipeline_id: Some(pipeline_id.to_string()),
            stage_id: Some(stage_id.to_string()),
            recovery:
                "emit every declared artifact FILE block exactly once and retry `pipeline capture`"
                    .to_string(),
        },
        CaptureInputParseError::UndeclaredBlock(path) => PipelineCaptureRefusal {
            classification: PipelineCaptureRefusalClassification::InvalidCaptureInput,
            summary: format!(
                "capture input emitted undeclared artifact block `{path}` for the selected stage"
            ),
            pipeline_id: Some(pipeline_id.to_string()),
            stage_id: Some(stage_id.to_string()),
            recovery:
                "emit only the declared artifact FILE blocks and retry `pipeline capture`"
                    .to_string(),
        },
    }
}

fn tampered_capture_cache_refusal(
    plan: &PipelineCapturePlan,
    summary: impl Into<String>,
) -> PipelineCaptureRefusal {
    PipelineCaptureRefusal {
        classification: PipelineCaptureRefusalClassification::TamperedCaptureCache,
        summary: summary.into(),
        pipeline_id: Some(plan.target.pipeline_id.clone()),
        stage_id: Some(plan.target.stage_id.clone()),
        recovery: "re-run `handbook pipeline capture --preview` to rebuild the cached capture"
            .to_string(),
    }
}

fn classify_compile_stage_load_refusal(
    err: crate::pipeline::CompileStageLoadError,
    pipeline_id: &str,
    stage_id: &str,
) -> PipelineCaptureRefusal {
    PipelineCaptureRefusal {
        classification: PipelineCaptureRefusalClassification::InvalidDefinition,
        summary: format!("failed to load selected stage definition: {err}"),
        pipeline_id: Some(pipeline_id.to_string()),
        stage_id: Some(stage_id.to_string()),
        recovery: "fix the stage definition and retry `pipeline capture`".to_string(),
    }
}

fn classify_state_read_refusal(
    err: RouteStateReadError,
    pipeline_id: &str,
    stage_id: &str,
) -> PipelineCaptureRefusal {
    match err {
        RouteStateReadError::MalformedState { reason, .. } if reason.contains("route_basis") => {
            PipelineCaptureRefusal {
                classification: PipelineCaptureRefusalClassification::MalformedRouteBasis,
                summary: format!("persisted route_basis is malformed: {reason}"),
                pipeline_id: Some(pipeline_id.to_string()),
                stage_id: Some(stage_id.to_string()),
                recovery: format!(
                    "run `handbook pipeline resolve --id {pipeline_id}` and retry `handbook pipeline capture --id {pipeline_id} --stage {stage_id}`"
                ),
            }
        }
        other => PipelineCaptureRefusal {
            classification: PipelineCaptureRefusalClassification::InvalidState,
            summary: other.to_string(),
            pipeline_id: Some(pipeline_id.to_string()),
            stage_id: Some(stage_id.to_string()),
            recovery: "fix the route state and retry `pipeline capture`".to_string(),
        },
    }
}

fn load_capture_target_registry(
    repo_root: &Path,
    pipeline_id: Option<&str>,
    stage_id: Option<&str>,
) -> Result<SupportedTargetRegistry, PipelineCaptureRefusal> {
    SupportedTargetRegistry::load(repo_root).map_err(|err| PipelineCaptureRefusal {
        classification: PipelineCaptureRefusalClassification::InvalidDefinition,
        summary: format!("failed to load supported target registry: {err}"),
        pipeline_id: pipeline_id.map(str::to_string),
        stage_id: stage_id.map(str::to_string),
        recovery: "fix the pipeline/stage definitions and retry `pipeline capture`".to_string(),
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SupportedCaptureTargetHelp {
    pipeline_id: String,
    default_stage_id: String,
    rendered_stage_list: String,
}

fn supported_capture_target_help(registry: &SupportedTargetRegistry) -> SupportedCaptureTargetHelp {
    let pipeline_id = registry
        .pipelines()
        .next()
        .expect("supported target registry must include one capture pipeline")
        .id
        .clone();
    let supported_stage_ids = registry
        .stages()
        .filter(|stage| registry.supports_capture_target(&pipeline_id, &stage.id))
        .map(|stage| stage.id.clone())
        .collect::<Vec<_>>();
    let default_stage_id = supported_stage_ids
        .first()
        .cloned()
        .expect("supported target registry must include at least one capture stage");
    let rendered_stage_list = supported_stage_ids
        .iter()
        .map(|stage_id| format!("`{stage_id}`"))
        .collect::<Vec<_>>()
        .join(", ");

    SupportedCaptureTargetHelp {
        pipeline_id,
        default_stage_id,
        rendered_stage_list,
    }
}

fn validate_supported_capture_target(
    registry: &SupportedTargetRegistry,
    pipeline_id: &str,
    stage_id: &str,
) -> Result<(), PipelineCaptureRefusal> {
    let supported_target = supported_capture_target_help(registry);

    if pipeline_id != supported_target.pipeline_id {
        return Err(PipelineCaptureRefusal {
            classification: PipelineCaptureRefusalClassification::UnsupportedTarget,
            summary: format!(
                "`pipeline capture` currently supports only pipeline `{}`",
                supported_target.pipeline_id
            ),
            pipeline_id: Some(pipeline_id.to_string()),
            stage_id: Some(stage_id.to_string()),
            recovery: format!(
                "retry with `pipeline capture --id {} --stage {}`",
                supported_target.pipeline_id, supported_target.default_stage_id
            ),
        });
    }
    if !registry.supports_capture_target(pipeline_id, stage_id) {
        return Err(PipelineCaptureRefusal {
            classification: PipelineCaptureRefusalClassification::UnsupportedTarget,
            summary: format!(
                "`pipeline capture` currently supports only stages {} for pipeline `{}`",
                supported_target.rendered_stage_list, supported_target.pipeline_id
            ),
            pipeline_id: Some(pipeline_id.to_string()),
            stage_id: Some(stage_id.to_string()),
            recovery: format!(
                "retry with `pipeline capture --id {} --stage {}` or another supported capture stage from {}",
                supported_target.pipeline_id,
                supported_target.default_stage_id,
                supported_target.rendered_stage_list
            ),
        });
    }
    Ok(())
}

fn render_supported_capture_stage_list(registry: &SupportedTargetRegistry) -> String {
    supported_capture_target_help(registry).rendered_stage_list
}

fn resolve_stage_selector(pipeline: &PipelineDefinition, selector: &str) -> Result<String, String> {
    let selector = selector.trim();
    if selector.is_empty() {
        return Err("stage selector must not be empty".to_string());
    }
    if selector_is_path_like(selector) {
        return Err(format!(
            "unsupported stage selector `{selector}`: raw file paths are evidence only; use a canonical stage id"
        ));
    }
    if pipeline
        .declared_stages()
        .iter()
        .any(|stage| stage.id == selector)
    {
        return Ok(selector.to_string());
    }
    let matches = pipeline
        .declared_stages()
        .iter()
        .filter(|stage| stage.id.strip_prefix("stage.") == Some(selector))
        .map(|stage| stage.id.clone())
        .collect::<Vec<_>>();
    if matches.len() > 1 {
        return Err(format!(
            "ambiguous stage selector `{selector}` matched multiple canonical ids: {}",
            matches.join(", ")
        ));
    }
    matches
        .into_iter()
        .next()
        .ok_or_else(|| format!("unknown stage selector `{selector}` for the selected pipeline"))
}

fn selector_is_path_like(selector: &str) -> bool {
    let path = Path::new(selector);
    path.is_absolute()
        || path
            .components()
            .any(|component| !matches!(component, Component::Normal(_)))
        || selector.contains('/')
}

fn check_route_basis_freshness(
    repo_root: &Path,
    pipeline: &PipelineDefinition,
    state: &RouteState,
    basis: &RouteBasis,
    canonical_basis: &RouteBasis,
    stage_id: &str,
) -> Result<(), PipelineCaptureRefusal> {
    if state.revision != basis.state_revision {
        return Err(stale_basis_refusal(
            &pipeline.header.id,
            stage_id,
            format!(
                "route state revision {} does not match persisted route_basis revision {}",
                state.revision, basis.state_revision
            ),
        ));
    }
    let effective_run = effective_route_basis_run(repo_root, pipeline, state);
    if state.routing != basis.routing
        || state.refs != basis.refs
        || effective_run != normalize_route_basis_run(&basis.run)
    {
        return Err(stale_basis_refusal(
            &pipeline.header.id,
            stage_id,
            "route_state routing/refs/run no longer match the persisted route_basis".to_string(),
        ));
    }
    if let Some(reason) = route_basis_mismatch_reason(basis, canonical_basis) {
        return Err(stale_basis_refusal(&pipeline.header.id, stage_id, reason));
    }
    Ok(())
}

fn stale_basis_refusal(
    pipeline_id: &str,
    stage_id: &str,
    summary: impl Into<String>,
) -> PipelineCaptureRefusal {
    PipelineCaptureRefusal {
        classification: PipelineCaptureRefusalClassification::StaleRouteBasis,
        summary: summary.into(),
        pipeline_id: Some(pipeline_id.to_string()),
        stage_id: Some(stage_id.to_string()),
        recovery: format!(
            "run `handbook pipeline resolve --id {pipeline_id}` and retry `handbook pipeline capture --id {pipeline_id} --stage {stage_id}`"
        ),
    }
}

fn build_output_variables(basis: &RouteBasis, state: &RouteState) -> BTreeMap<String, String> {
    let mut values = BTreeMap::new();
    for (name, value) in &basis.routing {
        values.insert(name.clone(), value.to_string());
    }
    if let Some(value) = &basis.refs.charter_ref {
        values.insert("charter_ref".to_string(), value.clone());
    }
    if let Some(value) = &basis.refs.project_context_ref {
        values.insert("project_context_ref".to_string(), value.clone());
    }
    values.insert("runner".to_string(), basis.runner.id.clone());
    values.insert("profile".to_string(), basis.profile.id.clone());
    values.insert("repo_root".to_string(), ".".to_string());
    if let Some(value) = &state.run.repo_root {
        values.insert("repo_root_absolute".to_string(), value.clone());
    }
    values
}

fn substitute_variables(input: &str, values: &BTreeMap<String, String>) -> String {
    let mut out = String::new();
    let mut rest = input;
    while let Some(start) = rest.find("${") {
        out.push_str(&rest[..start]);
        let after = &rest[start + 2..];
        let Some(end) = after.find('}') else {
            out.push_str(&rest[start..]);
            return out;
        };
        let key = &after[..end];
        match values.get(key).filter(|value| !value.is_empty()) {
            Some(value) => out.push_str(value),
            None => {
                out.push_str("${");
                out.push_str(key);
                out.push('}');
            }
        }
        rest = &after[end + 1..];
    }
    out.push_str(rest);
    out
}

fn normalize_output_relative_path(path: &str) -> Result<String, String> {
    let path = path.trim();
    if path.is_empty() {
        return Err("output path must not be empty".to_string());
    }
    let mut path = path.to_string();
    if let Some(stripped) = path.strip_prefix("./") {
        path = stripped.to_string();
    }
    if let Some(stripped) = path.strip_prefix("${repo_root}/") {
        path = stripped.to_string();
    }
    let validated = validate_repo_relative_path(&path)?;
    let normalized = validated
        .components()
        .filter_map(|component| match component {
            Component::Normal(part) => Some(part.to_string_lossy().to_string()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("/");
    if normalized.is_empty() {
        return Err("output path normalized to an empty repo-relative path".to_string());
    }
    Ok(normalized)
}

fn format_write_target_error(path: &str, err: RepoRelativeWritePathError) -> String {
    match err {
        RepoRelativeWritePathError::InvalidPath(reason) => {
            format!("write target `{path}` is invalid: {reason}")
        }
        RepoRelativeWritePathError::ParentNotDirectory(found) => {
            format!(
                "write target `{path}` cannot be written because {} is not a directory",
                found.display()
            )
        }
        RepoRelativeWritePathError::NotRegularFile(found) => {
            format!(
                "write target `{path}` cannot be written because {} is not a regular file target",
                found.display()
            )
        }
        RepoRelativeWritePathError::SymlinkNotAllowed(found) => {
            format!(
                "write target `{path}` cannot be written through symlink {}",
                found.display()
            )
        }
        RepoRelativeWritePathError::ReadFailure {
            path: found,
            source,
        } => {
            format!(
                "failed to inspect write target `{path}` at {}: {source}",
                found.display()
            )
        }
    }
}

fn format_repo_relative_mutation_error(path: &str, err: RepoRelativeMutationError) -> String {
    match err {
        RepoRelativeMutationError::InvalidPath(reason) => {
            format!("write target `{path}` is invalid: {reason}")
        }
        RepoRelativeMutationError::ParentNotDirectory(found) => {
            format!(
                "write target `{path}` cannot be written because {} is not a directory",
                found.display()
            )
        }
        RepoRelativeMutationError::NotRegularFile(found) => {
            format!(
                "write target `{path}` cannot be written because {} is not a regular file target",
                found.display()
            )
        }
        RepoRelativeMutationError::SymlinkNotAllowed(found) => {
            format!(
                "write target `{path}` cannot be written through symlink {}",
                found.display()
            )
        }
        RepoRelativeMutationError::ReadFailure {
            path: found,
            source,
        }
        | RepoRelativeMutationError::WriteFailure {
            path: found,
            source,
        } => {
            format!(
                "failed to mutate write target `{path}` at {}: {source}",
                found.display()
            )
        }
    }
}

fn render_route_basis_status(status: RouteBasisStageStatus) -> &'static str {
    match status {
        RouteBasisStageStatus::Active => "active",
        RouteBasisStageStatus::Skipped => "skipped",
        RouteBasisStageStatus::Blocked => "blocked",
        RouteBasisStageStatus::Next => "next",
    }
}

fn render_route_basis_reason_suffix(
    reason: Option<&crate::route_state::RouteBasisStageReason>,
) -> String {
    match reason {
        Some(crate::route_state::RouteBasisStageReason::SkippedActivationFalse {
            unsatisfied_variables,
            ..
        }) => format!(
            "; reason: activation evaluated false for variables: {}",
            unsatisfied_variables.join(", ")
        ),
        Some(crate::route_state::RouteBasisStageReason::NextMissingRouteVariables {
            missing_variables,
            ..
        }) => format!(
            "; reason: missing route variables: {}",
            missing_variables.join(", ")
        ),
        Some(crate::route_state::RouteBasisStageReason::BlockedByUnresolvedStage {
            upstream_stage_id,
            upstream_status,
        }) => format!(
            "; reason: blocked by unresolved stage {} ({})",
            upstream_stage_id,
            render_route_basis_status(*upstream_status)
        ),
        None => String::new(),
    }
}

impl From<PipelineCaptureStateValue> for RouteStateValue {
    fn from(value: PipelineCaptureStateValue) -> Self {
        match value {
            PipelineCaptureStateValue::Bool(value) => RouteStateValue::Bool(value),
            PipelineCaptureStateValue::String(value) => RouteStateValue::String(value),
        }
    }
}

impl fmt::Display for PipelineCaptureStateValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PipelineCaptureStateValue::Bool(value) => write!(f, "{value}"),
            PipelineCaptureStateValue::String(value) => write!(f, "{value}"),
        }
    }
}

impl fmt::Display for PipelineCaptureRefusalClassification {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            Self::UnsupportedTarget => "unsupported_target",
            Self::InvalidDefinition => "invalid_definition",
            Self::InvalidState => "invalid_state",
            Self::MissingRouteBasis => "missing_route_basis",
            Self::MalformedRouteBasis => "malformed_route_basis",
            Self::StaleRouteBasis => "stale_route_basis",
            Self::InactiveStage => "inactive_stage",
            Self::InvalidCaptureInput => "invalid_capture_input",
            Self::InvalidWriteTarget => "invalid_write_target",
            Self::MissingCaptureId => "missing_capture_id",
            Self::TamperedCaptureCache => "tampered_capture_cache",
            Self::RevisionConflict => "revision_conflict",
            Self::WriteFailure => "write_failure",
            Self::StatePersistenceFailure => "state_persistence_failure",
            Self::CacheFailure => "cache_failure",
        };
        write!(f, "{label}")
    }
}

#[cfg(test)]
mod tests {
    use super::{
        build_post_apply_next_safe_action, build_repo_mirror_writes, capture_cache_path,
        rollback_snapshots, snapshot_targets, write_capture_targets, PipelineCapturePlan,
        PipelineCaptureStateUpdate, PipelineCaptureStateValue, PipelineCaptureTarget,
        PipelineCaptureWriteIntent,
    };
    use crate::pipeline::{
        CompileStageDefinition, CompileStageGating, CompileStageInputs, CompileStageOutputs,
        PipelineStage,
    };
    use crate::route_state::{RouteBasis, RouteBasisProfilePack, RouteBasisRunner};
    use std::collections::BTreeMap;
    use std::fs;
    use std::path::{Path, PathBuf};

    fn stage_definition_with_sets(sets: Option<Vec<&str>>) -> CompileStageDefinition {
        CompileStageDefinition {
            stage: PipelineStage {
                id: "stage.test".to_string(),
                file: "core/stages/test.md".to_string(),
                sets: sets.map(|values| values.into_iter().map(str::to_string).collect()),
                activation: None,
            },
            source_path: PathBuf::from("core/stages/test.md"),
            kind: "stage".to_string(),
            id: "stage.test".to_string(),
            version: "0.1.0".to_string(),
            title: "Test Stage".to_string(),
            description: "Test stage".to_string(),
            work_level: None,
            includes: Vec::new(),
            inputs: CompileStageInputs::default(),
            outputs: CompileStageOutputs::default(),
            gating: CompileStageGating::default(),
            tags: Vec::new(),
            body: None,
        }
    }

    fn state_update(
        field_path: &str,
        value: PipelineCaptureStateValue,
    ) -> PipelineCaptureStateUpdate {
        PipelineCaptureStateUpdate {
            field_path: field_path.to_string(),
            value,
        }
    }

    fn test_plan(writes: Vec<PipelineCaptureWriteIntent>) -> PipelineCapturePlan {
        PipelineCapturePlan {
            target: PipelineCaptureTarget {
                pipeline_id: "pipeline.foundation_inputs".to_string(),
                stage_id: "stage.test".to_string(),
                stage_file: "core/stages/test.md".to_string(),
                title: "Test Stage".to_string(),
            },
            basis: RouteBasis {
                schema_version: "route_basis.v1".to_string(),
                pipeline_id: "pipeline.foundation_inputs".to_string(),
                pipeline_file: "core/pipelines/foundation_inputs.yaml".to_string(),
                pipeline_file_sha256: "pipeline".to_string(),
                state_revision: 1,
                routing: BTreeMap::new(),
                refs: Default::default(),
                run: Default::default(),
                route: Vec::new(),
                runner: RouteBasisRunner {
                    id: "runner".to_string(),
                    file: "runner.md".to_string(),
                    file_sha256: "runner".to_string(),
                },
                profile: RouteBasisProfilePack {
                    id: "default".to_string(),
                    profile_yaml_sha256: "profile".to_string(),
                    commands_yaml_sha256: "commands".to_string(),
                    conventions_md_sha256: "conventions".to_string(),
                },
            },
            artifact_writes: writes,
            repo_mirror_writes: Vec::new(),
            state_updates: Vec::new(),
            capture_id: "capture".to_string(),
            post_apply_next_safe_action: None,
        }
    }

    fn write_intent(path: &str, content: &str) -> PipelineCaptureWriteIntent {
        PipelineCaptureWriteIntent {
            path: path.to_string(),
            content: content.to_string(),
        }
    }

    fn read_file(path: &Path) -> String {
        fs::read_to_string(path).expect("read file")
    }

    #[test]
    fn next_safe_action_chains_state_set_then_resolve_when_sets_are_unresolved() {
        let stage_definition = stage_definition_with_sets(Some(vec!["needs_project_context"]));
        let updates = vec![state_update(
            "routing.charter_gaps_detected",
            PipelineCaptureStateValue::Bool(false),
        )];

        let action = build_post_apply_next_safe_action(
            "pipeline.foundation_inputs",
            &stage_definition,
            &updates,
        );

        assert_eq!(
            action.as_deref(),
            Some(
                "run `handbook pipeline state set --id pipeline.foundation_inputs --var needs_project_context=<true|false>`, then run `handbook pipeline resolve --id pipeline.foundation_inputs` before the next compile or capture"
            )
        );
    }

    #[test]
    fn next_safe_action_requires_resolve_when_only_automatic_state_updates_exist() {
        let stage_definition = stage_definition_with_sets(None);
        let updates = vec![state_update(
            "refs.charter_ref",
            PipelineCaptureStateValue::String("artifacts/charter/CHARTER.md".to_string()),
        )];

        let action = build_post_apply_next_safe_action(
            "pipeline.foundation_inputs",
            &stage_definition,
            &updates,
        );

        assert_eq!(
            action.as_deref(),
            Some(
                "run `handbook pipeline resolve --id pipeline.foundation_inputs` before the next compile or capture"
            )
        );
    }

    #[test]
    fn next_safe_action_still_requires_manual_follow_up_when_route_state_already_has_value() {
        let stage_definition = stage_definition_with_sets(Some(vec!["needs_project_context"]));

        let action =
            build_post_apply_next_safe_action("pipeline.foundation_inputs", &stage_definition, &[]);

        assert_eq!(
            action.as_deref(),
            Some(
                "run `handbook pipeline state set --id pipeline.foundation_inputs --var needs_project_context=<true|false>`, then run `handbook pipeline resolve --id pipeline.foundation_inputs` before the next compile or capture"
            )
        );
    }

    #[test]
    fn next_safe_action_chains_multiple_manual_set_commands_in_declared_order() {
        let stage_definition =
            stage_definition_with_sets(Some(vec!["needs_project_context", "needs_repo_scan"]));

        let action =
            build_post_apply_next_safe_action("pipeline.foundation_inputs", &stage_definition, &[]);

        assert_eq!(
            action.as_deref(),
            Some(
                "run `handbook pipeline state set --id pipeline.foundation_inputs --var needs_project_context=<true|false>`, then run `handbook pipeline state set --id pipeline.foundation_inputs --var needs_repo_scan=<true|false>`, then run `handbook pipeline resolve --id pipeline.foundation_inputs` before the next compile or capture"
            )
        );
    }

    #[test]
    fn next_safe_action_excludes_set_variables_with_deterministic_routing_updates() {
        let stage_definition =
            stage_definition_with_sets(Some(vec!["needs_project_context", "needs_repo_scan"]));
        let updates = vec![state_update(
            "routing.needs_project_context",
            PipelineCaptureStateValue::Bool(true),
        )];

        let action = build_post_apply_next_safe_action(
            "pipeline.foundation_inputs",
            &stage_definition,
            &updates,
        );

        assert_eq!(
            action.as_deref(),
            Some(
                "run `handbook pipeline state set --id pipeline.foundation_inputs --var needs_repo_scan=<true|false>`, then run `handbook pipeline resolve --id pipeline.foundation_inputs` before the next compile or capture"
            )
        );
    }

    #[test]
    fn next_safe_action_is_none_when_no_follow_up_is_required() {
        let stage_definition = stage_definition_with_sets(None);

        let action =
            build_post_apply_next_safe_action("pipeline.foundation_inputs", &stage_definition, &[]);

        assert_eq!(action, None);
    }

    #[test]
    fn capture_cache_path_resolves_within_repo_root() {
        let repo_root = tempfile::tempdir().expect("tempdir");
        let capture_id = "1111111111111111111111111111111111111111111111111111111111111111";

        let path = capture_cache_path(repo_root.path(), capture_id).expect("cache path");

        assert_eq!(
            path,
            repo_root
                .path()
                .join(".handbook")
                .join("state")
                .join("pipeline")
                .join("capture")
                .join(format!("{capture_id}.yaml"))
        );
    }

    #[cfg(unix)]
    #[test]
    fn capture_cache_path_rejects_symlinked_parent_chain() {
        let repo_root = tempfile::tempdir().expect("tempdir");
        let external_root = tempfile::tempdir().expect("external tempdir");
        let system_root = repo_root.path().join(".handbook");
        let target_root = external_root.path().join("redirected-system");
        fs::create_dir_all(&target_root).expect("external target root");
        std::os::unix::fs::symlink(&target_root, &system_root).expect("symlink .handbook");

        let capture_id = "1111111111111111111111111111111111111111111111111111111111111111";
        let err = capture_cache_path(repo_root.path(), capture_id).expect_err("symlink refusal");

        assert!(
            err.contains("cannot be written through symlink"),
            "expected symlink refusal, got: {err}"
        );
    }

    #[cfg(unix)]
    #[test]
    fn write_capture_targets_rejects_parent_swap_after_snapshot_without_writing_outside_repo() {
        let repo_root = tempfile::tempdir().expect("tempdir");
        let external_root = tempfile::tempdir().expect("external tempdir");
        let real_parent = repo_root.path().join("mirror");
        let swapped_parent = repo_root.path().join("mirror-live");
        let target_path = real_parent.join("target.md");
        let escaped_path = external_root.path().join("target.md");
        fs::create_dir_all(&real_parent).expect("create parent");
        fs::write(&target_path, "before\n").expect("seed target");

        let plan = test_plan(vec![write_intent("mirror/target.md", "after\n")]);
        let mut snapshots = snapshot_targets(repo_root.path(), &plan).expect("snapshot");

        fs::rename(&real_parent, &swapped_parent).expect("move real parent");
        std::os::unix::fs::symlink(external_root.path(), repo_root.path().join("mirror"))
            .expect("swap parent with symlink");

        let err =
            write_capture_targets(repo_root.path(), &plan, &mut snapshots).expect_err("refusal");

        assert!(
            err.contains("cannot be written through symlink"),
            "expected symlink refusal, got: {err}"
        );
        assert!(
            !escaped_path.exists(),
            "commit should not create or overwrite files outside the repo"
        );
        assert_eq!(read_file(&swapped_parent.join("target.md")), "before\n");
    }

    #[cfg(unix)]
    #[test]
    fn rollback_snapshots_refuses_symlink_swaps_for_restore_and_delete_paths() {
        let repo_root = tempfile::tempdir().expect("tempdir");
        let escaped_restore_root = tempfile::tempdir().expect("restore tempdir");
        let escaped_delete_root = tempfile::tempdir().expect("delete tempdir");
        let stable_parent = repo_root.path().join("stable");
        let failing_parent = repo_root.path().join("failing");
        fs::create_dir_all(&stable_parent).expect("create stable parent");
        fs::create_dir_all(&failing_parent).expect("create failing parent");
        fs::write(stable_parent.join("new.md"), "stable outside\n").expect("seed escaped delete");
        fs::write(failing_parent.join("existing.md"), "before\n").expect("seed existing target");

        let plan = test_plan(vec![
            write_intent("created/new.md", "created\n"),
            write_intent("failing/existing.md", "after\n"),
        ]);
        let mut snapshots = snapshot_targets(repo_root.path(), &plan).expect("snapshot");

        let real_failing_parent = repo_root.path().join("failing-real");
        fs::rename(&failing_parent, &real_failing_parent).expect("move failing parent");
        fs::write(
            escaped_restore_root.path().join("existing.md"),
            "outside restore\n",
        )
        .expect("seed escaped restore target");
        std::os::unix::fs::symlink(
            escaped_restore_root.path(),
            repo_root.path().join("failing"),
        )
        .expect("swap failing parent with symlink");

        let err =
            write_capture_targets(repo_root.path(), &plan, &mut snapshots).expect_err("refusal");
        assert!(
            err.contains("cannot be written through symlink"),
            "expected symlink refusal, got: {err}"
        );

        let created_parent = repo_root.path().join("created");
        assert_eq!(read_file(&created_parent.join("new.md")), "created\n");

        let real_created_parent = repo_root.path().join("created-real");
        fs::rename(&created_parent, &real_created_parent).expect("move created parent");
        fs::write(
            escaped_delete_root.path().join("new.md"),
            "outside delete\n",
        )
        .expect("seed escaped delete target");
        std::os::unix::fs::symlink(escaped_delete_root.path(), repo_root.path().join("created"))
            .expect("swap created parent with symlink");

        rollback_snapshots(repo_root.path(), &snapshots);

        assert_eq!(
            read_file(&real_created_parent.join("new.md")),
            "created\n",
            "rollback should refuse deleting through the swapped symlink"
        );
        assert_eq!(
            read_file(&escaped_delete_root.path().join("new.md")),
            "outside delete\n",
            "rollback should not delete files outside the repo"
        );
        assert_eq!(
            read_file(&real_failing_parent.join("existing.md")),
            "before\n",
            "rollback should leave the original in-repo file untouched when restore is refused"
        );
        assert_eq!(
            read_file(&escaped_restore_root.path().join("existing.md")),
            "outside restore\n",
            "rollback should not restore bytes through the swapped symlink"
        );
    }

    #[test]
    fn rollback_snapshots_restores_written_files_for_simulated_state_persistence_failure() {
        let repo_root = tempfile::tempdir().expect("tempdir");
        let existing_path = repo_root.path().join("artifact.md");
        fs::write(&existing_path, "before\n").expect("seed existing target");

        let plan = test_plan(vec![
            write_intent("artifact.md", "after\n"),
            write_intent("new-output.md", "created\n"),
        ]);
        let mut snapshots = snapshot_targets(repo_root.path(), &plan).expect("snapshot");

        let written =
            write_capture_targets(repo_root.path(), &plan, &mut snapshots).expect("write targets");
        assert_eq!(
            written,
            vec!["artifact.md".to_string(), "new-output.md".to_string()]
        );
        assert_eq!(read_file(&existing_path), "after\n");
        assert_eq!(
            read_file(&repo_root.path().join("new-output.md")),
            "created\n"
        );

        // Simulate a later state-persistence failure after file writes succeeded.
        rollback_snapshots(repo_root.path(), &snapshots);

        assert_eq!(read_file(&existing_path), "before\n");
        assert!(
            !repo_root.path().join("new-output.md").exists(),
            "rollback should delete files that did not exist before the failed apply"
        );
    }

    #[test]
    fn build_repo_mirror_writes_refuses_missing_basename_match() {
        let err = build_repo_mirror_writes(
            &[write_intent(
                "artifacts/foundation/FOUNDATION_STRATEGY.md",
                "strategy\n",
            )],
            &["ENVIRONMENT_INVENTORY.md".to_string()],
        )
        .expect_err("missing basename match should refuse");

        assert!(
            err.contains("could not be derived from the declared artifact outputs"),
            "expected missing-match refusal, got: {err}"
        );
    }

    #[test]
    fn build_repo_mirror_writes_refuses_ambiguous_basename_match() {
        let err = build_repo_mirror_writes(
            &[
                write_intent("artifacts/a/ENVIRONMENT_INVENTORY.md", "first\n"),
                write_intent("artifacts/b/ENVIRONMENT_INVENTORY.md", "second\n"),
            ],
            &["ENVIRONMENT_INVENTORY.md".to_string()],
        )
        .expect_err("ambiguous basename match should refuse");

        assert!(
            err.contains("matches multiple artifact outputs by basename"),
            "expected ambiguous-match refusal, got: {err}"
        );
    }
}
