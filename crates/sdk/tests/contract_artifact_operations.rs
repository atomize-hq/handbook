use handbook_sdk::artifact::{
    ArtifactInputDocument, ArtifactSdk, ArtifactSelector, ArtifactValidateRequest,
};
use handbook_sdk::contract_membrane::{
    admit_operation_definition, artifact_operation_definitions, artifact_operation_request,
    canonical_json_bytes, operation_inventory, parse_canonical_json,
    render_artifact_operation_legacy, AdmissionReason, ArtifactOperationInputV1, ContractLimits,
    ContractValue, DiscoveryState, OperationAdmissionResult, OperationTransport, TerminalStatus,
};

const ARTIFACT_OPERATION_REFS: [&str; 10] = [
    "artifact.kind.list@1.0.0",
    "artifact.instance.list@1.0.0",
    "artifact.read@1.0.0",
    "artifact.validate@1.0.0",
    "intake.definition.read@1.0.0",
    "intake.coverage.evaluate@1.0.0",
    "intake.record.append@1.0.0",
    "artifact.candidate.validate@1.0.0",
    "artifact.candidate.append@1.0.0",
    "artifact.candidate.promote@1.0.0",
];

#[test]
fn ten_artifact_definitions_are_exact_independently_admitted_and_cli_only() {
    let definitions = artifact_operation_definitions().unwrap();
    assert_eq!(definitions.len(), ARTIFACT_OPERATION_REFS.len());

    for (definition, expected_ref) in definitions.iter().zip(ARTIFACT_OPERATION_REFS) {
        assert_eq!(definition.operation().as_str(), expected_ref);
        assert!(matches!(
            admit_operation_definition(definition).unwrap(),
            OperationAdmissionResult::Admitted(_)
        ));

        let row = operation_inventory()
            .iter()
            .find(|row| row.operation_ref().as_str() == expected_ref)
            .unwrap();
        assert_eq!(row.admission_reason(), AdmissionReason::Admitted);
        assert_eq!(row.discovery(), DiscoveryState::Discoverable);
        let expected: &[OperationTransport] =
            &[OperationTransport::CliJson, OperationTransport::Tauri];
        assert_eq!(row.current_transports(), expected);
    }

    let admitted = operation_inventory()
        .iter()
        .filter(|row| row.admission_reason() == AdmissionReason::Admitted)
        .collect::<Vec<_>>();
    assert_eq!(admitted.len(), 28);
    let posture = admitted
        .iter()
        .find(|row| row.operation_ref().as_str() == "posture.transition.apply@1.0.0")
        .unwrap();
    assert_eq!(
        posture.current_transports(),
        &[OperationTransport::RustSdk, OperationTransport::Tauri]
    );
}

fn copy_tree(source: &std::path::Path, target: &std::path::Path) {
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

#[test]
fn kind_list_executes_once_into_one_typed_ordinary_response() {
    let repo = tempfile::tempdir().unwrap();
    copy_tree(
        &std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../engine/tests/fixtures/hcm_2_3_generic_custom_kind"),
        repo.path(),
    );
    handbook_engine::RepositoryInvocationIdentityServiceV1::new()
        .initialize_for_setup(repo.path())
        .unwrap();

    let request = artifact_operation_request(None, ArtifactOperationInputV1::KindList).unwrap();
    let response = ArtifactSdk::open(repo.path())
        .execute_operation(&request)
        .unwrap();

    assert_eq!(response.terminal().status(), TerminalStatus::Ok);
    let data = response.terminal().data().unwrap();
    let bytes = canonical_json_bytes(data, &ContractLimits::packet0()).unwrap();
    let value = parse_canonical_json::<ContractValue>(&bytes, &ContractLimits::packet0()).unwrap();
    assert_eq!(value, data.clone());
    assert!(response.receipts().is_empty());
}

#[test]
fn request_admission_enforces_exact_schema_constants_patterns_and_bounds() {
    let invalid_absent = artifact_operation_request(
        None,
        ArtifactOperationInputV1::CandidateValidate {
            kind_ref: "example.artifact-kind.registry-brief@1.0.0".to_owned(),
            instance_id: "registry_brief".to_owned(),
            intake_record_ref: ".handbook/state/artifacts/registry_brief/intake-records/intake_0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef.json".to_owned(),
            intake_record_fingerprint:
                "sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
                    .to_owned(),
            expected_current_artifact_fingerprint: Some("absent".to_owned()),
        },
    );
    assert!(invalid_absent.is_err());

    let empty_target = artifact_operation_request(
        None,
        ArtifactOperationInputV1::Read {
            kind_ref: String::new(),
            instance_id: String::new(),
        },
    );
    assert!(empty_target.is_err());
}

#[test]
fn all_ten_operations_execute_directly_through_their_exact_sdk_requests() {
    let repo = tempfile::tempdir().unwrap();
    copy_tree(
        &std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../engine/tests/fixtures/hcm_2_3_generic_custom_kind"),
        repo.path(),
    );
    handbook_engine::RepositoryInvocationIdentityServiceV1::new()
        .initialize_for_setup(repo.path())
        .unwrap();
    let kind_ref = "example.artifact-kind.registry-brief@1.0.0";
    let instance_id = "registry_brief";
    let execute = |input| {
        let request = artifact_operation_request(None, input).unwrap();
        let response = ArtifactSdk::open(repo.path())
            .execute_operation(&request)
            .unwrap();
        assert_eq!(response.terminal().status(), TerminalStatus::Ok);
        response
    };
    let data = |response: &handbook_sdk::contract_membrane::OperationResponseV1| {
        let bytes = canonical_json_bytes(
            response.terminal().data().unwrap(),
            &ContractLimits::packet0(),
        )
        .unwrap();
        serde_json::from_slice::<serde_json::Value>(&bytes).unwrap()
    };
    let mut observed = Vec::new();
    let mut run = |input| {
        let response = execute(input);
        observed.push(response.operation().reference().as_str().to_owned());
        response
    };

    run(ArtifactOperationInputV1::KindList);
    run(ArtifactOperationInputV1::InstanceList);
    run(ArtifactOperationInputV1::Read {
        kind_ref: kind_ref.to_owned(),
        instance_id: instance_id.to_owned(),
    });
    let validation = run(ArtifactOperationInputV1::Validate {
        kind_ref: kind_ref.to_owned(),
        instance_id: instance_id.to_owned(),
    });
    let current = data(&validation)["artifact_fingerprint"]
        .as_str()
        .unwrap()
        .to_owned();
    run(ArtifactOperationInputV1::IntakeDefinitionRead {
        kind_ref: kind_ref.to_owned(),
        instance_id: instance_id.to_owned(),
    });
    let coverage_submissions = serde_json::json!([
        {
            "coverage_id": "registry_brief.title",
            "state": "supplied",
            "source_kind": "user_declaration",
            "value": "Direct SDK Brief",
            "specificity": "exact",
            "confidence": "high",
            "contradiction_refs": []
        },
        {
            "coverage_id": "registry_brief.summary",
            "state": "supplied",
            "source_kind": "user_declaration",
            "value": "All ten ordinary operations",
            "specificity": "concrete",
            "confidence": "high",
            "contradiction_refs": []
        }
    ]);
    let document = |value: serde_json::Value| {
        ArtifactInputDocument::new(serde_json::to_vec(&value).unwrap()).unwrap()
    };
    run(ArtifactOperationInputV1::IntakeCoverageEvaluate {
        kind_ref: kind_ref.to_owned(),
        instance_id: instance_id.to_owned(),
        mode: handbook_sdk::artifact::ArtifactAcquisitionMode::Express,
        expected_current_artifact_fingerprint: Some(current.clone()),
        input: document(serde_json::json!({
            "coverage_submissions": coverage_submissions.clone()
        })),
    });
    let intake = run(ArtifactOperationInputV1::IntakeRecordAppend {
        kind_ref: kind_ref.to_owned(),
        instance_id: instance_id.to_owned(),
        input: document(serde_json::json!({
            "idempotency_key": "direct_sdk_intake_000001",
            "acquisition_mode": "express",
            "expected_current_artifact_fingerprint": current.clone(),
            "coverage_submissions": coverage_submissions
        })),
    });
    let intake_data = data(&intake);
    let intake_ref = intake_data["authoritative_outputs"][0]["ref"]
        .as_str()
        .unwrap()
        .to_owned();
    let intake_fingerprint = intake_data["authoritative_outputs"][0]["fingerprint"]
        .as_str()
        .unwrap()
        .to_owned();
    let candidate = run(ArtifactOperationInputV1::CandidateValidate {
        kind_ref: kind_ref.to_owned(),
        instance_id: instance_id.to_owned(),
        intake_record_ref: intake_ref.clone(),
        intake_record_fingerprint: intake_fingerprint.clone(),
        expected_current_artifact_fingerprint: Some(current.clone()),
    });
    let candidate_data = data(&candidate);
    let candidate_append = run(ArtifactOperationInputV1::CandidateAppend {
        kind_ref: kind_ref.to_owned(),
        instance_id: instance_id.to_owned(),
        input: document(serde_json::json!({
            "idempotency_key": "direct_sdk_candidate_00001",
            "intake_record_ref": intake_ref,
            "intake_record_fingerprint": intake_fingerprint,
            "expected_candidate_fingerprint": candidate_data["candidate_fingerprint"]
        })),
    });
    let candidate_append_data = data(&candidate_append);
    let promotion = run(ArtifactOperationInputV1::CandidatePromote {
        kind_ref: kind_ref.to_owned(),
        instance_id: instance_id.to_owned(),
        input: document(serde_json::json!({
            "idempotency_key": "direct_sdk_promotion_0001",
            "candidate_ref": candidate_append_data["authoritative_outputs"][0]["ref"],
            "candidate_fingerprint": candidate_append_data["authoritative_outputs"][0]["fingerprint"],
            "expected_current_artifact_fingerprint": current
        })),
    });
    assert_eq!(promotion.receipts().len(), 2);

    assert_eq!(
        observed,
        ARTIFACT_OPERATION_REFS
            .into_iter()
            .map(str::to_owned)
            .collect::<Vec<_>>()
    );
    for response in [intake, candidate_append, promotion] {
        let bytes = canonical_json_bytes(&response, &ContractLimits::packet0()).unwrap();
        let text = String::from_utf8(bytes).unwrap();
        assert!(!text.contains("direct_sdk_"));
        assert!(!text.contains("\"subordinate_outputs\":"));
        assert!(!text.contains("\"staged_outputs\":"));
        assert!(!text.contains("\"marker_fingerprint\":"));
    }
}

#[test]
fn intake_append_projects_one_public_receipt_and_replay_preserves_semantics() {
    let repo = tempfile::tempdir().unwrap();
    copy_tree(
        &std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../engine/tests/fixtures/hcm_2_3_generic_custom_kind"),
        repo.path(),
    );
    handbook_engine::RepositoryInvocationIdentityServiceV1::new()
        .initialize_for_setup(repo.path())
        .unwrap();
    let kind_ref = "example.artifact-kind.registry-brief@1.0.0";
    let instance_id = "registry_brief";
    let current = ArtifactSdk::open(repo.path())
        .validate(ArtifactValidateRequest {
            selector: ArtifactSelector::new(kind_ref, instance_id),
        })
        .unwrap()
        .artifact_fingerprint;
    let input = || {
        ArtifactInputDocument::new(
            serde_json::to_vec(&serde_json::json!({
                "idempotency_key": "contract_intake_00000001",
                "acquisition_mode": "express",
                "expected_current_artifact_fingerprint": current,
                "coverage_submissions": [
                    {
                        "coverage_id": "registry_brief.title",
                        "state": "supplied",
                        "source_kind": "user_declaration",
                        "value": "Contract Brief",
                        "specificity": "exact",
                        "confidence": "high",
                        "contradiction_refs": []
                    },
                    {
                        "coverage_id": "registry_brief.summary",
                        "state": "supplied",
                        "source_kind": "user_declaration",
                        "value": "Typed mutation proof",
                        "specificity": "concrete",
                        "confidence": "high",
                        "contradiction_refs": []
                    }
                ]
            }))
            .unwrap(),
        )
        .unwrap()
    };
    let execute = || {
        let request = artifact_operation_request(
            None,
            ArtifactOperationInputV1::IntakeRecordAppend {
                kind_ref: kind_ref.to_owned(),
                instance_id: instance_id.to_owned(),
                input: input(),
            },
        )
        .unwrap();
        ArtifactSdk::open(repo.path())
            .execute_operation(&request)
            .unwrap()
    };
    let committed = execute();
    assert_eq!(committed.terminal().status(), TerminalStatus::Ok);
    assert_eq!(committed.receipts().len(), 1);
    assert_eq!(
        committed.receipts()[0].record_kind().as_str(),
        "intake_record"
    );
    assert_eq!(
        committed.receipts()[0].record().reference().as_str(),
        "handbook.artifact-record.intake-record@1.0.0"
    );
    let legacy = render_artifact_operation_legacy(&committed, false).unwrap();
    assert_eq!(legacy.last(), Some(&b'\n'));
    let legacy_value: serde_json::Value =
        serde_json::from_slice(&legacy[..legacy.len() - 1]).unwrap();
    let mut expected_legacy = serde_json::to_vec(&legacy_value).unwrap();
    expected_legacy.push(b'\n');
    assert_eq!(legacy, expected_legacy);
    let human = render_artifact_operation_legacy(&committed, true).unwrap();
    let mut expected_human = serde_json::to_vec_pretty(&legacy_value).unwrap();
    expected_human.push(b'\n');
    assert_eq!(human, expected_human);
    let replayed = execute();
    assert_eq!(replayed.terminal().status(), TerminalStatus::Ok);
    assert_eq!(replayed.receipts(), committed.receipts());
    assert_eq!(replayed.terminal().data(), committed.terminal().data());
}
