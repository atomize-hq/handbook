use super::serialization::classify_contract_error_message;
use super::{
    canonical_fingerprint, canonical_json_bytes, parse_canonical_json, with_owner_idempotency_key,
    BoundedString, BoundedVec, ContractError, ContractLimits, ContractValue, ContractValueRef,
    DefinitionPinnedKeyPointer, Diagnostic, ExactBinding, ExactRef, Fingerprint,
    IdempotencyKeyConsumer, IdempotencyState, IdempotencyStateRef, NextAction, OperationClass,
    OperationDefinition, OperationIdempotency, Problem, ProblemCategory, RawIdempotencyKey,
    TerminalFields, TerminalStatus, WriteReceipt,
};
use serde::de::Error as _;
use serde::ser::{SerializeMap, SerializeStruct};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RequestId(BoundedString<128>);

impl RequestId {
    pub fn parse(value: &str) -> Result<Self, ContractError> {
        let bytes = value.as_bytes();
        if bytes.is_empty()
            || bytes.len() > 128
            || !bytes[0].is_ascii_alphanumeric()
            || !bytes[1..].iter().all(|byte| {
                byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b':' | b'-')
            })
        {
            return Err(ContractError::InvalidString);
        }
        Ok(Self(BoundedString::new(value.to_owned())?))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl Serialize for RequestId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for RequestId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)
            .map_err(|_| D::Error::custom(ContractError::InvalidJson))?;
        Self::parse(&value).map_err(D::Error::custom)
    }
}

/// Structural JSON-Schema validation used before typed envelope admission.
///
/// Implementing this interface never grants semantic-owner authority. Owner
/// admission remains a separate, owner-native token boundary.
pub trait OperationSchemaValidator {
    fn validate_instance(
        &self,
        schema: &ExactBinding,
        instance: &ContractValue,
    ) -> Result<(), ContractError>;

    fn validate_problem(
        &self,
        _status: TerminalStatus,
        problem: &Problem,
    ) -> Result<(), ContractError> {
        self.validate_instance(problem.details_schema(), problem.details())?;
        if let Some(recheck) = problem.recheck() {
            self.validate_instance(recheck.condition_schema(), recheck.condition())?;
        }
        Ok(())
    }
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct RawOperationRequestV1 {
    schema: ExactBinding,
    #[serde(default)]
    request_id: RequiredOption<RequestId>,
    operation: ExactBinding,
    body_schema: ExactBinding,
    body: ContractValue,
    request_fingerprint: Fingerprint,
}

pub struct OperationRequestV1 {
    schema: ExactBinding,
    request_id: Option<RequestId>,
    operation: ExactBinding,
    body_schema: ExactBinding,
    body: ContractValue,
    raw_idempotency_key: Option<RawIdempotencyKey>,
    idempotency_key_fingerprint: Option<Fingerprint>,
    request_fingerprint: Fingerprint,
}

impl OperationRequestV1 {
    pub fn new(
        schema: ExactBinding,
        request_id: Option<RequestId>,
        definition: &OperationDefinition,
        wire_body: ContractValue,
        validator: &dyn OperationSchemaValidator,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        require_exact_ref(&schema, "handbook.operation-request@1.0.0")?;
        validator.validate_instance(definition.request_schema(), &wire_body)?;
        let requires_key = !matches!(definition.idempotency(), OperationIdempotency::Safe);
        let (body, raw_idempotency_key) = extract_raw_key(wire_body, requires_key)?;
        Self::from_parts(
            schema,
            request_id,
            definition.binding(),
            definition.request_schema().clone(),
            body,
            raw_idempotency_key,
            limits,
        )
    }

    pub fn parse_and_admit(
        bytes: &[u8],
        definition: &OperationDefinition,
        validator: &dyn OperationSchemaValidator,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        let raw = parse_canonical_json::<RawOperationRequestV1>(bytes, limits)?;
        if raw.operation != definition.binding() || raw.body_schema != *definition.request_schema()
        {
            return Err(ContractError::InvalidInvariant);
        }
        let value = Self::new(
            raw.schema,
            raw.request_id.into_option()?,
            definition,
            raw.body,
            validator,
            limits,
        )?;
        if value.request_fingerprint != raw.request_fingerprint {
            return Err(ContractError::FingerprintMismatch);
        }
        Ok(value)
    }

    #[allow(clippy::too_many_arguments)]
    fn from_parts(
        schema: ExactBinding,
        request_id: Option<RequestId>,
        operation: ExactBinding,
        body_schema: ExactBinding,
        body: ContractValue,
        raw_idempotency_key: Option<RawIdempotencyKey>,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        require_exact_ref(&schema, "handbook.operation-request@1.0.0")?;
        let idempotency_key_fingerprint = raw_idempotency_key
            .as_ref()
            .map(|key| key.scoped_fingerprint(operation.reference()));
        let request_fingerprint = canonical_fingerprint(
            &RequestFingerprintPreimage {
                schema: &schema,
                operation: &operation,
                body_schema: &body_schema,
                sanitized_body: &body,
                idempotency_key_fingerprint: idempotency_key_fingerprint.as_ref(),
            },
            limits,
        )?;
        let value = Self {
            schema,
            request_id,
            operation,
            body_schema,
            body,
            raw_idempotency_key,
            idempotency_key_fingerprint,
            request_fingerprint,
        };
        canonical_json_bytes(&value, limits)?;
        Ok(value)
    }

    pub fn request_id(&self) -> Option<&RequestId> {
        self.request_id.as_ref()
    }

    pub fn schema(&self) -> &ExactBinding {
        &self.schema
    }

    pub fn operation(&self) -> &ExactBinding {
        &self.operation
    }

    pub fn body_schema(&self) -> &ExactBinding {
        &self.body_schema
    }

    pub fn body(&self) -> &ContractValue {
        &self.body
    }

    pub fn idempotency_key_fingerprint(&self) -> Option<&Fingerprint> {
        self.idempotency_key_fingerprint.as_ref()
    }

    pub fn request_fingerprint(&self) -> &Fingerprint {
        &self.request_fingerprint
    }

    pub fn with_owner_idempotency_key(
        &self,
        consumer: &mut dyn IdempotencyKeyConsumer,
    ) -> Option<Fingerprint> {
        self.raw_idempotency_key.as_ref().map(|key| {
            with_owner_idempotency_key(
                key,
                DefinitionPinnedKeyPointer::BodyIdempotencyKey,
                consumer,
            )
        })
    }
}

impl fmt::Debug for OperationRequestV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("OperationRequestV1")
            .field("schema", &self.schema)
            .field("request_id", &self.request_id)
            .field("operation", &self.operation)
            .field("body_schema", &self.body_schema)
            .field("body", &self.body)
            .field("raw_idempotency_key", &self.raw_idempotency_key)
            .field(
                "idempotency_key_fingerprint",
                &self.idempotency_key_fingerprint,
            )
            .field("request_fingerprint", &self.request_fingerprint)
            .finish()
    }
}

impl Serialize for OperationRequestV1 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut object = serializer.serialize_struct("OperationRequestV1", 6)?;
        object.serialize_field("schema", &self.schema)?;
        object.serialize_field("request_id", &self.request_id)?;
        object.serialize_field("operation", &self.operation)?;
        object.serialize_field("body_schema", &self.body_schema)?;
        object.serialize_field(
            "body",
            &WireBody {
                sanitized: &self.body,
                raw_key: self.raw_idempotency_key.as_ref(),
            },
        )?;
        object.serialize_field("request_fingerprint", &self.request_fingerprint)?;
        object.end()
    }
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct RawOperationResponseV1 {
    schema: ExactBinding,
    #[serde(default)]
    request_id: RequiredOption<RequestId>,
    request_fingerprint: Fingerprint,
    operation: ExactBinding,
    result_schema: ExactBinding,
    status: TerminalStatus,
    data: ContractValue,
    blockers: BoundedVec<Problem, 128>,
    refusals: BoundedVec<Problem, 128>,
    errors: BoundedVec<Problem, 128>,
    idempotency: IdempotencyState,
    receipts: BoundedVec<WriteReceipt, 128>,
    diagnostics: BoundedVec<Diagnostic, 128>,
    next_actions: BoundedVec<NextAction, 128>,
    result_fingerprint: Fingerprint,
    response_fingerprint: Fingerprint,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct OperationResponseV1 {
    schema: ExactBinding,
    request_id: Option<RequestId>,
    request_fingerprint: Fingerprint,
    operation: ExactBinding,
    result_schema: ExactBinding,
    #[serde(flatten)]
    terminal: TerminalFields<ContractValue>,
    idempotency: IdempotencyState,
    receipts: BoundedVec<WriteReceipt, 128>,
    diagnostics: BoundedVec<Diagnostic, 128>,
    next_actions: BoundedVec<NextAction, 128>,
    result_fingerprint: Fingerprint,
    response_fingerprint: Fingerprint,
}

impl OperationResponseV1 {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        schema: ExactBinding,
        request_id: Option<RequestId>,
        request_fingerprint: Fingerprint,
        definition: &OperationDefinition,
        terminal: TerminalFields<ContractValue>,
        idempotency: IdempotencyState,
        receipts: BoundedVec<WriteReceipt, 128>,
        diagnostics: BoundedVec<Diagnostic, 128>,
        next_actions: BoundedVec<NextAction, 128>,
        validator: &dyn OperationSchemaValidator,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        require_exact_ref(&schema, "handbook.operation-response@1.0.0")?;
        if let Some(data) = terminal.data() {
            validator.validate_instance(definition.result_schema(), data)?;
        }
        for problem in terminal
            .blockers()
            .iter()
            .chain(terminal.refusals())
            .chain(terminal.errors())
        {
            validator.validate_problem(terminal.status(), problem)?;
        }
        validate_idempotency(
            definition.class(),
            terminal.status(),
            &idempotency,
            &request_fingerprint,
        )?;
        validate_receipts(
            definition,
            terminal.status(),
            receipts.as_slice(),
            validator,
        )?;
        Self::from_parts(
            schema,
            request_id,
            request_fingerprint,
            definition.binding(),
            definition.result_schema().clone(),
            terminal,
            idempotency,
            receipts,
            diagnostics,
            next_actions,
            limits,
        )
    }

    pub fn parse_and_admit(
        bytes: &[u8],
        definition: &OperationDefinition,
        validator: &dyn OperationSchemaValidator,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        let raw = parse_canonical_json::<RawOperationResponseV1>(bytes, limits)?;
        if raw.operation != definition.binding() || raw.result_schema != *definition.result_schema()
        {
            return Err(ContractError::InvalidInvariant);
        }
        let data = match raw.status {
            TerminalStatus::Ok => Some(raw.data),
            _ if matches!(raw.data.as_ref(), ContractValueRef::Null) => None,
            _ => return Err(ContractError::InvalidInvariant),
        };
        let terminal =
            TerminalFields::new(raw.status, data, raw.blockers, raw.refusals, raw.errors)?;
        let value = Self::new(
            raw.schema,
            raw.request_id.into_option()?,
            raw.request_fingerprint,
            definition,
            terminal,
            raw.idempotency,
            raw.receipts,
            raw.diagnostics,
            raw.next_actions,
            validator,
            limits,
        )?;
        if value.result_fingerprint != raw.result_fingerprint
            || value.response_fingerprint != raw.response_fingerprint
        {
            return Err(ContractError::FingerprintMismatch);
        }
        Ok(value)
    }

    #[allow(clippy::too_many_arguments)]
    fn from_parts(
        schema: ExactBinding,
        request_id: Option<RequestId>,
        request_fingerprint: Fingerprint,
        operation: ExactBinding,
        result_schema: ExactBinding,
        terminal: TerminalFields<ContractValue>,
        idempotency: IdempotencyState,
        receipts: BoundedVec<WriteReceipt, 128>,
        diagnostics: BoundedVec<Diagnostic, 128>,
        next_actions: BoundedVec<NextAction, 128>,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        require_exact_ref(&schema, "handbook.operation-response@1.0.0")?;
        let result_fingerprint = canonical_fingerprint(
            &ResultPreimage {
                operation: &operation,
                result_schema: &result_schema,
                terminal: &terminal,
                receipts: &receipts,
            },
            limits,
        )?;
        validate_replay_state(&idempotency, &request_fingerprint, &result_fingerprint)?;
        let response_fingerprint = canonical_fingerprint(
            &ResponsePreimage {
                schema: &schema,
                request_id: request_id.as_ref(),
                request_fingerprint: &request_fingerprint,
                operation: &operation,
                result_schema: &result_schema,
                terminal: &terminal,
                idempotency: &idempotency,
                receipts: &receipts,
                diagnostics: &diagnostics,
                next_actions: &next_actions,
                result_fingerprint: &result_fingerprint,
            },
            limits,
        )?;
        let value = Self {
            schema,
            request_id,
            request_fingerprint,
            operation,
            result_schema,
            terminal,
            idempotency,
            receipts,
            diagnostics,
            next_actions,
            result_fingerprint,
            response_fingerprint,
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

    pub fn request_fingerprint(&self) -> &Fingerprint {
        &self.request_fingerprint
    }

    pub fn operation(&self) -> &ExactBinding {
        &self.operation
    }

    pub fn result_schema(&self) -> &ExactBinding {
        &self.result_schema
    }

    pub fn terminal(&self) -> &TerminalFields<ContractValue> {
        &self.terminal
    }

    pub fn idempotency(&self) -> &IdempotencyState {
        &self.idempotency
    }

    pub fn receipts(&self) -> &[WriteReceipt] {
        self.receipts.as_slice()
    }

    pub fn diagnostics(&self) -> &[Diagnostic] {
        self.diagnostics.as_slice()
    }

    pub fn next_actions(&self) -> &[NextAction] {
        self.next_actions.as_slice()
    }

    pub fn result_fingerprint(&self) -> &Fingerprint {
        &self.result_fingerprint
    }

    pub fn response_fingerprint(&self) -> &Fingerprint {
        &self.response_fingerprint
    }

    pub fn verify_replay_against(&self, original: &Self) -> Result<(), ContractError> {
        let (original_result, replay_key, replay_request) = match self.idempotency.as_ref() {
            IdempotencyStateRef::Established {
                idempotency_key_fingerprint,
                request_fingerprint,
                replayed: true,
                original_result_fingerprint: Some(fingerprint),
                ..
            } => (
                fingerprint,
                idempotency_key_fingerprint,
                request_fingerprint,
            ),
            _ => return Err(ContractError::InvalidInvariant),
        };
        let (original_key, original_request) = match original.idempotency.as_ref() {
            IdempotencyStateRef::Established {
                idempotency_key_fingerprint,
                request_fingerprint,
                replayed: false,
                original_result_fingerprint: None,
                ..
            } => (idempotency_key_fingerprint, request_fingerprint),
            _ => return Err(ContractError::InvalidInvariant),
        };
        if self.operation != original.operation
            || self.result_schema != original.result_schema
            || self.terminal != original.terminal
            || self.receipts != original.receipts
            || self.result_fingerprint != original.result_fingerprint
            || original_result != original.result_fingerprint()
            || replay_key != original_key
            || replay_request != original_request
        {
            return Err(ContractError::InvalidInvariant);
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct OperationAdmissionRefusalV1 {
    schema: ExactBinding,
    request_id: Option<RequestId>,
    attempted_operation: Option<ExactRef>,
    attempted_definition: Option<ExactBinding>,
    request_fingerprint: Option<Fingerprint>,
    #[serde(flatten)]
    terminal: TerminalFields<ContractValue>,
    response_fingerprint: Fingerprint,
}

impl OperationAdmissionRefusalV1 {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        schema: ExactBinding,
        request_id: Option<RequestId>,
        attempted_operation: Option<ExactRef>,
        attempted_definition: Option<ExactBinding>,
        request_fingerprint: Option<Fingerprint>,
        refusal: Problem,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        require_exact_ref(&schema, "handbook.operation-admission-refusal@1.0.0")?;
        if !matches!(
            refusal.category(),
            ProblemCategory::Schema | ProblemCategory::Compatibility | ProblemCategory::Capability
        ) {
            return Err(ContractError::InvalidInvariant);
        }
        let terminal = TerminalFields::new(
            TerminalStatus::Refused,
            None,
            BoundedVec::new(Vec::new())?,
            BoundedVec::new(vec![refusal])?,
            BoundedVec::new(Vec::new())?,
        )?;
        Self::from_parts(
            schema,
            request_id,
            attempted_operation,
            attempted_definition,
            request_fingerprint,
            terminal,
            limits,
        )
    }

    fn from_parts(
        schema: ExactBinding,
        request_id: Option<RequestId>,
        attempted_operation: Option<ExactRef>,
        attempted_definition: Option<ExactBinding>,
        request_fingerprint: Option<Fingerprint>,
        terminal: TerminalFields<ContractValue>,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        require_exact_ref(&schema, "handbook.operation-admission-refusal@1.0.0")?;
        if terminal.status() != TerminalStatus::Refused
            || terminal.refusals().len() != 1
            || terminal.data().is_some()
            || !terminal.blockers().is_empty()
            || !terminal.errors().is_empty()
        {
            return Err(ContractError::InvalidInvariant);
        }
        let response_fingerprint = canonical_fingerprint(
            &AdmissionRefusalPreimage {
                schema: &schema,
                request_id: request_id.as_ref(),
                attempted_operation: attempted_operation.as_ref(),
                attempted_definition: attempted_definition.as_ref(),
                request_fingerprint: request_fingerprint.as_ref(),
                terminal: &terminal,
            },
            limits,
        )?;
        Ok(Self {
            schema,
            request_id,
            attempted_operation,
            attempted_definition,
            request_fingerprint,
            terminal,
            response_fingerprint,
        })
    }

    pub fn schema(&self) -> &ExactBinding {
        &self.schema
    }

    pub fn request_id(&self) -> Option<&RequestId> {
        self.request_id.as_ref()
    }

    pub fn attempted_operation(&self) -> Option<&ExactRef> {
        self.attempted_operation.as_ref()
    }

    pub fn attempted_definition(&self) -> Option<&ExactBinding> {
        self.attempted_definition.as_ref()
    }

    pub fn request_fingerprint(&self) -> Option<&Fingerprint> {
        self.request_fingerprint.as_ref()
    }

    pub fn terminal(&self) -> &TerminalFields<ContractValue> {
        &self.terminal
    }

    pub fn response_fingerprint(&self) -> &Fingerprint {
        &self.response_fingerprint
    }
}

impl<'de> Deserialize<'de> for OperationAdmissionRefusalV1 {
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
            #[serde(default)]
            attempted_operation: RequiredOption<ExactRef>,
            #[serde(default)]
            attempted_definition: RequiredOption<ExactBinding>,
            #[serde(default)]
            request_fingerprint: RequiredOption<Fingerprint>,
            #[serde(flatten)]
            terminal: TerminalFields<ContractValue>,
            response_fingerprint: Fingerprint,
        }
        let raw = Raw::deserialize(deserializer).map_err(sanitize_deserialize_error)?;
        let value = Self::from_parts(
            raw.schema,
            raw.request_id.into_option().map_err(D::Error::custom)?,
            raw.attempted_operation
                .into_option()
                .map_err(D::Error::custom)?,
            raw.attempted_definition
                .into_option()
                .map_err(D::Error::custom)?,
            raw.request_fingerprint
                .into_option()
                .map_err(D::Error::custom)?,
            raw.terminal,
            &ContractLimits::packet0(),
        )
        .map_err(D::Error::custom)?;
        if value.response_fingerprint != raw.response_fingerprint {
            return Err(D::Error::custom(ContractError::FingerprintMismatch));
        }
        Ok(value)
    }
}

struct WireBody<'a> {
    sanitized: &'a ContractValue,
    raw_key: Option<&'a RawIdempotencyKey>,
}

impl Serialize for WireBody<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let Some(raw_key) = self.raw_key else {
            return self.sanitized.serialize(serializer);
        };
        let ContractValueRef::Object(members) = self.sanitized.as_ref() else {
            return Err(serde::ser::Error::custom(ContractError::InvalidInvariant));
        };
        let mut map = serializer.serialize_map(Some(members.len() + 1))?;
        let mut inserted = false;
        for member in members {
            if !inserted && member.key().as_str() > "idempotency_key" {
                map.serialize_entry("idempotency_key", raw_key.as_str())?;
                inserted = true;
            }
            map.serialize_entry(member.key().as_str(), member.value())?;
        }
        if !inserted {
            map.serialize_entry("idempotency_key", raw_key.as_str())?;
        }
        map.end()
    }
}

#[derive(Serialize)]
struct RequestFingerprintPreimage<'a> {
    schema: &'a ExactBinding,
    operation: &'a ExactBinding,
    body_schema: &'a ExactBinding,
    sanitized_body: &'a ContractValue,
    idempotency_key_fingerprint: Option<&'a Fingerprint>,
}

#[derive(Serialize)]
struct ResultPreimage<'a> {
    operation: &'a ExactBinding,
    result_schema: &'a ExactBinding,
    #[serde(flatten)]
    terminal: &'a TerminalFields<ContractValue>,
    receipts: &'a BoundedVec<WriteReceipt, 128>,
}

#[derive(Serialize)]
struct ResponsePreimage<'a> {
    schema: &'a ExactBinding,
    request_id: Option<&'a RequestId>,
    request_fingerprint: &'a Fingerprint,
    operation: &'a ExactBinding,
    result_schema: &'a ExactBinding,
    #[serde(flatten)]
    terminal: &'a TerminalFields<ContractValue>,
    idempotency: &'a IdempotencyState,
    receipts: &'a BoundedVec<WriteReceipt, 128>,
    diagnostics: &'a BoundedVec<Diagnostic, 128>,
    next_actions: &'a BoundedVec<NextAction, 128>,
    result_fingerprint: &'a Fingerprint,
}

#[derive(Serialize)]
struct AdmissionRefusalPreimage<'a> {
    schema: &'a ExactBinding,
    request_id: Option<&'a RequestId>,
    attempted_operation: Option<&'a ExactRef>,
    attempted_definition: Option<&'a ExactBinding>,
    request_fingerprint: Option<&'a Fingerprint>,
    #[serde(flatten)]
    terminal: &'a TerminalFields<ContractValue>,
}

fn extract_raw_key(
    body: ContractValue,
    required: bool,
) -> Result<(ContractValue, Option<RawIdempotencyKey>), ContractError> {
    let (body, key) = extract_raw_key_if_present(body)?;
    if required != key.is_some() {
        return Err(ContractError::InvalidInvariant);
    }
    Ok((body, key))
}

fn extract_raw_key_if_present(
    body: ContractValue,
) -> Result<(ContractValue, Option<RawIdempotencyKey>), ContractError> {
    let ContractValueRef::Object(members) = body.as_ref() else {
        return Ok((body, None));
    };
    let mut retained = Vec::with_capacity(members.len());
    let mut raw_key = None;
    for member in members {
        if member.key().as_str() == "idempotency_key" {
            if raw_key.is_some() {
                return Err(ContractError::DuplicateMember);
            }
            let ContractValueRef::String(value) = member.value().as_ref() else {
                return Err(ContractError::InvalidString);
            };
            raw_key = Some(RawIdempotencyKey::new(value.as_bytes().to_vec(), 65_536)?);
        } else {
            retained.push(member.clone());
        }
    }
    if raw_key.is_none() {
        return Ok((body, None));
    }
    Ok((ContractValue::object(retained)?, raw_key))
}

fn validate_idempotency(
    class: OperationClass,
    status: TerminalStatus,
    state: &IdempotencyState,
    envelope_request_fingerprint: &Fingerprint,
) -> Result<(), ContractError> {
    let valid = match (class, status, state.as_ref()) {
        (OperationClass::ReadOnly, _, IdempotencyStateRef::NotApplicable) => true,
        (
            OperationClass::AppendOnly | OperationClass::CompareAndWrite,
            TerminalStatus::Ok | TerminalStatus::Refused,
            IdempotencyStateRef::Established {
                request_fingerprint,
                ..
            },
        ) => request_fingerprint == envelope_request_fingerprint,
        (
            OperationClass::AppendOnly | OperationClass::CompareAndWrite,
            TerminalStatus::Blocked | TerminalStatus::Refused | TerminalStatus::Error,
            IdempotencyStateRef::NotEstablished { .. },
        ) => true,
        _ => false,
    };
    if valid {
        Ok(())
    } else {
        Err(ContractError::InvalidInvariant)
    }
}

fn validate_replay_state(
    state: &IdempotencyState,
    request_fingerprint: &Fingerprint,
    result_fingerprint: &Fingerprint,
) -> Result<(), ContractError> {
    if let IdempotencyStateRef::Established {
        request_fingerprint: state_request,
        replayed,
        original_result_fingerprint,
        ..
    } = state.as_ref()
    {
        if state_request != request_fingerprint
            || (replayed && original_result_fingerprint != Some(result_fingerprint))
            || (!replayed && original_result_fingerprint.is_some())
        {
            return Err(ContractError::InvalidInvariant);
        }
    }
    Ok(())
}

fn validate_receipts(
    definition: &OperationDefinition,
    status: TerminalStatus,
    receipts: &[WriteReceipt],
    validator: &dyn OperationSchemaValidator,
) -> Result<(), ContractError> {
    if definition.class() == OperationClass::ReadOnly {
        return if receipts.is_empty() {
            Ok(())
        } else {
            Err(ContractError::InvalidInvariant)
        };
    }
    let partial_terminal_receipts = status != TerminalStatus::Ok
        && matches!(
            definition.operation().as_str(),
            "pipeline.capture.apply@1.0.0"
                | "pipeline.handoff.emit@1.0.0"
                | "pipeline.state.apply@1.0.0"
        );
    if status != TerminalStatus::Ok && !partial_terminal_receipts {
        return if receipts.is_empty() {
            Ok(())
        } else {
            Err(ContractError::InvalidInvariant)
        };
    }
    if (status == TerminalStatus::Ok && receipts.len() != definition.write_set().len())
        || receipts.len() > definition.write_set().len()
    {
        return Err(ContractError::InvalidInvariant);
    }
    for (declared, realized) in definition.write_set().iter().zip(receipts) {
        if declared.record_kind() != realized.record_kind().as_str()
            || declared.authority_class() != realized.authority_class().as_str()
            || declared.atomic_group() != realized.atomic_group().as_str()
        {
            return Err(ContractError::InvalidInvariant);
        }
        validator.validate_instance(declared.condition_schema(), realized.condition())?;
    }
    Ok(())
}

fn require_exact_ref(binding: &ExactBinding, expected: &str) -> Result<(), ContractError> {
    if binding.reference().as_str() == expected {
        Ok(())
    } else {
        Err(ContractError::InvalidInvariant)
    }
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

impl<T: Serialize> Serialize for RequiredOption<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Present(value) => value.serialize(serializer),
            Self::Missing => Err(serde::ser::Error::custom(ContractError::InvalidInvariant)),
        }
    }
}

fn sanitize_deserialize_error<E: serde::de::Error>(error: E) -> E {
    E::custom(classify_contract_error_message(&error.to_string()))
}
