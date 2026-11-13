use crate::bounds::*;
use crate::canonical::{canonical_bytes, canonical_fingerprint};
use crate::DockErrorV1;
use handbook_contracts::{
    ContractIdV1, ExactBindingV1, ExactRefV1, FingerprintV1, ResolutionEnvelopeV1, StableIdV1,
    VersionV1,
};
use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize};

pub const DOCK_OWNER_SOURCE_REF_V1: &str = "handbook.owner.dock@0.1.0";
pub const DOCK_PROCESS_PROTOCOL_REF_V1: &str = "handbook.dock.process-json@1.0.0";
pub const DOCK_MANIFEST_SCHEMA_REF_V1: &str = "handbook.dock-manifest@1.0.0";
pub const DOCK_PROCESS_REQUEST_SCHEMA_REF_V1: &str = "handbook.dock-process-request@1.0.0";
pub const DOCK_PROCESS_RESULT_SCHEMA_REF_V1: &str = "handbook.dock-process-result@1.0.0";
pub const DOCK_EXECUTION_RECORD_SCHEMA_REF_V1: &str = "handbook.dock-execution-record@1.0.0";

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SafeRelativePathV1(String);

impl SafeRelativePathV1 {
    pub fn parse(value: &str) -> Result<Self, DockErrorV1> {
        let safe = !value.is_empty()
            && value.len() <= MAX_SAFE_RELATIVE_PATH_BYTES
            && !value.starts_with('/')
            && !value.ends_with('/')
            && !value.contains('\\')
            && !value.contains(':')
            && !value.contains('\0')
            && value.split('/').all(valid_path_segment);
        if !safe {
            return Err(DockErrorV1::InvalidIdentity);
        }
        Ok(Self(value.to_owned()))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<'de> Deserialize<'de> for SafeRelativePathV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Self::parse(&String::deserialize(deserializer)?).map_err(D::Error::custom)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct DockBundleFileV1 {
    path: SafeRelativePathV1,
    length: u64,
    sha256: FingerprintV1,
}

impl DockBundleFileV1 {
    pub fn new(
        path: SafeRelativePathV1,
        length: u64,
        sha256: FingerprintV1,
    ) -> Result<Self, DockErrorV1> {
        if length > MAX_BUNDLE_FILE_BYTES {
            return Err(DockErrorV1::BoundExceeded);
        }
        Ok(Self {
            path,
            length,
            sha256,
        })
    }
    pub fn path(&self) -> &SafeRelativePathV1 {
        &self.path
    }
    pub fn length(&self) -> u64 {
        self.length
    }
    pub fn sha256(&self) -> &FingerprintV1 {
        &self.sha256
    }
}

impl<'de> Deserialize<'de> for DockBundleFileV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Raw {
            path: SafeRelativePathV1,
            length: u64,
            sha256: FingerprintV1,
        }
        let raw = Raw::deserialize(deserializer)?;
        Self::new(raw.path, raw.length, raw.sha256).map_err(D::Error::custom)
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct DockTargetTupleV1(String);

impl DockTargetTupleV1 {
    pub fn parse(value: &str) -> Result<Self, DockErrorV1> {
        let valid = !value.is_empty()
            && value.len() <= 255
            && value.is_ascii()
            && value.split('-').count() >= 3
            && value.split('-').all(|part| !part.is_empty())
            && value.bytes().all(|b| {
                b.is_ascii_lowercase() || b.is_ascii_digit() || matches!(b, b'_' | b'-' | b'.')
            });
        if !valid {
            return Err(DockErrorV1::InvalidIdentity);
        }
        Ok(Self(value.to_owned()))
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<'de> Deserialize<'de> for DockTargetTupleV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Self::parse(&String::deserialize(deserializer)?).map_err(D::Error::custom)
    }
}

#[derive(Clone, Debug)]
pub struct DockImplementationBundleInputV1 {
    pub bundle_ref: ExactRefV1,
    pub files: Vec<DockBundleFileV1>,
    pub entrypoint: SafeRelativePathV1,
    pub entrypoint_fingerprint: FingerprintV1,
    pub fixed_arguments: Vec<String>,
    pub target: DockTargetTupleV1,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct DockImplementationBundleV1 {
    bundle_ref: ExactRefV1,
    files: Vec<DockBundleFileV1>,
    entrypoint: SafeRelativePathV1,
    entrypoint_fingerprint: FingerprintV1,
    fixed_arguments: Vec<String>,
    arguments_fingerprint: FingerprintV1,
    target: DockTargetTupleV1,
    bundle_fingerprint: FingerprintV1,
}

impl DockImplementationBundleV1 {
    pub fn new(input: DockImplementationBundleInputV1) -> Result<Self, DockErrorV1> {
        if input.files.is_empty()
            || input.files.len() > MAX_BUNDLE_FILES
            || input.fixed_arguments.len() > MAX_FIXED_ARGUMENTS
        {
            return Err(DockErrorV1::BoundExceeded);
        }
        let mut previous: Option<&SafeRelativePathV1> = None;
        let mut aggregate = 0_u64;
        for file in &input.files {
            if previous.is_some_and(|path| path >= file.path()) {
                return Err(DockErrorV1::InvalidInvariant);
            }
            previous = Some(file.path());
            aggregate = aggregate
                .checked_add(file.length())
                .ok_or(DockErrorV1::BoundExceeded)?;
        }
        if aggregate > MAX_BUNDLE_BYTES
            || input.fixed_arguments.iter().any(|arg| {
                arg.is_empty() || arg.len() > MAX_FIXED_ARGUMENT_BYTES || arg.contains('\0')
            })
        {
            return Err(DockErrorV1::BoundExceeded);
        }
        let entry = input
            .files
            .iter()
            .find(|file| file.path() == &input.entrypoint)
            .ok_or(DockErrorV1::InvalidInvariant)?;
        if entry.sha256() != &input.entrypoint_fingerprint {
            return Err(DockErrorV1::DerivedValueMismatch);
        }
        let arguments_fingerprint = canonical_fingerprint(&input.fixed_arguments)?;
        let preimage = BundlePreimage {
            bundle_ref: &input.bundle_ref,
            files: &input.files,
            entrypoint: &input.entrypoint,
            entrypoint_fingerprint: &input.entrypoint_fingerprint,
            fixed_arguments: &input.fixed_arguments,
            arguments_fingerprint: &arguments_fingerprint,
            target: &input.target,
        };
        let bundle_fingerprint = canonical_fingerprint(&preimage)?;
        Ok(Self {
            bundle_ref: input.bundle_ref,
            files: input.files,
            entrypoint: input.entrypoint,
            entrypoint_fingerprint: input.entrypoint_fingerprint,
            fixed_arguments: input.fixed_arguments,
            arguments_fingerprint,
            target: input.target,
            bundle_fingerprint,
        })
    }
    pub fn bundle_ref(&self) -> &ExactRefV1 {
        &self.bundle_ref
    }
    pub fn bundle_fingerprint(&self) -> &FingerprintV1 {
        &self.bundle_fingerprint
    }
    pub fn files(&self) -> &[DockBundleFileV1] {
        &self.files
    }
    pub fn entrypoint(&self) -> &SafeRelativePathV1 {
        &self.entrypoint
    }
    pub fn entrypoint_fingerprint(&self) -> &FingerprintV1 {
        &self.entrypoint_fingerprint
    }
    pub fn fixed_arguments(&self) -> &[String] {
        &self.fixed_arguments
    }
    pub fn arguments_fingerprint(&self) -> &FingerprintV1 {
        &self.arguments_fingerprint
    }
    pub fn target(&self) -> &DockTargetTupleV1 {
        &self.target
    }
    pub fn binding(&self) -> ExactBindingV1 {
        ExactBindingV1::new(self.bundle_ref.clone(), self.bundle_fingerprint.clone())
    }
}

impl<'de> Deserialize<'de> for DockImplementationBundleV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Raw {
            bundle_ref: ExactRefV1,
            files: Vec<DockBundleFileV1>,
            entrypoint: SafeRelativePathV1,
            entrypoint_fingerprint: FingerprintV1,
            fixed_arguments: Vec<String>,
            arguments_fingerprint: FingerprintV1,
            target: DockTargetTupleV1,
            bundle_fingerprint: FingerprintV1,
        }
        let raw = Raw::deserialize(deserializer)?;
        let supplied_arguments = raw.arguments_fingerprint;
        let supplied_bundle = raw.bundle_fingerprint;
        let value = Self::new(DockImplementationBundleInputV1 {
            bundle_ref: raw.bundle_ref,
            files: raw.files,
            entrypoint: raw.entrypoint,
            entrypoint_fingerprint: raw.entrypoint_fingerprint,
            fixed_arguments: raw.fixed_arguments,
            target: raw.target,
        })
        .map_err(D::Error::custom)?;
        if value.arguments_fingerprint != supplied_arguments
            || value.bundle_fingerprint != supplied_bundle
        {
            return Err(D::Error::custom(DockErrorV1::DerivedValueMismatch));
        }
        Ok(value)
    }
}

#[derive(Serialize)]
struct BundlePreimage<'a> {
    bundle_ref: &'a ExactRefV1,
    files: &'a [DockBundleFileV1],
    entrypoint: &'a SafeRelativePathV1,
    entrypoint_fingerprint: &'a FingerprintV1,
    fixed_arguments: &'a [String],
    arguments_fingerprint: &'a FingerprintV1,
    target: &'a DockTargetTupleV1,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DockImplementationKindV1 {
    NativeProcess,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct DockInputRoleV1 {
    role: StableIdV1,
    media_type: String,
    schema: ExactBindingV1,
}
impl DockInputRoleV1 {
    pub fn new(
        role: StableIdV1,
        media_type: &str,
        schema: ExactBindingV1,
    ) -> Result<Self, DockErrorV1> {
        if !valid_media_type(media_type) {
            return Err(DockErrorV1::InvalidInvariant);
        }
        Ok(Self {
            role,
            media_type: media_type.to_owned(),
            schema,
        })
    }
    pub fn role(&self) -> &StableIdV1 {
        &self.role
    }
    pub fn media_type(&self) -> &str {
        &self.media_type
    }
    pub fn schema(&self) -> &ExactBindingV1 {
        &self.schema
    }
}
impl<'de> Deserialize<'de> for DockInputRoleV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Raw {
            role: StableIdV1,
            media_type: String,
            schema: ExactBindingV1,
        }
        let raw = Raw::deserialize(deserializer)?;
        Self::new(raw.role, &raw.media_type, raw.schema).map_err(D::Error::custom)
    }
}

#[derive(Clone, Debug)]
pub struct DockManifestInputV1 {
    pub manifest_schema: ExactBindingV1,
    pub dock_id: ContractIdV1,
    pub dock_version: VersionV1,
    pub protocol: ExactBindingV1,
    pub request_schema: ExactBindingV1,
    pub result_schema: ExactBindingV1,
    pub implementation_kind: DockImplementationKindV1,
    pub implementation_bundle: DockImplementationBundleV1,
    pub supported_evidence_kinds: Vec<StableIdV1>,
    pub supported_inputs: Vec<DockInputRoleV1>,
    pub resolution_ceiling: ResolutionEnvelopeV1,
    pub limits: DockProtocolLimitsV1,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct DockManifestV1 {
    manifest_schema: ExactBindingV1,
    dock_id: ContractIdV1,
    dock_version: VersionV1,
    dock_ref: ExactRefV1,
    protocol: ExactBindingV1,
    request_schema: ExactBindingV1,
    result_schema: ExactBindingV1,
    implementation_kind: DockImplementationKindV1,
    implementation_bundle: DockImplementationBundleV1,
    supported_evidence_kinds: Vec<StableIdV1>,
    supported_inputs: Vec<DockInputRoleV1>,
    resolution_ceiling: ResolutionEnvelopeV1,
    limits: DockProtocolLimitsV1,
    deterministic: bool,
    offline: bool,
    read_only: bool,
    manifest_fingerprint: FingerprintV1,
}

impl DockManifestV1 {
    pub fn new(input: DockManifestInputV1) -> Result<Self, DockErrorV1> {
        require_ref(&input.manifest_schema, DOCK_MANIFEST_SCHEMA_REF_V1)?;
        require_ref(&input.protocol, DOCK_PROCESS_PROTOCOL_REF_V1)?;
        require_ref(&input.request_schema, DOCK_PROCESS_REQUEST_SCHEMA_REF_V1)?;
        require_ref(&input.result_schema, DOCK_PROCESS_RESULT_SCHEMA_REF_V1)?;
        validate_strict_ids(&input.supported_evidence_kinds, MAX_CANDIDATE_OBSERVATIONS)?;
        if input.supported_inputs.is_empty() || input.supported_inputs.len() > MAX_INLINE_INPUTS {
            return Err(DockErrorV1::BoundExceeded);
        }
        if input
            .supported_inputs
            .windows(2)
            .any(|pair| pair[0].role() >= pair[1].role())
        {
            return Err(DockErrorV1::InvalidInvariant);
        }
        let dock_ref = ExactRefV1::derive(&input.dock_id, &input.dock_version);
        let preimage = ManifestPreimage {
            manifest_schema: &input.manifest_schema,
            dock_id: &input.dock_id,
            dock_version: &input.dock_version,
            dock_ref: &dock_ref,
            protocol: &input.protocol,
            request_schema: &input.request_schema,
            result_schema: &input.result_schema,
            implementation_kind: input.implementation_kind,
            implementation_bundle: &input.implementation_bundle,
            supported_evidence_kinds: &input.supported_evidence_kinds,
            supported_inputs: &input.supported_inputs,
            resolution_ceiling: &input.resolution_ceiling,
            limits: &input.limits,
            deterministic: true,
            offline: true,
            read_only: true,
        };
        let manifest_fingerprint = canonical_fingerprint(&preimage)?;
        Ok(Self {
            manifest_schema: input.manifest_schema,
            dock_id: input.dock_id,
            dock_version: input.dock_version,
            dock_ref,
            protocol: input.protocol,
            request_schema: input.request_schema,
            result_schema: input.result_schema,
            implementation_kind: input.implementation_kind,
            implementation_bundle: input.implementation_bundle,
            supported_evidence_kinds: input.supported_evidence_kinds,
            supported_inputs: input.supported_inputs,
            resolution_ceiling: input.resolution_ceiling,
            limits: input.limits,
            deterministic: true,
            offline: true,
            read_only: true,
            manifest_fingerprint,
        })
    }
    pub fn dock_ref(&self) -> &ExactRefV1 {
        &self.dock_ref
    }
    pub fn manifest_fingerprint(&self) -> &FingerprintV1 {
        &self.manifest_fingerprint
    }
    pub fn protocol(&self) -> &ExactBindingV1 {
        &self.protocol
    }
    pub fn request_schema(&self) -> &ExactBindingV1 {
        &self.request_schema
    }
    pub fn result_schema(&self) -> &ExactBindingV1 {
        &self.result_schema
    }
    pub fn implementation_bundle(&self) -> &DockImplementationBundleV1 {
        &self.implementation_bundle
    }
    pub fn supported_evidence_kinds(&self) -> &[StableIdV1] {
        &self.supported_evidence_kinds
    }
    pub fn supported_inputs(&self) -> &[DockInputRoleV1] {
        &self.supported_inputs
    }
    pub fn resolution_ceiling(&self) -> &ResolutionEnvelopeV1 {
        &self.resolution_ceiling
    }
    pub fn limits(&self) -> &DockProtocolLimitsV1 {
        &self.limits
    }
    pub fn binding(&self) -> ExactBindingV1 {
        ExactBindingV1::new(self.dock_ref.clone(), self.manifest_fingerprint.clone())
    }
    pub fn canonical_json_bytes(&self) -> Result<Vec<u8>, DockErrorV1> {
        canonical_bytes(self)
    }
    pub fn admit_canonical_json(bytes: &[u8]) -> Result<Self, DockErrorV1> {
        crate::canonical::admit_canonical_json(bytes, MAX_PROCESS_JSON_BYTES)
    }
}

impl<'de> Deserialize<'de> for DockManifestV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Raw {
            manifest_schema: ExactBindingV1,
            dock_id: ContractIdV1,
            dock_version: VersionV1,
            dock_ref: ExactRefV1,
            protocol: ExactBindingV1,
            request_schema: ExactBindingV1,
            result_schema: ExactBindingV1,
            implementation_kind: DockImplementationKindV1,
            implementation_bundle: DockImplementationBundleV1,
            supported_evidence_kinds: Vec<StableIdV1>,
            supported_inputs: Vec<DockInputRoleV1>,
            resolution_ceiling: ResolutionEnvelopeV1,
            limits: DockProtocolLimitsV1,
            deterministic: bool,
            offline: bool,
            read_only: bool,
            manifest_fingerprint: FingerprintV1,
        }
        let raw = Raw::deserialize(deserializer)?;
        if !raw.deterministic || !raw.offline || !raw.read_only {
            return Err(D::Error::custom(DockErrorV1::InvalidInvariant));
        }
        let supplied_ref = raw.dock_ref;
        let supplied_fingerprint = raw.manifest_fingerprint;
        let value = Self::new(DockManifestInputV1 {
            manifest_schema: raw.manifest_schema,
            dock_id: raw.dock_id,
            dock_version: raw.dock_version,
            protocol: raw.protocol,
            request_schema: raw.request_schema,
            result_schema: raw.result_schema,
            implementation_kind: raw.implementation_kind,
            implementation_bundle: raw.implementation_bundle,
            supported_evidence_kinds: raw.supported_evidence_kinds,
            supported_inputs: raw.supported_inputs,
            resolution_ceiling: raw.resolution_ceiling,
            limits: raw.limits,
        })
        .map_err(D::Error::custom)?;
        if value.dock_ref != supplied_ref || value.manifest_fingerprint != supplied_fingerprint {
            return Err(D::Error::custom(DockErrorV1::DerivedValueMismatch));
        }
        Ok(value)
    }
}

#[derive(Serialize)]
struct ManifestPreimage<'a> {
    manifest_schema: &'a ExactBindingV1,
    dock_id: &'a ContractIdV1,
    dock_version: &'a VersionV1,
    dock_ref: &'a ExactRefV1,
    protocol: &'a ExactBindingV1,
    request_schema: &'a ExactBindingV1,
    result_schema: &'a ExactBindingV1,
    implementation_kind: DockImplementationKindV1,
    implementation_bundle: &'a DockImplementationBundleV1,
    supported_evidence_kinds: &'a [StableIdV1],
    supported_inputs: &'a [DockInputRoleV1],
    resolution_ceiling: &'a ResolutionEnvelopeV1,
    limits: &'a DockProtocolLimitsV1,
    deterministic: bool,
    offline: bool,
    read_only: bool,
}

pub(crate) fn require_ref(binding: &ExactBindingV1, expected: &str) -> Result<(), DockErrorV1> {
    if binding.reference().as_str() != expected {
        return Err(DockErrorV1::InvalidIdentity);
    }
    Ok(())
}

pub(crate) fn validate_strict_ids(values: &[StableIdV1], max: usize) -> Result<(), DockErrorV1> {
    if values.is_empty() || values.len() > max {
        return Err(DockErrorV1::BoundExceeded);
    }
    if values.windows(2).any(|pair| pair[0] >= pair[1]) {
        return Err(DockErrorV1::InvalidInvariant);
    }
    Ok(())
}

pub(crate) fn valid_media_type(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 255
        && value.is_ascii()
        && value.contains('/')
        && value
            .bytes()
            .all(|b| !b.is_ascii_control() && !b.is_ascii_whitespace())
}

fn valid_path_segment(segment: &str) -> bool {
    if segment.is_empty()
        || segment == "."
        || segment == ".."
        || segment.trim() != segment
        || segment.ends_with('.')
        || segment.chars().any(|character| {
            character.is_control() || matches!(character, '<' | '>' | '"' | '|' | '?' | '*')
        })
    {
        return false;
    }
    let stem = segment.split('.').next().unwrap_or_default();
    let upper = stem.to_ascii_uppercase();
    !matches!(upper.as_str(), "CON" | "PRN" | "AUX" | "NUL")
        && !(upper.len() == 4
            && (upper.starts_with("COM") || upper.starts_with("LPT"))
            && matches!(upper.as_bytes()[3], b'1'..=b'9'))
}
