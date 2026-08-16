//! Flow-owner admission for membrane operation-definition claims.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OperationAdmissionClaim<'a> {
    pub operation_ref: &'a str,
    pub definition_fingerprint: &'a str,
    pub owner_crate: &'a str,
    pub owner_version: &'a str,
    pub owner_source_ref: &'a str,
    pub owner_source_fingerprint: &'a str,
    pub mutability: &'a str,
    pub idempotency: &'a str,
    pub idempotency_fingerprint: &'a str,
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OwnerVersionEvidence {
    pub owner_crate: &'static str,
    pub owner_version: &'static str,
    pub owner_source_ref: &'static str,
    pub source_bytes: &'static [u8],
}

pub fn operation_owner_version_evidence() -> OwnerVersionEvidence {
    OwnerVersionEvidence {
        owner_crate: env!("CARGO_PKG_NAME"),
        owner_version: env!("CARGO_PKG_VERSION"),
        owner_source_ref: "handbook.owner.flow@0.1.1",
        source_bytes: include_bytes!("operation_admission.rs"),
    }
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

const EXPECTED: [(&str, &str, &str); 4] = [
    ("projection.definition.read@1.0.0", "read_only", "safe"),
    ("projection.create@1.0.0", "read_only", "safe"),
    ("snapshot.project@1.0.0", "read_only", "safe"),
    ("flow.resolve@1.0.0", "read_only", "safe"),
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
        && claim.owner_source_ref == "handbook.owner.flow@0.1.1"
        && claim.request_schema_ref == format!("handbook.operation.{schema_stem}-request@1.0.0")
        && claim.result_schema_ref == format!("handbook.operation.{schema_stem}-result@1.0.0")
        && claim.blocker_schema_ref == "handbook.operation-blocker@1.0.0"
        && claim.refusal_schema_ref == "handbook.operation-refusal@1.0.0"
        && claim.error_schema_ref == "handbook.operation-error@1.0.0"
}
