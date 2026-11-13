use crate::project_posture::{
    bind_exact_canonical_bytes, derive_project_posture_kernel, fixed_dimension_binding,
    prepare_posture_change, replay_project_posture_kernel, verify_one_leaf_deep_diff,
    verify_precommit_cas, PostureChangeRequest, PostureRefusal,
};
use crate::CanonicalCharter;

const CANONICAL_CHARTER: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/specs/handbook-contract-membrane/slices/HCM-2.2/contracts/canonical-charter-boundary-v1.1.yaml"
));

fn charter() -> CanonicalCharter {
    serde_yaml_bw::from_slice(CANONICAL_CHARTER).expect("test Charter must decode")
}

fn request_for(
    charter: &CanonicalCharter,
    dimension_id: &str,
    proposed_effective_level: u8,
) -> PostureChangeRequest {
    let binding = fixed_dimension_binding(dimension_id).expect("registered dimension");
    let stored = charter.engineering_posture.dimensions[binding.index].level_override;
    PostureChangeRequest {
        dimension_id: dimension_id.to_owned(),
        dimension_index: binding.index as u8,
        authority_path: binding.authority_path.to_owned(),
        expected_stored_value: stored,
        expected_effective_level: stored.unwrap_or(charter.posture.baseline_level),
        proposed_effective_level,
    }
}

#[test]
fn fixed_dimension_map_is_complete_ordered_and_uses_only_level_override_leaves() {
    let expected = [
        (0, "speed_vs_quality"),
        (1, "type_safety_static_analysis"),
        (2, "testing_rigor"),
        (3, "scalability_performance"),
        (4, "reliability_operability"),
        (5, "security_privacy"),
        (6, "observability"),
        (7, "dx_tooling_automation"),
        (8, "ux_polish_api_usability"),
    ];

    for (index, dimension_id) in expected {
        let binding = fixed_dimension_binding(dimension_id).expect("registered dimension");
        assert_eq!(binding.index, index);
        assert_eq!(
            binding.authority_path,
            format!("/engineering_posture/dimensions/{index}/level_override")
        );
    }
    assert!(fixed_dimension_binding("context_resolution").is_none());
}

#[test]
fn kernel_derivation_uses_baseline_when_override_is_null_and_replays_exactly() {
    let source = charter();
    let canonical = bind_exact_canonical_bytes(CANONICAL_CHARTER).unwrap();

    let kernel = derive_project_posture_kernel(&source, &canonical).unwrap();
    assert_eq!(kernel.dimensions[0].effective_level, 3);
    assert_eq!(kernel.dimensions[2].effective_level, 4);

    let replayed = replay_project_posture_kernel(&kernel.replay_plan, &source, &canonical).unwrap();
    assert_eq!(replayed, kernel);
}

#[test]
fn preparation_encodes_baseline_equal_proposal_as_null_and_changes_one_leaf() {
    let source = charter();
    let canonical = bind_exact_canonical_bytes(CANONICAL_CHARTER).unwrap();
    let request = request_for(&source, "testing_rigor", 3);

    let prepared = prepare_posture_change(&source, canonical, request).unwrap();

    assert_eq!(prepared.change.dimension_index, 2);
    assert_eq!(prepared.change.expected_stored_value, Some(4));
    assert_eq!(prepared.change.expected_effective_level, 4);
    assert_eq!(prepared.change.proposed_stored_value, None);
    assert_eq!(prepared.change.proposed_effective_level, 3);
    assert_eq!(
        prepared.resulting_charter.engineering_posture.dimensions[2].level_override,
        None
    );
}

#[test]
fn preparation_refuses_noop_malformed_or_stale_requests_without_a_resulting_charter() {
    let source = charter();
    let canonical = bind_exact_canonical_bytes(CANONICAL_CHARTER).unwrap();
    let baseline = source.posture.baseline_level;

    let no_op = prepare_posture_change(
        &source,
        canonical.clone(),
        request_for(
            &source,
            "testing_rigor",
            source.engineering_posture.dimensions[2]
                .level_override
                .unwrap_or(baseline),
        ),
    );
    assert_eq!(no_op.unwrap_err(), PostureRefusal::SameEffectiveLevel);

    let mut wrong_index = request_for(&source, "testing_rigor", 5);
    wrong_index.dimension_index = 3;
    assert_eq!(
        prepare_posture_change(&source, canonical.clone(), wrong_index).unwrap_err(),
        PostureRefusal::DimensionIndexMismatch
    );

    let mut keyed_path = request_for(&source, "testing_rigor", 5);
    keyed_path.authority_path =
        "/engineering_posture/dimensions/testing_rigor/level_override".to_owned();
    assert_eq!(
        prepare_posture_change(&source, canonical.clone(), keyed_path).unwrap_err(),
        PostureRefusal::AuthorityPathMismatch
    );

    let mut stale_stored_value = request_for(&source, "testing_rigor", 5);
    stale_stored_value.expected_stored_value = Some(1);
    assert_eq!(
        prepare_posture_change(&source, canonical, stale_stored_value).unwrap_err(),
        PostureRefusal::ExpectedStoredValueMismatch
    );
}

#[test]
fn deep_diff_refuses_baseline_and_multi_leaf_changes() {
    let source = charter();
    let canonical = bind_exact_canonical_bytes(CANONICAL_CHARTER).unwrap();
    let prepared =
        prepare_posture_change(&source, canonical, request_for(&source, "testing_rigor", 5))
            .unwrap();

    let binding = fixed_dimension_binding("testing_rigor").unwrap();
    assert!(verify_one_leaf_deep_diff(
        &source,
        &prepared.resulting_charter,
        binding,
        prepared.change.expected_stored_value,
        prepared.change.proposed_stored_value,
    )
    .is_ok());

    let mut baseline_change = prepared.resulting_charter.clone();
    baseline_change.posture.baseline_level = 2;
    assert_eq!(
        verify_one_leaf_deep_diff(
            &source,
            &baseline_change,
            binding,
            prepared.change.expected_stored_value,
            prepared.change.proposed_stored_value,
        ),
        Err(PostureRefusal::DeepDiffMismatch)
    );

    let mut multi_leaf_change = prepared.resulting_charter.clone();
    multi_leaf_change.engineering_posture.dimensions[3].level_override = Some(5);
    assert_eq!(
        verify_one_leaf_deep_diff(
            &source,
            &multi_leaf_change,
            binding,
            prepared.change.expected_stored_value,
            prepared.change.proposed_stored_value,
        ),
        Err(PostureRefusal::DeepDiffMismatch)
    );
}

#[test]
fn precommit_cas_requires_exact_retained_bytes_not_only_length_or_fingerprint() {
    let source = charter();
    let canonical = bind_exact_canonical_bytes(CANONICAL_CHARTER).unwrap();
    let prepared =
        prepare_posture_change(&source, canonical, request_for(&source, "testing_rigor", 5))
            .unwrap();

    assert!(verify_precommit_cas(&prepared, CANONICAL_CHARTER).is_ok());

    let mut changed_live_bytes = CANONICAL_CHARTER.to_vec();
    changed_live_bytes[0] ^= 1;
    assert_eq!(
        verify_precommit_cas(&prepared, &changed_live_bytes),
        Err(PostureRefusal::CompareAndSwapMismatch)
    );
}
