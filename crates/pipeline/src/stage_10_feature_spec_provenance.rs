use crate::layout::RepoLayoutRoot;
use crate::pipeline::{
    load_selected_pipeline_definition, load_stage_compile_definition, SupportedTargetRegistry,
};
use crate::pipeline_compile::{render_pipeline_compile_payload, PipelineCompileResult};
use crate::repo_file_access::{
    read_repo_relative_string, sha256_repo_relative_file, write_repo_relative_bytes,
    NormalizedRepoRelativePath,
};
use crate::route_state::RouteBasis;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::path::Path;

pub(crate) const FEATURE_SPEC_ARTIFACT_PATH: &str = "artifacts/feature_spec/FEATURE_SPEC.md";
pub(crate) const STAGE_10_FEATURE_SPEC_CAPTURE_PROVENANCE_SCHEMA_VERSION: &str =
    "m5-stage-10-feature-spec-capture-provenance-v1";

const SUPPORTED_PIPELINE_ID: &str = "pipeline.foundation_inputs";
const SUPPORTED_STAGE_ID: &str = "stage.10_feature_spec";
const FEATURE_SPEC_TEMPLATE_SUFFIX: &str = "FEATURE_SPEC.md.tmpl";

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Stage10FeatureSpecCompileProvenance {
    pub route_basis_state_revision: u64,
    pub route_basis_fingerprint_sha256: String,
    pub stage_file: String,
    pub stage_version: String,
    pub template_path: Option<String>,
    pub template_sha256: Option<String>,
    pub payload_sha256: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Stage10FeatureSpecCaptureProvenance {
    pub schema_version: String,
    pub pipeline_id: String,
    pub stage_id: String,
    pub feature_spec_path: String,
    pub feature_spec_sha256: String,
    pub route_basis_state_revision: u64,
    pub route_basis_fingerprint_sha256: String,
    pub stage_file: String,
    pub stage_version: String,
    pub template_path: Option<String>,
    pub template_sha256: Option<String>,
    pub payload_sha256: String,
}

pub(crate) fn build_stage_10_feature_spec_capture_provenance(
    repo_root: &Path,
    compile_result: &PipelineCompileResult,
    feature_spec_sha256: &str,
) -> Result<Stage10FeatureSpecCaptureProvenance, String> {
    let compile_provenance =
        build_stage_10_feature_spec_compile_provenance(repo_root, compile_result)?;
    Ok(Stage10FeatureSpecCaptureProvenance {
        schema_version: STAGE_10_FEATURE_SPEC_CAPTURE_PROVENANCE_SCHEMA_VERSION.to_string(),
        pipeline_id: compile_result.target.pipeline_id.clone(),
        stage_id: compile_result.target.stage_id.clone(),
        feature_spec_path: FEATURE_SPEC_ARTIFACT_PATH.to_string(),
        feature_spec_sha256: feature_spec_sha256.to_string(),
        route_basis_state_revision: compile_provenance.route_basis_state_revision,
        route_basis_fingerprint_sha256: compile_provenance.route_basis_fingerprint_sha256,
        stage_file: compile_provenance.stage_file,
        stage_version: compile_provenance.stage_version,
        template_path: compile_provenance.template_path,
        template_sha256: compile_provenance.template_sha256,
        payload_sha256: compile_provenance.payload_sha256,
    })
}

pub(crate) fn build_stage_10_feature_spec_compile_provenance(
    repo_root: &Path,
    compile_result: &PipelineCompileResult,
) -> Result<Stage10FeatureSpecCompileProvenance, String> {
    let registry = load_supported_target_registry(repo_root)?;
    validate_supported_target(
        &registry,
        &compile_result.target.pipeline_id,
        &compile_result.target.stage_id,
    )?;

    let pipeline = load_selected_pipeline_definition(repo_root, &compile_result.target.pipeline_id)
        .map_err(|err| format!("failed to reload selected pipeline definition: {err}"))?;
    let stage_definition =
        load_stage_compile_definition(repo_root, &pipeline, &compile_result.target.stage_id)
            .map_err(|err| format!("failed to load feature-spec stage definition: {err}"))?;
    let template_path = compile_result
        .documents
        .iter()
        .find(|document| document.path.ends_with(FEATURE_SPEC_TEMPLATE_SUFFIX))
        .map(|document| document.path.clone());
    let template_sha256 = template_path
        .as_ref()
        .map(|path| {
            sha256_repo_relative_file(repo_root, path).map_err(|err| {
                format!(
                    "failed to fingerprint feature-spec template provenance at `{path}`: {}",
                    format_repo_file_access_error(&err)
                )
            })
        })
        .transpose()?;

    Ok(Stage10FeatureSpecCompileProvenance {
        route_basis_state_revision: compile_result.basis.state_revision,
        route_basis_fingerprint_sha256: route_basis_fingerprint_sha256(&compile_result.basis)?,
        stage_file: stage_definition.source_path.to_string_lossy().into_owned(),
        stage_version: stage_definition.version,
        template_path,
        template_sha256,
        payload_sha256: pipeline_compile_payload_sha256(compile_result),
    })
}

pub(crate) fn persist_stage_10_feature_spec_capture_provenance(
    repo_root: &Path,
    provenance: &Stage10FeatureSpecCaptureProvenance,
) -> Result<(), String> {
    let provenance_path = stage_10_feature_spec_capture_provenance_path(repo_root);
    let bytes = serde_json::to_vec_pretty(provenance)
        .map_err(|err| format!("failed to serialize stage-10 capture provenance: {err}"))?;
    write_repo_relative_bytes(repo_root, provenance_path.as_str(), &bytes)
        .map_err(|err| format_repo_mutation_error(provenance_path.as_str(), err))
}

pub(crate) fn load_stage_10_feature_spec_capture_provenance(
    repo_root: &Path,
) -> Result<Stage10FeatureSpecCaptureProvenance, String> {
    let provenance_path = stage_10_feature_spec_capture_provenance_path(repo_root);
    let body = read_repo_relative_string(repo_root, provenance_path.as_str()).map_err(|err| {
        format!(
            "stage-10 capture provenance is missing or unreadable at `{}`: {}",
            provenance_path.as_str(),
            format_repo_file_access_error(&err)
        )
    })?;
    let provenance: Stage10FeatureSpecCaptureProvenance =
        serde_json::from_str(&body).map_err(|err| {
            format!(
                "stage-10 capture provenance at `{}` is not valid JSON: {err}",
                provenance_path.as_str()
            )
        })?;
    let registry = load_supported_target_registry(repo_root)?;
    validate_capture_provenance(&registry, &provenance)?;
    Ok(provenance)
}

pub(crate) fn validate_stage_10_feature_spec_capture_provenance_match(
    stored: &Stage10FeatureSpecCaptureProvenance,
    current: &Stage10FeatureSpecCaptureProvenance,
) -> Result<(), String> {
    if stored.feature_spec_path != current.feature_spec_path {
        return Err(format!(
            "stage-10 capture provenance feature_spec_path `{}` does not match current `{}`",
            stored.feature_spec_path, current.feature_spec_path
        ));
    }
    if stored.feature_spec_sha256 != current.feature_spec_sha256 {
        return Err(format!(
            "stage-10 capture provenance feature_spec_sha256 `{}` does not match current FEATURE_SPEC.md `{}`",
            stored.feature_spec_sha256, current.feature_spec_sha256
        ));
    }
    if stored.route_basis_state_revision != current.route_basis_state_revision
        || stored.route_basis_fingerprint_sha256 != current.route_basis_fingerprint_sha256
    {
        return Err(format!(
            "stage-10 capture provenance route-basis revision/hash {}/{} do not match fresh compile {}/{}",
            stored.route_basis_state_revision,
            stored.route_basis_fingerprint_sha256,
            current.route_basis_state_revision,
            current.route_basis_fingerprint_sha256
        ));
    }
    if stored.stage_file != current.stage_file || stored.stage_version != current.stage_version {
        return Err(format!(
            "stage-10 capture provenance stage file/version `{}` / `{}` do not match fresh compile `{}` / `{}`",
            stored.stage_file, stored.stage_version, current.stage_file, current.stage_version
        ));
    }
    if stored.template_path != current.template_path
        || stored.template_sha256 != current.template_sha256
    {
        return Err(format!(
            "stage-10 capture provenance template path/hash `{}` / `{}` do not match fresh compile `{}` / `{}`",
            render_optional_field(stored.template_path.as_deref()),
            render_optional_field(stored.template_sha256.as_deref()),
            render_optional_field(current.template_path.as_deref()),
            render_optional_field(current.template_sha256.as_deref()),
        ));
    }
    if stored.payload_sha256 != current.payload_sha256 {
        return Err(format!(
            "stage-10 capture provenance payload_sha256 `{}` does not match fresh compile `{}`",
            stored.payload_sha256, current.payload_sha256
        ));
    }
    Ok(())
}

pub(crate) fn pipeline_compile_payload_sha256(compile_result: &PipelineCompileResult) -> String {
    let payload = render_pipeline_compile_payload(compile_result);
    sha256_hex(normalize_compile_payload_for_provenance(&payload).as_bytes())
}

pub(crate) fn route_basis_fingerprint_sha256(route_basis: &RouteBasis) -> Result<String, String> {
    let bytes = serde_json::to_vec(route_basis).map_err(|err| err.to_string())?;
    Ok(sha256_hex(&bytes))
}

pub(crate) fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}

fn normalize_compile_payload_for_provenance(payload: &str) -> String {
    let mut normalized = payload
        .lines()
        .filter(|line| !line.trim_start().starts_with("- now_utc: "))
        .collect::<Vec<_>>()
        .join("\n");
    if payload.ends_with('\n') {
        normalized.push('\n');
    }
    normalized
}

fn stage_10_feature_spec_capture_provenance_path(repo_root: &Path) -> NormalizedRepoRelativePath {
    RepoLayoutRoot::new(repo_root)
        .capture_provenance()
        .stage_capture_provenance_relative_path(SUPPORTED_PIPELINE_ID, SUPPORTED_STAGE_ID)
}

fn load_supported_target_registry(repo_root: &Path) -> Result<SupportedTargetRegistry, String> {
    SupportedTargetRegistry::load(repo_root)
        .map_err(|err| format!("failed to load supported target registry: {err}"))
}

fn validate_supported_target(
    registry: &SupportedTargetRegistry,
    pipeline_id: &str,
    stage_id: &str,
) -> Result<(), String> {
    if !registry.supports_provenance_target(pipeline_id, stage_id) {
        let supported_target = registry.compile_target();
        return Err(format!(
            "stage-10 capture provenance currently supports only `{}` + `{}`, got `{pipeline_id}` + `{stage_id}`",
            supported_target.pipeline.id, supported_target.stage.id
        ));
    }
    Ok(())
}

fn validate_capture_provenance(
    registry: &SupportedTargetRegistry,
    provenance: &Stage10FeatureSpecCaptureProvenance,
) -> Result<(), String> {
    if provenance.schema_version != STAGE_10_FEATURE_SPEC_CAPTURE_PROVENANCE_SCHEMA_VERSION {
        return Err(format!(
            "stage-10 capture provenance schema_version `{}` does not match expected `{}`",
            provenance.schema_version, STAGE_10_FEATURE_SPEC_CAPTURE_PROVENANCE_SCHEMA_VERSION
        ));
    }
    validate_supported_target(registry, &provenance.pipeline_id, &provenance.stage_id)?;
    if provenance.feature_spec_path != FEATURE_SPEC_ARTIFACT_PATH {
        return Err(format!(
            "stage-10 capture provenance feature_spec_path `{}` does not match expected `{}`",
            provenance.feature_spec_path, FEATURE_SPEC_ARTIFACT_PATH
        ));
    }
    Ok(())
}

fn render_optional_field(value: Option<&str>) -> &str {
    value.unwrap_or("<none>")
}

fn format_repo_file_access_error(
    err: &crate::repo_file_access::RepoRelativeFileAccessError,
) -> String {
    match err {
        crate::repo_file_access::RepoRelativeFileAccessError::Missing(path) => {
            format!("missing {}", path.display())
        }
        crate::repo_file_access::RepoRelativeFileAccessError::InvalidPath(reason) => reason.clone(),
        crate::repo_file_access::RepoRelativeFileAccessError::SymlinkNotAllowed(path) => {
            format!("symlink not allowed: {}", path.display())
        }
        crate::repo_file_access::RepoRelativeFileAccessError::NotRegularFile(path) => {
            format!("not a regular file: {}", path.display())
        }
        crate::repo_file_access::RepoRelativeFileAccessError::ReadFailure { path, source } => {
            format!("{} ({source})", path.display())
        }
    }
}

fn format_repo_mutation_error(
    path: &str,
    err: crate::repo_file_access::RepoRelativeMutationError,
) -> String {
    match err {
        crate::repo_file_access::RepoRelativeMutationError::InvalidPath(reason) => {
            format!("write target `{path}` is invalid: {reason}")
        }
        crate::repo_file_access::RepoRelativeMutationError::ParentNotDirectory(found) => {
            format!(
                "write target `{path}` cannot be written because {} is not a directory",
                found.display()
            )
        }
        crate::repo_file_access::RepoRelativeMutationError::NotRegularFile(found) => {
            format!(
                "write target `{path}` cannot be written because {} is not a regular file target",
                found.display()
            )
        }
        crate::repo_file_access::RepoRelativeMutationError::SymlinkNotAllowed(found) => {
            format!(
                "write target `{path}` cannot be written through symlink {}",
                found.display()
            )
        }
        crate::repo_file_access::RepoRelativeMutationError::ReadFailure {
            path: found,
            source,
        }
        | crate::repo_file_access::RepoRelativeMutationError::WriteFailure {
            path: found,
            source,
        } => {
            format!(
                "failed to mutate write target `{path}` at {}: {source}",
                found.display()
            )
        }
    }
}
