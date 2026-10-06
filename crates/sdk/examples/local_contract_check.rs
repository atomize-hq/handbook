//! Ordinary typed Packet 2 consumer. All project choices, terms and review
//! outcomes are supplied. Source resolution does not complete Packet 3's run
//! authorization, Coordination, Purpose or explained-truth closure.
use handbook_contracts::*;
use handbook_dock::*;
use handbook_sdk::local_check::*;
use handbook_sdk::local_setup::{
    LocalInitialSetupLocatorV1, LocalInitialSetupRequestV1, LocalRevokeRequestV1,
};
use handbook_sdk::HandbookSdkV1;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;

fn main() {
    // Supply an explicit bounded worker stack for this Windows debug host.
    // The ordinary operations and native owners use their normal dependencies.
    let result = std::thread::Builder::new()
        .name("local-contract-consumer".into())
        .stack_size(8 * 1024 * 1024)
        .spawn(run)
        .expect("start local consumer")
        .join()
        .expect("local consumer thread completed");
    if let Err(error) = result {
        eprintln!("{error}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), String> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    if args.len() < 3 {
        return Err("usage: local_contract_check initial ROOT REQUEST RECOVERY | adopt ROOT INITIAL TERMS RECOVERY | author ROOT SUCCESSOR INITIAL DEFINITION KEY | transition ROOT SUCCESSOR INITIAL BINDING INPUT ACTOR PARTICIPANT KEY | use ROOT INITIAL BINDING INTENT | produce ROOT INITIAL BINDING VALIDATION_CONTENT NONCE KEY | resolve-source/admit ROOT SUCCESSOR INITIAL BINDING CANDIDATE INTENT | append ROOT SUCCESSOR INITIAL BINDING CANDIDATE INTENT KEY | verdict ROOT SUCCESSOR INITIAL BINDING RUN INTENT | establish-g ROOT REQUEST | history/recover ROOT SUCCESSOR | current ROOT SUCCESSOR INITIAL | revoke ROOT SUCCESSOR REQUEST".into());
    }
    let sdk = HandbookSdkV1::open(&args[1]);
    if args[0] == "initial" && args.len() == 4 {
        let request: LocalInitialSetupRequestV1 = decode(&read(&args[2])?)?;
        let prepared = sdk.prepare_local_initial_setup(&request).map_err(debug)?;
        save_new(
            Path::new(&args[3]),
            &prepared.locator().canonical_json_bytes().map_err(debug)?,
        )?;
        return print_json(&sdk.establish_local_initial_setup(prepared));
    }
    if args[0] == "establish-g" && args.len() == 3 {
        let request: handbook_sdk::admission_governance::DeferredEstablishmentRequestV3 =
            decode(&read(&args[2])?)?;
        let prepared = sdk
            .prepare_deferred_project_admission_governance(&request)
            .map_err(debug)?;
        return print_json(&sdk.establish_deferred_project_admission_governance(prepared));
    }
    if args[0] == "adopt" && args.len() == 5 {
        let initial =
            LocalInitialSetupLocatorV1::from_json_bytes(&read(&args[2])?).map_err(debug)?;
        let terms = LocalCheckTermsV1::from_json_bytes(&read(&args[3])?).map_err(debug)?;
        let prepared = sdk
            .prepare_local_check_terms(&initial, terms)
            .map_err(debug)?;
        save_new(
            Path::new(&args[4]),
            &prepared.locator().canonical_json_bytes().map_err(debug)?,
        )?;
        return print_json(&sdk.adopt_local_check_terms(prepared));
    }
    if args[0] == "use" && args.len() == 5 {
        let initial =
            LocalInitialSetupLocatorV1::from_json_bytes(&read(&args[2])?).map_err(debug)?;
        let binding: ExactBindingV1 = decode(&read(&args[3])?)?;
        let use_basis = sdk
            .prepare_local_contract_use(&initial, &binding, intent(&args[4])?)
            .map_err(debug)?;
        return print_json(use_basis.applicability());
    }
    if args[0] == "produce" && args.len() == 7 {
        let initial =
            LocalInitialSetupLocatorV1::from_json_bytes(&read(&args[2])?).map_err(debug)?;
        let binding: ExactBindingV1 = decode(&read(&args[3])?)?;
        let usable = sdk
            .prepare_local_contract_use(&initial, &binding, LocalContractUseIntentV2::Diagnostic)
            .map_err(debug)?;
        let bundle = DockResolvedFirstPartyBundleV1::resolve_production().map_err(debug)?;
        let manifest = bundle.manifest();
        let role = &manifest.supported_inputs()[0];
        let capture = DockInlineJsonInputV1::new(
            role.role().clone(),
            role.media_type(),
            role.schema().clone(),
            decode(&read(&args[4])?)?,
        )
        .map_err(debug)?;
        let dependencies = usable.definition().dependencies();
        // This owner-native run supports the Packet 2 source seam only. It
        // deliberately carries no later Evaluation Authorization closure.
        let run = sdk
            .prepare_local_source_seam_run(
                PrepareEvaluationRunInputV1::new(
                    binding,
                    usable.lifecycle_head_fingerprint().clone(),
                    capture.captured_evaluation_subject_v1().map_err(debug)?,
                    vec![capture
                        .captured_evaluation_case_v1(
                            StableIdV1::parse("case.schema-conformance").map_err(debug)?,
                        )
                        .map_err(debug)?],
                    manifest.resolution_ceiling().clone(),
                    MatcherBindingV1::exact_fields_v1(),
                    dependencies
                        .freshness_policies
                        .iter()
                        .map(|p| p.exact_binding())
                        .collect(),
                    vec![dependencies.stability_policy.clone()],
                    dependencies.gate_policy.exact_binding(),
                    local_closeout_policy_binding_v1().map_err(debug)?,
                    EvaluationRunNonceV1::parse(&args[5]).map_err(debug)?,
                )
                .map_err(debug)?,
            )
            .map_err(debug)?;
        let request = DockProcessRequestV1::new(
            manifest,
            DockProcessRequestInputV1 {
                run: DockEvaluationRunBindingV1::new(DockEvaluationRunBindingInputV1 {
                    evaluation_run: ContentAddressedBindingV1::new(
                        run.evaluation_run_ref(),
                        run.evaluation_run_fingerprint().clone(),
                    )
                    .map_err(debug)?,
                    evaluation_lifecycle_basis: run.evaluation_lifecycle_basis().clone(),
                    contract: run.contract(),
                    claim_ids: run.claims().iter().map(|c| c.claim_id().clone()).collect(),
                    subject: run.subject().clone(),
                    cases: run.cases().to_vec(),
                    requested_resolution: run.requested_resolution().clone(),
                    semantic_time_utc: run.owner_currentness().semantic_time_utc().clone(),
                    lifecycle_head_fingerprint: run
                        .owner_currentness()
                        .lifecycle_head_fingerprint()
                        .clone(),
                })
                .map_err(debug)?,
                requested_evidence_kinds: vec![StableIdV1::parse(
                    "handbook.evidence.schema-conformance",
                )
                .map_err(debug)?],
                inputs: vec![capture],
                effective_limits: manifest.limits().clone(),
            },
        )
        .map_err(debug)?;
        let store = DockOperationalStoreV1::open(&args[1]).map_err(debug)?;
        let receipt = DockProcessExecutorV1::new()
            .execute(DockExecutionInputV1 {
                store: &store,
                resolved_bundle: &bundle,
                request: &request,
                evaluation_run_jcs: &run.canonical_json_bytes().map_err(debug)?,
                idempotency_key: &args[6],
                cancellation: &DockCancellationV1::new(),
            })
            .map_err(debug)?;
        if receipt.execution_record().host_outcome() != DockHostOutcomeV1::Completed
            || receipt.candidates().is_empty()
        {
            return Err(format!(
                "native production {:?}; no usable candidate; retained execution {}",
                receipt.execution_record().host_outcome(),
                receipt.execution_record().record_ref()
            ));
        }
        return print_json(
            &serde_json::json!({"execution": receipt.execution_record(), "candidates": receipt.candidates(), "source_seam_run": run}),
        );
    }
    let locator = LocalCheckLocatorV1::from_json_bytes(&read(&args[2])?).map_err(debug)?;
    match args[0].as_str() {
        "history" if args.len() == 3 => {
            print_json(&sdk.resolve_local_check_terms_request(&locator))
        }
        "recover" if args.len() == 3 => print_json(&sdk.resume_local_check_terms_request(&locator)),
        "current" if args.len() == 4 => {
            let initial =
                LocalInitialSetupLocatorV1::from_json_bytes(&read(&args[3])?).map_err(debug)?;
            print_json(
                &sdk.resolve_current_local_check_terms(&initial, &locator)
                    .map_err(debug)?,
            )
        }
        "author" if args.len() == 6 => {
            let initial =
                LocalInitialSetupLocatorV1::from_json_bytes(&read(&args[3])?).map_err(debug)?;
            let definition =
                LocalContractDefinitionV2::from_json_bytes(&read(&args[4])?).map_err(debug)?;
            let profile = handbook_engine::local_authority::resolved_local_operating_profile()
                .map_err(debug)?;
            let prepared = sdk
                .prepare_local_contract_definition(
                    &initial,
                    definition,
                    profile.context_resolution(),
                )
                .map_err(debug)?;
            print_json(
                &sdk.append_local_contract_definition(
                    &locator,
                    prepared,
                    SemanticTimeV1::owner_observation_now_v1().map_err(debug)?,
                    &args[5],
                )
                .map_err(debug)?,
            )
        }
        "transition" if args.len() == 9 => {
            let initial =
                LocalInitialSetupLocatorV1::from_json_bytes(&read(&args[3])?).map_err(debug)?;
            let binding: ExactBindingV1 = decode(&read(&args[4])?)?;
            let input: LocalLifecycleTransitionInputV2 = decode(&read(&args[5])?)?;
            let time = input
                .review
                .as_ref()
                .map(|review| review.semantic_time.clone())
                .unwrap_or(SemanticTimeV1::owner_observation_now_v1().map_err(debug)?);
            let prepared = sdk
                .prepare_local_contract_lifecycle(&initial, &binding, input)
                .map_err(debug)?;
            print_json(
                &sdk.transition_local_contract_lifecycle(
                    &locator,
                    prepared,
                    args[6].clone(),
                    args[7].clone(),
                    time,
                    &args[8],
                )
                .map_err(debug)?,
            )
        }
        "resolve-source" if args.len() == 7 => {
            let initial =
                LocalInitialSetupLocatorV1::from_json_bytes(&read(&args[3])?).map_err(debug)?;
            let binding: ExactBindingV1 = decode(&read(&args[4])?)?;
            let candidate: ContentAddressedBindingV1 = decode(&read(&args[5])?)?;
            let mut source = sdk
                .open_local_contract_evidence_source(&initial, &locator, binding, intent(&args[6])?)
                .map_err(debug)?;
            let observed = source
                .resolve_exact(
                    &candidate,
                    |resolved| -> Result<serde_json::Value, String> {
                        resolved.validate_current().map_err(debug)?;
                        decode(resolved.candidate_jcs())
                    },
                )
                .map_err(debug)??;
            print_json(&observed)
        }
        "admit" | "append" | "verdict"
            if args.len() == 7 || (args[0] == "append" && args.len() == 8) =>
        {
            let initial =
                LocalInitialSetupLocatorV1::from_json_bytes(&read(&args[3])?).map_err(debug)?;
            let binding: ExactBindingV1 = decode(&read(&args[4])?)?;
            let usable = sdk
                .prepare_local_contract_use(&initial, &binding, intent(&args[6])?)
                .map_err(debug)?;
            let policies = usable
                .definition()
                .dependencies()
                .policy_registry()
                .map_err(debug)?;
            let profile = handbook_engine::local_authority::resolved_local_operating_profile()
                .map_err(debug)?;
            let stack = profile.context_resolution();
            let owner = ContractOwnerRepositoryV1::open_existing_for_local_use_v2(&args[1])
                .map_err(debug)?;
            let mut source = sdk
                .open_local_contract_evidence_source(&initial, &locator, binding, intent(&args[6])?)
                .map_err(debug)?;
            let mut currentness = DockCapturedInputCurrentnessV1::open(&args[1]).map_err(debug)?;
            if args[0] == "verdict" {
                let run: ContractEvaluationRunV1 = decode(&read(&args[5])?)?;
                let evaluated = evaluate_claim_verdicts_v1(
                    &owner,
                    &mut source,
                    &mut currentness,
                    stack,
                    &policies,
                    &run,
                )
                .map_err(debug)?;
                return print_json(
                    &serde_json::json!({"evidence_repository_root": evaluated.evidence_repository_root(), "verdicts": evaluated.verdicts()}),
                );
            }
            let candidate: ContentAddressedBindingV1 = decode(&read(&args[5])?)?;
            if args[0] == "append" {
                if args.len() != 8 {
                    return Err("append requires a supplied idempotency key".into());
                }
                let appended = owner
                    .append_evidence_candidate_v1(
                        &mut source,
                        &mut currentness,
                        stack,
                        &policies,
                        &candidate,
                        &args[7],
                    )
                    .map_err(debug)?;
                print_json(
                    &serde_json::json!({"disposition": format!("{:?}", appended.disposition()), "receipt": appended.receipt(), "record": appended.record()}),
                )
            } else {
                print_json(
                    &admit_evidence_candidate_v1(
                        &owner,
                        &mut source,
                        &mut currentness,
                        stack,
                        &policies,
                        &candidate,
                    )
                    .map_err(debug)?,
                )
            }
        }
        "revoke" if args.len() == 4 => {
            let value = handbook_engine::parse_schema_json(&read(&args[3])?).map_err(debug)?;
            let request: LocalRevokeRequestV1 = serde_json::from_value(value).map_err(debug)?;
            sdk.revoke_local_check_terms(&locator, &request)
                .map_err(debug)?;
            println!("exact local-check authority revoked");
            Ok(())
        }
        _ => Err("invalid consumer command or argument count".into()),
    }
}

fn intent(value: &str) -> Result<LocalContractUseIntentV2, String> {
    match value {
        "diagnostic" => Ok(LocalContractUseIntentV2::Diagnostic),
        "decision-seeking" => Ok(LocalContractUseIntentV2::DecisionSeeking),
        _ => Err("intent must be diagnostic or decision-seeking".into()),
    }
}

fn decode<T: serde::de::DeserializeOwned>(bytes: &[u8]) -> Result<T, String> {
    serde_json::from_value(handbook_engine::parse_schema_json(bytes).map_err(debug)?).map_err(debug)
}

fn debug(error: impl std::fmt::Debug) -> String {
    format!("{error:?}")
}

fn read(path: &str) -> Result<Vec<u8>, String> {
    let metadata = fs::symlink_metadata(path).map_err(debug)?;
    if !metadata.is_file() || metadata.file_type().is_symlink() || metadata.len() > 2_097_152 {
        return Err("input must be a regular bounded file".into());
    }
    fs::read(path).map_err(debug)
}

fn save_new(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(debug)?;
    file.write_all(bytes).map_err(debug)?;
    file.sync_all().map_err(debug)
}

fn print_json(value: &impl serde::Serialize) -> Result<(), String> {
    println!("{}", serde_json::to_string(value).map_err(debug)?);
    Ok(())
}
