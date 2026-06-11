pub use handbook_engine::baseline_validation::{
    baseline_artifact_validation_for_path, BaselineArtifactValidation, BaselineArtifactVerdict,
};

use crate::author::{
    validate_charter_markdown, validate_environment_inventory_markdown,
    validate_project_context_markdown,
};
use crate::canonical_artifacts::{CanonicalArtifactKind, CanonicalArtifacts};

pub fn baseline_artifact_validations(
    artifacts: &CanonicalArtifacts,
) -> Vec<BaselineArtifactValidation> {
    handbook_engine::baseline_validation::baseline_artifact_validations(
        artifacts,
        validate_artifact_markdown,
    )
}

pub fn baseline_artifact_validation(
    artifacts: &CanonicalArtifacts,
    kind: CanonicalArtifactKind,
) -> Option<BaselineArtifactValidation> {
    handbook_engine::baseline_validation::baseline_artifact_validation(
        artifacts,
        kind,
        validate_artifact_markdown,
    )
}

fn validate_artifact_markdown(kind: CanonicalArtifactKind, markdown: &str) -> Result<(), String> {
    match kind {
        CanonicalArtifactKind::Charter => validate_charter_markdown(markdown),
        CanonicalArtifactKind::ProjectContext => {
            validate_project_context_markdown(markdown).map_err(|err| err.to_string())
        }
        CanonicalArtifactKind::EnvironmentInventory => {
            validate_environment_inventory_markdown(markdown).map_err(|err| err.summary)
        }
        CanonicalArtifactKind::FeatureSpec => {
            Err("feature spec is not part of baseline validation".to_string())
        }
    }
}
