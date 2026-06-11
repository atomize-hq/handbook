use crate::declarative_roots::{is_canonical_declarative_path, DECLARATIVE_ROOT};
use crate::layout::RepoLayoutRoot;
use crate::pipeline::{
    load_selected_pipeline_definition, supported_route_state_variables, SupportedTargetRegistry,
};
use crate::pipeline_compile::{
    compile_pipeline_stage, PipelineCompileDocument, PipelineCompileDocumentKind,
    PipelineCompileDocumentStatus, PipelineCompileRefusal, PipelineCompileRefusalClassification,
    PipelineCompileResult,
};
use crate::repo_file_access::{
    normalize_repo_relative_path, read_repo_relative_string, sha256_repo_relative_file,
    write_repo_relative_bytes, RepoRelativeFileAccessError, RepoRelativeMutationError,
};
use crate::route_state::{
    load_route_state_with_supported_variables, rebuild_canonical_route_basis, RouteStateReadError,
};
use crate::stage_10_feature_spec_provenance::{
    build_stage_10_feature_spec_capture_provenance, load_stage_10_feature_spec_capture_provenance,
    route_basis_fingerprint_sha256, sha256_hex,
    validate_stage_10_feature_spec_capture_provenance_match, FEATURE_SPEC_ARTIFACT_PATH,
};
use handbook_engine::{ArtifactManifest, ArtifactPresence, ManifestInputs};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::path::Path;

const SUPPORTED_CONSUMER_ID: &str = "feature-slice-decomposer";
const SUPPORTED_STAGE_ID: &str = "stage.10_feature_spec";
const HANDOFF_SCHEMA_VERSION: &str = "m5-pipeline-handoff-v1";
const READ_ALLOWLIST_SCHEMA_VERSION: &str = "m5-pipeline-handoff-read-allowlist-v1";
const SCORECARD_SCHEMA_VERSION: &str = "m5-pipeline-handoff-scorecard-v1";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineHandoffEmitRequest {
    pub pipeline_selector: String,
    pub consumer_selector: String,
    pub producer_command: String,
    pub producer_version: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineHandoffEmitResult {
    pub manifest: PipelineHandoffManifest,
    pub read_allowlist: PipelineHandoffReadAllowlist,
    pub bundle_root: String,
    pub written_files: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineHandoffValidatedBundle {
    pub manifest: PipelineHandoffManifest,
    pub read_allowlist: PipelineHandoffReadAllowlist,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineHandoffRefusal {
    pub classification: PipelineHandoffRefusalClassification,
    pub summary: String,
    pub pipeline_id: Option<String>,
    pub consumer_id: Option<String>,
    pub recovery: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PipelineHandoffRefusalClassification {
    UnsupportedTarget,
    InvalidState,
    MissingRequiredInput,
    InvalidProvenance,
    WriteFailure,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineHandoffValidationFailure {
    pub classification: PipelineHandoffValidationFailureClassification,
    pub summary: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PipelineHandoffValidationFailureClassification {
    UnsupportedTarget,
    StaleCanonicalProvenance,
    TamperedDerivedInput,
    MissingOrCorruptProvenance,
    TrustClassMismatch,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PipelineHandoffTrustClass {
    Canonical,
    CompilerDerived,
    ExternalManualDerived,
}

impl PipelineHandoffTrustClass {
    fn bundle_segment(&self) -> &'static str {
        match self {
            Self::Canonical => "canonical",
            Self::CompilerDerived => "compiler_derived",
            Self::ExternalManualDerived => "external_manual_derived",
        }
    }
}

impl fmt::Display for PipelineHandoffTrustClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Canonical => write!(f, "canonical"),
            Self::CompilerDerived => write!(f, "compiler_derived"),
            Self::ExternalManualDerived => write!(f, "external_manual_derived"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PipelineHandoffManifest {
    pub schema_version: String,
    pub producer: PipelineHandoffProducer,
    pub pipeline_id: String,
    pub consumer_id: String,
    pub feature_id: String,
    pub bundle_root: String,
    pub route_basis: PipelineHandoffRouteBasisProvenance,
    pub canonical_provenance: PipelineHandoffCanonicalProvenance,
    pub inputs: Vec<PipelineHandoffInput>,
    pub feature_spec_compile: PipelineHandoffFeatureSpecCompileProvenance,
    pub fallback: PipelineHandoffFallbackMetadata,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PipelineHandoffProducer {
    pub command: String,
    pub version: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PipelineHandoffRouteBasisProvenance {
    pub state_revision: u64,
    pub fingerprint_sha256: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PipelineHandoffCanonicalProvenance {
    pub manifest_fingerprint_sha256: String,
    pub artifact_fingerprints: Vec<PipelineHandoffCanonicalArtifactFingerprint>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PipelineHandoffCanonicalArtifactFingerprint {
    pub path: String,
    pub sha256: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PipelineHandoffInput {
    pub source_path: String,
    pub bundle_path: String,
    pub trust_class: PipelineHandoffTrustClass,
    pub sha256: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PipelineHandoffFeatureSpecCompileProvenance {
    pub stage_id: String,
    pub stage_file: String,
    pub stage_version: String,
    pub template_path: Option<String>,
    pub template_sha256: Option<String>,
    pub payload_sha256: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PipelineHandoffFallbackMetadata {
    pub repo_reread_allowed: bool,
    pub mode: String,
    pub notes: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PipelineHandoffReadAllowlist {
    pub schema_version: String,
    pub pipeline_id: String,
    pub consumer_id: String,
    pub feature_id: String,
    pub bundle_root: String,
    pub allow_read_paths: Vec<String>,
    pub repo_reread_allowed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PipelineHandoffScorecardMetadata {
    schema_version: String,
    pipeline_id: String,
    consumer_id: String,
    feature_id: String,
    bundle_root: String,
    status: String,
}

#[derive(Debug, Clone)]
struct InputCopyPlan {
    source_path: String,
    bundle_path: String,
    trust_class: PipelineHandoffTrustClass,
    sha256: String,
    bytes: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SupportedHandoffTarget {
    pipeline_id: String,
    stage_id: String,
    consumer_id: String,
}

pub fn emit_pipeline_handoff_bundle(
    repo_root: impl AsRef<Path>,
    request: &PipelineHandoffEmitRequest,
) -> Result<PipelineHandoffEmitResult, PipelineHandoffRefusal> {
    let repo_root = repo_root.as_ref();
    let registry = load_supported_handoff_target_registry_for_emit(repo_root)?;
    let supported_target = supported_handoff_target(&registry);
    validate_supported_consumer(&supported_target, &request.consumer_selector)?;

    let compile_result = compile_pipeline_stage(
        repo_root,
        &request.pipeline_selector,
        &supported_target.stage_id,
    )
    .map_err(|refusal| map_compile_refusal(refusal, request.consumer_selector.trim()))?;
    validate_supported_compile_target(
        &registry,
        &supported_target,
        &compile_result.target.pipeline_id,
        &compile_result.target.stage_id,
        request.consumer_selector.trim(),
    )?;

    let feature_spec_body = read_repo_relative_string(repo_root, FEATURE_SPEC_ARTIFACT_PATH)
        .map_err(|err| PipelineHandoffRefusal {
            classification: PipelineHandoffRefusalClassification::MissingRequiredInput,
            summary: format!(
                "required handoff source `{FEATURE_SPEC_ARTIFACT_PATH}` is unavailable: {}",
                format_repo_file_access_error(&err)
            ),
            pipeline_id: Some(compile_result.target.pipeline_id.clone()),
            consumer_id: Some(supported_target.consumer_id.clone()),
            recovery: format!(
                "capture `{}` output before retrying `pipeline handoff emit`",
                supported_target.stage_id
            ),
        })?;
    let feature_spec_sha256 = sha256_repo_relative_file(repo_root, FEATURE_SPEC_ARTIFACT_PATH)
        .map_err(|err| PipelineHandoffRefusal {
            classification: PipelineHandoffRefusalClassification::MissingRequiredInput,
            summary: format!(
                "required handoff source `{FEATURE_SPEC_ARTIFACT_PATH}` is unavailable: {}",
                format_repo_file_access_error(&err)
            ),
            pipeline_id: Some(compile_result.target.pipeline_id.clone()),
            consumer_id: Some(supported_target.consumer_id.clone()),
            recovery: format!(
                "capture `{}` output before retrying `pipeline handoff emit`",
                supported_target.stage_id
            ),
        })?;
    let current_capture_provenance = build_stage_10_feature_spec_capture_provenance(
        repo_root,
        &compile_result,
        &feature_spec_sha256,
    )
    .map_err(|reason| PipelineHandoffRefusal {
        classification: PipelineHandoffRefusalClassification::InvalidProvenance,
        summary: format!("failed to rebuild current stage-10 compile provenance: {reason}"),
        pipeline_id: Some(compile_result.target.pipeline_id.clone()),
        consumer_id: Some(supported_target.consumer_id.clone()),
        recovery: stage_10_provenance_recovery(),
    })?;
    let stored_capture_provenance = load_stage_10_feature_spec_capture_provenance(repo_root)
        .map_err(|reason| PipelineHandoffRefusal {
            classification: PipelineHandoffRefusalClassification::InvalidProvenance,
            summary: reason,
            pipeline_id: Some(compile_result.target.pipeline_id.clone()),
            consumer_id: Some(supported_target.consumer_id.clone()),
            recovery: stage_10_provenance_recovery(),
        })?;
    validate_stage_10_feature_spec_capture_provenance_match(
        &stored_capture_provenance,
        &current_capture_provenance,
    )
    .map_err(|reason| PipelineHandoffRefusal {
        classification: PipelineHandoffRefusalClassification::InvalidProvenance,
        summary: reason,
        pipeline_id: Some(compile_result.target.pipeline_id.clone()),
        consumer_id: Some(supported_target.consumer_id.clone()),
        recovery: stage_10_provenance_recovery(),
    })?;

    let feature_id = derive_feature_id(&feature_spec_body, &feature_spec_sha256);
    let bundle_root = RepoLayoutRoot::new(repo_root)
        .handoff_bundle()
        .feature_slice_bundle_root(&feature_id)
        .as_str()
        .to_string();
    let input_plans = build_input_copy_plans(&compile_result, &feature_spec_body)?;

    let canonical_manifest = ArtifactManifest::generate(repo_root, ManifestInputs::default())
        .map_err(|err| PipelineHandoffRefusal {
            classification: PipelineHandoffRefusalClassification::InvalidProvenance,
            summary: format!("failed to compute canonical artifact provenance: {err}"),
            pipeline_id: Some(compile_result.target.pipeline_id.clone()),
            consumer_id: Some(supported_target.consumer_id.clone()),
            recovery: "repair canonical artifact provenance and retry `pipeline handoff emit`"
                .to_string(),
        })?;
    let canonical_provenance = PipelineHandoffCanonicalProvenance {
        manifest_fingerprint_sha256: canonical_manifest.freshness.fingerprint_sha256.clone(),
        artifact_fingerprints: canonical_manifest
            .artifacts
            .into_iter()
            .filter_map(|artifact| {
                if artifact.packet_required
                    || artifact.presence == ArtifactPresence::PresentNonEmpty
                {
                    artifact.content_sha256.map(|sha256| {
                        PipelineHandoffCanonicalArtifactFingerprint {
                            path: artifact.relative_path.to_string(),
                            sha256,
                        }
                    })
                } else {
                    None
                }
            })
            .collect(),
    };

    let manifest = PipelineHandoffManifest {
        schema_version: HANDOFF_SCHEMA_VERSION.to_string(),
        producer: PipelineHandoffProducer {
            command: request.producer_command.trim().to_string(),
            version: request.producer_version.trim().to_string(),
        },
        pipeline_id: compile_result.target.pipeline_id.clone(),
        consumer_id: supported_target.consumer_id.clone(),
        feature_id: feature_id.clone(),
        bundle_root: bundle_root.clone(),
        route_basis: PipelineHandoffRouteBasisProvenance {
            state_revision: current_capture_provenance.route_basis_state_revision,
            fingerprint_sha256: current_capture_provenance
                .route_basis_fingerprint_sha256
                .clone(),
        },
        canonical_provenance,
        inputs: input_plans
            .iter()
            .map(|plan| PipelineHandoffInput {
                source_path: plan.source_path.clone(),
                bundle_path: plan.bundle_path.clone(),
                trust_class: plan.trust_class.clone(),
                sha256: plan.sha256.clone(),
            })
            .collect(),
        feature_spec_compile: PipelineHandoffFeatureSpecCompileProvenance {
            stage_id: current_capture_provenance.stage_id.clone(),
            stage_file: current_capture_provenance.stage_file.clone(),
            stage_version: current_capture_provenance.stage_version.clone(),
            template_path: current_capture_provenance.template_path.clone(),
            template_sha256: current_capture_provenance.template_sha256.clone(),
            payload_sha256: current_capture_provenance.payload_sha256.clone(),
        },
        fallback: PipelineHandoffFallbackMetadata {
            repo_reread_allowed: false,
            mode: "none".to_string(),
            notes: "happy path requires downstream consumers to read only the emitted bundle"
                .to_string(),
        },
    };

    let trust_matrix = render_trust_matrix(&manifest);
    let scorecard_metadata = PipelineHandoffScorecardMetadata {
        schema_version: SCORECARD_SCHEMA_VERSION.to_string(),
        pipeline_id: manifest.pipeline_id.clone(),
        consumer_id: manifest.consumer_id.clone(),
        feature_id: manifest.feature_id.clone(),
        bundle_root: manifest.bundle_root.clone(),
        status: "seeded".to_string(),
    };
    let scorecard_metadata_json =
        serde_json::to_vec_pretty(&scorecard_metadata).map_err(|err| PipelineHandoffRefusal {
            classification: PipelineHandoffRefusalClassification::InvalidProvenance,
            summary: format!("failed to serialize scorecard metadata: {err}"),
            pipeline_id: Some(manifest.pipeline_id.clone()),
            consumer_id: Some(manifest.consumer_id.clone()),
            recovery:
                "stabilize scorecard metadata serialization and retry `pipeline handoff emit`"
                    .to_string(),
        })?;

    let allow_read_paths = build_allow_read_paths(&input_plans);
    let read_allowlist = PipelineHandoffReadAllowlist {
        schema_version: READ_ALLOWLIST_SCHEMA_VERSION.to_string(),
        pipeline_id: manifest.pipeline_id.clone(),
        consumer_id: manifest.consumer_id.clone(),
        feature_id: manifest.feature_id.clone(),
        bundle_root: manifest.bundle_root.clone(),
        allow_read_paths: allow_read_paths.clone(),
        repo_reread_allowed: false,
    };
    let read_allowlist_json =
        serde_json::to_vec_pretty(&read_allowlist).map_err(|err| PipelineHandoffRefusal {
            classification: PipelineHandoffRefusalClassification::InvalidProvenance,
            summary: format!("failed to serialize read allowlist: {err}"),
            pipeline_id: Some(manifest.pipeline_id.clone()),
            consumer_id: Some(manifest.consumer_id.clone()),
            recovery: "stabilize read-allowlist serialization and retry `pipeline handoff emit`"
                .to_string(),
        })?;
    let manifest_json =
        serde_json::to_vec_pretty(&manifest).map_err(|err| PipelineHandoffRefusal {
            classification: PipelineHandoffRefusalClassification::InvalidProvenance,
            summary: format!("failed to serialize handoff manifest: {err}"),
            pipeline_id: Some(manifest.pipeline_id.clone()),
            consumer_id: Some(manifest.consumer_id.clone()),
            recovery: "stabilize handoff manifest serialization and retry `pipeline handoff emit`"
                .to_string(),
        })?;

    let mut writes = Vec::new();
    for input in &input_plans {
        let repo_relative_path = bundle_repo_relative_path(&bundle_root, &input.bundle_path);
        write_bundle_file(repo_root, &repo_relative_path, &input.bytes, &manifest)?;
        writes.push(repo_relative_path);
    }

    for (path, bytes) in [
        ("handoff_manifest.json", manifest_json.as_slice()),
        ("trust_matrix.md", trust_matrix.as_bytes()),
        ("read_allowlist.json", read_allowlist_json.as_slice()),
        (
            "scorecard/metadata.json",
            scorecard_metadata_json.as_slice(),
        ),
    ] {
        let repo_relative_path = bundle_repo_relative_path(&bundle_root, path);
        write_bundle_file(repo_root, &repo_relative_path, bytes, &manifest)?;
        writes.push(repo_relative_path);
    }

    Ok(PipelineHandoffEmitResult {
        manifest,
        read_allowlist,
        bundle_root,
        written_files: writes,
    })
}

pub fn validate_pipeline_handoff_bundle(
    repo_root: impl AsRef<Path>,
    bundle_root: &str,
) -> Result<PipelineHandoffValidatedBundle, PipelineHandoffValidationFailure> {
    let repo_root = repo_root.as_ref();
    let registry = load_supported_handoff_target_registry_for_validation(repo_root)?;
    let bundle_root = bundle_root.trim();
    let normalized_bundle_root = normalize_repo_relative_path(bundle_root).map_err(|reason| {
        PipelineHandoffValidationFailure {
            classification:
                PipelineHandoffValidationFailureClassification::MissingOrCorruptProvenance,
            summary: format!("bundle root `{bundle_root}` is invalid: {reason}"),
        }
    })?;

    let manifest = read_bundle_json::<PipelineHandoffManifest>(
        repo_root,
        &bundle_repo_relative_path(bundle_root, "handoff_manifest.json"),
        "handoff manifest",
    )?;
    validate_supported_manifest_target(&registry, &manifest)?;
    let normalized_manifest_bundle_root =
        normalize_bundle_root_from_metadata("handoff manifest", &manifest.bundle_root)?;
    if normalized_bundle_root != normalized_manifest_bundle_root {
        return Err(PipelineHandoffValidationFailure {
            classification:
                PipelineHandoffValidationFailureClassification::MissingOrCorruptProvenance,
            summary: format!(
                "requested bundle root `{}` does not match handoff manifest bundle_root `{}`",
                normalized_bundle_root, normalized_manifest_bundle_root
            ),
        });
    }

    let read_allowlist = read_bundle_json::<PipelineHandoffReadAllowlist>(
        repo_root,
        &bundle_repo_relative_path(bundle_root, "read_allowlist.json"),
        "read allowlist",
    )?;
    if read_allowlist.schema_version != READ_ALLOWLIST_SCHEMA_VERSION {
        return Err(PipelineHandoffValidationFailure {
            classification:
                PipelineHandoffValidationFailureClassification::MissingOrCorruptProvenance,
            summary: format!(
                "read allowlist schema_version `{}` does not match expected `{}`",
                read_allowlist.schema_version, READ_ALLOWLIST_SCHEMA_VERSION
            ),
        });
    }
    let normalized_allowlist_bundle_root =
        normalize_bundle_root_from_metadata("read allowlist", &read_allowlist.bundle_root)?;
    if normalized_allowlist_bundle_root != normalized_manifest_bundle_root
        || read_allowlist.pipeline_id != manifest.pipeline_id
        || read_allowlist.consumer_id != manifest.consumer_id
        || read_allowlist.feature_id != manifest.feature_id
        || read_allowlist.repo_reread_allowed
    {
        return Err(PipelineHandoffValidationFailure {
            classification:
                PipelineHandoffValidationFailureClassification::MissingOrCorruptProvenance,
            summary: "read allowlist metadata does not match the handoff manifest".to_string(),
        });
    }

    read_bundle_json::<PipelineHandoffScorecardMetadata>(
        repo_root,
        &bundle_repo_relative_path(bundle_root, "scorecard/metadata.json"),
        "scorecard metadata",
    )?;
    read_repo_relative_string(
        repo_root,
        &bundle_repo_relative_path(bundle_root, "trust_matrix.md"),
    )
    .map_err(|err| PipelineHandoffValidationFailure {
        classification: PipelineHandoffValidationFailureClassification::MissingOrCorruptProvenance,
        summary: format!(
            "trust matrix is missing or unreadable: {}",
            format_repo_file_access_error(&err)
        ),
    })?;

    let allow_read_paths = read_allowlist
        .allow_read_paths
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    for required in required_bundle_read_paths(&manifest) {
        if !allow_read_paths.contains(&required) {
            return Err(PipelineHandoffValidationFailure {
                classification:
                    PipelineHandoffValidationFailureClassification::MissingOrCorruptProvenance,
                summary: format!("read allowlist is missing required bundle path `{required}`"),
            });
        }
    }

    for input in &manifest.inputs {
        let expected_trust_class =
            expected_trust_class_for_source(&input.source_path).map_err(|summary| {
                PipelineHandoffValidationFailure {
                    classification:
                        PipelineHandoffValidationFailureClassification::TrustClassMismatch,
                    summary,
                }
            })?;
        if input.trust_class != expected_trust_class {
            return Err(PipelineHandoffValidationFailure {
                classification: PipelineHandoffValidationFailureClassification::TrustClassMismatch,
                summary: format!(
                    "input `{}` trust class `{}` does not match expected `{}`",
                    input.source_path, input.trust_class, expected_trust_class
                ),
            });
        }
        let expected_prefix = format!("inputs/{}/", input.trust_class.bundle_segment());
        if !input.bundle_path.starts_with(&expected_prefix) {
            return Err(PipelineHandoffValidationFailure {
                classification:
                    PipelineHandoffValidationFailureClassification::TrustClassMismatch,
                summary: format!(
                    "input `{}` bundle path `{}` does not match trust-class prefix `{expected_prefix}`",
                    input.source_path, input.bundle_path
                ),
            });
        }
        if !allow_read_paths.contains(&input.bundle_path) {
            return Err(PipelineHandoffValidationFailure {
                classification:
                    PipelineHandoffValidationFailureClassification::MissingOrCorruptProvenance,
                summary: format!(
                    "read allowlist is missing input bundle path `{}`",
                    input.bundle_path
                ),
            });
        }

        let bundle_file = bundle_repo_relative_path(bundle_root, &input.bundle_path);
        let bundle_sha256 = sha256_repo_relative_file(repo_root, &bundle_file).map_err(|err| {
            PipelineHandoffValidationFailure {
                classification:
                    PipelineHandoffValidationFailureClassification::TamperedDerivedInput,
                summary: format!(
                    "bundle input `{}` is missing or unreadable: {}",
                    input.bundle_path,
                    format_repo_file_access_error(&err)
                ),
            }
        })?;
        if bundle_sha256 != input.sha256 {
            return Err(PipelineHandoffValidationFailure {
                classification:
                    PipelineHandoffValidationFailureClassification::TamperedDerivedInput,
                summary: format!(
                    "bundle input `{}` sha256 `{bundle_sha256}` does not match manifest `{}`",
                    input.bundle_path, input.sha256
                ),
            });
        }
    }

    validate_canonical_provenance(repo_root, &manifest)?;

    Ok(PipelineHandoffValidatedBundle {
        manifest,
        read_allowlist,
    })
}

pub fn render_pipeline_handoff_emit_result(result: &PipelineHandoffEmitResult) -> String {
    let mut out = String::new();
    out.push_str("OUTCOME: EMITTED\n");
    out.push_str(&format!("PIPELINE: {}\n", result.manifest.pipeline_id));
    out.push_str(&format!("CONSUMER: {}\n", result.manifest.consumer_id));
    out.push_str(&format!("FEATURE ID: {}\n", result.manifest.feature_id));
    out.push_str(&format!("BUNDLE ROOT: {}\n", result.bundle_root));
    out.push_str("WRITTEN FILES:\n");
    for path in &result.written_files {
        out.push_str(&format!("  - {path}\n"));
    }
    out.push_str("REPO REREAD FALLBACK: disabled");
    out
}

pub fn render_pipeline_handoff_refusal(refusal: &PipelineHandoffRefusal) -> String {
    let mut out = String::new();
    out.push_str("OUTCOME: REFUSED\n");
    out.push_str(&format!(
        "PIPELINE: {}\n",
        refusal.pipeline_id.as_deref().unwrap_or("<unknown>")
    ));
    out.push_str(&format!(
        "CONSUMER: {}\n",
        refusal.consumer_id.as_deref().unwrap_or("<unknown>")
    ));
    out.push_str(&format!(
        "REASON: {}: {}\n",
        render_pipeline_handoff_refusal_classification(refusal.classification),
        refusal.summary.trim()
    ));
    out.push_str("NEXT SAFE ACTION: ");
    out.push_str(refusal.recovery.trim());
    out
}

fn render_pipeline_handoff_refusal_classification(
    classification: PipelineHandoffRefusalClassification,
) -> &'static str {
    match classification {
        PipelineHandoffRefusalClassification::UnsupportedTarget => "unsupported_target",
        PipelineHandoffRefusalClassification::InvalidState => "invalid_state",
        PipelineHandoffRefusalClassification::MissingRequiredInput => "missing_required_input",
        PipelineHandoffRefusalClassification::InvalidProvenance => "invalid_provenance",
        PipelineHandoffRefusalClassification::WriteFailure => "write_failure",
    }
}

fn load_supported_handoff_target_registry_for_emit(
    repo_root: &Path,
) -> Result<SupportedTargetRegistry, PipelineHandoffRefusal> {
    SupportedTargetRegistry::load(repo_root).map_err(|err| PipelineHandoffRefusal {
        classification: PipelineHandoffRefusalClassification::InvalidState,
        summary: format!("failed to load supported target registry: {err}"),
        pipeline_id: None,
        consumer_id: None,
        recovery: "fix the pipeline/stage definitions and retry `pipeline handoff emit`"
            .to_string(),
    })
}

fn load_supported_handoff_target_registry_for_validation(
    repo_root: &Path,
) -> Result<SupportedTargetRegistry, PipelineHandoffValidationFailure> {
    SupportedTargetRegistry::load(repo_root).map_err(|err| PipelineHandoffValidationFailure {
        classification: PipelineHandoffValidationFailureClassification::MissingOrCorruptProvenance,
        summary: format!("failed to load supported target registry: {err}"),
    })
}

fn supported_handoff_target(registry: &SupportedTargetRegistry) -> SupportedHandoffTarget {
    let compile_target = registry.compile_target();
    let pipeline_id = compile_target.pipeline.id.clone();
    let stage_id = compile_target.stage.id.clone();
    let consumer_id = registry
        .consumers()
        .find(|consumer| registry.supports_handoff_target(&pipeline_id, &stage_id, &consumer.id))
        .expect("supported target registry must include one handoff consumer")
        .id
        .clone();

    SupportedHandoffTarget {
        pipeline_id,
        stage_id,
        consumer_id,
    }
}

fn validate_supported_consumer(
    supported_target: &SupportedHandoffTarget,
    consumer_selector: &str,
) -> Result<(), PipelineHandoffRefusal> {
    let consumer_selector = consumer_selector.trim();
    if consumer_selector == supported_target.consumer_id {
        Ok(())
    } else {
        Err(PipelineHandoffRefusal {
            classification: PipelineHandoffRefusalClassification::UnsupportedTarget,
            summary: format!(
                "M5 handoff emit currently supports only consumer `{}`",
                supported_target.consumer_id
            ),
            pipeline_id: Some(supported_target.pipeline_id.clone()),
            consumer_id: Some(consumer_selector.to_string()),
            recovery: format!(
                "retry with `handbook pipeline handoff emit --id {} --consumer {}`",
                supported_target.pipeline_id, supported_target.consumer_id
            ),
        })
    }
}

fn validate_supported_compile_target(
    registry: &SupportedTargetRegistry,
    supported_target: &SupportedHandoffTarget,
    pipeline_id: &str,
    stage_id: &str,
    consumer_id: &str,
) -> Result<(), PipelineHandoffRefusal> {
    if registry.supports_handoff_target(pipeline_id, stage_id, consumer_id) {
        Ok(())
    } else {
        Err(PipelineHandoffRefusal {
            classification: PipelineHandoffRefusalClassification::UnsupportedTarget,
            summary: format!(
                "M5 handoff emit currently supports only `{}` -> `{}`",
                supported_target.pipeline_id, supported_target.consumer_id
            ),
            pipeline_id: Some(pipeline_id.to_string()),
            consumer_id: Some(consumer_id.to_string()),
            recovery: format!(
                "retry with `handbook pipeline handoff emit --id {} --consumer {}`",
                supported_target.pipeline_id, supported_target.consumer_id
            ),
        })
    }
}

fn validate_supported_manifest_target(
    registry: &SupportedTargetRegistry,
    manifest: &PipelineHandoffManifest,
) -> Result<(), PipelineHandoffValidationFailure> {
    let supported_target = supported_handoff_target(registry);
    if manifest.schema_version != HANDOFF_SCHEMA_VERSION {
        return Err(PipelineHandoffValidationFailure {
            classification:
                PipelineHandoffValidationFailureClassification::MissingOrCorruptProvenance,
            summary: format!(
                "handoff manifest schema_version `{}` does not match expected `{}`",
                manifest.schema_version, HANDOFF_SCHEMA_VERSION
            ),
        });
    }
    if manifest.pipeline_id != supported_target.pipeline_id
        || manifest.consumer_id != supported_target.consumer_id
    {
        return Err(PipelineHandoffValidationFailure {
            classification: PipelineHandoffValidationFailureClassification::UnsupportedTarget,
            summary: format!(
                "handoff bundle targets unsupported pipeline `{}` or consumer `{}`",
                manifest.pipeline_id, manifest.consumer_id
            ),
        });
    }
    if !registry.supports_handoff_target(
        &manifest.pipeline_id,
        &manifest.feature_spec_compile.stage_id,
        &manifest.consumer_id,
    ) {
        return Err(PipelineHandoffValidationFailure {
            classification: PipelineHandoffValidationFailureClassification::UnsupportedTarget,
            summary: format!(
                "handoff bundle targets unsupported stage `{}`",
                manifest.feature_spec_compile.stage_id
            ),
        });
    }
    if manifest.fallback.repo_reread_allowed || manifest.fallback.mode != "none" {
        return Err(PipelineHandoffValidationFailure {
            classification:
                PipelineHandoffValidationFailureClassification::MissingOrCorruptProvenance,
            summary: "handoff fallback metadata must declare bundle-only happy-path reads"
                .to_string(),
        });
    }
    Ok(())
}

fn build_input_copy_plans(
    compile_result: &PipelineCompileResult,
    feature_spec_body: &str,
) -> Result<Vec<InputCopyPlan>, PipelineHandoffRefusal> {
    let mut plans = Vec::new();
    for document in &compile_result.documents {
        if document.status != PipelineCompileDocumentStatus::Present {
            continue;
        }
        let Some(content) = document.content.as_ref() else {
            continue;
        };
        let trust_class = trust_class_for_compile_document(document).map_err(|summary| {
            PipelineHandoffRefusal {
                classification: PipelineHandoffRefusalClassification::InvalidProvenance,
                summary,
                pipeline_id: Some(compile_result.target.pipeline_id.clone()),
                consumer_id: Some(SUPPORTED_CONSUMER_ID.to_string()),
                recovery: "repair compile input provenance and retry `pipeline handoff emit`"
                    .to_string(),
            }
        })?;
        let bundle_bytes = content.as_bytes().to_vec();
        plans.push(InputCopyPlan {
            source_path: document.path.clone(),
            bundle_path: format!("inputs/{}/{}", trust_class.bundle_segment(), document.path),
            trust_class,
            sha256: sha256_hex(&bundle_bytes),
            bytes: bundle_bytes,
        });
    }

    let feature_spec_bytes = feature_spec_body.as_bytes().to_vec();
    plans.push(InputCopyPlan {
        source_path: FEATURE_SPEC_ARTIFACT_PATH.to_string(),
        bundle_path: format!(
            "inputs/{}/{}",
            PipelineHandoffTrustClass::ExternalManualDerived.bundle_segment(),
            FEATURE_SPEC_ARTIFACT_PATH
        ),
        trust_class: PipelineHandoffTrustClass::ExternalManualDerived,
        sha256: sha256_hex(&feature_spec_bytes),
        bytes: feature_spec_bytes,
    });
    Ok(plans)
}

fn trust_class_for_compile_document(
    document: &PipelineCompileDocument,
) -> Result<PipelineHandoffTrustClass, String> {
    let expected = match document.kind {
        PipelineCompileDocumentKind::Runner
        | PipelineCompileDocumentKind::Profile
        | PipelineCompileDocumentKind::Include
        | PipelineCompileDocumentKind::Library => PipelineHandoffTrustClass::Canonical,
        PipelineCompileDocumentKind::Artifact => expected_trust_class_for_source(&document.path)?,
    };
    Ok(expected)
}

fn expected_trust_class_for_source(source_path: &str) -> Result<PipelineHandoffTrustClass, String> {
    let source_path = source_path.trim();
    if is_canonical_declarative_path(source_path) {
        return Ok(PipelineHandoffTrustClass::Canonical);
    }
    if source_path == FEATURE_SPEC_ARTIFACT_PATH {
        return Ok(PipelineHandoffTrustClass::ExternalManualDerived);
    }
    if source_path.starts_with("artifacts/") {
        return Ok(PipelineHandoffTrustClass::CompilerDerived);
    }

    Err(format!(
        "source path `{source_path}` is outside the supported M5 trust model; canonical declarative inputs must live under `{DECLARATIVE_ROOT}/`"
    ))
}

fn build_allow_read_paths(input_plans: &[InputCopyPlan]) -> Vec<String> {
    let mut paths = BTreeSet::new();
    paths.insert("handoff_manifest.json".to_string());
    paths.insert("trust_matrix.md".to_string());
    paths.insert("read_allowlist.json".to_string());
    paths.insert("scorecard/metadata.json".to_string());
    for input in input_plans {
        paths.insert(input.bundle_path.clone());
    }
    paths.into_iter().collect()
}

fn required_bundle_read_paths(manifest: &PipelineHandoffManifest) -> Vec<String> {
    let mut paths = vec![
        "handoff_manifest.json".to_string(),
        "trust_matrix.md".to_string(),
        "read_allowlist.json".to_string(),
        "scorecard/metadata.json".to_string(),
    ];
    for input in &manifest.inputs {
        paths.push(input.bundle_path.clone());
    }
    paths
}

fn validate_canonical_provenance(
    repo_root: &Path,
    manifest: &PipelineHandoffManifest,
) -> Result<(), PipelineHandoffValidationFailure> {
    let current_manifest = ArtifactManifest::generate(repo_root, ManifestInputs::default())
        .map_err(|err| PipelineHandoffValidationFailure {
            classification:
                PipelineHandoffValidationFailureClassification::StaleCanonicalProvenance,
            summary: format!("failed to recompute canonical artifact provenance: {err}"),
        })?;
    if current_manifest.freshness.fingerprint_sha256
        != manifest.canonical_provenance.manifest_fingerprint_sha256
    {
        return Err(PipelineHandoffValidationFailure {
            classification:
                PipelineHandoffValidationFailureClassification::StaleCanonicalProvenance,
            summary: format!(
                "canonical manifest fingerprint `{}` no longer matches bundle `{}`",
                current_manifest.freshness.fingerprint_sha256,
                manifest.canonical_provenance.manifest_fingerprint_sha256
            ),
        });
    }

    let current_artifacts = current_manifest
        .artifacts
        .into_iter()
        .filter_map(|artifact| {
            artifact
                .content_sha256
                .map(|sha256| (artifact.relative_path.to_string(), sha256))
        })
        .collect::<BTreeMap<_, _>>();
    for expected in &manifest.canonical_provenance.artifact_fingerprints {
        let Some(current_sha256) = current_artifacts.get(&expected.path) else {
            return Err(PipelineHandoffValidationFailure {
                classification:
                    PipelineHandoffValidationFailureClassification::StaleCanonicalProvenance,
                summary: format!(
                    "canonical artifact `{}` is missing from the current repo provenance",
                    expected.path
                ),
            });
        };
        if current_sha256 != &expected.sha256 {
            return Err(PipelineHandoffValidationFailure {
                classification:
                    PipelineHandoffValidationFailureClassification::StaleCanonicalProvenance,
                summary: format!(
                    "canonical artifact `{}` sha256 `{current_sha256}` does not match bundle `{}`",
                    expected.path, expected.sha256
                ),
            });
        }
    }

    let pipeline =
        load_selected_pipeline_definition(repo_root, &manifest.pipeline_id).map_err(|err| {
            PipelineHandoffValidationFailure {
                classification:
                    PipelineHandoffValidationFailureClassification::StaleCanonicalProvenance,
                summary: format!(
                    "failed to reload selected pipeline definition during validation: {err}"
                ),
            }
        })?;
    let supported_variables = supported_route_state_variables(&pipeline);
    let state = load_route_state_with_supported_variables(
        repo_root,
        &manifest.pipeline_id,
        &supported_variables,
    )
    .map_err(|err| PipelineHandoffValidationFailure {
        classification: PipelineHandoffValidationFailureClassification::StaleCanonicalProvenance,
        summary: format!(
            "failed to reload route state during validation: {}",
            format_route_state_read_error(&err)
        ),
    })?;
    let current_basis =
        rebuild_canonical_route_basis(repo_root, &pipeline, &state).map_err(|reason| {
            PipelineHandoffValidationFailure {
                classification:
                    PipelineHandoffValidationFailureClassification::StaleCanonicalProvenance,
                summary: format!(
                    "failed to rebuild canonical route basis during validation: {reason}"
                ),
            }
        })?;
    let current_route_basis_fingerprint =
        route_basis_fingerprint_sha256(&current_basis).map_err(|reason| {
            PipelineHandoffValidationFailure {
                classification:
                    PipelineHandoffValidationFailureClassification::StaleCanonicalProvenance,
                summary: format!("failed to fingerprint current route basis: {reason}"),
            }
        })?;
    if current_basis.state_revision != manifest.route_basis.state_revision
        || current_route_basis_fingerprint != manifest.route_basis.fingerprint_sha256
    {
        return Err(PipelineHandoffValidationFailure {
            classification:
                PipelineHandoffValidationFailureClassification::StaleCanonicalProvenance,
            summary: format!(
                "route basis revision/hash {}/{} no longer match bundle {}/{}",
                current_basis.state_revision,
                current_route_basis_fingerprint,
                manifest.route_basis.state_revision,
                manifest.route_basis.fingerprint_sha256
            ),
        });
    }

    Ok(())
}

fn render_trust_matrix(manifest: &PipelineHandoffManifest) -> String {
    let mut out = String::new();
    out.push_str("# Trust Matrix\n\n");
    out.push_str("| Source Path | Bundle Path | Trust Class | sha256 |\n");
    out.push_str("| --- | --- | --- | --- |\n");
    for input in &manifest.inputs {
        out.push_str(&format!(
            "| `{}` | `{}` | `{}` | `{}` |\n",
            input.source_path, input.bundle_path, input.trust_class, input.sha256
        ));
    }
    out.push_str("\n## Canonical Provenance\n");
    out.push_str(&format!(
        "- canonical manifest fingerprint: `{}`\n",
        manifest.canonical_provenance.manifest_fingerprint_sha256
    ));
    out.push_str(&format!(
        "- route-basis revision/hash: `{}` / `{}`\n",
        manifest.route_basis.state_revision, manifest.route_basis.fingerprint_sha256
    ));
    out.push_str("- repo reread fallback: disabled\n");
    out
}

fn write_bundle_file(
    repo_root: &Path,
    repo_relative_path: &str,
    bytes: &[u8],
    manifest: &PipelineHandoffManifest,
) -> Result<(), PipelineHandoffRefusal> {
    write_repo_relative_bytes(repo_root, repo_relative_path, bytes).map_err(|err| {
        PipelineHandoffRefusal {
            classification: PipelineHandoffRefusalClassification::WriteFailure,
            summary: format!(
                "failed to write handoff bundle file `{repo_relative_path}`: {}",
                format_repo_mutation_error(&err)
            ),
            pipeline_id: Some(manifest.pipeline_id.clone()),
            consumer_id: Some(manifest.consumer_id.clone()),
            recovery: "repair the bundle target path and retry `pipeline handoff emit`".to_string(),
        }
    })
}

fn read_bundle_json<T: for<'de> Deserialize<'de>>(
    repo_root: &Path,
    relative_path: &str,
    label: &str,
) -> Result<T, PipelineHandoffValidationFailure> {
    let body = read_repo_relative_string(repo_root, relative_path).map_err(|err| {
        PipelineHandoffValidationFailure {
            classification:
                PipelineHandoffValidationFailureClassification::MissingOrCorruptProvenance,
            summary: format!(
                "{label} is missing or unreadable at `{relative_path}`: {}",
                format_repo_file_access_error(&err)
            ),
        }
    })?;
    serde_json::from_str(&body).map_err(|err| PipelineHandoffValidationFailure {
        classification: PipelineHandoffValidationFailureClassification::MissingOrCorruptProvenance,
        summary: format!("{label} at `{relative_path}` is not valid JSON: {err}"),
    })
}

fn bundle_repo_relative_path(bundle_root: &str, bundle_relative_path: &str) -> String {
    format!(
        "{}/{}",
        bundle_root.trim_end_matches('/'),
        bundle_relative_path.trim_start_matches('/')
    )
}

fn normalize_bundle_root_from_metadata(
    label: &str,
    bundle_root: &str,
) -> Result<String, PipelineHandoffValidationFailure> {
    normalize_repo_relative_path(bundle_root).map_err(|reason| PipelineHandoffValidationFailure {
        classification: PipelineHandoffValidationFailureClassification::MissingOrCorruptProvenance,
        summary: format!("{label} bundle_root `{bundle_root}` is invalid: {reason}"),
    })
}

fn stage_10_provenance_recovery() -> String {
    format!(
        "recapture `{SUPPORTED_STAGE_ID}` from the current compile payload before retrying `pipeline handoff emit`"
    )
}

fn derive_feature_id(feature_spec_body: &str, feature_spec_sha256: &str) -> String {
    if let Some(spec_id) = feature_spec_body
        .lines()
        .map(str::trim)
        .find_map(|line| line.strip_prefix("- Spec ID:"))
        .map(str::trim)
        .filter(|value| !value.is_empty() && !value.contains('{') && !value.contains('}'))
    {
        let slug = slugify(spec_id);
        if !slug.is_empty() {
            return slug;
        }
    }

    if let Some(heading) = feature_spec_body
        .lines()
        .map(str::trim)
        .find(|line| line.starts_with("# "))
    {
        let title = heading.trim_start_matches("# ").trim();
        let normalized = title
            .trim_end_matches("— Feature Specification")
            .trim_end_matches("- Feature Specification")
            .trim();
        let slug = slugify(normalized);
        if !slug.is_empty() {
            return slug;
        }
    }

    format!("feature-{}", &feature_spec_sha256[..12])
}

fn slugify(value: &str) -> String {
    let mut slug = String::new();
    let mut last_was_dash = false;
    for ch in value.chars() {
        let lower = ch.to_ascii_lowercase();
        if lower.is_ascii_alphanumeric() {
            slug.push(lower);
            last_was_dash = false;
        } else if !slug.is_empty() && !last_was_dash {
            slug.push('-');
            last_was_dash = true;
        }
    }
    slug.trim_matches('-').to_string()
}

fn map_compile_refusal(
    refusal: PipelineCompileRefusal,
    consumer_id: &str,
) -> PipelineHandoffRefusal {
    let classification = match refusal.classification {
        PipelineCompileRefusalClassification::UnsupportedTarget => {
            PipelineHandoffRefusalClassification::UnsupportedTarget
        }
        PipelineCompileRefusalClassification::MissingRequiredInput
        | PipelineCompileRefusalClassification::EmptyRequiredInput => {
            PipelineHandoffRefusalClassification::MissingRequiredInput
        }
        PipelineCompileRefusalClassification::InvalidDefinition
        | PipelineCompileRefusalClassification::InvalidState
        | PipelineCompileRefusalClassification::MissingRouteBasis
        | PipelineCompileRefusalClassification::MalformedRouteBasis
        | PipelineCompileRefusalClassification::StaleRouteBasis
        | PipelineCompileRefusalClassification::InactiveStage => {
            PipelineHandoffRefusalClassification::InvalidState
        }
    };
    PipelineHandoffRefusal {
        classification,
        summary: refusal.summary,
        pipeline_id: refusal.pipeline_id,
        consumer_id: Some(consumer_id.to_string()),
        recovery: refusal.recovery,
    }
}

fn format_repo_file_access_error(err: &RepoRelativeFileAccessError) -> String {
    match err {
        RepoRelativeFileAccessError::Missing(path) => format!("missing {}", path.display()),
        RepoRelativeFileAccessError::InvalidPath(reason) => reason.clone(),
        RepoRelativeFileAccessError::SymlinkNotAllowed(path) => {
            format!("symlink not allowed: {}", path.display())
        }
        RepoRelativeFileAccessError::NotRegularFile(path) => {
            format!("not a regular file: {}", path.display())
        }
        RepoRelativeFileAccessError::ReadFailure { path, source } => {
            format!("{} ({source})", path.display())
        }
    }
}

fn format_repo_mutation_error(err: &RepoRelativeMutationError) -> String {
    match err {
        RepoRelativeMutationError::InvalidPath(reason) => reason.clone(),
        RepoRelativeMutationError::ParentNotDirectory(path) => {
            format!("parent is not a directory: {}", path.display())
        }
        RepoRelativeMutationError::NotRegularFile(path) => {
            format!("not a regular file target: {}", path.display())
        }
        RepoRelativeMutationError::SymlinkNotAllowed(path) => {
            format!("symlink not allowed: {}", path.display())
        }
        RepoRelativeMutationError::ReadFailure { path, source } => {
            format!("{} ({source})", path.display())
        }
        RepoRelativeMutationError::WriteFailure { path, source } => {
            format!("{} ({source})", path.display())
        }
    }
}

fn format_route_state_read_error(err: &RouteStateReadError) -> String {
    match err {
        RouteStateReadError::InvalidPipelineId {
            pipeline_id,
            reason,
        } => {
            format!("invalid pipeline id `{pipeline_id}`: {reason}")
        }
        RouteStateReadError::ReadFailure { path, source } => {
            format!("{} ({source})", path.display())
        }
        RouteStateReadError::MalformedState { path, reason } => {
            format!("{} ({reason})", path.display())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        derive_feature_id, expected_trust_class_for_source, slugify, PipelineHandoffTrustClass,
    };

    #[test]
    fn derive_feature_id_prefers_spec_id_when_present() {
        let feature_spec =
            "# Ignore Me — Feature Specification\n- Spec ID: FEAT-127 Primary Journey\n";
        assert_eq!(
            derive_feature_id(
                feature_spec,
                "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
            ),
            "feat-127-primary-journey"
        );
    }

    #[test]
    fn derive_feature_id_falls_back_to_heading_slug() {
        let feature_spec = "# Pipeline Foundation Journey — Feature Specification\n";
        assert_eq!(
            derive_feature_id(
                feature_spec,
                "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
            ),
            "pipeline-foundation-journey"
        );
    }

    #[test]
    fn derive_feature_id_uses_hash_when_heading_is_not_slugifiable() {
        let feature_spec = "# !!!\n";
        assert_eq!(
            derive_feature_id(
                feature_spec,
                "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
            ),
            "feature-0123456789ab"
        );
    }

    #[test]
    fn trust_classes_match_supported_sources() {
        assert_eq!(
            expected_trust_class_for_source("core/library/feature_spec/FEATURE_SPEC.md.tmpl")
                .unwrap(),
            PipelineHandoffTrustClass::Canonical
        );
        assert_eq!(
            expected_trust_class_for_source("artifacts/base/BASE_CONTEXT.md").unwrap(),
            PipelineHandoffTrustClass::CompilerDerived
        );
        assert_eq!(
            expected_trust_class_for_source("artifacts/feature_spec/FEATURE_SPEC.md").unwrap(),
            PipelineHandoffTrustClass::ExternalManualDerived
        );
    }

    #[test]
    fn slugify_collapses_non_alphanumeric_boundaries() {
        assert_eq!(slugify("Feature / Slice: Alpha"), "feature-slice-alpha");
    }
}
