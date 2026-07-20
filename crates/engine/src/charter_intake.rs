use crate::charter_artifact::{
    serialize_canonical_charter, CanonicalCharter, CharterArtifactError,
};
use crate::definition_identity::canonical_json_bytes;
use crate::{DefinitionFingerprint, ResolvedProfileDecisions};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

const CHARTER_KIND_REF: &str = "handbook.artifact-kind.project-authority@1.1.0";
const CHARTER_INSTANCE_ID: &str = "project_authority";
const CHARTER_SCHEMA_REF: &str = "handbook.schemas.artifacts.project-authority@1.1.0";
const CHARTER_INTAKE_REF: &str = "handbook.intake.charter@1.0.0";
const CHARTER_APPROVAL_POLICY_REF: &str = "handbook.approval.constitutional-candidate@1.0.0";

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

const NORMATIVE_COVERAGE: [&str; 11] = [
    "delivery.default_implications",
    "engineering_posture.baseline",
    "policy.authority_and_revision",
    "governance.decision_authority",
    "governance.required_approvals",
    "governance.exception_policy",
    "engineering_posture.dimensions",
    "engineering_posture.red_lines",
    "governance.review_triggers",
    "governance.reassessment_triggers",
    "decisions.records",
];

const WAIVABLE_COVERAGE: [&str; 10] = [
    "delivery.default_implications",
    "policy.authority_and_revision",
    "governance.decision_authority",
    "governance.required_approvals",
    "governance.exception_policy",
    "engineering_posture.dimensions",
    "engineering_posture.red_lines",
    "governance.review_triggers",
    "governance.reassessment_triggers",
    "decisions.records",
];

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CharterAcquisitionMode {
    GuidedAdaptive,
    Express,
    AgentAssisted,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CharterIntakeSourceKind {
    UserDeclaration,
    EvidencedInference,
    DeterministicDefault,
    KnownUnknown,
    Contradiction,
    Waiver,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CharterCoverageSubmission {
    pub coverage_id: String,
    pub source_kind: CharterIntakeSourceKind,
    pub value_ref: String,
    pub evidence_refs: Vec<String>,
    pub confidence: String,
    pub freshness: Option<String>,
    pub sensitivity: String,
    pub contradiction_refs: Vec<String>,
    pub waiver_ref: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CharterIntakeEnvelope {
    pub mode: CharterAcquisitionMode,
    pub content: Value,
    pub coverage: Vec<CharterCoverageSubmission>,
    pub consumer: CharterIntakeConsumer,
    pub prompt_event_refs: Vec<String>,
    pub finalized_at_utc: String,
    pub expected_current_fingerprint: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CharterIntakeConsumer {
    pub kind: String,
    pub id: String,
    pub version: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CharterCoverageResult {
    pub coverage_id: String,
    pub applicability: String,
    pub source_kind: CharterIntakeSourceKind,
    pub value_ref: String,
    pub evidence_refs: Vec<String>,
    pub confidence: String,
    pub freshness: Option<String>,
    pub sensitivity: String,
    pub evaluation: String,
    pub contradiction_refs: Vec<String>,
    pub waiver_ref: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CharterIntakeRecordV11 {
    pub schema_id: String,
    pub schema_version: String,
    pub intake_record_id: String,
    pub intake_definition_ref: String,
    pub acquisition_mode: CharterAcquisitionMode,
    pub target_kind_ref: String,
    pub target_instance_id: String,
    pub profile_ref: String,
    pub resolved_profile_fingerprint: String,
    pub basis_artifact_fingerprint: Option<String>,
    pub consumer: CharterIntakeConsumer,
    pub coverage_results: Vec<CharterCoverageResult>,
    pub prompt_event_refs: Vec<String>,
    pub finalized_at_utc: String,
    pub record_fingerprint: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CharterFieldSource {
    pub target_path: String,
    pub coverage_id: String,
    pub source_kind: CharterIntakeSourceKind,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CharterCandidateV11 {
    pub schema_id: String,
    pub schema_version: String,
    pub candidate_id: String,
    pub intake_record_ref: String,
    pub target_kind_ref: String,
    pub target_instance_id: String,
    pub target_schema_ref: String,
    pub profile_ref: String,
    pub resolved_profile_fingerprint: String,
    pub basis_artifact_fingerprint: Option<String>,
    pub normalized_content_ref: String,
    pub field_sources: Vec<CharterFieldSource>,
    pub validation_result_refs: Vec<String>,
    pub unresolved_coverage_ids: Vec<String>,
    pub promotion_eligibility: String,
    pub required_approval_policy_ref: String,
    pub candidate_fingerprint: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CharterCandidateBundle {
    pub intake: CharterIntakeRecordV11,
    pub candidate: CharterCandidateV11,
    pub normalized_content: Vec<u8>,
    pub charter: CanonicalCharter,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CharterIntakeErrorKind {
    CoverageMismatch,
    SourceAuthorityMismatch,
    UnknownOrContradictedCoverage,
    WaiverMismatch,
    BasisMismatch,
    CandidateContentInvalid,
    FingerprintFailed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CharterIntakeError {
    kind: CharterIntakeErrorKind,
    detail: String,
}

impl CharterIntakeError {
    fn new(kind: CharterIntakeErrorKind, detail: impl Into<String>) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }

    pub fn kind(&self) -> CharterIntakeErrorKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

impl From<CharterArtifactError> for CharterIntakeError {
    fn from(error: CharterArtifactError) -> Self {
        Self::new(
            CharterIntakeErrorKind::CandidateContentInvalid,
            error.detail().to_owned(),
        )
    }
}

pub fn evaluate_charter_intake(
    decisions: &ResolvedProfileDecisions,
    envelope: CharterIntakeEnvelope,
    observed_current_fingerprint: Option<&DefinitionFingerprint>,
) -> Result<CharterCandidateBundle, CharterIntakeError> {
    let basis = validate_basis(
        envelope.expected_current_fingerprint.as_deref(),
        observed_current_fingerprint,
    )?;
    let submissions = validate_and_order_coverage(&envelope.coverage)?;
    let charter: CanonicalCharter =
        serde_json::from_value(envelope.content.clone()).map_err(|_| {
            CharterIntakeError::new(
                CharterIntakeErrorKind::CandidateContentInvalid,
                "candidate content could not be closed-decoded as CanonicalCharter",
            )
        })?;
    let normalized_content = serialize_canonical_charter(decisions, &charter)?;
    let normalized_content_fingerprint = DefinitionFingerprint::from_bytes(&normalized_content);
    validate_audit_fields(&envelope)?;
    let field_sources = field_sources(&envelope.content, &submissions)?;

    let coverage_results = submissions
        .values()
        .map(|submission| CharterCoverageResult {
            coverage_id: submission.coverage_id.clone(),
            applicability: "applicable".to_owned(),
            source_kind: submission.source_kind,
            value_ref: submission.value_ref.clone(),
            evidence_refs: submission.evidence_refs.clone(),
            confidence: submission.confidence.clone(),
            freshness: submission.freshness.clone(),
            sensitivity: submission.sensitivity.clone(),
            evaluation: if submission.waiver_ref.is_some() {
                "waived".to_owned()
            } else {
                "satisfied".to_owned()
            },
            contradiction_refs: submission.contradiction_refs.clone(),
            waiver_ref: submission.waiver_ref.clone(),
        })
        .collect::<Vec<_>>();

    let profile_ref = decisions.profile_ref().as_str().to_owned();
    let profile_fingerprint = decisions.profile_definition_fingerprint().to_string();
    let normalized_content_ref = format!(
        "candidate-content/charter_{}.yaml",
        fingerprint_hex(&normalized_content_fingerprint)
    );
    let intake_identity = fingerprint_value(serde_json::json!({
        "schema_id": "handbook.artifact-intake-record",
        "schema_version": "1.1",
        "intake_definition_ref": CHARTER_INTAKE_REF,
        "acquisition_mode": envelope.mode,
        "target_kind_ref": CHARTER_KIND_REF,
        "target_instance_id": CHARTER_INSTANCE_ID,
        "profile_ref": decisions.profile_ref().as_str(),
        "resolved_profile_fingerprint": decisions.profile_definition_fingerprint().to_string(),
        "basis_artifact_fingerprint": basis.clone(),
        "consumer": envelope.consumer.clone(),
        "coverage_results": coverage_results.clone(),
        "prompt_event_refs": envelope.prompt_event_refs.clone(),
    }))?;
    let intake_id = format!("intake_{}", fingerprint_hex(&intake_identity));
    let intake_record_ref = format!("intake-records/{intake_id}.json");

    let candidate_identity = fingerprint_value(serde_json::json!({
        "schema_id": "handbook.artifact-candidate",
        "schema_version": "1.1",
        "intake_record_ref": intake_record_ref.clone(),
        "target_kind_ref": CHARTER_KIND_REF,
        "target_instance_id": CHARTER_INSTANCE_ID,
        "target_schema_ref": CHARTER_SCHEMA_REF,
        "profile_ref": profile_ref,
        "resolved_profile_fingerprint": profile_fingerprint,
        "basis_artifact_fingerprint": basis.clone(),
        "normalized_content_ref": normalized_content_ref.clone(),
        "field_sources": field_sources.clone(),
        "validation_result_refs": [],
        "unresolved_coverage_ids": [],
        "promotion_eligibility": "requires_approval",
        "required_approval_policy_ref": CHARTER_APPROVAL_POLICY_REF,
    }))?;
    let candidate_hex = fingerprint_hex(&candidate_identity);
    let candidate_id = format!("candidate_{candidate_hex}");
    let intake = CharterIntakeRecordV11 {
        schema_id: "handbook.artifact-intake-record".to_owned(),
        schema_version: "1.1".to_owned(),
        intake_record_id: intake_id.clone(),
        intake_definition_ref: CHARTER_INTAKE_REF.to_owned(),
        acquisition_mode: envelope.mode,
        target_kind_ref: CHARTER_KIND_REF.to_owned(),
        target_instance_id: CHARTER_INSTANCE_ID.to_owned(),
        profile_ref: decisions.profile_ref().as_str().to_owned(),
        resolved_profile_fingerprint: decisions.profile_definition_fingerprint().to_string(),
        basis_artifact_fingerprint: basis.clone(),
        consumer: envelope.consumer,
        coverage_results,
        prompt_event_refs: envelope.prompt_event_refs,
        finalized_at_utc: envelope.finalized_at_utc,
        record_fingerprint: intake_identity.to_string(),
    };
    let candidate = CharterCandidateV11 {
        schema_id: "handbook.artifact-candidate".to_owned(),
        schema_version: "1.1".to_owned(),
        candidate_id,
        intake_record_ref,
        target_kind_ref: CHARTER_KIND_REF.to_owned(),
        target_instance_id: CHARTER_INSTANCE_ID.to_owned(),
        target_schema_ref: CHARTER_SCHEMA_REF.to_owned(),
        profile_ref: decisions.profile_ref().as_str().to_owned(),
        resolved_profile_fingerprint: decisions.profile_definition_fingerprint().to_string(),
        basis_artifact_fingerprint: basis,
        normalized_content_ref,
        field_sources,
        validation_result_refs: Vec::new(),
        unresolved_coverage_ids: Vec::new(),
        promotion_eligibility: "requires_approval".to_owned(),
        required_approval_policy_ref: CHARTER_APPROVAL_POLICY_REF.to_owned(),
        candidate_fingerprint: candidate_identity.to_string(),
    };

    Ok(CharterCandidateBundle {
        intake,
        candidate,
        normalized_content,
        charter,
    })
}

fn validate_basis(
    expected: Option<&str>,
    observed: Option<&DefinitionFingerprint>,
) -> Result<Option<String>, CharterIntakeError> {
    match (expected, observed) {
        (None, None) => Ok(None),
        (Some(expected), Some(observed)) if expected == observed.to_string() => {
            Ok(Some(expected.to_owned()))
        }
        (Some(_), None) => Err(CharterIntakeError::new(
            CharterIntakeErrorKind::BasisMismatch,
            "expected-current must be omitted for initial Charter creation",
        )),
        (None, Some(_)) => Err(CharterIntakeError::new(
            CharterIntakeErrorKind::BasisMismatch,
            "expected-current is mandatory for a Charter amendment",
        )),
        (Some(_), Some(_)) => Err(CharterIntakeError::new(
            CharterIntakeErrorKind::BasisMismatch,
            "expected-current does not match observed canonical Charter bytes",
        )),
    }
}

fn validate_and_order_coverage(
    coverage: &[CharterCoverageSubmission],
) -> Result<BTreeMap<usize, &CharterCoverageSubmission>, CharterIntakeError> {
    if coverage.len() != COVERAGE_ORDER.len() {
        return Err(CharterIntakeError::new(
            CharterIntakeErrorKind::CoverageMismatch,
            "Charter intake requires exactly sixteen coverage submissions",
        ));
    }
    let mut indexed = BTreeMap::new();
    let mut seen = BTreeSet::new();
    for submission in coverage {
        let Some(index) = COVERAGE_ORDER
            .iter()
            .position(|coverage_id| *coverage_id == submission.coverage_id)
        else {
            return Err(CharterIntakeError::new(
                CharterIntakeErrorKind::CoverageMismatch,
                format!("unknown Charter coverage ID `{}`", submission.coverage_id),
            ));
        };
        if !seen.insert(submission.coverage_id.as_str()) {
            return Err(CharterIntakeError::new(
                CharterIntakeErrorKind::CoverageMismatch,
                format!("duplicate Charter coverage ID `{}`", submission.coverage_id),
            ));
        }
        if submission.value_ref.trim().is_empty()
            || submission.confidence != "high"
            || submission.sensitivity.trim().is_empty()
        {
            return Err(CharterIntakeError::new(
                CharterIntakeErrorKind::CoverageMismatch,
                format!("coverage `{}` is incomplete", submission.coverage_id),
            ));
        }
        if !submission.contradiction_refs.is_empty()
            || matches!(
                submission.source_kind,
                CharterIntakeSourceKind::KnownUnknown | CharterIntakeSourceKind::Contradiction
            )
        {
            return Err(CharterIntakeError::new(
                CharterIntakeErrorKind::UnknownOrContradictedCoverage,
                format!(
                    "coverage `{}` is unknown or contradicted",
                    submission.coverage_id
                ),
            ));
        }
        if NORMATIVE_COVERAGE.contains(&submission.coverage_id.as_str())
            && submission.source_kind != CharterIntakeSourceKind::UserDeclaration
        {
            return Err(CharterIntakeError::new(
                CharterIntakeErrorKind::SourceAuthorityMismatch,
                format!(
                    "normative coverage `{}` requires user_declaration",
                    submission.coverage_id
                ),
            ));
        }
        if !NORMATIVE_COVERAGE.contains(&submission.coverage_id.as_str())
            && !matches!(
                submission.source_kind,
                CharterIntakeSourceKind::UserDeclaration
                    | CharterIntakeSourceKind::EvidencedInference
            )
        {
            return Err(CharterIntakeError::new(
                CharterIntakeErrorKind::SourceAuthorityMismatch,
                format!(
                    "coverage `{}` has an inadmissible source",
                    submission.coverage_id
                ),
            ));
        }
        if submission.source_kind == CharterIntakeSourceKind::EvidencedInference
            && submission.evidence_refs.is_empty()
        {
            return Err(CharterIntakeError::new(
                CharterIntakeErrorKind::SourceAuthorityMismatch,
                format!("coverage `{}` requires evidence", submission.coverage_id),
            ));
        }
        if submission.waiver_ref.is_some()
            && (!WAIVABLE_COVERAGE.contains(&submission.coverage_id.as_str())
                || submission.source_kind != CharterIntakeSourceKind::UserDeclaration)
        {
            return Err(CharterIntakeError::new(
                CharterIntakeErrorKind::WaiverMismatch,
                format!(
                    "coverage `{}` cannot use this waiver",
                    submission.coverage_id
                ),
            ));
        }
        indexed.insert(index, submission);
    }
    Ok(indexed)
}

fn field_sources(
    content: &Value,
    submissions: &BTreeMap<usize, &CharterCoverageSubmission>,
) -> Result<Vec<CharterFieldSource>, CharterIntakeError> {
    let mut leaves = Vec::new();
    collect_leaves(content, "", &mut leaves);
    leaves.retain(|path| {
        !matches!(
            path.as_str(),
            "/schema_id" | "/schema_version" | "/posture/rubric_scale"
        )
    });
    let mut sources = Vec::with_capacity(leaves.len());
    for path in leaves {
        let coverage_id = coverage_for_path(&path).ok_or_else(|| {
            CharterIntakeError::new(
                CharterIntakeErrorKind::CoverageMismatch,
                format!("candidate leaf `{path}` has no Charter coverage owner"),
            )
        })?;
        let index = COVERAGE_ORDER
            .iter()
            .position(|candidate| *candidate == coverage_id)
            .expect("closed coverage ID has an index");
        let submission = submissions[&index];
        sources.push(CharterFieldSource {
            target_path: path,
            coverage_id: coverage_id.to_owned(),
            source_kind: submission.source_kind,
        });
    }
    sources.sort_by(|left, right| left.target_path.cmp(&right.target_path));
    Ok(sources)
}

fn validate_audit_fields(envelope: &CharterIntakeEnvelope) -> Result<(), CharterIntakeError> {
    if envelope.consumer.kind.trim().is_empty()
        || envelope.consumer.id.trim().is_empty()
        || envelope.consumer.version.trim().is_empty()
        || !is_utc_timestamp(&envelope.finalized_at_utc)
    {
        return Err(CharterIntakeError::new(
            CharterIntakeErrorKind::CoverageMismatch,
            "intake consumer provenance and finalized_at_utc must be explicit and bounded",
        ));
    }
    Ok(())
}

fn is_utc_timestamp(value: &str) -> bool {
    value.len() >= 20
        && value.ends_with('Z')
        && value.as_bytes().get(4) == Some(&b'-')
        && value.as_bytes().get(7) == Some(&b'-')
        && value.as_bytes().get(10) == Some(&b'T')
        && value.as_bytes().get(13) == Some(&b':')
        && value.as_bytes().get(16) == Some(&b':')
}

fn collect_leaves(value: &Value, path: &str, leaves: &mut Vec<String>) {
    match value {
        Value::Object(object) => {
            for (key, child) in object {
                collect_leaves(child, &format!("{path}/{}", escape_pointer(key)), leaves);
            }
        }
        Value::Array(items) if items.is_empty() => leaves.push(path.to_owned()),
        Value::Array(items) => {
            for (index, child) in items.iter().enumerate() {
                collect_leaves(child, &format!("{path}/{index}"), leaves);
            }
        }
        _ => leaves.push(path.to_owned()),
    }
}

fn escape_pointer(value: &str) -> String {
    value.replace('~', "~0").replace('/', "~1")
}

fn coverage_for_path(path: &str) -> Option<&'static str> {
    const TARGETS: [(&str, &[&str]); 16] = [
        (
            "project_shape.definition",
            &[
                "/record_id",
                "/project/name",
                "/project/classification",
                "/project/team_size",
                "/project/users",
                "/project/expected_lifetime",
                "/project/surfaces",
                "/project/runtime_environments",
            ],
        ),
        ("delivery.constraints", &["/project/constraints"]),
        (
            "delivery.default_implications",
            &["/project/default_implications"],
        ),
        (
            "operational_reality.production_state",
            &["/project/operational_reality"],
        ),
        ("risk.domains", &["/domains"]),
        (
            "engineering_posture.baseline",
            &["/posture/baseline_level", "/posture/baseline_rationale"],
        ),
        ("policy.authority_and_revision", &["/policy"]),
        (
            "governance.decision_authority",
            &["/governance/decision_authority"],
        ),
        (
            "governance.required_approvals",
            &["/governance/required_approvals"],
        ),
        (
            "governance.exception_policy",
            &[
                "/governance/exception_policy",
                "/governance/exception_process",
            ],
        ),
        (
            "engineering_posture.dimensions",
            &["/engineering_posture/dimensions"],
        ),
        (
            "engineering_posture.red_lines",
            &["/engineering_posture/red_lines"],
        ),
        (
            "governance.review_triggers",
            &["/governance/review_triggers"],
        ),
        (
            "governance.reassessment_triggers",
            &["/governance/reassessment_triggers"],
        ),
        ("debt.register", &["/debt"]),
        ("decisions.records", &["/decision_records"]),
    ];
    TARGETS.iter().find_map(|(coverage_id, targets)| {
        targets
            .iter()
            .any(|target| path == *target || path.starts_with(&format!("{target}/")))
            .then_some(*coverage_id)
    })
}

fn fingerprint_value(value: Value) -> Result<DefinitionFingerprint, CharterIntakeError> {
    let bytes = canonical_json_bytes(&value).map_err(|_| {
        CharterIntakeError::new(
            CharterIntakeErrorKind::FingerprintFailed,
            "RFC 8785 fingerprint input could not be canonicalized",
        )
    })?;
    Ok(DefinitionFingerprint::from_bytes(&bytes))
}

fn fingerprint_hex(fingerprint: &DefinitionFingerprint) -> &str {
    fingerprint
        .as_str()
        .strip_prefix("sha256:")
        .expect("DefinitionFingerprint always uses sha256")
}
