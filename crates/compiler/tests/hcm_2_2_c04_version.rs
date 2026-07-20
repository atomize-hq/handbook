use handbook_compiler::{build_output_model, resolve, RenderError};
use handbook_flow::ResolveRequest;

#[test]
fn compiler_accepts_only_the_hcm_2_2_c04_result_version() {
    let dir = tempfile::tempdir().expect("tempdir");
    let mut result = resolve(dir.path(), ResolveRequest::default()).expect("resolver result");
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
