//! Independent Engine admission for the four HCM-4.6 snapshot operations.

use crate::operation_admission::{OperationAdmissionClaim, OwnerAdmissionRefusal};

const EMPTY_FINGERPRINT: &str =
    "sha256:4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945";
const SAFE_FINGERPRINT: &str =
    "sha256:e3a5961ff3a3a8141099426d699e73c9d6dd073456c0721a8cbd4a2f9bb9015d";
const OWNER_SOURCE_FINGERPRINT: &str =
    "sha256:99df01064d1f6a21e7db74a5746936e7af9dee784029a2850439b7ff64696e40";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SnapshotOperationAdmission {
    operation_ref: String,
    definition_fingerprint: String,
}

impl SnapshotOperationAdmission {
    pub fn operation_ref(&self) -> &str {
        &self.operation_ref
    }
    pub fn definition_fingerprint(&self) -> &str {
        &self.definition_fingerprint
    }
}

pub fn admit_snapshot_operation(
    claim: &OperationAdmissionClaim<'_>,
) -> Result<SnapshotOperationAdmission, OwnerAdmissionRefusal> {
    let expected = EXPECTED
        .iter()
        .find(|expected| expected.operation_ref == claim.operation_ref)
        .ok_or(OwnerAdmissionRefusal::UnknownOperation)?;
    if claim.owner_crate != "handbook-engine"
        || claim.owner_version != "0.2.0"
        || claim.owner_source_ref != "handbook.owner.engine@0.2.0"
        || claim.owner_source_fingerprint != OWNER_SOURCE_FINGERPRINT
    {
        return Err(OwnerAdmissionRefusal::OwnerBindingMismatch);
    }
    let request_ref = format!("handbook.operation.{}-request@1.0.0", expected.stem);
    let result_ref = format!("handbook.operation.{}-result@1.0.0", expected.stem);
    let exact = claim.definition_fingerprint == expected.definition_fingerprint
        && claim.definition_schema_ref == "handbook.operation-definition@1.0.0"
        && claim.definition_schema_fingerprint
            == "sha256:e668b32cb7ef3725e4d8c53696ec9032ba0741e387436883a7d8f264602ddd09"
        && claim.request_schema_ref == request_ref
        && claim.request_schema_fingerprint == expected.request_schema_fingerprint
        && claim.result_schema_ref == result_ref
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
        && claim.required_capabilities_fingerprint == EMPTY_FINGERPRINT;
    if !exact {
        return Err(OwnerAdmissionRefusal::DefinitionMismatch);
    }
    Ok(SnapshotOperationAdmission {
        operation_ref: claim.operation_ref.to_owned(),
        definition_fingerprint: claim.definition_fingerprint.to_owned(),
    })
}

struct ExpectedClaim {
    operation_ref: &'static str,
    stem: &'static str,
    definition_fingerprint: &'static str,
    request_schema_fingerprint: &'static str,
    result_schema_fingerprint: &'static str,
    mutability: &'static str,
    idempotency: &'static str,
    idempotency_fingerprint: &'static str,
    write_set_fingerprint: &'static str,
}

const EXPECTED: [ExpectedClaim; 4] = [
    ExpectedClaim {
        operation_ref: "snapshot.capture@1.0.0",
        stem: "snapshot-capture",
        definition_fingerprint:
            "sha256:277c0741450c0de7e8046d4da17ba46bfc4e04fde019bbd63831e75c4fdafc07",
        request_schema_fingerprint:
            "sha256:c0ce4b387e6f1e84c865318ff090b852440b1cd230a99a575c4f761e7ac5cb97",
        result_schema_fingerprint:
            "sha256:3e49eb6a78beb54dc65e93c9a70e2249a75d40af344174d3fc01a222d4f3b313",
        mutability: "append_only",
        idempotency: "idempotency_key_required",
        idempotency_fingerprint:
            "sha256:fb3eaacbcf74cee49007b3dda325f36e905ee49a4dc39f7c2440736f923412c1",
        write_set_fingerprint:
            "sha256:8746a1de3899426ce0613b6ff846df9514c2595309eb2596f79e16290ac7c2a3",
    },
    ExpectedClaim {
        operation_ref: "snapshot.delta@1.0.0",
        stem: "snapshot-delta",
        definition_fingerprint:
            "sha256:fe730641d0a12cac14b2b9ceabd180d1e795c6df08f685079365c80fc0e6d9ed",
        request_schema_fingerprint:
            "sha256:6b6c76f38a89e6dd9a8c1187eae5182d9aa0acb4080b6383a474196e4760cf8c",
        result_schema_fingerprint:
            "sha256:740d311dfef55cc0ce2cc6074e1d7e2e99041a6d39753364cec9b0fb79fde112",
        mutability: "read_only",
        idempotency: "safe",
        idempotency_fingerprint: SAFE_FINGERPRINT,
        write_set_fingerprint: EMPTY_FINGERPRINT,
    },
    ExpectedClaim {
        operation_ref: "snapshot.project@1.0.0",
        stem: "snapshot-project",
        definition_fingerprint:
            "sha256:fc3b2262bee158b3734e5916c4d53f6da392d53c4af3a898eb1960629831f66b",
        request_schema_fingerprint:
            "sha256:f58f997c2b059e83141ff0cce31df9c89db7a72992af93dafdb4410471e8f930",
        result_schema_fingerprint:
            "sha256:8bcf2a70a0d134ddaf7493c94115277913d32234c714e3708e35f2c9d2e82d39",
        mutability: "read_only",
        idempotency: "safe",
        idempotency_fingerprint: SAFE_FINGERPRINT,
        write_set_fingerprint: EMPTY_FINGERPRINT,
    },
    ExpectedClaim {
        operation_ref: "snapshot.read@1.0.0",
        stem: "snapshot-read",
        definition_fingerprint:
            "sha256:0f6ecd40c09432eb8b3c174370e7103aa5cd161df022c4a5c0f8bf047ff413a8",
        request_schema_fingerprint:
            "sha256:ac94138a278942d7f50f7a347abdb40a25e3714e051038053f9a7bf3f215e3db",
        result_schema_fingerprint:
            "sha256:c3d22b119b2e05870069fdba724a51c24eb0dbafe6f85c613f673193add6f7de",
        mutability: "read_only",
        idempotency: "safe",
        idempotency_fingerprint: SAFE_FINGERPRINT,
        write_set_fingerprint: EMPTY_FINGERPRINT,
    },
];
