use handbook_contracts::{
    lifecycle_policy_binding_v1, ApplicabilityRuleV1, ContractClaimInputV1, ContractClaimV1,
    ContractDefinitionInputV1, ContractDefinitionV1, ContractIdV1, ContractKindV1,
    EvidenceCardinalityV1, EvidenceCaseSelectionV1, EvidenceRequirementV1, ExactBindingV1,
    ExactRefV1, FingerprintV1, GateEffectV1, MatcherBindingV1, ResolutionMinimumV1,
    SelectorCriterionV1, SelectorV1, StableIdV1, VersionV1,
};
use handbook_sdk::contract_membrane::{
    canonical_json_bytes, parse_canonical_json, sha256_fingerprint, ContractDefinitionDtoV1,
    ContractLimits, ExactBinding, ExactRef, Fingerprint,
};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use std::process::{Command, Output};

const OPERATIONS: [(&str, &str, &str); 4] = [
    ("contract.definition.list@1.0.0", "read_only", "safe"),
    ("contract.definition.read@1.0.0", "read_only", "safe"),
    (
        "contract.definition.append@1.0.0",
        "append_only",
        "idempotency_key_required",
    ),
    (
        "contract.lifecycle.transition@1.0.0",
        "compare_and_write",
        "compare_and_write_required",
    ),
];

fn run(root: &Path, args: &[String]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_handbook"))
        .current_dir(root)
        .args(args)
        .output()
        .unwrap()
}

fn strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

fn machine_document(output: &Output) -> Value {
    assert_eq!(
        output.stdout.iter().filter(|byte| **byte == b'\n').count(),
        1,
        "machine stdout must contain exactly one JSON value plus LF: {}",
        String::from_utf8_lossy(&output.stdout)
    );
    assert!(output.stderr.is_empty());
    serde_json::from_slice(&output.stdout).unwrap()
}

fn capabilities(root: &Path) -> BTreeMap<String, Value> {
    let output = run(
        root,
        &strings(&[
            "capability",
            "catalog-read",
            "--catalog-kind",
            "capabilities",
            "--page-size",
            "128",
        ]),
    );
    assert!(output.status.success());
    let document = machine_document(&output);
    assert_eq!(document["outcome_kind"], "page");
    assert_eq!(document["response"]["root"]["total_entries"], 28);
    document["response"]["entries"]
        .as_array()
        .unwrap()
        .iter()
        .map(|entry| {
            (
                entry["operation"]["ref"].as_str().unwrap().to_owned(),
                entry.clone(),
            )
        })
        .collect()
}

fn schema_bytes(root: &Path, binding: &Value) -> Vec<u8> {
    let output = run(
        root,
        &strings(&[
            "capability",
            "schema-read",
            "--schema-ref",
            binding["ref"].as_str().unwrap(),
            "--manifest-fingerprint",
            binding["fingerprint"].as_str().unwrap(),
        ]),
    );
    assert!(output.status.success());
    assert!(output.stderr.is_empty());
    assert_eq!(
        sha256_fingerprint(&output.stdout).as_str(),
        binding["fingerprint"].as_str().unwrap()
    );
    output.stdout
}

fn invoke(root: &Path, capability: &Value, body: &[u8], mutation_authorization: bool) -> Output {
    let run_dir = tempfile::tempdir().unwrap();
    let body_path = run_dir.path().join("request.json");
    std::fs::write(&body_path, body).unwrap();
    let mut args = vec![
        "capability".to_owned(),
        "invoke".to_owned(),
        "--operation-ref".to_owned(),
        capability["operation"]["ref"].as_str().unwrap().to_owned(),
        "--definition-fingerprint".to_owned(),
        capability["operation"]["fingerprint"]
            .as_str()
            .unwrap()
            .to_owned(),
        "--capability-entry-fingerprint".to_owned(),
        capability["entry_fingerprint"].as_str().unwrap().to_owned(),
        "--from-input".to_owned(),
        body_path.to_string_lossy().into_owned(),
    ];
    if mutation_authorization {
        let authorization_path = run_dir.path().join("authorization.json");
        let authorization = json!({
            "capability_entry_fingerprint": capability["entry_fingerprint"],
            "operation": capability["operation"],
            "prepared_input_bytes_fingerprint": sha256_fingerprint(body),
            "request_schema": capability["request_schema"],
        });
        std::fs::write(
            &authorization_path,
            canonical_json_bytes(&authorization, &ContractLimits::packet0()).unwrap(),
        )
        .unwrap();
        args.extend([
            "--mutation-authorization-from".to_owned(),
            authorization_path.to_string_lossy().into_owned(),
        ]);
    }
    run(root, &args)
}

#[test]
fn generic_cli_discovers_schemas_and_invokes_all_four_contract_operations() {
    let repo = tempfile::tempdir().unwrap();
    let help = run(repo.path(), &strings(&["capability", "--help"]));
    assert!(help.status.success());
    let actions = String::from_utf8(help.stdout)
        .unwrap()
        .lines()
        .filter_map(|line| {
            let action = line.split_whitespace().next()?;
            ["bootstrap", "catalog-read", "schema-read", "invoke"]
                .contains(&action)
                .then_some(action.to_owned())
        })
        .collect::<BTreeSet<_>>();
    assert_eq!(
        actions,
        ["bootstrap", "catalog-read", "schema-read", "invoke"]
            .map(str::to_owned)
            .into_iter()
            .collect()
    );

    let capabilities = capabilities(repo.path());
    let (contract, definition) = outward_definition(&owner_definition("1.0.0"));
    let raw_key = "packet-five-cli-raw-key-must-not-leak";
    let bodies = BTreeMap::from([
        (
            "contract.definition.list@1.0.0",
            canonical_json_bytes(
                &json!({"cursor": null, "limit": 10, "root": null}),
                &ContractLimits::packet0(),
            )
            .unwrap(),
        ),
        (
            "contract.definition.read@1.0.0",
            canonical_json_bytes(
                &json!({"contract": contract.clone()}),
                &ContractLimits::packet0(),
            )
            .unwrap(),
        ),
        (
            "contract.definition.append@1.0.0",
            canonical_json_bytes(
                &json!({"definition": definition, "idempotency_key": raw_key}),
                &ContractLimits::packet0(),
            )
            .unwrap(),
        ),
        (
            "contract.lifecycle.transition@1.0.0",
            canonical_json_bytes(
                &json!({
                    "contract": contract,
                    "idempotency_key": "packet-five-cli-transition-key",
                    "transition": {
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
                    }
                }),
                &ContractLimits::packet0(),
            )
            .unwrap(),
        ),
    ]);

    for (operation_ref, class, idempotency_kind) in OPERATIONS {
        let capability = capabilities.get(operation_ref).unwrap();
        assert_eq!(capability["class"], class);
        assert_eq!(capability["idempotency"]["kind"], idempotency_kind);
        assert_eq!(capability["owner"]["owner_crate"], "handbook-contracts");
        assert_eq!(capability["owner"]["owner_version"], "0.1.0");
        assert_eq!(
            capability["owner"]["source"]["ref"],
            "handbook.owner.contracts@0.1.0"
        );
        assert_eq!(
            capability["current_transports"],
            json!(["cli_json", "rust_sdk", "tauri"])
        );
        for binding in [
            &capability["request_schema"],
            &capability["result_schema"],
            &capability["outcome_schemas"]["blocker"],
            &capability["outcome_schemas"]["refusal"],
            &capability["outcome_schemas"]["error"],
        ] {
            let schema: Value =
                serde_json::from_slice(&schema_bytes(repo.path(), binding)).unwrap();
            jsonschema::draft202012::options()
                .build(&schema)
                .expect("manifest-bound schema compiles");
        }

        let output = invoke(
            repo.path(),
            capability,
            bodies.get(operation_ref).unwrap(),
            class != "read_only",
        );
        let document = machine_document(&output);
        assert_eq!(document["response_kind"], "operation");
        assert_eq!(document["response"]["operation"], capability["operation"]);
        match operation_ref {
            "contract.definition.list@1.0.0" | "contract.definition.read@1.0.0" => {
                assert!(output.status.success());
                assert_eq!(document["response"]["status"], "ok");
                assert!(document["response"]["receipts"]
                    .as_array()
                    .unwrap()
                    .is_empty());
            }
            "contract.definition.append@1.0.0" => {
                assert_eq!(output.status.code(), Some(1));
                assert_eq!(document["response"]["status"], "refused");
                assert_eq!(
                    document["response"]["refusals"][0]["details"]["cause"],
                    "authority_absent"
                );
                assert_eq!(
                    document["response"]["idempotency"]["state"],
                    "not_established"
                );
                assert!(document["response"]["receipts"]
                    .as_array()
                    .unwrap()
                    .is_empty());
            }
            "contract.lifecycle.transition@1.0.0" => {
                assert_eq!(output.status.code(), Some(1));
                assert_eq!(document["response"]["status"], "refused");
                assert_eq!(
                    document["response"]["refusals"][0]["details"]["cause"],
                    "record_substituted"
                );
                assert!(document["response"]["receipts"]
                    .as_array()
                    .unwrap()
                    .is_empty());
            }
            _ => unreachable!(),
        }
        assert!(!output
            .stdout
            .windows(raw_key.len())
            .any(|window| window == raw_key.as_bytes()));
        assert!(!String::from_utf8(output.stdout)
            .unwrap()
            .contains(repo.path().to_string_lossy().as_ref()));
    }
    assert_tree_excludes(repo.path(), raw_key.as_bytes());
}

#[test]
fn contract_consumers_add_no_parser_command_skill_prompt_or_semantic_branch() {
    let cli_adapter = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/capability.rs"));
    let cli_parser = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/main.rs"));
    let tauri_facade = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../tauri/src/lib.rs"));
    let installed_skill = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../install/handbook-home/SKILL.md.tmpl"
    ));
    let installed_prompt = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../install/handbook-home/agents/openai.yaml"
    ));

    for source in [
        cli_adapter,
        cli_parser,
        tauri_facade,
        installed_skill,
        installed_prompt,
    ] {
        for (operation_ref, _, _) in OPERATIONS {
            assert!(!source.contains(operation_ref), "hardcoded {operation_ref}");
            assert!(!source.contains(operation_ref.trim_end_matches("@1.0.0")));
        }
    }
    assert_eq!(cli_adapter.matches("CapabilityCommand::").count(), 4);
    assert_eq!(
        cli_adapter
            .matches("let response = match consumer.invoke(")
            .count(),
        1
    );
    assert_eq!(
        tauri_facade.matches("#[tauri::command]").count(),
        3,
        "contract operations must not add Tauri commands"
    );
    assert_eq!(installed_skill.matches("capability invoke").count(), 1);
    for semantic_decision in [
        "authority_absent",
        "authenticator_unavailable",
        "record_substituted",
        "lifecycle_refused",
        "idempotency_conflict",
    ] {
        assert!(!installed_skill.contains(semantic_decision));
        assert!(!installed_prompt.contains(semantic_decision));
    }
}

fn owner_definition(version: &str) -> ContractDefinitionV1 {
    ContractDefinitionV1::new(ContractDefinitionInputV1 {
        definition_schema: owner_binding("handbook.contract-definition@1.0.0", 'b'),
        contract_id: ContractIdV1::parse("handbook.contract.packet-five-cli").unwrap(),
        contract_version: VersionV1::parse(version).unwrap(),
        contract_kind: ContractKindV1::ClaimSet,
        claims: vec![owner_claim()],
        definition_author_ref: StableIdV1::parse(&format!("credential.sha256-{}", "3".repeat(64)))
            .unwrap(),
        definition_authority: owner_binding(AUTHOR_AUTHORITY_REF, 'a'),
        lifecycle_policy: lifecycle_policy_binding_v1().unwrap(),
        matcher_bindings: vec![MatcherBindingV1::exact_fields_v1()],
        gate_policy: owner_binding("handbook.contract-gate-policy.closed-v1@1.0.0", 'c'),
        extensions_policy: owner_binding("handbook.contract-extensions.closed-v1@1.0.0", 'd'),
    })
    .unwrap()
}

const AUTHOR_AUTHORITY_REF: &str = "handbook.authority.contract-definition-author@1.0.0";

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

fn assert_tree_excludes(root: &Path, forbidden: &[u8]) {
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
