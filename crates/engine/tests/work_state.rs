use handbook_engine::work_state::*;
use serde_json::json;

fn input() -> WorkRegistrationInputV1 {
    WorkRegistrationInputV1 {
        request_key: "request".into(),
        item_key: "item".into(),
        label: "Investigate session renewal".into(),
        question: "Why does this session expire?".into(),
        actor: "agent".into(),
        expected_state: "absent".into(),
        composition: WorkCompositionV1::ordinary_leaf(),
    }
}
#[test]
fn identity_and_exact_descriptor_do_not_depend_on_codec_release_or_delivery() {
    let one = work_item_identity_v1("project", "item").unwrap();
    assert_eq!(one, work_item_identity_v1("project", "item").unwrap());
    assert_ne!(one, work_item_identity_v1("other-project", "item").unwrap());
    let descriptor = WorkDescriptorV1::ordinary_leaf();
    let b = descriptor.canonical_json_bytes().unwrap();
    assert!(!b.ends_with(b"\n"));
    let mut v = input();
    let initial = v.canonical_json_bytes().unwrap();
    v.request_key = "delivery-two".into();
    v.label = "Any display taxonomy".into();
    assert_eq!(one, work_item_identity_v1("project", &v.item_key).unwrap());
    assert_eq!(
        WorkRegistrationInputV1::from_json_bytes(&initial)
            .unwrap()
            .composition
            .descriptor
            .canonical_json_bytes()
            .unwrap(),
        b
    );
}
#[test]
fn future_roles_kinds_relations_state_and_unknown_fields_refuse_without_silent_projection() {
    let original = input();
    for (field, value) in [
        ("required_output", json!("produce C")),
        ("requirements_input", json!({"reference":"C@1"})),
        ("parent", json!("P")),
        ("state", json!("completed")),
    ] {
        let mut v = serde_json::to_value(&original).unwrap();
        v[field] = value;
        assert!(
            WorkRegistrationInputV1::from_json_bytes(&serde_json::to_vec(&v).unwrap()).is_err()
        );
    }
    let mut v = original.clone();
    v.composition.descriptor.role_schema = vec!["requirements_input".into()];
    assert!(v.canonical_json_bytes().is_err());
    v = original.clone();
    v.composition.descriptor.relation_support = vec!["parent".into()];
    assert!(v.canonical_json_bytes().is_err());
    v = original.clone();
    v.expected_state = "open".into();
    assert!(v.canonical_json_bytes().is_err());
    v = original.clone();
    v.composition.kind = Some(handbook_engine::local_authority::LocalBindingV1 {
        reference: "kind@1".into(),
        fingerprint: "test".into(),
    });
    assert!(v.canonical_json_bytes().is_err());
    v = original;
    v.composition.descriptor.compatibility_version = "2".into();
    assert!(v.canonical_json_bytes().is_err());
}
#[test]
fn complete_bounded_duplicate_safe_json_and_closed_nested_shapes() {
    let v = input();
    let b = v.canonical_json_bytes().unwrap();
    assert_eq!(WorkRegistrationInputV1::from_json_bytes(&b).unwrap(), v);
    let mut trailing = b.clone();
    trailing.extend_from_slice(b" {}");
    assert!(WorkRegistrationInputV1::from_json_bytes(&trailing).is_err());
    let raw = String::from_utf8(b).unwrap();
    let duplicate = raw.replacen("{", "{\"request_key\":\"injected\",", 1);
    assert!(WorkRegistrationInputV1::from_json_bytes(duplicate.as_bytes()).is_err());
    let mut too_long = v.clone();
    too_long.item_key = "x".repeat(129);
    assert!(too_long.canonical_json_bytes().is_err());
    too_long = v;
    too_long.question = "x".repeat(4097);
    assert!(too_long.canonical_json_bytes().is_err());
    for key in [".", "..", "../other", "a/b", ""] {
        assert!(work_item_identity_v1("project", key).is_err());
    }
    assert!(WorkRegistrationInputV1::from_json_bytes(&vec![b' '; 1024 * 1024 + 1]).is_err());
}
