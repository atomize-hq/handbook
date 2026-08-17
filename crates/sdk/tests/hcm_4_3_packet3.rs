use handbook_sdk::contract_membrane::{
    canonical_json_bytes, AuthorInputTransportRouteV1, AuthorWorkingDirectoryCommandV1,
    ContractLimits, InvocationResponseV1, TerminalStatus, TransportFailureKind, TransportFailureV1,
    TransportIoErrorV1, TransportPresentationViewV1,
};
use handbook_sdk::{
    ApproverCommandRequest, ApproverMapping, ApproverOperation, ApproverOperationResult,
    AuthorCausalDisposition, AuthorOperationRefusal, AuthorOperationStatus,
    AuthorProjectContextResult, AuthorPublicEffect, AuthorPublicEffectDisposition,
    CharterCommandRequest, CharterOperation, CharterOperationResult, HandbookSdkV1,
    ProjectContextCommandOutcome, ProjectContextCommandRequest, ProjectContextInput,
    RepositoryInvocationFailureView,
};

#[test]
fn charter_refusal_is_one_closed_use_case_result_before_projection() {
    let repository = tempfile::tempdir().unwrap();
    let limits = ContractLimits::packet0();
    let invocation = HandbookSdkV1::open(repository.path())
        .run_author_charter_use_case(CharterCommandRequest::Validate, &limits)
        .unwrap();

    assert_eq!(invocation.terminal_status(), TerminalStatus::Blocked);
    let document = serde_json::to_value(&invocation).unwrap();
    assert_eq!(
        document["response"]["use_case"]["ref"],
        "handbook.use-case.author-charter@1.0.0"
    );
    assert_eq!(
        document["response"]["result_schema"]["ref"],
        "handbook.use-case.author-charter-result@1.0.0"
    );

    let bytes = canonical_json_bytes(&invocation, &limits).unwrap();
    let admitted = InvocationResponseV1::parse_and_admit(&bytes, None, &limits).unwrap();
    let outcome = admitted
        .author_charter_outcome(&limits)
        .unwrap()
        .expect("author-charter outcome");
    assert_eq!(outcome.operation, CharterOperation::Validate);
    assert_eq!(outcome.refusal.unwrap().code, "canonical_charter_missing");
}

#[test]
fn project_context_validation_refusal_round_trips_input_presentation_facts() {
    let repository = tempfile::tempdir().unwrap();
    let limits = ContractLimits::packet0();
    let input = ProjectContextInput {
        schema_id: "handbook.project-context".to_owned(),
        schema_version: "1.0".to_owned(),
        record_id: "project_context".to_owned(),
        summary: String::new(),
        system_boundaries: Vec::new(),
        ownership: Vec::new(),
        authoritative_references: Vec::new(),
        known_unknowns: Vec::new(),
    };
    let use_case = HandbookSdkV1::open(repository.path())
        .run_author_project_context_use_case(
            "file",
            "context.yaml",
            ProjectContextCommandRequest::Validate(input),
            &limits,
        )
        .unwrap();
    assert_eq!(use_case.status(), TerminalStatus::Refused);
    let invocation = InvocationResponseV1::use_case(use_case, &limits).unwrap();
    let outcome = invocation
        .author_project_context_outcome(&limits)
        .unwrap()
        .expect("project-context outcome");
    match outcome {
        ProjectContextCommandOutcome::Validate {
            input_mode,
            input_source,
            result: Err(refusal),
        } => {
            assert_eq!(input_mode, "file");
            assert_eq!(input_source, "context.yaml");
            assert_eq!(
                refusal.kind,
                handbook_sdk::AuthorProjectContextRefusalKind::IncompleteStructuredInput
            );
        }
        other => panic!("unexpected outcome: {other:?}"),
    }
}

#[test]
fn cli_owned_charter_refusal_uses_the_same_closed_detail_root() {
    let limits = ContractLimits::packet0();
    let result = CharterOperationResult::invalid_request(CharterOperation::Author);
    let use_case =
        handbook_sdk::contract_membrane::UseCaseResponseV1::author_charter(&result, &limits)
            .unwrap();
    let invocation = InvocationResponseV1::use_case(use_case, &limits).unwrap();
    let round_trip = invocation
        .author_charter_outcome(&limits)
        .unwrap()
        .expect("author-charter outcome");
    assert_eq!(round_trip, result);
}

#[test]
fn approver_invalid_mapping_is_refused_without_a_write_receipt_or_secret() {
    let repository = tempfile::tempdir().unwrap();
    let limits = ContractLimits::packet0();
    let invocation = HandbookSdkV1::open(repository.path())
        .run_approver_administration_use_case(
            ApproverCommandRequest::Bootstrap {
                initial_charter_quorum: vec![ApproverMapping::new("", "owner")],
            },
            &limits,
        )
        .unwrap();

    assert_eq!(invocation.terminal_status(), TerminalStatus::Refused);
    let document = serde_json::to_value(&invocation).unwrap();
    assert!(document["response"]["receipts"]
        .as_array()
        .is_some_and(Vec::is_empty));
    let wire = String::from_utf8(canonical_json_bytes(&invocation, &limits).unwrap()).unwrap();
    assert!(!wire.contains("raw_key"));
    assert!(!wire.contains("private_key"));
    assert!(!wire.contains("authenticator_bytes"));
    let outcome = invocation
        .approver_administration_outcome(&limits)
        .unwrap()
        .expect("approver outcome");
    assert_eq!(outcome.refusal.unwrap().code, "invalid_request");
}

#[test]
fn repository_preflight_failure_closes_through_public_invocation_seams() {
    let repository = tempfile::tempdir().unwrap();
    let limits = ContractLimits::packet0();
    let charter = HandbookSdkV1::open(repository.path())
        .run_author_charter_use_case(
            CharterCommandRequest::Approve {
                candidate_ref: "candidates/candidate_a.json".to_owned(),
                approval_class: "project_owner".to_owned(),
                authority_ref: "project_owner".to_owned(),
                accepted_waiver_refs: Vec::new(),
            },
            &limits,
        )
        .unwrap();
    let approver = HandbookSdkV1::open(repository.path())
        .run_approver_administration_use_case(
            ApproverCommandRequest::Bootstrap {
                initial_charter_quorum: vec![ApproverMapping::new(
                    "charter_approval",
                    "repository_charter_approver",
                )],
            },
            &limits,
        )
        .unwrap();

    for invocation in [charter, approver] {
        assert_eq!(invocation.terminal_status(), TerminalStatus::Blocked);
        assert_eq!(
            serde_json::to_value(&invocation).unwrap()["response_kind"],
            "transport_failure"
        );
        assert_eq!(
            invocation.transport_failure_reason_code(&limits).unwrap(),
            Some("transport.repository_invocation_unavailable".to_owned())
        );
    }
}

#[test]
fn unavailable_native_authenticator_is_a_blocked_prerequisite_without_receipts() {
    let repository = tempfile::tempdir().unwrap();
    let limits = ContractLimits::packet0();
    std::fs::create_dir_all(repository.path().join(".handbook")).unwrap();
    handbook_engine::RepositoryInvocationIdentityServiceV1::new()
        .initialize_for_setup(repository.path())
        .unwrap();
    let result = HandbookSdkV1::open(repository.path()).manage_approvers(
        ApproverCommandRequest::Bootstrap {
            initial_charter_quorum: vec![ApproverMapping::new(
                "charter_approval",
                "repository_charter_approver",
            )],
        },
    );
    assert_eq!(
        result.causal_disposition,
        Some(AuthorCausalDisposition::Prerequisite)
    );
    let use_case = handbook_sdk::contract_membrane::UseCaseResponseV1::approver_administration(
        &result, &limits,
    )
    .unwrap_or_else(|error| panic!("{error:?}: {result:#?}"));

    assert_eq!(use_case.status(), TerminalStatus::Blocked);
    assert!(use_case.receipts().is_empty());
}

#[test]
fn packet_three_use_cases_do_not_inflate_operation_discovery() {
    let inventory = handbook_sdk::contract_membrane::operation_inventory();
    for forbidden in [
        "author-charter",
        "author-project-context",
        "approver-administration",
    ] {
        assert!(inventory
            .iter()
            .all(|row| !row.operation_ref().as_str().contains(forbidden)));
    }
}

#[test]
fn checked_in_schema_admits_packet_three_non_ok_use_cases() {
    let repository = tempfile::tempdir().unwrap();
    let limits = ContractLimits::packet0();
    let invocation = HandbookSdkV1::open(repository.path())
        .run_author_charter_use_case(CharterCommandRequest::Validate, &limits)
        .unwrap();
    let schema_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("schemas/2020-12/handbook.invocation-response/1.0.0.schema.json");
    let schema: serde_json::Value =
        serde_json::from_slice(&std::fs::read(schema_path).unwrap()).unwrap();
    let validator = jsonschema::draft202012::options().build(&schema).unwrap();
    assert!(
        validator.is_valid(&serde_json::to_value(invocation).unwrap()),
        "checked-in invocation schema must admit author-charter blockers"
    );

    let manifest = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("schemas/manifest.json"),
    )
    .unwrap();
    for reference in [
        "handbook.use-case.author-charter-details@1.0.0",
        "handbook.use-case.author-project-context-details@1.0.0",
        "handbook.use-case.approver-administration-details@1.0.0",
    ] {
        assert!(manifest.contains(reference), "missing {reference}");
    }
}

#[test]
fn author_input_transport_presentation_round_trips_inside_the_envelope() {
    let limits = ContractLimits::packet0();
    let error = TransportIoErrorV1::from_io_error(&std::io::Error::from_raw_os_error(2));
    let failure = TransportFailureV1::author_input_error(
        TransportFailureKind::Acquisition,
        "transport.author_project_context_input_unavailable",
        AuthorInputTransportRouteV1::ProjectContext,
        "inputs/context.yaml",
        Some(error.clone()),
        &limits,
    )
    .unwrap();
    let invocation = InvocationResponseV1::transport_failure(failure, &limits).unwrap();
    let bytes = canonical_json_bytes(&invocation, &limits).unwrap();
    let admitted = InvocationResponseV1::parse_and_admit(&bytes, None, &limits).unwrap();

    assert_eq!(
        admitted.transport_presentation(&limits).unwrap(),
        Some(TransportPresentationViewV1::AuthorInput {
            route: AuthorInputTransportRouteV1::ProjectContext,
            input_source: "inputs/context.yaml".to_owned(),
            error: Some(error),
        })
    );
}

#[test]
fn working_directory_transport_presentation_is_closed_and_round_trips() {
    let limits = ContractLimits::packet0();
    let error = TransportIoErrorV1::from_io_error(&std::io::Error::from_raw_os_error(3));
    let failure = TransportFailureV1::working_directory_error(
        TransportFailureKind::Adapter,
        "transport.repository_root_unavailable",
        AuthorWorkingDirectoryCommandV1::ProjectContextValidate,
        error.clone(),
        &limits,
    )
    .unwrap();
    assert_eq!(failure.status(), TerminalStatus::Error);
    let admitted = InvocationResponseV1::transport_failure(failure, &limits).unwrap();
    assert_eq!(
        admitted.transport_presentation(&limits).unwrap(),
        Some(TransportPresentationViewV1::WorkingDirectory {
            command: AuthorWorkingDirectoryCommandV1::ProjectContextValidate,
            error,
        })
    );
}

#[test]
fn true_input_io_is_an_error_and_cannot_be_admitted_as_caller_acquisition() {
    let limits = ContractLimits::packet0();
    let error = TransportIoErrorV1::from_io_error(&std::io::Error::from(
        std::io::ErrorKind::PermissionDenied,
    ));
    assert!(TransportFailureV1::author_input_error(
        TransportFailureKind::Acquisition,
        "transport.author_charter_input_unavailable",
        AuthorInputTransportRouteV1::Charter,
        "inputs/charter.yaml",
        Some(error.clone()),
        &limits,
    )
    .is_err());
    let failure = TransportFailureV1::author_input_error(
        TransportFailureKind::Adapter,
        "transport.author_charter_input_unavailable",
        AuthorInputTransportRouteV1::Charter,
        "inputs/charter.yaml",
        Some(error),
        &limits,
    )
    .unwrap();
    assert_eq!(failure.status(), TerminalStatus::Error);
}

#[test]
fn repository_transport_rejects_command_operation_mismatch() {
    let limits = ContractLimits::packet0();
    let failure = RepositoryInvocationFailureView {
        schema_id: "handbook.repository-invocation-preflight-result".to_owned(),
        schema_version: "1.0".to_owned(),
        operation: "bootstrap".to_owned(),
        stage: Some("repository_identity".to_owned()),
        status: "refused".to_owned(),
        repository_identity_fingerprint: None,
        operation_id: None,
        changed_paths: Vec::new(),
        refusal: AuthorOperationRefusal {
            code: "repository_identity_unavailable".to_owned(),
            message: "repository invocation identity is unavailable".to_owned(),
            retryable: false,
        },
        next_actions: vec![
            "run or repair handbook setup, then retry the complete operation".to_owned(),
        ],
        causal_disposition: AuthorCausalDisposition::Prerequisite,
    };
    assert!(TransportFailureV1::repository_invocation_error(
        "transport.repository_invocation_unavailable",
        handbook_sdk::contract_membrane::RepositoryInvocationCommandV1::CharterApprove,
        &failure,
        &limits,
    )
    .is_err());
}

#[test]
fn arbitrary_packet_three_receipt_partition_is_rejected() {
    let limits = ContractLimits::packet0();
    let result = ApproverOperationResult {
        schema_id: "handbook.approver-admin-adapter-result".to_owned(),
        schema_version: "1.0".to_owned(),
        operation: ApproverOperation::UpdateMapping,
        status: AuthorOperationStatus::Refused,
        changed_paths: Vec::new(),
        refusal: Some(AuthorOperationRefusal {
            code: "transaction_conflict".to_owned(),
            message: "refused".to_owned(),
            retryable: false,
        }),
        next_actions: Vec::new(),
        legacy_projection: None,
        causal_disposition: Some(AuthorCausalDisposition::ImplementationFailure),
        public_effects: vec![AuthorPublicEffect {
            target_ref: format!(
                ".handbook/state/private-records/private-record_{}.json",
                "a".repeat(64)
            ),
            content_fingerprint: Some(format!("sha256:{}", "a".repeat(64))),
            disposition: AuthorPublicEffectDisposition::DurableCreated,
        }],
    };
    assert!(
        handbook_sdk::contract_membrane::UseCaseResponseV1::approver_administration(
            &result, &limits,
        )
        .is_err()
    );
}

#[test]
fn non_ok_partial_approver_installs_emit_one_receipt_per_public_effect_only() {
    let limits = ContractLimits::packet0();
    let result = ApproverOperationResult {
        schema_id: "handbook.approver-admin-adapter-result".to_owned(),
        schema_version: "1.0".to_owned(),
        operation: ApproverOperation::UpdateMapping,
        status: AuthorOperationStatus::Refused,
        changed_paths: Vec::new(),
        refusal: Some(AuthorOperationRefusal {
            code: "transaction_conflict".to_owned(),
            message: "final durability could not be confirmed".to_owned(),
            retryable: false,
        }),
        next_actions: Vec::new(),
        legacy_projection: None,
        causal_disposition: Some(AuthorCausalDisposition::ImplementationFailure),
        public_effects: vec![
            AuthorPublicEffect {
                target_ref: format!(
                    ".handbook/state/registry-states/registry-state_{}.json",
                    "a".repeat(64)
                ),
                content_fingerprint: Some(
                    "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
                        .to_owned(),
                ),
                disposition: AuthorPublicEffectDisposition::InstalledDurabilityUnconfirmed,
            },
            AuthorPublicEffect {
                target_ref: format!(
                    ".handbook/state/registry-transitions/registry-transition_{}.json",
                    "b".repeat(64)
                ),
                content_fingerprint: Some(
                    "sha256:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"
                        .to_owned(),
                ),
                disposition: AuthorPublicEffectDisposition::DurableCreated,
            },
        ],
    };
    let use_case = handbook_sdk::contract_membrane::UseCaseResponseV1::approver_administration(
        &result, &limits,
    )
    .unwrap();

    assert_eq!(use_case.status(), TerminalStatus::Error);
    assert_eq!(use_case.receipts().len(), 2);
    let invocation = InvocationResponseV1::use_case(use_case, &limits).unwrap();
    let wire = String::from_utf8(canonical_json_bytes(&invocation, &limits).unwrap()).unwrap();
    assert!(wire.contains("installed_durability_unconfirmed"));
    assert!(wire.contains("durable_created"));
    for private in [
        "transactions",
        "pending",
        "use-head",
        "raw_key",
        "private_key",
    ] {
        assert!(!wire.contains(private), "private fact leaked: {private}");
    }
}

#[test]
fn project_context_success_receipt_admits_the_authoritative_canonical_target() {
    let limits = ContractLimits::packet0();
    let outcome = ProjectContextCommandOutcome::Author {
        input_mode: "structured_inputs_file".to_owned(),
        input_source: "context.yaml".to_owned(),
        result: Ok(AuthorProjectContextResult {
            canonical_repo_relative_path: ".handbook/project/context.yaml".to_owned(),
            bytes_written: 12,
            source_fingerprint:
                "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_owned(),
            rendered_output_fingerprint:
                "sha256:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb".to_owned(),
            rendered_media_type: "text/markdown",
            public_effects: vec![AuthorPublicEffect {
                target_ref: ".handbook/project/context.yaml".to_owned(),
                content_fingerprint: Some(
                    "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
                        .to_owned(),
                ),
                disposition: AuthorPublicEffectDisposition::DurableCreated,
            }],
        }),
    };

    let use_case = handbook_sdk::contract_membrane::UseCaseResponseV1::author_project_context(
        &outcome, &limits,
    )
    .expect("authoritative Project Context target must be receiptable");
    assert_eq!(use_case.receipts().len(), 1);
}

#[test]
fn promotion_receipts_follow_actual_effects_not_frozen_legacy_changed_paths() {
    let limits = ContractLimits::packet0();
    let result = CharterOperationResult {
        schema_id: "handbook.charter-command-adapter-result".to_owned(),
        schema_version: "1.0".to_owned(),
        operation: CharterOperation::Promote,
        status: AuthorOperationStatus::Succeeded,
        canonical_path: Some(".handbook/project/charter.yaml".to_owned()),
        source_fingerprint: None,
        rendered_output_fingerprint: None,
        intake_ref: None,
        intake_fingerprint: None,
        candidate_ref: None,
        candidate_fingerprint: None,
        approval_ref: None,
        approval_fingerprint: None,
        promotion_ref: Some(format!("promotions/promotion_{}.json", "b".repeat(64))),
        promotion_fingerprint: Some(format!("sha256:{}", "b".repeat(64))),
        changed_paths: vec![
            ".handbook/project/charter.yaml".to_owned(),
            format!(
                ".handbook/state/promotions/promotion_{}.json",
                "b".repeat(64)
            ),
            format!(
                ".handbook/state/lifecycle-transitions/lifecycle-transition_{}.json",
                "c".repeat(64)
            ),
        ],
        refusal: None,
        next_actions: Vec::new(),
        invocation_failure: None,
        causal_disposition: None,
        public_effects: vec![AuthorPublicEffect {
            target_ref: ".handbook/project/charter.yaml".to_owned(),
            content_fingerprint: Some(format!("sha256:{}", "a".repeat(64))),
            disposition: AuthorPublicEffectDisposition::DurableCreated,
        }],
    };

    let use_case =
        handbook_sdk::contract_membrane::UseCaseResponseV1::author_charter(&result, &limits)
            .unwrap();
    assert_eq!(use_case.receipts().len(), 1);
    let document = serde_json::to_value(&use_case).unwrap();
    let receipts = document["receipts"].as_array().unwrap();
    assert_eq!(
        receipts[0]["condition"]["target_ref"],
        ".handbook/project/charter.yaml"
    );
    assert!(!serde_json::to_string(receipts)
        .unwrap()
        .contains("lifecycle-transition_"));
}

#[test]
fn approval_use_transition_is_a_receiptable_public_effect_without_private_use_head_state() {
    let limits = ContractLimits::packet0();
    let result = CharterOperationResult {
        schema_id: "handbook.charter-command-adapter-result".to_owned(),
        schema_version: "1.0".to_owned(),
        operation: CharterOperation::Approve,
        status: AuthorOperationStatus::Succeeded,
        canonical_path: None,
        source_fingerprint: None,
        rendered_output_fingerprint: None,
        intake_ref: None,
        intake_fingerprint: None,
        candidate_ref: None,
        candidate_fingerprint: None,
        approval_ref: Some(format!("approvals/approval_{}.json", "a".repeat(64))),
        approval_fingerprint: Some(format!("sha256:{}", "a".repeat(64))),
        promotion_ref: None,
        promotion_fingerprint: None,
        changed_paths: Vec::new(),
        refusal: None,
        next_actions: Vec::new(),
        invocation_failure: None,
        causal_disposition: None,
        public_effects: vec![AuthorPublicEffect {
            target_ref: format!(
                ".handbook/state/authenticator-use-transitions/authenticator-use-transition_{}.json",
                "d".repeat(64)
            ),
            content_fingerprint: Some(format!("sha256:{}", "d".repeat(64))),
            disposition: AuthorPublicEffectDisposition::DurableCreated,
        }],
    };

    let use_case =
        handbook_sdk::contract_membrane::UseCaseResponseV1::author_charter(&result, &limits)
            .unwrap();
    assert_eq!(use_case.receipts().len(), 1);
    let wire = String::from_utf8(canonical_json_bytes(&use_case, &limits).unwrap()).unwrap();
    assert!(wire.contains("authenticator-use-transitions"));
    assert!(!wire.contains("authenticator-use-heads"));
    assert!(!wire.contains("raw_key"));
    assert!(!wire.contains("private_key"));
}
