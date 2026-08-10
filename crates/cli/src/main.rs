mod approvers;
mod artifact;
mod author;
mod doctor;
mod doctor_rendering;
mod exit_policy;
#[cfg(test)]
#[path = "../tests_support/flow_output_model_hcm_2_1_tests.rs"]
mod flow_output_model_hcm_2_1_tests;
#[cfg(test)]
#[path = "../tests_support/flow_output_model_hcm_2_2_tests.rs"]
mod flow_output_model_hcm_2_2_tests;
mod flow_rendering;
#[cfg(test)]
#[path = "../tests_support/flow_rendering_surface_tests.rs"]
mod flow_rendering_surface_tests;
mod generate;
mod inspect;
mod pipeline;
mod pipeline_help;
mod rendering;
mod request_shared;
mod setup;
mod shell_shared;

use clap::{CommandFactory, FromArgMatches, Parser, Subcommand, ValueEnum};
use std::process::ExitCode;

const PACKET_PLANNING_ID: &str = "planning.packet";
const PACKET_EXECUTION_DEMO_ID: &str = "execution.demo.packet";
const PACKET_EXECUTION_LIVE_ID: &str = "execution.live.packet";
const RELEASE_VERSION: &str = env!("HANDBOOK_RELEASE_VERSION");

fn main() -> ExitCode {
    let command = pipeline_help::apply_dynamic_pipeline_help(Cli::command());
    let matches = command.clone().get_matches();
    let cli = Cli::from_arg_matches(&matches).unwrap_or_else(|err| err.exit());

    match cli.command {
        Some(command) => command.run(),
        None => {
            let mut command = command;
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
    about = "Rust CLI for the reduced v1 handbook: `setup` initializes or refreshes canonical repo-local `.handbook/` inputs, `author` is the baseline authoring surface for charter and project context, `pipeline` is the orchestration surface for route resolution, explicit stage compilation, explicit stage-output capture, and route-state operations, planning packet generation uses canonical repo-local `.handbook/` inputs, fixture-backed execution demo flows through `execution.demo.packet`, live execution is explicitly refused, `inspect` is the packet proof surface, and `doctor` is the recovery surface.",
    long_about = "Rust CLI for the reduced v1 handbook. `setup` initializes or refreshes canonical repo-local `.handbook/` inputs. `author` is the baseline authoring surface for charter and project context. `pipeline` is the orchestration surface for route resolution, explicit stage compilation, explicit stage-output capture, and route-state operations. planning packet generation uses canonical repo-local `.handbook/` inputs. fixture-backed execution demo flows through `execution.demo.packet`. live execution is explicitly refused. `inspect` is the packet proof surface. `doctor` is the recovery surface."
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Initialize or refresh canonical repo-local `.handbook/` inputs.
    Setup(SetupArgs),
    /// Deterministic agent-facing baseline authoring from normalized inputs.
    Author(AuthorArgs),
    /// Manage repository approver credentials through native engine authority.
    Approvers(ApproversArgs),
    /// Operate on repository-selected artifact kinds through the generic engine path.
    Artifact(artifact::ArtifactArgs),
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
            Command::Approvers(args) => approvers::run(args),
            Command::Artifact(args) => artifact::run(args),
            Command::Pipeline(args) => pipeline::run(args),
            Command::Generate(args) => generate::run(args),
            Command::Inspect(args) => inspect::run(args),
            Command::Doctor(args) => doctor::run(args),
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
    /// Create, approve, promote, or validate the selected canonical Charter.
    Charter(AuthorCharterArgs),
    /// Deterministically author canonical `.handbook/project/context.yaml`.
    ProjectContext(AuthorProjectContextArgs),
}

#[derive(clap::Args, Debug)]
struct AuthorCharterArgs {
    /// Select the explicit Charter acquisition workflow.
    #[arg(long, value_enum)]
    mode: Option<CharterModeArg>,
    /// Read one typed Charter intake envelope from a YAML file or `-` for stdin.
    #[arg(long = "from-inputs", value_name = "path|-")]
    from_inputs: Option<String>,
    /// Compare an amendment against the exact selected canonical fingerprint.
    #[arg(long = "expected-current-fingerprint")]
    expected_current_fingerprint: Option<String>,
    /// Record one native-authority decision for an immutable candidate.
    #[arg(long = "approve-candidate")]
    approve_candidate: Option<String>,
    /// Select the exact required approval class.
    #[arg(long = "approval-class")]
    approval_class: Option<String>,
    /// Select the exact required authority reference.
    #[arg(long = "authority-ref")]
    authority_ref: Option<String>,
    /// Accept an exact candidate waiver reference.
    #[arg(long = "accept-waiver-ref")]
    accept_waiver_refs: Vec<String>,
    /// Promote one approved immutable candidate.
    #[arg(long = "promote-candidate")]
    promote_candidate: Option<String>,
    /// Supply the exact approval record selected for promotion.
    #[arg(long = "approval-ref")]
    approval_ref: Option<String>,
    /// Validate selected canonical Charter truth without mutation.
    #[arg(long)]
    validate: bool,
    /// Emit machine-readable JSON.
    #[arg(long)]
    json: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
enum CharterModeArg {
    GuidedAdaptive,
    Express,
    AgentAssisted,
}

#[derive(clap::Args, Debug)]
struct ApproversArgs {
    #[command(subcommand)]
    command: ApproversCommand,
}

#[derive(Subcommand, Debug)]
enum ApproversCommand {
    /// Bootstrap the immutable repository approver registry.
    Bootstrap(ApproverBootstrapArgs),
    /// Enroll a credential and request exact approval mappings.
    AddCredential(ApproverAddCredentialArgs),
    /// Revoke one credential by its exact hashed identifier.
    RevokeCredential(ApproverRevokeCredentialArgs),
    /// Replace the approval mappings for one credential.
    UpdateMapping(ApproverUpdateMappingArgs),
}

#[derive(clap::Args, Debug)]
struct ApproverBootstrapArgs {
    #[arg(long = "initial-charter-quorum", required = true)]
    initial_charter_quorum: Vec<String>,
    #[arg(long)]
    json: bool,
}

#[derive(clap::Args, Debug)]
struct ApproverAddCredentialArgs {
    #[arg(long = "approval-mapping", required = true)]
    approval_mappings: Vec<String>,
    #[arg(long)]
    json: bool,
}

#[derive(clap::Args, Debug)]
struct ApproverRevokeCredentialArgs {
    #[arg(long = "credential-id-hash")]
    credential_id_hash: String,
    #[arg(long)]
    json: bool,
}

#[derive(clap::Args, Debug)]
struct ApproverUpdateMappingArgs {
    #[arg(long = "credential-id-hash")]
    credential_id_hash: String,
    #[arg(long = "approval-mapping", required = true)]
    approval_mappings: Vec<String>,
    #[arg(long)]
    json: bool,
}

#[derive(clap::Args, Debug)]
struct AuthorProjectContextArgs {
    /// Read normalized structured inputs from a YAML file or `-` for stdin.
    #[arg(long = "from-inputs", value_name = "path|-")]
    from_inputs: Option<String>,
    /// Validate canonical YAML input and fixed-view rendering without mutation.
    #[arg(long)]
    validate: bool,
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
    #[command(about = pipeline_help::SUPPORTED_CAPTURE_HELP_SUMMARY)]
    Capture(PipelineCaptureArgs),
    #[command(about = pipeline_help::SUPPORTED_HANDOFF_HELP_SUMMARY)]
    Handoff(PipelineHandoffArgs),
    /// Route-state operations.
    State(PipelineStateArgs),
}

#[derive(clap::Args, Debug)]
#[command(after_help = pipeline_help::SUPPORTED_HANDOFF_HELP_EXAMPLES)]
struct PipelineHandoffArgs {
    #[command(subcommand)]
    command: PipelineHandoffCommand,
}

#[derive(Subcommand, Debug)]
enum PipelineHandoffCommand {
    #[command(about = pipeline_help::SUPPORTED_HANDOFF_EMIT_HELP_SUMMARY)]
    Emit(PipelineHandoffEmitArgs),
}

#[derive(clap::Args, Debug)]
#[command(after_help = pipeline_help::SUPPORTED_HANDOFF_HELP_EXAMPLES)]
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
#[command(after_help = pipeline_help::SUPPORTED_CAPTURE_HELP_EXAMPLES)]
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

const _: () = {
    let _ = (
        std::mem::size_of::<handbook_sdk::DecisionLog>(),
        std::mem::size_of::<handbook_sdk::PacketResult>(),
        std::mem::size_of::<handbook_sdk::Refusal>(),
    );
};
