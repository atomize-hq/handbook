use crate::bounds::*;
use crate::canonical::{canonical_bytes, canonical_fingerprint, fingerprint_hex, framed_json};
use crate::manifest::{require_ref, valid_media_type, validate_strict_ids};
use crate::{DockErrorV1, DockManifestV1, DockProtocolLimitsV1};
use handbook_contracts::{
    ContentAddressedBindingV1, EvaluationCaseV1, EvaluationSubjectV1, EvidenceObservationV1,
    ExactBindingV1, FingerprintV1, ResolutionEnvelopeV1, SemanticTimeV1, StableIdV1,
};
use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use std::collections::BTreeSet;

use crate::{
    DOCK_EXECUTION_RECORD_SCHEMA_REF_V1, DOCK_PROCESS_PROTOCOL_REF_V1,
    DOCK_PROCESS_REQUEST_SCHEMA_REF_V1, DOCK_PROCESS_RESULT_SCHEMA_REF_V1,
};

#[derive(Clone, Debug)]
pub struct DockEvaluationRunBindingInputV1 {
    pub evaluation_run: ContentAddressedBindingV1,
    pub evaluation_lifecycle_basis: ContentAddressedBindingV1,
    pub contract: ExactBindingV1,
    pub claim_ids: Vec<StableIdV1>,
    pub subject: EvaluationSubjectV1,
    pub cases: Vec<EvaluationCaseV1>,
    pub requested_resolution: ResolutionEnvelopeV1,
    pub semantic_time_utc: SemanticTimeV1,
    pub lifecycle_head_fingerprint: FingerprintV1,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct DockEvaluationRunBindingV1 {
    evaluation_run: ContentAddressedBindingV1,
    evaluation_lifecycle_basis: ContentAddressedBindingV1,
    contract: ExactBindingV1,
    claim_ids: Vec<StableIdV1>,
    subject: EvaluationSubjectV1,
    cases: Vec<EvaluationCaseV1>,
    requested_resolution: ResolutionEnvelopeV1,
    semantic_time_utc: SemanticTimeV1,
    lifecycle_head_fingerprint: FingerprintV1,
}

impl DockEvaluationRunBindingV1 {
    pub fn new(input: DockEvaluationRunBindingInputV1) -> Result<Self, DockErrorV1> {
        validate_strict_ids(&input.claim_ids, MAX_CANDIDATE_OBSERVATIONS)?;
        if input.cases.is_empty() || input.cases.len() > MAX_INLINE_INPUTS {
            return Err(DockErrorV1::BoundExceeded);
        }
        if input
            .cases
            .windows(2)
            .any(|pair| pair[0].case_id() >= pair[1].case_id())
        {
            return Err(DockErrorV1::InvalidInvariant);
        }
        Ok(Self {
            evaluation_run: input.evaluation_run,
            evaluation_lifecycle_basis: input.evaluation_lifecycle_basis,
            contract: input.contract,
            claim_ids: input.claim_ids,
            subject: input.subject,
            cases: input.cases,
            requested_resolution: input.requested_resolution,
            semantic_time_utc: input.semantic_time_utc,
            lifecycle_head_fingerprint: input.lifecycle_head_fingerprint,
        })
    }
    pub fn evaluation_run(&self) -> &ContentAddressedBindingV1 {
        &self.evaluation_run
    }
    pub fn evaluation_lifecycle_basis(&self) -> &ContentAddressedBindingV1 {
        &self.evaluation_lifecycle_basis
    }
    pub fn contract(&self) -> &ExactBindingV1 {
        &self.contract
    }
    pub fn claim_ids(&self) -> &[StableIdV1] {
        &self.claim_ids
    }
    pub fn subject(&self) -> &EvaluationSubjectV1 {
        &self.subject
    }
    pub fn cases(&self) -> &[EvaluationCaseV1] {
        &self.cases
    }
    pub fn requested_resolution(&self) -> &ResolutionEnvelopeV1 {
        &self.requested_resolution
    }
    pub fn semantic_time_utc(&self) -> &SemanticTimeV1 {
        &self.semantic_time_utc
    }
    pub fn lifecycle_head_fingerprint(&self) -> &FingerprintV1 {
        &self.lifecycle_head_fingerprint
    }
}

impl<'de> Deserialize<'de> for DockEvaluationRunBindingV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Raw {
            evaluation_run: ContentAddressedBindingV1,
            evaluation_lifecycle_basis: ContentAddressedBindingV1,
            contract: ExactBindingV1,
            claim_ids: Vec<StableIdV1>,
            subject: EvaluationSubjectV1,
            cases: Vec<EvaluationCaseV1>,
            requested_resolution: ResolutionEnvelopeV1,
            semantic_time_utc: SemanticTimeV1,
            lifecycle_head_fingerprint: FingerprintV1,
        }
        let raw = Raw::deserialize(deserializer)?;
        Self::new(DockEvaluationRunBindingInputV1 {
            evaluation_run: raw.evaluation_run,
            evaluation_lifecycle_basis: raw.evaluation_lifecycle_basis,
            contract: raw.contract,
            claim_ids: raw.claim_ids,
            subject: raw.subject,
            cases: raw.cases,
            requested_resolution: raw.requested_resolution,
            semantic_time_utc: raw.semantic_time_utc,
            lifecycle_head_fingerprint: raw.lifecycle_head_fingerprint,
        })
        .map_err(D::Error::custom)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct DockInlineJsonInputV1 {
    role: StableIdV1,
    media_type: String,
    schema: ExactBindingV1,
    content: Value,
    content_fingerprint: FingerprintV1,
}

impl DockInlineJsonInputV1 {
    pub fn new(
        role: StableIdV1,
        media_type: &str,
        schema: ExactBindingV1,
        content: Value,
    ) -> Result<Self, DockErrorV1> {
        if !valid_media_type(media_type) {
            return Err(DockErrorV1::InvalidInvariant);
        }
        let bytes = canonical_bytes(&content)?;
        if bytes.len() > MAX_INLINE_INPUT_BYTES {
            return Err(DockErrorV1::BoundExceeded);
        }
        let content_fingerprint = canonical_fingerprint(&content)?;
        Ok(Self {
            role,
            media_type: media_type.to_owned(),
            schema,
            content,
            content_fingerprint,
        })
    }
    pub fn role(&self) -> &StableIdV1 {
        &self.role
    }
    pub fn media_type(&self) -> &str {
        &self.media_type
    }
    pub fn schema(&self) -> &ExactBindingV1 {
        &self.schema
    }
    pub fn content(&self) -> &Value {
        &self.content
    }
    pub fn content_fingerprint(&self) -> &FingerprintV1 {
        &self.content_fingerprint
    }
}

impl<'de> Deserialize<'de> for DockInlineJsonInputV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Raw {
            role: StableIdV1,
            media_type: String,
            schema: ExactBindingV1,
            content: Value,
            content_fingerprint: FingerprintV1,
        }
        let raw = Raw::deserialize(deserializer)?;
        let supplied = raw.content_fingerprint;
        let value = Self::new(raw.role, &raw.media_type, raw.schema, raw.content)
            .map_err(D::Error::custom)?;
        if value.content_fingerprint != supplied {
            return Err(D::Error::custom(DockErrorV1::DerivedValueMismatch));
        }
        Ok(value)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DockNetworkPolicyV1 {
    Denied,
}

#[derive(Clone, Debug)]
pub struct DockProcessRequestInputV1 {
    pub run: DockEvaluationRunBindingV1,
    pub requested_evidence_kinds: Vec<StableIdV1>,
    pub inputs: Vec<DockInlineJsonInputV1>,
    pub effective_limits: DockProtocolLimitsV1,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct DockProcessRequestV1 {
    request_schema: ExactBindingV1,
    result_schema: ExactBindingV1,
    protocol: ExactBindingV1,
    request_ref: String,
    request_fingerprint: FingerprintV1,
    manifest: ExactBindingV1,
    implementation_bundle: ExactBindingV1,
    run: DockEvaluationRunBindingV1,
    resolution_ceiling: ResolutionEnvelopeV1,
    requested_evidence_kinds: Vec<StableIdV1>,
    inputs: Vec<DockInlineJsonInputV1>,
    effective_limits: DockProtocolLimitsV1,
    network: DockNetworkPolicyV1,
}

impl DockProcessRequestV1 {
    pub fn new(
        manifest: &DockManifestV1,
        input: DockProcessRequestInputV1,
    ) -> Result<Self, DockErrorV1> {
        if input.effective_limits.checked_min(manifest.limits())? != input.effective_limits {
            return Err(DockErrorV1::BoundExceeded);
        }
        if input.run.requested_resolution() != manifest.resolution_ceiling() {
            return Err(DockErrorV1::Substituted);
        }
        validate_strict_ids(
            &input.requested_evidence_kinds,
            manifest.limits().candidate_count(),
        )?;
        if input
            .requested_evidence_kinds
            .iter()
            .any(|kind| !manifest.supported_evidence_kinds().contains(kind))
        {
            return Err(DockErrorV1::Unknown);
        }
        validate_inputs(&input.inputs, manifest, &input.effective_limits)?;
        Self::from_parts(
            manifest.request_schema().clone(),
            manifest.result_schema().clone(),
            manifest.protocol().clone(),
            manifest.binding(),
            manifest.implementation_bundle().binding(),
            input.run,
            manifest.resolution_ceiling().clone(),
            input.requested_evidence_kinds,
            input.inputs,
            input.effective_limits,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn from_parts(
        request_schema: ExactBindingV1,
        result_schema: ExactBindingV1,
        protocol: ExactBindingV1,
        manifest: ExactBindingV1,
        implementation_bundle: ExactBindingV1,
        run: DockEvaluationRunBindingV1,
        resolution_ceiling: ResolutionEnvelopeV1,
        requested_evidence_kinds: Vec<StableIdV1>,
        inputs: Vec<DockInlineJsonInputV1>,
        effective_limits: DockProtocolLimitsV1,
    ) -> Result<Self, DockErrorV1> {
        require_ref(&request_schema, DOCK_PROCESS_REQUEST_SCHEMA_REF_V1)?;
        require_ref(&result_schema, DOCK_PROCESS_RESULT_SCHEMA_REF_V1)?;
        require_ref(&protocol, DOCK_PROCESS_PROTOCOL_REF_V1)?;
        if run.requested_resolution() != &resolution_ceiling {
            return Err(DockErrorV1::Substituted);
        }
        validate_strict_ids(
            &requested_evidence_kinds,
            effective_limits.candidate_count(),
        )?;
        validate_role_order(&inputs, effective_limits.input_count())?;
        let preimage = RequestPreimage {
            request_schema: &request_schema,
            result_schema: &result_schema,
            protocol: &protocol,
            manifest: &manifest,
            implementation_bundle: &implementation_bundle,
            run: &run,
            resolution_ceiling: &resolution_ceiling,
            requested_evidence_kinds: &requested_evidence_kinds,
            inputs: &inputs,
            effective_limits: &effective_limits,
            network: DockNetworkPolicyV1::Denied,
        };
        let request_fingerprint = canonical_fingerprint(&preimage)?;
        let request_ref = format!(
            "{}#request/{}",
            manifest.reference(),
            fingerprint_hex(&request_fingerprint)
        );
        let value = Self {
            request_schema,
            result_schema,
            protocol,
            request_ref,
            request_fingerprint,
            manifest,
            implementation_bundle,
            run,
            resolution_ceiling,
            requested_evidence_kinds,
            inputs,
            effective_limits,
            network: DockNetworkPolicyV1::Denied,
        };
        if canonical_bytes(&value)?.len() > value.effective_limits.request_bytes() {
            return Err(DockErrorV1::BoundExceeded);
        }
        Ok(value)
    }

    pub fn request_ref(&self) -> &str {
        &self.request_ref
    }
    pub fn request_fingerprint(&self) -> &FingerprintV1 {
        &self.request_fingerprint
    }
    pub fn manifest(&self) -> &ExactBindingV1 {
        &self.manifest
    }
    pub fn implementation_bundle(&self) -> &ExactBindingV1 {
        &self.implementation_bundle
    }
    pub fn run(&self) -> &DockEvaluationRunBindingV1 {
        &self.run
    }
    pub fn effective_limits(&self) -> &DockProtocolLimitsV1 {
        &self.effective_limits
    }
    pub fn requested_evidence_kinds(&self) -> &[StableIdV1] {
        &self.requested_evidence_kinds
    }
    pub fn inputs(&self) -> &[DockInlineJsonInputV1] {
        &self.inputs
    }
    pub fn binding(&self) -> Result<ContentAddressedBindingV1, DockErrorV1> {
        ContentAddressedBindingV1::new(&self.request_ref, self.request_fingerprint.clone())
            .map_err(|_| DockErrorV1::InvalidIdentity)
    }
    pub fn canonical_json_bytes(&self) -> Result<Vec<u8>, DockErrorV1> {
        canonical_bytes(self)
    }
    pub fn framed_json_bytes(&self) -> Result<Vec<u8>, DockErrorV1> {
        framed_json(self, self.effective_limits.request_bytes())
    }
    pub fn admit_framed_json(bytes: &[u8], manifest: &DockManifestV1) -> Result<Self, DockErrorV1> {
        let value: Self =
            crate::canonical::admit_framed_json(bytes, manifest.limits().request_bytes())?;
        let readmitted = Self::new(
            manifest,
            DockProcessRequestInputV1 {
                run: value.run.clone(),
                requested_evidence_kinds: value.requested_evidence_kinds.clone(),
                inputs: value.inputs.clone(),
                effective_limits: value.effective_limits.clone(),
            },
        )?;
        if value != readmitted {
            return Err(DockErrorV1::Substituted);
        }
        Ok(value)
    }
}

impl<'de> Deserialize<'de> for DockProcessRequestV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Raw {
            request_schema: ExactBindingV1,
            result_schema: ExactBindingV1,
            protocol: ExactBindingV1,
            request_ref: String,
            request_fingerprint: FingerprintV1,
            manifest: ExactBindingV1,
            implementation_bundle: ExactBindingV1,
            run: DockEvaluationRunBindingV1,
            resolution_ceiling: ResolutionEnvelopeV1,
            requested_evidence_kinds: Vec<StableIdV1>,
            inputs: Vec<DockInlineJsonInputV1>,
            effective_limits: DockProtocolLimitsV1,
            network: DockNetworkPolicyV1,
        }
        let raw = Raw::deserialize(deserializer)?;
        if raw.network != DockNetworkPolicyV1::Denied {
            return Err(D::Error::custom(DockErrorV1::InvalidInvariant));
        }
        let supplied_ref = raw.request_ref;
        let supplied_fp = raw.request_fingerprint;
        let value = Self::from_parts(
            raw.request_schema,
            raw.result_schema,
            raw.protocol,
            raw.manifest,
            raw.implementation_bundle,
            raw.run,
            raw.resolution_ceiling,
            raw.requested_evidence_kinds,
            raw.inputs,
            raw.effective_limits,
        )
        .map_err(D::Error::custom)?;
        if value.request_ref != supplied_ref || value.request_fingerprint != supplied_fp {
            return Err(D::Error::custom(DockErrorV1::DerivedValueMismatch));
        }
        Ok(value)
    }
}

#[derive(Serialize)]
struct RequestPreimage<'a> {
    request_schema: &'a ExactBindingV1,
    result_schema: &'a ExactBindingV1,
    protocol: &'a ExactBindingV1,
    manifest: &'a ExactBindingV1,
    implementation_bundle: &'a ExactBindingV1,
    run: &'a DockEvaluationRunBindingV1,
    resolution_ceiling: &'a ResolutionEnvelopeV1,
    requested_evidence_kinds: &'a [StableIdV1],
    inputs: &'a [DockInlineJsonInputV1],
    effective_limits: &'a DockProtocolLimitsV1,
    network: DockNetworkPolicyV1,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DockProcessResultStatusV1 {
    Completed,
    Refused,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DockProcessRefusalReasonV1 {
    InputRefused,
    UnsupportedInput,
    LimitRefused,
    ObservationUnavailable,
    InternalRefusal,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct DockDiagnosticV1 {
    code: StableIdV1,
    message: String,
}
impl DockDiagnosticV1 {
    pub fn new(code: StableIdV1, message: &str) -> Result<Self, DockErrorV1> {
        if message.is_empty()
            || message.len() > MAX_DIAGNOSTIC_BYTES
            || message.chars().any(char::is_control)
        {
            return Err(DockErrorV1::BoundExceeded);
        }
        Ok(Self {
            code,
            message: message.to_owned(),
        })
    }
}
impl<'de> Deserialize<'de> for DockDiagnosticV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Raw {
            code: StableIdV1,
            message: String,
        }
        let raw = Raw::deserialize(deserializer)?;
        Self::new(raw.code, &raw.message).map_err(D::Error::custom)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct DockCandidateObservationV1 {
    claim_id: StableIdV1,
    case_id: StableIdV1,
    evidence_kind: StableIdV1,
    payload_schema: ExactBindingV1,
    payload: Value,
    payload_fingerprint: FingerprintV1,
    observation: EvidenceObservationV1,
}
impl DockCandidateObservationV1 {
    pub fn new(
        claim_id: StableIdV1,
        case_id: StableIdV1,
        evidence_kind: StableIdV1,
        payload_schema: ExactBindingV1,
        payload: Value,
    ) -> Result<Self, DockErrorV1> {
        Self::new_with_observation(
            claim_id,
            case_id,
            evidence_kind,
            payload_schema,
            payload,
            EvidenceObservationV1::Satisfied,
        )
    }
    pub fn new_with_observation(
        claim_id: StableIdV1,
        case_id: StableIdV1,
        evidence_kind: StableIdV1,
        payload_schema: ExactBindingV1,
        payload: Value,
        observation: EvidenceObservationV1,
    ) -> Result<Self, DockErrorV1> {
        if canonical_bytes(&payload)?.len() > MAX_CANDIDATE_BYTES {
            return Err(DockErrorV1::BoundExceeded);
        }
        let payload_fingerprint = canonical_fingerprint(&payload)?;
        Ok(Self {
            claim_id,
            case_id,
            evidence_kind,
            payload_schema,
            payload,
            payload_fingerprint,
            observation,
        })
    }
    pub fn claim_id(&self) -> &StableIdV1 {
        &self.claim_id
    }
    pub fn case_id(&self) -> &StableIdV1 {
        &self.case_id
    }
    pub fn evidence_kind(&self) -> &StableIdV1 {
        &self.evidence_kind
    }
    pub fn payload(&self) -> &Value {
        &self.payload
    }
    pub fn payload_schema(&self) -> &ExactBindingV1 {
        &self.payload_schema
    }
    pub fn observation(&self) -> EvidenceObservationV1 {
        self.observation
    }
}
impl<'de> Deserialize<'de> for DockCandidateObservationV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Raw {
            claim_id: StableIdV1,
            case_id: StableIdV1,
            evidence_kind: StableIdV1,
            payload_schema: ExactBindingV1,
            payload: Value,
            payload_fingerprint: FingerprintV1,
            observation: EvidenceObservationV1,
        }
        let raw = Raw::deserialize(deserializer)?;
        let supplied = raw.payload_fingerprint;
        let value = Self::new_with_observation(
            raw.claim_id,
            raw.case_id,
            raw.evidence_kind,
            raw.payload_schema,
            raw.payload,
            raw.observation,
        )
        .map_err(D::Error::custom)?;
        if value.payload_fingerprint != supplied {
            return Err(D::Error::custom(DockErrorV1::DerivedValueMismatch));
        }
        Ok(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct DockClaimPartitionV1 {
    observed: Vec<StableIdV1>,
    unobserved: Vec<StableIdV1>,
    typed_excluded: Vec<StableIdV1>,
}
impl DockClaimPartitionV1 {
    pub fn new(
        observed: Vec<StableIdV1>,
        unobserved: Vec<StableIdV1>,
        typed_excluded: Vec<StableIdV1>,
    ) -> Result<Self, DockErrorV1> {
        for values in [&observed, &unobserved, &typed_excluded] {
            if values.windows(2).any(|pair| pair[0] >= pair[1]) {
                return Err(DockErrorV1::InvalidInvariant);
            }
        }
        let total = observed
            .len()
            .checked_add(unobserved.len())
            .and_then(|n| n.checked_add(typed_excluded.len()))
            .ok_or(DockErrorV1::BoundExceeded)?;
        let unique: BTreeSet<_> = observed
            .iter()
            .chain(&unobserved)
            .chain(&typed_excluded)
            .collect();
        if total != unique.len() || total > MAX_CANDIDATE_OBSERVATIONS {
            return Err(DockErrorV1::InvalidInvariant);
        }
        Ok(Self {
            observed,
            unobserved,
            typed_excluded,
        })
    }
    fn complete_for(&self, claims: &[StableIdV1]) -> bool {
        let values: BTreeSet<_> = self
            .observed
            .iter()
            .chain(&self.unobserved)
            .chain(&self.typed_excluded)
            .collect();
        let claims: BTreeSet<_> = claims.iter().collect();
        values == claims
    }
    pub fn observed(&self) -> &[StableIdV1] {
        &self.observed
    }
    pub fn unobserved(&self) -> &[StableIdV1] {
        &self.unobserved
    }
    pub fn typed_excluded(&self) -> &[StableIdV1] {
        &self.typed_excluded
    }
}

impl<'de> Deserialize<'de> for DockClaimPartitionV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Raw {
            observed: Vec<StableIdV1>,
            unobserved: Vec<StableIdV1>,
            typed_excluded: Vec<StableIdV1>,
        }
        let raw = Raw::deserialize(deserializer)?;
        Self::new(raw.observed, raw.unobserved, raw.typed_excluded).map_err(D::Error::custom)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct DockProcessResultV1 {
    result_schema: ExactBindingV1,
    protocol: ExactBindingV1,
    result_ref: String,
    result_fingerprint: FingerprintV1,
    request: ContentAddressedBindingV1,
    manifest: ExactBindingV1,
    implementation_bundle: ExactBindingV1,
    evaluation_run: ContentAddressedBindingV1,
    status: DockProcessResultStatusV1,
    claim_partition: Option<DockClaimPartitionV1>,
    candidates: Vec<DockCandidateObservationV1>,
    diagnostics: Vec<DockDiagnosticV1>,
    actual_resolution: Option<ResolutionEnvelopeV1>,
    refusal_reason: Option<DockProcessRefusalReasonV1>,
}

impl DockProcessResultV1 {
    pub fn completed(
        request: &DockProcessRequestV1,
        partition: DockClaimPartitionV1,
        candidates: Vec<DockCandidateObservationV1>,
        diagnostics: Vec<DockDiagnosticV1>,
        actual_resolution: ResolutionEnvelopeV1,
    ) -> Result<Self, DockErrorV1> {
        if actual_resolution != request.resolution_ceiling
            || actual_resolution != *request.run.requested_resolution()
            || !partition.complete_for(request.run.claim_ids())
            || candidates.iter().any(|candidate| {
                !partition.observed.contains(candidate.claim_id())
                    || !request
                        .requested_evidence_kinds
                        .contains(candidate.evidence_kind())
                    || !request
                        .run
                        .cases()
                        .iter()
                        .any(|case| case.case_id() == candidate.case_id())
            })
        {
            return Err(DockErrorV1::InvalidInvariant);
        }
        Self::from_request(
            request,
            DockProcessResultStatusV1::Completed,
            Some(partition),
            candidates,
            diagnostics,
            Some(actual_resolution),
            None,
        )
    }
    pub fn refused(
        request: &DockProcessRequestV1,
        reason: DockProcessRefusalReasonV1,
        diagnostics: Vec<DockDiagnosticV1>,
    ) -> Result<Self, DockErrorV1> {
        Self::from_request(
            request,
            DockProcessResultStatusV1::Refused,
            None,
            vec![],
            diagnostics,
            None,
            Some(reason),
        )
    }
    fn from_request(
        request: &DockProcessRequestV1,
        status: DockProcessResultStatusV1,
        claim_partition: Option<DockClaimPartitionV1>,
        candidates: Vec<DockCandidateObservationV1>,
        diagnostics: Vec<DockDiagnosticV1>,
        actual_resolution: Option<ResolutionEnvelopeV1>,
        refusal_reason: Option<DockProcessRefusalReasonV1>,
    ) -> Result<Self, DockErrorV1> {
        Self::from_parts(
            request.result_schema.clone(),
            request.protocol.clone(),
            request.binding()?,
            request.manifest.clone(),
            request.implementation_bundle.clone(),
            request.run.evaluation_run().clone(),
            status,
            claim_partition,
            candidates,
            diagnostics,
            actual_resolution,
            refusal_reason,
            Some(request),
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn from_parts(
        result_schema: ExactBindingV1,
        protocol: ExactBindingV1,
        request_binding: ContentAddressedBindingV1,
        manifest: ExactBindingV1,
        implementation_bundle: ExactBindingV1,
        evaluation_run: ContentAddressedBindingV1,
        status: DockProcessResultStatusV1,
        claim_partition: Option<DockClaimPartitionV1>,
        candidates: Vec<DockCandidateObservationV1>,
        diagnostics: Vec<DockDiagnosticV1>,
        actual_resolution: Option<ResolutionEnvelopeV1>,
        refusal_reason: Option<DockProcessRefusalReasonV1>,
        request: Option<&DockProcessRequestV1>,
    ) -> Result<Self, DockErrorV1> {
        require_ref(&result_schema, DOCK_PROCESS_RESULT_SCHEMA_REF_V1)?;
        require_ref(&protocol, DOCK_PROCESS_PROTOCOL_REF_V1)?;
        let structural = match status {
            DockProcessResultStatusV1::Completed => {
                claim_partition.is_some() && actual_resolution.is_some() && refusal_reason.is_none()
            }
            DockProcessResultStatusV1::Refused => {
                claim_partition.is_none()
                    && candidates.is_empty()
                    && actual_resolution.is_none()
                    && refusal_reason.is_some()
            }
        };
        if !structural
            || candidates.len() > MAX_CANDIDATE_OBSERVATIONS
            || diagnostics.len() > MAX_DIAGNOSTICS
        {
            return Err(DockErrorV1::InvalidInvariant);
        }
        if let Some(request) = request {
            if candidates.len() > request.effective_limits.candidate_count()
                || diagnostics.len() > request.effective_limits.diagnostic_count()
                || candidates.iter().any(|candidate| {
                    canonical_bytes(candidate.payload())
                        .is_ok_and(|bytes| bytes.len() > request.effective_limits.candidate_bytes())
                })
                || diagnostics.iter().any(|diagnostic| {
                    diagnostic.message.len() > request.effective_limits.diagnostic_bytes()
                })
            {
                return Err(DockErrorV1::BoundExceeded);
            }
        }
        let preimage = ResultPreimage {
            result_schema: &result_schema,
            protocol: &protocol,
            request: &request_binding,
            manifest: &manifest,
            implementation_bundle: &implementation_bundle,
            evaluation_run: &evaluation_run,
            status,
            claim_partition: &claim_partition,
            candidates: &candidates,
            diagnostics: &diagnostics,
            actual_resolution: &actual_resolution,
            refusal_reason,
        };
        let result_fingerprint = canonical_fingerprint(&preimage)?;
        let result_ref = format!(
            "{}#result/{}",
            request_binding.reference(),
            fingerprint_hex(&result_fingerprint)
        );
        let value = Self {
            result_schema,
            protocol,
            result_ref,
            result_fingerprint,
            request: request_binding,
            manifest,
            implementation_bundle,
            evaluation_run,
            status,
            claim_partition,
            candidates,
            diagnostics,
            actual_resolution,
            refusal_reason,
        };
        let limit = request.map_or(MAX_PROCESS_JSON_BYTES, |value| {
            value.effective_limits.result_bytes()
        });
        if canonical_bytes(&value)?.len() > limit {
            return Err(DockErrorV1::BoundExceeded);
        }
        Ok(value)
    }
    pub fn status(&self) -> DockProcessResultStatusV1 {
        self.status
    }
    pub fn candidates(&self) -> &[DockCandidateObservationV1] {
        &self.candidates
    }
    pub fn request(&self) -> &ContentAddressedBindingV1 {
        &self.request
    }
    pub fn manifest(&self) -> &ExactBindingV1 {
        &self.manifest
    }
    pub fn implementation_bundle(&self) -> &ExactBindingV1 {
        &self.implementation_bundle
    }
    pub fn evaluation_run(&self) -> &ContentAddressedBindingV1 {
        &self.evaluation_run
    }
    pub fn claim_partition(&self) -> Option<&DockClaimPartitionV1> {
        self.claim_partition.as_ref()
    }
    pub fn actual_resolution(&self) -> Option<&ResolutionEnvelopeV1> {
        self.actual_resolution.as_ref()
    }
    pub fn result_ref(&self) -> &str {
        &self.result_ref
    }
    pub fn result_fingerprint(&self) -> &FingerprintV1 {
        &self.result_fingerprint
    }
    pub fn binding(&self) -> Result<ContentAddressedBindingV1, DockErrorV1> {
        ContentAddressedBindingV1::new(&self.result_ref, self.result_fingerprint.clone())
            .map_err(|_| DockErrorV1::InvalidIdentity)
    }
    pub fn canonical_json_bytes(&self) -> Result<Vec<u8>, DockErrorV1> {
        canonical_bytes(self)
    }
    pub fn framed_json_bytes(&self) -> Result<Vec<u8>, DockErrorV1> {
        framed_json(self, MAX_PROCESS_JSON_BYTES)
    }
    pub fn admit_framed_json(
        bytes: &[u8],
        request: &DockProcessRequestV1,
    ) -> Result<Self, DockErrorV1> {
        let value: Self =
            crate::canonical::admit_framed_json(bytes, request.effective_limits.result_bytes())?;
        value.validate_against(request)?;
        Ok(value)
    }
    fn validate_against(&self, request: &DockProcessRequestV1) -> Result<(), DockErrorV1> {
        let rebuilt = match self.status {
            DockProcessResultStatusV1::Completed => Self::completed(
                request,
                self.claim_partition
                    .clone()
                    .ok_or(DockErrorV1::InvalidInvariant)?,
                self.candidates.clone(),
                self.diagnostics.clone(),
                self.actual_resolution
                    .clone()
                    .ok_or(DockErrorV1::InvalidInvariant)?,
            )?,
            DockProcessResultStatusV1::Refused => Self::refused(
                request,
                self.refusal_reason.ok_or(DockErrorV1::InvalidInvariant)?,
                self.diagnostics.clone(),
            )?,
        };
        if &rebuilt != self {
            return Err(DockErrorV1::Substituted);
        }
        Ok(())
    }
}

impl<'de> Deserialize<'de> for DockProcessResultV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Raw {
            result_schema: ExactBindingV1,
            protocol: ExactBindingV1,
            result_ref: String,
            result_fingerprint: FingerprintV1,
            request: ContentAddressedBindingV1,
            manifest: ExactBindingV1,
            implementation_bundle: ExactBindingV1,
            evaluation_run: ContentAddressedBindingV1,
            status: DockProcessResultStatusV1,
            claim_partition: Option<DockClaimPartitionV1>,
            candidates: Vec<DockCandidateObservationV1>,
            diagnostics: Vec<DockDiagnosticV1>,
            actual_resolution: Option<ResolutionEnvelopeV1>,
            refusal_reason: Option<DockProcessRefusalReasonV1>,
        }
        let raw = Raw::deserialize(deserializer)?;
        let supplied_ref = raw.result_ref;
        let supplied_fp = raw.result_fingerprint;
        let value = Self::from_parts(
            raw.result_schema,
            raw.protocol,
            raw.request,
            raw.manifest,
            raw.implementation_bundle,
            raw.evaluation_run,
            raw.status,
            raw.claim_partition,
            raw.candidates,
            raw.diagnostics,
            raw.actual_resolution,
            raw.refusal_reason,
            None,
        )
        .map_err(D::Error::custom)?;
        if value.result_ref != supplied_ref || value.result_fingerprint != supplied_fp {
            return Err(D::Error::custom(DockErrorV1::DerivedValueMismatch));
        }
        Ok(value)
    }
}

#[derive(Serialize)]
struct ResultPreimage<'a> {
    result_schema: &'a ExactBindingV1,
    protocol: &'a ExactBindingV1,
    request: &'a ContentAddressedBindingV1,
    manifest: &'a ExactBindingV1,
    implementation_bundle: &'a ExactBindingV1,
    evaluation_run: &'a ContentAddressedBindingV1,
    status: DockProcessResultStatusV1,
    claim_partition: &'a Option<DockClaimPartitionV1>,
    candidates: &'a [DockCandidateObservationV1],
    diagnostics: &'a [DockDiagnosticV1],
    actual_resolution: &'a Option<ResolutionEnvelopeV1>,
    refusal_reason: Option<DockProcessRefusalReasonV1>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DockHostOutcomeV1 {
    Completed,
    Refused,
    TimedOut,
    Cancelled,
    Failed,
    ProtocolError,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DockHostOutcomeFactsV1 {
    pub cleanup_confirmed: bool,
    pub durability_confirmed: bool,
    pub deadline_reached: bool,
    pub cancellation_accepted: bool,
    pub spawn_or_process_failed: bool,
    pub framing_or_binding_failed: bool,
    pub child_status: Option<DockProcessResultStatusV1>,
}

pub fn classify_host_outcome_v1(facts: &DockHostOutcomeFactsV1) -> DockHostOutcomeV1 {
    if !facts.cleanup_confirmed || !facts.durability_confirmed {
        DockHostOutcomeV1::Failed
    } else if facts.deadline_reached {
        DockHostOutcomeV1::TimedOut
    } else if facts.cancellation_accepted {
        DockHostOutcomeV1::Cancelled
    } else if facts.spawn_or_process_failed {
        DockHostOutcomeV1::Failed
    } else if facts.framing_or_binding_failed {
        DockHostOutcomeV1::ProtocolError
    } else if facts.child_status == Some(DockProcessResultStatusV1::Refused) {
        DockHostOutcomeV1::Refused
    } else if facts.child_status == Some(DockProcessResultStatusV1::Completed) {
        DockHostOutcomeV1::Completed
    } else {
        DockHostOutcomeV1::ProtocolError
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DockCleanupDispositionV1 {
    Confirmed,
    Uncertain,
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DockDurabilityDispositionV1 {
    Confirmed,
    Uncertain,
}

pub struct DockExecutionRecordInputV1<'a> {
    pub request: &'a DockProcessRequestV1,
    pub result: Option<&'a DockProcessResultV1>,
    pub facts: DockHostOutcomeFactsV1,
    pub exit_code: Option<i32>,
    pub wall_time_ms: u64,
    pub stdout_bytes: u64,
    pub stdout_truncated: bool,
    pub stderr_bytes: u64,
    pub stderr_truncated: bool,
    pub diagnostics: Vec<DockDiagnosticV1>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct DockExecutionRecordV1 {
    schema_ref: &'static str,
    record_ref: String,
    record_fingerprint: FingerprintV1,
    request: ContentAddressedBindingV1,
    result: Option<ContentAddressedBindingV1>,
    manifest: ExactBindingV1,
    implementation_bundle: ExactBindingV1,
    evaluation_run: ContentAddressedBindingV1,
    host_outcome: DockHostOutcomeV1,
    exit_code: Option<i32>,
    wall_time_ms: u64,
    stdout_bytes: u64,
    stdout_truncated: bool,
    stderr_bytes: u64,
    stderr_truncated: bool,
    cleanup: DockCleanupDispositionV1,
    durability: DockDurabilityDispositionV1,
    effective_limits: DockProtocolLimitsV1,
    diagnostics: Vec<DockDiagnosticV1>,
    candidate_observation_bundle_fingerprint: FingerprintV1,
    candidate_observation_count: usize,
}

impl DockExecutionRecordV1 {
    pub fn new(input: DockExecutionRecordInputV1<'_>) -> Result<Self, DockErrorV1> {
        let limits = input.request.effective_limits();
        let time_bound = limits
            .wall_timeout_ms()
            .checked_add(limits.cancellation_grace_ms())
            .ok_or(DockErrorV1::BoundExceeded)?;
        if input.wall_time_ms > time_bound
            || input.stdout_bytes > (limits.result_bytes() as u64 + 1)
            || input.stderr_bytes > limits.stderr_bytes() as u64
            || input.diagnostics.len() > limits.diagnostic_count()
            || input
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.message.len() > limits.diagnostic_bytes())
        {
            return Err(DockErrorV1::BoundExceeded);
        }
        let mut effective_facts = input.facts;
        if input.wall_time_ms >= limits.wall_timeout_ms() {
            effective_facts.deadline_reached = true;
        }
        if input.stdout_truncated || input.stderr_truncated {
            effective_facts.framing_or_binding_failed = true;
        }
        let host_outcome = classify_host_outcome_v1(&effective_facts);
        let result_framed_len = input
            .result
            .map(DockProcessResultV1::framed_json_bytes)
            .transpose()?
            .map(|bytes| bytes.len() as u64);
        match (host_outcome, input.result) {
            (DockHostOutcomeV1::Completed, Some(result))
                if result.status == DockProcessResultStatusV1::Completed
                    && input.exit_code == Some(0)
                    && result_framed_len == Some(input.stdout_bytes) =>
            {
                result.validate_against(input.request)?;
            }
            (DockHostOutcomeV1::Refused, Some(result))
                if result.status == DockProcessResultStatusV1::Refused
                    && input.exit_code == Some(0)
                    && result_framed_len == Some(input.stdout_bytes) =>
            {
                result.validate_against(input.request)?;
            }
            (DockHostOutcomeV1::Failed, Some(result))
                if (!input.facts.cleanup_confirmed || !input.facts.durability_confirmed)
                    && !input.facts.spawn_or_process_failed
                    && !input.facts.framing_or_binding_failed
                    && input.exit_code == Some(0)
                    && input.facts.child_status == Some(result.status)
                    && !input.stdout_truncated
                    && !input.stderr_truncated
                    && result_framed_len == Some(input.stdout_bytes) =>
            {
                result.validate_against(input.request)?;
            }
            (
                DockHostOutcomeV1::TimedOut
                | DockHostOutcomeV1::Cancelled
                | DockHostOutcomeV1::ProtocolError,
                None,
            ) => {}
            (DockHostOutcomeV1::Failed, None) => {}
            _ => return Err(DockErrorV1::InvalidInvariant),
        }
        let candidates = if host_outcome == DockHostOutcomeV1::Completed {
            let result = input.result.ok_or(DockErrorV1::InvalidInvariant)?;
            result.candidates.as_slice()
        } else {
            &[]
        };
        let candidate_observation_bundle_fingerprint = canonical_fingerprint(&candidates)?;
        Self::from_parts(
            input.request.binding()?,
            input.result.map(DockProcessResultV1::binding).transpose()?,
            input.request.manifest.clone(),
            input.request.implementation_bundle.clone(),
            input.request.run.evaluation_run().clone(),
            host_outcome,
            input.exit_code,
            input.wall_time_ms,
            input.stdout_bytes,
            input.stdout_truncated,
            input.stderr_bytes,
            input.stderr_truncated,
            if input.facts.cleanup_confirmed {
                DockCleanupDispositionV1::Confirmed
            } else {
                DockCleanupDispositionV1::Uncertain
            },
            if input.facts.durability_confirmed {
                DockDurabilityDispositionV1::Confirmed
            } else {
                DockDurabilityDispositionV1::Uncertain
            },
            limits.clone(),
            input.diagnostics,
            candidate_observation_bundle_fingerprint,
            candidates.len(),
        )
    }
    #[allow(clippy::too_many_arguments)]
    fn from_parts(
        request: ContentAddressedBindingV1,
        result: Option<ContentAddressedBindingV1>,
        manifest: ExactBindingV1,
        implementation_bundle: ExactBindingV1,
        evaluation_run: ContentAddressedBindingV1,
        host_outcome: DockHostOutcomeV1,
        exit_code: Option<i32>,
        wall_time_ms: u64,
        stdout_bytes: u64,
        stdout_truncated: bool,
        stderr_bytes: u64,
        stderr_truncated: bool,
        cleanup: DockCleanupDispositionV1,
        durability: DockDurabilityDispositionV1,
        effective_limits: DockProtocolLimitsV1,
        diagnostics: Vec<DockDiagnosticV1>,
        candidate_observation_bundle_fingerprint: FingerprintV1,
        candidate_observation_count: usize,
    ) -> Result<Self, DockErrorV1> {
        let result_presence_is_valid = match host_outcome {
            DockHostOutcomeV1::Completed | DockHostOutcomeV1::Refused => result.is_some(),
            DockHostOutcomeV1::Failed => true,
            DockHostOutcomeV1::TimedOut
            | DockHostOutcomeV1::Cancelled
            | DockHostOutcomeV1::ProtocolError => result.is_none(),
        };
        let dispositions_are_valid = host_outcome == DockHostOutcomeV1::Failed
            || (cleanup == DockCleanupDispositionV1::Confirmed
                && durability == DockDurabilityDispositionV1::Confirmed);
        let time_bound = effective_limits
            .wall_timeout_ms()
            .checked_add(effective_limits.cancellation_grace_ms())
            .ok_or(DockErrorV1::BoundExceeded)?;
        if wall_time_ms > time_bound
            || stdout_bytes > (effective_limits.result_bytes() as u64 + 1)
            || stderr_bytes > effective_limits.stderr_bytes() as u64
            || diagnostics.len() > effective_limits.diagnostic_count()
            || diagnostics
                .iter()
                .any(|diagnostic| diagnostic.message.len() > effective_limits.diagnostic_bytes())
            || candidate_observation_count > effective_limits.candidate_count()
            || (host_outcome != DockHostOutcomeV1::Completed && candidate_observation_count != 0)
            || !result_presence_is_valid
            || !dispositions_are_valid
            || (matches!(
                host_outcome,
                DockHostOutcomeV1::Completed | DockHostOutcomeV1::Refused
            ) && exit_code != Some(0))
            || (matches!(
                host_outcome,
                DockHostOutcomeV1::Completed | DockHostOutcomeV1::Refused
            ) && (stdout_truncated || stderr_truncated))
        {
            return Err(DockErrorV1::BoundExceeded);
        }
        if candidate_observation_count == 0 {
            let empty_fingerprint = canonical_fingerprint(&Vec::<u8>::new())?;
            if candidate_observation_bundle_fingerprint != empty_fingerprint {
                return Err(DockErrorV1::DerivedValueMismatch);
            }
        }
        let preimage = ExecutionPreimage {
            schema_ref: DOCK_EXECUTION_RECORD_SCHEMA_REF_V1,
            request: &request,
            result: &result,
            manifest: &manifest,
            implementation_bundle: &implementation_bundle,
            evaluation_run: &evaluation_run,
            host_outcome,
            exit_code,
            wall_time_ms,
            stdout_bytes,
            stdout_truncated,
            stderr_bytes,
            stderr_truncated,
            cleanup,
            durability,
            effective_limits: &effective_limits,
            diagnostics: &diagnostics,
            candidate_observation_bundle_fingerprint: &candidate_observation_bundle_fingerprint,
            candidate_observation_count,
        };
        let record_fingerprint = canonical_fingerprint(&preimage)?;
        let record_ref = format!(
            "{}#execution/{}",
            request.reference(),
            fingerprint_hex(&record_fingerprint)
        );
        Ok(Self {
            schema_ref: DOCK_EXECUTION_RECORD_SCHEMA_REF_V1,
            record_ref,
            record_fingerprint,
            request,
            result,
            manifest,
            implementation_bundle,
            evaluation_run,
            host_outcome,
            exit_code,
            wall_time_ms,
            stdout_bytes,
            stdout_truncated,
            stderr_bytes,
            stderr_truncated,
            cleanup,
            durability,
            effective_limits,
            diagnostics,
            candidate_observation_bundle_fingerprint,
            candidate_observation_count,
        })
    }
    pub fn host_outcome(&self) -> DockHostOutcomeV1 {
        self.host_outcome
    }
    pub fn candidate_observation_count(&self) -> usize {
        self.candidate_observation_count
    }
    pub fn record_ref(&self) -> &str {
        &self.record_ref
    }
    pub fn record_fingerprint(&self) -> &FingerprintV1 {
        &self.record_fingerprint
    }
    pub fn binding(&self) -> Result<ContentAddressedBindingV1, DockErrorV1> {
        ContentAddressedBindingV1::new(&self.record_ref, self.record_fingerprint.clone())
            .map_err(|_| DockErrorV1::InvalidIdentity)
    }
    pub fn request(&self) -> &ContentAddressedBindingV1 {
        &self.request
    }
    pub fn result(&self) -> Option<&ContentAddressedBindingV1> {
        self.result.as_ref()
    }
    pub fn manifest(&self) -> &ExactBindingV1 {
        &self.manifest
    }
    pub fn implementation_bundle(&self) -> &ExactBindingV1 {
        &self.implementation_bundle
    }
    pub fn evaluation_run(&self) -> &ContentAddressedBindingV1 {
        &self.evaluation_run
    }
    pub fn cleanup(&self) -> DockCleanupDispositionV1 {
        self.cleanup
    }
    pub fn durability(&self) -> DockDurabilityDispositionV1 {
        self.durability
    }
    pub fn candidate_observation_bundle_fingerprint(&self) -> &FingerprintV1 {
        &self.candidate_observation_bundle_fingerprint
    }
    pub fn canonical_json_bytes(&self) -> Result<Vec<u8>, DockErrorV1> {
        canonical_bytes(self)
    }
}

impl<'de> Deserialize<'de> for DockExecutionRecordV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Raw {
            schema_ref: String,
            record_ref: String,
            record_fingerprint: FingerprintV1,
            request: ContentAddressedBindingV1,
            result: Option<ContentAddressedBindingV1>,
            manifest: ExactBindingV1,
            implementation_bundle: ExactBindingV1,
            evaluation_run: ContentAddressedBindingV1,
            host_outcome: DockHostOutcomeV1,
            exit_code: Option<i32>,
            wall_time_ms: u64,
            stdout_bytes: u64,
            stdout_truncated: bool,
            stderr_bytes: u64,
            stderr_truncated: bool,
            cleanup: DockCleanupDispositionV1,
            durability: DockDurabilityDispositionV1,
            effective_limits: DockProtocolLimitsV1,
            diagnostics: Vec<DockDiagnosticV1>,
            candidate_observation_bundle_fingerprint: FingerprintV1,
            candidate_observation_count: usize,
        }
        let raw = Raw::deserialize(deserializer)?;
        if raw.schema_ref != DOCK_EXECUTION_RECORD_SCHEMA_REF_V1 {
            return Err(D::Error::custom(DockErrorV1::InvalidIdentity));
        }
        let supplied_ref = raw.record_ref;
        let supplied_fp = raw.record_fingerprint;
        let value = Self::from_parts(
            raw.request,
            raw.result,
            raw.manifest,
            raw.implementation_bundle,
            raw.evaluation_run,
            raw.host_outcome,
            raw.exit_code,
            raw.wall_time_ms,
            raw.stdout_bytes,
            raw.stdout_truncated,
            raw.stderr_bytes,
            raw.stderr_truncated,
            raw.cleanup,
            raw.durability,
            raw.effective_limits,
            raw.diagnostics,
            raw.candidate_observation_bundle_fingerprint,
            raw.candidate_observation_count,
        )
        .map_err(D::Error::custom)?;
        if value.record_ref != supplied_ref || value.record_fingerprint != supplied_fp {
            return Err(D::Error::custom(DockErrorV1::DerivedValueMismatch));
        }
        Ok(value)
    }
}

#[derive(Serialize)]
struct ExecutionPreimage<'a> {
    schema_ref: &'static str,
    request: &'a ContentAddressedBindingV1,
    result: &'a Option<ContentAddressedBindingV1>,
    manifest: &'a ExactBindingV1,
    implementation_bundle: &'a ExactBindingV1,
    evaluation_run: &'a ContentAddressedBindingV1,
    host_outcome: DockHostOutcomeV1,
    exit_code: Option<i32>,
    wall_time_ms: u64,
    stdout_bytes: u64,
    stdout_truncated: bool,
    stderr_bytes: u64,
    stderr_truncated: bool,
    cleanup: DockCleanupDispositionV1,
    durability: DockDurabilityDispositionV1,
    effective_limits: &'a DockProtocolLimitsV1,
    diagnostics: &'a [DockDiagnosticV1],
    candidate_observation_bundle_fingerprint: &'a FingerprintV1,
    candidate_observation_count: usize,
}

fn validate_role_order(inputs: &[DockInlineJsonInputV1], max: usize) -> Result<(), DockErrorV1> {
    if inputs.is_empty()
        || inputs.len() > max
        || inputs
            .windows(2)
            .any(|pair| pair[0].role() >= pair[1].role())
    {
        return Err(DockErrorV1::BoundExceeded);
    }
    let aggregate = inputs
        .iter()
        .try_fold(0_usize, |sum, input| {
            sum.checked_add(canonical_bytes(input.content()).ok()?.len())
        })
        .ok_or(DockErrorV1::BoundExceeded)?;
    if aggregate > MAX_INLINE_INPUT_AGGREGATE_BYTES {
        return Err(DockErrorV1::BoundExceeded);
    }
    Ok(())
}

fn validate_inputs(
    inputs: &[DockInlineJsonInputV1],
    manifest: &DockManifestV1,
    limits: &DockProtocolLimitsV1,
) -> Result<(), DockErrorV1> {
    validate_role_order(inputs, limits.input_count())?;
    for input in inputs {
        let definition = manifest
            .supported_inputs()
            .iter()
            .find(|definition| definition.role() == input.role())
            .ok_or(DockErrorV1::Unknown)?;
        if definition.media_type() != input.media_type() || definition.schema() != input.schema() {
            return Err(DockErrorV1::Substituted);
        }
    }
    Ok(())
}
