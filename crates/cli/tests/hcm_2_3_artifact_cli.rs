use std::io::Write;
use std::process::{Command, Output, Stdio};
use std::{fs, path::Path};

fn handbook(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_handbook"))
        .args(args)
        .output()
        .expect("run handbook")
}

fn stdout(output: &Output) -> String {
    String::from_utf8(output.stdout.clone()).expect("UTF-8 stdout")
}

fn stderr(output: &Output) -> String {
    String::from_utf8(output.stderr.clone()).expect("UTF-8 stderr")
}

const CLI_KIND_REF: &str = "example.artifact-kind.registry-brief@1.0.0";
const CLI_INSTANCE_ID: &str = "registry_brief";

fn copy_cli_tree(source: &Path, target: &Path) {
    fs::create_dir_all(target).unwrap();
    for entry in fs::read_dir(source).unwrap() {
        let entry = entry.unwrap();
        let source_path = entry.path();
        let target_path = target.join(entry.file_name());
        if source_path.is_dir() {
            copy_cli_tree(&source_path, &target_path);
        } else {
            fs::copy(source_path, target_path).unwrap();
        }
    }
}

fn cli_at(cwd: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_handbook"))
        .current_dir(cwd)
        .args(args)
        .output()
        .expect("run handbook from explicit working directory")
}

fn successful_json(output: Output) -> serde_json::Value {
    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    serde_json::from_slice(&output.stdout).expect("JSON stdout")
}

fn cli_fixture() -> tempfile::TempDir {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../engine/tests/fixtures/hcm_2_3_generic_custom_kind");
    let repo = tempfile::tempdir().unwrap();
    copy_cli_tree(&fixture, repo.path());
    let setup = cli_at(repo.path(), &["setup"]);
    assert!(
        repo.path()
            .join(".handbook/repository-identity.v1")
            .is_file(),
        "stdout={} stderr={}",
        stdout(&setup),
        stderr(&setup)
    );
    repo
}

struct CompletedCandidateCliFixture {
    repo: tempfile::TempDir,
    intake_record_ref: String,
    intake_record_fingerprint: String,
    candidate_request: std::path::PathBuf,
    candidate_result_fingerprint: String,
}

fn completed_candidate_cli_fixture() -> CompletedCandidateCliFixture {
    let repo = cli_fixture();
    let outside = tempfile::tempdir().unwrap();
    let root = repo.path().to_str().unwrap();
    let validation = successful_json(cli_at(
        outside.path(),
        &[
            "artifact",
            "validate",
            "--repository-root",
            root,
            "--kind-ref",
            CLI_KIND_REF,
            "--instance-id",
            CLI_INSTANCE_ID,
            "--json",
        ],
    ));
    let current = validation["artifact_fingerprint"].as_str().unwrap();
    let coverage = serde_json::json!([
        {
            "coverage_id": "registry_brief.title",
            "state": "supplied",
            "source_kind": "user_declaration",
            "value": "Registry Brief CLI Recovery",
            "specificity": "exact",
            "confidence": "high",
            "contradiction_refs": []
        },
        {
            "coverage_id": "registry_brief.summary",
            "state": "supplied",
            "source_kind": "user_declaration",
            "value": "Actual binary recovery proof",
            "specificity": "concrete",
            "confidence": "high",
            "contradiction_refs": []
        }
    ]);
    let intake_request = repo.path().join("cli-recovery-intake.json");
    fs::write(
        &intake_request,
        serde_json::to_vec(&serde_json::json!({
            "idempotency_key": "cli_recovery_intake_000001",
            "acquisition_mode": "express",
            "expected_current_artifact_fingerprint": current,
            "coverage_submissions": coverage,
        }))
        .unwrap(),
    )
    .unwrap();
    let intake = successful_json(cli_at(
        outside.path(),
        &[
            "artifact",
            "intake-append",
            "--repository-root",
            root,
            "--kind-ref",
            CLI_KIND_REF,
            "--instance-id",
            CLI_INSTANCE_ID,
            "--from-request",
            intake_request.to_str().unwrap(),
            "--json",
        ],
    ));
    let intake_record_ref = intake["authoritative_outputs"][0]["ref"]
        .as_str()
        .unwrap()
        .to_owned();
    let intake_record_fingerprint = intake["authoritative_outputs"][0]["fingerprint"]
        .as_str()
        .unwrap()
        .to_owned();
    let preview = successful_json(cli_at(
        outside.path(),
        &[
            "artifact",
            "candidate-validate",
            "--repository-root",
            root,
            "--kind-ref",
            CLI_KIND_REF,
            "--instance-id",
            CLI_INSTANCE_ID,
            "--intake-record-ref",
            &intake_record_ref,
            "--intake-record-fingerprint",
            &intake_record_fingerprint,
            "--expected-current-fingerprint",
            current,
            "--json",
        ],
    ));
    let candidate_request = repo.path().join("cli-recovery-candidate.json");
    fs::write(
        &candidate_request,
        serde_json::to_vec(&serde_json::json!({
            "idempotency_key": "cli_recovery_candidate_0001",
            "intake_record_ref": intake_record_ref,
            "intake_record_fingerprint": intake_record_fingerprint,
            "expected_candidate_fingerprint": preview["candidate_fingerprint"],
        }))
        .unwrap(),
    )
    .unwrap();
    let candidate = successful_json(cli_at(
        outside.path(),
        &[
            "artifact",
            "candidate-append",
            "--repository-root",
            root,
            "--kind-ref",
            CLI_KIND_REF,
            "--instance-id",
            CLI_INSTANCE_ID,
            "--from-request",
            candidate_request.to_str().unwrap(),
            "--json",
        ],
    ));
    CompletedCandidateCliFixture {
        repo,
        intake_record_ref,
        intake_record_fingerprint,
        candidate_request,
        candidate_result_fingerprint: candidate["result_fingerprint"].as_str().unwrap().to_owned(),
    }
}

fn candidate_journal(root: &Path) -> (std::path::PathBuf, serde_json::Value) {
    let family = root.join(".handbook/state/transactions/artifact-candidates");
    let committed = fs::read_dir(family)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .find(|path| {
            path.extension()
                .is_some_and(|extension| extension == "committed")
        })
        .expect("committed candidate journal");
    let intent = serde_json::from_slice(&fs::read(committed.join("intent.json")).unwrap())
        .expect("candidate intent");
    (committed, intent)
}

fn cli_tree_bytes(root: &Path) -> Vec<(String, Vec<u8>)> {
    fn visit(root: &Path, path: &Path, entries: &mut Vec<(String, Vec<u8>)>) {
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                visit(root, &path, entries);
            } else {
                entries.push((
                    path.strip_prefix(root)
                        .unwrap()
                        .to_string_lossy()
                        .replace('\\', "/"),
                    fs::read(path).unwrap(),
                ));
            }
        }
    }
    let mut entries = Vec::new();
    visit(root, root, &mut entries);
    entries.sort_by(|left, right| left.0.cmp(&right.0));
    entries
}

#[test]
fn artifact_command_exposes_only_the_frozen_generic_subcommands() {
    let output = handbook(&["artifact", "--help"]);
    assert!(output.status.success(), "{}", stderr(&output));
    let help = stdout(&output);
    for command in [
        "list-kinds",
        "list-instances",
        "read",
        "validate",
        "intake-definition",
        "intake-evaluate",
        "intake-append",
        "candidate-validate",
        "candidate-append",
        "promote",
    ] {
        assert!(help.contains(command), "missing {command}: {help}");
    }
    assert!(!help.contains("registry-brief"));
    assert!(!help.contains("approve"));
}

#[test]
fn every_artifact_operation_requires_an_explicit_repository_root() {
    for command in [
        "list-kinds",
        "list-instances",
        "read",
        "validate",
        "intake-definition",
        "intake-evaluate",
        "intake-append",
        "candidate-validate",
        "candidate-append",
        "promote",
    ] {
        let output = handbook(&["artifact", command]);
        assert!(!output.status.success(), "{command}");
        assert!(stderr(&output).contains("--repository-root"), "{command}");
    }
}

#[test]
fn mutation_idempotency_key_is_data_not_an_argv_option() {
    for command in ["intake-append", "candidate-append", "promote"] {
        let output = handbook(&["artifact", command, "--help"]);
        assert!(output.status.success(), "{}", stderr(&output));
        let help = stdout(&output);
        assert!(help.contains("--from-request"), "{command}: {help}");
        assert!(!help.contains("idempotency"), "{command}: {help}");
    }
}

#[test]
fn candidate_validate_uses_the_exact_committed_intake_pair() {
    let output = handbook(&["artifact", "candidate-validate", "--help"]);
    assert!(output.status.success(), "{}", stderr(&output));
    let help = stdout(&output);
    for option in [
        "--repository-root",
        "--kind-ref",
        "--instance-id",
        "--intake-record-ref",
        "--intake-record-fingerprint",
        "--expected-current-fingerprint",
        "--json",
    ] {
        assert!(help.contains(option), "missing {option}: {help}");
    }
    assert!(!help.contains("--approval"));
}

#[test]
fn actual_binary_resolves_reads_validates_and_purely_evaluates_the_custom_fixture() {
    fn copy_tree(source: &Path, target: &Path) {
        fs::create_dir_all(target).unwrap();
        for entry in fs::read_dir(source).unwrap() {
            let entry = entry.unwrap();
            let source_path = entry.path();
            let target_path = target.join(entry.file_name());
            if source_path.is_dir() {
                copy_tree(&source_path, &target_path);
            } else {
                fs::copy(source_path, target_path).unwrap();
            }
        }
    }

    fn run_at(root: &Path, args: &[&str]) -> Output {
        Command::new(env!("CARGO_BIN_EXE_handbook"))
            .current_dir(root)
            .args(args)
            .output()
            .expect("run handbook at fixture root")
    }

    fn run_at_with_stdin(root: &Path, args: &[&str], input: &[u8]) -> Output {
        let mut child = Command::new(env!("CARGO_BIN_EXE_handbook"))
            .current_dir(root)
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("run handbook with stdin");
        child.stdin.as_mut().unwrap().write_all(input).unwrap();
        child.wait_with_output().expect("collect handbook output")
    }

    fn json(output: Output) -> serde_json::Value {
        assert!(output.status.success(), "{}", stderr(&output));
        serde_json::from_slice(&output.stdout).expect("JSON output")
    }

    fn snapshot_tree(root: &Path) -> Vec<(String, Vec<u8>)> {
        fn visit(root: &Path, path: &Path, entries: &mut Vec<(String, Vec<u8>)>) {
            for entry in fs::read_dir(path).unwrap() {
                let entry = entry.unwrap();
                let path = entry.path();
                if path.is_dir() {
                    visit(root, &path, entries);
                } else {
                    entries.push((
                        path.strip_prefix(root)
                            .unwrap()
                            .to_string_lossy()
                            .replace('\\', "/"),
                        fs::read(path).unwrap(),
                    ));
                }
            }
        }

        let mut entries = Vec::new();
        visit(root, root, &mut entries);
        entries.sort_by(|left, right| left.0.cmp(&right.0));
        entries
    }

    fn retained_json(root: &Path, relative_ref: &str) -> serde_json::Value {
        let bytes = fs::read(root.join(relative_ref)).unwrap();
        assert_eq!(bytes.last(), Some(&b'\n'));
        serde_json::from_slice(&bytes[..bytes.len() - 1]).expect("retained JSON")
    }

    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../engine/tests/fixtures/hcm_2_3_generic_custom_kind");
    let repo = tempfile::tempdir().unwrap();
    copy_tree(&fixture, repo.path());
    let setup = run_at(repo.path(), &["setup"]);
    assert!(
        repo.path()
            .join(".handbook/repository-identity.v1")
            .is_file(),
        "stdout={} stderr={}",
        stdout(&setup),
        stderr(&setup)
    );

    let root = repo.path().to_str().unwrap();
    let kind_ref = "example.artifact-kind.registry-brief@1.0.0";
    let instance_id = "registry_brief";
    let kinds = json(run_at(
        repo.path(),
        &[
            "artifact",
            "list-kinds",
            "--repository-root",
            root,
            "--json",
        ],
    ));
    assert!(kinds["kinds"]
        .as_array()
        .unwrap()
        .iter()
        .any(|kind| kind["kind_ref"] == kind_ref));
    let instances = json(run_at(
        repo.path(),
        &[
            "artifact",
            "list-instances",
            "--repository-root",
            root,
            "--json",
        ],
    ));
    assert!(instances["instances"]
        .as_array()
        .unwrap()
        .iter()
        .any(|instance| instance["instance_id"] == instance_id));

    let mut current_artifact_fingerprint = None;
    for operation in ["read", "validate", "intake-definition"] {
        let result = json(run_at(
            repo.path(),
            &[
                "artifact",
                operation,
                "--repository-root",
                root,
                "--kind-ref",
                kind_ref,
                "--instance-id",
                instance_id,
                "--json",
            ],
        ));
        assert_eq!(
            result["operation_id"],
            match operation {
                "read" => "artifact.read",
                "validate" => "artifact.validate",
                _ => "intake.definition.read",
            }
        );
        if operation == "validate" {
            current_artifact_fingerprint =
                result["artifact_fingerprint"].as_str().map(str::to_owned);
        }
    }

    let input_path = repo.path().join("coverage-input.json");
    fs::write(
        &input_path,
        br#"{"coverage_submissions":[{"coverage_id":"registry_brief.title","state":"supplied","source_kind":"user_declaration","value":"Registry Brief Proof","specificity":"exact","confidence":"high","contradiction_refs":[]},{"coverage_id":"registry_brief.summary","state":"supplied","source_kind":"user_declaration","value":"A reusable registry brief","specificity":"concrete","confidence":"high","contradiction_refs":[]}]}"#,
    )
    .unwrap();
    let input = input_path.to_str().unwrap();
    let before_state = fs::read_dir(repo.path().join(".handbook/state"))
        .unwrap()
        .map(|entry| entry.unwrap().file_name())
        .collect::<Vec<_>>();
    let stale_evaluation = run_at(
        repo.path(),
        &[
            "artifact",
            "intake-evaluate",
            "--repository-root",
            root,
            "--kind-ref",
            kind_ref,
            "--instance-id",
            instance_id,
            "--mode",
            "express",
            "--from-inputs",
            input,
            "--expected-current-fingerprint",
            "absent",
            "--json",
        ],
    );
    assert!(!stale_evaluation.status.success());
    assert!(stderr(&stale_evaluation).contains("expected current"));
    let mut normalized = None;
    for mode in ["guided-adaptive", "express", "agent-assisted"] {
        let result = json(run_at(
            repo.path(),
            &[
                "artifact",
                "intake-evaluate",
                "--repository-root",
                root,
                "--kind-ref",
                kind_ref,
                "--instance-id",
                instance_id,
                "--mode",
                mode,
                "--from-inputs",
                input,
                "--expected-current-fingerprint",
                current_artifact_fingerprint.as_deref().unwrap(),
                "--json",
            ],
        ));
        let content = result["normalized_content"].clone();
        assert_eq!(
            content,
            serde_json::json!({
                "summary": "A reusable registry brief",
                "title": "Registry Brief Proof"
            })
        );
        if let Some(previous) = &normalized {
            assert_eq!(previous, &content);
        }
        normalized = Some(content);
    }
    let stdin_evaluation = json(run_at_with_stdin(
        repo.path(),
        &[
            "artifact",
            "intake-evaluate",
            "--repository-root",
            root,
            "--kind-ref",
            kind_ref,
            "--instance-id",
            instance_id,
            "--mode",
            "express",
            "--from-inputs",
            "-",
            "--expected-current-fingerprint",
            current_artifact_fingerprint.as_deref().unwrap(),
            "--json",
        ],
        &fs::read(&input_path).unwrap(),
    ));
    assert_eq!(stdin_evaluation["normalized_content"], normalized.unwrap());
    let after_state = fs::read_dir(repo.path().join(".handbook/state"))
        .unwrap()
        .map(|entry| entry.unwrap().file_name())
        .collect::<Vec<_>>();
    assert_eq!(before_state, after_state, "pure evaluation changed state");
    assert!(!repo.path().join(".handbook/evidence/artifacts").exists());

    let intake_request_path = repo.path().join("intake-append.json");
    fs::write(
        &intake_request_path,
        serde_json::to_vec(&serde_json::json!({
            "idempotency_key": "registry_intake_00000001",
            "acquisition_mode": "express",
            "expected_current_artifact_fingerprint": current_artifact_fingerprint,
            "coverage_submissions": [
                {
                    "coverage_id": "registry_brief.title",
                    "state": "supplied",
                    "source_kind": "user_declaration",
                    "value": "Registry Brief Proof",
                    "specificity": "exact",
                    "confidence": "high",
                    "contradiction_refs": []
                },
                {
                    "coverage_id": "registry_brief.summary",
                    "state": "supplied",
                    "source_kind": "user_declaration",
                    "value": "A reusable registry brief",
                    "specificity": "concrete",
                    "confidence": "high",
                    "contradiction_refs": []
                }
            ]
        }))
        .unwrap(),
    )
    .unwrap();
    let intake = json(run_at(
        repo.path(),
        &[
            "artifact",
            "intake-append",
            "--repository-root",
            root,
            "--kind-ref",
            kind_ref,
            "--instance-id",
            instance_id,
            "--from-request",
            intake_request_path.to_str().unwrap(),
            "--json",
        ],
    ));
    assert_eq!(intake["operation_id"], "intake.record.append");
    assert_eq!(intake["outcome"], "committed");
    assert_eq!(intake["authoritative_outputs"].as_array().unwrap().len(), 1);
    let stale_intake_request_path = repo.path().join("stale-intake-append.json");
    let mut stale_intake_request: serde_json::Value =
        serde_json::from_slice(&fs::read(&intake_request_path).unwrap()).unwrap();
    stale_intake_request["idempotency_key"] =
        serde_json::Value::String("registry_intake_stale_00000001".to_string());
    stale_intake_request["expected_current_artifact_fingerprint"] = serde_json::Value::Null;
    fs::write(
        &stale_intake_request_path,
        serde_json::to_vec(&stale_intake_request).unwrap(),
    )
    .unwrap();
    let stale_intake = run_at(
        repo.path(),
        &[
            "artifact",
            "intake-append",
            "--repository-root",
            root,
            "--kind-ref",
            kind_ref,
            "--instance-id",
            instance_id,
            "--from-request",
            stale_intake_request_path.to_str().unwrap(),
            "--json",
        ],
    );
    assert!(!stale_intake.status.success());
    let stale_intake: serde_json::Value =
        serde_json::from_slice(&stale_intake.stdout).expect("typed refusal JSON");
    assert_eq!(stale_intake["outcome"], "refused");
    assert_eq!(stale_intake["execution_disposition"], "refused");
    assert_eq!(stale_intake["refusal"]["code"], "stale_current_artifact");
    assert!(stale_intake["authoritative_outputs"]
        .as_array()
        .unwrap()
        .is_empty());
    assert!(stale_intake["internal_transaction_evidence_ref"].is_string());
    let after_stale_refusal = snapshot_tree(&repo.path().join(".handbook"));
    let stale_replay = run_at(
        repo.path(),
        &[
            "artifact",
            "intake-append",
            "--repository-root",
            root,
            "--kind-ref",
            kind_ref,
            "--instance-id",
            instance_id,
            "--from-request",
            stale_intake_request_path.to_str().unwrap(),
            "--json",
        ],
    );
    assert!(!stale_replay.status.success());
    let stale_replay: serde_json::Value =
        serde_json::from_slice(&stale_replay.stdout).expect("replayed refusal JSON");
    assert_eq!(stale_replay["outcome"], "refused");
    assert_eq!(stale_replay["execution_disposition"], "replayed");
    assert_eq!(
        stale_replay["result_fingerprint"],
        stale_intake["result_fingerprint"]
    );
    assert_eq!(
        after_stale_refusal,
        snapshot_tree(&repo.path().join(".handbook")),
        "retained refusal replay changed repository state"
    );
    let intake_output = &intake["authoritative_outputs"][0];
    let intake_record_ref = intake_output["ref"].as_str().unwrap();
    let intake_record_fingerprint = intake_output["fingerprint"].as_str().unwrap();
    let intake_record = retained_json(repo.path(), intake_record_ref);
    assert_eq!(
        intake_record["schema_id"],
        "handbook.artifact-intake-record"
    );
    assert_eq!(intake_record["schema_version"], "1.2");
    assert_eq!(intake_record["target_kind_ref"], kind_ref);
    assert_eq!(intake_record["target_instance_id"], instance_id);
    assert_eq!(
        intake_record["record_fingerprint"],
        intake_record_fingerprint
    );
    assert_eq!(
        intake_record["basis_artifact_fingerprint"],
        current_artifact_fingerprint.as_deref().unwrap()
    );
    assert_eq!(
        intake_record["coverage_results"].as_array().unwrap().len(),
        2
    );
    let before_candidate_validate = snapshot_tree(&repo.path().join(".handbook"));
    let candidate = json(run_at(
        repo.path(),
        &[
            "artifact",
            "candidate-validate",
            "--repository-root",
            root,
            "--kind-ref",
            kind_ref,
            "--instance-id",
            instance_id,
            "--intake-record-ref",
            intake_record_ref,
            "--intake-record-fingerprint",
            intake_record_fingerprint,
            "--expected-current-fingerprint",
            current_artifact_fingerprint.as_deref().unwrap(),
            "--json",
        ],
    ));
    assert_eq!(candidate["operation_id"], "artifact.candidate.validate");
    assert_eq!(candidate["outcome"], "valid");
    assert_eq!(candidate["intake_record_ref"], intake_record_ref);
    assert_eq!(
        candidate["intake_record_fingerprint"],
        intake_record_fingerprint
    );
    assert_eq!(
        candidate["normalized_content"],
        serde_json::json!({
            "summary": "A reusable registry brief",
            "title": "Registry Brief Proof"
        })
    );
    assert!(candidate["candidate_fingerprint"]
        .as_str()
        .unwrap()
        .starts_with("sha256:"));
    assert!(candidate["validation_result_fingerprint"]
        .as_str()
        .unwrap()
        .starts_with("sha256:"));
    assert_eq!(
        before_candidate_validate,
        snapshot_tree(&repo.path().join(".handbook")),
        "candidate validation changed repository state"
    );

    let candidate_mismatch_path = repo.path().join("candidate-append-mismatch.json");
    fs::write(
        &candidate_mismatch_path,
        serde_json::to_vec(&serde_json::json!({
            "idempotency_key": "registry_candidate_mismatch_0001",
            "intake_record_ref": intake_record_ref,
            "intake_record_fingerprint": intake_record_fingerprint,
            "expected_candidate_fingerprint": "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        }))
        .unwrap(),
    )
    .unwrap();
    let mismatch_args = [
        "artifact",
        "candidate-append",
        "--repository-root",
        root,
        "--kind-ref",
        kind_ref,
        "--instance-id",
        instance_id,
        "--from-request",
        candidate_mismatch_path.to_str().unwrap(),
        "--json",
    ];
    let mismatch = run_at(repo.path(), &mismatch_args);
    assert!(!mismatch.status.success());
    let mismatch: serde_json::Value = serde_json::from_slice(&mismatch.stdout).unwrap();
    assert_eq!(mismatch["outcome"], "refused");
    assert_eq!(mismatch["execution_disposition"], "refused");
    assert_eq!(mismatch["refusal"]["code"], "stale_basis");
    assert_eq!(mismatch["refusal"]["layer"], "currentness");
    assert!(mismatch["authoritative_outputs"]
        .as_array()
        .unwrap()
        .is_empty());
    let after_mismatch = snapshot_tree(&repo.path().join(".handbook"));
    let mismatch_replay = run_at(repo.path(), &mismatch_args);
    assert!(!mismatch_replay.status.success());
    let mismatch_replay: serde_json::Value =
        serde_json::from_slice(&mismatch_replay.stdout).unwrap();
    assert_eq!(mismatch_replay["execution_disposition"], "replayed");
    assert_eq!(
        mismatch_replay["result_fingerprint"],
        mismatch["result_fingerprint"]
    );
    assert!(mismatch_replay["authoritative_outputs"]
        .as_array()
        .unwrap()
        .is_empty());
    assert_eq!(
        after_mismatch,
        snapshot_tree(&repo.path().join(".handbook"))
    );

    let candidate_request_path = repo.path().join("candidate-append.json");
    fs::write(
        &candidate_request_path,
        serde_json::to_vec(&serde_json::json!({
            "idempotency_key": "registry_candidate_00000001",
            "intake_record_ref": intake_record_ref,
            "intake_record_fingerprint": intake_record_fingerprint,
            "expected_candidate_fingerprint": candidate["candidate_fingerprint"],
        }))
        .unwrap(),
    )
    .unwrap();
    let candidate_append = json(run_at(
        repo.path(),
        &[
            "artifact",
            "candidate-append",
            "--repository-root",
            root,
            "--kind-ref",
            kind_ref,
            "--instance-id",
            instance_id,
            "--from-request",
            candidate_request_path.to_str().unwrap(),
            "--json",
        ],
    ));
    assert_eq!(
        candidate_append["operation_id"],
        "artifact.candidate.append"
    );
    assert_eq!(candidate_append["outcome"], "committed");
    assert_eq!(
        candidate_append["authoritative_outputs"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    assert_eq!(
        candidate_append["authoritative_outputs"][0]["fingerprint"],
        candidate["candidate_fingerprint"]
    );

    let committed_candidate = &candidate_append["authoritative_outputs"][0];
    let candidate_record = retained_json(repo.path(), committed_candidate["ref"].as_str().unwrap());
    assert_eq!(candidate_record["schema_id"], "handbook.artifact-candidate");
    assert_eq!(candidate_record["schema_version"], "1.4");
    assert_eq!(candidate_record["target_kind_ref"], kind_ref);
    assert_eq!(candidate_record["target_instance_id"], instance_id);
    assert_eq!(candidate_record["intake_record_ref"], intake_record_ref);
    assert_eq!(
        candidate_record["intake_record_fingerprint"],
        intake_record_fingerprint
    );
    assert_eq!(
        candidate_record["operation_context_fingerprint"],
        intake_record["operation_context_fingerprint"]
    );
    assert_eq!(
        candidate_record["field_sources"].as_array().unwrap().len(),
        2
    );
    assert_eq!(
        retained_json(
            repo.path(),
            candidate_record["normalized_content_ref"].as_str().unwrap()
        ),
        candidate["normalized_content"]
    );
    let validation_record = retained_json(
        repo.path(),
        candidate_record["validation_result_refs"][0]
            .as_str()
            .unwrap(),
    );
    assert_eq!(
        validation_record["schema_id"],
        "handbook.artifact-validation-result"
    );
    assert_eq!(validation_record["schema_version"], "1.0");
    assert_eq!(validation_record["target_kind_ref"], kind_ref);
    assert_eq!(validation_record["target_instance_id"], instance_id);
    assert_eq!(validation_record["outcome"], "valid");
    let promote_request_path = repo.path().join("promote.json");
    fs::write(
        &promote_request_path,
        serde_json::to_vec(&serde_json::json!({
            "idempotency_key": "registry_promotion_00000001",
            "candidate_ref": committed_candidate["ref"],
            "candidate_fingerprint": committed_candidate["fingerprint"],
            "expected_current_artifact_fingerprint": current_artifact_fingerprint,
        }))
        .unwrap(),
    )
    .unwrap();
    let promotion = json(run_at(
        repo.path(),
        &[
            "artifact",
            "promote",
            "--repository-root",
            root,
            "--kind-ref",
            kind_ref,
            "--instance-id",
            instance_id,
            "--from-request",
            promote_request_path.to_str().unwrap(),
            "--json",
        ],
    ));
    assert_eq!(promotion["operation_id"], "artifact.candidate.promote");
    assert_eq!(promotion["outcome"], "committed");
    assert_eq!(
        promotion["authoritative_outputs"].as_array().unwrap().len(),
        2
    );
    let promotion_record = retained_json(
        repo.path(),
        promotion["authoritative_outputs"][1]["ref"]
            .as_str()
            .unwrap(),
    );
    assert_eq!(
        promotion_record["schema_id"],
        "handbook.artifact-promotion-record"
    );
    assert_eq!(promotion_record["schema_version"], "1.2");
    assert_eq!(promotion_record["target_instance_id"], instance_id);
    assert_eq!(
        promotion_record["candidate_ref"],
        committed_candidate["ref"]
    );
    assert_eq!(
        promotion_record["candidate_fingerprint"],
        committed_candidate["fingerprint"]
    );
    assert_eq!(
        promotion_record["operation_context_fingerprint"],
        candidate_record["operation_context_fingerprint"]
    );
    assert_eq!(promotion_record["decision"], "not_required");
    assert!(promotion_record["approval_refs"]
        .as_array()
        .unwrap()
        .is_empty());
    assert_eq!(
        fs::read(repo.path().join(".handbook/project/registry-brief.yaml")).unwrap(),
        b"summary: \"A reusable registry brief\"\ntitle: \"Registry Brief Proof\"\n"
    );
    let restarted_validation = json(run_at(
        repo.path(),
        &[
            "artifact",
            "validate",
            "--repository-root",
            root,
            "--kind-ref",
            kind_ref,
            "--instance-id",
            instance_id,
            "--json",
        ],
    ));
    assert_eq!(restarted_validation["outcome"], "valid");
    assert_eq!(
        restarted_validation["content"],
        candidate["normalized_content"]
    );

    let before_replay = snapshot_tree(&repo.path().join(".handbook"));
    for (operation, request_path, committed) in [
        ("intake-append", &intake_request_path, &intake),
        (
            "candidate-append",
            &candidate_request_path,
            &candidate_append,
        ),
        ("promote", &promote_request_path, &promotion),
    ] {
        let replay = json(run_at(
            repo.path(),
            &[
                "artifact",
                operation,
                "--repository-root",
                root,
                "--kind-ref",
                kind_ref,
                "--instance-id",
                instance_id,
                "--from-request",
                request_path.to_str().unwrap(),
                "--json",
            ],
        ));
        assert_eq!(replay["execution_disposition"], "replayed");
        assert_eq!(
            replay["result_fingerprint"],
            committed["result_fingerprint"]
        );
        assert_eq!(
            replay["authoritative_outputs"],
            committed["authoritative_outputs"]
        );
    }
    let stdin_intake_replay = json(run_at_with_stdin(
        repo.path(),
        &[
            "artifact",
            "intake-append",
            "--repository-root",
            root,
            "--kind-ref",
            kind_ref,
            "--instance-id",
            instance_id,
            "--from-request",
            "-",
            "--json",
        ],
        &fs::read(&intake_request_path).unwrap(),
    ));
    assert_eq!(stdin_intake_replay["execution_disposition"], "replayed");
    assert_eq!(
        before_replay,
        snapshot_tree(&repo.path().join(".handbook")),
        "idempotent replay changed repository state"
    );
    for raw_key in [
        b"registry_intake_00000001".as_slice(),
        b"registry_candidate_00000001".as_slice(),
        b"registry_promotion_00000001".as_slice(),
        b"registry_intake_stale_00000001".as_slice(),
    ] {
        assert!(
            before_replay
                .iter()
                .all(|(_, bytes)| !bytes.windows(raw_key.len()).any(|window| window == raw_key)),
            "raw idempotency key persisted in .handbook"
        );
    }
}

#[test]
fn actual_binary_uses_exact_exit_codes_explicit_roots_and_absent_spelling() {
    let clap_failure = handbook(&["artifact", "list-kinds"]);
    assert_eq!(clap_failure.status.code(), Some(2));

    let repo = cli_fixture();
    let outside = tempfile::tempdir().unwrap();
    let root = repo.path().to_str().unwrap();
    let human_success = cli_at(
        outside.path(),
        &["artifact", "list-kinds", "--repository-root", root],
    );
    assert_eq!(
        human_success.status.code(),
        Some(0),
        "{}",
        stderr(&human_success)
    );
    assert!(stdout(&human_success).contains("artifact.kind.list"));
    assert_ne!(outside.path(), repo.path());

    let missing_selection = cli_fixture();
    fs::remove_file(
        missing_selection
            .path()
            .join(".handbook/profile-selection.json"),
    )
    .unwrap();
    let missing_root = missing_selection.path().to_str().unwrap();
    let human_refusal = cli_at(
        outside.path(),
        &["artifact", "list-kinds", "--repository-root", missing_root],
    );
    assert_eq!(human_refusal.status.code(), Some(1));
    assert!(stderr(&human_refusal).contains("artifact operation refused"));

    let absent_repo = cli_fixture();
    fs::remove_file(
        absent_repo
            .path()
            .join(".handbook/project/registry-brief.yaml"),
    )
    .unwrap();
    let inputs = absent_repo.path().join("absent-inputs.json");
    fs::write(
        &inputs,
        br#"{"coverage_submissions":[{"coverage_id":"registry_brief.title","state":"supplied","source_kind":"user_declaration","value":"Absent Grammar","specificity":"exact","confidence":"high","contradiction_refs":[]},{"coverage_id":"registry_brief.summary","state":"supplied","source_kind":"user_declaration","value":"Exact lowercase token","specificity":"concrete","confidence":"high","contradiction_refs":[]}]}"#,
    )
    .unwrap();
    let absent_root = absent_repo.path().to_str().unwrap();
    let evaluate = |spelling: &str| {
        cli_at(
            outside.path(),
            &[
                "artifact",
                "intake-evaluate",
                "--repository-root",
                absent_root,
                "--kind-ref",
                CLI_KIND_REF,
                "--instance-id",
                CLI_INSTANCE_ID,
                "--mode",
                "express",
                "--from-inputs",
                inputs.to_str().unwrap(),
                "--expected-current-fingerprint",
                spelling,
                "--json",
            ],
        )
    };
    assert_eq!(evaluate("absent").status.code(), Some(0));
    let wrong_case = evaluate("ABSENT");
    assert_eq!(wrong_case.status.code(), Some(1));
    assert!(stderr(&wrong_case).contains("fingerprint"));
}

#[test]
fn actual_binary_recovers_every_post_marker_prefix_and_refuses_non_prefixes() {
    let base = completed_candidate_cli_fixture();
    let request_name = base.candidate_request.file_name().unwrap();
    for prefix_len in 0..=4 {
        let repo = tempfile::tempdir().unwrap();
        copy_cli_tree(base.repo.path(), repo.path());
        let (committed, intent) = candidate_journal(repo.path());
        let transaction_id = intent["transaction_id"].as_str().unwrap();
        let key = intent["domain_mutation_key_fingerprint"]
            .as_str()
            .unwrap()
            .strip_prefix("sha256:")
            .unwrap();
        let result = repo
            .path()
            .join(".handbook/state/idempotency/generic-artifact-operations/results")
            .join(format!("{transaction_id}.json"));
        let ledger = repo
            .path()
            .join(".handbook/state/idempotency/generic-artifact-operations/ledger")
            .join(format!("{key}.json"));
        if prefix_len < 1 {
            fs::remove_file(committed.join("evidence.json")).unwrap();
        }
        if prefix_len < 2 {
            fs::remove_file(&result).unwrap();
        }
        if prefix_len < 3 {
            fs::remove_file(&ledger).unwrap();
        }
        if prefix_len < 4 {
            fs::rename(&committed, committed.with_extension("pending")).unwrap();
        }
        let outside = tempfile::tempdir().unwrap();
        let output = cli_at(
            outside.path(),
            &[
                "artifact",
                "candidate-append",
                "--repository-root",
                repo.path().to_str().unwrap(),
                "--kind-ref",
                CLI_KIND_REF,
                "--instance-id",
                CLI_INSTANCE_ID,
                "--from-request",
                repo.path().join(request_name).to_str().unwrap(),
                "--json",
            ],
        );
        let replay = successful_json(output);
        assert_eq!(replay["execution_disposition"], "replayed");
        assert_eq!(
            replay["result_fingerprint"],
            base.candidate_result_fingerprint
        );
    }

    for missing in ["evidence", "result"] {
        let repo = tempfile::tempdir().unwrap();
        copy_cli_tree(base.repo.path(), repo.path());
        let (committed, intent) = candidate_journal(repo.path());
        if missing == "evidence" {
            fs::remove_file(committed.join("evidence.json")).unwrap();
        } else {
            fs::remove_file(
                repo.path()
                    .join(".handbook/state/idempotency/generic-artifact-operations/results")
                    .join(format!(
                        "{}.json",
                        intent["transaction_id"].as_str().unwrap()
                    )),
            )
            .unwrap();
        }
        fs::rename(&committed, committed.with_extension("pending")).unwrap();
        let before = cli_tree_bytes(repo.path());
        let outside = tempfile::tempdir().unwrap();
        let output = cli_at(
            outside.path(),
            &[
                "artifact",
                "candidate-append",
                "--repository-root",
                repo.path().to_str().unwrap(),
                "--kind-ref",
                CLI_KIND_REF,
                "--instance-id",
                CLI_INSTANCE_ID,
                "--from-request",
                repo.path().join(request_name).to_str().unwrap(),
                "--json",
            ],
        );
        assert_eq!(output.status.code(), Some(1), "missing {missing}");
        assert_eq!(cli_tree_bytes(repo.path()), before, "missing {missing}");
    }
}

#[test]
fn actual_binary_enforces_exact_and_different_requests_under_a_tombstone() {
    let fixture = completed_candidate_cli_fixture();
    let (committed, intent) = candidate_journal(fixture.repo.path());
    assert!(committed.is_dir());
    let key = intent["domain_mutation_key_fingerprint"]
        .as_str()
        .unwrap()
        .strip_prefix("sha256:")
        .unwrap();
    let ledger_path = fixture
        .repo
        .path()
        .join(".handbook/state/idempotency/generic-artifact-operations/ledger")
        .join(format!("{key}.json"));
    let mut ledger: serde_json::Value =
        serde_json::from_slice(&fs::read(&ledger_path).unwrap()).unwrap();
    ledger["state"] = serde_json::Value::String("tombstone".to_owned());
    ledger["result_ref"] = serde_json::Value::Null;
    ledger["result_fingerprint"] = serde_json::Value::Null;
    ledger["retained_until_utc"] = serde_json::Value::Null;
    ledger.as_object_mut().unwrap().remove("entry_fingerprint");
    let fingerprint = handbook_engine::DefinitionFingerprint::from_json_value(&ledger).unwrap();
    ledger["entry_fingerprint"] = serde_json::Value::String(fingerprint.as_str().to_owned());
    let mut bytes = serde_json_canonicalizer::to_vec(&ledger).unwrap();
    bytes.push(b'\n');
    fs::write(&ledger_path, bytes).unwrap();

    let outside = tempfile::tempdir().unwrap();
    let invoke = |request: &Path| {
        cli_at(
            outside.path(),
            &[
                "artifact",
                "candidate-append",
                "--repository-root",
                fixture.repo.path().to_str().unwrap(),
                "--kind-ref",
                CLI_KIND_REF,
                "--instance-id",
                CLI_INSTANCE_ID,
                "--from-request",
                request.to_str().unwrap(),
                "--json",
            ],
        )
    };
    let exact = invoke(&fixture.candidate_request);
    assert_eq!(exact.status.code(), Some(1));
    assert!(stderr(&exact).contains("expired"));

    let different_path = fixture
        .repo
        .path()
        .join("cli-recovery-candidate-different.json");
    let mut different: serde_json::Value =
        serde_json::from_slice(&fs::read(&fixture.candidate_request).unwrap()).unwrap();
    different["expected_candidate_fingerprint"] =
        serde_json::Value::String(format!("sha256:{}", "a".repeat(64)));
    fs::write(&different_path, serde_json::to_vec(&different).unwrap()).unwrap();
    let conflict = invoke(&different_path);
    assert_eq!(conflict.status.code(), Some(1));
    assert!(stderr(&conflict).contains("different request"));

    assert!(!fixture.intake_record_ref.is_empty());
    assert!(fixture.intake_record_fingerprint.starts_with("sha256:"));
}
