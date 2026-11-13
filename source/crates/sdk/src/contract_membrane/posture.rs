use super::{
    admit_operation_definition, canonical_json_bytes, operation_inventory, parse_canonical_json,
    sha256_fingerprint, BoundedString, BoundedVec, CapabilityEntry, ContractError, ContractLimits,
    ContractValue, CorrelationRef, CurrentTransportEvidence, CurrentTransportEvidenceSealed,
    DeclaredWrite, ExactBinding, ExactRef, Fingerprint, IdempotencyKeyConsumer,
    IdempotencyKeyConsumerSealed, IdempotencyStage, IdempotencyState, JsonPointer,
    OperationAdmissionResult, OperationClass, OperationDefinition, OperationIdempotency,
    OperationOutcomeSchemas, OperationOwnerBinding, OperationRequestV1, OperationResponseV1,
    OperationSchemaValidator, OperationTransport, OwnerAdmissionToken, Problem, ProblemBinding,
    ProblemCategory, RecheckCondition, RequestId, RetryDirective, SafeU64, SemVer, TerminalFields,
    TerminalStatus, UtcTimestamp, WriteReceipt,
};
use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize};

pub(crate) const OPERATION_REF: &str = "posture.transition.apply@1.0.0";
pub(crate) const REQUEST_SCHEMA_REF: &str =
    "handbook.operation.posture-transition-apply-request@1.0.0";
pub(crate) const RESULT_SCHEMA_REF: &str =
    "handbook.operation.posture-transition-apply-result@1.0.0";
pub(crate) const CANONICAL_CONDITION_SCHEMA_REF: &str =
    "handbook.posture-write-condition.canonical-truth@1.0.0";
pub(crate) const SEMANTIC_CONDITION_SCHEMA_REF: &str =
    "handbook.posture-write-condition.semantic-record@1.0.0";
pub(crate) const UNSUPPORTED_RECHECK_SCHEMA_REF: &str =
    "handbook.posture-recheck.unsupported-platform@1.0.0";
pub(crate) const RETENTION_UNTIL_UTC: &str = "9999-12-31T23:59:59Z";

const CANONICAL_REFERENCE: &str = ".handbook/project/charter.yaml";
const MAX_IDEMPOTENCY_KEY_BYTES: usize = 256;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct PostureReferenceDtoV1 {
    reference: BoundedString<255>,
    fingerprint: Fingerprint,
}

impl PostureReferenceDtoV1 {
    pub fn new(reference: String, fingerprint: String) -> Result<Self, ContractError> {
        Ok(Self {
            reference: BoundedString::new(reference)?,
            fingerprint: Fingerprint::parse(&fingerprint)?,
        })
    }

    pub fn reference(&self) -> &str {
        self.reference.as_str()
    }

    pub fn fingerprint(&self) -> &Fingerprint {
        &self.fingerprint
    }
}

impl<'de> Deserialize<'de> for PostureReferenceDtoV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Raw {
            reference: String,
            fingerprint: String,
        }
        let raw = Raw::deserialize(deserializer).map_err(sanitize_deserialize_error)?;
        Self::new(raw.reference, raw.fingerprint).map_err(D::Error::custom)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct PostureCanonicalBindingDtoV1 {
    reference: BoundedString<255>,
    fingerprint: Fingerprint,
    document_sha256: Fingerprint,
    byte_length: SafeU64,
}

impl PostureCanonicalBindingDtoV1 {
    pub fn new(
        reference: String,
        fingerprint: String,
        document_sha256: String,
        byte_length: u64,
    ) -> Result<Self, ContractError> {
        let reference = BoundedString::new(reference)?;
        let fingerprint = Fingerprint::parse(&fingerprint)?;
        let document_sha256 = Fingerprint::parse(&document_sha256)?;
        let byte_length = SafeU64::new(byte_length)?;
        if reference.as_str() != CANONICAL_REFERENCE
            || fingerprint != document_sha256
            || byte_length.get() == 0
        {
            return Err(ContractError::InvalidInvariant);
        }
        Ok(Self {
            reference,
            fingerprint,
            document_sha256,
            byte_length,
        })
    }

    pub fn reference(&self) -> &str {
        self.reference.as_str()
    }

    pub fn fingerprint(&self) -> &Fingerprint {
        &self.fingerprint
    }

    pub fn document_sha256(&self) -> &Fingerprint {
        &self.document_sha256
    }

    pub fn byte_length(&self) -> SafeU64 {
        self.byte_length
    }
}

impl<'de> Deserialize<'de> for PostureCanonicalBindingDtoV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Raw {
            reference: String,
            fingerprint: String,
            document_sha256: String,
            byte_length: u64,
        }
        let raw = Raw::deserialize(deserializer).map_err(sanitize_deserialize_error)?;
        Self::new(
            raw.reference,
            raw.fingerprint,
            raw.document_sha256,
            raw.byte_length,
        )
        .map_err(D::Error::custom)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct PostureDimensionChangeDtoV1 {
    dimension_id: BoundedString<255>,
    dimension_index: u8,
    authority_path: JsonPointer,
    expected_stored_value: Option<u8>,
    expected_effective_level: u8,
    proposed_effective_level: u8,
}

impl PostureDimensionChangeDtoV1 {
    pub fn new(
        dimension_id: String,
        dimension_index: u8,
        authority_path: String,
        expected_stored_value: Option<u8>,
        expected_effective_level: u8,
        proposed_effective_level: u8,
    ) -> Result<Self, ContractError> {
        Ok(Self {
            dimension_id: BoundedString::new(dimension_id)?,
            dimension_index,
            authority_path: JsonPointer::parse(&authority_path)?,
            expected_stored_value,
            expected_effective_level,
            proposed_effective_level,
        })
    }

    pub fn dimension_id(&self) -> &str {
        self.dimension_id.as_str()
    }

    pub fn dimension_index(&self) -> u8 {
        self.dimension_index
    }

    pub fn authority_path(&self) -> &str {
        self.authority_path.as_str()
    }

    pub fn expected_stored_value(&self) -> Option<u8> {
        self.expected_stored_value
    }

    pub fn expected_effective_level(&self) -> u8 {
        self.expected_effective_level
    }

    pub fn proposed_effective_level(&self) -> u8 {
        self.proposed_effective_level
    }
}

impl<'de> Deserialize<'de> for PostureDimensionChangeDtoV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Raw {
            dimension_id: String,
            dimension_index: u8,
            authority_path: String,
            #[serde(default)]
            expected_stored_value: RequiredOption<u8>,
            expected_effective_level: u8,
            proposed_effective_level: u8,
        }
        let raw = Raw::deserialize(deserializer).map_err(sanitize_deserialize_error)?;
        Self::new(
            raw.dimension_id,
            raw.dimension_index,
            raw.authority_path,
            raw.expected_stored_value
                .into_option()
                .map_err(D::Error::custom)?,
            raw.expected_effective_level,
            raw.proposed_effective_level,
        )
        .map_err(D::Error::custom)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct PostureTransitionApplyRequestDataV1 {
    repository_identity_fingerprint: Fingerprint,
    expected_canonical: PostureCanonicalBindingDtoV1,
    recommendation: PostureReferenceDtoV1,
    reassessment_coverage_ids: BoundedVec<BoundedString<256>, 64>,
    change: PostureDimensionChangeDtoV1,
    effective_at_utc: UtcTimestamp,
}

impl PostureTransitionApplyRequestDataV1 {
    pub fn new(
        repository_identity_fingerprint: String,
        expected_canonical: PostureCanonicalBindingDtoV1,
        recommendation: PostureReferenceDtoV1,
        reassessment_coverage_ids: Vec<String>,
        change: PostureDimensionChangeDtoV1,
        effective_at_utc: String,
    ) -> Result<Self, ContractError> {
        let reassessment_coverage_ids = BoundedVec::new(
            reassessment_coverage_ids
                .into_iter()
                .map(BoundedString::new)
                .collect::<Result<Vec<_>, _>>()?,
        )?;
        if reassessment_coverage_ids.as_slice().is_empty()
            || reassessment_coverage_ids
                .as_slice()
                .iter()
                .any(|id| !id.as_str().bytes().all(|byte| byte.is_ascii_graphic()))
            || reassessment_coverage_ids
                .as_slice()
                .windows(2)
                .any(|pair| pair[0].as_str() >= pair[1].as_str())
        {
            return Err(ContractError::InvalidInvariant);
        }
        Ok(Self {
            repository_identity_fingerprint: Fingerprint::parse(&repository_identity_fingerprint)?,
            expected_canonical,
            recommendation,
            reassessment_coverage_ids,
            change,
            effective_at_utc: UtcTimestamp::parse(&effective_at_utc)?,
        })
    }

    pub fn repository_identity_fingerprint(&self) -> &Fingerprint {
        &self.repository_identity_fingerprint
    }

    pub fn expected_canonical(&self) -> &PostureCanonicalBindingDtoV1 {
        &self.expected_canonical
    }

    pub fn recommendation(&self) -> &PostureReferenceDtoV1 {
        &self.recommendation
    }

    pub fn reassessment_coverage_ids(&self) -> &[BoundedString<256>] {
        self.reassessment_coverage_ids.as_slice()
    }

    pub fn change(&self) -> &PostureDimensionChangeDtoV1 {
        &self.change
    }

    pub fn effective_at_utc(&self) -> &UtcTimestamp {
        &self.effective_at_utc
    }
}

impl<'de> Deserialize<'de> for PostureTransitionApplyRequestDataV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Raw {
            repository_identity_fingerprint: String,
            expected_canonical: PostureCanonicalBindingDtoV1,
            recommendation: PostureReferenceDtoV1,
            reassessment_coverage_ids: Vec<String>,
            change: PostureDimensionChangeDtoV1,
            effective_at_utc: String,
        }
        let raw = Raw::deserialize(deserializer).map_err(sanitize_deserialize_error)?;
        Self::new(
            raw.repository_identity_fingerprint,
            raw.expected_canonical,
            raw.recommendation,
            raw.reassessment_coverage_ids,
            raw.change,
            raw.effective_at_utc,
        )
        .map_err(D::Error::custom)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct PostureTransitionApplyResultDataV1 {
    idempotency_key_fingerprint: Fingerprint,
    repository_identity_fingerprint: Fingerprint,
    prior_canonical: PostureCanonicalBindingDtoV1,
    resulting_canonical: PostureCanonicalBindingDtoV1,
    posture_transition: PostureReferenceDtoV1,
    lifecycle_transition: PostureReferenceDtoV1,
    resulting_kernel: PostureReferenceDtoV1,
}

impl<'de> Deserialize<'de> for PostureTransitionApplyResultDataV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Raw {
            idempotency_key_fingerprint: Fingerprint,
            repository_identity_fingerprint: Fingerprint,
            prior_canonical: PostureCanonicalBindingDtoV1,
            resulting_canonical: PostureCanonicalBindingDtoV1,
            posture_transition: PostureReferenceDtoV1,
            lifecycle_transition: PostureReferenceDtoV1,
            resulting_kernel: PostureReferenceDtoV1,
        }
        let raw = Raw::deserialize(deserializer).map_err(sanitize_deserialize_error)?;
        Ok(Self::new(
            raw.idempotency_key_fingerprint,
            raw.repository_identity_fingerprint,
            raw.prior_canonical,
            raw.resulting_canonical,
            raw.posture_transition,
            raw.lifecycle_transition,
            raw.resulting_kernel,
        ))
    }
}

impl PostureTransitionApplyResultDataV1 {
    pub(crate) fn new(
        idempotency_key_fingerprint: Fingerprint,
        repository_identity_fingerprint: Fingerprint,
        prior_canonical: PostureCanonicalBindingDtoV1,
        resulting_canonical: PostureCanonicalBindingDtoV1,
        posture_transition: PostureReferenceDtoV1,
        lifecycle_transition: PostureReferenceDtoV1,
        resulting_kernel: PostureReferenceDtoV1,
    ) -> Self {
        Self {
            idempotency_key_fingerprint,
            repository_identity_fingerprint,
            prior_canonical,
            resulting_canonical,
            posture_transition,
            lifecycle_transition,
            resulting_kernel,
        }
    }

    pub fn idempotency_key_fingerprint(&self) -> &Fingerprint {
        &self.idempotency_key_fingerprint
    }

    pub fn repository_identity_fingerprint(&self) -> &Fingerprint {
        &self.repository_identity_fingerprint
    }

    pub fn prior_canonical(&self) -> &PostureCanonicalBindingDtoV1 {
        &self.prior_canonical
    }

    pub fn resulting_canonical(&self) -> &PostureCanonicalBindingDtoV1 {
        &self.resulting_canonical
    }

    pub fn posture_transition(&self) -> &PostureReferenceDtoV1 {
        &self.posture_transition
    }

    pub fn lifecycle_transition(&self) -> &PostureReferenceDtoV1 {
        &self.lifecycle_transition
    }

    pub fn resulting_kernel(&self) -> &PostureReferenceDtoV1 {
        &self.resulting_kernel
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub enum PostureOwnerVariantV1 {
    UnsupportedPlatform,
    InvalidIdempotencyKey,
    RepositoryIdentityMismatch,
    InvalidFixedDimension,
    StaleCanonical,
    MissingLifecycleAuthority,
    LifecycleNotCurrent,
    InvalidSemanticInput,
    AuthorityConflict,
    UnsafeFilesystem,
    IntegrityOrDurability,
    Io,
}

impl<'de> Deserialize<'de> for PostureOwnerVariantV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)
            .map_err(|_| D::Error::custom(ContractError::InvalidJson))?;
        match value.as_str() {
            "UnsupportedPlatform" => Ok(Self::UnsupportedPlatform),
            "InvalidIdempotencyKey" => Ok(Self::InvalidIdempotencyKey),
            "RepositoryIdentityMismatch" => Ok(Self::RepositoryIdentityMismatch),
            "InvalidFixedDimension" => Ok(Self::InvalidFixedDimension),
            "StaleCanonical" => Ok(Self::StaleCanonical),
            "MissingLifecycleAuthority" => Ok(Self::MissingLifecycleAuthority),
            "LifecycleNotCurrent" => Ok(Self::LifecycleNotCurrent),
            "InvalidSemanticInput" => Ok(Self::InvalidSemanticInput),
            "AuthorityConflict" => Ok(Self::AuthorityConflict),
            "UnsafeFilesystem" => Ok(Self::UnsafeFilesystem),
            "IntegrityOrDurability" => Ok(Self::IntegrityOrDurability),
            "Io" => Ok(Self::Io),
            _ => Err(D::Error::custom(ContractError::InvalidJson)),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct PostureOwnerVariantDetailsV1 {
    owner_variant: PostureOwnerVariantV1,
}

impl PostureOwnerVariantDetailsV1 {
    pub(crate) fn new(owner_variant: PostureOwnerVariantV1) -> Self {
        Self { owner_variant }
    }

    pub fn owner_variant(&self) -> PostureOwnerVariantV1 {
        self.owner_variant
    }
}

impl<'de> Deserialize<'de> for PostureOwnerVariantDetailsV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Raw {
            owner_variant: PostureOwnerVariantV1,
        }
        let raw = Raw::deserialize(deserializer).map_err(sanitize_deserialize_error)?;
        Ok(Self::new(raw.owner_variant))
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct PostureCanonicalWriteConditionV1 {
    prior_canonical: PostureCanonicalBindingDtoV1,
    resulting_canonical: PostureCanonicalBindingDtoV1,
}

impl PostureCanonicalWriteConditionV1 {
    pub(crate) fn new(
        prior_canonical: PostureCanonicalBindingDtoV1,
        resulting_canonical: PostureCanonicalBindingDtoV1,
    ) -> Self {
        Self {
            prior_canonical,
            resulting_canonical,
        }
    }

    pub fn prior_canonical(&self) -> &PostureCanonicalBindingDtoV1 {
        &self.prior_canonical
    }

    pub fn resulting_canonical(&self) -> &PostureCanonicalBindingDtoV1 {
        &self.resulting_canonical
    }
}

impl<'de> Deserialize<'de> for PostureCanonicalWriteConditionV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Raw {
            prior_canonical: PostureCanonicalBindingDtoV1,
            resulting_canonical: PostureCanonicalBindingDtoV1,
        }
        let raw = Raw::deserialize(deserializer).map_err(sanitize_deserialize_error)?;
        Ok(Self::new(raw.prior_canonical, raw.resulting_canonical))
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct PostureSemanticWriteConditionV1 {
    lifecycle_transition: PostureReferenceDtoV1,
    posture_transition: PostureReferenceDtoV1,
    resulting_kernel: PostureReferenceDtoV1,
}

impl<'de> Deserialize<'de> for PostureSemanticWriteConditionV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Raw {
            lifecycle_transition: PostureReferenceDtoV1,
            posture_transition: PostureReferenceDtoV1,
            resulting_kernel: PostureReferenceDtoV1,
        }
        let raw = Raw::deserialize(deserializer).map_err(sanitize_deserialize_error)?;
        Ok(Self::new(
            raw.lifecycle_transition,
            raw.posture_transition,
            raw.resulting_kernel,
        ))
    }
}

impl PostureSemanticWriteConditionV1 {
    pub(crate) fn new(
        lifecycle_transition: PostureReferenceDtoV1,
        posture_transition: PostureReferenceDtoV1,
        resulting_kernel: PostureReferenceDtoV1,
    ) -> Self {
        Self {
            lifecycle_transition,
            posture_transition,
            resulting_kernel,
        }
    }

    pub fn lifecycle_transition(&self) -> &PostureReferenceDtoV1 {
        &self.lifecycle_transition
    }

    pub fn posture_transition(&self) -> &PostureReferenceDtoV1 {
        &self.posture_transition
    }

    pub fn resulting_kernel(&self) -> &PostureReferenceDtoV1 {
        &self.resulting_kernel
    }
}

#[derive(Serialize)]
struct PostureTransitionApplyWireBodyV1<'a> {
    change: &'a PostureDimensionChangeDtoV1,
    effective_at_utc: &'a UtcTimestamp,
    expected_canonical: &'a PostureCanonicalBindingDtoV1,
    idempotency_key: &'a str,
    reassessment_coverage_ids: &'a BoundedVec<BoundedString<256>, 64>,
    recommendation: &'a PostureReferenceDtoV1,
    repository_identity_fingerprint: &'a Fingerprint,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct RawPostureTransitionApplyWireBodyV1 {
    change: PostureDimensionChangeDtoV1,
    effective_at_utc: String,
    expected_canonical: PostureCanonicalBindingDtoV1,
    idempotency_key: String,
    reassessment_coverage_ids: Vec<String>,
    recommendation: PostureReferenceDtoV1,
    repository_identity_fingerprint: String,
}

impl RawPostureTransitionApplyWireBodyV1 {
    fn validate(self) -> Result<PostureTransitionApplyRequestDataV1, ContractError> {
        if self.idempotency_key.is_empty()
            || self.idempotency_key.len() > MAX_IDEMPOTENCY_KEY_BYTES
            || !self
                .idempotency_key
                .bytes()
                .all(|byte| byte.is_ascii_graphic())
        {
            return Err(ContractError::InvalidString);
        }
        PostureTransitionApplyRequestDataV1::new(
            self.repository_identity_fingerprint,
            self.expected_canonical,
            self.recommendation,
            self.reassessment_coverage_ids,
            self.change,
            self.effective_at_utc,
        )
    }
}

pub fn posture_transition_apply_definition() -> Result<OperationDefinition, ContractError> {
    let limits = ContractLimits::packet0();
    let owner = handbook_engine::operation_admission::operation_owner_version_evidence();
    let expected_basis = [
        "/body/change",
        "/body/effective_at_utc",
        "/body/expected_canonical",
        "/body/reassessment_coverage_ids",
        "/body/recommendation",
        "/body/repository_identity_fingerprint",
    ]
    .into_iter()
    .map(JsonPointer::parse)
    .collect::<Result<Vec<_>, _>>()?;
    let atomic_group = || BoundedString::new("posture_transition_apply".to_owned());
    OperationDefinition::new(
        schema_binding("handbook.operation-definition@1.0.0")?,
        ExactRef::parse(OPERATION_REF)?,
        OperationOwnerBinding::new(
            BoundedString::new(owner.owner_crate.to_owned())?,
            SemVer::parse(owner.owner_version)?,
            ExactBinding::new(
                ExactRef::parse(owner.owner_source_ref)?,
                sha256_fingerprint(owner.source_bytes),
            ),
        )?,
        schema_binding(REQUEST_SCHEMA_REF)?,
        schema_binding(RESULT_SCHEMA_REF)?,
        OperationOutcomeSchemas::new(
            schema_binding("handbook.operation-blocker@1.0.0")?,
            schema_binding("handbook.operation-refusal@1.0.0")?,
            schema_binding("handbook.operation-error@1.0.0")?,
        )?,
        BoundedVec::new(Vec::new())?,
        OperationClass::CompareAndWrite,
        OperationIdempotency::compare_and_write(
            JsonPointer::parse("/body/idempotency_key")?,
            BoundedVec::new(expected_basis)?,
        )?,
        BoundedVec::new(vec![
            DeclaredWrite::new(
                BoundedString::new("constitutional_root".to_owned())?,
                BoundedString::new("canonical_truth".to_owned())?,
                atomic_group()?,
                schema_binding(CANONICAL_CONDITION_SCHEMA_REF)?,
            )?,
            DeclaredWrite::new(
                BoundedString::new("posture_transition".to_owned())?,
                BoundedString::new("semantic_record".to_owned())?,
                atomic_group()?,
                schema_binding(SEMANTIC_CONDITION_SCHEMA_REF)?,
            )?,
        ])?,
        BoundedVec::new(vec![
            OperationTransport::CliJson,
            OperationTransport::RustSdk,
            OperationTransport::Tauri,
        ])?,
        None,
        &limits,
    )
}

pub fn posture_transition_apply_request(
    request_id: Option<&str>,
    idempotency_key: &str,
    data: PostureTransitionApplyRequestDataV1,
) -> Result<OperationRequestV1, ContractError> {
    let limits = ContractLimits::packet0();
    let definition = posture_transition_apply_definition()?;
    let wire = PostureTransitionApplyWireBodyV1 {
        change: &data.change,
        effective_at_utc: &data.effective_at_utc,
        expected_canonical: &data.expected_canonical,
        idempotency_key,
        reassessment_coverage_ids: &data.reassessment_coverage_ids,
        recommendation: &data.recommendation,
        repository_identity_fingerprint: &data.repository_identity_fingerprint,
    };
    let body =
        parse_canonical_json::<ContractValue>(&canonical_json_bytes(&wire, &limits)?, &limits)?;
    OperationRequestV1::new(
        schema_binding("handbook.operation-request@1.0.0")?,
        request_id.map(RequestId::parse).transpose()?,
        &definition,
        body,
        &PostureSchemaValidator,
        &limits,
    )
}

pub fn admit_posture_transition_apply_request(
    bytes: &[u8],
) -> Result<OperationRequestV1, ContractError> {
    OperationRequestV1::parse_and_admit(
        bytes,
        &posture_transition_apply_definition()?,
        &PostureSchemaValidator,
        &ContractLimits::packet0(),
    )
}

pub(crate) fn request_data_from_body(
    body: &ContractValue,
) -> Result<PostureTransitionApplyRequestDataV1, ContractError> {
    parse_canonical_json(
        &canonical_json_bytes(body, &ContractLimits::packet0())?,
        &ContractLimits::packet0(),
    )
}

pub(crate) fn apply_posture_transition_operation(
    facade: &handbook_engine::PostureTransitionEngineFacadeV1,
    request: &OperationRequestV1,
) -> Result<OperationResponseV1, ContractError> {
    let limits = ContractLimits::packet0();
    let definition = posture_transition_apply_definition()?;
    if request.operation() != &definition.binding()
        || request.body_schema() != definition.request_schema()
        || !inventory_row_is_posture(request.operation().reference())
    {
        return Err(ContractError::InvalidInvariant);
    }
    match admit_operation_definition(&definition)? {
        OperationAdmissionResult::Admitted(token)
            if token.operation_ref() == definition.operation().as_str()
                && token.definition_fingerprint()
                    == definition.definition_fingerprint().as_str() => {}
        _ => return Err(ContractError::InvalidInvariant),
    }

    let data = request_data_from_body(request.body())?;
    let mut bridge = PostureOwnerBridge {
        facade,
        data: &data,
        fixed_pointer_observed: false,
        result: None,
    };
    let owner_key_fingerprint = request
        .with_owner_idempotency_key(&mut bridge)
        .ok_or(ContractError::InvalidInvariant)?;
    if !bridge.fixed_pointer_observed {
        return Err(ContractError::InvalidInvariant);
    }
    let owner_result = bridge.result.ok_or(ContractError::InvalidInvariant)?;
    project_owner_result(
        request,
        &definition,
        owner_key_fingerprint,
        owner_result,
        &limits,
    )
}

pub(crate) fn current_posture_capability() -> Result<CapabilityEntry, ContractError> {
    let definition = posture_transition_apply_definition()?;
    let owner = match admit_operation_definition(&definition)? {
        OperationAdmissionResult::Admitted(owner) => owner,
        OperationAdmissionResult::Omitted(_) => return Err(ContractError::InvalidInvariant),
    };
    let evidence = PostureRustTransportEvidence { owner };
    CapabilityEntry::new(&definition, &evidence, &ContractLimits::packet0())
}

struct PostureRustTransportEvidence {
    owner: OwnerAdmissionToken,
}

impl CurrentTransportEvidenceSealed for PostureRustTransportEvidence {}

impl CurrentTransportEvidence for PostureRustTransportEvidence {
    fn owner_admission(&self) -> &OwnerAdmissionToken {
        &self.owner
    }

    fn current_transports(&self) -> &[OperationTransport] {
        &[OperationTransport::RustSdk, OperationTransport::Tauri]
    }
}

struct PostureOwnerBridge<'a> {
    facade: &'a handbook_engine::PostureTransitionEngineFacadeV1,
    data: &'a PostureTransitionApplyRequestDataV1,
    fixed_pointer_observed: bool,
    result: Option<handbook_engine::PostureTransitionApplyResultV1>,
}

impl IdempotencyKeyConsumerSealed for PostureOwnerBridge<'_> {
    fn consume_owner_idempotency_key(&mut self, pointer: &str, bytes: &[u8]) -> Fingerprint {
        self.fixed_pointer_observed = pointer == "/body/idempotency_key";
        let raw_key = std::str::from_utf8(bytes)
            .expect("admitted raw key remains visible ASCII")
            .to_owned();
        self.result = Some(
            self.facade.apply_posture_transition(
                handbook_engine::PostureTransitionApplyRequestV1 {
                    idempotency_key: raw_key,
                    repository_identity_fingerprint: self
                        .data
                        .repository_identity_fingerprint()
                        .as_str()
                        .to_owned(),
                    expected_canonical: engine_canonical(self.data.expected_canonical()),
                    recommendation: engine_reference(self.data.recommendation()),
                    reassessment_coverage_ids: self
                        .data
                        .reassessment_coverage_ids()
                        .iter()
                        .map(|id| id.as_str().to_owned())
                        .collect(),
                    change: handbook_engine::PostureDimensionChangeRequestV1 {
                        dimension_id: self.data.change().dimension_id().to_owned(),
                        dimension_index: self.data.change().dimension_index(),
                        authority_path: self.data.change().authority_path().to_owned(),
                        expected_stored_value: self.data.change().expected_stored_value(),
                        expected_effective_level: self.data.change().expected_effective_level(),
                        proposed_effective_level: self.data.change().proposed_effective_level(),
                    },
                    effective_at_utc: self.data.effective_at_utc().as_str().to_owned(),
                },
            ),
        );
        sha256_fingerprint(bytes)
    }
}

impl IdempotencyKeyConsumer for PostureOwnerBridge<'_> {}

fn engine_reference(value: &PostureReferenceDtoV1) -> handbook_engine::PostureReferenceV1 {
    handbook_engine::PostureReferenceV1 {
        reference: value.reference().to_owned(),
        fingerprint: value.fingerprint().as_str().to_owned(),
    }
}

fn engine_canonical(
    value: &PostureCanonicalBindingDtoV1,
) -> handbook_engine::PostureCanonicalBindingV1 {
    handbook_engine::PostureCanonicalBindingV1 {
        reference: value.reference().to_owned(),
        fingerprint: value.fingerprint().as_str().to_owned(),
        document_sha256: value.document_sha256().as_str().to_owned(),
        byte_length: value.byte_length().get(),
    }
}

fn project_owner_result(
    request: &OperationRequestV1,
    definition: &OperationDefinition,
    owner_key_fingerprint: Fingerprint,
    result: handbook_engine::PostureTransitionApplyResultV1,
    limits: &ContractLimits,
) -> Result<OperationResponseV1, ContractError> {
    use handbook_engine::{
        PostureTransitionApplyResultV1 as ResultV1, PostureTransitionBlockedCodeV1 as Blocked,
        PostureTransitionErrorCodeV1 as Error, PostureTransitionRefusalCodeV1 as Refusal,
    };

    match result {
        ResultV1::Applied(receipt) => project_success(
            request,
            definition,
            owner_key_fingerprint,
            receipt,
            false,
            limits,
        ),
        ResultV1::Replayed(receipt) => project_success(
            request,
            definition,
            owner_key_fingerprint,
            receipt,
            true,
            limits,
        ),
        ResultV1::Blocked(Blocked::UnsupportedPlatform) => project_problem(
            request,
            definition,
            ProblemProjection::blocked(PostureOwnerVariantV1::UnsupportedPlatform),
            limits,
        ),
        ResultV1::Refused(code) => {
            let projection = match code {
                Refusal::InvalidIdempotencyKey => ProblemProjection::refused(
                    PostureOwnerVariantV1::InvalidIdempotencyKey,
                    IdempotencyStage::RequestValidation,
                    ProblemCategory::Schema,
                    RetryDirective::AfterRequestChange,
                ),
                Refusal::RepositoryIdentityMismatch => ProblemProjection::refused(
                    PostureOwnerVariantV1::RepositoryIdentityMismatch,
                    IdempotencyStage::AuthorityValidation,
                    ProblemCategory::Authority,
                    RetryDirective::AfterRequestChange,
                ),
                Refusal::InvalidFixedDimension => ProblemProjection::refused(
                    PostureOwnerVariantV1::InvalidFixedDimension,
                    IdempotencyStage::PreconditionValidation,
                    ProblemCategory::Precondition,
                    RetryDirective::AfterRequestChange,
                ),
                Refusal::StaleCanonical => ProblemProjection::refused(
                    PostureOwnerVariantV1::StaleCanonical,
                    IdempotencyStage::PreconditionValidation,
                    ProblemCategory::Precondition,
                    RetryDirective::AfterRequestChange,
                ),
                Refusal::MissingLifecycleAuthority => ProblemProjection::refused(
                    PostureOwnerVariantV1::MissingLifecycleAuthority,
                    IdempotencyStage::AuthorityValidation,
                    ProblemCategory::Authority,
                    RetryDirective::AfterAuthorityChange,
                ),
                Refusal::LifecycleNotCurrent => ProblemProjection::refused(
                    PostureOwnerVariantV1::LifecycleNotCurrent,
                    IdempotencyStage::PreconditionValidation,
                    ProblemCategory::Precondition,
                    RetryDirective::AfterAuthorityChange,
                ),
                Refusal::InvalidSemanticInput => ProblemProjection::refused(
                    PostureOwnerVariantV1::InvalidSemanticInput,
                    IdempotencyStage::PreconditionValidation,
                    ProblemCategory::Precondition,
                    RetryDirective::AfterRequestChange,
                ),
                Refusal::AuthorityConflict => ProblemProjection::refused(
                    PostureOwnerVariantV1::AuthorityConflict,
                    IdempotencyStage::AuthorityValidation,
                    ProblemCategory::Authority,
                    RetryDirective::AfterRequestChange,
                ),
            };
            project_problem(request, definition, projection, limits)
        }
        ResultV1::Error(code) => {
            let projection = match code {
                Error::UnsafeFilesystem => ProblemProjection::error(
                    PostureOwnerVariantV1::UnsafeFilesystem,
                    RetryDirective::Never,
                ),
                Error::IntegrityOrDurability => ProblemProjection::error(
                    PostureOwnerVariantV1::IntegrityOrDurability,
                    RetryDirective::Never,
                ),
                Error::Io => {
                    ProblemProjection::error(PostureOwnerVariantV1::Io, RetryDirective::Transient)
                }
            };
            project_problem(request, definition, projection, limits)
        }
    }
}

fn project_success(
    request: &OperationRequestV1,
    definition: &OperationDefinition,
    owner_key_fingerprint: Fingerprint,
    receipt: handbook_engine::PostureTransitionReceiptV1,
    replayed: bool,
    limits: &ContractLimits,
) -> Result<OperationResponseV1, ContractError> {
    if Fingerprint::parse(&receipt.idempotency_key_fingerprint)? != owner_key_fingerprint {
        return Err(ContractError::FingerprintMismatch);
    }
    let scoped_key = request
        .idempotency_key_fingerprint()
        .cloned()
        .ok_or(ContractError::InvalidInvariant)?;
    let data = PostureTransitionApplyResultDataV1::new(
        scoped_key.clone(),
        Fingerprint::parse(&receipt.repository_identity_fingerprint)?,
        public_canonical(receipt.prior_canonical)?,
        public_canonical(receipt.resulting_canonical)?,
        public_reference(receipt.posture_transition)?,
        public_reference(receipt.lifecycle_transition)?,
        public_reference(receipt.resulting_kernel)?,
    );
    let receipts = success_receipts(&data, limits)?;
    let terminal = TerminalFields::new(
        TerminalStatus::Ok,
        Some(to_contract_value(&data)?),
        BoundedVec::new(Vec::new())?,
        BoundedVec::new(Vec::new())?,
        BoundedVec::new(Vec::new())?,
    )?;
    let retention = UtcTimestamp::parse(RETENTION_UNTIL_UTC)?;
    let original_result = if replayed {
        let original = response_from_parts(
            request,
            definition,
            terminal.clone(),
            IdempotencyState::established(
                scoped_key.clone(),
                request.request_fingerprint().clone(),
                false,
                None,
                retention.clone(),
            )?,
            receipts.clone(),
            limits,
        )?;
        Some(original.result_fingerprint().clone())
    } else {
        None
    };
    response_from_parts(
        request,
        definition,
        terminal,
        IdempotencyState::established(
            scoped_key,
            request.request_fingerprint().clone(),
            replayed,
            original_result,
            retention,
        )?,
        receipts,
        limits,
    )
}

fn public_reference(
    value: handbook_engine::PostureReferenceV1,
) -> Result<PostureReferenceDtoV1, ContractError> {
    PostureReferenceDtoV1::new(value.reference, value.fingerprint)
}

fn public_canonical(
    value: handbook_engine::PostureCanonicalBindingV1,
) -> Result<PostureCanonicalBindingDtoV1, ContractError> {
    PostureCanonicalBindingDtoV1::new(
        value.reference,
        value.fingerprint,
        value.document_sha256,
        value.byte_length,
    )
}

fn success_receipts(
    data: &PostureTransitionApplyResultDataV1,
    limits: &ContractLimits,
) -> Result<BoundedVec<WriteReceipt, 128>, ContractError> {
    let atomic_group = || BoundedString::new("posture_transition_apply".to_owned());
    BoundedVec::new(vec![
        WriteReceipt::new(
            BoundedString::new("constitutional_root".to_owned())?,
            ExactBinding::new(
                ExactRef::parse("handbook.posture-record.constitutional-root@1.0.0")?,
                data.resulting_canonical.fingerprint().clone(),
            ),
            BoundedString::new("canonical_truth".to_owned())?,
            to_contract_value(&PostureCanonicalWriteConditionV1::new(
                data.prior_canonical.clone(),
                data.resulting_canonical.clone(),
            ))?,
            atomic_group()?,
            limits,
        )?,
        WriteReceipt::new(
            BoundedString::new("posture_transition".to_owned())?,
            ExactBinding::new(
                ExactRef::parse("handbook.posture-record.transition@1.0.0")?,
                data.posture_transition.fingerprint().clone(),
            ),
            BoundedString::new("semantic_record".to_owned())?,
            to_contract_value(&PostureSemanticWriteConditionV1::new(
                data.lifecycle_transition.clone(),
                data.posture_transition.clone(),
                data.resulting_kernel.clone(),
            ))?,
            atomic_group()?,
            limits,
        )?,
    ])
}

#[derive(Clone, Copy)]
struct ProblemProjection {
    variant: PostureOwnerVariantV1,
    status: TerminalStatus,
    stage: IdempotencyStage,
    category: ProblemCategory,
    retry: RetryDirective,
}

impl ProblemProjection {
    fn blocked(variant: PostureOwnerVariantV1) -> Self {
        Self {
            variant,
            status: TerminalStatus::Blocked,
            stage: IdempotencyStage::CapabilityValidation,
            category: ProblemCategory::Prerequisite,
            retry: RetryDirective::AfterRecheck,
        }
    }

    fn refused(
        variant: PostureOwnerVariantV1,
        stage: IdempotencyStage,
        category: ProblemCategory,
        retry: RetryDirective,
    ) -> Self {
        Self {
            variant,
            status: TerminalStatus::Refused,
            stage,
            category,
            retry,
        }
    }

    fn error(variant: PostureOwnerVariantV1, retry: RetryDirective) -> Self {
        Self {
            variant,
            status: TerminalStatus::Error,
            stage: IdempotencyStage::ExecutionStart,
            category: ProblemCategory::Implementation,
            retry,
        }
    }
}

fn project_problem(
    request: &OperationRequestV1,
    definition: &OperationDefinition,
    projection: ProblemProjection,
    limits: &ContractLimits,
) -> Result<OperationResponseV1, ContractError> {
    let (rust_name, kebab, snake) = variant_names(projection.variant);
    let details = to_contract_value(&PostureOwnerVariantDetailsV1::new(projection.variant))?;
    let details_schema = schema_binding(&format!("handbook.posture-details.{kebab}@1.0.0"))?;
    PostureSchemaValidator.validate_instance(&details_schema, &details)?;
    let rule = if projection.status == TerminalStatus::Refused {
        Some(ExactBinding::new(
            ExactRef::parse(&format!("handbook.posture-rule.{kebab}@1.0.0"))?,
            definition.owner().source().fingerprint().clone(),
        ))
    } else {
        None
    };
    let recheck = if projection.status == TerminalStatus::Blocked {
        let condition_schema = schema_binding(UNSUPPORTED_RECHECK_SCHEMA_REF)?;
        PostureSchemaValidator.validate_instance(&condition_schema, &details)?;
        Some(RecheckCondition::new(
            condition_schema,
            details.clone(),
            limits,
        )?)
    } else {
        None
    };
    let correlation_ref = if projection.status == TerminalStatus::Error {
        let mut preimage = b"handbook.posture-error-correlation-v1\0".to_vec();
        preimage.extend_from_slice(request.request_fingerprint().as_str().as_bytes());
        preimage.push(0);
        preimage.extend_from_slice(rust_name.as_bytes());
        let digest = sha256_fingerprint(&preimage);
        Some(CorrelationRef::new(BoundedString::new(format!(
            "corr_{}",
            digest.as_str().trim_start_matches("sha256:")
        ))?)?)
    } else {
        None
    };
    let problem_id = BoundedString::new(format!("posture.{snake}"))?;
    let problem = Problem::new(
        problem_id.clone(),
        BoundedString::new(format!("posture.{snake}"))?,
        projection.category,
        definition.binding(),
        rule,
        details_schema,
        details,
        BoundedVec::new(Vec::new())?,
        projection.retry,
        recheck,
        correlation_ref,
        limits,
    )?;
    let problem_binding = ProblemBinding::new(problem_id, problem.problem_fingerprint().clone())?;
    let terminal = match projection.status {
        TerminalStatus::Blocked => TerminalFields::new(
            TerminalStatus::Blocked,
            None,
            BoundedVec::new(vec![problem])?,
            BoundedVec::new(Vec::new())?,
            BoundedVec::new(Vec::new())?,
        )?,
        TerminalStatus::Refused => TerminalFields::new(
            TerminalStatus::Refused,
            None,
            BoundedVec::new(Vec::new())?,
            BoundedVec::new(vec![problem])?,
            BoundedVec::new(Vec::new())?,
        )?,
        TerminalStatus::Error => TerminalFields::new(
            TerminalStatus::Error,
            None,
            BoundedVec::new(Vec::new())?,
            BoundedVec::new(Vec::new())?,
            BoundedVec::new(vec![problem])?,
        )?,
        TerminalStatus::Ok => return Err(ContractError::InvalidInvariant),
    };
    response_from_parts(
        request,
        definition,
        terminal,
        IdempotencyState::not_established(projection.stage, problem_binding)?,
        BoundedVec::new(Vec::new())?,
        limits,
    )
}

fn response_from_parts(
    request: &OperationRequestV1,
    definition: &OperationDefinition,
    terminal: TerminalFields<ContractValue>,
    idempotency: IdempotencyState,
    receipts: BoundedVec<WriteReceipt, 128>,
    limits: &ContractLimits,
) -> Result<OperationResponseV1, ContractError> {
    OperationResponseV1::new(
        schema_binding("handbook.operation-response@1.0.0")?,
        request.request_id().cloned(),
        request.request_fingerprint().clone(),
        definition,
        terminal,
        idempotency,
        receipts,
        BoundedVec::new(Vec::new())?,
        BoundedVec::new(Vec::new())?,
        &PostureSchemaValidator,
        limits,
    )
}

fn variant_names(variant: PostureOwnerVariantV1) -> (&'static str, &'static str, &'static str) {
    match variant {
        PostureOwnerVariantV1::UnsupportedPlatform => (
            "UnsupportedPlatform",
            "unsupported-platform",
            "unsupported_platform",
        ),
        PostureOwnerVariantV1::InvalidIdempotencyKey => (
            "InvalidIdempotencyKey",
            "invalid-idempotency-key",
            "invalid_idempotency_key",
        ),
        PostureOwnerVariantV1::RepositoryIdentityMismatch => (
            "RepositoryIdentityMismatch",
            "repository-identity-mismatch",
            "repository_identity_mismatch",
        ),
        PostureOwnerVariantV1::InvalidFixedDimension => (
            "InvalidFixedDimension",
            "invalid-fixed-dimension",
            "invalid_fixed_dimension",
        ),
        PostureOwnerVariantV1::StaleCanonical => {
            ("StaleCanonical", "stale-canonical", "stale_canonical")
        }
        PostureOwnerVariantV1::MissingLifecycleAuthority => (
            "MissingLifecycleAuthority",
            "missing-lifecycle-authority",
            "missing_lifecycle_authority",
        ),
        PostureOwnerVariantV1::LifecycleNotCurrent => (
            "LifecycleNotCurrent",
            "lifecycle-not-current",
            "lifecycle_not_current",
        ),
        PostureOwnerVariantV1::InvalidSemanticInput => (
            "InvalidSemanticInput",
            "invalid-semantic-input",
            "invalid_semantic_input",
        ),
        PostureOwnerVariantV1::AuthorityConflict => (
            "AuthorityConflict",
            "authority-conflict",
            "authority_conflict",
        ),
        PostureOwnerVariantV1::UnsafeFilesystem => {
            ("UnsafeFilesystem", "unsafe-filesystem", "unsafe_filesystem")
        }
        PostureOwnerVariantV1::IntegrityOrDurability => (
            "IntegrityOrDurability",
            "integrity-or-durability",
            "integrity_or_durability",
        ),
        PostureOwnerVariantV1::Io => ("Io", "io", "io"),
    }
}

pub(crate) struct PostureSchemaValidator;

impl OperationSchemaValidator for PostureSchemaValidator {
    fn validate_instance(
        &self,
        schema: &ExactBinding,
        instance: &ContractValue,
    ) -> Result<(), ContractError> {
        let bytes = canonical_json_bytes(instance, &ContractLimits::packet0())?;
        match schema.reference().as_str() {
            REQUEST_SCHEMA_REF => parse_canonical_json::<RawPostureTransitionApplyWireBodyV1>(
                &bytes,
                &ContractLimits::packet0(),
            )?
            .validate()
            .map(|_| ()),
            RESULT_SCHEMA_REF => parse_canonical_json::<PostureTransitionApplyResultDataV1>(
                &bytes,
                &ContractLimits::packet0(),
            )
            .map(|_| ()),
            CANONICAL_CONDITION_SCHEMA_REF => parse_canonical_json::<
                PostureCanonicalWriteConditionV1,
            >(&bytes, &ContractLimits::packet0())
            .map(|_| ()),
            SEMANTIC_CONDITION_SCHEMA_REF => {
                parse_canonical_json::<PostureSemanticWriteConditionV1>(
                    &bytes,
                    &ContractLimits::packet0(),
                )
                .map(|_| ())
            }
            UNSUPPORTED_RECHECK_SCHEMA_REF => {
                let details = parse_canonical_json::<PostureOwnerVariantDetailsV1>(
                    &bytes,
                    &ContractLimits::packet0(),
                )?;
                if details.owner_variant() == PostureOwnerVariantV1::UnsupportedPlatform {
                    Ok(())
                } else {
                    Err(ContractError::InvalidInvariant)
                }
            }
            reference if reference.starts_with("handbook.posture-details.") => {
                let details = parse_canonical_json::<PostureOwnerVariantDetailsV1>(
                    &bytes,
                    &ContractLimits::packet0(),
                )?;
                let (_, kebab, _) = variant_names(details.owner_variant());
                if reference == format!("handbook.posture-details.{kebab}@1.0.0") {
                    Ok(())
                } else {
                    Err(ContractError::InvalidInvariant)
                }
            }
            _ => Err(ContractError::InvalidInvariant),
        }
    }
}

pub(crate) fn schema_binding(reference: &str) -> Result<ExactBinding, ContractError> {
    super::production_witness::current_schema_binding(reference)
}

pub(crate) fn to_contract_value<T: Serialize>(value: &T) -> Result<ContractValue, ContractError> {
    let limits = ContractLimits::packet0();
    parse_canonical_json(&canonical_json_bytes(value, &limits)?, &limits)
}

pub(crate) fn inventory_row_is_posture(operation_ref: &ExactRef) -> bool {
    operation_inventory().iter().any(|row| {
        row.operation_ref() == operation_ref && row.operation_ref().as_str() == OPERATION_REF
    })
}

fn sanitize_deserialize_error<E: serde::de::Error>(error: E) -> E {
    E::custom(super::serialization::classify_contract_error_message(
        &error.to_string(),
    ))
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contract_membrane::IdempotencyStateRef;
    use handbook_engine::{
        PostureTransitionApplyResultV1 as OwnerResult, PostureTransitionBlockedCodeV1,
        PostureTransitionErrorCodeV1, PostureTransitionRefusalCodeV1,
    };

    const RAW_KEY: &str = "packet-four-owner-key";

    fn fingerprint(seed: char) -> String {
        format!("sha256:{}", seed.to_string().repeat(64))
    }

    fn request(request_id: &str) -> OperationRequestV1 {
        posture_transition_apply_request(
            Some(request_id),
            RAW_KEY,
            PostureTransitionApplyRequestDataV1::new(
                fingerprint('1'),
                PostureCanonicalBindingDtoV1::new(
                    CANONICAL_REFERENCE.to_owned(),
                    fingerprint('2'),
                    fingerprint('2'),
                    100,
                )
                .unwrap(),
                PostureReferenceDtoV1::new("candidate.yaml".to_owned(), fingerprint('3')).unwrap(),
                vec!["engineering_posture.dimensions".to_owned()],
                PostureDimensionChangeDtoV1::new(
                    "testing_rigor".to_owned(),
                    2,
                    "/engineering_posture/dimensions/2/level_override".to_owned(),
                    Some(4),
                    4,
                    3,
                )
                .unwrap(),
                "2026-08-10T16:30:00Z".to_owned(),
            )
            .unwrap(),
        )
        .unwrap()
    }

    fn owner_receipt() -> handbook_engine::PostureTransitionReceiptV1 {
        handbook_engine::PostureTransitionReceiptV1 {
            idempotency_key_fingerprint: sha256_fingerprint(RAW_KEY.as_bytes()).as_str().to_owned(),
            repository_identity_fingerprint: fingerprint('1'),
            prior_canonical: handbook_engine::PostureCanonicalBindingV1 {
                reference: CANONICAL_REFERENCE.to_owned(),
                fingerprint: fingerprint('2'),
                document_sha256: fingerprint('2'),
                byte_length: 100,
            },
            resulting_canonical: handbook_engine::PostureCanonicalBindingV1 {
                reference: CANONICAL_REFERENCE.to_owned(),
                fingerprint: fingerprint('4'),
                document_sha256: fingerprint('4'),
                byte_length: 101,
            },
            posture_transition: handbook_engine::PostureReferenceV1 {
                reference: "posture-transitions/transition.json".to_owned(),
                fingerprint: fingerprint('5'),
            },
            lifecycle_transition: handbook_engine::PostureReferenceV1 {
                reference: "lifecycle-transitions/transition.json".to_owned(),
                fingerprint: fingerprint('6'),
            },
            resulting_kernel: handbook_engine::PostureReferenceV1 {
                reference: "project-posture-kernels/result".to_owned(),
                fingerprint: fingerprint('7'),
            },
        }
    }

    #[test]
    fn applied_replayed_and_request_correlation_preserve_one_semantic_result() {
        let definition = posture_transition_apply_definition().unwrap();
        let limits = ContractLimits::packet0();
        let first_request = request("correlation-a");
        let second_request = request("correlation-b");
        assert_eq!(
            first_request.request_fingerprint(),
            second_request.request_fingerprint()
        );
        let owner_key = sha256_fingerprint(RAW_KEY.as_bytes());
        let applied = project_owner_result(
            &first_request,
            &definition,
            owner_key.clone(),
            OwnerResult::Applied(owner_receipt()),
            &limits,
        )
        .unwrap();
        let correlated = project_owner_result(
            &second_request,
            &definition,
            owner_key.clone(),
            OwnerResult::Applied(owner_receipt()),
            &limits,
        )
        .unwrap();
        let replayed = project_owner_result(
            &second_request,
            &definition,
            owner_key,
            OwnerResult::Replayed(owner_receipt()),
            &limits,
        )
        .unwrap();

        assert_eq!(
            applied.result_fingerprint(),
            correlated.result_fingerprint()
        );
        assert_ne!(
            applied.response_fingerprint(),
            correlated.response_fingerprint()
        );
        assert_eq!(applied.result_fingerprint(), replayed.result_fingerprint());
        assert_ne!(
            applied.response_fingerprint(),
            replayed.response_fingerprint()
        );
        replayed.verify_replay_against(&applied).unwrap();
        assert_eq!(applied.receipts().len(), 2);
        assert_eq!(
            applied.receipts()[0].record().reference().as_str(),
            "handbook.posture-record.constitutional-root@1.0.0"
        );
        assert_eq!(
            applied.receipts()[1].record().reference().as_str(),
            "handbook.posture-record.transition@1.0.0"
        );
        assert_eq!(
            applied.receipts()[0].record().fingerprint().as_str(),
            fingerprint('4')
        );
        assert_eq!(
            applied.receipts()[1].record().fingerprint().as_str(),
            fingerprint('5')
        );
        assert!(matches!(
            applied.idempotency().as_ref(),
            IdempotencyStateRef::Established {
                replayed: false,
                original_result_fingerprint: None,
                ..
            }
        ));
        assert!(matches!(
            replayed.idempotency().as_ref(),
            IdempotencyStateRef::Established {
                replayed: true,
                original_result_fingerprint: Some(original),
                ..
            } if original == replayed.result_fingerprint()
        ));
        for response in [&applied, &correlated, &replayed] {
            let bytes = canonical_json_bytes(response, &limits).unwrap();
            assert!(!bytes
                .windows(RAW_KEY.len())
                .any(|window| window == RAW_KEY.as_bytes()));
        }
    }

    #[test]
    fn every_closed_owner_failure_maps_to_its_exact_terminal_problem() {
        let definition = posture_transition_apply_definition().unwrap();
        let request = request("failure-correlation");
        let limits = ContractLimits::packet0();
        let cases = vec![
            (
                OwnerResult::Blocked(PostureTransitionBlockedCodeV1::UnsupportedPlatform),
                TerminalStatus::Blocked,
                IdempotencyStage::CapabilityValidation,
                ProblemCategory::Prerequisite,
                "posture.unsupported_platform",
                RetryDirective::AfterRecheck,
            ),
            (
                OwnerResult::Refused(PostureTransitionRefusalCodeV1::InvalidIdempotencyKey),
                TerminalStatus::Refused,
                IdempotencyStage::RequestValidation,
                ProblemCategory::Schema,
                "posture.invalid_idempotency_key",
                RetryDirective::AfterRequestChange,
            ),
            (
                OwnerResult::Refused(PostureTransitionRefusalCodeV1::RepositoryIdentityMismatch),
                TerminalStatus::Refused,
                IdempotencyStage::AuthorityValidation,
                ProblemCategory::Authority,
                "posture.repository_identity_mismatch",
                RetryDirective::AfterRequestChange,
            ),
            (
                OwnerResult::Refused(PostureTransitionRefusalCodeV1::InvalidFixedDimension),
                TerminalStatus::Refused,
                IdempotencyStage::PreconditionValidation,
                ProblemCategory::Precondition,
                "posture.invalid_fixed_dimension",
                RetryDirective::AfterRequestChange,
            ),
            (
                OwnerResult::Refused(PostureTransitionRefusalCodeV1::StaleCanonical),
                TerminalStatus::Refused,
                IdempotencyStage::PreconditionValidation,
                ProblemCategory::Precondition,
                "posture.stale_canonical",
                RetryDirective::AfterRequestChange,
            ),
            (
                OwnerResult::Refused(PostureTransitionRefusalCodeV1::MissingLifecycleAuthority),
                TerminalStatus::Refused,
                IdempotencyStage::AuthorityValidation,
                ProblemCategory::Authority,
                "posture.missing_lifecycle_authority",
                RetryDirective::AfterAuthorityChange,
            ),
            (
                OwnerResult::Refused(PostureTransitionRefusalCodeV1::LifecycleNotCurrent),
                TerminalStatus::Refused,
                IdempotencyStage::PreconditionValidation,
                ProblemCategory::Precondition,
                "posture.lifecycle_not_current",
                RetryDirective::AfterAuthorityChange,
            ),
            (
                OwnerResult::Refused(PostureTransitionRefusalCodeV1::InvalidSemanticInput),
                TerminalStatus::Refused,
                IdempotencyStage::PreconditionValidation,
                ProblemCategory::Precondition,
                "posture.invalid_semantic_input",
                RetryDirective::AfterRequestChange,
            ),
            (
                OwnerResult::Refused(PostureTransitionRefusalCodeV1::AuthorityConflict),
                TerminalStatus::Refused,
                IdempotencyStage::AuthorityValidation,
                ProblemCategory::Authority,
                "posture.authority_conflict",
                RetryDirective::AfterRequestChange,
            ),
            (
                OwnerResult::Error(PostureTransitionErrorCodeV1::UnsafeFilesystem),
                TerminalStatus::Error,
                IdempotencyStage::ExecutionStart,
                ProblemCategory::Implementation,
                "posture.unsafe_filesystem",
                RetryDirective::Never,
            ),
            (
                OwnerResult::Error(PostureTransitionErrorCodeV1::IntegrityOrDurability),
                TerminalStatus::Error,
                IdempotencyStage::ExecutionStart,
                ProblemCategory::Implementation,
                "posture.integrity_or_durability",
                RetryDirective::Never,
            ),
            (
                OwnerResult::Error(PostureTransitionErrorCodeV1::Io),
                TerminalStatus::Error,
                IdempotencyStage::ExecutionStart,
                ProblemCategory::Implementation,
                "posture.io",
                RetryDirective::Transient,
            ),
        ];

        for (owner, status, stage, category, code, retry) in cases {
            let response = project_owner_result(
                &request,
                &definition,
                sha256_fingerprint(RAW_KEY.as_bytes()),
                owner,
                &limits,
            )
            .unwrap();
            assert_eq!(response.terminal().status(), status);
            assert!(response.terminal().data().is_none());
            assert!(response.receipts().is_empty());
            assert!(response.diagnostics().is_empty());
            assert!(response.next_actions().is_empty());
            let problem = match status {
                TerminalStatus::Blocked => &response.terminal().blockers()[0],
                TerminalStatus::Refused => &response.terminal().refusals()[0],
                TerminalStatus::Error => &response.terminal().errors()[0],
                TerminalStatus::Ok => unreachable!(),
            };
            assert_eq!(problem.code().as_str(), code);
            assert_eq!(problem.category(), category);
            assert_eq!(problem.retry(), retry);
            assert_eq!(problem.rule().is_some(), status == TerminalStatus::Refused);
            assert_eq!(
                problem.recheck().is_some(),
                status == TerminalStatus::Blocked
            );
            assert_eq!(
                problem.correlation_ref().is_some(),
                status == TerminalStatus::Error
            );
            assert!(matches!(
                response.idempotency().as_ref(),
                IdempotencyStateRef::NotEstablished {
                    stage: actual_stage,
                    terminal_problem,
                } if actual_stage == stage
                    && terminal_problem.problem_fingerprint() == problem.problem_fingerprint()
            ));
            let bytes = canonical_json_bytes(&response, &limits).unwrap();
            assert!(!bytes
                .windows(RAW_KEY.len())
                .any(|window| window == RAW_KEY.as_bytes()));
        }

        let hostile = serde_json::json!({"owner_variant": RAW_KEY});
        let error = serde_json::from_value::<PostureOwnerVariantDetailsV1>(hostile)
            .unwrap_err()
            .to_string();
        assert!(!error.contains(RAW_KEY));
    }
}
