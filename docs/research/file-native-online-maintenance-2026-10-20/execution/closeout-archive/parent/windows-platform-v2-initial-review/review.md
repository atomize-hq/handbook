# Independent Windows initial-v2 source review

**REMEDIATE / SOURCE NO_GO.** This verdict applies only to the preserved initial snapshot whose receipt is `1e023ecd0156cad2e2165c83b4e1120f1158b2988bd5d7f5d8180dcb515b73cf`. It makes no claim about newer RAM/live source. A fresh review of the final frozen packet is required after source cutoff; no author code changes after cutoff are authorized here.

The immutable source is the adjacent `source` directory: exactly 27 files / 341882 bytes, receipt 7619 bytes, canonical 26-entry manifest `ace7e15a6753feb121d4b4a09ae12602e6bdf9857a56c4bc9778a57b0408a485`. All file sizes/hashes match the receipt and every decoded member of `transfers/windows-platform-v2-initial-1e0-source.json.gz` (126171 bytes, SHA256 `c1add25f71315aeff4a563fc53ab0cfc227adb92bd68bcb17782c27f67775f26`). No archive was extracted or source copied by this reviewer.

## Actionable source findings

### V2-1 / P1 — correlated metadata mutates an already ended call

`windows_evidence.py:68-98`, triggered by `windows_capsule.py:240-241`.

The v2 append path intentionally sets local `token=None` for kinds outside begin/start/data/rss/end, but line 77 reconstructs the token from the unchanged value's `call`/`call_token`. Lines 94-98 then update that call's hash, byte count, frame count and end offset for any correlated metadata. `LaneHost.invoke` appends `invocation-closure(call=token)` after native END. This metadata receives no sequence at lines 69-71 but extends the completed call's accounting. The original reference returned at END stays valid, while later `call_reference(token)` and response projection obtain a different reference ending in unsequenced metadata. The extractor rejects that reference because END must be final and every selected row must have its exact next seq. It breaks the forensic reconstruction contract on a normal successful invocation, without a fault injection.

Keep correlation separate from frame membership, freeze call accounting at END and refuse additional same-call frames. Metadata may refer to the immutable END reference without entering its hash domain. Prepare a trace for successful END -> invocation metadata -> projected response -> extraction, checking reference immutability throughout. No such trace was executed here.

### V2-2 / P1 — settlement asserts closure without observing it

`windows_capsule.py:169-174,312-333`; `windows_evidence.py:154-160`; consumer `windows_growth.py:146-166`.

`writer_closure_proof()` returns process_reaped/io_closed/callbacks_joined/raw_writer_released=True unconditionally. `GrowthLedger.settle(actual,proof)` accepts those booleans and replaces the upper lease with actual counts. During finalization, the stream is closed and settled even if drain failed or a callback remains live. `Evidence.close` neither acquires the append lock nor fences a writer already inside append after its not-closed check; it can snapshot and settle `written` while that writer is still completing a physical write. Consequently the proof is not derived from actual writer ownership, and unwritten potential can be released before a source-proven closure boundary. `record()` uses the same blanket proof, including during phases with live processes.

Derive lease-specific closure from the actual owner state; distinguish a synchronously closed record writer from the whole raw/native/callback closure. Fence appends atomically under the same lock before measuring final retained bytes. Retain the upper lease on uncertainty or drain failure. A dictionary of bound source/profile/budget hashes does not establish those runtime facts. This finding concerns release of unwritten potential, not deletion or actual-byte credit.

### V2-3 / P2 — completed intentional reader ends become host drain failures

`windows_capsule.py:294-295,332-333`, interacting with `windows_adapter.py:finish_interrupted`.

The corrected reader-end path validates cancel/death as the exact same call's expected AssertionError row and caches a closed proof. It leaves that exception in the retained callback handle. Host drain now records every non-null handle.error as a callback failure, so a successfully validated intentional cancel/death later produces drain failures and `own_closed=False`. Blanket removal would hide technical failures; instead retain a bounded exact-consumption marker tied to the validated token/kind/row and distinguish that expected completion from start/join/output/RSS/closure errors. The prepared W5 trace checks genuine start/join errors but not this successful W4 -> host drain interaction.

### V2-4 / P2 — X1000 opens the same case twice

`windows_capsule.py:261-267`; exact shared `focused_protocol.py:218-235,803-806` (file SHA `7ce0a4a06a35fa9e7f4f4080a40315a418bb45e47a8946ca06d78c2117d855e8`).

`run_case` opens `harness.case` before dispatching every body. Its X-Windows1000 branch calls `mapping_stress`, which opens another same-root case. Shared case rejects a root already in `active`, so the real 1000-activation body cannot start. Assign exactly one owner to each route: wrappers for donor bodies that need one, or self-owning shared/helper bodies. Check HISTORY/LONG dispatch for the same ownership assumption and exact donor/phase case IDs; do not relax the Harness root guard.

### V2-5 / P2 — donor adapters are names without a complete callable Windows ABI

Concrete independent examples in the initial snapshot:

- `windows_hooks.py:10` omits `strict`, but `CaseTemplates.load:341`, `NativeCase.document:287`, phase observations:212, guard prefix:230 and donor authority:246 call it. Template load fails with NameError before the selected body can run.
- `CaseTemplates.dispatch:366` always passes two arguments. Exact matched `suite_units.py:6` and `suite_ordinary.py:83` define one-argument `run(c)`. Those routes fail even after the missing global is fixed.
- The matched units and ordinary bodies require adapter budget/commands/emit and other interfaces absent from NativeAdapter/NativeCase. Ordinary source is a Bash script invoking `/usr/bin/python3`, not a native PowerShell implementation. Its 17 routes do not become Windows-qualified by mapping `run`.
- In matched `suite_recovery.py:96-118`, selected_once contains an inner import. The loader marks it bad, then transitively marks the common dispatcher bad. Thus even dispatcher branches that do not use selected_once are refused by `load:332-336`. Recovery also needs absent `backup_restore`.

Port the actual closed caller dependencies and callable signatures before marking constructors reviewed. Keep Linux providers out of Windows activation; neither a function name nor source-only AST binding supplies the missing capability. These examples substantiate the packet's declared incomplete ports; they are not assertions that newer author source still has them.

### V2-6 / P1 admission blocker — planned input ownership and all-sink closure remain incomplete

This blocker is explicitly disclosed by the initial packet and independently visible in the matched shared Harness `call:257-306`: request-before-dispatch and call-result persist input payloads, while Evidence.begin_call persists planned input again. `WindowsDriver.call:92-104` can fail in binary verification/admission before `_execute` creates a call token and planned capture. Response projection does not capture those predispatch failures or remove input copies. Thus one-input capture and serializer-derived whole-phase fit are not established. Source-bound planned-input ownership/predispatch reservation, exact current helper invocation schemas, quantitative profiles and complete sinks must be reviewed together. No average, compression ratio or existing retained-byte reclamation closes this gap.

## Review scope and dependency limitations

Reviewed all 27 snapshot files, tracing the changed Python bodies against the exact preserved 034 archive (`dbbe3867c15e07f3ceae1a4f3736d3b5112b66f388e23bc13eebebaed7a77027`, 100021 bytes). Native/QPC source is byte-identical to previously fully reviewed 034. W5's ident guard, isolated joins, retained failures, independent finalization and bootstrap nested finally address the original unstarted-thread abort in source; V2-3 is the new interaction with valid expected interruptions.

The six case maps contain exactly **166 unique IDs**, all PENDING: packing52, recovery24, strict24, focused/Windows12, protocol48, workspace/ordinary6. 165 name an entry; the D-N64-N1k-AandB-payload-title body remains absent. All 20 constructor obligations retain pending fields. Four actual Windows sharing bodies and X1000 are source-visible; the sharing routine uses CreateFileW without delete sharing, exact held hashes, per-HANDLE closure and release/retry, but it is unexecuted. No case name or named source body is counted as a passed case.

Complete matching external bodies read: recovery 8089 bytes / `da30aececf9ae58b48322dfcdb91f33d83928b874236b2c34e600104353d6f2e`; units3189 / `05b49bb269b404eedc433f598328be6217a99823a9ef90ae3d09dcaf1daa10f9`; ordinary6561 / `fccad1637fa9c85f5fb3d77094f13ddc9bc810bca9410dc9aed01f940c80e942`. Shared7ce case/call/mapping contracts and the matching HISTORY run boundary were checked against this adapter. This does not certify every external body or transitive dependency.

Exact mapped versions of three dependencies were unavailable at their paths:

| Dependency | Frozen map requires | Current path observed |
| --- | --- | --- |
| suite_packing.py | 21362 / a551bdec0dc38aff58a7db9014019c130d5ea38eda8275508f1d562fc154d029 | 21996 / c455908bdff907ef35cf424e1cd2d84804fd1577a39de31646513b96551a2ee1 |
| suite_extensions.py | 21182 / 26ca560fd5ff9fca1a6fa90557ffd6ced625908c3ddea5473a8237e72f1c2ad0 | 32947 / 4ad6ad44d3e39251087c353780c1020ed538d3142068059132bb7dcc6a3c6978 |
| long_lane.py | 32459 / 416c98fa585dd5528e79f22b3e1bce8aec8f96fcdddfc5b7ee549290bee59c68 | 29946 / bc018fecaed9a9c691506e1c2406eeb51f2c9a37181d89fb10059b6bc32bd433 |

No drifted body was substituted. The parent explicitly directed this finite initial review to record these missing exact old dependencies rather than seek unbounded archives. Consequently full 166-body acceptance is **not possible** for this snapshot. Original source-bindings observed-helper entries also retain older HISTORY/LONG hashes while the case map names newer versions; final review needs one coherent actual closure.

036 runtime runner/ports, exact 031 guard source/schema, 035 typed quantitative profiles, final postexit join, final engine/checkpoint/unit/native CLI, complete independent terminal-v2/quota oracle, Windows historical producer, ordinary native shell, backup/restore and constructor decoder queries remain unclosed/unreleased in this packet. Bootstrap's explicit differing-guard schema/release gate is source-visible, but it does not prove compatibility of an absent exact controller closure. HistoryPorts.guard_prefix is provisional and must never supply final empty Job/gzip/finalJSON/census evidence. Its row-field expectations and closed port outputs require validation against the actual frozen guard/shared sources in the final review. Extractor RSS/END validation and actual source/profile/identity bindings need source-bound v2 negative controls; none are prepared as a qualified result here.

All64 existing memory groups, including the prepared W5 subtrace, remain unexecuted. V2-specific controls are explicitly NOT_PREPARED_NOT_EXECUTED. Syntax/AST method hashes in owner metadata are not independent reviewer execution evidence. No supplied module import, AST/compile/control/native API, engine/build/Job, source edit, Git/index/bootstrap, fixture/corpus write or delegation occurred. Standard-library byte/hash/JSON/gzip/tar/diff reads and report writes only. GitNexus exact-checkout risk remains UNKNOWN, manually HIGH across transport, registry/callback ownership, helpers and closeout; no graph count is claimed.

## Admission and disposition

Budget039:3167 bytes / SHA `b79a0cccaf96be247bba2b48bada220417bc083eb4b09828aa43642719e5eaa3`. All individual/group/whole bounds pass: source192767/196608 in11, case102129/262144 in8, bindings including receipt46986/65536 in8; whole341882/524288 in27 exact flat names. This is source allocation only.

Before the report write, the whole physical META10 v5 census was freshly recomputed over all recursive P files/archives/copies, closed Landing roots/subtrees and exact promoted20. The measured checkpoint at04:08:07Z was9618693 bytes/1130files, with a second fresh check immediately before creation. The full20480-byte two-report reserve and262144 future bytes fit below10485760. Membership SHA is `ce01f5bc510f5472feaac941b82da16bf78b3c09d737231c4d2f2c9e4d01b3be`; exact prewrite output is retained in the tool transcript. No deletion, relabel, expiry or retroqualification credit was taken.

Final verdict remains SOURCE NO_GO / REMEDIATE. Preserve this negative initial packet and review; review the final 27-file freeze plus its exact dependencies independently. No full-body/native/memory/import authority follows from this report. Source cutoff04:10:14Z, functional11:10:14Z, protected closeout13:10:14Z on2026-10-21 remain fixed. Read-only review after cutoff is allowed; source fixes after cutoff are not. Structured-data standard remains PROPOSED; no migration/default approval.
