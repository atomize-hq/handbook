mod admission_governance_support;
use admission_governance_support::*;
use handbook_sdk::admission_governance::*;
use handbook_sdk::admission_governance::{ProjectGovernanceExecutionV1, ProjectGovernanceLocatorV1};
use handbook_contracts::project_admission_governance::*;
use handbook_sdk::{HandbookSdkV1, SetupMode, SetupRequest};
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use tempfile::tempdir;

fn request(key: &str, ceiling: u64) -> GovernanceRequestInputV1 {
    GovernanceRequestInputV1::standalone(
        key,
        ceiling,
        "Establish this exact governance for a new local project.",
    )
}

fn committed(execution: GovernanceExecutionV1) -> GovernanceCommittedOutcomeV1 {
    match execution {
        GovernanceExecutionV1::Committed { outcome, .. } => outcome,
        other => panic!("expected a confirmed owner commit, got {other:?}"),
    }
}

#[test]
fn prepare_preserves_the_original_locator_before_any_reservation() {
    let root = tempdir().unwrap();
    let sdk = HandbookSdkV1::open(root.path());
    let prepared = prepare_initial_established(&sdk, &request("before-first-send", 7_200))
        .unwrap();
    let retained = prepared.locator().canonical_json_bytes().unwrap();
    let locator = GovernanceRequestLocatorV1::from_json_bytes(&retained).unwrap();
    assert_eq!(&locator, prepared.locator());
    assert!(!root.path().join(".handbook").exists());

    let outcome = committed(initialize_established(&sdk, prepared));
    assert_eq!(outcome.locator, locator);
    assert!(outcome.validation_confirmed);
    assert!(outcome.advisory_permission_accepted);
    assert!(outcome.activation_committed);
}

#[test]
fn both_caller_supplied_ceilings_remain_exact_through_reopen_resolution_and_replay() {
    for ceiling in [86_400, 7_200] {
        let root = tempdir().unwrap();
        let sdk = HandbookSdkV1::open(root.path());
        let input = request("establish-once", ceiling);
        let prepared = prepare_initial_established(&sdk, &input).unwrap();
        let locator = prepared.locator().clone();
        let original = committed(initialize_established(&sdk, prepared));
        drop(sdk);

        let reopened = HandbookSdkV1::open(root.path());
        let expected = ExpectedGovernanceBasisV1 {
            configuration_fingerprint: original.validated.configuration_fingerprint.clone(),
            outcome_fingerprint: original.outcome_fingerprint.clone(),
        };
        let active = current_initial_established(&reopened,
                &locator.project,
                &locator.request.scope,
                Some(&expected),
            )
            .unwrap();
        assert_eq!(active.project, locator.project);
        assert_eq!(active.configuration, input.configuration);
        assert_eq!(
            active.configuration.bindings[0].max_observation_age_seconds,
            ceiling
        );
        assert_eq!(
            active.establishment_rule,
            original.validated.establishment_rule
        );
        assert_eq!(active.owner_rule, original.validated.owner_rule);
        assert_eq!(
            active.configuration_fingerprint,
            expected.configuration_fingerprint
        );
        assert_eq!(active.outcome_fingerprint, expected.outcome_fingerprint);

        let replay = resolve_initial_established(&reopened, &locator);
        assert!(matches!(
            replay,
            GovernanceExecutionV1::Committed {
                delivery: GovernanceDeliveryV1::Replay,
                ..
            }
        ));
        assert_eq!(committed(replay), original);
        assert_eq!(
            current_initial_established(&reopened,
                    &locator.project,
                    &locator.request.scope,
                    None
                )
                .unwrap(),
            active
        );
    }
}

#[test]
fn supported_serialized_input_refuses_omitted_invalid_and_unknown_fields_before_reservation() {
    let root = tempdir().unwrap();
    let sdk = HandbookSdkV1::open(root.path());
    let original = serde_json::to_value(request("strict-input", 7_200)).unwrap();
    for invalid in [
        Value::Null,
        json!(0),
        json!(-1),
        json!(1.5),
        json!("7200"),
        json!(JSON_SAFE_INTEGER_MAX + 1),
    ] {
        let mut value = original.clone();
        value["configuration"]["bindings"][0]["max_observation_age_seconds"] = invalid;
        assert!(parse_governance_request_json(&serde_json::to_vec(&value).unwrap()).is_err());
        assert!(!root.path().join(".handbook").exists());
    }
    let mut omitted = original.clone();
    omitted["configuration"]["bindings"][0]
        .as_object_mut()
        .unwrap()
        .remove("max_observation_age_seconds");
    assert!(parse_governance_request_json(&serde_json::to_vec(&omitted).unwrap()).is_err());
    let mut unknown = original;
    unknown["fallback_ceiling"] = json!(86_400);
    assert!(parse_governance_request_json(&serde_json::to_vec(&unknown).unwrap()).is_err());
    for ceiling in [0, JSON_SAFE_INTEGER_MAX + 1] {
        assert!(prepare_initial_established(&sdk, &request("invalid-typed", ceiling))
            .is_err());
    }
    assert!(!root.path().join(".handbook").exists());
}

#[test]
fn canonical_request_fixture_keeps_the_ceiling_and_advisory_provenance_explicit() {
    let mut input = request("canonical-request", 7_200);
    input.instruction.asserted_principal = Some("local-owner".into());
    input.instruction.asserted_agent = Some("sdk-caller".into());
    let bytes = input.canonical_json_bytes().unwrap();
    let expected = r#"{"configuration":{"bindings":[{"approval_sources":[],"candidate_kind":"EvidenceFreshnessPolicyV1","exception_sources":[],"future_skew_seconds":0,"max_observation_age_seconds":7200,"owner":"handbook-contracts","require_exact_source_revision":true,"required_authored_sources":[],"rule_name":"handbook.contracts.freshness-policy-admission","rule_version":"1.0.0"}],"name":"project.admission.freshness","owner":"handbook-contracts","version":"1.0.0"},"expected_initial_state":"fresh_registration_unused","instruction":{"asserted_agent":"sdk-caller","asserted_delegation":null,"asserted_principal":"local-owner","mode":"standalone_local_advisory","text":"Establish this exact governance for a new local project."},"replacement_basis":{"explicit_local_instruction_required":true,"mode":"standalone_local_advisory","preserve_fixed_constraints":true,"require_confirmed_cutover":true,"require_exact_predecessor":true,"same_project_and_scope_only":true,"terms_immutable":true},"request_key":"canonical-request","scope":"contracts.evidence-freshness-policy/root"}"#;
    assert_eq!(bytes, expected.as_bytes());
    assert_eq!(
        GovernanceRequestInputV1::from_json_bytes(&bytes).unwrap(),
        input
    );

    let duplicate = expected.replace(
        "\"max_observation_age_seconds\":7200",
        "\"max_observation_age_seconds\":7200,\"max_observation_age_seconds\":86400",
    );
    assert!(GovernanceRequestInputV1::from_json_bytes(duplicate.as_bytes()).is_err());
    assert!(
        GovernanceRequestInputV1::from_json_bytes(format!("{expected} {{}}").as_bytes()).is_err()
    );
    assert!(GovernanceRequestInputV1::from_json_bytes(&vec![
        b' ';
        MAX_GOVERNANCE_RECORD_BYTES_V1 + 1
    ])
    .is_err());

    let root = tempdir().unwrap();
    let sdk = HandbookSdkV1::open(root.path());
    let prepared = prepare_initial_established(&sdk, &input).unwrap();
    let outcome = committed(initialize_established(&sdk, prepared));
    assert_eq!(outcome.validated.input.instruction, input.instruction);
    assert_eq!(outcome.replacement_basis, input.replacement_basis);
}

#[test]
fn lost_reply_before_the_first_consumer_serialization_byte_resolves_original_commit() {
    struct UnavailableConsumer;
    impl std::io::Write for UnavailableConsumer {
        fn write(&mut self, _bytes: &[u8]) -> std::io::Result<usize> {
            Err(std::io::Error::other(
                "consumer disconnected before serialization",
            ))
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    let root = tempdir().unwrap();
    let sdk = HandbookSdkV1::open(root.path());
    let prepared = prepare_initial_established(&sdk, &request("lost-reply", 7_200))
        .unwrap();
    let retained_locator = prepared.locator().canonical_json_bytes().unwrap();
    let execution = initialize_established(&sdk, prepared);
    assert!(matches!(
        execution,
        GovernanceExecutionV1::Committed {
            delivery: GovernanceDeliveryV1::Original,
            ..
        }
    ));
    assert!(serde_json::to_writer(UnavailableConsumer, &execution).is_err());
    drop(execution);
    drop(sdk);

    let locator = GovernanceRequestLocatorV1::from_json_bytes(&retained_locator).unwrap();
    let reopened = HandbookSdkV1::open(root.path());
    let before = retained_governance_files(root.path());
    let resolved = resolve_initial_established(&reopened, &locator);
    assert!(matches!(
        resolved,
        GovernanceExecutionV1::Committed {
            delivery: GovernanceDeliveryV1::Replay,
            ..
        }
    ));
    let original_outcome = committed(resolved);
    assert_eq!(original_outcome.locator, locator);
    assert_eq!(
        original_outcome.validated.input.configuration.bindings[0].max_observation_age_seconds,
        7_200
    );
    assert_eq!(
        committed(resolve_initial_established(&reopened, &locator)),
        original_outcome
    );
    assert_eq!(retained_governance_files(root.path()), before);
}

#[test]
fn actual_sdk_refresh_and_state_reset_preserve_all_native_governance_bytes() {
    let root = tempdir().unwrap();
    fs::create_dir(root.path().join(".git")).unwrap();
    let sdk = HandbookSdkV1::open(root.path());
    let prepared = prepare_initial_established(&sdk, &request("retained-reset", 7_200))
        .unwrap();
    let locator = prepared.locator().clone();
    let original = committed(initialize_established(&sdk, prepared));
    let before = retained_governance_files(root.path());
    assert!(!before.is_empty());
    let state = root.path().join(".handbook/state/temporary");
    fs::create_dir_all(&state).unwrap();
    fs::write(state.join("runtime.json"), b"{}\n").unwrap();

    let refresh = sdk
        .run_setup(&SetupRequest {
            mode: SetupMode::Refresh,
            rewrite: false,
            reset_state: true,
        })
        .unwrap();
    assert!(refresh.reset_applied);
    assert!(!state.exists());
    assert!(refresh
        .plan
        .reset_paths
        .iter()
        .all(|path| path.starts_with(".handbook/state/")));
    assert_eq!(retained_governance_files(root.path()), before);

    let reopened = HandbookSdkV1::open(root.path());
    let active = current_initial_established(&reopened, &locator.project, &locator.request.scope, None)
        .unwrap();
    assert_eq!(
        active.configuration.bindings[0].max_observation_age_seconds,
        7_200
    );
    assert_eq!(
        active.configuration_fingerprint,
        original.validated.configuration_fingerprint
    );
    assert_eq!(active.outcome_fingerprint, original.outcome_fingerprint);
    assert_eq!(
        committed(resolve_initial_established(&reopened, &locator)),
        original
    );
}

#[test]
fn key_content_conflict_and_expected_basis_mismatch_preserve_the_committed_active_basis() {
    let root = tempdir().unwrap();
    let sdk = HandbookSdkV1::open(root.path());
    let input = request("immutable-key", 86_400);
    let prepared = prepare_initial_established(&sdk, &input).unwrap();
    let locator = prepared.locator().clone();
    let original = committed(initialize_established(&sdk, prepared));
    let active = current_initial_established(&sdk, &locator.project, &locator.request.scope, None)
        .unwrap();
    let mut changed = input;
    changed.configuration.bindings[0].max_observation_age_seconds = 7_200;
    let conflicting_locator =
        handbook_contracts::project_admission_governance::create_request_locator(
            locator.project.clone(),
            changed,
        )
        .unwrap();
    assert_ne!(
        conflicting_locator.request_fingerprint,
        locator.request_fingerprint
    );
    assert!(matches!(
        resolve_initial_established(&sdk, &conflicting_locator),
        GovernanceExecutionV1::Refused { .. }
    ));

    let wrong = ExpectedGovernanceBasisV1 {
        configuration_fingerprint: FingerprintV1::parse(&format!("sha256:{}", "0".repeat(64)))
            .unwrap(),
        outcome_fingerprint: original.outcome_fingerprint.clone(),
    };
    assert!(current_initial_established(&sdk,
            &locator.project,
            &locator.request.scope,
            Some(&wrong),
        )
        .is_err());
    assert!(current_initial_established(&sdk,
            &locator.project,
            "contracts.evidence-freshness-policy/other",
            None,
        )
        .is_err());
    assert_eq!(
        current_initial_established(&sdk, &locator.project, &locator.request.scope, None,)
            .unwrap(),
        active
    );
    assert_eq!(
        committed(resolve_initial_established(&sdk, &locator)),
        original
    );
}

#[test]
fn existing_root_cannot_acquire_positive_new_project_eligibility() {
    let root = tempdir().unwrap();
    let sdk = HandbookSdkV1::open(root.path());
    let prepared = prepare_initial_established(&sdk, &request("existing-project", 7_200))
        .unwrap();
    fs::create_dir(root.path().join(".handbook")).unwrap();
    fs::write(root.path().join(".handbook/existing-record"), b"existing").unwrap();
    assert!(prepare_initial_established(&sdk, &request("another-request", 7_200))
        .is_err());
    assert!(matches!(
        initialize_established(&sdk, prepared),
        GovernanceExecutionV1::Refused { .. }
    ));
    assert_eq!(
        fs::read(root.path().join(".handbook/existing-record")).unwrap(),
        b"existing"
    );
    assert!(!root
        .path()
        .join(handbook_engine::REPOSITORY_IDENTITY_REPO_PATH)
        .exists());
}

#[test]
fn purpose_named_sdk_methods_do_not_change_generic_catalogs() {
    use handbook_sdk::contract_membrane::{operation_inventory, ProductionWitness, UtcTimestamp};
    let rows = operation_inventory();
    assert_eq!(rows.len(), 62);
    assert!(rows.iter().all(|row| !row
        .operation_ref()
        .as_str()
        .contains("admission-governance")));
    let witness =
        ProductionWitness::from_current(UtcTimestamp::parse("2031-01-01T00:00:00Z").unwrap())
            .unwrap();
    assert_eq!(witness.capabilities().len(), 35);
    assert_eq!(witness.schema_manifest().len(), 180);
    let fixed_use_case_results = witness
        .schema_manifest()
        .iter()
        .map(|entry| entry.schema().reference().as_str())
        .filter(|name| name.starts_with("handbook.use-case.") && name.ends_with("-result@1.0.0"))
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(
        fixed_use_case_results,
        [
            "handbook.use-case.approver-administration-result@1.0.0",
            "handbook.use-case.author-charter-result@1.0.0",
            "handbook.use-case.author-project-context-result@1.0.0",
            "handbook.use-case.pipeline-route-resolve-and-persist-result@1.0.0",
            "handbook.use-case.repository-doctor-result@1.0.0",
            "handbook.use-case.repository-setup-result@1.0.0",
            "handbook.use-case.session-grounding-boundary-result@1.0.0",
        ]
        .into_iter()
        .collect()
    );
    assert!(witness.capabilities().iter().all(|entry| !entry
        .operation()
        .reference()
        .as_str()
        .contains("admission-governance")));
    assert!(witness.schema_manifest().iter().all(|entry| !entry
        .schema()
        .reference()
        .as_str()
        .contains("admission-governance")));
}

#[test]
#[cfg(feature = "governance-test-support")]
fn sdk_request_resolution_preserves_predecision_noncommit_and_decided_recovery() {
    use handbook_contracts::project_admission_governance::GovernanceFaultBoundaryV1;
    for (boundary, expected_commit) in [
        (GovernanceFaultBoundaryV1::Stage(2), false),
        (GovernanceFaultBoundaryV1::Decision, true),
    ] {
        let root = tempdir().unwrap();
        let sdk = HandbookSdkV1::open(root.path());
        let semantic_request: InitialEstablishmentRequestV1 = serde_json::from_value(serde_json::to_value(request("recover-original", 7_200)).unwrap()).unwrap();
        let prepared = sdk.prepare_initial_project_governance(&InitialProjectGovernanceV1::Established(semantic_request)).unwrap();
        let saved = prepared.locator().clone();
        let ProjectGovernanceLocatorV1::InitializeEstablished(ref original) = saved else { panic!("wrong locator") };
        let locator: GovernanceRequestLocatorV1 = serde_json::from_value(serde_json::to_value(original).unwrap()).unwrap();
        let interruption = sdk.initialize_project_governance_with_fault_for_testing(prepared, GovernanceTestFaultV1::Initial(boundary));
        assert!(matches!(&interruption, ProjectGovernanceExecutionV1::Uncertain { locator: original, .. } if original == &saved));
        let canonical = serde_json_canonicalizer::to_vec(&interruption).unwrap();
        let restored: ProjectGovernanceExecutionV1 = serde_json::from_slice(&canonical).unwrap();
        assert_eq!(restored, interruption);
        drop(sdk);

        let reopened = HandbookSdkV1::open(root.path());
        assert!(prepare_initial_established(&reopened, &request("new-attempt", 7_200))
            .is_err());
        let resolved = resolve_initial_established(&reopened, &locator);
        if expected_commit {
            let outcome = committed(resolved);
            assert_eq!(outcome.locator, locator);
            assert!(outcome.activation_committed);
            assert_eq!(
                current_initial_established(&reopened,
                        &locator.project,
                        &locator.request.scope,
                        None,
                    )
                    .unwrap()
                    .outcome_fingerprint,
                outcome.outcome_fingerprint,
            );
        } else {
            assert!(matches!(resolved, GovernanceExecutionV1::NotCommitted {
                ref outcome,
                delivery: GovernanceDeliveryV1::Replay,
            } if outcome.locator == locator && !outcome.activation_committed));
            assert_eq!(
                resolve_initial_established(&reopened, &locator),
                resolved
            );
            assert!(current_initial_established(&reopened,
                    &locator.project,
                    &locator.request.scope,
                    None,
                )
                .is_err());
        }
    }
}

#[test]
fn sdk_does_not_infer_noncommit_or_new_registration_from_corrupt_committed_inventory() {
    use handbook_contracts::project_admission_governance::GOVERNANCE_STORAGE_PATH_V1;
    let root = tempdir().unwrap();
    let sdk = HandbookSdkV1::open(root.path());
    let prepared = prepare_initial_established(&sdk, &request("corrupt-active", 7_200))
        .unwrap();
    let locator = prepared.locator().clone();
    let original = committed(initialize_established(&sdk, prepared));
    assert!(original.activation_committed);
    fs::remove_file(
        root.path()
            .join(GOVERNANCE_STORAGE_PATH_V1)
            .join("active.json"),
    )
    .unwrap();
    let before = retained_governance_files(root.path());
    drop(sdk);

    let reopened = HandbookSdkV1::open(root.path());
    let unresolved = resolve_initial_established(&reopened, &locator);
    assert!(matches!(unresolved, GovernanceExecutionV1::Uncertain {
        locator: ref original, ..
    } if original == &locator));
    assert!(current_initial_established(&reopened, &locator.project, &locator.request.scope, None)
        .is_err());
    assert!(prepare_initial_established(&reopened, &request("replacement-identity", 7_200))
        .is_err());
    assert_eq!(retained_governance_files(root.path()), before);
}

fn retained_governance_files(root: &Path) -> BTreeMap<String, Vec<u8>> {
    fn collect(path: &Path, relative: &str, result: &mut BTreeMap<String, Vec<u8>>) {
        if path.is_dir() {
            for entry in fs::read_dir(path).unwrap() {
                let entry = entry.unwrap();
                collect(
                    &entry.path(),
                    &format!("{relative}/{}", entry.file_name().to_str().unwrap()),
                    result,
                );
            }
        } else {
            result.insert(relative.to_owned(), fs::read(path).unwrap());
        }
    }
    let mut result = BTreeMap::new();
    for entry in fs::read_dir(root).unwrap() {
        let entry = entry.unwrap();
        let name = entry.file_name().into_string().unwrap();
        if name.starts_with(".handbook-") {
            collect(&entry.path(), &name, &mut result);
        }
    }
    for entry in fs::read_dir(root.join(".handbook")).unwrap() {
        let entry = entry.unwrap();
        let name = entry.file_name().into_string().unwrap();
        if name.starts_with("admission-governance") {
            collect(&entry.path(), &name, &mut result);
        }
    }
    result
}

#[test]
fn unified_current_initial_governance_keeps_verified_original_and_replay_separate() {
    let root = tempdir().unwrap();
    let sdk = HandbookSdkV1::open(root.path());
    let input = InitialEstablishmentRequestV1::standalone("unified-initial", 7200, "Establish this exact governance for a new local project.").unwrap();
    let prepared = sdk.prepare_initial_project_governance(
        &InitialProjectGovernanceV1::Established(input.clone()),
    ).unwrap();
    let bytes = prepared.locator().canonical_json_bytes().unwrap();
    let locator = ProjectGovernanceLocatorV1::from_json_bytes(&bytes).unwrap();
    assert!(!root.path().join(".handbook").exists());
    let outcome = match sdk.initialize_project_governance(prepared) {
        ProjectGovernanceExecutionV1::Committed { outcome: ProjectGovernanceCommittedV1::InitialEstablished(outcome), delivery: GovernanceDeliveryV1::Original } => outcome,
        other => panic!("expected initial commit: {other:?}"),
    };
    let ProjectGovernanceLocatorV1::InitializeEstablished(original) = &locator else { panic!("wrong locator") };
    assert_eq!(&outcome.locator, original);
    let current = sdk.resolve_current_project_governance(
        &GovernanceProjectSelectionV1::InitiallyEstablished(original.project.clone()),
        &input.scope, None,
    ).unwrap();
    match current {
        CurrentProjectGovernanceV1::Established { origin: EstablishedGovernanceOriginV1::Initial { basis, original: verified } } => {
            assert_eq!(verified, *original);
            assert_eq!(basis.configuration, input.configuration);
            assert_eq!(basis.outcome_fingerprint, outcome.outcome_fingerprint);
        }
        other => panic!("unexpected current: {other:?}"),
    }
    assert!(matches!(sdk.resolve_project_governance_request(&locator), ProjectGovernanceExecutionV1::Committed { outcome: ProjectGovernanceCommittedV1::InitialEstablished(ref replay), delivery: GovernanceDeliveryV1::Replay } if replay == &outcome));
    let other_root = tempdir().unwrap();
    let other = HandbookSdkV1::open(other_root.path());
    assert!(!matches!(other.resolve_project_governance_request(&locator), ProjectGovernanceExecutionV1::Committed { .. }));
}

#[test]
fn unified_deferred_to_established_preserves_current_origin_and_originals() {
    let root = tempdir().unwrap();
    let sdk = HandbookSdkV1::open(root.path());
    let input = InitialDeferralRequestV1::human_approved("unified-deferred", "Approve deferring this exact scope.").unwrap();
    let prepared = sdk.prepare_initial_project_governance(&InitialProjectGovernanceV1::Deferred(input.clone())).unwrap();
    let locator = ProjectGovernanceLocatorV1::from_json_bytes(&prepared.locator().canonical_json_bytes().unwrap()).unwrap();
    assert!(!root.path().join(".handbook").exists());
    let outcome = match sdk.initialize_project_governance(prepared) {
        ProjectGovernanceExecutionV1::Committed { outcome: ProjectGovernanceCommittedV1::InitialDeferred(outcome), delivery: GovernanceDeliveryV1::Original } => outcome,
        other => panic!("expected deferral: {other:?}"),
    };
    let ProjectGovernanceLocatorV1::InitializeDeferred(original) = &locator else { panic!("wrong locator") };
    let binding = GovernanceProjectSelectionV1::InitiallyDeferred(original.project.clone());
    let deferred = match sdk.resolve_current_project_governance(&binding, &input.scope, None).unwrap() {
        CurrentProjectGovernanceV1::Deferred { basis, original: verified } => { assert_eq!(verified, *original); basis }
        other => panic!("expected deferred: {other:?}"),
    };
    assert_eq!(deferred.outcome_fingerprint, outcome.outcome_fingerprint);
    let expected = DeferredRevisionV1 { outcome_fingerprint: deferred.outcome_fingerprint.clone(), relation_fingerprint: deferred.relation_fingerprint.clone() };
    let establishment = GovernanceEstablishmentRequestV1::human_approved(original.project.clone(), expected.clone(), 7200, "unified-establishment", "Approve this exact configuration.").unwrap();
    let prepared = sdk.prepare_project_governance_establishment(&establishment).unwrap();
    let establishment_locator = ProjectGovernanceLocatorV1::from_json_bytes(&prepared.locator().canonical_json_bytes().unwrap()).unwrap();
    let result = sdk.establish_project_governance(prepared);
    let established = match result {
        ProjectGovernanceExecutionV1::Committed { outcome: ProjectGovernanceCommittedV1::EstablishedFromDeferred(outcome), delivery: GovernanceDeliveryV1::Original } => outcome,
        other => panic!("expected establishment: {other:?}"),
    };
    match sdk.resolve_current_project_governance(&binding, &input.scope, None).unwrap() {
        CurrentProjectGovernanceV1::Established { origin: EstablishedGovernanceOriginV1::FromDeferred { basis, original: verified } } => {
            assert_eq!(verified, *original);
            assert_eq!(basis.predecessor, expected);
            assert_eq!(basis.locator, established.locator);
            assert_eq!(basis.configuration, establishment.configuration);
            assert_eq!(basis.outcome_fingerprint, established.outcome_fingerprint);
        }
        other => panic!("expected established: {other:?}"),
    }
    assert!(sdk.resolve_current_project_governance(&binding, &input.scope, Some(&ExpectedCurrentGovernanceV1::Deferred(expected))).is_err());
    assert!(matches!(sdk.resolve_project_governance_request(&locator), ProjectGovernanceExecutionV1::Committed { outcome: ProjectGovernanceCommittedV1::InitialDeferred(ref replay), delivery: GovernanceDeliveryV1::Replay } if replay == &outcome));
    assert!(matches!(sdk.resolve_project_governance_request(&establishment_locator), ProjectGovernanceExecutionV1::Committed { outcome: ProjectGovernanceCommittedV1::EstablishedFromDeferred(ref replay), delivery: GovernanceDeliveryV1::Replay } if replay == &established));
    assert!(matches!(sdk.resolve_current_project_governance(&binding, &input.scope, None).unwrap(), CurrentProjectGovernanceV1::Established { origin: EstablishedGovernanceOriginV1::FromDeferred { .. } }));
}

#[test]
#[cfg(feature = "governance-test-support")]
fn unified_resolver_distinguishes_interrupted_owner_decisions_and_changed_original() {
    use handbook_contracts::project_admission_governance::GovernanceFaultBoundaryV1;
    for (boundary, commits) in [(GovernanceFaultBoundaryV1::Stage(2), false), (GovernanceFaultBoundaryV1::Decision, true)] {
        let root = tempdir().unwrap();
        let sdk = HandbookSdkV1::open(root.path());
        let request: InitialEstablishmentRequestV1 = serde_json::from_value(serde_json::to_value(request("unified-interrupted", 7200)).unwrap()).unwrap();
        let prepared = sdk.prepare_initial_project_governance(&InitialProjectGovernanceV1::Established(request)).unwrap();
        let saved = prepared.locator().clone();
        let restored = ProjectGovernanceLocatorV1::from_json_bytes(&saved.canonical_json_bytes().unwrap()).unwrap();
        let interruption = sdk.initialize_project_governance_with_fault_for_testing(prepared, GovernanceTestFaultV1::Initial(boundary));
        assert!(matches!(interruption, ProjectGovernanceExecutionV1::Uncertain { ref locator, .. } if locator == &saved));
        let resolved = sdk.resolve_project_governance_request(&restored);
        if commits {
            assert!(matches!(resolved, ProjectGovernanceExecutionV1::Committed { outcome: ProjectGovernanceCommittedV1::InitialEstablished(ref outcome), delivery: GovernanceDeliveryV1::Replay } if outcome.activation_committed));
        } else {
            assert!(matches!(resolved, ProjectGovernanceExecutionV1::NotCommitted { outcome: ProjectGovernanceNotCommittedV1::InitialEstablished(ref outcome), delivery: GovernanceDeliveryV1::Replay } if !outcome.activation_committed));
        }
        let ProjectGovernanceLocatorV1::InitializeEstablished(mut changed) = restored else { unreachable!() };
        changed.request.instruction.text.push_str(" changed");
        assert!(matches!(sdk.resolve_project_governance_request(&ProjectGovernanceLocatorV1::InitializeEstablished(changed)), ProjectGovernanceExecutionV1::Refused { .. }));
    }
}

#[test]
fn unified_sdk_lost_reply_recovers_saved_original_without_reinitializing() {
    let root = tempdir().unwrap();
    let sdk = HandbookSdkV1::open(root.path());
    let request = InitialEstablishmentRequestV1::standalone("discarded-sdk-reply", 7200, "Approve exact initial governance.").unwrap();
    let prepared = sdk.prepare_initial_project_governance(&InitialProjectGovernanceV1::Established(request)).unwrap();
    let saved = ProjectGovernanceLocatorV1::from_json_bytes(&prepared.locator().canonical_json_bytes().unwrap()).unwrap();
    drop(sdk.initialize_project_governance(prepared));
    let reopened = HandbookSdkV1::open(root.path());
    let replay = reopened.resolve_project_governance_request(&saved);
    assert!(matches!(replay, ProjectGovernanceExecutionV1::Committed { outcome: ProjectGovernanceCommittedV1::InitialEstablished(_), delivery: GovernanceDeliveryV1::Replay }));
    assert_eq!(reopened.resolve_project_governance_request(&saved), replay);
}
