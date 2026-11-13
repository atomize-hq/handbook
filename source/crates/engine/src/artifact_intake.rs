use crate::artifact_intake_registry::{
    AcquisitionModeV1, ArtifactIntakeDefinitionV1, CoverageMinimumSpecificityV1,
};
use crate::artifact_operation_context::ArtifactOperationContextV1;
use crate::DefinitionFingerprint;
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

pub const MAX_ARTIFACT_INPUT_DOCUMENT_BYTES: usize = 1024 * 1024;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CoverageSubmissionStateV1 {
    Supplied,
    KnownUnknown,
    Contradicted,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CoverageSourceKindV1 {
    UserDeclaration,
    Evidence,
    Inferred,
    Defaulted,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CoverageSpecificityV1 {
    General,
    Concrete,
    Exact,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CoverageConfidenceV1 {
    Low,
    Medium,
    High,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CoverageSubmissionV1 {
    pub coverage_id: String,
    pub state: CoverageSubmissionStateV1,
    pub source_kind: CoverageSourceKindV1,
    pub value: Option<Value>,
    pub specificity: CoverageSpecificityV1,
    pub confidence: CoverageConfidenceV1,
    pub contradiction_refs: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CoverageInputDocumentV1 {
    pub coverage_submissions: Vec<CoverageSubmissionV1>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CoverageEvaluationDispositionV1 {
    Satisfied,
    Blocked,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CoverageEvaluationOutcomeV1 {
    Complete,
    Blocked,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct CoverageResultV1 {
    pub coverage_id: String,
    pub state: CoverageSubmissionStateV1,
    pub source_kind: CoverageSourceKindV1,
    pub value: Option<Value>,
    pub specificity: CoverageSpecificityV1,
    pub confidence: CoverageConfidenceV1,
    pub contradiction_refs: Vec<String>,
    pub evaluation: CoverageEvaluationDispositionV1,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CoverageFieldSourceV1 {
    pub target_pointer: String,
    pub coverage_id: String,
    pub source_kind: CoverageSourceKindV1,
}

#[derive(Clone, Debug)]
pub struct CoverageEvaluationV1 {
    pub operation_context_fingerprint: DefinitionFingerprint,
    pub acquisition_mode: AcquisitionModeV1,
    pub basis_artifact_fingerprint: Option<DefinitionFingerprint>,
    pub coverage_results: Vec<CoverageResultV1>,
    pub normalized_content: Value,
    pub field_sources: Vec<CoverageFieldSourceV1>,
    pub outcome: CoverageEvaluationOutcomeV1,
    pub evaluation_fingerprint: DefinitionFingerprint,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ArtifactIntakeErrorKindV1 {
    InvalidInputDocument,
    UnsupportedMode,
    MissingCoverage,
    DuplicateCoverage,
    UnknownCoverage,
    BlockingUnknown,
    BlockingContradiction,
    WrongSource,
    MissingValue,
    LowSpecificity,
    LowConfidence,
    InvalidTarget,
    Canonicalization,
}

impl CoverageInputDocumentV1 {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ArtifactIntakeErrorV1> {
        require_artifact_input_document_bound(bytes)?;
        let value = crate::parse_definition_yaml(bytes).map_err(|_| {
            intake_error(
                ArtifactIntakeErrorKindV1::InvalidInputDocument,
                None,
                "coverage input is not one duplicate-safe document",
            )
        })?;
        serde_json::from_value(value).map_err(|_| {
            intake_error(
                ArtifactIntakeErrorKindV1::InvalidInputDocument,
                None,
                "coverage input does not match the closed request shape",
            )
        })
    }
}

pub(crate) fn require_artifact_input_document_bound(
    bytes: &[u8],
) -> Result<(), ArtifactIntakeErrorV1> {
    if bytes.len() > MAX_ARTIFACT_INPUT_DOCUMENT_BYTES {
        return Err(intake_error(
            ArtifactIntakeErrorKindV1::InvalidInputDocument,
            None,
            "artifact input document exceeds the 1 MiB limit",
        ));
    }
    Ok(())
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtifactIntakeErrorV1 {
    kind: ArtifactIntakeErrorKindV1,
    coverage_id: Option<String>,
    detail: &'static str,
}

impl ArtifactIntakeErrorV1 {
    pub fn kind(&self) -> ArtifactIntakeErrorKindV1 {
        self.kind
    }

    pub fn coverage_id(&self) -> Option<&str> {
        self.coverage_id.as_deref()
    }

    pub fn detail(&self) -> &'static str {
        self.detail
    }
}

impl fmt::Display for ArtifactIntakeErrorV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(coverage_id) = &self.coverage_id {
            write!(formatter, "{coverage_id}: {}", self.detail)
        } else {
            formatter.write_str(self.detail)
        }
    }
}

impl std::error::Error for ArtifactIntakeErrorV1 {}

pub fn evaluate_coverage(
    context: &ArtifactOperationContextV1,
    definition: &ArtifactIntakeDefinitionV1,
    acquisition_mode: AcquisitionModeV1,
    basis_artifact_fingerprint: Option<DefinitionFingerprint>,
    submissions: &[CoverageSubmissionV1],
) -> Result<CoverageEvaluationV1, ArtifactIntakeErrorV1> {
    if context.intake_definition_ref() != Some(definition.exact_ref())
        || context.intake_definition_fingerprint() != Some(definition.definition_fingerprint())
    {
        return Err(intake_error(
            ArtifactIntakeErrorKindV1::InvalidTarget,
            None,
            "operation context does not bind the supplied intake definition",
        ));
    }
    if !definition.supported_modes().contains(&acquisition_mode) {
        return Err(intake_error(
            ArtifactIntakeErrorKindV1::UnsupportedMode,
            None,
            "acquisition mode is not selected by the intake definition",
        ));
    }

    let known_ids = definition
        .coverage()
        .iter()
        .map(|row| row.coverage_id())
        .collect::<BTreeSet<_>>();
    let mut by_id = BTreeMap::new();
    for submission in submissions {
        if !known_ids.contains(submission.coverage_id.as_str()) {
            return Err(intake_error(
                ArtifactIntakeErrorKindV1::UnknownCoverage,
                Some(submission.coverage_id.clone()),
                "coverage submission is not selected by the definition",
            ));
        }
        if by_id
            .insert(submission.coverage_id.as_str(), submission)
            .is_some()
        {
            return Err(intake_error(
                ArtifactIntakeErrorKindV1::DuplicateCoverage,
                Some(submission.coverage_id.clone()),
                "coverage submission is duplicated",
            ));
        }
    }

    let mut normalized_content = Value::Object(Map::new());
    let mut coverage_results = Vec::with_capacity(definition.coverage().len());
    let mut field_sources = Vec::new();
    let mut outcome = CoverageEvaluationOutcomeV1::Complete;
    for row in definition.coverage() {
        let submission = by_id.get(row.coverage_id()).copied().ok_or_else(|| {
            intake_error(
                ArtifactIntakeErrorKindV1::MissingCoverage,
                Some(row.coverage_id().to_string()),
                "required coverage submission is missing",
            )
        })?;
        if submission.source_kind != CoverageSourceKindV1::UserDeclaration {
            return Err(intake_error(
                ArtifactIntakeErrorKindV1::WrongSource,
                Some(row.coverage_id().to_string()),
                "coverage requires direct user declaration",
            ));
        }
        if submission.contradiction_refs.len() > 16
            || submission
                .contradiction_refs
                .iter()
                .any(|reference| reference.is_empty() || reference.len() > 1024)
        {
            return Err(intake_error(
                ArtifactIntakeErrorKindV1::BlockingContradiction,
                Some(row.coverage_id().to_string()),
                "coverage contradiction refs are outside the closed limit",
            ));
        }
        let blocked = match submission.state {
            CoverageSubmissionStateV1::Supplied => !submission.contradiction_refs.is_empty(),
            CoverageSubmissionStateV1::KnownUnknown => {
                if submission.value.is_some() || !submission.contradiction_refs.is_empty() {
                    return Err(intake_error(
                        ArtifactIntakeErrorKindV1::BlockingUnknown,
                        Some(row.coverage_id().to_string()),
                        "known-unknown coverage cannot carry a value or contradiction refs",
                    ));
                }
                true
            }
            CoverageSubmissionStateV1::Contradicted => {
                if submission.value.is_some() || submission.contradiction_refs.is_empty() {
                    return Err(intake_error(
                        ArtifactIntakeErrorKindV1::BlockingContradiction,
                        Some(row.coverage_id().to_string()),
                        "contradicted coverage requires refs and cannot carry a value",
                    ));
                }
                true
            }
        };
        if blocked {
            outcome = CoverageEvaluationOutcomeV1::Blocked;
            coverage_results.push(CoverageResultV1 {
                coverage_id: row.coverage_id().to_string(),
                state: submission.state,
                source_kind: submission.source_kind,
                value: None,
                specificity: submission.specificity,
                confidence: submission.confidence,
                contradiction_refs: submission.contradiction_refs.clone(),
                evaluation: CoverageEvaluationDispositionV1::Blocked,
            });
            continue;
        }
        let value = submission.value.as_ref().ok_or_else(|| {
            intake_error(
                ArtifactIntakeErrorKindV1::MissingValue,
                Some(row.coverage_id().to_string()),
                "supplied coverage requires an explicit value",
            )
        })?;
        let required_specificity = match row.minimum_specificity() {
            CoverageMinimumSpecificityV1::Concrete => CoverageSpecificityV1::Concrete,
            CoverageMinimumSpecificityV1::Exact => CoverageSpecificityV1::Exact,
        };
        if submission.specificity < required_specificity {
            return Err(intake_error(
                ArtifactIntakeErrorKindV1::LowSpecificity,
                Some(row.coverage_id().to_string()),
                "coverage specificity is below the definition minimum",
            ));
        }
        if submission.confidence < CoverageConfidenceV1::High {
            return Err(intake_error(
                ArtifactIntakeErrorKindV1::LowConfidence,
                Some(row.coverage_id().to_string()),
                "coverage confidence is below the definition minimum",
            ));
        }

        for target_path in row.target_paths() {
            insert_pointer_value(&mut normalized_content, target_path, value.clone()).map_err(
                |_| {
                    intake_error(
                        ArtifactIntakeErrorKindV1::InvalidTarget,
                        Some(row.coverage_id().to_string()),
                        "coverage target cannot be represented in the JSON data model",
                    )
                },
            )?;
            field_sources.push(CoverageFieldSourceV1 {
                target_pointer: target_path.clone(),
                coverage_id: row.coverage_id().to_string(),
                source_kind: CoverageSourceKindV1::UserDeclaration,
            });
        }
        coverage_results.push(CoverageResultV1 {
            coverage_id: row.coverage_id().to_string(),
            state: submission.state,
            source_kind: submission.source_kind,
            value: submission.value.clone(),
            specificity: submission.specificity,
            confidence: submission.confidence,
            contradiction_refs: submission.contradiction_refs.clone(),
            evaluation: CoverageEvaluationDispositionV1::Satisfied,
        });
    }

    let subject = evaluation_subject(
        context.context_fingerprint(),
        acquisition_mode,
        basis_artifact_fingerprint.as_ref(),
        &coverage_results,
        &normalized_content,
        &field_sources,
        outcome,
    )?;
    let evaluation_fingerprint =
        DefinitionFingerprint::from_json_value(&subject).map_err(|_| {
            intake_error(
                ArtifactIntakeErrorKindV1::Canonicalization,
                None,
                "coverage evaluation canonicalization failed",
            )
        })?;
    Ok(CoverageEvaluationV1 {
        operation_context_fingerprint: context.context_fingerprint().clone(),
        acquisition_mode,
        basis_artifact_fingerprint,
        coverage_results,
        normalized_content,
        field_sources,
        outcome,
        evaluation_fingerprint,
    })
}

impl CoverageEvaluationV1 {
    pub fn to_json_value(&self) -> Result<Value, ArtifactIntakeErrorV1> {
        let mut value = evaluation_subject(
            &self.operation_context_fingerprint,
            self.acquisition_mode,
            self.basis_artifact_fingerprint.as_ref(),
            &self.coverage_results,
            &self.normalized_content,
            &self.field_sources,
            self.outcome,
        )?;
        value
            .as_object_mut()
            .expect("coverage evaluation subject is always an object")
            .insert(
                "evaluation_fingerprint".to_string(),
                Value::String(self.evaluation_fingerprint.to_string()),
            );
        Ok(value)
    }
}

fn evaluation_subject(
    operation_context_fingerprint: &DefinitionFingerprint,
    acquisition_mode: AcquisitionModeV1,
    basis_artifact_fingerprint: Option<&DefinitionFingerprint>,
    coverage_results: &[CoverageResultV1],
    normalized_content: &Value,
    field_sources: &[CoverageFieldSourceV1],
    outcome: CoverageEvaluationOutcomeV1,
) -> Result<Value, ArtifactIntakeErrorV1> {
    let acquisition_mode = serde_json::to_value(acquisition_mode).map_err(|_| {
        intake_error(
            ArtifactIntakeErrorKindV1::Canonicalization,
            None,
            "acquisition mode serialization failed",
        )
    })?;
    let coverage_results = serde_json::to_value(coverage_results).map_err(|_| {
        intake_error(
            ArtifactIntakeErrorKindV1::Canonicalization,
            None,
            "coverage result serialization failed",
        )
    })?;
    let field_sources = serde_json::to_value(field_sources).map_err(|_| {
        intake_error(
            ArtifactIntakeErrorKindV1::Canonicalization,
            None,
            "field-source serialization failed",
        )
    })?;
    Ok(json!({
        "schema_id": "handbook.intake-coverage-evaluation",
        "schema_version": "1.0",
        "operation_context_fingerprint": operation_context_fingerprint.as_str(),
        "acquisition_mode": acquisition_mode,
        "basis_artifact_fingerprint": basis_artifact_fingerprint.map(DefinitionFingerprint::as_str),
        "coverage_results": coverage_results,
        "normalized_content": normalized_content,
        "field_sources": field_sources,
        "outcome": outcome,
    }))
}

fn insert_pointer_value(target: &mut Value, pointer: &str, value: Value) -> Result<(), ()> {
    let tokens = pointer
        .split('/')
        .skip(1)
        .map(|token| token.replace("~1", "/").replace("~0", "~"))
        .collect::<Vec<_>>();
    if tokens.is_empty() {
        return Err(());
    }
    let mut current = target;
    for token in &tokens[..tokens.len() - 1] {
        let object = current.as_object_mut().ok_or(())?;
        current = object
            .entry(token.clone())
            .or_insert_with(|| Value::Object(Map::new()));
    }
    let object = current.as_object_mut().ok_or(())?;
    if object
        .insert(tokens.last().expect("nonempty").clone(), value)
        .is_some()
    {
        return Err(());
    }
    Ok(())
}

fn intake_error(
    kind: ArtifactIntakeErrorKindV1,
    coverage_id: Option<String>,
    detail: &'static str,
) -> ArtifactIntakeErrorV1 {
    ArtifactIntakeErrorV1 {
        kind,
        coverage_id,
        detail,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coverage_document_accepts_the_exact_shared_limit_and_refuses_one_byte_more() {
        let mut exact = br#"{"coverage_submissions":[]}"#.to_vec();
        exact.resize(MAX_ARTIFACT_INPUT_DOCUMENT_BYTES, b' ');
        assert!(CoverageInputDocumentV1::from_bytes(&exact).is_ok());

        let mut over = exact;
        over.push(b' ');
        let error = CoverageInputDocumentV1::from_bytes(&over).unwrap_err();
        assert_eq!(
            error.kind(),
            ArtifactIntakeErrorKindV1::InvalidInputDocument
        );
    }
}
