use crate::rendering::PreparedFlowOutput;
use std::process::ExitCode;

pub(crate) fn success() -> ExitCode {
    ExitDisposition::Success.exit_code()
}

pub(crate) fn failure() -> ExitCode {
    ExitDisposition::Failure.exit_code()
}

pub(crate) fn flow_output(output: &PreparedFlowOutput) -> ExitCode {
    if output.is_ready() {
        success()
    } else {
        failure()
    }
}

pub(crate) fn repository_status(status: handbook_sdk::RepositoryReadinessStatus) -> ExitCode {
    match status {
        handbook_sdk::RepositoryReadinessStatus::Ready => success(),
        handbook_sdk::RepositoryReadinessStatus::ActionRequired
        | handbook_sdk::RepositoryReadinessStatus::Indeterminate
        | handbook_sdk::RepositoryReadinessStatus::Invalid => failure(),
    }
}

enum ExitDisposition {
    Success,
    Failure,
}

impl ExitDisposition {
    fn exit_code(self) -> ExitCode {
        match self {
            Self::Success => ExitCode::SUCCESS,
            Self::Failure => ExitCode::from(1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success_and_failure_match_shell_contract() {
        assert_eq!(success(), ExitCode::SUCCESS);
        assert_eq!(failure(), ExitCode::from(1));
    }

    #[test]
    fn repository_readiness_exit_table_is_exhaustive() {
        use handbook_sdk::RepositoryReadinessStatus as Status;
        assert_eq!(repository_status(Status::Ready), ExitCode::SUCCESS);
        for status in [
            Status::ActionRequired,
            Status::Indeterminate,
            Status::Invalid,
        ] {
            assert_eq!(repository_status(status), ExitCode::from(1));
        }
    }
}
