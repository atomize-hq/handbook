#[path = "support/pipeline_proof_corpus_support.rs"]
mod pipeline_proof_corpus_support;

use handbook_sdk::contract_membrane::{
    operation_inventory, ApproverAdministrationRequestV1, AuthorCharterRequestV1,
    AuthorProjectContextRequestV1, ContractError, ContractLimits, InvocationRequestV1,
    InvocationResponseKind, InvocationResponseV1, OperationTransport,
    PipelineRouteResolveAndPersistRequestV1, ProductionWitness, ProjectContextInputV1,
    RepositorySetupModeV1, RepositorySetupRequestV1, UseCaseRequestV1, UseCaseResponseV1,
    UtcTimestamp,
};
use handbook_sdk::{
    ApproverCommandRequest, CharterCommandRequest, HandbookSdkV1, ProjectContextCommandRequest,
    ProjectContextInput, SetupMode, SetupRequest,
};
use std::path::Path;

const PROJECT_CONTEXT_INPUT_MODE: &str = "structured_inputs_tauri";
const PROJECT_CONTEXT_INPUT_SOURCE: &str = "tauri";

fn invocation(request: UseCaseRequestV1) -> InvocationRequestV1 {
    InvocationRequestV1::use_case(request, &ContractLimits::packet0()).unwrap()
}

fn close_use_case(response: UseCaseResponseV1) -> InvocationResponseV1 {
    InvocationResponseV1::use_case(response, &ContractLimits::packet0()).unwrap()
}

fn project_context_input_v1() -> ProjectContextInputV1 {
    ProjectContextInputV1::new(
        "handbook.artifact.project-context",
        "1.0",
        "handbook.project-context",
        "Packet 4 typed request",
        vec!["SDK facade".to_owned()],
        vec!["Project owner".to_owned()],
        vec!["docs/START_HERE.md".to_owned()],
        vec!["None".to_owned()],
    )
    .unwrap()
}

fn project_context_input() -> ProjectContextInput {
    ProjectContextInput {
        schema_id: "handbook.artifact.project-context".to_owned(),
        schema_version: "1.0".to_owned(),
        record_id: "handbook.project-context".to_owned(),
        summary: "Packet 4 typed request".to_owned(),
        system_boundaries: vec!["SDK facade".to_owned()],
        ownership: vec!["Project owner".to_owned()],
        authoritative_references: vec!["docs/START_HERE.md".to_owned()],
        known_unknowns: vec!["None".to_owned()],
    }
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

#[test]
fn dispatcher_routes_each_fixed_use_case_once_through_its_existing_sdk_composition() {
    let limits = ContractLimits::packet0();

    let setup_direct = tempfile::tempdir().unwrap();
    let setup_dispatch = tempfile::tempdir().unwrap();
    let setup_seed = tempfile::tempdir().unwrap();
    std::fs::create_dir(setup_seed.path().join(".git")).unwrap();
    handbook_engine::RepositoryInvocationIdentityServiceV1::new()
        .initialize_for_setup(setup_seed.path())
        .unwrap();
    copy_tree(setup_seed.path(), setup_direct.path());
    copy_tree(setup_seed.path(), setup_dispatch.path());
    let setup_request = SetupRequest {
        mode: SetupMode::Refresh,
        rewrite: false,
        reset_state: false,
    };
    let expected = close_use_case(
        HandbookSdkV1::open(setup_direct.path())
            .run_setup_use_case(&setup_request, &limits)
            .unwrap(),
    );
    let request = invocation(
        UseCaseRequestV1::repository_setup(
            RepositorySetupRequestV1::new(RepositorySetupModeV1::Refresh, false, false),
            &limits,
        )
        .unwrap(),
    );
    let actual = HandbookSdkV1::open(setup_dispatch.path())
        .invoke_tauri(&request, &limits)
        .unwrap();
    assert_eq!(actual, expected);

    let doctor_direct = tempfile::tempdir().unwrap();
    let doctor_dispatch = tempfile::tempdir().unwrap();
    let direct = HandbookSdkV1::open(doctor_direct.path()).doctor();
    let expected = close_use_case(match direct {
        Ok(report) => UseCaseResponseV1::repository_doctor(&report, &limits).unwrap(),
        Err(error) => UseCaseResponseV1::repository_doctor_error(error, &limits).unwrap(),
    });
    let request = invocation(UseCaseRequestV1::repository_doctor(&limits).unwrap());
    let actual = HandbookSdkV1::open(doctor_dispatch.path())
        .invoke_tauri(&request, &limits)
        .unwrap();
    assert_eq!(actual, expected);

    let charter_direct = tempfile::tempdir().unwrap();
    let charter_dispatch = tempfile::tempdir().unwrap();
    let expected = HandbookSdkV1::open(charter_direct.path())
        .run_author_charter_use_case(CharterCommandRequest::Validate, &limits)
        .unwrap();
    let request = invocation(
        UseCaseRequestV1::author_charter(AuthorCharterRequestV1::Validate, &limits).unwrap(),
    );
    let actual = HandbookSdkV1::open(charter_dispatch.path())
        .invoke_tauri(&request, &limits)
        .unwrap();
    assert_eq!(actual, expected);

    let context_direct = tempfile::tempdir().unwrap();
    let context_dispatch = tempfile::tempdir().unwrap();
    let expected = close_use_case(
        HandbookSdkV1::open(context_direct.path())
            .run_author_project_context_use_case(
                PROJECT_CONTEXT_INPUT_MODE,
                PROJECT_CONTEXT_INPUT_SOURCE,
                ProjectContextCommandRequest::Validate(project_context_input()),
                &limits,
            )
            .unwrap(),
    );
    let request = invocation(
        UseCaseRequestV1::author_project_context(
            AuthorProjectContextRequestV1::Validate(project_context_input_v1()),
            &limits,
        )
        .unwrap(),
    );
    let actual = HandbookSdkV1::open(context_dispatch.path())
        .invoke_tauri(&request, &limits)
        .unwrap();
    assert_eq!(actual, expected);

    let approver_direct = tempfile::tempdir().unwrap();
    let approver_dispatch = tempfile::tempdir().unwrap();
    let expected = HandbookSdkV1::open(approver_direct.path())
        .run_approver_administration_use_case(
            ApproverCommandRequest::RevokeCredential {
                credential_id_hash: "sha256:packet-four".to_owned(),
            },
            &limits,
        )
        .unwrap();
    let request = invocation(
        UseCaseRequestV1::approver_administration(
            ApproverAdministrationRequestV1::revoke_credential("sha256:packet-four").unwrap(),
            &limits,
        )
        .unwrap(),
    );
    let actual = HandbookSdkV1::open(approver_dispatch.path())
        .invoke_tauri(&request, &limits)
        .unwrap();
    assert_eq!(actual, expected);

    let pipeline_direct = tempfile::tempdir().unwrap();
    let pipeline_dispatch = tempfile::tempdir().unwrap();
    let expected = close_use_case(
        HandbookSdkV1::open(pipeline_direct.path())
            .resolve_pipeline_use_case("packet-four-missing", &limits)
            .unwrap(),
    );
    let request = invocation(
        UseCaseRequestV1::pipeline_route_resolve_and_persist(
            PipelineRouteResolveAndPersistRequestV1::new("packet-four-missing").unwrap(),
            &limits,
        )
        .unwrap(),
    );
    let actual = HandbookSdkV1::open(pipeline_dispatch.path())
        .invoke_tauri(&request, &limits)
        .unwrap();
    assert_eq!(actual, expected);

    for response in [actual, expected] {
        assert_eq!(response.response_kind(), InvocationResponseKind::UseCase);
    }
}

#[test]
fn write_bearing_setup_keeps_realized_receipts_without_duplicate_targets() {
    let repo = tempfile::tempdir().unwrap();
    std::fs::create_dir(repo.path().join(".git")).unwrap();
    let limits = ContractLimits::packet0();
    let request = invocation(
        UseCaseRequestV1::repository_setup(
            RepositorySetupRequestV1::new(RepositorySetupModeV1::Init, false, false),
            &limits,
        )
        .unwrap(),
    );

    let response = HandbookSdkV1::open(repo.path())
        .invoke_tauri(&request, &limits)
        .unwrap();
    let document = serde_json::to_value(response).unwrap();
    let receipts = document["response"]["receipts"].as_array().unwrap();
    assert_eq!(receipts.len(), 2);
    let record_kinds = receipts
        .iter()
        .map(|receipt| receipt["record_kind"].as_str().unwrap())
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(record_kinds.len(), receipts.len());
    assert!(repo.path().join(".handbook").is_dir());
    assert!(repo
        .path()
        .join(".handbook/repository-identity.v1")
        .is_file());
}

#[test]
fn write_bearing_pipeline_route_keeps_realized_receipt_without_duplicate_execution() {
    let limits = ContractLimits::packet0();
    let (_direct_dir, direct_root) =
        pipeline_proof_corpus_support::install_foundation_inputs_repo();
    let (_dispatch_dir, dispatch_root) =
        pipeline_proof_corpus_support::install_foundation_inputs_repo();
    let selector = pipeline_proof_corpus_support::FOUNDATION_INPUTS_PIPELINE_ID;
    let expected = close_use_case(
        HandbookSdkV1::open(&direct_root)
            .resolve_pipeline_use_case(selector, &limits)
            .unwrap(),
    );
    let request = invocation(
        UseCaseRequestV1::pipeline_route_resolve_and_persist(
            PipelineRouteResolveAndPersistRequestV1::new(selector).unwrap(),
            &limits,
        )
        .unwrap(),
    );

    let actual = HandbookSdkV1::open(&dispatch_root)
        .invoke_tauri(&request, &limits)
        .unwrap();
    assert_eq!(actual, expected);
    let document = serde_json::to_value(actual).unwrap();
    assert_eq!(
        document["response"]["receipts"].as_array().unwrap().len(),
        1
    );
    assert!(pipeline_proof_corpus_support::pipeline_state_path(&dispatch_root).is_file());
}

#[test]
fn readmission_rejects_substituted_use_case_response() {
    let repo = tempfile::tempdir().unwrap();
    let limits = ContractLimits::packet0();
    let originating = invocation(UseCaseRequestV1::repository_doctor(&limits).unwrap());
    let substituted_request = invocation(
        UseCaseRequestV1::author_charter(AuthorCharterRequestV1::Validate, &limits).unwrap(),
    );
    let sdk = HandbookSdkV1::open(repo.path());
    let substituted = sdk.invoke_tauri(&substituted_request, &limits).unwrap();

    assert_eq!(
        sdk.readmit_invocation_response(&originating, &substituted, &limits),
        Err(ContractError::InvalidInvariant)
    );
}

#[test]
fn use_cases_remain_outside_operation_inventory_owner_admission_and_discovery() {
    let use_case_refs = [
        "handbook.use-case.repository-setup@1.0.0",
        "handbook.use-case.repository-doctor@1.0.0",
        "handbook.use-case.author-charter@1.0.0",
        "handbook.use-case.author-project-context@1.0.0",
        "handbook.use-case.approver-administration@1.0.0",
        "handbook.use-case.pipeline-route-resolve-and-persist@1.0.0",
        "handbook.use-case.session-grounding-boundary@1.0.0",
    ];
    let rows = operation_inventory();
    assert_eq!(rows.len(), 62);
    for use_case_ref in use_case_refs {
        assert!(rows
            .iter()
            .all(|row| row.operation_ref().as_str() != use_case_ref));
    }

    let witness =
        ProductionWitness::from_current(UtcTimestamp::parse("2030-01-01T00:00:00Z").unwrap())
            .unwrap();
    assert_eq!(witness.capabilities().len(), 35);
    for use_case_ref in use_case_refs {
        assert!(witness
            .capabilities()
            .iter()
            .all(|entry| entry.operation().reference().as_str() != use_case_ref));
    }
    assert_eq!(
        rows.iter()
            .filter(|row| row
                .current_transports()
                .contains(&OperationTransport::Tauri))
            .count(),
        35
    );
}
