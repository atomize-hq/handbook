use crate::charter_lifecycle_transition_v11::{
    lifecycle_transition_output_record_v11, validate_lifecycle_transition_bindings_v11,
    ValidatedLifecycleTransitionV11,
};
use crate::{parse_schema_json, DefinitionFingerprint};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::cmp::Ordering;
use std::fmt;

pub(crate) const MAX_POSTURE_RECORD_BYTES_V1: usize = 262_144;
pub(crate) const MAX_POSTURE_CANONICAL_BYTES_V1: u64 = 1_048_576;

const CANONICAL_CHARTER_REF: &str = ".handbook/project/charter.yaml";
const INTAKE_DEFINITION_REF: &str = "handbook.intake.charter@1.0.0";
const INTAKE_DEFINITION_FINGERPRINT: &str =
    "sha256:a92229722f25119c7d91137e1feef4ce51b88ae766ce308b585d37f39eb52d1c";
const DIMENSION_BINDINGS: [(&str, &str); 9] = [
    (
        "speed_vs_quality",
        "/engineering_posture/dimensions/0/level_override",
    ),
    (
        "type_safety_static_analysis",
        "/engineering_posture/dimensions/1/level_override",
    ),
    (
        "testing_rigor",
        "/engineering_posture/dimensions/2/level_override",
    ),
    (
        "scalability_performance",
        "/engineering_posture/dimensions/3/level_override",
    ),
    (
        "reliability_operability",
        "/engineering_posture/dimensions/4/level_override",
    ),
    (
        "security_privacy",
        "/engineering_posture/dimensions/5/level_override",
    ),
    (
        "observability",
        "/engineering_posture/dimensions/6/level_override",
    ),
    (
        "dx_tooling_automation",
        "/engineering_posture/dimensions/7/level_override",
    ),
    (
        "ux_polish_api_usability",
        "/engineering_posture/dimensions/8/level_override",
    ),
];

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PostureRecordValidationErrorV1 {
    detail: String,
}

impl PostureRecordValidationErrorV1 {
    pub(crate) fn new(detail: impl Into<String>) -> Self {
        Self {
            detail: detail.into(),
        }
    }

    pub(crate) fn detail(&self) -> &str {
        &self.detail
    }
}

impl fmt::Display for PostureRecordValidationErrorV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.detail)
    }
}

impl std::error::Error for PostureRecordValidationErrorV1 {}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PairV1 {
    #[serde(rename = "ref")]
    pub(crate) reference: String,
    pub(crate) fingerprint: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct CanonicalDocumentV1 {
    #[serde(rename = "ref")]
    pub(crate) reference: String,
    pub(crate) fingerprint: String,
    pub(crate) document_sha256: String,
    pub(crate) byte_length: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct OutputRecordV1 {
    #[serde(rename = "ref")]
    pub(crate) reference: String,
    pub(crate) fingerprint: String,
    pub(crate) document_sha256: String,
    pub(crate) byte_length: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct EmptyExtensionsV1 {}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct AuthorityHeadV1 {
    pub(crate) kind: String,
    pub(crate) source: PairV1,
    pub(crate) canonical: CanonicalDocumentV1,
    pub(crate) lifecycle_transition: PairV1,
    pub(crate) promotion_ancestor: PairV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PostureChangeV1 {
    pub(crate) dimension_id: String,
    pub(crate) dimension_index: u8,
    pub(crate) authority_path: String,
    pub(crate) operation: String,
    pub(crate) baseline_level: u8,
    pub(crate) expected_stored_value: Option<u8>,
    pub(crate) expected_effective_level: u8,
    pub(crate) proposed_stored_value: Option<u8>,
    pub(crate) proposed_effective_level: u8,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PostureReassessmentV1 {
    pub(crate) intake_definition: PairV1,
    pub(crate) affected_coverage_ids: Vec<String>,
    pub(crate) validation_result_inputs: Vec<PairV1>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct KernelReplayDimensionV1 {
    pub(crate) dimension_id: String,
    pub(crate) source_effective_level: u8,
    pub(crate) resulting_effective_level: u8,
    pub(crate) floor: u8,
    pub(crate) red_line_refs: Vec<String>,
    pub(crate) trigger_refs: Vec<String>,
    pub(crate) allowed_shortcut_refs: Vec<String>,
    pub(crate) proof_obligation_refs: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct KernelReplayV1 {
    pub(crate) constitutional_artifact_ref: String,
    pub(crate) source_authority_fingerprint: String,
    pub(crate) resulting_authority_fingerprint: String,
    pub(crate) source_input_fingerprint: String,
    pub(crate) resulting_input_fingerprint: String,
    pub(crate) profile_input: PairV1,
    pub(crate) override_inputs: Vec<PairV1>,
    pub(crate) condition_inputs: Vec<PairV1>,
    pub(crate) contract_inputs: Vec<PairV1>,
    pub(crate) evidence_inputs: Vec<PairV1>,
    pub(crate) snapshot_inputs: Vec<PairV1>,
    pub(crate) freshness_basis: Option<PairV1>,
    pub(crate) dimensions: Vec<KernelReplayDimensionV1>,
    pub(crate) applicable_scope_refs: Vec<String>,
    pub(crate) omitted_condition_refs: Vec<String>,
    pub(crate) unresolved_condition_refs: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PostureTransitionRecordV1 {
    pub(crate) schema_id: String,
    pub(crate) schema_version: String,
    pub(crate) transition_id: String,
    pub(crate) recommendation: PairV1,
    pub(crate) source_kernel: PairV1,
    pub(crate) evaluation_policy: PairV1,
    pub(crate) target_authority_ref: String,
    pub(crate) target_authority_class: String,
    pub(crate) prior_authority_head: AuthorityHeadV1,
    pub(crate) expected_canonical: CanonicalDocumentV1,
    pub(crate) change: PostureChangeV1,
    pub(crate) approval_inputs: Vec<PairV1>,
    pub(crate) authorized_by_ref: String,
    pub(crate) reassessment: PostureReassessmentV1,
    pub(crate) kernel_replay: KernelReplayV1,
    pub(crate) resulting_canonical: CanonicalDocumentV1,
    pub(crate) resulting_kernel: PairV1,
    pub(crate) effective_at_utc: String,
    pub(crate) extensions: EmptyExtensionsV1,
    pub(crate) transition_fingerprint: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PostureTransitionDraftV1 {
    pub(crate) recommendation: PairV1,
    pub(crate) source_kernel: PairV1,
    pub(crate) evaluation_policy: PairV1,
    pub(crate) prior_authority_head: AuthorityHeadV1,
    pub(crate) expected_canonical: CanonicalDocumentV1,
    pub(crate) change: PostureChangeV1,
    pub(crate) approval_inputs: Vec<PairV1>,
    pub(crate) authorized_by_ref: String,
    pub(crate) reassessment: PostureReassessmentV1,
    pub(crate) kernel_replay: KernelReplayV1,
    pub(crate) resulting_canonical: CanonicalDocumentV1,
    pub(crate) resulting_kernel: PairV1,
    pub(crate) effective_at_utc: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ValidatedPostureTransitionV1 {
    pub(crate) record: PostureTransitionRecordV1,
    pub(crate) raw_bytes: Vec<u8>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PostureAuthorityInputsV1 {
    pub(crate) recommendation: PairV1,
    pub(crate) source_kernel: PairV1,
    pub(crate) evaluation_policy: PairV1,
    pub(crate) approval_inputs: Vec<PairV1>,
    pub(crate) authorized_by_ref: String,
    pub(crate) reassessment: PostureReassessmentV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PostureTransactionOutputsV1 {
    pub(crate) canonical: CanonicalDocumentV1,
    pub(crate) posture_transition: OutputRecordV1,
    pub(crate) lifecycle_transition: OutputRecordV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PostureTransactionRecoveryV1 {
    pub(crate) old_canonical_status: String,
    pub(crate) old_canonical: CanonicalDocumentV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PostureTransactionIntentV1 {
    pub(crate) schema_id: String,
    pub(crate) schema_version: String,
    pub(crate) transaction_id: String,
    pub(crate) mutation_mode: String,
    pub(crate) basis_head: AuthorityHeadV1,
    pub(crate) expected_canonical: CanonicalDocumentV1,
    pub(crate) change: PostureChangeV1,
    pub(crate) authority_inputs: PostureAuthorityInputsV1,
    pub(crate) outputs: PostureTransactionOutputsV1,
    pub(crate) kernel_replay: KernelReplayV1,
    pub(crate) recovery: PostureTransactionRecoveryV1,
    pub(crate) intent_fingerprint: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ValidatedPostureTransactionIntentV1 {
    pub(crate) record: PostureTransactionIntentV1,
    pub(crate) raw_bytes: Vec<u8>,
}

pub(crate) fn parse_posture_transition_v1(
    bytes: &[u8],
) -> Result<ValidatedPostureTransitionV1, PostureRecordValidationErrorV1> {
    let record = parse_jcs_lf_v1(bytes, MAX_POSTURE_RECORD_BYTES_V1, "posture transition 1.0")?;
    validate_posture_transition_record_v1(&record)?;
    Ok(ValidatedPostureTransitionV1 {
        record,
        raw_bytes: bytes.to_vec(),
    })
}

pub(crate) fn construct_posture_transition_v1(
    draft: PostureTransitionDraftV1,
) -> Result<ValidatedPostureTransitionV1, PostureRecordValidationErrorV1> {
    let mut record = PostureTransitionRecordV1 {
        schema_id: "handbook.posture-transition".to_owned(),
        schema_version: "1.0".to_owned(),
        transition_id: String::new(),
        recommendation: draft.recommendation,
        source_kernel: draft.source_kernel,
        evaluation_policy: draft.evaluation_policy,
        target_authority_ref: CANONICAL_CHARTER_REF.to_owned(),
        target_authority_class: "constitutional_root".to_owned(),
        prior_authority_head: draft.prior_authority_head,
        expected_canonical: draft.expected_canonical,
        change: draft.change,
        approval_inputs: draft.approval_inputs,
        authorized_by_ref: draft.authorized_by_ref,
        reassessment: draft.reassessment,
        kernel_replay: draft.kernel_replay,
        resulting_canonical: draft.resulting_canonical,
        resulting_kernel: draft.resulting_kernel,
        effective_at_utc: draft.effective_at_utc,
        extensions: EmptyExtensionsV1 {},
        transition_fingerprint: String::new(),
    };
    let fingerprint = fingerprint_excluding_v1(
        &record,
        &[
            "transition_id",
            "transition_fingerprint",
            "effective_at_utc",
        ],
        "posture transition 1.0",
    )?;
    record.transition_id = format!(
        "posture-transition_{}",
        fingerprint
            .strip_prefix("sha256:")
            .expect("validated SHA-256 prefix")
    );
    record.transition_fingerprint = fingerprint;
    validate_posture_transition_record_v1(&record)?;
    let raw_bytes = encode_jcs_lf_v1(
        &record,
        MAX_POSTURE_RECORD_BYTES_V1,
        "posture transition 1.0",
    )?;
    Ok(ValidatedPostureTransitionV1 { record, raw_bytes })
}

pub(crate) fn posture_transition_pair_v1(transition: &ValidatedPostureTransitionV1) -> PairV1 {
    PairV1 {
        reference: format!(
            "posture-transitions/{}.json",
            transition.record.transition_id
        ),
        fingerprint: transition.record.transition_fingerprint.clone(),
    }
}

pub(crate) fn posture_transition_output_record_v1(
    transition: &ValidatedPostureTransitionV1,
) -> OutputRecordV1 {
    OutputRecordV1 {
        reference: posture_transition_pair_v1(transition).reference,
        fingerprint: transition.record.transition_fingerprint.clone(),
        document_sha256: DefinitionFingerprint::from_bytes(&transition.raw_bytes).to_string(),
        byte_length: transition.raw_bytes.len() as u64,
    }
}

pub(crate) fn posture_transition_marker_v1(record_bytes: &[u8]) -> Vec<u8> {
    format!("sha256:{:x}\n", Sha256::digest(record_bytes)).into_bytes()
}

pub(crate) fn parse_posture_transaction_intent_v1(
    bytes: &[u8],
) -> Result<ValidatedPostureTransactionIntentV1, PostureRecordValidationErrorV1> {
    let record = parse_jcs_lf_v1(
        bytes,
        MAX_POSTURE_RECORD_BYTES_V1,
        "posture transaction intent 1.0",
    )?;
    validate_posture_transaction_intent_record_v1(&record)?;
    Ok(ValidatedPostureTransactionIntentV1 {
        record,
        raw_bytes: bytes.to_vec(),
    })
}

pub(crate) fn construct_posture_transaction_intent_v1<F>(
    posture: &ValidatedPostureTransitionV1,
    lifecycle: &ValidatedLifecycleTransitionV11,
    old_canonical_bytes: &[u8],
    new_canonical_bytes: &[u8],
    mut transaction_id_available: F,
) -> Result<ValidatedPostureTransactionIntentV1, PostureRecordValidationErrorV1>
where
    F: FnMut(&str) -> bool,
{
    validate_lifecycle_transition_bindings_v11(lifecycle, posture)?;
    validate_exact_canonical_bytes_v1(
        old_canonical_bytes,
        &posture.record.expected_canonical,
        "old canonical",
    )?;
    validate_exact_canonical_bytes_v1(
        new_canonical_bytes,
        &posture.record.resulting_canonical,
        "new canonical",
    )?;

    let record = PostureTransactionIntentV1 {
        schema_id: "handbook.charter-posture-transaction-intent".to_owned(),
        schema_version: "1.0".to_owned(),
        transaction_id: allocate_transaction_id_v1(&mut transaction_id_available)?,
        mutation_mode: "single_dimension_level_override_replace".to_owned(),
        basis_head: posture.record.prior_authority_head.clone(),
        expected_canonical: posture.record.expected_canonical.clone(),
        change: posture.record.change.clone(),
        authority_inputs: PostureAuthorityInputsV1 {
            recommendation: posture.record.recommendation.clone(),
            source_kernel: posture.record.source_kernel.clone(),
            evaluation_policy: posture.record.evaluation_policy.clone(),
            approval_inputs: posture.record.approval_inputs.clone(),
            authorized_by_ref: posture.record.authorized_by_ref.clone(),
            reassessment: posture.record.reassessment.clone(),
        },
        outputs: PostureTransactionOutputsV1 {
            canonical: posture.record.resulting_canonical.clone(),
            posture_transition: posture_transition_output_record_v1(posture),
            lifecycle_transition: lifecycle_transition_output_record_v11(lifecycle),
        },
        kernel_replay: posture.record.kernel_replay.clone(),
        recovery: PostureTransactionRecoveryV1 {
            old_canonical_status: "present".to_owned(),
            old_canonical: posture.record.expected_canonical.clone(),
        },
        intent_fingerprint: String::new(),
    };
    let intent_fingerprint = fingerprint_excluding_v1(
        &record,
        &["intent_fingerprint"],
        "posture transaction intent 1.0",
    )?;
    let record = PostureTransactionIntentV1 {
        intent_fingerprint,
        ..record
    };
    validate_posture_transaction_intent_record_v1(&record)?;
    let raw_bytes = encode_jcs_lf_v1(
        &record,
        MAX_POSTURE_RECORD_BYTES_V1,
        "posture transaction intent 1.0",
    )?;
    Ok(ValidatedPostureTransactionIntentV1 { record, raw_bytes })
}

pub(crate) fn validate_posture_transaction_intent_bindings_v1(
    intent: &ValidatedPostureTransactionIntentV1,
    posture: &ValidatedPostureTransitionV1,
    lifecycle: &ValidatedLifecycleTransitionV11,
    old_canonical_bytes: &[u8],
    new_canonical_bytes: &[u8],
) -> Result<(), PostureRecordValidationErrorV1> {
    validate_posture_transaction_intent_record_v1(&intent.record)?;
    validate_posture_transition_record_v1(&posture.record)?;
    validate_lifecycle_transition_bindings_v11(lifecycle, posture)?;
    validate_exact_canonical_bytes_v1(
        old_canonical_bytes,
        &intent.record.recovery.old_canonical,
        "intent old canonical",
    )?;
    validate_exact_canonical_bytes_v1(
        new_canonical_bytes,
        &intent.record.outputs.canonical,
        "intent new canonical",
    )?;

    let record = &intent.record;
    if record.basis_head != posture.record.prior_authority_head
        || record.expected_canonical != posture.record.expected_canonical
        || record.basis_head.canonical != record.expected_canonical
        || record.change != posture.record.change
        || record.authority_inputs.recommendation != posture.record.recommendation
        || record.authority_inputs.source_kernel != posture.record.source_kernel
        || record.authority_inputs.evaluation_policy != posture.record.evaluation_policy
        || record.authority_inputs.approval_inputs != posture.record.approval_inputs
        || record.authority_inputs.authorized_by_ref != posture.record.authorized_by_ref
        || record.authority_inputs.reassessment != posture.record.reassessment
        || record.kernel_replay != posture.record.kernel_replay
        || record.outputs.canonical != posture.record.resulting_canonical
        || record.recovery.old_canonical != posture.record.expected_canonical
    {
        return Err(invalid(
            "posture transaction intent has substituted posture bindings",
        ));
    }
    if record.outputs.posture_transition != posture_transition_output_record_v1(posture)
        || record.outputs.lifecycle_transition != lifecycle_transition_output_record_v11(lifecycle)
    {
        return Err(invalid(
            "posture transaction intent output record bindings are substituted",
        ));
    }
    Ok(())
}

pub(crate) fn posture_transaction_intent_marker_v1(intent_bytes: &[u8]) -> Vec<u8> {
    format!("sha256:{:x}\n", Sha256::digest(intent_bytes)).into_bytes()
}

pub(crate) fn parse_jcs_lf_v1<T>(
    bytes: &[u8],
    max_bytes: usize,
    label: &str,
) -> Result<T, PostureRecordValidationErrorV1>
where
    T: DeserializeOwned + Serialize,
{
    if bytes.is_empty()
        || bytes.len() > max_bytes
        || !bytes.ends_with(b"\n")
        || bytes[..bytes.len() - 1].ends_with(b"\n")
    {
        return Err(invalid(format!(
            "{label} must be bounded exact JCS plus one LF"
        )));
    }
    let jcs = &bytes[..bytes.len() - 1];
    let value = parse_schema_json(jcs)
        .map_err(|_| invalid(format!("{label} is not duplicate-free JSON")))?;
    let canonical = serde_json_canonicalizer::to_vec(&value)
        .map_err(|_| invalid(format!("{label} cannot be canonicalized")))?;
    if canonical != jcs {
        return Err(invalid(format!("{label} bytes are not exact RFC 8785 JCS")));
    }
    let record: T = serde_json::from_value(value.clone())
        .map_err(|_| invalid(format!("{label} violates its closed shape")))?;
    if serde_json::to_value(&record)
        .map_err(|_| invalid(format!("{label} cannot be reserialized")))?
        != value
    {
        return Err(invalid(format!("{label} omits a required nullable field")));
    }
    Ok(record)
}

pub(crate) fn encode_jcs_lf_v1<T>(
    record: &T,
    max_bytes: usize,
    label: &str,
) -> Result<Vec<u8>, PostureRecordValidationErrorV1>
where
    T: Serialize,
{
    let mut bytes = serde_json_canonicalizer::to_vec(record)
        .map_err(|_| invalid(format!("{label} cannot be canonicalized")))?;
    bytes.push(b'\n');
    if bytes.len() > max_bytes {
        return Err(invalid(format!("{label} exceeds its byte bound")));
    }
    Ok(bytes)
}

pub(crate) fn fingerprint_excluding_v1<T>(
    record: &T,
    excluded_fields: &[&str],
    label: &str,
) -> Result<String, PostureRecordValidationErrorV1>
where
    T: Serialize,
{
    let mut value = serde_json::to_value(record)
        .map_err(|_| invalid(format!("{label} fingerprint preimage cannot be serialized")))?;
    let object = value
        .as_object_mut()
        .ok_or_else(|| invalid(format!("{label} fingerprint preimage is not an object")))?;
    for field in excluded_fields {
        object.remove(*field);
    }
    DefinitionFingerprint::from_json_value(&value)
        .map(|fingerprint| fingerprint.to_string())
        .map_err(|_| invalid(format!("{label} fingerprint preimage is not canonical")))
}

pub(crate) fn validate_pair_v1(
    value: &PairV1,
    label: &str,
) -> Result<(), PostureRecordValidationErrorV1> {
    validate_safe_ref_v1(&value.reference, 512, label)?;
    validate_fingerprint_v1(&value.fingerprint, label)
}

pub(crate) fn validate_fingerprint_v1(
    value: &str,
    label: &str,
) -> Result<(), PostureRecordValidationErrorV1> {
    if !is_fingerprint(value) {
        return Err(invalid(format!(
            "{label} is not a lowercase SHA-256 fingerprint"
        )));
    }
    Ok(())
}

pub(crate) fn validate_safe_ref_v1(
    value: &str,
    max_bytes: usize,
    label: &str,
) -> Result<(), PostureRecordValidationErrorV1> {
    if value.is_empty()
        || value.len() > max_bytes
        || value.trim() != value
        || value.contains('\\')
        || value.chars().any(char::is_control)
        || value
            .split('/')
            .any(|segment| matches!(segment, "" | "." | ".."))
    {
        return Err(invalid(format!("{label} is not a safe bounded reference")));
    }
    Ok(())
}

pub(crate) fn validate_content_ref_v1(
    value: &PairV1,
    prefix: &str,
    suffix: &str,
    label: &str,
) -> Result<(), PostureRecordValidationErrorV1> {
    validate_pair_v1(value, label)?;
    let Some(hex) = value
        .reference
        .strip_prefix(prefix)
        .and_then(|remainder| remainder.strip_suffix(suffix))
    else {
        return Err(invalid(format!(
            "{label} has an invalid content reference grammar"
        )));
    };
    if !is_lowercase_sha256_hex(hex) || value.fingerprint != format!("sha256:{hex}") {
        return Err(invalid(format!(
            "{label} content reference and fingerprint disagree"
        )));
    }
    Ok(())
}

pub(crate) fn validate_output_record_v1(
    value: &OutputRecordV1,
    prefix: &str,
    suffix: &str,
    label: &str,
) -> Result<(), PostureRecordValidationErrorV1> {
    validate_content_ref_v1(
        &PairV1 {
            reference: value.reference.clone(),
            fingerprint: value.fingerprint.clone(),
        },
        prefix,
        suffix,
        label,
    )?;
    validate_fingerprint_v1(&value.document_sha256, label)?;
    if value.byte_length == 0 || value.byte_length > MAX_POSTURE_RECORD_BYTES_V1 as u64 {
        return Err(invalid(format!("{label} has an invalid byte length")));
    }
    Ok(())
}

fn validate_posture_transition_record_v1(
    record: &PostureTransitionRecordV1,
) -> Result<(), PostureRecordValidationErrorV1> {
    if record.schema_id != "handbook.posture-transition"
        || record.schema_version != "1.0"
        || record.target_authority_ref != CANONICAL_CHARTER_REF
        || record.target_authority_class != "constitutional_root"
    {
        return Err(invalid(
            "posture transition schema or target constants are invalid",
        ));
    }
    validate_transition_id_v1(
        &record.transition_id,
        "posture-transition_",
        "posture transition ID",
    )?;
    validate_pair_v1(&record.recommendation, "posture recommendation")?;
    validate_pair_v1(&record.source_kernel, "posture source kernel")?;
    validate_pair_v1(&record.evaluation_policy, "posture evaluation policy")?;
    validate_authority_head_v1(&record.prior_authority_head)?;
    validate_canonical_document_v1(&record.expected_canonical, "expected canonical")?;
    if record.prior_authority_head.canonical != record.expected_canonical {
        return Err(invalid(
            "posture authority head canonical differs from expected canonical",
        ));
    }
    validate_change_v1(&record.change)?;
    validate_sorted_pairs_v1(&record.approval_inputs, 1, 16, "posture approval inputs")?;
    validate_safe_ref_v1(&record.authorized_by_ref, 256, "posture authorized-by ref")?;
    validate_reassessment_v1(&record.reassessment)?;
    validate_canonical_document_v1(&record.resulting_canonical, "resulting canonical")?;
    if record.resulting_canonical.fingerprint == record.expected_canonical.fingerprint {
        return Err(invalid(
            "posture resulting canonical must differ from its expected canonical",
        ));
    }
    validate_kernel_replay_v1(
        &record.kernel_replay,
        &record.change,
        &record.expected_canonical,
        &record.resulting_canonical,
    )?;
    validate_pair_v1(&record.resulting_kernel, "posture resulting kernel")?;
    validate_utc_second_v1(&record.effective_at_utc, "posture effective time")?;
    validate_fingerprint_v1(
        &record.transition_fingerprint,
        "posture transition fingerprint",
    )?;
    let computed = fingerprint_excluding_v1(
        record,
        &[
            "transition_id",
            "transition_fingerprint",
            "effective_at_utc",
        ],
        "posture transition 1.0",
    )?;
    if record.transition_fingerprint != computed {
        return Err(invalid(
            "posture transition fingerprint has an invalid semantic preimage",
        ));
    }
    let expected_id = format!(
        "posture-transition_{}",
        record
            .transition_fingerprint
            .strip_prefix("sha256:")
            .expect("validated SHA-256 prefix")
    );
    if record.transition_id != expected_id {
        return Err(invalid(
            "posture transition ID does not derive from its fingerprint",
        ));
    }
    Ok(())
}

fn validate_posture_transaction_intent_record_v1(
    record: &PostureTransactionIntentV1,
) -> Result<(), PostureRecordValidationErrorV1> {
    if record.schema_id != "handbook.charter-posture-transaction-intent"
        || record.schema_version != "1.0"
        || record.mutation_mode != "single_dimension_level_override_replace"
        || record.recovery.old_canonical_status != "present"
    {
        return Err(invalid(
            "posture transaction intent schema or constants are invalid",
        ));
    }
    validate_transaction_id_v1(&record.transaction_id)?;
    validate_authority_head_v1(&record.basis_head)?;
    validate_canonical_document_v1(&record.expected_canonical, "intent expected canonical")?;
    validate_change_v1(&record.change)?;
    validate_authority_inputs_v1(&record.authority_inputs)?;
    validate_canonical_document_v1(&record.outputs.canonical, "intent output canonical")?;
    validate_output_record_v1(
        &record.outputs.posture_transition,
        "posture-transitions/posture-transition_",
        ".json",
        "intent posture output",
    )?;
    validate_output_record_v1(
        &record.outputs.lifecycle_transition,
        "lifecycle-transitions/lifecycle-transition_",
        ".json",
        "intent lifecycle output",
    )?;
    validate_kernel_replay_v1(
        &record.kernel_replay,
        &record.change,
        &record.expected_canonical,
        &record.outputs.canonical,
    )?;
    validate_canonical_document_v1(&record.recovery.old_canonical, "intent old canonical")?;
    if record.basis_head.canonical != record.expected_canonical
        || record.recovery.old_canonical != record.expected_canonical
    {
        return Err(invalid(
            "posture transaction intent basis/recovery canonical values differ",
        ));
    }
    validate_fingerprint_v1(&record.intent_fingerprint, "posture intent fingerprint")?;
    let computed = fingerprint_excluding_v1(
        record,
        &["intent_fingerprint"],
        "posture transaction intent 1.0",
    )?;
    if record.intent_fingerprint != computed {
        return Err(invalid(
            "posture intent fingerprint has an invalid complete preimage",
        ));
    }
    Ok(())
}

fn validate_authority_inputs_v1(
    value: &PostureAuthorityInputsV1,
) -> Result<(), PostureRecordValidationErrorV1> {
    validate_pair_v1(&value.recommendation, "intent recommendation")?;
    validate_pair_v1(&value.source_kernel, "intent source kernel")?;
    validate_pair_v1(&value.evaluation_policy, "intent evaluation policy")?;
    validate_sorted_pairs_v1(&value.approval_inputs, 1, 16, "intent approval inputs")?;
    validate_safe_ref_v1(&value.authorized_by_ref, 256, "intent authorized-by ref")?;
    validate_reassessment_v1(&value.reassessment)
}

fn validate_authority_head_v1(
    value: &AuthorityHeadV1,
) -> Result<(), PostureRecordValidationErrorV1> {
    match value.kind.as_str() {
        "promotion" => validate_content_ref_v1(
            &value.source,
            "promotions/promotion_",
            ".json",
            "authority head promotion source",
        )?,
        "posture_transition" => validate_content_ref_v1(
            &value.source,
            "posture-transitions/posture-transition_",
            ".json",
            "authority head posture source",
        )?,
        _ => return Err(invalid("authority head kind is unsupported")),
    }
    validate_canonical_document_v1(&value.canonical, "authority head canonical")?;
    validate_content_ref_v1(
        &value.lifecycle_transition,
        "lifecycle-transitions/lifecycle-transition_",
        ".json",
        "authority head lifecycle transition",
    )?;
    validate_content_ref_v1(
        &value.promotion_ancestor,
        "promotions/promotion_",
        ".json",
        "authority head promotion ancestor",
    )
}

fn validate_canonical_document_v1(
    value: &CanonicalDocumentV1,
    label: &str,
) -> Result<(), PostureRecordValidationErrorV1> {
    if value.reference != CANONICAL_CHARTER_REF
        || value.fingerprint != value.document_sha256
        || value.byte_length == 0
        || value.byte_length > MAX_POSTURE_CANONICAL_BYTES_V1
    {
        return Err(invalid(format!("{label} has an invalid canonical binding")));
    }
    validate_safe_ref_v1(&value.reference, 512, label)?;
    validate_fingerprint_v1(&value.fingerprint, label)?;
    validate_fingerprint_v1(&value.document_sha256, label)
}

fn validate_change_v1(value: &PostureChangeV1) -> Result<(), PostureRecordValidationErrorV1> {
    let Some((expected_id, expected_path)) = DIMENSION_BINDINGS.get(value.dimension_index as usize)
    else {
        return Err(invalid("posture change dimension index is out of bounds"));
    };
    if value.dimension_id != *expected_id
        || value.authority_path != *expected_path
        || value.operation != "replace"
        || !is_level(value.baseline_level)
        || !is_level(value.expected_effective_level)
        || !is_level(value.proposed_effective_level)
        || value
            .expected_stored_value
            .is_some_and(|level| !is_level(level))
        || value
            .proposed_stored_value
            .is_some_and(|level| !is_level(level))
        || value.expected_effective_level
            != value.expected_stored_value.unwrap_or(value.baseline_level)
        || value.proposed_effective_level
            != value.proposed_stored_value.unwrap_or(value.baseline_level)
        || value.expected_effective_level == value.proposed_effective_level
    {
        return Err(invalid(
            "posture change does not match its fixed leaf algebra",
        ));
    }
    Ok(())
}

pub(crate) fn validate_reassessment_v1(
    value: &PostureReassessmentV1,
) -> Result<(), PostureRecordValidationErrorV1> {
    if value.intake_definition.reference != INTAKE_DEFINITION_REF
        || value.intake_definition.fingerprint != INTAKE_DEFINITION_FINGERPRINT
        || value.affected_coverage_ids != ["engineering_posture.dimensions"]
    {
        return Err(invalid("posture reassessment constants are invalid"));
    }
    validate_pair_v1(&value.intake_definition, "posture intake definition")?;
    validate_sorted_pairs_v1(
        &value.validation_result_inputs,
        1,
        16,
        "posture validation result inputs",
    )
}

fn validate_kernel_replay_v1(
    value: &KernelReplayV1,
    change: &PostureChangeV1,
    expected_canonical: &CanonicalDocumentV1,
    resulting_canonical: &CanonicalDocumentV1,
) -> Result<(), PostureRecordValidationErrorV1> {
    if value.constitutional_artifact_ref != CANONICAL_CHARTER_REF
        || value.source_authority_fingerprint != expected_canonical.fingerprint
        || value.resulting_authority_fingerprint != resulting_canonical.fingerprint
    {
        return Err(invalid("kernel replay authority bindings are invalid"));
    }
    validate_safe_ref_v1(
        &value.constitutional_artifact_ref,
        512,
        "kernel replay constitutional ref",
    )?;
    for (fingerprint, label) in [
        (
            &value.source_authority_fingerprint,
            "kernel replay source authority fingerprint",
        ),
        (
            &value.resulting_authority_fingerprint,
            "kernel replay resulting authority fingerprint",
        ),
        (
            &value.source_input_fingerprint,
            "kernel replay source input fingerprint",
        ),
        (
            &value.resulting_input_fingerprint,
            "kernel replay resulting input fingerprint",
        ),
    ] {
        validate_fingerprint_v1(fingerprint, label)?;
    }
    validate_pair_v1(&value.profile_input, "kernel replay profile input")?;
    for (pairs, label) in [
        (&value.override_inputs, "kernel replay override inputs"),
        (&value.condition_inputs, "kernel replay condition inputs"),
        (&value.contract_inputs, "kernel replay contract inputs"),
        (&value.evidence_inputs, "kernel replay evidence inputs"),
        (&value.snapshot_inputs, "kernel replay snapshot inputs"),
    ] {
        validate_sorted_pairs_v1(pairs, 0, 256, label)?;
    }
    if let Some(freshness_basis) = &value.freshness_basis {
        validate_pair_v1(freshness_basis, "kernel replay freshness basis")?;
    }
    if value.dimensions.len() != DIMENSION_BINDINGS.len() {
        return Err(invalid(
            "kernel replay dimensions are not the fixed nine-row map",
        ));
    }
    for (index, dimension) in value.dimensions.iter().enumerate() {
        let (expected_id, _) = DIMENSION_BINDINGS[index];
        if dimension.dimension_id != expected_id
            || !is_level(dimension.source_effective_level)
            || !is_level(dimension.resulting_effective_level)
            || !is_level(dimension.floor)
            || (index != change.dimension_index as usize
                && dimension.source_effective_level != dimension.resulting_effective_level)
            || (index == change.dimension_index as usize
                && (dimension.source_effective_level != change.expected_effective_level
                    || dimension.resulting_effective_level != change.proposed_effective_level))
        {
            return Err(invalid(
                "kernel replay dimension values are invalid or substituted",
            ));
        }
        for (references, label) in [
            (&dimension.red_line_refs, "kernel replay red-line refs"),
            (&dimension.trigger_refs, "kernel replay trigger refs"),
            (
                &dimension.allowed_shortcut_refs,
                "kernel replay allowed-shortcut refs",
            ),
            (
                &dimension.proof_obligation_refs,
                "kernel replay proof-obligation refs",
            ),
        ] {
            validate_sorted_refs_v1(references, 0, 256, label)?;
        }
    }
    for (references, label) in [
        (
            &value.applicable_scope_refs,
            "kernel replay applicable-scope refs",
        ),
        (
            &value.omitted_condition_refs,
            "kernel replay omitted-condition refs",
        ),
        (
            &value.unresolved_condition_refs,
            "kernel replay unresolved-condition refs",
        ),
    ] {
        validate_sorted_refs_v1(references, 0, 256, label)?;
    }
    Ok(())
}

fn validate_sorted_pairs_v1(
    values: &[PairV1],
    min: usize,
    max: usize,
    label: &str,
) -> Result<(), PostureRecordValidationErrorV1> {
    if !(min..=max).contains(&values.len()) {
        return Err(invalid(format!("{label} has an invalid cardinality")));
    }
    for value in values {
        validate_pair_v1(value, label)?;
    }
    if values
        .windows(2)
        .any(|window| pair_order_v1(&window[0], &window[1]) != Ordering::Less)
    {
        return Err(invalid(format!(
            "{label} must be unique and strictly pair-sorted"
        )));
    }
    Ok(())
}

fn validate_sorted_refs_v1(
    values: &[String],
    min: usize,
    max: usize,
    label: &str,
) -> Result<(), PostureRecordValidationErrorV1> {
    if !(min..=max).contains(&values.len()) {
        return Err(invalid(format!("{label} has an invalid cardinality")));
    }
    for value in values {
        validate_safe_ref_v1(value, 512, label)?;
    }
    if values
        .windows(2)
        .any(|window| window[0].as_bytes().cmp(window[1].as_bytes()) != Ordering::Less)
    {
        return Err(invalid(format!(
            "{label} must be unique and strictly byte-sorted"
        )));
    }
    Ok(())
}

fn validate_exact_canonical_bytes_v1(
    bytes: &[u8],
    canonical: &CanonicalDocumentV1,
    label: &str,
) -> Result<(), PostureRecordValidationErrorV1> {
    validate_canonical_document_v1(canonical, label)?;
    if bytes.is_empty()
        || bytes.len() > MAX_POSTURE_CANONICAL_BYTES_V1 as usize
        || bytes.len() as u64 != canonical.byte_length
        || DefinitionFingerprint::from_bytes(bytes).to_string() != canonical.document_sha256
    {
        return Err(invalid(format!(
            "{label} bytes differ from their exact canonical binding"
        )));
    }
    Ok(())
}

fn validate_transition_id_v1(
    value: &str,
    prefix: &str,
    label: &str,
) -> Result<(), PostureRecordValidationErrorV1> {
    let Some(hex) = value.strip_prefix(prefix) else {
        return Err(invalid(format!("{label} prefix is invalid")));
    };
    if !is_lowercase_sha256_hex(hex) {
        return Err(invalid(format!(
            "{label} must end in 64 lowercase hex characters"
        )));
    }
    Ok(())
}

fn validate_transaction_id_v1(value: &str) -> Result<(), PostureRecordValidationErrorV1> {
    let Some(hex) = value.strip_prefix("posture-transaction_") else {
        return Err(invalid("posture transaction ID prefix is invalid"));
    };
    if hex.len() != 32 || !hex.bytes().all(is_lowercase_hex) {
        return Err(invalid(
            "posture transaction ID must carry exactly 128 random bits",
        ));
    }
    Ok(())
}

fn allocate_transaction_id_v1<F>(
    transaction_id_available: &mut F,
) -> Result<String, PostureRecordValidationErrorV1>
where
    F: FnMut(&str) -> bool,
{
    for _ in 0..64 {
        let mut random = [0_u8; 16];
        getrandom::fill(&mut random)
            .map_err(|_| invalid("posture transaction random allocation failed"))?;
        let suffix = random
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect::<String>();
        let transaction_id = format!("posture-transaction_{suffix}");
        if transaction_id_available(&transaction_id) {
            return Ok(transaction_id);
        }
    }
    Err(invalid(
        "posture transaction ID allocation exhausted collision retries",
    ))
}

pub(crate) fn validate_utc_second_v1(
    value: &str,
    label: &str,
) -> Result<(), PostureRecordValidationErrorV1> {
    let bytes = value.as_bytes();
    if bytes.len() != 20
        || !matches!(bytes[4], b'-')
        || !matches!(bytes[7], b'-')
        || !matches!(bytes[10], b'T')
        || !matches!(bytes[13], b':')
        || !matches!(bytes[16], b':')
        || !matches!(bytes[19], b'Z')
        || [0, 1, 2, 3, 5, 6, 8, 9, 11, 12, 14, 15, 17, 18]
            .into_iter()
            .any(|index| !bytes[index].is_ascii_digit())
    {
        return Err(invalid(format!("{label} is not canonical UTC-second text")));
    }
    let year = decimal(&bytes[0..4]);
    let month = decimal(&bytes[5..7]);
    let day = decimal(&bytes[8..10]);
    let hour = decimal(&bytes[11..13]);
    let minute = decimal(&bytes[14..16]);
    let second = decimal(&bytes[17..19]);
    let max_day = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) => 29,
        2 => 28,
        _ => 0,
    };
    if year == 0 || day == 0 || day > max_day || hour > 23 || minute > 59 || second > 59 {
        return Err(invalid(format!("{label} is not a valid UTC second")));
    }
    Ok(())
}

fn pair_order_v1(left: &PairV1, right: &PairV1) -> Ordering {
    left.reference
        .as_bytes()
        .cmp(right.reference.as_bytes())
        .then_with(|| {
            left.fingerprint
                .as_bytes()
                .cmp(right.fingerprint.as_bytes())
        })
}

fn decimal(bytes: &[u8]) -> u32 {
    bytes
        .iter()
        .fold(0, |value, byte| value * 10 + u32::from(byte - b'0'))
}

fn is_level(value: u8) -> bool {
    (1..=5).contains(&value)
}

fn is_fingerprint(value: &str) -> bool {
    value
        .strip_prefix("sha256:")
        .is_some_and(is_lowercase_sha256_hex)
}

fn is_lowercase_sha256_hex(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(is_lowercase_hex)
}

fn is_lowercase_hex(value: u8) -> bool {
    value.is_ascii_digit() || matches!(value, b'a'..=b'f')
}

fn invalid(detail: impl Into<String>) -> PostureRecordValidationErrorV1 {
    PostureRecordValidationErrorV1::new(detail)
}
