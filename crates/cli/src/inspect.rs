use crate::{rendering, request_shared, shell_shared::discover_managed_repo_root, RequestArgs};
use std::process::ExitCode;

pub(super) fn run(args: RequestArgs) -> ExitCode {
    let cwd = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(err) => {
            println!("BLOCKED: failed to determine repo root: {err}");
            return ExitCode::from(1);
        }
    };
    let repo_root = discover_managed_repo_root(&cwd);
    let request = match request_shared::prepare_request(&args, &repo_root) {
        Ok(request) => request,
        Err(err) => {
            println!("BLOCKED: {err}");
            return ExitCode::from(1);
        }
    };

    let result = match handbook_flow::resolve(
        &request.compiler_root,
        handbook_flow::ResolveRequest {
            packet_id: request.packet_id.as_str(),
            ..handbook_flow::ResolveRequest::default()
        },
    ) {
        Ok(result) => result,
        Err(err) => {
            println!("BLOCKED: resolver error: {err:?}");
            return ExitCode::from(1);
        }
    };

    let output = match rendering::prepare_flow_output(result) {
        Ok(output) => output,
        Err(err) => {
            println!("PRESENTATION FAILURE: {err}");
            return ExitCode::from(1);
        }
    };

    println!("{}", output.render_inspect());

    output.exit_code()
}
