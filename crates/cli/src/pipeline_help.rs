use crate::shell_shared::discover_managed_repo_root;
use clap::Command;
use handbook_sdk::pipeline_route_api::{
    self, PipelineCatalogRequest, PipelineShowOutcome, PipelineShowRequest,
};
use std::path::Path;

pub(crate) const SUPPORTED_CAPTURE_HELP_SUMMARY: &str =
    "Capture one supported stage output and materialize declared artifact and repo-mirror files for the supported pipeline stages.";
pub(crate) const SUPPORTED_CAPTURE_HELP_EXAMPLES: &str =
    "Examples are loaded from the current managed repo when supported target metadata is available.";
pub(crate) const SUPPORTED_HANDOFF_HELP_SUMMARY: &str =
    "Emit one bounded handoff bundle for the supported pipeline and consumer.";
pub(crate) const SUPPORTED_HANDOFF_EMIT_HELP_SUMMARY: &str = SUPPORTED_HANDOFF_HELP_SUMMARY;
pub(crate) const SUPPORTED_HANDOFF_HELP_EXAMPLES: &str =
    "Examples are loaded from the current managed repo when supported target metadata is available.";
const SUPPORTED_HANDOFF_CONSUMER_ID: &str = "feature-slice-decomposer";
const SUPPORTED_COMPILE_STAGE_PATH: &str = "core/stages/10_feature_spec.md";
const SUPPORTED_CAPTURE_STAGE_PATHS: &[&str] = &[
    "core/stages/04_charter_inputs.md",
    "core/stages/05_charter_synthesize.md",
    "core/stages/06_project_context_interview.md",
    "core/stages/07_foundation_pack.md",
    SUPPORTED_COMPILE_STAGE_PATH,
];
const SUPPORTED_STAGE_PATHS: &[&str] = &[
    "core/stages/00_base.md",
    "core/stages/04_charter_inputs.md",
    "core/stages/05_charter_synthesize.md",
    "core/stages/06_project_context_interview.md",
    "core/stages/07_foundation_pack.md",
    SUPPORTED_COMPILE_STAGE_PATH,
];
#[derive(Debug, Clone)]
struct PipelineHelpText {
    capture_summary: String,
    capture_examples: String,
    handoff_summary: String,
    handoff_examples: String,
}

#[derive(Debug, Clone)]
pub(crate) struct SupportedPipelineHelpTarget {
    pub(crate) pipeline_id: String,
    pub(crate) compile_stage_id: String,
    pub(crate) capture_stage_ids: Vec<String>,
    pub(crate) consumer_id: String,
}

impl SupportedPipelineHelpTarget {
    pub(crate) fn handoff_emit_command(&self) -> String {
        render_supported_handoff_emit_command(&self.pipeline_id, &self.consumer_id)
    }
}

pub(crate) fn apply_dynamic_pipeline_help(command: Command) -> Command {
    let Some(help) = load_dynamic_pipeline_help() else {
        return command;
    };

    command.mut_subcommand("pipeline", move |pipeline| {
        let capture_summary = help.capture_summary.clone();
        let capture_examples = help.capture_examples.clone();
        let handoff_summary = help.handoff_summary.clone();
        let handoff_examples = help.handoff_examples.clone();

        pipeline
            .mut_subcommand("capture", move |capture| {
                capture.about(capture_summary).after_help(capture_examples)
            })
            .mut_subcommand("handoff", move |handoff| {
                let emit_summary = handoff_summary.clone();
                let emit_examples = handoff_examples.clone();
                handoff
                    .about(handoff_summary)
                    .after_help(handoff_examples)
                    .mut_subcommand("emit", move |emit| {
                        emit.about(emit_summary).after_help(emit_examples)
                    })
            })
    })
}

pub(crate) fn render_supported_handoff_emit_command(
    pipeline_id: &str,
    consumer_id: &str,
) -> String {
    format!(
        "handbook pipeline handoff emit --id {} --consumer {}",
        pipeline_id, consumer_id
    )
}

pub(crate) fn load_supported_pipeline_help_target(
    repo_root: &Path,
) -> Option<SupportedPipelineHelpTarget> {
    let catalog = pipeline_route_api::list_pipeline_catalog(&PipelineCatalogRequest {
        repository_root: repo_root.to_path_buf(),
    })
    .ok()?;
    let mut matches = catalog
        .pipelines
        .into_iter()
        .filter_map(|entry| {
            let outcome = pipeline_route_api::show_pipeline(&PipelineShowRequest {
                repository_root: repo_root.to_path_buf(),
                selector: entry.id,
            })
            .ok()?;
            let PipelineShowOutcome::Pipeline(pipeline) = outcome else {
                return None;
            };
            (pipeline.stages.len() == SUPPORTED_STAGE_PATHS.len()
                && pipeline
                    .stages
                    .iter()
                    .zip(SUPPORTED_STAGE_PATHS)
                    .all(|(stage, expected)| stage.source_path == Path::new(expected)))
            .then_some(pipeline)
        })
        .collect::<Vec<_>>();
    let pipeline = matches.pop()?;
    if !matches.is_empty() {
        return None;
    }
    let compile_stage_id = pipeline
        .stages
        .iter()
        .find(|stage| stage.source_path == Path::new(SUPPORTED_COMPILE_STAGE_PATH))
        .map(|stage| stage.id.clone())?;
    let capture_stage_ids = pipeline
        .stages
        .iter()
        .filter(|stage| {
            SUPPORTED_CAPTURE_STAGE_PATHS
                .iter()
                .any(|expected| stage.source_path == Path::new(expected))
        })
        .map(|stage| stage.id.clone())
        .collect::<Vec<_>>();
    if capture_stage_ids.len() != SUPPORTED_CAPTURE_STAGE_PATHS.len() {
        return None;
    }
    Some(SupportedPipelineHelpTarget {
        pipeline_id: pipeline.id,
        compile_stage_id,
        capture_stage_ids,
        consumer_id: SUPPORTED_HANDOFF_CONSUMER_ID.to_owned(),
    })
}

fn load_dynamic_pipeline_help() -> Option<PipelineHelpText> {
    let cwd = std::env::current_dir().ok()?;
    let repo_root = discover_managed_repo_root(&cwd);
    let target = load_supported_pipeline_help_target(&repo_root)?;
    let first_capture_stage_id = target.capture_stage_ids.first()?.clone();
    let preview_stage_id = target
        .capture_stage_ids
        .iter()
        .rev()
        .find(|stage_id| *stage_id != &target.compile_stage_id)
        .cloned()
        .unwrap_or_else(|| first_capture_stage_id.clone());
    let rendered_stage_list = target
        .capture_stage_ids
        .iter()
        .map(|stage_id| format!("`{stage_id}`"))
        .collect::<Vec<_>>();
    let handoff_command = target.handoff_emit_command();

    Some(PipelineHelpText {
        capture_summary: format!(
            "Capture one supported stage output and materialize declared artifact and repo-mirror files for `{}` stages {}",
            target.pipeline_id,
            render_human_stage_list(&rendered_stage_list)
        ),
        capture_examples: format!(
            "Examples:\n  handbook pipeline capture --id {} --stage {first_capture_stage_id}\n  handbook pipeline capture --id {} --stage {preview_stage_id} --preview\n  handbook pipeline capture --id {} --stage {} < /tmp/FEATURE_SPEC.md\n  handbook pipeline capture apply --capture-id <capture-id>",
            target.pipeline_id,
            target.pipeline_id,
            target.pipeline_id,
            target.compile_stage_id,
        ),
        handoff_summary: format!(
            "Emit one bounded handoff bundle for `{}` -> `{}`",
            target.pipeline_id, target.consumer_id
        ),
        handoff_examples: format!("Example:\n  {handoff_command}"),
    })
}

fn render_human_stage_list(stage_ids: &[String]) -> String {
    match stage_ids {
        [] => String::new(),
        [only] => only.clone(),
        [left, right] => format!("{left} and {right}"),
        _ => format!(
            "{}, and {}",
            stage_ids[..stage_ids.len() - 1].join(", "),
            stage_ids.last().expect("non-empty stage list")
        ),
    }
}
