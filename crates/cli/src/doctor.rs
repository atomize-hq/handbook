use crate::{doctor_rendering, exit_policy, shell_shared::discover_managed_repo_root, DoctorArgs};
use handbook_sdk::contract_membrane::{
    ContractLimits, InvocationResponseV1, TransportFailureKind, TransportFailureV1,
    UseCaseResponseV1,
};
use std::process::ExitCode;

pub(super) fn run(args: DoctorArgs, machine_json: bool) -> ExitCode {
    let limits = ContractLimits::packet0();
    let cwd = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(_) => {
            if machine_json {
                return emit_transport_failure(
                    TransportFailureKind::Repository,
                    "transport.repository_root_unavailable",
                    &limits,
                );
            }
            return exit_policy::emit_stdout(
                b"OUTCOME: ERROR\nCATEGORY: repository_root_unavailable\n",
                exit_policy::failure(),
            );
        }
    };
    let repo_root = discover_managed_repo_root(&cwd);
    let outcome = handbook_sdk::HandbookSdkV1::open(&repo_root).doctor();
    let use_case = match &outcome {
        Ok(report) => UseCaseResponseV1::repository_doctor(report, &limits),
        Err(error) => UseCaseResponseV1::repository_doctor_error(*error, &limits),
    };
    let invocation =
        use_case.and_then(|response| InvocationResponseV1::use_case(response, &limits));
    let invocation = match invocation {
        Ok(invocation) => invocation,
        Err(_) => {
            if machine_json {
                return emit_transport_failure(
                    TransportFailureKind::Serialization,
                    "transport.invocation_construction_failed",
                    &limits,
                );
            }
            return exit_policy::emit_stdout(
                b"OUTCOME: ERROR\nCATEGORY: serialization\nREASON: failed to construct doctor response\n",
                exit_policy::failure(),
            );
        }
    };
    let semantic_exit = exit_policy::terminal(invocation.terminal_status());

    if machine_json {
        return match doctor_rendering::render_invocation(&invocation) {
            Ok(bytes) => exit_policy::emit_stdout(&bytes, semantic_exit),
            Err(_) => emit_transport_failure(
                TransportFailureKind::Serialization,
                "transport.serialization_failed",
                &limits,
            ),
        };
    }

    if args.legacy_json {
        match doctor_rendering::render_json(&invocation) {
            Ok(output) => exit_policy::emit_stdout(output.as_bytes(), semantic_exit),
            Err(_) => emit_transport_failure(
                TransportFailureKind::Serialization,
                "transport.legacy_serialization_failed",
                &limits,
            ),
        }
    } else {
        match doctor_rendering::render_text(&invocation) {
            Ok(output) => exit_policy::emit_stdout(output.as_bytes(), semantic_exit),
            Err(_) => exit_policy::emit_stdout(
                b"OUTCOME: ERROR\nCATEGORY: serialization\nREASON: failed to render doctor response\n",
                exit_policy::failure(),
            ),
        }
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
        .and_then(|invocation| doctor_rendering::render_invocation(&invocation).map_err(|_| ()));
    match bytes {
        Ok(bytes) => exit_policy::emit_stdout(&bytes, exit_policy::failure()),
        Err(()) => {
            eprintln!("handbook: failed to prepare transport-failure response");
            exit_policy::failure()
        }
    }
}
