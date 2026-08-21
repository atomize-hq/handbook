#[path = "../package-support/staging.rs"]
mod staging;

use handbook_dock_json_schema::{
    ExactSchemaBindingV1, JsonInstanceV1, JsonSchemaValidationInputV1, LogicalSchemaPathV1,
    SchemaClosureMemberV1,
};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

const DIALECT: &str = "https://json-schema.org/draft/2020-12/schema";
const ROOT_PATH: &str = "definitions/schemas/example/root/1.0.0.schema.json";
const ZERO_FP: &str = "sha256:0000000000000000000000000000000000000000000000000000000000000000";
type RequestMutation = (&'static str, fn(&mut Value));

fn fingerprint<T: serde::Serialize>(value: &T) -> String {
    let bytes = serde_json_canonicalizer::to_vec(value).unwrap();
    format!("sha256:{:x}", Sha256::digest(bytes))
}

fn exact_binding(reference: &str, fingerprint: &str) -> Value {
    json!({"ref": reference, "fingerprint": fingerprint})
}

fn validation_input(schema: Value, instance: Value) -> Value {
    let member =
        SchemaClosureMemberV1::new(LogicalSchemaPathV1::parse(ROOT_PATH).unwrap(), schema).unwrap();
    let input = JsonSchemaValidationInputV1::new(
        ExactSchemaBindingV1::parse(
            "handbook.schema.example@1.0.0",
            member.document_fingerprint().clone(),
        )
        .unwrap(),
        LogicalSchemaPathV1::parse(ROOT_PATH).unwrap(),
        vec![member],
        JsonInstanceV1::new(instance).unwrap(),
    )
    .unwrap();
    serde_json::to_value(input).unwrap()
}

struct Fixture {
    root: PathBuf,
    manifest: Value,
}

impl Fixture {
    fn new() -> Self {
        let binary = Path::new(env!("CARGO_BIN_EXE_handbook-dock-json-schema"));
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let root = std::env::temp_dir().join(format!(
            "handbook-dock-json-schema-packet3-{}-{nonce}",
            std::process::id()
        ));
        let staged = staging::stage_native_package_v1(
            binary,
            &root,
            env!("HANDBOOK_DOCK_JSON_SCHEMA_TARGET"),
            std::env::consts::EXE_SUFFIX,
        )
        .unwrap();
        assert!(staged
            .package_root
            .ends_with(env!("HANDBOOK_DOCK_JSON_SCHEMA_TARGET")));
        assert!(staged.executable_path.is_file());
        let manifest = serde_json::from_slice(&staged.manifest_bytes).unwrap();
        Self { root, manifest }
    }

    fn request(&self, content: Value) -> Value {
        let content_fingerprint = fingerprint(&content);
        let resolution = self.manifest["resolution_ceiling"].clone();
        let limits = self.manifest["limits"].clone();
        let mut request = json!({
            "request_schema": self.manifest["request_schema"].clone(),
            "result_schema": self.manifest["result_schema"].clone(),
            "protocol": self.manifest["protocol"].clone(),
            "request_ref": "pending",
            "request_fingerprint": ZERO_FP,
            "manifest": {
                "ref": self.manifest["dock_ref"].clone(),
                "fingerprint": self.manifest["manifest_fingerprint"].clone()
            },
            "implementation_bundle": {
                "ref": self.manifest["implementation_bundle"]["bundle_ref"].clone(),
                "fingerprint": self.manifest["implementation_bundle"]["bundle_fingerprint"].clone()
            },
            "run": {
                "evaluation_run": {
                    "ref": "handbook.contract.fixture@1.0.0#evaluation/run-0001",
                    "fingerprint": format!("sha256:{}", "7".repeat(64))
                },
                "evaluation_lifecycle_basis": {
                    "ref": "handbook.contract.fixture@1.0.0#evaluation-lifecycle-basis/exact",
                    "fingerprint": format!("sha256:{}", "8".repeat(64))
                },
                "contract": exact_binding(
                    "handbook.contract.fixture@1.0.0",
                    &format!("sha256:{}", "6".repeat(64))
                ),
                "claim_ids": ["payload.schema-conformant"],
                "subject": {
                    "subject": {
                        "ref": "subject.fixture#revision/exact",
                        "fingerprint": format!("sha256:{}", "a".repeat(64))
                    },
                    "matcher_inputs": [{"field": "artifact.kind", "value": "json-document"}]
                },
                "cases": [{
                    "case_id": "case.schema-conformance",
                    "case": {
                        "ref": "case.fixture#revision/exact",
                        "fingerprint": format!("sha256:{}", "b".repeat(64))
                    },
                    "matcher_inputs": [{"field": "validation.case", "value": "primary"}]
                }],
                "requested_resolution": resolution.clone(),
                "semantic_time_utc": "2026-08-20T12:00:00Z",
                "lifecycle_head_fingerprint": format!("sha256:{}", "c".repeat(64))
            },
            "resolution_ceiling": resolution,
            "requested_evidence_kinds": ["handbook.evidence.schema-conformance"],
            "inputs": [{
                "role": "handbook.input.json-schema-validation",
                "media_type": "application/json",
                "schema": self.manifest["supported_inputs"][0]["schema"].clone(),
                "content": content,
                "content_fingerprint": content_fingerprint
            }],
            "effective_limits": limits,
            "network": "denied"
        });
        refresh_request_binding(&mut request);
        request
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn refresh_request_binding(request: &mut Value) {
    let mut preimage = request.clone();
    preimage.as_object_mut().unwrap().remove("request_ref");
    preimage
        .as_object_mut()
        .unwrap()
        .remove("request_fingerprint");
    let request_fingerprint = fingerprint(&preimage);
    let hex = request_fingerprint.strip_prefix("sha256:").unwrap();
    let manifest_ref = request["manifest"]["ref"].as_str().unwrap();
    request["request_ref"] = json!(format!("{manifest_ref}#request/{hex}"));
    request["request_fingerprint"] = json!(request_fingerprint);
}

fn run_frame(frame: &[u8]) -> Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_handbook-dock-json-schema"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all(frame).unwrap();
    child.wait_with_output().unwrap()
}

fn run_request(request: &Value) -> Value {
    let mut frame = serde_json_canonicalizer::to_vec(request).unwrap();
    frame.push(b'\n');
    let output = run_frame(&frame);
    assert!(output.status.success(), "stderr={:?}", output.stderr);
    assert!(output.stderr.is_empty());
    assert_eq!(output.stdout.last(), Some(&b'\n'));
    assert!(!output.stdout[..output.stdout.len() - 1].contains(&b'\n'));
    let result: Value = serde_json::from_slice(&output.stdout[..output.stdout.len() - 1]).unwrap();
    assert_eq!(
        serde_json_canonicalizer::to_vec(&result).unwrap(),
        output.stdout[..output.stdout.len() - 1]
    );
    verify_result_binding(&result);
    result
}

fn verify_result_binding(result: &Value) {
    let mut preimage = result.clone();
    preimage.as_object_mut().unwrap().remove("result_ref");
    preimage
        .as_object_mut()
        .unwrap()
        .remove("result_fingerprint");
    let expected = fingerprint(&preimage);
    assert_eq!(result["result_fingerprint"], expected);
    assert!(result["result_ref"]
        .as_str()
        .unwrap()
        .ends_with(expected.strip_prefix("sha256:").unwrap()));
}

#[test]
fn canonical_conformant_and_nonconformant_requests_complete_deterministically() {
    let fixture = Fixture::new();
    for (instance, expected_conformance, expected_observation) in [
        (json!("ok"), "conformant", "satisfied"),
        (json!(7), "nonconformant", "violated"),
    ] {
        let request = fixture.request(validation_input(
            json!({"$schema": DIALECT, "type": "string"}),
            instance,
        ));
        let first = run_request(&request);
        let second = run_request(&request);
        assert_eq!(first, second);
        assert_eq!(first["status"], "completed");
        assert_eq!(first["candidates"].as_array().unwrap().len(), 1);
        assert_eq!(first["candidates"][0]["observation"], expected_observation);
        assert_eq!(
            first["candidates"][0]["payload"]["conformance"],
            expected_conformance
        );
        assert_eq!(
            first["candidates"][0]["payload_schema"],
            exact_binding(
                staging::FACT_SCHEMA_REF_V1,
                staging::FACT_SCHEMA_FINGERPRINT_V1
            )
        );
        assert_eq!(
            first["claim_partition"]["observed"],
            json!(["payload.schema-conformant"])
        );
        assert_eq!(first["claim_partition"]["unobserved"], json!([]));
        assert_eq!(first["claim_partition"]["typed_excluded"], json!([]));
        assert_eq!(first["diagnostics"], json!([]));
        assert_eq!(
            first["actual_resolution"],
            fixture.manifest["resolution_ceiling"]
        );
    }
}

#[test]
fn valid_requests_map_every_reachable_child_refusal_to_the_closed_wire_taxonomy() {
    let fixture = Fixture::new();
    let cases = [
        (
            json!({"$schema": DIALECT, "$id": "urn:forbidden"}),
            json!(null),
            "unsupported_input",
            "handbook.validator.schema-unsupported",
        ),
        (
            json!({"$schema": DIALECT, "$ref": "https://example.invalid/schema"}),
            json!(null),
            "unsupported_input",
            "handbook.validator.ref-remote",
        ),
        (
            json!({"$schema": DIALECT, "$ref": "missing.schema.json#/x"}),
            json!(null),
            "unsupported_input",
            "handbook.validator.ref-missing",
        ),
        (
            json!({"$schema": DIALECT, "pattern": "^(?!bad).+$"}),
            json!("bad"),
            "observation_unavailable",
            "handbook.validator.observation-unavailable",
        ),
    ];
    for (schema, instance, reason, code) in cases {
        let result = run_request(&fixture.request(validation_input(schema, instance)));
        assert_eq!(result["status"], "refused");
        assert_eq!(result["refusal_reason"], reason);
        assert_eq!(result["diagnostics"][0]["code"], code);
        assert_eq!(result["candidates"], json!([]));
        assert!(result["claim_partition"].is_null());
        assert!(result["actual_resolution"].is_null());
    }

    let mut malformed = fixture.request(validation_input(json!({"$schema": DIALECT}), json!(null)));
    malformed["inputs"][0]["role"] = json!("handbook.input.wrong");
    refresh_request_binding(&mut malformed);
    let result = run_request(&malformed);
    assert_eq!(result["refusal_reason"], "input_refused");
    assert_eq!(
        result["diagnostics"][0]["code"],
        "handbook.validator.input-malformed"
    );

    let mut substituted =
        fixture.request(validation_input(json!({"$schema": DIALECT}), json!(null)));
    substituted["inputs"][0]["content"]["root_document_fingerprint"] = json!(ZERO_FP);
    let content = substituted["inputs"][0]["content"].clone();
    substituted["inputs"][0]["content_fingerprint"] = json!(fingerprint(&content));
    refresh_request_binding(&mut substituted);
    let result = run_request(&substituted);
    assert_eq!(result["refusal_reason"], "input_refused");
    assert_eq!(
        result["diagnostics"][0]["code"],
        "handbook.validator.input-substituted"
    );

    let mut limited = fixture.request(validation_input(json!({"$schema": DIALECT}), json!(null)));
    let long_path = format!("definitions/schemas/{}.schema.json", "a".repeat(1_100));
    limited["inputs"][0]["content"]["root_document"] = json!(long_path);
    let content = limited["inputs"][0]["content"].clone();
    limited["inputs"][0]["content_fingerprint"] = json!(fingerprint(&content));
    refresh_request_binding(&mut limited);
    let result = run_request(&limited);
    assert_eq!(result["refusal_reason"], "limit_refused");
    assert_eq!(
        result["diagnostics"][0]["code"],
        "handbook.validator.limit-exceeded"
    );
}

#[test]
fn malformed_empty_extra_multiple_and_noncanonical_frames_are_process_failures() {
    let fixture = Fixture::new();
    let request = fixture.request(validation_input(json!({"$schema": DIALECT}), json!(null)));
    let canonical = serde_json_canonicalizer::to_vec(&request).unwrap();
    let mut canonical_frame = canonical.clone();
    canonical_frame.push(b'\n');
    let invalid_frames = [
        Vec::new(),
        b"not-json\n".to_vec(),
        canonical.clone(),
        [canonical_frame.clone(), b"x".to_vec()].concat(),
        [canonical_frame.clone(), canonical_frame].concat(),
        serde_json::to_vec_pretty(&request)
            .unwrap()
            .into_iter()
            .chain([b'\n'])
            .collect(),
    ];
    for frame in invalid_frames {
        let output = run_frame(&frame);
        assert!(!output.status.success());
        assert!(
            output.stdout.is_empty(),
            "frame leaked stdout: {:?}",
            output.stdout
        );
    }
}

#[test]
fn wrong_protocol_schema_manifest_bundle_partition_resolution_limits_and_fingerprints_fail_closed()
{
    let fixture = Fixture::new();
    let base = fixture.request(validation_input(json!({"$schema": DIALECT}), json!(null)));

    for field in ["request_schema", "result_schema", "protocol"] {
        let mut request = base.clone();
        request[field]["fingerprint"] = json!(ZERO_FP);
        refresh_request_binding(&mut request);
        let result = run_request(&request);
        assert_eq!(result["refusal_reason"], "input_refused", "{field}");
        assert_eq!(
            result["diagnostics"][0]["code"],
            "handbook.validator.input-substituted"
        );
    }

    let mutations: [RequestMutation; 8] = [
        ("manifest", |request| {
            request["manifest"]["ref"] = json!("handbook.dock.other@1.0.0")
        }),
        ("bundle", |request| {
            request["implementation_bundle"]["ref"] = json!("handbook.dock.bundle.other@1.0.0")
        }),
        ("claim", |request| {
            request["run"]["claim_ids"] = json!(["claim.other"])
        }),
        ("case", |request| {
            request["run"]["cases"][0]["case_id"] = json!("case.other")
        }),
        ("kind", |request| {
            request["requested_evidence_kinds"] = json!(["handbook.evidence.other"])
        }),
        ("role", |request| {
            request["inputs"][0]["role"] = json!("handbook.input.other")
        }),
        ("resolution", |request| {
            request["resolution_ceiling"]["scope_horizon"] = json!("resolution.scope.other")
        }),
        ("limits", |request| {
            request["effective_limits"]["candidate_count"] = json!(2)
        }),
    ];
    for (name, mutate) in mutations {
        let mut request = base.clone();
        mutate(&mut request);
        if name == "resolution" {
            request["run"]["requested_resolution"] = request["resolution_ceiling"].clone();
        }
        refresh_request_binding(&mut request);
        let result = run_request(&request);
        assert_eq!(result["refusal_reason"], "input_refused", "{name}");
        assert_eq!(
            result["diagnostics"][0]["code"],
            "handbook.validator.input-malformed"
        );
    }

    for path in [
        "request_fingerprint",
        "manifest",
        "implementation_bundle",
        "run",
    ] {
        let mut request = base.clone();
        match path {
            "request_fingerprint" => request["request_fingerprint"] = json!(ZERO_FP),
            "manifest" => request["manifest"]["fingerprint"] = json!(ZERO_FP),
            "implementation_bundle" => {
                request["implementation_bundle"]["fingerprint"] = json!(ZERO_FP)
            }
            "run" => request["run"]["evaluation_run"]["fingerprint"] = json!(ZERO_FP),
            _ => unreachable!(),
        }
        let mut frame = serde_json_canonicalizer::to_vec(&request).unwrap();
        frame.push(b'\n');
        let output = run_frame(&frame);
        assert!(!output.status.success(), "{path}");
        assert!(output.stdout.is_empty(), "{path}");
    }
}

#[test]
fn staged_package_is_exact_single_member_target_bound_and_registry_neutral() {
    let fixture = Fixture::new();
    let target = env!("HANDBOOK_DOCK_JSON_SCHEMA_TARGET");
    let package = fixture
        .root
        .join("handbook-docks/handbook.dock.json-schema/1.0.0")
        .join(target);
    let manifest_bytes = fs::read(package.join("manifest.json")).unwrap();
    assert_eq!(
        manifest_bytes,
        serde_json_canonicalizer::to_vec(&fixture.manifest).unwrap()
    );
    assert_eq!(fixture.manifest["dock_ref"], staging::DOCK_REF_V1);
    assert_eq!(fixture.manifest["implementation_kind"], "native_process");
    assert_eq!(
        fixture.manifest["implementation_bundle"]["fixed_arguments"],
        json!([])
    );
    assert_eq!(fixture.manifest["implementation_bundle"]["target"], target);
    assert_eq!(
        fixture.manifest["supported_evidence_kinds"],
        json!([staging::EVIDENCE_KIND_V1])
    );
    assert_eq!(fixture.manifest["deterministic"], true);
    assert_eq!(fixture.manifest["offline"], true);
    assert_eq!(fixture.manifest["read_only"], true);

    let bundle = package.join("bundle");
    let files = fs::read_dir(bundle.join("bin"))
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .collect::<Vec<_>>();
    assert_eq!(files.len(), 1);
    assert_eq!(
        files[0].file_name().unwrap().to_string_lossy(),
        format!("handbook-dock-json-schema{}", std::env::consts::EXE_SUFFIX)
    );
    assert_eq!(
        fixture.manifest["implementation_bundle"]["files"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    let file_bytes = fs::read(&files[0]).unwrap();
    let file = &fixture.manifest["implementation_bundle"]["files"][0];
    assert_eq!(file["length"], file_bytes.len() as u64);
    assert_eq!(
        file["sha256"],
        format!("sha256:{:x}", Sha256::digest(&file_bytes))
    );
    assert_eq!(
        fixture.manifest["implementation_bundle"]["arguments_fingerprint"],
        fingerprint(&json!([]))
    );
    let mut bundle_preimage = fixture.manifest["implementation_bundle"].clone();
    bundle_preimage
        .as_object_mut()
        .unwrap()
        .remove("bundle_fingerprint");
    assert_eq!(
        fixture.manifest["implementation_bundle"]["bundle_fingerprint"],
        fingerprint(&bundle_preimage)
    );
    let mut manifest_preimage = fixture.manifest.clone();
    manifest_preimage
        .as_object_mut()
        .unwrap()
        .remove("manifest_fingerprint");
    assert_eq!(
        fixture.manifest["manifest_fingerprint"],
        fingerprint(&manifest_preimage)
    );

    let repeat_root = fixture.root.with_extension("repeat");
    let repeated = staging::stage_native_package_v1(
        Path::new(env!("CARGO_BIN_EXE_handbook-dock-json-schema")),
        &repeat_root,
        target,
        std::env::consts::EXE_SUFFIX,
    )
    .unwrap();
    assert_eq!(repeated.manifest_bytes, manifest_bytes);
    assert_eq!(fs::read(repeated.executable_path).unwrap(), file_bytes);
    fs::remove_dir_all(repeat_root).unwrap();

    let registry =
        fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("../dock/src/registry.rs"))
            .unwrap();
    assert!(registry.contains("manifests: Vec::new()"));
    assert!(!registry.contains(staging::DOCK_REF_V1));
}

#[test]
fn default_binary_source_and_dependency_closure_has_no_ambient_authority() {
    let crate_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let manifest = fs::read_to_string(crate_root.join("Cargo.toml")).unwrap();
    let normal_dependencies = manifest
        .split("[dependencies]")
        .nth(1)
        .unwrap()
        .split("[dev-dependencies]")
        .next()
        .unwrap();
    for forbidden in [
        "handbook-contracts",
        "handbook-dock =",
        "handbook-engine",
        "handbook-sdk",
        "python",
    ] {
        assert!(!normal_dependencies.contains(forbidden), "{forbidden}");
    }
    let main = fs::read_to_string(crate_root.join("src/main.rs")).unwrap();
    let wire = fs::read_to_string(crate_root.join("src/wire.rs")).unwrap();
    let normal = format!("{main}{wire}");
    for forbidden in [
        "std::fs",
        "std::env",
        "std::net",
        "std::process",
        "Command::new",
        "target_os",
        "cfg!(windows)",
        "cfg(windows)",
        "reqwest",
        "with_retriever",
        ".retriever(",
        "libloading",
        "python",
    ] {
        assert!(!normal.contains(forbidden), "{forbidden}");
    }
}

#[test]
fn canonical_request_and_result_match_the_shared_cross_crate_vectors() {
    let fixture = Fixture::new();
    let mut request = fixture.request(validation_input(
        json!({"$schema": DIALECT, "type": "string"}),
        json!("ok"),
    ));
    request["manifest"]["fingerprint"] = json!(format!("sha256:{}", "d".repeat(64)));
    request["implementation_bundle"]["fingerprint"] = json!(format!("sha256:{}", "e".repeat(64)));
    refresh_request_binding(&mut request);
    let result = run_request(&request);
    assert_eq!(
        serde_json_canonicalizer::to_vec(&request).unwrap(),
        include_bytes!("vectors/process-request-v1.json")
            .strip_suffix(b"\n")
            .unwrap()
    );
    assert_eq!(
        serde_json_canonicalizer::to_vec(&result).unwrap(),
        include_bytes!("vectors/process-result-v1.json")
            .strip_suffix(b"\n")
            .unwrap()
    );
}
