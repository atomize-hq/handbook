use handbook_engine::grounding::{
    GroundingDefinitionRef, GroundingDeltaRef, GroundingDisclosureRef, GroundingSnapshotRef,
};

const FINGERPRINT: &str = "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

#[test]
fn grounding_references_are_opaque_and_require_an_exact_fingerprint_binding() {
    let snapshot = format!("handbook.grounding.snapshot.current@1.0.0#{FINGERPRINT}");
    let delta = format!("handbook.grounding.delta.prior-current@1.0.0#{FINGERPRINT}");
    let definition = format!("handbook.grounding.summary.hcm-3-5-p1@1.0.0#{FINGERPRINT}");
    let disclosure = format!("handbook.grounding.disclosure.hcm-3-5-p1@1.0.0#{FINGERPRINT}");

    assert!(GroundingSnapshotRef::parse_exact(&snapshot).is_ok());
    assert!(GroundingDeltaRef::parse_exact(&delta).is_ok());
    assert!(GroundingDefinitionRef::parse_exact(&definition).is_ok());
    assert!(GroundingDisclosureRef::parse_exact(&disclosure).is_ok());
    assert!(GroundingSnapshotRef::parse_exact("../snapshot.json").is_err());
    assert!(GroundingDeltaRef::parse_exact("handbook.delta@1.0.0").is_err());
}
