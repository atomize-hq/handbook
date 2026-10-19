# Independent review and reporting corrections

A separate review agent examined the core lifetime/replay correction, then the
history truth/export chain and measurement interpretation. Review was read-only;
the reviewer launched no workloads and edited no files. This is local independent
review, not a new external Pro consultation.

The focused Candidate v2 review found no blocker: deterministic replay captures
and resets exact object/file sets, eligible GC retains them, reopened selection
compares them, and accepted retry resolves before old candidate parsing. The
old-source failure and final A/B regression remain distinct evidence.

| Review finding | Disposition |
| --- | --- |
| Missing/censored engine clocks became false zeros; A phase key mismatched | `summarize.py` now emits null and reads `A-completion-inclusive`; direct summary checks passed. |
| Original 8/64 imports include equal-byte repeats | Report labels them 7/56 genuinely new source changes; separate `genuine-*` batches and independent old/final digest verification provide corrected observations. Summary filter includes them. |
| Packet export could scan an orphan publication | `verify_genuine.py` authenticates the positive selected predecessor chain and exact combined-packet manifest binding. Packet byte equality alone is not acceptance proof. |
| Top-level API success is weaker than accepted/completed publication | Report separates nested disposition, durable authority, A completion and censor. History receipts require both `published` and `accepted`. |
| Allowed long-history GC may fail or censor | Report uses actual raw disposition/counts; subsequent request resolution is not evidence of reclamation. Small supplement/replay cases prove actual source/intermediate deletion. |
| History oracle does not replay every intermediate graph | State its exact scope: complete selected chain, membership/outcome/result equality and independently reconstructed final graph/index/source recipe. |
| Raw history category `equal-content` performs cost=0 | Label as already-zero cost assignment/equal-state operation; no source import/dedup timing claim. |
| History per-row and controller timings differ | Per-row engine excludes ledger serialization/sync; controller includes it, startup/capture and watchdog effects. |
| Tree geometry is not total storage; export is not restartable | Keep selected-tree census, whole-root actual storage and conservative quota/owner charges separate; call portable export a proof capsule. |

At least 10,000 dense successful ledger rows plus authenticated chain/index/result
equality are required before claiming the mandatory durable depth. A selected but
unreceipted censored tail stays uncertain/censored. The original native API,
process-death and power-loss qualifications remain controlling.

A follow-up read-only review found no blocker in the strengthened touched-header
history proof or independent per-document renderer. It confirmed hash-checked
node reuse with parent height/range checks, genuine old/new source and label
transitions, and unchanged graph/projection roots for maintenance/equal-state.
The renderer's static root navigation text and requested packet recipe are not
independently recomputed there; selected digest checks and separate packet cases
cover those stated scopes. B link counts establish current hardlink independence.

Late audit review found and reproduced a required-evidence gap on 7ac in A/B:
full rendering recreated missing bulk-layout projection children and navigation
sources before checking them. Requested packet FileSpec sources could be missing
or corrupt while separate retained capsule copies still passed. The N128 owned
before-probe preserves all responses and exact diagnostic interval; missing root
already refused. The correction traverses and hash-checks every selected source
before oracle generation/exclusions. Explicit readable-view repair is unchanged.
GitNexus full_oracle impact was UNKNOWN (target absent from stale index); scoped
inspection found the explicit audit API as its sole direct caller. Ordinary
publication, history selection, formats and owner contracts are unchanged.

The final verification controller received a separate read-only review before
launch. It now records remaining commands unperformed when the reporting deadline
expires, checks that the complete guest cap fits, and passes the reporting epoch
deadline to the guest watchdog. Two waiting controllers were replaced before any
work ran; their cancellation receipts are preserved. The guest 6 GiB guard covers
all owned incremental source/build/runtime roots, so local+6 GiB admission is a
conservative combined bound, not an extra uncounted guest allocation.

Subsequent read-only review separated accepted evidence from retireable private
candidate/graph contents in the proof. Exact requests are independently derived
and hashed against selected intents. Strict pre-GC proof retains all transition
checks. Explicit post-retirement mode validates selector markers, still checks
every accepted result/index/intent and final full state, and lists unavailable
retired header transitions. Fresh H82 adversarial fixtures proved candidate
independence, rejected altered requests/results, and distinguished legitimate
retired graph removal. Proof export/restore precedes any long-history retirement.

Review also rejected a generic telemetry join that could attribute a concurrent
writer's CPU to a read-only/deduplicated reply. Fallback now accepts only exact
writer-producing dispositions/selected IDs; explicit PID, executable hash, call
window and OS lifetime checks remain. Synthetic attribution fixtures are separate
from actual engine events. Missing samples remain unavailable.

A final static review found the paired-intent reservation recovery blocker:
preparing two different intents for one ID before either selects could poison
unrelated recovery and prevent abandonment after the winner selected. Windows
A/B reproduction confirmed it. The reviewed correction uses acceptance without
the losing intent for internal accounting, transfers only the exact accepted
attempt with matching intent/owner, and preserves public intent mismatch refusal.
Windows/Linux/Mac each passed 181 focused responses and locked tests. WSL remains
queued. Ordinary history/document measurements keep their earlier pins.

Final proof/admission bounds use the original serial prerequisite's file creation
time, conservatively preceding the reporting lane start. Replaced waiting queues
are preserved as unexecuted setup history. The focused WSL accounting export is
qualified separately: periodic monitoring covers engine work, completed export
has final footprint/free-space checks, and a host timeout leaves guest export
completion unconfirmed until inspected. Final process quiescence and combined
resources still require actual post-run evidence.

The report review distinguishes the main 7ac Windows measurement build from the
later 5a accounting build, each with its separate source receipt. Saved history
reproduction paths now share one disposable-store value; this does not modify
the already loaded long-running harness or remove its original setup refusal.
The evidence inventory's immediate byte/membership round trip passed. Linux
census review qualified per-path block totals (possible hardlink repeats) and its
explicit engine/Python process filter; it does not claim universal supervisor
detection. Completed controller records remain separate termination evidence.

The closing claims review checked the completed 100k/checkpoint/lost-reply counts,
homogeneous groups, storage totals, contention dispositions and distinct source
pins against retained observations. It found no additional blocker. The aggregate
projection pass's 900.406-second censor is retained, with four completed matrix
roots; the separately admitted two-calibration-root check does not claim A100k
completion. WSL's 7ac common capsule and companions completed 771 responses,
647 success exits, 71 expected refusals and 53 injected exits, with no censors.
Its strict 7e correction also completed 42 responses and locked tests/build.

Pre-execution review of the final resource census caught an admission/enforcement
gap: two subprocess caps could outlive the admitted 600 seconds. The helper now
uses monotonic per-file/directory reserves, bounded Git/guest/PowerShell waits and
a guest alarm; a separate recorded controller enforces the actual outer600-second
descendant-process cap. Review confirmed the corrected bound, worktree resolution,
NUL delimiter and conservative accounting/process-filter qualifications. This
static review is separate from the final census's actual disposition.
