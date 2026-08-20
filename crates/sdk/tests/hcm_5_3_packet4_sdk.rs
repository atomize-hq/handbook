use handbook_sdk::contract_membrane::{
    admit_operation_definition, canonical_json_bytes, dock_manifest_list_operation_request,
    dock_manifest_read_operation_request, dock_operation_definitions, execute_dock_operation,
    operation_inventory, CapabilityConsumer, ContractLimits, ContractRecordBindingDtoV1,
    DiscoveryState, Fingerprint, OperationAdmissionResult, OperationClass, OperationIdempotency,
    OperationOwner, OperationTransport, ProductionWitness, RequestId, UtcTimestamp,
    DOCK_MANIFEST_LIST_REF, DOCK_MANIFEST_READ_REF, DOCK_OPERATION_REFS, DOCK_RUN_REF,
};
use handbook_sdk::contract_membrane::{DockManifestListRequestV1, DockManifestReadRequestV1};
use serde_json::Value;
use tempfile::tempdir;

#[test]
fn the_three_reserved_rows_are_live_dock_owned_and_exact() {
    assert_eq!(operation_inventory().len(), 62);
    assert_eq!(DOCK_OPERATION_REFS.len(), 3);

    let expected = [
        (
            DOCK_MANIFEST_LIST_REF,
            OperationClass::ReadOnly,
            "safe",
            "sha256:c64b2974ccdcea9004504ff79b5b5aba6dd351cea236abfc93be7dd4f1a1f60e",
        ),
        (
            DOCK_MANIFEST_READ_REF,
            OperationClass::ReadOnly,
            "safe",
            "sha256:9f98e8f72fe92af145e89d50ddb307a3210806c545dad99c55f17a2af4ac0942",
        ),
        (
            DOCK_RUN_REF,
            OperationClass::AppendOnly,
            "idempotency_key_required",
            "sha256:515032d7abaeacff00ae865594d09bc7a4610117161ccec04f6595a00392ad73",
        ),
    ];
    let definitions = dock_operation_definitions().unwrap();
    assert_eq!(definitions.len(), expected.len());

    for ((operation_ref, class, idempotency, fingerprint), definition) in
        expected.into_iter().zip(definitions)
    {
        let row = operation_inventory()
            .iter()
            .find(|row| row.operation_ref().as_str() == operation_ref)
            .unwrap();
        assert_eq!(row.owner(), OperationOwner::Dock);
        assert_eq!(row.class(), class);
        assert_eq!(row.discovery(), DiscoveryState::Discoverable);
        assert_eq!(
            row.current_transports(),
            [
                OperationTransport::CliJson,
                OperationTransport::RustSdk,
                OperationTransport::Tauri,
            ]
        );
        assert_eq!(definition.operation().as_str(), operation_ref);
        assert_eq!(definition.class(), class);
        assert_eq!(definition.definition_fingerprint().as_str(), fingerprint);
        if operation_ref == DOCK_RUN_REF {
            assert_eq!(
                handbook_sdk::contract_membrane::canonical_fingerprint(
                    &definition.write_set(),
                    &ContractLimits::packet0(),
                )
                .unwrap()
                .as_str(),
                "sha256:cfb2f1fecb8402a1f44b41c89e4da8aa53a321aff86a90cee6b1f15707d7d5af"
            );
        }
        assert_eq!(
            match definition.idempotency() {
                OperationIdempotency::Safe => "safe",
                OperationIdempotency::IdempotencyKeyRequired { .. } => {
                    "idempotency_key_required"
                }
                OperationIdempotency::CompareAndWriteRequired { .. } => {
                    "compare_and_write_required"
                }
            },
            idempotency
        );
        assert!(matches!(
            admit_operation_definition(&definition)
                .unwrap_or_else(|error| panic!("{operation_ref}: {error:?}")),
            OperationAdmissionResult::Admitted(_)
        ));
    }
}

#[test]
fn dock_owner_admission_is_independent_exact_and_structurally_preceded() {
    use handbook_dock::operation_admission::{
        admit_dock_operation, OperationAdmissionClaim, OwnerAdmissionRefusal,
    };

    let exact = OperationAdmissionClaim {
        operation_ref: DOCK_MANIFEST_LIST_REF,
        definition_fingerprint:
            "sha256:c64b2974ccdcea9004504ff79b5b5aba6dd351cea236abfc93be7dd4f1a1f60e",
        owner_crate: "handbook-dock",
        owner_version: "0.1.0",
        owner_source_ref: "handbook.owner.dock@0.1.0",
        owner_source_fingerprint:
            "sha256:7eb77f6c73e5fb80e2ba1e7bc16411b94517910b51932294d6c128b42a33a38e",
        mutability: "read_only",
        idempotency: "safe",
        idempotency_fingerprint:
            "sha256:e3a5961ff3a3a8141099426d699e73c9d6dd073456c0721a8cbd4a2f9bb9015d",
        definition_schema_ref: "handbook.operation-definition@1.0.0",
        definition_schema_fingerprint:
            "sha256:e668b32cb7ef3725e4d8c53696ec9032ba0741e387436883a7d8f264602ddd09",
        request_schema_ref: "handbook.operation.dock-manifest-list-request@1.0.0",
        request_schema_fingerprint:
            "sha256:1f3983b882dac4fc18e685167defdf27e8ab6eacfc2da85ee95b82bb0895887e",
        result_schema_ref: "handbook.operation.dock-manifest-list-result@1.0.0",
        result_schema_fingerprint:
            "sha256:0164fcffa45a6da7f68186f92da838d8c5739c19b783724575c3f1d4f662f970",
        blocker_schema_ref: "handbook.operation-blocker@1.0.0",
        blocker_schema_fingerprint:
            "sha256:1125afb3a783af6ed4ee5b8caad86e00264420103d18db36e0337e34b889a009",
        refusal_schema_ref: "handbook.operation-refusal@1.0.0",
        refusal_schema_fingerprint:
            "sha256:16357baa6276ae2d6b27e22296e789db78d646403845f98653148178afec8b23",
        error_schema_ref: "handbook.operation-error@1.0.0",
        error_schema_fingerprint:
            "sha256:569cce41b965e3140591ed8c75ebad39aef2809c57e82b5c8f50db639d634355",
        write_set_fingerprint:
            "sha256:4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945",
        required_capabilities_fingerprint:
            "sha256:4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945",
    };
    assert_eq!(
        admit_dock_operation(&exact).unwrap().operation_ref(),
        DOCK_MANIFEST_LIST_REF
    );
    let mut substituted_source = exact;
    substituted_source.owner_source_fingerprint =
        "sha256:0000000000000000000000000000000000000000000000000000000000000000";
    assert_eq!(
        admit_dock_operation(&substituted_source),
        Err(OwnerAdmissionRefusal::DefinitionMismatch)
    );
    let mut wrong_owner = exact;
    wrong_owner.owner_crate = "handbook-sdk";
    assert_eq!(
        admit_dock_operation(&wrong_owner),
        Err(OwnerAdmissionRefusal::OwnerBindingMismatch)
    );

    let admission_source = include_str!("../src/contract_membrane/owner_admission.rs");
    assert!(
        admission_source.find("if row.class() != definition.class()")
            < admission_source.find("match row.owner()")
    );
}

#[test]
fn empty_production_manifest_use_cases_are_re_admitted_outward() {
    let repo = tempdir().unwrap();
    let list = dock_manifest_list_operation_request(
        Some("packet4-manifest-list"),
        DockManifestListRequestV1::first(25).unwrap(),
    )
    .unwrap();
    let listed = execute_dock_operation(repo.path(), &list).unwrap();
    let listed: Value = serde_json::from_slice(
        &canonical_json_bytes(
            listed.terminal().data().unwrap(),
            &ContractLimits::packet0(),
        )
        .unwrap(),
    )
    .unwrap();
    assert_eq!(listed["entries"], serde_json::json!([]));
    assert_eq!(listed["next_cursor"], Value::Null);
    assert_eq!(listed["retention"], "immutable_exact");

    let requested = ContractRecordBindingDtoV1::new(
        "handbook.dock.absent@1.0.0",
        Fingerprint::parse(&format!("sha256:{}", "a".repeat(64))).unwrap(),
    )
    .unwrap();
    let read = dock_manifest_read_operation_request(
        Some("packet4-manifest-read"),
        DockManifestReadRequestV1::new(requested),
    )
    .unwrap();
    let read = execute_dock_operation(repo.path(), &read).unwrap();
    let read: Value = serde_json::from_slice(
        &canonical_json_bytes(read.terminal().data().unwrap(), &ContractLimits::packet0()).unwrap(),
    )
    .unwrap();
    assert_eq!(read["outcome"], "absent");
}

#[test]
fn generic_capability_consumer_dispatches_dock_without_a_specific_adapter() {
    let repo = tempdir().unwrap();
    let witness =
        ProductionWitness::from_current(UtcTimestamp::parse("9999-12-31T23:59:59Z").unwrap())
            .unwrap();
    let capability = witness
        .capabilities()
        .iter()
        .find(|entry| entry.operation().reference().as_str() == DOCK_MANIFEST_LIST_REF)
        .unwrap();
    let body = canonical_json_bytes(
        &DockManifestListRequestV1::first(10).unwrap(),
        &ContractLimits::packet0(),
    )
    .unwrap();
    let response = CapabilityConsumer::open(repo.path())
        .unwrap()
        .invoke(
            capability.operation(),
            capability.entry_fingerprint(),
            OperationTransport::RustSdk,
            Some(RequestId::parse("packet4-generic-dock-list").unwrap()),
            &body,
        )
        .unwrap();
    let bytes = canonical_json_bytes(&response, &ContractLimits::packet0()).unwrap();
    let rendered = String::from_utf8(bytes).unwrap();
    assert!(rendered.contains("\"entries\":[]"));
    assert!(rendered.contains(DOCK_MANIFEST_LIST_REF));

    let consumer_source = include_str!("../src/contract_membrane/consumer.rs");
    assert!(!consumer_source.contains("dock.manifest.list"));
    assert!(!consumer_source.contains("dock.manifest.read"));
    assert!(!consumer_source.contains("dock.run"));
}

#[test]
fn run_preparation_and_readmission_precede_process_body_and_executor_access() {
    let source = include_str!("../src/contract_membrane/dock_operation.rs");
    let default_start = source.find("fn execute_run(").unwrap();
    let runtime_start = source.find("fn execute_run_with_runtime(").unwrap();
    let bridge_start = source.find("struct DockRunBridge").unwrap();
    let default_run = &source[default_start..runtime_start];
    let runtime_run = &source[runtime_start..bridge_start];

    for run in [default_run, runtime_run] {
        assert!(
            run.find("prepare_current_run(").unwrap() < run.find("from_body(request)").unwrap()
        );
    }
    assert!(
        runtime_run.find("prepare_current_run(").unwrap()
            < runtime_run.find("context.manifest.binding()").unwrap()
    );
    assert!(
        runtime_run.find("prepare_current_run(").unwrap()
            < runtime_run.find("with_owner_idempotency_key").unwrap()
    );
    let preparation_start = source.find("fn prepare_current_run(").unwrap();
    let preparation = &source[preparation_start..bridge_start];
    assert!(
        preparation.find("prepare_evaluation_run_v1").unwrap()
            < preparation
                .find("readmit_current_evaluation_run_v1")
                .unwrap()
    );
    assert!(!preparation.contains("context.manifest"));
    assert!(!preparation.contains("DockProcessExecutorV1"));
}

#[test]
fn production_witness_closes_the_exact_168_root_31_capability_inventory() {
    let witness =
        ProductionWitness::from_current(UtcTimestamp::parse("9999-12-31T23:59:59Z").unwrap())
            .unwrap();
    assert_eq!(witness.schema_manifest().len(), 168);
    assert_eq!(witness.capabilities().len(), 31);
    assert!(witness
        .owner_versions()
        .iter()
        .any(|owner| owner.owner_crate() == "handbook-dock"));
    let roots = witness
        .schema_manifest()
        .iter()
        .map(|entry| entry.schema().reference().as_str())
        .collect::<std::collections::BTreeSet<_>>();
    for expected in [
        "handbook.dock-manifest@1.0.0",
        "handbook.dock-process-request@1.0.0",
        "handbook.dock-process-result@1.0.0",
        "handbook.dock-execution-record@1.0.0",
        "handbook.dock-operation-details@1.0.0",
        "handbook.operation.dock-manifest-list-request@1.0.0",
        "handbook.operation.dock-manifest-list-result@1.0.0",
        "handbook.operation.dock-manifest-read-request@1.0.0",
        "handbook.operation.dock-manifest-read-result@1.0.0",
        "handbook.operation.dock-run-request@1.0.0",
        "handbook.operation.dock-run-result@1.0.0",
    ] {
        assert!(roots.contains(expected));
    }
}
