use handbook_sdk::contract_membrane::{
    sha256_fingerprint, BootstrapFacadeOutcomeKindV1, BootstrapFacadeResponseBodyV1,
    CapabilityConsumer, CapabilityConsumerError, CatalogKindV1, CatalogPageSize,
    CatalogReadOutcomeKindV1, CatalogReadResponseBodyV1, ExactBinding, ExactRef,
    InvocationResponseKind, OperationTransport, SemVer,
};

fn open_consumer() -> (tempfile::TempDir, CapabilityConsumer) {
    let repo = tempfile::tempdir().unwrap();
    let consumer = CapabilityConsumer::open(repo.path()).unwrap();
    (repo, consumer)
}

fn discovered_capability(
    consumer: &CapabilityConsumer,
    operation_ref: &str,
) -> handbook_sdk::contract_membrane::CapabilityEntry {
    let bootstrap = consumer
        .bootstrap(
            SemVer::parse("1.0.0").unwrap(),
            CatalogPageSize::new(128).unwrap(),
            None,
        )
        .unwrap();
    let BootstrapFacadeResponseBodyV1::Ok(response) = bootstrap.response() else {
        panic!("current bootstrap was refused");
    };
    response
        .first_capability_page()
        .entries()
        .iter()
        .find(|entry| entry.operation().reference().as_str() == operation_ref)
        .unwrap()
        .clone()
}

#[test]
fn consumer_closes_bootstrap_catalog_schema_and_read_only_invocation() {
    let (_repo, consumer) = open_consumer();

    let bootstrap = consumer
        .bootstrap(
            SemVer::parse("1.0.0").unwrap(),
            CatalogPageSize::new(128).unwrap(),
            None,
        )
        .unwrap();
    assert_eq!(bootstrap.outcome_kind(), BootstrapFacadeOutcomeKindV1::Ok);
    let BootstrapFacadeResponseBodyV1::Ok(response) = bootstrap.response() else {
        panic!("current bootstrap was refused");
    };
    assert_eq!(response.first_capability_page().entries().len(), 31);

    let catalog = consumer
        .catalog_read(
            CatalogKindV1::Capabilities,
            CatalogPageSize::new(128).unwrap(),
            None,
        )
        .unwrap();
    assert_eq!(catalog.outcome_kind(), CatalogReadOutcomeKindV1::Page);

    let capability = response
        .first_capability_page()
        .entries()
        .iter()
        .find(|entry| entry.operation().reference().as_str() == "artifact.kind.list@1.0.0")
        .unwrap();
    let schema_bytes = consumer.schema_bytes(capability.request_schema()).unwrap();
    assert_eq!(
        sha256_fingerprint(schema_bytes),
        *capability.request_schema().fingerprint()
    );
    assert!(schema_bytes.ends_with(b"\n"));

    let invocation = consumer
        .invoke(
            capability.operation(),
            capability.entry_fingerprint(),
            OperationTransport::CliJson,
            None,
            br#"{}"#,
        )
        .unwrap();
    assert_eq!(
        invocation.response_kind(),
        InvocationResponseKind::Operation
    );
    let wire = serde_json::to_value(&invocation).unwrap();
    assert_eq!(
        wire["response"]["operation"]["ref"],
        capability.operation().reference().as_str()
    );
}

#[test]
fn incompatible_bootstrap_version_is_a_typed_refusal() {
    let (_repo, consumer) = open_consumer();
    let response = consumer
        .bootstrap(
            SemVer::parse("2.0.0").unwrap(),
            CatalogPageSize::new(1).unwrap(),
            None,
        )
        .unwrap();
    assert_eq!(
        response.outcome_kind(),
        BootstrapFacadeOutcomeKindV1::Refused
    );
}

#[test]
fn catalog_cursor_remains_exact_across_fresh_consumer_process_boundaries() {
    let (_repo, first) = open_consumer();
    let first_page = first
        .catalog_read(
            CatalogKindV1::Capabilities,
            CatalogPageSize::new(1).unwrap(),
            None,
        )
        .unwrap();
    let CatalogReadResponseBodyV1::CapabilityPage(first_page) = first_page.response() else {
        panic!("capability page was unavailable");
    };
    let cursor = first_page.next_cursor().unwrap().clone();

    let second = CapabilityConsumer::open(_repo.path()).unwrap();
    let second_page = second
        .catalog_read(
            CatalogKindV1::Capabilities,
            CatalogPageSize::new(1).unwrap(),
            Some(cursor),
        )
        .unwrap();
    let CatalogReadResponseBodyV1::CapabilityPage(second_page) = second_page.response() else {
        panic!("capability cursor did not survive a fresh consumer");
    };
    assert_eq!(second_page.start_index().get(), 1);
}

#[test]
fn stale_or_absent_selection_schema_transport_and_input_fail_closed() {
    let (repo, consumer) = open_consumer();
    let sentinel = repo.path().join("sentinel.txt");
    std::fs::write(&sentinel, b"unchanged").unwrap();
    let capability = discovered_capability(&consumer, "artifact.kind.list@1.0.0");

    let unknown = ExactBinding::new(
        ExactRef::parse("handbook.use-case.repository-doctor@1.0.0").unwrap(),
        sha256_fingerprint(b"fixed-use-case-is-not-a-capability"),
    );
    assert_eq!(
        consumer.invoke(
            &unknown,
            capability.entry_fingerprint(),
            OperationTransport::CliJson,
            None,
            br#"{}"#,
        ),
        Err(CapabilityConsumerError::CapabilityUnavailable)
    );

    let stale_version = ExactBinding::new(
        ExactRef::parse("artifact.kind.list@2.0.0").unwrap(),
        capability.operation().fingerprint().clone(),
    );
    assert_eq!(
        consumer.invoke(
            &stale_version,
            capability.entry_fingerprint(),
            OperationTransport::CliJson,
            None,
            br#"{}"#,
        ),
        Err(CapabilityConsumerError::CapabilityUnavailable)
    );

    let stale_definition = ExactBinding::new(
        capability.operation().reference().clone(),
        sha256_fingerprint(b"stale-definition"),
    );
    assert_eq!(
        consumer.invoke(
            &stale_definition,
            capability.entry_fingerprint(),
            OperationTransport::CliJson,
            None,
            br#"{}"#,
        ),
        Err(CapabilityConsumerError::DefinitionBindingMismatch)
    );
    assert_eq!(
        consumer.invoke(
            capability.operation(),
            &sha256_fingerprint(b"stale-entry"),
            OperationTransport::CliJson,
            None,
            br#"{}"#,
        ),
        Err(CapabilityConsumerError::CapabilityEntryMismatch)
    );

    let stale_schema = ExactBinding::new(
        capability.request_schema().reference().clone(),
        sha256_fingerprint(b"stale-schema"),
    );
    assert_eq!(
        consumer.schema_bytes(&stale_schema),
        Err(CapabilityConsumerError::SchemaUnavailable)
    );

    let posture = discovered_capability(&consumer, "posture.transition.apply@1.0.0");
    assert_eq!(
        consumer.invoke(
            posture.operation(),
            posture.entry_fingerprint(),
            OperationTransport::CliJson,
            None,
            br#"{}"#,
        ),
        Err(CapabilityConsumerError::TransportUnavailable)
    );

    assert_eq!(
        consumer.invoke(
            capability.operation(),
            capability.entry_fingerprint(),
            OperationTransport::CliJson,
            None,
            br#"{"unexpected":true}"#,
        ),
        Err(CapabilityConsumerError::InvalidInput)
    );
    assert_eq!(
        consumer.invoke(
            capability.operation(),
            capability.entry_fingerprint(),
            OperationTransport::CliJson,
            None,
            br#"{"#,
        ),
        Err(CapabilityConsumerError::InvalidInput)
    );

    let mutation = discovered_capability(&consumer, "intake.record.append@1.0.0");
    assert_eq!(
        consumer.invoke(
            mutation.operation(),
            mutation.entry_fingerprint(),
            OperationTransport::CliJson,
            None,
            br#"{}"#,
        ),
        Err(CapabilityConsumerError::InvalidInput)
    );

    assert_eq!(std::fs::read(&sentinel).unwrap(), b"unchanged");
    let entries = std::fs::read_dir(repo.path())
        .unwrap()
        .map(|entry| entry.unwrap().file_name())
        .collect::<Vec<_>>();
    assert_eq!(entries, [std::ffi::OsString::from("sentinel.txt")]);
}
