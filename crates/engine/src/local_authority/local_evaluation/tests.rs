use super::*;

fn fp(digit: &str) -> String {
    format!("sha256:{}", digit.repeat(64))
}
fn supplied() -> (
    LocalEvaluationTermsV1,
    LocalEvaluationAuthorizationRequestV1,
) {
    let ceiling = LocalEnvelopeCeilingV1 {
        active_level_id: "execution".into(),
        dimensions: [
            "assigned_unit",
            "normal",
            "immediate",
            "local_write",
            "execution",
            "unit_closeout",
        ]
        .map(str::to_owned),
    };
    let mut terms = LocalEvaluationTermsV1 {
        request_key: "approved-evaluation-terms".into(),
        rule_ref: LOCAL_EVALUATION_RULE_V1.into(),
        principal: "human".into(),
        agent: "local-agent".into(),
        delegation: "complete captured check".into(),
        instruction: "Perform the supplied bounded check".into(),
        approval_source_ref: "human.supplied-evaluation-terms".into(),
        human_approval_asserted: true,
        approved_content_fingerprint: String::new(),
        expected_predecessor_fingerprint: fp("1"),
        expected_charter_fingerprint: fp("2"),
        ceiling,
        constraint_inputs: Vec::new(),
        allowed_paths: vec![
            format!("{LOCAL_EVALUATION_ROOT_V1}/**"),
            ".handbook/state/contracts/**".into(),
            ".handbook/state/dock/**".into(),
        ],
        allowed_input_paths: vec![
            ".handbook-initialization-receipt.v1.json".into(),
            ".handbook-initialization.lock".into(),
            ".handbook-initialization.v1.json".into(),
            ".handbook/**".into(),
            "inputs/*".into(),
        ],
        allowed_use_intents: vec![
            LocalEvaluationUseIntentV1::DecisionSeeking,
            LocalEvaluationUseIntentV1::Diagnostic,
        ],
        required_checkpoints: required_checkpoints(),
        retention_until_utc: "2099-01-01T00:00:00Z".into(),
        revocation_principal: "human".into(),
    };
    terms.approved_content_fingerprint = terms.fingerprint_approved_content().unwrap();
    let requested = LocalEvaluationReachV1::selected(&terms.ceiling).unwrap();
    let plan = LocalEvaluationPlanV1 {
        schema_ref: LOCAL_EVALUATION_PLAN_SCHEMA_V1.into(),
        requirements_fingerprint: fp("3"),
        selections: vec![LocalEvaluationSelectionV1 {
            selection_id: "native-schema".into(),
            role_ref: LOCAL_EVALUATION_ROLE_V1.into(),
            producer_ref: LOCAL_EVALUATION_PRODUCER_V1.into(),
            configuration_fingerprint: fp("4"),
            capability_fingerprint: fp("5"),
            operating_ceiling_fingerprint: fp("6"),
            host_prerequisites_fingerprint: fp("7"),
            requested_reach: requested,
        }],
        composition: "single".into(),
        max_attempts: 1,
    };
    let mut purpose = LocalEvaluationPurposeV1 {
        schema_ref: LOCAL_EVALUATION_PURPOSE_SCHEMA_V1.into(),
        owner_identity: "contracts-owner".into(),
        project_identity: "project-identity".into(),
        occurrence_identity: fp("8"),
        requirements_fingerprint: plan.requirements_fingerprint.clone(),
        requirements_source: "direct_contract".into(),
        contract: LocalBindingV1 {
            reference: "contract.orders@3.0.0".into(),
            fingerprint: fp("9"),
        },
        subject_ref: "inputs/order.json".into(),
        subject_fingerprint: fp("a"),
        scope_fingerprint: fp("b"),
        objective: "Does this exact captured JSON conform to the exact schema?".into(),
        permitted_use: "decision_seeking_eligible".into(),
        use_intent: LocalEvaluationUseIntentV1::DecisionSeeking,
        plan_fingerprint: plan.fingerprint().unwrap(),
        formation_policy_fingerprint: fp("c"),
        evaluation_rule_fingerprint: fp("d"),
        time_rules_fingerprint: fp("e"),
        retention_until_utc: "2098-01-01T00:00:00Z".into(),
        content_fingerprint: String::new(),
        formed_at_utc: "2026-10-06T10:00:00Z".into(),
    };
    purpose.content_fingerprint = purpose.fingerprint_semantic_content().unwrap();
    let request = LocalEvaluationAuthorizationRequestV1 {
        request_key: "first-complete-check".into(),
        envelope: LocalEvaluationEnvelopeV1::local(
            &purpose.subject_ref,
            LocalEvaluationReachV1::selected(&terms.ceiling).unwrap(),
            &terms,
        ),
        purpose,
        plan,
        current_basis: LocalCheckCurrentBasisV1 {
            relation_fingerprint: fp("1"),
            governance_tree_fingerprint: fp("2"),
        },
        reach: LocalEvaluationPathReachV1 {
            owner_write_paths: vec![
                LOCAL_EVALUATION_ROOT_V1.into(),
                ".handbook/state/contracts".into(),
                ".handbook/state/dock".into(),
            ],
            input_paths: terms.allowed_input_paths.clone(),
        },
        retention_until_utc: "2098-01-01T00:00:00Z".into(),
    };
    (terms, request)
}
fn on_stack(test: impl FnOnce() + Send + 'static) {
    std::thread::Builder::new()
        .stack_size(8 * 1024 * 1024)
        .spawn(test)
        .unwrap()
        .join()
        .unwrap();
}

#[test]
fn complete_purpose_plan_request_closes_and_changed_selection_or_role_refuses() {
    on_stack(|| {
        let (terms, request) = supplied();
        request.validate_under(&terms).unwrap();
        let mut changed = request.clone();
        changed.plan.selections[0].configuration_fingerprint = fp("f");
        assert_eq!(
            changed.validate_under(&terms).unwrap_err().code,
            "request_congruence"
        );
        changed = request.clone();
        changed.plan.selections[0].role_ref = "runtime_side_effects".into();
        assert_eq!(
            changed.validate_under(&terms).unwrap_err().code,
            "selection"
        );
        changed = request;
        changed.plan.max_attempts = 2;
        assert_eq!(changed.validate_under(&terms).unwrap_err().code, "plan");
    });
}

#[test]
fn explicit_engine_mutation_and_proof_read_reach_precede_any_repository_effect() {
    on_stack(|| {
        let (terms, request) = supplied();
        let mut omitted = request.clone();
        omitted.reach.owner_write_paths.remove(0);
        assert_eq!(
            omitted.validate_under(&terms).unwrap_err().code,
            "required_owner_subtree"
        );
        omitted = request.clone();
        omitted.reach.input_paths.retain(|p| p != ".handbook/**");
        assert_eq!(
            omitted.validate_under(&terms).unwrap_err().code,
            "required_owner_subtree"
        );
        omitted = request;
        omitted.reach.owner_write_paths[0] =
            ".handbook/local-operating/local-evaluation-v1/issuances".into();
        assert_eq!(
            omitted.validate_under(&terms).unwrap_err().code,
            "required_owner_subtree"
        );
    });
}

#[test]
fn private_envelope_constraints_and_mutation_rules_are_independently_checked() {
    on_stack(|| {
        let (terms, request) = supplied();
        let mut changed = request.clone();
        changed.envelope.constraint_inputs.push(LocalBindingV1 {
            reference: "constraint.secret@1.0.0".into(),
            fingerprint: fp("f"),
        });
        assert_eq!(
            changed.validate_under(&terms).unwrap_err().code,
            "private_envelope"
        );
        changed = request.clone();
        changed
            .envelope
            .mutation_rules
            .retain(|r| r.selector != ".handbook/state/contracts/**");
        assert_eq!(
            changed.validate_under(&terms).unwrap_err().code,
            "envelope_mutation"
        );
        changed = request;
        changed
            .envelope
            .mutation_rules
            .push(LocalEvaluationMutationRuleV1 {
                rule_id: "deny-contracts".into(),
                effect: "deny".into(),
                target_kind: "repository_path".into(),
                selector: ".handbook/state/contracts/**".into(),
            });
        assert_eq!(
            changed.validate_under(&terms).unwrap_err().code,
            "envelope_mutation"
        );
    });
}

#[test]
fn whole_owner_subtree_refuses_every_intersecting_descendant_deny() {
    on_stack(|| {
        let (terms, request) = supplied();
        for selector in [
            ".handbook/state/contracts/records/private.json",
            ".handbook/state/contracts/records/**",
            ".handbook/state/contracts/*/private.json",
            ".handbook/*/contracts/records/*.json",
        ] {
            let mut denied = request.clone();
            denied
                .envelope
                .mutation_rules
                .push(LocalEvaluationMutationRuleV1 {
                    rule_id: "deny-descendant".into(),
                    effect: "deny".into(),
                    target_kind: "repository_path".into(),
                    selector: selector.into(),
                });
            assert_eq!(
                denied.validate_under(&terms).unwrap_err().code,
                "envelope_mutation",
                "{selector}"
            );
        }
        let mut unrelated = request;
        unrelated
            .envelope
            .mutation_rules
            .push(LocalEvaluationMutationRuleV1 {
                rule_id: "deny-unrelated".into(),
                effect: "deny".into(),
                target_kind: "repository_path".into(),
                selector: ".handbook/state/other-owner/**".into(),
            });
        unrelated.validate_under(&terms).unwrap();
    });
}

#[test]
fn producer_resolution_must_equal_exact_admitted_selection_without_alias_or_stack_substitution() {
    on_stack(|| {
        let (_, request) = supplied();
        let selected = &request.plan.selections[0].requested_reach;
        let ids = [
            "resolution.scope.assigned-unit",
            "resolution.detail.normal",
            "resolution.time.immediate",
            "resolution.authority.local-write",
            "resolution.memory.execution",
            "resolution.validation.unit-closeout",
        ];
        require_exact_resolution_projection(selected, &selected.stack, ids).unwrap();
        for (axis, id) in [
            (0, "resolution.scope.project"),
            (0, "resolution.scope.assigned_unit"),
            (1, "resolution.detail.summary"),
            (2, "resolution.time.current-slice"),
            (2, "resolution.temporal.immediate"),
            (3, "resolution.authority.slice-write"),
            (4, "resolution.memory.coordination"),
            (5, "resolution.validation.slice-closeout"),
        ] {
            let mut changed = ids;
            changed[axis] = id;
            assert_eq!(
                require_exact_resolution_projection(selected, &selected.stack, changed)
                    .unwrap_err()
                    .code,
                "requested_resolution_congruence"
            );
        }
        let mut wrong_stack = selected.stack.clone();
        wrong_stack.fingerprint = fp("f");
        assert_eq!(
            require_exact_resolution_projection(selected, &wrong_stack, ids)
                .unwrap_err()
                .code,
            "requested_resolution_congruence"
        );

        // Use the shipped native Dock projection as the independent positive
        // source: the serialized temporal key uses the resolution.time domain.
        let manifest: serde_json::Value = serde_json::from_str(include_str!(
            "../../../../dock/production-manifests/x86_64-pc-windows-msvc/manifest.json"
        ))
        .unwrap();
        let ceiling = &manifest["resolution_ceiling"];
        let native_ids = [
            "scope_horizon",
            "detail_resolution",
            "temporal_horizon",
            "authority_horizon",
            "memory_horizon",
            "validation_horizon",
        ]
        .map(|key| ceiling[key].as_str().unwrap());
        let native_stack = LocalBindingV1 {
            reference: ceiling["resolution_stack"]["ref"].as_str().unwrap().into(),
            fingerprint: ceiling["resolution_stack"]["fingerprint"]
                .as_str()
                .unwrap()
                .into(),
        };
        let native_reach = LocalEvaluationReachV1::selected(&LocalEnvelopeCeilingV1 {
            active_level_id: "execution".into(),
            dimensions: [
                "local_observation",
                "identifier_only",
                "current_operation",
                "read_only",
                "operation",
                "observation_only",
            ]
            .map(str::to_owned),
        })
        .unwrap();
        require_exact_resolution_projection(&native_reach, &native_stack, native_ids).unwrap();
        let mut broader_detail = native_ids;
        broader_detail[1] = "resolution.detail.normal";
        assert_eq!(
            require_exact_resolution_projection(&native_reach, &native_stack, broader_detail)
                .unwrap_err()
                .code,
            "requested_resolution_congruence"
        );
    });
}

#[test]
fn full_read_selector_requires_declared_and_approved_population_coverage() {
    let declared = vec![".handbook/**".into(), "handbook-docks/**".into()];
    let approved = declared.clone();
    for selector in [".handbook/state/**", "handbook-docks/**"] {
        require_input_selector_coverage(&declared, &approved, selector).unwrap();
        assert!(
            require_read(&declared, selector).is_err(),
            "a selector is not an exact file-read target"
        );
    }
    require_input_selector_coverage(
        &declared,
        &approved,
        ".handbook/state/contracts/retained.json",
    )
    .unwrap();
    for incomplete in [
        vec![".handbook/state".into()],
        vec![".handbook/state/*".into()],
        vec![".handbook/state/contracts/**".into()],
    ] {
        assert_eq!(
            require_input_selector_coverage(&incomplete, &approved, ".handbook/state/**")
                .unwrap_err()
                .code,
            "required_input_reach"
        );
    }
    let approved_narrow = vec![".handbook/state/contracts/**".into()];
    assert_eq!(
        require_input_selector_coverage(&declared, &approved_narrow, ".handbook/state/**")
            .unwrap_err()
            .code,
        "path_expansion"
    );
    assert!(
        require_input_selector_coverage(&declared, &approved, ".handbook/state/../outside/**")
            .is_err()
    );
}

#[test]
fn semantic_content_does_not_collapse_occurrence_time_policy_or_retention_identity() {
    on_stack(|| {
        let (_, request) = supplied();
        let purpose = request.purpose;
        let mut later = purpose.clone();
        later.occurrence_identity = fp("f");
        later.formed_at_utc = "2026-10-06T10:01:00Z".into();
        assert_eq!(
            purpose.fingerprint_semantic_content().unwrap(),
            later.fingerprint_semantic_content().unwrap()
        );
        assert_ne!(purpose.fingerprint().unwrap(), later.fingerprint().unwrap());
        later = purpose.clone();
        later.formation_policy_fingerprint = fp("f");
        assert_ne!(
            purpose.fingerprint_semantic_content().unwrap(),
            later.fingerprint_semantic_content().unwrap()
        );
        later = purpose.clone();
        later.retention_until_utc = "2097-01-01T00:00:00Z".into();
        assert_ne!(
            purpose.fingerprint_semantic_content().unwrap(),
            later.fingerprint_semantic_content().unwrap()
        );
    });
}

#[test]
fn diagnostic_incompatibility_horizon_and_omitted_checkpoints_refuse() {
    on_stack(|| {
        let (mut terms, mut request) = supplied();
        request.purpose.permitted_use = "diagnostic_only".into();
        request.purpose.content_fingerprint =
            request.purpose.fingerprint_semantic_content().unwrap();
        assert_eq!(request.validate().unwrap_err().code, "purpose");
        request.purpose.use_intent = LocalEvaluationUseIntentV1::Diagnostic;
        request.purpose.content_fingerprint =
            request.purpose.fingerprint_semantic_content().unwrap();
        request.validate_under(&terms).unwrap();
        terms.required_checkpoints.pop();
        terms.approved_content_fingerprint = terms.fingerprint_approved_content().unwrap();
        assert_eq!(terms.validate().unwrap_err().code, "evaluation_terms");
        assert_eq!(
            horizon("2026-10-06T10:00:00Z", "2026-10-06T10:00:01Z")
                .unwrap_err()
                .code,
            "retention_horizon"
        );
    });
}

#[test]
fn final_and_launch_checkpoints_cannot_omit_exact_consuming_target() {
    assert_eq!(
        validate_checkpoint(LocalEvaluationCheckpointV1::FinalContext, None)
            .unwrap_err()
            .code,
        "checkpoint_target"
    );
    assert_eq!(
        validate_checkpoint(LocalEvaluationCheckpointV1::Launch, None)
            .unwrap_err()
            .code,
        "checkpoint_target"
    );
    validate_checkpoint(LocalEvaluationCheckpointV1::FinalContext, Some(&fp("1"))).unwrap();
}

#[test]
fn captured_logical_subject_uses_actual_native_proof_reach_and_exact_fingerprint() {
    on_stack(|| {
        let (terms, mut request) = supplied();
        request.purpose.subject_ref = format!(
            "handbook.dock.captured-input@1.0.0#role/handbook.input.json-schema-validation/{}",
            &request.purpose.subject_fingerprint[7..]
        );
        request.purpose.content_fingerprint =
            request.purpose.fingerprint_semantic_content().unwrap();
        request.envelope.objective_ref = request.purpose.subject_ref.clone();
        request.validate_under(&terms).unwrap();
        let mut wrong = request.clone();
        wrong.purpose.subject_ref.push('0');
        wrong.purpose.content_fingerprint = wrong.purpose.fingerprint_semantic_content().unwrap();
        wrong.envelope.objective_ref = wrong.purpose.subject_ref.clone();
        assert_eq!(
            wrong.validate_under(&terms).unwrap_err().code,
            "captured_subject"
        );
        let no_dock = vec!["inputs/*".into()];
        assert_eq!(
            require_subject_reach(
                &no_dock,
                &request.purpose.subject_ref,
                &request.purpose.subject_fingerprint
            )
            .unwrap_err()
            .code,
            "required_owner_subtree"
        );
    });
}

fn fixed_set_source(
    request: &LocalEvaluationAuthorizationRequestV1,
) -> LocalEvaluationFinalSetSourceV1 {
    let identity = fp("f");
    let set = fp("5");
    let occurrence = serde_json::json!({"ref":"handbook.local-evaluation-occurrence@3.0.0#owner/1","fingerprint":request.purpose.occurrence_identity});
    let basis = serde_json::json!({"ref":"handbook.local-evaluation-basis@3.0.0#basis/1","fingerprint":fp("6")});
    let value = serde_json::json!({"stage":"set_closed","identity":{"ref":"handbook.local-evaluation-state@3.0.0#state/1","fingerprint":identity},
        "purpose":request.purpose,"plan":request.plan,"locator":{"occurrence":occurrence},"committed_basis":basis,
        "evidence_set":{"identity":{"ref":"handbook.local-evaluation-set@3.0.0#set/1","fingerprint":set},"occurrence":occurrence,"basis":basis,
            "formation_policy":{"ref":"handbook.local-evaluation-formation-policy@3.0.0","fingerprint":request.purpose.formation_policy_fingerprint},"closed_at":"2026-10-06T10:00:00Z"},
        "original_input":{"formation":{"retention_until_utc":request.retention_until_utc}}});
    let bytes = encode(&value).unwrap();
    LocalEvaluationFinalSetSourceV1 {
        relative_path: format!(
            ".handbook/state/contracts/local-evaluation-v3/records/{}.json",
            &identity[7..]
        ),
        record_fingerprint: DefinitionFingerprint::from_bytes(&bytes).to_string(),
        canonical_record_bytes: bytes,
        set_fingerprint: set,
    }
}

#[test]
fn final_source_requires_already_retained_exact_native_set_bytes_before_verification() {
    on_stack(|| {
        let (_, request) = supplied();
        let source = fixed_set_source(&request);
        source.validate(&request).unwrap();
        let dir = tempfile::tempdir().unwrap();
        let root = RetainedRepositoryRootV1::open_exact(dir.path()).unwrap();
        assert!(pin_final_set_source(&root, &source, &request, &request.reach).is_err());
        std::fs::create_dir_all(
            dir.path()
                .join(".handbook/state/contracts/local-evaluation-v3/records"),
        )
        .unwrap();
        std::fs::write(
            dir.path().join(&source.relative_path),
            &source.canonical_record_bytes,
        )
        .unwrap();
        let checkpoint = pin_final_set_source(&root, &source, &request, &request.reach).unwrap();
        checkpoint.validate().unwrap();
        let mut substituted = source.clone();
        substituted.canonical_record_bytes.push(b' ');
        assert_eq!(
            substituted.validate(&request).unwrap_err().code,
            "final_set_source"
        );
        let mut changed = request.clone();
        changed.purpose.occurrence_identity = fp("e");
        changed.purpose.content_fingerprint =
            changed.purpose.fingerprint_semantic_content().unwrap();
        assert_eq!(
            source.validate(&changed).unwrap_err().code,
            "final_set_congruence"
        );
        let no_read = LocalEvaluationPathReachV1::default();
        assert_eq!(
            pin_final_set_source(&root, &source, &request, &no_read)
                .err()
                .unwrap()
                .code,
            "required_input_reach"
        );
    });
}

#[test]
fn pinned_issuance_tuple_remains_exact_across_repeated_use_and_source_substitution() {
    on_stack(|| {
        // These closed records are untrusted fixture inputs, not an admission.
        // Exercise the native pin/congruence boundary without granting authority.
        let (_, request) = supplied();
        let original = LocalEvaluationBindingLocatorV1 {
            request_key: request.request_key.clone(),
            request_fingerprint: request.fingerprint().unwrap(),
            authority_fingerprint: fp("1"),
        };
        let intent = IssuanceIntent {
            schema_ref: LOCAL_EVALUATION_RULE_V1.into(),
            locator: original.clone(),
            request: request.clone(),
            issuance_time_utc: "2026-10-06T10:01:00Z".into(),
            operating_lock_fingerprint: fp("2"),
            authority_owner_identity: fp("3"),
        };
        let binding = expected_binding(&intent).unwrap();
        let dir = tempfile::tempdir().unwrap();
        let paths = issuance_paths(&original);
        for file in ["request.json", "outcome.json", "terminal.json"] {
            let path = dir.path().join(issuance_path(&original, file));
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            let bytes = if file == "request.json" {
                encode(&intent).unwrap()
            } else {
                encode(&binding).unwrap()
            };
            std::fs::write(path, bytes).unwrap();
        }
        let root = RetainedRepositoryRootV1::open_exact(dir.path()).unwrap();
        let refs = paths.iter().map(String::as_str).collect::<Vec<_>>();
        let files = root.retain_exact_files_checkpoint_v1(&refs).unwrap();
        for _ in 0..2 {
            let retained = require_pinned_issuance_tuple(
                &root,
                &files,
                &original,
                &request,
                &binding,
                (request.clone(), binding.clone()),
            )
            .unwrap();
            assert_eq!(retained, (request.clone(), binding.clone()));
            files.validate().unwrap();
        }
        let mut substituted = request.clone();
        substituted.purpose.objective.push_str(" substituted");
        assert_eq!(
            require_pinned_issuance_tuple(
                &root,
                &files,
                &original,
                &request,
                &binding,
                (substituted, binding.clone()),
            )
            .unwrap_err()
            .code,
            "issuance_congruence"
        );
        let outcome = dir.path().join(issuance_path(&original, "outcome.json"));
        let mut changed_binding = binding.clone();
        changed_binding.plan_fingerprint = fp("4");
        let changed_bytes = encode(&changed_binding).unwrap();
        if std::fs::write(&outcome, &changed_bytes).is_ok() {
            assert!(files.validate().is_err());
            assert!(require_pinned_issuance_tuple(
                &root,
                &files,
                &original,
                &request,
                &binding,
                (request.clone(), binding.clone()),
            )
            .is_err());
        } else {
            // Windows denies mutation of the file while native read pins live.
            files.validate().unwrap();
        }
        drop(files);
        // A pin established after an early read must not bless substituted bytes.
        std::fs::write(&outcome, changed_bytes).unwrap();
        let substituted_files = root.retain_exact_files_checkpoint_v1(&refs).unwrap();
        assert!(require_pinned_issuance_tuple(
            &root,
            &substituted_files,
            &original,
            &request,
            &binding,
            (request.clone(), binding.clone()),
        )
        .is_err());
    });
}
