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

    let renderer_golden = fs::read_to_string(
        workspace_root().join(
            "crates/engine/tests/fixtures/hcm_2_4_work_specification/artifacts/feature_spec/FEATURE_SPEC.md",
        ),
    )
    .expect("renderer golden");
    for case in ["happy_path", "skip_path"] {
        let yaml_path = root
            .join("model_outputs")
            .join(case)
            .join("stage_10_feature_spec.md");
        let bytes = fs::read(&yaml_path).expect("Work Specification model output");
        let content = parse_canonical_yaml(&bytes).expect("duplicate-safe Work Specification YAML");
        assert_eq!(
            canonical_yaml_bytes(&content).expect("canonical Work Specification YAML"),
            bytes
        );
        let object = content.as_object().expect("Work Specification object");
        assert_eq!(object.len(), 8);
        assert_eq!(object["schema_id"], "handbook.artifact.work-specification");
        assert_eq!(object["schema_version"], "1.0");
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
        assert_eq!(expected, renderer_golden);
    }
}
