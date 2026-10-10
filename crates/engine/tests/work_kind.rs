use handbook_engine::work_state::*;

fn definition() -> WorkKindDefinitionV1 {
    WorkKindDefinitionV1 {
        schema_ref: WORK_KIND_DEFINITION_SCHEMA_V1.into(),
        reference: "example.work-kind.planning@1.0.0".into(),
        template: WorkCompositionClauseV1 {
            descriptor: WorkDescriptorV1::planning_output(),
            label_max_bytes: 256,
            work_question_max_bytes: 512,
            allow_omitted_display_level: false,
            display_levels: vec!["planning".into()],
            role_permission: WorkRolePermissionV1::PlanningOutput {
                question_max_bytes: 512,
            },
        },
    }
}

#[test]
fn whole_definition_identity_is_exact_closed_and_bounded() {
    let original = definition();
    let bytes = original.canonical_json_bytes().unwrap();
    assert_eq!(
        WorkKindDefinitionV1::from_json_bytes(&bytes).unwrap(),
        original
    );
    let fingerprint = original.fingerprint().unwrap();
    let mut different = original.clone();
    different.template.label_max_bytes -= 1;
    assert_ne!(different.fingerprint().unwrap(), fingerprint);
    different = original.clone();
    different.reference = "example.work-kind.planning@2.0.0".into();
    assert_ne!(different.fingerprint().unwrap(), fingerprint);
    for reference in [
        "example.kind@latest",
        "example.kind@^1.0.0",
        "Example.kind@1.0.0",
        "example..kind@1.0.0",
        "example.kind@01.0.0",
        "example.kind-@1.0.0",
        "kind@1.0.0",
    ] {
        different = original.clone();
        different.reference = reference.into();
        assert!(different.canonical_json_bytes().is_err(), "{reference}");
    }
    for bound in [0, 4097, u32::MAX] {
        different = original.clone();
        different.template.label_max_bytes = bound;
        assert!(different.canonical_json_bytes().is_err());
    }
    for field in ["hooks", "assessment", "default", "relations", "latest"] {
        let mut wire = serde_json::to_value(&original).unwrap();
        wire[field] = serde_json::json!(true);
        assert!(
            WorkKindDefinitionV1::from_json_bytes(&serde_json::to_vec(&wire).unwrap()).is_err()
        );
    }
    let raw = String::from_utf8(bytes.clone()).unwrap();
    let duplicate = raw.replacen('{', "{\"schema_ref\":\"injected\",", 1);
    assert!(WorkKindDefinitionV1::from_json_bytes(duplicate.as_bytes()).is_err());
    assert!(WorkKindDefinitionV1::from_json_bytes(&[bytes, b" {}".to_vec()].concat()).is_err());
    assert!(WorkKindDefinitionV1::from_json_bytes(&vec![b' '; 1024 * 1024 + 1]).is_err());
}

#[test]
fn definition_fixes_supported_descriptor_and_complete_role_bounds() {
    let original = definition();
    for change in 0..8 {
        let mut value = original.clone();
        match change {
            0 => value.template.descriptor.compatibility_version = "2".into(),
            1 => value.template.descriptor.initial_state = "completed".into(),
            2 => value.template.descriptor.completion_support = "assessed".into(),
            3 => value
                .template
                .descriptor
                .relation_support
                .push("parent".into()),
            4 => value
                .template
                .descriptor
                .role_schema
                .push("assessment_result".into()),
            5 => value.template.role_permission = WorkRolePermissionV1::Ordinary {},
            6 => value.template.display_levels.push("planning".into()),
            _ => value.template.display_levels.clear(),
        }
        assert!(value.canonical_json_bytes().is_err(), "change {change}");
    }
    let mut value = original;
    value.template.display_levels = (0..12_000)
        .map(|i| format!("{}.{i}", "x".repeat(120)))
        .collect();
    assert!(value.canonical_json_bytes().is_err());
}
