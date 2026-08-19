use handbook_sdk::artifact::{ArtifactSdk, ArtifactSelector, ArtifactValidateRequest};
use handbook_sdk::contract_membrane::{canonical_json_bytes, sha256_fingerprint, ContractLimits};
use std::path::Path;
use std::process::{Command, Output};

const INVOCATION_RESPONSE_SCHEMA_BYTES: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../sdk/schemas/2020-12/handbook.invocation-response/1.0.0.schema.json"
));

#[derive(Clone)]
struct DiscoveredCapability {
    operation_ref: String,
    definition_fingerprint: String,
    entry_fingerprint: String,
    request_schema_ref: String,
    request_schema_fingerprint: String,
}

fn copy_tree(source: &Path, target: &Path) {
    std::fs::create_dir_all(target).unwrap();
    for entry in std::fs::read_dir(source).unwrap() {
        let entry = entry.unwrap();
        let target_path = target.join(entry.file_name());
        if entry.path().is_dir() {
            copy_tree(&entry.path(), &target_path);
        } else {
            std::fs::copy(entry.path(), target_path).unwrap();
        }
    }
}

fn artifact_repo() -> tempfile::TempDir {
    let repo = tempfile::tempdir().unwrap();
    copy_tree(
        &Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../engine/tests/fixtures/hcm_2_3_generic_custom_kind"),
        repo.path(),
    );
    handbook_engine::RepositoryInvocationIdentityServiceV1::new()
        .initialize_for_setup(repo.path())
        .unwrap();
    repo
}

fn run_in(root: &Path, args: &[String]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_handbook"))
        .current_dir(root)
        .args(args)
        .output()
        .unwrap()
}

fn machine_document(output: &Output) -> serde_json::Value {
    assert_eq!(
        output.stdout.iter().filter(|byte| **byte == b'\n').count(),
        1,
        "machine stdout must be one value plus LF: {}",
        String::from_utf8_lossy(&output.stdout)
    );
    assert!(output.stderr.is_empty());
    serde_json::from_slice(&output.stdout).unwrap()
}

fn assert_adapter_failure(output: &Output, expected: &str) {
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    assert_eq!(output.stderr, expected.as_bytes());
}

fn discover_complete(root: &Path, operation_ref: &str) -> DiscoveredCapability {
    let bootstrap = run_in(
        root,
        &[
            "capability",
            "bootstrap",
            "--client-version",
            "1.0.0",
            "--page-size",
            "3",
        ]
        .map(str::to_owned),
    );
    assert!(bootstrap.status.success());
    let bootstrap = machine_document(&bootstrap);
    assert_eq!(bootstrap["outcome_kind"], "ok");
    let mut entries = bootstrap["response"]["first_capability_page"]["entries"]
        .as_array()
        .unwrap()
        .clone();
    let mut cursor = bootstrap["response"]["first_capability_page"]["next_cursor"].clone();
    while !cursor.is_null() {
        let cursor_json = serde_json::to_string(&cursor).unwrap();
        let page = run_in(
            root,
            &[
                "capability".to_owned(),
                "catalog-read".to_owned(),
                "--catalog-kind".to_owned(),
                "capabilities".to_owned(),
                "--page-size".to_owned(),
                "3".to_owned(),
                "--cursor".to_owned(),
                cursor_json,
            ],
        );
        assert!(page.status.success());
        let page = machine_document(&page);
        assert_eq!(page["outcome_kind"], "page");
        entries.extend(page["response"]["entries"].as_array().unwrap().clone());
        cursor = page["response"]["next_cursor"].clone();
    }
    assert_eq!(entries.len(), 23, "complete live capability catalog");
    let entry = entries
        .iter()
        .find(|entry| entry["operation"]["ref"] == operation_ref)
        .unwrap();
    let schema_binding = &entry["request_schema"];
    assert!(bootstrap["response"]["schema_manifest"]
        .as_array()
        .unwrap()
        .iter()
        .any(
            |manifest| manifest["schema"]["ref"] == schema_binding["ref"]
                && manifest["schema"]["fingerprint"] == schema_binding["fingerprint"]
        ));
    DiscoveredCapability {
        operation_ref: operation_ref.to_owned(),
        definition_fingerprint: entry["operation"]["fingerprint"]
            .as_str()
            .unwrap()
            .to_owned(),
        entry_fingerprint: entry["entry_fingerprint"].as_str().unwrap().to_owned(),
        request_schema_ref: schema_binding["ref"].as_str().unwrap().to_owned(),
        request_schema_fingerprint: schema_binding["fingerprint"].as_str().unwrap().to_owned(),
    }
}

fn intake_body(current: &str, raw_key: &str, summary: &str) -> Vec<u8> {
    serde_json::to_vec(&serde_json::json!({
        "acquisition_mode": "express",
        "coverage_submissions": [
            {
                "confidence": "high",
                "contradiction_refs": [],
                "coverage_id": "registry_brief.title",
                "source_kind": "user_declaration",
                "specificity": "exact",
                "state": "supplied",
                "value": "Packet Five Brief"
            },
            {
                "confidence": "high",
                "contradiction_refs": [],
                "coverage_id": "registry_brief.summary",
                "source_kind": "user_declaration",
                "specificity": "concrete",
                "state": "supplied",
                "value": summary
            }
        ],
        "expected_current_artifact_fingerprint": current,
        "idempotency_key": raw_key,
        "instance_id": "registry_brief",
        "kind_ref": "example.artifact-kind.registry-brief@1.0.0"
    }))
    .unwrap()
}

fn invoke_mutation(
    root: &Path,
    capability: &DiscoveredCapability,
    input_path: &Path,
    authorization_path: &Path,
) -> Output {
    run_in(
        root,
        &[
            "capability".to_owned(),
            "invoke".to_owned(),
            "--operation-ref".to_owned(),
            capability.operation_ref.clone(),
            "--definition-fingerprint".to_owned(),
            capability.definition_fingerprint.clone(),
            "--capability-entry-fingerprint".to_owned(),
            capability.entry_fingerprint.clone(),
            "--from-input".to_owned(),
            input_path.to_string_lossy().into_owned(),
            "--mutation-authorization-from".to_owned(),
            authorization_path.to_string_lossy().into_owned(),
        ],
    )
}

fn intake_record_count(root: &Path) -> usize {
    let path = root.join(".handbook/state/artifacts/registry_brief/intake-records");
    std::fs::read_dir(path)
        .map(|entries| entries.count())
        .unwrap_or(0)
}

fn assert_tree_excludes(root: &Path, forbidden: &[u8]) {
    for entry in std::fs::read_dir(root).unwrap() {
        let entry = entry.unwrap();
        if entry.path().is_dir() {
            assert_tree_excludes(&entry.path(), forbidden);
        } else {
            let bytes = std::fs::read(entry.path()).unwrap();
            assert!(
                !bytes
                    .windows(forbidden.len())
                    .any(|window| window == forbidden),
                "runtime raw key reached {}",
                entry.path().display()
            );
        }
    }
}

fn discover(root: &Path, operation_ref: &str) -> DiscoveredCapability {
    let output = run_in(
        root,
        &[
            "capability",
            "catalog-read",
            "--catalog-kind",
            "capabilities",
            "--page-size",
            "128",
        ]
        .map(str::to_owned),
    );
    assert!(output.status.success());
    let document: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    let entry = document["response"]["entries"]
        .as_array()
        .unwrap()
        .iter()
        .find(|entry| entry["operation"]["ref"] == operation_ref)
        .unwrap();
    DiscoveredCapability {
        operation_ref: operation_ref.to_owned(),
        definition_fingerprint: entry["operation"]["fingerprint"]
            .as_str()
            .unwrap()
            .to_owned(),
        entry_fingerprint: entry["entry_fingerprint"].as_str().unwrap().to_owned(),
        request_schema_ref: entry["request_schema"]["ref"].as_str().unwrap().to_owned(),
        request_schema_fingerprint: entry["request_schema"]["fingerprint"]
            .as_str()
            .unwrap()
            .to_owned(),
    }
}

fn write_authorization(
    path: &Path,
    capability: &DiscoveredCapability,
    input_bytes: &[u8],
    entry_fingerprint: &str,
) {
    let document = serde_json::json!({
        "capability_entry_fingerprint": entry_fingerprint,
        "operation": {
            "fingerprint": capability.definition_fingerprint,
            "ref": capability.operation_ref
        },
        "prepared_input_bytes_fingerprint": sha256_fingerprint(input_bytes),
        "request_schema": {
            "fingerprint": capability.request_schema_fingerprint,
            "ref": capability.request_schema_ref
        }
    });
    std::fs::write(
        path,
        canonical_json_bytes(&document, &ContractLimits::packet0()).unwrap(),
    )
    .unwrap();
}

#[test]
fn discovered_mutation_requires_exact_authorization_before_owner_execution() {
    let repo = artifact_repo();
    let capability = discover(repo.path(), "intake.record.append@1.0.0");
    let current = ArtifactSdk::open(repo.path())
        .validate(ArtifactValidateRequest {
            selector: ArtifactSelector::new(
                "example.artifact-kind.registry-brief@1.0.0",
                "registry_brief",
            ),
        })
        .unwrap()
        .artifact_fingerprint;
    let run_dir = tempfile::tempdir().unwrap();
    let input_path = run_dir.path().join("request.json");
    std::fs::write(
        &input_path,
        serde_json::to_vec(&serde_json::json!({
            "acquisition_mode": "express",
            "coverage_submissions": [],
            "expected_current_artifact_fingerprint": current,
            "idempotency_key": format!(
                "packet5-{}-{}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos()
            ),
            "instance_id": "registry_brief",
            "kind_ref": "example.artifact-kind.registry-brief@1.0.0"
        }))
        .unwrap(),
    )
    .unwrap();
    let output = run_in(
        repo.path(),
        &[
            "capability".to_owned(),
            "invoke".to_owned(),
            "--operation-ref".to_owned(),
            capability.operation_ref,
            "--definition-fingerprint".to_owned(),
            capability.definition_fingerprint,
            "--capability-entry-fingerprint".to_owned(),
            capability.entry_fingerprint,
            "--from-input".to_owned(),
            input_path.to_string_lossy().into_owned(),
        ],
    );
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    assert_eq!(
        output.stderr,
        b"handbook capability: mutation authorization required\n"
    );
}

#[test]
fn stale_capability_entry_authorization_is_rejected_before_owner_execution() {
    let repo = artifact_repo();
    let capability = discover(repo.path(), "intake.record.append@1.0.0");
    let current = ArtifactSdk::open(repo.path())
        .validate(ArtifactValidateRequest {
            selector: ArtifactSelector::new(
                "example.artifact-kind.registry-brief@1.0.0",
                "registry_brief",
            ),
        })
        .unwrap()
        .artifact_fingerprint;
    let run_dir = tempfile::tempdir().unwrap();
    let input_path = run_dir.path().join("request.json");
    let input_bytes = serde_json::to_vec(&serde_json::json!({
        "acquisition_mode": "express",
        "coverage_submissions": [],
        "expected_current_artifact_fingerprint": current,
        "idempotency_key": format!(
            "packet5-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ),
        "instance_id": "registry_brief",
        "kind_ref": "example.artifact-kind.registry-brief@1.0.0"
    }))
    .unwrap();
    std::fs::write(&input_path, &input_bytes).unwrap();
    let authorization_path = run_dir.path().join("authorization.json");
    let stale_entry = format!("sha256:{}", "0".repeat(64));
    write_authorization(&authorization_path, &capability, &input_bytes, &stale_entry);

    let output = run_in(
        repo.path(),
        &[
            "capability".to_owned(),
            "invoke".to_owned(),
            "--operation-ref".to_owned(),
            capability.operation_ref,
            "--definition-fingerprint".to_owned(),
            capability.definition_fingerprint,
            "--capability-entry-fingerprint".to_owned(),
            capability.entry_fingerprint,
            "--from-input".to_owned(),
            input_path.to_string_lossy().into_owned(),
            "--mutation-authorization-from".to_owned(),
            authorization_path.to_string_lossy().into_owned(),
        ],
    );
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    assert_eq!(
        output.stderr,
        b"handbook capability: mutation authorization rejected\n"
    );
}

#[test]
fn discovered_mutation_first_execution_and_explicit_replay_preserve_owner_truth() {
    let repo = artifact_repo();
    let capability = discover_complete(repo.path(), "intake.record.append@1.0.0");
    let current = ArtifactSdk::open(repo.path())
        .validate(ArtifactValidateRequest {
            selector: ArtifactSelector::new(
                "example.artifact-kind.registry-brief@1.0.0",
                "registry_brief",
            ),
        })
        .unwrap()
        .artifact_fingerprint;
    let raw_key = format!(
        "packet5-runtime-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    );
    let body = intake_body(&current, &raw_key, "Mutation and replay proof");
    let run_dir = tempfile::tempdir().unwrap();
    let input_path = run_dir.path().join("request.json");
    std::fs::write(&input_path, &body).unwrap();

    let schema = run_in(
        repo.path(),
        &[
            "capability".to_owned(),
            "schema-read".to_owned(),
            "--schema-ref".to_owned(),
            capability.request_schema_ref.clone(),
            "--manifest-fingerprint".to_owned(),
            capability.request_schema_fingerprint.clone(),
        ],
    );
    assert!(schema.status.success());
    assert!(schema.stderr.is_empty());
    let request_schema: serde_json::Value = serde_json::from_slice(&schema.stdout).unwrap();
    let request_validator = jsonschema::draft202012::options()
        .build(&request_schema)
        .unwrap();
    assert!(request_validator.is_valid(&serde_json::from_slice(&body).unwrap()));

    let authorization_path = run_dir.path().join("authorization.json");
    write_authorization(
        &authorization_path,
        &capability,
        &body,
        &capability.entry_fingerprint,
    );
    let authorization_bytes = std::fs::read(&authorization_path).unwrap();
    assert!(!authorization_bytes
        .windows(raw_key.len())
        .any(|bytes| bytes == raw_key.as_bytes()));
    let before = intake_record_count(repo.path());
    let first = invoke_mutation(repo.path(), &capability, &input_path, &authorization_path);
    assert!(
        first.status.success(),
        "stdout={} stderr={}",
        String::from_utf8_lossy(&first.stdout),
        String::from_utf8_lossy(&first.stderr)
    );
    let first_document = machine_document(&first);
    let response_validator = jsonschema::draft202012::options()
        .build(&serde_json::from_slice(INVOCATION_RESPONSE_SCHEMA_BYTES).unwrap())
        .unwrap();
    assert!(response_validator.is_valid(&first_document));
    assert_eq!(first_document["response"]["status"], "ok");
    assert_eq!(
        first_document["response"]["idempotency"]["state"],
        "established"
    );
    assert_eq!(first_document["response"]["idempotency"]["replayed"], false);
    assert_eq!(
        first_document["response"]["receipts"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    assert_eq!(intake_record_count(repo.path()), before + 1);

    let replay = invoke_mutation(repo.path(), &capability, &input_path, &authorization_path);
    assert!(replay.status.success());
    let replay_document = machine_document(&replay);
    assert_eq!(replay_document["response"]["idempotency"]["replayed"], true);
    assert_eq!(
        replay_document["response"]["idempotency"]["original_result_fingerprint"],
        first_document["response"]["result_fingerprint"]
    );
    for field in [
        "result_fingerprint",
        "data",
        "blockers",
        "refusals",
        "errors",
        "diagnostics",
        "next_actions",
        "receipts",
    ] {
        assert_eq!(
            replay_document["response"][field],
            first_document["response"][field]
        );
    }
    assert_eq!(intake_record_count(repo.path()), before + 1);
    assert!(!first
        .stdout
        .windows(raw_key.len())
        .any(|bytes| bytes == raw_key.as_bytes()));
    assert!(!replay
        .stdout
        .windows(raw_key.len())
        .any(|bytes| bytes == raw_key.as_bytes()));
    assert_tree_excludes(repo.path(), raw_key.as_bytes());
}

#[test]
fn same_key_changed_valid_body_preserves_typed_owner_conflict_without_another_write() {
    let repo = artifact_repo();
    let capability = discover(repo.path(), "intake.record.append@1.0.0");
    let current = ArtifactSdk::open(repo.path())
        .validate(ArtifactValidateRequest {
            selector: ArtifactSelector::new(
                "example.artifact-kind.registry-brief@1.0.0",
                "registry_brief",
            ),
        })
        .unwrap()
        .artifact_fingerprint;
    let raw_key = format!(
        "packet5-conflict-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    );
    let run_dir = tempfile::tempdir().unwrap();
    let input_path = run_dir.path().join("request.json");
    let authorization_path = run_dir.path().join("authorization.json");
    let first_body = intake_body(&current, &raw_key, "Original authorized body");
    std::fs::write(&input_path, &first_body).unwrap();
    write_authorization(
        &authorization_path,
        &capability,
        &first_body,
        &capability.entry_fingerprint,
    );
    let before = intake_record_count(repo.path());
    let first = invoke_mutation(repo.path(), &capability, &input_path, &authorization_path);
    assert!(first.status.success());
    assert_eq!(intake_record_count(repo.path()), before + 1);

    let changed_body = intake_body(&current, &raw_key, "Changed authorized body");
    std::fs::write(&input_path, &changed_body).unwrap();
    write_authorization(
        &authorization_path,
        &capability,
        &changed_body,
        &capability.entry_fingerprint,
    );
    let conflict = invoke_mutation(repo.path(), &capability, &input_path, &authorization_path);
    assert_eq!(conflict.status.code(), Some(1));
    let conflict_document = machine_document(&conflict);
    assert_eq!(conflict_document["response"]["status"], "refused");
    assert_eq!(
        conflict_document["response"]["refusals"][0]["details"]["cause"],
        "idempotency_conflict"
    );
    assert_eq!(intake_record_count(repo.path()), before + 1);
    assert!(!conflict
        .stdout
        .windows(raw_key.len())
        .any(|bytes| bytes == raw_key.as_bytes()));
    assert_tree_excludes(repo.path(), raw_key.as_bytes());
}

#[test]
fn schema_invalid_authorized_body_is_rejected_before_owner_execution() {
    let repo = artifact_repo();
    let capability = discover(repo.path(), "intake.record.append@1.0.0");
    let run_dir = tempfile::tempdir().unwrap();
    let input_path = run_dir.path().join("invalid.json");
    let invalid_body = br#"{"private_raw_key":"runtime-only-invalid"}"#;
    std::fs::write(&input_path, invalid_body).unwrap();
    let authorization_path = run_dir.path().join("authorization.json");
    write_authorization(
        &authorization_path,
        &capability,
        invalid_body,
        &capability.entry_fingerprint,
    );
    let before = intake_record_count(repo.path());
    let output = invoke_mutation(repo.path(), &capability, &input_path, &authorization_path);
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    assert_eq!(output.stderr, b"handbook capability: input rejected\n");
    assert_eq!(intake_record_count(repo.path()), before);
    assert!(!String::from_utf8_lossy(&output.stderr).contains("runtime-only-invalid"));
}

#[test]
fn changed_input_stale_identity_schema_and_unavailable_transport_all_fail_before_write() {
    let repo = artifact_repo();
    let mutation = discover(repo.path(), "intake.record.append@1.0.0");
    let current = ArtifactSdk::open(repo.path())
        .validate(ArtifactValidateRequest {
            selector: ArtifactSelector::new(
                "example.artifact-kind.registry-brief@1.0.0",
                "registry_brief",
            ),
        })
        .unwrap()
        .artifact_fingerprint;
    let raw_key = format!(
        "packet5-gates-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    );
    let body = intake_body(&current, &raw_key, "Authorized gate body");
    let run_dir = tempfile::tempdir().unwrap();
    let input_path = run_dir.path().join("request.json");
    let authorization_path = run_dir.path().join("authorization.json");
    std::fs::write(&input_path, &body).unwrap();
    write_authorization(
        &authorization_path,
        &mutation,
        &body,
        &mutation.entry_fingerprint,
    );
    let before = intake_record_count(repo.path());

    let changed_body = intake_body(&current, &raw_key, "Changed after authorization");
    std::fs::write(&input_path, &changed_body).unwrap();
    assert_adapter_failure(
        &invoke_mutation(repo.path(), &mutation, &input_path, &authorization_path),
        "handbook capability: mutation authorization rejected\n",
    );
    std::fs::write(&input_path, &body).unwrap();

    let stale = format!("sha256:{}", "0".repeat(64));
    for field in ["operation", "request_schema"] {
        let mut authorization: serde_json::Value =
            serde_json::from_slice(&std::fs::read(&authorization_path).unwrap()).unwrap();
        authorization[field]["fingerprint"] = serde_json::Value::String(stale.clone());
        std::fs::write(
            &authorization_path,
            canonical_json_bytes(&authorization, &ContractLimits::packet0()).unwrap(),
        )
        .unwrap();
        assert_adapter_failure(
            &invoke_mutation(repo.path(), &mutation, &input_path, &authorization_path),
            "handbook capability: mutation authorization rejected\n",
        );
        write_authorization(
            &authorization_path,
            &mutation,
            &body,
            &mutation.entry_fingerprint,
        );
    }
    let mut changed_operation_authorization: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&authorization_path).unwrap()).unwrap();
    changed_operation_authorization["operation"]["ref"] =
        serde_json::Value::String("artifact.candidate.append@1.0.0".to_owned());
    std::fs::write(
        &authorization_path,
        canonical_json_bytes(&changed_operation_authorization, &ContractLimits::packet0()).unwrap(),
    )
    .unwrap();
    assert_adapter_failure(
        &invoke_mutation(repo.path(), &mutation, &input_path, &authorization_path),
        "handbook capability: mutation authorization rejected\n",
    );
    write_authorization(
        &authorization_path,
        &mutation,
        &body,
        &mutation.entry_fingerprint,
    );

    for stale_selection in [
        DiscoveredCapability {
            definition_fingerprint: stale.clone(),
            ..mutation.clone()
        },
        DiscoveredCapability {
            entry_fingerprint: stale.clone(),
            ..mutation.clone()
        },
    ] {
        assert_adapter_failure(
            &invoke_mutation(
                repo.path(),
                &stale_selection,
                &input_path,
                &authorization_path,
            ),
            "handbook capability: operation selection stale\n",
        );
    }

    let posture = discover(repo.path(), "posture.transition.apply@1.0.0");
    std::fs::write(&input_path, b"{}").unwrap();
    assert_adapter_failure(
        &invoke_mutation(repo.path(), &posture, &input_path, &authorization_path),
        "handbook capability: transport unavailable\n",
    );
    assert_eq!(intake_record_count(repo.path()), before);
    assert_tree_excludes(repo.path(), raw_key.as_bytes());
}

#[test]
fn installed_skill_describes_one_generic_exact_authorized_invoke_without_retry_or_raw_key() {
    let skill = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../install/handbook-home/SKILL.md.tmpl"
    ));
    for required in [
        "prepared_input_bytes_fingerprint",
        "--mutation-authorization-from",
        "python3",
        "chmod 600",
        "Never retry automatically",
    ] {
        assert!(skill.contains(required), "missing {required}");
    }
    assert!(!skill.contains("intake.record.append@1.0.0"));
    assert!(!skill.contains("idempotency_key\":"));

    let adapter = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/capability.rs"));
    assert_eq!(
        adapter
            .matches("let response = match consumer.invoke(")
            .count(),
        1,
        "production adapter must contain one owner-dispatch attempt"
    );
    for forbidden in ["artifact intake", "--legacy-json", "std::thread::sleep"] {
        assert!(
            !adapter.contains(forbidden),
            "fallback or retry marker: {forbidden}"
        );
    }
}
