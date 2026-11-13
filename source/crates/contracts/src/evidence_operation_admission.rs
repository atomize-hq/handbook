//! Independent owner admission for the five HCM-5.2 evidence operations.
//!
//! This registry is additive so the HCM-5.1 owner-source evidence and four
//! predecessor operation definitions remain byte-stable.

use crate::operation_admission::{operation_owner_version_evidence, OperationAdmissionClaim};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceOwnerOperationAdmission {
    operation_ref: String,
    definition_fingerprint: String,
}

impl EvidenceOwnerOperationAdmission {
    pub fn operation_ref(&self) -> &str {
        &self.operation_ref
    }

    pub fn definition_fingerprint(&self) -> &str {
        &self.definition_fingerprint
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceOwnerAdmissionRefusal {
    UnknownOperation,
    OwnerBindingMismatch,
    DefinitionMismatch,
}

pub fn admit_contract_evidence_operation(
    claim: &OperationAdmissionClaim<'_>,
) -> Result<EvidenceOwnerOperationAdmission, EvidenceOwnerAdmissionRefusal> {
    let owner = operation_owner_version_evidence();
    if claim.owner_crate != owner.owner_crate
        || claim.owner_version != owner.owner_version
        || claim.owner_source_ref != owner.owner_source_ref
    {
        return Err(EvidenceOwnerAdmissionRefusal::OwnerBindingMismatch);
    }
    let Some(expected) = expected(claim.operation_ref) else {
        return Err(EvidenceOwnerAdmissionRefusal::UnknownOperation);
    };
    if claim.owner_source_fingerprint
        != "sha256:eb0435914f7fd52b4335f5ca9b84782d01f9e38508a3cb89104e0a8c2d137b35"
        || claim.definition_fingerprint != expected.definition_fingerprint
        || claim.mutability != expected.mutability
        || claim.idempotency != expected.idempotency
        || claim.idempotency_fingerprint != expected.idempotency_fingerprint
        || claim.definition_schema_ref != "handbook.operation-definition@1.0.0"
        || claim.definition_schema_fingerprint
            != "sha256:e668b32cb7ef3725e4d8c53696ec9032ba0741e387436883a7d8f264602ddd09"
        || claim.request_schema_ref != expected.request_schema_ref
        || claim.request_schema_fingerprint != expected.request_schema_fingerprint
        || claim.result_schema_ref != expected.result_schema_ref
        || claim.result_schema_fingerprint != expected.result_schema_fingerprint
        || claim.blocker_schema_ref != "handbook.operation-blocker@1.0.0"
        || claim.blocker_schema_fingerprint
            != "sha256:1125afb3a783af6ed4ee5b8caad86e00264420103d18db36e0337e34b889a009"
        || claim.refusal_schema_ref != "handbook.operation-refusal@1.0.0"
        || claim.refusal_schema_fingerprint
            != "sha256:16357baa6276ae2d6b27e22296e789db78d646403845f98653148178afec8b23"
        || claim.error_schema_ref != "handbook.operation-error@1.0.0"
        || claim.error_schema_fingerprint
            != "sha256:569cce41b965e3140591ed8c75ebad39aef2809c57e82b5c8f50db639d634355"
        || claim.write_set_fingerprint != expected.write_set_fingerprint
        || claim.required_capabilities_fingerprint
            != "sha256:4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945"
    {
        return Err(EvidenceOwnerAdmissionRefusal::DefinitionMismatch);
    }
    Ok(EvidenceOwnerOperationAdmission {
        operation_ref: claim.operation_ref.to_owned(),
        definition_fingerprint: claim.definition_fingerprint.to_owned(),
    })
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
        "contract.evidence.list@1.0.0" => ExpectedClaim {
            definition_fingerprint:
                "sha256:12150f909ed0798bf45b092aca6c852528b2bfbd35ce8b831c30a7f82f3c6549",
            mutability: "read_only",
            idempotency: "safe",
            idempotency_fingerprint:
                "sha256:e3a5961ff3a3a8141099426d699e73c9d6dd073456c0721a8cbd4a2f9bb9015d",
            request_schema_ref: "handbook.operation.contract-evidence-list-request@1.0.0",
            request_schema_fingerprint:
                "sha256:c4d3e687e0a7ed925032a42cd63261eff708d00c43be1c0de70e2aa0bd06d92b",
            result_schema_ref: "handbook.operation.contract-evidence-list-result@1.0.0",
            result_schema_fingerprint:
                "sha256:da759dff5e9054ca6ae7eeaa0875fc772073777c05ad206ae441d8e9eceac2c9",
            write_set_fingerprint:
                "sha256:4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945",
        },
        "contract.evidence.read@1.0.0" => ExpectedClaim {
            definition_fingerprint:
                "sha256:77ee25feb3bf058ecc068bb4394ca7fc58ca55a61260cc67798f681ae6934d97",
            mutability: "read_only",
            idempotency: "safe",
            idempotency_fingerprint:
                "sha256:e3a5961ff3a3a8141099426d699e73c9d6dd073456c0721a8cbd4a2f9bb9015d",
            request_schema_ref: "handbook.operation.contract-evidence-read-request@1.0.0",
            request_schema_fingerprint:
                "sha256:e057db5a0ad68aab9dcb73620cb9429ee8b750d961e28ef47f50af3230d52b28",
            result_schema_ref: "handbook.operation.contract-evidence-read-result@1.0.0",
            result_schema_fingerprint:
                "sha256:bbd9b3864b71f5a4ce6fb9de773442e5f43dfd17a6a0ff47e80cd41d2be3af4d",
            write_set_fingerprint:
                "sha256:4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945",
        },
        "contract.evidence.append@1.0.0" => ExpectedClaim {
            definition_fingerprint:
                "sha256:a61d7d09c1ee6a510129c71acfbf4d2b67055bcf058116d8b610e54630a70c06",
            mutability: "append_only",
            idempotency: "idempotency_key_required",
            idempotency_fingerprint:
                "sha256:fb3eaacbcf74cee49007b3dda325f36e905ee49a4dc39f7c2440736f923412c1",
            request_schema_ref: "handbook.operation.contract-evidence-append-request@1.0.0",
            request_schema_fingerprint:
                "sha256:32b1f334ba42cbef72c0637c52ecf0eb414280c4fc438168d82c7178a6c7c61a",
            result_schema_ref: "handbook.operation.contract-evidence-append-result@1.0.0",
            result_schema_fingerprint:
                "sha256:01a869404a349b1613b47e73e27e474a52c8e6b4f3facc7e20badc958e7c9437",
            write_set_fingerprint:
                "sha256:d3de359f7043fbf4d95a2e5f2cb3a51d173761babb4274d65fd5e8d4eccb959d",
        },
        "contract.verdict.evaluate@1.0.0" => ExpectedClaim {
            definition_fingerprint:
                "sha256:48f72f9c699503412fa9617c2efd973552ed27373b5a86fddf39eafccc9e0b56",
            mutability: "read_only",
            idempotency: "safe",
            idempotency_fingerprint:
                "sha256:e3a5961ff3a3a8141099426d699e73c9d6dd073456c0721a8cbd4a2f9bb9015d",
            request_schema_ref: "handbook.operation.contract-verdict-evaluate-request@1.0.0",
            request_schema_fingerprint:
                "sha256:1fc5f0936158ea5edd8de362160f945ab008c20a9f3c939af94b847aabb085a9",
            result_schema_ref: "handbook.operation.contract-verdict-evaluate-result@1.0.0",
            result_schema_fingerprint:
                "sha256:02664c17d08b1548a4fdbf81e2c9fdfd8d1eba24bdce2a4f8a6347f2436b1477",
            write_set_fingerprint:
                "sha256:4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945",
        },
        "contract.gate.evaluate@1.0.0" => ExpectedClaim {
            definition_fingerprint:
                "sha256:842ef94553118a9eee97f243e106f5d8f737d24e05986b919644f55ac06b0d57",
            mutability: "read_only",
            idempotency: "safe",
            idempotency_fingerprint:
                "sha256:e3a5961ff3a3a8141099426d699e73c9d6dd073456c0721a8cbd4a2f9bb9015d",
            request_schema_ref: "handbook.operation.contract-gate-evaluate-request@1.0.0",
            request_schema_fingerprint:
                "sha256:bbca5693b0f008f04b9f5c59a7aab1e81d94fb39dd11a6dbf39cb2e056c8a750",
            result_schema_ref: "handbook.operation.contract-gate-evaluate-result@1.0.0",
            result_schema_fingerprint:
                "sha256:37a5f940c87f9b59027c9d8c7065803496e96645118a8bf3bdb25024b73b273f",
            write_set_fingerprint:
                "sha256:4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945",
        },
        _ => return None,
    })
}
