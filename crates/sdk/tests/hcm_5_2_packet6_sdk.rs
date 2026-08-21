use handbook_contracts::{
    lifecycle_policy_binding_v1, prepare_contract_definition_append, ApplicabilityRuleV1,
    CandidateObservationFixtureV1, ContentAddressedBindingV1, ContractApprovalClassV1,
    ContractClaimInputV1, ContractClaimV1, ContractDefinitionInputV1, ContractDefinitionV1,
    ContractEvaluationRunV1, ContractIdV1, ContractKindV1, ContractLifecycleTransitionInputV1,
    ContractOwnerRepositoryV1, EvaluationCaseV1, EvaluationPolicyRegistryV1, EvaluationRunNonceV1,
    EvaluationSubjectV1, EvidenceCandidateSourceErrorKindV1, EvidenceCandidateSourceErrorV1,
    EvidenceCandidateSourceV1, EvidenceCardinalityV1, EvidenceCaseSelectionV1,
    EvidenceFreshnessPolicyInputV1, EvidenceFreshnessPolicyV1, EvidenceRequirementV1,
    ExactBindingV1 as OwnerBinding, ExactRefV1 as OwnerRef, FingerprintV1 as OwnerFingerprint,
    GateEffectV1, GatePolicyAlgorithmV1, GatePolicyInputV1, GatePolicyV1, LifecycleEvidenceKindV1,
    LifecycleReasonV1, LifecycleStateV1, LifecycleSupportingEvidenceV1, MatcherBindingV1,
    MatcherInputValueV1, PrepareEvaluationRunInputV1, ResolutionEnvelopeV1, ResolutionMinimumV1,
    ResolvedEvidenceCandidateObservationV1, SelectorCriterionV1, SelectorV1, StableIdV1, VersionV1,
};
use handbook_engine::{
    observe_committed_approver_registry, ApproverAdminRequestV1, ApproverRegistryServiceV1,
    ContextResolutionPolicyRegistry, ContextResolutionStackDefinition,
    CredentialAuthorityAdmissionServiceV1, EvidenceSourceAdmissionErrorV1,
    NativeAuthenticatorPortErrorV1, NativeAuthenticatorPortV1, SourceCurrentnessObservationV1,
    SourceCurrentnessPortV1, SourceCurrentnessRequestV1, AUTHENTICATOR_RP_ID,
};
use handbook_sdk::contract_membrane::{
    admit_operation_definition, canonical_json_bytes, contract_evidence_append_operation_request,
    contract_evidence_list_operation_request, contract_evidence_operation_definitions,
    contract_evidence_read_operation_request, contract_gate_evaluate_operation_request,
    contract_operation_definitions, contract_verdict_evaluate_operation_request,
    operation_inventory, parse_canonical_json, AdmissionReason, ContractEvaluationRunDtoV1,
    ContractEvidenceAppendRequestV1, ContractEvidenceAppendResultV1, ContractEvidenceListRequestV1,
    ContractEvidenceListResultV1, ContractEvidenceOperationSchemaValidator,
    ContractEvidenceReadRequestV1, ContractEvidenceReadResultV1, ContractGateEvaluateRequestV1,
    ContractGateEvaluateResultV1, ContractLimits, ContractRecordBindingDtoV1, ContractValue,
    ContractVerdictEvaluateRequestV1, ContractVerdictEvaluateResultV1, DiscoveryState, Fingerprint,
    InvocationResponseKind, OperationAdmissionResult, OperationAvailability, OperationClass,
    OperationIdempotency, OperationOwner, OperationResponseV1, OperationTransport,
    ProductionWitness, RequestId, TerminalStatus, UtcTimestamp, CONTRACT_EVIDENCE_LIST_REF,
};
use p256::ecdsa::{signature::Signer, Signature, SigningKey};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

const OPERATIONS: [(&str, OperationClass); 5] = [
    ("contract.evidence.list@1.0.0", OperationClass::ReadOnly),
    ("contract.evidence.read@1.0.0", OperationClass::ReadOnly),
    ("contract.evidence.append@1.0.0", OperationClass::AppendOnly),
    ("contract.verdict.evaluate@1.0.0", OperationClass::ReadOnly),
    ("contract.gate.evaluate@1.0.0", OperationClass::ReadOnly),
];

const NEW_SCHEMA_ROOTS: [&str; 22] = [
    "handbook.contract-evidence-candidate@1.0.0",
    "handbook.contract-evidence-record@1.0.0",
    "handbook.contract-evidence-admission-policy@1.0.0",
    "handbook.contract-evidence-freshness-policy@1.0.0",
    "handbook.contract-evidence-stability-policy@1.0.0",
    "handbook.contract-evaluation-run@1.0.0",
    "handbook.contract-claim-verdict@1.0.0",
    "handbook.contract-gate-policy@1.0.0",
    "handbook.contract-gate-result@1.0.0",
    "handbook.contract-local-closeout-policy@1.0.0",
    "handbook.contract-local-closeout-result@1.0.0",
    "handbook.contract-evidence-operation-details@1.0.0",
    "handbook.operation.contract-evidence-list-request@1.0.0",
    "handbook.operation.contract-evidence-list-result@1.0.0",
    "handbook.operation.contract-evidence-read-request@1.0.0",
    "handbook.operation.contract-evidence-read-result@1.0.0",
    "handbook.operation.contract-evidence-append-request@1.0.0",
    "handbook.operation.contract-evidence-append-result@1.0.0",
    "handbook.operation.contract-verdict-evaluate-request@1.0.0",
    "handbook.operation.contract-verdict-evaluate-result@1.0.0",
    "handbook.operation.contract-gate-evaluate-request@1.0.0",
    "handbook.operation.contract-gate-evaluate-result@1.0.0",
];

#[test]
fn packet_six_closes_exactly_five_reserved_rows_without_inventory_growth() {
    let definitions = contract_evidence_operation_definitions().unwrap();
    assert_eq!(definitions.len(), OPERATIONS.len());
    assert_eq!(operation_inventory().len(), 62);

    for ((expected_ref, expected_class), definition) in OPERATIONS.iter().zip(&definitions) {
        assert_eq!(definition.operation().as_str(), *expected_ref);
        assert_eq!(definition.class(), *expected_class);
        assert_eq!(definition.owner().owner_crate(), "handbook-contracts");
        assert_eq!(definition.owner().owner_version().as_str(), "0.1.0");
        assert_eq!(
            definition.owner().source().reference().as_str(),
            "handbook.owner.contracts@0.1.0"
        );
        assert_eq!(
            definition.declared_transports(),
            &[
                OperationTransport::CliJson,
                OperationTransport::RustSdk,
                OperationTransport::Tauri,
            ]
        );
        assert!(matches!(
            (expected_class, definition.idempotency()),
            (OperationClass::ReadOnly, OperationIdempotency::Safe)
                | (
                    OperationClass::AppendOnly,
                    OperationIdempotency::IdempotencyKeyRequired { .. }
                )
        ));
        assert!(matches!(
            admit_operation_definition(definition).unwrap(),
            OperationAdmissionResult::Admitted(_)
        ));

        let row = operation_inventory()
            .iter()
            .find(|row| row.operation_ref().as_str() == *expected_ref)
            .unwrap();
        assert_eq!(row.owner(), OperationOwner::Contracts);
        assert_eq!(row.availability(), OperationAvailability::LivePrecursor);
        assert_eq!(row.admission_reason(), AdmissionReason::Admitted);
        assert_eq!(row.discovery(), DiscoveryState::Discoverable);
        assert_eq!(row.current_transports(), definition.declared_transports());
    }
}

#[test]
fn packet_six_closes_exact_schema_and_capability_counts() {
    let witness =
        ProductionWitness::from_current(UtcTimestamp::parse("9999-12-31T23:59:59Z").unwrap())
            .unwrap();
    assert_eq!(witness.schema_manifest().len(), 170);
    assert_eq!(witness.capabilities().len(), 31);

    let roots = witness
        .schema_manifest()
        .iter()
        .map(|entry| entry.schema().reference().as_str())
        .collect::<BTreeSet<_>>();
    assert_eq!(roots.len(), 170);
    for expected in NEW_SCHEMA_ROOTS {
        assert!(roots.contains(expected), "missing schema root {expected}");
    }
    assert_eq!(
        roots.iter().filter(|root| !root.contains("dock")).count(),
        158
    );

    let capabilities = witness
        .capabilities()
        .iter()
        .map(|entry| entry.operation().reference().as_str())
        .collect::<BTreeSet<_>>();
    for (expected, _) in OPERATIONS {
        assert!(capabilities.contains(expected));
    }
    assert_eq!(
        capabilities
            .iter()
            .filter(|operation| !operation.starts_with("dock."))
            .count(),
        28
    );
}

#[test]
fn packet_six_preserves_all_four_predecessor_definition_identities() {
    let expected = [
        (
            "contract.definition.list@1.0.0",
            "sha256:c942ce40901e5288165ca9377652e66f6a513e209b8369a955dca9571cc2a809",
        ),
        (
            "contract.definition.read@1.0.0",
            "sha256:73a8458f0ef3e9349af0f802e68040080c0b3cbe2a6c45ae29efc7a8ecb6ed18",
        ),
        (
            "contract.definition.append@1.0.0",
            "sha256:1bd097fd56c0ae53ef91cc5ecd887a2c88c80b750ab9332e7f9a0c0c00d1abf8",
        ),
        (
            "contract.lifecycle.transition@1.0.0",
            "sha256:fcc11925da6d1ad53707325d6b1c0c6a351f5aafaa83b84632c2c978564f039f",
        ),
    ];
    let definitions = contract_operation_definitions().unwrap();
    for (operation_ref, fingerprint) in expected {
        let definition = definitions
            .iter()
            .find(|definition| definition.operation().as_str() == operation_ref)
            .unwrap();
        assert_eq!(definition.definition_fingerprint().as_str(), fingerprint);
    }
}

#[test]
fn list_and_read_dispatch_owner_results_and_readmit_exact_response_shapes() {
    let repo = tempfile::tempdir().unwrap();
    let sdk = handbook_sdk::HandbookSdkV1::open(repo.path());

    let list = contract_evidence_list_operation_request(
        Some("hcm52-evidence-list"),
        ContractEvidenceListRequestV1::first(10).unwrap(),
    )
    .unwrap();
    let listed = sdk.execute_contract_evidence_operation(&list).unwrap();
    assert_eq!(listed.terminal().status(), TerminalStatus::Ok);
    assert!(listed.receipts().is_empty());
    let listed_data: ContractEvidenceListResultV1 = parse_canonical_json(
        &handbook_sdk::contract_membrane::canonical_json_bytes(
            listed.terminal().data().unwrap(),
            &ContractLimits::packet0(),
        )
        .unwrap(),
        &ContractLimits::packet0(),
    )
    .unwrap();
    let round_trip = serde_json::to_value(listed_data).unwrap();
    assert_eq!(round_trip["entries"], serde_json::json!([]));
    assert_eq!(round_trip["retention"], "immutable_all");
    let listed_bytes = canonical_json_bytes(&listed, &ContractLimits::packet0()).unwrap();
    assert_eq!(
        OperationResponseV1::parse_and_admit(
            &listed_bytes,
            &contract_evidence_operation_definitions().unwrap()[0],
            &ContractEvidenceOperationSchemaValidator,
            &ContractLimits::packet0(),
        )
        .unwrap(),
        listed
    );

    let requested = ContractRecordBindingDtoV1::new(
        "handbook.contract.test@1.0.0#evidence/aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        Fingerprint::parse(
            "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        )
        .unwrap(),
    )
    .unwrap();
    let read = contract_evidence_read_operation_request(
        Some("hcm52-evidence-read"),
        ContractEvidenceReadRequestV1::new(requested.clone()),
    )
    .unwrap();
    let absent = sdk.execute_contract_evidence_operation(&read).unwrap();
    assert_eq!(absent.terminal().status(), TerminalStatus::Ok);
    let data: ContractEvidenceReadResultV1 = parse_canonical_json(
        &handbook_sdk::contract_membrane::canonical_json_bytes(
            absent.terminal().data().unwrap(),
            &ContractLimits::packet0(),
        )
        .unwrap(),
        &ContractLimits::packet0(),
    )
    .unwrap();
    assert_eq!(data, ContractEvidenceReadResultV1::Absent { requested });
    let absent_bytes = canonical_json_bytes(&absent, &ContractLimits::packet0()).unwrap();
    assert_eq!(
        OperationResponseV1::parse_and_admit(
            &absent_bytes,
            &contract_evidence_operation_definitions().unwrap()[1],
            &ContractEvidenceOperationSchemaValidator,
            &ContractLimits::packet0(),
        )
        .unwrap(),
        absent
    );

    let witness =
        ProductionWitness::from_current(UtcTimestamp::parse("9999-12-31T23:59:59Z").unwrap())
            .unwrap();
    let capability = witness
        .capabilities()
        .iter()
        .find(|entry| entry.operation().reference().as_str() == CONTRACT_EVIDENCE_LIST_REF)
        .unwrap();
    let consumer = handbook_sdk::contract_membrane::CapabilityConsumer::open(repo.path()).unwrap();
    let body = handbook_sdk::contract_membrane::canonical_json_bytes(
        &ContractEvidenceListRequestV1::first(10).unwrap(),
        &ContractLimits::packet0(),
    )
    .unwrap();
    let invoked = consumer
        .invoke(
            capability.operation(),
            capability.entry_fingerprint(),
            OperationTransport::CliJson,
            Some(RequestId::parse("hcm52-generic-list").unwrap()),
            &body,
        )
        .unwrap();
    assert_eq!(invoked.response_kind(), InvocationResponseKind::Operation);
    assert_eq!(invoked.terminal_status(), TerminalStatus::Ok);
}

#[test]
fn owner_backed_append_verdict_and_gate_close_replay_conflict_and_readmission() {
    let mut fixture = EvidenceSdkFixture::new();
    let sdk = handbook_sdk::HandbookSdkV1::open(fixture.repo.path());
    let candidate = outward_binding(&fixture.candidate);
    let run = outward_run(&fixture.run);
    let append_key = "packet-six-evidence-append-0001";
    let append = contract_evidence_append_operation_request(
        Some("packet-six-evidence-append"),
        append_key,
        ContractEvidenceAppendRequestV1::new(candidate.clone()),
    )
    .unwrap();
    let before = repository_snapshot(fixture.repo.path());
    let committed = sdk
        .execute_contract_evidence_operation_with_context(
            &append,
            &mut fixture.source,
            &mut fixture.currentness,
            &fixture.stack,
            &fixture.policies,
        )
        .unwrap();
    assert_eq!(committed.terminal().status(), TerminalStatus::Ok);
    let committed_value = serde_json::to_value(&committed).unwrap();
    assert_eq!(committed_value["idempotency"]["replayed"], false);
    assert_eq!(committed.receipts().len(), 1);
    let _: ContractEvidenceAppendResultV1 = response_data(&committed);
    assert_response_readmits(&append, &committed);
    let after_commit = repository_snapshot(fixture.repo.path());
    assert_ne!(after_commit, before);

    let replayed = sdk
        .execute_contract_evidence_operation_with_context(
            &append,
            &mut fixture.source,
            &mut fixture.currentness,
            &fixture.stack,
            &fixture.policies,
        )
        .unwrap();
    let replayed_value = serde_json::to_value(&replayed).unwrap();
    assert_eq!(replayed.terminal().status(), TerminalStatus::Ok);
    assert_eq!(replayed_value["idempotency"]["replayed"], true);
    assert_eq!(
        replayed.result_fingerprint(),
        committed.result_fingerprint()
    );
    assert_eq!(replayed.receipts(), committed.receipts());
    assert_response_readmits(&append, &replayed);
    assert_eq!(repository_snapshot(fixture.repo.path()), after_commit);

    let conflicting = contract_evidence_append_operation_request(
        Some("packet-six-evidence-conflict"),
        "packet-six-evidence-append-0002",
        ContractEvidenceAppendRequestV1::new(candidate),
    )
    .unwrap();
    let conflict = sdk
        .execute_contract_evidence_operation_with_context(
            &conflicting,
            &mut fixture.source,
            &mut fixture.currentness,
            &fixture.stack,
            &fixture.policies,
        )
        .unwrap();
    assert_problem(&conflict, TerminalStatus::Refused, "conflict");
    assert!(conflict.receipts().is_empty());
    assert_response_readmits(&conflicting, &conflict);
    assert_eq!(repository_snapshot(fixture.repo.path()), after_commit);

    let verdict_request = contract_verdict_evaluate_operation_request(
        Some("packet-six-verdict"),
        ContractVerdictEvaluateRequestV1::new(run.clone()),
    )
    .unwrap();
    let verdict = sdk
        .execute_contract_evidence_operation_with_context(
            &verdict_request,
            &mut fixture.source,
            &mut fixture.currentness,
            &fixture.stack,
            &fixture.policies,
        )
        .unwrap();
    assert_eq!(verdict.terminal().status(), TerminalStatus::Ok);
    let _: ContractVerdictEvaluateResultV1 = response_data(&verdict);
    assert_response_readmits(&verdict_request, &verdict);

    let gate_request = contract_gate_evaluate_operation_request(
        Some("packet-six-gate"),
        ContractGateEvaluateRequestV1::new(run.clone()),
    )
    .unwrap();
    let gate = sdk
        .execute_contract_evidence_operation_with_context(
            &gate_request,
            &mut fixture.source,
            &mut fixture.currentness,
            &fixture.stack,
            &fixture.policies,
        )
        .unwrap();
    assert_eq!(gate.terminal().status(), TerminalStatus::Ok);
    let gate_data: ContractGateEvaluateResultV1 = response_data(&gate);
    let gate_value = serde_json::to_value(gate_data).unwrap();
    assert_eq!(gate_value["gate"]["document"]["body"]["decision"], "passed");
    assert_response_readmits(&gate_request, &gate);

    let substituted = ContractEvaluationRunDtoV1::new(
        ContractRecordBindingDtoV1::new(
            run.binding().reference(),
            Fingerprint::parse(&format!("sha256:{}", "f".repeat(64))).unwrap(),
        )
        .unwrap(),
        run.document().clone(),
    );
    let calls_before_refusals = fixture.currentness.calls;
    let snapshot_before_refusals = repository_snapshot(fixture.repo.path());
    for request in [
        contract_verdict_evaluate_operation_request(
            Some("packet-six-verdict-substituted"),
            ContractVerdictEvaluateRequestV1::new(substituted.clone()),
        )
        .unwrap(),
        contract_gate_evaluate_operation_request(
            Some("packet-six-gate-substituted"),
            ContractGateEvaluateRequestV1::new(substituted.clone()),
        )
        .unwrap(),
    ] {
        let refused = sdk
            .execute_contract_evidence_operation_with_context(
                &request,
                &mut fixture.source,
                &mut fixture.currentness,
                &fixture.stack,
                &fixture.policies,
            )
            .unwrap();
        assert_problem(&refused, TerminalStatus::Refused, "substituted");
        assert_response_readmits(&request, &refused);
    }
    assert_eq!(fixture.currentness.calls, calls_before_refusals);
    assert_eq!(
        repository_snapshot(fixture.repo.path()),
        snapshot_before_refusals
    );

    let persisted = repository_bytes(fixture.repo.path());
    for raw_key in [append_key, "packet-six-evidence-append-0002"] {
        assert!(!persisted
            .windows(raw_key.len())
            .any(|window| window == raw_key.as_bytes()));
        for response in [&committed, &replayed, &conflict] {
            let bytes = canonical_json_bytes(response, &ContractLimits::packet0()).unwrap();
            assert!(!bytes
                .windows(raw_key.len())
                .any(|window| window == raw_key.as_bytes()));
        }
    }
}

#[test]
fn append_preserves_missing_stale_and_unavailable_source_causes_without_writes() {
    let mut fixture = EvidenceSdkFixture::without_candidate();
    let sdk = handbook_sdk::HandbookSdkV1::open(fixture.repo.path());
    let candidate = outward_binding(&fixture.candidate);
    let before = repository_snapshot(fixture.repo.path());

    let missing_request = contract_evidence_append_operation_request(
        Some("packet-six-evidence-missing"),
        "packet-six-evidence-missing-key",
        ContractEvidenceAppendRequestV1::new(candidate.clone()),
    )
    .unwrap();
    let missing = sdk
        .execute_contract_evidence_operation_with_context(
            &missing_request,
            &mut fixture.source,
            &mut fixture.currentness,
            &fixture.stack,
            &fixture.policies,
        )
        .unwrap();
    assert_problem(&missing, TerminalStatus::Refused, "missing");
    assert_response_readmits(&missing_request, &missing);
    assert_eq!(repository_snapshot(fixture.repo.path()), before);

    for (suffix, kind, status, cause) in [
        (
            "stale",
            EvidenceCandidateSourceErrorKindV1::StaleAuthority,
            TerminalStatus::Refused,
            "stale",
        ),
        (
            "unavailable",
            EvidenceCandidateSourceErrorKindV1::Unavailable,
            TerminalStatus::Error,
            "candidate_source_unavailable",
        ),
    ] {
        let request = contract_evidence_append_operation_request(
            Some(&format!("packet-six-evidence-{suffix}")),
            &format!("packet-six-evidence-{suffix}-key"),
            ContractEvidenceAppendRequestV1::new(candidate.clone()),
        )
        .unwrap();
        let mut source = FailingCandidateSource { kind };
        let response = sdk
            .execute_contract_evidence_operation_with_context(
                &request,
                &mut source,
                &mut fixture.currentness,
                &fixture.stack,
                &fixture.policies,
            )
            .unwrap();
        assert_problem(&response, status, cause);
        assert!(response.receipts().is_empty());
        assert_response_readmits(&request, &response);
        assert_eq!(repository_snapshot(fixture.repo.path()), before);
    }
}

fn response_data<T: serde::de::DeserializeOwned + serde::Serialize>(
    response: &OperationResponseV1,
) -> T {
    parse_canonical_json(
        &canonical_json_bytes(
            response.terminal().data().unwrap(),
            &ContractLimits::packet0(),
        )
        .unwrap(),
        &ContractLimits::packet0(),
    )
    .unwrap()
}

fn assert_response_readmits(
    request: &handbook_sdk::contract_membrane::OperationRequestV1,
    response: &OperationResponseV1,
) {
    let definition = contract_evidence_operation_definitions()
        .unwrap()
        .into_iter()
        .find(|definition| definition.binding() == *request.operation())
        .unwrap();
    let bytes = canonical_json_bytes(response, &ContractLimits::packet0()).unwrap();
    assert_eq!(
        OperationResponseV1::parse_and_admit(
            &bytes,
            &definition,
            &ContractEvidenceOperationSchemaValidator,
            &ContractLimits::packet0(),
        )
        .unwrap(),
        *response
    );
}

fn assert_problem(response: &OperationResponseV1, status: TerminalStatus, cause: &str) {
    assert_eq!(response.terminal().status(), status);
    let value = serde_json::to_value(response).unwrap();
    let problems = match status {
        TerminalStatus::Refused => &value["refusals"],
        TerminalStatus::Error => &value["errors"],
        _ => panic!("problem assertion requires refused or error"),
    };
    assert_eq!(problems[0]["details"]["cause"], cause);
}

fn outward_binding(binding: &ContentAddressedBindingV1) -> ContractRecordBindingDtoV1 {
    ContractRecordBindingDtoV1::new(
        binding.reference(),
        Fingerprint::parse(binding.fingerprint().as_str()).unwrap(),
    )
    .unwrap()
}

fn outward_run(run: &ContractEvaluationRunV1) -> ContractEvaluationRunDtoV1 {
    ContractEvaluationRunDtoV1::new(
        ContractRecordBindingDtoV1::new(
            run.evaluation_run_ref(),
            Fingerprint::parse(run.evaluation_run_fingerprint().as_str()).unwrap(),
        )
        .unwrap(),
        parse_canonical_json::<ContractValue>(
            &run.canonical_json_bytes().unwrap(),
            &ContractLimits::packet0(),
        )
        .unwrap(),
    )
}

struct EvidenceSdkFixture {
    repo: tempfile::TempDir,
    source: handbook_contracts::RepositoryEvidenceCandidateSourceV1<TestAuthenticatorPort>,
    currentness: FixedSourceCurrentness,
    stack: ContextResolutionStackDefinition,
    policies: EvaluationPolicyRegistryV1,
    run: ContractEvaluationRunV1,
    candidate: ContentAddressedBindingV1,
}

impl EvidenceSdkFixture {
    fn new() -> Self {
        Self::with_candidate(true)
    }

    fn without_candidate() -> Self {
        let fixture = Self::with_candidate(false);
        std::fs::create_dir_all(
            fixture
                .repo
                .path()
                .join(".handbook/state/contracts/v1/candidate-observations"),
        )
        .unwrap();
        fixture
    }

    fn with_candidate(seed: bool) -> Self {
        let mut harness = OwnerHarness::new();
        let (stack, stack_binding) = evidence_resolution_stack();
        let freshness = EvidenceFreshnessPolicyV1::new(
            EvidenceFreshnessPolicyInputV1::new(
                ContractIdV1::parse("handbook.contract-evidence-freshness-policy.fixture").unwrap(),
                VersionV1::parse("1.0.0+packet6").unwrap(),
                300,
                true,
                0,
            )
            .unwrap(),
        )
        .unwrap();
        let gate = GatePolicyV1::new(
            GatePolicyInputV1::new(
                ContractIdV1::parse("handbook.contract-gate-policy.fixture").unwrap(),
                VersionV1::parse("1.0.2").unwrap(),
                GatePolicyAlgorithmV1::ClosedAccounting,
                None,
                handbook_contracts::local_closeout_policy_binding_v1().unwrap(),
            )
            .unwrap(),
        )
        .unwrap();
        let mut policies = EvaluationPolicyRegistryV1::new().unwrap();
        policies.register_freshness(freshness.clone()).unwrap();
        policies.register_gate(gate.clone()).unwrap();
        let definition = evidence_definition(
            &harness.registry_state_fingerprint,
            freshness.binding().clone(),
            gate.binding().clone(),
            stack_binding,
        );
        let (contract, definition_fingerprint, active_head) =
            activate_definition(&mut harness, definition);
        let resolution = evidence_resolution(&stack);
        let run = harness
            .owner
            .prepare_evaluation_run_v1(
                PrepareEvaluationRunInputV1::new(
                    OwnerBinding::new(contract, definition_fingerprint),
                    active_head,
                    evaluation_subject('7'),
                    vec![evaluation_case('8')],
                    resolution.clone(),
                    MatcherBindingV1::exact_fields_v1(),
                    vec![freshness.binding().clone()],
                    vec![handbook_contracts::evidence_stability_policy_binding_v1().unwrap()],
                    gate.binding().clone(),
                    handbook_contracts::local_closeout_policy_binding_v1().unwrap(),
                    EvaluationRunNonceV1::parse("packet-six-owner-backed-run-0001").unwrap(),
                )
                .unwrap(),
                &policies,
            )
            .unwrap();
        let observed_at = run
            .owner_currentness()
            .semantic_time_utc()
            .as_str()
            .to_owned();
        let candidate =
            candidate_fixture(harness.repo.path(), &run, &resolution, &observed_at, seed);
        let OwnerHarness {
            repo,
            owner: _,
            service,
            registry_state_fingerprint: _,
        } = harness;
        let source = handbook_contracts::RepositoryEvidenceCandidateSourceV1::open(
            repo.path(),
            service.into_port(),
        )
        .unwrap();
        Self {
            repo,
            source,
            currentness: FixedSourceCurrentness {
                head_ref: "source/head".to_owned(),
                head_fingerprint: owner_fp('c').as_str().to_owned(),
                observed_at,
                calls: 0,
            },
            stack,
            policies,
            run,
            candidate,
        }
    }
}

struct FailingCandidateSource {
    kind: EvidenceCandidateSourceErrorKindV1,
}

impl EvidenceCandidateSourceV1 for FailingCandidateSource {
    fn resolve_exact<R>(
        &mut self,
        _candidate: &ContentAddressedBindingV1,
        _consume: impl for<'authority> FnOnce(ResolvedEvidenceCandidateObservationV1<'authority>) -> R,
    ) -> Result<R, EvidenceCandidateSourceErrorV1> {
        Err(EvidenceCandidateSourceErrorV1::for_testing(self.kind))
    }
}

const REPOSITORY_IDENTITY: &str =
    "sha256:1111111111111111111111111111111111111111111111111111111111111111";
const AUTHOR_AUTHORITY_REF: &str = "handbook.authority.contract-definition-author@1.0.0";

struct OwnerHarness {
    repo: tempfile::TempDir,
    owner: ContractOwnerRepositoryV1,
    service: CredentialAuthorityAdmissionServiceV1<TestAuthenticatorPort>,
    registry_state_fingerprint: String,
}

impl OwnerHarness {
    fn new() -> Self {
        let repo = tempfile::tempdir().unwrap();
        let mut registry =
            ApproverRegistryServiceV1::for_repository(repo.path(), TestAuthenticatorPort::new());
        let request = ApproverAdminRequestV1::from_json_value(json!({
            "schema_id":"handbook.approver-admin-request",
            "schema_version":"1.0",
            "operation":"bootstrap",
            "operation_id":"hcm-5-2-packet-six-bootstrap",
            "repository_identity_fingerprint":REPOSITORY_IDENTITY,
            "expected_registry_state_fingerprint":null,
            "expected_transition_fingerprint":null,
            "initial_charter_quorum":[
                {"approval_class":"contract_definition_author","authority_ref":AUTHOR_AUTHORITY_REF},
                {"approval_class":"contract_evidence_source","authority_ref":"handbook.authority.contract-evidence-source@1.0.0"}
            ]
        }))
        .unwrap();
        let result = registry
            .bootstrap_approver_registry(request)
            .to_json_value()
            .unwrap();
        assert_eq!(result["status"], "succeeded", "{result:#}");
        let add = ApproverAdminRequestV1::from_json_value(json!({
            "schema_id":"handbook.approver-admin-request",
            "schema_version":"1.0",
            "operation":"add_credential",
            "operation_id":"hcm-5-2-packet-six-lifecycle-add",
            "repository_identity_fingerprint":REPOSITORY_IDENTITY,
            "expected_registry_state_fingerprint":result["result_registry_state_fingerprint"],
            "expected_transition_fingerprint":result["transition_fingerprint"],
            "approval_mappings":[
                {"approval_class":"contract_activation","authority_ref":"handbook.authority.contract-activation@1.0.0"},
                {"approval_class":"contract_independent_lock","authority_ref":"handbook.authority.contract-independent-lock@1.0.0"},
                {"approval_class":"contract_lifecycle","authority_ref":"handbook.authority.contract-lifecycle@1.0.0"}
            ]
        }))
        .unwrap();
        let added = registry
            .add_approver_credential(add)
            .to_json_value()
            .unwrap();
        assert_eq!(added["status"], "succeeded", "{added:#}");
        let observation = observe_committed_approver_registry(repo.path()).unwrap();
        let registry_state_fingerprint = observation.state_fingerprint.clone();
        drop(observation);
        let service = CredentialAuthorityAdmissionServiceV1::new(repo.path(), registry.into_port());
        let owner = ContractOwnerRepositoryV1::open(repo.path()).unwrap();
        Self {
            repo,
            owner,
            service,
            registry_state_fingerprint,
        }
    }

    fn append_definition(
        &mut self,
        definition: ContractDefinitionV1,
        key: &str,
    ) -> handbook_contracts::DefinitionAppendExecutionV1 {
        let prepared = prepare_contract_definition_append(definition).unwrap();
        self.service
            .admit(prepared.authority_request(), |admission| {
                self.owner.append_definition(prepared, admission, key)
            })
            .unwrap()
            .unwrap()
    }

    fn transition(
        &mut self,
        contract: &OwnerRef,
        definition_fingerprint: &OwnerFingerprint,
        input: ContractLifecycleTransitionInputV1,
        key: &str,
    ) -> handbook_contracts::LifecycleTransitionExecutionV1 {
        let prepared = self
            .owner
            .prepare_lifecycle_transition(contract, definition_fingerprint, input)
            .unwrap();
        self.service
            .admit(prepared.authority_request(), |admission| {
                self.owner.transition_lifecycle(prepared, admission, key)
            })
            .unwrap()
            .unwrap()
    }
}

fn activate_definition(
    harness: &mut OwnerHarness,
    definition: ContractDefinitionV1,
) -> (OwnerRef, OwnerFingerprint, OwnerFingerprint) {
    let contract = definition.contract_ref().clone();
    let definition_fingerprint = definition.definition_fingerprint().clone();
    let draft = harness
        .append_definition(definition, "packet-six-definition-append")
        .lifecycle_head()
        .clone();
    let ready = harness.transition(
        &contract,
        &definition_fingerprint,
        ContractLifecycleTransitionInputV1 {
            expected_lifecycle_fingerprint: draft.lifecycle_fingerprint().clone(),
            from_state: LifecycleStateV1::Draft,
            to_state: LifecycleStateV1::ReviewReady,
            approval_class: ContractApprovalClassV1::DefinitionAuthor,
            supporting_evidence: vec![LifecycleSupportingEvidenceV1::new(
                LifecycleEvidenceKindV1::DefinitionValidation,
                owner_binding("handbook.contract-lifecycle-evidence.packet-six@1.0.0", '1'),
            )],
            reason: LifecycleReasonV1::ReadyForReview,
        },
        "packet-six-definition-ready",
    );
    let locked = harness.transition(
        &contract,
        &definition_fingerprint,
        ContractLifecycleTransitionInputV1 {
            expected_lifecycle_fingerprint: ready.lifecycle_head().lifecycle_fingerprint().clone(),
            from_state: LifecycleStateV1::ReviewReady,
            to_state: LifecycleStateV1::Locked,
            approval_class: ContractApprovalClassV1::IndependentLock,
            supporting_evidence: vec![LifecycleSupportingEvidenceV1::new(
                LifecycleEvidenceKindV1::IndependentReview,
                owner_binding("handbook.contract-lifecycle-evidence.packet-six@1.0.0", '2'),
            )],
            reason: LifecycleReasonV1::IndependentlyLocked,
        },
        "packet-six-definition-lock",
    );
    let active = harness.transition(
        &contract,
        &definition_fingerprint,
        ContractLifecycleTransitionInputV1 {
            expected_lifecycle_fingerprint: locked.lifecycle_head().lifecycle_fingerprint().clone(),
            from_state: LifecycleStateV1::Locked,
            to_state: LifecycleStateV1::Active,
            approval_class: ContractApprovalClassV1::Activation,
            supporting_evidence: vec![
                LifecycleSupportingEvidenceV1::new(
                    LifecycleEvidenceKindV1::ApplicabilityClosure,
                    owner_binding("handbook.contract-lifecycle-evidence.packet-six@1.0.0", '3'),
                ),
                LifecycleSupportingEvidenceV1::new(
                    LifecycleEvidenceKindV1::MatcherClosure,
                    owner_binding("handbook.contract-lifecycle-evidence.packet-six@1.0.0", '4'),
                ),
                LifecycleSupportingEvidenceV1::new(
                    LifecycleEvidenceKindV1::PolicyClosure,
                    owner_binding("handbook.contract-lifecycle-evidence.packet-six@1.0.0", '5'),
                ),
                LifecycleSupportingEvidenceV1::new(
                    LifecycleEvidenceKindV1::CurrentnessClosure,
                    owner_binding("handbook.contract-lifecycle-evidence.packet-six@1.0.0", '6'),
                ),
            ],
            reason: LifecycleReasonV1::Activated,
        },
        "packet-six-definition-active",
    );
    (
        contract,
        definition_fingerprint,
        active.lifecycle_head().lifecycle_fingerprint().clone(),
    )
}

fn evidence_definition(
    registry_state_fingerprint: &str,
    freshness_policy: OwnerBinding,
    gate_policy: OwnerBinding,
    stack_binding: OwnerBinding,
) -> ContractDefinitionV1 {
    ContractDefinitionV1::new(ContractDefinitionInputV1 {
        definition_schema: owner_binding("handbook.contract-definition@1.0.0", 'b'),
        contract_id: ContractIdV1::parse("handbook.contract.packet-six-evidence").unwrap(),
        contract_version: VersionV1::parse("1.0.0").unwrap(),
        contract_kind: ContractKindV1::ClaimSet,
        claims: vec![ContractClaimV1::new(ContractClaimInputV1 {
            claim_id: owner_id("schema.payload.valid"),
            statement: "Payload conforms to its exact schema.".to_owned(),
            subject_selector: owner_selector("subject.kind", "artifact"),
            case_selector: owner_selector("case.kind", "positive"),
            applicability_rule: ApplicabilityRuleV1::SubjectAndCaseMatch,
            gate_effect: GateEffectV1::Required,
            evidence_requirements: vec![EvidenceRequirementV1::new(
                owner_id("handbook.evidence.schema-conformance"),
                EvidenceCardinalityV1::ExactlyOne,
                EvidenceCaseSelectionV1::ClaimCaseSelector,
                handbook_contracts::evidence_stability_policy_binding_v1().unwrap(),
            )
            .unwrap()],
            freshness_policy,
            minimum_resolution: ResolutionMinimumV1::new(
                stack_binding,
                owner_id("resolution.scope.local-observation"),
                owner_id("resolution.detail.identifier-only"),
                owner_id("resolution.time.current-operation"),
                owner_id("resolution.authority.read-only"),
                owner_id("resolution.memory.operation"),
                owner_id("resolution.validation.observation-only"),
            ),
            score_weight: None,
        })
        .unwrap()],
        definition_author_ref: StableIdV1::parse(&format!(
            "credential.sha256-{:x}",
            Sha256::digest([0x32; 32])
        ))
        .unwrap(),
        definition_authority: OwnerBinding::new(
            OwnerRef::parse(AUTHOR_AUTHORITY_REF).unwrap(),
            OwnerFingerprint::parse(registry_state_fingerprint).unwrap(),
        ),
        lifecycle_policy: lifecycle_policy_binding_v1().unwrap(),
        matcher_bindings: vec![MatcherBindingV1::exact_fields_v1()],
        gate_policy,
        extensions_policy: owner_binding("handbook.contract-extensions.closed-v1@1.0.0", 'd'),
    })
    .unwrap()
}

fn evaluation_subject(hex: char) -> EvaluationSubjectV1 {
    let fingerprint = owner_fp(hex);
    EvaluationSubjectV1::new(
        ContentAddressedBindingV1::new(
            &format!("subject.example/{}", &fingerprint.as_str()[7..]),
            fingerprint,
        )
        .unwrap(),
        vec![MatcherInputValueV1::new(owner_id("subject.kind"), "artifact").unwrap()],
    )
    .unwrap()
}

fn evaluation_case(hex: char) -> EvaluationCaseV1 {
    let fingerprint = owner_fp(hex);
    EvaluationCaseV1::new(
        owner_id("case.positive"),
        ContentAddressedBindingV1::new(
            &format!("case.example/{}", &fingerprint.as_str()[7..]),
            fingerprint,
        )
        .unwrap(),
        vec![MatcherInputValueV1::new(owner_id("case.kind"), "positive").unwrap()],
    )
    .unwrap()
}

fn evidence_resolution_stack() -> (ContextResolutionStackDefinition, OwnerBinding) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../engine")
        .canonicalize()
        .unwrap();
    let policies = ContextResolutionPolicyRegistry::load(
        &root,
        &[
            "definitions/context-resolution-policies/handbook.mutation-matcher.core/1.0.0.yaml"
                .to_owned(),
            "definitions/context-resolution-policies/handbook.resolution-escalation.core/1.0.0.yaml"
                .to_owned(),
            "definitions/context-resolution-policies/handbook.memory-promotion.core/1.0.0.yaml"
                .to_owned(),
        ],
    )
    .unwrap();
    let stack = ContextResolutionStackDefinition::load(
        &root,
        "definitions/context-resolution/handbook.context-resolution.shipped-root/1.0.0.yaml",
        &policies,
    )
    .unwrap();
    let binding = OwnerBinding::new(
        OwnerRef::parse(stack.exact_ref().as_str()).unwrap(),
        OwnerFingerprint::parse(stack.definition_fingerprint().as_str()).unwrap(),
    );
    (stack, binding)
}

fn evidence_resolution(stack: &ContextResolutionStackDefinition) -> ResolutionEnvelopeV1 {
    ResolutionEnvelopeV1::new(
        OwnerBinding::new(
            OwnerRef::parse(stack.exact_ref().as_str()).unwrap(),
            OwnerFingerprint::parse(stack.definition_fingerprint().as_str()).unwrap(),
        ),
        owner_id("resolution.scope.local-observation"),
        owner_id("resolution.detail.identifier-only"),
        owner_id("resolution.time.current-operation"),
        owner_id("resolution.authority.read-only"),
        owner_id("resolution.memory.operation"),
        owner_id("resolution.validation.observation-only"),
    )
}

fn candidate_fixture(
    repo: &Path,
    run: &ContractEvaluationRunV1,
    resolution: &ResolutionEnvelopeV1,
    observed_at: &str,
    seed: bool,
) -> ContentAddressedBindingV1 {
    let producer = content_binding("producer/ref", '5');
    let request = content_binding("request/ref", '6');
    let execution = content_binding("execution/ref", '9');
    let body = json!({
        "evaluation_run": {
            "ref": run.evaluation_run_ref(),
            "fingerprint": run.evaluation_run_fingerprint(),
        },
        "evaluation_lifecycle_basis": run.evaluation_lifecycle_basis(),
        "contract": run.contract(),
        "subject": run.subject(),
        "case": &run.cases()[0],
        "observed_claim_ids": ["schema.payload.valid"],
        "evidence_kind": "handbook.evidence.schema-conformance",
        "fact_schema": owner_binding("handbook.fact.schema-conformance@1.0.0", 'b'),
        "fact_payload": {"valid": true},
        "observation": "satisfied",
        "producer": producer,
        "request_record": request,
        "execution_record": execution,
        "sources": [{
            "source": content_binding("source/ref", 'a'),
            "revision": "revision-1",
            "collected_at": observed_at,
        }],
        "claim_partition": {
            "observed": ["schema.payload.valid"],
            "unobserved": [],
            "excluded": [],
        },
        "request_resolution": resolution,
        "actual_resolution": resolution,
        "observed_at": observed_at,
    });
    let preimage = json!({
        "schema_ref": handbook_contracts::EVIDENCE_CANDIDATE_SCHEMA_REF_V1,
        "body": body,
    });
    let fingerprint = OwnerFingerprint::parse(&format!(
        "sha256:{:x}",
        Sha256::digest(serde_json_canonicalizer::to_vec(&preimage).unwrap())
    ))
    .unwrap();
    let reference = format!(
        "producer/candidate/{}",
        fingerprint.as_str().strip_prefix("sha256:").unwrap()
    );
    let value = json!({
        "schema_ref": handbook_contracts::EVIDENCE_CANDIDATE_SCHEMA_REF_V1,
        "body": preimage["body"].clone(),
        "candidate_ref": reference,
        "candidate_fingerprint": fingerprint,
    });
    let candidate = ContentAddressedBindingV1::new(
        value["candidate_ref"].as_str().unwrap(),
        serde_json::from_value(value["candidate_fingerprint"].clone()).unwrap(),
    )
    .unwrap();
    let fixture = CandidateObservationFixtureV1::new(
        candidate.clone(),
        serde_json_canonicalizer::to_vec(&value).unwrap(),
        run.canonical_json_bytes().unwrap(),
        serde_json::from_value(value["body"]["producer"].clone()).unwrap(),
        serde_json::from_value(value["body"]["request_record"].clone()).unwrap(),
        serde_json::from_value(value["body"]["execution_record"].clone()).unwrap(),
        None,
    )
    .unwrap();
    if seed {
        handbook_contracts::seed_committed_candidate_observation_for_testing_v1(repo, &fixture)
            .unwrap();
    }
    candidate
}

struct FixedSourceCurrentness {
    head_ref: String,
    head_fingerprint: String,
    observed_at: String,
    calls: usize,
}

impl SourceCurrentnessPortV1 for FixedSourceCurrentness {
    fn observe_current(
        &mut self,
        request: &SourceCurrentnessRequestV1,
    ) -> Result<SourceCurrentnessObservationV1, EvidenceSourceAdmissionErrorV1> {
        self.calls += 1;
        SourceCurrentnessObservationV1::new(
            request,
            &self.head_ref,
            &self.head_fingerprint,
            &self.observed_at,
        )
    }
}

fn content_binding(reference: &str, hex: char) -> ContentAddressedBindingV1 {
    ContentAddressedBindingV1::new(reference, owner_fp(hex)).unwrap()
}

fn owner_fp(hex: char) -> OwnerFingerprint {
    OwnerFingerprint::parse(&format!("sha256:{}", hex.to_string().repeat(64))).unwrap()
}

fn owner_binding(reference: &str, hex: char) -> OwnerBinding {
    OwnerBinding::new(OwnerRef::parse(reference).unwrap(), owner_fp(hex))
}

fn owner_selector(field: &str, expected: &str) -> SelectorV1 {
    SelectorV1::new(
        MatcherBindingV1::exact_fields_v1(),
        vec![SelectorCriterionV1::new(owner_id(field), expected).unwrap()],
    )
    .unwrap()
}

fn owner_id(value: &str) -> StableIdV1 {
    StableIdV1::parse(value).unwrap()
}

fn repository_snapshot(root: &Path) -> BTreeMap<PathBuf, Vec<u8>> {
    fn visit(root: &Path, path: &Path, output: &mut BTreeMap<PathBuf, Vec<u8>>) {
        for entry in std::fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            if entry.path().is_dir() {
                visit(root, &entry.path(), output);
            } else {
                output.insert(
                    entry.path().strip_prefix(root).unwrap().to_path_buf(),
                    std::fs::read(entry.path()).unwrap(),
                );
            }
        }
    }
    let mut output = BTreeMap::new();
    visit(root, root, &mut output);
    output
}

fn repository_bytes(root: &Path) -> Vec<u8> {
    repository_snapshot(root).into_values().flatten().collect()
}

struct TestAuthenticatorPort {
    author_key: SigningKey,
    authority_key: SigningKey,
    author_credential_id: Vec<u8>,
    authority_credential_id: Vec<u8>,
    make_calls: usize,
    author_assertions: u32,
    authority_assertions: u32,
}

impl TestAuthenticatorPort {
    fn new() -> Self {
        Self {
            author_key: SigningKey::from_slice(&[7_u8; 32]).unwrap(),
            authority_key: SigningKey::from_slice(&[6_u8; 32]).unwrap(),
            author_credential_id: vec![0x32; 32],
            authority_credential_id: vec![0x31; 32],
            make_calls: 0,
            author_assertions: 0,
            authority_assertions: 0,
        }
    }
}

impl NativeAuthenticatorPortV1 for TestAuthenticatorPort {
    fn make_credential(
        &mut self,
        _request_cbor: &[u8],
    ) -> Result<Vec<u8>, NativeAuthenticatorPortErrorV1> {
        self.make_calls += 1;
        if self.make_calls == 1 {
            Ok(make_credential_response(
                &self.author_key,
                &self.author_credential_id,
            ))
        } else {
            Ok(make_credential_response(
                &self.authority_key,
                &self.authority_credential_id,
            ))
        }
    }

    fn get_assertion(
        &mut self,
        request_cbor: &[u8],
    ) -> Result<Vec<u8>, NativeAuthenticatorPortErrorV1> {
        let marker = request_cbor
            .windows(3)
            .position(|window| window == [0x02, 0x58, 0x20])
            .unwrap();
        let client_data_hash = request_cbor[marker + 3..marker + 35].try_into().unwrap();
        if request_cbor
            .windows(self.authority_credential_id.len())
            .any(|window| window == self.authority_credential_id)
        {
            self.authority_assertions += 1;
            Ok(assertion_response(
                &self.authority_key,
                &self.authority_credential_id,
                client_data_hash,
                self.authority_assertions,
            ))
        } else {
            self.author_assertions += 1;
            Ok(assertion_response(
                &self.author_key,
                &self.author_credential_id,
                client_data_hash,
                self.author_assertions,
            ))
        }
    }
}

fn make_credential_response(key: &SigningKey, credential_id: &[u8]) -> Vec<u8> {
    let point = key.verifying_key().to_encoded_point(false);
    let mut cose = vec![0xa5, 0x01, 0x02, 0x03, 0x26, 0x20, 0x01, 0x21, 0x58, 0x20];
    cose.extend_from_slice(point.x().unwrap());
    cose.extend_from_slice(&[0x22, 0x58, 0x20]);
    cose.extend_from_slice(point.y().unwrap());
    let mut auth_data = Sha256::digest(AUTHENTICATOR_RP_ID.as_bytes()).to_vec();
    auth_data.push(0x45);
    auth_data.extend_from_slice(&0_u32.to_be_bytes());
    auth_data.extend_from_slice(&[0_u8; 16]);
    auth_data.extend_from_slice(&(credential_id.len() as u16).to_be_bytes());
    auth_data.extend_from_slice(credential_id);
    auth_data.extend_from_slice(&cose);
    let mut response = vec![0, 0xa3];
    cbor_text(&mut response, "fmt");
    cbor_text(&mut response, "none");
    cbor_text(&mut response, "attStmt");
    response.push(0xa0);
    cbor_text(&mut response, "authData");
    cbor_bytes(&mut response, &auth_data);
    response
}

fn assertion_response(
    key: &SigningKey,
    credential_id: &[u8],
    client_data_hash: [u8; 32],
    sign_count: u32,
) -> Vec<u8> {
    let mut authenticator_data = Sha256::digest(AUTHENTICATOR_RP_ID.as_bytes()).to_vec();
    authenticator_data.push(0x05);
    authenticator_data.extend_from_slice(&sign_count.to_be_bytes());
    let mut preimage = authenticator_data.clone();
    preimage.extend_from_slice(&client_data_hash);
    let signature: Signature = key.sign(&preimage);
    let der = signature.to_der();
    let mut response = vec![0, 0xa3, 0x01, 0xa2];
    cbor_text(&mut response, "id");
    cbor_bytes(&mut response, credential_id);
    cbor_text(&mut response, "type");
    cbor_text(&mut response, "public-key");
    response.push(0x02);
    cbor_bytes(&mut response, &authenticator_data);
    response.push(0x03);
    cbor_bytes(&mut response, der.as_bytes());
    response
}

fn cbor_text(output: &mut Vec<u8>, value: &str) {
    cbor_length(output, 3, value.len());
    output.extend_from_slice(value.as_bytes());
}

fn cbor_bytes(output: &mut Vec<u8>, value: &[u8]) {
    cbor_length(output, 2, value.len());
    output.extend_from_slice(value);
}

fn cbor_length(output: &mut Vec<u8>, major: u8, length: usize) {
    if length < 24 {
        output.push((major << 5) | length as u8);
    } else if length <= u8::MAX as usize {
        output.extend_from_slice(&[(major << 5) | 24, length as u8]);
    } else {
        output.push((major << 5) | 25);
        output.extend_from_slice(&(length as u16).to_be_bytes());
    }
}
