use handbook_sdk::admission_governance::*;
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
    let prepared = sdk
        .prepare_project_admission_governance(&request("before-first-send", 7_200))
        .unwrap();
    let retained = prepared.locator().canonical_json_bytes().unwrap();
    let locator = GovernanceRequestLocatorV1::from_json_bytes(&retained).unwrap();
    assert_eq!(&locator, prepared.locator());
    assert!(!root.path().join(".handbook").exists());

    let outcome = committed(sdk.initialize_project_admission_governance(prepared));
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
        let prepared = sdk.prepare_project_admission_governance(&input).unwrap();
        let locator = prepared.locator().clone();
        let original = committed(sdk.initialize_project_admission_governance(prepared));
        drop(sdk);

        let reopened = HandbookSdkV1::open(root.path());
        let expected = ExpectedGovernanceBasisV1 {
            configuration_fingerprint: original.validated.configuration_fingerprint.clone(),
            outcome_fingerprint: original.outcome_fingerprint.clone(),
        };
        let active = reopened
            .resolve_project_admission_governance(
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

        let replay = reopened.resolve_admission_governance_request(&locator);
        assert!(matches!(
            replay,
            GovernanceExecutionV1::Committed {
                delivery: GovernanceDeliveryV1::Replay,
                ..
            }
        ));
        assert_eq!(committed(replay), original);
        assert_eq!(
            reopened
                .resolve_project_admission_governance(
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
        assert!(sdk
            .prepare_project_admission_governance(&request("invalid-typed", ceiling))
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
    let prepared = sdk.prepare_project_admission_governance(&input).unwrap();
    let outcome = committed(sdk.initialize_project_admission_governance(prepared));
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
    let prepared = sdk
        .prepare_project_admission_governance(&request("lost-reply", 7_200))
        .unwrap();
    let retained_locator = prepared.locator().canonical_json_bytes().unwrap();
    let execution = sdk.initialize_project_admission_governance(prepared);
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
    let resolved = reopened.resolve_admission_governance_request(&locator);
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
        committed(reopened.resolve_admission_governance_request(&locator)),
        original_outcome
    );
    assert_eq!(retained_governance_files(root.path()), before);
}

#[test]
fn actual_sdk_refresh_and_state_reset_preserve_all_native_governance_bytes() {
    let root = tempdir().unwrap();
    fs::create_dir(root.path().join(".git")).unwrap();
    let sdk = HandbookSdkV1::open(root.path());
    let prepared = sdk
        .prepare_project_admission_governance(&request("retained-reset", 7_200))
        .unwrap();
    let locator = prepared.locator().clone();
    let original = committed(sdk.initialize_project_admission_governance(prepared));
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
    let active = reopened
        .resolve_project_admission_governance(&locator.project, &locator.request.scope, None)
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
        committed(reopened.resolve_admission_governance_request(&locator)),
        original
    );
}

#[test]
fn key_content_conflict_and_expected_basis_mismatch_preserve_the_committed_active_basis() {
    let root = tempdir().unwrap();
    let sdk = HandbookSdkV1::open(root.path());
    let input = request("immutable-key", 86_400);
    let prepared = sdk.prepare_project_admission_governance(&input).unwrap();
    let locator = prepared.locator().clone();
    let original = committed(sdk.initialize_project_admission_governance(prepared));
    let active = sdk
        .resolve_project_admission_governance(&locator.project, &locator.request.scope, None)
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
        sdk.resolve_admission_governance_request(&conflicting_locator),
        GovernanceExecutionV1::Refused { .. }
    ));

    let wrong = ExpectedGovernanceBasisV1 {
        configuration_fingerprint: FingerprintV1::parse(&format!("sha256:{}", "0".repeat(64)))
            .unwrap(),
        outcome_fingerprint: original.outcome_fingerprint.clone(),
    };
    assert!(sdk
        .resolve_project_admission_governance(
            &locator.project,
            &locator.request.scope,
            Some(&wrong),
        )
        .is_err());
    assert!(sdk
        .resolve_project_admission_governance(
            &locator.project,
            "contracts.evidence-freshness-policy/other",
            None,
        )
        .is_err());
    assert_eq!(
        sdk.resolve_project_admission_governance(&locator.project, &locator.request.scope, None,)
            .unwrap(),
        active
    );
    assert_eq!(
        committed(sdk.resolve_admission_governance_request(&locator)),
        original
    );
}

#[test]
fn existing_root_cannot_acquire_positive_new_project_eligibility() {
    let root = tempdir().unwrap();
    let sdk = HandbookSdkV1::open(root.path());
    let prepared = sdk
        .prepare_project_admission_governance(&request("existing-project", 7_200))
        .unwrap();
    fs::create_dir(root.path().join(".handbook")).unwrap();
    fs::write(root.path().join(".handbook/existing-record"), b"existing").unwrap();
    assert!(sdk
        .prepare_project_admission_governance(&request("another-request", 7_200))
        .is_err());
    assert!(matches!(
        sdk.initialize_project_admission_governance(prepared),
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
fn sdk_request_resolution_preserves_predecision_noncommit_and_decided_recovery() {
    use handbook_contracts::project_admission_governance::{
        GovernanceFaultBoundaryV1, ProjectAdmissionGovernanceV1,
    };
    for (boundary, expected_commit) in [
        (GovernanceFaultBoundaryV1::Stage(2), false),
        (GovernanceFaultBoundaryV1::Decision, true),
    ] {
        let root = tempdir().unwrap();
        let sdk = HandbookSdkV1::open(root.path());
        let prepared = sdk
            .prepare_project_admission_governance(&request("recover-original", 7_200))
            .unwrap();
        let locator = prepared.locator().clone();
        // Interrupt at the native durability seam. The selected consumer then
        // recovers the retained original locator, never an initialization retry.
        let interruption = ProjectAdmissionGovernanceV1::new()
            .with_fault_for_testing(boundary)
            .initialize(root.path(), prepared);
        assert!(matches!(&interruption, GovernanceExecutionV1::Uncertain {
            locator: original, ..
        } if original == &locator));
        let canonical = serde_json_canonicalizer::to_vec(&interruption).unwrap();
        let restored: GovernanceExecutionV1 = serde_json::from_slice(&canonical).unwrap();
        assert_eq!(restored, interruption);
        drop(sdk);

        let reopened = HandbookSdkV1::open(root.path());
        assert!(reopened
            .prepare_project_admission_governance(&request("new-attempt", 7_200))
            .is_err());
        let resolved = reopened.resolve_admission_governance_request(&locator);
        if expected_commit {
            let outcome = committed(resolved);
            assert_eq!(outcome.locator, locator);
            assert!(outcome.activation_committed);
            assert_eq!(
                reopened
                    .resolve_project_admission_governance(
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
                reopened.resolve_admission_governance_request(&locator),
                resolved
            );
            assert!(reopened
                .resolve_project_admission_governance(
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
    let prepared = sdk
        .prepare_project_admission_governance(&request("corrupt-active", 7_200))
        .unwrap();
    let locator = prepared.locator().clone();
    let original = committed(sdk.initialize_project_admission_governance(prepared));
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
    let unresolved = reopened.resolve_admission_governance_request(&locator);
    assert!(matches!(unresolved, GovernanceExecutionV1::Uncertain {
        locator: ref original, ..
    } if original == &locator));
    assert!(reopened
        .resolve_project_admission_governance(&locator.project, &locator.request.scope, None)
        .is_err());
    assert!(reopened
        .prepare_project_admission_governance(&request("replacement-identity", 7_200))
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
