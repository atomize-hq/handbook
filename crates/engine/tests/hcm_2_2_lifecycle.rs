use handbook_engine::{
    address_charter_lifecycle, apply_charter_lifecycle_events, CharterLifecycleEvent,
    CharterLifecycleEventKind, CharterLifecycleState,
};

fn event(fingerprint: &str, kind: CharterLifecycleEventKind) -> CharterLifecycleEvent {
    CharterLifecycleEvent {
        event_fingerprint: fingerprint.to_owned(),
        basis_artifact_fingerprint:
            "sha256:1111111111111111111111111111111111111111111111111111111111111111".to_owned(),
        kind,
        evidence_ref: format!("evidence/{fingerprint}"),
        evidence_fingerprint:
            "sha256:2222222222222222222222222222222222222222222222222222222222222222".to_owned(),
    }
}

fn fingerprint(character: char) -> String {
    format!("sha256:{}", character.to_string().repeat(64))
}

#[test]
fn reassessment_precedes_review_and_reopens_only_the_targeted_coverage() {
    let transition = apply_charter_lifecycle_events(
        CharterLifecycleState::Current,
        &[],
        vec![
            event(&fingerprint('b'), CharterLifecycleEventKind::Review),
            event(
                &fingerprint('a'),
                CharterLifecycleEventKind::ProductionPostureChanged,
            ),
        ],
    )
    .expect("lifecycle transition");

    assert_eq!(
        transition.result_state,
        CharterLifecycleState::ReassessmentRequired
    );
    assert_eq!(
        transition
            .new_observations
            .iter()
            .map(|observation| observation.event_fingerprint.as_str())
            .collect::<Vec<_>>(),
        [fingerprint('a'), fingerprint('b')]
    );
    assert_eq!(
        transition.reopened_coverage_ids,
        ["operational_reality.production_state"]
    );
}

#[test]
fn duplicate_events_are_idempotent_and_clearance_requires_every_active_observation() {
    let initial = apply_charter_lifecycle_events(
        CharterLifecycleState::Current,
        &[],
        vec![event(&fingerprint('a'), CharterLifecycleEventKind::Review)],
    )
    .unwrap();
    let duplicate = apply_charter_lifecycle_events(
        initial.result_state,
        &initial.active_observations,
        vec![event(&fingerprint('a'), CharterLifecycleEventKind::Review)],
    )
    .unwrap();
    assert!(duplicate.new_observations.is_empty());
    assert_eq!(duplicate.active_observations, initial.active_observations);

    assert!(address_charter_lifecycle(&initial.active_observations, &[], &[]).is_err());
    let cleared = address_charter_lifecycle(&initial.active_observations, &[fingerprint('a')], &[])
        .expect("fully addressed review");
    assert_eq!(cleared, CharterLifecycleState::Current);
}

#[test]
fn trust_boundary_reassessment_requires_exact_exception_policy_reevaluation() {
    let transition = apply_charter_lifecycle_events(
        CharterLifecycleState::Current,
        &[],
        vec![event(
            &fingerprint('c'),
            CharterLifecycleEventKind::TrustBoundaryChanged,
        )],
    )
    .unwrap();
    assert!(
        address_charter_lifecycle(&transition.active_observations, &[fingerprint('c')], &[],)
            .is_err()
    );
    assert_eq!(
        address_charter_lifecycle(
            &transition.active_observations,
            &[fingerprint('c')],
            &["governance.exception_policy".to_owned()],
        )
        .unwrap(),
        CharterLifecycleState::Current
    );
}
