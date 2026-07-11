mod author;
mod author_prompting;
mod doctor;
mod doctor_rendering;
mod exit_policy;
mod generate;
mod inspect;
mod pipeline;
mod pipeline_help;
mod rendering;
mod request_shared;
mod setup;
mod shell_shared;

use clap::{CommandFactory, FromArgMatches, Parser, Subcommand};
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
    /// Author canonical `.handbook/charter/CHARTER.md`.
    Charter(AuthorCharterArgs),
    /// Author canonical `.handbook/project_context/PROJECT_CONTEXT.md`.
    ProjectContext(AuthorProjectContextArgs),
    /// Author canonical `.handbook/environment_inventory/ENVIRONMENT_INVENTORY.md`.
    EnvironmentInventory(AuthorEnvironmentInventoryArgs),
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
    /// Validate normalized structured inputs and repo write preconditions without mutation.
    #[arg(long)]
    validate: bool,
}

#[derive(clap::Args, Debug)]
struct AuthorEnvironmentInventoryArgs {
    /// Read normalized structured inputs from a YAML file or `-` for stdin.
    #[arg(long = "from-inputs", value_name = "path|-")]
    from_inputs: Option<String>,
    /// Validate normalized structured inputs and repo write preconditions without mutation.
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
        std::mem::size_of::<handbook_compiler::DecisionLog>(),
        std::mem::size_of::<handbook_flow::PacketResult>(),
        std::mem::size_of::<handbook_compiler::CompilerError>(),
        std::mem::size_of::<handbook_compiler::Refusal>(),
    );
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::author::execute_author_charter_command;
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
            |_, _| Ok(()),
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
}
