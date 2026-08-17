use crate::{flow_command, RequestArgs};
use std::process::ExitCode;

pub(super) fn run(args: RequestArgs, machine_json: bool) -> ExitCode {
    flow_command::run(args, machine_json, flow_command::FlowRenderer::Generate)
}
