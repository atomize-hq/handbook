use crate::{
    exit_policy, rendering, request_shared, shell_shared::discover_managed_repo_root, RequestArgs,
};
use std::process::ExitCode;

pub(super) fn run(args: RequestArgs) -> ExitCode {
    let cwd = match std::env::current_dir() {
        Ok(dir) => dir,
        Err(err) => {
            println!("BLOCKED: failed to determine repo root: {err}");
            return exit_policy::failure();
        }
    };
    let repo_root = discover_managed_repo_root(&cwd);
    let request = match request_shared::prepare_request(&args, &repo_root) {
        Ok(request) => request,
        Err(err) => {
            println!("BLOCKED: {err}");
            return exit_policy::failure();
        }
    };

    let result = match handbook_sdk::HandbookSdkV1::open(&request.compiler_root).inspect_packet(
        handbook_sdk::flow_api::InspectPacketRequest::new(request.packet_id.as_str()),
    ) {
        Ok(result) => result,
        Err(err) => {
            println!("BLOCKED: resolver error: {err:?}");
            return exit_policy::failure();
        }
    };

    let output = match rendering::prepare_flow_output(result.into_resolution()) {
        Ok(output) => output,
        Err(err) => {
            println!("PRESENTATION FAILURE: {err}");
            return exit_policy::failure();
        }
    };

    println!("{}", output.render_inspect());

    exit_policy::flow_output(&output)
}
