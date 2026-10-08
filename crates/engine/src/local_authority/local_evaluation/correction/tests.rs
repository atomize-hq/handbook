use super::*;

fn fp(c: &str) -> String {
    format!("sha256:{}", c.repeat(64))
}
fn candidate() -> LocalInputCorrectionCandidateV1 {
    let original = serde_json::json!({"amount":-1});
    let proposed = serde_json::json!({"amount":125});
    let original_fp = DefinitionFingerprint::from_bytes(&encode(&original).unwrap()).to_string();
    let proposed_fp = DefinitionFingerprint::from_bytes(&encode(&proposed).unwrap()).to_string();
    let binding = |fp: &str| serde_json::json!({"ref":format!("captured#{fp}"),"fingerprint":fp});
    let subject = |fp: &str| serde_json::json!({"subject":binding(fp),"matcher_inputs":[]});
    let case =
        |fp: &str| serde_json::json!({"case_id":"payload","case":binding(fp),"matcher_inputs":[]});
    let prospective = serde_json::json!({"request_key":"one-approved-correction","subject":subject(&proposed_fp),"case":case(&proposed_fp),
        "contract":{"ref":"contract@3.0.0","fingerprint":fp("6")},"requested_resolution":{},"formation":{"subject_fingerprint":proposed_fp}});
    let mut rejected = prospective.clone();
    rejected["subject"] = subject(&original_fp);
    rejected["case"] = case(&original_fp);
    let mut candidate = LocalInputCorrectionCandidateV1 {
        schema_ref: LOCAL_INPUT_CORRECTION_CANDIDATE_SCHEMA_V1.into(),
        owner_identity: "actual-contracts-owner".into(),
        project_identity: "actual-project".into(),
        original_result: LocalBindingV1 {
            reference: "result-original".into(),
            fingerprint: fp("1"),
        },
        rejected_candidate_fingerprint: DefinitionFingerprint::from_bytes(
            &encode(&rejected).unwrap(),
        )
        .to_string(),
        missing_condition: "subject_case_input_congruence".into(),
        original_input_fingerprint: DefinitionFingerprint::from_bytes(&encode(&original).unwrap())
            .to_string(),
        proposed_input_fingerprint: DefinitionFingerprint::from_bytes(&encode(&proposed).unwrap())
            .to_string(),
        formation_request_key: "one-approved-correction".into(),
        formation_request_fingerprint: fp("5"),
        original_state_path: String::new(),
        original_state_bytes_fingerprint: String::new(),
        original_state_identity: LocalBindingV1 {
            reference: String::new(),
            fingerprint: String::new(),
        },
        rejected_candidate: rejected,
        prospective_candidate: prospective,
        original_capture: original,
        proposed_capture: proposed,
    };
    let bytes = original_state_bytes(&candidate);
    let value: serde_json::Value = decode(&bytes).unwrap();
    candidate.original_state_identity = LocalBindingV1 {
        reference: value["identity"]["ref"].as_str().unwrap().into(),
        fingerprint: value["identity"]["fingerprint"].as_str().unwrap().into(),
    };
    candidate.original_state_bytes_fingerprint =
        DefinitionFingerprint::from_bytes(&bytes).to_string();
    candidate.original_state_path = format!(
        "{ORIGINAL_STATE_ROOT}/{}.json",
        &candidate.original_state_identity.fingerprint[7..]
    );
    candidate
}
fn original_state_bytes(candidate: &LocalInputCorrectionCandidateV1) -> Vec<u8> {
    let request = encode(&serde_json::json!({"inputs":[candidate.original_capture]})).unwrap();
    let mut state = serde_json::json!({"stage":"result_closed", "locator":{"owner_identity":candidate.owner_identity},
        "purpose":{"project_identity":candidate.project_identity}, "result":{"identity":{"ref":candidate.original_result.reference,"fingerprint":candidate.original_result.fingerprint}},
        "original_input":candidate.rejected_candidate,"origin_intent":{"request_jcs":String::from_utf8(request).unwrap()}});
    let identity =
        DefinitionFingerprint::from_bytes(&encode(&(ORIGINAL_STATE_DOMAIN, &state)).unwrap())
            .to_string();
    state["identity"] = serde_json::json!({"ref":format!("{ORIGINAL_STATE_DOMAIN}#{}",&identity[7..]),"fingerprint":identity});
    encode(&state).unwrap()
}

#[test]
fn correction_native_candidate_requires_owner_bytes_before_request_identity() {
    let candidate = candidate();
    let bytes = candidate.canonical_json_bytes().unwrap();
    let source = LocalInputCorrectionCandidateSourceV1 {
        record_fingerprint: DefinitionFingerprint::from_bytes(&bytes).to_string(),
        relative_path: format!(
            "{CANDIDATE_ROOT}/{}.json",
            &DefinitionFingerprint::from_bytes(&bytes).as_str()[7..]
        ),
        canonical_record_bytes: bytes,
    };
    assert_eq!(source.candidate().unwrap(), candidate);
    let dir = tempfile::tempdir().unwrap();
    let root = RetainedRepositoryRootV1::open_exact(dir.path()).unwrap();
    let reach = LocalEvaluationPathReachV1 {
        owner_write_paths: vec![],
        input_paths: vec![".handbook/**".into()],
    };
    assert!(pin_candidate_source(&root, &source, &reach).is_err());
    std::fs::create_dir_all(dir.path().join(CANDIDATE_ROOT)).unwrap();
    std::fs::write(
        dir.path().join(&source.relative_path),
        &source.canonical_record_bytes,
    )
    .unwrap();
    // A caller-written candidate record still cannot supply a nonexistent
    // original result. The actual native state file must also be retained.
    assert!(pin_candidate_source(&root, &source, &reach).is_err());
    std::fs::create_dir_all(dir.path().join(ORIGINAL_STATE_ROOT)).unwrap();
    std::fs::write(
        dir.path().join(&candidate.original_state_path),
        original_state_bytes(&candidate),
    )
    .unwrap();
    let checkpoint = pin_candidate_source(&root, &source, &reach).unwrap();
    checkpoint.validate().unwrap();
    std::fs::write(dir.path().join(&source.relative_path), b"{}").unwrap();
    assert!(checkpoint.validate().is_err());
}

#[test]
fn correction_candidate_rejects_duplicate_fields_wrong_path_and_equal_capture() {
    let mut candidate = candidate();
    candidate.proposed_input_fingerprint = candidate.original_input_fingerprint.clone();
    assert!(candidate.canonical_json_bytes().is_err());
    let source = LocalInputCorrectionCandidateSourceV1 {
        relative_path: format!("{CANDIDATE_ROOT}/{}.json", "0".repeat(64)),
        canonical_record_bytes: b"{\"schema_ref\":1,\"schema_ref\":2}".to_vec(),
        record_fingerprint: fp("0"),
    };
    assert!(source.candidate().is_err());
}

fn retained_request_projection() -> LocalInputCorrectionRequestRecordV1 {
    // Closed test projections are deliberately not owner-established permission.
    // These tests isolate immutable native head/history verification.
    let selected = LocalEvaluationReachV1::selected(&LocalEnvelopeCeilingV1 {
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
    })
    .unwrap();
    let candidate = candidate();
    let bytes = candidate.canonical_json_bytes().unwrap();
    let record_fp = DefinitionFingerprint::from_bytes(&bytes).to_string();
    let request = LocalInputCorrectionRequestV1 {
        request_key: "unresolved-choice".into(),
        submitter: "caller".into(),
        context_source_ref: "caller.supplied-context".into(),
        additional_context: "".into(),
        candidate_source: LocalInputCorrectionCandidateSourceV1 {
            relative_path: format!("{CANDIDATE_ROOT}/{}.json", &record_fp[7..]),
            canonical_record_bytes: bytes,
            record_fingerprint: record_fp,
        },
        formation: LocalEvaluationFormationRequestV1 {
            subject_ref: "inputs/payload.json".into(),
            subject_fingerprint: candidate.proposed_input_fingerprint,
            scope_fingerprint: fp("7"),
            objective: "Check the exact captured payload".into(),
            use_intent: LocalEvaluationUseIntentV1::Diagnostic,
            requested_reach: selected,
            reach: LocalEvaluationPathReachV1::default(),
            retention_until_utc: "2098-01-01T00:00:00Z".into(),
        },
        current_basis: LocalCheckCurrentBasisV1 {
            relation_fingerprint: fp("8"),
            governance_tree_fingerprint: fp("9"),
        },
    };
    let mut record = LocalInputCorrectionRequestRecordV1 {
        schema_ref: LOCAL_INPUT_CORRECTION_RULE_V1.into(),
        locator: request.locator(&fp("e")).unwrap(),
        request,
        head_fingerprint: String::new(),
        retained_at_utc: "2026-10-08T00:00:00Z".into(),
    };
    record.head_fingerprint = request_record_identity(&record).unwrap();
    record
}
fn on_stack(test: impl FnOnce() + Send + 'static) {
    std::thread::Builder::new()
        .stack_size(8 * 1024 * 1024)
        .spawn(test)
        .unwrap()
        .join()
        .unwrap();
}
fn decision_projection(
    request: &LocalInputCorrectionRequestRecordV1,
    key: &str,
    head: &str,
    outcome: LocalInputCorrectionDecisionV1,
) -> LocalInputCorrectionDecisionRecordV1 {
    let submission = LocalInputCorrectionSubmissionV1 {
        submission_key: key.into(),
        expected_head: head.into(),
        candidate_choice_fingerprint: request.request.candidate_source.record_fingerprint.clone(),
        deciding_actor: "named-decider".into(),
        submitter: "different-delivery-caller".into(),
        decision_source_ref: "decider.actual-response".into(),
        assurance: "local_advisory".into(),
        outcome,
        reason: "exact supplied choice".into(),
    };
    let mut record = LocalInputCorrectionDecisionRecordV1 {
        schema_ref: LOCAL_INPUT_CORRECTION_RULE_V1.into(),
        locator: submission.locator(&request.locator).unwrap(),
        submission,
        request_fingerprint: request.locator.request_fingerprint.clone(),
        decision_fingerprint: String::new(),
        recorded_at_utc: "2026-10-08T00:01:00Z".into(),
    };
    record.decision_fingerprint = decision_identity(&record).unwrap();
    record
}
fn retain_decision_projection(
    root: &RetainedRepositoryRootV1,
    request: &LocalInputCorrectionRequestRecordV1,
    record: &LocalInputCorrectionDecisionRecordV1,
    terminal: bool,
) {
    let bytes = encode(record).unwrap();
    root.create_directories(&format!(
        "{LOCAL_INPUT_CORRECTION_ROOT_V1}/requests/{}/decisions/{}",
        record.locator.request_key, record.locator.submission_key
    ))
    .unwrap();
    root.create_directories(&format!(
        "{LOCAL_INPUT_CORRECTION_ROOT_V1}/requests/{}/heads",
        record.locator.request_key
    ))
    .unwrap();
    write_exact(
        root,
        &decision_path(&record.locator, "request.json"),
        &bytes,
    )
    .unwrap();
    write_exact(
        root,
        &head_path(&request.locator, &record.submission.expected_head).unwrap(),
        &encode(&record.locator).unwrap(),
    )
    .unwrap();
    if terminal {
        write_exact(
            root,
            &decision_path(&record.locator, "outcome.json"),
            &bytes,
        )
        .unwrap();
        write_exact(
            root,
            &decision_path(&record.locator, "terminal.json"),
            &bytes,
        )
        .unwrap();
    }
}

#[test]
fn correction_head_retains_clarification_then_exact_approval_and_rejects_changed_locator() {
    on_stack(|| {
        let request = retained_request_projection();
        let dir = tempfile::tempdir().unwrap();
        let root = RetainedRepositoryRootV1::open_exact(dir.path()).unwrap();
        assert_eq!(
            request_head(&root, &request).unwrap(),
            (request.head_fingerprint.clone(), None)
        );
        let clarify = decision_projection(
            &request,
            "ask-for-source",
            &request.head_fingerprint,
            LocalInputCorrectionDecisionV1::Clarify,
        );
        retain_decision_projection(&root, &request, &clarify, true);
        assert_eq!(
            request_head(&root, &request).unwrap(),
            (clarify.decision_fingerprint.clone(), Some(clarify.clone()))
        );
        let approve = decision_projection(
            &request,
            "approve-source",
            &clarify.decision_fingerprint,
            LocalInputCorrectionDecisionV1::Approve,
        );
        let reach = LocalEvaluationPathReachV1 {
            owner_write_paths: vec![],
            input_paths: vec![".handbook/**".into()],
        };
        let dependencies = predecessor_decision_dependencies(
            &root,
            &request,
            &clarify.decision_fingerprint,
            &reach,
        )
        .unwrap();
        assert_eq!(dependencies.len(), 4);
        let files = pin_dependencies(&root, &dependencies).unwrap();
        retain_decision_projection(&root, &request, &approve, true);
        assert_eq!(
            request_head(&root, &request).unwrap(),
            (approve.decision_fingerprint.clone(), Some(approve.clone()))
        );
        assert_eq!(
            read_decision(&root, &request, &approve.locator).unwrap(),
            approve
        );
        let mut changed = approve.locator;
        changed.submission_fingerprint = fp("f");
        assert_eq!(
            read_decision(&root, &request, &changed).unwrap_err().code,
            "correction_decision_conflict"
        );
        assert!(predecessor_decision_dependencies(&root, &request, &fp("f"), &reach).is_err());
        files.validate().unwrap();
        std::fs::write(
            dir.path()
                .join(decision_path(&clarify.locator, "terminal.json")),
            b"{}",
        )
        .unwrap();
        assert!(files.validate().is_err());
    });
}

#[test]
fn correction_reserved_pending_head_has_no_approval_and_original_intent_is_uncertain() {
    on_stack(|| {
        let request = retained_request_projection();
        let dir = tempfile::tempdir().unwrap();
        let root = RetainedRepositoryRootV1::open_exact(dir.path()).unwrap();
        let approve = decision_projection(
            &request,
            "lost-response",
            &request.head_fingerprint,
            LocalInputCorrectionDecisionV1::Approve,
        );
        assert_eq!(
            require_recovery_intent(
                &root,
                &decision_path(&approve.locator, "request.json"),
                &decision_path(&approve.locator, "terminal.json"),
                "correction_decision"
            )
            .unwrap_err()
            .code,
            "correction_decision_absent"
        );
        retain_decision_projection(&root, &request, &approve, false);
        assert!(request_head(&root, &request).is_err());
        let pending = require_recovery_intent(
            &root,
            &decision_path(&approve.locator, "request.json"),
            &decision_path(&approve.locator, "terminal.json"),
            "correction_decision",
        )
        .unwrap_err();
        let outcome: LocalInputCorrectionExecutionV1<()> = execute(Err(pending), false, true);
        assert_eq!(outcome.disposition, LocalAuthorityDispositionV1::Uncertain);
        assert!(outcome.outcome.is_none());
    });
}

#[test]
fn correction_source_read_requires_declared_root_and_full_content_hashes() {
    let mut candidate = candidate();
    candidate.proposed_capture = serde_json::json!({"amount":126});
    assert_eq!(
        candidate.canonical_json_bytes().unwrap_err().code,
        "correction_candidate_content"
    );
    let candidate = self::candidate();
    let bytes = candidate.canonical_json_bytes().unwrap();
    let record_fingerprint = DefinitionFingerprint::from_bytes(&bytes).to_string();
    let source = LocalInputCorrectionCandidateSourceV1 {
        relative_path: format!("{CANDIDATE_ROOT}/{}.json", &record_fingerprint[7..]),
        canonical_record_bytes: bytes,
        record_fingerprint,
    };
    let dir = tempfile::tempdir().unwrap();
    let root = RetainedRepositoryRootV1::open_exact(dir.path()).unwrap();
    let reach = LocalEvaluationPathReachV1 {
        owner_write_paths: vec![],
        input_paths: vec![format!("{LOCAL_EVALUATION_ROOT_V1}/**")],
    };
    assert_eq!(
        pin_candidate_source(&root, &source, &reach)
            .err()
            .unwrap()
            .code,
        "required_input_reach"
    );
}
