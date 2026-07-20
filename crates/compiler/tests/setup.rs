#[cfg(not(unix))]
use handbook_compiler::run_setup;
use handbook_compiler::{
    plan_setup, plan_setup_with_decisions, run_setup_with_decisions, RepositoryReadinessStatus,
    SetupArtifactActionKind, SetupErrorCode, SetupErrorKind, SetupErrorReasonCode, SetupMode,
    SetupRequest, SetupRootAction,
};
use handbook_engine::{resolve_shipped_profile_decisions, REPOSITORY_IDENTITY_REPO_PATH};
use std::fs;
use tempfile::tempdir;

#[cfg(unix)]
use handbook_compiler::doctor_with_decisions;
#[cfg(unix)]
use handbook_engine::{
    resolve_profile_selection, shipped_root_artifact_instance_values, ArtifactInstanceRegistry,
    DefinitionFingerprint, DefinitionSource, DefinitionSourceBinding, ExactDefinitionRef,
    ProfileSelectionRequest, ResolvedProfileDecisions,
};
#[cfg(unix)]
use serde_json::{json, Value};
#[cfg(unix)]
use std::path::Path;

#[test]
fn setup_modes_serialize_to_the_frozen_wire_values() {
    assert_eq!(serde_json::to_string(&SetupMode::Auto).unwrap(), "\"auto\"");
    assert_eq!(serde_json::to_string(&SetupMode::Init).unwrap(), "\"init\"");
    assert_eq!(
        serde_json::to_string(&SetupMode::Refresh).unwrap(),
        "\"refresh\""
    );
}

#[test]
fn init_plans_root_creation_but_writes_no_selected_artifact() {
    let repo = tempdir().unwrap();
    let decisions = resolve_shipped_profile_decisions(repo.path()).unwrap();
    let request = SetupRequest::default();
    let plan = plan_setup_with_decisions(repo.path(), &request, &decisions).unwrap();

    assert_eq!(plan.resolved_mode, SetupMode::Init);
    assert_eq!(plan.root_action, SetupRootAction::Create);
    assert!(plan.artifacts.iter().all(|artifact| {
        matches!(
            artifact.action,
            SetupArtifactActionKind::AuthorRequired
                | SetupArtifactActionKind::ConditionIndeterminate
                | SetupArtifactActionKind::OptionalAbsent
                | SetupArtifactActionKind::Invalid
        )
    }));

    let outcome = run_setup_with_decisions(repo.path(), &request, &decisions).unwrap();
    assert_ne!(outcome.status, RepositoryReadinessStatus::Ready);
    assert!(repo.path().join(".handbook").is_dir());
    let identity = fs::read(repo.path().join(REPOSITORY_IDENTITY_REPO_PATH)).unwrap();
    assert_eq!(identity.len(), 71);
    assert!(identity.starts_with(b"sha256:"));
    for artifact in &outcome.plan.artifacts {
        assert!(!repo.path().join(&artifact.artifact.canonical_path).exists());
    }
}

#[cfg(unix)]
#[test]
fn mutation_admitted_setup_initializes_then_preserves_repository_identity() {
    let repo = tempdir().unwrap();
    let decisions = resolve_shipped_profile_decisions(repo.path()).unwrap();

    let first =
        run_setup_with_decisions(repo.path(), &SetupRequest::default(), &decisions).unwrap();
    assert!(matches!(
        first.status,
        RepositoryReadinessStatus::Ready | RepositoryReadinessStatus::ActionRequired
    ));
    let identity_path = repo.path().join(REPOSITORY_IDENTITY_REPO_PATH);
    let before = fs::read(&identity_path).unwrap();
    assert_eq!(before.len(), 71);

    let refresh = SetupRequest {
        mode: SetupMode::Refresh,
        rewrite: false,
        reset_state: false,
    };
    run_setup_with_decisions(repo.path(), &refresh, &decisions).unwrap();
    assert_eq!(fs::read(identity_path).unwrap(), before);
}

#[cfg(unix)]
#[test]
fn reset_state_never_selects_or_replaces_repository_identity() {
    let repo = tempdir().unwrap();
    let decisions = resolve_shipped_profile_decisions(repo.path()).unwrap();
    run_setup_with_decisions(repo.path(), &SetupRequest::default(), &decisions).unwrap();
    let identity_path = repo.path().join(REPOSITORY_IDENTITY_REPO_PATH);
    let before = fs::read(&identity_path).unwrap();
    fs::create_dir_all(repo.path().join(".handbook/state/runtime")).unwrap();
    fs::write(repo.path().join(".handbook/state/runtime/stale"), b"stale").unwrap();

    let reset = SetupRequest {
        mode: SetupMode::Refresh,
        rewrite: false,
        reset_state: true,
    };
    let plan = plan_setup_with_decisions(repo.path(), &reset, &decisions).unwrap();
    assert!(!plan
        .reset_paths
        .iter()
        .any(|path| path == REPOSITORY_IDENTITY_REPO_PATH));
    let outcome = run_setup_with_decisions(repo.path(), &reset, &decisions).unwrap();
    assert!(outcome.reset_applied);
    assert_eq!(fs::read(identity_path).unwrap(), before);
}

#[cfg(unix)]
#[test]
fn malformed_repository_identity_projects_a_typed_setup_error_without_repair() {
    let repo = tempdir().unwrap();
    fs::create_dir(repo.path().join(".handbook")).unwrap();
    let identity_path = repo.path().join(REPOSITORY_IDENTITY_REPO_PATH);
    let malformed = b"sha256:AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";
    fs::write(&identity_path, malformed).unwrap();
    let decisions = resolve_shipped_profile_decisions(repo.path()).unwrap();

    let error = run_setup_with_decisions(
        repo.path(),
        &SetupRequest {
            mode: SetupMode::Refresh,
            rewrite: false,
            reset_state: false,
        },
        &decisions,
    )
    .unwrap_err();
    assert_eq!(error.kind(), SetupErrorKind::RepositoryIdentity);
    assert_eq!(
        error.reason_code(),
        SetupErrorReasonCode::RepositoryIdentityUnsafe
    );
    assert_eq!(
        error.repo_relative_path(),
        Some(REPOSITORY_IDENTITY_REPO_PATH)
    );
    assert_eq!(fs::read(identity_path).unwrap(), malformed);
}

#[test]
fn rewrite_refuses_before_root_or_reset_mutation() {
    let repo = tempdir().unwrap();
    fs::create_dir_all(repo.path().join(".handbook/state")).unwrap();
    fs::write(repo.path().join(".handbook/state/keep.json"), b"{}").unwrap();
    let request = SetupRequest {
        mode: SetupMode::Refresh,
        rewrite: true,
        reset_state: true,
    };
    let error = plan_setup(repo.path(), &request).unwrap_err();
    assert_eq!(error.kind(), SetupErrorKind::MaterializerUnavailable);
    assert_eq!(
        error.reason_code(),
        SetupErrorReasonCode::RewriteHasNoMaterializer
    );
    assert!(repo.path().join(".handbook/state/keep.json").exists());
}

#[test]
fn root_symlink_and_non_directory_refuse_without_repair() {
    let repo = tempdir().unwrap();
    fs::write(repo.path().join(".handbook"), b"do not delete").unwrap();
    let error = plan_setup(repo.path(), &SetupRequest::default()).unwrap_err();
    assert_eq!(error.kind(), SetupErrorKind::InvalidCanonicalRoot);
    assert_eq!(error.reason_code(), SetupErrorReasonCode::RootNotDirectory);
    assert_eq!(
        fs::read(repo.path().join(".handbook")).unwrap(),
        b"do not delete"
    );
}

#[test]
fn setup_root_mode_and_request_table_is_total_and_ordered() {
    #[derive(Clone, Copy, Debug)]
    enum RootFixture {
        Missing,
        Directory,
        NonDirectory,
    }

    #[derive(Clone, Copy, Debug)]
    enum Expected {
        Plan(SetupMode, SetupRootAction),
        Error(SetupErrorCode),
    }

    #[derive(Clone, Copy, Debug)]
    struct Case {
        name: &'static str,
        root: RootFixture,
        mode: SetupMode,
        rewrite: bool,
        reset_state: bool,
        expected: Expected,
    }

    let cases = [
        Case {
            name: "auto missing",
            root: RootFixture::Missing,
            mode: SetupMode::Auto,
            rewrite: false,
            reset_state: false,
            expected: Expected::Plan(SetupMode::Init, SetupRootAction::Create),
        },
        Case {
            name: "auto directory",
            root: RootFixture::Directory,
            mode: SetupMode::Auto,
            rewrite: false,
            reset_state: false,
            expected: Expected::Plan(SetupMode::Refresh, SetupRootAction::Preserve),
        },
        Case {
            name: "init missing",
            root: RootFixture::Missing,
            mode: SetupMode::Init,
            rewrite: false,
            reset_state: false,
            expected: Expected::Plan(SetupMode::Init, SetupRootAction::Create),
        },
        Case {
            name: "refresh directory",
            root: RootFixture::Directory,
            mode: SetupMode::Refresh,
            rewrite: false,
            reset_state: false,
            expected: Expected::Plan(SetupMode::Refresh, SetupRootAction::Preserve),
        },
        Case {
            name: "init directory",
            root: RootFixture::Directory,
            mode: SetupMode::Init,
            rewrite: false,
            reset_state: false,
            expected: Expected::Error(SetupErrorCode::RootAlreadyInitialized),
        },
        Case {
            name: "refresh missing",
            root: RootFixture::Missing,
            mode: SetupMode::Refresh,
            rewrite: false,
            reset_state: false,
            expected: Expected::Error(SetupErrorCode::RefreshRootMissing),
        },
        Case {
            name: "auto non-directory",
            root: RootFixture::NonDirectory,
            mode: SetupMode::Auto,
            rewrite: false,
            reset_state: false,
            expected: Expected::Error(SetupErrorCode::RootNotDirectory),
        },
        Case {
            name: "init non-directory",
            root: RootFixture::NonDirectory,
            mode: SetupMode::Init,
            rewrite: false,
            reset_state: false,
            expected: Expected::Error(SetupErrorCode::RootNotDirectory),
        },
        Case {
            name: "refresh non-directory",
            root: RootFixture::NonDirectory,
            mode: SetupMode::Refresh,
            rewrite: false,
            reset_state: false,
            expected: Expected::Error(SetupErrorCode::RootNotDirectory),
        },
        Case {
            name: "auto missing rewrite resolves init before flags",
            root: RootFixture::Missing,
            mode: SetupMode::Auto,
            rewrite: true,
            reset_state: false,
            expected: Expected::Error(SetupErrorCode::InitRejectsRefreshFlags),
        },
        Case {
            name: "auto missing reset resolves init before flags",
            root: RootFixture::Missing,
            mode: SetupMode::Auto,
            rewrite: false,
            reset_state: true,
            expected: Expected::Error(SetupErrorCode::InitRejectsRefreshFlags),
        },
        Case {
            name: "init flags precede existing root",
            root: RootFixture::Directory,
            mode: SetupMode::Init,
            rewrite: true,
            reset_state: true,
            expected: Expected::Error(SetupErrorCode::InitRejectsRefreshFlags),
        },
        Case {
            name: "auto directory rewrite resolves refresh",
            root: RootFixture::Directory,
            mode: SetupMode::Auto,
            rewrite: true,
            reset_state: false,
            expected: Expected::Error(SetupErrorCode::RewriteHasNoMaterializer),
        },
        Case {
            name: "refresh directory rewrite",
            root: RootFixture::Directory,
            mode: SetupMode::Refresh,
            rewrite: true,
            reset_state: false,
            expected: Expected::Error(SetupErrorCode::RewriteHasNoMaterializer),
        },
        Case {
            name: "auto directory reset",
            root: RootFixture::Directory,
            mode: SetupMode::Auto,
            rewrite: false,
            reset_state: true,
            expected: Expected::Plan(SetupMode::Refresh, SetupRootAction::Preserve),
        },
        Case {
            name: "refresh directory reset",
            root: RootFixture::Directory,
            mode: SetupMode::Refresh,
            rewrite: false,
            reset_state: true,
            expected: Expected::Plan(SetupMode::Refresh, SetupRootAction::Preserve),
        },
        Case {
            name: "refresh missing rewrite checks root first",
            root: RootFixture::Missing,
            mode: SetupMode::Refresh,
            rewrite: true,
            reset_state: false,
            expected: Expected::Error(SetupErrorCode::RefreshRootMissing),
        },
        Case {
            name: "refresh non-directory rewrite checks root first",
            root: RootFixture::NonDirectory,
            mode: SetupMode::Refresh,
            rewrite: true,
            reset_state: false,
            expected: Expected::Error(SetupErrorCode::RootNotDirectory),
        },
    ];
    let decisions =
        resolve_shipped_profile_decisions(std::path::Path::new(env!("CARGO_MANIFEST_DIR")))
            .unwrap();

    for case in cases {
        let repo = tempdir().unwrap();
        match case.root {
            RootFixture::Missing => {}
            RootFixture::Directory => fs::create_dir(repo.path().join(".handbook")).unwrap(),
            RootFixture::NonDirectory => {
                fs::write(repo.path().join(".handbook"), b"preserve me").unwrap()
            }
        }
        let request = SetupRequest {
            mode: case.mode,
            rewrite: case.rewrite,
            reset_state: case.reset_state,
        };
        let actual = plan_setup_with_decisions(repo.path(), &request, &decisions);

        match case.expected {
            Expected::Plan(mode, action) => {
                let plan = actual.unwrap_or_else(|error| {
                    panic!("{}: unexpected error {:?}", case.name, error.code())
                });
                assert_eq!(plan.requested_mode, case.mode, "{}", case.name);
                assert_eq!(plan.resolved_mode, mode, "{}", case.name);
                assert_eq!(plan.root_action, action, "{}", case.name);
            }
            Expected::Error(code) => {
                assert_eq!(actual.unwrap_err().code(), code, "{}", case.name);
            }
        }
    }
}

#[cfg(unix)]
#[test]
fn real_root_symlink_is_refused_for_auto_init_and_refresh_without_repair() {
    use std::os::unix::fs::symlink;

    let decisions =
        resolve_shipped_profile_decisions(std::path::Path::new(env!("CARGO_MANIFEST_DIR")))
            .unwrap();
    for mode in [SetupMode::Auto, SetupMode::Init, SetupMode::Refresh] {
        let repo = tempdir().unwrap();
        let outside = tempdir().unwrap();
        fs::write(outside.path().join("keep"), b"keep").unwrap();
        symlink(outside.path(), repo.path().join(".handbook")).unwrap();

        let error = plan_setup_with_decisions(
            repo.path(),
            &SetupRequest {
                mode,
                rewrite: false,
                reset_state: false,
            },
            &decisions,
        )
        .unwrap_err();

        assert_eq!(error.code(), SetupErrorCode::RootSymlinkRefused);
        assert!(fs::symlink_metadata(repo.path().join(".handbook"))
            .unwrap()
            .file_type()
            .is_symlink());
        assert_eq!(fs::read(outside.path().join("keep")).unwrap(), b"keep");
    }
}

#[test]
fn default_and_injected_setup_project_the_same_closure() {
    let repo = tempdir().unwrap();
    fs::create_dir(repo.path().join(".handbook")).unwrap();
    let decisions = resolve_shipped_profile_decisions(repo.path()).unwrap();
    let request = SetupRequest {
        mode: SetupMode::Refresh,
        rewrite: false,
        reset_state: false,
    };
    assert_eq!(
        plan_setup(repo.path(), &request).unwrap(),
        plan_setup_with_decisions(repo.path(), &request, &decisions).unwrap()
    );
}

#[cfg(unix)]
#[test]
fn custom_profile_setup_and_doctor_project_byte_equal_closure_rows() {
    let repo = tempdir().unwrap();
    fs::create_dir_all(repo.path().join(".handbook/project")).unwrap();
    write_ready_profile_artifacts(repo.path());
    let decisions = ready_custom_decisions(repo.path());
    let request = SetupRequest {
        mode: SetupMode::Refresh,
        rewrite: false,
        reset_state: false,
    };

    let setup = plan_setup_with_decisions(repo.path(), &request, &decisions).unwrap();
    let doctor = doctor_with_decisions(repo.path(), &decisions).unwrap();

    assert_eq!(doctor.status, RepositoryReadinessStatus::Ready);
    assert_eq!(setup.profile_ref, doctor.profile_ref);
    assert_eq!(setup.profile_fingerprint, doctor.profile_fingerprint);
    assert_eq!(
        setup.stable_role_registry_ref,
        doctor.stable_role_registry_ref
    );
    assert_eq!(
        setup.stable_role_registry_fingerprint,
        doctor.stable_role_registry_fingerprint
    );
    assert_eq!(setup.conditions, doctor.conditions);
    assert_eq!(setup.capabilities, doctor.capabilities);
    assert_eq!(setup.artifacts.len(), doctor.artifacts.len());
    for (setup_row, doctor_row) in setup.artifacts.iter().zip(&doctor.artifacts) {
        assert_eq!(&setup_row.artifact, doctor_row);
    }
}

#[cfg(unix)]
#[test]
fn setup_and_doctor_cover_all_statuses_with_exact_action_precedence() {
    let ready_repo = tempdir().unwrap();
    fs::create_dir_all(ready_repo.path().join(".handbook/project")).unwrap();
    write_ready_profile_artifacts(ready_repo.path());
    let ready = ready_custom_decisions(ready_repo.path());
    assert_setup_doctor_status_and_actions(
        ready_repo.path(),
        &ready,
        RepositoryReadinessStatus::Ready,
        &[
            (
                "environment_context",
                SetupArtifactActionKind::OptionalAbsent,
            ),
            ("project_authority", SetupArtifactActionKind::Preserve),
            ("project_context", SetupArtifactActionKind::Preserve),
        ],
    );

    let action_repo = tempdir().unwrap();
    fs::create_dir_all(action_repo.path().join(".handbook/project")).unwrap();
    write_ready_profile_artifacts(action_repo.path());
    fs::remove_file(action_repo.path().join(".handbook/project/context.yaml")).unwrap();
    let action_required = ready_custom_decisions(action_repo.path());
    assert_setup_doctor_status_and_actions(
        action_repo.path(),
        &action_required,
        RepositoryReadinessStatus::ActionRequired,
        &[
            (
                "environment_context",
                SetupArtifactActionKind::OptionalAbsent,
            ),
            ("project_authority", SetupArtifactActionKind::Preserve),
            ("project_context", SetupArtifactActionKind::AuthorRequired),
        ],
    );

    let indeterminate_repo = tempdir().unwrap();
    fs::create_dir_all(indeterminate_repo.path().join(".handbook/project")).unwrap();
    let indeterminate = resolve_shipped_profile_decisions(indeterminate_repo.path()).unwrap();
    assert_setup_doctor_status_and_actions(
        indeterminate_repo.path(),
        &indeterminate,
        RepositoryReadinessStatus::Indeterminate,
        &[
            (
                "environment_context",
                SetupArtifactActionKind::ConditionIndeterminate,
            ),
            ("project_authority", SetupArtifactActionKind::AuthorRequired),
            ("project_context", SetupArtifactActionKind::AuthorRequired),
        ],
    );

    let invalid_repo = tempdir().unwrap();
    fs::create_dir_all(invalid_repo.path().join(".handbook/project")).unwrap();
    fs::write(
        invalid_repo.path().join(".handbook/project/charter.yaml"),
        b"schema_id: wrong\n",
    )
    .unwrap();
    let invalid = resolve_shipped_profile_decisions(invalid_repo.path()).unwrap();
    assert_setup_doctor_status_and_actions(
        invalid_repo.path(),
        &invalid,
        RepositoryReadinessStatus::Invalid,
        &[
            (
                "environment_context",
                SetupArtifactActionKind::ConditionIndeterminate,
            ),
            ("project_authority", SetupArtifactActionKind::Invalid),
            ("project_context", SetupArtifactActionKind::AuthorRequired),
        ],
    );
}

#[cfg(unix)]
fn assert_setup_doctor_status_and_actions(
    repo: &Path,
    decisions: &ResolvedProfileDecisions,
    expected_status: RepositoryReadinessStatus,
    expected_actions: &[(&str, SetupArtifactActionKind)],
) {
    let request = SetupRequest {
        mode: SetupMode::Refresh,
        rewrite: false,
        reset_state: false,
    };
    let setup = run_setup_with_decisions(repo, &request, decisions).unwrap();
    let doctor = doctor_with_decisions(repo, decisions).unwrap();

    assert_eq!(setup.status, expected_status);
    assert_eq!(doctor.status, expected_status);
    for (instance_id, expected_action) in expected_actions {
        let row = setup
            .plan
            .artifacts
            .iter()
            .find(|row| row.artifact.instance_id == *instance_id)
            .unwrap_or_else(|| panic!("missing setup row {instance_id}"));
        assert_eq!(row.action, *expected_action, "{instance_id}");
    }
}

#[cfg(unix)]
#[test]
fn injected_ready_profile_reset_succeeds_and_preserves_non_state_bytes() {
    let repo = tempdir().unwrap();
    fs::create_dir_all(repo.path().join(".handbook/project")).unwrap();
    write_ready_profile_artifacts(repo.path());
    fs::create_dir_all(repo.path().join(".handbook/state/nested")).unwrap();
    fs::write(repo.path().join(".handbook/state/a.yaml"), b"a: 1\n").unwrap();
    fs::write(repo.path().join(".handbook/state/nested/b.yaml"), b"b: 2\n").unwrap();
    fs::write(repo.path().join(".handbook/KEEP"), b"keep\n").unwrap();
    let charter_before = fs::read(repo.path().join(".handbook/project/charter.yaml")).unwrap();
    let context_before = fs::read(repo.path().join(".handbook/project/context.yaml")).unwrap();
    let decisions = ready_custom_decisions(repo.path());
    let request = SetupRequest {
        mode: SetupMode::Refresh,
        rewrite: false,
        reset_state: true,
    };

    let outcome = run_setup_with_decisions(repo.path(), &request, &decisions).unwrap();

    assert_eq!(outcome.status, RepositoryReadinessStatus::Ready);
    assert!(outcome.reset_applied);
    assert!(!repo.path().join(".handbook/state/a.yaml").exists());
    assert!(!repo.path().join(".handbook/state/nested/b.yaml").exists());
    assert_eq!(
        fs::read(repo.path().join(".handbook/project/charter.yaml")).unwrap(),
        charter_before
    );
    assert_eq!(
        fs::read(repo.path().join(".handbook/project/context.yaml")).unwrap(),
        context_before
    );
    assert_eq!(
        fs::read(repo.path().join(".handbook/KEEP")).unwrap(),
        b"keep\n"
    );
}

#[cfg(unix)]
#[test]
fn injected_ready_profile_reset_refuses_symlink_before_any_state_mutation() {
    use std::os::unix::fs::symlink;

    let repo = tempdir().unwrap();
    fs::create_dir_all(repo.path().join(".handbook/project")).unwrap();
    write_ready_profile_artifacts(repo.path());
    fs::create_dir_all(repo.path().join(".handbook/state")).unwrap();
    fs::write(repo.path().join(".handbook/state/a.yaml"), b"a: 1\n").unwrap();
    let outside = tempdir().unwrap();
    symlink(outside.path(), repo.path().join(".handbook/state/z_link")).unwrap();
    let decisions = ready_custom_decisions(repo.path());
    let request = SetupRequest {
        mode: SetupMode::Refresh,
        rewrite: false,
        reset_state: true,
    };

    let error = run_setup_with_decisions(repo.path(), &request, &decisions).unwrap_err();

    assert_eq!(error.kind(), SetupErrorKind::RuntimeStatePlan);
    assert_eq!(
        error.reason_code(),
        SetupErrorReasonCode::RuntimeStateTargetUnsafe
    );
    assert_eq!(
        fs::read(repo.path().join(".handbook/state/a.yaml")).unwrap(),
        b"a: 1\n"
    );
    assert!(repo.path().join(".handbook/state/z_link").exists());
}

#[cfg(unix)]
fn write_ready_profile_artifacts(repo: &Path) {
    let authority = json!({
        "schema_id": "handbook.artifact.project-authority",
        "schema_version": "1.0",
        "record_id": "example.record.project-authority",
        "policy": {"revision": "1", "authority_statement": "The project authority is explicit."},
        "governance": {
            "decision_authority": ["Owner"], "required_approvals": ["Owner"],
            "exception_policy": "Exceptions require explicit approval.",
            "review_triggers": ["Authority changes"], "reassessment_triggers": ["Scope changes"]
        },
        "engineering_posture": {"dimensions": ["Reliability"], "red_lines": ["No silent authority mutation"]}
    });
    let context = json!({
        "schema_id": "handbook.artifact.project-context", "schema_version": "1.0",
        "record_id": "example.record.project-context", "summary": "Summary",
        "system_boundaries": ["API"], "ownership": ["Team"],
        "authoritative_references": [], "known_unknowns": []
    });
    for (relative, value) in [
        (".handbook/project/charter.yaml", authority),
        (".handbook/project/context.yaml", context),
    ] {
        let path = repo.join(relative);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, serde_yaml_bw::to_string(&value).unwrap()).unwrap();
    }
}

#[cfg(unix)]
fn ready_custom_decisions(repo: &Path) -> ResolvedProfileDecisions {
    const ROLE_REF: &str = "handbook.roles.core@1.1.0";
    let baseline = shipped_selection();
    let mut instances = shipped_root_artifact_instance_values();
    let environment = instances
        .iter_mut()
        .find(|instance| instance["id"] == json!("environment_context"))
        .unwrap();
    environment["requiredness"] = json!({"mode": "optional", "condition_ref": null});
    let instance_registry =
        ArtifactInstanceRegistry::resolve(&instances, baseline.artifact_kind_registry(), &[])
            .unwrap();
    let profile_ref = "example.profile.ready-root@1.0.0";
    let artifact_names = [
        "project-authority",
        "project-context",
        "environment-context",
        "work-specification",
        "decision-record",
        "risk-record",
    ];
    let schema_refs = artifact_names.map(|name| format!("handbook.schemas.artifacts.{name}@1.0.0"));
    let kind_refs = artifact_names.map(|name| format!("handbook.artifact-kind.{name}@1.0.0"));
    let definition = json!({
        "schema_id": "handbook.instance-profile",
        "schema_version": "1.0",
        "profile_id": "example.profile.ready-root",
        "profile_version": "1.0.0",
        "profile_scope": "shipped",
        "extends_profile_ref": null,
        "stable_role_registry": {
            "ref": ROLE_REF,
            "fingerprint": baseline.stable_role_registry().fingerprint().as_str(),
        },
        "schema_registry_sources": schema_refs,
        "artifact_kind_sources": kind_refs,
        "artifact_instances": instances,
        "vocabulary_ref": baseline.vocabulary().exact_ref().as_str(),
        "context_resolution_ref": baseline.context_resolution().exact_ref().as_str(),
        "projection_catalog_refs": [],
        "posture_evaluation_policy": null,
        "dock_requirement_refs": [],
        "adapter_overlay_refs": [],
        "extensions": {},
    });
    let mut dependencies = Vec::new();
    dependencies.push(json!({
        "definition_class": "stable_role_registry",
        "reference": ROLE_REF,
        "fingerprint": baseline.stable_role_registry().fingerprint().as_str(),
    }));
    for name in artifact_names {
        let schema_ref = exact(&format!("handbook.schemas.artifacts.{name}@1.0.0"));
        let kind_ref = exact(&format!("handbook.artifact-kind.{name}@1.0.0"));
        dependencies.push(json!({
            "definition_class": "schema_entry",
            "reference": schema_ref.as_str(),
            "fingerprint": baseline.artifact_kind_registry().schema_registry()
                .entry(&schema_ref).unwrap().entry_fingerprint().as_str(),
        }));
        dependencies.push(json!({
            "definition_class": "artifact_kind",
            "reference": kind_ref.as_str(),
            "fingerprint": baseline.artifact_kind_registry()
                .kind(&kind_ref).unwrap().definition_fingerprint().as_str(),
        }));
    }
    dependencies.extend([
        json!({
            "definition_class": "artifact_instance_registry",
            "reference": profile_ref,
            "fingerprint": instance_registry.fingerprint().as_str(),
        }),
        json!({
            "definition_class": "vocabulary",
            "reference": baseline.vocabulary().exact_ref().as_str(),
            "fingerprint": baseline.vocabulary().vocabulary_fingerprint().as_str(),
        }),
        json!({
            "definition_class": "context_resolution",
            "reference": baseline.context_resolution().exact_ref().as_str(),
            "fingerprint": baseline.context_resolution().definition_fingerprint().as_str(),
        }),
    ]);
    dependencies.sort_by(|left: &Value, right: &Value| {
        (
            left["definition_class"].as_str(),
            left["reference"].as_str(),
        )
            .cmp(&(
                right["definition_class"].as_str(),
                right["reference"].as_str(),
            ))
    });
    let profile_fingerprint = DefinitionFingerprint::from_json_value(&json!({
        "definition": definition,
        "dependencies": dependencies,
    }))
    .unwrap();
    let mut profile = definition;
    profile["profile_fingerprint"] = Value::String(profile_fingerprint.to_string());
    let profile_path = "ready.profile.yaml";
    fs::write(
        repo.join(profile_path),
        serde_yaml_bw::to_string(&profile).unwrap(),
    )
    .unwrap();

    let selected = resolve_profile_selection(
        repo,
        ProfileSelectionRequest {
            selected_profile_ref: exact(profile_ref),
            profile_sources: vec![DefinitionSourceBinding {
                definition_ref: exact(profile_ref),
                source: DefinitionSource::RepositoryPath(profile_path.to_owned()),
            }],
            stable_role_registry_sources: vec![builtin(ROLE_REF)],
            schema_entry_sources: artifact_names
                .iter()
                .map(|name| builtin(&format!("handbook.schemas.artifacts.{name}@1.0.0")))
                .collect(),
            artifact_kind_sources: artifact_names
                .iter()
                .map(|name| builtin(&format!("handbook.artifact-kind.{name}@1.0.0")))
                .collect(),
            semantic_capability_sources: vec![builtin(
                "handbook.capabilities.constitutional-root@1.0.0",
            )],
            semantic_validator_sources: vec![builtin(
                "handbook.semantic-validation.constitutional-root@1.0.0",
            )],
            project_condition_sources: vec![],
            vocabulary_sources: vec![builtin("handbook.vocabulary.shipped-root@1.0.0")],
            context_resolution_sources: vec![builtin(
                "handbook.context-resolution.shipped-root@1.0.0",
            )],
            context_resolution_policy_sources: vec![
                builtin("handbook.mutation-matcher.core@1.0.0"),
                builtin("handbook.resolution-escalation.core@1.0.0"),
                builtin("handbook.memory-promotion.core@1.0.0"),
            ],
            allowed_schema_roots: vec!["definitions/schemas".to_owned()],
        },
    )
    .unwrap();
    ResolvedProfileDecisions::from_profile(&selected).unwrap()
}

#[cfg(unix)]
fn shipped_selection() -> handbook_engine::ResolvedInstanceProfile {
    let artifact_names = [
        "project-authority",
        "project-context",
        "environment-context",
        "work-specification",
        "decision-record",
        "risk-record",
    ];
    resolve_profile_selection(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../engine"),
        ProfileSelectionRequest {
            selected_profile_ref: exact("handbook.profile.shipped-root@1.0.0"),
            profile_sources: vec![builtin("handbook.profile.shipped-root@1.0.0")],
            stable_role_registry_sources: vec![builtin("handbook.roles.core@1.1.0")],
            schema_entry_sources: artifact_names
                .iter()
                .map(|name| builtin(&format!("handbook.schemas.artifacts.{name}@1.0.0")))
                .collect(),
            artifact_kind_sources: artifact_names
                .iter()
                .map(|name| builtin(&format!("handbook.artifact-kind.{name}@1.0.0")))
                .collect(),
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
            context_resolution_sources: vec![builtin(
                "handbook.context-resolution.shipped-root@1.0.0",
            )],
            context_resolution_policy_sources: vec![
                builtin("handbook.mutation-matcher.core@1.0.0"),
                builtin("handbook.resolution-escalation.core@1.0.0"),
                builtin("handbook.memory-promotion.core@1.0.0"),
            ],
            allowed_schema_roots: vec!["definitions/schemas".to_owned()],
        },
    )
    .unwrap()
}

#[cfg(unix)]
fn exact(value: &str) -> ExactDefinitionRef {
    ExactDefinitionRef::parse(value).unwrap()
}

#[cfg(unix)]
fn builtin(value: &str) -> DefinitionSourceBinding {
    let definition_ref = exact(value);
    DefinitionSourceBinding {
        definition_ref: definition_ref.clone(),
        source: DefinitionSource::BuiltIn(definition_ref),
    }
}

#[cfg(windows)]
#[test]
fn windows_profile_inspection_refusal_prevents_setup_mutation() {
    let repo = tempdir().unwrap();
    let outcome = run_setup(repo.path(), &SetupRequest::default()).unwrap();
    assert_eq!(outcome.status, RepositoryReadinessStatus::Indeterminate);
    assert!(repo.path().join(".handbook").is_dir());
    let identity = fs::read(repo.path().join(REPOSITORY_IDENTITY_REPO_PATH)).unwrap();
    assert_eq!(identity.len(), 71);
    assert!(identity.starts_with(b"sha256:"));
    for artifact in &outcome.plan.artifacts {
        assert!(!repo.path().join(&artifact.artifact.canonical_path).exists());
    }
}
