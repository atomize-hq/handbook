use super::*;
use super::semantic_types::*;

/// Explicit initial governance selection. Preparation validates the entire
/// native request and does not mutate the repository.
#[derive(Clone, Debug)]
pub enum InitialProjectGovernanceV1 {
    Deferred(InitialDeferralRequestV1),
    Established(InitialEstablishmentRequestV1),
}

/// A complete original request identity. Restoring this value authorizes only
/// resolution; it never reconstructs a creation or establishment capability.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(
    tag = "operation",
    content = "original",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum ProjectGovernanceLocatorV1 {
    InitializeDeferred(InitialDeferralLocatorV1),
    InitializeEstablished(InitialEstablishmentLocatorV1),
    EstablishDeferred(GovernanceEstablishmentLocatorV1),
}

/// Opaque, single-use initial capability; only the locator can be serialized.
#[derive(Debug)]
pub struct PreparedInitialProjectGovernanceV1 {
    locator: ProjectGovernanceLocatorV1,
    prepared: InitialPrepared,
}
#[derive(Debug)]
enum InitialPrepared {
    Deferred(PreparedAdmissionDeferralV2),
    Established(PreparedAdmissionGovernanceV1),
}
impl PreparedInitialProjectGovernanceV1 {
    pub fn locator(&self) -> &ProjectGovernanceLocatorV1 {
        &self.locator
    }
}

/// Opaque, single-use capability for establishing an already deferred project.
#[derive(Debug)]
pub struct PreparedProjectGovernanceEstablishmentV1 {
    locator: ProjectGovernanceLocatorV1,
    prepared: PreparedDeferredGovernanceV3,
}
impl PreparedProjectGovernanceEstablishmentV1 {
    pub fn locator(&self) -> &ProjectGovernanceLocatorV1 {
        &self.locator
    }
}

/// Original owner facts, preserving the precise versioned identity preimages.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ProjectGovernanceCommittedV1 {
    InitialDeferred(InitialDeferralOutcomeV1),
    InitialEstablished(InitialEstablishmentOutcomeV1),
    EstablishedFromDeferred(GovernanceEstablishmentOutcomeV1),
}
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ProjectGovernanceNotCommittedV1 {
    InitialDeferred(InitialDeferralNoncommitV1),
    InitialEstablished(InitialEstablishmentNoncommitV1),
    EstablishedFromDeferred(GovernanceEstablishmentNoncommitV1),
}
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ProjectGovernanceExecutionV1 {
    Committed {
        outcome: ProjectGovernanceCommittedV1,
        delivery: GovernanceDeliveryV1,
    },
    Refused {
        refusal: GovernanceRefusalV1,
    },
    NotCommitted {
        outcome: ProjectGovernanceNotCommittedV1,
        delivery: GovernanceDeliveryV1,
    },
    Uncertain {
        locator: ProjectGovernanceLocatorV1,
        condition: String,
    },
}

/// Selects the exact native registration associated with a project.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum GovernanceProjectSelectionV1 {
    InitiallyEstablished(GovernanceProjectV1),
    InitiallyDeferred(GovernanceProjectV1),
}
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ExpectedCurrentGovernanceV1 {
    InitiallyEstablished {
        configuration_fingerprint: FingerprintV1,
        outcome_fingerprint: FingerprintV1,
    },
    Deferred(DeferredRevisionV1),
    EstablishedFromDeferred {
        configuration_fingerprint: FingerprintV1,
        outcome_fingerprint: FingerprintV1,
        relation_fingerprint: FingerprintV1,
    },
}

/// Owner-verified current selection, with the exact initial request that created
/// this project. Established-from-deferred also retains its current locator and
/// predecessor inside the native basis. Native basis bytes remain authoritative.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum CurrentProjectGovernanceV1 {
    Deferred {
        basis: DeferredGovernanceV1,
        original: InitialDeferralLocatorV1,
    },
    Established {
        origin: EstablishedGovernanceOriginV1,
    },
}
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum EstablishedGovernanceOriginV1 {
    Initial {
        basis: InitiallyEstablishedGovernanceV1,
        original: InitialEstablishmentLocatorV1,
    },
    FromDeferred {
        basis: EstablishedDeferredGovernanceV1,
        original: InitialDeferralLocatorV1,
    },
}

pub struct ProjectGovernanceV1;

impl ProjectGovernanceV1 {
    pub fn new() -> Self { Self }
    pub fn prepare_initial_project_governance(
        &self,
        root: &std::path::Path,
        choice: &InitialProjectGovernanceV1,
    ) -> Result<PreparedInitialProjectGovernanceV1, GovernanceRefusalV1> {
        match choice {
            InitialProjectGovernanceV1::Deferred(request) => {
                let prepared = super::ProjectAdmissionDeferralV2::new().prepare(root, &super::semantic_types::translate(request)?)?;
                Ok(PreparedInitialProjectGovernanceV1 {
                    locator: ProjectGovernanceLocatorV1::InitializeDeferred(
                        super::semantic_types::translate(prepared.locator())?,
                    ),
                    prepared: InitialPrepared::Deferred(prepared),
                })
            }
            InitialProjectGovernanceV1::Established(request) => {
                let prepared = ProjectAdmissionGovernanceV1::new()
                    .prepare(root, &super::semantic_types::translate(request)?)?;
                Ok(PreparedInitialProjectGovernanceV1 {
                    locator: ProjectGovernanceLocatorV1::InitializeEstablished(
                        super::semantic_types::translate(prepared.locator())?,
                    ),
                    prepared: InitialPrepared::Established(prepared),
                })
            }
        }
    }

    pub fn initialize_project_governance(
        &self,
        root: &std::path::Path,
        prepared: PreparedInitialProjectGovernanceV1,
    ) -> ProjectGovernanceExecutionV1 {
        let fallback = prepared.locator;
        match prepared.prepared {
            InitialPrepared::Deferred(value) => convert_deferral(super::ProjectAdmissionDeferralV2::new().initialize(root, value), fallback),
            InitialPrepared::Established(value) => convert_initial(super::ProjectAdmissionGovernanceV1::new().initialize(root, value), fallback),
        }
    }

    pub fn prepare_project_governance_establishment(
        &self,
        root: &std::path::Path,
        request: &GovernanceEstablishmentRequestV1,
    ) -> Result<PreparedProjectGovernanceEstablishmentV1, GovernanceRefusalV1> {
        let prepared =
            super::ProjectAdmissionEstablishmentV3::new(
            )
            .prepare(root, &super::semantic_types::translate(request)?)?;
        Ok(PreparedProjectGovernanceEstablishmentV1 {
            locator: ProjectGovernanceLocatorV1::EstablishDeferred(
                super::semantic_types::translate(prepared.locator())?,
            ),
            prepared,
        })
    }

    pub fn establish_project_governance(
        &self,
        root: &std::path::Path,
        prepared: PreparedProjectGovernanceEstablishmentV1,
    ) -> ProjectGovernanceExecutionV1 {
        convert_establishment(
            super::ProjectAdmissionEstablishmentV3::new(
            )
            .establish(root, prepared.prepared),
            prepared.locator,
        )
    }

    pub fn resolve_project_governance_request(
        &self,
        root: &std::path::Path,
        locator: &ProjectGovernanceLocatorV1,
    ) -> ProjectGovernanceExecutionV1 {
        if let Err(refusal) = validate_native_locator(locator) {
            return ProjectGovernanceExecutionV1::Refused { refusal };
        }
        match locator {
            ProjectGovernanceLocatorV1::InitializeDeferred(value) => {
                let native = match super::semantic_types::translate(value) {
                    Ok(v) => v,
                    Err(refusal) => return ProjectGovernanceExecutionV1::Refused { refusal },
                };
                convert_deferral(super::ProjectAdmissionDeferralV2::new().resolve_request(root, &native), locator.clone())
            }
            ProjectGovernanceLocatorV1::InitializeEstablished(value) => {
                let native = match super::semantic_types::translate(value) {
                    Ok(v) => v,
                    Err(refusal) => return ProjectGovernanceExecutionV1::Refused { refusal },
                };
                convert_initial(
                    ProjectAdmissionGovernanceV1::new().resolve_request(root, &native),
                    locator.clone(),
                )
            }
            ProjectGovernanceLocatorV1::EstablishDeferred(value) => {
                let native = match super::semantic_types::translate(value) {
                    Ok(v) => v,
                    Err(refusal) => return ProjectGovernanceExecutionV1::Refused { refusal },
                };
                convert_establishment(super::ProjectAdmissionEstablishmentV3::new().resolve_request(root, &native), locator.clone())
            }
        }
    }

    pub fn resolve_current_project_governance(
        &self,
        root: &std::path::Path,
        project: &GovernanceProjectSelectionV1,
        scope: &str,
        expected: Option<&ExpectedCurrentGovernanceV1>,
    ) -> Result<CurrentProjectGovernanceV1, GovernanceRefusalV1> {
        match (project, expected) {
            (GovernanceProjectSelectionV1::InitiallyEstablished(project), None) => {
                let native = super::semantic_types::translate(project)?;
                let (basis, original) = ProjectAdmissionGovernanceV1::new()
                    .resolve_active_with_original(root, &native, scope, None)?;
                Ok(CurrentProjectGovernanceV1::Established {
                    origin: EstablishedGovernanceOriginV1::Initial {
                        basis: super::semantic_types::translate(&basis)?,
                        original: super::semantic_types::translate(&original)?,
                    },
                })
            }
            (
                GovernanceProjectSelectionV1::InitiallyEstablished(project),
                Some(ExpectedCurrentGovernanceV1::InitiallyEstablished {
                    configuration_fingerprint,
                    outcome_fingerprint,
                }),
            ) => {
                let native = super::semantic_types::translate(project)?;
                let expected = ExpectedGovernanceBasisV1 {
                    configuration_fingerprint: configuration_fingerprint.clone(),
                    outcome_fingerprint: outcome_fingerprint.clone(),
                };
                let (basis, original) = ProjectAdmissionGovernanceV1::new()
                    .resolve_active_with_original(
                        root,
                        &native,
                        scope,
                        Some(&expected),
                    )?;
                Ok(CurrentProjectGovernanceV1::Established {
                    origin: EstablishedGovernanceOriginV1::Initial {
                        basis: super::semantic_types::translate(&basis)?,
                        original: super::semantic_types::translate(&original)?,
                    },
                })
            }
            (GovernanceProjectSelectionV1::InitiallyDeferred(project), expected) => {
                let native = project.into();
                let expected = match expected {
                    None => None,
                    Some(ExpectedCurrentGovernanceV1::Deferred(value)) => {
                        Some(ExpectedProjectAdmissionBasisV3::DeferredV2(
                            super::semantic_types::translate(value)?,
                        ))
                    }
                    Some(ExpectedCurrentGovernanceV1::EstablishedFromDeferred {
                        configuration_fingerprint,
                        outcome_fingerprint,
                        relation_fingerprint,
                    }) => Some(ExpectedProjectAdmissionBasisV3::EstablishedFromDeferredV3(
                        ExpectedEstablishedFromDeferredBasisV3 {
                            configuration_fingerprint: configuration_fingerprint.clone(),
                            outcome_fingerprint: outcome_fingerprint.clone(),
                            relation_fingerprint: relation_fingerprint.clone(),
                        },
                    )),
                    _ => {
                        return Err(GovernanceRefusalV1::new(
                            "basis_contract_conflict",
                            "Expected basis must name the exact project contract",
                        ))
                    }
                };
                let (basis, original) = super::ProjectAdmissionEstablishmentV3::new().resolve_current_with_original(root, &native, scope, expected.as_ref())?;
                Ok(match basis {
                    DeferredScopeBasisV3::DeferredV2(basis) => {
                        CurrentProjectGovernanceV1::Deferred {
                            basis: super::semantic_types::translate(&basis)?,
                            original: super::semantic_types::translate(&original)?,
                        }
                    }
                    DeferredScopeBasisV3::EstablishedFromDeferredV3(basis) => {
                        CurrentProjectGovernanceV1::Established {
                            origin: EstablishedGovernanceOriginV1::FromDeferred {
                                basis: super::semantic_types::translate(&basis)?,
                                original: super::semantic_types::translate(&original)?,
                            },
                        }
                    }
                })
            }
            _ => Err(GovernanceRefusalV1::new(
                "basis_contract_conflict",
                "Expected basis must name the exact project contract",
            )),
        }
    }
}

fn projection_uncertain(
    locator: ProjectGovernanceLocatorV1,
    error: GovernanceRefusalV1,
) -> ProjectGovernanceExecutionV1 {
    ProjectGovernanceExecutionV1::Uncertain {
        locator,
        condition: format!("governance_projection: {}", error.detail),
    }
}
fn convert_deferral(
    value: DeferralExecutionV2,
    fallback: ProjectGovernanceLocatorV1,
) -> ProjectGovernanceExecutionV1 {
    match value {
        DeferralExecutionV2::Committed { outcome, delivery } => {
            match super::semantic_types::translate(&outcome) {
                Ok(outcome) => ProjectGovernanceExecutionV1::Committed {
                    outcome: ProjectGovernanceCommittedV1::InitialDeferred(outcome),
                    delivery,
                },
                Err(error) => projection_uncertain(fallback, error),
            }
        }
        DeferralExecutionV2::NotCommitted { outcome, delivery } => {
            match super::semantic_types::translate(&outcome) {
                Ok(outcome) => ProjectGovernanceExecutionV1::NotCommitted {
                    outcome: ProjectGovernanceNotCommittedV1::InitialDeferred(outcome),
                    delivery,
                },
                Err(error) => projection_uncertain(fallback, error),
            }
        }
        DeferralExecutionV2::Refused { refusal } => {
            ProjectGovernanceExecutionV1::Refused { refusal }
        }
        DeferralExecutionV2::Uncertain { condition, .. } => {
            ProjectGovernanceExecutionV1::Uncertain {
                locator: fallback,
                condition,
            }
        }
    }
}
fn convert_initial(
    value: GovernanceExecutionV1,
    fallback: ProjectGovernanceLocatorV1,
) -> ProjectGovernanceExecutionV1 {
    match value {
        GovernanceExecutionV1::Committed { outcome, delivery } => {
            match super::semantic_types::translate(&outcome) {
                Ok(outcome) => ProjectGovernanceExecutionV1::Committed {
                    outcome: ProjectGovernanceCommittedV1::InitialEstablished(outcome),
                    delivery,
                },
                Err(error) => projection_uncertain(fallback, error),
            }
        }
        GovernanceExecutionV1::NotCommitted { outcome, delivery } => {
            match super::semantic_types::translate(&outcome) {
                Ok(outcome) => ProjectGovernanceExecutionV1::NotCommitted {
                    outcome: ProjectGovernanceNotCommittedV1::InitialEstablished(outcome),
                    delivery,
                },
                Err(error) => projection_uncertain(fallback, error),
            }
        }
        GovernanceExecutionV1::Refused { refusal } => {
            ProjectGovernanceExecutionV1::Refused { refusal }
        }
        GovernanceExecutionV1::Uncertain { condition, .. } => {
            ProjectGovernanceExecutionV1::Uncertain {
                locator: fallback,
                condition,
            }
        }
    }
}
fn convert_establishment(
    value: DeferredEstablishmentExecutionV3,
    fallback: ProjectGovernanceLocatorV1,
) -> ProjectGovernanceExecutionV1 {
    match value {
        DeferredEstablishmentExecutionV3::Committed { outcome, delivery } => {
            match super::semantic_types::translate(&outcome) {
                Ok(outcome) => ProjectGovernanceExecutionV1::Committed {
                    outcome: ProjectGovernanceCommittedV1::EstablishedFromDeferred(outcome),
                    delivery,
                },
                Err(error) => projection_uncertain(fallback, error),
            }
        }
        DeferredEstablishmentExecutionV3::NotCommitted { outcome, delivery } => {
            match super::semantic_types::translate(&outcome) {
                Ok(outcome) => ProjectGovernanceExecutionV1::NotCommitted {
                    outcome: ProjectGovernanceNotCommittedV1::EstablishedFromDeferred(outcome),
                    delivery,
                },
                Err(error) => projection_uncertain(fallback, error),
            }
        }
        DeferredEstablishmentExecutionV3::Refused { refusal } => {
            ProjectGovernanceExecutionV1::Refused { refusal }
        }
        DeferredEstablishmentExecutionV3::Uncertain { condition, .. } => {
            ProjectGovernanceExecutionV1::Uncertain {
                locator: fallback,
                condition,
            }
        }
    }
}


fn validate_native_locator(locator: &ProjectGovernanceLocatorV1) -> Result<(), GovernanceRefusalV1> {
    match locator {
        ProjectGovernanceLocatorV1::InitializeDeferred(value) => {
            let native: DeferralRequestLocatorV2 = super::semantic_types::translate(value)?;
            native.canonical_json_bytes()?;
        }
        ProjectGovernanceLocatorV1::InitializeEstablished(value) => {
            let native: GovernanceRequestLocatorV1 = super::semantic_types::translate(value)?;
            native.canonical_json_bytes()?;
        }
        ProjectGovernanceLocatorV1::EstablishDeferred(value) => {
            let native: DeferredEstablishmentLocatorV3 = super::semantic_types::translate(value)?;
            native.canonical_json_bytes()?;
        }
    }
    Ok(())
}

/// Fault injection through the semantic owner, for integration tests only.
#[cfg(feature = "test-support")]
#[derive(Clone, Debug)]
pub enum GovernanceTestFaultV1 {
    Initial(GovernanceFaultBoundaryV1),
    Deferred(DeferralFaultBoundaryV2),
    Establishment(EstablishmentFaultBoundaryV3),
    PauseInitial(GovernanceFaultBoundaryV1, std::path::PathBuf, std::path::PathBuf),
    PauseDeferred(DeferralFaultBoundaryV2, std::path::PathBuf, std::path::PathBuf),
    PauseEstablishment(EstablishmentFaultBoundaryV3, std::path::PathBuf, std::path::PathBuf),
}
#[cfg(feature = "test-support")]
impl ProjectGovernanceV1 {
    pub fn initialize_with_fault_for_testing(&self, root: &std::path::Path, prepared: PreparedInitialProjectGovernanceV1, fault: GovernanceTestFaultV1) -> ProjectGovernanceExecutionV1 {
        let fallback = prepared.locator;
        match (prepared.prepared, fault) {
            (InitialPrepared::Deferred(value), GovernanceTestFaultV1::Deferred(boundary)) => convert_deferral(ProjectAdmissionDeferralV2::new().with_fault_for_testing(boundary).initialize(root, value), fallback),
            (InitialPrepared::Deferred(value), GovernanceTestFaultV1::PauseDeferred(boundary, ready, release)) => convert_deferral(ProjectAdmissionDeferralV2::new().with_pause_for_testing(boundary, ready, release).initialize(root, value), fallback),
            (InitialPrepared::Established(value), GovernanceTestFaultV1::Initial(boundary)) => convert_initial(ProjectAdmissionGovernanceV1::new().with_fault_for_testing(boundary).initialize(root, value), fallback),
            (InitialPrepared::Established(value), GovernanceTestFaultV1::PauseInitial(boundary, ready, release)) => convert_initial(ProjectAdmissionGovernanceV1::new().with_pause_for_testing(boundary, ready, release).initialize(root, value), fallback),
            _ => ProjectGovernanceExecutionV1::Refused { refusal: GovernanceRefusalV1::new("test_fault_mismatch", "Fault does not apply to this prepared operation") },
        }
    }
    pub fn resolve_project_governance_request_with_fault_for_testing(
        &self, root: &std::path::Path, locator: &ProjectGovernanceLocatorV1, fault: GovernanceTestFaultV1,
    ) -> ProjectGovernanceExecutionV1 {
        if let Err(refusal) = validate_native_locator(locator) {
            return ProjectGovernanceExecutionV1::Refused { refusal };
        }
        match (locator, fault) {
            (ProjectGovernanceLocatorV1::InitializeEstablished(value),
             GovernanceTestFaultV1::PauseInitial(boundary @ GovernanceFaultBoundaryV1::NativeClosure(_), ready, release)) => {
                let native = match super::semantic_types::translate(value) {
                    Ok(value) => value, Err(refusal) => return ProjectGovernanceExecutionV1::Refused { refusal },
                };
                convert_initial(ProjectAdmissionGovernanceV1::new()
                    .with_pause_for_testing(boundary, ready, release).resolve_request(root, &native), locator.clone())
            }
            (ProjectGovernanceLocatorV1::InitializeDeferred(value),
             GovernanceTestFaultV1::PauseDeferred(boundary @ DeferralFaultBoundaryV2::NativeClosure(_), ready, release)) => {
                let native = match super::semantic_types::translate(value) {
                    Ok(value) => value, Err(refusal) => return ProjectGovernanceExecutionV1::Refused { refusal },
                };
                convert_deferral(ProjectAdmissionDeferralV2::new()
                    .with_pause_for_testing(boundary, ready, release).resolve_request(root, &native), locator.clone())
            }
            (ProjectGovernanceLocatorV1::EstablishDeferred(value),
             GovernanceTestFaultV1::PauseEstablishment(boundary @ EstablishmentFaultBoundaryV3::NativeClosure(_), ready, release)) => {
                let native = match super::semantic_types::translate(value) {
                    Ok(value) => value, Err(refusal) => return ProjectGovernanceExecutionV1::Refused { refusal },
                };
                convert_establishment(ProjectAdmissionEstablishmentV3::new()
                    .with_pause_for_testing(boundary, ready, release).resolve_request(root, &native), locator.clone())
            }
            _ => ProjectGovernanceExecutionV1::Refused {
                refusal: GovernanceRefusalV1::new("test_fault_mismatch", "Exact native closure fault required"),
            },
        }
    }
    pub fn establish_with_fault_for_testing(&self, root: &std::path::Path, prepared: PreparedProjectGovernanceEstablishmentV1, fault: GovernanceTestFaultV1) -> ProjectGovernanceExecutionV1 {
        let result = match fault {
            GovernanceTestFaultV1::Establishment(boundary) => ProjectAdmissionEstablishmentV3::new().with_fault_for_testing(boundary).establish(root, prepared.prepared),
            GovernanceTestFaultV1::PauseEstablishment(boundary, ready, release) => ProjectAdmissionEstablishmentV3::new().with_pause_for_testing(boundary, ready, release).establish(root, prepared.prepared),
            _ => return ProjectGovernanceExecutionV1::Refused { refusal: GovernanceRefusalV1::new("test_fault_mismatch", "Fault does not apply to this prepared operation") },
        };
        convert_establishment(result, prepared.locator)
    }
}

#[cfg(test)]
mod owner_comparison_tests {
    use super::*;

    #[test]
    fn semantic_replay_and_current_match_native_owner_facts() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        let owner = ProjectGovernanceV1::new();
        let request = InitialDeferralRequestV1::human_approved("owner-comparison-d", "Approve the exact deferral.").unwrap();
        let prepared = owner.prepare_initial_project_governance(root, &InitialProjectGovernanceV1::Deferred(request)).unwrap();
        let d_locator = prepared.locator().clone();
        assert!(matches!(owner.initialize_project_governance(root, prepared), ProjectGovernanceExecutionV1::Committed { .. }));
        let ProjectGovernanceLocatorV1::InitializeDeferred(ref d) = d_locator else { panic!("expected deferral") };
        let native_d: DeferralRequestLocatorV2 = super::super::semantic_types::translate(d).unwrap();
        let DeferralExecutionV2::Committed { outcome: native_d_outcome, delivery: GovernanceDeliveryV1::Replay } = ProjectAdmissionDeferralV2::new().resolve_request(root, &native_d) else { panic!("expected native replay") };
        let ProjectGovernanceExecutionV1::Committed { outcome: ProjectGovernanceCommittedV1::InitialDeferred(d_outcome), delivery: GovernanceDeliveryV1::Replay } = owner.resolve_project_governance_request(root, &d_locator) else { panic!("expected semantic replay") };
        let projected_d: DeferralCommittedOutcomeV2 = super::super::semantic_types::translate(&d_outcome).unwrap();
        assert_eq!(projected_d, native_d_outcome);
        let native_basis = ProjectAdmissionDeferralV2::new().resolve_current(root, &native_d.project, ADMISSION_GOVERNANCE_SCOPE_V1, None).unwrap();
        let selection = GovernanceProjectSelectionV1::InitiallyDeferred(d.project.clone());
        let CurrentProjectGovernanceV1::Deferred { basis, original } = owner.resolve_current_project_governance(root, &selection, ADMISSION_GOVERNANCE_SCOPE_V1, None).unwrap() else { panic!("expected semantic deferred") };
        assert_eq!(original, *d);
        let projected_basis: DeferredGovernanceBasisV2 = super::super::semantic_types::translate(&basis).unwrap();
        assert_eq!(projected_basis, native_basis);
        assert_eq!(projected_basis.canonical_json_bytes().unwrap(), native_basis.canonical_json_bytes().unwrap());
        let request = GovernanceEstablishmentRequestV1::human_approved(basis.project, DeferredRevisionV1 { outcome_fingerprint: basis.outcome_fingerprint, relation_fingerprint: basis.relation_fingerprint }, 7200, "owner-comparison-g", "Approve the exact configuration.").unwrap();
        let prepared = owner.prepare_project_governance_establishment(root, &request).unwrap();
        let g_locator = prepared.locator().clone();
        assert!(matches!(owner.establish_project_governance(root, prepared), ProjectGovernanceExecutionV1::Committed { .. }));
        assert!(resolve_project_admission_scope(
            root,
            &ProjectAdmissionBindingV2::ScopedChoiceV2(native_d.project.clone()),
            ADMISSION_GOVERNANCE_SCOPE_V1,
            None,
        ).is_err());
        let ProjectGovernanceLocatorV1::EstablishDeferred(ref g) = g_locator else { panic!("expected establishment") };
        let native_g: DeferredEstablishmentLocatorV3 = super::super::semantic_types::translate(g).unwrap();
        let DeferredEstablishmentExecutionV3::Committed { outcome: native_g_outcome, delivery: GovernanceDeliveryV1::Replay } = ProjectAdmissionEstablishmentV3::new().resolve_request(root, &native_g) else { panic!("expected native replay") };
        let ProjectGovernanceExecutionV1::Committed { outcome: ProjectGovernanceCommittedV1::EstablishedFromDeferred(g_outcome), delivery: GovernanceDeliveryV1::Replay } = owner.resolve_project_governance_request(root, &g_locator) else { panic!("expected semantic replay") };
        let projected_g: DeferredEstablishmentCommittedOutcomeV3 = super::super::semantic_types::translate(&g_outcome).unwrap();
        assert_eq!(projected_g, native_g_outcome);
        let DeferredScopeBasisV3::EstablishedFromDeferredV3(native_basis) = ProjectAdmissionEstablishmentV3::new().resolve_current(root, &native_d.project, ADMISSION_GOVERNANCE_SCOPE_V1, None).unwrap() else { panic!("expected native established") };
        let CurrentProjectGovernanceV1::Established { origin: EstablishedGovernanceOriginV1::FromDeferred { basis, original } } = owner.resolve_current_project_governance(root, &selection, ADMISSION_GOVERNANCE_SCOPE_V1, None).unwrap() else { panic!("expected semantic established") };
        assert_eq!(original, *d);
        let projected_basis: EstablishedFromDeferredBasisV3 = super::super::semantic_types::translate(&basis).unwrap();
        assert_eq!(projected_basis, native_basis);
        assert_eq!(projected_basis.canonical_json_bytes().unwrap(), native_basis.canonical_json_bytes().unwrap());
    }

    #[test]
    fn semantic_preparation_matches_native_unavailable_predecessor() {
        for missing_claim in [true, false] {
            let dir = tempfile::tempdir().unwrap();
            let root = dir.path();
            let owner = ProjectGovernanceV1::new();
            let request = InitialDeferralRequestV1::human_approved("unavailable-predecessor-d", "Approve the exact deferral.").unwrap();
            let prepared = owner.prepare_initial_project_governance(root, &InitialProjectGovernanceV1::Deferred(request)).unwrap();
            let ProjectGovernanceLocatorV1::InitializeDeferred(d) = prepared.locator().clone() else { panic!("expected deferral") };
            assert!(matches!(owner.initialize_project_governance(root, prepared), ProjectGovernanceExecutionV1::Committed { .. }));
            let selection = GovernanceProjectSelectionV1::InitiallyDeferred(d.project);
            let CurrentProjectGovernanceV1::Deferred { basis, .. } = owner.resolve_current_project_governance(root, &selection, ADMISSION_GOVERNANCE_SCOPE_V1, None).unwrap() else { panic!("expected deferred") };
            let request = GovernanceEstablishmentRequestV1::human_approved(basis.project, DeferredRevisionV1 { outcome_fingerprint: basis.outcome_fingerprint, relation_fingerprint: basis.relation_fingerprint }, 7200, "unavailable-predecessor-g", "Approve the exact configuration.").unwrap();
            let native: DeferredEstablishmentRequestV3 = super::super::semantic_types::translate(&request).unwrap();
            if missing_claim {
                std::fs::remove_file(root.join(handbook_engine::REPOSITORY_INITIALIZATION_CLAIM_PATH_V1)).unwrap();
            } else {
                std::fs::create_dir(root.join(".handbook/admission-governance/v4")).unwrap();
            }
            let semantic_error = owner.prepare_project_governance_establishment(root, &request).unwrap_err();
            let native_error = ProjectAdmissionEstablishmentV3::new().prepare(root, &native).unwrap_err();
            assert_eq!(semantic_error, native_error);
        }
    }
}
