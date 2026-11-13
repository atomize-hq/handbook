use super::{
    canonical_fingerprint, canonical_json_bytes, ArtifactRef, BoundedString, BoundedVec,
    ContractError, ContractLimits, ContractValue, ExactBinding, Fingerprint, SafeU64,
};
use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::cmp::Ordering;

macro_rules! closed_enum {
    ($name:ident { $($variant:ident),+ $(,)? }) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
        #[serde(rename_all = "snake_case")]
        pub enum $name {
            $($variant),+
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                #[derive(Deserialize)]
                #[serde(rename_all = "snake_case")]
                enum Raw {
                    $($variant),+
                }

                let raw = Raw::deserialize(deserializer).map_err(sanitize_deserialize_error)?;
                Ok(match raw {
                    $(Raw::$variant => Self::$variant),+
                })
            }
        }
    };
}

closed_enum!(ProblemCategory {
    Prerequisite,
    Schema,
    Compatibility,
    Capability,
    Resolution,
    Authority,
    Precondition,
    Safety,
    Idempotency,
    Implementation,
    Adapter,
});

closed_enum!(DiagnosticSeverity {
    Information,
    Warning,
});

closed_enum!(RetryDirective {
    AfterRecheck,
    AfterRequestChange,
    AfterAuthorityChange,
    Transient,
    Never,
});

closed_enum!(NextActionKind {
    Recheck,
    ChangeRequest,
    ChangeAuthority,
    RetryTransient,
    ContactOperator,
});

closed_enum!(OmissionReason {
    OutOfResolution,
    Redacted,
    Unavailable,
    Unsupported,
});

closed_enum!(ProofEffect {
    None,
    Partial,
    Blocked,
    Refused,
});

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub enum DraftVersion {
    #[serde(rename = "2020-12")]
    Draft202012,
}

impl<'de> Deserialize<'de> for DraftVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        enum Raw {
            #[serde(rename = "2020-12")]
            Draft202012,
        }

        Raw::deserialize(deserializer)
            .map(|Raw::Draft202012| Self::Draft202012)
            .map_err(sanitize_deserialize_error)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub enum SchemaMediaType {
    #[serde(rename = "application/schema+json")]
    ApplicationSchemaJson,
}

impl<'de> Deserialize<'de> for SchemaMediaType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        enum Raw {
            #[serde(rename = "application/schema+json")]
            ApplicationSchemaJson,
        }

        Raw::deserialize(deserializer)
            .map(|Raw::ApplicationSchemaJson| Self::ApplicationSchemaJson)
            .map_err(sanitize_deserialize_error)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct Problem {
    problem_id: BoundedString<255>,
    code: BoundedString<255>,
    category: ProblemCategory,
    subject: ExactBinding,
    rule: Option<ExactBinding>,
    details_schema: ExactBinding,
    details: ContractValue,
    details_fingerprint: Fingerprint,
    evidence: BoundedVec<ArtifactRef, 128>,
    retry: RetryDirective,
    recheck: Option<RecheckCondition>,
    correlation_ref: Option<CorrelationRef>,
    problem_fingerprint: Fingerprint,
}

impl Problem {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        problem_id: BoundedString<255>,
        code: BoundedString<255>,
        category: ProblemCategory,
        subject: ExactBinding,
        rule: Option<ExactBinding>,
        details_schema: ExactBinding,
        details: ContractValue,
        evidence: BoundedVec<ArtifactRef, 128>,
        retry: RetryDirective,
        recheck: Option<RecheckCondition>,
        correlation_ref: Option<CorrelationRef>,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        validate_token(problem_id.as_str())?;
        validate_token(code.as_str())?;
        validate_problem_shape(
            category,
            rule.as_ref(),
            retry,
            recheck.as_ref(),
            correlation_ref.as_ref(),
        )?;
        validate_evidence_order(evidence.as_slice())?;

        let details_fingerprint = canonical_fingerprint(
            &DetailsPreimage {
                details_schema: &details_schema,
                details: &details,
            },
            limits,
        )?;
        let problem_fingerprint = canonical_fingerprint(
            &ProblemPreimage {
                problem_id: &problem_id,
                code: &code,
                category,
                subject: &subject,
                rule: rule.as_ref(),
                details_schema: &details_schema,
                details: &details,
                details_fingerprint: &details_fingerprint,
                evidence: &evidence,
                retry,
                recheck: recheck.as_ref(),
                correlation_ref: correlation_ref.as_ref(),
            },
            limits,
        )?;
        let value = Self {
            problem_id,
            code,
            category,
            subject,
            rule,
            details_schema,
            details,
            details_fingerprint,
            evidence,
            retry,
            recheck,
            correlation_ref,
            problem_fingerprint,
        };
        validate_serializable(&value, limits)?;
        Ok(value)
    }

    pub fn problem_id(&self) -> &BoundedString<255> {
        &self.problem_id
    }

    pub fn code(&self) -> &BoundedString<255> {
        &self.code
    }

    pub fn category(&self) -> ProblemCategory {
        self.category
    }

    pub fn subject(&self) -> &ExactBinding {
        &self.subject
    }

    pub fn rule(&self) -> Option<&ExactBinding> {
        self.rule.as_ref()
    }

    pub fn details_schema(&self) -> &ExactBinding {
        &self.details_schema
    }

    pub fn details(&self) -> &ContractValue {
        &self.details
    }

    pub fn details_fingerprint(&self) -> &Fingerprint {
        &self.details_fingerprint
    }

    pub fn evidence(&self) -> &BoundedVec<ArtifactRef, 128> {
        &self.evidence
    }

    pub fn retry(&self) -> RetryDirective {
        self.retry
    }

    pub fn recheck(&self) -> Option<&RecheckCondition> {
        self.recheck.as_ref()
    }

    pub fn correlation_ref(&self) -> Option<&CorrelationRef> {
        self.correlation_ref.as_ref()
    }

    pub fn problem_fingerprint(&self) -> &Fingerprint {
        &self.problem_fingerprint
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawProblem {
    problem_id: BoundedString<255>,
    code: BoundedString<255>,
    category: ProblemCategory,
    subject: ExactBinding,
    #[serde(default)]
    rule: RequiredOption<ExactBinding>,
    details_schema: ExactBinding,
    details: ContractValue,
    details_fingerprint: Fingerprint,
    evidence: BoundedVec<ArtifactRef, 128>,
    retry: RetryDirective,
    #[serde(default)]
    recheck: RequiredOption<RecheckCondition>,
    #[serde(default)]
    correlation_ref: RequiredOption<CorrelationRef>,
    problem_fingerprint: Fingerprint,
}

impl<'de> Deserialize<'de> for Problem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = RawProblem::deserialize(deserializer).map_err(sanitize_deserialize_error)?;
        let rule = raw.rule.into_option().map_err(D::Error::custom)?;
        let recheck = raw.recheck.into_option().map_err(D::Error::custom)?;
        let correlation_ref = raw
            .correlation_ref
            .into_option()
            .map_err(D::Error::custom)?;
        let value = Self::new(
            raw.problem_id,
            raw.code,
            raw.category,
            raw.subject,
            rule,
            raw.details_schema,
            raw.details,
            raw.evidence,
            raw.retry,
            recheck,
            correlation_ref,
            &ContractLimits::packet0(),
        )
        .map_err(D::Error::custom)?;
        if raw.details_fingerprint != value.details_fingerprint
            || raw.problem_fingerprint != value.problem_fingerprint
        {
            return Err(D::Error::custom(ContractError::FingerprintMismatch));
        }
        Ok(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct Diagnostic {
    code: BoundedString<255>,
    severity: DiagnosticSeverity,
    subject: ExactBinding,
    details_schema: ExactBinding,
    details: ContractValue,
    details_fingerprint: Fingerprint,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_message: Option<BoundedString<1024>>,
}

impl Diagnostic {
    pub fn new(
        code: BoundedString<255>,
        severity: DiagnosticSeverity,
        subject: ExactBinding,
        details_schema: ExactBinding,
        details: ContractValue,
        display_message: Option<BoundedString<1024>>,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        validate_token(code.as_str())?;
        let details_fingerprint = canonical_fingerprint(
            &DetailsPreimage {
                details_schema: &details_schema,
                details: &details,
            },
            limits,
        )?;
        let value = Self {
            code,
            severity,
            subject,
            details_schema,
            details,
            details_fingerprint,
            display_message,
        };
        validate_serializable(&value, limits)?;
        Ok(value)
    }

    pub fn code(&self) -> &BoundedString<255> {
        &self.code
    }

    pub fn severity(&self) -> DiagnosticSeverity {
        self.severity
    }

    pub fn subject(&self) -> &ExactBinding {
        &self.subject
    }

    pub fn details_schema(&self) -> &ExactBinding {
        &self.details_schema
    }

    pub fn details(&self) -> &ContractValue {
        &self.details
    }

    pub fn details_fingerprint(&self) -> &Fingerprint {
        &self.details_fingerprint
    }

    pub fn display_message(&self) -> Option<&BoundedString<1024>> {
        self.display_message.as_ref()
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawDiagnostic {
    code: BoundedString<255>,
    severity: DiagnosticSeverity,
    subject: ExactBinding,
    details_schema: ExactBinding,
    details: ContractValue,
    details_fingerprint: Fingerprint,
    #[serde(default)]
    display_message: OmittedOption<BoundedString<1024>>,
}

impl<'de> Deserialize<'de> for Diagnostic {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = RawDiagnostic::deserialize(deserializer).map_err(sanitize_deserialize_error)?;
        let display_message = raw.display_message.into_option();
        let value = Self::new(
            raw.code,
            raw.severity,
            raw.subject,
            raw.details_schema,
            raw.details,
            display_message,
            &ContractLimits::packet0(),
        )
        .map_err(D::Error::custom)?;
        if raw.details_fingerprint != value.details_fingerprint {
            return Err(D::Error::custom(ContractError::FingerprintMismatch));
        }
        Ok(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct NextAction {
    action_id: BoundedString<255>,
    kind: NextActionKind,
    subject: ExactBinding,
    parameter_schema: ExactBinding,
    parameters: ContractValue,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_label: Option<BoundedString<1024>>,
}

impl NextAction {
    pub fn new(
        action_id: BoundedString<255>,
        kind: NextActionKind,
        subject: ExactBinding,
        parameter_schema: ExactBinding,
        parameters: ContractValue,
        display_label: Option<BoundedString<1024>>,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        validate_token(action_id.as_str())?;
        let value = Self {
            action_id,
            kind,
            subject,
            parameter_schema,
            parameters,
            display_label,
        };
        validate_serializable(&value, limits)?;
        Ok(value)
    }

    pub fn action_id(&self) -> &BoundedString<255> {
        &self.action_id
    }

    pub fn kind(&self) -> NextActionKind {
        self.kind
    }

    pub fn subject(&self) -> &ExactBinding {
        &self.subject
    }

    pub fn parameter_schema(&self) -> &ExactBinding {
        &self.parameter_schema
    }

    pub fn parameters(&self) -> &ContractValue {
        &self.parameters
    }

    pub fn display_label(&self) -> Option<&BoundedString<1024>> {
        self.display_label.as_ref()
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawNextAction {
    action_id: BoundedString<255>,
    kind: NextActionKind,
    subject: ExactBinding,
    parameter_schema: ExactBinding,
    parameters: ContractValue,
    #[serde(default)]
    display_label: OmittedOption<BoundedString<1024>>,
}

impl<'de> Deserialize<'de> for NextAction {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = RawNextAction::deserialize(deserializer).map_err(sanitize_deserialize_error)?;
        let display_label = raw.display_label.into_option();
        Self::new(
            raw.action_id,
            raw.kind,
            raw.subject,
            raw.parameter_schema,
            raw.parameters,
            display_label,
            &ContractLimits::packet0(),
        )
        .map_err(D::Error::custom)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct SourceBinding {
    source: ExactBinding,
    captured_revision: ExactBinding,
    adapter: ExactBinding,
}

impl SourceBinding {
    pub fn new(
        source: ExactBinding,
        captured_revision: ExactBinding,
        adapter: ExactBinding,
    ) -> Result<Self, ContractError> {
        Ok(Self {
            source,
            captured_revision,
            adapter,
        })
    }

    pub fn source(&self) -> &ExactBinding {
        &self.source
    }

    pub fn captured_revision(&self) -> &ExactBinding {
        &self.captured_revision
    }

    pub fn adapter(&self) -> &ExactBinding {
        &self.adapter
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawSourceBinding {
    source: ExactBinding,
    captured_revision: ExactBinding,
    adapter: ExactBinding,
}

impl<'de> Deserialize<'de> for SourceBinding {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw =
            RawSourceBinding::deserialize(deserializer).map_err(sanitize_deserialize_error)?;
        Self::new(raw.source, raw.captured_revision, raw.adapter).map_err(D::Error::custom)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct Omission {
    item: ExactBinding,
    reason: OmissionReason,
    proof_effect: ProofEffect,
    source: Option<SourceBinding>,
    ordinal: SafeU64,
}

impl Omission {
    pub fn new(
        item: ExactBinding,
        reason: OmissionReason,
        proof_effect: ProofEffect,
        source: Option<SourceBinding>,
        ordinal: SafeU64,
    ) -> Result<Self, ContractError> {
        Ok(Self {
            item,
            reason,
            proof_effect,
            source,
            ordinal,
        })
    }

    pub fn item(&self) -> &ExactBinding {
        &self.item
    }

    pub fn reason(&self) -> OmissionReason {
        self.reason
    }

    pub fn proof_effect(&self) -> ProofEffect {
        self.proof_effect
    }

    pub fn source(&self) -> Option<&SourceBinding> {
        self.source.as_ref()
    }

    pub fn ordinal(&self) -> SafeU64 {
        self.ordinal
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawOmission {
    item: ExactBinding,
    reason: OmissionReason,
    proof_effect: ProofEffect,
    #[serde(default)]
    source: RequiredOption<SourceBinding>,
    ordinal: SafeU64,
}

impl<'de> Deserialize<'de> for Omission {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = RawOmission::deserialize(deserializer).map_err(sanitize_deserialize_error)?;
        let source = raw.source.into_option().map_err(D::Error::custom)?;
        Self::new(raw.item, raw.reason, raw.proof_effect, source, raw.ordinal)
            .map_err(D::Error::custom)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct SchemaManifestEntry {
    schema: ExactBinding,
    draft: DraftVersion,
    byte_length: SafeU64,
    media_type: SchemaMediaType,
}

impl SchemaManifestEntry {
    pub fn new(
        schema: ExactBinding,
        draft: DraftVersion,
        byte_length: SafeU64,
        media_type: SchemaMediaType,
    ) -> Result<Self, ContractError> {
        Ok(Self {
            schema,
            draft,
            byte_length,
            media_type,
        })
    }

    pub fn schema(&self) -> &ExactBinding {
        &self.schema
    }

    pub fn draft(&self) -> DraftVersion {
        self.draft
    }

    pub fn byte_length(&self) -> SafeU64 {
        self.byte_length
    }

    pub fn media_type(&self) -> SchemaMediaType {
        self.media_type
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawSchemaManifestEntry {
    schema: ExactBinding,
    draft: DraftVersion,
    byte_length: SafeU64,
    media_type: SchemaMediaType,
}

impl<'de> Deserialize<'de> for SchemaManifestEntry {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = RawSchemaManifestEntry::deserialize(deserializer)
            .map_err(sanitize_deserialize_error)?;
        Self::new(raw.schema, raw.draft, raw.byte_length, raw.media_type).map_err(D::Error::custom)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct RecheckCondition {
    condition_schema: ExactBinding,
    condition: ContractValue,
    condition_fingerprint: Fingerprint,
}

impl RecheckCondition {
    pub fn new(
        condition_schema: ExactBinding,
        condition: ContractValue,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        let condition_fingerprint = canonical_fingerprint(
            &ConditionPreimage {
                condition_schema: &condition_schema,
                condition: &condition,
            },
            limits,
        )?;
        let value = Self {
            condition_schema,
            condition,
            condition_fingerprint,
        };
        validate_serializable(&value, limits)?;
        Ok(value)
    }

    pub fn condition_schema(&self) -> &ExactBinding {
        &self.condition_schema
    }

    pub fn condition(&self) -> &ContractValue {
        &self.condition
    }

    pub fn condition_fingerprint(&self) -> &Fingerprint {
        &self.condition_fingerprint
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawRecheckCondition {
    condition_schema: ExactBinding,
    condition: ContractValue,
    condition_fingerprint: Fingerprint,
}

impl<'de> Deserialize<'de> for RecheckCondition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw =
            RawRecheckCondition::deserialize(deserializer).map_err(sanitize_deserialize_error)?;
        let value = Self::new(
            raw.condition_schema,
            raw.condition,
            &ContractLimits::packet0(),
        )
        .map_err(D::Error::custom)?;
        if raw.condition_fingerprint != value.condition_fingerprint {
            return Err(D::Error::custom(ContractError::FingerprintMismatch));
        }
        Ok(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CorrelationRef {
    value: BoundedString<133>,
}

impl CorrelationRef {
    pub fn new(value: BoundedString<133>) -> Result<Self, ContractError> {
        let suffix = value.as_str().strip_prefix("corr_");
        if !suffix.is_some_and(|suffix| {
            (16..=128).contains(&suffix.len())
                && suffix
                    .bytes()
                    .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-'))
        }) {
            return Err(ContractError::InvalidString);
        }
        Ok(Self { value })
    }

    pub fn value(&self) -> &BoundedString<133> {
        &self.value
    }
}

impl Serialize for CorrelationRef {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.value.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for CorrelationRef {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value =
            BoundedString::<133>::deserialize(deserializer).map_err(sanitize_deserialize_error)?;
        Self::new(value).map_err(D::Error::custom)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct WriteReceipt {
    record_kind: BoundedString<255>,
    record: ExactBinding,
    authority_class: BoundedString<255>,
    condition: ContractValue,
    atomic_group: BoundedString<255>,
    receipt_fingerprint: Fingerprint,
}

impl WriteReceipt {
    pub fn new(
        record_kind: BoundedString<255>,
        record: ExactBinding,
        authority_class: BoundedString<255>,
        condition: ContractValue,
        atomic_group: BoundedString<255>,
        limits: &ContractLimits,
    ) -> Result<Self, ContractError> {
        validate_token(record_kind.as_str())?;
        validate_token(authority_class.as_str())?;
        validate_token(atomic_group.as_str())?;
        let receipt_fingerprint = canonical_fingerprint(
            &ReceiptPreimage {
                record_kind: &record_kind,
                record: &record,
                authority_class: &authority_class,
                condition: &condition,
                atomic_group: &atomic_group,
            },
            limits,
        )?;
        let value = Self {
            record_kind,
            record,
            authority_class,
            condition,
            atomic_group,
            receipt_fingerprint,
        };
        validate_serializable(&value, limits)?;
        Ok(value)
    }

    pub fn record_kind(&self) -> &BoundedString<255> {
        &self.record_kind
    }

    pub fn record(&self) -> &ExactBinding {
        &self.record
    }

    pub fn authority_class(&self) -> &BoundedString<255> {
        &self.authority_class
    }

    pub fn condition(&self) -> &ContractValue {
        &self.condition
    }

    pub fn atomic_group(&self) -> &BoundedString<255> {
        &self.atomic_group
    }

    pub fn receipt_fingerprint(&self) -> &Fingerprint {
        &self.receipt_fingerprint
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawWriteReceipt {
    record_kind: BoundedString<255>,
    record: ExactBinding,
    authority_class: BoundedString<255>,
    condition: ContractValue,
    atomic_group: BoundedString<255>,
    receipt_fingerprint: Fingerprint,
}

impl<'de> Deserialize<'de> for WriteReceipt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = RawWriteReceipt::deserialize(deserializer).map_err(sanitize_deserialize_error)?;
        let value = Self::new(
            raw.record_kind,
            raw.record,
            raw.authority_class,
            raw.condition,
            raw.atomic_group,
            &ContractLimits::packet0(),
        )
        .map_err(D::Error::custom)?;
        if raw.receipt_fingerprint != value.receipt_fingerprint {
            return Err(D::Error::custom(ContractError::FingerprintMismatch));
        }
        Ok(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ProblemBinding {
    problem_id: BoundedString<255>,
    problem_fingerprint: Fingerprint,
}

impl ProblemBinding {
    pub fn new(
        problem_id: BoundedString<255>,
        problem_fingerprint: Fingerprint,
    ) -> Result<Self, ContractError> {
        validate_token(problem_id.as_str())?;
        Ok(Self {
            problem_id,
            problem_fingerprint,
        })
    }

    pub fn problem_id(&self) -> &BoundedString<255> {
        &self.problem_id
    }

    pub fn problem_fingerprint(&self) -> &Fingerprint {
        &self.problem_fingerprint
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawProblemBinding {
    problem_id: BoundedString<255>,
    problem_fingerprint: Fingerprint,
}

impl<'de> Deserialize<'de> for ProblemBinding {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw =
            RawProblemBinding::deserialize(deserializer).map_err(sanitize_deserialize_error)?;
        Self::new(raw.problem_id, raw.problem_fingerprint).map_err(D::Error::custom)
    }
}

#[derive(Serialize)]
struct DetailsPreimage<'a> {
    details_schema: &'a ExactBinding,
    details: &'a ContractValue,
}

#[derive(Serialize)]
struct ProblemPreimage<'a> {
    problem_id: &'a BoundedString<255>,
    code: &'a BoundedString<255>,
    category: ProblemCategory,
    subject: &'a ExactBinding,
    rule: Option<&'a ExactBinding>,
    details_schema: &'a ExactBinding,
    details: &'a ContractValue,
    details_fingerprint: &'a Fingerprint,
    evidence: &'a BoundedVec<ArtifactRef, 128>,
    retry: RetryDirective,
    recheck: Option<&'a RecheckCondition>,
    correlation_ref: Option<&'a CorrelationRef>,
}

#[derive(Serialize)]
struct ConditionPreimage<'a> {
    condition_schema: &'a ExactBinding,
    condition: &'a ContractValue,
}

#[derive(Serialize)]
struct ReceiptPreimage<'a> {
    record_kind: &'a BoundedString<255>,
    record: &'a ExactBinding,
    authority_class: &'a BoundedString<255>,
    condition: &'a ContractValue,
    atomic_group: &'a BoundedString<255>,
}

enum OmittedOption<T> {
    Missing,
    Present(T),
}

impl<T> OmittedOption<T> {
    fn into_option(self) -> Option<T> {
        match self {
            Self::Missing => None,
            Self::Present(value) => Some(value),
        }
    }
}

impl<T> Default for OmittedOption<T> {
    fn default() -> Self {
        Self::Missing
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for OmittedOption<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        T::deserialize(deserializer).map(Self::Present)
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

fn validate_problem_shape(
    category: ProblemCategory,
    rule: Option<&ExactBinding>,
    retry: RetryDirective,
    recheck: Option<&RecheckCondition>,
    correlation_ref: Option<&CorrelationRef>,
) -> Result<(), ContractError> {
    let valid = match category {
        ProblemCategory::Prerequisite => {
            rule.is_none()
                && recheck.is_some()
                && correlation_ref.is_none()
                && retry == RetryDirective::AfterRecheck
        }
        ProblemCategory::Schema
        | ProblemCategory::Compatibility
        | ProblemCategory::Capability
        | ProblemCategory::Resolution
        | ProblemCategory::Authority
        | ProblemCategory::Precondition
        | ProblemCategory::Safety
        | ProblemCategory::Idempotency => {
            rule.is_some()
                && recheck.is_none()
                && correlation_ref.is_none()
                && matches!(
                    retry,
                    RetryDirective::AfterRequestChange
                        | RetryDirective::AfterAuthorityChange
                        | RetryDirective::Never
                )
        }
        ProblemCategory::Implementation | ProblemCategory::Adapter => {
            rule.is_none()
                && recheck.is_none()
                && correlation_ref.is_some()
                && matches!(retry, RetryDirective::Transient | RetryDirective::Never)
        }
    };
    if valid {
        Ok(())
    } else {
        Err(ContractError::InvalidInvariant)
    }
}

fn validate_evidence_order(evidence: &[ArtifactRef]) -> Result<(), ContractError> {
    for pair in evidence.windows(2) {
        match artifact_binding_cmp(&pair[0], &pair[1]) {
            Ordering::Less => {}
            Ordering::Equal => return Err(ContractError::DuplicateMember),
            Ordering::Greater => return Err(ContractError::NonCanonicalOrder),
        }
    }
    Ok(())
}

fn artifact_binding_cmp(left: &ArtifactRef, right: &ArtifactRef) -> Ordering {
    (
        left.artifact().reference().as_str(),
        left.artifact().fingerprint().as_str(),
    )
        .cmp(&(
            right.artifact().reference().as_str(),
            right.artifact().fingerprint().as_str(),
        ))
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

fn validate_serializable<T: Serialize>(
    value: &T,
    limits: &ContractLimits,
) -> Result<(), ContractError> {
    canonical_json_bytes(value, limits).map(|_| ())
}

fn sanitize_deserialize_error<E: serde::de::Error>(error: E) -> E {
    E::custom(super::serialization::classify_contract_error_message(
        &error.to_string(),
    ))
}

#[cfg(test)]
mod tests {
    use super::super::{parse_canonical_json, ExactRef};
    use super::*;
    use serde::de::DeserializeOwned;
    use std::fmt;

    const SENTINEL: &str = "raw-key-sentinel-do-not-echo";
    const ZERO_FINGERPRINT: &str =
        "sha256:0000000000000000000000000000000000000000000000000000000000000000";

    #[test]
    fn display_fields_accept_omission_but_reject_explicit_null() {
        let limits = ContractLimits::packet0();
        let diagnostic = Diagnostic::new(
            bounded("diagnostic.ready"),
            DiagnosticSeverity::Information,
            binding("subject.item"),
            binding("details.schema"),
            ContractValue::null(),
            None,
            &limits,
        )
        .unwrap();
        let diagnostic_json =
            String::from_utf8(canonical_json_bytes(&diagnostic, &limits).unwrap()).unwrap();
        assert_eq!(
            parse_canonical_json::<Diagnostic>(diagnostic_json.as_bytes(), &limits).unwrap(),
            diagnostic
        );
        let diagnostic_null =
            diagnostic_json.replace(r#","severity":"#, r#","display_message":null,"severity":"#);
        assert_explicit_null_refuses::<Diagnostic>(&diagnostic_null, &limits);

        let action = NextAction::new(
            bounded("action.retry"),
            NextActionKind::RetryTransient,
            binding("subject.item"),
            binding("parameter.schema"),
            ContractValue::null(),
            None,
            &limits,
        )
        .unwrap();
        let action_json =
            String::from_utf8(canonical_json_bytes(&action, &limits).unwrap()).unwrap();
        assert_eq!(
            parse_canonical_json::<NextAction>(action_json.as_bytes(), &limits).unwrap(),
            action
        );
        let action_null = action_json.replace(r#","kind":"#, r#","display_label":null,"kind":"#);
        assert_explicit_null_refuses::<NextAction>(&action_null, &limits);
    }

    #[test]
    fn public_dto_deserializers_and_enums_redact_rejected_input() {
        assert_redacted::<ProblemCategory>(&format!(r#""{SENTINEL}""#));
        assert_redacted::<DiagnosticSeverity>(&format!(r#""{SENTINEL}""#));
        assert_redacted::<RetryDirective>(&format!(r#""{SENTINEL}""#));
        assert_redacted::<NextActionKind>(&format!(r#""{SENTINEL}""#));
        assert_redacted::<OmissionReason>(&format!(r#""{SENTINEL}""#));
        assert_redacted::<ProofEffect>(&format!(r#""{SENTINEL}""#));
        assert_redacted::<DraftVersion>(&format!(r#""{SENTINEL}""#));
        assert_redacted::<SchemaMediaType>(&format!(r#""{SENTINEL}""#));

        assert_object_boundary_redacts::<Problem>();
        assert_object_boundary_redacts::<Diagnostic>();
        assert_object_boundary_redacts::<NextAction>();
        assert_object_boundary_redacts::<SourceBinding>();
        assert_object_boundary_redacts::<Omission>();
        assert_object_boundary_redacts::<SchemaManifestEntry>();
        assert_object_boundary_redacts::<RecheckCondition>();
        assert_object_boundary_redacts::<WriteReceipt>();
        assert_object_boundary_redacts::<ProblemBinding>();
        assert_redacted::<CorrelationRef>(&format!(r#""{SENTINEL}""#));
    }

    fn assert_explicit_null_refuses<T>(json: &str, limits: &ContractLimits)
    where
        T: DeserializeOwned + Serialize + fmt::Debug + Eq,
    {
        let direct_error = serde_json::from_str::<T>(json).unwrap_err().to_string();
        assert!(direct_error.starts_with(&ContractError::InvalidJson.to_string()));
        assert_eq!(
            parse_canonical_json::<T>(json.as_bytes(), limits),
            Err(ContractError::InvalidJson)
        );
    }

    fn assert_object_boundary_redacts<T>()
    where
        T: DeserializeOwned + fmt::Debug,
    {
        assert_redacted::<T>(&format!(r#"{{"{SENTINEL}":true}}"#));
        assert_redacted::<T>(&format!(r#""{SENTINEL}""#));
    }

    fn assert_redacted<T>(json: &str)
    where
        T: DeserializeOwned + fmt::Debug,
    {
        let error = serde_json::from_str::<T>(json).unwrap_err().to_string();
        assert!(
            !error.contains(SENTINEL),
            "public error echoed input: {error}"
        );
    }

    fn bounded<const N: usize>(value: &str) -> BoundedString<N> {
        BoundedString::new(value.to_owned()).unwrap()
    }

    fn binding(identity: &str) -> ExactBinding {
        ExactBinding::new(
            ExactRef::parse(&format!("{identity}@1.0.0")).unwrap(),
            Fingerprint::parse(ZERO_FINGERPRINT).unwrap(),
        )
    }
}
