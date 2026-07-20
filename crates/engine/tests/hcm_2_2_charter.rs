use handbook_engine::{
    charter_rendered_fingerprint, charter_source_fingerprint, parse_canonical_charter,
    render_canonical_charter_markdown, resolve_shipped_profile_decisions,
    serialize_canonical_charter, CharterArtifactErrorKind,
};
use std::path::Path;

const BOUNDARY_YAML: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/specs/handbook-contract-membrane/slices/HCM-2.2/contracts/canonical-charter-boundary-v1.1.yaml"
));
const BOUNDARY_MARKDOWN: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/specs/handbook-contract-membrane/slices/HCM-2.2/contracts/charter-review-boundary-v1.0.md"
));

fn decisions() -> handbook_engine::ResolvedProfileDecisions {
    resolve_shipped_profile_decisions(Path::new(env!("CARGO_MANIFEST_DIR")))
        .expect("shipped decisions")
}

#[test]
fn canonical_charter_reproduces_the_literal_yaml_and_markdown_boundaries() {
    let charter =
        parse_canonical_charter(&decisions(), BOUNDARY_YAML).expect("selected canonical Charter");

    assert_eq!(charter.schema_id, "handbook.artifact.project-authority");
    assert_eq!(charter.schema_version, "1.1");
    assert_eq!(charter.record_id, "project.charter.boundary");
    assert_eq!(charter.engineering_posture.dimensions.len(), 9);
    assert_eq!(
        serialize_canonical_charter(&decisions(), &charter).expect("canonical bytes"),
        BOUNDARY_YAML
    );
    assert_eq!(
        render_canonical_charter_markdown(&charter).expect("review view"),
        BOUNDARY_MARKDOWN
    );
    assert_eq!(
        charter_source_fingerprint(BOUNDARY_YAML).to_string(),
        "sha256:36d607d2aa8dd5b433ce79fcb106ff2fa2a2fe291509b4ec874d349866226e44"
    );
    assert_eq!(
        charter_rendered_fingerprint(BOUNDARY_MARKDOWN).to_string(),
        "sha256:4bebff7b716e624a955637f9462e65f4b42b6857b31b0b2add8809034c7b1061"
    );
}

#[test]
fn canonical_charter_refuses_duplicate_keys_wrong_dimension_order_and_controls() {
    let duplicate = String::from_utf8(BOUNDARY_YAML.to_vec()).unwrap().replacen(
        "schema_version: \"1.1\"",
        "schema_version: \"1.1\"\nschema_version: \"1.1\"",
        1,
    );
    assert_eq!(
        parse_canonical_charter(&decisions(), duplicate.as_bytes())
            .unwrap_err()
            .kind(),
        CharterArtifactErrorKind::DuplicateKey
    );

    let reordered = String::from_utf8(BOUNDARY_YAML.to_vec()).unwrap().replacen(
        "dimension_id: \"speed_vs_quality\"",
        "dimension_id: \"testing_rigor\"",
        1,
    );
    assert_eq!(
        parse_canonical_charter(&decisions(), reordered.as_bytes())
            .unwrap_err()
            .kind(),
        CharterArtifactErrorKind::StructuralValidationFailed
    );

    let mut charter = parse_canonical_charter(&decisions(), BOUNDARY_YAML).unwrap();
    charter.policy.authority_statement = "refuse\u{7f}view".to_owned();
    assert_eq!(
        render_canonical_charter_markdown(&charter)
            .unwrap_err()
            .kind(),
        CharterArtifactErrorKind::RenderedViewRefused
    );
}

#[test]
fn disabled_decision_records_ignore_bounded_non_empty_metadata_in_the_view() {
    let mut charter = parse_canonical_charter(&decisions(), BOUNDARY_YAML).unwrap();
    charter.decision_records.enabled = false;
    charter.decision_records.path = "ignored/path".to_owned();
    charter.decision_records.format = "ignored format".to_owned();

    let rendered = String::from_utf8(
        render_canonical_charter_markdown(&charter).expect("disabled decision-record view"),
    )
    .unwrap();
    assert!(rendered.contains("ADRs are not mandatory by default for this project."));
    assert!(!rendered.contains("ignored/path"));
    assert!(!rendered.contains("ignored format"));
}
