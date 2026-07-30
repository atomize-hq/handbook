use handbook_engine::{
    ArtifactApplicability, ArtifactPresence, CanonicalArtifactIdentity, CanonicalArtifactKind,
    RequirednessMode,
};
use handbook_flow::{
    evaluate_budget_with_effective_bytes, BudgetByteDomain, BudgetDisposition,
    BudgetEffectiveBytes, BudgetPolicy,
};

const SELECTED_PATH: &str = ".handbook/project/context.yaml";

fn selected_identity(source_byte_len: u64) -> CanonicalArtifactIdentity {
    CanonicalArtifactIdentity {
        instance_id: "project_context".to_owned(),
        kind_ref: "handbook.artifact-kind.project-context@1.1.0".to_owned(),
        kind: CanonicalArtifactKind::ProjectContext,
        label: "Project Context".to_owned(),
        relative_path: SELECTED_PATH.to_owned(),
        requiredness_mode: RequirednessMode::Always,
        applicability: ArtifactApplicability::Required,
        renderer_definition_refs: vec![
            "handbook.renderer.project-context-review-markdown@1.0.0".to_owned()
        ],
        packet_required: true,
        baseline_required: true,
        setup_scaffolded: false,
        presence: ArtifactPresence::PresentNonEmpty,
        byte_len: Some(source_byte_len),
        content_sha256: Some("source".to_owned()),
        matches_setup_starter_template: false,
    }
}

fn evaluate(
    source_byte_len: u64,
    rendered_byte_len: u64,
    threshold: u64,
) -> handbook_flow::BudgetOutcome {
    evaluate_budget_with_effective_bytes(
        &[selected_identity(source_byte_len)],
        &[BudgetEffectiveBytes {
            canonical_repo_relative_path: SELECTED_PATH.to_owned(),
            byte_len: rendered_byte_len,
            byte_domain: BudgetByteDomain::RenderedOutput,
        }],
        BudgetPolicy {
            max_total_bytes: Some(threshold),
            max_per_artifact_bytes: Some(threshold),
        },
    )
}

#[test]
fn rendered_output_domain_controls_both_thresholds_when_render_is_larger() {
    let below = evaluate(5, 10, 9);
    assert_eq!(below.disposition, BudgetDisposition::Refuse);
    assert_eq!(below.targets[0].canonical_repo_relative_path, SELECTED_PATH);
    assert_eq!(below.targets[0].byte_len, 10);
    assert_eq!(
        below.targets[0].byte_domain,
        BudgetByteDomain::RenderedOutput
    );

    assert_eq!(evaluate(5, 10, 10).disposition, BudgetDisposition::Keep);
    assert_eq!(evaluate(5, 10, 11).disposition, BudgetDisposition::Keep);
}

#[test]
fn rendered_output_domain_controls_both_thresholds_when_source_is_larger() {
    let below = evaluate(10, 5, 4);
    assert_eq!(below.disposition, BudgetDisposition::Refuse);
    assert_eq!(below.targets[0].byte_len, 5);
    assert_eq!(
        below.targets[0].byte_domain,
        BudgetByteDomain::RenderedOutput
    );

    assert_eq!(evaluate(10, 5, 5).disposition, BudgetDisposition::Keep);
    assert_eq!(evaluate(10, 5, 6).disposition, BudgetDisposition::Keep);
}

#[test]
fn fixed_artifacts_retain_the_source_budget_domain() {
    let fixed = CanonicalArtifactIdentity {
        instance_id: "work_specification".to_owned(),
        kind_ref: "handbook.artifact-kind.work-specification@1.0.0".to_owned(),
        kind: CanonicalArtifactKind::FeatureSpec,
        label: "Work Specification".to_owned(),
        relative_path: ".handbook/feature_spec/FEATURE_SPEC.md".to_owned(),
        requiredness_mode: RequirednessMode::Optional,
        applicability: ArtifactApplicability::Optional,
        renderer_definition_refs: Vec::new(),
        packet_required: false,
        baseline_required: false,
        setup_scaffolded: false,
        presence: ArtifactPresence::PresentNonEmpty,
        byte_len: Some(11),
        content_sha256: Some("source".to_owned()),
        matches_setup_starter_template: false,
    };
    let outcome = evaluate_budget_with_effective_bytes(
        &[fixed],
        &[],
        BudgetPolicy {
            max_total_bytes: None,
            max_per_artifact_bytes: Some(10),
        },
    );
    assert_eq!(outcome.disposition, BudgetDisposition::Summarize);
    assert_eq!(outcome.targets[0].byte_domain, BudgetByteDomain::Source);
}

fn identity(path: &str, byte_len: u64, packet_required: bool) -> CanonicalArtifactIdentity {
    CanonicalArtifactIdentity {
        instance_id: path.replace(['/', '.'], "_"),
        kind_ref: "example.artifact-kind.unrendered@1.0.0".to_owned(),
        kind: CanonicalArtifactKind::FeatureSpec,
        label: "Unrendered Artifact".to_owned(),
        relative_path: path.to_owned(),
        requiredness_mode: if packet_required {
            RequirednessMode::Always
        } else {
            RequirednessMode::Optional
        },
        applicability: if packet_required {
            ArtifactApplicability::Required
        } else {
            ArtifactApplicability::Optional
        },
        renderer_definition_refs: Vec::new(),
        packet_required,
        baseline_required: false,
        setup_scaffolded: false,
        presence: ArtifactPresence::PresentNonEmpty,
        byte_len: Some(byte_len),
        content_sha256: Some(format!("source-{path}")),
        matches_setup_starter_template: false,
    }
}

#[test]
fn total_threshold_isolated_from_per_artifact_threshold_is_exact() {
    let artifact = selected_identity(5);
    let effective = [BudgetEffectiveBytes {
        canonical_repo_relative_path: SELECTED_PATH.to_owned(),
        byte_len: 10,
        byte_domain: BudgetByteDomain::RenderedOutput,
    }];
    let evaluate_total = |maximum| {
        evaluate_budget_with_effective_bytes(
            std::slice::from_ref(&artifact),
            &effective,
            BudgetPolicy {
                max_total_bytes: Some(maximum),
                max_per_artifact_bytes: None,
            },
        )
    };

    let above = evaluate_total(9);
    assert_eq!(above.disposition, BudgetDisposition::Refuse);
    assert_eq!(above.targets[0].byte_len, 10);
    assert_eq!(
        above.targets[0].byte_domain,
        BudgetByteDomain::RenderedOutput
    );
    assert_eq!(evaluate_total(10).disposition, BudgetDisposition::Keep);
    assert_eq!(evaluate_total(11).disposition, BudgetDisposition::Keep);
}

#[test]
fn per_artifact_threshold_isolated_from_total_threshold_is_exact() {
    let artifact = selected_identity(5);
    let effective = [BudgetEffectiveBytes {
        canonical_repo_relative_path: SELECTED_PATH.to_owned(),
        byte_len: 10,
        byte_domain: BudgetByteDomain::RenderedOutput,
    }];
    let evaluate_per_artifact = |maximum| {
        evaluate_budget_with_effective_bytes(
            std::slice::from_ref(&artifact),
            &effective,
            BudgetPolicy {
                max_total_bytes: None,
                max_per_artifact_bytes: Some(maximum),
            },
        )
    };

    assert_eq!(
        evaluate_per_artifact(9).disposition,
        BudgetDisposition::Refuse
    );
    assert_eq!(
        evaluate_per_artifact(10).disposition,
        BudgetDisposition::Keep
    );
    assert_eq!(
        evaluate_per_artifact(11).disposition,
        BudgetDisposition::Keep
    );
}

#[test]
fn total_budget_excludes_optional_targets_until_recomputed_total_fits() {
    let artifacts = [
        identity("required", 5, true),
        identity("optional-a", 4, false),
        identity("optional-b", 3, false),
    ];
    let outcome = evaluate_budget_with_effective_bytes(
        &artifacts,
        &[],
        BudgetPolicy {
            max_total_bytes: Some(6),
            max_per_artifact_bytes: None,
        },
    );

    assert_eq!(outcome.disposition, BudgetDisposition::Exclude);
    assert_eq!(
        outcome
            .targets
            .iter()
            .map(|target| target.canonical_repo_relative_path.as_str())
            .collect::<Vec<_>>(),
        ["optional-a", "optional-b"]
    );
}

#[test]
fn total_budget_refuses_when_required_remainder_exceeds_limit() {
    let artifacts = [
        identity("required-a", 5, true),
        identity("required-b", 6, true),
        identity("optional", 3, false),
    ];
    let outcome = evaluate_budget_with_effective_bytes(
        &artifacts,
        &[],
        BudgetPolicy {
            max_total_bytes: Some(10),
            max_per_artifact_bytes: None,
        },
    );

    assert_eq!(outcome.disposition, BudgetDisposition::Refuse);
    assert_eq!(outcome.targets.len(), 1);
    assert_eq!(
        outcome.targets[0].canonical_repo_relative_path,
        "required-b"
    );
}
