//! Ordinary local advisory consumer. All approvals and actual review outcomes
//! arrive as supplied files; this program never manufactures either.
use handbook_contracts::*;
use handbook_sdk::local_check::LocalCheckLocatorV1;
use handbook_sdk::local_evaluation::*;
use handbook_sdk::local_setup::{LocalInitialSetupLocatorV1, LocalInitialSetupRequestV1};
use handbook_sdk::HandbookSdkV1;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;

fn main() {
    let result = std::thread::Builder::new()
        .name("local-evaluation-consumer".into())
        .stack_size(8 * 1024 * 1024)
        .spawn(run)
        .expect("start bounded consumer")
        .join()
        .expect("consumer completed");
    if let Err(error) = result {
        eprintln!("{error}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), String> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    if args.len() < 3 {
        return Err(usage());
    }
    let sdk = HandbookSdkV1::open(&args[1]);
    match args[0].as_str() {
        "initial" if args.len() == 4 => {
            let request: LocalInitialSetupRequestV1 = decode_file(&args[2])?;
            let prepared = sdk.prepare_local_initial_setup(&request).map_err(debug)?;
            save_new(
                Path::new(&args[3]),
                &prepared.locator().canonical_json_bytes().map_err(debug)?,
            )?;
            print_json(&sdk.establish_local_initial_setup(prepared))
        }
        "adopt-check" if args.len() == 5 => {
            let initial = initial(&args[2])?;
            let terms =
                handbook_sdk::local_check::LocalCheckTermsV1::from_json_bytes(&read(&args[3])?)
                    .map_err(debug)?;
            let prepared = sdk
                .prepare_local_check_terms(&initial, terms)
                .map_err(debug)?;
            save_new(
                Path::new(&args[4]),
                &prepared.locator().canonical_json_bytes().map_err(debug)?,
            )?;
            print_json(&sdk.adopt_local_check_terms(prepared))
        }
        "author" if args.len() == 6 => {
            let check = check(&args[2])?;
            let initial = initial(&args[3])?;
            let definition =
                LocalContractDefinitionV3::from_json_bytes(&read(&args[4])?).map_err(debug)?;
            let profile = handbook_engine::local_authority::resolved_local_operating_profile()
                .map_err(debug)?;
            let prepared = sdk
                .prepare_local_evaluation_definition(
                    &initial,
                    &check,
                    definition,
                    profile.context_resolution(),
                )
                .map_err(debug)?;
            print_json(
                &sdk.append_local_evaluation_definition(
                    &check,
                    prepared,
                    SemanticTimeV1::owner_observation_now_v1().map_err(debug)?,
                    &args[5],
                )
                .map_err(debug)?,
            )
        }
        "transition" if args.len() == 9 => {
            let check = check(&args[2])?;
            let initial = initial(&args[3])?;
            let binding: ExactBindingV1 = decode_file(&args[4])?;
            let input: LocalLifecycleTransitionInputV3 = decode_file(&args[5])?;
            let time = input
                .review
                .as_ref()
                .map(|r| r.semantic_time.clone())
                .unwrap_or(SemanticTimeV1::owner_observation_now_v1().map_err(debug)?);
            let prepared = sdk
                .prepare_local_evaluation_lifecycle(&initial, &binding, input)
                .map_err(debug)?;
            print_json(
                &sdk.transition_local_evaluation_lifecycle(
                    &check,
                    prepared,
                    args[6].clone(),
                    args[7].clone(),
                    time,
                    &args[8],
                )
                .map_err(debug)?,
            )
        }
        "use" if args.len() == 5 => {
            let initial = initial(&args[2])?;
            let binding: ExactBindingV1 = decode_file(&args[3])?;
            let usable = sdk
                .prepare_local_evaluation_use(&initial, &binding, intent(&args[4])?)
                .map_err(debug)?;
            print_json(
                &serde_json::json!({"definition": usable.definition(), "applicability": usable.applicability(), "current_basis": usable.current_basis(), "lifecycle_head": usable.lifecycle_head_fingerprint()}),
            )
        }
        "adopt-evaluation" if args.len() == 6 => {
            let check = check(&args[2])?;
            let terms = LocalEvaluationTermsV1::from_json_bytes(&read(&args[3])?).map_err(debug)?;
            let basis = decode_file(&args[4])?;
            let prepared = sdk
                .prepare_local_evaluation_terms(&check, terms, basis)
                .map_err(debug)?;
            save_new(
                Path::new(&args[5]),
                &serde_json::to_vec(prepared.locator()).map_err(debug)?,
            )?;
            print_json(&sdk.adopt_local_evaluation_terms(prepared))
        }
        "recover-terms" if args.len() == 3 => {
            print_json(&sdk.recover_local_evaluation_terms(&decode_file(&args[2])?))
        }
        "terms-history" if args.len() == 3 => {
            print_json(&sdk.read_local_evaluation_terms_history(&decode_file(&args[2])?))
        }
        "adopt-correction-delegation" if args.len() == 6 => {
            let initial = initial(&args[2])?;
            let authority = decode_file(&args[3])?;
            let prepared = sdk
                .prepare_local_input_correction_delegation(
                    &initial,
                    &authority,
                    LocalInputCorrectionDelegationV1::from_json_bytes(&read(&args[4])?)
                        .map_err(debug)?,
                )
                .map_err(debug)?;
            save_new(
                Path::new(&args[5]),
                &prepared.locator().canonical_json_bytes().map_err(debug)?,
            )?;
            print_json(&sdk.adopt_local_input_correction_delegation(&authority, prepared))
        }
        "recover-correction-delegation" if args.len() == 4 => {
            print_json(&sdk.recover_local_input_correction_delegation(
                &decode_file(&args[2])?,
                &correction_delegation(&args[3])?,
            ))
        }
        "resume-correction-delegation" if args.len() == 4 => {
            print_json(&sdk.resume_local_input_correction_delegation(
                &decode_file(&args[2])?,
                &correction_delegation(&args[3])?,
            ))
        }
        "request-correction" if args.len() == 7 => {
            let initial = initial(&args[2])?;
            let authority = decode_file(&args[3])?;
            let delegation = correction_delegation(&args[4])?;
            let input =
                LocalInputCorrectionRequestV1::from_json_bytes(&read(&args[5])?).map_err(debug)?;
            let adopted = sdk
                .recover_local_input_correction_delegation(&authority, &delegation)
                .outcome
                .ok_or_else(|| "exact adopted delegation is unavailable".to_owned())?;
            let original = input
                .locator(&adopted.delegation_fingerprint)
                .map_err(debug)?;
            save_new(
                Path::new(&args[6]),
                &original.canonical_json_bytes().map_err(debug)?,
            )?;
            print_json(
                &sdk.retain_local_input_correction_request(
                    &initial,
                    &authority,
                    &delegation,
                    &input,
                )
                .map_err(debug)?,
            )
        }
        "submit-correction" if args.len() == 8 => {
            let initial = initial(&args[2])?;
            let authority = decode_file(&args[3])?;
            let delegation = correction_delegation(&args[4])?;
            let request = correction_request(&args[5])?;
            let submission = LocalInputCorrectionSubmissionV1::from_json_bytes(&read(&args[6])?)
                .map_err(debug)?;
            let original = submission.locator(&request).map_err(debug)?;
            save_new(
                Path::new(&args[7]),
                &original.canonical_json_bytes().map_err(debug)?,
            )?;
            print_json(
                &sdk.submit_local_input_correction_decision(
                    &initial,
                    &authority,
                    &delegation,
                    &request,
                    &submission,
                )
                .map_err(debug)?,
            )
        }
        "recover-correction-request" if args.len() == 5 => {
            print_json(&sdk.recover_local_input_correction_request(
                &decode_file(&args[2])?,
                &correction_delegation(&args[3])?,
                &correction_request(&args[4])?,
            ))
        }
        "recover-correction-decision" if args.len() == 6 => {
            print_json(&sdk.recover_local_input_correction_decision(
                &decode_file(&args[2])?,
                &correction_delegation(&args[3])?,
                &correction_request(&args[4])?,
                &correction_decision(&args[5])?,
            ))
        }
        "issue" if args.len() == 5 => {
            let authority = decode_file(&args[2])?;
            let request = LocalEvaluationAuthorizationRequestV1::from_json_bytes(&read(&args[3])?)
                .map_err(debug)?;
            let original = sdk
                .local_evaluation_authorization_locator(&authority, &request)
                .map_err(debug)?;
            save_new(
                Path::new(&args[4]),
                &serde_json::to_vec(&original).map_err(debug)?,
            )?;
            print_json(&sdk.issue_local_evaluation_authorization(&authority, &request))
        }
        "resume-authorization" if args.len() == 5 => {
            print_json(&sdk.resume_local_evaluation_authorization(
                &decode_file(&args[2])?,
                &decode_file(&args[3])?,
                &decode_file(&args[4])?,
            ))
        }
        "revoke-evaluation" if args.len() == 5 => {
            sdk.revoke_local_evaluation_terms(
                &decode_file(&args[2])?,
                &decode_file(&args[3])?,
                &decode_file(&args[4])?,
            )
            .map_err(debug)?;
            print_json(&serde_json::json!({"owner":"engine","revocation":"retained"}))
        }
        "recover-authorization" if args.len() == 5 => {
            print_json(&sdk.recover_local_evaluation_authorization(
                &decode_file(&args[2])?,
                &decode_file(&args[3])?,
                &decode_file(&args[4])?,
            ))
        }
        "authorization-history" if args.len() == 5 => {
            print_json(&sdk.read_local_evaluation_authorization_history(
                &decode_file(&args[2])?,
                &decode_file(&args[3])?,
                &decode_file(&args[4])?,
            ))
        }
        "prepare-input" if args.len() == 7 => {
            let initial = initial(&args[2])?;
            let binding: ExactBindingV1 = decode_file(&args[3])?;
            let content = decode_file(&args[4])?;
            let question: SuppliedQuestion = decode_file(&args[5])?;
            let head = sdk
                .current_local_evaluation_lifecycle(&binding)
                .map_err(debug)?;
            let manifest = handbook_dock::DockManifestRegistryV1::production();
            let page = manifest
                .list(handbook_dock::DockManifestListRequestV1::first(1).map_err(debug)?)
                .map_err(debug)?;
            let manifest = match manifest
                .read(
                    page.entries()[0].manifest_ref(),
                    page.entries()[0].manifest_fingerprint(),
                )
                .map_err(debug)?
            {
                handbook_dock::DockManifestReadOutcomeV1::Found(m) => m,
                _ => return Err("native manifest absent".into()),
            };
            let role = &manifest.supported_inputs()[0];
            let capture = handbook_dock::DockInlineJsonInputV1::new(
                role.role().clone(),
                role.media_type(),
                role.schema().clone(),
                content,
            )
            .map_err(debug)?;
            let subject = capture.captured_evaluation_subject_v1().map_err(debug)?;
            let input = LocalEvaluationPrepareInputV3 {
                request_key: question.request_key,
                contract: binding,
                expected_lifecycle_fingerprint: head.lifecycle_fingerprint().clone(),
                subject: subject.clone(),
                case: capture
                    .captured_evaluation_case_v1(
                        StableIdV1::parse("case.schema-conformance").map_err(debug)?,
                    )
                    .map_err(debug)?,
                requested_resolution: question.requested_resolution,
                formation: LocalEvaluationFormationRequestV1 {
                    subject_ref: subject.subject().reference().into(),
                    subject_fingerprint: subject.subject().fingerprint().as_str().into(),
                    scope_fingerprint: question.scope_fingerprint,
                    objective: question.objective,
                    use_intent: question.use_intent,
                    requested_reach: question.requested_reach,
                    reach: question.reach,
                    retention_until_utc: question.retention_until_utc,
                },
            };
            let _ = initial;
            save_new(
                Path::new(&args[6]),
                &serde_json::to_vec(&input).map_err(debug)?,
            )?;
            print_json(&input)
        }
        "form" if args.len() == 7 => {
            let initial = initial(&args[2])?;
            let authority = decode_file(&args[3])?;
            let input = decode_file(&args[4])?;
            let prepared = sdk
                .prepare_local_evaluation(&initial, &authority, input)
                .map_err(debug)?;
            save_new(
                Path::new(&args[6]),
                &serde_json::to_vec(&prepared.request_locator()).map_err(debug)?,
            )?;
            let formed = sdk
                .form_local_evaluation(&authority, prepared)
                .map_err(debug)?;
            save_new(
                Path::new(&args[5]),
                &serde_json::to_vec(&formed.locator).map_err(debug)?,
            )?;
            print_json(&formed)
        }
        "repeat" | "correct" if args.len() == 7 => {
            let initial = initial(&args[2])?;
            let authority = decode_file(&args[3])?;
            let input = followup_input(&args[4])?;
            let action = if args[0] == "repeat" {
                LocalEvaluationFollowupActionV1::FreshRepeat
            } else {
                LocalEvaluationFollowupActionV1::CorrectedInput
            };
            if input.action != action {
                return Err("supplied followup action differs from explicit caller choice".into());
            }
            let prepared = sdk
                .prepare_local_evaluation_followup(&initial, &authority, input)
                .map_err(debug)?;
            save_new(
                Path::new(&args[6]),
                &serde_json::to_vec(&prepared.request_locator()).map_err(debug)?,
            )?;
            let formed = sdk
                .form_local_evaluation_followup(&authority, prepared)
                .map_err(debug)?;
            save_new(
                Path::new(&args[5]),
                &serde_json::to_vec(&formed.locator).map_err(debug)?,
            )?;
            print_json(&formed)
        }
        "lineage" if args.len() == 3 => print_json(
            &sdk.read_local_evaluation_lineage(&decode_file(&args[2])?)
                .map_err(debug)?,
        ),
        "correction-candidate" if args.len() == 7 => {
            let initial = initial(&args[2])?;
            let authority = decode_file(&args[3])?;
            let prepared = sdk
                .prepare_local_evaluation_followup(&initial, &authority, followup_input(&args[4])?)
                .map_err(debug)?;
            let source = sdk
                .retain_local_input_correction_candidate(
                    &initial,
                    &authority,
                    &prepared,
                    decode_file(&args[5])?,
                )
                .map_err(debug)?;
            save_new(
                Path::new(&args[6]),
                &source.canonical_json_bytes().map_err(debug)?,
            )?;
            print_json(&source)
        }
        "consume-correction" if args.len() == 10 => {
            let initial = initial(&args[2])?;
            let authority = decode_file(&args[3])?;
            let prepared = sdk
                .prepare_local_evaluation_followup(&initial, &authority, followup_input(&args[7])?)
                .map_err(debug)?;
            save_new(
                Path::new(&args[9]),
                &serde_json::to_vec(&prepared.request_locator()).map_err(debug)?,
            )?;
            let formed = sdk
                .form_approved_local_input_correction(
                    &initial,
                    &authority,
                    &correction_delegation(&args[4])?,
                    &correction_request(&args[5])?,
                    &correction_decision(&args[6])?,
                    prepared,
                )
                .map_err(debug)?;
            save_new(
                Path::new(&args[8]),
                &serde_json::to_vec(&formed.locator).map_err(debug)?,
            )?;
            print_json(&formed)
        }
        "capability" if args.len() == 6 => print_json(
            &sdk.observe_local_evaluation_capability(
                &initial(&args[2])?,
                &decode_file(&args[3])?,
                &decode_file(&args[4])?,
                &decode_file(&args[5])?,
            )
            .map_err(debug)?,
        ),
        "authorization-request" if args.len() == 6 => {
            let authority: LocalEvaluationAuthorityLocatorV1 = decode_file(&args[2])?;
            let snapshot = sdk
                .recover_local_evaluation(&decode_file(&args[3])?)
                .map_err(debug)?;
            let reach = decode_file(&args[4])?;
            let requested = snapshot.plan.selections[0].requested_reach.clone();
            let envelope = LocalEvaluationEnvelopeV1::local(
                &snapshot.purpose.subject_ref,
                requested,
                &authority.terms,
            );
            let request = snapshot.authorization_request(envelope, reach);
            save_new(
                Path::new(&args[5]),
                &request.canonical_json_bytes().map_err(debug)?,
            )?;
            print_json(
                &serde_json::json!({"purpose": snapshot.purpose.content_fingerprint, "request_saved": args[5]}),
            )
        }
        "process-request" if args.len() == 5 => {
            let snapshot = sdk
                .recover_local_evaluation(&decode_file(&args[2])?)
                .map_err(debug)?;
            let request = process_request(&snapshot, decode_file(&args[3])?)?;
            save_new(
                Path::new(&args[4]),
                &request.canonical_json_bytes().map_err(debug)?,
            )?;
            print_json(&request.binding().map_err(debug)?)
        }
        "recover" if args.len() == 3 => print_json(
            &sdk.recover_local_evaluation(&decode_file(&args[2])?)
                .map_err(debug)?,
        ),
        "recover-request" if args.len() == 3 => print_json(
            &sdk.recover_local_evaluation_request(&decode_file(&args[2])?)
                .map_err(debug)?,
        ),
        "history" if args.len() == 3 => {
            let original = decode_file(&args[2])?;
            let history = sdk
                .read_local_evaluation_history(&original)
                .map_err(debug)?;
            print_history(&sdk, &original, history)
        }
        "result-history" if args.len() == 3 => {
            let original: LocalEvaluationResultLocatorV3 = decode_file(&args[2])?;
            let history = sdk
                .read_local_evaluation_result_history(&original)
                .map_err(debug)?;
            print_history(&sdk, &original.evaluation, history)
        }
        "recover-attempt" if args.len() == 4 => print_json(
            &sdk.recover_local_evaluation_attempt(&decode_file(&args[2])?, &decode_file(&args[3])?)
                .map_err(debug)?,
        ),
        "assess" | "commit" | "execute" | "admit" | "frontier" | "set" | "close"
            if args.len() >= 7 =>
        {
            let initial = initial(&args[2])?;
            let authority = decode_file(&args[3])?;
            let original_issuance = decode_file(&args[4])?;
            let authorization_request =
                LocalEvaluationAuthorizationRequestV1::from_json_bytes(&read(&args[5])?)
                    .map_err(debug)?;
            let evaluation = decode_file(&args[6])?;
            let session = LocalEvaluationSessionV1 {
                initial: &initial,
                authority: &authority,
                original_issuance: &original_issuance,
                authorization_request: &authorization_request,
                evaluation: &evaluation,
            };
            match args[0].as_str() {
                "assess" if args.len() == 9 => print_json(
                    &sdk.assess_local_evaluation(
                        &session,
                        &decode_file(&args[7])?,
                        &decode_file(&args[8])?,
                    )
                    .map_err(debug)?,
                ),
                "commit" if args.len() == 7 => print_json(
                    &sdk.commit_local_evaluation_collection(&session)
                        .map_err(debug)?,
                ),
                "execute" if args.len() == 9 => print_json(
                    &sdk.execute_local_evaluation_once(
                        &session,
                        &decode_file(&args[7])?,
                        &decode_file(&args[8])?,
                        &handbook_dock::DockCancellationV1::new(),
                    )
                    .map_err(debug)?,
                ),
                "admit" if args.len() == 9 => print_json(
                    &sdk.admit_local_evaluation_origin(&session, &decode_file(&args[7])?, &args[8])
                        .map_err(debug)?,
                ),
                "frontier" if args.len() == 7 => print_json(
                    &sdk.seal_local_evaluation_frontier(&session)
                        .map_err(debug)?,
                ),
                "set" if args.len() == 7 => {
                    print_json(&sdk.close_local_evaluation_set(&session).map_err(debug)?)
                }
                "close" if args.len() == 7 || args.len() == 8 => {
                    let closed = sdk.close_local_evaluation_result(&session).map_err(debug)?;
                    if args.len() == 8 {
                        save_new(
                            Path::new(&args[7]),
                            &serde_json::to_vec(&closed.result_locator().map_err(debug)?)
                                .map_err(debug)?,
                        )?;
                    }
                    print_json(&closed)
                }
                _ => Err(usage()),
            }
        }
        _ => Err(usage()),
    }
}

fn process_request(
    snapshot: &LocalEvaluationSnapshotV3,
    content: serde_json::Value,
) -> Result<handbook_dock::DockProcessRequestV1, String> {
    use handbook_dock::*;
    // Request bytes are untrusted supplied input, including when the actual
    // native package is unavailable. Dock qualifies its package at consumption.
    let registry = DockManifestRegistryV1::production();
    let page = registry
        .list(DockManifestListRequestV1::first(1).map_err(debug)?)
        .map_err(debug)?;
    let manifest = match registry
        .read(
            page.entries()[0].manifest_ref(),
            page.entries()[0].manifest_fingerprint(),
        )
        .map_err(debug)?
    {
        DockManifestReadOutcomeV1::Found(manifest) => manifest,
        _ => return Err("native manifest absent".into()),
    };
    let role = &manifest.supported_inputs()[0];
    let capture = DockInlineJsonInputV1::new(
        role.role().clone(),
        role.media_type(),
        role.schema().clone(),
        content,
    )
    .map_err(debug)?;
    let run = &snapshot.run;
    if capture.captured_evaluation_subject_v1().map_err(debug)? != *run.subject() {
        return Err("supplied capture differs from original exact Purpose subject".into());
    }
    DockProcessRequestV1::new(
        &manifest,
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
    .map_err(debug)
}

fn print_history(
    sdk: &HandbookSdkV1,
    original: &LocalEvaluationLocatorV3,
    history: LocalEvaluationHistoryV3,
) -> Result<(), String> {
    let (lineage, lineage_refusal) = match sdk.read_local_evaluation_lineage(original) {
        Ok(lineage) => (lineage, None),
        Err(error) => (None, Some(debug(error))),
    };
    print_json(
        &serde_json::json!({"history":history,"summary":history.result.human_summary(),
        "lineage":lineage,"lineage_refusal":lineage_refusal}),
    )
}

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct SuppliedQuestion {
    request_key: String,
    objective: String,
    scope_fingerprint: String,
    use_intent: LocalEvaluationUseIntentV1,
    requested_reach: LocalEvaluationReachV1,
    requested_resolution: ResolutionEnvelopeV1,
    reach: LocalEvaluationPathReachV1,
    retention_until_utc: String,
}

fn usage() -> String {
    concat!(
        "usage: local_evaluation COMMAND ABSOLUTE_ROOT supplied-files...\n",
        "initial REQUEST RECOVERY | adopt-check INITIAL TERMS RECOVERY\n",
        "author CHECK INITIAL DEFINITION KEY | transition CHECK INITIAL BINDING INPUT ACTOR PARTICIPANT KEY\n",
        "use INITIAL BINDING INTENT | adopt-evaluation CHECK TERMS BASIS RECOVERY\n",
        "prepare-input INITIAL BINDING CONTENT QUESTION OUTPUT | form INITIAL AUTHORITY INPUT OCCURRENCE_RECOVERY REQUEST_RECOVERY\n",
        "authorization-request AUTHORITY OCCURRENCE REACH OUTPUT | issue AUTHORITY REQUEST RECOVERY\n",
        "capability INITIAL AUTHORITY FORMATION DOCK_REACH | process-request OCCURRENCE CONTENT OUTPUT\n",
        "assess INITIAL AUTHORITY ISSUANCE REQUEST OCCURRENCE CAPABILITY DOCK_REACH\n",
        "commit/frontier/set INITIAL AUTHORITY ISSUANCE REQUEST OCCURRENCE\n",
        "close INITIAL AUTHORITY ISSUANCE REQUEST OCCURRENCE [RESULT_RECOVERY]\n",
        "execute INITIAL AUTHORITY ISSUANCE REQUEST OCCURRENCE PROCESS_REQUEST DOCK_REACH\n",
        "admit INITIAL AUTHORITY ISSUANCE REQUEST OCCURRENCE DOCK_REACH OPERATION_KEY\n",
        "recover-request REQUEST_RECOVERY | recover/history OCCURRENCE | recover-attempt ATTEMPT DOCK_REACH\n",
        "recover-authorization/resume-authorization/authorization-history AUTHORITY ORIGINAL REACH\n",
        "revoke-evaluation AUTHORITY PRINCIPAL REACH | result-history RESULT_RECOVERY\n",
        "recover-terms/terms-history AUTHORITY",
        "\nrepeat/correct INITIAL AUTHORITY FOLLOWUP OCCURRENCE_RECOVERY REQUEST_RECOVERY | lineage OCCURRENCE\n",
        "adopt-correction-delegation INITIAL AUTHORITY APPROVED_DELEGATION RECOVERY\n",
        "recover-correction-delegation/resume-correction-delegation AUTHORITY DELEGATION\n",
        "correction-candidate INITIAL AUTHORITY FOLLOWUP REJECTED_CANDIDATE SOURCE_OUTPUT\n",
        "request-correction INITIAL AUTHORITY DELEGATION REQUEST_INPUT RECOVERY\n",
        "submit-correction INITIAL AUTHORITY DELEGATION REQUEST SUBMISSION RECOVERY\n",
        "recover-correction-request AUTHORITY DELEGATION REQUEST\n",
        "recover-correction-decision AUTHORITY DELEGATION REQUEST DECISION\n",
        "consume-correction INITIAL AUTHORITY DELEGATION REQUEST DECISION FOLLOWUP OCCURRENCE_RECOVERY REQUEST_RECOVERY"
    ).into()
}
fn initial(path: &str) -> Result<LocalInitialSetupLocatorV1, String> {
    LocalInitialSetupLocatorV1::from_json_bytes(&read(path)?).map_err(debug)
}
fn followup_input(path: &str) -> Result<LocalEvaluationFollowupInputV1, String> {
    LocalEvaluationFollowupInputV1::from_json_bytes(&read(path)?).map_err(debug)
}
fn correction_delegation(path: &str) -> Result<LocalInputCorrectionDelegationLocatorV1, String> {
    LocalInputCorrectionDelegationLocatorV1::from_json_bytes(&read(path)?).map_err(debug)
}
fn correction_request(path: &str) -> Result<LocalInputCorrectionRequestLocatorV1, String> {
    LocalInputCorrectionRequestLocatorV1::from_json_bytes(&read(path)?).map_err(debug)
}
fn correction_decision(path: &str) -> Result<LocalInputCorrectionDecisionLocatorV1, String> {
    LocalInputCorrectionDecisionLocatorV1::from_json_bytes(&read(path)?).map_err(debug)
}
fn check(path: &str) -> Result<LocalCheckLocatorV1, String> {
    LocalCheckLocatorV1::from_json_bytes(&read(path)?).map_err(debug)
}
fn intent(text: &str) -> Result<LocalContractUseIntentV2, String> {
    match text {
        "diagnostic" => Ok(LocalContractUseIntentV2::Diagnostic),
        "decision_seeking" => Ok(LocalContractUseIntentV2::DecisionSeeking),
        _ => Err("unsupported intent".into()),
    }
}
fn read(path: &str) -> Result<Vec<u8>, String> {
    fs::read(path).map_err(debug)
}
fn decode_file<T: DeserializeOwned>(path: &str) -> Result<T, String> {
    serde_json::from_slice(&read(path)?).map_err(debug)
}
fn save_new(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let mut file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(path)
        .map_err(debug)?;
    file.write_all(bytes).map_err(debug)?;
    file.sync_all().map_err(debug)
}
fn print_json(value: &impl Serialize) -> Result<(), String> {
    println!("{}", serde_json::to_string(value).map_err(debug)?);
    Ok(())
}
fn debug(value: impl std::fmt::Debug) -> String {
    format!("{value:?}")
}
