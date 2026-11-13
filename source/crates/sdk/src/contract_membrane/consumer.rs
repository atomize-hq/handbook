use super::serialization::parse_structured_json;
use super::{
    canonical_json_bytes, BootstrapFacadeResponseV1, BootstrapOutcome, BootstrapRequestV1,
    CatalogKindV1, CatalogPageSize, CatalogReadRequestV1, CatalogReadResponseV1, ContractError,
    ContractLimits, ContractValue, ExactBinding, Fingerprint, InvocationResponseV1,
    OperationTransport, ProductionCatalogRead, ProductionWitness, RequestId, SemVer,
    TypedCatalogCursor, UtcTimestamp,
};
use std::path::{Path, PathBuf};
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

const RETENTION_UNTIL_UTC: &str = "9999-12-31T23:59:59Z";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CapabilityConsumerError {
    RepositoryUnavailable,
    ClockUnavailable,
    WitnessUnavailable,
    CapabilityUnavailable,
    DefinitionBindingMismatch,
    CapabilityEntryMismatch,
    SchemaUnavailable,
    TransportUnavailable,
    InvalidInput,
    ContractUnavailable,
}

pub struct CapabilityConsumer {
    repo_root: PathBuf,
    witness: ProductionWitness,
    limits: ContractLimits,
}

impl CapabilityConsumer {
    pub fn open(repo_root: impl AsRef<Path>) -> Result<Self, CapabilityConsumerError> {
        let repo_root = std::fs::canonicalize(repo_root)
            .map_err(|_| CapabilityConsumerError::RepositoryUnavailable)?;
        if !repo_root.is_dir() {
            return Err(CapabilityConsumerError::RepositoryUnavailable);
        }
        let retention_until_utc = UtcTimestamp::parse(RETENTION_UNTIL_UTC)
            .map_err(|_| CapabilityConsumerError::WitnessUnavailable)?;
        let witness = ProductionWitness::from_current(retention_until_utc)
            .map_err(|_| CapabilityConsumerError::WitnessUnavailable)?;
        Ok(Self {
            repo_root,
            witness,
            limits: ContractLimits::packet0(),
        })
    }

    pub fn bootstrap(
        &self,
        api_version: SemVer,
        page_size: CatalogPageSize,
        request_id: Option<RequestId>,
    ) -> Result<BootstrapFacadeResponseV1, CapabilityConsumerError> {
        let request = BootstrapRequestV1::new(
            self.witness
                .descriptor()
                .schemas()
                .bootstrap_request()
                .clone(),
            request_id,
            self.witness.descriptor().binding(),
            api_version,
            page_size,
            &self.limits,
        )
        .map_err(|_| CapabilityConsumerError::ContractUnavailable)?;
        let response = match self
            .witness
            .bootstrap(&request, &current_utc()?, &self.limits)
            .map_err(|_| CapabilityConsumerError::ContractUnavailable)?
        {
            BootstrapOutcome::Ok(response) => {
                BootstrapFacadeResponseV1::ok(*response, &self.limits)
            }
            BootstrapOutcome::Refused(response) => {
                BootstrapFacadeResponseV1::refused(*response, &self.limits)
            }
        }
        .map_err(|_| CapabilityConsumerError::ContractUnavailable)?;
        readmit_bootstrap(response, &self.limits)
    }

    pub fn catalog_read(
        &self,
        catalog_kind: CatalogKindV1,
        page_size: CatalogPageSize,
        cursor: Option<TypedCatalogCursor>,
    ) -> Result<CatalogReadResponseV1, CapabilityConsumerError> {
        let request = CatalogReadRequestV1::new(catalog_kind, page_size, cursor, &self.limits)
            .map_err(|_| CapabilityConsumerError::ContractUnavailable)?;
        let now_utc = current_utc()?;
        let response = match catalog_kind {
            CatalogKindV1::Capabilities => close_catalog_read(
                catalog_kind,
                self.witness
                    .read_capabilities(
                        request.page_size(),
                        request.cursor(),
                        &now_utc,
                        &self.limits,
                    )
                    .map_err(|_| CapabilityConsumerError::ContractUnavailable)?,
                CatalogReadResponseV1::capability_page,
                &self.limits,
            ),
            CatalogKindV1::Schemas => close_catalog_read(
                catalog_kind,
                self.witness
                    .read_schemas(
                        request.page_size(),
                        request.cursor(),
                        &now_utc,
                        &self.limits,
                    )
                    .map_err(|_| CapabilityConsumerError::ContractUnavailable)?,
                CatalogReadResponseV1::schema_page,
                &self.limits,
            ),
            CatalogKindV1::Profiles => close_catalog_read(
                catalog_kind,
                self.witness
                    .read_profiles(
                        request.page_size(),
                        request.cursor(),
                        &now_utc,
                        &self.limits,
                    )
                    .map_err(|_| CapabilityConsumerError::ContractUnavailable)?,
                CatalogReadResponseV1::profile_page,
                &self.limits,
            ),
        }?;
        readmit_catalog(response, &self.limits)
    }

    pub fn schema_bytes(
        &self,
        schema: &ExactBinding,
    ) -> Result<&'static [u8], CapabilityConsumerError> {
        self.witness
            .schema_bytes(schema)
            .ok_or(CapabilityConsumerError::SchemaUnavailable)
    }

    pub fn invoke(
        &self,
        operation: &ExactBinding,
        entry_fingerprint: &Fingerprint,
        transport: OperationTransport,
        request_id: Option<RequestId>,
        raw_body: &[u8],
    ) -> Result<InvocationResponseV1, CapabilityConsumerError> {
        let capability = self
            .witness
            .capabilities()
            .iter()
            .find(|entry| entry.operation().reference() == operation.reference())
            .ok_or(CapabilityConsumerError::CapabilityUnavailable)?;
        if capability.operation() != operation {
            return Err(CapabilityConsumerError::DefinitionBindingMismatch);
        }
        if capability.entry_fingerprint() != entry_fingerprint {
            return Err(CapabilityConsumerError::CapabilityEntryMismatch);
        }
        if !capability.declared_transports().contains(&transport)
            || !capability.current_transports().contains(&transport)
        {
            return Err(CapabilityConsumerError::TransportUnavailable);
        }
        let body = parse_structured_json::<ContractValue>(raw_body, &self.limits)
            .map_err(|_| CapabilityConsumerError::InvalidInput)?;
        let request = super::invocation_dispatch::operation_invocation_request(
            operation,
            request_id,
            body,
            &self.limits,
        )
        .map_err(|error| match error {
            ContractError::InvalidJson
            | ContractError::InvalidString
            | ContractError::InvalidNumber
            | ContractError::NonCanonicalOrder
            | ContractError::DuplicateMember
            | ContractError::BoundExceeded
            | ContractError::InvalidInvariant => CapabilityConsumerError::InvalidInput,
            _ => CapabilityConsumerError::ContractUnavailable,
        })?;
        let response = super::invocation_dispatch::dispatch_invocation(
            &self.repo_root,
            &request,
            &self.limits,
        )
        .map_err(|_| CapabilityConsumerError::ContractUnavailable)?;
        super::invocation_dispatch::readmit_invocation_response(&request, &response, &self.limits)
            .map_err(|_| CapabilityConsumerError::ContractUnavailable)
    }
}

fn current_utc() -> Result<UtcTimestamp, CapabilityConsumerError> {
    let value = OffsetDateTime::now_utc()
        .replace_nanosecond(0)
        .map_err(|_| CapabilityConsumerError::ClockUnavailable)?
        .format(&Rfc3339)
        .map_err(|_| CapabilityConsumerError::ClockUnavailable)?;
    UtcTimestamp::parse(&value).map_err(|_| CapabilityConsumerError::ClockUnavailable)
}

fn close_catalog_read<T>(
    kind: CatalogKindV1,
    read: ProductionCatalogRead<T>,
    page: fn(T, &ContractLimits) -> Result<CatalogReadResponseV1, ContractError>,
    limits: &ContractLimits,
) -> Result<CatalogReadResponseV1, CapabilityConsumerError> {
    match read {
        ProductionCatalogRead::Page(value) => page(value, limits),
        ProductionCatalogRead::CursorMismatch => {
            CatalogReadResponseV1::cursor_mismatch(kind, limits)
        }
        ProductionCatalogRead::Blocked(value) => {
            CatalogReadResponseV1::blocked(kind, *value, limits)
        }
    }
    .map_err(|_| CapabilityConsumerError::ContractUnavailable)
}

fn readmit_bootstrap(
    response: BootstrapFacadeResponseV1,
    limits: &ContractLimits,
) -> Result<BootstrapFacadeResponseV1, CapabilityConsumerError> {
    let bytes = canonical_json_bytes(&response, limits)
        .map_err(|_| CapabilityConsumerError::ContractUnavailable)?;
    BootstrapFacadeResponseV1::parse_and_admit(&bytes, limits)
        .map_err(|_| CapabilityConsumerError::ContractUnavailable)
}

fn readmit_catalog(
    response: CatalogReadResponseV1,
    limits: &ContractLimits,
) -> Result<CatalogReadResponseV1, CapabilityConsumerError> {
    let bytes = canonical_json_bytes(&response, limits)
        .map_err(|_| CapabilityConsumerError::ContractUnavailable)?;
    CatalogReadResponseV1::parse_and_admit(&bytes, limits)
        .map_err(|_| CapabilityConsumerError::ContractUnavailable)
}
