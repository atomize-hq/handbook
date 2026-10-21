# Independent affected-source review: Guard R4.4

Verdict: SOURCE_ACCEPTED_FOR_BOUNDED_PHASE_ADMISSION for this exact prospective sampling/observer repair. No blocking finding in the affected change. All actual finite27, denial regression, fixedSSH/native terminal and later phases require separate all-sink runtime admission. This is not a native qualification or a guarantee of40ms scheduling.

The failed R4.3 attempt remains permanently unqualified: outer active recorded gap42.3152ms, case02 next-start gap45.9856ms, only six completed case records,07partial and08–27UNRUN. Nothing in this review rescales, drops, rescues or reclassifies that attempt. No Cargo feature is present or accepted.

## Exact source identity and review boundary

| File | Bytes | SHA256 |
|---|---:|---|
| job_guard_bounded.py | 77669 | 3a8cf04020b3402186ec0a30c0b21bf57512aa43b1b459a18d92a13a0186e430 |
| unlocked_capture.py | 63278 | 32d928eb6bfbdef2f445a36c18bd721b9fd58d40f6de472ec7f0ba20cfe98806 |
| controls.py | 15022 | 29de5e8d71b5cd7f14e9c5573f86b6bf4f1f23588665a81686545e766e6e7b85 |
| README.md | 3893 | eec097df175de691bb78fe9797599cbb375787b21496729a05b7b80422068fe0 |
| source-receipt.json | 4085 | f0cbfd7f7e2f03b0d7e6dad9ea80657cfd4beeba10ee71657024527c09b64534 |

Total163947/166912 bytes,5/5, all within the individual058caps. Budget058 SHAea7e3478651f0dc53c8a04dd8d74e0a8d9b7073535eeb8c5d13e289e0ec967d1 remains controlling. Source receipt freeze10:17:10.321675Z; reviewer began10:27:39Z. Final pre/post artifact checks are recorded/delivered.

Fully read the changed guard run function913–1382, including the complete nested memory_watch and all acquisition/monitor/publication/cleanup paths; complete outer _start plus adjacent start/finish paths; all297 control lines including every injection and reset; and complete README/receipt. This is an affected-source review grounded in the prior full-five R4.2 review and source-bound R4.3 timestamp acceptance. It is not a new full read of every unchanged guard/capture line.

Independent RAM text reversal proves the complete code delta:
- Replacing only guard1101 time.sleep with the prior stop.wait reproduces77668 bytes/SHA48feda3aee5cabe4140ea6ca5604e7a4dce43276b82de86e5cf1d409d6fdd6a7.
- Replacing only Capture659 time.sleep with prior self.stop.wait reproduces63282 bytes/SHAf095c98c591ce8349a1caa61219a014288785244996844d8ab26afdf110d70ee.
- Reversing only the seven-case conditional/indentation at controls237–239 reproduces14902 bytes/SHA24b88e544417d909cebdd5dd4e4cb993e35dcfa230b30d3f3059cb3238c2d9c5.

Each reversed source matches the exact previously reviewed whole-file hash; no hidden code change is accepted by this method. README and receipt were independently read in full. Preservation tree27725048354049d99326234ef37434ac56e07fb5 is recorded as supplied; no Git command was used.

## Wait substitution and lifecycle

Both call sites change only the waiting callee. The remaining delay expression stays max(0,(next_tick-current_QPC)/1e9), with the existing20,000,000ns due increment and existing max-to-current-QPC late-grid handling. The wait return value was unused. The existing time module is already imported. No frequency, clock domain, counter width or40,000,000ns threshold changes.

Inner memory_watch still checks stop/state on each loop, requires positive initial suspended-root RSS, detects a late start before collecting and checks collection/completion/recorded gaps. Outer _start still requires prepared native capability, performs final pre-resume limit/readiness checks, checks the actual endpoint before waiting and handles late samples through fail/finally.

time.sleep does not provide the old Event.set wakeup behavior. A stop during the sleep is therefore noticed after it returns; ordinary remaining delay is bounded by the same nominal20ms grid. No hard wake-latency guarantee is inferred. Independent native termination/absolute timers, stop checks, checked worker joins, unknown-closure fences and nonqualifying cleanup remain controlling. A stalled/unjoined sleeper cannot manufacture positive physical closure or qualification. The source retains failure on observed>40ms cadence.

Read cleanup paths retain root reap, two native empty-Job observations, checked identity/process/Job closes, final SAME-Job denial accounting before close, and independent health versus physical closure. Unknown acquisition/IO stays retained and blocks admission. Final receipt/gzip/stdio/census and caller completion remain bounded. Whole-call initial/worker3600s,controls<=900s,actual tighten-only endpoint and12s nonqualifying expiry cleanup remain unchanged. Functional11:10:14Z and closeout13:10:14Z do not reset.

## Complete trace-hook dataflow

Only installation is conditional; the trace implementation and all27 IDs/order remain unchanged. Both threading.settrace and sys.settrace are installed together for exactly these seven cases:

| Case | Required dataflow retained |
|---|---|
| slow-first | memory_watch line locator delays collection60ms and records injection hit. |
| start-gap | Later memory_watch tick delays60ms and records hit. |
| root-end-gap | Main run root-end locator blocks sampling, delays65ms, records hit; gate release remains. |
| whole-job-gap | Main run descendant/tail locator uses the same block/delay mechanism. |
| error-storm | Main run wait locator appends bounded controlled errors and records hit. |
| pipe-fence | Owned drain trace stalls only with actual private data; immutable false row and separate owned reconciliation remain. |
| exit-race | Trace populates holder.terminate from the actual memory_watch frame; memory shim uses it for controlled descendant departure, waits for native signal, then records hit. |

exit-race must remain in this set even though its fault is invoked by the memory shim. It is included. Missing-live-rss uses the always-installed memory shim and its positive LIVE check; it has no trace-only dependency. eof233 retains the PeekNamedPipe shim; census-error retains owned-disk walk/onerror injection; close-creation/transient retain their separately owned negative-child programs. All evidence-cap/stdout/stderr/time/identity cases use their original mechanisms without trace installation. Benign normal-nested and sequential128 no longer incur the trace hook.

All other injection requirements still assert a hit. Expected censors, actual closure/census/raw gzip/stdio checks, >=129 cumulative sequential identities, termination-attempt limit and fail/UNRUN propagation remain. Both trace hooks are reset unconditionally in finally276–277, with all patched functions restored and unknown controllers halting later admission. No required control or fault is omitted to obtain a pass.

## Evidence limits and prospective claim

The prior raw audit independently verified outer/inner02 stored/raw SHA and single-member EOF. Its tape locates inter-sample stalls rather than slow collection, but lacks marker-write-end and wait-entry/return timestamps. Therefore it cannot uniquely distinguish marker IO, wait/wake, scheduling or GIL/trace contribution. That uncertainty remains.

The author-reported eight-second pinned-Python probe and selected pure argument fixtures are supporting prospective evidence only. They are not an engine/native workload, were not rerun by this reviewer, do not establish the old unique cause, and do not prove a future27/27 result. The existing write-marker measurement boundary remains unchanged; no new telemetry or claim of complete marker-flush timing is made.

Source review accepts changing the waiting primitive and limiting observer hooks while retaining actual cadence censors. A fresh finite attempt must retain true timestamps, all27 outcomes, all sinks and negative history. Any new cadence miss remains negative under the existing40ms rule. No repeat, prefix reuse, new runtime GO or resource permission follows from this source verdict.

## Audit and accounting

Source-reviewed no-Job-handle-duplication and no-inheritance declarations are inherited from the exact unchanged resource paths and prior full review; no new native-handle action exists in this delta. Actual budget/bounds/config associations,12-key source-freeze allocation and32MiB sealed receipt caps remain byte-identical to R4.3 code outside the proved substitutions. FixedSSH transport and timestamp repairs are preserved; unrelated HYST/body/Context/native reviews remain separate.

No reviewed module import, AST/compile, native API, sleep probe, test/control rerun, SSH, Job, engine, Cargo, Git, source edit, subagent or new chat was executed. Review used text/hash/metadata only and writes only this allocated pair. Structured standard remains proposed; no migration/default approval. GitNexus tools are unavailable and Landing registration was UNKNOWN; manual lifecycle impact HIGH, no index bootstrap.

The exact reassigned071 allocation is P/bounded-guard-review/R4.4, jointly24576 bytes/2. The unmaterialized Cargo allocation is not duplicated. v15 membership SHA9d058392f529481f43fffadb81ef9644234520aa5aa46e54c6eb13d9a5df0d2c, cap11862016. Fresh complete physical-path census precedes each write and follows the pair. Future=112617+remaining R4.4/Windows/Qualification three24K pairs+remaining v38return221184+remaining v38review24576; publication0. Actual occupied bytes reduce only their own remaining caps; prior closed holds remain0. Unused pair remainder remains held until parent explicitly closes/releases it.
