#[path = "../../sdk/tests/support/pipeline_proof_corpus_support.rs"]
mod pipeline_proof_corpus_support;

use handbook_sdk::artifact::{
    ArtifactInputDocument, ArtifactSdk, ArtifactSelector, ArtifactValidateRequest,
};
use handbook_sdk::contract_membrane::{
    artifact_operation_definition, artifact_operation_request, canonical_json_bytes,
    pipeline_capture_direct_operation_request, pipeline_handoff_emit_operation_request,
    pipeline_operation_definitions, pipeline_state_apply_operation_request, sha256_fingerprint,
    ArtifactOperationInputV1, ContractError, ContractLimits, ContractValue, ExactBinding,
    InvocationRequestV1, InvocationResponseV1, OperationDefinition, OperationRequestV1,
    OperationSchemaValidator, PipelineOperationSchemaValidator,
};
use handbook_sdk::pipeline_route_api::PipelineStateMutation;
use handbook_sdk::HandbookSdkV1;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Output, Stdio};

const KIND_REF: &str = "example.artifact-kind.registry-brief@1.0.0";
const INSTANCE_ID: &str = "registry_brief";
const IDENTITY: &str = "sha256:abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789";

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

fn cli_value(root: &Path, args: Vec<String>, stdin: Option<&[u8]>) -> serde_json::Value {
    let output = run_in(root, &args, stdin);
    assert!(
        output.stderr.is_empty(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(output.stdout.ends_with(b"\n"));
    assert_eq!(
        output.stdout.iter().filter(|byte| **byte == b'\n').count(),
        1
    );
    serde_json::from_slice(&output.stdout).unwrap()
}

fn sdk_value(root: &Path, request: OperationRequestV1) -> serde_json::Value {
    let limits = ContractLimits::packet0();
    let invocation = InvocationRequestV1::operation(request, &limits).unwrap();
    serde_json::to_value(
        HandbookSdkV1::open(root)
            .invoke(&invocation, &limits)
            .unwrap(),
    )
    .unwrap()
}

struct ParitySchemaValidator;

impl OperationSchemaValidator for ParitySchemaValidator {
    fn validate_instance(
        &self,
        _schema: &ExactBinding,
        instance: &ContractValue,
    ) -> Result<(), ContractError> {
        canonical_json_bytes(instance, &ContractLimits::packet0()).map(|_| ())
    }
}

fn ordinary_definition(operation_ref: &str) -> OperationDefinition {
    if operation_ref.starts_with("pipeline.") {
        pipeline_operation_definitions()
            .unwrap()
            .into_iter()
            .find(|definition| definition.operation().as_str() == operation_ref)
            .unwrap()
    } else {
        artifact_operation_definition(operation_ref).unwrap()
    }
}

fn readmit_ordinary_response(document: &serde_json::Value) {
    let operation_ref = document["response"]["operation"]["ref"].as_str().unwrap();
    let definition = ordinary_definition(operation_ref);
    let limits = ContractLimits::packet0();
    let bytes = canonical_json_bytes(document, &limits).unwrap();
    let validator: &dyn OperationSchemaValidator = if operation_ref.starts_with("pipeline.") {
        &PipelineOperationSchemaValidator
    } else {
        &ParitySchemaValidator
    };
    InvocationResponseV1::parse_and_admit(&bytes, Some((&definition, validator)), &limits).unwrap();
}

fn assert_sha256(value: &serde_json::Value, path: &str) {
    let value = value
        .as_str()
        .unwrap_or_else(|| panic!("{path}: not a string"));
    let digest = value
        .strip_prefix("sha256:")
        .unwrap_or_else(|| panic!("{path}: not a sha256 fingerprint"));
    assert_eq!(digest.len(), 64, "{path}");
    assert!(
        digest.bytes().all(|byte| byte.is_ascii_hexdigit()),
        "{path}"
    );
}

fn assert_prefixed_digest(value: &str, prefix: &str, path: &str) {
    let digest = value
        .strip_prefix(prefix)
        .unwrap_or_else(|| panic!("{path}: missing {prefix} prefix"));
    assert_eq!(digest.len(), 64, "{path}");
    assert!(
        digest.bytes().all(|byte| byte.is_ascii_hexdigit()),
        "{path}"
    );
}

fn is_indexed_path(path: &str, prefix: &str, suffix: &str) -> bool {
    path.strip_prefix(prefix)
        .and_then(|rest| rest.strip_suffix(suffix))
        .is_some_and(|index| index.parse::<usize>().is_ok())
}

fn is_double_indexed_path(path: &str, prefix: &str, middle: &str, suffix: &str) -> bool {
    path.strip_prefix(prefix)
        .and_then(|rest| rest.split_once(middle))
        .and_then(|(first, rest)| rest.strip_suffix(suffix).map(|second| (first, second)))
        .is_some_and(|(first, second)| {
            first.parse::<usize>().is_ok() && second.parse::<usize>().is_ok()
        })
}

fn assert_realization_difference(
    path: &str,
    actual: &serde_json::Value,
    expected: &serde_json::Value,
) {
    let fingerprint = matches!(
        path,
        "/response_fingerprint"
            | "/response/response_fingerprint"
            | "/response/request_fingerprint"
            | "/response/result_fingerprint"
            | "/response/idempotency/request_fingerprint"
            | "/response/data/candidate_fingerprint"
            | "/response/data/candidate/candidate_fingerprint"
            | "/response/data/candidate/intake_record_fingerprint"
            | "/response/data/internal_transaction_evidence_fingerprint"
            | "/response/data/intake_record_fingerprint"
            | "/response/data/validation_result/intake_record_fingerprint"
            | "/response/data/validation_result/validation_result_fingerprint"
            | "/response/data/validation_result_fingerprint"
            | "/response/data/request_fingerprint"
            | "/response/data/result_fingerprint"
    ) || is_indexed_path(
        path,
        "/response/data/authoritative_outputs/",
        "/fingerprint",
    ) || is_indexed_path(path, "/response/receipts/", "/receipt_fingerprint")
        || is_indexed_path(path, "/response/receipts/", "/record/fingerprint")
        || is_indexed_path(path, "/response/receipts/", "/condition/fingerprint")
        || is_double_indexed_path(
            path,
            "/response/receipts/",
            "/condition/effects/",
            "/fingerprint",
        );
    if fingerprint {
        assert_sha256(actual, path);
        assert_sha256(expected, path);
        return;
    }

    let reference = path == "/response/data/internal_transaction_evidence_ref"
        || path == "/response/data/candidate_ref"
        || path == "/response/data/intake_record_ref"
        || path == "/response/data/validation_result/intake_record_ref"
        || path == "/response/data/validation_result_ref"
        || path == "/response/data/candidate/intake_record_ref"
        || is_indexed_path(path, "/response/data/candidate/validation_result_refs/", "")
        || is_indexed_path(path, "/response/data/authoritative_outputs/", "/ref")
        || is_indexed_path(path, "/response/receipts/", "/record/ref")
        || is_indexed_path(path, "/response/receipts/", "/condition/ref")
        || is_double_indexed_path(path, "/response/receipts/", "/condition/effects/", "/ref");
    if reference {
        let actual = actual
            .as_str()
            .unwrap_or_else(|| panic!("{path}: not a string"));
        let expected = expected
            .as_str()
            .unwrap_or_else(|| panic!("{path}: not a string"));
        assert!(actual.starts_with(".handbook/"), "{path}: {actual}");
        assert!(expected.starts_with(".handbook/"), "{path}: {expected}");
        assert_eq!(
            Path::new(actual).extension(),
            Path::new(expected).extension(),
            "{path}"
        );
        return;
    }

    if path == "/response/data/candidate/candidate_id" {
        assert_prefixed_digest(actual.as_str().unwrap(), "candidate_", path);
        assert_prefixed_digest(expected.as_str().unwrap(), "candidate_", path);
        return;
    }
    if path == "/response/data/validation_result/validation_result_id" {
        assert_prefixed_digest(actual.as_str().unwrap(), "validation_", path);
        assert_prefixed_digest(expected.as_str().unwrap(), "validation_", path);
        return;
    }

    let transaction_id = path == "/response/data/transaction_id"
        || is_indexed_path(path, "/response/receipts/", "/condition/transaction_id");
    if transaction_id {
        let actual = actual
            .as_str()
            .unwrap_or_else(|| panic!("{path}: not a string"));
        let expected = expected
            .as_str()
            .unwrap_or_else(|| panic!("{path}: not a string"));
        assert!(!actual.is_empty(), "{path}");
        assert_eq!(
            actual.split('_').next(),
            expected.split('_').next(),
            "{path}"
        );
        return;
    }

    panic!("unexpected semantic difference at {path}: actual={actual} expected={expected}");
}

fn assert_semantic_value(actual: &serde_json::Value, expected: &serde_json::Value, path: &str) {
    match (actual, expected) {
        (serde_json::Value::Object(actual), serde_json::Value::Object(expected)) => {
            assert_eq!(
                actual.keys().collect::<Vec<_>>(),
                expected.keys().collect::<Vec<_>>(),
                "{path}"
            );
            for (key, actual) in actual {
                assert_semantic_value(actual, &expected[key], &format!("{path}/{key}"));
            }
        }
        (serde_json::Value::Array(actual), serde_json::Value::Array(expected)) => {
            assert_eq!(actual.len(), expected.len(), "{path}");
            for (index, (actual, expected)) in actual.iter().zip(expected).enumerate() {
                assert_semantic_value(actual, expected, &format!("{path}/{index}"));
            }
        }
        _ if actual == expected => {}
        _ => assert_realization_difference(path, actual, expected),
    }
}

fn assert_semantic_parity(actual: &serde_json::Value, expected: &serde_json::Value) {
    readmit_ordinary_response(actual);
    readmit_ordinary_response(expected);
    assert_semantic_value(actual, expected, "");
}

fn strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
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
    std::fs::write(
        repo.path().join(".handbook/repository-identity.v1"),
        IDENTITY,
    )
    .unwrap();
    repo
}

fn pipeline_cli_key(operation_ref: &str, material: &str) -> String {
    let fingerprint = sha256_fingerprint(
        format!("handbook.pipeline-cli-idempotency-v1\0{operation_ref}\0{material}").as_bytes(),
    );
    format!("cli-{}", fingerprint.as_str().trim_start_matches("sha256:"))
}

#[test]
fn six_cli_mutations_match_sdk_dispatch_on_isolated_equivalent_repositories() {
    let direct_repo = artifact_repo();
    let cli_repo = artifact_repo();
    let direct_root = direct_repo.path();
    let cli_root = cli_repo.path();
    let cli_root_arg = cli_root.to_str().unwrap();
    let current = ArtifactSdk::open(direct_root)
        .validate(ArtifactValidateRequest {
            selector: ArtifactSelector::new(KIND_REF, INSTANCE_ID),
        })
        .unwrap()
        .artifact_fingerprint;
    assert_eq!(
        ArtifactSdk::open(cli_root)
            .validate(ArtifactValidateRequest {
                selector: ArtifactSelector::new(KIND_REF, INSTANCE_ID),
            })
            .unwrap()
            .artifact_fingerprint,
        current
    );
    let coverage = serde_json::json!([
        {
            "coverage_id": "registry_brief.title",
            "state": "supplied",
            "source_kind": "user_declaration",
            "value": "Packet 5 CLI parity",
            "specificity": "exact",
            "confidence": "high",
            "contradiction_refs": []
        },
        {
            "coverage_id": "registry_brief.summary",
            "state": "supplied",
            "source_kind": "user_declaration",
            "value": "Isolated mutation repositories",
            "specificity": "concrete",
            "confidence": "high",
            "contradiction_refs": []
        }
    ]);
    let intake_json = serde_json::json!({
        "idempotency_key": "packet5_cli_intake_0001",
        "acquisition_mode": "express",
        "expected_current_artifact_fingerprint": current,
        "coverage_submissions": coverage
    });
    let intake_bytes = serde_json::to_vec(&intake_json).unwrap();
    let intake_path = cli_root.join("packet5-intake.json");
    std::fs::write(&intake_path, &intake_bytes).unwrap();
    let direct_intake = sdk_value(
        direct_root,
        artifact_operation_request(
            None,
            ArtifactOperationInputV1::IntakeRecordAppend {
                kind_ref: KIND_REF.to_owned(),
                instance_id: INSTANCE_ID.to_owned(),
                input: ArtifactInputDocument::new(intake_bytes).unwrap(),
            },
        )
        .unwrap(),
    );
    let cli_intake = cli_value(
        cli_root,
        strings(&[
            "--json",
            "artifact",
            "intake-append",
            "--repository-root",
            cli_root_arg,
            "--kind-ref",
            KIND_REF,
            "--instance-id",
            INSTANCE_ID,
            "--from-request",
            intake_path.to_str().unwrap(),
        ]),
        None,
    );
    assert_semantic_parity(&cli_intake, &direct_intake);
    let direct_intake_ref = direct_intake["response"]["data"]["authoritative_outputs"][0]["ref"]
        .as_str()
        .unwrap();
    let direct_intake_fingerprint = direct_intake["response"]["data"]["authoritative_outputs"][0]
        ["fingerprint"]
        .as_str()
        .unwrap();
    let cli_intake_ref = cli_intake["response"]["data"]["authoritative_outputs"][0]["ref"]
        .as_str()
        .unwrap();
    let cli_intake_fingerprint = cli_intake["response"]["data"]["authoritative_outputs"][0]
        ["fingerprint"]
        .as_str()
        .unwrap();

    let direct_preview = sdk_value(
        direct_root,
        artifact_operation_request(
            None,
            ArtifactOperationInputV1::CandidateValidate {
                kind_ref: KIND_REF.to_owned(),
                instance_id: INSTANCE_ID.to_owned(),
                intake_record_ref: direct_intake_ref.to_owned(),
                intake_record_fingerprint: direct_intake_fingerprint.to_owned(),
                expected_current_artifact_fingerprint: Some(current.clone()),
            },
        )
        .unwrap(),
    );
    let cli_preview = cli_value(
        cli_root,
        strings(&[
            "--json",
            "artifact",
            "candidate-validate",
            "--repository-root",
            cli_root_arg,
            "--kind-ref",
            KIND_REF,
            "--instance-id",
            INSTANCE_ID,
            "--intake-record-ref",
            cli_intake_ref,
            "--intake-record-fingerprint",
            cli_intake_fingerprint,
            "--expected-current-fingerprint",
            &current,
        ]),
        None,
    );
    assert_semantic_parity(&cli_preview, &direct_preview);
    let direct_candidate_json = serde_json::json!({
        "idempotency_key": "packet5_cli_candidate_0001",
        "intake_record_ref": direct_intake_ref,
        "intake_record_fingerprint": direct_intake_fingerprint,
        "expected_candidate_fingerprint": direct_preview["response"]["data"]["candidate_fingerprint"]
    });
    let cli_candidate_json = serde_json::json!({
        "idempotency_key": "packet5_cli_candidate_0001",
        "intake_record_ref": cli_intake_ref,
        "intake_record_fingerprint": cli_intake_fingerprint,
        "expected_candidate_fingerprint": cli_preview["response"]["data"]["candidate_fingerprint"]
    });
    let direct_candidate_bytes = serde_json::to_vec(&direct_candidate_json).unwrap();
    let cli_candidate_bytes = serde_json::to_vec(&cli_candidate_json).unwrap();
    let candidate_path = cli_root.join("packet5-candidate.json");
    std::fs::write(&candidate_path, &cli_candidate_bytes).unwrap();
    let direct_candidate = sdk_value(
        direct_root,
        artifact_operation_request(
            None,
            ArtifactOperationInputV1::CandidateAppend {
                kind_ref: KIND_REF.to_owned(),
                instance_id: INSTANCE_ID.to_owned(),
                input: ArtifactInputDocument::new(direct_candidate_bytes).unwrap(),
            },
        )
        .unwrap(),
    );
    let cli_candidate = cli_value(
        cli_root,
        strings(&[
            "--json",
            "artifact",
            "candidate-append",
            "--repository-root",
            cli_root_arg,
            "--kind-ref",
            KIND_REF,
            "--instance-id",
            INSTANCE_ID,
            "--from-request",
            candidate_path.to_str().unwrap(),
        ]),
        None,
    );
    assert_semantic_parity(&cli_candidate, &direct_candidate);
    let direct_candidate_ref = direct_candidate["response"]["data"]["authoritative_outputs"][0]
        ["ref"]
        .as_str()
        .unwrap();
    let direct_candidate_fingerprint = direct_candidate["response"]["data"]
        ["authoritative_outputs"][0]["fingerprint"]
        .as_str()
        .unwrap();
    let cli_candidate_ref = cli_candidate["response"]["data"]["authoritative_outputs"][0]["ref"]
        .as_str()
        .unwrap();
    let cli_candidate_fingerprint = cli_candidate["response"]["data"]["authoritative_outputs"][0]
        ["fingerprint"]
        .as_str()
        .unwrap();
    let direct_promote_json = serde_json::json!({
        "idempotency_key": "packet5_cli_promote_0001",
        "candidate_ref": direct_candidate_ref,
        "candidate_fingerprint": direct_candidate_fingerprint,
        "expected_current_artifact_fingerprint": current
    });
    let cli_promote_json = serde_json::json!({
        "idempotency_key": "packet5_cli_promote_0001",
        "candidate_ref": cli_candidate_ref,
        "candidate_fingerprint": cli_candidate_fingerprint,
        "expected_current_artifact_fingerprint": current
    });
    let direct_promote_bytes = serde_json::to_vec(&direct_promote_json).unwrap();
    let cli_promote_bytes = serde_json::to_vec(&cli_promote_json).unwrap();
    let promote_path = cli_root.join("packet5-promote.json");
    std::fs::write(&promote_path, &cli_promote_bytes).unwrap();
    let direct_promote = sdk_value(
        direct_root,
        artifact_operation_request(
            None,
            ArtifactOperationInputV1::CandidatePromote {
                kind_ref: KIND_REF.to_owned(),
                instance_id: INSTANCE_ID.to_owned(),
                input: ArtifactInputDocument::new(direct_promote_bytes).unwrap(),
            },
        )
        .unwrap(),
    );
    let cli_promote = cli_value(
        cli_root,
        strings(&[
            "--json",
            "artifact",
            "promote",
            "--repository-root",
            cli_root_arg,
            "--kind-ref",
            KIND_REF,
            "--instance-id",
            INSTANCE_ID,
            "--from-request",
            promote_path.to_str().unwrap(),
        ]),
        None,
    );
    assert_semantic_parity(&cli_promote, &direct_promote);

    let (_direct_capture_dir, direct_capture_root) =
        pipeline_proof_corpus_support::install_stage_04_capture_ready_repo();
    let (_cli_capture_dir, cli_capture_root) =
        pipeline_proof_corpus_support::install_stage_04_capture_ready_repo();
    let capture_input =
        std::fs::read_to_string(direct_capture_root.join("artifacts/charter/CHARTER_INPUTS.yaml"))
            .unwrap();
    assert_eq!(
        std::fs::read_to_string(cli_capture_root.join("artifacts/charter/CHARTER_INPUTS.yaml"))
            .unwrap(),
        capture_input
    );
    let capture_material = format!(
        "{}\0{}\0{}",
        pipeline_proof_corpus_support::FOUNDATION_INPUTS_PIPELINE_ID,
        pipeline_proof_corpus_support::STAGE_04_CHARTER_INPUTS_ID,
        capture_input
    );
    let capture_key = pipeline_cli_key("pipeline.capture.apply@1.0.0", &capture_material);
    let direct_capture = sdk_value(
        &direct_capture_root,
        pipeline_capture_direct_operation_request(
            None,
            pipeline_proof_corpus_support::FOUNDATION_INPUTS_PIPELINE_ID,
            pipeline_proof_corpus_support::STAGE_04_CHARTER_INPUTS_ID,
            &capture_input,
            &capture_key,
        )
        .unwrap(),
    );
    let cli_capture = cli_value(
        &cli_capture_root,
        strings(&[
            "--json",
            "pipeline",
            "capture",
            "--id",
            pipeline_proof_corpus_support::FOUNDATION_INPUTS_PIPELINE_ID,
            "--stage",
            pipeline_proof_corpus_support::STAGE_04_CHARTER_INPUTS_ID,
        ]),
        Some(capture_input.as_bytes()),
    );
    assert_semantic_parity(&cli_capture, &direct_capture);

    let (_direct_handoff_dir, direct_handoff_root) =
        pipeline_proof_corpus_support::install_foundation_inputs_repo();
    let (_cli_handoff_dir, cli_handoff_root) =
        pipeline_proof_corpus_support::install_foundation_inputs_repo();
    let consumer = "feature-slice-decomposer";
    let producer_command = format!(
        "handbook pipeline handoff emit --id {} --consumer {}",
        pipeline_proof_corpus_support::FOUNDATION_INPUTS_PIPELINE_ID,
        consumer
    );
    let version = include_str!("../../../VERSION").trim();
    let handoff_material = format!(
        "{}\0{}\0{}\0{}",
        pipeline_proof_corpus_support::FOUNDATION_INPUTS_PIPELINE_ID,
        consumer,
        producer_command,
        version
    );
    let handoff_key = pipeline_cli_key("pipeline.handoff.emit@1.0.0", &handoff_material);
    let direct_handoff = sdk_value(
        &direct_handoff_root,
        pipeline_handoff_emit_operation_request(
            None,
            pipeline_proof_corpus_support::FOUNDATION_INPUTS_PIPELINE_ID,
            consumer,
            &producer_command,
            version,
            &handoff_key,
        )
        .unwrap(),
    );
    let cli_handoff = cli_value(
        &cli_handoff_root,
        strings(&[
            "--json",
            "pipeline",
            "handoff",
            "emit",
            "--id",
            pipeline_proof_corpus_support::FOUNDATION_INPUTS_PIPELINE_ID,
            "--consumer",
            consumer,
        ]),
        None,
    );
    assert_semantic_parity(&cli_handoff, &direct_handoff);

    let (_direct_state_dir, direct_state_root) =
        pipeline_proof_corpus_support::install_foundation_inputs_repo();
    let (_cli_state_dir, cli_state_root) =
        pipeline_proof_corpus_support::install_foundation_inputs_repo();
    let (definition, supported_variables) =
        pipeline_proof_corpus_support::load_foundation_inputs_definition(&direct_state_root);
    let state = handbook_pipeline::route_state::load_route_state_with_supported_variables(
        &direct_state_root,
        &definition.header.id,
        &supported_variables,
    )
    .unwrap();
    let mutation = PipelineStateMutation::RoutingVariable {
        variable: "needs_project_context".to_owned(),
        value: true,
    };
    let state_material = format!(
        "{}\0{mutation:?}\0{}",
        pipeline_proof_corpus_support::FOUNDATION_INPUTS_PIPELINE_ID,
        state.revision
    );
    let state_key = pipeline_cli_key("pipeline.state.apply@1.0.0", &state_material);
    let direct_state = sdk_value(
        &direct_state_root,
        pipeline_state_apply_operation_request(
            None,
            pipeline_proof_corpus_support::FOUNDATION_INPUTS_PIPELINE_ID,
            &mutation,
            state.revision,
            &state_key,
        )
        .unwrap(),
    );
    let revision = state.revision.to_string();
    let cli_state = cli_value(
        &cli_state_root,
        strings(&[
            "--json",
            "pipeline",
            "state",
            "set",
            "--id",
            pipeline_proof_corpus_support::FOUNDATION_INPUTS_PIPELINE_ID,
            "--var",
            "needs_project_context=true",
            "--expected-revision",
            &revision,
        ]),
        None,
    );
    assert_semantic_parity(&cli_state, &direct_state);
}
