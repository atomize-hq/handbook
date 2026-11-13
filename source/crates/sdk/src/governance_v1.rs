pub use handbook_contracts::project_admission_governance::{
    GovernanceProjectV1, InitialDeferralChoiceV1, InitialDeferralInstructionV1,
    InitialDeferralRequestV1, InitialEstablishmentRequestV1, DeferredRevisionV1,
    GovernanceEstablishmentInstructionV1, GovernanceEstablishmentRequestV1,
    InitialDeferralLocatorV1, InitialEstablishmentLocatorV1, GovernanceEstablishmentLocatorV1,
    ValidatedInitialDeferralV1, ValidatedInitialEstablishmentV1, ValidatedGovernanceEstablishmentV1,
    InitialDeferralOutcomeV1, InitialEstablishmentOutcomeV1, GovernanceEstablishmentOutcomeV1,
    InitialDeferralNoncommitV1, InitialEstablishmentNoncommitV1, GovernanceEstablishmentNoncommitV1,
    DeferredGovernanceV1, InitiallyEstablishedGovernanceV1, EstablishedDeferredGovernanceV1,
};
use handbook_contracts::project_admission_governance::GovernanceRefusalV1;
use serde::Serialize;
pub(crate) fn translate<T: Serialize, U: serde::de::DeserializeOwned>(value: &T) -> Result<U, GovernanceRefusalV1> {
    serde_json::to_value(value).and_then(serde_json::from_value)
        .map_err(|error| GovernanceRefusalV1::new("governance_projection", error.to_string()))
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::contract_membrane::{canonical_json_bytes, ContractLimits};
    use handbook_contracts::project_admission_governance as native;

    #[test]
    fn semantic_locator_fields_reconstruct_exact_native_owner_identity() {
        use crate::admission_governance::*;
        let dir = tempfile::tempdir().unwrap();
        let sdk = crate::HandbookSdkV1::open(dir.path());
        let request = InitialDeferralRequestV1::human_approved("semantic-locator", "Approve this exact deferral.").unwrap();
        let prepared = sdk.prepare_initial_project_governance(&InitialProjectGovernanceV1::Deferred(request)).unwrap();
        let ProjectGovernanceLocatorV1::InitializeDeferred(semantic) = prepared.locator().clone() else { panic!("wrong locator") };
        let original: native::DeferralRequestLocatorV2 = translate(&semantic).unwrap();
        let restored: InitialDeferralLocatorV1 = translate(&original).unwrap();
        assert_eq!(restored, semantic);
        assert_eq!(restored.canonical_json_bytes().unwrap(), original.canonical_json_bytes().unwrap());
        assert_eq!(canonical_json_bytes(&semantic, &ContractLimits::packet0()).unwrap(), canonical_json_bytes(&original, &ContractLimits::packet0()).unwrap());
        assert!(matches!(sdk.initialize_project_governance(prepared), ProjectGovernanceExecutionV1::Committed { .. }));
        let current = sdk.resolve_current_project_governance(&GovernanceProjectSelectionV1::InitiallyDeferred(semantic.project.clone()), native::ADMISSION_GOVERNANCE_SCOPE_V1, None).unwrap();
        let CurrentProjectGovernanceV1::Deferred { basis, original: current_original } = current else { panic!("expected deferred") };
        assert_eq!(current_original, semantic);
        let input = GovernanceEstablishmentRequestV1::human_approved(basis.project, DeferredRevisionV1 { outcome_fingerprint: basis.outcome_fingerprint, relation_fingerprint: basis.relation_fingerprint }, 7200, "semantic-establish", "Approve the exact terms.").unwrap();
        let prepared = sdk.prepare_project_governance_establishment(&input).unwrap();
        let ProjectGovernanceLocatorV1::EstablishDeferred(semantic_g) = prepared.locator().clone() else { panic!("wrong locator") };
        let original_g: native::DeferredEstablishmentLocatorV3 = translate(&semantic_g).unwrap();
        let restored_g: GovernanceEstablishmentLocatorV1 = translate(&original_g).unwrap();
        assert_eq!(restored_g, semantic_g);
        let native_again: native::DeferredEstablishmentLocatorV3 = translate(&restored_g).unwrap();
        assert_eq!(native_again.canonical_json_bytes().unwrap(), original_g.canonical_json_bytes().unwrap());
    }
}
