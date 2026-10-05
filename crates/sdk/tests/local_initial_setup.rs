use handbook_engine::{
    CharterAcquisitionMode, CharterCoverageSubmission, CharterIntakeConsumer,
    CharterIntakeEnvelope, CharterIntakeSourceKind, ContextResolutionMutationDecision,
    ContextResolutionValidationDecision,
};
use handbook_sdk::admission_governance::{DeferralExecutionV2, DeferralRequestInputV2};
use handbook_sdk::local_setup::*;
use handbook_sdk::HandbookSdkV1;
use std::fs;
use tempfile::tempdir;

const CHARTER: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/specs/handbook-contract-membrane/slices/HCM-2.2/contracts/canonical-charter-boundary-v1.1.yaml"
));

fn supplied_request() -> LocalInitialSetupRequestV1 {
    // Test-only supplied initial decisions, submitted to real native owners.
    // They assert local advisory approval and contain no credential assertion.
    let content = handbook_engine::parse_definition_yaml(CHARTER).unwrap();
    let mut terms = LocalOperatingTermsV1 {
        request_key: "terms-once".into(),
        mode: "local_advisory".into(),
        rule_ref: handbook_engine::local_authority::LOCAL_OPERATING_RULE_V1.into(),
        instruction: "Validate exact local inputs and write only the supplied owner reach.".into(),
        principal: "test-human".into(),
        agent: "test-agent".into(),
        delegation: "bounded local validation and owner writes".into(),
        approval_source_ref: "decision.test.initial-terms".into(),
        human_approval_asserted: true,
        approved_content_fingerprint: String::new(),
        ceiling: LocalEnvelopeCeilingV1 {
            active_level_id: "execution".into(),
            dimensions: [
                "assigned_unit",
                "normal",
                "immediate",
                "local_write",
                "execution",
                "unit_closeout",
            ]
            .map(str::to_owned),
        },
        constraint_inputs: vec![],
        allowed_input_paths: vec!["inputs/*".into()],
        allowed_paths: vec![".handbook/project/local-output.json".into()],
        operation_classes: vec![
            LocalOperationClassV1::LocalValidation,
            LocalOperationClassV1::OwnerWrite,
        ],
        reserved_decisions: vec![
            LocalDecisionClassV1::PathExpansion,
            LocalDecisionClassV1::CharterReplacement,
            LocalDecisionClassV1::TermsReplacement,
        ],
        delegated_decisions: vec![LocalDecisionClassV1::InputCorrection],
        revocation_principal: "test-human".into(),
    };
    terms.approved_content_fingerprint = terms.fingerprint_approved_content().unwrap();
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
    .map(|coverage_id| CharterCoverageSubmission {
        coverage_id: coverage_id.into(),
        source_kind: CharterIntakeSourceKind::UserDeclaration,
        value_ref: format!("intake-values/{coverage_id}.json"),
        evidence_refs: vec![],
        confidence: "high".into(),
        freshness: None,
        sensitivity: "internal".into(),
        contradiction_refs: vec![],
        waiver_ref: None,
    })
    .collect();
    LocalInitialSetupRequestV1 {
        deferral: DeferralRequestInputV2::human_approved(
            "deferral-once",
            "The human supplied and approved the initial scoped deferral.",
        ),
        charter: NativeLocalCharterRequestV1 {
            request_key: "charter-once".into(),
            intake: CharterIntakeEnvelope {
                mode: CharterAcquisitionMode::Express,
                content,
                coverage,
                consumer: CharterIntakeConsumer {
                    kind: "sdk_consumer".into(),
                    id: "local-initial-test".into(),
                    version: "1.0.0".into(),
                },
                prompt_event_refs: vec!["decision.test.initial-charter".into()],
                finalized_at_utc: "2026-10-05T12:00:00Z".into(),
                expected_current_fingerprint: None,
            },
            human_approval: LocalCharterHumanApprovalV1 {
                accepted: true,
                principal_claim: "test-human".into(),
                agent_claim: "test-agent".into(),
                delegation_claim: "initial local Charter establishment".into(),
                source_ref: "decision.test.initial-charter".into(),
                approved_content_fingerprint: handbook_engine::charter_source_fingerprint(CHARTER)
                    .to_string(),
            },
        },
        terms,
    }
}

fn envelope_request() -> LocalEnvelopeRequestV1 {
    let profile = handbook_engine::local_authority::resolved_local_operating_profile().unwrap();
    let stack = profile.context_resolution();
    LocalEnvelopeRequestV1 {
        envelope_id: "actual-local-request".into(),
        objective_ref: "inputs/payload.json".into(),
        resolved_profile: LocalBindingV1 {
            reference: profile.exact_ref().to_string(),
            fingerprint: profile.resolved_profile_fingerprint().to_string(),
        },
        resolution_stack: LocalBindingV1 {
            reference: stack.exact_ref().to_string(),
            fingerprint: stack.definition_fingerprint().to_string(),
        },
        active_level_id: "execution".into(),
        dimensions: [
            "assigned_unit",
            "normal",
            "immediate",
            "local_write",
            "execution",
            "unit_closeout",
        ]
        .map(str::to_owned),
        parent: None,
        constraint_inputs: vec![],
        mutation_rules: vec![LocalMutationRuleInputV1 {
            rule_id: "local-owner-write".into(),
            effect: LocalMutationEffectV1::Allow,
            target_kind: "repository_path".into(),
            selector: ".handbook/project/local-output.json".into(),
        }],
        escalation_triggers: vec![],
    }
}

fn established(sdk: &HandbookSdkV1) -> (LocalInitialSetupLocatorV1, LocalInitialSetupExecutionV1) {
    let prepared = sdk
        .prepare_local_initial_setup(&supplied_request())
        .unwrap();
    let locator = prepared.locator().clone();
    let execution = sdk.establish_local_initial_setup(prepared);
    assert!(
        matches!(execution.deferral, DeferralExecutionV2::Committed { .. }),
        "deferral did not commit"
    );
    assert_eq!(
        execution.charter.as_ref().unwrap().disposition,
        LocalCharterDispositionV1::Committed,
        "Charter refusal: {:?}",
        execution.charter.as_ref().unwrap().refusal
    );
    assert_eq!(
        execution.terms.as_ref().unwrap().disposition,
        LocalAuthorityDispositionV1::Committed,
        "terms refusal: {:?}",
        execution.terms.as_ref().unwrap().refusal
    );
    (locator, execution)
}

#[test]
fn restored_composed_locator_rejects_duplicate_and_unknown_fields() {
    assert!(LocalInitialSetupLocatorV1::from_json_bytes(
        br#"{"schema_id":"handbook.sdk.local-initial-setup-locator","schema_id":"other"}"#
    )
    .is_err());
    assert!(LocalInitialSetupLocatorV1::from_json_bytes(br#"{"unexpected":true}"#).is_err());
}

#[test]
fn complete_locators_precede_effects_and_reopen_admits_native_bounds() {
    let root = tempdir().unwrap();
    let sdk = HandbookSdkV1::open(root.path());
    let prepared = sdk
        .prepare_local_initial_setup(&supplied_request())
        .unwrap();
    let bytes = prepared.locator().canonical_json_bytes().unwrap();
    let locator = LocalInitialSetupLocatorV1::from_json_bytes(&bytes).unwrap();
    assert_eq!(fs::read_dir(root.path()).unwrap().count(), 0);
    let execution = sdk.establish_local_initial_setup(prepared);
    assert!(execution.stopped.is_none(), "{:?}", execution.stopped);
    assert!(
        execution
            .terms
            .as_ref()
            .unwrap()
            .outcome
            .as_ref()
            .unwrap()
            .terms_committed,
        "terms owner did not commit"
    );
    drop(sdk);
    let reopened = HandbookSdkV1::open(root.path());
    let current = reopened
        .resolve_current_local_initial_setup(&locator)
        .unwrap();
    assert_eq!(
        current.charter.canonical_fingerprint,
        locator.charter.canonical_fingerprint
    );
    let envelope = reopened
        .admit_local_envelope_request(&locator, &envelope_request())
        .unwrap();
    assert_eq!(
        envelope.authorize_validation("unit_closeout").unwrap(),
        ContextResolutionValidationDecision::Authorized
    );
    assert_eq!(
        envelope.authorize_validation("slice_closeout").unwrap(),
        ContextResolutionValidationDecision::NotAuthorized
    );
    assert_eq!(
        envelope.evaluate_mutation("repository_path", ".handbook/project/local-output.json"),
        ContextResolutionMutationDecision::Allowed
    );
    assert_eq!(
        envelope.evaluate_mutation("repository_path", ".handbook/project/other.json"),
        ContextResolutionMutationDecision::Denied
    );
}

#[test]
fn missing_actual_initial_approval_refuses_without_effects() {
    let root = tempdir().unwrap();
    let sdk = HandbookSdkV1::open(root.path());
    let mut request = supplied_request();
    request.charter.human_approval.accepted = false;
    assert!(sdk.prepare_local_initial_setup(&request).is_err());
    assert_eq!(fs::read_dir(root.path()).unwrap().count(), 0);
    request = supplied_request();
    request.terms.human_approval_asserted = false;
    assert!(sdk.prepare_local_initial_setup(&request).is_err());
    assert_eq!(fs::read_dir(root.path()).unwrap().count(), 0);
    request = supplied_request();
    request.deferral.instruction.asserted_principal = Some("different-human".into());
    assert_eq!(
        sdk.prepare_local_initial_setup(&request).unwrap_err().code,
        "project_claim_conflict"
    );
    assert_eq!(fs::read_dir(root.path()).unwrap().count(), 0);
    request = supplied_request();
    request.deferral.instruction.asserted_agent = Some("different-agent".into());
    assert_eq!(
        sdk.prepare_local_initial_setup(&request).unwrap_err().code,
        "project_claim_conflict"
    );
    assert_eq!(fs::read_dir(root.path()).unwrap().count(), 0);
}

#[test]
fn exact_owner_replay_never_reactivates_revoked_terms() {
    let root = tempdir().unwrap();
    let sdk = HandbookSdkV1::open(root.path());
    let (locator, initial) = established(&sdk);
    let envelope = sdk
        .admit_local_envelope_request(&locator, &envelope_request())
        .unwrap();
    let original = initial.terms.unwrap().outcome.unwrap();
    let revoke = sdk
        .revoke_local_operating_authority(
            &locator,
            LocalRevokeRequestV1 {
                request_key: "revoke-once".into(),
                expected_current_fingerprint: original.outcome_fingerprint.clone(),
                principal: "test-human".into(),
                source_ref: "decision.test.revoke".into(),
                instruction: "Revoke the exact supplied local operating authority.".into(),
            },
        )
        .unwrap();
    assert_eq!(revoke.disposition, LocalAuthorityDispositionV1::Committed);
    assert!(envelope.authorize_validation("unit_closeout").is_err());
    let replay = sdk.resume_local_initial_setup_request(&locator).unwrap();
    assert_eq!(
        replay.terms.as_ref().unwrap().disposition,
        LocalAuthorityDispositionV1::Replayed
    );
    assert_eq!(
        replay.terms.unwrap().outcome.unwrap().outcome_fingerprint,
        original.outcome_fingerprint
    );
    assert!(sdk.resolve_current_local_initial_setup(&locator).is_err());
    assert!(sdk
        .admit_local_envelope_request(&locator, &envelope_request())
        .is_err());
}

#[test]
fn wrong_root_profile_constraints_and_wider_actual_reach_refuse() {
    let root = tempdir().unwrap();
    let sdk = HandbookSdkV1::open(root.path());
    let (locator, _) = established(&sdk);
    let other = tempdir().unwrap();
    let refusal = HandbookSdkV1::open(other.path())
        .admit_local_envelope_request(&locator, &envelope_request())
        .unwrap_err();
    assert_eq!(
        refusal.unresolved_request.unwrap().permitted_resolver_class,
        "missing_authority"
    );
    let mut request = envelope_request();
    request.resolved_profile.fingerprint = format!("sha256:{}", "0".repeat(64));
    assert!(sdk
        .admit_local_envelope_request(&locator, &request)
        .is_err());
    request = envelope_request();
    request.constraint_inputs.push(LocalBindingV1 {
        reference: "constraint.unsupplied".into(),
        fingerprint: format!("sha256:{}", "0".repeat(64)),
    });
    assert!(sdk
        .admit_local_envelope_request(&locator, &request)
        .is_err());
    request = envelope_request();
    request.dimensions[5] = "slice_closeout".into();
    assert!(sdk
        .admit_local_envelope_request(&locator, &request)
        .is_err());
    request = envelope_request();
    request.objective_ref = "other/payload.json".into();
    assert!(sdk
        .admit_local_envelope_request(&locator, &request)
        .is_err());
    request = envelope_request();
    request.mutation_rules[0].selector = ".handbook/project/*".into();
    assert!(sdk
        .admit_local_envelope_request(&locator, &request)
        .is_err());

    // Loss of the complete retained terms partition cannot turn this registered
    // project back into a fresh authority bootstrap.
    let partition = root.path().join(".handbook/local-operating");
    assert!(partition.starts_with(root.path()));
    fs::remove_dir_all(&partition).unwrap();
    assert!(
        handbook_engine::local_authority::NativeLocalAuthorityServiceV1::prepare(
            root.path(),
            &locator.charter,
            locator.terms.terms.clone(),
            locator
                .terms
                .expected_governance_relation_fingerprint
                .clone(),
        )
        .is_err()
    );
    assert!(sdk.resolve_current_local_initial_setup(&locator).is_err());
    let replay = sdk.resume_local_initial_setup_request(&locator).unwrap();
    assert!(replay.terms.unwrap().outcome.is_none());
    assert!(!partition.exists());
}

#[test]
fn actual_refusal_preserves_reserved_delegated_and_missing_resolution_context() {
    let root = tempdir().unwrap();
    let sdk = HandbookSdkV1::open(root.path());
    let mut supplied = supplied_request();
    supplied.terms.reserved_decisions = vec![LocalDecisionClassV1::HorizonExpansion];
    supplied.terms.delegated_decisions = vec![LocalDecisionClassV1::PathExpansion];
    supplied.terms.approved_content_fingerprint =
        supplied.terms.fingerprint_approved_content().unwrap();
    let prepared = sdk.prepare_local_initial_setup(&supplied).unwrap();
    let locator = prepared.locator().clone();
    let execution = sdk.establish_local_initial_setup(prepared);
    assert_eq!(
        execution.terms.unwrap().disposition,
        LocalAuthorityDispositionV1::Committed
    );

    let mut request = envelope_request();
    request.dimensions[5] = "slice_closeout".into();
    let refusal = sdk
        .admit_local_envelope_request(&locator, &request)
        .unwrap_err();
    let context = refusal.unresolved_request.unwrap();
    assert_eq!(
        context.requested_choice,
        LocalDecisionClassV1::HorizonExpansion
    );
    assert_eq!(context.permitted_resolver_class, "reserved_human_decision");
    assert_eq!(context.original_basis, locator.terms.request_fingerprint);
    assert_eq!(
        context.original_candidate["input"]["dimensions"]["values"][5],
        "slice_closeout"
    );

    request = envelope_request();
    request.objective_ref = "other/payload.json".into();
    let refusal = sdk
        .admit_local_envelope_request(&locator, &request)
        .unwrap_err();
    let context = refusal.unresolved_request.unwrap();
    assert_eq!(
        context.requested_choice,
        LocalDecisionClassV1::PathExpansion
    );
    assert_eq!(
        context.permitted_resolver_class,
        "delegated_project_agent_delivery_unimplemented"
    );
    assert_eq!(
        context.original_candidate["input"]["objective_ref"],
        "other/payload.json"
    );
    assert!(context
        .source_references
        .contains(&locator.terms.terms.approval_source_ref));

    request = envelope_request();
    request.constraint_inputs.push(LocalBindingV1 {
        reference: "constraint.unapproved".into(),
        fingerprint: format!("sha256:{}", "0".repeat(64)),
    });
    let refusal = sdk
        .admit_local_envelope_request(&locator, &request)
        .unwrap_err();
    let context = refusal.unresolved_request.unwrap();
    assert_eq!(
        context.requested_choice,
        LocalDecisionClassV1::InputCorrection
    );
    assert_eq!(context.permitted_resolver_class, "missing_authority");
    assert_eq!(
        context.original_candidate["input"]["constraint_inputs"][0]["reference"],
        "constraint.unapproved"
    );
}

#[test]
fn charter_commit_and_terms_refusal_remain_separately_attributable() {
    let workspace = tempdir().unwrap();
    let root = workspace.path().join("project");
    fs::create_dir(&root).unwrap();
    let recovery = workspace.path().join("recovery.json");
    let interrupted = std::process::Command::new(std::env::current_exe().unwrap())
        .args(["--exact", "native_process_child", "--nocapture"])
        .env("HANDBOOK_LOCAL_SETUP_CHILD_MODE", "crash_before_terms")
        .env("HANDBOOK_LOCAL_SETUP_CHILD_ROOT", &root)
        .env("HANDBOOK_LOCAL_SETUP_CHILD_RECOVERY", &recovery)
        .output()
        .unwrap();
    assert_eq!(
        interrupted.status.code(),
        Some(71),
        "{}",
        String::from_utf8_lossy(&interrupted.stderr)
    );
    let locator =
        LocalInitialSetupLocatorV1::from_json_bytes(&fs::read(recovery).unwrap()).unwrap();
    let sdk = HandbookSdkV1::open(&root);
    let operating_lock = root.join(".handbook/state/locks/local-operating.lock");
    assert!(!operating_lock.exists());
    let recovered = sdk.resume_local_initial_setup_request(&locator).unwrap();
    assert!(
        matches!(recovered.deferral, DeferralExecutionV2::Committed { .. }),
        "original deferral did not replay"
    );
    assert!(recovered.terms.as_ref().unwrap().outcome.is_none());
    assert_eq!(
        recovered.charter.unwrap().disposition,
        LocalCharterDispositionV1::Replayed
    );
    assert_eq!(
        recovered.terms.unwrap().refusal.unwrap().code,
        "required_lock_state_missing"
    );
    assert!(sdk.resolve_current_local_initial_setup(&locator).is_err());
    assert!(sdk
        .resolve_local_initial_setup_request(&locator)
        .unwrap()
        .terms
        .unwrap()
        .outcome
        .is_none());
    assert!(!operating_lock.exists());
    // Recovery never initiates an absent owner. Explicit preparation of the
    // supplied original terms remains eligible after the unreached stage.
    let prepared = handbook_engine::local_authority::NativeLocalAuthorityServiceV1::prepare(
        &root,
        &locator.charter,
        locator.terms.terms.clone(),
        locator
            .terms
            .expected_governance_relation_fingerprint
            .clone(),
    )
    .unwrap();
    assert_eq!(prepared.locator(), &locator.terms);
    assert!(!operating_lock.exists());
    let execution =
        handbook_engine::local_authority::NativeLocalAuthorityServiceV1::establish(&root, prepared);
    assert_eq!(
        execution.disposition,
        LocalAuthorityDispositionV1::Committed,
        "{execution:?}"
    );
    sdk.resolve_current_local_initial_setup(&locator).unwrap();
}

#[test]
fn native_process_child() {
    let Ok(mode) = std::env::var("HANDBOOK_LOCAL_SETUP_CHILD_MODE") else {
        return;
    };
    let root =
        std::path::PathBuf::from(std::env::var_os("HANDBOOK_LOCAL_SETUP_CHILD_ROOT").unwrap());
    let recovery =
        std::path::PathBuf::from(std::env::var_os("HANDBOOK_LOCAL_SETUP_CHILD_RECOVERY").unwrap());
    let sdk = HandbookSdkV1::open(&root);
    if mode == "write_consumer_inputs" {
        fs::write(&recovery, serde_json::to_vec(&supplied_request()).unwrap()).unwrap();
        fs::write(
            recovery.with_extension("envelope.json"),
            serde_json::to_vec(&envelope_request()).unwrap(),
        )
        .unwrap();
        return;
    }
    if mode == "crash" || mode == "crash_before_terms" {
        let request = supplied_request();
        let deferral = sdk
            .prepare_project_admission_deferral(&request.deferral)
            .unwrap();
        let binding = LocalProjectBindingV1 {
            initialization_claim: deferral.locator().initialization_claim.clone(),
        };
        let charter = handbook_engine::local_charter::NativeLocalCharterServiceV1::prepare(
            &root,
            &binding,
            request.charter,
        )
        .unwrap();
        let terms = handbook_engine::local_authority::NativeLocalAuthorityServiceV1::prepare(
            &root,
            charter.locator(),
            request.terms,
            handbook_engine::DefinitionFingerprint::from_bytes(
                &deferral
                    .prospective_deferred_basis()
                    .unwrap()
                    .canonical_json_bytes()
                    .unwrap(),
            )
            .as_str()
            .to_owned(),
        )
        .unwrap();
        let locator = LocalInitialSetupLocatorV1 {
            schema_id: "handbook.sdk.local-initial-setup-locator".into(),
            schema_version: "1.0.0".into(),
            deferral: deferral.locator().clone(),
            charter: charter.locator().clone(),
            terms: terms.locator().clone(),
        };
        let mut file = fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&recovery)
            .unwrap();
        std::io::Write::write_all(&mut file, &locator.canonical_json_bytes().unwrap()).unwrap();
        file.sync_all().unwrap();
        assert!(matches!(
            sdk.initialize_project_admission_deferral(deferral),
            DeferralExecutionV2::Committed { .. }
        ));
        let established =
            handbook_engine::local_charter::NativeLocalCharterServiceV1::establish(&root, charter);
        assert_eq!(
            established.disposition,
            LocalCharterDispositionV1::Committed,
            "{established:?}"
        );
        if mode == "crash_before_terms" {
            std::process::exit(71);
        }
        handbook_engine::local_authority::NativeLocalAuthorityServiceV1::establish_with_interruption_for_test(&root, terms, || std::process::exit(71));
        panic!("the real process interruption hook was not reached");
    }
    assert_eq!(mode, "reopen");
    let locator =
        LocalInitialSetupLocatorV1::from_json_bytes(&fs::read(&recovery).unwrap()).unwrap();
    let recovered = sdk.resume_local_initial_setup_request(&locator).unwrap();
    assert!(recovered.stopped.is_none(), "{:?}", recovered.stopped);
    assert!(
        recovered
            .terms
            .as_ref()
            .unwrap()
            .outcome
            .as_ref()
            .unwrap()
            .terms_committed,
        "original terms did not commit after reopen"
    );
    sdk.resolve_current_local_initial_setup(&locator).unwrap();
    let envelope = sdk
        .admit_local_envelope_request(&locator, &envelope_request())
        .unwrap();
    assert_eq!(
        envelope.authorize_validation("unit_closeout").unwrap(),
        ContextResolutionValidationDecision::Authorized
    );
}

#[test]
fn actual_process_interruption_reopens_and_finishes_only_original_terms() {
    let workspace = tempdir().unwrap();
    let root = workspace.path().join("project");
    fs::create_dir(&root).unwrap();
    let recovery = workspace.path().join("recovery.json");
    let child = |mode: &str| {
        std::process::Command::new(std::env::current_exe().unwrap())
            .args(["--exact", "native_process_child", "--nocapture"])
            .env("HANDBOOK_LOCAL_SETUP_CHILD_MODE", mode)
            .env("HANDBOOK_LOCAL_SETUP_CHILD_ROOT", &root)
            .env("HANDBOOK_LOCAL_SETUP_CHILD_RECOVERY", &recovery)
            .output()
            .unwrap()
    };
    let crashed = child("crash");
    assert_eq!(
        crashed.status.code(),
        Some(71),
        "{}",
        String::from_utf8_lossy(&crashed.stderr)
    );
    assert!(!root
        .join(".handbook/local-operating/v1/current.json")
        .exists());
    let original = fs::read(&recovery).unwrap();
    let reopened = child("reopen");
    assert!(
        reopened.status.success(),
        "{}\n{}",
        String::from_utf8_lossy(&reopened.stdout),
        String::from_utf8_lossy(&reopened.stderr)
    );
    assert_eq!(fs::read(&recovery).unwrap(), original);
    let sdk = HandbookSdkV1::open(&root);
    let locator = LocalInitialSetupLocatorV1::from_json_bytes(&original).unwrap();
    let current = sdk.resolve_current_local_initial_setup(&locator).unwrap();
    assert_eq!(current.terms.locator, locator.terms);
}

#[test]
fn paused_actual_validation_serializes_revoke_before_later_use_refuses() {
    use std::sync::mpsc;
    use std::time::Duration;
    let root = tempdir().unwrap();
    let sdk = HandbookSdkV1::open(root.path());
    let (locator, initial) = established(&sdk);
    let envelope = sdk
        .admit_local_envelope_request(&locator, &envelope_request())
        .unwrap();
    let (paused_tx, paused_rx) = mpsc::channel();
    let (release_tx, release_rx) = mpsc::channel();
    let consuming = envelope.clone();
    let use_thread = std::thread::spawn(move || {
        consuming.authorize_local_validation_with_pause_for_test("unit_closeout", || {
            paused_tx.send(()).unwrap();
            release_rx.recv().unwrap();
        })
    });
    paused_rx.recv_timeout(Duration::from_secs(10)).unwrap();
    let request = LocalRevokeRequestV1 {
        request_key: "paused-revoke".into(),
        expected_current_fingerprint: initial.terms.unwrap().outcome.unwrap().outcome_fingerprint,
        principal: "test-human".into(),
        source_ref: "decision.test.paused-revoke".into(),
        instruction: "Revoke the current terms.".into(),
    };
    let (done_tx, done_rx) = mpsc::channel();
    let root_path = root.path().to_path_buf();
    let revoke_locator = locator.clone();
    let revoke_thread = std::thread::spawn(move || {
        let result = HandbookSdkV1::open(root_path)
            .revoke_local_operating_authority(&revoke_locator, request);
        done_tx.send(()).unwrap();
        result
    });
    assert!(matches!(
        done_rx.recv_timeout(Duration::from_millis(100)),
        Err(mpsc::RecvTimeoutError::Timeout)
    ));
    release_tx.send(()).unwrap();
    assert_eq!(
        use_thread.join().unwrap().unwrap(),
        ContextResolutionValidationDecision::Authorized
    );
    assert_eq!(
        revoke_thread.join().unwrap().unwrap().disposition,
        LocalAuthorityDispositionV1::Committed
    );
    assert!(envelope.authorize_validation("unit_closeout").is_err());
}

#[test]
fn paused_actual_validation_detects_charter_byte_substitution() {
    use std::sync::mpsc;
    use std::time::Duration;
    let root = tempdir().unwrap();
    let sdk = HandbookSdkV1::open(root.path());
    let (locator, _) = established(&sdk);
    let envelope = sdk
        .admit_local_envelope_request(&locator, &envelope_request())
        .unwrap();
    let (paused_tx, paused_rx) = mpsc::channel();
    let (release_tx, release_rx) = mpsc::channel();
    let use_thread = std::thread::spawn(move || {
        envelope.authorize_local_validation_with_pause_for_test("unit_closeout", || {
            paused_tx.send(()).unwrap();
            release_rx.recv().unwrap();
        })
    });
    paused_rx.recv_timeout(Duration::from_secs(10)).unwrap();
    let canonical = root.path().join(".handbook/project/charter.yaml");
    let substituted = fs::write(&canonical, b"changed retained Charter bytes");
    release_tx.send(()).unwrap();
    let decision = use_thread.join().unwrap();
    if substituted.is_ok() {
        assert!(decision.is_err());
    } else {
        assert_eq!(
            decision.unwrap(),
            ContextResolutionValidationDecision::Authorized
        );
    }
    // Once the consuming guards are released, force the changed retained
    // Charter on every platform. Original owner history remains deliverable.
    let changed = b"changed retained Charter bytes";
    fs::write(&canonical, changed).unwrap();
    assert!(sdk.resolve_current_local_initial_setup(&locator).is_err());
    for original in [
        sdk.resolve_local_initial_setup_request(&locator).unwrap(),
        sdk.resume_local_initial_setup_request(&locator).unwrap(),
    ] {
        assert_eq!(
            original.charter.unwrap().disposition,
            LocalCharterDispositionV1::Replayed
        );
        assert_eq!(
            original.terms.as_ref().unwrap().disposition,
            LocalAuthorityDispositionV1::Replayed
        );
        assert!(original.terms.unwrap().outcome.is_some());
    }
    assert_eq!(fs::read(&canonical).unwrap(), changed);
}

#[test]
fn paused_actual_validation_prevents_or_detects_physical_root_substitution() {
    use std::sync::mpsc;
    use std::time::Duration;
    let workspace = tempdir().unwrap();
    let root = workspace.path().join("project");
    fs::create_dir(&root).unwrap();
    let sdk = HandbookSdkV1::open(&root);
    let (locator, _) = established(&sdk);
    let envelope = sdk
        .admit_local_envelope_request(&locator, &envelope_request())
        .unwrap();
    let (paused_tx, paused_rx) = mpsc::channel();
    let (release_tx, release_rx) = mpsc::channel();
    let use_thread = std::thread::spawn(move || {
        envelope.authorize_local_validation_with_pause_for_test("unit_closeout", || {
            paused_tx.send(()).unwrap();
            release_rx.recv().unwrap();
        })
    });
    paused_rx.recv_timeout(Duration::from_secs(10)).unwrap();
    let displaced = workspace.path().join("displaced-original");
    let substitution = fs::rename(&root, &displaced);
    if substitution.is_ok() {
        fs::create_dir(&root).unwrap();
    }
    release_tx.send(()).unwrap();
    let decision = use_thread.join().unwrap();
    if substitution.is_ok() {
        assert!(decision.is_err());
    } else {
        assert_eq!(
            decision.unwrap(),
            ContextResolutionValidationDecision::Authorized
        );
    }
}

#[test]
fn v3_noncommit_keeps_current_d_usable_then_establishment_refuses_stale_d_use() {
    use handbook_contracts::project_admission_governance::{
        EstablishmentFaultBoundaryV3, ProjectAdmissionEstablishmentV3,
    };
    use handbook_sdk::admission_governance::*;
    let root = tempdir().unwrap();
    let sdk = HandbookSdkV1::open(root.path());
    let (locator, _) = established(&sdk);
    let current = sdk.resolve_current_local_initial_setup(&locator).unwrap();
    let ProjectAdmissionScopeBasisV3::DeferredV2(basis) = current.governance else {
        panic!("expected current D")
    };
    let request = |key: &str| {
        DeferredEstablishmentRequestV3::human_approved(
            basis.project.clone(),
            ExpectedDeferredBasisV2 {
                outcome_fingerprint: basis.outcome_fingerprint.clone(),
                relation_fingerprint: basis.relation_fingerprint.clone(),
            },
            300,
            key,
            "The test human approved exact new establishment configuration and replacement terms.",
        )
        .unwrap()
    };
    let prepared = sdk
        .prepare_deferred_project_admission_governance(&request("positive-noncommit"))
        .unwrap();
    let governance_locator = prepared.locator().clone();
    let interrupted = ProjectAdmissionEstablishmentV3::new()
        .with_fault_for_testing(EstablishmentFaultBoundaryV3::Stage(2))
        .establish(root.path(), prepared);
    assert!(matches!(
        interrupted,
        DeferredEstablishmentExecutionV3::Uncertain { .. }
    ));
    assert!(matches!(
        sdk.resolve_deferred_admission_governance_request(&governance_locator),
        DeferredEstablishmentExecutionV3::NotCommitted { .. }
    ));
    sdk.resolve_current_local_initial_setup(&locator).unwrap();
    let envelope = sdk
        .admit_local_envelope_request(&locator, &envelope_request())
        .unwrap();
    assert_eq!(
        envelope.authorize_validation("unit_closeout").unwrap(),
        ContextResolutionValidationDecision::Authorized
    );
    let prepared = sdk
        .prepare_deferred_project_admission_governance(&request("establish-g"))
        .unwrap();
    assert!(matches!(
        sdk.establish_deferred_project_admission_governance(prepared),
        DeferredEstablishmentExecutionV3::Committed { .. }
    ));
    let refusal = sdk
        .resolve_current_local_initial_setup(&locator)
        .unwrap_err();
    assert_eq!(refusal.code, "unsupported_governance_basis");
    assert!(matches!(
        refusal.observed_governance_basis,
        Some(ProjectAdmissionScopeBasisV3::EstablishedFromDeferredV3(_))
    ));
    assert!(envelope.authorize_validation("unit_closeout").is_err());
    let replay = sdk.resume_local_initial_setup_request(&locator).unwrap();
    assert_eq!(replay.stopped.unwrap().code, "unsupported_governance_basis");
    assert!(replay.terms.unwrap().outcome.is_some());
}

#[test]
fn relation_substitution_after_v3_refuses_before_any_terms_intent() {
    use handbook_engine::local_authority::NativeLocalAuthorityServiceV1;
    use handbook_engine::local_charter::NativeLocalCharterServiceV1;
    use handbook_sdk::admission_governance::{
        ProjectAdmissionBindingV3, ProjectAdmissionScopeBasisV3, ADMISSION_GOVERNANCE_SCOPE_V1,
    };
    let root = tempdir().unwrap();
    let sdk = HandbookSdkV1::open(root.path());
    let request = supplied_request();
    let deferral = sdk
        .prepare_project_admission_deferral(&request.deferral)
        .unwrap();
    let relation = deferral
        .prospective_deferred_basis()
        .unwrap()
        .canonical_json_bytes()
        .unwrap();
    let expected = handbook_engine::DefinitionFingerprint::from_bytes(&relation)
        .as_str()
        .to_owned();
    let charter = NativeLocalCharterServiceV1::prepare(
        root.path(),
        &LocalProjectBindingV1 {
            initialization_claim: deferral.locator().initialization_claim.clone(),
        },
        request.charter,
    )
    .unwrap();
    let terms = NativeLocalAuthorityServiceV1::prepare(
        root.path(),
        charter.locator(),
        request.terms,
        expected,
    )
    .unwrap();
    let locator = LocalInitialSetupLocatorV1 {
        schema_id: "handbook.sdk.local-initial-setup-locator".into(),
        schema_version: "1.0.0".into(),
        deferral: deferral.locator().clone(),
        charter: charter.locator().clone(),
        terms: terms.locator().clone(),
    };
    let restored =
        LocalInitialSetupLocatorV1::from_json_bytes(&locator.canonical_json_bytes().unwrap())
            .unwrap();
    assert_eq!(fs::read_dir(root.path()).unwrap().count(), 0);
    assert!(matches!(
        sdk.initialize_project_admission_deferral(deferral),
        DeferralExecutionV2::Committed { .. }
    ));
    assert_eq!(
        NativeLocalCharterServiceV1::establish(root.path(), charter).disposition,
        LocalCharterDispositionV1::Committed
    );
    assert!(matches!(
        sdk.resolve_project_admission_scope_v3(
            &ProjectAdmissionBindingV3::ScopedChoiceV2(restored.deferral.project.clone()),
            ADMISSION_GOVERNANCE_SCOPE_V1,
            None,
        )
        .unwrap(),
        ProjectAdmissionScopeBasisV3::DeferredV2(_)
    ));
    let current_path = root
        .path()
        .join(".handbook/admission-governance/v2/scope-current.json");
    assert_eq!(fs::read(&current_path).unwrap(), relation);
    let mut changed: serde_json::Value = serde_json::from_slice(&relation).unwrap();
    changed["scope"] = "substituted-scope-after-positive-v3".into();
    fs::write(current_path, serde_json::to_vec(&changed).unwrap()).unwrap();
    let execution = NativeLocalAuthorityServiceV1::establish(root.path(), terms);
    assert_eq!(
        execution.disposition,
        LocalAuthorityDispositionV1::Refused,
        "{execution:?}"
    );
    assert!(execution.outcome.is_none());
    assert!(!root.path().join(".handbook/local-operating").exists());
    assert!(!root
        .path()
        .join(".handbook/state/locks/local-operating.lock")
        .exists());
}
