mod author;
mod setup;
mod shell_shared;

use clap::{CommandFactory, Parser, Subcommand};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

const PACKET_PLANNING_ID: &str = "planning.packet";
const PACKET_EXECUTION_DEMO_ID: &str = "execution.demo.packet";
const PACKET_EXECUTION_LIVE_ID: &str = "execution.live.packet";
const RELEASE_VERSION: &str = env!("HANDBOOK_RELEASE_VERSION");

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
            Command::Setup(args) => setup::run(args),
            Command::Author(args) => author::run(args),
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
    shell_shared::discover_managed_repo_root(start)
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
        blockers: result
            .blockers
            .into_iter()
            .map(flow_blocker_for_rendering)
            .collect(),
    }
}

fn flow_refusal_for_rendering(
    refusal: handbook_flow::ResolverRefusal,
) -> handbook_compiler::Refusal {
    handbook_compiler::Refusal {
        category: flow_refusal_category_for_rendering(refusal.category),
        summary: refusal.summary,
        broken_subject: flow_subject_ref_for_rendering(refusal.broken_subject),
        next_safe_action: flow_next_safe_action_for_rendering(refusal.next_safe_action),
    }
}

fn flow_blocker_for_rendering(
    blocker: handbook_flow::ResolverBlocker,
) -> handbook_compiler::Blocker {
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
        handbook_flow::ResolverNextSafeAction::RunSetup => {
            handbook_compiler::NextSafeAction::RunSetup
        }
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
        handbook_flow::ResolverNextSafeAction::RunDoctor => {
            handbook_compiler::NextSafeAction::RunDoctor
        }
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
    use crate::author::{execute_author_charter_command, render_author_charter_refusal};
    use std::{cell::Cell, fs};

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
