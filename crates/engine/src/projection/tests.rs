use super::*;

const SOURCE: &[u8] = include_bytes!("fixtures/source.json");
const PROFILE: &[u8] = include_bytes!("fixtures/resolved-profile.json");
const VOCABULARY: &[u8] = include_bytes!("fixtures/vocabulary.json");
const DEFINITION: &[u8] = include_bytes!("fixtures/definition.json");
type SemanticMutation = fn(&mut serde_json::Value, &mut serde_json::Value);

#[test]
fn configured_custom_kind_reveal_and_derive_are_deterministic() {
    let configuration = ProjectionConfiguration::load(PROFILE, VOCABULARY, DEFINITION).unwrap();
    let source = ProjectionSource::load(SOURCE).unwrap();
    let broad = TestAuthority::broad(&configuration);

    let reveal = request(&configuration, &source, &broad, ProjectionOperation::Reveal);
    let reveal_first = execute_projection(&configuration, &broad, &reveal, &[&source]).unwrap();
    let reveal_second = execute_projection(&configuration, &broad, &reveal, &[&source]).unwrap();
    assert_eq!(
        reveal_first.canonical_bytes().unwrap(),
        reveal_second.canonical_bytes().unwrap()
    );
    assert_eq!(
        reveal_first.result_fingerprint,
        reveal_second.result_fingerprint
    );
    assert_eq!(reveal_first.authority_effect, AuthorityEffect::None);

    let derive = request(&configuration, &source, &broad, ProjectionOperation::Derive);
    let derive_first = execute_projection(&configuration, &broad, &derive, &[&source]).unwrap();
    let derive_second = execute_projection(&configuration, &broad, &derive, &[&source]).unwrap();
    assert_eq!(
        derive_first.canonical_bytes().unwrap(),
        derive_second.canonical_bytes().unwrap()
    );
    assert_eq!(
        derive_first.result_fingerprint,
        derive_second.result_fingerprint
    );
    assert_eq!(derive_first.derivations.len(), 2);
    assert_eq!(derive_first.lossiness, ProjectionLossiness::Collapsed);
}

#[test]
fn corrected_two_envelope_matrix_varies_only_resolution_and_preserves_source() {
    let configuration = ProjectionConfiguration::load(PROFILE, VOCABULARY, DEFINITION).unwrap();
    let source = ProjectionSource::load(SOURCE).unwrap();
    let before = source.exact_bytes().to_vec();
    let before_fingerprint = source.exact_pair().fingerprint.clone();
    let admitted = admitted_envelopes::Fixture::new();
    let narrow = admitted.narrow.projection_authority_view().unwrap();
    let broad = admitted.broad.projection_authority_view().unwrap();
    assert_ne!(narrow.envelope_pair(), broad.envelope_pair());
    assert_eq!(
        narrow.resolved_profile_pair(),
        broad.resolved_profile_pair()
    );
    assert_eq!(
        narrow.resolution_stack_pair(),
        broad.resolution_stack_pair()
    );

    let narrow_request = request(
        &configuration,
        &source,
        &narrow,
        ProjectionOperation::Reveal,
    );
    let broad_request = request(&configuration, &source, &broad, ProjectionOperation::Reveal);
    assert_eq!(
        narrow_request.non_envelope_fingerprint().unwrap(),
        broad_request.non_envelope_fingerprint().unwrap()
    );

    let narrow_results = [
        execute_projection(&configuration, &narrow, &narrow_request, &[&source]).unwrap(),
        execute_projection(&configuration, &narrow, &narrow_request, &[&source]).unwrap(),
    ];
    let broad_results = [
        execute_projection(&configuration, &broad, &broad_request, &[&source]).unwrap(),
        execute_projection(&configuration, &broad, &broad_request, &[&source]).unwrap(),
    ];
    assert_eq!(
        narrow_results[0].canonical_bytes().unwrap(),
        narrow_results[1].canonical_bytes().unwrap()
    );
    assert_eq!(
        broad_results[0].canonical_bytes().unwrap(),
        broad_results[1].canonical_bytes().unwrap()
    );
    assert_ne!(
        narrow_results[0].output.content_fingerprint,
        broad_results[0].output.content_fingerprint
    );
    assert_ne!(
        narrow_results[0].result_fingerprint,
        broad_results[0].result_fingerprint
    );
    assert_eq!(
        narrow_results[0].disposition_for("reveal_internal_notes"),
        Some(FinalDisposition::OutOfResolution)
    );
    assert_eq!(
        broad_results[0].disposition_for("reveal_internal_notes"),
        Some(FinalDisposition::Included)
    );
    assert_eq!(source.exact_bytes(), before);
    assert_eq!(source.exact_pair().fingerprint, before_fingerprint);

    let narrow_derive = request(
        &configuration,
        &source,
        &narrow,
        ProjectionOperation::Derive,
    );
    let broad_derive = request(&configuration, &source, &broad, ProjectionOperation::Derive);
    let narrow_equal =
        execute_projection(&configuration, &narrow, &narrow_derive, &[&source]).unwrap();
    let broad_equal =
        execute_projection(&configuration, &broad, &broad_derive, &[&source]).unwrap();
    assert_eq!(
        narrow_equal.output.content_fingerprint,
        broad_equal.output.content_fingerprint
    );
    assert_ne!(
        narrow_equal.result_fingerprint,
        broad_equal.result_fingerprint
    );

    admitted.make_authority_stale();
    assert_eq!(
        admitted
            .broad
            .projection_authority_view()
            .unwrap_err()
            .kind(),
        crate::context_resolution::ContextResolutionKernelErrorKind::StaleAuthority
    );
}

#[test]
fn accounting_omissions_proof_effects_lossiness_and_no_read_short_circuits_are_complete() {
    let configuration = ProjectionConfiguration::load(PROFILE, VOCABULARY, DEFINITION).unwrap();
    let source = ProjectionSource::load(SOURCE).unwrap();
    let narrow = TestAuthority::narrow(&configuration);
    let result = execute_projection(
        &configuration,
        &narrow,
        &request(
            &configuration,
            &source,
            &narrow,
            ProjectionOperation::Reveal,
        ),
        &[&source],
    )
    .unwrap();

    assert_eq!(
        result.included.len() + result.omissions.len() + result.not_applicable.len(),
        configuration.definition.field_rules.len()
    );
    assert_eq!(result.disclosure_evaluations.len(), 5);
    assert_eq!(result.lossiness, ProjectionLossiness::Redacted);
    assert_eq!(
        result
            .omission("reveal_internal_notes")
            .unwrap()
            .proof_effect,
        ProofEffect::NotObserved
    );
    assert_eq!(
        result.omission("reveal_optional_detail").unwrap().reason,
        OmissionReason::Unavailable
    );
    assert_eq!(
        result.omission("reveal_secret_notes").unwrap().reason,
        OmissionReason::Redacted
    );
    assert_eq!(
        result.omission("reveal_upstream_hidden").unwrap().reason,
        OmissionReason::Redacted
    );
    assert!(result
        .not_applicable
        .iter()
        .all(|entry| entry.reason == NotApplicableReason::OperationMismatch));

    let denied_only = configuration.only_rule("reveal_secret_notes").unwrap();
    let denied_source = ProjectionSource::load(SOURCE).unwrap();
    let denied_authority = TestAuthority::narrow(&denied_only);
    execute_projection(
        &denied_only,
        &denied_authority,
        &request(
            &denied_only,
            &denied_source,
            &denied_authority,
            ProjectionOperation::Reveal,
        ),
        &[&denied_source],
    )
    .unwrap();
    assert_eq!(denied_source.payload_read_count(), 0);
}

#[test]
fn exact_binding_cardinality_operation_surface_and_expansion_refuse_closed() {
    let configuration = ProjectionConfiguration::load(PROFILE, VOCABULARY, DEFINITION).unwrap();
    let source = ProjectionSource::load(SOURCE).unwrap();
    let broad = TestAuthority::broad(&configuration);
    let base = request(&configuration, &source, &broad, ProjectionOperation::Reveal);

    assert_kind(
        execute_projection(&configuration, &broad, &base, &[]),
        ProjectionErrorKind::Cardinality,
    );
    assert_kind(
        execute_projection(&configuration, &broad, &base, &[&source, &source]),
        ProjectionErrorKind::Cardinality,
    );
    let mut stale = base.clone();
    stale.resolved_profile.fingerprint = fake_fingerprint('9');
    assert_kind(
        execute_projection(&configuration, &broad, &stale, &[&source]),
        ProjectionErrorKind::StaleBinding,
    );
    let mut unsupported_surface = base.clone();
    unsupported_surface.surface = "transport_owned".to_owned();
    assert_kind(
        execute_projection(&configuration, &broad, &unsupported_surface, &[&source]),
        ProjectionErrorKind::UnsupportedSurface,
    );
    let mut unsupported_operation = base.clone();
    unsupported_operation.operation = ProjectionOperation::SynthesizeCandidate;
    assert_kind(
        execute_projection(&configuration, &broad, &unsupported_operation, &[&source]),
        ProjectionErrorKind::UnsupportedOperation,
    );
    let mut expansion = base.clone();
    expansion.target_resolution_ranks = [4, 3, 3, 3, 3, 3];
    assert_kind(
        execute_projection(&configuration, &broad, &expansion, &[&source]),
        ProjectionErrorKind::ResolutionEscalationRequired,
    );
}

#[test]
fn definition_profile_evaluator_and_forbidden_content_refuse_closed() {
    let mut definition: serde_json::Value = serde_json::from_slice(DEFINITION).unwrap();
    definition["allowed_operations"] = serde_json::json!(["synthesize_candidate"]);
    assert_load_kind(
        PROFILE,
        VOCABULARY,
        &canonical_json(&definition),
        ProjectionErrorKind::ForbiddenDefinition,
    );

    let mut definition: serde_json::Value = serde_json::from_slice(DEFINITION).unwrap();
    definition["remote_code"] = serde_json::json!("https://example.invalid/hook");
    assert_load_kind(
        PROFILE,
        VOCABULARY,
        &canonical_json(&definition),
        ProjectionErrorKind::ForbiddenDefinition,
    );

    let mut definition: serde_json::Value = serde_json::from_slice(DEFINITION).unwrap();
    definition["field_rules"][1]["target_pointer"] =
        definition["field_rules"][0]["target_pointer"].clone();
    repair_definition_fingerprint(&mut definition);
    let (profile, vocabulary) = repair_profile_for_definition(PROFILE, VOCABULARY, &definition);
    assert_load_kind(
        &profile,
        &vocabulary,
        &canonical_json(&definition),
        ProjectionErrorKind::InvalidDefinition,
    );

    let mut definition: serde_json::Value = serde_json::from_slice(DEFINITION).unwrap();
    definition["support_evaluator"]["decision_input_fields"] = serde_json::json!(["payload_value"]);
    repair_definition_fingerprint(&mut definition);
    let (profile, vocabulary) = repair_profile_for_definition(PROFILE, VOCABULARY, &definition);
    assert_load_kind(
        &profile,
        &vocabulary,
        &canonical_json(&definition),
        ProjectionErrorKind::InvalidEvaluator,
    );

    let mut profile: serde_json::Value = serde_json::from_slice(PROFILE).unwrap();
    profile["projection_catalog"] = serde_json::json!([{
        "fingerprint": fake_fingerprint('8'),
        "ref": "handbook.projection.unrelated@1.0.0"
    }]);
    repair_fingerprint_field(&mut profile, "profile_fingerprint");
    assert_load_kind(
        &canonical_json(&profile),
        VOCABULARY,
        DEFINITION,
        ProjectionErrorKind::DefinitionUnlisted,
    );
}

#[test]
fn currentness_none_and_exact_captured_revision_closures_refuse_substitution() {
    let configuration = ProjectionConfiguration::load(PROFILE, VOCABULARY, DEFINITION).unwrap();
    let source = ProjectionSource::load(SOURCE).unwrap();
    let broad = TestAuthority::broad(&configuration);
    let mut invalid_none = request(&configuration, &source, &broad, ProjectionOperation::Reveal);
    invalid_none.currentness.revision_basis = Some("captured_revision".to_owned());
    assert_kind(
        execute_projection(&configuration, &broad, &invalid_none, &[&source]),
        ProjectionErrorKind::InvalidCurrentness,
    );

    let exact = configuration
        .with_exact_currentness(
            "roadmap",
            "custom_roadmap",
            "handbook.adapter.fixture@1.0.0",
            &["content"],
        )
        .unwrap();
    let source = snapshot_source();
    let exact_authority = TestAuthority::broad(&exact);
    let mut valid = request(
        &exact,
        &source,
        &exact_authority,
        ProjectionOperation::Reveal,
    );
    valid.currentness = ProjectionCurrentnessRequest::captured(&source, "roadmap").unwrap();
    let live = live_observation(&source);
    let result = execute_projection_with_live_observer(
        &exact,
        &exact_authority,
        &valid,
        &[&source],
        &|_| Ok(live.clone()),
    )
    .unwrap();
    assert_eq!(result.currentness_validation.checks.len(), 2);

    for mutate in [
        |request: &mut ProjectionRequest| request.currentness.expected_family_revisions.clear(),
        |request: &mut ProjectionRequest| {
            request.currentness.expected_family_revisions[0].family_revision =
                "equal-live-but-not-captured".to_owned()
        },
        |request: &mut ProjectionRequest| {
            request.currentness.expected_family_revisions[0].adapter =
                "handbook.adapter.stale@1.0.0".to_owned()
        },
        |request: &mut ProjectionRequest| {
            request.currentness.expected_family_revisions[0]
                .slots
                .push(RevisionSlot {
                    slot: "extra".to_owned(),
                    revision: "x".to_owned(),
                })
        },
    ] {
        let mut changed = valid.clone();
        mutate(&mut changed);
        assert_kind(
            execute_projection_with_live_observer(
                &exact,
                &exact_authority,
                &changed,
                &[&source],
                &|_| Ok(live.clone()),
            ),
            ProjectionErrorKind::InvalidCurrentness,
        );
    }
}

#[test]
fn evaluator_first_reason_semantic_drift_and_unsupported_paths_are_deterministic() {
    let configuration = ProjectionConfiguration::load(PROFILE, VOCABULARY, DEFINITION).unwrap();
    let source = ProjectionSource::load(SOURCE).unwrap();
    let unsupported = configuration
        .only_rule("reveal_objective")
        .unwrap()
        .with_rule_source_schema("reveal_objective", "boolean")
        .unwrap();
    let unsupported_authority = TestAuthority::broad(&unsupported);
    let result = execute_projection(
        &unsupported,
        &unsupported_authority,
        &request(
            &unsupported,
            &source,
            &unsupported_authority,
            ProjectionOperation::Reveal,
        ),
        &[&source],
    )
    .unwrap();
    let evaluation = result.evaluation("reveal_objective").unwrap();
    assert_eq!(
        evaluation.support_reason,
        Some(SupportReason::SourcePointerTypeIncompatible)
    );
    assert_eq!(evaluation.payload_access, PayloadAccess::NotAttempted);
    assert_eq!(source.payload_read_count(), 0);

    let drifted = configuration
        .with_evaluator_reason_order_reversed()
        .unwrap();
    assert_ne!(
        configuration.definition.support_evaluator.exact_pair,
        drifted.definition.support_evaluator.exact_pair
    );
    assert_ne!(
        configuration.definition.exact_pair,
        drifted.definition.exact_pair
    );
    let original_authority = TestAuthority::broad(&configuration);
    let drifted_authority = TestAuthority::broad(&drifted);
    let original = execute_projection(
        &configuration,
        &original_authority,
        &request(
            &configuration,
            &source,
            &original_authority,
            ProjectionOperation::Reveal,
        ),
        &[&source],
    )
    .unwrap();
    let drifted_result = execute_projection(
        &drifted,
        &drifted_authority,
        &request(
            &drifted,
            &source,
            &drifted_authority,
            ProjectionOperation::Reveal,
        ),
        &[&source],
    )
    .unwrap();
    assert_ne!(
        original.result_fingerprint,
        drifted_result.result_fingerprint
    );
}

#[test]
fn lossiness_precedence_and_short_circuit_payload_access_are_exact() {
    let configuration = ProjectionConfiguration::load(PROFILE, VOCABULARY, DEFINITION).unwrap();
    let cases = [
        (
            "reveal_secret_notes",
            false,
            ProjectionLossiness::Redacted,
            0,
        ),
        (
            "reveal_optional_detail",
            false,
            ProjectionLossiness::Partial,
            1,
        ),
        (
            "reveal_internal_notes",
            false,
            ProjectionLossiness::Collapsed,
            0,
        ),
        ("reveal_objective", true, ProjectionLossiness::Lossless, 1),
    ];
    for (rule_id, broad, expected_lossiness, expected_reads) in cases {
        let selected = configuration.only_rule(rule_id).unwrap();
        let source = ProjectionSource::load(SOURCE).unwrap();
        let authority = if broad {
            TestAuthority::broad(&selected)
        } else {
            TestAuthority::narrow(&selected)
        };
        let result = execute_projection(
            &selected,
            &authority,
            &request(&selected, &source, &authority, ProjectionOperation::Reveal),
            &[&source],
        )
        .unwrap();
        assert_eq!(result.lossiness, expected_lossiness, "{rule_id}");
        assert_eq!(source.payload_read_count(), expected_reads, "{rule_id}");
    }
}

#[test]
fn stale_envelope_stack_custom_kind_policy_and_evaluator_dependencies_refuse() {
    let configuration = ProjectionConfiguration::load(PROFILE, VOCABULARY, DEFINITION).unwrap();
    let source = ProjectionSource::load(SOURCE).unwrap();
    let authority = TestAuthority::broad(&configuration);
    let base = request(
        &configuration,
        &source,
        &authority,
        ProjectionOperation::Reveal,
    );

    let mut stale_envelope = base.clone();
    stale_envelope.resolution_envelope.fingerprint = fake_fingerprint('7');
    assert_kind(
        execute_projection(&configuration, &authority, &stale_envelope, &[&source]),
        ProjectionErrorKind::StaleBinding,
    );
    let mut stale_stack = base;
    stale_stack.resolution_stack.fingerprint = fake_fingerprint('6');
    assert_kind(
        execute_projection(&configuration, &authority, &stale_stack, &[&source]),
        ProjectionErrorKind::StaleBinding,
    );

    let mut profile: serde_json::Value = serde_json::from_slice(PROFILE).unwrap();
    profile["configured_kinds"][0]["capability"]["fingerprint"] =
        serde_json::json!(fake_fingerprint('5'));
    repair_fingerprint_field(&mut profile, "profile_fingerprint");
    assert_load_kind(
        &canonical_json(&profile),
        VOCABULARY,
        DEFINITION,
        ProjectionErrorKind::StaleBinding,
    );

    let mut definition: serde_json::Value = serde_json::from_slice(DEFINITION).unwrap();
    definition["disclosure_policy"]["unmatched_action"] = serde_json::json!("allow");
    repair_definition_fingerprint(&mut definition);
    let (profile, vocabulary) = repair_profile_for_definition(PROFILE, VOCABULARY, &definition);
    assert_load_kind(
        &profile,
        &vocabulary,
        &canonical_json(&definition),
        ProjectionErrorKind::InvalidPolicy,
    );

    let mut definition: serde_json::Value = serde_json::from_slice(DEFINITION).unwrap();
    definition["support_evaluator"]["pointer_semantics"]["ref"] =
        serde_json::json!("handbook.json-pointer.stale@1.0.0");
    repair_definition_fingerprint(&mut definition);
    let (profile, vocabulary) = repair_profile_for_definition(PROFILE, VOCABULARY, &definition);
    assert_load_kind(
        &profile,
        &vocabulary,
        &canonical_json(&definition),
        ProjectionErrorKind::InvalidEvaluator,
    );
}

#[test]
fn exact_currentness_requires_snapshot_selection_and_independent_live_observation() {
    let mut definition: serde_json::Value = serde_json::from_slice(DEFINITION).unwrap();
    definition["currentness_requirements"] = serde_json::json!({
        "families": [{
            "adapter": "handbook.adapter.fixture@1.0.0",
            "family": "custom_roadmap",
            "selector_id": "roadmap",
            "slots": ["content"]
        }],
        "mode": "exact_revision_check",
        "revision_basis": "captured_revision"
    });
    repair_definition_fingerprint(&mut definition);
    let (profile, vocabulary) = repair_profile_for_definition(PROFILE, VOCABULARY, &definition);
    assert_load_kind(
        &profile,
        &vocabulary,
        &canonical_json(&definition),
        ProjectionErrorKind::InvalidCurrentness,
    );

    let configuration = ProjectionConfiguration::load(PROFILE, VOCABULARY, DEFINITION).unwrap();
    let exact = configuration
        .with_exact_currentness(
            "roadmap",
            "custom_roadmap",
            "handbook.adapter.fixture@1.0.0",
            &["content"],
        )
        .unwrap();
    let source = snapshot_source();
    let authority = TestAuthority::broad(&exact);
    let mut exact_request = request(&exact, &source, &authority, ProjectionOperation::Reveal);
    exact_request.currentness = ProjectionCurrentnessRequest::captured(&source, "roadmap").unwrap();
    assert_kind(
        execute_projection(&exact, &authority, &exact_request, &[&source]),
        ProjectionErrorKind::InvalidCurrentness,
    );
    let mut unrelated = live_observation(&source);
    unrelated.source = ExactPair::new("snapshot.unrelated@7", &fake_fingerprint('0')).unwrap();
    assert_kind(
        execute_projection_with_live_observer(
            &exact,
            &authority,
            &exact_request,
            &[&source],
            &|_| Ok(unrelated.clone()),
        ),
        ProjectionErrorKind::InvalidCurrentness,
    );
    assert_eq!(source.payload_read_count(), 0);
}

#[test]
fn validated_narrower_target_is_the_effective_evaluation_boundary() {
    let configuration = ProjectionConfiguration::load(PROFILE, VOCABULARY, DEFINITION).unwrap();
    let source = ProjectionSource::load(SOURCE).unwrap();
    let authority = TestAuthority::broad(&configuration);

    let broad_request = request(
        &configuration,
        &source,
        &authority,
        ProjectionOperation::Reveal,
    );
    let broad_result =
        execute_projection(&configuration, &authority, &broad_request, &[&source]).unwrap();

    let mut collapsed_request = broad_request.clone();
    collapsed_request.target_resolution_ranks = [0; 6];
    let collapsed_result =
        execute_projection(&configuration, &authority, &collapsed_request, &[&source]).unwrap();

    assert_eq!(
        collapsed_result.disposition_for("reveal_internal_notes"),
        Some(FinalDisposition::OutOfResolution)
    );
    assert_ne!(
        broad_result.output.content_fingerprint,
        collapsed_result.output.content_fingerprint
    );
    assert_ne!(
        broad_result.result_fingerprint,
        collapsed_result.result_fingerprint
    );
    assert_eq!(collapsed_result.target_resolution_ranks, [0; 6]);
}

#[test]
fn authored_profile_configuration_pair_is_request_and_result_identity() {
    let original = ProjectionConfiguration::load(PROFILE, VOCABULARY, DEFINITION).unwrap();
    let mut changed_profile: serde_json::Value = serde_json::from_slice(PROFILE).unwrap();
    changed_profile["purposes"] = serde_json::json!([
        "delegated_execution_context",
        "independently_authored_profile_identity"
    ]);
    repair_fingerprint_field(&mut changed_profile, "profile_fingerprint");
    let changed =
        ProjectionConfiguration::load(&canonical_json(&changed_profile), VOCABULARY, DEFINITION)
            .unwrap();
    assert_eq!(original.profile.exact_pair, changed.profile.exact_pair);
    assert_ne!(
        original.profile.configuration_pair,
        changed.profile.configuration_pair
    );

    let source = ProjectionSource::load(SOURCE).unwrap();
    let authority = TestAuthority::broad(&original);
    let original_request = request(&original, &source, &authority, ProjectionOperation::Reveal);
    let changed_request = request(&changed, &source, &authority, ProjectionOperation::Reveal);
    assert_ne!(
        original_request.non_envelope_fingerprint().unwrap(),
        changed_request.non_envelope_fingerprint().unwrap()
    );
    assert_kind(
        execute_projection(&changed, &authority, &original_request, &[&source]),
        ProjectionErrorKind::StaleBinding,
    );
    let changed_result =
        execute_projection(&changed, &authority, &changed_request, &[&source]).unwrap();
    assert_eq!(
        changed_result.profile_configuration,
        changed.profile.configuration_pair
    );
}

#[test]
fn repaired_fingerprints_cannot_substitute_trusted_semantic_dependencies() {
    let cases: &[(&str, SemanticMutation)] = &[
        ("capability", |profile, definition| {
            profile["configured_kinds"][0]["capability"]["fingerprint"] =
                serde_json::json!(fake_fingerprint('1'));
            definition["source_selectors"][0]["capability"]["fingerprint"] =
                serde_json::json!(fake_fingerprint('1'));
        }),
        ("source_schema", |profile, definition| {
            profile["configured_kinds"][0]["source_schema"]["fingerprint"] =
                serde_json::json!(fake_fingerprint('2'));
            definition["source_selectors"][0]["source_schema"]["fingerprint"] =
                serde_json::json!(fake_fingerprint('2'));
        }),
        ("target_schema", |_profile, definition| {
            definition["target_schema"]["fingerprint"] = serde_json::json!(fake_fingerprint('3'));
        }),
        ("derivation", |_profile, definition| {
            definition["derivations"][0]["fingerprint"] = serde_json::json!(fake_fingerprint('4'));
            definition["field_rules"][0]["derivation"]["fingerprint"] =
                serde_json::json!(fake_fingerprint('4'));
        }),
        ("matcher", |_profile, definition| {
            definition["disclosure_policy"]["matcher_definition"]["fingerprint"] =
                serde_json::json!(fake_fingerprint('5'));
        }),
        ("classification", |_profile, definition| {
            definition["disclosure_policy"]["classification_registry"]["fingerprint"] =
                serde_json::json!(fake_fingerprint('6'));
        }),
        ("schema_registry", |_profile, definition| {
            definition["support_evaluator"]["schema_registry_contract"]["fingerprint"] =
                serde_json::json!(fake_fingerprint('7'));
        }),
        ("pointer_semantics", |_profile, definition| {
            definition["support_evaluator"]["pointer_semantics"]["fingerprint"] =
                serde_json::json!(fake_fingerprint('8'));
        }),
        ("derivation_compatibility", |_profile, definition| {
            definition["support_evaluator"]["derivation_compatibility"]["fingerprint"] =
                serde_json::json!(fake_fingerprint('9'));
        }),
        ("policy_semantics", |_profile, definition| {
            definition["disclosure_policy"]["rules"][0]["action"] = serde_json::json!("redact");
        }),
        ("evaluator_semantics", |_profile, definition| {
            definition["support_evaluator"]["supported_source_kinds"] =
                serde_json::json!(["semantic_record"]);
        }),
    ];

    for (label, mutate) in cases {
        let mut profile: serde_json::Value = serde_json::from_slice(PROFILE).unwrap();
        let mut definition: serde_json::Value = serde_json::from_slice(DEFINITION).unwrap();
        mutate(&mut profile, &mut definition);
        repair_definition_fingerprint(&mut definition);
        profile["projection_catalog"][0]["fingerprint"] =
            definition["definition_fingerprint"].clone();
        repair_fingerprint_field(&mut profile, "profile_fingerprint");
        let result = ProjectionConfiguration::load(
            &canonical_json(&profile),
            VOCABULARY,
            &canonical_json(&definition),
        );
        assert_eq!(
            result.unwrap_err().kind,
            ProjectionErrorKind::StaleBinding,
            "{label}"
        );
    }

    let configuration = ProjectionConfiguration::load(PROFILE, VOCABULARY, DEFINITION).unwrap();
    let mut drifted_source: serde_json::Value = serde_json::from_slice(SOURCE).unwrap();
    drifted_source["declared_pointers"]["/objective"] = serde_json::json!("boolean");
    let source = ProjectionSource::load(&canonical_json(&drifted_source)).unwrap();
    let authority = TestAuthority::broad(&configuration);
    let projection_request = request(
        &configuration,
        &source,
        &authority,
        ProjectionOperation::Reveal,
    );
    assert_kind(
        execute_projection(&configuration, &authority, &projection_request, &[&source]),
        ProjectionErrorKind::StaleBinding,
    );
    assert_eq!(source.payload_read_count(), 0);
}

fn request<A: ProjectionAuthorityAccess>(
    configuration: &ProjectionConfiguration,
    source: &ProjectionSource,
    authority: &A,
    operation: ProjectionOperation,
) -> ProjectionRequest {
    ProjectionRequest::new(
        "projection-request.fixture",
        vec![ProjectionSourceSelection {
            selector_id: "roadmap".to_owned(),
            exact_pair: source.exact_pair(),
        }],
        configuration.profile.configuration_pair.clone(),
        configuration.profile.exact_pair.clone(),
        configuration.vocabulary.exact_pair.clone(),
        configuration.definition.exact_pair.clone(),
        authority.envelope_pair(),
        authority.resolution_stack_pair(),
        operation,
        "agent_packet",
        "delegated_execution_context",
        ProjectionCurrentnessRequest::none(),
        authority.dimension_ranks(),
    )
}

fn snapshot_source() -> ProjectionSource {
    let mut source: serde_json::Value = serde_json::from_slice(SOURCE).unwrap();
    source["authority_class"] = serde_json::json!("observation_snapshot");
    source["source_kind"] = serde_json::json!("snapshot");
    ProjectionSource::load(&canonical_json(&source)).unwrap()
}

fn live_observation(source: &ProjectionSource) -> LiveCurrentnessObservation {
    LiveCurrentnessObservation {
        selector_id: "roadmap".to_owned(),
        source: source.exact_pair(),
        family: "custom_roadmap".to_owned(),
        adapter: "handbook.adapter.fixture@1.0.0".to_owned(),
        family_revision: "roadmap-revision-7".to_owned(),
        slots: BTreeMap::from([("content".to_owned(), "content-revision-11".to_owned())]),
    }
}

#[derive(Clone)]
struct TestAuthority {
    envelope: ExactPair,
    profile: ExactPair,
    stack: ExactPair,
    level: String,
    values: [String; 6],
    ranks: [u8; 6],
}

impl TestAuthority {
    fn narrow(configuration: &ProjectionConfiguration) -> Self {
        Self::new(
            configuration,
            "envelope.fixture.narrow",
            '1',
            "operation",
            [0, 0, 0, 0, 0, 0],
        )
    }

    fn broad(configuration: &ProjectionConfiguration) -> Self {
        Self::new(
            configuration,
            "envelope.fixture.broad",
            '2',
            "execution",
            [1, 2, 1, 1, 1, 1],
        )
    }

    fn new(
        configuration: &ProjectionConfiguration,
        reference: &str,
        fill: char,
        level: &str,
        ranks: [u8; 6],
    ) -> Self {
        Self {
            envelope: ExactPair::new(reference, &fake_fingerprint(fill)).unwrap(),
            profile: configuration.profile.exact_pair.clone(),
            stack: configuration.profile.resolution_stack.clone(),
            level: level.to_owned(),
            values: ranks.map(|rank| format!("rank-{rank}")),
            ranks,
        }
    }
}

impl ProjectionAuthorityAccess for TestAuthority {
    fn envelope_pair(&self) -> ExactPair {
        self.envelope.clone()
    }
    fn resolved_profile_pair(&self) -> ExactPair {
        self.profile.clone()
    }
    fn resolution_stack_pair(&self) -> ExactPair {
        self.stack.clone()
    }
    fn active_level(&self) -> &str {
        &self.level
    }
    fn dimension_values(&self) -> [&str; 6] {
        self.values.each_ref().map(String::as_str)
    }
    fn dimension_ranks(&self) -> [u8; 6] {
        self.ranks
    }
}

fn assert_kind(result: Result<ProjectionResult, ProjectionError>, kind: ProjectionErrorKind) {
    assert_eq!(result.unwrap_err().kind, kind);
}

fn assert_load_kind(
    profile: &[u8],
    vocabulary: &[u8],
    definition: &[u8],
    kind: ProjectionErrorKind,
) {
    assert_eq!(
        ProjectionConfiguration::load(profile, vocabulary, definition)
            .unwrap_err()
            .kind,
        kind
    );
}

fn fake_fingerprint(fill: char) -> String {
    format!("sha256:{}", fill.to_string().repeat(64))
}

fn canonical_json(value: &serde_json::Value) -> Vec<u8> {
    serde_json_canonicalizer::to_vec(value).unwrap()
}

fn repair_fingerprint_field(value: &mut serde_json::Value, field: &str) {
    let fingerprint = fingerprint_without_field(value, field).unwrap().to_string();
    value[field] = serde_json::json!(fingerprint);
}

fn repair_definition_fingerprint(definition: &mut serde_json::Value) {
    repair_fingerprint_field(&mut definition["disclosure_policy"], "policy_fingerprint");
    repair_fingerprint_field(
        &mut definition["support_evaluator"],
        "evaluator_fingerprint",
    );
    repair_fingerprint_field(definition, "definition_fingerprint");
}

fn repair_profile_for_definition(
    profile: &[u8],
    vocabulary: &[u8],
    definition: &serde_json::Value,
) -> (Vec<u8>, Vec<u8>) {
    let vocabulary = vocabulary.to_vec();
    let mut profile: serde_json::Value = serde_json::from_slice(profile).unwrap();
    profile["projection_catalog"][0]["fingerprint"] = definition["definition_fingerprint"].clone();
    repair_fingerprint_field(&mut profile, "profile_fingerprint");
    (canonical_json(&profile), vocabulary)
}

mod admitted_envelopes {
    use base64::{engine::general_purpose::STANDARD as BASE64_STANDARD, Engine as _};
    use p256::ecdsa::{signature::Signer, Signature, SigningKey};
    use serde_json::json;
    use sha2::{Digest, Sha256};
    use std::fs;
    use std::path::Path;

    use crate::context_resolution::{
        ContextResolutionAuthorityAdmissionResolver, ContextResolutionAuthorityUse,
        ContextResolutionDimensions, ContextResolutionEnvelope, ContextResolutionEnvelopeInput,
        ContextResolutionExactBinding, ContextResolutionMutationRule,
    };
    use crate::{
        resolve_profile_selection, ApproverAdminRequestV1, ApproverRegistryServiceV1,
        ContextResolutionPolicyRegistry, ContextResolutionStackDefinition, DefinitionFingerprint,
        ExactDefinitionRef, NativeAuthenticatorPortErrorV1, NativeAuthenticatorPortV1,
        RepositoryInvocationIdentityServiceV1, ResolvedInstanceProfile, AUTHENTICATOR_RP_ID,
    };

    const AUTHORITY_KIND_REF: &str =
        "handbook.artifact-kind.context-resolution-authority-binding@1.0.0";
    const AUTHORITY_INSTANCE_ID: &str = "context_resolution_authority";
    const AUTHORITY_PATH: &str = ".handbook/project/context-resolution-authority.yaml";
    const STACK_PATH: &str =
        "definitions/context-resolution/handbook.context-resolution.shipped-root/1.0.0.yaml";
    const POLICY_PATHS: [&str; 3] = [
        "definitions/context-resolution-policies/handbook.mutation-matcher.core/1.0.0.yaml",
        "definitions/context-resolution-policies/handbook.resolution-escalation.core/1.0.0.yaml",
        "definitions/context-resolution-policies/handbook.memory-promotion.core/1.0.0.yaml",
    ];

    pub(super) struct Fixture {
        repo: tempfile::TempDir,
        pub(super) narrow: ContextResolutionEnvelope,
        pub(super) broad: ContextResolutionEnvelope,
    }

    impl Fixture {
        pub(super) fn new() -> Self {
            let (repo, mut authenticator, artifact_fingerprint) = seed_authority_repository();
            let (profile, stack) = load_profile_stack(repo.path());
            let mut resolver = ContextResolutionAuthorityAdmissionResolver::new(
                repo.path(),
                AUTHORITY_KIND_REF,
                AUTHORITY_INSTANCE_ID,
                &artifact_fingerprint,
            )
            .unwrap();
            let narrow = admit_root(
                &profile,
                &stack,
                &mut resolver,
                &mut authenticator,
                "projection-narrow",
                "operation",
                [
                    "local_observation",
                    "identifier_only",
                    "current_operation",
                    "read_only",
                    "operation",
                    "observation_only",
                ],
            );
            let broad = admit_root(
                &profile,
                &stack,
                &mut resolver,
                &mut authenticator,
                "projection-broad",
                "strategic",
                [
                    "program",
                    "full",
                    "long_range",
                    "program_policy",
                    "strategic",
                    "program_gate",
                ],
            );
            assert_ne!(narrow.exact_binding(), broad.exact_binding());
            Self {
                repo,
                narrow,
                broad,
            }
        }

        pub(super) fn make_authority_stale(&self) {
            fs::write(self.repo.path().join(AUTHORITY_PATH), b"stale-authority\n").unwrap();
        }
    }

    fn admit_root(
        profile: &ResolvedInstanceProfile,
        stack: &ContextResolutionStackDefinition,
        resolver: &mut ContextResolutionAuthorityAdmissionResolver,
        authenticator: &mut Authenticator,
        id: &str,
        level: &str,
        dimensions: [&str; 6],
    ) -> ContextResolutionEnvelope {
        let input = ContextResolutionEnvelopeInput::new(
            id,
            "objective.fixture",
            ContextResolutionExactBinding::new(
                profile.exact_ref().as_str(),
                profile.resolved_profile_fingerprint().as_str(),
            )
            .unwrap(),
            ContextResolutionExactBinding::new(
                stack.exact_ref().as_str(),
                stack.definition_fingerprint().as_str(),
            )
            .unwrap(),
            level,
            ContextResolutionDimensions::new(dimensions).unwrap(),
            None,
            Vec::new(),
            Vec::<ContextResolutionMutationRule>::new(),
            Vec::new(),
        )
        .unwrap();
        let admission = resolver
            .admit(
                authenticator,
                ContextResolutionAuthorityUse::RootCreation,
                input.candidate_binding(),
            )
            .unwrap();
        ContextResolutionEnvelope::resolve_root(profile, stack, input, &admission, &[]).unwrap()
    }

    fn load_profile_stack(
        repo: &Path,
    ) -> (ResolvedInstanceProfile, ContextResolutionStackDefinition) {
        let selection =
            crate::artifact_intake_registry::RepositoryProfileSelectionV1::from_json_bytes(
                &fs::read(repo.join(".handbook/profile-selection.json")).unwrap(),
            )
            .unwrap();
        let profile = resolve_profile_selection(repo, selection.profile_request()).unwrap();
        assert_eq!(
            profile.exact_ref().as_str(),
            "example.profile.hcm-3-2-context-resolution-authority@1.0.0"
        );
        assert_eq!(
            profile.resolved_profile_fingerprint().as_str(),
            "sha256:054022325d0b044dab5d21d14f8577bd6a42de206bab6d44bb255dde2b5ed133"
        );
        let policies = ContextResolutionPolicyRegistry::load(
            Path::new(env!("CARGO_MANIFEST_DIR")),
            &POLICY_PATHS.map(str::to_string),
        )
        .unwrap();
        let stack = ContextResolutionStackDefinition::load(
            Path::new(env!("CARGO_MANIFEST_DIR")),
            STACK_PATH,
            &policies,
        )
        .unwrap();
        (profile, stack)
    }

    struct Authenticator {
        key: SigningKey,
        credential_id: Vec<u8>,
        make_calls: usize,
        get_calls: usize,
        last_response: Option<Vec<u8>>,
    }

    impl Authenticator {
        fn new() -> Self {
            Self {
                key: SigningKey::from_slice(&[7_u8; 32]).unwrap(),
                credential_id: vec![0x32; 32],
                make_calls: 0,
                get_calls: 0,
                last_response: None,
            }
        }
    }

    impl NativeAuthenticatorPortV1 for Authenticator {
        fn make_credential(
            &mut self,
            _request_cbor: &[u8],
        ) -> Result<Vec<u8>, NativeAuthenticatorPortErrorV1> {
            self.make_calls += 1;
            Ok(make_credential_response(&self.key, &self.credential_id))
        }

        fn get_assertion(
            &mut self,
            request_cbor: &[u8],
        ) -> Result<Vec<u8>, NativeAuthenticatorPortErrorV1> {
            self.get_calls += 1;
            let marker = request_cbor
                .windows(3)
                .position(|window| window == [0x02, 0x58, 0x20])
                .expect("GetAssertion client-data hash marker");
            let client_data_hash = request_cbor[marker + 3..marker + 35].try_into().unwrap();
            let response = assertion_response(
                &self.key,
                &self.credential_id,
                client_data_hash,
                self.get_calls as u32,
            );
            self.last_response = Some(response.clone());
            Ok(response)
        }
    }

    fn make_credential_response(key: &SigningKey, credential_id: &[u8]) -> Vec<u8> {
        let point = key.verifying_key().to_encoded_point(false);
        let mut cose = vec![0xa5, 0x01, 0x02, 0x03, 0x26, 0x20, 0x01, 0x21, 0x58, 0x20];
        cose.extend_from_slice(point.x().unwrap());
        cose.extend_from_slice(&[0x22, 0x58, 0x20]);
        cose.extend_from_slice(point.y().unwrap());
        let mut auth_data = Sha256::digest(AUTHENTICATOR_RP_ID.as_bytes()).to_vec();
        auth_data.push(0x45);
        auth_data.extend_from_slice(&0_u32.to_be_bytes());
        auth_data.extend_from_slice(&[0_u8; 16]);
        auth_data.extend_from_slice(&(credential_id.len() as u16).to_be_bytes());
        auth_data.extend_from_slice(credential_id);
        auth_data.extend_from_slice(&cose);
        let mut response = vec![0, 0xa3];
        cbor_text(&mut response, "fmt");
        cbor_text(&mut response, "none");
        cbor_text(&mut response, "attStmt");
        response.push(0xa0);
        cbor_text(&mut response, "authData");
        cbor_bytes(&mut response, &auth_data);
        response
    }

    fn assertion_response(
        key: &SigningKey,
        credential_id: &[u8],
        client_data_hash: [u8; 32],
        sign_count: u32,
    ) -> Vec<u8> {
        let mut authenticator_data = Sha256::digest(AUTHENTICATOR_RP_ID.as_bytes()).to_vec();
        authenticator_data.push(0x05);
        authenticator_data.extend_from_slice(&sign_count.to_be_bytes());
        let mut preimage = authenticator_data.clone();
        preimage.extend_from_slice(&client_data_hash);
        let signature: Signature = key.sign(&preimage);
        let mut response = vec![0, 0xa3, 0x01, 0xa2];
        cbor_text(&mut response, "id");
        cbor_bytes(&mut response, credential_id);
        cbor_text(&mut response, "type");
        cbor_text(&mut response, "public-key");
        response.push(0x02);
        cbor_bytes(&mut response, &authenticator_data);
        response.push(0x03);
        cbor_bytes(&mut response, signature.to_der().as_bytes());
        response
    }

    fn cbor_text(output: &mut Vec<u8>, value: &str) {
        cbor_length(output, 3, value.len());
        output.extend_from_slice(value.as_bytes());
    }

    fn cbor_bytes(output: &mut Vec<u8>, value: &[u8]) {
        cbor_length(output, 2, value.len());
        output.extend_from_slice(value);
    }

    fn cbor_length(output: &mut Vec<u8>, major: u8, length: usize) {
        if length < 24 {
            output.push((major << 5) | length as u8);
        } else if length <= u8::MAX as usize {
            output.extend_from_slice(&[(major << 5) | 24, length as u8]);
        } else {
            output.push((major << 5) | 25);
            output.extend_from_slice(&(length as u16).to_be_bytes());
        }
    }

    fn copy_fixture_tree(source: &Path, target: &Path) {
        fs::create_dir_all(target).unwrap();
        for entry in fs::read_dir(source).unwrap() {
            let entry = entry.unwrap();
            let source_path = entry.path();
            let target_path = target.join(entry.file_name());
            if source_path.is_dir() {
                copy_fixture_tree(&source_path, &target_path);
            } else {
                fs::copy(source_path, target_path).unwrap();
            }
        }
    }

    fn tagged_fingerprint(tag: &[u8], bytes: &[u8]) -> String {
        let mut digest = Sha256::new();
        digest.update(tag);
        digest.update((bytes.len() as u64).to_be_bytes());
        digest.update(bytes);
        format!("sha256:{:x}", digest.finalize())
    }

    fn authority_mappings() -> serde_json::Value {
        json!({
            "root_creation":{"approval_class":"context_resolution_root","authority_ref":"work.root.owner"},
            "parent":{"approval_class":"context_resolution_parent","authority_ref":"work.parent.owner"},
            "requested":{"approval_class":"context_resolution_requested","authority_ref":"work.request.owner"},
            "approving":{"approval_class":"context_resolution_approving","authority_ref":"work.approver"},
            "decision":{"approval_class":"context_resolution_decision","authority_ref":"decision.owner"},
            "evidence":{"approval_class":"context_resolution_evidence","authority_ref":"evidence.owner"},
            "trigger_condition":{"approval_class":"context_resolution_trigger","authority_ref":"trigger.owner"},
            "constraint":{"approval_class":"context_resolution_constraint","authority_ref":"constraint.owner"},
            "source":{"approval_class":"context_resolution_source","authority_ref":"source.owner"},
            "target":{"approval_class":"context_resolution_target","authority_ref":"target.owner"},
            "target_memory":{"approval_class":"context_resolution_target_memory","authority_ref":"memory.owner"}
        })
    }

    fn candidate_mappings() -> serde_json::Value {
        json!({
            "dimension_rank_increase":{"trigger":{"ref":"trigger.dimension-rank-increase@1.0.0","fingerprint":"sha256:1111111111111111111111111111111111111111111111111111111111111111"},"missing_condition":"one_or_more_dimension_ranks_exceed_parent","evidence_requirement":"current_and_proposed_envelopes"},
            "mutation_allow_expansion":{"trigger":{"ref":"trigger.mutation-allow-expansion@1.0.0","fingerprint":"sha256:2222222222222222222222222222222222222222222222222222222222222222"},"missing_condition":"child_allow_not_provably_contained","evidence_requirement":"parent_and_child_mutation_rules"},
            "missing_context":{"trigger":{"ref":"trigger.missing-context@1.0.0","fingerprint":"sha256:3333333333333333333333333333333333333333333333333333333333333333"},"missing_condition":"required_context_record_absent","evidence_requirement":"available_context_and_missing_ref"},
            "missing_authority":{"trigger":{"ref":"trigger.missing-authority@1.0.0","fingerprint":"sha256:4444444444444444444444444444444444444444444444444444444444444444"},"missing_condition":"required_authority_mapping_absent","evidence_requirement":"current_authority_and_requested_pair"}
        })
    }

    fn seed_authority_repository() -> (tempfile::TempDir, Authenticator, String) {
        let repo = tempfile::tempdir().unwrap();
        copy_fixture_tree(
            &Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("tests/fixtures/hcm_3_2_context_resolution"),
            repo.path(),
        );
        let identity = RepositoryInvocationIdentityServiceV1::new()
            .initialize_for_setup(repo.path())
            .unwrap();
        let selection =
            crate::artifact_intake_registry::RepositoryProfileSelectionV1::from_json_bytes(
                &fs::read(repo.path().join(".handbook/profile-selection.json")).unwrap(),
            )
            .unwrap();
        let profile = resolve_profile_selection(repo.path(), selection.profile_request()).unwrap();
        let registry =
            crate::artifact_registry::ResolvedArtifactRegistry::from_profile(&profile).unwrap();
        assert_eq!(
            registry
                .intake_schema_leaf_shapes(
                    &ExactDefinitionRef::parse(
                        "handbook.schemas.context-resolution-authority-capsule@1.0.0",
                    )
                    .unwrap(),
                )
                .unwrap()
                .len(),
            1
        );

        let mappings = authority_mappings();
        let mut quorum = mappings
            .as_object()
            .unwrap()
            .values()
            .cloned()
            .collect::<Vec<_>>();
        quorum.push(json!({"approval_class":"context_resolution_authority_publisher","authority_ref":"repository_context_resolution_authority_publisher"}));
        quorum.sort_by_key(|value| value.to_string());
        let request = ApproverAdminRequestV1::from_json_value(json!({
            "schema_id":"handbook.approver-admin-request",
            "schema_version":"1.0",
            "operation":"bootstrap",
            "operation_id":"hcm-3-3-projection-authority-bootstrap",
            "repository_identity_fingerprint":identity.repository_identity_fingerprint(),
            "expected_registry_state_fingerprint":null,
            "expected_transition_fingerprint":null,
            "initial_charter_quorum":quorum
        }))
        .unwrap();
        let mut approvers =
            ApproverRegistryServiceV1::for_repository(repo.path(), Authenticator::new());
        let bootstrap = approvers
            .bootstrap_approver_registry(request)
            .to_json_value()
            .unwrap();
        assert_eq!(bootstrap["status"], "succeeded", "{bootstrap:#}");
        let registry_binding = json!({
            "state_ref":bootstrap["result_registry_state_ref"],
            "state_fingerprint":bootstrap["result_registry_state_fingerprint"],
            "head_transition_ref":bootstrap["transition_ref"],
            "head_transition_fingerprint":bootstrap["transition_fingerprint"]
        });

        let mut binding = json!({
            "schema_id":"handbook.context-resolution-authority-binding",
            "schema_version":"1.0",
            "binding_id":"handbook.context-resolution-authority.fixture",
            "binding_version":"1.0.0",
            "repository_identity_fingerprint":identity.repository_identity_fingerprint(),
            "resolved_profile":{"ref":profile.exact_ref().as_str(),"fingerprint":profile.resolved_profile_fingerprint().as_str()},
            "resolution_stack":{"ref":profile.context_resolution().exact_ref().as_str(),"fingerprint":profile.context_resolution().definition_fingerprint().as_str()},
            "approver_registry":registry_binding,
            "authority_mappings":mappings,
            "candidate_mappings":candidate_mappings(),
            "extensions":{}
        });
        let semantic_jcs = serde_json_canonicalizer::to_vec(&binding).unwrap();
        let binding_fingerprint =
            tagged_fingerprint(b"handbook.hcm-3.2.semantic-binding.v1\0", &semantic_jcs);
        binding["binding_fingerprint"] = json!(binding_fingerprint.clone());
        let payload_jcs = serde_json_canonicalizer::to_vec(&binding).unwrap();
        let payload_byte_fingerprint =
            tagged_fingerprint(b"handbook.hcm-3.2.payload-jcs.v1\0", &payload_jcs);
        let declaration = json!({
            "schema_id":"handbook.context-resolution-authority-declaration",
            "schema_version":"1.0",
            "codec_id":"handbook.hcm-3.2.context-resolution-authority-binding-jcs",
            "codec_version":"1.0",
            "payload_jcs":String::from_utf8(payload_jcs).unwrap(),
            "payload_byte_fingerprint":payload_byte_fingerprint,
            "semantic_binding_fingerprint":binding_fingerprint,
            "predecessor":{"outer_artifact_ref":null,"outer_artifact_fingerprint":null,"payload_byte_fingerprint":null,"semantic_binding_fingerprint":null},
            "prior_registry":registry_binding
        });
        let wrapper = json!({
            "declaration_jcs":String::from_utf8(serde_json_canonicalizer::to_vec(&declaration).unwrap()).unwrap()
        });
        let intake_request = serde_json::to_vec(&json!({
            "idempotency_key":"hcm33_projection_capsule_intake_0001",
            "acquisition_mode":"express",
            "expected_current_artifact_fingerprint":null,
            "coverage_submissions":[{
                "coverage_id":"context_resolution_authority.declaration_jcs",
                "state":"supplied",
                "source_kind":"user_declaration",
                "value":wrapper["declaration_jcs"],
                "specificity":"exact",
                "confidence":"high",
                "contradiction_refs":[]
            }]
        }))
        .unwrap();
        let intake = crate::artifact_mutation::ArtifactMutationServiceV1::intake_append(
            repo.path(),
            AUTHORITY_KIND_REF,
            AUTHORITY_INSTANCE_ID,
            &intake_request,
        )
        .unwrap();
        let intake_output = &intake.result.authoritative_outputs[0];
        let preview = crate::artifact_mutation::ArtifactMutationServiceV1::candidate_validate(
            repo.path(),
            AUTHORITY_KIND_REF,
            AUTHORITY_INSTANCE_ID,
            &intake_output.relative_ref,
            &intake_output.fingerprint,
            Some("absent"),
        )
        .unwrap();
        let candidate_request = serde_json::to_vec(&json!({
            "idempotency_key":"hcm33_projection_capsule_candidate_0001",
            "intake_record_ref":intake_output.relative_ref,
            "intake_record_fingerprint":intake_output.fingerprint,
            "expected_candidate_fingerprint":preview.candidate_fingerprint
        }))
        .unwrap();
        let candidate = crate::artifact_mutation::ArtifactMutationServiceV1::candidate_append(
            repo.path(),
            AUTHORITY_KIND_REF,
            AUTHORITY_INSTANCE_ID,
            &candidate_request,
        )
        .unwrap();
        let candidate_output = &candidate.result.authoritative_outputs[0];
        let canonical_bytes = crate::canonical_yaml::canonical_yaml_bytes(&wrapper).unwrap();
        let promoted = DefinitionFingerprint::from_bytes(&canonical_bytes).to_string();
        let outer_hex = promoted.strip_prefix("sha256:").unwrap();
        quorum.push(
            json!({"approval_class":"registry_admin","authority_ref":"repository_registry_admin"}),
        );
        quorum.push(json!({"approval_class":"context_resolution_authority_publication","authority_ref":format!("context-resolution-authority/{outer_hex}")}));
        quorum.sort_by_key(|value| value.to_string());
        let publisher_credential_id_hash =
            DefinitionFingerprint::from_bytes(&[0x32; 32]).to_string();
        let update = ApproverAdminRequestV1::from_json_value(json!({
            "schema_id":"handbook.approver-admin-request",
            "schema_version":"1.0",
            "operation":"update_mapping",
            "operation_id":format!("hcm32-authorize-{outer_hex}"),
            "repository_identity_fingerprint":identity.repository_identity_fingerprint(),
            "expected_registry_state_fingerprint":bootstrap["result_registry_state_fingerprint"],
            "expected_transition_fingerprint":bootstrap["transition_fingerprint"],
            "credential_id_hash":publisher_credential_id_hash,
            "approval_mappings":quorum
        }))
        .unwrap();
        let publication = approvers
            .update_approver_mapping(update)
            .to_json_value()
            .unwrap();
        assert_eq!(publication["status"], "succeeded", "{publication:#}");
        let assertion: serde_json::Value = serde_json::from_slice(
            &fs::read(
                repo.path()
                    .join(".handbook/state")
                    .join(publication["assertion_ref"].as_str().unwrap()),
            )
            .unwrap(),
        )
        .unwrap();
        let response: serde_json::Value = serde_json::from_slice(
            &fs::read(
                repo.path()
                    .join(".handbook/state")
                    .join(assertion["decoded_response_ref"].as_str().unwrap()),
            )
            .unwrap(),
        )
        .unwrap();
        let publisher_authority = json!({
            "schema_id":"handbook.context-resolution-publisher-authority",
            "schema_version":"1.0",
            "result_registry_state_ref":publication["result_registry_state_ref"],
            "result_registry_state_fingerprint":publication["result_registry_state_fingerprint"],
            "result_transition_ref":publication["transition_ref"],
            "result_transition_fingerprint":publication["transition_fingerprint"],
            "publisher_credential_id_hash":publisher_credential_id_hash,
            "challenge_jcs_base64":assertion["challenge_jcs_base64"],
            "raw_assertion_response_base64":response["raw_response_base64"]
        });
        let authenticator = approvers.into_port();
        assert_eq!(
            BASE64_STANDARD
                .decode(
                    publisher_authority["raw_assertion_response_base64"]
                        .as_str()
                        .unwrap()
                )
                .unwrap(),
            authenticator.last_response.as_ref().unwrap().as_slice()
        );
        let promotion_request = serde_json::to_vec(&json!({
            "idempotency_key":format!("hcm32crpub_{outer_hex}"),
            "candidate_ref":candidate_output.relative_ref,
            "candidate_fingerprint":candidate_output.fingerprint,
            "expected_current_artifact_fingerprint":null,
            "publisher_authority":publisher_authority
        }))
        .unwrap();
        crate::artifact_mutation::ArtifactMutationServiceV1::promote(
            repo.path(),
            AUTHORITY_KIND_REF,
            AUTHORITY_INSTANCE_ID,
            &promotion_request,
        )
        .unwrap();
        (repo, authenticator, promoted)
    }
}
