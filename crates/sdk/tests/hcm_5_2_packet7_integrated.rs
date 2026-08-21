use handbook_sdk::contract_membrane::{
    canonical_json_bytes, operation_inventory, CapabilityConsumer, CapabilityConsumerError,
    CatalogKindV1, CatalogPageSize, CatalogReadResponseBodyV1, ContractLimits,
    InvocationResponseKind, OperationTransport, ProductionWitness, RequestId, TerminalStatus,
    UtcTimestamp,
};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

const OPERATIONS: [&str; 5] = [
    "contract.evidence.list@1.0.0",
    "contract.evidence.read@1.0.0",
    "contract.evidence.append@1.0.0",
    "contract.verdict.evaluate@1.0.0",
    "contract.gate.evaluate@1.0.0",
];

fn binding(reference: &str, hex: char) -> Value {
    json!({
        "ref": reference,
        "fingerprint": format!("sha256:{}", hex.to_string().repeat(64)),
    })
}

#[test]
fn final_generic_sdk_proof_preserves_five_operations_and_exact_counts() {
    let repo = tempfile::tempdir().unwrap();
    let witness =
        ProductionWitness::from_current(UtcTimestamp::parse("9999-12-31T23:59:59Z").unwrap())
            .unwrap();
    assert_eq!(operation_inventory().len(), 62);
    assert_eq!(witness.capabilities().len(), 31);
    assert_eq!(witness.schema_manifest().len(), 170);
    assert_eq!(
        witness
            .schema_manifest()
            .iter()
            .map(|entry| entry.schema().reference().as_str())
            .collect::<BTreeSet<_>>()
            .len(),
        170
    );

    let consumer = CapabilityConsumer::open(repo.path()).unwrap();
    let catalog = consumer
        .catalog_read(
            CatalogKindV1::Capabilities,
            CatalogPageSize::new(128).unwrap(),
            None,
        )
        .unwrap();
    let CatalogReadResponseBodyV1::CapabilityPage(page) = catalog.response() else {
        panic!("capability catalog returned a non-capability page");
    };
    assert_eq!(page.root().total_entries().get(), 31);
    let capabilities = page
        .entries()
        .iter()
        .filter(|entry| OPERATIONS.contains(&entry.operation().reference().as_str()))
        .map(|entry| (entry.operation().reference().as_str(), entry))
        .collect::<BTreeMap<_, _>>();
    assert_eq!(capabilities.len(), OPERATIONS.len());

    let run = json!({
        "binding": binding(
            "handbook.contract.packet-seven@1.0.0#evaluation/aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            'a'
        ),
        "document": {}
    });
    let raw_key = "packet-seven-sdk-raw-key-must-not-leak";
    let bodies = BTreeMap::from([
        (
            "contract.evidence.list@1.0.0",
            json!({"cursor": null, "limit": 10, "root": null}),
        ),
        (
            "contract.evidence.read@1.0.0",
            json!({"evidence": binding(
                "handbook.contract.packet-seven@1.0.0#evidence/aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                'a'
            )}),
        ),
        (
            "contract.evidence.append@1.0.0",
            json!({
                "candidate": binding("handbook.evidence-candidate.packet-seven@1.0.0", 'b'),
                "idempotency_key": raw_key,
            }),
        ),
        (
            "contract.verdict.evaluate@1.0.0",
            json!({"evaluation_run": run.clone()}),
        ),
        (
            "contract.gate.evaluate@1.0.0",
            json!({"evaluation_run": run}),
        ),
    ]);
    let before = repository_bytes(repo.path());
    let mut request_number = 0;

    for operation_ref in OPERATIONS {
        let capability = capabilities[operation_ref];
        assert_eq!(
            capability.current_transports(),
            &[
                OperationTransport::CliJson,
                OperationTransport::RustSdk,
                OperationTransport::Tauri,
            ]
        );
        for transport in [
            OperationTransport::RustSdk,
            OperationTransport::CliJson,
            OperationTransport::Tauri,
        ] {
            request_number += 1;
            let body =
                canonical_json_bytes(&bodies[operation_ref], &ContractLimits::packet0()).unwrap();
            let response = consumer
                .invoke(
                    capability.operation(),
                    capability.entry_fingerprint(),
                    transport,
                    Some(RequestId::parse(&format!("packet-seven-sdk-{request_number}")).unwrap()),
                    &body,
                )
                .unwrap();
            assert_eq!(response.response_kind(), InvocationResponseKind::Operation);
            let value = serde_json::to_value(&response).unwrap();
            match operation_ref {
                "contract.evidence.list@1.0.0" => {
                    assert_eq!(response.terminal_status(), TerminalStatus::Ok);
                    assert_eq!(value["response"]["data"]["entries"], json!([]));
                }
                "contract.evidence.read@1.0.0" => {
                    assert_eq!(response.terminal_status(), TerminalStatus::Ok);
                    assert_eq!(value["response"]["data"]["outcome"], "absent");
                }
                "contract.evidence.append@1.0.0" => {
                    assert_eq!(response.terminal_status(), TerminalStatus::Error);
                    assert_eq!(
                        value["response"]["errors"][0]["details"]["cause"],
                        "candidate_source_unavailable"
                    );
                }
                "contract.verdict.evaluate@1.0.0" | "contract.gate.evaluate@1.0.0" => {
                    assert_eq!(response.terminal_status(), TerminalStatus::Error);
                    assert_eq!(
                        value["response"]["errors"][0]["details"]["cause"],
                        "repository_unavailable"
                    );
                }
                _ => unreachable!(),
            }
            let bytes = canonical_json_bytes(&response, &ContractLimits::packet0()).unwrap();
            assert!(!bytes
                .windows(raw_key.len())
                .any(|window| window == raw_key.as_bytes()));
            assert!(!String::from_utf8(bytes)
                .unwrap()
                .contains(repo.path().to_string_lossy().as_ref()));
        }
    }
    assert_eq!(repository_bytes(repo.path()), before);
}

#[test]
fn structural_refusal_precedes_owner_execution_and_writes_nothing() {
    let repo = tempfile::tempdir().unwrap();
    let consumer = CapabilityConsumer::open(repo.path()).unwrap();
    let catalog = consumer
        .catalog_read(
            CatalogKindV1::Capabilities,
            CatalogPageSize::new(128).unwrap(),
            None,
        )
        .unwrap();
    let CatalogReadResponseBodyV1::CapabilityPage(page) = catalog.response() else {
        panic!("capability catalog returned a non-capability page");
    };
    let capability = page
        .entries()
        .iter()
        .find(|entry| entry.operation().reference().as_str() == "contract.evidence.append@1.0.0")
        .unwrap();
    let raw_key = "packet-seven-invalid-raw-key-must-not-leak";
    let before = repository_bytes(repo.path());
    let error = consumer
        .invoke(
            capability.operation(),
            capability.entry_fingerprint(),
            OperationTransport::RustSdk,
            None,
            format!(r#"{{"private_raw_key":"{raw_key}"}}"#).as_bytes(),
        )
        .unwrap_err();
    assert_eq!(error, CapabilityConsumerError::InvalidInput);
    assert_eq!(repository_bytes(repo.path()), before);
}

fn repository_bytes(root: &Path) -> Vec<u8> {
    fn visit(path: &Path, output: &mut Vec<u8>) {
        if path.is_dir() {
            for entry in std::fs::read_dir(path).unwrap() {
                visit(&entry.unwrap().path(), output);
            }
        } else {
            output.extend(std::fs::read(path).unwrap());
        }
    }
    let mut output = Vec::new();
    visit(root, &mut output);
    output
}
