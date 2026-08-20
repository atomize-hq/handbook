use crate::{DockErrorV1, DockEvaluationRunBindingInputV1, DockEvaluationRunBindingV1};
use handbook_contracts::{
    ContentAddressedBindingV1, ContractEvaluationRunV1, ContractOwnerRepositoryV1,
    EvaluationPolicyRegistryV1,
};

/// A complete Contracts run that Dock has independently re-admitted against
/// current owner repository and policy truth.
#[derive(Clone, Debug)]
pub struct DockAdmittedEvaluationRunV1 {
    run: ContractEvaluationRunV1,
    binding: DockEvaluationRunBindingV1,
    canonical_json: Vec<u8>,
}

impl DockAdmittedEvaluationRunV1 {
    pub fn run(&self) -> &ContractEvaluationRunV1 {
        &self.run
    }

    pub fn binding(&self) -> &DockEvaluationRunBindingV1 {
        &self.binding
    }

    pub fn canonical_json(&self) -> &[u8] {
        &self.canonical_json
    }
}

pub fn readmit_current_evaluation_run_v1(
    owner: &ContractOwnerRepositoryV1,
    run: &ContractEvaluationRunV1,
    policies: &EvaluationPolicyRegistryV1,
) -> Result<DockAdmittedEvaluationRunV1, DockErrorV1> {
    let canonical_json = run
        .canonical_json_bytes()
        .map_err(|_| DockErrorV1::Canonicalization)?;
    let reparsed: ContractEvaluationRunV1 =
        serde_json::from_slice(&canonical_json).map_err(|_| DockErrorV1::Canonicalization)?;
    if &reparsed != run
        || reparsed
            .canonical_json_bytes()
            .map_err(|_| DockErrorV1::Canonicalization)?
            != canonical_json
    {
        return Err(DockErrorV1::Substituted);
    }
    owner
        .readmit_evaluation_run_v1(&reparsed, policies)
        .map_err(|_| DockErrorV1::Substituted)?;
    let binding = DockEvaluationRunBindingV1::new(DockEvaluationRunBindingInputV1 {
        evaluation_run: ContentAddressedBindingV1::new(
            reparsed.evaluation_run_ref(),
            reparsed.evaluation_run_fingerprint().clone(),
        )
        .map_err(|_| DockErrorV1::InvalidIdentity)?,
        evaluation_lifecycle_basis: reparsed.evaluation_lifecycle_basis().clone(),
        contract: reparsed.contract(),
        claim_ids: reparsed
            .claims()
            .iter()
            .map(|claim| claim.claim_id().clone())
            .collect(),
        subject: reparsed.subject().clone(),
        cases: reparsed.cases().to_vec(),
        requested_resolution: reparsed.requested_resolution().clone(),
        semantic_time_utc: reparsed.owner_currentness().semantic_time_utc().clone(),
        lifecycle_head_fingerprint: reparsed
            .owner_currentness()
            .lifecycle_head_fingerprint()
            .clone(),
    })?;
    Ok(DockAdmittedEvaluationRunV1 {
        run: reparsed,
        binding,
        canonical_json,
    })
}
