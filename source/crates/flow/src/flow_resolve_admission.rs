//! Exact Flow-owned admission claim for `flow.resolve@1.0.0`.
//!
//! This registry intentionally lives outside `operation_admission.rs` because
//! that file is the owner-source evidence preimage carried by the definition.

use crate::operation_admission::OperationAdmissionClaim;

pub(crate) fn matches(claim: &OperationAdmissionClaim<'_>) -> bool {
    claim == &FLOW_RESOLVE
}

const FLOW_RESOLVE: OperationAdmissionClaim<'static> = OperationAdmissionClaim {
    operation_ref: "flow.resolve@1.0.0",
    definition_fingerprint:
        "sha256:474294e652345777b3b1072a9b355bbda4cc70cbe739245191b819ed6c4d2942",
    owner_crate: "handbook-flow",
    owner_version: "0.1.1",
    owner_source_ref: "handbook.owner.flow@0.1.1",
    owner_source_fingerprint:
        "sha256:d5c2c556cfd97c44d3e8226dd055da00f747b241643250aa045fcb16b25d7467",
    mutability: "read_only",
    idempotency: "safe",
    idempotency_fingerprint:
        "sha256:e3a5961ff3a3a8141099426d699e73c9d6dd073456c0721a8cbd4a2f9bb9015d",
    definition_schema_ref: "handbook.operation-definition@1.0.0",
    definition_schema_fingerprint:
        "sha256:e668b32cb7ef3725e4d8c53696ec9032ba0741e387436883a7d8f264602ddd09",
    request_schema_ref: "handbook.operation.flow-resolve-request@1.0.0",
    request_schema_fingerprint:
        "sha256:03245c10d3a47d0b81b22152253ed5b0acd5bad59aa773d8e35a8a0771dc3bd9",
    result_schema_ref: "handbook.operation.flow-resolve-result@1.0.0",
    result_schema_fingerprint:
        "sha256:9e3910c95b6ee18a03ea0abd57d36a22217092b830181deb4707da758d7be8ef",
    blocker_schema_ref: "handbook.operation-blocker@1.0.0",
    blocker_schema_fingerprint:
        "sha256:1125afb3a783af6ed4ee5b8caad86e00264420103d18db36e0337e34b889a009",
    refusal_schema_ref: "handbook.operation-refusal@1.0.0",
    refusal_schema_fingerprint:
        "sha256:16357baa6276ae2d6b27e22296e789db78d646403845f98653148178afec8b23",
    error_schema_ref: "handbook.operation-error@1.0.0",
    error_schema_fingerprint:
        "sha256:569cce41b965e3140591ed8c75ebad39aef2809c57e82b5c8f50db639d634355",
    write_set_fingerprint:
        "sha256:4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945",
    required_capabilities_fingerprint:
        "sha256:4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945",
};
