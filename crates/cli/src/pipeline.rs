use super::{
    PipelineArgs, PipelineCaptureArgs, PipelineCaptureCommand, PipelineCommand,
    PipelineCompileArgs, PipelineHandoffArgs, PipelineHandoffCommand, PipelineSelectorArgs,
    PipelineShowArgs, PipelineStateCommand, PipelineStateSetArgs, RELEASE_VERSION,
};
use crate::shell_shared::{discover_managed_repo_root, read_stdin};
use std::process::ExitCode;

pub(super) fn run(args: PipelineArgs) -> ExitCode {
    match args.command {
        PipelineCommand::List => pipeline_list(),
        PipelineCommand::Show(args) => pipeline_show(args),
        PipelineCommand::Resolve(args) => pipeline_resolve(args),
        PipelineCommand::Compile(args) => pipeline_compile(args),
        PipelineCommand::Capture(args) => pipeline_capture(args),
        PipelineCommand::Handoff(args) => pipeline_handoff(args),
        PipelineCommand::State(args) => match args.command {
            PipelineStateCommand::Set(args) => pipeline_state_set(args),
        },
    }
}

fn pipeline_list() -> ExitCode {
    let cwd = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(err) => {
            println!("REFUSED: failed to determine repo root: {err}");
            return ExitCode::from(1);
        }
    };
    let repo_root = discover_managed_repo_root(&cwd);

    let catalog = match handbook_pipeline::load_pipeline_catalog_metadata(&repo_root) {
        Ok(catalog) => catalog,
        Err(err) => {
            println!("REFUSED: pipeline catalog error: {err}");
            return ExitCode::from(1);
        }
    };

    println!("{}", handbook_pipeline::render_pipeline_list(&catalog));
    ExitCode::SUCCESS
}

fn pipeline_show(args: PipelineShowArgs) -> ExitCode {
    let cwd = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(err) => {
            println!("REFUSED: failed to determine repo root: {err}");
            return ExitCode::from(1);
        }
    };
    let repo_root = discover_managed_repo_root(&cwd);

    let selection = match handbook_pipeline::load_pipeline_selection_metadata(&repo_root, &args.id)
    {
        Ok(selection) => selection,
        Err(handbook_pipeline::PipelineMetadataSelectionError::Catalog(err)) => {
            println!("REFUSED: pipeline catalog error: {err}");
            return ExitCode::from(1);
        }
        Err(handbook_pipeline::PipelineMetadataSelectionError::Lookup(err)) => {
            println!("{}", render_pipeline_selector_refusal(err));
            return ExitCode::from(1);
        }
    };

    println!("{}", handbook_pipeline::render_pipeline_show(&selection));
    ExitCode::SUCCESS
}

fn pipeline_resolve(args: PipelineSelectorArgs) -> ExitCode {
    let cwd = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(err) => {
            println!("REFUSED: failed to determine repo root: {err}");
            return ExitCode::from(1);
        }
    };
    let repo_root = discover_managed_repo_root(&cwd);

    let catalog = match handbook_pipeline::load_pipeline_catalog(&repo_root) {
        Ok(catalog) => catalog,
        Err(err) => {
            println!("REFUSED: pipeline catalog error: {err}");
            return ExitCode::from(1);
        }
    };

    let pipeline = match handbook_pipeline::resolve_pipeline_only_selector(&catalog, &args.id) {
        Ok(pipeline) => pipeline,
        Err(err) => {
            println!("{}", render_pipeline_selector_refusal(err));
            return ExitCode::from(1);
        }
    };

    let supported_variables =
        handbook_pipeline::supported_route_state_variables(&pipeline.definition);
    let state = match handbook_pipeline::load_route_state_with_supported_variables(
        &repo_root,
        &pipeline.definition.header.id,
        &supported_variables,
    ) {
        Ok(state) => state,
        Err(err) => {
            println!("REFUSED: {err}");
            return ExitCode::from(1);
        }
    };

    let route_variables = match handbook_pipeline::RouteVariables::new(state.routing.clone()) {
        Ok(variables) => variables,
        Err(err) => {
            println!("REFUSED: malformed route state variables: {err}");
            return ExitCode::from(1);
        }
    };

    let route =
        match handbook_pipeline::resolve_pipeline_route(&pipeline.definition, &route_variables) {
            Ok(route) => route,
            Err(err) => {
                println!("REFUSED: route resolution error: {err}");
                return ExitCode::from(1);
            }
        };

    let route_basis = match handbook_pipeline::build_route_basis(
        &repo_root,
        &pipeline.definition,
        &state,
        &route,
    ) {
        Ok(route_basis) => route_basis,
        Err(err) => {
            println!("REFUSED: route basis build error: {err}");
            return ExitCode::from(1);
        }
    };

    match handbook_pipeline::persist_route_basis(
        &repo_root,
        &pipeline.definition.header.id,
        route_basis,
    ) {
        Ok(handbook_pipeline::RouteBasisPersistOutcome::Applied(_)) => {}
        Ok(handbook_pipeline::RouteBasisPersistOutcome::Refused(refusal)) => {
            println!("REFUSED: route basis persistence refused: {refusal}");
            return ExitCode::from(1);
        }
        Err(err) => {
            println!("REFUSED: route basis persistence error: {err}");
            return ExitCode::from(1);
        }
    }

    println!(
        "{}",
        render_pipeline_resolve_output(
            &pipeline.definition.header.id,
            &state,
            &handbook_pipeline::effective_route_basis_run(&repo_root, &pipeline.definition, &state),
            &route,
        )
    );
    ExitCode::SUCCESS
}

fn pipeline_compile(args: PipelineCompileArgs) -> ExitCode {
    let cwd = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(err) => {
            println!("REFUSED: failed to determine repo root: {err}");
            return ExitCode::from(1);
        }
    };
    let repo_root = discover_managed_repo_root(&cwd);

    match handbook_pipeline::compile_pipeline_stage(&repo_root, &args.id, &args.stage) {
        Ok(result) => {
            if args.explain {
                println!(
                    "{}",
                    handbook_pipeline::render_pipeline_compile_explain(&result)
                );
            } else {
                println!(
                    "{}",
                    handbook_pipeline::render_pipeline_compile_payload(&result)
                );
            }
            ExitCode::SUCCESS
        }
        Err(refusal) => {
            println!(
                "{}",
                render_pipeline_compile_refusal(refusal, &args.id, &args.stage)
            );
            ExitCode::from(1)
        }
    }
}

fn pipeline_capture(args: PipelineCaptureArgs) -> ExitCode {
    let cwd = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(err) => {
            println!("REFUSED: failed to determine repo root: {err}");
            return ExitCode::from(1);
        }
    };
    let repo_root = discover_managed_repo_root(&cwd);

    match args.command {
        Some(PipelineCaptureCommand::Apply(apply_args)) => {
            match handbook_pipeline::apply_pipeline_capture(&repo_root, &apply_args.capture_id) {
                Ok(result) => {
                    println!(
                        "{}",
                        handbook_pipeline::render_pipeline_capture_apply_result(&result)
                    );
                    ExitCode::SUCCESS
                }
                Err(refusal) => {
                    println!(
                        "{}",
                        handbook_pipeline::render_pipeline_capture_refusal(&refusal, None, None)
                    );
                    ExitCode::from(1)
                }
            }
        }
        None => {
            let Some(pipeline_id) = args.id.as_deref() else {
                println!("REFUSED: `pipeline capture` requires --id");
                return ExitCode::from(1);
            };
            let Some(stage_id) = args.stage.as_deref() else {
                println!("REFUSED: `pipeline capture` requires --stage");
                return ExitCode::from(1);
            };
            let stdin = match read_stdin() {
                Ok(value) => value,
                Err(err) => {
                    println!("REFUSED: failed to read capture input from stdin: {err}");
                    return ExitCode::from(1);
                }
            };
            let request = handbook_pipeline::PipelineCaptureRequest {
                pipeline_selector: pipeline_id.to_string(),
                stage_selector: stage_id.to_string(),
                input: stdin,
            };

            if args.preview {
                match handbook_pipeline::preview_pipeline_capture(&repo_root, &request) {
                    Ok(preview) => {
                        println!(
                            "{}",
                            handbook_pipeline::render_pipeline_capture_preview(&preview)
                        );
                        ExitCode::SUCCESS
                    }
                    Err(refusal) => {
                        println!(
                            "{}",
                            handbook_pipeline::render_pipeline_capture_refusal(
                                &refusal,
                                Some(pipeline_id),
                                Some(stage_id),
                            )
                        );
                        ExitCode::from(1)
                    }
                }
            } else {
                match handbook_pipeline::capture_pipeline_output(&repo_root, &request) {
                    Ok(result) => {
                        println!(
                            "{}",
                            handbook_pipeline::render_pipeline_capture_apply_result(&result)
                        );
                        ExitCode::SUCCESS
                    }
                    Err(refusal) => {
                        println!(
                            "{}",
                            handbook_pipeline::render_pipeline_capture_refusal(
                                &refusal,
                                Some(pipeline_id),
                                Some(stage_id),
                            )
                        );
                        ExitCode::from(1)
                    }
                }
            }
        }
    }
}

fn pipeline_handoff(args: PipelineHandoffArgs) -> ExitCode {
    let cwd = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(err) => {
            println!("REFUSED: failed to determine repo root: {err}");
            return ExitCode::from(1);
        }
    };
    let repo_root = discover_managed_repo_root(&cwd);

    match args.command {
        PipelineHandoffCommand::Emit(emit_args) => {
            let supported_target = match handbook_pipeline::pipeline::SupportedTargetRegistry::load(
                &repo_root,
            ) {
                Ok(registry) => registry.handoff_target(),
                Err(err) => {
                    println!(
                            "{}",
                            handbook_pipeline::render_pipeline_handoff_refusal(
                                &handbook_pipeline::PipelineHandoffRefusal {
                                    classification:
                                        handbook_pipeline::PipelineHandoffRefusalClassification::InvalidState,
                                    summary: format!(
                                        "failed to load supported target registry: {err}"
                                    ),
                                    pipeline_id: None,
                                    consumer_id: None,
                                    recovery:
                                        "fix the pipeline/stage definitions and retry `pipeline handoff emit`"
                                            .to_string(),
                                }
                            )
                        );
                    return ExitCode::from(1);
                }
            };
            let request = handbook_pipeline::PipelineHandoffEmitRequest {
                pipeline_selector: emit_args.id,
                consumer_selector: emit_args.consumer,
                producer_command:
                    handbook_pipeline::pipeline_handoff::render_supported_handoff_emit_command(
                        &supported_target,
                    ),
                producer_version: RELEASE_VERSION.to_string(),
            };
            match handbook_pipeline::emit_pipeline_handoff_bundle(&repo_root, &request) {
                Ok(result) => {
                    println!(
                        "{}",
                        handbook_pipeline::render_pipeline_handoff_emit_result(&result)
                    );
                    ExitCode::SUCCESS
                }
                Err(refusal) => {
                    println!(
                        "{}",
                        handbook_pipeline::render_pipeline_handoff_refusal(&refusal)
                    );
                    ExitCode::from(1)
                }
            }
        }
    }
}

fn pipeline_state_set(args: PipelineStateSetArgs) -> ExitCode {
    let cwd = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(err) => {
            println!("REFUSED: failed to determine repo root: {err}");
            return ExitCode::from(1);
        }
    };
    let repo_root = discover_managed_repo_root(&cwd);

    let catalog = match handbook_pipeline::load_pipeline_catalog(&repo_root) {
        Ok(catalog) => catalog,
        Err(err) => {
            println!("REFUSED: pipeline catalog error: {err}");
            return ExitCode::from(1);
        }
    };

    let pipeline = match handbook_pipeline::resolve_pipeline_only_selector(&catalog, &args.id) {
        Ok(pipeline) => pipeline,
        Err(err) => {
            println!("{}", render_pipeline_selector_refusal(err));
            return ExitCode::from(1);
        }
    };

    let supported_variables =
        handbook_pipeline::supported_route_state_variables(&pipeline.definition);
    let current_state = match handbook_pipeline::load_route_state_with_supported_variables(
        &repo_root,
        &pipeline.definition.header.id,
        &supported_variables,
    ) {
        Ok(state) => state,
        Err(err) => {
            println!("REFUSED: {err}");
            return ExitCode::from(1);
        }
    };

    let mutation = match parse_route_state_mutation(&args) {
        Ok(mutation) => mutation,
        Err(err) => {
            println!("REFUSED: {err}");
            return ExitCode::from(1);
        }
    };

    let expected_revision = args.expected_revision.unwrap_or(current_state.revision);
    let outcome = match handbook_pipeline::set_route_state(
        &repo_root,
        &pipeline.definition.header.id,
        supported_variables,
        mutation,
        expected_revision,
    ) {
        Ok(outcome) => outcome,
        Err(err) => {
            println!("REFUSED: route state mutation error: {err}");
            return ExitCode::from(1);
        }
    };

    match outcome {
        handbook_pipeline::RouteStateMutationOutcome::Applied(state) => {
            println!(
                "{}",
                render_pipeline_state_set_output(
                    &pipeline.definition.header.id,
                    handbook_pipeline::RouteStateMutationOutcome::Applied(state),
                )
            );
            ExitCode::SUCCESS
        }
        handbook_pipeline::RouteStateMutationOutcome::Refused(refusal) => {
            println!(
                "{}",
                render_pipeline_state_set_output(
                    &pipeline.definition.header.id,
                    handbook_pipeline::RouteStateMutationOutcome::Refused(refusal),
                )
            );
            ExitCode::from(1)
        }
    }
}

fn render_pipeline_selector_refusal(err: handbook_pipeline::PipelineLookupError) -> String {
    match err {
        handbook_pipeline::PipelineLookupError::AmbiguousSelector { selector, matches } => {
            format!(
                "REFUSED: ambiguous selector `{selector}` matched multiple canonical ids: {}\nNEXT SAFE ACTION: use the full canonical id or rename the conflicting ids",
                matches.join(", ")
            )
        }
        handbook_pipeline::PipelineLookupError::UnknownSelector { selector } => format!(
            "REFUSED: unknown pipeline selector `{selector}`; use a canonical id or `pipeline list` to inspect available inventory\nNEXT SAFE ACTION: run `pipeline list` and retry with the full canonical id"
        ),
        handbook_pipeline::PipelineLookupError::UnsupportedSelector { selector, reason } => {
            let next_safe_action = if reason.contains("raw file paths are evidence only") {
                "use `pipeline list` to inspect available inventory and retry with a canonical pipeline or stage id"
            } else {
                "retry with a canonical pipeline id"
            };

            format!(
                "REFUSED: unsupported selector `{selector}`: {reason}\nNEXT SAFE ACTION: {next_safe_action}"
            )
        }
    }
}

fn render_pipeline_compile_refusal(
    refusal: handbook_pipeline::PipelineCompileRefusal,
    requested_pipeline_id: &str,
    requested_stage_id: &str,
) -> String {
    let pipeline_id = refusal
        .pipeline_id
        .as_deref()
        .unwrap_or(requested_pipeline_id.trim());
    let stage_id = refusal
        .stage_id
        .as_deref()
        .unwrap_or(requested_stage_id.trim());
    let mut out = String::new();
    out.push_str("OUTCOME: REFUSED\n");
    out.push_str(&format!("PIPELINE: {pipeline_id}\n"));
    out.push_str(&format!("STAGE: {stage_id}\n"));
    out.push_str(&format!(
        "REASON: {}: {}\n",
        render_pipeline_compile_refusal_classification(refusal.classification),
        refusal.summary.trim()
    ));
    out.push_str(&format!(
        "BROKEN SUBJECT: pipeline `{pipeline_id}` stage `{stage_id}`\n"
    ));
    out.push_str(&format!(
        "NEXT SAFE ACTION: {}\n",
        render_pipeline_compile_next_safe_action(&refusal, pipeline_id, stage_id)
    ));
    out.trim_end().to_string()
}

fn render_pipeline_compile_refusal_classification(
    classification: handbook_pipeline::PipelineCompileRefusalClassification,
) -> &'static str {
    match classification {
        handbook_pipeline::PipelineCompileRefusalClassification::UnsupportedTarget => {
            "unsupported_target"
        }
        handbook_pipeline::PipelineCompileRefusalClassification::InvalidDefinition => {
            "invalid_definition"
        }
        handbook_pipeline::PipelineCompileRefusalClassification::InvalidState => "invalid_state",
        handbook_pipeline::PipelineCompileRefusalClassification::MissingRouteBasis => {
            "missing_route_basis"
        }
        handbook_pipeline::PipelineCompileRefusalClassification::MalformedRouteBasis => {
            "malformed_route_basis"
        }
        handbook_pipeline::PipelineCompileRefusalClassification::StaleRouteBasis => {
            "stale_route_basis"
        }
        handbook_pipeline::PipelineCompileRefusalClassification::InactiveStage => "inactive_stage",
        handbook_pipeline::PipelineCompileRefusalClassification::MissingRequiredInput => {
            "missing_required_input"
        }
        handbook_pipeline::PipelineCompileRefusalClassification::EmptyRequiredInput => {
            "empty_required_input"
        }
    }
}

fn render_pipeline_compile_next_safe_action(
    refusal: &handbook_pipeline::PipelineCompileRefusal,
    pipeline_id: &str,
    stage_id: &str,
) -> String {
    match refusal.classification {
        handbook_pipeline::PipelineCompileRefusalClassification::UnsupportedTarget => {
            if refusal
                .recovery
                .trim()
                .contains("confirm the selected stage is declared in the pipeline")
            {
                format!(
                    "run `handbook pipeline resolve --id {pipeline_id}` and confirm `{stage_id}` is declared in pipeline `{pipeline_id}` before retrying `handbook pipeline compile --id {pipeline_id} --stage {stage_id}`"
                )
            } else {
                refusal.recovery.trim().to_string()
            }
        }
        handbook_pipeline::PipelineCompileRefusalClassification::MissingRouteBasis
        | handbook_pipeline::PipelineCompileRefusalClassification::MalformedRouteBasis
        | handbook_pipeline::PipelineCompileRefusalClassification::StaleRouteBasis => format!(
            "run `handbook pipeline resolve --id {pipeline_id}` and then retry `handbook pipeline compile --id {pipeline_id} --stage {stage_id}`"
        ),
        handbook_pipeline::PipelineCompileRefusalClassification::InactiveStage => format!(
            "run `handbook pipeline resolve --id {pipeline_id}`, adjust route state if needed, and then retry `handbook pipeline compile --id {pipeline_id} --stage {stage_id}`"
        ),
        _ => format!(
            "{}; then retry `handbook pipeline compile --id {pipeline_id} --stage {stage_id}`",
            refusal.recovery.trim()
        ),
    }
}

fn parse_route_state_mutation(
    args: &PipelineStateSetArgs,
) -> Result<handbook_pipeline::RouteStateMutation, String> {
    match (&args.var, &args.field) {
        (Some(value), None) => parse_route_state_var_assignment(value),
        (None, Some(value)) => parse_route_state_field_assignment(value),
        (Some(_), Some(_)) => Err("use exactly one of --var or --field".to_string()),
        (None, None) => Err("one of --var or --field is required".to_string()),
    }
}

fn parse_route_state_var_assignment(
    value: &str,
) -> Result<handbook_pipeline::RouteStateMutation, String> {
    let trimmed = value.trim();
    let Some((name, raw_value)) = trimmed.split_once('=') else {
        return Err("expected --var in name=value form".to_string());
    };

    let name = name.trim();
    let raw_value = raw_value.trim();
    if name.is_empty() {
        return Err("--var name must not be empty".to_string());
    }

    let parsed_value = match raw_value {
        "true" => true,
        "false" => false,
        _ => {
            return Err(format!(
                "unsupported --var value `{raw_value}`; expected `true` or `false`"
            ));
        }
    };

    Ok(handbook_pipeline::RouteStateMutation::RoutingVariable {
        variable: name.to_string(),
        value: parsed_value,
    })
}

fn parse_route_state_field_assignment(
    value: &str,
) -> Result<handbook_pipeline::RouteStateMutation, String> {
    let trimmed = value.trim();
    let Some((field_path, raw_value)) = trimmed.split_once('=') else {
        return Err("expected --field in field.path=value form".to_string());
    };

    let field_path = field_path.trim();
    let raw_value = raw_value.trim();
    if field_path.is_empty() {
        return Err("--field path must not be empty".to_string());
    }
    if raw_value.is_empty() {
        return Err("--field value must not be empty".to_string());
    }

    match field_path {
        "run.runner" => Ok(handbook_pipeline::RouteStateMutation::RunRunner {
            value: raw_value.to_string(),
        }),
        "run.profile" => Ok(handbook_pipeline::RouteStateMutation::RunProfile {
            value: raw_value.to_string(),
        }),
        "refs.charter_ref" => Ok(handbook_pipeline::RouteStateMutation::RefCharterRef {
            value: raw_value.to_string(),
        }),
        "refs.project_context_ref" => {
            Ok(handbook_pipeline::RouteStateMutation::RefProjectContextRef {
                value: raw_value.to_string(),
            })
        }
        _ => Err(format!(
            "unsupported --field path `{field_path}`; expected one of `run.runner`, `run.profile`, `refs.charter_ref`, or `refs.project_context_ref`"
        )),
    }
}

fn render_pipeline_resolve_output(
    pipeline_id: &str,
    state: &handbook_pipeline::RouteState,
    effective_run: &handbook_pipeline::RouteStateRun,
    route: &handbook_pipeline::ResolvedPipelineRoute,
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

fn render_optional_route_basis_field(out: &mut String, name: &str, value: Option<&str>) {
    match value {
        Some(value) => out.push_str(&format!("    {} = {}\n", name, value)),
        None => out.push_str(&format!("    {} = <unset>\n", name)),
    }
}

fn render_route_stage_reason(reason: &handbook_pipeline::RouteStageReason) -> String {
    match reason {
        handbook_pipeline::RouteStageReason::SkippedActivationFalse {
            unsatisfied_variables,
            ..
        } => format!(
            "activation evaluated false for variables: {}",
            unsatisfied_variables.join(", ")
        ),
        handbook_pipeline::RouteStageReason::NextMissingRouteVariables {
            missing_variables,
            ..
        } => format!("missing route variables: {}", missing_variables.join(", ")),
        handbook_pipeline::RouteStageReason::BlockedByUnresolvedStage {
            upstream_stage_id,
            upstream_status,
        } => format!(
            "blocked by unresolved stage {} ({})",
            upstream_stage_id,
            upstream_status.as_str()
        ),
    }
}

fn render_pipeline_state_set_output(
    pipeline_id: &str,
    outcome: handbook_pipeline::RouteStateMutationOutcome,
) -> String {
    let mut out = String::new();
    match outcome {
        handbook_pipeline::RouteStateMutationOutcome::Applied(state) => {
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
        handbook_pipeline::RouteStateMutationOutcome::Refused(refusal) => {
            out.push_str("OUTCOME: REFUSED\n");
            out.push_str(&format!("PIPELINE: {pipeline_id}\n"));
            out.push_str(&format!("REASON: {}\n", refusal));
        }
    }

    out.trim_end().to_string()
}

fn render_optional_state_field(out: &mut String, name: &str, value: Option<&str>) {
    match value {
        Some(value) => out.push_str(&format!("  {} = {}\n", name, value)),
        None => out.push_str(&format!("  {} = <unset>\n", name)),
    }
}
