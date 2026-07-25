use handbook_engine::{
    DefinitionSource, DefinitionSourceBinding, ExactDefinitionRef, ProfileSelectionRequest,
};

#[test]
fn profile_selection_request_carries_the_separate_typed_intake_source_class() {
    let intake_ref = ExactDefinitionRef::parse("example.intake.registry-brief@1.0.0").unwrap();
    let request = ProfileSelectionRequest {
        selected_profile_ref: ExactDefinitionRef::parse("handbook.profile.shipped-root@1.1.0")
            .unwrap(),
        profile_sources: Vec::new(),
        stable_role_registry_sources: Vec::new(),
        schema_entry_sources: Vec::new(),
        artifact_kind_sources: Vec::new(),
        semantic_capability_sources: Vec::new(),
        semantic_validator_sources: Vec::new(),
        project_condition_sources: Vec::new(),
        vocabulary_sources: Vec::new(),
        context_resolution_sources: Vec::new(),
        context_resolution_policy_sources: Vec::new(),
        intake_definition_sources: vec![DefinitionSourceBinding {
            definition_ref: intake_ref.clone(),
            source: DefinitionSource::RepositoryPath(
                ".handbook/definitions/intakes/registry-brief-1.0.0.yaml".into(),
            ),
        }],
        allowed_schema_roots: Vec::new(),
    };

    assert_eq!(request.intake_definition_sources.len(), 1);
    assert_eq!(
        request.intake_definition_sources[0].definition_ref,
        intake_ref
    );
}
