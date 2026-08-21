use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};

pub const DOCK_REF_V1: &str = "handbook.dock.json-schema@1.0.0";
pub const BUNDLE_REF_V1: &str = "handbook.dock.bundle.json-schema-native@1.0.0";
pub const EVIDENCE_KIND_V1: &str = "handbook.evidence.schema-conformance";
pub const FACT_SCHEMA_REF_V1: &str = "handbook.fact.schema-conformance@1.0.0";
pub const FACT_SCHEMA_FINGERPRINT_V1: &str =
    "sha256:c8a0f54ae4e504dd6a0921ec57b6736ea9ec918448969bc931782d5c1969a6f6";

const MANIFEST_SCHEMA_REF: &str = "handbook.dock-manifest@1.0.0";
const MANIFEST_SCHEMA_FP: &str =
    "sha256:c3878a01a83493800a768bab015cdf9b5762caa43da890cffabf5911ec029db6";
const PROTOCOL_REF: &str = "handbook.dock.process-json@1.0.0";
const REQUEST_SCHEMA_REF: &str = "handbook.dock-process-request@1.0.0";
const REQUEST_SCHEMA_FP: &str =
    "sha256:794aec1a3b700e53d8e168f2f7dde54113effa297e023257d44fa89e90decdf8";
const RESULT_SCHEMA_REF: &str = "handbook.dock-process-result@1.0.0";
const RESULT_SCHEMA_FP: &str =
    "sha256:d731a74e456dce9f914817d2e4eb65eceeb908ded48913470bd6df4257ac1c86";
const INPUT_SCHEMA_REF: &str = "handbook.dock-json-schema-validation-input@1.0.0";
const INPUT_SCHEMA_FP: &str =
    "sha256:bdd32eed78e57bc52fc25ef8fa7d8745a3d30e22e752461dd0553d47f9fef2e7";
const RESOLUTION_STACK_REF: &str = "handbook.context-resolution.shipped-root@1.0.0";
const RESOLUTION_STACK_FP: &str =
    "sha256:9e95fdef90b98e28acb60bfe96a72f122418b56b1531eafe8ab6cff3eb7668b4";

pub struct StagedPackageV1 {
    pub manifest_bytes: Vec<u8>,
    pub package_root: PathBuf,
    pub executable_path: PathBuf,
}

pub fn stage_native_package_v1(
    executable: &Path,
    host_executable_directory: &Path,
    target: &str,
    executable_suffix: &str,
) -> Result<StagedPackageV1, String> {
    validate_target(target)?;
    validate_suffix(executable_suffix)?;
    let expected_name = format!("handbook-dock-json-schema{executable_suffix}");
    if executable.file_name().and_then(|name| name.to_str()) != Some(expected_name.as_str()) {
        return Err("validator executable name does not match the target suffix".to_owned());
    }
    let metadata = fs::symlink_metadata(executable).map_err(io_error("inspect executable"))?;
    if !metadata.file_type().is_file() || metadata.file_type().is_symlink() {
        return Err("validator executable must be one regular file".to_owned());
    }
    let executable_bytes = fs::read(executable).map_err(io_error("read executable"))?;
    let executable_fingerprint = bytes_fingerprint(&executable_bytes);
    let entrypoint = format!("bin/{expected_name}");
    let arguments_fingerprint = canonical_fingerprint(&json!([]))?;
    let files = json!([{
        "path": entrypoint,
        "length": executable_bytes.len() as u64,
        "sha256": executable_fingerprint
    }]);
    let bundle_preimage = json!({
        "bundle_ref": BUNDLE_REF_V1,
        "files": files,
        "entrypoint": entrypoint,
        "entrypoint_fingerprint": executable_fingerprint,
        "fixed_arguments": [],
        "arguments_fingerprint": arguments_fingerprint,
        "target": target
    });
    let bundle_fingerprint = canonical_fingerprint(&bundle_preimage)?;
    let mut bundle = bundle_preimage;
    bundle["bundle_fingerprint"] = json!(bundle_fingerprint);

    let manifest_schema = binding(MANIFEST_SCHEMA_REF, MANIFEST_SCHEMA_FP);
    let request_schema = binding(REQUEST_SCHEMA_REF, REQUEST_SCHEMA_FP);
    let result_schema = binding(RESULT_SCHEMA_REF, RESULT_SCHEMA_FP);
    let protocol_descriptor = json!({
        "protocol": PROTOCOL_REF,
        "request_schema": request_schema,
        "result_schema": result_schema
    });
    let protocol = binding(PROTOCOL_REF, &canonical_fingerprint(&protocol_descriptor)?);
    let resolution = json!({
        "resolution_stack": binding(RESOLUTION_STACK_REF, RESOLUTION_STACK_FP),
        "scope_horizon": "resolution.scope.local-observation",
        "detail_resolution": "resolution.detail.identifier-only",
        "temporal_horizon": "resolution.time.current-operation",
        "authority_horizon": "resolution.authority.read-only",
        "memory_horizon": "resolution.memory.operation",
        "validation_horizon": "resolution.validation.observation-only"
    });
    let limits = json!({
        "request_bytes": 2 * 1024 * 1024,
        "result_bytes": 512 * 1024,
        "stderr_bytes": 4096,
        "diagnostic_bytes": 1024,
        "candidate_bytes": 256 * 1024,
        "input_count": 1,
        "candidate_count": 1,
        "diagnostic_count": 8,
        "wall_timeout_ms": 5000,
        "cancellation_grace_ms": 100
    });
    let manifest_preimage = json!({
        "manifest_schema": manifest_schema,
        "dock_id": "handbook.dock.json-schema",
        "dock_version": "1.0.0",
        "dock_ref": DOCK_REF_V1,
        "protocol": protocol,
        "request_schema": request_schema,
        "result_schema": result_schema,
        "implementation_kind": "native_process",
        "implementation_bundle": bundle,
        "supported_evidence_kinds": [EVIDENCE_KIND_V1],
        "supported_inputs": [{
            "role": "handbook.input.json-schema-validation",
            "media_type": "application/json",
            "schema": binding(INPUT_SCHEMA_REF, INPUT_SCHEMA_FP)
        }],
        "resolution_ceiling": resolution,
        "limits": limits,
        "deterministic": true,
        "offline": true,
        "read_only": true
    });
    let manifest_fingerprint = canonical_fingerprint(&manifest_preimage)?;
    let mut manifest = manifest_preimage;
    manifest["manifest_fingerprint"] = json!(manifest_fingerprint);
    let manifest_bytes = canonical_bytes(&manifest)?;

    let package_root = host_executable_directory
        .join("handbook-docks")
        .join("handbook.dock.json-schema")
        .join("1.0.0")
        .join(target);
    if package_root.exists() {
        return Err("target package root already exists".to_owned());
    }
    let bundle_bin = package_root.join("bundle").join("bin");
    fs::create_dir_all(&bundle_bin).map_err(io_error("create package layout"))?;
    let staged_executable = bundle_bin.join(expected_name);
    fs::write(&staged_executable, &executable_bytes).map_err(io_error("stage executable"))?;
    fs::write(package_root.join("manifest.json"), &manifest_bytes)
        .map_err(io_error("stage manifest"))?;
    let staged_bytes = fs::read(&staged_executable).map_err(io_error("verify executable"))?;
    if staged_bytes != executable_bytes {
        return Err("staged executable bytes changed".to_owned());
    }
    Ok(StagedPackageV1 {
        manifest_bytes,
        package_root,
        executable_path: staged_executable,
    })
}

fn binding(reference: &str, fingerprint: &str) -> Value {
    json!({"ref": reference, "fingerprint": fingerprint})
}

fn canonical_bytes(value: &Value) -> Result<Vec<u8>, String> {
    serde_json_canonicalizer::to_vec(value).map_err(|_| "canonicalization failed".to_owned())
}

fn canonical_fingerprint(value: &Value) -> Result<String, String> {
    Ok(bytes_fingerprint(&canonical_bytes(value)?))
}

fn bytes_fingerprint(bytes: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(bytes))
}

fn validate_target(target: &str) -> Result<(), String> {
    let valid = !target.is_empty()
        && target.len() <= 255
        && target.is_ascii()
        && target.split('-').count() >= 3
        && target.split('-').all(|part| !part.is_empty())
        && target.bytes().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'_' | b'-' | b'.')
        });
    if valid {
        Ok(())
    } else {
        Err("invalid Cargo target tuple".to_owned())
    }
}

fn validate_suffix(suffix: &str) -> Result<(), String> {
    let valid = suffix.is_empty()
        || (suffix.starts_with('.')
            && suffix.len() <= 16
            && suffix[1..]
                .bytes()
                .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit()));
    if valid {
        Ok(())
    } else {
        Err("invalid native executable suffix".to_owned())
    }
}

fn io_error(action: &'static str) -> impl FnOnce(std::io::Error) -> String {
    move |_| format!("{action} failed")
}
