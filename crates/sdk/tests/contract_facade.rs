use handbook_sdk::contract_membrane::{
    canonical_json_bytes, pipeline_catalog_list_operation_request, sha256_fingerprint,
    ApproverAdministrationRequestV1, AuthorCharterRequestV1, AuthorProjectContextRequestV1,
    BootstrapFacadeOutcomeKindV1, BootstrapFacadeResponseV1, BootstrapOutcome, BootstrapRequestV1,
    CatalogKindV1, CatalogPageSize, CatalogReadOutcomeKindV1, CatalogReadRequestV1,
    CatalogReadResponseV1, CharterAcquisitionModeV1, CharterInputValueV1, CharterIntakeConsumerV1,
    CharterIntakeEnvelopeV1, ContractError, ContractLimits, ExactBinding, ExactRef,
    InvocationRequestKindV1, InvocationRequestV1, PipelineRouteResolveAndPersistRequestV1,
    ProductionCatalogRead, ProductionWitness, ProjectContextInputV1, RepositorySetupModeV1,
    RepositorySetupRequestV1, SemVer, UseCaseRequestV1, UtcTimestamp,
};
use serde_json::Value;

const ZERO_FINGERPRINT: &str =
    "sha256:0000000000000000000000000000000000000000000000000000000000000000";
const RETENTION: &str = "2030-01-01T00:00:00Z";
const NOW: &str = "2029-01-01T00:00:00Z";

#[test]
fn six_use_case_requests_are_exact_typed_and_canonical() {
    let limits = ContractLimits::packet0();
    let requests = vec![
        UseCaseRequestV1::repository_setup(
            RepositorySetupRequestV1::new(RepositorySetupModeV1::Init, false, true),
            &limits,
        )
        .unwrap(),
        UseCaseRequestV1::repository_doctor(&limits).unwrap(),
        UseCaseRequestV1::author_charter(
            AuthorCharterRequestV1::Author {
                mode: CharterAcquisitionModeV1::Express,
                intake: CharterIntakeEnvelopeV1::new(
                    CharterAcquisitionModeV1::Express,
                    CharterInputValueV1::string("").unwrap(),
                    Vec::new(),
                    CharterIntakeConsumerV1::new("facade", "contract-test", "1.0.0").unwrap(),
                    Vec::new(),
                    UtcTimestamp::parse(NOW).unwrap(),
                    None,
                )
                .unwrap(),
            },
            &limits,
        )
        .unwrap(),
        UseCaseRequestV1::author_project_context(
            AuthorProjectContextRequestV1::Validate(
                ProjectContextInputV1::new(
                    "handbook.project-context",
                    "1.0.0",
                    "project-context.current",
                    "Packet one contract fixture",
                    vec!["SDK boundary".to_owned()],
                    vec!["SDK composition".to_owned()],
                    vec!["docs/spec.md".to_owned()],
                    vec!["none".to_owned()],
                )
                .unwrap(),
            ),
            &limits,
        )
        .unwrap(),
        UseCaseRequestV1::approver_administration(
            ApproverAdministrationRequestV1::revoke_credential("sha256:credential").unwrap(),
            &limits,
        )
        .unwrap(),
        UseCaseRequestV1::pipeline_route_resolve_and_persist(
            PipelineRouteResolveAndPersistRequestV1::new("release").unwrap(),
            &limits,
        )
        .unwrap(),
    ];
    let expected_refs = [
        "handbook.use-case.repository-setup@1.0.0",
        "handbook.use-case.repository-doctor@1.0.0",
        "handbook.use-case.author-charter@1.0.0",
        "handbook.use-case.author-project-context@1.0.0",
        "handbook.use-case.approver-administration@1.0.0",
        "handbook.use-case.pipeline-route-resolve-and-persist@1.0.0",
    ];

    for (request, expected_ref) in requests.iter().zip(expected_refs) {
        assert_eq!(
            request.schema().reference().as_str(),
            "handbook.use-case-request@1.0.0"
        );
        assert_eq!(request.use_case().reference().as_str(), expected_ref);
        let bytes = canonical_json_bytes(request, &limits).unwrap();
        let admitted = UseCaseRequestV1::parse_and_admit(&bytes, &limits).unwrap();
        assert_eq!(admitted.use_case(), request.use_case());
        assert_eq!(
            admitted.request_fingerprint(),
            request.request_fingerprint()
        );
        assert_eq!(canonical_json_bytes(&admitted, &limits).unwrap(), bytes);
    }
}

#[test]
fn invocation_request_closes_operation_and_use_case_discriminators() {
    let limits = ContractLimits::packet0();
    let operation = InvocationRequestV1::operation(
        pipeline_catalog_list_operation_request(Some("facade.request")).unwrap(),
        &limits,
    )
    .unwrap();
    assert_eq!(operation.request_kind(), InvocationRequestKindV1::Operation);
    assert_eq!(
        operation.schema().reference().as_str(),
        "handbook.invocation-request@1.0.0"
    );
    let operation_bytes = canonical_json_bytes(&operation, &limits).unwrap();
    let admitted = InvocationRequestV1::parse_and_admit(&operation_bytes, &limits).unwrap();
    assert_eq!(admitted.request_kind(), InvocationRequestKindV1::Operation);
    assert_eq!(
        admitted.request_fingerprint(),
        operation.request_fingerprint()
    );

    let use_case = InvocationRequestV1::use_case(
        UseCaseRequestV1::repository_doctor(&limits).unwrap(),
        &limits,
    )
    .unwrap();
    assert_eq!(use_case.request_kind(), InvocationRequestKindV1::UseCase);
    let use_case_bytes = canonical_json_bytes(&use_case, &limits).unwrap();
    assert_eq!(
        InvocationRequestV1::parse_and_admit(&use_case_bytes, &limits)
            .unwrap()
            .request_kind(),
        InvocationRequestKindV1::UseCase
    );

    let mut mismatch = serde_json::from_slice::<Value>(&use_case_bytes).unwrap();
    mismatch["request_kind"] = Value::String("operation".to_owned());
    assert!(
        InvocationRequestV1::parse_and_admit(&canonical_value_bytes(&mismatch), &limits).is_err()
    );
}

#[test]
fn request_admission_rejects_ref_body_fingerprint_shape_and_canonical_mismatch() {
    let limits = ContractLimits::packet0();
    let request = UseCaseRequestV1::repository_setup(
        RepositorySetupRequestV1::new(RepositorySetupModeV1::Refresh, true, true),
        &limits,
    )
    .unwrap();
    let bytes = canonical_json_bytes(&request, &limits).unwrap();

    let mut mismatched_ref = serde_json::from_slice::<Value>(&bytes).unwrap();
    mismatched_ref["use_case"] = serde_json::to_value(identity_binding(
        "handbook.use-case.repository-doctor@1.0.0",
    ))
    .unwrap();
    assert!(
        UseCaseRequestV1::parse_and_admit(&canonical_value_bytes(&mismatched_ref), &limits)
            .is_err()
    );

    let mut unknown_ref = serde_json::from_slice::<Value>(&bytes).unwrap();
    unknown_ref["use_case"] =
        serde_json::to_value(identity_binding("handbook.use-case.unknown@1.0.0")).unwrap();
    assert!(
        UseCaseRequestV1::parse_and_admit(&canonical_value_bytes(&unknown_ref), &limits).is_err()
    );

    let mut bad_fingerprint = serde_json::from_slice::<Value>(&bytes).unwrap();
    bad_fingerprint["request_fingerprint"] = Value::String(ZERO_FINGERPRINT.to_owned());
    assert_eq!(
        UseCaseRequestV1::parse_and_admit(&canonical_value_bytes(&bad_fingerprint), &limits),
        Err(ContractError::FingerprintMismatch)
    );

    let mut unknown_member = serde_json::from_slice::<Value>(&bytes).unwrap();
    unknown_member["request"]["unexpected"] = Value::Bool(true);
    assert!(
        UseCaseRequestV1::parse_and_admit(&canonical_value_bytes(&unknown_member), &limits)
            .is_err()
    );

    let mut noncanonical = vec![b' '];
    noncanonical.extend_from_slice(&bytes);
    assert_eq!(
        UseCaseRequestV1::parse_and_admit(&noncanonical, &limits),
        Err(ContractError::NonCanonicalJson)
    );

    let wire = String::from_utf8(bytes).unwrap();
    let fingerprint = request.request_fingerprint().as_str();
    let duplicate = wire.replace(
        &format!("\"request_fingerprint\":\"{fingerprint}\""),
        &format!(
            "\"request_fingerprint\":\"{fingerprint}\",\"request_fingerprint\":\"{fingerprint}\""
        ),
    );
    assert!(serde_json::from_str::<UseCaseRequestV1>(&duplicate).is_err());
}

#[test]
fn bootstrap_and_catalog_wrappers_preserve_existing_typed_outcomes() {
    let limits = ContractLimits::packet0();
    let witness = ProductionWitness::from_current(UtcTimestamp::parse(RETENTION).unwrap()).unwrap();
    let bootstrap_request = BootstrapRequestV1::new(
        witness.descriptor().schemas().bootstrap_request().clone(),
        None,
        witness.descriptor().binding(),
        SemVer::parse("1.0.0").unwrap(),
        CatalogPageSize::new(1).unwrap(),
        &limits,
    )
    .unwrap();
    let bootstrap = match witness
        .bootstrap(
            &bootstrap_request,
            &UtcTimestamp::parse(NOW).unwrap(),
            &limits,
        )
        .unwrap()
    {
        BootstrapOutcome::Ok(response) => {
            BootstrapFacadeResponseV1::ok(*response, &limits).unwrap()
        }
        BootstrapOutcome::Refused(_) => panic!("current bootstrap was refused"),
    };
    assert_eq!(bootstrap.outcome_kind(), BootstrapFacadeOutcomeKindV1::Ok);
    assert_eq!(
        bootstrap.schema().reference().as_str(),
        "handbook.bootstrap-facade-response@1.0.0"
    );
    let bootstrap_bytes = canonical_json_bytes(&bootstrap, &limits).unwrap();
    assert_eq!(
        BootstrapFacadeResponseV1::parse_and_admit(&bootstrap_bytes, &limits)
            .unwrap()
            .response_fingerprint(),
        bootstrap.response_fingerprint()
    );
    let mut rebound_bootstrap = serde_json::from_slice::<Value>(&bootstrap_bytes).unwrap();
    rebound_bootstrap["response"]["schema"]["fingerprint"] =
        Value::String(ZERO_FINGERPRINT.to_owned());
    refresh_fingerprint(&mut rebound_bootstrap["response"], "response_fingerprint");
    refresh_fingerprint(&mut rebound_bootstrap, "response_fingerprint");
    assert!(BootstrapFacadeResponseV1::parse_and_admit(
        &canonical_value_bytes(&rebound_bootstrap),
        &limits,
    )
    .is_err());

    let request = CatalogReadRequestV1::new(
        CatalogKindV1::Schemas,
        CatalogPageSize::new(1).unwrap(),
        None,
        &limits,
    )
    .unwrap();
    assert_eq!(
        request.schema().reference().as_str(),
        "handbook.catalog-read-request@1.0.0"
    );
    let request_bytes = canonical_json_bytes(&request, &limits).unwrap();
    assert_eq!(
        CatalogReadRequestV1::parse_and_admit(&request_bytes, &limits)
            .unwrap()
            .request_fingerprint(),
        request.request_fingerprint()
    );

    let page = match witness
        .read_schemas(
            request.page_size(),
            request.cursor(),
            &UtcTimestamp::parse(NOW).unwrap(),
            &limits,
        )
        .unwrap()
    {
        ProductionCatalogRead::Page(page) => page,
        _ => panic!("schema catalog page was unavailable"),
    };
    let response = CatalogReadResponseV1::schema_page(page, &limits).unwrap();
    assert_eq!(response.catalog_kind(), CatalogKindV1::Schemas);
    assert_eq!(response.outcome_kind(), CatalogReadOutcomeKindV1::Page);
    assert_eq!(
        response.schema().reference().as_str(),
        "handbook.catalog-read-response@1.0.0"
    );
    let response_bytes = canonical_json_bytes(&response, &limits).unwrap();
    assert_eq!(
        CatalogReadResponseV1::parse_and_admit(&response_bytes, &limits)
            .unwrap()
            .response_fingerprint(),
        response.response_fingerprint()
    );

    let cursor_mismatch =
        CatalogReadResponseV1::cursor_mismatch(CatalogKindV1::Schemas, &limits).unwrap();
    assert_eq!(
        CatalogReadResponseV1::parse_and_admit(
            &canonical_json_bytes(&cursor_mismatch, &limits).unwrap(),
            &limits,
        )
        .unwrap()
        .outcome_kind(),
        CatalogReadOutcomeKindV1::CursorMismatch
    );

    let blocked = match witness
        .read_schemas(
            CatalogPageSize::new(1).unwrap(),
            None,
            &UtcTimestamp::parse("2031-01-01T00:00:00Z").unwrap(),
            &limits,
        )
        .unwrap()
    {
        ProductionCatalogRead::Blocked(blocked) => blocked,
        _ => panic!("expired catalog did not block"),
    };
    let blocked =
        CatalogReadResponseV1::blocked(CatalogKindV1::Schemas, *blocked, &limits).unwrap();
    assert_eq!(blocked.outcome_kind(), CatalogReadOutcomeKindV1::Blocked);
    CatalogReadResponseV1::parse_and_admit(
        &canonical_json_bytes(&blocked, &limits).unwrap(),
        &limits,
    )
    .unwrap();
}

#[test]
fn facade_response_discriminator_and_fingerprint_mismatches_fail_closed() {
    let limits = ContractLimits::packet0();
    let response = CatalogReadResponseV1::cursor_mismatch(CatalogKindV1::Schemas, &limits).unwrap();
    let bytes = canonical_json_bytes(&response, &limits).unwrap();

    let mut kind_mismatch = serde_json::from_slice::<Value>(&bytes).unwrap();
    kind_mismatch["catalog_kind"] = Value::String("profiles".to_owned());
    assert!(CatalogReadResponseV1::parse_and_admit(
        &canonical_value_bytes(&kind_mismatch),
        &limits
    )
    .is_err());

    let mut outcome_mismatch = serde_json::from_slice::<Value>(&bytes).unwrap();
    outcome_mismatch["outcome_kind"] = Value::String("blocked".to_owned());
    assert!(CatalogReadResponseV1::parse_and_admit(
        &canonical_value_bytes(&outcome_mismatch),
        &limits
    )
    .is_err());

    let mut fingerprint_mismatch = serde_json::from_slice::<Value>(&bytes).unwrap();
    fingerprint_mismatch["response_fingerprint"] = Value::String(ZERO_FINGERPRINT.to_owned());
    assert_eq!(
        CatalogReadResponseV1::parse_and_admit(
            &canonical_value_bytes(&fingerprint_mismatch),
            &limits
        ),
        Err(ContractError::FingerprintMismatch)
    );
}

#[cfg(feature = "contract-schema-generation")]
#[test]
fn facade_constructor_instances_validate_against_all_five_new_roots() {
    let limits = ContractLimits::packet0();
    let use_cases = vec![
        UseCaseRequestV1::repository_setup(
            RepositorySetupRequestV1::new(RepositorySetupModeV1::Auto, false, false),
            &limits,
        )
        .unwrap(),
        UseCaseRequestV1::repository_doctor(&limits).unwrap(),
        UseCaseRequestV1::author_charter(AuthorCharterRequestV1::Validate, &limits).unwrap(),
        UseCaseRequestV1::author_project_context(
            AuthorProjectContextRequestV1::Validate(
                ProjectContextInputV1::new(
                    "handbook.project-context",
                    "1.0.0",
                    "project-context.schema-proof",
                    "Schema proof",
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                )
                .unwrap(),
            ),
            &limits,
        )
        .unwrap(),
        UseCaseRequestV1::approver_administration(
            ApproverAdministrationRequestV1::revoke_credential("sha256:schema-proof").unwrap(),
            &limits,
        )
        .unwrap(),
        UseCaseRequestV1::pipeline_route_resolve_and_persist(
            PipelineRouteResolveAndPersistRequestV1::new("release").unwrap(),
            &limits,
        )
        .unwrap(),
    ];
    for request in &use_cases {
        assert_facade_schema_valid("handbook.use-case-request@1.0.0", request);
    }
    assert_facade_schema_valid(
        "handbook.invocation-request@1.0.0",
        &InvocationRequestV1::operation(
            pipeline_catalog_list_operation_request(None).unwrap(),
            &limits,
        )
        .unwrap(),
    );
    assert_facade_schema_valid(
        "handbook.invocation-request@1.0.0",
        &InvocationRequestV1::use_case(
            UseCaseRequestV1::repository_doctor(&limits).unwrap(),
            &limits,
        )
        .unwrap(),
    );

    let witness = ProductionWitness::from_current(UtcTimestamp::parse(RETENTION).unwrap()).unwrap();
    let bootstrap_request = BootstrapRequestV1::new(
        witness.descriptor().schemas().bootstrap_request().clone(),
        None,
        witness.descriptor().binding(),
        SemVer::parse("1.0.0").unwrap(),
        CatalogPageSize::new(1).unwrap(),
        &limits,
    )
    .unwrap();
    let bootstrap = match witness
        .bootstrap(
            &bootstrap_request,
            &UtcTimestamp::parse(NOW).unwrap(),
            &limits,
        )
        .unwrap()
    {
        BootstrapOutcome::Ok(response) => {
            BootstrapFacadeResponseV1::ok(*response, &limits).unwrap()
        }
        BootstrapOutcome::Refused(_) => panic!("current bootstrap was refused"),
    };
    assert_facade_schema_valid("handbook.bootstrap-facade-response@1.0.0", &bootstrap);

    let catalog_request = CatalogReadRequestV1::new(
        CatalogKindV1::Schemas,
        CatalogPageSize::new(1).unwrap(),
        None,
        &limits,
    )
    .unwrap();
    assert_facade_schema_valid("handbook.catalog-read-request@1.0.0", &catalog_request);
    let page = match witness
        .read_schemas(
            catalog_request.page_size(),
            None,
            &UtcTimestamp::parse(NOW).unwrap(),
            &limits,
        )
        .unwrap()
    {
        ProductionCatalogRead::Page(page) => page,
        _ => panic!("schema catalog page was unavailable"),
    };
    assert_facade_schema_valid(
        "handbook.catalog-read-response@1.0.0",
        &CatalogReadResponseV1::schema_page(page, &limits).unwrap(),
    );
    assert_facade_schema_valid(
        "handbook.catalog-read-response@1.0.0",
        &CatalogReadResponseV1::cursor_mismatch(CatalogKindV1::Profiles, &limits).unwrap(),
    );
}

#[cfg(feature = "contract-schema-generation")]
fn assert_facade_schema_valid(exact_ref: &str, value: &impl serde::Serialize) {
    let (identity, version) = exact_ref.split_once('@').unwrap();
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("schemas/2020-12")
        .join(identity)
        .join(format!("{version}.schema.json"));
    let schema: Value = serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();
    let validator = jsonschema::draft202012::options().build(&schema).unwrap();
    let instance: Value =
        serde_json::from_slice(&canonical_json_bytes(value, &ContractLimits::packet0()).unwrap())
            .unwrap();
    assert!(validator.is_valid(&instance), "schema rejected {exact_ref}");
}

fn identity_binding(reference: &str) -> ExactBinding {
    ExactBinding::new(
        ExactRef::parse(reference).unwrap(),
        sha256_fingerprint(reference.as_bytes()),
    )
}

fn canonical_value_bytes(value: &Value) -> Vec<u8> {
    canonical_json_bytes(value, &ContractLimits::packet0()).unwrap()
}

fn refresh_fingerprint(value: &mut Value, field: &str) {
    let mut preimage = value.clone();
    preimage.as_object_mut().unwrap().remove(field);
    value[field] = Value::String(
        sha256_fingerprint(&canonical_value_bytes(&preimage))
            .as_str()
            .to_owned(),
    );
}
