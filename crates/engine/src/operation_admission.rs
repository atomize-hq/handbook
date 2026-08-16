//! Owner-native admission for membrane operation-definition claims.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OperationAdmissionClaim<'a> {
    pub operation_ref: &'a str,
    pub definition_fingerprint: &'a str,
    pub owner_crate: &'a str,
    pub owner_version: &'a str,
    pub owner_source_ref: &'a str,
    pub owner_source_fingerprint: &'a str,
    pub definition_schema_ref: &'a str,
    pub definition_schema_fingerprint: &'a str,
    pub request_schema_ref: &'a str,
    pub request_schema_fingerprint: &'a str,
    pub result_schema_ref: &'a str,
    pub result_schema_fingerprint: &'a str,
    pub blocker_schema_ref: &'a str,
    pub blocker_schema_fingerprint: &'a str,
    pub refusal_schema_ref: &'a str,
    pub refusal_schema_fingerprint: &'a str,
    pub error_schema_ref: &'a str,
    pub error_schema_fingerprint: &'a str,
    pub mutability: &'a str,
    pub idempotency: &'a str,
    pub idempotency_fingerprint: &'a str,
    pub write_set_fingerprint: &'a str,
    pub required_capabilities_fingerprint: &'a str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OwnerOperationAdmission {
    operation_ref: String,
    definition_fingerprint: String,
}

impl OwnerOperationAdmission {
    pub fn operation_ref(&self) -> &str {
        &self.operation_ref
    }

    pub fn definition_fingerprint(&self) -> &str {
        &self.definition_fingerprint
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OwnerAdmissionRefusal {
    UnknownOperation,
    OwnerBindingMismatch,
    DefinitionMismatch,
}

pub fn admit_contract_operation(
    claim: &OperationAdmissionClaim<'_>,
) -> Result<OwnerOperationAdmission, OwnerAdmissionRefusal> {
    if claim.owner_crate != env!("CARGO_PKG_NAME")
        || claim.owner_version != env!("CARGO_PKG_VERSION")
    {
        return Err(OwnerAdmissionRefusal::OwnerBindingMismatch);
    }
    let expected = EXPECTED
        .iter()
        .find(|(operation, _, _)| *operation == claim.operation_ref)
        .ok_or(OwnerAdmissionRefusal::UnknownOperation)?;
    if expected.1 != claim.mutability || expected.2 != claim.idempotency || !known_shape(claim) {
        return Err(OwnerAdmissionRefusal::DefinitionMismatch);
    }

    let Some(expected_claim) = ADMITTED
        .iter()
        .find(|candidate| candidate.operation_ref == claim.operation_ref)
    else {
        // No engine-owned row has an owner-selected exact definition yet.
        return Err(OwnerAdmissionRefusal::DefinitionMismatch);
    };
    if !exact_claim_matches(claim, expected_claim) {
        return Err(OwnerAdmissionRefusal::DefinitionMismatch);
    }
    Ok(OwnerOperationAdmission {
        operation_ref: claim.operation_ref.to_owned(),
        definition_fingerprint: claim.definition_fingerprint.to_owned(),
    })
}

const ADMITTED: [OperationAdmissionClaim<'static>; 0] = [];

fn exact_claim_matches(
    claim: &OperationAdmissionClaim<'_>,
    expected: &OperationAdmissionClaim<'_>,
) -> bool {
    claim == expected
}

const EXPECTED: [(&str, &str, &str); 31] = [
    ("profile.list@1.0.0", "read_only", "safe"),
    ("profile.resolve@1.0.0", "read_only", "safe"),
    ("schema.list@1.0.0", "read_only", "safe"),
    ("schema.read@1.0.0", "read_only", "safe"),
    ("vocabulary.read@1.0.0", "read_only", "safe"),
    ("resolution.stack.read@1.0.0", "read_only", "safe"),
    ("artifact.kind.list@1.0.0", "read_only", "safe"),
    ("artifact.instance.list@1.0.0", "read_only", "safe"),
    ("artifact.read@1.0.0", "read_only", "safe"),
    ("artifact.validate@1.0.0", "read_only", "safe"),
    ("artifact.render@1.0.0", "read_only", "safe"),
    ("intake.definition.read@1.0.0", "read_only", "safe"),
    ("intake.coverage.evaluate@1.0.0", "read_only", "safe"),
    (
        "intake.record.append@1.0.0",
        "append_only",
        "idempotency_key_required",
    ),
    ("record.list@1.0.0", "read_only", "safe"),
    ("record.read@1.0.0", "read_only", "safe"),
    ("artifact.candidate.validate@1.0.0", "read_only", "safe"),
    (
        "artifact.candidate.append@1.0.0",
        "append_only",
        "idempotency_key_required",
    ),
    (
        "artifact.approval.append@1.0.0",
        "append_only",
        "idempotency_key_required",
    ),
    (
        "artifact.candidate.promote@1.0.0",
        "compare_and_write",
        "compare_and_write_required",
    ),
    ("posture.resolve@1.0.0", "read_only", "safe"),
    ("posture.recommendation.evaluate@1.0.0", "read_only", "safe"),
    (
        "posture.recommendation.append@1.0.0",
        "append_only",
        "idempotency_key_required",
    ),
    (
        "posture.recommendation.acknowledge@1.0.0",
        "append_only",
        "idempotency_key_required",
    ),
    (
        "posture.transition.apply@1.0.0",
        "compare_and_write",
        "compare_and_write_required",
    ),
    (
        "resolution.escalation.request.append@1.0.0",
        "append_only",
        "idempotency_key_required",
    ),
    (
        "resolution.escalation.disposition.append@1.0.0",
        "append_only",
        "idempotency_key_required",
    ),
    (
        "memory.promotion.request.append@1.0.0",
        "append_only",
        "idempotency_key_required",
    ),
    (
        "memory.promotion.disposition.append@1.0.0",
        "compare_and_write",
        "compare_and_write_required",
    ),
    ("snapshot.read@1.0.0", "read_only", "safe"),
    ("snapshot.delta@1.0.0", "read_only", "safe"),
];

fn known_shape(claim: &OperationAdmissionClaim<'_>) -> bool {
    let operation_id = claim.operation_ref.strip_suffix("@1.0.0").unwrap_or("");
    let schema_stem = operation_id.replace('.', "-");
    [
        claim.definition_fingerprint,
        claim.definition_schema_fingerprint,
        claim.request_schema_fingerprint,
        claim.result_schema_fingerprint,
        claim.blocker_schema_fingerprint,
        claim.refusal_schema_fingerprint,
        claim.error_schema_fingerprint,
        claim.owner_source_fingerprint,
        claim.idempotency_fingerprint,
        claim.write_set_fingerprint,
        claim.required_capabilities_fingerprint,
    ]
    .iter()
    .all(|value| !value.is_empty())
        && claim.definition_schema_ref == "handbook.operation-definition@1.0.0"
        && claim.owner_source_ref == "handbook.owner.engine@0.2.0"
        && claim.request_schema_ref == format!("handbook.operation.{schema_stem}-request@1.0.0")
        && claim.result_schema_ref == format!("handbook.operation.{schema_stem}-result@1.0.0")
        && claim.blocker_schema_ref == "handbook.operation-blocker@1.0.0"
        && claim.refusal_schema_ref == "handbook.operation-refusal@1.0.0"
        && claim.error_schema_ref == "handbook.operation-error@1.0.0"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_owner_claim_comparison_rejects_every_single_field_substitution() {
        let expected = OperationAdmissionClaim {
            operation_ref: "artifact.validate@1.0.0",
            definition_fingerprint: "definition-fingerprint",
            owner_crate: "handbook-engine",
            owner_version: "0.2.0",
            owner_source_ref: "handbook.owner.engine@0.2.0",
            owner_source_fingerprint: "owner-source-fingerprint",
            definition_schema_ref: "handbook.operation-definition@1.0.0",
            definition_schema_fingerprint: "definition-schema-fingerprint",
            request_schema_ref: "handbook.operation.artifact-validate-request@1.0.0",
            request_schema_fingerprint: "request-schema-fingerprint",
            result_schema_ref: "handbook.operation.artifact-validate-result@1.0.0",
            result_schema_fingerprint: "result-schema-fingerprint",
            blocker_schema_ref: "handbook.operation-blocker@1.0.0",
            blocker_schema_fingerprint: "blocker-schema-fingerprint",
            refusal_schema_ref: "handbook.operation-refusal@1.0.0",
            refusal_schema_fingerprint: "refusal-schema-fingerprint",
            error_schema_ref: "handbook.operation-error@1.0.0",
            error_schema_fingerprint: "error-schema-fingerprint",
            mutability: "read_only",
            idempotency: "safe",
            idempotency_fingerprint: "idempotency-fingerprint",
            write_set_fingerprint: "write-set-fingerprint",
            required_capabilities_fingerprint: "capabilities-fingerprint",
        };
        assert!(exact_claim_matches(&expected, &expected));

        macro_rules! assert_substitution_refused {
            ($field:ident, $replacement:literal) => {{
                let mut substituted = expected;
                substituted.$field = $replacement;
                assert!(!exact_claim_matches(&substituted, &expected));
            }};
        }

        assert_substitution_refused!(operation_ref, "different");
        assert_substitution_refused!(definition_fingerprint, "different");
        assert_substitution_refused!(owner_crate, "different");
        assert_substitution_refused!(owner_version, "different");
        assert_substitution_refused!(owner_source_ref, "different");
        assert_substitution_refused!(owner_source_fingerprint, "different");
        assert_substitution_refused!(definition_schema_ref, "different");
        assert_substitution_refused!(definition_schema_fingerprint, "different");
        assert_substitution_refused!(request_schema_ref, "different");
        assert_substitution_refused!(request_schema_fingerprint, "different");
        assert_substitution_refused!(result_schema_ref, "different");
        assert_substitution_refused!(result_schema_fingerprint, "different");
        assert_substitution_refused!(blocker_schema_ref, "different");
        assert_substitution_refused!(blocker_schema_fingerprint, "different");
        assert_substitution_refused!(refusal_schema_ref, "different");
        assert_substitution_refused!(refusal_schema_fingerprint, "different");
        assert_substitution_refused!(error_schema_ref, "different");
        assert_substitution_refused!(error_schema_fingerprint, "different");
        assert_substitution_refused!(mutability, "different");
        assert_substitution_refused!(idempotency, "different");
        assert_substitution_refused!(idempotency_fingerprint, "different");
        assert_substitution_refused!(write_set_fingerprint, "different");
        assert_substitution_refused!(required_capabilities_fingerprint, "different");
    }
}
