use handbook_sdk::contract_membrane::{
    admit_operation_definition, canonical_json_bytes, operation_inventory, parse_canonical_json,
    validate_complete_definition_set, validate_complete_operation_ref_set, AdmissionReason,
    BoundedString, BoundedVec, ContractError, ContractLimits, ContractMember, ContractValue,
    CorrelationRef, DeclaredWrite, DefinitionSetError, Diagnostic, DiscoveryState, ExactBinding,
    ExactRef, IdempotencyState, JsonPointer, NextAction, OperationAdmissionRefusalV1,
    OperationAdmissionResult, OperationAvailability, OperationClass, OperationDefinition,
    OperationIdempotency, OperationOutcomeSchemas, OperationOwner, OperationOwnerBinding,
    OperationRequestV1, OperationResponseV1, OperationSchemaValidator, OperationTransport, Problem,
    ProblemCategory, RecheckCondition, RequestId, RetryDirective, SemVer, TerminalFields,
    TerminalStatus, UtcTimestamp, WriteReceipt,
};
use std::collections::BTreeSet;

#[test]
fn inventory_is_the_exact_contiguous_unique_sixty_two_row_set() {
    let rows = operation_inventory();
    assert_eq!(rows.len(), 62);

    let proof_ordinals = rows
        .iter()
        .map(|row| row.proof_ordinal())
        .collect::<Vec<_>>();
    assert_eq!(proof_ordinals, (1..=62).collect::<Vec<_>>());
    assert_eq!(
        rows.iter()
            .map(|row| row.operation_id())
            .collect::<BTreeSet<_>>()
            .len(),
        62
    );
    assert!(rows
        .iter()
        .all(|row| row.operation_ref().version().as_str() == "1.0.0"));
    assert!(rows.iter().all(|row| row.current_transports().is_empty()));
    assert!(rows
        .iter()
        .all(|row| row.discovery() == DiscoveryState::Omit));

    assert_eq!(
        rows.iter()
            .filter(|row| row.availability() == OperationAvailability::LivePrecursor)
            .count(),
        26
    );
    assert_eq!(
        rows.iter()
            .filter(|row| row.availability() == OperationAvailability::Phase5Deferred)
            .count(),
        12
    );
    assert_eq!(
        rows.iter()
            .filter(|row| row.owner() == OperationOwner::SdkComposition)
            .count(),
        7
    );
    assert_eq!(
        rows.iter()
            .filter(|row| row.class() == OperationClass::AppendOnly)
            .count(),
        13
    );
    assert_eq!(
        rows.iter()
            .filter(|row| row.class() == OperationClass::CompareAndWrite)
            .count(),
        7
    );

    let frozen_tuples = rows
        .iter()
        .map(|row| {
            serde_json::json!({
                "admission": row.admission_reason(),
                "availability": row.availability(),
                "class": row.class(),
                "operation": row.operation_ref(),
                "owner": row.owner(),
                "proof": row.proof_ordinal(),
            })
        })
        .collect::<Vec<_>>();
    let frozen_bytes = canonical_json_bytes(&frozen_tuples, &ContractLimits::packet0()).unwrap();
    assert_eq!(
        handbook_sdk::contract_membrane::sha256_fingerprint(&frozen_bytes).as_str(),
        "sha256:ec8eaa6447efd2473340d1de7225acaa8e81e04d6ce9184258b51fdd6c4380a5"
    );
}

#[test]
fn current_admission_reasons_preserve_owner_and_semantic_precedence() {
    let rows = operation_inventory();
    let row = |operation: &str| {
        rows.iter()
            .find(|row| row.operation_id() == operation)
            .unwrap()
    };

    assert_eq!(
        row("artifact.validate").admission_reason(),
        AdmissionReason::IncompleteClosure
    );
    assert_eq!(
        row("pipeline.route.resolve").admission_reason(),
        AdmissionReason::SemanticMismatch
    );
    assert_eq!(
        row("repository.setup.apply").admission_reason(),
        AdmissionReason::IndependentOwnerAbsent
    );
    assert_eq!(
        row("dock.run").admission_reason(),
        AdmissionReason::OwnerDeferred
    );
}

#[test]
fn complete_definition_set_rejects_missing_duplicate_and_unknown_identities() {
    let expected_refs = operation_inventory()
        .iter()
        .map(|row| row.operation_ref().clone())
        .collect::<Vec<_>>();
    validate_complete_operation_ref_set(&expected_refs).unwrap();

    let expected = complete_definition_set();
    validate_complete_definition_set(&expected).unwrap();

    assert_eq!(
        validate_complete_definition_set(&expected[..61]),
        Err(DefinitionSetError::MissingDefinition)
    );

    let mut duplicate = expected.clone();
    duplicate[61] = duplicate[60].clone();
    assert_eq!(
        validate_complete_definition_set(&duplicate),
        Err(DefinitionSetError::DuplicateDefinition)
    );

    let mut unknown = expected;
    unknown[61] = definition_with(
        "unknown.operation",
        OperationClass::ReadOnly,
        "handbook-engine",
        "0.2.0",
    );
    assert_eq!(
        validate_complete_definition_set(&unknown),
        Err(DefinitionSetError::UnknownDefinition)
    );

    let mut mismatched = complete_definition_set();
    mismatched[11] = definition_with(
        "artifact.validate",
        OperationClass::ReadOnly,
        "handbook-flow",
        "0.1.1",
    );
    assert_eq!(
        validate_complete_definition_set(&mismatched),
        Err(DefinitionSetError::DefinitionMismatch)
    );
}

#[test]
fn definition_binds_exact_identity_schemas_owner_class_and_fingerprint() {
    let definition = read_definition("artifact.validate");
    assert_eq!(definition.operation().as_str(), "artifact.validate@1.0.0");
    assert_eq!(
        definition.request_schema().reference().as_str(),
        "handbook.operation.artifact-validate-request@1.0.0"
    );
    assert_eq!(
        definition.result_schema().reference().as_str(),
        "handbook.operation.artifact-validate-result@1.0.0"
    );
    assert_eq!(definition.binding().reference(), definition.operation());
    assert_eq!(
        definition.binding().fingerprint(),
        definition.definition_fingerprint()
    );

    let limits = ContractLimits::packet0();
    let bytes = canonical_json_bytes(&definition, &limits).unwrap();
    assert_eq!(
        parse_canonical_json::<OperationDefinition>(&bytes, &limits).unwrap(),
        definition
    );

    let mut stale: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    stale["definition_fingerprint"] = serde_json::Value::String(
        "sha256:0000000000000000000000000000000000000000000000000000000000000000".to_owned(),
    );
    let stale = canonical_json_bytes(&stale, &limits).unwrap();
    assert_eq!(
        parse_canonical_json::<OperationDefinition>(&stale, &limits),
        Err(ContractError::FingerprintMismatch)
    );
}

#[test]
fn mutation_definitions_require_exact_key_basis_and_write_set_shapes() {
    let limits = ContractLimits::packet0();
    let row = operation_inventory()
        .iter()
        .find(|row| row.operation_id() == "posture.transition.apply")
        .unwrap();
    let write = DeclaredWrite::new(
        bounded("posture.transition"),
        bounded("semantic.record"),
        bounded("posture.atomic"),
        binding("condition.posture"),
    )
    .unwrap();
    let definition = OperationDefinition::new(
        binding("handbook.operation-definition"),
        row.operation_ref().clone(),
        owner_binding("handbook-engine", "0.2.0"),
        binding("handbook.operation.posture-transition-apply-request"),
        binding("handbook.operation.posture-transition-apply-result"),
        outcome_schemas(),
        BoundedVec::new(Vec::new()).unwrap(),
        OperationClass::CompareAndWrite,
        OperationIdempotency::compare_and_write(
            JsonPointer::parse("/body/idempotency_key").unwrap(),
            BoundedVec::new(vec![JsonPointer::parse("/body/expected_canonical").unwrap()]).unwrap(),
        )
        .unwrap(),
        BoundedVec::new(vec![write]).unwrap(),
        declared_transports(),
        None,
        &limits,
    )
    .unwrap();
    assert_eq!(definition.class(), OperationClass::CompareAndWrite);

    assert_eq!(
        OperationIdempotency::compare_and_write(
            JsonPointer::parse("/body/idempotency_key").unwrap(),
            BoundedVec::new(Vec::new()).unwrap(),
        ),
        Err(ContractError::InvalidInvariant)
    );
}

#[test]
fn request_wire_extracts_and_redacts_raw_key_with_stable_scoped_identity() {
    let definition = mutation_definition();
    let limits = ContractLimits::packet0();
    let raw = "raw-key-sentinel-do-not-echo";
    let body = handbook_sdk::contract_membrane::ContractValue::object(vec![
        ContractMember::new(
            bounded("expected_canonical"),
            handbook_sdk::contract_membrane::ContractValue::null(),
        ),
        ContractMember::new(
            bounded("idempotency_key"),
            handbook_sdk::contract_membrane::ContractValue::string(bounded(raw)),
        ),
    ])
    .unwrap();
    let first = OperationRequestV1::new(
        binding("handbook.operation-request"),
        Some(RequestId::parse("request-a").unwrap()),
        &definition,
        body,
        &FixtureSchemaValidator,
        &limits,
    )
    .unwrap();
    let first_bytes = canonical_json_bytes(&first, &limits).unwrap();
    assert!(String::from_utf8(first_bytes.clone())
        .unwrap()
        .contains(raw));
    assert!(!format!("{first:?}").contains(raw));
    assert!(
        !String::from_utf8(canonical_json_bytes(first.body(), &limits).unwrap())
            .unwrap()
            .contains(raw)
    );
    assert!(first.idempotency_key_fingerprint().is_some());

    let parsed = OperationRequestV1::parse_and_admit(
        &first_bytes,
        &definition,
        &FixtureSchemaValidator,
        &limits,
    )
    .unwrap();
    assert_eq!(parsed.request_fingerprint(), first.request_fingerprint());
    assert_eq!(
        parsed.idempotency_key_fingerprint(),
        first.idempotency_key_fingerprint()
    );

    let value: serde_json::Value = serde_json::from_slice(&first_bytes).unwrap();
    let body_bytes = canonical_json_bytes(&value["body"], &limits).unwrap();
    let second = OperationRequestV1::new(
        binding("handbook.operation-request"),
        Some(RequestId::parse("request-b").unwrap()),
        &definition,
        parse_canonical_json(&body_bytes, &limits).unwrap(),
        &FixtureSchemaValidator,
        &limits,
    )
    .unwrap();
    assert_eq!(first.request_fingerprint(), second.request_fingerprint());
    assert_ne!(first.request_id(), second.request_id());
}

#[test]
fn request_admission_rejects_definition_schema_key_and_validation_bypasses() {
    let limits = ContractLimits::packet0();
    let mutation = mutation_definition();
    let body = keyed_body("raw-key-for-adversarial-proof");
    let admitted = OperationRequestV1::new(
        binding("handbook.operation-request"),
        None,
        &mutation,
        body,
        &FixtureSchemaValidator,
        &limits,
    )
    .unwrap();
    let mut missing_key = to_json_value(&admitted);
    missing_key["body"]
        .as_object_mut()
        .unwrap()
        .remove("idempotency_key");
    rewrite_request_fingerprint(&mut missing_key, None);
    let missing_key = canonical_json_bytes(&missing_key, &limits).unwrap();
    assert!(matches!(
        OperationRequestV1::parse_and_admit(
            &missing_key,
            &mutation,
            &FixtureSchemaValidator,
            &limits,
        ),
        Err(ContractError::InvalidInvariant)
    ));

    let read = read_definition("artifact.validate");
    let read_request = OperationRequestV1::new(
        binding("handbook.operation-request"),
        None,
        &read,
        ContractValue::object(Vec::new()).unwrap(),
        &FixtureSchemaValidator,
        &limits,
    )
    .unwrap();
    let mut extra_key = to_json_value(&read_request);
    extra_key["body"]["idempotency_key"] = serde_json::Value::String("extra-key".to_owned());
    let scoped = scoped_key_fingerprint(read.operation(), "extra-key");
    rewrite_request_fingerprint(&mut extra_key, Some(&scoped));
    let extra_key = canonical_json_bytes(&extra_key, &limits).unwrap();
    assert!(matches!(
        OperationRequestV1::parse_and_admit(&extra_key, &read, &FixtureSchemaValidator, &limits,),
        Err(ContractError::InvalidInvariant)
    ));

    let mut substituted_schema = to_json_value(&read_request);
    substituted_schema["body_schema"] = to_json_value(&binding("substituted.request-schema"));
    rewrite_request_fingerprint(&mut substituted_schema, None);
    let substituted_schema = canonical_json_bytes(&substituted_schema, &limits).unwrap();
    assert!(matches!(
        OperationRequestV1::parse_and_admit(
            &substituted_schema,
            &read,
            &FixtureSchemaValidator,
            &limits,
        ),
        Err(ContractError::InvalidInvariant)
    ));

    let valid_bytes = canonical_json_bytes(&read_request, &limits).unwrap();
    assert!(matches!(
        OperationRequestV1::parse_and_admit(
            &valid_bytes,
            &read,
            &RejectAllSchemaValidator,
            &limits,
        ),
        Err(ContractError::InvalidInvariant)
    ));
}

#[test]
fn ordinary_response_preserves_four_terminals_and_receipt_rules() {
    let limits = ContractLimits::packet0();
    let read = read_definition("artifact.validate");
    let terminals = vec![
        TerminalFields::new(
            TerminalStatus::Ok,
            Some(handbook_sdk::contract_membrane::ContractValue::null()),
            BoundedVec::new(Vec::new()).unwrap(),
            BoundedVec::new(Vec::new()).unwrap(),
            BoundedVec::new(Vec::new()).unwrap(),
        )
        .unwrap(),
        TerminalFields::new(
            TerminalStatus::Blocked,
            None,
            BoundedVec::new(vec![problem(ProblemCategory::Prerequisite)]).unwrap(),
            BoundedVec::new(Vec::new()).unwrap(),
            BoundedVec::new(Vec::new()).unwrap(),
        )
        .unwrap(),
        TerminalFields::new(
            TerminalStatus::Refused,
            None,
            BoundedVec::new(Vec::new()).unwrap(),
            BoundedVec::new(vec![problem(ProblemCategory::Schema)]).unwrap(),
            BoundedVec::new(Vec::new()).unwrap(),
        )
        .unwrap(),
        TerminalFields::new(
            TerminalStatus::Error,
            None,
            BoundedVec::new(Vec::new()).unwrap(),
            BoundedVec::new(Vec::new()).unwrap(),
            BoundedVec::new(vec![problem(ProblemCategory::Implementation)]).unwrap(),
        )
        .unwrap(),
    ];
    for terminal in terminals {
        let response = OperationResponseV1::new(
            binding("handbook.operation-response"),
            None,
            handbook_sdk::contract_membrane::sha256_fingerprint(b"request"),
            &read,
            terminal,
            IdempotencyState::not_applicable().unwrap(),
            BoundedVec::new(Vec::new()).unwrap(),
            BoundedVec::<Diagnostic, 128>::new(Vec::new()).unwrap(),
            BoundedVec::<NextAction, 128>::new(Vec::new()).unwrap(),
            &FixtureSchemaValidator,
            &limits,
        )
        .unwrap();
        let bytes = canonical_json_bytes(&response, &limits).unwrap();
        let value: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        for field in ["status", "data", "blockers", "refusals", "errors"] {
            assert!(value.get(field).is_some(), "missing {field}");
        }
        assert_eq!(
            OperationResponseV1::parse_and_admit(&bytes, &read, &FixtureSchemaValidator, &limits,)
                .unwrap(),
            response
        );
    }

    let mutation = mutation_definition();
    let receipt = matching_receipt();
    let terminal = TerminalFields::new(
        TerminalStatus::Ok,
        Some(handbook_sdk::contract_membrane::ContractValue::null()),
        BoundedVec::new(Vec::new()).unwrap(),
        BoundedVec::new(Vec::new()).unwrap(),
        BoundedVec::new(Vec::new()).unwrap(),
    )
    .unwrap();
    let original = OperationResponseV1::new(
        binding("handbook.operation-response"),
        Some(RequestId::parse("request-original").unwrap()),
        handbook_sdk::contract_membrane::sha256_fingerprint(b"request"),
        &mutation,
        terminal.clone(),
        IdempotencyState::established(
            handbook_sdk::contract_membrane::sha256_fingerprint(b"key"),
            handbook_sdk::contract_membrane::sha256_fingerprint(b"request"),
            false,
            None,
            UtcTimestamp::parse("2030-01-01T00:00:00Z").unwrap(),
        )
        .unwrap(),
        BoundedVec::new(vec![receipt.clone()]).unwrap(),
        BoundedVec::new(Vec::new()).unwrap(),
        BoundedVec::new(Vec::new()).unwrap(),
        &FixtureSchemaValidator,
        &limits,
    )
    .unwrap();
    let replay = OperationResponseV1::new(
        binding("handbook.operation-response"),
        Some(RequestId::parse("request-replay").unwrap()),
        handbook_sdk::contract_membrane::sha256_fingerprint(b"request"),
        &mutation,
        terminal,
        IdempotencyState::established(
            handbook_sdk::contract_membrane::sha256_fingerprint(b"key"),
            handbook_sdk::contract_membrane::sha256_fingerprint(b"request"),
            true,
            Some(original.result_fingerprint().clone()),
            UtcTimestamp::parse("2030-01-01T00:00:00Z").unwrap(),
        )
        .unwrap(),
        BoundedVec::new(vec![receipt]).unwrap(),
        BoundedVec::new(Vec::new()).unwrap(),
        BoundedVec::new(Vec::new()).unwrap(),
        &FixtureSchemaValidator,
        &limits,
    )
    .unwrap();
    assert_eq!(original.result_fingerprint(), replay.result_fingerprint());
    assert_ne!(
        original.response_fingerprint(),
        replay.response_fingerprint()
    );
    replay.verify_replay_against(&original).unwrap();

    assert_eq!(
        OperationResponseV1::new(
            binding("handbook.operation-response"),
            None,
            handbook_sdk::contract_membrane::sha256_fingerprint(b"request"),
            &read,
            TerminalFields::new(
                TerminalStatus::Ok,
                Some(handbook_sdk::contract_membrane::ContractValue::null()),
                BoundedVec::new(Vec::new()).unwrap(),
                BoundedVec::new(Vec::new()).unwrap(),
                BoundedVec::new(Vec::new()).unwrap(),
            )
            .unwrap(),
            IdempotencyState::not_applicable().unwrap(),
            BoundedVec::new(vec![matching_receipt()]).unwrap(),
            BoundedVec::new(Vec::new()).unwrap(),
            BoundedVec::new(Vec::new()).unwrap(),
            &FixtureSchemaValidator,
            &limits,
        ),
        Err(ContractError::InvalidInvariant)
    );
}

#[test]
fn response_admission_rejects_receipt_idempotency_condition_and_replay_bypasses() {
    let limits = ContractLimits::packet0();
    let request_fingerprint = handbook_sdk::contract_membrane::sha256_fingerprint(b"request");
    let read = read_definition("artifact.validate");
    let read_response = OperationResponseV1::new(
        binding("handbook.operation-response"),
        None,
        request_fingerprint.clone(),
        &read,
        ok_terminal(),
        IdempotencyState::not_applicable().unwrap(),
        BoundedVec::new(Vec::new()).unwrap(),
        BoundedVec::new(Vec::new()).unwrap(),
        BoundedVec::new(Vec::new()).unwrap(),
        &FixtureSchemaValidator,
        &limits,
    )
    .unwrap();

    let mut forged_idempotency = to_json_value(&read_response);
    forged_idempotency["idempotency"] = serde_json::json!({
        "idempotency_key_fingerprint": handbook_sdk::contract_membrane::sha256_fingerprint(b"key"),
        "original_result_fingerprint": null,
        "replayed": false,
        "request_fingerprint": request_fingerprint,
        "result_retention_until_utc": "2030-01-01T00:00:00Z",
        "state": "established"
    });
    rewrite_response_fingerprints(&mut forged_idempotency);
    let forged_idempotency = canonical_json_bytes(&forged_idempotency, &limits).unwrap();
    assert_eq!(
        OperationResponseV1::parse_and_admit(
            &forged_idempotency,
            &read,
            &FixtureSchemaValidator,
            &limits,
        ),
        Err(ContractError::InvalidInvariant)
    );

    let mut forged_receipt = to_json_value(&read_response);
    forged_receipt["receipts"] = serde_json::json!([matching_receipt()]);
    rewrite_response_fingerprints(&mut forged_receipt);
    let forged_receipt = canonical_json_bytes(&forged_receipt, &limits).unwrap();
    assert_eq!(
        OperationResponseV1::parse_and_admit(
            &forged_receipt,
            &read,
            &FixtureSchemaValidator,
            &limits,
        ),
        Err(ContractError::InvalidInvariant)
    );

    let mutation = mutation_definition();
    assert_eq!(
        OperationResponseV1::new(
            binding("handbook.operation-response"),
            None,
            request_fingerprint.clone(),
            &mutation,
            ok_terminal(),
            IdempotencyState::established(
                handbook_sdk::contract_membrane::sha256_fingerprint(b"key"),
                handbook_sdk::contract_membrane::sha256_fingerprint(b"different-request"),
                false,
                None,
                UtcTimestamp::parse("2030-01-01T00:00:00Z").unwrap(),
            )
            .unwrap(),
            BoundedVec::new(vec![matching_receipt()]).unwrap(),
            BoundedVec::new(Vec::new()).unwrap(),
            BoundedVec::new(Vec::new()).unwrap(),
            &FixtureSchemaValidator,
            &limits,
        ),
        Err(ContractError::InvalidInvariant)
    );

    assert_eq!(
        OperationResponseV1::new(
            binding("handbook.operation-response"),
            None,
            request_fingerprint.clone(),
            &mutation,
            ok_terminal(),
            IdempotencyState::established(
                handbook_sdk::contract_membrane::sha256_fingerprint(b"key"),
                request_fingerprint,
                true,
                Some(handbook_sdk::contract_membrane::sha256_fingerprint(
                    b"not-the-result",
                )),
                UtcTimestamp::parse("2030-01-01T00:00:00Z").unwrap(),
            )
            .unwrap(),
            BoundedVec::new(vec![matching_receipt()]).unwrap(),
            BoundedVec::new(Vec::new()).unwrap(),
            BoundedVec::new(Vec::new()).unwrap(),
            &FixtureSchemaValidator,
            &limits,
        ),
        Err(ContractError::InvalidInvariant)
    );

    assert_eq!(
        OperationResponseV1::new(
            binding("handbook.operation-response"),
            None,
            handbook_sdk::contract_membrane::sha256_fingerprint(b"request"),
            &mutation_definition_with_condition("condition.reject"),
            ok_terminal(),
            IdempotencyState::established(
                handbook_sdk::contract_membrane::sha256_fingerprint(b"key"),
                handbook_sdk::contract_membrane::sha256_fingerprint(b"request"),
                false,
                None,
                UtcTimestamp::parse("2030-01-01T00:00:00Z").unwrap(),
            )
            .unwrap(),
            BoundedVec::new(vec![matching_receipt()]).unwrap(),
            BoundedVec::new(Vec::new()).unwrap(),
            BoundedVec::new(Vec::new()).unwrap(),
            &FixtureSchemaValidator,
            &limits,
        ),
        Err(ContractError::InvalidInvariant)
    );
}

#[test]
fn pre_owner_refusal_is_closed_and_contains_no_owner_result_or_receipt() {
    let limits = ContractLimits::packet0();
    let refusal = OperationAdmissionRefusalV1::new(
        binding("handbook.operation-admission-refusal"),
        None,
        Some(ExactRef::parse("unknown.operation@1.0.0").unwrap()),
        None,
        None,
        problem(ProblemCategory::Capability),
        &limits,
    )
    .unwrap();
    let bytes = canonical_json_bytes(&refusal, &limits).unwrap();
    let value: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(value["status"], "refused");
    assert!(value["data"].is_null());
    assert_eq!(value["blockers"], serde_json::json!([]));
    assert_eq!(value["refusals"].as_array().unwrap().len(), 1);
    assert_eq!(value["errors"], serde_json::json!([]));
    for forbidden in ["result_schema", "idempotency", "receipts", "owner_result"] {
        assert!(value.get(forbidden).is_none());
    }
    assert_eq!(
        parse_canonical_json::<OperationAdmissionRefusalV1>(&bytes, &limits).unwrap(),
        refusal
    );
}

#[test]
fn public_typed_surface_exposes_every_non_secret_binding_and_sealed_key_bridge() {
    let limits = ContractLimits::packet0();
    let definition = read_definition("artifact.validate");
    assert_eq!(
        definition.declared_transports(),
        declared_transports().as_slice()
    );
    assert!(definition.deprecation().is_none());

    let request = OperationRequestV1::new(
        binding("handbook.operation-request"),
        Some(RequestId::parse("typed-surface").unwrap()),
        &definition,
        ContractValue::null(),
        &FixtureSchemaValidator,
        &limits,
    )
    .unwrap();
    assert_eq!(
        request.schema().reference().as_str(),
        "handbook.operation-request@1.0.0"
    );
    assert_eq!(request.operation(), &definition.binding());
    assert_eq!(request.body_schema(), definition.request_schema());
    let _: fn(
        &OperationRequestV1,
        &mut dyn handbook_sdk::contract_membrane::IdempotencyKeyConsumer,
    ) -> Option<handbook_sdk::contract_membrane::Fingerprint> =
        OperationRequestV1::with_owner_idempotency_key;

    let response = OperationResponseV1::new(
        binding("handbook.operation-response"),
        request.request_id().cloned(),
        request.request_fingerprint().clone(),
        &definition,
        ok_terminal(),
        IdempotencyState::not_applicable().unwrap(),
        BoundedVec::new(Vec::new()).unwrap(),
        BoundedVec::new(Vec::new()).unwrap(),
        BoundedVec::new(Vec::new()).unwrap(),
        &FixtureSchemaValidator,
        &limits,
    )
    .unwrap();
    assert_eq!(response.request_id(), request.request_id());
    assert_eq!(response.operation(), &definition.binding());
    assert_eq!(response.result_schema(), definition.result_schema());
    assert_eq!(response.terminal().status(), TerminalStatus::Ok);
    assert!(response.receipts().is_empty());
    assert!(response.diagnostics().is_empty());
    assert!(response.next_actions().is_empty());

    let refusal = OperationAdmissionRefusalV1::new(
        binding("handbook.operation-admission-refusal"),
        request.request_id().cloned(),
        Some(definition.operation().clone()),
        Some(definition.binding()),
        Some(request.request_fingerprint().clone()),
        problem(ProblemCategory::Capability),
        &limits,
    )
    .unwrap();
    assert_eq!(refusal.request_id(), request.request_id());
    assert_eq!(refusal.attempted_operation(), Some(definition.operation()));
    assert_eq!(refusal.attempted_definition(), Some(&definition.binding()));
    assert_eq!(
        refusal.request_fingerprint(),
        Some(request.request_fingerprint())
    );
    assert_eq!(refusal.terminal().status(), TerminalStatus::Refused);
    assert_eq!(
        refusal.schema().reference().as_str(),
        "handbook.operation-admission-refusal@1.0.0"
    );
    assert_eq!(refusal.response_fingerprint().as_str().len(), 71);
}

#[test]
fn semantic_owners_independently_refuse_current_incomplete_or_mismatched_claims() {
    assert_eq!(
        admit_operation_definition(&read_definition("artifact.validate")),
        Err(ContractError::InvalidInvariant)
    );
    assert_eq!(
        admit_operation_definition(&definition_for("flow.resolve", "handbook-flow", "0.1.1",)),
        Err(ContractError::InvalidInvariant)
    );
    assert_eq!(
        admit_operation_definition(&definition_for(
            "pipeline.catalog.list",
            "handbook-pipeline",
            "0.1.2",
        )),
        Err(ContractError::InvalidInvariant)
    );
    assert_eq!(
        admit_operation_definition(&definition_for(
            "pipeline.route.resolve",
            "handbook-pipeline",
            "0.1.2",
        ))
        .unwrap(),
        OperationAdmissionResult::Omitted(AdmissionReason::SemanticMismatch)
    );
    assert_eq!(
        admit_operation_definition(&definition_for(
            "repository.doctor",
            "handbook-sdk",
            "0.1.0",
        ))
        .unwrap(),
        OperationAdmissionResult::Omitted(AdmissionReason::IndependentOwnerAbsent)
    );
    assert_eq!(
        admit_operation_definition(&definition_for(
            "contract.definition.list",
            "future-handbook-contracts",
            "0.0.0",
        ))
        .unwrap(),
        OperationAdmissionResult::Omitted(AdmissionReason::OwnerDeferred)
    );
}

#[test]
fn all_live_symbol_claims_and_known_source_inconsistencies_resolve() {
    let artifact = include_str!("../src/artifact.rs");
    for symbol in [
        "pub fn list_kinds",
        "pub fn list_instances",
        "pub fn read(",
        "pub fn validate(",
        "pub fn intake_definition",
        "pub fn evaluate_intake",
        "pub fn intake_append",
        "pub fn candidate_validate",
        "pub fn candidate_append",
        "pub fn promote(",
    ] {
        assert!(
            artifact.contains(symbol),
            "missing artifact symbol {symbol}"
        );
    }
    let sdk = include_str!("../src/lib.rs");
    for symbol in [
        "pub fn apply_posture_transition",
        "pub fn run_setup",
        "pub fn doctor",
        "pub fn generate_packet",
    ] {
        assert!(sdk.contains(symbol), "missing SDK symbol {symbol}");
    }
    let context = include_str!("../../engine/src/context_resolution.rs");
    for symbol in [
        "pub fn admit_escalation_request",
        "pub fn admit_escalation_disposition",
        "pub fn admit_promotion_request",
        "pub fn admit_promotion_disposition",
    ] {
        assert!(context.contains(symbol), "missing engine symbol {symbol}");
    }
    let posture = include_str!("../../engine/src/posture_transition_facade.rs");
    assert!(posture.contains("pub fn apply_posture_transition"));
    let flow = include_str!("../../flow/src/resolver.rs");
    assert!(flow.contains("pub fn resolve("));
    let pipeline = include_str!("../src/pipeline_route_api.rs");
    for symbol in [
        "pub fn list_pipeline_catalog",
        "pub fn show_pipeline",
        "pub fn resolve_pipeline",
        "pub fn compile_pipeline",
        "pub fn preview_pipeline_capture",
        "pub fn apply_pipeline_capture",
        "pub fn emit_pipeline_handoff",
        "pub fn update_pipeline_state",
    ] {
        assert!(
            pipeline.contains(symbol),
            "missing pipeline symbol {symbol}"
        );
    }
    assert!(pipeline.contains("persist_route_basis("));
    let setup = include_str!("../src/setup.rs");
    assert!(setup.contains("#[cfg(test)]\npub(crate) fn plan_setup("));
}

struct FixtureSchemaValidator;

impl OperationSchemaValidator for FixtureSchemaValidator {
    fn validate_instance(
        &self,
        schema: &ExactBinding,
        _instance: &ContractValue,
    ) -> Result<(), ContractError> {
        if schema.reference().as_str().contains("reject") {
            Err(ContractError::InvalidInvariant)
        } else {
            Ok(())
        }
    }
}

struct RejectAllSchemaValidator;

impl OperationSchemaValidator for RejectAllSchemaValidator {
    fn validate_instance(
        &self,
        _schema: &ExactBinding,
        _instance: &ContractValue,
    ) -> Result<(), ContractError> {
        Err(ContractError::InvalidInvariant)
    }
}

fn complete_definition_set() -> Vec<OperationDefinition> {
    operation_inventory()
        .iter()
        .map(|row| {
            let (owner, version) = match row.owner() {
                OperationOwner::Engine => ("handbook-engine", "0.2.0"),
                OperationOwner::Flow => ("handbook-flow", "0.1.1"),
                OperationOwner::Pipeline => ("handbook-pipeline", "0.1.2"),
                OperationOwner::SdkComposition => ("handbook-sdk", "0.1.0"),
                OperationOwner::Phase5 => ("future-handbook-contracts", "0.0.0"),
            };
            definition_with(row.operation_id(), row.class(), owner, version)
        })
        .collect()
}

fn read_definition(operation_id: &str) -> OperationDefinition {
    let row = operation_inventory()
        .iter()
        .find(|row| row.operation_id() == operation_id)
        .unwrap();
    let hyphenated = operation_id.replace('.', "-");
    OperationDefinition::new(
        binding("handbook.operation-definition"),
        row.operation_ref().clone(),
        owner_binding("handbook-engine", "0.2.0"),
        binding(&format!("handbook.operation.{hyphenated}-request")),
        binding(&format!("handbook.operation.{hyphenated}-result")),
        outcome_schemas(),
        BoundedVec::new(Vec::new()).unwrap(),
        OperationClass::ReadOnly,
        OperationIdempotency::safe(),
        BoundedVec::new(Vec::new()).unwrap(),
        declared_transports(),
        None,
        &ContractLimits::packet0(),
    )
    .unwrap()
}

fn definition_for(operation_id: &str, owner: &str, version: &str) -> OperationDefinition {
    let row = operation_inventory()
        .iter()
        .find(|row| row.operation_id() == operation_id)
        .unwrap();
    definition_with(operation_id, row.class(), owner, version)
}

fn definition_with(
    operation_id: &str,
    class: OperationClass,
    owner: &str,
    version: &str,
) -> OperationDefinition {
    let hyphenated = operation_id.replace('.', "-");
    OperationDefinition::new(
        binding("handbook.operation-definition"),
        ExactRef::parse(&format!("{operation_id}@1.0.0")).unwrap(),
        owner_binding(owner, version),
        binding(&format!("handbook.operation.{hyphenated}-request")),
        binding(&format!("handbook.operation.{hyphenated}-result")),
        outcome_schemas(),
        BoundedVec::new(Vec::new()).unwrap(),
        class,
        match class {
            OperationClass::ReadOnly => OperationIdempotency::safe(),
            OperationClass::AppendOnly => OperationIdempotency::key_required(
                JsonPointer::parse("/body/idempotency_key").unwrap(),
            )
            .unwrap(),
            OperationClass::CompareAndWrite => OperationIdempotency::compare_and_write(
                JsonPointer::parse("/body/idempotency_key").unwrap(),
                BoundedVec::new(vec![JsonPointer::parse("/body/expected").unwrap()]).unwrap(),
            )
            .unwrap(),
        },
        if class == OperationClass::ReadOnly {
            BoundedVec::new(Vec::new()).unwrap()
        } else {
            BoundedVec::new(vec![DeclaredWrite::new(
                bounded("operation.record"),
                bounded("semantic.record"),
                bounded("operation.atomic"),
                binding("condition.operation"),
            )
            .unwrap()])
            .unwrap()
        },
        declared_transports(),
        None,
        &ContractLimits::packet0(),
    )
    .unwrap()
}

fn mutation_definition() -> OperationDefinition {
    mutation_definition_with_condition("condition.posture")
}

fn mutation_definition_with_condition(condition_schema: &str) -> OperationDefinition {
    let row = operation_inventory()
        .iter()
        .find(|row| row.operation_id() == "posture.transition.apply")
        .unwrap();
    OperationDefinition::new(
        binding("handbook.operation-definition"),
        row.operation_ref().clone(),
        owner_binding("handbook-engine", "0.2.0"),
        binding("handbook.operation.posture-transition-apply-request"),
        binding("handbook.operation.posture-transition-apply-result"),
        outcome_schemas(),
        BoundedVec::new(Vec::new()).unwrap(),
        OperationClass::CompareAndWrite,
        OperationIdempotency::compare_and_write(
            JsonPointer::parse("/body/idempotency_key").unwrap(),
            BoundedVec::new(vec![JsonPointer::parse("/body/expected_canonical").unwrap()]).unwrap(),
        )
        .unwrap(),
        BoundedVec::new(vec![DeclaredWrite::new(
            bounded("posture.transition"),
            bounded("semantic.record"),
            bounded("posture.atomic"),
            binding(condition_schema),
        )
        .unwrap()])
        .unwrap(),
        declared_transports(),
        None,
        &ContractLimits::packet0(),
    )
    .unwrap()
}

fn keyed_body(raw_key: &str) -> ContractValue {
    ContractValue::object(vec![
        ContractMember::new(bounded("expected_canonical"), ContractValue::null()),
        ContractMember::new(
            bounded("idempotency_key"),
            ContractValue::string(bounded(raw_key)),
        ),
    ])
    .unwrap()
}

fn ok_terminal() -> TerminalFields<ContractValue> {
    TerminalFields::new(
        TerminalStatus::Ok,
        Some(ContractValue::null()),
        BoundedVec::new(Vec::new()).unwrap(),
        BoundedVec::new(Vec::new()).unwrap(),
        BoundedVec::new(Vec::new()).unwrap(),
    )
    .unwrap()
}

fn to_json_value(value: &impl serde::Serialize) -> serde_json::Value {
    serde_json::from_slice(&canonical_json_bytes(value, &ContractLimits::packet0()).unwrap())
        .unwrap()
}

fn scoped_key_fingerprint(
    operation: &ExactRef,
    raw_key: &str,
) -> handbook_sdk::contract_membrane::Fingerprint {
    let mut preimage = Vec::new();
    preimage.extend_from_slice(b"handbook.operation-idempotency-key-v1\0");
    preimage.extend_from_slice(operation.as_str().as_bytes());
    preimage.extend_from_slice(b"\0/body/idempotency_key\0");
    preimage.extend_from_slice(raw_key.as_bytes());
    handbook_sdk::contract_membrane::sha256_fingerprint(&preimage)
}

fn rewrite_request_fingerprint(
    request: &mut serde_json::Value,
    idempotency_key_fingerprint: Option<&handbook_sdk::contract_membrane::Fingerprint>,
) {
    let mut sanitized_body = request["body"].clone();
    if let Some(body) = sanitized_body.as_object_mut() {
        body.remove("idempotency_key");
    }
    let preimage = serde_json::json!({
        "body_schema": request["body_schema"].clone(),
        "idempotency_key_fingerprint": idempotency_key_fingerprint,
        "operation": request["operation"].clone(),
        "sanitized_body": sanitized_body,
        "schema": request["schema"].clone(),
    });
    let bytes = canonical_json_bytes(&preimage, &ContractLimits::packet0()).unwrap();
    request["request_fingerprint"] = serde_json::Value::String(
        handbook_sdk::contract_membrane::sha256_fingerprint(&bytes)
            .as_str()
            .to_owned(),
    );
}

fn rewrite_response_fingerprints(response: &mut serde_json::Value) {
    let result_preimage = serde_json::json!({
        "blockers": response["blockers"].clone(),
        "data": response["data"].clone(),
        "errors": response["errors"].clone(),
        "operation": response["operation"].clone(),
        "receipts": response["receipts"].clone(),
        "refusals": response["refusals"].clone(),
        "result_schema": response["result_schema"].clone(),
        "status": response["status"].clone(),
    });
    let result_bytes = canonical_json_bytes(&result_preimage, &ContractLimits::packet0()).unwrap();
    response["result_fingerprint"] = serde_json::Value::String(
        handbook_sdk::contract_membrane::sha256_fingerprint(&result_bytes)
            .as_str()
            .to_owned(),
    );
    let mut response_preimage = response.clone();
    response_preimage
        .as_object_mut()
        .unwrap()
        .remove("response_fingerprint");
    let response_bytes =
        canonical_json_bytes(&response_preimage, &ContractLimits::packet0()).unwrap();
    response["response_fingerprint"] = serde_json::Value::String(
        handbook_sdk::contract_membrane::sha256_fingerprint(&response_bytes)
            .as_str()
            .to_owned(),
    );
}

fn matching_receipt() -> WriteReceipt {
    WriteReceipt::new(
        bounded("posture.transition"),
        binding("record.posture-transition"),
        bounded("semantic.record"),
        handbook_sdk::contract_membrane::ContractValue::null(),
        bounded("posture.atomic"),
        &ContractLimits::packet0(),
    )
    .unwrap()
}

fn problem(category: ProblemCategory) -> Problem {
    let (rule, retry, recheck, correlation) = match category {
        ProblemCategory::Prerequisite => (
            None,
            RetryDirective::AfterRecheck,
            Some(
                RecheckCondition::new(
                    binding("recheck.condition"),
                    handbook_sdk::contract_membrane::ContractValue::null(),
                    &ContractLimits::packet0(),
                )
                .unwrap(),
            ),
            None,
        ),
        ProblemCategory::Implementation => (
            None,
            RetryDirective::Never,
            None,
            Some(CorrelationRef::new(bounded("corr_abcdefghijklmnop")).unwrap()),
        ),
        _ => (
            Some(binding("rule.operation")),
            RetryDirective::Never,
            None,
            None,
        ),
    };
    Problem::new(
        bounded(&format!("problem.{category:?}").to_ascii_lowercase()),
        bounded("operation.problem"),
        category,
        binding("operation.subject"),
        rule,
        binding("problem.details"),
        handbook_sdk::contract_membrane::ContractValue::null(),
        BoundedVec::new(Vec::new()).unwrap(),
        retry,
        recheck,
        correlation,
        &ContractLimits::packet0(),
    )
    .unwrap()
}

fn owner_binding(owner: &str, version: &str) -> OperationOwnerBinding {
    let source_identity = match owner {
        "handbook-engine" => "handbook.owner.engine",
        "handbook-flow" => "handbook.owner.flow",
        "handbook-pipeline" => "handbook.owner.pipeline",
        "handbook-sdk" => "handbook.owner.sdk-composition",
        _ => "handbook.owner.deferred",
    };
    OperationOwnerBinding::new(
        bounded(owner),
        SemVer::parse(version).unwrap(),
        ExactBinding::new(
            ExactRef::parse(&format!("{source_identity}@{version}")).unwrap(),
            handbook_sdk::contract_membrane::sha256_fingerprint(source_identity.as_bytes()),
        ),
    )
    .unwrap()
}

fn outcome_schemas() -> OperationOutcomeSchemas {
    OperationOutcomeSchemas::new(
        binding("handbook.operation-blocker"),
        binding("handbook.operation-refusal"),
        binding("handbook.operation-error"),
    )
    .unwrap()
}

fn declared_transports() -> BoundedVec<OperationTransport, 3> {
    BoundedVec::new(vec![
        OperationTransport::CliJson,
        OperationTransport::RustSdk,
        OperationTransport::Tauri,
    ])
    .unwrap()
}

fn bounded<const N: usize>(value: &str) -> BoundedString<N> {
    BoundedString::new(value.to_owned()).unwrap()
}

fn binding(identity: &str) -> ExactBinding {
    ExactBinding::new(
        ExactRef::parse(&format!("{identity}@1.0.0")).unwrap(),
        handbook_sdk::contract_membrane::sha256_fingerprint(identity.as_bytes()),
    )
}
