# CP4 staged-handoff remediation review: freeze 9ec7c33e

**Bounded engineering disposition: INTEGRATE exact frozen source/control set `9ec7c33e31ae7a23b5c0079dccc13d3f4f2a7c1798438f57458305bfd8920252`. Full qualification: INCOMPLETE.** Original freeze924bf794 F1/F2 are addressed structurally. No new blocking defect was established. This accepts the bounded engineering source handoff; it is not build/RUN GO, a committed-pin claim, native qualification, allocation expansion or automatic merge.

The selected implementation is the active-namespace STAGE alternative. The separately discussed prefix-repair alternative was not reviewed as implementation and is not required by this verdict. The original negative924 report, its manifest and the a3/CP3 reviews remain immutable historical evidence.

## Exact boundary and actions

Base/HEAD: `a3f9805eccc19ede8dd5c735b54e675e98e63714`, same CP2/CP3 checkout. Four Rust files remain modified and checkpoint4 untracked. Source-freeze-stages.json is6,774bytes. All29 recorded file sizes/SHA-256 values independently match:25 Rust/Cargo and4 controls/docs. Only the five declared entries differ from924: lifetime.rs and checkpoint4 README/control.py/protocol.py/forecast.json. pack/packing/store match the prior fully reviewed source freeze.

Current lifetime SHA-256: `3172ba1b1e78d048c2eb168b02ca524b4d3979abaa6528aaebb149a8a2c3b824`. The previous three changed source hashes remain pack a421e97d..., packing9b000069..., store379d42d6.... No Candidate/publication/result/pack/page/Cargo representation change was added.

Full current lifetime.rs, all four complete checkpoint controls/docs, all29 bindings and declared change membership were read/verified. The prior full four-file review and causal API/Store/rolling/capture/collect trace remain applicable where hashes are unchanged. Five-axis source review was local, with no subagents. GitNexus remains UNKNOWN for the unregistered exact checkout; manual blast radius remains HIGH across reservation admission/selection/failure ownership, collection, native files and construction accounting.

No reviewer engine/build/unit/protocol/oracle invocation, source edit, fixture mutation/copy/recovery, indexing, commit, new chat/worktree or subagent occurred. All20 source tests and the prepared protocol remain UNRUN at this review boundary. Old ce8b/a3 execution receipts do not qualify the new source. Source-read completion and freeze release were notified to meta before report generation; source was not reread afterward.

The structured standard remains proposed. V2 reservations are an explicit private disposable owner/version contract, preserving active8/quota32GiB/terminal4096 and selected semantic/public-format contracts. All terminal garbage remains charged; there is no online terminal GC.

All engine line citations below are within `docs/research/file-native-incremental-engine-experiment-2026-10-18/src/` in this byte freeze.

## F1: addressed — incomplete receipts stay in preclaimed private stages

`lifetime.rs:72,85-96` defines fixed attempts/<attempt>/terminal-{receipt,control}-stage slots and checks both absent before persisting a NEW ordinary sealed-preselection owner. Legacy/preparing-before-opt-in provenance cannot manufacture this claim. The durable active owner, exact candidate digest/request/intent/expected identity and full original allowance establish stage ownership before creation.

Release still authenticates the exact active preselection owner, historical expected selector and current outcome/semantic/physical/control tokens outside W (`270-317`). It validates used+128KiB<=original allowance and complete replacement quotas before writing. An active-absent retry still requires a complete matching final receipt; it has no repair/write path.

For an active owner, prove_stage opens any retained stage without truncation outside W, checks bounded regular/native identity and takes a nonblocking exclusive lock. A missing stage stays uncreated until the W-held token CAS succeeds. write_stage (`240-258`) uses create_new for a missing path; if another writer made it appear after preproof, it returns deferred rather than adopting the file under W.

The receipt stage can be empty or contain any legitimate bounded interrupted prefix, because its authority comes from the exact durable active namespace claim, not those bytes. It is rewritten using its verified handle, split-write interruption checkpoints, file sync and parent sync. Only the fully synced complete stage is atomically moved to abandoned/<attempt> (`327-336`). Thus death after create, before first write or during prefix write leaves no partial final fence and retains active full charging.

The final receipt is never repaired/overwritten. After the exact three-token CAS under W, receipt_control rereads actual final bytes (`322-324`), covering a second caller that previously saw None before another writer published a receipt and died with unchanged registry bytes. Complete exact bytes resume via sync; mismatch refuses. Control-stage mutation follows publication/resync of the complete final receipt. ONE whole-envelope replacement still removes exact active ownership and adds terminal charge/count atomically.

The selected active namespace already owns both slots; benign stale control-stage contents are not authority. No control payload is used for a decision before being replaced by the freshly prepared exact registry bytes and committed atomically. No global pending, filename search, random per-retry stage family or full receipt enumeration is introduced.

Static crash trace:
- no stage: active full allowance, no fence;
- empty/prefix receipt stage: active full allowance, no final fence, point-recoverable stage;
- full stage before publication: active full allowance, no final fence;
- complete final receipt before control: active full allowance/fence, exact receipt resumes;
- empty/prefix/full control stage before replacement: active allowance and immutable final receipt, stage never authority;
- replacement visible with uncertain reply: active absent plus aggregate/complete receipt, point retry does not increment;
- reply repeated after a fresh same-request acceptance: complete terminal receipt remains bound to its original attempt, accepted new result is preserved.

This closes the original direct-create-final receipt window. Runtime death/restart proof remains required and is not inferred from this source trace.

## F2: addressed — verify native file identity before destructive access

`lifetime.rs:195-264` introduces same-handle verification:
- Unix checks regular-file metadata, nlink==1, dev/ino; existing-path preproof compares the opened identity with symlink_metadata.
- Windows queries GetFileInformationByHandle, checks regular type/reparse attributes and nNumberOfLinks==1, and records volume serial plus64-bit file index.
- Existing stages open with no-follow behavior, read/write access and NO truncation; Windows sharing denies other writers/deleters while that handle remains open.
- A nonblocking exclusive stage lock is held from preproof until the stage writer uses/drops that same handle.
- Immediately before set_len/write under W, write_stage rechecks the same handle's native identity, single-link/type and size. New paths require create_new and the same native checks before truncation.
- Complete final receipt reads also use no-follow/native single-link checks, so the formerly accepted hardlinked final receipt shape refuses.

The original regular-hardlink-to-SELECTED case now fails on nlink before set_len. A hardlink that appears at a previously absent stage path is not opened/truncated as an existing file under W: create_new returns the race deferral. Identity/path checks are native bounded control work, not candidate/index hashing or a directory traversal inside W.

All file bytes are synced and the open stage handle closes before pathname rename. This avoids the writer blocking its own rename on Windows. Existing-proved stages remain protected through destructive writing; W serializes cooperating final/control publication. Actual native contention/sharing/rename behavior is still a qualification obligation.

For native-platform source validation, Linux x86_64 O_NOFOLLOW=0x20000 and Darwin/BSD0x100 match the installed locked libc0.2.191 sources. Unsupported no-follow platforms fail explicitly at v2 enable. Microsoft documents volume/file-index comparison and nNumberOfLinks in [GetFileInformationByHandle](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-getfileinformationbyhandle) / [BY_HANDLE_FILE_INFORMATION](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/ns-fileapi-by_handle_file_information), and sharing/reparse semantics in [CreateFileW](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilew). These source checks concern the selected native hosts; they do not establish a general ReFS/network-filesystem guarantee or replace native tests.

## Unchanged causal conclusions

The prior reviewed v2 control remains closed/bounded, version-aware and preserved by all mutators. Legacy owners remain ineligible even when sealing after opt-in. New selection-started is persisted before accepting selection writes; ordinary accepted-result fast paths and lost-reply semantics remain unchanged. transfer cannot remove fenced owners or bypass the started requirement; abandon cannot overwrite JSON receipts.

All expensive release proofs/path walks, candidate/request/receipt hashes and serialization run outside W with shared P and the captured physical handle held. Under W only bounded exact control comparisons, native file/handle checks and declared bounded stage/receipt/control writes/sync/rename occur. No Q acquisition, historical scan, deletion or index traversal is added.

Both stages live inside the attempt namespace. A pre-fence partial left if selection later succeeds remains under the accepted publication's existing allocation owner, whose conservative full allowance explicitly includes intermediates/stages. Terminal release reports zero reclaimed bytes, preserves its complete receipt/garbage charge and never treats unused reservation allowance as deletion credit.

V2 collect still refuses before W/head/scans/deletion. V1 collection still uses exclusive P then one nonblocking W try after mode recheck, avoiding the W->P waiter cycle. No terminal offline GC, hidden stage deletion or new charge-removal path is added.

The O(1) builder capacity sum and production pre-allocation ledger are byte-identical to the prior freeze and remain structurally sound. The heavy test still genuinely combines actual cache, a separately allocated borrowed source and production builder/serialized output with eviction before allocation. Its original80,976,318-byte setup is still required; no reduced arithmetic witness is relabelled an actual64MiB pass.

## Prepared tests and their limits

The complete415-line protocol and all20 source tests were read, not executed. Updated source adds:
-18 explicit handoff death windows covering stage create, zero-before-write, nonempty partial, full write, fsync, receipt publication and control-visible-before-parent-sync;
-10 distinct genuine receipt-stage create/partial deaths followed by recovery, exceeding the active8 count across sequential completed handoffs;
-4 concurrent preproof/death schedules, including stale None observations and unexpectedly appearing stages;
-3 hardlink endpoint fixtures (receipt stage, control stage, final receipt), plus a tiny native unit requiring refusal before canary truncation;
-a preclaim unknown-slot injection that must refuse sealing;
-a labeled structural partial-stage injection followed by an actual selected publication whose allocation owner retains that attempt scratch;
-the retained A/B four-publisher rounds, >15s held reader, other preparation/packet protection, request retry, selection-started ordering, accepted/lost reply, token races and quota/corruption/collect refusal controls.

Fault hardlinks are confined to newly created disposable canaries, with all names/bytes charged and retained. They are not a representation, transport or space-saving technique. Cap4096 controls remain synthetic boundary fixtures, not4096 actual attempts. No actual20-unit pass, process-closure result, byte oracle, native recovery or independently verified full store is claimed.

Control launch remains the same locked/offline workspace/target with unfiltered serial tests and source/control input capture. Guard50ecc QPC cadence and fresh budget/parent RUN GO still control future execution. The previous unused runtime budget and negative source freeze are preserved; this review grants no execution itself.

## Pre-write META accounting

Before writing this report pair I independently enumerated the exact parent-selected closed membership, deduplicating absolute paths: all P regular files; Landing R/execution admission, admission-claims, native-macos, reviews, final-protocol, windows-matrix excluding stream-snapshot-controls, windows-cadence; both R-root and execution-root regular files; and the exact18 promoted harness files.

At that census:917 files /7,792,975bytes, including the new6,774-byte stage manifest. P was5,447,727bytes. The18 promoted harness files sum179,468bytes. R-root files sum34,805bytes and execution-root files36,626bytes. The remaining Landing subtree groups sum the balance; excluded original native-linux/qualification-lanes/windows-final-platform and stream-owned assets receive no reclamation credit.

Adding the full40,960-byte report-pair allowance and262,144-byte future-parent reserve gives8,096,079bytes, below the8,388,608-byte META cap by292,529bytes. This census counts P plus Landing, not P alone, and already includes the parent manifest. This report pair must remain within its40KiB grant. A later census may change as other authorized work proceeds; no new allocation is inferred.

## Handoff

**INTEGRATE this exact9ec7 frozen source set for bounded engineering.** F1/F2 from924 are closed at this source boundary; the original negative report remains truthful. Meta owns any build/protocol admission and later exact committed source/binary binding.

All20 actual units, the actual64MiB/native construction interaction, prepared stage/alias/crash/concurrency runs, complete independent v2-aware oracles, final native Windows/Linux/macOS capsules, full small/scaled/history/long cohort matrix,1000real activations, measured work/latency/cadence, actual coexistence/census and backup/restore plus accepted successor/B selected-once controls remain required. Full qualification is INCOMPLETE. Neither native documentation, static causal tracing nor prepared tests supplies those missing executions.

