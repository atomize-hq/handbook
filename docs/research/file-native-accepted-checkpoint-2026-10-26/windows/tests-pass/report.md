# Windows corrected final-engine build/tests — PASS

Date: October25,2026. Host: SPENSERSDESKTOP, native Windows/NTFS. **Compilation,31 engine tests, three binary/consumer test harnesses and library doctests PASS**. Exactly one complete unfiltered corrected-source invocation was run with `--no-fail-fast`;0failed/0ignored/0filtered. The real80,976,318-byte cache-boundary fixture was created and passed. Previous failure and binaries remain preserved. No pilot/matrix or further repair was run.

## Exact source identity and scope

Source/cwd: `C:/Users/spmcc/.codex/worktrees/file-native-online-cp2-20261020/handbook/docs/research/file-native-incremental-engine-experiment-2026-10-18`.

**Identity is baseline0628322e4b48e0efad19cdc32238a622d4aaa7a8 PLUS the one-line telemetry fixture patch**, not unchanged062. Original freeze remains `464dcf878410def63bd3b04d4f48f17d75418c04262cb4745ebc0e6d5e1de6b3`.

Only `src/io.rs:290`, inside `#[cfg(test)] telemetry_old_suffix_vs_deferred`, changed:

```diff
-    crate::store::Store::init(&root,"A","project",1,false,&mut setup).unwrap();
+    crate::store::Store::init(&root,"A","project",10,false,&mut setup).unwrap();
```

Corrected io.rs:21,980bytes; SHA256 `836c8a6ceac0f1e972bf77ab3e54be30333f28e763ff1d0ec2f1bff3b34a599d`.
Original io.rs SHA256 `1e401ad489b78e10845dac65ccc1b36ee9f99753c0870aa5a41c4da6661aa02c`.
`telemetry-fixture10.patch` SHA256 `26bb447503a79c16d54871576341d2b134e6dbdf3042ebffae3b131f4b84ddd4`.

HEAD, dirty/index state and source were rebound before editing. Original source was copied to `io.rs.baseline`. Byte-level verification requires corrected bytes to equal original bytes with exactly that single replacement. All29 other freeze files remain identical; all30 corrected expected file hashes match before/after compile, catalog and execution. Production min10, all test assertions and lockfile are unchanged. Only the intended io.rs worktree modification exists; index entry and staged state are unchanged. No commit/push/merge was performed.

GitNexus list/impact used the already installed direct CLI1.6.12 BEFORE editing; the exact checkout was not registered and returned repository-not-found. No bootstrap/indexing occurred. Automated graph scope is UNKNOWN, not a zero-impact finding. Manual read-only scope: Rust's test harness calls this private cfg(test) function; no production caller or production behavior changes. The sole change supplies a valid count to the existing minimum10 initializer. Scoped diff and byte/hash checks found no adjacent changes. Same patch/hash and a focused independent correspondence check were sent to the existing Mac session; no new owner/review campaign was created.

## Commands and actual outcomes

Output root for all relative paths below:
`C:/Users/spmcc/.codex/visualizations/2026/10/24/01a16fd3-b2c6-7af1-b036-fd72bdba257d/final062-fixture10-20261025`.

Installed Cargo1.89.0/rustc1.89.0; ordinary existing global cache, locked and offline; fresh target/fixtures/temp. Exact argv and controlled environment names are retained in `*-command.json`.

Common command:
`C:/Users/spmcc/.cargo/bin/cargo.exe test --manifest-path <source>/Cargo.toml --locked --workspace --release -j2 --target-dir <output>/target`.

| Stage | Added arguments | Exit | Supervisor duration |
| --- | --- | --- | --- |
| Compile | `--no-run` |0|42.078s|
| Catalog | `-- --list` |0|0.781s|
| Full corrected workspace ONCE | `--no-fail-fast -- --test-threads=1 --nocapture` |0|2.625s; engine harness1.98s|

Catalog reconciles exactly31 full test names against the retained frozen source catalog (`catalog-reconciliation.json`). Actual engine results: **31passed,0failed,0ignored,0measured,0filtered**. Engine main, project consumer, research consumer and library doctests each ran and returned0passed/0failed/0ignored/0filtered. The stdout contains five successful result blocks; stderr names all four executables and the doctest stage. The formerly failing telemetry test passed and emitted `telemetry actual-accepted-old-body=1 explicit-deferred-context=1 total-body=2 scope-restored=true`.

Environment: `EXP_PACK_TEST_ROOT=<output>/fixtures`; TEMP/TMP/TMPDIR point to `<output>/temp`; CARGO_INCREMENTAL=0; CARGO_NET_OFFLINE=true. Inherited experiment injections/alternate compiler-wrapper/flag/cache/target settings were removed by name. No dependency retrieval, upgrade, install or lockfile mutation was needed on Windows. All output was retained without tool-script truncation.

## Boundary and binary proof

Actual passing test: `packing::tests::loading_indexes_evicts_before_transient_allocation`.
Retained fresh fixture: `<output>/fixtures/packing-cache-boundary-22992-1792949448873174900-55`.
Its packs directory has six actual13,496,053-byte indexes (five source indexes and changed-index negative copy) plus one zero-byte shared pack: **80,976,318bytes**, checked from current retained files. No old archive or reduced fixture substituted.

Actual emitted receipt fields:
`total_index_charge=69740745 transient_charged_peak=66505862 remaining_cache_entries=1 source_charge=13948149 before_finish=79524513 approved_constructor=37684082 actual_output_capacity=14351124 changed_index_refused_before_allocation=true constructor_eviction_before_output=true overflow_refused_before_allocation=true extra_fixture_bytes=0`.

`boundary-fixture.json` binds actual names/sizes and complete emitted receipt. Internal charged peak66,505,862B is below64MiB67,108,864B; this is an exercised accounting assertion, not an independently proved all-instant process peak.

Fresh test executables under `<output>/target/release/deps`; all four hashes unchanged before/after tests:

| Executable | Bytes | SHA256 |
| --- | --- | --- |
| file_native_incremental-59093b51f757fd16.exe (31-test library) |3543552|8e44ed93ec2fd72f2fdd6f7b48965bdfc9e3ee41492b7005575622c7d299713d|
| file_native_incremental-50bbacae6623bd61.exe (main harness) |745472|7ac6051f182e4e8b8b8b6d1ac1a7260ca227f317bd5126bed721b699df47584a|
| project_consumer-7e76476ff57557fb.exe |745472|9c1f9305cbd7258cfe63851133dd230aead9af2adefaadde9ef05ef686bbc000|
| research_consumer-9e66335d7f3c0dde.exe |745472|3fc0ac0ee03daea022c9f53724986bd64643b9a421236c3f621e489057753f8f|

## Supervision, room and closure

Native Windows Job containment: create Cargo suspended, assign Job before resume, kill-on-close;4GiB Job commit cap, sampled aggregate RSS check including supervisor;2GiB test-process cap; j2; finite900s compile/test,120s catalog timeouts;140GiB C: free-space floor. Approximate250ms polling is **not40ms qualification or a hard RSS maximum**. No resource/timeout stop occurred.

Prior retained Windows build/test output occupies244,704,631B/720files. Retained F11 unmaterialized Windows work allowance676,345,051B/8000files leaves431,640,420B for this new task output, including dependency transport and source/report files; the monitor uses that actual remaining room, not a resurrected arbitrary old subcap. New final test-stage task actual247,144,566B/783files fits. Prior+new≈491.85MB remains below the held allowance;44GiB/5million combined ceilings and1GiB/100000 inside reserve were not increased or consumed as extra headroom. No protected cleanup/corpus restore or actual deletion credit. No fresh all-host physical census is claimed. Future matrix must reconcile these retained actuals.

C: free at test end179,635,167,232B (>140GiB). Sampled aggregate peaks: compile952,709,120B; catalog69,525,504B; tests150,441,984B. Test sampled largest process89,071,616B. All3 Job records end `job_empty=true`; native handles closed. The build's sole remaining owned VCTIP helper was closed only after original Cargo exited and exact helper image/Job membership were verified. Owned build/test work is terminated. Old failure/binaries/fixtures and dirty maintenance/matrix work remain preserved.

## Exact dependency supply for Mac

Five archives and their exact original sparse-index version/checksum JSON records were found in the existing Windows cache, freshly verified against unchanged Cargo.lock SHA256 `20e60d9dfc38e093a3926f6867b5519356685bdb6ca38c488857216f5bc9bb7d`. No cache/config/credentials were copied or altered.

Archive source directory:
`C:/Users/spmcc/.cargo/registry/cache/index.crates.io-1949cf8c6b5b557f`.
Index source directory:
`C:/Users/spmcc/.cargo/registry/index/index.crates.io-1949cf8c6b5b557f/.cache`.

| Archive name/version | Bytes | Lock/index/archive SHA256 | Exact index relative path |
| --- | --- | --- | --- |
| cc-1.9.0.crate |147091|877bf67ac8e52449eb2f9e98a344ea163beb70dd1f9eebe3cc5357dcfeaee4df|2/cc|
| find-msvc-tools-0.1.15.crate |31634|1c66915cc5391097de7a2b0b972e8505aa33c0df401fd3a6811dbdb32881ba48|fi/nd/find-msvc-tools|
| zstd-0.14.3.crate |52615|db3ae9bf90af6e40197bef767eeefc46ad6397420fe873830bd85e5fbbfb3c70|zs/td/zstd|
| zstd-safe-8.2.0.crate |46799|ded520f16b29e1a9b771c722221b2b0a0398c2316d9bb3c42b458a5414d7434b|zs/td/zstd-safe|
| zstd-sys-2.1.2+zstd.1.5.7.crate |785774|481ada9c543b2323cbb7bbdbb5249177fbf0188ed959ed2c15e4b22421c84ed3|zs/td/zstd-sys|

Fallback bundle: `<output>/mac-exact-dependencies.tar`;1,095,680B; SHA256 `e3661689635e145c84d49ed460b9f522421397839e9af0138d4c9142c5f9bd11`.
Manifest: `<output>/mac-exact-dependencies/manifest.json`; SHA256 `4c791d6925f8c4a518e67c1066693e455ece1cf8163b44fc934219543dac2a0a`.
Only five crates, five exact index records and this manifest are in the TAR. `dependency-bundle.json` supplies absolute original archive/index paths, original full-index SHA, exact-record byte counts/SHA, versions, yanked flags and archive sizes/checksums. The Windows-local bundle is NOT assumed Mac-accessible.

Mac existing session subsequently confirmed ordinary registry retrieval working and requested no new transfer service/large payload. Exact index JSON records and bundle/manifest facts were messaged to that session and coordinator. Thus no cross-host binary transfer was needed/performed by Windows, and no new transfer route was invented. Mac owns actual download/cache restoration and its execution result; this report does not claim that work completed there.

## Evidence files

Full logs: `logs/{compile,list,test}.stdout.log`, `.stderr.log`, `.samples.jsonl`.
Exact commands/results: `{compile,list,test}-command.json`, `*-result.json`.
Source bindings: all six `*-before-source.json`/`*-after-source.json`, original `io.rs.baseline`, `before-status.txt`, before/after index entry, `telemetry-fixture10.patch`, `gitnexus-impact.txt`.
Runtime bindings: `catalog-reconciliation.json`, `test-binaries-before.json`, `test-binaries-after.json`, `boundary-fixture.json`, compiler helper closure and `log-bindings.json`.
Dependency bindings: `dependency-bundle.json`, `mac-exact-dependencies/manifest.json`, exact five index records and five crates, transport TAR.

Functional build/tests now pass on this corrected Windows source. This does not establish pilot, full matrix, performance envelope, native other-host results or hard resource qualification. No automatic follow-on experiment is started.
