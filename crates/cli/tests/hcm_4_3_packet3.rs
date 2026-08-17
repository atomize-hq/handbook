use std::fs;
use std::path::Path;
use std::process::{Command, Output};

fn run(dir: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_handbook"))
        .current_dir(dir)
        .args(args)
        .output()
        .unwrap()
}

fn value(output: &Output) -> serde_json::Value {
    assert!(output.stdout.ends_with(b"\n"));
    assert!(!output.stdout.ends_with(b"\n\n"));
    serde_json::from_slice(&output.stdout).unwrap()
}

fn initialize_identity(dir: &Path) {
    handbook_engine::RepositoryInvocationIdentityServiceV1::new()
        .initialize_for_setup(dir)
        .unwrap();
}

#[test]
fn charter_validate_json_is_one_fixed_use_case_invocation() {
    let repo = tempfile::tempdir().unwrap();
    let output = run(repo.path(), &["author", "charter", "--validate", "--json"]);
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stderr.is_empty());
    let document = value(&output);
    assert_eq!(document["response_kind"], "use_case");
    assert_eq!(
        document["response"]["use_case"]["ref"],
        "handbook.use-case.author-charter@1.0.0"
    );
    assert_eq!(document["response"]["status"], "blocked");
}

#[test]
fn charter_human_projection_uses_the_same_typed_failure_facts_as_new_json() {
    let repo = tempfile::tempdir().unwrap();
    let machine = run(repo.path(), &["author", "charter", "--validate", "--json"]);
    let human = run(repo.path(), &["author", "charter", "--validate"]);
    assert_eq!(machine.status.code(), human.status.code());
    assert!(machine.stderr.is_empty());
    assert!(human.stderr.is_empty());

    let document = value(&machine);
    let details = &document["response"]["blockers"][0]["details"];
    let refusal = &details["refusal"];
    let expected = format!(
        "OUTCOME: REFUSED\nOPERATION: {}\nCODE: {}\nMESSAGE: {}\nRETRYABLE: {}\nNEXT SAFE ACTION: {}\n",
        details["operation"].as_str().unwrap(),
        refusal["code"].as_str().unwrap(),
        refusal["message"].as_str().unwrap(),
        refusal["retryable"].as_bool().unwrap(),
        details["next_actions"][0].as_str().unwrap(),
    );
    assert_eq!(String::from_utf8(human.stdout).unwrap(), expected);
}

#[test]
fn project_context_decode_failure_is_one_transport_invocation() {
    let repo = tempfile::tempdir().unwrap();
    let input = repo.path().join("context.yaml");
    fs::write(&input, "schema_id: [broken\n").unwrap();
    let output = run(
        repo.path(),
        &[
            "author",
            "project-context",
            "--from-inputs",
            input.to_str().unwrap(),
            "--validate",
            "--json",
        ],
    );
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stderr.is_empty());
    let document = value(&output);
    assert_eq!(document["response_kind"], "transport_failure");
    assert_eq!(document["response"]["failure_kind"], "decoding");
    let details = &document["response"]["refusals"][0]["details"];
    assert_eq!(details["presentation"]["presentation_kind"], "author_input");
    assert_eq!(details["presentation"]["route"], "project_context");
    assert_eq!(details["presentation"]["input_source"], "context.yaml");
    assert!(!String::from_utf8_lossy(&output.stdout).contains(repo.path().to_str().unwrap()));
}

#[test]
fn approver_repository_failure_is_one_transport_invocation() {
    let repo = tempfile::tempdir().unwrap();
    let output = run(
        repo.path(),
        &[
            "approvers",
            "bootstrap",
            "--initial-charter-quorum",
            "Project owner=Project owner",
            "--json",
        ],
    );
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stderr.is_empty());
    let document = value(&output);
    assert_eq!(document["response_kind"], "transport_failure");
    assert_eq!(document["response"]["failure_kind"], "repository");
    let details = &document["response"]["blockers"][0]["details"];
    assert_eq!(
        details["presentation"]["presentation_kind"],
        "repository_invocation"
    );
    assert_eq!(details["presentation"]["command"], "approver_bootstrap");
    let wire = String::from_utf8(output.stdout.clone()).unwrap();
    for secret in ["raw_key", "private_key", "authenticator_bytes"] {
        assert!(!wire.contains(secret));
    }
}

#[test]
fn new_and_legacy_json_selectors_are_mutually_exclusive() {
    let repo = tempfile::tempdir().unwrap();
    let output = run(
        repo.path(),
        &["author", "charter", "--validate", "--json", "--legacy-json"],
    );
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
}

#[test]
fn invalid_approver_mapping_uses_one_use_case_for_all_render_modes() {
    let repo = tempfile::tempdir().unwrap();
    let base = [
        "approvers",
        "bootstrap",
        "--initial-charter-quorum",
        "not-a-mapping",
    ];
    let machine = run(repo.path(), &[base[0], base[1], base[2], base[3], "--json"]);
    let human = run(repo.path(), &base);
    let legacy = run(
        repo.path(),
        &[base[0], base[1], base[2], base[3], "--legacy-json"],
    );

    assert_eq!(machine.status.code(), Some(1));
    assert_eq!(human.status.code(), machine.status.code());
    assert_eq!(legacy.status.code(), machine.status.code());
    assert!(machine.stderr.is_empty());
    assert!(human.stderr.is_empty());
    assert!(legacy.stderr.is_empty());
    let document = value(&machine);
    assert_eq!(document["response_kind"], "use_case");
    assert_eq!(document["response"]["status"], "refused");
    assert_eq!(
        document["response"]["use_case"]["ref"],
        "handbook.use-case.approver-administration@1.0.0"
    );
    assert!(String::from_utf8(human.stdout)
        .unwrap()
        .starts_with("OUTCOME: REFUSED\nOPERATION: bootstrap\nCODE: invalid_request\n"));
    let legacy_document: serde_json::Value = serde_json::from_slice(&legacy.stdout).unwrap();
    assert_eq!(legacy_document["operation"], "bootstrap");
    assert_eq!(legacy_document["status"], "refused");
}

#[test]
fn unavailable_charter_input_keeps_selected_human_and_legacy_projections() {
    let repo = tempfile::tempdir().unwrap();
    let missing = repo.path().join("missing-charter-input.yaml");
    let input = missing.to_str().unwrap();
    let common = [
        "author",
        "charter",
        "--mode",
        "guided-adaptive",
        "--from-inputs",
        input,
    ];
    let machine = run(
        repo.path(),
        &[
            common[0], common[1], common[2], common[3], common[4], common[5], "--json",
        ],
    );
    let human = run(repo.path(), &common);
    let legacy = run(
        repo.path(),
        &[
            common[0],
            common[1],
            common[2],
            common[3],
            common[4],
            common[5],
            "--legacy-json",
        ],
    );

    assert_eq!(machine.status.code(), Some(1));
    assert_eq!(human.status.code(), machine.status.code());
    assert_eq!(legacy.status.code(), machine.status.code());
    assert_eq!(value(&machine)["response_kind"], "transport_failure");
    assert!(String::from_utf8(human.stdout)
        .unwrap()
        .starts_with("OUTCOME: REFUSED\nOPERATION: author\nCODE: invalid_request\n"));
    let legacy_document: serde_json::Value = serde_json::from_slice(&legacy.stdout).unwrap();
    assert_eq!(legacy_document["operation"], "author");
    assert_eq!(legacy_document["status"], "refused");
}

#[test]
fn project_context_input_transport_preserves_exact_human_io_presentation() {
    let repo = tempfile::tempdir().unwrap();
    let missing = repo.path().join("missing-project-context.yaml");
    let input = missing.to_str().unwrap();
    let io_error = fs::read_to_string(&missing).unwrap_err().to_string();
    let human = run(
        repo.path(),
        &["author", "project-context", "--from-inputs", input],
    );
    let machine = run(
        repo.path(),
        &[
            "author",
            "project-context",
            "--from-inputs",
            input,
            "--json",
        ],
    );
    let next = "repair the structured input file and retry `handbook author project-context --from-inputs <path|->`";
    let input_label = "missing-project-context.yaml";
    let human_stdout = String::from_utf8(human.stdout).unwrap();
    assert_eq!(
        human_stdout,
        format!(
            "OUTCOME: REFUSED\nOBJECT: author project-context\nNEXT SAFE ACTION: {next}\n## REFUSAL\nCATEGORY: InputReadFailure\nSUMMARY: failed to read structured inputs from `{input_label}`: {io_error}\nBROKEN SUBJECT: structured input source\nNEXT SAFE ACTION: {next}\n"
        )
    );
    assert!(!human_stdout.contains(repo.path().to_str().unwrap()));
    assert!(human.stderr.is_empty());
    assert_eq!(human.status.code(), Some(1));
    let document = value(&machine);
    assert_eq!(document["response_kind"], "transport_failure");
    assert_eq!(
        document["response"]["refusals"][0]["details"]["presentation"]["error"]["kind"],
        "not_found"
    );
}

#[test]
fn over_four_kib_project_context_source_stays_in_the_selected_human_mode() {
    let repo = tempfile::tempdir().unwrap();
    let input = "x".repeat(4097);
    let io_error = fs::read_to_string(&input).unwrap_err().to_string();
    let human = run(
        repo.path(),
        &["author", "project-context", "--from-inputs", &input],
    );
    let next = "repair the structured input file and retry `handbook author project-context --from-inputs <path|->`";
    assert_eq!(human.status.code(), Some(1));
    assert!(human.stderr.is_empty());
    assert_eq!(
        String::from_utf8(human.stdout).unwrap(),
        format!(
            "OUTCOME: REFUSED\nOBJECT: author project-context\nNEXT SAFE ACTION: {next}\n## REFUSAL\nCATEGORY: InputReadFailure\nSUMMARY: failed to read structured inputs from `{input}`: {io_error}\nBROKEN SUBJECT: structured input source\nNEXT SAFE ACTION: {next}\n"
        )
    );
}

#[test]
fn all_ten_packet_three_modes_close_through_their_fixed_use_case() {
    const HASH: &str = "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    const CANDIDATE: &str =
        "candidates/candidate_aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.json";
    const APPROVAL: &str =
        "approvals/approval_aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.json";
    let cases: [(&str, &str, Vec<String>); 10] = [
        (
            "charter-author",
            "handbook.use-case.author-charter@1.0.0",
            vec![
                "author".into(),
                "charter".into(),
                "--mode".into(),
                "guided-adaptive".into(),
                "--from-inputs".into(),
                "charter-input.yaml".into(),
                "--json".into(),
            ],
        ),
        (
            "charter-approve",
            "handbook.use-case.author-charter@1.0.0",
            vec![
                "author".into(),
                "charter".into(),
                "--approve-candidate".into(),
                CANDIDATE.into(),
                "--approval-class".into(),
                "project_owner".into(),
                "--authority-ref".into(),
                "project_owner".into(),
                "--json".into(),
            ],
        ),
        (
            "charter-promote",
            "handbook.use-case.author-charter@1.0.0",
            vec![
                "author".into(),
                "charter".into(),
                "--promote-candidate".into(),
                CANDIDATE.into(),
                "--approval-ref".into(),
                APPROVAL.into(),
                "--json".into(),
            ],
        ),
        (
            "charter-validate",
            "handbook.use-case.author-charter@1.0.0",
            vec![
                "author".into(),
                "charter".into(),
                "--validate".into(),
                "--json".into(),
            ],
        ),
        (
            "project-context-author",
            "handbook.use-case.author-project-context@1.0.0",
            vec![
                "author".into(),
                "project-context".into(),
                "--from-inputs".into(),
                "project-context-input.yaml".into(),
                "--json".into(),
            ],
        ),
        (
            "project-context-validate",
            "handbook.use-case.author-project-context@1.0.0",
            vec![
                "author".into(),
                "project-context".into(),
                "--from-inputs".into(),
                "project-context-input.yaml".into(),
                "--validate".into(),
                "--json".into(),
            ],
        ),
        (
            "approver-bootstrap",
            "handbook.use-case.approver-administration@1.0.0",
            vec![
                "approvers".into(),
                "bootstrap".into(),
                "--initial-charter-quorum".into(),
                "project_owner=project_owner".into(),
                "--json".into(),
            ],
        ),
        (
            "approver-add-credential",
            "handbook.use-case.approver-administration@1.0.0",
            vec![
                "approvers".into(),
                "add-credential".into(),
                "--approval-mapping".into(),
                "project_owner=project_owner".into(),
                "--json".into(),
            ],
        ),
        (
            "approver-revoke-credential",
            "handbook.use-case.approver-administration@1.0.0",
            vec![
                "approvers".into(),
                "revoke-credential".into(),
                "--credential-id-hash".into(),
                HASH.into(),
                "--json".into(),
            ],
        ),
        (
            "approver-update-mapping",
            "handbook.use-case.approver-administration@1.0.0",
            vec![
                "approvers".into(),
                "update-mapping".into(),
                "--credential-id-hash".into(),
                HASH.into(),
                "--approval-mapping".into(),
                "project_owner=project_owner".into(),
                "--json".into(),
            ],
        ),
    ];

    for (case, use_case_ref, args) in cases {
        let repo = tempfile::tempdir().unwrap();
        initialize_identity(repo.path());
        fs::write(
            repo.path().join("charter-input.yaml"),
            "mode: guided_adaptive\ncontent: {}\ncoverage: []\nconsumer:\n  kind: agent\n  id: packet-three\n  version: '1.0'\nprompt_event_refs: []\nfinalized_at_utc: '2026-08-16T00:00:00Z'\nexpected_current_fingerprint: null\n",
        )
        .unwrap();
        fs::write(
            repo.path().join("project-context-input.yaml"),
            "schema_id: handbook.artifact.project-context\nschema_version: '1.0'\nrecord_id: handbook.project-context\nsummary: ''\nsystem_boundaries: []\nownership: []\nauthoritative_references: []\nknown_unknowns: []\n",
        )
        .unwrap();
        let borrowed = args.iter().map(String::as_str).collect::<Vec<_>>();
        let output = run(repo.path(), &borrowed);
        assert_eq!(output.status.code(), Some(1), "{case}");
        assert!(output.stderr.is_empty(), "{case}: {:?}", output.stderr);
        let document = value(&output);
        assert_eq!(document["response_kind"], "use_case", "{case}");
        assert_eq!(
            document["response"]["use_case"]["ref"], use_case_ref,
            "{case}"
        );
    }
}

#[test]
fn approver_legacy_json_keeps_the_pre_membrane_repository_failure_projection() {
    let repo = tempfile::tempdir().unwrap();
    let output = run(
        repo.path(),
        &[
            "approvers",
            "bootstrap",
            "--initial-charter-quorum",
            "project_owner=project_owner",
            "--legacy-json",
        ],
    );
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stderr.is_empty());
    assert!(output.stdout.ends_with(b"\n"));
    let document: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert!(document.get("response_kind").is_none());
    assert_eq!(
        document["schema_id"],
        "handbook.repository-invocation-preflight-result"
    );
    assert_eq!(document["schema_version"], "1.0");
    assert_eq!(document["operation"], "bootstrap");
    assert_eq!(document["status"], "refused");
    assert_eq!(document["changed_paths"], serde_json::json!([]));
    assert_eq!(
        document["refusal"]["code"],
        "repository_identity_unavailable"
    );
}
