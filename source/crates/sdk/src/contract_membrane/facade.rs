mod request;
mod response;

pub(crate) use request::SdkUseCaseRequest;

pub use request::{
    ApproverAdministrationRequestV1, ApproverMappingRequestV1, AuthorCharterRequestV1,
    AuthorProjectContextRequestV1, CharterAcquisitionModeV1, CharterCoverageSubmissionV1,
    CharterInputEntryV1, CharterInputValueV1, CharterIntakeConsumerV1, CharterIntakeEnvelopeV1,
    CharterIntakeSourceKindV1, InvocationRequestBodyV1, InvocationRequestKindV1,
    InvocationRequestV1, PipelineRouteResolveAndPersistRequestV1, ProjectContextInputV1,
    RepositoryDoctorRequestV1, RepositorySetupRequestV1, SessionGroundingBoundaryKindV1,
    SessionGroundingBoundaryRequestV1, UseCaseRequestBodyV1, UseCaseRequestV1,
};
pub use response::{
    BootstrapFacadeOutcomeKindV1, BootstrapFacadeResponseBodyV1, BootstrapFacadeResponseV1,
    CatalogKindV1, CatalogReadOutcomeKindV1, CatalogReadRequestV1, CatalogReadResponseBodyV1,
    CatalogReadResponseV1,
};

use super::serialization::classify_contract_error_message;
use super::{
    canonical_json_bytes, parse_canonical_json, sha256_fingerprint, ContractError, ContractLimits,
    ExactBinding, ExactRef,
};
use serde::{Deserialize, Deserializer};
use serde_json::Value;

pub(crate) const INVOCATION_REQUEST_REF: &str = "handbook.invocation-request@1.0.0";
pub(crate) const USE_CASE_REQUEST_REF: &str = "handbook.use-case-request@1.0.0";
pub(crate) const BOOTSTRAP_FACADE_RESPONSE_REF: &str = "handbook.bootstrap-facade-response@1.0.0";
pub(crate) const CATALOG_READ_REQUEST_REF: &str = "handbook.catalog-read-request@1.0.0";
pub(crate) const CATALOG_READ_RESPONSE_REF: &str = "handbook.catalog-read-response@1.0.0";

fn schema_binding(reference: &str) -> Result<ExactBinding, ContractError> {
    super::production_witness::current_schema_binding(reference)
}

fn identity_binding(reference: &str) -> Result<ExactBinding, ContractError> {
    Ok(ExactBinding::new(
        ExactRef::parse(reference)?,
        sha256_fingerprint(reference.as_bytes()),
    ))
}

fn require_schema_binding(value: &ExactBinding, reference: &str) -> Result<(), ContractError> {
    if value != &schema_binding(reference)? {
        return Err(ContractError::InvalidInvariant);
    }
    Ok(())
}

fn parse_contract_value<T>(value: &Value, limits: &ContractLimits) -> Result<T, ContractError>
where
    T: for<'de> Deserialize<'de>,
{
    serde_json::from_slice(&canonical_json_bytes(value, limits)?)
        .map_err(|error| classify_contract_error_message(&error.to_string()))
}

fn parse_canonical_wire(bytes: &[u8], limits: &ContractLimits) -> Result<Value, ContractError> {
    parse_canonical_json(bytes, limits)
}

enum RequiredOption<T> {
    Missing,
    Present(Option<T>),
}

impl<T> RequiredOption<T> {
    fn into_option(self) -> Result<Option<T>, ContractError> {
        match self {
            Self::Missing => Err(ContractError::InvalidJson),
            Self::Present(value) => Ok(value),
        }
    }
}

impl<T> Default for RequiredOption<T> {
    fn default() -> Self {
        Self::Missing
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for RequiredOption<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Option::<T>::deserialize(deserializer).map(Self::Present)
    }
}
