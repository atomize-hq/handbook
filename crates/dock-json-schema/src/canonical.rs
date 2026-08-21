use crate::{CanonicalFingerprintV1, ValidatorRefusalV1};
use serde::Serialize;
use sha2::{Digest, Sha256};

pub(crate) fn canonical_bytes<T: Serialize>(value: &T) -> Result<Vec<u8>, ValidatorRefusalV1> {
    serde_json_canonicalizer::to_vec(value).map_err(|_| ValidatorRefusalV1::InternalRefusal)
}

pub(crate) fn canonical_fingerprint<T: Serialize>(
    value: &T,
) -> Result<CanonicalFingerprintV1, ValidatorRefusalV1> {
    let bytes = canonical_bytes(value)?;
    let digest: [u8; 32] = Sha256::digest(bytes).into();
    Ok(CanonicalFingerprintV1::from_digest(digest))
}
