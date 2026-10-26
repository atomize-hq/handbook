# Final engine Windows build/tests — FAIL (2026-10-24)

Native host: SPENSERSDESKTOP, Windows/NTFS. Source HEAD `0628322e4b48e0efad19cdc32238a622d4aaa7a8`; retained freeze `464dcf878410def63bd3b04d4f48f17d75418c04262cb4745ebc0e6d5e1de6b3`. All30 freeze files matched before and after every stage; checkout remained clean. Four compiled test executables retained identical hashes before/after execution. Installed Cargo1.89.0/rustc1.89.0, locked dependencies, offline existing global cache, release profile, j2, no incremental build. No retrieval/install/upgrade or lockfile/source changes were needed.

Compilation PASS; full engine test run FAIL: **30 passed,1 failed,0 ignored,0 measured,0 filtered**. All31 engine test bodies actually executed once. Real80,976,318-byte cache-boundary fixture PASSED. No source/test repairs, reduced fixtures, filters, repeated tests, pilot or matrix were run.

## Source, command and outputs

Source/cwd:
`C:/Users/spmcc/.codex/worktrees/file-native-online-cp2-20261020/handbook/docs/research/file-native-incremental-engine-experiment-2026-10-18`.

All output paths below are relative to this absolute task output root:
`C:/Users/spmcc/.codex/visualizations/2026/10/24/01a16fd3-b2c6-7af1-b036-fd72bdba257d/final-engine-build-tests`.

Common exact argv prefix:
`C:/Users/spmcc/.cargo/bin/cargo.exe test --manifest-path <source>/Cargo.toml --locked --workspace --release -j2 --target-dir <output>/target`.

| Stage | Added argv | Exit | Time/outcome |
| --- | --- | --- | --- |
| Compilation | `--no-run` |0| Cargo43.68s; supervisor126.531s including retained MSVC telemetry helper closure. Four workspace test executables produced. |
| Catalog | `-- --list` |0|0.781s; exactly31 library tests; zero tests in engine main/project/research harnesses and doctests. |
| Single unfiltered execution | `-- --test-threads=1 --nocapture` |101| Supervisor1.812s; Rust harness1.62s;30/31 passed. |

Catalog names reconcile exactly with the retained freeze's31 declared source-test names; `catalog-reconciliation.json` retains every actual full name. Every executed case and outcome is in `logs/test.stdout.log`. Cargo stopped after the failed library harness, so the three zero-test binary/consumer harnesses and zero-test doctest stage were compiled/listed but **not executed by the test invocation**. No complete-workspace PASS is claimed, and no supplemental run was used to bypass this failure.

Exact per-stage argv/cwd/environment override names, timeout and caps are retained in `compile-command.json`, `list-command.json`, `test-command.json`. `EXP_PACK_TEST_ROOT=<output>/fixtures`; `TEMP`, `TMP`, `TMPDIR=<output>/temp`; `CARGO_INCREMENTAL=0`; `CARGO_NET_OFFLINE=true`. Inherited experiment injection, compiler-wrapper/alternate compiler/cache/target/flag settings were removed by name without logging their values. Old binaries/targets and dirty maintenance/matrix work were preserved.

## Concrete failure

`io::telemetry_tests::telemetry_old_suffix_vs_deferred` panicked at `src/io.rs:290:71`:

```text
called `Result::unwrap()` on an `Err` value: "refused:init-arguments-existing"
```

Read-only source inspection identifies a direct setup mismatch: the test calls `Store::init(&root,"A","project",1,false,...)`; `src/store.rs:410` rejects `n<10`. The failed telemetry setup uses n=1 and therefore refuses before its intended historical-body/deferred telemetry assertions. This is a concrete source-test setup defect; no inference of an existing dirty store or path-length failure is needed. The source and test were not changed, and the test was not rerun.

## Real boundary evidence

`packing::tests::loading_indexes_evicts_before_transient_allocation ... ok` appears in the actual unfiltered run. Its emitted stderr receipt:

```text
total_index_charge=69740745 transient_charged_peak=66505862 remaining_cache_entries=1 source_charge=13948149 before_finish=79524513 approved_constructor=37684082 actual_output_capacity=14351124 changed_index_refused_before_allocation=true constructor_eviction_before_output=true overflow_refused_before_allocation=true extra_fixture_bytes=0
```

Retained actual fixture root:
`<output>/fixtures/packing-cache-boundary-6652-1792891587773668200-11`.
Its packs directory contains **six actual13,496,053-byte .idx files and one zero-byte shared .pack**, summing **80,976,318 bytes**. The six indexes are five newly built source indexes and the changed-index negative-control copy. The real files remain retained; no old archive or reduced fixture substituted. `boundary-fixture.json` records names/sizes and the complete emitted receipt. Other actual emitted builder/catalog/Work/discovery/paged boundary receipts remain in stderr.

The charged cache/transient peak in the engine receipt is66,505,862B (<64MiB67,108,864B). This is the test's exercised internal accounting assertion, not an independently proved all-instant process-memory maximum.

## Source-to-binary bindings

Four executables under `<output>/target/release/deps`:

| Executable | Bytes | SHA256 |
| --- | --- | --- |
| file_native_incremental-59093b51f757fd16.exe (31-test library) |3,543,552|78551ab2a06b876aa13b7f921620e8df3ec5fdc1c7d3c0d7a020a6b409ed1b42|
| file_native_incremental-50bbacae6623bd61.exe (main harness) |745,472|a235ef86e860912288ad62e66e2177e016f4daaebed87a39e5edc95bc42b7b30|
| project_consumer-7e76476ff57557fb.exe |745,472|31c1221983d2c8dd4714a7b11cac22208882e7ab72a64e1d1ef87759e2813cba|
| research_consumer-9e66335d7f3c0dde.exe |745,472|445ab3a007e06a7fa231166f09d6a064b996cda5c90d97c78bfbad431e80bca2|

These are fresh final-source test artifacts, separate from the earlier20-test compile-only binaries. `test-binaries-before.json` and `test-binaries-after.json` bind absolute paths, sizes, hashes and unchanged state. Each `*-before-source.json` / `*-after-source.json` binds HEAD, clean status and all30 actual source/control file hashes to the exact original freeze.

## Supervision, resources and termination

Fresh target/fixtures/temp/log directories only. `resource-check.md` records the practical room check and use of retained, unmaterialized Windows work forecast without increasing44GiB/5million-file ceilings, touching the inside reserve, deleting actual data, or claiming a fresh all-host census. The future full matrix must reconcile these retained actuals before using the same forecast again.

Each native Cargo root was created suspended, assigned to a Windows Job Object, then resumed. Job kill-on-close and4GiB commit cap, approximate250ms RSS polling including the supervisor,2GiB test-process commit/RSS cap, j2, finite900s compile/test and120s catalog timeouts,600MiB/8000file task stop and140GiB C: free-space floor were in effect. No timeout, memory/output/free-space stop fired. Full stdout/stderr were written directly to retained files; no output truncation was imposed by the execution scripts.

Observed sampled aggregate peaks: compile955,179,008B; list101,126,144B; tests126,697,472B, including supervisor. Test sampled largest process66,486,272B. These are functional run supervision observations, **not40ms monitoring or hard peak qualification**. Final test-stage task inventory before report additions was244,679,617B/714files; C: free180,440,756,224B (>140GiB).

MSVC spawned one owned `VCTIP.EXE` helper that remained after Cargo compilation completed. It was the sole retained compile Job member and was closed after its image/start/native process handle was checked; details are in `compiler-helper-closure.json`. Original Cargo exit0 was retained. All three stage records show `job_empty=true`; each Job and native process/file handles were closed. No owned build/test process remains running. All prior experiment failures, fixtures, targets and reports remain intact.

## Complete retained logs

- `logs/compile.stdout.log`, `logs/compile.stderr.log`, `logs/compile.samples.jsonl`
- `logs/list.stdout.log`, `logs/list.stderr.log`, `logs/list.samples.jsonl`
- `logs/test.stdout.log`, `logs/test.stderr.log`, `logs/test.samples.jsonl`
- `compile-result.json`, `list-result.json`, `test-result.json`
- `catalog-reconciliation.json`, `boundary-fixture.json`, `test-binaries-before.json`, `test-binaries-after.json`, all six source-verification snapshots, `resource-check.md`, `compiler-helper-closure.json`, and task-owned `run_stage.py`.

Compilation and the real heavy boundary passed; final engine functional qualification fails on the one setup defect above. No automated follow-on experiment is started.
