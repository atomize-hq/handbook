use handbook_engine::{
    ExactDefinitionRef, RegistryLoadErrorKind, StableRoleCategory, StableRoleRegistry,
};

const CORE_1_0: &str = "handbook.roles.core@1.0.0";
const CORE_1_1: &str = "handbook.roles.core@1.1.0";

#[test]
fn package_owned_core_registries_replay_the_frozen_fingerprints() {
    let v1 = StableRoleRegistry::load_builtin(&ExactDefinitionRef::parse(CORE_1_0).unwrap())
        .expect("core 1.0.0");
    let v1_1 = StableRoleRegistry::load_builtin(&ExactDefinitionRef::parse(CORE_1_1).unwrap())
        .expect("core 1.1.0");

    assert_eq!(
        v1.fingerprint().as_str(),
        "sha256:7d9407b43ebdda9ac73206bdfcb0e60e3906bdba980820ed12717d63c28e5c3f"
    );
    assert_eq!(
        v1_1.fingerprint().as_str(),
        "sha256:0c85b1b53786e7980c4fd0d7975cd9cde1a3eae2bc8daceb23be1a1731263029"
    );
    assert_eq!(v1.roles().len(), 7);
    assert_eq!(v1_1.roles().len(), 8);
}

#[test]
fn environment_context_is_a_distinct_artifact_role_only_in_core_1_1() {
    let v1 = StableRoleRegistry::load_builtin(&ExactDefinitionRef::parse(CORE_1_0).unwrap())
        .expect("core 1.0.0");
    let v1_1 = StableRoleRegistry::load_builtin(&ExactDefinitionRef::parse(CORE_1_1).unwrap())
        .expect("core 1.1.0");

    assert!(v1.role("environment_context").is_none());
    let environment = v1_1.role("environment_context").expect("new role");
    assert_eq!(environment.category(), StableRoleCategory::Artifact);
    assert_eq!(environment.canonical_display_label(), "Environment Context");
    assert_ne!(environment.role_id(), "project_context");
}

#[test]
fn builtin_selection_is_exact_without_minor_or_latest_substitution() {
    for invalid in ["handbook.roles.core@1.2.0", "handbook.roles.other@1.1.0"] {
        let error = StableRoleRegistry::load_builtin(&ExactDefinitionRef::parse(invalid).unwrap())
            .expect_err(invalid);
        assert_eq!(
            error.kind(),
            RegistryLoadErrorKind::StableRoleRegistryMismatch
        );
    }
}

#[test]
fn stable_role_records_are_closed_and_fingerprint_checked() {
    let base = include_str!("../definitions/stable-roles/handbook.roles.core/1.0.0.yaml");

    let unknown_top = base.replace(
        "registry_fingerprint:",
        "unexpected: true\nregistry_fingerprint:",
    );
    assert_eq!(
        StableRoleRegistry::parse_yaml(unknown_top.as_bytes())
            .expect_err("unknown top-level field")
            .kind(),
        RegistryLoadErrorKind::UnknownField
    );

    let unknown_nested = base.replace(
        "    category: governance",
        "    category: governance\n    capability: forbidden",
    );
    assert_eq!(
        StableRoleRegistry::parse_yaml(unknown_nested.as_bytes())
            .expect_err("unknown nested field")
            .kind(),
        RegistryLoadErrorKind::UnknownField
    );

    let secret_field = format!("SECRET_STABLE_FIELD_{}", "x".repeat(500));
    let unknown_secret = base.replace(
        "registry_fingerprint:",
        &format!("{secret_field}: true\nregistry_fingerprint:"),
    );
    let error = StableRoleRegistry::parse_yaml(unknown_secret.as_bytes())
        .expect_err("secret unknown field");
    assert_eq!(error.kind(), RegistryLoadErrorKind::UnknownField);
    assert!(!error.detail().contains("SECRET_STABLE_FIELD"));
    assert!(!error.to_string().contains("SECRET_STABLE_FIELD"));
    assert!(error.detail().len() < 256);

    let changed_bytes = base.replace("Project Context", "Project Facts");
    assert_eq!(
        StableRoleRegistry::parse_yaml(changed_bytes.as_bytes())
            .expect_err("fingerprint mismatch")
            .kind(),
        RegistryLoadErrorKind::FingerprintMismatch
    );
}

#[test]
fn duplicate_roles_and_invalid_categories_refuse() {
    let base = include_str!("../definitions/stable-roles/handbook.roles.core/1.0.0.yaml");
    let duplicate = base.replace(
        "role_id: project_context",
        "role_id: constitutional_authority",
    );
    assert_eq!(
        StableRoleRegistry::parse_yaml(duplicate.as_bytes())
            .expect_err("duplicate role")
            .kind(),
        RegistryLoadErrorKind::DuplicateIdentity
    );

    let invalid_category = base.replace("category: governance", "category: executable");
    assert_eq!(
        StableRoleRegistry::parse_yaml(invalid_category.as_bytes())
            .expect_err("invalid category")
            .kind(),
        RegistryLoadErrorKind::InvalidStableRoleCategory
    );

    let secret_category = format!("SECRET_CATEGORY_{}", "x".repeat(500));
    let invalid_secret = base.replace(
        "category: governance",
        &format!("category: {secret_category}"),
    );
    let error = StableRoleRegistry::parse_yaml(invalid_secret.as_bytes())
        .expect_err("secret invalid category");
    assert_eq!(
        error.kind(),
        RegistryLoadErrorKind::InvalidStableRoleCategory
    );
    assert!(!error.detail().contains("SECRET_CATEGORY"));
    assert!(!error.to_string().contains("SECRET_CATEGORY"));
    assert!(error.detail().len() < 256);
}
