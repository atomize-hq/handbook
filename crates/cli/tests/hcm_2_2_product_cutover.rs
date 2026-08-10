use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::process::{Command, Output};

const CHARTER_YAML: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/specs/handbook-contract-membrane/slices/HCM-2.2/contracts/canonical-charter-boundary-v1.1.yaml"
));

const COVERAGE_IDS: [&str; 16] = [
    "project_shape.definition",
    "delivery.constraints",
    "delivery.default_implications",
    "operational_reality.production_state",
    "risk.domains",
    "engineering_posture.baseline",
    "policy.authority_and_revision",
    "governance.decision_authority",
    "governance.required_approvals",
    "governance.exception_policy",
    "engineering_posture.dimensions",
    "engineering_posture.red_lines",
    "governance.review_triggers",
    "governance.reassessment_triggers",
    "debt.register",
    "decisions.records",
];

fn binary_in(dir: &Path) -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_handbook"));
    command.current_dir(dir);
    command
}

fn run(dir: &Path, args: &[&str]) -> Output {
    binary_in(dir)
        .args(args)
        .output()
        .unwrap_or_else(|error| panic!("run `{}`: {error}", args.join(" ")))
}

fn stdout(output: &Output) -> String {
    String::from_utf8(output.stdout.clone()).expect("stdout UTF-8")
}

fn snapshot_files(root: &Path) -> BTreeMap<String, Vec<u8>> {
    fn visit(root: &Path, current: &Path, files: &mut BTreeMap<String, Vec<u8>>) {
        if !current.exists() {
            return;
        }
        let mut entries = fs::read_dir(current)
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .collect::<Vec<_>>();
        entries.sort();
        for path in entries {
            if path.is_dir() {
                visit(root, &path, files);
            } else if path.is_file() {
                files.insert(
                    path.strip_prefix(root)
                        .unwrap()
                        .to_string_lossy()
                        .replace('\\', "/"),
                    fs::read(path).unwrap(),
                );
            }
        }
    }

    let mut files = BTreeMap::new();
    visit(root, root, &mut files);
    files
}

fn valid_intake_yaml() -> String {
    let mut document = String::from("mode: guided_adaptive\ncontent:\n");
    for line in CHARTER_YAML.lines() {
        document.push_str("  ");
        document.push_str(line);
        document.push('\n');
    }
    document.push_str("coverage:\n");
    for coverage_id in COVERAGE_IDS {
        document.push_str(&format!(
            "  - coverage_id: {coverage_id:?}\n    source_kind: user_declaration\n    value_ref: \"input://{coverage_id}\"\n    evidence_refs: []\n    confidence: high\n    freshness: null\n    sensitivity: public\n    contradiction_refs: []\n    waiver_ref: null\n"
        ));
    }
    document.push_str(
        "consumer:\n  kind: agent\n  id: handbook-charter-intake\n  version: \"1.0\"\nprompt_event_refs: []\nfinalized_at_utc: \"2026-07-20T00:00:00Z\"\nexpected_current_fingerprint: null\n",
    );
    document
}

#[test]
fn charter_help_exposes_only_the_frozen_operation_grammar() {
    let repo = tempfile::tempdir().unwrap();
    let output = run(repo.path(), &["author", "charter", "--help"]);
    assert!(output.status.success(), "{}", stdout(&output));
    let help = stdout(&output);
    for flag in [
        "--mode",
        "--from-inputs",
        "--expected-current-fingerprint",
        "--approve-candidate",
        "--approval-class",
        "--authority-ref",
        "--accept-waiver-ref",
        "--promote-candidate",
        "--approval-ref",
        "--validate",
        "--json",
    ] {
        assert!(help.contains(flag), "missing {flag} in:\n{help}");
    }
    assert!(!help.contains("--interactive"), "{help}");
    assert!(!help.contains("--yes"), "{help}");
}

#[test]
fn legacy_from_inputs_without_mode_refuses_before_read_or_mutation() {
    let repo = tempfile::tempdir().unwrap();
    let legacy = repo.path().join("legacy-inputs.yaml");
    fs::write(&legacy, "this is deliberately not selected YAML\n").unwrap();
    let legacy_charter = repo.path().join(".handbook/charter/CHARTER.md");
    fs::create_dir_all(legacy_charter.parent().unwrap()).unwrap();
    fs::write(&legacy_charter, "conflicting legacy charter\n").unwrap();
    let before = fs::read(&legacy_charter).unwrap();

    let output = run(
        repo.path(),
        &[
            "author",
            "charter",
            "--from-inputs",
            legacy.to_str().unwrap(),
            "--json",
        ],
    );

    assert!(!output.status.success());
    let value: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(value["operation"], "author");
    assert_eq!(value["status"], "refused");
    assert_eq!(value["refusal"]["code"], "legacy_input_refused");
    assert_eq!(value["changed_paths"], serde_json::json!([]));
    assert_eq!(fs::read(&legacy_charter).unwrap(), before);
    assert!(!repo.path().join(".handbook/project/charter.yaml").exists());
    assert!(!repo.path().join(".handbook/state").exists());
}

#[test]
fn author_intent_is_evaluated_but_fails_closed_without_lineage_persistence() {
    let repo = tempfile::tempdir().unwrap();
    let input = repo.path().join("charter-intake.yaml");
    let intake_yaml = valid_intake_yaml();
    fs::write(&input, &intake_yaml).unwrap();
    let decisions = handbook_engine::resolve_shipped_profile_decisions(repo.path()).unwrap();
    let bundle = handbook_engine::evaluate_charter_intake(
        &decisions,
        serde_yaml_bw::from_str(&intake_yaml).unwrap(),
        None,
    )
    .unwrap();
    let expected_intake_ref = bundle.candidate_subject.intake_record_ref.clone();

    let output = run(
        repo.path(),
        &[
            "author",
            "charter",
            "--mode",
            "guided-adaptive",
            "--from-inputs",
            input.to_str().unwrap(),
            "--json",
        ],
    );

    assert!(output.status.success(), "{}", stdout(&output));
    let value: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(value["operation"], "author");
    assert_eq!(value["status"], "succeeded");
    assert_eq!(value["intake_ref"], expected_intake_ref);
    assert_eq!(
        value["intake_fingerprint"],
        bundle.intake.record_fingerprint
    );
    let candidate_ref = value["candidate_ref"].as_str().unwrap();
    let candidate: serde_json::Value = serde_json::from_slice(
        &fs::read(
            repo.path()
                .join(".handbook/evidence/charter")
                .join(candidate_ref),
        )
        .unwrap(),
    )
    .unwrap();
    assert_eq!(candidate["schema_version"], "1.3");
    assert_eq!(
        value["candidate_fingerprint"],
        candidate["candidate_fingerprint"]
    );
    assert_eq!(
        candidate["candidate_subject_fingerprint"],
        bundle.candidate_subject.candidate_subject_fingerprint
    );
    let validation_result_ref = candidate["validation_result_binding"]["validation_result_ref"]
        .as_str()
        .unwrap();
    let expected_paths = serde_json::json!([
        format!(
            ".handbook/state/{}",
            bundle.candidate_subject.normalized_content_ref
        ),
        format!(".handbook/state/{expected_intake_ref}"),
        format!(".handbook/state/{validation_result_ref}"),
        format!(".handbook/evidence/charter/{candidate_ref}"),
    ]);
    assert_eq!(value["changed_paths"], expected_paths);
    assert!(value["refusal"].is_null());
    assert!(!value["next_actions"].as_array().unwrap().is_empty());
    assert!(!repo.path().join(".handbook/project/charter.yaml").exists());
    for path in value["changed_paths"].as_array().unwrap() {
        assert!(repo.path().join(path.as_str().unwrap()).is_file());
    }

    let replay = run(
        repo.path(),
        &[
            "author",
            "charter",
            "--mode",
            "guided-adaptive",
            "--from-inputs",
            input.to_str().unwrap(),
            "--json",
        ],
    );
    assert!(replay.status.success(), "{}", stdout(&replay));
    let replay_value: serde_json::Value = serde_json::from_slice(&replay.stdout).unwrap();
    assert_eq!(replay_value["intake_ref"], value["intake_ref"]);
    assert_eq!(replay_value["candidate_ref"], value["candidate_ref"]);
    assert_eq!(replay_value["changed_paths"], value["changed_paths"]);
    assert!(!repo.path().join(".handbook/project/charter.yaml").exists());
}

#[test]
fn validate_is_selected_truth_only_and_non_mutating() {
    let repo = tempfile::tempdir().unwrap();
    let selected = repo.path().join(".handbook/project/charter.yaml");
    fs::create_dir_all(selected.parent().unwrap()).unwrap();
    fs::write(&selected, CHARTER_YAML).unwrap();
    let before = fs::read(&selected).unwrap();

    let output = run(repo.path(), &["author", "charter", "--validate", "--json"]);

    assert!(output.status.success(), "{}", stdout(&output));
    let value: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(value["operation"], "validate");
    assert_eq!(value["status"], "succeeded");
    assert_eq!(value["canonical_path"], ".handbook/project/charter.yaml");
    assert!(value["source_fingerprint"]
        .as_str()
        .unwrap()
        .starts_with("sha256:"));
    assert!(value["rendered_output_fingerprint"]
        .as_str()
        .unwrap()
        .starts_with("sha256:"));
    assert_eq!(value["changed_paths"], serde_json::json!([]));
    assert_eq!(fs::read(&selected).unwrap(), before);
    assert!(!repo.path().join(".handbook/state").exists());
}

#[test]
fn approver_commands_map_to_exact_identity_preflight_before_native_authority() {
    let cases: [(&[&str], &str); 4] = [
        (
            &[
                "approvers",
                "bootstrap",
                "--initial-charter-quorum",
                "Project owner approval=Project owner",
                "--json",
            ],
            "bootstrap",
        ),
        (
            &[
                "approvers",
                "add-credential",
                "--approval-mapping",
                "Security approval=Security owner",
                "--json",
            ],
            "add_credential",
        ),
        (
            &[
                "approvers",
                "revoke-credential",
                "--credential-id-hash",
                "sha256:2222222222222222222222222222222222222222222222222222222222222222",
                "--json",
            ],
            "revoke_credential",
        ),
        (
            &[
                "approvers",
                "update-mapping",
                "--credential-id-hash",
                "sha256:2222222222222222222222222222222222222222222222222222222222222222",
                "--approval-mapping",
                "Security approval=Security owner",
                "--json",
            ],
            "update_mapping",
        ),
    ];

    for (args, operation) in cases {
        let repo = tempfile::tempdir().unwrap();
        let output = run(repo.path(), args);
        assert!(!output.status.success(), "{operation}");
        let value: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(
            value["schema_id"],
            "handbook.repository-invocation-preflight-result"
        );
        assert_eq!(value["schema_version"], "1.0");
        assert_eq!(value["operation"], operation);
        assert_eq!(value["stage"], "repository_identity");
        assert_eq!(value["status"], "refused");
        assert_eq!(value["refusal"]["code"], "repository_identity_unavailable");
        assert_eq!(
            value["repository_identity_fingerprint"],
            serde_json::Value::Null
        );
        assert_eq!(value["operation_id"], serde_json::Value::Null);
        assert_eq!(value["changed_paths"], serde_json::json!([]));
        assert!(!repo.path().join(".handbook").exists());
    }
}

#[test]
fn approver_mapping_grammar_refusal_stays_cli_owned_and_non_mutating() {
    let repo = tempfile::tempdir().unwrap();
    let before = snapshot_files(repo.path());

    let output = run(
        repo.path(),
        &[
            "approvers",
            "add-credential",
            "--approval-mapping",
            "missing-separator",
            "--json",
        ],
    );

    assert_eq!(output.status.code(), Some(1));
    assert_eq!(output.stderr, b"");
    assert_eq!(
        output.stdout,
        br#"{
  "changed_paths": [],
  "next_actions": [
    "use one or more exact non-empty class=authority approval mappings"
  ],
  "operation": "add_credential",
  "refusal": {
    "code": "invalid_request",
    "message": "approver mapping must use exact class=authority syntax",
    "retryable": false
  },
  "schema_id": "handbook.approver-admin-adapter-result",
  "schema_version": "1.0",
  "status": "refused"
}
"#,
    );
    assert_eq!(snapshot_files(repo.path()), before);

    let duplicate = run(
        repo.path(),
        &[
            "approvers",
            "add-credential",
            "--approval-mapping",
            "owner=authority",
            "--approval-mapping",
            "owner=authority",
            "--json",
        ],
    );

    assert_eq!(duplicate.status.code(), Some(1));
    assert_eq!(duplicate.stderr, b"");
    assert_eq!(
        duplicate.stdout,
        br#"{
  "changed_paths": [],
  "next_actions": [
    "use one or more exact non-empty class=authority approval mappings"
  ],
  "operation": "add_credential",
  "refusal": {
    "code": "invalid_request",
    "message": "approver mappings must be unique",
    "retryable": false
  },
  "schema_id": "handbook.approver-admin-adapter-result",
  "schema_version": "1.0",
  "status": "refused"
}
"#,
    );
    assert_eq!(snapshot_files(repo.path()), before);
}

#[test]
fn unsafe_predecessor_recovery_projects_the_exact_closed_refusal_with_zero_delta() {
    let repo = tempfile::tempdir().unwrap();
    fs::create_dir_all(
        repo.path()
            .join(".handbook/state/transactions/registry/unknown-evidence"),
    )
    .unwrap();
    let before = snapshot_files(repo.path());

    let output = run(
        repo.path(),
        &[
            "approvers",
            "bootstrap",
            "--initial-charter-quorum",
            "Project owner approval=Project owner",
            "--json",
        ],
    );

    assert!(!output.status.success());
    let value: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(
        value,
        serde_json::json!({
            "schema_id": "handbook.repository-invocation-recovery-refusal",
            "schema_version": "1.0",
            "operation": "bootstrap",
            "status": "refused",
            "repository_identity_fingerprint": null,
            "operation_id": null,
            "changed_paths": [],
            "refusal": {
                "code": "authority_recovery_blocked",
                "message": "repository authority recovery could not complete safely",
                "retryable": false
            },
            "next_actions": [
                "repair retained registry/approval recovery evidence, then retry the complete operation"
            ]
        })
    );
    let mut after = snapshot_files(repo.path());
    assert_eq!(
        after.remove(".handbook/state/locks/promotion.lock"),
        Some(Vec::new())
    );
    assert_eq!(
        after.remove(".handbook/state/locks/registry.lock"),
        Some(Vec::new())
    );
    assert_eq!(after, before);
    assert!(!repo
        .path()
        .join(".handbook/repository-identity.v1")
        .exists());
}

#[test]
fn bootstrap_reaches_engine_and_native_unavailability_has_zero_current_operation_delta() {
    let repo = tempfile::tempdir().unwrap();
    fs::create_dir(repo.path().join(".handbook")).unwrap();
    handbook_engine::RepositoryInvocationIdentityServiceV1::new()
        .initialize_for_setup(repo.path())
        .unwrap();
    let identity =
        fs::read_to_string(repo.path().join(".handbook/repository-identity.v1")).unwrap();
    let before = snapshot_files(repo.path());

    let output = run(
        repo.path(),
        &[
            "approvers",
            "bootstrap",
            "--initial-charter-quorum",
            "Project owner approval=Project owner",
            "--json",
        ],
    );

    assert!(!output.status.success());
    let value: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(value["schema_id"], "handbook.approver-admin-result");
    assert_eq!(value["schema_version"], "1.0");
    assert_eq!(value["operation"], "bootstrap");
    assert_eq!(value["status"], "refused");
    assert_eq!(value["repository_identity_fingerprint"], identity);
    assert!(value["operation_id"]
        .as_str()
        .unwrap()
        .starts_with("bootstrap-"));
    assert_eq!(value["refusal"]["code"], "AUTHENTICATOR_UNAVAILABLE");
    assert_eq!(
        value["refusal"]["message"],
        "native CTAP2.1 authenticator API is unavailable"
    );
    assert_eq!(value["refusal"]["retryable"], true);
    assert_eq!(value["changed_paths"], serde_json::json!([]));
    assert_eq!(snapshot_files(repo.path()), before);
}

#[test]
fn human_refusal_projection_carries_a_typed_next_action() {
    let repo = tempfile::tempdir().unwrap();
    let output = run(repo.path(), &["author", "charter", "--from-inputs", "x"]);
    assert!(!output.status.success());
    let text = stdout(&output);
    assert!(text.starts_with("OUTCOME: REFUSED\n"), "{text}");
    assert!(text.contains("OPERATION: author"), "{text}");
    assert!(text.contains("CODE: legacy_input_refused"), "{text}");
    assert!(text.contains("NEXT SAFE ACTION:"), "{text}");
}
