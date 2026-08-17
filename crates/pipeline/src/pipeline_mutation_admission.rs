//! Exact Pipeline-owned admission claims for Packet-7 ordinary mutation operations.

use crate::operation_admission::OperationAdmissionClaim;

pub(crate) fn matches(claim: &OperationAdmissionClaim<'_>) -> bool {
    PIPELINE_MUTATION_OPERATIONS
        .iter()
        .any(|expected| claim == expected)
}

const PIPELINE_MUTATION_OPERATIONS: [OperationAdmissionClaim<'static>; 4] = [
    mutation_claim(
        "pipeline.capture.plan@1.0.0",
        "sha256:11511ba076bb47379c2042be379d8046ddc36276b38323ccc63061577b4045e8",
        "read_only",
        "safe",
        "sha256:e3a5961ff3a3a8141099426d699e73c9d6dd073456c0721a8cbd4a2f9bb9015d",
        "handbook.operation.pipeline-capture-plan-request@1.0.0",
        "sha256:6c267140d4791a3747f7ae6863244c4d8fa19099f549fc51f380ef8794a83934",
        "handbook.operation.pipeline-capture-plan-result@1.0.0",
        "sha256:14e764375ca4c01562a5ac90a0d656fee25295c92bf7299630577c1be0ac6173",
        "sha256:4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945",
    ),
    mutation_claim(
        "pipeline.capture.apply@1.0.0",
        "sha256:808fc5b72aab1359bd5e7c9b5c0846f8f70c9fc4345b0c2a6f00bb5c2da084e8",
        "compare_and_write",
        "compare_and_write_required",
        "sha256:0e5b5c4f07a81f6cb1e74664a8ba63d5a98100732f34947c27af77cfba4a2d70",
        "handbook.operation.pipeline-capture-apply-request@1.0.0",
        "sha256:5d7bc76a15b0b7779207ea54b0a76863635fec953d5d1c5f3c145ad046be83e0",
        "handbook.operation.pipeline-capture-apply-result@1.0.0",
        "sha256:4bf787bec50f2f177ebe9e3f5cdaef92bd825fe4b97790ccf43994a9253c4225",
        "sha256:bcd71fc3f3bed56056b736786d52f878b90a47aaefcf067873c667381ba57f39",
    ),
    mutation_claim(
        "pipeline.handoff.emit@1.0.0",
        "sha256:e17f542c781ab0aa1adf7e304838d87c8b9a62ad100a834b3ff2b6adcbe94e0f",
        "append_only",
        "idempotency_key_required",
        "sha256:fb3eaacbcf74cee49007b3dda325f36e905ee49a4dc39f7c2440736f923412c1",
        "handbook.operation.pipeline-handoff-emit-request@1.0.0",
        "sha256:cb719229809ba6aa2971645540f6cdfc4ac0b04680332ab850608b700994a2a8",
        "handbook.operation.pipeline-handoff-emit-result@1.0.0",
        "sha256:c4571e86d52a578bce63c7cce7e462744be1415105bca23e3bb55109e8f492a9",
        "sha256:a3088bb5ac47445901ef78823653292e2613632425834fa923b90ec9a8279584",
    ),
    mutation_claim(
        "pipeline.state.apply@1.0.0",
        "sha256:8db9a1975636692c5abe2ba1821d40a2883e5894ae992d5f26c105a08725c95d",
        "compare_and_write",
        "compare_and_write_required",
        "sha256:956a142cabb88dfeea128c6e1fca97b3bce681b567c9648a701d88e0f2193dfb",
        "handbook.operation.pipeline-state-apply-request@1.0.0",
        "sha256:9d524a43df44852648f29eb1566c9ee479f334682eb6bfaa73c41df07334c8d3",
        "handbook.operation.pipeline-state-apply-result@1.0.0",
        "sha256:8d2a25c3a782ba949f4ba4fb68b523aa8dfa2ebfe39d24fa9568268d61ea79dd",
        "sha256:27ecd44a2f3b8949af8ce6c060e82dbd4e465d3490c5b89809f9dd0b17d61a4c",
    ),
];

#[allow(clippy::too_many_arguments)]
const fn mutation_claim(
    operation_ref: &'static str,
    definition_fingerprint: &'static str,
    mutability: &'static str,
    idempotency: &'static str,
    idempotency_fingerprint: &'static str,
    request_schema_ref: &'static str,
    request_schema_fingerprint: &'static str,
    result_schema_ref: &'static str,
    result_schema_fingerprint: &'static str,
    write_set_fingerprint: &'static str,
) -> OperationAdmissionClaim<'static> {
    OperationAdmissionClaim {
        operation_ref,
        definition_fingerprint,
        owner_crate: "handbook-pipeline",
        owner_version: "0.1.2",
        owner_source_ref: "handbook.owner.pipeline@0.1.2",
        owner_source_fingerprint:
            "sha256:e498d7a6cc903d283ce33a90987a784f08a2da6c58e593b21d34234592a61c7b",
        mutability,
        idempotency,
        idempotency_fingerprint,
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
        write_set_fingerprint,
        required_capabilities_fingerprint:
            "sha256:4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945",
    }
}
