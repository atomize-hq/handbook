pub use handbook_engine::baseline_validation::{
    BaselineArtifactValidation, BaselineArtifactVerdict,
};

use crate::author::validate_charter_markdown;
use crate::canonical_artifacts::{CanonicalArtifactIdentity, CanonicalArtifacts};

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
    instance_id: &str,
) -> Option<BaselineArtifactValidation> {
    handbook_engine::baseline_validation::baseline_artifact_validation(
        artifacts,
        instance_id,
        validate_artifact_markdown,
    )
}

fn validate_artifact_markdown(
    identity: &CanonicalArtifactIdentity,
    markdown: &str,
) -> Result<(), String> {
    match (identity.instance_id.as_str(), identity.kind_ref.as_str()) {
        ("project_authority", "handbook.artifact-kind.project-authority@1.1.0") => {
            validate_charter_markdown(markdown)
        }
        ("project_context", "handbook.artifact-kind.project-context@1.1.0") => {
            Err("selected Project Context YAML is validated through profile inspection".to_owned())
        }
        ("environment_context", "handbook.artifact-kind.environment-context@1.1.0") => {
            Err("Environment Context is validated from selected canonical YAML".to_owned())
        }
        _ => Err("selected artifact has no baseline Markdown validator".to_owned()),
    }
}
