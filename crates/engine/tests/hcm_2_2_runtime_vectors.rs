use handbook_engine::validate_runtime_record_fingerprint_vectors;

const VECTORS: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/specs/handbook-contract-membrane/slices/HCM-2.2/contracts/runtime-record-fingerprint-vectors-v1.0.json"
));

#[test]
fn every_runtime_record_identity_and_binding_vector_recomputes_exactly() {
    let report = validate_runtime_record_fingerprint_vectors(VECTORS)
        .expect("closed HCM-2.2 runtime identity fixture");
    assert_eq!(report.vector_count, 22);
    assert_eq!(report.cross_record_binding_count, 31);
    assert_eq!(report.external_reference_binding_count, 17);
}
