mod local_contract_support;

use handbook_contracts::*;
use handbook_dock::*;
use handbook_engine::local_authority::local_check::*;
use handbook_sdk::local_setup::{LocalAuthorityDispositionV1, LocalRevokeRequestV1};
use handbook_sdk::HandbookSdkV1;
use local_contract_support::*;

struct CountingSource<S> {
    inner: S,
    calls: usize,
}

impl<S: EvidenceCandidateSourceV1> EvidenceCandidateSourceV1 for CountingSource<S> {
    fn resolve_exact<R>(
        &mut self,
        candidate: &ContentAddressedBindingV1,
        consume: impl for<'authority> FnOnce(ResolvedEvidenceCandidateObservationV1<'authority>) -> R,
    ) -> Result<R, EvidenceCandidateSourceErrorV1> {
        self.calls += 1;
        self.inner.resolve_exact(candidate, consume)
    }
}

#[test]
fn captured_question_binds_payload_and_complete_offline_schema_closure() {
    let manifest = production_manifest();
    let role = &manifest.supported_inputs()[0];
    let content = supplied_validation_content();
    let capture = DockInlineJsonInputV1::new(
        role.role().clone(),
        role.media_type(),
        role.schema().clone(),
        content.clone(),
    )
    .unwrap();
    let subject = capture.captured_evaluation_subject_v1().unwrap();
    let case = capture
        .captured_evaluation_case_v1(StableIdV1::parse("case.order-conformance").unwrap())
        .unwrap();
    assert_eq!(
        subject.subject().fingerprint(),
        case.case_binding().fingerprint()
    );
    assert_ne!(
        subject.subject().fingerprint(),
        capture.content_fingerprint()
    );
    assert_ne!(
        capture.content_fingerprint().as_str(),
        content["instance"]["instance_fingerprint"]
            .as_str()
            .unwrap()
    );
    assert_ne!(
        capture.content_fingerprint().as_str(),
        content["closure_fingerprint"].as_str().unwrap()
    );
    assert_eq!(
        subject
            .matcher_inputs()
            .iter()
            .map(|v| (v.field().as_str(), v.value()))
            .collect::<Vec<_>>(),
        vec![
            ("dock.input.media-type", "application/json"),
            ("dock.input.role", "handbook.input.json-schema-validation"),
            (
                "dock.input.schema",
                "handbook.dock-json-schema-validation-input@1.0.0"
            ),
        ]
    );
    let mut changed_schema = content.clone();
    changed_schema["documents"][0]["schema"]["properties"]["amount"]["minimum"] =
        serde_json::json!(1);
    let changed = DockInlineJsonInputV1::new(
        role.role().clone(),
        role.media_type(),
        role.schema().clone(),
        changed_schema,
    )
    .unwrap();
    assert_ne!(
        changed.captured_evaluation_subject_v1().unwrap().subject(),
        subject.subject()
    );
    let mut changed_payload = content;
    changed_payload["instance"]["value"]["amount"] = serde_json::json!(126);
    let changed = DockInlineJsonInputV1::new(
        role.role().clone(),
        role.media_type(),
        role.schema().clone(),
        changed_payload,
    )
    .unwrap();
    assert_ne!(
        changed
            .captured_evaluation_case_v1(case.case_id().clone())
            .unwrap()
            .case_binding(),
        case.case_binding()
    );
    let changed_schema_identity = ExactBindingV1::new(
        role.schema().reference().clone(),
        FingerprintV1::parse(&value_fingerprint(&serde_json::json!(
            "different exact capture schema"
        )))
        .unwrap(),
    );
    let changed = DockInlineJsonInputV1::new(
        role.role().clone(),
        role.media_type(),
        changed_schema_identity,
        capture.content().clone(),
    )
    .unwrap();
    assert_eq!(changed.content_fingerprint(), capture.content_fingerprint());
    assert_ne!(
        changed.captured_evaluation_subject_v1().unwrap().subject(),
        subject.subject()
    );
}

#[test]
fn successor_requires_exact_supplied_approval_and_preserves_original_history() {
    let directory = tempfile::tempdir().unwrap();
    let sdk = HandbookSdkV1::open(directory.path());
    let initial = establish_initial(&sdk);
    let original = sdk
        .resolve_local_initial_setup_request(&initial)
        .unwrap()
        .terms
        .unwrap()
        .outcome
        .unwrap();
    let mut terms = supplied_successor_terms(&initial, &original.outcome_fingerprint);
    terms.human_approval_asserted = false;
    let relation = std::fs::read(
        directory
            .path()
            .join(".handbook/admission-governance/v2/scope-current.json"),
    )
    .unwrap();
    let basis = NativeLocalCheckAuthorityV1::observe_basis(
        directory.path(),
        handbook_engine::DefinitionFingerprint::from_bytes(&relation).to_string(),
    )
    .unwrap();
    assert!(
        NativeLocalCheckAuthorityV1::prepare(directory.path(), &initial.terms, terms, basis)
            .is_err()
    );
    assert!(!directory.path().join(LOCAL_CHECK_ROOT_V1).exists());
    let (locator, outcome) = adopt_check(&sdk, directory.path(), &initial);
    assert_eq!(outcome.assurance, "local_advisory");
    let reopened = HandbookSdkV1::open(directory.path());
    let restored =
        LocalCheckLocatorV1::from_json_bytes(&locator.canonical_json_bytes().unwrap()).unwrap();
    let history = NativeLocalCheckAuthorityV1::resolve_request(directory.path(), &restored);
    assert_eq!(history.disposition, LocalAuthorityDispositionV1::Replayed);
    assert_eq!(
        history.outcome.unwrap().outcome_fingerprint,
        outcome.outcome_fingerprint
    );
    assert!(reopened
        .resolve_current_local_initial_setup(&initial)
        .is_err());
    assert_eq!(
        reopened
            .resolve_local_initial_setup_request(&initial)
            .unwrap()
            .terms
            .unwrap()
            .outcome
            .unwrap()
            .outcome_fingerprint,
        original.outcome_fingerprint
    );
    let replay = NativeLocalCheckAuthorityV1::resume_request(directory.path(), &restored);
    assert_eq!(replay.disposition, LocalAuthorityDispositionV1::Replayed);
    assert_eq!(
        replay.outcome.unwrap().outcome_fingerprint,
        outcome.outcome_fingerprint
    );
}

#[test]
fn revoked_successor_history_never_restores_current_permission() {
    let directory = tempfile::tempdir().unwrap();
    let sdk = HandbookSdkV1::open(directory.path());
    let initial = establish_initial(&sdk);
    let (locator, outcome) = adopt_check(&sdk, directory.path(), &initial);
    NativeLocalCheckAuthorityV1::revoke(
        directory.path(),
        &locator,
        &LocalRevokeRequestV1 {
            request_key: "packet2-successor-revoke".into(),
            expected_current_fingerprint: outcome.outcome_fingerprint.clone(),
            principal: "packet2.human".into(),
            source_ref: "decision.packet2.revoke".into(),
            instruction:
                "Revoke this exact local-check authority after inspecting its retained source."
                    .into(),
        },
    )
    .unwrap();
    let action = LocalCheckActionRequestV1 {
        action: LocalCheckActionV1::DefinitionAuthor,
        action_ref: "contract.definition.local.append@2.0.0".into(),
        subject_ref: "handbook.contract.order-capture@1.0.0".into(),
        content_fingerprint: outcome.outcome_fingerprint.clone(),
        closure_fingerprint: outcome.outcome_fingerprint.clone(),
        actor_ref: "packet2.author".into(),
        participant_ref: "packet2.author".into(),
        semantic_time_utc: "2026-10-05T12:00:00Z".into(),
        owner_write_paths: vec![".handbook/state/contracts/local-v2/requests".into()],
        input_paths: vec![],
    };
    assert!(NativeLocalCheckAuthorityV1::with_admission(
        directory.path(),
        &locator,
        &locator.adoption_basis,
        &action,
        |_| Ok(())
    )
    .is_err());
    let replay = NativeLocalCheckAuthorityV1::resume_request(directory.path(), &locator);
    assert_eq!(replay.disposition, LocalAuthorityDispositionV1::Replayed);
    assert_eq!(
        replay.outcome.unwrap().outcome_fingerprint,
        outcome.outcome_fingerprint
    );
    assert!(NativeLocalCheckAuthorityV1::with_admission(
        directory.path(),
        &locator,
        &locator.adoption_basis,
        &action,
        |_| Ok(())
    )
    .is_err());
}

#[test]
fn native_owner_directory_write_reach_requires_approved_subtree() {
    for (selector, permitted) in [
        (".handbook/state/contracts/v2", false),
        (".handbook/state/contracts/v2/definitions/**", false),
        (".handbook/state/contracts/v2/**", true),
        (".handbook/state/contracts/**", true),
    ] {
        let directory = tempfile::tempdir().unwrap();
        let sdk = HandbookSdkV1::open(directory.path());
        let initial = establish_initial(&sdk);
        let original = sdk
            .resolve_local_initial_setup_request(&initial)
            .unwrap()
            .terms
            .unwrap()
            .outcome
            .unwrap();
        let mut terms = supplied_successor_terms(&initial, &original.outcome_fingerprint);
        terms.allowed_paths = vec![
            ".handbook/state/contracts/v1/transactions/**".into(),
            selector.into(),
        ];
        terms.allowed_paths.sort();
        terms.approved_content_fingerprint = terms.fingerprint_approved_content().unwrap();
        let prepared = sdk.prepare_local_check_terms(&initial, terms).unwrap();
        let locator = prepared.locator().clone();
        let adopted = sdk.adopt_local_check_terms(prepared);
        assert_eq!(adopted.disposition, LocalAuthorityDispositionV1::Committed);
        let authority = adopted.outcome.unwrap();
        let (definition, stack) = supplied_definition(
            LocalPermittedUseV2::DecisionSeekingEligible,
            120,
            author_authority(&authority),
        );
        let owner = ContractOwnerRepositoryV1::open(directory.path()).unwrap();
        let prepared = owner
            .prepare_local_definition_append_v2(&project(&initial), definition, &stack)
            .unwrap();
        let request = prepared.authority_request(
            "packet2.author".into(),
            "packet2.author".into(),
            SemanticTimeV1::owner_observation_now_v1().unwrap(),
        );
        assert!(request
            .owner_write_paths
            .iter()
            .any(|path| path == ".handbook/state/contracts/v2"));
        let mut consuming_effect_entered = false;
        let result = NativeLocalCheckAuthorityV1::with_admission(
            directory.path(),
            &locator,
            prepared.current_basis(),
            &request,
            |_| {
                consuming_effect_entered = true;
                Ok(())
            },
        );
        if permitted {
            result.unwrap();
            assert!(consuming_effect_entered);
        } else {
            assert_eq!(result.unwrap_err().code, "path_expansion");
            assert!(!consuming_effect_entered);
        }
    }
}

#[test]
fn native_owner_consumers_refuse_omitted_mutation_roots_without_output_effects() {
    let directory = tempfile::tempdir().unwrap();
    let root = directory.path();
    let sdk = HandbookSdkV1::open(root);
    let initial = establish_initial(&sdk);
    let (locator, authority) = adopt_check(&sdk, root, &initial);
    let (definition, stack) = supplied_definition(
        LocalPermittedUseV2::DecisionSeekingEligible,
        120,
        author_authority(&authority),
    );
    let owner = ContractOwnerRepositoryV1::open(root).unwrap();
    for missing in ["transactions", "v2", "both"] {
        let prepared = owner
            .prepare_local_definition_append_v2(&project(&initial), definition.clone(), &stack)
            .unwrap();
        let basis = prepared.current_basis().clone();
        let mut request = prepared.authority_request(
            "packet2.author".into(),
            "packet2.author".into(),
            SemanticTimeV1::owner_observation_now_v1().unwrap(),
        );
        omit_local_owner_root(&mut request, missing);
        let before = native_owner_file_fingerprints(root);
        let mut admission_entered = false;
        let result = NativeLocalCheckAuthorityV1::with_admission(
            root,
            &locator,
            &basis,
            &request,
            |admission| {
                admission_entered = true;
                Ok(owner.append_local_definition_v2(
                    prepared,
                    admission,
                    &format!("packet2-omitted-author-{missing}"),
                ))
            },
        )
        .unwrap();
        assert!(admission_entered);
        assert_eq!(
            result.unwrap_err().kind(),
            ContractOwnerRepositoryErrorKindV1::InvalidRequest
        );
        assert_eq!(native_owner_file_fingerprints(root), before);
        assert!(owner
            .read_local_definition_v2(&definition.binding())
            .is_err());
    }
    let authored = append_definition(
        root,
        &owner,
        &initial,
        &locator,
        definition.clone(),
        &stack,
        "packet2-genuine-root-bound-author",
    );
    assert_eq!(authored.disposition, OwnerMutationDispositionV1::Committed);
    for missing in ["transactions", "v2", "both"] {
        let head = owner
            .current_local_lifecycle_head_v2(&definition.binding())
            .unwrap();
        let prepared = owner
            .prepare_local_lifecycle_transition_v2(
                &project(&initial),
                &definition.binding(),
                LocalLifecycleTransitionInputV2 {
                    expected_lifecycle_fingerprint: head.lifecycle_fingerprint().clone(),
                    to_state: LifecycleStateV1::ReviewReady,
                    review: None,
                    reason: "Explicit supplied review-ready transition under exact owner reach."
                        .into(),
                },
            )
            .unwrap();
        let basis = prepared.current_basis().clone();
        let mut request = prepared
            .authority_request(
                "packet2.author".into(),
                "packet2.author".into(),
                SemanticTimeV1::owner_observation_now_v1().unwrap(),
            )
            .unwrap();
        omit_local_owner_root(&mut request, missing);
        let before = native_owner_file_fingerprints(root);
        let mut admission_entered = false;
        let result = NativeLocalCheckAuthorityV1::with_admission(
            root,
            &locator,
            &basis,
            &request,
            |admission| {
                admission_entered = true;
                Ok(owner.transition_local_lifecycle_v2(
                    prepared,
                    admission,
                    &format!("packet2-omitted-lifecycle-{missing}"),
                ))
            },
        )
        .unwrap();
        assert!(admission_entered);
        assert_eq!(
            result.unwrap_err().kind(),
            ContractOwnerRepositoryErrorKindV1::InvalidRequest
        );
        assert_eq!(native_owner_file_fingerprints(root), before);
        assert_eq!(
            owner
                .current_local_lifecycle_head_v2(&definition.binding())
                .unwrap(),
            head
        );
    }
    let ready = transition(
        root,
        &owner,
        &initial,
        &locator,
        &definition,
        LifecycleStateV1::ReviewReady,
        None,
        "packet2-genuine-root-bound-transition",
    );
    assert_eq!(ready.disposition, OwnerMutationDispositionV1::Committed);
    assert_eq!(ready.lifecycle_head.state(), LifecycleStateV1::ReviewReady);
}

fn omit_local_owner_root(request: &mut LocalCheckActionRequestV1, missing: &str) {
    if missing == "both" {
        request.owner_write_paths.clear();
    } else {
        let absent = if missing == "transactions" {
            ".handbook/state/contracts/v1/transactions"
        } else {
            ".handbook/state/contracts/v2"
        };
        assert!(request.owner_write_paths.iter().any(|path| path == absent));
        request.owner_write_paths.retain(|path| path != absent);
    }
}

fn native_owner_file_fingerprints(root: &std::path::Path) -> Vec<(String, String)> {
    let mut paths = Vec::new();
    collect_native_checkpoint_files(root, &root.join(".handbook/state/contracts"), &mut paths);
    paths.sort();
    paths
        .into_iter()
        .map(|path| {
            let fingerprint = handbook_engine::DefinitionFingerprint::from_bytes(
                &std::fs::read(root.join(&path)).unwrap(),
            )
            .to_string();
            (path, fingerprint)
        })
        .collect()
}

#[test]
fn complete_definition_keeps_permitted_use_immutable_and_refuses_missing_dependencies() {
    let directory = tempfile::tempdir().unwrap();
    let sdk = HandbookSdkV1::open(directory.path());
    let initial = establish_initial(&sdk);
    let (_, outcome) = adopt_check(&sdk, directory.path(), &initial);
    let (eligible, stack) = supplied_definition(
        LocalPermittedUseV2::DecisionSeekingEligible,
        120,
        author_authority(&outcome),
    );
    let (diagnostic, _) = supplied_definition(
        LocalPermittedUseV2::DiagnosticOnly,
        120,
        author_authority(&outcome),
    );
    assert_eq!(
        eligible.semantic_definition(),
        diagnostic.semantic_definition()
    );
    assert_ne!(eligible.binding(), diagnostic.binding());
    assert_eq!(
        eligible.permitted_use(),
        LocalPermittedUseV2::DecisionSeekingEligible
    );
    assert_eq!(
        diagnostic.permitted_use(),
        LocalPermittedUseV2::DiagnosticOnly
    );
    assert_eq!(
        LocalContractDefinitionV2::from_json_bytes(&eligible.canonical_json_bytes().unwrap())
            .unwrap(),
        eligible
    );
    let mut missing = eligible.dependencies().clone();
    missing.freshness_policies.clear();
    assert!(LocalContractDefinitionV2::new(
        LocalContractDefinitionInputV2 {
            semantic_definition: eligible.semantic_definition().clone(),
            permitted_use: eligible.permitted_use(),
            dependencies: missing,
        },
        &stack
    )
    .is_err());
    let mut substituted = eligible.dependencies().clone();
    substituted.schema_closure.documents[0].schema["properties"]["amount"]["minimum"] =
        serde_json::json!(1);
    assert!(LocalContractDefinitionV2::new(
        LocalContractDefinitionInputV2 {
            semantic_definition: eligible.semantic_definition().clone(),
            permitted_use: eligible.permitted_use(),
            dependencies: substituted,
        },
        &stack
    )
    .is_err());
    let mut missing_document = eligible.dependencies().clone();
    missing_document.schema_closure.documents.clear();
    assert!(LocalContractDefinitionV2::new(
        LocalContractDefinitionInputV2 {
            semantic_definition: eligible.semantic_definition().clone(),
            permitted_use: eligible.permitted_use(),
            dependencies: missing_document,
        },
        &stack
    )
    .is_err());
}

#[test]
fn exact_supplied_closed_accounting_policy_preserves_supported_gate_effects() {
    // Pure construction checks no current author permission. This exact source
    // identifies the supplied illustrative definition only.
    let author_source = ExactBindingV1::new(
        ExactRefV1::parse(LOCAL_CHECK_RULE_V1).unwrap(),
        FingerprintV1::parse(&value_fingerprint(&serde_json::json!(
            "supplied constructor-only author source"
        )))
        .unwrap(),
    );
    for permitted in [
        LocalPermittedUseV2::DiagnosticOnly,
        LocalPermittedUseV2::DecisionSeekingEligible,
    ] {
        for effect in [GateEffectV1::Advisory, GateEffectV1::HardFail] {
            let (definition, _) = supplied_definition_with_gate_effect(
                permitted,
                3600,
                author_source.clone(),
                effect,
            );
            assert_eq!(
                definition.semantic_definition().claims()[0].gate_effect(),
                effect
            );
            assert_eq!(
                definition.dependencies().gate_policy.algorithm(),
                GatePolicyAlgorithmV1::ClosedAccounting
            );
            assert_eq!(definition.permitted_use(), permitted);
        }
    }
}

#[test]
#[ignore = "exports exact illustrative proof inputs only when explicitly requested"]
fn export_supplied_normal_consumer_inputs() {
    let output = std::path::PathBuf::from(
        std::env::var_os("HANDBOOK_PACKET2_EXPORT_DIR").expect("supply export directory"),
    );
    assert!(output.is_absolute());
    assert!(
        !output.exists(),
        "proof export must never overwrite prior inputs"
    );
    std::fs::create_dir_all(&output).unwrap();
    let root = output.join("project");
    std::fs::create_dir(&root).unwrap();
    let sdk = HandbookSdkV1::open(&root);
    let initial = establish_initial(&sdk);
    let (locator, authority) = adopt_check(&sdk, &root, &initial);
    let (definition, _) = supplied_definition(
        LocalPermittedUseV2::DecisionSeekingEligible,
        3600,
        author_authority(&authority),
    );
    for (name, bytes) in [
        (
            "initial-request.json",
            serde_json_canonicalizer::to_vec(&supplied_initial_request()).unwrap(),
        ),
        ("initial.json", initial.canonical_json_bytes().unwrap()),
        (
            "successor-terms.json",
            locator.terms.canonical_json_bytes().unwrap(),
        ),
        ("successor.json", locator.canonical_json_bytes().unwrap()),
        (
            "definition.json",
            definition.canonical_json_bytes().unwrap(),
        ),
        (
            "binding.json",
            serde_json_canonicalizer::to_vec(&definition.binding()).unwrap(),
        ),
        (
            "validation-content.json",
            serde_json_canonicalizer::to_vec(&supplied_validation_content()).unwrap(),
        ),
    ] {
        std::fs::write(output.join(name), bytes).unwrap();
    }
    println!(
        "proof path={} binding={:?} closure={}",
        output.display(),
        definition.binding(),
        definition.closure_fingerprint().as_str()
    );
    // The actual separate review JSON is supplied after inspection. This
    // exporter creates no approval outcome or authored/lifecycle owner record.
}

#[test]
fn selected_actual_review_makes_complete_deferred_definition_active_and_usable() {
    for permitted in [
        LocalPermittedUseV2::DecisionSeekingEligible,
        LocalPermittedUseV2::DiagnosticOnly,
    ] {
        let directory = tempfile::tempdir().unwrap();
        let sdk = HandbookSdkV1::open(directory.path());
        let initial = establish_initial(&sdk);
        let (locator, authority) = adopt_check(&sdk, directory.path(), &initial);
        let (definition, stack) = supplied_definition(permitted, 120, author_authority(&authority));
        let owner = ContractOwnerRepositoryV1::open(directory.path()).unwrap();
        let authored = append_definition(
            directory.path(),
            &owner,
            &initial,
            &locator,
            definition.clone(),
            &stack,
            "packet2-review-definition",
        );
        assert_eq!(authored.lifecycle_head.state(), LifecycleStateV1::Draft);
        assert!(owner
            .prepare_local_contract_use_v2(
                &project(&initial),
                &definition.binding(),
                LocalContractUseIntentV2::Diagnostic
            )
            .is_err());
        transition(
            directory.path(),
            &owner,
            &initial,
            &locator,
            &definition,
            LifecycleStateV1::ReviewReady,
            None,
            "packet2-ready-for-review",
        );
        let head = owner
            .current_local_lifecycle_head_v2(&definition.binding())
            .unwrap();
        let input = |review| LocalLifecycleTransitionInputV2 {
            expected_lifecycle_fingerprint: head.lifecycle_fingerprint().clone(),
            to_state: LifecycleStateV1::Locked,
            review,
            reason: "Apply the selected exact actual review outcome.".into(),
        };
        assert!(owner
            .prepare_local_lifecycle_transition_v2(
                &project(&initial),
                &definition.binding(),
                input(None)
            )
            .is_err());
        let mut rejected = supplied_review_outcome(&definition);
        rejected.decision = LocalReviewDecisionV2::Reject;
        rejected.outcome_text = "Reject: the reviewer does not approve this exact closure.".into();
        assert!(owner
            .prepare_local_lifecycle_transition_v2(
                &project(&initial),
                &definition.binding(),
                input(Some(rejected))
            )
            .is_err());
        let mut relabeled_author = supplied_review_outcome(&definition);
        relabeled_author.reviewer_actor_ref = "packet2.author".into();
        assert!(owner
            .prepare_local_lifecycle_transition_v2(
                &project(&initial),
                &definition.binding(),
                input(Some(relabeled_author))
            )
            .is_err());
        let mut same_participant = supplied_review_outcome(&definition);
        same_participant.reviewer_participant_ref = "packet2.author".into();
        assert!(owner
            .prepare_local_lifecycle_transition_v2(
                &project(&initial),
                &definition.binding(),
                input(Some(same_participant))
            )
            .is_err());
        let mut different_closure = supplied_review_outcome(&definition);
        different_closure.closure_fingerprint =
            FingerprintV1::parse(&format!("sha256:{}", "0".repeat(64))).unwrap();
        assert!(owner
            .prepare_local_lifecycle_transition_v2(
                &project(&initial),
                &definition.binding(),
                input(Some(different_closure))
            )
            .is_err());
        let mut unselected = supplied_review_outcome(&definition);
        unselected.reviewer_participant_ref = "outside-reviewer".into();
        unselected.reviewer_actor_ref = "outside-reviewer".into();
        let prepared = owner
            .prepare_local_lifecycle_transition_v2(
                &project(&initial),
                &definition.binding(),
                input(Some(unselected)),
            )
            .unwrap();
        let request = prepared
            .authority_request(
                "outside-reviewer".into(),
                "outside-reviewer".into(),
                SemanticTimeV1::parse("2026-10-05T12:00:00Z").unwrap(),
            )
            .unwrap();
        assert!(NativeLocalCheckAuthorityV1::with_admission(
            directory.path(),
            &locator,
            prepared.current_basis(),
            &request,
            |_| Ok(())
        )
        .is_err());
        assert_eq!(
            owner
                .current_local_lifecycle_head_v2(&definition.binding())
                .unwrap(),
            head
        );
        let reviewed = transition(
            directory.path(),
            &owner,
            &initial,
            &locator,
            &definition,
            LifecycleStateV1::Locked,
            Some(supplied_review_outcome(&definition)),
            "packet2-reviewed-exact-closure",
        );
        assert_eq!(
            reviewed
                .lifecycle_head
                .approving_review()
                .unwrap()
                .reviewer_actor_ref,
            "packet2.reviewer"
        );
        transition(
            directory.path(),
            &owner,
            &initial,
            &locator,
            &definition,
            LifecycleStateV1::Active,
            None,
            "packet2-active-after-review",
        );
        let reopened =
            ContractOwnerRepositoryV1::open_existing_for_local_use_v2(directory.path()).unwrap();
        assert_eq!(
            reopened
                .read_local_definition_v2(&definition.binding())
                .unwrap()
                .definition(),
            &definition
        );
        let usable = reopened
            .prepare_local_contract_use_v2(
                &project(&initial),
                &definition.binding(),
                LocalContractUseIntentV2::Diagnostic,
            )
            .unwrap();
        assert!(matches!(usable.applicability().governance_basis, handbook_contracts::project_admission_governance::ProjectAdmissionScopeBasisV3::DeferredV2(_)));
        assert!(usable
            .applicability()
            .dependencies
            .iter()
            .all(|d| d.qualification == LocalDependencyQualificationV2::DeferredIntrinsic));
        assert_eq!(
            reopened
                .prepare_local_contract_use_v2(
                    &project(&initial),
                    &definition.binding(),
                    LocalContractUseIntentV2::DecisionSeeking
                )
                .is_ok(),
            permitted == LocalPermittedUseV2::DecisionSeekingEligible
        );
    }
}

#[test]
fn stale_deferred_use_refuses_then_fresh_governed_use_qualifies_exact_dependencies() {
    use handbook_sdk::admission_governance::*;
    let directory = tempfile::tempdir().unwrap();
    let sdk = HandbookSdkV1::open(directory.path());
    let initial = establish_initial(&sdk);
    let (locator, authority) = adopt_check(&sdk, directory.path(), &initial);
    let (definition, stack) = supplied_definition(
        LocalPermittedUseV2::DecisionSeekingEligible,
        120,
        author_authority(&authority),
    );
    let owner = ContractOwnerRepositoryV1::open(directory.path()).unwrap();
    activate_definition(directory.path(), &owner, &initial, &locator, &definition);
    let d_use = owner
        .prepare_local_contract_use_v2(
            &project(&initial),
            &definition.binding(),
            LocalContractUseIntentV2::DecisionSeeking,
        )
        .unwrap();
    let ProjectAdmissionScopeBasisV3::DeferredV2(d) =
        d_use.applicability().governance_basis.clone()
    else {
        panic!("must start in D")
    };
    let prepared = sdk.prepare_deferred_project_admission_governance(&DeferredEstablishmentRequestV3::human_approved(
        d.project, ExpectedDeferredBasisV2 { outcome_fingerprint: d.outcome_fingerprint, relation_fingerprint: d.relation_fingerprint },
        300, "packet2-establish-exact-g", "The supplied human approved this exact freshness ceiling and same-project establishment.",
    ).unwrap()).unwrap();
    assert!(matches!(
        sdk.establish_deferred_project_admission_governance(prepared),
        DeferredEstablishmentExecutionV3::Committed { .. }
    ));
    let source_action = LocalCheckActionRequestV1 {
        action: LocalCheckActionV1::EvidenceSource,
        action_ref: "handbook.contract-evidence-source.local@2.0.0".into(),
        subject_ref: definition.binding().reference().as_str().into(),
        content_fingerprint: definition.binding().fingerprint().as_str().into(),
        closure_fingerprint: definition.closure_fingerprint().as_str().into(),
        actor_ref: "packet2.author".into(),
        participant_ref: "packet2.author".into(),
        semantic_time_utc: "2026-10-05T12:00:00Z".into(),
        owner_write_paths: vec![],
        input_paths: vec![],
    };
    assert!(NativeLocalCheckAuthorityV1::with_admission(
        directory.path(),
        &locator,
        d_use.current_basis(),
        &source_action,
        |_| Ok(())
    )
    .is_err());
    let g_use = owner
        .prepare_local_contract_use_v2(
            &project(&initial),
            &definition.binding(),
            LocalContractUseIntentV2::DecisionSeeking,
        )
        .unwrap();
    assert!(matches!(
        g_use.applicability().governance_basis,
        ProjectAdmissionScopeBasisV3::EstablishedFromDeferredV3(_)
    ));
    assert_ne!(g_use.current_basis(), d_use.current_basis());
    assert!(g_use
        .applicability()
        .dependencies
        .iter()
        .filter(|d| d.kind == "evidence_freshness_policy")
        .all(|d| d.qualification == LocalDependencyQualificationV2::QualifiedFreshness));
    assert!(g_use
        .applicability()
        .dependencies
        .iter()
        .filter(|d| d.kind != "evidence_freshness_policy")
        .all(|d| d.qualification == LocalDependencyQualificationV2::NotApplicableToFreshnessScope));
    assert!(g_use
        .applicability()
        .dependencies
        .iter()
        .any(|d| d.kind == "schema_document"));
    NativeLocalCheckAuthorityV1::with_admission(
        directory.path(),
        &locator,
        g_use.current_basis(),
        &source_action,
        |_| Ok(()),
    )
    .unwrap();
    let (over_ceiling, _) = supplied_definition(
        LocalPermittedUseV2::DecisionSeekingEligible,
        301,
        author_authority(&authority),
    );
    assert!(owner
        .prepare_local_definition_append_v2(&project(&initial), over_ceiling, &stack)
        .is_err());
    assert!(EvidenceFreshnessPolicyInputV1::new(
        ContractIdV1::parse("handbook.freshness.order-capture").unwrap(),
        VersionV1::parse("1.0.0").unwrap(),
        0,
        true,
        0,
    )
    .is_err());
    let mut q = definition.dependencies().clone();
    q.required_governance_scopes = vec!["contracts.unsupported-required-scope/root".into()];
    let q = LocalContractDefinitionV2::new(
        LocalContractDefinitionInputV2 {
            semantic_definition: definition.semantic_definition().clone(),
            permitted_use: definition.permitted_use(),
            dependencies: q,
        },
        &stack,
    )
    .unwrap();
    assert!(owner
        .prepare_local_definition_append_v2(&project(&initial), q, &stack)
        .is_err());
    assert_eq!(
        owner
            .read_local_definition_v2(&definition.binding())
            .unwrap()
            .definition(),
        &definition
    );
    assert_eq!(
        sdk.resolve_local_initial_setup_request(&initial)
            .unwrap()
            .terms
            .unwrap()
            .outcome
            .unwrap()
            .outcome_fingerprint,
        locator.terms.expected_predecessor_fingerprint
    );
    owner
        .prepare_local_contract_use_v2(
            &project(&initial),
            &definition.binding(),
            LocalContractUseIntentV2::DecisionSeeking,
        )
        .unwrap();
}

#[test]
fn positive_v3_noncommit_keeps_fresh_successor_definition_use_deferred() {
    use handbook_contracts::project_admission_governance::{
        EstablishmentFaultBoundaryV3, ProjectAdmissionEstablishmentV3,
    };
    use handbook_sdk::admission_governance::*;
    let directory = tempfile::tempdir().unwrap();
    let sdk = HandbookSdkV1::open(directory.path());
    let initial = establish_initial(&sdk);
    let (locator, authority) = adopt_check(&sdk, directory.path(), &initial);
    let (definition, _) = supplied_definition(
        LocalPermittedUseV2::DecisionSeekingEligible,
        120,
        author_authority(&authority),
    );
    let owner = ContractOwnerRepositoryV1::open(directory.path()).unwrap();
    activate_definition(directory.path(), &owner, &initial, &locator, &definition);
    let before = owner
        .prepare_local_contract_use_v2(
            &project(&initial),
            &definition.binding(),
            LocalContractUseIntentV2::DecisionSeeking,
        )
        .unwrap();
    let ProjectAdmissionScopeBasisV3::DeferredV2(d) =
        before.applicability().governance_basis.clone()
    else {
        panic!("D expected")
    };
    let prepared = sdk
        .prepare_deferred_project_admission_governance(
            &DeferredEstablishmentRequestV3::human_approved(
                d.project,
                ExpectedDeferredBasisV2 {
                    outcome_fingerprint: d.outcome_fingerprint,
                    relation_fingerprint: d.relation_fingerprint,
                },
                300,
                "packet2-g-positive-noncommit",
                "The supplied human approved the bounded establishment attempt.",
            )
            .unwrap(),
        )
        .unwrap();
    let governance_locator = prepared.locator().clone();
    assert!(matches!(
        ProjectAdmissionEstablishmentV3::new()
            .with_fault_for_testing(EstablishmentFaultBoundaryV3::Stage(2))
            .establish(directory.path(), prepared),
        DeferredEstablishmentExecutionV3::Uncertain { .. }
    ));
    assert!(matches!(
        sdk.resolve_deferred_admission_governance_request(&governance_locator),
        DeferredEstablishmentExecutionV3::NotCommitted { .. }
    ));
    let fresh = sdk
        .prepare_local_contract_use(
            &initial,
            &definition.binding(),
            LocalContractUseIntentV2::DecisionSeeking,
        )
        .unwrap();
    assert!(matches!(
        fresh.applicability().governance_basis,
        ProjectAdmissionScopeBasisV3::DeferredV2(_)
    ));
    assert_ne!(fresh.current_basis(), before.current_basis());
    sdk.resolve_current_local_check_terms(&initial, &locator)
        .unwrap();
}

#[test]
fn real_native_sources_reopen_and_revalidate_through_admission_append_and_verdict() {
    use handbook_engine::{SourceCurrentnessPortV1, SourceCurrentnessRequestV1};
    use handbook_sdk::admission_governance::*;
    let _package = InstalledProductionPackage::install();
    let directory = tempfile::tempdir().unwrap();
    let sdk = HandbookSdkV1::open(directory.path());
    let initial = establish_initial(&sdk);
    let (locator, authority) = adopt_check(&sdk, directory.path(), &initial);
    let (definition, stack) = supplied_definition(
        LocalPermittedUseV2::DecisionSeekingEligible,
        3600,
        author_authority(&authority),
    );
    let owner = ContractOwnerRepositoryV1::open(directory.path()).unwrap();
    let active = activate_definition(directory.path(), &owner, &initial, &locator, &definition);
    let manifest = production_manifest();
    let role = &manifest.supported_inputs()[0];
    let capture = DockInlineJsonInputV1::new(
        role.role().clone(),
        role.media_type(),
        role.schema().clone(),
        supplied_validation_content(),
    )
    .unwrap();
    // This real native owner run intentionally isolates Packet 2's source seam.
    // It has no Packet 3 Coordination, occurrence/plan or Evaluation Authorization.
    let run = sdk
        .prepare_local_source_seam_run(
            PrepareEvaluationRunInputV1::new(
                definition.binding(),
                active.lifecycle_fingerprint().clone(),
                capture.captured_evaluation_subject_v1().unwrap(),
                vec![capture
                    .captured_evaluation_case_v1(id("case.schema-conformance"))
                    .unwrap()],
                manifest.resolution_ceiling().clone(),
                MatcherBindingV1::exact_fields_v1(),
                definition
                    .dependencies()
                    .freshness_policies
                    .iter()
                    .map(|p| p.exact_binding())
                    .collect(),
                vec![definition.dependencies().stability_policy.clone()],
                definition.dependencies().gate_policy.exact_binding(),
                local_closeout_policy_binding_v1().unwrap(),
                EvaluationRunNonceV1::parse("packet2-real-native-source").unwrap(),
            )
            .unwrap(),
        )
        .unwrap();
    assert!(run.is_local_source_seam_v2());
    let dock_run = DockEvaluationRunBindingV1::new(DockEvaluationRunBindingInputV1 {
        evaluation_run: ContentAddressedBindingV1::new(
            run.evaluation_run_ref(),
            run.evaluation_run_fingerprint().clone(),
        )
        .unwrap(),
        evaluation_lifecycle_basis: run.evaluation_lifecycle_basis().clone(),
        contract: run.contract(),
        claim_ids: run.claims().iter().map(|c| c.claim_id().clone()).collect(),
        subject: run.subject().clone(),
        cases: run.cases().to_vec(),
        requested_resolution: run.requested_resolution().clone(),
        semantic_time_utc: run.owner_currentness().semantic_time_utc().clone(),
        lifecycle_head_fingerprint: run.owner_currentness().lifecycle_head_fingerprint().clone(),
    })
    .unwrap();
    let request = DockProcessRequestV1::new(
        &manifest,
        DockProcessRequestInputV1 {
            run: dock_run,
            requested_evidence_kinds: vec![id("handbook.evidence.schema-conformance")],
            inputs: vec![capture.clone()],
            effective_limits: manifest.limits().clone(),
        },
    )
    .unwrap();
    let store = DockOperationalStoreV1::open(directory.path()).unwrap();
    let bundle = DockResolvedFirstPartyBundleV1::resolve_production().unwrap();
    let produced = DockProcessExecutorV1::new()
        .execute(DockExecutionInputV1 {
            store: &store,
            resolved_bundle: &bundle,
            request: &request,
            evaluation_run_jcs: &run.canonical_json_bytes().unwrap(),
            idempotency_key: "packet2-native-validator-source",
            cancellation: &DockCancellationV1::new(),
        })
        .unwrap();
    assert_eq!(
        produced.execution_record().host_outcome(),
        DockHostOutcomeV1::Completed
    );
    assert_eq!(produced.candidates().len(), 1);
    let candidate = produced.candidates()[0].clone();
    let mut observed = DockCapturedInputCurrentnessV1::open(directory.path()).unwrap();
    let source_ref = format!("{}#input/{}", request.request_ref(), role.role().as_str());
    let supplied_source = SourceCurrentnessRequestV1::new(
        &source_ref,
        capture.content_fingerprint().as_str(),
        capture.content_fingerprint().as_str(),
    )
    .unwrap();
    let actual_source = observed.observe_current(&supplied_source).unwrap();
    assert_eq!(
        actual_source.revision(),
        capture.content_fingerprint().as_str()
    );
    assert_eq!(
        actual_source.current_head_fingerprint(),
        request.request_fingerprint().as_str()
    );
    let wrong_expected = SourceCurrentnessRequestV1::new(
        &source_ref,
        capture.content_fingerprint().as_str(),
        &format!("sha256:{}", "0".repeat(64)),
    )
    .unwrap();
    assert!(
        observed.observe_current(&wrong_expected).is_err(),
        "observer must never echo caller's expected revision"
    );

    let recovery = directory.path().join("packet2-source-check.json");
    std::fs::write(&recovery, serde_json::to_vec(&serde_json::json!({
        "locator": locator, "project": project(&initial), "definition": definition.binding(), "candidate": candidate,
    })).unwrap()).unwrap();
    let child = std::process::Command::new(std::env::current_exe().unwrap())
        .args(["--exact", "reopened_native_source_child", "--nocapture"])
        .env("HANDBOOK_PACKET2_SOURCE_CHILD_ROOT", directory.path())
        .env("HANDBOOK_PACKET2_SOURCE_CHILD_INPUT", &recovery)
        .output()
        .unwrap();
    assert!(
        child.status.success(),
        "{}\n{}",
        String::from_utf8_lossy(&child.stdout),
        String::from_utf8_lossy(&child.stderr)
    );

    let local_source = sdk
        .open_local_contract_evidence_source(
            &initial,
            &locator,
            definition.binding(),
            LocalContractUseIntentV2::DecisionSeeking,
        )
        .unwrap();
    let mut source = CountingSource {
        inner: local_source,
        calls: 0,
    };
    let policies = definition.dependencies().policy_registry().unwrap();
    let admitted = admit_evidence_candidate_v1(
        &owner,
        &mut source,
        &mut observed,
        &stack,
        &policies,
        &candidate,
    )
    .unwrap();
    assert_eq!(source.calls, 1);
    let d_admission = serde_json::to_value(&admitted).unwrap();
    assert_eq!(
        d_admission["schema_ref"],
        "handbook.contract-evidence-record@2.0.0"
    );
    let d_authority = &d_admission["body"]["evidence_source_authority"];
    assert_eq!(
        d_authority["schema_ref"],
        "handbook.contracts.local-evidence-source-authority@2.0.0"
    );
    assert_eq!(d_authority["assurance"], "local_advisory");
    assert!(["registry_state", "registry_head", "authority"]
        .iter()
        .all(|field| d_authority.get(*field).is_none()));
    let mut downgraded = d_admission.clone();
    downgraded["schema_ref"] = serde_json::json!("handbook.contract-evidence-record@1.0.0");
    assert!(serde_json::from_value::<EvidenceRecordV1>(downgraded).is_err());
    let appended = owner
        .append_evidence_candidate_v1(
            &mut source,
            &mut observed,
            &stack,
            &policies,
            &candidate,
            "packet2-real-evidence-append",
        )
        .unwrap();
    assert_eq!(
        appended.disposition(),
        OwnerMutationDispositionV1::Committed
    );
    assert_eq!(
        source.calls, 3,
        "append must resolve the real source before admission and again through commit"
    );
    let evaluated =
        evaluate_claim_verdicts_v1(&owner, &mut source, &mut observed, &stack, &policies, &run)
            .unwrap();
    assert_eq!(
        source.calls, 5,
        "verdict must perform both initial and final fresh source revalidation"
    );
    assert_eq!(evaluated.verdicts().len(), 1);
    assert_eq!(
        evaluated.verdicts()[0].decision(),
        ClaimVerdictDecisionV1::Pass
    );
    assert_eq!(
        serde_json::to_value(&evaluated.verdicts()[0]).unwrap()["schema_ref"],
        "handbook.contract-claim-verdict@2.0.0"
    );
    let historical_d_bytes = appended.record().canonical_json_bytes().unwrap();
    let historical_d_authority = serde_json::to_value(appended.record()).unwrap()["body"]
        ["evidence_source_authority"]
        .clone();
    let d_applicability: LocalContractApplicabilityV2 =
        serde_json::from_value(historical_d_authority["applicability"].clone()).unwrap();
    let ProjectAdmissionScopeBasisV3::DeferredV2(d) = d_applicability.governance_basis else {
        panic!("native evidence must start in D")
    };
    let establishment = sdk.prepare_deferred_project_admission_governance(&DeferredEstablishmentRequestV3::human_approved(
        d.project, ExpectedDeferredBasisV2 { outcome_fingerprint: d.outcome_fingerprint, relation_fingerprint: d.relation_fingerprint },
        3600, "packet2-native-source-g", "The supplied human approved this exact same-project freshness ceiling for the retained native order capture.",
    ).unwrap()).unwrap();
    assert!(matches!(
        sdk.establish_deferred_project_admission_governance(establishment),
        DeferredEstablishmentExecutionV3::Committed { .. }
    ));
    let fresh_g = admit_evidence_candidate_v1(
        &owner,
        &mut source,
        &mut observed,
        &stack,
        &policies,
        &candidate,
    )
    .unwrap();
    let g_authority =
        serde_json::to_value(&fresh_g).unwrap()["body"]["evidence_source_authority"].clone();
    let g_applicability: LocalContractApplicabilityV2 =
        serde_json::from_value(g_authority["applicability"].clone()).unwrap();
    assert!(matches!(
        g_applicability.governance_basis,
        ProjectAdmissionScopeBasisV3::EstablishedFromDeferredV3(_)
    ));
    assert!(g_applicability
        .dependencies
        .iter()
        .filter(|d| d.kind == "evidence_freshness_policy")
        .all(|d| d.qualification == LocalDependencyQualificationV2::QualifiedFreshness));
    assert_ne!(
        g_authority["native_basis"],
        historical_d_authority["native_basis"]
    );
    assert_eq!(
        g_authority["definition"],
        historical_d_authority["definition"]
    );
    let fresh_verdict =
        evaluate_claim_verdicts_v1(&owner, &mut source, &mut observed, &stack, &policies, &run)
            .unwrap();
    assert_eq!(
        fresh_verdict.verdicts()[0].decision(),
        ClaimVerdictDecisionV1::Pass
    );
    let EvidenceReadOutcomeV1::Found(historical) = owner
        .read_evidence(
            appended.record().record_ref(),
            appended.record().record_fingerprint(),
        )
        .unwrap()
    else {
        panic!("D evidence history must remain present")
    };
    assert_eq!(historical.record_bytes(), historical_d_bytes);
    assert_public_normalizer_refuses_forged_pass(
        directory.path(),
        &sdk,
        &initial,
        &locator,
        &definition,
        &store,
        &bundle,
    );
    let request_path = directory.path().join(format!(
        ".handbook/state/dock/v1/requests/{}.json",
        request
            .request_fingerprint()
            .as_str()
            .trim_start_matches("sha256:")
    ));
    let exact_request = std::fs::read(&request_path).unwrap();
    let attempted_substitution = source.resolve_exact(&candidate, |resolved| {
        assert!(resolved.validate_current().is_ok());
        let changed = std::fs::write(&request_path, b"substituted retained inline capture");
        if changed.is_ok() {
            assert!(resolved.validate_current().is_err());
        }
        changed.is_ok()
    });
    match attempted_substitution {
        Ok(false) => {} // Native pinned Windows handles prevent the write.
        Err(_) => {}    // Other supported hosts detect the changed capture.
        other => panic!("changed captured input must not finish as current: {other:?}"),
    }
    std::fs::write(&request_path, b"substituted retained inline capture").unwrap();
    assert!(observed.observe_current(&supplied_source).is_err());
    assert!(source.resolve_exact(&candidate, |_| ()).is_err());
    std::fs::write(&request_path, &exact_request).unwrap();
    sdk.revoke_local_check_terms(
        &locator,
        &LocalRevokeRequestV1 {
            request_key: "packet2-source-revoke".into(),
            expected_current_fingerprint: authority.outcome_fingerprint,
            principal: "packet2.human".into(),
            source_ref: "decision.packet2.source-revoke".into(),
            instruction: "Revoke exact source authority.".into(),
        },
    )
    .unwrap();
    assert!(source.resolve_exact(&candidate, |_| ()).is_err());
    let blocked =
        evaluate_claim_verdicts_v1(&owner, &mut source, &mut observed, &stack, &policies, &run)
            .unwrap();
    assert_eq!(
        blocked.verdicts()[0].decision(),
        ClaimVerdictDecisionV1::Blocked
    );
    assert_eq!(
        sdk.resume_local_check_terms_request(&locator).disposition,
        LocalAuthorityDispositionV1::Replayed
    );
    assert!(source.resolve_exact(&candidate, |_| ()).is_err());
    drop(source);
    drop(store);
    let lost_lock = directory
        .path()
        .join(".handbook/state/locks/dock-operational-store.lock");
    std::fs::remove_file(&lost_lock).unwrap();
    assert!(DockCapturedInputCurrentnessV1::open(directory.path()).is_err());
    assert!(DockLocalEvidenceCandidateSourceV2::open(
        directory.path(),
        locator,
        project(&initial),
        definition.binding(),
        LocalContractUseIntentV2::DecisionSeeking
    )
    .is_err());
    assert!(
        !lost_lock.exists(),
        "source/currentness reads must not recreate lost owner locks"
    );
}

fn assert_public_normalizer_refuses_forged_pass(
    root: &std::path::Path,
    sdk: &HandbookSdkV1,
    initial: &handbook_sdk::local_setup::LocalInitialSetupLocatorV1,
    locator: &LocalCheckLocatorV1,
    definition: &LocalContractDefinitionV2,
    store: &DockOperationalStoreV1,
    bundle: &DockResolvedFirstPartyBundleV1,
) {
    let manifest = bundle.manifest();
    let role = &manifest.supported_inputs()[0];
    let mut content = supplied_validation_content();
    content["instance"]["value"]["amount"] = serde_json::json!(-1);
    content["instance"]["instance_fingerprint"] =
        serde_json::json!(value_fingerprint(&content["instance"]["value"]));
    let capture = DockInlineJsonInputV1::new(
        role.role().clone(),
        role.media_type(),
        role.schema().clone(),
        content,
    )
    .unwrap();
    let head = sdk
        .current_local_contract_lifecycle(&definition.binding())
        .unwrap();
    let run = sdk
        .prepare_local_source_seam_run(
            PrepareEvaluationRunInputV1::new(
                definition.binding(),
                head.lifecycle_fingerprint().clone(),
                capture.captured_evaluation_subject_v1().unwrap(),
                vec![capture
                    .captured_evaluation_case_v1(id("case.schema-conformance"))
                    .unwrap()],
                manifest.resolution_ceiling().clone(),
                MatcherBindingV1::exact_fields_v1(),
                definition
                    .dependencies()
                    .freshness_policies
                    .iter()
                    .map(|p| p.exact_binding())
                    .collect(),
                vec![definition.dependencies().stability_policy.clone()],
                definition.dependencies().gate_policy.exact_binding(),
                local_closeout_policy_binding_v1().unwrap(),
                EvaluationRunNonceV1::parse("packet2-real-violation-for-refusal").unwrap(),
            )
            .unwrap(),
        )
        .unwrap();
    let request = DockProcessRequestV1::new(
        manifest,
        DockProcessRequestInputV1 {
            run: DockEvaluationRunBindingV1::new(DockEvaluationRunBindingInputV1 {
                evaluation_run: ContentAddressedBindingV1::new(
                    run.evaluation_run_ref(),
                    run.evaluation_run_fingerprint().clone(),
                )
                .unwrap(),
                evaluation_lifecycle_basis: run.evaluation_lifecycle_basis().clone(),
                contract: run.contract(),
                claim_ids: run.claims().iter().map(|c| c.claim_id().clone()).collect(),
                subject: run.subject().clone(),
                cases: run.cases().to_vec(),
                requested_resolution: run.requested_resolution().clone(),
                semantic_time_utc: run.owner_currentness().semantic_time_utc().clone(),
                lifecycle_head_fingerprint: run
                    .owner_currentness()
                    .lifecycle_head_fingerprint()
                    .clone(),
            })
            .unwrap(),
            requested_evidence_kinds: vec![id("handbook.evidence.schema-conformance")],
            inputs: vec![capture],
            effective_limits: manifest.limits().clone(),
        },
    )
    .unwrap();
    let produced = DockProcessExecutorV1::new()
        .execute(DockExecutionInputV1 {
            store,
            resolved_bundle: bundle,
            request: &request,
            evaluation_run_jcs: &run.canonical_json_bytes().unwrap(),
            idempotency_key: "packet2-native-violation-for-refusal",
            cancellation: &DockCancellationV1::new(),
        })
        .unwrap();
    assert_eq!(
        produced.execution_record().host_outcome(),
        DockHostOutcomeV1::Completed
    );
    let candidate = &produced.candidates()[0];
    let mut source = sdk
        .open_local_contract_evidence_source(
            initial,
            locator,
            definition.binding(),
            LocalContractUseIntentV2::DecisionSeeking,
        )
        .unwrap();
    let original = source
        .resolve_exact(candidate, |resolved| {
            ResolvedEvidenceCandidateObservationInputV1 {
                candidate_jcs: resolved.candidate_jcs().to_vec(),
                evaluation_run_jcs: resolved.evaluation_run_jcs().to_vec(),
                producer: resolved.producer().clone(),
                request_record: resolved.request_record().clone(),
                execution_record: resolved.execution_record().clone(),
                producer_resolution_ceiling: resolved.producer_resolution_ceiling().cloned(),
            }
        })
        .unwrap();
    let genuine_input = original.clone();
    let mut forged: serde_json::Value = serde_json::from_slice(&original.candidate_jcs).unwrap();
    assert_eq!(forged["body"]["observation"], "violated");
    forged["body"]["observation"] = serde_json::json!("satisfied");
    forged["body"]["fact_payload"]["conformance"] = serde_json::json!("conformant");
    let fingerprint = value_fingerprint(
        &serde_json::json!({"schema_ref": forged["schema_ref"], "body": forged["body"]}),
    );
    forged["candidate_fingerprint"] = serde_json::json!(fingerprint);
    forged["candidate_ref"] = serde_json::json!(format!(
        "{}#candidate/{}",
        original.producer.reference(),
        fingerprint.trim_start_matches("sha256:")
    ));
    let restored: EvidenceCandidateV1 = serde_json::from_value(forged.clone()).unwrap();
    let forged_binding = ContentAddressedBindingV1::new(
        restored.candidate_ref(),
        restored.candidate_fingerprint().clone(),
    )
    .unwrap();
    assert_ne!(&forged_binding, candidate);
    let forged_input = ResolvedEvidenceCandidateObservationInputV1 {
        candidate_jcs: serde_json_canonicalizer::to_vec(&forged).unwrap(),
        ..original
    };
    let owner = ContractOwnerRepositoryV1::open_existing_for_local_use_v2(root).unwrap();
    let local_use = owner
        .prepare_local_contract_use_v2(
            &project(initial),
            &definition.binding(),
            LocalContractUseIntentV2::DecisionSeeking,
        )
        .unwrap();
    let retained_root = handbook_engine::RetainedRepositoryRootV1::open_exact(root).unwrap();
    let mut paths = local_use.required_state_paths().to_vec();
    collect_native_checkpoint_files(root, &root.join(".handbook/state/dock/v1"), &mut paths);
    paths.push(".handbook/state/locks/dock-operational-store.lock".into());
    paths.sort();
    paths.dedup();
    let forged_path = format!(
        ".handbook/state/dock/v1/candidates/{}.json",
        fingerprint.trim_start_matches("sha256:")
    );
    assert!(!root.join(&forged_path).exists());
    let action = LocalCheckActionRequestV1 {
        action: LocalCheckActionV1::EvidenceSource,
        action_ref: "handbook.contract-evidence-source.local@2.0.0".into(),
        subject_ref: forged_binding.reference().into(),
        content_fingerprint: forged_binding.fingerprint().as_str().into(),
        closure_fingerprint: local_use
            .applicability()
            .policy_admission_closure_fingerprint
            .as_str()
            .into(),
        actor_ref: locator.terms.agent.clone(),
        participant_ref: locator.terms.author_participant_ref.clone(),
        semantic_time_utc: SemanticTimeV1::owner_observation_now_v1()
            .unwrap()
            .as_str()
            .into(),
        owner_write_paths: vec![".handbook/state/contracts/v1/evidence-v1".into()],
        input_paths: paths.clone(),
    };
    let checkpoint = retained_root
        .retain_exact_files_checkpoint_v1(&paths.iter().map(String::as_str).collect::<Vec<_>>())
        .unwrap();
    let mut genuine_action = action.clone();
    genuine_action.subject_ref = candidate.reference().into();
    genuine_action.content_fingerprint = candidate.fingerprint().as_str().into();
    let mut genuine_consumed = 0;
    NativeLocalCheckAuthorityV1::with_admission(
        root,
        locator,
        local_use.current_basis(),
        &genuine_action,
        |admission| {
            resolve_repository_local_candidate_observation_v2(
                &retained_root,
                &owner,
                admission,
                &local_use,
                &checkpoint,
                candidate,
                genuine_input.clone(),
                |_| {
                    genuine_consumed += 1;
                },
            )
            .unwrap();
            Ok(())
        },
    )
    .unwrap();
    assert_eq!(
        genuine_consumed, 1,
        "the same complete retained checkpoint must accept the genuine native violation"
    );
    for omit_inputs in [true, false] {
        let mut omitted_reach = genuine_action.clone();
        if omit_inputs {
            omitted_reach.input_paths.clear();
        } else {
            omitted_reach.owner_write_paths.clear();
        }
        let mut admission_entered = false;
        let mut omitted_consumed = false;
        NativeLocalCheckAuthorityV1::with_admission(
            root,
            locator,
            local_use.current_basis(),
            &omitted_reach,
            |admission| {
                admission_entered = true;
                let result = resolve_repository_local_candidate_observation_v2(
                    &retained_root,
                    &owner,
                    admission,
                    &local_use,
                    &checkpoint,
                    candidate,
                    genuine_input.clone(),
                    |_| omitted_consumed = true,
                );
                assert_eq!(
                    result.unwrap_err().kind(),
                    EvidenceCandidateSourceErrorKindV1::AuthorityAmbiguous,
                    "genuine candidate and checkpoint cannot borrow omitted read/write reach"
                );
                Ok(())
            },
        )
        .unwrap();
        assert!(admission_entered);
        assert!(!omitted_consumed);
    }
    let forbidden_input = "inputs/nested/outside-selected-schema.json";
    std::fs::create_dir_all(root.join("inputs/nested")).unwrap();
    std::fs::write(
        root.join(forbidden_input),
        serde_json_canonicalizer::to_vec(&serde_json::json!({"type": "object"})).unwrap(),
    )
    .unwrap();
    let mut forbidden_action = genuine_action.clone();
    forbidden_action.input_paths.push(forbidden_input.into());
    forbidden_action.input_paths.sort();
    let forbidden_checkpoint = retained_root
        .retain_exact_files_checkpoint_v1(
            &forbidden_action
                .input_paths
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>(),
        )
        .unwrap();
    forbidden_checkpoint
        .require_file_membership(&retained_root, &[forbidden_input])
        .unwrap();
    let mut forbidden_admission_entered = false;
    assert!(NativeLocalCheckAuthorityV1::with_admission(
        root,
        locator,
        local_use.current_basis(),
        &forbidden_action,
        |_| {
            forbidden_admission_entered = true;
            Ok(())
        },
    )
    .is_err());
    assert!(!forbidden_admission_entered);
    drop(forbidden_checkpoint);
    let mut consumed = false;
    NativeLocalCheckAuthorityV1::with_admission(
        root,
        locator,
        local_use.current_basis(),
        &action,
        |admission| {
            let result = resolve_repository_local_candidate_observation_v2(
                &retained_root,
                &owner,
                admission,
                &local_use,
                &checkpoint,
                &forged_binding,
                forged_input,
                |_| {
                    consumed = true;
                },
            );
            assert!(
                result.is_err(),
                "self-consistent forged Pass must not borrow genuine native provenance"
            );
            assert_eq!(
                result.unwrap_err().kind(),
                EvidenceCandidateSourceErrorKindV1::CandidateSubstituted
            );
            Ok(())
        },
    )
    .unwrap();
    assert!(!consumed);
}

fn collect_native_checkpoint_files(
    root: &std::path::Path,
    directory: &std::path::Path,
    paths: &mut Vec<String>,
) {
    for entry in std::fs::read_dir(directory).unwrap() {
        let entry = entry.unwrap();
        let metadata = entry.file_type().unwrap();
        assert!(!metadata.is_symlink());
        if metadata.is_dir() {
            collect_native_checkpoint_files(root, &entry.path(), paths);
        } else {
            paths.push(
                entry
                    .path()
                    .strip_prefix(root)
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .replace('\\', "/"),
            );
        }
    }
}

#[test]
fn reopened_native_source_child() {
    let Some(root) = std::env::var_os("HANDBOOK_PACKET2_SOURCE_CHILD_ROOT") else {
        return;
    };
    let input: serde_json::Value = serde_json::from_slice(
        &std::fs::read(std::env::var_os("HANDBOOK_PACKET2_SOURCE_CHILD_INPUT").unwrap()).unwrap(),
    )
    .unwrap();
    let mut source = DockLocalEvidenceCandidateSourceV2::open(
        root,
        serde_json::from_value(input["locator"].clone()).unwrap(),
        serde_json::from_value(input["project"].clone()).unwrap(),
        serde_json::from_value(input["definition"].clone()).unwrap(),
        LocalContractUseIntentV2::DecisionSeeking,
    )
    .unwrap();
    let candidate = serde_json::from_value(input["candidate"].clone()).unwrap();
    source
        .resolve_exact(&candidate, |resolved| {
            assert!(resolved.validate_current().is_ok());
            let body: serde_json::Value = serde_json::from_slice(resolved.candidate_jcs()).unwrap();
            assert!(body["body"]["sources"][0]["source"]["ref"]
                .as_str()
                .unwrap()
                .contains("#input/handbook.input.json-schema-validation"));
        })
        .unwrap();
}
