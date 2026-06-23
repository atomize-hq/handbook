#[path = "support/pipeline_proof_corpus_support.rs"]
mod pipeline_proof_corpus_support;

use handbook_pipeline::{
    pipeline::{
        load_pipeline_definition, supported_route_state_variables, ActivationClause,
        ActivationConditionSet, ActivationOperator, PipelineBody, PipelineDefaults,
        PipelineDefinition, PipelineHeader, PipelineStage, StageActivation,
    },
    pipeline_route::{
        resolve_pipeline_route, RouteEvaluationError, RouteStageReason, RouteStageStatus,
        RouteVariables,
    },
    route_state::{
        load_route_state_with_storage_layout, load_route_state_with_supported_variables,
        set_route_state, set_route_state_with_storage_layout, RouteStateMutation,
        RouteStateMutationOutcome,
    },
    PipelineStorageLayoutContract,
};
use std::path::PathBuf;

fn fixture(path: &str) -> PipelineDefinition {
    let root = pipeline_proof_corpus_support::committed_repo_root();
    load_pipeline_definition(&root, path).expect("pipeline fixture")
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
    repo_root: &std::path::Path,
    pipeline_id: &str,
    storage_layout: PipelineStorageLayoutContract,
) -> PathBuf {
    repo_root
        .join(storage_layout.pipeline_dir_relative())
        .join(format!("{pipeline_id}.yaml"))
}

#[test]
fn route_evaluation_preserves_declared_order_and_marks_false_activation_skipped() {
    let definition = fixture("core/pipelines/foundation_inputs.yaml");
    let variables = RouteVariables::from_pairs([
        ("needs_project_context", false),
        ("charter_gaps_detected", false),
    ])
    .expect("route variables");

    let result = resolve_pipeline_route(&definition, &variables).expect("route");

    assert_eq!(result.pipeline_id, definition.header.id);
    assert_eq!(result.stages.len(), definition.declared_stages().len());

    assert_eq!(
        result
            .stages
            .iter()
            .map(|stage| stage.stage_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "stage.00_base",
            "stage.04_charter_inputs",
            "stage.05_charter_synthesize",
            "stage.06_project_context_interview",
            "stage.07_foundation_pack",
            "stage.10_feature_spec",
        ]
    );
    assert_eq!(
        result
            .stages
            .iter()
            .map(|stage| stage.status)
            .collect::<Vec<_>>(),
        vec![
            RouteStageStatus::Active,
            RouteStageStatus::Active,
            RouteStageStatus::Active,
            RouteStageStatus::Skipped,
            RouteStageStatus::Active,
            RouteStageStatus::Active,
        ]
    );

    match &result.stages[3].reason {
        Some(RouteStageReason::SkippedActivationFalse {
            operator,
            unsatisfied_variables,
        }) => {
            assert_eq!(*operator, ActivationOperator::Any);
            assert_eq!(
                unsatisfied_variables,
                &vec![
                    "charter_gaps_detected".to_string(),
                    "needs_project_context".to_string(),
                ]
            );
        }
        other => panic!("expected skipped reason, got {other:?}"),
    }
}

#[test]
fn route_evaluation_emits_exactly_one_next_when_route_state_is_missing() {
    let definition = fixture("core/pipelines/foundation_inputs.yaml");
    let variables = RouteVariables::empty();

    let result = resolve_pipeline_route(&definition, &variables).expect("route");

    assert_eq!(result.pipeline_id, definition.header.id);

    assert_eq!(
        result
            .stages
            .iter()
            .map(|stage| stage.status)
            .collect::<Vec<_>>(),
        vec![
            RouteStageStatus::Active,
            RouteStageStatus::Active,
            RouteStageStatus::Active,
            RouteStageStatus::Next,
            RouteStageStatus::Blocked,
            RouteStageStatus::Blocked,
        ]
    );
    assert_eq!(
        result
            .stages
            .iter()
            .filter(|stage| stage.status == RouteStageStatus::Next)
            .count(),
        1
    );

    match &result.stages[3].reason {
        Some(RouteStageReason::NextMissingRouteVariables {
            operator,
            missing_variables,
        }) => {
            assert_eq!(*operator, ActivationOperator::Any);
            assert_eq!(
                missing_variables,
                &vec![
                    "charter_gaps_detected".to_string(),
                    "needs_project_context".to_string(),
                ]
            );
        }
        other => panic!("expected next reason, got {other:?}"),
    }

    match &result.stages[4].reason {
        Some(RouteStageReason::BlockedByUnresolvedStage {
            upstream_stage_id,
            upstream_status,
        }) => {
            assert_eq!(upstream_stage_id, "stage.06_project_context_interview");
            assert_eq!(*upstream_status, RouteStageStatus::Next);
        }
        other => panic!("expected blocked reason, got {other:?}"),
    }
}

#[test]
fn route_evaluation_refuses_out_of_contract_activation_inputs() {
    let definition = PipelineDefinition {
        source_path: PathBuf::from("core/pipelines/manual.yaml"),
        header: PipelineHeader {
            kind: "pipeline".to_string(),
            id: "pipeline.manual".to_string(),
            version: "0.1.0".to_string(),
            title: "Manual".to_string(),
            description: "manual".to_string(),
        },
        body: PipelineBody {
            defaults: PipelineDefaults {
                runner: "codex-cli".to_string(),
                profile: "python-uv".to_string(),
                enable_complexity: false,
            },
            stages: vec![PipelineStage {
                id: "stage.00_base".to_string(),
                file: "core/stages/00_base.md".to_string(),
                sets: None,
                activation: Some(StageActivation {
                    when: ActivationConditionSet {
                        operator: ActivationOperator::Any,
                        clauses: vec![ActivationClause {
                            variable: "9bad".to_string(),
                            value: true,
                        }],
                    },
                }),
            }],
        },
    };

    let variables = RouteVariables::empty();
    let err = resolve_pipeline_route(&definition, &variables).expect_err("route refusal");

    match err {
        RouteEvaluationError::InvalidActivationClause {
            stage_id, variable, ..
        } => {
            assert_eq!(stage_id, "stage.00_base");
            assert_eq!(variable, "9bad");
        }
        other => panic!("expected invalid-activation-clause refusal, got {other:?}"),
    }
}

#[test]
fn supported_route_state_variables_include_stage_sets_and_activation_clauses() {
    let definition = fixture("core/pipelines/foundation_inputs.yaml");
    let variables = supported_route_state_variables(&definition);

    assert_eq!(
        variables.into_iter().collect::<Vec<_>>(),
        vec![
            "charter_gaps_detected".to_string(),
            "needs_project_context".to_string(),
        ]
    );
}

#[test]
fn all_operator_with_known_false_and_missing_inputs_is_skipped_not_next() {
    let definition = PipelineDefinition {
        source_path: PathBuf::from("core/pipelines/manual-all.yaml"),
        header: PipelineHeader {
            kind: "pipeline".to_string(),
            id: "pipeline.manual_all".to_string(),
            version: "0.1.0".to_string(),
            title: "Manual All".to_string(),
            description: "manual all".to_string(),
        },
        body: PipelineBody {
            defaults: PipelineDefaults {
                runner: "codex-cli".to_string(),
                profile: "python-uv".to_string(),
                enable_complexity: false,
            },
            stages: vec![PipelineStage {
                id: "stage.00_base".to_string(),
                file: "core/stages/00_base.md".to_string(),
                sets: None,
                activation: Some(StageActivation {
                    when: ActivationConditionSet {
                        operator: ActivationOperator::All,
                        clauses: vec![
                            ActivationClause {
                                variable: "needs_project_context".to_string(),
                                value: true,
                            },
                            ActivationClause {
                                variable: "charter_gaps_detected".to_string(),
                                value: true,
                            },
                        ],
                    },
                }),
            }],
        },
    };

    let variables =
        RouteVariables::from_pairs([("needs_project_context", false)]).expect("route variables");
    let result = resolve_pipeline_route(&definition, &variables).expect("route");

    assert_eq!(result.stages[0].status, RouteStageStatus::Skipped);
    match &result.stages[0].reason {
        Some(RouteStageReason::SkippedActivationFalse {
            operator,
            unsatisfied_variables,
        }) => {
            assert_eq!(*operator, ActivationOperator::All);
            assert_eq!(
                unsatisfied_variables,
                &vec!["needs_project_context".to_string()]
            );
        }
        other => panic!("expected skipped reason, got {other:?}"),
    }
}

#[test]
fn shared_proof_corpus_route_outputs_match_repo_owned_goldens() {
    let (_dir, root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    let definition = load_pipeline_definition(&root, "core/pipelines/foundation_inputs.yaml")
        .expect("proof corpus definition");
    let pipeline_id = definition.header.id.clone();
    let supported_variables = supported_route_state_variables(&definition);

    let initial_state =
        load_route_state_with_supported_variables(&root, &pipeline_id, &supported_variables)
            .expect("initial state");
    let initial_route = resolve_pipeline_route(
        &definition,
        &RouteVariables::new(initial_state.routing.clone()).expect("route variables"),
    )
    .expect("initial route");
    pipeline_proof_corpus_support::assert_matches_golden(
        &pipeline_proof_corpus_support::render_pipeline_resolve_output(
            &pipeline_id,
            &initial_state,
            &handbook_pipeline::route_state::effective_route_basis_run(
                &root,
                &definition,
                &initial_state,
            ),
            &initial_route,
        ),
        &root,
        None,
        "resolve.initial.txt",
    );

    let first_outcome = set_route_state(
        &root,
        &pipeline_id,
        supported_variables.clone(),
        RouteStateMutation::RoutingVariable {
            variable: "needs_project_context".to_string(),
            value: true,
        },
        initial_state.revision,
    )
    .expect("first mutation");
    let first_state = match &first_outcome {
        RouteStateMutationOutcome::Applied(state) => state.as_ref().clone(),
        RouteStateMutationOutcome::Refused(refusal) => {
            panic!("expected applied mutation, got {refusal}")
        }
    };
    pipeline_proof_corpus_support::assert_matches_golden(
        &pipeline_proof_corpus_support::render_pipeline_state_set_output(
            &pipeline_id,
            first_outcome,
        ),
        &root,
        None,
        "state_set.var.needs_project_context.applied.txt",
    );

    let second_outcome = set_route_state(
        &root,
        &pipeline_id,
        supported_variables.clone(),
        RouteStateMutation::RoutingVariable {
            variable: "charter_gaps_detected".to_string(),
            value: true,
        },
        first_state.revision,
    )
    .expect("second mutation");
    pipeline_proof_corpus_support::assert_matches_golden(
        &pipeline_proof_corpus_support::render_pipeline_state_set_output(
            &pipeline_id,
            second_outcome,
        ),
        &root,
        None,
        "state_set.var.charter_gaps_detected.applied.txt",
    );

    let activated_state =
        load_route_state_with_supported_variables(&root, &pipeline_id, &supported_variables)
            .expect("activated state");
    let activated_route = resolve_pipeline_route(
        &definition,
        &RouteVariables::new(activated_state.routing.clone()).expect("route variables"),
    )
    .expect("activated route");
    pipeline_proof_corpus_support::assert_matches_golden(
        &pipeline_proof_corpus_support::render_pipeline_resolve_output(
            &pipeline_id,
            &activated_state,
            &handbook_pipeline::route_state::effective_route_basis_run(
                &root,
                &definition,
                &activated_state,
            ),
            &activated_route,
        ),
        &root,
        None,
        "resolve.after_full_activation.txt",
    );
}

#[test]
fn shared_proof_corpus_state_mutation_outputs_match_repo_owned_goldens() {
    let (_dir, root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    let definition = load_pipeline_definition(&root, "core/pipelines/foundation_inputs.yaml")
        .expect("proof corpus definition");
    let pipeline_id = definition.header.id.clone();
    let supported_variables = supported_route_state_variables(&definition);

    let runner_outcome = set_route_state(
        &root,
        &pipeline_id,
        supported_variables.clone(),
        RouteStateMutation::RunRunner {
            value: "codex-cli".to_string(),
        },
        0,
    )
    .expect("runner mutation");
    let runner_state = match &runner_outcome {
        RouteStateMutationOutcome::Applied(state) => state.as_ref().clone(),
        RouteStateMutationOutcome::Refused(refusal) => {
            panic!("expected applied mutation, got {refusal}")
        }
    };
    pipeline_proof_corpus_support::assert_matches_golden(
        &pipeline_proof_corpus_support::render_pipeline_state_set_output(
            &pipeline_id,
            runner_outcome,
        ),
        &root,
        None,
        "state_set.field.run_runner.applied.txt",
    );

    let ref_outcome = set_route_state(
        &root,
        &pipeline_id,
        supported_variables.clone(),
        RouteStateMutation::RefCharterRef {
            value: "artifacts/charter/CHARTER.md".to_string(),
        },
        runner_state.revision,
    )
    .expect("ref mutation");
    pipeline_proof_corpus_support::assert_matches_golden(
        &pipeline_proof_corpus_support::render_pipeline_state_set_output(&pipeline_id, ref_outcome),
        &root,
        None,
        "state_set.field.refs_charter_ref.applied.txt",
    );

    let routed_state =
        load_route_state_with_supported_variables(&root, &pipeline_id, &supported_variables)
            .expect("routed state");
    let route = resolve_pipeline_route(
        &definition,
        &RouteVariables::new(routed_state.routing.clone()).expect("route variables"),
    )
    .expect("route");
    pipeline_proof_corpus_support::assert_matches_golden(
        &pipeline_proof_corpus_support::render_pipeline_resolve_output(
            &pipeline_id,
            &routed_state,
            &handbook_pipeline::route_state::effective_route_basis_run(
                &root,
                &definition,
                &routed_state,
            ),
            &route,
        ),
        &root,
        None,
        "resolve.after_run_and_refs.txt",
    );

    let unsupported = set_route_state(
        &root,
        &pipeline_id,
        supported_variables.clone(),
        RouteStateMutation::RoutingVariable {
            variable: "unsupported_flag".to_string(),
            value: true,
        },
        routed_state.revision,
    )
    .expect("unsupported mutation");
    pipeline_proof_corpus_support::assert_matches_golden(
        &pipeline_proof_corpus_support::render_pipeline_state_set_output(&pipeline_id, unsupported),
        &root,
        None,
        "state_set.refused.unsupported_variable.txt",
    );

    let (_revision_dir, revision_root) =
        pipeline_proof_corpus_support::install_foundation_inputs_repo();
    pipeline_proof_corpus_support::install_state_seed(
        &revision_root,
        "revision_conflict_state.yaml",
    );
    let revision_conflict = set_route_state(
        &revision_root,
        &pipeline_id,
        supported_variables.clone(),
        RouteStateMutation::RoutingVariable {
            variable: "needs_project_context".to_string(),
            value: false,
        },
        0,
    )
    .expect("revision conflict outcome");
    pipeline_proof_corpus_support::assert_matches_golden(
        &pipeline_proof_corpus_support::render_pipeline_state_set_output(
            &pipeline_id,
            revision_conflict,
        ),
        &revision_root,
        None,
        "state_set.refused.revision_conflict.txt",
    );

    let (_malformed_dir, malformed_root) =
        pipeline_proof_corpus_support::install_foundation_inputs_repo();
    let malformed_state_path = pipeline_proof_corpus_support::install_state_seed(
        &malformed_root,
        "malformed_route_state.yaml",
    );
    let malformed = load_route_state_with_supported_variables(
        &malformed_root,
        &pipeline_id,
        &supported_variables,
    )
    .expect_err("malformed state refusal");
    pipeline_proof_corpus_support::assert_matches_golden(
        &pipeline_proof_corpus_support::render_load_route_state_refusal(malformed),
        &malformed_root,
        Some(&malformed_state_path),
        "state_set.refused.malformed_route_state.txt",
    );
}

#[test]
fn route_state_storage_layout_public_api_drives_route_resolution_from_non_default_state_root() {
    let (_dir, root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    let storage_layout = custom_storage_layout();
    let definition = load_pipeline_definition(&root, "core/pipelines/foundation_inputs.yaml")
        .expect("proof corpus definition");
    let pipeline_id = definition.header.id.clone();
    let supported_variables = supported_route_state_variables(&definition);

    let first_outcome = set_route_state_with_storage_layout(
        &root,
        &pipeline_id,
        supported_variables.iter().map(String::as_str),
        RouteStateMutation::RoutingVariable {
            variable: "needs_project_context".to_string(),
            value: true,
        },
        0,
        storage_layout,
    )
    .expect("first custom mutation");
    let first_state = match first_outcome {
        RouteStateMutationOutcome::Applied(state) => *state,
        RouteStateMutationOutcome::Refused(refusal) => {
            panic!("expected applied custom mutation, got {refusal:?}")
        }
    };

    let second_outcome = set_route_state_with_storage_layout(
        &root,
        &pipeline_id,
        supported_variables.iter().map(String::as_str),
        RouteStateMutation::RoutingVariable {
            variable: "charter_gaps_detected".to_string(),
            value: true,
        },
        first_state.revision,
        storage_layout,
    )
    .expect("second custom mutation");
    match second_outcome {
        RouteStateMutationOutcome::Applied(_) => {}
        RouteStateMutationOutcome::Refused(refusal) => {
            panic!("expected second custom mutation to apply, got {refusal:?}")
        }
    }

    let custom_path = custom_state_path(&root, &pipeline_id, storage_layout);
    assert!(custom_path.exists(), "custom state path should exist");
    assert!(
        !root
            .join(".handbook")
            .join("state")
            .join("pipeline")
            .join(format!("{pipeline_id}.yaml"))
            .exists(),
        "default state path should remain unused"
    );

    let routed_state = load_route_state_with_storage_layout(&root, &pipeline_id, storage_layout)
        .expect("load custom routed state");
    let route = resolve_pipeline_route(
        &definition,
        &RouteVariables::new(routed_state.routing.clone()).expect("route variables"),
    )
    .expect("route");

    assert!(route
        .stages
        .iter()
        .all(|stage| stage.status == RouteStageStatus::Active));
}
