//! Native operational facts and a durable, non-retryable local evaluation attempt.
//! Contracts owns readiness, collection closure and evaluation truth.
use crate::canonical::{
    admit_canonical_json, canonical_bytes, canonical_fingerprint, fingerprint_hex,
};
use crate::*;
use handbook_contracts::{
    ContentAddressedBindingV1, ExactBindingV1, FingerprintV1, LocalDefinitionDependenciesV2,
    NativeLocalEvaluationCapabilityV3, SemanticTimeV1,
};
use handbook_engine::{RepositoryReadV1, RetainedRepositoryRootV1};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::path::{Path, PathBuf};

pub const DOCK_LOCAL_EVALUATION_ROOT_V1: &str = ".handbook/state/dock/local-evaluation-v1";
/// Native owner-relative package source; this is not a repository or host path.
pub fn dock_native_package_read_selector_v1() -> String {
    format!(
        "handbook-docks/handbook.dock.json-schema/1.0.0/{}/**",
        host_target_tuple_v1()
    )
}
const LOCK: &str = ".handbook/state/locks/dock-operational-store.lock";
const MAX_BYTES: usize = 8 * 1024 * 1024;
// The retained native filesystem reader accepts bounds up to 16MiB.
const MAX_NATIVE_ARTIFACT_BYTES: usize = 16 * 1024 * 1024;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DockLocalEvaluationReachV1 {
    /// Exact files or explicit complete child subtrees ending in `/**`.
    pub read_paths: Vec<String>,
    /// Complete child subtrees ending in `/**`, or the exact existing owner lock.
    pub write_subtrees: Vec<String>,
}
impl DockLocalEvaluationReachV1 {
    pub fn validate(&self) -> Result<(), DockLocalEvaluationErrorV1> {
        for paths in [&self.read_paths, &self.write_subtrees] {
            if paths.len() > 4096
                || paths.windows(2).any(|p| p[0] >= p[1])
                || paths.iter().any(|p| !valid_path(p))
            {
                return Err(error(
                    DockLocalEvaluationErrorKindV1::ReachRefused,
                    "invalid declared native reach",
                ));
            }
        }
        Ok(())
    }
    fn read(&self, path: &str) -> Result<(), DockLocalEvaluationErrorV1> {
        self.validate()?;
        if self.read_paths.iter().any(|p| covers(p, path)) {
            Ok(())
        } else {
            Err(error(
                DockLocalEvaluationErrorKindV1::ReachRefused,
                "native read path was not declared",
            ))
        }
    }
    fn mutation(&self) -> Result<(), DockLocalEvaluationErrorV1> {
        self.validate()?;
        for required in [
            format!("{DOCK_LOCAL_EVALUATION_ROOT_V1}/**"),
            format!("{DOCK_OPERATIONAL_STORE_RELATIVE_ROOT_V1}/**"),
            LOCK.into(),
        ] {
            if !self
                .write_subtrees
                .iter()
                .any(|p| p == &required || covers(p, &required))
            {
                return Err(error(
                    DockLocalEvaluationErrorKindV1::ReachRefused,
                    "complete Dock owner mutation subtree was not declared",
                ));
            }
        }
        self.read(LOCK)?;
        self.read(DOCK_LOCAL_EVALUATION_ROOT_V1)?;
        self.read(DOCK_OPERATIONAL_STORE_RELATIVE_ROOT_V1)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DockLocalEvaluationErrorKindV1 {
    InvalidRequest,
    ReachRefused,
    AuthorityRefused,
    Unavailable,
    Substituted,
    Conflict,
    Uncertain,
    RetentionRefused,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DockLocalEvaluationErrorV1 {
    kind: DockLocalEvaluationErrorKindV1,
    detail: &'static str,
}
impl DockLocalEvaluationErrorV1 {
    pub fn kind(&self) -> DockLocalEvaluationErrorKindV1 {
        self.kind
    }
    pub fn detail(&self) -> &'static str {
        self.detail
    }
}
impl std::fmt::Display for DockLocalEvaluationErrorV1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.detail)
    }
}
impl std::error::Error for DockLocalEvaluationErrorV1 {}
fn error(kind: DockLocalEvaluationErrorKindV1, detail: &'static str) -> DockLocalEvaluationErrorV1 {
    DockLocalEvaluationErrorV1 { kind, detail }
}
fn unavailable() -> DockLocalEvaluationErrorV1 {
    error(
        DockLocalEvaluationErrorKindV1::Unavailable,
        "exact native Dock dependency is unavailable",
    )
}
fn substituted() -> DockLocalEvaluationErrorV1 {
    error(
        DockLocalEvaluationErrorKindV1::Substituted,
        "exact native Dock dependency was substituted",
    )
}

/// Untrusted coordinates are checked against Engine's sealed launch admission.
#[derive(Clone, Debug)]
pub struct DockLocalEvaluationCoordinatesV1 {
    pub occurrence: ContentAddressedBindingV1,
    pub basis: ContentAddressedBindingV1,
    pub selection: ExactBindingV1,
    pub authorization: ContentAddressedBindingV1,
    pub capability_observation: ContentAddressedBindingV1,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DockLocalEvaluationAttemptIntentV1 {
    pub schema_ref: String,
    pub attempt_ref: String,
    pub attempt_fingerprint: FingerprintV1,
    pub occurrence: ContentAddressedBindingV1,
    pub basis: ContentAddressedBindingV1,
    pub selection: ExactBindingV1,
    pub authorization: ContentAddressedBindingV1,
    pub capability_observation: ContentAddressedBindingV1,
    pub ordinal: u8,
    pub request: ContentAddressedBindingV1,
    pub evaluation_run: ContentAddressedBindingV1,
    pub request_jcs: String,
    pub evaluation_run_jcs: String,
    pub read_paths: Vec<String>,
    pub write_subtrees: Vec<String>,
    pub retention_until: SemanticTimeV1,
    pub lock_generation: FingerprintV1,
}
impl DockLocalEvaluationAttemptIntentV1 {
    pub fn binding(&self) -> Result<ContentAddressedBindingV1, DockLocalEvaluationErrorV1> {
        ContentAddressedBindingV1::new(&self.attempt_ref, self.attempt_fingerprint.clone())
            .map_err(|_| substituted())
    }
    pub fn relative_path(&self) -> String {
        format!(
            "{}/intent.json",
            attempt_directory(&self.attempt_fingerprint)
        )
    }
    pub fn terminal_path(&self) -> String {
        format!(
            "{}/terminal.json",
            attempt_directory(&self.attempt_fingerprint)
        )
    }
    pub fn canonical_json_bytes(&self) -> Result<Vec<u8>, DockLocalEvaluationErrorV1> {
        self.validate()?;
        canonical_bytes(self).map_err(|_| substituted())
    }
    fn validate(&self) -> Result<(), DockLocalEvaluationErrorV1> {
        if self.schema_ref != "handbook.dock.local-evaluation-attempt-intent@1.0.0"
            || self.ordinal != 1
            || record_fingerprint(self, &["attempt_ref", "attempt_fingerprint"])?
                != self.attempt_fingerprint
            || self.attempt_ref
                != format!(
                    "handbook.dock.local-evaluation-attempt@1.0.0#{}",
                    fingerprint_hex(&self.attempt_fingerprint)
                )
        {
            return Err(substituted());
        }
        let reach = self.reach();
        reach.mutation()?;
        let request: DockProcessRequestV1 =
            admit_canonical_json(self.request_jcs.as_bytes(), MAX_BYTES)
                .map_err(|_| substituted())?;
        if request.binding().map_err(|_| substituted())? != self.request
            || request.manifest() != &self.selection
            || request.run().evaluation_run() != &self.evaluation_run
        {
            return Err(substituted());
        }
        validate_run(self.evaluation_run_jcs.as_bytes(), &self.evaluation_run)?;
        Ok(())
    }
    fn reach(&self) -> DockLocalEvaluationReachV1 {
        DockLocalEvaluationReachV1 {
            read_paths: self.read_paths.clone(),
            write_subtrees: self.write_subtrees.clone(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DockLocalEvaluationAttemptTerminalV1 {
    pub schema_ref: String,
    pub attempt: ContentAddressedBindingV1,
    pub kind: String,
    pub execution: Option<ContentAddressedBindingV1>,
    pub store_root: Option<FingerprintV1>,
    pub candidates: Vec<ContentAddressedBindingV1>,
    pub reason: String,
    pub retention_until: SemanticTimeV1,
    pub terminal_fingerprint: FingerprintV1,
}
impl DockLocalEvaluationAttemptTerminalV1 {
    pub fn binding(&self) -> Result<ContentAddressedBindingV1, DockLocalEvaluationErrorV1> {
        self.validate()?;
        ContentAddressedBindingV1::new(
            &format!(
                "handbook.dock.local-evaluation-attempt-terminal@1.0.0#{}",
                fingerprint_hex(&self.terminal_fingerprint)
            ),
            self.terminal_fingerprint.clone(),
        )
        .map_err(|_| substituted())
    }
    fn validate(&self) -> Result<(), DockLocalEvaluationErrorV1> {
        if self.schema_ref != "handbook.dock.local-evaluation-attempt-terminal@1.0.0"
            || !matches!(
                self.kind.as_str(),
                "completed"
                    | "refused"
                    | "timed_out"
                    | "cancelled"
                    | "failed"
                    | "protocol_error"
                    | "cleanup_uncertain"
                    | "cancelled_before_launch"
                    | "refused_before_launch"
            )
            || self.terminal_fingerprint != record_fingerprint(self, &["terminal_fingerprint"])?
            || self.candidates.len() > 1
            || (self.kind != "completed" && !self.candidates.is_empty())
            || self.execution.is_some() != self.store_root.is_some()
            || (matches!(
                self.kind.as_str(),
                "cancelled_before_launch" | "refused_before_launch"
            ) && self.execution.is_some())
        {
            return Err(substituted());
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case", deny_unknown_fields)]
pub enum DockLocalEvaluationAttemptOutcomeV1 {
    Absent,
    EffectUncertain {
        intent: DockLocalEvaluationAttemptIntentV1,
    },
    Terminal {
        intent: DockLocalEvaluationAttemptIntentV1,
        terminal: DockLocalEvaluationAttemptTerminalV1,
        replayed: bool,
    },
}

pub struct DockLocalEvaluationOwnerV1 {
    root: RetainedRepositoryRootV1,
    repo_root: PathBuf,
}
impl DockLocalEvaluationOwnerV1 {
    /// Explicit first-use initialization under live preauthorization terms.
    /// Existing owner state with a lost lock is never repaired by this path.
    pub fn initialize(
        root: impl AsRef<Path>,
        admission:&handbook_engine::local_authority::local_evaluation::LocalEvaluationTermsAdmissionV1<'_>,
        reach: &DockLocalEvaluationReachV1,
    ) -> Result<Self, DockLocalEvaluationErrorV1> {
        reach.mutation()?;
        reach.read(".handbook/state/dock")?;
        reach.read(".handbook/state")?;
        let retained =
            RetainedRepositoryRootV1::open_exact(root.as_ref()).map_err(|_| unavailable())?;
        admission.require_repository_root(&retained).map_err(|_| {
            error(
                DockLocalEvaluationErrorKindV1::AuthorityRefused,
                "Dock initialization root is not admitted",
            )
        })?;
        for subtree in &reach.write_subtrees {
            admission
                .require_owner_write_subtree(subtree.trim_end_matches("/**"))
                .map_err(|_| {
                    error(
                        DockLocalEvaluationErrorKindV1::ReachRefused,
                        "Dock initialization mutation subtree is not admitted",
                    )
                })?;
        }
        for path in &reach.read_paths {
            admission.require_input_selector(path).map_err(|_| {
                error(
                    DockLocalEvaluationErrorKindV1::ReachRefused,
                    "Dock initialization read reach is not admitted",
                )
            })?;
        }
        if retained
            .existing_lock_physical_fingerprint_v1(LOCK)
            .is_err()
        {
            let state = retained
                .list_directory_bounded(".handbook/state", 4096)
                .map_err(|_| unavailable())?;
            if state.iter().any(|entry| entry.name() == "dock") {
                return Err(unavailable());
            }
        }
        admission.validate_current().map_err(|_| {
            error(
                DockLocalEvaluationErrorKindV1::AuthorityRefused,
                "Dock initialization terms changed",
            )
        })?;
        DockOperationalStoreV1::open(root.as_ref()).map_err(|_| unavailable())?;
        admission.validate_current().map_err(|_| {
            error(
                DockLocalEvaluationErrorKindV1::AuthorityRefused,
                "Dock initialization terms changed before delivery",
            )
        })?;
        Self::open_existing(root)
    }
    /// This inspection cannot initialize or repair missing locks or owner state.
    pub fn open_existing(root: impl AsRef<Path>) -> Result<Self, DockLocalEvaluationErrorV1> {
        let repo_root = root.as_ref().to_path_buf();
        let root = RetainedRepositoryRootV1::open_exact(&repo_root).map_err(|_| unavailable())?;
        root.existing_lock_physical_fingerprint_v1(LOCK)
            .map_err(|_| unavailable())?;
        Ok(Self { root, repo_root })
    }
    pub fn retained_root(&self) -> &RetainedRepositoryRootV1 {
        &self.root
    }

    /// Recovery identity only; this neither reserves an attempt nor grants launch.
    #[allow(clippy::too_many_arguments)]
    pub fn prepare_attempt_locator(
        &self,
        coordinates: &DockLocalEvaluationCoordinatesV1,
        request: &DockProcessRequestV1,
        run_jcs: &[u8],
        reach: &DockLocalEvaluationReachV1,
        retention_until: SemanticTimeV1,
    ) -> Result<ContentAddressedBindingV1, DockLocalEvaluationErrorV1> {
        self.make_intent(coordinates, request, run_jcs, reach, retention_until)?
            .binding()
    }

    /// Observe the embedded semantic profile and actual installed native package.
    /// Unknown records preserve an inability to observe; absence is known limited.
    pub fn observe_capability(
        &self,
        admission:&handbook_engine::local_authority::local_evaluation::LocalEvaluationTermsAdmissionV1<'_>,
        reach: &DockLocalEvaluationReachV1,
        retention_until: SemanticTimeV1,
    ) -> Result<NativeLocalEvaluationCapabilityV3, DockLocalEvaluationErrorV1> {
        reach.mutation()?;
        reach.read(dock_native_package_read_selector_v1().trim_end_matches("/**"))?;
        admission.require_repository_root(&self.root).map_err(|_| {
            error(
                DockLocalEvaluationErrorKindV1::AuthorityRefused,
                "Dock capability observation root was not admitted",
            )
        })?;
        for subtree in &reach.write_subtrees {
            admission
                .require_owner_write_subtree(subtree.trim_end_matches("/**"))
                .map_err(|_| {
                    error(
                        DockLocalEvaluationErrorKindV1::ReachRefused,
                        "Dock capability mutation subtree was not admitted",
                    )
                })?;
        }
        for path in &reach.read_paths {
            admission.require_input_selector(path).map_err(|_| {
                error(
                    DockLocalEvaluationErrorKindV1::ReachRefused,
                    "Dock capability read reach was not admitted",
                )
            })?;
        }
        let now = SemanticTimeV1::owner_observation_now_v1().map_err(|_| unavailable())?;
        if retention_until < now {
            return Err(error(
                DockLocalEvaluationErrorKindV1::RetentionRefused,
                "supplied Dock retention horizon is already past",
            ));
        }
        let manifest = crate::registry::production_manifest_v1();
        let resolved = DockResolvedFirstPartyBundleV1::resolve_production();
        let (availability, reason, files) = match resolved {
            Ok(bundle) => (
                "available",
                "exact_production_package_verified",
                bundle
                    .capture_verified_files_v1()
                    .map_err(|_| substituted())?,
            ),
            Err(failure) if failure.kind() == DockProductionBundleErrorKindV1::LaunchRefused => {
                return Err(error(
                    DockLocalEvaluationErrorKindV1::Substituted,
                    "native production package or target was refused",
                ));
            }
            Err(_) => {
                let observed_absent = std::env::current_exe()
                    .ok()
                    .and_then(|executable| {
                        executable.parent().map(|parent| {
                            parent
                                .join("handbook-docks")
                                .join("handbook.dock.json-schema")
                                .join("1.0.0")
                                .join(host_target_tuple_v1())
                        })
                    })
                    .is_some_and(|path| {
                        std::fs::symlink_metadata(&path)
                            .is_err_and(|error| error.kind() == std::io::ErrorKind::NotFound)
                            || std::fs::symlink_metadata(path.join("manifest.json"))
                                .is_err_and(|error| error.kind() == std::io::ErrorKind::NotFound)
                    });
                if observed_absent {
                    ("known_limited", "production_package_absent", Vec::new())
                } else {
                    (
                        "unknown",
                        "production_package_availability_unobservable",
                        Vec::new(),
                    )
                }
            }
        };
        let lock = self
            .root
            .acquire_existing_exclusive_lock_v1(LOCK)
            .map_err(|_| unavailable())?;
        let lock_generation = lock.physical_fingerprint_v1().map_err(|_| unavailable())?;
        let package_root = format!(
            "{DOCK_LOCAL_EVALUATION_ROOT_V1}/packages/{}",
            fingerprint_hex(manifest.implementation_bundle().bundle_fingerprint())
        );
        self.root
            .create_directories(&format!("{DOCK_LOCAL_EVALUATION_ROOT_V1}/capabilities"))
            .map_err(|_| unavailable())?;
        self.root
            .create_directories(&format!("{DOCK_LOCAL_EVALUATION_ROOT_V1}/attempts"))
            .map_err(|_| unavailable())?;
        self.root
            .create_directories(&format!("{DOCK_LOCAL_EVALUATION_ROOT_V1}/packages"))
            .map_err(|_| unavailable())?;
        let mut package_files = Vec::new();
        if availability == "available" {
            self.root
                .create_directories(&package_root)
                .map_err(|_| unavailable())?;
            let manifest_path = format!("{package_root}/manifest.json");
            reach.read(&manifest_path)?;
            self.write_exact(
                &manifest_path,
                &manifest.canonical_json_bytes().map_err(|_| substituted())?,
            )?;
            for (relative, bytes) in files {
                let path = format!("{package_root}/{relative}");
                reach.read(&path)?;
                if let Some(parent) = path.rsplit_once('/').map(|(parent, _)| parent) {
                    self.root
                        .create_directories(parent)
                        .map_err(|_| unavailable())?;
                }
                self.write_exact(&path, &bytes)?;
                package_files.push(
                    json!({"path":path,"length":bytes.len(),"sha256":fingerprint_bytes(&bytes)}),
                );
            }
        }
        let controls = json!({"profile":"first_party_native_advisory", "environment":"cleared", "cwd":"private",
            "time":"monotonic_bounded", "output":"bounded", "cancellation":"host_signal_and_bounded_termination",
            "network_enforcement":"not_provided", "availability_reason":reason, "target":host_target_tuple_v1(),
            "manifest_path":if availability=="available" { Some(format!("{package_root}/manifest.json")) } else { None },
            "package_files":package_files,"lock_generation":lock_generation,
            "operating_ceiling":manifest.resolution_ceiling(),"effective_limits":manifest.limits()});
        let record = NativeLocalEvaluationCapabilityV3::new(
            manifest.binding(),
            ContentAddressedBindingV1::new(
                manifest.implementation_bundle().bundle_ref().as_str(),
                manifest
                    .implementation_bundle()
                    .bundle_fingerprint()
                    .clone(),
            )
            .map_err(|_| substituted())?,
            host_target_tuple_v1().into(),
            availability.into(),
            controls,
            now,
            retention_until,
        )
        .map_err(|_| substituted())?;
        reach.read(&record.relative_path())?;
        admission.validate_current().map_err(|_| {
            error(
                DockLocalEvaluationErrorKindV1::AuthorityRefused,
                "Dock capability authority changed before publication",
            )
        })?;
        self.write_exact(
            &record.relative_path(),
            &record.canonical_json_bytes().map_err(|_| substituted())?,
        )?;
        lock.release().map_err(|_| unavailable())?;
        admission.validate_current().map_err(|_| {
            error(
                DockLocalEvaluationErrorKindV1::AuthorityRefused,
                "Dock capability authority changed before delivery",
            )
        })?;
        Ok(record)
    }

    pub fn read_capability(
        &self,
        binding: &ContentAddressedBindingV1,
        reach: &DockLocalEvaluationReachV1,
    ) -> Result<NativeLocalEvaluationCapabilityV3, DockLocalEvaluationErrorV1> {
        let path = format!(
            "{DOCK_LOCAL_EVALUATION_ROOT_V1}/capabilities/{}.json",
            fingerprint_hex(binding.fingerprint())
        );
        reach.read(LOCK)?;
        let lock = self
            .root
            .acquire_existing_exclusive_lock_v1(LOCK)
            .map_err(|_| unavailable())?;
        let record: NativeLocalEvaluationCapabilityV3 = admit_canonical_json(
            &self.read_optional(&path, reach)?.ok_or_else(unavailable)?,
            MAX_BYTES,
        )
        .map_err(|_| substituted())?;
        if record.binding().map_err(|_| substituted())? != *binding {
            return Err(substituted());
        }
        lock.release().map_err(|_| unavailable())?;
        Ok(record)
    }

    pub fn with_capability<R>(
        &self,
        record: &NativeLocalEvaluationCapabilityV3,
        dependencies: &LocalDefinitionDependenciesV2,
        reach: &DockLocalEvaluationReachV1,
        consume: impl for<'a> FnOnce(&handbook_contracts::ResolvedLocalEvaluationCapabilityV3<'a>) -> R,
    ) -> Result<R, DockLocalEvaluationErrorV1> {
        let mut paths = vec![LOCK.to_owned(), record.relative_path()];
        if let Some(path) = record.controls["manifest_path"].as_str() {
            paths.push(path.into());
        }
        for file in record.controls["package_files"]
            .as_array()
            .ok_or_else(substituted)?
        {
            paths.push(file["path"].as_str().ok_or_else(substituted)?.into());
        }
        paths.sort();
        paths.dedup();
        for path in &paths {
            reach.read(path)?;
        }
        let checkpoint = self
            .root
            .retain_exact_files_checkpoint_v1(&paths.iter().map(String::as_str).collect::<Vec<_>>())
            .map_err(|_| unavailable())?;
        if dependencies.validator.manifest != crate::registry::production_manifest_v1().binding() {
            return Err(substituted());
        }
        handbook_contracts::resolve_native_local_evaluation_capability_v3(
            &self.root,
            &checkpoint,
            record.clone(),
            dependencies,
            consume,
        )
        .map_err(|_| substituted())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn execute_once(
        &self,
        admission: &handbook_engine::local_authority::local_evaluation::LocalEvaluationAdmissionV1<
            '_,
        >,
        coordinates: &DockLocalEvaluationCoordinatesV1,
        request: &DockProcessRequestV1,
        evaluation_run_jcs: &[u8],
        reach: &DockLocalEvaluationReachV1,
        retention_until: SemanticTimeV1,
        cancellation: &DockCancellationV1,
    ) -> Result<DockLocalEvaluationAttemptOutcomeV1, DockLocalEvaluationErrorV1> {
        use handbook_engine::local_authority::local_evaluation::LocalEvaluationCheckpointV1;
        reach.mutation()?;
        admission.require_repository_root(&self.root).map_err(|_| {
            error(
                DockLocalEvaluationErrorKindV1::AuthorityRefused,
                "Dock launch root is outside Engine admission",
            )
        })?;
        admission.validate_current().map_err(|_| {
            error(
                DockLocalEvaluationErrorKindV1::AuthorityRefused,
                "Engine launch admission is not current",
            )
        })?;
        for subtree in &reach.write_subtrees {
            admission
                .require_owner_write_subtree(subtree.trim_end_matches("/**"))
                .map_err(|_| {
                    error(
                        DockLocalEvaluationErrorKindV1::ReachRefused,
                        "Dock mutation reach was not admitted by Engine",
                    )
                })?;
        }
        for path in &reach.read_paths {
            admission.require_input_selector(path).map_err(|_| {
                error(
                    DockLocalEvaluationErrorKindV1::ReachRefused,
                    "Dock read reach was not admitted by Engine",
                )
            })?;
        }
        let authorization = admission.binding();
        let authority_request = admission.request();
        if admission.checkpoint() != LocalEvaluationCheckpointV1::Launch
            || admission.checkpoint_target() != Some(coordinates.basis.fingerprint().as_str())
            || authorization.issuance_fingerprint
                != coordinates.authorization.fingerprint().as_str()
            || authorization.occurrence_identity != coordinates.occurrence.fingerprint().as_str()
            || request.manifest() != &coordinates.selection
            || coordinates.selection != crate::registry::production_manifest_v1().binding()
            || request.run().subject().subject().reference()
                != authority_request.purpose.subject_ref
            || request.run().subject().subject().fingerprint().as_str()
                != authority_request.purpose.subject_fingerprint
            || request.run().contract().fingerprint().as_str()
                != authority_request.purpose.contract.fingerprint
            || retention_until.as_str() > authorization.retention_until_utc.as_str()
            || retention_until.as_str() != authority_request.purpose.retention_until_utc
        {
            return Err(error(DockLocalEvaluationErrorKindV1::AuthorityRefused,"exact Dock occurrence/basis/selection/request/retention does not match purpose-bound Engine authorization"));
        }
        validate_run(evaluation_run_jcs, request.run().evaluation_run())?;
        let full_run: handbook_contracts::ContractEvaluationRunV1 =
            admit_canonical_json(evaluation_run_jcs, MAX_BYTES).map_err(|_| substituted())?;
        if !full_run.is_local_evaluation_v3() {
            return Err(substituted());
        }
        if request.run().evaluation_lifecycle_basis() != full_run.evaluation_lifecycle_basis()
            || request.run().contract() != &full_run.contract()
            || request.run().claim_ids()
                != full_run
                    .claims()
                    .iter()
                    .map(|claim| claim.claim_id().clone())
                    .collect::<Vec<_>>()
            || request.run().subject() != full_run.subject()
            || request.run().cases() != full_run.cases()
            || request.run().requested_resolution() != full_run.requested_resolution()
            || request.run().semantic_time_utc() != full_run.owner_currentness().semantic_time_utc()
            || request.run().lifecycle_head_fingerprint()
                != full_run.owner_currentness().lifecycle_head_fingerprint()
        {
            return Err(substituted());
        }
        let requested = request.run().requested_resolution();
        let dimensions = serde_json::to_value(requested).map_err(|_| substituted())?;
        admission
            .require_requested_resolution(
                &handbook_engine::local_authority::LocalBindingV1 {
                    reference: requested.resolution_stack().reference().as_str().into(),
                    fingerprint: requested.resolution_stack().fingerprint().as_str().into(),
                },
                [
                    "scope_horizon",
                    "detail_resolution",
                    "temporal_horizon",
                    "authority_horizon",
                    "memory_horizon",
                    "validation_horizon",
                ]
                .map(|key| dimensions[key].as_str().unwrap_or("")),
            )
            .map_err(|_| {
                error(
                    DockLocalEvaluationErrorKindV1::AuthorityRefused,
                    "actual process Resolution differs from the exact Engine selection reach",
                )
            })?;
        let input = request.inputs().first().ok_or_else(substituted)?;
        if request.inputs().len() != 1
            || request.run().cases().len() != 1
            || input
                .captured_evaluation_subject_v1()
                .map_err(|_| substituted())?
                != *request.run().subject()
            || input
                .captured_evaluation_case_v1(request.run().cases()[0].case_id().clone())
                .map_err(|_| substituted())?
                != request.run().cases()[0]
        {
            return Err(substituted());
        }
        let manifest = crate::registry::production_manifest_v1();
        let validator = json!({"manifest":manifest.binding(),"canonical_manifest":serde_json::to_value(&manifest).map_err(|_| substituted())?});
        let schema_closure = json!({"root_schema":input.content().get("root_schema"),"root_document":input.content().get("root_document"),"documents":input.content().get("documents"),"closure_fingerprint":input.content().get("closure_fingerprint")});
        let selection = &authority_request.plan.selections[0];
        if canonical_fingerprint(&(validator, schema_closure))
            .map_err(|_| substituted())?
            .as_str()
            != selection.configuration_fingerprint
            || handbook_contracts::native_local_evaluation_capability_binding_v3()
                .map_err(|_| substituted())?
                .fingerprint()
                .as_str()
                != selection.capability_fingerprint
            || canonical_fingerprint(manifest.resolution_ceiling())
                .map_err(|_| substituted())?
                .as_str()
                != selection.operating_ceiling_fingerprint
            || canonical_fingerprint(&json!([]))
                .map_err(|_| substituted())?
                .as_str()
                != selection.host_prerequisites_fingerprint
        {
            return Err(substituted());
        }
        let capability = self.read_capability(&coordinates.capability_observation, reach)?;
        if capability.retention_until < retention_until {
            return Err(error(
                DockLocalEvaluationErrorKindV1::RetentionRefused,
                "original native capability does not cover the attempt retention horizon",
            ));
        }
        if capability.availability != "available"
            && authority_request.purpose.use_intent != handbook_engine::local_authority::local_evaluation::LocalEvaluationUseIntentV1::Diagnostic {
            return Err(error(DockLocalEvaluationErrorKindV1::AuthorityRefused, "decision-seeking cannot attempt an unavailable original capability observation"));
        }
        let capability_paths = self.capability_paths(&capability, reach)?;
        let source_checkpoint = self
            .root
            .retain_exact_files_checkpoint_v1(
                &capability_paths
                    .iter()
                    .map(String::as_str)
                    .collect::<Vec<_>>(),
            )
            .map_err(|_| unavailable())?;
        self.validate_launch_capability(&capability, &source_checkpoint, reach)?;
        let intent = self.make_intent(
            coordinates,
            request,
            evaluation_run_jcs,
            reach,
            retention_until,
        )?;
        let (intent, fresh) = self.reserve_intent(intent)?;
        if !fresh {
            return self.resolve_exact_attempt(&intent.binding()?, reach, true);
        }
        // The reservation lock has been released: the ordinary executor acquires
        // the same native lock. A competing caller can observe intent but cannot
        // launch. An interrupted reservation is conservatively effect-uncertain.
        if cancellation.is_requested() {
            return self.commit_prelaunch_terminal(
                intent,
                "cancelled_before_launch",
                "native cancellation observed before launch",
                false,
            );
        }
        source_checkpoint.validate().map_err(|_| substituted())?;
        if admission.validate_current().is_err() {
            return self.commit_prelaunch_terminal(
                intent,
                "refused_before_launch",
                "Engine launch admission changed after reservation",
                false,
            );
        }
        // The originally committed observation remains fixed. A diagnostic
        // unavailable selection retains an exact no-effect terminal without
        // resolving, refreshing or launching the installed producer.
        if capability.availability != "available" {
            return self.commit_prelaunch_terminal(
                intent,
                "refused_before_launch",
                "original native capability observation cannot establish an available producer",
                false,
            );
        }
        reach.read(dock_native_package_read_selector_v1().trim_end_matches("/**"))?;
        let bundle = match DockResolvedFirstPartyBundleV1::resolve_production() {
            Ok(bundle) => bundle,
            Err(failure) => {
                return self.commit_prelaunch_terminal(
                    intent,
                    "refused_before_launch",
                    match failure.kind() {
                        DockProductionBundleErrorKindV1::Unavailable => {
                            "production package unavailable before launch"
                        }
                        DockProductionBundleErrorKindV1::LaunchRefused => {
                            "production package refused before launch"
                        }
                    },
                    false,
                )
            }
        };
        let store = DockOperationalStoreV1::open_existing_for_recovery_v3(self.root_path()?)
            .map_err(|_| unavailable())?;
        let outcome =
            DockProcessExecutorV1::new().execute_local_evaluation_v3(DockExecutionInputV1 {
                store: &store,
                resolved_bundle: &bundle,
                request,
                evaluation_run_jcs,
                idempotency_key: &attempt_key(&intent),
                cancellation,
            });
        source_checkpoint.validate().map_err(|_| substituted())?;
        match outcome {
            Ok(receipt) => self.commit_receipt_terminal(intent, &receipt, false),
            Err(failure) if failure.kind() == DockExecutorErrorKindV1::CancelledBeforeLaunch => {
                self.commit_prelaunch_terminal(
                    intent,
                    "cancelled_before_launch",
                    failure.detail(),
                    false,
                )
            }
            Err(failure) if failure.kind() == DockExecutorErrorKindV1::AdmissionRefused => self
                .commit_prelaunch_terminal(
                    intent,
                    "refused_before_launch",
                    failure.detail(),
                    false,
                ),
            Err(_) => Ok(DockLocalEvaluationAttemptOutcomeV1::EffectUncertain { intent }),
        }
    }

    /// Original locator recovery. No branch in this method launches a producer.
    /// `recover` permits only completion of already retained V1 store output.
    pub fn resolve_exact_attempt(
        &self,
        attempt: &ContentAddressedBindingV1,
        reach: &DockLocalEvaluationReachV1,
        recover: bool,
    ) -> Result<DockLocalEvaluationAttemptOutcomeV1, DockLocalEvaluationErrorV1> {
        reach.validate()?;
        if recover {
            reach.mutation()?;
        }
        let directory = attempt_directory(attempt.fingerprint());
        let path = format!("{directory}/intent.json");
        reach.read(&path)?;
        reach.read(LOCK)?;
        let lock = self
            .root
            .acquire_existing_exclusive_lock_v1(LOCK)
            .map_err(|_| unavailable())?;
        let attempts = format!("{DOCK_LOCAL_EVALUATION_ROOT_V1}/attempts");
        reach.read(&attempts)?;
        let entries = match self.root.list_directory_bounded(&attempts, 4096) {
            Ok(entries) => entries,
            Err(failure)
                if failure.kind() == handbook_engine::RepositoryDurabilityErrorKindV1::NotFound =>
            {
                lock.release().map_err(|_| unavailable())?;
                return Ok(DockLocalEvaluationAttemptOutcomeV1::Absent);
            }
            Err(_) => return Err(unavailable()),
        };
        if !entries
            .iter()
            .any(|entry| entry.name() == fingerprint_hex(attempt.fingerprint()))
        {
            lock.release().map_err(|_| unavailable())?;
            return Ok(DockLocalEvaluationAttemptOutcomeV1::Absent);
        }
        let bytes = self.read_optional(&path, reach)?.ok_or_else(|| {
            error(
                DockLocalEvaluationErrorKindV1::Uncertain,
                "native attempt allocation exists without complete retained intent",
            )
        })?;
        let intent: DockLocalEvaluationAttemptIntentV1 =
            admit_canonical_json(&bytes, MAX_BYTES).map_err(|_| substituted())?;
        intent.validate()?;
        if intent.binding()? != *attempt
            || intent.lock_generation.as_str()
                != lock.physical_fingerprint_v1().map_err(|_| unavailable())?
        {
            return Err(substituted());
        }
        let terminal = self
            .read_optional(&intent.terminal_path(), reach)?
            .map(|bytes| {
                admit_canonical_json::<DockLocalEvaluationAttemptTerminalV1>(&bytes, MAX_BYTES)
                    .map_err(|_| substituted())
            })
            .transpose()?;
        lock.release().map_err(|_| unavailable())?;
        if let Some(terminal) = terminal {
            self.require_terminal(&intent, &terminal)?;
            self.validate_terminal_dependencies(&intent, &terminal, reach, false)?;
            return Ok(DockLocalEvaluationAttemptOutcomeV1::Terminal {
                intent,
                terminal,
                replayed: true,
            });
        }
        reach.read(DOCK_OPERATIONAL_STORE_RELATIVE_ROOT_V1)?;
        let request: DockProcessRequestV1 =
            admit_canonical_json(intent.request_jcs.as_bytes(), MAX_BYTES)
                .map_err(|_| substituted())?;
        let store = DockOperationalStoreV1::open_existing_for_recovery_v3(self.root_path()?)
            .map_err(|_| unavailable())?;
        let receipt = store
            .replay_existing_for_request_v3(&request, &attempt_key(&intent), recover)
            .map_err(|_| unavailable())?;
        match receipt {
            Some(receipt) if recover => self.commit_receipt_terminal(intent, &receipt, true),
            _ => Ok(DockLocalEvaluationAttemptOutcomeV1::EffectUncertain { intent }),
        }
    }

    /// Current admission and immutable historical inspection use the same exact
    /// retained producer chain. This method makes no current authority decision.
    #[allow(clippy::too_many_arguments)]
    pub fn with_origin<R>(
        &self,
        attempt: &ContentAddressedBindingV1,
        coordinates: &DockLocalEvaluationCoordinatesV1,
        run: &handbook_contracts::ContractEvaluationRunV1,
        dependencies: &LocalDefinitionDependenciesV2,
        reach: &DockLocalEvaluationReachV1,
        consume: impl for<'a> FnOnce(&handbook_contracts::ResolvedLocalEvaluationOriginV3<'a>) -> R,
    ) -> Result<R, DockLocalEvaluationErrorV1> {
        let DockLocalEvaluationAttemptOutcomeV1::Terminal {
            intent, terminal, ..
        } = self.resolve_exact_attempt(attempt, reach, false)?
        else {
            return Err(error(
                DockLocalEvaluationErrorKindV1::Uncertain,
                "native origin has no exact closed terminal",
            ));
        };
        if intent.occurrence != coordinates.occurrence
            || intent.basis != coordinates.basis
            || intent.selection != coordinates.selection
            || intent.authorization != coordinates.authorization
            || intent.capability_observation != coordinates.capability_observation
            || intent.evaluation_run_jcs.as_bytes()
                != run.canonical_json_bytes().map_err(|_| substituted())?
        {
            return Err(substituted());
        }
        let mut paths = vec![LOCK.into(), intent.relative_path(), intent.terminal_path()];
        let capability = self.read_capability(&intent.capability_observation, reach)?;
        paths.extend(self.capability_paths(&capability, reach)?);
        if let Some(execution) = &terminal.execution {
            let key = store_key_fingerprint(&attempt_key(&intent));
            let replay_path = format!(
                "{DOCK_OPERATIONAL_STORE_RELATIVE_ROOT_V1}/replays/{}.json",
                fingerprint_hex(&key)
            );
            let replay: Value = admit_canonical_json(
                &self
                    .read_optional(&replay_path, reach)?
                    .ok_or_else(unavailable)?,
                MAX_BYTES,
            )
            .map_err(|_| substituted())?;
            paths.extend([
                replay_path,
                format!(
                    "{DOCK_OPERATIONAL_STORE_RELATIVE_ROOT_V1}/requests/{}.json",
                    fingerprint_hex(intent.request.fingerprint())
                ),
                format!(
                    "{DOCK_OPERATIONAL_STORE_RELATIVE_ROOT_V1}/executions/{}.json",
                    fingerprint_hex(execution.fingerprint())
                ),
                format!(
                    "{DOCK_OPERATIONAL_STORE_RELATIVE_ROOT_V1}/transactions/{}/intent.json",
                    fingerprint_hex(&key)
                ),
                format!(
                    "{DOCK_OPERATIONAL_STORE_RELATIVE_ROOT_V1}/transactions/{}/committed.json",
                    fingerprint_hex(&key)
                ),
            ]);
            if !replay["result"].is_null() {
                let result: ContentAddressedBindingV1 =
                    serde_json::from_value(replay["result"].clone()).map_err(|_| substituted())?;
                paths.push(format!(
                    "{DOCK_OPERATIONAL_STORE_RELATIVE_ROOT_V1}/results/{}.json",
                    fingerprint_hex(result.fingerprint())
                ));
            }
            let store = DockOperationalStoreV1::open_existing_for_recovery_v3(self.root_path()?)
                .map_err(|_| unavailable())?;
            for candidate in &terminal.candidates {
                let source = store
                    .read_local_candidate_v2(candidate)
                    .map_err(|_| unavailable())?;
                paths.extend(source.exact_paths);
            }
        }
        paths.sort();
        paths.dedup();
        for path in &paths {
            reach.read(path)?;
        }
        let checkpoint = self
            .root
            .retain_exact_files_checkpoint_v1(&paths.iter().map(String::as_str).collect::<Vec<_>>())
            .map_err(|_| unavailable())?;
        handbook_contracts::resolve_native_local_evaluation_origin_v3(
            &self.root,
            &checkpoint,
            attempt,
            &coordinates.occurrence,
            &coordinates.basis,
            &coordinates.selection,
            &coordinates.authorization,
            run,
            dependencies,
            consume,
        )
        .map_err(|_| substituted())
    }

    fn write_exact(&self, path: &str, bytes: &[u8]) -> Result<(), DockLocalEvaluationErrorV1> {
        match self
            .root
            .read_regular_bounded(path, native_read_limit(path)?)
            .map_err(|_| unavailable())?
        {
            RepositoryReadV1::Found(existing) if existing == bytes => Ok(()),
            RepositoryReadV1::Found(_) => Err(substituted()),
            RepositoryReadV1::Absent => self.root.write_new_durable(path, bytes).map_err(|_| {
                error(
                    DockLocalEvaluationErrorKindV1::Uncertain,
                    "native immutable write was uncertain",
                )
            }),
        }
    }
    fn capability_paths(
        &self,
        record: &NativeLocalEvaluationCapabilityV3,
        reach: &DockLocalEvaluationReachV1,
    ) -> Result<Vec<String>, DockLocalEvaluationErrorV1> {
        let mut paths = vec![LOCK.into(), record.relative_path()];
        if let Some(path) = record.controls["manifest_path"].as_str() {
            paths.push(path.into());
        }
        for file in record.controls["package_files"]
            .as_array()
            .ok_or_else(substituted)?
        {
            paths.push(file["path"].as_str().ok_or_else(substituted)?.into());
        }
        paths.sort();
        paths.dedup();
        for path in &paths {
            reach.read(path)?;
        }
        Ok(paths)
    }
    fn validate_launch_capability(
        &self,
        record: &NativeLocalEvaluationCapabilityV3,
        checkpoint: &handbook_engine::RepositoryExactFilesCheckpointV1<'_>,
        reach: &DockLocalEvaluationReachV1,
    ) -> Result<(), DockLocalEvaluationErrorV1> {
        record.validate().map_err(|_| substituted())?;
        let manifest = crate::registry::production_manifest_v1();
        if record.manifest != manifest.binding()
            || record.target != host_target_tuple_v1()
            || record.implementation_bundle
                != ContentAddressedBindingV1::new(
                    manifest.implementation_bundle().bundle_ref().as_str(),
                    manifest
                        .implementation_bundle()
                        .bundle_fingerprint()
                        .clone(),
                )
                .map_err(|_| substituted())?
            || record.semantic_capability
                != handbook_contracts::native_local_evaluation_capability_binding_v3()
                    .map_err(|_| substituted())?
            || record.controls["lock_generation"]
                != self
                    .root
                    .existing_lock_physical_fingerprint_v1(LOCK)
                    .map_err(|_| unavailable())?
            || record.controls["network_enforcement"] != "not_provided"
            || record.controls["profile"] != "first_party_native_advisory"
            || record.controls["environment"] != "cleared"
            || record.controls["cwd"] != "private"
            || record.controls["time"] != "monotonic_bounded"
            || record.controls["output"] != "bounded"
            || record.controls["cancellation"] != "host_signal_and_bounded_termination"
            || record.controls["target"] != host_target_tuple_v1()
            || record
                .controls
                .as_object()
                .is_none_or(|controls| controls.len() != 14)
            || record.controls["operating_ceiling"]
                != serde_json::to_value(manifest.resolution_ceiling()).map_err(|_| substituted())?
            || record.controls["effective_limits"]
                != serde_json::to_value(manifest.limits()).map_err(|_| substituted())?
        {
            return Err(substituted());
        }
        checkpoint
            .require_exact_file(
                &self.root,
                &record.relative_path(),
                &record.canonical_json_bytes().map_err(|_| substituted())?,
            )
            .map_err(|_| substituted())?;
        if record.availability != "available" {
            if !matches!(
                (
                    record.availability.as_str(),
                    record.controls["availability_reason"].as_str()
                ),
                ("known_limited", Some("production_package_absent"))
                    | (
                        "unknown",
                        Some("production_package_availability_unobservable")
                    )
            ) || !record.controls["manifest_path"].is_null()
                || record.controls["package_files"] != json!([])
            {
                return Err(substituted());
            }
            return checkpoint.validate().map_err(|_| substituted());
        }
        if record.controls["availability_reason"] != "exact_production_package_verified" {
            return Err(substituted());
        }
        let package_root = format!(
            "{DOCK_LOCAL_EVALUATION_ROOT_V1}/packages/{}",
            fingerprint_hex(manifest.implementation_bundle().bundle_fingerprint())
        );
        let manifest_path = format!("{package_root}/manifest.json");
        reach.read(&manifest_path)?;
        if record.controls["manifest_path"] != manifest_path {
            return Err(substituted());
        }
        checkpoint
            .require_exact_file(
                &self.root,
                &manifest_path,
                &manifest.canonical_json_bytes().map_err(|_| substituted())?,
            )
            .map_err(|_| substituted())?;
        let proofs = record.controls["package_files"]
            .as_array()
            .ok_or_else(substituted)?;
        if proofs.len() != manifest.implementation_bundle().files().len() {
            return Err(substituted());
        }
        for (proof, expected) in proofs.iter().zip(manifest.implementation_bundle().files()) {
            let path = format!("{package_root}/{}", expected.path().as_str());
            reach.read(&path)?;
            let bytes = self.read_optional(&path, reach)?.ok_or_else(unavailable)?;
            if proof != &json!({"path":path,"length":expected.length(),"sha256":expected.sha256()})
                || bytes.len() as u64 != expected.length()
                || fingerprint_bytes(&bytes) != *expected.sha256()
            {
                return Err(substituted());
            }
            checkpoint
                .require_exact_file(&self.root, &path, &bytes)
                .map_err(|_| substituted())?;
        }
        checkpoint.validate().map_err(|_| substituted())
    }
    fn read_optional(
        &self,
        path: &str,
        reach: &DockLocalEvaluationReachV1,
    ) -> Result<Option<Vec<u8>>, DockLocalEvaluationErrorV1> {
        reach.read(path)?;
        match self
            .root
            .read_regular_bounded(path, native_read_limit(path)?)
            .map_err(|_| unavailable())?
        {
            RepositoryReadV1::Found(bytes) => Ok(Some(bytes)),
            RepositoryReadV1::Absent => Ok(None),
        }
    }

    fn root_path(&self) -> Result<&Path, DockLocalEvaluationErrorV1> {
        // This validates the retained epoch before handing the same original
        // path to the existing owner, which independently retains its root.
        self.root
            .existing_lock_physical_fingerprint_v1(LOCK)
            .map_err(|_| unavailable())?;
        Ok(&self.repo_root)
    }
    fn make_intent(
        &self,
        coordinates: &DockLocalEvaluationCoordinatesV1,
        request: &DockProcessRequestV1,
        run: &[u8],
        reach: &DockLocalEvaluationReachV1,
        retention_until: SemanticTimeV1,
    ) -> Result<DockLocalEvaluationAttemptIntentV1, DockLocalEvaluationErrorV1> {
        reach.mutation()?;
        let now = SemanticTimeV1::owner_observation_now_v1().map_err(|_| unavailable())?;
        if retention_until < now {
            return Err(error(
                DockLocalEvaluationErrorKindV1::RetentionRefused,
                "Dock horizon has expired before launch",
            ));
        }
        let mut intent = DockLocalEvaluationAttemptIntentV1 {
            schema_ref: "handbook.dock.local-evaluation-attempt-intent@1.0.0".into(),
            attempt_ref: String::new(),
            attempt_fingerprint: fingerprint_bytes(b""),
            occurrence: coordinates.occurrence.clone(),
            basis: coordinates.basis.clone(),
            selection: coordinates.selection.clone(),
            authorization: coordinates.authorization.clone(),
            capability_observation: coordinates.capability_observation.clone(),
            ordinal: 1,
            request: request.binding().map_err(|_| substituted())?,
            evaluation_run: request.run().evaluation_run().clone(),
            request_jcs: String::from_utf8(canonical_bytes(request).map_err(|_| substituted())?)
                .map_err(|_| substituted())?,
            evaluation_run_jcs: String::from_utf8(run.to_vec()).map_err(|_| substituted())?,
            read_paths: reach.read_paths.clone(),
            write_subtrees: reach.write_subtrees.clone(),
            retention_until,
            lock_generation: FingerprintV1::parse(
                &self
                    .root
                    .existing_lock_physical_fingerprint_v1(LOCK)
                    .map_err(|_| unavailable())?,
            )
            .map_err(|_| substituted())?,
        };
        intent.attempt_fingerprint =
            record_fingerprint(&intent, &["attempt_ref", "attempt_fingerprint"])?;
        intent.attempt_ref = format!(
            "handbook.dock.local-evaluation-attempt@1.0.0#{}",
            fingerprint_hex(&intent.attempt_fingerprint)
        );
        intent.validate()?;
        Ok(intent)
    }
    fn reserve_intent(
        &self,
        intent: DockLocalEvaluationAttemptIntentV1,
    ) -> Result<(DockLocalEvaluationAttemptIntentV1, bool), DockLocalEvaluationErrorV1> {
        let reach = intent.reach();
        reach.mutation()?;
        let lock = self
            .root
            .acquire_existing_exclusive_lock_v1(LOCK)
            .map_err(|_| unavailable())?;
        if intent.lock_generation.as_str()
            != lock.physical_fingerprint_v1().map_err(|_| unavailable())?
        {
            return Err(substituted());
        }
        self.root
            .create_directories(&format!("{DOCK_LOCAL_EVALUATION_ROOT_V1}/attempts"))
            .map_err(|_| unavailable())?;
        for entry in self
            .root
            .list_directory_bounded(&format!("{DOCK_LOCAL_EVALUATION_ROOT_V1}/attempts"), 4096)
            .map_err(|_| unavailable())?
        {
            let path = format!(
                "{DOCK_LOCAL_EVALUATION_ROOT_V1}/attempts/{}/intent.json",
                entry.name()
            );
            let bytes = self.read_optional(&path, &reach)?.ok_or_else(substituted)?;
            let existing: DockLocalEvaluationAttemptIntentV1 =
                admit_canonical_json(&bytes, MAX_BYTES).map_err(|_| substituted())?;
            existing.validate()?;
            if existing.occurrence.fingerprint() == intent.occurrence.fingerprint() {
                if existing != intent {
                    return Err(error(
                        DockLocalEvaluationErrorKindV1::Conflict,
                        "one occurrence cannot acquire a second native attempt",
                    ));
                }
                lock.release().map_err(|_| unavailable())?;
                return Ok((existing, false));
            }
        }
        let directory = attempt_directory(&intent.attempt_fingerprint);
        self.root
            .create_directory_new_durable(&directory)
            .map_err(|_| {
                error(
                    DockLocalEvaluationErrorKindV1::Uncertain,
                    "native attempt allocation is uncertain",
                )
            })?;
        self.root
            .write_new_durable(&intent.relative_path(), &intent.canonical_json_bytes()?)
            .map_err(|_| {
                error(
                    DockLocalEvaluationErrorKindV1::Uncertain,
                    "native pre-effect attempt intent is uncertain",
                )
            })?;
        lock.release().map_err(|_| {
            error(
                DockLocalEvaluationErrorKindV1::Uncertain,
                "native attempt reservation lock release uncertain",
            )
        })?;
        Ok((intent, true))
    }
    fn commit_prelaunch_terminal(
        &self,
        intent: DockLocalEvaluationAttemptIntentV1,
        kind: &str,
        reason: &str,
        replayed: bool,
    ) -> Result<DockLocalEvaluationAttemptOutcomeV1, DockLocalEvaluationErrorV1> {
        let terminal = self.make_terminal(&intent, kind, None, None, vec![], reason)?;
        self.commit_terminal(intent, terminal, replayed)
    }
    fn commit_receipt_terminal(
        &self,
        intent: DockLocalEvaluationAttemptIntentV1,
        receipt: &DockStoreReceiptV1,
        replayed: bool,
    ) -> Result<DockLocalEvaluationAttemptOutcomeV1, DockLocalEvaluationErrorV1> {
        let execution = receipt.execution_record();
        if execution.request() != &intent.request
            || receipt.evaluation_run_jcs() != intent.evaluation_run_jcs.as_bytes()
            || execution.evaluation_run() != &intent.evaluation_run
        {
            return Err(substituted());
        }
        let kind = if execution.cleanup() == DockCleanupDispositionV1::Uncertain {
            "cleanup_uncertain"
        } else {
            match execution.host_outcome() {
                DockHostOutcomeV1::Completed => "completed",
                DockHostOutcomeV1::Refused => "refused",
                DockHostOutcomeV1::TimedOut => "timed_out",
                DockHostOutcomeV1::Cancelled => "cancelled",
                DockHostOutcomeV1::Failed => "failed",
                DockHostOutcomeV1::ProtocolError => "protocol_error",
            }
        };
        let terminal = self.make_terminal(
            &intent,
            kind,
            Some(execution.binding().map_err(|_| substituted())?),
            Some(receipt.root().clone()),
            receipt.candidates().to_vec(),
            "exact native execution terminal",
        )?;
        self.commit_terminal(intent, terminal, replayed)
    }
    fn make_terminal(
        &self,
        intent: &DockLocalEvaluationAttemptIntentV1,
        kind: &str,
        execution: Option<ContentAddressedBindingV1>,
        store_root: Option<FingerprintV1>,
        candidates: Vec<ContentAddressedBindingV1>,
        reason: &str,
    ) -> Result<DockLocalEvaluationAttemptTerminalV1, DockLocalEvaluationErrorV1> {
        let mut terminal = DockLocalEvaluationAttemptTerminalV1 {
            schema_ref: "handbook.dock.local-evaluation-attempt-terminal@1.0.0".into(),
            attempt: intent.binding()?,
            kind: kind.into(),
            execution,
            store_root,
            candidates,
            reason: reason.into(),
            retention_until: intent.retention_until.clone(),
            terminal_fingerprint: fingerprint_bytes(b""),
        };
        terminal.terminal_fingerprint = record_fingerprint(&terminal, &["terminal_fingerprint"])?;
        terminal.validate()?;
        Ok(terminal)
    }
    fn require_terminal(
        &self,
        intent: &DockLocalEvaluationAttemptIntentV1,
        terminal: &DockLocalEvaluationAttemptTerminalV1,
    ) -> Result<(), DockLocalEvaluationErrorV1> {
        terminal.validate()?;
        if terminal.attempt != intent.binding()?
            || terminal.retention_until != intent.retention_until
        {
            return Err(substituted());
        }
        Ok(())
    }
    fn commit_terminal(
        &self,
        intent: DockLocalEvaluationAttemptIntentV1,
        terminal: DockLocalEvaluationAttemptTerminalV1,
        replayed: bool,
    ) -> Result<DockLocalEvaluationAttemptOutcomeV1, DockLocalEvaluationErrorV1> {
        self.require_terminal(&intent, &terminal)?;
        intent.reach().mutation()?;
        let lock = self
            .root
            .acquire_existing_exclusive_lock_v1(LOCK)
            .map_err(|_| unavailable())?;
        if intent.lock_generation.as_str()
            != lock.physical_fingerprint_v1().map_err(|_| unavailable())?
        {
            return Err(substituted());
        }
        if self
            .read_optional(&intent.relative_path(), &intent.reach())?
            .as_deref()
            != Some(intent.canonical_json_bytes()?.as_slice())
        {
            return Err(substituted());
        }
        self.write_exact(
            &intent.terminal_path(),
            &canonical_bytes(&terminal).map_err(|_| substituted())?,
        )?;
        lock.release().map_err(|_| unavailable())?;
        Ok(DockLocalEvaluationAttemptOutcomeV1::Terminal {
            intent,
            terminal,
            replayed,
        })
    }
    fn validate_terminal_dependencies(
        &self,
        intent: &DockLocalEvaluationAttemptIntentV1,
        terminal: &DockLocalEvaluationAttemptTerminalV1,
        reach: &DockLocalEvaluationReachV1,
        recover: bool,
    ) -> Result<(), DockLocalEvaluationErrorV1> {
        let Some(execution) = &terminal.execution else {
            return Ok(());
        };
        reach.read(DOCK_OPERATIONAL_STORE_RELATIVE_ROOT_V1)?;
        let request: DockProcessRequestV1 =
            admit_canonical_json(intent.request_jcs.as_bytes(), MAX_BYTES)
                .map_err(|_| substituted())?;
        let store = DockOperationalStoreV1::open_existing_for_recovery_v3(self.root_path()?)
            .map_err(|_| unavailable())?;
        let receipt = store
            .replay_existing_for_request_v3(&request, &attempt_key(intent), recover)
            .map_err(|_| unavailable())?
            .ok_or_else(unavailable)?;
        if receipt.root() != terminal.store_root.as_ref().ok_or_else(substituted)?
            || receipt
                .execution_record()
                .binding()
                .map_err(|_| substituted())?
                != *execution
            || receipt.candidates() != terminal.candidates
        {
            return Err(substituted());
        }
        let expected = self.make_terminal(
            intent,
            &terminal.kind,
            Some(execution.clone()),
            terminal.store_root.clone(),
            terminal.candidates.clone(),
            "exact native execution terminal",
        )?;
        if expected != *terminal {
            return Err(substituted());
        }
        Ok(())
    }
}

fn record_fingerprint(
    value: &impl Serialize,
    remove: &[&str],
) -> Result<FingerprintV1, DockLocalEvaluationErrorV1> {
    let mut value = serde_json::to_value(value).map_err(|_| substituted())?;
    let object = value.as_object_mut().ok_or_else(substituted)?;
    for key in remove {
        object.remove(*key);
    }
    canonical_fingerprint(&value).map_err(|_| substituted())
}
fn fingerprint_bytes(bytes: &[u8]) -> FingerprintV1 {
    use sha2::{Digest, Sha256};
    FingerprintV1::parse(&format!("sha256:{:x}", Sha256::digest(bytes)))
        .expect("native SHA256 digest")
}
fn attempt_directory(fp: &FingerprintV1) -> String {
    format!(
        "{DOCK_LOCAL_EVALUATION_ROOT_V1}/attempts/{}",
        fingerprint_hex(fp)
    )
}
fn attempt_key(intent: &DockLocalEvaluationAttemptIntentV1) -> String {
    format!(
        "local-evaluation-{}",
        fingerprint_hex(&intent.attempt_fingerprint)
    )
}
fn store_key_fingerprint(key: &str) -> FingerprintV1 {
    use sha2::{Digest, Sha256};
    let mut digest = Sha256::new();
    digest.update(b"handbook.dock-run-idempotency-key@1.0.0\0");
    digest.update(key.as_bytes());
    FingerprintV1::parse(&format!("sha256:{:x}", digest.finalize()))
        .expect("native SHA256 identity")
}
fn valid_path(path: &str) -> bool {
    let path = path.strip_suffix("/**").unwrap_or(path);
    !path.is_empty()
        && !path.starts_with('/')
        && !path.contains(['\\', ':', '*'])
        && path
            .split('/')
            .all(|p| !p.is_empty() && p != "." && p != "..")
}
fn covers(declared: &str, actual: &str) -> bool {
    declared == actual
        || declared.strip_suffix("/**").is_some_and(|base| {
            actual == base
                || actual
                    .strip_prefix(base)
                    .is_some_and(|tail| tail.starts_with('/'))
        })
}
fn native_read_limit(path: &str) -> Result<usize, DockLocalEvaluationErrorV1> {
    let manifest = crate::registry::production_manifest_v1();
    let package_root = format!(
        "{DOCK_LOCAL_EVALUATION_ROOT_V1}/packages/{}/",
        fingerprint_hex(manifest.implementation_bundle().bundle_fingerprint())
    );
    if let Some(relative) = path.strip_prefix(&package_root) {
        if let Some(member) = manifest
            .implementation_bundle()
            .files()
            .iter()
            .find(|member| member.path().as_str() == relative)
        {
            let length = usize::try_from(member.length()).map_err(|_| substituted())?;
            if length > MAX_NATIVE_ARTIFACT_BYTES {
                return Err(substituted());
            }
            return Ok(length);
        }
    }
    Ok(MAX_BYTES)
}

fn validate_run(
    bytes: &[u8],
    binding: &ContentAddressedBindingV1,
) -> Result<(), DockLocalEvaluationErrorV1> {
    let value: Value = admit_canonical_json(bytes, MAX_BYTES).map_err(|_| substituted())?;
    if value["evaluation_run_ref"] != binding.reference()
        || value["evaluation_run_fingerprint"] != binding.fingerprint().as_str()
    {
        return Err(substituted());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use handbook_contracts::{EvaluationCaseV1, EvaluationSubjectV1, ExactRefV1, StableIdV1};
    fn content(reference: &str) -> ContentAddressedBindingV1 {
        ContentAddressedBindingV1::new(reference, fingerprint_bytes(reference.as_bytes())).unwrap()
    }
    fn reach() -> DockLocalEvaluationReachV1 {
        DockLocalEvaluationReachV1 {
            read_paths: vec![
                format!("{DOCK_LOCAL_EVALUATION_ROOT_V1}/**"),
                format!("{DOCK_OPERATIONAL_STORE_RELATIVE_ROOT_V1}/**"),
                LOCK.into(),
            ],
            write_subtrees: vec![
                format!("{DOCK_LOCAL_EVALUATION_ROOT_V1}/**"),
                format!("{DOCK_OPERATIONAL_STORE_RELATIVE_ROOT_V1}/**"),
                LOCK.into(),
            ],
        }
    }
    fn request(payload: Value) -> DockProcessRequestV1 {
        let manifest = crate::registry::production_manifest_v1();
        let input_role = &manifest.supported_inputs()[0];
        let input = DockInlineJsonInputV1::new(
            input_role.role().clone(),
            input_role.media_type(),
            input_role.schema().clone(),
            payload,
        )
        .unwrap();
        let subject: EvaluationSubjectV1 = input.captured_evaluation_subject_v1().unwrap();
        let case: EvaluationCaseV1 = input
            .captured_evaluation_case_v1(StableIdV1::parse("case.mechanical").unwrap())
            .unwrap();
        let run = DockEvaluationRunBindingV1::new(DockEvaluationRunBindingInputV1 {
            evaluation_run: content("handbook.contract.mechanical@1.0.0#evaluation/exact"),
            evaluation_lifecycle_basis: content("handbook.contract.mechanical@1.0.0#basis/exact"),
            contract: ExactBindingV1::new(
                ExactRefV1::parse("handbook.contract.mechanical@1.0.0").unwrap(),
                fingerprint_bytes(b"contract"),
            ),
            claim_ids: vec![StableIdV1::parse("claim.schema-conformance").unwrap()],
            subject,
            cases: vec![case],
            requested_resolution: manifest.resolution_ceiling().clone(),
            semantic_time_utc: SemanticTimeV1::parse("2026-10-06T00:00:00Z").unwrap(),
            lifecycle_head_fingerprint: fingerprint_bytes(b"head"),
        })
        .unwrap();
        DockProcessRequestV1::new(
            &manifest,
            DockProcessRequestInputV1 {
                run,
                requested_evidence_kinds: manifest.supported_evidence_kinds().to_vec(),
                inputs: vec![input],
                effective_limits: manifest.limits().clone(),
            },
        )
        .unwrap()
    }
    fn intent(
        owner: &DockLocalEvaluationOwnerV1,
        payload: Value,
    ) -> DockLocalEvaluationAttemptIntentV1 {
        let request = request(payload);
        let run=canonical_bytes(&json!({"evaluation_run_ref":request.run().evaluation_run().reference(),"evaluation_run_fingerprint":request.run().evaluation_run().fingerprint(),"schema_ref":"mechanical_owner_record"})).unwrap();
        let coordinates = DockLocalEvaluationCoordinatesV1 {
            occurrence: content("handbook.contract.mechanical-occurrence@1.0.0#one"),
            basis: content("handbook.contract.mechanical-basis@1.0.0#one"),
            selection: crate::registry::production_manifest_v1().binding(),
            authorization: content("handbook.engine.mechanical@1.0.0#issuance/one"),
            capability_observation: content(
                "handbook.dock.mechanical-capability@1.0.0#observation/one",
            ),
        };
        owner
            .make_intent(
                &coordinates,
                &request,
                &run,
                &reach(),
                SemanticTimeV1::parse("2030-01-01T00:00:00Z").unwrap(),
            )
            .unwrap()
    }
    fn repository() -> (tempfile::TempDir, DockLocalEvaluationOwnerV1) {
        let repo = tempfile::tempdir().unwrap();
        DockOperationalStoreV1::open(repo.path()).unwrap();
        let owner = DockLocalEvaluationOwnerV1::open_existing(repo.path()).unwrap();
        (repo, owner)
    }
    /// Exercises the file boundary using the actual frozen production bytes.
    /// This private proof test does not construct any authorization admission.
    #[test]
    #[ignore = "requires the exact frozen production package beside the test executable"]
    fn frozen_native_member_uses_exact_artifact_bound_for_repeat_and_launch() {
        let (_repo, owner) = repository();
        let bundle = DockResolvedFirstPartyBundleV1::resolve_production().unwrap();
        let manifest = bundle.manifest();
        let package_root = format!(
            "{DOCK_LOCAL_EVALUATION_ROOT_V1}/packages/{}",
            fingerprint_hex(manifest.implementation_bundle().bundle_fingerprint())
        );
        owner.root.create_directories(&package_root).unwrap();
        owner
            .root
            .create_directories(&format!("{DOCK_LOCAL_EVALUATION_ROOT_V1}/capabilities"))
            .unwrap();
        let manifest_path = format!("{package_root}/manifest.json");
        owner
            .write_exact(&manifest_path, &manifest.canonical_json_bytes().unwrap())
            .unwrap();
        let mut package_files = Vec::new();
        let mut largest = None;
        for (relative, bytes) in bundle.capture_verified_files_v1().unwrap() {
            let path = format!("{package_root}/{relative}");
            owner
                .root
                .create_directories(path.rsplit_once('/').unwrap().0)
                .unwrap();
            assert_eq!(native_read_limit(&path).unwrap(), bytes.len());
            assert!(bytes.len() <= MAX_NATIVE_ARTIFACT_BYTES);
            owner.write_exact(&path, &bytes).unwrap();
            owner.write_exact(&path, &bytes).unwrap();
            assert_eq!(
                owner.read_optional(&path, &reach()).unwrap(),
                Some(bytes.clone())
            );
            package_files
                .push(json!({"path":path,"length":bytes.len(),"sha256":fingerprint_bytes(&bytes)}));
            if largest
                .as_ref()
                .is_none_or(|(_, old): &(String, Vec<u8>)| old.len() < bytes.len())
            {
                largest = Some((path, bytes));
            }
        }
        let (artifact_path, artifact) = largest.unwrap();
        if cfg!(target_os = "windows") {
            assert!(
                artifact.len() > MAX_BYTES,
                "frozen Windows validator reproduces the old 8MiB boundary failure"
            );
            assert!(owner
                .root
                .read_regular_bounded(&artifact_path, MAX_BYTES)
                .is_err());
        }
        let record = NativeLocalEvaluationCapabilityV3::new(
            manifest.binding(),
            ContentAddressedBindingV1::new(
                manifest.implementation_bundle().bundle_ref().as_str(),
                manifest.implementation_bundle().bundle_fingerprint().clone(),
            ).unwrap(),
            host_target_tuple_v1().into(),
            "available".into(),
            json!({"profile":"first_party_native_advisory","environment":"cleared","cwd":"private",
                "time":"monotonic_bounded","output":"bounded","cancellation":"host_signal_and_bounded_termination",
                "network_enforcement":"not_provided","availability_reason":"exact_production_package_verified",
                "target":host_target_tuple_v1(),"manifest_path":manifest_path,"package_files":package_files,
                "lock_generation":owner.root.existing_lock_physical_fingerprint_v1(LOCK).unwrap(),
                "operating_ceiling":manifest.resolution_ceiling(),"effective_limits":manifest.limits()}),
            SemanticTimeV1::owner_observation_now_v1().unwrap(),
            SemanticTimeV1::parse("2098-01-01T00:00:00Z").unwrap(),
        ).unwrap();
        owner
            .write_exact(
                &record.relative_path(),
                &record.canonical_json_bytes().unwrap(),
            )
            .unwrap();
        let paths = owner.capability_paths(&record, &reach()).unwrap();
        let checkpoint = owner
            .root
            .retain_exact_files_checkpoint_v1(&paths.iter().map(String::as_str).collect::<Vec<_>>())
            .unwrap();
        owner
            .validate_launch_capability(&record, &checkpoint, &reach())
            .unwrap();
        drop(checkpoint);
        let mut changed = artifact.clone();
        changed[0] ^= 1;
        assert_eq!(
            owner
                .write_exact(&artifact_path, &changed)
                .unwrap_err()
                .kind(),
            DockLocalEvaluationErrorKindV1::Substituted
        );
        let mut oversized = artifact;
        oversized.push(0);
        std::fs::write(owner.root_path().unwrap().join(&artifact_path), oversized).unwrap();
        assert!(owner.read_optional(&artifact_path, &reach()).is_err());
        assert_eq!(
            native_read_limit(&format!("{package_root}/unrecognized.bin")).unwrap(),
            MAX_BYTES
        );
        assert_eq!(
            native_read_limit(&record.relative_path()).unwrap(),
            MAX_BYTES
        );
    }
    #[test]
    fn a_reserved_intent_is_not_absence_or_permission_to_retry_after_reopen() {
        let (repo, owner) = repository();
        let intent = intent(&owner, json!({"captured":1}));
        let binding = intent.binding().unwrap();
        assert!(owner.reserve_intent(intent.clone()).unwrap().1);
        drop(owner);
        let owner = DockLocalEvaluationOwnerV1::open_existing(repo.path()).unwrap();
        assert!(!owner.reserve_intent(intent.clone()).unwrap().1);
        assert_eq!(
            owner
                .resolve_exact_attempt(&binding, &reach(), true)
                .unwrap(),
            DockLocalEvaluationAttemptOutcomeV1::EffectUncertain { intent }
        );
        assert!(
            std::fs::read_dir(repo.path().join(".handbook/state/dock/v1/executions"))
                .unwrap()
                .next()
                .is_none()
        );
    }
    #[test]
    fn exact_prelaunch_cancellation_is_retained_and_replayed_without_execution_records() {
        let (repo, owner) = repository();
        let (intent, _) = owner
            .reserve_intent(intent(&owner, json!({"captured":2})))
            .unwrap();
        let binding = intent.binding().unwrap();
        let original = owner
            .commit_prelaunch_terminal(
                intent,
                "cancelled_before_launch",
                "cancelled by native signal",
                false,
            )
            .unwrap();
        let DockLocalEvaluationAttemptOutcomeV1::Terminal {
            terminal: original_terminal,
            ..
        } = original
        else {
            panic!("terminal")
        };
        drop(owner);
        let owner = DockLocalEvaluationOwnerV1::open_existing(repo.path()).unwrap();
        let DockLocalEvaluationAttemptOutcomeV1::Terminal {
            terminal, replayed, ..
        } = owner
            .resolve_exact_attempt(&binding, &reach(), false)
            .unwrap()
        else {
            panic!("terminal")
        };
        assert!(replayed);
        assert_eq!(terminal, original_terminal);
        assert!(terminal.execution.is_none());
        assert!(
            std::fs::read_dir(repo.path().join(".handbook/state/dock/v1/executions"))
                .unwrap()
                .next()
                .is_none()
        );
    }
    #[test]
    fn one_occurrence_cannot_reserve_changed_input_or_a_second_basis() {
        let (_repo, owner) = repository();
        owner
            .reserve_intent(intent(&owner, json!({"captured":3})))
            .unwrap();
        assert_eq!(
            owner
                .reserve_intent(intent(&owner, json!({"captured":4})))
                .unwrap_err()
                .kind(),
            DockLocalEvaluationErrorKindV1::Conflict
        );
    }
    #[test]
    fn reach_is_checked_before_any_new_namespace_mutation() {
        let (repo, owner) = repository();
        let mut restricted = reach();
        restricted
            .write_subtrees
            .retain(|p| !p.starts_with(DOCK_LOCAL_EVALUATION_ROOT_V1));
        assert_eq!(
            restricted.mutation().unwrap_err().kind(),
            DockLocalEvaluationErrorKindV1::ReachRefused
        );
        assert!(!repo.path().join(DOCK_LOCAL_EVALUATION_ROOT_V1).exists());
        assert!(owner
            .root
            .existing_lock_physical_fingerprint_v1(LOCK)
            .is_ok());
    }
    #[test]
    fn lost_lock_inspection_never_creates_a_replacement() {
        let (repo, owner) = repository();
        let store = DockOperationalStoreV1::open_existing_for_recovery_v3(repo.path()).unwrap();
        let request = request(json!({"captured":"lost-lock"}));
        drop(owner);
        std::fs::remove_file(repo.path().join(LOCK)).unwrap();
        assert_eq!(
            DockLocalEvaluationOwnerV1::open_existing(repo.path())
                .err()
                .unwrap()
                .kind(),
            DockLocalEvaluationErrorKindV1::Unavailable
        );
        assert!(store
            .begin_existing_execution_with_idempotency_request_v3(
                &request,
                "original-lost-lock",
                request.request_fingerprint()
            )
            .is_err());
        assert!(store
            .replay_existing_for_request_v3(&request, "original-lost-lock", true)
            .is_err());
        assert!(!repo.path().join(LOCK).exists());
    }
    #[test]
    fn terminal_substitution_refuses_instead_of_changing_history() {
        let (repo, owner) = repository();
        let (intent, _) = owner
            .reserve_intent(intent(&owner, json!({"captured":5})))
            .unwrap();
        let binding = intent.binding().unwrap();
        let path = intent.terminal_path();
        owner
            .commit_prelaunch_terminal(intent, "refused_before_launch", "native refusal", false)
            .unwrap();
        let file = repo.path().join(&path);
        let mut value: Value = serde_json::from_slice(&std::fs::read(&file).unwrap()).unwrap();
        value["reason"] = json!("altered");
        std::fs::write(file, canonical_bytes(&value).unwrap()).unwrap();
        assert_eq!(
            owner
                .resolve_exact_attempt(&binding, &reach(), false)
                .unwrap_err()
                .kind(),
            DockLocalEvaluationErrorKindV1::Substituted
        );
    }
    #[test]
    fn absent_and_partially_allocated_attempts_have_distinct_recovery_outcomes() {
        let (_repo, owner) = repository();
        let original = intent(&owner, json!({"captured":"partial"}));
        let binding = original.binding().unwrap();
        owner
            .root
            .create_directories(&format!("{DOCK_LOCAL_EVALUATION_ROOT_V1}/attempts"))
            .unwrap();
        assert_eq!(
            owner
                .resolve_exact_attempt(&binding, &reach(), false)
                .unwrap(),
            DockLocalEvaluationAttemptOutcomeV1::Absent
        );
        owner
            .root
            .create_directories(&attempt_directory(&original.attempt_fingerprint))
            .unwrap();
        assert_eq!(
            owner
                .resolve_exact_attempt(&binding, &reach(), false)
                .unwrap_err()
                .kind(),
            DockLocalEvaluationErrorKindV1::Uncertain
        );
    }
    #[test]
    fn explicit_subtree_reach_never_matches_a_sibling_or_traversal() {
        assert!(covers(
            ".handbook/state/dock/v1/**",
            ".handbook/state/dock/v1/requests/a.json"
        ));
        assert!(!covers(
            ".handbook/state/dock/v1/**",
            ".handbook/state/dock/v11/requests/a.json"
        ));
        assert!(!valid_path(".handbook/state/dock/../contracts/**"));
    }
    #[test]
    fn separate_process_reservations_leave_one_uncertain_attempt_after_interrupted_delivery() {
        const CHILD_ROOT: &str = "HANDBOOK_DOCK_LOCAL_ATTEMPT_TEST_ROOT";
        if let Some(root) = std::env::var_os(CHILD_ROOT) {
            let owner = DockLocalEvaluationOwnerV1::open_existing(root).unwrap();
            let (_, fresh) = owner
                .reserve_intent(intent(&owner, json!({"captured":"native-process"})))
                .unwrap();
            // Native writes are durable; deliberately omit any execution/terminal
            // response and all normal process cleanup after reservation.
            std::process::exit(if fresh { 23 } else { 24 });
        }
        let (repo, owner) = repository();
        let original = intent(&owner, json!({"captured":"native-process"}));
        let binding = original.binding().unwrap();
        let mut children = Vec::new();
        for _ in 0..2 {
            children.push(std::process::Command::new(std::env::current_exe().unwrap())
                .args(["--exact","local_evaluation::tests::separate_process_reservations_leave_one_uncertain_attempt_after_interrupted_delivery","--nocapture"])
                .env(CHILD_ROOT,repo.path()).spawn().unwrap());
        }
        let mut statuses = children
            .into_iter()
            .map(|mut child| child.wait().unwrap().code().unwrap())
            .collect::<Vec<_>>();
        statuses.sort();
        assert_eq!(statuses, [23, 24]);
        drop(owner);
        let reopened = DockLocalEvaluationOwnerV1::open_existing(repo.path()).unwrap();
        assert_eq!(
            reopened
                .resolve_exact_attempt(&binding, &reach(), true)
                .unwrap(),
            DockLocalEvaluationAttemptOutcomeV1::EffectUncertain { intent: original }
        );
        assert_eq!(
            std::fs::read_dir(
                repo.path()
                    .join(format!("{DOCK_LOCAL_EVALUATION_ROOT_V1}/attempts"))
            )
            .unwrap()
            .count(),
            1
        );
    }
}
