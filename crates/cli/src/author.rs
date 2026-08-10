use crate::{
    shell_shared::{discover_managed_repo_root, read_stdin},
    AuthorArgs, AuthorCharterArgs, AuthorCommand, AuthorProjectContextArgs, CharterModeArg, Cli,
};
use serde::Deserialize;
use std::fmt::Write as _;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
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

pub(crate) fn run(args: AuthorArgs) -> ExitCode {
    match args.command {
        Some(AuthorCommand::Charter(args)) => author_charter_command(args),
        Some(AuthorCommand::ProjectContext(args)) => author_project_context_command(args),
        None => crate::shell_shared::print_subcommand_help::<Cli>(&["author"]),
    }
}

pub(crate) struct RenderedCommand {
    pub(crate) output: String,
    pub(crate) exit_code: ExitCode,
}

fn author_charter_command(args: AuthorCharterArgs) -> ExitCode {
    let json = args.json;
    let result = execute_selected_author_charter_command(args);
    let rendered = if json {
        render_charter_operation_json(&result)
    } else {
        render_charter_operation_text(&result)
    };
    print!("{rendered}");
    if result.status == handbook_sdk::AuthorOperationStatus::Succeeded {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}

fn execute_selected_author_charter_command(
    args: AuthorCharterArgs,
) -> handbook_sdk::CharterOperationResult {
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
        return handbook_sdk::CharterOperationResult::legacy_input_refused();
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
        return handbook_sdk::CharterOperationResult::invalid_request(operation);
    }

    let cwd = match std::env::current_dir() {
        Ok(cwd) => cwd,
        Err(_) => return handbook_sdk::CharterOperationResult::invalid_request(operation),
    };
    let repo_root = discover_managed_repo_root(&cwd);
    let intent = if author_selected {
        let path_or_dash = args
            .from_inputs
            .as_deref()
            .expect("author selection requires an input source");
        let yaml = match read_author_inputs_source(
            "author charter",
            "handbook author charter --mode <mode> --from-inputs",
            path_or_dash,
        ) {
            Ok(yaml) => yaml,
            Err(_) => return handbook_sdk::CharterOperationResult::invalid_request(operation),
        };
        let mut envelope = match parse_charter_intake_envelope(&yaml) {
            Ok(envelope) => envelope,
            Err(()) => return handbook_sdk::CharterOperationResult::invalid_intake_envelope(),
        };
        if args.expected_current_fingerprint.is_some()
            && envelope.expected_current_fingerprint.is_some()
            && args.expected_current_fingerprint.as_deref()
                != envelope.expected_current_fingerprint.as_deref()
        {
            return handbook_sdk::CharterOperationResult::invalid_request(operation);
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
    handbook_sdk::HandbookSdkV1::open(repo_root).author_charter(intent)
}

fn charter_mode(mode: CharterModeArg) -> handbook_sdk::CharterAcquisitionMode {
    match mode {
        CharterModeArg::GuidedAdaptive => handbook_sdk::CharterAcquisitionMode::GuidedAdaptive,
        CharterModeArg::Express => handbook_sdk::CharterAcquisitionMode::Express,
        CharterModeArg::AgentAssisted => handbook_sdk::CharterAcquisitionMode::AgentAssisted,
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

fn author_project_context_command(args: AuthorProjectContextArgs) -> ExitCode {
    let rendered = execute_author_project_context_command(
        args,
        std::env::current_dir,
        |repo_root| {
            handbook_sdk::HandbookSdkV1::open(repo_root).preflight_project_context_authoring()
        },
        |repo_root, input| {
            handbook_sdk::HandbookSdkV1::open(repo_root).author_project_context(input)
        },
    );
    println!("{}", rendered.output);
    rendered.exit_code
}

fn execute_author_project_context_command<GetCurrentDir, PreflightAuthoring, RunAuthor>(
    args: AuthorProjectContextArgs,
    get_current_dir: GetCurrentDir,
    preflight_authoring: PreflightAuthoring,
    run_author: RunAuthor,
) -> RenderedCommand
where
    GetCurrentDir: FnOnce() -> io::Result<PathBuf>,
    PreflightAuthoring: Fn(&Path) -> Result<(), handbook_sdk::AuthorProjectContextRefusal>,
    RunAuthor: Fn(
        &Path,
        &handbook_sdk::ProjectContextInput,
    ) -> Result<
        handbook_sdk::AuthorProjectContextResult,
        handbook_sdk::AuthorProjectContextRefusal,
    >,
{
    let Some(path_or_dash) = args.from_inputs.as_deref() else {
        return RenderedCommand {
            output: render_author_custom_refusal(
                "author project-context",
                "REFUSED",
                "InvalidRequest",
                "`handbook author project-context` requires `--from-inputs <path|->`",
                "command arguments",
                "retry `handbook author project-context --from-inputs <path|->`",
            ),
            exit_code: ExitCode::from(1),
        };
    };

    let cwd = match get_current_dir() {
        Ok(dir) => dir,
        Err(err) => {
            return RenderedCommand {
                output: render_author_custom_refusal(
                    "author project-context",
                    "REFUSED",
                    "WorkingDirectoryUnavailable",
                    &format!("failed to determine repo root: {err}"),
                    "current working directory",
                    "repair the current working directory and retry `handbook author project-context --from-inputs <path|->`",
                ),
                exit_code: ExitCode::from(1),
            };
        }
    };
    let repo_root = discover_managed_repo_root(&cwd);

    let yaml = match read_author_inputs_source(
        "author project-context",
        "handbook author project-context --from-inputs",
        path_or_dash,
    ) {
        Ok(yaml) => yaml,
        Err(rendered) => {
            return RenderedCommand {
                output: rendered,
                exit_code: ExitCode::from(1),
            };
        }
    };
    let input = match parse_project_context_input_yaml(&yaml) {
        Ok(input) => input,
        Err(refusal) => {
            return RenderedCommand {
                output: render_project_context_refusal(&refusal),
                exit_code: ExitCode::from(1),
            };
        }
    };

    if let Err(refusal) =
        handbook_sdk::HandbookSdkV1::open(&repo_root).validate_project_context(&input)
    {
        return RenderedCommand {
            output: render_project_context_refusal(&refusal),
            exit_code: ExitCode::from(1),
        };
    }

    let input_mode = if path_or_dash == "-" {
        "structured_inputs_stdin"
    } else {
        "structured_inputs_file"
    };
    if args.validate {
        return RenderedCommand {
            output: render_author_project_context_validation_success(input_mode, path_or_dash),
            exit_code: ExitCode::SUCCESS,
        };
    }

    if let Err(refusal) = preflight_authoring(&repo_root) {
        return RenderedCommand {
            output: render_project_context_refusal(&refusal),
            exit_code: ExitCode::from(1),
        };
    }

    match run_author(&repo_root, &input) {
        Ok(result) => RenderedCommand {
            output: render_author_project_context_success(&result, input_mode, path_or_dash),
            exit_code: ExitCode::SUCCESS,
        },
        Err(refusal) => RenderedCommand {
            output: render_project_context_refusal(&refusal),
            exit_code: ExitCode::from(1),
        },
    }
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

fn read_author_inputs_source(
    object: &str,
    command_with_flag: &str,
    path_or_dash: &str,
) -> Result<String, String> {
    if path_or_dash == "-" {
        return read_stdin().map_err(|err| {
            render_author_custom_refusal(
                object,
                "REFUSED",
                "InputReadFailure",
                &format!("failed to read structured inputs from stdin: {err}"),
                "structured input source",
                &format!("repair stdin and retry `{command_with_flag} -`"),
            )
        });
    }

    fs::read_to_string(path_or_dash).map_err(|err| {
        render_author_custom_refusal(
            object,
            "REFUSED",
            "InputReadFailure",
            &format!("failed to read structured inputs from `{path_or_dash}`: {err}"),
            "structured input source",
            &format!("repair the structured input file and retry `{command_with_flag} <path|->`"),
        )
    })
}

fn author_project_context_refusal_outcome_name(
    kind: handbook_sdk::AuthorProjectContextRefusalKind,
) -> &'static str {
    match kind {
        handbook_sdk::AuthorProjectContextRefusalKind::MissingSystemRoot
        | handbook_sdk::AuthorProjectContextRefusalKind::InvalidSystemRoot
        | handbook_sdk::AuthorProjectContextRefusalKind::MutationRefused => "BLOCKED",
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
        handbook_sdk::AuthorProjectContextRefusalKind::MissingSystemRoot => "MissingSystemRoot",
        handbook_sdk::AuthorProjectContextRefusalKind::InvalidSystemRoot => "InvalidSystemRoot",
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
        handbook_sdk::AuthorProjectContextRefusalKind::UnsupportedPlatformStrictMutation => {
            "UnsupportedPlatformStrictMutation"
        }
    }
}
