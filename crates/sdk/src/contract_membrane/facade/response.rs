use super::{
    parse_canonical_wire, parse_contract_value, require_schema_binding, schema_binding,
    RequiredOption, BOOTSTRAP_FACADE_RESPONSE_REF, CATALOG_READ_REQUEST_REF,
    CATALOG_READ_RESPONSE_REF,
};
use crate::contract_membrane::serialization::classify_contract_error_message;
use crate::contract_membrane::{
    canonical_fingerprint, canonical_json_bytes, BootstrapRefusalV1, BootstrapResponseV1,
    CapabilityCatalogPage, CatalogPageSize, CatalogReadBlockedV1, ContractError, ContractLimits,
    ExactBinding, Fingerprint, ProfileCatalogPage, SchemaCatalogPage, TypedCatalogCursor,
};
use serde::de::Error as _;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;

const CAPABILITIES_CATALOG_REF: &str = "handbook.catalog.operations@1.0.0";
const SCHEMAS_CATALOG_REF: &str = "handbook.catalog.schemas@1.0.0";
const PROFILES_CATALOG_REF: &str = "handbook.catalog.profiles@1.0.0";

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BootstrapFacadeOutcomeKindV1 {
    Ok,
    Refused,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BootstrapFacadeResponseBodyV1 {
    Ok(Box<BootstrapResponseV1>),
    Refused(Box<BootstrapRefusalV1>),
}

impl Serialize for BootstrapFacadeResponseBodyV1 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Ok(value) => value.serialize(serializer),
            Self::Refused(value) => value.serialize(serializer),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct BootstrapFacadeResponseV1 {
    schema: ExactBinding,
    outcome_kind: BootstrapFacadeOutcomeKindV1,
    response: BootstrapFacadeResponseBodyV1,
    response_fingerprint: Fingerprint,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawBootstrapFacadeResponseV1 {
    schema: ExactBinding,
    outcome_kind: BootstrapFacadeOutcomeKindV1,
    response: Value,
    response_fingerprint: Fingerprint,
}

impl BootstrapFacadeResponseV1 {
    pub fn ok(
        response: BootstrapResponseV1,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        Self::from_parts(
            schema_binding(BOOTSTRAP_FACADE_RESPONSE_REF)?,
            BootstrapFacadeOutcomeKindV1::Ok,
            BootstrapFacadeResponseBodyV1::Ok(Box::new(response)),
            limits,
        )
    }

    pub fn refused(
        response: BootstrapRefusalV1,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        Self::from_parts(
            schema_binding(BOOTSTRAP_FACADE_RESPONSE_REF)?,
            BootstrapFacadeOutcomeKindV1::Refused,
            BootstrapFacadeResponseBodyV1::Refused(Box::new(response)),
            limits,
        )
    }

    fn from_parts(
        schema: ExactBinding,
        outcome_kind: BootstrapFacadeOutcomeKindV1,
        response: BootstrapFacadeResponseBodyV1,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        require_schema_binding(&schema, BOOTSTRAP_FACADE_RESPONSE_REF)?;
        let nested_schema_ref = match (outcome_kind, &response) {
            (BootstrapFacadeOutcomeKindV1::Ok, BootstrapFacadeResponseBodyV1::Ok(_)) => {
                "handbook.bootstrap-response@1.0.0"
            }
            (BootstrapFacadeOutcomeKindV1::Refused, BootstrapFacadeResponseBodyV1::Refused(_)) => {
                "handbook.bootstrap-refusal@1.0.0"
            }
            _ => return Err(ContractError::InvalidInvariant),
        };
        require_schema_binding(
            &embedded_schema_binding(&response, limits)?,
            nested_schema_ref,
        )?;
        let response_fingerprint = canonical_fingerprint(
            &BootstrapFacadeResponsePreimage {
                schema: &schema,
                outcome_kind,
                response: &response,
            },
            limits,
        )?;
        let value = Self {
            schema,
            outcome_kind,
            response,
            response_fingerprint,
        };
        canonical_json_bytes(&value, limits)?;
        Ok(value)
    }

    fn from_raw(
        raw: RawBootstrapFacadeResponseV1,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        let response = match raw.outcome_kind {
            BootstrapFacadeOutcomeKindV1::Ok => BootstrapFacadeResponseBodyV1::Ok(Box::new(
                parse_contract_value(&raw.response, limits)?,
            )),
            BootstrapFacadeOutcomeKindV1::Refused => BootstrapFacadeResponseBodyV1::Refused(
                Box::new(parse_contract_value(&raw.response, limits)?),
            ),
        };
        let value = Self::from_parts(raw.schema, raw.outcome_kind, response, limits)?;
        if value.response_fingerprint != raw.response_fingerprint {
            return Err(ContractError::FingerprintMismatch);
        }
        Ok(value)
    }

    pub fn parse_and_admit(bytes: &[u8], limits: &ContractLimits) -> Result<Self, ContractError> {
        let value = parse_canonical_wire(bytes, limits)?;
        Self::from_raw(parse_contract_value(&value, limits)?, limits)
    }

    pub fn schema(&self) -> &ExactBinding {
        &self.schema
    }

    pub fn outcome_kind(&self) -> BootstrapFacadeOutcomeKindV1 {
        self.outcome_kind
    }

    pub fn response(&self) -> &BootstrapFacadeResponseBodyV1 {
        &self.response
    }

    pub fn response_fingerprint(&self) -> &Fingerprint {
        &self.response_fingerprint
    }
}

impl<'de> Deserialize<'de> for BootstrapFacadeResponseV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = RawBootstrapFacadeResponseV1::deserialize(deserializer)
            .map_err(sanitize_deserialize_error)?;
        Self::from_raw(raw, &ContractLimits::packet0()).map_err(D::Error::custom)
    }
}

#[derive(Serialize)]
struct BootstrapFacadeResponsePreimage<'a> {
    schema: &'a ExactBinding,
    outcome_kind: BootstrapFacadeOutcomeKindV1,
    response: &'a BootstrapFacadeResponseBodyV1,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CatalogKindV1 {
    Capabilities,
    Schemas,
    Profiles,
}

impl CatalogKindV1 {
    fn exact_ref(self) -> &'static str {
        match self {
            Self::Capabilities => CAPABILITIES_CATALOG_REF,
            Self::Schemas => SCHEMAS_CATALOG_REF,
            Self::Profiles => PROFILES_CATALOG_REF,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CatalogReadRequestV1 {
    schema: ExactBinding,
    catalog_kind: CatalogKindV1,
    page_size: CatalogPageSize,
    cursor: Option<TypedCatalogCursor>,
    request_fingerprint: Fingerprint,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawCatalogReadRequestV1 {
    schema: ExactBinding,
    catalog_kind: CatalogKindV1,
    page_size: CatalogPageSize,
    #[serde(default)]
    cursor: RequiredOption<TypedCatalogCursor>,
    request_fingerprint: Fingerprint,
}

impl CatalogReadRequestV1 {
    pub fn new(
        catalog_kind: CatalogKindV1,
        page_size: CatalogPageSize,
        cursor: Option<TypedCatalogCursor>,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        Self::from_parts(
            schema_binding(CATALOG_READ_REQUEST_REF)?,
            catalog_kind,
            page_size,
            cursor,
            limits,
        )
    }

    fn from_parts(
        schema: ExactBinding,
        catalog_kind: CatalogKindV1,
        page_size: CatalogPageSize,
        cursor: Option<TypedCatalogCursor>,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        require_schema_binding(&schema, CATALOG_READ_REQUEST_REF)?;
        let request_fingerprint = canonical_fingerprint(
            &CatalogReadRequestPreimage {
                schema: &schema,
                catalog_kind,
                page_size,
                cursor: cursor.as_ref(),
            },
            limits,
        )?;
        let value = Self {
            schema,
            catalog_kind,
            page_size,
            cursor,
            request_fingerprint,
        };
        canonical_json_bytes(&value, limits)?;
        Ok(value)
    }

    fn from_raw(
        raw: RawCatalogReadRequestV1,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        let value = Self::from_parts(
            raw.schema,
            raw.catalog_kind,
            raw.page_size,
            raw.cursor.into_option()?,
            limits,
        )?;
        if value.request_fingerprint != raw.request_fingerprint {
            return Err(ContractError::FingerprintMismatch);
        }
        Ok(value)
    }

    pub fn parse_and_admit(bytes: &[u8], limits: &ContractLimits) -> Result<Self, ContractError> {
        let value = parse_canonical_wire(bytes, limits)?;
        Self::from_raw(parse_contract_value(&value, limits)?, limits)
    }

    pub fn schema(&self) -> &ExactBinding {
        &self.schema
    }

    pub fn catalog_kind(&self) -> CatalogKindV1 {
        self.catalog_kind
    }

    pub fn page_size(&self) -> CatalogPageSize {
        self.page_size
    }

    pub fn cursor(&self) -> Option<&TypedCatalogCursor> {
        self.cursor.as_ref()
    }

    pub fn request_fingerprint(&self) -> &Fingerprint {
        &self.request_fingerprint
    }
}

impl<'de> Deserialize<'de> for CatalogReadRequestV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = RawCatalogReadRequestV1::deserialize(deserializer)
            .map_err(sanitize_deserialize_error)?;
        Self::from_raw(raw, &ContractLimits::packet0()).map_err(D::Error::custom)
    }
}

#[derive(Serialize)]
struct CatalogReadRequestPreimage<'a> {
    schema: &'a ExactBinding,
    catalog_kind: CatalogKindV1,
    page_size: CatalogPageSize,
    cursor: Option<&'a TypedCatalogCursor>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CatalogReadOutcomeKindV1 {
    Page,
    CursorMismatch,
    Blocked,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CatalogReadResponseBodyV1 {
    CapabilityPage(Box<CapabilityCatalogPage>),
    SchemaPage(Box<SchemaCatalogPage>),
    ProfilePage(Box<ProfileCatalogPage>),
    CursorMismatch,
    Blocked(Box<CatalogReadBlockedV1>),
}

impl Serialize for CatalogReadResponseBodyV1 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::CapabilityPage(value) => value.serialize(serializer),
            Self::SchemaPage(value) => value.serialize(serializer),
            Self::ProfilePage(value) => value.serialize(serializer),
            Self::CursorMismatch => serializer.serialize_none(),
            Self::Blocked(value) => value.serialize(serializer),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CatalogReadResponseV1 {
    schema: ExactBinding,
    catalog_kind: CatalogKindV1,
    outcome_kind: CatalogReadOutcomeKindV1,
    response: CatalogReadResponseBodyV1,
    response_fingerprint: Fingerprint,
}

impl Serialize for CatalogReadResponseV1 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut object = serializer.serialize_struct("CatalogReadResponseV1", 5)?;
        object.serialize_field("schema", &self.schema)?;
        object.serialize_field("catalog_kind", &self.catalog_kind)?;
        object.serialize_field("outcome_kind", &self.outcome_kind)?;
        object.serialize_field("response", &self.response)?;
        object.serialize_field("response_fingerprint", &self.response_fingerprint)?;
        object.end()
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawCatalogReadResponseV1 {
    schema: ExactBinding,
    catalog_kind: CatalogKindV1,
    outcome_kind: CatalogReadOutcomeKindV1,
    response: Value,
    response_fingerprint: Fingerprint,
}

impl CatalogReadResponseV1 {
    pub fn capability_page(
        response: CapabilityCatalogPage,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        Self::from_parts(
            schema_binding(CATALOG_READ_RESPONSE_REF)?,
            CatalogKindV1::Capabilities,
            CatalogReadOutcomeKindV1::Page,
            CatalogReadResponseBodyV1::CapabilityPage(Box::new(response)),
            limits,
        )
    }

    pub fn schema_page(
        response: SchemaCatalogPage,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        Self::from_parts(
            schema_binding(CATALOG_READ_RESPONSE_REF)?,
            CatalogKindV1::Schemas,
            CatalogReadOutcomeKindV1::Page,
            CatalogReadResponseBodyV1::SchemaPage(Box::new(response)),
            limits,
        )
    }

    pub fn profile_page(
        response: ProfileCatalogPage,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        Self::from_parts(
            schema_binding(CATALOG_READ_RESPONSE_REF)?,
            CatalogKindV1::Profiles,
            CatalogReadOutcomeKindV1::Page,
            CatalogReadResponseBodyV1::ProfilePage(Box::new(response)),
            limits,
        )
    }

    pub fn cursor_mismatch(
        catalog_kind: CatalogKindV1,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        Self::from_parts(
            schema_binding(CATALOG_READ_RESPONSE_REF)?,
            catalog_kind,
            CatalogReadOutcomeKindV1::CursorMismatch,
            CatalogReadResponseBodyV1::CursorMismatch,
            limits,
        )
    }

    pub fn blocked(
        catalog_kind: CatalogKindV1,
        response: CatalogReadBlockedV1,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        Self::from_parts(
            schema_binding(CATALOG_READ_RESPONSE_REF)?,
            catalog_kind,
            CatalogReadOutcomeKindV1::Blocked,
            CatalogReadResponseBodyV1::Blocked(Box::new(response)),
            limits,
        )
    }

    fn from_parts(
        schema: ExactBinding,
        catalog_kind: CatalogKindV1,
        outcome_kind: CatalogReadOutcomeKindV1,
        response: CatalogReadResponseBodyV1,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        require_schema_binding(&schema, CATALOG_READ_RESPONSE_REF)?;
        validate_catalog_response(catalog_kind, outcome_kind, &response, limits)?;
        let response_fingerprint = canonical_fingerprint(
            &CatalogReadResponsePreimage {
                schema: &schema,
                catalog_kind,
                outcome_kind,
                response: &response,
            },
            limits,
        )?;
        let value = Self {
            schema,
            catalog_kind,
            outcome_kind,
            response,
            response_fingerprint,
        };
        canonical_json_bytes(&value, limits)?;
        Ok(value)
    }

    fn from_raw(
        raw: RawCatalogReadResponseV1,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        let response = match raw.outcome_kind {
            CatalogReadOutcomeKindV1::Page => match raw.catalog_kind {
                CatalogKindV1::Capabilities => CatalogReadResponseBodyV1::CapabilityPage(Box::new(
                    parse_contract_value(&raw.response, limits)?,
                )),
                CatalogKindV1::Schemas => CatalogReadResponseBodyV1::SchemaPage(Box::new(
                    parse_contract_value(&raw.response, limits)?,
                )),
                CatalogKindV1::Profiles => CatalogReadResponseBodyV1::ProfilePage(Box::new(
                    parse_contract_value(&raw.response, limits)?,
                )),
            },
            CatalogReadOutcomeKindV1::CursorMismatch => {
                if !raw.response.is_null() {
                    return Err(ContractError::InvalidInvariant);
                }
                CatalogReadResponseBodyV1::CursorMismatch
            }
            CatalogReadOutcomeKindV1::Blocked => CatalogReadResponseBodyV1::Blocked(Box::new(
                parse_contract_value(&raw.response, limits)?,
            )),
        };
        let value = Self::from_parts(
            raw.schema,
            raw.catalog_kind,
            raw.outcome_kind,
            response,
            limits,
        )?;
        if value.response_fingerprint != raw.response_fingerprint {
            return Err(ContractError::FingerprintMismatch);
        }
        Ok(value)
    }

    pub fn parse_and_admit(bytes: &[u8], limits: &ContractLimits) -> Result<Self, ContractError> {
        let value = parse_canonical_wire(bytes, limits)?;
        Self::from_raw(parse_contract_value(&value, limits)?, limits)
    }

    pub fn schema(&self) -> &ExactBinding {
        &self.schema
    }

    pub fn catalog_kind(&self) -> CatalogKindV1 {
        self.catalog_kind
    }

    pub fn outcome_kind(&self) -> CatalogReadOutcomeKindV1 {
        self.outcome_kind
    }

    pub fn response(&self) -> &CatalogReadResponseBodyV1 {
        &self.response
    }

    pub fn response_fingerprint(&self) -> &Fingerprint {
        &self.response_fingerprint
    }
}

impl<'de> Deserialize<'de> for CatalogReadResponseV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = RawCatalogReadResponseV1::deserialize(deserializer)
            .map_err(sanitize_deserialize_error)?;
        Self::from_raw(raw, &ContractLimits::packet0()).map_err(D::Error::custom)
    }
}

#[derive(Serialize)]
struct CatalogReadResponsePreimage<'a> {
    schema: &'a ExactBinding,
    catalog_kind: CatalogKindV1,
    outcome_kind: CatalogReadOutcomeKindV1,
    response: &'a CatalogReadResponseBodyV1,
}

fn validate_catalog_response(
    catalog_kind: CatalogKindV1,
    outcome_kind: CatalogReadOutcomeKindV1,
    response: &CatalogReadResponseBodyV1,
    limits: &ContractLimits,
) -> Result<(), ContractError> {
    let valid = match (catalog_kind, outcome_kind, response) {
        (
            CatalogKindV1::Capabilities,
            CatalogReadOutcomeKindV1::Page,
            CatalogReadResponseBodyV1::CapabilityPage(page),
        ) => {
            require_schema_binding(page.schema(), "handbook.capability-catalog-page@1.0.0")?;
            page.root().catalog().reference().as_str() == catalog_kind.exact_ref()
        }
        (
            CatalogKindV1::Schemas,
            CatalogReadOutcomeKindV1::Page,
            CatalogReadResponseBodyV1::SchemaPage(page),
        ) => {
            require_schema_binding(page.schema(), "handbook.schema-catalog-page@1.0.0")?;
            page.root().catalog().reference().as_str() == catalog_kind.exact_ref()
        }
        (
            CatalogKindV1::Profiles,
            CatalogReadOutcomeKindV1::Page,
            CatalogReadResponseBodyV1::ProfilePage(page),
        ) => {
            require_schema_binding(page.schema(), "handbook.profile-catalog-page@1.0.0")?;
            page.root().catalog().reference().as_str() == catalog_kind.exact_ref()
        }
        (
            _,
            CatalogReadOutcomeKindV1::CursorMismatch,
            CatalogReadResponseBodyV1::CursorMismatch,
        ) => true,
        (_, CatalogReadOutcomeKindV1::Blocked, CatalogReadResponseBodyV1::Blocked(blocked)) => {
            let (schema, catalog) = blocked_bindings(blocked, limits)?;
            require_schema_binding(&schema, "handbook.catalog-read-blocked@1.0.0")?;
            catalog.reference().as_str() == catalog_kind.exact_ref()
        }
        _ => false,
    };
    if !valid {
        return Err(ContractError::InvalidInvariant);
    }
    Ok(())
}

fn blocked_bindings(
    blocked: &CatalogReadBlockedV1,
    limits: &ContractLimits,
) -> Result<(ExactBinding, ExactBinding), ContractError> {
    #[derive(Deserialize)]
    struct Probe {
        schema: ExactBinding,
        catalog: ExactBinding,
    }
    let bytes = canonical_json_bytes(blocked, limits)?;
    let probe: Probe = serde_json::from_slice(&bytes)
        .map_err(|error| classify_contract_error_message(&error.to_string()))?;
    Ok((probe.schema, probe.catalog))
}

fn embedded_schema_binding(
    value: &impl Serialize,
    limits: &ContractLimits,
) -> Result<ExactBinding, ContractError> {
    #[derive(Deserialize)]
    struct Probe {
        schema: ExactBinding,
    }
    let bytes = canonical_json_bytes(value, limits)?;
    let probe: Probe = serde_json::from_slice(&bytes)
        .map_err(|error| classify_contract_error_message(&error.to_string()))?;
    Ok(probe.schema)
}

fn sanitize_deserialize_error<E: serde::de::Error>(error: E) -> E {
    E::custom(classify_contract_error_message(&error.to_string()))
}
