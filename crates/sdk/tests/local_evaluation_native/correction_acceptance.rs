//! Packet 4 uses actual native owners; supplied actors and choices are test data.
use super::*;
use handbook_engine::local_authority::local_evaluation::correction::{
    LocalInputCorrectionDecisionV1, NativeLocalInputCorrectionV1,
};

fn capture(content: Value) -> DockInlineJsonInputV1 {
    let manifest = production_manifest();
    let role = &manifest.supported_inputs()[0];
    DockInlineJsonInputV1::new(
        role.role().clone(),
        role.media_type(),
        role.schema().clone(),
        content,
    )
    .unwrap()
}

fn followup(
    fixture: &Fixture,
    original: &LocalEvaluationSnapshotV3,
    original_capture: &DockInlineJsonInputV1,
    corrected_capture: &DockInlineJsonInputV1,
    action: LocalEvaluationFollowupActionV1,
    key: &str,
) -> LocalEvaluationFollowupInputV1 {
    let mut input = original.original_input.clone();
    input.request_key = key.into();
    input.expected_lifecycle_fingerprint = fixture
        .sdk
        .current_local_evaluation_lifecycle(&input.contract)
        .unwrap()
        .lifecycle_fingerprint()
        .clone();
    input.subject = corrected_capture.captured_evaluation_subject_v1().unwrap();
    input.case = corrected_capture
        .captured_evaluation_case_v1(input.case.case_id().clone())
        .unwrap();
    input.formation.subject_ref = input.subject.subject().reference().into();
    input.formation.subject_fingerprint = input.subject.subject().fingerprint().as_str().into();
    LocalEvaluationFollowupInputV1 {
        action,
        original: original.result_locator().unwrap(),
        original_capture: serde_json::to_value(original_capture).unwrap(),
        capture: serde_json::to_value(corrected_capture).unwrap(),
        input,
    }
}

fn issue(
    fixture: &Fixture,
    snapshot: LocalEvaluationSnapshotV3,
    capture: DockInlineJsonInputV1,
) -> Formed {
    let request = snapshot.authorization_request(
        LocalEvaluationEnvelopeV1::local(
            &snapshot.purpose.subject_ref,
            snapshot.plan.selections[0].requested_reach.clone(),
            &fixture.authority.terms,
        ),
        engine_reach(),
    );
    let issuance = fixture
        .sdk
        .local_evaluation_authorization_locator(&fixture.authority, &request)
        .unwrap();
    let issued = fixture
        .sdk
        .issue_local_evaluation_authorization(&fixture.authority, &request);
    assert!(issued.binding.is_some(), "{issued:?}");
    Formed {
        formation: snapshot.original_input.formation.clone(),
        process_request: Some(process_request(&snapshot, capture)),
        snapshot,
        request,
        issuance,
    }
}

fn complete(fixture: &Fixture, formed: &Formed) -> LocalEvaluationSnapshotV3 {
    assert_eq!(
        fixture.assess(formed).stage,
        LocalEvaluationStageV3::Attemptable
    );
    let session = fixture.session(formed);
    fixture
        .sdk
        .commit_local_evaluation_collection(&session)
        .unwrap();
    match fixture
        .sdk
        .execute_local_evaluation_once(
            &session,
            formed.process_request.as_ref().unwrap(),
            &dock_reach(),
            &DockCancellationV1::new(),
        )
        .unwrap()
    {
        DockLocalEvaluationAttemptOutcomeV1::Terminal { terminal, .. } => {
            assert_eq!(terminal.kind, "completed")
        }
        other => panic!("actual native attempt required: {other:?}"),
    }
    finish(fixture, formed);
    fixture
        .sdk
        .recover_local_evaluation(&formed.snapshot.locator)
        .unwrap()
}

#[test]
fn native_packet4_routine_correction_and_repeat_preserve_original_truth() {
    on_stack(|| {
        let _package = InstalledProductionPackage::install();
        let fixture = Fixture::new();
        let mut negative = supplied_validation_content();
        negative["instance"]["value"]["amount"] = json!(-1);
        negative["instance"]["instance_fingerprint"] =
            json!(value_fingerprint(&negative["instance"]["value"]));
        let old_capture = capture(negative.clone());
        let old = fixture.form(
            "packet4-native-original-negative",
            negative,
            production_manifest().resolution_ceiling().clone(),
            LocalEvaluationUseIntentV1::DecisionSeeking,
        );
        let original = complete(&fixture, &old);
        let original_result = original.result.as_ref().unwrap();
        assert_eq!(original_result.gate, LocalEvaluationGateV3::Fail);
        let original_bytes = original_result.canonical_json_bytes().unwrap();
        let original_snapshot_bytes = std::fs::read(fixture.root.path().join(format!(
            "{LOCAL_EVALUATION_OWNER_ROOT_V3}/records/{}.json",
            original.identity.fingerprint().as_str().trim_start_matches("sha256:"))))
        .unwrap();
        let corrected_capture = capture(supplied_validation_content());
        let input = followup(
            &fixture,
            &original,
            &old_capture,
            &corrected_capture,
            LocalEvaluationFollowupActionV1::CorrectedInput,
            "packet4-native-routine-correction",
        );
        let prepared = fixture
            .sdk
            .prepare_local_evaluation_followup(&fixture.initial, &fixture.authority, input.clone())
            .unwrap();
        let request = prepared.request_locator();
        assert!(fixture
            .sdk
            .recover_local_evaluation_request(&request)
            .unwrap()
            .is_none());
        let corrected = fixture
            .sdk
            .form_local_evaluation_followup(&fixture.authority, prepared)
            .unwrap();
        let replay = fixture
            .sdk
            .prepare_local_evaluation_followup(&fixture.initial, &fixture.authority, input.clone())
            .unwrap();
        assert_eq!(
            fixture
                .sdk
                .form_local_evaluation_followup(&fixture.authority, replay)
                .unwrap(),
            corrected
        );
        let lineage = fixture
            .sdk
            .read_local_evaluation_lineage(&corrected.locator)
            .unwrap()
            .unwrap();
        assert_eq!(lineage.original, original.result_locator().unwrap());
        assert_eq!(
            lineage.action,
            LocalEvaluationFollowupActionV1::CorrectedInput
        );
        assert_eq!(lineage.evaluation, corrected.locator);
        assert!(
            lineage.decision_provenance.is_none(),
            "routine correction needs no decision ceremony"
        );
        let corrected = complete(
            &fixture,
            &issue(&fixture, corrected, corrected_capture.clone()),
        );
        assert_eq!(
            corrected.result.as_ref().unwrap().gate,
            LocalEvaluationGateV3::Pass
        );
        assert_ne!(corrected.locator.occurrence, original.locator.occurrence);
        assert_ne!(
            corrected.purpose.content_fingerprint,
            original.purpose.content_fingerprint
        );
        let repeat = followup(
            &fixture,
            &corrected,
            &corrected_capture,
            &corrected_capture,
            LocalEvaluationFollowupActionV1::FreshRepeat,
            "packet4-native-deliberate-repeat",
        );
        let repeated = fixture
            .sdk
            .form_local_evaluation_followup(
                &fixture.authority,
                fixture
                    .sdk
                    .prepare_local_evaluation_followup(&fixture.initial, &fixture.authority, repeat)
                    .unwrap(),
            )
            .unwrap();
        assert_eq!(
            repeated.original_input.subject,
            corrected.original_input.subject
        );
        assert_ne!(repeated.locator.occurrence, corrected.locator.occurrence);
        assert_ne!(
            repeated.purpose.occurrence_identity,
            corrected.purpose.occurrence_identity
        );
        assert_eq!(
            repeated.purpose.content_fingerprint,
            corrected.purpose.content_fingerprint
        );
        assert_ne!(
            repeated.purpose.fingerprint().unwrap(),
            corrected.purpose.fingerprint().unwrap()
        );
        let repeated = complete(&fixture, &issue(&fixture, repeated, corrected_capture));
        assert_eq!(
            repeated.result.as_ref().unwrap().gate,
            LocalEvaluationGateV3::Pass
        );
        assert_ne!(repeated.issuance_locator, corrected.issuance_locator);
        let reopened = HandbookSdkV1::open(fixture.root.path());
        assert_eq!(
            reopened
                .recover_local_evaluation_request(&request)
                .unwrap()
                .unwrap(),
            corrected
        );
        assert_eq!(
            reopened
                .recover_local_evaluation(&original.locator)
                .unwrap(),
            original
        );
        assert_eq!(
            reopened
                .read_local_evaluation_result_history(&original.result_locator().unwrap())
                .unwrap()
                .result
                .canonical_json_bytes()
                .unwrap(),
            original_bytes
        );
        assert_eq!(
            std::fs::read(fixture.root.path().join(format!(
            "{LOCAL_EVALUATION_OWNER_ROOT_V3}/records/{}.json",
            original.identity.fingerprint().as_str().trim_start_matches("sha256:"))))
            .unwrap(),
            original_snapshot_bytes
        );
        let mut conflicting = input;
        conflicting
            .input
            .formation
            .objective
            .push_str(" conflicting reuse");
        let result = reopened.prepare_local_evaluation_followup(
            &fixture.initial,
            &fixture.authority,
            conflicting,
        );
        match result {
            Ok(prepared) => assert!(reopened
                .form_local_evaluation_followup(&fixture.authority, prepared)
                .is_err()),
            Err(_) => (),
        }
    });
}

fn delegation(fixture: &Fixture) -> LocalInputCorrectionDelegationV1 {
    let authority = &fixture.authority;
    let mut input = LocalInputCorrectionDelegationV1 {
        request_key: "packet4-native-upfront-delegation".into(),
        principal: authority.terms.principal.clone(),
        decision_agent: "packet4.decider".into(),
        executor: authority.terms.agent.clone(),
        approval_source_ref: "decision.packet4.illustrative-exact-initial-human-binding".into(),
        human_approval_asserted: true,
        approved_content_fingerprint: String::new(),
        expected_initial_fingerprint: authority
            .predecessor
            .terms
            .expected_predecessor_fingerprint
            .clone(),
        expected_local_check_fingerprint: authority.terms.expected_predecessor_fingerprint.clone(),
        expected_evaluation_fingerprint: fixture
            .sdk
            .recover_local_evaluation_terms(authority)
            .outcome
            .unwrap()
            .outcome_fingerprint,
        expected_charter_fingerprint: authority.terms.expected_charter_fingerprint.clone(),
        decision_class: LocalDecisionClassV1::InputCorrection,
        assurance: "local_advisory".into(),
        requested_reach: LocalEvaluationReachV1::selected(&authority.terms.ceiling).unwrap(),
        constraint_inputs: authority.terms.constraint_inputs.clone(),
        reach: engine_reach(),
        retention_until_utc: HORIZON.into(),
    };
    input.approved_content_fingerprint = input.fingerprint_approved_content().unwrap();
    input
}

fn request(
    fixture: &Fixture,
    delegation: &LocalInputCorrectionDelegationLocatorV1,
    prepared: &PreparedLocalEvaluationFollowupV1,
    original: &LocalEvaluationSnapshotV3,
    key: &str,
) -> LocalInputCorrectionRequestRecordV1 {
    let mut rejected = prepared.input().input.clone();
    rejected.subject = original.original_input.subject.clone();
    rejected.case = original.original_input.case.clone();
    assert!(
        fixture
            .sdk
            .prepare_local_evaluation(&fixture.initial, &fixture.authority, rejected.clone())
            .is_err(),
        "actual inconsistent candidate must refuse before Purpose allocation"
    );
    let source = fixture
        .sdk
        .retain_local_input_correction_candidate(
            &fixture.initial,
            &fixture.authority,
            prepared,
            rejected,
        )
        .unwrap();
    let input = LocalInputCorrectionRequestV1 {
        request_key: key.into(),
        submitter: fixture.authority.terms.agent.clone(),
        context_source_ref: "context.packet4.caller-carried-unresolved-choice".into(),
        additional_context:
            "The capture correction is unresolved; supplied context grants nothing.".into(),
        candidate_source: source,
        formation: prepared.formation_request().clone(),
        current_basis: prepared.current_basis().clone(),
    };
    if key == "packet4-decision-unresolved" {
        let adopted = fixture
            .sdk
            .recover_local_input_correction_delegation(&fixture.authority, delegation)
            .outcome
            .unwrap();
        let locator = input.locator(&adopted.delegation_fingerprint).unwrap();
        assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            NativeLocalInputCorrectionV1::retain_request_interrupted(
                fixture.root.path(),
                &fixture.authority,
                delegation,
                &input.current_basis,
                &input,
                || panic!("retained request response interruption"),
            )
        }))
        .is_err());
        let recovered = fixture.sdk.recover_local_input_correction_request(
            &fixture.authority,
            delegation,
            &locator,
        );
        assert!(recovered.outcome.is_none());
        assert!(recovered.refusal.unwrap().code.ends_with("pending"));
    }
    let record = fixture
        .sdk
        .retain_local_input_correction_request(
            &fixture.initial,
            &fixture.authority,
            delegation,
            &input,
        )
        .unwrap();
    record
        .outcome
        .unwrap_or_else(|| panic!("{:?}", record.refusal))
}

fn submission(
    fixture: &Fixture,
    request: &LocalInputCorrectionRequestRecordV1,
    head: &str,
    key: &str,
    outcome: LocalInputCorrectionDecisionV1,
) -> LocalInputCorrectionSubmissionV1 {
    LocalInputCorrectionSubmissionV1 {
        submission_key: key.into(),
        expected_head: head.into(),
        candidate_choice_fingerprint: request.request.candidate_source.record_fingerprint.clone(),
        deciding_actor: "packet4.decider".into(),
        submitter: fixture.authority.terms.agent.clone(),
        decision_source_ref: format!("decision.packet4.illustrative-{key}"),
        assurance: "local_advisory".into(),
        outcome,
        reason: "Supplied exact bounded decision; shipment and completion remain unassessed."
            .into(),
    }
}

#[test]
fn native_packet4_upfront_distinct_decider_and_interrupted_single_correction() {
    on_stack(|| {
        let _package = InstalledProductionPackage::install();
        let fixture = Fixture::new();
        let approved = delegation(&fixture);
        let mut changed = approved.clone();
        changed.decision_agent = "packet4.substituted-decider".into();
        assert!(fixture
            .sdk
            .prepare_local_input_correction_delegation(
                &fixture.initial,
                &fixture.authority,
                changed
            )
            .is_err());
        let mut self_approved = approved.clone();
        self_approved.principal = self_approved.decision_agent.clone();
        self_approved.approved_content_fingerprint =
            self_approved.fingerprint_approved_content().unwrap();
        assert!(fixture
            .sdk
            .prepare_local_input_correction_delegation(
                &fixture.initial,
                &fixture.authority,
                self_approved
            )
            .is_err());
        let mut executor_decides = approved.clone();
        executor_decides.decision_agent = executor_decides.executor.clone();
        executor_decides.approved_content_fingerprint =
            executor_decides.fingerprint_approved_content().unwrap();
        assert!(fixture
            .sdk
            .prepare_local_input_correction_delegation(
                &fixture.initial,
                &fixture.authority,
                executor_decides
            )
            .is_ok());
        let current_path = fixture
            .root
            .path()
            .join(".handbook/local-operating/local-evaluation-v1/current.json");
        let selected_executor_bytes = std::fs::read(&current_path).unwrap();
        let prepared_grant = fixture
            .sdk
            .prepare_local_input_correction_delegation(
                &fixture.initial,
                &fixture.authority,
                approved.clone(),
            )
            .unwrap();
        let grant = prepared_grant.locator().clone();
        let adopted = fixture
            .sdk
            .adopt_local_input_correction_delegation(&fixture.authority, prepared_grant);
        assert!(adopted.outcome.is_some(), "{adopted:?}");
        assert_ne!(grant.delegation.executor, grant.delegation.decision_agent);
        assert_eq!(
            std::fs::read(&current_path).unwrap(),
            selected_executor_bytes
        );
        assert!(
            fixture
                .sdk
                .prepare_local_input_correction_delegation(
                    &fixture.initial,
                    &fixture.authority,
                    approved
                )
                .is_err(),
            "binding replacement is unsupported even with a new preparation"
        );
        assert!(
            !fixture
                .root
                .path()
                .join(".handbook/local-operating/local-evaluation-v1/input-correction-v1/requests")
                .exists(),
            "upfront adoption needs no unresolved request"
        );
        let mut negative = supplied_validation_content();
        negative["instance"]["value"]["amount"] = json!(-1);
        negative["instance"]["instance_fingerprint"] =
            json!(value_fingerprint(&negative["instance"]["value"]));
        let old_capture = capture(negative.clone());
        let old = fixture.form(
            "packet4-decision-original-negative",
            negative,
            production_manifest().resolution_ceiling().clone(),
            LocalEvaluationUseIntentV1::DecisionSeeking,
        );
        let original = complete(&fixture, &old);
        let original_bytes = original
            .result
            .as_ref()
            .unwrap()
            .canonical_json_bytes()
            .unwrap();
        let corrected_capture = capture(supplied_validation_content());
        let input = followup(
            &fixture,
            &original,
            &old_capture,
            &corrected_capture,
            LocalEvaluationFollowupActionV1::CorrectedInput,
            "packet4-decision-approved-formation",
        );
        let prepared = fixture
            .sdk
            .prepare_local_evaluation_followup(&fixture.initial, &fixture.authority, input.clone())
            .unwrap();
        let original_request = prepared.request_locator();
        let requested = request(
            &fixture,
            &grant,
            &prepared,
            &original,
            "packet4-decision-unresolved",
        );
        let clarify = submission(
            &fixture,
            &requested,
            &requested.head_fingerprint,
            "packet4-decision-clarify",
            LocalInputCorrectionDecisionV1::Clarify,
        );
        let clarification = fixture
            .sdk
            .submit_local_input_correction_decision(
                &fixture.initial,
                &fixture.authority,
                &grant,
                &requested.locator,
                &clarify,
            )
            .unwrap();
        let clarification = clarification
            .outcome
            .unwrap_or_else(|| panic!("{:?}", clarification.refusal));
        assert!(fixture
            .sdk
            .recover_local_evaluation_request(&original_request)
            .unwrap()
            .is_none());
        assert!(fixture
            .sdk
            .form_approved_local_input_correction(
                &fixture.initial,
                &fixture.authority,
                &grant,
                &requested.locator,
                &clarification.locator,
                prepared.clone()
            )
            .is_err());
        let stale = submission(
            &fixture,
            &requested,
            &requested.head_fingerprint,
            "packet4-decision-stale-head",
            LocalInputCorrectionDecisionV1::Approve,
        );
        assert!(fixture
            .sdk
            .submit_local_input_correction_decision(
                &fixture.initial,
                &fixture.authority,
                &grant,
                &requested.locator,
                &stale
            )
            .unwrap()
            .outcome
            .is_none());
        let approve = submission(
            &fixture,
            &requested,
            &clarification.decision_fingerprint,
            "packet4-decision-approve",
            LocalInputCorrectionDecisionV1::Approve,
        );
        let mut wrong = approve.clone();
        wrong.deciding_actor = fixture.authority.terms.agent.clone();
        assert!(
            fixture
                .sdk
                .submit_local_input_correction_decision(
                    &fixture.initial,
                    &fixture.authority,
                    &grant,
                    &requested.locator,
                    &wrong
                )
                .unwrap()
                .outcome
                .is_none(),
            "delivery executor cannot become decider"
        );
        let original_decision = approve.locator(&requested.locator).unwrap();
        assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            NativeLocalInputCorrectionV1::submit_decision_interrupted(
                fixture.root.path(),
                &fixture.authority,
                &grant,
                &requested.locator,
                prepared.current_basis(),
                &approve,
                || panic!("retained decision response interruption"),
            )
        }))
        .is_err());
        let decision_path = fixture.root.path().join(format!(
            ".handbook/local-operating/local-evaluation-v1/input-correction-v1/requests/{}/decisions/{}/request.json",
            original_decision.request_key, original_decision.submission_key));
        let interrupted_decision_bytes = std::fs::read(&decision_path).unwrap();
        let pending = fixture.sdk.recover_local_input_correction_decision(
            &fixture.authority,
            &grant,
            &requested.locator,
            &original_decision,
        );
        assert!(pending.outcome.is_none());
        assert!(pending.refusal.unwrap().code.ends_with("pending"));
        let approved = fixture
            .sdk
            .submit_local_input_correction_decision(
                &fixture.initial,
                &fixture.authority,
                &grant,
                &requested.locator,
                &approve,
            )
            .unwrap();
        let approved = approved
            .outcome
            .unwrap_or_else(|| panic!("{:?}", approved.refusal));
        assert_eq!(
            approved.canonical_json_bytes().unwrap(),
            interrupted_decision_bytes
        );
        assert_eq!(
            fixture
                .sdk
                .submit_local_input_correction_decision(
                    &fixture.initial,
                    &fixture.authority,
                    &grant,
                    &requested.locator,
                    &approve
                )
                .unwrap()
                .outcome
                .unwrap(),
            approved
        );
        let mut conflict = approve.clone();
        conflict.reason.push_str(" conflicting reuse");
        assert!(fixture
            .sdk
            .submit_local_input_correction_decision(
                &fixture.initial,
                &fixture.authority,
                &grant,
                &requested.locator,
                &conflict
            )
            .unwrap()
            .outcome
            .is_none());
        let owner = ContractOwnerRepositoryV1::open_existing_for_local_use_v3(fixture.root.path())
            .unwrap()
            .with_fault_for_testing(OwnerRepositoryFaultBoundaryV1::CommittedRenameDurable);
        let interrupted = NativeLocalInputCorrectionV1::with_admission(
            fixture.root.path(),
            &fixture.authority,
            &grant,
            &requested.locator,
            &approved.locator,
            prepared.current_basis(),
            prepared.formation_request(),
            &fixture.authority.terms.agent,
            |admission| {
                Ok(owner.form_decided_local_evaluation_followup_v1(prepared.clone(), admission))
            },
        )
        .unwrap();
        assert_eq!(
            interrupted.unwrap_err().kind(),
            ContractOwnerRepositoryErrorKindV1::DurabilityUncertain
        );
        let recovered = HandbookSdkV1::open(fixture.root.path())
            .recover_local_evaluation_request(&original_request)
            .unwrap()
            .unwrap();
        assert_eq!(recovered.ordinal, original.ordinal + 1);
        let repeated_delivery = fixture
            .sdk
            .form_approved_local_input_correction(
                &fixture.initial,
                &fixture.authority,
                &grant,
                &requested.locator,
                &approved.locator,
                prepared,
            )
            .unwrap();
        assert_eq!(repeated_delivery, recovered);
        let lineage = fixture
            .sdk
            .read_local_evaluation_lineage(&recovered.locator)
            .unwrap()
            .unwrap();
        assert!(lineage.decision_provenance.is_some());
        assert!(lineage
            .dependencies
            .iter()
            .any(|d| d.path.contains("input-correction-v1/delegation")));
        assert!(lineage
            .dependencies
            .iter()
            .any(|d| d.path.contains("input-correction-v1/requests")));
        let corrected = complete(&fixture, &issue(&fixture, recovered, corrected_capture));
        assert_eq!(
            corrected.result.as_ref().unwrap().gate,
            LocalEvaluationGateV3::Pass
        );
        assert_eq!(
            fixture
                .sdk
                .read_local_evaluation_result_history(&original.result_locator().unwrap())
                .unwrap()
                .result
                .canonical_json_bytes()
                .unwrap(),
            original_bytes
        );
        assert_eq!(
            fixture
                .sdk
                .recover_local_input_correction_decision(
                    &fixture.authority,
                    &grant,
                    &requested.locator,
                    &approved.locator
                )
                .outcome
                .unwrap(),
            approved
        );
        let deny_input = followup(
            &fixture,
            &original,
            &old_capture,
            &capture(supplied_validation_content()),
            LocalEvaluationFollowupActionV1::CorrectedInput,
            "packet4-decision-denied-formation",
        );
        let denied_prepared = fixture
            .sdk
            .prepare_local_evaluation_followup(&fixture.initial, &fixture.authority, deny_input)
            .unwrap();
        let denied_request = denied_prepared.request_locator();
        let denied = request(
            &fixture,
            &grant,
            &denied_prepared,
            &original,
            "packet4-decision-denied-request",
        );
        let deny = submission(
            &fixture,
            &denied,
            &denied.head_fingerprint,
            "packet4-decision-deny",
            LocalInputCorrectionDecisionV1::Deny,
        );
        let deny = fixture
            .sdk
            .submit_local_input_correction_decision(
                &fixture.initial,
                &fixture.authority,
                &grant,
                &denied.locator,
                &deny,
            )
            .unwrap()
            .outcome
            .unwrap();
        assert!(fixture
            .sdk
            .form_approved_local_input_correction(
                &fixture.initial,
                &fixture.authority,
                &grant,
                &denied.locator,
                &deny.locator,
                denied_prepared
            )
            .is_err());
        assert!(fixture
            .sdk
            .recover_local_evaluation_request(&denied_request)
            .unwrap()
            .is_none());
        assert_eq!(
            std::fs::read(&current_path).unwrap(),
            selected_executor_bytes
        );

        let race_input = followup(
            &fixture,
            &original,
            &old_capture,
            &capture(supplied_validation_content()),
            LocalEvaluationFollowupActionV1::CorrectedInput,
            "packet4-decision-concurrent-formation",
        );
        let race_prepared = fixture
            .sdk
            .prepare_local_evaluation_followup(&fixture.initial, &fixture.authority, race_input)
            .unwrap();
        let raced_request = request(
            &fixture,
            &grant,
            &race_prepared,
            &original,
            "packet4-decision-concurrent-request",
        );
        let barrier = std::sync::Arc::new(std::sync::Barrier::new(2));
        let mut workers = Vec::new();
        for (key, choice) in [
            (
                "packet4-decision-concurrent-approve",
                LocalInputCorrectionDecisionV1::Approve,
            ),
            (
                "packet4-decision-concurrent-deny",
                LocalInputCorrectionDecisionV1::Deny,
            ),
        ] {
            let sdk = HandbookSdkV1::open(fixture.root.path());
            let initial = fixture.initial.clone();
            let authority = fixture.authority.clone();
            let grant = grant.clone();
            let request = raced_request.locator.clone();
            let input = submission(
                &fixture,
                &raced_request,
                &raced_request.head_fingerprint,
                key,
                choice,
            );
            let barrier = barrier.clone();
            workers.push(std::thread::spawn(move || {
                barrier.wait();
                sdk.submit_local_input_correction_decision(
                    &initial, &authority, &grant, &request, &input,
                )
                .unwrap()
            }));
        }
        let results: Vec<_> = workers.into_iter().map(|w| w.join().unwrap()).collect();
        assert_eq!(
            results.iter().filter(|r| r.outcome.is_some()).count(),
            1,
            "incompatible concurrent submissions consume at most one exact head"
        );
        assert!(fixture
            .sdk
            .recover_local_evaluation_request(&race_prepared.request_locator())
            .unwrap()
            .is_none());

        // A genuinely owner-established G invalidates a pending D choice, while
        // the immutable participant selection remains usable by a fresh G request.
        use handbook_contracts::project_admission_governance::{
            ExpectedDeferredBasisV2, ProjectAdmissionScopeBasisV3,
        };
        use handbook_sdk::admission_governance::{
            DeferredEstablishmentExecutionV3, DeferredEstablishmentRequestV3,
        };
        let d_input = followup(
            &fixture,
            &original,
            &old_capture,
            &capture(supplied_validation_content()),
            LocalEvaluationFollowupActionV1::CorrectedInput,
            "packet4-decision-pending-d-formation",
        );
        let d_prepared = fixture
            .sdk
            .prepare_local_evaluation_followup(&fixture.initial, &fixture.authority, d_input)
            .unwrap();
        let d_requested = request(
            &fixture,
            &grant,
            &d_prepared,
            &original,
            "packet4-decision-pending-d-request",
        );
        let d_submission = submission(
            &fixture,
            &d_requested,
            &d_requested.head_fingerprint,
            "packet4-decision-pending-d-approve",
            LocalInputCorrectionDecisionV1::Approve,
        );
        let d_decision = fixture
            .sdk
            .submit_local_input_correction_decision(
                &fixture.initial,
                &fixture.authority,
                &grant,
                &d_requested.locator,
                &d_submission,
            )
            .unwrap()
            .outcome
            .unwrap();
        let ProjectAdmissionScopeBasisV3::DeferredV2(d) =
            original.requirements.applicability.governance_basis.clone()
        else {
            panic!("actual owner D required")
        };
        let establishment = fixture
            .sdk
            .prepare_deferred_project_admission_governance(
                &DeferredEstablishmentRequestV3::human_approved(
                    d.project,
                    ExpectedDeferredBasisV2 {
                        outcome_fingerprint: d.outcome_fingerprint,
                        relation_fingerprint: d.relation_fingerprint,
                    },
                    3600,
                    "packet4-decision-establish-g",
                    "Supplied exact same-project freshness governance approval.",
                )
                .unwrap(),
            )
            .unwrap();
        assert!(matches!(
            fixture
                .sdk
                .establish_deferred_project_admission_governance(establishment),
            DeferredEstablishmentExecutionV3::Committed { .. }
        ));
        assert!(fixture
            .sdk
            .form_approved_local_input_correction(
                &fixture.initial,
                &fixture.authority,
                &grant,
                &d_requested.locator,
                &d_decision.locator,
                d_prepared
            )
            .is_err());
        let mut stale_submit = d_submission.clone();
        stale_submit.submission_key = "packet4-decision-d-cannot-be-g".into();
        assert!(fixture
            .sdk
            .submit_local_input_correction_decision(
                &fixture.initial,
                &fixture.authority,
                &grant,
                &d_requested.locator,
                &stale_submit
            )
            .unwrap()
            .outcome
            .is_none());
        assert_eq!(
            fixture
                .sdk
                .recover_local_input_correction_decision(
                    &fixture.authority,
                    &grant,
                    &d_requested.locator,
                    &d_decision.locator
                )
                .outcome
                .unwrap(),
            d_decision
        );
        let g_input = followup(
            &fixture,
            &original,
            &old_capture,
            &capture(supplied_validation_content()),
            LocalEvaluationFollowupActionV1::CorrectedInput,
            "packet4-decision-fresh-g-formation",
        );
        let g_prepared = fixture
            .sdk
            .prepare_local_evaluation_followup(
                &fixture.initial,
                &fixture.authority,
                g_input.clone(),
            )
            .unwrap();
        assert_ne!(
            g_prepared.current_basis(),
            &d_requested.request.current_basis
        );
        let g_requested = request(
            &fixture,
            &grant,
            &g_prepared,
            &original,
            "packet4-decision-fresh-g-request",
        );
        let g_submission = submission(
            &fixture,
            &g_requested,
            &g_requested.head_fingerprint,
            "packet4-decision-fresh-g-approve",
            LocalInputCorrectionDecisionV1::Approve,
        );
        let g_decision = fixture
            .sdk
            .submit_local_input_correction_decision(
                &fixture.initial,
                &fixture.authority,
                &grant,
                &g_requested.locator,
                &g_submission,
            )
            .unwrap()
            .outcome
            .unwrap();
        let g_recovery_prepared = g_prepared.clone();
        let g_formed = fixture
            .sdk
            .form_approved_local_input_correction(
                &fixture.initial,
                &fixture.authority,
                &grant,
                &g_requested.locator,
                &g_decision.locator,
                g_prepared,
            )
            .unwrap();
        assert!(matches!(
            g_formed.requirements.applicability.governance_basis,
            ProjectAdmissionScopeBasisV3::EstablishedFromDeferredV3(_)
        ));
        let g_closed = complete(
            &fixture,
            &issue(&fixture, g_formed, capture(supplied_validation_content())),
        );
        assert_eq!(
            g_closed.result.as_ref().unwrap().gate,
            LocalEvaluationGateV3::Pass
        );
        let mut revoke_input = g_input;
        revoke_input.input.request_key = "packet4-decision-revoke-before-consume".into();
        let revoke_prepared = fixture
            .sdk
            .prepare_local_evaluation_followup(&fixture.initial, &fixture.authority, revoke_input)
            .unwrap();
        let revoke_requested = request(
            &fixture,
            &grant,
            &revoke_prepared,
            &original,
            "packet4-decision-revoke-pending-request",
        );
        let revoke_submission = submission(
            &fixture,
            &revoke_requested,
            &revoke_requested.head_fingerprint,
            "packet4-decision-revoke-pending-approve",
            LocalInputCorrectionDecisionV1::Approve,
        );
        let revoke_decision = fixture
            .sdk
            .submit_local_input_correction_decision(
                &fixture.initial,
                &fixture.authority,
                &grant,
                &revoke_requested.locator,
                &revoke_submission,
            )
            .unwrap()
            .outcome
            .unwrap();
        fixture.sdk.revoke_local_evaluation_terms(&fixture.authority, &LocalRevokeRequestV1 {
            request_key: "packet4-decision-revoke".into(),
            expected_current_fingerprint: fixture.sdk.read_local_evaluation_terms_history(&fixture.authority).outcome.unwrap().outcome_fingerprint,
            principal: fixture.authority.terms.revocation_principal.clone(),
            source_ref: "decision.packet4.illustrative-revoke".into(),
            instruction: "Revoke this exact evaluation permission before pending correction consumption.".into(),
        }, &engine_reach()).unwrap();
        assert!(fixture
            .sdk
            .form_approved_local_input_correction(
                &fixture.initial,
                &fixture.authority,
                &grant,
                &revoke_requested.locator,
                &revoke_decision.locator,
                revoke_prepared
            )
            .is_err());
        assert_eq!(
            fixture
                .sdk
                .submit_local_input_correction_decision(
                    &fixture.initial,
                    &fixture.authority,
                    &grant,
                    &revoke_requested.locator,
                    &revoke_submission
                )
                .unwrap()
                .outcome
                .unwrap(),
            revoke_decision,
            "exact submission replay after revoke is historical delivery only"
        );
        let mut fresh_after_revoke = revoke_submission;
        fresh_after_revoke.submission_key = "packet4-decision-after-revoke".into();
        assert!(fixture
            .sdk
            .submit_local_input_correction_decision(
                &fixture.initial,
                &fixture.authority,
                &grant,
                &revoke_requested.locator,
                &fresh_after_revoke
            )
            .unwrap()
            .outcome
            .is_none());
        assert_eq!(
            fixture
                .sdk
                .form_approved_local_input_correction(
                    &fixture.initial,
                    &fixture.authority,
                    &grant,
                    &g_requested.locator,
                    &g_decision.locator,
                    g_recovery_prepared.clone()
                )
                .unwrap(),
            g_closed,
            "original formation replay after revoke delivers exact closed truth without fresh use"
        );
        let mut conflicting_decision = g_decision.locator.clone();
        conflicting_decision.submission_fingerprint = format!("sha256:{}", "9".repeat(64));
        assert!(
            fixture
                .sdk
                .form_approved_local_input_correction(
                    &fixture.initial,
                    &fixture.authority,
                    &grant,
                    &g_requested.locator,
                    &conflicting_decision,
                    g_recovery_prepared
                )
                .is_err(),
            "historical formation replay still requires exact original consumed decision"
        );
        assert_eq!(
            fixture
                .sdk
                .read_local_evaluation_result_history(&original.result_locator().unwrap())
                .unwrap()
                .result
                .canonical_json_bytes()
                .unwrap(),
            original_bytes
        );
        assert_eq!(
            fixture
                .sdk
                .recover_local_input_correction_delegation(&fixture.authority, &grant)
                .outcome
                .unwrap(),
            adopted.outcome.unwrap()
        );
        assert_eq!(
            fixture
                .sdk
                .read_local_evaluation_result_history(&g_closed.result_locator().unwrap())
                .unwrap()
                .result,
            g_closed.result.unwrap()
        );
    });
}

#[test]
fn native_packet4_interrupted_upfront_delegation_recovers_exact_original_intent() {
    on_stack(|| {
        use handbook_engine::local_authority::LocalAuthorityDispositionV1;
        let _package = InstalledProductionPackage::install();
        let fixture = Fixture::new();
        let current_path = fixture
            .root
            .path()
            .join(".handbook/local-operating/local-evaluation-v1/current.json");
        let executor_before = std::fs::read(&current_path).unwrap();
        let mut wrong_predecessor = delegation(&fixture);
        wrong_predecessor.expected_initial_fingerprint = format!("sha256:{}", "0".repeat(64));
        wrong_predecessor.approved_content_fingerprint =
            wrong_predecessor.fingerprint_approved_content().unwrap();
        assert!(fixture
            .sdk
            .prepare_local_input_correction_delegation(
                &fixture.initial,
                &fixture.authority,
                wrong_predecessor
            )
            .is_err());
        let prepared = fixture
            .sdk
            .prepare_local_input_correction_delegation(
                &fixture.initial,
                &fixture.authority,
                delegation(&fixture),
            )
            .unwrap();
        let original = prepared.locator().clone();
        assert_ne!(
            original.delegation.decision_agent,
            original.delegation.executor
        );
        let grant_root = fixture.root.path().join(format!(
            ".handbook/local-operating/local-evaluation-v1/input-correction-v1/delegation/{}",
            original.delegation.request_key
        ));
        for absent in [
            fixture
                .sdk
                .recover_local_input_correction_delegation(&fixture.authority, &original),
            fixture
                .sdk
                .resume_local_input_correction_delegation(&fixture.authority, &original),
        ] {
            assert!(matches!(
                absent.disposition,
                LocalAuthorityDispositionV1::Refused
            ));
            assert!(absent.outcome.is_none());
            assert_eq!(absent.refusal.unwrap().code, "correction_delegation_absent");
        }
        assert!(
            !grant_root.exists(),
            "portable locators cannot create absent intents"
        );
        assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            NativeLocalInputCorrectionV1::adopt_delegation_interrupted(
                fixture.root.path(),
                &fixture.authority,
                prepared,
                || panic!("durable upfront delegation response interruption"),
            )
        }))
        .is_err());
        let request_path = grant_root.join("request.json");
        let original_intent_bytes = std::fs::read(&request_path).unwrap();
        let pending = fixture
            .sdk
            .recover_local_input_correction_delegation(&fixture.authority, &original);
        assert!(matches!(
            pending.disposition,
            LocalAuthorityDispositionV1::Uncertain
        ));
        assert!(pending.outcome.is_none());
        assert_eq!(
            pending.refusal.unwrap().code,
            "correction_delegation_pending"
        );
        let resumed = fixture
            .sdk
            .resume_local_input_correction_delegation(&fixture.authority, &original);
        assert!(matches!(
            resumed.disposition,
            LocalAuthorityDispositionV1::Committed
        ));
        let resumed = resumed.outcome.unwrap();
        assert_eq!(resumed.locator, original);
        assert_eq!(std::fs::read(&request_path).unwrap(), original_intent_bytes);
        let terminal_before = std::fs::read(grant_root.join("terminal.json")).unwrap();
        let recovered = fixture
            .sdk
            .recover_local_input_correction_delegation(&fixture.authority, &original);
        assert!(matches!(
            recovered.disposition,
            LocalAuthorityDispositionV1::Replayed
        ));
        assert_eq!(recovered.outcome.unwrap(), resumed);
        let resumed_again = fixture
            .sdk
            .resume_local_input_correction_delegation(&fixture.authority, &original);
        assert!(matches!(
            resumed_again.disposition,
            LocalAuthorityDispositionV1::Replayed
        ));
        assert_eq!(resumed_again.outcome.unwrap(), resumed);
        assert_eq!(
            std::fs::read(grant_root.join("terminal.json")).unwrap(),
            terminal_before
        );
        assert_eq!(std::fs::read(&request_path).unwrap(), original_intent_bytes);
        assert_eq!(std::fs::read(&current_path).unwrap(), executor_before);
        assert!(
            !fixture
                .root
                .path()
                .join(".handbook/local-operating/local-evaluation-v1/input-correction-v1/requests")
                .exists(),
            "upfront adoption and recovery create no unresolved request"
        );
    });
}
