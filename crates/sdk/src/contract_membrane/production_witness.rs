use super::{
    canonical_json_bytes, operation_inventory, parse_canonical_json, sha256_fingerprint,
    BootstrapDescriptorV1, BootstrapRefusalV1, BootstrapRequestV1, BootstrapResponseV1,
    BootstrapSchemaBindings, BoundedString, BoundedVec, CapabilityCatalogPage, CapabilityEntry,
    CatalogPageSize, CatalogReadBlockedV1, CatalogReadError, CatalogRoots, ContractError,
    ContractLimits, DiscoveryState, ExactBinding, ExactRef, Fingerprint, ImmutableCatalog,
    OperationTransport, OwnerVersionEntry, ProfileCatalogEntry, ProfileCatalogPage,
    SchemaCatalogPage, SchemaManifestEntry, SemVer, TypedCatalogCursor, UtcTimestamp,
};
use std::collections::BTreeSet;

const INVENTORY_DIGEST: &str =
    "sha256:0beed3959b30cb80a8b260258556b21d012c0029babbc110dbe05025e18d3126";

macro_rules! schema_asset {
    ($reference:literal, $path:literal) => {
        ($reference, include_bytes!($path).as_slice())
    };
}

const SCHEMA_MANIFEST_BYTES: &[u8] = include_bytes!("../../schemas/manifest.json");
const SCHEMA_ASSETS: [(&str, &[u8]); 135] = [
    schema_asset!(
        "handbook.artifact-operation-details@1.0.0",
        "../../schemas/2020-12/handbook.artifact-operation-details/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.artifact-recheck.repository-authority@1.0.0",
        "../../schemas/2020-12/handbook.artifact-recheck.repository-authority/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.artifact-recheck.supported-platform@1.0.0",
        "../../schemas/2020-12/handbook.artifact-recheck.supported-platform/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.artifact-ref@1.0.0",
        "../../schemas/2020-12/handbook.artifact-ref/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.artifact-write-condition.canonical-truth@1.0.0",
        "../../schemas/2020-12/handbook.artifact-write-condition.canonical-truth/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.artifact-write-condition.semantic-record@1.0.0",
        "../../schemas/2020-12/handbook.artifact-write-condition.semantic-record/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.bootstrap-descriptor-schema@1.0.0",
        "../../schemas/2020-12/handbook.bootstrap-descriptor-schema/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.bootstrap-facade-response@1.0.0",
        "../../schemas/2020-12/handbook.bootstrap-facade-response/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.bootstrap-refusal-details@1.0.0",
        "../../schemas/2020-12/handbook.bootstrap-refusal-details/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.bootstrap-refusal@1.0.0",
        "../../schemas/2020-12/handbook.bootstrap-refusal/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.bootstrap-request@1.0.0",
        "../../schemas/2020-12/handbook.bootstrap-request/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.bootstrap-response@1.0.0",
        "../../schemas/2020-12/handbook.bootstrap-response/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.capability-catalog-page@1.0.0",
        "../../schemas/2020-12/handbook.capability-catalog-page/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.capability-entry@1.0.0",
        "../../schemas/2020-12/handbook.capability-entry/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.catalog-cursor@1.0.0",
        "../../schemas/2020-12/handbook.catalog-cursor/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.catalog-page@1.0.0",
        "../../schemas/2020-12/handbook.catalog-page/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.catalog-read-blocked@1.0.0",
        "../../schemas/2020-12/handbook.catalog-read-blocked/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.catalog-read-request@1.0.0",
        "../../schemas/2020-12/handbook.catalog-read-request/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.catalog-read-response@1.0.0",
        "../../schemas/2020-12/handbook.catalog-read-response/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.catalog-root@1.0.0",
        "../../schemas/2020-12/handbook.catalog-root/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.catalog-roots@1.0.0",
        "../../schemas/2020-12/handbook.catalog-roots/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.contract-applicability-outcome@1.0.0",
        "../../schemas/2020-12/handbook.contract-applicability-outcome/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.contract-claim@1.0.0",
        "../../schemas/2020-12/handbook.contract-claim/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.contract-definition@1.0.0",
        "../../schemas/2020-12/handbook.contract-definition/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.contract-evaluation-lifecycle-basis@1.0.0",
        "../../schemas/2020-12/handbook.contract-evaluation-lifecycle-basis/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.contract-lifecycle-policy@1.0.0",
        "../../schemas/2020-12/handbook.contract-lifecycle-policy/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.contract-lifecycle-transition@1.0.0",
        "../../schemas/2020-12/handbook.contract-lifecycle-transition/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.contract-operation-details@1.0.0",
        "../../schemas/2020-12/handbook.contract-operation-details/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.correlation-ref@1.0.0",
        "../../schemas/2020-12/handbook.correlation-ref/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.diagnostic@1.0.0",
        "../../schemas/2020-12/handbook.diagnostic/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.flow-operation-details@1.0.0",
        "../../schemas/2020-12/handbook.flow-operation-details/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.flow-recheck@1.0.0",
        "../../schemas/2020-12/handbook.flow-recheck/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.idempotency-state@1.0.0",
        "../../schemas/2020-12/handbook.idempotency-state/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.invocation-request@1.0.0",
        "../../schemas/2020-12/handbook.invocation-request/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.invocation-response@1.0.0",
        "../../schemas/2020-12/handbook.invocation-response/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.next-action@1.0.0",
        "../../schemas/2020-12/handbook.next-action/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.omission@1.0.0",
        "../../schemas/2020-12/handbook.omission/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.operation-admission-refusal@1.0.0",
        "../../schemas/2020-12/handbook.operation-admission-refusal/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.operation-blocker@1.0.0",
        "../../schemas/2020-12/handbook.operation-blocker/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.operation-definition@1.0.0",
        "../../schemas/2020-12/handbook.operation-definition/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.operation-error@1.0.0",
        "../../schemas/2020-12/handbook.operation-error/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.operation-refusal@1.0.0",
        "../../schemas/2020-12/handbook.operation-refusal/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.operation-request@1.0.0",
        "../../schemas/2020-12/handbook.operation-request/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.operation-response@1.0.0",
        "../../schemas/2020-12/handbook.operation-response/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.operation.artifact-candidate-append-request@1.0.0",
        "../../schemas/2020-12/handbook.operation.artifact-candidate-append-request/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.operation.artifact-candidate-append-result@1.0.0",
        "../../schemas/2020-12/handbook.operation.artifact-candidate-append-result/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.operation.artifact-candidate-promote-request@1.0.0",
        "../../schemas/2020-12/handbook.operation.artifact-candidate-promote-request/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.operation.artifact-candidate-promote-result@1.0.0",
        "../../schemas/2020-12/handbook.operation.artifact-candidate-promote-result/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.operation.artifact-candidate-validate-request@1.0.0",
        "../../schemas/2020-12/handbook.operation.artifact-candidate-validate-request/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.operation.artifact-candidate-validate-result@1.0.0",
        "../../schemas/2020-12/handbook.operation.artifact-candidate-validate-result/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.operation.artifact-instance-list-request@1.0.0",
        "../../schemas/2020-12/handbook.operation.artifact-instance-list-request/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.operation.artifact-instance-list-result@1.0.0",
        "../../schemas/2020-12/handbook.operation.artifact-instance-list-result/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.operation.artifact-kind-list-request@1.0.0",
        "../../schemas/2020-12/handbook.operation.artifact-kind-list-request/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.operation.artifact-kind-list-result@1.0.0",
        "../../schemas/2020-12/handbook.operation.artifact-kind-list-result/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.operation.artifact-read-request@1.0.0",
        "../../schemas/2020-12/handbook.operation.artifact-read-request/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.operation.artifact-read-result@1.0.0",
        "../../schemas/2020-12/handbook.operation.artifact-read-result/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.operation.artifact-validate-request@1.0.0",
        "../../schemas/2020-12/handbook.operation.artifact-validate-request/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.operation.artifact-validate-result@1.0.0",
        "../../schemas/2020-12/handbook.operation.artifact-validate-result/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.operation.contract-definition-append-request@1.0.0",
        "../../schemas/2020-12/handbook.operation.contract-definition-append-request/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.operation.contract-definition-append-result@1.0.0",
        "../../schemas/2020-12/handbook.operation.contract-definition-append-result/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.operation.contract-definition-list-request@1.0.0",
        "../../schemas/2020-12/handbook.operation.contract-definition-list-request/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.operation.contract-definition-list-result@1.0.0",
        "../../schemas/2020-12/handbook.operation.contract-definition-list-result/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.operation.contract-definition-read-request@1.0.0",
        "../../schemas/2020-12/handbook.operation.contract-definition-read-request/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.operation.contract-definition-read-result@1.0.0",
        "../../schemas/2020-12/handbook.operation.contract-definition-read-result/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.operation.contract-lifecycle-transition-request@1.0.0",
        "../../schemas/2020-12/handbook.operation.contract-lifecycle-transition-request/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.operation.contract-lifecycle-transition-result@1.0.0",
        "../../schemas/2020-12/handbook.operation.contract-lifecycle-transition-result/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.operation.flow-resolve-request@1.0.0",
        "../../schemas/2020-12/handbook.operation.flow-resolve-request/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.operation.flow-resolve-result@1.0.0",
        "../../schemas/2020-12/handbook.operation.flow-resolve-result/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.operation.intake-coverage-evaluate-request@1.0.0",
        "../../schemas/2020-12/handbook.operation.intake-coverage-evaluate-request/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.operation.intake-coverage-evaluate-result@1.0.0",
        "../../schemas/2020-12/handbook.operation.intake-coverage-evaluate-result/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.operation.intake-definition-read-request@1.0.0",
        "../../schemas/2020-12/handbook.operation.intake-definition-read-request/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.operation.intake-definition-read-result@1.0.0",
        "../../schemas/2020-12/handbook.operation.intake-definition-read-result/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.operation.intake-record-append-request@1.0.0",
        "../../schemas/2020-12/handbook.operation.intake-record-append-request/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.operation.intake-record-append-result@1.0.0",
        "../../schemas/2020-12/handbook.operation.intake-record-append-result/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.operation.pipeline-capture-apply-request@1.0.0",
        "../../schemas/2020-12/handbook.operation.pipeline-capture-apply-request/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.operation.pipeline-capture-apply-result@1.0.0",
        "../../schemas/2020-12/handbook.operation.pipeline-capture-apply-result/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.operation.pipeline-capture-plan-request@1.0.0",
        "../../schemas/2020-12/handbook.operation.pipeline-capture-plan-request/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.operation.pipeline-capture-plan-result@1.0.0",
        "../../schemas/2020-12/handbook.operation.pipeline-capture-plan-result/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.operation.pipeline-catalog-list-request@1.0.0",
        "../../schemas/2020-12/handbook.operation.pipeline-catalog-list-request/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.operation.pipeline-catalog-list-result@1.0.0",
        "../../schemas/2020-12/handbook.operation.pipeline-catalog-list-result/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.operation.pipeline-catalog-read-request@1.0.0",
        "../../schemas/2020-12/handbook.operation.pipeline-catalog-read-request/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.operation.pipeline-catalog-read-result@1.0.0",
        "../../schemas/2020-12/handbook.operation.pipeline-catalog-read-result/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.operation.pipeline-compile-request@1.0.0",
        "../../schemas/2020-12/handbook.operation.pipeline-compile-request/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.operation.pipeline-compile-result@1.0.0",
        "../../schemas/2020-12/handbook.operation.pipeline-compile-result/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.operation.pipeline-handoff-emit-request@1.0.0",
        "../../schemas/2020-12/handbook.operation.pipeline-handoff-emit-request/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.operation.pipeline-handoff-emit-result@1.0.0",
        "../../schemas/2020-12/handbook.operation.pipeline-handoff-emit-result/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.operation.pipeline-state-apply-request@1.0.0",
        "../../schemas/2020-12/handbook.operation.pipeline-state-apply-request/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.operation.pipeline-state-apply-result@1.0.0",
        "../../schemas/2020-12/handbook.operation.pipeline-state-apply-result/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.operation.posture-transition-apply-request@1.0.0",
        "../../schemas/2020-12/handbook.operation.posture-transition-apply-request/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.operation.posture-transition-apply-result@1.0.0",
        "../../schemas/2020-12/handbook.operation.posture-transition-apply-result/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.owner-version-entry@1.0.0",
        "../../schemas/2020-12/handbook.owner-version-entry/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.pipeline-operation-details@1.0.0",
        "../../schemas/2020-12/handbook.pipeline-operation-details/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.pipeline-recheck@1.0.0",
        "../../schemas/2020-12/handbook.pipeline-recheck/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.pipeline-write-condition@1.0.0",
        "../../schemas/2020-12/handbook.pipeline-write-condition/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.posture-details.authority-conflict@1.0.0",
        "../../schemas/2020-12/handbook.posture-details.authority-conflict/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.posture-details.integrity-or-durability@1.0.0",
        "../../schemas/2020-12/handbook.posture-details.integrity-or-durability/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.posture-details.invalid-fixed-dimension@1.0.0",
        "../../schemas/2020-12/handbook.posture-details.invalid-fixed-dimension/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.posture-details.invalid-idempotency-key@1.0.0",
        "../../schemas/2020-12/handbook.posture-details.invalid-idempotency-key/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.posture-details.invalid-semantic-input@1.0.0",
        "../../schemas/2020-12/handbook.posture-details.invalid-semantic-input/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.posture-details.io@1.0.0",
        "../../schemas/2020-12/handbook.posture-details.io/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.posture-details.lifecycle-not-current@1.0.0",
        "../../schemas/2020-12/handbook.posture-details.lifecycle-not-current/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.posture-details.missing-lifecycle-authority@1.0.0",
        "../../schemas/2020-12/handbook.posture-details.missing-lifecycle-authority/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.posture-details.repository-identity-mismatch@1.0.0",
        "../../schemas/2020-12/handbook.posture-details.repository-identity-mismatch/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.posture-details.stale-canonical@1.0.0",
        "../../schemas/2020-12/handbook.posture-details.stale-canonical/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.posture-details.unsafe-filesystem@1.0.0",
        "../../schemas/2020-12/handbook.posture-details.unsafe-filesystem/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.posture-details.unsupported-platform@1.0.0",
        "../../schemas/2020-12/handbook.posture-details.unsupported-platform/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.posture-recheck.unsupported-platform@1.0.0",
        "../../schemas/2020-12/handbook.posture-recheck.unsupported-platform/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.posture-write-condition.canonical-truth@1.0.0",
        "../../schemas/2020-12/handbook.posture-write-condition.canonical-truth/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.posture-write-condition.semantic-record@1.0.0",
        "../../schemas/2020-12/handbook.posture-write-condition.semantic-record/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.problem-binding@1.0.0",
        "../../schemas/2020-12/handbook.problem-binding/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.problem@1.0.0",
        "../../schemas/2020-12/handbook.problem/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.profile-catalog-entry@1.0.0",
        "../../schemas/2020-12/handbook.profile-catalog-entry/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.profile-catalog-page@1.0.0",
        "../../schemas/2020-12/handbook.profile-catalog-page/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.recheck-condition@1.0.0",
        "../../schemas/2020-12/handbook.recheck-condition/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.schema-catalog-page@1.0.0",
        "../../schemas/2020-12/handbook.schema-catalog-page/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.schema-manifest-entry@1.0.0",
        "../../schemas/2020-12/handbook.schema-manifest-entry/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.source-binding@1.0.0",
        "../../schemas/2020-12/handbook.source-binding/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.transport-failure-details@1.0.0",
        "../../schemas/2020-12/handbook.transport-failure-details/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.transport-failure@1.0.0",
        "../../schemas/2020-12/handbook.transport-failure/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.typed-catalog-cursor@1.0.0",
        "../../schemas/2020-12/handbook.typed-catalog-cursor/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.use-case-request@1.0.0",
        "../../schemas/2020-12/handbook.use-case-request/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.use-case-response@1.0.0",
        "../../schemas/2020-12/handbook.use-case-response/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.use-case.approver-administration-details@1.0.0",
        "../../schemas/2020-12/handbook.use-case.approver-administration-details/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.use-case.approver-administration-result@1.0.0",
        "../../schemas/2020-12/handbook.use-case.approver-administration-result/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.use-case.author-charter-details@1.0.0",
        "../../schemas/2020-12/handbook.use-case.author-charter-details/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.use-case.author-charter-result@1.0.0",
        "../../schemas/2020-12/handbook.use-case.author-charter-result/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.use-case.author-project-context-details@1.0.0",
        "../../schemas/2020-12/handbook.use-case.author-project-context-details/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.use-case.author-project-context-result@1.0.0",
        "../../schemas/2020-12/handbook.use-case.author-project-context-result/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.use-case.pipeline-route-resolve-and-persist-details@1.0.0",
        "../../schemas/2020-12/handbook.use-case.pipeline-route-resolve-and-persist-details/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.use-case.pipeline-route-resolve-and-persist-result@1.0.0",
        "../../schemas/2020-12/handbook.use-case.pipeline-route-resolve-and-persist-result/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.use-case.repository-doctor-details@1.0.0",
        "../../schemas/2020-12/handbook.use-case.repository-doctor-details/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.use-case.repository-doctor-result@1.0.0",
        "../../schemas/2020-12/handbook.use-case.repository-doctor-result/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.use-case.repository-setup-details@1.0.0",
        "../../schemas/2020-12/handbook.use-case.repository-setup-details/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.use-case.repository-setup-result@1.0.0",
        "../../schemas/2020-12/handbook.use-case.repository-setup-result/1.0.0.schema.json"
    ),
    schema_asset!(
        "handbook.write-receipt@1.0.0",
        "../../schemas/2020-12/handbook.write-receipt/1.0.0.schema.json"
    ),
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProductionWitnessError {
    ColdManifest,
    StaleManifest,
    TamperedSchema,
    DuplicateSchema,
    ManifestGap,
    ManifestCountMismatch,
    InventoryMismatch,
    OwnerEvidenceMismatch,
    MissingProfile,
    DuplicateProfile,
    UnknownOperation,
    MissingOperationSchema,
    UnprovenTransport,
    Contract(ContractError),
}

impl From<ContractError> for ProductionWitnessError {
    fn from(value: ContractError) -> Self {
        Self::Contract(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BootstrapOutcome {
    Ok(Box<BootstrapResponseV1>),
    Refused(Box<BootstrapRefusalV1>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProductionCatalogRead<T> {
    Page(T),
    CursorMismatch,
    Blocked(Box<CatalogReadBlockedV1>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProductionWitness {
    descriptor: BootstrapDescriptorV1,
    owner_versions: Vec<OwnerVersionEntry>,
    schema_manifest: Vec<SchemaManifestEntry>,
    capabilities: ImmutableCatalog<CapabilityEntry>,
    schemas: ImmutableCatalog<SchemaManifestEntry>,
    profiles: ImmutableCatalog<ProfileCatalogEntry>,
    roots: CatalogRoots,
}

impl ProductionWitness {
    pub fn from_current(retention_until_utc: UtcTimestamp) -> Result<Self, ProductionWitnessError> {
        let manifest = verify_schema_assets(SCHEMA_MANIFEST_BYTES, &SCHEMA_ASSETS)?;
        let owners = current_owner_entries()?;
        let profiles = current_profile_entries()?;
        let mut capabilities = super::artifact_operation::current_artifact_capabilities()?;
        capabilities.push(super::flow_operation::current_flow_capability()?);
        capabilities.extend(super::pipeline_operation::current_pipeline_capabilities()?);
        capabilities.push(super::posture::current_posture_capability()?);
        capabilities.extend(super::contract_operation::current_contract_capabilities()?);
        capabilities.sort_by(|left, right| {
            left.operation()
                .reference()
                .as_str()
                .cmp(right.operation().reference().as_str())
        });
        Self::assemble(
            manifest,
            owners,
            profiles,
            capabilities,
            retention_until_utc,
        )
    }

    pub fn descriptor(&self) -> &BootstrapDescriptorV1 {
        &self.descriptor
    }

    pub fn catalog_roots(&self) -> &CatalogRoots {
        &self.roots
    }

    pub fn owner_versions(&self) -> &[OwnerVersionEntry] {
        &self.owner_versions
    }

    pub fn schema_manifest(&self) -> &[SchemaManifestEntry] {
        &self.schema_manifest
    }

    pub fn capabilities(&self) -> &[CapabilityEntry] {
        self.capabilities.entries()
    }

    pub fn profiles(&self) -> &[ProfileCatalogEntry] {
        self.profiles.entries()
    }

    pub(super) fn schema_bytes(&self, binding: &ExactBinding) -> Option<&'static [u8]> {
        let index = self
            .schema_manifest
            .iter()
            .position(|entry| entry.schema() == binding)?;
        let (reference, bytes) = SCHEMA_ASSETS.get(index)?;
        (*reference == binding.reference().as_str()).then_some(*bytes)
    }

    pub fn bootstrap(
        &self,
        request: &BootstrapRequestV1,
        now_utc: &UtcTimestamp,
        limits: &ContractLimits,
    ) -> Result<BootstrapOutcome, ContractError> {
        if let Err(reason) = self.descriptor.negotiate(request) {
            return Ok(BootstrapOutcome::Refused(Box::new(
                BootstrapRefusalV1::new(
                    self.descriptor.schemas().bootstrap_refusal().clone(),
                    request,
                    Some(self.descriptor.binding()),
                    reason,
                    BoundedVec::new(self.descriptor.supported_api_versions().to_vec())?,
                    BoundedVec::new(self.schema_manifest.clone())?,
                    limits,
                )?,
            )));
        }
        let first = match self
            .capabilities
            .page(request.page_size(), None, now_utc, limits)
        {
            Ok(page) => page,
            Err(_) => return Err(ContractError::InvalidInvariant),
        };
        Ok(BootstrapOutcome::Ok(Box::new(BootstrapResponseV1::new(
            self.descriptor.schemas().bootstrap_response().clone(),
            request,
            self.descriptor.clone(),
            self.roots.clone(),
            first,
            BoundedVec::new(self.owner_versions.clone())?,
            BoundedVec::new(self.schema_manifest.clone())?,
            limits,
        )?)))
    }

    pub fn read_schemas(
        &self,
        page_size: CatalogPageSize,
        cursor: Option<&TypedCatalogCursor>,
        now_utc: &UtcTimestamp,
        limits: &ContractLimits,
    ) -> Result<ProductionCatalogRead<SchemaCatalogPage>, ContractError> {
        self.map_read(
            self.schemas.page(page_size, cursor, now_utc, limits),
            limits,
        )
    }

    pub fn read_profiles(
        &self,
        page_size: CatalogPageSize,
        cursor: Option<&TypedCatalogCursor>,
        now_utc: &UtcTimestamp,
        limits: &ContractLimits,
    ) -> Result<ProductionCatalogRead<ProfileCatalogPage>, ContractError> {
        self.map_read(
            self.profiles.page(page_size, cursor, now_utc, limits),
            limits,
        )
    }

    pub fn read_capabilities(
        &self,
        page_size: CatalogPageSize,
        cursor: Option<&TypedCatalogCursor>,
        now_utc: &UtcTimestamp,
        limits: &ContractLimits,
    ) -> Result<ProductionCatalogRead<CapabilityCatalogPage>, ContractError> {
        self.map_read(
            self.capabilities.page(page_size, cursor, now_utc, limits),
            limits,
        )
    }

    fn map_read<T>(
        &self,
        result: Result<T, CatalogReadError>,
        limits: &ContractLimits,
    ) -> Result<ProductionCatalogRead<T>, ContractError> {
        match result {
            Ok(page) => Ok(ProductionCatalogRead::Page(page)),
            Err(CatalogReadError::CursorMismatch) => Ok(ProductionCatalogRead::CursorMismatch),
            Err(CatalogReadError::Expired { root }) => Ok(ProductionCatalogRead::Blocked(
                Box::new(CatalogReadBlockedV1::expired(
                    self.descriptor.schemas().catalog_read_blocked().clone(),
                    *root,
                    self.descriptor.schemas().catalog_root().clone(),
                    limits,
                )?),
            )),
        }
    }

    fn assemble(
        manifest: Vec<SchemaManifestEntry>,
        owner_versions: Vec<OwnerVersionEntry>,
        profiles: Vec<ProfileCatalogEntry>,
        capabilities: Vec<CapabilityEntry>,
        retention_until_utc: UtcTimestamp,
    ) -> Result<Self, ProductionWitnessError> {
        verify_inventory()?;
        validate_owner_entries(&owner_versions)?;
        validate_profile_entries(&profiles)?;
        validate_capabilities(&capabilities, &manifest, &owner_versions)?;
        let limits = ContractLimits::packet0();
        let bindings = BootstrapSchemaBindings::from_manifest(&manifest)?;
        let descriptor = BootstrapDescriptorV1::new(
            bindings,
            BoundedVec::new(vec![SemVer::parse("1.0.0")?])?,
            &limits,
        )?;
        let capabilities = ImmutableCatalog::new(
            descriptor.schemas().capability_catalog_page().clone(),
            descriptor.schemas().typed_catalog_cursor().clone(),
            descriptor.schemas().capability_entry().clone(),
            capabilities,
            retention_until_utc.clone(),
            &limits,
        )?;
        let manifest_entry_schema = manifest
            .iter()
            .find(|entry| {
                entry.schema().reference().as_str() == "handbook.schema-manifest-entry@1.0.0"
            })
            .ok_or(ProductionWitnessError::ManifestGap)?
            .schema()
            .clone();
        let schemas = ImmutableCatalog::new(
            descriptor.schemas().schema_catalog_page().clone(),
            descriptor.schemas().typed_catalog_cursor().clone(),
            manifest_entry_schema,
            manifest.clone(),
            retention_until_utc.clone(),
            &limits,
        )?;
        let profiles = ImmutableCatalog::new(
            descriptor.schemas().profile_catalog_page().clone(),
            descriptor.schemas().typed_catalog_cursor().clone(),
            descriptor.schemas().profile_catalog_entry().clone(),
            profiles,
            retention_until_utc,
            &limits,
        )?;
        let roots = CatalogRoots::new(
            descriptor.schemas().catalog_roots().clone(),
            capabilities.root().clone(),
            schemas.root().clone(),
            profiles.root().clone(),
            &limits,
        )?;
        Ok(Self {
            descriptor,
            owner_versions,
            schema_manifest: manifest,
            capabilities,
            schemas,
            profiles,
            roots,
        })
    }
}

fn verify_schema_assets(
    manifest_bytes: &[u8],
    assets: &[(&str, &[u8])],
) -> Result<Vec<SchemaManifestEntry>, ProductionWitnessError> {
    if manifest_bytes.is_empty() {
        return Err(ProductionWitnessError::ColdManifest);
    }
    if !manifest_bytes.ends_with(b"\n") || manifest_bytes.ends_with(b"\n\n") {
        return Err(ProductionWitnessError::StaleManifest);
    }
    let manifest = parse_canonical_json::<Vec<SchemaManifestEntry>>(
        &manifest_bytes[..manifest_bytes.len() - 1],
        &ContractLimits::packet0(),
    )
    .map_err(|_| ProductionWitnessError::StaleManifest)?;
    if manifest.len() != 135 || assets.len() != 135 || manifest.len() != assets.len() {
        return Err(ProductionWitnessError::ManifestCountMismatch);
    }
    let mut seen = BTreeSet::new();
    for (index, entry) in manifest.iter().enumerate() {
        let reference = entry.schema().reference().as_str();
        if !seen.insert(reference) {
            return Err(ProductionWitnessError::DuplicateSchema);
        }
        let (asset_ref, bytes) = assets
            .get(index)
            .ok_or(ProductionWitnessError::ManifestGap)?;
        if reference != *asset_ref {
            return Err(ProductionWitnessError::ManifestGap);
        }
        if entry.byte_length().get() != bytes.len() as u64
            || entry.schema().fingerprint() != &sha256_fingerprint(bytes)
        {
            return Err(ProductionWitnessError::TamperedSchema);
        }
    }
    Ok(manifest)
}

pub(super) fn current_schema_binding(reference: &str) -> Result<ExactBinding, ContractError> {
    verify_schema_assets(SCHEMA_MANIFEST_BYTES, &SCHEMA_ASSETS)
        .map_err(|_| ContractError::InvalidInvariant)?
        .into_iter()
        .find(|entry| entry.schema().reference().as_str() == reference)
        .map(|entry| entry.schema().clone())
        .ok_or(ContractError::InvalidInvariant)
}

fn verify_inventory() -> Result<(), ProductionWitnessError> {
    let rows = operation_inventory();
    let unique_operation_refs = rows
        .iter()
        .map(|row| row.operation_ref().as_str())
        .collect::<BTreeSet<_>>();
    if rows.len() != 62
        || unique_operation_refs.len() != rows.len()
        || rows
            .iter()
            .enumerate()
            .any(|(index, row)| row.proof_ordinal() != (index + 1) as u16)
        || rows.iter().any(|row| {
            let is_posture = row.operation_ref().as_str() == super::posture::OPERATION_REF;
            let is_artifact = super::artifact_operation::ARTIFACT_OPERATION_REFS
                .contains(&row.operation_ref().as_str());
            let is_flow = row.operation_ref().as_str() == super::flow_operation::FLOW_OPERATION_REF;
            let is_contract =
                super::contract_operation::inventory_row_is_contract(row.operation_ref());
            let is_pipeline = [
                "pipeline.catalog.list@1.0.0",
                "pipeline.catalog.read@1.0.0",
                "pipeline.compile@1.0.0",
                "pipeline.capture.plan@1.0.0",
                "pipeline.capture.apply@1.0.0",
                "pipeline.handoff.emit@1.0.0",
                "pipeline.state.apply@1.0.0",
            ]
            .contains(&row.operation_ref().as_str());
            if is_posture {
                row.admission_reason() != super::AdmissionReason::Admitted
                    || row.current_transports()
                        != [OperationTransport::RustSdk, OperationTransport::Tauri]
                    || row.discovery() != DiscoveryState::Discoverable
            } else if is_contract {
                row.admission_reason() != super::AdmissionReason::Admitted
                    || row.current_transports()
                        != [
                            OperationTransport::CliJson,
                            OperationTransport::RustSdk,
                            OperationTransport::Tauri,
                        ]
                    || row.discovery() != DiscoveryState::Discoverable
            } else if is_artifact || is_flow || is_pipeline {
                row.admission_reason() != super::AdmissionReason::Admitted
                    || row.current_transports()
                        != [OperationTransport::CliJson, OperationTransport::Tauri]
                    || row.discovery() != DiscoveryState::Discoverable
            } else {
                !row.current_transports().is_empty() || row.discovery() != DiscoveryState::Omit
            }
        })
    {
        return Err(ProductionWitnessError::InventoryMismatch);
    }
    let frozen = rows
        .iter()
        .map(|row| {
            serde_json::json!({
                "admission": row.admission_reason(),
                "availability": row.availability(),
                "class": row.class(),
                "operation": row.operation_ref(),
                "owner": row.owner(),
                "proof": row.proof_ordinal(),
            })
        })
        .collect::<Vec<_>>();
    let bytes = canonical_json_bytes(&frozen, &ContractLimits::packet0())?;
    if sha256_fingerprint(&bytes).as_str() != INVENTORY_DIGEST {
        return Err(ProductionWitnessError::InventoryMismatch);
    }
    Ok(())
}

fn current_owner_entries() -> Result<Vec<OwnerVersionEntry>, ProductionWitnessError> {
    let contracts = handbook_contracts::operation_admission::operation_owner_version_evidence();
    let engine = handbook_engine::operation_admission::operation_owner_version_evidence();
    let flow = handbook_flow::operation_admission::operation_owner_version_evidence();
    let pipeline = handbook_pipeline::operation_admission::operation_owner_version_evidence();
    [
        (
            contracts.owner_crate,
            contracts.owner_version,
            contracts.owner_source_ref,
            contracts.source_bytes,
        ),
        (
            engine.owner_crate,
            engine.owner_version,
            engine.owner_source_ref,
            engine.source_bytes,
        ),
        (
            flow.owner_crate,
            flow.owner_version,
            flow.owner_source_ref,
            flow.source_bytes,
        ),
        (
            pipeline.owner_crate,
            pipeline.owner_version,
            pipeline.owner_source_ref,
            pipeline.source_bytes,
        ),
    ]
    .into_iter()
    .map(|(owner, version, source_ref, bytes)| {
        Ok(OwnerVersionEntry::new(
            BoundedString::new(owner.to_owned())?,
            SemVer::parse(version)?,
            ExactBinding::new(ExactRef::parse(source_ref)?, sha256_fingerprint(bytes)),
            &ContractLimits::packet0(),
        )?)
    })
    .collect()
}

fn current_profile_entries() -> Result<Vec<ProfileCatalogEntry>, ProductionWitnessError> {
    handbook_engine::operation_admission::shipped_profile_evidence()
        .into_iter()
        .map(|profile| {
            let profile_fingerprint =
                profile_content_fingerprint(profile.profile_ref, profile.source_bytes);
            let source_fingerprint = sha256_fingerprint(profile.source_bytes);
            Ok(ProfileCatalogEntry::new(
                ExactBinding::new(ExactRef::parse(profile.profile_ref)?, profile_fingerprint),
                ExactBinding::new(ExactRef::parse(profile.source_ref)?, source_fingerprint),
                &ContractLimits::packet0(),
            )?)
        })
        .collect()
}

fn profile_content_fingerprint(profile_ref: &str, source_bytes: &[u8]) -> Fingerprint {
    let mut content = Vec::with_capacity(30 + profile_ref.len() + source_bytes.len());
    content.extend_from_slice(b"handbook.profile-content-v1\0");
    content.extend_from_slice(profile_ref.as_bytes());
    content.push(0);
    content.extend_from_slice(source_bytes);
    sha256_fingerprint(&content)
}

fn validate_owner_entries(entries: &[OwnerVersionEntry]) -> Result<(), ProductionWitnessError> {
    let expected = [
        (
            "handbook-contracts",
            "0.1.0",
            "handbook.owner.contracts@0.1.0",
        ),
        ("handbook-engine", "0.2.0", "handbook.owner.engine@0.2.0"),
        ("handbook-flow", "0.1.1", "handbook.owner.flow@0.1.1"),
        (
            "handbook-pipeline",
            "0.1.2",
            "handbook.owner.pipeline@0.1.2",
        ),
    ];
    if entries.len() != expected.len()
        || entries.iter().zip(expected).any(|(entry, expected)| {
            entry.owner_crate() != expected.0
                || entry.owner_version().as_str() != expected.1
                || entry.source().reference().as_str() != expected.2
        })
    {
        return Err(ProductionWitnessError::OwnerEvidenceMismatch);
    }
    Ok(())
}

fn validate_profile_entries(entries: &[ProfileCatalogEntry]) -> Result<(), ProductionWitnessError> {
    if entries.len() != 3 {
        return Err(ProductionWitnessError::MissingProfile);
    }
    let mut seen = BTreeSet::new();
    for (index, entry) in entries.iter().enumerate() {
        let expected = format!("handbook.profile.shipped-root@1.{index}.0");
        if !seen.insert(entry.profile().reference().as_str()) {
            return Err(ProductionWitnessError::DuplicateProfile);
        }
        if entry.profile().reference().as_str() != expected {
            return Err(ProductionWitnessError::MissingProfile);
        }
    }
    Ok(())
}

fn validate_capabilities(
    entries: &[CapabilityEntry],
    manifest: &[SchemaManifestEntry],
    owners: &[OwnerVersionEntry],
) -> Result<(), ProductionWitnessError> {
    let schemas = manifest
        .iter()
        .map(|entry| entry.schema().reference().as_str())
        .collect::<BTreeSet<_>>();
    let owners = owners
        .iter()
        .map(|entry| (entry.owner_crate(), entry.owner_version().as_str()))
        .collect::<BTreeSet<_>>();
    for entry in entries {
        let row = operation_inventory()
            .iter()
            .find(|row| row.operation_ref() == entry.operation().reference())
            .ok_or(ProductionWitnessError::UnknownOperation)?;
        if !owners.contains(&(
            entry.owner().owner_crate(),
            entry.owner().owner_version().as_str(),
        )) {
            return Err(ProductionWitnessError::OwnerEvidenceMismatch);
        }
        for schema in [
            entry.request_schema(),
            entry.result_schema(),
            entry.outcome_schemas().blocker(),
            entry.outcome_schemas().refusal(),
            entry.outcome_schemas().error(),
        ] {
            if !schemas.contains(schema.reference().as_str()) {
                return Err(ProductionWitnessError::MissingOperationSchema);
            }
        }
        if row.discovery() != DiscoveryState::Discoverable
            || row.current_transports().is_empty()
            || entry.current_transports().is_empty()
            || entry.current_transports() != row.current_transports()
            || entry.current_transports().iter().any(|transport| {
                !matches!(
                    transport,
                    OperationTransport::CliJson
                        | OperationTransport::RustSdk
                        | OperationTransport::Tauri
                )
            })
        {
            return Err(ProductionWitnessError::UnprovenTransport);
        }
    }
    let discoverable = operation_inventory()
        .iter()
        .filter(|row| row.discovery() == DiscoveryState::Discoverable)
        .map(|row| row.operation_ref().as_str())
        .collect::<BTreeSet<_>>();
    if entries.len() != discoverable.len()
        || entries
            .iter()
            .any(|entry| !discoverable.contains(entry.operation().reference().as_str()))
    {
        return Err(ProductionWitnessError::UnprovenTransport);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn production_witness_is_independent_exact_and_exposes_only_proven_capability() {
        let witness =
            ProductionWitness::from_current(UtcTimestamp::parse("2030-01-01T00:00:00Z").unwrap())
                .unwrap();
        assert_eq!(witness.schema_manifest().len(), 135);
        assert_eq!(witness.owner_versions().len(), 4);
        assert_eq!(witness.profiles().len(), 3);
        assert_eq!(witness.capabilities().len(), 23);
        let posture = witness
            .capabilities()
            .iter()
            .find(|entry| {
                entry.operation().reference().as_str() == super::super::posture::OPERATION_REF
            })
            .unwrap();
        assert_eq!(
            posture.current_transports(),
            &[OperationTransport::RustSdk, OperationTransport::Tauri]
        );
        for operation_ref in super::super::artifact_operation::ARTIFACT_OPERATION_REFS {
            let capability = witness
                .capabilities()
                .iter()
                .find(|entry| entry.operation().reference().as_str() == operation_ref)
                .unwrap();
            let expected: &[OperationTransport] =
                &[OperationTransport::CliJson, OperationTransport::Tauri];
            assert_eq!(capability.current_transports(), expected);
        }
        let flow = witness
            .capabilities()
            .iter()
            .find(|entry| entry.operation().reference().as_str() == "flow.resolve@1.0.0")
            .unwrap();
        assert_eq!(
            flow.current_transports(),
            &[OperationTransport::CliJson, OperationTransport::Tauri]
        );
        for operation_ref in super::super::contract_operation::CONTRACT_OPERATION_REFS {
            let capability = witness
                .capabilities()
                .iter()
                .find(|entry| entry.operation().reference().as_str() == operation_ref)
                .unwrap();
            assert_eq!(
                capability.current_transports(),
                &[
                    OperationTransport::CliJson,
                    OperationTransport::RustSdk,
                    OperationTransport::Tauri,
                ]
            );
        }
        assert_eq!(
            witness.catalog_roots().operations().total_entries().get(),
            23
        );
        assert_ne!(
            witness.descriptor().schema().fingerprint(),
            witness.descriptor().descriptor_content_fingerprint()
        );
        let expected_profile_fingerprints = [
            "sha256:dfaf429674838bc892ee32791770a45b0ad6856bbc19d7f62a334bcf83017b00",
            "sha256:4cd1e2a0036847b08f30f13218d6793ac48c8684bb18cd9ce954cc760bd6f055",
            "sha256:9a8fcd9f9f1e95603e5619ffb87e74f94939f9a5ca67d902496d3030b5f269c1",
        ];
        let expected_source_fingerprints = [
            "sha256:5439176ee37aaa8163d04b99fb0231adaf77f5bb43c4cb59f70251ad45ec4303",
            "sha256:56f92ee105319b609fe21bf299770cd118df121f23e9fcc8c83ebd4b64d66557",
            "sha256:5090e56e503c98df20df61a27ef946b239a0d0e9fb26d712427514a4697ed42f",
        ];
        for ((entry, profile_fingerprint), source_fingerprint) in witness
            .profiles()
            .iter()
            .zip(expected_profile_fingerprints)
            .zip(expected_source_fingerprints)
        {
            assert_eq!(entry.profile().fingerprint().as_str(), profile_fingerprint);
            assert_eq!(entry.source().fingerprint().as_str(), source_fingerprint);
            assert_ne!(entry.profile().fingerprint(), entry.source().fingerprint());
        }
    }

    #[test]
    fn manifest_cold_stale_tampered_duplicate_gap_and_count_mismatch_fail_closed() {
        assert_eq!(
            verify_schema_assets(&[], &SCHEMA_ASSETS),
            Err(ProductionWitnessError::ColdManifest)
        );
        assert_eq!(
            verify_schema_assets(b"[]", &SCHEMA_ASSETS),
            Err(ProductionWitnessError::StaleManifest)
        );
        let mut assets = SCHEMA_ASSETS;
        assets[0].1 = b"tampered";
        assert_eq!(
            verify_schema_assets(SCHEMA_MANIFEST_BYTES, &assets),
            Err(ProductionWitnessError::TamperedSchema)
        );
        let mut gap = SCHEMA_ASSETS;
        gap.swap(0, 1);
        assert_eq!(
            verify_schema_assets(SCHEMA_MANIFEST_BYTES, &gap),
            Err(ProductionWitnessError::ManifestGap)
        );

        let mut manifest = verify_schema_assets(SCHEMA_MANIFEST_BYTES, &SCHEMA_ASSETS).unwrap();
        manifest[1] = manifest[0].clone();
        let mut bytes = canonical_json_bytes(&manifest, &ContractLimits::packet0()).unwrap();
        bytes.push(b'\n');
        assert_eq!(
            verify_schema_assets(&bytes, &SCHEMA_ASSETS),
            Err(ProductionWitnessError::DuplicateSchema)
        );
        manifest.pop();
        let mut bytes = canonical_json_bytes(&manifest, &ContractLimits::packet0()).unwrap();
        bytes.push(b'\n');
        assert_eq!(
            verify_schema_assets(&bytes, &SCHEMA_ASSETS),
            Err(ProductionWitnessError::ManifestCountMismatch)
        );
    }

    #[test]
    fn missing_duplicate_profile_and_unknown_capability_are_refused() {
        let mut profiles = current_profile_entries().unwrap();
        assert_eq!(
            validate_profile_entries(&profiles[..2]),
            Err(ProductionWitnessError::MissingProfile)
        );
        profiles[1] = profiles[0].clone();
        assert_eq!(
            validate_profile_entries(&profiles),
            Err(ProductionWitnessError::DuplicateProfile)
        );

        let definition = super::super::OperationDefinition::new(
            ExactBinding::new(
                ExactRef::parse("handbook.operation-definition@1.0.0").unwrap(),
                sha256_fingerprint(b"definition-schema"),
            ),
            ExactRef::parse("unknown.operation@1.0.0").unwrap(),
            super::super::OperationOwnerBinding::new(
                BoundedString::new("handbook-engine".to_owned()).unwrap(),
                SemVer::parse("0.2.0").unwrap(),
                ExactBinding::new(
                    ExactRef::parse("handbook.owner.engine@0.2.0").unwrap(),
                    sha256_fingerprint(b"engine"),
                ),
            )
            .unwrap(),
            ExactBinding::new(
                ExactRef::parse("handbook.operation.unknown-operation-request@1.0.0").unwrap(),
                sha256_fingerprint(b"request"),
            ),
            ExactBinding::new(
                ExactRef::parse("handbook.operation.unknown-operation-result@1.0.0").unwrap(),
                sha256_fingerprint(b"result"),
            ),
            super::super::OperationOutcomeSchemas::new(
                ExactBinding::new(
                    ExactRef::parse("handbook.operation-blocker@1.0.0").unwrap(),
                    sha256_fingerprint(b"blocker"),
                ),
                ExactBinding::new(
                    ExactRef::parse("handbook.operation-refusal@1.0.0").unwrap(),
                    sha256_fingerprint(b"refusal"),
                ),
                ExactBinding::new(
                    ExactRef::parse("handbook.operation-error@1.0.0").unwrap(),
                    sha256_fingerprint(b"error"),
                ),
            )
            .unwrap(),
            BoundedVec::new(Vec::new()).unwrap(),
            super::super::OperationClass::ReadOnly,
            super::super::OperationIdempotency::safe(),
            BoundedVec::new(Vec::new()).unwrap(),
            BoundedVec::new(vec![
                OperationTransport::CliJson,
                OperationTransport::RustSdk,
                OperationTransport::Tauri,
            ])
            .unwrap(),
            None,
            &ContractLimits::packet0(),
        )
        .unwrap();
        let capability = CapabilityEntry::structurally_admitted(
            &definition,
            BoundedVec::new(vec![OperationTransport::RustSdk]).unwrap(),
            &ContractLimits::packet0(),
        )
        .unwrap();
        assert_eq!(
            validate_capabilities(
                &[capability],
                &verify_schema_assets(SCHEMA_MANIFEST_BYTES, &SCHEMA_ASSETS).unwrap(),
                &current_owner_entries().unwrap(),
            ),
            Err(ProductionWitnessError::UnknownOperation)
        );

        let known_definition = super::super::OperationDefinition::new(
            definition.definition_schema().clone(),
            ExactRef::parse("artifact.validate@1.0.0").unwrap(),
            definition.owner().clone(),
            ExactBinding::new(
                ExactRef::parse("handbook.operation.artifact-validate-request@1.0.0").unwrap(),
                sha256_fingerprint(b"request"),
            ),
            ExactBinding::new(
                ExactRef::parse("handbook.operation.artifact-validate-result@1.0.0").unwrap(),
                sha256_fingerprint(b"result"),
            ),
            definition.outcome_schemas().clone(),
            BoundedVec::new(definition.required_capabilities().to_vec()).unwrap(),
            definition.class(),
            definition.idempotency().clone(),
            BoundedVec::new(definition.write_set().to_vec()).unwrap(),
            BoundedVec::new(definition.declared_transports().to_vec()).unwrap(),
            definition.deprecation().cloned(),
            &ContractLimits::packet0(),
        )
        .unwrap();
        let capability = CapabilityEntry::structurally_admitted(
            &known_definition,
            BoundedVec::new(vec![OperationTransport::RustSdk]).unwrap(),
            &ContractLimits::packet0(),
        )
        .unwrap();
        assert_eq!(
            validate_capabilities(
                &[capability],
                &verify_schema_assets(SCHEMA_MANIFEST_BYTES, &SCHEMA_ASSETS).unwrap(),
                &current_owner_entries().unwrap(),
            ),
            Err(ProductionWitnessError::UnprovenTransport)
        );
    }

    #[test]
    fn bootstrap_negotiation_and_expired_restart_are_deterministic() {
        let limits = ContractLimits::packet0();
        let witness =
            ProductionWitness::from_current(UtcTimestamp::parse("2030-01-01T00:00:00Z").unwrap())
                .unwrap();
        let request = |binding: ExactBinding, version: &str| {
            BootstrapRequestV1::new(
                witness.descriptor().schemas().bootstrap_request().clone(),
                None,
                binding,
                SemVer::parse(version).unwrap(),
                CatalogPageSize::new(1).unwrap(),
                &limits,
            )
            .unwrap()
        };
        assert!(matches!(
            witness
                .bootstrap(
                    &request(witness.descriptor().binding(), "1.0.0"),
                    &UtcTimestamp::parse("2029-01-01T00:00:00Z").unwrap(),
                    &limits,
                )
                .unwrap(),
            BootstrapOutcome::Ok(_)
        ));
        for (bad, reason) in [
            (
                ExactBinding::new(
                    witness.descriptor().descriptor().clone(),
                    sha256_fingerprint(b"tampered"),
                ),
                super::super::BootstrapRefusalReason::TamperedDescriptor,
            ),
            (
                ExactBinding::new(
                    ExactRef::parse("handbook.bootstrap-descriptor@1.0.1").unwrap(),
                    sha256_fingerprint(b"stale"),
                ),
                super::super::BootstrapRefusalReason::StaleDescriptor,
            ),
            (
                ExactBinding::new(
                    ExactRef::parse("handbook.bootstrap-descriptor@2.0.0").unwrap(),
                    sha256_fingerprint(b"cross"),
                ),
                super::super::BootstrapRefusalReason::CrossMajorDescriptor,
            ),
        ] {
            match witness
                .bootstrap(
                    &request(bad, "1.0.0"),
                    &UtcTimestamp::parse("2029-01-01T00:00:00Z").unwrap(),
                    &limits,
                )
                .unwrap()
            {
                BootstrapOutcome::Refused(refusal) => {
                    assert_eq!(refusal.refusal_reason(), Some(reason));
                }
                BootstrapOutcome::Ok(_) => panic!("invalid descriptor was accepted"),
            }
        }
        match witness
            .bootstrap(
                &request(witness.descriptor().binding(), "2.0.0"),
                &UtcTimestamp::parse("2029-01-01T00:00:00Z").unwrap(),
                &limits,
            )
            .unwrap()
        {
            BootstrapOutcome::Refused(refusal) => assert_eq!(
                refusal.refusal_reason(),
                Some(super::super::BootstrapRefusalReason::UnsupportedApiMajor)
            ),
            BootstrapOutcome::Ok(_) => panic!("unsupported major was accepted"),
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
                assert!(blocked.restart_from_first_page());
                let recheck = blocked.blockers()[0].recheck().unwrap();
                assert_eq!(
                    recheck.condition_schema(),
                    witness.descriptor().schemas().catalog_root()
                );
                let root_bytes = canonical_json_bytes(recheck.condition(), &limits).unwrap();
                let root = parse_canonical_json::<super::super::CatalogRoot>(&root_bytes, &limits)
                    .unwrap();
                assert_eq!(root, witness.catalog_roots().schemas().clone());
            }
            _ => panic!("expired catalog did not return a blocked restart"),
        }
    }
}
