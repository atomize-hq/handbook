//! Shared native Dock capture DTO and read-only proof of retained source bytes.
use crate::canonical::{canonical_bytes, canonical_fingerprint};
use crate::evidence_candidate_source::{
    EvidenceCandidateSourceErrorKindV1, EvidenceCandidateSourceErrorV1,
};
use crate::{
    ContentAddressedBindingV1, ContractEvaluationRunV1, EvaluationCaseV1, EvaluationSubjectV1,
    EvidenceCandidateV1, FingerprintV1, LocalDefinitionDependenciesV2, MatcherInputValueV1,
    ResolutionEnvelopeV1, StableIdV1,
};
use handbook_engine::{
    RepositoryExactFilesCheckpointV1, RepositoryReadV1, RetainedRepositoryRootV1,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::BTreeMap;

/// Data only. Source authority requires the corresponding actual retained file.
/// Dock uses this unchanged V1 DTO for its established writer and reader.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NativeDockCandidateObservationV1 {
    pub schema_ref: String,
    pub candidate: ContentAddressedBindingV1,
    pub candidate_jcs: String,
    pub evaluation_run_jcs: String,
    pub producer: ContentAddressedBindingV1,
    pub request_record: ContentAddressedBindingV1,
    pub execution_record: ContentAddressedBindingV1,
    pub producer_resolution_ceiling: Option<ResolutionEnvelopeV1>,
}

const ROOT: &str = ".handbook/state/dock/v1";
const MAX: usize = 8 * 1024 * 1024;
fn refused() -> EvidenceCandidateSourceErrorV1 {
    EvidenceCandidateSourceErrorV1::repository_refusal(
        EvidenceCandidateSourceErrorKindV1::CandidateSubstituted,
    )
}
fn bytes(value: &impl Serialize) -> Result<Vec<u8>, EvidenceCandidateSourceErrorV1> {
    canonical_bytes(value).map_err(|_| refused())
}
fn fp(value: &impl Serialize) -> Result<FingerprintV1, EvidenceCandidateSourceErrorV1> {
    canonical_fingerprint(value).map_err(|_| refused())
}
fn path(kind: &str, fingerprint: &FingerprintV1) -> String {
    format!(
        "{ROOT}/{kind}/{}.json",
        fingerprint.as_str().strip_prefix("sha256:").unwrap()
    )
}
pub(crate) fn require_pinned_file(
    root: &RetainedRepositoryRootV1,
    checkpoint: &RepositoryExactFilesCheckpointV1<'_>,
    relative: &str,
) -> Result<Vec<u8>, EvidenceCandidateSourceErrorV1> {
    checkpoint
        .require_file_membership(root, &[relative])
        .map_err(|_| refused())?;
    let RepositoryReadV1::Found(value) = root
        .read_regular_bounded(relative, MAX)
        .map_err(|_| refused())?
    else {
        return Err(refused());
    };
    checkpoint
        .require_exact_file(root, relative, &value)
        .map_err(|_| refused())?;
    Ok(value)
}
fn read(
    root: &RetainedRepositoryRootV1,
    checkpoint: &RepositoryExactFilesCheckpointV1<'_>,
    relative: &str,
) -> Result<Value, EvidenceCandidateSourceErrorV1> {
    let raw = require_pinned_file(root, checkpoint, relative)?;
    let value: Value = serde_json::from_slice(&raw).map_err(|_| refused())?;
    if bytes(&value)? != raw || !value.is_object() {
        return Err(refused());
    }
    Ok(value)
}
fn identity(
    value: &Value,
    binding: &ContentAddressedBindingV1,
    reference: &str,
    digest: &str,
    keys: &[&str],
) -> Result<(), EvidenceCandidateSourceErrorV1> {
    let object = value.as_object().ok_or_else(refused)?;
    if object.len() != keys.len()
        || object.keys().any(|key| !keys.contains(&key.as_str()))
        || value[reference] != binding.reference()
        || value[digest] != binding.fingerprint().as_str()
    {
        return Err(refused());
    }
    let mut preimage = value.clone();
    preimage.as_object_mut().unwrap().remove(reference);
    preimage.as_object_mut().unwrap().remove(digest);
    if &fp(&preimage)? != binding.fingerprint() {
        return Err(refused());
    }
    Ok(())
}

/// Mandatory native captured source proof. No caller flag or trait can replace
/// the fp-derived candidate file and the completed linked native transaction.
pub(crate) fn require_completed_native_dock_source(
    root: &RetainedRepositoryRootV1,
    checkpoint: &RepositoryExactFilesCheckpointV1<'_>,
    stored: &NativeDockCandidateObservationV1,
    candidate: &EvidenceCandidateV1,
    evaluation_run: &ContractEvaluationRunV1,
    dependencies: &LocalDefinitionDependenciesV2,
) -> Result<(), EvidenceCandidateSourceErrorV1> {
    checkpoint
        .require_exact_file(
            root,
            &path("candidates", stored.candidate.fingerprint()),
            &bytes(stored)?,
        )
        .map_err(|_| refused())?;
    checkpoint
        .require_exact_file(
            root,
            ".handbook/state/locks/dock-operational-store.lock",
            &[],
        )
        .map_err(|_| refused())?;
    let request_path = path("requests", stored.request_record.fingerprint());
    let execution_path = path("executions", stored.execution_record.fingerprint());
    let request = read(root, checkpoint, &request_path)?;
    let execution = read(root, checkpoint, &execution_path)?;
    identity(
        &request,
        &stored.request_record,
        "request_ref",
        "request_fingerprint",
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
    identity(
        &execution,
        &stored.execution_record,
        "record_ref",
        "record_fingerprint",
        &[
            "schema_ref",
            "record_ref",
            "record_fingerprint",
            "request",
            "result",
            "manifest",
            "implementation_bundle",
            "evaluation_run",
            "host_outcome",
            "exit_code",
            "wall_time_ms",
            "stdout_bytes",
            "stdout_truncated",
            "stderr_bytes",
            "stderr_truncated",
            "cleanup",
            "durability",
            "effective_limits",
            "diagnostics",
            "candidate_observation_bundle_fingerprint",
            "candidate_observation_count",
        ],
    )?;
    let body = serde_json::to_value(candidate.admission_body()).map_err(|_| refused())?;
    let run: Value = serde_json::from_str(&stored.evaluation_run_jcs).map_err(|_| refused())?;
    let result_binding: ContentAddressedBindingV1 =
        serde_json::from_value(execution["result"].clone()).map_err(|_| refused())?;
    let result_path = path("results", result_binding.fingerprint());
    let result = read(root, checkpoint, &result_path)?;
    identity(
        &result,
        &result_binding,
        "result_ref",
        "result_fingerprint",
        &[
            "result_schema",
            "protocol",
            "result_ref",
            "result_fingerprint",
            "request",
            "manifest",
            "implementation_bundle",
            "evaluation_run",
            "status",
            "claim_partition",
            "candidates",
            "diagnostics",
            "actual_resolution",
            "refusal_reason",
        ],
    )?;
    let manifest = &dependencies.validator.canonical_manifest;
    let bundle = &manifest["implementation_bundle"];
    if execution["schema_ref"] != "handbook.dock-execution-record@1.0.0"
        || request["request_schema"]["ref"] != "handbook.dock-process-request@1.0.0"
        || result["result_schema"]["ref"] != "handbook.dock-process-result@1.0.0"
        || request["request_schema"] != manifest["request_schema"]
        || request["result_schema"] != manifest["result_schema"]
        || result["result_schema"] != manifest["result_schema"]
        || request["protocol"] != manifest["protocol"]
        || result["protocol"] != manifest["protocol"]
        || request["implementation_bundle"]
            != json!({"ref":bundle["bundle_ref"],"fingerprint":bundle["bundle_fingerprint"]})
        || execution["host_outcome"] != "completed"
        || execution["cleanup"] != "confirmed"
        || execution["durability"] != "confirmed"
        || execution["exit_code"] != 0
        || execution["stdout_truncated"] != false
        || execution["stderr_truncated"] != false
        || result["status"] != "completed"
        || request["network"] != "denied"
        || result["request"]
            != serde_json::to_value(&stored.request_record).map_err(|_| refused())?
        || execution["request"] != result["request"]
        || execution["manifest"] != request["manifest"]
        || result["manifest"] != request["manifest"]
        || execution["implementation_bundle"] != request["implementation_bundle"]
        || result["implementation_bundle"] != request["implementation_bundle"]
        || request["manifest"] != body["producer"]
        || execution["evaluation_run"] != body["evaluation_run"]
        || result["evaluation_run"] != body["evaluation_run"]
        || request["run"]["evaluation_run"] != body["evaluation_run"]
        || request["run"]["evaluation_lifecycle_basis"] != body["evaluation_lifecycle_basis"]
        || request["run"]["contract"] != body["contract"]
        || request["run"]["subject"] != body["subject"]
        || request["run"]["subject"] != run["subject"]
        || request["run"]["cases"] != run["cases"]
        || request["run"]["requested_resolution"] != body["request_resolution"]
        || result["actual_resolution"] != body["actual_resolution"]
        || serde_json::to_value(&stored.producer_resolution_ceiling).map_err(|_| refused())?
            != request["resolution_ceiling"]
        || request["run"]["semantic_time_utc"] != body["observed_at"]
    {
        return Err(refused());
    }
    require_captured_question(&request, candidate, evaluation_run, dependencies)?;
    let observations = result["candidates"].as_array().ok_or_else(refused)?;
    if observations.len() != 1
        || execution["candidate_observation_count"] != 1
        || execution["candidate_observation_bundle_fingerprint"] != fp(observations)?.as_str()
    {
        return Err(refused());
    }
    let observation = &observations[0];
    if observation["observation"] != body["observation"]
        || observation["payload"] != body["fact_payload"]
        || observation["payload_schema"] != body["fact_schema"]
        || observation["evidence_kind"] != body["evidence_kind"]
        || observation["case_id"] != body["case"]["case_id"]
        || body["observed_claim_ids"] != json!([observation["claim_id"]])
        || observation["payload_fingerprint"] != fp(&observation["payload"])?.as_str()
        || result["claim_partition"]["observed"] != body["claim_partition"]["observed"]
        || result["claim_partition"]["unobserved"] != body["claim_partition"]["unobserved"]
    {
        return Err(refused());
    }
    let excluded = result["claim_partition"]["typed_excluded"]
        .as_array()
        .ok_or_else(refused)?
        .iter()
        .map(|claim| json!({"claim_id":claim,"kind":"unsupported_kind"}))
        .collect::<Vec<_>>();
    if json!(excluded) != body["claim_partition"]["excluded"] {
        return Err(refused());
    }
    let sources = request["inputs"]
        .as_array()
        .ok_or_else(refused)?
        .iter()
        .map(|input| {
            if fp(&input["content"])?.as_str()
                != input["content_fingerprint"].as_str().ok_or_else(refused)?
            {
                return Err(refused());
            }
            Ok(json!({
                "source": {
                    "ref": format!(
                        "{}#input/{}",
                        stored.request_record.reference(),
                        input["role"].as_str().ok_or_else(refused)?
                    ),
                    "fingerprint": input["content_fingerprint"],
                },
                "revision": input["content_fingerprint"],
                "collected_at": request["run"]["semantic_time_utc"],
            }))
        })
        .collect::<Result<Vec<_>, EvidenceCandidateSourceErrorV1>>()?;
    if json!(sources) != body["sources"] {
        return Err(refused());
    }
    let mut selected = None;
    for relative in checkpoint.pinned_file_paths() {
        let Some(hex) = relative
            .strip_prefix(&format!("{ROOT}/replays/"))
            .and_then(|name| name.strip_suffix(".json"))
        else {
            continue;
        };
        if hex.len() != 64
            || !hex
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
        {
            continue;
        }
        let value = read(root, checkpoint, relative)?;
        if value["candidates"] == json!([stored.candidate]) {
            if selected.is_some() {
                return Err(refused());
            }
            selected = Some((relative.to_owned(), value));
        }
    }
    let (replay_path, replay) = selected.ok_or_else(refused)?;
    checkpoint
        .require_exact_file(root, &replay_path, &bytes(&replay)?)
        .map_err(|_| refused())?;
    let mut preimage = replay.clone();
    preimage.as_object_mut().ok_or_else(refused)?.remove("root");
    if replay["schema_ref"] != "handbook.dock.internal-run-replay@1.0.0"
        || replay["root"] != fp(&preimage)?.as_str()
        || replay["request"] != result["request"]
        || replay["result"] != execution["result"]
        || replay["execution"]
            != serde_json::to_value(&stored.execution_record).map_err(|_| refused())?
        || replay["evaluation_run_jcs"] != stored.evaluation_run_jcs
    {
        return Err(refused());
    }
    let key: FingerprintV1 =
        serde_json::from_value(replay["key_fingerprint"].clone()).map_err(|_| refused())?;
    if replay_path != path("replays", &key) {
        return Err(refused());
    }
    let transaction = format!(
        "{ROOT}/transactions/{}",
        key.as_str().strip_prefix("sha256:").unwrap()
    );
    let intent = read(root, checkpoint, &format!("{transaction}/intent.json"))?;
    let marker = read(root, checkpoint, &format!("{transaction}/committed.json"))?;
    let mut intent_replay = intent.clone();
    intent_replay
        .as_object_mut()
        .ok_or_else(refused)?
        .remove("outputs");
    intent_replay["schema_ref"] = json!("handbook.dock.internal-run-replay@1.0.0");
    if intent["schema_ref"] != "handbook.dock.internal-store-intent@1.0.0"
        || intent_replay != replay
        || marker
            != json!({"schema_ref":"handbook.dock.internal-store-commit@1.0.0","root":replay["root"]})
    {
        return Err(refused());
    }
    let records = BTreeMap::from([
        (
            path("candidates", stored.candidate.fingerprint()),
            bytes(stored)?,
        ),
        (request_path, bytes(&request)?),
        (execution_path, bytes(&execution)?),
        (result_path, bytes(&result)?),
        (replay_path, bytes(&replay)?),
    ]);
    let outputs = intent["outputs"].as_array().ok_or_else(refused)?;
    if outputs.len() != records.len() {
        return Err(refused());
    }
    let mut observed = std::collections::BTreeSet::new();
    for (index, output) in outputs.iter().enumerate() {
        let target = output["target"].as_str().ok_or_else(refused)?;
        let raw = records.get(target).ok_or_else(refused)?;
        if !observed.insert(target)
            || output
                != &json!({"stage":format!("{transaction}/{index:04}.stage"),"target":target,
            "byte_length":raw.len(),"content_fingerprint":FingerprintV1::from_bytes(raw)})
        {
            return Err(refused());
        }
    }
    checkpoint.validate().map_err(|_| refused())
}

// Derive the same complete inline-input question as Dock's public capture
// helper from the actual retained request. A caller-selected run or definition
// cannot replace the captured role, exact schema, media type, or closure.
fn require_captured_question(
    request: &Value,
    candidate: &EvidenceCandidateV1,
    run: &ContractEvaluationRunV1,
    dependencies: &LocalDefinitionDependenciesV2,
) -> Result<(), EvidenceCandidateSourceErrorV1> {
    let inputs = request["inputs"].as_array().ok_or_else(refused)?;
    if inputs.len() != 1 || run.cases().len() != 1 {
        return Err(refused());
    }
    let input = &inputs[0];
    let fields = input.as_object().ok_or_else(refused)?;
    let role = input["role"].as_str().ok_or_else(refused)?;
    let media_type = input["media_type"].as_str().ok_or_else(refused)?;
    let schema: crate::ExactBindingV1 =
        serde_json::from_value(input["schema"].clone()).map_err(|_| refused())?;
    let supported_inputs = dependencies.validator.canonical_manifest["supported_inputs"]
        .as_array()
        .ok_or_else(refused)?;
    let expected_input = json!({"role":role,"media_type":media_type,"schema":schema});
    if fields.len() != 5
        || fields.keys().any(|key| {
            ![
                "role",
                "media_type",
                "schema",
                "content",
                "content_fingerprint",
            ]
            .contains(&key.as_str())
        })
        || role != "handbook.input.json-schema-validation"
        || supported_inputs.as_slice() != std::slice::from_ref(&expected_input)
        || fp(&input["content"])?.as_str() != input["content_fingerprint"]
        || serde_json::to_value(candidate.admission_body()).map_err(|_| refused())?["fact_schema"]
            != serde_json::to_value(&dependencies.fact_schema.binding).map_err(|_| refused())?
    {
        return Err(refused());
    }
    let content = &input["content"];
    let closure = json!({
        "root_schema":content.get("root_schema"),
        "root_document":content.get("root_document"),
        "documents":content.get("documents"),
        "closure_fingerprint":content.get("closure_fingerprint"),
    });
    if closure != serde_json::to_value(&dependencies.schema_closure).map_err(|_| refused())? {
        return Err(refused());
    }
    let captured_fingerprint = fp(input)?;
    let captured = ContentAddressedBindingV1::new(
        &format!(
            "handbook.dock.captured-input@1.0.0#role/{role}/{}",
            captured_fingerprint
                .as_str()
                .strip_prefix("sha256:")
                .unwrap()
        ),
        captured_fingerprint,
    )
    .map_err(|_| refused())?;
    let matcher_inputs = [
        ("dock.input.role", role),
        ("dock.input.media-type", media_type),
        ("dock.input.schema", schema.reference().as_str()),
    ]
    .into_iter()
    .map(|(field, value)| {
        MatcherInputValueV1::new(StableIdV1::parse(field).map_err(|_| refused())?, value)
            .map_err(|_| refused())
    })
    .collect::<Result<Vec<_>, _>>()?;
    let subject = EvaluationSubjectV1::new(captured.clone(), matcher_inputs.clone())
        .map_err(|_| refused())?;
    let case = EvaluationCaseV1::new(run.cases()[0].case_id().clone(), captured, matcher_inputs)
        .map_err(|_| refused())?;
    if run.subject() != &subject
        || run.cases() != std::slice::from_ref(&case)
        || candidate.admission_body().subject() != &subject
        || candidate.admission_body().case() != &case
    {
        return Err(refused());
    }
    Ok(())
}
