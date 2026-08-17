use crate::{
    doctor_rendering, exit_policy,
    shell_shared::{discover_managed_repo_root, read_stdin},
    AuthorArgs, AuthorCharterArgs, AuthorCommand, AuthorProjectContextArgs, CharterModeArg, Cli,
};
use handbook_sdk::contract_membrane::{
    AuthorInputTransportRouteV1, AuthorWorkingDirectoryCommandV1, ContractLimits,
    InvocationResponseV1, RepositoryInvocationCommandV1, TransportFailureKind, TransportFailureV1,
    TransportIoErrorV1, TransportPresentationViewV1, UseCaseResponseV1,
};
use serde::Deserialize;
use std::fmt::Write as _;
use std::fs;
use std::process::ExitCode;

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
enum CharterModeWire {
    GuidedAdaptive,
    Express,
    AgentAssisted,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
enum CharterIntakeSourceKindWire {
    UserDeclaration,
    EvidencedInference,
    DeterministicDefault,
    KnownUnknown,
    Contradiction,
    Waiver,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CharterCoverageWire {
    coverage_id: String,
    source_kind: CharterIntakeSourceKindWire,
    value_ref: String,
    evidence_refs: Vec<String>,
    confidence: String,
    freshness: Option<String>,
    sensitivity: String,
    contradiction_refs: Vec<String>,
    waiver_ref: Option<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CharterConsumerWire {
    kind: String,
    id: String,
    version: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CharterIntakeEnvelopeWire {
    mode: CharterModeWire,
    content: serde_json::Value,
    coverage: Vec<CharterCoverageWire>,
    consumer: CharterConsumerWire,
    prompt_event_refs: Vec<String>,
    finalized_at_utc: String,
    expected_current_fingerprint: Option<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ProjectContextInputWire {
    schema_id: String,
    schema_version: String,
    record_id: String,
    summary: String,
    system_boundaries: Vec<String>,
    ownership: Vec<String>,
    authoritative_references: Vec<String>,
    known_unknowns: Vec<String>,
}

pub(crate) fn run(args: AuthorArgs, machine_json: bool) -> ExitCode {
    match args.command {
        Some(AuthorCommand::Charter(args)) => author_charter_command(args, machine_json),
        Some(AuthorCommand::ProjectContext(args)) => {
            author_project_context_command(args, machine_json)
        }
        None => crate::shell_shared::print_subcommand_help::<Cli>(&["author"]),
    }
}

fn author_charter_command(args: AuthorCharterArgs, machine_json: bool) -> ExitCode {
    let legacy_json = args.json;
    let limits = ContractLimits::packet0();
    let invocation = match execute_selected_author_charter_command(args, &limits) {
        Ok(invocation) => Ok(invocation),
        Err(failure) => failure
            .into_response(&limits)
            .and_then(|response| InvocationResponseV1::transport_failure(response, &limits)),
    };
    let invocation = match invocation {
        Ok(invocation) => invocation,
        Err(_) => {
            return emit_transport_failure(
                TransportFailureKind::Serialization,
                "transport.invocation_construction_failed",
                &limits,
            )
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
    let result = match invocation.author_charter_outcome(&limits) {
        Ok(Some(result)) => result,
        Ok(None) => match charter_transport_legacy_projection(&invocation, &limits) {
            Some(result) => result,
            None => {
                return emit_transport_failure(
                    TransportFailureKind::Serialization,
                    "transport.presentation_failed",
                    &limits,
                )
            }
        },
        Err(_) => {
            return emit_transport_failure(
                TransportFailureKind::Serialization,
                "transport.presentation_failed",
                &limits,
            )
        }
    };
    let rendered = if legacy_json {
        render_charter_operation_json(&result)
    } else {
        render_charter_operation_text(&result)
    };
    exit_policy::emit_stdout(rendered.as_bytes(), semantic_exit)
}

struct TransportDescriptor {
    kind: TransportFailureKind,
    reason_code: &'static str,
    presentation: Option<TransportDescriptorPresentation>,
}

enum TransportDescriptorPresentation {
    AuthorInput {
        route: AuthorInputTransportRouteV1,
        input_source: String,
        error: Option<TransportIoErrorV1>,
    },
    WorkingDirectory {
        command: AuthorWorkingDirectoryCommandV1,
        error: TransportIoErrorV1,
    },
}

impl TransportDescriptor {
    fn plain(kind: TransportFailureKind, reason_code: &'static str) -> Self {
        Self {
            kind,
            reason_code,
            presentation: None,
        }
    }

    fn into_response(
        self,
        limits: &ContractLimits,
    ) -> Result<TransportFailureV1, handbook_sdk::contract_membrane::ContractError> {
        match self.presentation {
            None => TransportFailureV1::adapter_error(self.kind, self.reason_code, limits),
            Some(TransportDescriptorPresentation::AuthorInput {
                route,
                input_source,
                error,
            }) => TransportFailureV1::author_input_error(
                self.kind,
                self.reason_code,
                route,
                &input_source,
                error,
                limits,
            ),
            Some(TransportDescriptorPresentation::WorkingDirectory { command, error }) => {
                TransportFailureV1::working_directory_error(
                    self.kind,
                    self.reason_code,
                    command,
                    error,
                    limits,
                )
            }
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

fn charter_transport_legacy_projection(
    invocation: &InvocationResponseV1,
    limits: &ContractLimits,
) -> Option<handbook_sdk::CharterOperationResult> {
    let reason = invocation
        .transport_failure_reason_code(limits)
        .ok()
        .flatten()
        .unwrap_or_default();
    match invocation.transport_presentation(limits).ok().flatten()? {
        TransportPresentationViewV1::RepositoryInvocation { command, failure } => {
            charter_repository_failure_result(command, *failure)
        }
        TransportPresentationViewV1::AuthorInput {
            route: AuthorInputTransportRouteV1::Charter,
            ..
        } if reason == "transport.author_charter_input_invalid" => {
            Some(handbook_sdk::CharterOperationResult::invalid_intake_envelope())
        }
        TransportPresentationViewV1::AuthorInput {
            route: AuthorInputTransportRouteV1::Charter,
            ..
        } if reason == "transport.author_charter_input_unavailable" => {
            Some(handbook_sdk::CharterOperationResult::invalid_request(
                handbook_sdk::CharterOperation::Author,
            ))
        }
        TransportPresentationViewV1::WorkingDirectory { command, .. } => {
            charter_working_directory_operation(command)
                .map(handbook_sdk::CharterOperationResult::invalid_request)
        }
        _ => None,
    }
}

fn charter_repository_failure_result(
    command: RepositoryInvocationCommandV1,
    failure: handbook_sdk::RepositoryInvocationFailureView,
) -> Option<handbook_sdk::CharterOperationResult> {
    let operation = match command {
        RepositoryInvocationCommandV1::CharterAuthor => handbook_sdk::CharterOperation::Author,
        RepositoryInvocationCommandV1::CharterApprove => handbook_sdk::CharterOperation::Approve,
        RepositoryInvocationCommandV1::CharterPromote => handbook_sdk::CharterOperation::Promote,
        RepositoryInvocationCommandV1::CharterValidate => handbook_sdk::CharterOperation::Validate,
        RepositoryInvocationCommandV1::ApproverBootstrap
        | RepositoryInvocationCommandV1::ApproverAddCredential
        | RepositoryInvocationCommandV1::ApproverRevokeCredential
        | RepositoryInvocationCommandV1::ApproverUpdateMapping => {
            return None;
        }
    };
    Some(handbook_sdk::CharterOperationResult {
        schema_id: failure.schema_id.clone(),
        schema_version: failure.schema_version.clone(),
        operation,
        status: handbook_sdk::AuthorOperationStatus::Refused,
        canonical_path: None,
        source_fingerprint: None,
        rendered_output_fingerprint: None,
        intake_ref: None,
        intake_fingerprint: None,
        candidate_ref: None,
        candidate_fingerprint: None,
        approval_ref: None,
        approval_fingerprint: None,
        promotion_ref: None,
        promotion_fingerprint: None,
        changed_paths: failure.changed_paths.clone(),
        refusal: Some(failure.refusal.clone()),
        next_actions: failure.next_actions.clone(),
        invocation_failure: Some(failure),
        causal_disposition: None,
        public_effects: Vec::new(),
    })
}

fn execute_selected_author_charter_command(
    args: AuthorCharterArgs,
    limits: &ContractLimits,
) -> Result<InvocationResponseV1, TransportDescriptor> {
    let author_selected = args.mode.is_some()
        && args.from_inputs.is_some()
        && args.approve_candidate.is_none()
        && args.approval_class.is_none()
        && args.authority_ref.is_none()
        && args.accept_waiver_refs.is_empty()
        && args.promote_candidate.is_none()
        && args.approval_ref.is_none()
        && !args.validate;
    let approve_selected = args.mode.is_none()
        && args.from_inputs.is_none()
        && args.expected_current_fingerprint.is_none()
        && args.approve_candidate.is_some()
        && args.approval_class.is_some()
        && args.authority_ref.is_some()
        && args.promote_candidate.is_none()
        && args.approval_ref.is_none()
        && !args.validate;
    let promote_selected = args.mode.is_none()
        && args.from_inputs.is_none()
        && args.approve_candidate.is_none()
        && args.approval_class.is_none()
        && args.authority_ref.is_none()
        && args.accept_waiver_refs.is_empty()
        && args.promote_candidate.is_some()
        && args.approval_ref.is_some()
        && !args.validate;
    let validate_selected = args.mode.is_none()
        && args.from_inputs.is_none()
        && args.expected_current_fingerprint.is_none()
        && args.approve_candidate.is_none()
        && args.approval_class.is_none()
        && args.authority_ref.is_none()
        && args.accept_waiver_refs.is_empty()
        && args.promote_candidate.is_none()
        && args.approval_ref.is_none()
        && args.validate;
    let legacy_selected = args.mode.is_none()
        && args.from_inputs.is_some()
        && args.approve_candidate.is_none()
        && args.approval_class.is_none()
        && args.authority_ref.is_none()
        && args.accept_waiver_refs.is_empty()
        && args.promote_candidate.is_none()
        && args.approval_ref.is_none()
        && !args.validate;

    if legacy_selected {
        return project_charter_result(
            handbook_sdk::CharterOperationResult::legacy_input_refused(),
            limits,
        );
    }

    let operation = if approve_selected {
        handbook_sdk::CharterOperation::Approve
    } else if promote_selected {
        handbook_sdk::CharterOperation::Promote
    } else if validate_selected {
        handbook_sdk::CharterOperation::Validate
    } else {
        handbook_sdk::CharterOperation::Author
    };
    if !(author_selected || approve_selected || promote_selected || validate_selected) {
        return project_charter_result(
            handbook_sdk::CharterOperationResult::invalid_request(operation),
            limits,
        );
    }

    let cwd = match std::env::current_dir() {
        Ok(cwd) => cwd,
        Err(error) => {
            return Err(TransportDescriptor {
                kind: TransportFailureKind::Adapter,
                reason_code: "transport.repository_root_unavailable",
                presentation: Some(TransportDescriptorPresentation::WorkingDirectory {
                    command: author_working_directory_command(operation),
                    error: TransportIoErrorV1::from_io_error(&error),
                }),
            })
        }
    };
    let repo_root = discover_managed_repo_root(&cwd);
    let intent = if author_selected {
        let path_or_dash = args
            .from_inputs
            .as_deref()
            .expect("author selection requires an input source");
        let yaml = match read_author_inputs_source(path_or_dash) {
            Ok(yaml) => yaml,
            Err(error) => {
                let error = TransportIoErrorV1::from_io_error(&error);
                return Err(TransportDescriptor {
                    kind: author_input_transport_kind(&error),
                    reason_code: "transport.author_charter_input_unavailable",
                    presentation: Some(TransportDescriptorPresentation::AuthorInput {
                        route: AuthorInputTransportRouteV1::Charter,
                        input_source: path_or_dash.to_owned(),
                        error: Some(error),
                    }),
                });
            }
        };
        let mut envelope = match parse_charter_intake_envelope(&yaml) {
            Ok(envelope) => envelope,
            Err(()) => {
                return Err(TransportDescriptor {
                    kind: TransportFailureKind::Decoding,
                    reason_code: "transport.author_charter_input_invalid",
                    presentation: Some(TransportDescriptorPresentation::AuthorInput {
                        route: AuthorInputTransportRouteV1::Charter,
                        input_source: path_or_dash.to_owned(),
                        error: None,
                    }),
                })
            }
        };
        if args.expected_current_fingerprint.is_some()
            && envelope.expected_current_fingerprint.is_some()
            && args.expected_current_fingerprint.as_deref()
                != envelope.expected_current_fingerprint.as_deref()
        {
            return project_charter_result(
                handbook_sdk::CharterOperationResult::invalid_request(operation),
                limits,
            );
        }
        if args.expected_current_fingerprint.is_some() {
            envelope.expected_current_fingerprint = args.expected_current_fingerprint;
        }
        handbook_sdk::CharterCommandRequest::Author {
            mode: charter_mode(args.mode.expect("author selection requires a mode")),
            intake: envelope,
        }
    } else if approve_selected {
        handbook_sdk::CharterCommandRequest::Approve {
            candidate_ref: args
                .approve_candidate
                .expect("approval selection requires a candidate"),
            approval_class: args
                .approval_class
                .expect("approval selection requires a class"),
            authority_ref: args
                .authority_ref
                .expect("approval selection requires an authority"),
            accepted_waiver_refs: args.accept_waiver_refs,
        }
    } else if promote_selected {
        handbook_sdk::CharterCommandRequest::Promote {
            candidate_ref: args
                .promote_candidate
                .expect("promotion selection requires a candidate"),
            approval_ref: args
                .approval_ref
                .expect("promotion selection requires an approval"),
            expected_current_fingerprint: args.expected_current_fingerprint,
        }
    } else {
        handbook_sdk::CharterCommandRequest::Validate
    };
    handbook_sdk::HandbookSdkV1::open(repo_root)
        .run_author_charter_use_case(intent, limits)
        .map_err(|_| {
            TransportDescriptor::plain(
                TransportFailureKind::Serialization,
                "transport.invocation_construction_failed",
            )
        })
}

fn project_charter_result(
    result: handbook_sdk::CharterOperationResult,
    limits: &ContractLimits,
) -> Result<InvocationResponseV1, TransportDescriptor> {
    UseCaseResponseV1::author_charter(&result, limits)
        .and_then(|response| InvocationResponseV1::use_case(response, limits))
        .map_err(|_| {
            TransportDescriptor::plain(
                TransportFailureKind::Serialization,
                "transport.invocation_construction_failed",
            )
        })
}

fn charter_mode(mode: CharterModeArg) -> handbook_sdk::CharterAcquisitionMode {
    match mode {
        CharterModeArg::GuidedAdaptive => handbook_sdk::CharterAcquisitionMode::GuidedAdaptive,
        CharterModeArg::Express => handbook_sdk::CharterAcquisitionMode::Express,
        CharterModeArg::AgentAssisted => handbook_sdk::CharterAcquisitionMode::AgentAssisted,
    }
}

fn author_working_directory_command(
    operation: handbook_sdk::CharterOperation,
) -> AuthorWorkingDirectoryCommandV1 {
    match operation {
        handbook_sdk::CharterOperation::Author => AuthorWorkingDirectoryCommandV1::CharterAuthor,
        handbook_sdk::CharterOperation::Approve => AuthorWorkingDirectoryCommandV1::CharterApprove,
        handbook_sdk::CharterOperation::Promote => AuthorWorkingDirectoryCommandV1::CharterPromote,
        handbook_sdk::CharterOperation::Validate => {
            AuthorWorkingDirectoryCommandV1::CharterValidate
        }
    }
}

fn charter_working_directory_operation(
    command: AuthorWorkingDirectoryCommandV1,
) -> Option<handbook_sdk::CharterOperation> {
    match command {
        AuthorWorkingDirectoryCommandV1::CharterAuthor => {
            Some(handbook_sdk::CharterOperation::Author)
        }
        AuthorWorkingDirectoryCommandV1::CharterApprove => {
            Some(handbook_sdk::CharterOperation::Approve)
        }
        AuthorWorkingDirectoryCommandV1::CharterPromote => {
            Some(handbook_sdk::CharterOperation::Promote)
        }
        AuthorWorkingDirectoryCommandV1::CharterValidate => {
            Some(handbook_sdk::CharterOperation::Validate)
        }
        AuthorWorkingDirectoryCommandV1::ProjectContextAuthor
        | AuthorWorkingDirectoryCommandV1::ProjectContextValidate => None,
    }
}

fn parse_charter_intake_envelope(yaml: &str) -> Result<handbook_sdk::CharterIntakeEnvelope, ()> {
    let wire: CharterIntakeEnvelopeWire = serde_yaml_bw::from_str(yaml).map_err(|_| ())?;
    Ok(handbook_sdk::CharterIntakeEnvelope {
        mode: charter_mode_from_wire(wire.mode),
        content: charter_input_value(wire.content),
        coverage: wire
            .coverage
            .into_iter()
            .map(|coverage| handbook_sdk::CharterCoverageSubmission {
                coverage_id: coverage.coverage_id,
                source_kind: charter_intake_source_kind(coverage.source_kind),
                value_ref: coverage.value_ref,
                evidence_refs: coverage.evidence_refs,
                confidence: coverage.confidence,
                freshness: coverage.freshness,
                sensitivity: coverage.sensitivity,
                contradiction_refs: coverage.contradiction_refs,
                waiver_ref: coverage.waiver_ref,
            })
            .collect(),
        consumer: handbook_sdk::CharterIntakeConsumer {
            kind: wire.consumer.kind,
            id: wire.consumer.id,
            version: wire.consumer.version,
        },
        prompt_event_refs: wire.prompt_event_refs,
        finalized_at_utc: wire.finalized_at_utc,
        expected_current_fingerprint: wire.expected_current_fingerprint,
    })
}

fn charter_mode_from_wire(mode: CharterModeWire) -> handbook_sdk::CharterAcquisitionMode {
    match mode {
        CharterModeWire::GuidedAdaptive => handbook_sdk::CharterAcquisitionMode::GuidedAdaptive,
        CharterModeWire::Express => handbook_sdk::CharterAcquisitionMode::Express,
        CharterModeWire::AgentAssisted => handbook_sdk::CharterAcquisitionMode::AgentAssisted,
    }
}

fn charter_intake_source_kind(
    source_kind: CharterIntakeSourceKindWire,
) -> handbook_sdk::CharterIntakeSourceKind {
    match source_kind {
        CharterIntakeSourceKindWire::UserDeclaration => {
            handbook_sdk::CharterIntakeSourceKind::UserDeclaration
        }
        CharterIntakeSourceKindWire::EvidencedInference => {
            handbook_sdk::CharterIntakeSourceKind::EvidencedInference
        }
        CharterIntakeSourceKindWire::DeterministicDefault => {
            handbook_sdk::CharterIntakeSourceKind::DeterministicDefault
        }
        CharterIntakeSourceKindWire::KnownUnknown => {
            handbook_sdk::CharterIntakeSourceKind::KnownUnknown
        }
        CharterIntakeSourceKindWire::Contradiction => {
            handbook_sdk::CharterIntakeSourceKind::Contradiction
        }
        CharterIntakeSourceKindWire::Waiver => handbook_sdk::CharterIntakeSourceKind::Waiver,
    }
}

fn charter_input_value(value: serde_json::Value) -> handbook_sdk::CharterInputValue {
    match value {
        serde_json::Value::Null => handbook_sdk::CharterInputValue::Null,
        serde_json::Value::Bool(value) => handbook_sdk::CharterInputValue::Boolean(value),
        serde_json::Value::Number(value) => {
            handbook_sdk::CharterInputValue::Number(value.to_string())
        }
        serde_json::Value::String(value) => handbook_sdk::CharterInputValue::String(value),
        serde_json::Value::Array(values) => handbook_sdk::CharterInputValue::Sequence(
            values.into_iter().map(charter_input_value).collect(),
        ),
        serde_json::Value::Object(values) => handbook_sdk::CharterInputValue::Mapping(
            values
                .into_iter()
                .map(|(key, value)| (key, charter_input_value(value)))
                .collect(),
        ),
    }
}

fn render_charter_operation_json(result: &handbook_sdk::CharterOperationResult) -> String {
    let document = if let Some(failure) = &result.invocation_failure {
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
    } else {
        serde_json::json!({
            "schema_id": result.schema_id,
            "schema_version": result.schema_version,
            "operation": charter_operation_name(result.operation),
            "status": author_operation_status_name(result.status),
            "canonical_path": result.canonical_path,
            "source_fingerprint": result.source_fingerprint,
            "rendered_output_fingerprint": result.rendered_output_fingerprint,
            "intake_ref": result.intake_ref,
            "intake_fingerprint": result.intake_fingerprint,
            "candidate_ref": result.candidate_ref,
            "candidate_fingerprint": result.candidate_fingerprint,
            "approval_ref": result.approval_ref,
            "approval_fingerprint": result.approval_fingerprint,
            "promotion_ref": result.promotion_ref,
            "promotion_fingerprint": result.promotion_fingerprint,
            "changed_paths": result.changed_paths,
            "refusal": result.refusal.as_ref().map(|refusal| serde_json::json!({
                "code": refusal.code,
                "message": refusal.message,
                "retryable": refusal.retryable,
            })),
            "next_actions": result.next_actions,
        })
    };
    let mut output = serde_json::to_string_pretty(&document)
        .unwrap_or_else(|_| "{\"status\":\"refused\"}".to_owned());
    output.push('\n');
    output
}

fn render_charter_operation_text(result: &handbook_sdk::CharterOperationResult) -> String {
    let status = match result.status {
        handbook_sdk::AuthorOperationStatus::Succeeded => "SUCCEEDED",
        handbook_sdk::AuthorOperationStatus::Refused => "REFUSED",
    };
    let operation = match result.operation {
        handbook_sdk::CharterOperation::Author => "author",
        handbook_sdk::CharterOperation::Approve => "approve",
        handbook_sdk::CharterOperation::Promote => "promote",
        handbook_sdk::CharterOperation::Validate => "validate",
    };
    let mut output = format!("OUTCOME: {status}\nOPERATION: {operation}\n");
    if let Some(refusal) = &result.refusal {
        writeln!(&mut output, "CODE: {}", refusal.code).expect("string write");
        writeln!(&mut output, "MESSAGE: {}", refusal.message).expect("string write");
        writeln!(&mut output, "RETRYABLE: {}", refusal.retryable).expect("string write");
    }
    if let Some(path) = &result.canonical_path {
        writeln!(&mut output, "CANONICAL PATH: {path}").expect("string write");
    }
    if let Some(fingerprint) = &result.source_fingerprint {
        writeln!(&mut output, "SOURCE FINGERPRINT: {fingerprint}").expect("string write");
    }
    if let Some(fingerprint) = &result.rendered_output_fingerprint {
        writeln!(&mut output, "RENDERED OUTPUT FINGERPRINT: {fingerprint}").expect("string write");
    }
    for action in &result.next_actions {
        writeln!(&mut output, "NEXT SAFE ACTION: {action}").expect("string write");
    }
    output
}

fn charter_operation_name(operation: handbook_sdk::CharterOperation) -> &'static str {
    match operation {
        handbook_sdk::CharterOperation::Author => "author",
        handbook_sdk::CharterOperation::Approve => "approve",
        handbook_sdk::CharterOperation::Promote => "promote",
        handbook_sdk::CharterOperation::Validate => "validate",
    }
}

fn author_operation_status_name(status: handbook_sdk::AuthorOperationStatus) -> &'static str {
    match status {
        handbook_sdk::AuthorOperationStatus::Succeeded => "succeeded",
        handbook_sdk::AuthorOperationStatus::Refused => "refused",
    }
}

fn author_project_context_command(args: AuthorProjectContextArgs, machine_json: bool) -> ExitCode {
    let limits = ContractLimits::packet0();
    let validate = args.validate;
    let invocation = if let Some(path_or_dash) = args.from_inputs.as_deref() {
        let cwd = match std::env::current_dir() {
            Ok(dir) => dir,
            Err(error) => {
                let command = if validate {
                    AuthorWorkingDirectoryCommandV1::ProjectContextValidate
                } else {
                    AuthorWorkingDirectoryCommandV1::ProjectContextAuthor
                };
                let failure = TransportFailureV1::working_directory_error(
                    TransportFailureKind::Adapter,
                    "transport.repository_root_unavailable",
                    command,
                    TransportIoErrorV1::from_io_error(&error),
                    &limits,
                );
                return emit_author_project_context_transport(failure, machine_json, &limits);
            }
        };
        let repo_root = discover_managed_repo_root(&cwd);
        let yaml = match read_author_inputs_source(path_or_dash) {
            Ok(yaml) => yaml,
            Err(error) => {
                let error = TransportIoErrorV1::from_io_error(&error);
                let failure = TransportFailureV1::author_input_error(
                    author_input_transport_kind(&error),
                    "transport.author_project_context_input_unavailable",
                    AuthorInputTransportRouteV1::ProjectContext,
                    path_or_dash,
                    Some(error),
                    &limits,
                );
                return emit_author_project_context_transport(failure, machine_json, &limits);
            }
        };
        let input = match parse_project_context_input_yaml(&yaml) {
            Ok(input) => input,
            Err(_) => {
                let failure = TransportFailureV1::author_input_error(
                    TransportFailureKind::Decoding,
                    "transport.author_project_context_input_invalid",
                    AuthorInputTransportRouteV1::ProjectContext,
                    path_or_dash,
                    None,
                    &limits,
                );
                return emit_author_project_context_transport(failure, machine_json, &limits);
            }
        };
        let input_mode = if path_or_dash == "-" {
            "structured_inputs_stdin"
        } else {
            "structured_inputs_file"
        };
        let request = if validate {
            handbook_sdk::ProjectContextCommandRequest::Validate(input)
        } else {
            handbook_sdk::ProjectContextCommandRequest::Author(input)
        };
        handbook_sdk::HandbookSdkV1::open(repo_root)
            .run_author_project_context_use_case(input_mode, path_or_dash, request, &limits)
            .and_then(|response| InvocationResponseV1::use_case(response, &limits))
    } else {
        let refusal = handbook_sdk::AuthorProjectContextRefusal {
            kind: handbook_sdk::AuthorProjectContextRefusalKind::InvalidRequest,
            summary: "`handbook author project-context` requires `--from-inputs <path|->`"
                .to_owned(),
            broken_subject: "command arguments".to_owned(),
            next_safe_action: "retry `handbook author project-context --from-inputs <path|->`"
                .to_owned(),
            causal_disposition: handbook_sdk::AuthorCausalDisposition::CallerRefusal,
            public_effects: Vec::new(),
        };
        let outcome = if validate {
            handbook_sdk::ProjectContextCommandOutcome::Validate {
                input_mode: "structured_inputs_missing".to_owned(),
                input_source: "missing".to_owned(),
                result: Err(refusal),
            }
        } else {
            handbook_sdk::ProjectContextCommandOutcome::Author {
                input_mode: "structured_inputs_missing".to_owned(),
                input_source: "missing".to_owned(),
                result: Err(refusal),
            }
        };
        UseCaseResponseV1::author_project_context(&outcome, &limits)
            .and_then(|response| InvocationResponseV1::use_case(response, &limits))
    };
    let invocation = match invocation {
        Ok(invocation) => invocation,
        Err(_) => {
            return emit_transport_failure(
                TransportFailureKind::Serialization,
                "transport.invocation_construction_failed",
                &limits,
            )
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
    let output = match invocation.author_project_context_outcome(&limits) {
        Ok(Some(handbook_sdk::ProjectContextCommandOutcome::Author {
            input_mode,
            input_source,
            result: Ok(result),
        })) => render_author_project_context_success(&result, &input_mode, &input_source),
        Ok(Some(handbook_sdk::ProjectContextCommandOutcome::Validate {
            input_mode,
            input_source,
            result: Ok(()),
        })) => render_author_project_context_validation_success(&input_mode, &input_source),
        Ok(Some(handbook_sdk::ProjectContextCommandOutcome::Author {
            result: Err(refusal),
            ..
        }))
        | Ok(Some(handbook_sdk::ProjectContextCommandOutcome::Validate {
            result: Err(refusal),
            ..
        })) => render_project_context_refusal(&refusal),
        Ok(None) | Err(_) => {
            return emit_transport_failure(
                TransportFailureKind::Serialization,
                "transport.presentation_failed",
                &limits,
            )
        }
    };
    let mut bytes = output.into_bytes();
    bytes.push(b'\n');
    exit_policy::emit_stdout(&bytes, semantic_exit)
}

fn emit_author_project_context_transport(
    failure: Result<TransportFailureV1, handbook_sdk::contract_membrane::ContractError>,
    machine_json: bool,
    limits: &ContractLimits,
) -> ExitCode {
    let invocation =
        failure.and_then(|failure| InvocationResponseV1::transport_failure(failure, limits));
    let invocation = match invocation {
        Ok(invocation) => invocation,
        Err(_) => {
            return emit_transport_failure(
                TransportFailureKind::Serialization,
                "transport.invocation_construction_failed",
                limits,
            )
        }
    };
    let semantic_exit = exit_policy::terminal(invocation.terminal_status());
    if machine_json {
        return match doctor_rendering::render_invocation(&invocation) {
            Ok(bytes) => exit_policy::emit_stdout(&bytes, semantic_exit),
            Err(_) => emit_transport_failure(
                TransportFailureKind::Serialization,
                "transport.serialization_failed",
                limits,
            ),
        };
    }
    let reason = invocation
        .transport_failure_reason_code(limits)
        .ok()
        .flatten();
    let output = match invocation.transport_presentation(limits) {
        Ok(Some(TransportPresentationViewV1::AuthorInput {
            route: AuthorInputTransportRouteV1::ProjectContext,
            input_source: _,
            error: _,
        })) if reason.as_deref() == Some("transport.author_project_context_input_invalid") => {
            render_project_context_refusal(&handbook_sdk::AuthorProjectContextRefusal {
                kind: handbook_sdk::AuthorProjectContextRefusalKind::MalformedStructuredInput,
                summary: "failed to parse canonical Project Context YAML".to_owned(),
                broken_subject: "canonical project-context input".to_owned(),
                next_safe_action: "repair the canonical `1.0` Project Context YAML and retry"
                    .to_owned(),
                causal_disposition: handbook_sdk::AuthorCausalDisposition::CallerRefusal,
                public_effects: Vec::new(),
            })
        }
        Ok(Some(TransportPresentationViewV1::AuthorInput {
            route: AuthorInputTransportRouteV1::ProjectContext,
            input_source,
            error: Some(error),
        })) => render_author_custom_refusal(
            "author project-context",
            "REFUSED",
            "InputReadFailure",
            &if input_source == "-" {
                format!(
                    "failed to read structured inputs from stdin: {}",
                    error.display_message()
                )
            } else {
                format!(
                    "failed to read structured inputs from `{input_source}`: {}",
                    error.display_message()
                )
            },
            "structured input source",
            if input_source == "-" {
                "repair stdin and retry `handbook author project-context --from-inputs -`"
            } else {
                "repair the structured input file and retry `handbook author project-context --from-inputs <path|->`"
            },
        ),
        Ok(Some(TransportPresentationViewV1::WorkingDirectory {
            command:
                AuthorWorkingDirectoryCommandV1::ProjectContextAuthor
                | AuthorWorkingDirectoryCommandV1::ProjectContextValidate,
            error,
        })) => render_author_custom_refusal(
            "author project-context",
            "REFUSED",
            "WorkingDirectoryUnavailable",
            &format!("failed to determine repo root: {}", error.display_message()),
            "current working directory",
            "repair the current working directory and retry `handbook author project-context --from-inputs <path|->`",
        ),
        _ => {
            return emit_transport_failure(
                TransportFailureKind::Serialization,
                "transport.presentation_failed",
                limits,
            )
        }
    };
    let mut bytes = output.into_bytes();
    bytes.push(b'\n');
    exit_policy::emit_stdout(&bytes, semantic_exit)
}

fn render_author_project_context_success(
    result: &handbook_sdk::AuthorProjectContextResult,
    input_mode: &str,
    input_source: &str,
) -> String {
    let mut out = String::new();
    out.push_str("OUTCOME: AUTHORED\n");
    out.push_str("OBJECT: author project-context\n");
    out.push_str("NEXT SAFE ACTION: run `handbook doctor`\n");
    out.push_str("## CANONICAL ARTIFACT\n");
    out.push_str(&format!("PATH: {}\n", result.canonical_repo_relative_path));
    out.push_str(&format!("BYTES WRITTEN: {}\n", result.bytes_written));
    out.push_str(&format!(
        "SOURCE FINGERPRINT: {}\n",
        result.source_fingerprint
    ));
    out.push_str(&format!(
        "RENDERED OUTPUT FINGERPRINT: {}\n",
        result.rendered_output_fingerprint
    ));
    out.push_str(&format!(
        "RENDERED MEDIA TYPE: {}\n",
        result.rendered_media_type
    ));
    out.push_str("## INPUT MODE\n");
    out.push_str(&format!("MODE: {input_mode}\n"));
    out.push_str(&format!("SOURCE: {input_source}\n"));
    out.trim_end().to_string()
}

fn render_author_project_context_validation_success(
    input_mode: &str,
    input_source: &str,
) -> String {
    let mut out = String::new();
    out.push_str("OUTCOME: VALIDATED\n");
    out.push_str("OBJECT: author project-context\n");
    out.push_str(
        "NEXT SAFE ACTION: run `handbook author project-context --from-inputs <path|->`\n",
    );
    out.push_str("## INPUT MODE\n");
    out.push_str(&format!("MODE: {input_mode}\n"));
    out.push_str(&format!("SOURCE: {input_source}\n"));
    out.push_str("## SUMMARY\n");
    out.push_str(
        "Canonical Project Context YAML and its fixed Markdown view validated without mutation.",
    );
    out.trim_end().to_string()
}

fn render_author_custom_refusal(
    object: &str,
    outcome: &str,
    category: &str,
    summary: &str,
    broken_subject: &str,
    next_safe_action: &str,
) -> String {
    let mut out = String::new();
    out.push_str(&format!("OUTCOME: {outcome}\n"));
    out.push_str(&format!("OBJECT: {object}\n"));
    out.push_str(&format!("NEXT SAFE ACTION: {next_safe_action}\n"));
    out.push_str("## REFUSAL\n");
    out.push_str(&format!("CATEGORY: {category}\n"));
    out.push_str(&format!("SUMMARY: {summary}\n"));
    out.push_str(&format!("BROKEN SUBJECT: {broken_subject}\n"));
    out.push_str(&format!("NEXT SAFE ACTION: {next_safe_action}\n"));
    out.trim_end().to_string()
}

fn render_author_simple_refusal(
    object: &str,
    outcome: &str,
    category: &str,
    summary: &str,
    broken_subject: &str,
    next_safe_action: &str,
) -> String {
    render_author_custom_refusal(
        object,
        outcome,
        category,
        summary,
        broken_subject,
        next_safe_action,
    )
}

fn render_project_context_refusal(refusal: &handbook_sdk::AuthorProjectContextRefusal) -> String {
    render_author_simple_refusal(
        "author project-context",
        author_project_context_refusal_outcome_name(refusal.kind),
        author_project_context_refusal_kind_name(refusal.kind),
        refusal.summary.trim(),
        refusal.broken_subject.trim(),
        refusal.next_safe_action.trim(),
    )
}

fn parse_project_context_input_yaml(
    yaml: &str,
) -> Result<handbook_sdk::ProjectContextInput, handbook_sdk::AuthorProjectContextRefusal> {
    let wire: ProjectContextInputWire =
        serde_yaml_bw::from_str(yaml).map_err(|_| handbook_sdk::AuthorProjectContextRefusal {
            kind: handbook_sdk::AuthorProjectContextRefusalKind::MalformedStructuredInput,
            summary: "failed to parse canonical Project Context YAML".to_owned(),
            broken_subject: "canonical project-context input".to_owned(),
            next_safe_action: "repair the canonical `1.0` Project Context YAML and retry"
                .to_owned(),
            causal_disposition: handbook_sdk::AuthorCausalDisposition::CallerRefusal,
            public_effects: Vec::new(),
        })?;
    Ok(handbook_sdk::ProjectContextInput {
        schema_id: wire.schema_id,
        schema_version: wire.schema_version,
        record_id: wire.record_id,
        summary: wire.summary,
        system_boundaries: wire.system_boundaries,
        ownership: wire.ownership,
        authoritative_references: wire.authoritative_references,
        known_unknowns: wire.known_unknowns,
    })
}

fn read_author_inputs_source(path_or_dash: &str) -> Result<String, std::io::Error> {
    if path_or_dash == "-" {
        return read_stdin();
    }
    fs::read_to_string(path_or_dash)
}

fn author_input_transport_kind(error: &TransportIoErrorV1) -> TransportFailureKind {
    if error.is_caller_acquisition() {
        TransportFailureKind::Acquisition
    } else {
        TransportFailureKind::Adapter
    }
}

fn author_project_context_refusal_outcome_name(
    kind: handbook_sdk::AuthorProjectContextRefusalKind,
) -> &'static str {
    match kind {
        handbook_sdk::AuthorProjectContextRefusalKind::InvalidRequest => "REFUSED",
        handbook_sdk::AuthorProjectContextRefusalKind::MissingSystemRoot
        | handbook_sdk::AuthorProjectContextRefusalKind::InvalidSystemRoot
        | handbook_sdk::AuthorProjectContextRefusalKind::ShippedContractUnavailable
        | handbook_sdk::AuthorProjectContextRefusalKind::SelectedDecisionMissing
        | handbook_sdk::AuthorProjectContextRefusalKind::SystemRootInspectionFailed
        | handbook_sdk::AuthorProjectContextRefusalKind::UnsafeSystemRoot
        | handbook_sdk::AuthorProjectContextRefusalKind::CanonicalInspectionFailed
        | handbook_sdk::AuthorProjectContextRefusalKind::MutationRefused
        | handbook_sdk::AuthorProjectContextRefusalKind::MutationSafetyRefused
        | handbook_sdk::AuthorProjectContextRefusalKind::MutationIoFailed => "BLOCKED",
        handbook_sdk::AuthorProjectContextRefusalKind::MalformedStructuredInput
        | handbook_sdk::AuthorProjectContextRefusalKind::IncompleteStructuredInput
        | handbook_sdk::AuthorProjectContextRefusalKind::ExistingCanonicalTruth
        | handbook_sdk::AuthorProjectContextRefusalKind::UnsupportedPlatformStrictMutation => {
            "REFUSED"
        }
    }
}

fn author_project_context_refusal_kind_name(
    kind: handbook_sdk::AuthorProjectContextRefusalKind,
) -> &'static str {
    match kind {
        handbook_sdk::AuthorProjectContextRefusalKind::InvalidRequest => "InvalidRequest",
        handbook_sdk::AuthorProjectContextRefusalKind::MissingSystemRoot => "MissingSystemRoot",
        handbook_sdk::AuthorProjectContextRefusalKind::InvalidSystemRoot => "InvalidSystemRoot",
        handbook_sdk::AuthorProjectContextRefusalKind::ShippedContractUnavailable
        | handbook_sdk::AuthorProjectContextRefusalKind::SelectedDecisionMissing
        | handbook_sdk::AuthorProjectContextRefusalKind::SystemRootInspectionFailed
        | handbook_sdk::AuthorProjectContextRefusalKind::UnsafeSystemRoot => "InvalidSystemRoot",
        handbook_sdk::AuthorProjectContextRefusalKind::MalformedStructuredInput => {
            "MalformedStructuredInput"
        }
        handbook_sdk::AuthorProjectContextRefusalKind::IncompleteStructuredInput => {
            "IncompleteStructuredInput"
        }
        handbook_sdk::AuthorProjectContextRefusalKind::ExistingCanonicalTruth => {
            "ExistingCanonicalTruth"
        }
        handbook_sdk::AuthorProjectContextRefusalKind::MutationRefused => "MutationRefused",
        handbook_sdk::AuthorProjectContextRefusalKind::CanonicalInspectionFailed
        | handbook_sdk::AuthorProjectContextRefusalKind::MutationSafetyRefused
        | handbook_sdk::AuthorProjectContextRefusalKind::MutationIoFailed => "MutationRefused",
        handbook_sdk::AuthorProjectContextRefusalKind::UnsupportedPlatformStrictMutation => {
            "UnsupportedPlatformStrictMutation"
        }
    }
}
