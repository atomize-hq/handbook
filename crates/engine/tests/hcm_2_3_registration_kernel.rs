#![allow(dead_code)]

use handbook_engine::*;
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

#[path = "../src/artifact_intake.rs"]
mod artifact_intake;
#[path = "../src/artifact_intake_registry.rs"]
mod artifact_intake_registry;
#[path = "../src/artifact_operation_context.rs"]
mod artifact_operation_context;
#[path = "../src/artifact_operations.rs"]
mod artifact_operations;
#[path = "../src/canonical_yaml.rs"]
mod canonical_yaml;

use artifact_intake::*;
use artifact_intake_registry::*;
use artifact_operation_context::*;
use artifact_operations::*;
use canonical_yaml::*;

const SELECTION_VECTORS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/specs/handbook-contract-membrane/slices/HCM-2.3/contracts/repository-profile-selection-vectors-v1.0.json"
));

fn exact(value: &str) -> ExactDefinitionRef {
    ExactDefinitionRef::parse(value).expect("exact ref")
}

fn fp(value: &str) -> DefinitionFingerprint {
    DefinitionFingerprint::parse(value).expect("fingerprint")
}

fn builtin(value: &str) -> DefinitionSourceBinding {
    let definition_ref = exact(value);
    DefinitionSourceBinding {
        definition_ref: definition_ref.clone(),
        source: DefinitionSource::BuiltIn(definition_ref),
    }
}

fn selection_vector_record() -> Value {
    serde_json::from_str::<Value>(SELECTION_VECTORS).expect("selection vectors")["positive_vectors"]
        [0]["record"]
        .clone()
}

fn intake_source() -> (ExactDefinitionRef, Vec<u8>) {
    (
        exact("example.intake.registry-brief@1.0.0"),
        include_bytes!(
            "fixtures/hcm_2_3_generic_custom_kind/.handbook/definitions/intakes/registry-brief-1.0.0.yaml"
        )
        .to_vec(),
    )
}

fn intake_registry() -> ArtifactIntakeRegistry {
    let (definition_ref, bytes) = intake_source();
    ArtifactIntakeRegistry::load(&[ArtifactIntakeSourceV1 {
        definition_ref,
        bytes,
    }])
    .expect("intake registry")
}

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

fn tree_bytes(root: &Path) -> BTreeMap<String, Vec<u8>> {
    fn visit(base: &Path, current: &Path, output: &mut BTreeMap<String, Vec<u8>>) {
        if !current.exists() {
            return;
        }
        for entry in fs::read_dir(current).expect("tree directory") {
            let path = entry.expect("tree entry").path();
            if path.is_dir() {
                visit(base, &path, output);
            } else {
                output.insert(
                    path.strip_prefix(base)
                        .expect("tree-relative path")
                        .to_string_lossy()
                        .replace('\\', "/"),
                    fs::read(path).expect("tree bytes"),
                );
            }
        }
    }
    let mut output = BTreeMap::new();
    visit(root, root, &mut output);
    output
}

fn padded_document(value: Value, length: usize) -> Vec<u8> {
    let mut bytes = serde_json::to_vec(&value).expect("request JSON");
    assert!(bytes.len() <= length);
    bytes.resize(length, b' ');
    bytes
}

fn rewrite_custom_intake(repo_root: &Path, mutate: impl FnOnce(&mut Value)) {
    let path = repo_root.join(".handbook/definitions/intakes/registry-brief-1.0.0.yaml");
    let mut value = parse_definition_yaml(&fs::read(&path).unwrap()).unwrap();
    mutate(&mut value);
    let object = value.as_object_mut().unwrap();
    object.remove("intake_definition_fingerprint");
    let fingerprint = DefinitionFingerprint::from_json_value(&value).unwrap();
    value.as_object_mut().unwrap().insert(
        "intake_definition_fingerprint".into(),
        Value::String(fingerprint.to_string()),
    );
    fs::write(path, serde_json::to_vec_pretty(&value).unwrap()).unwrap();
}

fn operation_context() -> ArtifactOperationContextV1 {
    ArtifactOperationContextV1::new(ArtifactOperationContextInputV1 {
        repository_identity_fingerprint: fp(
            "sha256:1111111111111111111111111111111111111111111111111111111111111111",
        ),
        selection_fingerprint: fp(
            "sha256:3a5c945e68c29fa765bc4568ff4eae1eacbd84a9f90a71a00a56bb5d04cc82c9",
        ),
        profile_ref: exact("example.profile.registry-root@1.1.0"),
        resolved_profile_fingerprint: fp(
            "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        ),
        kind_ref: exact("example.artifact-kind.registry-brief@1.0.0"),
        kind_fingerprint: fp(
            "sha256:2222222222222222222222222222222222222222222222222222222222222222",
        ),
        instance_id: SymbolicId::parse("registry_brief").expect("instance id"),
        descriptor_fingerprint: fp(
            "sha256:3333333333333333333333333333333333333333333333333333333333333333",
        ),
        schema_ref: exact("example.schemas.registry-brief@1.0.0"),
        schema_entry_fingerprint: fp(
            "sha256:4444444444444444444444444444444444444444444444444444444444444444",
        ),
        schema_document_fingerprint: fp(
            "sha256:5555555555555555555555555555555555555555555555555555555555555555",
        ),
        intake_definition_ref: Some(exact("example.intake.registry-brief@1.0.0")),
        intake_definition_fingerprint: Some(fp(
            "sha256:cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc",
        )),
        resolved_definitions: vec![
            DefinitionBindingV1 {
                definition_ref: exact("example.artifact-kind.registry-brief@1.0.0"),
                definition_fingerprint: fp(
                    "sha256:2222222222222222222222222222222222222222222222222222222222222222",
                ),
            },
            DefinitionBindingV1 {
                definition_ref: exact("example.intake.registry-brief@1.0.0"),
                definition_fingerprint: fp(
                    "sha256:cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc",
                ),
            },
            DefinitionBindingV1 {
                definition_ref: exact("example.profile.registry-root@1.1.0"),
                definition_fingerprint: fp(
                    "sha256:9999999999999999999999999999999999999999999999999999999999999999",
                ),
            },
            DefinitionBindingV1 {
                definition_ref: exact("example.schemas.registry-brief@1.0.0"),
                definition_fingerprint: fp(
                    "sha256:4444444444444444444444444444444444444444444444444444444444444444",
                ),
            },
        ],
    })
    .expect("operation context")
}

#[test]
fn fixed_selection_record_maps_every_explicit_source_class_and_replays_fingerprint() {
    let record = selection_vector_record();
    let selection = RepositoryProfileSelectionV1::from_json_bytes(
        &serde_json::to_vec(&record).expect("selection bytes"),
    )
    .expect("selection record");

    assert_eq!(
        selection.selection_fingerprint().as_str(),
        "sha256:3a5c945e68c29fa765bc4568ff4eae1eacbd84a9f90a71a00a56bb5d04cc82c9"
    );
    let request = selection.profile_request();
    assert_eq!(request.profile_sources.len(), 2);
    assert_eq!(request.schema_entry_sources.len(), 18);
    assert_eq!(request.artifact_kind_sources.len(), 8);
    assert_eq!(request.intake_definition_sources.len(), 2);
    assert_eq!(
        request.allowed_schema_roots,
        [".handbook/definitions/schemas"]
    );

    let mut reversed = record;
    for (key, value) in reversed.as_object_mut().expect("selection object") {
        if (key.ends_with("_sources") || key == "allowed_schema_roots") && value.is_array() {
            value.as_array_mut().expect("source array").reverse();
        }
    }
    let reversed = RepositoryProfileSelectionV1::from_json_bytes(
        &serde_json::to_vec(&reversed).expect("reversed bytes"),
    )
    .expect("reversed selection");
    assert_eq!(
        selection.selection_fingerprint(),
        reversed.selection_fingerprint()
    );
}

#[test]
fn selection_ref_rebinding_is_refused_before_any_definition_is_admitted() {
    let mut record = selection_vector_record();
    let duplicate = record["profile_sources"][0].clone();
    record["intake_definition_sources"]
        .as_array_mut()
        .expect("intake sources")
        .push(duplicate);

    let error = RepositoryProfileSelectionV1::from_json_bytes(
        &serde_json::to_vec(&record).expect("selection bytes"),
    )
    .unwrap_err();
    assert_eq!(
        error.kind(),
        ArtifactRegistrationErrorKindV1::DefinitionClassMismatch
    );
}

#[test]
fn complete_fixture_selection_resolves_with_only_its_repository_schema_root() {
    let fixture_root =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/hcm_2_3_generic_custom_kind");
    let selection = RepositoryProfileSelectionV1::from_json_bytes(include_bytes!(
        "fixtures/hcm_2_3_generic_custom_kind/.handbook/profile-selection.json"
    ))
    .expect("fixture selection");
    let request = selection.profile_request();
    assert_eq!(
        request.allowed_schema_roots,
        [".handbook/definitions/schemas"]
    );
    let profile = resolve_profile_selection(&fixture_root, request)
        .expect("mixed built-in/repository profile closure");

    assert_eq!(
        profile.exact_ref().as_str(),
        "example.profile.registry-root@1.1.0"
    );
    assert_eq!(
        profile.resolved_profile_fingerprint().as_str(),
        "sha256:791e540069e1d155a5a82803b0f43e94ccc6c5e9da649f815c72107c36b94a47"
    );
    assert_eq!(
        profile
            .artifact_instances()
            .instance(&SymbolicId::parse("registry_brief").expect("instance id"))
            .expect("registry brief")
            .intake_definition_ref()
            .expect("intake ref")
            .as_str(),
        "example.intake.registry-brief@1.0.0"
    );
}

#[test]
fn engine_repository_session_reads_and_validates_the_real_custom_instance() {
    let fixture_root =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/hcm_2_3_generic_custom_kind");
    let repo = tempfile::tempdir().unwrap();
    copy_tree(&fixture_root, repo.path());
    RepositoryInvocationIdentityServiceV1::new()
        .initialize_for_setup(repo.path())
        .expect("repository identity");

    let repository = handbook_engine::artifact_repository::ArtifactRepositoryV1::open(repo.path())
        .expect("artifact repository session");
    let charter_context = repository
        .operation_context(
            &exact("handbook.artifact-kind.project-authority@1.1.0"),
            &SymbolicId::parse("project_authority").unwrap(),
        )
        .expect("built-in charter intake remains in the resolved context");
    assert_eq!(
        charter_context
            .intake_definition_ref()
            .expect("charter intake ref")
            .as_str(),
        "handbook.intake.charter@1.0.0"
    );
    assert!(repository
        .list_kinds()
        .expect("recovered kind list")
        .iter()
        .any(|kind| kind.kind_ref.as_str() == "example.artifact-kind.registry-brief@1.0.0"));
    assert!(repository
        .list_instances()
        .expect("recovered instance list")
        .iter()
        .any(|instance| instance.instance_id.as_str() == "registry_brief"));

    let kind_ref = exact("example.artifact-kind.registry-brief@1.0.0");
    let instance_id = SymbolicId::parse("registry_brief").unwrap();
    let read = repository
        .read(&kind_ref, &instance_id)
        .expect("descriptor-selected safe read");
    assert_eq!(
        read.content,
        json!({
            "summary": "A reusable registry brief",
            "title": "Registry Brief Proof"
        })
    );
    repository
        .validate(&kind_ref, &instance_id)
        .expect("selected exact structural schema");

    let wrong_typed = [
        handbook_engine::artifact_intake::CoverageSubmissionV1 {
            coverage_id: "registry_brief.title".into(),
            state: handbook_engine::artifact_intake::CoverageSubmissionStateV1::Supplied,
            source_kind: handbook_engine::artifact_intake::CoverageSourceKindV1::UserDeclaration,
            value: Some(json!(7)),
            specificity: handbook_engine::artifact_intake::CoverageSpecificityV1::Concrete,
            confidence: handbook_engine::artifact_intake::CoverageConfidenceV1::High,
            contradiction_refs: Vec::new(),
        },
        handbook_engine::artifact_intake::CoverageSubmissionV1 {
            coverage_id: "registry_brief.summary".into(),
            state: handbook_engine::artifact_intake::CoverageSubmissionStateV1::Supplied,
            source_kind: handbook_engine::artifact_intake::CoverageSourceKindV1::UserDeclaration,
            value: Some(json!("valid summary")),
            specificity: handbook_engine::artifact_intake::CoverageSpecificityV1::Concrete,
            confidence: handbook_engine::artifact_intake::CoverageConfidenceV1::High,
            contradiction_refs: Vec::new(),
        },
    ];
    assert_eq!(
        repository
            .evaluate_intake(
                &kind_ref,
                &instance_id,
                handbook_engine::artifact_intake_registry::AcquisitionModeV1::Express,
                None,
                &wrong_typed,
            )
            .unwrap_err()
            .kind(),
        handbook_engine::artifact_repository::ArtifactRepositoryErrorKindV1::StructuralValidation
    );
}

#[test]
fn raw_generic_lineage_store_is_not_a_public_engine_module() {
    let library = include_str!("../src/lib.rs");
    assert!(library.contains("mod artifact_lineage_store;"));
    assert!(!library.contains("pub mod artifact_lineage_store;"));
    assert!(library.contains("pub mod artifact_mutation;"));
}

#[test]
fn nonselected_target_and_structurally_invalid_intake_establish_no_authoritative_outputs() {
    let fixture_root =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/hcm_2_3_generic_custom_kind");
    let repo = tempfile::tempdir().unwrap();
    copy_tree(&fixture_root, repo.path());
    RepositoryInvocationIdentityServiceV1::new()
        .initialize_for_setup(repo.path())
        .expect("repository identity");
    let invalid_target_request = serde_json::to_vec(&json!({
        "idempotency_key": "nonselected_target_000001",
        "acquisition_mode": "express",
        "expected_current_artifact_fingerprint": null,
        "coverage_submissions": []
    }))
    .unwrap();
    let error = handbook_engine::artifact_mutation::ArtifactMutationServiceV1::intake_append(
        repo.path(),
        "example.artifact-kind.registry-brief@1.0.0",
        "project_authority",
        &invalid_target_request,
    )
    .expect_err("kind/instance mismatch must fail before generic establishment");
    assert_eq!(
        error.kind(),
        handbook_engine::artifact_mutation::ArtifactMutationErrorKindV1::RepositoryIdentity
    );
    assert!(!repo.path().join(".handbook/state/transactions").exists());

    let canonical = fs::read(repo.path().join(".handbook/project/registry-brief.yaml")).unwrap();
    let current = DefinitionFingerprint::from_bytes(&canonical);
    let structural_request = serde_json::to_vec(&json!({
        "idempotency_key": "structural_refusal_000001",
        "acquisition_mode": "express",
        "expected_current_artifact_fingerprint": current.as_str(),
        "coverage_submissions": [
            {
                "coverage_id": "registry_brief.title",
                "state": "supplied",
                "source_kind": "user_declaration",
                "value": {"not": "a string"},
                "specificity": "exact",
                "confidence": "high",
                "contradiction_refs": []
            },
            {
                "coverage_id": "registry_brief.summary",
                "state": "supplied",
                "source_kind": "user_declaration",
                "value": "summary",
                "specificity": "concrete",
                "confidence": "high",
                "contradiction_refs": []
            }
        ]
    }))
    .unwrap();
    let first = handbook_engine::artifact_mutation::ArtifactMutationServiceV1::intake_append(
        repo.path(),
        "example.artifact-kind.registry-brief@1.0.0",
        "registry_brief",
        &structural_request,
    )
    .expect("structural refusal is an established result");
    assert_eq!(first.result.outcome, "refused");
    assert_eq!(
        first.result.refusal.as_ref().expect("refusal").code,
        handbook_engine::artifact_mutation::EstablishedRefusalCodeV1::StructuralValidationFailed
    );
    assert!(first.result.authoritative_outputs.is_empty());
    assert!(!repo.path().join(".handbook/evidence/artifacts").exists());
    let after_first = tree_bytes(&repo.path().join(".handbook"));
    let replay = handbook_engine::artifact_mutation::ArtifactMutationServiceV1::intake_append(
        repo.path(),
        "example.artifact-kind.registry-brief@1.0.0",
        "registry_brief",
        &structural_request,
    )
    .expect("structural refusal replays");
    assert_eq!(
        replay.disposition,
        handbook_engine::artifact_mutation::GenericExecutionDispositionV1::Replayed
    );
    assert_eq!(
        replay.result.result_fingerprint,
        first.result.result_fingerprint
    );
    assert_eq!(after_first, tree_bytes(&repo.path().join(".handbook")));
}

#[test]
fn engine_owner_byte_documents_refuse_over_limit_before_repository_mutation() {
    const LIMIT: usize = 1024 * 1024;
    let fixture_root =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/hcm_2_3_generic_custom_kind");

    let repo = tempfile::tempdir().unwrap();
    copy_tree(&fixture_root, repo.path());
    RepositoryInvocationIdentityServiceV1::new()
        .initialize_for_setup(repo.path())
        .expect("repository identity");
    let repository = handbook_engine::artifact_repository::ArtifactRepositoryV1::open(repo.path())
        .expect("repository");
    let target = handbook_engine::artifact_repository::ArtifactTargetV1::parse(
        "example.artifact-kind.registry-brief@1.0.0",
        "registry_brief",
    )
    .unwrap();
    let before = tree_bytes(repo.path());
    let over = padded_document(json!({"coverage_submissions": []}), LIMIT + 1);
    assert!(repository
        .evaluate_intake_document(
            &target,
            handbook_engine::artifact_intake_registry::AcquisitionModeV1::Express,
            None,
            &over,
        )
        .is_err());
    assert_eq!(tree_bytes(repo.path()), before);

    let requests = [
        (
            "intake",
            padded_document(
                json!({
                    "idempotency_key": "overlimit_intake_0001",
                    "acquisition_mode": "express",
                    "expected_current_artifact_fingerprint": null,
                    "coverage_submissions": []
                }),
                LIMIT + 1,
            ),
        ),
        (
            "candidate",
            padded_document(
                json!({
                    "idempotency_key": "overlimit_candidate_0001",
                    "intake_record_ref": ".handbook/state/artifacts/registry_brief/intake-records/intake_missing.json",
                    "intake_record_fingerprint": "sha256:1111111111111111111111111111111111111111111111111111111111111111",
                    "expected_candidate_fingerprint": "sha256:2222222222222222222222222222222222222222222222222222222222222222"
                }),
                LIMIT + 1,
            ),
        ),
        (
            "promotion",
            padded_document(
                json!({
                    "idempotency_key": "overlimit_promotion_0001",
                    "candidate_ref": ".handbook/state/artifacts/registry_brief/candidates/candidate_missing.json",
                    "candidate_fingerprint": "sha256:3333333333333333333333333333333333333333333333333333333333333333",
                    "expected_current_artifact_fingerprint": null
                }),
                LIMIT + 1,
            ),
        ),
    ];
    for (operation, bytes) in requests {
        let before = tree_bytes(repo.path());
        let result = match operation {
            "intake" => {
                handbook_engine::artifact_mutation::ArtifactMutationServiceV1::intake_append(
                    repo.path(),
                    "example.artifact-kind.registry-brief@1.0.0",
                    "registry_brief",
                    &bytes,
                )
            }
            "candidate" => {
                handbook_engine::artifact_mutation::ArtifactMutationServiceV1::candidate_append(
                    repo.path(),
                    "example.artifact-kind.registry-brief@1.0.0",
                    "registry_brief",
                    &bytes,
                )
            }
            "promotion" => handbook_engine::artifact_mutation::ArtifactMutationServiceV1::promote(
                repo.path(),
                "example.artifact-kind.registry-brief@1.0.0",
                "registry_brief",
                &bytes,
            ),
            _ => unreachable!(),
        };
        let error = result.expect_err("over-limit owner document must refuse");
        assert_eq!(
            error.kind(),
            handbook_engine::artifact_mutation::ArtifactMutationErrorKindV1::InvalidRequest,
            "operation: {operation}"
        );
        assert_eq!(tree_bytes(repo.path()), before, "operation: {operation}");
    }
}

#[test]
fn intake_consumer_version_is_fixed_by_the_engine_release() {
    let fixture_root =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/hcm_2_3_generic_custom_kind");
    let repo = tempfile::tempdir().unwrap();
    copy_tree(&fixture_root, repo.path());
    RepositoryInvocationIdentityServiceV1::new()
        .initialize_for_setup(repo.path())
        .expect("repository identity");
    let canonical = fs::read(repo.path().join(".handbook/project/registry-brief.yaml")).unwrap();
    let request = serde_json::to_vec(&json!({
        "idempotency_key": "fixed_consumer_version_0001",
        "acquisition_mode": "express",
        "expected_current_artifact_fingerprint": DefinitionFingerprint::from_bytes(&canonical).as_str(),
        "coverage_submissions": [
            {
                "coverage_id": "registry_brief.title",
                "state": "supplied",
                "source_kind": "user_declaration",
                "value": "Fixed version",
                "specificity": "exact",
                "confidence": "high",
                "contradiction_refs": []
            },
            {
                "coverage_id": "registry_brief.summary",
                "state": "supplied",
                "source_kind": "user_declaration",
                "value": "The engine owns the release version.",
                "specificity": "concrete",
                "confidence": "high",
                "contradiction_refs": []
            }
        ]
    }))
    .unwrap();
    let execution = handbook_engine::artifact_mutation::ArtifactMutationServiceV1::intake_append(
        repo.path(),
        "example.artifact-kind.registry-brief@1.0.0",
        "registry_brief",
        &request,
    )
    .expect("intake append");
    let record_ref = execution
        .result
        .authoritative_outputs
        .iter()
        .find(|output| {
            output.authority_class
                == handbook_engine::artifact_mutation::GenericAuthorityClassV1::SemanticRecord
        })
        .expect("semantic record")
        .relative_ref
        .clone();
    let record: Value = serde_json::from_slice(&fs::read(repo.path().join(record_ref)).unwrap())
        .expect("intake record JSON");
    assert_eq!(
        record["consumer"]["version"],
        include_str!("../../../VERSION").trim()
    );
}

#[test]
fn repository_intake_meta_validation_rejects_unknown_omitted_and_overlapping_targets() {
    let fixture_root =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/hcm_2_3_generic_custom_kind");
    for mutation in ["unknown", "omitted", "overlap"] {
        let repo = tempfile::tempdir().unwrap();
        copy_tree(&fixture_root, repo.path());
        rewrite_custom_intake(repo.path(), |value| match mutation {
            "unknown" => {
                value["coverage"][1]["target_paths"] = json!(["/not_a_schema_leaf"]);
            }
            "omitted" => {
                value["coverage"].as_array_mut().unwrap().pop();
            }
            "overlap" => {
                value["coverage"][0]["target_paths"] = json!(["/title", "/title/child"]);
            }
            _ => unreachable!(),
        });
        RepositoryInvocationIdentityServiceV1::new()
            .initialize_for_setup(repo.path())
            .unwrap();
        let error = handbook_engine::artifact_repository::ArtifactRepositoryV1::open(repo.path())
            .unwrap_err();
        assert_eq!(
            error.kind(),
            handbook_engine::artifact_repository::ArtifactRepositoryErrorKindV1::IntakeAdmission,
            "mutation {mutation}"
        );
    }
}

#[test]
fn intake_registry_is_exact_closed_and_source_order_independent() {
    let (definition_ref, bytes) = intake_source();
    let source = ArtifactIntakeSourceV1 {
        definition_ref: definition_ref.clone(),
        bytes: bytes.clone(),
    };
    let registry =
        ArtifactIntakeRegistry::load(std::slice::from_ref(&source)).expect("intake registry");
    let definition = registry
        .definition(&definition_ref)
        .expect("registered intake");
    assert_eq!(
        definition.artifact_kind_ref().as_str(),
        "example.artifact-kind.registry-brief@1.0.0"
    );
    assert_eq!(
        definition.candidate_schema_ref().as_str(),
        "example.schemas.registry-brief@1.0.0"
    );
    assert_eq!(definition.coverage().len(), 2);

    let duplicate = ArtifactIntakeRegistry::load(&[source.clone(), source]).unwrap_err();
    assert_eq!(
        duplicate.kind(),
        ArtifactRegistrationErrorKindV1::DuplicateIdentity
    );

    let mismatch = ArtifactIntakeRegistry::load(&[ArtifactIntakeSourceV1 {
        definition_ref: exact("example.intake.another-brief@1.0.0"),
        bytes,
    }])
    .unwrap_err();
    assert_eq!(
        mismatch.kind(),
        ArtifactRegistrationErrorKindV1::BoundRefMismatch
    );
}

#[test]
fn operation_context_replays_the_normative_fingerprint_and_freezes_definition_order() {
    let context = operation_context();
    assert_eq!(
        context.context_fingerprint().as_str(),
        "sha256:6247735430f2296162010902e7f6e1e20403f67ab6de7a4e4cc7fcdf9bd466c8"
    );
    assert_eq!(
        context
            .resolved_definitions()
            .iter()
            .map(|binding| binding.definition_ref.as_str())
            .collect::<Vec<_>>(),
        [
            "example.artifact-kind.registry-brief@1.0.0",
            "example.intake.registry-brief@1.0.0",
            "example.profile.registry-root@1.1.0",
            "example.schemas.registry-brief@1.0.0",
        ]
    );

    let mut input = context.into_input();
    input.resolved_definitions.swap(0, 1);
    let error = ArtifactOperationContextV1::new(input).unwrap_err();
    assert_eq!(
        error.kind(),
        ArtifactRegistrationErrorKindV1::InvalidDefinitionOrder
    );
}

#[test]
fn pure_coverage_evaluation_matches_the_control_vector_without_io_or_defaults() {
    let registry = intake_registry();
    let definition = registry
        .definition(&exact("example.intake.registry-brief@1.0.0"))
        .expect("intake definition");
    let mut context_input = operation_context().into_input();
    context_input.intake_definition_fingerprint = Some(definition.definition_fingerprint().clone());
    context_input
        .resolved_definitions
        .iter_mut()
        .find(|binding| binding.definition_ref == *definition.exact_ref())
        .expect("intake definition binding")
        .definition_fingerprint = definition.definition_fingerprint().clone();
    let context = ArtifactOperationContextV1::new(context_input).expect("fixture context");
    let submissions = vec![
        CoverageSubmissionV1 {
            coverage_id: "registry_brief.title".to_string(),
            state: CoverageSubmissionStateV1::Supplied,
            source_kind: CoverageSourceKindV1::UserDeclaration,
            value: Some(json!("Registry Brief Proof")),
            specificity: CoverageSpecificityV1::Exact,
            confidence: CoverageConfidenceV1::High,
            contradiction_refs: Vec::new(),
        },
        CoverageSubmissionV1 {
            coverage_id: "registry_brief.summary".to_string(),
            state: CoverageSubmissionStateV1::Supplied,
            source_kind: CoverageSourceKindV1::UserDeclaration,
            value: Some(json!("A reusable registry brief")),
            specificity: CoverageSpecificityV1::Concrete,
            confidence: CoverageConfidenceV1::High,
            contradiction_refs: Vec::new(),
        },
    ];

    let evaluation = evaluate_coverage(
        &context,
        definition,
        AcquisitionModeV1::Express,
        None,
        &submissions,
    )
    .expect("complete evaluation");
    assert_eq!(
        evaluation.normalized_content,
        json!({
            "summary": "A reusable registry brief",
            "title": "Registry Brief Proof"
        })
    );
    assert_eq!(
        evaluation.evaluation_fingerprint.as_str(),
        "sha256:7c002ab8916ab288ef84ac846b3430b2cf0d89aaacaa26c466b1feaa4feb3150"
    );

    let mut missing = submissions;
    missing.pop();
    let error = evaluate_coverage(
        &context,
        definition,
        AcquisitionModeV1::Express,
        None,
        &missing,
    )
    .unwrap_err();
    assert_eq!(error.kind(), ArtifactIntakeErrorKindV1::MissingCoverage);
}

#[test]
fn blocking_coverage_is_retained_as_a_bounded_auditable_result() {
    let registry = intake_registry();
    let definition = registry
        .definition(&exact("example.intake.registry-brief@1.0.0"))
        .unwrap();
    let mut context_input = operation_context().into_input();
    context_input.intake_definition_fingerprint = Some(definition.definition_fingerprint().clone());
    context_input
        .resolved_definitions
        .iter_mut()
        .find(|binding| binding.definition_ref == *definition.exact_ref())
        .unwrap()
        .definition_fingerprint = definition.definition_fingerprint().clone();
    let context = ArtifactOperationContextV1::new(context_input).unwrap();
    let submissions = vec![
        CoverageSubmissionV1 {
            coverage_id: "registry_brief.title".into(),
            state: CoverageSubmissionStateV1::KnownUnknown,
            source_kind: CoverageSourceKindV1::UserDeclaration,
            value: None,
            specificity: CoverageSpecificityV1::Concrete,
            confidence: CoverageConfidenceV1::High,
            contradiction_refs: Vec::new(),
        },
        CoverageSubmissionV1 {
            coverage_id: "registry_brief.summary".into(),
            state: CoverageSubmissionStateV1::Contradicted,
            source_kind: CoverageSourceKindV1::UserDeclaration,
            value: None,
            specificity: CoverageSpecificityV1::Concrete,
            confidence: CoverageConfidenceV1::High,
            contradiction_refs: vec!["evidence://one".into()],
        },
    ];

    let evaluation = evaluate_coverage(
        &context,
        definition,
        AcquisitionModeV1::Express,
        None,
        &submissions,
    )
    .expect("blocked evaluation is a result, not a collapsed error");
    assert_eq!(evaluation.outcome, CoverageEvaluationOutcomeV1::Blocked);
    assert_eq!(evaluation.normalized_content, json!({}));
    assert!(evaluation.field_sources.is_empty());
    assert!(evaluation
        .coverage_results
        .iter()
        .all(|row| row.evaluation == CoverageEvaluationDispositionV1::Blocked));
    assert_eq!(evaluation.coverage_results[1].contradiction_refs.len(), 1);
}

#[test]
fn canonical_yaml_is_utf8_lf_deterministic_and_round_trips_the_json_model() {
    let left = json!({
        "title": "Registry Brief Proof",
        "summary": "A reusable registry brief"
    });
    let right = json!({
        "summary": "A reusable registry brief",
        "title": "Registry Brief Proof"
    });
    let expected = b"summary: \"A reusable registry brief\"\ntitle: \"Registry Brief Proof\"\n";

    let left_bytes = canonical_yaml_bytes(&left).expect("left canonical yaml");
    let right_bytes = canonical_yaml_bytes(&right).expect("right canonical yaml");
    assert_eq!(left_bytes, expected);
    assert_eq!(right_bytes, expected);
    assert_eq!(
        DefinitionFingerprint::from_bytes(&left_bytes).as_str(),
        "sha256:698d2414185110592a993896e8239f4a324a41e6293a3f77b1ec7538436ab63d"
    );
    assert_eq!(parse_canonical_yaml(&left_bytes).expect("round trip"), left);
}

fn shipped_request() -> ProfileSelectionRequest {
    let names = [
        "project-authority",
        "project-context",
        "environment-context",
        "work-specification",
        "decision-record",
        "risk-record",
    ];
    ProfileSelectionRequest {
        selected_profile_ref: exact("handbook.profile.shipped-root@1.0.0"),
        profile_sources: vec![builtin("handbook.profile.shipped-root@1.0.0")],
        stable_role_registry_sources: vec![builtin("handbook.roles.core@1.1.0")],
        schema_entry_sources: names
            .map(|name| builtin(&format!("handbook.schemas.artifacts.{name}@1.0.0")))
            .to_vec(),
        artifact_kind_sources: names
            .map(|name| builtin(&format!("handbook.artifact-kind.{name}@1.0.0")))
            .to_vec(),
        semantic_capability_sources: vec![builtin(
            "handbook.capabilities.constitutional-root@1.0.0",
        )],
        semantic_validator_sources: vec![builtin(
            "handbook.semantic-validation.constitutional-root@1.0.0",
        )],
        project_condition_sources: vec![builtin(
            "handbook.condition.project.managed-operational-surface@1.0.0",
        )],
        vocabulary_sources: vec![builtin("handbook.vocabulary.shipped-root@1.0.0")],
        context_resolution_sources: vec![builtin("handbook.context-resolution.shipped-root@1.0.0")],
        context_resolution_policy_sources: vec![
            builtin("handbook.mutation-matcher.core@1.0.0"),
            builtin("handbook.resolution-escalation.core@1.0.0"),
            builtin("handbook.memory-promotion.core@1.0.0"),
        ],
        intake_definition_sources: Vec::new(),
        allowed_schema_roots: vec!["definitions/schemas".to_string()],
    }
}

#[test]
fn generic_operation_service_routes_only_by_stable_operation_and_registered_target() {
    let profile =
        resolve_profile_selection(Path::new(env!("CARGO_MANIFEST_DIR")), shipped_request())
            .expect("shipped profile");
    let registry = ResolvedArtifactRegistry::from_profile(&profile).expect("artifact registry");
    let intake_registry = ArtifactIntakeRegistry::empty();
    let context = ArtifactOperationContextV1::resolve(
        &registry,
        &intake_registry,
        &exact("handbook.artifact-kind.project-context@1.0.0"),
        &SymbolicId::parse("project_context").expect("instance"),
        ArtifactOperationAuthorityV1 {
            repository_identity_fingerprint: fp(
                "sha256:1111111111111111111111111111111111111111111111111111111111111111",
            ),
            selection_fingerprint: fp(
                "sha256:2222222222222222222222222222222222222222222222222222222222222222",
            ),
            selected_profile_definition_fingerprint: fp(
                "sha256:3333333333333333333333333333333333333333333333333333333333333333",
            ),
            descriptor_fingerprint: fp(
                "sha256:4444444444444444444444444444444444444444444444444444444444444444",
            ),
        },
    )
    .expect("resolved context");
    let service = ArtifactOperationServiceV1::new(&registry, &intake_registry, &context)
        .expect("operation service");
    let bytes = br#"schema_id: handbook.artifact.project-context
schema_version: "1.0"
record_id: project.context
summary: "A bounded project context"
system_boundaries: ["repository"]
ownership: ["maintainers"]
authoritative_references: []
known_unknowns: []
"#;

    let validated = service
        .validate_from_bytes(bytes)
        .expect("registered structural validation");
    assert_eq!(validated.operation_id.as_str(), "artifact.validate");
    assert!(validated.structural_errors.is_empty());
    assert_eq!(
        service.intake_definition().unwrap_err().kind(),
        ArtifactOperationErrorKindV1::NotApplicable
    );
    assert_eq!(
        ArtifactOperationIdV1::KindList.as_str(),
        "artifact.kind.list"
    );
    assert_eq!(
        ArtifactOperationIdV1::CoverageEvaluate.as_str(),
        "intake.coverage.evaluate"
    );

    let invalid = String::from_utf8(bytes.to_vec())
        .expect("utf8 fixture")
        .replace("A bounded project context", "");
    let error = service.validate_from_bytes(invalid.as_bytes()).unwrap_err();
    assert_eq!(error.kind(), ArtifactOperationErrorKindV1::Structural);
}
