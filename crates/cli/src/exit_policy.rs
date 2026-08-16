use crate::rendering::PreparedFlowOutput;
use std::io::{self, Write};
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

pub(crate) fn terminal(status: handbook_sdk::contract_membrane::TerminalStatus) -> ExitCode {
    match status {
        handbook_sdk::contract_membrane::TerminalStatus::Ok => success(),
        handbook_sdk::contract_membrane::TerminalStatus::Blocked
        | handbook_sdk::contract_membrane::TerminalStatus::Refused
        | handbook_sdk::contract_membrane::TerminalStatus::Error => failure(),
    }
}

pub(crate) fn emit_stdout(bytes: &[u8], semantic_status: ExitCode) -> ExitCode {
    let stdout = io::stdout();
    let mut lock = stdout.lock();
    match emit_to(&mut lock, bytes, semantic_status) {
        Ok(status) => status,
        Err(()) => {
            eprintln!("handbook: stdout emission failed");
            failure()
        }
    }
}

fn emit_to(
    writer: &mut dyn Write,
    bytes: &[u8],
    semantic_status: ExitCode,
) -> Result<ExitCode, ()> {
    match writer.write(bytes) {
        Ok(written) if written == bytes.len() => {
            writer.flush().map(|_| semantic_status).map_err(|_| ())
        }
        Ok(_) | Err(_) => Err(()),
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

    #[test]
    fn terminal_table_and_physical_write_failure_are_shared_and_closed() {
        use handbook_sdk::contract_membrane::TerminalStatus;
        assert_eq!(terminal(TerminalStatus::Ok), ExitCode::SUCCESS);
        for status in [
            TerminalStatus::Blocked,
            TerminalStatus::Refused,
            TerminalStatus::Error,
        ] {
            assert_eq!(terminal(status), ExitCode::from(1));
        }

        struct Broken;
        impl Write for Broken {
            fn write(&mut self, _buffer: &[u8]) -> io::Result<usize> {
                Err(io::Error::new(io::ErrorKind::BrokenPipe, "bounded test"))
            }

            fn flush(&mut self) -> io::Result<()> {
                Ok(())
            }
        }
        assert_eq!(emit_to(&mut Broken, b"{}\n", ExitCode::SUCCESS), Err(()));

        struct Partial {
            writes: usize,
        }
        impl Write for Partial {
            fn write(&mut self, _buffer: &[u8]) -> io::Result<usize> {
                self.writes += 1;
                Ok(1)
            }

            fn flush(&mut self) -> io::Result<()> {
                Ok(())
            }
        }
        let mut partial = Partial { writes: 0 };
        assert_eq!(emit_to(&mut partial, b"{}\n", ExitCode::SUCCESS), Err(()));
        assert_eq!(partial.writes, 1, "partial writes must never be retried");
    }
}
