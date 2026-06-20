use std::fs;
use std::path::{Path, PathBuf};

use handbook_pipeline::declarative_roots::{pipeline_root, profile_root};
use handbook_pipeline::pipeline::SupportedTargetRegistry;
use handbook_pipeline::{
    handbook_product_pipeline_declarative_roots, load_pipeline_catalog,
    load_pipeline_catalog_metadata, load_pipeline_definition, load_pipeline_selection_metadata,
    load_stage_compile_definition, render_pipeline_list, render_pipeline_show, CompileStageInput,
    PipelineCatalogError, PipelineDeclarativeRootsContract, PipelineLoadError, PipelineLookupError,
    PipelineMetadataSelectionError, PipelineSelection, PipelineValidationError,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root")
}

#[test]
fn declarative_root_contract_preserves_handbook_defaults_and_derives_non_default_paths() {
    let defaults = handbook_product_pipeline_declarative_roots();
    assert_eq!(defaults.pipeline_root_relative(), "core/pipelines");
    assert_eq!(defaults.profile_root_relative(), "core/profiles");
    assert_eq!(defaults.runner_root_relative(), "core/runners");
    assert_eq!(defaults.stage_root_relative(), "core/stages");
    assert_eq!(pipeline_root(), defaults.pipeline_root());
    assert_eq!(profile_root(), defaults.profile_root());
    assert_eq!(
        defaults.pipeline_file("foundation.yaml"),
        "core/pipelines/foundation.yaml"
    );
    assert_eq!(defaults.stage_file("00_base.md"), "core/stages/00_base.md");

    let imported = PipelineDeclarativeRootsContract::from_paths(
        ".substrate/handbook/core/pipelines",
        ".substrate/handbook/core/profiles",
        ".substrate/handbook/core/runners",
        ".substrate/handbook/core/stages",
    );
    assert_eq!(
        imported.pipeline_file("foundation.yaml"),
        ".substrate/handbook/core/pipelines/foundation.yaml"
    );
    assert_eq!(
        imported.profile_file("python-uv", "profile.yaml"),
        ".substrate/handbook/core/profiles/python-uv/profile.yaml"
    );
    assert_eq!(
        imported.runner_file("codex-cli"),
        ".substrate/handbook/core/runners/codex-cli.md"
    );
    assert_eq!(
        imported.stage_file("00_base.md"),
        ".substrate/handbook/core/stages/00_base.md"
    );
    assert!(imported.is_profile_file(
        ".substrate/handbook/core/profiles/python-uv/commands.yaml",
        "python-uv"
    ));
}

#[test]
fn catalog_discovers_canonical_pipeline_and_stage_ids() {
    let catalog = load_pipeline_catalog_metadata(repo_root()).expect("catalog");

    assert_eq!(catalog.pipeline_count(), 4);
    assert!(catalog.stage_count() >= 8);

    let pipeline = catalog
        .resolve_selector("foundation")
        .expect("pipeline shorthand");
    match pipeline {
        PipelineSelection::Pipeline(pipeline) => {
            assert_eq!(pipeline.definition.header.id, "pipeline.foundation");
        }
        other => panic!("expected pipeline selection, got {other:?}"),
    }

    let unique_stage = catalog
        .resolve_selector("00_base")
        .expect("stage shorthand");
    match unique_stage {
        PipelineSelection::Stage(stage) => {
            assert_eq!(stage.id, "stage.00_base");
            assert_eq!(
                stage.pipelines,
                vec![
                    "pipeline.foundation".to_string(),
                    "pipeline.foundation_inputs".to_string(),
                    "pipeline.release".to_string(),
                    "pipeline.sprint".to_string(),
                ]
            );
        }
        other => panic!("expected stage selection, got {other:?}"),
    }

    let unsupported = catalog
        .resolve_selector("core/pipelines/foundation.yaml")
        .expect_err("path selector refusal");
    match unsupported {
        PipelineLookupError::UnsupportedSelector { selector, reason } => {
            assert_eq!(selector, "core/pipelines/foundation.yaml");
            assert!(reason.contains("evidence only"));
        }
        other => panic!("expected unsupported-selector refusal, got {other:?}"),
    }
}

#[test]
fn catalog_refuses_ambiguous_shorthand_with_explicit_conflicting_ids() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    write_file(
        &root.join("core/stages/alpha.md"),
        r#"---
kind: stage
id: stage.alpha
version: 0.1.0
title: Alpha Stage
description: alpha
---
# alpha
"#,
    );
    write_file(
        &root.join("core/pipelines/alpha.yaml"),
        r#"---
kind: pipeline
id: pipeline.alpha
version: 0.1.0
title: Alpha Pipeline
description: alpha
---
defaults:
  runner: codex-cli
  profile: python-uv
  enable_complexity: false
stages:
  - id: stage.alpha
    file: core/stages/alpha.md
"#,
    );

    let catalog = load_pipeline_catalog_metadata(root).expect("catalog");
    let err = catalog.resolve_selector("alpha").expect_err("ambiguous");
    match err {
        PipelineLookupError::AmbiguousSelector { selector, matches } => {
            assert_eq!(selector, "alpha");
            assert_eq!(
                matches,
                vec!["pipeline.alpha".to_string(), "stage.alpha".to_string()]
            );
        }
        other => panic!("expected ambiguous-selector refusal, got {other:?}"),
    }
}

#[test]
fn catalog_renders_pipeline_yaml_and_stage_front_matter_as_distinct_sources() {
    let catalog = load_pipeline_catalog_metadata(repo_root()).expect("catalog");

    let pipeline = catalog
        .resolve_selector("pipeline.foundation_inputs")
        .expect("pipeline selection");
    let pipeline_render = render_pipeline_show(&pipeline);
    assert!(pipeline_render.contains("PIPELINE: pipeline.foundation_inputs"));
    assert!(pipeline_render.contains("TITLE: Foundation Pipeline (Dev/Test Charter Inputs"));
    assert!(pipeline_render.contains("SOURCE: core/pipelines/foundation_inputs.yaml"));
    assert!(pipeline_render.contains("DEFAULTS:"));
    assert!(pipeline_render.contains("runner: codex-cli"));
    assert!(pipeline_render.contains("profile: python-uv"));
    assert!(pipeline_render.contains("enable_complexity: false"));
    assert!(pipeline_render.contains("  1. stage.00_base | core/stages/00_base.md"));
    assert!(pipeline_render.contains("stage.04_charter_inputs"));
    assert!(pipeline_render.contains("core/stages/04_charter_inputs.md"));
    assert!(pipeline_render.contains("sets: [needs_project_context]"));
    assert!(pipeline_render.contains(
        "activation: activation.when.any [variables.charter_gaps_detected == true, variables.needs_project_context == true]"
    ));
    assert!(pipeline_render
        .contains("  5. stage.07_foundation_pack | core/stages/07_foundation_pack.md"));
    assert!(pipeline_render.contains("DEFAULTS:"));
    assert!(pipeline_render.contains("STAGES:"));

    let stage = catalog
        .resolve_selector("stage.07_foundation_pack")
        .expect("stage selection");
    let stage_render = render_pipeline_show(&stage);
    assert!(stage_render.contains("STAGE: stage.07_foundation_pack"));
    assert!(stage_render.contains("KIND: stage"));
    assert!(stage_render.contains("VERSION: 0.1.0"));
    assert!(stage_render.contains("TITLE: Foundation Pack Synthesis"));
    assert!(stage_render.contains("DESCRIPTION: Synthesizes project-specific foundation artifacts"));
    assert!(stage_render.contains("WORK_LEVEL: L1"));
    assert!(stage_render.contains("SOURCE: core/stages/07_foundation_pack.md"));
    assert!(stage_render.contains("pipeline.foundation"));
    assert!(stage_render.contains("pipeline.foundation_inputs"));
    assert!(stage_render.contains("PIPELINES:"));
    assert!(!stage_render.contains("DEFAULTS:"));
    assert!(!stage_render.contains("STAGES:"));

    let list = render_pipeline_list(&catalog);
    assert!(list.contains("PIPELINE INVENTORY"));
    assert!(list.contains("PIPELINE COUNT: 4"));
    assert!(list.contains("PIPELINE: pipeline.foundation"));
    assert!(list.contains("SOURCE: core/pipelines/foundation.yaml"));
    assert!(list.contains("PIPELINE: pipeline.sprint"));
}

#[test]
fn catalog_outputs_stay_deterministic_and_ignore_unrelated_route_state() {
    let source_root = repo_root();
    let source_catalog = load_pipeline_catalog_metadata(&source_root).expect("source catalog");

    let baseline_list = render_pipeline_list(&source_catalog);
    let baseline_pipeline = render_pipeline_show(
        &source_catalog
            .resolve_selector("pipeline.foundation_inputs")
            .expect("pipeline selection"),
    );
    let baseline_stage = render_pipeline_show(
        &source_catalog
            .resolve_selector("stage.07_foundation_pack")
            .expect("stage selection"),
    );

    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();
    copy_tree(
        &source_root.join("core/pipelines"),
        &root.join("core/pipelines"),
    );
    copy_tree(&source_root.join("core/stages"), &root.join("core/stages"));

    let state_path = root
        .join(".handbook")
        .join("state")
        .join("pipeline")
        .join("pipeline.foundation_inputs.yaml");
    write_file(
        &state_path,
        r#"---
schema_version: m1-pipeline-state-v1
pipeline_id: pipeline.foundation_inputs
revision: 1
variables:
  unexpected: true
audit:
  - revision: 1
    variable: unexpected
    value: true
"#,
    );

    let entries_before = dir_entries(root.join(".handbook/state/pipeline").as_path());

    let catalog = load_pipeline_catalog_metadata(root).expect("catalog with unrelated route state");
    assert_eq!(render_pipeline_list(&catalog), baseline_list);
    assert_eq!(
        render_pipeline_show(
            &catalog
                .resolve_selector("pipeline.foundation_inputs")
                .expect("pipeline selection")
        ),
        baseline_pipeline
    );
    assert_eq!(
        render_pipeline_show(
            &catalog
                .resolve_selector("stage.07_foundation_pack")
                .expect("stage selection")
        ),
        baseline_stage
    );
    assert_eq!(
        dir_entries(root.join(".handbook/state/pipeline").as_path()),
        entries_before,
        "catalog loading and rendering must not create or mutate route state"
    );
}

#[test]
fn stage_library_inputs_remain_the_authoritative_declarative_source() {
    let root = repo_root();
    let pipeline = load_pipeline_definition(&root, "core/pipelines/foundation_inputs.yaml")
        .expect("foundation inputs pipeline");

    let charter_stage =
        load_stage_compile_definition(&root, &pipeline, "stage.05_charter_synthesize")
            .expect("charter synthesize stage");
    assert_eq!(
        charter_stage.inputs.library,
        vec![
            CompileStageInput {
                path: "core/library/charter/charter_synthesize_directive.md".to_string(),
                required: true,
            },
            CompileStageInput {
                path: "core/library/charter/charter.md.tmpl".to_string(),
                required: true,
            },
        ]
    );

    let foundation_pack_stage =
        load_stage_compile_definition(&root, &pipeline, "stage.07_foundation_pack")
            .expect("foundation pack stage");
    assert!(foundation_pack_stage
        .inputs
        .library
        .contains(&CompileStageInput {
            path: "core/library/environment_inventory/environment_inventory_directive.md"
                .to_string(),
            required: true,
        }));
    assert!(foundation_pack_stage
        .inputs
        .library
        .contains(&CompileStageInput {
            path: "core/library/environment_inventory/ENVIRONMENT_INVENTORY.md.tmpl".to_string(),
            required: true,
        }));
}

#[test]
fn metadata_catalog_keeps_activation_drift_out_of_inventory_inspection() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    write_file(
        &root.join("core/stages/00_base.md"),
        r#"---
kind: stage
id: stage.00_base
version: 0.1.0
title: Base
description: base
activation:
  when:
    any:
      - variables.needs_project_context == true
---
# base
"#,
    );
    write_file(
        &root.join("core/pipelines/drift.yaml"),
        r#"---
kind: pipeline
id: pipeline.drift
version: 0.1.0
title: Drift
description: drift
---
defaults:
  runner: codex-cli
  profile: python-uv
  enable_complexity: false
stages:
  - id: stage.00_base
    file: core/stages/00_base.md
"#,
    );

    let catalog = load_pipeline_catalog_metadata(root).expect("metadata catalog should load");
    let list = render_pipeline_list(&catalog);
    assert!(list.contains("PIPELINE INVENTORY"));
    assert!(list.contains("PIPELINE: pipeline.drift"));

    let pipeline = catalog
        .resolve_selector("pipeline.drift")
        .expect("pipeline selection");
    let pipeline_render = render_pipeline_show(&pipeline);
    assert!(pipeline_render.contains("PIPELINE: pipeline.drift"));
    assert!(pipeline_render.contains("SOURCE: core/pipelines/drift.yaml"));
    assert!(pipeline_render.contains("stage.00_base"));

    let stage = catalog
        .resolve_selector("stage.00_base")
        .expect("stage selection");
    let stage_render = render_pipeline_show(&stage);
    assert!(stage_render.contains("STAGE: stage.00_base"));
    assert!(stage_render.contains("PIPELINES:"));
}

#[test]
fn route_aware_catalog_still_refuses_activation_drift_between_pipeline_yaml_and_stage_front_matter()
{
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    write_file(
        &root.join("core/stages/00_base.md"),
        r#"---
kind: stage
id: stage.00_base
version: 0.1.0
title: Base
description: base
activation:
  when:
    any:
      - variables.needs_project_context == true
---
# base
"#,
    );
    write_file(
        &root.join("core/pipelines/drift.yaml"),
        r#"---
kind: pipeline
id: pipeline.drift
version: 0.1.0
title: Drift
description: drift
---
defaults:
  runner: codex-cli
  profile: python-uv
  enable_complexity: false
stages:
  - id: stage.00_base
    file: core/stages/00_base.md
"#,
    );

    let err = load_pipeline_catalog(root).expect_err("route-aware catalog should refuse");

    match err {
        PipelineCatalogError::PipelineLoad { source, .. } => match source.as_ref() {
            PipelineLoadError::Validation {
                error: PipelineValidationError::ActivationDrift { stage_id, file, .. },
                ..
            } => {
                assert_eq!(stage_id, "stage.00_base");
                assert_eq!(file, "core/stages/00_base.md");
            }
            other => panic!("expected activation-drift validation, got {other:?}"),
        },
        other => panic!("expected pipeline-load catalog refusal, got {other:?}"),
    }
}

#[test]
fn metadata_catalog_ignores_unused_broken_stage_files_but_strict_catalog_still_refuses() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    write_file(
        &root.join("core/stages/00_base.md"),
        r#"---
kind: stage
id: stage.00_base
version: 0.1.0
title: Base
description: base
---
# base
"#,
    );
    write_file(
        &root.join("core/stages/99_bad_unused.md"),
        r#"---
kind: nonsense
id: stage.bad_unused
version: 0.1.0
title: Bad Unused Stage
description: bad
---
# bad
"#,
    );
    write_file(
        &root.join("core/pipelines/foundation.yaml"),
        r#"---
kind: pipeline
id: pipeline.foundation
version: 0.1.0
title: Foundation
description: foundation
---
defaults:
  runner: codex-cli
  profile: python-uv
  enable_complexity: false
stages:
  - id: stage.00_base
    file: core/stages/00_base.md
"#,
    );

    let catalog = load_pipeline_catalog_metadata(root).expect("metadata catalog should load");
    assert_eq!(catalog.pipeline_count(), 1);
    assert_eq!(catalog.stage_count(), 1);
    let list = render_pipeline_list(&catalog);
    assert!(list.contains("PIPELINE: pipeline.foundation"));
    assert!(!list.contains("stage.bad_unused"));

    let err = load_pipeline_catalog(root).expect_err("strict catalog should refuse");
    match err {
        PipelineCatalogError::StageKindMismatch { path, actual } => {
            assert_eq!(path, root.join("core/stages/99_bad_unused.md"));
            assert_eq!(actual, "nonsense");
        }
        other => panic!("expected stage-kind-mismatch refusal, got {other:?}"),
    }
}

#[test]
fn metadata_catalog_ignores_unrelated_broken_pipeline_but_strict_catalog_still_refuses() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    write_file(
        &root.join("core/stages/00_base.md"),
        r#"---
kind: stage
id: stage.00_base
version: 0.1.0
title: Base
description: base
---
# base
"#,
    );
    write_file(
        &root.join("core/pipelines/foundation.yaml"),
        r#"---
kind: pipeline
id: pipeline.foundation
version: 0.1.0
title: Foundation
description: foundation
---
defaults:
  runner: codex-cli
  profile: python-uv
  enable_complexity: false
stages:
  - id: stage.00_base
    file: core/stages/00_base.md
"#,
    );
    write_file(
        &root.join("core/pipelines/bad-id.yaml"),
        r#"---
kind: pipeline
id: pipeline.bad/path
version: 0.1.0
title: Bad Id
description: bad
---
defaults:
  runner: codex-cli
  profile: python-uv
  enable_complexity: false
stages:
  - id: stage.00_base
    file: core/stages/00_base.md
"#,
    );

    let catalog = load_pipeline_catalog_metadata(root).expect("metadata catalog should load");
    assert_eq!(catalog.pipeline_count(), 1);
    assert!(catalog.resolve_selector("pipeline.foundation").is_ok());
    assert!(catalog.resolve_selector("pipeline.bad/path").is_err());

    let err = load_pipeline_catalog(root).expect_err("strict catalog should refuse");
    match err {
        PipelineCatalogError::PipelineLoad { source, .. } => match source.as_ref() {
            PipelineLoadError::Validation {
                error: PipelineValidationError::InvalidCanonicalId { value, .. },
                ..
            } => {
                assert_eq!(value, "pipeline.bad/path");
            }
            other => panic!("expected invalid-canonical-id refusal, got {other:?}"),
        },
        other => panic!("expected pipeline-load refusal, got {other:?}"),
    }
}

#[test]
fn metadata_selection_refuses_when_selected_pipeline_is_malformed() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    write_file(
        &root.join("core/stages/00_base.md"),
        r#"---
kind: stage
id: stage.00_base
version: 0.1.0
title: Base
description: base
---
# base
"#,
    );
    write_file(
        &root.join("core/pipelines/foundation.yaml"),
        r#"---
kind: pipeline
id: pipeline.foundation
version: 0.1.0
title: Foundation
description: foundation
---
defaults:
  runner: codex-cli
  profile: python-uv
  enable_complexity: false
stages:
  - id: stage.00_base
    file: core/stages/00_base.md
"#,
    );
    write_file(
        &root.join("core/pipelines/broken.yaml"),
        r#"---
kind: pipeline
id: pipeline.broken
version: 0.1.0
title: Broken
description: broken
---
defaults:
  runner:
"#,
    );

    let err = load_pipeline_selection_metadata(root, "pipeline.broken")
        .expect_err("selected malformed pipeline should refuse");
    match err {
        PipelineMetadataSelectionError::Catalog(PipelineCatalogError::PipelineLoad {
            source,
            ..
        }) => match source.as_ref() {
            PipelineLoadError::BodyParse { .. } => {}
            other => panic!("expected body-parse refusal, got {other:?}"),
        },
        other => panic!("expected catalog pipeline-load refusal, got {other:?}"),
    }
}

#[test]
fn metadata_selection_refuses_when_selected_pipeline_references_broken_stage_metadata() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    write_file(
        &root.join("core/stages/00_base.md"),
        r#"---
kind: stage
id: stage.00_base
version: 0.1.0
title: Base
description: base
---
# base
"#,
    );
    write_file(
        &root.join("core/stages/bad.md"),
        r#"---
kind: nonsense
id: stage.bad
version: 0.1.0
title: Bad
description: bad
---
# bad
"#,
    );
    write_file(
        &root.join("core/pipelines/foundation.yaml"),
        r#"---
kind: pipeline
id: pipeline.foundation
version: 0.1.0
title: Foundation
description: foundation
---
defaults:
  runner: codex-cli
  profile: python-uv
  enable_complexity: false
stages:
  - id: stage.00_base
    file: core/stages/00_base.md
"#,
    );
    write_file(
        &root.join("core/pipelines/broken.yaml"),
        r#"---
kind: pipeline
id: pipeline.broken
version: 0.1.0
title: Broken
description: broken
---
defaults:
  runner: codex-cli
  profile: python-uv
  enable_complexity: false
stages:
  - id: stage.bad
    file: core/stages/bad.md
"#,
    );

    let err = load_pipeline_selection_metadata(root, "pipeline.broken")
        .expect_err("selected pipeline with broken stage metadata should refuse");
    match err {
        PipelineMetadataSelectionError::Catalog(PipelineCatalogError::StageKindMismatch {
            path,
            actual,
        }) => {
            assert_eq!(path, root.join("core/stages/bad.md"));
            assert_eq!(actual, "nonsense");
        }
        other => panic!("expected stage-kind-mismatch refusal, got {other:?}"),
    }
}

#[test]
fn supported_target_registry_derives_current_pipeline_and_stage_wedge_from_catalog_truth() {
    let registry = SupportedTargetRegistry::load(repo_root()).expect("supported target registry");

    assert_eq!(
        registry.canonical_compile_pipeline_id(),
        "pipeline.foundation_inputs"
    );
    assert_eq!(
        registry.canonical_compile_stage_id(),
        "stage.10_feature_spec"
    );

    let compile_target = registry.compile_target();
    assert_eq!(compile_target.pipeline.id, "pipeline.foundation_inputs");
    assert_eq!(compile_target.stage.id, "stage.10_feature_spec");

    let capture_stage_ids = registry
        .stages()
        .filter(|stage| registry.supports_capture_target(&compile_target.pipeline.id, &stage.id))
        .map(|stage| stage.id.clone())
        .collect::<Vec<_>>();
    assert_eq!(
        capture_stage_ids,
        vec![
            "stage.04_charter_inputs".to_string(),
            "stage.05_charter_synthesize".to_string(),
            "stage.06_project_context_interview".to_string(),
            "stage.07_foundation_pack".to_string(),
            "stage.10_feature_spec".to_string(),
        ]
    );
    assert!(!registry.supports_capture_target(&compile_target.pipeline.id, "stage.00_base"));
}

#[test]
fn explicit_roots_drive_catalog_discovery_and_supported_target_derivation() {
    let source_root = repo_root();
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();
    let roots = PipelineDeclarativeRootsContract::from_paths(
        "core/pipelines",
        "core/profiles",
        "core/runners",
        ".substrate/handbook/core/stages",
    );

    copy_tree(
        &source_root.join("core/stages"),
        &root.join(roots.stage_root_relative()),
    );

    let imported_pipeline =
        fs::read_to_string(source_root.join("core/pipelines/foundation_inputs.yaml"))
            .expect("read source pipeline")
            .replace("core/stages/", &format!("{}/", roots.stage_root_relative()));
    write_file(
        &root.join(roots.pipeline_file("foundation_inputs.yaml")),
        &imported_pipeline,
    );

    let catalog = handbook_pipeline::pipeline::load_pipeline_catalog_with_roots(root, &roots)
        .expect("catalog should discover stages from the explicit stage root");
    assert_eq!(catalog.pipeline_count(), 1);
    assert!(catalog.stage_count() >= 6);

    let stage = catalog
        .resolve_selector("stage.10_feature_spec")
        .expect("stage selection");
    match stage {
        PipelineSelection::Stage(stage) => {
            assert_eq!(
                stage.source_path,
                PathBuf::from(roots.stage_file("10_feature_spec.md"))
            );
        }
        other => panic!("expected stage selection, got {other:?}"),
    }

    let registry = SupportedTargetRegistry::load_with_roots(root, &roots)
        .expect("supported target registry should follow explicit stage roots");
    assert_eq!(
        registry.compile_target().stage.source_path,
        PathBuf::from(roots.stage_file("10_feature_spec.md"))
    );
    assert_eq!(
        registry.compile_target().pipeline.id,
        "pipeline.foundation_inputs"
    );
}

#[test]
fn supported_target_registry_refuses_foundation_inputs_shape_that_widens_beyond_packet_wedge() {
    let source_root = repo_root();
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();
    copy_tree(
        &source_root.join("core/pipelines"),
        &root.join("core/pipelines"),
    );
    copy_tree(&source_root.join("core/stages"), &root.join("core/stages"));

    write_file(
        &root.join("core/stages/11_extra_capture.md"),
        r#"---
kind: stage
id: stage.11_extra_capture
version: 0.1.0
title: Extra Capture
description: extra capture stage
---
# extra
"#,
    );
    write_file(
        &root.join("core/pipelines/foundation_inputs.yaml"),
        r#"---
kind: pipeline
id: pipeline.foundation_inputs
version: 0.1.0
title: "Foundation Pipeline (Dev/Test Charter Inputs → Charter → Context? → Foundation Pack)"
description: >
  Development/testing pipeline that avoids the multi-turn Charter interview by generating
  CHARTER_INPUTS.yaml and synthesizing CHARTER.md from it in a single shot.
---
defaults:
  runner: codex-cli
  profile: python-uv
  enable_complexity: false

stages:
  - id: stage.00_base
    file: core/stages/00_base.md
  - id: stage.04_charter_inputs
    file: core/stages/04_charter_inputs.md
  - id: stage.05_charter_synthesize
    file: core/stages/05_charter_synthesize.md
    sets:
      - needs_project_context
  - id: stage.06_project_context_interview
    file: core/stages/06_project_context_interview.md
    activation:
      when:
        any:
          - variables.needs_project_context == true
          - variables.charter_gaps_detected == true
  - id: stage.07_foundation_pack
    file: core/stages/07_foundation_pack.md
  - id: stage.11_extra_capture
    file: core/stages/11_extra_capture.md
  - id: stage.10_feature_spec
    file: core/stages/10_feature_spec.md
"#,
    );

    let err = SupportedTargetRegistry::load(root).expect_err("extra capture stage must refuse");
    match err {
        handbook_pipeline::pipeline::SupportedTargetRegistryLoadError::MissingCatalogBackedPipelineShape {
            ..
        } => {}
        other => panic!("expected bounded-pipeline-shape refusal, got {other:?}"),
    }
}

#[test]
fn supported_target_registry_ignores_unrelated_compile_broken_pipeline_during_topology_resolution()
{
    let source_root = repo_root();
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();
    copy_tree(
        &source_root.join("core/pipelines"),
        &root.join("core/pipelines"),
    );
    copy_tree(&source_root.join("core/stages"), &root.join("core/stages"));

    write_file(
        &root.join("core/stages/99_unrelated.md"),
        r#"---
kind: stage
id: stage.99_unrelated
version: 0.1.0
title: Unrelated
description: unrelated stage whose compile front matter is intentionally invalid
activation:
  when:
    any:
      - variables.unrelated == true
---
# unrelated
"#,
    );
    write_file(
        &root.join("core/pipelines/unrelated.yaml"),
        r#"---
kind: pipeline
id: pipeline.unrelated
version: 0.1.0
title: Unrelated
description: unrelated pipeline
---
defaults:
  runner: codex-cli
  profile: python-uv
  enable_complexity: false

stages:
  - id: stage.00_base
    file: core/stages/00_base.md
  - id: stage.99_unrelated
    file: core/stages/99_unrelated.md
    activation:
      when:
        any:
          - variables.unrelated == true
"#,
    );

    let unrelated_pipeline = load_pipeline_definition(root, "core/pipelines/unrelated.yaml")
        .expect("unrelated pipeline definition");
    let broken_stage_err =
        load_stage_compile_definition(root, &unrelated_pipeline, "stage.99_unrelated")
            .expect_err("unrelated stage compile definition should be broken");
    match broken_stage_err {
        handbook_pipeline::CompileStageLoadError::ParseFrontMatter { .. } => {}
        other => panic!("expected compile front-matter parse refusal, got {other:?}"),
    }

    let registry = SupportedTargetRegistry::load(root)
        .expect("supported target registry should ignore unrelated broken pipeline");
    assert_eq!(
        registry.canonical_compile_pipeline_id(),
        "pipeline.foundation_inputs"
    );
    assert_eq!(
        registry.canonical_compile_stage_id(),
        "stage.10_feature_spec"
    );
}

fn write_file(path: &Path, contents: &str) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("mkdirs");
    }
    std::fs::write(path, contents).expect("write");
}

fn copy_tree(src: &Path, dst: &Path) {
    fs::create_dir_all(dst).expect("mkdirs");

    for entry in fs::read_dir(src).expect("read dir") {
        let entry = entry.expect("dir entry");
        let file_type = entry.file_type().expect("file type");
        let target = dst.join(entry.file_name());

        if file_type.is_dir() {
            copy_tree(&entry.path(), &target);
        } else if file_type.is_file() {
            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent).expect("mkdirs");
            }
            fs::copy(entry.path(), &target).expect("copy file");
        }
    }
}

fn dir_entries(path: &Path) -> Vec<String> {
    if !path.exists() {
        return Vec::new();
    }

    let mut entries = fs::read_dir(path)
        .expect("read dir")
        .map(|entry| {
            entry
                .expect("dir entry")
                .file_name()
                .to_string_lossy()
                .to_string()
        })
        .collect::<Vec<_>>();
    entries.sort();
    entries
}
