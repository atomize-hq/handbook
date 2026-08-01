use handbook_engine::{
    DefinitionFingerprint, RegistryLoadErrorKind, VocabularyDefinition, VocabularyResolution,
};
use std::path::Path;
const PATH: &str = "definitions/vocabularies/handbook.vocabulary.shipped-root/1.0.0.yaml";
const BYTES: &[u8] =
    include_bytes!("../definitions/vocabularies/handbook.vocabulary.shipped-root/1.0.0.yaml");
const NONEMPTY_PATH: &str = "definitions/vocabularies/example.vocabulary.hcm-3-1/1.0.0.yaml";
const NONEMPTY_BYTES: &[u8] = include_bytes!(
    "fixtures/hcm_3_1_vocabulary_resolution/definitions/vocabularies/example.vocabulary.hcm-3-1/1.0.0.yaml"
);
fn write(p: &Path, b: &[u8]) {
    if let Some(x) = p.parent() {
        std::fs::create_dir_all(x).unwrap();
    }
    std::fs::write(p, b).unwrap();
}

fn nonempty_value() -> serde_json::Value {
    serde_yaml_bw::from_slice(NONEMPTY_BYTES).unwrap()
}

fn close_fingerprint(value: &mut serde_json::Value) {
    let object = value.as_object_mut().unwrap();
    object.remove("vocabulary_fingerprint");
    let fingerprint = DefinitionFingerprint::from_json_value(value).unwrap();
    value["vocabulary_fingerprint"] = serde_json::json!(fingerprint.as_str());
}

fn load_value(
    value: &serde_json::Value,
) -> Result<VocabularyDefinition, handbook_engine::RegistryLoadError> {
    let repo = tempfile::tempdir().unwrap();
    write(
        &repo.path().join(NONEMPTY_PATH),
        serde_yaml_bw::to_string(value).unwrap().as_bytes(),
    );
    VocabularyDefinition::load(repo.path(), NONEMPTY_PATH)
}
#[test]
fn exact_empty_mapping_vocabulary_loads() {
    let r = tempfile::tempdir().unwrap();
    write(&r.path().join(PATH), BYTES);
    let vocabulary = VocabularyDefinition::load(r.path(), PATH).unwrap();
    assert_eq!(
        vocabulary.exact_ref().as_str(),
        "handbook.vocabulary.shipped-root@1.0.0"
    );
    assert_eq!(
        vocabulary.stable_role_registry_ref().as_str(),
        "handbook.roles.core@1.1.0"
    );
    assert_eq!(
        vocabulary.stable_role_registry_fingerprint().as_str(),
        "sha256:0c85b1b53786e7980c4fd0d7975cd9cde1a3eae2bc8daceb23be1a1731263029"
    );
}
#[test]
fn changed_mapping_or_role_pair_refuses() {
    for field in ["labels", "stable_role_registry"] {
        let r = tempfile::tempdir().unwrap();
        let mut v: serde_json::Value = serde_yaml_bw::from_slice(BYTES).unwrap();
        if field == "labels" {
            v[field] = serde_json::json!({"project_context":"Renamed"});
        } else {
            v[field]["ref"] = "handbook.roles.core@1.0.0".into();
        }
        write(
            &r.path().join(PATH),
            serde_yaml_bw::to_string(&v).unwrap().as_bytes(),
        );
        assert!(VocabularyDefinition::load(r.path(), PATH).is_err());
    }
}

#[test]
fn complete_vocabulary_record_and_nested_selection_matrix_refuses() {
    for case in [
        "missing",
        "extra",
        "wrong_type",
        "unsupported_version",
        "fingerprint",
        "selection_missing",
        "selection_extra",
        "selection_wrong_type",
        "labels_nonempty",
        "aliases_nonempty",
        "absorptions_nonempty",
        "extensions_nonempty",
    ] {
        let r = tempfile::tempdir().unwrap();
        let mut value: serde_json::Value = serde_yaml_bw::from_slice(BYTES).unwrap();
        let expected = match case {
            "missing" => {
                value.as_object_mut().unwrap().remove("vocabulary_id");
                RegistryLoadErrorKind::SyntaxError
            }
            "extra" => {
                value["unexpected"] = serde_json::json!(true);
                RegistryLoadErrorKind::UnknownField
            }
            "wrong_type" => {
                value["labels"] = serde_json::json!([]);
                RegistryLoadErrorKind::SyntaxError
            }
            "unsupported_version" => {
                value["schema_version"] = serde_json::json!("2.0");
                RegistryLoadErrorKind::UnsupportedRecord
            }
            "fingerprint" => {
                value["vocabulary_fingerprint"] = serde_json::json!(
                    "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
                );
                RegistryLoadErrorKind::FingerprintMismatch
            }
            "selection_missing" => {
                value["stable_role_registry"]
                    .as_object_mut()
                    .unwrap()
                    .remove("ref");
                RegistryLoadErrorKind::SyntaxError
            }
            "selection_extra" => {
                value["stable_role_registry"]["unexpected"] = serde_json::json!(true);
                RegistryLoadErrorKind::UnknownField
            }
            "selection_wrong_type" => {
                value["stable_role_registry"]["fingerprint"] = serde_json::json!(true);
                RegistryLoadErrorKind::SyntaxError
            }
            "labels_nonempty" => {
                value["labels"]["project_context"] = serde_json::json!("Renamed");
                RegistryLoadErrorKind::FingerprintMismatch
            }
            "aliases_nonempty" => {
                value["aliases"]["project"] = serde_json::json!("project_context");
                RegistryLoadErrorKind::SyntaxError
            }
            "absorptions_nonempty" => {
                value["absorptions"]
                    .as_array_mut()
                    .unwrap()
                    .push(serde_json::json!("project_context"));
                RegistryLoadErrorKind::SyntaxError
            }
            _ => {
                value["extensions"]["future"] = serde_json::json!(true);
                RegistryLoadErrorKind::UnsupportedRecord
            }
        };
        write(
            &r.path().join(PATH),
            serde_yaml_bw::to_string(&value).unwrap().as_bytes(),
        );
        let error = VocabularyDefinition::load(r.path(), PATH).unwrap_err();
        assert_eq!(error.kind(), expected, "{case}");
    }
}

#[test]
fn nonempty_vocabulary_resolves_labels_aliases_and_absorptions_without_machine_conflation() {
    let repo = tempfile::tempdir().unwrap();
    write(&repo.path().join(NONEMPTY_PATH), NONEMPTY_BYTES);
    let vocabulary = VocabularyDefinition::load(repo.path(), NONEMPTY_PATH).unwrap();

    assert_eq!(
        vocabulary.vocabulary_fingerprint().as_str(),
        "sha256:0a28353460ce60a0fc53ba5e99ea2ec753abf94638489fe1eebd69b317d90ec4"
    );
    assert_eq!(vocabulary.explicit_label("delivery_unit"), Some("Feature"));
    assert_eq!(
        vocabulary.display_label("atomic_action"),
        Some("Atomic Action")
    );
    assert_eq!(
        vocabulary
            .resolve_typed_role("implementation_unit")
            .unwrap()
            .role_id(),
        "implementation_unit"
    );
    assert_eq!(
        vocabulary.resolve_untyped("FEATURE"),
        VocabularyResolution::Unique("delivery_unit".to_string())
    );
    assert_eq!(
        vocabulary.resolve_untyped("task"),
        VocabularyResolution::Ambiguous(vec![
            "atomic_action".to_string(),
            "implementation_unit".to_string(),
        ])
    );
    assert_eq!(
        vocabulary.resolve_untyped("does not exist"),
        VocabularyResolution::Unknown
    );
    assert_eq!(
        vocabulary.resolve_typed_role("task"),
        None,
        "aliases never resolve typed stable-role context"
    );

    let absorptions = vocabulary.absorptions();
    assert_eq!(absorptions.len(), 2);
    assert_eq!(absorptions[0].unit_id(), "delivery_unit");
    assert_eq!(absorptions[0].absorbs(), ["implementation_unit"]);
    assert_eq!(absorptions[1].unit_id(), "implementation_unit");
    assert_eq!(
        absorptions[1].absorbs(),
        ["atomic_action", "execution_envelope"]
    );
}

#[test]
fn duplicate_display_labels_are_legal_and_untyped_resolution_is_ambiguous() {
    let mut value = nonempty_value();
    value["labels"]["implementation_unit"] = serde_json::json!("Feature");
    close_fingerprint(&mut value);
    let vocabulary = load_value(&value).unwrap();

    assert_eq!(vocabulary.display_label("delivery_unit"), Some("Feature"));
    assert_eq!(
        vocabulary.display_label("implementation_unit"),
        Some("Feature")
    );
    assert_eq!(
        vocabulary.resolve_untyped("feature"),
        VocabularyResolution::Ambiguous(vec![
            "delivery_unit".to_string(),
            "implementation_unit".to_string(),
        ])
    );
    assert_eq!(
        vocabulary
            .resolve_typed_role("delivery_unit")
            .unwrap()
            .role_id(),
        "delivery_unit"
    );
}

#[test]
fn canonical_semantic_order_and_alias_normalization_replay_one_identity() {
    let mut reordered = nonempty_value();
    reordered["aliases"]["delivery_unit"] = serde_json::json!(["FEATURE"]);
    reordered["absorptions"].as_array_mut().unwrap().reverse();
    reordered["absorptions"][0]["absorbs"]
        .as_array_mut()
        .unwrap()
        .reverse();

    let vocabulary = load_value(&reordered).unwrap();
    assert_eq!(
        vocabulary.vocabulary_fingerprint().as_str(),
        "sha256:0a28353460ce60a0fc53ba5e99ea2ec753abf94638489fe1eebd69b317d90ec4"
    );
    assert_eq!(
        vocabulary.resolve_untyped("feature"),
        VocabularyResolution::Unique("delivery_unit".to_string())
    );
}

#[test]
fn vocabulary_validation_refuses_unknown_capability_authority_ownership_and_cycles() {
    let cases: Vec<(&str, serde_json::Value, RegistryLoadErrorKind)> = {
        let mut cases = Vec::new();

        let mut unknown_label = nonempty_value();
        unknown_label["labels"]["unknown_role"] = serde_json::json!("Unknown");
        close_fingerprint(&mut unknown_label);
        cases.push((
            "unknown label role",
            unknown_label,
            RegistryLoadErrorKind::UnknownStableRole,
        ));

        let mut capability_alias = nonempty_value();
        capability_alias["aliases"]["constitutional_root"] = serde_json::json!(["root"]);
        close_fingerprint(&mut capability_alias);
        cases.push((
            "capability alias role",
            capability_alias,
            RegistryLoadErrorKind::UnknownStableRole,
        ));

        let mut capability_absorption = nonempty_value();
        capability_absorption["absorptions"][0]["absorbs"] =
            serde_json::json!(["constitutional_root"]);
        close_fingerprint(&mut capability_absorption);
        cases.push((
            "capability absorption",
            capability_absorption,
            RegistryLoadErrorKind::UnknownStableRole,
        ));

        let mut constitutional = nonempty_value();
        constitutional["absorptions"][0]["absorbs"] =
            serde_json::json!(["constitutional_authority"]);
        close_fingerprint(&mut constitutional);
        cases.push((
            "constitutional absorption",
            constitutional,
            RegistryLoadErrorKind::InvalidStableRoleCategory,
        ));

        let mut artifact = nonempty_value();
        artifact["absorptions"][0]["absorbs"] = serde_json::json!(["project_context"]);
        close_fingerprint(&mut artifact);
        cases.push((
            "artifact absorption",
            artifact,
            RegistryLoadErrorKind::InvalidStableRoleCategory,
        ));

        let mut duplicate_owner = nonempty_value();
        duplicate_owner["absorptions"]
            .as_array_mut()
            .unwrap()
            .push(serde_json::json!({
                "unit_id": "feature_unit",
                "absorbs": ["atomic_action"]
            }));
        close_fingerprint(&mut duplicate_owner);
        cases.push((
            "duplicate owner",
            duplicate_owner,
            RegistryLoadErrorKind::DuplicateIdentity,
        ));

        let mut duplicate_target = nonempty_value();
        duplicate_target["absorptions"][0]["absorbs"] =
            serde_json::json!(["implementation_unit", "implementation_unit"]);
        close_fingerprint(&mut duplicate_target);
        cases.push((
            "duplicate target",
            duplicate_target,
            RegistryLoadErrorKind::DuplicateIdentity,
        ));

        let mut cycle = nonempty_value();
        cycle["absorptions"][1]["absorbs"]
            .as_array_mut()
            .unwrap()
            .push(serde_json::json!("delivery_unit"));
        close_fingerprint(&mut cycle);
        cases.push(("cycle", cycle, RegistryLoadErrorKind::DependencyCycle));

        let mut extensions = nonempty_value();
        extensions["extensions"]["future"] = serde_json::json!(true);
        close_fingerprint(&mut extensions);
        cases.push((
            "undeclared extension",
            extensions,
            RegistryLoadErrorKind::UnsupportedRecord,
        ));

        cases
    };

    for (name, value, expected) in cases {
        let error = load_value(&value).unwrap_err();
        assert_eq!(error.kind(), expected, "{name}");
    }
}
