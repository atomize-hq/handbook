# Mac final062 + fixture10 — complete workspace PASS

October25,2026. Host Mac, macOS26.6.1 arm64/APFS; installed Cargo/rustc1.89.0. Source `/private/tmp/file-native-online-macos-20261020/source/final-engine`. Output `/private/tmp/file-native-macos-inventory-20261024/final062-fixture10-20261025`.

**Functional compilation, exact31-test catalog reconciliation and one COMPLETE UNFILTERED release workspace run passed.** Engine31 passed /0failed /0ignored /0filtered /0measured. Engine-main, project-consumer, research-consumer and library-doc-test targets each completed with0tests; all five target summaries are successful. Cargo build/list/run each exited0. No pilot or matrix ran.

## Exact source and focused review

Identity is **baseline `0628322e4b48e0efad19cdc32238a622d4aaa7a8` PLUS the authorized test-fixture patch**, not unchanged062. Before editing all25 admitted files matched freeze SHA256 `464dcf878410def63bd3b04d4f48f17d75418c04262cb4745ebc0e6d5e1de6b3`. Only `src/io.rs`, `telemetry_old_suffix_vs_deferred`, changed:

```diff
-    crate::store::Store::init(&root,"A","project",1,false,&mut setup).unwrap();
+    crate::store::Store::init(&root,"A","project",10,false,&mut setup).unwrap();
```

Old io.rs SHA `1e401ad489b78e10845dac65ccc1b36ee9f99753c0870aa5a41c4da6661aa02c`; new io.rs SHA `836c8a6ceac0f1e972bf77ab3e54be30333f28e763ff1d0ec2f1bff3b34a599d`,21980bytes. All24 other admitted files stayed exactly baseline; all25 remained unchanged after patch through completion. Original source manifest/unit catalog retained unchanged with their original UNRUN wording. These fresh result files supply subsequent runtime status without rewriting frozen evidence. `baseline-source.json`, `io.rs.baseline`, `fixture.patch`, `patch-review.json`, `patched-source.json` and `source-after.json` retain exact bytes/hashes/diff.

GitNexus upstream impact was attempted before editing this symbol; standalone engine path was unregistered. Graph coverage UNKNOWN; no bootstrap. Manual focused review found test-only low-risk scope: Store::init rejects n<10; n10 is the minimum valid setup; every counter/scope assertion and production bounds remain unchanged. No structured-format/default migration or other source repair occurred. Windows later independently reported matching corrected io.rs SHA; this Mac proof derives from its own source and actual run.

## Build, listing and sole full run

Common argv: `/Users/spensermcconnell/.cargo/bin/cargo test --manifest-path /private/tmp/file-native-online-macos-20261020/source/final-engine/Cargo.toml --locked --workspace --no-fail-fast --release -j2 --target-dir /private/tmp/file-native-macos-inventory-20261024/final062-fixture10-20261025/target`.

| Stage | Suffix | Exit | Wall seconds including native closure |
|---|---|---:|---:|
| Compilation | `--no-run` |0|37.213627|
| Catalog listing | `-- --list` |0|1.276680|
| One complete execution | `-- --test-threads=1 --nocapture` |0|4.456997|

Cwd was final-engine. `CARGO_HOME` was fresh task-owned `cargo-home`; `EXP_PACK_TEST_ROOT` fresh `fixtures`; `TMPDIR`, `TMP`, `TEMP` fresh `tmp`; `CARGO_INCREMENTAL=0`, `CARGO_NET_OFFLINE=false`, `PYTHONDONTWRITEBYTECODE=1`. No credentials were logged. `logs/{build,list,test}-command.json` gives exact argv/environment names/values. Full stdout/stderr are `logs/{build,list,test}.stdout` and `.stderr`; original waits/births/groups/resources/closure are in corresponding `-child.json`, `-result.json`, `-samples.jsonl`.

The produced library list matches every one of the31 expected source-catalog test names, with no missing/extra. `catalog-reconciliation.json` records fully qualified names, `tests-result.json` all31 actual individual outcomes and five target totals. The repaired telemetry test passed and emitted `actual-accepted-old-body=1 explicit-deferred-context=1 total-body=2 scope-restored=true`. Compiler warnings are retained without unrelated cleanup. No filtered run, heavy-only run, assertion changes, substituted fixture or retry-to-pass occurred.

Four produced release test executables were hashed after listing and rechecked after execution. This is test-executable binding, not a claim that a separate production CLI binary or pilot was built/run.

| Test executable (under target/release/deps) | Bytes | SHA256 |
|---|---:|---|
| `file_native_incremental-e34927de26675ca1` | 3340448 | `93a75efeac9016f084ae50b353f4d122cb39938046ab2958ff5c032190a3cc09` |
| `file_native_incremental-0af3934fad45e678` | 937456 | `df2941946267e1726036ff4c5405daa5b2a43b3fefa61eccf0d85150f4708b22` |
| `project_consumer-185eefe826719af5` | 937456 | `47ef7c4c5e18f3838a9f7d043ab022d51c055b191a08c5e2866991f9abeffe00` |
| `research_consumer-daa897f3e2d792c0` | 937456 | `aaf859dde8328155291be5fbc88322467e55d5e3124cd555cb7621bf170e846f` |

Absolute paths are retained in `test-binary-bindings.json`.

## Real boundary fixture, once

`packing::tests::loading_indexes_evicts_before_transient_allocation` passed in the sole complete run. Its actual retained fixture root is `/private/tmp/file-native-macos-inventory-20261024/final062-fixture10-20261025/fixtures/packing-cache-boundary-28490-1792949306460418000-55`. Fresh stat-only census found **8regular files totaling exactly80,976,318bytes**; no archive was substituted or dataset hashed/copied. File geometry and actual test status are in `boundary-evidence.json`.

Actual emitted receipt:

```text
cache-boundary-receipt root=/private/tmp/file-native-macos-inventory-20261024/final062-fixture10-20261025/fixtures/packing-cache-boundary-28490-1792949306460418000-55 total_index_charge=69740745 transient_charged_peak=66505838 remaining_cache_entries=1 source_charge=13948149 before_finish=79524497 approved_constructor=37684082 actual_output_capacity=14351124 changed_index_refused_before_allocation=true constructor_eviction_before_output=true overflow_refused_before_allocation=true extra_fixture_bytes=0
```

Thus source fixture construction, changed-index refusal, constructor eviction and overflow refusal assertions actually ran on this patched native source. Receipt transient charged peak66,505,838bytes is the instrumented cache/transient charge, distinct from process RSS and not an OS hard peak.

## Dependencies and prior evidence preservation

The P-cache disappearance remains preserved in earlier reports; it was not silently repaired in place. Five exact missing archives (`cc1.9.0`, `find-msvc-tools0.1.15`, `zstd0.14.3`, `zstd-safe8.2.0`, `zstd-sys2.1.2+zstd.1.5.7`) were downloaded directly from ordinary static.crates.io after runtime network access became available. All45 archive SHA256s and selected index version/checksum records were verified against the unchanged frozen Cargo.lock, before use/after completion (`dependency-bindings.json`, `dependency-after.json`). Other exact inputs came from prior task-owned cache; no blanket cache clone/substitution/upgrade or existing cache mutation was required. Windows confirmed matching exact inputs and prepared a local fallback; no cross-host archive/service/tool installation was used.

Earlier October24 missing-exit/closure attempt remains UNVERIFIED. Subsequent blocked/failed supervisor/cache attempts remain intact; their failures are not rescored by this success. No Git stage/commit/fetch/push/merge/reset/cleanup/publication or protected corpus restoration occurred.

## Native supervision and limits

The task-owned corrected launcher reused existing native libproc identity/RSS and OwnedTree traversal. Original direct PID/birth/PGID/handle and actual waits were retained. Finite1800s stage timeout,3GiB aggregate and2GiB test-process RSS guards,20GiB Mac free floor, and finally-path owned-only bounded stop/wait remained in place. All three process groups had two valid empty closure observations, no escaped retained survivors, and no signals were needed.

| Stage | Sampled aggregate peak incl supervisor | Sampled largest child | Missing RSS observations |
|---|---:|---:|---:|
| Build |977371136|470712320|1|
| List |166133760|110870528|0|
| Test |204636160|149323776|0|

The one build observation failed RSS as that child disappeared; errno/identity absence are retained. No cap breach was observed. These are sampled functional results, **not40ms monitoring qualification, all-instants hard peaks or full experiment resource qualification**.

Initial fresh room was about122GB. Final task categories: cache38799183bytes/1538files; target89973362bytes/406files; fixtures83575018bytes/255files; tmp1090bytes/2files; logs768344bytes/18files. Free122196365312bytes, above20GiB. Actual new work fits the retained unmaterialized native cache/build/fixture/evidence forecasts;44GiB/5million-file combined ceiling and inside1GiB/100k-file reserve remain controlling. No precise fresh all-host census or transient disk peak is claimed. Prior data/reserve were not deleted/consumed to create room. `final-footprint.json` records category snapshots before this report.

## Stop state

Requested local fixture correction/dependency restoration/build/catalog/full-suite proof is complete. No product pilot or full matrix was authorized or started. This is functional validation of baseline062 plus the one test patch, not demonstration of an online-maintenance operating envelope.
