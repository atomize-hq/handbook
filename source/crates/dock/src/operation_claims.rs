//! Exact SDK operation claims admitted by `handbook-dock`.
//!
//! This registry is deliberately outside the owner-source evidence preimage.

use crate::operation_admission::OperationAdmissionClaim;

pub(crate) fn matches(claim: &OperationAdmissionClaim<'_>) -> bool {
    let Some(expected) = expected(claim.operation_ref) else {
        return false;
    };
    claim.definition_fingerprint == expected.definition_fingerprint
        && claim.owner_crate == "handbook-dock"
        && claim.owner_version == "0.1.0"
        && claim.owner_source_ref == "handbook.owner.dock@0.1.0"
        && claim.owner_source_fingerprint
            == "sha256:7eb77f6c73e5fb80e2ba1e7bc16411b94517910b51932294d6c128b42a33a38e"
        && claim.idempotency_fingerprint == expected.idempotency_fingerprint
        && claim.definition_schema_fingerprint
            == "sha256:e668b32cb7ef3725e4d8c53696ec9032ba0741e387436883a7d8f264602ddd09"
        && claim.request_schema_fingerprint == expected.request_schema_fingerprint
        && claim.result_schema_fingerprint == expected.result_schema_fingerprint
        && claim.blocker_schema_fingerprint
            == "sha256:1125afb3a783af6ed4ee5b8caad86e00264420103d18db36e0337e34b889a009"
        && claim.refusal_schema_fingerprint
            == "sha256:16357baa6276ae2d6b27e22296e789db78d646403845f98653148178afec8b23"
        && claim.error_schema_fingerprint
            == "sha256:569cce41b965e3140591ed8c75ebad39aef2809c57e82b5c8f50db639d634355"
        && claim.write_set_fingerprint == expected.write_set_fingerprint
        && claim.required_capabilities_fingerprint
            == "sha256:4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945"
}

struct ExpectedClaim {
    definition_fingerprint: &'static str,
    idempotency_fingerprint: &'static str,
    request_schema_fingerprint: &'static str,
    result_schema_fingerprint: &'static str,
    write_set_fingerprint: &'static str,
}

fn expected(operation_ref: &str) -> Option<ExpectedClaim> {
    Some(match operation_ref {
        "dock.manifest.list@1.0.0" => ExpectedClaim {
            definition_fingerprint:
                "sha256:c64b2974ccdcea9004504ff79b5b5aba6dd351cea236abfc93be7dd4f1a1f60e",
            idempotency_fingerprint:
                "sha256:e3a5961ff3a3a8141099426d699e73c9d6dd073456c0721a8cbd4a2f9bb9015d",
            request_schema_fingerprint:
                "sha256:1f3983b882dac4fc18e685167defdf27e8ab6eacfc2da85ee95b82bb0895887e",
            result_schema_fingerprint:
                "sha256:0164fcffa45a6da7f68186f92da838d8c5739c19b783724575c3f1d4f662f970",
            write_set_fingerprint:
                "sha256:4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945",
        },
        "dock.manifest.read@1.0.0" => ExpectedClaim {
            definition_fingerprint:
                "sha256:9f98e8f72fe92af145e89d50ddb307a3210806c545dad99c55f17a2af4ac0942",
            idempotency_fingerprint:
                "sha256:e3a5961ff3a3a8141099426d699e73c9d6dd073456c0721a8cbd4a2f9bb9015d",
            request_schema_fingerprint:
                "sha256:c73429cec27a6a449cd9feead9897e44852e0e1f7fd30ab99b3914144b221ecf",
            result_schema_fingerprint:
                "sha256:f8123da286bfc54cf568d321d57f124ed376ac70c2455a812e3b4643b326f354",
            write_set_fingerprint:
                "sha256:4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945",
        },
        "dock.run@1.0.0" => ExpectedClaim {
            definition_fingerprint:
                "sha256:515032d7abaeacff00ae865594d09bc7a4610117161ccec04f6595a00392ad73",
            idempotency_fingerprint:
                "sha256:fb3eaacbcf74cee49007b3dda325f36e905ee49a4dc39f7c2440736f923412c1",
            request_schema_fingerprint:
                "sha256:7ec94ad755c776499f0ec28a010a72b9290a764cae49e303ebd5fa06a8470cbc",
            result_schema_fingerprint:
                "sha256:713eb630405a40a80a6c93e5b7b472d28a29bc8f155ccab36919d744677df02e",
            write_set_fingerprint:
                "sha256:cfb2f1fecb8402a1f44b41c89e4da8aa53a321aff86a90cee6b1f15707d7d5af",
        },
        _ => return None,
    })
}
