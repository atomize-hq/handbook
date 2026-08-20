use crate::DockErrorV1;
use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize};

pub const MAX_BUNDLE_FILES: usize = 256;
pub const MAX_SAFE_RELATIVE_PATH_BYTES: usize = 4_096;
pub const MAX_BUNDLE_FILE_BYTES: u64 = 64 * 1024 * 1024;
pub const MAX_BUNDLE_BYTES: u64 = 256 * 1024 * 1024;
pub const MAX_FIXED_ARGUMENTS: usize = 16;
pub const MAX_FIXED_ARGUMENT_BYTES: usize = 4_096;
pub const MAX_INLINE_INPUTS: usize = 128;
pub const MAX_INLINE_INPUT_BYTES: usize = 1024 * 1024;
pub const MAX_INLINE_INPUT_AGGREGATE_BYTES: usize = 8 * 1024 * 1024;
pub const MAX_PROCESS_JSON_BYTES: usize = 8 * 1024 * 1024;
pub const MAX_STDERR_BYTES: usize = 65_536;
pub const MAX_CANDIDATE_OBSERVATIONS: usize = 128;
pub const MAX_CANDIDATE_BYTES: usize = 512 * 1024;
pub const MAX_DIAGNOSTICS: usize = 128;
pub const MAX_DIAGNOSTIC_BYTES: usize = 4_096;
pub const MAX_WALL_TIMEOUT_MS: u64 = 300_000;
pub const MAX_CANCELLATION_GRACE_MS: u64 = 5_000;

#[derive(Clone, Debug)]
pub struct DockProtocolLimitInputV1 {
    pub request_bytes: usize,
    pub result_bytes: usize,
    pub stderr_bytes: usize,
    pub diagnostic_bytes: usize,
    pub candidate_bytes: usize,
    pub input_count: usize,
    pub candidate_count: usize,
    pub diagnostic_count: usize,
    pub wall_timeout_ms: u64,
    pub cancellation_grace_ms: u64,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DockProtocolLimitsV1 {
    request_bytes: usize,
    result_bytes: usize,
    stderr_bytes: usize,
    diagnostic_bytes: usize,
    candidate_bytes: usize,
    input_count: usize,
    candidate_count: usize,
    diagnostic_count: usize,
    wall_timeout_ms: u64,
    cancellation_grace_ms: u64,
}

impl DockProtocolLimitsV1 {
    pub fn new(input: DockProtocolLimitInputV1) -> Result<Self, DockErrorV1> {
        let positive = input.request_bytes > 0
            && input.result_bytes > 0
            && input.stderr_bytes > 0
            && input.diagnostic_bytes > 0
            && input.candidate_bytes > 0
            && input.input_count > 0
            && input.candidate_count > 0
            && input.diagnostic_count > 0
            && input.wall_timeout_ms > 0
            && input.cancellation_grace_ms > 0;
        let bounded = input.request_bytes <= MAX_PROCESS_JSON_BYTES
            && input.result_bytes <= MAX_PROCESS_JSON_BYTES
            && input.stderr_bytes <= MAX_STDERR_BYTES
            && input.diagnostic_bytes <= MAX_DIAGNOSTIC_BYTES
            && input.candidate_bytes <= MAX_CANDIDATE_BYTES
            && input.input_count <= MAX_INLINE_INPUTS
            && input.candidate_count <= MAX_CANDIDATE_OBSERVATIONS
            && input.diagnostic_count <= MAX_DIAGNOSTICS
            && input.wall_timeout_ms <= MAX_WALL_TIMEOUT_MS
            && input.cancellation_grace_ms <= MAX_CANCELLATION_GRACE_MS;
        if !positive || !bounded {
            return Err(DockErrorV1::BoundExceeded);
        }
        Ok(Self {
            request_bytes: input.request_bytes,
            result_bytes: input.result_bytes,
            stderr_bytes: input.stderr_bytes,
            diagnostic_bytes: input.diagnostic_bytes,
            candidate_bytes: input.candidate_bytes,
            input_count: input.input_count,
            candidate_count: input.candidate_count,
            diagnostic_count: input.diagnostic_count,
            wall_timeout_ms: input.wall_timeout_ms,
            cancellation_grace_ms: input.cancellation_grace_ms,
        })
    }

    pub fn checked_min(&self, other: &Self) -> Result<Self, DockErrorV1> {
        Self::new(DockProtocolLimitInputV1 {
            request_bytes: self.request_bytes.min(other.request_bytes),
            result_bytes: self.result_bytes.min(other.result_bytes),
            stderr_bytes: self.stderr_bytes.min(other.stderr_bytes),
            diagnostic_bytes: self.diagnostic_bytes.min(other.diagnostic_bytes),
            candidate_bytes: self.candidate_bytes.min(other.candidate_bytes),
            input_count: self.input_count.min(other.input_count),
            candidate_count: self.candidate_count.min(other.candidate_count),
            diagnostic_count: self.diagnostic_count.min(other.diagnostic_count),
            wall_timeout_ms: self.wall_timeout_ms.min(other.wall_timeout_ms),
            cancellation_grace_ms: self.cancellation_grace_ms.min(other.cancellation_grace_ms),
        })
    }

    pub fn request_bytes(&self) -> usize {
        self.request_bytes
    }
    pub fn result_bytes(&self) -> usize {
        self.result_bytes
    }
    pub fn stderr_bytes(&self) -> usize {
        self.stderr_bytes
    }
    pub fn diagnostic_bytes(&self) -> usize {
        self.diagnostic_bytes
    }
    pub fn candidate_bytes(&self) -> usize {
        self.candidate_bytes
    }
    pub fn input_count(&self) -> usize {
        self.input_count
    }
    pub fn candidate_count(&self) -> usize {
        self.candidate_count
    }
    pub fn diagnostic_count(&self) -> usize {
        self.diagnostic_count
    }
    pub fn wall_timeout_ms(&self) -> u64 {
        self.wall_timeout_ms
    }
    pub fn cancellation_grace_ms(&self) -> u64 {
        self.cancellation_grace_ms
    }
}

impl<'de> Deserialize<'de> for DockProtocolLimitsV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Raw {
            request_bytes: usize,
            result_bytes: usize,
            stderr_bytes: usize,
            diagnostic_bytes: usize,
            candidate_bytes: usize,
            input_count: usize,
            candidate_count: usize,
            diagnostic_count: usize,
            wall_timeout_ms: u64,
            cancellation_grace_ms: u64,
        }
        let raw = Raw::deserialize(deserializer)?;
        Self::new(DockProtocolLimitInputV1 {
            request_bytes: raw.request_bytes,
            result_bytes: raw.result_bytes,
            stderr_bytes: raw.stderr_bytes,
            diagnostic_bytes: raw.diagnostic_bytes,
            candidate_bytes: raw.candidate_bytes,
            input_count: raw.input_count,
            candidate_count: raw.candidate_count,
            diagnostic_count: raw.diagnostic_count,
            wall_timeout_ms: raw.wall_timeout_ms,
            cancellation_grace_ms: raw.cancellation_grace_ms,
        })
        .map_err(D::Error::custom)
    }
}
