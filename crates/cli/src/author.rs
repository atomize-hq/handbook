use crate::{
    shell_shared::{discover_managed_repo_root, read_stdin},
    AuthorArgs, AuthorCharterArgs, AuthorCommand, AuthorProjectContextArgs, CharterModeArg, Cli,
};
use std::fmt::Write as _;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

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
    if result.status == handbook_compiler::AdapterOperationStatus::Succeeded {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}

fn execute_selected_author_charter_command(
    args: AuthorCharterArgs,
) -> handbook_compiler::CharterOperationResult {
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
        return handbook_compiler::legacy_charter_input_refusal();
    }

    let operation = if approve_selected {
        handbook_compiler::CharterOperation::Approve
    } else if promote_selected {
        handbook_compiler::CharterOperation::Promote
    } else if validate_selected {
        handbook_compiler::CharterOperation::Validate
    } else {
        handbook_compiler::CharterOperation::Author
    };
    if !(author_selected || approve_selected || promote_selected || validate_selected) {
        return handbook_compiler::invalid_charter_command_refusal(operation);
    }

    let cwd = match std::env::current_dir() {
        Ok(cwd) => cwd,
        Err(_) => return handbook_compiler::invalid_charter_command_refusal(operation),
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
            Err(_) => return handbook_compiler::invalid_charter_command_refusal(operation),
        };
        let mut envelope = match handbook_compiler::parse_charter_intake_envelope(&yaml) {
            Ok(envelope) => envelope,
            Err(refusal) => return refusal,
        };
        if args.expected_current_fingerprint.is_some()
            && envelope.expected_current_fingerprint.is_some()
            && args.expected_current_fingerprint.as_deref()
                != envelope.expected_current_fingerprint.as_deref()
        {
            return handbook_compiler::invalid_charter_command_refusal(operation);
        }
        if args.expected_current_fingerprint.is_some() {
            envelope.expected_current_fingerprint = args.expected_current_fingerprint;
        }
        handbook_compiler::CharterCommandIntent::Author {
            mode: charter_mode(args.mode.expect("author selection requires a mode")),
            envelope,
        }
    } else if approve_selected {
        handbook_compiler::CharterCommandIntent::Approve {
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
        handbook_compiler::CharterCommandIntent::Promote {
            candidate_ref: args
                .promote_candidate
                .expect("promotion selection requires a candidate"),
            approval_ref: args
                .approval_ref
                .expect("promotion selection requires an approval"),
            expected_current_fingerprint: args.expected_current_fingerprint,
        }
    } else {
        handbook_compiler::CharterCommandIntent::Validate
    };
    handbook_compiler::execute_charter_command(repo_root, intent)
}

fn charter_mode(mode: CharterModeArg) -> handbook_engine::CharterAcquisitionMode {
    match mode {
        CharterModeArg::GuidedAdaptive => handbook_engine::CharterAcquisitionMode::GuidedAdaptive,
        CharterModeArg::Express => handbook_engine::CharterAcquisitionMode::Express,
        CharterModeArg::AgentAssisted => handbook_engine::CharterAcquisitionMode::AgentAssisted,
    }
}

fn render_charter_operation_json(result: &handbook_compiler::CharterOperationResult) -> String {
    let mut output = serde_json::to_string_pretty(result)
        .unwrap_or_else(|_| "{\"status\":\"refused\"}".to_owned());
    output.push('\n');
    output
}

fn render_charter_operation_text(result: &handbook_compiler::CharterOperationResult) -> String {
    let status = match result.status {
        handbook_compiler::AdapterOperationStatus::Succeeded => "SUCCEEDED",
        handbook_compiler::AdapterOperationStatus::Refused => "REFUSED",
    };
    let operation = match result.operation {
        handbook_compiler::CharterOperation::Author => "author",
        handbook_compiler::CharterOperation::Approve => "approve",
        handbook_compiler::CharterOperation::Promote => "promote",
        handbook_compiler::CharterOperation::Validate => "validate",
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

fn author_project_context_command(args: AuthorProjectContextArgs) -> ExitCode {
    let rendered = execute_author_project_context_command(
        args,
        std::env::current_dir,
        |repo_root| handbook_compiler::preflight_author_project_context(repo_root),
        |repo_root, input| handbook_compiler::author_project_context_from_input(repo_root, input),
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
    PreflightAuthoring: Fn(&Path) -> Result<(), handbook_compiler::AuthorProjectContextRefusal>,
    RunAuthor: Fn(
        &Path,
        &handbook_engine::CanonicalProjectContext,
    ) -> Result<
        handbook_compiler::AuthorProjectContextResult,
        handbook_compiler::AuthorProjectContextRefusal,
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
    let input = match handbook_compiler::parse_project_context_input_yaml(&yaml) {
        Ok(input) => input,
        Err(refusal) => {
            return RenderedCommand {
                output: render_project_context_refusal(&refusal),
                exit_code: ExitCode::from(1),
            };
        }
    };

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
    result: &handbook_compiler::AuthorProjectContextResult,
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

fn render_project_context_refusal(
    refusal: &handbook_compiler::AuthorProjectContextRefusal,
) -> String {
    render_author_simple_refusal(
        "author project-context",
        author_project_context_refusal_outcome_name(refusal.kind),
        author_project_context_refusal_kind_name(refusal.kind),
        refusal.summary.trim(),
        refusal.broken_subject.trim(),
        refusal.next_safe_action.trim(),
    )
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
    kind: handbook_compiler::AuthorProjectContextRefusalKind,
) -> &'static str {
    match kind {
        handbook_compiler::AuthorProjectContextRefusalKind::MissingSystemRoot
        | handbook_compiler::AuthorProjectContextRefusalKind::InvalidSystemRoot
        | handbook_compiler::AuthorProjectContextRefusalKind::MutationRefused => "BLOCKED",
        handbook_compiler::AuthorProjectContextRefusalKind::MalformedStructuredInput
        | handbook_compiler::AuthorProjectContextRefusalKind::IncompleteStructuredInput
        | handbook_compiler::AuthorProjectContextRefusalKind::ExistingCanonicalTruth
        | handbook_compiler::AuthorProjectContextRefusalKind::UnsupportedPlatformStrictMutation => {
            "REFUSED"
        }
    }
}

fn author_project_context_refusal_kind_name(
    kind: handbook_compiler::AuthorProjectContextRefusalKind,
) -> &'static str {
    match kind {
        handbook_compiler::AuthorProjectContextRefusalKind::MissingSystemRoot => {
            "MissingSystemRoot"
        }
        handbook_compiler::AuthorProjectContextRefusalKind::InvalidSystemRoot => {
            "InvalidSystemRoot"
        }
        handbook_compiler::AuthorProjectContextRefusalKind::MalformedStructuredInput => {
            "MalformedStructuredInput"
        }
        handbook_compiler::AuthorProjectContextRefusalKind::IncompleteStructuredInput => {
            "IncompleteStructuredInput"
        }
        handbook_compiler::AuthorProjectContextRefusalKind::ExistingCanonicalTruth => {
            "ExistingCanonicalTruth"
        }
        handbook_compiler::AuthorProjectContextRefusalKind::MutationRefused => "MutationRefused",
        handbook_compiler::AuthorProjectContextRefusalKind::UnsupportedPlatformStrictMutation => {
            "UnsupportedPlatformStrictMutation"
        }
    }
}
