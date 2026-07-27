use handbook_engine::artifact_intake_registry::{
    AcquisitionModeV1, ArtifactIntakeRegistry, ArtifactIntakeSourceV1,
    ArtifactRegistrationErrorKindV1,
};
use handbook_engine::artifact_repository::{
    ArtifactRepositoryErrorKindV1, ArtifactRepositoryV1, ArtifactTargetV1,
};
use handbook_engine::{
    DefinitionFingerprint, ExactDefinitionRef, RepositoryInvocationIdentityServiceV1,
};
use serde_json::{json, Value};
use std::fs;
use std::path::Path;

const PROJECT_CONTEXT_INTAKE_REF: &str = "handbook.intake.project-context@1.0.0";
const PROJECT_CONTEXT_KIND_REF: &str = "handbook.artifact-kind.project-context@1.1.0";
const PROJECT_CONTEXT_KIND_FINGERPRINT: &str =
    "sha256:16526ff8179adf1c0e72fb6980800ea4756c919f5b1499cb97227d7727517fba";
const PROJECT_CONTEXT_SCHEMA_REF: &str = "handbook.schemas.artifacts.project-context@1.0.0";
const PROJECT_CONTEXT_SCHEMA_FINGERPRINT: &str =
    "sha256:792ea38fc2760f90bca9a376182971d44f4ab19107cc7bf88bdf1786c93b4e24";
const STALE_FINGERPRINT: &str =
    "sha256:0000000000000000000000000000000000000000000000000000000000000000";

fn built_in(reference: &str) -> Value {
    json!({
        "exact_ref": reference,
        "source": {"kind": "built_in"},
    })
}

fn repository_path(reference: &str, path: &str) -> Value {
    json!({
        "exact_ref": reference,
        "source": {"kind": "repository_path", "path": path},
    })
}

fn p1a_selection_record() -> Value {
    let schemas = [
        "handbook.schemas.artifacts.project-authority@1.0.0",
        "handbook.schemas.artifacts.project-authority@1.1.0",
        "handbook.schemas.artifacts.project-context@1.0.0",
        "handbook.schemas.artifacts.environment-context@1.0.0",
        "handbook.schemas.artifacts.work-specification@1.0.0",
        "handbook.schemas.artifacts.decision-record@1.0.0",
        "handbook.schemas.artifacts.risk-record@1.0.0",
        "handbook.schemas.lifecycle.trigger-evidence@1.0.0",
        "handbook.schemas.security.approver-registry@1.0.0",
        "handbook.schemas.security.approver-registry-transition@1.0.0",
        "handbook.schemas.security.authenticator-registration-request@1.0.0",
        "handbook.schemas.security.authenticator-challenge@1.0.0",
        "handbook.schemas.security.authenticator-make-credential-response@1.0.0",
        "handbook.schemas.security.authenticator-registration@1.0.0",
        "handbook.schemas.security.authenticator-get-assertion-response@1.0.0",
        "handbook.schemas.security.authenticator-assertion@1.0.0",
        "handbook.schemas.security.approver-admin-api@1.0.0",
    ];
    let kinds = [
        "handbook.artifact-kind.project-authority@1.0.0",
        "handbook.artifact-kind.project-authority@1.1.0",
        "handbook.artifact-kind.project-context@1.0.0",
        "handbook.artifact-kind.project-context@1.1.0",
        "handbook.artifact-kind.environment-context@1.0.0",
        "handbook.artifact-kind.environment-context@1.1.0",
        "handbook.artifact-kind.work-specification@1.0.0",
        "handbook.artifact-kind.work-specification@1.1.0",
        "handbook.artifact-kind.decision-record@1.0.0",
        "handbook.artifact-kind.decision-record@1.1.0",
        "handbook.artifact-kind.risk-record@1.0.0",
        "handbook.artifact-kind.risk-record@1.1.0",
    ];
    let intakes = [
        "handbook.intake.charter@1.0.0",
        "handbook.intake.project-context@1.0.0",
        "handbook.intake.environment-context@1.0.0",
        "handbook.intake.work-specification@1.0.0",
        "handbook.intake.decision-record@1.0.0",
        "handbook.intake.risk-record@1.0.0",
    ];

    json!({
        "schema_id": "handbook.repository-profile-selection",
        "schema_version": "1.0",
        "selected_profile_ref": "handbook.profile.shipped-root@1.2.0",
        "profile_sources": [built_in("handbook.profile.shipped-root@1.2.0")],
        "stable_role_registry_sources": [built_in("handbook.roles.core@1.1.0")],
        "schema_entry_sources": schemas.map(built_in),
        "artifact_kind_sources": kinds.map(built_in),
        "semantic_capability_sources": [
            built_in("handbook.capabilities.constitutional-root@1.0.0")
        ],
        "semantic_validator_sources": [
            built_in("handbook.semantic-validation.constitutional-root@1.0.0"),
            built_in("handbook.semantic-validation.constitutional-root@1.1.0")
        ],
        "project_condition_sources": [
            built_in("handbook.condition.project.managed-operational-surface@1.0.0")
        ],
        "vocabulary_sources": [built_in("handbook.vocabulary.shipped-root@1.0.0")],
        "context_resolution_sources": [
            built_in("handbook.context-resolution.shipped-root@1.0.0")
        ],
        "context_resolution_policy_sources": [
            built_in("handbook.mutation-matcher.core@1.0.0"),
            built_in("handbook.resolution-escalation.core@1.0.0"),
            built_in("handbook.memory-promotion.core@1.0.0")
        ],
        "intake_definition_sources": intakes.map(built_in),
        "allowed_schema_roots": ["definitions/schemas"],
        "extensions": {},
    })
}

fn open_p1a_repository(selection: &Value) -> tempfile::TempDir {
    let repo = tempfile::tempdir().expect("repository root");
    fs::create_dir_all(repo.path().join(".handbook")).expect("create .handbook");
    fs::write(
        repo.path().join(".handbook/profile-selection.json"),
        serde_json::to_vec_pretty(selection).expect("selection JSON"),
    )
    .expect("write selection");
    RepositoryInvocationIdentityServiceV1::new()
        .initialize_for_setup(repo.path())
        .expect("repository identity");
    repo
}

fn project_context_intake() -> Value {
    serde_json::from_slice(
        &fs::read(format!(
            "{}/definitions/intakes/handbook.intake.project-context/1.0.0.yaml",
            env!("CARGO_MANIFEST_DIR")
        ))
        .expect("project-context intake bytes"),
    )
    .expect("project-context intake JSON")
}

fn set_project_context_fingerprint(
    definition: &mut Value,
    kind_fingerprint: &str,
    schema_fingerprint: &str,
) {
    definition
        .as_object_mut()
        .expect("intake object")
        .remove("intake_definition_fingerprint");
    let fingerprint = DefinitionFingerprint::from_json_value(&json!({
        "definition": definition,
        "resolved_dependencies": [
            {
                "definition_fingerprint": kind_fingerprint,
                "definition_ref": PROJECT_CONTEXT_KIND_REF,
                "dependency_role": "artifact_kind",
            },
            {
                "definition_fingerprint": schema_fingerprint,
                "definition_ref": PROJECT_CONTEXT_SCHEMA_REF,
                "dependency_role": "candidate_schema",
            },
        ],
    }))
    .expect("intake closure fingerprint")
    .to_string();
    definition.as_object_mut().expect("intake object").insert(
        "intake_definition_fingerprint".to_owned(),
        Value::String(fingerprint),
    );
}

fn load_project_context_intake(
    definition: &Value,
) -> Result<
    ArtifactIntakeRegistry,
    handbook_engine::artifact_intake_registry::ArtifactRegistrationErrorV1,
> {
    ArtifactIntakeRegistry::load(&[ArtifactIntakeSourceV1 {
        definition_ref: ExactDefinitionRef::parse(PROJECT_CONTEXT_INTAKE_REF)
            .expect("project-context intake ref"),
        bytes: serde_json::to_vec(definition).expect("intake bytes"),
    }])
}

#[test]
fn artifact_repository_open_semantically_admits_all_p1a_intakes() {
    for slug in [
        "project-context",
        "environment-context",
        "work-specification",
        "decision-record",
        "risk-record",
    ] {
        let definition_ref = ExactDefinitionRef::parse(&format!("handbook.intake.{slug}@1.0.0"))
            .expect("intake ref");
        let bytes = fs::read(format!(
            "{}/definitions/intakes/handbook.intake.{slug}/1.0.0.yaml",
            env!("CARGO_MANIFEST_DIR")
        ))
        .expect("intake bytes");
        ArtifactIntakeRegistry::load(&[ArtifactIntakeSourceV1 {
            definition_ref,
            bytes,
        }])
        .unwrap_or_else(|error| panic!("{slug}: {error:?}"));
    }

    let repo = open_p1a_repository(&p1a_selection_record());
    let repository = ArtifactRepositoryV1::open(repo.path()).expect("P1A repository admission");

    for (kind_ref, instance_id, expected_coverage) in [
        (
            "handbook.artifact-kind.project-context@1.1.0",
            "project_context",
            8,
        ),
        (
            "handbook.artifact-kind.environment-context@1.1.0",
            "environment_context",
            9,
        ),
    ] {
        let target = ArtifactTargetV1::parse(kind_ref, instance_id).expect("artifact target");
        let intake = repository
            .intake_definition(&target)
            .expect("semantic intake");
        assert_eq!(
            intake.supported_modes(),
            [
                AcquisitionModeV1::GuidedAdaptive,
                AcquisitionModeV1::Express,
                AcquisitionModeV1::AgentAssisted,
            ]
        );
        assert_eq!(intake.coverage().len(), expected_coverage);
    }

    let charter = repository
        .intake_definition(
            &ArtifactTargetV1::parse(
                "handbook.artifact-kind.project-authority@1.1.0",
                "project_authority",
            )
            .expect("Charter target"),
        )
        .expect("Charter compatibility intake");
    assert!(charter.supported_modes().is_empty());
    assert!(charter.coverage().is_empty());
}

#[test]
fn package_owned_intake_refs_refuse_repository_source_rebinding() {
    for (reference, package_path) in [
        (
            PROJECT_CONTEXT_INTAKE_REF,
            "definitions/intakes/handbook.intake.project-context/1.0.0.yaml",
        ),
        (
            "handbook.intake.charter@1.0.0",
            "definitions/intakes/handbook.intake.charter/1.0.0.yaml",
        ),
    ] {
        let repository_definition_path = format!(
            ".handbook/definitions/intakes/{}.yaml",
            reference.replace(['.', '@'], "-")
        );
        let mut selection = p1a_selection_record();
        let binding = selection["intake_definition_sources"]
            .as_array_mut()
            .expect("intake bindings")
            .iter_mut()
            .find(|binding| binding["exact_ref"] == reference)
            .expect("package intake binding");
        *binding = repository_path(reference, &repository_definition_path);

        let repo = open_p1a_repository(&selection);
        let destination = repo.path().join(&repository_definition_path);
        fs::create_dir_all(destination.parent().expect("definition parent"))
            .expect("create definition parent");
        fs::copy(
            Path::new(env!("CARGO_MANIFEST_DIR")).join(package_path),
            &destination,
        )
        .expect("copy exact package intake bytes");

        let error = ArtifactRepositoryV1::open(repo.path())
            .expect_err("a package-owned intake ref must remain package-owned");
        assert_eq!(
            error.kind(),
            ArtifactRepositoryErrorKindV1::IntakeSourceRead,
            "{reference}"
        );
    }
}

#[test]
fn p1a_intake_admission_refuses_stale_subordinate_fingerprints() {
    for (kind_fingerprint, schema_fingerprint) in [
        (STALE_FINGERPRINT, PROJECT_CONTEXT_SCHEMA_FINGERPRINT),
        (PROJECT_CONTEXT_KIND_FINGERPRINT, STALE_FINGERPRINT),
    ] {
        let mut definition = project_context_intake();
        set_project_context_fingerprint(&mut definition, kind_fingerprint, schema_fingerprint);
        let error = load_project_context_intake(&definition)
            .expect_err("stale subordinate fingerprint must be refused");
        assert_eq!(
            error.kind(),
            ArtifactRegistrationErrorKindV1::FingerprintMismatch
        );
    }
}

#[test]
fn p1a_intake_admission_refuses_wrong_kind_and_schema_bindings() {
    for field in ["artifact_kind_ref", "candidate_schema_ref"] {
        let mut definition = project_context_intake();
        definition[field] = Value::String("example.definition.wrong@1.0.0".to_owned());
        let error = load_project_context_intake(&definition)
            .expect_err("wrong P1A binding must be refused");
        assert_eq!(
            error.kind(),
            ArtifactRegistrationErrorKindV1::IncompatibleBinding
        );
    }
}

#[test]
fn p1a_intake_admission_refuses_missing_coverage() {
    let mut definition = project_context_intake();
    definition["coverage"] = json!([]);
    set_project_context_fingerprint(
        &mut definition,
        PROJECT_CONTEXT_KIND_FINGERPRINT,
        PROJECT_CONTEXT_SCHEMA_FINGERPRINT,
    );
    let error =
        load_project_context_intake(&definition).expect_err("missing coverage must be refused");
    assert_eq!(
        error.kind(),
        ArtifactRegistrationErrorKindV1::InvalidCoverage
    );
}

#[test]
fn p1a_intake_admission_refuses_unsupported_mode() {
    let mut definition = project_context_intake();
    definition["supported_modes"] = json!(["unsupported"]);
    set_project_context_fingerprint(
        &mut definition,
        PROJECT_CONTEXT_KIND_FINGERPRINT,
        PROJECT_CONTEXT_SCHEMA_FINGERPRINT,
    );
    let error =
        load_project_context_intake(&definition).expect_err("unsupported mode must be refused");
    assert_eq!(error.kind(), ArtifactRegistrationErrorKindV1::InvalidRecord);
}

#[test]
fn p1a_intake_admission_refuses_invalid_coverage_target() {
    let mut definition = project_context_intake();
    definition["coverage"][0]["target_paths"] = json!(["/not//normalized"]);
    set_project_context_fingerprint(
        &mut definition,
        PROJECT_CONTEXT_KIND_FINGERPRINT,
        PROJECT_CONTEXT_SCHEMA_FINGERPRINT,
    );
    let error = load_project_context_intake(&definition)
        .expect_err("invalid coverage target must be refused");
    assert_eq!(
        error.kind(),
        ArtifactRegistrationErrorKindV1::InvalidCoverage
    );
}
