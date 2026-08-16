use super::serialization::classify_contract_error_message;
use super::{
    canonical_fingerprint, canonical_json_bytes, BoundedString, BoundedVec, ContractError,
    ContractLimits, ExactBinding, ExactRef, Fingerprint, SemVer,
};
use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::sync::OnceLock;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OperationOwner {
    Engine,
    Flow,
    Pipeline,
    SdkComposition,
    Phase5,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OperationAvailability {
    Absent,
    LivePrecursor,
    Phase5Deferred,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OperationClass {
    ReadOnly,
    AppendOnly,
    CompareAndWrite,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AdmissionReason {
    Admitted,
    IncompleteClosure,
    SemanticMismatch,
    IndependentOwnerAbsent,
    OwnerDeferred,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DiscoveryState {
    Discoverable,
    Omit,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OperationTransport {
    CliJson,
    RustSdk,
    Tauri,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OperationInventoryRow {
    proof_ordinal: u16,
    operation_id: &'static str,
    operation_ref: ExactRef,
    owner: OperationOwner,
    availability: OperationAvailability,
    class: OperationClass,
    admission_reason: AdmissionReason,
    current_transports: &'static [OperationTransport],
    discovery: DiscoveryState,
}

impl OperationInventoryRow {
    pub fn proof_ordinal(&self) -> u16 {
        self.proof_ordinal
    }

    pub fn operation_id(&self) -> &'static str {
        self.operation_id
    }

    pub fn operation_ref(&self) -> &ExactRef {
        &self.operation_ref
    }

    pub fn owner(&self) -> OperationOwner {
        self.owner
    }

    pub fn availability(&self) -> OperationAvailability {
        self.availability
    }

    pub fn class(&self) -> OperationClass {
        self.class
    }

    pub fn admission_reason(&self) -> AdmissionReason {
        self.admission_reason
    }

    pub fn current_transports(&self) -> &'static [OperationTransport] {
        self.current_transports
    }

    pub fn discovery(&self) -> DiscoveryState {
        self.discovery
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DefinitionSetError {
    MissingDefinition,
    DuplicateDefinition,
    UnknownDefinition,
    DefinitionMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct JsonPointer(BoundedString<255>);

impl JsonPointer {
    pub fn parse(value: &str) -> Result<Self, ContractError> {
        if !valid_json_pointer(value) {
            return Err(ContractError::InvalidString);
        }
        Ok(Self(BoundedString::new(value.to_owned())?))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl Serialize for JsonPointer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for JsonPointer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)
            .map_err(|_| D::Error::custom(ContractError::InvalidJson))?;
        Self::parse(&value).map_err(D::Error::custom)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct OperationOwnerBinding {
    owner_crate: BoundedString<255>,
    owner_version: SemVer,
    source: ExactBinding,
}

impl OperationOwnerBinding {
    pub fn new(
        owner_crate: BoundedString<255>,
        owner_version: SemVer,
        source: ExactBinding,
    ) -> Result<Self, ContractError> {
        validate_token(owner_crate.as_str())?;
        Ok(Self {
            owner_crate,
            owner_version,
            source,
        })
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
}

impl<'de> Deserialize<'de> for OperationOwnerBinding {
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
        }

        let raw = Raw::deserialize(deserializer).map_err(sanitize_deserialize_error)?;
        Self::new(raw.owner_crate, raw.owner_version, raw.source).map_err(D::Error::custom)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct OperationOutcomeSchemas {
    blocker: ExactBinding,
    refusal: ExactBinding,
    error: ExactBinding,
}

impl OperationOutcomeSchemas {
    pub fn new(
        blocker: ExactBinding,
        refusal: ExactBinding,
        error: ExactBinding,
    ) -> Result<Self, ContractError> {
        require_exact_ref(&blocker, "handbook.operation-blocker@1.0.0")?;
        require_exact_ref(&refusal, "handbook.operation-refusal@1.0.0")?;
        require_exact_ref(&error, "handbook.operation-error@1.0.0")?;
        Ok(Self {
            blocker,
            refusal,
            error,
        })
    }

    pub fn blocker(&self) -> &ExactBinding {
        &self.blocker
    }

    pub fn refusal(&self) -> &ExactBinding {
        &self.refusal
    }

    pub fn error(&self) -> &ExactBinding {
        &self.error
    }
}

impl<'de> Deserialize<'de> for OperationOutcomeSchemas {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Raw {
            blocker: ExactBinding,
            refusal: ExactBinding,
            error: ExactBinding,
        }
        let raw = Raw::deserialize(deserializer).map_err(sanitize_deserialize_error)?;
        Self::new(raw.blocker, raw.refusal, raw.error).map_err(D::Error::custom)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct DeclaredWrite {
    record_kind: BoundedString<255>,
    authority_class: BoundedString<255>,
    atomic_group: BoundedString<255>,
    condition_schema: ExactBinding,
}

impl DeclaredWrite {
    pub fn new(
        record_kind: BoundedString<255>,
        authority_class: BoundedString<255>,
        atomic_group: BoundedString<255>,
        condition_schema: ExactBinding,
    ) -> Result<Self, ContractError> {
        validate_token(record_kind.as_str())?;
        validate_token(authority_class.as_str())?;
        validate_token(atomic_group.as_str())?;
        Ok(Self {
            record_kind,
            authority_class,
            atomic_group,
            condition_schema,
        })
    }

    pub fn record_kind(&self) -> &str {
        self.record_kind.as_str()
    }

    pub fn authority_class(&self) -> &str {
        self.authority_class.as_str()
    }

    pub fn atomic_group(&self) -> &str {
        self.atomic_group.as_str()
    }

    pub fn condition_schema(&self) -> &ExactBinding {
        &self.condition_schema
    }
}

impl<'de> Deserialize<'de> for DeclaredWrite {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Raw {
            record_kind: BoundedString<255>,
            authority_class: BoundedString<255>,
            atomic_group: BoundedString<255>,
            condition_schema: ExactBinding,
        }
        let raw = Raw::deserialize(deserializer).map_err(sanitize_deserialize_error)?;
        Self::new(
            raw.record_kind,
            raw.authority_class,
            raw.atomic_group,
            raw.condition_schema,
        )
        .map_err(D::Error::custom)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum OperationIdempotency {
    Safe,
    IdempotencyKeyRequired {
        key_pointer: JsonPointer,
    },
    CompareAndWriteRequired {
        key_pointer: JsonPointer,
        expected_basis: BoundedVec<JsonPointer, 128>,
    },
}

impl OperationIdempotency {
    pub fn safe() -> Self {
        Self::Safe
    }

    pub fn key_required(key_pointer: JsonPointer) -> Result<Self, ContractError> {
        require_key_pointer(&key_pointer)?;
        Ok(Self::IdempotencyKeyRequired { key_pointer })
    }

    pub fn compare_and_write(
        key_pointer: JsonPointer,
        expected_basis: BoundedVec<JsonPointer, 128>,
    ) -> Result<Self, ContractError> {
        require_key_pointer(&key_pointer)?;
        if expected_basis.as_slice().is_empty()
            || expected_basis.as_slice().iter().any(|pointer| {
                pointer.as_str() == key_pointer.as_str() || !pointer.as_str().starts_with("/body/")
            })
        {
            return Err(ContractError::InvalidInvariant);
        }
        validate_strict_order(expected_basis.as_slice(), |pointer| pointer.as_str())?;
        Ok(Self::CompareAndWriteRequired {
            key_pointer,
            expected_basis,
        })
    }
}

impl<'de> Deserialize<'de> for OperationIdempotency {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
        enum Raw {
            Safe {},
            IdempotencyKeyRequired {
                key_pointer: JsonPointer,
            },
            CompareAndWriteRequired {
                key_pointer: JsonPointer,
                expected_basis: BoundedVec<JsonPointer, 128>,
            },
        }
        match Raw::deserialize(deserializer).map_err(sanitize_deserialize_error)? {
            Raw::Safe {} => Ok(Self::safe()),
            Raw::IdempotencyKeyRequired { key_pointer } => Self::key_required(key_pointer),
            Raw::CompareAndWriteRequired {
                key_pointer,
                expected_basis,
            } => Self::compare_and_write(key_pointer, expected_basis),
        }
        .map_err(D::Error::custom)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct OperationDefinition {
    definition_schema: ExactBinding,
    operation: ExactRef,
    owner: OperationOwnerBinding,
    request_schema: ExactBinding,
    result_schema: ExactBinding,
    outcome_schemas: OperationOutcomeSchemas,
    required_capabilities: BoundedVec<ExactBinding, 128>,
    class: OperationClass,
    idempotency: OperationIdempotency,
    write_set: BoundedVec<DeclaredWrite, 128>,
    declared_transports: BoundedVec<OperationTransport, 3>,
    deprecation: Option<ExactBinding>,
    definition_fingerprint: Fingerprint,
}

impl OperationDefinition {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        definition_schema: ExactBinding,
        operation: ExactRef,
        owner: OperationOwnerBinding,
        request_schema: ExactBinding,
        result_schema: ExactBinding,
        outcome_schemas: OperationOutcomeSchemas,
        required_capabilities: BoundedVec<ExactBinding, 128>,
        class: OperationClass,
        idempotency: OperationIdempotency,
        write_set: BoundedVec<DeclaredWrite, 128>,
        declared_transports: BoundedVec<OperationTransport, 3>,
        deprecation: Option<ExactBinding>,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        require_exact_ref(&definition_schema, "handbook.operation-definition@1.0.0")?;
        require_operation_schema(&operation, &request_schema, "request")?;
        require_operation_schema(&operation, &result_schema, "result")?;
        validate_binding_order(required_capabilities.as_slice())?;
        validate_definition_behavior(class, &idempotency, write_set.as_slice())?;
        validate_write_set(write_set.as_slice())?;
        if declared_transports.as_slice()
            != [
                OperationTransport::CliJson,
                OperationTransport::RustSdk,
                OperationTransport::Tauri,
            ]
        {
            return Err(ContractError::InvalidInvariant);
        }

        let definition_fingerprint = canonical_fingerprint(
            &DefinitionPreimage {
                definition_schema: &definition_schema,
                operation: &operation,
                owner: &owner,
                request_schema: &request_schema,
                result_schema: &result_schema,
                outcome_schemas: &outcome_schemas,
                required_capabilities: &required_capabilities,
                class,
                idempotency: &idempotency,
                write_set: &write_set,
                declared_transports: &declared_transports,
                deprecation: deprecation.as_ref(),
            },
            limits,
        )?;
        let value = Self {
            definition_schema,
            operation,
            owner,
            request_schema,
            result_schema,
            outcome_schemas,
            required_capabilities,
            class,
            idempotency,
            write_set,
            declared_transports,
            deprecation,
            definition_fingerprint,
        };
        canonical_json_bytes(&value, limits)?;
        Ok(value)
    }

    pub fn operation(&self) -> &ExactRef {
        &self.operation
    }

    pub fn definition_schema(&self) -> &ExactBinding {
        &self.definition_schema
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

    pub fn write_set(&self) -> &[DeclaredWrite] {
        self.write_set.as_slice()
    }

    pub fn declared_transports(&self) -> &[OperationTransport] {
        self.declared_transports.as_slice()
    }

    pub fn deprecation(&self) -> Option<&ExactBinding> {
        self.deprecation.as_ref()
    }

    pub fn definition_fingerprint(&self) -> &Fingerprint {
        &self.definition_fingerprint
    }

    pub fn binding(&self) -> ExactBinding {
        ExactBinding::new(self.operation.clone(), self.definition_fingerprint.clone())
    }
}

impl<'de> Deserialize<'de> for OperationDefinition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Raw {
            definition_schema: ExactBinding,
            operation: ExactRef,
            owner: OperationOwnerBinding,
            request_schema: ExactBinding,
            result_schema: ExactBinding,
            outcome_schemas: OperationOutcomeSchemas,
            required_capabilities: BoundedVec<ExactBinding, 128>,
            class: OperationClass,
            idempotency: OperationIdempotency,
            write_set: BoundedVec<DeclaredWrite, 128>,
            declared_transports: BoundedVec<OperationTransport, 3>,
            #[serde(default)]
            deprecation: RequiredOption<ExactBinding>,
            definition_fingerprint: Fingerprint,
        }
        let raw = Raw::deserialize(deserializer).map_err(sanitize_deserialize_error)?;
        let value = Self::new(
            raw.definition_schema,
            raw.operation,
            raw.owner,
            raw.request_schema,
            raw.result_schema,
            raw.outcome_schemas,
            raw.required_capabilities,
            raw.class,
            raw.idempotency,
            raw.write_set,
            raw.declared_transports,
            raw.deprecation.into_option().map_err(D::Error::custom)?,
            &ContractLimits::packet0(),
        )
        .map_err(D::Error::custom)?;
        if value.definition_fingerprint != raw.definition_fingerprint {
            return Err(D::Error::custom(ContractError::FingerprintMismatch));
        }
        Ok(value)
    }
}

#[derive(Serialize)]
struct DefinitionPreimage<'a> {
    definition_schema: &'a ExactBinding,
    operation: &'a ExactRef,
    owner: &'a OperationOwnerBinding,
    request_schema: &'a ExactBinding,
    result_schema: &'a ExactBinding,
    outcome_schemas: &'a OperationOutcomeSchemas,
    required_capabilities: &'a BoundedVec<ExactBinding, 128>,
    class: OperationClass,
    idempotency: &'a OperationIdempotency,
    write_set: &'a BoundedVec<DeclaredWrite, 128>,
    declared_transports: &'a BoundedVec<OperationTransport, 3>,
    deprecation: Option<&'a ExactBinding>,
}

pub fn operation_inventory() -> &'static [OperationInventoryRow] {
    static INVENTORY: OnceLock<Vec<OperationInventoryRow>> = OnceLock::new();
    INVENTORY.get_or_init(build_inventory)
}

pub fn validate_complete_operation_ref_set(
    definitions: &[ExactRef],
) -> Result<(), DefinitionSetError> {
    if definitions.len() < operation_inventory().len() {
        return Err(DefinitionSetError::MissingDefinition);
    }

    let mut actual = BTreeSet::new();
    for definition in definitions {
        if !actual.insert(definition.as_str()) {
            return Err(DefinitionSetError::DuplicateDefinition);
        }
    }

    let expected = operation_inventory()
        .iter()
        .map(|row| row.operation_ref().as_str())
        .collect::<BTreeSet<_>>();
    if actual
        .iter()
        .any(|definition| !expected.contains(definition))
    {
        return Err(DefinitionSetError::UnknownDefinition);
    }
    if actual != expected {
        return Err(DefinitionSetError::MissingDefinition);
    }
    Ok(())
}

pub fn validate_complete_definition_set(
    definitions: &[OperationDefinition],
) -> Result<(), DefinitionSetError> {
    let references = definitions
        .iter()
        .map(|definition| definition.operation().clone())
        .collect::<Vec<_>>();
    validate_complete_operation_ref_set(&references)?;

    for definition in definitions {
        let row = operation_inventory()
            .iter()
            .find(|row| row.operation_ref() == definition.operation())
            .ok_or(DefinitionSetError::UnknownDefinition)?;
        let (expected_owner, expected_version) = expected_owner_binding(row.owner());
        if definition.class() != row.class()
            || definition.owner().owner_crate() != expected_owner
            || definition.owner().owner_version().as_str() != expected_version
            || definition.binding().fingerprint() != definition.definition_fingerprint()
        {
            return Err(DefinitionSetError::DefinitionMismatch);
        }
    }
    Ok(())
}

fn expected_owner_binding(owner: OperationOwner) -> (&'static str, &'static str) {
    match owner {
        OperationOwner::Engine => ("handbook-engine", "0.2.0"),
        OperationOwner::Flow => ("handbook-flow", "0.1.1"),
        OperationOwner::Pipeline => ("handbook-pipeline", "0.1.2"),
        OperationOwner::SdkComposition => ("handbook-sdk", "0.1.0"),
        OperationOwner::Phase5 => ("future-handbook-contracts", "0.0.0"),
    }
}

fn build_inventory() -> Vec<OperationInventoryRow> {
    use AdmissionReason::{
        IncompleteClosure as IC, IndependentOwnerAbsent as IOA, OwnerDeferred as OD,
        SemanticMismatch as SM,
    };
    use OperationAvailability::{Absent as A, LivePrecursor as L, Phase5Deferred as P5};
    use OperationClass::{AppendOnly as AP, CompareAndWrite as CW, ReadOnly as RO};
    use OperationOwner::{
        Engine as E, Flow as F, Phase5 as P, Pipeline as PL, SdkComposition as S,
    };

    #[allow(clippy::type_complexity)]
    const ROWS: [(
        &str,
        OperationOwner,
        OperationAvailability,
        OperationClass,
        AdmissionReason,
    ); 62] = [
        ("capabilities.describe", S, A, RO, IOA),
        ("profile.list", E, A, RO, IC),
        ("profile.resolve", E, A, RO, IC),
        ("schema.list", E, A, RO, IC),
        ("schema.read", E, A, RO, IC),
        ("vocabulary.read", E, A, RO, IC),
        ("resolution.stack.read", E, A, RO, IC),
        ("projection.definition.read", F, A, RO, IC),
        ("artifact.kind.list", E, L, RO, IC),
        ("artifact.instance.list", E, L, RO, IC),
        ("artifact.read", E, L, RO, IC),
        ("artifact.validate", E, L, RO, IC),
        ("artifact.render", E, A, RO, IC),
        ("intake.definition.read", E, L, RO, IC),
        ("intake.coverage.evaluate", E, L, RO, IC),
        ("intake.record.append", E, L, AP, IC),
        ("record.list", E, A, RO, IC),
        ("record.read", E, A, RO, IC),
        ("artifact.candidate.validate", E, L, RO, IC),
        ("artifact.candidate.append", E, L, AP, IC),
        ("artifact.approval.append", E, A, AP, IC),
        ("artifact.candidate.promote", E, L, CW, IC),
        ("posture.resolve", E, A, RO, IC),
        ("posture.recommendation.evaluate", E, A, RO, IC),
        ("posture.recommendation.append", E, A, AP, IC),
        ("posture.recommendation.acknowledge", E, A, AP, IC),
        ("posture.transition.apply", E, L, CW, IC),
        ("projection.create", F, A, RO, IC),
        ("resolution.escalation.request.append", E, L, AP, IC),
        ("resolution.escalation.disposition.append", E, L, AP, IC),
        ("memory.promotion.request.append", E, L, AP, IC),
        ("memory.promotion.disposition.append", E, L, CW, IC),
        ("snapshot.capture", S, A, AP, IOA),
        ("snapshot.read", E, A, RO, IC),
        ("snapshot.delta", E, A, RO, IC),
        ("snapshot.project", F, A, RO, IC),
        ("snapshot.verify_current", S, A, RO, IOA),
        ("snapshot.resolve_applicable", S, A, RO, IOA),
        ("repository.setup.plan", S, A, RO, IOA),
        ("repository.setup.apply", S, L, CW, IOA),
        ("repository.doctor", S, L, RO, IOA),
        ("flow.resolve", F, L, RO, IC),
        ("pipeline.catalog.list", PL, L, RO, IC),
        ("pipeline.catalog.read", PL, L, RO, IC),
        ("pipeline.route.resolve", PL, L, RO, SM),
        ("pipeline.compile", PL, L, RO, IC),
        ("pipeline.capture.plan", PL, L, RO, IC),
        ("pipeline.capture.apply", PL, L, CW, IC),
        ("pipeline.handoff.emit", PL, L, AP, IC),
        ("pipeline.state.apply", PL, L, CW, IC),
        ("contract.definition.list", P, P5, RO, OD),
        ("contract.definition.read", P, P5, RO, OD),
        ("contract.definition.append", P, P5, AP, OD),
        ("contract.lifecycle.transition", P, P5, CW, OD),
        ("contract.evidence.list", P, P5, RO, OD),
        ("contract.evidence.read", P, P5, RO, OD),
        ("contract.evidence.append", P, P5, AP, OD),
        ("contract.verdict.evaluate", P, P5, RO, OD),
        ("contract.gate.evaluate", P, P5, RO, OD),
        ("dock.manifest.list", P, P5, RO, OD),
        ("dock.manifest.read", P, P5, RO, OD),
        ("dock.run", P, P5, AP, OD),
    ];

    ROWS.into_iter()
        .enumerate()
        .map(
            |(index, (operation_id, owner, availability, class, admission_reason))| {
                let is_posture_apply = operation_id == "posture.transition.apply";
                OperationInventoryRow {
                    proof_ordinal: (index + 1) as u16,
                    operation_id,
                    operation_ref: ExactRef::parse(&format!("{operation_id}@1.0.0"))
                        .unwrap_or_else(|_| operation_inventory_invariant()),
                    owner,
                    availability,
                    class,
                    admission_reason: if is_posture_apply {
                        AdmissionReason::Admitted
                    } else {
                        admission_reason
                    },
                    current_transports: if is_posture_apply {
                        &[OperationTransport::RustSdk]
                    } else {
                        &[]
                    },
                    discovery: if is_posture_apply {
                        DiscoveryState::Discoverable
                    } else {
                        DiscoveryState::Omit
                    },
                }
            },
        )
        .collect()
}

fn operation_inventory_invariant() -> ! {
    panic!("Packet-2 operation inventory contains an invalid exact ref")
}

fn require_exact_ref(binding: &ExactBinding, expected: &str) -> Result<(), ContractError> {
    if binding.reference().as_str() == expected {
        Ok(())
    } else {
        Err(ContractError::InvalidInvariant)
    }
}

fn require_operation_schema(
    operation: &ExactRef,
    schema: &ExactBinding,
    suffix: &str,
) -> Result<(), ContractError> {
    let identity = operation.identity().replace('.', "-");
    let expected = format!(
        "handbook.operation.{identity}-{suffix}@{}",
        operation.version()
    );
    require_exact_ref(schema, &expected)
}

fn require_key_pointer(pointer: &JsonPointer) -> Result<(), ContractError> {
    if pointer.as_str() == "/body/idempotency_key" {
        Ok(())
    } else {
        Err(ContractError::InvalidInvariant)
    }
}

fn validate_definition_behavior(
    class: OperationClass,
    idempotency: &OperationIdempotency,
    write_set: &[DeclaredWrite],
) -> Result<(), ContractError> {
    let valid = match (class, idempotency) {
        (OperationClass::ReadOnly, OperationIdempotency::Safe) => write_set.is_empty(),
        (OperationClass::AppendOnly, OperationIdempotency::IdempotencyKeyRequired { .. }) => {
            !write_set.is_empty()
        }
        (OperationClass::CompareAndWrite, OperationIdempotency::CompareAndWriteRequired { .. }) => {
            !write_set.is_empty()
        }
        _ => false,
    };
    if valid {
        Ok(())
    } else {
        Err(ContractError::InvalidInvariant)
    }
}

fn validate_write_set(write_set: &[DeclaredWrite]) -> Result<(), ContractError> {
    if let Some(first) = write_set.first() {
        if write_set
            .iter()
            .any(|item| item.atomic_group() != first.atomic_group())
        {
            return Err(ContractError::InvalidInvariant);
        }
    }
    for pair in write_set.windows(2) {
        let left_identity = (
            pair[0].atomic_group(),
            pair[0].record_kind(),
            pair[0].authority_class(),
        );
        let right_identity = (
            pair[1].atomic_group(),
            pair[1].record_kind(),
            pair[1].authority_class(),
        );
        if left_identity == right_identity {
            return Err(ContractError::DuplicateMember);
        }
        let ordering = (
            left_identity,
            pair[0].condition_schema().reference().as_str(),
            pair[0].condition_schema().fingerprint().as_str(),
        )
            .cmp(&(
                right_identity,
                pair[1].condition_schema().reference().as_str(),
                pair[1].condition_schema().fingerprint().as_str(),
            ));
        if ordering != Ordering::Less {
            return Err(ContractError::NonCanonicalOrder);
        }
    }
    Ok(())
}

fn validate_binding_order(bindings: &[ExactBinding]) -> Result<(), ContractError> {
    for pair in bindings.windows(2) {
        match (pair[0].reference().as_str(), pair[0].fingerprint().as_str())
            .cmp(&(pair[1].reference().as_str(), pair[1].fingerprint().as_str()))
        {
            Ordering::Less => {}
            Ordering::Equal => return Err(ContractError::DuplicateMember),
            Ordering::Greater => return Err(ContractError::NonCanonicalOrder),
        }
    }
    Ok(())
}

fn validate_strict_order<T>(values: &[T], key: impl Fn(&T) -> &str) -> Result<(), ContractError> {
    for pair in values.windows(2) {
        match key(&pair[0]).cmp(key(&pair[1])) {
            Ordering::Less => {}
            Ordering::Equal => return Err(ContractError::DuplicateMember),
            Ordering::Greater => return Err(ContractError::NonCanonicalOrder),
        }
    }
    Ok(())
}

fn valid_json_pointer(value: &str) -> bool {
    if !value.starts_with('/') || value.len() > 255 {
        return false;
    }
    value.split('/').skip(1).all(|token| {
        !token.is_empty()
            && token.as_bytes().iter().enumerate().all(|(index, byte)| {
                *byte != b'~'
                    || token
                        .as_bytes()
                        .get(index + 1)
                        .is_some_and(|next| matches!(next, b'0' | b'1'))
            })
            && !token.as_bytes().windows(2).any(|pair| pair == b"~~")
    })
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

fn sanitize_deserialize_error<E: serde::de::Error>(error: E) -> E {
    E::custom(classify_contract_error_message(&error.to_string()))
}
