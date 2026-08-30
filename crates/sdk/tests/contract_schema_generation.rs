use handbook_sdk::contract_membrane::{
    canonical_json_bytes, parse_canonical_json, sha256_fingerprint, ArtifactLocator, ArtifactRef,
    BootstrapOutcome, BootstrapRefusalDetails, BootstrapRefusalReason, BootstrapRequestV1,
    BoundedString, BoundedVec, CatalogCursor, CatalogPage, CatalogPageSize, CatalogRoot,
    ContractError, ContractLimits, ContractValue, ContractWitness, CorrelationRef, Diagnostic,
    DiagnosticSeverity, DraftVersion, ExactBinding, ExactRef, Fingerprint, IdempotencyStage,
    IdempotencyState, NextAction, NextActionKind, Omission, OmissionReason,
    OperationAdmissionRefusalV1, OperationClass, OperationDefinition, OperationIdempotency,
    OperationOutcomeSchemas, OperationOwnerBinding, OperationRequestV1, OperationResponseV1,
    OperationSchemaValidator, OperationTransport, Problem, ProblemBinding, ProblemCategory,
    ProductionCatalogRead, ProductionWitness, ProofEffect, RecheckCondition, RequestId,
    RetryDirective, SafeU64, SchemaManifestEntry, SchemaMediaType, SemVer, Sensitivity,
    SourceBinding, TerminalFields, TerminalStatus, UtcTimestamp, VerificationError, Verified,
    WriteReceipt,
};
use jsonschema::{PatternOptions, Retrieve, Uri, Validator};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

const ROOTS: [(&str, &str); 180] = [
    (
        "handbook.artifact-operation-details@1.0.0",
        "handbook.artifact-operation-details/1.0.0.schema.json",
    ),
    (
        "handbook.artifact-recheck.repository-authority@1.0.0",
        "handbook.artifact-recheck.repository-authority/1.0.0.schema.json",
    ),
    (
        "handbook.artifact-recheck.supported-platform@1.0.0",
        "handbook.artifact-recheck.supported-platform/1.0.0.schema.json",
    ),
    (
        "handbook.artifact-ref@1.0.0",
        "handbook.artifact-ref/1.0.0.schema.json",
    ),
    (
        "handbook.artifact-write-condition.canonical-truth@1.0.0",
        "handbook.artifact-write-condition.canonical-truth/1.0.0.schema.json",
    ),
    (
        "handbook.artifact-write-condition.semantic-record@1.0.0",
        "handbook.artifact-write-condition.semantic-record/1.0.0.schema.json",
    ),
    (
        "handbook.bootstrap-descriptor-schema@1.0.0",
        "handbook.bootstrap-descriptor-schema/1.0.0.schema.json",
    ),
    (
        "handbook.bootstrap-facade-response@1.0.0",
        "handbook.bootstrap-facade-response/1.0.0.schema.json",
    ),
    (
        "handbook.bootstrap-refusal-details@1.0.0",
        "handbook.bootstrap-refusal-details/1.0.0.schema.json",
    ),
    (
        "handbook.bootstrap-refusal@1.0.0",
        "handbook.bootstrap-refusal/1.0.0.schema.json",
    ),
    (
        "handbook.bootstrap-request@1.0.0",
        "handbook.bootstrap-request/1.0.0.schema.json",
    ),
    (
        "handbook.bootstrap-response@1.0.0",
        "handbook.bootstrap-response/1.0.0.schema.json",
    ),
    (
        "handbook.capability-catalog-page@1.0.0",
        "handbook.capability-catalog-page/1.0.0.schema.json",
    ),
    (
        "handbook.capability-entry@1.0.0",
        "handbook.capability-entry/1.0.0.schema.json",
    ),
    (
        "handbook.catalog-cursor@1.0.0",
        "handbook.catalog-cursor/1.0.0.schema.json",
    ),
    (
        "handbook.catalog-page@1.0.0",
        "handbook.catalog-page/1.0.0.schema.json",
    ),
    (
        "handbook.catalog-read-blocked@1.0.0",
        "handbook.catalog-read-blocked/1.0.0.schema.json",
    ),
    (
        "handbook.catalog-read-request@1.0.0",
        "handbook.catalog-read-request/1.0.0.schema.json",
    ),
    (
        "handbook.catalog-read-response@1.0.0",
        "handbook.catalog-read-response/1.0.0.schema.json",
    ),
    (
        "handbook.catalog-root@1.0.0",
        "handbook.catalog-root/1.0.0.schema.json",
    ),
    (
        "handbook.catalog-roots@1.0.0",
        "handbook.catalog-roots/1.0.0.schema.json",
    ),
    (
        "handbook.contract-applicability-outcome@1.0.0",
        "handbook.contract-applicability-outcome/1.0.0.schema.json",
    ),
    (
        "handbook.contract-claim-verdict@1.0.0",
        "handbook.contract-claim-verdict/1.0.0.schema.json",
    ),
    (
        "handbook.contract-claim@1.0.0",
        "handbook.contract-claim/1.0.0.schema.json",
    ),
    (
        "handbook.contract-definition@1.0.0",
        "handbook.contract-definition/1.0.0.schema.json",
    ),
    (
        "handbook.contract-evaluation-lifecycle-basis@1.0.0",
        "handbook.contract-evaluation-lifecycle-basis/1.0.0.schema.json",
    ),
    (
        "handbook.contract-evaluation-run@1.0.0",
        "handbook.contract-evaluation-run/1.0.0.schema.json",
    ),
    (
        "handbook.contract-evidence-admission-policy@1.0.0",
        "handbook.contract-evidence-admission-policy/1.0.0.schema.json",
    ),
    (
        "handbook.contract-evidence-candidate@1.0.0",
        "handbook.contract-evidence-candidate/1.0.0.schema.json",
    ),
    (
        "handbook.contract-evidence-freshness-policy@1.0.0",
        "handbook.contract-evidence-freshness-policy/1.0.0.schema.json",
    ),
    (
        "handbook.contract-evidence-operation-details@1.0.0",
        "handbook.contract-evidence-operation-details/1.0.0.schema.json",
    ),
    (
        "handbook.contract-evidence-record@1.0.0",
        "handbook.contract-evidence-record/1.0.0.schema.json",
    ),
    (
        "handbook.contract-evidence-stability-policy@1.0.0",
        "handbook.contract-evidence-stability-policy/1.0.0.schema.json",
    ),
    (
        "handbook.contract-gate-policy@1.0.0",
        "handbook.contract-gate-policy/1.0.0.schema.json",
    ),
    (
        "handbook.contract-gate-result@1.0.0",
        "handbook.contract-gate-result/1.0.0.schema.json",
    ),
    (
        "handbook.contract-local-closeout-policy@1.0.0",
        "handbook.contract-local-closeout-policy/1.0.0.schema.json",
    ),
    (
        "handbook.contract-local-closeout-result@1.0.0",
        "handbook.contract-local-closeout-result/1.0.0.schema.json",
    ),
    (
        "handbook.contract-lifecycle-policy@1.0.0",
        "handbook.contract-lifecycle-policy/1.0.0.schema.json",
    ),
    (
        "handbook.contract-lifecycle-transition@1.0.0",
        "handbook.contract-lifecycle-transition/1.0.0.schema.json",
    ),
    (
        "handbook.contract-operation-details@1.0.0",
        "handbook.contract-operation-details/1.0.0.schema.json",
    ),
    (
        "handbook.correlation-ref@1.0.0",
        "handbook.correlation-ref/1.0.0.schema.json",
    ),
    (
        "handbook.diagnostic@1.0.0",
        "handbook.diagnostic/1.0.0.schema.json",
    ),
    (
        "handbook.flow-operation-details@1.0.0",
        "handbook.flow-operation-details/1.0.0.schema.json",
    ),
    (
        "handbook.flow-recheck@1.0.0",
        "handbook.flow-recheck/1.0.0.schema.json",
    ),
    (
        "handbook.idempotency-state@1.0.0",
        "handbook.idempotency-state/1.0.0.schema.json",
    ),
    (
        "handbook.invocation-request@1.0.0",
        "handbook.invocation-request/1.0.0.schema.json",
    ),
    (
        "handbook.invocation-response@1.0.0",
        "handbook.invocation-response/1.0.0.schema.json",
    ),
    (
        "handbook.next-action@1.0.0",
        "handbook.next-action/1.0.0.schema.json",
    ),
    (
        "handbook.omission@1.0.0",
        "handbook.omission/1.0.0.schema.json",
    ),
    (
        "handbook.operation-admission-refusal@1.0.0",
        "handbook.operation-admission-refusal/1.0.0.schema.json",
    ),
    (
        "handbook.operation-blocker@1.0.0",
        "handbook.operation-blocker/1.0.0.schema.json",
    ),
    (
        "handbook.operation-definition@1.0.0",
        "handbook.operation-definition/1.0.0.schema.json",
    ),
    (
        "handbook.operation-error@1.0.0",
        "handbook.operation-error/1.0.0.schema.json",
    ),
    (
        "handbook.operation-refusal@1.0.0",
        "handbook.operation-refusal/1.0.0.schema.json",
    ),
    (
        "handbook.operation-request@1.0.0",
        "handbook.operation-request/1.0.0.schema.json",
    ),
    (
        "handbook.operation-response@1.0.0",
        "handbook.operation-response/1.0.0.schema.json",
    ),
    (
        "handbook.operation.artifact-candidate-append-request@1.0.0",
        "handbook.operation.artifact-candidate-append-request/1.0.0.schema.json",
    ),
    (
        "handbook.operation.artifact-candidate-append-result@1.0.0",
        "handbook.operation.artifact-candidate-append-result/1.0.0.schema.json",
    ),
    (
        "handbook.operation.artifact-candidate-promote-request@1.0.0",
        "handbook.operation.artifact-candidate-promote-request/1.0.0.schema.json",
    ),
    (
        "handbook.operation.artifact-candidate-promote-result@1.0.0",
        "handbook.operation.artifact-candidate-promote-result/1.0.0.schema.json",
    ),
    (
        "handbook.operation.artifact-candidate-validate-request@1.0.0",
        "handbook.operation.artifact-candidate-validate-request/1.0.0.schema.json",
    ),
    (
        "handbook.operation.artifact-candidate-validate-result@1.0.0",
        "handbook.operation.artifact-candidate-validate-result/1.0.0.schema.json",
    ),
    (
        "handbook.operation.artifact-instance-list-request@1.0.0",
        "handbook.operation.artifact-instance-list-request/1.0.0.schema.json",
    ),
    (
        "handbook.operation.artifact-instance-list-result@1.0.0",
        "handbook.operation.artifact-instance-list-result/1.0.0.schema.json",
    ),
    (
        "handbook.operation.artifact-kind-list-request@1.0.0",
        "handbook.operation.artifact-kind-list-request/1.0.0.schema.json",
    ),
    (
        "handbook.operation.artifact-kind-list-result@1.0.0",
        "handbook.operation.artifact-kind-list-result/1.0.0.schema.json",
    ),
    (
        "handbook.operation.artifact-read-request@1.0.0",
        "handbook.operation.artifact-read-request/1.0.0.schema.json",
    ),
    (
        "handbook.operation.artifact-read-result@1.0.0",
        "handbook.operation.artifact-read-result/1.0.0.schema.json",
    ),
    (
        "handbook.operation.artifact-validate-request@1.0.0",
        "handbook.operation.artifact-validate-request/1.0.0.schema.json",
    ),
    (
        "handbook.operation.artifact-validate-result@1.0.0",
        "handbook.operation.artifact-validate-result/1.0.0.schema.json",
    ),
    (
        "handbook.operation.contract-definition-append-request@1.0.0",
        "handbook.operation.contract-definition-append-request/1.0.0.schema.json",
    ),
    (
        "handbook.operation.contract-definition-append-result@1.0.0",
        "handbook.operation.contract-definition-append-result/1.0.0.schema.json",
    ),
    (
        "handbook.operation.contract-definition-list-request@1.0.0",
        "handbook.operation.contract-definition-list-request/1.0.0.schema.json",
    ),
    (
        "handbook.operation.contract-definition-list-result@1.0.0",
        "handbook.operation.contract-definition-list-result/1.0.0.schema.json",
    ),
    (
        "handbook.operation.contract-definition-read-request@1.0.0",
        "handbook.operation.contract-definition-read-request/1.0.0.schema.json",
    ),
    (
        "handbook.operation.contract-definition-read-result@1.0.0",
        "handbook.operation.contract-definition-read-result/1.0.0.schema.json",
    ),
    (
        "handbook.operation.contract-evidence-append-request@1.0.0",
        "handbook.operation.contract-evidence-append-request/1.0.0.schema.json",
    ),
    (
        "handbook.operation.contract-evidence-append-result@1.0.0",
        "handbook.operation.contract-evidence-append-result/1.0.0.schema.json",
    ),
    (
        "handbook.operation.contract-evidence-list-request@1.0.0",
        "handbook.operation.contract-evidence-list-request/1.0.0.schema.json",
    ),
    (
        "handbook.operation.contract-evidence-list-result@1.0.0",
        "handbook.operation.contract-evidence-list-result/1.0.0.schema.json",
    ),
    (
        "handbook.operation.contract-evidence-read-request@1.0.0",
        "handbook.operation.contract-evidence-read-request/1.0.0.schema.json",
    ),
    (
        "handbook.operation.contract-evidence-read-result@1.0.0",
        "handbook.operation.contract-evidence-read-result/1.0.0.schema.json",
    ),
    (
        "handbook.operation.contract-gate-evaluate-request@1.0.0",
        "handbook.operation.contract-gate-evaluate-request/1.0.0.schema.json",
    ),
    (
        "handbook.operation.contract-gate-evaluate-result@1.0.0",
        "handbook.operation.contract-gate-evaluate-result/1.0.0.schema.json",
    ),
    (
        "handbook.operation.contract-lifecycle-transition-request@1.0.0",
        "handbook.operation.contract-lifecycle-transition-request/1.0.0.schema.json",
    ),
    (
        "handbook.operation.contract-lifecycle-transition-result@1.0.0",
        "handbook.operation.contract-lifecycle-transition-result/1.0.0.schema.json",
    ),
    (
        "handbook.operation.contract-verdict-evaluate-request@1.0.0",
        "handbook.operation.contract-verdict-evaluate-request/1.0.0.schema.json",
    ),
    (
        "handbook.operation.contract-verdict-evaluate-result@1.0.0",
        "handbook.operation.contract-verdict-evaluate-result/1.0.0.schema.json",
    ),
    (
        "handbook.operation.flow-resolve-request@1.0.0",
        "handbook.operation.flow-resolve-request/1.0.0.schema.json",
    ),
    (
        "handbook.operation.flow-resolve-result@1.0.0",
        "handbook.operation.flow-resolve-result/1.0.0.schema.json",
    ),
    (
        "handbook.operation.intake-coverage-evaluate-request@1.0.0",
        "handbook.operation.intake-coverage-evaluate-request/1.0.0.schema.json",
    ),
    (
        "handbook.operation.intake-coverage-evaluate-result@1.0.0",
        "handbook.operation.intake-coverage-evaluate-result/1.0.0.schema.json",
    ),
    (
        "handbook.operation.intake-definition-read-request@1.0.0",
        "handbook.operation.intake-definition-read-request/1.0.0.schema.json",
    ),
    (
        "handbook.operation.intake-definition-read-result@1.0.0",
        "handbook.operation.intake-definition-read-result/1.0.0.schema.json",
    ),
    (
        "handbook.operation.intake-record-append-request@1.0.0",
        "handbook.operation.intake-record-append-request/1.0.0.schema.json",
    ),
    (
        "handbook.operation.intake-record-append-result@1.0.0",
        "handbook.operation.intake-record-append-result/1.0.0.schema.json",
    ),
    (
        "handbook.operation.pipeline-capture-apply-request@1.0.0",
        "handbook.operation.pipeline-capture-apply-request/1.0.0.schema.json",
    ),
    (
        "handbook.operation.pipeline-capture-apply-result@1.0.0",
        "handbook.operation.pipeline-capture-apply-result/1.0.0.schema.json",
    ),
    (
        "handbook.operation.pipeline-capture-plan-request@1.0.0",
        "handbook.operation.pipeline-capture-plan-request/1.0.0.schema.json",
    ),
    (
        "handbook.operation.pipeline-capture-plan-result@1.0.0",
        "handbook.operation.pipeline-capture-plan-result/1.0.0.schema.json",
    ),
    (
        "handbook.operation.pipeline-catalog-list-request@1.0.0",
        "handbook.operation.pipeline-catalog-list-request/1.0.0.schema.json",
    ),
    (
        "handbook.operation.pipeline-catalog-list-result@1.0.0",
        "handbook.operation.pipeline-catalog-list-result/1.0.0.schema.json",
    ),
    (
        "handbook.operation.pipeline-catalog-read-request@1.0.0",
        "handbook.operation.pipeline-catalog-read-request/1.0.0.schema.json",
    ),
    (
        "handbook.operation.pipeline-catalog-read-result@1.0.0",
        "handbook.operation.pipeline-catalog-read-result/1.0.0.schema.json",
    ),
    (
        "handbook.operation.pipeline-compile-request@1.0.0",
        "handbook.operation.pipeline-compile-request/1.0.0.schema.json",
    ),
    (
        "handbook.operation.pipeline-compile-result@1.0.0",
        "handbook.operation.pipeline-compile-result/1.0.0.schema.json",
    ),
    (
        "handbook.operation.pipeline-handoff-emit-request@1.0.0",
        "handbook.operation.pipeline-handoff-emit-request/1.0.0.schema.json",
    ),
    (
        "handbook.operation.pipeline-handoff-emit-result@1.0.0",
        "handbook.operation.pipeline-handoff-emit-result/1.0.0.schema.json",
    ),
    (
        "handbook.operation.pipeline-state-apply-request@1.0.0",
        "handbook.operation.pipeline-state-apply-request/1.0.0.schema.json",
    ),
    (
        "handbook.operation.pipeline-state-apply-result@1.0.0",
        "handbook.operation.pipeline-state-apply-result/1.0.0.schema.json",
    ),
    (
        "handbook.operation.posture-transition-apply-request@1.0.0",
        "handbook.operation.posture-transition-apply-request/1.0.0.schema.json",
    ),
    (
        "handbook.operation.posture-transition-apply-result@1.0.0",
        "handbook.operation.posture-transition-apply-result/1.0.0.schema.json",
    ),
    (
        "handbook.operation.snapshot-capture-request@1.0.0",
        "handbook.operation.snapshot-capture-request/1.0.0.schema.json",
    ),
    (
        "handbook.operation.snapshot-capture-result@1.0.0",
        "handbook.operation.snapshot-capture-result/1.0.0.schema.json",
    ),
    (
        "handbook.operation.snapshot-delta-request@1.0.0",
        "handbook.operation.snapshot-delta-request/1.0.0.schema.json",
    ),
    (
        "handbook.operation.snapshot-delta-result@1.0.0",
        "handbook.operation.snapshot-delta-result/1.0.0.schema.json",
    ),
    (
        "handbook.operation.snapshot-project-request@1.0.0",
        "handbook.operation.snapshot-project-request/1.0.0.schema.json",
    ),
    (
        "handbook.operation.snapshot-project-result@1.0.0",
        "handbook.operation.snapshot-project-result/1.0.0.schema.json",
    ),
    (
        "handbook.operation.snapshot-read-request@1.0.0",
        "handbook.operation.snapshot-read-request/1.0.0.schema.json",
    ),
    (
        "handbook.operation.snapshot-read-result@1.0.0",
        "handbook.operation.snapshot-read-result/1.0.0.schema.json",
    ),
    (
        "handbook.owner-version-entry@1.0.0",
        "handbook.owner-version-entry/1.0.0.schema.json",
    ),
    (
        "handbook.pipeline-operation-details@1.0.0",
        "handbook.pipeline-operation-details/1.0.0.schema.json",
    ),
    (
        "handbook.pipeline-recheck@1.0.0",
        "handbook.pipeline-recheck/1.0.0.schema.json",
    ),
    (
        "handbook.pipeline-write-condition@1.0.0",
        "handbook.pipeline-write-condition/1.0.0.schema.json",
    ),
    (
        "handbook.posture-details.authority-conflict@1.0.0",
        "handbook.posture-details.authority-conflict/1.0.0.schema.json",
    ),
    (
        "handbook.posture-details.integrity-or-durability@1.0.0",
        "handbook.posture-details.integrity-or-durability/1.0.0.schema.json",
    ),
    (
        "handbook.posture-details.invalid-fixed-dimension@1.0.0",
        "handbook.posture-details.invalid-fixed-dimension/1.0.0.schema.json",
    ),
    (
        "handbook.posture-details.invalid-idempotency-key@1.0.0",
        "handbook.posture-details.invalid-idempotency-key/1.0.0.schema.json",
    ),
    (
        "handbook.posture-details.invalid-semantic-input@1.0.0",
        "handbook.posture-details.invalid-semantic-input/1.0.0.schema.json",
    ),
    (
        "handbook.posture-details.io@1.0.0",
        "handbook.posture-details.io/1.0.0.schema.json",
    ),
    (
        "handbook.posture-details.lifecycle-not-current@1.0.0",
        "handbook.posture-details.lifecycle-not-current/1.0.0.schema.json",
    ),
    (
        "handbook.posture-details.missing-lifecycle-authority@1.0.0",
        "handbook.posture-details.missing-lifecycle-authority/1.0.0.schema.json",
    ),
    (
        "handbook.posture-details.repository-identity-mismatch@1.0.0",
        "handbook.posture-details.repository-identity-mismatch/1.0.0.schema.json",
    ),
    (
        "handbook.posture-details.stale-canonical@1.0.0",
        "handbook.posture-details.stale-canonical/1.0.0.schema.json",
    ),
    (
        "handbook.posture-details.unsafe-filesystem@1.0.0",
        "handbook.posture-details.unsafe-filesystem/1.0.0.schema.json",
    ),
    (
        "handbook.posture-details.unsupported-platform@1.0.0",
        "handbook.posture-details.unsupported-platform/1.0.0.schema.json",
    ),
    (
        "handbook.posture-recheck.unsupported-platform@1.0.0",
        "handbook.posture-recheck.unsupported-platform/1.0.0.schema.json",
    ),
    (
        "handbook.posture-write-condition.canonical-truth@1.0.0",
        "handbook.posture-write-condition.canonical-truth/1.0.0.schema.json",
    ),
    (
        "handbook.posture-write-condition.semantic-record@1.0.0",
        "handbook.posture-write-condition.semantic-record/1.0.0.schema.json",
    ),
    (
        "handbook.problem-binding@1.0.0",
        "handbook.problem-binding/1.0.0.schema.json",
    ),
    (
        "handbook.problem@1.0.0",
        "handbook.problem/1.0.0.schema.json",
    ),
    (
        "handbook.profile-catalog-entry@1.0.0",
        "handbook.profile-catalog-entry/1.0.0.schema.json",
    ),
    (
        "handbook.profile-catalog-page@1.0.0",
        "handbook.profile-catalog-page/1.0.0.schema.json",
    ),
    (
        "handbook.recheck-condition@1.0.0",
        "handbook.recheck-condition/1.0.0.schema.json",
    ),
    (
        "handbook.schema-catalog-page@1.0.0",
        "handbook.schema-catalog-page/1.0.0.schema.json",
    ),
    (
        "handbook.schema-manifest-entry@1.0.0",
        "handbook.schema-manifest-entry/1.0.0.schema.json",
    ),
    (
        "handbook.source-binding@1.0.0",
        "handbook.source-binding/1.0.0.schema.json",
    ),
    (
        "handbook.transport-failure-details@1.0.0",
        "handbook.transport-failure-details/1.0.0.schema.json",
    ),
    (
        "handbook.transport-failure@1.0.0",
        "handbook.transport-failure/1.0.0.schema.json",
    ),
    (
        "handbook.typed-catalog-cursor@1.0.0",
        "handbook.typed-catalog-cursor/1.0.0.schema.json",
    ),
    (
        "handbook.use-case-request@1.0.0",
        "handbook.use-case-request/1.0.0.schema.json",
    ),
    (
        "handbook.use-case-response@1.0.0",
        "handbook.use-case-response/1.0.0.schema.json",
    ),
    (
        "handbook.use-case.approver-administration-details@1.0.0",
        "handbook.use-case.approver-administration-details/1.0.0.schema.json",
    ),
    (
        "handbook.use-case.approver-administration-result@1.0.0",
        "handbook.use-case.approver-administration-result/1.0.0.schema.json",
    ),
    (
        "handbook.use-case.author-charter-details@1.0.0",
        "handbook.use-case.author-charter-details/1.0.0.schema.json",
    ),
    (
        "handbook.use-case.author-charter-result@1.0.0",
        "handbook.use-case.author-charter-result/1.0.0.schema.json",
    ),
    (
        "handbook.use-case.author-project-context-details@1.0.0",
        "handbook.use-case.author-project-context-details/1.0.0.schema.json",
    ),
    (
        "handbook.use-case.author-project-context-result@1.0.0",
        "handbook.use-case.author-project-context-result/1.0.0.schema.json",
    ),
    (
        "handbook.use-case.pipeline-route-resolve-and-persist-details@1.0.0",
        "handbook.use-case.pipeline-route-resolve-and-persist-details/1.0.0.schema.json",
    ),
    (
        "handbook.use-case.pipeline-route-resolve-and-persist-result@1.0.0",
        "handbook.use-case.pipeline-route-resolve-and-persist-result/1.0.0.schema.json",
    ),
    (
        "handbook.use-case.repository-doctor-details@1.0.0",
        "handbook.use-case.repository-doctor-details/1.0.0.schema.json",
    ),
    (
        "handbook.use-case.repository-doctor-result@1.0.0",
        "handbook.use-case.repository-doctor-result/1.0.0.schema.json",
    ),
    (
        "handbook.use-case.repository-setup-details@1.0.0",
        "handbook.use-case.repository-setup-details/1.0.0.schema.json",
    ),
    (
        "handbook.use-case.repository-setup-result@1.0.0",
        "handbook.use-case.repository-setup-result/1.0.0.schema.json",
    ),
    (
        "handbook.use-case.session-grounding-boundary-details@1.0.0",
        "handbook.use-case.session-grounding-boundary-details/1.0.0.schema.json",
    ),
    (
        "handbook.use-case.session-grounding-boundary-result@1.0.0",
        "handbook.use-case.session-grounding-boundary-result/1.0.0.schema.json",
    ),
    (
        "handbook.write-receipt@1.0.0",
        "handbook.write-receipt/1.0.0.schema.json",
    ),
    (
        "handbook.dock-execution-record@1.0.0",
        "handbook.dock-execution-record/1.0.0.schema.json",
    ),
    (
        "handbook.dock-json-schema-validation-input@1.0.0",
        "handbook.dock-json-schema-validation-input/1.0.0.schema.json",
    ),
    (
        "handbook.dock-manifest@1.0.0",
        "handbook.dock-manifest/1.0.0.schema.json",
    ),
    (
        "handbook.dock-operation-details@1.0.0",
        "handbook.dock-operation-details/1.0.0.schema.json",
    ),
    (
        "handbook.dock-process-request@1.0.0",
        "handbook.dock-process-request/1.0.0.schema.json",
    ),
    (
        "handbook.dock-process-result@1.0.0",
        "handbook.dock-process-result/1.0.0.schema.json",
    ),
    (
        "handbook.fact.schema-conformance@1.0.0",
        "handbook.fact.schema-conformance/1.0.0.schema.json",
    ),
    (
        "handbook.operation.dock-manifest-list-request@1.0.0",
        "handbook.operation.dock-manifest-list-request/1.0.0.schema.json",
    ),
    (
        "handbook.operation.dock-manifest-list-result@1.0.0",
        "handbook.operation.dock-manifest-list-result/1.0.0.schema.json",
    ),
    (
        "handbook.operation.dock-manifest-read-request@1.0.0",
        "handbook.operation.dock-manifest-read-request/1.0.0.schema.json",
    ),
    (
        "handbook.operation.dock-manifest-read-result@1.0.0",
        "handbook.operation.dock-manifest-read-result/1.0.0.schema.json",
    ),
    (
        "handbook.operation.dock-run-request@1.0.0",
        "handbook.operation.dock-run-request/1.0.0.schema.json",
    ),
    (
        "handbook.operation.dock-run-result@1.0.0",
        "handbook.operation.dock-run-result/1.0.0.schema.json",
    ),
];

const DIALECT: &str = "https://json-schema.org/draft/2020-12/schema";
const ZERO_FINGERPRINT: &str =
    "sha256:0000000000000000000000000000000000000000000000000000000000000000";
type IdempotencyVerifier = fn(
    IdempotencyState,
    &dyn ContractWitness,
    &ContractLimits,
) -> Result<Verified<IdempotencyState>, VerificationError>;

#[derive(Debug)]
struct RejectExternal;

#[derive(Debug)]
struct RetrievalDisabled;

impl fmt::Display for RetrievalDisabled {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("external schema retrieval is disabled")
    }
}

impl std::error::Error for RetrievalDisabled {}

impl Retrieve for RejectExternal {
    fn retrieve(
        &self,
        _uri: &Uri<String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
        Err(Box::new(RetrievalDisabled))
    }
}

#[test]
fn checked_in_tree_contains_exactly_the_authorized_roots() {
    let schema_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("schemas/2020-12");
    let actual = schema_files(&schema_root);
    let expected = ROOTS
        .iter()
        .map(|(_, relative_path)| {
            PathBuf::from(relative_path.replace('/', std::path::MAIN_SEPARATOR_STR))
        })
        .collect::<BTreeSet<_>>();

    assert_eq!(actual, expected);
    assert!(Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("schemas/manifest.json")
        .is_file());
}

#[test]
fn schemas_are_meta_valid_self_contained_closed_and_canonical() {
    for (exact_ref, relative_path) in ROOTS {
        let bytes = read_schema(relative_path);
        assert_one_terminal_lf(&bytes, relative_path);
        let without_lf = &bytes[..bytes.len() - 1];
        let schema = parse_canonical_json::<Value>(without_lf, &ContractLimits::packet0())
            .expect("schema bytes must be Packet-0 canonical JSON");
        assert_eq!(
            canonical_json_bytes(&schema, &ContractLimits::packet0()).unwrap(),
            without_lf
        );
        assert_eq!(schema["$schema"], DIALECT);
        let (identity, version) = exact_ref.split_once('@').unwrap();
        assert_eq!(
            schema["$id"],
            format!("urn:handbook:schema:{identity}:{version}")
        );
        jsonschema::draft202012::meta::validate(&schema)
            .expect("schema must validate against the bundled Draft 2020-12 meta-schema");
        assert_local_refs_resolve(&schema, &schema);
        assert_structural_objects_are_closed(&schema);

        jsonschema::draft202012::options()
            .with_pattern_options(PatternOptions::regex())
            .with_retriever(RejectExternal)
            .build(&schema)
            .expect("self-contained schema must compile with external retrieval disabled");
    }
}

#[test]
fn manifest_exactly_binds_sorted_refs_lengths_and_checked_in_bytes() {
    let manifest_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("schemas/manifest.json");
    let bytes = fs::read(&manifest_path).unwrap();
    assert_one_terminal_lf(&bytes, "manifest.json");
    let without_lf = &bytes[..bytes.len() - 1];
    let value = parse_canonical_json::<Value>(without_lf, &ContractLimits::packet0()).unwrap();
    assert_eq!(
        canonical_json_bytes(&value, &ContractLimits::packet0()).unwrap(),
        without_lf
    );
    let entries =
        parse_canonical_json::<Vec<SchemaManifestEntry>>(without_lf, &ContractLimits::packet0())
            .unwrap();
    assert_eq!(entries.len(), 180);

    let mapping = ROOTS.into_iter().collect::<BTreeMap<_, _>>();
    let refs = entries
        .iter()
        .map(|entry| entry.schema().reference().as_str())
        .collect::<Vec<_>>();
    assert!(refs.windows(2).all(|pair| pair[0] < pair[1]));
    assert_eq!(refs, mapping.keys().copied().collect::<Vec<_>>());

    for entry in entries {
        assert_eq!(entry.draft(), DraftVersion::Draft202012);
        assert_eq!(entry.media_type(), SchemaMediaType::ApplicationSchemaJson);
        let exact_ref = entry.schema().reference().as_str();
        let schema_bytes = read_schema(mapping[exact_ref]);
        assert_eq!(entry.byte_length().get(), schema_bytes.len() as u64);
        assert_eq!(
            entry.schema().fingerprint(),
            &sha256_fingerprint(&schema_bytes)
        );
    }
}

#[test]
fn packet_three_constructor_instances_validate_against_every_new_root_shape() {
    let limits = ContractLimits::packet0();
    let witness =
        ProductionWitness::from_current(UtcTimestamp::parse("2030-01-01T00:00:00Z").unwrap())
            .unwrap();
    assert_schema_valid(
        "handbook.bootstrap-descriptor-schema@1.0.0",
        witness.descriptor(),
    );
    assert_schema_valid("handbook.catalog-roots@1.0.0", witness.catalog_roots());
    for owner in witness.owner_versions() {
        assert_schema_valid("handbook.owner-version-entry@1.0.0", owner);
    }
    for profile in witness.profiles() {
        assert_schema_valid("handbook.profile-catalog-entry@1.0.0", profile);
    }

    let request = BootstrapRequestV1::new(
        witness.descriptor().schemas().bootstrap_request().clone(),
        None,
        witness.descriptor().binding(),
        SemVer::parse("1.0.0").unwrap(),
        CatalogPageSize::new(1).unwrap(),
        &limits,
    )
    .unwrap();
    assert_schema_valid("handbook.bootstrap-request@1.0.0", &request);
    assert_schema_valid(
        "handbook.bootstrap-refusal-details@1.0.0",
        &BootstrapRefusalDetails::new(
            BootstrapRefusalReason::UnsupportedApiMajor,
            BoundedVec::new(vec![SemVer::parse("1.0.0").unwrap()]).unwrap(),
        )
        .unwrap(),
    );
    match witness
        .bootstrap(
            &request,
            &UtcTimestamp::parse("2029-01-01T00:00:00Z").unwrap(),
            &limits,
        )
        .unwrap()
    {
        BootstrapOutcome::Ok(response) => {
            assert_schema_valid("handbook.bootstrap-response@1.0.0", &response);
            assert_schema_valid(
                "handbook.capability-catalog-page@1.0.0",
                response.first_capability_page(),
            );
        }
        BootstrapOutcome::Refused(_) => panic!("current descriptor was refused"),
    }

    let refused = BootstrapRequestV1::new(
        witness.descriptor().schemas().bootstrap_request().clone(),
        None,
        witness.descriptor().binding(),
        SemVer::parse("2.0.0").unwrap(),
        CatalogPageSize::new(1).unwrap(),
        &limits,
    )
    .unwrap();
    match witness
        .bootstrap(
            &refused,
            &UtcTimestamp::parse("2029-01-01T00:00:00Z").unwrap(),
            &limits,
        )
        .unwrap()
    {
        BootstrapOutcome::Refused(refusal) => {
            assert_schema_valid("handbook.bootstrap-refusal@1.0.0", &refusal)
        }
        BootstrapOutcome::Ok(_) => panic!("unsupported major was accepted"),
    }

    match witness
        .read_schemas(
            CatalogPageSize::new(1).unwrap(),
            None,
            &UtcTimestamp::parse("2029-01-01T00:00:00Z").unwrap(),
            &limits,
        )
        .unwrap()
    {
        ProductionCatalogRead::Page(page) => {
            assert_schema_valid("handbook.schema-catalog-page@1.0.0", &page);
            assert_schema_valid(
                "handbook.typed-catalog-cursor@1.0.0",
                page.next_cursor().unwrap(),
            );
        }
        _ => panic!("schema catalog first page was unavailable"),
    }
    match witness
        .read_profiles(
            CatalogPageSize::new(2).unwrap(),
            None,
            &UtcTimestamp::parse("2029-01-01T00:00:00Z").unwrap(),
            &limits,
        )
        .unwrap()
    {
        ProductionCatalogRead::Page(page) => {
            assert_schema_valid("handbook.profile-catalog-page@1.0.0", &page)
        }
        _ => panic!("profile catalog first page was unavailable"),
    }
    match witness
        .read_schemas(
            CatalogPageSize::new(1).unwrap(),
            None,
            &UtcTimestamp::parse("2031-01-01T00:00:00Z").unwrap(),
            &limits,
        )
        .unwrap()
    {
        ProductionCatalogRead::Blocked(blocked) => {
            assert_schema_valid("handbook.catalog-read-blocked@1.0.0", &blocked)
        }
        _ => panic!("expired catalog did not return a blocked restart"),
    }
    assert_eq!(witness.capabilities().len(), 35);
}

#[test]
fn constructor_instances_validate_and_round_trip_through_packet_one_roots() {
    let limits = ContractLimits::packet0();
    let source = SourceBinding::new(
        binding("source.item"),
        binding("source.revision"),
        binding("source.adapter"),
    )
    .unwrap();
    let recheck =
        RecheckCondition::new(binding("condition.schema"), ContractValue::null(), &limits).unwrap();
    let correlation = CorrelationRef::new(bounded("corr_abcdefghijklmnop")).unwrap();
    let artifact = artifact("artifact.sample", "reports/sample.json");
    let problem = Problem::new(
        bounded("problem.prerequisite"),
        bounded("prerequisite.missing"),
        ProblemCategory::Prerequisite,
        binding("subject.item"),
        None,
        binding("details.schema"),
        ContractValue::null(),
        BoundedVec::new(Vec::new()).unwrap(),
        RetryDirective::AfterRecheck,
        Some(recheck.clone()),
        None,
        &limits,
    )
    .unwrap();
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
    let action = NextAction::new(
        bounded("action.retry"),
        NextActionKind::RetryTransient,
        binding("subject.item"),
        binding("parameter.schema"),
        ContractValue::null(),
        Some(bounded("Retry now")),
        &limits,
    )
    .unwrap();
    let omission = Omission::new(
        binding("item.omitted"),
        OmissionReason::Unavailable,
        ProofEffect::Partial,
        Some(source.clone()),
        SafeU64::new(0).unwrap(),
    )
    .unwrap();
    let manifest_entry = SchemaManifestEntry::new(
        binding("schema.sample"),
        DraftVersion::Draft202012,
        SafeU64::new(1).unwrap(),
        SchemaMediaType::ApplicationSchemaJson,
    )
    .unwrap();
    let root = CatalogRoot::new(
        binding("catalog.sample"),
        binding("entry.schema"),
        SafeU64::new(0).unwrap(),
        binding("sort.definition"),
        UtcTimestamp::parse("2030-01-01T00:00:00Z").unwrap(),
    )
    .unwrap();
    let cursor =
        CatalogCursor::new(root.catalog().clone(), ContractValue::null(), &limits).unwrap();
    let page = CatalogPage::new(
        root.clone(),
        BoundedVec::new(Vec::new()).unwrap(),
        None,
        &limits,
    )
    .unwrap();
    let receipt = WriteReceipt::new(
        bounded("record.kind"),
        binding("record.item"),
        bounded("authority.class"),
        ContractValue::null(),
        bounded("atomic.group"),
        &limits,
    )
    .unwrap();
    let problem_binding = ProblemBinding::new(
        problem.problem_id().clone(),
        problem.problem_fingerprint().clone(),
    )
    .unwrap();
    let idempotency = IdempotencyState::established(
        zero_fingerprint(),
        sha256_fingerprint(b"request"),
        false,
        None,
        UtcTimestamp::parse("2030-01-01T00:00:00Z").unwrap(),
    )
    .unwrap();

    assert_schema_and_typed("handbook.problem@1.0.0", &problem);
    assert_schema_and_typed("handbook.diagnostic@1.0.0", &diagnostic);
    assert_schema_and_typed("handbook.next-action@1.0.0", &action);
    assert_schema_and_typed("handbook.artifact-ref@1.0.0", &artifact);
    assert_schema_and_typed("handbook.source-binding@1.0.0", &source);
    assert_schema_and_typed("handbook.omission@1.0.0", &omission);
    assert_schema_and_typed("handbook.schema-manifest-entry@1.0.0", &manifest_entry);
    assert_schema_and_typed("handbook.catalog-root@1.0.0", &root);
    assert_schema_and_typed("handbook.catalog-cursor@1.0.0", &cursor);
    assert_schema_and_typed("handbook.catalog-page@1.0.0", &page);
    assert_schema_and_typed("handbook.recheck-condition@1.0.0", &recheck);
    assert_schema_and_typed("handbook.correlation-ref@1.0.0", &correlation);
    assert_schema_and_typed("handbook.write-receipt@1.0.0", &receipt);
    assert_schema_and_typed("handbook.idempotency-state@1.0.0", &idempotency);
    assert_schema_and_typed("handbook.problem-binding@1.0.0", &problem_binding);

    let content_binding = binding("artifact.content");
    let content_locator = ArtifactLocator::content_addressed(
        binding("store.content"),
        content_binding.fingerprint().clone(),
    );
    let content_artifact = ArtifactRef::new(
        content_binding,
        bounded("application/json"),
        SafeU64::new(2).unwrap(),
        Sensitivity::Restricted,
        content_locator,
    )
    .unwrap();
    assert_schema_and_typed("handbook.artifact-ref@1.0.0", &content_artifact);
    assert_schema_and_typed(
        "handbook.idempotency-state@1.0.0",
        &IdempotencyState::not_applicable().unwrap(),
    );
    assert_schema_and_typed(
        "handbook.idempotency-state@1.0.0",
        &IdempotencyState::not_established(IdempotencyStage::RequestValidation, problem_binding)
            .unwrap(),
    );
}

#[test]
fn packet_two_constructor_instances_validate_and_round_trip_through_new_roots() {
    let limits = ContractLimits::packet0();
    let definition = operation_definition(&limits);
    assert_schema_and_typed("handbook.operation-definition@1.0.0", &definition);

    let request = OperationRequestV1::new(
        binding("handbook.operation-request"),
        Some(RequestId::parse("request-schema-proof").unwrap()),
        &definition,
        ContractValue::null(),
        &FixtureSchemaValidator,
        &limits,
    )
    .unwrap();
    let request_bytes = canonical_json_bytes(&request, &limits).unwrap();
    let request_value: Value = serde_json::from_slice(&request_bytes).unwrap();
    assert!(validator("handbook.operation-request@1.0.0").is_valid(&request_value));
    let parsed_request = OperationRequestV1::parse_and_admit(
        &request_bytes,
        &definition,
        &FixtureSchemaValidator,
        &limits,
    )
    .unwrap();
    assert_eq!(parsed_request.request_id(), request.request_id());
    assert_eq!(
        parsed_request.request_fingerprint(),
        request.request_fingerprint()
    );

    let response = OperationResponseV1::new(
        binding("handbook.operation-response"),
        request.request_id().cloned(),
        request.request_fingerprint().clone(),
        &definition,
        TerminalFields::new(
            TerminalStatus::Ok,
            Some(ContractValue::null()),
            BoundedVec::new(Vec::new()).unwrap(),
            BoundedVec::new(Vec::new()).unwrap(),
            BoundedVec::new(Vec::new()).unwrap(),
        )
        .unwrap(),
        IdempotencyState::not_applicable().unwrap(),
        BoundedVec::new(Vec::new()).unwrap(),
        BoundedVec::new(Vec::new()).unwrap(),
        BoundedVec::new(Vec::new()).unwrap(),
        &FixtureSchemaValidator,
        &limits,
    )
    .unwrap();
    assert_schema_valid("handbook.operation-response@1.0.0", &response);
    let response_bytes = canonical_json_bytes(&response, &limits).unwrap();
    assert_eq!(
        OperationResponseV1::parse_and_admit(
            &response_bytes,
            &definition,
            &FixtureSchemaValidator,
            &limits,
        )
        .unwrap(),
        response
    );

    let refusal_problem = operation_problem(&limits);
    let refusal = OperationAdmissionRefusalV1::new(
        binding("handbook.operation-admission-refusal"),
        request.request_id().cloned(),
        Some(definition.operation().clone()),
        Some(definition.binding()),
        Some(request.request_fingerprint().clone()),
        refusal_problem.clone(),
        &limits,
    )
    .unwrap();
    assert_schema_and_typed("handbook.operation-admission-refusal@1.0.0", &refusal);

    for exact_ref in [
        "handbook.operation-blocker@1.0.0",
        "handbook.operation-refusal@1.0.0",
        "handbook.operation-error@1.0.0",
    ] {
        assert_schema_and_typed(exact_ref, &refusal_problem);
    }
}

#[test]
fn parser_schema_typed_and_owner_witness_gates_remain_distinct() {
    let limits = ContractLimits::packet0();
    let source = SourceBinding::new(
        binding("source.item"),
        binding("source.revision"),
        binding("source.adapter"),
    )
    .unwrap();
    let source_bytes = canonical_json_bytes(&source, &limits).unwrap();
    let source_value: Value = serde_json::from_slice(&source_bytes).unwrap();
    let source_text = String::from_utf8(source_bytes.clone()).unwrap();
    let duplicate = format!(
        "{{\"adapter\":{},{}",
        source_value["adapter"],
        &source_text[1..]
    );
    assert_eq!(
        parse_canonical_json::<Value>(duplicate.as_bytes(), &limits),
        Err(ContractError::InvalidJson)
    );
    let mut framed = source_bytes.clone();
    framed.push(b'\n');
    assert_eq!(
        parse_canonical_json::<Value>(&framed, &limits),
        Err(ContractError::NonCanonicalJson)
    );

    let mut unknown = source_value.clone();
    unknown["unknown"] = Value::Bool(true);
    assert_schema_rejects("handbook.source-binding@1.0.0", &unknown);
    let unknown_bytes = canonical_json_bytes(&unknown, &limits).unwrap();
    assert!(parse_canonical_json::<Value>(&unknown_bytes, &limits).is_ok());
    assert_eq!(
        parse_canonical_json::<SourceBinding>(&unknown_bytes, &limits),
        Err(ContractError::InvalidJson)
    );

    assert_schema_rejects(
        "handbook.idempotency-state@1.0.0",
        &serde_json::json!({"state": "unknown"}),
    );
    assert_schema_rejects(
        "handbook.correlation-ref@1.0.0",
        &Value::String("corr_too_short".to_owned()),
    );

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
    let mut diagnostic_value = to_value(&diagnostic);
    diagnostic_value.as_object_mut().unwrap().remove("code");
    assert_schema_rejects("handbook.diagnostic@1.0.0", &diagnostic_value);
    let mut diagnostic_null = to_value(&diagnostic);
    diagnostic_null["display_message"] = Value::Null;
    assert_schema_rejects("handbook.diagnostic@1.0.0", &diagnostic_null);
    let mut diagnostic_enum = to_value(&diagnostic);
    diagnostic_enum["severity"] = Value::String("critical".to_owned());
    assert_schema_rejects("handbook.diagnostic@1.0.0", &diagnostic_enum);

    let semantic_only = serde_json::json!({
        "idempotency_key_fingerprint": zero_fingerprint(),
        "original_result_fingerprint": sha256_fingerprint(b"result"),
        "replayed": false,
        "request_fingerprint": sha256_fingerprint(b"request"),
        "result_retention_until_utc": "2030-01-01T00:00:00Z",
        "state": "established"
    });
    assert!(validator("handbook.idempotency-state@1.0.0").is_valid(&semantic_only));
    let semantic_bytes = canonical_json_bytes(&semantic_only, &limits).unwrap();
    assert_eq!(
        parse_canonical_json::<IdempotencyState>(&semantic_bytes, &limits),
        Err(ContractError::InvalidInvariant)
    );

    let root = CatalogRoot::new(
        binding("catalog.sample"),
        binding("entry.schema"),
        SafeU64::new(0).unwrap(),
        binding("sort.definition"),
        UtcTimestamp::parse("2030-01-01T00:00:00Z").unwrap(),
    )
    .unwrap();
    let cursor =
        CatalogCursor::new(root.catalog().clone(), ContractValue::null(), &limits).unwrap();
    let mut mismatched_cursor = to_value(&cursor);
    mismatched_cursor["cursor_fingerprint"] = Value::String(ZERO_FINGERPRINT.to_owned());
    assert!(validator("handbook.catalog-cursor@1.0.0").is_valid(&mismatched_cursor));
    let mismatched_bytes = canonical_json_bytes(&mismatched_cursor, &limits).unwrap();
    assert_eq!(
        parse_canonical_json::<CatalogCursor>(&mismatched_bytes, &limits),
        Err(ContractError::FingerprintMismatch)
    );

    let _: IdempotencyVerifier = handbook_sdk::contract_membrane::verify::<IdempotencyState>;
}

#[test]
fn custom_wire_forms_are_exact_and_unfingerprinted_key_material_is_absent() {
    let correlation = schema_value("handbook.correlation-ref@1.0.0");
    assert_eq!(correlation["type"], "string");
    assert!(correlation.get("properties").is_none());

    let problem = schema_value("handbook.problem@1.0.0");
    let exact_binding = &problem["$defs"]["ExactBinding"];
    assert!(exact_binding["properties"].get("ref").is_some());
    assert!(exact_binding["properties"].get("reference").is_none());
    assert!(problem["$defs"]["ContractValue"]["oneOf"].is_array());
    assert!(problem["$defs"].get("ContractValueKind").is_none());
    for required_nullable in ["rule", "recheck", "correlation_ref"] {
        assert!(problem["required"]
            .as_array()
            .unwrap()
            .iter()
            .any(|field| field == required_nullable));
        assert!(problem["properties"][required_nullable]["anyOf"].is_array());
    }

    let diagnostic = schema_value("handbook.diagnostic@1.0.0");
    assert!(!diagnostic["required"]
        .as_array()
        .unwrap()
        .iter()
        .any(|field| field == "display_message"));
    assert_eq!(
        diagnostic["properties"]["display_message"]["$ref"],
        "#/$defs/BoundedString_1024"
    );

    let artifact = schema_value("handbook.artifact-ref@1.0.0");
    let locator = &artifact["$defs"]["ArtifactLocator"]["oneOf"];
    assert_eq!(locator[0]["properties"]["no_follow"]["const"], true);
    assert_eq!(
        locator[1]["properties"]["algorithm"]["$ref"],
        "#/$defs/DigestAlgorithm"
    );
    assert!(artifact["$defs"].get("ArtifactLocatorKind").is_none());

    let idempotency = schema_value("handbook.idempotency-state@1.0.0");
    assert_eq!(idempotency["oneOf"].as_array().unwrap().len(), 3);
    assert!(idempotency["$defs"].get("IdempotencyStateKind").is_none());

    for (exact_ref, relative_path) in ROOTS {
        let schema = serde_json::from_slice::<Value>(&read_schema(relative_path)).unwrap();
        if [
            "handbook.operation.posture-transition-apply-request@1.0.0",
            "handbook.operation.intake-record-append-request@1.0.0",
            "handbook.operation.artifact-candidate-append-request@1.0.0",
            "handbook.operation.artifact-candidate-promote-request@1.0.0",
            "handbook.operation.pipeline-capture-apply-request@1.0.0",
            "handbook.operation.pipeline-handoff-emit-request@1.0.0",
            "handbook.operation.pipeline-state-apply-request@1.0.0",
            "handbook.operation.contract-definition-append-request@1.0.0",
            "handbook.operation.contract-lifecycle-transition-request@1.0.0",
            "handbook.operation.contract-evidence-append-request@1.0.0",
            "handbook.operation.dock-run-request@1.0.0",
            "handbook.operation.snapshot-capture-request@1.0.0",
        ]
        .contains(&exact_ref)
        {
            assert!(schema.to_string().contains("idempotency_key"));
        } else {
            assert_no_unfingerprinted_key_name(&schema);
        }
    }
    let manifest =
        fs::read(Path::new(env!("CARGO_MANIFEST_DIR")).join("schemas/manifest.json")).unwrap();
    assert_no_unfingerprinted_key_name(&serde_json::from_slice::<Value>(&manifest).unwrap());
}

#[test]
fn packet_four_request_result_details_and_receipt_conditions_are_exact() {
    let request = schema_value("handbook.operation.posture-transition-apply-request@1.0.0");
    let expected_request = [
        "change",
        "effective_at_utc",
        "expected_canonical",
        "idempotency_key",
        "reassessment_coverage_ids",
        "recommendation",
        "repository_identity_fingerprint",
    ]
    .into_iter()
    .collect::<BTreeSet<_>>();
    assert_eq!(
        request["properties"]
            .as_object()
            .unwrap()
            .keys()
            .map(String::as_str)
            .collect::<BTreeSet<_>>(),
        expected_request
    );
    assert_eq!(
        request["required"]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.as_str().unwrap())
            .collect::<BTreeSet<_>>(),
        expected_request
    );
    assert_eq!(
        request["properties"]["idempotency_key"]["pattern"],
        "^[!-~]{1,256}$"
    );
    assert_eq!(
        request["properties"]["reassessment_coverage_ids"]["minItems"],
        1
    );
    assert_eq!(
        request["properties"]["reassessment_coverage_ids"]["maxItems"],
        64
    );
    assert_eq!(
        request["$defs"]["PostureCanonicalBindingDtoV1"]["properties"]["reference"]["const"],
        ".handbook/project/charter.yaml"
    );

    let result = schema_value("handbook.operation.posture-transition-apply-result@1.0.0");
    assert_eq!(result["properties"].as_object().unwrap().len(), 7);
    assert_eq!(result["required"].as_array().unwrap().len(), 7);
    assert!(result["properties"].get("idempotency_key").is_none());

    for exact_ref in [
        "handbook.posture-details.authority-conflict@1.0.0",
        "handbook.posture-details.integrity-or-durability@1.0.0",
        "handbook.posture-details.invalid-fixed-dimension@1.0.0",
        "handbook.posture-details.invalid-idempotency-key@1.0.0",
        "handbook.posture-details.invalid-semantic-input@1.0.0",
        "handbook.posture-details.io@1.0.0",
        "handbook.posture-details.lifecycle-not-current@1.0.0",
        "handbook.posture-details.missing-lifecycle-authority@1.0.0",
        "handbook.posture-details.repository-identity-mismatch@1.0.0",
        "handbook.posture-details.stale-canonical@1.0.0",
        "handbook.posture-details.unsafe-filesystem@1.0.0",
        "handbook.posture-details.unsupported-platform@1.0.0",
        "handbook.posture-recheck.unsupported-platform@1.0.0",
    ] {
        let schema = schema_value(exact_ref);
        assert_eq!(schema["properties"].as_object().unwrap().len(), 1);
        assert_eq!(schema["required"], serde_json::json!(["owner_variant"]));
    }

    let canonical = schema_value("handbook.posture-write-condition.canonical-truth@1.0.0");
    assert_eq!(canonical["properties"].as_object().unwrap().len(), 2);
    assert_eq!(
        canonical["required"],
        serde_json::json!(["prior_canonical", "resulting_canonical"])
    );
    let semantic = schema_value("handbook.posture-write-condition.semantic-record@1.0.0");
    assert_eq!(semantic["properties"].as_object().unwrap().len(), 3);
    assert_eq!(
        semantic["required"],
        serde_json::json!([
            "lifecycle_transition",
            "posture_transition",
            "resulting_kernel"
        ])
    );

    let assert_fields = |exact_ref: &str, expected: &[&str]| {
        let schema = schema_value(exact_ref);
        let expected = expected.iter().copied().collect::<BTreeSet<_>>();
        assert_eq!(
            schema["properties"]
                .as_object()
                .unwrap()
                .keys()
                .map(String::as_str)
                .collect::<BTreeSet<_>>(),
            expected,
            "properties: {exact_ref}"
        );
        assert_eq!(
            schema["required"]
                .as_array()
                .unwrap()
                .iter()
                .map(|value| value.as_str().unwrap())
                .collect::<BTreeSet<_>>(),
            expected,
            "required: {exact_ref}"
        );
    };
    for (exact_ref, fields) in [
        (
            "handbook.operation.artifact-kind-list-request@1.0.0",
            &[][..],
        ),
        (
            "handbook.operation.artifact-kind-list-result@1.0.0",
            &["kinds", "operation_id", "selection_fingerprint"][..],
        ),
        (
            "handbook.operation.artifact-instance-list-request@1.0.0",
            &[][..],
        ),
        (
            "handbook.operation.artifact-instance-list-result@1.0.0",
            &["instances", "operation_id", "selection_fingerprint"][..],
        ),
        (
            "handbook.operation.artifact-read-request@1.0.0",
            &["instance_id", "kind_ref"][..],
        ),
        (
            "handbook.operation.artifact-read-result@1.0.0",
            &[
                "artifact_fingerprint",
                "canonical_path",
                "content",
                "instance_id",
                "kind_ref",
                "operation_context_fingerprint",
                "operation_id",
            ][..],
        ),
        (
            "handbook.operation.artifact-validate-request@1.0.0",
            &["instance_id", "kind_ref"][..],
        ),
        (
            "handbook.operation.artifact-validate-result@1.0.0",
            &[
                "artifact_fingerprint",
                "content",
                "layers",
                "operation_context_fingerprint",
                "operation_id",
                "outcome",
            ][..],
        ),
        (
            "handbook.operation.intake-definition-read-request@1.0.0",
            &["instance_id", "kind_ref"][..],
        ),
        (
            "handbook.operation.intake-definition-read-result@1.0.0",
            &[
                "candidate_schema_ref",
                "coverage",
                "instance_id",
                "intake_definition_fingerprint",
                "intake_definition_ref",
                "kind_ref",
                "operation_id",
            ][..],
        ),
        (
            "handbook.operation.intake-coverage-evaluate-request@1.0.0",
            &[
                "acquisition_mode",
                "coverage_submissions",
                "expected_current_artifact_fingerprint",
                "instance_id",
                "kind_ref",
            ][..],
        ),
        (
            "handbook.operation.intake-coverage-evaluate-result@1.0.0",
            &[
                "acquisition_mode",
                "basis_artifact_fingerprint",
                "coverage_results",
                "evaluation_fingerprint",
                "field_sources",
                "normalized_content",
                "operation_context_fingerprint",
                "outcome",
                "schema_id",
                "schema_version",
            ][..],
        ),
        (
            "handbook.operation.artifact-candidate-validate-request@1.0.0",
            &[
                "expected_current_artifact_fingerprint",
                "instance_id",
                "intake_record_fingerprint",
                "intake_record_ref",
                "kind_ref",
            ][..],
        ),
        (
            "handbook.operation.artifact-candidate-validate-result@1.0.0",
            &[
                "basis_artifact_fingerprint",
                "candidate",
                "candidate_fingerprint",
                "candidate_ref",
                "intake_record_fingerprint",
                "intake_record_ref",
                "normalized_content",
                "normalized_content_fingerprint",
                "normalized_content_ref",
                "operation_context_fingerprint",
                "operation_id",
                "outcome",
                "validation_result",
                "validation_result_fingerprint",
                "validation_result_ref",
            ][..],
        ),
        (
            "handbook.operation.intake-record-append-request@1.0.0",
            &[
                "acquisition_mode",
                "coverage_submissions",
                "expected_current_artifact_fingerprint",
                "idempotency_key",
                "instance_id",
                "kind_ref",
            ][..],
        ),
        (
            "handbook.operation.artifact-candidate-append-request@1.0.0",
            &[
                "expected_candidate_fingerprint",
                "idempotency_key",
                "instance_id",
                "intake_record_fingerprint",
                "intake_record_ref",
                "kind_ref",
            ][..],
        ),
        (
            "handbook.operation.artifact-candidate-promote-request@1.0.0",
            &[
                "candidate_fingerprint",
                "candidate_ref",
                "expected_current_artifact_fingerprint",
                "idempotency_key",
                "instance_id",
                "kind_ref",
                "publisher_authority",
            ][..],
        ),
    ] {
        assert_fields(exact_ref, fields);
    }

    let mutation_fields = [
        "authoritative_outputs",
        "internal_transaction_evidence_fingerprint",
        "internal_transaction_evidence_ref",
        "operation_id",
        "outcome",
        "refusal",
        "request_fingerprint",
        "result_fingerprint",
        "schema_id",
        "schema_version",
        "transaction_id",
    ];
    for (exact_ref, operation_id, expected_authority_classes) in [
        (
            "handbook.operation.intake-record-append-result@1.0.0",
            "intake.record.append",
            &["semantic_record"][..],
        ),
        (
            "handbook.operation.artifact-candidate-append-result@1.0.0",
            "artifact.candidate.append",
            &["semantic_record"][..],
        ),
        (
            "handbook.operation.artifact-candidate-promote-result@1.0.0",
            "artifact.candidate.promote",
            &["canonical_truth", "semantic_record"][..],
        ),
    ] {
        assert_fields(exact_ref, &mutation_fields);
        let schema = schema_value(exact_ref);
        assert_eq!(schema["properties"]["operation_id"]["const"], operation_id);
        assert_eq!(
            schema["oneOf"][0]["properties"]["authoritative_outputs"]["minItems"],
            expected_authority_classes.len()
        );
        assert_eq!(
            schema["oneOf"][0]["properties"]["authoritative_outputs"]["maxItems"],
            expected_authority_classes.len()
        );
        assert_eq!(
            schema["oneOf"][0]["properties"]["authoritative_outputs"]["prefixItems"]
                .as_array()
                .unwrap()
                .iter()
                .map(
                    |item| item["allOf"][1]["properties"]["authority_class"]["const"]
                        .as_str()
                        .unwrap()
                )
                .collect::<Vec<_>>(),
            expected_authority_classes
        );
        assert_eq!(
            schema["oneOf"][1]["properties"]["authoritative_outputs"]["maxItems"],
            0
        );
        assert_eq!(
            schema["$defs"]["ArtifactMutationRefusalV1"]["oneOf"]
                .as_array()
                .unwrap()
                .len(),
            7
        );
    }

    assert_fields(
        "handbook.artifact-operation-details@1.0.0",
        &["cause", "detail", "mutation_result", "operation_id"],
    );
    for (exact_ref, authority_class) in [
        (
            "handbook.artifact-write-condition.canonical-truth@1.0.0",
            "canonical_truth",
        ),
        (
            "handbook.artifact-write-condition.semantic-record@1.0.0",
            "semantic_record",
        ),
    ] {
        assert_fields(
            exact_ref,
            &[
                "authority_class",
                "fingerprint",
                "operation_id",
                "ref",
                "transaction_id",
            ],
        );
        assert_eq!(
            schema_value(exact_ref)["properties"]["authority_class"]["const"],
            authority_class
        );
    }
    assert_eq!(
        schema_value("handbook.artifact-recheck.supported-platform@1.0.0")["properties"]["cause"]
            ["enum"],
        serde_json::json!(["unsupported_platform"])
    );
    assert_eq!(
        schema_value("handbook.artifact-recheck.repository-authority@1.0.0")["properties"]["cause"]
            ["enum"],
        serde_json::json!([
            "selection_unavailable",
            "profile_unavailable",
            "registry_unavailable",
            "intake_source_unavailable",
            "repository_identity_unavailable",
            "artifact_unavailable"
        ])
    );
}

fn schema_files(root: &Path) -> BTreeSet<PathBuf> {
    fn visit(root: &Path, current: &Path, files: &mut BTreeSet<PathBuf>) {
        for entry in fs::read_dir(current).expect("generated schema directory must exist") {
            let entry = entry.expect("generated schema entry must be readable");
            let path = entry.path();
            if path.is_dir() {
                visit(root, &path, files);
            } else {
                files.insert(
                    path.strip_prefix(root)
                        .expect("schema path must remain below its root")
                        .to_path_buf(),
                );
            }
        }
    }

    let mut files = BTreeSet::new();
    visit(root, root, &mut files);
    files
}

fn read_schema(relative_path: &str) -> Vec<u8> {
    fs::read(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("schemas/2020-12")
            .join(relative_path),
    )
    .unwrap()
}

fn assert_one_terminal_lf(bytes: &[u8], subject: &str) {
    assert!(bytes.ends_with(b"\n"), "{subject} lacks its terminal LF");
    assert!(bytes.len() > 1 && bytes[bytes.len() - 2] != b'\n');
    assert!(!bytes[..bytes.len() - 1].contains(&b'\r'));
}

fn assert_local_refs_resolve(root: &Value, current: &Value) {
    match current {
        Value::Array(values) => {
            for value in values {
                assert_local_refs_resolve(root, value);
            }
        }
        Value::Object(members) => {
            if let Some(reference) = members.get("$ref") {
                let reference = reference.as_str().expect("$ref must be a string");
                let pointer = reference
                    .strip_prefix('#')
                    .filter(|pointer| pointer.starts_with("/$defs/"))
                    .expect("all schema refs must be local $defs fragments");
                assert!(
                    root.pointer(pointer).is_some(),
                    "unresolved ref: {reference}"
                );
            }
            for value in members.values() {
                assert_local_refs_resolve(root, value);
            }
        }
        _ => {}
    }
}

fn assert_structural_objects_are_closed(value: &Value) {
    match value {
        Value::Array(values) => {
            for value in values {
                assert_structural_objects_are_closed(value);
            }
        }
        Value::Object(members) => {
            if members.get("type") == Some(&Value::String("object".to_owned()))
                && members.contains_key("properties")
            {
                assert_eq!(
                    members.get("additionalProperties"),
                    Some(&Value::Bool(false))
                );
            }
            for value in members.values() {
                assert_structural_objects_are_closed(value);
            }
        }
        _ => {}
    }
}

fn validator(exact_ref: &str) -> Validator {
    let relative_path = ROOTS
        .iter()
        .find_map(|(candidate, path)| (*candidate == exact_ref).then_some(*path))
        .unwrap();
    let bytes = read_schema(relative_path);
    let schema: Value = serde_json::from_slice(&bytes).unwrap();
    jsonschema::draft202012::options()
        .with_pattern_options(PatternOptions::regex())
        .with_retriever(RejectExternal)
        .build(&schema)
        .unwrap()
}

fn schema_value(exact_ref: &str) -> Value {
    let relative_path = ROOTS
        .iter()
        .find_map(|(candidate, path)| (*candidate == exact_ref).then_some(*path))
        .unwrap();
    serde_json::from_slice(&read_schema(relative_path)).unwrap()
}

fn assert_schema_and_typed<T>(exact_ref: &str, value: &T)
where
    T: Serialize + DeserializeOwned + Eq + fmt::Debug,
{
    let limits = ContractLimits::packet0();
    let bytes = canonical_json_bytes(value, &limits).unwrap();
    let instance: Value = serde_json::from_slice(&bytes).unwrap();
    let validator = validator(exact_ref);
    assert!(
        validator.is_valid(&instance),
        "constructor instance failed {exact_ref}"
    );
    let parsed = parse_canonical_json::<T>(&bytes, &limits)
        .unwrap_or_else(|error| panic!("typed round trip failed for {exact_ref}: {error:?}"));
    assert_eq!(parsed, *value);
}

fn assert_schema_valid(exact_ref: &str, value: &impl Serialize) {
    let bytes = canonical_json_bytes(value, &ContractLimits::packet0()).unwrap();
    let instance: Value = serde_json::from_slice(&bytes).unwrap();
    assert!(
        validator(exact_ref).is_valid(&instance),
        "constructor instance failed {exact_ref}"
    );
}

fn assert_schema_rejects(exact_ref: &str, instance: &Value) {
    assert!(
        !validator(exact_ref).is_valid(instance),
        "schema unexpectedly accepted boundary fixture for {exact_ref}"
    );
}

fn to_value(value: &impl Serialize) -> Value {
    let bytes = canonical_json_bytes(value, &ContractLimits::packet0()).unwrap();
    serde_json::from_slice(&bytes).unwrap()
}

fn bounded<const N: usize>(value: &str) -> BoundedString<N> {
    BoundedString::new(value.to_owned()).unwrap()
}

fn binding(identity: &str) -> ExactBinding {
    ExactBinding::new(
        ExactRef::parse(&format!("{identity}@1.0.0")).unwrap(),
        sha256_fingerprint(identity.as_bytes()),
    )
}

fn zero_fingerprint() -> Fingerprint {
    Fingerprint::parse(ZERO_FINGERPRINT).unwrap()
}

fn artifact(identity: &str, path: &str) -> ArtifactRef {
    ArtifactRef::new(
        binding(identity),
        bounded("application/json"),
        SafeU64::new(1).unwrap(),
        Sensitivity::Internal,
        ArtifactLocator::repo_relative(sha256_fingerprint(b"repository"), bounded(path)).unwrap(),
    )
    .unwrap()
}

fn operation_definition(limits: &ContractLimits) -> OperationDefinition {
    OperationDefinition::new(
        binding("handbook.operation-definition"),
        ExactRef::parse("artifact.validate@1.0.0").unwrap(),
        OperationOwnerBinding::new(
            bounded("handbook-engine"),
            SemVer::parse("0.2.0").unwrap(),
            binding("source.owner"),
        )
        .unwrap(),
        binding("handbook.operation.artifact-validate-request"),
        binding("handbook.operation.artifact-validate-result"),
        OperationOutcomeSchemas::new(
            binding("handbook.operation-blocker"),
            binding("handbook.operation-refusal"),
            binding("handbook.operation-error"),
        )
        .unwrap(),
        BoundedVec::new(Vec::new()).unwrap(),
        OperationClass::ReadOnly,
        OperationIdempotency::safe(),
        BoundedVec::new(Vec::new()).unwrap(),
        BoundedVec::new(vec![
            OperationTransport::CliJson,
            OperationTransport::RustSdk,
            OperationTransport::Tauri,
        ])
        .unwrap(),
        None,
        limits,
    )
    .unwrap()
}

fn operation_problem(limits: &ContractLimits) -> Problem {
    Problem::new(
        bounded("problem.operation-refusal"),
        bounded("operation.not-admitted"),
        ProblemCategory::Capability,
        binding("operation.subject"),
        Some(binding("rule.owner-admission")),
        binding("operation.problem-details"),
        ContractValue::null(),
        BoundedVec::new(Vec::new()).unwrap(),
        RetryDirective::Never,
        None,
        None,
        limits,
    )
    .unwrap()
}

struct FixtureSchemaValidator;

impl OperationSchemaValidator for FixtureSchemaValidator {
    fn validate_instance(
        &self,
        _schema: &ExactBinding,
        _instance: &ContractValue,
    ) -> Result<(), ContractError> {
        Ok(())
    }
}

fn assert_no_unfingerprinted_key_name(value: &Value) {
    match value {
        Value::Array(values) => {
            for value in values {
                assert_no_unfingerprinted_key_name(value);
            }
        }
        Value::Object(members) => {
            for (key, value) in members {
                assert_ne!(key, "idempotency_key");
                assert_ne!(key, "raw_idempotency_key");
                assert_no_unfingerprinted_key_name(value);
            }
        }
        Value::String(value) => {
            assert_ne!(value, "idempotency_key");
            assert_ne!(value, "raw_idempotency_key");
        }
        _ => {}
    }
}
