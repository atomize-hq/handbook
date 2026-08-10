use crate::flow_rendering::{build_output_model, RenderError};
use handbook_sdk::flow_api::GeneratePacketRequest;
use handbook_sdk::HandbookSdkV1;

#[test]
fn compiler_accepts_only_the_hcm_2_2_c04_result_version() {
    let dir = tempfile::tempdir().expect("tempdir");
    let mut result = HandbookSdkV1::open(dir.path())
        .generate_packet(GeneratePacketRequest::new("planning.packet"))
        .expect("resolver result")
        .into_resolution();
    assert_eq!(result.c04_result_version, "reduced-v1-m8.3");
    build_output_model(&result).expect("C04 m8.3 accepted");

    result.c04_result_version = "reduced-v1-m8.2".to_owned();
    assert_eq!(
        build_output_model(&result),
        Err(RenderError::UnsupportedResultVersion {
            expected: "reduced-v1-m8.3",
            actual: "reduced-v1-m8.2".to_owned(),
        })
    );
}
