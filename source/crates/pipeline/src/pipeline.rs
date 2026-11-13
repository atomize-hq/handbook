use crate::declarative_roots::{handbook_product_declarative_roots, pipeline_root, stage_root};
use crate::repo_file_access::{
    CompilerWorkspace, NormalizedRepoRelativePath, RepoRelativeFileAccessError,
};
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, OnceLock};

use crate::declarative_roots::PipelineDeclarativeRootsContract;

const SUPPORTED_CONSUMER_TARGET_ID: &str = "feature-slice-decomposer";
const SUPPORTED_BASE_STAGE_FILE_NAME: &str = "00_base.md";
const SUPPORTED_COMPILE_STAGE_FILE_NAME: &str = "10_feature_spec.md";
const SUPPORTED_CAPTURE_STAGE_FILE_NAMES: &[&str] = &[
    "04_charter_inputs.md",
    "05_charter_synthesize.md",
    "06_project_context_interview.md",
    "07_foundation_pack.md",
    SUPPORTED_COMPILE_STAGE_FILE_NAME,
];
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineDefinition {
    pub source_path: PathBuf,
    pub header: PipelineHeader,
    pub body: PipelineBody,
}

impl PipelineDefinition {
    /// Returns the declared stage list in source order.
    ///
    /// Downstream routing code should consume this typed slice instead of
    /// reparsing the pipeline YAML or inferring order from the filesystem.
    pub fn declared_stages(&self) -> &[PipelineStage] {
        &self.body.stages
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineCatalog {
    pipelines: BTreeMap<String, PipelineCatalogEntry>,
    stages: BTreeMap<String, StageCatalogEntry>,
}

impl PipelineCatalog {
    pub fn pipelines(&self) -> impl Iterator<Item = &PipelineCatalogEntry> {
        self.pipelines.values()
    }

    pub fn stages(&self) -> impl Iterator<Item = &StageCatalogEntry> {
        self.stages.values()
    }

    pub fn pipeline_count(&self) -> usize {
        self.pipelines.len()
    }

    pub fn stage_count(&self) -> usize {
        self.stages.len()
    }

    pub fn resolve_selector(
        &self,
        selector: &str,
    ) -> Result<PipelineSelection, PipelineLookupError> {
        resolve_pipeline_selector(self, selector)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineCatalogEntry {
    pub definition: PipelineDefinition,
    pub stages: Vec<PipelineCatalogStageEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineCatalogStageEntry {
    pub stage_id: String,
    pub title: String,
    pub work_level: Option<String>,
    pub source_path: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StageCatalogEntry {
    pub id: String,
    pub kind: String,
    pub version: String,
    pub title: String,
    pub description: String,
    pub work_level: Option<String>,
    pub source_path: PathBuf,
    pub pipelines: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SupportedPipelineTarget {
    pub id: String,
    pub declared_stage_ids: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SupportedStageTarget {
    pub id: String,
    pub source_path: PathBuf,
    pub pipeline_ids: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SupportedConsumerTarget {
    pub id: String,
    pub allowed_pipeline_ids: Vec<String>,
    pub allowed_stage_ids: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SupportedCompileTarget {
    pub pipeline: SupportedPipelineTarget,
    pub stage: SupportedStageTarget,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SupportedHandoffTarget {
    pub pipeline_id: String,
    pub stage_id: String,
    pub consumer_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SupportedTargetRegistry {
    pipelines: BTreeMap<String, SupportedPipelineTarget>,
    stages: BTreeMap<String, SupportedStageTarget>,
    consumers: BTreeMap<String, SupportedConsumerTarget>,
    compile_pairs: BTreeSet<(String, String)>,
    capture_pairs: BTreeSet<(String, String)>,
    provenance_pairs: BTreeSet<(String, String)>,
    handoff_pairs: BTreeSet<(String, String, String)>,
}

impl SupportedTargetRegistry {
    #[allow(dead_code)]
    pub(crate) fn load(
        repo_root: impl AsRef<Path>,
    ) -> Result<Self, SupportedTargetRegistryLoadError> {
        let repo_root = repo_root.as_ref();
        let catalog = load_pipeline_catalog_metadata(repo_root)
            .map_err(SupportedTargetRegistryLoadError::Catalog)?;
        let supported_topology = resolve_supported_target_topology(&catalog)?;
        let supported_pipeline = supported_topology.pipeline;

        let mut stages = BTreeMap::new();
        for stage_id in &supported_topology.capture_stage_ids {
            let stage = catalog.stages.get(stage_id).ok_or_else(|| {
                SupportedTargetRegistryLoadError::MissingSupportedStage {
                    stage_id: stage_id.clone(),
                }
            })?;
            if !stage
                .pipelines
                .iter()
                .any(|pipeline_id| pipeline_id == &supported_pipeline.id)
            {
                return Err(
                    SupportedTargetRegistryLoadError::UnsupportedStagePipelineMembership {
                        pipeline_id: supported_pipeline.id.clone(),
                        stage_id: stage_id.clone(),
                    },
                );
            }
            stages.insert(
                stage.id.clone(),
                SupportedStageTarget {
                    id: stage.id.clone(),
                    source_path: stage.source_path.clone(),
                    pipeline_ids: stage.pipelines.clone(),
                },
            );
        }

        let pipeline_id = supported_pipeline.id.clone();
        let compile_stage_id = supported_topology.compile_stage_id;
        let mut pipelines = BTreeMap::new();
        pipelines.insert(pipeline_id.clone(), supported_pipeline);

        let mut consumers = BTreeMap::new();
        consumers.insert(
            SUPPORTED_CONSUMER_TARGET_ID.to_string(),
            SupportedConsumerTarget {
                id: SUPPORTED_CONSUMER_TARGET_ID.to_string(),
                allowed_pipeline_ids: vec![pipeline_id.clone()],
                allowed_stage_ids: vec![compile_stage_id.clone()],
            },
        );

        let compile_pairs = BTreeSet::from([(pipeline_id.clone(), compile_stage_id.clone())]);
        let capture_pairs = supported_topology
            .capture_stage_ids
            .iter()
            .map(|stage_id| (pipeline_id.clone(), stage_id.clone()))
            .collect();
        let provenance_pairs = compile_pairs.clone();
        let handoff_pairs = BTreeSet::from([(
            pipeline_id,
            compile_stage_id,
            SUPPORTED_CONSUMER_TARGET_ID.to_string(),
        )]);

        Ok(Self {
            pipelines,
            stages,
            consumers,
            compile_pairs,
            capture_pairs,
            provenance_pairs,
            handoff_pairs,
        })
    }

    #[allow(dead_code)]
    pub(crate) fn load_with_roots(
        repo_root: impl AsRef<Path>,
        roots: &PipelineDeclarativeRootsContract,
    ) -> Result<Self, SupportedTargetRegistryLoadError> {
        let repo_root = repo_root.as_ref();
        let catalog = load_pipeline_catalog_metadata_with_roots(repo_root, roots)
            .map_err(SupportedTargetRegistryLoadError::Catalog)?;
        let supported_topology = resolve_supported_target_topology_with_roots(&catalog, roots)?;
        let supported_pipeline = supported_topology.pipeline;

        let mut stages = BTreeMap::new();
        for stage_id in &supported_topology.capture_stage_ids {
            let stage = catalog.stages.get(stage_id).ok_or_else(|| {
                SupportedTargetRegistryLoadError::MissingSupportedStage {
                    stage_id: stage_id.clone(),
                }
            })?;
            if !stage
                .pipelines
                .iter()
                .any(|pipeline_id| pipeline_id == &supported_pipeline.id)
            {
                return Err(
                    SupportedTargetRegistryLoadError::UnsupportedStagePipelineMembership {
                        pipeline_id: supported_pipeline.id.clone(),
                        stage_id: stage_id.clone(),
                    },
                );
            }
            stages.insert(
                stage.id.clone(),
                SupportedStageTarget {
                    id: stage.id.clone(),
                    source_path: stage.source_path.clone(),
                    pipeline_ids: stage.pipelines.clone(),
                },
            );
        }

        let pipeline_id = supported_pipeline.id.clone();
        let compile_stage_id = supported_topology.compile_stage_id;
        let mut pipelines = BTreeMap::new();
        pipelines.insert(pipeline_id.clone(), supported_pipeline);

        let mut consumers = BTreeMap::new();
        consumers.insert(
            SUPPORTED_CONSUMER_TARGET_ID.to_string(),
            SupportedConsumerTarget {
                id: SUPPORTED_CONSUMER_TARGET_ID.to_string(),
                allowed_pipeline_ids: vec![pipeline_id.clone()],
                allowed_stage_ids: vec![compile_stage_id.clone()],
            },
        );

        let compile_pairs = BTreeSet::from([(pipeline_id.clone(), compile_stage_id.clone())]);
        let capture_pairs = supported_topology
            .capture_stage_ids
            .iter()
            .map(|stage_id| (pipeline_id.clone(), stage_id.clone()))
            .collect();
        let provenance_pairs = compile_pairs.clone();
        let handoff_pairs = BTreeSet::from([(
            pipeline_id,
            compile_stage_id,
            SUPPORTED_CONSUMER_TARGET_ID.to_string(),
        )]);

        Ok(Self {
            pipelines,
            stages,
            consumers,
            compile_pairs,
            capture_pairs,
            provenance_pairs,
            handoff_pairs,
        })
    }

    pub fn canonical_compile_pipeline_id(&self) -> &str {
        self.compile_pairs
            .iter()
            .next()
            .map(|(pipeline_id, _)| pipeline_id.as_str())
            .expect("supported target registry must include one compile pipeline")
    }

    pub fn canonical_compile_stage_id(&self) -> &str {
        self.compile_pairs
            .iter()
            .next()
            .map(|(_, stage_id)| stage_id.as_str())
            .expect("supported target registry must include one compile stage")
    }

    pub fn pipelines(&self) -> impl Iterator<Item = &SupportedPipelineTarget> {
        self.pipelines.values()
    }

    pub fn stages(&self) -> impl Iterator<Item = &SupportedStageTarget> {
        self.stages.values()
    }

    pub fn consumers(&self) -> impl Iterator<Item = &SupportedConsumerTarget> {
        self.consumers.values()
    }

    pub fn compile_target(&self) -> SupportedCompileTarget {
        let (pipeline_id, stage_id) = self
            .compile_pairs
            .iter()
            .next()
            .expect("supported target registry must include one compile target");
        SupportedCompileTarget {
            pipeline: self
                .pipelines
                .get(pipeline_id)
                .expect("supported target registry must include the compile pipeline")
                .clone(),
            stage: self
                .stages
                .get(stage_id)
                .expect("supported target registry must include the compile stage")
                .clone(),
        }
    }

    pub fn handoff_target(&self) -> SupportedHandoffTarget {
        let (pipeline_id, stage_id, consumer_id) = self
            .handoff_pairs
            .iter()
            .next()
            .expect("supported target registry must include one handoff target");
        SupportedHandoffTarget {
            pipeline_id: pipeline_id.clone(),
            stage_id: stage_id.clone(),
            consumer_id: consumer_id.clone(),
        }
    }

    pub fn resolve_compile_target(
        &self,
        pipeline_id: &str,
        stage_id: &str,
    ) -> Result<SupportedCompileTarget, SupportedTargetResolutionError> {
        let pipeline = self.pipelines.get(pipeline_id).ok_or_else(|| {
            SupportedTargetResolutionError::UnsupportedPipeline {
                pipeline_id: pipeline_id.to_string(),
            }
        })?;
        let stage = self.stages.get(stage_id).ok_or_else(|| {
            SupportedTargetResolutionError::UnsupportedStage {
                stage_id: stage_id.to_string(),
            }
        })?;

        if !self
            .compile_pairs
            .contains(&(pipeline_id.to_string(), stage_id.to_string()))
        {
            return Err(SupportedTargetResolutionError::UnsupportedCompilePairing {
                pipeline_id: pipeline_id.to_string(),
                stage_id: stage_id.to_string(),
            });
        }

        Ok(SupportedCompileTarget {
            pipeline: pipeline.clone(),
            stage: stage.clone(),
        })
    }

    pub fn supports_capture_target(&self, pipeline_id: &str, stage_id: &str) -> bool {
        self.capture_pairs
            .contains(&(pipeline_id.to_string(), stage_id.to_string()))
    }

    pub fn supports_provenance_target(&self, pipeline_id: &str, stage_id: &str) -> bool {
        self.provenance_pairs
            .contains(&(pipeline_id.to_string(), stage_id.to_string()))
    }

    pub fn supports_handoff_target(
        &self,
        pipeline_id: &str,
        stage_id: &str,
        consumer_id: &str,
    ) -> bool {
        self.handoff_pairs.contains(&(
            pipeline_id.to_string(),
            stage_id.to_string(),
            consumer_id.to_string(),
        ))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SupportedTargetTopology {
    pipeline: SupportedPipelineTarget,
    compile_stage_id: String,
    capture_stage_ids: Vec<String>,
}

#[allow(dead_code)]
fn resolve_supported_target_topology(
    catalog: &PipelineCatalog,
) -> Result<SupportedTargetTopology, SupportedTargetRegistryLoadError> {
    let roots = handbook_product_declarative_roots();
    let supported_pipeline_stage_source_paths = supported_pipeline_stage_source_paths(roots);
    let supported_compile_stage_source_path = supported_compile_stage_source_path(roots);
    let supported_base_stage_source_path = supported_base_stage_source_path(roots);
    let supported_capture_stage_source_paths = supported_capture_stage_source_paths(roots);
    let mut pipeline_candidates = catalog
        .pipelines
        .values()
        .filter(|pipeline| {
            supported_pipeline_stage_shape_matches(pipeline, &supported_pipeline_stage_source_paths)
        })
        .collect::<Vec<_>>();

    if pipeline_candidates.is_empty() {
        return Err(
            SupportedTargetRegistryLoadError::MissingCatalogBackedPipelineShape {
                required_stage_source_paths: supported_stage_source_paths(
                    &supported_pipeline_stage_source_paths,
                ),
            },
        );
    }

    if pipeline_candidates.len() > 1 {
        return Err(
            SupportedTargetRegistryLoadError::AmbiguousCatalogBackedPipelineShape {
                pipeline_ids: pipeline_candidates
                    .iter()
                    .map(|pipeline| pipeline.definition.header.id.clone())
                    .collect(),
            },
        );
    }

    let pipeline = pipeline_candidates.remove(0);
    let compile_stage_id = pipeline
        .stages
        .iter()
        .find(|stage| stage_source_path_matches(stage, &supported_compile_stage_source_path))
        .map(|stage| stage.stage_id.clone())
        .ok_or_else(
            || SupportedTargetRegistryLoadError::MissingCatalogBackedCompileStage {
                pipeline_id: pipeline.definition.header.id.clone(),
                required_stage_source_path: supported_compile_stage_source_path
                    .display()
                    .to_string(),
            },
        )?;

    if !pipeline
        .stages
        .iter()
        .any(|stage| stage_source_path_matches(stage, &supported_base_stage_source_path))
    {
        return Err(
            SupportedTargetRegistryLoadError::MissingCatalogBackedBaseStage {
                pipeline_id: pipeline.definition.header.id.clone(),
                required_stage_source_path: supported_base_stage_source_path.display().to_string(),
            },
        );
    }

    let capture_stage_ids = pipeline
        .stages
        .iter()
        .filter(|stage| {
            supported_capture_stage_source_path_matches(
                stage,
                &supported_capture_stage_source_paths,
            )
        })
        .map(|stage| stage.stage_id.clone())
        .collect::<Vec<_>>();
    if capture_stage_ids.len() != supported_capture_stage_source_paths.len() {
        return Err(
            SupportedTargetRegistryLoadError::MissingCatalogBackedCaptureStages {
                pipeline_id: pipeline.definition.header.id.clone(),
                required_stage_source_paths: supported_stage_source_paths(
                    &supported_capture_stage_source_paths,
                ),
            },
        );
    }

    if !capture_stage_ids
        .iter()
        .any(|stage_id| stage_id == &compile_stage_id)
    {
        return Err(SupportedTargetRegistryLoadError::MissingPipelineStage {
            pipeline_id: pipeline.definition.header.id.clone(),
            stage_id: compile_stage_id.clone(),
        });
    }

    Ok(SupportedTargetTopology {
        pipeline: SupportedPipelineTarget {
            id: pipeline.definition.header.id.clone(),
            declared_stage_ids: pipeline
                .stages
                .iter()
                .map(|stage| stage.stage_id.clone())
                .collect(),
        },
        compile_stage_id,
        capture_stage_ids,
    })
}

#[allow(dead_code)]
fn resolve_supported_target_topology_with_roots(
    catalog: &PipelineCatalog,
    roots: &PipelineDeclarativeRootsContract,
) -> Result<SupportedTargetTopology, SupportedTargetRegistryLoadError> {
    let supported_pipeline_stage_source_paths = supported_pipeline_stage_source_paths(roots);
    let supported_compile_stage_source_path = supported_compile_stage_source_path(roots);
    let supported_base_stage_source_path = supported_base_stage_source_path(roots);
    let supported_capture_stage_source_paths = supported_capture_stage_source_paths(roots);
    let mut pipeline_candidates = catalog
        .pipelines
        .values()
        .filter(|pipeline| {
            supported_pipeline_stage_shape_matches(pipeline, &supported_pipeline_stage_source_paths)
        })
        .collect::<Vec<_>>();

    if pipeline_candidates.is_empty() {
        return Err(
            SupportedTargetRegistryLoadError::MissingCatalogBackedPipelineShape {
                required_stage_source_paths: supported_stage_source_paths(
                    &supported_pipeline_stage_source_paths,
                ),
            },
        );
    }

    if pipeline_candidates.len() > 1 {
        return Err(
            SupportedTargetRegistryLoadError::AmbiguousCatalogBackedPipelineShape {
                pipeline_ids: pipeline_candidates
                    .iter()
                    .map(|pipeline| pipeline.definition.header.id.clone())
                    .collect(),
            },
        );
    }

    let pipeline = pipeline_candidates.remove(0);
    let compile_stage_id = pipeline
        .stages
        .iter()
        .find(|stage| stage_source_path_matches(stage, &supported_compile_stage_source_path))
        .map(|stage| stage.stage_id.clone())
        .ok_or_else(
            || SupportedTargetRegistryLoadError::MissingCatalogBackedCompileStage {
                pipeline_id: pipeline.definition.header.id.clone(),
                required_stage_source_path: supported_compile_stage_source_path
                    .display()
                    .to_string(),
            },
        )?;

    if !pipeline
        .stages
        .iter()
        .any(|stage| stage_source_path_matches(stage, &supported_base_stage_source_path))
    {
        return Err(
            SupportedTargetRegistryLoadError::MissingCatalogBackedBaseStage {
                pipeline_id: pipeline.definition.header.id.clone(),
                required_stage_source_path: supported_base_stage_source_path.display().to_string(),
            },
        );
    }

    let capture_stage_ids = pipeline
        .stages
        .iter()
        .filter(|stage| {
            supported_capture_stage_source_path_matches(
                stage,
                &supported_capture_stage_source_paths,
            )
        })
        .map(|stage| stage.stage_id.clone())
        .collect::<Vec<_>>();
    if capture_stage_ids.len() != supported_capture_stage_source_paths.len() {
        return Err(
            SupportedTargetRegistryLoadError::MissingCatalogBackedCaptureStages {
                pipeline_id: pipeline.definition.header.id.clone(),
                required_stage_source_paths: supported_stage_source_paths(
                    &supported_capture_stage_source_paths,
                ),
            },
        );
    }

    if !capture_stage_ids
        .iter()
        .any(|stage_id| stage_id == &compile_stage_id)
    {
        return Err(SupportedTargetRegistryLoadError::MissingPipelineStage {
            pipeline_id: pipeline.definition.header.id.clone(),
            stage_id: compile_stage_id.clone(),
        });
    }

    Ok(SupportedTargetTopology {
        pipeline: SupportedPipelineTarget {
            id: pipeline.definition.header.id.clone(),
            declared_stage_ids: pipeline
                .stages
                .iter()
                .map(|stage| stage.stage_id.clone())
                .collect(),
        },
        compile_stage_id,
        capture_stage_ids,
    })
}

fn supported_pipeline_stage_shape_matches(
    pipeline: &PipelineCatalogEntry,
    expected_stage_source_paths: &[PathBuf],
) -> bool {
    pipeline.stages.len() == expected_stage_source_paths.len()
        && pipeline
            .stages
            .iter()
            .zip(expected_stage_source_paths.iter())
            .all(|(stage, expected)| stage_source_path_matches(stage, expected))
}

fn supported_capture_stage_source_path_matches(
    stage: &PipelineCatalogStageEntry,
    expected_stage_source_paths: &[PathBuf],
) -> bool {
    expected_stage_source_paths
        .iter()
        .any(|expected| stage_source_path_matches(stage, expected))
}

fn stage_source_path_matches(stage: &PipelineCatalogStageEntry, expected: &Path) -> bool {
    stage.source_path == expected
}

fn supported_stage_source_paths(expected_paths: &[PathBuf]) -> Vec<String> {
    expected_paths
        .iter()
        .map(|path| path.display().to_string())
        .collect()
}

fn supported_base_stage_source_path(roots: &PipelineDeclarativeRootsContract) -> PathBuf {
    PathBuf::from(roots.stage_file(SUPPORTED_BASE_STAGE_FILE_NAME))
}

fn supported_compile_stage_source_path(roots: &PipelineDeclarativeRootsContract) -> PathBuf {
    PathBuf::from(roots.stage_file(SUPPORTED_COMPILE_STAGE_FILE_NAME))
}

fn supported_capture_stage_source_paths(roots: &PipelineDeclarativeRootsContract) -> Vec<PathBuf> {
    SUPPORTED_CAPTURE_STAGE_FILE_NAMES
        .iter()
        .map(|file_name| PathBuf::from(roots.stage_file(file_name)))
        .collect()
}

fn supported_pipeline_stage_source_paths(roots: &PipelineDeclarativeRootsContract) -> Vec<PathBuf> {
    std::iter::once(supported_base_stage_source_path(roots))
        .chain(supported_capture_stage_source_paths(roots))
        .collect()
}

#[derive(Debug)]
pub enum SupportedTargetRegistryLoadError {
    Catalog(PipelineCatalogError),
    MissingCatalogBackedPipelineShape {
        required_stage_source_paths: Vec<String>,
    },
    AmbiguousCatalogBackedPipelineShape {
        pipeline_ids: Vec<String>,
    },
    MissingCatalogBackedCompileStage {
        pipeline_id: String,
        required_stage_source_path: String,
    },
    MissingCatalogBackedBaseStage {
        pipeline_id: String,
        required_stage_source_path: String,
    },
    MissingCatalogBackedCaptureStages {
        pipeline_id: String,
        required_stage_source_paths: Vec<String>,
    },
    MissingSupportedPipeline {
        pipeline_id: String,
    },
    MissingPipelineStage {
        pipeline_id: String,
        stage_id: String,
    },
    MissingSupportedStage {
        stage_id: String,
    },
    UnsupportedStagePipelineMembership {
        pipeline_id: String,
        stage_id: String,
    },
}

impl fmt::Display for SupportedTargetRegistryLoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SupportedTargetRegistryLoadError::Catalog(err) => write!(f, "{err}"),
            SupportedTargetRegistryLoadError::MissingCatalogBackedPipelineShape {
                required_stage_source_paths,
            } => write!(
                f,
                "could not derive the approved supported pipeline from declarative catalog truth; expected exact stage sources [{}]",
                required_stage_source_paths.join(", ")
            ),
            SupportedTargetRegistryLoadError::AmbiguousCatalogBackedPipelineShape {
                pipeline_ids,
            } => write!(
                f,
                "multiple pipelines match the approved catalog-backed supported shape: {}",
                pipeline_ids.join(", ")
            ),
            SupportedTargetRegistryLoadError::MissingCatalogBackedCompileStage {
                pipeline_id,
                required_stage_source_path,
            } => write!(
                f,
                "pipeline `{pipeline_id}` does not declare the approved compile stage source `{required_stage_source_path}`"
            ),
            SupportedTargetRegistryLoadError::MissingCatalogBackedBaseStage {
                pipeline_id,
                required_stage_source_path,
            } => write!(
                f,
                "pipeline `{pipeline_id}` does not declare the approved base stage source `{required_stage_source_path}`"
            ),
            SupportedTargetRegistryLoadError::MissingCatalogBackedCaptureStages {
                pipeline_id,
                required_stage_source_paths,
            } => write!(
                f,
                "pipeline `{pipeline_id}` does not declare the approved capture stage sources [{}]",
                required_stage_source_paths.join(", ")
            ),
            SupportedTargetRegistryLoadError::MissingSupportedPipeline { pipeline_id } => write!(
                f,
                "approved supported pipeline `{pipeline_id}` is missing from declarative catalog truth"
            ),
            SupportedTargetRegistryLoadError::MissingPipelineStage {
                pipeline_id,
                stage_id,
            } => write!(
                f,
                "approved supported stage `{stage_id}` is not declared in pipeline `{pipeline_id}`"
            ),
            SupportedTargetRegistryLoadError::MissingSupportedStage { stage_id } => write!(
                f,
                "approved supported stage `{stage_id}` is missing from declarative catalog truth"
            ),
            SupportedTargetRegistryLoadError::UnsupportedStagePipelineMembership {
                pipeline_id,
                stage_id,
            } => write!(
                f,
                "approved supported stage `{stage_id}` is not cataloged as participating in pipeline `{pipeline_id}`"
            ),
        }
    }
}

impl std::error::Error for SupportedTargetRegistryLoadError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            SupportedTargetRegistryLoadError::Catalog(err) => Some(err),
            SupportedTargetRegistryLoadError::MissingCatalogBackedPipelineShape { .. }
            | SupportedTargetRegistryLoadError::AmbiguousCatalogBackedPipelineShape { .. }
            | SupportedTargetRegistryLoadError::MissingCatalogBackedCompileStage { .. }
            | SupportedTargetRegistryLoadError::MissingCatalogBackedBaseStage { .. }
            | SupportedTargetRegistryLoadError::MissingCatalogBackedCaptureStages { .. }
            | SupportedTargetRegistryLoadError::MissingSupportedPipeline { .. }
            | SupportedTargetRegistryLoadError::MissingPipelineStage { .. }
            | SupportedTargetRegistryLoadError::MissingSupportedStage { .. }
            | SupportedTargetRegistryLoadError::UnsupportedStagePipelineMembership { .. } => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SupportedTargetResolutionError {
    UnsupportedPipeline {
        pipeline_id: String,
    },
    UnsupportedStage {
        stage_id: String,
    },
    UnsupportedCompilePairing {
        pipeline_id: String,
        stage_id: String,
    },
}

impl fmt::Display for SupportedTargetResolutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SupportedTargetResolutionError::UnsupportedPipeline { pipeline_id } => {
                write!(f, "unsupported pipeline `{pipeline_id}`")
            }
            SupportedTargetResolutionError::UnsupportedStage { stage_id } => {
                write!(f, "unsupported stage `{stage_id}`")
            }
            SupportedTargetResolutionError::UnsupportedCompilePairing {
                pipeline_id,
                stage_id,
            } => write!(
                f,
                "unsupported compile pairing `{pipeline_id}` + `{stage_id}`"
            ),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompileStageDefinition {
    pub stage: PipelineStage,
    pub source_path: PathBuf,
    pub kind: String,
    pub id: String,
    pub version: String,
    pub title: String,
    pub description: String,
    pub work_level: Option<String>,
    pub includes: Vec<String>,
    pub inputs: CompileStageInputs,
    pub outputs: CompileStageOutputs,
    pub gating: CompileStageGating,
    pub tags: Vec<String>,
    pub body: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CompileStageInputs {
    pub library: Vec<CompileStageInput>,
    pub artifacts: Vec<CompileStageInput>,
    pub variables: Vec<CompileStageVariable>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompileStageInput {
    pub path: String,
    pub required: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompileStageVariable {
    pub name: String,
    pub optional: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CompileStageOutputs {
    pub artifacts: Vec<CompileStageOutput>,
    pub repo_files: Vec<CompileStageOutput>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompileStageOutput {
    pub path: String,
    pub required: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct CompileStageGating {
    pub mode: Option<String>,
    pub fail_on: Vec<String>,
    pub notes: Vec<String>,
}

impl CompileStageInputs {
    fn from_raw(raw: CompileStageInputsRaw) -> Self {
        Self {
            library: raw
                .library
                .into_iter()
                .map(CompileStageInput::from_raw)
                .collect(),
            artifacts: raw
                .artifacts
                .into_iter()
                .map(CompileStageInput::from_raw)
                .collect(),
            variables: raw
                .variables
                .into_iter()
                .map(|value| {
                    let optional = value.ends_with('?');
                    let name = if optional {
                        value.trim_end_matches('?')
                    } else {
                        value.as_str()
                    };
                    CompileStageVariable {
                        name: name.to_string(),
                        optional,
                    }
                })
                .collect(),
        }
    }
}

impl CompileStageInput {
    fn from_raw(raw: CompileStageInputRaw) -> Self {
        match raw {
            CompileStageInputRaw::Path(path) => Self {
                path,
                required: false,
            },
            CompileStageInputRaw::Entry { path, required } => Self { path, required },
        }
    }
}

impl CompileStageOutputs {
    fn from_raw(raw: CompileStageOutputsRaw) -> Self {
        Self {
            artifacts: raw
                .artifacts
                .into_iter()
                .map(CompileStageOutput::from_raw)
                .collect(),
            repo_files: raw
                .repo_files
                .into_iter()
                .map(CompileStageOutput::from_raw)
                .collect(),
        }
    }
}

impl CompileStageOutput {
    fn from_raw(raw: CompileStageOutputRaw) -> Self {
        match raw {
            CompileStageOutputRaw::Path(path) => Self {
                path,
                required: false,
            },
            CompileStageOutputRaw::Entry { path, required } => Self { path, required },
        }
    }
}

impl CompileStageGating {
    fn from_raw(raw: CompileStageGatingRaw) -> Self {
        Self {
            mode: raw.mode,
            fail_on: raw.fail_on,
            notes: raw.notes,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PipelineSelection {
    Pipeline(PipelineCatalogEntry),
    Stage(StageCatalogEntry),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PipelineLookupError {
    UnsupportedSelector {
        selector: String,
        reason: &'static str,
    },
    AmbiguousSelector {
        selector: String,
        matches: Vec<String>,
    },
    UnknownSelector {
        selector: String,
    },
}

#[derive(Debug)]
pub enum PipelineMetadataSelectionError {
    Catalog(PipelineCatalogError),
    Lookup(PipelineLookupError),
}

#[derive(Debug)]
pub enum SelectedPipelineLoadError {
    Catalog(PipelineCatalogError),
    Lookup(PipelineLookupError),
    Load(PipelineLoadError),
}

#[derive(Debug)]
pub enum PipelineCatalogError {
    ReadPipelineCatalog {
        path: PathBuf,
        source: std::io::Error,
    },
    ReadStageCatalog {
        path: PathBuf,
        source: std::io::Error,
    },
    PipelineLoad {
        path: PathBuf,
        source: Box<PipelineLoadError>,
    },
    StageFrontMatter {
        path: PathBuf,
        source: serde_yaml_bw::Error,
    },
    StageKindMismatch {
        path: PathBuf,
        actual: String,
    },
    InvalidStageCanonicalId {
        path: PathBuf,
        value: String,
        reason: &'static str,
    },
    StageIdMismatch {
        path: PathBuf,
        expected: String,
        actual: String,
    },
    DuplicatePipelineId {
        id: String,
    },
    DuplicateStageId {
        id: String,
    },
}

impl fmt::Display for PipelineCatalogError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PipelineCatalogError::ReadPipelineCatalog { path, source } => {
                write!(
                    f,
                    "failed to read pipeline catalog entry {}: {source}",
                    path.display()
                )
            }
            PipelineCatalogError::ReadStageCatalog { path, source } => {
                write!(
                    f,
                    "failed to read stage catalog entry {}: {source}",
                    path.display()
                )
            }
            PipelineCatalogError::PipelineLoad { path, source } => {
                write!(
                    f,
                    "failed to load pipeline definition {}: {source}",
                    path.display()
                )
            }
            PipelineCatalogError::StageFrontMatter { path, source } => {
                write!(
                    f,
                    "failed to load stage front matter {}: {source}",
                    path.display()
                )
            }
            PipelineCatalogError::StageKindMismatch { path, actual } => {
                write!(
                    f,
                    "stage front matter {} must declare kind `stage`, got `{actual}`",
                    path.display()
                )
            }
            PipelineCatalogError::InvalidStageCanonicalId {
                path,
                value,
                reason,
            } => write!(
                f,
                "stage front matter {} declares invalid canonical id `{value}`: {reason}",
                path.display()
            ),
            PipelineCatalogError::StageIdMismatch {
                path,
                expected,
                actual,
            } => write!(
                f,
                "stage front matter {} expected canonical id `{expected}` but found `{actual}`",
                path.display()
            ),
            PipelineCatalogError::DuplicatePipelineId { id } => {
                write!(f, "duplicate pipeline id `{id}` in pipeline catalog")
            }
            PipelineCatalogError::DuplicateStageId { id } => {
                write!(f, "duplicate stage id `{id}` in stage catalog")
            }
        }
    }
}

impl std::error::Error for PipelineCatalogError {}

impl fmt::Display for PipelineLookupError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PipelineLookupError::UnsupportedSelector { selector, reason } => {
                write!(f, "unsupported selector `{selector}`: {reason}")
            }
            PipelineLookupError::AmbiguousSelector { selector, matches } => {
                write!(
                    f,
                    "ambiguous selector `{selector}` matched multiple canonical ids: {}",
                    matches.join(", ")
                )
            }
            PipelineLookupError::UnknownSelector { selector } => {
                write!(
                    f,
                    "unknown pipeline selector `{selector}`; use a canonical id or `pipeline list` to inspect available inventory"
                )
            }
        }
    }
}

impl std::error::Error for PipelineLookupError {}

impl fmt::Display for PipelineMetadataSelectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PipelineMetadataSelectionError::Catalog(err) => write!(f, "{err}"),
            PipelineMetadataSelectionError::Lookup(err) => write!(f, "{err}"),
        }
    }
}

impl std::error::Error for PipelineMetadataSelectionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            PipelineMetadataSelectionError::Catalog(err) => Some(err),
            PipelineMetadataSelectionError::Lookup(err) => Some(err),
        }
    }
}

impl fmt::Display for SelectedPipelineLoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SelectedPipelineLoadError::Catalog(err) => write!(f, "{err}"),
            SelectedPipelineLoadError::Lookup(err) => write!(f, "{err}"),
            SelectedPipelineLoadError::Load(err) => write!(f, "{err}"),
        }
    }
}

impl std::error::Error for SelectedPipelineLoadError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            SelectedPipelineLoadError::Catalog(err) => Some(err),
            SelectedPipelineLoadError::Lookup(err) => Some(err),
            SelectedPipelineLoadError::Load(err) => Some(err),
        }
    }
}

impl fmt::Display for CompileStageLoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompileStageLoadError::StageNotDeclared {
                pipeline_id,
                stage_id,
            } => write!(
                f,
                "stage `{stage_id}` is not declared in pipeline `{pipeline_id}`"
            ),
            CompileStageLoadError::ReadFailure { path, source } => {
                write!(
                    f,
                    "failed to read compile stage definition {}: {source}",
                    path.display()
                )
            }
            CompileStageLoadError::MissingFrontMatter { path } => {
                write!(
                    f,
                    "compile stage definition {} is missing front matter",
                    path.display()
                )
            }
            CompileStageLoadError::ParseFrontMatter { path, source } => {
                write!(
                    f,
                    "failed to parse compile stage front matter {}: {source}",
                    path.display()
                )
            }
            CompileStageLoadError::StageKindMismatch { path, actual } => write!(
                f,
                "compile stage front matter {} must declare kind `stage`, got `{actual}`",
                path.display()
            ),
            CompileStageLoadError::StageIdMismatch {
                path,
                expected,
                actual,
            } => write!(
                f,
                "compile stage front matter {} expected canonical id `{expected}` but found `{actual}`",
                path.display()
            ),
        }
    }
}

impl std::error::Error for CompileStageLoadError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CompileStageLoadError::ReadFailure { source, .. } => Some(source),
            CompileStageLoadError::ParseFrontMatter { source, .. } => Some(source),
            CompileStageLoadError::StageNotDeclared { .. }
            | CompileStageLoadError::MissingFrontMatter { .. }
            | CompileStageLoadError::StageKindMismatch { .. }
            | CompileStageLoadError::StageIdMismatch { .. } => None,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
struct StageFrontMatter {
    pub kind: String,
    pub id: String,
    pub version: String,
    pub title: String,
    pub description: String,
    #[serde(default)]
    pub work_level: Option<String>,
    #[serde(default)]
    pub activation: Option<StageActivation>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct CompileStageFrontMatterRaw {
    pub kind: String,
    pub id: String,
    pub version: String,
    pub title: String,
    pub description: String,
    #[serde(default)]
    pub work_level: Option<String>,
    #[serde(default)]
    pub includes: Vec<String>,
    #[serde(default)]
    pub inputs: CompileStageInputsRaw,
    #[serde(default)]
    pub outputs: CompileStageOutputsRaw,
    #[serde(default)]
    pub gating: CompileStageGatingRaw,
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct CompileStageInputsRaw {
    #[serde(default)]
    pub library: Vec<CompileStageInputRaw>,
    #[serde(default)]
    pub artifacts: Vec<CompileStageInputRaw>,
    #[serde(default)]
    pub variables: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum CompileStageInputRaw {
    Path(String),
    Entry {
        path: String,
        #[serde(default)]
        required: bool,
    },
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct CompileStageOutputsRaw {
    #[serde(default)]
    pub artifacts: Vec<CompileStageOutputRaw>,
    #[serde(default)]
    pub repo_files: Vec<CompileStageOutputRaw>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum CompileStageOutputRaw {
    Path(String),
    Entry {
        path: String,
        #[serde(default)]
        required: bool,
    },
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct CompileStageGatingRaw {
    #[serde(default)]
    pub mode: Option<String>,
    #[serde(default)]
    pub fail_on: Vec<String>,
    #[serde(default)]
    pub notes: Vec<String>,
}

#[derive(Debug)]
enum StageFrontMatterLoadError {
    Read(std::io::Error),
    Parse(serde_yaml_bw::Error),
}

#[derive(Debug)]
pub enum CompileStageLoadError {
    StageNotDeclared {
        pipeline_id: String,
        stage_id: String,
    },
    ReadFailure {
        path: PathBuf,
        source: std::io::Error,
    },
    MissingFrontMatter {
        path: PathBuf,
    },
    ParseFrontMatter {
        path: PathBuf,
        source: serde_yaml_bw::Error,
    },
    StageKindMismatch {
        path: PathBuf,
        actual: String,
    },
    StageIdMismatch {
        path: PathBuf,
        expected: String,
        actual: String,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PipelineLoadMode {
    MetadataOnly,
    RouteAware,
}

#[allow(dead_code)]
pub(crate) fn load_pipeline_catalog(
    repo_root: impl AsRef<Path>,
) -> Result<PipelineCatalog, PipelineCatalogError> {
    load_pipeline_catalog_with_mode(repo_root, PipelineLoadMode::RouteAware)
}

#[allow(dead_code)]
pub(crate) fn load_pipeline_catalog_with_roots(
    repo_root: impl AsRef<Path>,
    roots: &PipelineDeclarativeRootsContract,
) -> Result<PipelineCatalog, PipelineCatalogError> {
    load_pipeline_catalog_with_mode_and_roots(
        repo_root.as_ref(),
        roots,
        PipelineLoadMode::RouteAware,
    )
}

pub fn load_pipeline_catalog_metadata(
    repo_root: impl AsRef<Path>,
) -> Result<PipelineCatalog, PipelineCatalogError> {
    load_pipeline_metadata_catalog_index(repo_root.as_ref())
}

pub fn load_pipeline_catalog_metadata_with_roots(
    repo_root: impl AsRef<Path>,
    roots: &PipelineDeclarativeRootsContract,
) -> Result<PipelineCatalog, PipelineCatalogError> {
    load_pipeline_metadata_catalog_index_with_roots(repo_root.as_ref(), roots)
}

#[allow(dead_code)]
pub(crate) fn handbook_product_pipeline_declarative_roots(
) -> &'static PipelineDeclarativeRootsContract {
    handbook_product_declarative_roots()
}

pub fn load_pipeline_selection_metadata(
    repo_root: impl AsRef<Path>,
    selector: &str,
) -> Result<PipelineSelection, PipelineMetadataSelectionError> {
    load_pipeline_selection_metadata_with_roots(
        repo_root,
        handbook_product_pipeline_declarative_roots(),
        selector,
    )
}

pub fn load_pipeline_selection_metadata_with_roots(
    repo_root: impl AsRef<Path>,
    roots: &PipelineDeclarativeRootsContract,
    selector: &str,
) -> Result<PipelineSelection, PipelineMetadataSelectionError> {
    let repo_root = repo_root.as_ref();
    let catalog = load_pipeline_catalog_metadata_with_roots(repo_root, roots)
        .map_err(PipelineMetadataSelectionError::Catalog)?;

    match resolve_pipeline_selector(&catalog, selector) {
        Ok(selection) => return Ok(selection),
        Err(PipelineLookupError::UnsupportedSelector { selector, reason }) => {
            return Err(PipelineMetadataSelectionError::Lookup(
                PipelineLookupError::UnsupportedSelector { selector, reason },
            ));
        }
        Err(PipelineLookupError::AmbiguousSelector { selector, matches }) => {
            return Err(PipelineMetadataSelectionError::Lookup(
                PipelineLookupError::AmbiguousSelector { selector, matches },
            ));
        }
        Err(PipelineLookupError::UnknownSelector { .. }) => {}
    }

    if let Some(err) = find_selected_pipeline_metadata_error_with_roots(repo_root, roots, selector)
        .map_err(PipelineMetadataSelectionError::Catalog)?
    {
        return Err(PipelineMetadataSelectionError::Catalog(err));
    }

    Err(PipelineMetadataSelectionError::Lookup(
        PipelineLookupError::UnknownSelector {
            selector: selector.trim().to_string(),
        },
    ))
}

pub fn load_stage_compile_definition(
    repo_root: impl AsRef<Path>,
    pipeline: &PipelineDefinition,
    stage_id: &str,
) -> Result<CompileStageDefinition, CompileStageLoadError> {
    let repo_root = repo_root.as_ref();
    let workspace = CompilerWorkspace::new(repo_root);
    let Some(stage) = pipeline
        .declared_stages()
        .iter()
        .find(|stage| stage.id == stage_id)
        .cloned()
    else {
        return Err(CompileStageLoadError::StageNotDeclared {
            pipeline_id: pipeline.header.id.clone(),
            stage_id: stage_id.to_string(),
        });
    };

    let relative_path = workspace
        .normalize_repo_relative(&stage.file)
        .map_err(|reason| CompileStageLoadError::ReadFailure {
            path: repo_root.join(&stage.file),
            source: std::io::Error::new(std::io::ErrorKind::InvalidInput, reason),
        })?;
    let path = repo_root.join(relative_path.as_str());
    let contents = workspace
        .read_string(&relative_path)
        .map_err(|err| compile_stage_read_failure(path.clone(), err))?;
    let Some((front_matter_text, body)) = extract_front_matter_parts(&contents) else {
        return Err(CompileStageLoadError::MissingFrontMatter { path });
    };

    let front_matter = serde_yaml_bw::from_str::<CompileStageFrontMatterRaw>(&front_matter_text)
        .map_err(|source| CompileStageLoadError::ParseFrontMatter {
            path: path.clone(),
            source,
        })?;

    if front_matter.kind != "stage" {
        return Err(CompileStageLoadError::StageKindMismatch {
            path,
            actual: front_matter.kind,
        });
    }

    if front_matter.id != stage.id {
        return Err(CompileStageLoadError::StageIdMismatch {
            path,
            expected: stage.id,
            actual: front_matter.id,
        });
    }

    Ok(CompileStageDefinition {
        stage,
        source_path: path,
        kind: front_matter.kind,
        id: front_matter.id,
        version: front_matter.version,
        title: front_matter.title,
        description: front_matter.description,
        work_level: front_matter.work_level,
        includes: front_matter.includes,
        inputs: CompileStageInputs::from_raw(front_matter.inputs),
        outputs: CompileStageOutputs::from_raw(front_matter.outputs),
        gating: CompileStageGating::from_raw(front_matter.gating),
        tags: front_matter.tags,
        body: normalize_optional_body(&body),
    })
}

pub fn load_selected_pipeline_definition(
    repo_root: impl AsRef<Path>,
    selector: &str,
) -> Result<PipelineDefinition, SelectedPipelineLoadError> {
    load_selected_pipeline_definition_with_roots(
        repo_root,
        handbook_product_pipeline_declarative_roots(),
        selector,
    )
}

pub fn load_selected_pipeline_definition_with_roots(
    repo_root: impl AsRef<Path>,
    roots: &PipelineDeclarativeRootsContract,
    selector: &str,
) -> Result<PipelineDefinition, SelectedPipelineLoadError> {
    let repo_root = repo_root.as_ref();
    let catalog = load_pipeline_catalog_metadata_with_roots(repo_root, roots)
        .map_err(SelectedPipelineLoadError::Catalog)?;
    let pipeline = match resolve_pipeline_only_selector(&catalog, selector) {
        Ok(pipeline) => pipeline,
        Err(err @ PipelineLookupError::UnknownSelector { .. }) => {
            if let Some(metadata_err) =
                find_selected_pipeline_metadata_error_with_roots(repo_root, roots, selector)
                    .map_err(SelectedPipelineLoadError::Catalog)?
            {
                return Err(SelectedPipelineLoadError::Catalog(metadata_err));
            }
            return Err(SelectedPipelineLoadError::Lookup(err));
        }
        Err(err) => return Err(SelectedPipelineLoadError::Lookup(err)),
    };

    load_pipeline_definition_with_roots(repo_root, roots, &pipeline.definition.source_path)
        .map_err(SelectedPipelineLoadError::Load)
}

#[allow(dead_code)]
fn load_pipeline_catalog_with_mode(
    repo_root: impl AsRef<Path>,
    mode: PipelineLoadMode,
) -> Result<PipelineCatalog, PipelineCatalogError> {
    let repo_root = repo_root.as_ref();
    let mut pipelines = std::collections::BTreeMap::<String, PipelineCatalogEntry>::new();
    let mut stages = std::collections::BTreeMap::<String, StageCatalogEntry>::new();
    let stage_catalog = load_stage_catalog(repo_root)?;
    let mut stage_memberships: std::collections::BTreeMap<String, Vec<String>> =
        std::collections::BTreeMap::new();

    for pipeline_path in discover_repo_relative_files(repo_root, pipeline_root(), "yaml")? {
        let definition = load_pipeline_definition_with_mode(repo_root, &pipeline_path, mode)
            .map_err(|source| PipelineCatalogError::PipelineLoad {
                path: repo_root.join(&pipeline_path),
                source: Box::new(source),
            })?;
        let pipeline_id = definition.header.id.clone();

        let mut stage_entries = Vec::with_capacity(definition.declared_stages().len());
        for stage in definition.declared_stages() {
            let stage_catalog_entry =
                stage_catalog.get(Path::new(&stage.file)).ok_or_else(|| {
                    PipelineCatalogError::StageFrontMatter {
                        path: repo_root.join(&stage.file),
                        source: <serde_yaml_bw::Error as serde::de::Error>::custom(format!(
                            "stage file {} is missing front matter or is not cataloged",
                            stage.file
                        )),
                    }
                })?;

            if stage_catalog_entry.id != stage.id {
                return Err(PipelineCatalogError::StageIdMismatch {
                    path: stage_catalog_entry.source_path.clone(),
                    expected: stage.id.clone(),
                    actual: stage_catalog_entry.id.clone(),
                });
            }

            stage_entries.push(PipelineCatalogStageEntry {
                stage_id: stage_catalog_entry.id.clone(),
                title: stage_catalog_entry.title.clone(),
                work_level: stage_catalog_entry.work_level.clone(),
                source_path: stage_catalog_entry.source_path.clone(),
            });

            stage_memberships
                .entry(stage_catalog_entry.id.clone())
                .or_default()
                .push(definition.header.id.clone());
        }

        if pipelines
            .insert(
                pipeline_id.clone(),
                PipelineCatalogEntry {
                    definition,
                    stages: stage_entries,
                },
            )
            .is_some()
        {
            return Err(PipelineCatalogError::DuplicatePipelineId { id: pipeline_id });
        }
    }

    for (stage_path, mut entry) in stage_catalog {
        entry.pipelines = stage_memberships.remove(&entry.id).unwrap_or_default();
        let stage_id = entry.id.clone();
        if stages.insert(stage_id.clone(), entry).is_some() {
            return Err(PipelineCatalogError::DuplicateStageId { id: stage_id });
        }
        let _ = stage_path;
    }

    Ok(PipelineCatalog { pipelines, stages })
}

#[allow(dead_code)]
fn load_pipeline_catalog_with_mode_and_roots(
    repo_root: &Path,
    roots: &PipelineDeclarativeRootsContract,
    mode: PipelineLoadMode,
) -> Result<PipelineCatalog, PipelineCatalogError> {
    let mut pipelines = std::collections::BTreeMap::<String, PipelineCatalogEntry>::new();
    let mut stages = std::collections::BTreeMap::<String, StageCatalogEntry>::new();
    let stage_catalog = load_stage_catalog_with_roots(repo_root, roots)?;
    let mut stage_memberships: std::collections::BTreeMap<String, Vec<String>> =
        std::collections::BTreeMap::new();

    for pipeline_path in discover_repo_relative_files_with_kind(
        repo_root,
        roots.pipeline_root(),
        "yaml",
        CatalogDiscoveryKind::Pipelines,
    )? {
        let definition =
            load_pipeline_definition_with_mode_and_roots(repo_root, roots, &pipeline_path, mode)
                .map_err(|source| PipelineCatalogError::PipelineLoad {
                    path: repo_root.join(&pipeline_path),
                    source: Box::new(source),
                })?;
        let pipeline_id = definition.header.id.clone();

        let mut stage_entries = Vec::with_capacity(definition.declared_stages().len());
        for stage in definition.declared_stages() {
            let stage_catalog_entry =
                stage_catalog.get(Path::new(&stage.file)).ok_or_else(|| {
                    PipelineCatalogError::StageFrontMatter {
                        path: repo_root.join(&stage.file),
                        source: <serde_yaml_bw::Error as serde::de::Error>::custom(format!(
                            "stage file {} is missing front matter or is not cataloged",
                            stage.file
                        )),
                    }
                })?;

            if stage_catalog_entry.id != stage.id {
                return Err(PipelineCatalogError::StageIdMismatch {
                    path: stage_catalog_entry.source_path.clone(),
                    expected: stage.id.clone(),
                    actual: stage_catalog_entry.id.clone(),
                });
            }

            stage_entries.push(PipelineCatalogStageEntry {
                stage_id: stage_catalog_entry.id.clone(),
                title: stage_catalog_entry.title.clone(),
                work_level: stage_catalog_entry.work_level.clone(),
                source_path: stage_catalog_entry.source_path.clone(),
            });

            stage_memberships
                .entry(stage_catalog_entry.id.clone())
                .or_default()
                .push(definition.header.id.clone());
        }

        if pipelines
            .insert(
                pipeline_id.clone(),
                PipelineCatalogEntry {
                    definition,
                    stages: stage_entries,
                },
            )
            .is_some()
        {
            return Err(PipelineCatalogError::DuplicatePipelineId { id: pipeline_id });
        }
    }

    for (stage_path, mut entry) in stage_catalog {
        entry.pipelines = stage_memberships.remove(&entry.id).unwrap_or_default();
        let stage_id = entry.id.clone();
        if stages.insert(stage_id.clone(), entry).is_some() {
            return Err(PipelineCatalogError::DuplicateStageId { id: stage_id });
        }
        let _ = stage_path;
    }

    Ok(PipelineCatalog { pipelines, stages })
}

fn load_pipeline_metadata_catalog_index(
    repo_root: &Path,
) -> Result<PipelineCatalog, PipelineCatalogError> {
    let mut pipelines = std::collections::BTreeMap::<String, PipelineCatalogEntry>::new();
    let mut stages = std::collections::BTreeMap::<String, StageCatalogEntry>::new();
    let mut stage_memberships: std::collections::BTreeMap<String, Vec<String>> =
        std::collections::BTreeMap::new();

    for pipeline_path in discover_repo_relative_files(repo_root, pipeline_root(), "yaml")? {
        let definition = match load_pipeline_definition_with_mode(
            repo_root,
            &pipeline_path,
            PipelineLoadMode::MetadataOnly,
        ) {
            Ok(definition) => definition,
            Err(_) => continue,
        };

        let entry = match build_pipeline_catalog_entry_metadata(repo_root, &definition) {
            Ok(entry) => entry,
            Err(_) => continue,
        };

        let pipeline_id = definition.header.id.clone();
        if pipelines.insert(pipeline_id.clone(), entry).is_some() {
            return Err(PipelineCatalogError::DuplicatePipelineId { id: pipeline_id });
        }

        for stage in definition.declared_stages() {
            let stage_entry = load_stage_catalog_entry_for_reference(repo_root, stage)?;
            stage_memberships
                .entry(stage_entry.id.clone())
                .or_default()
                .push(definition.header.id.clone());

            match stages.get(&stage_entry.id) {
                Some(existing) if existing.source_path != stage_entry.source_path => {
                    return Err(PipelineCatalogError::DuplicateStageId {
                        id: stage_entry.id.clone(),
                    });
                }
                Some(_) => {}
                None => {
                    stages.insert(stage_entry.id.clone(), stage_entry);
                }
            }
        }
    }

    for (stage_id, pipelines_for_stage) in stage_memberships {
        if let Some(stage) = stages.get_mut(&stage_id) {
            stage.pipelines = pipelines_for_stage;
        }
    }

    Ok(PipelineCatalog { pipelines, stages })
}

#[allow(dead_code)]
fn load_pipeline_metadata_catalog_index_with_roots(
    repo_root: &Path,
    roots: &PipelineDeclarativeRootsContract,
) -> Result<PipelineCatalog, PipelineCatalogError> {
    let mut pipelines = std::collections::BTreeMap::<String, PipelineCatalogEntry>::new();
    let mut stages = std::collections::BTreeMap::<String, StageCatalogEntry>::new();
    let mut stage_memberships: std::collections::BTreeMap<String, Vec<String>> =
        std::collections::BTreeMap::new();

    for pipeline_path in discover_repo_relative_files_with_kind(
        repo_root,
        roots.pipeline_root(),
        "yaml",
        CatalogDiscoveryKind::Pipelines,
    )? {
        let definition = match load_pipeline_definition_with_mode_and_roots(
            repo_root,
            roots,
            &pipeline_path,
            PipelineLoadMode::MetadataOnly,
        ) {
            Ok(definition) => definition,
            Err(_) => continue,
        };

        let entry = match build_pipeline_catalog_entry_metadata(repo_root, &definition) {
            Ok(entry) => entry,
            Err(_) => continue,
        };

        let pipeline_id = definition.header.id.clone();
        if pipelines.insert(pipeline_id.clone(), entry).is_some() {
            return Err(PipelineCatalogError::DuplicatePipelineId { id: pipeline_id });
        }

        for stage in definition.declared_stages() {
            let stage_entry = load_stage_catalog_entry_for_reference(repo_root, stage)?;
            stage_memberships
                .entry(stage_entry.id.clone())
                .or_default()
                .push(definition.header.id.clone());

            match stages.get(&stage_entry.id) {
                Some(existing) if existing.source_path != stage_entry.source_path => {
                    return Err(PipelineCatalogError::DuplicateStageId {
                        id: stage_entry.id.clone(),
                    });
                }
                Some(_) => {}
                None => {
                    stages.insert(stage_entry.id.clone(), stage_entry);
                }
            }
        }
    }

    for (stage_id, pipelines_for_stage) in stage_memberships {
        if let Some(stage) = stages.get_mut(&stage_id) {
            stage.pipelines = pipelines_for_stage;
        }
    }

    Ok(PipelineCatalog { pipelines, stages })
}

fn build_pipeline_catalog_entry_metadata(
    repo_root: &Path,
    definition: &PipelineDefinition,
) -> Result<PipelineCatalogEntry, PipelineCatalogError> {
    let mut stage_entries = Vec::with_capacity(definition.declared_stages().len());
    for stage in definition.declared_stages() {
        let stage_catalog_entry = load_stage_catalog_entry_for_reference(repo_root, stage)?;
        stage_entries.push(PipelineCatalogStageEntry {
            stage_id: stage_catalog_entry.id,
            title: stage_catalog_entry.title,
            work_level: stage_catalog_entry.work_level,
            source_path: stage_catalog_entry.source_path,
        });
    }

    Ok(PipelineCatalogEntry {
        definition: definition.clone(),
        stages: stage_entries,
    })
}

fn load_stage_catalog_entry_for_reference(
    repo_root: &Path,
    stage: &PipelineStage,
) -> Result<StageCatalogEntry, PipelineCatalogError> {
    let full_path = repo_root.join(&stage.file);
    let Some(front_matter) =
        load_stage_front_matter(repo_root, Path::new(&stage.file)).map_err(|err| match err {
            StageFrontMatterLoadError::Read(source) => PipelineCatalogError::ReadStageCatalog {
                path: full_path.clone(),
                source,
            },
            StageFrontMatterLoadError::Parse(source) => PipelineCatalogError::StageFrontMatter {
                path: full_path.clone(),
                source,
            },
        })?
    else {
        return Err(PipelineCatalogError::StageFrontMatter {
            path: full_path,
            source: <serde_yaml_bw::Error as serde::de::Error>::custom(format!(
                "stage file {} is missing front matter or is not cataloged",
                stage.file
            )),
        });
    };

    if front_matter.kind != "stage" {
        return Err(PipelineCatalogError::StageKindMismatch {
            path: full_path,
            actual: front_matter.kind,
        });
    }
    if let Some(reason) = canonical_id_path_like_reason(&front_matter.id) {
        return Err(PipelineCatalogError::InvalidStageCanonicalId {
            path: repo_root.join(&stage.file),
            value: front_matter.id,
            reason,
        });
    }
    if front_matter.id != stage.id {
        return Err(PipelineCatalogError::StageIdMismatch {
            path: PathBuf::from(&stage.file),
            expected: stage.id.clone(),
            actual: front_matter.id,
        });
    }

    Ok(StageCatalogEntry {
        id: stage.id.clone(),
        kind: front_matter.kind,
        version: front_matter.version,
        title: front_matter.title,
        description: front_matter.description,
        work_level: front_matter.work_level,
        source_path: PathBuf::from(&stage.file),
        pipelines: Vec::new(),
    })
}

fn find_selected_pipeline_metadata_error_with_roots(
    repo_root: &Path,
    roots: &PipelineDeclarativeRootsContract,
    selector: &str,
) -> Result<Option<PipelineCatalogError>, PipelineCatalogError> {
    for pipeline_path in discover_repo_relative_files_with_kind(
        repo_root,
        roots.pipeline_root(),
        "yaml",
        CatalogDiscoveryKind::Pipelines,
    )? {
        let definition = match load_pipeline_definition_with_mode_and_roots(
            repo_root,
            roots,
            &pipeline_path,
            PipelineLoadMode::MetadataOnly,
        ) {
            Ok(definition) => definition,
            Err(source) => {
                if pipeline_selector_matches_path(repo_root, &pipeline_path, selector)? {
                    return Ok(Some(PipelineCatalogError::PipelineLoad {
                        path: repo_root.join(&pipeline_path),
                        source: Box::new(source),
                    }));
                }
                continue;
            }
        };

        if matches_pipeline_selector(&definition.header.id, selector) {
            if let Err(err) = build_pipeline_catalog_entry_metadata(repo_root, &definition) {
                return Ok(Some(err));
            }
        }
    }

    Ok(None)
}

fn pipeline_selector_matches_path(
    repo_root: &Path,
    pipeline_path: &Path,
    selector: &str,
) -> Result<bool, PipelineCatalogError> {
    match load_pipeline_header(repo_root, pipeline_path) {
        Ok(Some(header)) => Ok(matches_pipeline_selector(&header.id, selector)),
        Ok(None) => Ok(false),
        Err(source) => Err(PipelineCatalogError::PipelineLoad {
            path: repo_root.join(pipeline_path),
            source: Box::new(source),
        }),
    }
}

fn matches_pipeline_selector(pipeline_id: &str, selector: &str) -> bool {
    let selector = selector.trim();
    selector == pipeline_id || pipeline_id.strip_prefix("pipeline.") == Some(selector)
}

pub fn render_pipeline_list(catalog: &PipelineCatalog) -> String {
    let mut out = String::new();
    out.push_str("PIPELINE INVENTORY\n");
    out.push_str(&format!("PIPELINE COUNT: {}\n", catalog.pipeline_count()));
    out.push_str(&format!("STAGE COUNT: {}\n", catalog.stage_count()));

    for pipeline in catalog.pipelines() {
        out.push_str(&format!("\nPIPELINE: {}\n", pipeline.definition.header.id));
        out.push_str(&format!("TITLE: {}\n", pipeline.definition.header.title));
        out.push_str(&format!(
            "SOURCE: {}\n",
            pipeline.definition.source_path.display()
        ));
        out.push_str(&format!("STAGES: {}\n", pipeline.stages.len()));
    }

    out.trim_end().to_string()
}

pub fn render_pipeline_show(selection: &PipelineSelection) -> String {
    match selection {
        PipelineSelection::Pipeline(pipeline) => render_pipeline_definition(pipeline),
        PipelineSelection::Stage(stage) => render_stage_definition(stage),
    }
}

pub fn resolve_pipeline_selector(
    catalog: &PipelineCatalog,
    selector: &str,
) -> Result<PipelineSelection, PipelineLookupError> {
    let selector = selector.trim();
    if selector.is_empty() {
        return Err(PipelineLookupError::UnknownSelector {
            selector: selector.to_string(),
        });
    }

    if canonical_id_path_like_reason(selector).is_some() {
        return Err(PipelineLookupError::UnsupportedSelector {
            selector: selector.to_string(),
            reason: "raw file paths are evidence only; use a canonical pipeline or stage id",
        });
    }

    if let Some(pipeline) = catalog.pipelines.get(selector) {
        return Ok(PipelineSelection::Pipeline(pipeline.clone()));
    }
    if let Some(stage) = catalog.stages.get(selector) {
        return Ok(PipelineSelection::Stage(stage.clone()));
    }

    let shorthand_matches = catalog
        .pipelines
        .values()
        .filter(|pipeline| {
            pipeline.definition.header.id.strip_prefix("pipeline.") == Some(selector)
        })
        .map(|pipeline| pipeline.definition.header.id.clone())
        .chain(
            catalog
                .stages
                .values()
                .filter(|stage| stage.id.strip_prefix("stage.") == Some(selector))
                .map(|stage| stage.id.clone()),
        )
        .collect::<Vec<_>>();

    if shorthand_matches.len() > 1 {
        return Err(PipelineLookupError::AmbiguousSelector {
            selector: selector.to_string(),
            matches: shorthand_matches,
        });
    }

    if let Some(canonical_id) = shorthand_matches.into_iter().next() {
        if let Some(pipeline) = catalog.pipelines.get(&canonical_id) {
            return Ok(PipelineSelection::Pipeline(pipeline.clone()));
        }
        if let Some(stage) = catalog.stages.get(&canonical_id) {
            return Ok(PipelineSelection::Stage(stage.clone()));
        }
    }

    Err(PipelineLookupError::UnknownSelector {
        selector: selector.to_string(),
    })
}

pub fn resolve_pipeline_only_selector(
    catalog: &PipelineCatalog,
    selector: &str,
) -> Result<PipelineCatalogEntry, PipelineLookupError> {
    match resolve_pipeline_selector(catalog, selector) {
        Ok(PipelineSelection::Pipeline(pipeline)) => Ok(pipeline),
        Ok(PipelineSelection::Stage(_stage)) => Err(PipelineLookupError::UnsupportedSelector {
            selector: selector.trim().to_string(),
            reason: "stage ids are not supported for `pipeline resolve` or `pipeline state set`; use a canonical pipeline id",
        }),
        Err(err) => Err(err),
    }
}

pub fn supported_route_state_variables(pipeline: &PipelineDefinition) -> BTreeSet<String> {
    let mut variables = BTreeSet::new();
    for stage in pipeline.declared_stages() {
        if let Some(sets) = &stage.sets {
            for variable in sets {
                variables.insert(variable.clone());
            }
        }
        if let Some(activation) = &stage.activation {
            for clause in &activation.when.clauses {
                variables.insert(clause.variable.clone());
            }
        }
    }

    variables
}

fn render_pipeline_definition(pipeline: &PipelineCatalogEntry) -> String {
    let mut out = String::new();
    out.push_str(&format!("PIPELINE: {}\n", pipeline.definition.header.id));
    out.push_str(&format!("TITLE: {}\n", pipeline.definition.header.title));
    out.push_str(&format!(
        "DESCRIPTION: {}\n",
        pipeline.definition.header.description.trim()
    ));
    out.push_str(&format!(
        "SOURCE: {}\n",
        pipeline.definition.source_path.display()
    ));
    out.push_str("DEFAULTS:\n");
    out.push_str(&format!(
        "  runner: {}\n",
        pipeline.definition.body.defaults.runner
    ));
    out.push_str(&format!(
        "  profile: {}\n",
        pipeline.definition.body.defaults.profile
    ));
    out.push_str(&format!(
        "  enable_complexity: {}\n",
        pipeline.definition.body.defaults.enable_complexity
    ));
    out.push_str("STAGES:\n");
    for (index, (stage, declared_stage)) in pipeline
        .stages
        .iter()
        .zip(pipeline.definition.declared_stages().iter())
        .enumerate()
    {
        out.push_str(&format!(
            "  {}. {} | {}",
            index + 1,
            stage.stage_id,
            stage.source_path.display()
        ));
        out.push_str(&format!(" | {}\n", stage.title));
        if let Some(work_level) = &stage.work_level {
            out.push_str(&format!("     work_level: {}\n", work_level));
        }
        if let Some(sets) = &declared_stage.sets {
            out.push_str(&format!("     sets: {}\n", render_sets(sets)));
        }
        if let Some(activation) = &declared_stage.activation {
            out.push_str(&format!(
                "     activation: {}\n",
                render_activation(activation)
            ));
        }
    }

    out.trim_end().to_string()
}

fn render_stage_definition(stage: &StageCatalogEntry) -> String {
    let mut out = String::new();
    out.push_str(&format!("STAGE: {}\n", stage.id));
    out.push_str(&format!("KIND: {}\n", stage.kind));
    out.push_str(&format!("VERSION: {}\n", stage.version));
    out.push_str(&format!("TITLE: {}\n", stage.title));
    out.push_str(&format!("DESCRIPTION: {}\n", stage.description.trim()));
    if let Some(work_level) = &stage.work_level {
        out.push_str(&format!("WORK_LEVEL: {}\n", work_level));
    }
    out.push_str(&format!("SOURCE: {}\n", stage.source_path.display()));
    out.push_str("PIPELINES:\n");
    for pipeline in &stage.pipelines {
        out.push_str(&format!("  - {}\n", pipeline));
    }

    out.trim_end().to_string()
}

#[allow(dead_code)]
fn load_stage_catalog(
    repo_root: &Path,
) -> Result<std::collections::BTreeMap<PathBuf, StageCatalogEntry>, PipelineCatalogError> {
    let mut out = std::collections::BTreeMap::new();
    for stage_path in discover_repo_relative_files(repo_root, stage_root(), "md")? {
        let full_path = repo_root.join(&stage_path);
        let Some(front_matter) =
            load_stage_front_matter(repo_root, &stage_path).map_err(|err| match err {
                StageFrontMatterLoadError::Read(source) => PipelineCatalogError::ReadStageCatalog {
                    path: full_path.clone(),
                    source,
                },
                StageFrontMatterLoadError::Parse(source) => {
                    PipelineCatalogError::StageFrontMatter {
                        path: full_path.clone(),
                        source,
                    }
                }
            })?
        else {
            continue;
        };

        if front_matter.kind != "stage" {
            return Err(PipelineCatalogError::StageKindMismatch {
                path: full_path.clone(),
                actual: front_matter.kind,
            });
        }
        if let Some(reason) = canonical_id_path_like_reason(&front_matter.id) {
            return Err(PipelineCatalogError::InvalidStageCanonicalId {
                path: full_path.clone(),
                value: front_matter.id,
                reason,
            });
        }

        out.insert(
            stage_path.clone(),
            StageCatalogEntry {
                id: front_matter.id,
                kind: front_matter.kind,
                version: front_matter.version,
                title: front_matter.title,
                description: front_matter.description,
                work_level: front_matter.work_level,
                source_path: stage_path.clone(),
                pipelines: Vec::new(),
            },
        );
    }

    Ok(out)
}

#[allow(dead_code)]
fn load_stage_catalog_with_roots(
    repo_root: &Path,
    roots: &PipelineDeclarativeRootsContract,
) -> Result<std::collections::BTreeMap<PathBuf, StageCatalogEntry>, PipelineCatalogError> {
    let mut out = std::collections::BTreeMap::new();
    for stage_path in discover_repo_relative_files_with_kind(
        repo_root,
        roots.stage_root(),
        "md",
        CatalogDiscoveryKind::Stages,
    )? {
        let full_path = repo_root.join(&stage_path);
        let Some(front_matter) =
            load_stage_front_matter(repo_root, &stage_path).map_err(|err| match err {
                StageFrontMatterLoadError::Read(source) => PipelineCatalogError::ReadStageCatalog {
                    path: full_path.clone(),
                    source,
                },
                StageFrontMatterLoadError::Parse(source) => {
                    PipelineCatalogError::StageFrontMatter {
                        path: full_path.clone(),
                        source,
                    }
                }
            })?
        else {
            continue;
        };

        if front_matter.kind != "stage" {
            return Err(PipelineCatalogError::StageKindMismatch {
                path: full_path.clone(),
                actual: front_matter.kind,
            });
        }
        if let Some(reason) = canonical_id_path_like_reason(&front_matter.id) {
            return Err(PipelineCatalogError::InvalidStageCanonicalId {
                path: full_path.clone(),
                value: front_matter.id,
                reason,
            });
        }

        out.insert(
            stage_path.clone(),
            StageCatalogEntry {
                id: front_matter.id,
                kind: front_matter.kind,
                version: front_matter.version,
                title: front_matter.title,
                description: front_matter.description,
                work_level: front_matter.work_level,
                source_path: stage_path.clone(),
                pipelines: Vec::new(),
            },
        );
    }

    Ok(out)
}

fn extract_front_matter_block(contents: &str) -> Option<String> {
    extract_front_matter_parts(contents).map(|(front_matter, _body)| front_matter)
}

fn extract_front_matter_parts(contents: &str) -> Option<(String, String)> {
    let mut lines = contents.lines();
    if lines.next()? != "---" {
        return None;
    }

    let mut front_matter = String::new();
    while let Some(line) = lines.next() {
        if line == "---" {
            let body = lines.collect::<Vec<_>>().join("\n");
            return Some((front_matter, body));
        }
        front_matter.push_str(line);
        front_matter.push('\n');
    }

    None
}

fn normalize_optional_body(body: &str) -> Option<String> {
    let trimmed = body.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

fn discover_repo_relative_files(
    repo_root: &Path,
    relative_dir: &Path,
    extension: &str,
) -> Result<Vec<PathBuf>, PipelineCatalogError> {
    let full_dir = repo_root.join(relative_dir);
    let entries = std::fs::read_dir(&full_dir).map_err(|source| {
        if relative_dir == pipeline_root() {
            PipelineCatalogError::ReadPipelineCatalog {
                path: full_dir.clone(),
                source,
            }
        } else {
            PipelineCatalogError::ReadStageCatalog {
                path: full_dir.clone(),
                source,
            }
        }
    })?;

    let mut out = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|source| {
            if relative_dir == pipeline_root() {
                PipelineCatalogError::ReadPipelineCatalog {
                    path: full_dir.clone(),
                    source,
                }
            } else {
                PipelineCatalogError::ReadStageCatalog {
                    path: full_dir.clone(),
                    source,
                }
            }
        })?;
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        if path.extension().and_then(|value| value.to_str()) != Some(extension) {
            continue;
        }

        let relative = path.strip_prefix(repo_root).unwrap_or(&path).to_path_buf();
        out.push(relative);
    }

    out.sort();
    Ok(out)
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
enum CatalogDiscoveryKind {
    Pipelines,
    Stages,
}

#[allow(dead_code)]
fn discover_repo_relative_files_with_kind(
    repo_root: &Path,
    relative_dir: &Path,
    extension: &str,
    kind: CatalogDiscoveryKind,
) -> Result<Vec<PathBuf>, PipelineCatalogError> {
    let full_dir = repo_root.join(relative_dir);
    let entries = std::fs::read_dir(&full_dir).map_err(|source| match kind {
        CatalogDiscoveryKind::Pipelines => PipelineCatalogError::ReadPipelineCatalog {
            path: full_dir.clone(),
            source,
        },
        CatalogDiscoveryKind::Stages => PipelineCatalogError::ReadStageCatalog {
            path: full_dir.clone(),
            source,
        },
    })?;

    let mut out = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|source| match kind {
            CatalogDiscoveryKind::Pipelines => PipelineCatalogError::ReadPipelineCatalog {
                path: full_dir.clone(),
                source,
            },
            CatalogDiscoveryKind::Stages => PipelineCatalogError::ReadStageCatalog {
                path: full_dir.clone(),
                source,
            },
        })?;
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        if path.extension().and_then(|value| value.to_str()) != Some(extension) {
            continue;
        }
        let relative_path = path
            .strip_prefix(repo_root)
            .expect("discovered file should stay within repo root");
        out.push(relative_path.to_path_buf());
    }

    out.sort();
    Ok(out)
}

fn canonical_id_path_like_reason(value: &str) -> Option<&'static str> {
    if value.contains('/')
        || value.contains('\\')
        || value.ends_with(".yaml")
        || value.ends_with(".yml")
        || value.ends_with(".md")
    {
        Some("canonical ids must not look like raw repo-relative paths")
    } else {
        None
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PipelineHeader {
    pub kind: String,
    pub id: String,
    pub version: String,
    pub title: String,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PipelineBody {
    pub defaults: PipelineDefaults,
    pub stages: Vec<PipelineStage>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PipelineDefaults {
    pub runner: String,
    pub profile: String,
    pub enable_complexity: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PipelineStage {
    pub id: String,
    pub file: String,
    pub sets: Option<Vec<String>>,
    pub activation: Option<StageActivation>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StageActivation {
    pub when: ActivationConditionSet,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActivationConditionSet {
    pub clauses: Vec<ActivationClause>,
    pub operator: ActivationOperator,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActivationOperator {
    Any,
    All,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActivationClause {
    pub variable: String,
    pub value: bool,
}

#[derive(Debug)]
pub enum PipelineLoadError {
    ReadFailure {
        path: PathBuf,
        source: std::io::Error,
    },
    UnsupportedPipelinePath {
        path: PathBuf,
        reason: &'static str,
    },
    WrongDocumentCount {
        path: PathBuf,
        actual: usize,
    },
    HeaderParse {
        path: PathBuf,
        source: serde_yaml_bw::Error,
    },
    BodyParse {
        path: PathBuf,
        source: serde_yaml_bw::Error,
    },
    Validation {
        path: PathBuf,
        error: PipelineValidationError,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PipelineValidationError {
    EmptyField {
        field: &'static str,
    },
    InvalidCanonicalId {
        field: &'static str,
        value: String,
        reason: &'static str,
    },
    UnsupportedKind {
        actual: String,
    },
    EmptyStages,
    DuplicateStageId {
        stage_id: String,
    },
    EmptySetsList {
        stage_id: String,
    },
    EmptySetVariable {
        stage_id: String,
        index: usize,
    },
    InvalidSetVariable {
        stage_id: String,
        variable: String,
        reason: &'static str,
    },
    StageFileOutsideRepoRoot {
        stage_id: String,
        file: String,
    },
    InvalidStageFile {
        stage_id: String,
        file: String,
        reason: StageFileValidationError,
    },
    InvalidActivation {
        stage_id: String,
        reason: ActivationValidationError,
    },
    InvalidStageFrontMatter {
        stage_id: String,
        file: String,
        detail: String,
    },
    ActivationDrift {
        stage_id: String,
        file: String,
        detail: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StageFileValidationError {
    OutsideStageDirectory { stage_root: &'static str },
    WrongExtension,
    Missing,
    NotRegularFile,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActivationValidationError {
    EmptyConditionList {
        operator: ActivationOperator,
    },
    UnsupportedClause {
        clause: String,
        detail: &'static str,
    },
}

pub fn load_pipeline_definition(
    repo_root: impl AsRef<Path>,
    pipeline_path: impl AsRef<Path>,
) -> Result<PipelineDefinition, PipelineLoadError> {
    load_pipeline_definition_with_mode(repo_root, pipeline_path, PipelineLoadMode::RouteAware)
}

pub fn load_pipeline_definition_with_roots(
    repo_root: impl AsRef<Path>,
    roots: &PipelineDeclarativeRootsContract,
    pipeline_path: impl AsRef<Path>,
) -> Result<PipelineDefinition, PipelineLoadError> {
    load_pipeline_definition_with_mode_and_roots(
        repo_root.as_ref(),
        roots,
        pipeline_path.as_ref(),
        PipelineLoadMode::RouteAware,
    )
}

fn load_pipeline_header(
    repo_root: &Path,
    pipeline_path: &Path,
) -> Result<Option<PipelineHeader>, PipelineLoadError> {
    let relative_pipeline_path = validate_pipeline_repo_relative_path(repo_root, pipeline_path)?;
    let full_path = repo_root.join(relative_pipeline_path.as_str());
    let contents =
        std::fs::read_to_string(&full_path).map_err(|source| PipelineLoadError::ReadFailure {
            path: full_path.clone(),
            source,
        })?;

    let mut docs = serde_yaml_bw::Deserializer::from_str(&contents);
    let header_doc = docs.next();

    let Some(header_doc) = header_doc else {
        return Ok(None);
    };

    let header = PipelineHeader::deserialize(header_doc).map_err(|source| {
        PipelineLoadError::HeaderParse {
            path: full_path,
            source,
        }
    })?;

    Ok(Some(header))
}

fn load_pipeline_definition_with_mode(
    repo_root: impl AsRef<Path>,
    pipeline_path: impl AsRef<Path>,
    mode: PipelineLoadMode,
) -> Result<PipelineDefinition, PipelineLoadError> {
    let repo_root = repo_root.as_ref();
    let pipeline_path = pipeline_path.as_ref();
    let relative_pipeline_path = validate_pipeline_repo_relative_path(repo_root, pipeline_path)?;
    let full_path = repo_root.join(relative_pipeline_path.as_str());
    let contents =
        std::fs::read_to_string(&full_path).map_err(|source| PipelineLoadError::ReadFailure {
            path: full_path.clone(),
            source,
        })?;

    let mut docs = serde_yaml_bw::Deserializer::from_str(&contents);
    let header_doc = docs.next();
    let body_doc = docs.next();

    if header_doc.is_none() {
        return Err(PipelineLoadError::WrongDocumentCount {
            path: full_path,
            actual: 0,
        });
    }
    if body_doc.is_none() {
        return Err(PipelineLoadError::WrongDocumentCount {
            path: full_path,
            actual: 1,
        });
    }

    let header =
        PipelineHeader::deserialize(header_doc.expect("header doc present")).map_err(|source| {
            PipelineLoadError::HeaderParse {
                path: full_path.clone(),
                source,
            }
        })?;
    let body =
        PipelineBody::deserialize(body_doc.expect("body doc present")).map_err(|source| {
            PipelineLoadError::BodyParse {
                path: full_path.clone(),
                source,
            }
        })?;

    if docs.next().is_some() {
        return Err(PipelineLoadError::WrongDocumentCount {
            path: full_path,
            actual: 3,
        });
    }

    validate_pipeline_definition(repo_root, &full_path, &header, &body, mode)?;

    Ok(PipelineDefinition {
        source_path: PathBuf::from(relative_pipeline_path.as_str()),
        header,
        body,
    })
}

#[allow(dead_code)]
fn load_pipeline_definition_with_mode_and_roots(
    repo_root: &Path,
    roots: &PipelineDeclarativeRootsContract,
    pipeline_path: &Path,
    mode: PipelineLoadMode,
) -> Result<PipelineDefinition, PipelineLoadError> {
    let relative_pipeline_path =
        validate_pipeline_repo_relative_path_with_roots(repo_root, roots, pipeline_path)?;
    let full_path = repo_root.join(relative_pipeline_path.as_str());
    let contents =
        std::fs::read_to_string(&full_path).map_err(|source| PipelineLoadError::ReadFailure {
            path: full_path.clone(),
            source,
        })?;

    let mut docs = serde_yaml_bw::Deserializer::from_str(&contents);
    let header_doc = docs.next();
    let body_doc = docs.next();

    if header_doc.is_none() {
        return Err(PipelineLoadError::WrongDocumentCount {
            path: full_path,
            actual: 0,
        });
    }
    if body_doc.is_none() {
        return Err(PipelineLoadError::WrongDocumentCount {
            path: full_path,
            actual: 1,
        });
    }

    let header =
        PipelineHeader::deserialize(header_doc.expect("header doc present")).map_err(|source| {
            PipelineLoadError::HeaderParse {
                path: full_path.clone(),
                source,
            }
        })?;
    let body =
        PipelineBody::deserialize(body_doc.expect("body doc present")).map_err(|source| {
            PipelineLoadError::BodyParse {
                path: full_path.clone(),
                source,
            }
        })?;

    if docs.next().is_some() {
        return Err(PipelineLoadError::WrongDocumentCount {
            path: full_path,
            actual: 3,
        });
    }

    validate_pipeline_definition_with_roots(repo_root, roots, &full_path, &header, &body, mode)?;

    Ok(PipelineDefinition {
        source_path: PathBuf::from(relative_pipeline_path.as_str()),
        header,
        body,
    })
}

impl<'de> Deserialize<'de> for ActivationConditionSet {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct ActivationWhenRaw {
            any: Option<Vec<String>>,
            all: Option<Vec<String>>,
        }

        let raw = ActivationWhenRaw::deserialize(deserializer)?;
        match (raw.any, raw.all) {
            (Some(clauses), None) => Ok(Self {
                clauses: clauses
                    .into_iter()
                    .map(parse_activation_clause)
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(serde::de::Error::custom)?,
                operator: ActivationOperator::Any,
            }),
            (None, Some(clauses)) => Ok(Self {
                clauses: clauses
                    .into_iter()
                    .map(parse_activation_clause)
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(serde::de::Error::custom)?,
                operator: ActivationOperator::All,
            }),
            (Some(_), Some(_)) => Err(serde::de::Error::custom(
                "activation.when must contain exactly one of `any` or `all`",
            )),
            (None, None) => Err(serde::de::Error::custom(
                "activation.when must contain exactly one of `any` or `all`",
            )),
        }
    }
}

impl fmt::Display for PipelineLoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PipelineLoadError::ReadFailure { path, source } => {
                write!(
                    f,
                    "failed to read pipeline YAML at {}: {source}",
                    path.display()
                )
            }
            PipelineLoadError::UnsupportedPipelinePath { path, reason } => {
                write!(f, "unsupported pipeline path {}: {reason}", path.display())
            }
            PipelineLoadError::WrongDocumentCount { path, actual } => write!(
                f,
                "pipeline YAML at {} must contain exactly 2 documents; found {}",
                path.display(),
                actual
            ),
            PipelineLoadError::HeaderParse { path, source } => {
                write!(
                    f,
                    "failed to parse pipeline header in {}: {source}",
                    path.display()
                )
            }
            PipelineLoadError::BodyParse { path, source } => {
                write!(
                    f,
                    "failed to parse pipeline body in {}: {source}",
                    path.display()
                )
            }
            PipelineLoadError::Validation { path, error } => {
                write!(
                    f,
                    "invalid pipeline definition at {}: {error}",
                    path.display()
                )
            }
        }
    }
}

impl std::error::Error for PipelineLoadError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            PipelineLoadError::ReadFailure { source, .. } => Some(source),
            PipelineLoadError::HeaderParse { source, .. } => Some(source),
            PipelineLoadError::BodyParse { source, .. } => Some(source),
            PipelineLoadError::UnsupportedPipelinePath { .. }
            | PipelineLoadError::WrongDocumentCount { .. }
            | PipelineLoadError::Validation { .. } => None,
        }
    }
}

impl fmt::Display for PipelineValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PipelineValidationError::EmptyField { field } => {
                write!(f, "field `{field}` must not be empty")
            }
            PipelineValidationError::InvalidCanonicalId {
                field,
                value,
                reason,
            } => write!(
                f,
                "field `{field}` has invalid canonical id `{value}`: {reason}"
            ),
            PipelineValidationError::UnsupportedKind { actual } => {
                write!(f, "field `kind` must be `pipeline`, got `{actual}`")
            }
            PipelineValidationError::EmptyStages => {
                write!(f, "pipeline must declare at least one stage")
            }
            PipelineValidationError::DuplicateStageId { stage_id } => {
                write!(f, "duplicate stage id `{stage_id}`")
            }
            PipelineValidationError::EmptySetsList { stage_id } => {
                write!(
                    f,
                    "stage `{stage_id}` must not declare an empty `sets` list"
                )
            }
            PipelineValidationError::EmptySetVariable { stage_id, index } => write!(
                f,
                "stage `{stage_id}` has an empty `sets` entry at index {index}"
            ),
            PipelineValidationError::InvalidSetVariable {
                stage_id,
                variable,
                reason,
            } => write!(
                f,
                "stage `{stage_id}` has an out-of-contract `sets` variable `{variable}`: {reason}"
            ),
            PipelineValidationError::StageFileOutsideRepoRoot { stage_id, file } => write!(
                f,
                "stage `{stage_id}` file `{file}` resolves outside the repo root"
            ),
            PipelineValidationError::InvalidStageFile {
                stage_id,
                file,
                reason,
            } => write!(f, "stage `{stage_id}` file `{file}` is invalid: {reason}"),
            PipelineValidationError::InvalidActivation { stage_id, reason } => {
                write!(f, "stage `{stage_id}` has invalid activation: {reason}")
            }
            PipelineValidationError::InvalidStageFrontMatter {
                stage_id,
                file,
                detail,
            } => write!(
                f,
                "stage `{stage_id}` file `{file}` has invalid front matter: {detail}"
            ),
            PipelineValidationError::ActivationDrift {
                stage_id,
                file,
                detail,
            } => write!(
                f,
                "stage `{stage_id}` file `{file}` has activation drift: {detail}"
            ),
        }
    }
}

impl fmt::Display for StageFileValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StageFileValidationError::OutsideStageDirectory { stage_root } => {
                f.write_str(&stage_file_outside_directory_reason(stage_root))
            }
            StageFileValidationError::WrongExtension => {
                write!(f, "must use the `.md` extension")
            }
            StageFileValidationError::Missing => {
                write!(f, "must reference an existing file")
            }
            StageFileValidationError::NotRegularFile => {
                write!(f, "must reference an existing regular file")
            }
        }
    }
}

impl fmt::Display for ActivationValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ActivationValidationError::EmptyConditionList { operator } => write!(
                f,
                "`activation.when.{}` must contain at least one clause",
                operator.label()
            ),
            ActivationValidationError::UnsupportedClause { clause, detail } => {
                write!(f, "unsupported clause `{clause}`: {detail}")
            }
        }
    }
}

fn stage_file_outside_directory_reason(stage_root: &str) -> String {
    format!("must live under `{stage_root}/`")
}

fn pipeline_yaml_root_reason() -> &'static str {
    pipeline_yaml_root_reason_for(handbook_product_declarative_roots().pipeline_root_relative())
}

fn pipeline_yaml_extension_reason() -> &'static str {
    pipeline_yaml_extension_reason_for(
        handbook_product_declarative_roots().pipeline_root_relative(),
    )
}

fn pipeline_yaml_root_reason_for(pipeline_root: &'static str) -> &'static str {
    cached_pipeline_root_reason(pipeline_root, "pipeline YAML must live under `{}`")
}

fn pipeline_yaml_extension_reason_for(pipeline_root: &'static str) -> &'static str {
    cached_pipeline_root_reason(
        pipeline_root,
        "pipeline YAML must use the `.yaml` extension under `{}`",
    )
}

fn cached_pipeline_root_reason(
    pipeline_root: &'static str,
    template: &'static str,
) -> &'static str {
    static REASONS: OnceLock<Mutex<BTreeMap<(&'static str, &'static str), &'static str>>> =
        OnceLock::new();
    let reasons = REASONS.get_or_init(|| Mutex::new(BTreeMap::new()));

    {
        let guard = reasons.lock().expect("pipeline root reason cache");
        if let Some(reason) = guard.get(&(template, pipeline_root)) {
            return reason;
        }
    }

    let leaked_reason = Box::leak(
        template
            .replace("{}", &format!("{pipeline_root}/"))
            .into_boxed_str(),
    );
    let mut guard = reasons.lock().expect("pipeline root reason cache");
    guard
        .entry((template, pipeline_root))
        .or_insert(leaked_reason)
}

fn validate_pipeline_definition(
    repo_root: &Path,
    path: &Path,
    header: &PipelineHeader,
    body: &PipelineBody,
    mode: PipelineLoadMode,
) -> Result<(), PipelineLoadError> {
    validate_non_empty(path, "kind", &header.kind)?;
    if header.kind != "pipeline" {
        return Err(PipelineLoadError::Validation {
            path: path.to_path_buf(),
            error: PipelineValidationError::UnsupportedKind {
                actual: header.kind.clone(),
            },
        });
    }
    validate_non_empty(path, "id", &header.id)?;
    validate_canonical_id(path, "id", &header.id)?;
    validate_non_empty(path, "version", &header.version)?;
    validate_non_empty(path, "title", &header.title)?;
    validate_non_empty(path, "description", &header.description)?;
    validate_non_empty(path, "defaults.runner", &body.defaults.runner)?;
    validate_non_empty(path, "defaults.profile", &body.defaults.profile)?;

    if body.stages.is_empty() {
        return Err(PipelineLoadError::Validation {
            path: path.to_path_buf(),
            error: PipelineValidationError::EmptyStages,
        });
    }

    let mut stage_ids = BTreeSet::new();
    for stage in &body.stages {
        validate_non_empty(path, "stage.id", &stage.id)?;
        validate_canonical_id(path, "stage.id", &stage.id)?;
        validate_non_empty(path, "stage.file", &stage.file)?;

        if !stage_ids.insert(stage.id.clone()) {
            return Err(PipelineLoadError::Validation {
                path: path.to_path_buf(),
                error: PipelineValidationError::DuplicateStageId {
                    stage_id: stage.id.clone(),
                },
            });
        }

        if let Some(sets) = &stage.sets {
            if sets.is_empty() {
                return Err(PipelineLoadError::Validation {
                    path: path.to_path_buf(),
                    error: PipelineValidationError::EmptySetsList {
                        stage_id: stage.id.clone(),
                    },
                });
            }

            for (index, entry) in sets.iter().enumerate() {
                if entry.trim().is_empty() {
                    return Err(PipelineLoadError::Validation {
                        path: path.to_path_buf(),
                        error: PipelineValidationError::EmptySetVariable {
                            stage_id: stage.id.clone(),
                            index,
                        },
                    });
                }
                if let Some(reason) = invalid_route_variable_name_reason(entry) {
                    return Err(PipelineLoadError::Validation {
                        path: path.to_path_buf(),
                        error: PipelineValidationError::InvalidSetVariable {
                            stage_id: stage.id.clone(),
                            variable: entry.clone(),
                            reason,
                        },
                    });
                }
            }
        }

        validate_stage_file(repo_root, path, stage)?;
        validate_stage_activation(path, stage)?;
        if mode == PipelineLoadMode::RouteAware {
            validate_stage_activation_equivalence(repo_root, path, stage)?;
        }
    }

    Ok(())
}

#[allow(dead_code)]
fn validate_pipeline_definition_with_roots(
    repo_root: &Path,
    roots: &PipelineDeclarativeRootsContract,
    path: &Path,
    header: &PipelineHeader,
    body: &PipelineBody,
    mode: PipelineLoadMode,
) -> Result<(), PipelineLoadError> {
    validate_non_empty(path, "kind", &header.kind)?;
    if header.kind != "pipeline" {
        return Err(PipelineLoadError::Validation {
            path: path.to_path_buf(),
            error: PipelineValidationError::UnsupportedKind {
                actual: header.kind.clone(),
            },
        });
    }
    validate_non_empty(path, "id", &header.id)?;
    validate_canonical_id(path, "id", &header.id)?;
    validate_non_empty(path, "version", &header.version)?;
    validate_non_empty(path, "title", &header.title)?;
    validate_non_empty(path, "description", &header.description)?;
    validate_non_empty(path, "defaults.runner", &body.defaults.runner)?;
    validate_non_empty(path, "defaults.profile", &body.defaults.profile)?;

    if body.stages.is_empty() {
        return Err(PipelineLoadError::Validation {
            path: path.to_path_buf(),
            error: PipelineValidationError::EmptyStages,
        });
    }

    let mut stage_ids = BTreeSet::new();
    for stage in &body.stages {
        validate_non_empty(path, "stage.id", &stage.id)?;
        validate_canonical_id(path, "stage.id", &stage.id)?;
        validate_non_empty(path, "stage.file", &stage.file)?;

        if !stage_ids.insert(stage.id.clone()) {
            return Err(PipelineLoadError::Validation {
                path: path.to_path_buf(),
                error: PipelineValidationError::DuplicateStageId {
                    stage_id: stage.id.clone(),
                },
            });
        }

        if let Some(sets) = &stage.sets {
            if sets.is_empty() {
                return Err(PipelineLoadError::Validation {
                    path: path.to_path_buf(),
                    error: PipelineValidationError::EmptySetsList {
                        stage_id: stage.id.clone(),
                    },
                });
            }

            for (index, entry) in sets.iter().enumerate() {
                if entry.trim().is_empty() {
                    return Err(PipelineLoadError::Validation {
                        path: path.to_path_buf(),
                        error: PipelineValidationError::EmptySetVariable {
                            stage_id: stage.id.clone(),
                            index,
                        },
                    });
                }
                if let Some(reason) = invalid_route_variable_name_reason(entry) {
                    return Err(PipelineLoadError::Validation {
                        path: path.to_path_buf(),
                        error: PipelineValidationError::InvalidSetVariable {
                            stage_id: stage.id.clone(),
                            variable: entry.clone(),
                            reason,
                        },
                    });
                }
            }
        }

        validate_stage_file_with_roots(repo_root, roots, path, stage)?;
        validate_stage_activation(path, stage)?;
        if mode == PipelineLoadMode::RouteAware {
            validate_stage_activation_equivalence(repo_root, path, stage)?;
        }
    }

    Ok(())
}

fn validate_non_empty(
    path: &Path,
    field: &'static str,
    value: &str,
) -> Result<(), PipelineLoadError> {
    if value.trim().is_empty() {
        return Err(PipelineLoadError::Validation {
            path: path.to_path_buf(),
            error: PipelineValidationError::EmptyField { field },
        });
    }

    Ok(())
}

fn validate_canonical_id(
    path: &Path,
    field: &'static str,
    value: &str,
) -> Result<(), PipelineLoadError> {
    let Some(reason) = canonical_id_path_like_reason(value) else {
        return Ok(());
    };

    Err(PipelineLoadError::Validation {
        path: path.to_path_buf(),
        error: PipelineValidationError::InvalidCanonicalId {
            field,
            value: value.to_string(),
            reason,
        },
    })
}

fn validate_stage_file(
    repo_root: &Path,
    path: &Path,
    stage: &PipelineStage,
) -> Result<(), PipelineLoadError> {
    let workspace = CompilerWorkspace::new(repo_root);
    let file_path = workspace
        .normalize_repo_relative(&stage.file)
        .map_err(|_| PipelineLoadError::Validation {
            path: path.to_path_buf(),
            error: PipelineValidationError::StageFileOutsideRepoRoot {
                stage_id: stage.id.clone(),
                file: stage.file.clone(),
            },
        })?;
    let file_path_view = Path::new(file_path.as_str());

    let active_roots = handbook_product_declarative_roots();
    if !file_path_view.starts_with(active_roots.stage_root()) {
        return Err(PipelineLoadError::Validation {
            path: path.to_path_buf(),
            error: PipelineValidationError::InvalidStageFile {
                stage_id: stage.id.clone(),
                file: stage.file.clone(),
                reason: StageFileValidationError::OutsideStageDirectory {
                    stage_root: active_roots.stage_root_relative(),
                },
            },
        });
    }

    if file_path_view.extension().and_then(|ext| ext.to_str()) != Some("md") {
        return Err(invalid_stage_file_error(
            path,
            stage,
            StageFileValidationError::WrongExtension,
        ));
    }

    match workspace.trusted_read(&file_path) {
        Ok(_) => Ok(()),
        Err(RepoRelativeFileAccessError::Missing(_)) => Err(invalid_stage_file_error(
            path,
            stage,
            StageFileValidationError::Missing,
        )),
        Err(
            RepoRelativeFileAccessError::SymlinkNotAllowed(_)
            | RepoRelativeFileAccessError::NotRegularFile(_)
            | RepoRelativeFileAccessError::ReadFailure { .. },
        ) => Err(invalid_stage_file_error(
            path,
            stage,
            StageFileValidationError::NotRegularFile,
        )),
        Err(RepoRelativeFileAccessError::InvalidPath(_)) => {
            unreachable!("stage file path was already normalized successfully")
        }
    }
}

fn invalid_stage_file_error(
    path: &Path,
    stage: &PipelineStage,
    reason: StageFileValidationError,
) -> PipelineLoadError {
    PipelineLoadError::Validation {
        path: path.to_path_buf(),
        error: PipelineValidationError::InvalidStageFile {
            stage_id: stage.id.clone(),
            file: stage.file.clone(),
            reason,
        },
    }
}

#[allow(dead_code)]
fn validate_stage_file_with_roots(
    repo_root: &Path,
    roots: &PipelineDeclarativeRootsContract,
    path: &Path,
    stage: &PipelineStage,
) -> Result<(), PipelineLoadError> {
    let workspace = CompilerWorkspace::new(repo_root);
    let file_path = workspace
        .normalize_repo_relative(&stage.file)
        .map_err(|_| PipelineLoadError::Validation {
            path: path.to_path_buf(),
            error: PipelineValidationError::StageFileOutsideRepoRoot {
                stage_id: stage.id.clone(),
                file: stage.file.clone(),
            },
        })?;
    let file_path_view = Path::new(file_path.as_str());

    if !file_path_view.starts_with(roots.stage_root()) {
        return Err(PipelineLoadError::Validation {
            path: path.to_path_buf(),
            error: PipelineValidationError::InvalidStageFile {
                stage_id: stage.id.clone(),
                file: stage.file.clone(),
                reason: StageFileValidationError::OutsideStageDirectory {
                    stage_root: roots.stage_root_relative(),
                },
            },
        });
    }

    if file_path_view.extension().and_then(|ext| ext.to_str()) != Some("md") {
        return Err(invalid_stage_file_error(
            path,
            stage,
            StageFileValidationError::WrongExtension,
        ));
    }

    match workspace.trusted_read(&file_path) {
        Ok(_) => Ok(()),
        Err(RepoRelativeFileAccessError::Missing(_)) => Err(invalid_stage_file_error(
            path,
            stage,
            StageFileValidationError::Missing,
        )),
        Err(
            RepoRelativeFileAccessError::SymlinkNotAllowed(_)
            | RepoRelativeFileAccessError::NotRegularFile(_)
            | RepoRelativeFileAccessError::ReadFailure { .. },
        ) => Err(invalid_stage_file_error(
            path,
            stage,
            StageFileValidationError::NotRegularFile,
        )),
        Err(RepoRelativeFileAccessError::InvalidPath(_)) => {
            unreachable!("stage file path was already normalized successfully")
        }
    }
}

fn validate_stage_activation(path: &Path, stage: &PipelineStage) -> Result<(), PipelineLoadError> {
    let Some(activation) = &stage.activation else {
        return Ok(());
    };

    if activation.when.clauses.is_empty() {
        return Err(PipelineLoadError::Validation {
            path: path.to_path_buf(),
            error: PipelineValidationError::InvalidActivation {
                stage_id: stage.id.clone(),
                reason: ActivationValidationError::EmptyConditionList {
                    operator: activation.when.operator,
                },
            },
        });
    }

    Ok(())
}

fn validate_stage_activation_equivalence(
    repo_root: &Path,
    path: &Path,
    stage: &PipelineStage,
) -> Result<(), PipelineLoadError> {
    let stage_front_matter = load_stage_front_matter(repo_root, Path::new(&stage.file))
        .map_err(|err| invalid_stage_front_matter_error(path, stage, err))?;

    let Some(stage_front_matter) = stage_front_matter else {
        return Ok(());
    };

    let Some(front_matter_activation) = stage_front_matter.activation.as_ref() else {
        return Ok(());
    };

    let Some(pipeline_activation) = stage.activation.as_ref() else {
        return Err(PipelineLoadError::Validation {
            path: path.to_path_buf(),
            error: PipelineValidationError::ActivationDrift {
                stage_id: stage.id.clone(),
                file: stage.file.clone(),
                detail: format!(
                    "stage front matter declares {} but pipeline YAML does not",
                    render_activation(front_matter_activation)
                ),
            },
        });
    };

    if activation_equivalent(front_matter_activation, pipeline_activation) {
        return Ok(());
    }

    Err(PipelineLoadError::Validation {
        path: path.to_path_buf(),
        error: PipelineValidationError::ActivationDrift {
            stage_id: stage.id.clone(),
            file: stage.file.clone(),
            detail: format!(
                "pipeline YAML {} does not match stage front matter {}",
                render_activation(pipeline_activation),
                render_activation(front_matter_activation)
            ),
        },
    })
}

fn invalid_stage_front_matter_error(
    path: &Path,
    stage: &PipelineStage,
    err: StageFrontMatterLoadError,
) -> PipelineLoadError {
    let detail = match err {
        StageFrontMatterLoadError::Read(source) => {
            format!("failed to read stage front matter: {source}")
        }
        StageFrontMatterLoadError::Parse(source) => {
            format!("failed to parse stage front matter: {source}")
        }
    };

    PipelineLoadError::Validation {
        path: path.to_path_buf(),
        error: PipelineValidationError::InvalidStageFrontMatter {
            stage_id: stage.id.clone(),
            file: stage.file.clone(),
            detail,
        },
    }
}

fn activation_equivalent(left: &StageActivation, right: &StageActivation) -> bool {
    left.when.operator == right.when.operator
        && normalized_activation_clauses(left) == normalized_activation_clauses(right)
}

fn normalized_activation_clauses(activation: &StageActivation) -> BTreeSet<(String, bool)> {
    activation
        .when
        .clauses
        .iter()
        .map(|clause| (clause.variable.clone(), clause.value))
        .collect()
}

fn render_activation(activation: &StageActivation) -> String {
    let clauses = normalized_activation_clauses(activation)
        .into_iter()
        .map(|(variable, value)| format!("variables.{variable} == {value}"))
        .collect::<Vec<_>>()
        .join(", ");
    format!(
        "activation.when.{} [{}]",
        activation.when.operator.label(),
        clauses
    )
}

fn render_sets(sets: &[String]) -> String {
    format!("[{}]", sets.join(", "))
}

fn load_stage_front_matter(
    repo_root: &Path,
    relative_path: &Path,
) -> Result<Option<StageFrontMatter>, StageFrontMatterLoadError> {
    let workspace = CompilerWorkspace::new(repo_root);
    let relative_path = workspace
        .normalize_repo_relative_path(relative_path)
        .map_err(stage_front_matter_path_error)?;
    let contents = workspace
        .read_string(&relative_path)
        .map_err(stage_front_matter_read_error)?;
    let Some(front_matter_text) = extract_front_matter_block(&contents) else {
        return Ok(None);
    };

    let front_matter = serde_yaml_bw::from_str::<StageFrontMatter>(&front_matter_text)
        .map_err(StageFrontMatterLoadError::Parse)?;

    Ok(Some(front_matter))
}

fn validate_pipeline_repo_relative_path(
    repo_root: &Path,
    path: &Path,
) -> Result<NormalizedRepoRelativePath, PipelineLoadError> {
    let workspace = CompilerWorkspace::new(repo_root);
    let relative_path = workspace
        .normalize_repo_relative_path(path)
        .map_err(|reason| PipelineLoadError::UnsupportedPipelinePath {
            path: path.to_path_buf(),
            reason: pipeline_path_reason(&reason),
        })?;
    let relative_path_view = Path::new(relative_path.as_str());

    let active_roots = handbook_product_declarative_roots();
    if !relative_path_view.starts_with(active_roots.pipeline_root()) {
        return Err(PipelineLoadError::UnsupportedPipelinePath {
            path: path.to_path_buf(),
            reason: pipeline_yaml_root_reason(),
        });
    }

    if relative_path_view.extension().and_then(|ext| ext.to_str()) != Some("yaml") {
        return Err(PipelineLoadError::UnsupportedPipelinePath {
            path: path.to_path_buf(),
            reason: pipeline_yaml_extension_reason(),
        });
    }

    Ok(relative_path)
}

#[allow(dead_code)]
fn validate_pipeline_repo_relative_path_with_roots(
    repo_root: &Path,
    roots: &PipelineDeclarativeRootsContract,
    path: &Path,
) -> Result<NormalizedRepoRelativePath, PipelineLoadError> {
    let workspace = CompilerWorkspace::new(repo_root);
    let relative_path = workspace
        .normalize_repo_relative_path(path)
        .map_err(|reason| PipelineLoadError::UnsupportedPipelinePath {
            path: path.to_path_buf(),
            reason: pipeline_path_reason(&reason),
        })?;
    let relative_path_view = Path::new(relative_path.as_str());

    if !relative_path_view.starts_with(roots.pipeline_root()) {
        return Err(PipelineLoadError::UnsupportedPipelinePath {
            path: path.to_path_buf(),
            reason: pipeline_yaml_root_reason_for(roots.pipeline_root_relative()),
        });
    }

    if relative_path_view.extension().and_then(|ext| ext.to_str()) != Some("yaml") {
        return Err(PipelineLoadError::UnsupportedPipelinePath {
            path: path.to_path_buf(),
            reason: pipeline_yaml_extension_reason_for(roots.pipeline_root_relative()),
        });
    }

    Ok(relative_path)
}

fn pipeline_path_reason(reason: &str) -> &'static str {
    match reason {
        "path must not be empty" => "path must not be empty",
        "path must not escape the repo root" => "path must not escape the repo root",
        "path must be repo-relative" => "path must be repo-relative",
        _ => "path must be repo-relative",
    }
}

fn compile_stage_read_failure(
    path: PathBuf,
    err: RepoRelativeFileAccessError,
) -> CompileStageLoadError {
    CompileStageLoadError::ReadFailure {
        path,
        source: repo_file_access_error_to_io_error(err),
    }
}

fn stage_front_matter_path_error(reason: String) -> StageFrontMatterLoadError {
    StageFrontMatterLoadError::Read(std::io::Error::new(
        std::io::ErrorKind::InvalidInput,
        reason,
    ))
}

fn stage_front_matter_read_error(err: RepoRelativeFileAccessError) -> StageFrontMatterLoadError {
    StageFrontMatterLoadError::Read(repo_file_access_error_to_io_error(err))
}

fn repo_file_access_error_to_io_error(err: RepoRelativeFileAccessError) -> std::io::Error {
    match err {
        RepoRelativeFileAccessError::Missing(_) => std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "repo-relative file is missing",
        ),
        RepoRelativeFileAccessError::InvalidPath(reason) => {
            std::io::Error::new(std::io::ErrorKind::InvalidInput, reason)
        }
        RepoRelativeFileAccessError::SymlinkNotAllowed(path) => std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!(
                "repo-relative file must not be a symlink: {}",
                path.display()
            ),
        ),
        RepoRelativeFileAccessError::NotRegularFile(path) => std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!(
                "repo-relative file must be an existing regular file: {}",
                path.display()
            ),
        ),
        RepoRelativeFileAccessError::ReadFailure { source, .. } => source,
    }
}

fn invalid_route_variable_name_reason(variable: &str) -> Option<&'static str> {
    let mut chars = variable.chars();
    let Some(first) = chars.next() else {
        return Some("variable name must not be empty");
    };

    if !(first.is_ascii_alphabetic() || first == '_') {
        return Some(
            "variable name must start with an ASCII letter or `_` and continue with ASCII alphanumeric or `_` characters",
        );
    }

    if chars.all(|ch| ch.is_ascii_alphanumeric() || ch == '_') {
        None
    } else {
        Some(
            "variable name must start with an ASCII letter or `_` and continue with ASCII alphanumeric or `_` characters",
        )
    }
}

fn parse_activation_clause(input: String) -> Result<ActivationClause, String> {
    let clause = input.trim();
    let Some((left, right)) = clause.split_once("==") else {
        return Err(format!(
            "unsupported clause `{clause}`: exactly one equality operator is required"
        ));
    };
    let left = left.trim();
    let right = right.trim();

    if right.contains("==") {
        return Err(format!(
            "unsupported clause `{clause}`: only one equality operator is supported"
        ));
    }

    if !is_supported_variable_path(left) {
        return Err(format!(
            "unsupported clause `{clause}`: left-hand side must match `variables.<name>`"
        ));
    }

    let value = parse_activation_bool(right).ok_or_else(|| {
        format!(
            "unsupported clause `{clause}`: reduced v1 supports only boolean activation values (`true` or `false`)"
        )
    })?;

    Ok(ActivationClause {
        variable: left.trim_start_matches("variables.").to_string(),
        value,
    })
}

fn is_supported_variable_path(input: &str) -> bool {
    let Some(rest) = input.strip_prefix("variables.") else {
        return false;
    };

    let mut chars = rest.chars();
    let Some(first) = chars.next() else {
        return false;
    };

    if !(first.is_ascii_alphabetic() || first == '_') {
        return false;
    }

    chars.all(|ch| ch.is_ascii_alphanumeric() || ch == '_')
}

fn parse_activation_bool(input: &str) -> Option<bool> {
    match input {
        "true" => Some(true),
        "false" => Some(false),
        _ => None,
    }
}

impl ActivationOperator {
    fn label(self) -> &'static str {
        match self {
            ActivationOperator::Any => "any",
            ActivationOperator::All => "all",
        }
    }
}
