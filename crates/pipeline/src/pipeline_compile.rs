use crate::declarative_roots::{is_profile_file, profile_file};
use crate::pipeline::{
    load_selected_pipeline_definition, load_stage_compile_definition, CompileStageDefinition,
    CompileStageInput, CompileStageLoadError, CompileStageVariable, PipelineDefinition,
    SelectedPipelineLoadError, SupportedTargetRegistry,
};
use crate::repo_file_access::{read_repo_relative_string, RepoRelativeFileAccessError};
use crate::route_state::{
    load_trusted_pipeline_session, normalize_route_basis_run, RouteBasis, RouteBasisStageReason,
    RouteBasisStageStatus, TrustedPipelineSessionRefusal, ROUTE_BASIS_REPO_ROOT_SENTINEL,
};
use std::collections::BTreeMap;
use std::fmt;
use std::path::{Component, Path, PathBuf};
use time::macros::format_description;
use time::OffsetDateTime;

pub const PIPELINE_COMPILE_NOW_UTC_ENV_VAR: &str = "HANDBOOK_PIPELINE_COMPILE_NOW_UTC";
const NOW_UTC_FORMAT: &[time::format_description::FormatItem<'static>] =
    format_description!("[year]-[month]-[day]T[hour]:[minute]:[second]Z");

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineCompileResult {
    pub target: PipelineCompileTarget,
    pub basis: RouteBasis,
    pub variables: Vec<PipelineCompileVariable>,
    pub documents: Vec<PipelineCompileDocument>,
    pub outputs: Vec<PipelineCompileOutput>,
    pub gating: PipelineCompileGatingSummary,
    pub stage_body: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineCompileTarget {
    pub pipeline_id: String,
    pub stage_id: String,
    pub stage_file: String,
    pub title: String,
    pub description: String,
    pub work_level: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineCompileVariable {
    pub name: String,
    pub value: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineCompileDocument {
    pub kind: PipelineCompileDocumentKind,
    pub path: String,
    pub required: bool,
    pub status: PipelineCompileDocumentStatus,
    pub content: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PipelineCompileDocumentKind {
    Include,
    Runner,
    Profile,
    Library,
    Artifact,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PipelineCompileDocumentStatus {
    Present,
    MissingOptional,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineCompileOutput {
    pub kind: PipelineCompileOutputKind,
    pub path: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PipelineCompileOutputKind {
    Artifact,
    RepoFile,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineCompileGatingSummary {
    pub mode: Option<String>,
    pub fail_on: Vec<String>,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineCompileRefusal {
    pub classification: PipelineCompileRefusalClassification,
    pub summary: String,
    pub pipeline_id: Option<String>,
    pub stage_id: Option<String>,
    pub recovery: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PipelineCompileRefusalClassification {
    UnsupportedTarget,
    InvalidDefinition,
    InvalidState,
    MissingRouteBasis,
    MalformedRouteBasis,
    StaleRouteBasis,
    InactiveStage,
    MissingRequiredInput,
    EmptyRequiredInput,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PipelineCompileRuntimeContext {
    pub now_utc_override: Option<String>,
}

pub fn compile_pipeline_stage(
    repo_root: impl AsRef<Path>,
    pipeline_selector: &str,
    stage_selector: &str,
) -> Result<PipelineCompileResult, PipelineCompileRefusal> {
    compile_pipeline_stage_with_runtime(
        repo_root,
        pipeline_selector,
        stage_selector,
        &PipelineCompileRuntimeContext::default(),
    )
}

pub fn compile_pipeline_stage_with_runtime(
    repo_root: impl AsRef<Path>,
    pipeline_selector: &str,
    stage_selector: &str,
    runtime: &PipelineCompileRuntimeContext,
) -> Result<PipelineCompileResult, PipelineCompileRefusal> {
    let repo_root = repo_root.as_ref();
    let canonical_pipeline_id = SupportedTargetRegistry::canonical_compile_pipeline_id();
    let canonical_stage_id = SupportedTargetRegistry::canonical_compile_stage_id();
    let pipeline = load_selected_pipeline_definition(repo_root, pipeline_selector).map_err(
        |err| match err {
            SelectedPipelineLoadError::Lookup(err) => PipelineCompileRefusal {
                classification: PipelineCompileRefusalClassification::UnsupportedTarget,
                summary: err.to_string(),
                pipeline_id: None,
                stage_id: None,
                recovery: format!("retry with the canonical pipeline id `{canonical_pipeline_id}`"),
            },
            SelectedPipelineLoadError::Catalog(err) => PipelineCompileRefusal {
                classification: PipelineCompileRefusalClassification::InvalidDefinition,
                summary: format!("failed to load selected pipeline definition: {err}"),
                pipeline_id: None,
                stage_id: None,
                recovery: "fix the pipeline/stage definitions and retry `pipeline compile`"
                    .to_string(),
            },
            SelectedPipelineLoadError::Load(err) => PipelineCompileRefusal {
                classification: PipelineCompileRefusalClassification::InvalidDefinition,
                summary: format!("failed to load selected pipeline definition: {err}"),
                pipeline_id: None,
                stage_id: None,
                recovery: "fix the pipeline/stage definitions and retry `pipeline compile`"
                    .to_string(),
            },
        },
    )?;

    let resolved_stage_id =
        resolve_stage_selector(&pipeline, stage_selector).map_err(|summary| {
            let recovery = if pipeline.header.id == canonical_pipeline_id
                && stage_selector.trim() == canonical_stage_id
            {
                "re-run `pipeline resolve` and confirm the selected stage is declared in the pipeline"
                    .to_string()
            } else {
                format!("retry with the canonical stage id `{canonical_stage_id}`")
            };
            PipelineCompileRefusal {
                classification: PipelineCompileRefusalClassification::UnsupportedTarget,
                summary,
                pipeline_id: Some(pipeline.header.id.clone()),
                stage_id: Some(stage_selector.trim().to_string()),
                recovery,
            }
        })?;

    let registry =
        SupportedTargetRegistry::load(repo_root).map_err(|err| PipelineCompileRefusal {
            classification: PipelineCompileRefusalClassification::InvalidDefinition,
            summary: format!("failed to load supported target registry: {err}"),
            pipeline_id: Some(pipeline.header.id.clone()),
            stage_id: Some(resolved_stage_id.clone()),
            recovery: "fix the pipeline/stage definitions and retry `pipeline compile`".to_string(),
        })?;
    let supported_compile_target = registry.compile_target();

    if registry
        .resolve_compile_target(&pipeline.header.id, &resolved_stage_id)
        .is_err()
    {
        return Err(PipelineCompileRefusal {
            classification: PipelineCompileRefusalClassification::UnsupportedTarget,
            summary: format!(
                "M2 compile currently supports only `{}` + `{}`",
                supported_compile_target.pipeline.id, supported_compile_target.stage.id
            ),
            pipeline_id: Some(pipeline.header.id.clone()),
            stage_id: Some(resolved_stage_id),
            recovery: format!(
                "retry with `pipeline compile --id {} --stage {}`",
                supported_compile_target.pipeline.id, supported_compile_target.stage.id
            ),
        });
    }

    let stage_id = supported_compile_target.stage.id.clone();
    let trusted_session = load_trusted_pipeline_session(repo_root, &pipeline).map_err(|err| {
        classify_trusted_pipeline_session_refusal(err, &pipeline.header.id, &stage_id)
    })?;
    trusted_session
        .require_active_stage(&stage_id)
        .map_err(|err| {
            classify_trusted_pipeline_session_refusal(err, &pipeline.header.id, &stage_id)
        })?;
    let route_basis = trusted_session.route_basis;

    let stage_definition = load_stage_compile_definition(repo_root, &pipeline, &stage_id)
        .map_err(|err| classify_compile_stage_error(err, &pipeline.header.id, &stage_id))?;
    let work_level = stage_definition
        .work_level
        .clone()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| "L1".to_string());
    let variable_values = resolve_compile_variables(
        &pipeline.header.id,
        &route_basis,
        &stage_definition,
        &work_level,
        runtime,
    )?;
    validate_required_variables(
        &pipeline.header.id,
        &stage_id,
        &stage_definition.inputs.variables,
        &variable_values,
    )?;
    let declared_variables = render_declared_variables(
        &stage_definition.inputs.variables,
        &variable_values,
        &work_level,
    );
    let documents = assemble_documents(
        repo_root,
        &pipeline.header.id,
        &stage_definition,
        &route_basis,
        &variable_values,
        &work_level,
    )?;
    let outputs = render_outputs(&stage_definition, &variable_values);

    Ok(PipelineCompileResult {
        target: PipelineCompileTarget {
            pipeline_id: pipeline.header.id.clone(),
            stage_id,
            stage_file: stage_definition.stage.file.clone(),
            title: stage_definition.title.clone(),
            description: stage_definition.description.clone(),
            work_level,
            tags: stage_definition.tags.clone(),
        },
        basis: route_basis,
        variables: declared_variables,
        documents,
        outputs,
        gating: PipelineCompileGatingSummary {
            mode: stage_definition.gating.mode.clone(),
            fail_on: stage_definition.gating.fail_on.clone(),
            notes: stage_definition.gating.notes.clone(),
        },
        stage_body: stage_definition.body.clone(),
    })
}

pub fn render_pipeline_compile_payload(result: &PipelineCompileResult) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "# {} - {}\n",
        result.target.stage_id, result.target.title
    ));
    if !result.target.description.trim().is_empty() {
        out.push('\n');
        out.push_str(result.target.description.trim());
        out.push('\n');
    }

    out.push_str("\n## Run Variables\n");
    for variable in &result.variables {
        out.push_str(&format!(
            "- {}: {}\n",
            variable.name,
            variable.value.as_deref().unwrap_or("<unset>")
        ));
    }

    render_document_section(
        &mut out,
        "Selected Runner",
        result
            .documents
            .iter()
            .filter(|doc| doc.kind == PipelineCompileDocumentKind::Runner),
    );
    render_document_section(
        &mut out,
        "Selected Profile",
        result
            .documents
            .iter()
            .filter(|doc| doc.kind == PipelineCompileDocumentKind::Profile),
    );
    render_document_section(
        &mut out,
        "Includes",
        result
            .documents
            .iter()
            .filter(|doc| doc.kind == PipelineCompileDocumentKind::Include),
    );
    render_document_section(
        &mut out,
        "Library Inputs",
        result
            .documents
            .iter()
            .filter(|doc| doc.kind == PipelineCompileDocumentKind::Library),
    );
    render_document_section(
        &mut out,
        "Artifact Inputs",
        result
            .documents
            .iter()
            .filter(|doc| doc.kind == PipelineCompileDocumentKind::Artifact),
    );

    out.push_str("\n## Outputs\n");
    render_output_section(
        &mut out,
        "Artifacts",
        result
            .outputs
            .iter()
            .filter(|output| output.kind == PipelineCompileOutputKind::Artifact),
    );
    render_output_section(
        &mut out,
        "Repo Files",
        result
            .outputs
            .iter()
            .filter(|output| output.kind == PipelineCompileOutputKind::RepoFile),
    );

    if result.gating.mode.is_some()
        || !result.gating.fail_on.is_empty()
        || !result.gating.notes.is_empty()
    {
        out.push_str("\n## Gating\n");
        out.push_str(&format!(
            "mode: {}\n",
            result.gating.mode.as_deref().unwrap_or("<unset>")
        ));
        if result.gating.fail_on.is_empty() {
            out.push_str("fail_on:\n- <none>\n");
        } else {
            out.push_str("fail_on:\n");
            for item in &result.gating.fail_on {
                out.push_str(&format!("- {item}\n"));
            }
        }
        if !result.gating.notes.is_empty() {
            out.push_str("notes:\n");
            for note in &result.gating.notes {
                out.push_str(&format!("- {note}\n"));
            }
        }
    }

    if let Some(stage_body) = &result.stage_body {
        out.push_str("\n## Stage Body\n");
        out.push_str(stage_body.trim());
        out.push('\n');
    }

    normalize_rendered_output(&out)
}

pub fn render_pipeline_compile_explain(result: &PipelineCompileResult) -> String {
    let mut out = String::new();
    out.push_str("OUTCOME: COMPILED\n");
    out.push_str("TARGET:\n");
    out.push_str(&format!("  pipeline = {}\n", result.target.pipeline_id));
    out.push_str(&format!("  stage = {}\n", result.target.stage_id));
    out.push_str(&format!("  stage_file = {}\n", result.target.stage_file));
    out.push_str(&format!("  work_level = {}\n", result.target.work_level));
    out.push_str("ROUTE BASIS:\n");
    out.push_str(&format!(
        "  schema_version = {}\n",
        result.basis.schema_version
    ));
    out.push_str(&format!(
        "  state_revision = {}\n",
        result.basis.state_revision
    ));
    out.push_str(&format!(
        "  pipeline_file = {}\n",
        result.basis.pipeline_file
    ));
    out.push_str(&format!(
        "  pipeline_file_sha256 = {}\n",
        result.basis.pipeline_file_sha256
    ));
    out.push_str("  routing:\n");
    if result.basis.routing.is_empty() {
        out.push_str("    <empty>\n");
    } else {
        for (name, value) in &result.basis.routing {
            out.push_str(&format!("    {} = {}\n", name, value));
        }
    }
    out.push_str("  refs:\n");
    render_optional_basis_value(
        &mut out,
        "charter_ref",
        result.basis.refs.charter_ref.as_deref(),
    );
    render_optional_basis_value(
        &mut out,
        "project_context_ref",
        result.basis.refs.project_context_ref.as_deref(),
    );
    out.push_str("  run:\n");
    render_optional_basis_value(&mut out, "runner", result.basis.run.runner.as_deref());
    render_optional_basis_value(&mut out, "profile", result.basis.run.profile.as_deref());
    render_optional_basis_value(&mut out, "repo_root", result.basis.run.repo_root.as_deref());
    out.push_str("  runner:\n");
    out.push_str(&format!("    id = {}\n", result.basis.runner.id));
    out.push_str(&format!("    file = {}\n", result.basis.runner.file));
    out.push_str(&format!(
        "    file_sha256 = {}\n",
        result.basis.runner.file_sha256
    ));
    out.push_str("  profile:\n");
    out.push_str(&format!("    id = {}\n", result.basis.profile.id));
    out.push_str(&format!(
        "    profile.yaml.sha256 = {}\n",
        result.basis.profile.profile_yaml_sha256
    ));
    out.push_str(&format!(
        "    commands.yaml.sha256 = {}\n",
        result.basis.profile.commands_yaml_sha256
    ));
    out.push_str(&format!(
        "    conventions.md.sha256 = {}\n",
        result.basis.profile.conventions_md_sha256
    ));
    out.push_str("ROUTE SNAPSHOT:\n");
    for (index, stage) in result.basis.route.iter().enumerate() {
        out.push_str(&format!(
            "  {}. {} | {} | {}\n",
            index + 1,
            stage.stage_id,
            render_route_basis_status(stage.status),
            stage.file
        ));
        if let Some(reason) = &stage.reason {
            out.push_str(&format!(
                "     REASON: {}\n",
                render_route_basis_reason(Some(reason))
            ));
        }
    }
    out.push_str("VARIABLES:\n");
    for variable in &result.variables {
        out.push_str(&format!(
            "  {} = {}\n",
            variable.name,
            variable.value.as_deref().unwrap_or("<unset>")
        ));
    }
    out.push_str("DOCUMENTS:\n");
    for (index, document) in result.documents.iter().enumerate() {
        out.push_str(&format!(
            "  {}. {} | {} | required={} | status={}\n",
            index + 1,
            render_document_kind(document.kind),
            document.path,
            document.required,
            render_document_status(document.status)
        ));
    }
    out.push_str("OUTPUTS:\n");
    for output in &result.outputs {
        out.push_str(&format!(
            "  {} | {}\n",
            render_output_kind(output.kind),
            output.path
        ));
    }
    out.push_str("GATING:\n");
    out.push_str(&format!(
        "  mode = {}\n",
        result.gating.mode.as_deref().unwrap_or("<unset>")
    ));
    if result.gating.fail_on.is_empty() {
        out.push_str("  fail_on = <none>\n");
    } else {
        out.push_str(&format!(
            "  fail_on = {}\n",
            result.gating.fail_on.join(", ")
        ));
    }
    if result.gating.notes.is_empty() {
        out.push_str("  notes = <none>\n");
    } else {
        for note in &result.gating.notes {
            out.push_str(&format!("  note = {note}\n"));
        }
    }
    out.push_str(&format!(
        "STAGE BODY: {}\n",
        if result.stage_body.is_some() {
            "present"
        } else {
            "absent"
        }
    ));

    normalize_rendered_output(&out)
}

fn classify_trusted_pipeline_session_refusal(
    err: TrustedPipelineSessionRefusal,
    pipeline_id: &str,
    stage_id: &str,
) -> PipelineCompileRefusal {
    match err {
        TrustedPipelineSessionRefusal::MissingRouteBasis { .. } => PipelineCompileRefusal {
            classification: PipelineCompileRefusalClassification::MissingRouteBasis,
            summary: "persisted route_basis is missing for the selected pipeline".to_string(),
            pipeline_id: Some(pipeline_id.to_string()),
            stage_id: Some(stage_id.to_string()),
            recovery: "re-run `pipeline resolve` and retry `pipeline compile`".to_string(),
        },
        TrustedPipelineSessionRefusal::StaleRouteBasis { reason, .. } => {
            stale_basis_refusal(pipeline_id, stage_id, reason)
        }
        TrustedPipelineSessionRefusal::MalformedRouteBasis { reason, .. } => {
            PipelineCompileRefusal {
                classification: PipelineCompileRefusalClassification::MalformedRouteBasis,
                summary: format!("persisted route_basis is malformed: {reason}"),
                pipeline_id: Some(pipeline_id.to_string()),
                stage_id: Some(stage_id.to_string()),
                recovery: "re-run `pipeline resolve` and retry `pipeline compile`".to_string(),
            }
        }
        TrustedPipelineSessionRefusal::MissingStage { .. } => PipelineCompileRefusal {
            classification: PipelineCompileRefusalClassification::UnsupportedTarget,
            summary: format!(
                "selected stage `{stage_id}` is absent from the persisted resolved route"
            ),
            pipeline_id: Some(pipeline_id.to_string()),
            stage_id: Some(stage_id.to_string()),
            recovery: "re-run `pipeline resolve` and confirm the selected stage is declared in the pipeline".to_string(),
        },
        TrustedPipelineSessionRefusal::InactiveStage { reason, .. } => PipelineCompileRefusal {
            classification: PipelineCompileRefusalClassification::InactiveStage,
            summary: format!(
                "selected stage `{stage_id}` is not active in the persisted route: {}",
                render_route_basis_reason(reason.as_ref())
            ),
            pipeline_id: Some(pipeline_id.to_string()),
            stage_id: Some(stage_id.to_string()),
            recovery: "re-run `pipeline resolve`, adjust route state if needed, and retry `pipeline compile`".to_string(),
        },
        other => PipelineCompileRefusal {
            classification: PipelineCompileRefusalClassification::InvalidState,
            summary: other.to_string(),
            pipeline_id: Some(pipeline_id.to_string()),
            stage_id: Some(stage_id.to_string()),
            recovery: "fix the persisted route state and retry `pipeline compile`".to_string(),
        },
    }
}

fn classify_compile_stage_error(
    err: CompileStageLoadError,
    pipeline_id: &str,
    stage_id: &str,
) -> PipelineCompileRefusal {
    let classification = match err {
        CompileStageLoadError::StageNotDeclared { .. } => {
            PipelineCompileRefusalClassification::UnsupportedTarget
        }
        _ => PipelineCompileRefusalClassification::InvalidDefinition,
    };

    PipelineCompileRefusal {
        classification,
        summary: err.to_string(),
        pipeline_id: Some(pipeline_id.to_string()),
        stage_id: Some(stage_id.to_string()),
        recovery: "fix the selected stage definition and retry `pipeline compile`".to_string(),
    }
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

fn stale_basis_refusal(
    pipeline_id: &str,
    stage_id: &str,
    summary: String,
) -> PipelineCompileRefusal {
    PipelineCompileRefusal {
        classification: PipelineCompileRefusalClassification::StaleRouteBasis,
        summary,
        pipeline_id: Some(pipeline_id.to_string()),
        stage_id: Some(stage_id.to_string()),
        recovery: "re-run `pipeline resolve` and retry `pipeline compile`".to_string(),
    }
}

fn resolve_compile_variables(
    pipeline_id: &str,
    basis: &RouteBasis,
    stage_definition: &CompileStageDefinition,
    work_level: &str,
    runtime: &PipelineCompileRuntimeContext,
) -> Result<BTreeMap<String, String>, PipelineCompileRefusal> {
    let now_utc = resolve_now_utc(runtime).map_err(|summary| PipelineCompileRefusal {
        classification: PipelineCompileRefusalClassification::InvalidState,
        summary,
        pipeline_id: Some(pipeline_id.to_string()),
        stage_id: Some(stage_definition.id.clone()),
        recovery: "restore the compile runtime context and retry `pipeline compile`".to_string(),
    })?;
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
    let normalized_run = normalize_route_basis_run(&basis.run);
    values.insert("runner".to_string(), basis.runner.id.clone());
    values.insert("profile".to_string(), basis.profile.id.clone());
    values.insert(
        "repo_root".to_string(),
        normalized_run
            .repo_root
            .unwrap_or_else(|| ROUTE_BASIS_REPO_ROOT_SENTINEL.to_string()),
    );
    values.insert("now_utc".to_string(), now_utc);
    values.insert("work_level".to_string(), work_level.to_string());

    Ok(values)
}

fn validate_required_variables(
    pipeline_id: &str,
    stage_id: &str,
    declared: &[CompileStageVariable],
    values: &BTreeMap<String, String>,
) -> Result<(), PipelineCompileRefusal> {
    for variable in declared {
        if variable.optional {
            continue;
        }

        match (
            values.get(&variable.name),
            normalize_variable_value(values.get(&variable.name)),
        ) {
            (None, None) => {
                return Err(PipelineCompileRefusal {
                    classification: PipelineCompileRefusalClassification::MissingRequiredInput,
                    summary: format!(
                        "required compile variable `{}` is missing for `{stage_id}`",
                        variable.name
                    ),
                    pipeline_id: Some(pipeline_id.to_string()),
                    stage_id: Some(stage_id.to_string()),
                    recovery: "restore the required variable source and retry `pipeline compile`"
                        .to_string(),
                })
            }
            (Some(_), None) => {
                return Err(PipelineCompileRefusal {
                    classification: PipelineCompileRefusalClassification::EmptyRequiredInput,
                    summary: format!(
                        "required compile variable `{}` is empty for `{stage_id}`",
                        variable.name
                    ),
                    pipeline_id: Some(pipeline_id.to_string()),
                    stage_id: Some(stage_id.to_string()),
                    recovery: "fill the required variable source and retry `pipeline compile`"
                        .to_string(),
                })
            }
            (_, Some(_)) => {}
        }
    }

    Ok(())
}

fn render_declared_variables(
    declared: &[CompileStageVariable],
    values: &BTreeMap<String, String>,
    work_level: &str,
) -> Vec<PipelineCompileVariable> {
    let mut variables = declared
        .iter()
        .map(|variable| PipelineCompileVariable {
            name: variable.name.clone(),
            value: normalize_variable_value(values.get(&variable.name)),
        })
        .collect::<Vec<_>>();

    if !variables
        .iter()
        .any(|variable| variable.name == "work_level")
    {
        variables.push(PipelineCompileVariable {
            name: "work_level".to_string(),
            value: Some(work_level.to_string()),
        });
    }

    variables
}

fn normalize_variable_value(value: Option<&String>) -> Option<String> {
    value
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn resolve_now_utc(runtime: &PipelineCompileRuntimeContext) -> Result<String, String> {
    if let Some(value) = runtime.now_utc_override.as_ref() {
        return Ok(value.clone());
    }

    if let Ok(value) = std::env::var(PIPELINE_COMPILE_NOW_UTC_ENV_VAR) {
        return Ok(value);
    }

    OffsetDateTime::now_utc()
        .format(NOW_UTC_FORMAT)
        .map_err(|err| format!("failed to derive compile variable `now_utc`: {err}"))
}

fn assemble_documents(
    repo_root: &Path,
    pipeline_id: &str,
    stage_definition: &CompileStageDefinition,
    basis: &RouteBasis,
    variables: &BTreeMap<String, String>,
    work_level: &str,
) -> Result<Vec<PipelineCompileDocument>, PipelineCompileRefusal> {
    let mut documents = Vec::new();
    let runner_path = basis.runner.file.clone();
    let profile_paths = [
        profile_file(&basis.profile.id, "profile.yaml"),
        profile_file(&basis.profile.id, "commands.yaml"),
        profile_file(&basis.profile.id, "conventions.md"),
    ];

    for include in &stage_definition.includes {
        let path = substitute_variables(include, variables);
        let kind = if path == runner_path {
            PipelineCompileDocumentKind::Runner
        } else if is_profile_file(&path, &basis.profile.id)
            || profile_paths.iter().any(|candidate| candidate == &path)
        {
            PipelineCompileDocumentKind::Profile
        } else {
            PipelineCompileDocumentKind::Include
        };
        documents.push(load_required_document(
            repo_root,
            pipeline_id,
            &stage_definition.id,
            kind,
            &path,
            work_level,
        )?);
    }

    for input in &stage_definition.inputs.library {
        documents.push(load_declared_input(
            repo_root,
            pipeline_id,
            &stage_definition.id,
            PipelineCompileDocumentKind::Library,
            input,
            variables,
            work_level,
        )?);
    }

    for input in &stage_definition.inputs.artifacts {
        documents.push(load_declared_input(
            repo_root,
            pipeline_id,
            &stage_definition.id,
            PipelineCompileDocumentKind::Artifact,
            input,
            variables,
            work_level,
        )?);
    }

    Ok(documents)
}

fn load_required_document(
    repo_root: &Path,
    pipeline_id: &str,
    stage_id: &str,
    kind: PipelineCompileDocumentKind,
    path: &str,
    work_level: &str,
) -> Result<PipelineCompileDocument, PipelineCompileRefusal> {
    match load_repo_relative_document(repo_root, path, Some(work_level)) {
        Ok(content) => Ok(PipelineCompileDocument {
            kind,
            path: path.to_string(),
            required: true,
            status: PipelineCompileDocumentStatus::Present,
            content: Some(content),
        }),
        Err(DocumentLoadError::Missing) => Err(PipelineCompileRefusal {
            classification: PipelineCompileRefusalClassification::MissingRequiredInput,
            summary: format!("required compile-shaping input `{path}` is missing for `{stage_id}`"),
            pipeline_id: Some(pipeline_id.to_string()),
            stage_id: Some(stage_id.to_string()),
            recovery: "restore the missing input and retry `pipeline compile`".to_string(),
        }),
        Err(DocumentLoadError::Empty) => Err(PipelineCompileRefusal {
            classification: PipelineCompileRefusalClassification::EmptyRequiredInput,
            summary: format!("required compile-shaping input `{path}` is empty for `{stage_id}`"),
            pipeline_id: Some(pipeline_id.to_string()),
            stage_id: Some(stage_id.to_string()),
            recovery: "fill the required input and retry `pipeline compile`".to_string(),
        }),
        Err(DocumentLoadError::InvalidPath(reason)) => Err(PipelineCompileRefusal {
            classification: PipelineCompileRefusalClassification::InvalidDefinition,
            summary: format!("compile input path `{path}` is invalid: {reason}"),
            pipeline_id: Some(pipeline_id.to_string()),
            stage_id: Some(stage_id.to_string()),
            recovery: "fix the stage definition and retry `pipeline compile`".to_string(),
        }),
        Err(DocumentLoadError::ReadFailure(err_path, source)) => Err(PipelineCompileRefusal {
            classification: PipelineCompileRefusalClassification::InvalidState,
            summary: format!(
                "failed to read compile input {}: {source}",
                err_path.display()
            ),
            pipeline_id: Some(pipeline_id.to_string()),
            stage_id: Some(stage_id.to_string()),
            recovery: "fix the unreadable input and retry `pipeline compile`".to_string(),
        }),
    }
}

fn load_declared_input(
    repo_root: &Path,
    pipeline_id: &str,
    stage_id: &str,
    kind: PipelineCompileDocumentKind,
    input: &CompileStageInput,
    variables: &BTreeMap<String, String>,
    work_level: &str,
) -> Result<PipelineCompileDocument, PipelineCompileRefusal> {
    let path = substitute_variables(&input.path, variables);
    match load_repo_relative_document(repo_root, &path, Some(work_level)) {
        Ok(content) => Ok(PipelineCompileDocument {
            kind,
            path,
            required: input.required,
            status: PipelineCompileDocumentStatus::Present,
            content: Some(content),
        }),
        Err(DocumentLoadError::Missing) if !input.required => Ok(PipelineCompileDocument {
            kind,
            path,
            required: false,
            status: PipelineCompileDocumentStatus::MissingOptional,
            content: None,
        }),
        Err(DocumentLoadError::Empty) if !input.required => Ok(PipelineCompileDocument {
            kind,
            path,
            required: false,
            status: PipelineCompileDocumentStatus::MissingOptional,
            content: None,
        }),
        Err(DocumentLoadError::Missing) => Err(PipelineCompileRefusal {
            classification: PipelineCompileRefusalClassification::MissingRequiredInput,
            summary: format!("required input `{path}` is missing for `{stage_id}`"),
            pipeline_id: Some(pipeline_id.to_string()),
            stage_id: Some(stage_id.to_string()),
            recovery: "restore the required input and retry `pipeline compile`".to_string(),
        }),
        Err(DocumentLoadError::Empty) => Err(PipelineCompileRefusal {
            classification: PipelineCompileRefusalClassification::EmptyRequiredInput,
            summary: format!("required input `{path}` is empty for `{stage_id}`"),
            pipeline_id: Some(pipeline_id.to_string()),
            stage_id: Some(stage_id.to_string()),
            recovery: "fill the required input and retry `pipeline compile`".to_string(),
        }),
        Err(DocumentLoadError::InvalidPath(reason)) => Err(PipelineCompileRefusal {
            classification: PipelineCompileRefusalClassification::InvalidDefinition,
            summary: format!("compile input path `{path}` is invalid: {reason}"),
            pipeline_id: Some(pipeline_id.to_string()),
            stage_id: Some(stage_id.to_string()),
            recovery: "fix the stage definition and retry `pipeline compile`".to_string(),
        }),
        Err(DocumentLoadError::ReadFailure(err_path, source)) => Err(PipelineCompileRefusal {
            classification: PipelineCompileRefusalClassification::InvalidState,
            summary: format!(
                "failed to read compile input {}: {source}",
                err_path.display()
            ),
            pipeline_id: Some(pipeline_id.to_string()),
            stage_id: Some(stage_id.to_string()),
            recovery: "fix the unreadable input and retry `pipeline compile`".to_string(),
        }),
    }
}

fn render_outputs(
    stage_definition: &CompileStageDefinition,
    variables: &BTreeMap<String, String>,
) -> Vec<PipelineCompileOutput> {
    let mut outputs = Vec::new();
    for output in &stage_definition.outputs.artifacts {
        outputs.push(PipelineCompileOutput {
            kind: PipelineCompileOutputKind::Artifact,
            path: substitute_variables(&output.path, variables),
        });
    }
    for output in &stage_definition.outputs.repo_files {
        outputs.push(PipelineCompileOutput {
            kind: PipelineCompileOutputKind::RepoFile,
            path: substitute_variables(&output.path, variables),
        });
    }
    outputs
}

enum DocumentLoadError {
    Missing,
    Empty,
    InvalidPath(String),
    ReadFailure(PathBuf, std::io::Error),
}

fn load_repo_relative_document(
    repo_root: &Path,
    relative_path: &str,
    scoped_work_level: Option<&str>,
) -> Result<String, DocumentLoadError> {
    let contents =
        read_repo_relative_string(repo_root, relative_path).map_err(|err| match err {
            RepoRelativeFileAccessError::Missing(_) => DocumentLoadError::Missing,
            RepoRelativeFileAccessError::InvalidPath(reason) => {
                DocumentLoadError::InvalidPath(reason)
            }
            RepoRelativeFileAccessError::SymlinkNotAllowed(path)
            | RepoRelativeFileAccessError::NotRegularFile(path) => DocumentLoadError::ReadFailure(
                path.clone(),
                std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!(
                        "repo-relative file is not a regular non-symlink file: {}",
                        path.display()
                    ),
                ),
            ),
            RepoRelativeFileAccessError::ReadFailure { path, source } => {
                DocumentLoadError::ReadFailure(path, source)
            }
        })?;

    let filtered = if let Some(work_level) = scoped_work_level {
        filter_scoped_blocks(&contents, work_level)
    } else {
        normalize_text(&contents)
    };
    if filtered.trim().is_empty() {
        return Err(DocumentLoadError::Empty);
    }

    Ok(filtered)
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

fn filter_scoped_blocks(text: &str, work_level: &str) -> String {
    let mut out_lines = Vec::new();
    let mut include = true;

    for line in text.lines() {
        let trimmed = line.trim();
        if let Some(scopes) = parse_scope_start(trimmed) {
            include = scopes
                .iter()
                .any(|scope| scope == "ALL" || scope == work_level);
            continue;
        }
        if trimmed == "<!-- END_SCOPE -->" {
            include = true;
            continue;
        }
        if include {
            out_lines.push(line.trim_end().to_string());
        }
    }

    normalize_text(&out_lines.join("\n"))
}

fn parse_scope_start(line: &str) -> Option<Vec<String>> {
    let inner = line
        .strip_prefix("<!-- SCOPE:")
        .and_then(|value| value.strip_suffix("-->"))?;
    Some(
        inner
            .split(',')
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty())
            .collect(),
    )
}

fn normalize_text(text: &str) -> String {
    let normalized = text.replace("\r\n", "\n");
    let lines = normalized
        .lines()
        .map(str::trim_end)
        .collect::<Vec<_>>()
        .join("\n");
    let trimmed = lines.trim();
    if trimmed.is_empty() {
        String::new()
    } else {
        format!("{trimmed}\n")
    }
}

fn normalize_rendered_output(text: &str) -> String {
    let normalized = normalize_text(text);
    if normalized.is_empty() {
        String::new()
    } else {
        normalized
    }
}

fn render_document_section<'a>(
    out: &mut String,
    title: &str,
    docs: impl Iterator<Item = &'a PipelineCompileDocument>,
) {
    let docs = docs
        .filter(|doc| doc.status == PipelineCompileDocumentStatus::Present)
        .collect::<Vec<_>>();
    out.push_str(&format!("\n## {title}\n"));
    if docs.is_empty() {
        out.push_str("(none)\n");
        return;
    }
    for doc in docs {
        out.push_str(&format!("\n### {}\n", doc.path));
        if let Some(content) = &doc.content {
            out.push_str(content.trim_end());
            out.push('\n');
        }
    }
}

fn render_output_section<'a>(
    out: &mut String,
    title: &str,
    outputs: impl Iterator<Item = &'a PipelineCompileOutput>,
) {
    let outputs = outputs.collect::<Vec<_>>();
    out.push_str(&format!("\n### {title}\n"));
    if outputs.is_empty() {
        out.push_str("(none declared)\n");
        return;
    }
    for output in outputs {
        out.push_str(&format!("- {}\n", output.path));
    }
}

fn render_optional_basis_value(out: &mut String, name: &str, value: Option<&str>) {
    match value {
        Some(value) => out.push_str(&format!("    {} = {}\n", name, value)),
        None => out.push_str(&format!("    {} = <unset>\n", name)),
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

fn render_route_basis_reason(reason: Option<&RouteBasisStageReason>) -> String {
    match reason {
        Some(RouteBasisStageReason::SkippedActivationFalse {
            unsatisfied_variables,
            ..
        }) => format!(
            "activation evaluated false for variables: {}",
            unsatisfied_variables.join(", ")
        ),
        Some(RouteBasisStageReason::NextMissingRouteVariables {
            missing_variables, ..
        }) => format!("missing route variables: {}", missing_variables.join(", ")),
        Some(RouteBasisStageReason::BlockedByUnresolvedStage {
            upstream_stage_id,
            upstream_status,
        }) => format!(
            "blocked by unresolved stage {} ({})",
            upstream_stage_id,
            render_route_basis_status(*upstream_status)
        ),
        None => "no persisted reason".to_string(),
    }
}

fn render_document_kind(kind: PipelineCompileDocumentKind) -> &'static str {
    match kind {
        PipelineCompileDocumentKind::Include => "include",
        PipelineCompileDocumentKind::Runner => "runner",
        PipelineCompileDocumentKind::Profile => "profile",
        PipelineCompileDocumentKind::Library => "library",
        PipelineCompileDocumentKind::Artifact => "artifact",
    }
}

fn render_document_status(status: PipelineCompileDocumentStatus) -> &'static str {
    match status {
        PipelineCompileDocumentStatus::Present => "present",
        PipelineCompileDocumentStatus::MissingOptional => "missing_optional",
    }
}

fn render_output_kind(kind: PipelineCompileOutputKind) -> &'static str {
    match kind {
        PipelineCompileOutputKind::Artifact => "artifact",
        PipelineCompileOutputKind::RepoFile => "repo_file",
    }
}

impl fmt::Display for PipelineCompileRefusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.summary)
    }
}

impl fmt::Display for PipelineCompileRefusalClassification {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            PipelineCompileRefusalClassification::UnsupportedTarget => "unsupported_target",
            PipelineCompileRefusalClassification::InvalidDefinition => "invalid_definition",
            PipelineCompileRefusalClassification::InvalidState => "invalid_state",
            PipelineCompileRefusalClassification::MissingRouteBasis => "missing_route_basis",
            PipelineCompileRefusalClassification::MalformedRouteBasis => "malformed_route_basis",
            PipelineCompileRefusalClassification::StaleRouteBasis => "stale_route_basis",
            PipelineCompileRefusalClassification::InactiveStage => "inactive_stage",
            PipelineCompileRefusalClassification::MissingRequiredInput => "missing_required_input",
            PipelineCompileRefusalClassification::EmptyRequiredInput => "empty_required_input",
        };
        write!(f, "{value}")
    }
}

#[cfg(test)]
mod tests {
    use super::filter_scoped_blocks;

    #[test]
    fn scoped_filter_excludes_l2_and_l3_blocks_for_l1() {
        let input = "\
before
<!-- SCOPE: L2 -->
l2 only
<!-- END_SCOPE -->
<!-- SCOPE: L3 -->
l3 only
<!-- END_SCOPE -->
after
";

        let filtered = filter_scoped_blocks(input, "L1");

        assert_eq!(filtered, "before\nafter\n");
    }

    #[test]
    fn scoped_filter_includes_l2_and_all_blocks_for_l2() {
        let input = "\
before
<!-- SCOPE: ALL -->
all levels
<!-- END_SCOPE -->
<!-- SCOPE: L2 -->
l2 only
<!-- END_SCOPE -->
<!-- SCOPE: L3 -->
l3 only
<!-- END_SCOPE -->
after
";

        let filtered = filter_scoped_blocks(input, "L2");

        assert_eq!(filtered, "before\nall levels\nl2 only\nafter\n");
    }

    #[test]
    fn scoped_filter_includes_l3_and_all_blocks_for_l3() {
        let input = "\
before
<!-- SCOPE: ALL -->
all levels
<!-- END_SCOPE -->
<!-- SCOPE: L2 -->
l2 only
<!-- END_SCOPE -->
<!-- SCOPE: L3 -->
l3 only
<!-- END_SCOPE -->
after
";

        let filtered = filter_scoped_blocks(input, "L3");

        assert_eq!(filtered, "before\nall levels\nl3 only\nafter\n");
    }
}
