use clap::{CommandFactory, Parser, Subcommand};
use std::cell::RefCell;
use std::fs;
use std::io::{self, IsTerminal, Read, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

const PACKET_PLANNING_ID: &str = "planning.packet";
const PACKET_EXECUTION_DEMO_ID: &str = "execution.demo.packet";
const PACKET_EXECUTION_LIVE_ID: &str = "execution.live.packet";
const RELEASE_VERSION: &str = env!("HANDBOOK_RELEASE_VERSION");

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

fn main() -> ExitCode {
    let cli = Cli::parse();

    match cli.command {
        Some(command) => command.run(),
        None => {
            let mut command = Cli::command();
            command.print_help().expect("help output");
            println!();
            ExitCode::SUCCESS
        }
    }
}

#[derive(Parser, Debug)]
#[command(
    name = "handbook",
    version = RELEASE_VERSION,
    disable_help_subcommand = true,
    about = "Rust CLI for the reduced v1 handbook: `setup` initializes or refreshes canonical repo-local `.handbook/` inputs, `author` is the baseline authoring surface for charter, project context, and environment inventory, `pipeline` is the orchestration surface for route resolution, explicit stage compilation, explicit stage-output capture, and route-state operations, planning packet generation uses canonical repo-local `.handbook/` inputs, fixture-backed execution demo flows through `execution.demo.packet`, live execution is explicitly refused, `inspect` is the packet proof surface, and `doctor` is the recovery surface.",
    long_about = "Rust CLI for the reduced v1 handbook. `setup` initializes or refreshes canonical repo-local `.handbook/` inputs. `author` is the baseline authoring surface for charter, project context, and environment inventory. `pipeline` is the orchestration surface for route resolution, explicit stage compilation, explicit stage-output capture, and route-state operations. planning packet generation uses canonical repo-local `.handbook/` inputs. fixture-backed execution demo flows through `execution.demo.packet`. live execution is explicitly refused. `inspect` is the packet proof surface. `doctor` is the recovery surface."
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Initialize or refresh canonical repo-local `.handbook/` inputs.
    Setup(SetupArgs),
    /// Human-guided and deterministic baseline authoring surfaces.
    Author(AuthorArgs),
    /// Pipeline operator surface for route resolution, explicit stage compilation, explicit stage-output capture, and route-state operations.
    Pipeline(PipelineArgs),
    /// Generate a reduced-v1 packet.
    Generate(RequestArgs),
    /// Inspect packet composition and decision evidence.
    Inspect(RequestArgs),
    /// Diagnose blockers and safe next actions.
    Doctor(DoctorArgs),
}

impl Command {
    fn run(self) -> ExitCode {
        match self {
            Command::Setup(args) => setup(args),
            Command::Author(args) => author(args),
            Command::Pipeline(args) => pipeline(args),
            Command::Generate(args) => generate(args),
            Command::Inspect(args) => inspect(args),
            Command::Doctor(args) => doctor(args),
        }
    }
}

#[derive(clap::Args, Debug)]
struct SetupArgs {
    #[command(subcommand)]
    command: Option<SetupCommand>,
}

#[derive(Subcommand, Debug)]
enum SetupCommand {
    /// Create canonical `.handbook/` scaffold and starter files for first-run setup.
    Init,
    /// Preserve canonical files by default and optionally rewrite starter files or reset `.handbook/state/**`.
    Refresh(SetupRefreshArgs),
}

#[derive(clap::Args, Debug)]
struct SetupRefreshArgs {
    /// Rewrite setup-owned starter files in place.
    #[arg(long)]
    rewrite: bool,
    /// Reset only `.handbook/state/**`.
    #[arg(long = "reset-state")]
    reset_state: bool,
}

#[derive(clap::Args, Debug)]
struct AuthorArgs {
    #[command(subcommand)]
    command: Option<AuthorCommand>,
}

#[derive(Subcommand, Debug)]
enum AuthorCommand {
    /// Author canonical `.handbook/charter/CHARTER.md`.
    Charter(AuthorCharterArgs),
    /// Author canonical `.handbook/project_context/PROJECT_CONTEXT.md`.
    ProjectContext(AuthorProjectContextArgs),
    /// Author canonical `.handbook/environment_inventory/ENVIRONMENT_INVENTORY.md`.
    EnvironmentInventory,
}

#[derive(clap::Args, Debug)]
struct AuthorCharterArgs {
    /// Read normalized structured inputs from a YAML file or `-` for stdin.
    #[arg(long = "from-inputs", value_name = "path|-")]
    from_inputs: Option<String>,
    /// Validate normalized structured inputs and repo write preconditions without mutation.
    #[arg(long)]
    validate: bool,
}

#[derive(clap::Args, Debug)]
struct AuthorProjectContextArgs {
    /// Read normalized structured inputs from a YAML file or `-` for stdin.
    #[arg(long = "from-inputs", value_name = "path|-")]
    from_inputs: Option<String>,
}

#[derive(clap::Args, Debug)]
struct DoctorArgs {
    /// Emit machine-readable JSON to stdout.
    #[arg(long)]
    json: bool,
}

#[derive(clap::Args, Debug)]
struct PipelineArgs {
    #[command(subcommand)]
    command: PipelineCommand,
}

#[derive(Subcommand, Debug)]
enum PipelineCommand {
    /// List available pipelines.
    List,
    /// Show one canonical pipeline or stage declaration.
    Show(PipelineShowArgs),
    /// Resolve one pipeline route from persisted route state.
    Resolve(PipelineSelectorArgs),
    /// Compile one supported stage payload from persisted route basis.
    Compile(PipelineCompileArgs),
    /// Capture one supported stage output and materialize declared artifact and repo-mirror files for `pipeline.foundation_inputs` stages `stage.04_charter_inputs`, `stage.05_charter_synthesize`, `stage.06_project_context_interview`, `stage.07_foundation_pack`, and `stage.10_feature_spec`.
    Capture(PipelineCaptureArgs),
    /// Emit one supported downstream handoff bundle from persisted stage and provenance surfaces.
    Handoff(PipelineHandoffArgs),
    /// Route-state operations.
    State(PipelineStateArgs),
}

#[derive(clap::Args, Debug)]
struct PipelineHandoffArgs {
    #[command(subcommand)]
    command: PipelineHandoffCommand,
}

#[derive(Subcommand, Debug)]
enum PipelineHandoffCommand {
    /// Emit one bounded handoff bundle for `pipeline.foundation_inputs` -> `feature-slice-decomposer`.
    Emit(PipelineHandoffEmitArgs),
}

#[derive(clap::Args, Debug)]
struct PipelineHandoffEmitArgs {
    /// Canonical id or unambiguous shorthand for a pipeline.
    #[arg(long)]
    id: String,
    /// Supported downstream consumer id.
    #[arg(long)]
    consumer: String,
}

#[derive(clap::Args, Debug)]
struct PipelineStateArgs {
    #[command(subcommand)]
    command: PipelineStateCommand,
}

#[derive(Subcommand, Debug)]
enum PipelineStateCommand {
    /// Set one supported route-state field.
    Set(PipelineStateSetArgs),
}

#[derive(clap::Args, Debug)]
struct PipelineShowArgs {
    /// Canonical id or unambiguous shorthand for a pipeline or stage.
    #[arg(long)]
    id: String,
}

#[derive(clap::Args, Debug)]
struct PipelineSelectorArgs {
    /// Canonical id or unambiguous shorthand for a pipeline.
    #[arg(long)]
    id: String,
}

#[derive(clap::Args, Debug)]
struct PipelineCompileArgs {
    /// Canonical id or unambiguous shorthand for a pipeline.
    #[arg(long)]
    id: String,
    /// Canonical id or unambiguous shorthand for a stage within the selected pipeline.
    #[arg(long)]
    stage: String,
    /// Render compile proof instead of the stage payload.
    #[arg(long)]
    explain: bool,
}

#[derive(clap::Args, Debug)]
struct PipelineCaptureArgs {
    #[command(subcommand)]
    command: Option<PipelineCaptureCommand>,
    /// Canonical id or unambiguous shorthand for a pipeline.
    #[arg(long)]
    id: Option<String>,
    /// Canonical id or unambiguous shorthand for a stage within the selected pipeline.
    #[arg(long)]
    stage: Option<String>,
    /// Validate and cache the capture plan without writing declared outputs.
    #[arg(long)]
    preview: bool,
}

#[derive(Subcommand, Debug)]
enum PipelineCaptureCommand {
    /// Apply one cached preview by capture id.
    Apply(PipelineCaptureApplyArgs),
}

#[derive(clap::Args, Debug)]
struct PipelineCaptureApplyArgs {
    /// Deterministic capture id returned by `pipeline capture --preview`.
    #[arg(long)]
    capture_id: String,
}

#[derive(clap::Args, Debug)]
struct PipelineStateSetArgs {
    /// Canonical id or unambiguous shorthand for a pipeline.
    #[arg(long)]
    id: String,
    /// Route-state routing assignment in name=value form.
    #[arg(long, conflicts_with = "field", required_unless_present = "field")]
    var: Option<String>,
    /// Route-state field assignment in field.path=value form.
    #[arg(long, conflicts_with = "var", required_unless_present = "var")]
    field: Option<String>,
    /// Expected route-state revision. Defaults to the currently loaded revision.
    #[arg(long)]
    expected_revision: Option<u64>,
}

#[derive(clap::Args, Debug)]
struct RequestArgs {
    /// Packet identity to generate or inspect.
    #[arg(long, default_value = "planning.packet")]
    packet: String,
    /// Fixture set id (required for `execution.demo.packet`).
    #[arg(long)]
    fixture_set: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PacketId {
    Planning,
    ExecutionDemo,
    ExecutionLive,
}

impl PacketId {
    fn as_str(self) -> &'static str {
        match self {
            PacketId::Planning => PACKET_PLANNING_ID,
            PacketId::ExecutionDemo => PACKET_EXECUTION_DEMO_ID,
            PacketId::ExecutionLive => PACKET_EXECUTION_LIVE_ID,
        }
    }
}

fn parse_packet_id(packet: &str) -> Result<PacketId, String> {
    let packet = packet.trim();
    match packet {
        PACKET_PLANNING_ID => Ok(PacketId::Planning),
        PACKET_EXECUTION_DEMO_ID => Ok(PacketId::ExecutionDemo),
        PACKET_EXECUTION_LIVE_ID => Ok(PacketId::ExecutionLive),
        _ => Err(format!(
            "unsupported --packet {packet:?} (allowed: {PACKET_PLANNING_ID:?}, {PACKET_EXECUTION_DEMO_ID:?}, {PACKET_EXECUTION_LIVE_ID:?})"
        )),
    }
}

fn validate_fixture_set_id(value: &str) -> Result<(), String> {
    let value = value.trim();
    if value.is_empty() {
        return Err("fixture_set_id must not be empty".to_string());
    }
    if value == "." || value == ".." {
        return Err("fixture_set_id must not be '.' or '..'".to_string());
    }
    if value
        .chars()
        .any(|c| !(c.is_ascii_alphanumeric() || c == '-' || c == '_'))
    {
        return Err("fixture_set_id must be ASCII [A-Za-z0-9_-] only".to_string());
    }
    Ok(())
}

fn execution_demo_fixture_set_dir(repo_root: &Path, fixture_set_id: &str) -> PathBuf {
    repo_root
        .join("tests/fixtures/execution_demo")
        .join(fixture_set_id)
}

fn ensure_dir(path: &Path, what: &str) -> Result<(), String> {
    match std::fs::metadata(path) {
        Ok(meta) if meta.is_dir() => Ok(()),
        Ok(_) => Err(format!("{what} is not a directory: {}", path.display())),
        Err(err) => Err(format!("{what} is missing: {} ({err})", path.display())),
    }
}

fn path_is_dir_or_file(path: &Path) -> bool {
    match std::fs::symlink_metadata(path) {
        Ok(meta) => meta.is_dir() || meta.is_file(),
        Err(_) => false,
    }
}

fn discover_enclosing_git_root(start: &Path) -> Option<PathBuf> {
    for candidate in start.ancestors() {
        if path_is_dir_or_file(&candidate.join(".git")) {
            return Some(candidate.to_path_buf());
        }
    }

    None
}

fn discover_nearest_managed_root(start: &Path) -> Option<PathBuf> {
    for candidate in start.ancestors() {
        if std::fs::symlink_metadata(candidate.join(".handbook")).is_ok() {
            return Some(candidate.to_path_buf());
        }
    }

    None
}

fn discover_managed_repo_root(start: &Path) -> PathBuf {
    if let Some(git_root) = discover_enclosing_git_root(start) {
        return git_root;
    }

    if let Some(managed_root) = discover_nearest_managed_root(start) {
        return managed_root;
    }

    start.to_path_buf()
}

fn fixture_lineage_for_demo(repo_root: &Path, fixture_set_id: &str) -> Vec<String> {
    let base = execution_demo_fixture_set_dir(repo_root, fixture_set_id).join(".handbook");

    let project_context = base.join("project_context/PROJECT_CONTEXT.md");

    let mut out = Vec::new();
    out.push(format!(
        "tests/fixtures/execution_demo/{fixture_set_id}/.handbook/charter/CHARTER.md"
    ));
    if project_context.is_file() {
        out.push(format!(
            "tests/fixtures/execution_demo/{fixture_set_id}/.handbook/project_context/PROJECT_CONTEXT.md"
        ));
    }
    out.push(format!(
        "tests/fixtures/execution_demo/{fixture_set_id}/.handbook/feature_spec/FEATURE_SPEC.md"
    ));
    out
}

fn fixture_section_for_demo(repo_root: &Path, fixture_set_id: &str) -> String {
    let mut out = String::new();
    out.push_str("MODE: fixture-backed execution demo\n");
    out.push_str("## FIXTURE DEMO\n");
    out.push_str(&format!("FIXTURE SET: {fixture_set_id}\n"));
    out.push_str(&format!(
        "FIXTURE BASIS ROOT: tests/fixtures/execution_demo/{fixture_set_id}/.handbook/\n"
    ));
    out.push_str("FIXTURE LINEAGE:\n");
    for (index, item) in fixture_lineage_for_demo(repo_root, fixture_set_id)
        .iter()
        .enumerate()
    {
        out.push_str(&format!("{}. {}\n", index + 1, item));
    }
    out
}

fn inject_after_first_three_lines(rendered: &str, injection: &str) -> String {
    let mut lines: Vec<&str> = rendered.split('\n').collect();
    let insert_at = 3.min(lines.len());
    lines.insert(insert_at, injection.trim_end_matches('\n'));
    lines.join("\n")
}

fn generate(args: RequestArgs) -> ExitCode {
    let cwd = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(err) => {
            println!("REFUSED: failed to determine repo root: {err}");
            return ExitCode::from(1);
        }
    };

    let packet_id = match parse_packet_id(&args.packet) {
        Ok(packet_id) => packet_id,
        Err(err) => {
            println!("REFUSED: {err}");
            return ExitCode::from(1);
        }
    };

    let repo_root = discover_managed_repo_root(&cwd);

    let compiler_root = match packet_id {
        PacketId::Planning | PacketId::ExecutionLive => repo_root.clone(),
        PacketId::ExecutionDemo => {
            let fixture_set_id = match args.fixture_set.as_deref() {
                Some(id) => id.trim(),
                None => {
                    println!("REFUSED: --fixture-set is required when --packet {PACKET_EXECUTION_DEMO_ID}");
                    return ExitCode::from(1);
                }
            };
            if let Err(err) = validate_fixture_set_id(fixture_set_id) {
                println!("REFUSED: invalid --fixture-set {fixture_set_id:?}: {err}");
                return ExitCode::from(1);
            }

            let fixture_set_dir = execution_demo_fixture_set_dir(&repo_root, fixture_set_id);
            if let Err(err) = ensure_dir(&fixture_set_dir, "fixture set directory") {
                println!("REFUSED: {err}");
                return ExitCode::from(1);
            }
            let basis_root = fixture_set_dir.join(".handbook");
            if let Err(err) = ensure_dir(&basis_root, "fixture basis root") {
                println!("REFUSED: {err}");
                return ExitCode::from(1);
            }
            fixture_set_dir
        }
    };

    let result = match handbook_flow::resolve(
        &compiler_root,
        handbook_flow::ResolveRequest {
            packet_id: packet_id.as_str(),
            ..handbook_flow::ResolveRequest::default()
        },
    ) {
        Ok(result) => result,
        Err(err) => {
            println!("REFUSED: resolver error: {err:?}");
            return ExitCode::from(1);
        }
    };

    let ready = result.selection.status == handbook_flow::PacketSelectionStatus::Selected
        && result.refusal.is_none()
        && result.blockers.is_empty();

    let compiler_result = flow_result_for_rendering(result);
    let model = match handbook_compiler::build_output_model(&compiler_result) {
        Ok(model) => model,
        Err(err) => {
            println!("PRESENTATION FAILURE: {err}");
            return ExitCode::from(1);
        }
    };

    println!("{}", handbook_compiler::render_markdown(&model));
    if ready {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}

fn author(args: AuthorArgs) -> ExitCode {
    match args.command {
        Some(AuthorCommand::Charter(args)) => author_charter_command(args),
        Some(AuthorCommand::ProjectContext(args)) => author_project_context_command(args),
        Some(AuthorCommand::EnvironmentInventory) => author_environment_inventory_command(),
        None => print_subcommand_help(&["author"]),
    }
}

struct RenderedCommand {
    output: String,
    exit_code: ExitCode,
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
fn execute_author_charter_command<
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
    CollectGuidedInput:
        Fn(&Path) -> Result<handbook_engine::ProjectContextStructuredInput, String>,
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
            let input = match handbook_engine::parse_project_context_structured_input_yaml(&yaml)
            {
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

fn flow_result_for_rendering(
    result: handbook_flow::ResolverResult,
) -> handbook_compiler::ResolverResult {
    handbook_compiler::ResolverResult {
        c04_result_version: result.c04_result_version,
        c03_schema_version: result.c03_schema_version,
        c03_manifest_generation_version: result.c03_manifest_generation_version,
        c03_fingerprint_sha256: result.c03_fingerprint_sha256,
        packet_result: result.packet_result,
        decision_log: handbook_compiler::DecisionLog {
            entries: result.decision_log_entries,
        },
        budget_outcome: result.budget_outcome,
        selection: result.selection,
        refusal: result.refusal.map(flow_refusal_for_rendering),
        blockers: result.blockers.into_iter().map(flow_blocker_for_rendering).collect(),
    }
}

fn flow_refusal_for_rendering(refusal: handbook_flow::ResolverRefusal) -> handbook_compiler::Refusal {
    handbook_compiler::Refusal {
        category: flow_refusal_category_for_rendering(refusal.category),
        summary: refusal.summary,
        broken_subject: flow_subject_ref_for_rendering(refusal.broken_subject),
        next_safe_action: flow_next_safe_action_for_rendering(refusal.next_safe_action),
    }
}

fn flow_blocker_for_rendering(blocker: handbook_flow::ResolverBlocker) -> handbook_compiler::Blocker {
    handbook_compiler::Blocker {
        category: flow_blocker_category_for_rendering(blocker.category),
        subject: flow_subject_ref_for_rendering(blocker.subject),
        summary: blocker.summary,
        next_safe_action: flow_next_safe_action_for_rendering(blocker.next_safe_action),
    }
}

fn flow_refusal_category_for_rendering(
    category: handbook_flow::ResolverRefusalCategory,
) -> handbook_compiler::RefusalCategory {
    match category {
        handbook_flow::ResolverRefusalCategory::NonCanonicalInputAttempt => {
            handbook_compiler::RefusalCategory::NonCanonicalInputAttempt
        }
        handbook_flow::ResolverRefusalCategory::SystemRootMissing => {
            handbook_compiler::RefusalCategory::SystemRootMissing
        }
        handbook_flow::ResolverRefusalCategory::SystemRootNotDir => {
            handbook_compiler::RefusalCategory::SystemRootNotDir
        }
        handbook_flow::ResolverRefusalCategory::SystemRootSymlinkNotAllowed => {
            handbook_compiler::RefusalCategory::SystemRootSymlinkNotAllowed
        }
        handbook_flow::ResolverRefusalCategory::RequiredArtifactMissing => {
            handbook_compiler::RefusalCategory::RequiredArtifactMissing
        }
        handbook_flow::ResolverRefusalCategory::RequiredArtifactEmpty => {
            handbook_compiler::RefusalCategory::RequiredArtifactEmpty
        }
        handbook_flow::ResolverRefusalCategory::RequiredArtifactStarterTemplate => {
            handbook_compiler::RefusalCategory::RequiredArtifactStarterTemplate
        }
        handbook_flow::ResolverRefusalCategory::RequiredArtifactInvalid => {
            handbook_compiler::RefusalCategory::RequiredArtifactInvalid
        }
        handbook_flow::ResolverRefusalCategory::ArtifactReadError => {
            handbook_compiler::RefusalCategory::ArtifactReadError
        }
        handbook_flow::ResolverRefusalCategory::FreshnessInvalid => {
            handbook_compiler::RefusalCategory::FreshnessInvalid
        }
        handbook_flow::ResolverRefusalCategory::BudgetRefused => {
            handbook_compiler::RefusalCategory::BudgetRefused
        }
        handbook_flow::ResolverRefusalCategory::UnsupportedRequest => {
            handbook_compiler::RefusalCategory::UnsupportedRequest
        }
    }
}

fn flow_blocker_category_for_rendering(
    category: handbook_flow::ResolverBlockerCategory,
) -> handbook_compiler::BlockerCategory {
    match category {
        handbook_flow::ResolverBlockerCategory::SystemRootMissing => {
            handbook_compiler::BlockerCategory::SystemRootMissing
        }
        handbook_flow::ResolverBlockerCategory::SystemRootNotDir => {
            handbook_compiler::BlockerCategory::SystemRootNotDir
        }
        handbook_flow::ResolverBlockerCategory::SystemRootSymlinkNotAllowed => {
            handbook_compiler::BlockerCategory::SystemRootSymlinkNotAllowed
        }
        handbook_flow::ResolverBlockerCategory::RequiredArtifactMissing => {
            handbook_compiler::BlockerCategory::RequiredArtifactMissing
        }
        handbook_flow::ResolverBlockerCategory::RequiredArtifactEmpty => {
            handbook_compiler::BlockerCategory::RequiredArtifactEmpty
        }
        handbook_flow::ResolverBlockerCategory::RequiredArtifactStarterTemplate => {
            handbook_compiler::BlockerCategory::RequiredArtifactStarterTemplate
        }
        handbook_flow::ResolverBlockerCategory::RequiredArtifactInvalid => {
            handbook_compiler::BlockerCategory::RequiredArtifactInvalid
        }
        handbook_flow::ResolverBlockerCategory::ArtifactReadError => {
            handbook_compiler::BlockerCategory::ArtifactReadError
        }
        handbook_flow::ResolverBlockerCategory::FreshnessInvalid => {
            handbook_compiler::BlockerCategory::FreshnessInvalid
        }
        handbook_flow::ResolverBlockerCategory::BudgetRefused => {
            handbook_compiler::BlockerCategory::BudgetRefused
        }
        handbook_flow::ResolverBlockerCategory::UnsupportedRequest => {
            handbook_compiler::BlockerCategory::UnsupportedRequest
        }
    }
}

fn flow_subject_ref_for_rendering(
    subject: handbook_flow::ResolverSubjectRef,
) -> handbook_compiler::SubjectRef {
    match subject {
        handbook_flow::ResolverSubjectRef::CanonicalArtifact {
            kind,
            canonical_repo_relative_path,
        } => handbook_compiler::SubjectRef::CanonicalArtifact {
            kind,
            canonical_repo_relative_path,
        },
        handbook_flow::ResolverSubjectRef::InheritedDependency {
            dependency_id,
            version,
        } => handbook_compiler::SubjectRef::InheritedDependency {
            dependency_id,
            version,
        },
        handbook_flow::ResolverSubjectRef::Policy { policy_id } => {
            handbook_compiler::SubjectRef::Policy { policy_id }
        }
    }
}

fn flow_next_safe_action_for_rendering(
    action: handbook_flow::ResolverNextSafeAction,
) -> handbook_compiler::NextSafeAction {
    match action {
        handbook_flow::ResolverNextSafeAction::RunSetup => handbook_compiler::NextSafeAction::RunSetup,
        handbook_flow::ResolverNextSafeAction::RunSetupInit => {
            handbook_compiler::NextSafeAction::RunSetupInit
        }
        handbook_flow::ResolverNextSafeAction::RunSetupRefresh => {
            handbook_compiler::NextSafeAction::RunSetupRefresh
        }
        handbook_flow::ResolverNextSafeAction::RunAuthorCharter => {
            handbook_compiler::NextSafeAction::RunAuthorCharter
        }
        handbook_flow::ResolverNextSafeAction::RunAuthorProjectContext => {
            handbook_compiler::NextSafeAction::RunAuthorProjectContext
        }
        handbook_flow::ResolverNextSafeAction::RunAuthorEnvironmentInventory => {
            handbook_compiler::NextSafeAction::RunAuthorEnvironmentInventory
        }
        handbook_flow::ResolverNextSafeAction::CreateSystemRoot {
            canonical_repo_relative_path,
        } => handbook_compiler::NextSafeAction::CreateSystemRoot {
            canonical_repo_relative_path,
        },
        handbook_flow::ResolverNextSafeAction::EnsureSystemRootIsDirectory {
            canonical_repo_relative_path,
        } => handbook_compiler::NextSafeAction::EnsureSystemRootIsDirectory {
            canonical_repo_relative_path,
        },
        handbook_flow::ResolverNextSafeAction::RemoveSystemRootSymlink {
            canonical_repo_relative_path,
        } => handbook_compiler::NextSafeAction::RemoveSystemRootSymlink {
            canonical_repo_relative_path,
        },
        handbook_flow::ResolverNextSafeAction::CreateCanonicalArtifact {
            canonical_repo_relative_path,
        } => handbook_compiler::NextSafeAction::CreateCanonicalArtifact {
            canonical_repo_relative_path,
        },
        handbook_flow::ResolverNextSafeAction::FillCanonicalArtifact {
            canonical_repo_relative_path,
        } => handbook_compiler::NextSafeAction::FillCanonicalArtifact {
            canonical_repo_relative_path,
        },
        handbook_flow::ResolverNextSafeAction::ReduceCanonicalArtifactSize {
            canonical_repo_relative_path,
        } => handbook_compiler::NextSafeAction::ReduceCanonicalArtifactSize {
            canonical_repo_relative_path,
        },
        handbook_flow::ResolverNextSafeAction::RunGenerate { packet_id } => {
            handbook_compiler::NextSafeAction::RunGenerate { packet_id }
        }
        handbook_flow::ResolverNextSafeAction::RunDoctor => handbook_compiler::NextSafeAction::RunDoctor,
    }
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

fn render_author_charter_refusal(refusal: &handbook_compiler::AuthorCharterRefusal) -> String {
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

fn parse_rollout_controls(
    value: &str,
) -> Result<handbook_engine::CharterRolloutControls, String> {
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

fn print_subcommand_help(path: &[&str]) -> ExitCode {
    let mut command = Cli::command();
    let mut current = &mut command;
    for segment in path {
        current = current
            .find_subcommand_mut(segment)
            .expect("subcommand help path");
    }
    current.print_help().expect("help output");
    println!();
    ExitCode::SUCCESS
}

fn setup(args: SetupArgs) -> ExitCode {
    let cwd = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(err) => {
            println!("REFUSED: failed to determine repo root: {err}");
            return ExitCode::from(1);
        }
    };
    let repo_root = discover_managed_repo_root(&cwd);

    let (request, routed_from_auto) = match args.command {
        None => (
            handbook_compiler::SetupRequest {
                mode: handbook_compiler::SetupMode::Auto,
                ..handbook_compiler::SetupRequest::default()
            },
            true,
        ),
        Some(SetupCommand::Init) => (
            handbook_compiler::SetupRequest {
                mode: handbook_compiler::SetupMode::Init,
                ..handbook_compiler::SetupRequest::default()
            },
            false,
        ),
        Some(SetupCommand::Refresh(refresh)) => (
            handbook_compiler::SetupRequest {
                mode: handbook_compiler::SetupMode::Refresh,
                rewrite: refresh.rewrite,
                reset_state: refresh.reset_state,
            },
            false,
        ),
    };

    match handbook_compiler::run_setup(&repo_root, &request) {
        Ok(outcome) => {
            println!("{}", render_setup_success(&outcome, routed_from_auto));
            ExitCode::SUCCESS
        }
        Err(refusal) => {
            println!("{}", render_setup_refusal(&refusal));
            ExitCode::from(1)
        }
    }
}

fn render_setup_success(
    outcome: &handbook_compiler::SetupOutcome,
    routed_from_auto: bool,
) -> String {
    let mut out = String::new();
    let starter_actions = outcome
        .plan
        .actions
        .iter()
        .filter(|action| action.label != handbook_compiler::SetupActionLabel::Reset)
        .collect::<Vec<_>>();
    let state_updates = outcome
        .plan
        .actions
        .iter()
        .filter(|action| action.label == handbook_compiler::SetupActionLabel::Reset)
        .collect::<Vec<_>>();
    out.push_str(&format!(
        "OUTCOME: {}\n",
        setup_success_outcome_name(outcome.disposition)
    ));
    out.push_str(&format!(
        "OBJECT: {}\n",
        setup_object_name(outcome.plan.resolved_mode)
    ));
    out.push_str(&format!("NEXT SAFE ACTION: {}\n", outcome.next_safe_action));
    out.push_str("## CANONICAL ROOT\n");
    out.push_str(match outcome.plan.resolved_mode {
        handbook_compiler::SetupMode::Init => "STATUS: established canonical `.handbook/` root\n",
        handbook_compiler::SetupMode::Refresh => "STATUS: reused canonical `.handbook/` root\n",
        handbook_compiler::SetupMode::Auto => {
            unreachable!("setup mode should resolve before render")
        }
    });
    out.push_str("## STARTER FILES\n");
    for action in starter_actions {
        out.push_str(&format!(
            "{} {}\n",
            setup_action_label_name(action.label),
            setup_action_path(action)
        ));
    }
    out.push_str("## STATE UPDATES\n");
    if state_updates.is_empty() {
        out.push_str("<none>\n");
    } else {
        for action in state_updates {
            out.push_str(&format!(
                "{} {}\n",
                setup_action_label_name(action.label),
                action.path
            ));
        }
    }
    out.push_str("## MODE NOTES\n");
    if routed_from_auto {
        out.push_str("ROUTED FROM: handbook setup -> ");
        out.push_str(setup_command_name(outcome.plan.resolved_mode));
        out.push('\n');
    }
    if outcome.disposition == handbook_compiler::SetupDisposition::Scaffolded {
        out.push_str(
            "Required starter files still contain shipped scaffold text; replace canonical truth before running `handbook doctor` or packet work.\n",
        );
    }

    out.trim_end().to_string()
}

fn setup_success_outcome_name(disposition: handbook_compiler::SetupDisposition) -> &'static str {
    match disposition {
        handbook_compiler::SetupDisposition::Ready => "READY",
        handbook_compiler::SetupDisposition::Scaffolded => "SCAFFOLDED",
    }
}

fn render_setup_refusal(refusal: &handbook_compiler::SetupRefusal) -> String {
    let mut out = String::new();
    let next_safe_action = refusal.next_safe_action.trim();
    out.push_str(&format!(
        "OUTCOME: {}\n",
        setup_refusal_outcome_name(refusal.kind)
    ));
    out.push_str("OBJECT: setup\n");
    out.push_str(&format!("NEXT SAFE ACTION: {next_safe_action}\n"));
    out.push_str("## REFUSAL\n");
    out.push_str(&format!(
        "CATEGORY: {}\n",
        setup_refusal_kind_name(refusal.kind)
    ));
    out.push_str(&format!("SUMMARY: {}\n", refusal.summary.trim()));
    out.push_str(&format!(
        "BROKEN SUBJECT: {}\n",
        refusal.broken_subject.trim()
    ));
    out.push_str(&format!("NEXT SAFE ACTION: {next_safe_action}\n"));
    out.trim_end().to_string()
}

fn setup_command_name(mode: handbook_compiler::SetupMode) -> &'static str {
    match mode {
        handbook_compiler::SetupMode::Auto => "handbook setup",
        handbook_compiler::SetupMode::Init => "handbook setup init",
        handbook_compiler::SetupMode::Refresh => "handbook setup refresh",
    }
}

fn setup_object_name(mode: handbook_compiler::SetupMode) -> &'static str {
    match mode {
        handbook_compiler::SetupMode::Auto => "setup",
        handbook_compiler::SetupMode::Init => "setup init",
        handbook_compiler::SetupMode::Refresh => "setup refresh",
    }
}

fn setup_action_label_name(label: handbook_compiler::SetupActionLabel) -> &'static str {
    match label {
        handbook_compiler::SetupActionLabel::Created => "created",
        handbook_compiler::SetupActionLabel::Preserved => "preserved",
        handbook_compiler::SetupActionLabel::Rewritten => "rewritten",
        handbook_compiler::SetupActionLabel::Reset => "reset",
    }
}

fn setup_action_path(action: &handbook_compiler::SetupAction) -> String {
    action.path.clone()
}

fn setup_refusal_outcome_name(kind: handbook_compiler::SetupRefusalKind) -> &'static str {
    match kind {
        handbook_compiler::SetupRefusalKind::AlreadyInitialized
        | handbook_compiler::SetupRefusalKind::InvalidRequest => "REFUSED",
        handbook_compiler::SetupRefusalKind::MissingCanonicalRoot
        | handbook_compiler::SetupRefusalKind::InvalidCanonicalRoot
        | handbook_compiler::SetupRefusalKind::MutationRefused => "BLOCKED",
    }
}

fn setup_refusal_kind_name(kind: handbook_compiler::SetupRefusalKind) -> &'static str {
    match kind {
        handbook_compiler::SetupRefusalKind::AlreadyInitialized => "AlreadyInitialized",
        handbook_compiler::SetupRefusalKind::MissingCanonicalRoot => "MissingCanonicalRoot",
        handbook_compiler::SetupRefusalKind::InvalidCanonicalRoot => "InvalidCanonicalRoot",
        handbook_compiler::SetupRefusalKind::InvalidRequest => "InvalidRequest",
        handbook_compiler::SetupRefusalKind::MutationRefused => "MutationRefused",
    }
}

fn render_doctor_json(report: &handbook_compiler::DoctorReport) -> Result<String, String> {
    serde_json::to_string_pretty(report)
        .map(|mut output| {
            output.push('\n');
            output
        })
        .map_err(|err| format!("failed to serialize doctor json: {err}"))
}

fn doctor(args: DoctorArgs) -> ExitCode {
    let cwd = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(err) => {
            println!("BLOCKED: failed to determine repo root: {err}");
            return ExitCode::from(1);
        }
    };
    let repo_root = discover_managed_repo_root(&cwd);

    let report = match handbook_compiler::doctor(&repo_root) {
        Ok(report) => report,
        Err(err) => {
            println!("INVALID_BASELINE");
            println!("SUMMARY: failed to inspect baseline truth: {err}");
            return ExitCode::from(1);
        }
    };

    if args.json {
        match render_doctor_json(&report) {
            Ok(json) => print!("{json}"),
            Err(err) => {
                println!("INVALID_BASELINE");
                println!("SUMMARY: {err}");
                return ExitCode::from(1);
            }
        }
    } else {
        println!("{}", doctor_status_name(report.status));
        println!(
            "ROOT STATUS: {}",
            doctor_root_status_name(report.system_root_status)
        );
        if let Some(next_safe_action) = &report.next_safe_action {
            println!(
                "NEXT SAFE ACTION: {}",
                handbook_compiler::render_next_safe_action_value(next_safe_action)
            );
        } else {
            println!("NEXT SAFE ACTION: <none>");
        }
        println!("## BASELINE CHECKLIST");
        for item in report.checklist.iter() {
            let subject_path = match &item.subject {
                handbook_compiler::SubjectRef::CanonicalArtifact {
                    canonical_repo_relative_path,
                    ..
                } => *canonical_repo_relative_path,
                _ => item.canonical_repo_relative_path,
            };
            println!(
                "{} [{}] STATUS: {} ACTION: {}",
                item.artifact_label,
                subject_path,
                doctor_artifact_status_name(item.status),
                item.author_command
            );
        }
    }

    if report.status == handbook_compiler::DoctorBaselineStatus::BaselineComplete {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
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

fn doctor_status_name(status: handbook_compiler::DoctorBaselineStatus) -> &'static str {
    match status {
        handbook_compiler::DoctorBaselineStatus::Scaffolded => "SCAFFOLDED",
        handbook_compiler::DoctorBaselineStatus::PartialBaseline => "PARTIAL_BASELINE",
        handbook_compiler::DoctorBaselineStatus::InvalidBaseline => "INVALID_BASELINE",
        handbook_compiler::DoctorBaselineStatus::BaselineComplete => "BASELINE_COMPLETE",
    }
}

fn doctor_root_status_name(status: handbook_compiler::SystemRootStatus) -> &'static str {
    match status {
        handbook_compiler::SystemRootStatus::Ok => "OK",
        handbook_compiler::SystemRootStatus::Missing => "MISSING",
        handbook_compiler::SystemRootStatus::NotDir => "NOT_DIR",
        handbook_compiler::SystemRootStatus::SymlinkNotAllowed => "SYMLINK_NOT_ALLOWED",
    }
}

fn doctor_artifact_status_name(status: handbook_compiler::DoctorArtifactStatus) -> &'static str {
    match status {
        handbook_compiler::DoctorArtifactStatus::Missing => "MISSING",
        handbook_compiler::DoctorArtifactStatus::Empty => "EMPTY",
        handbook_compiler::DoctorArtifactStatus::StarterOwned => "STARTER_OWNED",
        handbook_compiler::DoctorArtifactStatus::Invalid => "INVALID",
        handbook_compiler::DoctorArtifactStatus::ValidCanonicalTruth => "VALID_CANONICAL_TRUTH",
    }
}

fn pipeline(args: PipelineArgs) -> ExitCode {
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
            let request = handbook_pipeline::PipelineHandoffEmitRequest {
                pipeline_selector: emit_args.id,
                consumer_selector: emit_args.consumer,
                producer_command: "handbook pipeline handoff emit --id pipeline.foundation_inputs --consumer feature-slice-decomposer".to_string(),
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

fn read_stdin() -> Result<String, std::io::Error> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;
    Ok(input)
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

fn inspect(args: RequestArgs) -> ExitCode {
    let cwd = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(err) => {
            println!("BLOCKED: failed to determine repo root: {err}");
            return ExitCode::from(1);
        }
    };

    let packet_id = match parse_packet_id(&args.packet) {
        Ok(packet_id) => packet_id,
        Err(err) => {
            println!("BLOCKED: {err}");
            return ExitCode::from(1);
        }
    };

    let repo_root = discover_managed_repo_root(&cwd);

    let (compiler_root, demo_fixture_set_id) = match packet_id {
        PacketId::Planning | PacketId::ExecutionLive => (repo_root.clone(), None),
        PacketId::ExecutionDemo => {
            let fixture_set_id = match args.fixture_set.as_deref() {
                Some(id) => id.trim(),
                None => {
                    println!(
                        "BLOCKED: --fixture-set is required when --packet {PACKET_EXECUTION_DEMO_ID}"
                    );
                    return ExitCode::from(1);
                }
            };
            if let Err(err) = validate_fixture_set_id(fixture_set_id) {
                println!("BLOCKED: invalid --fixture-set {fixture_set_id:?}: {err}");
                return ExitCode::from(1);
            }

            let fixture_set_dir = execution_demo_fixture_set_dir(&repo_root, fixture_set_id);
            if let Err(err) = ensure_dir(&fixture_set_dir, "fixture set directory") {
                println!("BLOCKED: {err}");
                return ExitCode::from(1);
            }
            let basis_root = fixture_set_dir.join(".handbook");
            if let Err(err) = ensure_dir(&basis_root, "fixture basis root") {
                println!("BLOCKED: {err}");
                return ExitCode::from(1);
            }
            (fixture_set_dir, Some(fixture_set_id.to_string()))
        }
    };

    let result = match handbook_flow::resolve(
        &compiler_root,
        handbook_flow::ResolveRequest {
            packet_id: packet_id.as_str(),
            ..handbook_flow::ResolveRequest::default()
        },
    ) {
        Ok(result) => result,
        Err(err) => {
            println!("BLOCKED: resolver error: {err:?}");
            return ExitCode::from(1);
        }
    };

    let ready = result.selection.status == handbook_flow::PacketSelectionStatus::Selected
        && result.refusal.is_none()
        && result.blockers.is_empty();

    let compiler_result = flow_result_for_rendering(result);
    let model = match handbook_compiler::build_output_model(&compiler_result) {
        Ok(model) => model,
        Err(err) => {
            println!("PRESENTATION FAILURE: {err}");
            return ExitCode::from(1);
        }
    };

    if ready {
        println!("{}", handbook_compiler::render_inspect(&model));
    } else {
        let rendered = handbook_compiler::render_inspect(&model);
        if let Some(fixture_set_id) = demo_fixture_set_id.as_deref() {
            let section = fixture_section_for_demo(&repo_root, fixture_set_id);
            println!("{}", inject_after_first_three_lines(&rendered, &section));
        } else {
            println!("{rendered}");
        }
    }

    if ready {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}

const _: () = {
    let _ = (
        std::mem::size_of::<handbook_compiler::DecisionLog>(),
        std::mem::size_of::<handbook_flow::PacketResult>(),
        std::mem::size_of::<handbook_compiler::CompilerError>(),
        std::mem::size_of::<handbook_compiler::Refusal>(),
    );
};

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    fn valid_structured_inputs_yaml() -> &'static str {
        r#"schema_version: "0.1.0"
project:
  name: "Handbook"
  classification: greenfield
  team_size: 2
  users: internal
  expected_lifetime: months
  surfaces:
    - cli
    - api
  runtime_environments:
    - server
  constraints:
    deadline: ""
    budget: ""
    experience_notes: "small team"
    must_use_tech:
      - rust
  operational_reality:
    in_production_today: false
    prod_users_or_data: ""
    external_contracts_to_preserve: []
    uptime_expectations: "best effort"
  default_implications:
    backward_compatibility: not_required
    migration_planning: not_required
    rollout_controls: lightweight
    deprecation_policy: not_required_yet
    observability_threshold: standard
posture:
  rubric_scale: "1-5"
  baseline_level: 3
  baseline_rationale:
    - "internal operators"
    - "moderate blast radius"
domains:
  - name: "planning"
    blast_radius: "medium"
    touches:
      - "internal operators"
    constraints:
      - "preserve trust boundaries"
dimensions:
  - name: speed_vs_quality
    level: 3
    default_stance: "optimize for durability over shortcuts"
    raise_the_bar_triggers: ["production data"]
    allowed_shortcuts: ["time-boxed exploration"]
    red_lines: ["ship without review"]
    domain_overrides: []
  - name: type_safety_static_analysis
    level: 3
    default_stance: "type-safe by default"
    raise_the_bar_triggers: ["cross-boundary interfaces"]
    allowed_shortcuts: ["fixture-backed exploration"]
    red_lines: ["unchecked public contracts"]
    domain_overrides: []
  - name: testing_rigor
    level: 3
    default_stance: "test the shipped path"
    raise_the_bar_triggers: ["regression risk"]
    allowed_shortcuts: ["manual validation for throwaway work"]
    red_lines: ["merge without exercising the path"]
    domain_overrides: []
  - name: scalability_performance
    level: 3
    default_stance: "track obvious bottlenecks"
    raise_the_bar_triggers: ["user-visible latency"]
    allowed_shortcuts: ["defer micro-optimizations"]
    red_lines: ["ignore known load cliffs"]
    domain_overrides: []
  - name: reliability_operability
    level: 3
    default_stance: "prefer recoverable changes"
    raise_the_bar_triggers: ["long-lived state changes"]
    allowed_shortcuts: ["local-only iteration"]
    red_lines: ["unrecoverable migrations without a plan"]
    domain_overrides: []
  - name: security_privacy
    level: 3
    default_stance: "protect boundaries by default"
    raise_the_bar_triggers: ["credentials or user data"]
    allowed_shortcuts: ["synthetic data in local dev"]
    red_lines: ["plaintext secrets"]
    domain_overrides: []
  - name: observability
    level: 3
    default_stance: "emit enough proof to debug production issues"
    raise_the_bar_triggers: ["background jobs"]
    allowed_shortcuts: ["manual logs for local-only work"]
    red_lines: ["silent failures"]
    domain_overrides: []
  - name: dx_tooling_automation
    level: 3
    default_stance: "prefer automation that pays for itself"
    raise_the_bar_triggers: ["frequent repeated workflows"]
    allowed_shortcuts: ["temporary local scripts"]
    red_lines: ["manual-only release steps"]
    domain_overrides: []
  - name: ux_polish_api_usability
    level: 3
    default_stance: "clear operator and API ergonomics"
    raise_the_bar_triggers: ["external users"]
    allowed_shortcuts: ["rough internal copy while iterating"]
    red_lines: ["unclear operator failure modes"]
    domain_overrides: []
exceptions:
  approvers:
    - project_owner
  record_location: ".handbook/charter/CHARTER.md#exceptions"
  minimum_fields:
    - what
    - why
    - scope
    - risk
    - owner
    - expiry_or_revisit_date
debt_tracking:
  system: "issues"
  labels:
    - debt
  review_cadence: "monthly"
decision_records:
  enabled: false
  path: ""
  format: ""
"#
    }

    #[test]
    fn execute_author_charter_command_renders_guided_success_with_injected_author() {
        let dir = tempfile::tempdir().expect("tempdir");
        let collect_called = Cell::new(false);
        let author_called = Cell::new(false);

        let rendered = execute_author_charter_command(
            AuthorCharterArgs {
                from_inputs: None,
                validate: false,
            },
            || Ok(dir.path().to_path_buf()),
            || true,
            |_| Ok(()),
            |_, _| panic!("guided mode should not run from-input preflight"),
            || {
                collect_called.set(true);
                handbook_compiler::parse_charter_structured_input_yaml(
                    valid_structured_inputs_yaml(),
                )
                .map_err(|refusal| render_author_charter_refusal(&refusal))
            },
            |repo_root, input| {
                author_called.set(true);
                assert_eq!(repo_root, dir.path());
                assert_eq!(input.project.name, "Handbook");
                Ok(handbook_compiler::AuthorCharterResult {
                    canonical_repo_relative_path: ".handbook/charter/CHARTER.md",
                    bytes_written: 42,
                })
            },
            |_, _| panic!("guided mode should not run deterministic author"),
        );

        assert!(collect_called.get(), "guided input should be collected");
        assert!(author_called.get(), "authoring closure should be called");
        assert_eq!(rendered.exit_code, ExitCode::SUCCESS);
        assert!(rendered.output.contains("OUTCOME: AUTHORED"));
        assert!(rendered.output.contains("MODE: guided_interview"));
        assert!(rendered.output.contains("SOURCE: interactive terminal"));
    }

    #[test]
    fn execute_author_charter_command_renders_file_success_with_injected_author() {
        let dir = tempfile::tempdir().expect("tempdir");
        let inputs_path = dir.path().join("charter-inputs.yaml");
        fs::write(&inputs_path, valid_structured_inputs_yaml()).expect("write inputs");
        let author_called = Cell::new(false);

        let rendered = execute_author_charter_command(
            AuthorCharterArgs {
                from_inputs: Some(inputs_path.to_string_lossy().into_owned()),
                validate: false,
            },
            || Ok(dir.path().to_path_buf()),
            || panic!("file inputs should not check interactive tty state"),
            |_| panic!("file inputs should not run guided preflight"),
            |_, _| Ok(()),
            || panic!("file inputs should not run guided collection"),
            |_, _| panic!("file inputs should not run guided author"),
            |repo_root, input| {
                author_called.set(true);
                assert_eq!(repo_root, dir.path());
                assert_eq!(input.project.name, "Handbook");
                Ok(handbook_compiler::AuthorCharterResult {
                    canonical_repo_relative_path: ".handbook/charter/CHARTER.md",
                    bytes_written: 24,
                })
            },
        );

        assert!(author_called.get(), "authoring closure should be called");
        assert_eq!(rendered.exit_code, ExitCode::SUCCESS);
        assert!(rendered.output.contains("MODE: structured_inputs_file"));
        assert!(rendered
            .output
            .contains(&format!("SOURCE: {}", inputs_path.display())));
    }

    #[test]
    fn execute_author_charter_command_refuses_without_tty_for_guided_mode() {
        let dir = tempfile::tempdir().expect("tempdir");

        let rendered = execute_author_charter_command(
            AuthorCharterArgs {
                from_inputs: None,
                validate: false,
            },
            || Ok(dir.path().to_path_buf()),
            || false,
            |_| panic!("guided non-tty refusal should happen before preflight"),
            |_, _| panic!("guided non-tty refusal should happen before from-input preflight"),
            || panic!("guided collection should not run without tty"),
            |_, _| panic!("authoring should not run without tty"),
            |_, _| panic!("deterministic author should not run without tty"),
        );

        assert_eq!(rendered.exit_code, ExitCode::from(1));
        assert!(rendered.output.contains("OUTCOME: REFUSED"));
        assert!(rendered.output.contains("CATEGORY: NonInteractiveRefusal"));
        assert!(rendered
            .output
            .contains("run `handbook author charter --from-inputs <path|->`"));
    }

    #[test]
    fn execute_author_charter_command_refuses_during_preflight_before_guided_collection() {
        let dir = tempfile::tempdir().expect("tempdir");
        let collect_called = Cell::new(false);

        let rendered = execute_author_charter_command(
            AuthorCharterArgs {
                from_inputs: None,
                validate: false,
            },
            || Ok(dir.path().to_path_buf()),
            || true,
            |_| {
                Err(handbook_compiler::AuthorCharterRefusal {
                    kind: handbook_compiler::AuthorCharterRefusalKind::ExistingCanonicalTruth,
                    summary: "canonical charter truth already exists".to_string(),
                    broken_subject: ".handbook/charter/CHARTER.md".to_string(),
                    next_safe_action:
                        "inspect `.handbook/charter/CHARTER.md` instead of rerunning `handbook author charter`"
                            .to_string(),
                })
            },
            |_, _| panic!("guided preflight refusal should happen before from-input preflight"),
            || {
                collect_called.set(true);
                panic!("guided collection should not run after preflight refusal")
            },
            |_, _| panic!("authoring should not run after preflight refusal"),
            |_, _| panic!("deterministic author should not run after preflight refusal"),
        );

        assert!(
            !collect_called.get(),
            "guided input should not be collected"
        );
        assert_eq!(rendered.exit_code, ExitCode::from(1));
        assert!(rendered.output.contains("OUTCOME: REFUSED"));
        assert!(rendered.output.contains("CATEGORY: ExistingCanonicalTruth"));
    }
}
