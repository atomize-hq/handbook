use crate::{doctor_rendering, exit_policy, shell_shared::discover_managed_repo_root, DoctorArgs};
use std::process::ExitCode;

pub(super) fn run(args: DoctorArgs) -> ExitCode {
    let cwd = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(_) => {
            println!("OUTCOME: ERROR\nCATEGORY: repository_root_unavailable");
            return exit_policy::failure();
        }
    };
    let repo_root = discover_managed_repo_root(&cwd);
    let report = match handbook_sdk::HandbookSdkV1::open(&repo_root).doctor() {
        Ok(report) => report,
        Err(error) => {
            println!("OUTCOME: ERROR");
            println!("CATEGORY: {}", doctor_error_kind_name(error.kind()));
            println!("REASON: {}", doctor_error_reason_name(error.reason_code()));
            return exit_policy::failure();
        }
    };

    if args.json {
        match doctor_rendering::render_json(&report) {
            Ok(json) => print!("{json}"),
            Err(error) => {
                println!("OUTCOME: ERROR\nCATEGORY: serialization\nREASON: {error}");
                return exit_policy::failure();
            }
        }
    } else {
        print!("{}", doctor_rendering::render_text(&report));
    }
    exit_policy::repository_status(report.status)
}

fn doctor_error_kind_name(kind: handbook_sdk::DoctorErrorKind) -> &'static str {
    match kind {
        handbook_sdk::DoctorErrorKind::ProfileResolution => "profile_resolution",
        handbook_sdk::DoctorErrorKind::ProfileDecision => "profile_decision",
    }
}

fn doctor_error_reason_name(reason: handbook_sdk::DoctorErrorReasonCode) -> &'static str {
    match reason {
        handbook_sdk::DoctorErrorReasonCode::ShippedProfileUnavailable => {
            "shipped_profile_unavailable"
        }
        handbook_sdk::DoctorErrorReasonCode::SelectedProfileDecisionInvalid => {
            "selected_profile_decision_invalid"
        }
    }
}
