use std::path::Path;
use std::process::{Command, Output};

const BOOTSTRAP_REQUEST_SCHEMA_BYTES: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../sdk/schemas/2020-12/handbook.bootstrap-request/1.0.0.schema.json"
));

fn run_in(root: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_handbook"))
        .current_dir(root)
        .args(args)
        .output()
        .unwrap_or_else(|error| panic!("run `{}`: {error}", args.join(" ")))
}

fn machine_document(output: &Output) -> serde_json::Value {
    assert_eq!(
        output.stdout.iter().filter(|byte| **byte == b'\n').count(),
        1,
        "machine stdout must contain one value plus one LF: {}",
        String::from_utf8_lossy(&output.stdout)
    );
    assert!(output.stdout.ends_with(b"\n"));
    assert!(
        output.stderr.is_empty(),
        "typed outcomes belong only on stdout: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).expect("one JSON document")
}

fn assert_adapter_failure(output: &Output, expected: &str) {
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stdout.is_empty());
    assert_eq!(output.stderr, expected.as_bytes());
    assert_eq!(
        output.stderr.iter().filter(|byte| **byte == b'\n').count(),
        1
    );
}

#[test]
fn capability_help_exposes_all_generic_actions_and_requires_one() {
    let root = tempfile::tempdir().unwrap();
    let help = run_in(root.path(), &["capability", "--help"]);
    assert!(help.status.success());
    let stdout = String::from_utf8(help.stdout).unwrap();
    for action in ["bootstrap", "catalog-read", "schema-read", "invoke"] {
        assert!(stdout.contains(action), "missing {action}: {stdout}");
    }

    let output = run_in(root.path(), &["capability"]);
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    assert!(!output.stderr.is_empty());
}

#[test]
fn bootstrap_emits_exact_typed_success_and_refusal_framing() {
    let root = tempfile::tempdir().unwrap();
    let ok = run_in(
        root.path(),
        &[
            "capability",
            "bootstrap",
            "--client-version",
            "1.0.0",
            "--page-size",
            "1",
            "--correlation",
            "packet2-bootstrap",
        ],
    );
    assert!(ok.status.success());
    let ok = machine_document(&ok);
    assert_eq!(ok["outcome_kind"], "ok");
    assert_eq!(ok["response"]["request_id"], "packet2-bootstrap");
    assert_eq!(ok["response"]["first_capability_page"]["page_size"], 1);

    let refused = run_in(
        root.path(),
        &[
            "capability",
            "bootstrap",
            "--client-version",
            "2.0.0",
            "--page-size",
            "1",
        ],
    );
    assert_eq!(refused.status.code(), Some(1));
    assert_eq!(machine_document(&refused)["outcome_kind"], "refused");
}

#[test]
fn bootstrap_adapter_and_syntax_failures_never_fabricate_typed_outcomes() {
    let root = tempfile::tempdir().unwrap();
    for (args, expected) in [
        (
            vec![
                "capability",
                "bootstrap",
                "--client-version",
                "not-semver",
                "--page-size",
                "1",
            ],
            "handbook capability: client version rejected\n",
        ),
        (
            vec![
                "capability",
                "bootstrap",
                "--client-version",
                "1.0.0",
                "--page-size",
                "1",
                "--correlation",
                "not valid",
            ],
            "handbook capability: correlation rejected\n",
        ),
    ] {
        assert_adapter_failure(&run_in(root.path(), &args), expected);
    }

    let syntax = run_in(
        root.path(),
        &["capability", "bootstrap", "--client-version", "1.0.0"],
    );
    assert_eq!(syntax.status.code(), Some(2));
    assert!(syntax.stdout.is_empty());
    assert!(!syntax.stderr.is_empty());
    assert!(syntax.stderr.len() < 1024);
}

#[test]
fn catalog_read_preserves_pagination_cursor_and_mismatch_truth() {
    let root = tempfile::tempdir().unwrap();
    let first = run_in(
        root.path(),
        &[
            "capability",
            "catalog-read",
            "--catalog-kind",
            "capabilities",
            "--page-size",
            "1",
        ],
    );
    assert!(first.status.success());
    let first = machine_document(&first);
    assert_eq!(first["outcome_kind"], "page");
    assert_eq!(first["response"]["start_index"], 0);
    assert_eq!(first["response"]["entries"].as_array().unwrap().len(), 1);
    let cursor = serde_json::to_string(&first["response"]["next_cursor"]).unwrap();

    let second = run_in(
        root.path(),
        &[
            "capability",
            "catalog-read",
            "--catalog-kind",
            "capabilities",
            "--page-size",
            "1",
            "--cursor",
            &cursor,
        ],
    );
    assert!(second.status.success());
    let second = machine_document(&second);
    assert_eq!(second["outcome_kind"], "page");
    assert_eq!(second["response"]["start_index"], 1);

    let mismatch = run_in(
        root.path(),
        &[
            "capability",
            "catalog-read",
            "--catalog-kind",
            "schemas",
            "--page-size",
            "1",
            "--cursor",
            &cursor,
        ],
    );
    assert_eq!(mismatch.status.code(), Some(1));
    let mismatch = machine_document(&mismatch);
    assert_eq!(mismatch["catalog_kind"], "schemas");
    assert_eq!(mismatch["outcome_kind"], "cursor_mismatch");
    assert!(mismatch["response"].is_null());
}

#[test]
fn catalog_adapter_failures_keep_stdout_empty_and_stderr_bounded() {
    let root = tempfile::tempdir().unwrap();
    let cursor = run_in(
        root.path(),
        &[
            "capability",
            "catalog-read",
            "--catalog-kind",
            "capabilities",
            "--page-size",
            "1",
            "--cursor",
            "{}",
        ],
    );
    assert_adapter_failure(&cursor, "handbook capability: catalog cursor rejected\n");

    let page_size = run_in(
        root.path(),
        &[
            "capability",
            "catalog-read",
            "--catalog-kind",
            "capabilities",
            "--page-size",
            "129",
        ],
    );
    assert_adapter_failure(&page_size, "handbook capability: page size rejected\n");
}

#[test]
fn every_existing_catalog_kind_emits_its_exact_typed_page() {
    let root = tempfile::tempdir().unwrap();
    for kind in ["capabilities", "schemas", "profiles"] {
        let output = run_in(
            root.path(),
            &[
                "capability",
                "catalog-read",
                "--catalog-kind",
                kind,
                "--page-size",
                "128",
                "--json",
            ],
        );
        assert!(output.status.success());
        let document = machine_document(&output);
        assert_eq!(document["catalog_kind"], kind);
        assert_eq!(document["outcome_kind"], "page");
        if kind == "schemas" {
            assert!(!document["response"]["next_cursor"].is_null());
        } else {
            assert!(document["response"]["next_cursor"].is_null());
        }
    }
}

#[test]
fn schema_read_emits_exact_checked_in_bytes_without_an_adapter_lf() {
    let root = tempfile::tempdir().unwrap();
    assert!(BOOTSTRAP_REQUEST_SCHEMA_BYTES.ends_with(b"\n"));
    assert!(!BOOTSTRAP_REQUEST_SCHEMA_BYTES.ends_with(b"\n\n"));
    let fingerprint =
        handbook_sdk::contract_membrane::sha256_fingerprint(BOOTSTRAP_REQUEST_SCHEMA_BYTES);
    let output = run_in(
        root.path(),
        &[
            "capability",
            "schema-read",
            "--schema-ref",
            "handbook.bootstrap-request@1.0.0",
            "--manifest-fingerprint",
            fingerprint.as_str(),
        ],
    );
    assert!(output.status.success());
    assert_eq!(output.stdout, BOOTSTRAP_REQUEST_SCHEMA_BYTES);
    assert!(output.stderr.is_empty());
}

#[test]
fn schema_binding_and_manifest_failures_emit_no_schema_bytes() {
    let root = tempfile::tempdir().unwrap();
    let stale = format!("sha256:{}", "0".repeat(64));
    for (schema_ref, fingerprint, expected) in [
        (
            "handbook.bootstrap-request@1.0.0",
            stale.as_str(),
            "handbook capability: schema unavailable\n",
        ),
        (
            "handbook.unknown-schema@1.0.0",
            stale.as_str(),
            "handbook capability: schema unavailable\n",
        ),
        (
            "not-an-exact-ref",
            stale.as_str(),
            "handbook capability: schema binding rejected\n",
        ),
        (
            "handbook.bootstrap-request@1.0.0",
            "not-a-fingerprint",
            "handbook capability: manifest fingerprint rejected\n",
        ),
    ] {
        let output = run_in(
            root.path(),
            &[
                "capability",
                "schema-read",
                "--schema-ref",
                schema_ref,
                "--manifest-fingerprint",
                fingerprint,
            ],
        );
        assert_adapter_failure(&output, expected);
    }
}
