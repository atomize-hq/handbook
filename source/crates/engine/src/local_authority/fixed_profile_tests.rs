use super::*;

#[test]
fn fixed_profile_warm_reads_do_not_rebuild_validators() {
    profile().unwrap();
    crate::schema_registry::take_validator_build_count();
    let first = profile().unwrap();
    let second = profile().unwrap();
    assert_eq!(
        first.resolved_profile_fingerprint(),
        second.resolved_profile_fingerprint()
    );
    assert_eq!(crate::schema_registry::take_validator_build_count(), 0);
}

#[test]
fn fixed_profile_matches_uncached_closure_and_validates_new_content() {
    let fresh = resolved_local_operating_profile().unwrap();
    let cached = profile().unwrap();
    assert_eq!(fresh.exact_ref(), cached.exact_ref());
    assert_eq!(
        fresh.selected_profile_definition_fingerprint(),
        cached.selected_profile_definition_fingerprint()
    );
    assert_eq!(
        fresh.resolved_profile_fingerprint(),
        cached.resolved_profile_fingerprint()
    );
    assert_eq!(
        fresh.stable_role_registry().fingerprint(),
        cached.stable_role_registry().fingerprint()
    );
    assert_eq!(
        fresh.artifact_kind_registry().fingerprint(),
        cached.artifact_kind_registry().fingerprint()
    );
    assert_eq!(
        fresh.vocabulary().vocabulary_fingerprint(),
        cached.vocabulary().vocabulary_fingerprint()
    );
    assert_eq!(
        fresh.context_resolution().definition_fingerprint(),
        cached.context_resolution().definition_fingerprint()
    );
    let fresh_schemas = fresh.artifact_kind_registry().schema_registry();
    let cached_schemas = cached.artifact_kind_registry().schema_registry();
    assert_eq!(fresh_schemas.fingerprint(), cached_schemas.fingerprint());
    assert_eq!(fresh_schemas.entry_refs(), cached_schemas.entry_refs());
    assert_eq!(fresh_schemas.entry_refs().len(), 18);
    for reference in fresh_schemas.entry_refs() {
        let left = fresh_schemas.entry(&reference).unwrap();
        let right = cached_schemas.entry(&reference).unwrap();
        assert_eq!(left.entry_fingerprint(), right.entry_fingerprint());
        assert_eq!(left.document_fingerprint(), right.document_fingerprint());
        assert_eq!(left.closure_fingerprint(), right.closure_fingerprint());
    }
    let reference =
        crate::ExactDefinitionRef::parse("handbook.schemas.artifacts.project-authority@1.1.0")
            .unwrap();
    let mut content = crate::parse_definition_yaml(include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../docs/specs/handbook-contract-membrane/slices/HCM-2.2/contracts/canonical-charter-boundary-v1.1.yaml"))).unwrap();
    let fresh_schema = fresh_schemas.resolved(&reference).unwrap();
    let cached_schema = cached_schemas.resolved(&reference).unwrap();
    assert!(fresh_schema.validate_json(&content).is_ok());
    assert!(cached_schema.validate_json(&content).is_ok());
    content["project"]["team_size"] = serde_json::json!("invalid");
    assert_eq!(
        fresh_schema.validate_json(&content),
        cached_schema.validate_json(&content)
    );
    assert!(cached_schema.validate_json(&content).is_err());
}

#[test]
fn fixed_profile_refuses_repository_sources_in_every_collection() {
    let original = crate::profile_decision::local_operating_profile_request();
    require_fixed_builtin_profile_request(&original).unwrap();
    for index in 0..11 {
        let mut altered = original.clone();
        let collections = [
            &mut altered.profile_sources,
            &mut altered.stable_role_registry_sources,
            &mut altered.schema_entry_sources,
            &mut altered.artifact_kind_sources,
            &mut altered.semantic_capability_sources,
            &mut altered.semantic_validator_sources,
            &mut altered.project_condition_sources,
            &mut altered.vocabulary_sources,
            &mut altered.context_resolution_sources,
            &mut altered.context_resolution_policy_sources,
            &mut altered.intake_definition_sources,
        ];
        collections
            .into_iter()
            .nth(index)
            .unwrap()
            .push(crate::DefinitionSourceBinding {
                definition_ref: original.selected_profile_ref.clone(),
                source: crate::DefinitionSource::RepositoryPath(
                    "definitions/substituted.yaml".into(),
                ),
            });
        assert_eq!(
            require_fixed_builtin_profile_request(&altered)
                .unwrap_err()
                .code,
            "profile"
        );
    }
    let mut altered = original.clone();
    altered.selected_profile_ref =
        crate::ExactDefinitionRef::parse("handbook.profile.shipped-root@1.1.0").unwrap();
    assert!(require_fixed_builtin_profile_request(&altered).is_err());
    altered = original.clone();
    altered.allowed_schema_roots.push("outside".into());
    assert!(require_fixed_builtin_profile_request(&altered).is_err());
    altered = original;
    altered.profile_sources[0].source = crate::DefinitionSource::BuiltIn(
        crate::ExactDefinitionRef::parse("handbook.profile.shipped-root@1.1.0").unwrap(),
    );
    assert!(require_fixed_builtin_profile_request(&altered).is_err());
}

#[test]
fn fixed_profile_concurrent_initialization_has_one_exact_closure() {
    let cache = OnceLock::new();
    let results = std::thread::scope(|scope| {
        let workers = (0..8)
            .map(|_| {
                let cache = &cache;
                scope.spawn(move || {
                    crate::schema_registry::take_validator_build_count();
                    let selected = cache
                        .get_or_init(resolve_fixed_operating_profile)
                        .as_ref()
                        .unwrap();
                    (
                        selected as *const _ as usize,
                        selected.resolved_profile_fingerprint().clone(),
                        crate::schema_registry::take_validator_build_count(),
                    )
                })
            })
            .collect::<Vec<_>>();
        workers
            .into_iter()
            .map(|worker| worker.join().unwrap())
            .collect::<Vec<_>>()
    });
    assert!(results
        .iter()
        .all(|result| result.0 == results[0].0 && result.1 == results[0].1));
    assert_eq!(results.iter().map(|result| result.2).sum::<usize>(), 18);
    assert_eq!(
        results[0].1,
        profile().unwrap().resolved_profile_fingerprint().clone()
    );
}
