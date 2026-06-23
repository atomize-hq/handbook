#[path = "support/pipeline_proof_corpus_support.rs"]
mod pipeline_proof_corpus_support;

use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use handbook_pipeline::{
    pipeline::{load_pipeline_definition, supported_route_state_variables},
    pipeline_route::{resolve_pipeline_route, RouteVariables},
    route_state::{
        build_route_basis, effective_route_basis_run, load_route_state,
        load_route_state_with_storage_layout, load_route_state_with_supported_variables,
        load_trusted_pipeline_session, load_trusted_pipeline_session_with_storage_layout,
        persist_route_basis, persist_route_basis_with_storage_layout, set_route_state,
        set_route_state_with_storage_layout, RouteBasisPersistOutcome, RouteBasisPersistRefusal,
        RouteBasisStageStatus, RouteState, RouteStateMutation, RouteStateMutationOutcome,
        RouteStateMutationRefusal, RouteStateReadError, RouteStateStoreError, RouteStateValue,
        TrustedPipelineSessionRefusal, ROUTE_STATE_AUDIT_LIMIT, ROUTE_STATE_SCHEMA_VERSION,
    },
    PipelineStorageLayoutContract,
};

fn write_file(path: &Path, contents: &str) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("mkdirs");
    }
    std::fs::write(path, contents).expect("write");
}

fn seed_complete_profile_pack(repo_root: &Path, profile_id: &str) {
    write_file(
        &repo_root.join(format!("core/profiles/{profile_id}/profile.yaml")),
        &format!("kind: profile\nid: {profile_id}\n"),
    );
    write_file(
        &repo_root.join(format!("core/profiles/{profile_id}/commands.yaml")),
        "commands: []\n",
    );
    write_file(
        &repo_root.join(format!("core/profiles/{profile_id}/conventions.md")),
        "# conventions\n",
    );
}

fn seed_incomplete_profile_pack(repo_root: &Path, profile_id: &str) {
    write_file(
        &repo_root.join(format!("core/profiles/{profile_id}/profile.yaml")),
        &format!("kind: profile\nid: {profile_id}\n"),
    );
}

fn seed_run_inventory(repo_root: &Path) {
    write_file(&repo_root.join("core/runners/codex-cli.md"), "# runner");
    write_file(
        &repo_root.join("core/runners/examples/runner.example.md"),
        "# ignored noise",
    );
    seed_complete_profile_pack(repo_root, "python-uv");
    write_file(
        &repo_root.join("core/profiles/_template/profile.yaml"),
        "kind: profile\nid: _template\n",
    );
    write_file(&repo_root.join("core/profiles/.DS_Store"), "noise");
}

fn repo_root_string(repo_root: &Path) -> String {
    repo_root.to_string_lossy().into_owned()
}

fn state_path(repo_root: &Path, pipeline_id: &str) -> PathBuf {
    repo_root
        .join(".handbook")
        .join("state")
        .join("pipeline")
        .join(format!("{pipeline_id}.yaml"))
}

fn custom_storage_layout() -> PipelineStorageLayoutContract {
    PipelineStorageLayoutContract::try_from_paths(
        ".custom_handbook/state",
        ".custom_handbook/state/pipelines",
        ".custom_handbook/state/pipelines/stage_capture",
        ".custom_handbook/state/pipelines/capture_cache",
        "custom_artifacts/handoff/feature_slice",
    )
    .expect("valid custom storage layout")
}

fn custom_state_path(
    repo_root: &Path,
    pipeline_id: &str,
    storage_layout: PipelineStorageLayoutContract,
) -> PathBuf {
    repo_root
        .join(storage_layout.pipeline_dir_relative())
        .join(format!("{pipeline_id}.yaml"))
}

fn lock_path(state_path: &Path) -> PathBuf {
    state_path.with_extension("lock")
}

fn acquire_unix_lock(path: &Path) -> std::fs::File {
    use std::os::unix::io::AsRawFd;

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("mkdir lock parent");
    }

    let file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(path)
        .expect("open lock file");

    let rc = unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_EX) };
    assert_eq!(rc, 0, "acquire lock");
    file
}

fn release_unix_lock(file: &std::fs::File) {
    use std::os::unix::io::AsRawFd;

    let rc = unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_UN) };
    assert_eq!(rc, 0, "release lock");
}

#[test]
fn route_basis_defaults_runner_and_profile_into_run_snapshot() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    let definition = load_pipeline_definition(&repo_root, "core/pipelines/foundation_inputs.yaml")
        .expect("pipeline fixture");
    let supported_variables = supported_route_state_variables(&definition);
    let state = load_route_state_with_supported_variables(
        &repo_root,
        &definition.header.id,
        &supported_variables,
    )
    .expect("state");
    let route = resolve_pipeline_route(
        &definition,
        &RouteVariables::new(state.routing.clone()).expect("route variables"),
    )
    .expect("route");

    let route_basis = build_route_basis(&repo_root, &definition, &state, &route).expect("basis");

    assert_eq!(route_basis.run.runner.as_deref(), Some("codex-cli"));
    assert_eq!(route_basis.run.profile.as_deref(), Some("python-uv"));
    assert_eq!(
        route_basis.run.repo_root.as_deref(),
        Some(handbook_pipeline::route_state::ROUTE_BASIS_REPO_ROOT_SENTINEL)
    );
    assert_eq!(
        route_basis.run,
        effective_route_basis_run(&repo_root, &definition, &state)
    );
}

#[test]
fn persist_route_basis_accepts_defaulted_run_snapshot_against_unset_state_run() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    let definition = load_pipeline_definition(&repo_root, "core/pipelines/foundation_inputs.yaml")
        .expect("pipeline fixture");
    let supported_variables = supported_route_state_variables(&definition);
    let state = load_route_state_with_supported_variables(
        &repo_root,
        &definition.header.id,
        &supported_variables,
    )
    .expect("state");
    let route = resolve_pipeline_route(
        &definition,
        &RouteVariables::new(state.routing.clone()).expect("route variables"),
    )
    .expect("route");
    let route_basis = build_route_basis(&repo_root, &definition, &state, &route).expect("basis");

    let outcome =
        persist_route_basis(&repo_root, &definition.header.id, route_basis).expect("persist");

    match outcome {
        RouteBasisPersistOutcome::Applied(state) => {
            assert_eq!(
                state
                    .route_basis
                    .as_ref()
                    .and_then(|basis| basis.run.runner.as_deref()),
                Some("codex-cli")
            );
            assert_eq!(
                state
                    .route_basis
                    .as_ref()
                    .and_then(|basis| basis.run.profile.as_deref()),
                Some("python-uv")
            );
            assert_eq!(
                state
                    .route_basis
                    .as_ref()
                    .and_then(|basis| basis.run.repo_root.as_deref()),
                Some(handbook_pipeline::route_state::ROUTE_BASIS_REPO_ROOT_SENTINEL)
            );
        }
        RouteBasisPersistOutcome::Refused(refusal) => {
            panic!("expected route basis persist to apply, got {refusal:?}")
        }
    }
}

#[test]
fn route_basis_round_trips_when_written_by_resolve() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    let definition = load_pipeline_definition(&repo_root, "core/pipelines/foundation_inputs.yaml")
        .expect("pipeline fixture");
    let supported_variables = supported_route_state_variables(&definition);
    let state = load_route_state_with_supported_variables(
        &repo_root,
        &definition.header.id,
        &supported_variables,
    )
    .expect("state");
    let route = resolve_pipeline_route(
        &definition,
        &RouteVariables::new(state.routing.clone()).expect("route variables"),
    )
    .expect("route");
    let route_basis = build_route_basis(&repo_root, &definition, &state, &route).expect("basis");

    let outcome = persist_route_basis(&repo_root, &definition.header.id, route_basis.clone())
        .expect("persist");

    let persisted_state = match outcome {
        RouteBasisPersistOutcome::Applied(state) => *state,
        RouteBasisPersistOutcome::Refused(refusal) => {
            panic!("expected route basis persist to apply, got {refusal:?}")
        }
    };
    assert_eq!(persisted_state.route_basis.as_ref(), Some(&route_basis));

    let reloaded_state = load_route_state_with_supported_variables(
        &repo_root,
        &definition.header.id,
        &supported_variables,
    )
    .expect("reload state");
    assert_eq!(reloaded_state.route_basis.as_ref(), Some(&route_basis));
}

#[test]
fn trusted_pipeline_session_refuses_missing_route_basis() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    let (definition, _) =
        pipeline_proof_corpus_support::load_foundation_inputs_definition(&repo_root);

    let err = load_trusted_pipeline_session(&repo_root, &definition)
        .expect_err("missing route_basis should refuse");

    match err {
        TrustedPipelineSessionRefusal::MissingRouteBasis { pipeline_id } => {
            assert_eq!(
                pipeline_id,
                pipeline_proof_corpus_support::FOUNDATION_INPUTS_PIPELINE_ID
            );
        }
        other => panic!("expected missing-route-basis refusal, got {other:?}"),
    }
}

#[test]
fn trusted_pipeline_session_refuses_stale_route_basis() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    let (definition, supported_variables) =
        pipeline_proof_corpus_support::load_foundation_inputs_definition(&repo_root);
    let _ = pipeline_proof_corpus_support::persist_foundation_inputs_route_basis(&repo_root);
    pipeline_proof_corpus_support::apply_foundation_inputs_state_mutation(
        &repo_root,
        &supported_variables,
        RouteStateMutation::RoutingVariable {
            variable: "needs_project_context".to_string(),
            value: true,
        },
    );

    let err = load_trusted_pipeline_session(&repo_root, &definition)
        .expect_err("stale basis should refuse");

    match err {
        TrustedPipelineSessionRefusal::StaleRouteBasis {
            pipeline_id,
            reason,
        } => {
            assert_eq!(
                pipeline_id,
                pipeline_proof_corpus_support::FOUNDATION_INPUTS_PIPELINE_ID
            );
            assert!(reason.contains("revision"), "{reason}");
        }
        other => panic!("expected stale-route-basis refusal, got {other:?}"),
    }
}

#[test]
fn trusted_pipeline_session_refuses_malformed_route_basis() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    let (definition, _) =
        pipeline_proof_corpus_support::load_foundation_inputs_definition(&repo_root);
    let _ = pipeline_proof_corpus_support::persist_foundation_inputs_route_basis(&repo_root);
    let path = pipeline_proof_corpus_support::pipeline_state_path(&repo_root);
    let mut state: RouteState =
        serde_yaml_bw::from_str(&std::fs::read_to_string(&path).expect("read state"))
            .expect("deserialize state");
    let forged_stage = state
        .route_basis
        .as_mut()
        .expect("route_basis")
        .route
        .iter_mut()
        .find(|stage| stage.stage_id == pipeline_proof_corpus_support::STAGE_10_FEATURE_SPEC_ID)
        .expect("stage.10_feature_spec");
    forged_stage.status = RouteBasisStageStatus::Active;
    forged_stage.reason = None;
    std::fs::write(
        &path,
        serde_yaml_bw::to_string(&state).expect("serialize state"),
    )
    .expect("write state");

    let err = load_trusted_pipeline_session(&repo_root, &definition)
        .expect_err("malformed route_basis should refuse");

    match err {
        TrustedPipelineSessionRefusal::MalformedRouteBasis {
            pipeline_id,
            reason,
        } => {
            assert_eq!(
                pipeline_id,
                pipeline_proof_corpus_support::FOUNDATION_INPUTS_PIPELINE_ID
            );
            assert!(reason.contains("stage.10_feature_spec"), "{reason}");
            assert!(reason.contains("status `active`"), "{reason}");
            assert!(reason.contains("canonical `blocked`"), "{reason}");
        }
        other => panic!("expected malformed-route-basis refusal, got {other:?}"),
    }
}

#[test]
fn trusted_pipeline_session_classifies_load_time_malformed_route_basis() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    let (definition, _) =
        pipeline_proof_corpus_support::load_foundation_inputs_definition(&repo_root);
    let _ = pipeline_proof_corpus_support::persist_foundation_inputs_route_basis(&repo_root);
    let path = pipeline_proof_corpus_support::pipeline_state_path(&repo_root);
    let mut state: RouteState =
        serde_yaml_bw::from_str(&std::fs::read_to_string(&path).expect("read state"))
            .expect("deserialize state");
    state
        .route_basis
        .as_mut()
        .expect("route_basis")
        .schema_version = "route_basis.invalid".to_string();
    std::fs::write(
        &path,
        serde_yaml_bw::to_string(&state).expect("serialize state"),
    )
    .expect("write malformed route_basis state");

    let err = load_trusted_pipeline_session(&repo_root, &definition)
        .expect_err("load-time malformed route_basis should refuse");

    match err {
        TrustedPipelineSessionRefusal::MalformedRouteBasis {
            pipeline_id,
            reason,
        } => {
            assert_eq!(
                pipeline_id,
                pipeline_proof_corpus_support::FOUNDATION_INPUTS_PIPELINE_ID
            );
            assert!(reason.contains("route_basis schema_version"), "{reason}");
            assert!(reason.contains("route_basis.invalid"), "{reason}");
        }
        other => panic!("expected malformed-route-basis refusal, got {other:?}"),
    }
}

#[test]
fn trusted_pipeline_session_keeps_non_route_basis_unknown_top_level_keys_as_invalid_state() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    let (definition, _) =
        pipeline_proof_corpus_support::load_foundation_inputs_definition(&repo_root);
    let _ = pipeline_proof_corpus_support::persist_foundation_inputs_route_basis(&repo_root);
    let path = pipeline_proof_corpus_support::pipeline_state_path(&repo_root);
    let persisted = std::fs::read_to_string(&path).expect("read state");
    std::fs::write(&path, format!("{persisted}\nroute_basis_extra: true\n"))
        .expect("write malformed state");

    let err = load_trusted_pipeline_session(&repo_root, &definition)
        .expect_err("unknown top-level key should stay invalid state");

    match err {
        TrustedPipelineSessionRefusal::InvalidState {
            path: refusal_path,
            reason,
        } => {
            assert_eq!(refusal_path, path);
            assert!(reason.contains("unknown field"), "{reason}");
            assert!(reason.contains("route_basis_extra"), "{reason}");
        }
        other => panic!("expected invalid-state refusal, got {other:?}"),
    }
}

#[test]
fn trusted_pipeline_session_normalizes_repo_root_and_refuses_inactive_stage() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    let (definition, _) =
        pipeline_proof_corpus_support::load_foundation_inputs_definition(&repo_root);
    let _ = pipeline_proof_corpus_support::persist_foundation_inputs_route_basis(&repo_root);

    let session = load_trusted_pipeline_session(&repo_root, &definition).expect("trusted session");
    assert_eq!(
        session.route_state.run.repo_root.as_deref(),
        Some(handbook_pipeline::route_state::ROUTE_BASIS_REPO_ROOT_SENTINEL)
    );
    assert_eq!(
        session.route_basis.run.repo_root.as_deref(),
        Some(handbook_pipeline::route_state::ROUTE_BASIS_REPO_ROOT_SENTINEL)
    );

    let err = session
        .require_active_stage(pipeline_proof_corpus_support::STAGE_10_FEATURE_SPEC_ID)
        .expect_err("inactive stage should refuse");

    match err {
        TrustedPipelineSessionRefusal::InactiveStage {
            pipeline_id,
            stage_id,
            status,
            ..
        } => {
            assert_eq!(
                pipeline_id,
                pipeline_proof_corpus_support::FOUNDATION_INPUTS_PIPELINE_ID
            );
            assert_eq!(
                stage_id,
                pipeline_proof_corpus_support::STAGE_10_FEATURE_SPEC_ID
            );
            assert_eq!(status, RouteBasisStageStatus::Blocked);
        }
        other => panic!("expected inactive-stage refusal, got {other:?}"),
    }
}

#[test]
fn custom_storage_layout_round_trips_route_state_and_trusted_session_through_public_apis() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    let storage_layout = custom_storage_layout();
    let (definition, supported_variables) =
        pipeline_proof_corpus_support::load_foundation_inputs_definition(&repo_root);
    let pipeline_id = definition.header.id.clone();

    let initial = load_route_state_with_storage_layout(&repo_root, &pipeline_id, storage_layout)
        .expect("initial custom state");
    assert_eq!(initial.revision, 0);

    let outcome = set_route_state_with_storage_layout(
        &repo_root,
        &pipeline_id,
        supported_variables.iter().map(String::as_str),
        RouteStateMutation::RoutingVariable {
            variable: "needs_project_context".to_string(),
            value: true,
        },
        initial.revision,
        storage_layout,
    )
    .expect("set custom route state");
    let state = match outcome {
        RouteStateMutationOutcome::Applied(state) => *state,
        RouteStateMutationOutcome::Refused(refusal) => {
            panic!("expected custom route-state mutation to apply, got {refusal:?}")
        }
    };
    assert_eq!(state.routing.get("needs_project_context"), Some(&true));

    let custom_path = custom_state_path(&repo_root, &pipeline_id, storage_layout);
    assert!(custom_path.exists(), "custom state path should exist");
    assert!(
        !state_path(&repo_root, &pipeline_id).exists(),
        "default state path should remain unused"
    );

    let reloaded = load_route_state_with_storage_layout(&repo_root, &pipeline_id, storage_layout)
        .expect("reload custom state");
    let route = resolve_pipeline_route(
        &definition,
        &RouteVariables::new(reloaded.routing.clone()).expect("route variables"),
    )
    .expect("resolve route");
    let route_basis = build_route_basis(&repo_root, &definition, &reloaded, &route).expect("basis");

    let persisted = persist_route_basis_with_storage_layout(
        &repo_root,
        &pipeline_id,
        route_basis.clone(),
        storage_layout,
    )
    .expect("persist custom route basis");
    let persisted_state = match persisted {
        RouteBasisPersistOutcome::Applied(state) => *state,
        RouteBasisPersistOutcome::Refused(refusal) => {
            panic!("expected custom route basis persist to apply, got {refusal:?}")
        }
    };
    assert_eq!(persisted_state.route_basis.as_ref(), Some(&route_basis));

    let session =
        load_trusted_pipeline_session_with_storage_layout(&repo_root, &definition, storage_layout)
            .expect("trusted session via custom storage layout");
    assert_eq!(session.pipeline_id, pipeline_id);
    assert_eq!(session.route_basis, route_basis);
    assert_eq!(
        session.route_state.run.repo_root.as_deref(),
        Some(handbook_pipeline::route_state::ROUTE_BASIS_REPO_ROOT_SENTINEL)
    );
}

#[test]
fn load_route_state_accepts_legacy_absolute_route_basis_repo_root() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    let definition = load_pipeline_definition(&repo_root, "core/pipelines/foundation_inputs.yaml")
        .expect("pipeline fixture");
    let supported_variables = supported_route_state_variables(&definition);
    let state = load_route_state_with_supported_variables(
        &repo_root,
        &definition.header.id,
        &supported_variables,
    )
    .expect("state");
    let route = resolve_pipeline_route(
        &definition,
        &RouteVariables::new(state.routing.clone()).expect("route variables"),
    )
    .expect("route");
    let route_basis = build_route_basis(&repo_root, &definition, &state, &route).expect("basis");

    let outcome =
        persist_route_basis(&repo_root, &definition.header.id, route_basis).expect("persist");
    match outcome {
        RouteBasisPersistOutcome::Applied(_) => {}
        RouteBasisPersistOutcome::Refused(refusal) => {
            panic!("expected route basis persist to apply, got {refusal:?}")
        }
    }

    let path = state_path(&repo_root, &definition.header.id);
    let persisted = std::fs::read_to_string(&path).expect("read persisted state");
    let legacy = persisted.replacen(
        "    repo_root: ${repo_root}\n",
        &format!("    repo_root: {}\n", repo_root.display()),
        1,
    );
    assert_ne!(
        persisted, legacy,
        "route_basis repo_root should be rewritten for the test"
    );
    std::fs::write(&path, legacy).expect("write legacy route_basis repo_root");

    let loaded = load_route_state_with_supported_variables(
        &repo_root,
        &definition.header.id,
        &supported_variables,
    )
    .expect("legacy route_basis loads");
    let expected_repo_root = repo_root_string(&repo_root);
    assert_eq!(
        loaded
            .route_basis
            .as_ref()
            .and_then(|basis| basis.run.repo_root.as_deref()),
        Some(expected_repo_root.as_str())
    );
}

#[test]
fn persist_route_basis_refuses_forged_route_status() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    let definition = load_pipeline_definition(&repo_root, "core/pipelines/foundation_inputs.yaml")
        .expect("pipeline fixture");
    let supported_variables = supported_route_state_variables(&definition);
    let state = load_route_state_with_supported_variables(
        &repo_root,
        &definition.header.id,
        &supported_variables,
    )
    .expect("state");
    let route = resolve_pipeline_route(
        &definition,
        &RouteVariables::new(state.routing.clone()).expect("route variables"),
    )
    .expect("route");
    let mut route_basis =
        build_route_basis(&repo_root, &definition, &state, &route).expect("route basis");
    let forged_stage = route_basis
        .route
        .iter_mut()
        .find(|stage| stage.stage_id == "stage.10_feature_spec")
        .expect("stage.10_feature_spec");
    forged_stage.status = handbook_pipeline::route_state::RouteBasisStageStatus::Active;
    forged_stage.reason = None;

    let outcome = persist_route_basis(&repo_root, &definition.header.id, route_basis)
        .expect("persist outcome");

    match outcome {
        RouteBasisPersistOutcome::Refused(RouteBasisPersistRefusal::MalformedState { reason }) => {
            assert!(reason.contains("stage.10_feature_spec"), "{reason}");
            assert!(reason.contains("status `active`"), "{reason}");
            assert!(reason.contains("canonical `blocked`"), "{reason}");
        }
        other => panic!("expected malformed-state refusal, got {other:?}"),
    }
}

#[cfg(unix)]
#[test]
fn build_route_basis_refuses_symlinked_runner_file() {
    use std::os::unix::fs::symlink;

    let (_dir, repo_root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    let definition = load_pipeline_definition(&repo_root, "core/pipelines/foundation_inputs.yaml")
        .expect("pipeline fixture");
    let supported_variables = supported_route_state_variables(&definition);
    let state = load_route_state_with_supported_variables(
        &repo_root,
        &definition.header.id,
        &supported_variables,
    )
    .expect("state");
    let route = resolve_pipeline_route(
        &definition,
        &RouteVariables::new(state.routing.clone()).expect("route variables"),
    )
    .expect("route");

    let outside_dir = tempfile::tempdir().expect("outside tempdir");
    let outside_secret = outside_dir.path().join("system-review-secret.txt");
    write_file(&outside_secret, "outside-secret");

    let runner_file = repo_root.join("core/runners/codex-cli.md");
    std::fs::remove_file(&runner_file).expect("remove runner file");
    symlink(&outside_secret, &runner_file).expect("symlink runner");

    let err = build_route_basis(&repo_root, &definition, &state, &route).expect_err("basis error");

    match &err {
        handbook_pipeline::route_state::RouteBasisBuildError::ReadFailure { path, .. } => {
            assert_eq!(path, &runner_file);
        }
        other => panic!("expected read failure, got {other:?}"),
    }
    assert!(!err.to_string().contains("outside-secret"));
}

#[test]
fn build_route_basis_refuses_incomplete_default_profile_pack() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    seed_incomplete_profile_pack(&repo_root, "incomplete");

    let pipeline_path = repo_root.join("core/pipelines/foundation_inputs.yaml");
    let pipeline_contents = std::fs::read_to_string(&pipeline_path).expect("read pipeline");
    std::fs::write(
        &pipeline_path,
        pipeline_contents.replace("profile: python-uv", "profile: incomplete"),
    )
    .expect("write pipeline");

    let definition = load_pipeline_definition(&repo_root, "core/pipelines/foundation_inputs.yaml")
        .expect("pipeline fixture");
    let supported_variables = supported_route_state_variables(&definition);
    let state = load_route_state_with_supported_variables(
        &repo_root,
        &definition.header.id,
        &supported_variables,
    )
    .expect("state");
    let route = resolve_pipeline_route(
        &definition,
        &RouteVariables::new(state.routing.clone()).expect("route variables"),
    )
    .expect("route");

    let err = build_route_basis(&repo_root, &definition, &state, &route).expect_err("basis error");

    match &err {
        handbook_pipeline::route_state::RouteBasisBuildError::IncompleteSelectedProfilePack {
            profile_id,
            missing_files,
            ..
        } => {
            assert_eq!(profile_id, "incomplete");
            assert_eq!(missing_files, &vec!["commands.yaml", "conventions.md"]);
        }
        other => panic!("expected incomplete selected profile pack error, got {other:?}"),
    }
    assert!(err.to_string().contains("core/profiles/incomplete/"));
    assert!(!err.to_string().contains("failed to read route_basis input"));
}

#[test]
fn missing_state_loads_as_empty_and_round_trips_mixed_fields() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    let expected_repo_root = repo_root_string(repo_root);
    let pipeline_id = "pipeline.route_state";
    seed_run_inventory(repo_root);

    let empty = load_route_state(repo_root, pipeline_id).expect("empty state");
    assert_eq!(empty, RouteState::empty(pipeline_id));
    assert!(empty.routing.is_empty());
    assert_eq!(empty.refs.charter_ref, None);
    assert_eq!(empty.refs.project_context_ref, None);
    assert_eq!(empty.run.runner, None);
    assert_eq!(empty.run.profile, None);
    assert_eq!(empty.run.repo_root, None);

    let outcome = set_route_state(
        repo_root,
        pipeline_id,
        ["needs_project_context", "charter_gaps_detected"],
        RouteStateMutation::RoutingVariable {
            variable: "needs_project_context".to_string(),
            value: true,
        },
        0,
    )
    .expect("routing mutation");
    let mut state = match outcome {
        RouteStateMutationOutcome::Applied(state) => *state,
        other => panic!("expected success, got {other:?}"),
    };
    assert_eq!(state.schema_version, ROUTE_STATE_SCHEMA_VERSION);
    assert_eq!(state.pipeline_id, pipeline_id);
    assert_eq!(state.revision, 1);
    assert_eq!(state.routing.get("needs_project_context"), Some(&true));
    assert_eq!(
        state.run.repo_root.as_deref(),
        Some(expected_repo_root.as_str())
    );
    assert_eq!(state.audit.len(), 1);
    assert_eq!(state.audit[0].field_path, "routing.needs_project_context");
    assert_eq!(state.audit[0].value, RouteStateValue::Bool(true));

    let outcome = set_route_state(
        repo_root,
        pipeline_id,
        ["needs_project_context", "charter_gaps_detected"],
        RouteStateMutation::RunRunner {
            value: "codex-cli".to_string(),
        },
        1,
    )
    .expect("run mutation");
    state = match outcome {
        RouteStateMutationOutcome::Applied(state) => *state,
        other => panic!("expected success, got {other:?}"),
    };
    assert_eq!(state.revision, 2);
    assert_eq!(state.run.runner.as_deref(), Some("codex-cli"));
    assert_eq!(
        state.run.repo_root.as_deref(),
        Some(expected_repo_root.as_str())
    );
    assert_eq!(state.audit[1].field_path, "run.runner");
    assert_eq!(
        state.audit[1].value,
        RouteStateValue::String("codex-cli".to_string())
    );

    let outcome = set_route_state(
        repo_root,
        pipeline_id,
        ["needs_project_context", "charter_gaps_detected"],
        RouteStateMutation::RunProfile {
            value: "python-uv".to_string(),
        },
        2,
    )
    .expect("profile mutation");
    state = match outcome {
        RouteStateMutationOutcome::Applied(state) => *state,
        other => panic!("expected success, got {other:?}"),
    };
    assert_eq!(state.revision, 3);
    assert_eq!(state.run.profile.as_deref(), Some("python-uv"));
    assert_eq!(
        state.run.repo_root.as_deref(),
        Some(expected_repo_root.as_str())
    );

    let outcome = set_route_state(
        repo_root,
        pipeline_id,
        ["needs_project_context", "charter_gaps_detected"],
        RouteStateMutation::RefCharterRef {
            value: "artifacts/charter/CHARTER.md".to_string(),
        },
        3,
    )
    .expect("charter ref mutation");
    state = match outcome {
        RouteStateMutationOutcome::Applied(state) => *state,
        other => panic!("expected success, got {other:?}"),
    };
    assert_eq!(state.revision, 4);
    assert_eq!(
        state.refs.charter_ref.as_deref(),
        Some("artifacts/charter/CHARTER.md")
    );
    assert_eq!(
        state.run.repo_root.as_deref(),
        Some(expected_repo_root.as_str())
    );

    let outcome = set_route_state(
        repo_root,
        pipeline_id,
        ["needs_project_context", "charter_gaps_detected"],
        RouteStateMutation::RefProjectContextRef {
            value: "artifacts/project_context/PROJECT_CONTEXT.md".to_string(),
        },
        4,
    )
    .expect("project context ref mutation");
    state = match outcome {
        RouteStateMutationOutcome::Applied(state) => *state,
        other => panic!("expected success, got {other:?}"),
    };
    assert_eq!(state.revision, 5);
    assert_eq!(
        state.refs.project_context_ref.as_deref(),
        Some("artifacts/project_context/PROJECT_CONTEXT.md")
    );
    assert_eq!(
        state.run.repo_root.as_deref(),
        Some(expected_repo_root.as_str())
    );

    let loaded = load_route_state(repo_root, pipeline_id).expect("loaded state");
    assert_eq!(loaded, state);
}

#[test]
fn legacy_m1_state_without_route_basis_still_loads() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    let pipeline_id = "pipeline.route_state";
    let path = state_path(repo_root, pipeline_id);
    seed_run_inventory(repo_root);
    write_file(
        &path,
        r#"---
schema_version: m1-pipeline-state-v2
pipeline_id: pipeline.route_state
revision: 2
routing:
  needs_project_context: false
refs:
  charter_ref: artifacts/charter/CHARTER.md
run:
  runner: codex-cli
  profile: python-uv
audit:
  - revision: 1
    field_path: run.runner
    value: codex-cli
  - revision: 2
    field_path: routing.needs_project_context
    value: false
"#,
    );

    let state = load_route_state(repo_root, pipeline_id).expect("legacy state loads");
    assert_eq!(state.schema_version, "m1-pipeline-state-v2");
    assert_eq!(state.route_basis, None);
    assert_eq!(state.run.runner.as_deref(), Some("codex-cli"));
    assert_eq!(state.run.profile.as_deref(), Some("python-uv"));
    assert_eq!(state.routing.get("needs_project_context"), Some(&false));
}

#[test]
fn legacy_state_without_repo_root_backfills_on_next_successful_mutation() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    let expected_repo_root = repo_root_string(repo_root);
    let pipeline_id = "pipeline.route_state";
    let path = state_path(repo_root, pipeline_id);
    seed_run_inventory(repo_root);
    write_file(
        &path,
        r#"---
schema_version: m1-pipeline-state-v2
pipeline_id: pipeline.route_state
revision: 1
routing: {}
refs: {}
run:
  runner: codex-cli
  profile: python-uv
audit:
  - revision: 1
    field_path: run.runner
    value: codex-cli
"#,
    );

    let loaded = load_route_state(repo_root, pipeline_id).expect("legacy state loads");
    assert_eq!(loaded.run.repo_root, None);

    let outcome = set_route_state(
        repo_root,
        pipeline_id,
        ["needs_project_context"],
        RouteStateMutation::RoutingVariable {
            variable: "needs_project_context".to_string(),
            value: true,
        },
        1,
    )
    .expect("mutation");
    let state = match outcome {
        RouteStateMutationOutcome::Applied(state) => *state,
        other => panic!("expected success, got {other:?}"),
    };
    assert_eq!(state.revision, 2);
    assert_eq!(state.run.runner.as_deref(), Some("codex-cli"));
    assert_eq!(state.run.profile.as_deref(), Some("python-uv"));
    assert_eq!(
        state.run.repo_root.as_deref(),
        Some(expected_repo_root.as_str())
    );

    let reloaded = load_route_state(repo_root, pipeline_id).expect("reloaded state");
    assert_eq!(reloaded, state);
}

#[test]
fn malformed_state_refuses_unknown_top_level_keys() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    let pipeline_id = "pipeline.route_state";
    let path = state_path(repo_root, pipeline_id);
    write_file(
        &path,
        r#"---
schema_version: m1-pipeline-state-v2
pipeline_id: pipeline.route_state
revision: 1
routing: {}
refs: {}
run: {}
audit: []
unexpected: true
"#,
    );

    let err = load_route_state(repo_root, pipeline_id).expect_err("malformed state");
    match err {
        RouteStateReadError::MalformedState { path: err_path, .. } => {
            assert_eq!(err_path, path);
        }
        other => panic!("expected malformed-state refusal, got {other:?}"),
    }
}

#[test]
fn malformed_state_refuses_wrong_nested_scalar_types() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    let pipeline_id = "pipeline.route_state";
    let path = state_path(repo_root, pipeline_id);
    write_file(
        &path,
        r#"---
schema_version: m1-pipeline-state-v2
pipeline_id: pipeline.route_state
revision: 1
routing:
  needs_project_context: true
refs:
  charter_ref: true
run: {}
audit:
  - revision: 1
    field_path: run.runner
    value: true
"#,
    );

    let err = load_route_state(repo_root, pipeline_id).expect_err("malformed state");
    match err {
        RouteStateReadError::MalformedState { path: err_path, .. } => {
            assert_eq!(err_path, path);
        }
        other => panic!("expected malformed-state refusal, got {other:?}"),
    }
}

#[test]
fn malformed_state_refuses_invalid_repo_root_shape() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    let pipeline_id = "pipeline.route_state";
    let path = state_path(repo_root, pipeline_id);
    seed_run_inventory(repo_root);
    write_file(
        &path,
        r#"---
schema_version: m1-pipeline-state-v2
pipeline_id: pipeline.route_state
revision: 1
routing: {}
refs: {}
run:
  repo_root: relative/root
audit: []
"#,
    );

    let err = load_route_state(repo_root, pipeline_id).expect_err("malformed state");
    match err {
        RouteStateReadError::MalformedState { reason, .. } => {
            assert!(reason.contains("run.repo_root"), "{reason}");
            assert!(reason.contains("must be absolute"), "{reason}");
        }
        other => panic!("expected malformed-state refusal, got {other:?}"),
    }
}

#[test]
fn malformed_route_basis_is_distinct_from_malformed_route_state() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    let malformed_route_basis_path =
        pipeline_proof_corpus_support::install_state_seed(&repo_root, "malformed_route_basis.yaml");
    let malformed_route_basis_err = load_route_state_with_supported_variables(
        &repo_root,
        pipeline_proof_corpus_support::FOUNDATION_INPUTS_PIPELINE_ID,
        &std::collections::BTreeSet::from([
            "needs_project_context".to_string(),
            "charter_gaps_detected".to_string(),
        ]),
    )
    .expect_err("malformed route basis should refuse");

    match malformed_route_basis_err {
        RouteStateReadError::MalformedState { path, reason } => {
            assert_eq!(path, malformed_route_basis_path);
            assert!(reason.contains("route_basis"), "{reason}");
        }
        other => panic!("expected malformed-state refusal, got {other:?}"),
    }

    let malformed_route_state_path =
        pipeline_proof_corpus_support::install_state_seed(&repo_root, "malformed_route_state.yaml");
    let malformed_route_state_err = load_route_state_with_supported_variables(
        &repo_root,
        pipeline_proof_corpus_support::FOUNDATION_INPUTS_PIPELINE_ID,
        &std::collections::BTreeSet::from([
            "needs_project_context".to_string(),
            "charter_gaps_detected".to_string(),
        ]),
    )
    .expect_err("malformed route state should refuse");

    match malformed_route_state_err {
        RouteStateReadError::MalformedState { path, reason } => {
            assert_eq!(path, malformed_route_state_path);
            assert!(!reason.contains("route_basis"), "{reason}");
        }
        other => panic!("expected malformed-state refusal, got {other:?}"),
    }
}

#[test]
fn persisted_route_basis_refuses_incomplete_profile_pack_on_reload() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    let definition = load_pipeline_definition(&repo_root, "core/pipelines/foundation_inputs.yaml")
        .expect("pipeline fixture");
    let supported_variables = supported_route_state_variables(&definition);
    let state = load_route_state_with_supported_variables(
        &repo_root,
        &definition.header.id,
        &supported_variables,
    )
    .expect("state");
    let route = resolve_pipeline_route(
        &definition,
        &RouteVariables::new(state.routing.clone()).expect("route variables"),
    )
    .expect("route");
    let route_basis = build_route_basis(&repo_root, &definition, &state, &route).expect("basis");

    let outcome =
        persist_route_basis(&repo_root, &definition.header.id, route_basis).expect("persist");
    match outcome {
        RouteBasisPersistOutcome::Applied(_) => {}
        other => panic!("expected route basis persist to apply, got {other:?}"),
    }

    std::fs::remove_file(repo_root.join("core/profiles/python-uv/commands.yaml"))
        .expect("remove commands.yaml");

    let err = load_route_state_with_supported_variables(
        &repo_root,
        &definition.header.id,
        &supported_variables,
    )
    .expect_err("reloading state should refuse");

    match err {
        RouteStateReadError::MalformedState { reason, .. } => {
            assert!(reason.contains("route_basis"), "{reason}");
            assert!(reason.contains("core/profiles/python-uv/"), "{reason}");
            assert!(reason.contains("commands.yaml"), "{reason}");
        }
        other => panic!("expected malformed-state refusal, got {other:?}"),
    }
}

#[test]
fn stage_05_capture_ready_fixture_persists_route_basis_without_state_mutation() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_stage_05_capture_ready_repo();
    let definition = load_pipeline_definition(&repo_root, "core/pipelines/foundation_inputs.yaml")
        .expect("pipeline fixture");
    let supported_variables = supported_route_state_variables(&definition);
    let state = load_route_state_with_supported_variables(
        &repo_root,
        pipeline_proof_corpus_support::FOUNDATION_INPUTS_PIPELINE_ID,
        &supported_variables,
    )
    .expect("state");

    assert_eq!(state.revision, 0);
    assert!(state.route_basis.is_some());
    assert!(state.routing.is_empty());
    assert_eq!(state.refs.charter_ref, None);
    assert_eq!(state.refs.project_context_ref, None);
}

#[test]
fn stage_07_capture_ready_fixture_persists_only_post_charter_route_state() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_stage_07_capture_ready_repo();
    let definition = load_pipeline_definition(&repo_root, "core/pipelines/foundation_inputs.yaml")
        .expect("pipeline fixture");
    let supported_variables = supported_route_state_variables(&definition);
    let state = load_route_state_with_supported_variables(
        &repo_root,
        pipeline_proof_corpus_support::FOUNDATION_INPUTS_PIPELINE_ID,
        &supported_variables,
    )
    .expect("state");

    assert_eq!(state.revision, 3);
    assert!(state.route_basis.is_some());
    assert_eq!(
        state.refs.charter_ref.as_deref(),
        Some("artifacts/charter/CHARTER.md")
    );
    assert_eq!(state.refs.project_context_ref, None);
    assert_eq!(state.routing.get("needs_project_context"), Some(&false));
    assert_eq!(state.routing.get("charter_gaps_detected"), Some(&false));
}

#[test]
fn load_with_supported_variables_refuses_unsupported_persisted_routing_variables() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    let pipeline_id = "pipeline.route_state";
    let path = state_path(repo_root, pipeline_id);
    write_file(
        &path,
        r#"---
schema_version: m1-pipeline-state-v2
pipeline_id: pipeline.route_state
revision: 1
routing:
  charter_gaps_detected: true
refs: {}
run: {}
audit:
  - revision: 1
    field_path: routing.charter_gaps_detected
    value: true
"#,
    );

    let err = load_route_state_with_supported_variables(
        repo_root,
        pipeline_id,
        &std::collections::BTreeSet::from(["needs_project_context".to_string()]),
    )
    .expect_err("malformed state");

    match err {
        RouteStateReadError::MalformedState { path: err_path, .. } => {
            assert_eq!(err_path, path);
        }
        other => panic!("expected malformed-state refusal, got {other:?}"),
    }
}

#[test]
fn unsupported_variable_refuses_without_overwrite() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    let pipeline_id = "pipeline.route_state";
    let path = state_path(repo_root, pipeline_id);

    let outcome = set_route_state(
        repo_root,
        pipeline_id,
        ["needs_project_context"],
        RouteStateMutation::RoutingVariable {
            variable: "charter_gaps_detected".to_string(),
            value: true,
        },
        0,
    )
    .expect("mutation");

    match outcome {
        RouteStateMutationOutcome::Refused(RouteStateMutationRefusal::UnsupportedVariable {
            variable,
        }) => assert_eq!(variable, "charter_gaps_detected"),
        other => panic!("expected unsupported-variable refusal, got {other:?}"),
    }
    assert!(!path.exists());
}

#[test]
fn invalid_ref_value_is_rejected_before_write() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    let pipeline_id = "pipeline.route_state";

    let err = set_route_state(
        repo_root,
        pipeline_id,
        ["needs_project_context"],
        RouteStateMutation::RefCharterRef {
            value: "/tmp/CHARTER.md".to_string(),
        },
        0,
    )
    .expect_err("invalid mutation");

    match err {
        RouteStateStoreError::InvalidMutation { reason } => {
            assert!(reason.contains("must not be absolute"), "{reason}");
        }
        other => panic!("expected invalid mutation error, got {other:?}"),
    }
}

#[test]
fn invalid_runner_value_is_rejected_before_write() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    let pipeline_id = "pipeline.route_state";
    seed_run_inventory(repo_root);

    let err = set_route_state(
        repo_root,
        pipeline_id,
        ["needs_project_context"],
        RouteStateMutation::RunRunner {
            value: "not-a-runner".to_string(),
        },
        0,
    )
    .expect_err("invalid mutation");

    match err {
        RouteStateStoreError::InvalidMutation { reason } => {
            assert!(reason.contains("run.runner"), "{reason}");
            assert!(reason.contains("core/runners/"), "{reason}");
            assert!(reason.contains("not-a-runner"), "{reason}");
        }
        other => panic!("expected invalid mutation error, got {other:?}"),
    }
}

#[test]
fn invalid_profile_value_is_rejected_before_write() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    let pipeline_id = "pipeline.route_state";
    seed_run_inventory(repo_root);

    let err = set_route_state(
        repo_root,
        pipeline_id,
        ["needs_project_context"],
        RouteStateMutation::RunProfile {
            value: "not-a-profile".to_string(),
        },
        0,
    )
    .expect_err("invalid mutation");

    match err {
        RouteStateStoreError::InvalidMutation { reason } => {
            assert!(reason.contains("run.profile"), "{reason}");
            assert!(reason.contains("core/profiles/"), "{reason}");
            assert!(reason.contains("not-a-profile"), "{reason}");
        }
        other => panic!("expected invalid mutation error, got {other:?}"),
    }
}

#[test]
fn incomplete_profile_pack_is_rejected_before_write() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    let pipeline_id = "pipeline.route_state";
    seed_run_inventory(repo_root);
    seed_incomplete_profile_pack(repo_root, "incomplete");

    let err = set_route_state(
        repo_root,
        pipeline_id,
        ["needs_project_context"],
        RouteStateMutation::RunProfile {
            value: "incomplete".to_string(),
        },
        0,
    )
    .expect_err("invalid mutation");

    match err {
        RouteStateStoreError::InvalidMutation { reason } => {
            assert!(reason.contains("run.profile"), "{reason}");
            assert!(reason.contains("core/profiles/incomplete/"), "{reason}");
            assert!(reason.contains("commands.yaml"), "{reason}");
            assert!(reason.contains("conventions.md"), "{reason}");
        }
        other => panic!("expected invalid mutation error, got {other:?}"),
    }
}

#[test]
fn revision_conflict_refuses_without_overwrite() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    let pipeline_id = "pipeline.route_state";
    let path = state_path(repo_root, pipeline_id);
    seed_run_inventory(repo_root);
    write_file(
        &path,
        r#"---
schema_version: m1-pipeline-state-v2
pipeline_id: pipeline.route_state
revision: 2
routing:
  needs_project_context: true
refs: {}
run:
  runner: codex-cli
audit:
  - revision: 1
    field_path: routing.needs_project_context
    value: true
  - revision: 2
    field_path: run.runner
    value: codex-cli
"#,
    );

    let outcome = set_route_state(
        repo_root,
        pipeline_id,
        ["needs_project_context"],
        RouteStateMutation::RoutingVariable {
            variable: "needs_project_context".to_string(),
            value: false,
        },
        1,
    )
    .expect("mutation");

    match outcome {
        RouteStateMutationOutcome::Refused(RouteStateMutationRefusal::RevisionConflict {
            expected_revision,
            actual_revision,
        }) => {
            assert_eq!(expected_revision, 1);
            assert_eq!(actual_revision, 2);
        }
        other => panic!("expected revision-conflict refusal, got {other:?}"),
    }

    let loaded = load_route_state(repo_root, pipeline_id).expect("loaded state");
    assert_eq!(loaded.revision, 2);
    assert_eq!(loaded.routing.get("needs_project_context"), Some(&true));
    assert_eq!(loaded.run.runner.as_deref(), Some("codex-cli"));
}

#[test]
fn audit_history_trims_oldest_first_across_mixed_fields() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    let pipeline_id = "pipeline.route_state";
    let mut expected_revision = 0;
    seed_run_inventory(repo_root);

    for index in 0..(ROUTE_STATE_AUDIT_LIMIT + 3) {
        let mutation = match index % 4 {
            0 => RouteStateMutation::RoutingVariable {
                variable: "needs_project_context".to_string(),
                value: index % 2 == 0,
            },
            1 => RouteStateMutation::RunRunner {
                value: "codex-cli".to_string(),
            },
            2 => RouteStateMutation::RunProfile {
                value: "python-uv".to_string(),
            },
            _ => RouteStateMutation::RefCharterRef {
                value: format!("artifacts/charter/CHARTER-{index}.md"),
            },
        };

        let outcome = set_route_state(
            repo_root,
            pipeline_id,
            ["needs_project_context"],
            mutation,
            expected_revision,
        )
        .expect("mutation");

        let state = match outcome {
            RouteStateMutationOutcome::Applied(state) => *state,
            other => panic!("expected success, got {other:?}"),
        };
        expected_revision = state.revision;
    }

    let loaded = load_route_state(repo_root, pipeline_id).expect("loaded state");
    assert_eq!(loaded.audit.len(), ROUTE_STATE_AUDIT_LIMIT);
    assert_eq!(loaded.revision, (ROUTE_STATE_AUDIT_LIMIT + 3) as u64);
    assert_eq!(
        loaded.audit.first().expect("first audit").revision,
        loaded.revision - ROUTE_STATE_AUDIT_LIMIT as u64 + 1
    );
    assert_eq!(
        loaded.audit.last().expect("last audit").revision,
        loaded.revision
    );
}

#[test]
fn malformed_state_refuses_unallowlisted_run_runner() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    let pipeline_id = "pipeline.route_state";
    let path = state_path(repo_root, pipeline_id);
    seed_run_inventory(repo_root);
    write_file(
        &path,
        r#"---
schema_version: m1-pipeline-state-v2
pipeline_id: pipeline.route_state
revision: 1
routing: {}
refs: {}
run:
  runner: not-a-runner
audit:
  - revision: 1
    field_path: run.runner
    value: not-a-runner
"#,
    );

    let err = load_route_state(repo_root, pipeline_id).expect_err("malformed state");
    match err {
        RouteStateReadError::MalformedState { reason, .. } => {
            assert!(reason.contains("run.runner"), "{reason}");
            assert!(reason.contains("not-a-runner"), "{reason}");
            assert!(reason.contains("core/runners/"), "{reason}");
        }
        other => panic!("expected malformed-state refusal, got {other:?}"),
    }
}

#[test]
fn malformed_state_refuses_incomplete_run_profile_pack() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    let pipeline_id = "pipeline.route_state";
    let path = state_path(repo_root, pipeline_id);
    seed_run_inventory(repo_root);
    seed_incomplete_profile_pack(repo_root, "incomplete");
    write_file(
        &path,
        r#"---
schema_version: m1-pipeline-state-v2
pipeline_id: pipeline.route_state
revision: 1
routing: {}
refs: {}
run:
  profile: incomplete
audit:
  - revision: 1
    field_path: run.profile
    value: incomplete
"#,
    );

    let err = load_route_state(repo_root, pipeline_id).expect_err("malformed state");
    match err {
        RouteStateReadError::MalformedState { reason, .. } => {
            assert!(reason.contains("run.profile"), "{reason}");
            assert!(reason.contains("core/profiles/incomplete/"), "{reason}");
            assert!(reason.contains("commands.yaml"), "{reason}");
            assert!(reason.contains("conventions.md"), "{reason}");
        }
        other => panic!("expected malformed-state refusal, got {other:?}"),
    }
}

#[test]
fn malformed_state_refuses_unallowlisted_run_profile_audit_entry() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    let pipeline_id = "pipeline.route_state";
    let path = state_path(repo_root, pipeline_id);
    seed_run_inventory(repo_root);
    write_file(
        &path,
        r#"---
schema_version: m1-pipeline-state-v2
pipeline_id: pipeline.route_state
revision: 2
routing: {}
refs: {}
run:
  profile: python-uv
audit:
  - revision: 1
    field_path: run.profile
    value: not-a-profile
  - revision: 2
    field_path: run.profile
    value: python-uv
"#,
    );

    let err = load_route_state(repo_root, pipeline_id).expect_err("malformed state");
    match err {
        RouteStateReadError::MalformedState { reason, .. } => {
            assert!(reason.contains("run.profile"), "{reason}");
            assert!(reason.contains("not-a-profile"), "{reason}");
            assert!(reason.contains("core/profiles/"), "{reason}");
        }
        other => panic!("expected malformed-state refusal, got {other:?}"),
    }
}

#[test]
fn atomic_replace_happens_under_lock() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    let pipeline_id = "pipeline.route_state";
    let path = state_path(repo_root, pipeline_id);
    let lock = lock_path(&path);
    let guard = acquire_unix_lock(&lock);
    let (tx, rx) = mpsc::channel();

    let repo_root = repo_root.to_path_buf();
    let thread_repo_root = repo_root.clone();
    thread::spawn(move || {
        let outcome = set_route_state(
            &thread_repo_root,
            pipeline_id,
            ["needs_project_context"],
            RouteStateMutation::RoutingVariable {
                variable: "needs_project_context".to_string(),
                value: true,
            },
            0,
        );
        tx.send(outcome).expect("send");
    });

    thread::sleep(Duration::from_millis(100));
    assert!(rx.try_recv().is_err(), "mutation should block on lock");

    release_unix_lock(&guard);
    let outcome = rx
        .recv_timeout(Duration::from_secs(2))
        .expect("mutation result")
        .expect("mutation");

    match outcome {
        RouteStateMutationOutcome::Applied(state) => {
            assert_eq!(state.revision, 1);
            assert_eq!(state.routing.get("needs_project_context"), Some(&true));
        }
        other => panic!("expected success, got {other:?}"),
    }

    let loaded = load_route_state(&repo_root, pipeline_id).expect("loaded state");
    assert_eq!(loaded.revision, 1);
    assert!(path.exists());

    let dir_entries = std::fs::read_dir(path.parent().expect("parent"))
        .expect("read dir")
        .map(|entry| {
            entry
                .expect("entry")
                .file_name()
                .to_string_lossy()
                .to_string()
        })
        .collect::<Vec<_>>();
    assert!(dir_entries
        .iter()
        .any(|entry| entry == "pipeline.route_state.yaml"));
    assert!(dir_entries
        .iter()
        .any(|entry| entry == "pipeline.route_state.lock"));
    assert!(
        dir_entries.iter().all(|entry| !entry.contains(".tmp-")),
        "temp file should not remain after atomic replace"
    );
}

#[test]
fn legacy_flat_schema_refuses_explicitly() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    let pipeline_id = "pipeline.route_state";
    let path = state_path(repo_root, pipeline_id);
    write_file(
        &path,
        r#"---
schema_version: m1-pipeline-state-v1
pipeline_id: pipeline.route_state
revision: 1
variables:
  needs_project_context: true
audit: []
"#,
    );

    let err = load_route_state(repo_root, pipeline_id).expect_err("legacy schema should refuse");
    match err {
        RouteStateReadError::MalformedState { reason, .. } => {
            assert!(
                reason.contains("unknown field `variables`")
                    || reason.contains("unexpected schema_version"),
                "{reason}"
            );
        }
        other => panic!("expected malformed-state refusal, got {other:?}"),
    }
}
