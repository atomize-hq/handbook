#[path = "support/pipeline_proof_corpus_support.rs"]
mod pipeline_proof_corpus_support;

use handbook_sdk::contract_membrane::{
    admit_operation_definition, canonical_json_bytes, operation_inventory, parse_canonical_json,
    AdmissionReason, ContractLimits, ContractValue, DiscoveryState, IdempotencyStateRef,
    InvocationResponseV1, OperationAdmissionResult, OperationSchemaValidator, OperationTransport,
    PipelineOperationSchemaValidator, TerminalStatus, UseCaseResponseV1,
};
use handbook_sdk::pipeline_route_api::{
    PipelineResolveExecution, PipelineResolveExecutionFailure, PipelineRouteBasisPersistenceRefusal,
};

#[test]
fn exactly_three_pipeline_operations_are_independently_admitted_and_discoverable() {
    let definitions = handbook_sdk::contract_membrane::pipeline_operation_definitions().unwrap();
    assert_eq!(definitions.len(), 7);
    assert_eq!(
        definitions
            .iter()
            .map(|definition| definition.operation().as_str())
            .collect::<Vec<_>>(),
        vec![
            "pipeline.catalog.list@1.0.0",
            "pipeline.catalog.read@1.0.0",
            "pipeline.compile@1.0.0",
            "pipeline.capture.plan@1.0.0",
            "pipeline.capture.apply@1.0.0",
            "pipeline.handoff.emit@1.0.0",
            "pipeline.state.apply@1.0.0",
        ]
    );
    for definition in &definitions {
        let operation = definition.operation().as_str();
        let fingerprint = definition.definition_fingerprint().as_str();
        let admission = admit_operation_definition(definition);
        assert!(
            matches!(admission, Ok(OperationAdmissionResult::Admitted(_))),
            "{operation} {fingerprint}: {admission:?}"
        );
    }

    for operation in [
        "pipeline.catalog.list",
        "pipeline.catalog.read",
        "pipeline.compile",
        "pipeline.capture.plan",
        "pipeline.capture.apply",
        "pipeline.handoff.emit",
        "pipeline.state.apply",
    ] {
        let row = operation_inventory()
            .iter()
            .find(|row| row.operation_id() == operation)
            .unwrap();
        assert_eq!(row.admission_reason(), AdmissionReason::Admitted);
        assert_eq!(row.discovery(), DiscoveryState::Discoverable);
        assert_eq!(row.current_transports(), &[OperationTransport::CliJson]);
        let definition = definitions
            .iter()
            .find(|definition| definition.operation().identity() == operation)
            .unwrap();
        assert!(matches!(
            handbook_sdk::contract_membrane::admit_operation_definition(definition).unwrap(),
            handbook_sdk::contract_membrane::OperationAdmissionResult::Admitted(_)
        ));
    }

    let resolve = operation_inventory()
        .iter()
        .find(|row| row.operation_id() == "pipeline.route.resolve")
        .unwrap();
    assert_eq!(
        resolve.admission_reason(),
        AdmissionReason::SemanticMismatch
    );
    assert_eq!(resolve.discovery(), DiscoveryState::Omit);
    assert!(resolve.current_transports().is_empty());

    let _validator = PipelineOperationSchemaValidator;
}

#[test]
fn exact_pipeline_requests_are_closed_and_compile_modes_share_one_operation() {
    let list =
        handbook_sdk::contract_membrane::pipeline_catalog_list_operation_request(None).unwrap();
    assert_eq!(
        list.operation().reference().as_str(),
        "pipeline.catalog.list@1.0.0"
    );

    let read = handbook_sdk::contract_membrane::pipeline_catalog_read_operation_request(
        None,
        "pipeline.foundation_inputs",
    )
    .unwrap();
    assert_eq!(
        read.operation().reference().as_str(),
        "pipeline.catalog.read@1.0.0"
    );

    let compile = handbook_sdk::contract_membrane::pipeline_compile_operation_request(
        None,
        "pipeline.foundation_inputs",
        "stage.10_feature_spec",
    )
    .unwrap();
    assert_eq!(
        compile.operation().reference().as_str(),
        "pipeline.compile@1.0.0"
    );
    assert_eq!(
        compile.body_schema().reference().as_str(),
        "handbook.operation.pipeline-compile-request@1.0.0"
    );

    let limits = ContractLimits::packet0();
    let extra = parse_canonical_json(br#"{"pipeline_selector":"pipeline.foundation_inputs","raw":true,"stage_selector":"stage.10_feature_spec"}"#, &limits).unwrap();
    assert!(PipelineOperationSchemaValidator
        .validate_instance(compile.body_schema(), &extra)
        .is_err());
    assert!(parse_canonical_json::<ContractValue>(br#"{"selector":""}"#, &limits).is_err());

    let direct = handbook_sdk::contract_membrane::pipeline_capture_direct_operation_request(
        None,
        "pipeline.foundation_inputs",
        "stage.10_feature_spec",
        "completed output",
        "direct-key",
    )
    .unwrap();
    let cached = handbook_sdk::contract_membrane::pipeline_capture_apply_operation_request(
        None,
        "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "cached-key",
    )
    .unwrap();
    assert_eq!(direct.operation(), cached.operation());
    assert_eq!(
        direct.operation().reference().as_str(),
        "pipeline.capture.apply@1.0.0"
    );
    assert!(
        !String::from_utf8(canonical_json_bytes(direct.body(), &limits).unwrap())
            .unwrap()
            .contains("idempotency_key")
    );
    assert!(
        !String::from_utf8(canonical_json_bytes(cached.body(), &limits).unwrap())
            .unwrap()
            .contains("idempotency_key")
    );

    let apply_definition = handbook_sdk::contract_membrane::pipeline_operation_definitions()
        .unwrap()
        .into_iter()
        .find(|definition| definition.operation().as_str() == "pipeline.capture.apply@1.0.0")
        .unwrap();
    let smuggled = parse_canonical_json(
        br#"{"capture_id":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","idempotency_key":"k","input":"private plan","mode":"cached","pipeline_selector":null,"stage_selector":null}"#,
        &limits,
    )
    .unwrap();
    assert!(PipelineOperationSchemaValidator
        .validate_instance(apply_definition.request_schema(), &smuggled)
        .is_err());

    assert!(
        handbook_sdk::contract_membrane::pipeline_state_apply_operation_request(
            None,
            "pipeline.foundation_inputs",
            &handbook_sdk::pipeline_route_api::PipelineStateMutation::RoutingVariable {
                variable: "include_product_brief".to_owned(),
                value: true,
            },
            9_007_199_254_740_991,
            "overflow-key",
        )
        .is_err()
    );
}

#[test]
fn resolve_revision_conflict_preserves_exact_compare_and_write_truth() {
    let limits = ContractLimits::packet0();
    let failure = PipelineResolveExecutionFailure::RouteBasisPersistenceRefused(
        PipelineRouteBasisPersistenceRefusal::RevisionConflict {
            expected_revision: 7,
            actual_revision: 8,
        },
    );
    let response = UseCaseResponseV1::pipeline_route_resolve_and_persist_execution(
        &PipelineResolveExecution {
            result: Err(failure.clone()),
            effects: Vec::new(),
        },
        &limits,
    )
    .unwrap();
    assert_eq!(response.status(), TerminalStatus::Blocked);
    assert!(response.receipts().is_empty());

    let invocation = InvocationResponseV1::use_case(response, &limits).unwrap();
    let bytes = canonical_json_bytes(&invocation, &limits).unwrap();
    let value: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(
        value["response"]["blockers"][0]["details"]["cause"],
        "route_basis_revision_conflict"
    );
    assert_eq!(
        value["response"]["blockers"][0]["details"]["expected_revision"],
        7
    );
    assert_eq!(
        value["response"]["blockers"][0]["details"]["actual_revision"],
        8
    );

    let admitted = InvocationResponseV1::parse_and_admit(&bytes, None, &limits).unwrap();
    assert_eq!(
        admitted
            .pipeline_route_resolve_and_persist_outcome(&limits)
            .unwrap(),
        Some(Err(failure))
    );
}

#[test]
fn packet_seven_closes_exactly_four_pipeline_mutation_rows() {
    let definitions = handbook_sdk::contract_membrane::pipeline_operation_definitions().unwrap();
    assert_eq!(definitions.len(), 7);
    assert_eq!(
        definitions
            .iter()
            .map(|definition| definition.operation().as_str())
            .collect::<Vec<_>>(),
        vec![
            "pipeline.catalog.list@1.0.0",
            "pipeline.catalog.read@1.0.0",
            "pipeline.compile@1.0.0",
            "pipeline.capture.plan@1.0.0",
            "pipeline.capture.apply@1.0.0",
            "pipeline.handoff.emit@1.0.0",
            "pipeline.state.apply@1.0.0",
        ]
    );

    for operation in [
        "pipeline.capture.plan",
        "pipeline.capture.apply",
        "pipeline.handoff.emit",
        "pipeline.state.apply",
    ] {
        let row = operation_inventory()
            .iter()
            .find(|row| row.operation_id() == operation)
            .unwrap();
        assert_eq!(row.admission_reason(), AdmissionReason::Admitted);
        assert_eq!(row.discovery(), DiscoveryState::Discoverable);
        assert_eq!(row.current_transports(), &[OperationTransport::CliJson]);
    }

    let route_resolve = operation_inventory()
        .iter()
        .find(|row| row.operation_id() == "pipeline.route.resolve")
        .unwrap();
    assert_eq!(
        route_resolve.admission_reason(),
        AdmissionReason::SemanticMismatch
    );
    assert_eq!(route_resolve.discovery(), DiscoveryState::Omit);
    assert!(route_resolve.current_transports().is_empty());
}

#[test]
fn state_apply_success_replay_and_key_conflict_preserve_exact_owner_truth() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    let (definition, supported_variables) =
        pipeline_proof_corpus_support::load_foundation_inputs_definition(&repo_root);
    let variable = supported_variables.iter().next().unwrap().clone();
    let state = handbook_pipeline::route_state::load_route_state_with_supported_variables(
        &repo_root,
        &definition.header.id,
        &supported_variables,
    )
    .unwrap();
    let mutation = handbook_sdk::pipeline_route_api::PipelineStateMutation::RoutingVariable {
        variable: variable.clone(),
        value: true,
    };
    let request = handbook_sdk::contract_membrane::pipeline_state_apply_operation_request(
        None,
        &definition.header.id,
        &mutation,
        state.revision,
        "state-replay-key",
    )
    .unwrap();
    let sdk = handbook_sdk::HandbookSdkV1::open(&repo_root);

    let applied = sdk.execute_pipeline_operation(&request).unwrap();
    assert_eq!(applied.terminal().status(), TerminalStatus::Ok);
    assert_eq!(applied.receipts().len(), 1);
    assert!(matches!(
        applied.idempotency().as_ref(),
        IdempotencyStateRef::Established {
            replayed: false,
            original_result_fingerprint: None,
            ..
        }
    ));
    let after_applied = handbook_pipeline::route_state::load_route_state_with_supported_variables(
        &repo_root,
        &definition.header.id,
        &supported_variables,
    )
    .unwrap();
    assert_eq!(after_applied.revision, state.revision + 1);

    std::fs::remove_file(repo_root.join("core/pipelines/foundation_inputs.yaml")).unwrap();

    let replayed = sdk.execute_pipeline_operation(&request).unwrap();
    assert!(matches!(
        replayed.idempotency().as_ref(),
        IdempotencyStateRef::Established { replayed: true, .. }
    ));
    replayed.verify_replay_against(&applied).unwrap();
    let after_replay = handbook_pipeline::route_state::load_route_state_with_supported_variables(
        &repo_root,
        &definition.header.id,
        &supported_variables,
    )
    .unwrap();
    assert_eq!(after_replay.revision, after_applied.revision);

    let conflict_request = handbook_sdk::contract_membrane::pipeline_state_apply_operation_request(
        None,
        &definition.header.id,
        &handbook_sdk::pipeline_route_api::PipelineStateMutation::RoutingVariable {
            variable,
            value: false,
        },
        state.revision,
        "state-replay-key",
    )
    .unwrap();
    let conflict = sdk.execute_pipeline_operation(&conflict_request).unwrap();
    assert_eq!(conflict.terminal().status(), TerminalStatus::Refused);
    assert!(conflict.receipts().is_empty());
    let bytes = canonical_json_bytes(&conflict, &ContractLimits::packet0()).unwrap();
    let text = String::from_utf8(bytes).unwrap();
    assert!(text.contains("idempotency_conflict"));
    assert!(!text.contains("state-replay-key"));
    assert!(!text.contains(repo_root.to_string_lossy().as_ref()));
}

#[test]
fn state_apply_refuses_a_preexisting_out_of_domain_revision_before_projection() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    let (definition, supported_variables) =
        pipeline_proof_corpus_support::load_foundation_inputs_definition(&repo_root);
    let variable = supported_variables.iter().next().unwrap().clone();
    let mut state = handbook_pipeline::route_state::load_route_state_with_supported_variables(
        &repo_root,
        &definition.header.id,
        &supported_variables,
    )
    .unwrap();
    state.revision = 9_007_199_254_740_992;
    let state_path =
        handbook_pipeline::route_state::route_state_path(&repo_root, &definition.header.id)
            .unwrap();
    handbook_pipeline::route_state::ensure_state_parent_dir(&state_path).unwrap();
    handbook_pipeline::route_state::persist_route_state(&state_path, &state).unwrap();
    let request = handbook_sdk::contract_membrane::pipeline_state_apply_operation_request(
        None,
        &definition.header.id,
        &handbook_sdk::pipeline_route_api::PipelineStateMutation::RoutingVariable {
            variable,
            value: true,
        },
        0,
        "state-out-of-domain-key",
    )
    .unwrap();

    let response = handbook_sdk::HandbookSdkV1::open(&repo_root)
        .execute_pipeline_operation(&request)
        .unwrap();
    assert_eq!(response.terminal().status(), TerminalStatus::Blocked);
    assert!(response.receipts().is_empty());
    let text =
        String::from_utf8(canonical_json_bytes(&response, &ContractLimits::packet0()).unwrap())
            .unwrap();
    assert!(text.contains("state_malformed"));
    assert!(!text.contains("9007199254740992"));
    assert!(!text.contains("state-out-of-domain-key"));
}

fn stage_seven_capture_input() -> String {
    let mut output = String::new();
    for path in [
        "artifacts/foundation/FOUNDATION_STRATEGY.md",
        "artifacts/foundation/TECH_ARCH_BRIEF.md",
        "artifacts/foundation/TEST_STRATEGY_BRIEF.md",
        "artifacts/foundation/QUALITY_GATES_SPEC.md",
        "artifacts/foundation/quality_gates.yaml",
    ] {
        output.push_str(&format!("--- FILE: {path} ---\n"));
        output.push_str(&pipeline_proof_corpus_support::read_committed_fixture(path));
        if !output.ends_with('\n') {
            output.push('\n');
        }
    }
    output
}

fn install_stage_ten_work_specification_repo() -> (tempfile::TempDir, std::path::PathBuf) {
    let (dir, repo_root) = pipeline_proof_corpus_support::install_stage_10_capture_ready_repo();
    for relative_path in [
        "artifacts/work-specification/work-specification.yaml",
        "artifacts/feature_spec/FEATURE_SPEC.md",
    ] {
        let target = repo_root.join(relative_path);
        if target.exists() {
            std::fs::remove_file(target).unwrap();
        }
    }
    let fixture_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../engine/tests/fixtures/hcm_2_4_work_specification");
    std::fs::copy(
        fixture_root.join(".handbook/profile-selection.json"),
        repo_root.join(".handbook/profile-selection.json"),
    )
    .unwrap();
    let profile_target =
        repo_root.join(".handbook/definitions/profiles/work-specification-root-1.0.0.yaml");
    std::fs::create_dir_all(profile_target.parent().unwrap()).unwrap();
    std::fs::copy(
        fixture_root.join(".handbook/definitions/profiles/work-specification-root-1.0.0.yaml"),
        profile_target,
    )
    .unwrap();
    let source_core = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../core");
    for relative_path in [
        "stages/10_feature_spec.md",
        "library/feature_spec/feature_spec_architect_directive.md",
        "library/feature_spec/FEATURE_SPEC.md.tmpl",
    ] {
        std::fs::copy(
            source_core.join(relative_path),
            repo_root.join("core").join(relative_path),
        )
        .unwrap();
    }
    let _ = pipeline_proof_corpus_support::persist_foundation_inputs_route_basis(&repo_root);
    handbook_engine::RepositoryInvocationIdentityServiceV1::new()
        .initialize_for_setup(&repo_root)
        .unwrap();
    (dir, repo_root)
}

fn persist_out_of_domain_route_revision(repo_root: &std::path::Path) {
    let state_path = pipeline_proof_corpus_support::pipeline_state_path(repo_root);
    let mut state: handbook_pipeline::route_state::RouteState =
        serde_yaml_bw::from_str(&std::fs::read_to_string(&state_path).unwrap()).unwrap();
    state.revision = 9_007_199_254_740_992;
    state.route_basis.as_mut().unwrap().state_revision = state.revision;
    std::fs::write(&state_path, serde_yaml_bw::to_string(&state).unwrap()).unwrap();
}

#[test]
fn direct_capture_refuses_out_of_domain_route_revision_before_public_writes() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_stage_07_capture_ready_repo();
    let target = repo_root.join("artifacts/foundation/FOUNDATION_STRATEGY.md");
    let before_target = std::fs::read(&target).unwrap();
    persist_out_of_domain_route_revision(&repo_root);
    let request = handbook_sdk::contract_membrane::pipeline_capture_direct_operation_request(
        None,
        pipeline_proof_corpus_support::FOUNDATION_INPUTS_PIPELINE_ID,
        pipeline_proof_corpus_support::STAGE_07_FOUNDATION_PACK_ID,
        &stage_seven_capture_input(),
        "capture-out-of-domain-key",
    )
    .unwrap();

    let response = handbook_sdk::HandbookSdkV1::open(&repo_root)
        .execute_pipeline_operation(&request)
        .unwrap();

    assert_eq!(response.terminal().status(), TerminalStatus::Blocked);
    assert!(response.receipts().is_empty());
    assert_eq!(std::fs::read(&target).unwrap(), before_target);
    let text =
        String::from_utf8(canonical_json_bytes(&response, &ContractLimits::packet0()).unwrap())
            .unwrap();
    assert!(text.contains("capture_invalid_state"));
    assert!(!text.contains("9007199254740992"));
    assert!(!text.contains("capture-out-of-domain-key"));
    assert!(!text.contains(repo_root.to_string_lossy().as_ref()));
}

#[test]
fn handoff_emit_refuses_out_of_domain_route_revision_before_bundle_writes() {
    let (_dir, repo_root) = install_stage_ten_work_specification_repo();
    let input = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../engine/tests/fixtures/hcm_2_4_work_specification")
            .join("artifacts/work-specification/work-specification.yaml"),
    )
    .unwrap();
    let capture_request =
        handbook_sdk::contract_membrane::pipeline_capture_direct_operation_request(
            None,
            pipeline_proof_corpus_support::FOUNDATION_INPUTS_PIPELINE_ID,
            pipeline_proof_corpus_support::STAGE_10_FEATURE_SPEC_ID,
            &input,
            "handoff-setup-capture-key",
        )
        .unwrap();
    let sdk = handbook_sdk::HandbookSdkV1::open(&repo_root);
    let capture_response = sdk.execute_pipeline_operation(&capture_request).unwrap();
    assert_eq!(capture_response.terminal().status(), TerminalStatus::Ok);
    let bundle_root = repo_root.join("artifacts/handoff/feature_slice");
    assert!(!bundle_root.exists());
    persist_out_of_domain_route_revision(&repo_root);
    let request = handbook_sdk::contract_membrane::pipeline_handoff_emit_operation_request(
        None,
        pipeline_proof_corpus_support::FOUNDATION_INPUTS_PIPELINE_ID,
        "feature-slice-decomposer",
        "handbook pipeline handoff emit",
        "test-suite",
        "handoff-out-of-domain-key",
    )
    .unwrap();

    let response = sdk.execute_pipeline_operation(&request).unwrap();

    assert_eq!(response.terminal().status(), TerminalStatus::Blocked);
    assert!(response.receipts().is_empty());
    assert!(!bundle_root.exists());
    let text =
        String::from_utf8(canonical_json_bytes(&response, &ContractLimits::packet0()).unwrap())
            .unwrap();
    assert!(text.contains("handoff_invalid_state"));
    assert!(!text.contains("9007199254740992"));
    assert!(!text.contains("handoff-out-of-domain-key"));
    assert!(!text.contains(repo_root.to_string_lossy().as_ref()));
}

#[test]
fn direct_capture_and_plan_apply_share_one_admitted_semantic_operation() {
    let (_direct_dir, direct_root) =
        pipeline_proof_corpus_support::install_stage_07_capture_ready_repo();
    let (_cached_dir, cached_root) =
        pipeline_proof_corpus_support::install_stage_07_capture_ready_repo();
    let input = stage_seven_capture_input();

    let direct_request =
        handbook_sdk::contract_membrane::pipeline_capture_direct_operation_request(
            None,
            pipeline_proof_corpus_support::FOUNDATION_INPUTS_PIPELINE_ID,
            pipeline_proof_corpus_support::STAGE_07_FOUNDATION_PACK_ID,
            &input,
            "direct-capture-parity",
        )
        .unwrap();
    let direct_response = handbook_sdk::HandbookSdkV1::open(&direct_root)
        .execute_pipeline_operation(&direct_request)
        .unwrap();
    assert_eq!(direct_response.terminal().status(), TerminalStatus::Ok);
    assert_eq!(direct_response.receipts().len(), 1);
    let direct_replay = handbook_sdk::HandbookSdkV1::open(&direct_root)
        .execute_pipeline_operation(&direct_request)
        .unwrap();
    assert!(matches!(
        direct_replay.idempotency().as_ref(),
        IdempotencyStateRef::Established { replayed: true, .. }
    ));
    direct_replay
        .verify_replay_against(&direct_response)
        .unwrap();
    let direct_conflict_request =
        handbook_sdk::contract_membrane::pipeline_capture_direct_operation_request(
            None,
            pipeline_proof_corpus_support::FOUNDATION_INPUTS_PIPELINE_ID,
            pipeline_proof_corpus_support::STAGE_07_FOUNDATION_PACK_ID,
            &format!("{input}\n"),
            "direct-capture-parity",
        )
        .unwrap();
    let direct_conflict = handbook_sdk::HandbookSdkV1::open(&direct_root)
        .execute_pipeline_operation(&direct_conflict_request)
        .unwrap();
    assert_eq!(direct_conflict.terminal().status(), TerminalStatus::Refused);
    assert!(direct_conflict.receipts().is_empty());
    let direct =
        handbook_sdk::contract_membrane::pipeline_capture_apply_from_response(&direct_response)
            .unwrap()
            .unwrap();

    let plan_request = handbook_sdk::contract_membrane::pipeline_capture_plan_operation_request(
        None,
        pipeline_proof_corpus_support::FOUNDATION_INPUTS_PIPELINE_ID,
        pipeline_proof_corpus_support::STAGE_07_FOUNDATION_PACK_ID,
        &input,
    )
    .unwrap();
    let cached_sdk = handbook_sdk::HandbookSdkV1::open(&cached_root);
    let plan_response = cached_sdk
        .execute_pipeline_operation(&plan_request)
        .unwrap();
    assert_eq!(plan_response.terminal().status(), TerminalStatus::Ok);
    assert!(plan_response.receipts().is_empty());
    let preview =
        handbook_sdk::contract_membrane::pipeline_capture_plan_from_response(&plan_response)
            .unwrap()
            .unwrap();
    let apply_request = handbook_sdk::contract_membrane::pipeline_capture_apply_operation_request(
        None,
        &preview.plan.capture_id,
        "cached-capture-parity",
    )
    .unwrap();
    assert_eq!(direct_request.operation(), apply_request.operation());
    let cached_response = cached_sdk
        .execute_pipeline_operation(&apply_request)
        .unwrap();
    assert_eq!(cached_response.terminal().status(), TerminalStatus::Ok);
    assert_eq!(cached_response.receipts().len(), 1);
    let cached =
        handbook_sdk::contract_membrane::pipeline_capture_apply_from_response(&cached_response)
            .unwrap()
            .unwrap();

    assert_eq!(preview.plan, cached.plan);
    assert_eq!(direct, cached);
}

#[test]
fn cached_capture_apply_preserves_exact_revision_conflict_truth() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_stage_07_capture_ready_repo();
    let input = stage_seven_capture_input();
    let sdk = handbook_sdk::HandbookSdkV1::open(&repo_root);
    let plan_request = handbook_sdk::contract_membrane::pipeline_capture_plan_operation_request(
        None,
        pipeline_proof_corpus_support::FOUNDATION_INPUTS_PIPELINE_ID,
        pipeline_proof_corpus_support::STAGE_07_FOUNDATION_PACK_ID,
        &input,
    )
    .unwrap();
    let plan_response = sdk.execute_pipeline_operation(&plan_request).unwrap();
    let preview =
        handbook_sdk::contract_membrane::pipeline_capture_plan_from_response(&plan_response)
            .unwrap()
            .unwrap();
    let state_path = pipeline_proof_corpus_support::pipeline_state_path(&repo_root);
    let mut state: handbook_pipeline::route_state::RouteState =
        serde_yaml_bw::from_str(&std::fs::read_to_string(&state_path).unwrap()).unwrap();
    let previewed_revision = state.revision;
    state.revision += 1;
    if let Some(route_basis) = state.route_basis.as_mut() {
        route_basis.state_revision = state.revision;
    }
    std::fs::write(&state_path, serde_yaml_bw::to_string(&state).unwrap()).unwrap();

    let apply_request = handbook_sdk::contract_membrane::pipeline_capture_apply_operation_request(
        None,
        &preview.plan.capture_id,
        "capture-revision-conflict-key",
    )
    .unwrap();
    let conflict = sdk.execute_pipeline_operation(&apply_request).unwrap();
    assert_eq!(conflict.terminal().status(), TerminalStatus::Refused);
    assert!(conflict.receipts().is_empty());
    let text =
        String::from_utf8(canonical_json_bytes(&conflict, &ContractLimits::packet0()).unwrap())
            .unwrap();
    assert!(text.contains("capture_revision_conflict"));
    assert!(text.contains(&format!("\"expected_revision\":{previewed_revision}")));
    assert!(text.contains(&format!("\"actual_revision\":{}", previewed_revision + 1)));
    assert!(!text.contains("capture-revision-conflict-key"));
    assert!(!text.contains(repo_root.to_string_lossy().as_ref()));
}

#[test]
fn direct_capture_error_cannot_hide_realized_early_write_receipts() {
    let (_dir, repo_root) = install_stage_ten_work_specification_repo();
    let provenance_path = ".handbook/state/pipeline/stage_capture/pipeline.foundation_inputs.stage.10_feature_spec.json";
    std::fs::create_dir_all(repo_root.join(provenance_path)).unwrap();
    let input = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../engine/tests/fixtures/hcm_2_4_work_specification")
            .join("artifacts/work-specification/work-specification.yaml"),
    )
    .unwrap();
    let request = handbook_sdk::contract_membrane::pipeline_capture_direct_operation_request(
        None,
        pipeline_proof_corpus_support::FOUNDATION_INPUTS_PIPELINE_ID,
        pipeline_proof_corpus_support::STAGE_10_FEATURE_SPEC_ID,
        &input,
        "partial-capture-key",
    )
    .unwrap();

    let response = handbook_sdk::HandbookSdkV1::open(&repo_root)
        .execute_pipeline_operation(&request)
        .unwrap();

    assert_eq!(response.terminal().status(), TerminalStatus::Error);
    assert!(response.terminal().data().is_none());
    assert_eq!(response.receipts().len(), 1);
    let bytes = canonical_json_bytes(&response, &ContractLimits::packet0()).unwrap();
    let text = String::from_utf8(bytes).unwrap();
    assert!(text.contains("capture_state_persistence_failure"));
    assert!(text.contains("pipeline_capture_artifact"));
    assert!(!text.contains("partial-capture-key"));
    assert!(!text.contains(repo_root.to_string_lossy().as_ref()));
    assert!(!text.contains("idempotency/"));
}
