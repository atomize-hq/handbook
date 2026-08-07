use crate::definition_identity::read_bounded_regular_source;
use crate::snapshot_memory::derive_grounding_transition;
use crate::{parse_schema_json, DefinitionFingerprint, ExactDefinitionRef, SourceByteBudget};
use serde::Deserialize;
use serde_json::Value;
use std::path::Path;

#[cfg(test)]
use std::path::PathBuf;

const SOURCE_DIRECTORY: [&str; 4] = [".handbook", "grounding", "hcm-3.5", "v1"];
const PRIOR_END_REF: &str = "handbook.hcm-3-5.transition-snapshot.prior-end@1.0.0";
const SESSION_START_REF: &str = "handbook.hcm-3-5.transition-snapshot.session-start@1.0.0";
const SESSION_END_REF: &str = "handbook.hcm-3-5.transition-snapshot.session-end@1.0.0";
const PROJECTION_REF: &str = "handbook.hcm-3-5.grounding-projection@1.0.0";

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct GroundingTransitionRefs {
    prior_end_snapshot_ref: String,
    session_start_snapshot_ref: String,
    grounding_delta_ref: String,
    grounding_projection_ref: String,
    session_end_snapshot_ref: String,
    session_delta_ref: String,
}

impl GroundingTransitionRefs {
    pub(crate) fn prior_end_snapshot_ref(&self) -> &str {
        &self.prior_end_snapshot_ref
    }

    pub(crate) fn session_start_snapshot_ref(&self) -> &str {
        &self.session_start_snapshot_ref
    }

    pub(crate) fn grounding_delta_ref(&self) -> &str {
        &self.grounding_delta_ref
    }

    pub(crate) fn grounding_projection_ref(&self) -> &str {
        &self.grounding_projection_ref
    }

    pub(crate) fn session_end_snapshot_ref(&self) -> &str {
        &self.session_end_snapshot_ref
    }

    pub(crate) fn session_delta_ref(&self) -> &str {
        &self.session_delta_ref
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum GroundingTransitionMaterializationError {
    InvalidSource,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct GroundingProjectionDescriptor {
    schema_id: String,
    schema_version: String,
    snapshot_ref: String,
    delta_ref: String,
    definition_ref: String,
    disclosure_ref: String,
    summary_entry_count: usize,
    omission_count: usize,
    evidence_availability: String,
}

pub(crate) fn materialize_grounding_transition_refs(
    repo_root: &Path,
) -> Result<GroundingTransitionRefs, GroundingTransitionMaterializationError> {
    let mut source_budget = SourceByteBudget::default();
    let policy = read_source(repo_root, "snapshot-policy.json", &mut source_budget)?;
    let prior_capture = read_source(repo_root, "prior-capture.json", &mut source_budget)?;
    let prior_snapshot = read_source(repo_root, "prior-snapshot.json", &mut source_budget)?;
    let session_start_capture = read_source(repo_root, "current-capture.json", &mut source_budget)?;
    let session_start_snapshot =
        read_source(repo_root, "current-snapshot.json", &mut source_budget)?;
    let session_end_capture =
        read_source(repo_root, "session-end-capture.json", &mut source_budget)?;
    let session_end_snapshot =
        read_source(repo_root, "session-end-snapshot.json", &mut source_budget)?;
    let catalog = read_source(repo_root, "delta-catalog.json", &mut source_budget)?;
    let grounding_route = read_source(repo_root, "delta-route.json", &mut source_budget)?;
    let session_route = read_source(repo_root, "session-delta-route.json", &mut source_budget)?;
    let transition = derive_grounding_transition(
        &policy,
        &prior_capture,
        &prior_snapshot,
        &session_start_capture,
        &session_start_snapshot,
        &session_end_capture,
        &session_end_snapshot,
        &catalog,
        &grounding_route,
        &session_route,
    )
    .map_err(|_| GroundingTransitionMaterializationError::InvalidSource)?;

    let grounding_delta_ref = exact_token(
        transition.grounding_delta_ref(),
        transition.grounding_delta_route_fingerprint(),
    )?;
    let session_delta_ref = exact_token(
        transition.session_delta_ref(),
        transition.session_delta_route_fingerprint(),
    )?;
    let definition_ref = exact_json_ref(
        &read_source(repo_root, "definition.json", &mut source_budget)?,
        "definition_ref",
    )?;
    let disclosure_ref = exact_json_ref(
        &read_source(repo_root, "disclosure.json", &mut source_budget)?,
        "disclosure_ref",
    )?;
    let projection_value = parse_schema_json(&read_source(
        repo_root,
        "grounding-projection.json",
        &mut source_budget,
    )?)
    .map_err(|_| GroundingTransitionMaterializationError::InvalidSource)?;
    let projection: GroundingProjectionDescriptor =
        serde_json::from_value(projection_value.clone())
            .map_err(|_| GroundingTransitionMaterializationError::InvalidSource)?;
    let expected_snapshot_ref = exact_token(
        "handbook.grounding.snapshot.current@1.0.0",
        transition.session_start_snapshot_fingerprint(),
    )?;
    if projection.schema_id != "handbook.hcm-3-5-grounding-projection"
        || projection.schema_version != "1.0"
        || projection.snapshot_ref != expected_snapshot_ref
        || projection.delta_ref != grounding_delta_ref
        || projection.definition_ref != definition_ref
        || projection.disclosure_ref != disclosure_ref
        || projection.summary_entry_count != 2
        || projection.omission_count != 3
        || projection.evidence_availability != "unavailable"
    {
        return Err(GroundingTransitionMaterializationError::InvalidSource);
    }

    Ok(GroundingTransitionRefs {
        prior_end_snapshot_ref: exact_token(
            PRIOR_END_REF,
            transition.prior_end_snapshot_fingerprint(),
        )?,
        session_start_snapshot_ref: exact_token(
            SESSION_START_REF,
            transition.session_start_snapshot_fingerprint(),
        )?,
        grounding_delta_ref,
        grounding_projection_ref: exact_token(
            PROJECTION_REF,
            &DefinitionFingerprint::from_json_value(&projection_value)
                .map_err(|_| GroundingTransitionMaterializationError::InvalidSource)?,
        )?,
        session_end_snapshot_ref: exact_token(
            SESSION_END_REF,
            transition.session_end_snapshot_fingerprint(),
        )?,
        session_delta_ref,
    })
}

#[cfg(test)]
fn source_root(repo_root: &Path) -> PathBuf {
    SOURCE_DIRECTORY
        .iter()
        .fold(repo_root.to_path_buf(), |path, segment| path.join(segment))
}

fn read_source(
    repo_root: &Path,
    name: &str,
    budget: &mut SourceByteBudget,
) -> Result<Vec<u8>, GroundingTransitionMaterializationError> {
    read_bounded_regular_source(repo_root, &source_relative_path(name), budget)
        .map_err(|_| GroundingTransitionMaterializationError::InvalidSource)
}

fn exact_json_ref(
    bytes: &[u8],
    ref_field: &str,
) -> Result<String, GroundingTransitionMaterializationError> {
    let value = parse_schema_json(bytes)
        .map_err(|_| GroundingTransitionMaterializationError::InvalidSource)?;
    let reference = value
        .get(ref_field)
        .and_then(Value::as_str)
        .ok_or(GroundingTransitionMaterializationError::InvalidSource)?;
    exact_token(
        reference,
        &DefinitionFingerprint::from_json_value(&value)
            .map_err(|_| GroundingTransitionMaterializationError::InvalidSource)?,
    )
}

fn source_relative_path(name: &str) -> String {
    format!("{}/{}", SOURCE_DIRECTORY.join("/"), name)
}

fn exact_token(
    reference: &str,
    fingerprint: &DefinitionFingerprint,
) -> Result<String, GroundingTransitionMaterializationError> {
    ExactDefinitionRef::parse(reference)
        .map_err(|_| GroundingTransitionMaterializationError::InvalidSource)?;
    Ok(format!("{reference}#{fingerprint}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn materializes_the_p4_transition_refs_from_the_tracked_source_route() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let refs = materialize_grounding_transition_refs(&root).expect("P4 transition source");

        assert_eq!(
            refs.prior_end_snapshot_ref(),
            "handbook.hcm-3-5.transition-snapshot.prior-end@1.0.0#sha256:5a6d11b725b396fdcc01c4e675fadd46f65508b5cf83d0adafe85d78143666d4"
        );
        assert_eq!(
            refs.session_start_snapshot_ref(),
            "handbook.hcm-3-5.transition-snapshot.session-start@1.0.0#sha256:beac8e254d680875a8b40c4f8aab8c23f440d4ae806b3af0fcd9cf298b465f50"
        );
        assert_eq!(
            refs.grounding_delta_ref(),
            "handbook.grounding.delta.prior-end-to-session-start@1.0.0#sha256:839ba6ea4cc3728f02a4615ab1f2e38381d9f0f1448e919f96a05a85b4ab9438"
        );
        assert_eq!(
            refs.grounding_projection_ref(),
            "handbook.hcm-3-5.grounding-projection@1.0.0#sha256:fa8a0901be95c278cebcee276e7aeba92fd64d1f19afaeb766657d40cd49392b"
        );
        assert_eq!(
            refs.session_end_snapshot_ref(),
            "handbook.hcm-3-5.transition-snapshot.session-end@1.0.0#sha256:c1c20406831464451de0d6dd95bef69099132e15cd7448b54cedc86bb7e25179"
        );
        assert_eq!(
            refs.session_delta_ref(),
            "handbook.grounding.delta.session-start-to-session-end@1.0.0#sha256:049365356ec18ad1525b3621e9b0214e4a0adcdb4b2fc72993b7d548d4d8df27"
        );
    }

    #[test]
    fn hcm_3_5_transition_source_refusal_matrix_never_materializes_refs() {
        let cases = [
            "malformed",
            "mismatched",
            "reversed",
            "stale",
            "partial",
            "tampered",
            "duplicate-key",
            "oversized",
            "symlinked",
        ];

        for case in cases {
            let repo = persisted_transition_source_fixture();
            let root = source_root(repo.path());
            match case {
                "malformed" => fs::write(root.join("definition.json"), b"{").unwrap(),
                "mismatched" => {
                    let projection_path = root.join("grounding-projection.json");
                    let mut projection: Value =
                        serde_json::from_slice(&fs::read(&projection_path).unwrap()).unwrap();
                    projection["snapshot_ref"] = serde_json::json!(
                        "handbook.grounding.snapshot.current@1.0.0#sha256:ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
                    );
                    write_json(&projection_path, &projection);
                }
                "reversed" => {
                    let route_path = root.join("delta-route.json");
                    let mut route: Value =
                        serde_json::from_slice(&fs::read(&route_path).unwrap()).unwrap();
                    let prior = route["prior_snapshot_fingerprint"].clone();
                    route["prior_snapshot_fingerprint"] =
                        route["current_snapshot_fingerprint"].clone();
                    route["current_snapshot_fingerprint"] = prior;
                    write_json(&route_path, &route);
                }
                "stale" => {
                    let snapshot_path = root.join("session-end-snapshot.json");
                    let mut snapshot: Value =
                        serde_json::from_slice(&fs::read(&snapshot_path).unwrap()).unwrap();
                    snapshot["boundary_sequence"] = serde_json::json!(1);
                    write_json(&snapshot_path, &snapshot);
                }
                "partial" => fs::remove_file(root.join("snapshot-policy.json")).unwrap(),
                "tampered" => {
                    let definition_path = root.join("definition.json");
                    let mut definition: Value =
                        serde_json::from_slice(&fs::read(&definition_path).unwrap()).unwrap();
                    definition["maximum_cardinality"] = serde_json::json!(3);
                    write_json(&definition_path, &definition);
                }
                "duplicate-key" => {
                    let definition_path = root.join("definition.json");
                    let original = fs::read(&definition_path).unwrap();
                    let mut duplicate =
                        b"{\"schema_id\":\"handbook.grounding-summary-definition\",".to_vec();
                    duplicate.extend_from_slice(&original[1..]);
                    fs::write(definition_path, duplicate).unwrap();
                }
                "oversized" => {
                    let definition_path = root.join("definition.json");
                    let mut oversized =
                        vec![b' '; crate::definition_identity::MAX_SOURCE_DOCUMENT_BYTES + 1];
                    oversized.extend_from_slice(&fs::read(&definition_path).unwrap());
                    fs::write(definition_path, oversized).unwrap();
                }
                "symlinked" => {
                    let definition_path = root.join("definition.json");
                    let target = repo.path().join("outside-definition.json");
                    fs::write(&target, fs::read(&definition_path).unwrap()).unwrap();
                    fs::remove_file(&definition_path).unwrap();
                    create_file_symlink(&target, &definition_path);
                }
                _ => unreachable!("table case is exhaustive"),
            }

            assert_eq!(
                materialize_grounding_transition_refs(repo.path()),
                Err(GroundingTransitionMaterializationError::InvalidSource),
                "case {case} must not materialize usable transition references"
            );
        }
    }

    fn persisted_transition_source_fixture() -> tempfile::TempDir {
        let repo = tempfile::tempdir().unwrap();
        let destination = source_root(repo.path());
        fs::create_dir_all(&destination).unwrap();
        let source = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join(".handbook/grounding/hcm-3.5/v1");
        for name in [
            "snapshot-policy.json",
            "prior-capture.json",
            "prior-snapshot.json",
            "current-capture.json",
            "current-snapshot.json",
            "session-end-capture.json",
            "session-end-snapshot.json",
            "delta-catalog.json",
            "delta-route.json",
            "session-delta-route.json",
            "definition.json",
            "disclosure.json",
            "grounding-projection.json",
        ] {
            fs::copy(source.join(name), destination.join(name)).unwrap();
        }
        repo
    }

    fn write_json(path: &Path, value: &Value) {
        fs::write(path, serde_json::to_vec(value).unwrap()).unwrap();
    }

    #[cfg(unix)]
    fn create_file_symlink(target: &Path, link: &Path) {
        std::os::unix::fs::symlink(target, link).expect("test symlink is available");
    }

    #[cfg(windows)]
    fn create_file_symlink(target: &Path, link: &Path) {
        std::os::windows::fs::symlink_file(target, link).expect("test symlink is available");
    }

    #[cfg(all(not(unix), not(windows)))]
    fn create_file_symlink(_target: &Path, _link: &Path) {
        panic!("HCM-3.5 source admission requires a supported no-follow platform");
    }
}
