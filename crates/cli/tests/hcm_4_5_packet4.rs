use std::collections::BTreeSet;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

fn root_skill() -> &'static str {
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../install/handbook-home/SKILL.md.tmpl"
    ))
}

fn root_openai() -> &'static str {
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../install/handbook-home/agents/openai.yaml"
    ))
}

fn charter_skill() -> &'static str {
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../install/handbook-home/charter-intake/SKILL.md.tmpl"
    ))
}

fn invocation_response_schema_bytes() -> &'static [u8] {
    include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../sdk/schemas/2020-12/handbook.invocation-response/1.0.0.schema.json"
    ))
}

#[test]
fn root_skill_is_the_closed_generic_capability_algorithm() {
    for required in [
        "git rev-parse --show-toplevel",
        "$handbook_home/bin/handbook",
        "capability bootstrap --client-version 1.0.0 --page-size 128",
        "capability catalog-read",
        "--catalog-kind capabilities",
        "capability schema-read",
        "capability invoke",
        "entry_fingerprint",
        "declared_transports",
        "current_transports",
        "cli_json",
        "request_schema",
        "result_schema",
        "outcome_schemas",
        "required_capabilities",
        "handbook.invocation-response@1.0.0",
        "explicit authorization",
        "Never retry automatically",
        "typed response is controlling",
        "`handbook-charter-intake`",
    ] {
        assert!(
            root_skill().contains(required),
            "root skill missing {required}"
        );
    }
}

#[test]
fn root_skill_contains_no_current_inventory_fixed_use_case_or_transport_route() {
    for forbidden in [
        "artifact.candidate.append@1.0.0",
        "artifact.candidate.promote@1.0.0",
        "artifact.candidate.validate@1.0.0",
        "artifact.instance.list@1.0.0",
        "artifact.kind.list@1.0.0",
        "artifact.read@1.0.0",
        "artifact.validate@1.0.0",
        "flow.resolve@1.0.0",
        "intake.coverage.evaluate@1.0.0",
        "intake.definition.read@1.0.0",
        "intake.record.append@1.0.0",
        "pipeline.capture.apply@1.0.0",
        "pipeline.capture.plan@1.0.0",
        "pipeline.catalog.list@1.0.0",
        "pipeline.catalog.read@1.0.0",
        "pipeline.compile@1.0.0",
        "pipeline.handoff.emit@1.0.0",
        "pipeline.state.apply@1.0.0",
        "posture.transition.apply@1.0.0",
        "handbook.use-case.repository-setup@1.0.0",
        "handbook.use-case.repository-doctor@1.0.0",
        "handbook.use-case.author-charter@1.0.0",
        "handbook.use-case.author-project-context@1.0.0",
        "handbook.use-case.approver-administration@1.0.0",
        "handbook.use-case.pipeline-route-resolve-and-persist@1.0.0",
        "handbook_bootstrap_v1",
        "handbook_catalog_read_v1",
        "handbook_invoke_v1",
        "handbook author ",
        "handbook setup",
        "handbook doctor",
        "handbook pipeline",
    ] {
        assert!(
            !root_skill().contains(forbidden),
            "root skill embeds forbidden route or inventory value: {forbidden}"
        );
    }
}

#[test]
fn root_skill_keeps_sensitive_input_bounded_and_owner_semantics_controlling() {
    for required in [
        "umask 077",
        "mktemp -d",
        "chmod 600",
        "out of logs",
        "Do not print",
        "Preserve unknown",
        "Do not infer success from the process exit code",
        "Do not translate, merge, or reclassify",
        "receipts",
        "replay",
    ] {
        assert!(
            root_skill().contains(required),
            "root skill missing {required}"
        );
    }
    for secret_shape in [
        "sk-",
        "ghp_",
        "AKIA",
        "BEGIN PRIVATE KEY",
        "\"api_key\":",
        "\"idempotency_key\":",
        "export API_KEY=",
    ] {
        assert!(
            !root_skill().contains(secret_shape),
            "root skill contains secret-like material: {secret_shape}"
        );
    }
}

#[test]
fn openai_metadata_routes_generic_capabilities_without_promoting_charter() {
    assert!(root_openai().contains("Discover and invoke ordinary Handbook capabilities"));
    assert!(root_openai().contains("separate Charter intake skill"));
    assert!(!root_openai().contains("author charter"));
    assert!(charter_skill().contains("# Handbook Charter Intake"));
    assert!(charter_skill().contains("immutable intake and candidate lineage"));
}

#[test]
fn generator_projects_exact_thin_file_sets_and_source_bytes() {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repository root");
    let output = tempfile::Builder::new()
        .prefix(".hcm-4-5-packet4-projection-")
        .tempdir_in(&repo_root)
        .expect("temporary projection root");
    let bash_output_path = output
        .path()
        .file_name()
        .expect("relative projection root")
        .to_string_lossy()
        .into_owned();
    let status = Command::new("bash")
        .arg("tools/codex/generate.sh")
        .arg("--output-root")
        .arg(bash_output_path)
        .current_dir(&repo_root)
        .status()
        .expect("run projection generator");
    assert!(status.success(), "projection generator failed: {status}");

    let root_projection = output.path().join("handbook");
    let charter_projection = output.path().join("handbook-charter-intake");
    assert_eq!(
        relative_file_set(&root_projection),
        BTreeSet::from(["SKILL.md".to_owned(), "agents/openai.yaml".to_owned()])
    );
    assert_eq!(
        relative_file_set(&charter_projection),
        BTreeSet::from(["SKILL.md".to_owned(), "agents/openai.yaml".to_owned()])
    );
    assert_eq!(
        fs::read(root_projection.join("SKILL.md")).unwrap(),
        root_skill().as_bytes()
    );
    assert_eq!(
        fs::read(root_projection.join("agents/openai.yaml")).unwrap(),
        root_openai().as_bytes()
    );
    assert_eq!(
        fs::read(charter_projection.join("SKILL.md")).unwrap(),
        charter_skill().as_bytes()
    );
}

#[test]
fn live_skill_algorithm_discovers_schema_admits_and_invokes_without_embedded_identity() {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repository root");
    let bootstrap = run_handbook(
        &repo_root,
        &strings(&[
            "capability",
            "bootstrap",
            "--client-version",
            "1.0.0",
            "--page-size",
            "2",
        ]),
    );
    assert!(bootstrap.status.success());
    let bootstrap = machine_document(&bootstrap);
    assert_eq!(bootstrap["outcome_kind"], "ok");

    let first_page = &bootstrap["response"]["first_capability_page"];
    let mut entries = first_page["entries"].as_array().unwrap().clone();
    let mut next_cursor = first_page["next_cursor"].clone();
    while !next_cursor.is_null() {
        let cursor = serde_json::to_string(&next_cursor).unwrap();
        let page = run_handbook(
            &repo_root,
            &strings(&[
                "capability",
                "catalog-read",
                "--catalog-kind",
                "capabilities",
                "--page-size",
                "2",
                "--cursor",
                &cursor,
            ]),
        );
        assert!(page.status.success());
        let page = machine_document(&page);
        assert_eq!(page["catalog_kind"], "capabilities");
        entries.extend(page["response"]["entries"].as_array().unwrap().clone());
        next_cursor = page["response"]["next_cursor"].clone();
    }

    let selected = entries
        .iter()
        .filter(|entry| {
            entry["class"] == "read_only"
                && string_array_contains(&entry["declared_transports"], "cli_json")
                && string_array_contains(&entry["current_transports"], "cli_json")
        })
        .find(|entry| {
            let schema = run_handbook(
                &repo_root,
                &strings(&[
                    "capability",
                    "schema-read",
                    "--schema-ref",
                    entry["request_schema"]["ref"].as_str().unwrap(),
                    "--manifest-fingerprint",
                    entry["request_schema"]["fingerprint"].as_str().unwrap(),
                ]),
            );
            if !schema.status.success() || !schema.stderr.is_empty() {
                return false;
            }
            let schema: serde_json::Value = serde_json::from_slice(&schema.stdout).unwrap();
            jsonschema::draft202012::options()
                .build(&schema)
                .is_ok_and(|validator| validator.is_valid(&serde_json::json!({})))
        })
        .expect("one discovered read-only capability must admit an empty closed body");

    let mut request = tempfile::NamedTempFile::new().unwrap();
    request.write_all(b"{}").unwrap();
    request.flush().unwrap();
    let invocation = run_handbook(
        &repo_root,
        &strings(&[
            "capability",
            "invoke",
            "--operation-ref",
            selected["operation"]["ref"].as_str().unwrap(),
            "--definition-fingerprint",
            selected["operation"]["fingerprint"].as_str().unwrap(),
            "--capability-entry-fingerprint",
            selected["entry_fingerprint"].as_str().unwrap(),
            "--from-input",
            request.path().to_str().unwrap(),
        ]),
    );
    assert!(invocation.stderr.is_empty());
    let invocation = machine_document(&invocation);
    assert_eq!(invocation["response_kind"], "operation");
    assert_eq!(
        invocation["response"]["operation"]["ref"],
        selected["operation"]["ref"]
    );
    assert!(matches!(
        invocation["response"]["status"].as_str(),
        Some("ok" | "blocked" | "refused" | "error")
    ));
    let response_schema: serde_json::Value =
        serde_json::from_slice(invocation_response_schema_bytes()).unwrap();
    let response_validator = jsonschema::draft202012::options()
        .build(&response_schema)
        .unwrap();
    assert!(response_validator.is_valid(&invocation));
}

fn run_handbook(root: &Path, args: &[String]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_handbook"))
        .current_dir(root)
        .args(args)
        .output()
        .unwrap()
}

fn strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

fn machine_document(output: &Output) -> serde_json::Value {
    assert_eq!(
        output.stdout.iter().filter(|byte| **byte == b'\n').count(),
        1,
        "machine stdout must be one value plus LF: {}",
        String::from_utf8_lossy(&output.stdout)
    );
    assert!(output.stderr.is_empty());
    serde_json::from_slice(&output.stdout).unwrap()
}

fn string_array_contains(value: &serde_json::Value, expected: &str) -> bool {
    value
        .as_array()
        .is_some_and(|values| values.iter().any(|value| value == expected))
}

fn relative_file_set(root: &Path) -> BTreeSet<String> {
    let mut pending = vec![root.to_path_buf()];
    let mut files = BTreeSet::new();
    while let Some(directory) = pending.pop() {
        for entry in fs::read_dir(&directory).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                pending.push(path);
            } else {
                files.insert(
                    path.strip_prefix(root)
                        .unwrap()
                        .to_string_lossy()
                        .replace('\\', "/"),
                );
            }
        }
    }
    files
}
