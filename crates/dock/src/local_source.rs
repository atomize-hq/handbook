//! Real completed Dock sources under explicitly selected local advisory terms.
use crate::store::map_candidate_store_error;
use crate::{DockManifestReadOutcomeV1, DockManifestRegistryV1, DockOperationalStoreV1};
use handbook_contracts::project_admission_governance::ProjectAdmissionBindingV3;
use handbook_contracts::{
    resolve_repository_local_candidate_observation_v2, ContentAddressedBindingV1,
    ContractEvaluationRunV1, ContractOwnerRepositoryV1, EvidenceCandidateSourceErrorKindV1,
    EvidenceCandidateSourceErrorV1, EvidenceCandidateSourceV1, ExactBindingV1,
    LocalContractUseIntentV2, ResolvedEvidenceCandidateObservationInputV1,
    ResolvedEvidenceCandidateObservationV1, SemanticTimeV1,
};
use handbook_engine::local_authority::local_check::{
    LocalCheckActionRequestV1, LocalCheckActionV1, LocalCheckLocatorV1, NativeLocalCheckAuthorityV1,
};
use std::path::{Path, PathBuf};

pub struct DockLocalEvidenceCandidateSourceV2 {
    repo_root: PathBuf,
    store: DockOperationalStoreV1,
    locator: LocalCheckLocatorV1,
    project: ProjectAdmissionBindingV3,
    definition: ExactBindingV1,
    intent: LocalContractUseIntentV2,
}

impl DockLocalEvidenceCandidateSourceV2 {
    pub fn open(
        repo_root: impl AsRef<Path>,
        locator: LocalCheckLocatorV1,
        project: ProjectAdmissionBindingV3,
        definition: ExactBindingV1,
        intent: LocalContractUseIntentV2,
    ) -> Result<Self, EvidenceCandidateSourceErrorV1> {
        Ok(Self {
            repo_root: repo_root.as_ref().to_path_buf(),
            store: DockOperationalStoreV1::open_read_only(repo_root)
                .map_err(map_candidate_store_error)?,
            locator,
            project,
            definition,
            intent,
        })
    }
}

impl EvidenceCandidateSourceV1 for DockLocalEvidenceCandidateSourceV2 {
    fn resolve_exact<R>(
        &mut self,
        candidate: &ContentAddressedBindingV1,
        consume: impl for<'authority> FnOnce(ResolvedEvidenceCandidateObservationV1<'authority>) -> R,
    ) -> Result<R, EvidenceCandidateSourceErrorV1> {
        let retained = self
            .store
            .read_local_candidate_v2(candidate)
            .map_err(map_candidate_store_error)?;
        let owner = ContractOwnerRepositoryV1::open_existing_for_local_use_v2(&self.repo_root)
            .map_err(|_| refuse(EvidenceCandidateSourceErrorKindV1::Unavailable))?;
        // Governance qualification finishes before native Engine authority or
        // Contracts consuming locks are acquired. Each resolution does it anew.
        let local_use = owner
            .prepare_local_contract_use_v2(&self.project, &self.definition, self.intent)
            .map_err(|_| refuse(EvidenceCandidateSourceErrorKindV1::StaleAuthority))?;
        let run: ContractEvaluationRunV1 =
            serde_json::from_str(&retained.stored.evaluation_run_jcs)
                .map_err(|_| refuse(EvidenceCandidateSourceErrorKindV1::Malformed))?;
        let dependencies = local_use.definition().dependencies();
        if !run.is_local_source_seam_v2()
            || run.contract() != self.definition
            || local_use.definition().binding() != self.definition
            || run.claims() != local_use.definition().semantic_definition().claims()
            || run.freshness_policies()
                != dependencies
                    .freshness_policies
                    .iter()
                    .map(|p| p.exact_binding())
                    .collect::<Vec<_>>()
            || run.stability_policies() != [dependencies.stability_policy.clone()]
            || run.gate_policy() != &dependencies.gate_policy.exact_binding()
            || retained.request.manifest() != &dependencies.validator.manifest
        {
            return Err(refuse(
                EvidenceCandidateSourceErrorKindV1::CandidateSubstituted,
            ));
        }
        let production = match DockManifestRegistryV1::production()
            .read(
                dependencies.validator.manifest.reference(),
                dependencies.validator.manifest.fingerprint(),
            )
            .map_err(|_| refuse(EvidenceCandidateSourceErrorKindV1::CandidateSubstituted))?
        {
            DockManifestReadOutcomeV1::Found(manifest) => manifest,
            DockManifestReadOutcomeV1::Absent => {
                return Err(refuse(EvidenceCandidateSourceErrorKindV1::Missing))
            }
        };
        if serde_json::to_value(&production)
            .map_err(|_| refuse(EvidenceCandidateSourceErrorKindV1::Malformed))?
            != dependencies.validator.canonical_manifest
        {
            return Err(refuse(
                EvidenceCandidateSourceErrorKindV1::CandidateSubstituted,
            ));
        }
        let input = retained
            .request
            .inputs()
            .iter()
            .find(|input| input.role().as_str() == "handbook.input.json-schema-validation")
            .ok_or_else(|| refuse(EvidenceCandidateSourceErrorKindV1::Missing))?;
        if run.subject()
            != &input
                .captured_evaluation_subject_v1()
                .map_err(|_| refuse(EvidenceCandidateSourceErrorKindV1::Malformed))?
            || run.cases().len() != 1
            || run.cases()[0]
                != input
                    .captured_evaluation_case_v1(run.cases()[0].case_id().clone())
                    .map_err(|_| refuse(EvidenceCandidateSourceErrorKindV1::Malformed))?
        {
            return Err(refuse(
                EvidenceCandidateSourceErrorKindV1::CandidateSubstituted,
            ));
        }
        let closure = serde_json::json!({
            "root_schema": input.content().get("root_schema"),
            "root_document": input.content().get("root_document"),
            "documents": input.content().get("documents"),
            "closure_fingerprint": input.content().get("closure_fingerprint"),
        });
        if closure
            != serde_json::to_value(&dependencies.schema_closure)
                .map_err(|_| refuse(EvidenceCandidateSourceErrorKindV1::Malformed))?
        {
            return Err(refuse(
                EvidenceCandidateSourceErrorKindV1::CandidateSubstituted,
            ));
        }
        let candidate_value: serde_json::Value =
            serde_json::from_str(&retained.stored.candidate_jcs)
                .map_err(|_| refuse(EvidenceCandidateSourceErrorKindV1::Malformed))?;
        if candidate_value.pointer("/body/fact_schema")
            != Some(
                &serde_json::to_value(&dependencies.fact_schema.binding)
                    .map_err(|_| refuse(EvidenceCandidateSourceErrorKindV1::Malformed))?,
            )
        {
            return Err(refuse(
                EvidenceCandidateSourceErrorKindV1::CandidateSubstituted,
            ));
        }
        let now = SemanticTimeV1::owner_observation_now_v1()
            .map_err(|_| refuse(EvidenceCandidateSourceErrorKindV1::Unavailable))?;
        let mut input_paths = retained.exact_paths.clone();
        input_paths.extend(local_use.required_state_paths().iter().cloned());
        input_paths.sort();
        input_paths.dedup();
        let request = LocalCheckActionRequestV1 {
            action: LocalCheckActionV1::EvidenceSource,
            action_ref: "handbook.contract-evidence-source.local@2.0.0".into(),
            subject_ref: candidate.reference().into(),
            content_fingerprint: candidate.fingerprint().as_str().into(),
            closure_fingerprint: local_use
                .applicability()
                .policy_admission_closure_fingerprint
                .as_str()
                .into(),
            actor_ref: self.locator.terms.agent.clone(),
            participant_ref: self.locator.terms.author_participant_ref.clone(),
            semantic_time_utc: now.as_str().into(),
            owner_write_paths: vec![".handbook/state/contracts/v1/evidence-v1".into()],
            input_paths: input_paths.clone(),
        };
        NativeLocalCheckAuthorityV1::with_admission(
            &self.repo_root,
            &self.locator,
            local_use.current_basis(),
            &request,
            |admission| {
                // Pin actual immutable captures through the consumer. The Dock
                // lock is released so its read-only observer may run repeatedly.
                let current_use = owner
                    .validate_local_contract_use_current_v2(&local_use)
                    .map_err(|_| refuse(EvidenceCandidateSourceErrorKindV1::StaleAuthority));
                let checkpoint = current_use.and_then(|_| {
                    self.store
                        .repository_root
                        .retain_exact_files_checkpoint_v1(
                            &input_paths.iter().map(String::as_str).collect::<Vec<_>>(),
                        )
                        .map_err(|_| refuse(EvidenceCandidateSourceErrorKindV1::Unavailable))
                });
                let checked = checkpoint.and_then(|checkpoint| {
                    owner
                        .validate_local_contract_use_current_v2(&local_use)
                        .map_err(|_| refuse(EvidenceCandidateSourceErrorKindV1::StaleAuthority))?;
                    let fresh = self
                        .store
                        .read_local_candidate_v2(candidate)
                        .map_err(map_candidate_store_error)?;
                    if fresh.stored.candidate_jcs != retained.stored.candidate_jcs
                        || fresh.stored.evaluation_run_jcs != retained.stored.evaluation_run_jcs
                        || fresh.request != retained.request
                        || fresh.exact_paths != retained.exact_paths
                    {
                        return Err(refuse(
                            EvidenceCandidateSourceErrorKindV1::CandidateSubstituted,
                        ));
                    }
                    resolve_repository_local_candidate_observation_v2(
                        &self.store.repository_root,
                        &owner,
                        admission,
                        &local_use,
                        &checkpoint,
                        candidate,
                        ResolvedEvidenceCandidateObservationInputV1 {
                            candidate_jcs: retained.stored.candidate_jcs.clone().into_bytes(),
                            evaluation_run_jcs: retained
                                .stored
                                .evaluation_run_jcs
                                .clone()
                                .into_bytes(),
                            producer: retained.stored.producer.clone(),
                            request_record: retained.stored.request_record.clone(),
                            execution_record: retained.stored.execution_record.clone(),
                            producer_resolution_ceiling: retained
                                .stored
                                .producer_resolution_ceiling
                                .clone(),
                        },
                        consume,
                    )
                });
                Ok(checked)
            },
        )
        .map_err(|_| refuse(EvidenceCandidateSourceErrorKindV1::StaleAuthority))?
    }
}

fn refuse(kind: EvidenceCandidateSourceErrorKindV1) -> EvidenceCandidateSourceErrorV1 {
    EvidenceCandidateSourceErrorV1::repository_refusal(kind)
}
