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
