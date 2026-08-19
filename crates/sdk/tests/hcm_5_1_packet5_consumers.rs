use handbook_contracts::{
    lifecycle_policy_binding_v1, ApplicabilityRuleV1, ContractClaimInputV1, ContractClaimV1,
    ContractDefinitionInputV1, ContractDefinitionV1, ContractIdV1, ContractKindV1,
    ContractOwnerRepositoryErrorKindV1, ContractOwnerRepositoryV1, EvidenceCardinalityV1,
    EvidenceCaseSelectionV1, EvidenceRequirementV1, ExactBindingV1 as OwnerBinding,
    ExactRefV1 as OwnerRef, FingerprintV1 as OwnerFingerprint, GateEffectV1, MatcherBindingV1,
    ResolutionMinimumV1, SelectorCriterionV1, SelectorV1, StableIdV1, VersionV1,
};
use handbook_engine::{
    observe_committed_approver_registry, ApproverAdminRequestV1, ApproverRegistryServiceV1,
    NativeAuthenticatorPortErrorV1, NativeAuthenticatorPortV1, AUTHENTICATOR_RP_ID,
};
use handbook_sdk::contract_membrane::{
    canonical_json_bytes, contract_definition_append_operation_request,
    contract_definition_list_operation_request, contract_definition_read_operation_request,
    contract_lifecycle_transition_operation_request, contract_operation_definitions,
    operation_inventory, parse_canonical_json, sha256_fingerprint, AdmissionReason,
    CapabilityConsumer, CapabilityConsumerError, CatalogKindV1, CatalogPageSize,
    CatalogReadResponseBodyV1, ContractDefinitionAppendRequestV1, ContractDefinitionDtoV1,
    ContractDefinitionListRequestV1, ContractDefinitionReadRequestV1,
    ContractLifecycleTransitionInputDtoV1, ContractLifecycleTransitionRequestV1, ContractLimits,
    DiscoveryState, ExactBinding, ExactRef, Fingerprint, InvocationResponseKind, OperationClass,
    OperationIdempotency, OperationTransport, ProductionWitness, RequestId, TerminalStatus,
    UtcTimestamp,
};
use p256::ecdsa::{signature::Signer, Signature, SigningKey};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

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
fn four_contract_operations_have_exact_discovery_schema_owner_and_transport_parity() {
    let retention = UtcTimestamp::parse("9999-12-31T23:59:59Z").unwrap();
    let witness = ProductionWitness::from_current(retention).unwrap();
    let capabilities = witness
        .capabilities()
        .iter()
        .map(|entry| (entry.operation().reference().as_str(), entry))
        .collect::<BTreeMap<_, _>>();
    let manifest = witness
        .schema_manifest()
        .iter()
        .map(|entry| (entry.schema().reference().as_str(), entry.schema()))
        .collect::<BTreeMap<_, _>>();
    let definitions = contract_operation_definitions().unwrap();
    let rows = operation_inventory();

    assert_eq!(definitions.len(), OPERATIONS.len());
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

        let capability = capabilities.get(operation_ref).unwrap();
        assert_eq!(capability.class(), *class);
        assert_eq!(capability.operation(), &definition.binding());
        assert_eq!(capability.owner(), definition.owner());
        assert_eq!(
            capability.declared_transports(),
            definition.declared_transports()
        );
        assert_eq!(
            capability.current_transports(),
            definition.declared_transports()
        );

        for schema in [
            capability.request_schema(),
            capability.result_schema(),
            capability.outcome_schemas().blocker(),
            capability.outcome_schemas().refusal(),
            capability.outcome_schemas().error(),
        ] {
            assert_eq!(manifest.get(schema.reference().as_str()), Some(&schema));
        }

        let row = rows
            .iter()
            .find(|row| row.operation_ref().as_str() == *operation_ref)
            .unwrap();
        assert_eq!(row.admission_reason(), AdmissionReason::Admitted);
        assert_eq!(row.discovery(), DiscoveryState::Discoverable);
        assert_eq!(row.current_transports(), definition.declared_transports());
    }

    let current_contract_operations = capabilities
        .keys()
        .filter(|operation_ref| operation_ref.starts_with("contract."))
        .copied()
        .collect::<BTreeSet<_>>();
    let predecessor_operations = OPERATIONS
        .iter()
        .map(|(operation_ref, _)| *operation_ref)
        .collect::<BTreeSet<_>>();
    assert!(predecessor_operations.is_subset(&current_contract_operations));
}

#[test]
fn generic_sdk_discovers_reads_schemas_and_invokes_read_only_contract_operations() {
    let repo = tempfile::tempdir().unwrap();
    let consumer = CapabilityConsumer::open(repo.path()).unwrap();
    let page = consumer
        .catalog_read(
            CatalogKindV1::Capabilities,
            CatalogPageSize::new(128).unwrap(),
            None,
        )
        .unwrap();
    let CatalogReadResponseBodyV1::CapabilityPage(page) = page.response() else {
        panic!("capability catalog returned a non-capability body");
    };

    for (operation_ref, body) in [
        (
            "contract.definition.list@1.0.0",
            br#"{"cursor":null,"limit":10,"root":null}"#.as_slice(),
        ),
        (
            "contract.definition.read@1.0.0",
            br#"{"contract":{"fingerprint":"sha256:0000000000000000000000000000000000000000000000000000000000000000","ref":"handbook.contract.absent@1.0.0"}}"#
                .as_slice(),
        ),
    ] {
        let capability = page
            .entries()
            .iter()
            .find(|entry| entry.operation().reference().as_str() == operation_ref)
            .unwrap();
        let schema = consumer.schema_bytes(capability.request_schema()).unwrap();
        assert_eq!(
            sha256_fingerprint(schema),
            capability.request_schema().fingerprint().clone()
        );

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
                    Some(RequestId::parse("packet-five-read-only").unwrap()),
                    body,
                )
                .unwrap();
            assert_eq!(response.response_kind(), InvocationResponseKind::Operation);
            assert_eq!(response.terminal_status(), TerminalStatus::Ok);
            let bytes = handbook_sdk::contract_membrane::canonical_json_bytes(
                &response,
                &ContractLimits::packet0(),
            )
            .unwrap();
            assert!(!String::from_utf8(bytes)
                .unwrap()
                .contains(repo.path().to_string_lossy().as_ref()));
        }

        let substituted_operation = ExactBinding::new(
            capability.operation().reference().clone(),
            Fingerprint::parse(&format!("sha256:{}", "0".repeat(64))).unwrap(),
        );
        assert_eq!(
            consumer.invoke(
                &substituted_operation,
                capability.entry_fingerprint(),
                OperationTransport::RustSdk,
                None,
                body,
            ),
            Err(CapabilityConsumerError::DefinitionBindingMismatch)
        );
        assert_eq!(
            consumer.invoke(
                capability.operation(),
                &Fingerprint::parse(&format!("sha256:{}", "1".repeat(64))).unwrap(),
                OperationTransport::RustSdk,
                None,
                body,
            ),
            Err(CapabilityConsumerError::CapabilityEntryMismatch)
        );
    }
}

#[test]
fn authorized_sdk_mutations_preserve_replay_conflict_list_read_and_lifecycle_truth() {
    let (repo, mut port, registry_state_fingerprint) = authorized_repository();
    let sdk = handbook_sdk::HandbookSdkV1::open(repo.path());
    let first_definition = owner_definition(&registry_state_fingerprint, "1.0.0");
    let (first_contract, first_dto) = outward_definition(&first_definition);
    let append_key = "packet-five-authorized-append-0001";
    let append = contract_definition_append_operation_request(
        Some("packet-five-append"),
        append_key,
        ContractDefinitionAppendRequestV1::new(first_dto),
    )
    .unwrap();

    let before = repository_snapshot(repo.path());
    let first = sdk
        .execute_contract_operation_with_authenticator(&append, &mut port)
        .unwrap();
    let first_value = serde_json::to_value(&first).unwrap();
    assert_eq!(
        first.terminal().status(),
        TerminalStatus::Ok,
        "{first_value:#}"
    );
    assert_eq!(first_value["idempotency"]["state"], "established");
    assert_eq!(first_value["idempotency"]["replayed"], false);
    assert_eq!(first_value["receipts"].as_array().unwrap().len(), 2);
    let genesis = first_value["data"]["lifecycle_head"]["lifecycle_fingerprint"]
        .as_str()
        .unwrap()
        .to_owned();
    let committed = repository_snapshot(repo.path());
    assert_ne!(committed, before);

    let replay = sdk
        .execute_contract_operation_with_authenticator(&append, &mut port)
        .unwrap();
    let replay_value = serde_json::to_value(&replay).unwrap();
    assert_eq!(replay.terminal().status(), TerminalStatus::Ok);
    assert_eq!(replay_value["idempotency"]["replayed"], true);
    assert_eq!(
        replay_value["idempotency"]["original_result_fingerprint"],
        first_value["result_fingerprint"]
    );
    assert_eq!(repository_snapshot(repo.path()), committed);

    let conflicting_definition = owner_definition(&registry_state_fingerprint, "1.0.1");
    let (_, conflicting_dto) = outward_definition(&conflicting_definition);
    let conflict_request = contract_definition_append_operation_request(
        Some("packet-five-append-conflict"),
        append_key,
        ContractDefinitionAppendRequestV1::new(conflicting_dto),
    )
    .unwrap();
    let conflict = sdk
        .execute_contract_operation_with_authenticator(&conflict_request, &mut port)
        .unwrap();
    assert_problem(&conflict, TerminalStatus::Refused, "idempotency_conflict");
    assert!(conflict.receipts().is_empty());
    assert_eq!(repository_snapshot(repo.path()), committed);

    let (_, second_dto) = outward_definition(&conflicting_definition);
    let second_append = contract_definition_append_operation_request(
        Some("packet-five-append-second"),
        "packet-five-authorized-append-0002",
        ContractDefinitionAppendRequestV1::new(second_dto),
    )
    .unwrap();
    let second = sdk
        .execute_contract_operation_with_authenticator(&second_append, &mut port)
        .unwrap();
    assert_eq!(second.terminal().status(), TerminalStatus::Ok);

    let list = contract_definition_list_operation_request(
        Some("packet-five-list"),
        ContractDefinitionListRequestV1::first(1).unwrap(),
    )
    .unwrap();
    let first_page = sdk.execute_contract_operation(&list).unwrap();
    let first_page_value = serde_json::to_value(&first_page).unwrap();
    assert_eq!(first_page.terminal().status(), TerminalStatus::Ok);
    assert_eq!(first_page_value["data"]["retention"], "current_only");
    assert_eq!(first_page_value["data"]["current"], true);
    assert_eq!(
        first_page_value["data"]["entries"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    let root = first_page_value["data"]["root"].as_str().unwrap();
    let cursor = first_page_value["data"]["next_cursor"].as_str().unwrap();
    let continuation = contract_definition_list_operation_request(
        Some("packet-five-list-continue"),
        ContractDefinitionListRequestV1::continue_from(
            1,
            Fingerprint::parse(root).unwrap(),
            cursor.to_owned(),
        )
        .unwrap(),
    )
    .unwrap();
    let second_page = sdk.execute_contract_operation(&continuation).unwrap();
    let second_page_value = serde_json::to_value(&second_page).unwrap();
    assert_eq!(second_page.terminal().status(), TerminalStatus::Ok);
    assert_eq!(second_page_value["data"]["root"], root);
    assert_eq!(
        second_page_value["data"]["entries"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    assert!(second_page_value["data"]["next_cursor"].is_null());

    let substituted_root = contract_definition_list_operation_request(
        Some("packet-five-list-substituted-root"),
        ContractDefinitionListRequestV1::continue_from(
            1,
            Fingerprint::parse(&format!("sha256:{}", "0".repeat(64))).unwrap(),
            cursor.to_owned(),
        )
        .unwrap(),
    )
    .unwrap();
    let substituted_root = sdk.execute_contract_operation(&substituted_root).unwrap();
    assert_problem(
        &substituted_root,
        TerminalStatus::Refused,
        "record_substituted",
    );

    let read = contract_definition_read_operation_request(
        Some("packet-five-read"),
        ContractDefinitionReadRequestV1::new(first_contract.clone()),
    )
    .unwrap();
    let found = sdk.execute_contract_operation(&read).unwrap();
    let found_value = serde_json::to_value(&found).unwrap();
    assert_eq!(found.terminal().status(), TerminalStatus::Ok);
    assert_eq!(found_value["data"]["outcome"], "found");
    assert_eq!(
        found_value["data"]["definition"]["contract"],
        json!(first_contract)
    );

    let substituted_contract = ExactBinding::new(
        first_contract.reference().clone(),
        Fingerprint::parse(&format!("sha256:{}", "f".repeat(64))).unwrap(),
    );
    let substituted_read = contract_definition_read_operation_request(
        Some("packet-five-read-substituted"),
        ContractDefinitionReadRequestV1::new(substituted_contract),
    )
    .unwrap();
    let substituted_read = sdk.execute_contract_operation(&substituted_read).unwrap();
    assert_problem(
        &substituted_read,
        TerminalStatus::Refused,
        "record_substituted",
    );

    let ready_input = transition_input(
        &genesis,
        "draft",
        "review_ready",
        "definition_author",
        "definition_validation",
        "ready_for_review",
    );
    let ready_request = contract_lifecycle_transition_operation_request(
        Some("packet-five-ready"),
        "packet-five-ready-key-0001",
        ContractLifecycleTransitionRequestV1::new(first_contract.clone(), ready_input.clone()),
    )
    .unwrap();
    let ready = sdk
        .execute_contract_operation_with_authenticator(&ready_request, &mut port)
        .unwrap();
    let ready_value = serde_json::to_value(&ready).unwrap();
    assert_eq!(ready.terminal().status(), TerminalStatus::Ok);
    assert_eq!(
        ready_value["data"]["lifecycle_head"]["state"],
        "review_ready"
    );
    assert_eq!(ready_value["receipts"].as_array().unwrap().len(), 2);
    let ready_fingerprint = ready_value["data"]["lifecycle_head"]["lifecycle_fingerprint"]
        .as_str()
        .unwrap()
        .to_owned();
    let ready_snapshot = repository_snapshot(repo.path());

    let stale = contract_lifecycle_transition_operation_request(
        Some("packet-five-stale-head"),
        "packet-five-stale-head-key-0001",
        ContractLifecycleTransitionRequestV1::new(
            first_contract.clone(),
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
        .execute_contract_operation_with_authenticator(&stale, &mut port)
        .unwrap();
    assert_problem(&stale, TerminalStatus::Refused, "lifecycle_refused");
    assert_eq!(repository_snapshot(repo.path()), ready_snapshot);

    let self_lock = contract_lifecycle_transition_operation_request(
        Some("packet-five-self-lock"),
        "packet-five-self-lock-key-0001",
        ContractLifecycleTransitionRequestV1::new(
            first_contract.clone(),
            transition_input(
                &ready_fingerprint,
                "review_ready",
                "locked",
                "independent_lock",
                "independent_review",
                "independently_locked",
            ),
        ),
    )
    .unwrap();
    let self_lock = sdk
        .execute_contract_operation_with_authenticator(&self_lock, &mut port)
        .unwrap();
    assert_problem(&self_lock, TerminalStatus::Refused, "lifecycle_refused");
    assert_eq!(repository_snapshot(repo.path()), ready_snapshot);

    let owner = ContractOwnerRepositoryV1::open(repo.path()).unwrap();
    let basis = owner.evaluation_lifecycle_basis(
        &OwnerRef::parse(first_contract.reference().as_str()).unwrap(),
        &OwnerFingerprint::parse(first_contract.fingerprint().as_str()).unwrap(),
        &OwnerFingerprint::parse(&ready_fingerprint).unwrap(),
    );
    assert_eq!(
        basis.unwrap_err().kind(),
        ContractOwnerRepositoryErrorKindV1::LifecycleRefused
    );

    let persisted = repository_bytes(repo.path());
    for secret in [
        append_key,
        "packet-five-authorized-append-0002",
        "packet-five-ready-key-0001",
        "packet-five-stale-head-key-0001",
        "packet-five-self-lock-key-0001",
    ] {
        assert!(!persisted
            .windows(secret.len())
            .any(|window| window == secret.as_bytes()));
    }
}

#[test]
fn generic_transports_cannot_turn_valid_contract_mutations_into_owner_authority() {
    let (repo, mut port, registry_state_fingerprint) = authorized_repository();
    let definition = owner_definition(&registry_state_fingerprint, "2.0.0");
    let (contract, dto) = outward_definition(&definition);
    let sdk = handbook_sdk::HandbookSdkV1::open(repo.path());
    let seed = contract_definition_append_operation_request(
        Some("packet-five-generic-seed"),
        "packet-five-generic-seed-key-0001",
        ContractDefinitionAppendRequestV1::new(dto.clone()),
    )
    .unwrap();
    let seeded = sdk
        .execute_contract_operation_with_authenticator(&seed, &mut port)
        .unwrap();
    assert_eq!(seeded.terminal().status(), TerminalStatus::Ok);
    let seeded_value = serde_json::to_value(&seeded).unwrap();
    let genesis = seeded_value["data"]["lifecycle_head"]["lifecycle_fingerprint"]
        .as_str()
        .unwrap();
    let append_key = "packet-five-generic-authority-key-0001";
    let append_body = canonical_json_bytes(
        &json!({"definition": dto, "idempotency_key": append_key}),
        &ContractLimits::packet0(),
    )
    .unwrap();
    let transition_key = "packet-five-generic-transition-key-0001";
    let transition_body = canonical_json_bytes(
        &json!({
            "contract": contract,
            "idempotency_key": transition_key,
            "transition": transition_input(
                genesis,
                "draft",
                "review_ready",
                "definition_author",
                "definition_validation",
                "ready_for_review",
            ),
        }),
        &ContractLimits::packet0(),
    )
    .unwrap();
    let consumer = CapabilityConsumer::open(repo.path()).unwrap();
    let witness =
        ProductionWitness::from_current(UtcTimestamp::parse("9999-12-31T23:59:59Z").unwrap())
            .unwrap();
    let before = repository_snapshot(repo.path());

    for (operation_ref, body) in [
        ("contract.definition.append@1.0.0", append_body.as_slice()),
        (
            "contract.lifecycle.transition@1.0.0",
            transition_body.as_slice(),
        ),
    ] {
        let capability = witness
            .capabilities()
            .iter()
            .find(|entry| entry.operation().reference().as_str() == operation_ref)
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
                    Some(RequestId::parse("packet-five-generic-mutation").unwrap()),
                    body,
                )
                .unwrap();
            let value = serde_json::to_value(&response).unwrap();
            assert_eq!(
                response.terminal_status(),
                TerminalStatus::Blocked,
                "{value:#}"
            );
            assert_eq!(
                value["response"]["blockers"][0]["details"]["cause"],
                "authenticator_unavailable"
            );
            let bytes = canonical_json_bytes(&response, &ContractLimits::packet0()).unwrap();
            for secret in [append_key, transition_key] {
                assert!(!bytes
                    .windows(secret.len())
                    .any(|window| window == secret.as_bytes()));
            }
            assert!(!String::from_utf8(bytes)
                .unwrap()
                .contains(repo.path().to_string_lossy().as_ref()));
        }
    }

    let append_capability = witness
        .capabilities()
        .iter()
        .find(|entry| entry.operation().reference().as_str() == "contract.definition.append@1.0.0")
        .unwrap();
    let invalid =
        br#"{"credential":"transport-supplied","definition":{},"idempotency_key":"must-not-run"}"#;
    assert_eq!(
        consumer.invoke(
            append_capability.operation(),
            append_capability.entry_fingerprint(),
            OperationTransport::RustSdk,
            None,
            invalid,
        ),
        Err(CapabilityConsumerError::InvalidInput)
    );
    assert_eq!(repository_snapshot(repo.path()), before);
    let persisted = repository_bytes(repo.path());
    for secret in [
        "packet-five-generic-seed-key-0001",
        append_key,
        transition_key,
        "transport-supplied",
        "must-not-run",
    ] {
        assert!(!persisted
            .windows(secret.len())
            .any(|window| window == secret.as_bytes()));
    }
}

const REPOSITORY_IDENTITY: &str =
    "sha256:1111111111111111111111111111111111111111111111111111111111111111";
const AUTHOR_AUTHORITY_REF: &str = "handbook.authority.contract-definition-author@1.0.0";

struct TestAuthenticatorPort {
    key: SigningKey,
    credential_id: Vec<u8>,
    make_calls: usize,
    assertions: u32,
}

impl TestAuthenticatorPort {
    fn new() -> Self {
        Self {
            key: SigningKey::from_slice(&[7_u8; 32]).unwrap(),
            credential_id: vec![0x32; 32],
            make_calls: 0,
            assertions: 0,
        }
    }
}

impl NativeAuthenticatorPortV1 for TestAuthenticatorPort {
    fn make_credential(
        &mut self,
        _request_cbor: &[u8],
    ) -> Result<Vec<u8>, NativeAuthenticatorPortErrorV1> {
        self.make_calls += 1;
        Ok(make_credential_response(&self.key, &self.credential_id))
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
        self.assertions += 1;
        Ok(assertion_response(
            &self.key,
            &self.credential_id,
            client_data_hash,
            self.assertions,
        ))
    }
}

fn authorized_repository() -> (tempfile::TempDir, TestAuthenticatorPort, String) {
    let repo = tempfile::tempdir().unwrap();
    let mut registry =
        ApproverRegistryServiceV1::for_repository(repo.path(), TestAuthenticatorPort::new());
    let request = ApproverAdminRequestV1::from_json_value(json!({
        "schema_id":"handbook.approver-admin-request",
        "schema_version":"1.0",
        "operation":"bootstrap",
        "operation_id":"hcm-5-1-packet-five-bootstrap",
        "repository_identity_fingerprint":REPOSITORY_IDENTITY,
        "expected_registry_state_fingerprint":null,
        "expected_transition_fingerprint":null,
        "initial_charter_quorum":[
            {
                "approval_class":"contract_definition_author",
                "authority_ref":AUTHOR_AUTHORITY_REF
            },
            {
                "approval_class":"contract_independent_lock",
                "authority_ref":"handbook.authority.contract-independent-lock@1.0.0"
            },
            {
                "approval_class":"contract_activation",
                "authority_ref":"handbook.authority.contract-activation@1.0.0"
            },
            {
                "approval_class":"contract_lifecycle",
                "authority_ref":"handbook.authority.contract-lifecycle@1.0.0"
            }
        ]
    }))
    .unwrap();
    let result = registry
        .bootstrap_approver_registry(request)
        .to_json_value()
        .unwrap();
    assert_eq!(result["status"], "succeeded", "{result:#}");
    let observation = observe_committed_approver_registry(repo.path()).unwrap();
    let state_fingerprint = observation.state_fingerprint.clone();
    drop(observation);
    (repo, registry.into_port(), state_fingerprint)
}

fn owner_definition(registry_state_fingerprint: &str, version: &str) -> ContractDefinitionV1 {
    ContractDefinitionV1::new(ContractDefinitionInputV1 {
        definition_schema: owner_binding("handbook.contract-definition@1.0.0", 'b'),
        contract_id: ContractIdV1::parse("handbook.contract.packet-five").unwrap(),
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
    ContractClaimV1::new(ContractClaimInputV1 {
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
            owner_binding("handbook.stability.exact@1.0.0", 'e'),
        )
        .unwrap()],
        freshness_policy: owner_binding("handbook.freshness.exact-revision@1.0.0", 'f'),
        minimum_resolution: ResolutionMinimumV1::new(
            owner_binding("handbook.context-resolution.shipped-root@1.0.0", '1'),
            owner_id("resolution.scope.local-observation"),
            owner_id("resolution.detail.identifier-only"),
            owner_id("resolution.time.current-operation"),
            owner_id("resolution.authority.read-only"),
            owner_id("resolution.memory.operation"),
            owner_id("resolution.validation.observation-only"),
        ),
        score_weight: None,
    })
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
        "supporting_evidence": [{
            "binding": {
                "fingerprint": format!("sha256:{}", "1".repeat(64)),
                "ref": "handbook.contract-lifecycle-evidence.packet-five@1.0.0"
            },
            "kind": evidence_kind
        }],
        "to_state": to_state
    });
    parse_canonical_json(
        &canonical_json_bytes(&value, &ContractLimits::packet0()).unwrap(),
        &ContractLimits::packet0(),
    )
    .unwrap()
}

fn assert_problem(
    response: &handbook_sdk::contract_membrane::OperationResponseV1,
    status: TerminalStatus,
    cause: &str,
) {
    assert_eq!(response.terminal().status(), status);
    let value = serde_json::to_value(response).unwrap();
    let problems = if status == TerminalStatus::Blocked {
        &value["blockers"]
    } else {
        &value["refusals"]
    };
    assert_eq!(problems[0]["details"]["cause"], cause);
    assert!(response.receipts().is_empty());
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
