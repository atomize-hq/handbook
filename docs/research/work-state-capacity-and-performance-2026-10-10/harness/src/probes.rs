use crate::*;
use handbook_engine::local_authority::local_check::LocalCheckCurrentBasisV1;
use handbook_engine::local_authority::LocalAuthorityRefusalV1;
use sha2::{Digest, Sha256};
use std::hint::black_box;

struct Fixture {
    dir: tempfile::TempDir,
    sdk: HandbookSdkV1,
    initial: LocalInitialSetupLocatorV1,
    check: LocalCheckLocatorV1,
    basis: LocalCheckCurrentBasisV1,
    grant: WorkGrantLocatorV1,
    binding: Option<WorkKindBindingV1>,
    first: Option<WorkRequestLocatorV1>,
    history: usize,
}

impl Fixture {
    fn new(kind: bool, cohort: usize) -> Self {
        let dir = tempfile::Builder::new()
            .prefix("hcm-work-capacity-research-")
            .tempdir()
            .unwrap();
        let sdk = HandbookSdkV1::open(dir.path());
        let (initial, check) = support::setup(&sdk);
        let prepared = sdk
            .prepare_work_registration_terms(
                &initial,
                &check,
                support::terms(&check, "grant"),
                WorkTermsTransitionV1::Initial,
            )
            .unwrap();
        let grant = prepared.locator().clone();
        let start = Instant::now();
        let result = sdk.activate_work_registration_terms(prepared);
        emit(
            "initial_activation_effect",
            "sdk",
            0,
            cohort,
            elapsed(start),
            &format!("{:?}", result.disposition),
            0,
        );
        assert_eq!(
            result.disposition,
            WorkDispositionV1::Committed,
            "{result:?}"
        );
        let binding = kind.then(|| {
            support::establish_kind(&sdk, &initial, &check, "kind", "research.kind@1.0.0")
        });
        let basis = sdk.observe_local_check_basis(&initial).unwrap();
        let history = if kind { 2 } else { 1 };
        Self {
            dir,
            sdk,
            initial,
            check,
            basis,
            grant,
            binding,
            first: None,
            history,
        }
    }
    fn path(&self) -> &Path {
        self.dir.path()
    }
    fn add(&mut self, pending: bool) -> WorkRequestLocatorV1 {
        let key = format!("item-{}", self.history);
        let prepared = self
            .sdk
            .prepare_work_registration(
                &self.initial,
                &self.grant,
                input(&self.grant, self.binding.as_ref(), &key),
            )
            .unwrap();
        let original = prepared.locator().clone();
        let result = if pending {
            Native::register_with_boundary_for_test(
                self.path(),
                prepared,
                interrupt(Boundary::Intent),
            )
        } else {
            self.sdk.register_work_item(prepared)
        };
        assert_eq!(
            result.disposition,
            if pending {
                WorkDispositionV1::Uncertain
            } else {
                WorkDispositionV1::Committed
            },
            "{result:?}"
        );
        self.history += 1;
        self.first.get_or_insert_with(|| original.clone());
        original
    }
    fn populate(&mut self, history: usize) {
        while self.history < history {
            self.add(false);
        }
    }
}

fn interrupt(at: Boundary) -> impl FnMut(Boundary) -> Result<(), LocalAuthorityRefusalV1> {
    move |stage| {
        if stage == at {
            Err(LocalAuthorityRefusalV1 {
                code: "research_interruption".into(),
                detail: "supplied isolated interruption at real durable boundary".into(),
                unresolved_request: None,
            })
        } else {
            Ok(())
        }
    }
}

fn repeat(f: &Fixture, op: &str, api: &str, cohort: usize, mut probe: impl FnMut()) {
    probe(); // One untimed warmup per same-store, same-operation group.
    let b = work_bytes(f.path());
    for sample in 0..5 {
        let start = Instant::now();
        probe();
        emit(
            op,
            api,
            f.history,
            cohort * 5 + sample,
            elapsed(start),
            "verified",
            b,
        );
    }
}

pub fn baseline() {
    for kind in [false, true] {
        for cohort in 0..1 {
            let mut f = Fixture::new(kind, cohort);
            for target in [2, 3, 8, 16, 31, 32] {
                f.populate(target);
                let suffix = if kind { "kind" } else { "explicit" };
                if let Some(original) = &f.first {
                    for api in ["native", "sdk"] {
                        repeat(&f, &format!("exact_item_{suffix}"), api, cohort, || {
                            let r = if api == "native" {
                                Native::resolve_request(f.path(), original)
                            } else {
                                f.sdk.resolve_work_registration_request(original)
                            };
                            assert_eq!(r.disposition, WorkDispositionV1::Replayed, "{r:?}");
                        });
                        repeat(&f, &format!("current_item_{suffix}"), api, cohort, || {
                            let r = if api == "native" {
                                Native::current_item(f.path(), &f.grant, &original.input.item_key)
                            } else {
                                f.sdk
                                    .resolve_current_work_item(&f.grant, &original.input.item_key)
                            }
                            .unwrap();
                            assert!(matches!(r, WorkCurrentItemV1::Open(_)));
                        });
                        repeat(
                            &f,
                            &format!("recover_committed_{suffix}"),
                            api,
                            cohort,
                            || {
                                let r = if api == "native" {
                                    Native::recover_request(f.path(), original)
                                } else {
                                    f.sdk.recover_work_registration_request(original)
                                };
                                assert_eq!(r.disposition, WorkDispositionV1::Replayed, "{r:?}");
                            },
                        );
                    }
                }
                for api in ["native", "sdk"] {
                    repeat(&f, &format!("current_terms_{suffix}"), api, cohort, || {
                        let r = if api == "native" {
                            Native::resolve_current_terms(f.path(), &f.grant, &f.basis)
                        } else {
                            f.sdk
                                .resolve_current_work_registration_terms(&f.initial, &f.grant)
                        };
                        assert!(r.is_ok(), "{r:?}");
                    });
                }
                let key = format!("item-{target}");
                let b = work_bytes(f.path());
                let start = Instant::now();
                let p = Native::prepare_registration(
                    f.path(),
                    &f.grant,
                    &f.basis,
                    input(&f.grant, f.binding.as_ref(), &key),
                )
                .unwrap();
                emit(
                    &format!("prepare_registration_{suffix}"),
                    "native",
                    target,
                    cohort,
                    elapsed(start),
                    "prepared",
                    b,
                );
                drop(p);
                let start = Instant::now();
                let p = f
                    .sdk
                    .prepare_work_registration(
                        &f.initial,
                        &f.grant,
                        input(&f.grant, f.binding.as_ref(), &key),
                    )
                    .unwrap();
                emit(
                    &format!("prepare_registration_{suffix}"),
                    "sdk",
                    target,
                    cohort,
                    elapsed(start),
                    "prepared",
                    b,
                );
                let original = p.locator().clone();
                let start = Instant::now();
                let mut boundaries = Vec::new();
                let native = matches!(target, 16 | 31);
                let r = if native {
                    Native::register_with_boundary_for_test(f.path(), p, |at| {
                        boundaries.push((format!("{at:?}"), elapsed(start)));
                        Ok(())
                    })
                } else {
                    f.sdk.register_work_item(p)
                };
                let ms = elapsed(start);
                emit(
                    &format!("registration_{suffix}_effect"),
                    if native {
                        "native_boundary_observation"
                    } else {
                        "sdk"
                    },
                    target,
                    cohort,
                    ms,
                    &format!("{:?}", r.disposition),
                    b,
                );
                if native {
                    println!(
                        "{}",
                        json!({"boundary_observation":{"composition":suffix,"history_slots_at_entry":target,"elapsed_ms":ms,"boundaries":boundaries}})
                    );
                }
                if target < 32 {
                    assert_eq!(r.disposition, WorkDispositionV1::Committed, "{r:?}");
                    f.history += 1;
                    f.first.get_or_insert(original);
                } else {
                    assert_eq!(r.disposition, WorkDispositionV1::Refused, "{r:?}");
                    assert_eq!(r.refusal.unwrap().code, "work_capacity");
                    if kind && cohort == 0 {
                        components(&f);
                        concurrent_reads(&f);
                    }
                    fresh_revoke(&f, cohort);
                }
            }
        }
    }
}

pub fn administration() {
    let mut f = Fixture::new(false, 0);
    for target in [2, 16, 31, 32] {
        f.populate(target);
        let b = work_bytes(f.path());
        let mut terms = support::terms(&f.check, &format!("scope-successor-{target}"));
        if let WorkRegistrationScopeV1::Explicit(scope) = &mut terms.scope {
            scope.clauses[0].label_max_bytes = if target == 16 { 256 } else { 128 };
        }
        terms.approved_content_fingerprint = terms.fingerprint_approved_content().unwrap();
        let start = Instant::now();
        let p = f
            .sdk
            .prepare_work_registration_terms(
                &f.initial,
                &f.check,
                terms,
                WorkTermsTransitionV1::ScopeChange(&f.grant),
            )
            .unwrap();
        emit(
            "scope_change_prepare",
            "sdk",
            target,
            0,
            elapsed(start),
            "prepared",
            b,
        );
        let start = Instant::now();
        let r = f.sdk.activate_work_registration_terms(p);
        emit(
            "scope_change_effect",
            "sdk",
            target,
            0,
            elapsed(start),
            &format!("{:?}", r.disposition),
            b,
        );
        assert_eq!(
            r.disposition,
            if target == 32 {
                WorkDispositionV1::Refused
            } else {
                WorkDispositionV1::Committed
            },
            "{r:?}"
        );
        if let Some(grant) = r.outcome {
            f.grant = grant.locator;
            f.history += 1;
        }
    }
    fresh_revoke(&f, 0);
    for target in [2, 31] {
        let mut f = Fixture::new(false, 0);
        f.populate(target);
        let original = fresh_revoke(&f, 0);
        f.history += 1;
        repeat(&f, "recover_committed_revocation", "sdk", 0, || {
            assert_eq!(
                f.sdk
                    .recover_work_registration_revocation_request(&original)
                    .disposition,
                WorkDispositionV1::Replayed
            )
        });
    }
    let mut f = Fixture::new(false, 0);
    f.populate(2);
    support::govern(&f.sdk, &f.initial);
    let b = work_bytes(f.path());
    let start = Instant::now();
    let p = f
        .sdk
        .prepare_work_registration_terms(
            &f.initial,
            &f.check,
            support::terms(&f.check, "grant-G"),
            WorkTermsTransitionV1::EstablishFromDeferred(&f.grant),
        )
        .unwrap();
    emit(
        "deferred_to_governed_prepare",
        "sdk",
        2,
        0,
        elapsed(start),
        "prepared",
        b,
    );
    let start = Instant::now();
    let r = f.sdk.activate_work_registration_terms(p);
    emit(
        "deferred_to_governed_effect",
        "sdk",
        2,
        0,
        elapsed(start),
        &format!("{:?}", r.disposition),
        b,
    );
    assert_eq!(r.disposition, WorkDispositionV1::Committed, "{r:?}");
}

fn fresh_revoke(f: &Fixture, cohort: usize) -> WorkRevokeLocatorV1 {
    let b = work_bytes(f.path());
    let start = Instant::now();
    let p = f
        .sdk
        .prepare_work_registration_revocation(
            &f.grant,
            "revoke",
            &f.grant.terms.revocation_principal,
            "human.revoke",
            "Revoke this exact supplied work grant",
        )
        .unwrap();
    emit(
        "revocation_prepare",
        "sdk",
        f.history,
        cohort,
        elapsed(start),
        "prepared",
        b,
    );
    let start = Instant::now();
    let r = f.sdk.revoke_work_registration_terms(&p);
    emit(
        "revocation_effect",
        "sdk",
        f.history,
        cohort,
        elapsed(start),
        &format!("{:?}", r.disposition),
        b,
    );
    assert_eq!(
        r.disposition,
        if f.history == 32 {
            WorkDispositionV1::Refused
        } else {
            WorkDispositionV1::Committed
        },
        "{r:?}"
    );
    if f.history == 32 {
        assert_eq!(r.refusal.unwrap().code, "work_capacity");
    }
    p
}

pub fn pending() {
    for cohort in 0..1 {
        let mut f = Fixture::new(true, cohort);
        f.populate(30);
        for target in [31, 32] {
            let original = f.add(true);
            let b = work_bytes(f.path());
            repeat(&f, "exact_pending_kind", "sdk", cohort, || {
                assert_eq!(
                    f.sdk
                        .resolve_work_registration_request(&original)
                        .disposition,
                    WorkDispositionV1::Pending
                )
            });
            let start = Instant::now();
            let r = f.sdk.recover_work_registration_request(&original);
            emit(
                "recover_pending_kind",
                "sdk",
                target,
                cohort,
                elapsed(start),
                &format!("{:?}", r.disposition),
                b,
            );
            assert_eq!(r.disposition, WorkDispositionV1::Committed, "{r:?}");
        }
        let mut f = Fixture::new(true, cohort);
        f.populate(31);
        let original = f.add(true);
        support::revoke_check(&f.sdk, &f.initial, &f.check);
        let b = work_bytes(f.path());
        let start = Instant::now();
        let r = f.sdk.recover_work_registration_request(&original);
        emit(
            "recover_pending_noncommit_kind",
            "sdk",
            32,
            cohort,
            elapsed(start),
            &format!("{:?}", r.disposition),
            b,
        );
        assert_eq!(r.disposition, WorkDispositionV1::NonCommitted, "{r:?}");
        repeat(&f, "exact_noncommit_kind", "sdk", cohort, || {
            assert_eq!(
                f.sdk
                    .resolve_work_registration_request(&original)
                    .disposition,
                WorkDispositionV1::NonCommitted
            )
        });
    }
}

fn all_files(path: &Path, files: &mut Vec<std::path::PathBuf>) {
    if path.is_file() {
        files.push(path.to_owned());
    } else {
        for e in fs::read_dir(path).unwrap() {
            all_files(&e.unwrap().path(), files);
        }
    }
}

fn components(f: &Fixture) {
    let mut paths = Vec::new();
    all_files(&f.path().join(".handbook/work-state"), &mut paths);
    paths.sort();
    let records = paths
        .iter()
        .filter(|p| {
            p.extension().is_some_and(|e| e == "json")
                || p.file_name().is_some_and(|n| n == "terminal.pending")
        })
        .map(|p| fs::read(p).unwrap())
        .collect::<Vec<_>>();
    let total = records.iter().map(Vec::len).sum::<usize>();
    for op in [
        "regular_file_read",
        "sha256_bytes",
        "serde_json_parse",
        "canonicalize_values",
    ] {
        let values = records
            .iter()
            .map(|r| serde_json::from_slice::<serde_json::Value>(r).unwrap())
            .collect::<Vec<_>>();
        for sample in 0..6 {
            let start = Instant::now();
            match op {
                "regular_file_read" => {
                    for p in &paths {
                        black_box(fs::read(p).unwrap());
                    }
                }
                "sha256_bytes" => {
                    for r in &records {
                        black_box(Sha256::digest(r));
                    }
                }
                "serde_json_parse" => {
                    for r in &records {
                        black_box(serde_json::from_slice::<serde_json::Value>(r).unwrap());
                    }
                }
                _ => {
                    for v in &values {
                        black_box(serde_json_canonicalizer::to_vec(v).unwrap());
                    }
                }
            }
            let ms = elapsed(start);
            if sample > 0 {
                emit(
                    op,
                    "component_surrogate",
                    32,
                    sample - 1,
                    ms,
                    "completed",
                    total as u64,
                );
            }
        }
    }
    let intent = paths
        .iter()
        .find(|p| {
            p.file_name().is_some_and(|n| n == "intent.json")
                && p.parent()
                    .unwrap()
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .starts_with("item-")
        })
        .unwrap_or_else(|| {
            paths
                .iter()
                .find(|p| p.file_name().is_some_and(|n| n == "intent.json"))
                .unwrap()
        });
    let raw = fs::read(intent).unwrap();
    for count in [32, 128, 512, 2048] {
        for sample in 0..6 {
            let start = Instant::now();
            for _ in 0..count {
                let value: serde_json::Value = serde_json::from_slice(&raw).unwrap();
                let canonical = serde_json_canonicalizer::to_vec(&value).unwrap();
                black_box(Sha256::digest(&canonical));
            }
            let ms = elapsed(start);
            if sample > 0 {
                emit(
                    "streamed_parse_canonical_hash",
                    "synthetic_repeat",
                    count,
                    sample - 1,
                    ms,
                    "completed",
                    (raw.len() * count) as u64,
                );
            }
        }
    }
    let value: serde_json::Value = serde_json::from_slice(&raw).unwrap();
    println!(
        "{}",
        json!({"component_metadata":{"regular_files":paths.len(),"json_records":records.len(),"json_record_bytes":total,"synthetic_intent_bytes":raw.len(),"authority_proof_json_bytes":serde_json::to_vec(&value["authority"]).unwrap().len(),"operation_locator_json_bytes":serde_json::to_vec(&value["operation"]["locator"]).unwrap().len(),"maximum_json_record_bytes":records.iter().map(Vec::len).max(),"synthetic_memory":"one parsed intent at a time; input reused","synthetic_constraint":"does not run native authority, completeness, root, lock, request identity or durable proof"}})
    );
}

fn concurrent_reads(f: &Fixture) {
    let original = f.first.as_ref().unwrap();
    for attempt in 0..3 {
        let barrier = std::sync::Arc::new(std::sync::Barrier::new(2));
        let mut workers = Vec::new();
        for worker in 0..2 {
            let barrier = barrier.clone();
            let root = f.path().to_owned();
            let grant = f.grant.clone();
            let key = original.input.item_key.clone();
            workers.push(
                std::thread::Builder::new()
                    .stack_size(32 * 1024 * 1024)
                    .spawn(move || {
                        barrier.wait();
                        let start = Instant::now();
                        let result = Native::current_item(root, &grant, &key);
                        (
                            worker,
                            elapsed(start),
                            match result {
                                Ok(WorkCurrentItemV1::Open(_)) => "Open".into(),
                                Err(e) => e.code,
                                other => panic!("{other:?}"),
                            },
                        )
                    })
                    .unwrap(),
            );
        }
        for w in workers {
            let (worker, ms, outcome) = w.join().unwrap();
            emit(
                "concurrent_current_item_kind",
                "native_two_workers",
                32,
                attempt * 2 + worker,
                ms,
                &outcome,
                work_bytes(f.path()),
            );
        }
    }
}
