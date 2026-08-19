use super::serialization::classify_contract_error_message;
use super::{
    canonical_fingerprint, canonical_json_bytes, parse_canonical_json, BootstrapDescriptorV1,
    BootstrapRefusalReason, BootstrapRequestV1, BoundedString, BoundedVec, CapabilityEntry,
    CatalogRoot, ContractError, ContractLimits, ContractValue, ExactBinding, Fingerprint, Problem,
    ProblemCategory, ProfileCatalogEntry, RecheckCondition, RetryDirective, SafeU64,
    SchemaManifestEntry, SemVer, TerminalStatus, TypedCatalogPage,
};
use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize};

pub type CapabilityCatalogPage = TypedCatalogPage<CapabilityEntry>;
pub type SchemaCatalogPage = TypedCatalogPage<SchemaManifestEntry>;
pub type ProfileCatalogPage = TypedCatalogPage<ProfileCatalogEntry>;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CatalogRoots {
    schema: ExactBinding,
    operations: CatalogRoot,
    schemas: CatalogRoot,
    profiles: CatalogRoot,
    roots_fingerprint: Fingerprint,
}

impl CatalogRoots {
    pub fn new(
        schema: ExactBinding,
        operations: CatalogRoot,
        schemas: CatalogRoot,
        profiles: CatalogRoot,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        require_ref(&schema, "handbook.catalog-roots@1.0.0")?;
        require_ref(operations.catalog(), "handbook.catalog.operations@1.0.0")?;
        require_ref(schemas.catalog(), "handbook.catalog.schemas@1.0.0")?;
        require_ref(profiles.catalog(), "handbook.catalog.profiles@1.0.0")?;
        let roots_fingerprint = canonical_fingerprint(
            &CatalogRootsPreimage {
                schema: &schema,
                operations: &operations,
                schemas: &schemas,
                profiles: &profiles,
            },
            limits,
        )?;
        let value = Self {
            schema,
            operations,
            schemas,
            profiles,
            roots_fingerprint,
        };
        canonical_json_bytes(&value, limits)?;
        Ok(value)
    }

    pub fn schema(&self) -> &ExactBinding {
        &self.schema
    }

    pub fn operations(&self) -> &CatalogRoot {
        &self.operations
    }

    pub fn schemas(&self) -> &CatalogRoot {
        &self.schemas
    }

    pub fn profiles(&self) -> &CatalogRoot {
        &self.profiles
    }

    pub fn roots_fingerprint(&self) -> &Fingerprint {
        &self.roots_fingerprint
    }
}

impl<'de> Deserialize<'de> for CatalogRoots {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Raw {
            schema: ExactBinding,
            operations: CatalogRoot,
            schemas: CatalogRoot,
            profiles: CatalogRoot,
            roots_fingerprint: Fingerprint,
        }
        let raw = Raw::deserialize(deserializer).map_err(sanitize_deserialize_error)?;
        let value = Self::new(
            raw.schema,
            raw.operations,
            raw.schemas,
            raw.profiles,
            &ContractLimits::packet0(),
        )
        .map_err(D::Error::custom)?;
        if raw.roots_fingerprint != value.roots_fingerprint {
            return Err(D::Error::custom(ContractError::FingerprintMismatch));
        }
        Ok(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct BootstrapResponseV1 {
    schema: ExactBinding,
    request_id: Option<super::RequestId>,
    request_fingerprint: Fingerprint,
    status: TerminalStatus,
    descriptor: BootstrapDescriptorV1,
    catalog_roots: CatalogRoots,
    first_capability_page: CapabilityCatalogPage,
    owner_versions: BoundedVec<super::OwnerVersionEntry, 128>,
    schema_manifest: BoundedVec<SchemaManifestEntry, 256>,
    response_fingerprint: Fingerprint,
}

impl BootstrapResponseV1 {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        schema: ExactBinding,
        request: &BootstrapRequestV1,
        descriptor: BootstrapDescriptorV1,
        catalog_roots: CatalogRoots,
        first_capability_page: CapabilityCatalogPage,
        owner_versions: BoundedVec<super::OwnerVersionEntry, 128>,
        schema_manifest: BoundedVec<SchemaManifestEntry, 256>,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        require_ref(&schema, "handbook.bootstrap-response@1.0.0")?;
        descriptor
            .negotiate(request)
            .map_err(|_| ContractError::InvalidInvariant)?;
        if first_capability_page.root() != catalog_roots.operations()
            || first_capability_page.start_index().get() != 0
            || first_capability_page.page_size() != request.page_size()
        {
            return Err(ContractError::InvalidInvariant);
        }
        validate_owner_order(owner_versions.as_slice())?;
        validate_manifest_order(schema_manifest.as_slice())?;
        let status = TerminalStatus::Ok;
        let response_fingerprint = canonical_fingerprint(
            &ResponsePreimage {
                schema: &schema,
                request_id: request.request_id(),
                request_fingerprint: request.request_fingerprint(),
                status,
                descriptor: &descriptor,
                catalog_roots: &catalog_roots,
                first_capability_page: &first_capability_page,
                owner_versions: &owner_versions,
                schema_manifest: &schema_manifest,
            },
            limits,
        )?;
        let value = Self {
            schema,
            request_id: request.request_id().cloned(),
            request_fingerprint: request.request_fingerprint().clone(),
            status,
            descriptor,
            catalog_roots,
            first_capability_page,
            owner_versions,
            schema_manifest,
            response_fingerprint,
        };
        canonical_json_bytes(&value, limits)?;
        Ok(value)
    }

    pub fn status(&self) -> TerminalStatus {
        self.status
    }

    pub fn descriptor(&self) -> &BootstrapDescriptorV1 {
        &self.descriptor
    }

    pub fn catalog_roots(&self) -> &CatalogRoots {
        &self.catalog_roots
    }

    pub fn first_capability_page(&self) -> &CapabilityCatalogPage {
        &self.first_capability_page
    }

    pub fn owner_versions(&self) -> &[super::OwnerVersionEntry] {
        self.owner_versions.as_slice()
    }

    pub fn schema_manifest(&self) -> &[SchemaManifestEntry] {
        self.schema_manifest.as_slice()
    }

    pub fn response_fingerprint(&self) -> &Fingerprint {
        &self.response_fingerprint
    }
}

impl<'de> Deserialize<'de> for BootstrapResponseV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Raw {
            schema: ExactBinding,
            #[serde(default)]
            request_id: RequiredOption<super::RequestId>,
            request_fingerprint: Fingerprint,
            status: TerminalStatus,
            descriptor: BootstrapDescriptorV1,
            catalog_roots: CatalogRoots,
            first_capability_page: CapabilityCatalogPage,
            owner_versions: BoundedVec<super::OwnerVersionEntry, 128>,
            schema_manifest: BoundedVec<SchemaManifestEntry, 256>,
            response_fingerprint: Fingerprint,
        }
        let raw = Raw::deserialize(deserializer).map_err(sanitize_deserialize_error)?;
        let request_id = raw.request_id.into_option().map_err(D::Error::custom)?;
        if raw.status != TerminalStatus::Ok {
            return Err(D::Error::custom(ContractError::InvalidInvariant));
        }
        validate_owner_order(raw.owner_versions.as_slice()).map_err(D::Error::custom)?;
        validate_manifest_order(raw.schema_manifest.as_slice()).map_err(D::Error::custom)?;
        if raw.first_capability_page.root() != raw.catalog_roots.operations()
            || raw.first_capability_page.start_index().get() != 0
        {
            return Err(D::Error::custom(ContractError::InvalidInvariant));
        }
        let expected = canonical_fingerprint(
            &ResponsePreimage {
                schema: &raw.schema,
                request_id: request_id.as_ref(),
                request_fingerprint: &raw.request_fingerprint,
                status: raw.status,
                descriptor: &raw.descriptor,
                catalog_roots: &raw.catalog_roots,
                first_capability_page: &raw.first_capability_page,
                owner_versions: &raw.owner_versions,
                schema_manifest: &raw.schema_manifest,
            },
            &ContractLimits::packet0(),
        )
        .map_err(D::Error::custom)?;
        if raw.response_fingerprint != expected {
            return Err(D::Error::custom(ContractError::FingerprintMismatch));
        }
        Ok(Self {
            schema: raw.schema,
            request_id,
            request_fingerprint: raw.request_fingerprint,
            status: raw.status,
            descriptor: raw.descriptor,
            catalog_roots: raw.catalog_roots,
            first_capability_page: raw.first_capability_page,
            owner_versions: raw.owner_versions,
            schema_manifest: raw.schema_manifest,
            response_fingerprint: raw.response_fingerprint,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct BootstrapRefusalDetails {
    reason: BootstrapRefusalReason,
    supported_api_versions: BoundedVec<SemVer, 16>,
}

impl<'de> Deserialize<'de> for BootstrapRefusalDetails {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Raw {
            reason: BootstrapRefusalReason,
            supported_api_versions: BoundedVec<SemVer, 16>,
        }
        let raw = Raw::deserialize(deserializer).map_err(sanitize_deserialize_error)?;
        Self::new(raw.reason, raw.supported_api_versions).map_err(D::Error::custom)
    }
}

impl BootstrapRefusalDetails {
    pub fn new(
        reason: BootstrapRefusalReason,
        supported_api_versions: BoundedVec<SemVer, 16>,
    ) -> Result<Self, ContractError> {
        if supported_api_versions.as_slice().is_empty() {
            return Err(ContractError::Empty);
        }
        if supported_api_versions
            .as_slice()
            .iter()
            .any(|version| version.prerelease().is_some() || version.build().is_some())
        {
            return Err(ContractError::InvalidInvariant);
        }
        for pair in supported_api_versions.as_slice().windows(2) {
            let previous = (pair[0].major(), pair[0].minor(), pair[0].patch());
            let current = (pair[1].major(), pair[1].minor(), pair[1].patch());
            if previous >= current {
                return Err(if pair[0] == pair[1] {
                    ContractError::DuplicateMember
                } else {
                    ContractError::NonCanonicalOrder
                });
            }
        }
        Ok(Self {
            reason,
            supported_api_versions,
        })
    }

    pub fn reason(&self) -> BootstrapRefusalReason {
        self.reason
    }

    pub fn supported_api_versions(&self) -> &[SemVer] {
        self.supported_api_versions.as_slice()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct BootstrapRefusalV1 {
    schema: ExactBinding,
    request_id: Option<super::RequestId>,
    request_fingerprint: Fingerprint,
    bootstrap_descriptor: Option<ExactBinding>,
    status: TerminalStatus,
    data: Option<BootstrapRefusalDetails>,
    blockers: BoundedVec<Problem, 128>,
    refusals: BoundedVec<Problem, 128>,
    errors: BoundedVec<Problem, 128>,
    schema_manifest: BoundedVec<SchemaManifestEntry, 256>,
    response_fingerprint: Fingerprint,
}

impl BootstrapRefusalV1 {
    pub fn new(
        schema: ExactBinding,
        request: &BootstrapRequestV1,
        bootstrap_descriptor: Option<ExactBinding>,
        reason: BootstrapRefusalReason,
        supported_api_versions: BoundedVec<SemVer, 16>,
        schema_manifest: BoundedVec<SchemaManifestEntry, 256>,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        require_ref(&schema, "handbook.bootstrap-refusal@1.0.0")?;
        validate_manifest_order(schema_manifest.as_slice())?;
        let details_schema = manifest_binding(
            schema_manifest.as_slice(),
            "handbook.bootstrap-refusal-details@1.0.0",
        )?;
        let details = BootstrapRefusalDetails::new(reason, supported_api_versions)?;
        let details_value: ContractValue =
            parse_canonical_json(&canonical_json_bytes(&details, limits)?, limits)?;
        let refusal = Problem::new(
            BoundedString::new("bootstrap.negotiation.refused".to_owned())?,
            BoundedString::new("bootstrap_negotiation_refused".to_owned())?,
            ProblemCategory::Compatibility,
            request.bootstrap_descriptor().clone(),
            Some(details_schema.clone()),
            details_schema,
            details_value,
            BoundedVec::new(Vec::new())?,
            RetryDirective::AfterRequestChange,
            None,
            None,
            limits,
        )?;
        let status = TerminalStatus::Refused;
        let data = None;
        let blockers = BoundedVec::new(Vec::new())?;
        let refusals = BoundedVec::new(vec![refusal])?;
        let errors = BoundedVec::new(Vec::new())?;
        let response_fingerprint = canonical_fingerprint(
            &RefusalPreimage {
                schema: &schema,
                request_id: request.request_id(),
                request_fingerprint: request.request_fingerprint(),
                bootstrap_descriptor: bootstrap_descriptor.as_ref(),
                status,
                data: data.as_ref(),
                blockers: &blockers,
                refusals: &refusals,
                errors: &errors,
                schema_manifest: &schema_manifest,
            },
            limits,
        )?;
        let value = Self {
            schema,
            request_id: request.request_id().cloned(),
            request_fingerprint: request.request_fingerprint().clone(),
            bootstrap_descriptor,
            status,
            data,
            blockers,
            refusals,
            errors,
            schema_manifest,
            response_fingerprint,
        };
        canonical_json_bytes(&value, limits)?;
        Ok(value)
    }

    pub fn status(&self) -> TerminalStatus {
        self.status
    }

    pub fn data(&self) -> Option<&BootstrapRefusalDetails> {
        self.data.as_ref()
    }

    pub fn refusals(&self) -> &[Problem] {
        self.refusals.as_slice()
    }

    pub fn refusal_reason(&self) -> Option<BootstrapRefusalReason> {
        let problem = self.refusals.as_slice().first()?;
        let bytes = canonical_json_bytes(problem.details(), &ContractLimits::packet0()).ok()?;
        parse_canonical_json::<BootstrapRefusalDetails>(&bytes, &ContractLimits::packet0())
            .ok()
            .map(|details| details.reason())
    }
}

impl<'de> Deserialize<'de> for BootstrapRefusalV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Raw {
            schema: ExactBinding,
            #[serde(default)]
            request_id: RequiredOption<super::RequestId>,
            request_fingerprint: Fingerprint,
            #[serde(default)]
            bootstrap_descriptor: RequiredOption<ExactBinding>,
            status: TerminalStatus,
            #[serde(default)]
            data: RequiredOption<BootstrapRefusalDetails>,
            blockers: BoundedVec<Problem, 128>,
            refusals: BoundedVec<Problem, 128>,
            errors: BoundedVec<Problem, 128>,
            schema_manifest: BoundedVec<SchemaManifestEntry, 256>,
            response_fingerprint: Fingerprint,
        }
        let raw = Raw::deserialize(deserializer).map_err(sanitize_deserialize_error)?;
        let request_id = raw.request_id.into_option().map_err(D::Error::custom)?;
        let bootstrap_descriptor = raw
            .bootstrap_descriptor
            .into_option()
            .map_err(D::Error::custom)?;
        let data = raw.data.into_option().map_err(D::Error::custom)?;
        validate_manifest_order(raw.schema_manifest.as_slice()).map_err(D::Error::custom)?;
        if raw.status != TerminalStatus::Refused
            || data.is_some()
            || !raw.blockers.as_slice().is_empty()
            || raw.refusals.as_slice().len() != 1
            || !raw.errors.as_slice().is_empty()
        {
            return Err(D::Error::custom(ContractError::InvalidInvariant));
        }
        let expected = canonical_fingerprint(
            &RefusalPreimage {
                schema: &raw.schema,
                request_id: request_id.as_ref(),
                request_fingerprint: &raw.request_fingerprint,
                bootstrap_descriptor: bootstrap_descriptor.as_ref(),
                status: raw.status,
                data: data.as_ref(),
                blockers: &raw.blockers,
                refusals: &raw.refusals,
                errors: &raw.errors,
                schema_manifest: &raw.schema_manifest,
            },
            &ContractLimits::packet0(),
        )
        .map_err(D::Error::custom)?;
        if raw.response_fingerprint != expected {
            return Err(D::Error::custom(ContractError::FingerprintMismatch));
        }
        Ok(Self {
            schema: raw.schema,
            request_id,
            request_fingerprint: raw.request_fingerprint,
            bootstrap_descriptor,
            status: raw.status,
            data,
            blockers: raw.blockers,
            refusals: raw.refusals,
            errors: raw.errors,
            schema_manifest: raw.schema_manifest,
            response_fingerprint: raw.response_fingerprint,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CatalogReadBlockedV1 {
    schema: ExactBinding,
    api_major: SafeU64,
    catalog: ExactBinding,
    status: TerminalStatus,
    data: Option<ContractValue>,
    blockers: BoundedVec<Problem, 128>,
    refusals: BoundedVec<Problem, 128>,
    errors: BoundedVec<Problem, 128>,
    restart_from_first_page: bool,
    response_fingerprint: Fingerprint,
}

impl CatalogReadBlockedV1 {
    pub fn expired(
        schema: ExactBinding,
        root: CatalogRoot,
        catalog_root_schema: ExactBinding,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        require_ref(&schema, "handbook.catalog-read-blocked@1.0.0")?;
        require_ref(&catalog_root_schema, "handbook.catalog-root@1.0.0")?;
        let root_bytes = canonical_json_bytes(&root, limits)?;
        let root_value = parse_canonical_json::<ContractValue>(&root_bytes, limits)?;
        let catalog = root.catalog().clone();
        let condition =
            RecheckCondition::new(catalog_root_schema.clone(), root_value.clone(), limits)?;
        let blocker = Problem::new(
            BoundedString::new("catalog.snapshot.expired".to_owned())?,
            BoundedString::new("catalog_snapshot_expired".to_owned())?,
            ProblemCategory::Prerequisite,
            catalog.clone(),
            None,
            catalog_root_schema,
            root_value,
            BoundedVec::new(Vec::new())?,
            RetryDirective::AfterRecheck,
            Some(condition),
            None,
            limits,
        )?;
        let api_major = SafeU64::new(1)?;
        let status = TerminalStatus::Blocked;
        let data = None;
        let blockers = BoundedVec::new(vec![blocker])?;
        let refusals = BoundedVec::new(Vec::new())?;
        let errors = BoundedVec::new(Vec::new())?;
        let restart_from_first_page = true;
        let response_fingerprint = canonical_fingerprint(
            &CatalogBlockedPreimage {
                schema: &schema,
                api_major,
                catalog: &catalog,
                status,
                data: data.as_ref(),
                blockers: &blockers,
                refusals: &refusals,
                errors: &errors,
                restart_from_first_page,
            },
            limits,
        )?;
        let value = Self {
            schema,
            api_major,
            catalog,
            status,
            data,
            blockers,
            refusals,
            errors,
            restart_from_first_page,
            response_fingerprint,
        };
        canonical_json_bytes(&value, limits)?;
        Ok(value)
    }

    pub fn status(&self) -> TerminalStatus {
        self.status
    }

    pub fn blockers(&self) -> &[Problem] {
        self.blockers.as_slice()
    }

    pub fn restart_from_first_page(&self) -> bool {
        self.restart_from_first_page
    }
}

impl<'de> Deserialize<'de> for CatalogReadBlockedV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Raw {
            schema: ExactBinding,
            api_major: SafeU64,
            catalog: ExactBinding,
            status: TerminalStatus,
            #[serde(default)]
            data: RequiredOption<ContractValue>,
            blockers: BoundedVec<Problem, 128>,
            refusals: BoundedVec<Problem, 128>,
            errors: BoundedVec<Problem, 128>,
            restart_from_first_page: bool,
            response_fingerprint: Fingerprint,
        }
        let raw = Raw::deserialize(deserializer).map_err(sanitize_deserialize_error)?;
        let data = raw.data.into_option().map_err(D::Error::custom)?;
        if raw.api_major.get() != 1
            || raw.status != TerminalStatus::Blocked
            || data.is_some()
            || raw.blockers.as_slice().len() != 1
            || !raw.refusals.as_slice().is_empty()
            || !raw.errors.as_slice().is_empty()
            || !raw.restart_from_first_page
        {
            return Err(D::Error::custom(ContractError::InvalidInvariant));
        }
        let expected = canonical_fingerprint(
            &CatalogBlockedPreimage {
                schema: &raw.schema,
                api_major: raw.api_major,
                catalog: &raw.catalog,
                status: raw.status,
                data: data.as_ref(),
                blockers: &raw.blockers,
                refusals: &raw.refusals,
                errors: &raw.errors,
                restart_from_first_page: raw.restart_from_first_page,
            },
            &ContractLimits::packet0(),
        )
        .map_err(D::Error::custom)?;
        if raw.response_fingerprint != expected {
            return Err(D::Error::custom(ContractError::FingerprintMismatch));
        }
        Ok(Self {
            schema: raw.schema,
            api_major: raw.api_major,
            catalog: raw.catalog,
            status: raw.status,
            data,
            blockers: raw.blockers,
            refusals: raw.refusals,
            errors: raw.errors,
            restart_from_first_page: raw.restart_from_first_page,
            response_fingerprint: raw.response_fingerprint,
        })
    }
}

#[derive(Serialize)]
struct CatalogRootsPreimage<'a> {
    schema: &'a ExactBinding,
    operations: &'a CatalogRoot,
    schemas: &'a CatalogRoot,
    profiles: &'a CatalogRoot,
}

#[derive(Serialize)]
struct ResponsePreimage<'a> {
    schema: &'a ExactBinding,
    request_id: Option<&'a super::RequestId>,
    request_fingerprint: &'a Fingerprint,
    status: TerminalStatus,
    descriptor: &'a BootstrapDescriptorV1,
    catalog_roots: &'a CatalogRoots,
    first_capability_page: &'a CapabilityCatalogPage,
    owner_versions: &'a BoundedVec<super::OwnerVersionEntry, 128>,
    schema_manifest: &'a BoundedVec<SchemaManifestEntry, 256>,
}

#[derive(Serialize)]
struct RefusalPreimage<'a> {
    schema: &'a ExactBinding,
    request_id: Option<&'a super::RequestId>,
    request_fingerprint: &'a Fingerprint,
    bootstrap_descriptor: Option<&'a ExactBinding>,
    status: TerminalStatus,
    data: Option<&'a BootstrapRefusalDetails>,
    blockers: &'a BoundedVec<Problem, 128>,
    refusals: &'a BoundedVec<Problem, 128>,
    errors: &'a BoundedVec<Problem, 128>,
    schema_manifest: &'a BoundedVec<SchemaManifestEntry, 256>,
}

#[derive(Serialize)]
struct CatalogBlockedPreimage<'a> {
    schema: &'a ExactBinding,
    api_major: SafeU64,
    catalog: &'a ExactBinding,
    status: TerminalStatus,
    data: Option<&'a ContractValue>,
    blockers: &'a BoundedVec<Problem, 128>,
    refusals: &'a BoundedVec<Problem, 128>,
    errors: &'a BoundedVec<Problem, 128>,
    restart_from_first_page: bool,
}

fn validate_owner_order(entries: &[super::OwnerVersionEntry]) -> Result<(), ContractError> {
    validate_order(entries.iter().map(super::OwnerVersionEntry::owner_crate))
}

fn validate_manifest_order(entries: &[SchemaManifestEntry]) -> Result<(), ContractError> {
    validate_order(
        entries
            .iter()
            .map(|entry| entry.schema().reference().as_str()),
    )
}

fn validate_order<'a>(values: impl Iterator<Item = &'a str>) -> Result<(), ContractError> {
    let mut previous = None;
    for value in values {
        if let Some(previous) = previous {
            if previous >= value {
                return Err(if previous == value {
                    ContractError::DuplicateMember
                } else {
                    ContractError::NonCanonicalOrder
                });
            }
        }
        previous = Some(value);
    }
    Ok(())
}

fn manifest_binding(
    manifest: &[SchemaManifestEntry],
    reference: &str,
) -> Result<ExactBinding, ContractError> {
    manifest
        .iter()
        .find(|entry| entry.schema().reference().as_str() == reference)
        .map(|entry| entry.schema().clone())
        .ok_or(ContractError::InvalidInvariant)
}

fn require_ref(binding: &ExactBinding, expected: &str) -> Result<(), ContractError> {
    if binding.reference().as_str() == expected {
        Ok(())
    } else {
        Err(ContractError::InvalidInvariant)
    }
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
