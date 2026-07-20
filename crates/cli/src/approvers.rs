use crate::{
    shell_shared::discover_managed_repo_root, ApproverAddCredentialArgs, ApproverBootstrapArgs,
    ApproverRevokeCredentialArgs, ApproverUpdateMappingArgs, ApproversArgs, ApproversCommand,
};
use std::fmt::Write as _;
use std::process::ExitCode;

pub(crate) fn run(args: ApproversArgs) -> ExitCode {
    let (intent, json) = match args.command {
        ApproversCommand::Bootstrap(ApproverBootstrapArgs {
            initial_charter_quorum,
            json,
        }) => (
            handbook_compiler::ApproverAdminIntent::Bootstrap {
                initial_charter_quorum,
            },
            json,
        ),
        ApproversCommand::AddCredential(ApproverAddCredentialArgs {
            approval_mappings,
            json,
        }) => (
            handbook_compiler::ApproverAdminIntent::AddCredential { approval_mappings },
            json,
        ),
        ApproversCommand::RevokeCredential(ApproverRevokeCredentialArgs {
            credential_id_hash,
            json,
        }) => (
            handbook_compiler::ApproverAdminIntent::RevokeCredential { credential_id_hash },
            json,
        ),
        ApproversCommand::UpdateMapping(ApproverUpdateMappingArgs {
            credential_id_hash,
            approval_mappings,
            json,
        }) => (
            handbook_compiler::ApproverAdminIntent::UpdateMapping {
                credential_id_hash,
                approval_mappings,
            },
            json,
        ),
    };
    let cwd = std::env::current_dir().unwrap_or_else(|_| ".".into());
    let repo_root = discover_managed_repo_root(&cwd);
    let result = handbook_compiler::execute_approver_admin_intent(&repo_root, intent);
    if json {
        let mut output = serde_json::to_string_pretty(&result)
            .unwrap_or_else(|_| "{\"status\":\"refused\"}".to_owned());
        output.push('\n');
        print!("{output}");
    } else {
        print!("{}", render_text(&result));
    }
    if result.status == handbook_compiler::AdapterOperationStatus::Succeeded {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}

fn render_text(result: &handbook_compiler::ApproverAdapterResult) -> String {
    let operation = match result.operation {
        handbook_compiler::ApproverAdminOperation::Bootstrap => "bootstrap",
        handbook_compiler::ApproverAdminOperation::AddCredential => "add_credential",
        handbook_compiler::ApproverAdminOperation::RevokeCredential => "revoke_credential",
        handbook_compiler::ApproverAdminOperation::UpdateMapping => "update_mapping",
    };
    let outcome = match result.status {
        handbook_compiler::AdapterOperationStatus::Succeeded => "SUCCEEDED",
        handbook_compiler::AdapterOperationStatus::Refused => "REFUSED",
    };
    let mut output = format!("OUTCOME: {outcome}\nOPERATION: {operation}\n");
    for path in &result.changed_paths {
        writeln!(&mut output, "CHANGED PATH: {path}").expect("string write");
    }
    if let Some(refusal) = &result.refusal {
        writeln!(&mut output, "CODE: {}", refusal.code).expect("string write");
        writeln!(&mut output, "MESSAGE: {}", refusal.message).expect("string write");
        writeln!(&mut output, "RETRYABLE: {}", refusal.retryable).expect("string write");
    }
    for action in &result.next_actions {
        writeln!(&mut output, "NEXT SAFE ACTION: {action}").expect("string write");
    }
    output
}
