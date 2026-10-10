//! Isolated, supplied test authority; never opens a retained user project.
#[allow(dead_code, unused_imports)]
#[path = "../../../../../crates/sdk/tests/work_kind_support/mod.rs"]
mod support;

use handbook_engine::work_state::{NativeWorkStateV1 as Native, WorkWriteBoundaryV1 as Boundary};
use handbook_sdk::local_check::*;
use handbook_sdk::local_setup::*;
use handbook_sdk::work_state::*;
use handbook_sdk::HandbookSdkV1;
use serde_json::json;
use std::{fs, path::Path, time::Instant};
mod probes;

fn main() {
    std::thread::Builder::new()
        .name("isolated-work-state-measurement".into())
        .stack_size(32 * 1024 * 1024)
        .spawn(run)
        .unwrap()
        .join()
        .unwrap();
}

fn emit(op: &str, api: &str, history: usize, sample: usize, ms: f64, outcome: &str, bytes: u64) {
    if api == "synthetic_repeat" {
        println!(
            "{}",
            json!({"operation":op,"api":api,"synthetic_record_count":history,
            "sample":sample,"elapsed_ms":ms,"outcome":outcome,"processed_input_bytes":bytes})
        );
        return;
    }
    println!(
        "{}",
        json!({"operation":op,"api":api,"history_slots_at_entry":history,
        "sample":sample,"elapsed_ms":ms,"outcome":outcome,"retained_work_bytes_at_entry":bytes})
    );
}

fn elapsed(start: Instant) -> f64 {
    start.elapsed().as_secs_f64() * 1000.0
}

fn bytes(path: &Path) -> u64 {
    if path.is_file() {
        return fs::metadata(path).unwrap().len();
    }
    fs::read_dir(path)
        .unwrap()
        .map(|e| bytes(&e.unwrap().path()))
        .sum()
}

fn work_bytes(path: &Path) -> u64 {
    bytes(&path.join(".handbook/work-state"))
}

fn input(
    grant: &WorkGrantLocatorV1,
    binding: Option<&WorkKindBindingV1>,
    key: &str,
) -> WorkRegistrationInputV1 {
    WorkRegistrationInputV1 {
        request_key: key.into(),
        item_key: key.into(),
        label: "Leaf".into(),
        question: "Exact bounded work question".into(),
        actor: grant.terms.agent.clone(),
        expected_state: "absent".into(),
        composition: binding.map_or_else(
            || WorkCompositionV1::ordinary_leaf().into(),
            |b| WorkCompositionInputV1::kind(b.clone(), None),
        ),
        display_level: None,
    }
}

fn run() {
    let selected = std::env::args().nth(1).unwrap_or_else(|| "all".into());
    if selected == "all" || selected == "baseline" {
        probes::baseline();
    }
    if selected == "all" || selected == "administration" {
        probes::administration();
    }
    if selected == "all" || selected == "pending" {
        probes::pending();
    }
    if !["all", "baseline", "administration", "pending"].contains(&selected.as_str()) {
        panic!("usage: work-state-capacity-research [all|baseline|administration|pending]");
    }
}
