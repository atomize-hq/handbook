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
        composition: WorkCompositionV1::ordinary_leaf().into(),
        display_level: None,
    }
}
#[test]
fn retained_explicit_binding_uses_supplied_exact_basis_and_original_bytes_without_native_reads() {
    let bytes = include_bytes!("../../sdk/tests/fixtures/work_state_v1/committed-request.json");
    let original = WorkRequestLocatorV1::from_json_bytes(bytes).unwrap();
    let directory = tempfile::tempdir().unwrap();
    let missing_root = directory.path().join("not-established");
    let prepared = NativeWorkStateV1::prepare_registration(
        &missing_root,
        &original.grant,
        &original.grant.basis,
        original.input.clone(),
    )
    .unwrap();
    assert_eq!(prepared.locator(), &original);
    assert_eq!(prepared.locator().canonical_json_bytes().unwrap(), bytes);
    assert!(!missing_root.exists());
    let mut different = original.grant.basis.clone();
    different.governance_tree_fingerprint = format!("sha256:{}", "b".repeat(64));
    let error = NativeWorkStateV1::prepare_registration(
        &missing_root,
        &original.grant,
        &different,
        original.input.clone(),
    )
    .err()
    .unwrap();
    assert_eq!(error.code, "work_basis_stale");
    different.governance_tree_fingerprint = "malformed".into();
    assert!(NativeWorkStateV1::prepare_registration(
        &missing_root,
        &original.grant,
        &different,
        original.input.clone(),
    )
    .is_err());
    let mut actor = original.input.clone();
    actor.actor = "other-agent".into();
    assert!(NativeWorkStateV1::prepare_registration(
        &missing_root,
        &original.grant,
        &original.grant.basis,
        actor
    )
    .is_err());
    assert!(!missing_root.exists());
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
            .explicit_composition()
            .unwrap()
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
    v.composition.explicit_mut().unwrap().descriptor.role_schema =
        vec!["requirements_input".into()];
    assert!(v.canonical_json_bytes().is_err());
    v = original.clone();
    v.composition
        .explicit_mut()
        .unwrap()
        .descriptor
        .relation_support = vec!["parent".into()];
    assert!(v.canonical_json_bytes().is_err());
    v = original.clone();
    v.expected_state = "open".into();
    assert!(v.canonical_json_bytes().is_err());
    v = original.clone();
    v.composition.explicit_mut().unwrap().kind =
        Some(handbook_engine::local_authority::LocalBindingV1 {
            reference: "kind@1".into(),
            fingerprint: "test".into(),
        });
    assert!(v.canonical_json_bytes().is_err());
    v = original;
    v.composition
        .explicit_mut()
        .unwrap()
        .descriptor
        .compatibility_version = "2".into();
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

fn definition(family: WorkDefinitionFamilyV1) -> WorkRequirementsDefinitionV1 {
    WorkRequirementsDefinitionV1 {
        owner: "handbook-contracts".into(),
        family,
        schema_ref: family.schema_ref().into(),
        reference: "example.requirements@1.0.0".into(),
        fingerprint: format!("sha256:{}", "a".repeat(64)),
    }
}
#[test]
fn declarations_are_closed_and_do_not_assert_contract_truth() {
    for family in [
        WorkDefinitionFamilyV1::IntrinsicV1,
        WorkDefinitionFamilyV1::CompleteLocalV2,
        WorkDefinitionFamilyV1::FullLocalV3,
    ] {
        let mut value = input();
        value.composition = WorkCompositionV1::requirements_input(definition(family)).into();
        value.display_level = Some("implementation".into());
        let bytes = value.canonical_json_bytes().unwrap();
        assert_eq!(
            WorkRegistrationInputV1::from_json_bytes(&bytes).unwrap(),
            value
        );
        let mut malformed = definition(family);
        malformed.schema_ref = "handbook.contract-definition@9.0.0".into();
        assert!(malformed.validate().is_err());
        for reference in [
            "example.requirements@latest",
            "example.requirements@1",
            "../path@1.0.0",
            "example.requirements@01.0.0",
            "Example.requirements@1.0.0",
        ] {
            malformed = definition(family);
            malformed.reference = reference.into();
            assert!(malformed.validate().is_err(), "{reference}");
        }
        let mut wire = serde_json::to_value(&value).unwrap();
        wire["composition"]["role"]["assessment"] = json!("passed");
        assert!(
            WorkRegistrationInputV1::from_json_bytes(&serde_json::to_vec(&wire).unwrap()).is_err()
        );
    }
    let mut value = input();
    value.composition =
        WorkCompositionV1::planning_output("What exact requirements are needed?").into();
    let bytes = value.canonical_json_bytes().unwrap();
    assert_eq!(
        WorkRegistrationInputV1::from_json_bytes(&bytes).unwrap(),
        value
    );
    let mut wire = serde_json::to_value(&value).unwrap();
    wire["composition"]["role"]["definition"] =
        json!(definition(WorkDefinitionFamilyV1::IntrinsicV1));
    assert!(WorkRegistrationInputV1::from_json_bytes(&serde_json::to_vec(&wire).unwrap()).is_err());
    value.composition.explicit_mut().unwrap().role = None;
    assert!(value.canonical_json_bytes().is_err());
    value.composition = WorkCompositionV1::planning_output("Ã©".repeat(2049)).into();
    assert!(value.canonical_json_bytes().is_err());
}

fn scoped_terms(clauses: Vec<WorkCompositionClauseV1>) -> WorkRegistrationTermsV1 {
    let mut terms = WorkRegistrationTermsV1 {
        request_key: "scope".into(),
        rule_ref: WORK_REGISTRATION_RULE_V1.into(),
        mode: "local_advisory".into(),
        principal: "human".into(),
        agent: "agent".into(),
        instruction: "Approve these exact finite terms".into(),
        delegation: "bounded work".into(),
        approval_source_ref: "human.scope".into(),
        human_approval_asserted: true,
        approved_content_fingerprint: String::new(),
        effect: "register_open_leaf".into(),
        scope: WorkRegistrationScopeV1::explicit(clauses),
        owner_write_reach: WORK_STATE_FULL_OWNER_REACH_V1.into(),
        owner_read_reach: WORK_STATE_FULL_OWNER_REACH_V1.into(),
        reserved_decisions: vec![
            "composition_change".into(),
            "terms_change".into(),
            "close".into(),
        ],
        revocation_principal: "human".into(),
        retention: "retain_original_history".into(),
    };
    terms.approved_content_fingerprint = terms.fingerprint_approved_content().unwrap();
    terms
}
fn clause() -> WorkCompositionClauseV1 {
    WorkCompositionClauseV1 {
        descriptor: WorkDescriptorV1::ordinary_leaf(),
        label_max_bytes: 4096,
        work_question_max_bytes: 256,
        allow_omitted_display_level: true,
        display_levels: vec!["task".into()],
        role_permission: WorkRolePermissionV1::Ordinary {},
    }
}
#[test]
fn approved_scope_is_finite_complete_and_rejects_irrelevant_or_duplicate_clauses() {
    let terms = scoped_terms(vec![clause()]);
    let bytes = terms.canonical_json_bytes().unwrap();
    assert_eq!(
        WorkRegistrationTermsV1::from_json_bytes(&bytes).unwrap(),
        terms
    );
    for clauses in [vec![], vec![clause(), clause()]] {
        assert!(scoped_terms(clauses).canonical_json_bytes().is_err());
    }
    for bad in [0, 4097, u32::MAX] {
        let mut c = clause();
        c.label_max_bytes = bad;
        assert!(scoped_terms(vec![c]).canonical_json_bytes().is_err());
    }
    let mut c = clause();
    c.role_permission = WorkRolePermissionV1::PlanningOutput {
        question_max_bytes: 10,
    };
    assert!(scoped_terms(vec![c]).canonical_json_bytes().is_err());
    let mut c = clause();
    c.allow_omitted_display_level = false;
    c.display_levels.clear();
    assert!(scoped_terms(vec![c]).canonical_json_bytes().is_err());
    for levels in [
        vec!["task".into(), "task".into()],
        vec!["*".into()],
        vec!["x".repeat(129)],
    ] {
        let mut c = clause();
        c.display_levels = levels;
        assert!(scoped_terms(vec![c]).canonical_json_bytes().is_err());
    }
    let mut c = clause();
    c.descriptor = WorkDescriptorV1::requirements_input();
    c.role_permission = WorkRolePermissionV1::RequirementsInput {
        definitions: vec![],
    };
    assert!(scoped_terms(vec![c.clone()])
        .canonical_json_bytes()
        .is_err());
    c.role_permission = WorkRolePermissionV1::RequirementsInput {
        definitions: vec![definition(WorkDefinitionFamilyV1::FullLocalV3); 2],
    };
    assert!(scoped_terms(vec![c]).canonical_json_bytes().is_err());
    let mut wire = serde_json::to_value(&terms).unwrap();
    wire["composition"]["all_future"] = json!(true);
    assert!(WorkRegistrationTermsV1::from_json_bytes(&serde_json::to_vec(&wire).unwrap()).is_err());
    let mut wire = serde_json::to_value(&terms).unwrap();
    wire["composition"]["clauses"][0]["role_permission"]["question_max_bytes"] = json!(128);
    assert!(WorkRegistrationTermsV1::from_json_bytes(&serde_json::to_vec(&wire).unwrap()).is_err());
}
