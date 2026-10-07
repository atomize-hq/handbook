//! Exact new authored candidates and actual distinct review drive qualification.
use super::*;

fn selected_definition(
    fixture: &Fixture,
    version: &str,
    age: u64,
    non_applicable: bool,
    unsupported_scope: bool,
) -> LocalContractDefinitionV3 {
    let (intrinsic, stack) = supplied_definition_with_version(
        LocalPermittedUseV2::DecisionSeekingEligible,
        age,
        fixture
            .definition
            .semantic_definition()
            .definition_authority()
            .clone(),
        GateEffectV1::Required,
        version,
    );
    let semantic = intrinsic.semantic_definition();
    let mut claims = semantic.claims().to_vec();
    if non_applicable {
        let mut value = serde_json::to_value(&claims[0]).unwrap();
        value["subject_selector"]["criteria"][0]["expected"] =
            json!("handbook.input.another-known-role");
        claims[0] = serde_json::from_value(value).unwrap();
    }
    let value = serde_json::to_value(semantic).unwrap();
    let semantic = ContractDefinitionV1::new(ContractDefinitionInputV1 {
        definition_schema: serde_json::from_value(value["definition_schema"].clone()).unwrap(),
        contract_id: semantic.contract_id().clone(),
        contract_version: semantic.contract_version().clone(),
        contract_kind: semantic.contract_kind(),
        claims,
        definition_author_ref: semantic.definition_author_ref().clone(),
        definition_authority: semantic.definition_authority().clone(),
        lifecycle_policy: semantic.lifecycle_policy().clone(),
        matcher_bindings: vec![MatcherBindingV1::exact_fields_v1()],
        gate_policy: intrinsic.dependencies().gate_policy.exact_binding(),
        extensions_policy: intrinsic.dependencies().extensions_policy.clone(),
    })
    .unwrap();
    let mut dependencies = intrinsic.dependencies().clone();
    if unsupported_scope {
        dependencies.required_governance_scopes = vec!["unsupported.required.q".into()];
    }
    let intrinsic = LocalContractDefinitionV2::new(
        LocalContractDefinitionInputV2 {
            semantic_definition: semantic,
            permitted_use: LocalPermittedUseV2::DecisionSeekingEligible,
            dependencies,
        },
        &stack,
    )
    .unwrap();
    LocalContractDefinitionV3::new(
        LocalContractDefinitionInputV3 {
            dependencies: LocalDefinitionDependenciesV3::new(intrinsic.dependencies().clone())
                .unwrap(),
            intrinsic_definition: intrinsic,
        },
        &stack,
    )
    .unwrap()
}

fn register_reviewed(fixture: &Fixture, definition: &LocalContractDefinitionV3, label: &str) {
    let profile = handbook_engine::local_authority::resolved_local_operating_profile().unwrap();
    let check = &fixture.authority.predecessor;
    let prepared = fixture
        .sdk
        .prepare_local_evaluation_definition(
            &fixture.initial,
            check,
            definition.clone(),
            profile.context_resolution(),
        )
        .unwrap();
    fixture
        .sdk
        .append_local_evaluation_definition(
            check,
            prepared,
            now(),
            &format!("packet3-{label}-author-0001"),
        )
        .unwrap();
    for (state, review, edge) in [
        (LifecycleStateV1::ReviewReady, None, "ready"),
        (LifecycleStateV1::Locked, Some(LocalReviewInputV3 {
            definition: definition.binding(),
            closure_fingerprint: definition.closure_fingerprint().clone(),
            permitted_use: definition.permitted_use(),
            author_participant_ref: "packet2.author".into(),
            reviewer_participant_ref: "packet2.reviewer".into(),
            reviewer_actor_ref: "packet2.reviewer".into(),
            decision: LocalReviewDecisionV2::Approve,
            source_ref: format!("review.packet3.illustrative-{label}-exact-c"),
            outcome_text: "Supplied illustrative distinct participant review of this exact new selector, freshness and full evaluation policy closure.".into(),
            semantic_time: now(),
        }), "review"),
        (LifecycleStateV1::Active, None, "active"),
    ] {
        let head = fixture.sdk.current_local_evaluation_lifecycle(&definition.binding()).unwrap();
        let prepared = fixture.sdk.prepare_local_evaluation_lifecycle(&fixture.initial, &definition.binding(), LocalLifecycleTransitionInputV3 {
            expected_lifecycle_fingerprint: head.lifecycle_fingerprint().clone(),
            to_state: state, review: review.clone(), reason: "Supplied exact new candidate lifecycle edge.".into(),
        }).unwrap();
        let (actor, participant, time) = review.map(|r| (r.reviewer_actor_ref, r.reviewer_participant_ref, r.semantic_time))
            .unwrap_or_else(|| ("packet2.author".into(), "packet2.author".into(), now()));
        fixture.sdk.transition_local_evaluation_lifecycle(check, prepared, actor, participant, time, &format!("packet3-{label}-{edge}-0001")).unwrap();
    }
}

#[test]
fn native_exact_non_applicability_and_fresh_g_qualify_while_required_q_and_over_ceiling_refuse() {
    on_stack(|| {
        let _package = InstalledProductionPackage::install();
        let mut fixture = Fixture::new();
        let original = fixture.definition.clone();
        let non_applicable = selected_definition(&fixture, "3.0.0", 3600, true, false);
        register_reviewed(&fixture, &non_applicable, "non-applicable");
        let over_ceiling = selected_definition(&fixture, "4.0.0", 3601, false, false);
        register_reviewed(&fixture, &over_ceiling, "over-ceiling");
        let unsupported = selected_definition(&fixture, "5.0.0", 3600, false, true);
        let profile = handbook_engine::local_authority::resolved_local_operating_profile().unwrap();
        assert!(fixture
            .sdk
            .prepare_local_evaluation_definition(
                &fixture.initial,
                &fixture.authority.predecessor,
                unsupported,
                profile.context_resolution(),
            )
            .is_err());
        use handbook_contracts::project_admission_governance::{
            EstablishmentFaultBoundaryV3, ProjectAdmissionEstablishmentV3,
        };
        let local_use = fixture
            .sdk
            .prepare_local_evaluation_use(
                &fixture.initial,
                &original.binding(),
                LocalContractUseIntentV2::DecisionSeeking,
            )
            .unwrap();
        let ProjectAdmissionScopeBasisV3::DeferredV2(noncommit_d) =
            local_use.applicability().governance_basis.clone()
        else {
            panic!("actual D required")
        };
        let governance_owner = ProjectAdmissionEstablishmentV3::new();
        let request = DeferredEstablishmentRequestV3::human_approved(
            noncommit_d.project.clone(), ExpectedDeferredBasisV2 { outcome_fingerprint: noncommit_d.outcome_fingerprint.clone(), relation_fingerprint: noncommit_d.relation_fingerprint.clone() },
            3600, "packet3-qualification-closed-noncommit-0001", "Illustrative supplied exact establishment request whose retained intent is interrupted before decision.",
        ).unwrap();
        let prepared = governance_owner
            .prepare(fixture.root.path(), &request)
            .unwrap();
        let original_locator = prepared.locator().clone();
        assert!(matches!(
            ProjectAdmissionEstablishmentV3::new()
                .with_fault_for_testing(EstablishmentFaultBoundaryV3::Intent)
                .establish(fixture.root.path(), prepared),
            DeferredEstablishmentExecutionV3::Uncertain { .. }
        ));
        assert!(matches!(
            governance_owner.resolve_request(fixture.root.path(), &original_locator),
            DeferredEstablishmentExecutionV3::NotCommitted { .. }
        ));
        fixture.definition = non_applicable;
        let non_applicable = fixture.form(
            "packet3-native-not-applicable-0001",
            supplied_validation_content(),
            production_manifest().resolution_ceiling().clone(),
            LocalEvaluationUseIntentV1::DecisionSeeking,
        );
        fixture.assess(&non_applicable);
        let session = fixture.session(&non_applicable);
        fixture
            .sdk
            .commit_local_evaluation_collection(&session)
            .unwrap();
        fixture
            .sdk
            .execute_local_evaluation_once(
                &session,
                non_applicable.process_request.as_ref().unwrap(),
                &dock_reach(),
                &DockCancellationV1::new(),
            )
            .unwrap();
        let result = finish(&fixture, &non_applicable);
        assert_eq!(result.gate, LocalEvaluationGateV3::NotApplicable);
        assert_eq!(
            result.claim.applicability,
            ApplicabilityOutcomeV1::NotApplicable
        );
        assert_eq!(result.claim.finding, "not_assessed");
        assert_eq!(
            result.evidence_set.members.len(),
            1,
            "positive applicability comes from matcher, not omitted source"
        );
        let original_result_bytes = result.canonical_json_bytes().unwrap();
        use handbook_contracts::project_admission_governance::{
            ExpectedDeferredBasisV2, ProjectAdmissionScopeBasisV3,
        };
        use handbook_sdk::admission_governance::{
            DeferredEstablishmentExecutionV3, DeferredEstablishmentRequestV3,
        };
        let ProjectAdmissionScopeBasisV3::DeferredV2(d) = non_applicable
            .snapshot
            .requirements
            .applicability
            .governance_basis
            .clone()
        else {
            panic!("actual D required")
        };
        let prepared = fixture.sdk.prepare_deferred_project_admission_governance(&DeferredEstablishmentRequestV3::human_approved(
            d.project, ExpectedDeferredBasisV2 { outcome_fingerprint: d.outcome_fingerprint, relation_fingerprint: d.relation_fingerprint },
            3600, "packet3-qualification-establish-g-0001", "Illustrative supplied human approval of this exact same-project freshness ceiling.",
        ).unwrap()).unwrap();
        assert!(matches!(
            fixture
                .sdk
                .establish_deferred_project_admission_governance(prepared),
            DeferredEstablishmentExecutionV3::Committed { .. }
        ));
        fixture.definition = original;
        let fresh_g = fixture.form(
            "packet3-native-fresh-g-complete-0001",
            supplied_validation_content(),
            production_manifest().resolution_ceiling().clone(),
            LocalEvaluationUseIntentV1::DecisionSeeking,
        );
        assert!(matches!(
            fresh_g.snapshot.requirements.applicability.governance_basis,
            ProjectAdmissionScopeBasisV3::EstablishedFromDeferredV3(_)
        ));
        assert!(fresh_g
            .snapshot
            .requirements
            .applicability
            .dependencies
            .iter()
            .any(|d| d.kind == "evidence_freshness_policy"
                && d.qualification == LocalDependencyQualificationV2::QualifiedFreshness));
        assert!(fresh_g
            .snapshot
            .requirements
            .applicability
            .dependencies
            .iter()
            .filter(|d| d.kind != "evidence_freshness_policy")
            .all(|d| d.qualification
                == LocalDependencyQualificationV2::NotApplicableToFreshnessScope));
        fixture.assess(&fresh_g);
        let session = fixture.session(&fresh_g);
        fixture
            .sdk
            .commit_local_evaluation_collection(&session)
            .unwrap();
        fixture
            .sdk
            .execute_local_evaluation_once(
                &session,
                fresh_g.process_request.as_ref().unwrap(),
                &dock_reach(),
                &DockCancellationV1::new(),
            )
            .unwrap();
        assert_eq!(finish(&fixture, &fresh_g).gate, LocalEvaluationGateV3::Pass);
        assert!(fixture
            .sdk
            .prepare_local_evaluation_use(
                &fixture.initial,
                &over_ceiling.binding(),
                LocalContractUseIntentV2::DecisionSeeking
            )
            .is_err());
        let mut input = fresh_g.snapshot.original_input.clone();
        input.request_key = "packet3-over-ceiling-dependent-formation-0001".into();
        input.contract = over_ceiling.binding();
        input.expected_lifecycle_fingerprint = fixture
            .sdk
            .current_local_evaluation_lifecycle(&over_ceiling.binding())
            .unwrap()
            .lifecycle_fingerprint()
            .clone();
        assert!(fixture
            .sdk
            .prepare_local_evaluation(&fixture.initial, &fixture.authority, input)
            .is_err());
        let history = fixture
            .sdk
            .read_local_evaluation_history(&non_applicable.snapshot.locator)
            .unwrap();
        assert_eq!(
            history.result.canonical_json_bytes().unwrap(),
            original_result_bytes
        );
        assert_eq!(history.result.gate, LocalEvaluationGateV3::NotApplicable);
    });
}
