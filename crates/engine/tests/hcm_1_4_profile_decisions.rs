#[cfg(unix)]
use handbook_engine::{
    resolve_profile_selection, DefinitionSource, DefinitionSourceBinding, ExactDefinitionRef,
    ProfileSelectionRequest, ResolvedProfileDecisions,
};
use handbook_engine::{
    resolve_shipped_profile_decisions, ArtifactApplicability, ProjectConditionDecisionReason,
    ProjectConditionOutcome, RequirednessMode,
};
use std::path::Path;

#[test]
fn shipped_profile_registry_retains_exact_six_kind_refs() {
    let decisions = resolve_shipped_profile_decisions(Path::new(env!("CARGO_MANIFEST_DIR")))
        .expect("shipped decisions");
    let actual = decisions
        .registry()
        .kind_refs()
        .into_iter()
        .map(|reference| reference.as_str())
        .collect::<Vec<_>>();
    assert_eq!(
        actual,
        [
            "handbook.artifact-kind.decision-record@1.0.0",
            "handbook.artifact-kind.environment-context@1.0.0",
            "handbook.artifact-kind.project-authority@1.0.0",
            "handbook.artifact-kind.project-authority@1.1.0",
            "handbook.artifact-kind.project-context@1.0.0",
            "handbook.artifact-kind.risk-record@1.0.0",
            "handbook.artifact-kind.work-specification@1.0.0",
        ]
    );
}

#[test]
fn shipped_profile_artifact_rows_match_exact_selected_fields() {
    let decisions = resolve_shipped_profile_decisions(Path::new(env!("CARGO_MANIFEST_DIR")))
        .expect("shipped decisions");
    let actual = decisions
        .artifact_decisions()
        .iter()
        .map(|decision| {
            (
                decision.instance_id().as_str(),
                decision.kind_ref().as_str(),
                decision.role_id(),
                decision.canonical_path(),
                decision.requiredness_mode(),
                decision.applicability(),
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(
        actual,
        [
            (
                "environment_context",
                "handbook.artifact-kind.environment-context@1.0.0",
                Some("environment_context"),
                ".handbook/project/environment.yaml",
                RequirednessMode::Conditional,
                ArtifactApplicability::Indeterminate,
            ),
            (
                "project_authority",
                "handbook.artifact-kind.project-authority@1.1.0",
                Some("constitutional_authority"),
                ".handbook/project/charter.yaml",
                RequirednessMode::Always,
                ArtifactApplicability::Required,
            ),
            (
                "project_context",
                "handbook.artifact-kind.project-context@1.0.0",
                Some("project_context"),
                ".handbook/project/context.yaml",
                RequirednessMode::Always,
                ArtifactApplicability::Required,
            ),
        ]
    );
}

#[test]
fn always_requiredness_maps_to_required_without_condition_truth() {
    let decisions = resolve_shipped_profile_decisions(Path::new(env!("CARGO_MANIFEST_DIR")))
        .expect("shipped decisions");
    for id in ["project_authority", "project_context"] {
        let decision = decisions
            .artifact_decisions()
            .iter()
            .find(|decision| decision.instance_id().as_str() == id)
            .expect("always decision");
        assert_eq!(decision.requiredness_mode(), RequirednessMode::Always);
        assert_eq!(decision.applicability(), ArtifactApplicability::Required);
        assert!(decision.condition_ref().is_none());
        assert!(decision.condition_outcome().is_none());
        assert!(decision.condition_reason().is_none());
        assert!(decision.evidence_closure_fingerprint().is_none());
    }
}

#[test]
fn conditional_requiredness_binds_exact_unresolved_definition_truth() {
    let decisions = resolve_shipped_profile_decisions(Path::new(env!("CARGO_MANIFEST_DIR")))
        .expect("shipped decisions");
    let decision = decisions
        .artifact_decisions()
        .iter()
        .find(|decision| decision.instance_id().as_str() == "environment_context")
        .expect("conditional decision");
    assert_eq!(decision.requiredness_mode(), RequirednessMode::Conditional);
    assert_eq!(
        decision.condition_ref().map(|reference| reference.as_str()),
        Some("handbook.condition.project.managed-operational-surface@1.0.0")
    );
    assert_eq!(
        decision.condition_outcome(),
        Some(ProjectConditionOutcome::Unresolved)
    );
    assert_eq!(
        decision.condition_reason(),
        Some(ProjectConditionDecisionReason::EvidenceContractUnavailable)
    );
    assert_eq!(
        decision.applicability(),
        ArtifactApplicability::Indeterminate
    );
    assert!(decision.evidence_closure_fingerprint().is_none());

    let evaluation = decisions.condition_evaluations().first().unwrap();
    assert_eq!(
        evaluation.condition_ref().as_str(),
        "handbook.condition.project.managed-operational-surface@1.0.0"
    );
    assert_eq!(
        evaluation.condition_definition_fingerprint().as_str(),
        "sha256:2ae25788c7860f3062f30659a7674c2ccd8f56b0f8809f1134003e04dea20b61"
    );
    assert_eq!(evaluation.outcome(), ProjectConditionOutcome::Unresolved);
    assert_eq!(
        evaluation.reason(),
        ProjectConditionDecisionReason::EvidenceContractUnavailable
    );
    assert!(evaluation.evidence_closure_fingerprint().is_none());
}

#[test]
fn shipped_capability_identity_projection_is_exact_and_four_field_only() {
    let decisions = resolve_shipped_profile_decisions(Path::new(env!("CARGO_MANIFEST_DIR")))
        .expect("shipped decisions");
    let rows = decisions.capability_truth();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].instance_id().as_str(), "project_authority");
    assert_eq!(rows[0].capability_id().as_str(), "constitutional_root");
    assert_eq!(
        rows[0].contract_ref().as_str(),
        "handbook.capabilities.constitutional-root@1.0.0"
    );
    assert_eq!(
        rows[0].contract_fingerprint().as_str(),
        "sha256:1d4a1c2f85158c14524559e6846bf805eff4e531d8a78ac5c4890e2a4c0b0998"
    );
}

#[test]
fn shipped_profile_decisions_are_complete_ordered_and_evidence_free() {
    let repo = Path::new(env!("CARGO_MANIFEST_DIR"));
    let decisions = resolve_shipped_profile_decisions(repo).expect("shipped decisions");

    assert_eq!(
        decisions.profile_ref().as_str(),
        "handbook.profile.shipped-root@1.1.0"
    );
    assert!(!decisions.artifact_decisions().is_empty());
    assert!(decisions
        .artifact_decisions()
        .windows(2)
        .all(|pair| pair[0].instance_id() < pair[1].instance_id()));

    for decision in decisions.artifact_decisions() {
        match decision.requiredness_mode() {
            RequirednessMode::Always => {
                assert_eq!(decision.applicability(), ArtifactApplicability::Required);
                assert!(decision.condition_ref().is_none());
            }
            RequirednessMode::Optional => {
                assert_eq!(decision.applicability(), ArtifactApplicability::Optional);
                assert!(decision.condition_ref().is_none());
            }
            RequirednessMode::Conditional => {
                assert_eq!(
                    decision.applicability(),
                    ArtifactApplicability::Indeterminate
                );
                assert_eq!(
                    decision.condition_outcome(),
                    Some(ProjectConditionOutcome::Unresolved)
                );
                assert_eq!(
                    decision.condition_reason(),
                    Some(ProjectConditionDecisionReason::EvidenceContractUnavailable)
                );
                assert!(decision.evidence_closure_fingerprint().is_none());
            }
        }
    }
}

#[test]
#[cfg(unix)]
fn repository_selected_profile_and_source_permutations_produce_identical_decisions() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let forward = resolve_profile_selection(root, repository_profile_request(false)).unwrap();
    let reverse = resolve_profile_selection(root, repository_profile_request(true)).unwrap();
    let forward = ResolvedProfileDecisions::from_profile(&forward).unwrap();
    let reverse = ResolvedProfileDecisions::from_profile(&reverse).unwrap();

    assert_eq!(forward.profile_ref().as_str(), "example.profile.root@1.0.0");
    assert_eq!(forward.profile_fingerprint(), reverse.profile_fingerprint());
    assert_eq!(
        forward
            .artifact_decisions()
            .iter()
            .map(|decision| (
                decision.instance_id().as_str(),
                decision.applicability(),
                decision.condition_outcome(),
            ))
            .collect::<Vec<_>>(),
        reverse
            .artifact_decisions()
            .iter()
            .map(|decision| (
                decision.instance_id().as_str(),
                decision.applicability(),
                decision.condition_outcome(),
            ))
            .collect::<Vec<_>>()
    );
}

#[cfg(unix)]
fn repository_profile_request(reverse: bool) -> ProfileSelectionRequest {
    let names = [
        "project-authority",
        "project-context",
        "environment-context",
        "work-specification",
        "decision-record",
        "risk-record",
    ];
    let mut schemas = names
        .map(|name| builtin(&format!("handbook.schemas.artifacts.{name}@1.0.0")))
        .to_vec();
    let mut kinds = names
        .map(|name| builtin(&format!("handbook.artifact-kind.{name}@1.0.0")))
        .to_vec();
    if reverse {
        schemas.reverse();
        kinds.reverse();
    }
    ProfileSelectionRequest {
        selected_profile_ref: exact("example.profile.root@1.0.0"),
        profile_sources: vec![DefinitionSourceBinding {
            definition_ref: exact("example.profile.root@1.0.0"),
            source: DefinitionSource::RepositoryPath(
                "tests/fixtures/hcm_1_2_repository_profile/root.yaml".to_owned(),
            ),
        }],
        stable_role_registry_sources: vec![builtin("handbook.roles.core@1.1.0")],
        schema_entry_sources: schemas,
        artifact_kind_sources: kinds,
        semantic_capability_sources: vec![builtin(
            "handbook.capabilities.constitutional-root@1.0.0",
        )],
        semantic_validator_sources: vec![builtin(
            "handbook.semantic-validation.constitutional-root@1.0.0",
        )],
        project_condition_sources: vec![builtin(
            "handbook.condition.project.managed-operational-surface@1.0.0",
        )],
        vocabulary_sources: vec![builtin("handbook.vocabulary.shipped-root@1.0.0")],
        context_resolution_sources: vec![builtin("handbook.context-resolution.shipped-root@1.0.0")],
        context_resolution_policy_sources: vec![
            builtin("handbook.mutation-matcher.core@1.0.0"),
            builtin("handbook.resolution-escalation.core@1.0.0"),
            builtin("handbook.memory-promotion.core@1.0.0"),
        ],
        intake_definition_sources: vec![],
        allowed_schema_roots: vec!["definitions/schemas".to_owned()],
    }
}

#[cfg(unix)]
fn exact(value: &str) -> ExactDefinitionRef {
    ExactDefinitionRef::parse(value).unwrap()
}

#[cfg(unix)]
fn builtin(value: &str) -> DefinitionSourceBinding {
    let reference = exact(value);
    DefinitionSourceBinding {
        definition_ref: reference.clone(),
        source: DefinitionSource::BuiltIn(reference),
    }
}
