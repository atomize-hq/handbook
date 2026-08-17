//! Exact Pipeline-owned admission claims for Packet-6 read-only operations.
//!
//! This registry intentionally lives outside `operation_admission.rs` because
//! that file is the owner-source evidence preimage carried by each definition.

use crate::operation_admission::OperationAdmissionClaim;

pub(crate) fn matches(claim: &OperationAdmissionClaim<'_>) -> bool {
    PIPELINE_READ_OPERATIONS
        .iter()
        .any(|expected| claim == expected)
}

const PIPELINE_READ_OPERATIONS: [OperationAdmissionClaim<'static>; 3] = [
    read_claim(
        "pipeline.catalog.list@1.0.0",
        "sha256:e1042da34e947a306a0d0161989b5f62bebeb38277b058972a98828fdf836982",
        "handbook.operation.pipeline-catalog-list-request@1.0.0",
        "sha256:e1dee1816e73dff88d397de58e40880a1727a2ffb95bb7bf9c2194f02a0a1b0d",
        "handbook.operation.pipeline-catalog-list-result@1.0.0",
        "sha256:d0211a5bcbaba228a4830aaccb93fa6f8b0266c247ccf3677dcb94f8940ad06e",
    ),
    read_claim(
        "pipeline.catalog.read@1.0.0",
        "sha256:806c020409ec4e95c1fac0cc5b1dc6d651432eb224713466eb27e7842d528470",
        "handbook.operation.pipeline-catalog-read-request@1.0.0",
        "sha256:8732315b3c3813017404b002bab67e548c3aaece02b53d61e39f32cb4488a730",
        "handbook.operation.pipeline-catalog-read-result@1.0.0",
        "sha256:c1500bf9b6c5127056060be4b3d91442455042763f256af6c4dbe347cfb61ac0",
    ),
    read_claim(
        "pipeline.compile@1.0.0",
        "sha256:2d7c637ceca7ea59bd323a7e642202028ea59c0271e8906f3d28f63d8d3170f0",
        "handbook.operation.pipeline-compile-request@1.0.0",
        "sha256:c4fa8f59d8fb13f861a1cb8e6b73f1378a4c09c83ec9aaa78968975157081a27",
        "handbook.operation.pipeline-compile-result@1.0.0",
        "sha256:8e10d58a4fb53a36f1ff4792ee2fde197658ae3796286b47bf492ca5ac466efc",
    ),
];

const fn read_claim(
    operation_ref: &'static str,
    definition_fingerprint: &'static str,
    request_schema_ref: &'static str,
    request_schema_fingerprint: &'static str,
    result_schema_ref: &'static str,
    result_schema_fingerprint: &'static str,
) -> OperationAdmissionClaim<'static> {
    OperationAdmissionClaim {
        operation_ref,
        definition_fingerprint,
        owner_crate: "handbook-pipeline",
        owner_version: "0.1.2",
        owner_source_ref: "handbook.owner.pipeline@0.1.2",
        owner_source_fingerprint:
            "sha256:e498d7a6cc903d283ce33a90987a784f08a2da6c58e593b21d34234592a61c7b",
        mutability: "read_only",
        idempotency: "safe",
        idempotency_fingerprint:
            "sha256:e3a5961ff3a3a8141099426d699e73c9d6dd073456c0721a8cbd4a2f9bb9015d",
        definition_schema_ref: "handbook.operation-definition@1.0.0",
        definition_schema_fingerprint:
            "sha256:e668b32cb7ef3725e4d8c53696ec9032ba0741e387436883a7d8f264602ddd09",
        request_schema_ref,
        request_schema_fingerprint,
        result_schema_ref,
        result_schema_fingerprint,
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
    }
}
