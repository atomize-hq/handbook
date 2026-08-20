#[cfg(feature = "test-support")]
use crate::DockStoreFaultBoundaryV1;
use crate::{
    DockExecutionRecordInputV1, DockExecutionRecordV1, DockExecutionStartV1,
    DockHostOutcomeFactsV1, DockManifestV1, DockOperationalStoreV1, DockProcessRequestV1,
    DockProcessResultV1, DockStoreErrorKindV1, DockStoreReceiptV1, DOCK_PROCESS_PROTOCOL_REF_V1,
};
use handbook_contracts::FingerprintV1;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;
use std::fs::{self, File};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, ExitStatus, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

/// The exact Cargo target tuple for the running first-party host.
pub fn host_target_tuple_v1() -> &'static str {
    env!("HANDBOOK_DOCK_TARGET")
}

/// An idempotent host-owned cancellation signal for one dock execution.
#[derive(Clone, Debug, Default)]
pub struct DockCancellationV1 {
    requested: Arc<AtomicBool>,
}

impl DockCancellationV1 {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn cancel(&self) -> bool {
        !self.requested.swap(true, Ordering::AcqRel)
    }

    pub fn is_requested(&self) -> bool {
        self.requested.load(Ordering::Acquire)
    }
}

/// Portable first-party host for the one-shot dock process protocol.
#[derive(Clone, Copy, Debug, Default)]
pub struct DockProcessExecutorV1;

pub type DockExecutionReceiptV1 = DockStoreReceiptV1;

/// An owner-resolved first-party bundle capability.
///
/// Production code cannot construct this capability from a caller-supplied
/// path. The empty production registry therefore admits no executable bundle.
pub struct DockResolvedFirstPartyBundleV1 {
    manifest: DockManifestV1,
    bundle_root: PathBuf,
}

impl DockResolvedFirstPartyBundleV1 {
    #[cfg(feature = "test-support")]
    pub fn for_testing(
        manifest: DockManifestV1,
        bundle_root: impl AsRef<Path>,
    ) -> Result<Self, DockExecutorErrorV1> {
        let bytes = manifest
            .canonical_json_bytes()
            .map_err(|_| admission("manifest bytes were refused"))?;
        let manifest = DockManifestV1::admit_canonical_json(&bytes)
            .map_err(|_| admission("manifest bytes were refused"))?;
        Ok(Self {
            manifest,
            bundle_root: bundle_root.as_ref().to_owned(),
        })
    }
}

pub struct DockExecutionInputV1<'a> {
    pub store: &'a DockOperationalStoreV1,
    pub resolved_bundle: &'a DockResolvedFirstPartyBundleV1,
    pub request: &'a DockProcessRequestV1,
    pub evaluation_run_jcs: &'a [u8],
    pub idempotency_key: &'a str,
    pub cancellation: &'a DockCancellationV1,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DockExecutorErrorKindV1 {
    AdmissionRefused,
    IdempotencyConflict,
    CancelledBeforeLaunch,
    DurabilityUncertain,
    InternalInvariant,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DockExecutorErrorV1 {
    kind: DockExecutorErrorKindV1,
    detail: &'static str,
}

impl DockExecutorErrorV1 {
    pub fn kind(&self) -> DockExecutorErrorKindV1 {
        self.kind
    }

    pub fn detail(&self) -> &'static str {
        self.detail
    }
}

impl fmt::Display for DockExecutorErrorV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.detail)
    }
}

impl Error for DockExecutorErrorV1 {}

#[cfg(feature = "test-support")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DockExecutorFaultV1 {
    CleanupUncertain,
    SubstituteSourceAfterPrivateStage,
    Store(DockStoreFaultBoundaryV1),
}

impl DockProcessExecutorV1 {
    pub fn new() -> Self {
        Self
    }

    pub fn execute(
        &self,
        input: DockExecutionInputV1<'_>,
    ) -> Result<DockExecutionReceiptV1, DockExecutorErrorV1> {
        self.execute_inner(input, None)
    }

    #[cfg(feature = "test-support")]
    pub fn execute_with_fault_for_testing(
        &self,
        input: DockExecutionInputV1<'_>,
        fault: DockExecutorFaultV1,
    ) -> Result<DockExecutionReceiptV1, DockExecutorErrorV1> {
        self.execute_inner(input, Some(fault))
    }

    fn execute_inner(
        &self,
        input: DockExecutionInputV1<'_>,
        #[cfg(feature = "test-support")] fault: Option<DockExecutorFaultV1>,
        #[cfg(not(feature = "test-support"))] _fault: Option<()>,
    ) -> Result<DockExecutionReceiptV1, DockExecutorErrorV1> {
        let manifest_bytes = input
            .resolved_bundle
            .manifest
            .canonical_json_bytes()
            .map_err(|_| admission("manifest bytes were refused"))?;
        let manifest = DockManifestV1::admit_canonical_json(&manifest_bytes)
            .map_err(|_| admission("manifest bytes were refused"))?;
        let request_bytes = input
            .request
            .framed_json_bytes()
            .map_err(|_| admission("request bytes were refused"))?;
        let request = DockProcessRequestV1::admit_framed_json(&request_bytes, &manifest)
            .map_err(|_| admission("request bytes were refused"))?;
        if manifest != input.resolved_bundle.manifest
            || &request != input.request
            || request.manifest() != &manifest.binding()
            || request.implementation_bundle() != &manifest.implementation_bundle().binding()
            || manifest.implementation_bundle().target().as_str() != host_target_tuple_v1()
        {
            return Err(admission(
                "exact manifest, request, bundle, or target was substituted",
            ));
        }
        let lease = match input
            .store
            .begin_execution(&request, input.idempotency_key)
            .map_err(map_store_error)?
        {
            DockExecutionStartV1::Replay(receipt) => return Ok(*receipt),
            DockExecutionStartV1::Lease(lease) => lease,
        };
        if input.cancellation.is_requested() {
            return Err(DockExecutorErrorV1 {
                kind: DockExecutorErrorKindV1::CancelledBeforeLaunch,
                detail: "dock execution was cancelled before launch",
            });
        }

        verify_bundle(&manifest, &input.resolved_bundle.bundle_root)?;
        let private_root = tempfile::Builder::new()
            .prefix("handbook-dock-")
            .tempdir()
            .map_err(|_| admission("private execution directory is unavailable"))?;
        let staged_bundle = private_root.path().join("bundle");
        let private_cwd = private_root.path().join("work");
        fs::create_dir(&staged_bundle)
            .and_then(|_| fs::create_dir(&private_cwd))
            .map_err(|_| admission("private execution directory is unavailable"))?;
        stage_bundle(
            &manifest,
            &input.resolved_bundle.bundle_root,
            &staged_bundle,
        )?;
        let entrypoint = verify_bundle(&manifest, &staged_bundle)?;
        #[cfg(feature = "test-support")]
        if fault == Some(DockExecutorFaultV1::SubstituteSourceAfterPrivateStage) {
            substitute_source_entrypoint_for_testing(
                &manifest,
                &input.resolved_bundle.bundle_root,
            )?;
        }
        if input.cancellation.is_requested() {
            return Err(DockExecutorErrorV1 {
                kind: DockExecutorErrorKindV1::CancelledBeforeLaunch,
                detail: "dock execution was cancelled before launch",
            });
        }
        let request_frame = request
            .framed_json_bytes()
            .map_err(|_| invariant("admitted request framing became unavailable"))?;
        let stdin_path = private_root.path().join("stdin.json");
        let stdout_path = private_root.path().join("stdout.json");
        let stderr_path = private_root.path().join("stderr.log");
        fs::write(&stdin_path, &request_frame)
            .map_err(|_| admission("private request stream is unavailable"))?;
        let stdin = File::open(&stdin_path)
            .map_err(|_| admission("private request stream is unavailable"))?;
        let stdout = File::create(&stdout_path)
            .map_err(|_| admission("private result stream is unavailable"))?;
        let stderr = File::create(&stderr_path)
            .map_err(|_| admission("private diagnostic stream is unavailable"))?;
        let stdout_limit = request
            .effective_limits()
            .result_bytes()
            .checked_add(1)
            .ok_or_else(|| invariant("framed result byte bound overflowed"))?;
        let mut command = Command::new(&entrypoint);
        command
            .args(manifest.implementation_bundle().fixed_arguments())
            .env_clear()
            .env("HANDBOOK_DOCK_PROTOCOL", DOCK_PROCESS_PROTOCOL_REF_V1)
            .current_dir(&private_cwd)
            .stdin(Stdio::from(stdin))
            .stdout(Stdio::from(stdout))
            .stderr(Stdio::from(stderr));
        let spawn = command.spawn();
        let execution = match spawn {
            Ok(child) => {
                let started = Instant::now();
                run_child(
                    child,
                    &request,
                    input.cancellation,
                    &stdout_path,
                    &stderr_path,
                    stdout_limit,
                    started,
                )
            }
            Err(_) => ChildExecution::spawn_failed(0),
        };
        let cleanup_confirmed = execution.termination_confirmed && private_root.close().is_ok();
        #[cfg(feature = "test-support")]
        let cleanup_confirmed =
            cleanup_confirmed && fault != Some(DockExecutorFaultV1::CleanupUncertain);
        let result = execution.result.as_ref();
        let record = DockExecutionRecordV1::new(DockExecutionRecordInputV1 {
            request: &request,
            result,
            facts: DockHostOutcomeFactsV1 {
                cleanup_confirmed,
                durability_confirmed: true,
                deadline_reached: execution.deadline_reached,
                cancellation_accepted: execution.cancellation_accepted,
                spawn_or_process_failed: execution.process_failed,
                framing_or_binding_failed: execution.protocol_failed,
                child_status: result.map(DockProcessResultV1::status),
            },
            exit_code: execution.exit_code,
            wall_time_ms: execution.wall_time_ms,
            stdout_bytes: execution.stdout.len() as u64,
            stdout_truncated: execution.stdout_truncated,
            stderr_bytes: execution.stderr.len() as u64,
            stderr_truncated: execution.stderr_truncated,
            diagnostics: vec![],
        })
        .map_err(|error| match error {
            crate::DockErrorV1::BoundExceeded => invariant("execution record bounds were exceeded"),
            _ => invariant("execution record construction failed"),
        })?;

        #[cfg(feature = "test-support")]
        let committed = lease.commit(
            &request,
            result,
            &record,
            input.evaluation_run_jcs,
            match fault {
                Some(DockExecutorFaultV1::Store(store_fault)) => Some(store_fault),
                _ => None,
            },
        );
        #[cfg(not(feature = "test-support"))]
        let committed = lease.commit(&request, result, &record, input.evaluation_run_jcs, None);
        committed.map_err(map_store_error)
    }
}

struct ChildExecution {
    result: Option<DockProcessResultV1>,
    exit_code: Option<i32>,
    stdout: Vec<u8>,
    stdout_truncated: bool,
    stderr: Vec<u8>,
    stderr_truncated: bool,
    deadline_reached: bool,
    cancellation_accepted: bool,
    process_failed: bool,
    protocol_failed: bool,
    termination_confirmed: bool,
    wall_time_ms: u64,
}

impl ChildExecution {
    fn spawn_failed(wall_time_ms: u64) -> Self {
        Self {
            result: None,
            exit_code: None,
            stdout: vec![],
            stdout_truncated: false,
            stderr: vec![],
            stderr_truncated: false,
            deadline_reached: false,
            cancellation_accepted: false,
            process_failed: true,
            protocol_failed: false,
            termination_confirmed: true,
            wall_time_ms,
        }
    }
}

fn run_child(
    mut child: Child,
    request: &DockProcessRequestV1,
    cancellation: &DockCancellationV1,
    stdout_path: &Path,
    stderr_path: &Path,
    stdout_limit: usize,
    started: Instant,
) -> ChildExecution {
    let timeout = Duration::from_millis(request.effective_limits().wall_timeout_ms());
    let grace = Duration::from_millis(request.effective_limits().cancellation_grace_ms());
    let deadline = started.checked_add(timeout).unwrap_or(started);
    let hard_end = deadline.checked_add(grace).unwrap_or(deadline);
    let mut deadline_reached = false;
    let mut cancellation_accepted = false;
    let mut process_failed = false;
    let mut protocol_failed = false;
    let mut status = None;
    let mut termination_end = None;
    let mut force_at = None;
    loop {
        let now = Instant::now();
        if now >= deadline {
            deadline_reached = true;
            termination_end = Some(hard_end);
            force_at = Some(now);
            break;
        }
        if cancellation.is_requested() {
            cancellation_accepted = true;
            let end = now.checked_add(grace).unwrap_or(hard_end).min(hard_end);
            let reserve = grace.min(Duration::from_millis(10));
            termination_end = Some(end);
            force_at = Some(deadline.min(end.checked_sub(reserve).unwrap_or(end)));
            break;
        }
        if output_over_limit(stdout_path, stdout_limit)
            || output_over_limit(stderr_path, request.effective_limits().stderr_bytes())
        {
            protocol_failed = true;
            termination_end = Some(now.checked_add(grace).unwrap_or(hard_end).min(hard_end));
            force_at = Some(now);
            break;
        }
        match child.try_wait() {
            Ok(Some(value)) => {
                status = Some(value);
                break;
            }
            Ok(None) => bounded_poll_pause(deadline),
            Err(_) => {
                process_failed = true;
                termination_end = Some(now.checked_add(grace).unwrap_or(hard_end).min(hard_end));
                force_at = Some(now);
                break;
            }
        }
    }
    let mut termination_confirmed = status.is_some();
    if let Some(termination_end) = termination_end {
        let kill_at = force_at.unwrap_or(termination_end);
        while status.is_none() && Instant::now() < kill_at {
            if Instant::now() >= deadline {
                deadline_reached = true;
            }
            match child.try_wait() {
                Ok(Some(value)) => status = Some(value),
                Ok(None) => bounded_poll_pause(kill_at),
                Err(_) => {
                    process_failed = true;
                    break;
                }
            }
        }
        if status.is_none() {
            if child.kill().is_err() {
                process_failed = true;
            }
            while status.is_none() && Instant::now() < termination_end {
                if Instant::now() >= deadline {
                    deadline_reached = true;
                }
                match child.try_wait() {
                    Ok(Some(value)) => status = Some(value),
                    Ok(None) => bounded_poll_pause(termination_end),
                    Err(_) => {
                        process_failed = true;
                        break;
                    }
                }
            }
        }
        deadline_reached |= Instant::now() >= deadline;
        termination_confirmed = status.is_some();
        process_failed |= !termination_confirmed;
    }
    let wall_time_ms = elapsed_millis(started);
    drop(child);
    let captured_stdout = capture_file(stdout_path, stdout_limit);
    let captured_stderr = capture_file(stderr_path, request.effective_limits().stderr_bytes());
    let exit_code = status.as_ref().and_then(ExitStatus::code);
    let normal_zero = status.as_ref().is_some_and(ExitStatus::success)
        && !deadline_reached
        && !cancellation_accepted;
    let stream_failed = captured_stdout.truncated || captured_stderr.truncated;
    let result = if normal_zero && !stream_failed {
        DockProcessResultV1::admit_framed_json(&captured_stdout.bytes, request).ok()
    } else {
        None
    };
    protocol_failed |= normal_zero && (stream_failed || result.is_none());
    process_failed |= !protocol_failed
        && (!deadline_reached
            && !cancellation_accepted
            && status.as_ref().is_none_or(|value| !value.success()));
    ChildExecution {
        result,
        exit_code,
        stdout: captured_stdout.bytes,
        stdout_truncated: captured_stdout.truncated,
        stderr: captured_stderr.bytes,
        stderr_truncated: captured_stderr.truncated,
        deadline_reached,
        cancellation_accepted,
        process_failed,
        protocol_failed,
        termination_confirmed,
        wall_time_ms,
    }
}

fn bounded_poll_pause(end: Instant) {
    let remaining = end.saturating_duration_since(Instant::now());
    if !remaining.is_zero() {
        thread::sleep(remaining.min(Duration::from_millis(1)));
    }
}

struct CapturedStream {
    bytes: Vec<u8>,
    truncated: bool,
}

fn output_over_limit(path: &Path, limit: usize) -> bool {
    fs::metadata(path)
        .map(|metadata| metadata.len() > limit as u64)
        .unwrap_or(true)
}

fn capture_file(path: &Path, limit: usize) -> CapturedStream {
    let before = fs::metadata(path).ok().map(|metadata| metadata.len());
    let mut bytes = Vec::with_capacity(limit.min(64 * 1024));
    let read_limit = u64::try_from(limit).unwrap_or(u64::MAX).saturating_add(1);
    let read = File::open(path)
        .and_then(|file| file.take(read_limit).read_to_end(&mut bytes))
        .ok();
    let after = fs::metadata(path).ok().map(|metadata| metadata.len());
    let truncated = read.is_none()
        || before.is_none()
        || before != after
        || before.is_some_and(|length| length > limit as u64)
        || bytes.len() > limit;
    bytes.truncate(limit);
    CapturedStream { bytes, truncated }
}

fn stage_bundle(
    manifest: &DockManifestV1,
    source_root: &Path,
    staged_root: &Path,
) -> Result<(), DockExecutorErrorV1> {
    for definition in manifest.implementation_bundle().files() {
        let relative = definition
            .path()
            .as_str()
            .replace('/', std::path::MAIN_SEPARATOR_STR);
        let source = source_root.join(&relative);
        let destination = staged_root.join(&relative);
        let parent = destination
            .parent()
            .ok_or_else(|| invariant("staged bundle member has no parent"))?;
        fs::create_dir_all(parent)
            .map_err(|_| admission("private bundle staging is unavailable"))?;
        fs::copy(source, destination)
            .map_err(|_| admission("implementation bundle changed during private staging"))?;
    }
    Ok(())
}

#[cfg(feature = "test-support")]
fn substitute_source_entrypoint_for_testing(
    manifest: &DockManifestV1,
    source_root: &Path,
) -> Result<(), DockExecutorErrorV1> {
    let entry = source_root.join(
        manifest
            .implementation_bundle()
            .entrypoint()
            .as_str()
            .replace('/', std::path::MAIN_SEPARATOR_STR),
    );
    let length = fs::metadata(&entry)
        .map_err(|_| admission("test source entrypoint is unavailable"))?
        .len();
    let bytes = vec![
        0_u8;
        usize::try_from(length)
            .map_err(|_| admission("test source entrypoint is too large"))?
    ];
    fs::write(entry, bytes).map_err(|_| admission("test source entrypoint cannot be substituted"))
}

fn verify_bundle(
    manifest: &DockManifestV1,
    bundle_root: &Path,
) -> Result<PathBuf, DockExecutorErrorV1> {
    let root_metadata = fs::symlink_metadata(bundle_root)
        .map_err(|_| admission("implementation bundle is unavailable"))?;
    if !root_metadata.is_dir() || root_metadata.file_type().is_symlink() {
        return Err(admission("implementation bundle root was substituted"));
    }
    let root = fs::canonicalize(bundle_root)
        .map_err(|_| admission("implementation bundle root is unavailable"))?;
    let expected = manifest.implementation_bundle().files();
    let expected_files: BTreeSet<_> = expected
        .iter()
        .map(|file| file.path().as_str().to_owned())
        .collect();
    let mut expected_directories = BTreeSet::new();
    for path in &expected_files {
        let components: Vec<_> = path.split('/').collect();
        for length in 1..components.len() {
            expected_directories.insert(components[..length].join("/"));
        }
    }
    let mut actual = BTreeMap::new();
    collect_bundle_files(
        &root,
        &root,
        &expected_files,
        &expected_directories,
        &mut actual,
    )?;
    if actual.len() != expected.len() {
        return Err(admission("implementation bundle inventory was substituted"));
    }
    for file in expected {
        let Some(path) = actual.get(file.path().as_str()) else {
            return Err(admission("implementation bundle member is absent"));
        };
        let fingerprint = hash_regular_file(path, file.length())?;
        if &fingerprint != file.sha256() {
            return Err(admission("implementation bundle member was substituted"));
        }
    }
    let entrypoint = root.join(
        manifest
            .implementation_bundle()
            .entrypoint()
            .as_str()
            .replace('/', std::path::MAIN_SEPARATOR_STR),
    );
    let entry_definition = expected
        .iter()
        .find(|file| file.path() == manifest.implementation_bundle().entrypoint())
        .ok_or_else(|| admission("implementation entrypoint is absent"))?;
    let fingerprint = hash_regular_file(&entrypoint, entry_definition.length())?;
    if fingerprint != *manifest.implementation_bundle().entrypoint_fingerprint() {
        return Err(admission(
            "implementation executable bytes were substituted",
        ));
    }
    Ok(entrypoint)
}

fn collect_bundle_files(
    root: &Path,
    directory: &Path,
    expected_files: &BTreeSet<String>,
    expected_directories: &BTreeSet<String>,
    files: &mut BTreeMap<String, PathBuf>,
) -> Result<(), DockExecutorErrorV1> {
    for entry in fs::read_dir(directory)
        .map_err(|_| admission("implementation bundle inventory is unavailable"))?
    {
        let entry =
            entry.map_err(|_| admission("implementation bundle inventory is unavailable"))?;
        let path = entry.path();
        let metadata = fs::symlink_metadata(&path)
            .map_err(|_| admission("implementation bundle member is unavailable"))?;
        if metadata.file_type().is_symlink() {
            return Err(admission("implementation bundle symlink was refused"));
        }
        let relative = path
            .strip_prefix(root)
            .map_err(|_| admission("implementation bundle member escaped its root"))?;
        let relative = relative
            .to_str()
            .ok_or_else(|| admission("implementation bundle path is not UTF-8"))?
            .replace(std::path::MAIN_SEPARATOR, "/");
        crate::SafeRelativePathV1::parse(&relative)
            .map_err(|_| admission("implementation bundle path was refused"))?;
        if metadata.is_dir() {
            if !expected_directories.contains(&relative) {
                return Err(admission("implementation bundle directory was substituted"));
            }
            collect_bundle_files(root, &path, expected_files, expected_directories, files)?;
        } else if metadata.is_file() {
            if !expected_files.contains(&relative) {
                return Err(admission("implementation bundle member was substituted"));
            }
            if files.insert(relative, path).is_some() {
                return Err(admission("implementation bundle inventory is ambiguous"));
            }
        } else {
            return Err(admission(
                "implementation bundle member is not a regular file",
            ));
        }
    }
    Ok(())
}

fn hash_regular_file(
    path: &Path,
    expected_length: u64,
) -> Result<FingerprintV1, DockExecutorErrorV1> {
    let before = fs::symlink_metadata(path)
        .map_err(|_| admission("implementation bundle member is unavailable"))?;
    if !before.is_file() || before.file_type().is_symlink() {
        return Err(admission(
            "implementation bundle member is not a regular file",
        ));
    }
    if before.len() != expected_length {
        return Err(admission(
            "implementation bundle member length was substituted",
        ));
    }
    let mut input =
        File::open(path).map_err(|_| admission("implementation bundle member cannot be opened"))?;
    let mut digest = Sha256::new();
    let mut length = 0_u64;
    let mut buffer = [0_u8; 64 * 1024];
    loop {
        let count = input
            .read(&mut buffer)
            .map_err(|_| admission("implementation bundle member cannot be read"))?;
        if count == 0 {
            break;
        }
        length = length
            .checked_add(count as u64)
            .ok_or_else(|| admission("implementation bundle length overflowed"))?;
        digest.update(&buffer[..count]);
    }
    let after = fs::symlink_metadata(path)
        .map_err(|_| admission("implementation bundle member is unavailable"))?;
    if before.len() != after.len()
        || length != expected_length
        || length != after.len()
        || !after.is_file()
    {
        return Err(admission(
            "implementation bundle member changed during verification",
        ));
    }
    let fingerprint = FingerprintV1::parse(&format!("sha256:{:x}", digest.finalize()))
        .map_err(|_| invariant("implementation fingerprint construction failed"))?;
    Ok(fingerprint)
}

fn elapsed_millis(started: Instant) -> u64 {
    u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX)
}

fn map_store_error(error: crate::DockStoreErrorV1) -> DockExecutorErrorV1 {
    let kind = match error.kind() {
        DockStoreErrorKindV1::IdempotencyConflict => DockExecutorErrorKindV1::IdempotencyConflict,
        DockStoreErrorKindV1::DurabilityUncertain => DockExecutorErrorKindV1::DurabilityUncertain,
        _ => DockExecutorErrorKindV1::InternalInvariant,
    };
    DockExecutorErrorV1 {
        kind,
        detail: "dock operational commit was refused",
    }
}

const fn admission(detail: &'static str) -> DockExecutorErrorV1 {
    DockExecutorErrorV1 {
        kind: DockExecutorErrorKindV1::AdmissionRefused,
        detail,
    }
}

const fn invariant(detail: &'static str) -> DockExecutorErrorV1 {
    DockExecutorErrorV1 {
        kind: DockExecutorErrorKindV1::InternalInvariant,
        detail,
    }
}
