use crate::{
    discover_managed_repo_root, read_stdin, AuthorArgs, AuthorCharterArgs, AuthorCommand,
    AuthorProjectContextArgs, Cli,
};
use std::cell::RefCell;
use std::fs;
use std::io::{self, IsTerminal, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

pub(crate) fn run(args: AuthorArgs) -> ExitCode {
    match args.command {
        Some(AuthorCommand::Charter(args)) => author_charter_command(args),
        Some(AuthorCommand::ProjectContext(args)) => author_project_context_command(args),
        Some(AuthorCommand::EnvironmentInventory) => author_environment_inventory_command(),
        None => crate::shell_shared::print_subcommand_help::<Cli>(&["author"]),
    }
}

struct GuidedPromptContext {
    object: &'static str,
    interview_name: &'static str,
    broken_subject: &'static str,
    retry_command: &'static str,
    restart_or_from_inputs: &'static str,
}

const CHARTER_PROMPT_CONTEXT: GuidedPromptContext = GuidedPromptContext {
    object: "author charter",
    interview_name: "guided charter interview",
    broken_subject: "structured charter input",
    retry_command: "repair the interactive terminal and retry `handbook author charter`",
    restart_or_from_inputs:
        "restart `handbook author charter` or use `handbook author charter --from-inputs <path|->`",
};

const PROJECT_CONTEXT_PROMPT_CONTEXT: GuidedPromptContext = GuidedPromptContext {
    object: "author project-context",
    interview_name: "guided project-context interview",
    broken_subject: "structured project-context input",
    retry_command: "repair the interactive terminal and retry `handbook author project-context`",
    restart_or_from_inputs:
        "restart `handbook author project-context` or use `handbook author project-context --from-inputs <path|->`",
};

thread_local! {
    static GUIDED_PROMPT_CONTEXT: RefCell<&'static GuidedPromptContext> =
        const { RefCell::new(&CHARTER_PROMPT_CONTEXT) };
}

struct GuidedPromptContextGuard {
    previous: &'static GuidedPromptContext,
}

impl GuidedPromptContextGuard {
    fn push(next: &'static GuidedPromptContext) -> Self {
        let previous = GUIDED_PROMPT_CONTEXT.with(|slot| {
            let previous = *slot.borrow();
            *slot.borrow_mut() = next;
            previous
        });
        Self { previous }
    }
}

impl Drop for GuidedPromptContextGuard {
    fn drop(&mut self) {
        GUIDED_PROMPT_CONTEXT.with(|slot| {
            *slot.borrow_mut() = self.previous;
        });
    }
}

fn current_guided_prompt_context() -> &'static GuidedPromptContext {
    GUIDED_PROMPT_CONTEXT.with(|slot| *slot.borrow())
}

pub(crate) struct RenderedCommand {
    pub(crate) output: String,
    pub(crate) exit_code: ExitCode,
}

fn author_charter_command(args: AuthorCharterArgs) -> ExitCode {
    let rendered = execute_author_charter_command(
        args,
        std::env::current_dir,
        interactive_authoring_is_allowed,
        |repo_root| handbook_compiler::preflight_author_charter(repo_root),
        |repo_root, input| handbook_compiler::preflight_author_charter_from_input(repo_root, input),
        collect_guided_charter_input,
        |repo_root, input| handbook_compiler::author_charter_guided(repo_root, input),
        |repo_root, input| handbook_compiler::author_charter(repo_root, input),
    );
    println!("{}", rendered.output);
    rendered.exit_code
}

fn author_project_context_command(args: AuthorProjectContextArgs) -> ExitCode {
    let rendered = execute_author_project_context_command(
        args,
        std::env::current_dir,
        interactive_authoring_is_allowed,
        |repo_root| handbook_compiler::preflight_author_project_context(repo_root),
        collect_guided_project_context_input,
        |repo_root, input| handbook_compiler::author_project_context_from_input(repo_root, input),
    );
    println!("{}", rendered.output);
    rendered.exit_code
}

fn author_environment_inventory_command() -> ExitCode {
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
                    "repair the current working directory and retry `handbook author environment-inventory`",
                )
            );
            return ExitCode::from(1);
        }
    };
    let repo_root = discover_managed_repo_root(&cwd);
    if let Err(refusal) = handbook_compiler::preflight_author_environment_inventory(&repo_root) {
        println!("{}", render_environment_inventory_refusal(&refusal));
        return ExitCode::from(1);
    }
    match handbook_compiler::author_environment_inventory(&repo_root) {
        Ok(result) => {
            println!(
                "{}",
                render_author_simple_success(
                    "author environment-inventory",
                    result.canonical_repo_relative_path,
                    result.bytes_written,
                    "Wrote canonical environment inventory to .handbook/environment_inventory/ENVIRONMENT_INVENTORY.md",
                )
            );
            ExitCode::SUCCESS
        }
        Err(refusal) => {
            println!("{}", render_environment_inventory_refusal(&refusal));
            ExitCode::from(1)
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn execute_author_charter_command<
    GetCurrentDir,
    InteractiveAllowed,
    PreflightGuided,
    PreflightFromInput,
    CollectGuidedInput,
    RunGuidedAuthor,
    RunDeterministicAuthor,
>(
    args: AuthorCharterArgs,
    get_current_dir: GetCurrentDir,
    interactive_allowed: InteractiveAllowed,
    preflight_guided: PreflightGuided,
    preflight_from_input: PreflightFromInput,
    collect_guided_input: CollectGuidedInput,
    run_guided_author: RunGuidedAuthor,
    run_deterministic_author: RunDeterministicAuthor,
) -> RenderedCommand
where
    GetCurrentDir: FnOnce() -> io::Result<PathBuf>,
    InteractiveAllowed: Fn() -> bool,
    PreflightGuided: Fn(&Path) -> Result<(), handbook_compiler::AuthorCharterRefusal>,
    PreflightFromInput: Fn(
        &Path,
        &handbook_engine::CharterStructuredInput,
    ) -> Result<(), handbook_compiler::AuthorCharterRefusal>,
    CollectGuidedInput: Fn() -> Result<handbook_engine::CharterStructuredInput, String>,
    RunGuidedAuthor: Fn(
        &Path,
        &handbook_engine::CharterStructuredInput,
    ) -> Result<
        handbook_compiler::AuthorCharterResult,
        handbook_compiler::AuthorCharterRefusal,
    >,
    RunDeterministicAuthor: Fn(
        &Path,
        &handbook_engine::CharterStructuredInput,
    ) -> Result<
        handbook_compiler::AuthorCharterResult,
        handbook_compiler::AuthorCharterRefusal,
    >,
{
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
                    "repair the current working directory and retry `handbook author charter`",
                ),
                exit_code: ExitCode::from(1),
            };
        }
    };
    let repo_root = discover_managed_repo_root(&cwd);

    if args.validate && args.from_inputs.is_none() {
        return RenderedCommand {
            output: render_author_custom_refusal(
                "author charter",
                "REFUSED",
                "InvalidRequest",
                "`handbook author charter --validate` requires `--from-inputs <path|->`",
                "command arguments",
                "retry `handbook author charter --validate --from-inputs <path|->`",
            ),
            exit_code: ExitCode::from(1),
        };
    }

    if args.from_inputs.is_none() && !interactive_allowed() {
        return RenderedCommand {
            output: render_author_custom_refusal(
                "author charter",
                "REFUSED",
                "NonInteractiveRefusal",
                "`handbook author charter` is a TTY-only guided interview",
                "interactive terminal",
                "run `handbook author charter --from-inputs <path|->`",
            ),
            exit_code: ExitCode::from(1),
        };
    }

    if args.from_inputs.is_none() {
        if let Err(refusal) = preflight_guided(&repo_root) {
            return RenderedCommand {
                output: render_author_charter_refusal(&refusal),
                exit_code: ExitCode::from(1),
            };
        }
    }

    let (input, input_mode, input_source, guided_mode) = match args.from_inputs.as_deref() {
        Some(path_or_dash) => {
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
            let input = match handbook_engine::parse_charter_structured_input_yaml(&yaml) {
                Ok(input) => input,
                Err(err) => {
                    let refusal = map_engine_charter_core_error(err);
                    return RenderedCommand {
                        output: render_author_charter_refusal(&refusal),
                        exit_code: ExitCode::from(1),
                    };
                }
            };
            let input_mode = if path_or_dash == "-" {
                "structured_inputs_stdin"
            } else {
                "structured_inputs_file"
            };
            (input, input_mode, path_or_dash.to_string(), false)
        }
        None => {
            let input = match collect_guided_input() {
                Ok(input) => input,
                Err(rendered) => {
                    return RenderedCommand {
                        output: rendered,
                        exit_code: ExitCode::from(1),
                    };
                }
            };
            (
                input,
                "guided_interview",
                "interactive terminal".to_string(),
                true,
            )
        }
    };

    if !guided_mode {
        if let Err(refusal) = preflight_from_input(&repo_root, &input) {
            return RenderedCommand {
                output: render_author_charter_refusal(&refusal),
                exit_code: ExitCode::from(1),
            };
        }
        if args.validate {
            return RenderedCommand {
                output: render_author_charter_validation_success(input_mode, &input_source),
                exit_code: ExitCode::SUCCESS,
            };
        }
    }

    let result = if guided_mode {
        run_guided_author(&repo_root, &input)
    } else {
        run_deterministic_author(&repo_root, &input)
    };

    match result {
        Ok(result) => RenderedCommand {
            output: render_author_charter_success(&result, input_mode, &input_source),
            exit_code: ExitCode::SUCCESS,
        },
        Err(refusal) => RenderedCommand {
            output: render_author_charter_refusal(&refusal),
            exit_code: ExitCode::from(1),
        },
    }
}

fn execute_author_project_context_command<
    GetCurrentDir,
    InteractiveAllowed,
    PreflightAuthoring,
    CollectGuidedInput,
    RunAuthor,
>(
    args: AuthorProjectContextArgs,
    get_current_dir: GetCurrentDir,
    interactive_allowed: InteractiveAllowed,
    preflight_authoring: PreflightAuthoring,
    collect_guided_input: CollectGuidedInput,
    run_author: RunAuthor,
) -> RenderedCommand
where
    GetCurrentDir: FnOnce() -> io::Result<PathBuf>,
    InteractiveAllowed: Fn() -> bool,
    PreflightAuthoring: Fn(&Path) -> Result<(), handbook_compiler::AuthorProjectContextRefusal>,
    CollectGuidedInput: Fn(&Path) -> Result<handbook_engine::ProjectContextStructuredInput, String>,
    RunAuthor: Fn(
        &Path,
        &handbook_engine::ProjectContextStructuredInput,
    ) -> Result<
        handbook_compiler::AuthorProjectContextResult,
        handbook_compiler::AuthorProjectContextRefusal,
    >,
{
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
                    "repair the current working directory and retry `handbook author project-context`",
                ),
                exit_code: ExitCode::from(1),
            };
        }
    };
    let repo_root = discover_managed_repo_root(&cwd);

    if args.from_inputs.is_none() && !interactive_allowed() {
        return RenderedCommand {
            output: render_author_custom_refusal(
                "author project-context",
                "REFUSED",
                "NonInteractiveRefusal",
                "`handbook author project-context` is a TTY-only guided interview",
                "interactive terminal",
                "run `handbook author project-context --from-inputs <path|->`",
            ),
            exit_code: ExitCode::from(1),
        };
    }

    if let Err(refusal) = preflight_authoring(&repo_root) {
        return RenderedCommand {
            output: render_project_context_refusal(&refusal),
            exit_code: ExitCode::from(1),
        };
    }

    let (input, input_mode, input_source) = match args.from_inputs.as_deref() {
        Some(path_or_dash) => {
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
            let input = match handbook_engine::parse_project_context_structured_input_yaml(&yaml) {
                Ok(input) => input,
                Err(err) => {
                    let refusal = map_engine_project_context_core_error(err);
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
            (input, input_mode, path_or_dash.to_string())
        }
        None => {
            let input = match collect_guided_input(&repo_root) {
                Ok(input) => input,
                Err(rendered) => {
                    return RenderedCommand {
                        output: rendered,
                        exit_code: ExitCode::from(1),
                    };
                }
            };
            (
                input,
                "guided_interview",
                "interactive terminal".to_string(),
            )
        }
    };

    match run_author(&repo_root, &input) {
        Ok(result) => RenderedCommand {
            output: render_author_project_context_success(&result, input_mode, &input_source),
            exit_code: ExitCode::SUCCESS,
        },
        Err(refusal) => RenderedCommand {
            output: render_project_context_refusal(&refusal),
            exit_code: ExitCode::from(1),
        },
    }
}

fn interactive_authoring_is_allowed() -> bool {
    io::stdin().is_terminal() && io::stdout().is_terminal()
}

fn map_engine_charter_core_error(
    err: handbook_engine::CharterCoreError,
) -> handbook_compiler::AuthorCharterRefusal {
    match err.kind {
        handbook_engine::CharterCoreErrorKind::MalformedStructuredInput => {
            handbook_compiler::AuthorCharterRefusal {
                kind: handbook_compiler::AuthorCharterRefusalKind::MalformedStructuredInput,
                summary: err.summary,
                broken_subject: "structured charter input".to_string(),
                next_safe_action:
                    "repair the structured charter input and retry `handbook author charter --from-inputs <path|->`"
                        .to_string(),
            }
        }
        handbook_engine::CharterCoreErrorKind::IncompleteStructuredInput => {
            handbook_compiler::AuthorCharterRefusal {
                kind: handbook_compiler::AuthorCharterRefusalKind::IncompleteStructuredInput,
                summary: err.summary,
                broken_subject: "structured charter input".to_string(),
                next_safe_action:
                    "repair the structured charter input and retry `handbook author charter --from-inputs <path|->`"
                        .to_string(),
            }
        }
        handbook_engine::CharterCoreErrorKind::DeterministicRenderFailed => {
            handbook_compiler::AuthorCharterRefusal {
                kind: handbook_compiler::AuthorCharterRefusalKind::SynthesisFailed,
                summary: err.summary,
                broken_subject: "final charter render".to_string(),
                next_safe_action:
                    "repair the structured charter input or compiler-owned charter render path and retry `handbook author charter --from-inputs <path|->`"
                        .to_string(),
            }
        }
    }
}

fn map_engine_project_context_core_error(
    err: handbook_engine::ProjectContextCoreError,
) -> handbook_compiler::AuthorProjectContextRefusal {
    match err.kind {
        handbook_engine::ProjectContextCoreErrorKind::MalformedStructuredInput => {
            handbook_compiler::AuthorProjectContextRefusal {
                kind: handbook_compiler::AuthorProjectContextRefusalKind::MalformedStructuredInput,
                summary: err.summary,
                broken_subject: "structured project-context input".to_string(),
                next_safe_action:
                    "repair the structured project-context input and retry `handbook author project-context --from-inputs <path|->`"
                        .to_string(),
            }
        }
        handbook_engine::ProjectContextCoreErrorKind::IncompleteStructuredInput
        | handbook_engine::ProjectContextCoreErrorKind::DeterministicRenderFailed => {
            handbook_compiler::AuthorProjectContextRefusal {
                kind: handbook_compiler::AuthorProjectContextRefusalKind::IncompleteStructuredInput,
                summary: err.summary,
                broken_subject: "structured project-context input".to_string(),
                next_safe_action:
                    "repair the structured project-context input and retry `handbook author project-context --from-inputs <path|->`"
                        .to_string(),
            }
        }
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

fn render_author_simple_success(
    object: &str,
    canonical_repo_relative_path: &str,
    bytes_written: usize,
    summary: &str,
) -> String {
    let mut out = String::new();
    out.push_str("OUTCOME: AUTHORED\n");
    out.push_str(&format!("OBJECT: {object}\n"));
    out.push_str("NEXT SAFE ACTION: run `handbook doctor`\n");
    out.push_str("## CANONICAL ARTIFACT\n");
    out.push_str(&format!("PATH: {canonical_repo_relative_path}\n"));
    out.push_str(&format!("BYTES WRITTEN: {bytes_written}\n"));
    out.push_str("## SUMMARY\n");
    out.push_str(summary);
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

fn collect_guided_charter_input() -> Result<handbook_engine::CharterStructuredInput, String> {
    println!("Guided charter interview");
    println!("Answer with the documented value form. Comma-separated prompts accept `a, b, c`.");

    let project_name = prompt_required_concrete(
        "Project name",
        "Project name needs a concrete handbook name, not a placeholder",
        "project name",
    )?;
    let classification = prompt_choice(
        "Project classification [greenfield|brownfield|integration|modernization|hardening]",
        parse_project_classification,
    )?;
    let team_size = prompt_u32("Team size (> 0)")?;
    let users = prompt_choice("Audience [internal|external|mixed]", parse_audience)?;
    let expected_lifetime = prompt_choice(
        "Expected lifetime [days|weeks|months|years]",
        parse_expected_lifetime,
    )?;
    let surfaces = prompt_csv_choice(
        "Surfaces [web_app, api, cli, lib, infra, ml]",
        parse_surface,
    )?;
    let runtime_environments = prompt_csv_choice(
        "Runtime environments [browser, server, cloud, on_prem, edge]",
        parse_runtime_environment,
    )?;
    let deadline = prompt_optional("Deadline or delivery window")?;
    let budget = prompt_optional("Budget notes")?;
    let experience_notes = prompt_required_concrete(
        "Experience notes",
        "Experience notes need a concrete summary of team experience or delivery constraints",
        "experience notes",
    )?;
    let must_use_tech = prompt_csv_optional("Must-use tech (comma-separated, optional)")?;
    let in_production_today = prompt_bool("In production today? [yes|no]")?;
    let prod_users_or_data = prompt_optional("Production users or data notes")?;
    let external_contracts_to_preserve =
        prompt_csv_optional("External contracts to preserve (comma-separated, optional)")?;
    let uptime_expectations = prompt_optional("Uptime expectations")?;
    let baseline_level = prompt_u8_in_range("Baseline rubric level [1-5]", 1, 5)?;
    let baseline_rationale = prompt_csv_non_empty_concrete(
        "Baseline rationale (comma-separated, at least one)",
        "Baseline rationale needs concrete reasons, not placeholders",
        "baseline rationale",
    )?;
    let backward_compatibility = prompt_choice(
        "Backward compatibility [required|not_required|boundary_only]",
        parse_backward_compatibility,
    )?;
    let migration_planning = prompt_choice(
        "Migration planning [required|not_required]",
        parse_requiredness,
    )?;
    let rollout_controls = prompt_choice(
        "Rollout controls [none|lightweight|required]",
        parse_rollout_controls,
    )?;
    let deprecation_policy = prompt_choice(
        "Deprecation policy [required|not_required_yet]",
        parse_deprecation_policy,
    )?;
    let observability_threshold = prompt_choice(
        "Observability threshold [minimal|standard|high|regulated]",
        parse_observability_threshold,
    )?;
    let primary_domain_name = prompt_optional("Primary domain name (optional)")?;
    let domains = if primary_domain_name.trim().is_empty() {
        Vec::new()
    } else {
        let blast_radius = prompt_required_concrete(
            "Primary domain blast radius",
            "Primary domain blast radius needs a concrete impact or failure description",
            "primary domain blast radius",
        )?;
        let touches = prompt_csv_optional("Primary domain touches (comma-separated, optional)")?;
        let constraints =
            prompt_csv_optional("Primary domain constraints (comma-separated, optional)")?;
        vec![handbook_engine::CharterDomainInput {
            name: primary_domain_name,
            blast_radius,
            touches,
            constraints,
        }]
    };
    let dimensions = collect_dimension_inputs(baseline_level, &project_name, in_production_today)?;
    let approvers = prompt_csv_non_empty_concrete(
        "Exception approvers (comma-separated, at least one)",
        "Exception approvers need concrete owners or roles",
        "exception approvers",
    )?;
    let record_location = prompt_with_default(
        "Exception record location",
        handbook_engine::DEFAULT_EXCEPTION_RECORD_LOCATION,
    )?;
    let minimum_fields_input = prompt_optional(
        "Exception minimum fields (comma-separated; press enter for standard fields)",
    )?;
    let minimum_fields = if minimum_fields_input.trim().is_empty() {
        default_exception_minimum_fields()
    } else {
        split_csv_required(&minimum_fields_input)?
    };
    let debt_tracking_system = prompt_required_concrete(
        "Debt tracking system",
        "Debt tracking handbook needs a concrete tracker or repository location",
        "debt tracking system",
    )?;
    let debt_tracking_labels =
        prompt_csv_optional("Debt tracking labels (comma-separated, optional)")?;
    let debt_tracking_review_cadence = prompt_required_concrete(
        "Debt tracking review cadence",
        "Debt tracking review cadence needs a concrete cadence such as weekly or monthly",
        "debt tracking review cadence",
    )?;
    let decision_records_enabled = prompt_bool("Decision records enabled? [yes|no]")?;
    let (decision_records_path, decision_records_format) = if decision_records_enabled {
        (
            prompt_required_concrete(
                "Decision records path",
                "Decision records path needs a concrete folder path",
                "decision records path",
            )?,
            prompt_required_concrete(
                "Decision records format",
                "Decision records format needs a concrete format such as md",
                "decision records format",
            )?,
        )
    } else {
        (String::new(), String::new())
    };

    Ok(handbook_engine::CharterStructuredInput {
        schema_version: "0.1.0".to_string(),
        project: handbook_engine::CharterProjectInput {
            name: project_name.clone(),
            classification,
            team_size,
            users,
            expected_lifetime,
            surfaces,
            runtime_environments,
            constraints: handbook_engine::CharterProjectConstraintsInput {
                deadline,
                budget,
                experience_notes: experience_notes.clone(),
                must_use_tech,
            },
            operational_reality: handbook_engine::CharterOperationalRealityInput {
                in_production_today,
                prod_users_or_data,
                external_contracts_to_preserve,
                uptime_expectations,
            },
            default_implications: handbook_engine::CharterDefaultImplicationsInput {
                backward_compatibility,
                migration_planning,
                rollout_controls,
                deprecation_policy,
                observability_threshold,
            },
        },
        posture: handbook_engine::CharterPostureInput {
            rubric_scale: "1-5".to_string(),
            baseline_level,
            baseline_rationale,
        },
        domains,
        dimensions,
        exceptions: handbook_engine::CharterExceptionsInput {
            approvers,
            record_location,
            minimum_fields,
        },
        debt_tracking: handbook_engine::CharterDebtTrackingInput {
            system: debt_tracking_system,
            labels: debt_tracking_labels,
            review_cadence: debt_tracking_review_cadence,
        },
        decision_records: handbook_engine::CharterDecisionRecordsInput {
            enabled: decision_records_enabled,
            path: decision_records_path,
            format: decision_records_format,
        },
    })
}

struct ProjectContextGuidedDefaults {
    project_name: String,
    repo_or_project_ref: String,
    charter_ref: String,
}

fn collect_guided_project_context_input(
    repo_root: &Path,
) -> Result<handbook_engine::ProjectContextStructuredInput, String> {
    let _prompt_context = GuidedPromptContextGuard::push(&PROJECT_CONTEXT_PROMPT_CONTEXT);
    let defaults = project_context_guided_defaults(repo_root);

    println!("Guided project-context interview");
    println!(
        "Answer with factual project truth. Use `None` or `Not applicable` when that is the truth."
    );

    let project_name = prompt_project_context_required_concrete_with_default(
        "Project name",
        &defaults.project_name,
        "project name",
    )?;
    let owner = prompt_project_context_required_concrete(
        "Owner",
        "Owner needs a concrete person, role, or team name",
        "owner",
    )?;
    let team = prompt_project_context_required_concrete(
        "Team",
        "Team needs a concrete owning team or group name",
        "team",
    )?;
    let repo_or_project_ref = prompt_project_context_required_concrete_with_default(
        "Repo / project reference",
        &defaults.repo_or_project_ref,
        "repo / project reference",
    )?;
    let charter_ref = prompt_project_context_required_concrete_with_default(
        "Charter ref",
        &defaults.charter_ref,
        "charter ref",
    )?;

    let what_this_project_is = prompt_project_context_required_concrete(
        "What this project is",
        "Provide a concrete one-line project summary",
        "project summary",
    )?;
    let primary_surface = prompt_project_context_required_concrete(
        "Primary surface",
        "Provide the primary surface such as CLI, API, web app, or library",
        "primary surface",
    )?;
    let primary_users = prompt_project_context_required_concrete(
        "Primary users",
        "Provide the primary users for this project",
        "primary users",
    )?;
    let key_workflows = prompt_project_context_csv_non_empty_concrete(
        "Key workflows (comma-separated, 1-3)",
        "Provide 1-3 concrete workflows",
        "key workflows",
    )?;
    let non_goals = prompt_optional("Non-goals (optional)")?;

    let is_live_in_production_today = prompt_project_context_required_concrete(
        "Is anything live in production today?",
        "Answer with a factual production status such as yes, no, or not applicable",
        "operational reality",
    )?;
    let users = prompt_project_context_required_concrete(
        "Users",
        "Provide the current users such as internal operators, external customers, or none",
        "users",
    )?;
    let data_in_production = prompt_project_context_required_concrete(
        "Data in production",
        "Provide the production data reality such as none, internal data, or customer data",
        "data in production",
    )?;
    let uptime_expectations = prompt_project_context_required_concrete(
        "Uptime expectations / SLA",
        "Provide a factual uptime expectation or `Not applicable`",
        "uptime expectations",
    )?;
    let incident_on_call_reality = prompt_project_context_required_concrete(
        "Incident / on-call reality",
        "Provide the current incident ownership or `Not applicable`",
        "incident/on-call reality",
    )?;
    let primary_risk_flags_present = prompt_project_context_required_concrete(
        "Primary risk flags present",
        "Provide the top current planning or delivery risks",
        "primary risk flags",
    )?;

    let project_type = prompt_project_context_required_concrete(
        "Project type",
        "Provide the project type such as greenfield, brownfield, integration, or modernization",
        "project type",
    )?;
    let backward_compatibility_required = prompt_project_context_required_concrete(
        "Backward compatibility required?",
        "Provide a factual yes/no/not applicable answer",
        "backward compatibility required",
    )?;
    let backward_compatibility_notes = prompt_project_context_required_concrete(
        "Backward compatibility notes",
        "Provide the concrete compatibility constraint or `Not applicable`",
        "backward compatibility notes",
    )?;
    let migration_planning_required = prompt_project_context_required_concrete(
        "Migration planning required?",
        "Provide a factual yes/no/not applicable answer",
        "migration planning required",
    )?;
    let migration_planning_notes = prompt_project_context_required_concrete(
        "Migration planning notes",
        "Provide the concrete migration reality or `Not applicable`",
        "migration planning notes",
    )?;
    let deprecation_policy_exists = prompt_project_context_required_concrete(
        "Deprecation policy exists?",
        "Provide a factual yes/no/not applicable answer",
        "deprecation policy exists",
    )?;
    let deprecation_policy_notes = prompt_project_context_required_concrete(
        "Deprecation policy notes",
        "Provide the concrete deprecation policy reality or `Not applicable`",
        "deprecation policy notes",
    )?;
    let rollout_controls_required = prompt_project_context_required_concrete(
        "Rollout controls required?",
        "Provide a factual yes/no/not applicable answer",
        "rollout controls required",
    )?;
    let rollout_controls_notes = prompt_project_context_required_concrete(
        "Rollout controls notes",
        "Provide the concrete rollout control reality or `Not applicable`",
        "rollout controls notes",
    )?;

    let owned_areas = prompt_project_context_csv_non_empty_concrete(
        "Owned areas (comma-separated)",
        "Provide at least one concrete owned area",
        "owned areas",
    )?;
    let external_dependencies = prompt_project_context_csv_non_empty_concrete(
        "External dependencies (comma-separated)",
        "Provide at least one concrete external dependency or `None`",
        "external dependencies",
    )?;

    let integration_count = prompt_usize_in_range_with_default("Integration count [0-5]", 0, 5, 0)?;
    let mut integrations = Vec::with_capacity(integration_count);
    for index in 0..integration_count {
        let label = index + 1;
        let name = prompt_project_context_required_concrete(
            &format!("Integration {label} name"),
            "Integration name needs a concrete identifier",
            &format!("integration {label} name"),
        )?;
        let integration_type = prompt_project_context_required_concrete(
            &format!("Integration {label} type"),
            "Integration type needs a concrete value such as internal service, external SaaS, API, DB, or file",
            &format!("integration {label} type"),
        )?;
        let contract_surface = prompt_project_context_required_concrete(
            &format!("Integration {label} contract surface"),
            "Contract surface needs a concrete boundary or protocol",
            &format!("integration {label} contract surface"),
        )?;
        let authentication_authorization = prompt_project_context_required_concrete(
            &format!("Integration {label} authentication / authorization"),
            "Authentication / authorization needs a concrete value or `None`",
            &format!("integration {label} authentication"),
        )?;
        let failure_mode_expectations = prompt_project_context_required_concrete(
            &format!("Integration {label} failure mode expectations"),
            "Failure mode expectations need a concrete value",
            &format!("integration {label} failure modes"),
        )?;
        integrations.push(handbook_engine::ProjectContextIntegrationInput {
            name,
            integration_type,
            contract_surface,
            authentication_authorization,
            failure_mode_expectations,
        });
    }

    let environments_that_exist = prompt_project_context_required_concrete(
        "Environments that exist",
        "Provide the existing environments such as local dev, CI, staging, or production",
        "environments that exist",
    )?;
    let deployment_model = prompt_project_context_required_concrete(
        "Deployment model",
        "Provide the deployment model such as local execution, containers, serverless, or desktop",
        "deployment model",
    )?;
    let ci_cd_reality = prompt_project_context_required_concrete(
        "CI/CD reality",
        "Provide the current CI/CD reality or `None`",
        "CI/CD reality",
    )?;
    let release_cadence = prompt_project_context_required_concrete(
        "Release cadence",
        "Provide the release cadence or `Not applicable`",
        "release cadence",
    )?;
    let config_and_secrets = prompt_project_context_required_concrete(
        "Config & secrets",
        "Provide the config and secrets handling reality",
        "config and secrets",
    )?;
    let observability_stack = prompt_project_context_required_concrete(
        "Observability stack",
        "Provide the observability stack or `None`",
        "observability stack",
    )?;

    let primary_data_stores = prompt_project_context_required_concrete(
        "Primary data stores",
        "Provide the primary data stores or `None`",
        "primary data stores",
    )?;
    let data_classification = prompt_project_context_required_concrete(
        "Data classification",
        "Provide the data classification or `None`",
        "data classification",
    )?;
    let retention_requirements = prompt_project_context_required_concrete(
        "Retention requirements",
        "Provide the retention requirement or `Not applicable`",
        "retention requirements",
    )?;
    let backups_disaster_recovery = prompt_project_context_required_concrete(
        "Backups / DR reality",
        "Provide the backup / DR reality or `Not applicable`",
        "backups / DR reality",
    )?;
    let existing_migrations_history = prompt_project_context_required_concrete(
        "Existing migrations / history",
        "Provide the migration history or `Not applicable`",
        "existing migrations/history",
    )?;

    let codebase_exists_today = prompt_bool_with_default("Codebase exists today?", true)?;
    let current_maturity = prompt_project_context_required_concrete(
        "Current maturity",
        "Provide the current codebase maturity such as small, medium, large, or not applicable",
        "current maturity",
    )?;
    let key_modules_or_areas =
        prompt_csv_optional("Key modules / areas to be aware of (comma-separated, optional)")?;
    let known_constraints_from_existing_code = prompt_project_context_required_concrete(
        "Known constraints from existing code",
        "Provide the concrete code constraints or `None`",
        "known constraints from existing code",
    )?;

    let deadline_time_constraints = prompt_project_context_required_concrete(
        "Deadline / time constraints",
        "Provide the deadline or `None`",
        "deadline/time constraints",
    )?;
    let budget_constraints = prompt_project_context_required_concrete(
        "Budget constraints",
        "Provide the budget constraint or `None`",
        "budget constraints",
    )?;
    let must_use_or_prohibited_tech = prompt_project_context_required_concrete(
        "Must-use / prohibited tech",
        "Provide the required or prohibited tech or `None`",
        "must-use or prohibited tech",
    )?;
    let compliance_legal_constraints = prompt_project_context_required_concrete(
        "Compliance / legal constraints",
        "Provide the compliance or legal constraints or `None`",
        "compliance/legal constraints",
    )?;
    let performance_constraints = prompt_project_context_required_concrete(
        "Performance constraints",
        "Provide the performance constraints or `None`",
        "performance constraints",
    )?;
    let security_constraints = prompt_project_context_required_concrete(
        "Security constraints",
        "Provide the security constraints or `None`",
        "security constraints",
    )?;

    let known_unknown_count =
        prompt_usize_in_range_with_default("Known unknown count [1-5]", 1, 5, 1)?;
    let mut known_unknowns = Vec::with_capacity(known_unknown_count);
    for index in 0..known_unknown_count {
        let label = index + 1;
        let item = prompt_project_context_required_concrete(
            &format!("Known unknown {label} item"),
            "Known unknown item needs a concrete planning unknown",
            &format!("known unknown {label} item"),
        )?;
        let unknown_owner = prompt_project_context_required_concrete(
            &format!("Known unknown {label} owner"),
            "Known unknown owner needs a concrete person, role, or team",
            &format!("known unknown {label} owner"),
        )?;
        let revisit_trigger = prompt_project_context_required_concrete(
            &format!("Known unknown {label} revisit trigger"),
            "Known unknown revisit trigger needs a concrete milestone or condition",
            &format!("known unknown {label} revisit trigger"),
        )?;
        known_unknowns.push(handbook_engine::ProjectContextKnownUnknownInput {
            item,
            owner: unknown_owner,
            revisit_trigger,
        });
    }

    Ok(handbook_engine::ProjectContextStructuredInput {
        schema_version: "0.1.0".to_string(),
        project_name,
        owner,
        team,
        repo_or_project_ref,
        charter_ref,
        project_summary: handbook_engine::ProjectContextSummaryInput {
            what_this_project_is,
            primary_surface,
            primary_users,
            key_workflows,
            non_goals,
        },
        operational_reality: handbook_engine::ProjectContextOperationalRealityInput {
            is_live_in_production_today,
            users,
            data_in_production,
            uptime_expectations,
            incident_on_call_reality,
            primary_risk_flags_present,
        },
        classification_implications:
            handbook_engine::ProjectContextClassificationImplicationsInput {
                project_type,
                backward_compatibility_required,
                backward_compatibility_notes,
                migration_planning_required,
                migration_planning_notes,
                deprecation_policy_exists,
                deprecation_policy_notes,
                rollout_controls_required,
                rollout_controls_notes,
            },
        system_boundaries: handbook_engine::ProjectContextSystemBoundariesInput {
            owned_areas,
            external_dependencies,
        },
        integrations,
        environments_and_delivery: handbook_engine::ProjectContextEnvironmentsAndDeliveryInput {
            environments_that_exist,
            deployment_model,
            ci_cd_reality,
            release_cadence,
            config_and_secrets,
            observability_stack,
        },
        data_reality: handbook_engine::ProjectContextDataRealityInput {
            primary_data_stores,
            data_classification,
            retention_requirements,
            backups_disaster_recovery,
            existing_migrations_history,
        },
        repo_codebase_reality: handbook_engine::ProjectContextRepoCodebaseRealityInput {
            codebase_exists_today,
            current_maturity,
            key_modules_or_areas,
            known_constraints_from_existing_code,
        },
        constraints: handbook_engine::ProjectContextConstraintsInput {
            deadline_time_constraints,
            budget_constraints,
            must_use_or_prohibited_tech,
            compliance_legal_constraints,
            performance_constraints,
            security_constraints,
        },
        known_unknowns,
    })
}

fn project_context_guided_defaults(repo_root: &Path) -> ProjectContextGuidedDefaults {
    let project_name = repo_root
        .file_name()
        .and_then(|name| name.to_str())
        .filter(|name| !name.trim().is_empty())
        .unwrap_or("Project")
        .to_string();

    ProjectContextGuidedDefaults {
        project_name,
        repo_or_project_ref: repo_root.display().to_string(),
        charter_ref: ".handbook/charter/CHARTER.md".to_string(),
    }
}

fn prompt_project_context_required_concrete(
    prompt: &str,
    follow_up_prompt: &str,
    field_name: &str,
) -> Result<String, String> {
    let value = prompt_line(prompt)?;
    if let Some(normalized) = normalize_required_free_text(&value) {
        return Ok(normalized);
    }

    let follow_up = prompt_line(follow_up_prompt)?;
    if let Some(normalized) = normalize_required_free_text(&follow_up) {
        return Ok(normalized);
    }

    Err(render_project_context_interview_incomplete_refusal(
        &format!(
        "guided project-context interview could not normalize a concrete answer for {field_name}"
    ),
    ))
}

fn prompt_project_context_required_concrete_with_default(
    prompt: &str,
    default_value: &str,
    field_name: &str,
) -> Result<String, String> {
    loop {
        let value = prompt_line(&format!("{prompt} [{default_value}]"))?;
        if value.trim().is_empty() {
            return Ok(default_value.to_string());
        }
        if let Some(normalized) = normalize_required_free_text(&value) {
            return Ok(normalized);
        }
        println!("Provide a concrete answer or press enter to keep the default.");
        println!(
            "guided project-context interview needs a concrete answer for {field_name} when customizing"
        );
    }
}

fn prompt_project_context_csv_non_empty_concrete(
    prompt: &str,
    follow_up_prompt: &str,
    field_name: &str,
) -> Result<Vec<String>, String> {
    let value = prompt_line(prompt)?;
    if let Some(items) = normalize_required_csv(&value) {
        return Ok(items);
    }

    let follow_up = prompt_line(follow_up_prompt)?;
    if let Some(items) = normalize_required_csv(&follow_up) {
        return Ok(items);
    }

    Err(render_project_context_interview_incomplete_refusal(
        &format!(
        "guided project-context interview could not normalize a concrete answer for {field_name}"
    ),
    ))
}

fn prompt_line(prompt: &str) -> Result<String, String> {
    let context = current_guided_prompt_context();
    print!("{prompt}: ");
    io::stdout().flush().map_err(|err| {
        render_author_custom_refusal(
            context.object,
            "REFUSED",
            "PromptWriteFailure",
            &format!("failed to render guided prompt: {err}"),
            "interactive terminal",
            context.retry_command,
        )
    })?;

    let mut value = String::new();
    let bytes_read = io::stdin().read_line(&mut value).map_err(|err| {
        render_author_custom_refusal(
            context.object,
            "REFUSED",
            "PromptReadFailure",
            &format!("failed to read guided answer: {err}"),
            "interactive terminal",
            context.retry_command,
        )
    })?;

    if bytes_read == 0 {
        return Err(render_author_custom_refusal(
            context.object,
            "REFUSED",
            "InterviewIncomplete",
            &format!(
                "{} ended before all required answers were collected",
                context.interview_name
            ),
            context.broken_subject,
            context.restart_or_from_inputs,
        ));
    }

    Ok(value.trim().to_string())
}

fn prompt_required_concrete(
    prompt: &str,
    follow_up_prompt: &str,
    field_name: &str,
) -> Result<String, String> {
    let value = prompt_line(prompt)?;
    if let Some(normalized) = normalize_required_free_text(&value) {
        return Ok(normalized);
    }

    let follow_up = prompt_line(follow_up_prompt)?;
    if let Some(normalized) = normalize_required_free_text(&follow_up) {
        return Ok(normalized);
    }

    Err(render_interview_incomplete_refusal(&format!(
        "guided charter interview could not normalize a concrete answer for {field_name}"
    )))
}

fn prompt_optional(prompt: &str) -> Result<String, String> {
    prompt_line(prompt).map(|value| normalize_free_text_answer(&value))
}

fn prompt_with_default(prompt: &str, default_value: &str) -> Result<String, String> {
    let value = prompt_line(&format!("{prompt} [{default_value}]"))?;
    if value.trim().is_empty() {
        Ok(default_value.to_string())
    } else {
        Ok(normalize_free_text_answer(&value))
    }
}

fn prompt_bool(prompt: &str) -> Result<bool, String> {
    loop {
        let value = prompt_line(prompt)?;
        match value.trim().to_ascii_lowercase().as_str() {
            "y" | "yes" | "true" => return Ok(true),
            "n" | "no" | "false" => return Ok(false),
            _ => println!("Expected yes/no."),
        }
    }
}

fn prompt_bool_with_default(prompt: &str, default_value: bool) -> Result<bool, String> {
    let default_label = if default_value { "yes" } else { "no" };
    loop {
        let value = prompt_line(&format!("{prompt} [yes|no] [{default_label}]"))?;
        if value.trim().is_empty() {
            return Ok(default_value);
        }
        match value.trim().to_ascii_lowercase().as_str() {
            "y" | "yes" | "true" => return Ok(true),
            "n" | "no" | "false" => return Ok(false),
            _ => println!("Expected yes/no."),
        }
    }
}

fn prompt_u32(prompt: &str) -> Result<u32, String> {
    loop {
        let value = prompt_line(prompt)?;
        match value.trim().parse::<u32>() {
            Ok(parsed) if parsed > 0 => return Ok(parsed),
            _ => println!("Expected an integer greater than 0."),
        }
    }
}

fn prompt_u8_in_range(prompt: &str, min: u8, max: u8) -> Result<u8, String> {
    loop {
        let value = prompt_line(prompt)?;
        match value.trim().parse::<u8>() {
            Ok(parsed) if (min..=max).contains(&parsed) => return Ok(parsed),
            _ => println!("Expected an integer in the allowed range."),
        }
    }
}

fn prompt_usize_in_range_with_default(
    prompt: &str,
    min: usize,
    max: usize,
    default_value: usize,
) -> Result<usize, String> {
    loop {
        let value = prompt_line(&format!("{prompt} [{default_value}]"))?;
        if value.trim().is_empty() {
            return Ok(default_value);
        }
        match value.trim().parse::<usize>() {
            Ok(parsed) if (min..=max).contains(&parsed) => return Ok(parsed),
            _ => println!("Expected an integer in the allowed range."),
        }
    }
}

fn prompt_u8_in_range_with_default(
    prompt: &str,
    min: u8,
    max: u8,
    default_value: u8,
) -> Result<u8, String> {
    loop {
        let value = prompt_line(&format!("{prompt} [{default_value}]"))?;
        if value.trim().is_empty() {
            return Ok(default_value);
        }
        match value.trim().parse::<u8>() {
            Ok(parsed) if (min..=max).contains(&parsed) => return Ok(parsed),
            _ => println!("Expected an integer in the allowed range."),
        }
    }
}

fn prompt_choice<T>(prompt: &str, parser: fn(&str) -> Result<T, String>) -> Result<T, String> {
    loop {
        let value = prompt_line(prompt)?;
        match parser(&value) {
            Ok(parsed) => return Ok(parsed),
            Err(err) => println!("{err}"),
        }
    }
}

fn prompt_csv_choice<T>(
    prompt: &str,
    parser: fn(&str) -> Result<T, String>,
) -> Result<Vec<T>, String> {
    loop {
        let value = prompt_line(prompt)?;
        match split_csv_required(&value) {
            Ok(items) => {
                let mut parsed = Vec::new();
                let mut error = None;
                for item in items {
                    match parser(&item) {
                        Ok(value) => parsed.push(value),
                        Err(err) => {
                            error = Some(err);
                            break;
                        }
                    }
                }
                if let Some(err) = error {
                    println!("{err}");
                    continue;
                }
                return Ok(parsed);
            }
            Err(err) => println!("{err}"),
        }
    }
}

fn prompt_csv_optional(prompt: &str) -> Result<Vec<String>, String> {
    let value = prompt_line(prompt)?;
    if value.trim().is_empty() {
        Ok(Vec::new())
    } else {
        split_csv_required(&value)
    }
}

fn prompt_csv_non_empty_concrete(
    prompt: &str,
    follow_up_prompt: &str,
    field_name: &str,
) -> Result<Vec<String>, String> {
    let value = prompt_line(prompt)?;
    if let Some(items) = normalize_required_csv(&value) {
        return Ok(items);
    }

    let follow_up = prompt_line(follow_up_prompt)?;
    if let Some(items) = normalize_required_csv(&follow_up) {
        return Ok(items);
    }

    Err(render_interview_incomplete_refusal(&format!(
        "guided charter interview could not normalize a concrete answer for {field_name}"
    )))
}

fn prompt_required_concrete_with_default(
    prompt: &str,
    default_value: &str,
    field_name: &str,
) -> Result<String, String> {
    loop {
        let value = prompt_line(&format!("{prompt} [{default_value}]"))?;
        if value.trim().is_empty() {
            return Ok(default_value.to_string());
        }
        if let Some(normalized) = normalize_required_free_text(&value) {
            return Ok(normalized);
        }
        println!("Provide a concrete answer or press enter to keep the baseline.");
        println!(
            "guided charter interview needs a concrete answer for {field_name} when customizing"
        );
    }
}

fn prompt_csv_non_empty_concrete_with_default(
    prompt: &str,
    default_value: &[String],
    field_name: &str,
) -> Result<Vec<String>, String> {
    let default_display = join_csv_default(default_value);
    loop {
        let value = prompt_line(&format!("{prompt} [{default_display}]"))?;
        if value.trim().is_empty() {
            return Ok(default_value.to_vec());
        }
        if let Some(items) = normalize_required_csv(&value) {
            return Ok(items);
        }
        println!("Provide concrete comma-separated values or press enter to keep the baseline.");
        println!(
            "guided charter interview needs a concrete answer for {field_name} when customizing"
        );
    }
}

fn prompt_csv_optional_with_default(
    prompt: &str,
    default_value: &[String],
) -> Result<Vec<String>, String> {
    let default_display = join_csv_default(default_value);
    let value = prompt_line(&format!("{prompt} [{default_display}]"))?;
    if value.trim().is_empty() {
        Ok(default_value.to_vec())
    } else {
        split_csv_required(&value)
    }
}

fn split_csv_required(value: &str) -> Result<Vec<String>, String> {
    let items = value
        .split(',')
        .map(normalize_free_text_answer)
        .filter(|item| !item.is_empty())
        .collect::<Vec<_>>();
    if items.is_empty() {
        Err("Provide at least one comma-separated value.".to_string())
    } else {
        Ok(items)
    }
}

fn normalize_required_free_text(value: &str) -> Option<String> {
    let normalized = normalize_free_text_answer(value);
    if normalized.is_empty() || is_unusably_vague_text(&normalized) {
        None
    } else {
        Some(normalized)
    }
}

fn normalize_required_csv(value: &str) -> Option<Vec<String>> {
    let items = split_csv_required(value).ok()?;
    if items.iter().any(|item| is_unusably_vague_text(item)) {
        None
    } else {
        Some(items)
    }
}

fn normalize_free_text_answer(value: &str) -> String {
    handbook_engine::author::normalize_charter_free_text(value)
}

fn join_csv_default(items: &[String]) -> String {
    if items.is_empty() {
        "none".to_string()
    } else {
        items.join(", ")
    }
}

fn is_unusably_vague_text(value: &str) -> bool {
    handbook_engine::author::is_unusably_vague_charter_text(value)
}

fn render_interview_incomplete_refusal(summary: &str) -> String {
    render_author_custom_refusal(
        "author charter",
        "REFUSED",
        "InterviewIncomplete",
        summary,
        "structured charter input",
        "restart `handbook author charter` or use `handbook author charter --from-inputs <path|->`",
    )
}

fn render_project_context_interview_incomplete_refusal(summary: &str) -> String {
    render_author_custom_refusal(
        "author project-context",
        "REFUSED",
        "InterviewIncomplete",
        summary,
        "structured project-context input",
        "restart `handbook author project-context` or use `handbook author project-context --from-inputs <path|->`",
    )
}

fn parse_project_classification(
    value: &str,
) -> Result<handbook_engine::CharterProjectClassification, String> {
    match value.trim().to_ascii_lowercase().as_str() {
        "greenfield" => Ok(handbook_engine::CharterProjectClassification::Greenfield),
        "brownfield" => Ok(handbook_engine::CharterProjectClassification::Brownfield),
        "integration" => Ok(handbook_engine::CharterProjectClassification::Integration),
        "modernization" => Ok(handbook_engine::CharterProjectClassification::Modernization),
        "hardening" => Ok(handbook_engine::CharterProjectClassification::Hardening),
        _ => Err(
            "Expected one of greenfield, brownfield, integration, modernization, or hardening."
                .to_string(),
        ),
    }
}

fn parse_audience(value: &str) -> Result<handbook_engine::CharterAudience, String> {
    match value.trim().to_ascii_lowercase().as_str() {
        "internal" => Ok(handbook_engine::CharterAudience::Internal),
        "external" => Ok(handbook_engine::CharterAudience::External),
        "mixed" => Ok(handbook_engine::CharterAudience::Mixed),
        _ => Err("Expected one of internal, external, or mixed.".to_string()),
    }
}

fn parse_expected_lifetime(
    value: &str,
) -> Result<handbook_engine::CharterExpectedLifetime, String> {
    match value.trim().to_ascii_lowercase().as_str() {
        "days" => Ok(handbook_engine::CharterExpectedLifetime::Days),
        "weeks" => Ok(handbook_engine::CharterExpectedLifetime::Weeks),
        "months" => Ok(handbook_engine::CharterExpectedLifetime::Months),
        "years" => Ok(handbook_engine::CharterExpectedLifetime::Years),
        _ => Err("Expected one of days, weeks, months, or years.".to_string()),
    }
}

fn parse_surface(value: &str) -> Result<handbook_engine::CharterSurface, String> {
    match value.trim().to_ascii_lowercase().as_str() {
        "web_app" => Ok(handbook_engine::CharterSurface::WebApp),
        "api" => Ok(handbook_engine::CharterSurface::Api),
        "cli" => Ok(handbook_engine::CharterSurface::Cli),
        "lib" => Ok(handbook_engine::CharterSurface::Lib),
        "infra" => Ok(handbook_engine::CharterSurface::Infra),
        "ml" => Ok(handbook_engine::CharterSurface::Ml),
        _ => Err("Expected one of web_app, api, cli, lib, infra, or ml.".to_string()),
    }
}

fn parse_runtime_environment(
    value: &str,
) -> Result<handbook_engine::CharterRuntimeEnvironment, String> {
    match value.trim().to_ascii_lowercase().as_str() {
        "browser" => Ok(handbook_engine::CharterRuntimeEnvironment::Browser),
        "server" => Ok(handbook_engine::CharterRuntimeEnvironment::Server),
        "cloud" => Ok(handbook_engine::CharterRuntimeEnvironment::Cloud),
        "on_prem" => Ok(handbook_engine::CharterRuntimeEnvironment::OnPrem),
        "edge" => Ok(handbook_engine::CharterRuntimeEnvironment::Edge),
        _ => Err("Expected one of browser, server, cloud, on_prem, or edge.".to_string()),
    }
}

fn parse_backward_compatibility(
    value: &str,
) -> Result<handbook_engine::CharterBackwardCompatibility, String> {
    match value.trim().to_ascii_lowercase().as_str() {
        "required" => Ok(handbook_engine::CharterBackwardCompatibility::Required),
        "not_required" => Ok(handbook_engine::CharterBackwardCompatibility::NotRequired),
        "boundary_only" => Ok(handbook_engine::CharterBackwardCompatibility::BoundaryOnly),
        _ => Err("Expected one of required, not_required, or boundary_only.".to_string()),
    }
}

fn parse_requiredness(value: &str) -> Result<handbook_engine::CharterRequiredness, String> {
    match value.trim().to_ascii_lowercase().as_str() {
        "required" => Ok(handbook_engine::CharterRequiredness::Required),
        "not_required" => Ok(handbook_engine::CharterRequiredness::NotRequired),
        _ => Err("Expected one of required or not_required.".to_string()),
    }
}

fn parse_rollout_controls(value: &str) -> Result<handbook_engine::CharterRolloutControls, String> {
    match value.trim().to_ascii_lowercase().as_str() {
        "none" => Ok(handbook_engine::CharterRolloutControls::None),
        "lightweight" => Ok(handbook_engine::CharterRolloutControls::Lightweight),
        "required" => Ok(handbook_engine::CharterRolloutControls::Required),
        _ => Err("Expected one of none, lightweight, or required.".to_string()),
    }
}

fn parse_deprecation_policy(
    value: &str,
) -> Result<handbook_engine::CharterDeprecationPolicy, String> {
    match value.trim().to_ascii_lowercase().as_str() {
        "required" => Ok(handbook_engine::CharterDeprecationPolicy::Required),
        "not_required_yet" => Ok(handbook_engine::CharterDeprecationPolicy::NotRequiredYet),
        _ => Err("Expected one of required or not_required_yet.".to_string()),
    }
}

fn parse_observability_threshold(
    value: &str,
) -> Result<handbook_engine::CharterObservabilityThreshold, String> {
    match value.trim().to_ascii_lowercase().as_str() {
        "minimal" => Ok(handbook_engine::CharterObservabilityThreshold::Minimal),
        "standard" => Ok(handbook_engine::CharterObservabilityThreshold::Standard),
        "high" => Ok(handbook_engine::CharterObservabilityThreshold::High),
        "regulated" => Ok(handbook_engine::CharterObservabilityThreshold::Regulated),
        _ => Err("Expected one of minimal, standard, high, or regulated.".to_string()),
    }
}

fn default_exception_minimum_fields() -> Vec<String> {
    [
        "what",
        "why",
        "scope",
        "risk",
        "owner",
        "expiry_or_revisit_date",
    ]
    .into_iter()
    .map(str::to_string)
    .collect()
}

fn collect_dimension_inputs(
    baseline_level: u8,
    project_name: &str,
    in_production_today: bool,
) -> Result<Vec<handbook_engine::CharterDimensionInput>, String> {
    let mut dimensions = Vec::with_capacity(all_dimension_names().len());
    for name in all_dimension_names() {
        let baseline =
            default_dimension_input(name, baseline_level, project_name, in_production_today);
        let dimension_label = dimension_label(name);
        let keep_baseline =
            prompt_bool_with_default(&format!("Keep baseline for {dimension_label}?"), true)?;
        if keep_baseline {
            dimensions.push(baseline);
            continue;
        }

        let level = Some(prompt_u8_in_range_with_default(
            &format!("{dimension_label} level [1-5]"),
            1,
            5,
            baseline.level.unwrap_or(baseline_level),
        )?);
        let default_stance = prompt_required_concrete_with_default(
            &format!("{dimension_label} default stance"),
            &baseline.default_stance,
            &format!("{dimension_label} default stance"),
        )?;
        let raise_the_bar_triggers = prompt_csv_non_empty_concrete_with_default(
            &format!("{dimension_label} raise-the-bar triggers (comma-separated)"),
            &baseline.raise_the_bar_triggers,
            &format!("{dimension_label} raise-the-bar triggers"),
        )?;
        let allowed_shortcuts = prompt_csv_non_empty_concrete_with_default(
            &format!("{dimension_label} allowed shortcuts (comma-separated)"),
            &baseline.allowed_shortcuts,
            &format!("{dimension_label} allowed shortcuts"),
        )?;
        let red_lines = prompt_csv_non_empty_concrete_with_default(
            &format!("{dimension_label} red lines (comma-separated)"),
            &baseline.red_lines,
            &format!("{dimension_label} red lines"),
        )?;
        let domain_overrides = prompt_csv_optional_with_default(
            &format!("{dimension_label} domain overrides (comma-separated, optional)"),
            &baseline.domain_overrides,
        )?;

        dimensions.push(handbook_engine::CharterDimensionInput {
            name,
            level,
            default_stance,
            raise_the_bar_triggers,
            allowed_shortcuts,
            red_lines,
            domain_overrides,
        });
    }
    Ok(dimensions)
}

fn all_dimension_names() -> [handbook_engine::CharterDimensionName; 9] {
    [
        handbook_engine::CharterDimensionName::SpeedVsQuality,
        handbook_engine::CharterDimensionName::TypeSafetyStaticAnalysis,
        handbook_engine::CharterDimensionName::TestingRigor,
        handbook_engine::CharterDimensionName::ScalabilityPerformance,
        handbook_engine::CharterDimensionName::ReliabilityOperability,
        handbook_engine::CharterDimensionName::SecurityPrivacy,
        handbook_engine::CharterDimensionName::Observability,
        handbook_engine::CharterDimensionName::DxToolingAutomation,
        handbook_engine::CharterDimensionName::UxPolishApiUsability,
    ]
}

fn default_dimension_input(
    name: handbook_engine::CharterDimensionName,
    baseline_level: u8,
    project_name: &str,
    in_production_today: bool,
) -> handbook_engine::CharterDimensionInput {
    let dimension_label = dimension_label(name);
    let production_trigger = if in_production_today {
        "changes touching live users, data, or uptime"
    } else {
        "changes that create irreversible migration or trust-boundary cost"
    };

    handbook_engine::CharterDimensionInput {
        name,
        level: Some(baseline_level),
        default_stance: format!(
            "{project_name} defaults to level {baseline_level} on {dimension_label}; raise the bar whenever blast radius, trust boundaries, or recovery cost increases."
        ),
        raise_the_bar_triggers: vec![
            production_trigger.to_string(),
            "new external interfaces or contracts".to_string(),
        ],
        allowed_shortcuts: vec![
            "time-boxed exploration before merge".to_string(),
            "fixture-backed or local-only iteration with explicit follow-up".to_string(),
        ],
        red_lines: vec![
            format!("do not waive {dimension_label} expectations on shipped work"),
            "do not hide known risk without recording an exception".to_string(),
        ],
        domain_overrides: Vec::new(),
    }
}

fn dimension_label(name: handbook_engine::CharterDimensionName) -> &'static str {
    match name {
        handbook_engine::CharterDimensionName::SpeedVsQuality => "speed vs quality",
        handbook_engine::CharterDimensionName::TypeSafetyStaticAnalysis => {
            "type safety and static analysis"
        }
        handbook_engine::CharterDimensionName::TestingRigor => "testing rigor",
        handbook_engine::CharterDimensionName::ScalabilityPerformance => {
            "scalability and performance"
        }
        handbook_engine::CharterDimensionName::ReliabilityOperability => {
            "reliability and operability"
        }
        handbook_engine::CharterDimensionName::SecurityPrivacy => "security and privacy",
        handbook_engine::CharterDimensionName::Observability => "observability",
        handbook_engine::CharterDimensionName::DxToolingAutomation => {
            "developer tooling and automation"
        }
        handbook_engine::CharterDimensionName::UxPolishApiUsability => {
            "ux polish and api usability"
        }
    }
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
        handbook_compiler::AuthorEnvironmentInventoryRefusalKind::MissingRequiredCharter
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
