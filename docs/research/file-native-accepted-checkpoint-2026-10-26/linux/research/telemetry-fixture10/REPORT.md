# Linux telemetry fixture correction and complete workspace rerun

**PASS: build and catalog passed; one complete unfiltered workspace run passed all 31 engine tests, with 0 failed, 0 ignored and 0 filtered.** Both workspace consumers, engine main harness and library doctests completed with 0 tests. The newly generated real 80,976,318-byte cache-boundary fixture passed.

Host: `spenser-linux`, native Manjaro/ext4 `/dev/nvme0n1p2`. Execution: `2026-10-25T17:28:15.494387+00:00` through `2026-10-25T17:28:53.543240+00:00` (2026-10-25 EDT). Rust/Cargo 1.89.0.

## Exact source identity and correction

This result is **baseline-plus-patch**, not unchanged commit062. Source workspace: `/home/spenser/file-native-online-linux-20261020/source/final-engine`.

| Identity | SHA / value |
| --- | --- |
| Baseline commit association | `0628322e4b48e0efad19cdc32238a622d4aaa7a8` |
| Unchanged original SOURCE_FREEZE.json | `464dcf878410def63bd3b04d4f48f17d75418c04262cb4745ebc0e6d5e1de6b3` |
| Original src/io.rs | `1e401ad489b78e10845dac65ccc1b36ee9f99753c0870aa5a41c4da6661aa02c` |
| Corrected src/io.rs | `836c8a6ceac0f1e972bf77ab3e54be30333f28e763ff1d0ec2f1bff3b34a599d` |
| Exact unified patch | `2dc0263e4ead79cdee5c9aa7af2839ec4495d2cfcb02a6e818dad776e41b1d61` |
| Corrected 25-file code manifest | `1076b20ec662bc01a90119f15504789599fc3057f9014e9d5016c0d663fb646b` |

The baseline matched all25 imported code files immediately before editing. Exactly one byte was inserted: initialization argument1→10 inside `io::telemetry_tests::telemetry_old_suffix_vs_deferred`. All assertions and production `Store::init` minimum10 remain unchanged; the other24 files remain byte-identical to baseline. Patched source matched before/after compilation and execution. Original freeze metadata was not rewritten to pretend the corrected source was baseline.

```diff
--- a/src/io.rs
+++ b/src/io.rs
@@ -287,7 +287,7 @@
 }
 #[test]fn telemetry_old_suffix_vs_deferred(){
     let root=std::path::PathBuf::from(std::env::var_os("EXP_PACK_TEST_ROOT").unwrap()).join(unique("telemetry-body")).join("disposable").join("store");let mut setup=Metrics::default();
-    crate::store::Store::init(&root,"A","project",1,false,&mut setup).unwrap();
+    crate::store::Store::init(&root,"A","project",10,false,&mut setup).unwrap();
     let store=crate::store::Store::open(&root,&mut setup).unwrap();
     crate::lifetime::run(&store,&serde_json::json!({"operation":"setup"}),&mut setup).unwrap();
     let old:crate::model::Selector=serde_json::from_slice(&fs::read(root.join("SELECTED.json")).unwrap()).unwrap();
```

Original `io.rs` and freeze bytes are preserved as `io.rs.baseline` and `SOURCE_FREEZE.baseline.json` beside this report. `baseline-code-files.json`, `corrected-code-manifest.json`, `patch-plan.json` and `patch-applied.json` provide machine-readable correspondence.

## Scoped impact and read-only check

GitNexus registry was consulted and upstream impact requested before the edit for the exact copied source/function. It returned repository-not-found; this standalone workspace is unindexed. Graph impact is UNKNOWN, not zero. No indexing/bootstrap or unrelated repository refresh occurred. `gitnexus-impact.json` retains the exact request/error. Manual scope: one literal argument in a Rust `#[cfg(test)]` module; no production behavior/API/default/limit change.

A focused owner read-only delta/proof check confirmed the one-byte correction, unchanged assertions/engine guard/other24 files, catalog31/31, telemetry receipt, actual heavy fixture, preservation and closure. No independent reviewer or new owner was started; no independent-review claim is made. See `focused-readonly-check.json`.

## Compilation, catalog and single full execution

Cwd for all stages: `/home/spenser/file-native-online-linux-20261020/source/final-engine`. Installed caches were used with `--offline`; no retrieval, installation, upgrade or lockfile change was needed. `--no-fail-fast` ensures every workspace target and doctests are attempted even after a failure; no test-name filter or fixture substitution was used.

```text
cargo test --manifest-path /home/spenser/file-native-online-linux-20261020/source/final-engine/Cargo.toml --locked --offline --workspace --release -j2 --target-dir /var/tmp/file-native-final062-telemetry-n10-20261025-01a16fd3/target --no-run
```

```text
cargo test --manifest-path /home/spenser/file-native-online-linux-20261020/source/final-engine/Cargo.toml --locked --offline --workspace --release -j2 --target-dir /var/tmp/file-native-final062-telemetry-n10-20261025-01a16fd3/target -- --list
```

```text
cargo test --manifest-path /home/spenser/file-native-online-linux-20261020/source/final-engine/Cargo.toml --locked --offline --workspace --release -j2 --target-dir /var/tmp/file-native-final062-telemetry-n10-20261025-01a16fd3/target --no-fail-fast -- --test-threads=1 --nocapture
```

Explicit environment:

```text
EXP_PACK_TEST_ROOT=/var/tmp/file-native-final062-telemetry-n10-20261025-01a16fd3/fixtures
TMPDIR=/var/tmp/file-native-final062-telemetry-n10-20261025-01a16fd3/tmp
TMP=/var/tmp/file-native-final062-telemetry-n10-20261025-01a16fd3/tmp
TEMP=/var/tmp/file-native-final062-telemetry-n10-20261025-01a16fd3/tmp
CARGO_TERM_COLOR=never
```

CARGO_HOME remained the installed default `/home/spenser/.cargo`. Ambient experiment switches/test overrides were cleared by name; no credentials/full environment dump are retained.

| Stage | Exit | Supervised seconds | Result |
| --- | ---: | ---: | --- |
| compile | 0 | 31.504 | PASS |
| catalog | 0 | 0.647 | PASS |
| workspace-tests | 0 | 4.679 | PASS |

Actual binary listing reconciles31/31 source catalog names, with no missing/unexpected names. All31 bodies ran once in the library target. Engine main, project-consumer and research-consumer harnesses and library doctests completed successfully with0tests. Compiler warnings are retained without cleanup.

| Engine test | Outcome |
| --- | --- |
| `bounds_tests::bounded_file_and_indirection_routes_refuse` | PASS |
| `disk::codec_tests::compact_refuses_damage_before_allocating` | PASS |
| `io::telemetry_tests::telemetry_old_suffix_vs_deferred` | PASS |
| `io::telemetry_tests::telemetry_writer_observes_nested_io` | PASS |
| `lifetime::tests::legacy_owner_cannot_acquire_new_preselection_proof` | PASS |
| `lifetime::tests::stage_handle_aliases_refuse_before_truncation` | PASS |
| `lifetime::tests::terminal_caps_and_charge_are_independent_of_live_slots` | PASS |
| `lifetime::tests::v1_bytes_and_old_parser_boundary_are_preserved` | PASS |
| `pack::boundary_tests::raw_record_builder_bound` | PASS |
| `pack::tests::cached_metadata_matches_owned_allocations_on_growth_and_failure` | PASS |
| `pack::tests::decoder_never_uses_advertised_size` | PASS |
| `pack::tests::typed_keys_exclude_controls_and_active_payloads` | PASS |
| `packing::boundary_tests::catalog_count_bound` | PASS |
| `packing::boundary_tests::install_birth_no_adoption` | PASS |
| `packing::pace_tests::closed` | PASS |
| `packing::pace_tests::header` | PASS |
| `packing::pace_tests::schedule` | PASS |
| `packing::tests::caller_limit_precedes_raw_and_zstd_bodies_after_all_routes` | PASS |
| `packing::tests::constructor_reservation_evicts_before_allocating_output` | PASS |
| `packing::tests::loading_indexes_evicts_before_transient_allocation` | PASS |
| `packing::tests::mixed_capture_and_cold_warm_measurements_reconcile` | PASS |
| `packing::tests::paged_cache_identity_preserves_physical_instance` | PASS |
| `packing::tests::paged_four_routes_negative_duplicates_and_conflict` | PASS |
| `packing::tests::production_builder_preserves_near_limit_index_bytes` | PASS |
| `paged::tests::paged_exact_geometry_and_closed_keys` | PASS |
| `paged::tests::paged_maximum_keys_split_before_total_index_bound` | PASS |
| `paged::tests::paged_positive_negative_boundary_and_cache` | PASS |
| `paged::tests::paged_touched_untouched_and_malformed_refusal` | PASS |
| `physical_lifetime::boundary_tests::work_byte_bound` | PASS |
| `rolling::boundary_tests::discovery_byte_bound` | PASS |
| `rolling::boundary_tests::discovery_count_bound` | PASS |

## Fixed telemetry behavior and real boundary evidence

The previously failing test now reaches all its unchanged assertions and emits:

```text
telemetry actual-accepted-old-body=1 explicit-deferred-context=1 total-body=2 scope-restored=true
```

`packing::tests::loading_indexes_evicts_before_transient_allocation` also passes. New fixture root: `/var/tmp/file-native-final062-telemetry-n10-20261025-01a16fd3/fixtures/packing-cache-boundary-3617516-1792949331918498481-55`. Post-run inventory is **8files totaling80,976,318logical bytes**: six13,496,053-byte index files, an empty pack and lease. It was generated by this run, not restored/reused from prior evidence.

Actual emitted receipt:

```text
cache-boundary-receipt root=/var/tmp/file-native-final062-telemetry-n10-20261025-01a16fd3/fixtures/packing-cache-boundary-3617516-1792949331918498481-55 total_index_charge=69740745 transient_charged_peak=66505838 remaining_cache_entries=1 source_charge=13948149 before_finish=79524497 approved_constructor=37684082 actual_output_capacity=14351124 changed_index_refused_before_allocation=true constructor_eviction_before_output=true overflow_refused_before_allocation=true extra_fixture_bytes=0
```

Full current file-size inventory is in `boundary-evidence.json`; additional structural/paged/constructor receipts remain in full stderr.

## Compiled and executed binary bindings

All four test binaries were hashed before and after execution; hashes match. No standalone production CLI build/qualification is claimed.

| Actual test executable | Bytes | SHA-256 |
| --- | ---: | --- |
| `/var/tmp/file-native-final062-telemetry-n10-20261025-01a16fd3/target/release/deps/file_native_incremental-f5485a3108d76dc6` | 4149184 | `bc90a7eaf47eb246ea2c26a07f7a6db0fc8bbd7659f782a981d9be31ac70b790` |
| `/var/tmp/file-native-final062-telemetry-n10-20261025-01a16fd3/target/release/deps/file_native_incremental-aa2c821bde1b3abd` | 1080888 | `caa0b8b498ba775d225e2e620748fda69038a3e85e483544d77b068c42793d72` |
| `/var/tmp/file-native-final062-telemetry-n10-20261025-01a16fd3/target/release/deps/project_consumer-79daabc3bcb76399` | 1080872 | `d9b54f1cb7dbbffd42a5320d80ca4498de0979532b1a2ffb60343052ceb3107e` |
| `/var/tmp/file-native-final062-telemetry-n10-20261025-01a16fd3/target/release/deps/research_consumer-82c16566f263e0c1` | 1080880 | `1b2f58f8929470b575def8b69c49d4185f89cffebefa95bb012231fda60a8980` |

## Original failure and other work preserved

The earlier unchanged-baseline run remains at `/var/tmp/file-native-final062-buildtest-20261025-01a16fd3`: original30pass/1fail logs/result, original binary bindings and all four actual test binaries. Those selected artifacts were hashed before the patch and after the corrected run and remain unchanged. The original library-test binary SHA is `88179e5e85fe3d09ec3279595f826d02292d6614773b9262cda1b55fd00877c5`; the corrected library-test binary SHA is `bc90a7eaf47eb246ea2c26a07f7a6db0fc8bbd7659f782a981d9be31ac70b790`. The original baseline result is not relabeled a pass.

Both older CP1 native binaries remain unchanged at SHA `11c306c4fa30d74a6f83274666024591e0e150674419d1e4dbe06e9a251a34f5`. Earlier blocked report, host inventory and Sentinel assessment remain intact. No Sentinel cleanup, Git mutation/publication, other source repair, dependency change, pilot or full matrix occurred.

## Practical supervision and resource limits

Fresh target/fixture/temp/log root: `/var/tmp/file-native-final062-telemetry-n10-20261025-01a16fd3`, on writable native ext4. No previous target/heavy fixture was reused. Native available bytes before 63991111680; after 63803351040, above the20GiBfloor. Final task allocated file bytes 193548288 across693files before report copies. Retained reference targets (~130MB) and boundary (~81MB) plus1GiBworking room justified proceeding without a new all-host census.

Limits retained: `-j2`,20GiBnative free floor,3GiBaggregate new-work RSS, applicable2GiBprocess limit; finite900scompile/120slist/600scomplete-run timeouts;2GiB/100,000-file new task cap. No stop fired. Original44GiB/5million-file combined experiment ceiling was not expanded; no fresh global occupancy/qualification claim is made.

| Stage | Sampled tree + supervisor peak RSS | Sampled largest process RSS | Closure |
| --- | ---: | ---: | --- |
| compile | 1027518464 | 492556288 | two empty observations |
| catalog | 220168192 | 170389504 | two empty observations |
| workspace-tests | 117825536 | 67981312 | two empty observations |

Approximately200mssampling is functional supervision, not a40mscadence/hard-peak qualification. Owned process groups and observed descendant identities terminated; closure verified for every stage.

## Reports, exact logs and stop point

- Report: `/tmp/file-native-host-inventory-linux-20261024-01a16fd3/telemetry-n10-correction-20261025/REPORT.md`.
- Exact patch: `/tmp/file-native-host-inventory-linux-20261024-01a16fd3/telemetry-n10-correction-20261025/telemetry-n10.patch`.
- Native full result: `/var/tmp/file-native-final062-telemetry-n10-20261025-01a16fd3/result.json`.
- Compile stdout/stderr: `/var/tmp/file-native-final062-telemetry-n10-20261025-01a16fd3/logs/compile.stdout`, `/var/tmp/file-native-final062-telemetry-n10-20261025-01a16fd3/logs/compile.stderr`.
- Actual listing stdout/stderr: `/var/tmp/file-native-final062-telemetry-n10-20261025-01a16fd3/logs/catalog.stdout`, `/var/tmp/file-native-final062-telemetry-n10-20261025-01a16fd3/logs/catalog.stderr`.
- Full unfiltered test stdout/stderr: `/var/tmp/file-native-final062-telemetry-n10-20261025-01a16fd3/logs/workspace-tests.stdout`, `/var/tmp/file-native-final062-telemetry-n10-20261025-01a16fd3/logs/workspace-tests.stderr`.
- Stage `*.json` contains exact argv/cwd/exit/duration/limits/closure; `*.resources.json` contains sampled resources.
- Source/binary/catalog/boundary JSON copies and original-failure preservation readbacks are beside this report.
- `delivery.json` records supported transport discovery and coordinator completion outcome.

**Requested correction and one complete rerun are finished. Stop here for coordinator reconciliation of corrected platform source identities/results; no pilot or matrix is authorized.**
