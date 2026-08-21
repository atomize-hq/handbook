use handbook_dock_json_schema::{
    evaluate_json_schema_v1, ConformanceV1, JsonSchemaValidationInputV1, ValidatorRefusalV1,
    SCHEMA_CONFORMANCE_FACT_SCHEMA_REF_V1, VALIDATION_INPUT_SCHEMA_REF_V1,
};
use serde::de::{MapAccess, SeqAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::fmt;
use std::io::{Read, Write};

const REQUEST_BYTES: usize = 2 * 1024 * 1024;
const RESULT_BYTES: usize = 512 * 1024;
const DOCK_REF: &str = "handbook.dock.json-schema@1.0.0";
const BUNDLE_REF: &str = "handbook.dock.bundle.json-schema-native@1.0.0";
const PROTOCOL_REF: &str = "handbook.dock.process-json@1.0.0";
const REQUEST_SCHEMA_REF: &str = "handbook.dock-process-request@1.0.0";
const REQUEST_SCHEMA_FP: &str =
    "sha256:794aec1a3b700e53d8e168f2f7dde54113effa297e023257d44fa89e90decdf8";
const RESULT_SCHEMA_REF: &str = "handbook.dock-process-result@1.0.0";
const RESULT_SCHEMA_FP: &str =
    "sha256:d731a74e456dce9f914817d2e4eb65eceeb908ded48913470bd6df4257ac1c86";
const INPUT_SCHEMA_FP: &str =
    "sha256:bdd32eed78e57bc52fc25ef8fa7d8745a3d30e22e752461dd0553d47f9fef2e7";
const FACT_SCHEMA_FP: &str =
    "sha256:c8a0f54ae4e504dd6a0921ec57b6736ea9ec918448969bc931782d5c1969a6f6";
const RESOLUTION_STACK_REF: &str = "handbook.context-resolution.shipped-root@1.0.0";
const RESOLUTION_STACK_FP: &str =
    "sha256:9e95fdef90b98e28acb60bfe96a72f122418b56b1531eafe8ab6cff3eb7668b4";
const EVIDENCE_KIND: &str = "handbook.evidence.schema-conformance";
const INPUT_ROLE: &str = "handbook.input.json-schema-validation";

#[derive(Debug)]
pub struct ProcessFailure;

pub fn run_one_shot() -> Result<(), ProcessFailure> {
    let mut bytes = Vec::new();
    std::io::stdin()
        .lock()
        .take((REQUEST_BYTES + 2) as u64)
        .read_to_end(&mut bytes)
        .map_err(|_| ProcessFailure)?;
    let request = admit_frame(&bytes)?;
    let result = match validate_request(&request) {
        Ok(input) => match evaluate_json_schema_v1(&input) {
            Ok(fact) => completed_result(&request, fact)?,
            Err(error) => refused_result(&request, error)?,
        },
        Err(error) => refused_result(&request, error)?,
    };
    let mut output = canonical_bytes(&result).map_err(|_| ProcessFailure)?;
    if output.is_empty() || output.len() > request.effective_limits.result_bytes {
        return Err(ProcessFailure);
    }
    output.push(b'\n');
    let mut stdout = std::io::stdout().lock();
    stdout.write_all(&output).map_err(|_| ProcessFailure)?;
    stdout.flush().map_err(|_| ProcessFailure)
}

fn admit_frame(bytes: &[u8]) -> Result<Request, ProcessFailure> {
    if bytes.len() < 2
        || bytes.len() > REQUEST_BYTES + 1
        || bytes.last() != Some(&b'\n')
        || bytes[..bytes.len() - 1].contains(&b'\n')
    {
        return Err(ProcessFailure);
    }
    let body = &bytes[..bytes.len() - 1];
    let _: DuplicateCheckedValue = serde_json::from_slice(body).map_err(|_| ProcessFailure)?;
    let request: Request = serde_json::from_slice(body).map_err(|_| ProcessFailure)?;
    if canonical_bytes(&request).map_err(|_| ProcessFailure)? != body {
        return Err(ProcessFailure);
    }
    request.verify_derived_binding()?;
    Ok(request)
}

fn validate_request(request: &Request) -> Result<JsonSchemaValidationInputV1, ValidatorRefusalV1> {
    validate_binding(
        &request.request_schema,
        REQUEST_SCHEMA_REF,
        Some(REQUEST_SCHEMA_FP),
    )?;
    validate_binding(
        &request.result_schema,
        RESULT_SCHEMA_REF,
        Some(RESULT_SCHEMA_FP),
    )?;
    validate_binding(&request.manifest, DOCK_REF, None)?;
    validate_binding(&request.implementation_bundle, BUNDLE_REF, None)?;
    validate_binding(
        &request.protocol,
        PROTOCOL_REF,
        Some(&protocol_fingerprint()?),
    )?;
    validate_resolution(&request.resolution_ceiling)?;
    if request.run.requested_resolution != request.resolution_ceiling {
        return Err(ValidatorRefusalV1::InputSubstituted);
    }
    validate_resolution(&request.run.requested_resolution)?;
    validate_run(&request.run)?;
    if request.requested_evidence_kinds != [EVIDENCE_KIND]
        || request.inputs.len() != 1
        || request.network != NetworkPolicy::Denied
        || request.effective_limits != ProtocolLimits::v1()
    {
        return Err(ValidatorRefusalV1::InputMalformed);
    }
    let input = &request.inputs[0];
    if input.role != INPUT_ROLE || input.media_type != "application/json" {
        return Err(ValidatorRefusalV1::InputMalformed);
    }
    validate_binding(
        &input.schema,
        VALIDATION_INPUT_SCHEMA_REF_V1,
        Some(INPUT_SCHEMA_FP),
    )?;
    let computed = canonical_fingerprint(&input.content)?;
    validate_fingerprint(&input.content_fingerprint)?;
    if input.content_fingerprint != computed {
        return Err(ValidatorRefusalV1::InputSubstituted);
    }
    serde_json::from_value(input.content.clone()).map_err(|error| {
        let text = error.to_string();
        if text.contains("handbook.validator.input-substituted") {
            ValidatorRefusalV1::InputSubstituted
        } else if text.contains("handbook.validator.limit-exceeded") {
            ValidatorRefusalV1::LimitExceeded
        } else {
            ValidatorRefusalV1::InputMalformed
        }
    })
}

fn validate_run(run: &EvaluationRunBinding) -> Result<(), ValidatorRefusalV1> {
    validate_content_binding(&run.evaluation_run)?;
    validate_content_binding(&run.evaluation_lifecycle_basis)?;
    validate_binding(&run.contract, &run.contract.reference, None)?;
    validate_content_binding(&run.subject.subject)?;
    validate_matcher_inputs(&run.subject.matcher_inputs)?;
    validate_fingerprint(&run.lifecycle_head_fingerprint)?;
    if run.semantic_time_utc.is_empty()
        || run.semantic_time_utc.len() > 64
        || run.semantic_time_utc.chars().any(char::is_control)
        || run.claim_ids != ["payload.schema-conformant"]
        || run.cases.len() != 1
        || run.cases[0].case_id != "case.schema-conformance"
    {
        return Err(ValidatorRefusalV1::InputMalformed);
    }
    let case = &run.cases[0];
    validate_content_binding(&case.case)?;
    validate_matcher_inputs(&case.matcher_inputs)
}

fn validate_matcher_inputs(values: &[MatcherInput]) -> Result<(), ValidatorRefusalV1> {
    if values.is_empty()
        || values.windows(2).any(|pair| pair[0].field >= pair[1].field)
        || values.iter().any(|value| {
            !valid_stable_id(&value.field)
                || value.value.is_empty()
                || value.value.len() > 4_096
                || value.value.chars().any(char::is_control)
        })
    {
        Err(ValidatorRefusalV1::InputMalformed)
    } else {
        Ok(())
    }
}

fn validate_resolution(value: &Resolution) -> Result<(), ValidatorRefusalV1> {
    validate_binding(
        &value.resolution_stack,
        RESOLUTION_STACK_REF,
        Some(RESOLUTION_STACK_FP),
    )?;
    let exact = value.scope_horizon == "resolution.scope.local-observation"
        && value.detail_resolution == "resolution.detail.identifier-only"
        && value.temporal_horizon == "resolution.time.current-operation"
        && value.authority_horizon == "resolution.authority.read-only"
        && value.memory_horizon == "resolution.memory.operation"
        && value.validation_horizon == "resolution.validation.observation-only";
    if exact {
        Ok(())
    } else {
        Err(ValidatorRefusalV1::InputMalformed)
    }
}

fn validate_binding(
    binding: &Binding,
    expected_ref: &str,
    expected_fingerprint: Option<&str>,
) -> Result<(), ValidatorRefusalV1> {
    if binding.reference != expected_ref || !valid_exact_ref(&binding.reference) {
        return Err(ValidatorRefusalV1::InputMalformed);
    }
    validate_fingerprint(&binding.fingerprint)?;
    if expected_fingerprint.is_some_and(|expected| binding.fingerprint != expected) {
        return Err(ValidatorRefusalV1::InputSubstituted);
    }
    Ok(())
}

fn validate_content_binding(binding: &Binding) -> Result<(), ValidatorRefusalV1> {
    if binding.reference.is_empty()
        || binding.reference.len() > 1_024
        || binding.reference.chars().any(char::is_control)
    {
        return Err(ValidatorRefusalV1::InputMalformed);
    }
    validate_fingerprint(&binding.fingerprint)
}

fn validate_fingerprint(value: &str) -> Result<(), ValidatorRefusalV1> {
    let bytes = value.as_bytes();
    if bytes.len() == 71
        && value.starts_with("sha256:")
        && bytes[7..]
            .iter()
            .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
    {
        Ok(())
    } else {
        Err(ValidatorRefusalV1::InputMalformed)
    }
}

fn completed_result(
    request: &Request,
    fact: handbook_dock_json_schema::SchemaConformanceFactV1,
) -> Result<Value, ProcessFailure> {
    let payload = serde_json::to_value(&fact).map_err(|_| ProcessFailure)?;
    let observation = match fact.conformance() {
        ConformanceV1::Conformant => "satisfied",
        ConformanceV1::Nonconformant => "violated",
    };
    let candidate = json!({
        "claim_id": request.run.claim_ids[0],
        "case_id": request.run.cases[0].case_id,
        "evidence_kind": EVIDENCE_KIND,
        "payload_schema": binding_value(SCHEMA_CONFORMANCE_FACT_SCHEMA_REF_V1, FACT_SCHEMA_FP),
        "payload": payload,
        "payload_fingerprint": canonical_fingerprint(&payload).map_err(|_| ProcessFailure)?,
        "observation": observation
    });
    result_value(
        request,
        "completed",
        Some(json!({
            "observed": [request.run.claim_ids[0]],
            "unobserved": [],
            "typed_excluded": []
        })),
        vec![candidate],
        vec![],
        Some(serde_json::to_value(&request.resolution_ceiling).map_err(|_| ProcessFailure)?),
        None,
    )
}

fn refused_result(request: &Request, error: ValidatorRefusalV1) -> Result<Value, ProcessFailure> {
    let (reason, code, message) = diagnostic(error);
    result_value(
        request,
        "refused",
        None,
        vec![],
        vec![json!({"code": code, "message": message})],
        None,
        Some(reason),
    )
}

#[allow(clippy::too_many_arguments)]
fn result_value(
    request: &Request,
    status: &str,
    claim_partition: Option<Value>,
    candidates: Vec<Value>,
    diagnostics: Vec<Value>,
    actual_resolution: Option<Value>,
    refusal_reason: Option<&str>,
) -> Result<Value, ProcessFailure> {
    let preimage = json!({
        "result_schema": request.result_schema,
        "protocol": request.protocol,
        "request": binding_value(&request.request_ref, &request.request_fingerprint),
        "manifest": request.manifest,
        "implementation_bundle": request.implementation_bundle,
        "evaluation_run": request.run.evaluation_run,
        "status": status,
        "claim_partition": claim_partition,
        "candidates": candidates,
        "diagnostics": diagnostics,
        "actual_resolution": actual_resolution,
        "refusal_reason": refusal_reason
    });
    let result_fingerprint = canonical_fingerprint(&preimage).map_err(|_| ProcessFailure)?;
    let result_hex = result_fingerprint
        .strip_prefix("sha256:")
        .ok_or(ProcessFailure)?;
    let mut result = preimage;
    result["result_ref"] = json!(format!("{}#result/{result_hex}", request.request_ref));
    result["result_fingerprint"] = json!(result_fingerprint);
    Ok(result)
}

fn diagnostic(error: ValidatorRefusalV1) -> (&'static str, &'static str, &'static str) {
    match error {
        ValidatorRefusalV1::InputMalformed => (
            "input_refused",
            "handbook.validator.input-malformed",
            "validator input was refused",
        ),
        ValidatorRefusalV1::InputSubstituted => (
            "input_refused",
            "handbook.validator.input-substituted",
            "validator input binding was substituted",
        ),
        ValidatorRefusalV1::SchemaUnsupported => (
            "unsupported_input",
            "handbook.validator.schema-unsupported",
            "schema feature is unsupported",
        ),
        ValidatorRefusalV1::RefRemote => (
            "unsupported_input",
            "handbook.validator.ref-remote",
            "schema reference is not local",
        ),
        ValidatorRefusalV1::RefMissing => (
            "unsupported_input",
            "handbook.validator.ref-missing",
            "schema reference target is unavailable",
        ),
        ValidatorRefusalV1::LimitExceeded => (
            "limit_refused",
            "handbook.validator.limit-exceeded",
            "validator limit was exceeded",
        ),
        ValidatorRefusalV1::ObservationUnavailable => (
            "observation_unavailable",
            "handbook.validator.observation-unavailable",
            "normalized observation is unavailable",
        ),
        ValidatorRefusalV1::InternalRefusal => (
            "internal_refusal",
            "handbook.validator.internal-refusal",
            "validator internal invariant failed",
        ),
    }
}

fn protocol_fingerprint() -> Result<String, ValidatorRefusalV1> {
    canonical_fingerprint(&json!({
        "protocol": PROTOCOL_REF,
        "request_schema": binding_value(REQUEST_SCHEMA_REF, REQUEST_SCHEMA_FP),
        "result_schema": binding_value(RESULT_SCHEMA_REF, RESULT_SCHEMA_FP)
    }))
}

fn binding_value(reference: &str, fingerprint: &str) -> Value {
    json!({"ref": reference, "fingerprint": fingerprint})
}

fn canonical_bytes<T: Serialize>(value: &T) -> Result<Vec<u8>, ValidatorRefusalV1> {
    serde_json_canonicalizer::to_vec(value).map_err(|_| ValidatorRefusalV1::InternalRefusal)
}

fn canonical_fingerprint<T: Serialize>(value: &T) -> Result<String, ValidatorRefusalV1> {
    let bytes = canonical_bytes(value)?;
    Ok(format!("sha256:{:x}", Sha256::digest(bytes)))
}

fn valid_stable_id(value: &str) -> bool {
    (3..=255).contains(&value.len())
        && value.is_ascii()
        && value.split('.').count() >= 2
        && value.split('.').all(|segment| {
            let bytes = segment.as_bytes();
            !bytes.is_empty()
                && bytes[0].is_ascii_lowercase()
                && bytes
                    .last()
                    .is_some_and(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit())
                && bytes
                    .iter()
                    .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || *byte == b'-')
        })
}

fn valid_exact_ref(value: &str) -> bool {
    let Some((identity, version)) = value.rsplit_once('@') else {
        return false;
    };
    valid_stable_id(identity)
        && version.split('.').count() == 3
        && version.split('.').all(|part| {
            !part.is_empty()
                && part.bytes().all(|byte| byte.is_ascii_digit())
                && (part == "0" || !part.starts_with('0'))
        })
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Binding {
    #[serde(rename = "ref")]
    reference: String,
    fingerprint: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct MatcherInput {
    field: String,
    value: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct EvaluationSubject {
    subject: Binding,
    matcher_inputs: Vec<MatcherInput>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct EvaluationCase {
    case_id: String,
    case: Binding,
    matcher_inputs: Vec<MatcherInput>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Resolution {
    resolution_stack: Binding,
    scope_horizon: String,
    detail_resolution: String,
    temporal_horizon: String,
    authority_horizon: String,
    memory_horizon: String,
    validation_horizon: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct EvaluationRunBinding {
    evaluation_run: Binding,
    evaluation_lifecycle_basis: Binding,
    contract: Binding,
    claim_ids: Vec<String>,
    subject: EvaluationSubject,
    cases: Vec<EvaluationCase>,
    requested_resolution: Resolution,
    semantic_time_utc: String,
    lifecycle_head_fingerprint: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct InlineInput {
    role: String,
    media_type: String,
    schema: Binding,
    content: Value,
    content_fingerprint: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum NetworkPolicy {
    Denied,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ProtocolLimits {
    request_bytes: usize,
    result_bytes: usize,
    stderr_bytes: usize,
    diagnostic_bytes: usize,
    candidate_bytes: usize,
    input_count: usize,
    candidate_count: usize,
    diagnostic_count: usize,
    wall_timeout_ms: u64,
    cancellation_grace_ms: u64,
}

impl ProtocolLimits {
    const fn v1() -> Self {
        Self {
            request_bytes: REQUEST_BYTES,
            result_bytes: RESULT_BYTES,
            stderr_bytes: 4_096,
            diagnostic_bytes: 1_024,
            candidate_bytes: 256 * 1_024,
            input_count: 1,
            candidate_count: 1,
            diagnostic_count: 8,
            wall_timeout_ms: 5_000,
            cancellation_grace_ms: 100,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Request {
    request_schema: Binding,
    result_schema: Binding,
    protocol: Binding,
    request_ref: String,
    request_fingerprint: String,
    manifest: Binding,
    implementation_bundle: Binding,
    run: EvaluationRunBinding,
    resolution_ceiling: Resolution,
    requested_evidence_kinds: Vec<String>,
    inputs: Vec<InlineInput>,
    effective_limits: ProtocolLimits,
    network: NetworkPolicy,
}

impl Request {
    fn verify_derived_binding(&self) -> Result<(), ProcessFailure> {
        validate_fingerprint(&self.request_fingerprint).map_err(|_| ProcessFailure)?;
        let expected =
            canonical_fingerprint(&RequestPreimage::from(self)).map_err(|_| ProcessFailure)?;
        let hex = expected.strip_prefix("sha256:").ok_or(ProcessFailure)?;
        if self.request_fingerprint != expected
            || self.request_ref != format!("{}#request/{hex}", self.manifest.reference)
        {
            return Err(ProcessFailure);
        }
        Ok(())
    }
}

#[derive(Serialize)]
struct RequestPreimage<'a> {
    request_schema: &'a Binding,
    result_schema: &'a Binding,
    protocol: &'a Binding,
    manifest: &'a Binding,
    implementation_bundle: &'a Binding,
    run: &'a EvaluationRunBinding,
    resolution_ceiling: &'a Resolution,
    requested_evidence_kinds: &'a [String],
    inputs: &'a [InlineInput],
    effective_limits: &'a ProtocolLimits,
    network: NetworkPolicy,
}

impl<'a> From<&'a Request> for RequestPreimage<'a> {
    fn from(request: &'a Request) -> Self {
        Self {
            request_schema: &request.request_schema,
            result_schema: &request.result_schema,
            protocol: &request.protocol,
            manifest: &request.manifest,
            implementation_bundle: &request.implementation_bundle,
            run: &request.run,
            resolution_ceiling: &request.resolution_ceiling,
            requested_evidence_kinds: &request.requested_evidence_kinds,
            inputs: &request.inputs,
            effective_limits: &request.effective_limits,
            network: request.network,
        }
    }
}

struct DuplicateCheckedValue;

impl<'de> Deserialize<'de> for DuplicateCheckedValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CheckedVisitor;
        impl<'de> Visitor<'de> for CheckedVisitor {
            type Value = DuplicateCheckedValue;
            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("JSON value without duplicate object members")
            }
            fn visit_bool<E>(self, _: bool) -> Result<Self::Value, E> {
                Ok(DuplicateCheckedValue)
            }
            fn visit_i64<E>(self, _: i64) -> Result<Self::Value, E> {
                Ok(DuplicateCheckedValue)
            }
            fn visit_u64<E>(self, _: u64) -> Result<Self::Value, E> {
                Ok(DuplicateCheckedValue)
            }
            fn visit_f64<E>(self, _: f64) -> Result<Self::Value, E> {
                Ok(DuplicateCheckedValue)
            }
            fn visit_str<E>(self, _: &str) -> Result<Self::Value, E> {
                Ok(DuplicateCheckedValue)
            }
            fn visit_string<E>(self, _: String) -> Result<Self::Value, E> {
                Ok(DuplicateCheckedValue)
            }
            fn visit_none<E>(self) -> Result<Self::Value, E> {
                Ok(DuplicateCheckedValue)
            }
            fn visit_unit<E>(self) -> Result<Self::Value, E> {
                Ok(DuplicateCheckedValue)
            }
            fn visit_seq<A>(self, mut sequence: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                while sequence.next_element::<DuplicateCheckedValue>()?.is_some() {}
                Ok(DuplicateCheckedValue)
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut keys = BTreeSet::new();
                while let Some(key) = map.next_key::<String>()? {
                    if !keys.insert(key) {
                        return Err(serde::de::Error::custom("duplicate JSON member"));
                    }
                    map.next_value::<DuplicateCheckedValue>()?;
                }
                Ok(DuplicateCheckedValue)
            }
        }
        deserializer.deserialize_any(CheckedVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_refusal_reasons_and_diagnostics_are_closed_and_bounded() {
        let cases = [
            ValidatorRefusalV1::InputMalformed,
            ValidatorRefusalV1::InputSubstituted,
            ValidatorRefusalV1::SchemaUnsupported,
            ValidatorRefusalV1::RefRemote,
            ValidatorRefusalV1::RefMissing,
            ValidatorRefusalV1::LimitExceeded,
            ValidatorRefusalV1::ObservationUnavailable,
            ValidatorRefusalV1::InternalRefusal,
        ];
        let mut codes = BTreeSet::new();
        for error in cases {
            let (reason, code, message) = diagnostic(error);
            assert!(matches!(
                reason,
                "input_refused"
                    | "unsupported_input"
                    | "limit_refused"
                    | "observation_unavailable"
                    | "internal_refusal"
            ));
            assert!(codes.insert(code));
            assert!(message.len() <= 1_024);
            assert!(!message.chars().any(char::is_control));
        }
        assert_eq!(codes.len(), 8);
    }
}
