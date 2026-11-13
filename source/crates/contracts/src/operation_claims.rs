//! Exact SDK operation claims admitted by `handbook-contracts`.
//!
//! This registry is deliberately outside the owner-source evidence preimage.

use crate::operation_admission::OperationAdmissionClaim;

pub(crate) fn matches(claim: &OperationAdmissionClaim<'_>) -> bool {
    let Some(expected) = expected(claim.operation_ref) else {
        return false;
    };
    claim.definition_fingerprint == expected.definition_fingerprint
        && claim.owner_crate == "handbook-contracts"
        && claim.owner_version == "0.1.0"
        && claim.owner_source_ref == "handbook.owner.contracts@0.1.0"
        && claim.owner_source_fingerprint
            == "sha256:eb0435914f7fd52b4335f5ca9b84782d01f9e38508a3cb89104e0a8c2d137b35"
        && claim.mutability == expected.mutability
        && claim.idempotency == expected.idempotency
        && claim.idempotency_fingerprint == expected.idempotency_fingerprint
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
        && claim.write_set_fingerprint == expected.write_set_fingerprint
        && claim.required_capabilities_fingerprint
            == "sha256:4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945"
}

struct ExpectedClaim {
    definition_fingerprint: &'static str,
    mutability: &'static str,
    idempotency: &'static str,
    idempotency_fingerprint: &'static str,
    request_schema_ref: &'static str,
    request_schema_fingerprint: &'static str,
    result_schema_ref: &'static str,
    result_schema_fingerprint: &'static str,
    write_set_fingerprint: &'static str,
}

fn expected(operation_ref: &str) -> Option<ExpectedClaim> {
    Some(match operation_ref {
        "contract.definition.list@1.0.0" => ExpectedClaim {
            definition_fingerprint:
                "sha256:c942ce40901e5288165ca9377652e66f6a513e209b8369a955dca9571cc2a809",
            mutability: "read_only",
            idempotency: "safe",
            idempotency_fingerprint:
                "sha256:e3a5961ff3a3a8141099426d699e73c9d6dd073456c0721a8cbd4a2f9bb9015d",
            request_schema_ref: "handbook.operation.contract-definition-list-request@1.0.0",
            request_schema_fingerprint:
                "sha256:c42f680c2140b43f3bbe6bfe0f909ed5283bd5f1a02acff668f4b1efd2a5865a",
            result_schema_ref: "handbook.operation.contract-definition-list-result@1.0.0",
            result_schema_fingerprint:
                "sha256:f58a5990eda7799e77e39f96a2b1f36a1db482808fe4ec268d482fa2e482169b",
            write_set_fingerprint:
                "sha256:4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945",
        },
        "contract.definition.read@1.0.0" => ExpectedClaim {
            definition_fingerprint:
                "sha256:73a8458f0ef3e9349af0f802e68040080c0b3cbe2a6c45ae29efc7a8ecb6ed18",
            mutability: "read_only",
            idempotency: "safe",
            idempotency_fingerprint:
                "sha256:e3a5961ff3a3a8141099426d699e73c9d6dd073456c0721a8cbd4a2f9bb9015d",
            request_schema_ref: "handbook.operation.contract-definition-read-request@1.0.0",
            request_schema_fingerprint:
                "sha256:4a9b0b0fcef4345156f25faa4d9d38d725308007181ee72ce5ca0d1785b786d4",
            result_schema_ref: "handbook.operation.contract-definition-read-result@1.0.0",
            result_schema_fingerprint:
                "sha256:afedeb9457876fc63af023179c7674221f0723764db930ada0732152d92cd699",
            write_set_fingerprint:
                "sha256:4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945",
        },
        "contract.definition.append@1.0.0" => ExpectedClaim {
            definition_fingerprint:
                "sha256:1bd097fd56c0ae53ef91cc5ecd887a2c88c80b750ab9332e7f9a0c0c00d1abf8",
            mutability: "append_only",
            idempotency: "idempotency_key_required",
            idempotency_fingerprint:
                "sha256:fb3eaacbcf74cee49007b3dda325f36e905ee49a4dc39f7c2440736f923412c1",
            request_schema_ref: "handbook.operation.contract-definition-append-request@1.0.0",
            request_schema_fingerprint:
                "sha256:7de54c07141deb650ebeba7784760c1c31df362f5aa876104059d391485ca531",
            result_schema_ref: "handbook.operation.contract-definition-append-result@1.0.0",
            result_schema_fingerprint:
                "sha256:1af35fb56f88e252dcb02a5a7094876c95c7972a5610a6d0097b182e6a0bc618",
            write_set_fingerprint:
                "sha256:5a4813b056b916942063cc07ab01a3ab7291b47d699d92f54e99df6f91023b57",
        },
        "contract.lifecycle.transition@1.0.0" => ExpectedClaim {
            definition_fingerprint:
                "sha256:fcc11925da6d1ad53707325d6b1c0c6a351f5aafaa83b84632c2c978564f039f",
            mutability: "compare_and_write",
            idempotency: "compare_and_write_required",
            idempotency_fingerprint:
                "sha256:ef4c7da56e6682f5ebef955a7db47907c2d63d3edb4a28320d93022b08c031ba",
            request_schema_ref: "handbook.operation.contract-lifecycle-transition-request@1.0.0",
            request_schema_fingerprint:
                "sha256:7d4872f431f24ea08dc349f1522e4252f24233d7dc75027dada54b5f6f1526fa",
            result_schema_ref: "handbook.operation.contract-lifecycle-transition-result@1.0.0",
            result_schema_fingerprint:
                "sha256:dd9b191e3a62fe937b34001743727b838193971b0269055b83fdd860bb393b8a",
            write_set_fingerprint:
                "sha256:9bddafc5b3bdfb7a82360acb299c8459d56fd98b552538e8b5eb28d1b0407f0a",
        },
        _ => return None,
    })
}
