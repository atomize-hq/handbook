use handbook_contracts::{
    evaluate_applicability, lifecycle_policy_binding_v1, ApplicabilityInputSourceV1,
    ApplicabilityOutcomeV1, ApplicabilityRuleV1, ContractClaimInputV1, ContractClaimV1,
    ContractDefinitionInputV1, ContractDefinitionV1, ContractIdV1, ContractKindV1,
    ContractOwnerRepositoryErrorKindV1, ContractOwnerRepositoryV1, EvidenceCardinalityV1,
    EvidenceCaseSelectionV1, EvidenceRequirementV1, ExactBindingV1 as OwnerBinding,
    ExactRefV1 as OwnerRef, FingerprintV1 as OwnerFingerprint, GateEffectV1, MatcherBindingV1,
    MatcherInputCandidateV1, MatcherValueV1, ResolutionMinimumV1, ScoreWeightV1,
    SelectorCriterionV1, SelectorV1, StableIdV1, VersionV1,
};
use handbook_engine::{
    observe_committed_approver_registry, ApproverAdminRequestV1, ApproverRegistryServiceV1,
    NativeAuthenticatorPortErrorV1, NativeAuthenticatorPortV1, AUTHENTICATOR_RP_ID,
};
use handbook_sdk::contract_membrane::{
    canonical_json_bytes, contract_definition_append_operation_request,
    contract_definition_list_operation_request, contract_definition_read_operation_request,
    contract_lifecycle_transition_operation_request, contract_operation_definitions,
    operation_inventory, parse_canonical_json, CapabilityConsumer, CatalogKindV1, CatalogPageSize,
    CatalogReadResponseBodyV1, ContractDefinitionAppendRequestV1, ContractDefinitionDtoV1,
    ContractDefinitionListRequestV1, ContractDefinitionReadRequestV1,
    ContractLifecycleTransitionInputDtoV1, ContractLifecycleTransitionRequestV1, ContractLimits,
    ExactBinding, ExactRef, Fingerprint, InvocationResponseKind, OperationClass,
    OperationIdempotency, OperationTransport, ProductionWitness, RequestId, TerminalStatus,
    UtcTimestamp,
};
use p256::ecdsa::{signature::Signer, Signature, SigningKey};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

const AUTHOR_AUTHORITY_REF: &str = "handbook.authority.contract-definition-author@1.0.0";
const OPERATIONS: [(&str, OperationClass); 4] = [
    ("contract.definition.list@1.0.0", OperationClass::ReadOnly),
    ("contract.definition.read@1.0.0", OperationClass::ReadOnly),
    (
        "contract.definition.append@1.0.0",
        OperationClass::AppendOnly,
    ),
    (
        "contract.lifecycle.transition@1.0.0",
        OperationClass::CompareAndWrite,
    ),
];

#[test]
fn final_owner_to_sdk_and_generic_transport_proof_is_exact_and_fail_closed() {
    let definitions = contract_operation_definitions().unwrap();
    let inventory = operation_inventory();
    let witness =
        ProductionWitness::from_current(UtcTimestamp::parse("9999-12-31T23:59:59Z").unwrap())
            .unwrap();

    assert_eq!(definitions.len(), 4);
    assert_eq!(inventory.len(), 62);
    assert_eq!(witness.capabilities().len(), 28);
    assert_eq!(witness.schema_manifest().len(), 157);
    for ((operation_ref, class), definition) in OPERATIONS.iter().zip(&definitions) {
        assert_eq!(definition.operation().as_str(), *operation_ref);
        assert_eq!(definition.class(), *class);
        assert!(matches!(
            (class, definition.idempotency()),
            (OperationClass::ReadOnly, OperationIdempotency::Safe)
                | (
                    OperationClass::AppendOnly,
                    OperationIdempotency::IdempotencyKeyRequired { .. }
                )
                | (
                    OperationClass::CompareAndWrite,
                    OperationIdempotency::CompareAndWriteRequired { .. }
                )
        ));
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
    }

    let (repo, mut authenticator, registry_state_fingerprint) = authorized_repository();
    let sdk = handbook_sdk::HandbookSdkV1::open(repo.path());
    let definition = owner_definition(&registry_state_fingerprint, "1.0.0");
    prove_claim_and_applicability_closure(definition.claims()[0].clone());
    let normalized = canonical_json_bytes(&definition, &ContractLimits::packet0()).unwrap();
    let reparsed: ContractDefinitionV1 = serde_json::from_slice(&normalized).unwrap();
    assert_eq!(reparsed, definition);

    let (contract, definition_dto) = outward_definition(&definition);
    let append_key = "packet-six-append-key-must-not-leak";
    let append = contract_definition_append_operation_request(
        Some("packet-six-append"),
        append_key,
        ContractDefinitionAppendRequestV1::new(definition_dto),
    )
    .unwrap();
    let before = repository_snapshot(repo.path());
    let appended = sdk
        .execute_contract_operation_with_authenticator(&append, &mut authenticator)
        .unwrap();
    let appended_value = serde_json::to_value(&appended).unwrap();
    assert_eq!(appended.terminal().status(), TerminalStatus::Ok);
    assert_eq!(appended_value["idempotency"]["state"], "established");
    assert_eq!(appended_value["idempotency"]["replayed"], false);
    assert_eq!(appended_value["receipts"].as_array().unwrap().len(), 2);
    let genesis = appended_value["data"]["lifecycle_head"]["lifecycle_fingerprint"]
        .as_str()
        .unwrap()
        .to_owned();
    let committed = repository_snapshot(repo.path());
    assert_ne!(committed, before);

    let replay = sdk
        .execute_contract_operation_with_authenticator(&append, &mut authenticator)
        .unwrap();
    let replay_value = serde_json::to_value(&replay).unwrap();
    assert_eq!(replay.terminal().status(), TerminalStatus::Ok);
    assert_eq!(replay_value["idempotency"]["replayed"], true);
    assert_eq!(
        replay_value["idempotency"]["original_result_fingerprint"],
        appended_value["result_fingerprint"]
    );
    assert_eq!(repository_snapshot(repo.path()), committed);

    let conflict_definition = owner_definition(&registry_state_fingerprint, "1.0.1");
    let (_, conflict_dto) = outward_definition(&conflict_definition);
    let conflict = contract_definition_append_operation_request(
        Some("packet-six-conflict"),
        append_key,
        ContractDefinitionAppendRequestV1::new(conflict_dto),
    )
    .unwrap();
    let conflict = sdk
        .execute_contract_operation_with_authenticator(&conflict, &mut authenticator)
        .unwrap();
    assert_problem(&conflict, "idempotency_conflict");
    assert_eq!(repository_snapshot(repo.path()), committed);

    let ready = execute_transition(
        &sdk,
        &mut authenticator,
        &contract,
        "packet-six-ready",
        transition_input(
            &genesis,
            "draft",
            "review_ready",
            "definition_author",
            "definition_validation",
            "ready_for_review",
        ),
    );
    let ready_fingerprint = lifecycle_fingerprint(&ready);
    let ready_snapshot = repository_snapshot(repo.path());

    let stale = contract_lifecycle_transition_operation_request(
        Some("packet-six-stale"),
        "packet-six-stale-key-must-not-leak",
        ContractLifecycleTransitionRequestV1::new(
            contract.clone(),
            transition_input(
                &genesis,
                "draft",
                "closed",
                "definition_author",
                "definition_validation",
                "withdrawn",
            ),
        ),
    )
    .unwrap();
    let stale = sdk
        .execute_contract_operation_with_authenticator(&stale, &mut authenticator)
        .unwrap();
    assert_problem(&stale, "lifecycle_refused");
    assert_eq!(repository_snapshot(repo.path()), ready_snapshot);

    let locked = execute_transition(
        &sdk,
        &mut authenticator,
        &contract,
        "packet-six-lock",
        transition_input(
            &ready_fingerprint,
            "review_ready",
            "locked",
            "independent_lock",
            "independent_review",
            "independently_locked",
        ),
    );
    let locked_fingerprint = lifecycle_fingerprint(&locked);
    let active = execute_activation(&sdk, &mut authenticator, &contract, &locked_fingerprint);
    let active_fingerprint = lifecycle_fingerprint(&active);

    let owner = ContractOwnerRepositoryV1::open(repo.path()).unwrap();
    let basis = owner
        .evaluation_lifecycle_basis(
            &OwnerRef::parse(contract.reference().as_str()).unwrap(),
            &OwnerFingerprint::parse(contract.fingerprint().as_str()).unwrap(),
            &OwnerFingerprint::parse(&active_fingerprint).unwrap(),
        )
        .unwrap();
    assert_eq!(
        basis.active_resulting_lifecycle_fingerprint().as_str(),
        active_fingerprint
    );
    assert_eq!(
        owner
            .evaluation_lifecycle_basis(
                &OwnerRef::parse(contract.reference().as_str()).unwrap(),
                &OwnerFingerprint::parse(contract.fingerprint().as_str()).unwrap(),
                &OwnerFingerprint::parse(&locked_fingerprint).unwrap(),
            )
            .unwrap_err()
            .kind(),
        ContractOwnerRepositoryErrorKindV1::LifecycleRefused
    );
    drop(owner);

    prove_list_read_and_generic_consumers(repo.path(), &contract);
    let persisted = repository_bytes(repo.path());
    for forbidden in [
        append_key,
        "packet-six-ready-key-must-not-leak",
        "packet-six-stale-key-must-not-leak",
        "packet-six-lock-key-must-not-leak",
        "packet-six-active-key-must-not-leak",
    ] {
        assert!(!persisted
            .windows(forbidden.len())
            .any(|window| window == forbidden.as_bytes()));
    }
}

fn prove_claim_and_applicability_closure(claim: ContractClaimV1) {
    assert_eq!(claim.gate_effect(), GateEffectV1::Required);
    assert_eq!(claim.score_weight(), Some(ScoreWeightV1::new(1).unwrap()));
    assert_eq!(claim.evidence_requirements().len(), 3);
    assert!(serde_json::from_str::<GateEffectV1>("\"passed\"").is_err());
    assert!(serde_json::from_str::<EvidenceCardinalityV1>("\"newest_wins\"").is_err());
    assert!(ScoreWeightV1::new(0).is_err());
    assert!(ScoreWeightV1::new(9_007_199_254_740_991).is_ok());
    assert!(ScoreWeightV1::new(9_007_199_254_740_992).is_err());

    let schema = MatcherBindingV1::exact_fields_v1().input_schema().clone();
    let subject = MatcherInputCandidateV1::current(
        ApplicabilityInputSourceV1::OwnerRegistry,
        schema.clone(),
        vec![MatcherValueV1::new(owner_id("subject.kind"), "artifact").unwrap()],
    )
    .unwrap();
    let applicable_case = MatcherInputCandidateV1::current(
        ApplicabilityInputSourceV1::OwnerRegistry,
        schema.clone(),
        vec![MatcherValueV1::new(owner_id("case.kind"), "positive").unwrap()],
    )
    .unwrap();
    let inapplicable_case = MatcherInputCandidateV1::current(
        ApplicabilityInputSourceV1::OwnerRegistry,
        schema.clone(),
        vec![MatcherValueV1::new(owner_id("case.kind"), "negative").unwrap()],
    )
    .unwrap();
    let blocked_case = MatcherInputCandidateV1::current(
        ApplicabilityInputSourceV1::Dock,
        schema,
        vec![MatcherValueV1::new(owner_id("case.kind"), "positive").unwrap()],
    )
    .unwrap();
    assert_eq!(
        evaluate_applicability(&claim, &subject, &applicable_case),
        ApplicabilityOutcomeV1::Applicable
    );
    assert_eq!(
        evaluate_applicability(&claim, &subject, &inapplicable_case),
        ApplicabilityOutcomeV1::NotApplicable
    );
    assert_eq!(
        evaluate_applicability(&claim, &subject, &blocked_case),
        ApplicabilityOutcomeV1::Blocked
    );
}

fn owner_definition(registry_state_fingerprint: &str, version: &str) -> ContractDefinitionV1 {
    ContractDefinitionV1::new(ContractDefinitionInputV1 {
        definition_schema: owner_binding("handbook.contract-definition@1.0.0", 'b'),
        contract_id: ContractIdV1::parse("handbook.contract.packet-six-integrated").unwrap(),
        contract_version: VersionV1::parse(version).unwrap(),
        contract_kind: ContractKindV1::ClaimSet,
        claims: vec![owner_claim()],
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
        gate_policy: owner_binding("handbook.contract-gate-policy.closed-v1@1.0.0", 'c'),
        extensions_policy: owner_binding("handbook.contract-extensions.closed-v1@1.0.0", 'd'),
    })
    .unwrap()
}

fn owner_claim() -> ContractClaimV1 {
    let exact_cases =
        EvidenceCaseSelectionV1::exact_cases(vec![owner_id("case.one"), owner_id("case.two")])
            .unwrap();
    ContractClaimV1::new(ContractClaimInputV1 {
        claim_id: owner_id("schema.payload.valid"),
        statement: "Payload conforms to its exact schema.".to_owned(),
        subject_selector: owner_selector("subject.kind", "artifact"),
        case_selector: owner_selector("case.kind", "positive"),
        applicability_rule: ApplicabilityRuleV1::SubjectAndCaseMatch,
        gate_effect: GateEffectV1::Required,
        evidence_requirements: vec![
            evidence_requirement(
                "handbook.evidence.schema-conformance",
                EvidenceCardinalityV1::ExactlyOne,
                EvidenceCaseSelectionV1::ClaimCaseSelector,
                'e',
            ),
            evidence_requirement(
                "handbook.evidence.review",
                EvidenceCardinalityV1::AtLeastOne,
                EvidenceCaseSelectionV1::ClaimCaseSelector,
                'f',
            ),
            evidence_requirement(
                "handbook.evidence.case-coverage",
                EvidenceCardinalityV1::AllDeclaredCases,
                exact_cases,
                '7',
            ),
        ],
        freshness_policy: owner_binding("handbook.freshness.exact-revision@1.0.0", '8'),
        minimum_resolution: ResolutionMinimumV1::new(
            owner_binding("handbook.context-resolution.shipped-root@1.0.0", '9'),
            owner_id("resolution.scope.local-observation"),
            owner_id("resolution.detail.identifier-only"),
            owner_id("resolution.time.current-operation"),
            owner_id("resolution.authority.read-only"),
            owner_id("resolution.memory.operation"),
            owner_id("resolution.validation.observation-only"),
        ),
        score_weight: Some(ScoreWeightV1::new(1).unwrap()),
    })
    .unwrap()
}

fn evidence_requirement(
    evidence_kind: &str,
    cardinality: EvidenceCardinalityV1,
    cases: EvidenceCaseSelectionV1,
    hex: char,
) -> EvidenceRequirementV1 {
    EvidenceRequirementV1::new(
        owner_id(evidence_kind),
        cardinality,
        cases,
        owner_binding("handbook.stability.exact@1.0.0", hex),
    )
    .unwrap()
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

fn owner_binding(value: &str, hex: char) -> OwnerBinding {
    OwnerBinding::new(
        OwnerRef::parse(value).unwrap(),
        OwnerFingerprint::parse(&format!("sha256:{}", hex.to_string().repeat(64))).unwrap(),
    )
}

fn outward_definition(
    definition: &ContractDefinitionV1,
) -> (ExactBinding, ContractDefinitionDtoV1) {
    let contract = ExactBinding::new(
        ExactRef::parse(definition.contract_ref().as_str()).unwrap(),
        Fingerprint::parse(definition.definition_fingerprint().as_str()).unwrap(),
    );
    let document = parse_canonical_json(
        &canonical_json_bytes(definition, &ContractLimits::packet0()).unwrap(),
        &ContractLimits::packet0(),
    )
    .unwrap();
    let dto = ContractDefinitionDtoV1::new(contract.clone(), document).unwrap();
    (contract, dto)
}

fn execute_transition(
    sdk: &handbook_sdk::HandbookSdkV1,
    authenticator: &mut TestAuthenticatorPort,
    contract: &ExactBinding,
    request_id: &str,
    transition: ContractLifecycleTransitionInputDtoV1,
) -> handbook_sdk::contract_membrane::OperationResponseV1 {
    let request = contract_lifecycle_transition_operation_request(
        Some(request_id),
        &format!("{request_id}-key-must-not-leak"),
        ContractLifecycleTransitionRequestV1::new(contract.clone(), transition),
    )
    .unwrap();
    let response = sdk
        .execute_contract_operation_with_authenticator(&request, authenticator)
        .unwrap();
    assert_eq!(response.terminal().status(), TerminalStatus::Ok);
    assert_eq!(response.receipts().len(), 2);
    response
}

fn execute_activation(
    sdk: &handbook_sdk::HandbookSdkV1,
    authenticator: &mut TestAuthenticatorPort,
    contract: &ExactBinding,
    expected: &str,
) -> handbook_sdk::contract_membrane::OperationResponseV1 {
    let value = json!({
        "approval_class": "activation",
        "expected_lifecycle_fingerprint": expected,
        "from_state": "locked",
        "reason": {"kind": "activated"},
        "supporting_evidence": [
            lifecycle_evidence("applicability_closure", '3'),
            lifecycle_evidence("matcher_closure", '4'),
            lifecycle_evidence("policy_closure", '5'),
            lifecycle_evidence("currentness_closure", '6')
        ],
        "to_state": "active"
    });
    let input = parse_canonical_json(
        &canonical_json_bytes(&value, &ContractLimits::packet0()).unwrap(),
        &ContractLimits::packet0(),
    )
    .unwrap();
    let request = contract_lifecycle_transition_operation_request(
        Some("packet-six-active"),
        "packet-six-active-key-must-not-leak",
        ContractLifecycleTransitionRequestV1::new(contract.clone(), input),
    )
    .unwrap();
    let response = sdk
        .execute_contract_operation_with_authenticator(&request, authenticator)
        .unwrap();
    assert_eq!(response.terminal().status(), TerminalStatus::Ok);
    assert_eq!(response.receipts().len(), 2);
    response
}

fn transition_input(
    expected: &str,
    from_state: &str,
    to_state: &str,
    approval_class: &str,
    evidence_kind: &str,
    reason: &str,
) -> ContractLifecycleTransitionInputDtoV1 {
    let value = json!({
        "approval_class": approval_class,
        "expected_lifecycle_fingerprint": expected,
        "from_state": from_state,
        "reason": {"kind": reason},
        "supporting_evidence": [lifecycle_evidence(evidence_kind, '1')],
        "to_state": to_state
    });
    parse_canonical_json(
        &canonical_json_bytes(&value, &ContractLimits::packet0()).unwrap(),
        &ContractLimits::packet0(),
    )
    .unwrap()
}

fn lifecycle_evidence(kind: &str, hex: char) -> serde_json::Value {
    json!({
        "binding": {
            "fingerprint": format!("sha256:{}", hex.to_string().repeat(64)),
            "ref": "handbook.contract-lifecycle-evidence.packet-six@1.0.0"
        },
        "kind": kind
    })
}

fn lifecycle_fingerprint(
    response: &handbook_sdk::contract_membrane::OperationResponseV1,
) -> String {
    serde_json::to_value(response).unwrap()["data"]["lifecycle_head"]["lifecycle_fingerprint"]
        .as_str()
        .unwrap()
        .to_owned()
}

fn prove_list_read_and_generic_consumers(root: &Path, contract: &ExactBinding) {
    let sdk = handbook_sdk::HandbookSdkV1::open(root);
    let list = contract_definition_list_operation_request(
        Some("packet-six-list"),
        ContractDefinitionListRequestV1::first(1).unwrap(),
    )
    .unwrap();
    let page = sdk.execute_contract_operation(&list).unwrap();
    let page_value = serde_json::to_value(&page).unwrap();
    assert_eq!(page.terminal().status(), TerminalStatus::Ok);
    assert_eq!(page_value["data"]["retention"], "current_only");
    assert_eq!(page_value["data"]["current"], true);
    assert_eq!(page_value["data"]["entries"].as_array().unwrap().len(), 1);
    assert!(page_value["data"]["next_cursor"].is_null());

    let read = contract_definition_read_operation_request(
        Some("packet-six-read"),
        ContractDefinitionReadRequestV1::new(contract.clone()),
    )
    .unwrap();
    let found = sdk.execute_contract_operation(&read).unwrap();
    let found_value = serde_json::to_value(&found).unwrap();
    assert_eq!(found.terminal().status(), TerminalStatus::Ok);
    assert_eq!(found_value["data"]["outcome"], "found");
    assert_eq!(
        found_value["data"]["definition"]["contract"],
        json!(contract)
    );

    let substituted = contract_definition_read_operation_request(
        Some("packet-six-read-substituted"),
        ContractDefinitionReadRequestV1::new(ExactBinding::new(
            contract.reference().clone(),
            Fingerprint::parse(&format!("sha256:{}", "f".repeat(64))).unwrap(),
        )),
    )
    .unwrap();
    let substituted = sdk.execute_contract_operation(&substituted).unwrap();
    assert_problem(&substituted, "record_substituted");

    let consumer = CapabilityConsumer::open(root).unwrap();
    let page = consumer
        .catalog_read(
            CatalogKindV1::Capabilities,
            CatalogPageSize::new(128).unwrap(),
            None,
        )
        .unwrap();
    let CatalogReadResponseBodyV1::CapabilityPage(page) = page.response() else {
        panic!("capability catalog returned a non-capability page");
    };
    assert_eq!(page.root().total_entries().get(), 28);
    let contract_operations = page
        .entries()
        .iter()
        .filter(|entry| {
            entry
                .operation()
                .reference()
                .as_str()
                .starts_with("contract.")
        })
        .map(|entry| entry.operation().reference().as_str())
        .collect::<BTreeSet<_>>();
    let predecessor_operations = OPERATIONS
        .iter()
        .map(|(operation_ref, _)| *operation_ref)
        .collect::<BTreeSet<_>>();
    assert!(predecessor_operations.is_subset(&contract_operations));

    let capability = page
        .entries()
        .iter()
        .find(|entry| entry.operation().reference().as_str() == "contract.definition.list@1.0.0")
        .unwrap();
    for transport in [
        OperationTransport::RustSdk,
        OperationTransport::CliJson,
        OperationTransport::Tauri,
    ] {
        let response = consumer
            .invoke(
                capability.operation(),
                capability.entry_fingerprint(),
                transport,
                Some(RequestId::parse("packet-six-generic-list").unwrap()),
                br#"{"cursor":null,"limit":10,"root":null}"#,
            )
            .unwrap();
        assert_eq!(response.response_kind(), InvocationResponseKind::Operation);
        assert_eq!(response.terminal_status(), TerminalStatus::Ok);
        let bytes = canonical_json_bytes(&response, &ContractLimits::packet0()).unwrap();
        assert!(!String::from_utf8(bytes)
            .unwrap()
            .contains(root.to_string_lossy().as_ref()));
    }
}

fn assert_problem(response: &handbook_sdk::contract_membrane::OperationResponseV1, cause: &str) {
    assert_eq!(response.terminal().status(), TerminalStatus::Refused);
    let value = serde_json::to_value(response).unwrap();
    assert_eq!(value["refusals"][0]["details"]["cause"], cause);
    assert!(response.receipts().is_empty());
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

fn authorized_repository() -> (tempfile::TempDir, TestAuthenticatorPort, String) {
    let repo = tempfile::tempdir().unwrap();
    let mut registry =
        ApproverRegistryServiceV1::for_repository(repo.path(), TestAuthenticatorPort::new());
    let bootstrap = ApproverAdminRequestV1::from_json_value(json!({
        "schema_id":"handbook.approver-admin-request",
        "schema_version":"1.0",
        "operation":"bootstrap",
        "operation_id":"hcm-5-1-packet-six-bootstrap",
        "repository_identity_fingerprint":"sha256:4565072d36f446d7cf3adb0e0f3253725f11180d8d04d10ca982434ba364225f",
        "expected_registry_state_fingerprint":null,
        "expected_transition_fingerprint":null,
        "initial_charter_quorum":[{
            "approval_class":"contract_definition_author",
            "authority_ref":AUTHOR_AUTHORITY_REF
        }]
    }))
    .unwrap();
    let bootstrapped = registry
        .bootstrap_approver_registry(bootstrap)
        .to_json_value()
        .unwrap();
    assert_eq!(bootstrapped["status"], "succeeded", "{bootstrapped:#}");
    let add = ApproverAdminRequestV1::from_json_value(json!({
        "schema_id":"handbook.approver-admin-request",
        "schema_version":"1.0",
        "operation":"add_credential",
        "operation_id":"hcm-5-1-packet-six-independent-authority",
        "repository_identity_fingerprint":"sha256:4565072d36f446d7cf3adb0e0f3253725f11180d8d04d10ca982434ba364225f",
        "expected_registry_state_fingerprint":bootstrapped["result_registry_state_fingerprint"],
        "expected_transition_fingerprint":bootstrapped["transition_fingerprint"],
        "approval_mappings":[
            {
                "approval_class":"contract_activation",
                "authority_ref":"handbook.authority.contract-activation@1.0.0"
            },
            {
                "approval_class":"contract_independent_lock",
                "authority_ref":"handbook.authority.contract-independent-lock@1.0.0"
            },
            {
                "approval_class":"contract_lifecycle",
                "authority_ref":"handbook.authority.contract-lifecycle@1.0.0"
            }
        ]
    }))
    .unwrap();
    let added = registry
        .add_approver_credential(add)
        .to_json_value()
        .unwrap();
    assert_eq!(added["status"], "succeeded", "{added:#}");
    let observation = observe_committed_approver_registry(repo.path()).unwrap();
    let state_fingerprint = observation.state_fingerprint.clone();
    drop(observation);
    (repo, registry.into_port(), state_fingerprint)
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
