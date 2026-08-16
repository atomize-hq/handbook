#[path = "../src/coverage.rs"]
mod coverage;

use std::collections::{BTreeMap, BTreeSet};
use std::process::{Command, Output};

fn handbook() -> Command {
    Command::new(env!("CARGO_BIN_EXE_handbook"))
}

fn run(args: &[&str]) -> Output {
    handbook().args(args).output().expect("run handbook")
}

#[test]
fn coverage_table_is_closed_unique_and_matches_all_36_authorized_leaf_modes() {
    assert_eq!(coverage::validate(), Ok(()));
    let rows = coverage::rows();
    assert_eq!(rows.len(), 36);
    let leaves = rows.iter().map(|row| row.leaf).collect::<BTreeSet<_>>();
    assert_eq!(leaves.len(), rows.len(), "duplicate coverage row");

    let expected = [
        "setup.auto",
        "setup.init",
        "setup.refresh",
        "author.charter.author",
        "author.charter.approve",
        "author.charter.promote",
        "author.charter.validate",
        "author.project_context.author",
        "author.project_context.validate",
        "approvers.bootstrap",
        "approvers.add_credential",
        "approvers.revoke_credential",
        "approvers.update_mapping",
        "artifact.list_kinds",
        "artifact.list_instances",
        "artifact.read",
        "artifact.validate",
        "artifact.intake_definition_read",
        "artifact.intake_evaluate",
        "artifact.intake_append",
        "artifact.candidate_validate",
        "artifact.candidate_append",
        "artifact.promote",
        "pipeline.list",
        "pipeline.show",
        "pipeline.resolve",
        "pipeline.compile_payload",
        "pipeline.compile_explain",
        "pipeline.capture_preview",
        "pipeline.capture_direct",
        "pipeline.capture_apply",
        "pipeline.handoff_emit",
        "pipeline.state_set",
        "flow.generate",
        "flow.inspect",
        "doctor",
    ]
    .into_iter()
    .collect::<BTreeSet<_>>();
    assert_eq!(leaves, expected);
    assert!(rows.iter().all(|row| row.machine_json));
    assert_eq!(rows.iter().filter(|row| row.legacy_json).count(), 19);

    let by_class = rows.iter().fold(BTreeMap::new(), |mut counts, row| {
        *counts.entry(row.result_class).or_insert(0usize) += 1;
        counts
    });
    assert_eq!(by_class.values().sum::<usize>(), 36);
    assert_eq!(
        rows.iter()
            .filter(|row| matches!(row.result_class, coverage::ResultClass::UseCase(_)))
            .count(),
        15
    );
}

#[test]
fn doctor_new_json_and_explicit_legacy_json_are_singular_and_mutually_exclusive() {
    let root = tempfile::tempdir().unwrap();
    let new_output = handbook()
        .current_dir(root.path())
        .args(["doctor", "--json"])
        .output()
        .unwrap();
    assert_eq!(new_output.status.code(), Some(1));
    assert!(new_output.stderr.is_empty());
    assert!(new_output.stdout.ends_with(b"\n"));
    assert!(!new_output.stdout.ends_with(b"\n\n"));
    let value: serde_json::Value = serde_json::from_slice(&new_output.stdout).unwrap();
    assert_eq!(value["schema"]["ref"], "handbook.invocation-response@1.0.0");
    assert_eq!(value["response_kind"], "use_case");
    assert_eq!(
        value["response"]["use_case"]["ref"],
        "handbook.use-case.repository-doctor@1.0.0"
    );
    assert_eq!(value["response"]["status"], "blocked");

    let legacy = handbook()
        .current_dir(root.path())
        .args(["doctor", "--legacy-json"])
        .output()
        .unwrap();
    assert_eq!(legacy.status.code(), new_output.status.code());
    let legacy_value: serde_json::Value = serde_json::from_slice(&legacy.stdout).unwrap();
    assert_eq!(
        legacy_value["schema_id"],
        "handbook.repository-doctor-report"
    );
    assert_eq!(legacy_value["schema_version"], "1.2.0");

    let conflict = handbook()
        .current_dir(root.path())
        .args(["doctor", "--json", "--legacy-json"])
        .output()
        .unwrap();
    assert_eq!(conflict.status.code(), Some(2));
}

#[test]
fn json_is_uniform_but_legacy_json_is_limited_to_existing_compatibility_commands() {
    assert_eq!(
        run(&["generate", "--json", "--packet", "planning.packet"])
            .status
            .code(),
        Some(1)
    );
    assert_eq!(
        run(&["generate", "--legacy-json", "--packet", "planning.packet",])
            .status
            .code(),
        Some(2)
    );

    let help = handbook().args(["doctor", "--help"]).output().unwrap();
    let help = String::from_utf8(help.stdout).unwrap();
    assert!(help.contains("--json"));
    assert!(help.contains("--legacy-json"));
    assert!(help.contains("handbook.invocation-response@1.0.0"));
}
