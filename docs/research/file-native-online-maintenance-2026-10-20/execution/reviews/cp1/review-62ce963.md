# Independent CP1 source/protocol review

Disposition: **REMEDIATE — source blockers remain; qualification incomplete.**

Reviewed commit `62ce963085ac054bcc34eb21c6733b94db160975`, pushed branch `codex/file-native-online-cp1-20261020`, base `47d667f26a75fa1454dee2d1c50d238aec4cf6bd`. Remote head independently matched this commit during review. Candidate3 is `C:/file-native-online-cp1-20261020/evidence/candidate3.exe`, 2,902,528 bytes, SHA-256 `f1439fd9f222d327ac4c820ca2e0afa5c49bc7e83d8116b47c6b391a593fe4dc`. All 23 Rust/Cargo/consumer binding entries match the committed blobs and build3-source-before manifest. The implementation worktree began changing during review; these findings refer exclusively to committed 62ce963, not the unpublished replacement.

Scope: independent read-only review, manual caller tracing, hashing and parsing existing raw receipts. No engine calls, builds, recovery, corpus copies, source edits, dependency installation, subagents, or extra chats. GitNexus tools are unavailable; indexed caller coverage and index freshness are **UNKNOWN**. Manual blast radius is HIGH across ordinary APIs, nested Metrics, maintenance, semantic writers, consumer binaries, and direct format/backup clients. All new review output is confined to this directory under the assigned 256 KiB ceiling.

The complete controlling brief, reconciliation, structured-data standard, preflight protocol map and frozen gates were read. The structured standard remains proposed; existing owner/version contracts remain controlling. Human admission and fixed clocks are inherited, not reset. CP1 deliberately preserves the legacy whole-index reader and frozen/nonincremental cohort; CP2 rolling discovery/new-tail causal coverage and CP3 paged lookup remain required.

## Required findings

All line references below are within `docs/research/file-native-incremental-engine-experiment-2026-10-18/src/` at 62ce963. These are source-proven paths and proposed deterministic reproductions; this reviewer did not execute the reproductions.

### F1 — P2 Required: replacing PACKING loses aborted ownership before durable handover

Location: `packing.rs:472-473`, with inheritance at `491-492` and `258-276`.

A previous aborted-owned journal remains only in the local `previous` variable when the next build overwrites PACKING with its new building journal. Its containers and metadata are not inherited until construction completes. Schedule: build A far enough to create containers, abort A; start B and die at `pack-reservation`, or fail quota/construction before `append_work`; abort B and build C. C can discover only B's birth directory and direct journal metadata. A is absent from both selected work and the surviving journal, so its files have no future reclamation owner. This violates durable birth ownership, carried unfinished work, and eventual bookkeeping reclamation.

There is also a transitive case: let B finish installing its plan with A's garbage, abort B before selection, then build C. `inherit_aborted` imports B's direct containers/metadata but never B's inherited plan work, then schedules B's plan for deletion. A's inherited obligations disappear with that plan. Fix by durably transferring the complete bounded pending closure before replacing its sole root, including failure during the transfer. A journal status is not deletion authority. Test A-abort/B-death/C-successor and A-abort/B-installed-abort/C-successor with actual old-instance removal and no double credit.

State: **OPEN; routed for remediation**.

### F2 — P2 Required: mutable progress can erase pending work without native reconciliation

Location: `physical_lifetime.rs:169-181`; consumers `packing.rs:404-414` and `248-256`.

Load rejects introduced retirement descriptors but accepts removal of any immutable-plan retirement row, accepts arbitrary `pack_done`/`index_done` flags for an existing descriptor, and accepts removal of immutable garbage. Cleanup trusts the done flags, skips native removal, and drops the row. Successor construction loads that shortened work and can make it the new immutable authority, permanently orphaning the omitted old files. The held-generation omission fix is correct but does not cover these separate work obligations.

The existing `focus3-progress-before.json` snapshot has 10 pending retirement rows, 10 jobs, one older generation and one garbage item. Removing retirement rows or setting both flags true preserves the implemented descriptor/subset checks; retaining the unchanged older list avoids the held-generation omission branch. Dropping all garbage also preserves the introduced-garbage checks. This is a structural check of actual saved bytes, not a new engine test.

Do not accept omission/done claims as proof of deletion. Retain pending obligations or independently establish native absence, with zero absence credit. Validate advanced/dropped loose jobs against their remaining loose obligations as appropriate. Reproducer: from a valid selected plan with old containers present, forge done flags or omit a row, load/cleanup, activate a successor, then verify every obsolete instance and owned metadata item is still tracked or actually absent. Include immutable-garbage omission. A checksum of attacker/corruption-controlled progress alone does not supply the missing evidence.

State: **OPEN; routed for remediation**.

### F3 — P2 Required: interrupted atomic progress saves leave unreapable namespace entries

Location: `physical_lifetime.rs:155`, `126-134`, `195-199`; `io.rs:186-188`; `lifetime.rs:209-216`.

`save` uses `io::atomic`, which writes `physical-progress/stage-<pid>-<time>-<seq>` before replacing `<generation>.json`. Death at a write/sync checkpoint before replacement leaves the stage. It is neither tracked in selected work nor accepted by the garbage path grammar. Generation reaping schedules only the named progress JSON; the semantic collector's stage sweep omits physical-progress. Repeated cleanup deaths can therefore retain whole progress documents indefinitely, potentially up to 4 MiB each, despite later complete cleanup reports.

Also cover the empty `physical-stages/<id>` directory left by death after STORE replacement and before `activate` removes the directory (`packing.rs:328-331`); old-generation reaping has no corresponding directory work.

Provide a bounded M-serialized physical-namespace stage reconciliation path with durable/resumable progress and safe ownership checks. Test death during progress write/sync, restart, later successors, and selected-stage directory cleanup. Count directories and bytes, not only queued file records. Keep all native calls and parsing outside Q/W.

State: **OPEN; routed for remediation**.

### F4 — P2 Required: birth names lack a durable monotonic discriminator

Location: `packing.rs:472`, `io.rs:68`; unchecked ordinal addition at `packing.rs:249`.

The absolute never-reuse claim relies on PID, wall-clock nanoseconds and a process-local sequence. After an old instance has been reaped, PID reuse plus clock rollback and sequence restart can recreate its pathname; `create_new` no longer detects an absent historical name. The meta chat independently identified this issue. Prefixing candidate/generation names with a checked next physical ordinal supplies the durable selected-generation discriminator while preserving the nonce and create-new refusal for aborted same-parent collisions. Compute checked addition before birth side effects; refuse overflow. Preserve aborted ownership as in F1 so an aborted same-parent pathname cannot be reaped and then reused.

State: **OPEN; narrow fix authorized/routed by meta**.

## Fixed paths and supported conclusions

- Older generation omission in mutable progress now requires a fresh exclusive native pin check with exact identity; a held reader causes `integrity:progress-omitted-held-generation`. Missing pin errors propagate. The immutable base's old pin names remain protected until a successor omits them. This closes the earlier reader-safety hole at this pin.
- Progress may introduce garbage only from immutable base garbage or exact immutable older-generation pin/progress/catalog/plan derivations. Selected/retained/job-dependent metadata is protected before deletion. The earlier invented-garbage safety hole is closed; F2 concerns omission rather than introduction.
- Physical capture takes shared Q, reads STORE <=4 KiB, acquires/validates <=1 KiB selected pin, and releases Q before catalog/stat/index/semantic work. Metrics owns the pin through synchronous dependent I/O, including Store-open physical-before-semantic capture. Error unwinding drops ownership; process-exit release still needs native tests.
- Activation prepares and verifies outside Q/W, then takes Q and tries W once, releasing Q on contention. No maintenance-side transfer recovery remains under W. Ordinary original semantic writer/index costs remain baseline behavior. Physical CAS uses exact STORE bytes, so semantic advancement alone does not reject a candidate.
- Initial legacy conversion drains pack-readers once via nonblocking checks, then version 3 refuses old clients. Subsequent activation uses generation pins. Complete legacy native schedules remain unrun here.
- Loose cleanup conservatively waits every retained generation with ordinal before the job frontier, including generations where the key was unmapped. Shared packs/jobs/build input stay referenced; M serializes maintenance. Current B publication controls are verified and retained, including historical selectors. Payload/view/packet namespaces remain separate semantic lifetimes.
- Deletion credit is granted after successful logical removal; reconciled absence earns no credit and native allocation credit is zero. Windows sharing errors 32/33 defer. Lock helper explicitly sets share mode 3 and clears HANDLE_FLAG_INHERIT without duplicating pin handles. Native death/inheritance/sharing proof remains required.

## Evidence reviewed and limits

`build3-units.json`: exit 0, locked/offline release workspace build plus four tests passed, heavy cache test explicitly skipped. Raw stdout confirms four tests, zero failures, one filtered out. Build guard sampled peak RSS 420,765,696 bytes. Its sampling receipt records 8 intervals over 40 ms, maximum 47 ms; do not conceal these or infer a fully qualified resource envelope.

`focus3-events.jsonl`: 14 events, all bind the exact candidate3 executable SHA; 12 exit 0 and two expected exit-2 corruption refusals. The two actual activation hold samples are 760,000 ns and 632,500 ns. There are no per-event guard violations. The guard exits 0; its receipt records 10 sample intervals over 40 ms, maximum 47 ms, sampled aggregate peak RSS 30,691,328 bytes. Two samples do not establish activation percentiles or scalability.

`focus3-results.json` and recorded program: 16.094-second held old reader, two successor activations, intervening new reader and two publishers, retained old containers while held, exact old outcome after release, then 10 original container files absent. Both intended progress refusals are present in raw events. The B selected-once publication/manifest hashes and all 20 paths were checked against existing fixture bytes. This extraction is Python-based focused proof, not the required final PowerShell/Bash extraction matrix.

Candidate2's distinct 16.110-second run and earlier smoke are preserved under their own bindings. They are not relabeled candidate3. `faults4` appeared during review and is a partial failed harness run: the receipt/program binds candidate3, stderr ends `AssertionError: 8681977` on the harness's 8 MiB fixture assertion. Partial successful rows are useful observations; no full faults4 pass is claimed. It does not exercise the nested aborted-owner handover schedule in F1. The old smoke's obsolete collection.yaml assertion remains a preserved failure.

Full source/commit and raw-artifact hashes and compact structural observations are in `evidence-62ce963.json`.

## Open qualification gates

Complete small A/B N64/H64 cycles and independent full oracles; all crash/cancel/contender/stale-CAS/path-reuse/native-sharing schedules; packet/preparation/persistent snapshot and retired-content schedules; legacy old-client conversion/restore/successor proof; actual bookkeeping drainage through 1,000 activations; large-history bookkeeping/RSS bounds and cleanup 1,000-candidate/50 ms behavior; same-final-pin Windows/Linux/macOS capsules; ordinary extraction; full backup/restore and accepted successor remain OPEN. Existing focused saved-owner checks do not close the full matrix.

The fully causal new-unmapped-publication capture-gap plus tail compaction is explicitly CP2-pending because CP1 freezes its initial cohort. CP2 verified rolling discovery/deferred ownership/segments and CP3 authenticated paged routing remain unfinished. There is no complete experiment pass, public format freeze, automatic merge, new semantic index, or CP2 release in this review.

**Verdict for 62ce963/candidate3: REMEDIATE.** Re-review F1-F4 and binding on the replacement commit/executable, retain old negative results, then qualify the required schedules. Meta owns routing and landing.
