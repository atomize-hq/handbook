use super::serialization::classify_contract_error_message;
use super::{
    canonical_fingerprint, canonical_json_bytes, BoundedString, BoundedVec, CatalogPageSize,
    ContractError, ContractLimits, ExactBinding, ExactRef, Fingerprint, OperationClass,
    OperationDefinition, OperationIdempotency, OperationOutcomeSchemas, OperationOwnerBinding,
    OperationTransport, OwnerAdmissionToken, RequestId, SafeU64, SchemaManifestEntry, SemVer,
    TypedCatalogEntry,
};
use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize};
use std::cmp::Ordering;
use std::collections::BTreeMap;

const API_VERSION: &str = "1.0.0";
const API_MAJOR: u64 = 1;
const DESCRIPTOR_CONTENT_REF: &str = "handbook.bootstrap-descriptor@1.0.0";

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub struct BootstrapCeilings {
    bootstrap_request_max_bytes: SafeU64,
    bootstrap_response_max_bytes: SafeU64,
    ordinary_request_max_bytes: SafeU64,
    ordinary_response_max_bytes: SafeU64,
    catalog_page_size_max: SafeU64,
    catalog_entry_max_bytes: SafeU64,
    catalog_total_entries_max: SafeU64,
    owner_versions_max_items: SafeU64,
    idempotency_key_max_bytes: SafeU64,
    bridge_recovery_record_max_bytes: SafeU64,
}

impl BootstrapCeilings {
    pub fn packet3() -> Self {
        Self {
            bootstrap_request_max_bytes: safe(1_048_576),
            bootstrap_response_max_bytes: safe(1_048_576),
            ordinary_request_max_bytes: safe(1_048_576),
            ordinary_response_max_bytes: safe(1_048_576),
            catalog_page_size_max: safe(128),
            catalog_entry_max_bytes: safe(65_536),
            catalog_total_entries_max: safe(128),
            owner_versions_max_items: safe(128),
            idempotency_key_max_bytes: safe(65_536),
            bridge_recovery_record_max_bytes: safe(1_048_576),
        }
    }

    pub fn bootstrap_request_max_bytes(&self) -> SafeU64 {
        self.bootstrap_request_max_bytes
    }

    pub fn bootstrap_response_max_bytes(&self) -> SafeU64 {
        self.bootstrap_response_max_bytes
    }

    pub fn ordinary_request_max_bytes(&self) -> SafeU64 {
        self.ordinary_request_max_bytes
    }

    pub fn ordinary_response_max_bytes(&self) -> SafeU64 {
        self.ordinary_response_max_bytes
    }

    pub fn catalog_page_size_max(&self) -> SafeU64 {
        self.catalog_page_size_max
    }

    pub fn catalog_entry_max_bytes(&self) -> SafeU64 {
        self.catalog_entry_max_bytes
    }

    pub fn catalog_total_entries_max(&self) -> SafeU64 {
        self.catalog_total_entries_max
    }

    pub fn owner_versions_max_items(&self) -> SafeU64 {
        self.owner_versions_max_items
    }

    pub fn idempotency_key_max_bytes(&self) -> SafeU64 {
        self.idempotency_key_max_bytes
    }

    pub fn bridge_recovery_record_max_bytes(&self) -> SafeU64 {
        self.bridge_recovery_record_max_bytes
    }

    pub fn validate_bootstrap_request_bytes(&self, bytes: usize) -> Result<(), ContractError> {
        validate_byte_ceiling(bytes, self.bootstrap_request_max_bytes)
    }

    pub fn validate_bootstrap_response_bytes(&self, bytes: usize) -> Result<(), ContractError> {
        validate_byte_ceiling(bytes, self.bootstrap_response_max_bytes)
    }
}

impl<'de> Deserialize<'de> for BootstrapCeilings {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Raw {
            bootstrap_request_max_bytes: SafeU64,
            bootstrap_response_max_bytes: SafeU64,
            ordinary_request_max_bytes: SafeU64,
            ordinary_response_max_bytes: SafeU64,
            catalog_page_size_max: SafeU64,
            catalog_entry_max_bytes: SafeU64,
            catalog_total_entries_max: SafeU64,
            owner_versions_max_items: SafeU64,
            idempotency_key_max_bytes: SafeU64,
            bridge_recovery_record_max_bytes: SafeU64,
        }
        let raw = Raw::deserialize(deserializer).map_err(sanitize_deserialize_error)?;
        let value = Self {
            bootstrap_request_max_bytes: raw.bootstrap_request_max_bytes,
            bootstrap_response_max_bytes: raw.bootstrap_response_max_bytes,
            ordinary_request_max_bytes: raw.ordinary_request_max_bytes,
            ordinary_response_max_bytes: raw.ordinary_response_max_bytes,
            catalog_page_size_max: raw.catalog_page_size_max,
            catalog_entry_max_bytes: raw.catalog_entry_max_bytes,
            catalog_total_entries_max: raw.catalog_total_entries_max,
            owner_versions_max_items: raw.owner_versions_max_items,
            idempotency_key_max_bytes: raw.idempotency_key_max_bytes,
            bridge_recovery_record_max_bytes: raw.bridge_recovery_record_max_bytes,
        };
        if value != Self::packet3() {
            return Err(D::Error::custom(ContractError::InvalidInvariant));
        }
        Ok(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct BootstrapSchemaBindings {
    descriptor_schema: ExactBinding,
    bootstrap_request: ExactBinding,
    bootstrap_response: ExactBinding,
    bootstrap_refusal: ExactBinding,
    bootstrap_refusal_details: ExactBinding,
    capability_entry: ExactBinding,
    owner_version_entry: ExactBinding,
    profile_catalog_entry: ExactBinding,
    catalog_roots: ExactBinding,
    catalog_root: ExactBinding,
    typed_catalog_cursor: ExactBinding,
    capability_catalog_page: ExactBinding,
    schema_catalog_page: ExactBinding,
    profile_catalog_page: ExactBinding,
    catalog_read_blocked: ExactBinding,
    operation_definition: ExactBinding,
    operation_request: ExactBinding,
    operation_response: ExactBinding,
    operation_admission_refusal: ExactBinding,
}

impl BootstrapSchemaBindings {
    pub fn from_manifest(manifest: &[SchemaManifestEntry]) -> Result<Self, ContractError> {
        let mut bindings = BTreeMap::new();
        for entry in manifest {
            if bindings
                .insert(entry.schema().reference().as_str(), entry.schema().clone())
                .is_some()
            {
                return Err(ContractError::DuplicateMember);
            }
        }
        Self::from_map(&bindings)
    }

    fn from_map(bindings: &BTreeMap<&str, ExactBinding>) -> Result<Self, ContractError> {
        let get = |reference: &'static str| {
            bindings
                .get(reference)
                .cloned()
                .ok_or(ContractError::InvalidInvariant)
        };
        Ok(Self {
            descriptor_schema: get("handbook.bootstrap-descriptor-schema@1.0.0")?,
            bootstrap_request: get("handbook.bootstrap-request@1.0.0")?,
            bootstrap_response: get("handbook.bootstrap-response@1.0.0")?,
            bootstrap_refusal: get("handbook.bootstrap-refusal@1.0.0")?,
            bootstrap_refusal_details: get("handbook.bootstrap-refusal-details@1.0.0")?,
            capability_entry: get("handbook.capability-entry@1.0.0")?,
            owner_version_entry: get("handbook.owner-version-entry@1.0.0")?,
            profile_catalog_entry: get("handbook.profile-catalog-entry@1.0.0")?,
            catalog_roots: get("handbook.catalog-roots@1.0.0")?,
            catalog_root: get("handbook.catalog-root@1.0.0")?,
            typed_catalog_cursor: get("handbook.typed-catalog-cursor@1.0.0")?,
            capability_catalog_page: get("handbook.capability-catalog-page@1.0.0")?,
            schema_catalog_page: get("handbook.schema-catalog-page@1.0.0")?,
            profile_catalog_page: get("handbook.profile-catalog-page@1.0.0")?,
            catalog_read_blocked: get("handbook.catalog-read-blocked@1.0.0")?,
            operation_definition: get("handbook.operation-definition@1.0.0")?,
            operation_request: get("handbook.operation-request@1.0.0")?,
            operation_response: get("handbook.operation-response@1.0.0")?,
            operation_admission_refusal: get("handbook.operation-admission-refusal@1.0.0")?,
        })
    }

    pub fn descriptor_schema(&self) -> &ExactBinding {
        &self.descriptor_schema
    }

    pub fn bootstrap_request(&self) -> &ExactBinding {
        &self.bootstrap_request
    }

    pub fn bootstrap_response(&self) -> &ExactBinding {
        &self.bootstrap_response
    }

    pub fn bootstrap_refusal(&self) -> &ExactBinding {
        &self.bootstrap_refusal
    }

    pub fn bootstrap_refusal_details(&self) -> &ExactBinding {
        &self.bootstrap_refusal_details
    }

    pub fn capability_entry(&self) -> &ExactBinding {
        &self.capability_entry
    }

    pub fn owner_version_entry(&self) -> &ExactBinding {
        &self.owner_version_entry
    }

    pub fn profile_catalog_entry(&self) -> &ExactBinding {
        &self.profile_catalog_entry
    }

    pub fn catalog_roots(&self) -> &ExactBinding {
        &self.catalog_roots
    }

    pub fn catalog_root(&self) -> &ExactBinding {
        &self.catalog_root
    }

    pub fn typed_catalog_cursor(&self) -> &ExactBinding {
        &self.typed_catalog_cursor
    }

    pub fn capability_catalog_page(&self) -> &ExactBinding {
        &self.capability_catalog_page
    }

    pub fn schema_catalog_page(&self) -> &ExactBinding {
        &self.schema_catalog_page
    }

    pub fn profile_catalog_page(&self) -> &ExactBinding {
        &self.profile_catalog_page
    }

    pub fn catalog_read_blocked(&self) -> &ExactBinding {
        &self.catalog_read_blocked
    }

    pub fn operation_definition(&self) -> &ExactBinding {
        &self.operation_definition
    }

    pub fn operation_request(&self) -> &ExactBinding {
        &self.operation_request
    }

    pub fn operation_response(&self) -> &ExactBinding {
        &self.operation_response
    }

    pub fn operation_admission_refusal(&self) -> &ExactBinding {
        &self.operation_admission_refusal
    }
}

impl<'de> Deserialize<'de> for BootstrapSchemaBindings {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize, Serialize)]
        #[serde(deny_unknown_fields)]
        struct Raw {
            descriptor_schema: ExactBinding,
            bootstrap_request: ExactBinding,
            bootstrap_response: ExactBinding,
            bootstrap_refusal: ExactBinding,
            bootstrap_refusal_details: ExactBinding,
            capability_entry: ExactBinding,
            owner_version_entry: ExactBinding,
            profile_catalog_entry: ExactBinding,
            catalog_roots: ExactBinding,
            catalog_root: ExactBinding,
            typed_catalog_cursor: ExactBinding,
            capability_catalog_page: ExactBinding,
            schema_catalog_page: ExactBinding,
            profile_catalog_page: ExactBinding,
            catalog_read_blocked: ExactBinding,
            operation_definition: ExactBinding,
            operation_request: ExactBinding,
            operation_response: ExactBinding,
            operation_admission_refusal: ExactBinding,
        }
        let raw = Raw::deserialize(deserializer).map_err(sanitize_deserialize_error)?;
        let value =
            serde_json::to_value(&raw).map_err(|_| D::Error::custom(ContractError::InvalidJson))?;
        let object = value
            .as_object()
            .ok_or_else(|| D::Error::custom(ContractError::InvalidJson))?;
        let mut bindings = BTreeMap::new();
        for binding in object.values() {
            let binding: ExactBinding = serde_json::from_value(binding.clone())
                .map_err(|_| D::Error::custom(ContractError::InvalidJson))?;
            bindings.insert(binding.reference().as_str().to_owned(), binding);
        }
        let borrowed = bindings
            .iter()
            .map(|(reference, binding)| (reference.as_str(), binding.clone()))
            .collect::<BTreeMap<_, _>>();
        Self::from_map(&borrowed).map_err(D::Error::custom)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BootstrapRefusalReason {
    StaleDescriptor,
    TamperedDescriptor,
    UnsupportedApiVersion,
    UnsupportedApiMajor,
    CrossMajorDescriptor,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct BootstrapDescriptorV1 {
    schema: ExactBinding,
    descriptor: ExactRef,
    api_version: SemVer,
    api_major: SafeU64,
    supported_api_versions: BoundedVec<SemVer, 16>,
    schemas: BootstrapSchemaBindings,
    ceilings: BootstrapCeilings,
    declared_transports: BoundedVec<OperationTransport, 3>,
    descriptor_content_fingerprint: Fingerprint,
}

impl BootstrapDescriptorV1 {
    pub fn new(
        schemas: BootstrapSchemaBindings,
        supported_api_versions: BoundedVec<SemVer, 16>,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        validate_supported_versions(supported_api_versions.as_slice())?;
        let schema = schemas.descriptor_schema().clone();
        let descriptor = ExactRef::parse(DESCRIPTOR_CONTENT_REF)?;
        let api_version = SemVer::parse(API_VERSION)?;
        let api_major = SafeU64::new(API_MAJOR)?;
        let ceilings = BootstrapCeilings::packet3();
        let declared_transports = BoundedVec::new(vec![
            OperationTransport::CliJson,
            OperationTransport::RustSdk,
            OperationTransport::Tauri,
        ])?;
        let descriptor_content_fingerprint = canonical_fingerprint(
            &DescriptorPreimage {
                schema: &schema,
                descriptor: &descriptor,
                api_version: &api_version,
                api_major,
                supported_api_versions: &supported_api_versions,
                schemas: &schemas,
                ceilings,
                declared_transports: &declared_transports,
            },
            limits,
        )?;
        if descriptor_content_fingerprint == *schema.fingerprint() {
            return Err(ContractError::InvalidInvariant);
        }
        let value = Self {
            schema,
            descriptor,
            api_version,
            api_major,
            supported_api_versions,
            schemas,
            ceilings,
            declared_transports,
            descriptor_content_fingerprint,
        };
        canonical_json_bytes(&value, limits)?;
        Ok(value)
    }

    pub fn schema(&self) -> &ExactBinding {
        &self.schema
    }

    pub fn descriptor(&self) -> &ExactRef {
        &self.descriptor
    }

    pub fn api_version(&self) -> &SemVer {
        &self.api_version
    }

    pub fn api_major(&self) -> SafeU64 {
        self.api_major
    }

    pub fn supported_api_versions(&self) -> &[SemVer] {
        self.supported_api_versions.as_slice()
    }

    pub fn schemas(&self) -> &BootstrapSchemaBindings {
        &self.schemas
    }

    pub fn ceilings(&self) -> &BootstrapCeilings {
        &self.ceilings
    }

    pub fn declared_transports(&self) -> &[OperationTransport] {
        self.declared_transports.as_slice()
    }

    pub fn descriptor_content_fingerprint(&self) -> &Fingerprint {
        &self.descriptor_content_fingerprint
    }

    pub fn binding(&self) -> ExactBinding {
        ExactBinding::new(
            self.descriptor.clone(),
            self.descriptor_content_fingerprint.clone(),
        )
    }

    pub fn negotiate(&self, request: &BootstrapRequestV1) -> Result<(), BootstrapRefusalReason> {
        if request.bootstrap_descriptor().reference().identity() != self.descriptor.identity()
            || request.bootstrap_descriptor().reference().version().major()
                != self.descriptor.version().major()
        {
            return Err(BootstrapRefusalReason::CrossMajorDescriptor);
        }
        if request.bootstrap_descriptor().reference() != &self.descriptor {
            return Err(BootstrapRefusalReason::StaleDescriptor);
        }
        if request.bootstrap_descriptor().fingerprint() != &self.descriptor_content_fingerprint {
            return Err(BootstrapRefusalReason::TamperedDescriptor);
        }
        if request.api_version().major() != self.api_major.get() {
            return Err(BootstrapRefusalReason::UnsupportedApiMajor);
        }
        if !self
            .supported_api_versions
            .as_slice()
            .iter()
            .any(|version| version == request.api_version())
        {
            return Err(BootstrapRefusalReason::UnsupportedApiVersion);
        }
        Ok(())
    }
}

impl<'de> Deserialize<'de> for BootstrapDescriptorV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Raw {
            schema: ExactBinding,
            descriptor: ExactRef,
            api_version: SemVer,
            api_major: SafeU64,
            supported_api_versions: BoundedVec<SemVer, 16>,
            schemas: BootstrapSchemaBindings,
            ceilings: BootstrapCeilings,
            declared_transports: BoundedVec<OperationTransport, 3>,
            descriptor_content_fingerprint: Fingerprint,
        }
        let raw = Raw::deserialize(deserializer).map_err(sanitize_deserialize_error)?;
        let value = Self::new(
            raw.schemas,
            raw.supported_api_versions,
            &ContractLimits::packet0(),
        )
        .map_err(D::Error::custom)?;
        if raw.schema != value.schema
            || raw.descriptor != value.descriptor
            || raw.api_version != value.api_version
            || raw.api_major != value.api_major
            || raw.ceilings != value.ceilings
            || raw.declared_transports != value.declared_transports
            || raw.descriptor_content_fingerprint != value.descriptor_content_fingerprint
        {
            return Err(D::Error::custom(ContractError::InvalidInvariant));
        }
        Ok(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct BootstrapRequestV1 {
    schema: ExactBinding,
    request_id: Option<RequestId>,
    bootstrap_descriptor: ExactBinding,
    api_version: SemVer,
    page_size: CatalogPageSize,
    request_fingerprint: Fingerprint,
}

impl BootstrapRequestV1 {
    pub fn new(
        schema: ExactBinding,
        request_id: Option<RequestId>,
        bootstrap_descriptor: ExactBinding,
        api_version: SemVer,
        page_size: CatalogPageSize,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        require_binding_ref(&schema, "handbook.bootstrap-request@1.0.0")?;
        let request_fingerprint = canonical_fingerprint(
            &RequestPreimage {
                schema: &schema,
                bootstrap_descriptor: &bootstrap_descriptor,
                api_version: &api_version,
                page_size,
            },
            limits,
        )?;
        let value = Self {
            schema,
            request_id,
            bootstrap_descriptor,
            api_version,
            page_size,
            request_fingerprint,
        };
        canonical_json_bytes(&value, limits)?;
        Ok(value)
    }

    pub fn schema(&self) -> &ExactBinding {
        &self.schema
    }

    pub fn request_id(&self) -> Option<&RequestId> {
        self.request_id.as_ref()
    }

    pub fn bootstrap_descriptor(&self) -> &ExactBinding {
        &self.bootstrap_descriptor
    }

    pub fn api_version(&self) -> &SemVer {
        &self.api_version
    }

    pub fn page_size(&self) -> CatalogPageSize {
        self.page_size
    }

    pub fn request_fingerprint(&self) -> &Fingerprint {
        &self.request_fingerprint
    }
}

impl<'de> Deserialize<'de> for BootstrapRequestV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Raw {
            schema: ExactBinding,
            #[serde(default)]
            request_id: RequiredOption<RequestId>,
            bootstrap_descriptor: ExactBinding,
            api_version: SemVer,
            page_size: CatalogPageSize,
            request_fingerprint: Fingerprint,
        }
        let raw = Raw::deserialize(deserializer).map_err(sanitize_deserialize_error)?;
        let value = Self::new(
            raw.schema,
            raw.request_id.into_option().map_err(D::Error::custom)?,
            raw.bootstrap_descriptor,
            raw.api_version,
            raw.page_size,
            &ContractLimits::packet0(),
        )
        .map_err(D::Error::custom)?;
        if raw.request_fingerprint != value.request_fingerprint {
            return Err(D::Error::custom(ContractError::FingerprintMismatch));
        }
        Ok(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct OwnerVersionEntry {
    owner_crate: BoundedString<255>,
    owner_version: SemVer,
    source: ExactBinding,
    entry_fingerprint: Fingerprint,
}

impl OwnerVersionEntry {
    pub fn new(
        owner_crate: BoundedString<255>,
        owner_version: SemVer,
        source: ExactBinding,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        validate_token(owner_crate.as_str())?;
        if source.reference().version() != &owner_version {
            return Err(ContractError::InvalidInvariant);
        }
        let entry_fingerprint = canonical_fingerprint(
            &OwnerVersionPreimage {
                owner_crate: &owner_crate,
                owner_version: &owner_version,
                source: &source,
            },
            limits,
        )?;
        let value = Self {
            owner_crate,
            owner_version,
            source,
            entry_fingerprint,
        };
        canonical_json_bytes(&value, limits)?;
        Ok(value)
    }

    pub fn owner_crate(&self) -> &str {
        self.owner_crate.as_str()
    }

    pub fn owner_version(&self) -> &SemVer {
        &self.owner_version
    }

    pub fn source(&self) -> &ExactBinding {
        &self.source
    }

    pub fn entry_fingerprint(&self) -> &Fingerprint {
        &self.entry_fingerprint
    }
}

impl<'de> Deserialize<'de> for OwnerVersionEntry {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Raw {
            owner_crate: BoundedString<255>,
            owner_version: SemVer,
            source: ExactBinding,
            entry_fingerprint: Fingerprint,
        }
        let raw = Raw::deserialize(deserializer).map_err(sanitize_deserialize_error)?;
        let value = Self::new(
            raw.owner_crate,
            raw.owner_version,
            raw.source,
            &ContractLimits::packet0(),
        )
        .map_err(D::Error::custom)?;
        if raw.entry_fingerprint != value.entry_fingerprint {
            return Err(D::Error::custom(ContractError::FingerprintMismatch));
        }
        Ok(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ProfileCatalogEntry {
    profile: ExactBinding,
    source: ExactBinding,
    entry_fingerprint: Fingerprint,
}

impl ProfileCatalogEntry {
    pub fn new(
        profile: ExactBinding,
        source: ExactBinding,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        if !profile
            .reference()
            .identity()
            .starts_with("handbook.profile.")
            || !source
                .reference()
                .identity()
                .starts_with("handbook.profile-source.")
            || profile.reference().version() != source.reference().version()
            || profile.fingerprint() == source.fingerprint()
        {
            return Err(ContractError::InvalidInvariant);
        }
        let entry_fingerprint = canonical_fingerprint(
            &ProfilePreimage {
                profile: &profile,
                source: &source,
            },
            limits,
        )?;
        let value = Self {
            profile,
            source,
            entry_fingerprint,
        };
        canonical_json_bytes(&value, limits)?;
        Ok(value)
    }

    pub fn profile(&self) -> &ExactBinding {
        &self.profile
    }

    pub fn source(&self) -> &ExactBinding {
        &self.source
    }

    pub fn entry_fingerprint(&self) -> &Fingerprint {
        &self.entry_fingerprint
    }
}

impl TypedCatalogEntry for ProfileCatalogEntry {
    const CATALOG_REF: &'static str = "handbook.catalog.profiles@1.0.0";
    const ENTRY_SCHEMA_REF: &'static str = "handbook.profile-catalog-entry@1.0.0";
    const PAGE_SCHEMA_REF: &'static str = "handbook.profile-catalog-page@1.0.0";

    fn catalog_sort_key(&self) -> &ExactRef {
        self.profile.reference()
    }
}

impl<'de> Deserialize<'de> for ProfileCatalogEntry {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Raw {
            profile: ExactBinding,
            source: ExactBinding,
            entry_fingerprint: Fingerprint,
        }
        let raw = Raw::deserialize(deserializer).map_err(sanitize_deserialize_error)?;
        let value = Self::new(raw.profile, raw.source, &ContractLimits::packet0())
            .map_err(D::Error::custom)?;
        if raw.entry_fingerprint != value.entry_fingerprint {
            return Err(D::Error::custom(ContractError::FingerprintMismatch));
        }
        Ok(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CapabilityEntry {
    operation: ExactBinding,
    owner: OperationOwnerBinding,
    request_schema: ExactBinding,
    result_schema: ExactBinding,
    outcome_schemas: OperationOutcomeSchemas,
    required_capabilities: BoundedVec<ExactBinding, 128>,
    class: OperationClass,
    idempotency: OperationIdempotency,
    declared_transports: BoundedVec<OperationTransport, 3>,
    current_transports: BoundedVec<OperationTransport, 3>,
    deprecation: Option<ExactBinding>,
    entry_fingerprint: Fingerprint,
}

#[doc(hidden)]
pub(crate) trait CurrentTransportEvidenceSealed {}

/// A sealed proof that one owner-admitted definition has a current transport.
///
/// Packet 3 deliberately supplied no production implementation. Packet 4 adds
/// the single owner-admitted posture Rust implementation inside this crate;
/// declared transport targets still cannot be promoted into current capability
/// truth by a downstream crate:
///
/// ```compile_fail
/// use handbook_sdk::contract_membrane::{
///     CurrentTransportEvidence, OperationTransport, OwnerAdmissionToken,
/// };
///
/// struct DeclaredIsCurrent;
/// impl CurrentTransportEvidence for DeclaredIsCurrent {
///     fn owner_admission(&self) -> &OwnerAdmissionToken { unimplemented!() }
///     fn current_transports(&self) -> &[OperationTransport] { &[] }
/// }
/// ```
#[allow(private_bounds)]
pub trait CurrentTransportEvidence: CurrentTransportEvidenceSealed {
    fn owner_admission(&self) -> &OwnerAdmissionToken;
    fn current_transports(&self) -> &[OperationTransport];
}

impl CapabilityEntry {
    pub fn new(
        definition: &OperationDefinition,
        evidence: &dyn CurrentTransportEvidence,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        let owner_admission = evidence.owner_admission();
        if owner_admission.operation_ref() != definition.operation().as_str()
            || owner_admission.definition_fingerprint()
                != definition.definition_fingerprint().as_str()
        {
            return Err(ContractError::InvalidInvariant);
        }
        Self::structurally_admitted(
            definition,
            BoundedVec::new(evidence.current_transports().to_vec())?,
            limits,
        )
    }

    pub(super) fn structurally_admitted(
        definition: &OperationDefinition,
        current_transports: BoundedVec<OperationTransport, 3>,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        Self::from_parts(
            definition.binding(),
            definition.owner().clone(),
            definition.request_schema().clone(),
            definition.result_schema().clone(),
            definition.outcome_schemas().clone(),
            BoundedVec::new(definition.required_capabilities().to_vec())?,
            definition.class(),
            definition.idempotency().clone(),
            BoundedVec::new(definition.declared_transports().to_vec())?,
            current_transports,
            definition.deprecation().cloned(),
            limits,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn from_parts(
        operation: ExactBinding,
        owner: OperationOwnerBinding,
        request_schema: ExactBinding,
        result_schema: ExactBinding,
        outcome_schemas: OperationOutcomeSchemas,
        required_capabilities: BoundedVec<ExactBinding, 128>,
        class: OperationClass,
        idempotency: OperationIdempotency,
        declared_transports: BoundedVec<OperationTransport, 3>,
        current_transports: BoundedVec<OperationTransport, 3>,
        deprecation: Option<ExactBinding>,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        validate_capability_transports(
            declared_transports.as_slice(),
            current_transports.as_slice(),
        )?;
        validate_operation_schema_ref(operation.reference(), &request_schema, "request")?;
        validate_operation_schema_ref(operation.reference(), &result_schema, "result")?;
        let entry_fingerprint = canonical_fingerprint(
            &CapabilityPreimage {
                operation: &operation,
                owner: &owner,
                request_schema: &request_schema,
                result_schema: &result_schema,
                outcome_schemas: &outcome_schemas,
                required_capabilities: &required_capabilities,
                class,
                idempotency: &idempotency,
                declared_transports: &declared_transports,
                current_transports: &current_transports,
                deprecation: deprecation.as_ref(),
            },
            limits,
        )?;
        let value = Self {
            operation,
            owner,
            request_schema,
            result_schema,
            outcome_schemas,
            required_capabilities,
            class,
            idempotency,
            declared_transports,
            current_transports,
            deprecation,
            entry_fingerprint,
        };
        canonical_json_bytes(&value, limits)?;
        Ok(value)
    }

    pub fn operation(&self) -> &ExactBinding {
        &self.operation
    }

    pub fn owner(&self) -> &OperationOwnerBinding {
        &self.owner
    }

    pub fn request_schema(&self) -> &ExactBinding {
        &self.request_schema
    }

    pub fn result_schema(&self) -> &ExactBinding {
        &self.result_schema
    }

    pub fn outcome_schemas(&self) -> &OperationOutcomeSchemas {
        &self.outcome_schemas
    }

    pub fn required_capabilities(&self) -> &[ExactBinding] {
        self.required_capabilities.as_slice()
    }

    pub fn class(&self) -> OperationClass {
        self.class
    }

    pub fn idempotency(&self) -> &OperationIdempotency {
        &self.idempotency
    }

    pub fn declared_transports(&self) -> &[OperationTransport] {
        self.declared_transports.as_slice()
    }

    pub fn current_transports(&self) -> &[OperationTransport] {
        self.current_transports.as_slice()
    }

    pub fn deprecation(&self) -> Option<&ExactBinding> {
        self.deprecation.as_ref()
    }

    pub fn entry_fingerprint(&self) -> &Fingerprint {
        &self.entry_fingerprint
    }
}

impl TypedCatalogEntry for CapabilityEntry {
    const CATALOG_REF: &'static str = "handbook.catalog.operations@1.0.0";
    const ENTRY_SCHEMA_REF: &'static str = "handbook.capability-entry@1.0.0";
    const PAGE_SCHEMA_REF: &'static str = "handbook.capability-catalog-page@1.0.0";

    fn catalog_sort_key(&self) -> &ExactRef {
        self.operation.reference()
    }
}

impl<'de> Deserialize<'de> for CapabilityEntry {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Raw {
            operation: ExactBinding,
            owner: OperationOwnerBinding,
            request_schema: ExactBinding,
            result_schema: ExactBinding,
            outcome_schemas: OperationOutcomeSchemas,
            required_capabilities: BoundedVec<ExactBinding, 128>,
            class: OperationClass,
            idempotency: OperationIdempotency,
            declared_transports: BoundedVec<OperationTransport, 3>,
            current_transports: BoundedVec<OperationTransport, 3>,
            #[serde(default)]
            deprecation: RequiredOption<ExactBinding>,
            entry_fingerprint: Fingerprint,
        }
        let raw = Raw::deserialize(deserializer).map_err(sanitize_deserialize_error)?;
        let value = Self::from_parts(
            raw.operation,
            raw.owner,
            raw.request_schema,
            raw.result_schema,
            raw.outcome_schemas,
            raw.required_capabilities,
            raw.class,
            raw.idempotency,
            raw.declared_transports,
            raw.current_transports,
            raw.deprecation.into_option().map_err(D::Error::custom)?,
            &ContractLimits::packet0(),
        )
        .map_err(D::Error::custom)?;
        if raw.entry_fingerprint != value.entry_fingerprint {
            return Err(D::Error::custom(ContractError::FingerprintMismatch));
        }
        Ok(value)
    }
}

#[derive(Serialize)]
struct DescriptorPreimage<'a> {
    schema: &'a ExactBinding,
    descriptor: &'a ExactRef,
    api_version: &'a SemVer,
    api_major: SafeU64,
    supported_api_versions: &'a BoundedVec<SemVer, 16>,
    schemas: &'a BootstrapSchemaBindings,
    ceilings: BootstrapCeilings,
    declared_transports: &'a BoundedVec<OperationTransport, 3>,
}

#[derive(Serialize)]
struct RequestPreimage<'a> {
    schema: &'a ExactBinding,
    bootstrap_descriptor: &'a ExactBinding,
    api_version: &'a SemVer,
    page_size: CatalogPageSize,
}

#[derive(Serialize)]
struct OwnerVersionPreimage<'a> {
    owner_crate: &'a BoundedString<255>,
    owner_version: &'a SemVer,
    source: &'a ExactBinding,
}

#[derive(Serialize)]
struct ProfilePreimage<'a> {
    profile: &'a ExactBinding,
    source: &'a ExactBinding,
}

#[derive(Serialize)]
struct CapabilityPreimage<'a> {
    operation: &'a ExactBinding,
    owner: &'a OperationOwnerBinding,
    request_schema: &'a ExactBinding,
    result_schema: &'a ExactBinding,
    outcome_schemas: &'a OperationOutcomeSchemas,
    required_capabilities: &'a BoundedVec<ExactBinding, 128>,
    class: OperationClass,
    idempotency: &'a OperationIdempotency,
    declared_transports: &'a BoundedVec<OperationTransport, 3>,
    current_transports: &'a BoundedVec<OperationTransport, 3>,
    deprecation: Option<&'a ExactBinding>,
}

fn validate_supported_versions(versions: &[SemVer]) -> Result<(), ContractError> {
    if versions.is_empty() {
        return Err(ContractError::Empty);
    }
    let mut found_current = false;
    for (index, version) in versions.iter().enumerate() {
        if version.major() != API_MAJOR
            || version.prerelease().is_some()
            || version.build().is_some()
        {
            return Err(ContractError::InvalidInvariant);
        }
        found_current |= version.as_str() == API_VERSION;
        if index > 0 {
            let previous = &versions[index - 1];
            if (previous.minor(), previous.patch()) >= (version.minor(), version.patch()) {
                return Err(if previous == version {
                    ContractError::DuplicateMember
                } else {
                    ContractError::NonCanonicalOrder
                });
            }
        }
    }
    if found_current {
        Ok(())
    } else {
        Err(ContractError::InvalidInvariant)
    }
}

fn validate_capability_transports(
    declared: &[OperationTransport],
    current: &[OperationTransport],
) -> Result<(), ContractError> {
    if current.is_empty() {
        return Err(ContractError::Empty);
    }
    for pair in current.windows(2) {
        match pair[0].cmp(&pair[1]) {
            Ordering::Less => {}
            Ordering::Equal => return Err(ContractError::DuplicateMember),
            Ordering::Greater => return Err(ContractError::NonCanonicalOrder),
        }
    }
    if current.iter().all(|transport| declared.contains(transport)) {
        Ok(())
    } else {
        Err(ContractError::InvalidInvariant)
    }
}

fn validate_operation_schema_ref(
    operation: &ExactRef,
    schema: &ExactBinding,
    suffix: &str,
) -> Result<(), ContractError> {
    let stem = operation.identity().replace('.', "-");
    let expected = format!(
        "handbook.operation.{stem}-{suffix}@{}",
        operation.version().as_str()
    );
    if schema.reference().as_str() == expected {
        Ok(())
    } else {
        Err(ContractError::InvalidInvariant)
    }
}

fn validate_byte_ceiling(bytes: usize, ceiling: SafeU64) -> Result<(), ContractError> {
    if bytes as u64 <= ceiling.get() {
        Ok(())
    } else {
        Err(ContractError::BoundExceeded)
    }
}

fn require_binding_ref(binding: &ExactBinding, expected: &str) -> Result<(), ContractError> {
    if binding.reference().as_str() == expected {
        Ok(())
    } else {
        Err(ContractError::InvalidInvariant)
    }
}

fn validate_token(value: &str) -> Result<(), ContractError> {
    let bytes = value.as_bytes();
    if bytes.first().is_some_and(u8::is_ascii_lowercase)
        && bytes[1..].iter().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'.' | b'_' | b'-')
        })
    {
        Ok(())
    } else {
        Err(ContractError::InvalidString)
    }
}

fn safe(value: u64) -> SafeU64 {
    SafeU64::new(value).expect("Packet-3 ceilings fit the safe integer domain")
}

fn sanitize_deserialize_error<E: serde::de::Error>(error: E) -> E {
    E::custom(classify_contract_error_message(&error.to_string()))
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
        Option::deserialize(deserializer).map(Self::Present)
    }
}
