# Windows corrected real online pilot — PASS

October 25, 2026; SPENSERSDESKTOP, native Windows/NTFS. **One corrected A/N64/H3 pilot passed with 16 engine invocations and 4 independent checker processes.** All 20 processes returned exit 0, original waits signaled, stdout/stderr EOFs were observed, Jobs drained empty without failure termination, and all callbacks/handles closed. No additional pilot, matrix, publication or engine repair was started.

The old reader held its real barrier for **16.1077172 seconds** through H3 publication, changed physical activation and a cleanup attempt. Exact old/current outcomes passed, the old loose tail was actually deleted after release, the two drainage calls completed, independent unprotected backlog was zero, and the full independent H3 physical/semantic/outcome/graph/source/navigation oracle passed. No 50ms cleanup target miss occurred in this small run. This is a functional pilot result, not full performance, 40ms monitoring or hard-peak qualification.

## Source, executable and narrow correction

Source/cwd: `C:/Users/spmcc/.codex/worktrees/file-native-online-cp2-20261020/handbook/docs/research/file-native-incremental-engine-experiment-2026-10-18`.

Source identity remains **baseline0628322e4b48e0efad19cdc32238a622d4aaa7a8 plus ONLY the prior telemetry fixture n1→10 patch**, not unchanged062. Corrected io.rs SHA256 `836c8a6ceac0f1e972bf77ab3e54be30333f28e763ff1d0ec2f1bff3b34a599d`; original code freeze `464dcf878410def63bd3b04d4f48f17d75418c04262cb4745ebc0e6d5e1de6b3`. All 30 corrected expected files matched before/after; 29 baseline files and all prior passed test binaries remain unchanged. HEAD/index are unchanged; the only existing worktree modification is that previously authorized fixture patch. No product or test source was changed in this assignment.

The already built normal CLI was freshly checked against its prior exact binding, then copied to this task's bin/engine.exe: **3,460,096 bytes**, SHA256 **`e6116d2ddb7cf92f15368c47810d6f35ca11fc5dfe479069266b3643d3419535`**. No rebuild/dependency change was needed for this corrected attempt.

All files below are under this output root unless otherwise stated:
`C:/Users/spmcc/.codex/visualizations/2026/10/24/01a16fd3-b2c6-7af1-b036-fd72bdba257d/real-online-pilot-corrected-20261025`.

Changes were limited to:

1. `native_runtime.py`: retain checked original wait/exit and contemporaneous owned-Job members/time/pipe states; query each returned member through a native handle for creation/image/Job membership/wait state, preserving errors; retire our completed root handle after the first observation; allow Job-empty/EOF drainage only within the existing invocation/whole-pilot deadline. Preserve all memory/free-space/timeout/failure-termination checks. No PID/helper whitelist and no UNKNOWN→absent conversion.
2. `pilot_behavior.py`: change only first `h.pack(root)` to `h.pack(root, bootstrap=True)`. Final rolling.rs:76–77 explicitly requires this on first discovery. Later maintenance and every behavioral assertion remain unchanged.
3. `pilot_runner.py`: fresh output path by its own location, bind the existing exact CLI hash, and include the previous failed pilot's actual retained footprint in the room check.

Exact before/after hashes and diffs are in `correction-bindings.json`, `native_runtime.py.diff`, `pilot_behavior.py.diff`, `pilot_runner.py.diff`. Prior failed pilot source/store/logs remain intact in sibling `real-online-pilot-20261025`.

Task-owned source hashes:

| File | SHA256 |
| --- | --- |
| native_runtime.py |49f308f9736b625b8c5e45197af4fad3cfb03a66878f5f0bef8237623e65c159|
| pilot_behavior.py |2e1d8d15d0051e48c51570167d8d20d0dd70af8f4fe6572ea776254a59c0005e|
| pilot_runner.py |46ee1e88dc7ff6ba5ed0998d51a6aef83371036a37e195b8c63236782312aa12|
| oracle_worker.py, unchanged task adapter |216c343bc9bac66bc5f9a210655b7d497f223356412c3c3adbea469e34501ede|
| focused_protocol.py, unchanged retained SDK |7ce0a4a06a35fa9e7f4f4080a40315a418bb45e47a8946ca06d78c2117d855e8|

## Actual native roles and ordering

Fresh store: `<output>/disposable/store`. Initialization used the default A/project payload with **64 documents**. Lifecycle setup, explicit discovery bootstrap, base packing/cleanup, and accepted loose `readiness-tail` established H2. Independent mapping process #7 confirmed the H2 publication was unmapped before the concurrent schedule.

| Real native role | PID | Creation FILETIME | Positive RSS before stdin |
| --- | --- | --- | --- |
| Old reader |23712|134374404408116379|1,978,368 B|
| Maintainer |27848|134374404408367037|2,486,272 B|
| Publisher |14480|134374404409112588|3,366,912 B|

Each used the exact bound engine image and distinct actual process/handle ownership. Ready files contained the actual source checkpoint names `pack-catalog-captured`, `physical-before-selection`, and `candidate-ready`. Inputs, native starts, barriers, original waits, result streams and EOFs are retained per invocation.

The installed candidate was independently read from PACKING/immutable Work and checked by the retained recipe before publisher release:

- Captured H2 publication `publication-17496-1792966840321834900-6`, selector digest `6096343b3ecbe0c8af154ab8e0523232d75add79f6bbe1daa2cf09ca55646060`, semantic sequence 1.
- Candidate immutable plan SHA256 `e8c49145a7b8777e09bd7aebe1a2e8925f009fb53c88b23035664f31352dd429`; generation `physical-2-birth-27848-1792966840851339000-0`, ordinal 2.
- H3 accepted and selected at perf ns **164125131542600** while maintainer and old reader remained held. H3 publication `publication-14480-1792966840947115300-6`, digest `53a38d12a8ab1bbbe0602b852d101a0bb3cc20cd3a62f9786110f3ab94497451`, request `readiness-after-capture`, exact outcome accepted.
- Maintainer release at **164125135406300**, strictly after accepted H3; activation joined at **164125200602700**.
- STORE, catalog and Work matched the prepared candidate exactly at activation; H3 semantic selector remained unchanged. Physical ordinal advanced 1→2 and catalog descriptors changed 5→10 with new mapping entries, so this was a genuine physical activation.

Fresh current resolve returned exact H3 result. The cleanup attempt while the old reader remained held preserved its H2 loose publication bytes; it returned cleaning with protected work, not a false drain completion. Old reader ready time **164124873467100**, release time **164140981184300**: **16.1077172 seconds**. Its original call lasted 16.187 seconds and returned the exact accepted H2 result after release. Callback cleanup joined all three native roles.

## Actual drainage, deletion and independent oracle

After old-reader release, independent mapping process #13 confirmed the H2 tail was authentically mapped with the expected raw hash. Exactly two subsequent cleanup invocations ran (#14 and #15); no extra retry/drain loop was needed.

| Cleanup phase | Candidates | batch_ns | Deleted files / logical bytes | Result |
| --- | --- | --- | --- | --- |
| Base representation cleanup (#5) |154|21,144,200|5 / 17,243|complete|
| While old reader held (#12) |100|10,864,700|0 / 0|cleaning; protected work retained|
| Drain pass 1 (#14) |213|25,985,100|8 / 12,020|complete|
| Drain pass 2 (#15) |78|2,062,500|0 / 0|complete|

All four reported batches were below the 50,000,000ns cooperative target. This is four observations, not a percentile or general latency envelope. No native allocation/deletion credit was inferred.

Actual deleted loose path:
`publications/publication-17496-1792966840321834900-6.json`.
It existed unchanged during held-reader cleanup and is absent in the final store. Complete before/after inventories are in `events.jsonl`. Independent backlog process #16 reported **0 unprotected, 20 protected, 0 excluded** physical paths. The 20 protected paths are current pack/index containers; they are not removable backlog. The accepted current H3 loose publication remains as legitimate uncompacted permanent history beyond the captured H2 cutoff, not as an omitted removable tail.

Three fresh outcome calls (#17–19) resolved initial/H2/H3 exactly against the captured request/result ledger. Independent full checker #20 used the original retained v6 decoder plus physical/page/quota/semantic/navigation/request-intent implementations; no engine audit or Rust decoder was used. It proved:

- H3, all three accepted publication memberships/outcomes and exact request intentions; three retained candidate proofs.
- Complete mapped raw-record/cohort and page/tree validation, final ownership, graph and source checks.
- Three retained content-root visits, 68 sources, 64 readable/navigation documents and 68 projection files.
- Read-only before/after native inventory equality. A-only scope; no broader B or retired-history workload is inferred from inapplicable checker branches.

Full independent receipt: `oracle-4-full.json`, SHA256 **`9847ddb7790a7e6de57c2c54a6a4bbd8961afda392eb8c6adea408f7f7e8f96b`**. Wrapper/loaded source and independent Zstd DLL bindings are in `oracle-4-full-response.json`; exact original v6 companion checks were enforced inside the checker. Mapping/backlog receipts are `oracle-{1,2}-mapping*.json` and `oracle-3-backlog*.json`. `evidence-summary.json` joins ordering, roles, cleanup, current file existence, outcome/oracle and closure facts.

## Transport correction observations and closure

On this corrected attempt's init, the first actual post-root observation found an owned **conhost.exe** PID 3372, creation FILETIME 134374404383036166, with native wait code 258 (live), while both output EOFs were already present. The observation was retained; no assumption of terminal state or helper whitelist was made. After normal completed-root handle retirement and bounded drainage, the next observation about 50.75ms later had an empty Job and both EOFs, within the original timeout. No termination was issued. This positively identifies this attempt's helper only; it does **not** reconstruct old PID31268's missing image/birth or relabel the original failed pilot.

Every one of the 20 calls/checkers retained checked original wait code 0, exit 0, actual stdout/stderr EOFs, closed reader threads/handles, and an empty per-call Job before normal return. Each `logs/*.post-root.jsonl` records observations/errors and pipe state. **Failure termination count: 0.** Final aggregate Job empty, no active callbacks and no resource violation. No post-kill emptiness was used as a spontaneous-closure claim in this attempt.

Original resource policy remained: Windows aggregate new-work Job commit cap 4GiB and approximate 250ms RSS observations including supervisor; old reader 256MiB, checker/other process 2GiB; existing per-call and whole-pilot finite deadlines; C: free-space floor 140GiB. Sampled aggregate peak was **65,302,528 B**. C: free after evidence review was 179,373,441,024 B. These are useful sampled observations, not 40ms or all-instant hard RSS qualification.

The practical headroom check included both retained build/test runs and the prior failed pilot, preserving the combined 44GiB/5million-file ceiling, inside reserve and protected data. New output/store was about 5.87MB/343 files before this report. No all-host census, protected-data cleanup, corpus restoration or forecast deletion credit was claimed.

## Retained files

- `report.md`, `pilot-result.json`, `evidence-summary.json`, `engine-binding.json`.
- `logs/01…20`: exact input/argv, native identities, original waits/exits, stdout/stderr full files, EOFs, post-root member observations and result JSON.
- `events.jsonl`, `resource-samples.jsonl`, `native-closure.json`, `room-check.json`, `accepted-inputs.json`.
- `pilot-before-source.json`, `pilot-after-source.json`, source patch/index provenance copies, correction diffs/bindings and task adapter source.
- Four independent receipt/response families, exact loaded checker source/DLL hashes, full v6 proof and native quota identity inventory.
- `artifact-bindings.json`: exact task-owned source, binary, receipt and raw evidence hashes.
- `disposable/store`: final native A/N64/H3 store, retained intact.

The original failed pilot and corrected 31-test PASS remain preserved. This corrected pilot establishes the specified small functional behavior on Windows. Full matrix, larger histories, B schedules, broad native recovery/compatibility and performance/hard resource qualification remain separate work; none was automatically started.
