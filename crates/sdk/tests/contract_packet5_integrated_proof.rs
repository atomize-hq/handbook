#![cfg(feature = "contract-schema-generation")]

use handbook_sdk::contract_membrane::schema_generation::{run, GenerationMode};
use handbook_sdk::contract_membrane::{
    admit_operation_definition, canonical_json_bytes, operation_inventory, parse_canonical_json,
    posture_transition_apply_definition, posture_transition_apply_request, sha256_fingerprint,
    AdmissionReason, BoundedString, CatalogPageSize, ContractLimits, ContractValue, DiscoveryState,
    ExactBinding, ExactRef, OperationAdmissionResult, OperationTransport, OwnerVersionEntry,
    PostureCanonicalBindingDtoV1, PostureDimensionChangeDtoV1, PostureReferenceDtoV1,
    PostureTransitionApplyRequestDataV1, ProductionCatalogRead, ProductionWitness,
    SchemaManifestEntry, TerminalStatus, UtcTimestamp, WriteReceipt,
};
use handbook_sdk::HandbookSdkV1;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

const RAW_KEY: &str = "sdk-posture-proof-0001";
const RETENTION: &str = "2030-01-01T00:00:00Z";
const NOW: &str = "2029-01-01T00:00:00Z";

#[test]
fn regeneration_manifest_and_production_catalogs_close_over_one_exact_set() {
    run(GenerationMode::Check).expect("first regeneration must match tracked bytes");
    run(GenerationMode::Check).expect("second regeneration must be byte-identical too");

    let sdk_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let manifest_path = sdk_root.join("schemas/manifest.json");
    let manifest_bytes = fs::read(&manifest_path).unwrap();
    assert_one_terminal_lf(&manifest_bytes, &manifest_path);
    let manifest_without_lf = &manifest_bytes[..manifest_bytes.len() - 1];
    let manifest_value =
        parse_canonical_json::<Value>(manifest_without_lf, &ContractLimits::packet0()).unwrap();
    assert_eq!(
        canonical_json_bytes(&manifest_value, &ContractLimits::packet0()).unwrap(),
        manifest_without_lf
    );
    let manifest = parse_canonical_json::<Vec<SchemaManifestEntry>>(
        manifest_without_lf,
        &ContractLimits::packet0(),
    )
    .unwrap();
    assert_eq!(manifest.len(), 64);

    let mut refs = BTreeSet::new();
    let mut schema_paths = BTreeSet::new();
    for entry in &manifest {
        let exact_ref = entry.schema().reference();
        assert!(refs.insert(exact_ref.as_str()));
        let path = schema_path(sdk_root, exact_ref);
        assert!(schema_paths.insert(path.clone()));
        let bytes = fs::read(&path).unwrap();
        assert_one_terminal_lf(&bytes, &path);
        assert_eq!(entry.byte_length().get(), bytes.len() as u64);
        assert_eq!(entry.schema().fingerprint(), &sha256_fingerprint(&bytes));

        let without_lf = &bytes[..bytes.len() - 1];
        let schema = parse_canonical_json::<Value>(without_lf, &ContractLimits::packet0()).unwrap();
        assert_eq!(
            canonical_json_bytes(&schema, &ContractLimits::packet0()).unwrap(),
            without_lf
        );
        assert_eq!(
            schema["$id"],
            format!(
                "urn:handbook:schema:{}:{}",
                exact_ref.identity(),
                exact_ref.version()
            )
        );
        assert_eq!(
            schema["$schema"],
            "https://json-schema.org/draft/2020-12/schema"
        );
    }
    assert_eq!(
        schema_files(&sdk_root.join("schemas/2020-12")),
        schema_paths
    );

    let witness = current_witness();
    assert_eq!(witness.schema_manifest(), manifest);
    assert_eq!(
        witness.catalog_roots().schemas().total_entries().get(),
        manifest.len() as u64
    );
    assert_eq!(
        witness.catalog_roots().operations().total_entries().get(),
        1
    );
    assert_eq!(witness.catalog_roots().profiles().total_entries().get(), 3);
    assert_eq!(witness.capabilities().len(), 1);
    assert_eq!(
        witness.capabilities()[0].operation().reference().as_str(),
        "posture.transition.apply@1.0.0"
    );
    assert_eq!(
        witness.capabilities()[0].current_transports(),
        &[OperationTransport::RustSdk]
    );

    let mut paged_refs = Vec::new();
    let mut cursor = None;
    loop {
        let page = match witness
            .read_schemas(
                CatalogPageSize::new(7).unwrap(),
                cursor.as_ref(),
                &now(),
                &ContractLimits::packet0(),
            )
            .unwrap()
        {
            ProductionCatalogRead::Page(page) => page,
            other => panic!("current schema catalog must page without refusal: {other:?}"),
        };
        paged_refs.extend(
            page.entries()
                .iter()
                .map(|entry| entry.schema().reference().as_str().to_owned()),
        );
        cursor = page.next_cursor().cloned();
        if cursor.is_none() {
            break;
        }
    }
    assert_eq!(
        paged_refs,
        manifest
            .iter()
            .map(|entry| entry.schema().reference().as_str().to_owned())
            .collect::<Vec<_>>()
    );
}

#[test]
fn identity_domains_and_raw_key_confinement_hold_across_the_live_seam() {
    let limits = ContractLimits::packet0();
    let request_a = posture_transition_apply_request(
        Some("packet-five-correlation-a"),
        RAW_KEY,
        request_data(),
    )
    .unwrap();
    let wire = canonical_json_bytes(&request_a, &limits).unwrap();
    assert_eq!(occurrences(&wire, RAW_KEY.as_bytes()), 1);
    assert_absent(&canonical_json_bytes(request_a.body(), &limits).unwrap());
    assert!(!format!("{request_a:?}").contains(RAW_KEY));

    let request_b = posture_transition_apply_request(
        Some("packet-five-correlation-b"),
        RAW_KEY,
        request_data(),
    )
    .unwrap();
    assert_eq!(
        request_a.request_fingerprint(),
        request_b.request_fingerprint()
    );

    let repository = tempfile::tempdir().unwrap();
    let sdk = HandbookSdkV1::open(repository.path().join("missing-repository"));
    let response_a = sdk.apply_posture_transition_operation(&request_a).unwrap();
    let response_b = sdk.apply_posture_transition_operation(&request_b).unwrap();
    assert_ne!(response_a.terminal().status(), TerminalStatus::Ok);
    assert_eq!(
        response_a.result_fingerprint(),
        response_b.result_fingerprint()
    );
    assert_ne!(
        response_a.response_fingerprint(),
        response_b.response_fingerprint()
    );
    assert!(response_a.receipts().is_empty());
    assert!(response_b.receipts().is_empty());
    assert_absent(&canonical_json_bytes(&response_a, &limits).unwrap());
    assert_absent(&canonical_json_bytes(&response_b, &limits).unwrap());

    let witness = current_witness();
    let schema_page = match witness
        .read_schemas(CatalogPageSize::new(1).unwrap(), None, &now(), &limits)
        .unwrap()
    {
        ProductionCatalogRead::Page(page) => page,
        other => panic!("current schema catalog must return a page: {other:?}"),
    };
    let cursor = schema_page
        .next_cursor()
        .expect("the 64-entry catalog has a second page");
    let receipt = WriteReceipt::new(
        bounded("packet.five.record"),
        binding("handbook.packet-five-record@1.0.0", b"packet-five-record"),
        bounded("semantic.record"),
        ContractValue::null(),
        bounded("packet.five.atomic"),
        &limits,
    )
    .unwrap();

    let identity_domains = [
        (
            "schema-file",
            witness.schema_manifest()[0].schema().fingerprint().as_str(),
        ),
        (
            "canonical-instance",
            witness
                .descriptor()
                .descriptor_content_fingerprint()
                .as_str(),
        ),
        (
            "scoped-raw-key",
            request_a.idempotency_key_fingerprint().unwrap().as_str(),
        ),
        ("request", request_a.request_fingerprint().as_str()),
        ("semantic-result", response_a.result_fingerprint().as_str()),
        ("outer-response", response_a.response_fingerprint().as_str()),
        (
            "catalog-entry",
            witness.capabilities()[0].entry_fingerprint().as_str(),
        ),
        (
            "catalog-root",
            witness
                .catalog_roots()
                .schemas()
                .catalog()
                .fingerprint()
                .as_str(),
        ),
        ("catalog-page", schema_page.page_fingerprint().as_str()),
        ("catalog-cursor", cursor.cursor_fingerprint().as_str()),
        ("write-receipt", receipt.receipt_fingerprint().as_str()),
    ];
    let mut fingerprints = BTreeMap::new();
    for (domain, fingerprint) in identity_domains {
        assert!(
            fingerprints.insert(fingerprint, domain).is_none(),
            "identity domains accidentally alias: {fingerprints:?}"
        );
    }

    let public_witness = serde_json::json!({
        "capabilities": witness.capabilities(),
        "descriptor": witness.descriptor(),
        "manifest": witness.schema_manifest(),
        "owners": witness.owner_versions(),
        "profiles": witness.profiles(),
        "roots": witness.catalog_roots(),
        "schema_page": schema_page,
    });
    assert_absent(&canonical_json_bytes(&public_witness, &limits).unwrap());
    assert_absent(&canonical_json_bytes(&receipt, &limits).unwrap());
    assert_raw_key_absent_from_schema_assets();
    assert_adapter_sources_are_non_authoritative();
}

#[test]
fn owner_native_evidence_inventory_and_adapter_direction_remain_independent() {
    let witness = current_witness();
    let engine = handbook_engine::operation_admission::operation_owner_version_evidence();
    let flow = handbook_flow::operation_admission::operation_owner_version_evidence();
    let pipeline = handbook_pipeline::operation_admission::operation_owner_version_evidence();
    let expected = [
        (
            engine.owner_crate,
            engine.owner_version,
            engine.owner_source_ref,
            engine.source_bytes,
        ),
        (
            flow.owner_crate,
            flow.owner_version,
            flow.owner_source_ref,
            flow.source_bytes,
        ),
        (
            pipeline.owner_crate,
            pipeline.owner_version,
            pipeline.owner_source_ref,
            pipeline.source_bytes,
        ),
    ];
    assert_eq!(witness.owner_versions().len(), expected.len());
    for (entry, evidence) in witness.owner_versions().iter().zip(expected) {
        assert_owner_entry(entry, evidence);
    }
    assert_eq!(
        witness
            .profiles()
            .iter()
            .map(|entry| entry.profile().reference().as_str())
            .collect::<Vec<_>>(),
        vec![
            "handbook.profile.shipped-root@1.0.0",
            "handbook.profile.shipped-root@1.1.0",
            "handbook.profile.shipped-root@1.2.0",
        ]
    );

    let rows = operation_inventory();
    assert_eq!(rows.len(), 62);
    assert_eq!(
        rows.iter()
            .map(|row| row.operation_ref().as_str())
            .collect::<BTreeSet<_>>()
            .len(),
        62
    );
    assert!(rows
        .iter()
        .enumerate()
        .all(|(index, row)| row.proof_ordinal() == (index + 1) as u16));
    let admitted = rows
        .iter()
        .filter(|row| row.admission_reason() == AdmissionReason::Admitted)
        .collect::<Vec<_>>();
    assert_eq!(admitted.len(), 1);
    assert_eq!(
        admitted[0].operation_ref().as_str(),
        "posture.transition.apply@1.0.0"
    );
    assert_eq!(
        admitted[0].current_transports(),
        &[OperationTransport::RustSdk]
    );
    assert_eq!(admitted[0].discovery(), DiscoveryState::Discoverable);
    assert!(rows.iter().filter(|row| row != &admitted[0]).all(|row| {
        row.current_transports().is_empty() && row.discovery() == DiscoveryState::Omit
    }));

    assert!(matches!(
        admit_operation_definition(&posture_transition_apply_definition().unwrap()).unwrap(),
        OperationAdmissionResult::Admitted(_)
    ));
    assert_eq!(witness.capabilities().len(), admitted.len());
    assert_adapter_sources_are_non_authoritative();
}

fn current_witness() -> ProductionWitness {
    ProductionWitness::from_current(UtcTimestamp::parse(RETENTION).unwrap()).unwrap()
}

fn now() -> UtcTimestamp {
    UtcTimestamp::parse(NOW).unwrap()
}

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

fn schema_path(sdk_root: &Path, exact_ref: &ExactRef) -> PathBuf {
    sdk_root
        .join("schemas/2020-12")
        .join(exact_ref.identity())
        .join(format!("{}.schema.json", exact_ref.version()))
}

fn schema_files(root: &Path) -> BTreeSet<PathBuf> {
    fn visit(current: &Path, files: &mut BTreeSet<PathBuf>) {
        for entry in fs::read_dir(current).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                visit(&path, files);
            } else if path.is_file() {
                assert!(files.insert(path));
            }
        }
    }
    let mut files = BTreeSet::new();
    visit(root, &mut files);
    files
}

fn assert_one_terminal_lf(bytes: &[u8], path: &Path) {
    assert!(
        bytes.ends_with(b"\n"),
        "{} has no terminal LF",
        path.display()
    );
    assert!(
        !bytes.ends_with(b"\n\n") && !bytes[..bytes.len() - 1].contains(&b'\r'),
        "{} does not have exactly one LF framing byte",
        path.display()
    );
}

fn bounded(value: &str) -> BoundedString<255> {
    BoundedString::new(value.to_owned()).unwrap()
}

fn binding(reference: &str, bytes: &[u8]) -> ExactBinding {
    ExactBinding::new(
        ExactRef::parse(reference).unwrap(),
        sha256_fingerprint(bytes),
    )
}

fn occurrences(haystack: &[u8], needle: &[u8]) -> usize {
    haystack
        .windows(needle.len())
        .filter(|window| *window == needle)
        .count()
}

fn assert_absent(bytes: &[u8]) {
    assert_eq!(occurrences(bytes, RAW_KEY.as_bytes()), 0);
}

fn assert_raw_key_absent_from_schema_assets() {
    let schema_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("schemas");
    for path in schema_files(&schema_root) {
        assert_absent(&fs::read(path).unwrap());
    }
}

fn assert_owner_entry(entry: &OwnerVersionEntry, evidence: (&str, &str, &str, &[u8])) {
    assert_eq!(entry.owner_crate(), evidence.0);
    assert_eq!(entry.owner_version().as_str(), evidence.1);
    assert_eq!(entry.source().reference().as_str(), evidence.2);
    assert_eq!(
        entry.source().fingerprint(),
        &sha256_fingerprint(evidence.3)
    );
}

fn assert_adapter_sources_are_non_authoritative() {
    let workspace_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .unwrap();
    let cli_source = read_rust_sources(&workspace_root.join("crates/cli/src"));
    for prohibited in [
        "apply_posture_transition_operation",
        "posture.transition.apply",
        "operation_inventory",
        "ProductionWitness",
        "BootstrapDescriptorV1",
        "CapabilityEntry",
        "tauri::",
        "#[tauri::command]",
        "generate_handler!",
        "invoke_handler",
    ] {
        assert!(
            !cli_source.contains(prohibited),
            "active CLI source contains membrane or Tauri authority token {prohibited}"
        );
    }
    assert!(!cli_source.contains(RAW_KEY));

    let workspace_manifest = fs::read_to_string(workspace_root.join("Cargo.toml")).unwrap();
    let cli_manifest = fs::read_to_string(workspace_root.join("crates/cli/Cargo.toml")).unwrap();
    let sdk_manifest = fs::read_to_string(workspace_root.join("crates/sdk/Cargo.toml")).unwrap();
    assert!(cli_manifest.contains("handbook-sdk = { path = \"../sdk\" }"));
    assert!(!sdk_manifest.contains("handbook-cli"));
    assert!(!workspace_manifest.to_ascii_lowercase().contains("tauri"));
    assert!(!workspace_root.join("crates/tauri").exists());
}

fn read_rust_sources(root: &Path) -> String {
    fn visit(current: &Path, output: &mut String) {
        for entry in fs::read_dir(current).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                visit(&path, output);
            } else if path.extension().and_then(|value| value.to_str()) == Some("rs") {
                output.push_str(&fs::read_to_string(path).unwrap());
            }
        }
    }
    let mut output = String::new();
    visit(root, &mut output);
    output
}
