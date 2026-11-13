//! Independent Dock-owner admission for the three HCM-5.3 ordinary operations.

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
        owner_source_ref: crate::DOCK_OWNER_SOURCE_REF_V1,
        source_bytes: include_bytes!("operation_admission.rs"),
    }
}

pub fn admit_dock_operation(
    claim: &OperationAdmissionClaim<'_>,
) -> Result<OwnerOperationAdmission, OwnerAdmissionRefusal> {
    let owner = operation_owner_version_evidence();
    if claim.owner_crate != owner.owner_crate
        || claim.owner_version != owner.owner_version
        || claim.owner_source_ref != owner.owner_source_ref
    {
        return Err(OwnerAdmissionRefusal::OwnerBindingMismatch);
    }
    let Some((class, idempotency)) = expected_behavior(claim.operation_ref) else {
        return Err(OwnerAdmissionRefusal::UnknownOperation);
    };
    let schema_stem = claim
        .operation_ref
        .strip_suffix("@1.0.0")
        .unwrap_or("")
        .replace('.', "-");
    if claim.mutability != class
        || claim.idempotency != idempotency
        || !all_fingerprints_are_structural(claim)
        || claim.definition_schema_ref != "handbook.operation-definition@1.0.0"
        || claim.request_schema_ref != format!("handbook.operation.{schema_stem}-request@1.0.0")
        || claim.result_schema_ref != format!("handbook.operation.{schema_stem}-result@1.0.0")
        || claim.blocker_schema_ref != "handbook.operation-blocker@1.0.0"
        || claim.refusal_schema_ref != "handbook.operation-refusal@1.0.0"
        || claim.error_schema_ref != "handbook.operation-error@1.0.0"
        || !crate::operation_claims::matches(claim)
    {
        return Err(OwnerAdmissionRefusal::DefinitionMismatch);
    }
    Ok(OwnerOperationAdmission {
        operation_ref: claim.operation_ref.to_owned(),
        definition_fingerprint: claim.definition_fingerprint.to_owned(),
    })
}

fn expected_behavior(operation_ref: &str) -> Option<(&'static str, &'static str)> {
    match operation_ref {
        "dock.manifest.list@1.0.0" | "dock.manifest.read@1.0.0" => Some(("read_only", "safe")),
        "dock.run@1.0.0" => Some(("append_only", "idempotency_key_required")),
        _ => None,
    }
}

fn all_fingerprints_are_structural(claim: &OperationAdmissionClaim<'_>) -> bool {
    [
        claim.definition_fingerprint,
        claim.owner_source_fingerprint,
        claim.idempotency_fingerprint,
        claim.definition_schema_fingerprint,
        claim.request_schema_fingerprint,
        claim.result_schema_fingerprint,
        claim.blocker_schema_fingerprint,
        claim.refusal_schema_fingerprint,
        claim.error_schema_fingerprint,
        claim.write_set_fingerprint,
        claim.required_capabilities_fingerprint,
    ]
    .into_iter()
    .all(|value| {
        value.len() == 71
            && value.starts_with("sha256:")
            && value[7..]
                .bytes()
                .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
    })
}
