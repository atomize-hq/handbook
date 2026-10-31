//! Fixed finite Linux SDK comparison. Run only after source/measurement release.
//! Existing setup helpers supply ordinary public-owner authorization.
#[allow(dead_code)]
#[path = "../tests/local_contract_support/mod.rs"]
mod local_contract_support;
#[allow(dead_code)]
#[path = "../tests/work_kind_support/mod.rs"]
mod support;
use handbook_sdk::work_state::*;
use handbook_sdk::HandbookSdkV1;
use std::{
    fs::{File, OpenOptions},
    io::Write,
    path::Path,
    time::Instant,
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
fn append(file: &mut File, value: &serde_json::Value) -> Result<()> {
    serde_json::to_writer(&mut *file, value)?;
    file.write_all(b"\n")?;
    file.sync_all()?;
    Ok(())
}
fn save_original(proof: &Path, n: usize, locator: &WorkRequestLocatorV1) -> Result<()> {
    let bytes = locator
        .canonical_json_bytes()
        .map_err(|e| format!("{e:?}"))?;
    let mut file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(proof.join(format!("original-{n:02}.json")))?;
    file.write_all(&bytes)?;
    file.sync_all()?;
    File::open(proof)?.sync_all()?;
    Ok(())
}
fn settle_original(
    sdk: &HandbookSdkV1,
    locator: &WorkRequestLocatorV1,
    trace: &mut File,
) -> Result<()> {
    // No new key, replacement sample, or continued performance arm. Preserve every
    // observation, using the existing finite exact-original resolve/recover route.
    for _ in 0..2 {
        let resolved = sdk.resolve_work_registration_request(locator);
        append(
            trace,
            &serde_json::json!({"stage":"resolve-after-unexpected", "locator":locator, "execution":resolved}),
        )?;
        match resolved.disposition {
            WorkDispositionV1::Committed
            | WorkDispositionV1::Replayed
            | WorkDispositionV1::Unaccepted => return Ok(()),
            WorkDispositionV1::Pending | WorkDispositionV1::Uncertain => {
                let recovered = sdk.recover_work_registration_request(locator);
                append(
                    trace,
                    &serde_json::json!({"stage":"recover-after-unexpected", "locator":locator, "execution":recovered}),
                )?;
                if matches!(
                    recovered.disposition,
                    WorkDispositionV1::Committed
                        | WorkDispositionV1::Replayed
                        | WorkDispositionV1::Unaccepted
                ) {
                    return Ok(());
                }
            }
            _ => return Err(format!("exact-original resolution stopped: {resolved:?}").into()),
        }
    }
    Err("exact original unresolved after finite recovery; arm stopped".into())
}
fn main() -> Result<()> {
    assert!(
        cfg!(target_os = "linux"),
        "this driver targets native Linux only"
    );
    let args: Vec<_> = std::env::args_os().collect();
    assert_eq!(
        args.len(),
        3,
        "work_state_latency ABSENT_PROJECT PRECREATED_PROOF"
    );
    let root = Path::new(&args[1]);
    let proof = Path::new(&args[2]);
    assert!(root.is_absolute() && proof.is_absolute());
    assert!(!root.exists() && proof.is_dir());
    assert!(!root.starts_with(proof) && !proof.starts_with(root));
    let mut trace = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(proof.join("transitions.jsonl"))?;
    let mut samples = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(proof.join("samples.jsonl"))?;
    File::open(proof)?.sync_all()?;
    std::fs::create_dir(root)?;
    let setup_start = Instant::now();
    let sdk = HandbookSdkV1::open(root);
    let (initial, check) = support::setup(&sdk);
    let grant = support::activate(&sdk, &initial, &check, "latency-grant", None);
    append(
        &mut trace,
        &serde_json::json!({"stage":"setup-grant", "elapsed_ns":setup_start.elapsed().as_nanos(), "statistics":sdk.work_storage_statistics().map_err(|e|format!("{e:?}"))?}),
    )?;
    for n in 0..40 {
        let before = sdk
            .work_storage_statistics()
            .map_err(|e| format!("{e:?}"))?;
        let input = WorkRegistrationInputV1 {
            request_key: format!("latency-{n:02}"),
            item_key: format!("latency-item-{n:02}"),
            label: "L".repeat(1024),
            question: "Q".repeat(1024),
            actor: grant.terms.agent.clone(),
            expected_state: "absent".into(),
            composition: WorkCompositionV1::ordinary_leaf().into(),
            display_level: None,
        };
        let start = Instant::now();
        let prepared = match sdk.prepare_work_registration(&initial, &grant, input) {
            Ok(prepared) => prepared,
            Err(error) => {
                append(
                    &mut samples,
                    &serde_json::json!({"attempt":n,"stage":"prepare-error","total_ns":start.elapsed().as_nanos(),"error":format!("{error:?}")}),
                )?;
                return Err(format!("attempt {n} preparation failed; arm stopped").into());
            }
        };
        let prepare_ns = start.elapsed().as_nanos();
        let locator = prepared.locator().clone();
        let saving = Instant::now();
        if let Err(error) = save_original(proof, n, &locator) {
            append(
                &mut samples,
                &serde_json::json!({"attempt":n,"stage":"save-error","prepare_ns":prepare_ns,"total_ns":start.elapsed().as_nanos(),"error":error.to_string()}),
            )?;
            return Err(error);
        }
        let save_ns = saving.elapsed().as_nanos();
        file_native::take_branch_build_counts();
        let invoke_start = Instant::now();
        let execution = sdk.register_work_item(prepared);
        let invoke_ns = invoke_start.elapsed().as_nanos();
        let counts = file_native::take_branch_build_counts();
        let tracing = Instant::now();
        let trace_result = append(
            &mut trace,
            &serde_json::json!({"stage":"invoke", "attempt":n,"locator":locator,"execution":execution}),
        );
        let trace_ns = tracing.elapsed().as_nanos();
        let total_ns = start.elapsed().as_nanos();
        let after = sdk.work_storage_statistics().map_err(|e| format!("{e:?}"));
        append(
            &mut samples,
            &serde_json::json!({"attempt":n,"prepare_ns":prepare_ns,"save_ns":save_ns,"invoke_ns":invoke_ns,"trace_ns":trace_ns,"total_ns":total_ns,"execution":execution,"counts":counts,"before":before,"after":after.as_ref().ok(),"statistics_error":after.as_ref().err(),"trace_error":trace_result.as_ref().err().map(|e|e.to_string())}),
        )?;
        trace_result?;
        let after = after?;
        if execution.disposition != WorkDispositionV1::Committed {
            settle_original(&sdk, &locator, &mut trace)?;
            return Err(format!(
                "unexpected disposition on attempt {n}; arm stopped, no replacement sample"
            )
            .into());
        }
        let receipt = execution
            .outcome
            .as_ref()
            .ok_or("Committed missing receipt")?;
        assert_eq!(
            receipt
                .locator
                .canonical_json_bytes()
                .map_err(|e| format!("{e:?}"))?,
            locator
                .canonical_json_bytes()
                .map_err(|e| format!("{e:?}"))?
        );
        assert_eq!(
            after.database_publications - before.database_publications,
            5
        );
        assert_eq!(counts.positive_branches, 5);
        assert_eq!(counts.negative_branches, 5);
    }
    let maintenance_start = Instant::now();
    let mut calls = 0;
    loop {
        calls += 1;
        let result = sdk
            .maintain_work_storage(WorkMaintenanceBudgetV1::default())
            .map_err(|e| format!("{e:?}"))?;
        append(
            &mut trace,
            &serde_json::json!({"stage":"maintenance-after-arm","call":calls,"progress":result}),
        )?;
        if result.complete {
            break;
        }
    }
    append(
        &mut trace,
        &serde_json::json!({"stage":"finished","attempts":40,"maintenance_ns":maintenance_start.elapsed().as_nanos(),"statistics":sdk.work_storage_statistics().map_err(|e|format!("{e:?}"))?}),
    )?;
    Ok(())
}
