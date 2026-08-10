use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RenderError {
    UnsupportedResultVersion {
        expected: &'static str,
        actual: String,
    },
    EmptyPacketId,
    EmptyDecisionLog,
    EmptyPacketBody,
}

impl Display for RenderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RenderError::UnsupportedResultVersion { expected, actual } => {
                write!(f, "presentation failure: unsupported result version (expected {expected}, got {actual})")
            }
            RenderError::EmptyPacketId => {
                write!(f, "presentation failure: empty packet id")
            }
            RenderError::EmptyDecisionLog => {
                write!(f, "presentation failure: empty decision log")
            }
            RenderError::EmptyPacketBody => {
                write!(f, "presentation failure: empty packet body")
            }
        }
    }
}

impl std::error::Error for RenderError {}
