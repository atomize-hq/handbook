use handbook_contracts::{
    lifecycle_policy_binding_v1, ApplicabilityRuleV1, ContractClaimInputV1, ContractClaimV1,
    ContractDefinitionInputV1, ContractDefinitionV1, ContractIdV1, ContractKindV1,
    EvidenceCardinalityV1, EvidenceCaseSelectionV1, EvidenceRequirementV1,
    ExactBindingV1 as OwnerBinding, ExactRefV1 as OwnerRef, FingerprintV1 as OwnerFingerprint,
    GateEffectV1, MatcherBindingV1, ResolutionMinimumV1, SelectorCriterionV1, SelectorV1,
    StableIdV1, VersionV1,
};
use handbook_engine::{NativeAuthenticatorPortErrorV1, NativeAuthenticatorPortV1};
use handbook_sdk::contract_membrane::{
    admit_operation_definition, contract_operation_definitions, operation_inventory,
    AdmissionReason, CapabilityConsumer, ContractDefinitionAppendRequestV1,
    ContractDefinitionDtoV1, ContractDefinitionListRequestV1, ContractDefinitionReadRequestV1,
    ContractLimits, DiscoveryState, ExactBinding, ExactRef, Fingerprint, OperationAdmissionResult,
    OperationAvailability, OperationClass, OperationIdempotency, OperationOwner,
    OperationTransport, ProductionWitness, RequestId, TerminalStatus, UtcTimestamp,
};
use sha2::{Digest, Sha256};

const EXPECTED: [(&str, OperationClass); 4] = [
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
fn packet_four_closes_exactly_the_reserved_contract_operation_rows() {
    let definitions = contract_operation_definitions().expect("contract definitions");
    assert_eq!(definitions.len(), EXPECTED.len());

    for ((expected_ref, expected_class), definition) in EXPECTED.iter().zip(&definitions) {
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
            admit_operation_definition(definition).unwrap(),
            OperationAdmissionResult::Admitted(_)
        ));
        assert!(matches!(
            (expected_class, definition.idempotency()),
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

        let row = operation_inventory()
            .iter()
            .find(|row| row.operation_ref().as_str() == *expected_ref)
            .expect("reserved inventory row");
        assert_eq!(row.owner(), OperationOwner::Contracts);
        assert_eq!(row.availability(), OperationAvailability::LivePrecursor);
        assert_eq!(row.admission_reason(), AdmissionReason::Admitted);
        assert_eq!(row.discovery(), DiscoveryState::Discoverable);
        assert_eq!(row.current_transports(), definition.declared_transports());
    }

    assert_eq!(operation_inventory().len(), 62);
}

#[test]
fn list_read_and_mutation_refusal_preserve_owner_truth_and_generic_transport_evidence() {
    let repo = tempfile::tempdir().unwrap();
    let sdk = handbook_sdk::HandbookSdkV1::open(repo.path());
    let mut unavailable = UnavailableAuthenticator;

    let list = handbook_sdk::contract_membrane::contract_definition_list_operation_request(
        Some("contract-list-direct"),
        ContractDefinitionListRequestV1::first(10).unwrap(),
    )
    .unwrap();
    let listed = sdk.execute_contract_operation(&list).unwrap();
    assert_eq!(listed.terminal().status(), TerminalStatus::Ok);
    assert!(listed.receipts().is_empty());

    let owner_definition = fixture_definition();
    let contract = ExactBinding::new(
        ExactRef::parse(owner_definition.contract_ref().as_str()).unwrap(),
        Fingerprint::parse(owner_definition.definition_fingerprint().as_str()).unwrap(),
    );
    let read = handbook_sdk::contract_membrane::contract_definition_read_operation_request(
        Some("contract-read-direct"),
        ContractDefinitionReadRequestV1::new(contract.clone()),
    )
    .unwrap();
    let absent = sdk.execute_contract_operation(&read).unwrap();
    assert_eq!(absent.terminal().status(), TerminalStatus::Ok);
    assert!(absent.receipts().is_empty());

    let document = handbook_sdk::contract_membrane::parse_canonical_json(
        &handbook_sdk::contract_membrane::canonical_json_bytes(
            &owner_definition,
            &ContractLimits::packet0(),
        )
        .unwrap(),
        &ContractLimits::packet0(),
    )
    .unwrap();
    let append = handbook_sdk::contract_membrane::contract_definition_append_operation_request(
        Some("contract-append-no-authority"),
        "raw-owner-key-must-not-leak",
        ContractDefinitionAppendRequestV1::new(
            ContractDefinitionDtoV1::new(contract, document).unwrap(),
        ),
    )
    .unwrap();
    let refused = sdk
        .execute_contract_operation_with_authenticator(&append, &mut unavailable)
        .unwrap();
    assert_eq!(refused.terminal().status(), TerminalStatus::Refused);
    assert!(refused.receipts().is_empty());
    let response_bytes =
        handbook_sdk::contract_membrane::canonical_json_bytes(&refused, &ContractLimits::packet0())
            .unwrap();
    assert!(!String::from_utf8(response_bytes)
        .unwrap()
        .contains("raw-owner-key-must-not-leak"));

    let witness =
        ProductionWitness::from_current(UtcTimestamp::parse("9999-12-31T23:59:59Z").unwrap())
            .unwrap();
    let capability = witness
        .capabilities()
        .iter()
        .find(|entry| entry.operation().reference().as_str() == "contract.definition.list@1.0.0")
        .unwrap();
    let consumer = CapabilityConsumer::open(repo.path()).unwrap();
    let body = br#"{"cursor":null,"limit":10,"root":null}"#;
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
                Some(RequestId::parse("contract-generic-transport").unwrap()),
                body,
            )
            .unwrap();
        assert_eq!(response.terminal_status(), TerminalStatus::Ok);
    }
}

struct UnavailableAuthenticator;

impl NativeAuthenticatorPortV1 for UnavailableAuthenticator {
    fn make_credential(
        &mut self,
        _request_cbor: &[u8],
    ) -> Result<Vec<u8>, NativeAuthenticatorPortErrorV1> {
        Err(NativeAuthenticatorPortErrorV1::Unavailable)
    }

    fn get_assertion(
        &mut self,
        _request_cbor: &[u8],
    ) -> Result<Vec<u8>, NativeAuthenticatorPortErrorV1> {
        Err(NativeAuthenticatorPortErrorV1::Unavailable)
    }
}

fn fixture_definition() -> ContractDefinitionV1 {
    ContractDefinitionV1::new(ContractDefinitionInputV1 {
        definition_schema: owner_binding("handbook.contract-definition@1.0.0", 'b'),
        contract_id: ContractIdV1::parse("handbook.contract.sdk-packet-four").unwrap(),
        contract_version: VersionV1::parse("1.0.0").unwrap(),
        contract_kind: ContractKindV1::ClaimSet,
        claims: vec![fixture_claim()],
        definition_author_ref: StableIdV1::parse(&format!(
            "credential.sha256-{:x}",
            Sha256::digest([0x32; 32])
        ))
        .unwrap(),
        definition_authority: owner_binding(
            "handbook.authority.contract-definition-author@1.0.0",
            'a',
        ),
        lifecycle_policy: lifecycle_policy_binding_v1().unwrap(),
        matcher_bindings: vec![MatcherBindingV1::exact_fields_v1()],
        gate_policy: owner_binding("handbook.contract-gate-policy.closed-v1@1.0.0", 'c'),
        extensions_policy: owner_binding("handbook.contract-extensions.closed-v1@1.0.0", 'd'),
    })
    .unwrap()
}

fn fixture_claim() -> ContractClaimV1 {
    ContractClaimV1::new(ContractClaimInputV1 {
        claim_id: stable_id("schema.payload.valid"),
        statement: "Payload conforms to its exact schema.".to_owned(),
        subject_selector: selector("subject.kind", "artifact"),
        case_selector: selector("case.kind", "positive"),
        applicability_rule: ApplicabilityRuleV1::SubjectAndCaseMatch,
        gate_effect: GateEffectV1::Required,
        evidence_requirements: vec![EvidenceRequirementV1::new(
            stable_id("handbook.evidence.schema-conformance"),
            EvidenceCardinalityV1::ExactlyOne,
            EvidenceCaseSelectionV1::ClaimCaseSelector,
            owner_binding("handbook.stability.exact@1.0.0", 'e'),
        )
        .unwrap()],
        freshness_policy: owner_binding("handbook.freshness.exact-revision@1.0.0", 'f'),
        minimum_resolution: ResolutionMinimumV1::new(
            owner_binding("handbook.context-resolution.shipped-root@1.0.0", '1'),
            stable_id("resolution.scope.local-observation"),
            stable_id("resolution.detail.identifier-only"),
            stable_id("resolution.time.current-operation"),
            stable_id("resolution.authority.read-only"),
            stable_id("resolution.memory.operation"),
            stable_id("resolution.validation.observation-only"),
        ),
        score_weight: None,
    })
    .unwrap()
}

fn selector(field: &str, expected: &str) -> SelectorV1 {
    SelectorV1::new(
        MatcherBindingV1::exact_fields_v1(),
        vec![SelectorCriterionV1::new(stable_id(field), expected).unwrap()],
    )
    .unwrap()
}

fn stable_id(value: &str) -> StableIdV1 {
    StableIdV1::parse(value).unwrap()
}

fn owner_binding(value: &str, hex: char) -> OwnerBinding {
    OwnerBinding::new(
        OwnerRef::parse(value).unwrap(),
        OwnerFingerprint::parse(&format!("sha256:{}", hex.to_string().repeat(64))).unwrap(),
    )
}
