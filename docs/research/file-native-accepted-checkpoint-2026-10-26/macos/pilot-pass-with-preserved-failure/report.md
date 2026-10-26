# Mac small real online A/N64/H3 pilot

**PASS for the requested functional behavior**, with a retained task-adapter failure followed by proof-only continuation and **two measured50ms cooperative-cleanup target misses**. This is not an all-targets performance pass,40ms/hard-peak qualification, or full experiment completion.

Host Mac, macOS26.6.1 arm64, native APFS, October25,2026. Exactly **19engine invocations** total (16native schedule +3remaining outcome resolves), below37. One separate independent checker process; CLI compilation/bookkeeping are separate. There was one new store and no publication/activation/reader-hold/cleanup schedule replay.

## Identity and artifacts

Source `/private/tmp/file-native-online-macos-20261020/source/final-engine`: **baseline0628322e4b48e0efad19cdc32238a622d4aaa7a8 plus ONLY telemetry fixture n1→10**. io.rs SHA256 `836c8a6ceac0f1e972bf77ab3e54be30333f28e763ff1d0ec2f1bff3b34a599d`; the24other admitted files stay exact original freeze464dcf878410def63bd3b04d4f48f17d75418c04262cb4745ebc0e6d5e1de6b3. Source and original SDK/oracle files were rechecked after completion; no source changed during this assignment.

New output `/private/tmp/file-native-macos-inventory-20261024/pilot-A64-H3-20261025`. Store `/private/tmp/file-native-macos-inventory-20261024/pilot-A64-H3-20261025/store/disposable/store`. Native CLI `/private/tmp/file-native-macos-inventory-20261024/pilot-A64-H3-20261025/target/release/file-native-incremental`, SHA256 `d07adb431f4f6c5fd570e6ce8e30965bf443abd2dfd6b1b5bb3d08368b6f242e`. The prior corrected31-test result/test-binary hashes were checked; a fresh source-bound ordinary Cargo locked release CLI build (`-j2`, new target) exited0 in28.466497s. It did not replace or rerun previous tests. Full CLI build command/results are `logs/build-command.json`, `logs/build-result.json` and full stdout/stderr.

Retained original recipe `/private/tmp/file-native-online-macos-20261020/source/pilot-sdk-20261021/sdk/pilot_recipe.py`, SHA afb62d7b4cbba98e973fff2549ffe26a0e1b07ff4a5248ef702035a81f00c386. Task-owned `pilot_behavior.py` SHA35128756dab4d5cdffab04e0a758ff5c8aa165240fe80b4b4f51c5374076b491 reused its behavioral block. Only obsolete admission/expired-clock/qualified40ms wrappers were replaced under current human authorization; first packing got explicit `bootstrap=true`, required by final rolling.rs and added before any engine call. Every behavioral assertion remained. The unrun pre-bootstrap task copy and adaptation record are preserved. No fake native admission/phase/capability/PID or source allowlist changes were used.

Bindings: `pilot-input-bindings.json`, `sdk-bindings.json`, `source-before.json`, `source-after.json`, `recipe-adaptation.json`, `proof-continuation-binding.json`. Original independent v6/v5/paged/quota/outcome/navigation/source/semantic logic and companion hashes were unchanged. Codec used the existing Homebrew `/opt/homebrew/Cellar/zstd/1.5.7_1/lib/libzstd.1.5.7.dylib`, hash69d8dee81bccdee425d167300d11431f3c5ecc34a3011400e536aa89f7756189; no installation or engine-code oracle substitution.

## Actual native schedule and ownership

Fresh modeA,64documents. Initialization and lifecycle setup created actual root-bound disposable authority. Base physical build/cleanup completed, then genuinely accepted `readiness-tail` became H2 and was independently **unmapped**. H2/H3 use the retained recipe's empty-op accepted publications: semantic selected history changes while graph/source bytes remain equal; no payload-change concurrency claim is made.

| Native role | PID/PGID | Original birth seconds/microseconds | Positive RSS before stdin delivery |
|---|---:|---|---:|
| old | 44186 | `[1792959535, 661976]` | 671744 |
| maintenance | 44187 | `[1792959535, 706523]` | 704512 |
| publisher | 44188 | `[1792959535, 864644]` | 638976 |

Each was a separate real native CLI with retained original Popen handle, input JSON, environment barriers, positive native enrollment, stdout/stderr and wait status. Actual ready files contained `pack-catalog-captured`, `physical-before-selection`, `candidate-ready`. No synthetic worker was used.

Maintenance's immutable candidate completed discovery at exactly H2. Before releasing maintenance, publisher H3 was accepted and selected while STORE remained the prior physical generation; old reader and maintenance were still held. Actual monotonic/perf evidence: acceptance `3647984554460041`ns, changed activation joined `3647984634382791`ns. Role distinctness, ready/release order and old/fresh result equality assertions all passed in the native prefix.

- Captured H2 selector: `{"publication":"publication-44174-1792959535503206000-6","digest":"e3f635222aa8e0dbc2e2ce9c80e2f319ef09f8ff523bbe124125e89636755da4"}`.
- Accepted H3 selector: `{"digest":"b640ccdd3cde61354e2711e77652387b8bef5e010c1f45dc4073642252130f3f","publication":"publication-44188-1792959536022309000-6"}`.
- Prior physical id/ordinal: `physical-1-birth-44171-1792959534862337000-0` /1.
- Installed physical id/ordinal at activation: `physical-2-birth-44187-1792959535741410000-0` /2.
- Exact installed candidate plan: `8399de26cb93a1e41287d462563d7d3528c6fe81a1feaec1b39d2e20e02a8cb8`. It matched the prepared target/catalog and preserved H3 selection, with one actual activation hold and nonzero eligible/rolling-visited counters. `evidence/captured-plan-exact.json` was losslessly reserialized from retained JSON and hash-checked against the captured original plan; `evidence/captured-states.json` retains before/after full state. Later cleanup legitimately advanced immutable progress-plan hashes; that is distinct from the activation target.

The old reader remained native-alive across changed activation and a real cleanup attempt. While held, the exact H2 loose publication was read unchanged; a fresh native reader resolved H3 exactly. Old barrier-ready→first release was **16.103821917s**, strictly>15s. Native RSS samples observed this original PID294times across16.127351s; sampled old-reader peak3162112bytes, below256MiB. On release its actual resolve result exactly matched accepted H2.

## Deletion, drainage and full independent oracle

After old-reader wait, two required drainage passes ran on this same store. First postrelease pass needed3engine cleanup calls (cleaning/cleaning/complete); second needed1 and completed. The held-reader cleanup attempted earlier deleted0files and preserved the old fallback. No cleanup was replayed during proof continuation.

Actual removed loose key: `publications/publication-44174-1792959535503206000-6.json`. It was successfully read while pinned, then independently verified absent after drainage, while complete independent mapping resolved its packed bytes to SHA `e3f635222aa8e0dbc2e2ce9c80e2f319ef09f8ff523bbe124125e89636755da4`, exactly the H2 accepted publication digest. Actual engine cleanup reported deletion work; no planned deletion, inode-removal guess or allocation credit was counted as proof. The initial Python failure lost its in-memory full before-inventory, so no invented complete before/after file delta is presented; retained direct path assertions, native raw results, final inventory and independent mapped digest establish this key's removal.

The unchanged independent backlog logic from `suite_oracle.py` fully decoded selected Work/index candidates, probed actual held-generation locks, and classified stray/unprotected physical containers. Result **0unprotected,20protected current-mapping physical paths,0excluded**. Current mapping includes still-needed base-generation instances; their survival is protected data, not a failure to delete. H3's newly accepted loose tail and permanent graph/source/outcome/control data are not obsolete/removable merely because loose. Zero refers to actual unprotected selected-cleanup/physical backlog, not an empty store or zero ordinary files. Complete protected/excluded lists are in the proof result.

All3accepted request inputs/results were reconstructed exactly from authentic native input/output rows (including the recipe's explicit initial request), not guessed from a request ID. The remaining3native resolves matched every accepted result. Separate checker invoked unchanged `physical_decoder_v6.verify`, native quota snapshot, accepted membership/outcomes/intent proof, graph/source/projection and full navigation oracle. It verified H3,64documents,68exact sources, current A readable projection, all3accepted outcomes, no retired-content substitution. Engine audit was not used as the independent oracle. Root inventory was byte-hash equal before/after this proof-only continuation.

Complete oracle: `/private/tmp/file-native-macos-inventory-20261024/pilot-A64-H3-20261025/proof-continuation/checker/complete-oracle.json`, SHA256 `106e7e10431cce372f060cd520f9dc7185b09ccf08b2270681ec34bb2730fbcd`. Supporting quota identity manifest is beside it. `proof-continuation/recovered-ledger.json`, `final-inventory.json`, `logs/independent-complete-oracle.{input,stdout,stderr,result.json}` retain actual proof inputs/output/wait/closure.

## Preserved adapter failure and bounds

The original native schedule reached all joins and both drainage passes using16engine calls, then the task wrapper's extracted backlog helper raised `NameError: strict is not defined`. `pilot-result.json` remains **FAIL**, with full traceback and all original events/results. All16native commands exited0 and closed; this was an adapter namespace omission, not an engine product failure.

A separate task-owned `proof-continuation/continue.py` supplied the existing strict parser binding, recovered the exact accepted ledger from retained native rows, and resumed only remaining backlog/outcome/full-oracle checks on the unchanged H3 store. It executed3new resolve calls +1independent checker, no publication/maintenance/cleanup rerun. Both observations are retained in `final-case-ledger.json`; unrelated matrix rows stay PENDING/INCOMPLETE. This completion does not erase the failed prefix or turn it into a clean one-shot runner result.

**Timing limitation:**2actual batches exceeded the50ms cooperative target:51.227250ms during base cleanup and52.667042ms during postrelease drainage. Raw engine results reported indivisible legacy-index/entry-budget deferral. They are genuine target misses, not rescored as passes or silently relaxed. `evidence/cleanup-passes.json` retains all batches. This small functional pass therefore establishes no complete latency operating envelope.

Native libproc functional supervision retained original PID/birth/PGID/positive RSS before payload, finite300s workload windows,60s held-role and120s ordinary-engine/checker timeouts and finally owned-only stop/wait. Every one of19engine calls and the independent checker has actual exit0, original wait, stdin closure, both pipe EOFs, two valid empty group observations and no retained escaped survivor. Callback threads joined; pool monitor threads stopped. Prefix/continuation native aggregate sampled peaks were44,204,032/70,762,496bytes; no missing RSS or monitor failures. CLI-build sampled peak544,243,712bytes. No aggregate3GiB,old-reader256MiB or checker2GiB breach was observed. No40ms,hard-all-instants peak or full native qualification claim is made.

Fresh APFS free space was>113GB before native work; final snapshot112993292288bytes, above20GiB. New store is small and no large cohort was restored. Existing44GiB/5million-file combined ceiling, inside1GiB/100k-file reserve and protected originals remain controlling; no fictional fresh all-host census or allocation credit was created. Category snapshots are in `final-result.json`; previous tests/reports/binaries/source were preserved. No source/Git mutation/publication, cache upgrade, Sentinel work, pilot rerun or matrix followed.

## Authoritative paths and stop state

`final-result.json` is the combined functional disposition; `pilot-result.json` is the preserved failed task-wrapper prefix; `proof-continuation/result.json` is the proof-only completion. Raw schedule events and every numbered native input/output/child/result are under `logs/`; continuation rows under `proof-continuation/logs/`. `evidence/native-command-index.json` reconciles all20commands, `evidence/cleanup-passes.json` allcleanup passes, and `final-case-ledger.json` retains failed+completed observations with the full matrix still incomplete.

The authorized small pilot's required functional behavior is complete. Full matrix, performance/resource qualification and any publication remain unperformed and were not automatically started.
