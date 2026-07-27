use handbook_engine::artifact_intake::{
    CoverageConfidenceV1, CoverageEvaluationOutcomeV1, CoverageSourceKindV1, CoverageSpecificityV1,
    CoverageSubmissionStateV1, CoverageSubmissionV1,
};
use handbook_engine::artifact_intake_registry::{
    AcquisitionModeV1, ArtifactRegistrationErrorKindV1, RepositoryProfileSelectionV1,
};
use handbook_engine::artifact_repository::{
    ArtifactRepositoryErrorKindV1, ArtifactRepositoryV1, ArtifactTargetV1,
};
use handbook_engine::{
    parse_definition_yaml, resolve_profile_selection, ArtifactInstanceRegistry,
    DefinitionFingerprint, DefinitionSource, ExactDefinitionRef,
    RepositoryInvocationIdentityServiceV1, RequirednessMode, ResolvedInstanceProfile, SymbolicId,
};
use serde_json::{json, Value};
use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

const KIND_REF: &str = "handbook.artifact-kind.risk-record@1.1.0";
const INTAKE_REF: &str = "handbook.intake.risk-record@1.0.0";
const RENDERER_REF: &str = "handbook.renderer.risk-record-review-markdown@1.0.0";
const SCHEMA_REF: &str = "handbook.schemas.artifacts.risk-record@1.0.0";

fn exact(value: &str) -> ExactDefinitionRef {
    ExactDefinitionRef::parse(value).expect("exact definition ref")
}

fn fixture_root() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/hcm_2_4_risk_record")
}

fn fixture_selection() -> RepositoryProfileSelectionV1 {
    let selection_bytes = fs::read(fixture_root().join(".handbook/profile-selection.json"))
        .expect("P5 Risk Record repository selection fixture");
    RepositoryProfileSelectionV1::from_json_bytes(&selection_bytes)
        .expect("valid P5 repository selection")
}

fn fixture_selection_value() -> Value {
    serde_json::from_slice(
        &fs::read(fixture_root().join(".handbook/profile-selection.json"))
            .expect("P5 Risk Record repository selection fixture"),
    )
    .expect("P5 selection JSON")
}

fn shipped_profile() -> ResolvedInstanceProfile {
    let mut request = fixture_selection().profile_request();
    request.selected_profile_ref = exact("handbook.profile.shipped-root@1.2.0");
    request.profile_sources.retain(|binding| {
        matches!(&binding.source, DefinitionSource::BuiltIn(reference)
            if reference.as_str() == "handbook.profile.shipped-root@1.2.0")
    });
    resolve_profile_selection(fixture_root(), request).expect("shipped-root 1.2 source closure")
}

fn selected_profile() -> ResolvedInstanceProfile {
    resolve_profile_selection(fixture_root(), fixture_selection().profile_request())
        .expect("P5 profile resolves without runtime changes")
}

fn copy_tree(source: &Path, target: &Path) {
    fs::create_dir_all(target).expect("create fixture directory");
    for entry in fs::read_dir(source).expect("read fixture directory") {
        let entry = entry.expect("fixture entry");
        let source_path = entry.path();
        let target_path = target.join(entry.file_name());
        if source_path.is_dir() {
            copy_tree(&source_path, &target_path);
        } else {
            fs::copy(source_path, target_path).expect("copy fixture file");
        }
    }
}

fn open_fixture_repository() -> (tempfile::TempDir, ArtifactRepositoryV1) {
    let repo = tempfile::tempdir().expect("temporary repository");
    copy_tree(&fixture_root(), repo.path());
    RepositoryInvocationIdentityServiceV1::new()
        .initialize_for_setup(repo.path())
        .expect("repository identity");
    let repository = ArtifactRepositoryV1::open(repo.path()).expect("P5 artifact repository");
    (repo, repository)
}

fn risk_target() -> ArtifactTargetV1 {
    ArtifactTargetV1::parse(KIND_REF, "risk_record").expect("Risk Record target")
}

fn initial_risk_content() -> Value {
    json!({
        "schema_id": "handbook.artifact.risk-record",
        "schema_version": "1.0",
        "record_id": "example.record.risk",
        "uncertainty": "Delivery may slip",
        "evidence_refs": [],
        "owner": "Team",
        "treatment": "Track and mitigate",
        "status": "open",
        "review_basis": ["example.reference.review"],
    })
}

fn supplied(
    coverage_id: &str,
    value: Value,
    specificity: CoverageSpecificityV1,
) -> CoverageSubmissionV1 {
    CoverageSubmissionV1 {
        coverage_id: coverage_id.to_owned(),
        state: CoverageSubmissionStateV1::Supplied,
        source_kind: CoverageSourceKindV1::UserDeclaration,
        value: Some(value),
        specificity,
        confidence: CoverageConfidenceV1::High,
        contradiction_refs: Vec::new(),
    }
}

fn risk_submissions(content: &Value) -> Vec<CoverageSubmissionV1> {
    [
        ("schema_id", CoverageSpecificityV1::Exact),
        ("schema_version", CoverageSpecificityV1::Exact),
        ("record_id", CoverageSpecificityV1::Exact),
        ("uncertainty", CoverageSpecificityV1::Concrete),
        ("evidence_refs", CoverageSpecificityV1::Exact),
        ("owner", CoverageSpecificityV1::Concrete),
        ("treatment", CoverageSpecificityV1::Concrete),
        ("status", CoverageSpecificityV1::Exact),
        ("review_basis", CoverageSpecificityV1::Exact),
    ]
    .into_iter()
    .map(|(field, specificity)| {
        supplied(
            &format!("risk_record.{field}"),
            content[field].clone(),
            specificity,
        )
    })
    .collect()
}

#[test]
fn risk_record_fixture_is_repository_selected() {
    let selection = fixture_selection();

    assert_eq!(
        selection.profile_request().selected_profile_ref.as_str(),
        "example.profile.hcm-2-4-risk-record@1.0.0"
    );
}

#[test]
fn risk_record_fixture_resolves_through_the_generic_profile_selector() {
    let fixture_root = fixture_root();
    let profile = selected_profile();
    assert_eq!(
        profile.exact_ref().as_str(),
        "example.profile.hcm-2-4-risk-record@1.0.0"
    );
    assert!(fixture_root.join(".handbook/records/risk.yaml").is_file());
}

#[test]
fn risk_record_profile_fingerprint_replays_its_complete_typed_closure() {
    let fixture_root = fixture_root();
    let shipped = shipped_profile();

    let profile_path =
        fixture_root.join(".handbook/definitions/profiles/risk-record-root-1.0.0.yaml");
    let mut profile: Value =
        serde_json::from_slice(&fs::read(profile_path).expect("P5 profile bytes"))
            .expect("P5 profile JSON");
    let supplied = profile["profile_fingerprint"]
        .as_str()
        .expect("profile fingerprint")
        .to_owned();
    let definitions = profile["artifact_instances"]
        .as_array()
        .expect("artifact instances");
    let condition = shipped
        .project_condition_registry()
        .definition(&exact(
            "handbook.condition.project.managed-operational-surface@1.0.0",
        ))
        .expect("selected project condition");
    let descriptors = ArtifactInstanceRegistry::resolve(
        definitions,
        shipped.artifact_kind_registry(),
        &[condition],
    )
    .expect("exact descriptor closure");

    let mut dependencies = vec![
        json!({
            "definition_class": "profile",
            "reference": "handbook.profile.shipped-root@1.2.0",
            "fingerprint": "sha256:63cd999c95efc3fe65ae3514c2915b5d1457290211cf6da7b57b2cd75bafaf83",
        }),
        json!({
            "definition_class": "artifact_instance_registry",
            "reference": "example.profile.hcm-2-4-risk-record@1.0.0",
            "fingerprint": descriptors.fingerprint().as_str(),
        }),
    ];
    for reference in profile["schema_registry_sources"]
        .as_array()
        .expect("schema sources")
    {
        let reference = exact(reference.as_str().expect("schema ref"));
        let entry = shipped
            .artifact_kind_registry()
            .schema_registry()
            .entry(&reference)
            .expect("schema entry");
        dependencies.push(json!({
            "definition_class": "schema_entry",
            "reference": reference.as_str(),
            "fingerprint": entry.entry_fingerprint().as_str(),
        }));
    }
    for reference in profile["artifact_kind_sources"]
        .as_array()
        .expect("kind sources")
    {
        let reference = exact(reference.as_str().expect("kind ref"));
        let kind = shipped
            .artifact_kind_registry()
            .kind(&reference)
            .expect("artifact kind");
        dependencies.push(json!({
            "definition_class": "artifact_kind",
            "reference": reference.as_str(),
            "fingerprint": kind.definition_fingerprint().as_str(),
        }));
    }
    dependencies.sort_by(|left, right| {
        (
            left["definition_class"].as_str(),
            left["reference"].as_str(),
        )
            .cmp(&(
                right["definition_class"].as_str(),
                right["reference"].as_str(),
            ))
    });
    profile
        .as_object_mut()
        .expect("profile object")
        .remove("profile_fingerprint");
    let computed = DefinitionFingerprint::from_json_value(
        &json!({"definition": profile, "dependencies": dependencies}),
    )
    .expect("profile closure fingerprint");

    assert_eq!(supplied, computed.as_str());
}

#[test]
fn exact_descriptor_and_definition_sources_are_selected_without_root_expansion() {
    let profile = selected_profile();
    let parent = shipped_profile();
    let risk = profile
        .artifact_instances()
        .instance(&SymbolicId::parse("risk_record").expect("instance id"))
        .expect("selected Risk Record descriptor");

    assert_eq!(risk.kind_ref().as_str(), KIND_REF);
    assert_eq!(risk.role_ref(), None);
    assert_eq!(risk.canonical_path(), ".handbook/records/risk.yaml");
    assert_eq!(
        risk.intake_definition_ref().expect("intake").as_str(),
        INTAKE_REF
    );
    assert_eq!(
        risk.renderer_definition_refs()
            .iter()
            .map(ExactDefinitionRef::as_str)
            .collect::<Vec<_>>(),
        [RENDERER_REF]
    );
    assert_eq!(risk.requiredness_mode(), RequirednessMode::Always);
    assert!(risk.condition_ref().is_none());
    assert!(risk.capability_refs().is_empty());
    assert!(risk.dependencies().is_empty());
    assert!(risk.lifecycle_policy_ref().is_none());
    assert!(risk.projection_definition_refs().is_empty());
    assert!(risk.validation_overlay_refs().is_empty());
    assert!(risk.extensions().is_empty());

    let parent_ids = parent
        .artifact_instances()
        .ids()
        .into_iter()
        .collect::<BTreeSet<_>>();
    let selected_ids = profile
        .artifact_instances()
        .ids()
        .into_iter()
        .collect::<BTreeSet<_>>();
    assert_eq!(
        selected_ids
            .difference(&parent_ids)
            .copied()
            .collect::<Vec<_>>(),
        ["risk_record"]
    );
    assert!(!parent_ids.contains("risk_record"));

    let selection = fixture_selection_value();
    assert_eq!(
        selection["profile_sources"][1]["source"],
        json!({
            "kind": "repository_path",
            "path": ".handbook/definitions/profiles/risk-record-root-1.0.0.yaml",
        })
    );
    for (field, reference) in [
        ("schema_entry_sources", SCHEMA_REF),
        ("artifact_kind_sources", KIND_REF),
        ("intake_definition_sources", INTAKE_REF),
    ] {
        let source = selection[field]
            .as_array()
            .expect("definition sources")
            .iter()
            .find(|source| source["exact_ref"] == reference)
            .expect("exact Risk Record source");
        assert_eq!(source["source"], json!({"kind": "built_in"}));
    }
}

#[test]
fn repository_selected_schema_and_intake_cover_all_supported_modes() {
    let (_repo, repository) = open_fixture_repository();
    let target = risk_target();
    let kind_ref = exact(KIND_REF);
    let instance_id = SymbolicId::parse("risk_record").expect("instance id");
    let context = repository
        .operation_context(&kind_ref, &instance_id)
        .expect("operation context");
    assert_eq!(context.kind_ref().as_str(), KIND_REF);
    assert_eq!(
        context.kind_fingerprint().as_str(),
        "sha256:e30bea03a5d3d7b14f56da8f7ce62eb0dfbfe040fccd79782e0d36004d79dd5a"
    );
    assert_eq!(context.schema_ref().as_str(), SCHEMA_REF);
    assert_eq!(
        context.schema_entry_fingerprint().as_str(),
        "sha256:8d8dc18add39d768cc6c22416c61db83a0e60004d5539332cdb07797cdf37cd0"
    );
    assert_eq!(
        context.schema_document_fingerprint().as_str(),
        "sha256:df90eec6f85652603c19e56c92870b77599b59a7011110cab91b443d6eb2b01c"
    );
    assert_eq!(
        context.intake_definition_ref().expect("intake").as_str(),
        INTAKE_REF
    );
    assert_eq!(
        context
            .intake_definition_fingerprint()
            .expect("intake fingerprint")
            .as_str(),
        "sha256:5bdce9d0a7688ace79b2467f70e7ab7b5b82548cc7bb55695ceca37c550319d4"
    );

    let intake = repository
        .intake_definition(&target)
        .expect("selected intake definition");
    assert_eq!(intake.exact_ref().as_str(), INTAKE_REF);
    assert_eq!(intake.artifact_kind_ref().as_str(), KIND_REF);
    assert_eq!(intake.candidate_schema_ref().as_str(), SCHEMA_REF);
    assert_eq!(
        intake.supported_modes(),
        [
            AcquisitionModeV1::GuidedAdaptive,
            AcquisitionModeV1::Express,
            AcquisitionModeV1::AgentAssisted,
        ]
    );
    assert_eq!(
        intake
            .coverage()
            .iter()
            .map(|coverage| coverage.coverage_id())
            .collect::<Vec<_>>(),
        [
            "risk_record.schema_id",
            "risk_record.schema_version",
            "risk_record.record_id",
            "risk_record.uncertainty",
            "risk_record.evidence_refs",
            "risk_record.owner",
            "risk_record.treatment",
            "risk_record.status",
            "risk_record.review_basis",
        ]
    );

    let current = repository
        .current_artifact_fingerprint(&target)
        .expect("current fingerprint")
        .expect("canonical Risk Record");
    let submissions = risk_submissions(&initial_risk_content());
    for mode in [
        AcquisitionModeV1::GuidedAdaptive,
        AcquisitionModeV1::Express,
        AcquisitionModeV1::AgentAssisted,
    ] {
        let evaluated = repository
            .evaluate_intake(
                &kind_ref,
                &instance_id,
                mode,
                Some(current.clone()),
                &submissions,
            )
            .expect("schema-backed intake evaluation");
        assert_eq!(evaluated.outcome, CoverageEvaluationOutcomeV1::Complete);
        assert_eq!(evaluated.normalized_content, initial_risk_content());
    }
}

#[test]
fn generic_read_and_validate_retain_the_exact_canonical_source_bytes() {
    let (repo, repository) = open_fixture_repository();
    let canonical_path = repo.path().join(".handbook/records/risk.yaml");
    let canonical_bytes = fs::read(&canonical_path).expect("canonical Risk Record bytes");
    let kind_ref = exact(KIND_REF);
    let instance_id = SymbolicId::parse("risk_record").expect("instance id");
    let target = risk_target();

    assert_eq!(
        repository
            .canonical_path(&target)
            .expect("safe canonical path"),
        ".handbook/records/risk.yaml"
    );
    let read = repository
        .read(&kind_ref, &instance_id)
        .expect("descriptor-selected safe read");
    assert_eq!(read.canonical_path, ".handbook/records/risk.yaml");
    assert_eq!(read.content, initial_risk_content());
    assert_eq!(
        read.artifact_fingerprint,
        DefinitionFingerprint::from_bytes(&canonical_bytes)
    );
    let validated = repository
        .validate(&kind_ref, &instance_id)
        .expect("selected exact structural schema");
    assert!(validated.structural_errors.is_empty());
    assert_eq!(validated.content, initial_risk_content());
    assert_eq!(validated.artifact_fingerprint, read.artifact_fingerprint);
}

#[test]
fn generic_risk_record_mutation_derives_exact_coverage_tokens() {
    let (repo, repository) = open_fixture_repository();
    let target = risk_target();
    let current = repository
        .current_artifact_fingerprint(&target)
        .expect("current fingerprint")
        .expect("canonical Risk Record");
    let updated = json!({
        "schema_id": "handbook.artifact.risk-record",
        "schema_version": "1.0",
        "record_id": "example.record.risk",
        "uncertainty": "Delivery may slip after dependency churn",
        "evidence_refs": [],
        "owner": "Delivery Team",
        "treatment": "Mitigate before release",
        "status": "monitoring",
        "review_basis": ["example.reference.review"],
    });
    let intake_request = serde_json::to_vec(&json!({
        "idempotency_key": "p5_risk_blocker_000001",
        "acquisition_mode": "express",
        "expected_current_artifact_fingerprint": current.as_str(),
        "coverage_submissions": risk_submissions(&updated),
    }))
    .expect("intake request");

    let intake = handbook_engine::artifact_mutation::ArtifactMutationServiceV1::intake_append(
        repo.path(),
        KIND_REF,
        "risk_record",
        &intake_request,
    )
    .expect("underscore-bearing coverage IDs derive exact lineage tokens");
    let intent_path = repo
        .path()
        .join(".handbook/state/transactions/intake-records")
        .join(format!(
            "{}.committed/intent.json",
            intake.result.transaction_id
        ));
    let intent: Value =
        serde_json::from_slice(&fs::read(intent_path).expect("committed Risk intake intent"))
            .expect("parse committed Risk intake intent");
    let tokens = intent["outputs"]
        .as_array()
        .expect("Risk intake outputs")
        .iter()
        .map(|output| output["token"].as_str().expect("Risk output token"))
        .collect::<Vec<_>>();
    assert_eq!(
        tokens,
        vec![
            "schema-id-value",
            "schema-version-value",
            "record-id-value",
            "uncertainty-value",
            "evidence-refs-value",
            "owner-value",
            "treatment-value",
            "status-value",
            "review-basis-value",
            "intake-record",
        ]
    );
}

#[test]
fn generic_mutation_promotes_then_reads_and_validates_new_real_bytes() {
    let (repo, repository) = open_fixture_repository();
    let target = risk_target();
    let original_bytes =
        fs::read(repo.path().join(".handbook/records/risk.yaml")).expect("original bytes");
    let current = repository
        .current_artifact_fingerprint(&target)
        .expect("current fingerprint")
        .expect("canonical Risk Record");
    assert_eq!(current, DefinitionFingerprint::from_bytes(&original_bytes));

    let updated = json!({
        "schema_id": "handbook.artifact.risk-record",
        "schema_version": "1.0",
        "record_id": "example.record.risk",
        "uncertainty": "Delivery may slip after dependency churn",
        "evidence_refs": [],
        "owner": "Delivery Team",
        "treatment": "Mitigate before release",
        "status": "monitoring",
        "review_basis": ["example.reference.review"],
    });
    let intake_request = serde_json::to_vec(&json!({
        "idempotency_key": "p5_risk_intake_000001",
        "acquisition_mode": "express",
        "expected_current_artifact_fingerprint": current.as_str(),
        "coverage_submissions": risk_submissions(&updated),
    }))
    .expect("intake request");
    let intake = handbook_engine::artifact_mutation::ArtifactMutationServiceV1::intake_append(
        repo.path(),
        KIND_REF,
        "risk_record",
        &intake_request,
    )
    .expect("generic intake append");
    let intake_output = intake
        .result
        .authoritative_outputs
        .first()
        .expect("retained intake record");
    let preview =
        handbook_engine::artifact_mutation::ArtifactMutationServiceV1::candidate_validate(
            repo.path(),
            KIND_REF,
            "risk_record",
            &intake_output.relative_ref,
            &intake_output.fingerprint,
            Some(current.as_str()),
        )
        .expect("generic candidate validation");
    let candidate_request = serde_json::to_vec(&json!({
        "idempotency_key": "p5_risk_candidate_0001",
        "intake_record_ref": intake_output.relative_ref,
        "intake_record_fingerprint": intake_output.fingerprint,
        "expected_candidate_fingerprint": preview.candidate_fingerprint,
    }))
    .expect("candidate request");
    let candidate =
        handbook_engine::artifact_mutation::ArtifactMutationServiceV1::candidate_append(
            repo.path(),
            KIND_REF,
            "risk_record",
            &candidate_request,
        )
        .expect("generic candidate append");
    let candidate_output = candidate
        .result
        .authoritative_outputs
        .first()
        .expect("retained candidate");
    let promotion_request = serde_json::to_vec(&json!({
        "idempotency_key": "p5_risk_promotion_0001",
        "candidate_ref": candidate_output.relative_ref,
        "candidate_fingerprint": candidate_output.fingerprint,
        "expected_current_artifact_fingerprint": current.as_str(),
    }))
    .expect("promotion request");
    handbook_engine::artifact_mutation::ArtifactMutationServiceV1::promote(
        repo.path(),
        KIND_REF,
        "risk_record",
        &promotion_request,
    )
    .expect("generic candidate promotion");

    let promoted_bytes =
        fs::read(repo.path().join(".handbook/records/risk.yaml")).expect("promoted bytes");
    assert_ne!(promoted_bytes, original_bytes);
    let reopened = ArtifactRepositoryV1::open(repo.path()).expect("reopen promoted repository");
    let read = reopened
        .read(&exact(KIND_REF), &SymbolicId::parse("risk_record").unwrap())
        .expect("read promoted canonical Risk Record");
    assert_eq!(read.content, updated);
    assert_eq!(
        read.artifact_fingerprint,
        DefinitionFingerprint::from_bytes(&promoted_bytes)
    );
    let validated = reopened
        .validate(&exact(KIND_REF), &SymbolicId::parse("risk_record").unwrap())
        .expect("validate promoted canonical Risk Record");
    assert!(validated.structural_errors.is_empty());
    assert_eq!(validated.content, updated);
}

#[test]
fn malformed_duplicate_unknown_and_unsafe_selection_records_are_refused() {
    assert_eq!(
        RepositoryProfileSelectionV1::from_json_bytes(b"{")
            .expect_err("malformed selection")
            .kind(),
        ArtifactRegistrationErrorKindV1::InvalidRecord
    );

    let mut unknown = fixture_selection_value();
    unknown["unknown_field"] = json!(true);
    assert_eq!(
        RepositoryProfileSelectionV1::from_json_bytes(
            &serde_json::to_vec(&unknown).expect("unknown-field selection")
        )
        .expect_err("unknown selection field")
        .kind(),
        ArtifactRegistrationErrorKindV1::InvalidRecord
    );

    let mut duplicate = fixture_selection_value();
    let duplicate_source = duplicate["profile_sources"][0].clone();
    duplicate["profile_sources"]
        .as_array_mut()
        .expect("profile sources")
        .push(duplicate_source);
    assert_eq!(
        RepositoryProfileSelectionV1::from_json_bytes(
            &serde_json::to_vec(&duplicate).expect("duplicate selection")
        )
        .expect_err("duplicate profile source")
        .kind(),
        ArtifactRegistrationErrorKindV1::DuplicateSourceIdentity
    );

    let mut unsafe_path = fixture_selection_value();
    unsafe_path["profile_sources"][1]["source"]["path"] = json!("../risk-record-root-1.0.0.yaml");
    assert_eq!(
        RepositoryProfileSelectionV1::from_json_bytes(
            &serde_json::to_vec(&unsafe_path).expect("unsafe-path selection")
        )
        .expect_err("unsafe repository source path")
        .kind(),
        ArtifactRegistrationErrorKindV1::InvalidRepositoryPath
    );
}

#[test]
fn every_risk_descriptor_widening_is_refused_by_the_existing_registry() {
    let shipped = shipped_profile();
    let profile: Value = serde_json::from_slice(
        &fs::read(
            fixture_root().join(".handbook/definitions/profiles/risk-record-root-1.0.0.yaml"),
        )
        .expect("P5 profile"),
    )
    .expect("P5 profile JSON");
    let condition = shipped
        .project_condition_registry()
        .definition(&exact(
            "handbook.condition.project.managed-operational-surface@1.0.0",
        ))
        .expect("selected project condition");

    for (name, mutation) in [
        ("role", ("role_ref", json!("project_context"))),
        ("path", ("canonical_path", json!("../risk.yaml"))),
        ("intake", ("intake_definition_ref", Value::Null)),
        (
            "renderer",
            (
                "renderer_definition_refs",
                json!(["handbook.renderer.decision-record-review-markdown@1.0.0"]),
            ),
        ),
        (
            "Projection",
            (
                "projection_definition_refs",
                json!(["example.projection.risk-record@1.0.0"]),
            ),
        ),
        (
            "overlay",
            (
                "validation_overlay_refs",
                json!(["example.overlay.risk-record@1.0.0"]),
            ),
        ),
        ("extensions", ("extensions", json!({"widened": true}))),
    ] {
        let mut descriptors = profile["artifact_instances"]
            .as_array()
            .expect("artifact instances")
            .clone();
        let risk = descriptors
            .iter_mut()
            .find(|descriptor| descriptor["id"] == "risk_record")
            .expect("Risk Record descriptor");
        risk[mutation.0] = mutation.1;
        assert!(
            ArtifactInstanceRegistry::resolve(
                &descriptors,
                shipped.artifact_kind_registry(),
                &[condition],
            )
            .is_err(),
            "{name} widening must be refused"
        );
    }

    let mut duplicate = profile["artifact_instances"]
        .as_array()
        .expect("artifact instances")
        .clone();
    duplicate.push(
        duplicate
            .iter()
            .find(|descriptor| descriptor["id"] == "risk_record")
            .expect("Risk Record descriptor")
            .clone(),
    );
    assert!(
        ArtifactInstanceRegistry::resolve(
            &duplicate,
            shipped.artifact_kind_registry(),
            &[condition],
        )
        .is_err(),
        "duplicate descriptor must be refused"
    );

    let mut unknown = profile["artifact_instances"]
        .as_array()
        .expect("artifact instances")
        .clone();
    unknown
        .iter_mut()
        .find(|descriptor| descriptor["id"] == "risk_record")
        .expect("Risk Record descriptor")["unknown_field"] = json!(true);
    assert!(
        ArtifactInstanceRegistry::resolve(&unknown, shipped.artifact_kind_registry(), &[condition])
            .is_err(),
        "unknown descriptor field must be refused"
    );
}

#[test]
fn duplicate_keys_unknown_fields_and_missing_exact_filename_are_refused() {
    let kind_ref = exact(KIND_REF);
    let instance_id = SymbolicId::parse("risk_record").expect("instance id");

    let (duplicate_repo, duplicate_repository) = open_fixture_repository();
    fs::write(
        duplicate_repo.path().join(".handbook/records/risk.yaml"),
        b"schema_id: handbook.artifact.risk-record\nschema_id: handbook.artifact.risk-record\n",
    )
    .expect("duplicate-key Risk Record");
    assert_eq!(
        duplicate_repository
            .read(&kind_ref, &instance_id)
            .expect_err("duplicate YAML key")
            .kind(),
        ArtifactRepositoryErrorKindV1::ArtifactOperation
    );

    let (unknown_repo, unknown_repository) = open_fixture_repository();
    let mut unknown = initial_risk_content();
    unknown["unknown_field"] = json!("not admitted");
    fs::write(
        unknown_repo.path().join(".handbook/records/risk.yaml"),
        serde_yaml_bw::to_string(&unknown).expect("unknown-field YAML"),
    )
    .expect("write unknown-field Risk Record");
    assert_eq!(
        unknown_repository
            .validate(&kind_ref, &instance_id)
            .expect_err("schema unknown field")
            .kind(),
        ArtifactRepositoryErrorKindV1::ArtifactOperation
    );

    let (missing_repo, missing_repository) = open_fixture_repository();
    fs::rename(
        missing_repo.path().join(".handbook/records/risk.yaml"),
        missing_repo
            .path()
            .join(".handbook/records/risk-record.yaml"),
    )
    .expect("install inferred-filename decoy");
    assert_eq!(
        missing_repository
            .read(&kind_ref, &instance_id)
            .expect_err("filename inference is forbidden")
            .kind(),
        ArtifactRepositoryErrorKindV1::ArtifactRead
    );
}

#[test]
fn fixed_renderer_definition_and_golden_are_exact_resolution_free_bytes() {
    let definition_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("definitions");
    let mut renderer = parse_definition_yaml(
        &fs::read(
            definition_root
                .join("renderers/handbook.renderer.risk-record-review-markdown/1.0.0.yaml"),
        )
        .expect("Risk Record renderer"),
    )
    .expect("renderer definition");
    assert_eq!(renderer["resolution_input"], Value::Null);
    assert_eq!(renderer["input_schema_ref"], SCHEMA_REF);
    assert_eq!(
        renderer["implementation_id"],
        "risk-record-review-markdown-v1"
    );
    assert_eq!(
        renderer["determinism_profile"],
        "closed-risk-record-review-markdown-v1"
    );
    assert_eq!(renderer["extensions"], json!({}));
    let supplied = renderer
        .as_object_mut()
        .expect("renderer object")
        .remove("renderer_fingerprint")
        .expect("renderer fingerprint");
    let schema = selected_profile()
        .artifact_kind_registry()
        .schema_registry()
        .entry(&exact(SCHEMA_REF))
        .expect("Risk Record schema entry")
        .clone();
    let computed = DefinitionFingerprint::from_json_value(&json!({
        "definition": renderer,
        "resolved_dependencies": [{
            "definition_fingerprint": schema.entry_fingerprint().as_str(),
            "definition_ref": SCHEMA_REF,
            "dependency_role": "input_schema",
        }],
    }))
    .expect("renderer closure fingerprint");
    assert_eq!(supplied, computed.as_str());
    assert_eq!(
        computed.as_str(),
        "sha256:5cd8a531cfe8d83c078362ed2336d36eb87ee9b6715a364f19211026c6ee274c"
    );

    let goldens: Value = serde_json::from_slice(
        &fs::read(Path::new(env!("CARGO_MANIFEST_DIR")).join(
            "../../docs/specs/handbook-contract-membrane/slices/HCM-2.4/contracts/renderer-goldens-v1.0.json",
        ))
        .expect("renderer goldens"),
    )
    .expect("renderer golden JSON");
    assert_eq!(goldens["external_inputs"], json!([]));
    let golden = goldens["goldens"]
        .as_array()
        .expect("golden array")
        .iter()
        .find(|golden| golden["renderer_ref"] == RENDERER_REF)
        .expect("Risk Record renderer golden");
    assert_eq!(golden["renderer_fingerprint"], computed.as_str());
    assert_eq!(golden["resolution_input"], Value::Null);
    assert_eq!(golden["canonical_input"], initial_risk_content());
    selected_profile()
        .artifact_kind_registry()
        .schema_registry()
        .resolved(&exact(SCHEMA_REF))
        .expect("resolved Risk Record schema")
        .validate_json(&golden["canonical_input"])
        .expect("schema-valid golden input");
    let markdown = golden["expected_markdown_utf8"]
        .as_str()
        .expect("golden Markdown");
    assert_eq!(
        markdown.as_bytes().len() as u64,
        golden["expected_byte_length"]
            .as_u64()
            .expect("byte length")
    );
    assert_eq!(
        DefinitionFingerprint::from_bytes(markdown.as_bytes()).as_str(),
        golden["expected_sha256"].as_str().expect("golden SHA-256")
    );
}

#[test]
fn markdown_decoys_and_broader_surfaces_have_zero_canonical_influence() {
    let (repo, repository) = open_fixture_repository();
    let before = repository
        .read(
            &exact(KIND_REF),
            &SymbolicId::parse("risk_record").expect("instance id"),
        )
        .expect("canonical read before decoys");
    fs::write(
        repo.path().join(".handbook/records/risk.md"),
        b"# Malicious persistent mirror\n",
    )
    .expect("Markdown decoy");
    fs::write(
        repo.path().join(".handbook/records/RISK_RECORD.md"),
        b"# Inferred filename decoy\n",
    )
    .expect("inferred-filename decoy");
    let after = repository
        .read(
            &exact(KIND_REF),
            &SymbolicId::parse("risk_record").expect("instance id"),
        )
        .expect("canonical read after decoys");
    assert_eq!(after.content, before.content);
    assert_eq!(after.artifact_fingerprint, before.artifact_fingerprint);

    let cli = include_str!("../../cli/src/main.rs");
    assert!(!cli.contains("RiskRecord"));
    assert!(!cli.contains("risk-record"));
    assert!(selected_profile()
        .artifact_instances()
        .instance(&SymbolicId::parse("risk_record").unwrap())
        .expect("Risk Record descriptor")
        .projection_definition_refs()
        .is_empty());
}
