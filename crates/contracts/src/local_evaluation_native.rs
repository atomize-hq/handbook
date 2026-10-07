//! Contracts-recognized native Dock facts. Public records are data; only exact
//! retained native records under a pinned source checkpoint establish facts.
use crate::canonical::{canonical_bytes, canonical_fingerprint};
use crate::*;
use handbook_engine::{RepositoryExactFilesCheckpointV1, RetainedRepositoryRootV1};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

pub const NATIVE_LOCAL_EVALUATION_ROOT_V3: &str = ".handbook/state/dock/local-evaluation-v1";

pub fn native_local_evaluation_capability_binding_v3() -> Result<ExactBindingV1, ContractError> {
    Ok(ExactBindingV1::new(
        ExactRefV1::parse("handbook.dock.json-schema.semantic-capability@1.0.0")?,
        canonical_fingerprint(
            &json!({"schema_ref":"handbook.producer-semantic-capability@1.0.0",
            "producer":"handbook.dock.json-schema@1.0.0","role":"schema_conformance",
            "dialect":"https://json-schema.org/draft/2020-12/schema",
            "profile":"closed_offline_local_closure","input":"handbook.dock-json-schema-validation-input@1.0.0",
            "fact":"handbook.fact.schema-conformance@1.0.0","partial_completion":false,"max_attempts":1}),
        )?,
    ))
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NativeLocalEvaluationCapabilityV3 {
    pub schema_ref: String,
    pub semantic_capability: ExactBindingV1,
    pub manifest: ExactBindingV1,
    pub implementation_bundle: ContentAddressedBindingV1,
    pub target: String,
    pub availability: String,
    pub controls: Value,
    pub observed_at: SemanticTimeV1,
    pub retention_until: SemanticTimeV1,
    pub fingerprint: FingerprintV1,
}
impl NativeLocalEvaluationCapabilityV3 {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        manifest: ExactBindingV1,
        implementation_bundle: ContentAddressedBindingV1,
        target: String,
        availability: String,
        controls: Value,
        observed_at: SemanticTimeV1,
        retention_until: SemanticTimeV1,
    ) -> Result<Self, ContractError> {
        let mut value = Self {
            schema_ref: "handbook.dock.local-evaluation-capability@1.0.0".into(),
            semantic_capability: native_local_evaluation_capability_binding_v3()?,
            manifest,
            implementation_bundle,
            target,
            availability,
            controls,
            observed_at,
            retention_until,
            fingerprint: FingerprintV1::from_bytes(b""),
        };
        value.fingerprint = value.expected_fingerprint()?;
        value.validate()?;
        Ok(value)
    }
    fn expected_fingerprint(&self) -> Result<FingerprintV1, ContractError> {
        let mut value = serde_json::to_value(self).map_err(|_| ContractError::InvalidInvariant)?;
        value
            .as_object_mut()
            .ok_or(ContractError::InvalidInvariant)?
            .remove("fingerprint");
        canonical_fingerprint(&value)
    }
    pub fn validate(&self) -> Result<(), ContractError> {
        if self.schema_ref != "handbook.dock.local-evaluation-capability@1.0.0"
            || self.semantic_capability != native_local_evaluation_capability_binding_v3()?
            || !matches!(
                self.availability.as_str(),
                "available" | "known_limited" | "unknown"
            )
            || self.target.is_empty()
            || self.target.len() > 128
            || self.observed_at > self.retention_until
            || !self.controls.is_object()
            || self.fingerprint != self.expected_fingerprint()?
        {
            return Err(ContractError::InvalidInvariant);
        }
        Ok(())
    }
    pub fn canonical_json_bytes(&self) -> Result<Vec<u8>, ContractError> {
        self.validate()?;
        canonical_bytes(self)
    }
    pub fn binding(&self) -> Result<ContentAddressedBindingV1, ContractError> {
        ContentAddressedBindingV1::new(
            &format!(
                "handbook.dock.local-evaluation-capability@1.0.0#{}",
                self.fingerprint.as_str().trim_start_matches("sha256:")
            ),
            self.fingerprint.clone(),
        )
    }
    pub fn relative_path(&self) -> String {
        format!(
            "{NATIVE_LOCAL_EVALUATION_ROOT_V3}/capabilities/{}.json",
            self.fingerprint.as_str().trim_start_matches("sha256:")
        )
    }
}

pub struct ResolvedLocalEvaluationCapabilityV3<'a> {
    pub(crate) record: NativeLocalEvaluationCapabilityV3,
    pub(crate) root: &'a RetainedRepositoryRootV1,
    pub(crate) checkpoint: &'a RepositoryExactFilesCheckpointV1<'a>,
}
impl ResolvedLocalEvaluationCapabilityV3<'_> {
    pub(crate) fn require_repository_root(
        &self,
        root: &RetainedRepositoryRootV1,
    ) -> Result<(), ContractError> {
        if self
            .root
            .governance_physical_root_fingerprint_v1()
            .map_err(|_| ContractError::InvalidInvariant)?
            != root
                .governance_physical_root_fingerprint_v1()
                .map_err(|_| ContractError::InvalidInvariant)?
        {
            return Err(ContractError::InvalidInvariant);
        }
        self.validate_current()
    }
    pub fn record(&self) -> &NativeLocalEvaluationCapabilityV3 {
        &self.record
    }
    pub fn validate_current(&self) -> Result<(), ContractError> {
        self.record.validate()?;
        self.checkpoint
            .require_exact_file(
                self.root,
                &self.record.relative_path(),
                &self.record.canonical_json_bytes()?,
            )
            .map_err(|_| ContractError::InvalidInvariant)?;
        self.checkpoint
            .validate()
            .map_err(|_| ContractError::InvalidInvariant)
    }
}
pub fn resolve_native_local_evaluation_capability_v3<'a, R>(
    root: &'a RetainedRepositoryRootV1,
    checkpoint: &'a RepositoryExactFilesCheckpointV1<'a>,
    record: NativeLocalEvaluationCapabilityV3,
    dependencies: &LocalDefinitionDependenciesV2,
    consume: impl FnOnce(&ResolvedLocalEvaluationCapabilityV3<'a>) -> R,
) -> Result<R, ContractError> {
    record.validate()?;
    if record.manifest != dependencies.validator.manifest {
        return Err(ContractError::InvalidInvariant);
    }
    validate_capability_proof(root, checkpoint, &record, dependencies)?;
    checkpoint
        .require_file_membership(root, &[&record.relative_path()])
        .map_err(|_| ContractError::InvalidInvariant)?;
    let resolved = ResolvedLocalEvaluationCapabilityV3 {
        record,
        root,
        checkpoint,
    };
    resolved.validate_current()?;
    let result = consume(&resolved);
    resolved.validate_current()?;
    Ok(result)
}

fn pinned(
    root: &RetainedRepositoryRootV1,
    checkpoint: &RepositoryExactFilesCheckpointV1<'_>,
    path: &str,
) -> Result<Vec<u8>, ContractError> {
    checkpoint
        .require_file_membership(root, &[path])
        .map_err(|_| ContractError::InvalidInvariant)?;
    let handbook_engine::RepositoryReadV1::Found(bytes) = root
        .read_regular_bounded(path, 16 * 1024 * 1024)
        .map_err(|_| ContractError::InvalidInvariant)?
    else {
        return Err(ContractError::InvalidInvariant);
    };
    checkpoint
        .require_exact_file(root, path, &bytes)
        .map_err(|_| ContractError::InvalidInvariant)?;
    Ok(bytes)
}
fn parsed(
    root: &RetainedRepositoryRootV1,
    checkpoint: &RepositoryExactFilesCheckpointV1<'_>,
    path: &str,
) -> Result<Value, ContractError> {
    let bytes = pinned(root, checkpoint, path)?;
    let value: Value =
        serde_json::from_slice(&bytes).map_err(|_| ContractError::InvalidInvariant)?;
    if canonical_bytes(&value)? != bytes {
        return Err(ContractError::DerivedValueMismatch);
    }
    Ok(value)
}
fn binding(value: &Value) -> Result<ContentAddressedBindingV1, ContractError> {
    serde_json::from_value(value.clone()).map_err(|_| ContractError::InvalidInvariant)
}
fn text<'a>(value: &'a Value, key: &str) -> Result<&'a str, ContractError> {
    value[key].as_str().ok_or(ContractError::InvalidInvariant)
}
fn closed(value: &Value, keys: &[&str]) -> Result<(), ContractError> {
    let object = value.as_object().ok_or(ContractError::InvalidInvariant)?;
    if object.len() != keys.len() || object.keys().any(|key| !keys.contains(&key.as_str())) {
        return Err(ContractError::InvalidInvariant);
    }
    Ok(())
}
fn validate_capability_proof(
    root: &RetainedRepositoryRootV1,
    checkpoint: &RepositoryExactFilesCheckpointV1<'_>,
    record: &NativeLocalEvaluationCapabilityV3,
    dependencies: &LocalDefinitionDependenciesV2,
) -> Result<(), ContractError> {
    let recognized = LocalValidatorDependencyV2::from_manifest_json(&canonical_bytes(
        &dependencies.validator.canonical_manifest,
    )?)?;
    if recognized != dependencies.validator || record.manifest != recognized.manifest {
        return Err(ContractError::InvalidInvariant);
    }
    closed(
        &record.controls,
        &[
            "profile",
            "environment",
            "cwd",
            "time",
            "output",
            "cancellation",
            "network_enforcement",
            "availability_reason",
            "target",
            "manifest_path",
            "package_files",
            "lock_generation",
            "operating_ceiling",
            "effective_limits",
        ],
    )?;
    let reason = text(&record.controls, "availability_reason")?;
    if !matches!(
        (record.availability.as_str(), reason),
        ("available", "exact_production_package_verified")
            | ("known_limited", "production_package_absent")
            | ("unknown", "production_package_availability_unobservable")
    ) {
        return Err(ContractError::InvalidInvariant);
    }
    let lock_path = ".handbook/state/locks/dock-operational-store.lock";
    pinned(root, checkpoint, lock_path)?;
    if text(&record.controls, "lock_generation")?
        != root
            .existing_lock_physical_fingerprint_v1(lock_path)
            .map_err(|_| ContractError::InvalidInvariant)?
    {
        return Err(ContractError::InvalidInvariant);
    }
    let native_target = if cfg!(all(target_os = "windows", target_arch = "x86_64")) {
        "x86_64-pc-windows-msvc"
    } else if cfg!(all(target_os = "linux", target_arch = "x86_64")) {
        "x86_64-unknown-linux-gnu"
    } else {
        return Err(ContractError::InvalidInvariant);
    };
    if record.target != native_target
        || record.controls["target"] != native_target
        || record.controls["network_enforcement"] != "not_provided"
    {
        return Err(ContractError::InvalidInvariant);
    }
    for (key, value) in [
        ("profile", "first_party_native_advisory"),
        ("environment", "cleared"),
        ("cwd", "private"),
        ("time", "monotonic_bounded"),
        ("output", "bounded"),
        ("cancellation", "host_signal_and_bounded_termination"),
    ] {
        if record.controls[key] != value {
            return Err(ContractError::InvalidInvariant);
        }
    }
    let manifest = &dependencies.validator.canonical_manifest;
    let bundle = &manifest["implementation_bundle"];
    if bundle["target"] != record.target {
        return Err(ContractError::InvalidInvariant);
    }
    if record.implementation_bundle.reference() != text(bundle, "bundle_ref")?
        || record.implementation_bundle.fingerprint().as_str()
            != text(bundle, "bundle_fingerprint")?
    {
        return Err(ContractError::InvalidInvariant);
    }
    if record.controls["operating_ceiling"] != manifest["resolution_ceiling"]
        || record.controls["effective_limits"] != manifest["limits"]
    {
        return Err(ContractError::InvalidInvariant);
    }
    if record.availability == "available" {
        let manifest_path = text(&record.controls, "manifest_path")?;
        if manifest_path
            != format!(
                "{NATIVE_LOCAL_EVALUATION_ROOT_V3}/packages/{}/manifest.json",
                record
                    .implementation_bundle
                    .fingerprint()
                    .as_str()
                    .trim_start_matches("sha256:")
            )
        {
            return Err(ContractError::InvalidInvariant);
        }
        let observed = parsed(root, checkpoint, manifest_path)?;
        if &observed != manifest {
            return Err(ContractError::InvalidInvariant);
        }
        let listed = record.controls["package_files"]
            .as_array()
            .ok_or(ContractError::InvalidInvariant)?;
        let members = bundle["files"]
            .as_array()
            .or_else(|| bundle["artifacts"].as_array())
            .ok_or(ContractError::InvalidInvariant)?;
        if listed.len() != members.len() || listed.is_empty() {
            return Err(ContractError::InvalidInvariant);
        }
        for (proof, member) in listed.iter().zip(members) {
            closed(proof, &["path", "length", "sha256"])?;
            let path = text(proof, "path")?;
            let prefix = format!(
                "{NATIVE_LOCAL_EVALUATION_ROOT_V3}/packages/{}/",
                record
                    .implementation_bundle
                    .fingerprint()
                    .as_str()
                    .trim_start_matches("sha256:")
            );
            if path.strip_prefix(&prefix) != Some(text(member, "path")?) {
                return Err(ContractError::InvalidInvariant);
            }
            let bytes = pinned(root, checkpoint, path)?;
            let digest = format!("sha256:{:x}", Sha256::digest(&bytes));
            let expected_digest = member["sha256"]
                .as_str()
                .or_else(|| member["fingerprint"].as_str())
                .ok_or(ContractError::InvalidInvariant)?;
            if proof["length"] != bytes.len()
                || text(proof, "sha256")? != digest
                || member["length"] != bytes.len()
                || expected_digest.trim_start_matches("sha256:")
                    != digest.trim_start_matches("sha256:")
            {
                return Err(ContractError::InvalidInvariant);
            }
        }
    } else if !record.controls["manifest_path"].is_null()
        || record.controls["package_files"] != json!([])
    {
        return Err(ContractError::InvalidInvariant);
    }
    Ok(())
}

/// Retained native origin bytes are independently checked before this token can
/// be formed. It is intentionally neither serializable nor caller-constructible.
pub struct ResolvedLocalEvaluationOriginV3<'a> {
    pub(crate) intent: Value,
    pub(crate) terminal: Value,
    pub(crate) candidates: Vec<EvidenceCandidateV1>,
    pub(crate) native_paths: Vec<String>,
    pub(crate) root: &'a RetainedRepositoryRootV1,
    pub(crate) checkpoint: &'a RepositoryExactFilesCheckpointV1<'a>,
}
impl ResolvedLocalEvaluationOriginV3<'_> {
    pub(crate) fn require_repository_root(
        &self,
        root: &RetainedRepositoryRootV1,
    ) -> Result<(), ContractError> {
        if self
            .root
            .governance_physical_root_fingerprint_v1()
            .map_err(|_| ContractError::InvalidInvariant)?
            != root
                .governance_physical_root_fingerprint_v1()
                .map_err(|_| ContractError::InvalidInvariant)?
        {
            return Err(ContractError::InvalidInvariant);
        }
        self.validate_current()
    }
    pub fn intent(&self) -> &Value {
        &self.intent
    }
    pub fn terminal(&self) -> &Value {
        &self.terminal
    }
    pub fn candidates(&self) -> &[EvidenceCandidateV1] {
        &self.candidates
    }
    pub fn native_paths(&self) -> &[String] {
        &self.native_paths
    }
    pub fn validate_current(&self) -> Result<(), ContractError> {
        self.root
            .governance_physical_root_fingerprint_v1()
            .map_err(|_| ContractError::InvalidInvariant)?;
        self.checkpoint
            .validate()
            .map_err(|_| ContractError::InvalidInvariant)
    }
}

#[allow(clippy::too_many_arguments)]
pub fn resolve_native_local_evaluation_origin_v3<'a, R>(
    root: &'a RetainedRepositoryRootV1,
    checkpoint: &'a RepositoryExactFilesCheckpointV1<'a>,
    attempt: &ContentAddressedBindingV1,
    occurrence: &ContentAddressedBindingV1,
    basis: &ContentAddressedBindingV1,
    selection: &ExactBindingV1,
    authorization: &ContentAddressedBindingV1,
    run: &ContractEvaluationRunV1,
    dependencies: &LocalDefinitionDependenciesV2,
    consume: impl FnOnce(&ResolvedLocalEvaluationOriginV3<'a>) -> R,
) -> Result<R, ContractError> {
    if !run.is_local_evaluation_v3() {
        return Err(ContractError::InvalidInvariant);
    }
    let directory = format!(
        "{NATIVE_LOCAL_EVALUATION_ROOT_V3}/attempts/{}",
        attempt.fingerprint().as_str().trim_start_matches("sha256:")
    );
    let intent = parsed(root, checkpoint, &format!("{directory}/intent.json"))?;
    let terminal = parsed(root, checkpoint, &format!("{directory}/terminal.json"))?;
    closed(
        &intent,
        &[
            "schema_ref",
            "attempt_ref",
            "attempt_fingerprint",
            "occurrence",
            "basis",
            "selection",
            "authorization",
            "capability_observation",
            "ordinal",
            "request",
            "evaluation_run",
            "request_jcs",
            "evaluation_run_jcs",
            "read_paths",
            "write_subtrees",
            "retention_until",
            "lock_generation",
        ],
    )?;
    closed(
        &terminal,
        &[
            "schema_ref",
            "attempt",
            "kind",
            "execution",
            "store_root",
            "candidates",
            "reason",
            "retention_until",
            "terminal_fingerprint",
        ],
    )?;
    let capability_binding = binding(&intent["capability_observation"])?;
    let capability_path = format!(
        "{NATIVE_LOCAL_EVALUATION_ROOT_V3}/capabilities/{}.json",
        capability_binding
            .fingerprint()
            .as_str()
            .trim_start_matches("sha256:")
    );
    let capability: NativeLocalEvaluationCapabilityV3 =
        serde_json::from_value(parsed(root, checkpoint, &capability_path)?)
            .map_err(|_| ContractError::InvalidInvariant)?;
    capability.validate()?;
    if capability.binding()? != capability_binding {
        return Err(ContractError::InvalidInvariant);
    }
    validate_capability_proof(root, checkpoint, &capability, dependencies)?;
    let mut preimage = intent.clone();
    let object = preimage
        .as_object_mut()
        .ok_or(ContractError::InvalidInvariant)?;
    object.remove("attempt_ref");
    object.remove("attempt_fingerprint");
    if intent["schema_ref"] != "handbook.dock.local-evaluation-attempt-intent@1.0.0"
        || text(&intent, "attempt_ref")? != attempt.reference()
        || text(&intent, "attempt_fingerprint")? != attempt.fingerprint().as_str()
        || canonical_fingerprint(&preimage)? != *attempt.fingerprint()
        || binding(&intent["occurrence"])? != *occurrence
        || binding(&intent["basis"])? != *basis
        || intent["selection"]
            != serde_json::to_value(selection).map_err(|_| ContractError::InvalidInvariant)?
        || binding(&intent["authorization"])? != *authorization
        || intent["ordinal"] != 1
        || binding(&intent["evaluation_run"])?
            != ContentAddressedBindingV1::new(
                run.evaluation_run_ref(),
                run.evaluation_run_fingerprint().clone(),
            )?
        || text(&intent, "evaluation_run_jcs")?.as_bytes() != run.canonical_json_bytes()?
    {
        return Err(ContractError::InvalidInvariant);
    }
    pinned(
        root,
        checkpoint,
        ".handbook/state/locks/dock-operational-store.lock",
    )?;
    if text(&intent, "lock_generation")?
        != root
            .existing_lock_physical_fingerprint_v1(
                ".handbook/state/locks/dock-operational-store.lock",
            )
            .map_err(|_| ContractError::InvalidInvariant)?
    {
        return Err(ContractError::InvalidInvariant);
    }
    let mut terminal_preimage = terminal.clone();
    terminal_preimage
        .as_object_mut()
        .ok_or(ContractError::InvalidInvariant)?
        .remove("terminal_fingerprint");
    if terminal["schema_ref"] != "handbook.dock.local-evaluation-attempt-terminal@1.0.0"
        || binding(&terminal["attempt"])? != *attempt
        || text(&terminal, "terminal_fingerprint")?
            != canonical_fingerprint(&terminal_preimage)?.as_str()
        || terminal["retention_until"] != intent["retention_until"]
    {
        return Err(ContractError::InvalidInvariant);
    }
    let kind = text(&terminal, "kind")?;
    if ![
        "completed",
        "timed_out",
        "cancelled",
        "failed",
        "refused",
        "protocol_error",
        "cleanup_uncertain",
        "cancelled_before_launch",
        "refused_before_launch",
    ]
    .contains(&kind)
    {
        return Err(ContractError::InvalidInvariant);
    }
    if kind == "cleanup_uncertain" {
        return Err(ContractError::InvalidInvariant);
    }
    let request: Value = serde_json::from_str(text(&intent, "request_jcs")?)
        .map_err(|_| ContractError::InvalidInvariant)?;
    if canonical_bytes(&request)? != text(&intent, "request_jcs")?.as_bytes() {
        return Err(ContractError::InvalidInvariant);
    }
    validate_native_request_question(&request, &binding(&intent["request"])?, run, dependencies)?;
    let candidate_bindings: Vec<ContentAddressedBindingV1> =
        serde_json::from_value(terminal["candidates"].clone())
            .map_err(|_| ContractError::InvalidInvariant)?;
    if candidate_bindings.len() > 1 || (kind != "completed" && !candidate_bindings.is_empty()) {
        return Err(ContractError::InvalidInvariant);
    }
    let mut candidates = vec![];
    for candidate in &candidate_bindings {
        let path = format!(
            ".handbook/state/dock/v1/candidates/{}.json",
            candidate
                .fingerprint()
                .as_str()
                .trim_start_matches("sha256:")
        );
        let stored: NativeDockCandidateObservationV1 =
            serde_json::from_value(parsed(root, checkpoint, &path)?)
                .map_err(|_| ContractError::InvalidInvariant)?;
        let parsed_candidate: EvidenceCandidateV1 = serde_json::from_str(&stored.candidate_jcs)
            .map_err(|_| ContractError::InvalidInvariant)?;
        if stored.schema_ref != "handbook.dock.internal-candidate-observation@1.0.0"
            || stored.candidate != *candidate
            || parsed_candidate.candidate_ref() != candidate.reference()
            || parsed_candidate.candidate_fingerprint() != candidate.fingerprint()
            || parsed_candidate.admission_body().contract() != &run.contract()
            || stored.evaluation_run_jcs.as_bytes() != run.canonical_json_bytes()?
        {
            return Err(ContractError::InvalidInvariant);
        }
        crate::native_dock_source::require_completed_native_dock_source(
            root,
            checkpoint,
            &stored,
            &parsed_candidate,
            run,
            dependencies,
        )
        .map_err(|_| ContractError::InvalidInvariant)?;
        validate_native_fact(&parsed_candidate, &request, dependencies)?;
        candidates.push(parsed_candidate);
    }
    if kind == "completed" && candidates.len() != 1 {
        return Err(ContractError::InvalidInvariant);
    }
    if !kind.ends_with("before_launch") {
        validate_native_attempt_transaction(
            root, checkpoint, attempt, &intent, &terminal, &request,
        )?;
    }
    if kind.ends_with("before_launch") {
        if !terminal["execution"].is_null() || !terminal["store_root"].is_null() {
            return Err(ContractError::InvalidInvariant);
        }
    } else if kind != "completed" {
        let execution = binding(&terminal["execution"])?;
        let path = format!(
            ".handbook/state/dock/v1/executions/{}.json",
            execution
                .fingerprint()
                .as_str()
                .trim_start_matches("sha256:")
        );
        let record = parsed(root, checkpoint, &path)?;
        let mut preimage = record.clone();
        preimage
            .as_object_mut()
            .ok_or(ContractError::InvalidInvariant)?
            .remove("record_ref");
        preimage
            .as_object_mut()
            .unwrap()
            .remove("record_fingerprint");
        if canonical_fingerprint(&preimage)? != *execution.fingerprint()
            || record["record_ref"] != execution.reference()
            || record["request"] != intent["request"]
            || record["evaluation_run"] != intent["evaluation_run"]
            || record["host_outcome"] != kind
            || record["cleanup"] != "confirmed"
            || record["durability"] != "confirmed"
        {
            return Err(ContractError::InvalidInvariant);
        }
    }
    let native_paths = checkpoint.pinned_file_paths().map(str::to_owned).collect();
    let resolved = ResolvedLocalEvaluationOriginV3 {
        intent,
        terminal,
        candidates,
        native_paths,
        root,
        checkpoint,
    };
    resolved.validate_current()?;
    let result = consume(&resolved);
    resolved.validate_current()?;
    Ok(result)
}

fn native_identity(
    value: &Value,
    expected: &ContentAddressedBindingV1,
    reference: &str,
    fingerprint: &str,
) -> Result<(), ContractError> {
    let mut content = value.clone();
    let object = content
        .as_object_mut()
        .ok_or(ContractError::InvalidInvariant)?;
    object.remove(reference);
    object.remove(fingerprint);
    if value[reference] != expected.reference()
        || value[fingerprint] != expected.fingerprint().as_str()
        || canonical_fingerprint(&content)? != *expected.fingerprint()
    {
        return Err(ContractError::InvalidInvariant);
    }
    Ok(())
}
fn validate_native_fact(
    candidate: &EvidenceCandidateV1,
    request: &Value,
    dependencies: &LocalDefinitionDependenciesV2,
) -> Result<(), ContractError> {
    let value = serde_json::to_value(candidate).map_err(|_| ContractError::InvalidInvariant)?;
    let fact = &value["body"]["fact_payload"];
    let content = &request["inputs"][0]["content"];
    closed(
        fact,
        &[
            "root_schema",
            "root_document",
            "root_document_fingerprint",
            "closure_fingerprint",
            "instance_fingerprint",
            "closure_document_count",
            "conformance",
            "violations",
        ],
    )?;
    let violations = fact["violations"]
        .as_array()
        .ok_or(ContractError::InvalidInvariant)?;
    if fact["root_schema"] != content["root_schema"]
        || fact["root_document"] != content["root_document"]
        || fact["root_document_fingerprint"] != content["root_document_fingerprint"]
        || fact["closure_fingerprint"] != content["closure_fingerprint"]
        || fact["instance_fingerprint"] != content["instance"]["instance_fingerprint"]
        || fact["instance_fingerprint"]
            != canonical_fingerprint(&content["instance"]["value"])?.as_str()
        || fact["closure_document_count"] != dependencies.schema_closure.documents.len()
        || violations.len() > 128
        || !matches!(
            (
                candidate.admission_body().observation(),
                text(fact, "conformance")?,
                violations.is_empty()
            ),
            (EvidenceObservationV1::Satisfied, "conformant", true)
                | (EvidenceObservationV1::Violated, "nonconformant", false)
        )
    {
        return Err(ContractError::InvalidInvariant);
    }
    for violation in violations {
        closed(
            violation,
            &[
                "code",
                "instance_pointer",
                "schema_document",
                "schema_pointer",
            ],
        )?;
        if ![
            "additional_items",
            "additional_properties",
            "any_of",
            "const",
            "contains",
            "enum",
            "exclusive_maximum",
            "exclusive_minimum",
            "false_schema",
            "format",
            "max_items",
            "maximum",
            "max_length",
            "max_properties",
            "min_items",
            "minimum",
            "min_length",
            "min_properties",
            "multiple_of",
            "not",
            "one_of_multiple_valid",
            "one_of_not_valid",
            "pattern",
            "property_names",
            "required",
            "type",
            "unevaluated_items",
            "unevaluated_properties",
            "unique_items",
        ]
        .contains(&text(violation, "code")?)
            || !dependencies
                .schema_closure
                .documents
                .iter()
                .any(|d| Some(d.path.as_str()) == violation["schema_document"].as_str())
        {
            return Err(ContractError::InvalidInvariant);
        }
        for key in ["instance_pointer", "schema_pointer"] {
            let pointer = text(violation, key)?;
            if pointer.len() > 1024 || (!pointer.is_empty() && !pointer.starts_with('/')) {
                return Err(ContractError::InvalidInvariant);
            }
            let mut bytes = pointer.bytes();
            while let Some(byte) = bytes.next() {
                if byte == b'~' && !matches!(bytes.next(), Some(b'0' | b'1')) {
                    return Err(ContractError::InvalidInvariant);
                }
            }
        }
    }
    Ok(())
}
fn validate_native_request_question(
    request: &Value,
    expected: &ContentAddressedBindingV1,
    run: &ContractEvaluationRunV1,
    dependencies: &LocalDefinitionDependenciesV2,
) -> Result<(), ContractError> {
    closed(
        request,
        &[
            "request_schema",
            "result_schema",
            "protocol",
            "request_ref",
            "request_fingerprint",
            "manifest",
            "implementation_bundle",
            "run",
            "resolution_ceiling",
            "requested_evidence_kinds",
            "inputs",
            "effective_limits",
            "network",
        ],
    )?;
    closed(
        &request["run"],
        &[
            "evaluation_run",
            "evaluation_lifecycle_basis",
            "contract",
            "claim_ids",
            "subject",
            "cases",
            "requested_resolution",
            "semantic_time_utc",
            "lifecycle_head_fingerprint",
        ],
    )?;
    native_identity(request, expected, "request_ref", "request_fingerprint")?;
    let manifest = &dependencies.validator.canonical_manifest;
    let projected = serde_json::to_value(run).map_err(|_| ContractError::InvalidInvariant)?;
    if request["request_schema"] != manifest["request_schema"]
        || request["result_schema"] != manifest["result_schema"]
        || request["protocol"] != manifest["protocol"]
        || request["manifest"]
            != serde_json::to_value(&dependencies.validator.manifest)
                .map_err(|_| ContractError::InvalidInvariant)?
        || request["implementation_bundle"]
            != json!({"ref":manifest["implementation_bundle"]["bundle_ref"],"fingerprint":manifest["implementation_bundle"]["bundle_fingerprint"]})
        || request["resolution_ceiling"] != manifest["resolution_ceiling"]
        || request["network"] != "denied"
        || request["run"]["evaluation_run"]
            != serde_json::to_value(ContentAddressedBindingV1::new(
                run.evaluation_run_ref(),
                run.evaluation_run_fingerprint().clone(),
            )?)
            .map_err(|_| ContractError::InvalidInvariant)?
        || request["run"]["evaluation_lifecycle_basis"]
            != serde_json::to_value(run.evaluation_lifecycle_basis())
                .map_err(|_| ContractError::InvalidInvariant)?
        || request["run"]["contract"]
            != serde_json::to_value(run.contract()).map_err(|_| ContractError::InvalidInvariant)?
        || request["run"]["subject"] != projected["subject"]
        || request["run"]["cases"] != projected["cases"]
        || request["run"]["requested_resolution"] != projected["requested_resolution"]
        || request["run"]["claim_ids"]
            != json!(run
                .claims()
                .iter()
                .map(|c| c.claim_id())
                .collect::<Vec<_>>())
    {
        return Err(ContractError::InvalidInvariant);
    }
    let inputs = request["inputs"]
        .as_array()
        .ok_or(ContractError::InvalidInvariant)?;
    if inputs.len() != 1 || run.cases().len() != 1 {
        return Err(ContractError::InvalidInvariant);
    }
    let input = &inputs[0];
    let role = text(input, "role")?;
    let media = text(input, "media_type")?;
    let schema: ExactBindingV1 = serde_json::from_value(input["schema"].clone())
        .map_err(|_| ContractError::InvalidInvariant)?;
    closed(
        input,
        &[
            "role",
            "media_type",
            "schema",
            "content",
            "content_fingerprint",
        ],
    )?;
    if role != "handbook.input.json-schema-validation"
        || manifest["supported_inputs"] != json!([{"role":role,"media_type":media,"schema":schema}])
        || input["content_fingerprint"] != canonical_fingerprint(&input["content"])?.as_str()
        || json!({"root_schema":input["content"].get("root_schema"),"root_document":input["content"].get("root_document"),"documents":input["content"].get("documents"),"closure_fingerprint":input["content"].get("closure_fingerprint")})
            != serde_json::to_value(&dependencies.schema_closure)
                .map_err(|_| ContractError::InvalidInvariant)?
    {
        return Err(ContractError::InvalidInvariant);
    }
    let captured = canonical_fingerprint(input)?;
    let captured = ContentAddressedBindingV1::new(
        &format!(
            "handbook.dock.captured-input@1.0.0#role/{role}/{}",
            captured.as_str().trim_start_matches("sha256:")
        ),
        captured,
    )?;
    let matcher = [
        ("dock.input.role", role),
        ("dock.input.media-type", media),
        ("dock.input.schema", schema.reference().as_str()),
    ]
    .into_iter()
    .map(|(field, value)| MatcherInputValueV1::new(StableIdV1::parse(field)?, value))
    .collect::<Result<Vec<_>, _>>()?;
    if *run.subject() != EvaluationSubjectV1::new(captured.clone(), matcher.clone())?
        || run.cases()
            != [EvaluationCaseV1::new(
                run.cases()[0].case_id().clone(),
                captured,
                matcher,
            )?]
    {
        return Err(ContractError::InvalidInvariant);
    }
    Ok(())
}
fn validate_native_attempt_transaction(
    root: &RetainedRepositoryRootV1,
    checkpoint: &RepositoryExactFilesCheckpointV1<'_>,
    attempt: &ContentAddressedBindingV1,
    intent: &Value,
    terminal: &Value,
    request: &Value,
) -> Result<(), ContractError> {
    let mut digest = Sha256::new();
    digest.update(b"handbook.dock-run-idempotency-key@1.0.0\0");
    digest.update(format!(
        "local-evaluation-{}",
        attempt.fingerprint().as_str().trim_start_matches("sha256:")
    ));
    let key = FingerprintV1::parse(&format!("sha256:{:x}", digest.finalize()))?;
    let prefix = ".handbook/state/dock/v1";
    let replay_path = format!(
        "{prefix}/replays/{}.json",
        key.as_str().trim_start_matches("sha256:")
    );
    let replay = parsed(root, checkpoint, &replay_path)?;
    let mut preimage = replay.clone();
    preimage
        .as_object_mut()
        .ok_or(ContractError::InvalidInvariant)?
        .remove("root");
    if replay["schema_ref"] != "handbook.dock.internal-run-replay@1.0.0"
        || replay["root"] != canonical_fingerprint(&preimage)?.as_str()
        || replay["key_fingerprint"] != key.as_str()
        || replay["request"] != intent["request"]
        || replay["execution"] != terminal["execution"]
        || replay["candidates"] != terminal["candidates"]
        || replay["evaluation_run_jcs"] != intent["evaluation_run_jcs"]
        || replay["root"] != terminal["store_root"]
    {
        return Err(ContractError::InvalidInvariant);
    }
    let transaction = format!(
        "{prefix}/transactions/{}",
        key.as_str().trim_start_matches("sha256:")
    );
    let native_intent = parsed(root, checkpoint, &format!("{transaction}/intent.json"))?;
    let commit = parsed(root, checkpoint, &format!("{transaction}/committed.json"))?;
    let mut projection = native_intent.clone();
    projection
        .as_object_mut()
        .ok_or(ContractError::InvalidInvariant)?
        .remove("outputs");
    projection["schema_ref"] = json!("handbook.dock.internal-run-replay@1.0.0");
    if native_intent["schema_ref"] != "handbook.dock.internal-store-intent@1.0.0"
        || projection != replay
        || commit
            != json!({"schema_ref":"handbook.dock.internal-store-commit@1.0.0","root":replay["root"]})
    {
        return Err(ContractError::InvalidInvariant);
    }
    let request_binding = binding(&intent["request"])?;
    let execution_binding = binding(&terminal["execution"])?;
    let request_path = format!(
        "{prefix}/requests/{}.json",
        request_binding
            .fingerprint()
            .as_str()
            .trim_start_matches("sha256:")
    );
    if parsed(root, checkpoint, &request_path)? != *request {
        return Err(ContractError::InvalidInvariant);
    }
    let execution_path = format!(
        "{prefix}/executions/{}.json",
        execution_binding
            .fingerprint()
            .as_str()
            .trim_start_matches("sha256:")
    );
    let execution = parsed(root, checkpoint, &execution_path)?;
    native_identity(
        &execution,
        &execution_binding,
        "record_ref",
        "record_fingerprint",
    )?;
    if execution["request"] != intent["request"]
        || execution["result"] != replay["result"]
        || execution["manifest"] != request["manifest"]
        || execution["implementation_bundle"] != request["implementation_bundle"]
        || execution["evaluation_run"] != intent["evaluation_run"]
        || execution["host_outcome"] != terminal["kind"]
        || execution["cleanup"] != "confirmed"
        || execution["durability"] != "confirmed"
    {
        return Err(ContractError::InvalidInvariant);
    }
    let mut files = std::collections::BTreeMap::from([
        (request_path, canonical_bytes(request)?),
        (execution_path, canonical_bytes(&execution)?),
        (replay_path, canonical_bytes(&replay)?),
    ]);
    if !replay["result"].is_null() {
        let result_binding = binding(&replay["result"])?;
        let path = format!(
            "{prefix}/results/{}.json",
            result_binding
                .fingerprint()
                .as_str()
                .trim_start_matches("sha256:")
        );
        let result = parsed(root, checkpoint, &path)?;
        native_identity(&result, &result_binding, "result_ref", "result_fingerprint")?;
        if result["request"] != intent["request"]
            || result["manifest"] != request["manifest"]
            || result["implementation_bundle"] != request["implementation_bundle"]
            || result["evaluation_run"] != intent["evaluation_run"]
        {
            return Err(ContractError::InvalidInvariant);
        }
        files.insert(path, canonical_bytes(&result)?);
    }
    for candidate in terminal["candidates"]
        .as_array()
        .ok_or(ContractError::InvalidInvariant)?
    {
        let candidate = binding(candidate)?;
        let path = format!(
            "{prefix}/candidates/{}.json",
            candidate
                .fingerprint()
                .as_str()
                .trim_start_matches("sha256:")
        );
        files.insert(path.clone(), pinned(root, checkpoint, &path)?);
    }
    let outputs = native_intent["outputs"]
        .as_array()
        .ok_or(ContractError::InvalidInvariant)?;
    if outputs.len() != files.len() {
        return Err(ContractError::InvalidInvariant);
    }
    let mut observed = std::collections::BTreeSet::new();
    for (index, output) in outputs.iter().enumerate() {
        let target = text(output, "target")?;
        let bytes = files.get(target).ok_or(ContractError::InvalidInvariant)?;
        if !observed.insert(target)
            || *output
                != json!({"stage":format!("{transaction}/{index:04}.stage"),"target":target,"byte_length":bytes.len(),"content_fingerprint":FingerprintV1::from_bytes(bytes)})
        {
            return Err(ContractError::InvalidInvariant);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    fn dependencies() -> LocalDefinitionDependenciesV2 {
        let descriptor: Value = serde_json::from_slice(include_bytes!(
            "../../dock/production-manifests/x86_64-pc-windows-msvc/manifest.json"
        ))
        .unwrap();
        let validator =
            LocalValidatorDependencyV2::from_manifest_json(&canonical_bytes(&descriptor).unwrap())
                .unwrap();
        let document = LocalSchemaDocumentV2::new(
            "definitions/schemas/native-proof.json".into(),
            json!({"$schema":"https://json-schema.org/draft/2020-12/schema","type":"object"}),
        )
        .unwrap();
        let schema = ExactBindingV1::new(
            ExactRefV1::parse("handbook.schema.native-proof@1.0.0").unwrap(),
            document.document_fingerprint.clone(),
        );
        LocalDefinitionDependenciesV2 {
            definition_schema: LocalExactContentV2::new(
                ExactRefV1::parse("handbook.contract-definition@1.0.0").unwrap(),
                json!({}),
            )
            .unwrap(),
            extensions_policy: local_extensions_policy_binding_v2().unwrap(),
            freshness_policies: vec![],
            gate_policy: GatePolicyV1::new(
                GatePolicyInputV1::new(
                    ContractIdV1::parse("handbook.gate.native-proof").unwrap(),
                    VersionV1::parse("1.0.0").unwrap(),
                    GatePolicyAlgorithmV1::ClosedAccounting,
                    None,
                    local_closeout_policy_binding_v1().unwrap(),
                )
                .unwrap(),
            )
            .unwrap(),
            stability_policy: evidence_stability_policy_binding_v1().unwrap(),
            evidence_admission_policy: local_evidence_admission_policy_binding_v2().unwrap(),
            lifecycle_policy: local_lifecycle_policy_binding_v2().unwrap(),
            fact_schema: LocalExactContentV2::new(
                ExactRefV1::parse("handbook.fact.schema-conformance@1.0.0").unwrap(),
                json!({}),
            )
            .unwrap(),
            validator,
            schema_closure: LocalSchemaClosureV2::new(
                schema,
                document.path.clone(),
                vec![document],
            )
            .unwrap(),
            required_governance_scopes: vec![],
        }
    }
    #[test]
    fn caller_descriptor_and_available_flag_cannot_mint_native_capability() {
        let directory = tempfile::tempdir().unwrap();
        std::fs::write(directory.path().join("probe.json"), b"{}").unwrap();
        let root = RetainedRepositoryRootV1::open_exact(directory.path()).unwrap();
        let checkpoint = root
            .retain_exact_files_checkpoint_v1(&["probe.json"])
            .unwrap();
        let mut deps = dependencies();
        let bundle = &deps.validator.canonical_manifest["implementation_bundle"];
        let record=NativeLocalEvaluationCapabilityV3::new(deps.validator.manifest.clone(),ContentAddressedBindingV1::new(bundle["bundle_ref"].as_str().unwrap(),FingerprintV1::parse(bundle["bundle_fingerprint"].as_str().unwrap()).unwrap()).unwrap(),"x86_64-pc-windows-msvc".into(),"available".into(),json!({"profile":"first_party_native_advisory","availability_reason":"caller_says_native_ready"}),SemanticTimeV1::parse("2026-10-06T00:00:00Z").unwrap(),SemanticTimeV1::parse("2026-11-06T00:00:00Z").unwrap()).unwrap();
        let granted = std::cell::Cell::new(false);
        assert!(resolve_native_local_evaluation_capability_v3(
            &root,
            &checkpoint,
            record.clone(),
            &deps,
            |_| granted.set(true)
        )
        .is_err());
        assert!(!granted.get());
        deps.validator.canonical_manifest["implementation_bundle"]["entrypoint"] =
            json!("caller-validator.exe");
        assert!(resolve_native_local_evaluation_capability_v3(
            &root,
            &checkpoint,
            record,
            &deps,
            |_| granted.set(true)
        )
        .is_err());
        assert!(!granted.get());
    }
    #[test]
    fn native_proof_reader_uses_supported_retained_repository_limit() {
        let directory = tempfile::tempdir().unwrap();
        let bytes = vec![0x5a; 9 * 1024 * 1024];
        std::fs::write(directory.path().join("native-package-member.bin"), &bytes).unwrap();
        let root = RetainedRepositoryRootV1::open_exact(directory.path()).unwrap();
        let checkpoint = root
            .retain_exact_files_checkpoint_v1(&["native-package-member.bin"])
            .unwrap();
        // The frozen native artifact exceeds the record bound. The retained
        // filesystem separately refuses an unsupported requested read limit,
        // even when the exact file is smaller than that limit.
        assert!(root
            .read_regular_bounded("native-package-member.bin", 64 * 1024 * 1024)
            .is_err());
        assert_eq!(
            pinned(&root, &checkpoint, "native-package-member.bin").unwrap(),
            bytes
        );
        assert!(pinned(&root, &checkpoint, "undeclared.bin").is_err());
    }
}
