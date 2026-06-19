use handbook_pipeline::{
    handbook_product_pipeline_storage_layout_contract, PipelineStorageLayoutContract,
};

#[test]
fn public_storage_layout_contract_boundary_supports_default_and_custom_contracts() {
    let default_contract = *handbook_product_pipeline_storage_layout_contract();
    let expected_default = PipelineStorageLayoutContract::from_paths(
        ".handbook/state",
        ".handbook/state/pipeline",
        ".handbook/state/pipeline/stage_capture",
        ".handbook/state/pipeline/capture",
        "artifacts/handoff/feature_slice",
    );

    assert_eq!(default_contract, expected_default);

    let custom_contract = PipelineStorageLayoutContract::from_paths(
        ".substrate/handbook/state",
        ".substrate/handbook/state/pipeline",
        ".substrate/handbook/state/pipeline/stage_capture",
        ".substrate/handbook/state/pipeline/capture",
        ".substrate/handbook/artifacts/handoff/feature_slice",
    );

    assert_ne!(custom_contract, default_contract);
}
