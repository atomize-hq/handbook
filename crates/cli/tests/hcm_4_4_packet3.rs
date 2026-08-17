use handbook_sdk::artifact::{ArtifactAcquisitionMode, ArtifactInputDocument};
use handbook_sdk::contract_membrane::{
    artifact_operation_request, flow_resolve_operation_request,
    pipeline_capture_plan_operation_request, pipeline_catalog_list_operation_request,
    pipeline_catalog_read_operation_request, pipeline_compile_operation_request,
    ArtifactOperationInputV1, ContractLimits, InvocationRequestV1, OperationRequestV1,
};
use handbook_sdk::HandbookSdkV1;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Output, Stdio};

const KIND_REF: &str = "example.artifact-kind.registry-brief@1.0.0";
const INSTANCE_ID: &str = "registry_brief";
const INTAKE_REF: &str = ".handbook/state/artifacts/registry_brief/intake-records/intake_0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef.json";
const FINGERPRINT: &str = "sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

fn run_in(root: &Path, args: &[String], stdin: Option<&[u8]>) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_handbook"));
    command.current_dir(root).args(args);
    if let Some(input) = stdin {
        let mut child = command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        child.stdin.as_mut().unwrap().write_all(input).unwrap();
        child.wait_with_output().unwrap()
    } else {
        command.output().unwrap()
    }
}

fn assert_cli_parity(
    root: &Path,
    args: Vec<String>,
    stdin: Option<&[u8]>,
    request: OperationRequestV1,
) {
    let operation_ref = request.operation().reference().as_str().to_owned();
    let limits = ContractLimits::packet0();
    let invocation = InvocationRequestV1::operation(request, &limits).unwrap();
    let expected = HandbookSdkV1::open(root)
        .invoke(&invocation, &limits)
        .unwrap();
    let output = run_in(root, &args, stdin);
    assert!(
        output.stderr.is_empty(),
        "{operation_ref}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(output.stdout.ends_with(b"\n"), "{operation_ref}");
    assert_eq!(
        output.stdout.iter().filter(|byte| **byte == b'\n').count(),
        1,
        "{operation_ref}"
    );
    let actual: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    let expected = serde_json::to_value(expected).unwrap();
    assert_eq!(actual, expected, "{operation_ref}");

    let schema: serde_json::Value = serde_json::from_slice(include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../sdk/schemas/2020-12/handbook.invocation-response/1.0.0.schema.json"
    )))
    .unwrap();
    let validator = jsonschema::draft202012::options().build(&schema).unwrap();
    assert!(validator.is_valid(&actual), "{operation_ref}");
}

fn strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

#[test]
fn existing_cli_json_is_semantically_identical_to_the_sdk_dispatcher_for_all_twelve_rows() {
    let repo = tempfile::tempdir().unwrap();
    let root = repo.path();
    let root_arg = root.to_str().unwrap();
    let coverage = serde_json::json!({
        "coverage_submissions": [
            {
                "coverage_id": "registry_brief.title",
                "state": "supplied",
                "source_kind": "user_declaration",
                "value": "Packet 3 parity",
                "specificity": "exact",
                "confidence": "high",
                "contradiction_refs": []
            },
            {
                "coverage_id": "registry_brief.summary",
                "state": "supplied",
                "source_kind": "user_declaration",
                "value": "Read-only invocation dispatcher",
                "specificity": "concrete",
                "confidence": "high",
                "contradiction_refs": []
            }
        ]
    });
    let coverage_bytes = serde_json::to_vec(&coverage).unwrap();
    let coverage_path = root.join("coverage.json");
    std::fs::write(&coverage_path, &coverage_bytes).unwrap();

    for (command, request) in [
        ("list-kinds", ArtifactOperationInputV1::KindList),
        ("list-instances", ArtifactOperationInputV1::InstanceList),
    ] {
        assert_cli_parity(
            root,
            strings(&["--json", "artifact", command, "--repository-root", root_arg]),
            None,
            artifact_operation_request(None, request).unwrap(),
        );
    }
    for (command, request) in [
        (
            "read",
            ArtifactOperationInputV1::Read {
                kind_ref: KIND_REF.to_owned(),
                instance_id: INSTANCE_ID.to_owned(),
            },
        ),
        (
            "validate",
            ArtifactOperationInputV1::Validate {
                kind_ref: KIND_REF.to_owned(),
                instance_id: INSTANCE_ID.to_owned(),
            },
        ),
        (
            "intake-definition",
            ArtifactOperationInputV1::IntakeDefinitionRead {
                kind_ref: KIND_REF.to_owned(),
                instance_id: INSTANCE_ID.to_owned(),
            },
        ),
    ] {
        assert_cli_parity(
            root,
            strings(&[
                "--json",
                "artifact",
                command,
                "--repository-root",
                root_arg,
                "--kind-ref",
                KIND_REF,
                "--instance-id",
                INSTANCE_ID,
            ]),
            None,
            artifact_operation_request(None, request).unwrap(),
        );
    }
    assert_cli_parity(
        root,
        strings(&[
            "--json",
            "artifact",
            "intake-evaluate",
            "--repository-root",
            root_arg,
            "--kind-ref",
            KIND_REF,
            "--instance-id",
            INSTANCE_ID,
            "--mode",
            "express",
            "--from-inputs",
            coverage_path.to_str().unwrap(),
        ]),
        None,
        artifact_operation_request(
            None,
            ArtifactOperationInputV1::IntakeCoverageEvaluate {
                kind_ref: KIND_REF.to_owned(),
                instance_id: INSTANCE_ID.to_owned(),
                mode: ArtifactAcquisitionMode::Express,
                expected_current_artifact_fingerprint: None,
                input: ArtifactInputDocument::new(coverage_bytes).unwrap(),
            },
        )
        .unwrap(),
    );
    assert_cli_parity(
        root,
        strings(&[
            "--json",
            "artifact",
            "candidate-validate",
            "--repository-root",
            root_arg,
            "--kind-ref",
            KIND_REF,
            "--instance-id",
            INSTANCE_ID,
            "--intake-record-ref",
            INTAKE_REF,
            "--intake-record-fingerprint",
            FINGERPRINT,
        ]),
        None,
        artifact_operation_request(
            None,
            ArtifactOperationInputV1::CandidateValidate {
                kind_ref: KIND_REF.to_owned(),
                instance_id: INSTANCE_ID.to_owned(),
                intake_record_ref: INTAKE_REF.to_owned(),
                intake_record_fingerprint: FINGERPRINT.to_owned(),
                expected_current_artifact_fingerprint: None,
            },
        )
        .unwrap(),
    );
    assert_cli_parity(
        root,
        strings(&["generate", "--json", "--packet", "planning.packet"]),
        None,
        flow_resolve_operation_request(None, "planning.packet").unwrap(),
    );
    assert_cli_parity(
        root,
        strings(&["--json", "pipeline", "list"]),
        None,
        pipeline_catalog_list_operation_request(None).unwrap(),
    );
    assert_cli_parity(
        root,
        strings(&[
            "--json",
            "pipeline",
            "show",
            "--id",
            "pipeline.foundation_inputs",
        ]),
        None,
        pipeline_catalog_read_operation_request(None, "pipeline.foundation_inputs").unwrap(),
    );
    assert_cli_parity(
        root,
        strings(&[
            "--json",
            "pipeline",
            "compile",
            "--id",
            "pipeline.foundation_inputs",
            "--stage",
            "stage.10_feature_spec",
        ]),
        None,
        pipeline_compile_operation_request(
            None,
            "pipeline.foundation_inputs",
            "stage.10_feature_spec",
        )
        .unwrap(),
    );
    let capture = b"packet-3 preview\n";
    assert_cli_parity(
        root,
        strings(&[
            "--json",
            "pipeline",
            "capture",
            "--id",
            "pipeline.foundation_inputs",
            "--stage",
            "stage.10_feature_spec",
            "--preview",
        ]),
        Some(capture),
        pipeline_capture_plan_operation_request(
            None,
            "pipeline.foundation_inputs",
            "stage.10_feature_spec",
            "packet-3 preview\n",
        )
        .unwrap(),
    );
}
