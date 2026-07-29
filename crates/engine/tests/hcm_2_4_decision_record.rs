use handbook_engine::artifact_intake::{
    CoverageConfidenceV1, CoverageEvaluationOutcomeV1, CoverageSourceKindV1, CoverageSpecificityV1,
    CoverageSubmissionStateV1, CoverageSubmissionV1,
};
use handbook_engine::artifact_intake_registry::{AcquisitionModeV1, RepositoryProfileSelectionV1};
use handbook_engine::artifact_repository::{
    ArtifactRepositoryErrorKindV1, ArtifactRepositoryV1, ArtifactTargetV1,
};
use handbook_engine::canonical_yaml::canonical_yaml_bytes;
use handbook_engine::{
    parse_definition_yaml, resolve_profile_selection, ArtifactInstanceRegistry,
    DefinitionFingerprint, DefinitionSource, ExactDefinitionRef,
    RepositoryInvocationIdentityServiceV1, ResolvedInstanceProfile, SymbolicId,
};
use serde_json::{json, Value};
use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

const DECISION_KIND_REF: &str = "handbook.artifact-kind.decision-record@1.1.0";
const DECISION_INSTANCE_ID: &str = "decision_record";
const DECISION_RENDERER_REF: &str = "handbook.renderer.decision-record-review-markdown@1.0.0";
const DECISION_SCHEMA_REF: &str = "handbook.schemas.artifacts.decision-record@1.0.0";
const DECISION_PROFILE_REF: &str = "example.profile.hcm-2-4-decision-record@1.0.0";
const DECISION_PROFILE_FINGERPRINT: &str =
    "sha256:2c2d744185a63c328d1363d0df3f50d8a1e3cff0e86f9575e39f69c9a7dba37b";

fn decision_record_fixture_root() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/hcm_2_4_decision_record")
}

fn copy_decision_record_fixture_tree(source: &Path, target: &Path) {
    fs::create_dir_all(target).expect("fixture directory");
    for entry in fs::read_dir(source).expect("fixture tree") {
        let entry = entry.expect("fixture entry");
        let source_path = entry.path();
        let target_path = target.join(entry.file_name());
        if source_path.is_dir() {
            copy_decision_record_fixture_tree(&source_path, &target_path);
        } else {
            fs::copy(source_path, target_path).expect("copy fixture file");
        }
    }
}

fn open_decision_record_fixture_repository() -> (tempfile::TempDir, ArtifactRepositoryV1) {
    let repo = tempfile::tempdir().expect("repository root");
    copy_decision_record_fixture_tree(&decision_record_fixture_root(), repo.path());
    RepositoryInvocationIdentityServiceV1::new()
        .initialize_for_setup(repo.path())
        .expect("repository identity");
    let repository =
        ArtifactRepositoryV1::open(repo.path()).expect("Decision Record repository session");
    (repo, repository)
}

fn decision_target() -> ArtifactTargetV1 {
    ArtifactTargetV1::parse(DECISION_KIND_REF, DECISION_INSTANCE_ID)
        .expect("Decision Record target")
}

fn decision_fixture_selection() -> RepositoryProfileSelectionV1 {
    let selection_bytes =
        fs::read(decision_record_fixture_root().join(".handbook/profile-selection.json"))
            .expect("Decision Record repository selection fixture");
    RepositoryProfileSelectionV1::from_json_bytes(&selection_bytes)
        .expect("valid Decision Record repository selection")
}

fn shipped_root_decision_profile() -> ResolvedInstanceProfile {
    let mut request = decision_fixture_selection().profile_request();
    request.selected_profile_ref =
        ExactDefinitionRef::parse("handbook.profile.shipped-root@1.2.0").expect("shipped-root ref");
    request.profile_sources.retain(|binding| {
        matches!(&binding.source, DefinitionSource::BuiltIn(reference)
            if reference.as_str() == "handbook.profile.shipped-root@1.2.0")
    });
    resolve_profile_selection(decision_record_fixture_root(), request)
        .expect("shipped-root 1.2 source closure")
}

fn selected_decision_profile() -> ResolvedInstanceProfile {
    resolve_profile_selection(
        decision_record_fixture_root(),
        decision_fixture_selection().profile_request(),
    )
    .expect("Decision Record profile resolves without runtime changes")
}

fn decision_record_candidate_content() -> Value {
    json!({
        "schema_id": "handbook.artifact.decision-record",
        "schema_version": "1.0",
        "record_id": "example.record.decision",
        "context": "The generic path must stay bounded",
        "decision": "Use the admitted descriptor",
        "status": "accepted",
        "consequences": ["The canonical YAML remains authoritative"],
        "supersedes": []
    })
}

fn decision_record_coverage_submissions() -> Vec<CoverageSubmissionV1> {
    [
        (
            "decision_record.schema_id",
            json!("handbook.artifact.decision-record"),
            CoverageSpecificityV1::Exact,
        ),
        (
            "decision_record.schema_version",
            json!("1.0"),
            CoverageSpecificityV1::Exact,
        ),
        (
            "decision_record.record_id",
            json!("example.record.decision"),
            CoverageSpecificityV1::Exact,
        ),
        (
            "decision_record.context",
            json!("The generic path must stay bounded"),
            CoverageSpecificityV1::Concrete,
        ),
        (
            "decision_record.decision",
            json!("Use the admitted descriptor"),
            CoverageSpecificityV1::Concrete,
        ),
        (
            "decision_record.status",
            json!("accepted"),
            CoverageSpecificityV1::Exact,
        ),
        (
            "decision_record.consequences",
            json!(["The canonical YAML remains authoritative"]),
            CoverageSpecificityV1::Concrete,
        ),
        (
            "decision_record.supersedes",
            json!([]),
            CoverageSpecificityV1::Exact,
        ),
    ]
    .into_iter()
    .map(|(coverage_id, value, specificity)| CoverageSubmissionV1 {
        coverage_id: coverage_id.to_owned(),
        state: CoverageSubmissionStateV1::Supplied,
        source_kind: CoverageSourceKindV1::UserDeclaration,
        value: Some(value),
        specificity,
        confidence: CoverageConfidenceV1::High,
        contradiction_refs: Vec::new(),
    })
    .collect()
}

fn decision_record_fixture_profile_fingerprint(
    fixture_root: &Path,
    selection_bytes: &[u8],
) -> DefinitionFingerprint {
    let mut root_selection: Value =
        serde_json::from_slice(selection_bytes).expect("selection JSON");
    root_selection["selected_profile_ref"] = json!("handbook.profile.shipped-root@1.2.0");
    root_selection["profile_sources"] = json!([{
        "exact_ref": "handbook.profile.shipped-root@1.2.0",
        "source": {"kind": "built_in"}
    }]);
    let root_selection = RepositoryProfileSelectionV1::from_json_bytes(
        &serde_json::to_vec(&root_selection).expect("root selection JSON"),
    )
    .expect("root selection");
    let root = resolve_profile_selection(fixture_root, root_selection.profile_request())
        .expect("shipped-root 1.2 closure");

    let profile_path =
        fixture_root.join(".handbook/definitions/profiles/decision-record-root-1.0.0.yaml");
    let mut definition =
        parse_definition_yaml(&fs::read(profile_path).expect("Decision Record profile bytes"))
            .expect("Decision Record profile");
    let supplied = definition
        .as_object_mut()
        .expect("profile object")
        .remove("profile_fingerprint")
        .expect("profile fingerprint");
    let descriptors = ArtifactInstanceRegistry::resolve(
        definition["artifact_instances"]
            .as_array()
            .expect("artifact instances"),
        root.artifact_kind_registry(),
        &[],
    )
    .expect("descriptor closure");
    let computed = DefinitionFingerprint::from_json_value(&json!({
        "definition": definition,
        "dependencies": [
            {
                "definition_class": "artifact_instance_registry",
                "reference": "example.profile.hcm-2-4-decision-record@1.0.0",
                "fingerprint": descriptors.fingerprint().as_str()
            },
            {
                "definition_class": "profile",
                "reference": "handbook.profile.shipped-root@1.2.0",
                "fingerprint": root.selected_profile_definition_fingerprint().as_str()
            }
        ]
    }))
    .expect("profile fingerprint");
    assert_eq!(supplied, json!(computed.as_str()));
    computed
}

#[test]
fn repository_selected_decision_record_fixture_resolves_and_reads_real_bytes() {
    let fixture_root = decision_record_fixture_root();
    let selection_path = fixture_root.join(".handbook/profile-selection.json");
    let selection_bytes =
        fs::read(&selection_path).expect("Decision Record selection fixture must exist");
    assert_eq!(
        decision_record_fixture_profile_fingerprint(&fixture_root, &selection_bytes).as_str(),
        DECISION_PROFILE_FINGERPRINT
    );
    let selection = RepositoryProfileSelectionV1::from_json_bytes(&selection_bytes)
        .expect("Decision Record selection fixture must decode");
    let profile = resolve_profile_selection(&fixture_root, selection.profile_request())
        .expect("Decision Record profile must resolve");
    assert_eq!(profile.exact_ref().as_str(), DECISION_PROFILE_REF);

    let decision_id = SymbolicId::parse(DECISION_INSTANCE_ID).expect("Decision Record instance");
    let descriptor = profile
        .artifact_instances()
        .instance(&decision_id)
        .expect("selected Decision Record descriptor");
    assert_eq!(descriptor.kind_ref().as_str(), DECISION_KIND_REF);
    assert_eq!(descriptor.role_ref(), None);
    assert_eq!(
        descriptor.canonical_path(),
        ".handbook/records/decision.yaml"
    );
    assert_eq!(
        descriptor
            .intake_definition_ref()
            .expect("Decision Record intake")
            .as_str(),
        "handbook.intake.decision-record@1.0.0"
    );
    assert_eq!(
        descriptor
            .renderer_definition_refs()
            .iter()
            .map(ExactDefinitionRef::as_str)
            .collect::<Vec<_>>(),
        ["handbook.renderer.decision-record-review-markdown@1.0.0"]
    );
    assert!(descriptor.capability_refs().is_empty());
    assert!(descriptor.dependencies().is_empty());
    assert!(descriptor.lifecycle_policy_ref().is_none());
    assert!(descriptor.projection_definition_refs().is_empty());
    assert!(descriptor.validation_overlay_refs().is_empty());
    assert!(descriptor.extensions().is_empty());

    let (repo, repository) = open_decision_record_fixture_repository();
    let canonical_path = repo.path().join(".handbook/records/decision.yaml");
    let retained_bytes = fs::read(&canonical_path).expect("retained Decision Record bytes");
    let expected_content = json!({
        "schema_id": "handbook.artifact.decision-record",
        "schema_version": "1.0",
        "record_id": "example.record.decision",
        "context": "A choice exists",
        "decision": "Choose safety",
        "status": "accepted",
        "consequences": ["More checks"],
        "supersedes": []
    });
    assert_eq!(
        canonical_yaml_bytes(&expected_content).expect("canonical Decision Record"),
        retained_bytes
    );
    let read = repository
        .read(
            &ExactDefinitionRef::parse(DECISION_KIND_REF).expect("kind ref"),
            &decision_id,
        )
        .expect("descriptor-selected safe read");
    assert_eq!(read.canonical_path, ".handbook/records/decision.yaml");
    assert_eq!(read.content, expected_content);
    assert_eq!(
        read.artifact_fingerprint,
        DefinitionFingerprint::from_bytes(&retained_bytes)
    );

    let context = repository
        .operation_context(
            &ExactDefinitionRef::parse(DECISION_KIND_REF).expect("kind ref"),
            &decision_id,
        )
        .expect("Decision Record operation context");
    assert_eq!(context.profile_ref().as_str(), DECISION_PROFILE_REF);
    assert_eq!(context.kind_ref().as_str(), DECISION_KIND_REF);
    assert_eq!(
        context.schema_ref().as_str(),
        "handbook.schemas.artifacts.decision-record@1.0.0"
    );
    assert_eq!(
        context
            .intake_definition_ref()
            .expect("selected intake")
            .as_str(),
        "handbook.intake.decision-record@1.0.0"
    );
    assert!(context.resolved_definitions().iter().any(|binding| {
        binding.definition_ref.as_str() == DECISION_PROFILE_REF
            && binding.definition_fingerprint.as_str() == DECISION_PROFILE_FINGERPRINT
    }));
    repository
        .validate(
            &ExactDefinitionRef::parse(DECISION_KIND_REF).expect("kind ref"),
            &decision_id,
        )
        .expect("selected schema validation");
    assert_eq!(
        repository
            .current_artifact_fingerprint(&decision_target())
            .expect("current fingerprint"),
        Some(DefinitionFingerprint::from_bytes(&retained_bytes))
    );
}

#[test]
fn decision_record_intake_modes_share_the_selected_schema_and_closed_coverage() {
    let (_repo, repository) = open_decision_record_fixture_repository();
    let target = decision_target();
    let definition = repository
        .intake_definition(&target)
        .expect("selected Decision Record intake");
    assert_eq!(
        definition.exact_ref().as_str(),
        "handbook.intake.decision-record@1.0.0"
    );
    assert_eq!(definition.artifact_kind_ref().as_str(), DECISION_KIND_REF);
    assert_eq!(
        definition.candidate_schema_ref().as_str(),
        "handbook.schemas.artifacts.decision-record@1.0.0"
    );
    assert_eq!(
        definition.supported_modes(),
        [
            AcquisitionModeV1::GuidedAdaptive,
            AcquisitionModeV1::Express,
            AcquisitionModeV1::AgentAssisted
        ]
    );
    assert_eq!(definition.coverage().len(), 8);

    let current = repository
        .current_artifact_fingerprint(&target)
        .expect("current artifact")
        .expect("fixture artifact");
    let submissions = decision_record_coverage_submissions();
    for mode in [
        AcquisitionModeV1::GuidedAdaptive,
        AcquisitionModeV1::Express,
        AcquisitionModeV1::AgentAssisted,
    ] {
        let evaluation = repository
            .evaluate_intake(
                target.kind_ref(),
                target.instance_id(),
                mode,
                Some(current.clone()),
                &submissions,
            )
            .expect("schema-backed Decision Record intake");
        assert_eq!(evaluation.outcome, CoverageEvaluationOutcomeV1::Complete);
        assert_eq!(
            evaluation.normalized_content,
            decision_record_candidate_content()
        );
        assert_eq!(evaluation.basis_artifact_fingerprint, Some(current.clone()));
        assert_eq!(evaluation.coverage_results.len(), 8);
    }

    let mut missing = submissions.clone();
    missing.pop();
    assert_eq!(
        repository
            .evaluate_intake(
                target.kind_ref(),
                target.instance_id(),
                AcquisitionModeV1::Express,
                Some(current.clone()),
                &missing,
            )
            .expect_err("missing coverage")
            .kind(),
        ArtifactRepositoryErrorKindV1::IntakeEvaluation
    );

    let mut duplicate = submissions.clone();
    duplicate.push(submissions[0].clone());
    assert_eq!(
        repository
            .evaluate_intake(
                target.kind_ref(),
                target.instance_id(),
                AcquisitionModeV1::Express,
                Some(current.clone()),
                &duplicate,
            )
            .expect_err("duplicate coverage")
            .kind(),
        ArtifactRepositoryErrorKindV1::IntakeEvaluation
    );

    let mut unknown = submissions.clone();
    unknown[0].coverage_id = "decision_record.unknown".to_owned();
    assert_eq!(
        repository
            .evaluate_intake(
                target.kind_ref(),
                target.instance_id(),
                AcquisitionModeV1::Express,
                Some(current.clone()),
                &unknown,
            )
            .expect_err("unknown coverage")
            .kind(),
        ArtifactRepositoryErrorKindV1::IntakeEvaluation
    );

    let mut wrong_typed = submissions;
    wrong_typed[3].value = Some(json!(7));
    assert_eq!(
        repository
            .evaluate_intake(
                target.kind_ref(),
                target.instance_id(),
                AcquisitionModeV1::Express,
                Some(current),
                &wrong_typed,
            )
            .expect_err("schema-invalid coverage")
            .kind(),
        ArtifactRepositoryErrorKindV1::StructuralValidation
    );
}

#[test]
fn generic_decision_record_mutation_derives_exact_coverage_tokens() {
    let (repo, _repository) = open_decision_record_fixture_repository();
    let retained_fixture_bytes =
        fs::read(repo.path().join(".handbook/records/decision.yaml")).expect("fixture bytes");
    let intake_request = serde_json::to_vec(&json!({
        "idempotency_key": "decision_record_blocker_000001",
        "acquisition_mode": "express",
        "expected_current_artifact_fingerprint":
            DefinitionFingerprint::from_bytes(&retained_fixture_bytes).as_str(),
        "coverage_submissions": decision_record_coverage_submissions()
    }))
    .expect("intake request");

    let intake = handbook_engine::artifact_mutation::ArtifactMutationServiceV1::intake_append(
        repo.path(),
        DECISION_KIND_REF,
        DECISION_INSTANCE_ID,
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
        serde_json::from_slice(&fs::read(intent_path).expect("committed Decision intake intent"))
            .expect("parse committed Decision intake intent");
    let tokens = intent["outputs"]
        .as_array()
        .expect("Decision intake outputs")
        .iter()
        .map(|output| output["token"].as_str().expect("Decision output token"))
        .collect::<Vec<_>>();
    assert_eq!(
        tokens,
        vec![
            "schema-id-value",
            "schema-version-value",
            "record-id-value",
            "context-value",
            "decision-value",
            "status-value",
            "consequences-value",
            "supersedes-value",
            "intake-record",
        ]
    );
}

#[test]
fn generic_decision_record_mutation_retains_real_bytes_and_rejects_stale_basis() {
    let (repo, repository) = open_decision_record_fixture_repository();
    let target = decision_target();
    let canonical_path = repo.path().join(".handbook/records/decision.yaml");
    let retained_fixture_bytes = fs::read(&canonical_path).expect("retained fixture bytes");
    let initial_fingerprint = DefinitionFingerprint::from_bytes(&retained_fixture_bytes);
    let intake_request = serde_json::to_vec(&json!({
        "idempotency_key": "decision_record_intake_000001",
        "acquisition_mode": "express",
        "expected_current_artifact_fingerprint": initial_fingerprint.as_str(),
        "coverage_submissions": decision_record_coverage_submissions()
    }))
    .expect("intake request");
    let intake = handbook_engine::artifact_mutation::ArtifactMutationServiceV1::intake_append(
        repo.path(),
        DECISION_KIND_REF,
        DECISION_INSTANCE_ID,
        &intake_request,
    )
    .expect("generic intake append");
    assert_eq!(
        fs::read(&canonical_path).expect("canonical after intake"),
        retained_fixture_bytes
    );
    let intake_output = intake
        .result
        .authoritative_outputs
        .first()
        .expect("intake record");

    let preview =
        handbook_engine::artifact_mutation::ArtifactMutationServiceV1::candidate_validate(
            repo.path(),
            DECISION_KIND_REF,
            DECISION_INSTANCE_ID,
            &intake_output.relative_ref,
            &intake_output.fingerprint,
            Some(initial_fingerprint.as_str()),
        )
        .expect("candidate validation");
    let candidate_request = serde_json::to_vec(&json!({
        "idempotency_key": "decision_record_candidate_0001",
        "intake_record_ref": intake_output.relative_ref,
        "intake_record_fingerprint": intake_output.fingerprint,
        "expected_candidate_fingerprint": preview.candidate_fingerprint
    }))
    .expect("candidate request");
    let candidate =
        handbook_engine::artifact_mutation::ArtifactMutationServiceV1::candidate_append(
            repo.path(),
            DECISION_KIND_REF,
            DECISION_INSTANCE_ID,
            &candidate_request,
        )
        .expect("generic candidate append");
    assert_eq!(
        fs::read(&canonical_path).expect("canonical after candidate"),
        retained_fixture_bytes
    );
    let candidate_output = candidate
        .result
        .authoritative_outputs
        .first()
        .expect("candidate record");
    let promotion_request = serde_json::to_vec(&json!({
        "idempotency_key": "decision_record_promotion_0001",
        "candidate_ref": candidate_output.relative_ref,
        "candidate_fingerprint": candidate_output.fingerprint,
        "expected_current_artifact_fingerprint": initial_fingerprint.as_str()
    }))
    .expect("promotion request");
    handbook_engine::artifact_mutation::ArtifactMutationServiceV1::promote(
        repo.path(),
        DECISION_KIND_REF,
        DECISION_INSTANCE_ID,
        &promotion_request,
    )
    .expect("generic Decision Record promotion");

    let expected_content = decision_record_candidate_content();
    let expected_bytes =
        canonical_yaml_bytes(&expected_content).expect("promoted canonical Decision Record");
    assert_eq!(
        fs::read(&canonical_path).expect("promoted canonical bytes"),
        expected_bytes
    );
    let read = repository
        .read(target.kind_ref(), target.instance_id())
        .expect("read promoted Decision Record");
    assert_eq!(read.content, expected_content);
    assert_eq!(
        read.artifact_fingerprint,
        DefinitionFingerprint::from_bytes(&expected_bytes)
    );
    let validation = repository
        .validate(target.kind_ref(), target.instance_id())
        .expect("validate promoted Decision Record");
    assert!(validation.structural_errors.is_empty());
    assert_eq!(validation.content, read.content);
    assert_eq!(
        repository
            .read(target.kind_ref(), target.instance_id())
            .expect("stable second observation")
            .artifact_fingerprint,
        read.artifact_fingerprint
    );

    let stale_request = serde_json::to_vec(&json!({
        "idempotency_key": "decision_record_stale_intake_0001",
        "acquisition_mode": "express",
        "expected_current_artifact_fingerprint": initial_fingerprint.as_str(),
        "coverage_submissions": decision_record_coverage_submissions()
    }))
    .expect("stale intake request");
    let stale = handbook_engine::artifact_mutation::ArtifactMutationServiceV1::intake_append(
        repo.path(),
        DECISION_KIND_REF,
        DECISION_INSTANCE_ID,
        &stale_request,
    )
    .expect("stale mutation establishes a refusal");
    assert_eq!(stale.result.outcome, "refused");
    assert_eq!(
        stale.result.refusal.as_ref().expect("stale refusal").code,
        handbook_engine::artifact_mutation::EstablishedRefusalCodeV1::StaleCurrentArtifact
    );
    assert!(stale.result.authoritative_outputs.is_empty());
    assert_eq!(
        fs::read(&canonical_path).expect("canonical after stale refusal"),
        expected_bytes
    );

    let mut unknown_request: Value =
        serde_json::from_slice(&intake_request).expect("intake request JSON");
    unknown_request["unexpected"] = json!(true);
    let error = handbook_engine::artifact_mutation::ArtifactMutationServiceV1::intake_append(
        repo.path(),
        DECISION_KIND_REF,
        DECISION_INSTANCE_ID,
        &serde_json::to_vec(&unknown_request).expect("unknown-field request"),
    )
    .expect_err("unknown mutation request field");
    assert_eq!(
        error.kind(),
        handbook_engine::artifact_mutation::ArtifactMutationErrorKindV1::InvalidRequest
    );
    assert_eq!(
        fs::read(&canonical_path).expect("canonical after malformed request"),
        expected_bytes
    );
    assert!(!repo.path().join(".handbook/records/decision.md").exists());
}

#[test]
fn shipped_root_has_no_decision_record_instance_or_default() {
    let parent = shipped_root_decision_profile();
    let selected = selected_decision_profile();
    let parent_ids = parent
        .artifact_instances()
        .ids()
        .into_iter()
        .collect::<BTreeSet<_>>();
    let selected_ids = selected
        .artifact_instances()
        .ids()
        .into_iter()
        .collect::<BTreeSet<_>>();

    assert!(!parent_ids.contains(DECISION_INSTANCE_ID));
    assert!(parent_ids.iter().all(|id| {
        parent
            .artifact_instances()
            .instance(&SymbolicId::parse(id).expect("shipped-root instance id"))
            .expect("shipped-root descriptor")
            .canonical_path()
            != ".handbook/records/decision.yaml"
    }));
    let mut expected_selected_ids = parent_ids.clone();
    assert!(expected_selected_ids.insert(DECISION_INSTANCE_ID));
    assert_eq!(selected_ids, expected_selected_ids);
    assert_eq!(selected.exact_ref().as_str(), DECISION_PROFILE_REF);

    let selection: Value = serde_json::from_slice(
        &fs::read(decision_record_fixture_root().join(".handbook/profile-selection.json"))
            .expect("Decision Record selection fixture"),
    )
    .expect("Decision Record selection JSON");
    assert_eq!(
        selection["profile_sources"][1]["source"],
        json!({
            "kind": "repository_path",
            "path": ".handbook/definitions/profiles/decision-record-root-1.0.0.yaml"
        })
    );
}

#[test]
fn author_command_inventory_has_no_decision_record_surface() {
    let cli = include_str!("../../cli/src/main.rs");
    let author_command = cli
        .split_once("enum AuthorCommand")
        .expect("AuthorCommand enum")
        .1
        .split_once('}')
        .expect("AuthorCommand enum body")
        .0;
    assert!(author_command.contains("Charter(AuthorCharterArgs)"));
    assert!(author_command.contains("ProjectContext(AuthorProjectContextArgs)"));

    let help = include_str!("../../cli/tests/snapshots/handbook-author-help.txt");
    let emitted_commands = help
        .lines()
        .filter_map(|line| {
            let trimmed = line.strip_prefix("  ")?;
            (!trimmed.starts_with('-')).then(|| {
                trimmed
                    .split_whitespace()
                    .next()
                    .expect("author command name")
            })
        })
        .collect::<Vec<_>>();
    assert_eq!(emitted_commands, ["charter", "project-context"]);

    for forbidden in [
        "DecisionRecord",
        "Decision Record",
        "decision-record",
        "decision_record",
    ] {
        assert!(!author_command.contains(forbidden));
        assert!(!help.contains(forbidden));
    }
}

#[test]
fn missing_exact_decision_filename_refuses_inferred_and_dynamic_decoys() {
    let (repo, repository) = open_decision_record_fixture_repository();
    let canonical_path = repo.path().join(".handbook/records/decision.yaml");
    let canonical_bytes = fs::read(&canonical_path).expect("canonical Decision Record bytes");
    fs::remove_file(&canonical_path).expect("remove exact Decision Record filename");
    for decoy in [
        "decision-record.yaml",
        "decision_record.yaml",
        "example.record.decision.yaml",
    ] {
        fs::write(
            repo.path().join(".handbook/records").join(decoy),
            &canonical_bytes,
        )
        .expect("install alternate Decision Record filename");
    }

    assert_eq!(
        repository
            .read(
                &ExactDefinitionRef::parse(DECISION_KIND_REF).expect("Decision kind ref"),
                &SymbolicId::parse(DECISION_INSTANCE_ID).expect("Decision instance id"),
            )
            .expect_err("alternate filenames must not be discovered")
            .kind(),
        ArtifactRepositoryErrorKindV1::ArtifactRead
    );
}

#[test]
fn decision_record_projection_widening_is_refused() {
    let selected = selected_decision_profile();
    let descriptor = selected
        .artifact_instances()
        .instance(&SymbolicId::parse(DECISION_INSTANCE_ID).expect("Decision instance id"))
        .expect("Decision Record descriptor");
    assert!(descriptor.projection_definition_refs().is_empty());

    let profile: Value = serde_json::from_slice(
        &fs::read(
            decision_record_fixture_root()
                .join(".handbook/definitions/profiles/decision-record-root-1.0.0.yaml"),
        )
        .expect("Decision Record profile"),
    )
    .expect("Decision Record profile JSON");
    let mut descriptors = profile["artifact_instances"]
        .as_array()
        .expect("artifact instances")
        .clone();
    descriptors
        .iter_mut()
        .find(|descriptor| descriptor["id"] == DECISION_INSTANCE_ID)
        .expect("Decision Record descriptor")["projection_definition_refs"] =
        json!(["example.projection.decision-record@1.0.0"]);

    assert!(
        ArtifactInstanceRegistry::resolve(
            &descriptors,
            shipped_root_decision_profile().artifact_kind_registry(),
            &[],
        )
        .is_err(),
        "capitalized Projection widening must be refused"
    );
}

#[test]
fn markdown_decoys_have_zero_decision_record_authority() {
    let (repo, repository) = open_decision_record_fixture_repository();
    let kind_ref = ExactDefinitionRef::parse(DECISION_KIND_REF).expect("Decision kind ref");
    let instance_id = SymbolicId::parse(DECISION_INSTANCE_ID).expect("Decision instance id");
    let mirror = repo.path().join(".handbook/records/decision.md");
    let inferred = repo.path().join(".handbook/records/DECISION_RECORD.md");
    assert!(!mirror.exists());
    assert!(!inferred.exists());

    let before = repository
        .read(&kind_ref, &instance_id)
        .expect("canonical read before Markdown decoys");
    repository
        .validate(&kind_ref, &instance_id)
        .expect("generic validation before Markdown decoys");
    assert!(!mirror.exists());
    assert!(!inferred.exists());

    fs::write(&mirror, b"# Malicious persistent mirror\n").expect("Markdown mirror decoy");
    fs::write(&inferred, b"# Inferred Decision Record view\n").expect("inferred Markdown decoy");
    let after = repository
        .read(&kind_ref, &instance_id)
        .expect("canonical read after Markdown decoys");
    assert_eq!(after.content, before.content);
    assert_eq!(after.artifact_fingerprint, before.artifact_fingerprint);
    assert_eq!(
        repository
            .validate(&kind_ref, &instance_id)
            .expect("generic validation after Markdown decoys")
            .content,
        before.content
    );

    fs::remove_file(repo.path().join(".handbook/records/decision.yaml"))
        .expect("remove canonical Decision Record YAML");
    assert_eq!(
        repository
            .read(&kind_ref, &instance_id)
            .expect_err("Markdown must not become a fallback authority")
            .kind(),
        ArtifactRepositoryErrorKindV1::ArtifactRead
    );
}

#[test]
fn fixed_decision_renderer_golden_is_deterministic_and_resolution_free() {
    let definition_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("definitions");
    let mut renderer = parse_definition_yaml(
        &fs::read(
            definition_root
                .join("renderers/handbook.renderer.decision-record-review-markdown/1.0.0.yaml"),
        )
        .expect("Decision Record renderer"),
    )
    .expect("renderer definition");
    assert_eq!(renderer["resolution_input"], Value::Null);
    assert_eq!(renderer["input_schema_ref"], DECISION_SCHEMA_REF);
    assert_eq!(
        renderer["implementation_id"],
        "decision-record-review-markdown-v1"
    );
    assert_eq!(
        renderer["determinism_profile"],
        "closed-decision-record-review-markdown-v1"
    );
    assert_eq!(renderer["extensions"], json!({}));
    let supplied = renderer
        .as_object_mut()
        .expect("renderer object")
        .remove("renderer_fingerprint")
        .expect("renderer fingerprint");
    let schema = selected_decision_profile()
        .artifact_kind_registry()
        .schema_registry()
        .entry(&ExactDefinitionRef::parse(DECISION_SCHEMA_REF).expect("Decision schema ref"))
        .expect("Decision Record schema entry")
        .clone();
    let computed = DefinitionFingerprint::from_json_value(&json!({
        "definition": renderer,
        "resolved_dependencies": [{
            "definition_fingerprint": schema.entry_fingerprint().as_str(),
            "definition_ref": DECISION_SCHEMA_REF,
            "dependency_role": "input_schema"
        }]
    }))
    .expect("renderer closure fingerprint");
    assert_eq!(supplied, computed.as_str());
    assert_eq!(
        computed.as_str(),
        "sha256:81f616aeecb1eac6c5c9207bcd2eb82c8e79ee159d57b97e2f15492a6d6a12b3"
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
        .find(|golden| golden["renderer_ref"] == DECISION_RENDERER_REF)
        .expect("Decision Record renderer golden");
    assert_eq!(golden["renderer_fingerprint"], computed.as_str());
    assert_eq!(golden["resolution_input"], Value::Null);
    selected_decision_profile()
        .artifact_kind_registry()
        .schema_registry()
        .resolved(&ExactDefinitionRef::parse(DECISION_SCHEMA_REF).expect("Decision schema ref"))
        .expect("resolved Decision Record schema")
        .validate_json(&golden["canonical_input"])
        .expect("schema-valid golden input");
    let markdown = golden["expected_markdown_utf8"]
        .as_str()
        .expect("golden Markdown");
    assert_eq!(
        markdown.len() as u64,
        golden["expected_byte_length"]
            .as_u64()
            .expect("byte length")
    );
    assert_eq!(
        DefinitionFingerprint::from_bytes(markdown.as_bytes()).as_str(),
        golden["expected_sha256"].as_str().expect("golden SHA-256")
    );
}
