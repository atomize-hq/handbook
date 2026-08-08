use crate::charter_lifecycle::CharterLifecycleState;
use crate::charter_lifecycle_store::{
    charter_lifecycle_state_fingerprint, CHARTER_LIFECYCLE_POLICY_REF,
};
use crate::charter_posture_transaction_intent_v1::{
    encode_jcs_lf_v1, fingerprint_excluding_v1, parse_jcs_lf_v1, posture_transition_pair_v1,
    validate_content_ref_v1, validate_fingerprint_v1, validate_pair_v1, validate_reassessment_v1,
    validate_utc_second_v1, EmptyExtensionsV1, OutputRecordV1, PairV1, PostureReassessmentV1,
    PostureRecordValidationErrorV1, ValidatedPostureTransitionV1, MAX_POSTURE_RECORD_BYTES_V1,
};
use crate::DefinitionFingerprint;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

const LIFECYCLE_POLICY_REF: &str = "handbook.lifecycle.constitutional-review-lock@1.0.0";
const LIFECYCLE_POLICY_FINGERPRINT: &str =
    "sha256:88caafb9caaf137647c42a91cd2762ac0871e0a20e2a1844c2c0076d5fb43cc3";

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LifecycleTransitionRecordV11 {
    pub(crate) schema_id: String,
    pub(crate) schema_version: String,
    pub(crate) transition_id: String,
    pub(crate) transition_kind: String,
    pub(crate) lifecycle_policy: PairV1,
    pub(crate) target_instance_id: String,
    pub(crate) prior_transition: PairV1,
    pub(crate) prior_state: String,
    pub(crate) prior_state_fingerprint: String,
    pub(crate) new_observation_refs: Vec<String>,
    pub(crate) active_observation_refs: Vec<String>,
    pub(crate) result_state: String,
    pub(crate) result_state_fingerprint: String,
    pub(crate) prior_canonical_fingerprint: String,
    pub(crate) resulting_canonical_fingerprint: String,
    pub(crate) authority_transition: PairV1,
    pub(crate) reassessment: PostureReassessmentV1,
    pub(crate) transitioned_at_utc: String,
    pub(crate) extensions: EmptyExtensionsV1,
    pub(crate) transition_fingerprint: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct LifecycleTransitionDraftV11 {
    pub(crate) prior_transition: PairV1,
    pub(crate) prior_state_fingerprint: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ValidatedLifecycleTransitionV11 {
    pub(crate) record: LifecycleTransitionRecordV11,
    pub(crate) raw_bytes: Vec<u8>,
}

pub(crate) fn parse_lifecycle_transition_v11(
    bytes: &[u8],
) -> Result<ValidatedLifecycleTransitionV11, PostureRecordValidationErrorV1> {
    let record = parse_jcs_lf_v1(
        bytes,
        MAX_POSTURE_RECORD_BYTES_V1,
        "lifecycle transition 1.1",
    )?;
    validate_lifecycle_transition_record_v11(&record)?;
    Ok(ValidatedLifecycleTransitionV11 {
        record,
        raw_bytes: bytes.to_vec(),
    })
}

pub(crate) fn construct_lifecycle_transition_v11(
    posture: &ValidatedPostureTransitionV1,
    draft: LifecycleTransitionDraftV11,
) -> Result<ValidatedLifecycleTransitionV11, PostureRecordValidationErrorV1> {
    let result_state_fingerprint =
        lifecycle_current_state_fingerprint(&posture.record.resulting_canonical.fingerprint)?;
    let mut record = LifecycleTransitionRecordV11 {
        schema_id: "handbook.lifecycle-transition".to_owned(),
        schema_version: "1.1".to_owned(),
        transition_id: String::new(),
        transition_kind: "posture_transition".to_owned(),
        lifecycle_policy: PairV1 {
            reference: LIFECYCLE_POLICY_REF.to_owned(),
            fingerprint: LIFECYCLE_POLICY_FINGERPRINT.to_owned(),
        },
        target_instance_id: "project_authority".to_owned(),
        prior_transition: draft.prior_transition,
        prior_state: "current".to_owned(),
        prior_state_fingerprint: draft.prior_state_fingerprint,
        new_observation_refs: Vec::new(),
        active_observation_refs: Vec::new(),
        result_state: "current".to_owned(),
        result_state_fingerprint,
        prior_canonical_fingerprint: posture.record.expected_canonical.fingerprint.clone(),
        resulting_canonical_fingerprint: posture.record.resulting_canonical.fingerprint.clone(),
        authority_transition: posture_transition_pair_v1(posture),
        reassessment: posture.record.reassessment.clone(),
        transitioned_at_utc: posture.record.effective_at_utc.clone(),
        extensions: EmptyExtensionsV1 {},
        transition_fingerprint: String::new(),
    };
    let fingerprint = fingerprint_excluding_v1(
        &record,
        &[
            "transition_id",
            "transition_fingerprint",
            "transitioned_at_utc",
        ],
        "lifecycle transition 1.1",
    )?;
    record.transition_id = format!(
        "lifecycle-transition_{}",
        fingerprint
            .strip_prefix("sha256:")
            .expect("validated SHA-256 prefix")
    );
    record.transition_fingerprint = fingerprint;
    validate_lifecycle_transition_record_v11(&record)?;
    let raw_bytes = encode_jcs_lf_v1(
        &record,
        MAX_POSTURE_RECORD_BYTES_V1,
        "lifecycle transition 1.1",
    )?;
    Ok(ValidatedLifecycleTransitionV11 { record, raw_bytes })
}

pub(crate) fn validate_lifecycle_transition_bindings_v11(
    lifecycle: &ValidatedLifecycleTransitionV11,
    posture: &ValidatedPostureTransitionV1,
) -> Result<(), PostureRecordValidationErrorV1> {
    validate_lifecycle_transition_record_v11(&lifecycle.record)?;
    if lifecycle.record.prior_canonical_fingerprint != posture.record.expected_canonical.fingerprint
        || lifecycle.record.resulting_canonical_fingerprint
            != posture.record.resulting_canonical.fingerprint
        || lifecycle.record.authority_transition != posture_transition_pair_v1(posture)
        || lifecycle.record.reassessment != posture.record.reassessment
        || lifecycle.record.transitioned_at_utc != posture.record.effective_at_utc
    {
        return Err(invalid(
            "lifecycle transition posture-rebase bindings are substituted or incomplete",
        ));
    }
    let expected_prior =
        lifecycle_current_state_fingerprint(&posture.record.expected_canonical.fingerprint)?;
    let expected_result =
        lifecycle_current_state_fingerprint(&posture.record.resulting_canonical.fingerprint)?;
    if lifecycle.record.prior_state_fingerprint != expected_prior
        || lifecycle.record.result_state_fingerprint != expected_result
    {
        return Err(invalid(
            "lifecycle transition state fingerprints do not bind the exact prior and resulting Charters",
        ));
    }
    Ok(())
}

pub(crate) fn lifecycle_transition_output_record_v11(
    transition: &ValidatedLifecycleTransitionV11,
) -> OutputRecordV1 {
    OutputRecordV1 {
        reference: format!(
            "lifecycle-transitions/{}.json",
            transition.record.transition_id
        ),
        fingerprint: transition.record.transition_fingerprint.clone(),
        document_sha256: DefinitionFingerprint::from_bytes(&transition.raw_bytes).to_string(),
        byte_length: transition.raw_bytes.len() as u64,
    }
}

pub(crate) fn lifecycle_transition_marker_v11(record_bytes: &[u8]) -> Vec<u8> {
    format!("sha256:{:x}\n", Sha256::digest(record_bytes)).into_bytes()
}

fn validate_lifecycle_transition_record_v11(
    record: &LifecycleTransitionRecordV11,
) -> Result<(), PostureRecordValidationErrorV1> {
    if record.schema_id != "handbook.lifecycle-transition"
        || record.schema_version != "1.1"
        || record.transition_kind != "posture_transition"
        || record.target_instance_id != "project_authority"
        || record.lifecycle_policy.reference != LIFECYCLE_POLICY_REF
        || record.lifecycle_policy.fingerprint != LIFECYCLE_POLICY_FINGERPRINT
        || record.prior_state != "current"
        || record.result_state != "current"
        || !record.new_observation_refs.is_empty()
        || !record.active_observation_refs.is_empty()
    {
        return Err(invalid("lifecycle transition 1.1 constants are invalid"));
    }
    validate_lifecycle_id(&record.transition_id)?;
    validate_pair_v1(&record.lifecycle_policy, "lifecycle policy")?;
    validate_content_ref_v1(
        &record.prior_transition,
        "lifecycle-transitions/lifecycle-transition_",
        ".json",
        "lifecycle prior transition",
    )?;
    for (fingerprint, label) in [
        (
            &record.prior_state_fingerprint,
            "lifecycle prior state fingerprint",
        ),
        (
            &record.result_state_fingerprint,
            "lifecycle result state fingerprint",
        ),
        (
            &record.prior_canonical_fingerprint,
            "lifecycle prior canonical fingerprint",
        ),
        (
            &record.resulting_canonical_fingerprint,
            "lifecycle resulting canonical fingerprint",
        ),
        (
            &record.transition_fingerprint,
            "lifecycle transition fingerprint",
        ),
    ] {
        validate_fingerprint_v1(fingerprint, label)?;
    }
    validate_content_ref_v1(
        &record.authority_transition,
        "posture-transitions/posture-transition_",
        ".json",
        "lifecycle authority transition",
    )?;
    let expected_prior = lifecycle_current_state_fingerprint(&record.prior_canonical_fingerprint)?;
    let expected_result =
        lifecycle_current_state_fingerprint(&record.resulting_canonical_fingerprint)?;
    if record.prior_state_fingerprint != expected_prior
        || record.result_state_fingerprint != expected_result
    {
        return Err(invalid(
            "lifecycle transition state fingerprints do not bind the exact prior and resulting Charters",
        ));
    }
    validate_reassessment_v1(&record.reassessment)?;
    validate_utc_second_v1(&record.transitioned_at_utc, "lifecycle transition time")?;
    let computed = fingerprint_excluding_v1(
        record,
        &[
            "transition_id",
            "transition_fingerprint",
            "transitioned_at_utc",
        ],
        "lifecycle transition 1.1",
    )?;
    if record.transition_fingerprint != computed {
        return Err(invalid(
            "lifecycle transition fingerprint has an invalid semantic preimage",
        ));
    }
    let expected_id = format!(
        "lifecycle-transition_{}",
        record
            .transition_fingerprint
            .strip_prefix("sha256:")
            .expect("validated SHA-256 prefix")
    );
    if record.transition_id != expected_id {
        return Err(invalid(
            "lifecycle transition ID does not derive from its fingerprint",
        ));
    }
    Ok(())
}

fn lifecycle_current_state_fingerprint(
    canonical_fingerprint: &str,
) -> Result<String, PostureRecordValidationErrorV1> {
    charter_lifecycle_state_fingerprint(
        CHARTER_LIFECYCLE_POLICY_REF,
        LIFECYCLE_POLICY_FINGERPRINT,
        "project_authority",
        canonical_fingerprint,
        CharterLifecycleState::Current,
        &[],
    )
    .map_err(|failure| {
        invalid(format!(
            "lifecycle state fingerprint derivation refused: {}",
            failure.detail()
        ))
    })
}

fn validate_lifecycle_id(value: &str) -> Result<(), PostureRecordValidationErrorV1> {
    let Some(hex) = value.strip_prefix("lifecycle-transition_") else {
        return Err(invalid("lifecycle transition ID prefix is invalid"));
    };
    if hex.len() != 64
        || !hex
            .bytes()
            .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
    {
        return Err(invalid(
            "lifecycle transition ID must end in lowercase SHA-256 hex",
        ));
    }
    Ok(())
}

fn invalid(detail: impl Into<String>) -> PostureRecordValidationErrorV1 {
    PostureRecordValidationErrorV1::new(detail)
}
