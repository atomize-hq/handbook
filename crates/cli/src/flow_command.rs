use crate::{
    doctor_rendering, exit_policy, rendering, request_shared,
    shell_shared::discover_managed_repo_root, RequestArgs,
};
use handbook_sdk::contract_membrane::{
    flow_problem_detail, flow_resolution_from_response, flow_resolve_operation_request,
    ContractLimits, InvocationResponseV1, TransportFailureKind, TransportFailureV1,
};
use std::process::ExitCode;

#[derive(Clone, Copy)]
pub(super) enum FlowRenderer {
    Generate,
    Inspect,
}

impl FlowRenderer {
    fn human_prefix(self) -> &'static str {
        match self {
            Self::Generate => "REFUSED",
            Self::Inspect => "BLOCKED",
        }
    }
}

pub(super) fn run(args: RequestArgs, machine_json: bool, renderer: FlowRenderer) -> ExitCode {
    let limits = ContractLimits::packet0();
    let cwd = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(err) => {
            if machine_json {
                return emit_transport_failure(
                    TransportFailureKind::Adapter,
                    "transport.repository_root_unavailable",
                    &limits,
                );
            }
            return emit_human_failure(format!(
                "{}: failed to determine repo root: {err}",
                renderer.human_prefix()
            ));
        }
    };
    let repo_root = discover_managed_repo_root(&cwd);
    let prepared = match request_shared::prepare_request(&args, &repo_root) {
        Ok(request) => request,
        Err(error) => {
            if machine_json {
                let (kind, reason) = match error.kind() {
                    request_shared::RequestPreparationErrorKind::Acquisition => (
                        TransportFailureKind::Acquisition,
                        "transport.flow_input_unavailable",
                    ),
                    request_shared::RequestPreparationErrorKind::Decoding => (
                        TransportFailureKind::Decoding,
                        "transport.flow_input_invalid",
                    ),
                };
                return emit_transport_failure(kind, reason, &limits);
            }
            return emit_human_failure(format!("{}: {error}", renderer.human_prefix()));
        }
    };
    let request = match flow_resolve_operation_request(None, prepared.packet_id.as_str()) {
        Ok(request) => request,
        Err(_) if machine_json => {
            return emit_transport_failure(
                TransportFailureKind::Decoding,
                "transport.flow_request_invalid",
                &limits,
            )
        }
        Err(_) => {
            return emit_human_failure(format!(
                "{}: resolver request is invalid",
                renderer.human_prefix()
            ))
        }
    };
    let response = match handbook_sdk::HandbookSdkV1::open(&prepared.compiler_root)
        .execute_flow_operation(&request)
    {
        Ok(response) => response,
        Err(_) if machine_json => {
            return emit_transport_failure(
                TransportFailureKind::Adapter,
                "transport.flow_adapter_failed",
                &limits,
            )
        }
        Err(error) => {
            return emit_human_failure(format!(
                "{}: resolver error: {error:?}",
                renderer.human_prefix(),
            ))
        }
    };
    let semantic_exit = exit_policy::terminal(response.terminal().status());
    if machine_json {
        let bytes = InvocationResponseV1::operation_response(response, &limits)
            .map_err(|_| ())
            .and_then(|value| doctor_rendering::render_invocation(&value).map_err(|_| ()));
        return match bytes {
            Ok(bytes) => exit_policy::emit_stdout(&bytes, semantic_exit),
            Err(()) => emit_transport_failure(
                TransportFailureKind::Serialization,
                "transport.serialization_failed",
                &limits,
            ),
        };
    }

    match flow_resolution_from_response(&response) {
        Ok(Some(resolution)) => match rendering::prepare_flow_output(resolution) {
            Ok(output) => {
                let rendered = match renderer {
                    FlowRenderer::Generate => output.render_markdown(),
                    FlowRenderer::Inspect => output.render_inspect(),
                };
                let bytes = format!("{rendered}\n");
                exit_policy::emit_stdout(bytes.as_bytes(), semantic_exit)
            }
            Err(error) => emit_human_failure(format!("PRESENTATION FAILURE: {error}")),
        },
        Ok(None) => {
            let detail = flow_problem_detail(&response)
                .ok()
                .flatten()
                .unwrap_or_else(|| "Flow owner failure".to_owned());
            emit_human_failure(format!(
                "{}: resolver error: {detail}",
                renderer.human_prefix()
            ))
        }
        Err(_) => emit_human_failure("PRESENTATION FAILURE: invalid Flow response".to_owned()),
    }
}

fn emit_human_failure(message: String) -> ExitCode {
    let bytes = format!("{message}\n");
    exit_policy::emit_stdout(bytes.as_bytes(), exit_policy::failure())
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
