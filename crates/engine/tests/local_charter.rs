use handbook_engine::local_charter::{
    LocalCharterDispositionV1, LocalCharterHumanApprovalV1, LocalCharterLocatorV1,
    LocalProjectBindingV1, NativeLocalCharterRequestV1, NativeLocalCharterServiceV1,
};
use handbook_engine::{
    parse_definition_yaml, resolve_shipped_profile_decisions, serialize_canonical_charter,
    CanonicalCharter, CharterAcquisitionMode, CharterAuthorityTransactionServiceV1,
    CharterCoverageSubmission, CharterIntakeConsumer, CharterIntakeEnvelope,
    CharterIntakeSourceKind, CharterLifecycleEventIntentV1, CharterLifecycleState,
    CharterLifecycleStoreV1, CharterPromotionIntentV1, CharterPromotionWorkflowServiceV1,
    RepositoryInitializationClaimV1, RepositoryInitializationDispositionV1,
    RepositoryInitializationGuardV1, RepositoryInvocationIdentityServiceV1,
    RetainedRepositoryRootV1, REPOSITORY_INITIALIZATION_LOCK_PATH_V1,
};

const CONTENT: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"),
    "/../../docs/specs/handbook-contract-membrane/slices/HCM-2.2/contracts/canonical-charter-boundary-v1.1.yaml"));

fn request(repo: &std::path::Path) -> NativeLocalCharterRequestV1 {
    let content = parse_definition_yaml(CONTENT).unwrap();
    let charter: CanonicalCharter = serde_json::from_value(content.clone()).unwrap();
    let decisions = resolve_shipped_profile_decisions(repo).unwrap();
    let bytes = serialize_canonical_charter(&decisions, &charter).unwrap();
    let coverage = [
        "project_shape.definition",
        "delivery.constraints",
        "delivery.default_implications",
        "operational_reality.production_state",
        "risk.domains",
        "engineering_posture.baseline",
        "policy.authority_and_revision",
        "governance.decision_authority",
        "governance.required_approvals",
        "governance.exception_policy",
        "engineering_posture.dimensions",
        "engineering_posture.red_lines",
        "governance.review_triggers",
        "governance.reassessment_triggers",
        "debt.register",
        "decisions.records",
    ]
    .into_iter()
    .map(|id| CharterCoverageSubmission {
        coverage_id: id.into(),
        source_kind: CharterIntakeSourceKind::UserDeclaration,
        value_ref: format!("declared/{id}"),
        evidence_refs: vec![],
        confidence: "high".into(),
        freshness: None,
        sensitivity: "internal".into(),
        contradiction_refs: vec![],
        waiver_ref: None,
    })
    .collect();
    NativeLocalCharterRequestV1 {
        request_key: "initial-charter".into(),
        intake: CharterIntakeEnvelope {
            mode: CharterAcquisitionMode::Express,
            content,
            coverage,
            consumer: CharterIntakeConsumer {
                kind: "typed-sdk".into(),
                id: "project-agent".into(),
                version: "1.0".into(),
            },
            prompt_event_refs: vec![],
            finalized_at_utc: "2026-10-05T12:00:00Z".into(),
            expected_current_fingerprint: None,
        },
        human_approval: LocalCharterHumanApprovalV1 {
            accepted: true,
            principal_claim: "Project owner".into(),
            agent_claim: "Project agent".into(),
            delegation_claim: "Initial bounded setup".into(),
            source_ref: "conversation/approved-initial-charter".into(),
            approved_content_fingerprint: handbook_engine::DefinitionFingerprint::from_bytes(
                &bytes,
            )
            .to_string(),
        },
    }
}

fn registered(repo: &std::path::Path) -> LocalProjectBindingV1 {
    let identity = RepositoryInvocationIdentityServiceV1::new()
        .allocate_prepared_repository_identity()
        .unwrap();
    let root = RetainedRepositoryRootV1::open_exact(repo).unwrap();
    let claim = RepositoryInitializationClaimV1::prepare(
        repo,
        &root,
        "handbook-contracts.admission-deferral@2.0.0",
        &format!("sha256:{}", "1".repeat(64)),
        identity.repository_identity_fingerprint(),
    )
    .unwrap();
    let guard = RepositoryInitializationGuardV1::acquire(repo).unwrap();
    guard.retain_native_claim(&claim).unwrap();
    guard
        .root()
        .create_directory_new_durable(".handbook")
        .unwrap();
    RepositoryInvocationIdentityServiceV1::new()
        .persist_prepared_repository_identity_locked(&guard, identity)
        .unwrap();
    let terminal = b"native Engine initialization fixture terminal";
    let path = ".handbook/local-registration-fixture-terminal.json";
    guard.root().write_new_durable(path, terminal).unwrap();
    guard
        .complete_native_claim(
            &claim,
            RepositoryInitializationDispositionV1::Committed,
            path,
            terminal,
        )
        .unwrap();
    guard.release().unwrap();
    LocalProjectBindingV1 {
        initialization_claim: claim,
    }
}

fn establish(repo: &std::path::Path) -> LocalCharterLocatorV1 {
    let binding = registered(repo);
    let prepared = NativeLocalCharterServiceV1::prepare(repo, &binding, request(repo)).unwrap();
    let locator = prepared.locator().clone();
    let execution = NativeLocalCharterServiceV1::establish(repo, prepared);
    assert_eq!(
        execution.disposition,
        LocalCharterDispositionV1::Committed,
        "{:?}",
        execution.refusal
    );
    locator
}

#[test]
fn established_local_charter_reopens_through_native_common_readers() {
    let repo = tempfile::tempdir().unwrap();
    let locator = establish(repo.path());
    let reopened =
        LocalCharterLocatorV1::from_json_bytes(&locator.canonical_json_bytes().unwrap()).unwrap();
    let current = NativeLocalCharterServiceV1::resolve_current(repo.path(), &reopened).unwrap();
    assert!(current.human_approval_accepted && current.advisory_permission_accepted);
    let committed = CharterAuthorityTransactionServiceV1::new(repo.path())
        .read_committed_charter()
        .unwrap()
        .unwrap();
    assert_eq!(
        committed.canonical_fingerprint,
        current.canonical_fingerprint
    );
    assert_eq!(committed.promotion_ref, current.promotion_ref);
    assert_eq!(
        committed.lifecycle_transition_ref,
        current.lifecycle_transition_ref
    );
    let lifecycle = CharterLifecycleStoreV1::new(repo.path())
        .observe()
        .unwrap()
        .unwrap();
    assert_eq!(lifecycle.state, CharterLifecycleState::Current);
    assert_eq!(
        lifecycle.canonical_fingerprint,
        current.canonical_fingerprint
    );
    assert_eq!(
        NativeLocalCharterServiceV1::resume_request(repo.path(), &reopened).disposition,
        LocalCharterDispositionV1::Replayed
    );
    assert!(CharterLifecycleStoreV1::new(repo.path())
        .record_event(CharterLifecycleEventIntentV1::CandidateAmendment {
            candidate_ref: "candidate/unsupported".into(),
            candidate_fingerprint: format!("sha256:{}", "3".repeat(64)),
            observed_at_utc: "2026-10-05T12:00:00Z".into(),
        })
        .is_err());
    let refused = CharterPromotionWorkflowServiceV1::new(repo.path())
        .promote(CharterPromotionIntentV1 {
            candidate_ref: format!("candidates/candidate_{}.json", "3".repeat(64)),
            approval_ref: format!("approvals/approval_{}.json", "4".repeat(64)),
            expected_current_fingerprint: Some(locator.canonical_fingerprint.clone()),
        })
        .unwrap_err();
    assert!(refused.detail().contains("local initial Charter"));
}

#[test]
fn restored_locator_has_no_write_authority_without_retained_original_intent() {
    let repo = tempfile::tempdir().unwrap();
    let binding = registered(repo.path());
    let prepared =
        NativeLocalCharterServiceV1::prepare(repo.path(), &binding, request(repo.path())).unwrap();
    let restored =
        LocalCharterLocatorV1::from_json_bytes(&prepared.locator().canonical_json_bytes().unwrap())
            .unwrap();
    assert_eq!(
        NativeLocalCharterServiceV1::resume_request(repo.path(), &restored).disposition,
        LocalCharterDispositionV1::Refused
    );
    assert!(!repo.path().join(".handbook/local-charter").exists());
    assert!(!repo.path().join(".handbook/project/charter.yaml").exists());
}

#[test]
fn exact_retained_pending_terminal_recovers_original_request_only() {
    let repo = tempfile::tempdir().unwrap();
    let locator = establish(repo.path());
    // Reproduce the retained transcript immediately before its terminal rename.
    let partition = repo.path().join(".handbook/local-charter/v1");
    std::fs::rename(
        partition.join("committed.json"),
        partition.join("pending.json"),
    )
    .unwrap();
    std::fs::remove_file(partition.join("current.json")).unwrap();
    std::fs::remove_file(repo.path().join(".handbook/project/charter.yaml")).unwrap();
    let mut changed = locator.clone();
    changed.request.request_key = "different-request".into();
    assert_eq!(
        NativeLocalCharterServiceV1::resume_request(repo.path(), &changed).disposition,
        LocalCharterDispositionV1::Refused
    );
    assert!(NativeLocalCharterServiceV1::resolve_current(repo.path(), &locator).is_err());
    assert_eq!(
        NativeLocalCharterServiceV1::resolve_request(repo.path(), &locator).disposition,
        LocalCharterDispositionV1::Uncertain
    );
    let recovered = NativeLocalCharterServiceV1::resume_request(repo.path(), &locator);
    assert_eq!(
        recovered.disposition,
        LocalCharterDispositionV1::Committed,
        "{:?}",
        recovered.refusal
    );
    assert!(!partition.join("pending.json").exists());
    assert!(NativeLocalCharterServiceV1::resolve_current(repo.path(), &locator).is_ok());
}

#[test]
fn missing_native_charter_lock_is_not_healed_by_any_current_reader() {
    let repo = tempfile::tempdir().unwrap();
    let locator = establish(repo.path());
    let missing = repo.path().join(".handbook/state/locks/promotion.lock");
    std::fs::remove_file(&missing).unwrap();
    assert!(CharterAuthorityTransactionServiceV1::new(repo.path())
        .read_committed_charter()
        .is_err());
    assert!(CharterLifecycleStoreV1::new(repo.path()).observe().is_err());
    assert!(NativeLocalCharterServiceV1::resolve_current(repo.path(), &locator).is_err());
    assert_eq!(
        NativeLocalCharterServiceV1::resume_request(repo.path(), &locator).disposition,
        LocalCharterDispositionV1::Refused
    );
    assert!(!missing.exists());
}

#[test]
fn changed_current_charter_invalidates_current_without_reactivating_history() {
    let repo = tempfile::tempdir().unwrap();
    let locator = establish(repo.path());
    let canonical = repo.path().join(".handbook/project/charter.yaml");
    std::fs::write(&canonical, b"changed canonical content").unwrap();
    assert!(NativeLocalCharterServiceV1::resolve_current(repo.path(), &locator).is_err());
    assert!(CharterAuthorityTransactionServiceV1::new(repo.path())
        .read_committed_charter()
        .is_err());
    assert!(CharterLifecycleStoreV1::new(repo.path()).observe().is_err());
    assert_eq!(
        NativeLocalCharterServiceV1::resume_request(repo.path(), &locator).disposition,
        LocalCharterDispositionV1::Replayed
    );
    assert_eq!(
        std::fs::read(canonical).unwrap(),
        b"changed canonical content"
    );
}

#[test]
fn lost_final_terminal_is_not_reconstructed_by_resume() {
    let repo = tempfile::tempdir().unwrap();
    let locator = establish(repo.path());
    let terminal = repo
        .path()
        .join(".handbook/local-charter/v1/committed.json");
    std::fs::remove_file(&terminal).unwrap();
    let resumed = NativeLocalCharterServiceV1::resume_request(repo.path(), &locator);
    assert_eq!(resumed.disposition, LocalCharterDispositionV1::Refused);
    assert!(!terminal.exists());
}

#[test]
fn lost_intent_and_replaced_initialization_lock_refuse_common_readers() {
    for damaged in ["intent", "lock", "partition"] {
        let repo = tempfile::tempdir().unwrap();
        let locator = establish(repo.path());
        if damaged == "intent" {
            std::fs::remove_file(
                repo.path()
                    .join(".handbook/local-charter/v1/initial-intent.json"),
            )
            .unwrap();
        } else if damaged == "lock" {
            let lock = repo.path().join(REPOSITORY_INITIALIZATION_LOCK_PATH_V1);
            std::fs::rename(&lock, repo.path().join("original-initialization-lock")).unwrap();
            std::fs::write(lock, b"").unwrap();
        } else {
            // Remaining native operating owner evidence still selects the local
            // route when the Charter partition is lost; no old-route fallback.
            std::fs::create_dir_all(repo.path().join(".handbook/local-operating/v1")).unwrap();
            let partition = repo.path().join(".handbook/local-charter");
            assert!(partition.starts_with(repo.path()));
            std::fs::remove_dir_all(partition).unwrap();
        }
        assert!(CharterAuthorityTransactionServiceV1::new(repo.path())
            .read_committed_charter()
            .is_err());
        assert!(CharterLifecycleStoreV1::new(repo.path()).observe().is_err());
        assert!(NativeLocalCharterServiceV1::resolve_current(repo.path(), &locator).is_err());
        assert_eq!(
            NativeLocalCharterServiceV1::resume_request(repo.path(), &locator).disposition,
            LocalCharterDispositionV1::Refused
        );
    }
}

#[test]
fn prospective_local_charter_preparation_has_no_effects_and_requires_exact_approval() {
    let repo = tempfile::tempdir().unwrap();
    let root = RetainedRepositoryRootV1::open_exact(repo.path()).unwrap();
    let claim = RepositoryInitializationClaimV1::prepare(
        repo.path(),
        &root,
        "handbook-contracts.admission-deferral@2.0.0",
        &format!("sha256:{}", "1".repeat(64)),
        &format!("sha256:{}", "2".repeat(64)),
    )
    .unwrap();
    let binding = LocalProjectBindingV1 {
        initialization_claim: claim,
    };
    let approved = request(repo.path());
    let prepared =
        NativeLocalCharterServiceV1::prepare(repo.path(), &binding, approved.clone()).unwrap();
    assert_eq!(
        prepared.canonical_fingerprint(),
        approved.human_approval.approved_content_fingerprint
    );
    assert!(std::fs::read_dir(repo.path()).unwrap().next().is_none());
    let mut missing = approved;
    missing.human_approval.accepted = false;
    assert!(NativeLocalCharterServiceV1::prepare(repo.path(), &binding, missing).is_err());
}
