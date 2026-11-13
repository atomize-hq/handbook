use crate::{ContractError, FingerprintV1};
use serde::Serialize;

pub(crate) fn canonical_bytes<T: Serialize>(value: &T) -> Result<Vec<u8>, ContractError> {
    serde_json_canonicalizer::to_vec(value).map_err(|_| ContractError::Canonicalization)
}

pub(crate) fn canonical_fingerprint<T: Serialize>(
    value: &T,
) -> Result<FingerprintV1, ContractError> {
    canonical_bytes(value).map(|bytes| FingerprintV1::from_bytes(&bytes))
}
