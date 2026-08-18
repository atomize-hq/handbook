use crate::{exit_policy, shell_shared::discover_managed_repo_root};
use clap::{Args, Subcommand, ValueEnum};
use handbook_sdk::contract_membrane::{
    canonical_json_bytes, parse_canonical_json, BootstrapFacadeOutcomeKindV1, CapabilityConsumer,
    CapabilityConsumerError, CapabilityEntry, CatalogKindV1, CatalogPageSize,
    CatalogReadOutcomeKindV1, CatalogReadResponseBodyV1, ContractLimits, ExactBinding, ExactRef,
    Fingerprint, OperationClass, OperationTransport, RequestId, SemVer, TypedCatalogCursor,
};
use serde::Serialize;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use std::process::ExitCode;

#[derive(Args, Debug)]
pub(crate) struct CapabilityArgs {
    #[command(subcommand)]
    command: CapabilityCommand,
}

#[derive(Subcommand, Debug)]
enum CapabilityCommand {
    /// Negotiate the supported API version and read the first capability page.
    Bootstrap(BootstrapArgs),
    /// Read one exact typed catalog page.
    CatalogRead(CatalogReadArgs),
    /// Read exact checked-in schema bytes selected by one manifest entry content fingerprint.
    SchemaRead(SchemaReadArgs),
    /// Invoke one discovered read-only capability through its exact current binding.
    Invoke(InvokeArgs),
}

#[derive(Args, Debug)]
struct BootstrapArgs {
    /// Adapter client API version to negotiate.
    #[arg(long = "client-version")]
    client_version: String,
    /// Requested catalog page size in the closed range 1..=128.
    #[arg(long = "page-size")]
    page_size: u64,
    /// Optional bounded request correlation identifier.
    #[arg(long)]
    correlation: Option<String>,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum CatalogKindArg {
    Capabilities,
    Schemas,
    Profiles,
}

#[derive(Args, Debug)]
struct CatalogReadArgs {
    /// Existing typed catalog family.
    #[arg(long = "catalog-kind", value_enum)]
    catalog_kind: CatalogKindArg,
    /// Requested catalog page size in the closed range 1..=128.
    #[arg(long = "page-size")]
    page_size: u64,
    /// Optional canonical JSON encoding of one exact typed catalog cursor.
    #[arg(long)]
    cursor: Option<String>,
}

#[derive(Args, Debug)]
struct SchemaReadArgs {
    /// Exact schema reference selected from the current schema catalog.
    #[arg(long = "schema-ref")]
    schema_ref: String,
    /// Content fingerprint from that exact current schema manifest entry.
    #[arg(long = "manifest-fingerprint")]
    manifest_fingerprint: String,
}

#[derive(Args, Debug)]
struct InvokeArgs {
    /// Exact operation reference selected from live capability discovery.
    #[arg(long = "operation-ref")]
    operation_ref: String,
    /// Definition fingerprint paired with the discovered operation reference.
    #[arg(long = "definition-fingerprint")]
    definition_fingerprint: String,
    /// Fingerprint of the exact discovered capability entry.
    #[arg(long = "capability-entry-fingerprint")]
    capability_entry_fingerprint: String,
    /// Optional bounded request correlation identifier.
    #[arg(long)]
    correlation: Option<String>,
    /// Read one bounded structured JSON body from a path or `-` for stdin.
    #[arg(long = "from-input", value_name = "path|-")]
    from_input: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum AdapterError {
    Acquisition,
    Repository,
    Version,
    PageSize,
    Correlation,
    Cursor,
    OperationBinding,
    DefinitionFingerprint,
    CapabilityEntryFingerprint,
    OperationUnavailable,
    OperationSelectionStale,
    OperationClass,
    TransportUnavailable,
    InputUnavailable,
    InputRejected,
    SchemaBinding,
    ManifestFingerprint,
    SchemaUnavailable,
    Contract,
}

pub(crate) fn run(args: CapabilityArgs) -> ExitCode {
    let cwd = match std::env::current_dir() {
        Ok(value) => value,
        Err(_) => return emit_error(AdapterError::Acquisition),
    };
    let consumer = match open_consumer_at(&cwd) {
        Ok(value) => value,
        Err(error) => return emit_error(error),
    };
    match args.command {
        CapabilityCommand::Bootstrap(args) => bootstrap(&consumer, args),
        CapabilityCommand::CatalogRead(args) => catalog_read(&consumer, args),
        CapabilityCommand::SchemaRead(args) => schema_read(&consumer, args),
        CapabilityCommand::Invoke(args) => invoke(&consumer, args),
    }
}

fn open_consumer_at(start: &Path) -> Result<CapabilityConsumer, AdapterError> {
    CapabilityConsumer::open(discover_managed_repo_root(start))
        .map_err(|_| AdapterError::Repository)
}

fn bootstrap(consumer: &CapabilityConsumer, args: BootstrapArgs) -> ExitCode {
    let version = match SemVer::parse(&args.client_version) {
        Ok(value) => value,
        Err(_) => return emit_error(AdapterError::Version),
    };
    let page_size = match CatalogPageSize::new(args.page_size) {
        Ok(value) => value,
        Err(_) => return emit_error(AdapterError::PageSize),
    };
    let correlation = match args
        .correlation
        .as_deref()
        .map(RequestId::parse)
        .transpose()
    {
        Ok(value) => value,
        Err(_) => return emit_error(AdapterError::Correlation),
    };
    let response = match consumer.bootstrap(version, page_size, correlation) {
        Ok(value) => value,
        Err(_) => return emit_error(AdapterError::Contract),
    };
    let semantic_status = match response.outcome_kind() {
        BootstrapFacadeOutcomeKindV1::Ok => exit_policy::success(),
        BootstrapFacadeOutcomeKindV1::Refused => exit_policy::failure(),
    };
    emit_json(&response, semantic_status)
}

fn catalog_read(consumer: &CapabilityConsumer, args: CatalogReadArgs) -> ExitCode {
    let page_size = match CatalogPageSize::new(args.page_size) {
        Ok(value) => value,
        Err(_) => return emit_error(AdapterError::PageSize),
    };
    let cursor = match args
        .cursor
        .as_deref()
        .map(|value| {
            parse_canonical_json::<TypedCatalogCursor>(value.as_bytes(), &ContractLimits::packet0())
        })
        .transpose()
    {
        Ok(value) => value,
        Err(_) => return emit_error(AdapterError::Cursor),
    };
    let catalog_kind = match args.catalog_kind {
        CatalogKindArg::Capabilities => CatalogKindV1::Capabilities,
        CatalogKindArg::Schemas => CatalogKindV1::Schemas,
        CatalogKindArg::Profiles => CatalogKindV1::Profiles,
    };
    let response = match consumer.catalog_read(catalog_kind, page_size, cursor) {
        Ok(value) => value,
        Err(_) => return emit_error(AdapterError::Contract),
    };
    emit_catalog_json(&response)
}

fn emit_catalog_json(
    response: &handbook_sdk::contract_membrane::CatalogReadResponseV1,
) -> ExitCode {
    let (bytes, semantic_status) = match framed_catalog_json(response) {
        Ok(value) => value,
        Err(_) => return emit_error(AdapterError::Contract),
    };
    exit_policy::emit_stdout(&bytes, semantic_status)
}

fn framed_catalog_json(
    response: &handbook_sdk::contract_membrane::CatalogReadResponseV1,
) -> Result<(Vec<u8>, ExitCode), AdapterError> {
    let semantic_status = match response.outcome_kind() {
        CatalogReadOutcomeKindV1::Page => exit_policy::success(),
        CatalogReadOutcomeKindV1::CursorMismatch | CatalogReadOutcomeKindV1::Blocked => {
            exit_policy::failure()
        }
    };
    framed_json(response, semantic_status)
}

fn schema_read(consumer: &CapabilityConsumer, args: SchemaReadArgs) -> ExitCode {
    let schema_ref = match ExactRef::parse(&args.schema_ref) {
        Ok(value) => value,
        Err(_) => return emit_error(AdapterError::SchemaBinding),
    };
    let manifest_fingerprint = match Fingerprint::parse(&args.manifest_fingerprint) {
        Ok(value) => value,
        Err(_) => return emit_error(AdapterError::ManifestFingerprint),
    };
    let schema = ExactBinding::new(schema_ref, manifest_fingerprint);
    let bytes = match consumer.schema_bytes(&schema) {
        Ok(value) => value,
        Err(_) => return emit_error(AdapterError::SchemaUnavailable),
    };
    exit_policy::emit_stdout(bytes, exit_policy::success())
}

fn invoke(consumer: &CapabilityConsumer, args: InvokeArgs) -> ExitCode {
    let operation_ref = match ExactRef::parse(&args.operation_ref) {
        Ok(value) => value,
        Err(_) => return emit_error(AdapterError::OperationBinding),
    };
    let definition_fingerprint = match Fingerprint::parse(&args.definition_fingerprint) {
        Ok(value) => value,
        Err(_) => return emit_error(AdapterError::DefinitionFingerprint),
    };
    let entry_fingerprint = match Fingerprint::parse(&args.capability_entry_fingerprint) {
        Ok(value) => value,
        Err(_) => return emit_error(AdapterError::CapabilityEntryFingerprint),
    };
    let correlation = match args
        .correlation
        .as_deref()
        .map(RequestId::parse)
        .transpose()
    {
        Ok(value) => value,
        Err(_) => return emit_error(AdapterError::Correlation),
    };
    let capability = match discover_capability(consumer, &operation_ref) {
        Ok(value) => value,
        Err(error) => return emit_error(error),
    };
    if capability.class() != OperationClass::ReadOnly {
        return emit_error(AdapterError::OperationClass);
    }
    if consumer.schema_bytes(capability.request_schema()).is_err() {
        return emit_error(AdapterError::SchemaUnavailable);
    }
    let limits = ContractLimits::packet0();
    let raw_body = match read_bounded_input(&args.from_input, limits.document_bytes()) {
        Ok(value) => value,
        Err(error) => return emit_error(error),
    };
    let operation = ExactBinding::new(operation_ref, definition_fingerprint);
    let response = match consumer.invoke(
        &operation,
        &entry_fingerprint,
        OperationTransport::CliJson,
        correlation,
        &raw_body,
    ) {
        Ok(value) => value,
        Err(error) => return emit_error(invocation_error(error)),
    };
    let semantic_status = exit_policy::terminal(response.terminal_status());
    emit_json(&response, semantic_status)
}

fn discover_capability(
    consumer: &CapabilityConsumer,
    operation_ref: &ExactRef,
) -> Result<CapabilityEntry, AdapterError> {
    let page_size = CatalogPageSize::new(128).map_err(|_| AdapterError::Contract)?;
    let mut cursor = None;
    loop {
        let response = consumer
            .catalog_read(CatalogKindV1::Capabilities, page_size, cursor)
            .map_err(|_| AdapterError::Contract)?;
        let CatalogReadResponseBodyV1::CapabilityPage(page) = response.response() else {
            return Err(AdapterError::Contract);
        };
        if let Some(capability) = page
            .entries()
            .iter()
            .find(|entry| entry.operation().reference() == operation_ref)
        {
            return Ok(capability.clone());
        }
        cursor = match page.next_cursor() {
            Some(value) => Some(value.clone()),
            None => return Err(AdapterError::OperationUnavailable),
        };
    }
}

fn read_bounded_input(path: &str, max_bytes: usize) -> Result<Vec<u8>, AdapterError> {
    let mut reader: Box<dyn Read> = if path == "-" {
        Box::new(io::stdin())
    } else {
        Box::new(File::open(path).map_err(|_| AdapterError::InputUnavailable)?)
    };
    let mut bytes = Vec::new();
    reader
        .by_ref()
        .take((max_bytes + 1) as u64)
        .read_to_end(&mut bytes)
        .map_err(|_| AdapterError::InputUnavailable)?;
    if bytes.len() > max_bytes {
        return Err(AdapterError::InputRejected);
    }
    Ok(bytes)
}

fn invocation_error(error: CapabilityConsumerError) -> AdapterError {
    match error {
        CapabilityConsumerError::CapabilityUnavailable => AdapterError::OperationUnavailable,
        CapabilityConsumerError::DefinitionBindingMismatch
        | CapabilityConsumerError::CapabilityEntryMismatch => AdapterError::OperationSelectionStale,
        CapabilityConsumerError::SchemaUnavailable => AdapterError::SchemaUnavailable,
        CapabilityConsumerError::TransportUnavailable => AdapterError::TransportUnavailable,
        CapabilityConsumerError::InvalidInput => AdapterError::InputRejected,
        CapabilityConsumerError::RepositoryUnavailable
        | CapabilityConsumerError::ClockUnavailable
        | CapabilityConsumerError::WitnessUnavailable
        | CapabilityConsumerError::ContractUnavailable => AdapterError::Contract,
    }
}

fn emit_json(value: &impl Serialize, semantic_status: ExitCode) -> ExitCode {
    let bytes = match framed_json(value, semantic_status) {
        Ok((value, _)) => value,
        Err(_) => return emit_error(AdapterError::Contract),
    };
    exit_policy::emit_stdout(&bytes, semantic_status)
}

fn framed_json(
    value: &impl Serialize,
    semantic_status: ExitCode,
) -> Result<(Vec<u8>, ExitCode), AdapterError> {
    let mut bytes = canonical_json_bytes(value, &ContractLimits::packet0())
        .map_err(|_| AdapterError::Contract)?;
    bytes.push(b'\n');
    Ok((bytes, semantic_status))
}

fn emit_error(error: AdapterError) -> ExitCode {
    let stderr = io::stderr();
    let mut lock = stderr.lock();
    emit_error_to(&mut lock, error)
}

fn emit_error_to(writer: &mut dyn Write, error: AdapterError) -> ExitCode {
    let message = match error {
        AdapterError::Acquisition => {
            b"handbook capability: working directory unavailable\n".as_slice()
        }
        AdapterError::Repository => b"handbook capability: repository unavailable\n".as_slice(),
        AdapterError::Version => b"handbook capability: client version rejected\n".as_slice(),
        AdapterError::PageSize => b"handbook capability: page size rejected\n".as_slice(),
        AdapterError::Correlation => b"handbook capability: correlation rejected\n".as_slice(),
        AdapterError::Cursor => b"handbook capability: catalog cursor rejected\n".as_slice(),
        AdapterError::OperationBinding => {
            b"handbook capability: operation binding rejected\n".as_slice()
        }
        AdapterError::DefinitionFingerprint => {
            b"handbook capability: definition fingerprint rejected\n".as_slice()
        }
        AdapterError::CapabilityEntryFingerprint => {
            b"handbook capability: capability entry fingerprint rejected\n".as_slice()
        }
        AdapterError::OperationUnavailable => {
            b"handbook capability: operation unavailable\n".as_slice()
        }
        AdapterError::OperationSelectionStale => {
            b"handbook capability: operation selection stale\n".as_slice()
        }
        AdapterError::OperationClass => {
            b"handbook capability: operation class rejected\n".as_slice()
        }
        AdapterError::TransportUnavailable => {
            b"handbook capability: transport unavailable\n".as_slice()
        }
        AdapterError::InputUnavailable => b"handbook capability: input unavailable\n".as_slice(),
        AdapterError::InputRejected => b"handbook capability: input rejected\n".as_slice(),
        AdapterError::SchemaBinding => b"handbook capability: schema binding rejected\n".as_slice(),
        AdapterError::ManifestFingerprint => {
            b"handbook capability: manifest fingerprint rejected\n".as_slice()
        }
        AdapterError::SchemaUnavailable => b"handbook capability: schema unavailable\n".as_slice(),
        AdapterError::Contract => b"handbook capability: contract unavailable\n".as_slice(),
    };
    let _ = writer.write_all(message).and_then(|_| writer.flush());
    exit_policy::failure()
}

#[cfg(test)]
mod tests {
    use super::*;
    use handbook_sdk::contract_membrane::{
        BootstrapFacadeResponseBodyV1, CatalogReadBlockedV1, CatalogReadResponseV1,
    };

    #[test]
    fn repository_failure_is_one_fixed_redacted_stderr_line() {
        let root = tempfile::tempdir().unwrap();
        let unavailable = root.path().join("missing");
        assert!(matches!(
            open_consumer_at(&unavailable),
            Err(AdapterError::Repository)
        ));

        let mut stderr = Vec::new();
        assert_eq!(
            emit_error_to(&mut stderr, AdapterError::Repository),
            exit_policy::failure()
        );
        assert_eq!(stderr, b"handbook capability: repository unavailable\n");
        assert!(!String::from_utf8_lossy(&stderr).contains(unavailable.to_str().unwrap()));
    }

    #[test]
    fn admitted_blocked_catalog_response_keeps_typed_stdout_and_failure_exit() {
        let root = tempfile::tempdir().unwrap();
        let consumer = CapabilityConsumer::open(root.path()).unwrap();
        let limits = ContractLimits::packet0();
        let bootstrap = consumer
            .bootstrap(
                SemVer::parse("1.0.0").unwrap(),
                CatalogPageSize::new(1).unwrap(),
                None,
            )
            .unwrap();
        let BootstrapFacadeResponseBodyV1::Ok(bootstrap) = bootstrap.response() else {
            panic!("current bootstrap was refused");
        };
        let blocked = CatalogReadBlockedV1::expired(
            bootstrap
                .descriptor()
                .schemas()
                .catalog_read_blocked()
                .clone(),
            bootstrap.first_capability_page().root().clone(),
            bootstrap.descriptor().schemas().catalog_root().clone(),
            &limits,
        )
        .unwrap();
        let response =
            CatalogReadResponseV1::blocked(CatalogKindV1::Capabilities, blocked, &limits).unwrap();

        let (bytes, status) = framed_catalog_json(&response).unwrap();
        assert_eq!(status, exit_policy::failure());
        assert_eq!(bytes.iter().filter(|byte| **byte == b'\n').count(), 1);
        assert!(bytes.ends_with(b"\n"));
        let admitted =
            CatalogReadResponseV1::parse_and_admit(&bytes[..bytes.len() - 1], &limits).unwrap();
        assert_eq!(admitted.outcome_kind(), CatalogReadOutcomeKindV1::Blocked);
    }
}
