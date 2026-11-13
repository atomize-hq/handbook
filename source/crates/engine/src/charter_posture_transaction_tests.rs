use crate::charter_authority_transaction::CharterAuthorityTransactionServiceV1;
use crate::charter_lifecycle_store::CharterLifecycleStoreV1;
use crate::charter_lifecycle_transition_v11::{
    construct_lifecycle_transition_v11, lifecycle_transition_output_record_v11,
    LifecycleTransitionDraftV11,
};
use crate::charter_posture_transaction_intent_v1::{
    construct_posture_transaction_intent_v1, construct_posture_transition_v1,
    parse_posture_transition_v1, posture_transaction_intent_marker_v1,
    posture_transition_marker_v1, AuthorityHeadV1, CanonicalDocumentV1, KernelReplayDimensionV1,
    KernelReplayV1, PairV1, PostureChangeV1, PostureReassessmentV1, PostureTransitionDraftV1,
};
use crate::project_posture::{
    bind_exact_canonical_bytes, derive_project_posture_kernel, fixed_dimension_binding,
    prepare_posture_change, PostureChangeRequest,
};
use crate::DefinitionFingerprint;
use std::fs;

const DIMENSION_IDS: [&str; 9] = [
    "speed_vs_quality",
    "type_safety_static_analysis",
    "testing_rigor",
    "scalability_performance",
    "reliability_operability",
    "security_privacy",
    "observability",
    "dx_tooling_automation",
    "ux_polish_api_usability",
];

fn fingerprint(fill: char) -> String {
    format!("sha256:{}", fill.to_string().repeat(64))
}

fn lifecycle_state_fingerprint(canonical_fingerprint: &str) -> String {
    crate::charter_lifecycle_state_fingerprint(
        "handbook.lifecycle.constitutional-review-lock@1.0.0",
        "sha256:88caafb9caaf137647c42a91cd2762ac0871e0a20e2a1844c2c0076d5fb43cc3",
        "project_authority",
        canonical_fingerprint,
        crate::CharterLifecycleState::Current,
        &[],
    )
    .unwrap()
}

fn pair(reference: &str, fill: char) -> PairV1 {
    PairV1 {
        reference: reference.to_owned(),
        fingerprint: fingerprint(fill),
    }
}

fn canonical(bytes: &[u8]) -> CanonicalDocumentV1 {
    let fingerprint = DefinitionFingerprint::from_bytes(bytes).to_string();
    CanonicalDocumentV1 {
        reference: ".handbook/project/charter.yaml".to_owned(),
        fingerprint: fingerprint.clone(),
        document_sha256: fingerprint,
        byte_length: bytes.len() as u64,
    }
}

fn authority_head(old: &CanonicalDocumentV1) -> AuthorityHeadV1 {
    AuthorityHeadV1 {
        kind: "promotion".to_owned(),
        source: pair(
            &format!("promotions/promotion_{}.json", "a".repeat(64)),
            'a',
        ),
        canonical: old.clone(),
        lifecycle_transition: pair(
            &format!(
                "lifecycle-transitions/lifecycle-transition_{}.json",
                "b".repeat(64)
            ),
            'b',
        ),
        promotion_ancestor: pair(
            &format!("promotions/promotion_{}.json", "c".repeat(64)),
            'c',
        ),
    }
}

fn change() -> PostureChangeV1 {
    PostureChangeV1 {
        dimension_id: "testing_rigor".to_owned(),
        dimension_index: 2,
        authority_path: "/engineering_posture/dimensions/2/level_override".to_owned(),
        operation: "replace".to_owned(),
        baseline_level: 3,
        expected_stored_value: Some(4),
        expected_effective_level: 4,
        proposed_stored_value: None,
        proposed_effective_level: 3,
    }
}

fn reassessment() -> PostureReassessmentV1 {
    PostureReassessmentV1 {
        intake_definition: pair(
            &format!("intake-records/intake_{}.json", "a".repeat(64)),
            'a',
        ),
        affected_coverage_ids: vec!["engineering_posture.dimensions".to_owned()],
        validation_result_inputs: vec![pair("validation-results/example", 'd')],
    }
}

fn kernel_replay(old: &CanonicalDocumentV1, new: &CanonicalDocumentV1) -> KernelReplayV1 {
    KernelReplayV1 {
        constitutional_artifact_ref: ".handbook/project/charter.yaml".to_owned(),
        source_authority_fingerprint: old.fingerprint.clone(),
        resulting_authority_fingerprint: new.fingerprint.clone(),
        source_input_fingerprint: fingerprint('e'),
        resulting_input_fingerprint: fingerprint('f'),
        profile_input: pair("profiles/project", '1'),
        override_inputs: Vec::new(),
        condition_inputs: Vec::new(),
        contract_inputs: Vec::new(),
        evidence_inputs: Vec::new(),
        snapshot_inputs: Vec::new(),
        freshness_basis: None,
        dimensions: DIMENSION_IDS
            .iter()
            .enumerate()
            .map(|(index, dimension_id)| KernelReplayDimensionV1 {
                dimension_id: (*dimension_id).to_owned(),
                source_effective_level: if index == 2 { 4 } else { 3 },
                resulting_effective_level: 3,
                floor: 1,
                red_line_refs: Vec::new(),
                trigger_refs: Vec::new(),
                allowed_shortcut_refs: Vec::new(),
                proof_obligation_refs: Vec::new(),
            })
            .collect(),
        applicable_scope_refs: Vec::new(),
        omitted_condition_refs: Vec::new(),
        unresolved_condition_refs: Vec::new(),
    }
}

fn posture_draft(old: &CanonicalDocumentV1, new: &CanonicalDocumentV1) -> PostureTransitionDraftV1 {
    PostureTransitionDraftV1 {
        recommendation: pair("recommendations/example", '2'),
        repository_identity: pair(".handbook/repository-identity.v1", '1'),
        source_kernel: pair("project-posture-kernels/example", '3'),
        evaluation_policy: pair("posture-policies/example", '4'),
        prior_authority_head: authority_head(old),
        expected_canonical: old.clone(),
        change: change(),
        approval_inputs: vec![pair("approvals/example", '5')],
        authorized_by_ref: "actors/constitutional-reviewer".to_owned(),
        reassessment: reassessment(),
        kernel_replay: kernel_replay(old, new),
        resulting_canonical: new.clone(),
        resulting_kernel: pair("project-posture-kernels/result", '6'),
        effective_at_utc: "2026-08-08T14:40:00Z".to_owned(),
    }
}

fn record_bundle(
    old_bytes: &[u8],
    new_bytes: &[u8],
) -> (
    crate::charter_posture_transaction_intent_v1::ValidatedPostureTransitionV1,
    crate::charter_lifecycle_transition_v11::ValidatedLifecycleTransitionV11,
    crate::charter_posture_transaction_intent_v1::ValidatedPostureTransactionIntentV1,
) {
    let posture = construct_posture_transition_v1(posture_draft(
        &canonical(old_bytes),
        &canonical(new_bytes),
    ))
    .unwrap();
    let lifecycle = construct_lifecycle_transition_v11(
        &posture,
        LifecycleTransitionDraftV11 {
            prior_transition: posture
                .record
                .prior_authority_head
                .lifecycle_transition
                .clone(),
            prior_state_fingerprint: lifecycle_state_fingerprint(
                &posture.record.expected_canonical.fingerprint,
            ),
        },
    )
    .unwrap();
    let intent =
        construct_posture_transaction_intent_v1(&posture, &lifecycle, old_bytes, new_bytes, |_| {
            true
        })
        .unwrap();
    (posture, lifecycle, intent)
}

fn live_record_bundle(
    repo: &std::path::Path,
    basis_head: AuthorityHeadV1,
) -> (
    crate::charter_posture_transaction_intent_v1::ValidatedPostureTransitionV1,
    crate::charter_lifecycle_transition_v11::ValidatedLifecycleTransitionV11,
    crate::charter_posture_transaction_intent_v1::ValidatedPostureTransactionIntentV1,
    Vec<u8>,
    Vec<u8>,
) {
    let old = fs::read(repo.join(".handbook/project/charter.yaml")).unwrap();
    let decisions = crate::resolve_shipped_profile_decisions(repo).unwrap();
    let source = crate::parse_canonical_charter(&decisions, &old).unwrap();
    let source_exact = bind_exact_canonical_bytes(&old).unwrap();
    let binding = fixed_dimension_binding("testing_rigor").unwrap();
    let expected_stored = source.engineering_posture.dimensions[binding.index].level_override;
    let expected_effective = expected_stored.unwrap_or(source.posture.baseline_level);
    let change = prepare_posture_change(
        &source,
        source_exact.clone(),
        PostureChangeRequest {
            dimension_id: binding.dimension_id.to_owned(),
            dimension_index: binding.index as u8,
            authority_path: binding.authority_path.to_owned(),
            expected_stored_value: expected_stored,
            expected_effective_level: expected_effective,
            proposed_effective_level: 3,
        },
    )
    .unwrap();
    let new = crate::serialize_canonical_charter(&decisions, &change.resulting_charter).unwrap();
    let source_kernel = derive_project_posture_kernel(&source, &source_exact).unwrap();
    let resulting_exact = bind_exact_canonical_bytes(&new).unwrap();
    let resulting_kernel =
        derive_project_posture_kernel(&change.resulting_charter, &resulting_exact).unwrap();
    let old_document = canonical(&old);
    let new_document = canonical(&new);
    let kernel_replay = KernelReplayV1 {
        constitutional_artifact_ref: ".handbook/project/charter.yaml".to_owned(),
        source_authority_fingerprint: old_document.fingerprint.clone(),
        resulting_authority_fingerprint: new_document.fingerprint.clone(),
        source_input_fingerprint: source_kernel.input_fingerprint.clone(),
        resulting_input_fingerprint: resulting_kernel.input_fingerprint.clone(),
        profile_input: pair("profiles/project", '1'),
        override_inputs: Vec::new(),
        condition_inputs: Vec::new(),
        contract_inputs: Vec::new(),
        evidence_inputs: Vec::new(),
        snapshot_inputs: Vec::new(),
        freshness_basis: None,
        dimensions: source_kernel
            .dimensions
            .iter()
            .zip(&resulting_kernel.dimensions)
            .map(|(source, resulting)| KernelReplayDimensionV1 {
                dimension_id: source.dimension_id.clone(),
                source_effective_level: source.effective_level,
                resulting_effective_level: resulting.effective_level,
                floor: 1,
                red_line_refs: source.red_line_refs.clone(),
                trigger_refs: source.trigger_refs.clone(),
                allowed_shortcut_refs: source.allowed_shortcut_refs.clone(),
                proof_obligation_refs: Vec::new(),
            })
            .collect(),
        applicable_scope_refs: Vec::new(),
        omitted_condition_refs: Vec::new(),
        unresolved_condition_refs: Vec::new(),
    };
    let posture = construct_posture_transition_v1(PostureTransitionDraftV1 {
        recommendation: pair("recommendations/example", '2'),
        repository_identity: pair(".handbook/repository-identity.v1", '1'),
        source_kernel: PairV1 {
            reference: "project-posture-kernels/source".to_owned(),
            fingerprint: source_kernel.kernel_fingerprint,
        },
        evaluation_policy: pair("posture-policies/example", '4'),
        prior_authority_head: basis_head,
        expected_canonical: old_document,
        change: PostureChangeV1 {
            dimension_id: change.change.dimension_id.clone(),
            dimension_index: change.change.dimension_index,
            authority_path: change.change.authority_path.clone(),
            operation: "replace".to_owned(),
            baseline_level: change.change.baseline_level,
            expected_stored_value: change.change.expected_stored_value,
            expected_effective_level: change.change.expected_effective_level,
            proposed_stored_value: change.change.proposed_stored_value,
            proposed_effective_level: change.change.proposed_effective_level,
        },
        approval_inputs: vec![pair("approvals/example", '5')],
        authorized_by_ref: "actors/constitutional-reviewer".to_owned(),
        reassessment: reassessment(),
        kernel_replay,
        resulting_canonical: new_document,
        resulting_kernel: PairV1 {
            reference: "project-posture-kernels/result".to_owned(),
            fingerprint: resulting_kernel.kernel_fingerprint,
        },
        effective_at_utc: "2026-08-08T14:40:00Z".to_owned(),
    })
    .unwrap();
    let lifecycle_authority = CharterLifecycleStoreV1::new(repo)
        .observe()
        .unwrap()
        .unwrap();
    let lifecycle = construct_lifecycle_transition_v11(
        &posture,
        LifecycleTransitionDraftV11 {
            prior_transition: posture
                .record
                .prior_authority_head
                .lifecycle_transition
                .clone(),
            prior_state_fingerprint: lifecycle_authority.state_fingerprint,
        },
    )
    .unwrap();
    let intent =
        construct_posture_transaction_intent_v1(&posture, &lifecycle, &old, &new, |_| true)
            .unwrap();
    (posture, lifecycle, intent, old, new)
}

#[test]
fn posture_constructor_emits_exact_jcs_lf_content_identity_and_marker() {
    let old = canonical(b"old charter\n");
    let new = canonical(b"new charter\n");

    let posture = construct_posture_transition_v1(posture_draft(&old, &new)).unwrap();

    assert!(posture.raw_bytes.ends_with(b"\n"));
    assert_eq!(posture_transition_marker_v1(&posture.raw_bytes).len(), 72);
    assert_eq!(
        parse_posture_transition_v1(&posture.raw_bytes).unwrap(),
        posture
    );
    assert_eq!(
        posture.record.transition_id,
        format!(
            "posture-transition_{}",
            posture
                .record
                .transition_fingerprint
                .strip_prefix("sha256:")
                .unwrap()
        )
    );
}

#[test]
fn lifecycle_constructor_binds_the_completed_posture_without_a_back_edge() {
    let old = canonical(b"old charter\n");
    let new = canonical(b"new charter\n");
    let posture = construct_posture_transition_v1(posture_draft(&old, &new)).unwrap();
    let lifecycle = construct_lifecycle_transition_v11(
        &posture,
        LifecycleTransitionDraftV11 {
            prior_transition: pair(
                &format!(
                    "lifecycle-transitions/lifecycle-transition_{}.json",
                    "7".repeat(64)
                ),
                '7',
            ),
            prior_state_fingerprint: lifecycle_state_fingerprint(
                &posture.record.expected_canonical.fingerprint,
            ),
        },
    )
    .unwrap();

    assert_eq!(
        lifecycle.record.authority_transition,
        crate::charter_posture_transaction_intent_v1::posture_transition_pair_v1(&posture)
    );
    assert_eq!(
        lifecycle_transition_output_record_v11(&lifecycle).fingerprint,
        lifecycle.record.transition_fingerprint
    );
    assert_eq!(
        lifecycle.record.transitioned_at_utc,
        posture.record.effective_at_utc
    );
    assert_eq!(
        lifecycle.record.prior_state_fingerprint,
        lifecycle_state_fingerprint(&posture.record.expected_canonical.fingerprint)
    );
    assert_eq!(
        lifecycle.record.result_state_fingerprint,
        lifecycle_state_fingerprint(&posture.record.resulting_canonical.fingerprint)
    );
    assert_ne!(
        lifecycle.record.prior_state_fingerprint, lifecycle.record.result_state_fingerprint,
        "a posture rebase must retain distinct old and resulting lifecycle-current states"
    );
}

#[test]
fn intent_constructor_binds_exact_output_bytes_and_closed_parsers_refuse_substitution() {
    let old_bytes = b"old charter\n";
    let new_bytes = b"new charter\n";
    let posture = construct_posture_transition_v1(posture_draft(
        &canonical(old_bytes),
        &canonical(new_bytes),
    ))
    .unwrap();
    let lifecycle = construct_lifecycle_transition_v11(
        &posture,
        LifecycleTransitionDraftV11 {
            prior_transition: pair(
                &format!(
                    "lifecycle-transitions/lifecycle-transition_{}.json",
                    "9".repeat(64)
                ),
                '9',
            ),
            prior_state_fingerprint: lifecycle_state_fingerprint(
                &posture.record.expected_canonical.fingerprint,
            ),
        },
    )
    .unwrap();
    let intent =
        construct_posture_transaction_intent_v1(&posture, &lifecycle, old_bytes, new_bytes, |_| {
            true
        })
        .unwrap();

    crate::charter_posture_transaction_intent_v1::validate_posture_transaction_intent_bindings_v1(
        &intent, &posture, &lifecycle, old_bytes, new_bytes,
    )
    .unwrap();
    assert!(
        crate::charter_posture_transaction_intent_v1::validate_posture_transaction_intent_bindings_v1(
            &intent,
            &posture,
            &lifecycle,
            b"substituted old charter\n",
            new_bytes,
        )
        .is_err()
    );

    let mut unexpected = serde_json::to_value(&posture.record).unwrap();
    unexpected["unknown"] = serde_json::Value::String("refuse".to_owned());
    let mut unexpected_bytes = serde_json_canonicalizer::to_vec(&unexpected).unwrap();
    unexpected_bytes.push(b'\n');
    assert!(parse_posture_transition_v1(&unexpected_bytes).is_err());

    let non_jcs = b"{\"schema_id\": \"handbook.posture-transition\"}\n";
    assert!(parse_posture_transition_v1(non_jcs).is_err());
}

#[test]
fn posture_pending_basis_prefix_rolls_back_to_an_exact_terminal() {
    let repo = tempfile::tempdir().unwrap();
    let old = b"old charter\n";
    let new = b"new charter\n";
    let (_, _, intent) = record_bundle(old, new);
    let canonical = repo.path().join(".handbook/project/charter.yaml");
    fs::create_dir_all(canonical.parent().unwrap()).unwrap();
    fs::write(&canonical, old).unwrap();
    let pending = repo.path().join(format!(
        ".handbook/state/transactions/posture-transitions/{}.pending",
        intent.record.transaction_id
    ));
    fs::create_dir_all(&pending).unwrap();
    fs::write(pending.join("intent.json"), &intent.raw_bytes).unwrap();
    fs::write(pending.join("canonical.old"), old).unwrap();

    CharterAuthorityTransactionServiceV1::new(repo.path())
        .recover_posture_pending_for_testing(&pending)
        .expect("basis-prefix posture journal must roll back exactly");

    let terminal = repo.path().join(format!(
        ".handbook/state/transactions/posture-transitions/{}.rolled-back",
        intent.record.transaction_id
    ));
    assert!(!pending.exists());
    assert!(terminal.join("rolled-back").is_file());
    assert_eq!(
        fs::read_dir(&terminal)
            .unwrap()
            .map(|entry| entry.unwrap().file_name().into_string().unwrap())
            .collect::<std::collections::BTreeSet<_>>(),
        std::collections::BTreeSet::from([
            "canonical.old".to_owned(),
            "intent.json".to_owned(),
            "rolled-back".to_owned(),
        ])
    );
}

#[test]
fn unknown_posture_pending_evidence_is_preserved_and_refused() {
    let repo = tempfile::tempdir().unwrap();
    let (_, _, intent) = record_bundle(b"old charter\n", b"new charter\n");
    let pending = repo.path().join(format!(
        ".handbook/state/transactions/posture-transitions/{}.pending",
        intent.record.transaction_id
    ));
    fs::create_dir_all(&pending).unwrap();
    fs::write(pending.join("intent.json"), &intent.raw_bytes).unwrap();
    fs::write(pending.join("unknown-evidence"), b"preserve\n").unwrap();

    assert!(CharterAuthorityTransactionServiceV1::new(repo.path())
        .recover_posture_pending_for_testing(&pending)
        .is_err());
    assert!(pending.join("intent.json").is_file());
    assert_eq!(
        fs::read(pending.join("unknown-evidence")).unwrap(),
        b"preserve\n"
    );
}

#[test]
fn posture_preflight_refuses_without_creating_a_pending_journal() {
    let repo = tempfile::tempdir().unwrap();
    let (posture, lifecycle, intent) = record_bundle(b"old charter\n", b"new charter\n");

    assert!(CharterAuthorityTransactionServiceV1::new(repo.path())
        .apply_posture_transition(&posture, &lifecycle, &intent)
        .is_err());
    assert!(!repo
        .path()
        .join(".handbook/state/transactions/posture-transitions")
        .exists());
}

#[test]
fn posture_preflight_refuses_synthetic_authority_closure_without_journal_or_canonical_mutation() {
    let repo = tempfile::tempdir().unwrap();
    let service = CharterAuthorityTransactionServiceV1::new(repo.path());
    let basis_head = service.bootstrap_posture_genesis_for_testing().unwrap();
    let (posture, lifecycle, intent, old, _new) = live_record_bundle(repo.path(), basis_head);

    assert!(service
        .apply_posture_transition(&posture, &lifecycle, &intent)
        .is_err());
    assert_eq!(
        fs::read(repo.path().join(".handbook/project/charter.yaml")).unwrap(),
        old
    );
    assert!(!repo
        .path()
        .join(".handbook/state/transactions/posture-transitions")
        .exists());
}

#[test]
fn posture_preflight_refuses_an_occupied_private_output_before_creating_a_journal() {
    let repo = tempfile::tempdir().unwrap();
    let service = CharterAuthorityTransactionServiceV1::new(repo.path());
    let basis_head = service.bootstrap_posture_genesis_for_testing().unwrap();
    let (posture, lifecycle, intent, old, _new) = live_record_bundle(repo.path(), basis_head);
    let occupied = repo
        .path()
        .join(".handbook/state")
        .join(&intent.record.outputs.posture_transition.reference);
    fs::create_dir_all(occupied.parent().unwrap()).unwrap();
    fs::write(&occupied, b"pre-existing private output\n").unwrap();

    assert!(service
        .apply_posture_transition(&posture, &lifecycle, &intent)
        .is_err());
    assert_eq!(
        fs::read(repo.path().join(".handbook/project/charter.yaml")).unwrap(),
        old
    );
    assert!(!repo
        .path()
        .join(".handbook/state/transactions/posture-transitions")
        .exists());
}

#[test]
fn posture_pending_synthetic_result_is_preserved_and_refused() {
    let repo = tempfile::tempdir().unwrap();
    let service = CharterAuthorityTransactionServiceV1::new(repo.path());
    let basis_head = service.bootstrap_posture_genesis_for_testing().unwrap();
    let (posture, lifecycle, intent, old, new) = live_record_bundle(repo.path(), basis_head);
    let pending = repo.path().join(format!(
        ".handbook/state/transactions/posture-transitions/{}.pending",
        intent.record.transaction_id
    ));
    fs::create_dir_all(&pending).unwrap();
    fs::write(repo.path().join(".handbook/project/charter.yaml"), &new).unwrap();
    fs::write(pending.join("intent.json"), &intent.raw_bytes).unwrap();
    fs::write(pending.join("canonical.old"), &old).unwrap();
    fs::write(
        pending.join("prepared"),
        posture_transaction_intent_marker_v1(&intent.raw_bytes),
    )
    .unwrap();
    fs::write(pending.join("posture-transition.new"), &posture.raw_bytes).unwrap();
    fs::write(
        pending.join("lifecycle-transition.new"),
        &lifecycle.raw_bytes,
    )
    .unwrap();

    assert!(service
        .recover_posture_pending_for_testing(&pending)
        .is_err());

    let terminal = repo.path().join(format!(
        ".handbook/state/transactions/posture-transitions/{}.committed",
        intent.record.transaction_id
    ));
    assert!(pending.exists());
    assert!(!terminal.exists());
}
