use handbook_sdk::contract_membrane::{
    canonical_json_bytes, parse_canonical_json, sha256_fingerprint, BootstrapCeilings,
    BootstrapDescriptorV1, BootstrapRefusalV1, BootstrapRequestV1, BootstrapResponseV1,
    BootstrapSchemaBindings, BoundedString, BoundedVec, CapabilityEntry, CatalogPageSize,
    CatalogReadError, CatalogRoots, ContractError, ContractLimits, DraftVersion, ExactBinding,
    ExactRef, ImmutableCatalog, OperationTransport, OwnerVersionEntry, ProfileCatalogEntry,
    RequestId, SafeU64, SchemaManifestEntry, SchemaMediaType, SemVer, TerminalStatus,
    TypedCatalogCursor, TypedCatalogPage, UtcTimestamp,
};

fn binding(reference: &str, seed: &str) -> ExactBinding {
    ExactBinding::new(
        ExactRef::parse(reference).unwrap(),
        sha256_fingerprint(seed.as_bytes()),
    )
}

fn schema_entry(reference: &str, seed: &str) -> SchemaManifestEntry {
    SchemaManifestEntry::new(
        binding(reference, seed),
        DraftVersion::Draft202012,
        SafeU64::new(1).unwrap(),
        SchemaMediaType::ApplicationSchemaJson,
    )
    .unwrap()
}

fn required_bootstrap_manifest() -> Vec<SchemaManifestEntry> {
    let mut entries = [
        "handbook.bootstrap-descriptor-schema@1.0.0",
        "handbook.bootstrap-request@1.0.0",
        "handbook.bootstrap-response@1.0.0",
        "handbook.bootstrap-refusal@1.0.0",
        "handbook.bootstrap-refusal-details@1.0.0",
        "handbook.capability-entry@1.0.0",
        "handbook.owner-version-entry@1.0.0",
        "handbook.profile-catalog-entry@1.0.0",
        "handbook.catalog-roots@1.0.0",
        "handbook.catalog-root@1.0.0",
        "handbook.typed-catalog-cursor@1.0.0",
        "handbook.capability-catalog-page@1.0.0",
        "handbook.schema-catalog-page@1.0.0",
        "handbook.profile-catalog-page@1.0.0",
        "handbook.catalog-read-blocked@1.0.0",
        "handbook.operation-definition@1.0.0",
        "handbook.operation-request@1.0.0",
        "handbook.operation-response@1.0.0",
        "handbook.operation-admission-refusal@1.0.0",
    ]
    .into_iter()
    .map(|reference| schema_entry(reference, reference))
    .collect::<Vec<_>>();
    entries.sort_by(|left, right| {
        left.schema()
            .reference()
            .as_str()
            .cmp(right.schema().reference().as_str())
    });
    entries
}

fn catalog(
    entries: Vec<SchemaManifestEntry>,
    retention: &str,
) -> ImmutableCatalog<SchemaManifestEntry> {
    ImmutableCatalog::new(
        binding("handbook.schema-catalog-page@1.0.0", "page-schema"),
        binding("handbook.typed-catalog-cursor@1.0.0", "cursor-schema"),
        binding("handbook.schema-manifest-entry@1.0.0", "entry-schema"),
        entries,
        UtcTimestamp::parse(retention).unwrap(),
        &ContractLimits::packet0(),
    )
    .unwrap()
}

#[test]
fn typed_catalog_pages_are_sorted_contiguous_and_catalog_bound() {
    let limits = ContractLimits::packet0();
    let snapshot = catalog(
        vec![
            schema_entry("z.schema@1.0.0", "z"),
            schema_entry("a.schema@1.0.0", "a"),
            schema_entry("m.schema@1.0.0", "m"),
        ],
        "2030-01-01T00:00:00Z",
    );
    let page_size = CatalogPageSize::new(2).unwrap();
    let first = snapshot
        .page(
            page_size,
            None,
            &UtcTimestamp::parse("2029-01-01T00:00:00Z").unwrap(),
            &limits,
        )
        .unwrap();

    assert_eq!(first.start_index().get(), 0);
    assert_eq!(first.entries().len(), 2);
    assert_eq!(
        first.entries()[0].schema().reference().as_str(),
        "a.schema@1.0.0"
    );
    assert_eq!(
        first.entries()[1].schema().reference().as_str(),
        "m.schema@1.0.0"
    );
    let cursor = first.next_cursor().unwrap();
    assert_eq!(cursor.next_index().get(), 2);
    assert_eq!(cursor.next_sort_key().as_str(), "z.schema@1.0.0");

    let second = snapshot
        .page(
            page_size,
            Some(cursor),
            &UtcTimestamp::parse("2029-01-01T00:00:00Z").unwrap(),
            &limits,
        )
        .unwrap();
    assert_eq!(second.start_index().get(), 2);
    assert_eq!(second.entries().len(), 1);
    assert!(second.next_cursor().is_none());
    assert_eq!(second.root(), first.root());

    let wire = canonical_json_bytes(&first, &limits).unwrap();
    let parsed =
        parse_canonical_json::<TypedCatalogPage<SchemaManifestEntry>>(&wire, &limits).unwrap();
    assert_eq!(parsed, first);
}

#[test]
fn cursor_mismatch_expiry_concurrent_change_and_ceiling_fail_closed() {
    let limits = ContractLimits::packet0();
    assert!(CatalogPageSize::new(128).is_ok());
    assert_eq!(CatalogPageSize::new(129), Err(ContractError::BoundExceeded));
    assert_eq!(CatalogPageSize::new(0), Err(ContractError::Empty));

    let old = catalog(
        vec![
            schema_entry("a.schema@1.0.0", "a"),
            schema_entry("b.schema@1.0.0", "b"),
        ],
        "2030-01-01T00:00:00Z",
    );
    let first = old
        .page(
            CatalogPageSize::new(1).unwrap(),
            None,
            &UtcTimestamp::parse("2029-01-01T00:00:00Z").unwrap(),
            &limits,
        )
        .unwrap();
    let cursor = first.next_cursor().unwrap().clone();

    assert_eq!(
        old.page(
            CatalogPageSize::new(2).unwrap(),
            Some(&cursor),
            &UtcTimestamp::parse("2029-01-01T00:00:00Z").unwrap(),
            &limits,
        ),
        Err(CatalogReadError::CursorMismatch)
    );
    assert!(matches!(
        old.page(
            CatalogPageSize::new(1).unwrap(),
            Some(&cursor),
            &UtcTimestamp::parse("2031-01-01T00:00:00Z").unwrap(),
            &limits,
        ),
        Err(CatalogReadError::Expired { .. })
    ));

    let changed = catalog(
        vec![
            schema_entry("a.schema@1.0.0", "a"),
            schema_entry("b.schema@1.0.0", "b"),
            schema_entry("c.schema@1.0.0", "c"),
        ],
        "2030-01-01T00:00:00Z",
    );
    assert_ne!(old.root().catalog(), changed.root().catalog());
    assert_eq!(
        changed.page(
            CatalogPageSize::new(1).unwrap(),
            Some(&cursor),
            &UtcTimestamp::parse("2029-01-01T00:00:00Z").unwrap(),
            &limits,
        ),
        Err(CatalogReadError::CursorMismatch)
    );

    let old_second = old
        .page(
            CatalogPageSize::new(1).unwrap(),
            Some(&cursor),
            &UtcTimestamp::parse("2029-01-01T00:00:00Z").unwrap(),
            &limits,
        )
        .unwrap();
    assert_eq!(
        old_second.entries()[0].schema().reference().as_str(),
        "b.schema@1.0.0"
    );
}

#[test]
fn typed_cursor_and_page_reject_tampering_and_duplicate_catalog_keys() {
    let limits = ContractLimits::packet0();
    let duplicate = ImmutableCatalog::new(
        binding("handbook.schema-catalog-page@1.0.0", "page-schema"),
        binding("handbook.typed-catalog-cursor@1.0.0", "cursor-schema"),
        binding("handbook.schema-manifest-entry@1.0.0", "entry-schema"),
        vec![
            schema_entry("a.schema@1.0.0", "first"),
            schema_entry("a.schema@1.0.0", "second"),
        ],
        UtcTimestamp::parse("2030-01-01T00:00:00Z").unwrap(),
        &limits,
    );
    assert_eq!(duplicate, Err(ContractError::DuplicateMember));

    let snapshot = catalog(
        vec![
            schema_entry("a.schema@1.0.0", "a"),
            schema_entry("b.schema@1.0.0", "b"),
        ],
        "2030-01-01T00:00:00Z",
    );
    let page = snapshot
        .page(
            CatalogPageSize::new(1).unwrap(),
            None,
            &UtcTimestamp::parse("2029-01-01T00:00:00Z").unwrap(),
            &limits,
        )
        .unwrap();
    let cursor = page.next_cursor().unwrap();
    assert_eq!(
        TypedCatalogPage::new(
            binding("handbook.schema-catalog-page@1.0.0", "page-schema"),
            SafeU64::new(1).unwrap(),
            page.root().clone(),
            CatalogPageSize::new(1).unwrap(),
            SafeU64::new(1).unwrap(),
            BoundedVec::new(Vec::<SchemaManifestEntry>::new()).unwrap(),
            Some(cursor.clone()),
            &limits,
        ),
        Err(ContractError::InvalidInvariant)
    );
    let cursor_wire = String::from_utf8(canonical_json_bytes(cursor, &limits).unwrap()).unwrap();
    let tampered = cursor_wire.replacen("\"page_size\":1", "\"page_size\":2", 1);
    assert_eq!(
        parse_canonical_json::<TypedCatalogCursor>(tampered.as_bytes(), &limits),
        Err(ContractError::FingerprintMismatch)
    );

    let page_wire = String::from_utf8(canonical_json_bytes(&page, &limits).unwrap()).unwrap();
    let unknown = format!(
        "{},\"unknown\":null}}",
        page_wire.strip_suffix('}').unwrap()
    );
    assert_eq!(
        parse_canonical_json::<TypedCatalogPage<SchemaManifestEntry>>(unknown.as_bytes(), &limits,),
        Err(ContractError::InvalidJson)
    );
}

#[test]
fn descriptor_identity_ceiling_and_exact_api_negotiation_are_closed() {
    let limits = ContractLimits::packet0();
    let manifest = required_bootstrap_manifest();
    let schemas = BootstrapSchemaBindings::from_manifest(&manifest).unwrap();
    let descriptor = BootstrapDescriptorV1::new(
        schemas,
        BoundedVec::new(vec![
            SemVer::parse("1.0.0").unwrap(),
            SemVer::parse("1.1.0").unwrap(),
        ])
        .unwrap(),
        &limits,
    )
    .unwrap();

    assert_eq!(descriptor.api_major().get(), 1);
    assert_eq!(
        descriptor.declared_transports(),
        [
            OperationTransport::CliJson,
            OperationTransport::RustSdk,
            OperationTransport::Tauri,
        ]
    );
    assert_ne!(
        descriptor.binding().fingerprint(),
        descriptor.schema().fingerprint()
    );
    assert_eq!(descriptor.ceilings(), &BootstrapCeilings::packet3());
    assert!(descriptor
        .ceilings()
        .validate_bootstrap_request_bytes(1_048_576)
        .is_ok());
    assert_eq!(
        descriptor
            .ceilings()
            .validate_bootstrap_request_bytes(1_048_577),
        Err(ContractError::BoundExceeded)
    );

    let accepted = BootstrapRequestV1::new(
        descriptor.schemas().bootstrap_request().clone(),
        Some(RequestId::parse("request-1").unwrap()),
        descriptor.binding(),
        SemVer::parse("1.1.0").unwrap(),
        CatalogPageSize::new(128).unwrap(),
        &limits,
    )
    .unwrap();
    descriptor.negotiate(&accepted).unwrap();

    let unsupported_minor = BootstrapRequestV1::new(
        descriptor.schemas().bootstrap_request().clone(),
        None,
        descriptor.binding(),
        SemVer::parse("1.2.0").unwrap(),
        CatalogPageSize::new(1).unwrap(),
        &limits,
    )
    .unwrap();
    assert_eq!(
        descriptor.negotiate(&unsupported_minor),
        Err(handbook_sdk::contract_membrane::BootstrapRefusalReason::UnsupportedApiVersion)
    );

    let unsupported_major = BootstrapRequestV1::new(
        descriptor.schemas().bootstrap_request().clone(),
        None,
        descriptor.binding(),
        SemVer::parse("2.0.0").unwrap(),
        CatalogPageSize::new(1).unwrap(),
        &limits,
    )
    .unwrap();
    assert_eq!(
        descriptor.negotiate(&unsupported_major),
        Err(handbook_sdk::contract_membrane::BootstrapRefusalReason::UnsupportedApiMajor)
    );

    let wire = canonical_json_bytes(&descriptor, &limits).unwrap();
    assert_eq!(
        parse_canonical_json::<BootstrapDescriptorV1>(&wire, &limits).unwrap(),
        descriptor
    );
    let tampered = String::from_utf8(wire).unwrap().replacen(
        "\"catalog_page_size_max\":128",
        "\"catalog_page_size_max\":127",
        1,
    );
    assert_eq!(
        parse_canonical_json::<BootstrapDescriptorV1>(tampered.as_bytes(), &limits),
        Err(ContractError::InvalidInvariant)
    );
}

#[test]
fn owner_and_profile_entries_are_exact_sorted_projections() {
    let limits = ContractLimits::packet0();
    let owner = OwnerVersionEntry::new(
        BoundedString::new("handbook-engine".to_owned()).unwrap(),
        SemVer::parse("0.2.0").unwrap(),
        binding("handbook.owner.engine@0.2.0", "engine-source"),
        &limits,
    )
    .unwrap();
    let profile = ProfileCatalogEntry::new(
        binding("handbook.profile.shipped-root@1.2.0", "profile-content"),
        binding(
            "handbook.profile-source.shipped-root@1.2.0",
            "profile-source",
        ),
        &limits,
    )
    .unwrap();
    assert_eq!(owner.owner_crate(), "handbook-engine");
    assert_eq!(
        profile.profile().reference().as_str(),
        "handbook.profile.shipped-root@1.2.0"
    );
    for wire in [
        canonical_json_bytes(&owner, &limits).unwrap(),
        canonical_json_bytes(&profile, &limits).unwrap(),
    ] {
        assert!(!String::from_utf8(wire).unwrap().contains("idempotency_key"));
    }
}

#[test]
fn bootstrap_success_and_refusal_are_closed_fingerprinted_terminal_shapes() {
    let limits = ContractLimits::packet0();
    let manifest = required_bootstrap_manifest();
    let schemas = BootstrapSchemaBindings::from_manifest(&manifest).unwrap();
    let descriptor = BootstrapDescriptorV1::new(
        schemas,
        BoundedVec::new(vec![SemVer::parse("1.0.0").unwrap()]).unwrap(),
        &limits,
    )
    .unwrap();
    let request = BootstrapRequestV1::new(
        descriptor.schemas().bootstrap_request().clone(),
        Some(RequestId::parse("request-bootstrap").unwrap()),
        descriptor.binding(),
        SemVer::parse("1.0.0").unwrap(),
        CatalogPageSize::new(128).unwrap(),
        &limits,
    )
    .unwrap();
    let until = UtcTimestamp::parse("2030-01-01T00:00:00Z").unwrap();
    let capabilities = ImmutableCatalog::<CapabilityEntry>::new(
        descriptor.schemas().capability_catalog_page().clone(),
        descriptor.schemas().typed_catalog_cursor().clone(),
        descriptor.schemas().capability_entry().clone(),
        Vec::new(),
        until.clone(),
        &limits,
    )
    .unwrap();
    let schema_catalog = ImmutableCatalog::new(
        descriptor.schemas().schema_catalog_page().clone(),
        descriptor.schemas().typed_catalog_cursor().clone(),
        binding("handbook.schema-manifest-entry@1.0.0", "manifest-entry"),
        manifest.clone(),
        until.clone(),
        &limits,
    )
    .unwrap();
    let profile_catalog = ImmutableCatalog::<ProfileCatalogEntry>::new(
        descriptor.schemas().profile_catalog_page().clone(),
        descriptor.schemas().typed_catalog_cursor().clone(),
        descriptor.schemas().profile_catalog_entry().clone(),
        Vec::new(),
        until,
        &limits,
    )
    .unwrap();
    let roots = CatalogRoots::new(
        descriptor.schemas().catalog_roots().clone(),
        capabilities.root().clone(),
        schema_catalog.root().clone(),
        profile_catalog.root().clone(),
        &limits,
    )
    .unwrap();
    let first_capabilities = capabilities
        .page(
            request.page_size(),
            None,
            &UtcTimestamp::parse("2029-01-01T00:00:00Z").unwrap(),
            &limits,
        )
        .unwrap();
    let response = BootstrapResponseV1::new(
        descriptor.schemas().bootstrap_response().clone(),
        &request,
        descriptor.clone(),
        roots,
        first_capabilities,
        BoundedVec::new(Vec::new()).unwrap(),
        BoundedVec::new(manifest.clone()).unwrap(),
        &limits,
    )
    .unwrap();
    assert_eq!(response.status(), TerminalStatus::Ok);
    assert_eq!(
        response.catalog_roots().operations().total_entries().get(),
        0
    );
    let response_wire = canonical_json_bytes(&response, &limits).unwrap();
    assert_eq!(
        parse_canonical_json::<BootstrapResponseV1>(&response_wire, &limits).unwrap(),
        response
    );

    let refusal = BootstrapRefusalV1::new(
        descriptor.schemas().bootstrap_refusal().clone(),
        &request,
        Some(descriptor.binding()),
        handbook_sdk::contract_membrane::BootstrapRefusalReason::UnsupportedApiVersion,
        BoundedVec::new(descriptor.supported_api_versions().to_vec()).unwrap(),
        BoundedVec::new(manifest).unwrap(),
        &limits,
    )
    .unwrap();
    assert_eq!(refusal.status(), TerminalStatus::Refused);
    assert!(refusal.data().is_none());
    assert_eq!(refusal.refusals().len(), 1);
    let refusal_wire = String::from_utf8(canonical_json_bytes(&refusal, &limits).unwrap()).unwrap();
    assert!(refusal_wire.contains("\"data\":null"));
    assert_eq!(
        parse_canonical_json::<BootstrapRefusalV1>(refusal_wire.as_bytes(), &limits).unwrap(),
        refusal
    );
    let missing_null = refusal_wire.replace("\"data\":null,", "");
    assert_eq!(
        parse_canonical_json::<BootstrapRefusalV1>(missing_null.as_bytes(), &limits),
        Err(ContractError::InvalidJson)
    );
}
