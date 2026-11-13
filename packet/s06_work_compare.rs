//! One allocation; no retries or recovery. Do not run without comparison release.
//! Use exact platform-canonical paths for the existing project parent and proof.
//! On Windows this normally requires verbatim `\\?\` spelling; native preflight
//! must establish that these same paths are admitted by the SDK. A durable start
//! intent without a returned record is an unknown invocation outcome.
#[allow(dead_code)]
#[path = "../tests/local_contract_support/mod.rs"]
mod local_contract_support;
#[allow(dead_code)]
#[path = "../tests/work_kind_support/mod.rs"]
mod support;
use handbook_sdk::{admission_governance::*, work_state::*, HandbookSdkV1};
use std::{
    fs::{self, File, OpenOptions},
    io::Write,
    path::Path,
    time::Instant,
};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
fn sync_dir(path: &Path) -> std::io::Result<()> {
    #[cfg(unix)]
    {
        File::open(path)?.sync_all()
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs::OpenOptionsExt;
        OpenOptions::new()
            .write(true)
            .share_mode(7)
            .custom_flags(0x0200_0000 | 0x0020_0000)
            .open(path)?
            .sync_all()
    }
}
fn append(file: &mut File, v: &serde_json::Value) -> Result<()> {
    serde_json::to_writer(&mut *file, v)?;
    file.write_all(b"\n")?;
    file.sync_all()?;
    Ok(())
}
fn main() -> Result<()> {
    let args: Vec<_> = std::env::args_os().collect();
    if args.len() != 3 {
        return Err("usage: s06_work_compare ABSENT_PROJECT EMPTY_PROOF".into());
    }
    for key in [
        "HANDBOOK_D16_GATE_DIR",
        "HANDBOOK_D16_GATE_ROOT",
        "HANDBOOK_D16_GATE_ROLE",
    ] {
        if std::env::var_os(key).is_some() {
            return Err(format!("{key} must be absent").into());
        }
    }
    let root = Path::new(&args[1]);
    let proof = Path::new(&args[2]);
    if !root.is_absolute()
        || !proof.is_absolute()
        || fs::symlink_metadata(root).is_ok()
        || !proof.is_dir()
        || root.starts_with(proof)
        || proof.starts_with(root)
    {
        return Err("absolute disjoint absent project and empty proof required".into());
    }
    let parent = root.parent().ok_or("project parent required")?;
    if parent.canonicalize()? != parent
        || proof.canonicalize()? != proof
        || fs::read_dir(proof)?.next().is_some()
    {
        return Err("link-free normalized existing parents and empty proof required".into());
    }
    let mut trace = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(proof.join("trace.jsonl"))?;
    sync_dir(proof)?;
    append(
        &mut trace,
        &serde_json::json!({"stage":"allocated","slots":1,"started":0,"returned":0,"readback":0,"maintenance_cap":64}),
    )?;
    fs::create_dir(root)?;
    let sdk = HandbookSdkV1::open(root);
    let (initial, check) = support::setup(&sdk);
    let d_locator = ProjectGovernanceLocatorV1::InitializeDeferred(initial.deferral.clone());
    let d_original = sdk.resolve_project_governance_request(&d_locator);
    if !matches!(
        &d_original,
        ProjectGovernanceExecutionV1::Committed {
            outcome: ProjectGovernanceCommittedV1::InitialDeferred(_),
            delivery: GovernanceDeliveryV1::Replay
        }
    ) {
        return Err("exact deferred original not replayed".into());
    }
    let selection =
        GovernanceProjectSelectionV1::InitiallyDeferred(initial.deferral.project.clone());
    let d_current = sdk
        .resolve_current_project_governance(&selection, ADMISSION_GOVERNANCE_SCOPE_V1, None)
        .map_err(|e| format!("{e:?}"))?;
    if !matches!((&d_current,&d_original), (CurrentProjectGovernanceV1::Deferred {original,basis},ProjectGovernanceExecutionV1::Committed {outcome:ProjectGovernanceCommittedV1::InitialDeferred(outcome),..}) if original==&initial.deferral && outcome.locator==*original && basis.outcome_fingerprint==outcome.outcome_fingerprint)
    {
        return Err("exact current deferred governance and original join required".into());
    }
    if !root
        .join(".handbook/admission-governance/v2/database/READY.json")
        .is_file()
        || root
            .join(".handbook/admission-governance/v2/scope-current.json")
            .exists()
    {
        return Err("native governance backing footprint mismatch".into());
    }
    let prepared_grant = sdk
        .prepare_work_registration_terms(
            &initial,
            &check,
            support::terms(&check, "s06-compare-grant"),
            WorkTermsTransitionV1::Initial,
        )
        .map_err(|e| format!("{e:?}"))?;
    let grant = prepared_grant.locator().clone();
    let mut saved_grant = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(proof.join("grant-original.json"))?;
    saved_grant.write_all(&grant.canonical_json_bytes().map_err(|e| format!("{e:?}"))?)?;
    saved_grant.sync_all()?;
    sync_dir(proof)?;
    append(
        &mut trace,
        &serde_json::json!({"stage":"grant_intent", "invocation_observed":"unknown_until_return", "grant":grant}),
    )?;
    let grant_activation = sdk.activate_work_registration_terms(prepared_grant);
    append(
        &mut trace,
        &serde_json::json!({"stage":"grant_returned", "execution":grant_activation}),
    )?;
    if grant_activation.disposition != WorkDispositionV1::Committed
        || grant_activation.outcome.as_ref().map(|v| &v.locator) != Some(&grant)
    {
        return Err("original grant activation did not commit exact locator; stop".into());
    }
    let grant_original = sdk.resolve_work_registration_terms_request(&grant);
    if grant_original.disposition != WorkDispositionV1::Replayed
        || grant_original.outcome.as_ref() != grant_activation.outcome.as_ref()
    {
        return Err("grant original mismatch".into());
    }
    let grant_current = sdk
        .resolve_current_work_registration_terms(&initial, &grant)
        .map_err(|e| format!("{e:?}"))?;
    if grant_current.locator != grant {
        return Err("current grant mismatch".into());
    }
    let input = WorkRegistrationInputV1 {
        request_key: "s06-compare-request".into(),
        item_key: "s06-compare-item".into(),
        label: "L".repeat(1024),
        question: "Q".repeat(1024),
        actor: grant.terms.agent.clone(),
        expected_state: "absent".into(),
        composition: WorkCompositionV1::ordinary_leaf().into(),
        display_level: None,
    };
    if input.actor != "packet2.author" || grant.terms.principal != "packet2.human" {
        return Err("fixture authority differs".into());
    }
    let prepared = sdk
        .prepare_work_registration(&initial, &grant, input)
        .map_err(|e| format!("{e:?}"))?;
    let locator = prepared.locator().clone();
    let absent = sdk.resolve_work_registration_request(&locator);
    if absent.disposition != WorkDispositionV1::Unaccepted {
        return Err(format!("original not positively absent: {absent:?}").into());
    }
    let mut saved = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(proof.join("original.json"))?;
    saved.write_all(
        &locator
            .canonical_json_bytes()
            .map_err(|e| format!("{e:?}"))?,
    )?;
    saved.sync_all()?;
    sync_dir(proof)?;
    append(
        &mut trace,
        &serde_json::json!({"stage":"prepared","started":0,"returned":0,"initial":initial,"check":check,"grant":grant,"grant_activation":grant_activation,"grant_original":grant_original,"grant_current":grant_current,"d_original":d_original,"d_current":d_current,"locator":locator,"basis":locator.grant.basis,"maintenance_cap":64}),
    )?;
    append(
        &mut trace,
        &serde_json::json!({"stage":"start_intent","invocation_observed":"unknown_until_return","returned":0,"readback":0}),
    )?;
    let clock = Instant::now();
    let execution = sdk.register_work_item(prepared);
    let elapsed = clock.elapsed().as_nanos();
    append(
        &mut trace,
        &serde_json::json!({"stage":"returned","started":1,"returned":1,"readback":0,"execution":execution,"sdk_elapsed_ns":elapsed.to_string()}),
    )?;
    if execution.disposition != WorkDispositionV1::Committed {
        return Err("noncommitted or uncertain; stop with saved original; no recovery".into());
    }
    let receipt = execution
        .outcome
        .as_ref()
        .ok_or("committed receipt missing")?;
    if receipt.locator != locator {
        return Err("committed receipt locator mismatch".into());
    }
    let mut completed = false;
    for call in 1..=64 {
        let progress = sdk
            .maintain_work_storage(WorkMaintenanceBudgetV1::default())
            .map_err(|e| format!("{e:?}"))?;
        append(
            &mut trace,
            &serde_json::json!({"stage":"maintenance","call":call,"progress":progress}),
        )?;
        if progress.complete {
            completed = true;
            break;
        }
    }
    if !completed {
        return Err("maintenance cap reached; stop".into());
    }
    drop(sdk);
    let reopened = HandbookSdkV1::open(root);
    let saved_locator =
        WorkRequestLocatorV1::from_json_bytes(&fs::read(proof.join("original.json"))?)
            .map_err(|e| format!("{e:?}"))?;
    if saved_locator != locator {
        return Err("saved original differs".into());
    }
    let original = reopened.resolve_work_registration_request(&saved_locator);
    let current = reopened
        .resolve_current_work_item(&grant, "s06-compare-item")
        .map_err(|e| format!("{e:?}"))?;
    let grant_again = reopened.resolve_work_registration_terms_request(&grant);
    let grant_current_again = reopened
        .resolve_current_work_registration_terms(&initial, &grant)
        .map_err(|e| format!("{e:?}"))?;
    let d_again = reopened.resolve_project_governance_request(&d_locator);
    let d_current_again = reopened
        .resolve_current_project_governance(&selection, ADMISSION_GOVERNANCE_SCOPE_V1, None)
        .map_err(|e| format!("{e:?}"))?;
    let basis_again = reopened
        .observe_local_check_basis(&initial)
        .map_err(|e| format!("{e:?}"))?;
    if original.disposition != WorkDispositionV1::Replayed
        || original.outcome.as_ref() != Some(receipt)
        || !matches!(&current,WorkCurrentItemV1::Open(snapshot) if snapshot==&receipt.snapshot)
        || grant_again.disposition != WorkDispositionV1::Replayed
        || grant_again.outcome.as_ref() != grant_original.outcome.as_ref()
        || grant_current_again != grant_current
        || d_again != d_original
        || d_current_again != d_current
        || basis_again != locator.grant.basis
    {
        return Err("independent reopen mismatch; stop".into());
    }
    let stats = reopened
        .work_storage_statistics()
        .map_err(|e| format!("{e:?}"))?;
    append(
        &mut trace,
        &serde_json::json!({"stage":"reopened","started":1,"returned":1,"readback":1,"original":original,"current":current,"grant":grant_again,"grant_current":grant_current_again,"governance":d_again,"governance_current":d_current_again,"basis":basis_again,"statistics":stats}),
    )?;
    Ok(())
}
