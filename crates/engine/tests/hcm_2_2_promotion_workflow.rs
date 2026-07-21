use handbook_engine::{
    normalize_required_approval_pairs, ApproverAuthorityPairV1, CharterPromotionCommitV1,
    CharterPromotionIntentV1, CharterPromotionWorkflowCommitV1,
    CharterPromotionWorkflowErrorKindV1, CharterPromotionWorkflowServiceV1,
    MAX_LINEAGE_REFERENCE_BYTES,
};
use std::path::PathBuf;

const PROMOTION_WORKFLOW_SOURCE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/charter_promotion_workflow.rs"
));

#[test]
fn required_approval_pairs_are_sorted_and_duplicates_refuse() {
    let pairs = normalize_required_approval_pairs(vec![
        ApproverAuthorityPairV1 {
            approval_class: "Security approval".to_owned(),
            authority_ref: "Security lead".to_owned(),
        },
        ApproverAuthorityPairV1 {
            approval_class: "Project owner approval".to_owned(),
            authority_ref: "Project owner".to_owned(),
        },
    ])
    .expect("normalized pairs");

    assert_eq!(pairs[0].approval_class, "Project owner approval");
    assert_eq!(pairs[1].approval_class, "Security approval");

    let duplicate = normalize_required_approval_pairs(vec![pairs[0].clone(), pairs[0].clone()])
        .expect_err("duplicate pair must refuse");
    assert_eq!(
        duplicate.kind(),
        CharterPromotionWorkflowErrorKindV1::ApprovalRefused
    );
}

#[test]
fn frozen_promotion_intent_uses_one_approval_anchor_and_no_caller_transaction_id() {
    let intent = CharterPromotionIntentV1 {
        candidate_ref: "candidates/candidate_aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.json".to_owned(),
        approval_ref: "approvals/approval_bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb.json".to_owned(),
        expected_current_fingerprint: None,
    };

    assert!(intent.approval_ref.starts_with("approvals/approval_"));
}

#[test]
fn promotion_intent_refuses_an_unbounded_singular_approval_anchor_before_io() {
    let repo = tempfile::tempdir().expect("repository");
    let error = CharterPromotionWorkflowServiceV1::new(repo.path())
        .promote(CharterPromotionIntentV1 {
            candidate_ref: "candidates/candidate_aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.json".to_owned(),
            approval_ref: "x".repeat(MAX_LINEAGE_REFERENCE_BYTES + 1),
            expected_current_fingerprint: None,
        })
        .expect_err("unbounded approval anchor must refuse");
    assert_eq!(
        error.kind(),
        CharterPromotionWorkflowErrorKindV1::InvalidIntent
    );
    assert!(!repo.path().join(".handbook").exists());
}

#[test]
fn production_promotion_has_one_retained_guard_and_no_release_then_reacquire_path() {
    let promote_at = PROMOTION_WORKFLOW_SOURCE
        .split("    fn promote_at(")
        .nth(1)
        .expect("private production promotion implementation")
        .split("    #[allow(clippy::too_many_arguments)]")
        .next()
        .expect("promotion implementation boundary");

    assert!(promote_at.contains("begin_retained_authority"));
    assert!(promote_at.contains("observe_committed_approver_registry_locked"));
    assert!(promote_at.contains("observe_committed_candidate_approval_refs_locked"));
    assert!(promote_at.contains("observe_retained_locked"));
    assert!(promote_at.contains("promote_retained"));
    assert!(!promote_at.contains("drop(registry)"));
    assert!(!promote_at.contains(".transactions\n            .promote("));
    assert!(!promote_at.contains("observe_committed_approver_registry(&self.repo_root)"));
}

fn promotion_commit(
    promotion_ref: &str,
    lifecycle_transition_ref: &str,
) -> CharterPromotionWorkflowCommitV1 {
    CharterPromotionWorkflowCommitV1 {
        canonical_fingerprint: format!("sha256:{}", "a".repeat(64)),
        promotion_ref: promotion_ref.to_owned(),
        lifecycle_transition_ref: lifecycle_transition_ref.to_owned(),
        approval_refs: Vec::new(),
        required_approval_pairs: Vec::new(),
        transaction: CharterPromotionCommitV1 {
            promotion_ref: promotion_ref.to_owned(),
            lifecycle_transition_ref: lifecycle_transition_ref.to_owned(),
            committed_marker_path: PathBuf::from(
                ".handbook/state/transactions/promotions/secret-transaction.committed/committed",
            ),
        },
    }
}

#[test]
fn committed_promotion_projects_exact_semantic_product_effects_without_transaction_identity() {
    let promotion_ref = format!("promotions/promotion_{}.json", "b".repeat(64));
    let lifecycle_ref = format!(
        "lifecycle-transitions/lifecycle-transition_{}.json",
        "c".repeat(64)
    );
    let projection = promotion_commit(&promotion_ref, &lifecycle_ref)
        .product_projection()
        .expect("internally validated commit projects");

    assert_eq!(projection.canonical_path, ".handbook/project/charter.yaml");
    assert_eq!(
        projection.canonical_fingerprint,
        format!("sha256:{}", "a".repeat(64))
    );
    assert_eq!(projection.promotion_ref, promotion_ref);
    assert_eq!(
        projection.promotion_fingerprint,
        format!("sha256:{}", "b".repeat(64))
    );
    assert_eq!(
        projection.changed_paths,
        vec![
            ".handbook/project/charter.yaml".to_owned(),
            format!(".handbook/state/{promotion_ref}"),
            format!(".handbook/state/{lifecycle_ref}"),
        ]
    );
    assert_eq!(projection.next_actions.len(), 1);
    assert!(!projection.next_actions[0].is_empty());
    assert!(!format!("{projection:?}").contains("secret-transaction"));
}

#[test]
fn promotion_product_projection_refuses_malformed_unexpected_or_inconsistent_refs() {
    let valid_promotion = format!("promotions/promotion_{}.json", "b".repeat(64));
    let valid_lifecycle = format!(
        "lifecycle-transitions/lifecycle-transition_{}.json",
        "c".repeat(64)
    );
    let malformed = promotion_commit("promotions/not-content-addressed.json", &valid_lifecycle)
        .product_projection()
        .expect_err("malformed promotion ref must fail closed");
    assert_eq!(
        malformed.detail(),
        "promotion ref is not an exact content-addressed promotion ref"
    );

    let unexpected = promotion_commit(
        &format!("approvals/promotion_{}.json", "b".repeat(64)),
        &valid_lifecycle,
    )
    .product_projection()
    .expect_err("unexpected promotion partition must fail closed");
    assert_eq!(unexpected.kind(), malformed.kind());

    let malformed_lifecycle = promotion_commit(
        &valid_promotion,
        &format!("lifecycle-transitions/transition_{}.json", "c".repeat(64)),
    )
    .product_projection()
    .expect_err("unexpected lifecycle transition identity must fail closed");
    assert_ne!(malformed_lifecycle.kind(), malformed.kind());

    let mut inconsistent = promotion_commit(&valid_promotion, &valid_lifecycle);
    inconsistent.transaction.promotion_ref =
        format!("promotions/promotion_{}.json", "d".repeat(64));
    let inconsistent = inconsistent
        .product_projection()
        .expect_err("top-level and transaction refs must agree");
    assert_ne!(inconsistent.kind(), malformed.kind());
}

#[test]
fn every_projection_invariant_error_has_one_engine_owned_product_refusal() {
    let valid_promotion = format!("promotions/promotion_{}.json", "b".repeat(64));
    let valid_lifecycle = format!(
        "lifecycle-transitions/lifecycle-transition_{}.json",
        "c".repeat(64)
    );
    let mut invalid_canonical = promotion_commit(&valid_promotion, &valid_lifecycle);
    invalid_canonical.canonical_fingerprint = "not-a-fingerprint".to_owned();
    let invalid_canonical = invalid_canonical
        .product_projection()
        .expect_err("invalid canonical fingerprint must refuse");
    let invalid_promotion =
        promotion_commit("promotions/not-content-addressed.json", &valid_lifecycle)
            .product_projection()
            .expect_err("invalid promotion ref must refuse");
    let invalid_lifecycle =
        promotion_commit(&valid_promotion, "lifecycle-transitions/not-addressed.json")
            .product_projection()
            .expect_err("invalid lifecycle transition ref must refuse");
    let mut inconsistent = promotion_commit(&valid_promotion, &valid_lifecycle);
    inconsistent.transaction.lifecycle_transition_ref = format!(
        "lifecycle-transitions/lifecycle-transition_{}.json",
        "d".repeat(64)
    );
    let inconsistent = inconsistent
        .product_projection()
        .expect_err("inconsistent commit refs must refuse");

    for (error, expected_code) in [
        (
            invalid_canonical,
            "promotion_projection_invalid_canonical_fingerprint",
        ),
        (
            invalid_promotion,
            "promotion_projection_invalid_promotion_ref",
        ),
        (
            invalid_lifecycle,
            "promotion_projection_invalid_lifecycle_transition_ref",
        ),
        (inconsistent, "promotion_projection_inconsistent_commit"),
    ] {
        let detail = error.detail().to_owned();
        let refusal = error.product_refusal();
        assert_eq!(refusal.code, expected_code);
        assert_eq!(refusal.message, detail);
        assert!(!refusal.retryable);
        assert_eq!(refusal.next_actions.len(), 1);
        assert!(!refusal.next_actions[0].is_empty());
    }
}

#[test]
fn every_workflow_error_kind_has_one_engine_owned_product_refusal_projection() {
    let cases = [
        (
            CharterPromotionWorkflowErrorKindV1::InvalidIntent,
            "invalid_intent",
            false,
        ),
        (
            CharterPromotionWorkflowErrorKindV1::CandidateRefused,
            "candidate_refused",
            false,
        ),
        (
            CharterPromotionWorkflowErrorKindV1::ApprovalRefused,
            "approval_refused",
            false,
        ),
        (
            CharterPromotionWorkflowErrorKindV1::RegistryRefused,
            "registry_refused",
            false,
        ),
        (
            CharterPromotionWorkflowErrorKindV1::DefinitionDrift,
            "definition_drift",
            false,
        ),
        (
            CharterPromotionWorkflowErrorKindV1::BasisMismatch,
            "basis_mismatch",
            true,
        ),
        (
            CharterPromotionWorkflowErrorKindV1::LifecycleClearanceIncomplete,
            "lifecycle_clearance_incomplete",
            false,
        ),
        (
            CharterPromotionWorkflowErrorKindV1::TransactionRefused,
            "transaction_refused",
            false,
        ),
    ];

    for (kind, code, retryable) in cases {
        let refusal = kind.product_refusal(format!("detail for {code}"));
        assert_eq!(refusal.code, code);
        assert_eq!(refusal.message, format!("detail for {code}"));
        assert_eq!(refusal.retryable, retryable);
        assert_eq!(refusal.next_actions.len(), 1);
        assert!(!refusal.next_actions[0].is_empty());
    }
}

#[test]
fn workflow_error_product_refusal_preserves_the_original_detail_message() {
    let repo = tempfile::tempdir().expect("repository");
    let error = CharterPromotionWorkflowServiceV1::new(repo.path())
        .promote(CharterPromotionIntentV1 {
            candidate_ref: String::new(),
            approval_ref: String::new(),
            expected_current_fingerprint: None,
        })
        .expect_err("empty refs must refuse");
    let detail = error.detail().to_owned();
    let refusal = error.product_refusal();

    assert_eq!(refusal.code, "invalid_intent");
    assert_eq!(refusal.message, detail);
    assert!(!refusal.retryable);
    assert_eq!(refusal.next_actions.len(), 1);
}
