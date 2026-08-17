use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PipelineMutationEffect {
    pub effect_kind: String,
    pub portable_path: String,
    pub fingerprint: String,
    pub revision: Option<u64>,
}

impl PipelineMutationEffect {
    pub(crate) fn from_bytes(
        effect_kind: &str,
        portable_path: &str,
        bytes: &[u8],
        revision: Option<u64>,
    ) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(bytes);
        Self {
            effect_kind: effect_kind.to_owned(),
            portable_path: portable_path.replace('\\', "/"),
            fingerprint: format!("sha256:{:x}", hasher.finalize()),
            revision,
        }
    }

    pub(crate) fn deletion(effect_kind: &str, portable_path: &str) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(b"handbook.pipeline-deletion-v1\0");
        Self {
            effect_kind: effect_kind.to_owned(),
            portable_path: portable_path.replace('\\', "/"),
            fingerprint: format!("sha256:{:x}", hasher.finalize()),
            revision: None,
        }
    }
}
