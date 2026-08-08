use crate::{CanonicalCharter, DefinitionFingerprint};
use serde::Serialize;
use serde_json::{json, Value};

const CANONICAL_CHARTER_REF: &str = ".handbook/project/charter.yaml";
const MAX_CANONICAL_CHARTER_BYTES: usize = 1_048_576;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct PostureDimensionBinding {
    pub(crate) dimension_id: &'static str,
    pub(crate) index: usize,
    pub(crate) authority_path: &'static str,
}

const POSTURE_DIMENSION_BINDINGS: [PostureDimensionBinding; 9] = [
    PostureDimensionBinding {
        dimension_id: "speed_vs_quality",
        index: 0,
        authority_path: "/engineering_posture/dimensions/0/level_override",
    },
    PostureDimensionBinding {
        dimension_id: "type_safety_static_analysis",
        index: 1,
        authority_path: "/engineering_posture/dimensions/1/level_override",
    },
    PostureDimensionBinding {
        dimension_id: "testing_rigor",
        index: 2,
        authority_path: "/engineering_posture/dimensions/2/level_override",
    },
    PostureDimensionBinding {
        dimension_id: "scalability_performance",
        index: 3,
        authority_path: "/engineering_posture/dimensions/3/level_override",
    },
    PostureDimensionBinding {
        dimension_id: "reliability_operability",
        index: 4,
        authority_path: "/engineering_posture/dimensions/4/level_override",
    },
    PostureDimensionBinding {
        dimension_id: "security_privacy",
        index: 5,
        authority_path: "/engineering_posture/dimensions/5/level_override",
    },
    PostureDimensionBinding {
        dimension_id: "observability",
        index: 6,
        authority_path: "/engineering_posture/dimensions/6/level_override",
    },
    PostureDimensionBinding {
        dimension_id: "dx_tooling_automation",
        index: 7,
        authority_path: "/engineering_posture/dimensions/7/level_override",
    },
    PostureDimensionBinding {
        dimension_id: "ux_polish_api_usability",
        index: 8,
        authority_path: "/engineering_posture/dimensions/8/level_override",
    },
];

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum PostureRefusal {
    InvalidCanonicalByteLength,
    InvalidBaselineLevel,
    InvalidStoredLevel,
    InvalidEffectiveLevel,
    UnknownDimension,
    DimensionIndexMismatch,
    AuthorityPathMismatch,
    CharterDimensionMapMismatch,
    ExpectedStoredValueMismatch,
    ExpectedEffectiveLevelMismatch,
    SameEffectiveLevel,
    DeepDiffMismatch,
    CompareAndSwapMismatch,
    KernelReplayMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ExactCanonicalBytes {
    pub(crate) bytes: Vec<u8>,
    pub(crate) byte_length: u64,
    pub(crate) document_sha256: String,
    pub(crate) canonical_fingerprint: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PostureChangeRequest {
    pub(crate) dimension_id: String,
    pub(crate) dimension_index: u8,
    pub(crate) authority_path: String,
    pub(crate) expected_stored_value: Option<u8>,
    pub(crate) expected_effective_level: u8,
    pub(crate) proposed_effective_level: u8,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PreparedPostureChange {
    pub(crate) current_canonical: ExactCanonicalBytes,
    pub(crate) change: PreparedPostureLeafChange,
    pub(crate) resulting_charter: CanonicalCharter,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PreparedPostureLeafChange {
    pub(crate) dimension_id: String,
    pub(crate) dimension_index: u8,
    pub(crate) authority_path: String,
    pub(crate) baseline_level: u8,
    pub(crate) expected_stored_value: Option<u8>,
    pub(crate) expected_effective_level: u8,
    pub(crate) proposed_stored_value: Option<u8>,
    pub(crate) proposed_effective_level: u8,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub(crate) struct ProjectPostureDimension {
    pub(crate) dimension_id: String,
    pub(crate) effective_level: u8,
    pub(crate) red_line_refs: Vec<String>,
    pub(crate) trigger_refs: Vec<String>,
    pub(crate) allowed_shortcut_refs: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub(crate) struct ProjectPostureKernelReplayPlan {
    pub(crate) constitutional_artifact_ref: String,
    pub(crate) source_authority_fingerprint: String,
    pub(crate) dimensions: Vec<ProjectPostureDimension>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ProjectPostureKernel {
    pub(crate) baseline_level: u8,
    pub(crate) dimensions: Vec<ProjectPostureDimension>,
    pub(crate) input_fingerprint: String,
    pub(crate) kernel_fingerprint: String,
    pub(crate) replay_plan: ProjectPostureKernelReplayPlan,
}

#[derive(Serialize)]
struct KernelInputFingerprint<'a> {
    schema_id: &'static str,
    schema_version: &'static str,
    constitutional_artifact_ref: &'static str,
    source_authority_fingerprint: &'a str,
}

#[derive(Serialize)]
struct KernelFingerprint<'a> {
    input_fingerprint: &'a str,
    baseline_level: u8,
    dimensions: &'a [ProjectPostureDimension],
}

pub(crate) fn fixed_dimension_binding(dimension_id: &str) -> Option<PostureDimensionBinding> {
    POSTURE_DIMENSION_BINDINGS
        .iter()
        .copied()
        .find(|binding| binding.dimension_id == dimension_id)
}

pub(crate) fn bind_exact_canonical_bytes(
    bytes: &[u8],
) -> Result<ExactCanonicalBytes, PostureRefusal> {
    if bytes.is_empty() || bytes.len() > MAX_CANONICAL_CHARTER_BYTES {
        return Err(PostureRefusal::InvalidCanonicalByteLength);
    }

    let fingerprint = DefinitionFingerprint::from_bytes(bytes).to_string();
    Ok(ExactCanonicalBytes {
        bytes: bytes.to_vec(),
        byte_length: bytes.len() as u64,
        document_sha256: fingerprint.clone(),
        canonical_fingerprint: fingerprint,
    })
}

pub(crate) fn derive_project_posture_kernel(
    charter: &CanonicalCharter,
    canonical: &ExactCanonicalBytes,
) -> Result<ProjectPostureKernel, PostureRefusal> {
    validate_exact_canonical_bytes(canonical)?;
    validate_charter_dimension_map(charter)?;

    let dimensions = charter
        .engineering_posture
        .dimensions
        .iter()
        .zip(POSTURE_DIMENSION_BINDINGS)
        .map(|(dimension, binding)| ProjectPostureDimension {
            dimension_id: binding.dimension_id.to_owned(),
            effective_level: dimension
                .level_override
                .unwrap_or(charter.posture.baseline_level),
            red_line_refs: dimension.red_lines.clone(),
            trigger_refs: dimension.raise_the_bar_triggers.clone(),
            allowed_shortcut_refs: dimension.allowed_shortcuts.clone(),
        })
        .collect::<Vec<_>>();
    let replay_plan = ProjectPostureKernelReplayPlan {
        constitutional_artifact_ref: CANONICAL_CHARTER_REF.to_owned(),
        source_authority_fingerprint: canonical.canonical_fingerprint.clone(),
        dimensions: dimensions.clone(),
    };
    let input_fingerprint = fingerprint_json(&KernelInputFingerprint {
        schema_id: "handbook.project-posture-kernel",
        schema_version: "1.0",
        constitutional_artifact_ref: CANONICAL_CHARTER_REF,
        source_authority_fingerprint: &canonical.canonical_fingerprint,
    })?;
    let kernel_fingerprint = fingerprint_json(&KernelFingerprint {
        input_fingerprint: &input_fingerprint,
        baseline_level: charter.posture.baseline_level,
        dimensions: &dimensions,
    })?;

    Ok(ProjectPostureKernel {
        baseline_level: charter.posture.baseline_level,
        dimensions,
        input_fingerprint,
        kernel_fingerprint,
        replay_plan,
    })
}

pub(crate) fn replay_project_posture_kernel(
    plan: &ProjectPostureKernelReplayPlan,
    charter: &CanonicalCharter,
    canonical: &ExactCanonicalBytes,
) -> Result<ProjectPostureKernel, PostureRefusal> {
    if plan.constitutional_artifact_ref != CANONICAL_CHARTER_REF
        || plan.source_authority_fingerprint != canonical.canonical_fingerprint
    {
        return Err(PostureRefusal::KernelReplayMismatch);
    }

    let kernel = derive_project_posture_kernel(charter, canonical)?;
    if kernel.replay_plan != *plan {
        return Err(PostureRefusal::KernelReplayMismatch);
    }
    Ok(kernel)
}

pub(crate) fn prepare_posture_change(
    current: &CanonicalCharter,
    current_canonical: ExactCanonicalBytes,
    request: PostureChangeRequest,
) -> Result<PreparedPostureChange, PostureRefusal> {
    validate_exact_canonical_bytes(&current_canonical)?;
    validate_charter_dimension_map(current)?;

    let binding =
        fixed_dimension_binding(&request.dimension_id).ok_or(PostureRefusal::UnknownDimension)?;
    if request.dimension_index as usize != binding.index {
        return Err(PostureRefusal::DimensionIndexMismatch);
    }
    if request.authority_path != binding.authority_path {
        return Err(PostureRefusal::AuthorityPathMismatch);
    }
    if !is_level(request.expected_effective_level) || !is_level(request.proposed_effective_level) {
        return Err(PostureRefusal::InvalidEffectiveLevel);
    }
    if request
        .expected_stored_value
        .is_some_and(|level| !is_level(level))
    {
        return Err(PostureRefusal::InvalidStoredLevel);
    }

    let baseline_level = current.posture.baseline_level;
    let actual_stored_value = current.engineering_posture.dimensions[binding.index].level_override;
    let actual_effective_level = actual_stored_value.unwrap_or(baseline_level);
    if request.expected_stored_value != actual_stored_value {
        return Err(PostureRefusal::ExpectedStoredValueMismatch);
    }
    if request.expected_effective_level != actual_effective_level {
        return Err(PostureRefusal::ExpectedEffectiveLevelMismatch);
    }
    if request.proposed_effective_level == actual_effective_level {
        return Err(PostureRefusal::SameEffectiveLevel);
    }

    let proposed_stored_value = (request.proposed_effective_level != baseline_level)
        .then_some(request.proposed_effective_level);
    let mut resulting_charter = current.clone();
    resulting_charter.engineering_posture.dimensions[binding.index].level_override =
        proposed_stored_value;
    verify_one_leaf_deep_diff(
        current,
        &resulting_charter,
        binding,
        actual_stored_value,
        proposed_stored_value,
    )?;

    Ok(PreparedPostureChange {
        current_canonical,
        change: PreparedPostureLeafChange {
            dimension_id: binding.dimension_id.to_owned(),
            dimension_index: binding.index as u8,
            authority_path: binding.authority_path.to_owned(),
            baseline_level,
            expected_stored_value: actual_stored_value,
            expected_effective_level: actual_effective_level,
            proposed_stored_value,
            proposed_effective_level: request.proposed_effective_level,
        },
        resulting_charter,
    })
}

pub(crate) fn verify_one_leaf_deep_diff(
    before: &CanonicalCharter,
    after: &CanonicalCharter,
    binding: PostureDimensionBinding,
    expected_stored_value: Option<u8>,
    proposed_stored_value: Option<u8>,
) -> Result<(), PostureRefusal> {
    validate_charter_dimension_map(before)?;
    validate_charter_dimension_map(after)?;
    let before_value =
        serde_json::to_value(before).map_err(|_| PostureRefusal::DeepDiffMismatch)?;
    let after_value = serde_json::to_value(after).map_err(|_| PostureRefusal::DeepDiffMismatch)?;
    let mut changed_paths = Vec::new();
    collect_deep_diff_paths(&before_value, &after_value, "", &mut changed_paths);
    if changed_paths.len() != 1 || changed_paths[0] != binding.authority_path {
        return Err(PostureRefusal::DeepDiffMismatch);
    }

    let expected_value = stored_value_json(expected_stored_value);
    let proposed_value = stored_value_json(proposed_stored_value);
    if before_value.pointer(binding.authority_path) != Some(&expected_value)
        || after_value.pointer(binding.authority_path) != Some(&proposed_value)
    {
        return Err(PostureRefusal::DeepDiffMismatch);
    }
    Ok(())
}

pub(crate) fn verify_precommit_cas(
    prepared: &PreparedPostureChange,
    live_bytes: &[u8],
) -> Result<(), PostureRefusal> {
    let retained = &prepared.current_canonical;
    if live_bytes.len() as u64 != retained.byte_length
        || DefinitionFingerprint::from_bytes(live_bytes).to_string() != retained.document_sha256
        || retained.canonical_fingerprint != retained.document_sha256
        || live_bytes != retained.bytes
    {
        return Err(PostureRefusal::CompareAndSwapMismatch);
    }
    Ok(())
}

fn validate_exact_canonical_bytes(canonical: &ExactCanonicalBytes) -> Result<(), PostureRefusal> {
    if canonical.bytes.is_empty()
        || canonical.bytes.len() > MAX_CANONICAL_CHARTER_BYTES
        || canonical.byte_length != canonical.bytes.len() as u64
    {
        return Err(PostureRefusal::InvalidCanonicalByteLength);
    }
    let fingerprint = DefinitionFingerprint::from_bytes(&canonical.bytes).to_string();
    if canonical.document_sha256 != fingerprint || canonical.canonical_fingerprint != fingerprint {
        return Err(PostureRefusal::CompareAndSwapMismatch);
    }
    Ok(())
}

fn validate_charter_dimension_map(charter: &CanonicalCharter) -> Result<(), PostureRefusal> {
    if !is_level(charter.posture.baseline_level) {
        return Err(PostureRefusal::InvalidBaselineLevel);
    }
    if charter.engineering_posture.dimensions.len() != POSTURE_DIMENSION_BINDINGS.len() {
        return Err(PostureRefusal::CharterDimensionMapMismatch);
    }
    for (dimension, binding) in charter
        .engineering_posture
        .dimensions
        .iter()
        .zip(POSTURE_DIMENSION_BINDINGS)
    {
        if dimension.dimension_id != binding.dimension_id {
            return Err(PostureRefusal::CharterDimensionMapMismatch);
        }
        if dimension
            .level_override
            .is_some_and(|level| !is_level(level))
        {
            return Err(PostureRefusal::InvalidStoredLevel);
        }
    }
    Ok(())
}

fn fingerprint_json<T: Serialize>(value: &T) -> Result<String, PostureRefusal> {
    let value = serde_json::to_value(value).map_err(|_| PostureRefusal::KernelReplayMismatch)?;
    DefinitionFingerprint::from_json_value(&value)
        .map(|fingerprint| fingerprint.to_string())
        .map_err(|_| PostureRefusal::KernelReplayMismatch)
}

fn is_level(level: u8) -> bool {
    (1..=5).contains(&level)
}

fn stored_value_json(value: Option<u8>) -> Value {
    value.map_or(Value::Null, |level| json!(level))
}

fn collect_deep_diff_paths(
    before: &Value,
    after: &Value,
    path: &str,
    changed_paths: &mut Vec<String>,
) {
    match (before, after) {
        (Value::Object(before), Value::Object(after)) if before.len() == after.len() => {
            for (key, before_value) in before {
                let Some(after_value) = after.get(key) else {
                    changed_paths.push(path.to_owned());
                    return;
                };
                collect_deep_diff_paths(
                    before_value,
                    after_value,
                    &pointer_child(path, key),
                    changed_paths,
                );
            }
        }
        (Value::Array(before), Value::Array(after)) if before.len() == after.len() => {
            for (index, (before_value, after_value)) in before.iter().zip(after).enumerate() {
                collect_deep_diff_paths(
                    before_value,
                    after_value,
                    &pointer_child(path, &index.to_string()),
                    changed_paths,
                );
            }
        }
        _ if before != after => changed_paths.push(path.to_owned()),
        _ => {}
    }
}

fn pointer_child(parent: &str, token: &str) -> String {
    let escaped = token.replace('~', "~0").replace('/', "~1");
    format!("{parent}/{escaped}")
}
