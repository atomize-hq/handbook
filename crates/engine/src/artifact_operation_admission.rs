//! Exact Engine admission for the ten HCM-4.3 artifact/intake operation definitions.

pub(crate) fn matches_artifact_operation_claim(
    claim: &crate::operation_admission::OperationAdmissionClaim<'_>,
) -> bool {
    let Some(expected) = EXPECTED
        .iter()
        .find(|expected| expected.operation_ref == claim.operation_ref)
    else {
        return false;
    };
    claim.definition_fingerprint == expected.definition_fingerprint
        && claim.owner_crate == "handbook-engine"
        && claim.owner_version == "0.2.0"
        && claim.owner_source_ref == "handbook.owner.engine@0.2.0"
        && claim.owner_source_fingerprint
            == "sha256:99df01064d1f6a21e7db74a5746936e7af9dee784029a2850439b7ff64696e40"
        && claim.definition_schema_ref == "handbook.operation-definition@1.0.0"
        && claim.definition_schema_fingerprint
            == "sha256:e668b32cb7ef3725e4d8c53696ec9032ba0741e387436883a7d8f264602ddd09"
        && claim.request_schema_ref == expected.request_schema_ref
        && claim.request_schema_fingerprint == expected.request_schema_fingerprint
        && claim.result_schema_ref == expected.result_schema_ref
        && claim.result_schema_fingerprint == expected.result_schema_fingerprint
        && claim.blocker_schema_ref == "handbook.operation-blocker@1.0.0"
        && claim.blocker_schema_fingerprint
            == "sha256:1125afb3a783af6ed4ee5b8caad86e00264420103d18db36e0337e34b889a009"
        && claim.refusal_schema_ref == "handbook.operation-refusal@1.0.0"
        && claim.refusal_schema_fingerprint
            == "sha256:16357baa6276ae2d6b27e22296e789db78d646403845f98653148178afec8b23"
        && claim.error_schema_ref == "handbook.operation-error@1.0.0"
        && claim.error_schema_fingerprint
            == "sha256:569cce41b965e3140591ed8c75ebad39aef2809c57e82b5c8f50db639d634355"
        && claim.mutability == expected.mutability
        && claim.idempotency == expected.idempotency
        && claim.idempotency_fingerprint == expected.idempotency_fingerprint
        && claim.write_set_fingerprint == expected.write_set_fingerprint
        && claim.required_capabilities_fingerprint
            == "sha256:4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945"
}

struct ExpectedArtifactOperationClaim {
    operation_ref: &'static str,
    definition_fingerprint: &'static str,
    request_schema_ref: &'static str,
    request_schema_fingerprint: &'static str,
    result_schema_ref: &'static str,
    result_schema_fingerprint: &'static str,
    mutability: &'static str,
    idempotency: &'static str,
    idempotency_fingerprint: &'static str,
    write_set_fingerprint: &'static str,
}

const SAFE_IDEMPOTENCY: &str =
    "sha256:e3a5961ff3a3a8141099426d699e73c9d6dd073456c0721a8cbd4a2f9bb9015d";
const EMPTY_WRITE_SET: &str =
    "sha256:4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945";

const EXPECTED: [ExpectedArtifactOperationClaim; 10] = [
    expected(
        "artifact.kind.list@1.0.0",
        "sha256:3b4d21bb5ae7010a3b7ec964bef8bfd492b265501b94ba8c22f9f519cb33e418",
        "handbook.operation.artifact-kind-list-request@1.0.0",
        "sha256:a9b6deec437bb2bc54c911110095acc6df9e85d985f80b01470f889305fe07b7",
        "handbook.operation.artifact-kind-list-result@1.0.0",
        "sha256:2245fd4b2e5b9a26b9af20fb08141169501df0bdd0b33a7d62dbef616df04b51",
    ),
    expected(
        "artifact.instance.list@1.0.0",
        "sha256:ecc0a0720b388ade0575071e0bd8a3c299803e29293b44dc5a6e44197e59ea66",
        "handbook.operation.artifact-instance-list-request@1.0.0",
        "sha256:0a1f9c7234a29a90479067becb6e47b3e11d8b7acf9f86c3abb7a6280d768c1d",
        "handbook.operation.artifact-instance-list-result@1.0.0",
        "sha256:d78f6b8dce9564f559ae8eabe3cca03eecf15f2d08c0b857ef02a4108955ae42",
    ),
    expected(
        "artifact.read@1.0.0",
        "sha256:4166d40864139cb08a3fc60814d1753066d72c16c23d7e413ee4b091ae4b0697",
        "handbook.operation.artifact-read-request@1.0.0",
        "sha256:7594f11c64815dc3d16ae60c9da2634679c3f4a077d6e30428139c7d2608c86c",
        "handbook.operation.artifact-read-result@1.0.0",
        "sha256:902a7072190e7889c4839f3149402a2319c124c844adca3ad21b590113a74e6c",
    ),
    expected(
        "artifact.validate@1.0.0",
        "sha256:6839ab03729bbedcd2754070fb8a22cc50e859a3d9ac54a4e1b42334de4f1e77",
        "handbook.operation.artifact-validate-request@1.0.0",
        "sha256:a8892f76eb5d6b2f458d6a4418886de781d2f47dfe1b81e1fa0b6a68748e92fd",
        "handbook.operation.artifact-validate-result@1.0.0",
        "sha256:4f90a93f305e68c64dee5526d6483c405a5c67389e4daa89feab75c56b5a45a2",
    ),
    expected(
        "intake.definition.read@1.0.0",
        "sha256:fad0dabd3a53f1adeb21a94124c8eed0b5dd5728d275a7535341edb73408aa82",
        "handbook.operation.intake-definition-read-request@1.0.0",
        "sha256:a0e887f0378ddbda1b2cd71e869f6d74e56ae03c953cfbe87cdc136d3dd7076b",
        "handbook.operation.intake-definition-read-result@1.0.0",
        "sha256:ccadceedb452a4d3cfe8a23bf26142cebf001a7f6562a1437446226e77adee24",
    ),
    expected(
        "intake.coverage.evaluate@1.0.0",
        "sha256:c34dc2c706e19818a21adc3d67edf6ddcd31cf7a3a98d32840e3305a52c26134",
        "handbook.operation.intake-coverage-evaluate-request@1.0.0",
        "sha256:401ea1065ae0aa9759e09ca300ff2c17bc00a12c23aca5b551ff156483ec5765",
        "handbook.operation.intake-coverage-evaluate-result@1.0.0",
        "sha256:906f5f8b3f7ef1d9f6eeb1a77aed6827edf1d7698cd430ab3a00949f02062e07",
    ),
    ExpectedArtifactOperationClaim {
        operation_ref: "intake.record.append@1.0.0",
        definition_fingerprint:
            "sha256:78eb6fe63cee39aad447ae92d7e4e6d75f6fd59dd773da69c5a5469f438c6b3a",
        request_schema_ref: "handbook.operation.intake-record-append-request@1.0.0",
        request_schema_fingerprint:
            "sha256:33e357221716d62986c923b4c04ea23c037911adaf2200cde9d6810214166dd7",
        result_schema_ref: "handbook.operation.intake-record-append-result@1.0.0",
        result_schema_fingerprint:
            "sha256:56b0788fe51010692f56d7ad63c2d4ebdd224f45fe93c9aafb9b8b2b9fcdb848",
        mutability: "append_only",
        idempotency: "idempotency_key_required",
        idempotency_fingerprint:
            "sha256:fb3eaacbcf74cee49007b3dda325f36e905ee49a4dc39f7c2440736f923412c1",
        write_set_fingerprint:
            "sha256:737ecb4eef7df47446ee0783d3c8e7102486aa3137ee95c4522a7919a6f33286",
    },
    expected(
        "artifact.candidate.validate@1.0.0",
        "sha256:9799e93e4f60484d6bd39bfc126dd8bb1d89536ea5e4231cf903baf1bda50053",
        "handbook.operation.artifact-candidate-validate-request@1.0.0",
        "sha256:9a85a9f044b34911d79465ad0e84f6d33d3009a844cb7d5b9a83782dc1ad61ce",
        "handbook.operation.artifact-candidate-validate-result@1.0.0",
        "sha256:8f76f8fad6f23126e47dcd76efb67ad43ddaaa8cec5d70fcb8ad6a4c85669596",
    ),
    ExpectedArtifactOperationClaim {
        operation_ref: "artifact.candidate.append@1.0.0",
        definition_fingerprint:
            "sha256:549e58a5728040940bc48e155d747a9c6f50c6f40a797b9bac88fe11e2da50b2",
        request_schema_ref: "handbook.operation.artifact-candidate-append-request@1.0.0",
        request_schema_fingerprint:
            "sha256:b4cbd2f95b0f114f33ccef8e475ee3120bd1d776458df1c6034065b601cf3a31",
        result_schema_ref: "handbook.operation.artifact-candidate-append-result@1.0.0",
        result_schema_fingerprint:
            "sha256:1b10d29a5e1e81cad8ad57bebb2df2d3c6bd100948754b607f87a10f0e1ffbb8",
        mutability: "append_only",
        idempotency: "idempotency_key_required",
        idempotency_fingerprint:
            "sha256:fb3eaacbcf74cee49007b3dda325f36e905ee49a4dc39f7c2440736f923412c1",
        write_set_fingerprint:
            "sha256:af33ccb3a2c5e59c56f6be36dbd2aed557302c5d59f745eeaa35023706317216",
    },
    ExpectedArtifactOperationClaim {
        operation_ref: "artifact.candidate.promote@1.0.0",
        definition_fingerprint:
            "sha256:12bdc7c3516ea6a12b60f8a6cb1653de3d1f54c19cef944e1724041c0f6bd3f9",
        request_schema_ref: "handbook.operation.artifact-candidate-promote-request@1.0.0",
        request_schema_fingerprint:
            "sha256:09261194628003ad2f20c60bf6b91bbf851022a8c2cad8d348ca6f086b2d7fc8",
        result_schema_ref: "handbook.operation.artifact-candidate-promote-result@1.0.0",
        result_schema_fingerprint:
            "sha256:109a3a9ea970fe067249dae5cf83a4a4edda78931ba5f99dddbf2ea916710f08",
        mutability: "compare_and_write",
        idempotency: "compare_and_write_required",
        idempotency_fingerprint:
            "sha256:41f911bd97752d2f124678e46c9a2a1e1d04cfb19ab97625666d0d5fd0d34265",
        write_set_fingerprint:
            "sha256:53a0a467900b356535cd0774dcdb67c270695af76e59d7608a4bbf662152f01e",
    },
];

const fn expected(
    operation_ref: &'static str,
    definition_fingerprint: &'static str,
    request_schema_ref: &'static str,
    request_schema_fingerprint: &'static str,
    result_schema_ref: &'static str,
    result_schema_fingerprint: &'static str,
) -> ExpectedArtifactOperationClaim {
    ExpectedArtifactOperationClaim {
        operation_ref,
        definition_fingerprint,
        request_schema_ref,
        request_schema_fingerprint,
        result_schema_ref,
        result_schema_fingerprint,
        mutability: "read_only",
        idempotency: "safe",
        idempotency_fingerprint: SAFE_IDEMPOTENCY,
        write_set_fingerprint: EMPTY_WRITE_SET,
    }
}
