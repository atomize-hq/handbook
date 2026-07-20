#![cfg(unix)]

use handbook_engine::REPOSITORY_IDENTITY_REPO_PATH;
use std::fs;
use std::process::{Command, Output};
use tempfile::tempdir;

#[test]
fn setup_reports_typed_repository_identity_refusal_without_repair() {
    let repo = tempdir().unwrap();
    fs::create_dir(repo.path().join(".handbook")).unwrap();
    let identity_path = repo.path().join(REPOSITORY_IDENTITY_REPO_PATH);
    let malformed = b"sha256:AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";
    fs::write(&identity_path, malformed).unwrap();

    let output = run_in(repo.path(), &["setup"]);
    assert!(!output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("CATEGORY: repository_identity"), "{stdout}");
    assert!(
        stdout.contains("REASON: repository_identity_unsafe"),
        "{stdout}"
    );
    assert!(
        stdout.contains("SUBJECT: .handbook/repository-identity.v1"),
        "{stdout}"
    );
    assert_eq!(fs::read(identity_path).unwrap(), malformed);
}

#[test]
fn setup_reset_state_preserves_exact_repository_identity_bytes() {
    let repo = tempdir().unwrap();
    let first = run_in(repo.path(), &["setup"]);
    let first_stdout = String::from_utf8(first.stdout).unwrap();
    assert!(!first_stdout.contains("OUTCOME: ERROR"), "{first_stdout}");
    let identity_path = repo.path().join(REPOSITORY_IDENTITY_REPO_PATH);
    let before = fs::read(&identity_path).unwrap();
    fs::create_dir_all(repo.path().join(".handbook/state/runtime")).unwrap();
    fs::write(repo.path().join(".handbook/state/runtime/stale"), b"stale").unwrap();

    let reset = run_in(repo.path(), &["setup", "refresh", "--reset-state"]);
    let reset_stdout = String::from_utf8(reset.stdout).unwrap();
    assert!(!reset_stdout.contains("OUTCOME: ERROR"), "{reset_stdout}");
    assert_eq!(fs::read(identity_path).unwrap(), before);
}

fn run_in(root: &std::path::Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_handbook"))
        .current_dir(root)
        .args(args)
        .output()
        .unwrap()
}
