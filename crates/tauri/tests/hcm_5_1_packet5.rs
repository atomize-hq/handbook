use handbook_contracts::{
    lifecycle_policy_binding_v1, ApplicabilityRuleV1, ContractClaimInputV1, ContractClaimV1,
    ContractDefinitionInputV1, ContractDefinitionV1, ContractIdV1, ContractKindV1,
    EvidenceCardinalityV1, EvidenceCaseSelectionV1, EvidenceRequirementV1, ExactBindingV1,
    ExactRefV1, FingerprintV1, GateEffectV1, MatcherBindingV1, ResolutionMinimumV1,
    SelectorCriterionV1, SelectorV1, StableIdV1, VersionV1,
};
use handbook_sdk::contract_membrane::{
    canonical_json_bytes, contract_definition_append_operation_request,
    contract_definition_list_operation_request, contract_definition_read_operation_request,
    contract_lifecycle_transition_operation_request, parse_canonical_json,
    BootstrapFacadeResponseV1, BootstrapRequestV1, CatalogKindV1, CatalogPageSize,
    CatalogReadRequestV1, CatalogReadResponseBodyV1, CatalogReadResponseV1,
    ContractDefinitionAppendRequestV1, ContractDefinitionDtoV1, ContractDefinitionListRequestV1,
    ContractDefinitionReadRequestV1, ContractLifecycleTransitionInputDtoV1,
    ContractLifecycleTransitionRequestV1, ContractLimits, ExactBinding, ExactRef, Fingerprint,
    InvocationRequestV1, ProductionWitness, SemVer, UtcTimestamp,
};
use handbook_tauri::{FacadeState, TrustedClock};
use serde_json::json;
use std::collections::BTreeMap;
use tauri::ipc::{CallbackFn, InvokeBody, InvokeResponseBody};
use tauri::test::{
    get_ipc_response, mock_builder, mock_context, noop_assets, MockRuntime, INVOKE_KEY,
};
use tauri::webview::InvokeRequest;

const OPERATIONS: [&str; 4] = [
    "contract.definition.list@1.0.0",
    "contract.definition.read@1.0.0",
    "contract.definition.append@1.0.0",
    "contract.lifecycle.transition@1.0.0",
];

#[derive(Clone)]
struct FixedClock(UtcTimestamp);

impl TrustedClock for FixedClock {
    fn now_utc(&self) -> Option<UtcTimestamp> {
        Some(self.0.clone())
    }
}

fn invoke_ipc(
    webview: &tauri::WebviewWindow<MockRuntime>,
    command: &str,
    request: impl serde::Serialize,
) -> Vec<u8> {
    match get_ipc_response(
        webview,
        InvokeRequest {
            cmd: command.into(),
            callback: CallbackFn(0),
            error: CallbackFn(1),
            url: if cfg!(any(windows, target_os = "android")) {
                "http://tauri.localhost"
            } else {
                "tauri://localhost"
            }
            .parse()
            .unwrap(),
            body: InvokeBody::Json(json!({"request": request})),
            headers: Default::default(),
            invoke_key: INVOKE_KEY.to_string(),
        },
    )
    .unwrap_or_else(|error| panic!("{command}: {error:?}"))
    {
        InvokeResponseBody::Json(value) => value.into_bytes(),
        InvokeResponseBody::Raw(value) => value,
    }
}

#[test]
fn generic_tauri_facade_preserves_discovery_schema_and_invocation_envelope_parity() {
    let repo = tempfile::tempdir().unwrap();
    let retention = UtcTimestamp::parse("2030-01-01T00:00:00Z").unwrap();
    let now = UtcTimestamp::parse("2029-01-01T00:00:00Z").unwrap();
    let witness = ProductionWitness::from_current(retention.clone()).unwrap();
    let state = FacadeState::open(repo.path(), retention, FixedClock(now)).unwrap();
    let app = mock_builder()
        .manage(state)
        .invoke_handler(handbook_tauri::invoke_handler())
        .build(mock_context(noop_assets()))
        .unwrap();
    let webview = tauri::WebviewWindowBuilder::new(&app, "packet-five", Default::default())
        .build()
        .unwrap();
    let limits = ContractLimits::packet0();

    let bootstrap_request = BootstrapRequestV1::new(
        witness.descriptor().schemas().bootstrap_request().clone(),
        None,
        witness.descriptor().binding(),
        SemVer::parse("1.0.0").unwrap(),
        CatalogPageSize::new(128).unwrap(),
        &limits,
    )
    .unwrap();
    let bootstrap: BootstrapFacadeResponseV1 = serde_json::from_slice(&invoke_ipc(
        &webview,
        "handbook_bootstrap_v1",
        bootstrap_request,
    ))
    .unwrap();
    let bootstrap_value = serde_json::to_value(&bootstrap).unwrap();
    assert_eq!(bootstrap_value["outcome_kind"], "ok");

    let catalog_request = CatalogReadRequestV1::new(
        CatalogKindV1::Capabilities,
        CatalogPageSize::new(128).unwrap(),
        None,
        &limits,
    )
    .unwrap();
    let catalog: CatalogReadResponseV1 = serde_json::from_slice(&invoke_ipc(
        &webview,
        "handbook_catalog_read_v1",
        catalog_request,
    ))
    .unwrap();
    let CatalogReadResponseBodyV1::CapabilityPage(page) = catalog.response() else {
        panic!("Tauri capability read returned a non-capability page");
    };
    let capabilities = page
        .entries()
        .iter()
        .filter(|entry| OPERATIONS.contains(&entry.operation().reference().as_str()))
        .map(|entry| (entry.operation().reference().as_str(), entry))
        .collect::<BTreeMap<_, _>>();
    assert_eq!(capabilities.len(), OPERATIONS.len());
    let manifest = bootstrap_value["response"]["schema_manifest"]
        .as_array()
        .unwrap();
    for operation_ref in OPERATIONS {
        let capability = capabilities.get(operation_ref).unwrap();
        for binding in [
            capability.request_schema(),
            capability.result_schema(),
            capability.outcome_schemas().blocker(),
            capability.outcome_schemas().refusal(),
            capability.outcome_schemas().error(),
        ] {
            assert!(manifest.iter().any(|entry| {
                entry["schema"]["ref"] == binding.reference().as_str()
                    && entry["schema"]["fingerprint"] == binding.fingerprint().as_str()
            }));
        }
    }

    let (contract, definition) = outward_definition(&owner_definition());
    let raw_key = "packet-five-tauri-raw-key-must-not-leak";
    let transition: ContractLifecycleTransitionInputDtoV1 = parse_canonical_json(
        &canonical_json_bytes(
            &json!({
                "approval_class": "definition_author",
                "expected_lifecycle_fingerprint": format!("sha256:{}", "0".repeat(64)),
                "from_state": "draft",
                "reason": {"kind": "ready_for_review"},
                "supporting_evidence": [{
                    "binding": {
                        "fingerprint": format!("sha256:{}", "1".repeat(64)),
                        "ref": "handbook.contract-lifecycle-evidence.packet-five@1.0.0"
                    },
                    "kind": "definition_validation"
                }],
                "to_state": "review_ready"
            }),
            &limits,
        )
        .unwrap(),
        &limits,
    )
    .unwrap();
    let operation_requests = [
        contract_definition_list_operation_request(
            Some("packet-five-tauri-list"),
            ContractDefinitionListRequestV1::first(10).unwrap(),
        )
        .unwrap(),
        contract_definition_read_operation_request(
            Some("packet-five-tauri-read"),
            ContractDefinitionReadRequestV1::new(contract.clone()),
        )
        .unwrap(),
        contract_definition_append_operation_request(
            Some("packet-five-tauri-append"),
            raw_key,
            ContractDefinitionAppendRequestV1::new(definition),
        )
        .unwrap(),
        contract_lifecycle_transition_operation_request(
            Some("packet-five-tauri-transition"),
            "packet-five-tauri-transition-key",
            ContractLifecycleTransitionRequestV1::new(contract, transition),
        )
        .unwrap(),
    ];
    let sdk = handbook_sdk::HandbookSdkV1::open(repo.path());

    for request in operation_requests {
        let invocation = InvocationRequestV1::operation(request, &limits).unwrap();
        let expected = sdk.invoke_tauri(&invocation, &limits).unwrap();
        let actual_wire: serde_json::Value =
            serde_json::from_slice(&invoke_ipc(&webview, "handbook_invoke_v1", invocation))
                .unwrap();
        assert_eq!(actual_wire, serde_json::to_value(&expected).unwrap());
        assert_eq!(actual_wire["response_kind"], "operation");
        assert!(matches!(
            actual_wire["response"]["status"].as_str(),
            Some("ok" | "refused")
        ));
        let bytes = canonical_json_bytes(&actual_wire, &limits).unwrap();
        assert!(!bytes
            .windows(raw_key.len())
            .any(|window| window == raw_key.as_bytes()));
        assert!(!String::from_utf8(bytes)
            .unwrap()
            .contains(repo.path().to_string_lossy().as_ref()));
    }
    assert_tree_excludes(repo.path(), raw_key.as_bytes());
}

fn owner_definition() -> ContractDefinitionV1 {
    ContractDefinitionV1::new(ContractDefinitionInputV1 {
        definition_schema: owner_binding("handbook.contract-definition@1.0.0", 'b'),
        contract_id: ContractIdV1::parse("handbook.contract.packet-five-tauri").unwrap(),
        contract_version: VersionV1::parse("1.0.0").unwrap(),
        contract_kind: ContractKindV1::ClaimSet,
        claims: vec![owner_claim()],
        definition_author_ref: StableIdV1::parse(&format!("credential.sha256-{}", "3".repeat(64)))
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

fn owner_binding(value: &str, hex: char) -> ExactBindingV1 {
    ExactBindingV1::new(
        ExactRefV1::parse(value).unwrap(),
        FingerprintV1::parse(&format!("sha256:{}", hex.to_string().repeat(64))).unwrap(),
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

fn assert_tree_excludes(root: &std::path::Path, forbidden: &[u8]) {
    for entry in std::fs::read_dir(root).unwrap() {
        let entry = entry.unwrap();
        if entry.path().is_dir() {
            assert_tree_excludes(&entry.path(), forbidden);
        } else {
            let bytes = std::fs::read(entry.path()).unwrap();
            assert!(!bytes
                .windows(forbidden.len())
                .any(|window| window == forbidden));
        }
    }
}
