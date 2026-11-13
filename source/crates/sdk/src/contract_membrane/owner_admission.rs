use super::{
    canonical_fingerprint, operation_inventory, AdmissionReason, ContractError, ContractLimits,
    OperationClass, OperationDefinition, OperationIdempotency, OperationOwner,
};

#[derive(Debug, Eq, PartialEq)]
pub struct OwnerAdmissionToken {
    owner: OwnerToken,
}

#[derive(Debug, Eq, PartialEq)]
enum OwnerToken {
    Contracts(handbook_contracts::operation_admission::OwnerOperationAdmission),
    ContractEvidence(
        handbook_contracts::evidence_operation_admission::EvidenceOwnerOperationAdmission,
    ),
    Dock(handbook_dock::operation_admission::OwnerOperationAdmission),
    Engine(handbook_engine::operation_admission::OwnerOperationAdmission),
    Snapshot(handbook_engine::snapshot_operation_admission::SnapshotOperationAdmission),
    Flow(handbook_flow::operation_admission::OwnerOperationAdmission),
    Pipeline(handbook_pipeline::operation_admission::OwnerOperationAdmission),
}

impl OwnerAdmissionToken {
    pub fn operation_ref(&self) -> &str {
        match &self.owner {
            OwnerToken::Contracts(token) => token.operation_ref(),
            OwnerToken::ContractEvidence(token) => token.operation_ref(),
            OwnerToken::Dock(token) => token.operation_ref(),
            OwnerToken::Engine(token) => token.operation_ref(),
            OwnerToken::Snapshot(token) => token.operation_ref(),
            OwnerToken::Flow(token) => token.operation_ref(),
            OwnerToken::Pipeline(token) => token.operation_ref(),
        }
    }

    pub fn definition_fingerprint(&self) -> &str {
        match &self.owner {
            OwnerToken::Contracts(token) => token.definition_fingerprint(),
            OwnerToken::ContractEvidence(token) => token.definition_fingerprint(),
            OwnerToken::Dock(token) => token.definition_fingerprint(),
            OwnerToken::Engine(token) => token.definition_fingerprint(),
            OwnerToken::Snapshot(token) => token.definition_fingerprint(),
            OwnerToken::Flow(token) => token.definition_fingerprint(),
            OwnerToken::Pipeline(token) => token.definition_fingerprint(),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum OperationAdmissionResult {
    Admitted(OwnerAdmissionToken),
    Omitted(AdmissionReason),
}

pub fn admit_operation_definition(
    definition: &OperationDefinition,
) -> Result<OperationAdmissionResult, ContractError> {
    let row = operation_inventory()
        .iter()
        .find(|row| row.operation_ref() == definition.operation())
        .ok_or(ContractError::InvalidInvariant)?;
    if row.class() != definition.class() {
        return Err(ContractError::InvalidInvariant);
    }
    match row.owner() {
        OperationOwner::Contracts => admit_contracts(definition),
        OperationOwner::Dock => admit_dock(definition),
        OperationOwner::SdkComposition => Ok(OperationAdmissionResult::Omitted(
            AdmissionReason::IndependentOwnerAbsent,
        )),
        OperationOwner::Phase5 => Ok(OperationAdmissionResult::Omitted(
            AdmissionReason::OwnerDeferred,
        )),
        OperationOwner::Engine => admit_engine(definition),
        OperationOwner::Flow => admit_flow(definition),
        OperationOwner::Pipeline => admit_pipeline(definition),
    }
}

fn admit_dock(definition: &OperationDefinition) -> Result<OperationAdmissionResult, ContractError> {
    require_owner(definition, "handbook-dock", "0.1.0")?;
    let parts = ClaimParts::new(definition)?;
    let claim = handbook_dock::operation_admission::OperationAdmissionClaim {
        operation_ref: parts.operation_ref,
        definition_fingerprint: parts.definition_fingerprint,
        owner_crate: parts.owner_crate,
        owner_version: parts.owner_version,
        owner_source_ref: parts.owner_source_ref,
        owner_source_fingerprint: parts.owner_source_fingerprint,
        mutability: parts.mutability,
        idempotency: parts.idempotency,
        idempotency_fingerprint: parts.idempotency_fingerprint.as_str(),
        definition_schema_ref: parts.definition_schema_ref,
        definition_schema_fingerprint: parts.definition_schema_fingerprint,
        request_schema_ref: parts.request_schema_ref,
        request_schema_fingerprint: parts.request_schema_fingerprint,
        result_schema_ref: parts.result_schema_ref,
        result_schema_fingerprint: parts.result_schema_fingerprint,
        blocker_schema_ref: parts.blocker_schema_ref,
        blocker_schema_fingerprint: parts.blocker_schema_fingerprint,
        refusal_schema_ref: parts.refusal_schema_ref,
        refusal_schema_fingerprint: parts.refusal_schema_fingerprint,
        error_schema_ref: parts.error_schema_ref,
        error_schema_fingerprint: parts.error_schema_fingerprint,
        write_set_fingerprint: parts.write_set_fingerprint.as_str(),
        required_capabilities_fingerprint: parts.required_capabilities_fingerprint.as_str(),
    };
    handbook_dock::operation_admission::admit_dock_operation(&claim)
        .map(|owner| {
            OperationAdmissionResult::Admitted(OwnerAdmissionToken {
                owner: OwnerToken::Dock(owner),
            })
        })
        .map_err(|_| ContractError::InvalidInvariant)
}

fn admit_contracts(
    definition: &OperationDefinition,
) -> Result<OperationAdmissionResult, ContractError> {
    require_owner(definition, "handbook-contracts", "0.1.0")?;
    let parts = ClaimParts::new(definition)?;
    let claim = handbook_contracts::operation_admission::OperationAdmissionClaim {
        operation_ref: parts.operation_ref,
        definition_fingerprint: parts.definition_fingerprint,
        owner_crate: parts.owner_crate,
        owner_version: parts.owner_version,
        owner_source_ref: parts.owner_source_ref,
        owner_source_fingerprint: parts.owner_source_fingerprint,
        mutability: parts.mutability,
        idempotency: parts.idempotency,
        idempotency_fingerprint: parts.idempotency_fingerprint.as_str(),
        definition_schema_ref: parts.definition_schema_ref,
        definition_schema_fingerprint: parts.definition_schema_fingerprint,
        request_schema_ref: parts.request_schema_ref,
        request_schema_fingerprint: parts.request_schema_fingerprint,
        result_schema_ref: parts.result_schema_ref,
        result_schema_fingerprint: parts.result_schema_fingerprint,
        blocker_schema_ref: parts.blocker_schema_ref,
        blocker_schema_fingerprint: parts.blocker_schema_fingerprint,
        refusal_schema_ref: parts.refusal_schema_ref,
        refusal_schema_fingerprint: parts.refusal_schema_fingerprint,
        error_schema_ref: parts.error_schema_ref,
        error_schema_fingerprint: parts.error_schema_fingerprint,
        write_set_fingerprint: parts.write_set_fingerprint.as_str(),
        required_capabilities_fingerprint: parts.required_capabilities_fingerprint.as_str(),
    };
    if super::CONTRACT_EVIDENCE_OPERATION_REFS.contains(&definition.operation().as_str()) {
        handbook_contracts::evidence_operation_admission::admit_contract_evidence_operation(&claim)
            .map(|owner| {
                OperationAdmissionResult::Admitted(OwnerAdmissionToken {
                    owner: OwnerToken::ContractEvidence(owner),
                })
            })
            .map_err(|_| ContractError::InvalidInvariant)
    } else {
        handbook_contracts::operation_admission::admit_contract_operation(&claim)
            .map(|owner| {
                OperationAdmissionResult::Admitted(OwnerAdmissionToken {
                    owner: OwnerToken::Contracts(owner),
                })
            })
            .map_err(|_| ContractError::InvalidInvariant)
    }
}

fn admit_engine(
    definition: &OperationDefinition,
) -> Result<OperationAdmissionResult, ContractError> {
    require_owner(definition, "handbook-engine", "0.2.0")?;
    let parts = ClaimParts::new(definition)?;
    let claim = handbook_engine::operation_admission::OperationAdmissionClaim {
        operation_ref: parts.operation_ref,
        definition_fingerprint: parts.definition_fingerprint,
        owner_crate: parts.owner_crate,
        owner_version: parts.owner_version,
        owner_source_ref: parts.owner_source_ref,
        owner_source_fingerprint: parts.owner_source_fingerprint,
        definition_schema_ref: parts.definition_schema_ref,
        definition_schema_fingerprint: parts.definition_schema_fingerprint,
        request_schema_ref: parts.request_schema_ref,
        request_schema_fingerprint: parts.request_schema_fingerprint,
        result_schema_ref: parts.result_schema_ref,
        result_schema_fingerprint: parts.result_schema_fingerprint,
        blocker_schema_ref: parts.blocker_schema_ref,
        blocker_schema_fingerprint: parts.blocker_schema_fingerprint,
        refusal_schema_ref: parts.refusal_schema_ref,
        refusal_schema_fingerprint: parts.refusal_schema_fingerprint,
        error_schema_ref: parts.error_schema_ref,
        error_schema_fingerprint: parts.error_schema_fingerprint,
        mutability: parts.mutability,
        idempotency: parts.idempotency,
        idempotency_fingerprint: parts.idempotency_fingerprint.as_str(),
        write_set_fingerprint: parts.write_set_fingerprint.as_str(),
        required_capabilities_fingerprint: parts.required_capabilities_fingerprint.as_str(),
    };
    let result = if super::snapshot_operation::SNAPSHOT_OPERATION_REFS
        .contains(&definition.operation().as_str())
    {
        return handbook_engine::snapshot_operation_admission::admit_snapshot_operation(&claim)
            .map(|owner| {
                OperationAdmissionResult::Admitted(OwnerAdmissionToken {
                    owner: OwnerToken::Snapshot(owner),
                })
            })
            .map_err(|_| ContractError::InvalidInvariant);
    } else {
        handbook_engine::operation_admission::admit_contract_operation(&claim)
    };
    match result {
        Ok(owner) => Ok(OperationAdmissionResult::Admitted(OwnerAdmissionToken {
            owner: OwnerToken::Engine(owner),
        })),
        Err(_) => Err(ContractError::InvalidInvariant),
    }
}

fn admit_flow(definition: &OperationDefinition) -> Result<OperationAdmissionResult, ContractError> {
    require_owner(definition, "handbook-flow", "0.1.1")?;
    let parts = ClaimParts::new(definition)?;
    let claim = handbook_flow::operation_admission::OperationAdmissionClaim {
        operation_ref: parts.operation_ref,
        definition_fingerprint: parts.definition_fingerprint,
        owner_crate: parts.owner_crate,
        owner_version: parts.owner_version,
        owner_source_ref: parts.owner_source_ref,
        owner_source_fingerprint: parts.owner_source_fingerprint,
        mutability: parts.mutability,
        idempotency: parts.idempotency,
        idempotency_fingerprint: parts.idempotency_fingerprint.as_str(),
        definition_schema_ref: parts.definition_schema_ref,
        definition_schema_fingerprint: parts.definition_schema_fingerprint,
        request_schema_ref: parts.request_schema_ref,
        request_schema_fingerprint: parts.request_schema_fingerprint,
        result_schema_ref: parts.result_schema_ref,
        result_schema_fingerprint: parts.result_schema_fingerprint,
        blocker_schema_ref: parts.blocker_schema_ref,
        blocker_schema_fingerprint: parts.blocker_schema_fingerprint,
        refusal_schema_ref: parts.refusal_schema_ref,
        refusal_schema_fingerprint: parts.refusal_schema_fingerprint,
        error_schema_ref: parts.error_schema_ref,
        error_schema_fingerprint: parts.error_schema_fingerprint,
        write_set_fingerprint: parts.write_set_fingerprint.as_str(),
        required_capabilities_fingerprint: parts.required_capabilities_fingerprint.as_str(),
    };
    match handbook_flow::operation_admission::admit_contract_operation(&claim) {
        Ok(owner) => Ok(OperationAdmissionResult::Admitted(OwnerAdmissionToken {
            owner: OwnerToken::Flow(owner),
        })),
        Err(_) => Err(ContractError::InvalidInvariant),
    }
}

fn admit_pipeline(
    definition: &OperationDefinition,
) -> Result<OperationAdmissionResult, ContractError> {
    require_owner(definition, "handbook-pipeline", "0.1.2")?;
    let parts = ClaimParts::new(definition)?;
    let claim = handbook_pipeline::operation_admission::OperationAdmissionClaim {
        operation_ref: parts.operation_ref,
        definition_fingerprint: parts.definition_fingerprint,
        owner_crate: parts.owner_crate,
        owner_version: parts.owner_version,
        owner_source_ref: parts.owner_source_ref,
        owner_source_fingerprint: parts.owner_source_fingerprint,
        mutability: parts.mutability,
        idempotency: parts.idempotency,
        idempotency_fingerprint: parts.idempotency_fingerprint.as_str(),
        definition_schema_ref: parts.definition_schema_ref,
        definition_schema_fingerprint: parts.definition_schema_fingerprint,
        request_schema_ref: parts.request_schema_ref,
        request_schema_fingerprint: parts.request_schema_fingerprint,
        result_schema_ref: parts.result_schema_ref,
        result_schema_fingerprint: parts.result_schema_fingerprint,
        blocker_schema_ref: parts.blocker_schema_ref,
        blocker_schema_fingerprint: parts.blocker_schema_fingerprint,
        refusal_schema_ref: parts.refusal_schema_ref,
        refusal_schema_fingerprint: parts.refusal_schema_fingerprint,
        error_schema_ref: parts.error_schema_ref,
        error_schema_fingerprint: parts.error_schema_fingerprint,
        write_set_fingerprint: parts.write_set_fingerprint.as_str(),
        required_capabilities_fingerprint: parts.required_capabilities_fingerprint.as_str(),
    };
    match handbook_pipeline::operation_admission::admit_contract_operation(&claim) {
        Ok(owner) => Ok(OperationAdmissionResult::Admitted(OwnerAdmissionToken {
            owner: OwnerToken::Pipeline(owner),
        })),
        Err(handbook_pipeline::operation_admission::OwnerAdmissionRefusal::SemanticMismatch) => Ok(
            OperationAdmissionResult::Omitted(AdmissionReason::SemanticMismatch),
        ),
        Err(_) => Err(ContractError::InvalidInvariant),
    }
}

struct ClaimParts<'a> {
    operation_ref: &'a str,
    definition_fingerprint: &'a str,
    owner_crate: &'a str,
    owner_version: &'a str,
    owner_source_ref: &'a str,
    owner_source_fingerprint: &'a str,
    definition_schema_ref: &'a str,
    definition_schema_fingerprint: &'a str,
    request_schema_ref: &'a str,
    request_schema_fingerprint: &'a str,
    result_schema_ref: &'a str,
    result_schema_fingerprint: &'a str,
    blocker_schema_ref: &'a str,
    blocker_schema_fingerprint: &'a str,
    refusal_schema_ref: &'a str,
    refusal_schema_fingerprint: &'a str,
    error_schema_ref: &'a str,
    error_schema_fingerprint: &'a str,
    mutability: &'static str,
    idempotency: &'static str,
    idempotency_fingerprint: super::Fingerprint,
    write_set_fingerprint: super::Fingerprint,
    required_capabilities_fingerprint: super::Fingerprint,
}

impl<'a> ClaimParts<'a> {
    fn new(definition: &'a OperationDefinition) -> Result<Self, ContractError> {
        let limits = ContractLimits::packet0();
        Ok(Self {
            operation_ref: definition.operation().as_str(),
            definition_fingerprint: definition.definition_fingerprint().as_str(),
            owner_crate: definition.owner().owner_crate(),
            owner_version: definition.owner().owner_version().as_str(),
            owner_source_ref: definition.owner().source().reference().as_str(),
            owner_source_fingerprint: definition.owner().source().fingerprint().as_str(),
            definition_schema_ref: definition.definition_schema().reference().as_str(),
            definition_schema_fingerprint: definition.definition_schema().fingerprint().as_str(),
            request_schema_ref: definition.request_schema().reference().as_str(),
            request_schema_fingerprint: definition.request_schema().fingerprint().as_str(),
            result_schema_ref: definition.result_schema().reference().as_str(),
            result_schema_fingerprint: definition.result_schema().fingerprint().as_str(),
            blocker_schema_ref: definition.outcome_schemas().blocker().reference().as_str(),
            blocker_schema_fingerprint: definition
                .outcome_schemas()
                .blocker()
                .fingerprint()
                .as_str(),
            refusal_schema_ref: definition.outcome_schemas().refusal().reference().as_str(),
            refusal_schema_fingerprint: definition
                .outcome_schemas()
                .refusal()
                .fingerprint()
                .as_str(),
            error_schema_ref: definition.outcome_schemas().error().reference().as_str(),
            error_schema_fingerprint: definition.outcome_schemas().error().fingerprint().as_str(),
            mutability: class_label(definition.class()),
            idempotency: idempotency_label(definition.idempotency()),
            idempotency_fingerprint: canonical_fingerprint(definition.idempotency(), &limits)?,
            write_set_fingerprint: canonical_fingerprint(&definition.write_set(), &limits)?,
            required_capabilities_fingerprint: canonical_fingerprint(
                &definition.required_capabilities(),
                &limits,
            )?,
        })
    }
}

fn require_owner(
    definition: &OperationDefinition,
    owner_crate: &str,
    owner_version: &str,
) -> Result<(), ContractError> {
    if definition.owner().owner_crate() == owner_crate
        && definition.owner().owner_version().as_str() == owner_version
    {
        Ok(())
    } else {
        Err(ContractError::InvalidInvariant)
    }
}

fn class_label(class: OperationClass) -> &'static str {
    match class {
        OperationClass::ReadOnly => "read_only",
        OperationClass::AppendOnly => "append_only",
        OperationClass::CompareAndWrite => "compare_and_write",
    }
}

fn idempotency_label(idempotency: &OperationIdempotency) -> &'static str {
    match idempotency {
        OperationIdempotency::Safe => "safe",
        OperationIdempotency::IdempotencyKeyRequired { .. } => "idempotency_key_required",
        OperationIdempotency::CompareAndWriteRequired { .. } => "compare_and_write_required",
    }
}
