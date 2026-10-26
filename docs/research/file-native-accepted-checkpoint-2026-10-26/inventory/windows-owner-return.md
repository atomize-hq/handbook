Windows inventory complete. [Full report and evidence references](C:/Users/spmcc/.codex/visualizations/2026/10/24/01a16fd3-b2c6-7af1-b036-fd72bdba257d/windows-inventory/report.md).

The final online-maintenance engine exists intact, but its integrated runtime result is still missing. The CP2-named checkout is on CP3 at `0628322e4b48e0efad19cdc32238a622d4aaa7a8`. All **30 frozen source/control files** match the retained final-source receipt; current source contains **31 tests**. Existing Windows CP4 binaries instead match an earlier **20-test compile-only snapshot**, with zero tests or engine calls executed.

No builds, tests, experiment imports, existing-source changes, Git mutations, installations, transfers, archive extraction, corpus copying, deletion, or dataset rehashing occurred. Only task-owned inventory reports were written.

**Host and search coverage**

Observed host: `SPENSERSDESKTOP`; OS API description `Microsoft Windows 10.0.26200`; native Windows/MSVC. Both C: and D: report NTFS. Inspection date: October 24, 2026, America/Indianapolis.

I inspected registered worktrees, numbered and named handbook checkout candidates under `C:/Users/spmcc/.codex/worktrees`, the project and orchestration roots, all immediate `C:/file-native-*` roots, relevant Temp locations, shallow D: build/temp roots, retained October task-output directories, and locations identified by experiment manifests.

Found **11 experiment worktrees**, the primary checkout, and **23 standalone C:/file-native-* roots**. No separate CP4 checkout emerged; CP4 source lives in the CP2-named checkout and archived Git objects.

Coverage gaps: unrelated repositories, every hidden user directory, redirected Desktop locations, WSL filesystems, and every archive member were not exhaustively searched. `Get-Volume` was denied; DriveInfo independently supplied NTFS. Large corpora were deliberately not enumerated or rehashed. Windows-held Linux/Mac returns remain historical evidence; no cross-host connection was initiated.

**Repositories and candidate source**

All listed experiment HEADs are reachable from locally retained remote refs. No unpublished committed tail was found using `HEAD --not --remotes`. Matching origin branch tips were present locally; no fetch established current server state.

All these worktrees share `C:/Users/spmcc/Documents/__Project_Code/handbook/.git`.

| Absolute checkout | Branch | HEAD | Live status |
|---|---|---|---|
| C:/Users/spmcc/Documents/__Project_Code/handbook | feat/handbook-contract-membrane-clean | 379af4eda9b95ee82be9d3afcfdcdca2d6d132f9 | 39 untracked groups; no tracked changes reported |
| C:/Users/spmcc/.codex/worktrees/b3fc/handbook | codex/file-native-n-linux-20261019 | 3f4eb8f336a8b3658e2b15ae8fa178f34bc4af88 | Clean |
| C:/Users/spmcc/.codex/worktrees/file-native-incremental-20261018/handbook | codex/file-native-incremental-20261018 | 3396092cbf20f593e36b9640c6a76e5afd2f2a09 | Clean |
| C:/Users/spmcc/.codex/worktrees/file-native-navigation-20261019/handbook | codex/file-native-navigation-20261019 | 83f7330f00dd35ad7b6d8c3c850f5f8bcc1be75e | Clean |
| C:/Users/spmcc/.codex/worktrees/file-native-online-cp1-20261020/handbook | codex/file-native-online-cp1-20261020 | bd6dd9367acddaddcd7873c8043548502a170c3b | Clean |
| C:/Users/spmcc/.codex/worktrees/file-native-online-cp2-20261020/handbook | codex/file-native-online-cp3-20261020 | 0628322e4b48e0efad19cdc32238a622d4aaa7a8 | Clean; final engine source |
| C:/Users/spmcc/.codex/worktrees/file-native-online-maintenance-20261020/handbook | codex/file-native-online-maintenance-20261020 | 6b689db5afb589a6967c504200afca24add6fb73 | 11 untracked families |
| C:/Users/spmcc/.codex/worktrees/file-native-online-matrix-20261020/handbook | codex/file-native-online-matrix-20261020 | 5bb4d800e05203e681d237cbc501e97b4bd50ba7 | 3 tracked modifications; 22 untracked groups |
| C:/Users/spmcc/.codex/worktrees/file-native-packing-20261020/handbook | codex/file-native-packing-20261020 | eed8da74c22c3352b9fd919d181b82190f4652bb | Clean |
| C:/Users/spmcc/.codex/worktrees/file-native-q-linux-20261019/handbook | codex/file-native-q-linux-20261019 | e75a95c4bd27c62a9d872220ceb80f827d5a6a2e | Clean |
| C:/Users/spmcc/.codex/worktrees/file-native-qnp-integration-20261019/handbook | codex/file-native-qnp-20261019 | a0b92fe5dc64269508601dcddf7e2e2ca477e30d | Clean |
| C:/Users/spmcc/.codex/worktrees/file-native-recovery-20261019/handbook | codex/file-native-recovery-20261019 | f0904a63cb2500b9bea3a4034d19cbe38bfac729 | Clean |

The CP2 branch remains available at `7a6ee9db03f5cd2e388129a9019c0d5f274a1844`; its directory name does not identify its active branch.

Important source progression:

- CP1: `62ce9630`, followed by ownership/cleanup corrections `f6f90794`, `cd12ea85`, `bd6dd936`.
- Overlap routing: `dc868ef5`.
- Rolling history: `b876df9e`, corrected by `7a6ee9db`.
- Paged indexes: `f9b762a1`, remediated by `a3f9805e`.
- Final CP4: `0628322e`.

The archive `6b689db5` is **not a descendant of `0628322e`**, but their complete incremental-engine subtree has the identical Git tree ID:

`291bcd9bd6d9f2aab18466e37a5d8e3bbed95237`

Its direct subtree diff is empty. The maintenance checkout deliberately omits that crate through skip-worktree—1,417 tracked paths—so inspecting only that directory falsely suggests the engine is missing.

The actual final manifest is:

[C:/Users/spmcc/.codex/worktrees/file-native-online-cp2-20261020/handbook/docs/research/file-native-incremental-engine-experiment-2026-10-18/Cargo.toml](C:/Users/spmcc/.codex/worktrees/file-native-online-cp2-20261020/handbook/docs/research/file-native-incremental-engine-experiment-2026-10-18/Cargo.toml)

Standalone copied source also exists at `C:/file-native-recovery-20261019/baseline-source`, containing Cargo files, `src`, and consumers. Its source identity must come from recovery manifests; no independent Git identity was established there.

Unfinished material that must be preserved:

- Maintenance checkout: native Linux readiness, shared readiness/pilot/meta/resource/review files, three Windows readiness guard revisions, and six consultation families through October 24. These are untracked and absent from the committed closeout.
- Matrix checkout: modified `cp2_full_batch.py`, `final_document_controls.py`, and `matrix_budget.py`; untracked final protocol, decoder v5/v6, paged index, independent checker/backlog, event logging, quota/operational controls, cadence work, corrected-oracle receipts, and method proposals.

**Artifact-family inventory**

| Family | Existing locations and significance |
|---|---|
| Original publication experiment | `C:/file-native-experiment-20261017`, `-v2`, `-v3`; correctness, races, faults, growth, timeout/stopped negatives and special cases. Native executable remains in `C:/file-native-experiment-20261017-build`. |
| Original followup | `C:/file-native-followup-20261017`; COW/manifest correctness and performance profiles, Windows final variants and PowerShell extraction stores. Seven retained build variants exist under `C:/file-native-followup-20261017-build*`. |
| Incremental experiment | `C:/file-native-incremental-20261018`, plus `-build`, `-build-control`, `-build-final`; protocol/differential/namespace/recovery/accounting results, failed prefixes, source capsules and separately identified native Linux/WSL returns. |
| Protected original history | `C:/file-native-incremental-20261018/history-common-pin/store` and `history-pilot-1k/store` currently exist. Historical original-store census: 5,464,941,402 logical bytes and 1,413,491 files; not recounted here. |
| Navigation | `C:/file-native-navigation-20261019`; candidate binaries, focused/matrix variants, ordinary/N128 proofs, preserved focused-first archive and N10k corpus archive/manifest. |
| Recovery | `C:/file-native-recovery-20261019`; baseline source/build, final/compatible/legacy/replay/paired stores and receipts, restored history and operational backups. Multiple “final” receipts bind different earlier source versions. |
| Original operational backup | `C:/file-native-recovery-20261019/history-operational-backup`; `files.jsonl` is 248,280,348 bytes, transport is 2,466,253,310 bytes. Recorded transport SHA256: `9e43778043425cc46b1f92df7ffa9ab7d47501f035fdc688ff4e4db677ebec79`; not rehashed here. |
| Packing | `C:/file-native-packing-20261020`; candidate/test executables, build target, donors, long/operational matrices, backup/restore, preservation, reclamation and followup. |
| Baseline d70 executable | `C:/file-native-packing-20261020/bin/candidate2.exe` freshly hashes to `568f631acf528c7de911bb88ec85d07ead9d7b4435ebe970c61053f06e31a93b`, matching the documented d70 core executable. |
| Protected H101007 archive | `C:/file-native-packing-20261020/packed-operational-backup`; transport 313,275,597 bytes. Historical verified scope: 5,807 entries, 592,458,971 uncompressed bytes, including the 246,560,316-byte expected ledger. Recorded transport SHA: `2fabd1c2439fc95a37a0411c99025bfb5646dee855b6ed0e879fd190e35e324b`. |
| Protected successor | `C:/file-native-packing-20261020/packed-successor-operational-backup`; transport 313,303,876 bytes. Original, restored, successor and post-GC backups remain separate families. |
| Other preservation archives | `artifact-preservation/artifacts.bin.gz` is 66,100,201 bytes and has an incomplete-prefix record. `artifact-preservation-xz/artifacts.bin.xz` is 23,759,008 bytes with inventory/manifest. Do not equate the failed gzip attempt with completed XZ preservation. |
| QNP/native returns | `C:/file-native-qnp-20261019`; Linux reviewed returns and Mac build/unit/preservation/reclamation/full-native/closure/followup archives and extracted evidence. These are Windows-held historical returns. |
| Linux transport/support | `C:/file-native-p-linux-20261020`; staged source, qualification, preservation/reclamation and returned-review families. |
| Online checkpoint builds | `C:/file-native-online-cp1-20261020/{build,evidence,fixtures}`. This target was reused through CP4. Evidence preserves candidate1–candidate10 executables, CP2/CP3 builds and focused proofs, failures and corrected continuations. |
| Earlier CP4 binaries | `C:/file-native-online-cp1-20261020/build/release`; engine, two consumers and four test executables. All seven freshly match `evidence/cp4-compile-only-v1-result.json`. |
| Online history/matrix | `C:/file-native-online-matrix-20261020/{history,document-controls,protocol,receipts}`; authentic N128 histories, H1002/H10002 baselines, frozen inventories, old loose/PZ lookups and independent-oracle evidence. |
| Windows diagnostics | Same receipts root contains original r4/r44 controls, readiness attempt01/attempt02/diagnostic01, cadence work and checker batches. No attempt03 result was found. |
| Control records | `C:/file-native-online-control-20261020`; admission/owner/supervisor/lock files. Their existence does not establish currently held locks or running processes. |
| Cargo/cache | `C:/Users/spmcc/.cargo/{bin,git,registry}` and `C:/Users/spmcc/.rustup/toolchains/1.89.0-x86_64-pc-windows-msvc`; dependency completeness was not tested. |

The earlier CP4 engine executable is 3,357,696 bytes, SHA256:

`f154d60f02d86582206cbdf5291da5a4e74db205c714ff1485c2a7ded4ff14af`

Its receipt binds source-freeze SHA:

`9ec7c33e31ae7a23b5c0079dccc13d3f4f2a7c1798438f57458305bfd8920252`

That receipt explicitly records **20 compiled test functions, four compiled test executables, zero tests, zero engine calls, zero protocol calls**. It predates the final 31-test source. Current `.d` files identify the CP2 source location but cannot authenticate today’s source bytes as those earlier build inputs.

**Evidence and result boundaries**

| Pin or activity | Actual evidence | Qualification limit |
|---|---|---|
| d70 baseline / a0b92 integration | Native packing/recovery/compatibility evidence, executable bindings, operational backups and later Mac/B followup | Baseline only; historical file-count negatives remain negative. |
| CP1 versions | Builds/four small units, native ownership/sharing/fault/online/stress prefixes and independent full oracles | Version-specific; failures and partial attempts remain preserved. |
| CP2 `7a6ee9db` | `cp2-build3.json` exit 0; stdout **4 passed, 1 filtered**. Focused retired/delta/outcome/oversize controls | Heavy cache excluded; padded structural oversize control is not an authentic large corpus. |
| CP3 remediation | `cp3-build9.json` exit 0; stdout **11 passed, 3 filtered**. Green A/B owner/dedup/conversion/corruption results | Larger structural tests and heavy cache skipped; later edits prevent promotion to final-pin evidence. |
| Earlier CP4 | Verified compiled binaries and source freeze | Compile only, 20 tests, zero executed. |
| Final `0628322e` | Clean source, 30/30 frozen-file matches, 31 source tests, prepared protocol/pace/forecast controls | Final build/list/test31, full real fixture, real pilot and complete matrices remain unrun in inspected evidence. |
| Original Windows controls | r4.3 cadence violation; r4.4 collection-peaks row-cap failure and generic outer RuntimeError | Neither completed all 27 controls. Generic error does not identify a unique underlying cause. |
| Later Windows diagnostic | All 27 expected outcomes; operator exit 0; EOF and Job/root closure; historical closure of 457 original identities | Diagnostic only; original inconsistency did not recur. META readiness qualification remains false. |
| Later Mac/Linux records held here | Readiness closeout records Mac NORMAL/owned-stop success, keeper-death failure, and Linux blocked execution/Python compatibility problem | Historical returned reports, not current cross-host observations or Windows engine results. |

The original packing report says the N8000-B comparison was incomplete. Its later followup records **all four B comparisons** and a separate **24-case / 282-event Mac recovery run**. The followup must be read alongside the original report.

I independently checked the later Windows diagnostic’s seven small proof artifacts against its retained bindings: **all seven hashes match**. The original terminal `summary.jsonl` row corroborates all 27 expected statuses.

Raw diagnostic evidence:

`C:/file-native-online-matrix-20261020/receipts/windows-bounded-guard-readiness-20261021-diagnostic01`

Key bindings:

- Summary SHA: `c3c7d1b55efd4c58be35e97a58d1a1e489b20f37f1b51bb3cae3e41e065f041c`
- Operator receipt SHA: `fc0bd6ed709c088cc1a5b01be99b9f08ec7ba9d7812b0515c1d57ffe8d8e6ad8`
- Outer completion SHA: `9df526fe9266c36332002e70d89d41f77f2b3536e75530f3eb0751d38cd7afef`

Raw operator/completion `qualified=true` is narrower than readiness qualification. The retained META classification is `INCONCLUSIVE_UNREPRODUCED`, with `META_readiness_qualification=false`, `native31=UNRUN`, and `real_pilot=UNRUN`.

Several labels can mislead:

- Root `README.md` and `artifact-manifest.json` describe the initial admission-only closeout.
- Root `final-closeout.md/json` describes the subsequent experiment endpoint.
- `execution/readiness-20261021/meta/final-closeout.md/json` describes the later readiness endpoint.
- October 24 consultation files contain advice and extraction records, not engine build/test receipts.

**Original brief, closeouts and retained outputs**

The original online brief is:

`C:/Users/spmcc/.codex/worktrees/file-native-online-maintenance-20261020/handbook/docs/research/file-native-online-maintenance-2026-10-20/inputs/consultation/next-experiment-brief.md`

It is 22,544 bytes, recorded SHA:

`4bed1f24aa9f7e014cb0ae93b65dbcf712d2f91d2d3a655af9d254c14cfa425c`

Its proposed wording predates execution authorization.

The final experiment closeout is under that same research root as `final-closeout.md/json`; archive HEAD is `6b689db5afb589a6967c504200afca24add6fb73`.

The later readiness records are under:

`execution/readiness-20261021/meta/`

They include final closeout, artifact index, Windows diagnostic proof and final engine source verification. These later records remain untracked.

Original owner output root:

`C:/Users/spmcc/.codex/visualizations/2026/10/20/01a15ab8-d85f-71b1-8d37-c3c5689790be`

It retains CP1–4 reviews, native/platform/finalizer/checker/resource reviews, returned Mac source, transfers and `final-source-hashes.json`. The readiness META task-output directory exists but was empty during inspection; its substantive records are in the maintenance checkout. A task-output directory alone does not exhaust that chat’s work.

Native Linux references, read from existing nonsecret configuration and corroborating experiment source:

- SSH alias: `spenser-linux-codex`
- HostName: `100.107.14.113`
- User: `spenser`
- Product root: `/home/spenser/file-native-online-linux-20261020`
- Intended staged suite: `/home/spenser/file-native-online-linux-20261020/source/final-suite-v5/readiness-20261021`
- Older packing binary reference: `/home/spenser/file-native-p-linux-20261020/bin/candidate-linux-final`
- Cargo/cache: `/home/spenser/.cargo`

No credentials were printed and no SSH connection was attempted.

**Remaining product work and recommended next action**

Reusable components already exist: exact final source, earlier native source/binary bindings, generation-pin/rolling/paged implementations, independent decoder/checker versions, negative fixtures, authentic H1002/H10002 baseline cohorts, protected long-history operational archives, and a source-only real pilot.

The smallest next Windows product action is an **ordinary supervised compile-only build of exact final `0628322e`**, using the CP2-named source checkout and a fresh task-owned target. Retain source binding, command, stdout/stderr and exit status. Preserve the earlier `f154…` binaries and test evidence.

Only concrete build inputs need resolving: existing toolchain/dependencies, writable output, sensible supervision and stopping. A missing offline dependency would be a specific build-input issue. Old private-cache, clock, allocation and qualification machinery is not intrinsically required to establish compilation.

After compilation:

1. Run all **31 test bodies unfiltered**, including the actual **80,976,318-byte fixture**, with enough real workspace/fixture room.
2. Run the real A/N64/H3 online pilot: publication between capture and changed physical activation; separate publisher, maintainer and old-reader processes; old reader held over 15 seconds; exact current/old outcomes; actual pin release and cleanup; complete independent oracle and closure. The retained recipe allows at most **37 engine invocations**.
3. Continue the broader unanswered product experiments: small A/B adversaries and rolling cycles, H1k/H10k scaled concurrency and same-cohort paged lookups, retained long-history lookup/restore/successor, final-pin native compatibility/recovery, actual 1,000 Windows physical activations, cleanup/backlog benefit, construction/cache bounds and appropriately scoped performance measurements.

The Windows pilot factory remains prepared source with unresolved review/interface/parent-producer bindings. The historical Windows166/Linux158 and broader HISTORY/document/quota/retirement matrices were not completed.

Known concrete shortcomings are mainly in selected infrastructure:

- Windows owner/retained-handle uncertainty was not exercised by the diagnostic nonreproduction; unsafe raw handle-close injection had been rejected.
- The historical Linux copier’s signal-disposition weakness and later Python 3.11 `Path.is_junction()` incompatibility affect those routes. Later timeout-repair source exists; that does not establish native execution.
- Mac keeper identity/recovery shortcomings belong to the separate host audit.

No newly demonstrated final-engine functional defect emerged from this read-only inventory. Final correctness remains untested. Compilation, synthetic harness success and sampled RSS cannot establish the operating envelope or a hard transient-memory bound.

Supporting snapshots are saved beside the report: worktrees, refs, standalone roots, binaries, live final-source verification and CP4 binary verification.

Coordinator delivery remains blocked: the requested `local` route returned “no rollout found,” and the discovered Mac-host route was rejected because the messaging tool requires approval while its approval policy is `never`. No report-back message delivery is claimed.
