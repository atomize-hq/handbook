use handbook_engine::RepositoryInvocationIdentityServiceV1;
use handbook_sdk::contract_membrane::{
    ApproverAdministrationRequestV1, AuthorCharterRequestV1, AuthorProjectContextRequestV1,
    ContractLimits, InvocationRequestV1, InvocationResponseV1,
    PipelineRouteResolveAndPersistRequestV1, ProjectContextInputV1, RepositorySetupModeV1,
    RepositorySetupRequestV1, UseCaseRequestV1, UseCaseResponseV1,
};
use handbook_sdk::{
    ApproverCommandRequest, CharterCommandRequest, HandbookSdkV1, ProjectContextCommandRequest,
    ProjectContextInput, SetupMode, SetupRequest,
};
use std::path::Path;
use std::process::{Command, Output};

const CREDENTIAL_ID_HASH: &str =
    "sha256:2222222222222222222222222222222222222222222222222222222222222222";
const PROJECT_CONTEXT_INPUT: &str = "project-context-inputs.yaml";
const PROJECT_CONTEXT_INPUT_YAML: &str = r#"schema_id: "handbook.artifact.project-context"
schema_version: "1.0"
record_id: "handbook.project-context"
summary: "Packet 4 CLI parity"
system_boundaries:
  - "SDK facade"
ownership:
  - "Project owner"
authoritative_references: []
known_unknowns:
  - "None"
"#;

fn close_use_case(response: UseCaseResponseV1) -> InvocationResponseV1 {
    InvocationResponseV1::use_case(response, &ContractLimits::packet0()).unwrap()
}

fn run_in(root: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_handbook"))
        .current_dir(root)
        .args(args)
        .output()
        .unwrap_or_else(|error| panic!("run `{}`: {error}", args.join(" ")))
}

fn assert_cli_parity(
    root: &Path,
    args: &[&str],
    expected: InvocationResponseV1,
    use_case_ref: &str,
) {
    let output = run_in(root, args);
    assert!(
        output.stderr.is_empty(),
        "{use_case_ref}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(output.stdout.ends_with(b"\n"), "{use_case_ref}");
    assert_eq!(
        output.stdout.iter().filter(|byte| **byte == b'\n').count(),
        1,
        "{use_case_ref}"
    );
    let actual: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(actual["response_kind"], "use_case", "{use_case_ref}");
    assert_eq!(
        actual["response"]["use_case"]["ref"], use_case_ref,
        "{use_case_ref}"
    );
    assert_eq!(
        actual,
        serde_json::to_value(expected).unwrap(),
        "{use_case_ref}"
    );

    let schema: serde_json::Value = serde_json::from_slice(include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../sdk/schemas/2020-12/handbook.invocation-response/1.0.0.schema.json"
    )))
    .unwrap();
    let validator = jsonschema::draft202012::options().build(&schema).unwrap();
    assert!(validator.is_valid(&actual), "{use_case_ref}");
}

fn copy_tree(source: &Path, target: &Path) {
    std::fs::create_dir_all(target).unwrap();
    for entry in std::fs::read_dir(source).unwrap() {
        let entry = entry.unwrap();
        let destination = target.join(entry.file_name());
        if entry.file_type().unwrap().is_dir() {
            copy_tree(&entry.path(), &destination);
        } else {
            std::fs::copy(entry.path(), destination).unwrap();
        }
    }
}

fn equivalent_initialized_repositories() -> (tempfile::TempDir, tempfile::TempDir) {
    let seed = tempfile::tempdir().unwrap();
    std::fs::create_dir(seed.path().join(".git")).unwrap();
    RepositoryInvocationIdentityServiceV1::new()
        .initialize_for_setup(seed.path())
        .unwrap();
    let direct = tempfile::tempdir().unwrap();
    let cli = tempfile::tempdir().unwrap();
    copy_tree(seed.path(), direct.path());
    copy_tree(seed.path(), cli.path());
    (direct, cli)
}

fn project_context_input() -> ProjectContextInput {
    ProjectContextInput {
        schema_id: "handbook.artifact.project-context".to_owned(),
        schema_version: "1.0".to_owned(),
        record_id: "handbook.project-context".to_owned(),
        summary: "Packet 4 CLI parity".to_owned(),
        system_boundaries: vec!["SDK facade".to_owned()],
        ownership: vec!["Project owner".to_owned()],
        authoritative_references: Vec::new(),
        known_unknowns: vec!["None".to_owned()],
    }
}

fn project_context_input_v1() -> ProjectContextInputV1 {
    ProjectContextInputV1::new(
        "handbook.artifact.project-context",
        "1.0",
        "handbook.project-context",
        "Packet 4 CLI parity",
        vec!["SDK facade".to_owned()],
        vec!["Project owner".to_owned()],
        Vec::new(),
        vec!["None".to_owned()],
    )
    .unwrap()
}

#[test]
fn existing_cli_json_is_semantically_identical_to_the_six_fixed_sdk_use_cases() {
    let limits = ContractLimits::packet0();

    let (direct, cli) = equivalent_initialized_repositories();
    let expected = close_use_case(
        HandbookSdkV1::open(direct.path())
            .run_setup_use_case(
                &SetupRequest {
                    mode: SetupMode::Refresh,
                    rewrite: false,
                    reset_state: false,
                },
                &limits,
            )
            .unwrap(),
    );
    assert_cli_parity(
        cli.path(),
        &["setup", "refresh", "--json"],
        expected,
        "handbook.use-case.repository-setup@1.0.0",
    );

    let direct = tempfile::tempdir().unwrap();
    let cli = tempfile::tempdir().unwrap();
    let expected = close_use_case(match HandbookSdkV1::open(direct.path()).doctor() {
        Ok(report) => UseCaseResponseV1::repository_doctor(&report, &limits).unwrap(),
        Err(error) => UseCaseResponseV1::repository_doctor_error(error, &limits).unwrap(),
    });
    assert_cli_parity(
        cli.path(),
        &["doctor", "--json"],
        expected,
        "handbook.use-case.repository-doctor@1.0.0",
    );

    let direct = tempfile::tempdir().unwrap();
    let cli = tempfile::tempdir().unwrap();
    let expected = HandbookSdkV1::open(direct.path())
        .run_author_charter_use_case(CharterCommandRequest::Validate, &limits)
        .unwrap();
    assert_cli_parity(
        cli.path(),
        &["author", "charter", "--validate", "--json"],
        expected,
        "handbook.use-case.author-charter@1.0.0",
    );

    let direct = tempfile::tempdir().unwrap();
    let cli = tempfile::tempdir().unwrap();
    std::fs::write(
        cli.path().join(PROJECT_CONTEXT_INPUT),
        PROJECT_CONTEXT_INPUT_YAML,
    )
    .unwrap();
    let expected = close_use_case(
        HandbookSdkV1::open(direct.path())
            .run_author_project_context_use_case(
                "structured_inputs_file",
                PROJECT_CONTEXT_INPUT,
                ProjectContextCommandRequest::Validate(project_context_input()),
                &limits,
            )
            .unwrap(),
    );
    assert_cli_parity(
        cli.path(),
        &[
            "author",
            "project-context",
            "--from-inputs",
            PROJECT_CONTEXT_INPUT,
            "--validate",
            "--json",
        ],
        expected,
        "handbook.use-case.author-project-context@1.0.0",
    );

    let (direct, cli) = equivalent_initialized_repositories();
    let expected = HandbookSdkV1::open(direct.path())
        .run_approver_administration_use_case(
            ApproverCommandRequest::RevokeCredential {
                credential_id_hash: CREDENTIAL_ID_HASH.to_owned(),
            },
            &limits,
        )
        .unwrap();
    assert_cli_parity(
        cli.path(),
        &[
            "approvers",
            "revoke-credential",
            "--credential-id-hash",
            CREDENTIAL_ID_HASH,
            "--json",
        ],
        expected,
        "handbook.use-case.approver-administration@1.0.0",
    );

    let direct = tempfile::tempdir().unwrap();
    let cli = tempfile::tempdir().unwrap();
    let expected = close_use_case(
        HandbookSdkV1::open(direct.path())
            .resolve_pipeline_use_case("packet-four-missing", &limits)
            .unwrap(),
    );
    assert_cli_parity(
        cli.path(),
        &[
            "--json",
            "pipeline",
            "resolve",
            "--id",
            "packet-four-missing",
        ],
        expected,
        "handbook.use-case.pipeline-route-resolve-and-persist@1.0.0",
    );

    let request_refs = [
        UseCaseRequestV1::repository_setup(
            RepositorySetupRequestV1::new(RepositorySetupModeV1::Refresh, false, false),
            &limits,
        )
        .unwrap(),
        UseCaseRequestV1::repository_doctor(&limits).unwrap(),
        UseCaseRequestV1::author_charter(AuthorCharterRequestV1::Validate, &limits).unwrap(),
        UseCaseRequestV1::author_project_context(
            AuthorProjectContextRequestV1::Validate(project_context_input_v1()),
            &limits,
        )
        .unwrap(),
        UseCaseRequestV1::approver_administration(
            ApproverAdministrationRequestV1::revoke_credential(CREDENTIAL_ID_HASH).unwrap(),
            &limits,
        )
        .unwrap(),
        UseCaseRequestV1::pipeline_route_resolve_and_persist(
            PipelineRouteResolveAndPersistRequestV1::new("packet-four-missing").unwrap(),
            &limits,
        )
        .unwrap(),
    ]
    .map(|request| InvocationRequestV1::use_case(request, &limits).unwrap());
    assert_eq!(request_refs.len(), 6);
}
