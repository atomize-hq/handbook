use clap::{Args, Subcommand, ValueEnum};
use handbook_sdk::artifact::{
    ArtifactAcquisitionMode, ArtifactAuthorityClass, ArtifactCandidateAppendRequest,
    ArtifactCandidateValidateRequest, ArtifactDocument, ArtifactInputDocument,
    ArtifactIntakeAppendRequest, ArtifactIntakeDefinitionRequest, ArtifactIntakeEvaluateRequest,
    ArtifactMutationDisposition, ArtifactMutationExecution, ArtifactMutationOutcome,
    ArtifactMutationRefusalCode, ArtifactMutationRefusalLayer, ArtifactPromoteRequest,
    ArtifactReadRequest, ArtifactSdk, ArtifactSelector, ArtifactValidateRequest,
    ArtifactValidationLayerStatus, MAX_ARTIFACT_INPUT_DOCUMENT_BYTES,
};
use serde_json::{json, Map, Number, Value};
use std::fs::File;
use std::io::{self, Read};
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
            let result = ArtifactSdk::open(&args.repository_root)
                .list_kinds()
                .map_err(|error| error.to_string())?;
            let value = json!({
                "operation_id": "artifact.kind.list",
                "selection_fingerprint": result.selection_fingerprint,
                "kinds": result.kinds.into_iter().map(|kind| json!({
                    "kind_ref": kind.kind_ref,
                    "definition_fingerprint": kind.definition_fingerprint,
                    "schema_ref": kind.schema_ref,
                })).collect::<Vec<_>>()
            });
            emit(value, args.json)
        }
        ArtifactCommand::ListInstances(args) => {
            let result = ArtifactSdk::open(&args.repository_root)
                .list_instances()
                .map_err(|error| error.to_string())?;
            let value = json!({
                "operation_id": "artifact.instance.list",
                "selection_fingerprint": result.selection_fingerprint,
                "instances": result.instances.into_iter().map(|instance| json!({
                    "instance_id": instance.instance_id,
                    "kind_ref": instance.kind_ref,
                    "canonical_path": instance.canonical_path,
                    "intake_definition_ref": instance.intake_definition_ref,
                })).collect::<Vec<_>>()
            });
            emit(value, args.json)
        }
        ArtifactCommand::Read(args) => {
            let result = ArtifactSdk::open(&args.repository_root)
                .read(ArtifactReadRequest {
                    selector: selector(&args.kind_ref, &args.instance_id),
                })
                .map_err(|error| error.to_string())?;
            emit(
                json!({
                    "operation_id": result.operation_id,
                    "operation_context_fingerprint": result.operation_context_fingerprint,
                    "kind_ref": result.kind_ref,
                    "instance_id": result.instance_id,
                    "canonical_path": result.canonical_path,
                    "artifact_fingerprint": result.artifact_fingerprint,
                    "content": document_to_value(result.content)?,
                }),
                args.json,
            )
        }
        ArtifactCommand::Validate(args) => {
            let result = ArtifactSdk::open(&args.repository_root)
                .validate(ArtifactValidateRequest {
                    selector: selector(&args.kind_ref, &args.instance_id),
                })
                .map_err(|error| error.to_string())?;
            emit(
                json!({
                    "operation_id": result.operation_id,
                    "operation_context_fingerprint": result.operation_context_fingerprint,
                    "artifact_fingerprint": result.artifact_fingerprint,
                    "outcome": "valid",
                    "content": document_to_value(result.content)?,
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
            let definition = ArtifactSdk::open(&args.repository_root)
                .intake_definition(ArtifactIntakeDefinitionRequest {
                    selector: selector(&args.kind_ref, &args.instance_id),
                })
                .map_err(|error| error.to_string())?;
            emit(
                json!({
                    "operation_id": "intake.definition.read",
                    "kind_ref": definition.kind_ref,
                    "instance_id": definition.instance_id,
                    "intake_definition_ref": definition.intake_definition_ref,
                    "intake_definition_fingerprint": definition.intake_definition_fingerprint,
                    "candidate_schema_ref": definition.candidate_schema_ref,
                    "coverage": definition.coverage.into_iter().map(|row| json!({
                        "coverage_id": row.coverage_id,
                        "target_paths": row.target_paths,
                        "minimum_specificity": format!("{:?}", row.minimum_specificity).to_lowercase(),
                    })).collect::<Vec<_>>()
                }),
                args.json,
            )
        }
        ArtifactCommand::IntakeEvaluate(args) => {
            let evaluation = ArtifactSdk::open(&args.repository_root)
                .evaluate_intake(ArtifactIntakeEvaluateRequest {
                    selector: selector(&args.kind_ref, &args.instance_id),
                    mode: acquisition_mode(args.mode),
                    expected_current_fingerprint: args.expected_current_fingerprint,
                    input: input_document(read_bounded_input(&args.from_inputs)?)?,
                })
                .map_err(|error| error.to_string())?;
            emit(document_to_value(evaluation.document)?, args.json)
        }
        ArtifactCommand::IntakeAppend(args) => {
            let execution = ArtifactSdk::open(&args.repository_root)
                .intake_append(ArtifactIntakeAppendRequest {
                    selector: selector(&args.kind_ref, &args.instance_id),
                    input: input_document(read_bounded_input(&args.from_request)?)?,
                })
                .map_err(|error| error.to_string())?;
            emit_mutation(execution, args.json)
        }
        ArtifactCommand::CandidateValidate(args) => {
            let preview = ArtifactSdk::open(&args.repository_root)
                .candidate_validate(ArtifactCandidateValidateRequest {
                    selector: selector(&args.kind_ref, &args.instance_id),
                    intake_record_ref: args.intake_record_ref,
                    intake_record_fingerprint: args.intake_record_fingerprint,
                    expected_current_artifact_fingerprint: args.expected_current_fingerprint,
                })
                .map_err(|error| error.to_string())?;
            emit(document_to_value(preview.document)?, args.json)
        }
        ArtifactCommand::CandidateAppend(args) => {
            let execution = ArtifactSdk::open(&args.repository_root)
                .candidate_append(ArtifactCandidateAppendRequest {
                    selector: selector(&args.kind_ref, &args.instance_id),
                    input: input_document(read_bounded_input(&args.from_request)?)?,
                })
                .map_err(|error| error.to_string())?;
            emit_mutation(execution, args.json)
        }
        ArtifactCommand::Promote(args) => {
            let execution = ArtifactSdk::open(&args.repository_root)
                .promote(ArtifactPromoteRequest {
                    selector: selector(&args.kind_ref, &args.instance_id),
                    input: input_document(read_bounded_input(&args.from_request)?)?,
                })
                .map_err(|error| error.to_string())?;
            emit_mutation(execution, args.json)
        }
    }
}

fn selector(kind_ref: &str, instance_id: &str) -> ArtifactSelector {
    ArtifactSelector::new(kind_ref, instance_id)
}

fn acquisition_mode(mode: IntakeModeArg) -> ArtifactAcquisitionMode {
    match mode {
        IntakeModeArg::GuidedAdaptive => ArtifactAcquisitionMode::GuidedAdaptive,
        IntakeModeArg::Express => ArtifactAcquisitionMode::Express,
        IntakeModeArg::AgentAssisted => ArtifactAcquisitionMode::AgentAssisted,
    }
}

fn layer_status(status: ArtifactValidationLayerStatus) -> &'static str {
    match status {
        ArtifactValidationLayerStatus::Pass => "pass",
        ArtifactValidationLayerStatus::NotApplicable => "not_applicable",
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

fn input_document(bytes: Vec<u8>) -> Result<ArtifactInputDocument, String> {
    ArtifactInputDocument::new(bytes).map_err(|error| error.to_string())
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

fn document_to_value(document: ArtifactDocument) -> Result<Value, String> {
    match document {
        ArtifactDocument::Null => Ok(Value::Null),
        ArtifactDocument::Boolean(value) => Ok(Value::Bool(value)),
        ArtifactDocument::Number(value) => value
            .parse::<Number>()
            .map(Value::Number)
            .map_err(|_| "artifact result could not be rendered".to_string()),
        ArtifactDocument::String(value) => Ok(Value::String(value)),
        ArtifactDocument::Array(values) => values
            .into_iter()
            .map(document_to_value)
            .collect::<Result<Vec<_>, _>>()
            .map(Value::Array),
        ArtifactDocument::Object(values) => values
            .into_iter()
            .map(|(key, value)| document_to_value(value).map(|value| (key, value)))
            .collect::<Result<Map<_, _>, _>>()
            .map(Value::Object),
    }
}

fn emit_mutation(execution: ArtifactMutationExecution, json_mode: bool) -> Result<(), String> {
    let refused = execution.result.outcome == ArtifactMutationOutcome::Refused;
    let result = execution.result;
    let value = json!({
        "schema_id": result.schema_id,
        "schema_version": result.schema_version,
        "operation_id": result.operation_id,
        "transaction_id": result.transaction_id,
        "request_fingerprint": result.request_fingerprint,
        "outcome": mutation_outcome_name(result.outcome),
        "refusal": result.refusal.map(|refusal| json!({
            "code": mutation_refusal_code_name(refusal.code),
            "layer": mutation_refusal_layer_name(refusal.layer),
            "expected_fingerprint": refusal.expected_fingerprint,
            "observed_fingerprint": refusal.observed_fingerprint,
        })),
        "authoritative_outputs": result.authoritative_outputs.into_iter().map(|output| json!({
            "authority_class": authority_class_name(output.authority_class),
            "ref": output.reference,
            "fingerprint": output.fingerprint,
        })).collect::<Vec<_>>(),
        "internal_transaction_evidence_ref": result.transaction_evidence.as_ref().map(|evidence| evidence.reference.as_str()),
        "internal_transaction_evidence_fingerprint": result.transaction_evidence.as_ref().map(|evidence| evidence.fingerprint.as_str()),
        "result_fingerprint": result.result_fingerprint,
        "execution_disposition": mutation_disposition_name(execution.disposition),
    });
    emit(value, json_mode)?;
    if refused {
        Err("the established artifact mutation was refused".to_string())
    } else {
        Ok(())
    }
}

fn authority_class_name(authority_class: ArtifactAuthorityClass) -> &'static str {
    match authority_class {
        ArtifactAuthorityClass::SubordinateClosure => "subordinate_closure",
        ArtifactAuthorityClass::SemanticRecord => "semantic_record",
        ArtifactAuthorityClass::CanonicalTruth => "canonical_truth",
    }
}

fn mutation_outcome_name(outcome: ArtifactMutationOutcome) -> &'static str {
    match outcome {
        ArtifactMutationOutcome::Committed => "committed",
        ArtifactMutationOutcome::Refused => "refused",
    }
}

fn mutation_disposition_name(disposition: ArtifactMutationDisposition) -> &'static str {
    match disposition {
        ArtifactMutationDisposition::Committed => "committed",
        ArtifactMutationDisposition::Refused => "refused",
        ArtifactMutationDisposition::Replayed => "replayed",
    }
}

fn mutation_refusal_code_name(code: ArtifactMutationRefusalCode) -> &'static str {
    match code {
        ArtifactMutationRefusalCode::CanonicalSyntaxInvalid => "canonical_syntax_invalid",
        ArtifactMutationRefusalCode::StructuralValidationFailed => "structural_validation_failed",
        ArtifactMutationRefusalCode::IntakeCoverageBlocked => "intake_coverage_blocked",
        ArtifactMutationRefusalCode::StaleBasis => "stale_basis",
        ArtifactMutationRefusalCode::StaleCurrentArtifact => "stale_current_artifact",
        ArtifactMutationRefusalCode::OperationIneligible => "operation_ineligible",
        ArtifactMutationRefusalCode::PublicationBasisConflict => "publication_basis_conflict",
    }
}

fn mutation_refusal_layer_name(layer: ArtifactMutationRefusalLayer) -> &'static str {
    match layer {
        ArtifactMutationRefusalLayer::CanonicalSyntax => "canonical_syntax",
        ArtifactMutationRefusalLayer::Structural => "structural",
        ArtifactMutationRefusalLayer::Intake => "intake",
        ArtifactMutationRefusalLayer::Currentness => "currentness",
        ArtifactMutationRefusalLayer::Eligibility => "eligibility",
        ArtifactMutationRefusalLayer::Publication => "publication",
    }
}
