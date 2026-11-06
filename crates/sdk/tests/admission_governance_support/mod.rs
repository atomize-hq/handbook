//! Test-only bridge: all semantic operations call the unified SDK. Native owner
//! shapes are reconstructed solely for existing exact-identity assertions.
#![allow(dead_code)]
use handbook_contracts::project_admission_governance as native;
use handbook_sdk::admission_governance as api;
use handbook_sdk::HandbookSdkV1;
use serde::{de::DeserializeOwned, Serialize};

fn exact<T: Serialize, U: DeserializeOwned>(value: &T) -> U {
    serde_json::from_value(serde_json::to_value(value).unwrap()).unwrap()
}

#[derive(Debug)]
pub struct InitialEstablished {
    inner: api::PreparedInitialProjectGovernanceV1,
    locator: native::GovernanceRequestLocatorV1,
}
impl InitialEstablished {
    pub fn locator(&self) -> &native::GovernanceRequestLocatorV1 {
        &self.locator
    }
}
pub fn prepare_initial_established(
    sdk: &HandbookSdkV1,
    request: &native::GovernanceRequestInputV1,
) -> Result<InitialEstablished, native::GovernanceRefusalV1> {
    let inner = sdk.prepare_initial_project_governance(
        &api::InitialProjectGovernanceV1::Established(exact(request)),
    )?;
    let api::ProjectGovernanceLocatorV1::InitializeEstablished(locator) = inner.locator() else {
        panic!("initial established locator")
    };
    let locator = exact(locator);
    Ok(InitialEstablished { inner, locator })
}
pub fn initialize_established(
    sdk: &HandbookSdkV1,
    prepared: InitialEstablished,
) -> native::GovernanceExecutionV1 {
    native_initial_result(sdk.initialize_project_governance(prepared.inner))
}
pub fn resolve_initial_established(
    sdk: &HandbookSdkV1,
    locator: &native::GovernanceRequestLocatorV1,
) -> native::GovernanceExecutionV1 {
    native_initial_result(sdk.resolve_project_governance_request(
        &api::ProjectGovernanceLocatorV1::InitializeEstablished(exact(locator)),
    ))
}
fn native_initial_result(
    value: api::ProjectGovernanceExecutionV1,
) -> native::GovernanceExecutionV1 {
    match value {
        api::ProjectGovernanceExecutionV1::Committed {
            outcome: api::ProjectGovernanceCommittedV1::InitialEstablished(outcome),
            delivery,
        } => native::GovernanceExecutionV1::Committed {
            outcome: exact(&outcome),
            delivery,
        },
        api::ProjectGovernanceExecutionV1::NotCommitted {
            outcome: api::ProjectGovernanceNotCommittedV1::InitialEstablished(outcome),
            delivery,
        } => native::GovernanceExecutionV1::NotCommitted {
            outcome: exact(&outcome),
            delivery,
        },
        api::ProjectGovernanceExecutionV1::Refused { refusal } => {
            native::GovernanceExecutionV1::Refused { refusal }
        }
        api::ProjectGovernanceExecutionV1::Uncertain {
            locator: api::ProjectGovernanceLocatorV1::InitializeEstablished(locator),
            condition,
        } => native::GovernanceExecutionV1::Uncertain {
            locator: exact(&locator),
            condition,
        },
        other => panic!("unexpected initial established result: {other:?}"),
    }
}
pub fn current_initial_established(
    sdk: &HandbookSdkV1,
    project: &native::ProjectBindingV1,
    scope: &str,
    expected: Option<&native::ExpectedGovernanceBasisV1>,
) -> Result<native::ActiveGovernanceBasisV1, native::GovernanceRefusalV1> {
    let selection = api::GovernanceProjectSelectionV1::InitiallyEstablished(exact(project));
    let expected = expected.map(|e| api::ExpectedCurrentGovernanceV1::InitiallyEstablished {
        configuration_fingerprint: e.configuration_fingerprint.clone(),
        outcome_fingerprint: e.outcome_fingerprint.clone(),
    });
    match sdk.resolve_current_project_governance(&selection, scope, expected.as_ref())? {
        api::CurrentProjectGovernanceV1::Established {
            origin: api::EstablishedGovernanceOriginV1::Initial { basis, .. },
        } => Ok(exact(&basis)),
        other => panic!("unexpected current initial established: {other:?}"),
    }
}

#[derive(Debug)]
pub struct InitialDeferred {
    inner: api::PreparedInitialProjectGovernanceV1,
    locator: native::DeferralRequestLocatorV2,
}
impl InitialDeferred {
    pub fn locator(&self) -> &native::DeferralRequestLocatorV2 {
        &self.locator
    }
    pub fn prospective_deferred_basis(
        &self,
    ) -> Result<native::DeferredGovernanceBasisV2, native::GovernanceRefusalV1> {
        self.locator.prospective_deferred_basis()
    }
}
pub fn prepare_initial_deferred(
    sdk: &HandbookSdkV1,
    request: &native::DeferralRequestInputV2,
) -> Result<InitialDeferred, native::GovernanceRefusalV1> {
    let inner = sdk.prepare_initial_project_governance(
        &api::InitialProjectGovernanceV1::Deferred(exact(request)),
    )?;
    let api::ProjectGovernanceLocatorV1::InitializeDeferred(locator) = inner.locator() else {
        panic!("initial deferred locator")
    };
    let locator = exact(locator);
    Ok(InitialDeferred { inner, locator })
}
pub fn initialize_deferred(
    sdk: &HandbookSdkV1,
    prepared: InitialDeferred,
) -> native::DeferralExecutionV2 {
    native_deferral_result(sdk.initialize_project_governance(prepared.inner))
}
pub fn resolve_initial_deferred(
    sdk: &HandbookSdkV1,
    locator: &native::DeferralRequestLocatorV2,
) -> native::DeferralExecutionV2 {
    native_deferral_result(sdk.resolve_project_governance_request(
        &api::ProjectGovernanceLocatorV1::InitializeDeferred(exact(locator)),
    ))
}
fn native_deferral_result(value: api::ProjectGovernanceExecutionV1) -> native::DeferralExecutionV2 {
    match value {
        api::ProjectGovernanceExecutionV1::Committed {
            outcome: api::ProjectGovernanceCommittedV1::InitialDeferred(outcome),
            delivery,
        } => native::DeferralExecutionV2::Committed {
            outcome: exact(&outcome),
            delivery,
        },
        api::ProjectGovernanceExecutionV1::NotCommitted {
            outcome: api::ProjectGovernanceNotCommittedV1::InitialDeferred(outcome),
            delivery,
        } => native::DeferralExecutionV2::NotCommitted {
            outcome: exact(&outcome),
            delivery,
        },
        api::ProjectGovernanceExecutionV1::Refused { refusal } => {
            native::DeferralExecutionV2::Refused { refusal }
        }
        api::ProjectGovernanceExecutionV1::Uncertain {
            locator: api::ProjectGovernanceLocatorV1::InitializeDeferred(locator),
            condition,
        } => native::DeferralExecutionV2::Uncertain {
            locator: exact(&locator),
            condition,
        },
        other => panic!("unexpected initial deferred result: {other:?}"),
    }
}

#[derive(Debug)]
pub struct EstablishedDeferred {
    inner: api::PreparedProjectGovernanceEstablishmentV1,
    locator: native::DeferredEstablishmentLocatorV3,
}
impl EstablishedDeferred {
    pub fn locator(&self) -> &native::DeferredEstablishmentLocatorV3 {
        &self.locator
    }
}
pub fn prepare_deferred_establishment(
    sdk: &HandbookSdkV1,
    request: &native::DeferredEstablishmentRequestV3,
) -> Result<EstablishedDeferred, native::GovernanceRefusalV1> {
    let inner = sdk.prepare_project_governance_establishment(&exact(request))?;
    let api::ProjectGovernanceLocatorV1::EstablishDeferred(locator) = inner.locator() else {
        panic!("establishment locator")
    };
    let locator = exact(locator);
    Ok(EstablishedDeferred { inner, locator })
}
pub fn establish_deferred(
    sdk: &HandbookSdkV1,
    prepared: EstablishedDeferred,
) -> native::DeferredEstablishmentExecutionV3 {
    native_establishment_result(sdk.establish_project_governance(prepared.inner))
}
pub fn resolve_deferred_establishment(
    sdk: &HandbookSdkV1,
    locator: &native::DeferredEstablishmentLocatorV3,
) -> native::DeferredEstablishmentExecutionV3 {
    native_establishment_result(sdk.resolve_project_governance_request(
        &api::ProjectGovernanceLocatorV1::EstablishDeferred(exact(locator)),
    ))
}
fn native_establishment_result(
    value: api::ProjectGovernanceExecutionV1,
) -> native::DeferredEstablishmentExecutionV3 {
    match value {
        api::ProjectGovernanceExecutionV1::Committed {
            outcome: api::ProjectGovernanceCommittedV1::EstablishedFromDeferred(outcome),
            delivery,
        } => native::DeferredEstablishmentExecutionV3::Committed {
            outcome: exact(&outcome),
            delivery,
        },
        api::ProjectGovernanceExecutionV1::NotCommitted {
            outcome: api::ProjectGovernanceNotCommittedV1::EstablishedFromDeferred(outcome),
            delivery,
        } => native::DeferredEstablishmentExecutionV3::NotCommitted {
            outcome: exact(&outcome),
            delivery,
        },
        api::ProjectGovernanceExecutionV1::Refused { refusal } => {
            native::DeferredEstablishmentExecutionV3::Refused { refusal }
        }
        api::ProjectGovernanceExecutionV1::Uncertain {
            locator: api::ProjectGovernanceLocatorV1::EstablishDeferred(locator),
            condition,
        } => native::DeferredEstablishmentExecutionV3::Uncertain {
            locator: exact(&locator),
            condition,
        },
        other => panic!("unexpected deferred establishment result: {other:?}"),
    }
}

pub fn current_scope(
    sdk: &HandbookSdkV1,
    project: &native::ProjectAdmissionBindingV3,
    scope: &str,
    expected: Option<&native::ExpectedProjectAdmissionBasisV3>,
) -> Result<native::ProjectAdmissionScopeBasisV3, native::GovernanceRefusalV1> {
    let project = match project {
        native::ProjectAdmissionBindingV3::EstablishedV1(value) => {
            api::GovernanceProjectSelectionV1::InitiallyEstablished(exact(value))
        }
        native::ProjectAdmissionBindingV3::ScopedChoiceV2(value) => {
            api::GovernanceProjectSelectionV1::InitiallyDeferred(exact(value))
        }
    };
    let expected = expected.map(|value| match value {
        native::ExpectedProjectAdmissionBasisV3::EstablishedV1(v) => {
            api::ExpectedCurrentGovernanceV1::InitiallyEstablished {
                configuration_fingerprint: v.configuration_fingerprint.clone(),
                outcome_fingerprint: v.outcome_fingerprint.clone(),
            }
        }
        native::ExpectedProjectAdmissionBasisV3::DeferredV2(v) => {
            api::ExpectedCurrentGovernanceV1::Deferred(exact(v))
        }
        native::ExpectedProjectAdmissionBasisV3::EstablishedFromDeferredV3(v) => {
            api::ExpectedCurrentGovernanceV1::EstablishedFromDeferred {
                configuration_fingerprint: v.configuration_fingerprint.clone(),
                outcome_fingerprint: v.outcome_fingerprint.clone(),
                relation_fingerprint: v.relation_fingerprint.clone(),
            }
        }
    });
    Ok(
        match sdk.resolve_current_project_governance(&project, scope, expected.as_ref())? {
            api::CurrentProjectGovernanceV1::Deferred { basis, .. } => {
                native::ProjectAdmissionScopeBasisV3::DeferredV2(exact(&basis))
            }
            api::CurrentProjectGovernanceV1::Established {
                origin: api::EstablishedGovernanceOriginV1::Initial { basis, .. },
            } => native::ProjectAdmissionScopeBasisV3::EstablishedV1(exact(&basis)),
            api::CurrentProjectGovernanceV1::Established {
                origin: api::EstablishedGovernanceOriginV1::FromDeferred { basis, .. },
            } => native::ProjectAdmissionScopeBasisV3::EstablishedFromDeferredV3(exact(&basis)),
        },
    )
}

// The earlier owner-facing scope assertion shape is reconstructed only for
// exact comparison in existing tests; current truth still comes from the SDK.
pub fn current_scoped_assertion(
    sdk: &HandbookSdkV1,
    project: &native::ProjectAdmissionBindingV2,
    scope: &str,
    expected: Option<&native::ExpectedProjectAdmissionBasisV2>,
) -> Result<native::ProjectAdmissionScopeBasisV2, native::GovernanceRefusalV1> {
    let project = match project {
        native::ProjectAdmissionBindingV2::EstablishedV1(v) => {
            native::ProjectAdmissionBindingV3::EstablishedV1(v.clone())
        }
        native::ProjectAdmissionBindingV2::ScopedChoiceV2(v) => {
            native::ProjectAdmissionBindingV3::ScopedChoiceV2(v.clone())
        }
    };
    let expected = expected.map(|value| match value {
        native::ExpectedProjectAdmissionBasisV2::EstablishedV1(v) => {
            native::ExpectedProjectAdmissionBasisV3::EstablishedV1(v.clone())
        }
        native::ExpectedProjectAdmissionBasisV2::ScopedChoiceV2(v) => {
            native::ExpectedProjectAdmissionBasisV3::DeferredV2(v.clone())
        }
    });
    Ok(
        match current_scope(sdk, &project, scope, expected.as_ref())? {
            native::ProjectAdmissionScopeBasisV3::DeferredV2(v) => {
                native::ProjectAdmissionScopeBasisV2::Deferred(v)
            }
            native::ProjectAdmissionScopeBasisV3::EstablishedV1(v) => {
                native::ProjectAdmissionScopeBasisV2::Established(v)
            }
            other => {
                panic!("this earlier assertion has no established-from-deferred case: {other:?}")
            }
        },
    )
}

#[cfg(feature = "governance-test-support")]
pub fn initialize_deferred_with_fault(sdk: &HandbookSdkV1, prepared: InitialDeferred, fault: api::GovernanceTestFaultV1) -> native::DeferralExecutionV2 {
    native_deferral_result(sdk.initialize_project_governance_with_fault_for_testing(prepared.inner, fault))
}
#[cfg(feature = "governance-test-support")]
pub fn establish_deferred_with_fault(sdk: &HandbookSdkV1, prepared: EstablishedDeferred, fault: api::GovernanceTestFaultV1) -> native::DeferredEstablishmentExecutionV3 {
    native_establishment_result(sdk.establish_project_governance_with_fault_for_testing(prepared.inner, fault))
}
