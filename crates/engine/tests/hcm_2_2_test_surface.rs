const TRANSACTION_SOURCE: &str = include_str!("../src/charter_authority_transaction.rs");
const LIFECYCLE_SOURCE: &str = include_str!("../src/charter_lifecycle_store.rs");
const PROMOTION_SOURCE: &str = include_str!("../src/charter_promotion_workflow.rs");
const LIB_SOURCE: &str = include_str!("../src/lib.rs");

#[test]
fn production_surface_contains_no_fault_or_caller_clock_controls() {
    assert!(!LIFECYCLE_SOURCE.contains("pub enum CharterLifecycleFaultPointV1"));
    assert!(!LIFECYCLE_SOURCE.contains("pub fn record_event_with_fault_for_testing"));
    assert!(!PROMOTION_SOURCE.contains("pub fn promote_at_for_testing"));
    assert!(!LIB_SOURCE.contains("CharterLifecycleFaultPointV1,"));

    assert!(TRANSACTION_SOURCE.contains(
        "#[cfg(test)]\n#[derive(Clone, Copy, Debug, Eq, PartialEq)]\nenum CharterPromotionFaultPointV1"
    ));
    assert!(LIFECYCLE_SOURCE.contains(
        "#[cfg(test)]\n#[derive(Clone, Copy, Debug, Eq, PartialEq)]\nenum CharterLifecycleFaultPointV1"
    ));
    assert!(!TRANSACTION_SOURCE.contains("fault: Option<CharterPromotionFaultPointV1>"));
    assert!(!LIFECYCLE_SOURCE.contains("fault: Option<CharterLifecycleFaultPointV1>"));
}
