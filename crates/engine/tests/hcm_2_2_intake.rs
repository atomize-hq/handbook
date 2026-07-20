use handbook_engine::{
    evaluate_charter_intake, parse_definition_yaml, resolve_shipped_profile_decisions,
    CharterAcquisitionMode, CharterCoverageSubmission, CharterIntakeConsumer,
    CharterIntakeEnvelope, CharterIntakeSourceKind,
};
use std::path::Path;

const BOUNDARY_YAML: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/specs/handbook-contract-membrane/slices/HCM-2.2/contracts/canonical-charter-boundary-v1.1.yaml"
));

const OBSERVATIONAL: [&str; 5] = [
    "project_shape.definition",
    "delivery.constraints",
    "operational_reality.production_state",
    "risk.domains",
    "debt.register",
];

const NORMATIVE: [&str; 11] = [
    "delivery.default_implications",
    "engineering_posture.baseline",
    "policy.authority_and_revision",
    "governance.decision_authority",
    "governance.required_approvals",
    "governance.exception_policy",
    "engineering_posture.dimensions",
    "engineering_posture.red_lines",
    "governance.review_triggers",
    "governance.reassessment_triggers",
    "decisions.records",
];

fn decisions() -> handbook_engine::ResolvedProfileDecisions {
    resolve_shipped_profile_decisions(Path::new(env!("CARGO_MANIFEST_DIR")))
        .expect("shipped decisions")
}

fn envelope(mode: CharterAcquisitionMode) -> CharterIntakeEnvelope {
    let mut coverage = Vec::new();
    for coverage_id in OBSERVATIONAL {
        coverage.push(CharterCoverageSubmission {
            coverage_id: coverage_id.to_owned(),
            source_kind: CharterIntakeSourceKind::EvidencedInference,
            value_ref: format!("intake-values/{coverage_id}.json"),
            evidence_refs: vec![format!("evidence.{coverage_id}")],
            confidence: "high".to_owned(),
            freshness: Some("session".to_owned()),
            sensitivity: "internal".to_owned(),
            contradiction_refs: vec![],
            waiver_ref: None,
        });
    }
    for coverage_id in NORMATIVE {
        coverage.push(CharterCoverageSubmission {
            coverage_id: coverage_id.to_owned(),
            source_kind: CharterIntakeSourceKind::UserDeclaration,
            value_ref: format!("intake-values/{coverage_id}.json"),
            evidence_refs: vec![],
            confidence: "high".to_owned(),
            freshness: None,
            sensitivity: "internal".to_owned(),
            contradiction_refs: vec![],
            waiver_ref: None,
        });
    }
    CharterIntakeEnvelope {
        mode,
        content: parse_definition_yaml(BOUNDARY_YAML).unwrap(),
        coverage,
        consumer: CharterIntakeConsumer {
            kind: "handbook_skill".to_owned(),
            id: "handbook".to_owned(),
            version: "1.1".to_owned(),
        },
        prompt_event_refs: vec![],
        finalized_at_utc: "2026-07-19T16:00:00Z".to_owned(),
        expected_current_fingerprint: None,
    }
}

#[test]
fn all_three_acquisition_modes_produce_identical_candidate_content_and_identity() {
    let decisions = decisions();
    let bundles = [
        CharterAcquisitionMode::GuidedAdaptive,
        CharterAcquisitionMode::Express,
        CharterAcquisitionMode::AgentAssisted,
    ]
    .map(|mode| {
        evaluate_charter_intake(&decisions, envelope(mode), None)
            .expect("eligible canonical Charter candidate")
    });

    for bundle in &bundles[1..] {
        assert_eq!(bundle.normalized_content, bundles[0].normalized_content);
        assert_eq!(
            bundle.candidate.field_sources,
            bundles[0].candidate.field_sources
        );
    }
    assert_eq!(bundles[0].intake.coverage_results.len(), 16);
    assert_eq!(bundles[0].candidate.field_sources.len(), 113);
    assert!(bundles[0].candidate.unresolved_coverage_ids.is_empty());
}

#[test]
fn normative_inference_unknown_contradiction_and_wrong_basis_refuse_closed() {
    let decisions = decisions();

    let mut inferred_normative = envelope(CharterAcquisitionMode::Express);
    inferred_normative.coverage[5].source_kind = CharterIntakeSourceKind::EvidencedInference;
    assert!(evaluate_charter_intake(&decisions, inferred_normative, None).is_err());

    let mut contradiction = envelope(CharterAcquisitionMode::Express);
    contradiction.coverage[0].contradiction_refs = vec!["contradiction.project".to_owned()];
    assert!(evaluate_charter_intake(&decisions, contradiction, None).is_err());

    let current = handbook_engine::DefinitionFingerprint::from_bytes(b"current");
    assert!(evaluate_charter_intake(
        &decisions,
        envelope(CharterAcquisitionMode::Express),
        Some(&current),
    )
    .is_err());
}
