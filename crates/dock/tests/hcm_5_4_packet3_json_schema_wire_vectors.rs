use handbook_dock::{DockProcessRequestV1, DockProcessResultStatusV1, DockProcessResultV1};

const REQUEST_BYTES: &[u8] =
    include_bytes!("../../dock-json-schema/tests/vectors/process-request-v1.json");
const RESULT_BYTES: &[u8] =
    include_bytes!("../../dock-json-schema/tests/vectors/process-result-v1.json");

#[test]
fn native_json_schema_validator_vectors_are_byte_compatible_without_a_crate_edge() {
    let request_body = REQUEST_BYTES.strip_suffix(b"\n").unwrap();
    let request: DockProcessRequestV1 = serde_json::from_slice(request_body).unwrap();
    assert_eq!(request.canonical_json_bytes().unwrap(), request_body);

    let mut result_frame = RESULT_BYTES.to_vec();
    let result = DockProcessResultV1::admit_framed_json(&result_frame, &request).unwrap();
    assert_eq!(result.status(), DockProcessResultStatusV1::Completed);
    assert_eq!(result.candidates().len(), 1);
    assert_eq!(result.framed_json_bytes().unwrap(), result_frame);

    result_frame.extend_from_slice(b"extra");
    assert!(DockProcessResultV1::admit_framed_json(&result_frame, &request).is_err());
}
