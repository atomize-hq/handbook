use super::{
    PipelineArgs, PipelineCaptureArgs, PipelineCaptureCommand, PipelineCommand,
    PipelineCompileArgs, PipelineHandoffArgs, PipelineHandoffCommand, PipelineSelectorArgs,
    PipelineShowArgs, PipelineStateCommand, PipelineStateSetArgs, RELEASE_VERSION,
};
use crate::{
    doctor_rendering, exit_policy, pipeline_help,
    shell_shared::{discover_managed_repo_root, read_stdin},
};
use handbook_sdk::contract_membrane::{
    pipeline_capture_apply_from_response, pipeline_capture_apply_operation_request,
    pipeline_capture_direct_operation_request, pipeline_capture_plan_from_response,
    pipeline_capture_plan_operation_request, pipeline_capture_refusal_from_response,
    pipeline_catalog_list_from_response, pipeline_catalog_list_operation_request,
    pipeline_catalog_read_from_response, pipeline_catalog_read_operation_request,
    pipeline_compile_from_response, pipeline_compile_operation_request,
    pipeline_handoff_emit_from_response, pipeline_handoff_emit_operation_request,
    pipeline_handoff_refusal_from_response, pipeline_operation_problem_details,
    pipeline_state_apply_from_response, pipeline_state_apply_operation_request,
    pipeline_state_refusal_from_response, sha256_fingerprint, ContractLimits, InvocationResponseV1,
    OperationResponseV1, PipelineStateProblemProjection, TransportFailureKind, TransportFailureV1,
    UseCaseResponseV1,
};
use handbook_sdk::pipeline_route_api as pipeline_sdk;
use std::process::ExitCode;

pub(super) fn run(args: PipelineArgs, machine_json: bool) -> ExitCode {
    match args.command {
        PipelineCommand::List => pipeline_list(machine_json),
        PipelineCommand::Show(args) => pipeline_show(args, machine_json),
        PipelineCommand::Resolve(args) => pipeline_resolve(args, machine_json),
        PipelineCommand::Compile(args) => pipeline_compile(args, machine_json),
        PipelineCommand::Capture(args) => pipeline_capture(args, machine_json),
        PipelineCommand::Handoff(args) => pipeline_handoff(args, machine_json),
        PipelineCommand::State(args) => match args.command {
            PipelineStateCommand::Set(args) => pipeline_state_set(args, machine_json),
        },
    }
}

fn pipeline_list(machine_json: bool) -> ExitCode {
    let limits = ContractLimits::packet0();
    let cwd = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(err) => {
            return pipeline_root_failure(machine_json, &err.to_string(), &limits);
        }
    };
    let repo_root = discover_managed_repo_root(&cwd);
    let request = match pipeline_catalog_list_operation_request(None) {
        Ok(request) => request,
        Err(_) => return pipeline_adapter_failure(machine_json, &limits),
    };
    let response =
        match handbook_sdk::HandbookSdkV1::open(repo_root).execute_pipeline_operation(&request) {
            Ok(response) => response,
            Err(_) => return pipeline_adapter_failure(machine_json, &limits),
        };
    emit_pipeline_list(response, machine_json, &limits)
}

fn pipeline_show(args: PipelineShowArgs, machine_json: bool) -> ExitCode {
    let limits = ContractLimits::packet0();
    let cwd = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(err) => {
            return pipeline_root_failure(machine_json, &err.to_string(), &limits);
        }
    };
    let repo_root = discover_managed_repo_root(&cwd);
    let request = match pipeline_catalog_read_operation_request(None, &args.id) {
        Ok(request) => request,
        Err(_) => return pipeline_adapter_failure(machine_json, &limits),
    };
    let response =
        match handbook_sdk::HandbookSdkV1::open(repo_root).execute_pipeline_operation(&request) {
            Ok(response) => response,
            Err(_) => return pipeline_adapter_failure(machine_json, &limits),
        };
    emit_pipeline_show(response, machine_json, &limits)
}

fn pipeline_resolve(args: PipelineSelectorArgs, machine_json: bool) -> ExitCode {
    let limits = ContractLimits::packet0();
    let cwd = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(err) => {
            return pipeline_root_failure(machine_json, &err.to_string(), &limits);
        }
    };
    let repo_root = discover_managed_repo_root(&cwd);
    let response = match handbook_sdk::HandbookSdkV1::open(repo_root)
        .resolve_pipeline_use_case(&args.id, &limits)
    {
        Ok(response) => response,
        Err(_) => return pipeline_adapter_failure(machine_json, &limits),
    };
    emit_pipeline_resolve(response, machine_json, &limits)
}

fn pipeline_compile(args: PipelineCompileArgs, machine_json: bool) -> ExitCode {
    let limits = ContractLimits::packet0();
    let cwd = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(err) => {
            return pipeline_root_failure(machine_json, &err.to_string(), &limits);
        }
    };
    let repo_root = discover_managed_repo_root(&cwd);
    let request = match pipeline_compile_operation_request(None, &args.id, &args.stage) {
        Ok(request) => request,
        Err(_) => return pipeline_adapter_failure(machine_json, &limits),
    };
    let response =
        match handbook_sdk::HandbookSdkV1::open(repo_root).execute_pipeline_operation(&request) {
            Ok(response) => response,
            Err(_) => return pipeline_adapter_failure(machine_json, &limits),
        };
    emit_pipeline_compile(response, &args, machine_json, &limits)
}

fn emit_pipeline_list(
    response: OperationResponseV1,
    machine_json: bool,
    limits: &ContractLimits,
) -> ExitCode {
    let semantic_exit = exit_policy::terminal(response.terminal().status());
    if machine_json {
        return emit_operation_invocation(response, semantic_exit, limits);
    }
    let rendered = match pipeline_catalog_list_from_response(&response) {
        Ok(Some(catalog)) => render_pipeline_list(&catalog),
        Ok(None) => match pipeline_operation_problem_details(&response) {
            Ok(Some((detail, _, _))) => format!("REFUSED: pipeline catalog error: {detail}"),
            _ => "PRESENTATION FAILURE: invalid Pipeline catalog response".to_owned(),
        },
        Err(_) => "PRESENTATION FAILURE: invalid Pipeline catalog response".to_owned(),
    };
    emit_human(&rendered, semantic_exit)
}

fn emit_pipeline_show(
    response: OperationResponseV1,
    machine_json: bool,
    limits: &ContractLimits,
) -> ExitCode {
    let semantic_exit = exit_policy::terminal(response.terminal().status());
    if machine_json {
        return emit_operation_invocation(response, semantic_exit, limits);
    }
    let rendered = match pipeline_catalog_read_from_response(&response) {
        Ok(Some(selection)) => render_pipeline_show(&selection),
        Ok(None) => match pipeline_operation_problem_details(&response) {
            Ok(Some((_, Some(selector), _))) => render_pipeline_selector_refusal(selector),
            Ok(Some((detail, None, _))) => format!("REFUSED: pipeline catalog error: {detail}"),
            _ => "PRESENTATION FAILURE: invalid Pipeline catalog-read response".to_owned(),
        },
        Err(_) => "PRESENTATION FAILURE: invalid Pipeline catalog-read response".to_owned(),
    };
    emit_human(&rendered, semantic_exit)
}

fn emit_pipeline_compile(
    response: OperationResponseV1,
    args: &PipelineCompileArgs,
    machine_json: bool,
    limits: &ContractLimits,
) -> ExitCode {
    let semantic_exit = exit_policy::terminal(response.terminal().status());
    if machine_json {
        return emit_operation_invocation(response, semantic_exit, limits);
    }
    let rendered = match pipeline_compile_from_response(&response) {
        Ok(Some(result)) if args.explain => render_pipeline_compile_explain(&result),
        Ok(Some(result)) => render_pipeline_compile_payload(&result),
        Ok(None) => match pipeline_operation_problem_details(&response) {
            Ok(Some((_, _, Some(refusal)))) => {
                render_pipeline_compile_refusal(refusal, &args.id, &args.stage)
            }
            _ => "PRESENTATION FAILURE: invalid Pipeline compile response".to_owned(),
        },
        Err(_) => "PRESENTATION FAILURE: invalid Pipeline compile response".to_owned(),
    };
    emit_human(&rendered, semantic_exit)
}

fn emit_pipeline_resolve(
    response: UseCaseResponseV1,
    machine_json: bool,
    limits: &ContractLimits,
) -> ExitCode {
    let semantic_exit = exit_policy::terminal(response.status());
    if machine_json {
        let bytes = InvocationResponseV1::use_case(response, limits)
            .map_err(|_| ())
            .and_then(|value| doctor_rendering::render_invocation(&value).map_err(|_| ()));
        return match bytes {
            Ok(bytes) => exit_policy::emit_stdout(&bytes, semantic_exit),
            Err(()) => emit_transport_failure(
                TransportFailureKind::Serialization,
                "transport.serialization_failed",
                limits,
            ),
        };
    }
    let rendered = match response.pipeline_route_resolve_and_persist_outcome(limits) {
        Ok(Some(Ok(outcome))) => render_pipeline_resolve_output(
            &outcome.pipeline_id,
            &outcome.state,
            &outcome.effective_run,
            &outcome.stages,
        ),
        Ok(Some(Err(failure))) => render_pipeline_resolve_execution_failure(failure),
        _ => "PRESENTATION FAILURE: invalid Pipeline resolve response".to_owned(),
    };
    emit_human(&rendered, semantic_exit)
}

fn emit_operation_invocation(
    response: OperationResponseV1,
    semantic_exit: ExitCode,
    limits: &ContractLimits,
) -> ExitCode {
    let bytes = InvocationResponseV1::operation_response(response, limits)
        .map_err(|_| ())
        .and_then(|value| doctor_rendering::render_invocation(&value).map_err(|_| ()));
    match bytes {
        Ok(bytes) => exit_policy::emit_stdout(&bytes, semantic_exit),
        Err(()) => emit_transport_failure(
            TransportFailureKind::Serialization,
            "transport.serialization_failed",
            limits,
        ),
    }
}

fn pipeline_root_failure(machine_json: bool, detail: &str, limits: &ContractLimits) -> ExitCode {
    if machine_json {
        emit_transport_failure(
            TransportFailureKind::Adapter,
            "transport.repository_root_unavailable",
            limits,
        )
    } else {
        emit_human(
            &format!("REFUSED: failed to determine repo root: {detail}"),
            exit_policy::failure(),
        )
    }
}

fn pipeline_adapter_failure(machine_json: bool, limits: &ContractLimits) -> ExitCode {
    if machine_json {
        emit_transport_failure(
            TransportFailureKind::Adapter,
            "transport.pipeline_adapter_failed",
            limits,
        )
    } else {
        emit_human("REFUSED: pipeline adapter error", exit_policy::failure())
    }
}

fn emit_transport_failure(
    kind: TransportFailureKind,
    reason_code: &str,
    limits: &ContractLimits,
) -> ExitCode {
    let bytes = TransportFailureV1::adapter_error(kind, reason_code, limits)
        .and_then(|failure| InvocationResponseV1::transport_failure(failure, limits))
        .map_err(|_| ())
        .and_then(|value| doctor_rendering::render_invocation(&value).map_err(|_| ()));
    match bytes {
        Ok(bytes) => exit_policy::emit_stdout(&bytes, exit_policy::failure()),
        Err(()) => {
            eprintln!("handbook: failed to prepare transport-failure response");
            exit_policy::failure()
        }
    }
}

fn emit_human(rendered: &str, semantic_exit: ExitCode) -> ExitCode {
    exit_policy::emit_stdout(format!("{rendered}\n").as_bytes(), semantic_exit)
}

fn pipeline_capture(args: PipelineCaptureArgs, machine_json: bool) -> ExitCode {
    let limits = ContractLimits::packet0();
    let cwd = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(err) => {
            return pipeline_root_failure(machine_json, &err.to_string(), &limits);
        }
    };
    let repo_root = discover_managed_repo_root(&cwd);

    match args.command {
        Some(PipelineCaptureCommand::Apply(apply_args)) => {
            let idempotency_key = pipeline_cli_idempotency_key(
                "pipeline.capture.apply@1.0.0",
                &apply_args.capture_id,
            );
            let request = match pipeline_capture_apply_operation_request(
                None,
                &apply_args.capture_id,
                &idempotency_key,
            ) {
                Ok(request) => request,
                Err(_) => return pipeline_adapter_failure(machine_json, &limits),
            };
            let response = match handbook_sdk::HandbookSdkV1::open(repo_root)
                .execute_pipeline_operation(&request)
            {
                Ok(response) => response,
                Err(_) => return pipeline_adapter_failure(machine_json, &limits),
            };
            emit_pipeline_capture_apply(response, None, None, machine_json, &limits)
        }
        None => {
            let Some(pipeline_id) = args.id.as_deref() else {
                if machine_json {
                    return emit_transport_failure(
                        TransportFailureKind::Adapter,
                        "transport.pipeline_capture_id_required",
                        &limits,
                    );
                }
                return emit_human(
                    "REFUSED: `pipeline capture` requires --id",
                    exit_policy::failure(),
                );
            };
            let Some(stage_id) = args.stage.as_deref() else {
                if machine_json {
                    return emit_transport_failure(
                        TransportFailureKind::Adapter,
                        "transport.pipeline_capture_stage_required",
                        &limits,
                    );
                }
                return emit_human(
                    "REFUSED: `pipeline capture` requires --stage",
                    exit_policy::failure(),
                );
            };
            let stdin = match read_stdin() {
                Ok(value) => value,
                Err(err) => {
                    if machine_json {
                        return emit_transport_failure(
                            TransportFailureKind::Adapter,
                            "transport.pipeline_capture_input_unavailable",
                            &limits,
                        );
                    }
                    return emit_human(
                        &format!("REFUSED: failed to read capture input from stdin: {err}"),
                        exit_policy::failure(),
                    );
                }
            };
            if args.preview {
                let request = match pipeline_capture_plan_operation_request(
                    None,
                    pipeline_id,
                    stage_id,
                    &stdin,
                ) {
                    Ok(request) => request,
                    Err(_) => return pipeline_adapter_failure(machine_json, &limits),
                };
                let response = match handbook_sdk::HandbookSdkV1::open(repo_root)
                    .execute_pipeline_operation(&request)
                {
                    Ok(response) => response,
                    Err(_) => return pipeline_adapter_failure(machine_json, &limits),
                };
                emit_pipeline_capture_plan(
                    response,
                    Some(pipeline_id),
                    Some(stage_id),
                    machine_json,
                    &limits,
                )
            } else {
                let key_material = format!("{pipeline_id}\0{stage_id}\0{stdin}");
                let idempotency_key =
                    pipeline_cli_idempotency_key("pipeline.capture.apply@1.0.0", &key_material);
                let request = match pipeline_capture_direct_operation_request(
                    None,
                    pipeline_id,
                    stage_id,
                    &stdin,
                    &idempotency_key,
                ) {
                    Ok(request) => request,
                    Err(_) => return pipeline_adapter_failure(machine_json, &limits),
                };
                let response = match handbook_sdk::HandbookSdkV1::open(repo_root)
                    .execute_pipeline_operation(&request)
                {
                    Ok(response) => response,
                    Err(_) => return pipeline_adapter_failure(machine_json, &limits),
                };
                emit_pipeline_capture_apply(
                    response,
                    Some(pipeline_id),
                    Some(stage_id),
                    machine_json,
                    &limits,
                )
            }
        }
    }
}

fn emit_pipeline_capture_plan(
    response: OperationResponseV1,
    requested_pipeline_id: Option<&str>,
    requested_stage_id: Option<&str>,
    machine_json: bool,
    limits: &ContractLimits,
) -> ExitCode {
    let semantic_exit = exit_policy::terminal(response.terminal().status());
    if machine_json {
        return emit_operation_invocation(response, semantic_exit, limits);
    }
    let rendered = match pipeline_capture_plan_from_response(&response) {
        Ok(Some(preview)) => render_pipeline_capture_preview(&preview),
        Ok(None) => match pipeline_capture_refusal_from_response(&response) {
            Ok(Some(refusal)) => {
                render_pipeline_capture_refusal(&refusal, requested_pipeline_id, requested_stage_id)
            }
            _ => "PRESENTATION FAILURE: invalid Pipeline capture-plan response".to_owned(),
        },
        Err(_) => "PRESENTATION FAILURE: invalid Pipeline capture-plan response".to_owned(),
    };
    emit_human(&rendered, semantic_exit)
}

fn emit_pipeline_capture_apply(
    response: OperationResponseV1,
    requested_pipeline_id: Option<&str>,
    requested_stage_id: Option<&str>,
    machine_json: bool,
    limits: &ContractLimits,
) -> ExitCode {
    let semantic_exit = exit_policy::terminal(response.terminal().status());
    if machine_json {
        return emit_operation_invocation(response, semantic_exit, limits);
    }
    let rendered = match pipeline_capture_apply_from_response(&response) {
        Ok(Some(result)) => render_pipeline_capture_apply_result(&result),
        Ok(None) => match pipeline_capture_refusal_from_response(&response) {
            Ok(Some(refusal)) => {
                render_pipeline_capture_refusal(&refusal, requested_pipeline_id, requested_stage_id)
            }
            _ => "PRESENTATION FAILURE: invalid Pipeline capture-apply response".to_owned(),
        },
        Err(_) => "PRESENTATION FAILURE: invalid Pipeline capture-apply response".to_owned(),
    };
    emit_human(&rendered, semantic_exit)
}

fn pipeline_handoff(args: PipelineHandoffArgs, machine_json: bool) -> ExitCode {
    let limits = ContractLimits::packet0();
    let cwd = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(err) => {
            return pipeline_root_failure(machine_json, &err.to_string(), &limits);
        }
    };
    let repo_root = discover_managed_repo_root(&cwd);

    match args.command {
        PipelineHandoffCommand::Emit(emit_args) => {
            let supported_handoff_command = pipeline_help::render_supported_handoff_emit_command(
                "pipeline.foundation_inputs",
                "feature-slice-decomposer",
            );
            let key_material = format!(
                "{}\0{}\0{}\0{}",
                emit_args.id, emit_args.consumer, supported_handoff_command, RELEASE_VERSION
            );
            let idempotency_key =
                pipeline_cli_idempotency_key("pipeline.handoff.emit@1.0.0", &key_material);
            let request = match pipeline_handoff_emit_operation_request(
                None,
                &emit_args.id,
                &emit_args.consumer,
                &supported_handoff_command,
                RELEASE_VERSION,
                &idempotency_key,
            ) {
                Ok(request) => request,
                Err(_) => return pipeline_adapter_failure(machine_json, &limits),
            };
            let response = match handbook_sdk::HandbookSdkV1::open(repo_root)
                .execute_pipeline_operation(&request)
            {
                Ok(response) => response,
                Err(_) => return pipeline_adapter_failure(machine_json, &limits),
            };
            emit_pipeline_handoff(response, &supported_handoff_command, machine_json, &limits)
        }
    }
}

fn emit_pipeline_handoff(
    response: OperationResponseV1,
    supported_handoff_command: &str,
    machine_json: bool,
    limits: &ContractLimits,
) -> ExitCode {
    let semantic_exit = exit_policy::terminal(response.terminal().status());
    if machine_json {
        return emit_operation_invocation(response, semantic_exit, limits);
    }
    let rendered = match pipeline_handoff_emit_from_response(&response) {
        Ok(Some(result)) => render_pipeline_handoff_emit_result(&result),
        Ok(None) => match pipeline_handoff_refusal_from_response(&response) {
            Ok(Some(refusal)) => {
                render_pipeline_handoff_refusal(&refusal, supported_handoff_command)
            }
            _ => "PRESENTATION FAILURE: invalid Pipeline handoff response".to_owned(),
        },
        Err(_) => "PRESENTATION FAILURE: invalid Pipeline handoff response".to_owned(),
    };
    emit_human(&rendered, semantic_exit)
}

fn render_pipeline_handoff_refusal(
    refusal: &pipeline_sdk::PipelineHandoffRefusalView,
    supported_handoff_command: &str,
) -> String {
    let mut refusal = refusal.clone();
    if refusal.classification
        == pipeline_sdk::PipelineHandoffRefusalClassificationView::UnsupportedTarget
        && refusal.recovery == "retry with the supported handoff emit command"
    {
        refusal.recovery = format!("retry with `{supported_handoff_command}`");
    }
    render_pipeline_handoff_refusal_output(&refusal)
}

fn pipeline_state_set(args: PipelineStateSetArgs, machine_json: bool) -> ExitCode {
    let limits = ContractLimits::packet0();
    let cwd = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(err) => {
            return pipeline_root_failure(machine_json, &err.to_string(), &limits);
        }
    };
    let repo_root = discover_managed_repo_root(&cwd);

    let mutation = match parse_route_state_mutation(&args) {
        Ok(mutation) => mutation,
        Err(err) => {
            if machine_json {
                return emit_transport_failure(
                    TransportFailureKind::Adapter,
                    "transport.pipeline_state_request_invalid",
                    &limits,
                );
            }
            return emit_human(&format!("REFUSED: {err}"), exit_policy::failure());
        }
    };
    let expected_revision = match args.expected_revision {
        Some(revision) => revision,
        None => match pipeline_sdk::current_pipeline_state_revision(&repo_root, &args.id) {
            Ok(revision) => revision,
            Err(refusal) => {
                if machine_json {
                    return emit_transport_failure(
                        TransportFailureKind::Adapter,
                        "transport.pipeline_state_basis_unavailable",
                        &limits,
                    );
                }
                return emit_human(
                    &render_pipeline_state_update_refusal(refusal),
                    exit_policy::failure(),
                );
            }
        },
    };
    let key_material = format!("{}\0{mutation:?}\0{expected_revision}", args.id);
    let idempotency_key = pipeline_cli_idempotency_key("pipeline.state.apply@1.0.0", &key_material);
    let request = match pipeline_state_apply_operation_request(
        None,
        &args.id,
        &mutation,
        expected_revision,
        &idempotency_key,
    ) {
        Ok(request) => request,
        Err(_) => return pipeline_adapter_failure(machine_json, &limits),
    };
    let response =
        match handbook_sdk::HandbookSdkV1::open(&repo_root).execute_pipeline_operation(&request) {
            Ok(response) => response,
            Err(_) => return pipeline_adapter_failure(machine_json, &limits),
        };
    emit_pipeline_state(response, &args.id, machine_json, &limits)
}

fn emit_pipeline_state(
    response: OperationResponseV1,
    requested_pipeline_id: &str,
    machine_json: bool,
    limits: &ContractLimits,
) -> ExitCode {
    let semantic_exit = exit_policy::terminal(response.terminal().status());
    if machine_json {
        return emit_operation_invocation(response, semantic_exit, limits);
    }
    let rendered = match pipeline_state_apply_from_response(&response) {
        Ok(Some(mut outcome)) => {
            if let pipeline_sdk::PipelineStateUpdateResult::Applied(state) = &mut outcome.result {
                state.run.repository_root = Some("${repo_root}".to_owned());
            }
            render_pipeline_state_set_output(&outcome.pipeline_id, outcome.result)
        }
        Ok(None) => match pipeline_state_refusal_from_response(&response) {
            Ok(Some(PipelineStateProblemProjection::Outcome(outcome))) => {
                render_pipeline_state_set_output(&outcome.pipeline_id, outcome.result)
            }
            Ok(Some(PipelineStateProblemProjection::Failure(failure))) => {
                render_pipeline_state_update_refusal(failure)
            }
            Ok(None) => match pipeline_operation_problem_details(&response) {
                Ok(Some((_, Some(selector), _))) => render_pipeline_selector_refusal(selector),
                Ok(Some((detail, None, _))) => render_pipeline_state_set_output(
                    requested_pipeline_id,
                    pipeline_sdk::PipelineStateUpdateResult::Refused(detail),
                ),
                _ => "PRESENTATION FAILURE: invalid Pipeline state response".to_owned(),
            },
            Err(_) => "PRESENTATION FAILURE: invalid Pipeline state response".to_owned(),
        },
        Err(_) => "PRESENTATION FAILURE: invalid Pipeline state response".to_owned(),
    };
    emit_human(&rendered, semantic_exit)
}

fn pipeline_cli_idempotency_key(operation_ref: &str, material: &str) -> String {
    let fingerprint = sha256_fingerprint(
        format!("handbook.pipeline-cli-idempotency-v1\0{operation_ref}\0{material}").as_bytes(),
    );
    format!("cli-{}", fingerprint.as_str().trim_start_matches("sha256:"))
}

fn render_pipeline_selector_refusal(err: pipeline_sdk::PipelineSelectorRefusal) -> String {
    match err {
        pipeline_sdk::PipelineSelectorRefusal::Ambiguous { selector, matches } => {
            format!(
                "REFUSED: ambiguous selector `{selector}` matched multiple canonical ids: {}\nNEXT SAFE ACTION: use the full canonical id or rename the conflicting ids",
                matches.join(", ")
            )
        }
        pipeline_sdk::PipelineSelectorRefusal::Unknown { selector } => format!(
            "REFUSED: unknown pipeline selector `{selector}`; use a canonical id or `pipeline list` to inspect available inventory\nNEXT SAFE ACTION: run `pipeline list` and retry with the full canonical id"
        ),
        pipeline_sdk::PipelineSelectorRefusal::Unsupported { selector, reason } => {
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
    refusal: pipeline_sdk::PipelineCompileRefusalView,
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
    classification: pipeline_sdk::PipelineCompileRefusalClassificationView,
) -> &'static str {
    match classification {
        pipeline_sdk::PipelineCompileRefusalClassificationView::UnsupportedTarget => {
            "unsupported_target"
        }
        pipeline_sdk::PipelineCompileRefusalClassificationView::InvalidDefinition => {
            "invalid_definition"
        }
        pipeline_sdk::PipelineCompileRefusalClassificationView::InvalidState => "invalid_state",
        pipeline_sdk::PipelineCompileRefusalClassificationView::MissingRouteBasis => {
            "missing_route_basis"
        }
        pipeline_sdk::PipelineCompileRefusalClassificationView::MalformedRouteBasis => {
            "malformed_route_basis"
        }
        pipeline_sdk::PipelineCompileRefusalClassificationView::StaleRouteBasis => {
            "stale_route_basis"
        }
        pipeline_sdk::PipelineCompileRefusalClassificationView::InactiveStage => "inactive_stage",
        pipeline_sdk::PipelineCompileRefusalClassificationView::MissingRequiredInput => {
            "missing_required_input"
        }
        pipeline_sdk::PipelineCompileRefusalClassificationView::EmptyRequiredInput => {
            "empty_required_input"
        }
    }
}

fn render_pipeline_compile_next_safe_action(
    refusal: &pipeline_sdk::PipelineCompileRefusalView,
    pipeline_id: &str,
    stage_id: &str,
) -> String {
    match refusal.classification {
        pipeline_sdk::PipelineCompileRefusalClassificationView::UnsupportedTarget => {
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
        pipeline_sdk::PipelineCompileRefusalClassificationView::MissingRouteBasis
        | pipeline_sdk::PipelineCompileRefusalClassificationView::MalformedRouteBasis
        | pipeline_sdk::PipelineCompileRefusalClassificationView::StaleRouteBasis => format!(
            "run `handbook pipeline resolve --id {pipeline_id}` and then retry `handbook pipeline compile --id {pipeline_id} --stage {stage_id}`"
        ),
        pipeline_sdk::PipelineCompileRefusalClassificationView::InactiveStage => format!(
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
) -> Result<pipeline_sdk::PipelineStateMutation, String> {
    match (&args.var, &args.field) {
        (Some(value), None) => parse_route_state_var_assignment(value),
        (None, Some(value)) => parse_route_state_field_assignment(value),
        (Some(_), Some(_)) => Err("use exactly one of --var or --field".to_string()),
        (None, None) => Err("one of --var or --field is required".to_string()),
    }
}

fn parse_route_state_var_assignment(
    value: &str,
) -> Result<pipeline_sdk::PipelineStateMutation, String> {
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

    Ok(pipeline_sdk::PipelineStateMutation::RoutingVariable {
        variable: name.to_string(),
        value: parsed_value,
    })
}

fn parse_route_state_field_assignment(
    value: &str,
) -> Result<pipeline_sdk::PipelineStateMutation, String> {
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
        "run.runner" => Ok(pipeline_sdk::PipelineStateMutation::Runner {
            value: raw_value.to_string(),
        }),
        "run.profile" => Ok(pipeline_sdk::PipelineStateMutation::Profile {
            value: raw_value.to_string(),
        }),
        "refs.charter_ref" => Ok(pipeline_sdk::PipelineStateMutation::CharterReference {
            value: raw_value.to_string(),
        }),
        "refs.project_context_ref" => {
            Ok(pipeline_sdk::PipelineStateMutation::ProjectContextReference {
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
    state: &pipeline_sdk::PipelineRouteStateView,
    effective_run: &pipeline_sdk::PipelineRouteRunView,
    stages: &[pipeline_sdk::PipelineResolvedStageView],
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
    render_optional_route_basis_field(&mut out, "charter_ref", state.charter_ref.as_deref());
    render_optional_route_basis_field(
        &mut out,
        "project_context_ref",
        state.project_context_ref.as_deref(),
    );
    out.push_str("  run:\n");
    render_optional_route_basis_field(&mut out, "runner", effective_run.runner.as_deref());
    render_optional_route_basis_field(&mut out, "profile", effective_run.profile.as_deref());
    render_optional_route_basis_field(
        &mut out,
        "repo_root",
        effective_run.repository_root.as_deref(),
    );
    out.push_str("ROUTE:\n");

    for (index, stage) in stages.iter().enumerate() {
        out.push_str(&format!(
            "  {}. {} | {}\n",
            index + 1,
            stage.stage_id,
            render_route_stage_status(stage.status)
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

fn render_route_stage_reason(reason: &pipeline_sdk::PipelineRouteStageReason) -> String {
    match reason {
        pipeline_sdk::PipelineRouteStageReason::SkippedActivationFalse {
            unsatisfied_variables,
            ..
        } => format!(
            "activation evaluated false for variables: {}",
            unsatisfied_variables.join(", ")
        ),
        pipeline_sdk::PipelineRouteStageReason::NextMissingRouteVariables {
            missing_variables,
            ..
        } => format!("missing route variables: {}", missing_variables.join(", ")),
        pipeline_sdk::PipelineRouteStageReason::BlockedByUnresolvedStage {
            upstream_stage_id,
            upstream_status,
        } => format!(
            "blocked by unresolved stage {} ({})",
            upstream_stage_id,
            render_route_stage_status(*upstream_status)
        ),
    }
}

fn render_pipeline_state_set_output(
    pipeline_id: &str,
    outcome: pipeline_sdk::PipelineStateUpdateResult,
) -> String {
    let mut out = String::new();
    match outcome {
        pipeline_sdk::PipelineStateUpdateResult::Applied(state) => {
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
            render_optional_state_field(&mut out, "charter_ref", state.charter_ref.as_deref());
            render_optional_state_field(
                &mut out,
                "project_context_ref",
                state.project_context_ref.as_deref(),
            );
            out.push_str("RUN:\n");
            render_optional_state_field(&mut out, "runner", state.run.runner.as_deref());
            render_optional_state_field(&mut out, "profile", state.run.profile.as_deref());
            render_optional_state_field(
                &mut out,
                "repo_root",
                state.run.repository_root.as_deref(),
            );
        }
        pipeline_sdk::PipelineStateUpdateResult::Refused(refusal) => {
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

fn render_pipeline_list(catalog: &pipeline_sdk::PipelineCatalogOutcome) -> String {
    let mut out = String::new();
    out.push_str("PIPELINE INVENTORY\n");
    out.push_str(&format!("PIPELINE COUNT: {}\n", catalog.pipeline_count));
    out.push_str(&format!("STAGE COUNT: {}\n", catalog.stage_count));
    for pipeline in &catalog.pipelines {
        out.push_str(&format!("\nPIPELINE: {}\n", pipeline.id));
        out.push_str(&format!("TITLE: {}\n", pipeline.title));
        out.push_str(&format!("SOURCE: {}\n", pipeline.source_path.display()));
        out.push_str(&format!("STAGES: {}\n", pipeline.stage_count));
    }
    out.trim_end().to_string()
}

fn render_pipeline_show(selection: &pipeline_sdk::PipelineShowOutcome) -> String {
    match selection {
        pipeline_sdk::PipelineShowOutcome::Pipeline(pipeline) => {
            let mut out = String::new();
            out.push_str(&format!("PIPELINE: {}\n", pipeline.id));
            out.push_str(&format!("TITLE: {}\n", pipeline.title));
            out.push_str(&format!("DESCRIPTION: {}\n", pipeline.description.trim()));
            out.push_str(&format!("SOURCE: {}\n", pipeline.source_path.display()));
            out.push_str("DEFAULTS:\n");
            out.push_str(&format!("  runner: {}\n", pipeline.default_runner));
            out.push_str(&format!("  profile: {}\n", pipeline.default_profile));
            out.push_str(&format!(
                "  enable_complexity: {}\n",
                pipeline.default_enable_complexity
            ));
            out.push_str("STAGES:\n");
            for (index, stage) in pipeline.stages.iter().enumerate() {
                out.push_str(&format!(
                    "  {}. {} | {} | {}\n",
                    index + 1,
                    stage.id,
                    stage.source_path.display(),
                    stage.title
                ));
                if let Some(work_level) = &stage.work_level {
                    out.push_str(&format!("     work_level: {work_level}\n"));
                }
                if let Some(sets) = &stage.sets {
                    out.push_str(&format!("     sets: [{}]\n", sets.join(", ")));
                }
                if let Some(activation) = &stage.activation {
                    let operator = match activation.operator {
                        pipeline_sdk::PipelineActivationOperator::Any => "any",
                        pipeline_sdk::PipelineActivationOperator::All => "all",
                    };
                    let clauses = activation
                        .clauses
                        .iter()
                        .map(|clause| format!("variables.{} == {}", clause.variable, clause.value))
                        .collect::<Vec<_>>()
                        .join(", ");
                    out.push_str(&format!(
                        "     activation: activation.when.{operator} [{clauses}]\n"
                    ));
                }
            }
            out.trim_end().to_string()
        }
        pipeline_sdk::PipelineShowOutcome::Stage(stage) => {
            let mut out = String::new();
            out.push_str(&format!("STAGE: {}\n", stage.id));
            out.push_str(&format!("KIND: {}\n", stage.kind));
            out.push_str(&format!("VERSION: {}\n", stage.version));
            out.push_str(&format!("TITLE: {}\n", stage.title));
            out.push_str(&format!("DESCRIPTION: {}\n", stage.description.trim()));
            if let Some(work_level) = &stage.work_level {
                out.push_str(&format!("WORK_LEVEL: {work_level}\n"));
            }
            out.push_str(&format!("SOURCE: {}\n", stage.source_path.display()));
            out.push_str("PIPELINES:\n");
            for pipeline_id in &stage.pipeline_ids {
                out.push_str(&format!("  - {pipeline_id}\n"));
            }
            out.trim_end().to_string()
        }
    }
}

fn render_pipeline_resolve_execution_failure(
    failure: pipeline_sdk::PipelineResolveExecutionFailure,
) -> String {
    match failure {
        pipeline_sdk::PipelineResolveExecutionFailure::Selection(failure) => {
            render_selected_definition_refusal(failure)
        }
        pipeline_sdk::PipelineResolveExecutionFailure::RouteStateRead(reason) => {
            format!("REFUSED: {reason}")
        }
        pipeline_sdk::PipelineResolveExecutionFailure::RouteVariables(reason) => {
            format!("REFUSED: malformed route state variables: {reason}")
        }
        pipeline_sdk::PipelineResolveExecutionFailure::RouteResolution(reason) => {
            format!("REFUSED: route resolution error: {reason}")
        }
        pipeline_sdk::PipelineResolveExecutionFailure::RouteBasisBuild(reason) => {
            format!("REFUSED: route basis build error: {reason}")
        }
        pipeline_sdk::PipelineResolveExecutionFailure::RouteBasisPersistenceRefused(refusal) => {
            let reason = match refusal {
                pipeline_sdk::PipelineRouteBasisPersistenceRefusal::MalformedState { reason } => {
                    format!("malformed route state: {reason}")
                }
                pipeline_sdk::PipelineRouteBasisPersistenceRefusal::RevisionConflict {
                    expected_revision,
                    actual_revision,
                } => format!(
                    "revision conflict while persisting route_basis: expected {expected_revision}, found {actual_revision}"
                ),
            };
            format!("REFUSED: route basis persistence refused: {reason}")
        }
        pipeline_sdk::PipelineResolveExecutionFailure::RouteBasisPersistence(reason) => {
            format!("REFUSED: route basis persistence error: {reason}")
        }
    }
}

fn render_selected_definition_refusal(
    refusal: pipeline_sdk::PipelineSelectedDefinitionFailure,
) -> String {
    match refusal {
        pipeline_sdk::PipelineSelectedDefinitionFailure::Catalog(reason) => {
            format!("REFUSED: pipeline catalog error: {reason}")
        }
        pipeline_sdk::PipelineSelectedDefinitionFailure::Selector(refusal) => {
            render_pipeline_selector_refusal(refusal)
        }
        pipeline_sdk::PipelineSelectedDefinitionFailure::Definition(reason) => {
            format!("REFUSED: pipeline definition error: {reason}")
        }
    }
}

fn render_pipeline_state_update_refusal(
    refusal: pipeline_sdk::PipelineStateUpdateFailure,
) -> String {
    match refusal {
        pipeline_sdk::PipelineStateUpdateFailure::Selection(refusal) => {
            render_selected_definition_refusal(refusal)
        }
        pipeline_sdk::PipelineStateUpdateFailure::RouteStateRead(reason) => {
            format!("REFUSED: {reason}")
        }
        pipeline_sdk::PipelineStateUpdateFailure::Mutation(reason) => {
            format!("REFUSED: route state mutation error: {reason}")
        }
    }
}

fn render_route_stage_status(status: pipeline_sdk::PipelineRouteStageStatus) -> &'static str {
    match status {
        pipeline_sdk::PipelineRouteStageStatus::Active => "active",
        pipeline_sdk::PipelineRouteStageStatus::Skipped => "skipped",
        pipeline_sdk::PipelineRouteStageStatus::Blocked => "blocked",
        pipeline_sdk::PipelineRouteStageStatus::Next => "next",
    }
}

fn render_pipeline_compile_payload(result: &pipeline_sdk::PipelineCompileOutcome) -> String {
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
    render_compile_document_section(
        &mut out,
        "Selected Runner",
        result.documents.iter().filter(|document| {
            document.kind == pipeline_sdk::PipelineCompileDocumentKindView::Runner
        }),
    );
    render_compile_document_section(
        &mut out,
        "Selected Profile",
        result.documents.iter().filter(|document| {
            document.kind == pipeline_sdk::PipelineCompileDocumentKindView::Profile
        }),
    );
    render_compile_document_section(
        &mut out,
        "Includes",
        result.documents.iter().filter(|document| {
            document.kind == pipeline_sdk::PipelineCompileDocumentKindView::Include
        }),
    );
    render_compile_document_section(
        &mut out,
        "Library Inputs",
        result.documents.iter().filter(|document| {
            document.kind == pipeline_sdk::PipelineCompileDocumentKindView::Library
        }),
    );
    render_compile_document_section(
        &mut out,
        "Artifact Inputs",
        result.documents.iter().filter(|document| {
            document.kind == pipeline_sdk::PipelineCompileDocumentKindView::Artifact
        }),
    );
    out.push_str("\n## Outputs\n");
    render_compile_output_section(
        &mut out,
        "Artifacts",
        result
            .outputs
            .iter()
            .filter(|output| output.kind == pipeline_sdk::PipelineCompileOutputKindView::Artifact),
    );
    render_compile_output_section(
        &mut out,
        "Repo Files",
        result.outputs.iter().filter(|output| {
            output.kind == pipeline_sdk::PipelineCompileOutputKindView::RepositoryFile
        }),
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
    normalize_pipeline_rendered_output(&out)
}

fn render_pipeline_compile_explain(result: &pipeline_sdk::PipelineCompileOutcome) -> String {
    let mut out = String::new();
    out.push_str("OUTCOME: COMPILED\nTARGET:\n");
    out.push_str(&format!(
        "  pipeline = {}\n  stage = {}\n  stage_file = {}\n  work_level = {}\n",
        result.target.pipeline_id,
        result.target.stage_id,
        result.target.stage_file,
        result.target.work_level
    ));
    out.push_str("ROUTE BASIS:\n");
    out.push_str(&format!("  schema_version = {}\n  state_revision = {}\n  pipeline_file = {}\n  pipeline_file_sha256 = {}\n", result.basis.schema_version, result.basis.state_revision, result.basis.pipeline_file, result.basis.pipeline_file_sha256));
    out.push_str("  routing:\n");
    if result.basis.routing.is_empty() {
        out.push_str("    <empty>\n");
    } else {
        for (name, value) in &result.basis.routing {
            out.push_str(&format!("    {name} = {value}\n"));
        }
    }
    out.push_str("  refs:\n");
    render_optional_route_basis_field(&mut out, "charter_ref", result.basis.charter_ref.as_deref());
    render_optional_route_basis_field(
        &mut out,
        "project_context_ref",
        result.basis.project_context_ref.as_deref(),
    );
    out.push_str("  run:\n");
    render_optional_route_basis_field(&mut out, "runner", result.basis.run.runner.as_deref());
    render_optional_route_basis_field(&mut out, "profile", result.basis.run.profile.as_deref());
    render_optional_route_basis_field(
        &mut out,
        "repo_root",
        result.basis.run.repository_root.as_deref(),
    );
    out.push_str("  runner:\n");
    out.push_str(&format!(
        "    id = {}\n    file = {}\n    file_sha256 = {}\n",
        result.basis.runner.id, result.basis.runner.file, result.basis.runner.file_sha256
    ));
    out.push_str("  profile:\n");
    out.push_str(&format!("    id = {}\n    profile.yaml.sha256 = {}\n    commands.yaml.sha256 = {}\n    conventions.md.sha256 = {}\n", result.basis.profile.id, result.basis.profile.profile_yaml_sha256, result.basis.profile.commands_yaml_sha256, result.basis.profile.conventions_md_sha256));
    out.push_str("ROUTE SNAPSHOT:\n");
    for (index, stage) in result.basis.route.iter().enumerate() {
        out.push_str(&format!(
            "  {}. {} | {} | {}\n",
            index + 1,
            stage.stage_id,
            render_route_stage_status(stage.status),
            stage.file
        ));
        if let Some(reason) = &stage.reason {
            out.push_str(&format!(
                "     REASON: {}\n",
                render_route_stage_reason(reason)
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
            render_compile_document_kind(document.kind),
            document.path,
            document.required,
            render_compile_document_status(document.status)
        ));
    }
    out.push_str("OUTPUTS:\n");
    for output in &result.outputs {
        out.push_str(&format!(
            "  {} | {}\n",
            render_compile_output_kind(output.kind),
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
    normalize_pipeline_rendered_output(&out)
}

fn render_compile_document_section<'a>(
    out: &mut String,
    title: &str,
    documents: impl Iterator<Item = &'a pipeline_sdk::PipelineCompileDocumentView>,
) {
    let documents = documents
        .filter(|document| {
            document.status == pipeline_sdk::PipelineCompileDocumentStatusView::Present
        })
        .collect::<Vec<_>>();
    out.push_str(&format!("\n## {title}\n"));
    if documents.is_empty() {
        out.push_str("(none)\n");
        return;
    }
    for document in documents {
        out.push_str(&format!("\n### {}\n", document.path));
        if let Some(content) = &document.content {
            out.push_str(content.trim_end());
            out.push('\n');
        }
    }
}

fn render_compile_output_section<'a>(
    out: &mut String,
    title: &str,
    outputs: impl Iterator<Item = &'a pipeline_sdk::PipelineCompileOutputView>,
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

fn render_compile_document_kind(
    kind: pipeline_sdk::PipelineCompileDocumentKindView,
) -> &'static str {
    match kind {
        pipeline_sdk::PipelineCompileDocumentKindView::Include => "include",
        pipeline_sdk::PipelineCompileDocumentKindView::Runner => "runner",
        pipeline_sdk::PipelineCompileDocumentKindView::Profile => "profile",
        pipeline_sdk::PipelineCompileDocumentKindView::Library => "library",
        pipeline_sdk::PipelineCompileDocumentKindView::Artifact => "artifact",
    }
}
fn render_compile_document_status(
    status: pipeline_sdk::PipelineCompileDocumentStatusView,
) -> &'static str {
    match status {
        pipeline_sdk::PipelineCompileDocumentStatusView::Present => "present",
        pipeline_sdk::PipelineCompileDocumentStatusView::MissingOptional => "missing_optional",
    }
}
fn render_compile_output_kind(kind: pipeline_sdk::PipelineCompileOutputKindView) -> &'static str {
    match kind {
        pipeline_sdk::PipelineCompileOutputKindView::Artifact => "artifact",
        pipeline_sdk::PipelineCompileOutputKindView::RepositoryFile => "repo_file",
    }
}
fn normalize_pipeline_rendered_output(text: &str) -> String {
    let lines = text
        .replace("\r\n", "\n")
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

fn render_pipeline_capture_preview(preview: &pipeline_sdk::PipelineCapturePreviewView) -> String {
    let plan = &preview.plan;
    let mut out = String::new();
    out.push_str("OUTCOME: PREVIEW\n");
    out.push_str(&format!(
        "PIPELINE: {}\nSTAGE: {}\nCAPTURE ID: {}\nROUTE BASIS REVISION: {}\n",
        plan.target.pipeline_id, plan.target.stage_id, plan.capture_id, plan.route_basis_revision
    ));
    out.push_str("WRITE PLAN:\n");
    render_pipeline_capture_write_plan(&mut out, plan);
    out.push_str("POST-CAPTURE STATE UPDATES:\n");
    render_pipeline_capture_state_updates(&mut out, &plan.state_updates);
    out.push_str(&format!(
        "NEXT SAFE ACTION: run `handbook pipeline capture apply --capture-id {}`",
        plan.capture_id
    ));
    out
}

fn render_pipeline_capture_apply_result(result: &pipeline_sdk::PipelineCaptureApplyView) -> String {
    let mut out = String::new();
    out.push_str("OUTCOME: CAPTURED\n");
    out.push_str(&format!(
        "PIPELINE: {}\nSTAGE: {}\n",
        result.plan.target.pipeline_id, result.plan.target.stage_id
    ));
    out.push_str("WRITTEN FILES:\n");
    if result.written_files.is_empty() {
        out.push_str("  <none>\n");
    } else {
        for path in &result.written_files {
            out.push_str(&format!("  - {path}\n"));
        }
    }
    out.push_str("STATE UPDATES:\n");
    render_pipeline_capture_state_updates(&mut out, &result.plan.state_updates);
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

fn render_pipeline_capture_refusal(
    refusal: &pipeline_sdk::PipelineCaptureRefusalView,
    requested_pipeline_id: Option<&str>,
    requested_stage_id: Option<&str>,
) -> String {
    format!(
        "OUTCOME: REFUSED\nPIPELINE: {}\nSTAGE: {}\nREASON: {}: {}\nNEXT SAFE ACTION: {}",
        refusal
            .pipeline_id
            .as_deref()
            .or(requested_pipeline_id.map(str::trim))
            .unwrap_or("<unknown>"),
        refusal
            .stage_id
            .as_deref()
            .or(requested_stage_id.map(str::trim))
            .unwrap_or("<unknown>"),
        render_pipeline_capture_refusal_classification(refusal.classification),
        refusal.summary.trim(),
        refusal.recovery.trim()
    )
}

fn render_pipeline_capture_write_plan(
    out: &mut String,
    plan: &pipeline_sdk::PipelineCapturePlanView,
) {
    if plan.artifact_write_paths.is_empty() && plan.repository_mirror_write_paths.is_empty() {
        out.push_str("  <none>\n");
        return;
    }
    for path in &plan.artifact_write_paths {
        out.push_str(&format!("  - artifact: {path}\n"));
    }
    for path in &plan.repository_mirror_write_paths {
        out.push_str(&format!("  - repo_file: {path}\n"));
    }
}

fn render_pipeline_capture_state_updates(
    out: &mut String,
    updates: &[pipeline_sdk::PipelineCaptureStateUpdateView],
) {
    if updates.is_empty() {
        out.push_str("  <none>\n");
        return;
    }
    for update in updates {
        let value = match &update.value {
            pipeline_sdk::PipelineCaptureStateValue::Boolean(value) => value.to_string(),
            pipeline_sdk::PipelineCaptureStateValue::Text(value) => value.clone(),
        };
        out.push_str(&format!("  - {} = {value}\n", update.field_path));
    }
}

fn render_pipeline_capture_refusal_classification(
    classification: pipeline_sdk::PipelineCaptureRefusalClassificationView,
) -> &'static str {
    match classification {
        pipeline_sdk::PipelineCaptureRefusalClassificationView::UnsupportedTarget => {
            "unsupported_target"
        }
        pipeline_sdk::PipelineCaptureRefusalClassificationView::InvalidDefinition => {
            "invalid_definition"
        }
        pipeline_sdk::PipelineCaptureRefusalClassificationView::InvalidState => "invalid_state",
        pipeline_sdk::PipelineCaptureRefusalClassificationView::MissingRouteBasis => {
            "missing_route_basis"
        }
        pipeline_sdk::PipelineCaptureRefusalClassificationView::MalformedRouteBasis => {
            "malformed_route_basis"
        }
        pipeline_sdk::PipelineCaptureRefusalClassificationView::StaleRouteBasis => {
            "stale_route_basis"
        }
        pipeline_sdk::PipelineCaptureRefusalClassificationView::InactiveStage => "inactive_stage",
        pipeline_sdk::PipelineCaptureRefusalClassificationView::InvalidCaptureInput => {
            "invalid_capture_input"
        }
        pipeline_sdk::PipelineCaptureRefusalClassificationView::InvalidWriteTarget => {
            "invalid_write_target"
        }
        pipeline_sdk::PipelineCaptureRefusalClassificationView::MissingCaptureId => {
            "missing_capture_id"
        }
        pipeline_sdk::PipelineCaptureRefusalClassificationView::TamperedCaptureCache => {
            "tampered_capture_cache"
        }
        pipeline_sdk::PipelineCaptureRefusalClassificationView::RevisionConflict => {
            "revision_conflict"
        }
        pipeline_sdk::PipelineCaptureRefusalClassificationView::WriteFailure => "write_failure",
        pipeline_sdk::PipelineCaptureRefusalClassificationView::StatePersistenceFailure => {
            "state_persistence_failure"
        }
        pipeline_sdk::PipelineCaptureRefusalClassificationView::CacheFailure => "cache_failure",
    }
}

fn render_pipeline_handoff_emit_result(result: &pipeline_sdk::PipelineHandoffOutcome) -> String {
    let mut out = String::new();
    out.push_str("OUTCOME: EMITTED\n");
    out.push_str(&format!(
        "PIPELINE: {}\nCONSUMER: {}\nFEATURE ID: {}\nBUNDLE ROOT: {}\n",
        result.pipeline_id, result.consumer_id, result.feature_id, result.bundle_root
    ));
    out.push_str("WRITTEN FILES:\n");
    for path in &result.written_files {
        out.push_str(&format!("  - {path}\n"));
    }
    out.push_str("REPO REREAD FALLBACK: disabled");
    out
}

fn render_pipeline_handoff_refusal_output(
    refusal: &pipeline_sdk::PipelineHandoffRefusalView,
) -> String {
    format!(
        "OUTCOME: REFUSED\nPIPELINE: {}\nCONSUMER: {}\nREASON: {}: {}\nNEXT SAFE ACTION: {}",
        refusal.pipeline_id.as_deref().unwrap_or("<unknown>"),
        refusal.consumer_id.as_deref().unwrap_or("<unknown>"),
        render_pipeline_handoff_refusal_classification(refusal.classification),
        refusal.summary.trim(),
        refusal.recovery.trim()
    )
}

fn render_pipeline_handoff_refusal_classification(
    classification: pipeline_sdk::PipelineHandoffRefusalClassificationView,
) -> &'static str {
    match classification {
        pipeline_sdk::PipelineHandoffRefusalClassificationView::UnsupportedTarget => {
            "unsupported_target"
        }
        pipeline_sdk::PipelineHandoffRefusalClassificationView::InvalidState => "invalid_state",
        pipeline_sdk::PipelineHandoffRefusalClassificationView::MissingRequiredInput => {
            "missing_required_input"
        }
        pipeline_sdk::PipelineHandoffRefusalClassificationView::InvalidProvenance => {
            "invalid_provenance"
        }
        pipeline_sdk::PipelineHandoffRefusalClassificationView::WriteFailure => "write_failure",
    }
}
