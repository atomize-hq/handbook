use handbook_engine::artifact_intake::{
    CoverageConfidenceV1, CoverageSourceKindV1, CoverageSpecificityV1, CoverageSubmissionStateV1,
    CoverageSubmissionV1,
};
use handbook_engine::artifact_repository::{ArtifactRepositoryV1, ArtifactTargetV1};
use handbook_engine::{
    inspect_profile_repository, load_selected_environment_context,
    parse_canonical_environment_context, parse_definition_yaml,
    render_environment_context_markdown, resolve_shipped_profile_decisions,
    serialize_canonical_environment_context, ArtifactApplicability, ArtifactInspectionReason,
    ArtifactInspectionStatus, ArtifactInstanceRegistry, CanonicalEnvironmentContext,
    EnvironmentContextArtifactErrorKind, NamedEnvironment, RequirednessMode,
};
use handbook_engine::{DefinitionFingerprint, RepositoryInvocationIdentityServiceV1};
use serde_json::{json, Value};
use std::path::{Path, PathBuf};

const CANONICAL_YAML: &str = concat!(
    "authoritative_references:\n",
    "  - \"handbook.project.environments@1.0.0\"\n",
    "environments:\n",
    "  -\n",
    "    capabilities:\n",
    "      - \"rust.stable\"\n",
    "      - \"filesystem.workspace-write\"\n",
    "    description: \"Local development and focused tests.\"\n",
    "    environment_id: \"local-dev\"\n",
    "  -\n",
    "    capabilities:\n",
    "      - \"rust.stable\"\n",
    "      - \"os.linux\"\n",
    "    description: \"Linux completion target.\"\n",
    "    environment_id: \"ci-linux\"\n",
    "known_unknowns:\n",
    "  - \"Windows cross-target proof remains pending.\"\n",
    "record_id: \"handbook.environment-context\"\n",
    "schema_id: \"handbook.artifact.environment-context\"\n",
    "schema_version: \"1.1\"\n",
);

const RENDERED_MARKDOWN: &str = concat!(
    "# Environment Context\n\n",
    "## local\\-dev\n\n",
    "Local development and focused tests\\.\n\n",
    "### Capabilities\n\n",
    "- `rust.stable`\n",
    "- `filesystem.workspace-write`\n\n",
    "## ci\\-linux\n\n",
    "Linux completion target\\.\n\n",
    "### Capabilities\n\n",
    "- `rust.stable`\n",
    "- `os.linux`\n\n",
    "## Authoritative References\n\n",
    "- `handbook.project.environments@1.0.0`\n\n",
    "## Known Unknowns\n\n",
    "- Windows cross\\-target proof remains pending\\.\n",
);

fn decisions() -> handbook_engine::ResolvedProfileDecisions {
    resolve_shipped_profile_decisions(Path::new(env!("CARGO_MANIFEST_DIR")))
        .expect("shipped profile resolves")
}

fn environment_path(root: &Path) -> PathBuf {
    root.join(".handbook/project/environment.yaml")
}

fn write_environment(root: &Path, bytes: &[u8]) {
    let path = environment_path(root);
    std::fs::create_dir_all(path.parent().expect("environment parent"))
        .expect("create environment parent");
    std::fs::write(path, bytes).expect("write Environment Context fixture");
}

fn supplied_environment_field(
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

#[test]
fn shipped_environment_context_is_optional_advisory_context() {
    let decisions = decisions();
    let environment = decisions
        .artifact_decisions()
        .iter()
        .find(|decision| decision.instance_id().as_str() == "environment_context")
        .expect("selected Environment Context");

    assert_eq!(environment.requiredness_mode(), RequirednessMode::Optional);
    assert_eq!(environment.applicability(), ArtifactApplicability::Optional);
    assert!(environment.condition_ref().is_none());
    assert!(environment.condition_outcome().is_none());
    assert!(environment.condition_reason().is_none());
    assert!(environment.evidence_closure_fingerprint().is_none());
    assert!(decisions.condition_evaluations().is_empty());
}

#[test]
fn canonical_yaml_roundtrips_and_renders_fixed_bytes() {
    let decisions = decisions();
    let record = parse_canonical_environment_context(&decisions, CANONICAL_YAML.as_bytes())
        .expect("canonical Environment Context");

    assert_eq!(record.schema_version, "1.1");
    assert_eq!(record.environments.len(), 2);
    assert_eq!(record.environments[0].environment_id, "local-dev");
    assert_eq!(
        serialize_canonical_environment_context(&decisions, &record).expect("canonical YAML"),
        CANONICAL_YAML.as_bytes()
    );
    assert_eq!(
        render_environment_context_markdown(&record).expect("derived review view"),
        RENDERED_MARKDOWN.as_bytes()
    );
}

#[test]
fn schema_and_typed_boundary_refuse_unknown_fields_and_duplicate_environment_ids() {
    let decisions = decisions();
    let unknown = CANONICAL_YAML.replacen(
        "record_id: \"handbook.environment-context\"",
        "record_id: \"handbook.environment-context\"\nlegacy_authority: true",
        1,
    );
    assert_eq!(
        parse_canonical_environment_context(&decisions, unknown.as_bytes())
            .expect_err("unknown field must fail")
            .kind(),
        EnvironmentContextArtifactErrorKind::StructuralValidationFailed
    );

    let mut duplicate = parse_canonical_environment_context(&decisions, CANONICAL_YAML.as_bytes())
        .expect("canonical Environment Context");
    duplicate.environments.push(NamedEnvironment {
        environment_id: "local-dev".to_owned(),
        description: "Duplicate identity".to_owned(),
        capabilities: vec!["rust.stable".to_owned()],
    });
    assert_eq!(
        serialize_canonical_environment_context(&decisions, &duplicate)
            .expect_err("duplicate environment identity must fail")
            .kind(),
        EnvironmentContextArtifactErrorKind::DuplicateEnvironmentId
    );
}

#[test]
fn valid_context_is_loadable_and_visible_to_advisory_inspection() {
    let repo = tempfile::tempdir().expect("temporary repository");
    write_environment(repo.path(), CANONICAL_YAML.as_bytes());

    let projection = load_selected_environment_context(repo.path(), &decisions())
        .expect("validated advisory Environment Context");
    assert_eq!(
        projection.canonical_path(),
        ".handbook/project/environment.yaml"
    );
    assert_eq!(projection.source_byte_length(), CANONICAL_YAML.len());
    assert_eq!(projection.rendered_bytes(), RENDERED_MARKDOWN.as_bytes());

    let report = inspect_profile_repository(repo.path(), &decisions());
    let row = report
        .artifacts()
        .iter()
        .find(|artifact| artifact.instance_id().as_str() == "environment_context")
        .expect("Environment Context inspection row");
    assert_eq!(row.applicability(), ArtifactApplicability::Optional);
    assert_eq!(row.status(), ArtifactInspectionStatus::StructurallyValid);
    assert_eq!(
        row.reason(),
        ArtifactInspectionReason::PresentAndStructurallyValid
    );
}

#[test]
fn missing_or_invalid_unselected_context_is_reported_without_becoming_required() {
    let repo = tempfile::tempdir().expect("temporary repository");
    let missing = inspect_profile_repository(repo.path(), &decisions());
    let missing_row = missing
        .artifacts()
        .iter()
        .find(|artifact| artifact.instance_id().as_str() == "environment_context")
        .expect("Environment Context inspection row");
    assert_eq!(missing_row.applicability(), ArtifactApplicability::Optional);
    assert_eq!(missing_row.status(), ArtifactInspectionStatus::Missing);
    assert_eq!(
        missing_row.reason(),
        ArtifactInspectionReason::OptionalPathMissing
    );

    write_environment(repo.path(), b"schema_id: [\n");
    let invalid = inspect_profile_repository(repo.path(), &decisions());
    let invalid_row = invalid
        .artifacts()
        .iter()
        .find(|artifact| artifact.instance_id().as_str() == "environment_context")
        .expect("Environment Context inspection row");
    assert_eq!(invalid_row.applicability(), ArtifactApplicability::Optional);
    assert_eq!(
        invalid_row.status(),
        ArtifactInspectionStatus::StructurallyInvalid
    );
    assert_eq!(
        invalid_row.reason(),
        ArtifactInspectionReason::YamlSyntaxInvalid
    );
}

#[test]
fn legacy_environment_inventory_markdown_has_no_authority_or_readiness_influence() {
    let repo = tempfile::tempdir().expect("temporary repository");
    let legacy = repo
        .path()
        .join(".handbook/environment_inventory/ENVIRONMENT_INVENTORY.md");
    std::fs::create_dir_all(legacy.parent().expect("legacy parent"))
        .expect("create legacy directory");
    std::fs::write(&legacy, b"# Legacy Environment Inventory\n").expect("write legacy Markdown");

    let report = inspect_profile_repository(repo.path(), &decisions());
    let row = report
        .artifacts()
        .iter()
        .find(|artifact| artifact.instance_id().as_str() == "environment_context")
        .expect("Environment Context inspection row");
    assert_eq!(row.applicability(), ArtifactApplicability::Optional);
    assert_eq!(row.status(), ArtifactInspectionStatus::Missing);
    assert_eq!(row.reason(), ArtifactInspectionReason::OptionalPathMissing);
    assert!(load_selected_environment_context(repo.path(), &decisions()).is_err());
}

#[test]
fn serialization_refuses_records_outside_the_selected_schema() {
    let mut record = CanonicalEnvironmentContext {
        schema_id: "handbook.artifact.environment-context".to_owned(),
        schema_version: "1.1".to_owned(),
        record_id: "handbook.environment-context".to_owned(),
        environments: vec![],
        authoritative_references: vec![],
        known_unknowns: vec![],
    };
    assert_eq!(
        serialize_canonical_environment_context(&decisions(), &record)
            .expect_err("empty environment matrix must fail")
            .kind(),
        EnvironmentContextArtifactErrorKind::StructuralValidationFailed
    );

    record.environments.push(NamedEnvironment {
        environment_id: "local-dev".to_owned(),
        description: "Local development".to_owned(),
        capabilities: vec![],
    });
    assert_eq!(
        serialize_canonical_environment_context(&decisions(), &record)
            .expect_err("empty capability set must fail")
            .kind(),
        EnvironmentContextArtifactErrorKind::StructuralValidationFailed
    );
}

#[test]
fn generic_mutation_safely_promotes_environment_context_bytes() {
    let repo = tempfile::tempdir().expect("temporary repository");
    let fixture_selection = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/hcm_2_4_risk_record/.handbook/profile-selection.json");
    let mut selection: Value = serde_json::from_slice(
        &std::fs::read(fixture_selection).expect("repository selection fixture"),
    )
    .expect("repository selection JSON");
    selection["selected_profile_ref"] = json!("handbook.profile.shipped-root@1.2.0");
    selection["profile_sources"]
        .as_array_mut()
        .expect("profile sources")
        .retain(|source| source["exact_ref"] == json!("handbook.profile.shipped-root@1.2.0"));
    selection["project_condition_sources"] = json!([]);
    selection["intake_definition_sources"]
        .as_array_mut()
        .expect("intake sources")
        .retain(|source| {
            matches!(
                source["exact_ref"].as_str(),
                Some("handbook.intake.charter@1.0.0" | "handbook.intake.project-context@1.0.0")
            )
        });
    selection["intake_definition_sources"]
        .as_array_mut()
        .expect("intake source list")
        .push(json!({
            "exact_ref": "handbook.intake.environment-context@1.0.0",
            "source": {"kind": "built_in"}
        }));
    let selection_path = repo.path().join(".handbook/profile-selection.json");
    std::fs::create_dir_all(selection_path.parent().expect("selection parent"))
        .expect("create selection parent");
    std::fs::write(
        selection_path,
        serde_json::to_vec_pretty(&selection).expect("selection bytes"),
    )
    .expect("write repository selection");
    write_environment(repo.path(), CANONICAL_YAML.as_bytes());
    RepositoryInvocationIdentityServiceV1::new()
        .initialize_for_setup(repo.path())
        .expect("repository identity");

    let selection_bytes =
        std::fs::read(repo.path().join(".handbook/profile-selection.json")).expect("selection");
    let selection =
        handbook_engine::artifact_intake_registry::RepositoryProfileSelectionV1::from_json_bytes(
            &selection_bytes,
        )
        .expect("admitted repository selection");
    handbook_engine::resolve_profile_selection(repo.path(), selection.profile_request())
        .expect("selected shipped profile");
    let repository = ArtifactRepositoryV1::open(repo.path()).expect("artifact repository");
    let target = ArtifactTargetV1::parse(
        "handbook.artifact-kind.environment-context@1.1.0",
        "environment_context",
    )
    .expect("Environment Context target");
    let current = repository
        .current_artifact_fingerprint(&target)
        .expect("current fingerprint")
        .expect("current Environment Context");
    assert_eq!(
        current,
        DefinitionFingerprint::from_bytes(CANONICAL_YAML.as_bytes())
    );

    let updated = json!({
        "schema_id": "handbook.artifact.environment-context",
        "schema_version": "1.1",
        "record_id": "handbook.environment-context",
        "environments": [{
            "environment_id": "local-dev",
            "description": "Local development and safe mutation proof.",
            "capabilities": ["rust.stable", "filesystem.workspace-write"]
        }],
        "authoritative_references": ["handbook.project.environments@1.0.0"],
        "known_unknowns": []
    });
    let submissions = [
        ("schema_id", CoverageSpecificityV1::Exact),
        ("schema_version", CoverageSpecificityV1::Exact),
        ("record_id", CoverageSpecificityV1::Exact),
        ("environments", CoverageSpecificityV1::Concrete),
        ("authoritative_references", CoverageSpecificityV1::Exact),
        ("known_unknowns", CoverageSpecificityV1::Concrete),
    ]
    .into_iter()
    .map(|(field, specificity)| {
        supplied_environment_field(
            &format!("environment_context.{field}"),
            updated[field].clone(),
            specificity,
        )
    })
    .collect::<Vec<_>>();
    let intake_request = serde_json::to_vec(&json!({
        "idempotency_key": "p2_environment_intake_0001",
        "acquisition_mode": "express",
        "expected_current_artifact_fingerprint": current.as_str(),
        "coverage_submissions": submissions,
    }))
    .expect("intake request");
    let intake = handbook_engine::artifact_mutation::ArtifactMutationServiceV1::intake_append(
        repo.path(),
        "handbook.artifact-kind.environment-context@1.1.0",
        "environment_context",
        &intake_request,
    )
    .expect("Environment Context intake");
    let intake_output = intake
        .result
        .authoritative_outputs
        .first()
        .expect("retained intake");
    let preview =
        handbook_engine::artifact_mutation::ArtifactMutationServiceV1::candidate_validate(
            repo.path(),
            "handbook.artifact-kind.environment-context@1.1.0",
            "environment_context",
            &intake_output.relative_ref,
            &intake_output.fingerprint,
            Some(current.as_str()),
        )
        .expect("candidate validation");
    let candidate_request = serde_json::to_vec(&json!({
        "idempotency_key": "p2_environment_candidate_0001",
        "intake_record_ref": intake_output.relative_ref,
        "intake_record_fingerprint": intake_output.fingerprint,
        "expected_candidate_fingerprint": preview.candidate_fingerprint,
    }))
    .expect("candidate request");
    let candidate =
        handbook_engine::artifact_mutation::ArtifactMutationServiceV1::candidate_append(
            repo.path(),
            "handbook.artifact-kind.environment-context@1.1.0",
            "environment_context",
            &candidate_request,
        )
        .expect("candidate append");
    let candidate_output = candidate
        .result
        .authoritative_outputs
        .first()
        .expect("retained candidate");
    let promotion_request = serde_json::to_vec(&json!({
        "idempotency_key": "p2_environment_promotion_0001",
        "candidate_ref": candidate_output.relative_ref,
        "candidate_fingerprint": candidate_output.fingerprint,
        "expected_current_artifact_fingerprint": current.as_str(),
    }))
    .expect("promotion request");
    handbook_engine::artifact_mutation::ArtifactMutationServiceV1::promote(
        repo.path(),
        "handbook.artifact-kind.environment-context@1.1.0",
        "environment_context",
        &promotion_request,
    )
    .expect("safe promotion");

    let promoted =
        load_selected_environment_context(repo.path(), &decisions()).expect("promoted context");
    assert_eq!(promoted.record().environments.len(), 1);
    assert_eq!(
        promoted.record().environments[0].description,
        "Local development and safe mutation proof."
    );
}

#[test]
fn work_specification_fixture_replays_its_corrected_profile_fingerprint() {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/fixtures/pipeline_proof_corpus/foundation_inputs/repo");
    let selection_bytes = std::fs::read(fixture.join(".handbook/profile-selection.json"))
        .expect("pipeline fixture selection");
    let mut root_selection: Value =
        serde_json::from_slice(&selection_bytes).expect("selection JSON");
    root_selection["selected_profile_ref"] = json!("handbook.profile.shipped-root@1.2.0");
    root_selection["profile_sources"] = json!([{
        "exact_ref": "handbook.profile.shipped-root@1.2.0",
        "source": {"kind": "built_in"}
    }]);
    let root_selection =
        handbook_engine::artifact_intake_registry::RepositoryProfileSelectionV1::from_json_bytes(
            &serde_json::to_vec(&root_selection).expect("root selection bytes"),
        )
        .expect("root selection");
    let root =
        handbook_engine::resolve_profile_selection(&fixture, root_selection.profile_request())
            .expect("shipped root closure");

    let profile_path =
        fixture.join(".handbook/definitions/profiles/work-specification-root-1.0.0.yaml");
    let mut definition =
        parse_definition_yaml(&std::fs::read(profile_path).expect("work profile bytes"))
            .expect("work profile definition");
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
                "reference": "example.profile.hcm-2-4-work-specification@1.0.0",
                "fingerprint": descriptors.fingerprint().as_str()
            },
            {
                "definition_class": "profile",
                "reference": "handbook.profile.shipped-root@1.2.0",
                "fingerprint": root.selected_profile_definition_fingerprint().as_str()
            }
        ]
    }))
    .expect("work profile fingerprint");
    assert_eq!(supplied, json!(computed.as_str()));
}

#[test]
fn corrected_work_specification_fixture_profile_resolves() {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/fixtures/pipeline_proof_corpus/foundation_inputs/repo");
    let selection =
        handbook_engine::artifact_intake_registry::RepositoryProfileSelectionV1::from_json_bytes(
            &std::fs::read(fixture.join(".handbook/profile-selection.json"))
                .expect("pipeline fixture selection"),
        )
        .expect("repository selection");
    let profile = handbook_engine::resolve_profile_selection(&fixture, selection.profile_request())
        .expect("corrected Work Specification profile");
    assert_eq!(
        profile.exact_ref().as_str(),
        "example.profile.hcm-2-4-work-specification@1.0.0"
    );
}
