use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize};

/// Exact validator-local structural ceilings selected by HCM-5.4.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub struct ValidatorLimitsV1 {
    max_documents: u16,
    max_path_bytes: u16,
    max_path_components: u8,
    max_ref_depth: u8,
    max_violations: u16,
    max_pointer_bytes: u16,
}

impl ValidatorLimitsV1 {
    /// Returns the one closed V1 limit set.
    pub const fn v1() -> Self {
        Self {
            max_documents: 128,
            max_path_bytes: 1_024,
            max_path_components: 64,
            max_ref_depth: 32,
            max_violations: 128,
            max_pointer_bytes: 1_024,
        }
    }

    pub const fn max_documents(self) -> usize {
        self.max_documents as usize
    }
    pub const fn max_path_bytes(self) -> usize {
        self.max_path_bytes as usize
    }
    pub const fn max_path_components(self) -> usize {
        self.max_path_components as usize
    }
    pub const fn max_ref_depth(self) -> usize {
        self.max_ref_depth as usize
    }
    pub const fn max_violations(self) -> usize {
        self.max_violations as usize
    }
    pub const fn max_pointer_bytes(self) -> usize {
        self.max_pointer_bytes as usize
    }
}

impl<'de> Deserialize<'de> for ValidatorLimitsV1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Raw {
            max_documents: u16,
            max_path_bytes: u16,
            max_path_components: u8,
            max_ref_depth: u8,
            max_violations: u16,
            max_pointer_bytes: u16,
        }
        let raw = Raw::deserialize(deserializer)?;
        let supplied = Self {
            max_documents: raw.max_documents,
            max_path_bytes: raw.max_path_bytes,
            max_path_components: raw.max_path_components,
            max_ref_depth: raw.max_ref_depth,
            max_violations: raw.max_violations,
            max_pointer_bytes: raw.max_pointer_bytes,
        };
        if supplied != Self::v1() {
            return Err(D::Error::custom(
                "validator limits are not the closed V1 set",
            ));
        }
        Ok(supplied)
    }
}
