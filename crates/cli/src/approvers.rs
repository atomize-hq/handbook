use crate::{
    doctor_rendering, exit_policy, shell_shared::discover_managed_repo_root,
    ApproverAddCredentialArgs, ApproverBootstrapArgs, ApproverRevokeCredentialArgs,
    ApproverUpdateMappingArgs, ApproversArgs, ApproversCommand,
};
use handbook_sdk::contract_membrane::{
    ContractLimits, InvocationResponseV1, RepositoryInvocationCommandV1, TransportFailureKind,
    TransportFailureV1, TransportPresentationViewV1, UseCaseResponseV1,
};
use std::fmt::Write as _;
use std::process::ExitCode;

pub(crate) fn run(args: ApproversArgs, machine_json: bool) -> ExitCode {
    let (intent, json) = match args.command {
        ApproversCommand::Bootstrap(ApproverBootstrapArgs {
            initial_charter_quorum,
            json,
        }) => match parse_approver_mappings(initial_charter_quorum) {
            Ok(initial_charter_quorum) => (
                handbook_sdk::ApproverCommandRequest::Bootstrap {
                    initial_charter_quorum,
                },
                json,
            ),
            Err(message) => {
                return emit_invalid_request(
                    machine_json,
                    json,
                    handbook_sdk::ApproverOperation::Bootstrap,
                    &message,
                    "use one or more exact non-empty class=authority quorum pairs",
                );
            }
        },
        ApproversCommand::AddCredential(ApproverAddCredentialArgs {
            approval_mappings,
            json,
        }) => match parse_approver_mappings(approval_mappings) {
            Ok(approval_mappings) => (
                handbook_sdk::ApproverCommandRequest::AddCredential { approval_mappings },
                json,
            ),
            Err(message) => {
                return emit_invalid_request(
                    machine_json,
                    json,
                    handbook_sdk::ApproverOperation::AddCredential,
                    &message,
                    "use one or more exact non-empty class=authority approval mappings",
                );
            }
        },
        ApproversCommand::RevokeCredential(ApproverRevokeCredentialArgs {
            credential_id_hash,
            json,
        }) => (
            handbook_sdk::ApproverCommandRequest::RevokeCredential { credential_id_hash },
            json,
        ),
        ApproversCommand::UpdateMapping(ApproverUpdateMappingArgs {
            credential_id_hash,
            approval_mappings,
            json,
        }) => match parse_approver_mappings(approval_mappings) {
            Ok(approval_mappings) => (
                handbook_sdk::ApproverCommandRequest::UpdateMapping {
                    credential_id_hash,
                    approval_mappings,
                },
                json,
            ),
            Err(message) => {
                return emit_invalid_request(
                    machine_json,
                    json,
                    handbook_sdk::ApproverOperation::UpdateMapping,
                    &message,
                    "use one or more exact non-empty class=authority approval mappings",
                );
            }
        },
    };
    let cwd = std::env::current_dir().unwrap_or_else(|_| ".".into());
    let repo_root = discover_managed_repo_root(&cwd);
    let limits = ContractLimits::packet0();
    let invocation = match handbook_sdk::HandbookSdkV1::open(&repo_root)
        .run_approver_administration_use_case(intent, &limits)
    {
        Ok(invocation) => invocation,
        Err(_) => {
            return emit_machine_transport(
                TransportFailureKind::Serialization,
                "transport.invocation_construction_failed",
                &limits,
            )
        }
    };
    emit_approver_invocation(invocation, machine_json, json, &limits)
}

fn emit_approver_invocation(
    invocation: InvocationResponseV1,
    machine_json: bool,
    legacy_json: bool,
    limits: &ContractLimits,
) -> ExitCode {
    let semantic_exit = exit_policy::terminal(invocation.terminal_status());
    if machine_json {
        return match doctor_rendering::render_invocation(&invocation) {
            Ok(bytes) => exit_policy::emit_stdout(&bytes, semantic_exit),
            Err(_) => emit_machine_transport(
                TransportFailureKind::Serialization,
                "transport.serialization_failed",
                limits,
            ),
        };
    }
    let projected = match invocation.approver_administration_outcome(limits) {
        Ok(Some(result)) => result,
        Ok(None) => match approver_transport_legacy_projection(&invocation, limits) {
            Some(result) => result,
            None => {
                return emit_machine_transport(
                    TransportFailureKind::Serialization,
                    "transport.presentation_failed",
                    limits,
                )
            }
        },
        Err(_) => {
            return emit_machine_transport(
                TransportFailureKind::Serialization,
                "transport.presentation_failed",
                limits,
            )
        }
    };
    let output = if legacy_json {
        render_json(&projected)
    } else {
        render_text(&projected)
    };
    exit_policy::emit_stdout(output.as_bytes(), semantic_exit)
}

fn parse_approver_mappings(
    values: Vec<String>,
) -> Result<Vec<handbook_sdk::ApproverMapping>, String> {
    let mut mappings = values
        .into_iter()
        .map(|value| {
            let (approval_class, authority_ref) = value.split_once('=').ok_or_else(|| {
                "approver mapping must use exact class=authority syntax".to_owned()
            })?;
            if approval_class.is_empty()
                || authority_ref.is_empty()
                || approval_class.contains('\0')
                || authority_ref.contains('\0')
            {
                return Err(
                    "approver mapping class and authority must be non-empty NUL-free strings"
                        .to_owned(),
                );
            }
            Ok(handbook_sdk::ApproverMapping::new(
                approval_class,
                authority_ref,
            ))
        })
        .collect::<Result<Vec<_>, String>>()?;
    mappings.sort_by(|left, right| {
        left.approval_class()
            .cmp(right.approval_class())
            .then_with(|| left.authority_ref().cmp(right.authority_ref()))
    });
    if mappings.windows(2).any(|pair| pair[0] == pair[1]) {
        return Err("approver mappings must be unique".to_owned());
    }
    Ok(mappings)
}

fn approver_transport_legacy_projection(
    invocation: &InvocationResponseV1,
    limits: &ContractLimits,
) -> Option<handbook_sdk::ApproverOperationResult> {
    let Some(TransportPresentationViewV1::RepositoryInvocation { command, failure }) =
        invocation.transport_presentation(limits).ok()?
    else {
        return None;
    };
    let operation = match command {
        RepositoryInvocationCommandV1::ApproverBootstrap => {
            handbook_sdk::ApproverOperation::Bootstrap
        }
        RepositoryInvocationCommandV1::ApproverAddCredential => {
            handbook_sdk::ApproverOperation::AddCredential
        }
        RepositoryInvocationCommandV1::ApproverRevokeCredential => {
            handbook_sdk::ApproverOperation::RevokeCredential
        }
        RepositoryInvocationCommandV1::ApproverUpdateMapping => {
            handbook_sdk::ApproverOperation::UpdateMapping
        }
        RepositoryInvocationCommandV1::CharterAuthor
        | RepositoryInvocationCommandV1::CharterApprove
        | RepositoryInvocationCommandV1::CharterPromote
        | RepositoryInvocationCommandV1::CharterValidate => return None,
    };
    Some(handbook_sdk::ApproverOperationResult {
        schema_id: failure.schema_id.clone(),
        schema_version: failure.schema_version.clone(),
        operation,
        status: handbook_sdk::AuthorOperationStatus::Refused,
        changed_paths: failure.changed_paths.clone(),
        refusal: Some(failure.refusal.clone()),
        next_actions: failure.next_actions.clone(),
        legacy_projection: Some(
            handbook_sdk::ApproverOperationProjection::InvocationFailure(*failure),
        ),
        causal_disposition: None,
        public_effects: Vec::new(),
    })
}

fn emit_invalid_request(
    machine_json: bool,
    json: bool,
    operation: handbook_sdk::ApproverOperation,
    message: &str,
    next_action: &str,
) -> ExitCode {
    let limits = ContractLimits::packet0();
    let result = handbook_sdk::ApproverOperationResult {
        schema_id: "handbook.approver-admin-adapter-result".to_owned(),
        schema_version: "1.0".to_owned(),
        operation,
        status: handbook_sdk::AuthorOperationStatus::Refused,
        changed_paths: Vec::new(),
        refusal: Some(handbook_sdk::AuthorOperationRefusal {
            code: "invalid_request".to_owned(),
            message: message.to_owned(),
            retryable: false,
        }),
        next_actions: vec![next_action.to_owned()],
        legacy_projection: None,
        causal_disposition: Some(handbook_sdk::AuthorCausalDisposition::CallerRefusal),
        public_effects: Vec::new(),
    };
    let invocation = UseCaseResponseV1::approver_administration(&result, &limits)
        .and_then(|response| InvocationResponseV1::use_case(response, &limits));
    match invocation {
        Ok(invocation) => emit_approver_invocation(invocation, machine_json, json, &limits),
        Err(_) => emit_machine_transport(
            TransportFailureKind::Serialization,
            "transport.invocation_construction_failed",
            &limits,
        ),
    }
}

fn emit_machine_transport(
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

fn render_json(result: &handbook_sdk::ApproverOperationResult) -> String {
    let document = match &result.legacy_projection {
        Some(handbook_sdk::ApproverOperationProjection::Admin(fields)) => serde_json::json!({
            "schema_id": result.schema_id,
            "schema_version": result.schema_version,
            "operation_id": fields.operation_id,
            "repository_identity_fingerprint": fields.repository_identity_fingerprint,
            "prior_registry_state_ref": fields.prior_registry_state_ref,
            "prior_registry_state_fingerprint": fields.prior_registry_state_fingerprint,
            "result_registry_state_ref": fields.result_registry_state_ref,
            "result_registry_state_fingerprint": fields.result_registry_state_fingerprint,
            "transition_ref": fields.transition_ref,
            "transition_fingerprint": fields.transition_fingerprint,
            "registration_ref": fields.registration_ref,
            "registration_fingerprint": fields.registration_fingerprint,
            "assertion_ref": fields.assertion_ref,
            "assertion_fingerprint": fields.assertion_fingerprint,
            "next_actions": result.next_actions,
            "operation": approver_operation_name(result.operation),
            "status": author_operation_status_name(result.status),
            "refusal": result.refusal.as_ref().map(|refusal| serde_json::json!({
                "code": refusal.code,
                "message": refusal.message,
                "retryable": refusal.retryable,
            })),
            "changed_paths": result.changed_paths,
        }),
        Some(handbook_sdk::ApproverOperationProjection::InvocationFailure(failure)) => {
            if let Some(stage) = &failure.stage {
                serde_json::json!({
                    "schema_id": failure.schema_id,
                    "schema_version": failure.schema_version,
                    "operation": failure.operation,
                    "stage": stage,
                    "status": failure.status,
                    "repository_identity_fingerprint": failure.repository_identity_fingerprint,
                    "operation_id": failure.operation_id,
                    "changed_paths": failure.changed_paths,
                    "refusal": {
                        "code": failure.refusal.code,
                        "message": failure.refusal.message,
                        "retryable": failure.refusal.retryable,
                    },
                    "next_actions": failure.next_actions,
                })
            } else {
                serde_json::json!({
                    "schema_id": failure.schema_id,
                    "schema_version": failure.schema_version,
                    "operation": failure.operation,
                    "status": failure.status,
                    "repository_identity_fingerprint": failure.repository_identity_fingerprint,
                    "operation_id": failure.operation_id,
                    "changed_paths": failure.changed_paths,
                    "refusal": {
                        "code": failure.refusal.code,
                        "message": failure.refusal.message,
                        "retryable": failure.refusal.retryable,
                    },
                    "next_actions": failure.next_actions,
                })
            }
        }
        None => serde_json::json!({
            "schema_id": result.schema_id,
            "schema_version": result.schema_version,
            "operation": approver_operation_name(result.operation),
            "status": author_operation_status_name(result.status),
            "changed_paths": result.changed_paths,
            "refusal": result.refusal.as_ref().map(|refusal| serde_json::json!({
                "code": refusal.code,
                "message": refusal.message,
                "retryable": refusal.retryable,
            })),
            "next_actions": result.next_actions,
        }),
    };
    let mut output = serde_json::to_string_pretty(&document)
        .unwrap_or_else(|_| "{\"status\":\"refused\"}".to_owned());
    output.push('\n');
    output
}

fn render_text(result: &handbook_sdk::ApproverOperationResult) -> String {
    let operation = match result.operation {
        handbook_sdk::ApproverOperation::Bootstrap => "bootstrap",
        handbook_sdk::ApproverOperation::AddCredential => "add_credential",
        handbook_sdk::ApproverOperation::RevokeCredential => "revoke_credential",
        handbook_sdk::ApproverOperation::UpdateMapping => "update_mapping",
    };
    let outcome = match result.status {
        handbook_sdk::AuthorOperationStatus::Succeeded => "SUCCEEDED",
        handbook_sdk::AuthorOperationStatus::Refused => "REFUSED",
    };
    let mut output = format!("OUTCOME: {outcome}\nOPERATION: {operation}\n");
    for path in &result.changed_paths {
        writeln!(&mut output, "CHANGED PATH: {path}").expect("string write");
    }
    if let Some(refusal) = &result.refusal {
        writeln!(&mut output, "CODE: {}", refusal.code).expect("string write");
        writeln!(&mut output, "MESSAGE: {}", refusal.message).expect("string write");
        writeln!(&mut output, "RETRYABLE: {}", refusal.retryable).expect("string write");
    }
    for action in &result.next_actions {
        writeln!(&mut output, "NEXT SAFE ACTION: {action}").expect("string write");
    }
    output
}

fn approver_operation_name(operation: handbook_sdk::ApproverOperation) -> &'static str {
    match operation {
        handbook_sdk::ApproverOperation::Bootstrap => "bootstrap",
        handbook_sdk::ApproverOperation::AddCredential => "add_credential",
        handbook_sdk::ApproverOperation::RevokeCredential => "revoke_credential",
        handbook_sdk::ApproverOperation::UpdateMapping => "update_mapping",
    }
}

fn author_operation_status_name(status: handbook_sdk::AuthorOperationStatus) -> &'static str {
    match status {
        handbook_sdk::AuthorOperationStatus::Succeeded => "succeeded",
        handbook_sdk::AuthorOperationStatus::Refused => "refused",
    }
}
