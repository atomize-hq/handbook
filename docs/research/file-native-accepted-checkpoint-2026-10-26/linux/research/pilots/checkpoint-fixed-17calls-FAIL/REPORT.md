The checkpoint-fixed release CLI built successfully, and exactly one real A/N64/H3 pilot ran. The pilot FAILED the existing `unprotected-backlog-after-two-passes` assertion after 17 engine invocations. Every engine returned exit 0; the Python worker and outer stage also exited 0 after retaining the semantic FAIL. These exit codes do not imply a passing pilot. No further engine/checker call, repair, retry, cap increase, matrix release, publication or cleanup outside the task store followed.

Build duration was 24.624648 s; pilot stage duration was 22.465145 s. Execution began at 2026-10-26T02:22:11.976089+00:00 and finished at 2026-10-26T02:22:59.101410+00:00. Before work, available native ext4 space was 99,887,357,952 bytes, above the unchanged 20 GiB floor plus 1 GiB practical build/store margin. At final verification it was 99,760,525,312 bytes. The earlier blocked attempt remains blocked history.

The source is baseline `0628322e4b48e0efad19cdc32238a622d4aaa7a8` plus telemetry fixture10 and the previously reviewed checkpoint-resume repair. All 26 reviewed files matched before/after the build and pilot, including the complete set of Rust source files. The original freeze remains unchanged; this runtime is not described as untouched baseline062. Exact bindings:

- Reviewed26 manifest: `43a298bb575b724187eea89de6d55176d67992c2ed3f025e78f8e46d90c981cb`
- New release CLI: `058c306ea04c6b06057aa51a0197eead832363482c1c9559b1de6431a5dedf40` (4,063,928 bytes)
- Runtime src/physical_lifetime.rs: `36603ef499aa29234441f28b2095627ba8c93e3a1657a6f35aaa3a3198c4d8e0`
- Telemetry src/io.rs: `836c8a6ceac0f1e972bf77ab3e54be30333f28e763ff1d0ec2f1bff3b34a599d`
- Repair patch: `4ee1eceb2416a9c3d1124e453079992c46be6882e886693d8a56f93939909571`
- Original freeze: `464dcf878410def63bd3b04d4f48f17d75418c04262cb4745ebc0e6d5e1de6b3`

Build used `cargo build --locked --offline --release -j2 --bin file-native-incremental` with the exact manifest and fresh target directory retained in build-command.json. No broad tests or test reruns occurred. The worker, bootstrap=true recipe, checker, oracle mapping bridge and transferred SDK were copied unchanged from the retained attempt. Only an observation wrapper was added: it delegates the same calls and records read-only STORE/selected-plan/pending-stage snapshots immediately before and after cleanup. It adds no engine calls, changes no assertions or scheduling bounds and does not force checkpoints. Existing worker/recipe/oracle bytes are verified unchanged. Observational overhead sits outside engine-reported batch times; it is still part of this instrumented run.

| Required behavior | Observed outcome |
|---|---|
| Initialize A/N64 and bootstrap physical packing | Initial accepted result lists 64 changed document IDs; bootstrap=true build succeeded. |
| Initial bounded cleanup | Completed on its fifth permitted batch; no watchdog extension. |
| Capture H2, accept H3 before physical activation | Reached and passed exact candidate/parent/discovery and acceptance/activation assertions. |
| Three live native actors | Old reader PID 2296284; maintainer PID 2296288; publisher PID 2296292; distinct birth-bound processes with positive RSS before stdin. |
| Old reader >15 seconds | Barrier held 16.113507960 s. The old exact result and fresh current H3 result assertions passed. |
| Mapping transition | Independent mapping decoder reported the H2 tail unmapped before maintenance and mapped to the exact raw digest after actors joined. |
| Two final drainage invocations | Both executed; independent unprotected backlog was 6 then 8 (protected 20 and excluded 0 in both). Final zero-backlog assertion FAILED. |
| Required obsolete loose H2 tail deletion | Tail remains present after process closure. Deletion criterion FAILED. |
| Final all-accepted outcomes / complete independent H3 oracle | UNRUN because the earlier drainage assertion stopped the recipe; separate complete-checker processes: 0. No independent final N64/H3 certification. |

Cleanup PROGRESS is observed, while final drainage fails. Stable, digest-bound before/after snapshots show selected immutable plan changes in calls 8, 9, 15 and 17. Calls 8, 9 and 17 selected exactly the complete pending immutable plan retained before their invocation, within the same physical generation. This is direct pending-checkpoint continuation evidence. The initial selected job set dropped from five jobs to zero in call 8. The later generation still has five jobs at position zero in all post-activation snapshots; call17 instead retires the older-generation entry (one to zero). Selected removed_files/removed_bytes counters remain zero in these snapshots, so they are not credited as positive loose-deletion counters.

Calls 5–7 left the selected root unchanged despite pending work. In particular, call7 left a complete immutable candidate which call8 subsequently selected. Call16 similarly left the selected root unchanged and retained the candidate selected by call17. Immutable installation and rename/replacement counters alone are not labeled selected progress. Intermediate transitions inside a call are UNKNOWN; before/after snapshots cannot prove all cursor histories. The final backlog increase includes newly unprotected metadata, so it is not by itself proof that all cleanup regressed. Full snapshots and complete plan identities are retained in observations/ and cleanup-progress-analysis.json.

Cleanup TIMING failed: seven of eight batches exceeded the unchanged 50 ms target. Every overrun is retained; there is no discarded slow run or lucky retry. Reported batch durations and nested metrics follow. Inclusive categories overlap and must not be added; they do not isolate a complete causal cost model.

| Engine call | Status | Candidates | Batch ms | Sync ms | Recovery inclusive ms | Metadata inclusive ms |
|---|---|---:|---:|---:|---:|---:|
| engine-05-packing | cleaning | 94 | 52.632670 | 50.665964 | 0.309677 | 17.607492 |
| engine-06-packing | cleaning | 78 | 51.747186 | 50.220516 | 17.141318 | 25.497558 |
| engine-07-packing | cleaning | 72 | 51.538870 | 50.366333 | 17.268199 | 34.058374 |
| engine-08-packing | cleaning | 85 | 60.965041 | 59.088021 | 25.916390 | 35.112681 |
| engine-09-packing | complete | 75 | 31.741210 | 29.770579 | 31.019170 | 0.892546 |
| engine-15-packing | cleaning | 94 | 52.690431 | 51.261281 | 0.360860 | 52.308198 |
| engine-16-packing | cleaning | 70 | 52.793996 | 51.588164 | 0.382133 | 52.345945 |
| engine-17-packing | cleaning | 85 | 55.035779 | 52.678054 | 27.127095 | 18.739957 |

All seven overrun responses report deferred:cleanup-entry-budget. The successful initial final batch reports batch_ns 31.741210 ms and entry-inclusive 31.770607 ms; both original measurements remain intact. Seven miss events, full per-call metrics (including elapsed, sync, recovery, metadata and selection hold), raw requests/stdout/stderr, EOF/wait records and snapshots are retained. Calls5–9 are initial cleanup, call15 runs with the old reader held, and calls16–17 are exactly the final two passes.

Native and callback closure passed: all 17 original PID/birth identities were absent in two final observations, output readers joined, both EOFs and original waits were retained, owned groups/outer trees were empty twice, monitor joined, and no resource-stop or secondary callback error occurred. A read-only /proc/locks sample found no visible lock holder on the retained store; no lock was acquired to manufacture a closure claim. Build and pilot tree-plus-supervisor sampled RSS maxima were 561,836,032 and 58,986,496 bytes. Approximately 200 ms sampling does not establish hard peaks, 40 ms qualification or full experiment readiness.

Limits stayed at 3 GiB aggregate new-work RSS, 2 GiB per ordinary process/checker, 256 MiB old-reader role, 20 GiB native free floor, 2 GiB/100,000-file task growth cap, 600 s build timeout, 480 s outer pilot timeout, original 300 s functional window and <=37 engine calls. The inherited combined experiment ceiling remains 44 GiB / 5 million files; no new all-host/multi-host census or qualification is claimed. At execution close task allocation was 113,430,528 bytes across 723 files. Later reports add small retained evidence. Store inventory at final verification is 196 files / 1,707,648 logical bytes / 1,916,928 allocated bytes.

All prior attempts remain intact. Full retained evidence manifests were rechecked: blocked attempt 48 files; checkpoint repair 857; first failed pilot 657; bootstrap/watchdog pilot 277. All hashes and sizes match. Disk-assessment reports/manifests and original freeze/protected binaries also match their recorded preflight hashes. No unrelated Handbook, Sentinel, Substrate, UAA or cache content was removed. The only deletion activity was the engine’s authorized cleanup inside the new disposable task store.

The retained obsolete tail is `publications/publication-2296278-1792981362041846089-6.json`. Its present bytes, all remaining work/stages and backlog paths are preserved for a separately authorized diagnosis. This report does not diagnose or fix the new failure. Next state is stopped after the one authorized pilot.

Primary evidence: [result.json](result.json), [run-result.json](run-result.json), [pilot-result.json](pilot-result.json), [pilot-exception.txt](pilot-exception.txt), [cleanup-progress-analysis.json](cleanup-progress-analysis.json), [cleanup-target-misses.json](cleanup-target-misses.json), [engine-call-index.json](engine-call-index.json), [final-verification.json](final-verification.json), [prior-evidence-preservation.json](prior-evidence-preservation.json), [retained-store-inventory.json](retained-store-inventory.json), logs/, observations/ and events.jsonl.
