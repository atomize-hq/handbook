//! Feature-gated JSON Schema descriptions and deterministic checked-in generation.

use super::artifact_operation::{
    ArtifactAuthoritativeOutputV1, ArtifactInstanceDtoV1, ArtifactKindDtoV1,
    ArtifactMutationRefusalV1, ArtifactMutationResultDataV1, ArtifactProblemCauseV1,
    ArtifactProblemDetailsV1, ArtifactRecheckV1, ArtifactWriteConditionV1,
    CandidateAppendRequestV1, CandidateValidateRequestV1, CandidateValidationResultV1,
    CoverageDefinitionDtoV1, CoverageSubmissionDtoV1, EmptyRequestV1, InstanceListResultV1,
    IntakeAppendRequestV1, IntakeDefinitionResultV1, IntakeEvaluateRequestV1,
    IntakeEvaluationResultV1, KindListResultV1, PromoteRequestV1, ReadResultV1, TargetRequestV1,
    ValidateResultV1, ValidationLayersV1,
};
use super::contract_evidence_operation::{
    ContractClaimVerdictDtoV1, ContractEvaluationRunDtoV1, ContractEvidenceAdmissionPolicyDtoV1,
    ContractEvidenceAppendResultV1, ContractEvidenceAppendWireV1, ContractEvidenceCandidateDtoV1,
    ContractEvidenceFreshnessPolicyDtoV1, ContractEvidenceListRequestV1,
    ContractEvidenceListResultV1, ContractEvidenceOperationDetailsDtoV1,
    ContractEvidenceReadRequestV1, ContractEvidenceReadResultV1, ContractEvidenceRecordDtoV1,
    ContractEvidenceStabilityPolicyDtoV1, ContractGateEvaluateRequestV1,
    ContractGateEvaluateResultV1, ContractGatePolicyDtoV1, ContractGateResultDtoV1,
    ContractLocalCloseoutPolicyDtoV1, ContractLocalCloseoutResultDtoV1,
    ContractVerdictEvaluateRequestV1, ContractVerdictEvaluateResultV1,
};
use super::contract_operation::{AppendWireV1, TransitionWireV1};
use super::dock_operation::{
    DockExecutionRecordDtoV1, DockManifestDtoV1, DockManifestListRequestV1,
    DockManifestListResultV1, DockManifestReadRequestV1, DockManifestReadResultV1,
    DockOperationDetailsDtoV1, DockProcessRequestDtoV1, DockProcessResultDtoV1, DockRunResultV1,
    DockRunWireV1,
};
use super::flow_operation::{
    FlowArtifactPresenceV1, FlowBlockerCategoryV1, FlowBlockerV1, FlowBudgetByteDomainV1,
    FlowBudgetDispositionV1, FlowBudgetNextSafeActionV1, FlowBudgetOutcomeV1, FlowBudgetReasonV1,
    FlowBudgetTargetV1, FlowDecisionSummaryV1, FlowFixtureContextV1, FlowNextSafeActionV1,
    FlowPacketBodyNoteKindV1, FlowPacketBodyNoteV1, FlowPacketIdV1, FlowPacketResultV1,
    FlowPacketSectionModeV1, FlowPacketSectionV1, FlowPacketSourceV1, FlowPacketVariantV1,
    FlowProblemCauseV1, FlowProblemDetailsV1, FlowProblemOriginV1, FlowReadyActionV1,
    FlowRecheckV1, FlowRefusalCategoryV1, FlowRefusalV1, FlowResolveRequestV1, FlowResolveResultV1,
    FlowSelectionStatusV1, FlowSelectionV1, FlowSubjectRefV1,
};
use super::pipeline_operation::{
    PipelineActivationClauseV1, PipelineActivationOperatorV1, PipelineActivationV1,
    PipelineCaptureApplyModeV1, PipelineCaptureApplyRequestSchemaV1, PipelineCaptureApplyResultV1,
    PipelineCapturePlanDtoV1, PipelineCapturePlanRequestV1, PipelineCapturePlanResultV1,
    PipelineCaptureStateUpdateDtoV1, PipelineCaptureStateValueDtoV1, PipelineCaptureTargetDtoV1,
    PipelineCatalogEntryV1, PipelineCatalogListResultV1, PipelineCatalogReadRequestV1,
    PipelineCatalogReadResultV1, PipelineCompileDocumentKindV1, PipelineCompileDocumentStatusV1,
    PipelineCompileDocumentV1, PipelineCompileGatingV1, PipelineCompileOutputKindV1,
    PipelineCompileOutputV1, PipelineCompileRefusalClassificationV1, PipelineCompileRefusalV1,
    PipelineCompileRequestV1, PipelineCompileResultV1, PipelineCompileTargetV1,
    PipelineCompileVariableV1, PipelineDeclaredStageV1, PipelineDefinitionV1,
    PipelineEmptyRequestV1, PipelineHandoffEmitRequestSchemaV1, PipelineHandoffEmitResultV1,
    PipelineMutationProblemDetailsV1, PipelineProblemCauseV1, PipelineProblemDetailsV1,
    PipelineRecheckV1, PipelineRouteBasisProfileV1, PipelineRouteBasisRunnerV1,
    PipelineRouteBasisStageV1, PipelineRouteBasisV1,
    PipelineRouteRunV1 as PipelineOperationRouteRunV1,
    PipelineRouteStageReasonV1 as PipelineOperationRouteStageReasonV1,
    PipelineRouteStageStatusV1 as PipelineOperationRouteStageStatusV1, PipelineRouteStateDtoV1,
    PipelineRoutingEntryV1 as PipelineOperationRoutingEntryV1, PipelineStageV1,
    PipelineStateApplyRequestSchemaV1, PipelineStateApplyResultV1, PipelineStateMutationV1,
    PipelineWriteConditionV1, PipelineWriteEffectV1,
};
use super::posture::REQUEST_SCHEMA_REF;
use super::*;
use schemars::generate::SchemaSettings;
use schemars::{json_schema, JsonSchema, Schema, SchemaGenerator};
use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::marker::PhantomData;
use std::path::{Path, PathBuf};

const MAX_SAFE_INTEGER: u64 = 9_007_199_254_740_991;
const MIN_SAFE_INTEGER: i64 = -9_007_199_254_740_991;
const TOKEN_PATTERN: &str = "^[a-z][a-z0-9._-]*$";
const FINGERPRINT_PATTERN: &str = "^sha256:[0-9a-f]{64}$";
const CORRELATION_PATTERN: &str = "^corr_[A-Za-z0-9_-]{16,128}$";
const TIMESTAMP_PATTERN: &str =
    "^[0-9]{4}-(0[1-9]|1[0-2])-([0-2][0-9]|3[01])T([01][0-9]|2[0-3]):[0-5][0-9]:[0-5][0-9]Z$";
const EXACT_REF_PATTERN: &str = concat!(
    "^[a-z][a-z0-9._-]{0,254}@",
    "(0|[1-9][0-9]*)\\.(0|[1-9][0-9]*)\\.(0|[1-9][0-9]*)",
    "(-(0|[1-9][0-9]*|[0-9]*[A-Za-z-][0-9A-Za-z-]*)",
    "(\\.(0|[1-9][0-9]*|[0-9]*[A-Za-z-][0-9A-Za-z-]*))*)?",
    "(\\+[0-9A-Za-z-]+(\\.[0-9A-Za-z-]+)*)?$"
);
const SEMVER_PATTERN: &str = concat!(
    "^(0|[1-9][0-9]*)\\.(0|[1-9][0-9]*)\\.(0|[1-9][0-9]*)",
    "(-(0|[1-9][0-9]*|[0-9]*[A-Za-z-][0-9A-Za-z-]*)",
    "(\\.(0|[1-9][0-9]*|[0-9]*[A-Za-z-][0-9A-Za-z-]*))*)?",
    "(\\+[0-9A-Za-z-]+(\\.[0-9A-Za-z-]+)*)?$"
);

fn closed_object(properties: Vec<(&'static str, Schema)>, required: &[&'static str]) -> Schema {
    let properties = properties.into_iter().collect::<BTreeMap<_, _>>();
    json_schema!({
        "type": "object",
        "properties": properties,
        "required": required,
        "additionalProperties": false
    })
}

fn nullable<T: JsonSchema>(generator: &mut SchemaGenerator) -> Schema {
    let value = generator.subschema_for::<T>();
    json_schema!({"anyOf": [value, {"type": "null"}]})
}

fn string_enum(values: &[&str]) -> Schema {
    json_schema!({"type": "string", "enum": values})
}

macro_rules! impl_schema {
    ($type:ty, $name:literal, |$generator:ident| $schema:expr) => {
        impl JsonSchema for $type {
            fn schema_name() -> Cow<'static, str> {
                $name.into()
            }

            fn json_schema($generator: &mut SchemaGenerator) -> Schema {
                $schema
            }
        }
    };
}

impl<const N: usize> JsonSchema for BoundedString<N> {
    fn schema_name() -> Cow<'static, str> {
        format!("BoundedString_{N}").into()
    }

    fn schema_id() -> Cow<'static, str> {
        format!("handbook_sdk::contract_membrane::BoundedString<{N}>").into()
    }

    fn json_schema(_generator: &mut SchemaGenerator) -> Schema {
        json_schema!({"type": "string", "minLength": 1, "maxLength": N})
    }
}

impl<T: JsonSchema, const N: usize> JsonSchema for BoundedVec<T, N> {
    fn schema_name() -> Cow<'static, str> {
        format!("BoundedVec_{}_{}", T::schema_name(), N).into()
    }

    fn schema_id() -> Cow<'static, str> {
        format!(
            "handbook_sdk::contract_membrane::BoundedVec<{},{}>",
            T::schema_id(),
            N
        )
        .into()
    }

    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        let items = generator.subschema_for::<T>();
        json_schema!({"type": "array", "items": items, "maxItems": N})
    }
}

impl_schema!(SafeU64, "SafeU64", |_generator| json_schema!({
    "type": "integer",
    "minimum": 0,
    "maximum": MAX_SAFE_INTEGER
}));

impl_schema!(SafeI64, "SafeI64", |_generator| json_schema!({
    "type": "integer",
    "minimum": MIN_SAFE_INTEGER,
    "maximum": MAX_SAFE_INTEGER
}));

impl_schema!(UtcTimestamp, "UtcTimestamp", |_generator| json_schema!({
    "type": "string",
    "minLength": 20,
    "maxLength": 20,
    "pattern": TIMESTAMP_PATTERN
}));

impl_schema!(ExactRef, "ExactRef", |_generator| json_schema!({
    "type": "string",
    "minLength": 7,
    "maxLength": 511,
    "pattern": EXACT_REF_PATTERN
}));

impl_schema!(SemVer, "SemVer", |_generator| json_schema!({
    "type": "string",
    "minLength": 5,
    "maxLength": 255,
    "pattern": SEMVER_PATTERN
}));

impl_schema!(Fingerprint, "Fingerprint", |_generator| json_schema!({
    "type": "string",
    "minLength": 71,
    "maxLength": 71,
    "pattern": FINGERPRINT_PATTERN
}));

impl_schema!(ExactBinding, "ExactBinding", |generator| closed_object(
    vec![
        ("ref", generator.subschema_for::<ExactRef>()),
        ("fingerprint", generator.subschema_for::<Fingerprint>()),
    ],
    &["ref", "fingerprint"],
));

impl_schema!(RequestId, "RequestId", |_generator| json_schema!({
    "type": "string",
    "minLength": 1,
    "maxLength": 128,
    "pattern": "^[A-Za-z0-9][A-Za-z0-9._:-]{0,127}$"
}));

impl_schema!(
    crate::artifact::ArtifactDocument,
    "ArtifactDocument",
    |_generator| Schema::from(true)
);

fn artifact_text(max_length: usize) -> Schema {
    json_schema!({"type": "string", "minLength": 1, "maxLength": max_length})
}

fn artifact_fingerprint() -> Schema {
    json_schema!({
        "type": "string",
        "minLength": 71,
        "maxLength": 71,
        "pattern": FINGERPRINT_PATTERN
    })
}

fn artifact_target_properties() -> Vec<(&'static str, Schema)> {
    vec![
        (
            "kind_ref",
            json_schema!({
                "type": "string", "minLength": 7, "maxLength": 511,
                "pattern": EXACT_REF_PATTERN
            }),
        ),
        ("instance_id", artifact_text(255)),
    ]
}

impl_schema!(EmptyRequestV1, "ArtifactEmptyRequestV1", |_generator| {
    closed_object(Vec::new(), &[])
});

impl_schema!(TargetRequestV1, "ArtifactTargetRequestV1", |_generator| {
    closed_object(artifact_target_properties(), &["kind_ref", "instance_id"])
});

impl_schema!(
    CoverageSubmissionDtoV1,
    "CoverageSubmissionDtoV1",
    |generator| {
        closed_object(
            vec![
                ("coverage_id", artifact_text(256)),
                (
                    "state",
                    string_enum(&["supplied", "known_unknown", "contradicted"]),
                ),
                (
                    "source_kind",
                    string_enum(&["user_declaration", "evidence", "inferred", "defaulted"]),
                ),
                (
                    "value",
                    nullable::<crate::artifact::ArtifactDocument>(generator),
                ),
                (
                    "specificity",
                    string_enum(&["general", "concrete", "exact"]),
                ),
                ("confidence", string_enum(&["low", "medium", "high"])),
                (
                    "contradiction_refs",
                    json_schema!({
                        "type": "array", "maxItems": 128,
                        "items": artifact_text(1024)
                    }),
                ),
            ],
            &[
                "coverage_id",
                "state",
                "source_kind",
                "value",
                "specificity",
                "confidence",
                "contradiction_refs",
            ],
        )
    }
);

impl_schema!(
    IntakeEvaluateRequestV1,
    "IntakeEvaluateRequestV1",
    |generator| {
        let mut properties = artifact_target_properties();
        properties.extend([
            (
                "acquisition_mode",
                string_enum(&["guided_adaptive", "express", "agent_assisted"]),
            ),
            (
                "expected_current_artifact_fingerprint",
                nullable::<Fingerprint>(generator),
            ),
            (
                "coverage_submissions",
                json_schema!({
                    "type": "array", "maxItems": 128,
                    "items": generator.subschema_for::<CoverageSubmissionDtoV1>()
                }),
            ),
        ]);
        closed_object(
            properties,
            &[
                "kind_ref",
                "instance_id",
                "acquisition_mode",
                "expected_current_artifact_fingerprint",
                "coverage_submissions",
            ],
        )
    }
);

impl_schema!(
    IntakeAppendRequestV1,
    "IntakeAppendRequestV1",
    |generator| {
        let mut properties = artifact_target_properties();
        properties.extend([
        ("acquisition_mode", string_enum(&["guided_adaptive", "express", "agent_assisted"])),
        ("expected_current_artifact_fingerprint", nullable::<Fingerprint>(generator)),
        ("coverage_submissions", json_schema!({
            "type": "array", "maxItems": 128,
            "items": generator.subschema_for::<CoverageSubmissionDtoV1>()
        })),
        ("idempotency_key", json_schema!({"type": "string", "minLength": 1, "maxLength": 256, "pattern": "^[!-~]+$"})),
    ]);
        closed_object(
            properties,
            &[
                "kind_ref",
                "instance_id",
                "acquisition_mode",
                "expected_current_artifact_fingerprint",
                "coverage_submissions",
                "idempotency_key",
            ],
        )
    }
);

impl_schema!(
    CandidateValidateRequestV1,
    "CandidateValidateRequestV1",
    |generator| {
        let mut properties = artifact_target_properties();
        properties.extend([
            ("intake_record_ref", artifact_text(1024)),
            ("intake_record_fingerprint", artifact_fingerprint()),
            (
                "expected_current_artifact_fingerprint",
                nullable::<Fingerprint>(generator),
            ),
        ]);
        closed_object(
            properties,
            &[
                "kind_ref",
                "instance_id",
                "intake_record_ref",
                "intake_record_fingerprint",
                "expected_current_artifact_fingerprint",
            ],
        )
    }
);

impl_schema!(
    CandidateAppendRequestV1,
    "CandidateAppendRequestV1",
    |_generator| {
        let mut properties = artifact_target_properties();
        properties.extend([
        ("intake_record_ref", artifact_text(1024)),
        ("intake_record_fingerprint", artifact_fingerprint()),
        ("expected_candidate_fingerprint", artifact_fingerprint()),
        ("idempotency_key", json_schema!({"type": "string", "minLength": 1, "maxLength": 256, "pattern": "^[!-~]+$"})),
    ]);
        closed_object(
            properties,
            &[
                "kind_ref",
                "instance_id",
                "intake_record_ref",
                "intake_record_fingerprint",
                "expected_candidate_fingerprint",
                "idempotency_key",
            ],
        )
    }
);

impl_schema!(PromoteRequestV1, "PromoteRequestV1", |generator| {
    let mut properties = artifact_target_properties();
    properties.extend([
        ("candidate_ref", artifact_text(1024)),
        ("candidate_fingerprint", artifact_fingerprint()),
        ("expected_current_artifact_fingerprint", nullable::<Fingerprint>(generator)),
        ("publisher_authority", nullable::<crate::artifact::ArtifactDocument>(generator)),
        ("idempotency_key", json_schema!({"type": "string", "minLength": 1, "maxLength": 256, "pattern": "^[!-~]+$"})),
    ]);
    closed_object(
        properties,
        &[
            "kind_ref",
            "instance_id",
            "candidate_ref",
            "candidate_fingerprint",
            "expected_current_artifact_fingerprint",
            "publisher_authority",
            "idempotency_key",
        ],
    )
});

impl_schema!(ArtifactKindDtoV1, "ArtifactKindDtoV1", |_generator| {
    closed_object(
        vec![
            ("kind_ref", artifact_text(511)),
            ("definition_fingerprint", artifact_fingerprint()),
            ("schema_ref", artifact_text(511)),
        ],
        &["kind_ref", "definition_fingerprint", "schema_ref"],
    )
});

impl_schema!(KindListResultV1, "KindListResultV1", |generator| {
    closed_object(
        vec![
            (
                "operation_id",
                json_schema!({"const": "artifact.kind.list"}),
            ),
            ("selection_fingerprint", artifact_fingerprint()),
            (
                "kinds",
                json_schema!({"type": "array", "maxItems": 128, "items": generator.subschema_for::<ArtifactKindDtoV1>()}),
            ),
        ],
        &["operation_id", "selection_fingerprint", "kinds"],
    )
});

impl_schema!(
    ArtifactInstanceDtoV1,
    "ArtifactInstanceDtoV1",
    |generator| closed_object(
        vec![
            ("instance_id", artifact_text(255)),
            ("kind_ref", artifact_text(511)),
            ("canonical_path", artifact_text(4096)),
            (
                "intake_definition_ref",
                nullable::<BoundedString<511>>(generator)
            ),
        ],
        &[
            "instance_id",
            "kind_ref",
            "canonical_path",
            "intake_definition_ref"
        ],
    )
);

impl_schema!(InstanceListResultV1, "InstanceListResultV1", |generator| {
    closed_object(
        vec![
            (
                "operation_id",
                json_schema!({"const": "artifact.instance.list"}),
            ),
            ("selection_fingerprint", artifact_fingerprint()),
            (
                "instances",
                json_schema!({"type": "array", "maxItems": 128, "items": generator.subschema_for::<ArtifactInstanceDtoV1>()}),
            ),
        ],
        &["operation_id", "selection_fingerprint", "instances"],
    )
});

impl_schema!(ReadResultV1, "ReadResultV1", |generator| closed_object(
    vec![
        ("operation_id", json_schema!({"const": "artifact.read"})),
        ("operation_context_fingerprint", artifact_fingerprint()),
        ("kind_ref", artifact_text(511)),
        ("instance_id", artifact_text(255)),
        ("canonical_path", artifact_text(4096)),
        ("artifact_fingerprint", artifact_fingerprint()),
        (
            "content",
            generator.subschema_for::<crate::artifact::ArtifactDocument>()
        ),
    ],
    &[
        "operation_id",
        "operation_context_fingerprint",
        "kind_ref",
        "instance_id",
        "canonical_path",
        "artifact_fingerprint",
        "content"
    ],
));

impl_schema!(ValidationLayersV1, "ValidationLayersV1", |_generator| {
    let layer = || string_enum(&["pass", "not_applicable"]);
    closed_object(
        vec![
            ("structural", layer()),
            ("semantic", layer()),
            ("intake", layer()),
            ("approval", layer()),
            ("external_evidence", layer()),
        ],
        &[
            "structural",
            "semantic",
            "intake",
            "approval",
            "external_evidence",
        ],
    )
});

impl_schema!(ValidateResultV1, "ValidateResultV1", |generator| {
    closed_object(
        vec![
            ("operation_id", json_schema!({"const": "artifact.validate"})),
            ("operation_context_fingerprint", artifact_fingerprint()),
            ("artifact_fingerprint", artifact_fingerprint()),
            ("outcome", json_schema!({"const": "valid"})),
            (
                "content",
                generator.subschema_for::<crate::artifact::ArtifactDocument>(),
            ),
            ("layers", generator.subschema_for::<ValidationLayersV1>()),
        ],
        &[
            "operation_id",
            "operation_context_fingerprint",
            "artifact_fingerprint",
            "outcome",
            "content",
            "layers",
        ],
    )
});

impl_schema!(
    CoverageDefinitionDtoV1,
    "CoverageDefinitionDtoV1",
    |_generator| closed_object(
        vec![
            ("coverage_id", artifact_text(256)),
            (
                "target_paths",
                json_schema!({"type": "array", "maxItems": 128, "items": artifact_text(255)})
            ),
            ("minimum_specificity", string_enum(&["concrete", "exact"])),
        ],
        &["coverage_id", "target_paths", "minimum_specificity"],
    )
);

impl_schema!(
    IntakeDefinitionResultV1,
    "IntakeDefinitionResultV1",
    |generator| closed_object(
        vec![
            (
                "operation_id",
                json_schema!({"const": "intake.definition.read"})
            ),
            ("kind_ref", artifact_text(511)),
            ("instance_id", artifact_text(255)),
            ("intake_definition_ref", artifact_text(511)),
            ("intake_definition_fingerprint", artifact_fingerprint()),
            ("candidate_schema_ref", artifact_text(511)),
            (
                "coverage",
                json_schema!({"type": "array", "maxItems": 128, "items": generator.subschema_for::<CoverageDefinitionDtoV1>()})
            ),
        ],
        &[
            "operation_id",
            "kind_ref",
            "instance_id",
            "intake_definition_ref",
            "intake_definition_fingerprint",
            "candidate_schema_ref",
            "coverage"
        ],
    )
);

impl_schema!(
    IntakeEvaluationResultV1,
    "IntakeEvaluationResultV1",
    |generator| closed_object(
        vec![
            (
                "schema_id",
                json_schema!({"const": "handbook.intake-coverage-evaluation"})
            ),
            ("schema_version", json_schema!({"const": "1.0"})),
            ("operation_context_fingerprint", artifact_fingerprint()),
            (
                "acquisition_mode",
                string_enum(&["guided_adaptive", "express", "agent_assisted"])
            ),
            (
                "basis_artifact_fingerprint",
                nullable::<Fingerprint>(generator)
            ),
            (
                "coverage_results",
                json_schema!({"type": "array", "maxItems": 128, "items": generator.subschema_for::<crate::artifact::ArtifactDocument>()})
            ),
            (
                "normalized_content",
                generator.subschema_for::<crate::artifact::ArtifactDocument>()
            ),
            (
                "field_sources",
                json_schema!({"type": "array", "maxItems": 128, "items": generator.subschema_for::<crate::artifact::ArtifactDocument>()})
            ),
            ("outcome", string_enum(&["complete", "blocked"])),
            ("evaluation_fingerprint", artifact_fingerprint()),
        ],
        &[
            "schema_id",
            "schema_version",
            "operation_context_fingerprint",
            "acquisition_mode",
            "basis_artifact_fingerprint",
            "coverage_results",
            "normalized_content",
            "field_sources",
            "outcome",
            "evaluation_fingerprint"
        ],
    )
);

impl_schema!(
    CandidateValidationResultV1,
    "CandidateValidationResultV1",
    |generator| closed_object(
        vec![
            (
                "operation_id",
                json_schema!({"const": "artifact.candidate.validate"})
            ),
            ("outcome", json_schema!({"const": "valid"})),
            ("operation_context_fingerprint", artifact_fingerprint()),
            ("intake_record_ref", artifact_text(1024)),
            ("intake_record_fingerprint", artifact_fingerprint()),
            (
                "basis_artifact_fingerprint",
                nullable::<Fingerprint>(generator)
            ),
            (
                "normalized_content",
                generator.subschema_for::<crate::artifact::ArtifactDocument>()
            ),
            ("normalized_content_ref", artifact_text(1024)),
            ("normalized_content_fingerprint", artifact_fingerprint()),
            (
                "validation_result",
                generator.subschema_for::<crate::artifact::ArtifactDocument>()
            ),
            ("validation_result_ref", artifact_text(1024)),
            ("validation_result_fingerprint", artifact_fingerprint()),
            (
                "candidate",
                generator.subschema_for::<crate::artifact::ArtifactDocument>()
            ),
            ("candidate_ref", artifact_text(1024)),
            ("candidate_fingerprint", artifact_fingerprint()),
        ],
        &[
            "operation_id",
            "outcome",
            "operation_context_fingerprint",
            "intake_record_ref",
            "intake_record_fingerprint",
            "basis_artifact_fingerprint",
            "normalized_content",
            "normalized_content_ref",
            "normalized_content_fingerprint",
            "validation_result",
            "validation_result_ref",
            "validation_result_fingerprint",
            "candidate",
            "candidate_ref",
            "candidate_fingerprint"
        ],
    )
);

impl_schema!(
    ArtifactMutationRefusalV1,
    "ArtifactMutationRefusalV1",
    |generator| {
        let mut schema = closed_object(
            vec![
                (
                    "code",
                    string_enum(&[
                        "canonical_syntax_invalid",
                        "structural_validation_failed",
                        "intake_coverage_blocked",
                        "stale_basis",
                        "stale_current_artifact",
                        "operation_ineligible",
                        "publication_basis_conflict",
                    ]),
                ),
                (
                    "layer",
                    string_enum(&[
                        "canonical_syntax",
                        "structural",
                        "intake",
                        "currentness",
                        "eligibility",
                        "publication",
                    ]),
                ),
                ("expected_fingerprint", nullable::<Fingerprint>(generator)),
                ("observed_fingerprint", nullable::<Fingerprint>(generator)),
            ],
            &[
                "code",
                "layer",
                "expected_fingerprint",
                "observed_fingerprint",
            ],
        );
        schema.insert(
            "oneOf".to_owned(),
            serde_json::json!([
                {"properties": {"code": {"const": "canonical_syntax_invalid"}, "layer": {"const": "canonical_syntax"}}},
                {"properties": {"code": {"const": "structural_validation_failed"}, "layer": {"const": "structural"}}},
                {"properties": {"code": {"const": "intake_coverage_blocked"}, "layer": {"const": "intake"}}},
                {"properties": {"code": {"const": "stale_basis"}, "layer": {"const": "currentness"}}},
                {"properties": {"code": {"const": "stale_current_artifact"}, "layer": {"const": "currentness"}}},
                {"properties": {"code": {"const": "operation_ineligible"}, "layer": {"const": "eligibility"}}},
                {"properties": {"code": {"const": "publication_basis_conflict"}, "layer": {"const": "publication"}}}
            ]),
        );
        schema
    }
);

impl_schema!(
    ArtifactAuthoritativeOutputV1,
    "ArtifactAuthoritativeOutputV1",
    |_generator| closed_object(
        vec![
            (
                "authority_class",
                string_enum(&["semantic_record", "canonical_truth"])
            ),
            ("ref", artifact_text(1024)),
            ("fingerprint", artifact_fingerprint()),
        ],
        &["authority_class", "ref", "fingerprint"],
    )
);

impl_schema!(
    ArtifactMutationResultDataV1,
    "ArtifactMutationResultDataV1",
    |generator| closed_object(
        vec![
            (
                "schema_id",
                json_schema!({"const": "handbook.generic-domain-mutation-result"})
            ),
            ("schema_version", json_schema!({"const": "1.0"})),
            (
                "operation_id",
                string_enum(&[
                    "intake.record.append",
                    "artifact.candidate.append",
                    "artifact.candidate.promote"
                ])
            ),
            ("transaction_id", artifact_text(255)),
            ("request_fingerprint", artifact_fingerprint()),
            ("outcome", string_enum(&["committed", "refused"])),
            ("refusal", nullable::<ArtifactMutationRefusalV1>(generator)),
            (
                "authoritative_outputs",
                json_schema!({"type": "array", "maxItems": 2, "items": generator.subschema_for::<ArtifactAuthoritativeOutputV1>()})
            ),
            ("internal_transaction_evidence_ref", artifact_text(1024)),
            (
                "internal_transaction_evidence_fingerprint",
                artifact_fingerprint()
            ),
            ("result_fingerprint", artifact_fingerprint()),
        ],
        &[
            "schema_id",
            "schema_version",
            "operation_id",
            "transaction_id",
            "request_fingerprint",
            "outcome",
            "refusal",
            "authoritative_outputs",
            "internal_transaction_evidence_ref",
            "internal_transaction_evidence_fingerprint",
            "result_fingerprint"
        ],
    )
);

impl_schema!(
    ArtifactProblemCauseV1,
    "ArtifactProblemCauseV1",
    |_generator| string_enum(&[
        "selection_unavailable",
        "profile_unavailable",
        "registry_unavailable",
        "intake_source_unavailable",
        "repository_identity_unavailable",
        "artifact_unavailable",
        "unsupported_platform",
        "selection_admission",
        "intake_admission",
        "canonical_syntax",
        "structural_validation",
        "invalid_request",
        "target_resolution",
        "not_applicable",
        "stale_operation_context",
        "stale_current_artifact",
        "stale_basis",
        "idempotency_conflict",
        "idempotency_expired",
        "unsafe_reference",
        "request_bound_exceeded",
        "unsafe_filesystem",
        "retained_state_bound_exceeded",
        "io",
        "invalid_retained_state"
    ])
);

impl_schema!(
    ArtifactProblemDetailsV1,
    "ArtifactProblemDetailsV1",
    |generator| closed_object(
        vec![
            ("operation_id", artifact_text(255)),
            ("cause", generator.subschema_for::<ArtifactProblemCauseV1>()),
            ("detail", artifact_text(4096)),
            (
                "mutation_result",
                nullable::<ArtifactMutationResultDataV1>(generator)
            ),
        ],
        &["operation_id", "cause", "detail", "mutation_result"],
    )
);

impl_schema!(ArtifactRecheckV1, "ArtifactRecheckV1", |generator| {
    closed_object(
        vec![
            ("operation_id", artifact_text(255)),
            ("cause", generator.subschema_for::<ArtifactProblemCauseV1>()),
        ],
        &["operation_id", "cause"],
    )
});

impl_schema!(
    ArtifactWriteConditionV1,
    "ArtifactWriteConditionV1",
    |_generator| closed_object(
        vec![
            ("operation_id", artifact_text(255)),
            ("transaction_id", artifact_text(255)),
            (
                "authority_class",
                string_enum(&["semantic_record", "canonical_truth"])
            ),
            ("ref", artifact_text(1024)),
            ("fingerprint", artifact_fingerprint()),
        ],
        &[
            "operation_id",
            "transaction_id",
            "authority_class",
            "ref",
            "fingerprint"
        ],
    )
);

impl_schema!(JsonPointer, "JsonPointer", |_generator| json_schema!({
    "type": "string",
    "minLength": 2,
    "maxLength": 255,
    "pattern": "^/([^~/]|~[01])+(/([^~/]|~[01])+)*$"
}));

impl_schema!(
    OperationOwnerBinding,
    "OperationOwnerBinding",
    |generator| closed_object(
        vec![
            (
                "owner_crate",
                generator.subschema_for::<BoundedString<255>>()
            ),
            ("owner_version", generator.subschema_for::<SemVer>()),
            ("source", generator.subschema_for::<ExactBinding>()),
        ],
        &["owner_crate", "owner_version", "source"],
    )
);

impl_schema!(
    OperationOutcomeSchemas,
    "OperationOutcomeSchemas",
    |generator| closed_object(
        vec![
            ("blocker", generator.subschema_for::<ExactBinding>()),
            ("refusal", generator.subschema_for::<ExactBinding>()),
            ("error", generator.subschema_for::<ExactBinding>()),
        ],
        &["blocker", "refusal", "error"],
    )
);

impl_schema!(DeclaredWrite, "DeclaredWrite", |generator| closed_object(
    vec![
        (
            "record_kind",
            generator.subschema_for::<BoundedString<255>>()
        ),
        (
            "authority_class",
            generator.subschema_for::<BoundedString<255>>()
        ),
        (
            "atomic_group",
            generator.subschema_for::<BoundedString<255>>()
        ),
        (
            "condition_schema",
            generator.subschema_for::<ExactBinding>()
        ),
    ],
    &[
        "record_kind",
        "authority_class",
        "atomic_group",
        "condition_schema"
    ],
));

impl_schema!(OperationIdempotency, "OperationIdempotency", |generator| {
    let pointer = generator.subschema_for::<JsonPointer>();
    let basis = generator.subschema_for::<BoundedVec<JsonPointer, 128>>();
    json_schema!({
        "oneOf": [
            closed_object(vec![("kind", json_schema!({"const": "safe"}))], &["kind"]),
            closed_object(
                vec![
                    ("kind", json_schema!({"const": "idempotency_key_required"})),
                    ("key_pointer", pointer.clone()),
                ],
                &["kind", "key_pointer"],
            ),
            closed_object(
                vec![
                    ("kind", json_schema!({"const": "compare_and_write_required"})),
                    ("key_pointer", pointer),
                    ("expected_basis", basis),
                ],
                &["kind", "key_pointer", "expected_basis"],
            ),
        ]
    })
});

impl_schema!(OperationDefinition, "OperationDefinition", |generator| {
    closed_object(
        vec![
            (
                "definition_schema",
                generator.subschema_for::<ExactBinding>(),
            ),
            ("operation", generator.subschema_for::<ExactRef>()),
            ("owner", generator.subschema_for::<OperationOwnerBinding>()),
            ("request_schema", generator.subschema_for::<ExactBinding>()),
            ("result_schema", generator.subschema_for::<ExactBinding>()),
            (
                "outcome_schemas",
                generator.subschema_for::<OperationOutcomeSchemas>(),
            ),
            (
                "required_capabilities",
                generator.subschema_for::<BoundedVec<ExactBinding, 128>>(),
            ),
            ("class", generator.subschema_for::<OperationClass>()),
            (
                "idempotency",
                generator.subschema_for::<OperationIdempotency>(),
            ),
            (
                "write_set",
                generator.subschema_for::<BoundedVec<DeclaredWrite, 128>>(),
            ),
            (
                "declared_transports",
                generator.subschema_for::<BoundedVec<OperationTransport, 3>>(),
            ),
            ("deprecation", nullable::<ExactBinding>(generator)),
            (
                "definition_fingerprint",
                generator.subschema_for::<Fingerprint>(),
            ),
        ],
        &[
            "definition_schema",
            "operation",
            "owner",
            "request_schema",
            "result_schema",
            "outcome_schemas",
            "required_capabilities",
            "class",
            "idempotency",
            "write_set",
            "declared_transports",
            "deprecation",
            "definition_fingerprint",
        ],
    )
});

impl_schema!(OperationRequestV1, "OperationRequestV1", |generator| {
    closed_object(
        vec![
            ("schema", generator.subschema_for::<ExactBinding>()),
            ("request_id", nullable::<RequestId>(generator)),
            ("operation", generator.subschema_for::<ExactBinding>()),
            ("body_schema", generator.subschema_for::<ExactBinding>()),
            ("body", generator.subschema_for::<ContractValue>()),
            (
                "request_fingerprint",
                generator.subschema_for::<Fingerprint>(),
            ),
        ],
        &[
            "schema",
            "request_id",
            "operation",
            "body_schema",
            "body",
            "request_fingerprint",
        ],
    )
});

impl_schema!(OperationResponseV1, "OperationResponseV1", |generator| {
    closed_object(
        vec![
            ("schema", generator.subschema_for::<ExactBinding>()),
            ("request_id", nullable::<RequestId>(generator)),
            (
                "request_fingerprint",
                generator.subschema_for::<Fingerprint>(),
            ),
            ("operation", generator.subschema_for::<ExactBinding>()),
            ("result_schema", generator.subschema_for::<ExactBinding>()),
            ("status", generator.subschema_for::<TerminalStatus>()),
            ("data", nullable::<ContractValue>(generator)),
            (
                "blockers",
                generator.subschema_for::<BoundedVec<Problem, 128>>(),
            ),
            (
                "refusals",
                generator.subschema_for::<BoundedVec<Problem, 128>>(),
            ),
            (
                "errors",
                generator.subschema_for::<BoundedVec<Problem, 128>>(),
            ),
            ("idempotency", generator.subschema_for::<IdempotencyState>()),
            (
                "receipts",
                generator.subschema_for::<BoundedVec<WriteReceipt, 128>>(),
            ),
            (
                "diagnostics",
                generator.subschema_for::<BoundedVec<Diagnostic, 128>>(),
            ),
            (
                "next_actions",
                generator.subschema_for::<BoundedVec<NextAction, 128>>(),
            ),
            (
                "result_fingerprint",
                generator.subschema_for::<Fingerprint>(),
            ),
            (
                "response_fingerprint",
                generator.subschema_for::<Fingerprint>(),
            ),
        ],
        &[
            "schema",
            "request_id",
            "request_fingerprint",
            "operation",
            "result_schema",
            "status",
            "data",
            "blockers",
            "refusals",
            "errors",
            "idempotency",
            "receipts",
            "diagnostics",
            "next_actions",
            "result_fingerprint",
            "response_fingerprint",
        ],
    )
});

impl_schema!(
    OperationAdmissionRefusalV1,
    "OperationAdmissionRefusalV1",
    |generator| closed_object(
        vec![
            ("schema", generator.subschema_for::<ExactBinding>()),
            ("request_id", nullable::<RequestId>(generator)),
            ("attempted_operation", nullable::<ExactRef>(generator)),
            ("attempted_definition", nullable::<ExactBinding>(generator)),
            ("request_fingerprint", nullable::<Fingerprint>(generator)),
            ("status", json_schema!({"const": "refused"})),
            ("data", json_schema!({"type": "null"})),
            ("blockers", json_schema!({"type": "array", "maxItems": 0})),
            (
                "refusals",
                json_schema!({"type": "array", "items": generator.subschema_for::<Problem>(), "minItems": 1, "maxItems": 1})
            ),
            ("errors", json_schema!({"type": "array", "maxItems": 0})),
            (
                "response_fingerprint",
                generator.subschema_for::<Fingerprint>()
            ),
        ],
        &[
            "schema",
            "request_id",
            "attempted_operation",
            "attempted_definition",
            "request_fingerprint",
            "status",
            "data",
            "blockers",
            "refusals",
            "errors",
            "response_fingerprint",
        ],
    )
);

impl_schema!(ContractValue, "ContractValue", |generator| {
    let recursive = generator.subschema_for::<ContractValue>();
    json_schema!({
        "oneOf": [
            {"type": "null"},
            {"type": "boolean"},
            {"type": "string", "minLength": 1, "maxLength": 65_536},
            {"type": "integer", "minimum": MIN_SAFE_INTEGER, "maximum": MAX_SAFE_INTEGER},
            {"type": "array", "items": recursive, "maxItems": 128},
            {
                "type": "object",
                "additionalProperties": recursive,
                "maxProperties": 128,
                "propertyNames": {"type": "string", "minLength": 1, "maxLength": 255}
            }
        ]
    })
});

macro_rules! impl_string_enum {
    ($type:ty, $name:literal, [$($value:literal),+ $(,)?]) => {
        impl_schema!($type, $name, |_generator| string_enum(&[$($value),+]));
    };
}

impl_string_enum!(
    OperationClass,
    "OperationClass",
    ["read_only", "append_only", "compare_and_write"]
);
impl_string_enum!(
    OperationTransport,
    "OperationTransport",
    ["cli_json", "rust_sdk", "tauri"]
);
impl_string_enum!(
    TerminalStatus,
    "TerminalStatus",
    ["ok", "blocked", "refused", "error"]
);
impl_string_enum!(DigestAlgorithm, "DigestAlgorithm", ["sha256"]);
impl_string_enum!(
    Sensitivity,
    "Sensitivity",
    ["public", "internal", "restricted"]
);
impl_string_enum!(
    ProblemCategory,
    "ProblemCategory",
    [
        "prerequisite",
        "schema",
        "compatibility",
        "capability",
        "resolution",
        "authority",
        "precondition",
        "safety",
        "idempotency",
        "implementation",
        "adapter",
    ]
);
impl_string_enum!(
    DiagnosticSeverity,
    "DiagnosticSeverity",
    ["information", "warning"]
);
impl_string_enum!(
    RetryDirective,
    "RetryDirective",
    [
        "after_recheck",
        "after_request_change",
        "after_authority_change",
        "transient",
        "never",
    ]
);
impl_string_enum!(
    NextActionKind,
    "NextActionKind",
    [
        "recheck",
        "change_request",
        "change_authority",
        "retry_transient",
        "contact_operator",
    ]
);
impl_string_enum!(
    OmissionReason,
    "OmissionReason",
    [
        "out_of_resolution",
        "redacted",
        "unavailable",
        "unsupported"
    ]
);
impl_string_enum!(
    ProofEffect,
    "ProofEffect",
    ["none", "partial", "blocked", "refused"]
);
impl_string_enum!(
    IdempotencyStage,
    "IdempotencyStage",
    [
        "capability_validation",
        "request_validation",
        "resolution_validation",
        "authority_validation",
        "precondition_validation",
        "safety_validation",
        "idempotency_validation",
        "execution_start",
    ]
);

impl_schema!(DraftVersion, "DraftVersion", |_generator| json_schema!({
    "type": "string",
    "const": "2020-12"
}));
impl_schema!(
    SchemaMediaType,
    "SchemaMediaType",
    |_generator| json_schema!({
        "type": "string",
        "const": "application/schema+json"
    })
);

struct Token<const N: usize>(PhantomData<()>);

impl<const N: usize> JsonSchema for Token<N> {
    fn schema_name() -> Cow<'static, str> {
        format!("Token_{N}").into()
    }

    fn schema_id() -> Cow<'static, str> {
        format!("handbook_sdk::contract_membrane::schema_generation::Token<{N}>").into()
    }

    fn json_schema(_generator: &mut SchemaGenerator) -> Schema {
        json_schema!({
            "type": "string",
            "minLength": 1,
            "maxLength": N,
            "pattern": TOKEN_PATTERN
        })
    }
}

struct MediaType;

impl_schema!(MediaType, "MediaType", |_generator| json_schema!({
    "type": "string",
    "minLength": 3,
    "maxLength": 255,
    "pattern": "^[a-z0-9][a-z0-9!#$&^_.+\\-]*/[a-z0-9][a-z0-9!#$&^_.+\\-]*$"
}));

struct PortablePath;

impl_schema!(PortablePath, "PortablePath", |_generator| json_schema!({
    "type": "string",
    "minLength": 1,
    "maxLength": 4096,
    "pattern": "^[A-Za-z0-9._-]+(/[A-Za-z0-9._-]+)*$"
}));

impl_schema!(ArtifactLocator, "ArtifactLocator", |generator| {
    let repo_relative = closed_object(
        vec![
            (
                "kind",
                json_schema!({"type": "string", "const": "repo_relative"}),
            ),
            (
                "repository_identity_fingerprint",
                generator.subschema_for::<Fingerprint>(),
            ),
            ("path", generator.subschema_for::<PortablePath>()),
            (
                "no_follow",
                json_schema!({"type": "boolean", "const": true}),
            ),
        ],
        &[
            "kind",
            "repository_identity_fingerprint",
            "path",
            "no_follow",
        ],
    );
    let content_addressed = closed_object(
        vec![
            (
                "kind",
                json_schema!({"type": "string", "const": "content_addressed"}),
            ),
            ("store", generator.subschema_for::<ExactBinding>()),
            ("algorithm", generator.subschema_for::<DigestAlgorithm>()),
            ("digest", generator.subschema_for::<Fingerprint>()),
        ],
        &["kind", "store", "algorithm", "digest"],
    );
    json_schema!({"oneOf": [repo_relative, content_addressed]})
});

impl_schema!(ArtifactRef, "ArtifactRef", |generator| closed_object(
    vec![
        ("artifact", generator.subschema_for::<ExactBinding>()),
        ("media_type", generator.subschema_for::<MediaType>()),
        ("byte_length", generator.subschema_for::<SafeU64>()),
        ("sensitivity", generator.subschema_for::<Sensitivity>()),
        ("locator", generator.subschema_for::<ArtifactLocator>()),
    ],
    &[
        "artifact",
        "media_type",
        "byte_length",
        "sensitivity",
        "locator",
    ],
));

impl_schema!(SourceBinding, "SourceBinding", |generator| closed_object(
    vec![
        ("source", generator.subschema_for::<ExactBinding>()),
        (
            "captured_revision",
            generator.subschema_for::<ExactBinding>(),
        ),
        ("adapter", generator.subschema_for::<ExactBinding>()),
    ],
    &["source", "captured_revision", "adapter"],
));

impl_schema!(Omission, "Omission", |generator| closed_object(
    vec![
        ("item", generator.subschema_for::<ExactBinding>()),
        ("reason", generator.subschema_for::<OmissionReason>()),
        ("proof_effect", generator.subschema_for::<ProofEffect>()),
        ("source", nullable::<SourceBinding>(generator)),
        ("ordinal", generator.subschema_for::<SafeU64>()),
    ],
    &["item", "reason", "proof_effect", "source", "ordinal"],
));

impl_schema!(SchemaManifestEntry, "SchemaManifestEntry", |generator| {
    closed_object(
        vec![
            ("schema", generator.subschema_for::<ExactBinding>()),
            ("draft", generator.subschema_for::<DraftVersion>()),
            ("byte_length", generator.subschema_for::<SafeU64>()),
            ("media_type", generator.subschema_for::<SchemaMediaType>()),
        ],
        &["schema", "draft", "byte_length", "media_type"],
    )
});

impl_schema!(
    CorrelationRef,
    "CorrelationRef",
    |_generator| json_schema!({
        "type": "string",
        "minLength": 21,
        "maxLength": 133,
        "pattern": CORRELATION_PATTERN
    })
);

impl_schema!(ProblemBinding, "ProblemBinding", |generator| closed_object(
    vec![
        ("problem_id", generator.subschema_for::<Token<255>>()),
        (
            "problem_fingerprint",
            generator.subschema_for::<Fingerprint>(),
        ),
    ],
    &["problem_id", "problem_fingerprint"],
));

impl_schema!(RecheckCondition, "RecheckCondition", |generator| {
    closed_object(
        vec![
            (
                "condition_schema",
                generator.subschema_for::<ExactBinding>(),
            ),
            ("condition", generator.subschema_for::<ContractValue>()),
            (
                "condition_fingerprint",
                generator.subschema_for::<Fingerprint>(),
            ),
        ],
        &["condition_schema", "condition", "condition_fingerprint"],
    )
});

impl_schema!(Diagnostic, "Diagnostic", |generator| closed_object(
    vec![
        ("code", generator.subschema_for::<Token<255>>()),
        ("severity", generator.subschema_for::<DiagnosticSeverity>(),),
        ("subject", generator.subschema_for::<ExactBinding>()),
        ("details_schema", generator.subschema_for::<ExactBinding>(),),
        ("details", generator.subschema_for::<ContractValue>()),
        (
            "details_fingerprint",
            generator.subschema_for::<Fingerprint>(),
        ),
        (
            "display_message",
            generator.subschema_for::<BoundedString<1024>>(),
        ),
    ],
    &[
        "code",
        "severity",
        "subject",
        "details_schema",
        "details",
        "details_fingerprint",
    ],
));

impl_schema!(NextAction, "NextAction", |generator| closed_object(
    vec![
        ("action_id", generator.subschema_for::<Token<255>>()),
        ("kind", generator.subschema_for::<NextActionKind>()),
        ("subject", generator.subschema_for::<ExactBinding>()),
        (
            "parameter_schema",
            generator.subschema_for::<ExactBinding>(),
        ),
        ("parameters", generator.subschema_for::<ContractValue>()),
        (
            "display_label",
            generator.subschema_for::<BoundedString<1024>>(),
        ),
    ],
    &[
        "action_id",
        "kind",
        "subject",
        "parameter_schema",
        "parameters",
    ],
));

impl_schema!(WriteReceipt, "WriteReceipt", |generator| closed_object(
    vec![
        ("record_kind", generator.subschema_for::<Token<255>>()),
        ("record", generator.subschema_for::<ExactBinding>()),
        ("authority_class", generator.subschema_for::<Token<255>>(),),
        ("condition", generator.subschema_for::<ContractValue>()),
        ("atomic_group", generator.subschema_for::<Token<255>>()),
        (
            "receipt_fingerprint",
            generator.subschema_for::<Fingerprint>(),
        ),
    ],
    &[
        "record_kind",
        "record",
        "authority_class",
        "condition",
        "atomic_group",
        "receipt_fingerprint",
    ],
));

impl_schema!(CatalogRoot, "CatalogRoot", |generator| closed_object(
    vec![
        ("catalog", generator.subschema_for::<ExactBinding>()),
        ("entry_schema", generator.subschema_for::<ExactBinding>(),),
        ("total_entries", generator.subschema_for::<SafeU64>()),
        ("canonical_sort", generator.subschema_for::<ExactBinding>(),),
        (
            "retention_until_utc",
            generator.subschema_for::<UtcTimestamp>(),
        ),
    ],
    &[
        "catalog",
        "entry_schema",
        "total_entries",
        "canonical_sort",
        "retention_until_utc",
    ],
));

impl_schema!(CatalogCursor, "CatalogCursor", |generator| closed_object(
    vec![
        ("catalog", generator.subschema_for::<ExactBinding>()),
        ("next_sort_key", generator.subschema_for::<ContractValue>(),),
        (
            "cursor_fingerprint",
            generator.subschema_for::<Fingerprint>(),
        ),
    ],
    &["catalog", "next_sort_key", "cursor_fingerprint"],
));

impl_schema!(CatalogPage, "CatalogPage", |generator| closed_object(
    vec![
        ("root", generator.subschema_for::<CatalogRoot>()),
        (
            "entries",
            generator.subschema_for::<BoundedVec<ContractValue, 128>>(),
        ),
        ("next_cursor", nullable::<CatalogCursor>(generator)),
        ("page_fingerprint", generator.subschema_for::<Fingerprint>(),),
    ],
    &["root", "entries", "next_cursor", "page_fingerprint"],
));

impl_schema!(
    CatalogPageSize,
    "CatalogPageSize",
    |_generator| json_schema!({
        "type": "integer", "minimum": 1, "maximum": 128
    })
);

impl_string_enum!(
    BootstrapRefusalReason,
    "BootstrapRefusalReason",
    [
        "stale_descriptor",
        "tampered_descriptor",
        "unsupported_api_version",
        "unsupported_api_major",
        "cross_major_descriptor",
    ]
);

impl_schema!(BootstrapCeilings, "BootstrapCeilings", |_generator| {
    closed_object(
        vec![
            (
                "bootstrap_request_max_bytes",
                json_schema!({"const": 1_048_576}),
            ),
            (
                "bootstrap_response_max_bytes",
                json_schema!({"const": 1_048_576}),
            ),
            (
                "ordinary_request_max_bytes",
                json_schema!({"const": 1_048_576}),
            ),
            (
                "ordinary_response_max_bytes",
                json_schema!({"const": 1_048_576}),
            ),
            ("catalog_page_size_max", json_schema!({"const": 128})),
            ("catalog_entry_max_bytes", json_schema!({"const": 65_536})),
            ("catalog_total_entries_max", json_schema!({"const": 128})),
            ("owner_versions_max_items", json_schema!({"const": 128})),
            ("idempotency_key_max_bytes", json_schema!({"const": 65_536})),
            (
                "bridge_recovery_record_max_bytes",
                json_schema!({"const": 1_048_576}),
            ),
        ],
        &[
            "bootstrap_request_max_bytes",
            "bootstrap_response_max_bytes",
            "ordinary_request_max_bytes",
            "ordinary_response_max_bytes",
            "catalog_page_size_max",
            "catalog_entry_max_bytes",
            "catalog_total_entries_max",
            "owner_versions_max_items",
            "idempotency_key_max_bytes",
            "bridge_recovery_record_max_bytes",
        ],
    )
});

impl_schema!(
    BootstrapSchemaBindings,
    "BootstrapSchemaBindings",
    |generator| {
        let mut binding = || generator.subschema_for::<ExactBinding>();
        closed_object(
            vec![
                ("descriptor_schema", binding()),
                ("bootstrap_request", binding()),
                ("bootstrap_response", binding()),
                ("bootstrap_refusal", binding()),
                ("bootstrap_refusal_details", binding()),
                ("capability_entry", binding()),
                ("owner_version_entry", binding()),
                ("profile_catalog_entry", binding()),
                ("catalog_roots", binding()),
                ("catalog_root", binding()),
                ("typed_catalog_cursor", binding()),
                ("capability_catalog_page", binding()),
                ("schema_catalog_page", binding()),
                ("profile_catalog_page", binding()),
                ("catalog_read_blocked", binding()),
                ("operation_definition", binding()),
                ("operation_request", binding()),
                ("operation_response", binding()),
                ("operation_admission_refusal", binding()),
            ],
            &[
                "descriptor_schema",
                "bootstrap_request",
                "bootstrap_response",
                "bootstrap_refusal",
                "bootstrap_refusal_details",
                "capability_entry",
                "owner_version_entry",
                "profile_catalog_entry",
                "catalog_roots",
                "catalog_root",
                "typed_catalog_cursor",
                "capability_catalog_page",
                "schema_catalog_page",
                "profile_catalog_page",
                "catalog_read_blocked",
                "operation_definition",
                "operation_request",
                "operation_response",
                "operation_admission_refusal",
            ],
        )
    }
);

impl_schema!(
    BootstrapDescriptorV1,
    "BootstrapDescriptorV1",
    |generator| closed_object(
        vec![
            ("schema", generator.subschema_for::<ExactBinding>()),
            ("descriptor", generator.subschema_for::<ExactRef>()),
            ("api_version", generator.subschema_for::<SemVer>()),
            ("api_major", json_schema!({"const": 1})),
            (
                "supported_api_versions",
                generator.subschema_for::<BoundedVec<SemVer, 16>>()
            ),
            (
                "schemas",
                generator.subschema_for::<BootstrapSchemaBindings>()
            ),
            ("ceilings", generator.subschema_for::<BootstrapCeilings>()),
            (
                "declared_transports",
                generator.subschema_for::<BoundedVec<OperationTransport, 3>>()
            ),
            (
                "descriptor_content_fingerprint",
                generator.subschema_for::<Fingerprint>()
            ),
        ],
        &[
            "schema",
            "descriptor",
            "api_version",
            "api_major",
            "supported_api_versions",
            "schemas",
            "ceilings",
            "declared_transports",
            "descriptor_content_fingerprint"
        ],
    )
);

impl_schema!(BootstrapRequestV1, "BootstrapRequestV1", |generator| {
    closed_object(
        vec![
            ("schema", generator.subschema_for::<ExactBinding>()),
            ("request_id", nullable::<RequestId>(generator)),
            (
                "bootstrap_descriptor",
                generator.subschema_for::<ExactBinding>(),
            ),
            ("api_version", generator.subschema_for::<SemVer>()),
            ("page_size", generator.subschema_for::<CatalogPageSize>()),
            (
                "request_fingerprint",
                generator.subschema_for::<Fingerprint>(),
            ),
        ],
        &[
            "schema",
            "request_id",
            "bootstrap_descriptor",
            "api_version",
            "page_size",
            "request_fingerprint",
        ],
    )
});

impl_schema!(OwnerVersionEntry, "OwnerVersionEntry", |generator| {
    closed_object(
        vec![
            (
                "owner_crate",
                generator.subschema_for::<BoundedString<255>>(),
            ),
            ("owner_version", generator.subschema_for::<SemVer>()),
            ("source", generator.subschema_for::<ExactBinding>()),
            (
                "entry_fingerprint",
                generator.subschema_for::<Fingerprint>(),
            ),
        ],
        &[
            "owner_crate",
            "owner_version",
            "source",
            "entry_fingerprint",
        ],
    )
});

impl_schema!(ProfileCatalogEntry, "ProfileCatalogEntry", |generator| {
    closed_object(
        vec![
            ("profile", generator.subschema_for::<ExactBinding>()),
            ("source", generator.subschema_for::<ExactBinding>()),
            (
                "entry_fingerprint",
                generator.subschema_for::<Fingerprint>(),
            ),
        ],
        &["profile", "source", "entry_fingerprint"],
    )
});

impl_schema!(CapabilityEntry, "CapabilityEntry", |generator| {
    closed_object(
        vec![
            ("operation", generator.subschema_for::<ExactBinding>()),
            ("owner", generator.subschema_for::<OperationOwnerBinding>()),
            ("request_schema", generator.subschema_for::<ExactBinding>()),
            ("result_schema", generator.subschema_for::<ExactBinding>()),
            (
                "outcome_schemas",
                generator.subschema_for::<OperationOutcomeSchemas>(),
            ),
            (
                "required_capabilities",
                generator.subschema_for::<BoundedVec<ExactBinding, 128>>(),
            ),
            ("class", generator.subschema_for::<OperationClass>()),
            (
                "idempotency",
                generator.subschema_for::<OperationIdempotency>(),
            ),
            (
                "declared_transports",
                generator.subschema_for::<BoundedVec<OperationTransport, 3>>(),
            ),
            (
                "current_transports",
                generator.subschema_for::<BoundedVec<OperationTransport, 3>>(),
            ),
            ("deprecation", nullable::<ExactBinding>(generator)),
            (
                "entry_fingerprint",
                generator.subschema_for::<Fingerprint>(),
            ),
        ],
        &[
            "operation",
            "owner",
            "request_schema",
            "result_schema",
            "outcome_schemas",
            "required_capabilities",
            "class",
            "idempotency",
            "declared_transports",
            "current_transports",
            "deprecation",
            "entry_fingerprint",
        ],
    )
});

impl_schema!(CatalogRoots, "CatalogRoots", |generator| closed_object(
    vec![
        ("schema", generator.subschema_for::<ExactBinding>()),
        ("operations", generator.subschema_for::<CatalogRoot>()),
        ("schemas", generator.subschema_for::<CatalogRoot>()),
        ("profiles", generator.subschema_for::<CatalogRoot>()),
        (
            "roots_fingerprint",
            generator.subschema_for::<Fingerprint>()
        ),
    ],
    &[
        "schema",
        "operations",
        "schemas",
        "profiles",
        "roots_fingerprint"
    ],
));

impl_schema!(TypedCatalogCursor, "TypedCatalogCursor", |generator| {
    closed_object(
        vec![
            ("schema", generator.subschema_for::<ExactBinding>()),
            ("api_major", json_schema!({"const": 1})),
            ("catalog", generator.subschema_for::<ExactBinding>()),
            ("page_size", generator.subschema_for::<CatalogPageSize>()),
            ("next_index", generator.subschema_for::<SafeU64>()),
            ("next_sort_key", generator.subschema_for::<ExactRef>()),
            ("expires_at_utc", generator.subschema_for::<UtcTimestamp>()),
            (
                "cursor_fingerprint",
                generator.subschema_for::<Fingerprint>(),
            ),
        ],
        &[
            "schema",
            "api_major",
            "catalog",
            "page_size",
            "next_index",
            "next_sort_key",
            "expires_at_utc",
            "cursor_fingerprint",
        ],
    )
});

impl<T> JsonSchema for TypedCatalogPage<T>
where
    T: JsonSchema + TypedCatalogEntry,
{
    fn schema_name() -> Cow<'static, str> {
        format!("TypedCatalogPage_{}", T::schema_name()).into()
    }

    fn schema_id() -> Cow<'static, str> {
        format!(
            "handbook_sdk::contract_membrane::TypedCatalogPage<{}>",
            T::schema_id()
        )
        .into()
    }

    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        closed_object(
            vec![
                ("schema", generator.subschema_for::<ExactBinding>()),
                ("api_major", json_schema!({"const": 1})),
                ("root", generator.subschema_for::<CatalogRoot>()),
                ("page_size", generator.subschema_for::<CatalogPageSize>()),
                ("start_index", generator.subschema_for::<SafeU64>()),
                ("entries", generator.subschema_for::<BoundedVec<T, 128>>()),
                ("next_cursor", nullable::<TypedCatalogCursor>(generator)),
                ("page_fingerprint", generator.subschema_for::<Fingerprint>()),
            ],
            &[
                "schema",
                "api_major",
                "root",
                "page_size",
                "start_index",
                "entries",
                "next_cursor",
                "page_fingerprint",
            ],
        )
    }
}

impl_schema!(BootstrapResponseV1, "BootstrapResponseV1", |generator| {
    closed_object(
        vec![
            ("schema", generator.subschema_for::<ExactBinding>()),
            ("request_id", nullable::<RequestId>(generator)),
            (
                "request_fingerprint",
                generator.subschema_for::<Fingerprint>(),
            ),
            ("status", json_schema!({"const": "ok"})),
            (
                "descriptor",
                generator.subschema_for::<BootstrapDescriptorV1>(),
            ),
            ("catalog_roots", generator.subschema_for::<CatalogRoots>()),
            (
                "first_capability_page",
                generator.subschema_for::<CapabilityCatalogPage>(),
            ),
            (
                "owner_versions",
                generator.subschema_for::<BoundedVec<OwnerVersionEntry, 128>>(),
            ),
            (
                "schema_manifest",
                generator.subschema_for::<BoundedVec<SchemaManifestEntry, 256>>(),
            ),
            (
                "response_fingerprint",
                generator.subschema_for::<Fingerprint>(),
            ),
        ],
        &[
            "schema",
            "request_id",
            "request_fingerprint",
            "status",
            "descriptor",
            "catalog_roots",
            "first_capability_page",
            "owner_versions",
            "schema_manifest",
            "response_fingerprint",
        ],
    )
});

impl_schema!(
    BootstrapRefusalDetails,
    "BootstrapRefusalDetails",
    |generator| closed_object(
        vec![
            (
                "reason",
                generator.subschema_for::<BootstrapRefusalReason>()
            ),
            (
                "supported_api_versions",
                generator.subschema_for::<BoundedVec<SemVer, 16>>()
            ),
        ],
        &["reason", "supported_api_versions"],
    )
);

impl_schema!(BootstrapRefusalV1, "BootstrapRefusalV1", |generator| {
    closed_object(
        vec![
            ("schema", generator.subschema_for::<ExactBinding>()),
            ("request_id", nullable::<RequestId>(generator)),
            (
                "request_fingerprint",
                generator.subschema_for::<Fingerprint>(),
            ),
            ("bootstrap_descriptor", nullable::<ExactBinding>(generator)),
            ("status", json_schema!({"const": "refused"})),
            ("data", json_schema!({"type": "null"})),
            ("blockers", json_schema!({"type": "array", "maxItems": 0})),
            (
                "refusals",
                json_schema!({"type": "array", "items": generator.subschema_for::<Problem>(), "minItems": 1, "maxItems": 1}),
            ),
            ("errors", json_schema!({"type": "array", "maxItems": 0})),
            (
                "schema_manifest",
                generator.subschema_for::<BoundedVec<SchemaManifestEntry, 256>>(),
            ),
            (
                "response_fingerprint",
                generator.subschema_for::<Fingerprint>(),
            ),
        ],
        &[
            "schema",
            "request_id",
            "request_fingerprint",
            "bootstrap_descriptor",
            "status",
            "data",
            "blockers",
            "refusals",
            "errors",
            "schema_manifest",
            "response_fingerprint",
        ],
    )
});

impl_schema!(CatalogReadBlockedV1, "CatalogReadBlockedV1", |generator| {
    closed_object(
        vec![
            ("schema", generator.subschema_for::<ExactBinding>()),
            ("api_major", json_schema!({"const": 1})),
            ("catalog", generator.subschema_for::<ExactBinding>()),
            ("status", json_schema!({"const": "blocked"})),
            ("data", json_schema!({"type": "null"})),
            (
                "blockers",
                json_schema!({"type": "array", "items": generator.subschema_for::<Problem>(), "minItems": 1, "maxItems": 1}),
            ),
            ("refusals", json_schema!({"type": "array", "maxItems": 0})),
            ("errors", json_schema!({"type": "array", "maxItems": 0})),
            ("restart_from_first_page", json_schema!({"const": true})),
            (
                "response_fingerprint",
                generator.subschema_for::<Fingerprint>(),
            ),
        ],
        &[
            "schema",
            "api_major",
            "catalog",
            "status",
            "data",
            "blockers",
            "refusals",
            "errors",
            "restart_from_first_page",
            "response_fingerprint",
        ],
    )
});

impl_schema!(IdempotencyState, "IdempotencyState", |generator| {
    let not_applicable = closed_object(
        vec![(
            "state",
            json_schema!({"type": "string", "const": "not_applicable"}),
        )],
        &["state"],
    );
    let not_established = closed_object(
        vec![
            (
                "state",
                json_schema!({"type": "string", "const": "not_established"}),
            ),
            ("stage", generator.subschema_for::<IdempotencyStage>()),
            (
                "terminal_problem",
                generator.subschema_for::<ProblemBinding>(),
            ),
        ],
        &["state", "stage", "terminal_problem"],
    );
    let established = closed_object(
        vec![
            (
                "state",
                json_schema!({"type": "string", "const": "established"}),
            ),
            (
                "idempotency_key_fingerprint",
                generator.subschema_for::<Fingerprint>(),
            ),
            (
                "request_fingerprint",
                generator.subschema_for::<Fingerprint>(),
            ),
            ("replayed", json_schema!({"type": "boolean"})),
            (
                "original_result_fingerprint",
                nullable::<Fingerprint>(generator),
            ),
            (
                "result_retention_until_utc",
                generator.subschema_for::<UtcTimestamp>(),
            ),
        ],
        &[
            "state",
            "idempotency_key_fingerprint",
            "request_fingerprint",
            "replayed",
            "original_result_fingerprint",
            "result_retention_until_utc",
        ],
    );
    json_schema!({"oneOf": [not_applicable, not_established, established]})
});

impl_schema!(Problem, "Problem", |generator| closed_object(
    vec![
        ("problem_id", generator.subschema_for::<Token<255>>()),
        ("code", generator.subschema_for::<Token<255>>()),
        ("category", generator.subschema_for::<ProblemCategory>(),),
        ("subject", generator.subschema_for::<ExactBinding>()),
        ("rule", nullable::<ExactBinding>(generator)),
        ("details_schema", generator.subschema_for::<ExactBinding>(),),
        ("details", generator.subschema_for::<ContractValue>()),
        (
            "details_fingerprint",
            generator.subschema_for::<Fingerprint>(),
        ),
        (
            "evidence",
            generator.subschema_for::<BoundedVec<ArtifactRef, 128>>(),
        ),
        ("retry", generator.subschema_for::<RetryDirective>()),
        ("recheck", nullable::<RecheckCondition>(generator)),
        ("correlation_ref", nullable::<CorrelationRef>(generator),),
        (
            "problem_fingerprint",
            generator.subschema_for::<Fingerprint>(),
        ),
    ],
    &[
        "problem_id",
        "code",
        "category",
        "subject",
        "rule",
        "details_schema",
        "details",
        "details_fingerprint",
        "evidence",
        "retry",
        "recheck",
        "correlation_ref",
        "problem_fingerprint",
    ],
));

impl_schema!(
    PostureReferenceDtoV1,
    "PostureReferenceDtoV1",
    |generator| {
        closed_object(
            vec![
                ("reference", generator.subschema_for::<BoundedString<255>>()),
                ("fingerprint", generator.subschema_for::<Fingerprint>()),
            ],
            &["reference", "fingerprint"],
        )
    }
);

impl_schema!(
    PostureCanonicalBindingDtoV1,
    "PostureCanonicalBindingDtoV1",
    |generator| {
        closed_object(
            vec![
                (
                    "reference",
                    json_schema!({"const": ".handbook/project/charter.yaml"}),
                ),
                ("fingerprint", generator.subschema_for::<Fingerprint>()),
                ("document_sha256", generator.subschema_for::<Fingerprint>()),
                (
                    "byte_length",
                    json_schema!({"type": "integer", "minimum": 1, "maximum": MAX_SAFE_INTEGER}),
                ),
            ],
            &["reference", "fingerprint", "document_sha256", "byte_length"],
        )
    }
);

impl_schema!(
    PostureDimensionChangeDtoV1,
    "PostureDimensionChangeDtoV1",
    |generator| {
        let level = json_schema!({"type": "integer", "minimum": 0, "maximum": 255});
        closed_object(
            vec![
                (
                    "dimension_id",
                    generator.subschema_for::<BoundedString<255>>(),
                ),
                ("dimension_index", level.clone()),
                ("authority_path", generator.subschema_for::<JsonPointer>()),
                (
                    "expected_stored_value",
                    json_schema!({"anyOf": [level.clone(), {"type": "null"}]}),
                ),
                ("expected_effective_level", level.clone()),
                ("proposed_effective_level", level),
            ],
            &[
                "dimension_id",
                "dimension_index",
                "authority_path",
                "expected_stored_value",
                "expected_effective_level",
                "proposed_effective_level",
            ],
        )
    }
);

struct PostureTransitionApplyRequestSchemaV1;

impl_schema!(
    PostureTransitionApplyRequestSchemaV1,
    "PostureTransitionApplyRequestSchemaV1",
    |generator| {
        closed_object(
            vec![
                (
                    "change",
                    generator.subschema_for::<PostureDimensionChangeDtoV1>(),
                ),
                (
                    "effective_at_utc",
                    generator.subschema_for::<UtcTimestamp>(),
                ),
                (
                    "expected_canonical",
                    generator.subschema_for::<PostureCanonicalBindingDtoV1>(),
                ),
                (
                    "idempotency_key",
                    json_schema!({"type": "string", "minLength": 1, "maxLength": 256, "pattern": "^[!-~]{1,256}$"}),
                ),
                (
                    "reassessment_coverage_ids",
                    json_schema!({
                        "type": "array",
                        "items": {"type": "string", "minLength": 1, "maxLength": 256, "pattern": "^[!-~]{1,256}$"},
                        "minItems": 1,
                        "maxItems": 64,
                        "uniqueItems": true
                    }),
                ),
                (
                    "recommendation",
                    generator.subschema_for::<PostureReferenceDtoV1>(),
                ),
                (
                    "repository_identity_fingerprint",
                    generator.subschema_for::<Fingerprint>(),
                ),
            ],
            &[
                "change",
                "effective_at_utc",
                "expected_canonical",
                "idempotency_key",
                "reassessment_coverage_ids",
                "recommendation",
                "repository_identity_fingerprint",
            ],
        )
    }
);

impl_schema!(
    PostureTransitionApplyResultDataV1,
    "PostureTransitionApplyResultDataV1",
    |generator| {
        closed_object(
            vec![
                (
                    "idempotency_key_fingerprint",
                    generator.subschema_for::<Fingerprint>(),
                ),
                (
                    "repository_identity_fingerprint",
                    generator.subschema_for::<Fingerprint>(),
                ),
                (
                    "prior_canonical",
                    generator.subschema_for::<PostureCanonicalBindingDtoV1>(),
                ),
                (
                    "resulting_canonical",
                    generator.subschema_for::<PostureCanonicalBindingDtoV1>(),
                ),
                (
                    "posture_transition",
                    generator.subschema_for::<PostureReferenceDtoV1>(),
                ),
                (
                    "lifecycle_transition",
                    generator.subschema_for::<PostureReferenceDtoV1>(),
                ),
                (
                    "resulting_kernel",
                    generator.subschema_for::<PostureReferenceDtoV1>(),
                ),
            ],
            &[
                "idempotency_key_fingerprint",
                "repository_identity_fingerprint",
                "prior_canonical",
                "resulting_canonical",
                "posture_transition",
                "lifecycle_transition",
                "resulting_kernel",
            ],
        )
    }
);

impl_schema!(
    PostureCanonicalWriteConditionV1,
    "PostureCanonicalWriteConditionV1",
    |generator| closed_object(
        vec![
            (
                "prior_canonical",
                generator.subschema_for::<PostureCanonicalBindingDtoV1>(),
            ),
            (
                "resulting_canonical",
                generator.subschema_for::<PostureCanonicalBindingDtoV1>(),
            ),
        ],
        &["prior_canonical", "resulting_canonical"],
    )
);

impl_schema!(
    PostureSemanticWriteConditionV1,
    "PostureSemanticWriteConditionV1",
    |generator| closed_object(
        vec![
            (
                "lifecycle_transition",
                generator.subschema_for::<PostureReferenceDtoV1>(),
            ),
            (
                "posture_transition",
                generator.subschema_for::<PostureReferenceDtoV1>(),
            ),
            (
                "resulting_kernel",
                generator.subschema_for::<PostureReferenceDtoV1>(),
            ),
        ],
        &[
            "lifecycle_transition",
            "posture_transition",
            "resulting_kernel",
        ],
    )
);

macro_rules! posture_variant_schema {
    ($type:ident, $schema_name:literal, $variant:literal) => {
        struct $type;
        impl_schema!($type, $schema_name, |_generator| closed_object(
            vec![("owner_variant", json_schema!({"const": $variant}))],
            &["owner_variant"],
        ));
    };
}

posture_variant_schema!(
    PostureAuthorityConflictDetailsSchema,
    "PostureAuthorityConflictDetailsSchema",
    "AuthorityConflict"
);
posture_variant_schema!(
    PostureIntegrityOrDurabilityDetailsSchema,
    "PostureIntegrityOrDurabilityDetailsSchema",
    "IntegrityOrDurability"
);
posture_variant_schema!(
    PostureInvalidFixedDimensionDetailsSchema,
    "PostureInvalidFixedDimensionDetailsSchema",
    "InvalidFixedDimension"
);
posture_variant_schema!(
    PostureInvalidIdempotencyKeyDetailsSchema,
    "PostureInvalidIdempotencyKeyDetailsSchema",
    "InvalidIdempotencyKey"
);
posture_variant_schema!(
    PostureInvalidSemanticInputDetailsSchema,
    "PostureInvalidSemanticInputDetailsSchema",
    "InvalidSemanticInput"
);
posture_variant_schema!(PostureIoDetailsSchema, "PostureIoDetailsSchema", "Io");
posture_variant_schema!(
    PostureLifecycleNotCurrentDetailsSchema,
    "PostureLifecycleNotCurrentDetailsSchema",
    "LifecycleNotCurrent"
);
posture_variant_schema!(
    PostureMissingLifecycleAuthorityDetailsSchema,
    "PostureMissingLifecycleAuthorityDetailsSchema",
    "MissingLifecycleAuthority"
);
posture_variant_schema!(
    PostureRepositoryIdentityMismatchDetailsSchema,
    "PostureRepositoryIdentityMismatchDetailsSchema",
    "RepositoryIdentityMismatch"
);
posture_variant_schema!(
    PostureStaleCanonicalDetailsSchema,
    "PostureStaleCanonicalDetailsSchema",
    "StaleCanonical"
);
posture_variant_schema!(
    PostureUnsafeFilesystemDetailsSchema,
    "PostureUnsafeFilesystemDetailsSchema",
    "UnsafeFilesystem"
);
posture_variant_schema!(
    PostureUnsupportedPlatformDetailsSchema,
    "PostureUnsupportedPlatformDetailsSchema",
    "UnsupportedPlatform"
);
posture_variant_schema!(
    PostureUnsupportedPlatformRecheckSchema,
    "PostureUnsupportedPlatformRecheckSchema",
    "UnsupportedPlatform"
);

macro_rules! invocation_string_enum {
    ($type:ty, $name:literal, [$($value:literal),+ $(,)?]) => {
        impl_schema!($type, $name, |_generator| string_enum(&[$($value),+]));
    };
}

invocation_string_enum!(
    RepositoryReadinessStatusV1,
    "RepositoryReadinessStatusV1",
    ["ready", "action_required", "indeterminate", "invalid"]
);
invocation_string_enum!(
    ProfileConditionOutcomeV1,
    "ProfileConditionOutcomeV1",
    ["true", "false", "unknown", "unresolved", "stale", "refused"]
);
invocation_string_enum!(
    ProfileConditionReasonV1,
    "ProfileConditionReasonV1",
    ["evidence_contract_unavailable"]
);
invocation_string_enum!(
    ProfileRequirednessModeV1,
    "ProfileRequirednessModeV1",
    ["always", "conditional", "optional"]
);
invocation_string_enum!(
    ArtifactApplicabilityV1,
    "ArtifactApplicabilityV1",
    ["required", "optional", "indeterminate"]
);
invocation_string_enum!(
    ArtifactInspectionStatusV1,
    "ArtifactInspectionStatusV1",
    [
        "missing",
        "structurally_valid",
        "structurally_invalid",
        "unsafe_path",
        "unreadable",
        "not_inspected"
    ]
);
invocation_string_enum!(
    ArtifactInspectionReasonV1,
    "ArtifactInspectionReasonV1",
    [
        "present_and_structurally_valid",
        "required_path_missing",
        "optional_path_missing",
        "conditional_evidence_unavailable_path_missing",
        "conditional_evidence_unavailable_path_present",
        "yaml_syntax_invalid",
        "duplicate_yaml_key",
        "document_not_object",
        "structural_validation_failed",
        "document_limit_exceeded",
        "aggregate_read_limit_exceeded",
        "symlink_refused",
        "non_regular_file_refused",
        "unsafe_repository_path",
        "unsupported_platform_strict_read",
        "repository_read_failed",
        "typed_decode_failed",
        "rendered_view_refused",
        "observation_changed_during_inspection"
    ]
);
invocation_string_enum!(
    DoctorCharterDefinitionClosureStatusV1,
    "DoctorCharterDefinitionClosureStatusV1",
    ["resolved", "unavailable"]
);
invocation_string_enum!(
    DoctorCharterLifecycleStateV1,
    "DoctorCharterLifecycleStateV1",
    ["unobserved"]
);
invocation_string_enum!(
    DoctorCharterNextActionV1,
    "DoctorCharterNextActionV1",
    [
        "run_charter_author",
        "repair_canonical_charter",
        "repair_definition_closure",
        "observe_lifecycle"
    ]
);
invocation_string_enum!(
    RepositorySetupModeV1,
    "RepositorySetupModeV1",
    ["auto", "init", "refresh"]
);
invocation_string_enum!(
    RepositorySetupRootActionV1,
    "RepositorySetupRootActionV1",
    ["preserve", "create"]
);
invocation_string_enum!(
    RepositorySetupArtifactActionKindV1,
    "RepositorySetupArtifactActionKindV1",
    [
        "preserve",
        "author_required",
        "optional_absent",
        "condition_indeterminate",
        "invalid"
    ]
);
invocation_string_enum!(
    RepositorySetupErrorKindV1,
    "RepositorySetupErrorKindV1",
    [
        "profile_resolution",
        "profile_decision",
        "already_initialized",
        "missing_canonical_root",
        "invalid_canonical_root",
        "invalid_request",
        "materializer_unavailable",
        "runtime_state_plan",
        "runtime_state_apply",
        "repository_identity"
    ]
);
invocation_string_enum!(
    RepositorySetupErrorReasonV1,
    "RepositorySetupErrorReasonV1",
    [
        "shipped_profile_unavailable",
        "selected_profile_decision_invalid",
        "unresolved_mode",
        "init_rejects_refresh_flags",
        "root_already_initialized",
        "refresh_root_missing",
        "root_not_directory",
        "root_symlink_refused",
        "canonical_root_inspect_failed",
        "canonical_root_create_failed",
        "rewrite_has_no_materializer",
        "runtime_state_target_unsafe",
        "runtime_state_reset_plan_too_large",
        "runtime_state_reset_projection_too_large",
        "runtime_state_mutation_failed",
        "repository_authority_recovery_blocked",
        "repository_identity_unsafe",
        "repository_identity_mismatch",
        "repository_identity_entropy_unavailable",
        "repository_identity_persistence_failed"
    ]
);
invocation_string_enum!(
    RepositorySetupErrorCodeV1,
    "RepositorySetupErrorCodeV1",
    [
        "shipped_profile_unavailable",
        "selected_profile_decision_invalid",
        "unresolved_mode",
        "init_rejects_refresh_flags",
        "root_already_initialized",
        "refresh_root_missing",
        "root_not_directory",
        "root_symlink_refused",
        "canonical_root_inspect_failed",
        "canonical_root_create_failed",
        "rewrite_has_no_materializer",
        "runtime_state_target_unsafe",
        "runtime_state_reset_plan_too_large",
        "runtime_state_reset_projection_too_large",
        "runtime_state_mutation_failed",
        "repository_authority_recovery_blocked",
        "repository_identity_unsafe",
        "repository_identity_mismatch",
        "repository_identity_entropy_unavailable",
        "repository_identity_persistence_failed"
    ]
);
invocation_string_enum!(
    AuthorCharterOperationV1,
    "AuthorCharterOperationV1",
    ["author", "approve", "promote", "validate"]
);
invocation_string_enum!(
    UseCaseCauseV1,
    "UseCaseCauseV1",
    ["prerequisite", "caller_refusal", "implementation_failure"]
);
invocation_string_enum!(
    AuthorProjectContextOperationV1,
    "AuthorProjectContextOperationV1",
    ["author", "validate"]
);
invocation_string_enum!(
    AuthorProjectContextRefusalKindV1,
    "AuthorProjectContextRefusalKindV1",
    [
        "invalid_request",
        "missing_system_root",
        "invalid_system_root",
        "shipped_contract_unavailable",
        "selected_decision_missing",
        "system_root_inspection_failed",
        "unsafe_system_root",
        "canonical_inspection_failed",
        "malformed_structured_input",
        "incomplete_structured_input",
        "existing_canonical_truth",
        "mutation_refused",
        "mutation_safety_refused",
        "mutation_io_failed",
        "unsupported_platform_strict_mutation"
    ]
);
invocation_string_enum!(
    ApproverAdministrationOperationV1,
    "ApproverAdministrationOperationV1",
    [
        "bootstrap",
        "add_credential",
        "revoke_credential",
        "update_mapping"
    ]
);
invocation_string_enum!(
    PipelineRouteStageStatusV1,
    "PipelineRouteStageStatusV1",
    ["active", "skipped", "blocked", "next"]
);
invocation_string_enum!(
    RepositoryDoctorFailureKindV1,
    "RepositoryDoctorFailureKindV1",
    ["profile_resolution", "profile_decision"]
);
invocation_string_enum!(
    RepositoryDoctorFailureReasonV1,
    "RepositoryDoctorFailureReasonV1",
    [
        "shipped_profile_unavailable",
        "selected_profile_decision_invalid"
    ]
);
invocation_string_enum!(
    TransportFailureKind,
    "TransportFailureKind",
    [
        "repository",
        "acquisition",
        "decoding",
        "framing",
        "serialization",
        "emission",
        "adapter"
    ]
);
invocation_string_enum!(
    AuthorInputTransportRouteV1,
    "AuthorInputTransportRouteV1",
    ["charter", "project_context"]
);
invocation_string_enum!(
    TransportIoErrorKindV1,
    "TransportIoErrorKindV1",
    [
        "not_found",
        "permission_denied",
        "connection_refused",
        "connection_reset",
        "connection_aborted",
        "not_connected",
        "addr_in_use",
        "addr_not_available",
        "broken_pipe",
        "already_exists",
        "would_block",
        "invalid_input",
        "invalid_data",
        "timed_out",
        "write_zero",
        "interrupted",
        "unsupported",
        "unexpected_eof",
        "out_of_memory",
        "other"
    ]
);
invocation_string_enum!(
    AuthorWorkingDirectoryCommandV1,
    "AuthorWorkingDirectoryCommandV1",
    [
        "charter_author",
        "charter_approve",
        "charter_promote",
        "charter_validate",
        "project_context_author",
        "project_context_validate"
    ]
);
invocation_string_enum!(
    RepositoryInvocationCommandV1,
    "RepositoryInvocationCommandV1",
    [
        "charter_author",
        "charter_approve",
        "charter_promote",
        "charter_validate",
        "approver_bootstrap",
        "approver_add_credential",
        "approver_revoke_credential",
        "approver_update_mapping"
    ]
);
invocation_string_enum!(
    InvocationResponseKind,
    "InvocationResponseKind",
    ["operation", "use_case", "transport_failure"]
);

impl_schema!(
    RepositoryDoctorConditionV1,
    "RepositoryDoctorConditionV1",
    |generator| closed_object(
        vec![
            ("condition_ref", generator.subschema_for::<ExactRef>()),
            (
                "condition_definition_fingerprint",
                generator.subschema_for::<Fingerprint>(),
            ),
            (
                "outcome",
                generator.subschema_for::<ProfileConditionOutcomeV1>(),
            ),
            (
                "reason",
                generator.subschema_for::<ProfileConditionReasonV1>(),
            ),
            (
                "evidence_closure_fingerprint",
                nullable::<Fingerprint>(generator),
            ),
        ],
        &[
            "condition_ref",
            "condition_definition_fingerprint",
            "outcome",
            "reason",
            "evidence_closure_fingerprint",
        ],
    )
);

impl_schema!(
    RepositoryDoctorCapabilityV1,
    "RepositoryDoctorCapabilityV1",
    |generator| closed_object(
        vec![
            (
                "instance_id",
                generator.subschema_for::<BoundedString<255>>()
            ),
            (
                "capability_id",
                generator.subschema_for::<BoundedString<255>>()
            ),
            ("contract_ref", generator.subschema_for::<ExactRef>()),
            (
                "contract_fingerprint",
                generator.subschema_for::<Fingerprint>()
            ),
        ],
        &[
            "instance_id",
            "capability_id",
            "contract_ref",
            "contract_fingerprint"
        ],
    )
);

impl_schema!(
    RepositoryDoctorArtifactV1,
    "RepositoryDoctorArtifactV1",
    |generator| closed_object(
        vec![
            (
                "instance_id",
                generator.subschema_for::<BoundedString<255>>()
            ),
            ("kind_ref", generator.subschema_for::<ExactRef>()),
            ("role_id", nullable::<BoundedString<255>>(generator)),
            (
                "capability_ids",
                generator.subschema_for::<BoundedVec<BoundedString<255>, 128>>(),
            ),
            (
                "canonical_path",
                generator.subschema_for::<BoundedString<4096>>()
            ),
            (
                "requiredness",
                generator.subschema_for::<ProfileRequirednessModeV1>(),
            ),
            ("condition_ref", nullable::<ExactRef>(generator)),
            (
                "condition_outcome",
                nullable::<ProfileConditionOutcomeV1>(generator),
            ),
            (
                "condition_reason",
                nullable::<ProfileConditionReasonV1>(generator),
            ),
            (
                "evidence_closure_fingerprint",
                nullable::<Fingerprint>(generator),
            ),
            (
                "applicability",
                generator.subschema_for::<ArtifactApplicabilityV1>(),
            ),
            (
                "inspection_status",
                generator.subschema_for::<ArtifactInspectionStatusV1>(),
            ),
            (
                "inspection_reason",
                generator.subschema_for::<ArtifactInspectionReasonV1>(),
            ),
        ],
        &[
            "instance_id",
            "kind_ref",
            "role_id",
            "capability_ids",
            "canonical_path",
            "requiredness",
            "condition_ref",
            "condition_outcome",
            "condition_reason",
            "evidence_closure_fingerprint",
            "applicability",
            "inspection_status",
            "inspection_reason",
        ],
    )
);

impl_schema!(
    RepositoryDoctorProjectContextV1,
    "RepositoryDoctorProjectContextV1",
    |generator| closed_object(
        vec![
            (
                "instance_id",
                generator.subschema_for::<BoundedString<255>>()
            ),
            ("kind_ref", generator.subschema_for::<ExactRef>()),
            (
                "canonical_path",
                generator.subschema_for::<BoundedString<4096>>()
            ),
            (
                "source_fingerprint",
                generator.subschema_for::<Fingerprint>()
            ),
            (
                "rendered_output_fingerprint",
                generator.subschema_for::<Fingerprint>(),
            ),
            (
                "rendered_media_type",
                generator.subschema_for::<BoundedString<255>>(),
            ),
        ],
        &[
            "instance_id",
            "kind_ref",
            "canonical_path",
            "source_fingerprint",
            "rendered_output_fingerprint",
            "rendered_media_type",
        ],
    )
);

impl_schema!(
    RepositoryDoctorCharterDefinitionV1,
    "RepositoryDoctorCharterDefinitionV1",
    |generator| closed_object(
        vec![
            ("exact_ref", generator.subschema_for::<ExactRef>()),
            (
                "definition_fingerprint",
                generator.subschema_for::<Fingerprint>(),
            ),
            (
                "package_path",
                generator.subschema_for::<BoundedString<4096>>()
            ),
        ],
        &["exact_ref", "definition_fingerprint", "package_path"],
    )
);

impl_schema!(
    RepositoryDoctorCharterV1,
    "RepositoryDoctorCharterV1",
    |generator| closed_object(
        vec![
            (
                "instance_id",
                generator.subschema_for::<BoundedString<255>>()
            ),
            ("kind_ref", generator.subschema_for::<ExactRef>()),
            (
                "canonical_path",
                generator.subschema_for::<BoundedString<4096>>()
            ),
            (
                "definition_closure_status",
                generator.subschema_for::<DoctorCharterDefinitionClosureStatusV1>(),
            ),
            (
                "definition_closure",
                generator.subschema_for::<BoundedVec<RepositoryDoctorCharterDefinitionV1, 128>>(),
            ),
            (
                "canonical_status",
                generator.subschema_for::<ArtifactInspectionStatusV1>(),
            ),
            (
                "canonical_reason",
                generator.subschema_for::<ArtifactInspectionReasonV1>(),
            ),
            (
                "lifecycle_state",
                generator.subschema_for::<DoctorCharterLifecycleStateV1>(),
            ),
            ("source_fingerprint", nullable::<Fingerprint>(generator)),
            (
                "rendered_output_fingerprint",
                nullable::<Fingerprint>(generator),
            ),
            (
                "rendered_media_type",
                nullable::<BoundedString<255>>(generator),
            ),
            (
                "next_actions",
                generator.subschema_for::<BoundedVec<DoctorCharterNextActionV1, 128>>(),
            ),
        ],
        &[
            "instance_id",
            "kind_ref",
            "canonical_path",
            "definition_closure_status",
            "definition_closure",
            "canonical_status",
            "canonical_reason",
            "lifecycle_state",
            "source_fingerprint",
            "rendered_output_fingerprint",
            "rendered_media_type",
            "next_actions",
        ],
    )
);

impl_schema!(
    RepositoryDoctorResultV1,
    "RepositoryDoctorResultV1",
    |generator| closed_object(
        vec![
            ("profile", generator.subschema_for::<ExactBinding>()),
            (
                "stable_role_registry",
                generator.subschema_for::<ExactBinding>()
            ),
            (
                "conditions",
                generator.subschema_for::<BoundedVec<RepositoryDoctorConditionV1, 128>>(),
            ),
            (
                "capabilities",
                generator.subschema_for::<BoundedVec<RepositoryDoctorCapabilityV1, 128>>(),
            ),
            (
                "artifacts",
                generator.subschema_for::<BoundedVec<RepositoryDoctorArtifactV1, 128>>(),
            ),
            (
                "project_context",
                nullable::<RepositoryDoctorProjectContextV1>(generator),
            ),
            ("charter", nullable::<RepositoryDoctorCharterV1>(generator)),
            (
                "readiness",
                generator.subschema_for::<RepositoryReadinessStatusV1>(),
            ),
        ],
        &[
            "profile",
            "stable_role_registry",
            "conditions",
            "capabilities",
            "artifacts",
            "project_context",
            "charter",
            "readiness",
        ],
    )
);

impl_schema!(
    ReferenceFingerprintV1,
    "ReferenceFingerprintV1",
    |generator| closed_object(
        vec![
            (
                "reference",
                generator.subschema_for::<BoundedString<4096>>()
            ),
            ("fingerprint", generator.subschema_for::<Fingerprint>()),
        ],
        &["reference", "fingerprint"],
    )
);

impl_schema!(
    RepositorySetupArtifactV1,
    "RepositorySetupArtifactV1",
    |generator| closed_object(
        vec![
            (
                "artifact",
                generator.subschema_for::<RepositoryDoctorArtifactV1>()
            ),
            (
                "action",
                generator.subschema_for::<RepositorySetupArtifactActionKindV1>(),
            ),
        ],
        &["artifact", "action"],
    )
);

impl_schema!(
    RepositorySetupResultV1,
    "RepositorySetupResultV1",
    |generator| closed_object(
        vec![
            (
                "requested_mode",
                generator.subschema_for::<RepositorySetupModeV1>()
            ),
            (
                "resolved_mode",
                generator.subschema_for::<RepositorySetupModeV1>()
            ),
            (
                "root_action",
                generator.subschema_for::<RepositorySetupRootActionV1>()
            ),
            ("profile", generator.subschema_for::<ExactBinding>()),
            (
                "stable_role_registry",
                generator.subschema_for::<ExactBinding>()
            ),
            (
                "conditions",
                generator.subschema_for::<BoundedVec<RepositoryDoctorConditionV1, 128>>(),
            ),
            (
                "capabilities",
                generator.subschema_for::<BoundedVec<RepositoryDoctorCapabilityV1, 128>>(),
            ),
            (
                "artifacts",
                generator.subschema_for::<BoundedVec<RepositorySetupArtifactV1, 128>>(),
            ),
            (
                "reset_paths",
                generator.subschema_for::<BoundedVec<BoundedString<4096>, 128>>(),
            ),
            (
                "readiness",
                generator.subschema_for::<RepositoryReadinessStatusV1>()
            ),
            ("reset_applied", json_schema!({"type": "boolean"})),
        ],
        &[
            "requested_mode",
            "resolved_mode",
            "root_action",
            "profile",
            "stable_role_registry",
            "conditions",
            "capabilities",
            "artifacts",
            "reset_paths",
            "readiness",
            "reset_applied",
        ],
    )
);

impl_schema!(
    AuthorCharterResultV1,
    "AuthorCharterResultV1",
    |generator| {
        closed_object(
            vec![
                (
                    "operation",
                    generator.subschema_for::<AuthorCharterOperationV1>(),
                ),
                ("canonical_path", nullable::<BoundedString<4096>>(generator)),
                ("source_fingerprint", nullable::<Fingerprint>(generator)),
                (
                    "rendered_output_fingerprint",
                    nullable::<Fingerprint>(generator),
                ),
                ("intake", nullable::<ReferenceFingerprintV1>(generator)),
                ("candidate", nullable::<ReferenceFingerprintV1>(generator)),
                ("approval", nullable::<ReferenceFingerprintV1>(generator)),
                ("promotion", nullable::<ReferenceFingerprintV1>(generator)),
                (
                    "changed_paths",
                    generator.subschema_for::<BoundedVec<BoundedString<4096>, 128>>(),
                ),
                (
                    "next_actions",
                    generator.subschema_for::<BoundedVec<BoundedString<1024>, 128>>(),
                ),
            ],
            &[
                "operation",
                "canonical_path",
                "source_fingerprint",
                "rendered_output_fingerprint",
                "intake",
                "candidate",
                "approval",
                "promotion",
                "changed_paths",
                "next_actions",
            ],
        )
    }
);

fn author_operation_refusal_schema(generator: &mut SchemaGenerator) -> Schema {
    closed_object(
        vec![
            ("code", generator.subschema_for::<BoundedString<255>>()),
            ("message", generator.subschema_for::<BoundedString<4096>>()),
            ("retryable", json_schema!({"type": "boolean"})),
        ],
        &["code", "message", "retryable"],
    )
}

fn repository_invocation_failure_details_schema(generator: &mut SchemaGenerator) -> Schema {
    closed_object(
        vec![
            ("schema_id", generator.subschema_for::<BoundedString<255>>()),
            (
                "schema_version",
                generator.subschema_for::<BoundedString<255>>(),
            ),
            ("operation", generator.subschema_for::<BoundedString<255>>()),
            ("stage", nullable::<BoundedString<255>>(generator)),
            ("status", generator.subschema_for::<BoundedString<255>>()),
            (
                "repository_identity_fingerprint",
                nullable::<Fingerprint>(generator),
            ),
            ("operation_id", nullable::<BoundedString<255>>(generator)),
            (
                "changed_paths",
                generator.subschema_for::<BoundedVec<BoundedString<4096>, 128>>(),
            ),
            ("refusal", author_operation_refusal_schema(generator)),
            (
                "next_actions",
                generator.subschema_for::<BoundedVec<BoundedString<1024>, 128>>(),
            ),
        ],
        &[
            "schema_id",
            "schema_version",
            "operation",
            "stage",
            "status",
            "repository_identity_fingerprint",
            "operation_id",
            "changed_paths",
            "refusal",
            "next_actions",
        ],
    )
}

impl_schema!(
    AuthorCharterDetailsV1,
    "AuthorCharterDetailsV1",
    |generator| closed_object(
        vec![
            ("cause", generator.subschema_for::<UseCaseCauseV1>()),
            (
                "operation",
                generator.subschema_for::<AuthorCharterOperationV1>(),
            ),
            ("canonical_path", nullable::<BoundedString<4096>>(generator)),
            ("source_fingerprint", nullable::<Fingerprint>(generator)),
            (
                "rendered_output_fingerprint",
                nullable::<Fingerprint>(generator),
            ),
            ("intake", nullable::<ReferenceFingerprintV1>(generator)),
            ("candidate", nullable::<ReferenceFingerprintV1>(generator)),
            ("approval", nullable::<ReferenceFingerprintV1>(generator)),
            ("promotion", nullable::<ReferenceFingerprintV1>(generator)),
            (
                "changed_paths",
                generator.subschema_for::<BoundedVec<BoundedString<4096>, 128>>(),
            ),
            ("refusal", author_operation_refusal_schema(generator)),
            (
                "next_actions",
                generator.subschema_for::<BoundedVec<BoundedString<1024>, 128>>(),
            ),
        ],
        &[
            "cause",
            "operation",
            "canonical_path",
            "source_fingerprint",
            "rendered_output_fingerprint",
            "intake",
            "candidate",
            "approval",
            "promotion",
            "changed_paths",
            "refusal",
            "next_actions",
        ],
    )
);

impl_schema!(
    AuthorProjectContextResultV1,
    "AuthorProjectContextResultV1",
    |generator| {
        let author = closed_object(
            vec![
                ("result_kind", json_schema!({"const": "author"})),
                (
                    "input_mode",
                    generator.subschema_for::<BoundedString<255>>(),
                ),
                (
                    "input_source",
                    generator.subschema_for::<BoundedString<4096>>(),
                ),
                (
                    "canonical_path",
                    generator.subschema_for::<BoundedString<4096>>(),
                ),
                ("bytes_written", generator.subschema_for::<SafeU64>()),
                (
                    "source_fingerprint",
                    generator.subschema_for::<Fingerprint>(),
                ),
                (
                    "rendered_output_fingerprint",
                    generator.subschema_for::<Fingerprint>(),
                ),
                (
                    "rendered_media_type",
                    generator.subschema_for::<BoundedString<255>>(),
                ),
            ],
            &[
                "result_kind",
                "input_mode",
                "input_source",
                "canonical_path",
                "bytes_written",
                "source_fingerprint",
                "rendered_output_fingerprint",
                "rendered_media_type",
            ],
        );
        let validate = closed_object(
            vec![
                ("result_kind", json_schema!({"const": "validate"})),
                (
                    "input_mode",
                    generator.subschema_for::<BoundedString<255>>(),
                ),
                (
                    "input_source",
                    generator.subschema_for::<BoundedString<4096>>(),
                ),
            ],
            &["result_kind", "input_mode", "input_source"],
        );
        json_schema!({"oneOf": [author, validate]})
    }
);

impl_schema!(
    AuthorProjectContextDetailsV1,
    "AuthorProjectContextDetailsV1",
    |generator| closed_object(
        vec![
            ("cause", generator.subschema_for::<UseCaseCauseV1>()),
            (
                "operation",
                generator.subschema_for::<AuthorProjectContextOperationV1>(),
            ),
            (
                "input_mode",
                generator.subschema_for::<BoundedString<255>>(),
            ),
            (
                "input_source",
                generator.subschema_for::<BoundedString<4096>>(),
            ),
            (
                "refusal_kind",
                generator.subschema_for::<AuthorProjectContextRefusalKindV1>(),
            ),
            ("summary", generator.subschema_for::<BoundedString<4096>>()),
            (
                "broken_subject",
                generator.subschema_for::<BoundedString<4096>>(),
            ),
            (
                "next_safe_action",
                generator.subschema_for::<BoundedString<4096>>(),
            ),
        ],
        &[
            "cause",
            "operation",
            "input_mode",
            "input_source",
            "refusal_kind",
            "summary",
            "broken_subject",
            "next_safe_action",
        ],
    )
);

impl_schema!(
    ApproverAdministrationResultV1,
    "ApproverAdministrationResultV1",
    |generator| closed_object(
        vec![
            (
                "operation",
                generator.subschema_for::<ApproverAdministrationOperationV1>(),
            ),
            (
                "operation_id",
                generator.subschema_for::<BoundedString<255>>()
            ),
            (
                "repository_identity_fingerprint",
                generator.subschema_for::<Fingerprint>(),
            ),
            (
                "prior_registry_state",
                nullable::<ReferenceFingerprintV1>(generator),
            ),
            (
                "result_registry_state",
                nullable::<ReferenceFingerprintV1>(generator),
            ),
            ("transition", nullable::<ReferenceFingerprintV1>(generator)),
            (
                "registration",
                nullable::<ReferenceFingerprintV1>(generator)
            ),
            ("assertion", nullable::<ReferenceFingerprintV1>(generator)),
            (
                "changed_paths",
                generator.subschema_for::<BoundedVec<BoundedString<4096>, 128>>(),
            ),
            (
                "next_actions",
                generator.subschema_for::<BoundedVec<BoundedString<1024>, 128>>(),
            ),
        ],
        &[
            "operation",
            "operation_id",
            "repository_identity_fingerprint",
            "prior_registry_state",
            "result_registry_state",
            "transition",
            "registration",
            "assertion",
            "changed_paths",
            "next_actions",
        ],
    )
);

fn approver_admin_legacy_details_schema(generator: &mut SchemaGenerator) -> Schema {
    closed_object(
        vec![
            (
                "operation_id",
                generator.subschema_for::<BoundedString<255>>(),
            ),
            (
                "repository_identity_fingerprint",
                generator.subschema_for::<Fingerprint>(),
            ),
            (
                "prior_registry_state",
                nullable::<ReferenceFingerprintV1>(generator),
            ),
            (
                "result_registry_state",
                nullable::<ReferenceFingerprintV1>(generator),
            ),
            ("transition", nullable::<ReferenceFingerprintV1>(generator)),
            (
                "registration",
                nullable::<ReferenceFingerprintV1>(generator),
            ),
            ("assertion", nullable::<ReferenceFingerprintV1>(generator)),
        ],
        &[
            "operation_id",
            "repository_identity_fingerprint",
            "prior_registry_state",
            "result_registry_state",
            "transition",
            "registration",
            "assertion",
        ],
    )
}

fn approver_legacy_projection_schema(generator: &mut SchemaGenerator) -> Schema {
    closed_object(
        vec![
            ("projection_kind", json_schema!({"const": "admin"})),
            ("fields", approver_admin_legacy_details_schema(generator)),
        ],
        &["projection_kind", "fields"],
    )
}

impl_schema!(
    ApproverAdministrationDetailsV1,
    "ApproverAdministrationDetailsV1",
    |generator| {
        let legacy_projection = approver_legacy_projection_schema(generator);
        closed_object(
            vec![
                ("cause", generator.subschema_for::<UseCaseCauseV1>()),
                (
                    "operation",
                    generator.subschema_for::<ApproverAdministrationOperationV1>(),
                ),
                (
                    "changed_paths",
                    generator.subschema_for::<BoundedVec<BoundedString<4096>, 128>>(),
                ),
                ("refusal", author_operation_refusal_schema(generator)),
                (
                    "next_actions",
                    generator.subschema_for::<BoundedVec<BoundedString<1024>, 128>>(),
                ),
                (
                    "legacy_projection",
                    json_schema!({"anyOf": [legacy_projection, {"type": "null"}]}),
                ),
            ],
            &[
                "cause",
                "operation",
                "changed_paths",
                "refusal",
                "next_actions",
                "legacy_projection",
            ],
        )
    }
);

impl_schema!(PipelineRouteRunV1, "PipelineRouteRunV1", |generator| {
    closed_object(
        vec![
            ("runner", nullable::<BoundedString<4096>>(generator)),
            ("profile", nullable::<BoundedString<4096>>(generator)),
            (
                "repository_root",
                nullable::<BoundedString<4096>>(generator),
            ),
        ],
        &["runner", "profile", "repository_root"],
    )
});

impl_schema!(
    PipelineRoutingEntryV1,
    "PipelineRoutingEntryV1",
    |generator| closed_object(
        vec![
            ("variable", generator.subschema_for::<BoundedString<255>>()),
            ("value", json_schema!({"type": "boolean"})),
        ],
        &["variable", "value"],
    )
);

impl_schema!(PipelineRouteStateV1, "PipelineRouteStateV1", |generator| {
    closed_object(
        vec![
            ("revision", generator.subschema_for::<SafeU64>()),
            (
                "routing",
                generator.subschema_for::<BoundedVec<PipelineRoutingEntryV1, 128>>(),
            ),
            ("charter_ref", nullable::<BoundedString<4096>>(generator)),
            (
                "project_context_ref",
                nullable::<BoundedString<4096>>(generator),
            ),
            ("run", generator.subschema_for::<PipelineRouteRunV1>()),
        ],
        &[
            "revision",
            "routing",
            "charter_ref",
            "project_context_ref",
            "run",
        ],
    )
});

impl_schema!(
    PipelineRouteStageReasonV1,
    "PipelineRouteStageReasonV1",
    |generator| {
        let skipped = closed_object(
            vec![
                (
                    "reason_kind",
                    json_schema!({"const": "skipped_activation_false"}),
                ),
                (
                    "unsatisfied_variables",
                    generator.subschema_for::<BoundedVec<BoundedString<255>, 128>>(),
                ),
            ],
            &["reason_kind", "unsatisfied_variables"],
        );
        let next = closed_object(
            vec![
                (
                    "reason_kind",
                    json_schema!({"const": "next_missing_route_variables"}),
                ),
                (
                    "missing_variables",
                    generator.subschema_for::<BoundedVec<BoundedString<255>, 128>>(),
                ),
            ],
            &["reason_kind", "missing_variables"],
        );
        let blocked = closed_object(
            vec![
                (
                    "reason_kind",
                    json_schema!({"const": "blocked_by_unresolved_stage"}),
                ),
                (
                    "upstream_stage_id",
                    generator.subschema_for::<BoundedString<255>>(),
                ),
                (
                    "upstream_status",
                    generator.subschema_for::<PipelineRouteStageStatusV1>(),
                ),
            ],
            &["reason_kind", "upstream_stage_id", "upstream_status"],
        );
        json_schema!({"oneOf": [skipped, next, blocked]})
    }
);

impl_schema!(
    PipelineResolvedStageV1,
    "PipelineResolvedStageV1",
    |generator| closed_object(
        vec![
            ("stage_id", generator.subschema_for::<BoundedString<255>>()),
            (
                "status",
                generator.subschema_for::<PipelineRouteStageStatusV1>()
            ),
            ("reason", nullable::<PipelineRouteStageReasonV1>(generator)),
        ],
        &["stage_id", "status", "reason"],
    )
);

impl_schema!(
    PipelineRouteResolveAndPersistResultV1,
    "PipelineRouteResolveAndPersistResultV1",
    |generator| closed_object(
        vec![
            (
                "pipeline_id",
                generator.subschema_for::<BoundedString<255>>()
            ),
            ("state", generator.subschema_for::<PipelineRouteStateV1>()),
            (
                "effective_run",
                generator.subschema_for::<PipelineRouteRunV1>()
            ),
            (
                "stages",
                generator.subschema_for::<BoundedVec<PipelineResolvedStageV1, 128>>(),
            ),
        ],
        &["pipeline_id", "state", "effective_run", "stages"],
    )
);

impl_schema!(
    PipelineRouteResolveCauseV1,
    "PipelineRouteResolveCauseV1",
    |_generator| string_enum(&[
        "catalog",
        "unsupported_selector",
        "ambiguous_selector",
        "unknown_selector",
        "definition",
        "route_state_read",
        "route_variables",
        "route_resolution",
        "route_basis_build",
        "route_basis_persistence_malformed",
        "route_basis_revision_conflict",
        "route_basis_persistence",
    ])
);

impl_schema!(
    PipelineRouteResolveDetailsV1,
    "PipelineRouteResolveDetailsV1",
    |generator| closed_object(
        vec![
            (
                "cause",
                generator.subschema_for::<PipelineRouteResolveCauseV1>()
            ),
            ("reason", generator.subschema_for::<BoundedString<8192>>()),
            ("selector", nullable::<BoundedString<4096>>(generator)),
            (
                "matches",
                generator.subschema_for::<BoundedVec<BoundedString<4096>, 128>>(),
            ),
            ("expected_revision", nullable::<SafeU64>(generator)),
            ("actual_revision", nullable::<SafeU64>(generator)),
        ],
        &[
            "cause",
            "reason",
            "selector",
            "matches",
            "expected_revision",
            "actual_revision",
        ],
    )
);

impl_schema!(
    RepositoryDoctorDetailsV1,
    "RepositoryDoctorDetailsV1",
    |generator| {
        let readiness = closed_object(
            vec![
                ("details_kind", json_schema!({"const": "readiness"})),
                (
                    "result",
                    generator.subschema_for::<RepositoryDoctorResultV1>(),
                ),
            ],
            &["details_kind", "result"],
        );
        let failure = closed_object(
            vec![
                ("details_kind", json_schema!({"const": "doctor_failure"})),
                (
                    "failure_kind",
                    generator.subschema_for::<RepositoryDoctorFailureKindV1>(),
                ),
                (
                    "reason",
                    generator.subschema_for::<RepositoryDoctorFailureReasonV1>(),
                ),
            ],
            &["details_kind", "failure_kind", "reason"],
        );
        json_schema!({"oneOf": [readiness, failure]})
    }
);

impl_schema!(
    RepositorySetupBoundsV1,
    "RepositorySetupBoundsV1",
    |generator| closed_object(
        vec![
            ("observed_count", generator.subschema_for::<SafeU64>()),
            ("limit", generator.subschema_for::<SafeU64>()),
        ],
        &["observed_count", "limit"],
    )
);

impl_schema!(
    RepositorySetupSizeBoundsV1,
    "RepositorySetupSizeBoundsV1",
    |generator| closed_object(
        vec![
            ("observed_bytes", generator.subschema_for::<SafeU64>()),
            ("limit_bytes", generator.subschema_for::<SafeU64>()),
        ],
        &["observed_bytes", "limit_bytes"],
    )
);

impl_schema!(
    RepositorySetupDetailsV1,
    "RepositorySetupDetailsV1",
    |generator| {
        let readiness = closed_object(
            vec![
                ("details_kind", json_schema!({"const": "readiness"})),
                (
                    "result",
                    generator.subschema_for::<RepositorySetupResultV1>(),
                ),
            ],
            &["details_kind", "result"],
        );
        let failure = closed_object(
            vec![
                ("details_kind", json_schema!({"const": "setup_failure"})),
                (
                    "code",
                    generator.subschema_for::<RepositorySetupErrorCodeV1>(),
                ),
                (
                    "failure_kind",
                    generator.subschema_for::<RepositorySetupErrorKindV1>(),
                ),
                (
                    "reason",
                    generator.subschema_for::<RepositorySetupErrorReasonV1>(),
                ),
                (
                    "repo_relative_path",
                    nullable::<BoundedString<4096>>(generator),
                ),
                ("bounds", nullable::<RepositorySetupBoundsV1>(generator)),
                (
                    "size_bounds",
                    nullable::<RepositorySetupSizeBoundsV1>(generator),
                ),
            ],
            &[
                "details_kind",
                "code",
                "failure_kind",
                "reason",
                "repo_relative_path",
                "bounds",
                "size_bounds",
            ],
        );
        json_schema!({"oneOf": [readiness, failure]})
    }
);

impl_schema!(
    TransportFailureDetailsV1,
    "TransportFailureDetailsV1",
    |generator| {
        let author_input = closed_object(
            vec![
                ("presentation_kind", json_schema!({"const": "author_input"})),
                (
                    "route",
                    generator.subschema_for::<AuthorInputTransportRouteV1>(),
                ),
                (
                    "input_source",
                    generator.subschema_for::<BoundedString<65536>>(),
                ),
                ("error", nullable::<TransportIoErrorV1>(generator)),
            ],
            &["presentation_kind", "route", "input_source", "error"],
        );
        let repository_invocation = closed_object(
            vec![
                (
                    "presentation_kind",
                    json_schema!({"const": "repository_invocation"}),
                ),
                (
                    "command",
                    generator.subschema_for::<RepositoryInvocationCommandV1>(),
                ),
                (
                    "failure",
                    repository_invocation_failure_details_schema(generator),
                ),
            ],
            &["presentation_kind", "command", "failure"],
        );
        let working_directory = closed_object(
            vec![
                (
                    "presentation_kind",
                    json_schema!({"const": "working_directory"}),
                ),
                (
                    "command",
                    generator.subschema_for::<AuthorWorkingDirectoryCommandV1>(),
                ),
                ("error", generator.subschema_for::<TransportIoErrorV1>()),
            ],
            &["presentation_kind", "command", "error"],
        );
        closed_object(
            vec![
                (
                    "failure_kind",
                    generator.subschema_for::<TransportFailureKind>(),
                ),
                ("reason_code", generator.subschema_for::<Token<255>>()),
                (
                    "presentation",
                    json_schema!({"oneOf": [author_input, working_directory, repository_invocation]}),
                ),
            ],
            &["failure_kind", "reason_code"],
        )
    }
);

impl_schema!(TransportIoErrorV1, "TransportIoErrorV1", |generator| {
    closed_object(
        vec![
            ("kind", generator.subschema_for::<TransportIoErrorKindV1>()),
            ("raw_os_error", nullable::<i32>(generator)),
        ],
        &["kind", "raw_os_error"],
    )
});

fn exact_binding_schema(generator: &mut SchemaGenerator, reference: &'static str) -> Schema {
    closed_object(
        vec![
            ("ref", json_schema!({"const": reference})),
            ("fingerprint", generator.subschema_for::<Fingerprint>()),
        ],
        &["ref", "fingerprint"],
    )
}

impl_schema!(
    RepositorySetupRequestV1,
    "RepositorySetupRequestV1",
    |generator| {
        closed_object(
            vec![
                ("mode", generator.subschema_for::<RepositorySetupModeV1>()),
                ("rewrite", json_schema!({"type": "boolean"})),
                ("reset_state", json_schema!({"type": "boolean"})),
            ],
            &["mode", "rewrite", "reset_state"],
        )
    }
);

impl_schema!(
    RepositoryDoctorRequestV1,
    "RepositoryDoctorRequestV1",
    |_generator| closed_object(Vec::new(), &[])
);

impl_schema!(
    CharterAcquisitionModeV1,
    "CharterAcquisitionModeV1",
    |_generator| string_enum(&["guided_adaptive", "express", "agent_assisted"])
);

impl_schema!(
    CharterIntakeSourceKindV1,
    "CharterIntakeSourceKindV1",
    |_generator| {
        string_enum(&[
            "user_declaration",
            "evidenced_inference",
            "deterministic_default",
            "known_unknown",
            "contradiction",
            "waiver",
        ])
    }
);

impl_schema!(CharterInputEntryV1, "CharterInputEntryV1", |generator| {
    closed_object(
        vec![
            ("key", json_schema!({"type": "string", "maxLength": 65_536})),
            ("value", generator.subschema_for::<CharterInputValueV1>()),
        ],
        &["key", "value"],
    )
});

impl_schema!(CharterInputValueV1, "CharterInputValueV1", |generator| {
    let null_value = closed_object(
        vec![("value_kind", json_schema!({"const": "null"}))],
        &["value_kind"],
    );
    let boolean = closed_object(
        vec![
            ("value_kind", json_schema!({"const": "boolean"})),
            ("value", json_schema!({"type": "boolean"})),
        ],
        &["value_kind", "value"],
    );
    let number = closed_object(
        vec![
            ("value_kind", json_schema!({"const": "number"})),
            (
                "value",
                json_schema!({
                    "type": "string",
                    "maxLength": 65_536,
                    "pattern": "^-?(0|[1-9][0-9]*)(\\.[0-9]+)?([eE][+-]?[0-9]+)?$"
                }),
            ),
        ],
        &["value_kind", "value"],
    );
    let string = closed_object(
        vec![
            ("value_kind", json_schema!({"const": "string"})),
            (
                "value",
                json_schema!({"type": "string", "maxLength": 65_536}),
            ),
        ],
        &["value_kind", "value"],
    );
    let sequence = closed_object(
        vec![
            ("value_kind", json_schema!({"const": "sequence"})),
            (
                "values",
                generator.subschema_for::<BoundedVec<CharterInputValueV1, 128>>(),
            ),
        ],
        &["value_kind", "values"],
    );
    let mapping = closed_object(
        vec![
            ("value_kind", json_schema!({"const": "mapping"})),
            (
                "entries",
                generator.subschema_for::<BoundedVec<CharterInputEntryV1, 128>>(),
            ),
        ],
        &["value_kind", "entries"],
    );
    json_schema!({"oneOf": [null_value, boolean, number, string, sequence, mapping]})
});

impl_schema!(
    CharterCoverageSubmissionV1,
    "CharterCoverageSubmissionV1",
    |generator| {
        closed_object(
            vec![
                (
                    "coverage_id",
                    generator.subschema_for::<BoundedString<255>>(),
                ),
                (
                    "source_kind",
                    generator.subschema_for::<CharterIntakeSourceKindV1>(),
                ),
                (
                    "value_ref",
                    generator.subschema_for::<BoundedString<4096>>(),
                ),
                (
                    "evidence_refs",
                    generator.subschema_for::<BoundedVec<BoundedString<4096>, 128>>(),
                ),
                (
                    "confidence",
                    generator.subschema_for::<BoundedString<255>>(),
                ),
                ("freshness", nullable::<BoundedString<255>>(generator)),
                (
                    "sensitivity",
                    generator.subschema_for::<BoundedString<255>>(),
                ),
                (
                    "contradiction_refs",
                    generator.subschema_for::<BoundedVec<BoundedString<4096>, 128>>(),
                ),
                ("waiver_ref", nullable::<BoundedString<4096>>(generator)),
            ],
            &[
                "coverage_id",
                "source_kind",
                "value_ref",
                "evidence_refs",
                "confidence",
                "freshness",
                "sensitivity",
                "contradiction_refs",
                "waiver_ref",
            ],
        )
    }
);

impl_schema!(
    CharterIntakeConsumerV1,
    "CharterIntakeConsumerV1",
    |generator| {
        closed_object(
            vec![
                ("kind", generator.subschema_for::<BoundedString<255>>()),
                ("id", generator.subschema_for::<BoundedString<255>>()),
                ("version", generator.subschema_for::<BoundedString<255>>()),
            ],
            &["kind", "id", "version"],
        )
    }
);

impl_schema!(
    CharterIntakeEnvelopeV1,
    "CharterIntakeEnvelopeV1",
    |generator| {
        closed_object(
            vec![
                (
                    "mode",
                    generator.subschema_for::<CharterAcquisitionModeV1>(),
                ),
                ("content", generator.subschema_for::<CharterInputValueV1>()),
                (
                    "coverage",
                    generator.subschema_for::<BoundedVec<CharterCoverageSubmissionV1, 128>>(),
                ),
                (
                    "consumer",
                    generator.subschema_for::<CharterIntakeConsumerV1>(),
                ),
                (
                    "prompt_event_refs",
                    generator.subschema_for::<BoundedVec<BoundedString<4096>, 128>>(),
                ),
                (
                    "finalized_at_utc",
                    generator.subschema_for::<UtcTimestamp>(),
                ),
                (
                    "expected_current_fingerprint",
                    nullable::<Fingerprint>(generator),
                ),
            ],
            &[
                "mode",
                "content",
                "coverage",
                "consumer",
                "prompt_event_refs",
                "finalized_at_utc",
                "expected_current_fingerprint",
            ],
        )
    }
);

impl_schema!(
    AuthorCharterRequestV1,
    "AuthorCharterRequestV1",
    |generator| {
        let author = closed_object(
            vec![
                ("operation", json_schema!({"const": "author"})),
                (
                    "mode",
                    generator.subschema_for::<CharterAcquisitionModeV1>(),
                ),
                (
                    "intake",
                    generator.subschema_for::<CharterIntakeEnvelopeV1>(),
                ),
            ],
            &["operation", "mode", "intake"],
        );
        let approve = closed_object(
            vec![
                ("operation", json_schema!({"const": "approve"})),
                (
                    "candidate_ref",
                    generator.subschema_for::<BoundedString<4096>>(),
                ),
                (
                    "approval_class",
                    generator.subschema_for::<BoundedString<255>>(),
                ),
                (
                    "authority_ref",
                    generator.subschema_for::<BoundedString<4096>>(),
                ),
                (
                    "accepted_waiver_refs",
                    generator.subschema_for::<BoundedVec<BoundedString<4096>, 128>>(),
                ),
            ],
            &[
                "operation",
                "candidate_ref",
                "approval_class",
                "authority_ref",
                "accepted_waiver_refs",
            ],
        );
        let promote = closed_object(
            vec![
                ("operation", json_schema!({"const": "promote"})),
                (
                    "candidate_ref",
                    generator.subschema_for::<BoundedString<4096>>(),
                ),
                (
                    "approval_ref",
                    generator.subschema_for::<BoundedString<4096>>(),
                ),
                (
                    "expected_current_fingerprint",
                    nullable::<Fingerprint>(generator),
                ),
            ],
            &[
                "operation",
                "candidate_ref",
                "approval_ref",
                "expected_current_fingerprint",
            ],
        );
        let validate = closed_object(
            vec![("operation", json_schema!({"const": "validate"}))],
            &["operation"],
        );
        json_schema!({"oneOf": [author, approve, promote, validate]})
    }
);

impl_schema!(
    ProjectContextInputV1,
    "ProjectContextInputV1",
    |generator| {
        closed_object(
            vec![
                ("schema_id", generator.subschema_for::<BoundedString<255>>()),
                (
                    "schema_version",
                    generator.subschema_for::<BoundedString<255>>(),
                ),
                ("record_id", generator.subschema_for::<BoundedString<255>>()),
                ("summary", generator.subschema_for::<BoundedString<4096>>()),
                (
                    "system_boundaries",
                    generator.subschema_for::<BoundedVec<BoundedString<4096>, 128>>(),
                ),
                (
                    "ownership",
                    generator.subschema_for::<BoundedVec<BoundedString<4096>, 128>>(),
                ),
                (
                    "authoritative_references",
                    generator.subschema_for::<BoundedVec<BoundedString<4096>, 128>>(),
                ),
                (
                    "known_unknowns",
                    generator.subschema_for::<BoundedVec<BoundedString<4096>, 128>>(),
                ),
            ],
            &[
                "schema_id",
                "schema_version",
                "record_id",
                "summary",
                "system_boundaries",
                "ownership",
                "authoritative_references",
                "known_unknowns",
            ],
        )
    }
);

impl_schema!(
    AuthorProjectContextRequestV1,
    "AuthorProjectContextRequestV1",
    |generator| {
        let author = closed_object(
            vec![
                ("operation", json_schema!({"const": "author"})),
                ("input", generator.subschema_for::<ProjectContextInputV1>()),
            ],
            &["operation", "input"],
        );
        let validate = closed_object(
            vec![
                ("operation", json_schema!({"const": "validate"})),
                ("input", generator.subschema_for::<ProjectContextInputV1>()),
            ],
            &["operation", "input"],
        );
        json_schema!({"oneOf": [author, validate]})
    }
);

impl_schema!(
    ApproverMappingRequestV1,
    "ApproverMappingRequestV1",
    |generator| {
        closed_object(
            vec![
                (
                    "approval_class",
                    generator.subschema_for::<BoundedString<255>>(),
                ),
                (
                    "authority_ref",
                    generator.subschema_for::<BoundedString<4096>>(),
                ),
            ],
            &["approval_class", "authority_ref"],
        )
    }
);

impl_schema!(
    ApproverAdministrationRequestV1,
    "ApproverAdministrationRequestV1",
    |generator| {
        let mappings = generator.subschema_for::<BoundedVec<ApproverMappingRequestV1, 128>>();
        let bootstrap = closed_object(
            vec![
                ("operation", json_schema!({"const": "bootstrap"})),
                ("initial_charter_quorum", mappings.clone()),
            ],
            &["operation", "initial_charter_quorum"],
        );
        let add = closed_object(
            vec![
                ("operation", json_schema!({"const": "add_credential"})),
                ("approval_mappings", mappings.clone()),
            ],
            &["operation", "approval_mappings"],
        );
        let revoke = closed_object(
            vec![
                ("operation", json_schema!({"const": "revoke_credential"})),
                (
                    "credential_id_hash",
                    generator.subschema_for::<BoundedString<4096>>(),
                ),
            ],
            &["operation", "credential_id_hash"],
        );
        let update = closed_object(
            vec![
                ("operation", json_schema!({"const": "update_mapping"})),
                (
                    "credential_id_hash",
                    generator.subschema_for::<BoundedString<4096>>(),
                ),
                ("approval_mappings", mappings),
            ],
            &["operation", "credential_id_hash", "approval_mappings"],
        );
        json_schema!({"oneOf": [bootstrap, add, revoke, update]})
    }
);

impl_schema!(
    PipelineRouteResolveAndPersistRequestV1,
    "PipelineRouteResolveAndPersistRequestV1",
    |generator| {
        closed_object(
            vec![("selector", generator.subschema_for::<BoundedString<4096>>())],
            &["selector"],
        )
    }
);

fn use_case_request_schema<T: JsonSchema>(
    generator: &mut SchemaGenerator,
    use_case_ref: &'static str,
) -> Schema {
    closed_object(
        vec![
            (
                "schema",
                exact_binding_schema(generator, "handbook.use-case-request@1.0.0"),
            ),
            ("use_case", exact_binding_schema(generator, use_case_ref)),
            ("request", generator.subschema_for::<T>()),
            (
                "request_fingerprint",
                generator.subschema_for::<Fingerprint>(),
            ),
        ],
        &["schema", "use_case", "request", "request_fingerprint"],
    )
}

impl_schema!(UseCaseRequestV1, "UseCaseRequestV1", |generator| {
    json_schema!({"oneOf": [
        use_case_request_schema::<RepositorySetupRequestV1>(generator, "handbook.use-case.repository-setup@1.0.0"),
        use_case_request_schema::<RepositoryDoctorRequestV1>(generator, "handbook.use-case.repository-doctor@1.0.0"),
        use_case_request_schema::<AuthorCharterRequestV1>(generator, "handbook.use-case.author-charter@1.0.0"),
        use_case_request_schema::<AuthorProjectContextRequestV1>(generator, "handbook.use-case.author-project-context@1.0.0"),
        use_case_request_schema::<ApproverAdministrationRequestV1>(generator, "handbook.use-case.approver-administration@1.0.0"),
        use_case_request_schema::<PipelineRouteResolveAndPersistRequestV1>(generator, "handbook.use-case.pipeline-route-resolve-and-persist@1.0.0")
    ]})
});

impl_schema!(InvocationRequestV1, "InvocationRequestV1", |generator| {
    let allowed_operations = json_schema!({"allOf": [
        generator.subschema_for::<OperationRequestV1>(),
        {
            "properties": {
                "schema": {
                    "properties": {
                        "ref": {"const": "handbook.operation-request@1.0.0"}
                    },
                    "required": ["ref"]
                },
                "operation": {
                    "properties": {
                        "ref": {"enum": [
                            "artifact.kind.list@1.0.0",
                            "artifact.instance.list@1.0.0",
                            "artifact.read@1.0.0",
                            "artifact.validate@1.0.0",
                            "intake.definition.read@1.0.0",
                            "intake.coverage.evaluate@1.0.0",
                            "intake.record.append@1.0.0",
                            "artifact.candidate.validate@1.0.0",
                            "artifact.candidate.append@1.0.0",
                            "artifact.candidate.promote@1.0.0",
                            "flow.resolve@1.0.0",
                            "pipeline.catalog.list@1.0.0",
                            "pipeline.catalog.read@1.0.0",
                            "pipeline.compile@1.0.0",
                            "pipeline.capture.plan@1.0.0",
                            "pipeline.capture.apply@1.0.0",
                            "pipeline.handoff.emit@1.0.0",
                            "pipeline.state.apply@1.0.0",
                            "posture.transition.apply@1.0.0"
                        ]}
                    },
                    "required": ["ref"]
                }
            },
            "required": ["schema", "operation"]
        }
    ]});
    let operation = closed_object(
        vec![
            (
                "schema",
                exact_binding_schema(generator, "handbook.invocation-request@1.0.0"),
            ),
            ("request_kind", json_schema!({"const": "operation"})),
            ("request", allowed_operations),
            (
                "request_fingerprint",
                generator.subschema_for::<Fingerprint>(),
            ),
        ],
        &["schema", "request_kind", "request", "request_fingerprint"],
    );
    let use_case = closed_object(
        vec![
            (
                "schema",
                exact_binding_schema(generator, "handbook.invocation-request@1.0.0"),
            ),
            ("request_kind", json_schema!({"const": "use_case"})),
            ("request", generator.subschema_for::<UseCaseRequestV1>()),
            (
                "request_fingerprint",
                generator.subschema_for::<Fingerprint>(),
            ),
        ],
        &["schema", "request_kind", "request", "request_fingerprint"],
    );
    json_schema!({"oneOf": [operation, use_case]})
});

impl_schema!(
    BootstrapFacadeResponseV1,
    "BootstrapFacadeResponseV1",
    |generator| {
        let ok_response = json_schema!({"allOf": [
            generator.subschema_for::<BootstrapResponseV1>(),
            embedded_schema_ref_overlay("handbook.bootstrap-response@1.0.0")
        ]});
        let ok = closed_object(
            vec![
                (
                    "schema",
                    exact_binding_schema(generator, "handbook.bootstrap-facade-response@1.0.0"),
                ),
                ("outcome_kind", json_schema!({"const": "ok"})),
                ("response", ok_response),
                (
                    "response_fingerprint",
                    generator.subschema_for::<Fingerprint>(),
                ),
            ],
            &["schema", "outcome_kind", "response", "response_fingerprint"],
        );
        let refused_response = json_schema!({"allOf": [
            generator.subschema_for::<BootstrapRefusalV1>(),
            embedded_schema_ref_overlay("handbook.bootstrap-refusal@1.0.0")
        ]});
        let refused = closed_object(
            vec![
                (
                    "schema",
                    exact_binding_schema(generator, "handbook.bootstrap-facade-response@1.0.0"),
                ),
                ("outcome_kind", json_schema!({"const": "refused"})),
                ("response", refused_response),
                (
                    "response_fingerprint",
                    generator.subschema_for::<Fingerprint>(),
                ),
            ],
            &["schema", "outcome_kind", "response", "response_fingerprint"],
        );
        json_schema!({"oneOf": [ok, refused]})
    }
);

impl_schema!(CatalogReadRequestV1, "CatalogReadRequestV1", |generator| {
    closed_object(
        vec![
            (
                "schema",
                exact_binding_schema(generator, "handbook.catalog-read-request@1.0.0"),
            ),
            (
                "catalog_kind",
                string_enum(&["capabilities", "schemas", "profiles"]),
            ),
            ("page_size", generator.subschema_for::<CatalogPageSize>()),
            ("cursor", nullable::<TypedCatalogCursor>(generator)),
            (
                "request_fingerprint",
                generator.subschema_for::<Fingerprint>(),
            ),
        ],
        &[
            "schema",
            "catalog_kind",
            "page_size",
            "cursor",
            "request_fingerprint",
        ],
    )
});

fn embedded_schema_ref_overlay(schema_ref: &'static str) -> Schema {
    json_schema!({
        "properties": {
            "schema": {
                "properties": {"ref": {"const": schema_ref}},
                "required": ["ref"]
            }
        },
        "required": ["schema"]
    })
}

fn catalog_family_overlay(schema_ref: &'static str, catalog_ref: &'static str) -> Schema {
    json_schema!({
        "properties": {
            "schema": {
                "properties": {"ref": {"const": schema_ref}},
                "required": ["ref"]
            },
            "root": {
                "properties": {
                    "catalog": {
                        "properties": {"ref": {"const": catalog_ref}},
                        "required": ["ref"]
                    }
                },
                "required": ["catalog"]
            }
        },
        "required": ["schema", "root"]
    })
}

fn catalog_blocked_overlay(catalog_ref: &'static str) -> Schema {
    json_schema!({
        "properties": {
            "schema": {
                "properties": {"ref": {"const": "handbook.catalog-read-blocked@1.0.0"}},
                "required": ["ref"]
            },
            "catalog": {
                "properties": {"ref": {"const": catalog_ref}},
                "required": ["ref"]
            }
        },
        "required": ["schema", "catalog"]
    })
}

fn catalog_response_schema(
    generator: &mut SchemaGenerator,
    catalog_kind: &'static str,
    outcome_kind: &'static str,
    response: Schema,
) -> Schema {
    closed_object(
        vec![
            (
                "schema",
                exact_binding_schema(generator, "handbook.catalog-read-response@1.0.0"),
            ),
            ("catalog_kind", json_schema!({"const": catalog_kind})),
            ("outcome_kind", json_schema!({"const": outcome_kind})),
            ("response", response),
            (
                "response_fingerprint",
                generator.subschema_for::<Fingerprint>(),
            ),
        ],
        &[
            "schema",
            "catalog_kind",
            "outcome_kind",
            "response",
            "response_fingerprint",
        ],
    )
}

impl_schema!(
    CatalogReadResponseV1,
    "CatalogReadResponseV1",
    |generator| {
        let families = [
            (
                "capabilities",
                "handbook.capability-catalog-page@1.0.0",
                "handbook.catalog.operations@1.0.0",
                generator.subschema_for::<CapabilityCatalogPage>(),
            ),
            (
                "schemas",
                "handbook.schema-catalog-page@1.0.0",
                "handbook.catalog.schemas@1.0.0",
                generator.subschema_for::<SchemaCatalogPage>(),
            ),
            (
                "profiles",
                "handbook.profile-catalog-page@1.0.0",
                "handbook.catalog.profiles@1.0.0",
                generator.subschema_for::<ProfileCatalogPage>(),
            ),
        ];
        let mut variants = Vec::new();
        for (catalog_kind, schema_ref, catalog_ref, page) in families {
            variants.push(catalog_response_schema(
                generator,
                catalog_kind,
                "page",
                json_schema!({"allOf": [page, catalog_family_overlay(schema_ref, catalog_ref)]}),
            ));
            variants.push(catalog_response_schema(
                generator,
                catalog_kind,
                "cursor_mismatch",
                json_schema!({"type": "null"}),
            ));
            let blocked = generator.subschema_for::<CatalogReadBlockedV1>();
            variants.push(catalog_response_schema(
                generator,
                catalog_kind,
                "blocked",
                json_schema!({"allOf": [
                    blocked,
                    catalog_blocked_overlay(catalog_ref)
                ]}),
            ));
        }
        json_schema!({"oneOf": variants})
    }
);

fn typed_problem_schema<T: JsonSchema>(
    generator: &mut SchemaGenerator,
    subject_ref: &'static str,
    details_ref: &'static str,
) -> Schema {
    let problem = generator.subschema_for::<Problem>();
    let subject = exact_binding_schema(generator, subject_ref);
    let details_schema = exact_binding_schema(generator, details_ref);
    let details = generator.subschema_for::<T>();
    json_schema!({
        "allOf": [
            problem,
            {
                "properties": {
                    "subject": subject,
                    "details_schema": details_schema,
                    "details": details
                },
                "required": ["subject", "details_schema", "details"]
            }
        ]
    })
}

fn terminal_problem_array(problem: Schema) -> Schema {
    json_schema!({"type": "array", "items": problem, "minItems": 1, "maxItems": 128})
}

fn use_case_terminal_schema<T: JsonSchema>(
    generator: &mut SchemaGenerator,
    use_case_ref: &'static str,
    result_ref: &'static str,
    status: &'static str,
    problem: Option<Schema>,
) -> Schema {
    let (data, blockers, refusals, errors) = match status {
        "ok" => (
            generator.subschema_for::<T>(),
            json_schema!({"type": "array", "maxItems": 0}),
            json_schema!({"type": "array", "maxItems": 0}),
            json_schema!({"type": "array", "maxItems": 0}),
        ),
        "blocked" => (
            json_schema!({"type": "null"}),
            terminal_problem_array(problem.expect("blocked use case has typed details")),
            json_schema!({"type": "array", "maxItems": 0}),
            json_schema!({"type": "array", "maxItems": 0}),
        ),
        "refused" => (
            json_schema!({"type": "null"}),
            json_schema!({"type": "array", "maxItems": 0}),
            terminal_problem_array(problem.expect("refused use case has typed details")),
            json_schema!({"type": "array", "maxItems": 0}),
        ),
        "error" => (
            json_schema!({"type": "null"}),
            json_schema!({"type": "array", "maxItems": 0}),
            json_schema!({"type": "array", "maxItems": 0}),
            terminal_problem_array(problem.expect("error use case has typed details")),
        ),
        _ => unreachable!(),
    };
    closed_object(
        vec![
            (
                "schema",
                exact_binding_schema(generator, "handbook.use-case-response@1.0.0"),
            ),
            ("use_case", exact_binding_schema(generator, use_case_ref)),
            ("result_schema", exact_binding_schema(generator, result_ref)),
            ("status", json_schema!({"const": status})),
            ("data", data),
            ("blockers", blockers),
            ("refusals", refusals),
            ("errors", errors),
            (
                "receipts",
                generator.subschema_for::<BoundedVec<WriteReceipt, 128>>(),
            ),
            (
                "diagnostics",
                generator.subschema_for::<BoundedVec<Diagnostic, 128>>(),
            ),
            (
                "next_actions",
                generator.subschema_for::<BoundedVec<NextAction, 128>>(),
            ),
            (
                "result_fingerprint",
                generator.subschema_for::<Fingerprint>(),
            ),
            (
                "response_fingerprint",
                generator.subschema_for::<Fingerprint>(),
            ),
        ],
        &[
            "schema",
            "use_case",
            "result_schema",
            "status",
            "data",
            "blockers",
            "refusals",
            "errors",
            "receipts",
            "diagnostics",
            "next_actions",
            "result_fingerprint",
            "response_fingerprint",
        ],
    )
}

fn use_case_ok_schema<T: JsonSchema>(
    generator: &mut SchemaGenerator,
    use_case_ref: &'static str,
    result_ref: &'static str,
) -> Schema {
    use_case_terminal_schema::<T>(generator, use_case_ref, result_ref, "ok", None)
}

fn use_case_readiness_ok_schema<T: JsonSchema>(
    generator: &mut SchemaGenerator,
    use_case_ref: &'static str,
    result_ref: &'static str,
) -> Schema {
    let base = use_case_ok_schema::<T>(generator, use_case_ref, result_ref);
    json_schema!({
        "allOf": [
            base,
            {
                "properties": {
                    "data": {
                        "properties": {"readiness": {"const": "ready"}},
                        "required": ["readiness"]
                    }
                },
                "required": ["data"]
            }
        ]
    })
}

impl_schema!(UseCaseResponseV1, "UseCaseResponseV1", |generator| {
    let mut variants = vec![use_case_readiness_ok_schema::<RepositorySetupResultV1>(
        generator,
        "handbook.use-case.repository-setup@1.0.0",
        "handbook.use-case.repository-setup-result@1.0.0",
    )];
    for status in ["blocked", "refused", "error"] {
        let problem = typed_problem_schema::<RepositorySetupDetailsV1>(
            generator,
            "handbook.use-case.repository-setup@1.0.0",
            "handbook.use-case.repository-setup-details@1.0.0",
        );
        variants.push(use_case_terminal_schema::<RepositorySetupResultV1>(
            generator,
            "handbook.use-case.repository-setup@1.0.0",
            "handbook.use-case.repository-setup-result@1.0.0",
            status,
            Some(problem),
        ));
    }
    variants.push(use_case_readiness_ok_schema::<RepositoryDoctorResultV1>(
        generator,
        "handbook.use-case.repository-doctor@1.0.0",
        "handbook.use-case.repository-doctor-result@1.0.0",
    ));
    variants.push(use_case_ok_schema::<AuthorCharterResultV1>(
        generator,
        "handbook.use-case.author-charter@1.0.0",
        "handbook.use-case.author-charter-result@1.0.0",
    ));
    for status in ["blocked", "refused", "error"] {
        let problem = typed_problem_schema::<AuthorCharterDetailsV1>(
            generator,
            "handbook.use-case.author-charter@1.0.0",
            "handbook.use-case.author-charter-details@1.0.0",
        );
        variants.push(use_case_terminal_schema::<AuthorCharterResultV1>(
            generator,
            "handbook.use-case.author-charter@1.0.0",
            "handbook.use-case.author-charter-result@1.0.0",
            status,
            Some(problem),
        ));
    }
    variants.push(use_case_ok_schema::<AuthorProjectContextResultV1>(
        generator,
        "handbook.use-case.author-project-context@1.0.0",
        "handbook.use-case.author-project-context-result@1.0.0",
    ));
    for status in ["blocked", "refused", "error"] {
        let problem = typed_problem_schema::<AuthorProjectContextDetailsV1>(
            generator,
            "handbook.use-case.author-project-context@1.0.0",
            "handbook.use-case.author-project-context-details@1.0.0",
        );
        variants.push(use_case_terminal_schema::<AuthorProjectContextResultV1>(
            generator,
            "handbook.use-case.author-project-context@1.0.0",
            "handbook.use-case.author-project-context-result@1.0.0",
            status,
            Some(problem),
        ));
    }
    variants.push(use_case_ok_schema::<ApproverAdministrationResultV1>(
        generator,
        "handbook.use-case.approver-administration@1.0.0",
        "handbook.use-case.approver-administration-result@1.0.0",
    ));
    for status in ["blocked", "refused", "error"] {
        let problem = typed_problem_schema::<ApproverAdministrationDetailsV1>(
            generator,
            "handbook.use-case.approver-administration@1.0.0",
            "handbook.use-case.approver-administration-details@1.0.0",
        );
        variants.push(use_case_terminal_schema::<ApproverAdministrationResultV1>(
            generator,
            "handbook.use-case.approver-administration@1.0.0",
            "handbook.use-case.approver-administration-result@1.0.0",
            status,
            Some(problem),
        ));
    }
    variants.push(
        use_case_ok_schema::<PipelineRouteResolveAndPersistResultV1>(
            generator,
            "handbook.use-case.pipeline-route-resolve-and-persist@1.0.0",
            "handbook.use-case.pipeline-route-resolve-and-persist-result@1.0.0",
        ),
    );
    for status in ["blocked", "refused", "error"] {
        let problem = typed_problem_schema::<PipelineRouteResolveDetailsV1>(
            generator,
            "handbook.use-case.pipeline-route-resolve-and-persist@1.0.0",
            "handbook.use-case.pipeline-route-resolve-and-persist-details@1.0.0",
        );
        variants.push(use_case_terminal_schema::<
            PipelineRouteResolveAndPersistResultV1,
        >(
            generator,
            "handbook.use-case.pipeline-route-resolve-and-persist@1.0.0",
            "handbook.use-case.pipeline-route-resolve-and-persist-result@1.0.0",
            status,
            Some(problem),
        ));
    }
    for status in ["blocked", "refused", "error"] {
        let problem = typed_problem_schema::<RepositoryDoctorDetailsV1>(
            generator,
            "handbook.use-case.repository-doctor@1.0.0",
            "handbook.use-case.repository-doctor-details@1.0.0",
        );
        variants.push(use_case_terminal_schema::<RepositoryDoctorResultV1>(
            generator,
            "handbook.use-case.repository-doctor@1.0.0",
            "handbook.use-case.repository-doctor-result@1.0.0",
            status,
            Some(problem),
        ));
    }
    json_schema!({"oneOf": variants})
});

fn transport_problem_schema(
    generator: &mut SchemaGenerator,
    failure_kind: &'static str,
    category: &'static str,
    retry: &'static str,
    recheck_required: bool,
) -> Schema {
    let problem = generator.subschema_for::<Problem>();
    let subject = exact_binding_schema(generator, "handbook.transport-failure@1.0.0");
    let details_schema =
        exact_binding_schema(generator, "handbook.transport-failure-details@1.0.0");
    let details = json_schema!({
        "allOf": [
            generator.subschema_for::<TransportFailureDetailsV1>(),
            {
                "properties": {
                    "failure_kind": {"const": failure_kind}
                },
                "required": ["failure_kind"]
            }
        ]
    });
    let recheck = if recheck_required {
        generator.subschema_for::<RecheckCondition>()
    } else {
        json_schema!({"type": "null"})
    };
    let rule = if category == "precondition" {
        exact_binding_schema(generator, "handbook.transport-failure-rule@1.0.0")
    } else {
        json_schema!({"type": "null"})
    };
    let correlation_ref = if category == "adapter" {
        generator.subschema_for::<CorrelationRef>()
    } else {
        json_schema!({"type": "null"})
    };
    json_schema!({
        "allOf": [
            problem,
            {
                "properties": {
                    "category": {"const": category},
                    "subject": subject,
                    "rule": rule,
                    "details_schema": details_schema,
                    "details": details,
                    "retry": {"const": retry},
                    "recheck": recheck,
                    "correlation_ref": correlation_ref
                },
                "required": ["category", "subject", "rule", "details_schema", "details", "retry", "recheck", "correlation_ref"]
            }
        ]
    })
}

impl_schema!(TransportFailureV1, "TransportFailureV1", |generator| {
    let mut variants = Vec::new();
    for (failure_kind, status, category, retry, recheck_required) in [
        (
            "repository",
            "blocked",
            "prerequisite",
            "after_recheck",
            true,
        ),
        (
            "acquisition",
            "refused",
            "precondition",
            "after_request_change",
            false,
        ),
        (
            "decoding",
            "refused",
            "precondition",
            "after_request_change",
            false,
        ),
        ("framing", "error", "adapter", "never", false),
        ("serialization", "error", "adapter", "never", false),
        ("emission", "error", "adapter", "never", false),
        ("adapter", "error", "adapter", "never", false),
    ] {
        let problem =
            transport_problem_schema(generator, failure_kind, category, retry, recheck_required);
        let (blockers, refusals, errors) = match status {
            "blocked" => (
                terminal_problem_array(problem),
                json_schema!({"type": "array", "maxItems": 0}),
                json_schema!({"type": "array", "maxItems": 0}),
            ),
            "refused" => (
                json_schema!({"type": "array", "maxItems": 0}),
                terminal_problem_array(problem),
                json_schema!({"type": "array", "maxItems": 0}),
            ),
            "error" => (
                json_schema!({"type": "array", "maxItems": 0}),
                json_schema!({"type": "array", "maxItems": 0}),
                terminal_problem_array(problem),
            ),
            _ => unreachable!(),
        };
        variants.push(closed_object(
            vec![
                (
                    "schema",
                    exact_binding_schema(generator, "handbook.transport-failure@1.0.0"),
                ),
                ("failure_kind", json_schema!({"const": failure_kind})),
                ("status", json_schema!({"const": status})),
                ("data", json_schema!({"type": "null"})),
                ("blockers", blockers),
                ("refusals", refusals),
                ("errors", errors),
                (
                    "diagnostics",
                    generator.subschema_for::<BoundedVec<Diagnostic, 128>>(),
                ),
                (
                    "next_actions",
                    generator.subschema_for::<BoundedVec<NextAction, 128>>(),
                ),
                (
                    "response_fingerprint",
                    generator.subschema_for::<Fingerprint>(),
                ),
            ],
            &[
                "schema",
                "failure_kind",
                "status",
                "data",
                "blockers",
                "refusals",
                "errors",
                "diagnostics",
                "next_actions",
                "response_fingerprint",
            ],
        ));
    }
    json_schema!({"oneOf": variants})
});

impl_schema!(InvocationResponseV1, "InvocationResponseV1", |generator| {
    let operation = closed_object(
        vec![
            (
                "schema",
                exact_binding_schema(generator, "handbook.invocation-response@1.0.0"),
            ),
            ("response_kind", json_schema!({"const": "operation"})),
            (
                "response",
                json_schema!({"oneOf": [
                    generator.subschema_for::<OperationResponseV1>(),
                    generator.subschema_for::<OperationAdmissionRefusalV1>()
                ]}),
            ),
            (
                "response_fingerprint",
                generator.subschema_for::<Fingerprint>(),
            ),
        ],
        &[
            "schema",
            "response_kind",
            "response",
            "response_fingerprint",
        ],
    );
    let use_case = closed_object(
        vec![
            (
                "schema",
                exact_binding_schema(generator, "handbook.invocation-response@1.0.0"),
            ),
            ("response_kind", json_schema!({"const": "use_case"})),
            ("response", generator.subschema_for::<UseCaseResponseV1>()),
            (
                "response_fingerprint",
                generator.subschema_for::<Fingerprint>(),
            ),
        ],
        &[
            "schema",
            "response_kind",
            "response",
            "response_fingerprint",
        ],
    );
    let transport = closed_object(
        vec![
            (
                "schema",
                exact_binding_schema(generator, "handbook.invocation-response@1.0.0"),
            ),
            (
                "response_kind",
                json_schema!({"const": "transport_failure"}),
            ),
            ("response", generator.subschema_for::<TransportFailureV1>()),
            (
                "response_fingerprint",
                generator.subschema_for::<Fingerprint>(),
            ),
        ],
        &[
            "schema",
            "response_kind",
            "response",
            "response_fingerprint",
        ],
    );
    json_schema!({"oneOf": [operation, use_case, transport]})
});

fn flow_text(max_length: u64) -> Schema {
    json_schema!({"type": "string", "minLength": 1, "maxLength": max_length})
}

fn flow_path() -> Schema {
    json_schema!({
        "type": "string",
        "minLength": 1,
        "maxLength": 4096,
        "pattern": "^[A-Za-z0-9._-]+(/[A-Za-z0-9._-]+)*$"
    })
}

fn flow_directory_path() -> Schema {
    json_schema!({
        "type": "string",
        "minLength": 2,
        "maxLength": 4096,
        "pattern": "^[A-Za-z0-9._-]+(/[A-Za-z0-9._-]+)*/$"
    })
}

fn flow_path_array() -> Schema {
    json_schema!({
        "type": "array",
        "maxItems": 128,
        "items": flow_path()
    })
}

fn flow_array<T: JsonSchema>(generator: &mut SchemaGenerator) -> Schema {
    json_schema!({
        "type": "array",
        "maxItems": 128,
        "items": generator.subschema_for::<T>()
    })
}

fn flow_optional<T: JsonSchema>(generator: &mut SchemaGenerator) -> Schema {
    nullable::<T>(generator)
}

fn flow_optional_text(max_length: u64) -> Schema {
    json_schema!({
        "anyOf": [
            {"type":"string","minLength":1,"maxLength":max_length},
            {"type":"null"}
        ]
    })
}

fn flow_capture_input() -> Schema {
    json_schema!({"type": "string", "maxLength": 65_536})
}

fn flow_optional_capture_input() -> Schema {
    json_schema!({
        "anyOf": [
            {"type":"string","maxLength":65_536},
            {"type":"null"}
        ]
    })
}

fn flow_raw_hash() -> Schema {
    json_schema!({"type": "string", "pattern": "^[0-9a-f]{64}$"})
}

fn flow_content_hash() -> Schema {
    json_schema!({"type": "string", "pattern": "^(sha256:)?[0-9a-f]{64}$"})
}

impl_schema!(FlowPacketIdV1, "FlowPacketIdV1", |_generator| string_enum(
    &[
        "planning.packet",
        "execution.demo.packet",
        "execution.live.packet",
    ]
));
impl_schema!(
    FlowSelectionStatusV1,
    "FlowSelectionStatusV1",
    |_generator| string_enum(&["selected", "blocked"])
);
impl_schema!(
    FlowArtifactPresenceV1,
    "FlowArtifactPresenceV1",
    |_generator| string_enum(&["missing", "present_empty", "present_non_empty"])
);
impl_schema!(FlowPacketVariantV1, "FlowPacketVariantV1", |_generator| {
    string_enum(&["planning", "execution_demo", "execution_live"])
});
impl_schema!(
    FlowPacketBodyNoteKindV1,
    "FlowPacketBodyNoteKindV1",
    |_generator| string_enum(&["omission", "budget", "inherited_dependency"])
);
impl_schema!(
    FlowPacketSectionModeV1,
    "FlowPacketSectionModeV1",
    |_generator| string_enum(&["verbatim", "summary", "rendered"])
);
impl_schema!(FlowReadyActionV1, "FlowReadyActionV1", |_generator| {
    string_enum(&["inspect_proof", "generate", "run_doctor"])
});
impl_schema!(
    FlowBudgetDispositionV1,
    "FlowBudgetDispositionV1",
    |_generator| string_enum(&["keep", "summarize", "exclude", "refuse"])
);
impl_schema!(FlowBudgetReasonV1, "FlowBudgetReasonV1", |_generator| {
    string_enum(&[
        "within_budget",
        "optional_artifact_too_large",
        "total_bytes_exceeded",
        "required_artifact_too_large",
    ])
});
impl_schema!(
    FlowBudgetByteDomainV1,
    "FlowBudgetByteDomainV1",
    |_generator| string_enum(&["source", "rendered_output"])
);
impl_schema!(
    FlowRefusalCategoryV1,
    "FlowRefusalCategoryV1",
    |_generator| string_enum(&[
        "non_canonical_input_attempt",
        "system_root_missing",
        "system_root_not_dir",
        "system_root_symlink_not_allowed",
        "required_artifact_missing",
        "required_artifact_empty",
        "required_artifact_starter_template",
        "required_artifact_invalid",
        "artifact_read_error",
        "freshness_invalid",
        "budget_refused",
        "unsupported_request"
    ])
);
impl_schema!(
    FlowBlockerCategoryV1,
    "FlowBlockerCategoryV1",
    |_generator| string_enum(&[
        "system_root_missing",
        "system_root_not_dir",
        "system_root_symlink_not_allowed",
        "required_artifact_missing",
        "required_artifact_empty",
        "required_artifact_starter_template",
        "required_artifact_invalid",
        "artifact_read_error",
        "freshness_invalid",
        "budget_refused",
        "unsupported_request"
    ])
);
impl_schema!(FlowProblemCauseV1, "FlowProblemCauseV1", |_generator| {
    string_enum(&[
        "non_canonical_input_attempt",
        "system_root_missing",
        "system_root_not_dir",
        "system_root_symlink_not_allowed",
        "required_artifact_missing",
        "required_artifact_empty",
        "required_artifact_starter_template",
        "required_artifact_invalid",
        "artifact_read_error",
        "freshness_invalid",
        "budget_refused",
        "unsupported_request",
        "owner_read_failure",
        "owner_profile_decision",
    ])
});
impl_schema!(FlowProblemOriginV1, "FlowProblemOriginV1", |_generator| {
    string_enum(&["resolution", "owner_ingest", "owner_error"])
});

impl_schema!(FlowResolveRequestV1, "FlowResolveRequestV1", |generator| {
    closed_object(
        vec![("packet_id", generator.subschema_for::<FlowPacketIdV1>())],
        &["packet_id"],
    )
});

impl_schema!(FlowSubjectRefV1, "FlowSubjectRefV1", |_generator| {
    let canonical = closed_object(
        vec![
            ("kind", json_schema!({"const": "canonical_artifact"})),
            ("instance_id", flow_text(255)),
            ("kind_ref", flow_text(511)),
            ("label", flow_text(1024)),
            ("canonical_repo_relative_path", flow_path()),
        ],
        &[
            "kind",
            "instance_id",
            "kind_ref",
            "label",
            "canonical_repo_relative_path",
        ],
    );
    let dependency = closed_object(
        vec![
            ("kind", json_schema!({"const": "inherited_dependency"})),
            ("dependency_id", flow_text(511)),
            (
                "version",
                json_schema!({"anyOf": [flow_text(255), {"type": "null"}]}),
            ),
        ],
        &["kind", "dependency_id", "version"],
    );
    let policy = closed_object(
        vec![
            ("kind", json_schema!({"const": "policy"})),
            (
                "policy_id",
                string_enum(&["system_root", "freshness", "budget", "packet_id"]),
            ),
        ],
        &["kind", "policy_id"],
    );
    json_schema!({"oneOf": [canonical, dependency, policy]})
});

impl_schema!(FlowNextSafeActionV1, "FlowNextSafeActionV1", |generator| {
    let unit = |kind| closed_object(vec![("kind", json_schema!({"const": kind}))], &["kind"]);
    let path = |kind| {
        closed_object(
            vec![
                ("kind", json_schema!({"const": kind})),
                ("canonical_repo_relative_path", flow_path()),
            ],
            &["kind", "canonical_repo_relative_path"],
        )
    };
    json_schema!({"oneOf": [
        unit("run_setup"), unit("run_setup_init"), unit("run_setup_refresh"),
        unit("run_author_charter"), unit("run_author_project_context"),
        path("create_system_root"), path("ensure_system_root_is_directory"),
        path("remove_system_root_symlink"), path("create_canonical_artifact"),
        path("fill_canonical_artifact"), path("reduce_canonical_artifact_size"),
        closed_object(vec![
            ("kind", json_schema!({"const": "run_generate"})),
            ("packet_id", generator.subschema_for::<FlowPacketIdV1>()),
        ], &["kind", "packet_id"]),
        unit("run_doctor")
    ]})
});

impl_schema!(
    FlowBudgetNextSafeActionV1,
    "FlowBudgetNextSafeActionV1",
    |_generator| closed_object(
        vec![
            (
                "kind",
                json_schema!({"const": "reduce_canonical_artifact_size"})
            ),
            ("canonical_repo_relative_path", flow_path()),
        ],
        &["kind", "canonical_repo_relative_path"],
    )
);

impl_schema!(FlowPacketSourceV1, "FlowPacketSourceV1", |generator| {
    closed_object(
        vec![
            ("instance_id", flow_text(255)),
            ("kind_ref", flow_text(511)),
            ("label", flow_text(1024)),
            ("canonical_repo_relative_path", flow_path()),
            ("required", json_schema!({"type": "boolean"})),
            (
                "presence",
                generator.subschema_for::<FlowArtifactPresenceV1>(),
            ),
            (
                "byte_len",
                json_schema!({"anyOf": [{"type": "integer", "minimum": 0, "maximum": MAX_SAFE_INTEGER}, {"type": "null"}]}),
            ),
            (
                "content_sha256",
                json_schema!({"anyOf": [flow_content_hash(), {"type": "null"}]}),
            ),
            (
                "rendered_output_byte_len",
                json_schema!({"anyOf": [{"type": "integer", "minimum": 0, "maximum": MAX_SAFE_INTEGER}, {"type": "null"}]}),
            ),
            (
                "rendered_output_sha256",
                json_schema!({"anyOf": [flow_content_hash(), {"type": "null"}]}),
            ),
            (
                "rendered_media_type",
                json_schema!({"anyOf": [flow_text(255), {"type": "null"}]}),
            ),
        ],
        &[
            "instance_id",
            "kind_ref",
            "label",
            "canonical_repo_relative_path",
            "required",
            "presence",
            "byte_len",
            "content_sha256",
            "rendered_output_byte_len",
            "rendered_output_sha256",
            "rendered_media_type",
        ],
    )
});

impl_schema!(FlowPacketBodyNoteV1, "FlowPacketBodyNoteV1", |generator| {
    closed_object(
        vec![
            (
                "kind",
                generator.subschema_for::<FlowPacketBodyNoteKindV1>(),
            ),
            ("text", flow_text(8192)),
        ],
        &["kind", "text"],
    )
});
impl_schema!(FlowPacketSectionV1, "FlowPacketSectionV1", |generator| {
    closed_object(
        vec![
            ("instance_id", flow_text(255)),
            ("kind_ref", flow_text(511)),
            ("label", flow_text(1024)),
            ("canonical_repo_relative_path", flow_path()),
            ("title", flow_text(1024)),
            ("mode", generator.subschema_for::<FlowPacketSectionModeV1>()),
            ("contents", flow_text(1_048_576)),
            (
                "source_content_sha256",
                json_schema!({"anyOf": [flow_content_hash(), {"type": "null"}]}),
            ),
            (
                "rendered_output_sha256",
                json_schema!({"anyOf": [flow_content_hash(), {"type": "null"}]}),
            ),
        ],
        &[
            "instance_id",
            "kind_ref",
            "label",
            "canonical_repo_relative_path",
            "title",
            "mode",
            "contents",
            "source_content_sha256",
            "rendered_output_sha256",
        ],
    )
});

impl_schema!(FlowFixtureContextV1, "FlowFixtureContextV1", |generator| {
    closed_object(
        vec![
            ("fixture_set_id", flow_text(255)),
            ("fixture_basis_root", flow_directory_path()),
            (
                "fixture_lineage",
                flow_array::<FlowPacketSourceV1>(generator),
            ),
        ],
        &["fixture_set_id", "fixture_basis_root", "fixture_lineage"],
    )
});
impl_schema!(
    FlowDecisionSummaryV1,
    "FlowDecisionSummaryV1",
    |generator| closed_object(
        vec![
            (
                "packet_status",
                generator.subschema_for::<FlowSelectionStatusV1>()
            ),
            (
                "budget_disposition",
                generator.subschema_for::<FlowBudgetDispositionV1>()
            ),
            (
                "budget_reason",
                generator.subschema_for::<FlowBudgetReasonV1>()
            ),
            (
                "decision_log_entries",
                json_schema!({"type": "integer", "minimum": 0, "maximum": MAX_SAFE_INTEGER})
            ),
            ("summary_line", flow_text(8192)),
            (
                "ready_next_safe_action",
                generator.subschema_for::<FlowReadyActionV1>()
            ),
        ],
        &[
            "packet_status",
            "budget_disposition",
            "budget_reason",
            "decision_log_entries",
            "summary_line",
            "ready_next_safe_action"
        ]
    )
);
impl_schema!(FlowPacketResultV1, "FlowPacketResultV1", |generator| {
    closed_object(
        vec![
            ("packet_id", generator.subschema_for::<FlowPacketIdV1>()),
            ("variant", generator.subschema_for::<FlowPacketVariantV1>()),
            (
                "fixture_context",
                flow_optional::<FlowFixtureContextV1>(generator),
            ),
            (
                "included_sources",
                flow_array::<FlowPacketSourceV1>(generator),
            ),
            ("notes", flow_array::<FlowPacketBodyNoteV1>(generator)),
            (
                "decision_summary",
                generator.subschema_for::<FlowDecisionSummaryV1>(),
            ),
            ("sections", flow_array::<FlowPacketSectionV1>(generator)),
        ],
        &[
            "packet_id",
            "variant",
            "fixture_context",
            "included_sources",
            "notes",
            "decision_summary",
            "sections",
        ],
    )
});

impl_schema!(FlowBudgetTargetV1, "FlowBudgetTargetV1", |generator| {
    closed_object(
        vec![
            ("canonical_repo_relative_path", flow_path()),
            (
                "byte_len",
                json_schema!({"type": "integer", "minimum": 0, "maximum": MAX_SAFE_INTEGER}),
            ),
            (
                "byte_domain",
                generator.subschema_for::<FlowBudgetByteDomainV1>(),
            ),
        ],
        &["canonical_repo_relative_path", "byte_len", "byte_domain"],
    )
});
impl_schema!(FlowBudgetOutcomeV1, "FlowBudgetOutcomeV1", |generator| {
    closed_object(
        vec![
            (
                "disposition",
                generator.subschema_for::<FlowBudgetDispositionV1>(),
            ),
            ("reason", generator.subschema_for::<FlowBudgetReasonV1>()),
            ("targets", flow_array::<FlowBudgetTargetV1>(generator)),
            (
                "next_safe_action",
                flow_optional::<FlowBudgetNextSafeActionV1>(generator),
            ),
        ],
        &["disposition", "reason", "targets", "next_safe_action"],
    )
});
impl_schema!(FlowSelectionV1, "FlowSelectionV1", |generator| {
    closed_object(
        vec![
            ("packet_id", generator.subschema_for::<FlowPacketIdV1>()),
            ("status", generator.subschema_for::<FlowSelectionStatusV1>()),
        ],
        &["packet_id", "status"],
    )
});
impl_schema!(FlowRefusalV1, "FlowRefusalV1", |generator| closed_object(
    vec![
        (
            "category",
            generator.subschema_for::<FlowRefusalCategoryV1>()
        ),
        ("summary", flow_text(8192)),
        (
            "broken_subject",
            generator.subschema_for::<FlowSubjectRefV1>()
        ),
        (
            "next_safe_action",
            generator.subschema_for::<FlowNextSafeActionV1>()
        ),
    ],
    &["category", "summary", "broken_subject", "next_safe_action"]
));
impl_schema!(FlowBlockerV1, "FlowBlockerV1", |generator| closed_object(
    vec![
        (
            "category",
            generator.subschema_for::<FlowBlockerCategoryV1>()
        ),
        ("subject", generator.subschema_for::<FlowSubjectRefV1>()),
        ("summary", flow_text(8192)),
        (
            "next_safe_action",
            generator.subschema_for::<FlowNextSafeActionV1>()
        ),
    ],
    &["category", "subject", "summary", "next_safe_action"]
));

impl_schema!(FlowResolveResultV1, "FlowResolveResultV1", |generator| {
    closed_object(
        vec![
            ("c04_result_version", flow_text(255)),
            ("c03_schema_version", flow_text(255)),
            (
                "c03_manifest_generation_version",
                json_schema!({"type": "integer", "minimum": 0, "maximum": MAX_SAFE_INTEGER}),
            ),
            ("c03_fingerprint_sha256", flow_raw_hash()),
            (
                "packet_result",
                generator.subschema_for::<FlowPacketResultV1>(),
            ),
            (
                "decision_log_entries",
                json_schema!({"type": "array", "maxItems": 128, "items": flow_text(8192)}),
            ),
            (
                "budget_outcome",
                generator.subschema_for::<FlowBudgetOutcomeV1>(),
            ),
            ("selection", generator.subschema_for::<FlowSelectionV1>()),
            ("refusal", flow_optional::<FlowRefusalV1>(generator)),
            ("blockers", flow_array::<FlowBlockerV1>(generator)),
        ],
        &[
            "c04_result_version",
            "c03_schema_version",
            "c03_manifest_generation_version",
            "c03_fingerprint_sha256",
            "packet_result",
            "decision_log_entries",
            "budget_outcome",
            "selection",
            "refusal",
            "blockers",
        ],
    )
});

impl_schema!(FlowProblemDetailsV1, "FlowProblemDetailsV1", |generator| {
    closed_object(
        vec![
            ("origin", generator.subschema_for::<FlowProblemOriginV1>()),
            ("cause", generator.subschema_for::<FlowProblemCauseV1>()),
            ("detail", flow_text(4096)),
            (
                "resolution",
                flow_optional::<FlowResolveResultV1>(generator),
            ),
        ],
        &["origin", "cause", "detail", "resolution"],
    )
});
impl_schema!(FlowRecheckV1, "FlowRecheckV1", |generator| closed_object(
    vec![("cause", generator.subschema_for::<FlowProblemCauseV1>())],
    &["cause"],
));

macro_rules! pipeline_string_enum {
    ($ty:ty, $name:literal, [$($value:literal),+ $(,)?]) => {
        impl_schema!($ty, $name, |_generator| string_enum(&[$($value),+]));
    };
}

pipeline_string_enum!(
    PipelineActivationOperatorV1,
    "PipelineActivationOperatorV1",
    ["any", "all"]
);
pipeline_string_enum!(
    PipelineOperationRouteStageStatusV1,
    "PipelineOperationRouteStageStatusV1",
    ["active", "skipped", "blocked", "next"]
);
pipeline_string_enum!(
    PipelineCompileDocumentKindV1,
    "PipelineCompileDocumentKindV1",
    ["include", "runner", "profile", "library", "artifact"]
);
pipeline_string_enum!(
    PipelineCompileDocumentStatusV1,
    "PipelineCompileDocumentStatusV1",
    ["present", "missing_optional"]
);
pipeline_string_enum!(
    PipelineCompileOutputKindV1,
    "PipelineCompileOutputKindV1",
    ["artifact", "repository_file"]
);
pipeline_string_enum!(
    PipelineCompileRefusalClassificationV1,
    "PipelineCompileRefusalClassificationV1",
    [
        "unsupported_target",
        "invalid_definition",
        "invalid_state",
        "missing_route_basis",
        "malformed_route_basis",
        "stale_route_basis",
        "inactive_stage",
        "missing_required_input",
        "empty_required_input"
    ]
);
pipeline_string_enum!(
    PipelineProblemCauseV1,
    "PipelineProblemCauseV1",
    [
        "catalog_read",
        "catalog_invalid",
        "unsupported_selector",
        "ambiguous_selector",
        "unknown_selector",
        "unsupported_compile_target",
        "invalid_compile_definition",
        "invalid_route_state",
        "missing_route_basis",
        "malformed_route_basis",
        "stale_route_basis",
        "inactive_stage",
        "missing_required_input",
        "empty_required_input",
        "compile_result_bound_exceeded",
        "capture_unsupported_target",
        "capture_invalid_definition",
        "capture_invalid_state",
        "capture_missing_route_basis",
        "capture_malformed_route_basis",
        "capture_stale_route_basis",
        "capture_inactive_stage",
        "capture_invalid_input",
        "capture_invalid_write_target",
        "capture_missing_id",
        "capture_tampered_cache",
        "capture_revision_conflict",
        "capture_write_failure",
        "capture_state_persistence_failure",
        "capture_cache_failure",
        "handoff_unsupported_target",
        "handoff_invalid_state",
        "handoff_missing_input",
        "handoff_invalid_provenance",
        "handoff_write_failure",
        "state_malformed",
        "state_unsupported_variable",
        "state_revision_conflict",
        "state_invalid_pipeline_id",
        "state_invalid_supported_variables",
        "state_invalid_mutation",
        "state_read_failure",
        "state_lock_failure",
        "state_write_failure",
        "state_serialization_failure",
        "idempotency_invalid_operation",
        "idempotency_lock_failure",
        "idempotency_read_failure",
        "idempotency_malformed_entry",
        "idempotency_write_failure",
        "idempotency_conflict"
    ]
);

impl_schema!(
    PipelineEmptyRequestV1,
    "PipelineEmptyRequestV1",
    |_generator| closed_object(vec![], &[])
);
impl_schema!(
    PipelineCapturePlanRequestV1,
    "PipelineCapturePlanRequestV1",
    |_generator| closed_object(
        vec![
            ("pipeline_selector", flow_text(4096)),
            ("stage_selector", flow_text(4096)),
            ("input", flow_capture_input()),
        ],
        &["pipeline_selector", "stage_selector", "input"],
    )
);
impl_schema!(
    PipelineCaptureApplyModeV1,
    "PipelineCaptureApplyModeV1",
    |_generator| string_enum(&["direct", "cached"])
);
impl_schema!(
    PipelineCaptureApplyRequestSchemaV1,
    "PipelineCaptureApplyRequestSchemaV1",
    |generator| closed_object(
        vec![
            (
                "idempotency_key",
                json_schema!({"type":"string","minLength":1,"maxLength":256,"pattern":"^[!-~]{1,256}$"}),
            ),
            (
                "mode",
                generator.subschema_for::<PipelineCaptureApplyModeV1>()
            ),
            (
                "capture_id",
                json_schema!({"anyOf":[{"type":"string","pattern":"^[0-9a-f]{64}$"},{"type":"null"}]}),
            ),
            ("pipeline_selector", flow_optional_text(4096)),
            ("stage_selector", flow_optional_text(4096)),
            ("input", flow_optional_capture_input()),
        ],
        &[
            "idempotency_key",
            "mode",
            "capture_id",
            "pipeline_selector",
            "stage_selector",
            "input",
        ],
    )
);
impl_schema!(
    PipelineHandoffEmitRequestSchemaV1,
    "PipelineHandoffEmitRequestSchemaV1",
    |_generator| closed_object(
        vec![
            (
                "idempotency_key",
                json_schema!({"type":"string","minLength":1,"maxLength":256,"pattern":"^[!-~]{1,256}$"}),
            ),
            ("pipeline_selector", flow_text(4096)),
            ("consumer_selector", flow_text(4096)),
            ("producer_command", flow_text(4096)),
            ("producer_version", flow_text(255)),
        ],
        &[
            "idempotency_key",
            "pipeline_selector",
            "consumer_selector",
            "producer_command",
            "producer_version",
        ],
    )
);
impl_schema!(
    PipelineStateMutationV1,
    "PipelineStateMutationV1",
    |_generator| {
        let bool_variant = closed_object(
            vec![
                ("mutation_kind", json_schema!({"const":"routing_variable"})),
                ("variable", flow_text(255)),
                ("value", json_schema!({"type":"boolean"})),
            ],
            &["mutation_kind", "variable", "value"],
        );
        let text_variant = |kind: &'static str| {
            closed_object(
                vec![
                    ("mutation_kind", json_schema!({"const":kind})),
                    ("value", flow_text(4096)),
                ],
                &["mutation_kind", "value"],
            )
        };
        json_schema!({"oneOf":[
            bool_variant,
            text_variant("charter_reference"),
            text_variant("project_context_reference"),
            text_variant("runner"),
            text_variant("profile")
        ]})
    }
);
impl_schema!(
    PipelineStateApplyRequestSchemaV1,
    "PipelineStateApplyRequestSchemaV1",
    |generator| closed_object(
        vec![
            (
                "idempotency_key",
                json_schema!({"type":"string","minLength":1,"maxLength":256,"pattern":"^[!-~]{1,256}$"}),
            ),
            ("selector", flow_text(4096)),
            (
                "mutation",
                generator.subschema_for::<PipelineStateMutationV1>()
            ),
            (
                "expected_revision",
                json_schema!({"type":"integer","minimum":0,"maximum":MAX_SAFE_INTEGER - 1}),
            ),
        ],
        &[
            "idempotency_key",
            "selector",
            "mutation",
            "expected_revision"
        ],
    )
);
impl_schema!(
    PipelineCatalogReadRequestV1,
    "PipelineCatalogReadRequestV1",
    |_generator| closed_object(vec![("selector", flow_text(4096))], &["selector"])
);
impl_schema!(
    PipelineCompileRequestV1,
    "PipelineCompileRequestV1",
    |_generator| closed_object(
        vec![
            ("pipeline_selector", flow_text(4096)),
            ("stage_selector", flow_text(4096))
        ],
        &["pipeline_selector", "stage_selector"]
    )
);
impl_schema!(
    PipelineCaptureTargetDtoV1,
    "PipelineCaptureTargetDtoV1",
    |_generator| closed_object(
        vec![
            ("pipeline_id", flow_text(255)),
            ("stage_id", flow_text(255)),
            ("stage_file", flow_path()),
            ("title", flow_text(1024)),
        ],
        &["pipeline_id", "stage_id", "stage_file", "title"],
    )
);
impl_schema!(
    PipelineCaptureStateValueDtoV1,
    "PipelineCaptureStateValueDtoV1",
    |_generator| json_schema!({"oneOf":[{"type":"boolean"},flow_text(4096)]})
);
impl_schema!(
    PipelineCaptureStateUpdateDtoV1,
    "PipelineCaptureStateUpdateDtoV1",
    |generator| closed_object(
        vec![
            ("field_path", flow_text(4096)),
            (
                "value",
                generator.subschema_for::<PipelineCaptureStateValueDtoV1>(),
            ),
        ],
        &["field_path", "value"],
    )
);
impl_schema!(
    PipelineCapturePlanDtoV1,
    "PipelineCapturePlanDtoV1",
    |generator| closed_object(
        vec![
            (
                "target",
                generator.subschema_for::<PipelineCaptureTargetDtoV1>()
            ),
            (
                "route_basis_revision",
                json_schema!({"type":"integer","minimum":0,"maximum":MAX_SAFE_INTEGER}),
            ),
            ("artifact_write_paths", flow_path_array()),
            ("repository_mirror_write_paths", flow_path_array()),
            (
                "state_updates",
                flow_array::<PipelineCaptureStateUpdateDtoV1>(generator),
            ),
            ("capture_id", flow_raw_hash()),
            ("post_apply_next_safe_action", flow_optional_text(4096)),
        ],
        &[
            "target",
            "route_basis_revision",
            "artifact_write_paths",
            "repository_mirror_write_paths",
            "state_updates",
            "capture_id",
            "post_apply_next_safe_action",
        ],
    )
);
impl_schema!(
    PipelineCapturePlanResultV1,
    "PipelineCapturePlanResultV1",
    |generator| closed_object(
        vec![(
            "plan",
            generator.subschema_for::<PipelineCapturePlanDtoV1>()
        )],
        &["plan"],
    )
);
impl_schema!(
    PipelineCaptureApplyResultV1,
    "PipelineCaptureApplyResultV1",
    |generator| closed_object(
        vec![
            (
                "plan",
                generator.subschema_for::<PipelineCapturePlanDtoV1>()
            ),
            ("written_files", flow_path_array()),
            ("persisted_state_revision", nullable::<SafeU64>(generator)),
        ],
        &["plan", "written_files", "persisted_state_revision"],
    )
);
impl_schema!(
    PipelineHandoffEmitResultV1,
    "PipelineHandoffEmitResultV1",
    |_generator| closed_object(
        vec![
            ("pipeline_id", flow_text(255)),
            ("consumer_id", flow_text(255)),
            ("feature_id", flow_text(255)),
            ("bundle_root", flow_path()),
            ("written_files", flow_path_array()),
        ],
        &[
            "pipeline_id",
            "consumer_id",
            "feature_id",
            "bundle_root",
            "written_files",
        ],
    )
);
impl_schema!(
    PipelineRouteStateDtoV1,
    "PipelineRouteStateDtoV1",
    |generator| closed_object(
        vec![
            (
                "revision",
                json_schema!({"type":"integer","minimum":0,"maximum":MAX_SAFE_INTEGER}),
            ),
            (
                "routing",
                flow_array::<PipelineOperationRoutingEntryV1>(generator),
            ),
            ("charter_ref", flow_optional_text(4096)),
            ("project_context_ref", flow_optional_text(4096)),
            ("runner", flow_optional_text(4096)),
            ("profile", flow_optional_text(4096)),
        ],
        &[
            "revision",
            "routing",
            "charter_ref",
            "project_context_ref",
            "runner",
            "profile",
        ],
    )
);
impl_schema!(
    PipelineStateApplyResultV1,
    "PipelineStateApplyResultV1",
    |generator| closed_object(
        vec![
            ("pipeline_id", flow_text(255)),
            (
                "state",
                generator.subschema_for::<PipelineRouteStateDtoV1>()
            ),
        ],
        &["pipeline_id", "state"],
    )
);
impl_schema!(
    PipelineWriteEffectV1,
    "PipelineWriteEffectV1",
    |generator| closed_object(
        vec![
            ("effect_kind", flow_text(255)),
            ("portable_path", flow_path()),
            ("fingerprint", generator.subschema_for::<Fingerprint>()),
            ("revision", nullable::<SafeU64>(generator)),
        ],
        &["effect_kind", "portable_path", "fingerprint", "revision"],
    )
);
impl_schema!(
    PipelineWriteConditionV1,
    "PipelineWriteConditionV1",
    |generator| closed_object(
        vec![
            ("operation", generator.subschema_for::<ExactRef>()),
            (
                "effects",
                json_schema!({
                    "type":"array",
                    "minItems":1,
                    "maxItems":128,
                    "items":generator.subschema_for::<PipelineWriteEffectV1>()
                }),
            ),
        ],
        &["operation", "effects"],
    )
);
impl_schema!(
    PipelineCatalogEntryV1,
    "PipelineCatalogEntryV1",
    |_generator| closed_object(
        vec![
            ("id", flow_text(255)),
            ("title", flow_text(1024)),
            ("source_path", flow_path()),
            (
                "stage_count",
                json_schema!({"type":"integer","minimum":0,"maximum":MAX_SAFE_INTEGER})
            )
        ],
        &["id", "title", "source_path", "stage_count"]
    )
);
impl_schema!(
    PipelineCatalogListResultV1,
    "PipelineCatalogListResultV1",
    |generator| closed_object(
        vec![
            ("pipelines", flow_array::<PipelineCatalogEntryV1>(generator)),
            (
                "pipeline_count",
                json_schema!({"type":"integer","minimum":0,"maximum":MAX_SAFE_INTEGER})
            ),
            (
                "stage_count",
                json_schema!({"type":"integer","minimum":0,"maximum":MAX_SAFE_INTEGER})
            )
        ],
        &["pipelines", "pipeline_count", "stage_count"]
    )
);
impl_schema!(
    PipelineActivationClauseV1,
    "PipelineActivationClauseV1",
    |_generator| closed_object(
        vec![
            ("variable", flow_text(255)),
            ("value", json_schema!({"type":"boolean"}))
        ],
        &["variable", "value"]
    )
);
impl_schema!(PipelineActivationV1, "PipelineActivationV1", |generator| {
    closed_object(
        vec![
            (
                "operator",
                generator.subschema_for::<PipelineActivationOperatorV1>(),
            ),
            (
                "clauses",
                flow_array::<PipelineActivationClauseV1>(generator),
            ),
        ],
        &["operator", "clauses"],
    )
});
impl_schema!(
    PipelineDeclaredStageV1,
    "PipelineDeclaredStageV1",
    |generator| closed_object(
        vec![
            ("id", flow_text(255)),
            ("title", flow_text(1024)),
            ("source_path", flow_path()),
            ("work_level", flow_optional::<String>(generator)),
            (
                "sets",
                json_schema!({"anyOf":[{"type":"array","maxItems":128,"items":flow_text(255)},{"type":"null"}]})
            ),
            (
                "activation",
                flow_optional::<PipelineActivationV1>(generator)
            )
        ],
        &[
            "id",
            "title",
            "source_path",
            "work_level",
            "sets",
            "activation"
        ]
    )
);
impl_schema!(PipelineDefinitionV1, "PipelineDefinitionV1", |generator| {
    closed_object(
        vec![
            ("id", flow_text(255)),
            ("title", flow_text(1024)),
            ("description", flow_text(8192)),
            ("source_path", flow_path()),
            ("default_runner", flow_text(255)),
            ("default_profile", flow_text(255)),
            (
                "default_enable_complexity",
                json_schema!({"type":"boolean"}),
            ),
            ("stages", flow_array::<PipelineDeclaredStageV1>(generator)),
        ],
        &[
            "id",
            "title",
            "description",
            "source_path",
            "default_runner",
            "default_profile",
            "default_enable_complexity",
            "stages",
        ],
    )
});
impl_schema!(PipelineStageV1, "PipelineStageV1", |generator| {
    closed_object(
        vec![
            ("id", flow_text(255)),
            ("kind", flow_text(255)),
            ("version", flow_text(255)),
            ("title", flow_text(1024)),
            ("description", flow_text(8192)),
            ("work_level", flow_optional::<String>(generator)),
            ("source_path", flow_path()),
            (
                "pipeline_ids",
                json_schema!({"type":"array","maxItems":128,"items":flow_text(255)}),
            ),
        ],
        &[
            "id",
            "kind",
            "version",
            "title",
            "description",
            "work_level",
            "source_path",
            "pipeline_ids",
        ],
    )
});
impl_schema!(
    PipelineCatalogReadResultV1,
    "PipelineCatalogReadResultV1",
    |generator| json_schema!({"oneOf":[
        closed_object(vec![("selection_kind",json_schema!({"const":"pipeline"})),("pipeline",generator.subschema_for::<PipelineDefinitionV1>())], &["selection_kind","pipeline"]),
        closed_object(vec![("selection_kind",json_schema!({"const":"stage"})),("stage",generator.subschema_for::<PipelineStageV1>())], &["selection_kind","stage"])
    ]})
);
impl_schema!(
    PipelineOperationRouteStageReasonV1,
    "PipelineOperationRouteStageReasonV1",
    |generator| json_schema!({"oneOf":[
        closed_object(vec![("reason_kind",json_schema!({"const":"skipped_activation_false"})),("unsatisfied_variables",json_schema!({"type":"array","maxItems":128,"items":flow_text(255)}))], &["reason_kind","unsatisfied_variables"]),
        closed_object(vec![("reason_kind",json_schema!({"const":"next_missing_route_variables"})),("missing_variables",json_schema!({"type":"array","maxItems":128,"items":flow_text(255)}))], &["reason_kind","missing_variables"]),
        closed_object(vec![("reason_kind",json_schema!({"const":"blocked_by_unresolved_stage"})),("upstream_stage_id",flow_text(255)),("upstream_status",generator.subschema_for::<PipelineOperationRouteStageStatusV1>())], &["reason_kind","upstream_stage_id","upstream_status"])
    ]})
);
impl_schema!(
    PipelineOperationRouteRunV1,
    "PipelineOperationRouteRunV1",
    |generator| closed_object(
        vec![
            ("runner", flow_optional::<String>(generator)),
            ("profile", flow_optional::<String>(generator)),
            ("repository_root", flow_optional::<String>(generator))
        ],
        &["runner", "profile", "repository_root"]
    )
);
impl_schema!(
    PipelineRouteBasisRunnerV1,
    "PipelineRouteBasisRunnerV1",
    |_generator| closed_object(
        vec![
            ("id", flow_text(255)),
            ("file", flow_path()),
            ("file_sha256", flow_raw_hash())
        ],
        &["id", "file", "file_sha256"]
    )
);
impl_schema!(
    PipelineRouteBasisProfileV1,
    "PipelineRouteBasisProfileV1",
    |_generator| closed_object(
        vec![
            ("id", flow_text(255)),
            ("profile_yaml_sha256", flow_raw_hash()),
            ("commands_yaml_sha256", flow_raw_hash()),
            ("conventions_md_sha256", flow_raw_hash())
        ],
        &[
            "id",
            "profile_yaml_sha256",
            "commands_yaml_sha256",
            "conventions_md_sha256"
        ]
    )
);
impl_schema!(
    PipelineRouteBasisStageV1,
    "PipelineRouteBasisStageV1",
    |generator| closed_object(
        vec![
            ("stage_id", flow_text(255)),
            ("file", flow_path()),
            (
                "status",
                generator.subschema_for::<PipelineOperationRouteStageStatusV1>()
            ),
            (
                "reason",
                flow_optional::<PipelineOperationRouteStageReasonV1>(generator)
            )
        ],
        &["stage_id", "file", "status", "reason"]
    )
);
impl_schema!(
    PipelineOperationRoutingEntryV1,
    "PipelineOperationRoutingEntryV1",
    |_generator| closed_object(
        vec![
            ("variable", flow_text(255)),
            ("value", json_schema!({"type":"boolean"}))
        ],
        &["variable", "value"]
    )
);
impl_schema!(PipelineRouteBasisV1, "PipelineRouteBasisV1", |generator| {
    closed_object(
        vec![
            ("schema_version", flow_text(255)),
            (
                "state_revision",
                json_schema!({"type":"integer","minimum":0,"maximum":MAX_SAFE_INTEGER}),
            ),
            ("pipeline_file", flow_path()),
            ("pipeline_file_sha256", flow_raw_hash()),
            (
                "routing",
                flow_array::<PipelineOperationRoutingEntryV1>(generator),
            ),
            ("charter_ref", flow_optional_text(4096)),
            ("project_context_ref", flow_optional_text(4096)),
            ("run", generator.subschema_for::<PipelineRouteRunV1>()),
            (
                "runner",
                generator.subschema_for::<PipelineRouteBasisRunnerV1>(),
            ),
            (
                "profile",
                generator.subschema_for::<PipelineRouteBasisProfileV1>(),
            ),
            ("route", flow_array::<PipelineRouteBasisStageV1>(generator)),
        ],
        &[
            "schema_version",
            "state_revision",
            "pipeline_file",
            "pipeline_file_sha256",
            "routing",
            "charter_ref",
            "project_context_ref",
            "run",
            "runner",
            "profile",
            "route",
        ],
    )
});
impl_schema!(
    PipelineCompileTargetV1,
    "PipelineCompileTargetV1",
    |_generator| closed_object(
        vec![
            ("pipeline_id", flow_text(255)),
            ("stage_id", flow_text(255)),
            ("stage_file", flow_path()),
            ("title", flow_text(1024)),
            ("description", flow_text(8192)),
            ("work_level", flow_text(255)),
            (
                "tags",
                json_schema!({"type":"array","maxItems":128,"items":flow_text(255)})
            )
        ],
        &[
            "pipeline_id",
            "stage_id",
            "stage_file",
            "title",
            "description",
            "work_level",
            "tags"
        ]
    )
);
impl_schema!(
    PipelineCompileVariableV1,
    "PipelineCompileVariableV1",
    |_generator| closed_object(
        vec![
            ("name", flow_text(255)),
            ("value", flow_optional_text(65_536))
        ],
        &["name", "value"]
    )
);
impl_schema!(
    PipelineCompileDocumentV1,
    "PipelineCompileDocumentV1",
    |generator| closed_object(
        vec![
            (
                "kind",
                generator.subschema_for::<PipelineCompileDocumentKindV1>()
            ),
            ("path", flow_path()),
            ("required", json_schema!({"type":"boolean"})),
            (
                "status",
                generator.subschema_for::<PipelineCompileDocumentStatusV1>()
            ),
            ("content", flow_optional_text(65_536))
        ],
        &["kind", "path", "required", "status", "content"]
    )
);
impl_schema!(
    PipelineCompileOutputV1,
    "PipelineCompileOutputV1",
    |generator| closed_object(
        vec![
            (
                "kind",
                generator.subschema_for::<PipelineCompileOutputKindV1>()
            ),
            (
                "path",
                json_schema!({
                    "type":"string",
                    "minLength":1,
                    "maxLength":4096,
                    "pattern":"^(\\$\\{repo_root\\}/)?[A-Za-z0-9._-]+(/[A-Za-z0-9._-]+)*$"
                })
            )
        ],
        &["kind", "path"]
    )
);
impl_schema!(
    PipelineCompileGatingV1,
    "PipelineCompileGatingV1",
    |_generator| closed_object(
        vec![
            ("mode", flow_optional_text(255)),
            (
                "fail_on",
                json_schema!({"type":"array","maxItems":128,"items":flow_text(255)})
            ),
            (
                "notes",
                json_schema!({"type":"array","maxItems":128,"items":flow_text(8192)})
            )
        ],
        &["mode", "fail_on", "notes"]
    )
);
impl_schema!(
    PipelineCompileResultV1,
    "PipelineCompileResultV1",
    |generator| closed_object(
        vec![
            (
                "target",
                generator.subschema_for::<PipelineCompileTargetV1>()
            ),
            ("basis", generator.subschema_for::<PipelineRouteBasisV1>()),
            (
                "variables",
                flow_array::<PipelineCompileVariableV1>(generator)
            ),
            (
                "documents",
                flow_array::<PipelineCompileDocumentV1>(generator)
            ),
            ("outputs", flow_array::<PipelineCompileOutputV1>(generator)),
            (
                "gating",
                generator.subschema_for::<PipelineCompileGatingV1>()
            ),
            ("stage_body", flow_optional_text(65_536))
        ],
        &[
            "target",
            "basis",
            "variables",
            "documents",
            "outputs",
            "gating",
            "stage_body"
        ]
    )
);
impl_schema!(
    PipelineCompileRefusalV1,
    "PipelineCompileRefusalV1",
    |generator| closed_object(
        vec![
            (
                "classification",
                generator.subschema_for::<PipelineCompileRefusalClassificationV1>()
            ),
            ("summary", flow_text(8192)),
            ("pipeline_id", flow_optional::<String>(generator)),
            ("stage_id", flow_optional::<String>(generator)),
            ("recovery", flow_text(8192))
        ],
        &[
            "classification",
            "summary",
            "pipeline_id",
            "stage_id",
            "recovery"
        ]
    )
);
impl_schema!(
    PipelineMutationProblemDetailsV1,
    "PipelineMutationProblemDetailsV1",
    |generator| closed_object(
        vec![
            ("pipeline_id", flow_optional_text(255)),
            ("stage_id", flow_optional_text(255)),
            ("consumer_id", flow_optional_text(255)),
            ("recovery", flow_optional_text(8192)),
            ("expected_revision", nullable::<SafeU64>(generator)),
            ("actual_revision", nullable::<SafeU64>(generator)),
            (
                "established_request_fingerprint",
                nullable::<Fingerprint>(generator),
            ),
            (
                "actual_request_fingerprint",
                nullable::<Fingerprint>(generator),
            ),
        ],
        &[
            "pipeline_id",
            "stage_id",
            "consumer_id",
            "recovery",
            "expected_revision",
            "actual_revision",
            "established_request_fingerprint",
            "actual_request_fingerprint",
        ],
    )
);
impl_schema!(
    PipelineProblemDetailsV1,
    "PipelineProblemDetailsV1",
    |generator| closed_object(
        vec![
            ("cause", generator.subschema_for::<PipelineProblemCauseV1>()),
            ("detail", flow_text(8192)),
            ("selector", flow_optional::<String>(generator)),
            (
                "matches",
                json_schema!({"type":"array","maxItems":128,"items":flow_text(4096)})
            ),
            (
                "compile_refusal",
                flow_optional::<PipelineCompileRefusalV1>(generator)
            ),
            (
                "mutation",
                flow_optional::<PipelineMutationProblemDetailsV1>(generator)
            ),
        ],
        &[
            "cause",
            "detail",
            "selector",
            "matches",
            "compile_refusal",
            "mutation",
        ]
    )
);
impl_schema!(PipelineRecheckV1, "PipelineRecheckV1", |generator| {
    closed_object(
        vec![("cause", generator.subschema_for::<PipelineProblemCauseV1>())],
        &["cause"],
    )
});

const DIALECT: &str = "https://json-schema.org/draft/2020-12/schema";

struct SchemaDescriptor {
    exact_ref: &'static str,
    relative_path: &'static str,
    generate: fn() -> Schema,
}

const INVENTORY: [SchemaDescriptor; 168] = [
    descriptor::<ArtifactProblemDetailsV1>(
        "handbook.artifact-operation-details@1.0.0",
        "handbook.artifact-operation-details/1.0.0.schema.json",
    ),
    exact_descriptor(
        "handbook.artifact-recheck.repository-authority@1.0.0",
        "handbook.artifact-recheck.repository-authority/1.0.0.schema.json",
        artifact_repository_recheck_schema,
    ),
    exact_descriptor(
        "handbook.artifact-recheck.supported-platform@1.0.0",
        "handbook.artifact-recheck.supported-platform/1.0.0.schema.json",
        artifact_platform_recheck_schema,
    ),
    descriptor::<ArtifactRef>(
        "handbook.artifact-ref@1.0.0",
        "handbook.artifact-ref/1.0.0.schema.json",
    ),
    exact_descriptor(
        "handbook.artifact-write-condition.canonical-truth@1.0.0",
        "handbook.artifact-write-condition.canonical-truth/1.0.0.schema.json",
        artifact_canonical_condition_schema,
    ),
    exact_descriptor(
        "handbook.artifact-write-condition.semantic-record@1.0.0",
        "handbook.artifact-write-condition.semantic-record/1.0.0.schema.json",
        artifact_semantic_condition_schema,
    ),
    descriptor::<BootstrapDescriptorV1>(
        "handbook.bootstrap-descriptor-schema@1.0.0",
        "handbook.bootstrap-descriptor-schema/1.0.0.schema.json",
    ),
    descriptor::<BootstrapFacadeResponseV1>(
        "handbook.bootstrap-facade-response@1.0.0",
        "handbook.bootstrap-facade-response/1.0.0.schema.json",
    ),
    descriptor::<BootstrapRefusalDetails>(
        "handbook.bootstrap-refusal-details@1.0.0",
        "handbook.bootstrap-refusal-details/1.0.0.schema.json",
    ),
    descriptor::<BootstrapRefusalV1>(
        "handbook.bootstrap-refusal@1.0.0",
        "handbook.bootstrap-refusal/1.0.0.schema.json",
    ),
    descriptor::<BootstrapRequestV1>(
        "handbook.bootstrap-request@1.0.0",
        "handbook.bootstrap-request/1.0.0.schema.json",
    ),
    descriptor::<BootstrapResponseV1>(
        "handbook.bootstrap-response@1.0.0",
        "handbook.bootstrap-response/1.0.0.schema.json",
    ),
    descriptor::<CapabilityCatalogPage>(
        "handbook.capability-catalog-page@1.0.0",
        "handbook.capability-catalog-page/1.0.0.schema.json",
    ),
    descriptor::<CapabilityEntry>(
        "handbook.capability-entry@1.0.0",
        "handbook.capability-entry/1.0.0.schema.json",
    ),
    descriptor::<CatalogCursor>(
        "handbook.catalog-cursor@1.0.0",
        "handbook.catalog-cursor/1.0.0.schema.json",
    ),
    descriptor::<CatalogPage>(
        "handbook.catalog-page@1.0.0",
        "handbook.catalog-page/1.0.0.schema.json",
    ),
    descriptor::<CatalogReadBlockedV1>(
        "handbook.catalog-read-blocked@1.0.0",
        "handbook.catalog-read-blocked/1.0.0.schema.json",
    ),
    descriptor::<CatalogReadRequestV1>(
        "handbook.catalog-read-request@1.0.0",
        "handbook.catalog-read-request/1.0.0.schema.json",
    ),
    descriptor::<CatalogReadResponseV1>(
        "handbook.catalog-read-response@1.0.0",
        "handbook.catalog-read-response/1.0.0.schema.json",
    ),
    descriptor::<CatalogRoot>(
        "handbook.catalog-root@1.0.0",
        "handbook.catalog-root/1.0.0.schema.json",
    ),
    descriptor::<CatalogRoots>(
        "handbook.catalog-roots@1.0.0",
        "handbook.catalog-roots/1.0.0.schema.json",
    ),
    descriptor::<ContractApplicabilityOutcomeDtoV1>(
        "handbook.contract-applicability-outcome@1.0.0",
        "handbook.contract-applicability-outcome/1.0.0.schema.json",
    ),
    descriptor::<ContractClaimVerdictDtoV1>(
        "handbook.contract-claim-verdict@1.0.0",
        "handbook.contract-claim-verdict/1.0.0.schema.json",
    ),
    descriptor::<ContractClaimDtoV1>(
        "handbook.contract-claim@1.0.0",
        "handbook.contract-claim/1.0.0.schema.json",
    ),
    descriptor::<ContractDefinitionDtoV1>(
        "handbook.contract-definition@1.0.0",
        "handbook.contract-definition/1.0.0.schema.json",
    ),
    descriptor::<ContractEvaluationLifecycleBasisDtoV1>(
        "handbook.contract-evaluation-lifecycle-basis@1.0.0",
        "handbook.contract-evaluation-lifecycle-basis/1.0.0.schema.json",
    ),
    descriptor::<ContractEvaluationRunDtoV1>(
        "handbook.contract-evaluation-run@1.0.0",
        "handbook.contract-evaluation-run/1.0.0.schema.json",
    ),
    descriptor::<ContractEvidenceAdmissionPolicyDtoV1>(
        "handbook.contract-evidence-admission-policy@1.0.0",
        "handbook.contract-evidence-admission-policy/1.0.0.schema.json",
    ),
    descriptor::<ContractEvidenceCandidateDtoV1>(
        "handbook.contract-evidence-candidate@1.0.0",
        "handbook.contract-evidence-candidate/1.0.0.schema.json",
    ),
    descriptor::<ContractEvidenceFreshnessPolicyDtoV1>(
        "handbook.contract-evidence-freshness-policy@1.0.0",
        "handbook.contract-evidence-freshness-policy/1.0.0.schema.json",
    ),
    descriptor::<ContractEvidenceOperationDetailsDtoV1>(
        "handbook.contract-evidence-operation-details@1.0.0",
        "handbook.contract-evidence-operation-details/1.0.0.schema.json",
    ),
    descriptor::<ContractEvidenceRecordDtoV1>(
        "handbook.contract-evidence-record@1.0.0",
        "handbook.contract-evidence-record/1.0.0.schema.json",
    ),
    descriptor::<ContractEvidenceStabilityPolicyDtoV1>(
        "handbook.contract-evidence-stability-policy@1.0.0",
        "handbook.contract-evidence-stability-policy/1.0.0.schema.json",
    ),
    descriptor::<ContractGatePolicyDtoV1>(
        "handbook.contract-gate-policy@1.0.0",
        "handbook.contract-gate-policy/1.0.0.schema.json",
    ),
    descriptor::<ContractGateResultDtoV1>(
        "handbook.contract-gate-result@1.0.0",
        "handbook.contract-gate-result/1.0.0.schema.json",
    ),
    descriptor::<ContractLifecyclePolicyDtoV1>(
        "handbook.contract-lifecycle-policy@1.0.0",
        "handbook.contract-lifecycle-policy/1.0.0.schema.json",
    ),
    descriptor::<ContractLifecycleTransitionDtoV1>(
        "handbook.contract-lifecycle-transition@1.0.0",
        "handbook.contract-lifecycle-transition/1.0.0.schema.json",
    ),
    descriptor::<ContractLocalCloseoutPolicyDtoV1>(
        "handbook.contract-local-closeout-policy@1.0.0",
        "handbook.contract-local-closeout-policy/1.0.0.schema.json",
    ),
    descriptor::<ContractLocalCloseoutResultDtoV1>(
        "handbook.contract-local-closeout-result@1.0.0",
        "handbook.contract-local-closeout-result/1.0.0.schema.json",
    ),
    descriptor::<ContractOperationProblemDetailsV1>(
        "handbook.contract-operation-details@1.0.0",
        "handbook.contract-operation-details/1.0.0.schema.json",
    ),
    descriptor::<CorrelationRef>(
        "handbook.correlation-ref@1.0.0",
        "handbook.correlation-ref/1.0.0.schema.json",
    ),
    descriptor::<Diagnostic>(
        "handbook.diagnostic@1.0.0",
        "handbook.diagnostic/1.0.0.schema.json",
    ),
    descriptor::<DockExecutionRecordDtoV1>(
        "handbook.dock-execution-record@1.0.0",
        "handbook.dock-execution-record/1.0.0.schema.json",
    ),
    descriptor::<DockManifestDtoV1>(
        "handbook.dock-manifest@1.0.0",
        "handbook.dock-manifest/1.0.0.schema.json",
    ),
    descriptor::<DockOperationDetailsDtoV1>(
        "handbook.dock-operation-details@1.0.0",
        "handbook.dock-operation-details/1.0.0.schema.json",
    ),
    descriptor::<DockProcessRequestDtoV1>(
        "handbook.dock-process-request@1.0.0",
        "handbook.dock-process-request/1.0.0.schema.json",
    ),
    descriptor::<DockProcessResultDtoV1>(
        "handbook.dock-process-result@1.0.0",
        "handbook.dock-process-result/1.0.0.schema.json",
    ),
    descriptor::<FlowProblemDetailsV1>(
        "handbook.flow-operation-details@1.0.0",
        "handbook.flow-operation-details/1.0.0.schema.json",
    ),
    exact_descriptor(
        "handbook.flow-recheck@1.0.0",
        "handbook.flow-recheck/1.0.0.schema.json",
        flow_recheck_schema,
    ),
    descriptor::<IdempotencyState>(
        "handbook.idempotency-state@1.0.0",
        "handbook.idempotency-state/1.0.0.schema.json",
    ),
    descriptor::<InvocationRequestV1>(
        "handbook.invocation-request@1.0.0",
        "handbook.invocation-request/1.0.0.schema.json",
    ),
    descriptor::<InvocationResponseV1>(
        "handbook.invocation-response@1.0.0",
        "handbook.invocation-response/1.0.0.schema.json",
    ),
    descriptor::<NextAction>(
        "handbook.next-action@1.0.0",
        "handbook.next-action/1.0.0.schema.json",
    ),
    descriptor::<Omission>(
        "handbook.omission@1.0.0",
        "handbook.omission/1.0.0.schema.json",
    ),
    descriptor::<OperationAdmissionRefusalV1>(
        "handbook.operation-admission-refusal@1.0.0",
        "handbook.operation-admission-refusal/1.0.0.schema.json",
    ),
    descriptor::<Problem>(
        "handbook.operation-blocker@1.0.0",
        "handbook.operation-blocker/1.0.0.schema.json",
    ),
    descriptor::<OperationDefinition>(
        "handbook.operation-definition@1.0.0",
        "handbook.operation-definition/1.0.0.schema.json",
    ),
    descriptor::<Problem>(
        "handbook.operation-error@1.0.0",
        "handbook.operation-error/1.0.0.schema.json",
    ),
    descriptor::<Problem>(
        "handbook.operation-refusal@1.0.0",
        "handbook.operation-refusal/1.0.0.schema.json",
    ),
    descriptor::<OperationRequestV1>(
        "handbook.operation-request@1.0.0",
        "handbook.operation-request/1.0.0.schema.json",
    ),
    descriptor::<OperationResponseV1>(
        "handbook.operation-response@1.0.0",
        "handbook.operation-response/1.0.0.schema.json",
    ),
    descriptor::<CandidateAppendRequestV1>(
        "handbook.operation.artifact-candidate-append-request@1.0.0",
        "handbook.operation.artifact-candidate-append-request/1.0.0.schema.json",
    ),
    mutation_result_descriptor(
        "handbook.operation.artifact-candidate-append-result@1.0.0",
        "handbook.operation.artifact-candidate-append-result/1.0.0.schema.json",
        artifact_candidate_append_result_schema,
    ),
    descriptor::<PromoteRequestV1>(
        "handbook.operation.artifact-candidate-promote-request@1.0.0",
        "handbook.operation.artifact-candidate-promote-request/1.0.0.schema.json",
    ),
    mutation_result_descriptor(
        "handbook.operation.artifact-candidate-promote-result@1.0.0",
        "handbook.operation.artifact-candidate-promote-result/1.0.0.schema.json",
        artifact_candidate_promote_result_schema,
    ),
    descriptor::<CandidateValidateRequestV1>(
        "handbook.operation.artifact-candidate-validate-request@1.0.0",
        "handbook.operation.artifact-candidate-validate-request/1.0.0.schema.json",
    ),
    descriptor::<CandidateValidationResultV1>(
        "handbook.operation.artifact-candidate-validate-result@1.0.0",
        "handbook.operation.artifact-candidate-validate-result/1.0.0.schema.json",
    ),
    descriptor::<EmptyRequestV1>(
        "handbook.operation.artifact-instance-list-request@1.0.0",
        "handbook.operation.artifact-instance-list-request/1.0.0.schema.json",
    ),
    descriptor::<InstanceListResultV1>(
        "handbook.operation.artifact-instance-list-result@1.0.0",
        "handbook.operation.artifact-instance-list-result/1.0.0.schema.json",
    ),
    descriptor::<EmptyRequestV1>(
        "handbook.operation.artifact-kind-list-request@1.0.0",
        "handbook.operation.artifact-kind-list-request/1.0.0.schema.json",
    ),
    descriptor::<KindListResultV1>(
        "handbook.operation.artifact-kind-list-result@1.0.0",
        "handbook.operation.artifact-kind-list-result/1.0.0.schema.json",
    ),
    descriptor::<TargetRequestV1>(
        "handbook.operation.artifact-read-request@1.0.0",
        "handbook.operation.artifact-read-request/1.0.0.schema.json",
    ),
    descriptor::<ReadResultV1>(
        "handbook.operation.artifact-read-result@1.0.0",
        "handbook.operation.artifact-read-result/1.0.0.schema.json",
    ),
    descriptor::<TargetRequestV1>(
        "handbook.operation.artifact-validate-request@1.0.0",
        "handbook.operation.artifact-validate-request/1.0.0.schema.json",
    ),
    descriptor::<ValidateResultV1>(
        "handbook.operation.artifact-validate-result@1.0.0",
        "handbook.operation.artifact-validate-result/1.0.0.schema.json",
    ),
    descriptor::<AppendWireV1>(
        "handbook.operation.contract-definition-append-request@1.0.0",
        "handbook.operation.contract-definition-append-request/1.0.0.schema.json",
    ),
    descriptor::<ContractDefinitionAppendResultV1>(
        "handbook.operation.contract-definition-append-result@1.0.0",
        "handbook.operation.contract-definition-append-result/1.0.0.schema.json",
    ),
    descriptor::<ContractDefinitionListRequestV1>(
        "handbook.operation.contract-definition-list-request@1.0.0",
        "handbook.operation.contract-definition-list-request/1.0.0.schema.json",
    ),
    descriptor::<ContractDefinitionListResultV1>(
        "handbook.operation.contract-definition-list-result@1.0.0",
        "handbook.operation.contract-definition-list-result/1.0.0.schema.json",
    ),
    descriptor::<ContractDefinitionReadRequestV1>(
        "handbook.operation.contract-definition-read-request@1.0.0",
        "handbook.operation.contract-definition-read-request/1.0.0.schema.json",
    ),
    descriptor::<ContractDefinitionReadResultV1>(
        "handbook.operation.contract-definition-read-result@1.0.0",
        "handbook.operation.contract-definition-read-result/1.0.0.schema.json",
    ),
    descriptor::<ContractEvidenceAppendWireV1>(
        "handbook.operation.contract-evidence-append-request@1.0.0",
        "handbook.operation.contract-evidence-append-request/1.0.0.schema.json",
    ),
    descriptor::<ContractEvidenceAppendResultV1>(
        "handbook.operation.contract-evidence-append-result@1.0.0",
        "handbook.operation.contract-evidence-append-result/1.0.0.schema.json",
    ),
    descriptor::<ContractEvidenceListRequestV1>(
        "handbook.operation.contract-evidence-list-request@1.0.0",
        "handbook.operation.contract-evidence-list-request/1.0.0.schema.json",
    ),
    descriptor::<ContractEvidenceListResultV1>(
        "handbook.operation.contract-evidence-list-result@1.0.0",
        "handbook.operation.contract-evidence-list-result/1.0.0.schema.json",
    ),
    descriptor::<ContractEvidenceReadRequestV1>(
        "handbook.operation.contract-evidence-read-request@1.0.0",
        "handbook.operation.contract-evidence-read-request/1.0.0.schema.json",
    ),
    descriptor::<ContractEvidenceReadResultV1>(
        "handbook.operation.contract-evidence-read-result@1.0.0",
        "handbook.operation.contract-evidence-read-result/1.0.0.schema.json",
    ),
    descriptor::<ContractGateEvaluateRequestV1>(
        "handbook.operation.contract-gate-evaluate-request@1.0.0",
        "handbook.operation.contract-gate-evaluate-request/1.0.0.schema.json",
    ),
    descriptor::<ContractGateEvaluateResultV1>(
        "handbook.operation.contract-gate-evaluate-result@1.0.0",
        "handbook.operation.contract-gate-evaluate-result/1.0.0.schema.json",
    ),
    descriptor::<TransitionWireV1>(
        "handbook.operation.contract-lifecycle-transition-request@1.0.0",
        "handbook.operation.contract-lifecycle-transition-request/1.0.0.schema.json",
    ),
    descriptor::<ContractLifecycleTransitionResultV1>(
        "handbook.operation.contract-lifecycle-transition-result@1.0.0",
        "handbook.operation.contract-lifecycle-transition-result/1.0.0.schema.json",
    ),
    descriptor::<ContractVerdictEvaluateRequestV1>(
        "handbook.operation.contract-verdict-evaluate-request@1.0.0",
        "handbook.operation.contract-verdict-evaluate-request/1.0.0.schema.json",
    ),
    descriptor::<ContractVerdictEvaluateResultV1>(
        "handbook.operation.contract-verdict-evaluate-result@1.0.0",
        "handbook.operation.contract-verdict-evaluate-result/1.0.0.schema.json",
    ),
    descriptor::<DockManifestListRequestV1>(
        "handbook.operation.dock-manifest-list-request@1.0.0",
        "handbook.operation.dock-manifest-list-request/1.0.0.schema.json",
    ),
    descriptor::<DockManifestListResultV1>(
        "handbook.operation.dock-manifest-list-result@1.0.0",
        "handbook.operation.dock-manifest-list-result/1.0.0.schema.json",
    ),
    descriptor::<DockManifestReadRequestV1>(
        "handbook.operation.dock-manifest-read-request@1.0.0",
        "handbook.operation.dock-manifest-read-request/1.0.0.schema.json",
    ),
    descriptor::<DockManifestReadResultV1>(
        "handbook.operation.dock-manifest-read-result@1.0.0",
        "handbook.operation.dock-manifest-read-result/1.0.0.schema.json",
    ),
    descriptor::<DockRunWireV1>(
        "handbook.operation.dock-run-request@1.0.0",
        "handbook.operation.dock-run-request/1.0.0.schema.json",
    ),
    descriptor::<DockRunResultV1>(
        "handbook.operation.dock-run-result@1.0.0",
        "handbook.operation.dock-run-result/1.0.0.schema.json",
    ),
    descriptor::<FlowResolveRequestV1>(
        "handbook.operation.flow-resolve-request@1.0.0",
        "handbook.operation.flow-resolve-request/1.0.0.schema.json",
    ),
    descriptor::<FlowResolveResultV1>(
        "handbook.operation.flow-resolve-result@1.0.0",
        "handbook.operation.flow-resolve-result/1.0.0.schema.json",
    ),
    descriptor::<IntakeEvaluateRequestV1>(
        "handbook.operation.intake-coverage-evaluate-request@1.0.0",
        "handbook.operation.intake-coverage-evaluate-request/1.0.0.schema.json",
    ),
    descriptor::<IntakeEvaluationResultV1>(
        "handbook.operation.intake-coverage-evaluate-result@1.0.0",
        "handbook.operation.intake-coverage-evaluate-result/1.0.0.schema.json",
    ),
    descriptor::<TargetRequestV1>(
        "handbook.operation.intake-definition-read-request@1.0.0",
        "handbook.operation.intake-definition-read-request/1.0.0.schema.json",
    ),
    descriptor::<IntakeDefinitionResultV1>(
        "handbook.operation.intake-definition-read-result@1.0.0",
        "handbook.operation.intake-definition-read-result/1.0.0.schema.json",
    ),
    descriptor::<IntakeAppendRequestV1>(
        "handbook.operation.intake-record-append-request@1.0.0",
        "handbook.operation.intake-record-append-request/1.0.0.schema.json",
    ),
    mutation_result_descriptor(
        "handbook.operation.intake-record-append-result@1.0.0",
        "handbook.operation.intake-record-append-result/1.0.0.schema.json",
        intake_record_append_result_schema,
    ),
    descriptor::<PipelineCaptureApplyRequestSchemaV1>(
        "handbook.operation.pipeline-capture-apply-request@1.0.0",
        "handbook.operation.pipeline-capture-apply-request/1.0.0.schema.json",
    ),
    descriptor::<PipelineCaptureApplyResultV1>(
        "handbook.operation.pipeline-capture-apply-result@1.0.0",
        "handbook.operation.pipeline-capture-apply-result/1.0.0.schema.json",
    ),
    descriptor::<PipelineCapturePlanRequestV1>(
        "handbook.operation.pipeline-capture-plan-request@1.0.0",
        "handbook.operation.pipeline-capture-plan-request/1.0.0.schema.json",
    ),
    descriptor::<PipelineCapturePlanResultV1>(
        "handbook.operation.pipeline-capture-plan-result@1.0.0",
        "handbook.operation.pipeline-capture-plan-result/1.0.0.schema.json",
    ),
    descriptor::<PipelineEmptyRequestV1>(
        "handbook.operation.pipeline-catalog-list-request@1.0.0",
        "handbook.operation.pipeline-catalog-list-request/1.0.0.schema.json",
    ),
    descriptor::<PipelineCatalogListResultV1>(
        "handbook.operation.pipeline-catalog-list-result@1.0.0",
        "handbook.operation.pipeline-catalog-list-result/1.0.0.schema.json",
    ),
    descriptor::<PipelineCatalogReadRequestV1>(
        "handbook.operation.pipeline-catalog-read-request@1.0.0",
        "handbook.operation.pipeline-catalog-read-request/1.0.0.schema.json",
    ),
    descriptor::<PipelineCatalogReadResultV1>(
        "handbook.operation.pipeline-catalog-read-result@1.0.0",
        "handbook.operation.pipeline-catalog-read-result/1.0.0.schema.json",
    ),
    descriptor::<PipelineCompileRequestV1>(
        "handbook.operation.pipeline-compile-request@1.0.0",
        "handbook.operation.pipeline-compile-request/1.0.0.schema.json",
    ),
    descriptor::<PipelineCompileResultV1>(
        "handbook.operation.pipeline-compile-result@1.0.0",
        "handbook.operation.pipeline-compile-result/1.0.0.schema.json",
    ),
    descriptor::<PipelineHandoffEmitRequestSchemaV1>(
        "handbook.operation.pipeline-handoff-emit-request@1.0.0",
        "handbook.operation.pipeline-handoff-emit-request/1.0.0.schema.json",
    ),
    descriptor::<PipelineHandoffEmitResultV1>(
        "handbook.operation.pipeline-handoff-emit-result@1.0.0",
        "handbook.operation.pipeline-handoff-emit-result/1.0.0.schema.json",
    ),
    descriptor::<PipelineStateApplyRequestSchemaV1>(
        "handbook.operation.pipeline-state-apply-request@1.0.0",
        "handbook.operation.pipeline-state-apply-request/1.0.0.schema.json",
    ),
    descriptor::<PipelineStateApplyResultV1>(
        "handbook.operation.pipeline-state-apply-result@1.0.0",
        "handbook.operation.pipeline-state-apply-result/1.0.0.schema.json",
    ),
    descriptor::<PostureTransitionApplyRequestSchemaV1>(
        "handbook.operation.posture-transition-apply-request@1.0.0",
        "handbook.operation.posture-transition-apply-request/1.0.0.schema.json",
    ),
    descriptor::<PostureTransitionApplyResultDataV1>(
        "handbook.operation.posture-transition-apply-result@1.0.0",
        "handbook.operation.posture-transition-apply-result/1.0.0.schema.json",
    ),
    descriptor::<OwnerVersionEntry>(
        "handbook.owner-version-entry@1.0.0",
        "handbook.owner-version-entry/1.0.0.schema.json",
    ),
    descriptor::<PipelineProblemDetailsV1>(
        "handbook.pipeline-operation-details@1.0.0",
        "handbook.pipeline-operation-details/1.0.0.schema.json",
    ),
    descriptor::<PipelineRecheckV1>(
        "handbook.pipeline-recheck@1.0.0",
        "handbook.pipeline-recheck/1.0.0.schema.json",
    ),
    descriptor::<PipelineWriteConditionV1>(
        "handbook.pipeline-write-condition@1.0.0",
        "handbook.pipeline-write-condition/1.0.0.schema.json",
    ),
    descriptor::<PostureAuthorityConflictDetailsSchema>(
        "handbook.posture-details.authority-conflict@1.0.0",
        "handbook.posture-details.authority-conflict/1.0.0.schema.json",
    ),
    descriptor::<PostureIntegrityOrDurabilityDetailsSchema>(
        "handbook.posture-details.integrity-or-durability@1.0.0",
        "handbook.posture-details.integrity-or-durability/1.0.0.schema.json",
    ),
    descriptor::<PostureInvalidFixedDimensionDetailsSchema>(
        "handbook.posture-details.invalid-fixed-dimension@1.0.0",
        "handbook.posture-details.invalid-fixed-dimension/1.0.0.schema.json",
    ),
    descriptor::<PostureInvalidIdempotencyKeyDetailsSchema>(
        "handbook.posture-details.invalid-idempotency-key@1.0.0",
        "handbook.posture-details.invalid-idempotency-key/1.0.0.schema.json",
    ),
    descriptor::<PostureInvalidSemanticInputDetailsSchema>(
        "handbook.posture-details.invalid-semantic-input@1.0.0",
        "handbook.posture-details.invalid-semantic-input/1.0.0.schema.json",
    ),
    descriptor::<PostureIoDetailsSchema>(
        "handbook.posture-details.io@1.0.0",
        "handbook.posture-details.io/1.0.0.schema.json",
    ),
    descriptor::<PostureLifecycleNotCurrentDetailsSchema>(
        "handbook.posture-details.lifecycle-not-current@1.0.0",
        "handbook.posture-details.lifecycle-not-current/1.0.0.schema.json",
    ),
    descriptor::<PostureMissingLifecycleAuthorityDetailsSchema>(
        "handbook.posture-details.missing-lifecycle-authority@1.0.0",
        "handbook.posture-details.missing-lifecycle-authority/1.0.0.schema.json",
    ),
    descriptor::<PostureRepositoryIdentityMismatchDetailsSchema>(
        "handbook.posture-details.repository-identity-mismatch@1.0.0",
        "handbook.posture-details.repository-identity-mismatch/1.0.0.schema.json",
    ),
    descriptor::<PostureStaleCanonicalDetailsSchema>(
        "handbook.posture-details.stale-canonical@1.0.0",
        "handbook.posture-details.stale-canonical/1.0.0.schema.json",
    ),
    descriptor::<PostureUnsafeFilesystemDetailsSchema>(
        "handbook.posture-details.unsafe-filesystem@1.0.0",
        "handbook.posture-details.unsafe-filesystem/1.0.0.schema.json",
    ),
    descriptor::<PostureUnsupportedPlatformDetailsSchema>(
        "handbook.posture-details.unsupported-platform@1.0.0",
        "handbook.posture-details.unsupported-platform/1.0.0.schema.json",
    ),
    descriptor::<PostureUnsupportedPlatformRecheckSchema>(
        "handbook.posture-recheck.unsupported-platform@1.0.0",
        "handbook.posture-recheck.unsupported-platform/1.0.0.schema.json",
    ),
    descriptor::<PostureCanonicalWriteConditionV1>(
        "handbook.posture-write-condition.canonical-truth@1.0.0",
        "handbook.posture-write-condition.canonical-truth/1.0.0.schema.json",
    ),
    descriptor::<PostureSemanticWriteConditionV1>(
        "handbook.posture-write-condition.semantic-record@1.0.0",
        "handbook.posture-write-condition.semantic-record/1.0.0.schema.json",
    ),
    descriptor::<ProblemBinding>(
        "handbook.problem-binding@1.0.0",
        "handbook.problem-binding/1.0.0.schema.json",
    ),
    descriptor::<Problem>(
        "handbook.problem@1.0.0",
        "handbook.problem/1.0.0.schema.json",
    ),
    descriptor::<ProfileCatalogEntry>(
        "handbook.profile-catalog-entry@1.0.0",
        "handbook.profile-catalog-entry/1.0.0.schema.json",
    ),
    descriptor::<ProfileCatalogPage>(
        "handbook.profile-catalog-page@1.0.0",
        "handbook.profile-catalog-page/1.0.0.schema.json",
    ),
    descriptor::<RecheckCondition>(
        "handbook.recheck-condition@1.0.0",
        "handbook.recheck-condition/1.0.0.schema.json",
    ),
    descriptor::<SchemaCatalogPage>(
        "handbook.schema-catalog-page@1.0.0",
        "handbook.schema-catalog-page/1.0.0.schema.json",
    ),
    descriptor::<SchemaManifestEntry>(
        "handbook.schema-manifest-entry@1.0.0",
        "handbook.schema-manifest-entry/1.0.0.schema.json",
    ),
    descriptor::<SourceBinding>(
        "handbook.source-binding@1.0.0",
        "handbook.source-binding/1.0.0.schema.json",
    ),
    descriptor::<TransportFailureDetailsV1>(
        "handbook.transport-failure-details@1.0.0",
        "handbook.transport-failure-details/1.0.0.schema.json",
    ),
    descriptor::<TransportFailureV1>(
        "handbook.transport-failure@1.0.0",
        "handbook.transport-failure/1.0.0.schema.json",
    ),
    descriptor::<TypedCatalogCursor>(
        "handbook.typed-catalog-cursor@1.0.0",
        "handbook.typed-catalog-cursor/1.0.0.schema.json",
    ),
    descriptor::<UseCaseRequestV1>(
        "handbook.use-case-request@1.0.0",
        "handbook.use-case-request/1.0.0.schema.json",
    ),
    descriptor::<UseCaseResponseV1>(
        "handbook.use-case-response@1.0.0",
        "handbook.use-case-response/1.0.0.schema.json",
    ),
    descriptor::<ApproverAdministrationDetailsV1>(
        "handbook.use-case.approver-administration-details@1.0.0",
        "handbook.use-case.approver-administration-details/1.0.0.schema.json",
    ),
    descriptor::<ApproverAdministrationResultV1>(
        "handbook.use-case.approver-administration-result@1.0.0",
        "handbook.use-case.approver-administration-result/1.0.0.schema.json",
    ),
    descriptor::<AuthorCharterDetailsV1>(
        "handbook.use-case.author-charter-details@1.0.0",
        "handbook.use-case.author-charter-details/1.0.0.schema.json",
    ),
    descriptor::<AuthorCharterResultV1>(
        "handbook.use-case.author-charter-result@1.0.0",
        "handbook.use-case.author-charter-result/1.0.0.schema.json",
    ),
    descriptor::<AuthorProjectContextDetailsV1>(
        "handbook.use-case.author-project-context-details@1.0.0",
        "handbook.use-case.author-project-context-details/1.0.0.schema.json",
    ),
    descriptor::<AuthorProjectContextResultV1>(
        "handbook.use-case.author-project-context-result@1.0.0",
        "handbook.use-case.author-project-context-result/1.0.0.schema.json",
    ),
    descriptor::<PipelineRouteResolveDetailsV1>(
        "handbook.use-case.pipeline-route-resolve-and-persist-details@1.0.0",
        "handbook.use-case.pipeline-route-resolve-and-persist-details/1.0.0.schema.json",
    ),
    descriptor::<PipelineRouteResolveAndPersistResultV1>(
        "handbook.use-case.pipeline-route-resolve-and-persist-result@1.0.0",
        "handbook.use-case.pipeline-route-resolve-and-persist-result/1.0.0.schema.json",
    ),
    descriptor::<RepositoryDoctorDetailsV1>(
        "handbook.use-case.repository-doctor-details@1.0.0",
        "handbook.use-case.repository-doctor-details/1.0.0.schema.json",
    ),
    descriptor::<RepositoryDoctorResultV1>(
        "handbook.use-case.repository-doctor-result@1.0.0",
        "handbook.use-case.repository-doctor-result/1.0.0.schema.json",
    ),
    descriptor::<RepositorySetupDetailsV1>(
        "handbook.use-case.repository-setup-details@1.0.0",
        "handbook.use-case.repository-setup-details/1.0.0.schema.json",
    ),
    descriptor::<RepositorySetupResultV1>(
        "handbook.use-case.repository-setup-result@1.0.0",
        "handbook.use-case.repository-setup-result/1.0.0.schema.json",
    ),
    descriptor::<WriteReceipt>(
        "handbook.write-receipt@1.0.0",
        "handbook.write-receipt/1.0.0.schema.json",
    ),
];

const fn descriptor<T: JsonSchema>(
    exact_ref: &'static str,
    relative_path: &'static str,
) -> SchemaDescriptor {
    SchemaDescriptor {
        exact_ref,
        relative_path,
        generate: generate_root::<T>,
    }
}

const fn mutation_result_descriptor(
    exact_ref: &'static str,
    relative_path: &'static str,
    generate: fn() -> Schema,
) -> SchemaDescriptor {
    SchemaDescriptor {
        exact_ref,
        relative_path,
        generate,
    }
}

const fn exact_descriptor(
    exact_ref: &'static str,
    relative_path: &'static str,
    generate: fn() -> Schema,
) -> SchemaDescriptor {
    SchemaDescriptor {
        exact_ref,
        relative_path,
        generate,
    }
}

fn artifact_repository_recheck_schema() -> Schema {
    exact_recheck_schema(&[
        "selection_unavailable",
        "profile_unavailable",
        "registry_unavailable",
        "intake_source_unavailable",
        "repository_identity_unavailable",
        "artifact_unavailable",
    ])
}

fn flow_recheck_schema() -> Schema {
    let mut schema = generate_root::<FlowRecheckV1>();
    let properties = schema
        .get_mut("properties")
        .and_then(serde_json::Value::as_object_mut)
        .expect("Flow recheck schema has object properties");
    properties.insert(
        "cause".to_owned(),
        string_enum(&[
            "system_root_missing",
            "system_root_not_dir",
            "system_root_symlink_not_allowed",
            "required_artifact_missing",
            "required_artifact_empty",
            "required_artifact_starter_template",
            "required_artifact_invalid",
            "artifact_read_error",
            "freshness_invalid",
            "budget_refused",
        ])
        .into(),
    );
    schema
}

fn artifact_platform_recheck_schema() -> Schema {
    exact_recheck_schema(&["unsupported_platform"])
}

fn exact_recheck_schema(causes: &[&str]) -> Schema {
    let mut schema = generate_root::<ArtifactRecheckV1>();
    let properties = schema
        .get_mut("properties")
        .and_then(serde_json::Value::as_object_mut)
        .expect("artifact recheck schema has object properties");
    properties.insert("cause".to_owned(), string_enum(causes).into());
    schema
}

fn artifact_canonical_condition_schema() -> Schema {
    exact_write_condition_schema("canonical_truth")
}

fn artifact_semantic_condition_schema() -> Schema {
    exact_write_condition_schema("semantic_record")
}

fn exact_write_condition_schema(authority_class: &str) -> Schema {
    let mut schema = generate_root::<ArtifactWriteConditionV1>();
    let properties = schema
        .get_mut("properties")
        .and_then(serde_json::Value::as_object_mut)
        .expect("artifact write condition schema has object properties");
    properties.insert(
        "authority_class".to_owned(),
        json_schema!({"const": authority_class}).into(),
    );
    schema
}

fn intake_record_append_result_schema() -> Schema {
    exact_mutation_result_schema("intake.record.append", &["semantic_record"])
}

fn artifact_candidate_append_result_schema() -> Schema {
    exact_mutation_result_schema("artifact.candidate.append", &["semantic_record"])
}

fn artifact_candidate_promote_result_schema() -> Schema {
    exact_mutation_result_schema(
        "artifact.candidate.promote",
        &["canonical_truth", "semantic_record"],
    )
}

fn exact_mutation_result_schema(operation_id: &str, expected_authority_classes: &[&str]) -> Schema {
    let mut schema = generate_root::<ArtifactMutationResultDataV1>();
    let properties = schema
        .get_mut("properties")
        .and_then(serde_json::Value::as_object_mut)
        .expect("mutation result schema has object properties");
    properties.insert(
        "operation_id".to_owned(),
        json_schema!({"const": operation_id}).into(),
    );
    let prefix_items = expected_authority_classes
        .iter()
        .map(|authority_class| {
            serde_json::json!({
                "allOf": [
                    {"$ref": "#/$defs/ArtifactAuthoritativeOutputV1"},
                    {"properties": {"authority_class": {"const": authority_class}}}
                ]
            })
        })
        .collect::<Vec<_>>();
    let committed_outputs = expected_authority_classes.len();
    schema.insert(
        "oneOf".to_owned(),
        serde_json::json!([
            {
                "properties": {
                    "outcome": {"const": "committed"},
                    "refusal": {"type": "null"},
                    "authoritative_outputs": {
                        "minItems": committed_outputs,
                        "maxItems": committed_outputs,
                        "prefixItems": prefix_items,
                        "items": false
                    }
                }
            },
            {
                "properties": {
                    "outcome": {"const": "refused"},
                    "refusal": {"not": {"type": "null"}},
                    "authoritative_outputs": {"maxItems": 0}
                }
            }
        ]),
    );
    schema
}

fn generate_root<T: JsonSchema>() -> Schema {
    SchemaSettings::draft2020_12()
        .for_deserialize()
        .into_generator()
        .into_root_schema_for::<T>()
}

struct GeneratedTree {
    schemas: BTreeMap<&'static str, Vec<u8>>,
    manifest: Vec<u8>,
}

/// The only supported generator modes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GenerationMode {
    Check,
    Write,
}

/// Generates a complete temporary tree, then checks or replaces the tracked output.
pub fn run(mode: GenerationMode) -> Result<(), String> {
    let generated = generate_tree()?;
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_root = manifest_dir
        .parent()
        .and_then(Path::parent)
        .ok_or_else(|| "SDK manifest directory has no workspace root".to_owned())?;
    let temporary_root = workspace_root
        .join("target")
        .join("contract-schema-generation");

    if temporary_root.exists() {
        fs::remove_dir_all(&temporary_root).map_err(io_error("remove temporary tree"))?;
    }
    write_tree(&temporary_root, &generated)?;

    let result = match mode {
        GenerationMode::Check => compare_tree(&temporary_root, &manifest_dir.join("schemas")),
        GenerationMode::Write => {
            replace_tracked_tree(&temporary_root, &manifest_dir.join("schemas"))?;
            compare_tree(&temporary_root, &manifest_dir.join("schemas"))
        }
    };
    let cleanup = fs::remove_dir_all(&temporary_root).map_err(io_error("remove temporary tree"));
    result.and(cleanup)
}

fn generate_tree() -> Result<GeneratedTree, String> {
    if INVENTORY.len() != 168 {
        return Err("schema inventory must contain exactly 168 entries".to_owned());
    }

    let mut previous_ref = None;
    let mut paths = BTreeSet::new();
    let mut schemas = BTreeMap::new();
    let mut manifest = Vec::with_capacity(INVENTORY.len());
    let limits = ContractLimits::packet0();

    for descriptor in &INVENTORY {
        if previous_ref.is_some_and(|previous| previous >= descriptor.exact_ref) {
            return Err("schema inventory refs must be unique and byte-sorted".to_owned());
        }
        previous_ref = Some(descriptor.exact_ref);
        if !paths.insert(descriptor.relative_path) {
            return Err("schema inventory paths must be unique".to_owned());
        }

        let exact_ref = ExactRef::parse(descriptor.exact_ref)
            .map_err(|_| "schema inventory contains an invalid exact ref".to_owned())?;
        let identifier = format!(
            "urn:handbook:schema:{}:{}",
            exact_ref.identity(),
            exact_ref.version()
        );
        let mut schema = (descriptor.generate)();
        schema.insert("$id".to_owned(), identifier.clone().into());
        validate_schema(descriptor, &schema, &identifier)?;

        let mut bytes = canonical_json_bytes(&schema, &limits)
            .map_err(|_| "schema did not fit the Packet-0 canonical domain".to_owned())?;
        bytes.push(b'\n');
        validate_schema_bytes(&schema, &bytes, &limits)?;

        let byte_length = SafeU64::new(bytes.len() as u64)
            .map_err(|_| "schema byte length exceeded the safe integer domain".to_owned())?;
        let entry = SchemaManifestEntry::new(
            ExactBinding::new(exact_ref, sha256_fingerprint(&bytes)),
            DraftVersion::Draft202012,
            byte_length,
            SchemaMediaType::ApplicationSchemaJson,
        )
        .map_err(|_| "schema manifest entry construction failed".to_owned())?;
        manifest.push(entry);
        schemas.insert(descriptor.relative_path, bytes);
    }

    let mut manifest = canonical_json_bytes(&manifest, &limits)
        .map_err(|_| "schema manifest did not fit the Packet-0 canonical domain".to_owned())?;
    manifest.push(b'\n');

    Ok(GeneratedTree { schemas, manifest })
}

fn validate_schema(
    descriptor: &SchemaDescriptor,
    schema: &Schema,
    identifier: &str,
) -> Result<(), String> {
    if schema.get("$schema").and_then(serde_json::Value::as_str) != Some(DIALECT) {
        return Err(format!(
            "{} has the wrong schema dialect",
            descriptor.relative_path
        ));
    }
    if schema.get("$id").and_then(serde_json::Value::as_str) != Some(identifier) {
        return Err(format!(
            "{} has the wrong schema id",
            descriptor.relative_path
        ));
    }

    let value = schema.as_value();
    validate_local_references(value, value, descriptor.relative_path)?;
    validate_closed_objects(value, descriptor.relative_path)?;
    if matches!(
        descriptor.exact_ref,
        REQUEST_SCHEMA_REF
            | "handbook.operation.intake-record-append-request@1.0.0"
            | "handbook.operation.artifact-candidate-append-request@1.0.0"
            | "handbook.operation.artifact-candidate-promote-request@1.0.0"
            | "handbook.operation.pipeline-capture-apply-request@1.0.0"
            | "handbook.operation.pipeline-handoff-emit-request@1.0.0"
            | "handbook.operation.pipeline-state-apply-request@1.0.0"
            | "handbook.operation.contract-definition-append-request@1.0.0"
            | "handbook.operation.contract-evidence-append-request@1.0.0"
            | "handbook.operation.contract-lifecycle-transition-request@1.0.0"
            | "handbook.operation.dock-run-request@1.0.0"
    ) {
        validate_posture_request_raw_key_schema(value, descriptor.relative_path)
    } else {
        validate_no_raw_key_names(value, descriptor.relative_path)
    }
}

fn validate_local_references(
    root: &serde_json::Value,
    current: &serde_json::Value,
    relative_path: &str,
) -> Result<(), String> {
    match current {
        serde_json::Value::Array(values) => {
            for value in values {
                validate_local_references(root, value, relative_path)?;
            }
        }
        serde_json::Value::Object(members) => {
            if let Some(reference) = members.get("$ref") {
                let reference = reference
                    .as_str()
                    .ok_or_else(|| format!("{relative_path} has a non-string ref"))?;
                let pointer = reference
                    .strip_prefix('#')
                    .filter(|pointer| pointer.starts_with("/$defs/"))
                    .ok_or_else(|| format!("{relative_path} has a non-local ref"))?;
                if root.pointer(pointer).is_none() {
                    return Err(format!("{relative_path} has an unresolved local ref"));
                }
            }
            for value in members.values() {
                validate_local_references(root, value, relative_path)?;
            }
        }
        _ => {}
    }
    Ok(())
}

fn validate_closed_objects(value: &serde_json::Value, relative_path: &str) -> Result<(), String> {
    match value {
        serde_json::Value::Array(values) => {
            for value in values {
                validate_closed_objects(value, relative_path)?;
            }
        }
        serde_json::Value::Object(members) => {
            if members.get("type").and_then(serde_json::Value::as_str) == Some("object")
                && members.contains_key("properties")
                && members.get("additionalProperties") != Some(&serde_json::Value::Bool(false))
            {
                return Err(format!(
                    "{relative_path} contains an open structural object"
                ));
            }
            for value in members.values() {
                validate_closed_objects(value, relative_path)?;
            }
        }
        _ => {}
    }
    Ok(())
}

fn validate_posture_request_raw_key_schema(
    value: &serde_json::Value,
    relative_path: &str,
) -> Result<(), String> {
    let properties = value
        .get("properties")
        .and_then(serde_json::Value::as_object)
        .ok_or_else(|| format!("{relative_path} has no root properties"))?;
    let required = value
        .get("required")
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| format!("{relative_path} has no root required set"))?;
    if !properties.contains_key("idempotency_key")
        || required
            .iter()
            .filter(|member| member.as_str() == Some("idempotency_key"))
            .count()
            != 1
        || raw_key_name_occurrences(value) != 2
    {
        return Err(format!(
            "{relative_path} must name the raw key only as one required root wire member"
        ));
    }
    Ok(())
}

fn raw_key_name_occurrences(value: &serde_json::Value) -> usize {
    match value {
        serde_json::Value::Array(values) => values.iter().map(raw_key_name_occurrences).sum(),
        serde_json::Value::Object(members) => members
            .iter()
            .map(|(key, value)| {
                usize::from(matches!(
                    key.as_str(),
                    "idempotency_key" | "raw_idempotency_key"
                )) + raw_key_name_occurrences(value)
            })
            .sum(),
        serde_json::Value::String(value)
            if matches!(value.as_str(), "idempotency_key" | "raw_idempotency_key") =>
        {
            1
        }
        _ => 0,
    }
}

fn validate_no_raw_key_names(value: &serde_json::Value, relative_path: &str) -> Result<(), String> {
    match value {
        serde_json::Value::Array(values) => {
            for value in values {
                validate_no_raw_key_names(value, relative_path)?;
            }
        }
        serde_json::Value::Object(members) => {
            for (key, value) in members {
                if matches!(key.as_str(), "idempotency_key" | "raw_idempotency_key") {
                    return Err(format!("{relative_path} contains raw key material"));
                }
                validate_no_raw_key_names(value, relative_path)?;
            }
        }
        serde_json::Value::String(value)
            if matches!(value.as_str(), "idempotency_key" | "raw_idempotency_key") =>
        {
            return Err(format!("{relative_path} contains raw key material"));
        }
        _ => {}
    }
    Ok(())
}

fn validate_schema_bytes(
    schema: &Schema,
    bytes: &[u8],
    limits: &ContractLimits,
) -> Result<(), String> {
    if !bytes.ends_with(b"\n") || bytes.len() < 2 || bytes[bytes.len() - 2] == b'\n' {
        return Err("generated schema must end in exactly one LF".to_owned());
    }
    let parsed = parse_canonical_json::<serde_json::Value>(&bytes[..bytes.len() - 1], limits)
        .map_err(|_| "generated schema bytes are not canonical".to_owned())?;
    if &parsed != schema.as_value() {
        return Err("generated schema bytes changed their JSON value".to_owned());
    }
    Ok(())
}

fn write_tree(root: &Path, generated: &GeneratedTree) -> Result<(), String> {
    for (relative_path, bytes) in &generated.schemas {
        let path = root.join("2020-12").join(relative_path);
        let parent = path
            .parent()
            .ok_or_else(|| "generated schema path has no parent".to_owned())?;
        fs::create_dir_all(parent).map_err(io_error("create temporary schema directory"))?;
        fs::write(path, bytes).map_err(io_error("write temporary schema"))?;
    }
    fs::create_dir_all(root).map_err(io_error("create temporary schema root"))?;
    fs::write(root.join("manifest.json"), &generated.manifest)
        .map_err(io_error("write temporary manifest"))
}

fn compare_tree(temporary_root: &Path, tracked_root: &Path) -> Result<(), String> {
    let temporary = read_relative_files(&temporary_root.join("2020-12"))?;
    let tracked = read_relative_files(&tracked_root.join("2020-12"))?;
    if temporary != tracked {
        return Err("tracked Draft 2020-12 schema tree is not byte-identical".to_owned());
    }
    let temporary_manifest = fs::read(temporary_root.join("manifest.json"))
        .map_err(io_error("read temporary manifest"))?;
    let tracked_manifest =
        fs::read(tracked_root.join("manifest.json")).map_err(io_error("read tracked manifest"))?;
    if temporary_manifest != tracked_manifest {
        return Err("tracked schema manifest is not byte-identical".to_owned());
    }
    Ok(())
}

fn read_relative_files(root: &Path) -> Result<BTreeMap<PathBuf, Vec<u8>>, String> {
    fn visit(
        root: &Path,
        current: &Path,
        files: &mut BTreeMap<PathBuf, Vec<u8>>,
    ) -> Result<(), String> {
        for entry in fs::read_dir(current).map_err(io_error("read schema directory"))? {
            let entry = entry.map_err(io_error("read schema directory entry"))?;
            let path = entry.path();
            if path.is_dir() {
                visit(root, &path, files)?;
            } else {
                let relative = path
                    .strip_prefix(root)
                    .map_err(|_| "schema path escaped its root".to_owned())?
                    .to_path_buf();
                let bytes = fs::read(&path).map_err(io_error("read schema file"))?;
                files.insert(relative, bytes);
            }
        }
        Ok(())
    }

    let mut files = BTreeMap::new();
    visit(root, root, &mut files)?;
    Ok(files)
}

fn replace_tracked_tree(temporary_root: &Path, tracked_root: &Path) -> Result<(), String> {
    let tracked_schemas = tracked_root.join("2020-12");
    if tracked_schemas.exists() {
        fs::remove_dir_all(&tracked_schemas).map_err(io_error("remove tracked schema tree"))?;
    }
    fs::create_dir_all(&tracked_schemas).map_err(io_error("create tracked schema tree"))?;
    for (relative, bytes) in read_relative_files(&temporary_root.join("2020-12"))? {
        let path = tracked_schemas.join(relative);
        let parent = path
            .parent()
            .ok_or_else(|| "tracked schema path has no parent".to_owned())?;
        fs::create_dir_all(parent).map_err(io_error("create tracked schema directory"))?;
        fs::write(path, bytes).map_err(io_error("write tracked schema"))?;
    }
    fs::create_dir_all(tracked_root).map_err(io_error("create tracked schema root"))?;
    fs::copy(
        temporary_root.join("manifest.json"),
        tracked_root.join("manifest.json"),
    )
    .map_err(io_error("write tracked manifest"))?;
    Ok(())
}

fn io_error(context: &'static str) -> impl FnOnce(std::io::Error) -> String {
    move |_| context.to_owned()
}
