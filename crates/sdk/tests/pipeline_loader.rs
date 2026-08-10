use std::path::{Path, PathBuf};

use handbook_pipeline::pipeline::{
    load_pipeline_definition, ActivationOperator, ActivationValidationError, PipelineLoadError,
    PipelineValidationError, StageFileValidationError,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root")
}

fn write_file(path: &Path, contents: &str) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("mkdirs");
    }
    std::fs::write(path, contents).expect("write");
}

fn write_stage_with_front_matter(repo_root: &Path, relative_path: &str, front_matter_body: &str) {
    write_file(
        &repo_root.join(relative_path),
        &format!("---\n{front_matter_body}---\n# stage body\n"),
    );
}

#[test]
fn foundation_pipeline_loads_with_deterministic_stage_order() {
    let repo_root = repo_root();

    let definition = load_pipeline_definition(&repo_root, "core/pipelines/foundation.yaml")
        .expect("load pipeline");

    assert_eq!(definition.header.kind, "pipeline");
    assert_eq!(definition.header.id, "pipeline.foundation");
    assert_eq!(definition.body.defaults.runner, "codex-cli");
    assert_eq!(definition.body.defaults.profile, "python-uv");
    assert!(!definition.body.defaults.enable_complexity);
    assert_eq!(
        definition
            .body
            .stages
            .iter()
            .map(|stage| stage.id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "stage.00_base",
            "stage.05_charter_interview",
            "stage.06_project_context_interview",
            "stage.07_foundation_pack",
        ]
    );
    assert_eq!(
        definition.body.stages[1].sets.as_ref().expect("sets"),
        &vec!["needs_project_context".to_string()]
    );

    let activation = definition.body.stages[2]
        .activation
        .as_ref()
        .expect("activation");
    assert_eq!(activation.when.operator, ActivationOperator::Any);
    assert_eq!(activation.when.clauses.len(), 1);
    assert_eq!(activation.when.clauses[0].variable, "needs_project_context");
    assert!(activation.when.clauses[0].value);
}

#[test]
fn foundation_inputs_pipeline_parses_pipeline_entry_activation_only() {
    let repo_root = repo_root();

    let definition = load_pipeline_definition(&repo_root, "core/pipelines/foundation_inputs.yaml")
        .expect("load pipeline");

    let activation = definition.body.stages[3]
        .activation
        .as_ref()
        .expect("activation");
    assert_eq!(activation.when.operator, ActivationOperator::Any);
    assert_eq!(activation.when.clauses.len(), 2);
    assert_eq!(activation.when.clauses[0].variable, "needs_project_context");
    assert!(activation.when.clauses[0].value);
    assert_eq!(activation.when.clauses[1].variable, "charter_gaps_detected");
    assert!(activation.when.clauses[1].value);
}

#[test]
fn matching_front_matter_activation_copy_is_accepted() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();

    write_stage_with_front_matter(
        repo_root,
        "core/stages/00_base.md",
        r#"kind: stage
id: stage.00_base
version: 0.1.0
title: Base
description: base
activation:
  when:
    any:
      - variables.needs_project_context == true
"#,
    );
    write_file(
        &repo_root.join("core/pipelines/matching-front-matter-activation.yaml"),
        r#"---
kind: pipeline
id: pipeline.matching_front_matter_activation
version: 0.1.0
title: "Matching Front Matter Activation"
description: "header"
---
defaults:
  runner: codex-cli
  profile: python-uv
  enable_complexity: false
stages:
  - id: stage.00_base
    file: core/stages/00_base.md
    activation:
      when:
        any:
          - variables.needs_project_context == true
"#,
    );

    let definition = load_pipeline_definition(
        repo_root,
        "core/pipelines/matching-front-matter-activation.yaml",
    )
    .expect("matching activation should load");

    assert_eq!(definition.body.stages.len(), 1);
    assert!(definition.body.stages[0].activation.is_some());
}

#[test]
fn front_matter_activation_without_pipeline_activation_is_refused() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();

    write_stage_with_front_matter(
        repo_root,
        "core/stages/00_base.md",
        r#"kind: stage
id: stage.00_base
version: 0.1.0
title: Base
description: base
activation:
  when:
    any:
      - variables.needs_project_context == true
"#,
    );
    write_file(
        &repo_root.join("core/pipelines/front-matter-only-activation.yaml"),
        r#"---
kind: pipeline
id: pipeline.front_matter_only_activation
version: 0.1.0
title: "Front Matter Only Activation"
description: "header"
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

    let err = load_pipeline_definition(
        repo_root,
        "core/pipelines/front-matter-only-activation.yaml",
    )
    .expect_err("front matter only activation should refuse");

    match err {
        PipelineLoadError::Validation {
            error:
                PipelineValidationError::ActivationDrift {
                    stage_id,
                    file,
                    detail,
                },
            ..
        } => {
            assert_eq!(stage_id, "stage.00_base");
            assert_eq!(file, "core/stages/00_base.md");
            assert!(detail.contains("stage front matter declares"));
            assert!(detail.contains("pipeline YAML does not"));
        }
        other => panic!("expected activation-drift refusal, got {other:?}"),
    }
}

#[test]
fn semantic_activation_drift_is_refused() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();

    let cases = [
        (
            "value drift",
            r#"activation:
  when:
    any:
      - variables.needs_project_context == false
"#,
            r#"activation:
      when:
        any:
          - variables.needs_project_context == true
"#,
        ),
        (
            "variable drift",
            r#"activation:
  when:
    any:
      - variables.charter_gaps_detected == true
"#,
            r#"activation:
      when:
        any:
          - variables.needs_project_context == true
"#,
        ),
        (
            "operator drift",
            r#"activation:
  when:
    all:
      - variables.needs_project_context == true
"#,
            r#"activation:
      when:
        any:
          - variables.needs_project_context == true
"#,
        ),
    ];

    for (name, front_matter_activation, pipeline_activation) in cases {
        write_stage_with_front_matter(
            repo_root,
            "core/stages/00_base.md",
            &format!(
                "kind: stage\nid: stage.00_base\nversion: 0.1.0\ntitle: Base\ndescription: base\n{front_matter_activation}"
            ),
        );
        write_file(
            &repo_root.join("core/pipelines/activation-drift.yaml"),
            format!(
                r#"---
kind: pipeline
id: pipeline.activation_drift
version: 0.1.0
title: "Activation Drift"
description: "header"
---
defaults:
  runner: codex-cli
  profile: python-uv
  enable_complexity: false
stages:
  - id: stage.00_base
    file: core/stages/00_base.md
    {pipeline_activation}"#
            )
            .as_str(),
        );

        let err = load_pipeline_definition(repo_root, "core/pipelines/activation-drift.yaml")
            .expect_err(name);

        match err {
            PipelineLoadError::Validation {
                error: PipelineValidationError::ActivationDrift { stage_id, .. },
                ..
            } => assert_eq!(stage_id, "stage.00_base", "case={name}"),
            other => panic!("expected activation-drift refusal for {name}, got {other:?}"),
        }
    }
}

#[test]
fn reordered_semantically_identical_activation_is_accepted() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();

    write_stage_with_front_matter(
        repo_root,
        "core/stages/00_base.md",
        r#"kind: stage
id: stage.00_base
version: 0.1.0
title: Base
description: base
activation:
  when:
    any:
      - variables.charter_gaps_detected == true
      - variables.needs_project_context == false
"#,
    );
    write_file(
        &repo_root.join("core/pipelines/reordered-activation.yaml"),
        r#"---
kind: pipeline
id: pipeline.reordered_activation
version: 0.1.0
title: "Reordered Activation"
description: "header"
---
defaults:
  runner: codex-cli
  profile: python-uv
  enable_complexity: false
stages:
  - id: stage.00_base
    file: core/stages/00_base.md
    activation:
      when:
        any:
          - variables.needs_project_context == false
          - variables.charter_gaps_detected == true
"#,
    );

    load_pipeline_definition(repo_root, "core/pipelines/reordered-activation.yaml")
        .expect("reordered semantic activation should load");
}

#[test]
fn malformed_front_matter_activation_is_refused() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();

    write_stage_with_front_matter(
        repo_root,
        "core/stages/00_base.md",
        r#"kind: stage
id: stage.00_base
version: 0.1.0
title: Base
description: base
activation:
  when:
    any:
      - variables.needs_project_context != true
"#,
    );
    write_file(
        &repo_root.join("core/pipelines/malformed-front-matter-activation.yaml"),
        r#"---
kind: pipeline
id: pipeline.malformed_front_matter_activation
version: 0.1.0
title: "Malformed Front Matter Activation"
description: "header"
---
defaults:
  runner: codex-cli
  profile: python-uv
  enable_complexity: false
stages:
  - id: stage.00_base
    file: core/stages/00_base.md
    activation:
      when:
        any:
          - variables.needs_project_context == true
"#,
    );

    let err = load_pipeline_definition(
        repo_root,
        "core/pipelines/malformed-front-matter-activation.yaml",
    )
    .expect_err("malformed stage front matter activation should refuse");

    match err {
        PipelineLoadError::Validation {
            error:
                PipelineValidationError::InvalidStageFrontMatter {
                    stage_id,
                    file,
                    detail,
                },
            ..
        } => {
            assert_eq!(stage_id, "stage.00_base");
            assert_eq!(file, "core/stages/00_base.md");
            assert!(detail.contains("failed to parse stage front matter"));
            assert!(detail.contains("exactly one equality operator"));
        }
        other => panic!("expected invalid-stage-front-matter refusal, got {other:?}"),
    }
}

#[test]
fn declared_stage_order_is_preserved_for_core_pipelines() {
    let repo_root = repo_root();

    let cases = [
        (
            "core/pipelines/foundation_inputs.yaml",
            vec![
                "stage.00_base",
                "stage.04_charter_inputs",
                "stage.05_charter_synthesize",
                "stage.06_project_context_interview",
                "stage.07_foundation_pack",
                "stage.10_feature_spec",
            ],
        ),
        (
            "core/pipelines/release.yaml",
            vec!["stage.00_base", "stage.01_release_plan"],
        ),
        (
            "core/pipelines/sprint.yaml",
            vec!["stage.00_base", "stage.02_sprint_plan"],
        ),
    ];

    for (pipeline_path, expected_stage_ids) in cases {
        let definition =
            load_pipeline_definition(&repo_root, pipeline_path).expect("schema-compatible load");
        assert_eq!(definition.header.kind, "pipeline", "path={pipeline_path}");
        assert_eq!(
            definition
                .declared_stages()
                .iter()
                .map(|stage| stage.id.as_str())
                .collect::<Vec<_>>(),
            expected_stage_ids,
            "path={pipeline_path}"
        );
    }
}

#[test]
fn richer_root_pipeline_yaml_is_refused_as_out_of_scope_shape() {
    let repo_root = repo_root();

    let err = load_pipeline_definition(&repo_root, "pipeline.yaml").expect_err("root pipeline");

    match err {
        PipelineLoadError::UnsupportedPipelinePath { path, reason } => {
            assert_eq!(path, Path::new("pipeline.yaml"));
            assert!(reason.contains("core/pipelines/"));
        }
        other => panic!("expected unsupported path refusal, got {other:?}"),
    }
}

#[test]
fn wrong_document_count_is_refused() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    let pipeline_path = repo_root.join("core/pipelines/one-doc.yaml");
    write_file(
        &pipeline_path,
        r#"---
kind: pipeline
id: pipeline.one_doc
version: 0.1.0
title: "One Doc"
description: "missing body"
"#,
    );

    let err =
        load_pipeline_definition(repo_root, "core/pipelines/one-doc.yaml").expect_err("wrong docs");

    match err {
        PipelineLoadError::WrongDocumentCount { actual, .. } => assert_eq!(actual, 1),
        other => panic!("expected wrong-document-count refusal, got {other:?}"),
    }
}

#[test]
fn malformed_yaml_is_refused() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    let pipeline_path = repo_root.join("core/pipelines/malformed.yaml");
    write_file(
        &pipeline_path,
        "---\n\
kind: pipeline\n\
id: pipeline.bad\n\
version: 0.1.0\n\
title: \"Bad\"\n\
description: \"header\"\n\
---\n\
defaults:\n\
\trunner: codex-cli\n\
  profile: python-uv\n\
  enable_complexity: false\n\
stages:\n\
  - id: stage.00_base\n\
    file: core/stages/00_base.md\n",
    );

    let err = load_pipeline_definition(repo_root, "core/pipelines/malformed.yaml")
        .expect_err("malformed yaml");

    match err {
        PipelineLoadError::HeaderParse { .. } | PipelineLoadError::BodyParse { .. } => {}
        other => panic!("expected parse refusal, got {other:?}"),
    }
}

#[test]
fn empty_header_fields_are_refused() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    write_file(&repo_root.join("core/stages/00_base.md"), "base");
    let pipeline_path = repo_root.join("core/pipelines/empty-title.yaml");
    write_file(
        &pipeline_path,
        r#"---
kind: pipeline
id: pipeline.empty_title
version: 0.1.0
title: "   "
description: "header"
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

    let err = load_pipeline_definition(repo_root, "core/pipelines/empty-title.yaml")
        .expect_err("empty title");

    match err {
        PipelineLoadError::Validation {
            error: PipelineValidationError::EmptyField { field },
            ..
        } => assert_eq!(field, "title"),
        other => panic!("expected empty-field refusal, got {other:?}"),
    }
}

#[test]
fn path_like_pipeline_header_id_is_refused() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();

    write_stage_with_front_matter(
        repo_root,
        "core/stages/00_base.md",
        r#"kind: stage
id: stage.00_base
version: 0.1.0
title: Base
description: base
"#,
    );
    write_file(
        &repo_root.join("core/pipelines/bad-header-id.yaml"),
        r#"---
kind: pipeline
id: pipeline.bad/path
version: 0.1.0
title: "Bad Header Id"
description: "header"
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

    let err = load_pipeline_definition(repo_root, "core/pipelines/bad-header-id.yaml")
        .expect_err("path-like pipeline id should refuse");

    match err {
        PipelineLoadError::Validation {
            error:
                PipelineValidationError::InvalidCanonicalId {
                    field,
                    value,
                    reason,
                },
            ..
        } => {
            assert_eq!(field, "id");
            assert_eq!(value, "pipeline.bad/path");
            assert_eq!(
                reason,
                "canonical ids must not look like raw repo-relative paths"
            );
        }
        other => panic!("expected invalid-canonical-id refusal, got {other:?}"),
    }
}

#[test]
fn path_like_stage_id_is_refused() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();

    write_stage_with_front_matter(
        repo_root,
        "core/stages/00_base.md",
        r#"kind: stage
id: stage.bad/path
version: 0.1.0
title: Base
description: base
"#,
    );
    write_file(
        &repo_root.join("core/pipelines/bad-stage-id.yaml"),
        r#"---
kind: pipeline
id: pipeline.valid_stage_id_check
version: 0.1.0
title: "Bad Stage Id"
description: "header"
---
defaults:
  runner: codex-cli
  profile: python-uv
  enable_complexity: false
stages:
  - id: stage.bad/path
    file: core/stages/00_base.md
"#,
    );

    let err = load_pipeline_definition(repo_root, "core/pipelines/bad-stage-id.yaml")
        .expect_err("path-like stage id should refuse");

    match err {
        PipelineLoadError::Validation {
            error:
                PipelineValidationError::InvalidCanonicalId {
                    field,
                    value,
                    reason,
                },
            ..
        } => {
            assert_eq!(field, "stage.id");
            assert_eq!(value, "stage.bad/path");
            assert_eq!(
                reason,
                "canonical ids must not look like raw repo-relative paths"
            );
        }
        other => panic!("expected invalid-canonical-id refusal, got {other:?}"),
    }
}

#[test]
fn extension_shaped_pipeline_header_id_is_refused() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();

    write_stage_with_front_matter(
        repo_root,
        "core/stages/00_base.md",
        r#"kind: stage
id: stage.00_base
version: 0.1.0
title: Base
description: base
"#,
    );
    write_file(
        &repo_root.join("core/pipelines/bad-header-extension.yaml"),
        r#"---
kind: pipeline
id: pipeline.bad.yaml
version: 0.1.0
title: "Bad Header Extension"
description: "header"
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

    let err = load_pipeline_definition(repo_root, "core/pipelines/bad-header-extension.yaml")
        .expect_err("extension-shaped pipeline id should refuse");

    match err {
        PipelineLoadError::Validation {
            error:
                PipelineValidationError::InvalidCanonicalId {
                    field,
                    value,
                    reason,
                },
            ..
        } => {
            assert_eq!(field, "id");
            assert_eq!(value, "pipeline.bad.yaml");
            assert_eq!(
                reason,
                "canonical ids must not look like raw repo-relative paths"
            );
        }
        other => panic!("expected invalid-canonical-id refusal, got {other:?}"),
    }
}

#[test]
fn unknown_fields_are_refused() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    let pipeline_path = repo_root.join("core/pipelines/unknown-field.yaml");
    write_file(
        &pipeline_path,
        r#"---
kind: pipeline
id: pipeline.unknown_field
version: 0.1.0
title: "Unknown Field"
description: "header"
---
defaults:
  runner: codex-cli
  profile: python-uv
  enable_complexity: false
stages:
  - id: stage.00_base
    file: core/stages/00_base.md
    outputs:
      - artifacts/charter/CHARTER.md
"#,
    );

    let err = load_pipeline_definition(repo_root, "core/pipelines/unknown-field.yaml")
        .expect_err("unknown field");

    match err {
        PipelineLoadError::BodyParse { .. } => {}
        other => panic!("expected body parse refusal, got {other:?}"),
    }
}

#[test]
fn unsupported_header_kind_is_refused() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    let pipeline_path = repo_root.join("core/pipelines/wrong-kind.yaml");
    write_file(
        &pipeline_path,
        r#"---
kind: schema
id: pipeline.wrong_kind
version: 0.1.0
title: "Wrong Kind"
description: "header"
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

    let err = load_pipeline_definition(repo_root, "core/pipelines/wrong-kind.yaml")
        .expect_err("wrong header kind");

    match err {
        PipelineLoadError::Validation {
            error: PipelineValidationError::UnsupportedKind { actual },
            ..
        } => assert_eq!(actual, "schema"),
        other => panic!("expected unsupported-kind refusal, got {other:?}"),
    }
}

#[test]
fn duplicate_stage_ids_are_refused() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    write_file(&repo_root.join("core/stages/00_base.md"), "base");
    write_file(
        &repo_root.join("core/stages/05_charter_interview.md"),
        "charter",
    );
    let pipeline_path = repo_root.join("core/pipelines/duplicate-stage.yaml");
    write_file(
        &pipeline_path,
        r#"---
kind: pipeline
id: pipeline.duplicate_stage
version: 0.1.0
title: "Duplicate Stage"
description: "header"
---
defaults:
  runner: codex-cli
  profile: python-uv
  enable_complexity: false
stages:
  - id: stage.00_base
    file: core/stages/00_base.md
  - id: stage.00_base
    file: core/stages/05_charter_interview.md
"#,
    );

    let err = load_pipeline_definition(repo_root, "core/pipelines/duplicate-stage.yaml")
        .expect_err("duplicate stage");

    match err {
        PipelineLoadError::Validation {
            error: PipelineValidationError::DuplicateStageId { stage_id },
            ..
        } => assert_eq!(stage_id, "stage.00_base"),
        other => panic!("expected duplicate-stage refusal, got {other:?}"),
    }
}

#[test]
fn unsupported_activation_syntax_is_refused() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    let pipeline_path = repo_root.join("core/pipelines/bad-activation.yaml");
    write_file(
        &pipeline_path,
        r#"---
kind: pipeline
id: pipeline.bad_activation
version: 0.1.0
title: "Bad Activation"
description: "header"
---
defaults:
  runner: codex-cli
  profile: python-uv
  enable_complexity: false
stages:
  - id: stage.00_base
    file: core/stages/00_base.md
    activation:
      when:
        any:
          - variables.needs_project_context != true
"#,
    );

    let err = load_pipeline_definition(repo_root, "core/pipelines/bad-activation.yaml")
        .expect_err("bad activation");

    match err {
        PipelineLoadError::BodyParse { .. } => {}
        other => panic!("expected activation parse refusal, got {other:?}"),
    }
}

#[test]
fn unsupported_activation_value_type_is_refused() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    let pipeline_path = repo_root.join("core/pipelines/bad-activation-value.yaml");
    write_file(
        &pipeline_path,
        r#"---
kind: pipeline
id: pipeline.bad_activation_value
version: 0.1.0
title: "Bad Activation Value"
description: "header"
---
defaults:
  runner: codex-cli
  profile: python-uv
  enable_complexity: false
stages:
  - id: stage.00_base
    file: core/stages/00_base.md
    activation:
      when:
        any:
          - variables.target_env == "production"
"#,
    );

    let err = load_pipeline_definition(repo_root, "core/pipelines/bad-activation-value.yaml")
        .expect_err("bad activation value");

    match err {
        PipelineLoadError::BodyParse { source, .. } => {
            assert!(
                source.to_string().contains(
                    "reduced v1 supports only boolean activation values (`true` or `false`)"
                ),
                "unexpected parse error: {source}"
            );
        }
        other => panic!("expected activation-value refusal, got {other:?}"),
    }
}

#[test]
fn numeric_activation_value_is_refused() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    let pipeline_path = repo_root.join("core/pipelines/numeric-activation-value.yaml");
    write_file(
        &pipeline_path,
        r#"---
kind: pipeline
id: pipeline.numeric_activation_value
version: 0.1.0
title: "Numeric Activation Value"
description: "header"
---
defaults:
  runner: codex-cli
  profile: python-uv
  enable_complexity: false
stages:
  - id: stage.00_base
    file: core/stages/00_base.md
    activation:
      when:
        any:
          - variables.max_attempts == 3
"#,
    );

    let err = load_pipeline_definition(repo_root, "core/pipelines/numeric-activation-value.yaml")
        .expect_err("numeric activation value");

    match err {
        PipelineLoadError::BodyParse { source, .. } => {
            assert!(
                source.to_string().contains(
                    "reduced v1 supports only boolean activation values (`true` or `false`)"
                ),
                "unexpected parse error: {source}"
            );
        }
        other => panic!("expected activation-value refusal, got {other:?}"),
    }
}

#[test]
fn out_of_root_stage_paths_are_refused() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    let pipeline_path = repo_root.join("core/pipelines/out-of-root.yaml");
    write_file(
        &pipeline_path,
        r#"---
kind: pipeline
id: pipeline.out_of_root
version: 0.1.0
title: "Out Of Root"
description: "header"
---
defaults:
  runner: codex-cli
  profile: python-uv
  enable_complexity: false
stages:
  - id: stage.00_base
    file: ../outside.md
"#,
    );

    let err = load_pipeline_definition(repo_root, "core/pipelines/out-of-root.yaml")
        .expect_err("out-of-root stage path");

    match err {
        PipelineLoadError::Validation {
            error: PipelineValidationError::StageFileOutsideRepoRoot { stage_id, file },
            ..
        } => {
            assert_eq!(stage_id, "stage.00_base");
            assert_eq!(file, "../outside.md");
        }
        other => panic!("expected stage-path refusal, got {other:?}"),
    }
}

#[test]
fn missing_stage_files_are_refused() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    let pipeline_path = repo_root.join("core/pipelines/missing-stage.yaml");
    write_file(
        &pipeline_path,
        r#"---
kind: pipeline
id: pipeline.missing_stage
version: 0.1.0
title: "Missing Stage"
description: "header"
---
defaults:
  runner: codex-cli
  profile: python-uv
  enable_complexity: false
stages:
  - id: stage.00_base
    file: core/stages/does_not_exist.md
"#,
    );

    let err = load_pipeline_definition(repo_root, "core/pipelines/missing-stage.yaml")
        .expect_err("missing stage file");

    match err {
        PipelineLoadError::Validation {
            error:
                PipelineValidationError::InvalidStageFile {
                    stage_id,
                    file,
                    reason: StageFileValidationError::Missing,
                },
            ..
        } => {
            assert_eq!(stage_id, "stage.00_base");
            assert_eq!(file, "core/stages/does_not_exist.md");
        }
        other => panic!("expected missing-stage refusal, got {other:?}"),
    }
}

#[test]
fn repo_local_stage_paths_outside_stage_directory_are_refused() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    write_file(&repo_root.join("README.md"), "not a stage");
    let pipeline_path = repo_root.join("core/pipelines/outside-stage-directory.yaml");
    write_file(
        &pipeline_path,
        r#"---
kind: pipeline
id: pipeline.outside_surface
version: 0.1.0
title: "Outside Surface"
description: "header"
---
defaults:
  runner: codex-cli
  profile: python-uv
  enable_complexity: false
stages:
  - id: stage.00_base
    file: README.md
"#,
    );

    let err = load_pipeline_definition(repo_root, "core/pipelines/outside-stage-directory.yaml")
        .expect_err("outside stage directory");

    match err {
        PipelineLoadError::Validation {
            error:
                PipelineValidationError::InvalidStageFile {
                    stage_id,
                    file,
                    reason: StageFileValidationError::OutsideStageDirectory { stage_root },
                },
            ..
        } => {
            assert_eq!(stage_id, "stage.00_base");
            assert_eq!(file, "README.md");
            assert_eq!(stage_root, "core/stages");
        }
        other => panic!("expected stage-directory refusal, got {other:?}"),
    }
}

#[test]
fn stage_paths_with_wrong_extension_are_refused() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    write_file(
        &repo_root.join("core/stages/not_markdown.txt"),
        "not markdown",
    );
    let pipeline_path = repo_root.join("core/pipelines/wrong-extension-stage.yaml");
    write_file(
        &pipeline_path,
        r#"---
kind: pipeline
id: pipeline.wrong_extension_stage
version: 0.1.0
title: "Wrong Extension Stage"
description: "header"
---
defaults:
  runner: codex-cli
  profile: python-uv
  enable_complexity: false
stages:
  - id: stage.00_base
    file: core/stages/not_markdown.txt
"#,
    );

    let err = load_pipeline_definition(repo_root, "core/pipelines/wrong-extension-stage.yaml")
        .expect_err("wrong extension stage path");

    match err {
        PipelineLoadError::Validation {
            error:
                PipelineValidationError::InvalidStageFile {
                    stage_id,
                    file,
                    reason: StageFileValidationError::WrongExtension,
                },
            ..
        } => {
            assert_eq!(stage_id, "stage.00_base");
            assert_eq!(file, "core/stages/not_markdown.txt");
        }
        other => panic!("expected wrong-extension refusal, got {other:?}"),
    }
}

#[test]
fn non_regular_stage_paths_are_refused() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    let stage_dir = repo_root.join("core/stages/not_a_file.md");
    std::fs::create_dir_all(&stage_dir).expect("mkdirs");
    let pipeline_path = repo_root.join("core/pipelines/non-regular-stage.yaml");
    write_file(
        &pipeline_path,
        r#"---
kind: pipeline
id: pipeline.non_regular_stage
version: 0.1.0
title: "Non Regular Stage"
description: "header"
---
defaults:
  runner: codex-cli
  profile: python-uv
  enable_complexity: false
stages:
  - id: stage.00_base
    file: core/stages/not_a_file.md
"#,
    );

    let err = load_pipeline_definition(repo_root, "core/pipelines/non-regular-stage.yaml")
        .expect_err("non-regular stage path");

    match err {
        PipelineLoadError::Validation {
            error:
                PipelineValidationError::InvalidStageFile {
                    stage_id,
                    file,
                    reason: StageFileValidationError::NotRegularFile,
                },
            ..
        } => {
            assert_eq!(stage_id, "stage.00_base");
            assert_eq!(file, "core/stages/not_a_file.md");
        }
        other => panic!("expected non-regular-stage refusal, got {other:?}"),
    }
}

#[test]
fn activation_all_operator_parses_multiple_boolean_clauses() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    write_file(&repo_root.join("core/stages/00_base.md"), "base");
    let pipeline_path = repo_root.join("core/pipelines/all-activation.yaml");
    write_file(
        &pipeline_path,
        r#"---
kind: pipeline
id: pipeline.all_activation
version: 0.1.0
title: "All Activation"
description: "header"
---
defaults:
  runner: codex-cli
  profile: python-uv
  enable_complexity: false
stages:
  - id: stage.00_base
    file: core/stages/00_base.md
    activation:
      when:
        all:
          - variables.needs_project_context == true
          - variables.charter_gaps_detected == false
"#,
    );

    let definition =
        load_pipeline_definition(repo_root, "core/pipelines/all-activation.yaml").expect("load");

    let activation = definition.body.stages[0]
        .activation
        .as_ref()
        .expect("activation");
    assert_eq!(activation.when.operator, ActivationOperator::All);
    assert_eq!(activation.when.clauses.len(), 2);
    assert_eq!(activation.when.clauses[0].variable, "needs_project_context");
    assert!(activation.when.clauses[0].value);
    assert_eq!(activation.when.clauses[1].variable, "charter_gaps_detected");
    assert!(!activation.when.clauses[1].value);
}

#[cfg(unix)]
#[test]
fn symlinked_stage_file_is_classified_as_non_regular_file() {
    use std::os::unix::fs::symlink;

    let dir = tempfile::tempdir().expect("tempdir");
    let external = tempfile::tempdir().expect("external tempdir");
    let repo_root = dir.path();
    write_file(&external.path().join("00_base.md"), "# external stage");
    std::fs::create_dir_all(repo_root.join("core/stages")).expect("stage dir");
    symlink(
        external.path().join("00_base.md"),
        repo_root.join("core/stages/00_base.md"),
    )
    .expect("symlink stage");
    write_file(
        &repo_root.join("core/pipelines/symlinked-stage.yaml"),
        r#"---
kind: pipeline
id: pipeline.symlinked_stage
version: 0.1.0
title: "Symlinked Stage"
description: "header"
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

    let err = load_pipeline_definition(repo_root, "core/pipelines/symlinked-stage.yaml")
        .expect_err("symlinked stage path");

    match err {
        PipelineLoadError::Validation {
            error:
                PipelineValidationError::InvalidStageFile {
                    stage_id,
                    file,
                    reason: StageFileValidationError::NotRegularFile,
                },
            ..
        } => {
            assert_eq!(stage_id, "stage.00_base");
            assert_eq!(file, "core/stages/00_base.md");
        }
        other => panic!("expected symlinked-stage refusal, got {other:?}"),
    }
}

#[test]
fn pipeline_path_must_stay_repo_relative() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();

    let err = load_pipeline_definition(repo_root, "../core/pipelines/outside.yaml")
        .expect_err("pipeline path should be rejected");

    match err {
        PipelineLoadError::UnsupportedPipelinePath { reason, .. } => {
            assert_eq!(reason, "path must not escape the repo root");
        }
        other => panic!("expected unsupported-pipeline-path refusal, got {other:?}"),
    }
}

#[test]
fn missing_pipeline_file_is_reported_as_read_failure() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();

    let err = load_pipeline_definition(repo_root, "core/pipelines/missing.yaml")
        .expect_err("missing pipeline file");

    match err {
        PipelineLoadError::ReadFailure { path, .. } => {
            assert_eq!(path, repo_root.join("core/pipelines/missing.yaml"));
        }
        other => panic!("expected read-failure refusal, got {other:?}"),
    }
}

#[test]
fn extra_yaml_documents_are_refused() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    write_file(&repo_root.join("core/stages/00_base.md"), "base");
    let pipeline_path = repo_root.join("core/pipelines/extra-doc.yaml");
    write_file(
        &pipeline_path,
        r#"---
kind: pipeline
id: pipeline.extra_doc
version: 0.1.0
title: "Extra Doc"
description: "header"
---
defaults:
  runner: codex-cli
  profile: python-uv
  enable_complexity: false
stages:
  - id: stage.00_base
    file: core/stages/00_base.md
---
extra: true
"#,
    );

    let err = load_pipeline_definition(repo_root, "core/pipelines/extra-doc.yaml")
        .expect_err("third document should be refused");

    match err {
        PipelineLoadError::WrongDocumentCount { actual, .. } => assert_eq!(actual, 3),
        other => panic!("expected wrong-document-count refusal, got {other:?}"),
    }
}

#[test]
fn empty_stage_list_is_refused() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    let pipeline_path = repo_root.join("core/pipelines/empty-stages.yaml");
    write_file(
        &pipeline_path,
        r#"---
kind: pipeline
id: pipeline.empty_stages
version: 0.1.0
title: "Empty Stages"
description: "header"
---
defaults:
  runner: codex-cli
  profile: python-uv
  enable_complexity: false
stages: []
"#,
    );

    let err = load_pipeline_definition(repo_root, "core/pipelines/empty-stages.yaml")
        .expect_err("empty stages");

    match err {
        PipelineLoadError::Validation {
            error: PipelineValidationError::EmptyStages,
            ..
        } => {}
        other => panic!("expected empty-stages refusal, got {other:?}"),
    }
}

#[test]
fn empty_sets_list_is_refused() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    write_file(&repo_root.join("core/stages/00_base.md"), "base");
    let pipeline_path = repo_root.join("core/pipelines/empty-sets.yaml");
    write_file(
        &pipeline_path,
        r#"---
kind: pipeline
id: pipeline.empty_sets
version: 0.1.0
title: "Empty Sets"
description: "header"
---
defaults:
  runner: codex-cli
  profile: python-uv
  enable_complexity: false
stages:
  - id: stage.00_base
    file: core/stages/00_base.md
    sets: []
"#,
    );

    let err = load_pipeline_definition(repo_root, "core/pipelines/empty-sets.yaml")
        .expect_err("empty sets");

    match err {
        PipelineLoadError::Validation {
            error: PipelineValidationError::EmptySetsList { stage_id },
            ..
        } => assert_eq!(stage_id, "stage.00_base"),
        other => panic!("expected empty-sets refusal, got {other:?}"),
    }
}

#[test]
fn blank_set_variable_is_refused() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    write_file(&repo_root.join("core/stages/00_base.md"), "base");
    let pipeline_path = repo_root.join("core/pipelines/blank-set-variable.yaml");
    write_file(
        &pipeline_path,
        r#"---
kind: pipeline
id: pipeline.blank_set_variable
version: 0.1.0
title: "Blank Set Variable"
description: "header"
---
defaults:
  runner: codex-cli
  profile: python-uv
  enable_complexity: false
stages:
  - id: stage.00_base
    file: core/stages/00_base.md
    sets:
      - "   "
"#,
    );

    let err = load_pipeline_definition(repo_root, "core/pipelines/blank-set-variable.yaml")
        .expect_err("blank set variable");

    match err {
        PipelineLoadError::Validation {
            error: PipelineValidationError::EmptySetVariable { stage_id, index },
            ..
        } => {
            assert_eq!(stage_id, "stage.00_base");
            assert_eq!(index, 0);
        }
        other => panic!("expected empty-set-variable refusal, got {other:?}"),
    }
}

#[test]
fn invalid_set_variable_name_is_refused() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    write_file(&repo_root.join("core/stages/00_base.md"), "base");
    let pipeline_path = repo_root.join("core/pipelines/invalid-set-variable.yaml");
    write_file(
        &pipeline_path,
        r#"---
kind: pipeline
id: pipeline.invalid_set_variable
version: 0.1.0
title: "Invalid Set Variable"
description: "header"
---
defaults:
  runner: codex-cli
  profile: python-uv
  enable_complexity: false
stages:
  - id: stage.00_base
    file: core/stages/00_base.md
    sets:
      - 9bad
"#,
    );

    let err = load_pipeline_definition(repo_root, "core/pipelines/invalid-set-variable.yaml")
        .expect_err("invalid set variable");

    match err {
        PipelineLoadError::Validation {
            error:
                PipelineValidationError::InvalidSetVariable {
                    stage_id, variable, ..
                },
            ..
        } => {
            assert_eq!(stage_id, "stage.00_base");
            assert_eq!(variable, "9bad");
        }
        other => panic!("expected invalid-set-variable refusal, got {other:?}"),
    }
}

#[test]
fn empty_activation_clause_list_is_refused() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    write_file(&repo_root.join("core/stages/00_base.md"), "base");
    let pipeline_path = repo_root.join("core/pipelines/empty-activation-list.yaml");
    write_file(
        &pipeline_path,
        r#"---
kind: pipeline
id: pipeline.empty_activation_list
version: 0.1.0
title: "Empty Activation List"
description: "header"
---
defaults:
  runner: codex-cli
  profile: python-uv
  enable_complexity: false
stages:
  - id: stage.00_base
    file: core/stages/00_base.md
    activation:
      when:
        any: []
"#,
    );

    let err = load_pipeline_definition(repo_root, "core/pipelines/empty-activation-list.yaml")
        .expect_err("empty activation list");

    match err {
        PipelineLoadError::Validation {
            error:
                PipelineValidationError::InvalidActivation {
                    stage_id,
                    reason:
                        ActivationValidationError::EmptyConditionList {
                            operator: ActivationOperator::Any,
                        },
                },
            ..
        } => assert_eq!(stage_id, "stage.00_base"),
        other => panic!("expected empty-activation-list refusal, got {other:?}"),
    }
}
