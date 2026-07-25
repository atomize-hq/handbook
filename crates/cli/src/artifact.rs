use clap::{Args, Subcommand, ValueEnum};
use handbook_engine::artifact_intake::MAX_ARTIFACT_INPUT_DOCUMENT_BYTES;
use handbook_engine::artifact_intake_registry::AcquisitionModeV1;
use handbook_engine::artifact_mutation::{execution_disposition_name, ArtifactMutationServiceV1};
use handbook_engine::artifact_repository::{ArtifactRepositoryV1, ArtifactTargetV1};
use serde_json::{json, Value};
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use std::process::ExitCode;

#[derive(Args, Debug)]
pub(crate) struct ArtifactArgs {
    #[command(subcommand)]
    command: ArtifactCommand,
}

#[derive(Subcommand, Debug)]
enum ArtifactCommand {
    /// List artifact kinds selected by the repository record.
    ListKinds(RepositoryArgs),
    /// List artifact instances selected by the repository profile.
    ListInstances(RepositoryArgs),
    /// Read one exact selected artifact instance.
    Read(ArtifactSelectorArgs),
    /// Validate one exact selected artifact instance.
    Validate(ArtifactSelectorArgs),
    /// Read the intake definition selected by one artifact instance.
    IntakeDefinition(ArtifactSelectorArgs),
    /// Evaluate supplied intake values without mutation.
    IntakeEvaluate(IntakeEvaluateArgs),
    /// Append one finalized intake from a bounded request document.
    IntakeAppend(MutationRequestArgs),
    /// Validate a candidate preview from one committed intake pair without mutation.
    CandidateValidate(CandidateValidateArgs),
    /// Append one candidate closure from a bounded request document.
    CandidateAppend(MutationRequestArgs),
    /// Promote one retained candidate through compare-and-write.
    Promote(MutationRequestArgs),
}

#[derive(Args, Debug)]
struct RepositoryArgs {
    /// Exact repository root containing `.handbook/profile-selection.json`.
    #[arg(long = "repository-root")]
    repository_root: String,
    /// Emit machine-readable JSON.
    #[arg(long)]
    json: bool,
}

#[derive(Args, Debug)]
struct ArtifactSelectorArgs {
    /// Exact repository root containing `.handbook/profile-selection.json`.
    #[arg(long = "repository-root")]
    repository_root: String,
    /// Exact selected artifact-kind definition reference.
    #[arg(long = "kind-ref")]
    kind_ref: String,
    /// Exact selected artifact instance ID.
    #[arg(long = "instance-id")]
    instance_id: String,
    /// Emit machine-readable JSON.
    #[arg(long)]
    json: bool,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum IntakeModeArg {
    GuidedAdaptive,
    Express,
    AgentAssisted,
}

#[derive(Args, Debug)]
struct IntakeEvaluateArgs {
    /// Exact repository root containing `.handbook/profile-selection.json`.
    #[arg(long = "repository-root")]
    repository_root: String,
    /// Exact selected artifact-kind definition reference.
    #[arg(long = "kind-ref")]
    kind_ref: String,
    /// Exact selected artifact instance ID.
    #[arg(long = "instance-id")]
    instance_id: String,
    /// Frozen acquisition mode; all modes evaluate the same supplied content contract.
    #[arg(long, value_enum)]
    mode: IntakeModeArg,
    /// Read one bounded supplied-values document from a path or `-` for stdin.
    #[arg(long = "from-inputs", value_name = "path|-")]
    from_inputs: String,
    /// Compare against an exact current fingerprint or the literal `absent`.
    #[arg(long = "expected-current-fingerprint")]
    expected_current_fingerprint: Option<String>,
    /// Emit machine-readable JSON.
    #[arg(long)]
    json: bool,
}

#[derive(Args, Debug)]
struct MutationRequestArgs {
    /// Exact repository root containing `.handbook/profile-selection.json`.
    #[arg(long = "repository-root")]
    repository_root: String,
    /// Exact selected artifact-kind definition reference.
    #[arg(long = "kind-ref")]
    kind_ref: String,
    /// Exact selected artifact instance ID.
    #[arg(long = "instance-id")]
    instance_id: String,
    /// Read one bounded mutation request document from a path or `-` for stdin.
    #[arg(long = "from-request", value_name = "path|-")]
    from_request: String,
    /// Emit machine-readable JSON.
    #[arg(long)]
    json: bool,
}

#[derive(Args, Debug)]
struct CandidateValidateArgs {
    /// Exact repository root containing `.handbook/profile-selection.json`.
    #[arg(long = "repository-root")]
    repository_root: String,
    /// Exact selected artifact-kind definition reference.
    #[arg(long = "kind-ref")]
    kind_ref: String,
    /// Exact selected artifact instance ID.
    #[arg(long = "instance-id")]
    instance_id: String,
    /// Exact committed intake-record reference.
    #[arg(long = "intake-record-ref")]
    intake_record_ref: String,
    /// Exact committed intake-record fingerprint.
    #[arg(long = "intake-record-fingerprint")]
    intake_record_fingerprint: String,
    /// Compare against an exact current fingerprint or the literal `absent`.
    #[arg(long = "expected-current-fingerprint")]
    expected_current_fingerprint: Option<String>,
    /// Emit machine-readable JSON.
    #[arg(long)]
    json: bool,
}

pub(crate) fn run(args: ArtifactArgs) -> ExitCode {
    match execute(args.command) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("artifact operation refused: {error}");
            ExitCode::from(1)
        }
    }
}

fn execute(command: ArtifactCommand) -> Result<(), String> {
    match command {
        ArtifactCommand::ListKinds(args) => {
            let repository = open_repository(&args.repository_root)?;
            let selection_fingerprint = repository
                .selection_fingerprint()
                .map_err(|error| error.to_string())?;
            let kinds = repository.list_kinds().map_err(|error| error.to_string())?;
            let value = json!({
                "operation_id": "artifact.kind.list",
                "selection_fingerprint": selection_fingerprint.as_str(),
                "kinds": kinds.into_iter().map(|kind| json!({
                    "kind_ref": kind.kind_ref.as_str(),
                    "definition_fingerprint": kind.definition_fingerprint.as_str(),
                    "schema_ref": kind.schema_ref.as_str(),
                })).collect::<Vec<_>>()
            });
            emit(value, args.json)
        }
        ArtifactCommand::ListInstances(args) => {
            let repository = open_repository(&args.repository_root)?;
            let selection_fingerprint = repository
                .selection_fingerprint()
                .map_err(|error| error.to_string())?;
            let instances = repository
                .list_instances()
                .map_err(|error| error.to_string())?;
            let value = json!({
                "operation_id": "artifact.instance.list",
                "selection_fingerprint": selection_fingerprint.as_str(),
                "instances": instances.into_iter().map(|instance| json!({
                    "instance_id": instance.instance_id.as_str(),
                    "kind_ref": instance.kind_ref.as_str(),
                    "canonical_path": instance.canonical_path,
                    "intake_definition_ref": instance.intake_definition_ref.as_ref().map(|reference| reference.as_str()),
                })).collect::<Vec<_>>()
            });
            emit(value, args.json)
        }
        ArtifactCommand::Read(args) => {
            let repository = open_repository(&args.repository_root)?;
            let target = target(&args.kind_ref, &args.instance_id)?;
            let result = repository
                .read(target.kind_ref(), target.instance_id())
                .map_err(|error| error.to_string())?;
            emit(
                json!({
                    "operation_id": result.operation_id.as_str(),
                    "operation_context_fingerprint": result.operation_context_fingerprint.as_str(),
                    "kind_ref": result.kind_ref.as_str(),
                    "instance_id": result.instance_id.as_str(),
                    "canonical_path": result.canonical_path,
                    "artifact_fingerprint": result.artifact_fingerprint.as_str(),
                    "content": result.content,
                }),
                args.json,
            )
        }
        ArtifactCommand::Validate(args) => {
            let repository = open_repository(&args.repository_root)?;
            let target = target(&args.kind_ref, &args.instance_id)?;
            let result = repository
                .validate(target.kind_ref(), target.instance_id())
                .map_err(|error| error.to_string())?;
            emit(
                json!({
                    "operation_id": result.operation_id.as_str(),
                    "operation_context_fingerprint": result.operation_context_fingerprint.as_str(),
                    "artifact_fingerprint": result.artifact_fingerprint.as_str(),
                    "outcome": "valid",
                    "content": result.content,
                    "layers": {
                        "structural": "pass",
                        "semantic": layer_status(result.semantic_status),
                        "intake": layer_status(result.intake_status),
                        "approval": layer_status(result.approval_status),
                        "external_evidence": layer_status(result.external_evidence_status),
                    }
                }),
                args.json,
            )
        }
        ArtifactCommand::IntakeDefinition(args) => {
            let repository = open_repository(&args.repository_root)?;
            let target = target(&args.kind_ref, &args.instance_id)?;
            let definition = repository
                .intake_definition(&target)
                .map_err(|error| error.to_string())?;
            emit(
                json!({
                    "operation_id": "intake.definition.read",
                    "kind_ref": target.kind_ref().as_str(),
                    "instance_id": target.instance_id().as_str(),
                    "intake_definition_ref": definition.exact_ref().as_str(),
                    "intake_definition_fingerprint": definition.definition_fingerprint().as_str(),
                    "candidate_schema_ref": definition.candidate_schema_ref().as_str(),
                    "coverage": definition.coverage().iter().map(|row| json!({
                        "coverage_id": row.coverage_id(),
                        "target_paths": row.target_paths(),
                        "minimum_specificity": format!("{:?}", row.minimum_specificity()).to_lowercase(),
                    })).collect::<Vec<_>>()
                }),
                args.json,
            )
        }
        ArtifactCommand::IntakeEvaluate(args) => {
            let repository = open_repository(&args.repository_root)?;
            let target = target(&args.kind_ref, &args.instance_id)?;
            let bytes = read_bounded_input(&args.from_inputs)?;
            let evaluation = repository
                .evaluate_intake_document(
                    &target,
                    acquisition_mode(args.mode),
                    args.expected_current_fingerprint.as_deref(),
                    &bytes,
                )
                .map_err(|error| error.to_string())?;
            emit(
                evaluation
                    .to_json_value()
                    .map_err(|error| error.to_string())?,
                args.json,
            )
        }
        ArtifactCommand::IntakeAppend(args) => {
            let bytes = read_bounded_input(&args.from_request)?;
            let execution = ArtifactMutationServiceV1::intake_append(
                Path::new(&args.repository_root),
                &args.kind_ref,
                &args.instance_id,
                &bytes,
            )
            .map_err(|error| error.to_string())?;
            emit_mutation(execution, args.json)
        }
        ArtifactCommand::CandidateValidate(args) => {
            let preview = ArtifactMutationServiceV1::candidate_validate(
                Path::new(&args.repository_root),
                &args.kind_ref,
                &args.instance_id,
                &args.intake_record_ref,
                &args.intake_record_fingerprint,
                args.expected_current_fingerprint.as_deref(),
            )
            .map_err(|error| error.to_string())?;
            emit(
                serde_json::to_value(preview)
                    .map_err(|_| "candidate preview could not be rendered".to_string())?,
                args.json,
            )
        }
        ArtifactCommand::CandidateAppend(args) => {
            let bytes = read_bounded_input(&args.from_request)?;
            let execution = ArtifactMutationServiceV1::candidate_append(
                Path::new(&args.repository_root),
                &args.kind_ref,
                &args.instance_id,
                &bytes,
            )
            .map_err(|error| error.to_string())?;
            emit_mutation(execution, args.json)
        }
        ArtifactCommand::Promote(args) => {
            let bytes = read_bounded_input(&args.from_request)?;
            let execution = ArtifactMutationServiceV1::promote(
                Path::new(&args.repository_root),
                &args.kind_ref,
                &args.instance_id,
                &bytes,
            )
            .map_err(|error| error.to_string())?;
            emit_mutation(execution, args.json)
        }
    }
}

fn open_repository(path: &str) -> Result<ArtifactRepositoryV1, String> {
    ArtifactRepositoryV1::open(Path::new(path)).map_err(|error| error.to_string())
}

fn target(kind_ref: &str, instance_id: &str) -> Result<ArtifactTargetV1, String> {
    ArtifactTargetV1::parse(kind_ref, instance_id).map_err(|error| error.to_string())
}

fn acquisition_mode(mode: IntakeModeArg) -> AcquisitionModeV1 {
    match mode {
        IntakeModeArg::GuidedAdaptive => AcquisitionModeV1::GuidedAdaptive,
        IntakeModeArg::Express => AcquisitionModeV1::Express,
        IntakeModeArg::AgentAssisted => AcquisitionModeV1::AgentAssisted,
    }
}

fn layer_status(
    status: handbook_engine::artifact_operations::ArtifactValidationLayerStatusV1,
) -> &'static str {
    match status {
        handbook_engine::artifact_operations::ArtifactValidationLayerStatusV1::Pass => "pass",
        handbook_engine::artifact_operations::ArtifactValidationLayerStatusV1::NotApplicable => {
            "not_applicable"
        }
    }
}

fn read_bounded_input(path: &str) -> Result<Vec<u8>, String> {
    let mut reader: Box<dyn Read> = if path == "-" {
        Box::new(io::stdin())
    } else {
        Box::new(File::open(path).map_err(|_| "input document could not be opened".to_string())?)
    };
    let mut bytes = Vec::new();
    reader
        .by_ref()
        .take((MAX_ARTIFACT_INPUT_DOCUMENT_BYTES + 1) as u64)
        .read_to_end(&mut bytes)
        .map_err(|_| "input document could not be read".to_string())?;
    if bytes.len() > MAX_ARTIFACT_INPUT_DOCUMENT_BYTES {
        return Err("input document exceeds the 1 MiB limit".to_string());
    }
    Ok(bytes)
}

fn emit(value: Value, json_mode: bool) -> Result<(), String> {
    let rendered = if json_mode {
        serde_json::to_string(&value)
    } else {
        serde_json::to_string_pretty(&value)
    }
    .map_err(|_| "artifact result could not be rendered".to_string())?;
    println!("{rendered}");
    Ok(())
}

fn emit_mutation(
    execution: handbook_engine::artifact_mutation::GenericMutationExecutionV1,
    json_mode: bool,
) -> Result<(), String> {
    let refused = execution.result.outcome == "refused";
    let mut value = serde_json::to_value(&execution.result)
        .map_err(|_| "artifact mutation result could not be rendered".to_string())?;
    value
        .as_object_mut()
        .expect("the lineage result is a JSON object")
        .insert(
            "execution_disposition".to_string(),
            Value::String(execution_disposition_name(execution.disposition).to_string()),
        );
    emit(value, json_mode)?;
    if refused {
        Err("the established artifact mutation was refused".to_string())
    } else {
        Ok(())
    }
}
