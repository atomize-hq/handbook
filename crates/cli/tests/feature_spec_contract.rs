use handbook_engine::canonical_yaml::{canonical_yaml_bytes, parse_canonical_yaml};
use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    let start = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    start
        .ancestors()
        .find(|ancestor| {
            fs::read_to_string(ancestor.join("Cargo.toml"))
                .is_ok_and(|contents| contents.contains("[workspace]"))
        })
        .expect("workspace root")
        .to_path_buf()
}

fn demo_root() -> PathBuf {
    workspace_root().join("tests/fixtures/foundation_flow_demo")
}

#[test]
fn foundation_flow_demo_feature_specs_match_directive_and_template_contract() {
    let root = demo_root();
    let directive = fs::read_to_string(
        workspace_root().join("core/library/feature_spec/feature_spec_architect_directive.md"),
    )
    .expect("directive");
    let template =
        fs::read_to_string(workspace_root().join("core/library/feature_spec/FEATURE_SPEC.md.tmpl"))
            .expect("template");
    assert!(directive.contains("exactly `schema_id`, `schema_version`, `record_id`"));
    assert!(directive.contains("Do not emit Markdown"));
    for field in [
        "schema_id",
        "schema_version",
        "record_id",
        "objective",
        "scope",
        "non_goals",
        "acceptance_criteria",
        "status",
    ] {
        assert!(template
            .lines()
            .any(|line| line.starts_with(&format!("{field}:"))));
    }

    let mut first_model_output = None;
    let mut first_generated_view = None;
    for case in ["happy_path", "skip_path"] {
        let yaml_path = root
            .join("model_outputs")
            .join(case)
            .join("stage_10_feature_spec.md");
        let bytes = fs::read(&yaml_path).expect("Work Specification model output");
        if let Some(first) = &first_model_output {
            assert_eq!(
                &bytes, first,
                "happy and skip paths must use the same Work Specification input"
            );
        } else {
            first_model_output = Some(bytes.clone());
        }
        let content = parse_canonical_yaml(&bytes).expect("duplicate-safe Work Specification YAML");
        assert_eq!(
            canonical_yaml_bytes(&content).expect("canonical Work Specification YAML"),
            bytes
        );
        let object = content.as_object().expect("Work Specification object");
        assert_eq!(object.len(), 8);
        assert_eq!(object["schema_id"], "handbook.artifact.work-specification");
        assert_eq!(object["schema_version"], "1.0");
        let objective = object["objective"].as_str().expect("objective");
        for required_objective_clause in [
            "using staged external model output, canonical Work Specification YAML capture, and a deterministic Markdown view",
            "A credible alternative is to retain Markdown as Stage 10 authority",
            "but forfeits schema-selected validation, stable canonical identity, and byte-exact handoff provenance",
        ] {
            assert!(
                objective.contains(required_objective_clause),
                "objective must preserve the directive-required approach, alternative, and trade-off clause `{required_objective_clause}`"
            );
        }
        assert!(!object["scope"].as_array().expect("scope").is_empty());
        assert!(!object["acceptance_criteria"]
            .as_array()
            .expect("acceptance criteria")
            .is_empty());

        let expected = fs::read_to_string(
            root.join("expected")
                .join(case)
                .join("final_feature_spec.md"),
        )
        .expect("generated Markdown view");
        if let Some(first) = &first_generated_view {
            assert_eq!(
                &expected, first,
                "happy and skip paths must expect the same generated Markdown view"
            );
        } else {
            first_generated_view = Some(expected);
        }
    }
}
