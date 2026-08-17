use crate::{doctor_rendering, exit_policy};
use clap::{Args, Subcommand, ValueEnum};
use handbook_sdk::artifact::{
    ArtifactAcquisitionMode, ArtifactInputDocument, ArtifactSdk, MAX_ARTIFACT_INPUT_DOCUMENT_BYTES,
};
use handbook_sdk::contract_membrane::{
    artifact_operation_request, render_artifact_operation_legacy, ArtifactOperationInputV1,
    ContractLimits, Fingerprint, InvocationResponseV1, OperationResponseV1, TransportFailureKind,
    TransportFailureV1,
};
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
    /// Emit the deprecated pre-HCM-4.3 JSON projection.
    #[arg(id = "legacy_json", long = "legacy-json", conflicts_with = "json")]
    legacy_json: bool,
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
    /// Emit the deprecated pre-HCM-4.3 JSON projection.
    #[arg(id = "legacy_json", long = "legacy-json", conflicts_with = "json")]
    legacy_json: bool,
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
    /// Emit the deprecated pre-HCM-4.3 JSON projection.
    #[arg(id = "legacy_json", long = "legacy-json", conflicts_with = "json")]
    legacy_json: bool,
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
    /// Emit the deprecated pre-HCM-4.3 JSON projection.
    #[arg(id = "legacy_json", long = "legacy-json", conflicts_with = "json")]
    legacy_json: bool,
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
    expected_current_artifact_fingerprint: Option<String>,
    /// Emit the deprecated pre-HCM-4.3 JSON projection.
    #[arg(id = "legacy_json", long = "legacy-json", conflicts_with = "json")]
    legacy_json: bool,
}

pub(crate) fn run(args: ArtifactArgs, machine_json: bool) -> ExitCode {
    if machine_json && args.command.legacy_json() {
        eprintln!("error: the argument '--json' cannot be used with '--legacy-json'");
        return ExitCode::from(2);
    }
    let limits = ContractLimits::packet0();
    let execution = match execute(args.command) {
        Ok(execution) => execution,
        Err(error) if machine_json => {
            return emit_transport_failure(error.failure_kind(), error.reason_code(), &limits)
        }
        Err(error) => {
            eprintln!("artifact transport failed: {}", error.detail());
            return exit_policy::failure();
        }
    };
    emit_execution(execution, machine_json, &limits)
}

impl ArtifactCommand {
    fn legacy_json(&self) -> bool {
        match self {
            Self::ListKinds(args) | Self::ListInstances(args) => args.legacy_json,
            Self::Read(args) | Self::Validate(args) | Self::IntakeDefinition(args) => {
                args.legacy_json
            }
            Self::IntakeEvaluate(args) => args.legacy_json,
            Self::IntakeAppend(args) | Self::CandidateAppend(args) | Self::Promote(args) => {
                args.legacy_json
            }
            Self::CandidateValidate(args) => args.legacy_json,
        }
    }
}

struct ArtifactExecution {
    response: OperationResponseV1,
    legacy_json: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ArtifactExecutionErrorKind {
    Acquisition,
    Decoding,
    Adapter,
}

#[derive(Debug)]
struct ArtifactExecutionError {
    kind: ArtifactExecutionErrorKind,
    detail: String,
}

impl ArtifactExecutionError {
    fn new(kind: ArtifactExecutionErrorKind, detail: impl Into<String>) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }

    fn failure_kind(&self) -> TransportFailureKind {
        match self.kind {
            ArtifactExecutionErrorKind::Acquisition => TransportFailureKind::Acquisition,
            ArtifactExecutionErrorKind::Decoding => TransportFailureKind::Decoding,
            ArtifactExecutionErrorKind::Adapter => TransportFailureKind::Adapter,
        }
    }

    fn reason_code(&self) -> &'static str {
        match self.kind {
            ArtifactExecutionErrorKind::Acquisition => "transport.artifact_input_unavailable",
            ArtifactExecutionErrorKind::Decoding => "transport.artifact_input_invalid",
            ArtifactExecutionErrorKind::Adapter => "transport.artifact_adapter_failed",
        }
    }

    fn detail(&self) -> &str {
        &self.detail
    }
}

fn execute(command: ArtifactCommand) -> Result<ArtifactExecution, ArtifactExecutionError> {
    let (repository_root, legacy_json, input) = match command {
        ArtifactCommand::ListKinds(args) => (
            args.repository_root,
            args.legacy_json,
            ArtifactOperationInputV1::KindList,
        ),
        ArtifactCommand::ListInstances(args) => (
            args.repository_root,
            args.legacy_json,
            ArtifactOperationInputV1::InstanceList,
        ),
        ArtifactCommand::Read(args) => (
            args.repository_root,
            args.legacy_json,
            ArtifactOperationInputV1::Read {
                kind_ref: args.kind_ref,
                instance_id: args.instance_id,
            },
        ),
        ArtifactCommand::Validate(args) => (
            args.repository_root,
            args.legacy_json,
            ArtifactOperationInputV1::Validate {
                kind_ref: args.kind_ref,
                instance_id: args.instance_id,
            },
        ),
        ArtifactCommand::IntakeDefinition(args) => (
            args.repository_root,
            args.legacy_json,
            ArtifactOperationInputV1::IntakeDefinitionRead {
                kind_ref: args.kind_ref,
                instance_id: args.instance_id,
            },
        ),
        ArtifactCommand::IntakeEvaluate(args) => (
            args.repository_root,
            args.legacy_json,
            ArtifactOperationInputV1::IntakeCoverageEvaluate {
                kind_ref: args.kind_ref,
                instance_id: args.instance_id,
                mode: acquisition_mode(args.mode),
                expected_current_artifact_fingerprint: expected_current_fingerprint(
                    args.expected_current_fingerprint,
                )?,
                input: input_document(read_bounded_input(&args.from_inputs)?)?,
            },
        ),
        ArtifactCommand::IntakeAppend(args) => (
            args.repository_root,
            args.legacy_json,
            ArtifactOperationInputV1::IntakeRecordAppend {
                kind_ref: args.kind_ref,
                instance_id: args.instance_id,
                input: input_document(read_bounded_input(&args.from_request)?)?,
            },
        ),
        ArtifactCommand::CandidateValidate(args) => (
            args.repository_root,
            args.legacy_json,
            ArtifactOperationInputV1::CandidateValidate {
                kind_ref: args.kind_ref,
                instance_id: args.instance_id,
                intake_record_ref: args.intake_record_ref,
                intake_record_fingerprint: args.intake_record_fingerprint,
                expected_current_artifact_fingerprint: expected_current_fingerprint(
                    args.expected_current_artifact_fingerprint,
                )?,
            },
        ),
        ArtifactCommand::CandidateAppend(args) => (
            args.repository_root,
            args.legacy_json,
            ArtifactOperationInputV1::CandidateAppend {
                kind_ref: args.kind_ref,
                instance_id: args.instance_id,
                input: input_document(read_bounded_input(&args.from_request)?)?,
            },
        ),
        ArtifactCommand::Promote(args) => (
            args.repository_root,
            args.legacy_json,
            ArtifactOperationInputV1::CandidatePromote {
                kind_ref: args.kind_ref,
                instance_id: args.instance_id,
                input: input_document(read_bounded_input(&args.from_request)?)?,
            },
        ),
    };
    let request = artifact_operation_request(None, input).map_err(|_| {
        ArtifactExecutionError::new(
            ArtifactExecutionErrorKind::Decoding,
            "artifact request did not satisfy its exact contract",
        )
    })?;
    let response = ArtifactSdk::open(repository_root)
        .execute_operation(&request)
        .map_err(|_| {
            ArtifactExecutionError::new(
                ArtifactExecutionErrorKind::Adapter,
                "artifact response could not be constructed",
            )
        })?;
    Ok(ArtifactExecution {
        response,
        legacy_json,
    })
}

fn emit_execution(
    execution: ArtifactExecution,
    machine_json: bool,
    limits: &ContractLimits,
) -> ExitCode {
    let semantic_exit = exit_policy::terminal(execution.response.terminal().status());
    if machine_json {
        let bytes = InvocationResponseV1::operation_response(execution.response, limits)
            .map_err(|_| ())
            .and_then(|invocation| {
                doctor_rendering::render_invocation(&invocation).map_err(|_| ())
            });
        return match bytes {
            Ok(bytes) => exit_policy::emit_stdout(&bytes, semantic_exit),
            Err(()) => emit_transport_failure(
                TransportFailureKind::Serialization,
                "transport.serialization_failed",
                limits,
            ),
        };
    }

    match render_artifact_operation_legacy(&execution.response, !execution.legacy_json) {
        Ok(bytes) => exit_policy::emit_stdout(&bytes, semantic_exit),
        Err(_) => {
            eprintln!("handbook: artifact result rendering failed");
            exit_policy::failure()
        }
    }
}

fn acquisition_mode(mode: IntakeModeArg) -> ArtifactAcquisitionMode {
    match mode {
        IntakeModeArg::GuidedAdaptive => ArtifactAcquisitionMode::GuidedAdaptive,
        IntakeModeArg::Express => ArtifactAcquisitionMode::Express,
        IntakeModeArg::AgentAssisted => ArtifactAcquisitionMode::AgentAssisted,
    }
}

fn expected_current_fingerprint(
    value: Option<String>,
) -> Result<Option<String>, ArtifactExecutionError> {
    match value {
        Some(value) if value == "absent" => Ok(None),
        Some(value) if Fingerprint::parse(&value).is_ok() => Ok(Some(value)),
        Some(_) => Err(ArtifactExecutionError::new(
            ArtifactExecutionErrorKind::Decoding,
            "expected current fingerprint is invalid",
        )),
        None => Ok(None),
    }
}

fn read_bounded_input(path: &str) -> Result<Vec<u8>, ArtifactExecutionError> {
    let mut reader: Box<dyn Read> = if path == "-" {
        Box::new(io::stdin())
    } else {
        Box::new(File::open(path).map_err(|_| {
            ArtifactExecutionError::new(
                ArtifactExecutionErrorKind::Acquisition,
                "input document could not be opened",
            )
        })?)
    };
    let mut bytes = Vec::new();
    reader
        .by_ref()
        .take((MAX_ARTIFACT_INPUT_DOCUMENT_BYTES + 1) as u64)
        .read_to_end(&mut bytes)
        .map_err(|_| {
            ArtifactExecutionError::new(
                ArtifactExecutionErrorKind::Acquisition,
                "input document could not be read",
            )
        })?;
    if bytes.len() > MAX_ARTIFACT_INPUT_DOCUMENT_BYTES {
        return Err(ArtifactExecutionError::new(
            ArtifactExecutionErrorKind::Decoding,
            "input document exceeds the 1 MiB limit",
        ));
    }
    Ok(bytes)
}

fn input_document(bytes: Vec<u8>) -> Result<ArtifactInputDocument, ArtifactExecutionError> {
    ArtifactInputDocument::new(bytes).map_err(|error| {
        ArtifactExecutionError::new(ArtifactExecutionErrorKind::Decoding, error.to_string())
    })
}

fn emit_transport_failure(
    kind: TransportFailureKind,
    reason_code: &str,
    limits: &ContractLimits,
) -> ExitCode {
    let bytes = TransportFailureV1::adapter_error(kind, reason_code, limits)
        .and_then(|failure| InvocationResponseV1::transport_failure(failure, limits))
        .map_err(|_| ())
        .and_then(|invocation| doctor_rendering::render_invocation(&invocation).map_err(|_| ()));
    match bytes {
        Ok(bytes) => exit_policy::emit_stdout(&bytes, exit_policy::failure()),
        Err(()) => {
            eprintln!("handbook: failed to prepare transport-failure response");
            exit_policy::failure()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn artifact_transport_failures_keep_acquisition_decoding_and_adapter_distinct() {
        for (kind, expected_kind, expected_reason) in [
            (
                ArtifactExecutionErrorKind::Acquisition,
                TransportFailureKind::Acquisition,
                "transport.artifact_input_unavailable",
            ),
            (
                ArtifactExecutionErrorKind::Decoding,
                TransportFailureKind::Decoding,
                "transport.artifact_input_invalid",
            ),
            (
                ArtifactExecutionErrorKind::Adapter,
                TransportFailureKind::Adapter,
                "transport.artifact_adapter_failed",
            ),
        ] {
            let error = ArtifactExecutionError::new(kind, "arbitrary private detail");
            assert_eq!(error.failure_kind(), expected_kind);
            assert_eq!(error.reason_code(), expected_reason);
        }
    }
}
