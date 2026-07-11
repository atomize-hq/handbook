use crate::{
    shell_shared::{discover_managed_repo_root, read_stdin},
    AuthorArgs, AuthorCharterArgs, AuthorCommand, AuthorEnvironmentInventoryArgs,
    AuthorProjectContextArgs, Cli,
};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

pub(crate) fn run(args: AuthorArgs) -> ExitCode {
    match args.command {
        Some(AuthorCommand::Charter(args)) => author_charter_command(args),
        Some(AuthorCommand::ProjectContext(args)) => author_project_context_command(args),
        Some(AuthorCommand::EnvironmentInventory(args)) => {
            author_environment_inventory_command(args)
        }
        None => crate::shell_shared::print_subcommand_help::<Cli>(&["author"]),
    }
}

pub(crate) struct RenderedCommand {
    pub(crate) output: String,
    pub(crate) exit_code: ExitCode,
}

fn author_charter_command(args: AuthorCharterArgs) -> ExitCode {
    let rendered = execute_author_charter_command(
        args,
        std::env::current_dir,
        |repo_root, input| handbook_compiler::preflight_author_charter_from_input(repo_root, input),
        |repo_root, input| handbook_compiler::author_charter(repo_root, input),
    );
    println!("{}", rendered.output);
    rendered.exit_code
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

fn author_environment_inventory_command(args: AuthorEnvironmentInventoryArgs) -> ExitCode {
    let Some(path_or_dash) = args.from_inputs.as_deref() else {
        println!(
            "{}",
            render_author_simple_refusal(
                "author environment-inventory",
                "REFUSED",
                "InvalidRequest",
                "`handbook author environment-inventory` requires `--from-inputs <path|->`",
                "command arguments",
                "retry `handbook author environment-inventory --from-inputs <path|->`",
            )
        );
        return ExitCode::from(1);
    };

    let cwd = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(err) => {
            println!(
                "{}",
                render_author_simple_refusal(
                    "author environment-inventory",
                    "REFUSED",
                    "WorkingDirectoryUnavailable",
                    &format!("failed to determine repo root: {err}"),
                    "current working directory",
                    "repair the current working directory and retry `handbook author environment-inventory --from-inputs <path|->`",
                )
            );
            return ExitCode::from(1);
        }
    };
    let repo_root = discover_managed_repo_root(&cwd);

    let yaml = match read_author_inputs_source(
        "author environment-inventory",
        "handbook author environment-inventory --from-inputs",
        path_or_dash,
    ) {
        Ok(yaml) => yaml,
        Err(rendered) => {
            println!("{rendered}");
            return ExitCode::from(1);
        }
    };
    let input = match handbook_compiler::parse_environment_inventory_structured_input_yaml(&yaml) {
        Ok(input) => input,
        Err(refusal) => {
            println!("{}", render_environment_inventory_refusal(&refusal));
            return ExitCode::from(1);
        }
    };
    if let Err(refusal) =
        handbook_compiler::preflight_author_environment_inventory_from_input(&repo_root, &input)
    {
        println!("{}", render_environment_inventory_refusal(&refusal));
        return ExitCode::from(1);
    }

    let input_mode = if path_or_dash == "-" {
        "structured_inputs_stdin"
    } else {
        "structured_inputs_file"
    };
    if args.validate {
        println!(
            "{}",
            render_author_environment_inventory_validation_success(input_mode, path_or_dash)
        );
        return ExitCode::SUCCESS;
    }

    match handbook_compiler::author_environment_inventory_from_input(&repo_root, &input) {
        Ok(result) => {
            println!(
                "{}",
                render_author_environment_inventory_success(&result, input_mode, path_or_dash,)
            );
            ExitCode::SUCCESS
        }
        Err(refusal) => {
            println!("{}", render_environment_inventory_refusal(&refusal));
            ExitCode::from(1)
        }
    }
}

pub(crate) fn execute_author_charter_command<
    GetCurrentDir,
    PreflightFromInput,
    RunDeterministicAuthor,
>(
    args: AuthorCharterArgs,
    get_current_dir: GetCurrentDir,
    preflight_from_input: PreflightFromInput,
    run_deterministic_author: RunDeterministicAuthor,
) -> RenderedCommand
where
    GetCurrentDir: FnOnce() -> io::Result<PathBuf>,
    PreflightFromInput: Fn(
        &Path,
        &handbook_engine::CharterStructuredInput,
    ) -> Result<(), handbook_compiler::AuthorCharterRefusal>,
    RunDeterministicAuthor: Fn(
        &Path,
        &handbook_engine::CharterStructuredInput,
    ) -> Result<
        handbook_compiler::AuthorCharterResult,
        handbook_compiler::AuthorCharterRefusal,
    >,
{
    let Some(path_or_dash) = args.from_inputs.as_deref() else {
        let (summary, next_safe_action) = if args.validate {
            (
                "`handbook author charter --validate` requires `--from-inputs <path|->`",
                "retry `handbook author charter --validate --from-inputs <path|->`",
            )
        } else {
            (
                "`handbook author charter` requires `--from-inputs <path|->`",
                "retry `handbook author charter --from-inputs <path|->`",
            )
        };
        return RenderedCommand {
            output: render_author_custom_refusal(
                "author charter",
                "REFUSED",
                "InvalidRequest",
                summary,
                "command arguments",
                next_safe_action,
            ),
            exit_code: ExitCode::from(1),
        };
    };

    let cwd = match get_current_dir() {
        Ok(dir) => dir,
        Err(err) => {
            return RenderedCommand {
                output: render_author_custom_refusal(
                    "author charter",
                    "REFUSED",
                    "WorkingDirectoryUnavailable",
                    &format!("failed to determine repo root: {err}"),
                    "current working directory",
                    "repair the current working directory and retry `handbook author charter --from-inputs <path|->`",
                ),
                exit_code: ExitCode::from(1),
            };
        }
    };
    let repo_root = discover_managed_repo_root(&cwd);

    let yaml = match read_author_inputs_source(
        "author charter",
        "handbook author charter --from-inputs",
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
    let input = match handbook_compiler::parse_charter_structured_input_yaml(&yaml) {
        Ok(input) => input,
        Err(refusal) => {
            return RenderedCommand {
                output: render_author_charter_refusal(&refusal),
                exit_code: ExitCode::from(1),
            };
        }
    };

    if let Err(refusal) = preflight_from_input(&repo_root, &input) {
        return RenderedCommand {
            output: render_author_charter_refusal(&refusal),
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
            output: render_author_charter_validation_success(input_mode, path_or_dash),
            exit_code: ExitCode::SUCCESS,
        };
    }

    match run_deterministic_author(&repo_root, &input) {
        Ok(result) => RenderedCommand {
            output: render_author_charter_success(&result, input_mode, path_or_dash),
            exit_code: ExitCode::SUCCESS,
        },
        Err(refusal) => RenderedCommand {
            output: render_author_charter_refusal(&refusal),
            exit_code: ExitCode::from(1),
        },
    }
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
        &handbook_engine::ProjectContextStructuredInput,
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
    let input = match handbook_compiler::parse_project_context_structured_input_yaml(&yaml) {
        Ok(input) => input,
        Err(refusal) => {
            return RenderedCommand {
                output: render_project_context_refusal(&refusal),
                exit_code: ExitCode::from(1),
            };
        }
    };

    if let Err(refusal) = preflight_authoring(&repo_root) {
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

fn render_author_charter_success(
    result: &handbook_compiler::AuthorCharterResult,
    input_mode: &str,
    input_source: &str,
) -> String {
    let mut out = String::new();
    out.push_str("OUTCOME: AUTHORED\n");
    out.push_str("OBJECT: author charter\n");
    out.push_str("NEXT SAFE ACTION: run `handbook doctor`\n");
    out.push_str("## CANONICAL ARTIFACT\n");
    out.push_str(&format!("PATH: {}\n", result.canonical_repo_relative_path));
    out.push_str(&format!("BYTES WRITTEN: {}\n", result.bytes_written));
    out.push_str("## INPUT MODE\n");
    out.push_str(&format!("MODE: {input_mode}\n"));
    out.push_str(&format!("SOURCE: {input_source}\n"));
    out.trim_end().to_string()
}

fn render_author_charter_validation_success(input_mode: &str, input_source: &str) -> String {
    let mut out = String::new();
    out.push_str("OUTCOME: VALIDATED\n");
    out.push_str("OBJECT: author charter\n");
    out.push_str("NEXT SAFE ACTION: run `handbook author charter --from-inputs <path|->`\n");
    out.push_str("## INPUT MODE\n");
    out.push_str(&format!("MODE: {input_mode}\n"));
    out.push_str(&format!("SOURCE: {input_source}\n"));
    out.push_str("## SUMMARY\n");
    out.push_str(
        "Structured charter inputs and repo write preconditions validated without mutation.",
    );
    out.trim_end().to_string()
}

pub(crate) fn render_author_charter_refusal(
    refusal: &handbook_compiler::AuthorCharterRefusal,
) -> String {
    render_author_custom_refusal(
        "author charter",
        author_refusal_outcome_name(refusal.kind),
        author_refusal_kind_name(refusal.kind),
        refusal.summary.trim(),
        refusal.broken_subject.trim(),
        refusal.next_safe_action.trim(),
    )
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
        "Structured project-context inputs and repo write preconditions validated without mutation.",
    );
    out.trim_end().to_string()
}

fn render_author_environment_inventory_success(
    result: &handbook_compiler::AuthorEnvironmentInventoryResult,
    input_mode: &str,
    input_source: &str,
) -> String {
    let mut out = String::new();
    out.push_str("OUTCOME: AUTHORED\n");
    out.push_str("OBJECT: author environment-inventory\n");
    out.push_str("NEXT SAFE ACTION: run `handbook doctor`\n");
    out.push_str("## CANONICAL ARTIFACT\n");
    out.push_str(&format!("PATH: {}\n", result.canonical_repo_relative_path));
    out.push_str(&format!("BYTES WRITTEN: {}\n", result.bytes_written));
    out.push_str("Wrote canonical environment inventory to .handbook/environment_inventory/ENVIRONMENT_INVENTORY.md\n");
    out.push_str("## INPUT MODE\n");
    out.push_str(&format!("MODE: {input_mode}\n"));
    out.push_str(&format!("SOURCE: {input_source}\n"));
    out.trim_end().to_string()
}

fn render_author_environment_inventory_validation_success(
    input_mode: &str,
    input_source: &str,
) -> String {
    let mut out = String::new();
    out.push_str("OUTCOME: VALIDATED\n");
    out.push_str("OBJECT: author environment-inventory\n");
    out.push_str(
        "NEXT SAFE ACTION: run `handbook author environment-inventory --from-inputs <path|->`\n",
    );
    out.push_str("## INPUT MODE\n");
    out.push_str(&format!("MODE: {input_mode}\n"));
    out.push_str(&format!("SOURCE: {input_source}\n"));
    out.push_str("## SUMMARY\n");
    out.push_str(
        "Structured environment-inventory inputs and repo write preconditions validated without mutation.",
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

fn render_environment_inventory_refusal(
    refusal: &handbook_compiler::AuthorEnvironmentInventoryRefusal,
) -> String {
    render_author_simple_refusal(
        "author environment-inventory",
        author_environment_inventory_refusal_outcome_name(refusal.kind),
        author_environment_inventory_refusal_kind_name(refusal.kind),
        refusal.summary.trim(),
        refusal.broken_subject.trim(),
        refusal.next_safe_action.trim(),
    )
}

fn author_refusal_outcome_name(kind: handbook_compiler::AuthorCharterRefusalKind) -> &'static str {
    match kind {
        handbook_compiler::AuthorCharterRefusalKind::MissingSystemRoot
        | handbook_compiler::AuthorCharterRefusalKind::InvalidSystemRoot
        | handbook_compiler::AuthorCharterRefusalKind::MutationRefused
        | handbook_compiler::AuthorCharterRefusalKind::SynthesisFailed => "BLOCKED",
        handbook_compiler::AuthorCharterRefusalKind::MalformedStructuredInput
        | handbook_compiler::AuthorCharterRefusalKind::IncompleteStructuredInput
        | handbook_compiler::AuthorCharterRefusalKind::ExistingCanonicalTruth => "REFUSED",
    }
}

fn author_refusal_kind_name(kind: handbook_compiler::AuthorCharterRefusalKind) -> &'static str {
    match kind {
        handbook_compiler::AuthorCharterRefusalKind::MissingSystemRoot => "MissingSystemRoot",
        handbook_compiler::AuthorCharterRefusalKind::InvalidSystemRoot => "InvalidSystemRoot",
        handbook_compiler::AuthorCharterRefusalKind::MalformedStructuredInput => {
            "MalformedStructuredInput"
        }
        handbook_compiler::AuthorCharterRefusalKind::IncompleteStructuredInput => {
            "IncompleteStructuredInput"
        }
        handbook_compiler::AuthorCharterRefusalKind::ExistingCanonicalTruth => {
            "ExistingCanonicalTruth"
        }
        handbook_compiler::AuthorCharterRefusalKind::MutationRefused => "MutationRefused",
        handbook_compiler::AuthorCharterRefusalKind::SynthesisFailed => "SynthesisFailed",
    }
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
        | handbook_compiler::AuthorProjectContextRefusalKind::ExistingCanonicalTruth => "REFUSED",
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
    }
}

fn author_environment_inventory_refusal_outcome_name(
    kind: handbook_compiler::AuthorEnvironmentInventoryRefusalKind,
) -> &'static str {
    match kind {
        handbook_compiler::AuthorEnvironmentInventoryRefusalKind::MissingSystemRoot
        | handbook_compiler::AuthorEnvironmentInventoryRefusalKind::InvalidSystemRoot
        | handbook_compiler::AuthorEnvironmentInventoryRefusalKind::MutationRefused
        | handbook_compiler::AuthorEnvironmentInventoryRefusalKind::SynthesisFailed => "BLOCKED",
        handbook_compiler::AuthorEnvironmentInventoryRefusalKind::MalformedStructuredInput
        | handbook_compiler::AuthorEnvironmentInventoryRefusalKind::IncompleteStructuredInput
        | handbook_compiler::AuthorEnvironmentInventoryRefusalKind::MissingRequiredCharter
        | handbook_compiler::AuthorEnvironmentInventoryRefusalKind::InvalidUpstreamCanonicalTruth
        | handbook_compiler::AuthorEnvironmentInventoryRefusalKind::ExistingCanonicalTruth => {
            "REFUSED"
        }
    }
}

fn author_environment_inventory_refusal_kind_name(
    kind: handbook_compiler::AuthorEnvironmentInventoryRefusalKind,
) -> &'static str {
    match kind {
        handbook_compiler::AuthorEnvironmentInventoryRefusalKind::MissingSystemRoot => {
            "MissingSystemRoot"
        }
        handbook_compiler::AuthorEnvironmentInventoryRefusalKind::InvalidSystemRoot => {
            "InvalidSystemRoot"
        }
        handbook_compiler::AuthorEnvironmentInventoryRefusalKind::MalformedStructuredInput => {
            "MalformedStructuredInput"
        }
        handbook_compiler::AuthorEnvironmentInventoryRefusalKind::IncompleteStructuredInput => {
            "IncompleteStructuredInput"
        }
        handbook_compiler::AuthorEnvironmentInventoryRefusalKind::MissingRequiredCharter => {
            "MissingRequiredCharter"
        }
        handbook_compiler::AuthorEnvironmentInventoryRefusalKind::InvalidUpstreamCanonicalTruth => {
            "InvalidUpstreamCanonicalTruth"
        }
        handbook_compiler::AuthorEnvironmentInventoryRefusalKind::ExistingCanonicalTruth => {
            "ExistingCanonicalTruth"
        }
        handbook_compiler::AuthorEnvironmentInventoryRefusalKind::MutationRefused => {
            "MutationRefused"
        }
        handbook_compiler::AuthorEnvironmentInventoryRefusalKind::SynthesisFailed => {
            "SynthesisFailed"
        }
    }
}
