use handbook_sdk::contract_membrane::{
    canonical_json_bytes, parse_canonical_json, BoundedVec, ContractError, ContractLimits,
    InvocationResponseKind, InvocationResponseV1, RepositoryDoctorDetailsV1,
    RepositoryDoctorResultV1, TerminalStatus, TransportFailureDetailsV1, TransportFailureKind,
    TransportFailureV1, UseCaseResponseV1,
};
use handbook_sdk::{DoctorReport, RepositoryReadinessStatus};

fn doctor_report(status: RepositoryReadinessStatus) -> DoctorReport {
    DoctorReport {
        profile_ref: "handbook.profile.shipped-root@1.2.0".to_owned(),
        profile_fingerprint:
            "sha256:1111111111111111111111111111111111111111111111111111111111111111".to_owned(),
        stable_role_registry_ref: "handbook.roles.core@1.1.0".to_owned(),
        stable_role_registry_fingerprint:
            "sha256:2222222222222222222222222222222222222222222222222222222222222222".to_owned(),
        conditions: vec![],
        capabilities: vec![],
        artifacts: vec![],
        project_context: None,
        charter: None,
        status,
    }
}

#[test]
fn doctor_use_case_closes_status_bindings_fingerprints_and_typed_round_trip() {
    let limits = ContractLimits::packet0();
    for (status, expected_terminal, expected_code) in [
        (RepositoryReadinessStatus::Ready, TerminalStatus::Ok, None),
        (
            RepositoryReadinessStatus::ActionRequired,
            TerminalStatus::Blocked,
            Some("repository.action_required"),
        ),
        (
            RepositoryReadinessStatus::Indeterminate,
            TerminalStatus::Blocked,
            Some("repository.readiness_indeterminate"),
        ),
        (
            RepositoryReadinessStatus::Invalid,
            TerminalStatus::Blocked,
            Some("repository.readiness_invalid"),
        ),
    ] {
        let report = doctor_report(status);
        let use_case =
            UseCaseResponseV1::repository_doctor(&report, &limits).expect("doctor projection");
        assert_eq!(use_case.status(), expected_terminal);
        assert_eq!(
            use_case.use_case().reference().as_str(),
            "handbook.use-case.repository-doctor@1.0.0"
        );
        assert_eq!(
            use_case.result_schema().reference().as_str(),
            "handbook.use-case.repository-doctor-result@1.0.0"
        );
        assert_eq!(
            use_case
                .terminal_problems()
                .first()
                .map(|problem| problem.code().as_str()),
            expected_code
        );

        let outer = InvocationResponseV1::use_case(use_case, &limits).expect("outer response");
        assert_eq!(outer.response_kind(), InvocationResponseKind::UseCase);
        let bytes = canonical_json_bytes(&outer, &limits).expect("canonical bytes");
        let parsed = InvocationResponseV1::parse_and_admit(&bytes, None, &limits)
            .expect("typed outer admission");
        assert_eq!(parsed, outer);
        assert_eq!(
            parsed.repository_doctor_outcome(&limits).unwrap(),
            Some(Ok(report))
        );
    }
}

#[test]
fn invocation_and_use_case_discriminators_fail_closed() {
    let limits = ContractLimits::packet0();
    let response = InvocationResponseV1::use_case(
        UseCaseResponseV1::repository_doctor(
            &doctor_report(RepositoryReadinessStatus::Ready),
            &limits,
        )
        .unwrap(),
        &limits,
    )
    .unwrap();
    let bytes = canonical_json_bytes(&response, &limits).unwrap();
    let wire = String::from_utf8(bytes).unwrap();

    let unknown = wire.replacen("\"use_case\"", "\"unknown\"", 1);
    assert!(matches!(
        InvocationResponseV1::parse_and_admit(unknown.as_bytes(), None, &limits),
        Err(ContractError::InvalidJson | ContractError::InvalidInvariant)
    ));

    let mismatch = wire.replacen("\"use_case\"", "\"transport_failure\"", 1);
    assert!(InvocationResponseV1::parse_and_admit(mismatch.as_bytes(), None, &limits).is_err());
}

#[test]
fn public_family_is_nominal_and_transport_failure_can_never_report_ok_or_receipts() {
    fn assert_public<T>() {}
    assert_public::<RepositoryDoctorResultV1>();
    assert_public::<RepositoryDoctorDetailsV1>();
    assert_public::<TransportFailureDetailsV1>();

    let limits = ContractLimits::packet0();
    let failure = TransportFailureV1::new(
        TransportFailureKind::Serialization,
        TerminalStatus::Error,
        BoundedVec::new(Vec::new()).unwrap(),
        BoundedVec::new(Vec::new()).unwrap(),
        BoundedVec::new(Vec::new()).unwrap(),
        BoundedVec::new(Vec::new()).unwrap(),
        &limits,
    );
    assert_eq!(failure, Err(ContractError::InvalidInvariant));

    for (kind, expected_status) in [
        (TransportFailureKind::Repository, TerminalStatus::Blocked),
        (TransportFailureKind::Acquisition, TerminalStatus::Refused),
        (TransportFailureKind::Decoding, TerminalStatus::Refused),
        (TransportFailureKind::Framing, TerminalStatus::Error),
        (TransportFailureKind::Serialization, TerminalStatus::Error),
        (TransportFailureKind::Emission, TerminalStatus::Error),
        (TransportFailureKind::Adapter, TerminalStatus::Error),
    ] {
        let failure = TransportFailureV1::adapter_error(kind, "transport.test", &limits)
            .unwrap_or_else(|error| panic!("{kind:?}: {error:?}"));
        assert_eq!(failure.status(), expected_status);
        let invocation = InvocationResponseV1::transport_failure(failure, &limits).unwrap();
        let bytes = canonical_json_bytes(&invocation, &limits).unwrap();
        assert_eq!(
            InvocationResponseV1::parse_and_admit(&bytes, None, &limits).unwrap(),
            invocation
        );
    }

    let schema_bytes = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("schemas/manifest.json"),
    )
    .unwrap();
    assert!(!schema_bytes.contains("raw_idempotency_key"));
    assert!(!schema_bytes.contains("idempotency_key\""));
}

#[test]
fn six_success_result_types_are_distinct_closed_public_roots() {
    fn assert_public<T>() {}
    assert_public::<handbook_sdk::contract_membrane::RepositorySetupResultV1>();
    assert_public::<handbook_sdk::contract_membrane::RepositoryDoctorResultV1>();
    assert_public::<handbook_sdk::contract_membrane::AuthorCharterResultV1>();
    assert_public::<handbook_sdk::contract_membrane::AuthorProjectContextResultV1>();
    assert_public::<handbook_sdk::contract_membrane::ApproverAdministrationResultV1>();
    assert_public::<handbook_sdk::contract_membrane::PipelineRouteResolveAndPersistResultV1>();

    let empty: Result<UseCaseResponseV1, ContractError> =
        parse_canonical_json(b"{}", &ContractLimits::packet0());
    assert!(empty.is_err(), "empty use-case responses must fail closed");
}

#[test]
fn checked_in_invocation_schema_admits_doctor_and_closes_the_current_use_case_union() {
    let limits = ContractLimits::packet0();
    let outer = InvocationResponseV1::use_case(
        UseCaseResponseV1::repository_doctor(
            &doctor_report(RepositoryReadinessStatus::ActionRequired),
            &limits,
        )
        .unwrap(),
        &limits,
    )
    .unwrap();
    let schema_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("schemas/2020-12/handbook.invocation-response/1.0.0.schema.json");
    let schema_bytes = std::fs::read(&schema_path).unwrap();
    assert_eq!(
        outer.schema().fingerprint(),
        &handbook_sdk::contract_membrane::sha256_fingerprint(&schema_bytes)
    );
    let schema: serde_json::Value = serde_json::from_slice(&schema_bytes).unwrap();
    let validator = jsonschema::draft202012::options().build(&schema).unwrap();
    assert!(validator.is_valid(&serde_json::to_value(&outer).unwrap()));

    let use_case_schema_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("schemas/2020-12/handbook.use-case-response/1.0.0.schema.json");
    let use_case_schema: serde_json::Value =
        serde_json::from_slice(&std::fs::read(use_case_schema_path).unwrap()).unwrap();
    assert_eq!(use_case_schema["oneOf"].as_array().unwrap().len(), 9);
    let wire = use_case_schema.to_string();
    for reference in [
        "handbook.use-case.repository-setup@1.0.0",
        "handbook.use-case.repository-doctor@1.0.0",
        "handbook.use-case.author-charter@1.0.0",
        "handbook.use-case.author-project-context@1.0.0",
        "handbook.use-case.approver-administration@1.0.0",
        "handbook.use-case.pipeline-route-resolve-and-persist@1.0.0",
    ] {
        assert!(wire.contains(reference));
    }
    assert!(wire.contains("handbook.use-case.repository-doctor-details@1.0.0"));
}

#[test]
fn doctor_use_case_does_not_inflate_operation_admission_or_discovery() {
    assert!(handbook_sdk::contract_membrane::operation_inventory()
        .iter()
        .all(|row| !row.operation_ref().as_str().contains("repository-doctor")));
    let witness = handbook_sdk::contract_membrane::ProductionWitness::from_current(
        handbook_sdk::contract_membrane::UtcTimestamp::parse("2030-01-01T00:00:00Z").unwrap(),
    )
    .unwrap();
    assert_eq!(
        witness.catalog_roots().operations().total_entries().get(),
        1
    );
}
