pub use handbook_engine::baseline_validation::{
    BaselineArtifactValidation, BaselineArtifactVerdict,
};

use crate::author::validate_charter_markdown;
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
            Err("selected Project Context YAML is validated through profile inspection".to_owned())
        }
        CanonicalArtifactKind::EnvironmentContext => {
            Err("Environment Context is validated from selected canonical YAML".to_owned())
        }
        CanonicalArtifactKind::FeatureSpec => {
            Err("feature spec is not part of baseline validation".to_string())
        }
    }
}
