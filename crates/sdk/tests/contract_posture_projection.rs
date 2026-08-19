use handbook_sdk::contract_membrane::{
    admit_operation_definition, admit_posture_transition_apply_request, canonical_json_bytes,
    operation_inventory, posture_transition_apply_definition, posture_transition_apply_request,
    AdmissionReason, ContractError, ContractLimits, DiscoveryState, IdempotencyStateRef,
    OperationAdmissionResult, OperationClass, OperationIdempotency, OperationTransport,
    PostureCanonicalBindingDtoV1, PostureDimensionChangeDtoV1, PostureReferenceDtoV1,
    PostureTransitionApplyRequestDataV1, ProductionWitness, TerminalStatus, UtcTimestamp,
};

const RAW_KEY: &str = "packet-four-secret-0001";

fn request_data() -> PostureTransitionApplyRequestDataV1 {
    PostureTransitionApplyRequestDataV1::new(
        format!("sha256:{}", "1".repeat(64)),
        PostureCanonicalBindingDtoV1::new(
            ".handbook/project/charter.yaml".to_owned(),
            format!("sha256:{}", "2".repeat(64)),
            format!("sha256:{}", "2".repeat(64)),
            123,
        )
        .unwrap(),
        PostureReferenceDtoV1::new(
            format!("candidates/candidate_{}.yaml", "3".repeat(64)),
            format!("sha256:{}", "3".repeat(64)),
        )
        .unwrap(),
        vec!["engineering_posture.dimensions".to_owned()],
        PostureDimensionChangeDtoV1::new(
            "testing_rigor".to_owned(),
            2,
            "/engineering_posture/dimensions/2/level_override".to_owned(),
            Some(4),
            4,
            3,
        )
        .unwrap(),
        "2026-08-10T16:30:00Z".to_owned(),
    )
    .unwrap()
}

#[test]
fn exact_definition_is_owner_admitted_and_pins_two_atomic_writes() {
    let definition = posture_transition_apply_definition().unwrap();
    assert_eq!(
        definition.operation().as_str(),
        "posture.transition.apply@1.0.0"
    );
    assert_eq!(definition.class(), OperationClass::CompareAndWrite);
    assert!(definition.required_capabilities().is_empty());
    let OperationIdempotency::CompareAndWriteRequired {
        key_pointer,
        expected_basis,
    } = definition.idempotency()
    else {
        panic!("posture apply must be compare-and-write")
    };
    assert_eq!(key_pointer.as_str(), "/body/idempotency_key");
    assert_eq!(
        expected_basis
            .as_slice()
            .iter()
            .map(|pointer| pointer.as_str())
            .collect::<Vec<_>>(),
        vec![
            "/body/change",
            "/body/effective_at_utc",
            "/body/expected_canonical",
            "/body/reassessment_coverage_ids",
            "/body/recommendation",
            "/body/repository_identity_fingerprint",
        ]
    );
    assert_eq!(definition.write_set().len(), 2);
    assert_eq!(
        (
            definition.write_set()[0].record_kind(),
            definition.write_set()[0].authority_class(),
            definition.write_set()[0].atomic_group(),
            definition.write_set()[0]
                .condition_schema()
                .reference()
                .as_str(),
        ),
        (
            "constitutional_root",
            "canonical_truth",
            "posture_transition_apply",
            "handbook.posture-write-condition.canonical-truth@1.0.0",
        )
    );
    assert_eq!(
        (
            definition.write_set()[1].record_kind(),
            definition.write_set()[1].authority_class(),
            definition.write_set()[1].atomic_group(),
            definition.write_set()[1]
                .condition_schema()
                .reference()
                .as_str(),
        ),
        (
            "posture_transition",
            "semantic_record",
            "posture_transition_apply",
            "handbook.posture-write-condition.semantic-record@1.0.0",
        )
    );
    assert!(matches!(
        admit_operation_definition(&definition).unwrap(),
        OperationAdmissionResult::Admitted(_)
    ));
}

#[test]
fn admitted_request_confines_the_raw_key_and_enforces_required_nullable_correlation() {
    let limits = ContractLimits::packet0();
    let definition = posture_transition_apply_definition().unwrap();
    let request =
        posture_transition_apply_request(Some("packet-four-request"), RAW_KEY, request_data())
            .unwrap();

    assert!(!format!("{request:?}").contains(RAW_KEY));
    assert!(!canonical_json_bytes(request.body(), &limits)
        .unwrap()
        .windows(RAW_KEY.len())
        .any(|window| window == RAW_KEY.as_bytes()));
    let wire = canonical_json_bytes(&request, &limits).unwrap();
    assert!(wire
        .windows(RAW_KEY.len())
        .any(|window| window == RAW_KEY.as_bytes()));

    let mut value: serde_json::Value = serde_json::from_slice(&wire).unwrap();
    value["request_id"] = serde_json::Value::Null;
    let explicit_null = canonical_json_bytes(&value, &limits).unwrap();
    assert!(admit_posture_transition_apply_request(&explicit_null)
        .unwrap()
        .request_id()
        .is_none());

    value.as_object_mut().unwrap().remove("request_id").unwrap();
    let absent = canonical_json_bytes(&value, &limits).unwrap();
    assert!(matches!(
        admit_posture_transition_apply_request(&absent),
        Err(ContractError::InvalidJson)
    ));

    let mut invalid: serde_json::Value = serde_json::from_slice(&wire).unwrap();
    invalid["request_id"] = serde_json::json!("invalid request id");
    let invalid = canonical_json_bytes(&invalid, &limits).unwrap();
    assert!(matches!(
        admit_posture_transition_apply_request(&invalid),
        Err(ContractError::InvalidString)
    ));

    assert_eq!(request.operation(), &definition.binding());

    let mut unknown: serde_json::Value = serde_json::from_slice(&wire).unwrap();
    unknown["operation"]["ref"] = serde_json::json!("unknown.operation@1.0.0");
    let unknown = canonical_json_bytes(&unknown, &limits).unwrap();
    assert!(matches!(
        admit_posture_transition_apply_request(&unknown),
        Err(ContractError::InvalidInvariant | ContractError::FingerprintMismatch)
    ));
}

#[test]
fn production_witness_retains_the_proven_rust_posture_capability() {
    let rows = operation_inventory();
    let posture = rows
        .iter()
        .find(|row| row.operation_id() == "posture.transition.apply")
        .unwrap();
    assert_eq!(posture.admission_reason(), AdmissionReason::Admitted);
    assert_eq!(
        posture.current_transports(),
        &[OperationTransport::RustSdk, OperationTransport::Tauri]
    );
    assert_eq!(posture.discovery(), DiscoveryState::Discoverable);
    assert_eq!(
        rows.iter()
            .filter(|row| !row.current_transports().is_empty())
            .count(),
        28
    );

    let witness =
        ProductionWitness::from_current(UtcTimestamp::parse("2030-01-01T00:00:00Z").unwrap())
            .unwrap();
    assert_eq!(witness.capabilities().len(), 28);
    let capability = witness
        .capabilities()
        .iter()
        .find(|entry| entry.operation().reference().as_str() == "posture.transition.apply@1.0.0")
        .unwrap();
    assert_eq!(
        capability.operation().reference().as_str(),
        "posture.transition.apply@1.0.0"
    );
    assert_eq!(
        capability.current_transports(),
        &[OperationTransport::RustSdk, OperationTransport::Tauri]
    );
}

#[test]
fn closed_non_success_state_carries_no_semantic_data_or_receipts() {
    let request = posture_transition_apply_request(None, RAW_KEY, request_data()).unwrap();
    let sdk = handbook_sdk::HandbookSdkV1::open("path-that-does-not-exist");
    let response = sdk.apply_posture_transition_operation(&request).unwrap();
    assert_eq!(response.terminal().status(), TerminalStatus::Refused);
    assert!(response.terminal().data().is_none());
    assert!(response.receipts().is_empty());
    assert!(matches!(
        response.idempotency().as_ref(),
        IdempotencyStateRef::NotEstablished { .. }
    ));
    let bytes = canonical_json_bytes(&response, &ContractLimits::packet0()).unwrap();
    assert!(!bytes
        .windows(RAW_KEY.len())
        .any(|window| window == RAW_KEY.as_bytes()));
}
