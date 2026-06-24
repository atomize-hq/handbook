// Released-boundary proof fixture: same public API exercise as the Set 2 packaged proof,
// but intended for an exact crates.io release pin only.

use handbook_pipeline::{
    pipeline::{
        load_pipeline_catalog_metadata_with_roots, load_pipeline_definition,
        load_pipeline_definition_with_roots, load_pipeline_selection_metadata_with_roots,
        load_selected_pipeline_definition_with_roots, supported_route_state_variables,
        PipelineSelection,
    },
    pipeline_capture::{
        apply_pipeline_capture_with_storage_layout, preview_pipeline_capture_with_storage_layout,
        PipelineCaptureRequest,
    },
    pipeline_handoff::{
        emit_pipeline_handoff_bundle_with_storage_layout,
        validate_pipeline_handoff_bundle_with_storage_layout, PipelineHandoffEmitRequest,
    },
    pipeline_route::{resolve_pipeline_route, RouteVariables},
    route_state::{
        build_route_basis, load_route_state_with_storage_layout,
        load_trusted_pipeline_session_with_storage_layout, persist_route_basis_with_storage_layout,
        set_route_state_with_storage_layout, RouteBasisPersistOutcome, RouteStateMutation,
        RouteStateMutationOutcome,
    },
    PipelineDeclarativeRootsContract, PipelineStorageLayoutContract,
};
use std::error::Error;
use std::fs;
use std::io::{Error as IoError, ErrorKind};
use std::path::{Path, PathBuf};

const PIPELINE_ID: &str = "pipeline.foundation_inputs";
const STAGE_ID: &str = "stage.10_feature_spec";
const CONSUMER_ID: &str = "feature-slice-decomposer";

fn main() -> Result<(), Box<dyn Error>> {
    let proof_corpus_root = std::env::args().nth(1).ok_or_else(|| {
        io_error("usage: handbook-pipeline-minimal-boundary-proof <proof_corpus_root>")
    })?;
    let proof_corpus_root = PathBuf::from(proof_corpus_root);
    let repo_root = proof_corpus_root.join("repo");
    let model_output_path = proof_corpus_root
        .join("model_outputs")
        .join("stage_10_feature_spec.md");

    ensure(
        repo_root.is_dir(),
        format!("missing proof repo at {}", repo_root.display()),
    )?;
    ensure(
        model_output_path.is_file(),
        format!(
            "missing stage 10 model output at {}",
            model_output_path.display()
        ),
    )?;

    let declarative_roots = install_custom_declarative_roots_fixture(&repo_root)?;
    let storage_layout = PipelineStorageLayoutContract::try_from_paths(
        ".custom_handbook/state",
        ".custom_handbook/state/pipelines",
        ".custom_handbook/state/pipelines/stage_capture",
        ".custom_handbook/state/pipelines/capture_cache",
        "custom_artifacts/handoff/feature_slice",
    )?;
    install_canonical_inputs(&repo_root, &model_output_path)?;

    prove_metadata_and_definition_family(&repo_root, &declarative_roots)?;
    prove_route_state_family(&repo_root, storage_layout)?;
    prove_capture_family(&repo_root, &model_output_path, storage_layout)?;
    prove_handoff_family(&repo_root, storage_layout)?;

    println!("PASS metadata/definition public custom-roots facade");
    println!("PASS route-state public custom-storage facade");
    println!("PASS capture public custom-storage facade");
    println!("PASS handoff public custom-storage facade");
    Ok(())
}

fn prove_metadata_and_definition_family(
    repo_root: &Path,
    declarative_roots: &PipelineDeclarativeRootsContract,
) -> Result<(), Box<dyn Error>> {
    let catalog = load_pipeline_catalog_metadata_with_roots(repo_root, declarative_roots)?;
    ensure(
        catalog.pipeline_count() >= 1,
        "expected at least one pipeline".to_string(),
    )?;
    ensure(
        catalog.stage_count() >= 1,
        "expected at least one stage".to_string(),
    )?;

    let selection =
        load_pipeline_selection_metadata_with_roots(repo_root, declarative_roots, PIPELINE_ID)?;
    match selection {
        PipelineSelection::Pipeline(entry) => {
            ensure(
                entry.definition.header.id == PIPELINE_ID,
                format!(
                    "unexpected selected pipeline id: {}",
                    entry.definition.header.id
                ),
            )?;
            ensure(
                entry
                    .definition
                    .source_path
                    .starts_with(Path::new("custom/core/pipelines")),
                format!(
                    "selection source path did not use custom roots: {}",
                    entry.definition.source_path.display()
                ),
            )?;
        }
        other => {
            return Err(io_error(format!(
                "expected pipeline selection for {PIPELINE_ID}, got {other:?}"
            ))
            .into())
        }
    }

    let direct = load_pipeline_definition_with_roots(
        repo_root,
        declarative_roots,
        "custom/core/pipelines/foundation_inputs.yaml",
    )?;
    ensure(
        direct.header.id == PIPELINE_ID,
        format!("unexpected direct definition id: {}", direct.header.id),
    )?;

    let selected =
        load_selected_pipeline_definition_with_roots(repo_root, declarative_roots, PIPELINE_ID)?;
    ensure(
        selected.header.id == PIPELINE_ID,
        format!("unexpected selected definition id: {}", selected.header.id),
    )?;
    ensure(
        selected
            .source_path
            .starts_with(Path::new("custom/core/pipelines")),
        format!(
            "selected definition source path did not use custom roots: {}",
            selected.source_path.display()
        ),
    )?;

    Ok(())
}

fn prove_route_state_family(
    repo_root: &Path,
    storage_layout: PipelineStorageLayoutContract,
) -> Result<(), Box<dyn Error>> {
    let definition = load_pipeline_definition(repo_root, "core/pipelines/foundation_inputs.yaml")?;
    let supported_variables = supported_route_state_variables(&definition);

    let initial_state =
        load_route_state_with_storage_layout(repo_root, PIPELINE_ID, storage_layout)?;
    ensure(
        initial_state.revision == 0,
        format!(
            "expected empty route state revision 0, got {}",
            initial_state.revision
        ),
    )?;

    let mutations = [
        RouteStateMutation::RunRunner {
            value: "codex-cli".to_string(),
        },
        RouteStateMutation::RunProfile {
            value: "python-uv".to_string(),
        },
        RouteStateMutation::RefCharterRef {
            value: "artifacts/charter/CHARTER.md".to_string(),
        },
        RouteStateMutation::RefProjectContextRef {
            value: "artifacts/project_context/PROJECT_CONTEXT.md".to_string(),
        },
        RouteStateMutation::RoutingVariable {
            variable: "needs_project_context".to_string(),
            value: false,
        },
        RouteStateMutation::RoutingVariable {
            variable: "charter_gaps_detected".to_string(),
            value: false,
        },
    ];

    for mutation in mutations {
        apply_route_state_mutation(repo_root, &supported_variables, mutation, storage_layout)?;
    }

    let state = load_route_state_with_storage_layout(repo_root, PIPELINE_ID, storage_layout)?;
    ensure(
        state.revision == 6,
        format!(
            "expected six applied route-state revisions, got {}",
            state.revision
        ),
    )?;

    let route = resolve_pipeline_route(&definition, &RouteVariables::new(state.routing.clone())?)?;
    let route_basis = build_route_basis(repo_root, &definition, &state, &route)?;
    match persist_route_basis_with_storage_layout(
        repo_root,
        PIPELINE_ID,
        route_basis,
        storage_layout,
    )? {
        RouteBasisPersistOutcome::Applied(applied) => {
            ensure(
                applied.route_basis.is_some(),
                "expected persisted route basis in applied state".to_string(),
            )?;
        }
        RouteBasisPersistOutcome::Refused(refusal) => {
            return Err(io_error(format!(
                "expected route-basis persistence to apply, got refusal: {refusal:?}"
            ))
            .into())
        }
    }

    let trusted =
        load_trusted_pipeline_session_with_storage_layout(repo_root, &definition, storage_layout)
            .map_err(|refusal| io_error(format!("trusted session refusal: {refusal:?}")))?;
    ensure(
        trusted.pipeline_id == PIPELINE_ID,
        format!(
            "unexpected trusted session pipeline id: {}",
            trusted.pipeline_id
        ),
    )?;
    ensure(
        trusted.route_state.route_basis.is_some(),
        "trusted session should carry a route basis".to_string(),
    )?;

    let custom_state_path = repo_root
        .join(storage_layout.pipeline_dir_relative())
        .join(format!("{PIPELINE_ID}.yaml"));
    ensure(
        custom_state_path.is_file(),
        format!(
            "expected custom storage route-state file at {}",
            custom_state_path.display()
        ),
    )?;

    Ok(())
}

fn prove_capture_family(
    repo_root: &Path,
    model_output_path: &Path,
    storage_layout: PipelineStorageLayoutContract,
) -> Result<(), Box<dyn Error>> {
    let request = PipelineCaptureRequest {
        pipeline_selector: PIPELINE_ID.to_string(),
        stage_selector: STAGE_ID.to_string(),
        input: normalize_newlines(&fs::read_to_string(model_output_path)?),
    };

    let preview = preview_pipeline_capture_with_storage_layout(repo_root, &request, storage_layout)
        .map_err(|refusal| io_error(format!("capture preview refusal: {refusal:?}")))?;
    ensure(
        !preview.plan.capture_id.is_empty(),
        "expected capture preview to produce a capture id".to_string(),
    )?;

    let cache_entry_path = repo_root
        .join(storage_layout.capture_cache_root_relative())
        .join(format!("{}.yaml", preview.plan.capture_id));
    ensure(
        cache_entry_path.is_file(),
        format!(
            "expected custom capture cache entry at {}",
            cache_entry_path.display()
        ),
    )?;

    let applied = apply_pipeline_capture_with_storage_layout(
        repo_root,
        &preview.plan.capture_id,
        storage_layout,
    )
    .map_err(|refusal| io_error(format!("capture apply refusal: {refusal:?}")))?;
    ensure(
        !applied.written_files.is_empty(),
        "expected capture apply to write files".to_string(),
    )?;

    let stage_capture_provenance_path = repo_root
        .join(storage_layout.stage_capture_root_relative())
        .join(format!("{PIPELINE_ID}.{STAGE_ID}.json"));
    ensure(
        stage_capture_provenance_path.is_file(),
        format!(
            "expected custom stage-capture provenance at {}",
            stage_capture_provenance_path.display()
        ),
    )?;

    Ok(())
}

fn prove_handoff_family(
    repo_root: &Path,
    storage_layout: PipelineStorageLayoutContract,
) -> Result<(), Box<dyn Error>> {
    let emitted = emit_pipeline_handoff_bundle_with_storage_layout(
        repo_root,
        &PipelineHandoffEmitRequest {
            pipeline_selector: PIPELINE_ID.to_string(),
            consumer_selector: CONSUMER_ID.to_string(),
            producer_command: format!(
                "handbook pipeline handoff emit --id {PIPELINE_ID} --consumer {CONSUMER_ID}"
            ),
            producer_version: "set-2-packet-2-5-proof".to_string(),
        },
        storage_layout,
    )
    .map_err(|refusal| io_error(format!("handoff emit refusal: {refusal:?}")))?;

    ensure(
        emitted.manifest.pipeline_id == PIPELINE_ID,
        format!(
            "unexpected handoff manifest pipeline id: {}",
            emitted.manifest.pipeline_id
        ),
    )?;
    ensure(
        emitted.manifest.consumer_id == CONSUMER_ID,
        format!(
            "unexpected handoff manifest consumer id: {}",
            emitted.manifest.consumer_id
        ),
    )?;
    ensure(
        emitted
            .bundle_root
            .starts_with(storage_layout.feature_slice_root_relative()),
        format!(
            "expected bundle root under custom handoff layout, got {}",
            emitted.bundle_root
        ),
    )?;

    let validated = validate_pipeline_handoff_bundle_with_storage_layout(
        repo_root,
        &emitted.bundle_root,
        storage_layout,
    )
    .map_err(|failure| io_error(format!("handoff validation failure: {failure:?}")))?;
    ensure(
        validated.manifest.pipeline_id == PIPELINE_ID,
        format!(
            "unexpected validated handoff pipeline id: {}",
            validated.manifest.pipeline_id
        ),
    )?;
    ensure(
        validated.manifest.consumer_id == CONSUMER_ID,
        format!(
            "unexpected validated handoff consumer id: {}",
            validated.manifest.consumer_id
        ),
    )?;

    Ok(())
}

fn apply_route_state_mutation(
    repo_root: &Path,
    supported_variables: &std::collections::BTreeSet<String>,
    mutation: RouteStateMutation,
    storage_layout: PipelineStorageLayoutContract,
) -> Result<(), Box<dyn Error>> {
    let state = load_route_state_with_storage_layout(repo_root, PIPELINE_ID, storage_layout)?;
    match set_route_state_with_storage_layout(
        repo_root,
        PIPELINE_ID,
        supported_variables.iter().map(String::as_str),
        mutation,
        state.revision,
        storage_layout,
    )? {
        RouteStateMutationOutcome::Applied(_) => Ok(()),
        RouteStateMutationOutcome::Refused(refusal) => {
            Err(io_error(format!("route-state mutation refused: {refusal:?}")).into())
        }
    }
}

fn install_canonical_inputs(
    repo_root: &Path,
    model_output_path: &Path,
) -> Result<(), Box<dyn Error>> {
    copy_file(
        &repo_root.join("artifacts/charter/CHARTER.md"),
        &repo_root.join(".handbook/charter/CHARTER.md"),
    )?;
    copy_file(
        &repo_root.join("artifacts/project_context/PROJECT_CONTEXT.md"),
        &repo_root.join(".handbook/project_context/PROJECT_CONTEXT.md"),
    )?;
    copy_file(
        model_output_path,
        &repo_root.join(".handbook/feature_spec/FEATURE_SPEC.md"),
    )?;
    Ok(())
}

fn install_custom_declarative_roots_fixture(
    repo_root: &Path,
) -> Result<PipelineDeclarativeRootsContract, Box<dyn Error>> {
    copy_tree(
        &repo_root.join("core/pipelines"),
        &repo_root.join("custom/core/pipelines"),
    )?;
    copy_tree(
        &repo_root.join("core/profiles"),
        &repo_root.join("custom/core/profiles"),
    )?;
    copy_tree(
        &repo_root.join("core/runners"),
        &repo_root.join("custom/core/runners"),
    )?;
    copy_tree(
        &repo_root.join("core/stages"),
        &repo_root.join("custom/core/stages"),
    )?;

    for entry in fs::read_dir(repo_root.join("custom/core/pipelines"))? {
        let path = entry?.path();
        let contents = normalize_newlines(&fs::read_to_string(&path)?);
        fs::write(
            &path,
            contents.replace("file: core/stages/", "file: custom/core/stages/"),
        )?;
    }

    Ok(PipelineDeclarativeRootsContract::try_from_paths(
        "custom/core/pipelines",
        "custom/core/profiles",
        "custom/core/runners",
        "custom/core/stages",
    )?)
}

fn copy_tree(source: &Path, dest: &Path) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(dest)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let source_path = entry.path();
        let dest_path = dest.join(entry.file_name());
        let metadata = entry.metadata()?;
        if metadata.is_dir() {
            copy_tree(&source_path, &dest_path)?;
        } else if metadata.is_file() {
            copy_file(&source_path, &dest_path)?;
        } else {
            return Err(io_error(format!(
                "unsupported proof corpus entry {}",
                source_path.display()
            ))
            .into());
        }
    }
    Ok(())
}

fn copy_file(source: &Path, dest: &Path) -> Result<(), Box<dyn Error>> {
    let parent = dest
        .parent()
        .ok_or_else(|| io_error(format!("destination has no parent: {}", dest.display())))?;
    fs::create_dir_all(parent)?;
    fs::copy(source, dest)?;
    Ok(())
}

fn ensure(condition: bool, message: String) -> Result<(), Box<dyn Error>> {
    if condition {
        Ok(())
    } else {
        Err(io_error(message).into())
    }
}

fn io_error(message: impl Into<String>) -> IoError {
    IoError::new(ErrorKind::Other, message.into())
}

fn normalize_newlines(value: &str) -> String {
    value.replace("\r\n", "\n")
}
