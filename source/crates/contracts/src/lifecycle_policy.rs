use crate::canonical::canonical_fingerprint;
use crate::{ContractError, ExactBindingV1, ExactRefV1};
use serde::{Deserialize, Serialize};

pub const LIFECYCLE_POLICY_REF_V1: &str =
    "handbook.contract-lifecycle-policy.independent-lock@1.0.0";

const POLICY_SOURCE: &str =
    include_str!("../policies/contract-lifecycle-policy-independent-lock-1.0.0.json");

/// The only four approval classes selected by lifecycle policy V1.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ContractApprovalClassV1 {
    DefinitionAuthor,
    IndependentLock,
    Activation,
    Lifecycle,
}

impl ContractApprovalClassV1 {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DefinitionAuthor => "contract_definition_author",
            Self::IndependentLock => "contract_independent_lock",
            Self::Activation => "contract_activation",
            Self::Lifecycle => "contract_lifecycle",
        }
    }
}

pub fn lifecycle_policy_canonical_json_v1() -> &'static str {
    POLICY_SOURCE
        .strip_suffix('\n')
        .expect("checked-in lifecycle policy ends in one LF")
}

pub fn lifecycle_policy_binding_v1() -> Result<ExactBindingV1, ContractError> {
    let source = lifecycle_policy_canonical_json_v1();
    let value: serde_json::Value =
        serde_json::from_str(source).map_err(|_| ContractError::Canonicalization)?;
    if serde_json_canonicalizer::to_string(&value).map_err(|_| ContractError::Canonicalization)?
        != source
    {
        return Err(ContractError::Canonicalization);
    }
    Ok(ExactBindingV1::new(
        ExactRefV1::parse(LIFECYCLE_POLICY_REF_V1)?,
        canonical_fingerprint(&value)?,
    ))
}
