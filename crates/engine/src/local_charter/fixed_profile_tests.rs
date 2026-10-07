use super::*;
use crate::{
    CharterAcquisitionMode, CharterCoverageSubmission, CharterIntakeConsumer,
    CharterIntakeSourceKind,
};

fn supplied_request() -> NativeLocalCharterRequestV1 {
    let bytes = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../docs/specs/handbook-contract-membrane/slices/HCM-2.2/contracts/canonical-charter-boundary-v1.1.yaml"));
    NativeLocalCharterRequestV1 {
        request_key: "fixed-profile-equivalence".into(),
        intake: CharterIntakeEnvelope {
            mode: CharterAcquisitionMode::Express,
            content: parse_definition_yaml(bytes).unwrap(),
            coverage: crate::charter_intake::COVERAGE_ORDER
                .iter()
                .map(|id| CharterCoverageSubmission {
                    coverage_id: (*id).into(),
                    source_kind: CharterIntakeSourceKind::UserDeclaration,
                    value_ref: format!("intake-values/{id}.json"),
                    evidence_refs: vec![],
                    confidence: "high".into(),
                    freshness: None,
                    sensitivity: "internal".into(),
                    contradiction_refs: vec![],
                    waiver_ref: None,
                })
                .collect(),
            consumer: CharterIntakeConsumer {
                kind: "sdk_consumer".into(),
                id: "fixed-profile-test".into(),
                version: "1.0.0".into(),
            },
            prompt_event_refs: vec!["decision.test.fixed-profile".into()],
            finalized_at_utc: "2026-10-05T12:00:00Z".into(),
            expected_current_fingerprint: None,
        },
        human_approval: LocalCharterHumanApprovalV1 {
            accepted: true,
            principal_claim: "test-human".into(),
            agent_claim: "test-agent".into(),
            delegation_claim: "initial local Charter establishment".into(),
            source_ref: "decision.test.fixed-profile".into(),
            approved_content_fingerprint: crate::charter_source_fingerprint(bytes).to_string(),
        },
    }
}

#[test]
fn fixed_charter_profile_matches_uncached_validation_and_rechecks_supplied_content() {
    let fresh =
        crate::resolve_shipped_profile_decisions(Path::new("not-an-initialized-repository"))
            .unwrap();
    let cached = fixed_charter_decisions().unwrap();
    assert_eq!(fresh.profile_ref(), cached.profile_ref());
    assert_eq!(
        fresh.profile_definition_fingerprint(),
        cached.profile_definition_fingerprint()
    );
    assert_eq!(fresh.profile_fingerprint(), cached.profile_fingerprint());
    assert_eq!(
        fresh.stable_role_registry_fingerprint(),
        cached.stable_role_registry_fingerprint()
    );
    assert_eq!(fresh.artifact_decisions(), cached.artifact_decisions());
    assert_eq!(
        fresh.condition_evaluations(),
        cached.condition_evaluations()
    );
    assert_eq!(fresh.capability_truth(), cached.capability_truth());
    assert_eq!(
        local_profile(&fresh).unwrap(),
        local_profile(cached).unwrap()
    );
    let request = supplied_request();
    let uncached = evaluate_charter_intake(&fresh, request.intake.clone(), None).unwrap();
    let actual = evaluate(Path::new("another-uninitialized-root"), &request).unwrap();
    assert_eq!(actual.0, uncached.normalized_content);
    assert_eq!(actual.1, local_profile(&fresh).unwrap());
    let bound = bind_exact_canonical_bytes(&uncached.normalized_content).unwrap();
    let posture = derive_project_posture_kernel(&uncached.charter, &bound).unwrap();
    assert_eq!(
        actual.2,
        json!({"coverage_results":uncached.intake.coverage_results,"field_sources":uncached.candidate_subject.field_sources,"posture_kernel_fingerprint":posture.kernel_fingerprint,"posture_replay_plan":posture.replay_plan})
    );
    crate::schema_registry::take_validator_build_count();
    for semantic_error in [false, true] {
        let mut altered = request.clone();
        if semantic_error {
            altered.intake.content["engineering_posture"]["dimensions"][0]["dimension_id"] =
                json!("unrecognized");
        } else {
            altered.intake.content["project"]["team_size"] = json!("invalid");
        }
        let expected = evaluate_charter_intake(&fresh, altered.intake.clone(), None).unwrap_err();
        let refused = evaluate(Path::new("another-uninitialized-root"), &altered).unwrap_err();
        assert_eq!(refused.code, "charter_intake_refused");
        assert_eq!(refused.detail, expected.detail());
    }
    let mut altered = request.clone();
    altered.intake.content["project"]["name"] = json!("A separately supplied Charter");
    assert_eq!(
        evaluate(Path::new("another-uninitialized-root"), &altered)
            .unwrap_err()
            .code,
        "approved_content_mismatch"
    );
    let fresh_changed = evaluate_charter_intake(&fresh, altered.intake.clone(), None).unwrap();
    altered.human_approval.approved_content_fingerprint =
        DefinitionFingerprint::from_bytes(&fresh_changed.normalized_content).to_string();
    let changed = evaluate(Path::new("another-uninitialized-root"), &altered).unwrap();
    assert_eq!(changed.0, fresh_changed.normalized_content);
    assert_ne!(actual.0, changed.0);
    assert_eq!(crate::schema_registry::take_validator_build_count(), 0);
}

#[test]
#[ignore = "bounded performance discriminator; run alone for cold process caches"]
fn fixed_profile_cold_warm_discriminator() {
    use std::io::Write;
    use std::time::Instant;
    let mode = std::env::var("HANDBOOK_FIXED_PROFILE_PROBE").ok();
    if mode.as_deref() == Some("warm_cached") {
        crate::local_authority::profile().unwrap();
        fixed_charter_decisions().unwrap();
    }
    if mode.is_some() {
        println!("FIXED_PROFILE_READY");
        std::io::stdout().flush().unwrap();
        let mut start = String::new();
        std::io::stdin().read_line(&mut start).unwrap();
        assert_eq!(start.trim(), "start");
    }
    let measure = |label: &str, rounds: usize, work: &dyn Fn()| {
        if mode.as_deref().is_some_and(|selected| selected != label) {
            return;
        }
        crate::schema_registry::take_validator_build_count();
        let wall = Instant::now();
        for _ in 0..rounds {
            work();
        }
        println!(
            "FIXED_PROFILE {label} rounds={rounds} wall_seconds={:.6} validator_builds={}",
            wall.elapsed().as_secs_f64(),
            crate::schema_registry::take_validator_build_count()
        );
    };
    measure("uncached_control", 3, &|| {
        for _ in 0..4 {
            std::hint::black_box(
                crate::local_authority::resolved_local_operating_profile().unwrap(),
            );
        }
        for _ in 0..8 {
            std::hint::black_box(
                crate::resolve_shipped_profile_decisions(Path::new(env!("CARGO_MANIFEST_DIR")))
                    .unwrap(),
            );
        }
    });
    measure("cold_cached", 1, &|| {
        std::hint::black_box(crate::local_authority::profile().unwrap());
        std::hint::black_box(fixed_charter_decisions().unwrap());
    });
    measure("warm_cached", 100, &|| {
        for _ in 0..4 {
            std::hint::black_box(crate::local_authority::profile().unwrap());
        }
        for _ in 0..8 {
            std::hint::black_box(fixed_charter_decisions().unwrap());
        }
    });
}
