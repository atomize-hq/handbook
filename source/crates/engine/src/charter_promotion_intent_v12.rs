use crate::charter_lifecycle_validation::{
    definition_bindings, LifecycleActiveObservationV10, LifecycleDefinitionBindingV10,
};
use crate::{parse_schema_json, DefinitionFingerprint};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::fmt;

pub(crate) const MAX_PROMOTION_INTENT_BYTES_V12: usize = 262_144;

const COVERAGE_ORDER: [&str; 16] = [
    "project_shape.definition",
    "delivery.constraints",
    "delivery.default_implications",
    "operational_reality.production_state",
    "risk.domains",
    "engineering_posture.baseline",
    "policy.authority_and_revision",
    "governance.decision_authority",
    "governance.required_approvals",
    "governance.exception_policy",
    "engineering_posture.dimensions",
    "engineering_posture.red_lines",
    "governance.review_triggers",
    "governance.reassessment_triggers",
    "debt.register",
    "decisions.records",
];

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PromotionIntentTargetV12 {
    pub(crate) target_instance_id: String,
    pub(crate) canonical_artifact_ref: String,
    pub(crate) basis_artifact_fingerprint: Option<String>,
    pub(crate) observed_current_artifact_fingerprint: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PromotionCandidateLineageV12 {
    pub(crate) candidate_ref: String,
    pub(crate) candidate_fingerprint: String,
    pub(crate) candidate_subject_fingerprint: String,
    pub(crate) intake_record_ref: String,
    pub(crate) intake_record_fingerprint: String,
    pub(crate) normalized_content_ref: String,
    pub(crate) normalized_content_fingerprint: String,
    pub(crate) validation_result_ref: String,
    pub(crate) validation_result_fingerprint: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PromotionSelectedContractV12 {
    pub(crate) profile_ref: String,
    pub(crate) resolved_profile_fingerprint: String,
    pub(crate) resolved_definitions: Vec<LifecycleDefinitionBindingV10>,
    pub(crate) lifecycle_policy_ref: String,
    pub(crate) lifecycle_policy_fingerprint: String,
    pub(crate) prior_lifecycle_head_ref: Option<String>,
    pub(crate) prior_lifecycle_head_fingerprint: Option<String>,
    pub(crate) prior_lifecycle_state: String,
    pub(crate) prior_lifecycle_state_fingerprint: Option<String>,
    pub(crate) active_observations: Vec<LifecycleActiveObservationV10>,
    pub(crate) reopened_coverage_ids: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PromotionApprovalBindingV12 {
    pub(crate) approval_ref: String,
    pub(crate) approval_fingerprint: String,
    pub(crate) approval_class: String,
    pub(crate) authority_ref: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PromotionHumanAuthorityV12 {
    pub(crate) approval_bindings: Vec<PromotionApprovalBindingV12>,
    pub(crate) approver_registry_state_ref: String,
    pub(crate) approver_registry_state_fingerprint: String,
    pub(crate) registry_head_transition_ref: String,
    pub(crate) registry_head_transition_fingerprint: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PromotionOutputRecordV12 {
    pub(crate) record_ref: String,
    pub(crate) record_fingerprint: String,
    pub(crate) document_sha256: String,
    pub(crate) byte_length: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PromotionOutputsV12 {
    pub(crate) new_canonical_fingerprint: String,
    pub(crate) new_canonical_document_sha256: String,
    pub(crate) new_canonical_byte_length: u64,
    pub(crate) promotion_record: PromotionOutputRecordV12,
    pub(crate) lifecycle_transition: PromotionOutputRecordV12,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PromotionRecoveryV12 {
    pub(crate) old_canonical_status: String,
    pub(crate) old_canonical_fingerprint: Option<String>,
    pub(crate) old_canonical_document_sha256: Option<String>,
    pub(crate) old_canonical_byte_length: Option<u64>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PromotionTransactionIntentV12 {
    pub(crate) schema_id: String,
    pub(crate) schema_version: String,
    pub(crate) transaction_id: String,
    pub(crate) promotion_id: String,
    pub(crate) mutation_mode: String,
    pub(crate) target: PromotionIntentTargetV12,
    pub(crate) candidate_lineage: PromotionCandidateLineageV12,
    pub(crate) selected_contract: PromotionSelectedContractV12,
    pub(crate) human_authority: PromotionHumanAuthorityV12,
    pub(crate) outputs: PromotionOutputsV12,
    pub(crate) recovery: PromotionRecoveryV12,
    pub(crate) intent_fingerprint: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ValidatedPromotionIntentV12 {
    pub(crate) record: PromotionTransactionIntentV12,
    pub(crate) raw_bytes: Vec<u8>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PromotionIntentValidationErrorV12 {
    detail: String,
}

impl PromotionIntentValidationErrorV12 {
    fn new(detail: impl Into<String>) -> Self {
        Self {
            detail: detail.into(),
        }
    }

    pub(crate) fn detail(&self) -> &str {
        &self.detail
    }
}

impl fmt::Display for PromotionIntentValidationErrorV12 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.detail)
    }
}

impl std::error::Error for PromotionIntentValidationErrorV12 {}

pub(crate) fn parse_promotion_intent_v12(
    bytes: &[u8],
) -> Result<ValidatedPromotionIntentV12, PromotionIntentValidationErrorV12> {
    if bytes.is_empty()
        || bytes.len() > MAX_PROMOTION_INTENT_BYTES_V12
        || !bytes.ends_with(b"\n")
        || bytes[..bytes.len() - 1].ends_with(b"\n")
    {
        return Err(invalid(
            "promotion intent 1.2 must be bounded exact JCS plus one LF",
        ));
    }
    let jcs = &bytes[..bytes.len() - 1];
    let value = parse_schema_json(jcs)
        .map_err(|_| invalid("promotion intent 1.2 is not duplicate-free JSON"))?;
    let canonical = serde_json_canonicalizer::to_vec(&value)
        .map_err(|_| invalid("promotion intent 1.2 cannot be canonicalized"))?;
    if canonical != jcs {
        return Err(invalid(
            "promotion intent 1.2 bytes are not exact RFC 8785 JCS",
        ));
    }
    let record: PromotionTransactionIntentV12 = serde_json::from_value(value)
        .map_err(|_| invalid("promotion intent 1.2 violates its closed shape"))?;
    validate_promotion_intent_v12(&record)?;
    Ok(ValidatedPromotionIntentV12 {
        record,
        raw_bytes: bytes.to_vec(),
    })
}

pub(crate) fn promotion_intent_marker_v12(intent_bytes: &[u8]) -> Vec<u8> {
    format!("sha256:{:x}\n", Sha256::digest(intent_bytes)).into_bytes()
}

fn validate_promotion_intent_v12(
    record: &PromotionTransactionIntentV12,
) -> Result<(), PromotionIntentValidationErrorV12> {
    if record.schema_id != "handbook.charter-promotion-transaction-intent"
        || record.schema_version != "1.2"
        || record.target.target_instance_id != "project_authority"
        || record.target.canonical_artifact_ref != ".handbook/project/charter.yaml"
        || record.selected_contract.profile_ref != "handbook.profile.shipped-root@1.1.0"
        || record.selected_contract.resolved_profile_fingerprint
            != "sha256:6a7b41befa77b999b9ee20f513636051726a8401a81bf2f369501e8f3dd4fa74"
        || record.selected_contract.lifecycle_policy_ref
            != "handbook.lifecycle.constitutional-review-lock@1.0.0"
        || record.selected_contract.lifecycle_policy_fingerprint
            != "sha256:88caafb9caaf137647c42a91cd2762ac0871e0a20e2a1844c2c0076d5fb43cc3"
        || record.selected_contract.resolved_definitions != definition_bindings()
    {
        return Err(invalid(
            "promotion intent 1.2 schema or selected-contract constants are invalid",
        ));
    }
    validate_transaction_id(&record.transaction_id)?;
    validate_id(&record.promotion_id, "promotion_", "promotion ID")?;

    for fingerprint in promotion_intent_fingerprints(record) {
        DefinitionFingerprint::parse(fingerprint).map_err(|_| {
            invalid("promotion intent 1.2 contains a non-lowercase SHA-256 fingerprint")
        })?;
    }
    validate_ref(
        &record.candidate_lineage.candidate_ref,
        "candidates/candidate_",
        ".json",
        &record.candidate_lineage.candidate_fingerprint,
        "candidate ref",
    )?;
    validate_ref(
        &record.candidate_lineage.intake_record_ref,
        "intake-records/intake_",
        ".json",
        &record.candidate_lineage.intake_record_fingerprint,
        "intake ref",
    )?;
    validate_ref(
        &record.candidate_lineage.normalized_content_ref,
        "candidate-content/charter_",
        ".yaml",
        &record.candidate_lineage.normalized_content_fingerprint,
        "normalized-content ref",
    )?;
    validate_ref(
        &record.candidate_lineage.validation_result_ref,
        "lifecycle-validation-results/lifecycle-validation-result_",
        ".json",
        &record.candidate_lineage.validation_result_fingerprint,
        "validation-result ref",
    )?;
    validate_ref(
        &record.outputs.promotion_record.record_ref,
        "promotions/promotion_",
        ".json",
        &record.outputs.promotion_record.record_fingerprint,
        "promotion output ref",
    )?;
    validate_ref(
        &record.outputs.lifecycle_transition.record_ref,
        "lifecycle-transitions/lifecycle-transition_",
        ".json",
        &record.outputs.lifecycle_transition.record_fingerprint,
        "lifecycle output ref",
    )?;
    if record.promotion_id
        != record.outputs.promotion_record.record_ref
            ["promotions/".len()..record.outputs.promotion_record.record_ref.len() - ".json".len()]
    {
        return Err(invalid(
            "promotion ID does not equal the nested promotion output record ID",
        ));
    }
    if record.outputs.new_canonical_fingerprint != record.outputs.new_canonical_document_sha256
        || record.outputs.new_canonical_byte_length == 0
        || record.outputs.new_canonical_byte_length > 1_048_576
    {
        return Err(invalid(
            "new canonical output fingerprint/hash/length binding is invalid",
        ));
    }
    for output in [
        &record.outputs.promotion_record,
        &record.outputs.lifecycle_transition,
    ] {
        if output.byte_length == 0 || output.byte_length > 262_144 {
            return Err(invalid("promotion output record byte length is invalid"));
        }
    }
    validate_ref(
        &record.human_authority.approver_registry_state_ref,
        "registry-states/registry-state_",
        ".json",
        &record.human_authority.approver_registry_state_fingerprint,
        "approver registry state ref",
    )?;
    validate_ref(
        &record.human_authority.registry_head_transition_ref,
        "registry-transitions/registry-transition_",
        ".json",
        &record.human_authority.registry_head_transition_fingerprint,
        "registry head ref",
    )?;
    if record.human_authority.approval_bindings.is_empty()
        || record.human_authority.approval_bindings.len() > 16
        || record
            .human_authority
            .approval_bindings
            .iter()
            .collect::<BTreeSet<_>>()
            .len()
            != record.human_authority.approval_bindings.len()
    {
        return Err(invalid(
            "promotion intent approval bindings are empty, duplicate, or unbounded",
        ));
    }
    for approval in &record.human_authority.approval_bindings {
        if approval.approval_class.is_empty()
            || approval.approval_class.len() > 256
            || approval.authority_ref.is_empty()
            || approval.authority_ref.len() > 256
        {
            return Err(invalid("promotion approval authority strings are invalid"));
        }
        validate_ref(
            &approval.approval_ref,
            "approvals/approval_",
            ".json",
            &approval.approval_fingerprint,
            "approval ref",
        )?;
    }
    if record
        .human_authority
        .approval_bindings
        .windows(2)
        .any(|window| {
            (&window[0].approval_class, &window[0].authority_ref)
                >= (&window[1].approval_class, &window[1].authority_ref)
        })
    {
        return Err(invalid(
            "promotion approval authority pairs must be unique and in lexical pair order",
        ));
    }
    if record.selected_contract.active_observations.len() > 256
        || record
            .selected_contract
            .active_observations
            .iter()
            .map(|entry| (&entry.observation_ref, &entry.observation_fingerprint))
            .collect::<BTreeSet<_>>()
            .len()
            != record.selected_contract.active_observations.len()
    {
        return Err(invalid(
            "promotion intent active observations are duplicate or unbounded",
        ));
    }
    for observation in &record.selected_contract.active_observations {
        validate_ref(
            &observation.observation_ref,
            "lifecycle-observations/lifecycle-observation_",
            ".json",
            &observation.observation_fingerprint,
            "active observation ref",
        )?;
    }
    let reopened = record
        .selected_contract
        .reopened_coverage_ids
        .iter()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    if reopened.len() != record.selected_contract.reopened_coverage_ids.len()
        || reopened.len() > COVERAGE_ORDER.len()
        || record.selected_contract.reopened_coverage_ids
            != COVERAGE_ORDER
                .iter()
                .filter(|coverage_id| reopened.contains(**coverage_id))
                .map(|coverage_id| (*coverage_id).to_owned())
                .collect::<Vec<_>>()
    {
        return Err(invalid(
            "promotion intent reopened coverage is unknown, duplicate, or reordered",
        ));
    }

    match record.mutation_mode.as_str() {
        "create" => validate_create_nullability(record)?,
        "amend" => validate_amend_nullability(record)?,
        _ => return Err(invalid("promotion intent mutation mode is unsupported")),
    }
    let mut preimage = serde_json::to_value(record)
        .map_err(|_| invalid("promotion intent fingerprint preimage serialization failed"))?;
    preimage
        .as_object_mut()
        .expect("typed intent is an object")
        .remove("intent_fingerprint");
    let computed = DefinitionFingerprint::from_json_value(&preimage)
        .map_err(|_| invalid("promotion intent fingerprint preimage is not canonical"))?
        .to_string();
    if computed != record.intent_fingerprint {
        return Err(invalid(
            "promotion intent fingerprint does not match its exact exclusion preimage",
        ));
    }
    Ok(())
}

fn validate_create_nullability(
    record: &PromotionTransactionIntentV12,
) -> Result<(), PromotionIntentValidationErrorV12> {
    if record.target.basis_artifact_fingerprint.is_some()
        || record
            .target
            .observed_current_artifact_fingerprint
            .is_some()
        || record.selected_contract.prior_lifecycle_head_ref.is_some()
        || record
            .selected_contract
            .prior_lifecycle_head_fingerprint
            .is_some()
        || record.selected_contract.prior_lifecycle_state != "absent"
        || record
            .selected_contract
            .prior_lifecycle_state_fingerprint
            .is_some()
        || !record.selected_contract.active_observations.is_empty()
        || !record.selected_contract.reopened_coverage_ids.is_empty()
        || record.recovery.old_canonical_status != "absent"
        || record.recovery.old_canonical_fingerprint.is_some()
        || record.recovery.old_canonical_document_sha256.is_some()
        || record.recovery.old_canonical_byte_length.is_some()
    {
        return Err(invalid(
            "create promotion intent does not have its exact null/empty prior-authority closure",
        ));
    }
    Ok(())
}

fn validate_amend_nullability(
    record: &PromotionTransactionIntentV12,
) -> Result<(), PromotionIntentValidationErrorV12> {
    let basis = record
        .target
        .basis_artifact_fingerprint
        .as_deref()
        .ok_or_else(|| invalid("amend promotion intent basis is absent"))?;
    if record
        .target
        .observed_current_artifact_fingerprint
        .as_deref()
        != Some(basis)
        || record.selected_contract.prior_lifecycle_head_ref.is_none()
        || record
            .selected_contract
            .prior_lifecycle_head_fingerprint
            .is_none()
        || !matches!(
            record.selected_contract.prior_lifecycle_state.as_str(),
            "current" | "review_required" | "reassessment_required"
        )
        || record
            .selected_contract
            .prior_lifecycle_state_fingerprint
            .is_none()
        || record.recovery.old_canonical_status != "present"
        || record.recovery.old_canonical_fingerprint.as_deref() != Some(basis)
        || record.recovery.old_canonical_document_sha256.as_deref() != Some(basis)
        || !record
            .recovery
            .old_canonical_byte_length
            .is_some_and(|length| (1..=1_048_576).contains(&length))
    {
        return Err(invalid(
            "amend promotion intent prior-authority/recovery closure is incomplete or unequal",
        ));
    }
    validate_optional_ref(
        record.selected_contract.prior_lifecycle_head_ref.as_deref(),
        "lifecycle-transitions/lifecycle-transition_",
        ".json",
        record
            .selected_contract
            .prior_lifecycle_head_fingerprint
            .as_deref(),
        "prior lifecycle head ref",
    )?;
    Ok(())
}

fn promotion_intent_fingerprints(record: &PromotionTransactionIntentV12) -> Vec<&str> {
    let mut values = vec![
        record.candidate_lineage.candidate_fingerprint.as_str(),
        record
            .candidate_lineage
            .candidate_subject_fingerprint
            .as_str(),
        record.candidate_lineage.intake_record_fingerprint.as_str(),
        record
            .candidate_lineage
            .normalized_content_fingerprint
            .as_str(),
        record
            .candidate_lineage
            .validation_result_fingerprint
            .as_str(),
        record
            .selected_contract
            .resolved_profile_fingerprint
            .as_str(),
        record
            .selected_contract
            .lifecycle_policy_fingerprint
            .as_str(),
        record
            .human_authority
            .approver_registry_state_fingerprint
            .as_str(),
        record
            .human_authority
            .registry_head_transition_fingerprint
            .as_str(),
        record.outputs.new_canonical_fingerprint.as_str(),
        record.outputs.new_canonical_document_sha256.as_str(),
        record.outputs.promotion_record.record_fingerprint.as_str(),
        record.outputs.promotion_record.document_sha256.as_str(),
        record
            .outputs
            .lifecycle_transition
            .record_fingerprint
            .as_str(),
        record.outputs.lifecycle_transition.document_sha256.as_str(),
        record.intent_fingerprint.as_str(),
    ];
    for definition in &record.selected_contract.resolved_definitions {
        values.push(definition.definition_fingerprint.as_str());
    }
    for observation in &record.selected_contract.active_observations {
        values.push(observation.observation_fingerprint.as_str());
    }
    for approval in &record.human_authority.approval_bindings {
        values.push(approval.approval_fingerprint.as_str());
    }
    for optional in [
        record.target.basis_artifact_fingerprint.as_deref(),
        record
            .target
            .observed_current_artifact_fingerprint
            .as_deref(),
        record
            .selected_contract
            .prior_lifecycle_head_fingerprint
            .as_deref(),
        record
            .selected_contract
            .prior_lifecycle_state_fingerprint
            .as_deref(),
        record.recovery.old_canonical_fingerprint.as_deref(),
        record.recovery.old_canonical_document_sha256.as_deref(),
    ]
    .into_iter()
    .flatten()
    {
        values.push(optional);
    }
    values
}

fn validate_transaction_id(value: &str) -> Result<(), PromotionIntentValidationErrorV12> {
    let Some(suffix) = value.strip_prefix("promotion-transaction_") else {
        return Err(invalid("promotion transaction ID prefix is invalid"));
    };
    if suffix.is_empty()
        || suffix.len() > 128
        || !suffix.as_bytes()[0].is_ascii_alphanumeric()
        || !suffix
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-'))
    {
        return Err(invalid("promotion transaction ID grammar is invalid"));
    }
    Ok(())
}

fn validate_id(
    value: &str,
    prefix: &str,
    label: &str,
) -> Result<(), PromotionIntentValidationErrorV12> {
    let Some(hex) = value.strip_prefix(prefix) else {
        return Err(invalid(format!("{label} prefix is invalid")));
    };
    validate_hex(hex, label)
}

fn validate_ref(
    reference: &str,
    prefix: &str,
    suffix: &str,
    fingerprint: &str,
    label: &str,
) -> Result<(), PromotionIntentValidationErrorV12> {
    let Some(hex) = reference
        .strip_prefix(prefix)
        .and_then(|rest| rest.strip_suffix(suffix))
    else {
        return Err(invalid(format!("{label} grammar is invalid")));
    };
    validate_hex(hex, label)?;
    if fingerprint != format!("sha256:{hex}") {
        return Err(invalid(format!(
            "{label} basename and declared fingerprint disagree"
        )));
    }
    Ok(())
}

fn validate_optional_ref(
    reference: Option<&str>,
    prefix: &str,
    suffix: &str,
    fingerprint: Option<&str>,
    label: &str,
) -> Result<(), PromotionIntentValidationErrorV12> {
    match (reference, fingerprint) {
        (Some(reference), Some(fingerprint)) => {
            validate_ref(reference, prefix, suffix, fingerprint, label)
        }
        (None, None) => Ok(()),
        _ => Err(invalid(format!(
            "{label} ref/fingerprint nullability differs"
        ))),
    }
}

fn validate_hex(hex: &str, label: &str) -> Result<(), PromotionIntentValidationErrorV12> {
    if hex.len() != 64
        || !hex
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(invalid(format!("{label} is not lowercase SHA-256")));
    }
    Ok(())
}

fn invalid(detail: impl Into<String>) -> PromotionIntentValidationErrorV12 {
    PromotionIntentValidationErrorV12::new(detail)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    const VECTORS: &[u8] = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../docs/specs/handbook-contract-membrane/slices/HCM-2.2/contracts/promotion-transaction-intent-vectors-v1.0.json"
    ));

    fn amendment_record() -> Value {
        let vectors: Value = serde_json::from_slice(VECTORS).unwrap();
        let mut record = vectors["amendment_positive"]["intent_fingerprint_preimage"].clone();
        record.as_object_mut().unwrap().extend(
            vectors["amendment_positive"]["intent_record_additions"]
                .as_object()
                .unwrap()
                .clone(),
        );
        record
    }

    fn encode_value(value: &Value) -> Vec<u8> {
        let mut bytes = serde_json_canonicalizer::to_vec(value).unwrap();
        bytes.push(b'\n');
        bytes
    }

    fn resign_intent(value: &mut Value) {
        let mut preimage = value.clone();
        preimage
            .as_object_mut()
            .unwrap()
            .remove("intent_fingerprint");
        value["intent_fingerprint"] = Value::String(
            DefinitionFingerprint::from_json_value(&preimage)
                .unwrap()
                .to_string(),
        );
    }

    #[test]
    fn reviewed_amendment_vector_parses_with_exact_fingerprint_document_and_marker() {
        let vectors: Value = serde_json::from_slice(VECTORS).unwrap();
        let bytes = encode_value(&amendment_record());
        let parsed = parse_promotion_intent_v12(&bytes).unwrap();
        assert_eq!(
            bytes.len(),
            vectors["amendment_positive"]["expected_intent_document_byte_length"]
                .as_u64()
                .unwrap() as usize
        );
        assert_eq!(
            DefinitionFingerprint::from_bytes(&bytes).to_string(),
            vectors["amendment_positive"]["expected_intent_document_sha256"]
                .as_str()
                .unwrap()
        );
        assert_eq!(
            promotion_intent_marker_v12(&parsed.raw_bytes),
            vectors["amendment_positive"]["expected_marker_payload"]
                .as_str()
                .unwrap()
                .as_bytes()
        );
    }

    #[test]
    fn legacy_top_level_wrong_schema_and_fingerprint_mismatch_refuse() {
        let mut legacy = amendment_record();
        legacy["canonical_fingerprint"] = legacy["outputs"]["new_canonical_fingerprint"].clone();
        assert!(parse_promotion_intent_v12(&encode_value(&legacy)).is_err());

        let mut wrong_schema = amendment_record();
        wrong_schema["schema_version"] = Value::String("1.0".to_owned());
        assert!(parse_promotion_intent_v12(&encode_value(&wrong_schema)).is_err());

        let mut wrong_fingerprint = amendment_record();
        wrong_fingerprint["intent_fingerprint"] =
            Value::String(format!("sha256:{}", "0".repeat(64)));
        assert!(parse_promotion_intent_v12(&encode_value(&wrong_fingerprint)).is_err());
    }

    #[test]
    fn reordered_or_duplicate_approval_pairs_refuse_even_when_intent_is_resigned() {
        let second = serde_json::json!({
            "approval_ref": format!("approvals/approval_{}.json", "a".repeat(64)),
            "approval_fingerprint": format!("sha256:{}", "a".repeat(64)),
            "approval_class": "Security approval",
            "authority_ref": "Security lead",
        });
        let mut ordered = amendment_record();
        ordered["human_authority"]["approval_bindings"]
            .as_array_mut()
            .unwrap()
            .push(second.clone());
        resign_intent(&mut ordered);
        parse_promotion_intent_v12(&encode_value(&ordered))
            .expect("strictly increasing approval pairs are structurally valid");

        let mut reordered = ordered.clone();
        reordered["human_authority"]["approval_bindings"]
            .as_array_mut()
            .unwrap()
            .reverse();
        resign_intent(&mut reordered);
        assert!(parse_promotion_intent_v12(&encode_value(&reordered)).is_err());

        let mut duplicate_pair = amendment_record();
        let original = duplicate_pair["human_authority"]["approval_bindings"][0].clone();
        let mut duplicate = original;
        duplicate["approval_ref"] =
            Value::String(format!("approvals/approval_{}.json", "b".repeat(64)));
        duplicate["approval_fingerprint"] = Value::String(format!("sha256:{}", "b".repeat(64)));
        duplicate_pair["human_authority"]["approval_bindings"]
            .as_array_mut()
            .unwrap()
            .push(duplicate);
        resign_intent(&mut duplicate_pair);
        assert!(parse_promotion_intent_v12(&encode_value(&duplicate_pair)).is_err());
    }
}
