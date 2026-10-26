# Windows real online pilot — FAIL / INCOMPLETE

October 25, 2026; SPENSERSDESKTOP, native Windows/NTFS. **The single attempted pilot stopped after its first engine invocation on a task-owned native transport assertion. The engine initialization itself exited 0 and accepted the initial 64-document publication. No H2/H3 online behavior was reached.** This is not evidence of an engine semantic failure or a completed pilot.

No rerun, continuation, engine repair, full matrix, protected cleanup or publication was performed. The actual failed prefix and new store remain intact.

## Source and executable correspondence

Source is baseline `0628322e4b48e0efad19cdc32238a622d4aaa7a8` **plus only the already authorized telemetry fixture n=1→10 patch**, not unchanged baseline062. Corrected io.rs SHA256 `836c8a6ceac0f1e972bf77ab3e54be30333f28e763ff1d0ec2f1bff3b34a599d`; original freeze `464dcf878410def63bd3b04d4f48f17d75418c04262cb4745ebc0e6d5e1de6b3` preserved.

All 30 corrected expected source/control files matched before and after the pilot; the other 29 baseline files are unchanged. HEAD/index remain unchanged; sole worktree modification is the existing io.rs fixture patch. No product/test source was edited during this pilot assignment. All four earlier passed test-executable hashes remain unchanged.

The prior `cargo test --no-run` had produced test harnesses, so a normal CLI was built from the same verified source, locked/offline/release/j2, using the passed target's existing dependencies. It completed exit 0 in 0.781 seconds. This added the CLI without changing earlier test binaries. Compile command, source hashes, full logs and Job closure are retained.

Executed task-owned CLI:
`C:/Users/spmcc/.codex/visualizations/2026/10/24/01a16fd3-b2c6-7af1-b036-fd72bdba257d/real-online-pilot-20261025/bin/engine.exe`

Size 3,460,096 bytes; SHA256 **`e6116d2ddb7cf92f15368c47810d6f35ca11fc5dfe479069266b3643d3419535`**. Original new CLI is under the passed `final062-fixture10-20261025/target/release` target. `engine-binding.json` binds exact paths and source variant.

## Route and preserved implementation

Original recipe:
`C:/Users/spmcc/.codex/worktrees/file-native-online-maintenance-20261020/handbook/docs/research/file-native-online-maintenance-2026-10-20/execution/readiness-20261021/pilot/pilot_recipe.py`.

Task-owned `pilot_behavior.py` retains its complete behavioral body, including all actions, comparisons, three real-role callbacks, H2 capture/H3 acceptance-before-activation checks, old-reader >15 second requirement, cleanup, exact outcomes and independent oracle assertion. Only the expired admission/qualification preamble and historical return receipt were removed. No false 40ms qualification or old admission fields were constructed. The shared SDK input ledger, call/state/cleanup/outcome/oracle assertions were reused. Task-specific case ownership and current finite deadline wiring are in `pilot_runner.py`.

The task inspected and bound the original complete independent `physical_decoder_v6.py` and all six explicitly hash-checked companions. `oracle_worker.py` uses that original v6 verifier, with the retained complete SDK backlog method ported only from Unix flock to Windows LockFileEx. Original decoder/semantic/navigation/oracle sources were not edited. **No independent checker process ran on the store**, so no independent physical, semantic, navigation or backlog result is claimed.

`route-binding.json` records original paths/hashes and the exactly preserved behavior-body derivation. `artifact-bindings.json` records task-owned adapter/recipe/transport/oracle source hashes, executable and raw evidence hashes. These source preparations are not runtime qualification.

A pre-execution import-only check failed because the extracted backlog method used a different indentation width from the exact replacement template. Its original adapter is retained as `oracle_worker.pre-import-indent.py`, with the failure explained in `pre-execution-correction.txt`. The template match was corrected before any store or engine call. No oracle assertion or product behavior was weakened.

## Actual first invocation and concrete stop

Engine calls: **1 of at most 37**. Independent checker processes: **0**.

The actual input was init, mode A, consumer project, count 64, on this fresh store:
`C:/Users/spmcc/.codex/visualizations/2026/10/24/01a16fd3-b2c6-7af1-b036-fd72bdba257d/real-online-pilot-20261025/disposable/store`.

Native root PID **21356**, creation FILETIME **134374332888446364**, correct task-owned engine image. Positive native RSS before stdin: **1,974,272 bytes**, observed at perf-counter ns **156972893480000**. Stdin closed at **156972893792500**. Original process wait returned at **156973785333800**, actual engine exit **0**. Transport elapsed 0.985 seconds.

Raw stdout is a successful engine JSON envelope with `error=null`, `status=ok`, and an accepted `initial` result changing all 64 IDs d000000…d000063. The current publication file independently inspected as plain data has sequence **0**, request **initial**, predecessor **null**: actual accepted history **H1**, not H3.

Initial selector:
`publication-21356-1792959689339817800-80`, digest `5fddfa9076b84a7c8167f4e205f75801e38bf93c3c5348eafed4c258dee39398`.

After root exit, `native_runtime.py` required its per-invocation Job PID list to be immediately empty. That check raised **`unexpected-owned-descendant-after-root-exit`**. The global monitor had observed engine PID 21356 plus PID 31268 while initialization was active. The assertion did not retain its exact contemporaneous Job-list return or the second process's image/birth. Therefore this evidence does **not** establish whether a live helper, delayed Job accounting, retained root, or another lifecycle condition caused that immediate assertion. It must not be described as a proved leaked engine descendant or as the same historical Windows failure.

The concrete blocker is this task adapter's immediate Job-list classification after a successfully signaled engine exit. A future correction would need to retain actual returned Job identities and distinguish original root/helper/accounting states with native evidence and bounded closure. This report does not require resurrecting old phase/qualification machinery. No adapter repair/repeated pilot was performed after the engine call.

## Required behavior: observed versus unrun

| Obligation | Actual result |
| --- | --- |
| Native A store, 64 documents | Created; init successful, accepted H1 |
| Lifecycle/base physical representation and unmapped H2 | UNRUN; STORE remains version 1 without a physical catalog |
| Distinct old reader, maintainer, publisher and real barriers | UNRUN; no such role was launched |
| Captured H2, accepted H3 before changed activation | UNRUN |
| Old-reader hold >15s/current and old results | UNRUN; no hold duration exists |
| Actual post-pin loose deletion and two drainage passes | UNRUN |
| Independent zero unprotected backlog | UNRUN; no zero-backlog claim |
| Complete independent H3/outcomes/graph/source/navigation oracle | UNRUN |

No admission, capability, H3 result, barrier timestamp, deletion, reader hold or oracle observation was fabricated to complete these cells.

## Output, resources and actual closure

The task used original native waits and real stdout/stderr pipes. Both EOFs were recorded:

- stdout: 4,725 bytes, EOF perf ns 156973784525900, SHA256 `51020a7f122d6f0be99c8b06ae32d6017ff0e156e67da16b637c54c85ddc0cc2`.
- stderr: 5,212 bytes, EOF perf ns 156973784557600, SHA256 `f048c41484b0b74169515bcc24513343c3633fe9a463377a9fff2bad24deaa6b`.

The failed path terminated the owned per-call Job and joined the pipe readers. The final global closure record reports Job empty, no active callbacks, all transport handles closed, both EOFs, and no resource violation. This is later positive closure; it does not reconstruct the missing immediate Job-list classification.

Practical room check: C: free 179,398,746,112 bytes versus 140GiB floor; retained Windows build/test outputs 503,335,915 bytes; planned small pilot allowance 32MiB fits the retained 676,345,051-byte Windows future-work room without increasing the combined 44GiB/5million-file ceiling or touching its inside reserve. No fresh all-host physical census is claimed. Actual task output including store and source preparation was about 5.33MB/191 files before this final report/bindings. Old data and failed runs were not deleted.

Supervision used a 4GiB aggregate Job commit cap, sampled aggregate new-work RSS including supervisor, 2GiB process limits for init/checker roles, planned 256MiB old-reader limit, per-call timeouts and whole-pilot finite timeout. The old-reader cap was never exercised. Sampled aggregate peak was **43,016,192 bytes**, approximate 250ms observations; no 40ms or hard all-instant RSS claim. The normal CLI build's owned VCTIP helper was closed after Cargo exited, separately recorded in its compile result.

## Exact retained evidence

All paths below are under the absolute output root:
`C:/Users/spmcc/.codex/visualizations/2026/10/24/01a16fd3-b2c6-7af1-b036-fd72bdba257d/real-online-pilot-20261025`.

- `pilot-result.json`: complete terminal failure and traceback, source variant, call counts, source/binary preservation and closure.
- `logs/01-engine-init.{input,command,result}.json`: exact input, argv, native identity/start/wait/exit/EOF/hash records and adapter assertion.
- `logs/01-engine-init.stdout.log`, `logs/01-engine-init.stderr.log`: complete original engine streams.
- `events.jsonl`, `resource-samples.jsonl`, `native-closure.json`, `room-check.json`.
- `engine-binding.json`, `pilot-before-source.json`, `pilot-after-source.json`, `compile-{command,result}.json`, compile full stdout/stderr/sample logs.
- `route-binding.json`, `artifact-bindings.json`, `pilot_behavior.py`, original shared `focused_protocol.py` copy, `pilot_runner.py`, `native_runtime.py`, `oracle_worker.py`, preserved pre-execution failed adapter and correction note.
- `disposable/store`: retained native N64/H1 prefix with all generated files; no cleanup performed.

The earlier corrected 31-test PASS remains valid and preserved. This separate online pilot is **incomplete due to the concrete native transport assertion**, with no online engine behavior or complete oracle result to assess. No automatic follow-on work is started.
