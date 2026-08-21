use handbook_dock::DockManifestRegistryV1;
use handbook_sdk::contract_membrane::{
    canonical_json_bytes, operation_inventory, CapabilityConsumer, ContractLimits,
    ContractRecordBindingDtoV1, DockManifestListRequestV1, DockManifestReadRequestV1, Fingerprint,
    OperationTransport, ProductionWitness, RequestId, UtcTimestamp, DOCK_MANIFEST_LIST_REF,
    DOCK_MANIFEST_READ_REF, DOCK_RUN_REF,
};
use serde_json::{json, Value};
use std::collections::BTreeSet;

const VALIDATION_INPUT_REF: &str = "handbook.dock-json-schema-validation-input@1.0.0";
const FACT_REF: &str = "handbook.fact.schema-conformance@1.0.0";
const PRODUCTION_DOCK_REF: &str = "handbook.dock.json-schema@1.0.0";
const PRODUCTION_DOCK_FINGERPRINT: &str =
    "sha256:85ca3e4482b2316c654ee117f1ee2e496647734a8e187e36ad5bcae09d6cd893";

#[test]
fn packet6_closes_exact_public_truth_through_the_generic_rust_consumer() {
    let repository = tempfile::tempdir().unwrap();
    let witness =
        ProductionWitness::from_current(UtcTimestamp::parse("9999-12-31T23:59:59Z").unwrap())
            .unwrap();
    assert_eq!(operation_inventory().len(), 62);
    assert_eq!(witness.capabilities().len(), 31);
    assert_eq!(witness.schema_manifest().len(), 170);
    assert_eq!(DockManifestRegistryV1::production().production_len(), 1);

    let schema_refs = witness
        .schema_manifest()
        .iter()
        .map(|entry| entry.schema().reference().as_str())
        .collect::<BTreeSet<_>>();
    assert_eq!(schema_refs.len(), 170);
    assert!(schema_refs.contains(VALIDATION_INPUT_REF));
    assert!(schema_refs.contains(FACT_REF));

    let consumer = CapabilityConsumer::open(repository.path()).unwrap();
    for schema_ref in [VALIDATION_INPUT_REF, FACT_REF] {
        let entry = witness
            .schema_manifest()
            .iter()
            .find(|entry| entry.schema().reference().as_str() == schema_ref)
            .unwrap();
        let bytes = consumer.schema_bytes(entry.schema()).unwrap();
        assert_eq!(bytes.len() as u64, entry.byte_length().get());
        assert_eq!(
            handbook_sdk::contract_membrane::sha256_fingerprint(bytes),
            *entry.schema().fingerprint()
        );
    }

    let invoke = |operation_ref: &str, body: &[u8], request_id: &str| {
        let capability = witness
            .capabilities()
            .iter()
            .find(|entry| entry.operation().reference().as_str() == operation_ref)
            .unwrap();
        assert_eq!(
            capability.current_transports(),
            [
                OperationTransport::CliJson,
                OperationTransport::RustSdk,
                OperationTransport::Tauri,
            ]
        );
        consumer
            .invoke(
                capability.operation(),
                capability.entry_fingerprint(),
                OperationTransport::RustSdk,
                Some(RequestId::parse(request_id).unwrap()),
                body,
            )
            .unwrap()
    };
    let limits = ContractLimits::packet0();
    let listed = invoke(
        DOCK_MANIFEST_LIST_REF,
        &canonical_json_bytes(&DockManifestListRequestV1::first(10).unwrap(), &limits).unwrap(),
        "packet6-sdk-list",
    );
    let listed = serde_json::to_value(listed).unwrap();
    assert_eq!(
        listed["response"]["data"]["entries"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    assert_eq!(
        listed["response"]["data"]["entries"][0]["manifest"]["ref"],
        PRODUCTION_DOCK_REF
    );

    let requested = ContractRecordBindingDtoV1::new(
        PRODUCTION_DOCK_REF,
        Fingerprint::parse(PRODUCTION_DOCK_FINGERPRINT).unwrap(),
    )
    .unwrap();
    let read = invoke(
        DOCK_MANIFEST_READ_REF,
        &canonical_json_bytes(&DockManifestReadRequestV1::new(requested), &limits).unwrap(),
        "packet6-sdk-read",
    );
    let read: Value = serde_json::to_value(read).unwrap();
    assert_eq!(read["response"]["data"]["outcome"], "found");
    assert_eq!(
        read["response"]["data"]["manifest"]["document"]["dock_ref"],
        PRODUCTION_DOCK_REF
    );

    let raw_key = "packet-six-sdk-production-key-must-not-leak";
    let binding = |reference: &str, hex: char| {
        json!({
            "ref": reference,
            "fingerprint": format!("sha256:{}", hex.to_string().repeat(64)),
        })
    };
    let run_body = canonical_json_bytes(
        &json!({
            "manifest": {
                "ref": PRODUCTION_DOCK_REF,
                "fingerprint": PRODUCTION_DOCK_FINGERPRINT,
            },
            "evaluation": {
                "contract": binding("handbook.contract.absent@1.0.0", 'b'),
                "expected_lifecycle_fingerprint": format!("sha256:{}", "c".repeat(64)),
                "subject": {},
                "cases": [],
                "requested_resolution": {},
                "matcher": {},
                "freshness_policies": [],
                "stability_policies": [],
                "gate_policy": binding("handbook.contract-gate-policy.absent@1.0.0", 'd'),
                "local_closeout_policy": binding(
                    "handbook.contract-local-closeout-policy.absent@1.0.0",
                    'e',
                ),
                "nonce": "packet-six-sdk-run",
            },
            "requested_evidence_kinds": [],
            "inputs": [],
            "limits": {
                "request_bytes": 4096,
                "result_bytes": 4096,
                "stderr_bytes": 1024,
                "diagnostic_count": 4,
                "diagnostic_bytes": 1024,
                "candidate_count": 4,
                "candidate_bytes": 4096,
                "input_count": 4,
                "wall_timeout_ms": 1000,
                "cancellation_grace_ms": 100,
            },
            "idempotency_key": raw_key,
        }),
        &limits,
    )
    .unwrap();
    let run = invoke(DOCK_RUN_REF, &run_body, "packet-six-sdk-production-run");
    let run = serde_json::to_value(run).unwrap();
    assert_eq!(run["response"]["status"], "refused");
    assert_eq!(
        run["response"]["refusals"][0]["details"]["cause"],
        "malformed"
    );
    assert_eq!(run["response"]["idempotency"]["state"], "not_established");
    assert!(run["response"]["receipts"].as_array().unwrap().is_empty());
    let run_bytes = canonical_json_bytes(&run, &limits).unwrap();
    assert!(!run_bytes
        .windows(raw_key.len())
        .any(|window| window == raw_key.as_bytes()));

    let fixed_use_cases = [
        "handbook.use-case.repository-setup@1.0.0",
        "handbook.use-case.repository-doctor@1.0.0",
        "handbook.use-case.author-charter@1.0.0",
        "handbook.use-case.author-project-context@1.0.0",
        "handbook.use-case.approver-administration@1.0.0",
        "handbook.use-case.pipeline-route-resolve-and-persist@1.0.0",
    ];
    let use_case_source = include_str!("../src/contract_membrane/facade/request.rs");
    assert_eq!(fixed_use_cases.len(), 6);
    for use_case in fixed_use_cases {
        assert!(use_case_source.contains(use_case));
        assert!(!operation_inventory()
            .iter()
            .any(|row| row.operation_ref().as_str() == use_case));
        assert!(!witness
            .capabilities()
            .iter()
            .any(|entry| entry.operation().reference().as_str() == use_case));
    }
}

#[test]
fn packet6_adds_no_generic_consumer_route_or_validator_semantic_branch() {
    let cli = include_str!("../../cli/src/capability.rs");
    let cli_parser = include_str!("../../cli/src/main.rs");
    let tauri = include_str!("../../tauri/src/lib.rs");
    let installed_skill = include_str!("../../../install/handbook-home/SKILL.md.tmpl");

    for source in [cli, cli_parser, tauri, installed_skill] {
        for forbidden in [
            PRODUCTION_DOCK_REF,
            "dock.manifest.list@1.0.0",
            "dock.manifest.read@1.0.0",
            "dock.run@1.0.0",
            "schema-conformance",
            "manifest_unavailable",
            "launch_refused",
            "process_failed",
            "cleanup_uncertain",
        ] {
            assert!(!source.contains(forbidden), "{forbidden}");
        }
    }
    assert_eq!(cli.matches("CapabilityCommand::").count(), 4);
    assert_eq!(
        cli.matches("let response = match consumer.invoke(").count(),
        1
    );
    assert_eq!(tauri.matches("#[tauri::command]").count(), 3);
    assert_eq!(installed_skill.matches("capability invoke").count(), 1);
}
