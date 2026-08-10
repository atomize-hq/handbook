//! Closed, transport-free artifact operations over the engine-owned repository.

use handbook_engine::artifact_intake::MAX_ARTIFACT_INPUT_DOCUMENT_BYTES as ENGINE_MAX_INPUT_BYTES;
use handbook_engine::artifact_intake_registry::AcquisitionModeV1;
use handbook_engine::artifact_mutation::{
    ArtifactCandidatePreviewV1, ArtifactMutationServiceV1, GenericExecutionDispositionV1,
    GenericMutationExecutionV1,
};
use handbook_engine::artifact_operations::ArtifactValidationLayerStatusV1;
use handbook_engine::artifact_repository::{ArtifactRepositoryV1, ArtifactTargetV1};
use serde_json::Value;
use std::collections::BTreeMap;
use std::fmt;
use std::path::{Path, PathBuf};

/// Maximum accepted bytes for an Artifact route input document.
pub const MAX_ARTIFACT_INPUT_DOCUMENT_BYTES: usize = ENGINE_MAX_INPUT_BYTES;

/// A recursive, closed projection of an artifact-owned document.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ArtifactDocument {
    Null,
    Boolean(bool),
    Number(String),
    String(String),
    Array(Vec<Self>),
    Object(BTreeMap<String, Self>),
}

/// An opaque, bounded document accepted by an artifact operation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtifactInputDocument {
    bytes: Vec<u8>,
}

impl ArtifactInputDocument {
    pub fn new(bytes: Vec<u8>) -> Result<Self, ArtifactSdkError> {
        if bytes.len() > MAX_ARTIFACT_INPUT_DOCUMENT_BYTES {
            return Err(ArtifactSdkError::new(
                "input document exceeds the 1 MiB limit",
            ));
        }
        Ok(Self { bytes })
    }
}

/// One exact selected artifact target. Its validity remains engine-owned.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtifactSelector {
    kind_ref: String,
    instance_id: String,
}

impl ArtifactSelector {
    pub fn new(kind_ref: impl Into<String>, instance_id: impl Into<String>) -> Self {
        Self {
            kind_ref: kind_ref.into(),
            instance_id: instance_id.into(),
        }
    }
}

/// Repository-bound artifact coordinator. It composes existing engine owners only.
#[derive(Clone, Debug)]
pub struct ArtifactSdk {
    repo_root: PathBuf,
}

impl ArtifactSdk {
    pub fn open(repo_root: impl AsRef<Path>) -> Self {
        Self {
            repo_root: repo_root.as_ref().to_path_buf(),
        }
    }

    pub fn list_kinds(&self) -> Result<ArtifactKindList, ArtifactSdkError> {
        let repository = self.repository()?;
        let selection_fingerprint = repository
            .selection_fingerprint()
            .map_err(ArtifactSdkError::from_display)?
            .to_string();
        let kinds = repository
            .list_kinds()
            .map_err(ArtifactSdkError::from_display)?
            .into_iter()
            .map(|kind| ArtifactKind {
                kind_ref: kind.kind_ref.to_string(),
                definition_fingerprint: kind.definition_fingerprint.to_string(),
                schema_ref: kind.schema_ref.to_string(),
            })
            .collect();
        Ok(ArtifactKindList {
            selection_fingerprint,
            kinds,
        })
    }

    pub fn list_instances(&self) -> Result<ArtifactInstanceList, ArtifactSdkError> {
        let repository = self.repository()?;
        let selection_fingerprint = repository
            .selection_fingerprint()
            .map_err(ArtifactSdkError::from_display)?
            .to_string();
        let instances = repository
            .list_instances()
            .map_err(ArtifactSdkError::from_display)?
            .into_iter()
            .map(|instance| ArtifactInstance {
                instance_id: instance.instance_id.to_string(),
                kind_ref: instance.kind_ref.to_string(),
                canonical_path: instance.canonical_path,
                intake_definition_ref: instance
                    .intake_definition_ref
                    .map(|reference| reference.to_string()),
            })
            .collect();
        Ok(ArtifactInstanceList {
            selection_fingerprint,
            instances,
        })
    }

    pub fn read(
        &self,
        request: ArtifactReadRequest,
    ) -> Result<ArtifactReadResult, ArtifactSdkError> {
        let repository = self.repository()?;
        let target = target(&request.selector)?;
        let result = repository
            .read(target.kind_ref(), target.instance_id())
            .map_err(ArtifactSdkError::from_display)?;
        Ok(ArtifactReadResult {
            operation_id: result.operation_id.as_str().to_owned(),
            operation_context_fingerprint: result.operation_context_fingerprint.to_string(),
            kind_ref: result.kind_ref.to_string(),
            instance_id: result.instance_id.to_string(),
            canonical_path: result.canonical_path,
            artifact_fingerprint: result.artifact_fingerprint.to_string(),
            content: document(result.content),
        })
    }

    pub fn validate(
        &self,
        request: ArtifactValidateRequest,
    ) -> Result<ArtifactValidationResult, ArtifactSdkError> {
        let repository = self.repository()?;
        let target = target(&request.selector)?;
        let result = repository
            .validate(target.kind_ref(), target.instance_id())
            .map_err(ArtifactSdkError::from_display)?;
        Ok(ArtifactValidationResult {
            operation_id: result.operation_id.as_str().to_owned(),
            operation_context_fingerprint: result.operation_context_fingerprint.to_string(),
            artifact_fingerprint: result.artifact_fingerprint.to_string(),
            content: document(result.content),
            semantic_status: layer_status(result.semantic_status),
            intake_status: layer_status(result.intake_status),
            approval_status: layer_status(result.approval_status),
            external_evidence_status: layer_status(result.external_evidence_status),
        })
    }

    pub fn intake_definition(
        &self,
        request: ArtifactIntakeDefinitionRequest,
    ) -> Result<ArtifactIntakeDefinition, ArtifactSdkError> {
        let repository = self.repository()?;
        let target = target(&request.selector)?;
        let definition = repository
            .intake_definition(&target)
            .map_err(ArtifactSdkError::from_display)?;
        Ok(ArtifactIntakeDefinition {
            kind_ref: target.kind_ref().to_string(),
            instance_id: target.instance_id().to_string(),
            intake_definition_ref: definition.exact_ref().to_string(),
            intake_definition_fingerprint: definition.definition_fingerprint().to_string(),
            candidate_schema_ref: definition.candidate_schema_ref().to_string(),
            coverage: definition
                .coverage()
                .iter()
                .map(|row| ArtifactCoverageDefinition {
                    coverage_id: row.coverage_id().to_owned(),
                    target_paths: row.target_paths().to_vec(),
                    minimum_specificity: match row.minimum_specificity() {
                        handbook_engine::artifact_intake_registry::CoverageMinimumSpecificityV1::Concrete => {
                            ArtifactCoverageMinimumSpecificity::Concrete
                        }
                        handbook_engine::artifact_intake_registry::CoverageMinimumSpecificityV1::Exact => {
                            ArtifactCoverageMinimumSpecificity::Exact
                        }
                    },
                })
                .collect(),
        })
    }

    pub fn evaluate_intake(
        &self,
        request: ArtifactIntakeEvaluateRequest,
    ) -> Result<ArtifactIntakeEvaluation, ArtifactSdkError> {
        let repository = self.repository()?;
        let target = target(&request.selector)?;
        let evaluation = repository
            .evaluate_intake_document(
                &target,
                acquisition_mode(request.mode),
                request.expected_current_fingerprint.as_deref(),
                &request.input.bytes,
            )
            .map_err(ArtifactSdkError::from_display)?;
        let value = evaluation
            .to_json_value()
            .map_err(ArtifactSdkError::from_display)?;
        Ok(ArtifactIntakeEvaluation {
            document: document(value),
        })
    }

    pub fn intake_append(
        &self,
        request: ArtifactIntakeAppendRequest,
    ) -> Result<ArtifactMutationExecution, ArtifactSdkError> {
        ArtifactMutationServiceV1::intake_append(
            &self.repo_root,
            &request.selector.kind_ref,
            &request.selector.instance_id,
            &request.input.bytes,
        )
        .map(mutation_execution)
        .map_err(ArtifactSdkError::from_display)
    }

    pub fn candidate_validate(
        &self,
        request: ArtifactCandidateValidateRequest,
    ) -> Result<ArtifactCandidateValidation, ArtifactSdkError> {
        let preview = ArtifactMutationServiceV1::candidate_validate(
            &self.repo_root,
            &request.selector.kind_ref,
            &request.selector.instance_id,
            &request.intake_record_ref,
            &request.intake_record_fingerprint,
            request.expected_current_artifact_fingerprint.as_deref(),
        )
        .map_err(ArtifactSdkError::from_display)?;
        Ok(ArtifactCandidateValidation {
            document: document(candidate_preview(preview)?),
        })
    }

    pub fn candidate_append(
        &self,
        request: ArtifactCandidateAppendRequest,
    ) -> Result<ArtifactMutationExecution, ArtifactSdkError> {
        ArtifactMutationServiceV1::candidate_append(
            &self.repo_root,
            &request.selector.kind_ref,
            &request.selector.instance_id,
            &request.input.bytes,
        )
        .map(mutation_execution)
        .map_err(ArtifactSdkError::from_display)
    }

    pub fn promote(
        &self,
        request: ArtifactPromoteRequest,
    ) -> Result<ArtifactMutationExecution, ArtifactSdkError> {
        ArtifactMutationServiceV1::promote(
            &self.repo_root,
            &request.selector.kind_ref,
            &request.selector.instance_id,
            &request.input.bytes,
        )
        .map(mutation_execution)
        .map_err(ArtifactSdkError::from_display)
    }

    fn repository(&self) -> Result<ArtifactRepositoryV1, ArtifactSdkError> {
        ArtifactRepositoryV1::open(&self.repo_root).map_err(ArtifactSdkError::from_display)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtifactKindList {
    pub selection_fingerprint: String,
    pub kinds: Vec<ArtifactKind>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtifactKind {
    pub kind_ref: String,
    pub definition_fingerprint: String,
    pub schema_ref: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtifactInstanceList {
    pub selection_fingerprint: String,
    pub instances: Vec<ArtifactInstance>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtifactInstance {
    pub instance_id: String,
    pub kind_ref: String,
    pub canonical_path: String,
    pub intake_definition_ref: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtifactReadRequest {
    pub selector: ArtifactSelector,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtifactReadResult {
    pub operation_id: String,
    pub operation_context_fingerprint: String,
    pub kind_ref: String,
    pub instance_id: String,
    pub canonical_path: String,
    pub artifact_fingerprint: String,
    pub content: ArtifactDocument,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtifactValidateRequest {
    pub selector: ArtifactSelector,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtifactValidationResult {
    pub operation_id: String,
    pub operation_context_fingerprint: String,
    pub artifact_fingerprint: String,
    pub content: ArtifactDocument,
    pub semantic_status: ArtifactValidationLayerStatus,
    pub intake_status: ArtifactValidationLayerStatus,
    pub approval_status: ArtifactValidationLayerStatus,
    pub external_evidence_status: ArtifactValidationLayerStatus,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ArtifactValidationLayerStatus {
    Pass,
    NotApplicable,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtifactIntakeDefinitionRequest {
    pub selector: ArtifactSelector,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtifactIntakeDefinition {
    pub kind_ref: String,
    pub instance_id: String,
    pub intake_definition_ref: String,
    pub intake_definition_fingerprint: String,
    pub candidate_schema_ref: String,
    pub coverage: Vec<ArtifactCoverageDefinition>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtifactCoverageDefinition {
    pub coverage_id: String,
    pub target_paths: Vec<String>,
    pub minimum_specificity: ArtifactCoverageMinimumSpecificity,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ArtifactCoverageMinimumSpecificity {
    Concrete,
    Exact,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ArtifactAcquisitionMode {
    GuidedAdaptive,
    Express,
    AgentAssisted,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtifactIntakeEvaluateRequest {
    pub selector: ArtifactSelector,
    pub mode: ArtifactAcquisitionMode,
    pub expected_current_fingerprint: Option<String>,
    pub input: ArtifactInputDocument,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtifactIntakeEvaluation {
    pub document: ArtifactDocument,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtifactIntakeAppendRequest {
    pub selector: ArtifactSelector,
    pub input: ArtifactInputDocument,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtifactCandidateValidateRequest {
    pub selector: ArtifactSelector,
    pub intake_record_ref: String,
    pub intake_record_fingerprint: String,
    pub expected_current_artifact_fingerprint: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtifactCandidateValidation {
    pub document: ArtifactDocument,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtifactCandidateAppendRequest {
    pub selector: ArtifactSelector,
    pub input: ArtifactInputDocument,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtifactPromoteRequest {
    pub selector: ArtifactSelector,
    pub input: ArtifactInputDocument,
}

/// A typed projection of a durable receipt reference, without its record payload.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtifactReceipt {
    pub authority_class: ArtifactAuthorityClass,
    pub reference: String,
    pub fingerprint: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ArtifactAuthorityClass {
    SubordinateClosure,
    SemanticRecord,
    CanonicalTruth,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtifactMutationExecution {
    pub result: ArtifactMutationResult,
    pub disposition: ArtifactMutationDisposition,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ArtifactMutationDisposition {
    Committed,
    Refused,
    Replayed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtifactMutationResult {
    pub schema_id: String,
    pub schema_version: String,
    pub operation_id: String,
    pub transaction_id: String,
    pub request_fingerprint: String,
    pub outcome: ArtifactMutationOutcome,
    pub refusal: Option<ArtifactMutationRefusal>,
    pub authoritative_outputs: Vec<ArtifactReceipt>,
    pub transaction_evidence: Option<ArtifactTransactionEvidence>,
    pub result_fingerprint: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ArtifactMutationOutcome {
    Committed,
    Refused,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtifactMutationRefusal {
    pub code: ArtifactMutationRefusalCode,
    pub layer: ArtifactMutationRefusalLayer,
    pub expected_fingerprint: Option<String>,
    pub observed_fingerprint: Option<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ArtifactMutationRefusalCode {
    CanonicalSyntaxInvalid,
    StructuralValidationFailed,
    IntakeCoverageBlocked,
    StaleBasis,
    StaleCurrentArtifact,
    OperationIneligible,
    PublicationBasisConflict,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ArtifactMutationRefusalLayer {
    CanonicalSyntax,
    Structural,
    Intake,
    Currentness,
    Eligibility,
    Publication,
}

/// The receipt retained by the owner to establish a mutation outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtifactTransactionEvidence {
    pub reference: String,
    pub fingerprint: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtifactSdkError {
    detail: String,
}

impl ArtifactSdkError {
    fn new(detail: impl Into<String>) -> Self {
        Self {
            detail: detail.into(),
        }
    }

    fn from_display(error: impl fmt::Display) -> Self {
        Self::new(error.to_string())
    }
}

impl fmt::Display for ArtifactSdkError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.detail)
    }
}

impl std::error::Error for ArtifactSdkError {}

fn target(selector: &ArtifactSelector) -> Result<ArtifactTargetV1, ArtifactSdkError> {
    ArtifactTargetV1::parse(&selector.kind_ref, &selector.instance_id)
        .map_err(ArtifactSdkError::from_display)
}

fn acquisition_mode(mode: ArtifactAcquisitionMode) -> AcquisitionModeV1 {
    match mode {
        ArtifactAcquisitionMode::GuidedAdaptive => AcquisitionModeV1::GuidedAdaptive,
        ArtifactAcquisitionMode::Express => AcquisitionModeV1::Express,
        ArtifactAcquisitionMode::AgentAssisted => AcquisitionModeV1::AgentAssisted,
    }
}

fn layer_status(status: ArtifactValidationLayerStatusV1) -> ArtifactValidationLayerStatus {
    match status {
        ArtifactValidationLayerStatusV1::Pass => ArtifactValidationLayerStatus::Pass,
        ArtifactValidationLayerStatusV1::NotApplicable => {
            ArtifactValidationLayerStatus::NotApplicable
        }
    }
}

fn document(value: Value) -> ArtifactDocument {
    match value {
        Value::Null => ArtifactDocument::Null,
        Value::Bool(value) => ArtifactDocument::Boolean(value),
        Value::Number(value) => ArtifactDocument::Number(value.to_string()),
        Value::String(value) => ArtifactDocument::String(value),
        Value::Array(values) => ArtifactDocument::Array(values.into_iter().map(document).collect()),
        Value::Object(values) => ArtifactDocument::Object(
            values
                .into_iter()
                .map(|(key, value)| (key, document(value)))
                .collect(),
        ),
    }
}

fn candidate_preview(preview: ArtifactCandidatePreviewV1) -> Result<Value, ArtifactSdkError> {
    serde_json::to_value(preview)
        .map_err(|_| ArtifactSdkError::new("candidate preview could not be rendered"))
}

fn mutation_execution(execution: GenericMutationExecutionV1) -> ArtifactMutationExecution {
    let result = execution.result;
    ArtifactMutationExecution {
        result: ArtifactMutationResult {
            schema_id: result.schema_id,
            schema_version: result.schema_version,
            operation_id: result.operation_id,
            transaction_id: result.transaction_id,
            request_fingerprint: result.request_fingerprint,
            outcome: match result.outcome.as_str() {
                "committed" => ArtifactMutationOutcome::Committed,
                "refused" => ArtifactMutationOutcome::Refused,
                _ => unreachable!("engine mutation outcome is closed"),
            },
            refusal: result.refusal.map(|refusal| ArtifactMutationRefusal {
                code: match refusal.code {
                    handbook_engine::artifact_mutation::EstablishedRefusalCodeV1::CanonicalSyntaxInvalid => ArtifactMutationRefusalCode::CanonicalSyntaxInvalid,
                    handbook_engine::artifact_mutation::EstablishedRefusalCodeV1::StructuralValidationFailed => ArtifactMutationRefusalCode::StructuralValidationFailed,
                    handbook_engine::artifact_mutation::EstablishedRefusalCodeV1::IntakeCoverageBlocked => ArtifactMutationRefusalCode::IntakeCoverageBlocked,
                    handbook_engine::artifact_mutation::EstablishedRefusalCodeV1::StaleBasis => ArtifactMutationRefusalCode::StaleBasis,
                    handbook_engine::artifact_mutation::EstablishedRefusalCodeV1::StaleCurrentArtifact => ArtifactMutationRefusalCode::StaleCurrentArtifact,
                    handbook_engine::artifact_mutation::EstablishedRefusalCodeV1::OperationIneligible => ArtifactMutationRefusalCode::OperationIneligible,
                    handbook_engine::artifact_mutation::EstablishedRefusalCodeV1::PublicationBasisConflict => ArtifactMutationRefusalCode::PublicationBasisConflict,
                },
                layer: match refusal.layer {
                    handbook_engine::artifact_mutation::GenericRefusalLayerV1::CanonicalSyntax => ArtifactMutationRefusalLayer::CanonicalSyntax,
                    handbook_engine::artifact_mutation::GenericRefusalLayerV1::Structural => ArtifactMutationRefusalLayer::Structural,
                    handbook_engine::artifact_mutation::GenericRefusalLayerV1::Intake => ArtifactMutationRefusalLayer::Intake,
                    handbook_engine::artifact_mutation::GenericRefusalLayerV1::Currentness => ArtifactMutationRefusalLayer::Currentness,
                    handbook_engine::artifact_mutation::GenericRefusalLayerV1::Eligibility => ArtifactMutationRefusalLayer::Eligibility,
                    handbook_engine::artifact_mutation::GenericRefusalLayerV1::Publication => ArtifactMutationRefusalLayer::Publication,
                },
                expected_fingerprint: refusal.expected_fingerprint,
                observed_fingerprint: refusal.observed_fingerprint,
            }),
            authoritative_outputs: result
                .authoritative_outputs
                .into_iter()
                .map(|output| ArtifactReceipt {
                    authority_class: match output.authority_class {
                        handbook_engine::artifact_mutation::GenericAuthorityClassV1::SubordinateClosure => ArtifactAuthorityClass::SubordinateClosure,
                        handbook_engine::artifact_mutation::GenericAuthorityClassV1::SemanticRecord => ArtifactAuthorityClass::SemanticRecord,
                        handbook_engine::artifact_mutation::GenericAuthorityClassV1::CanonicalTruth => ArtifactAuthorityClass::CanonicalTruth,
                    },
                    reference: output.relative_ref,
                    fingerprint: output.fingerprint,
                })
                .collect(),
            transaction_evidence: result
                .internal_transaction_evidence_ref
                .zip(result.internal_transaction_evidence_fingerprint)
                .map(|(reference, fingerprint)| ArtifactTransactionEvidence { reference, fingerprint }),
            result_fingerprint: result.result_fingerprint,
        },
        disposition: match execution.disposition {
            GenericExecutionDispositionV1::Committed => ArtifactMutationDisposition::Committed,
            GenericExecutionDispositionV1::Refused => ArtifactMutationDisposition::Refused,
            GenericExecutionDispositionV1::Replayed => ArtifactMutationDisposition::Replayed,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn closed_document_projection_preserves_nested_values() {
        let document = document(serde_json::json!({
            "object": {"array": [null, true, 42, "value"]},
        }));

        assert_eq!(
            document,
            ArtifactDocument::Object(BTreeMap::from([(
                "object".to_owned(),
                ArtifactDocument::Object(BTreeMap::from([(
                    "array".to_owned(),
                    ArtifactDocument::Array(vec![
                        ArtifactDocument::Null,
                        ArtifactDocument::Boolean(true),
                        ArtifactDocument::Number("42".to_owned()),
                        ArtifactDocument::String("value".to_owned()),
                    ]),
                )])),
            )])),
        );
    }
}
