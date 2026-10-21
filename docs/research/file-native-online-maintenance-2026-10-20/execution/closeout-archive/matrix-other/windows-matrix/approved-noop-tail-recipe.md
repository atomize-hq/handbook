# Approved no-op tail recipe and admission conditions

The coordinator explicitly approved this method on2026-10-21: all1,600 NEW
accepted tail publications use `ops=[]`, `packet=false`. Original mixed baseline
histories and failed evidence remain intact. This measures publication/history
maintenance contention, not mixed payload transition costs. F07 payload/title
pairs are unchanged. **Execution remains held** for final source/terminal-v2,
initial-setup review, output bounds and actual preflight admission.

## Machine recipe

```json
{
  "version": 1,
  "method": "coordinator-approved-noop-tail",
  "execution_authorized": false,
  "cohorts": [
    {"name":"H1","baseline":"H1000-baseline.json","initial_H":1002,"final_H":1802},
    {"name":"H10","baseline":"H10000-baseline.json","initial_H":10002,"final_H":10802}
  ],
  "phases_per_cohort": [
    {"name":"control","publishers":1,"start_offset":0,"end_offset":100,"maintenance":false},
    {"name":"online1","publishers":1,"start_offset":100,"end_offset":200,"maintenance":true},
    {"name":"online2","publishers":1,"start_offset":200,"end_offset":300,"maintenance":true},
    {"name":"online3","publishers":1,"start_offset":300,"end_offset":400,"maintenance":true},
    {"name":"control","publishers":4,"start_offset":400,"end_offset":500,"maintenance":false},
    {"name":"online1","publishers":4,"start_offset":500,"end_offset":600,"maintenance":true},
    {"name":"online2","publishers":4,"start_offset":600,"end_offset":700,"maintenance":true},
    {"name":"online3","publishers":4,"start_offset":700,"end_offset":800,"maintenance":true}
  ],
  "accepted_per_phase":100,
  "max_offers_one_publisher":100,
  "max_offers_four_publishers":400,
  "unexpected_one_publisher_conflict":"unqualified-stop-no-hidden-retry",
  "request_id":"H{initial_H}-P{publishers}-{phase}-S{slot:03d}-A{offer:03d}",
  "request_id_max_ascii_bytes":27,
  "request":{"request":"generated-id","base":"exact-captured-selector","ops":[],"packet":false},
  "slot_range":[0,99],
  "offer_range":[0,399],
  "readers":{"workers":4,"category":"early","requests_per_call":1,"count":1000,"max_calls_total":40,"max_calls_per_worker":10},
  "reader_limit_before_publishers_finish":"unqualified-stop-no-extra-queries",
  "terminal_v2":{"required":true,"api":"PENDING-FINAL-SOURCE","sealed_conflict":"conflict:expected-publication","preallocation_conflict":"conflict:captured-base"},
  "routine_physical":{"codec":"zstd","segment_limit":4,"discovery_limit":4096,"include_attempts":true,"force":false},
  "retained_failures_max":2400,
  "history_logical_byte_cap":1073741824,
  "history_file_cap":400000,
  "planned_deletion_credit":0,
  "native_controller_sha256":"50ecc7c40c1cdda6493817261b5755fe2d823fdae271118698828c39a0400378"
}
```

Baseline files are under `C:/file-native-online-matrix-20261020/receipts/` and bind
the actual roots, selected pointers, N128/A/project, source/binary and all existing
outcomes. Root names are not guessed from labels. Phase H is initial_H+offset;
every accepted publication is counted, and no other semantic writer is allowed.
Two cohorts x two publisher cells x four100-accept phases gives1,600 accepts.
Eight p4 phases permit at most3,200 offers/800 accepts/2,400 losers. Eight p1
phases permit800 offers/800 accepts. Count every offer, including allocation-free
captured-base refusals. Never convert a timeout, RSS error, busy/quota refusal,
unknown error or lost reply into a normal semantic conflict.

## Preconditions, load and measurements

At original H1002/H10002 FIRST, independently verify exact mixed history/current A
and chosen paged representation, then six-category early/middle/recent/missing/
cancelled/lost-reply lookup:50 fresh/category,500 warm/category. Match L/PZ outcomes,
digests and unchanged SELECTED. Keep all samples/maxima; no device-cold/fresh-p99.

Require N128/A/project, current VIEW=SELECTED, no PENDING, profile4, fallback=false,
exact existing packet-not-requested files, empty live reservations/no foreign writer.
Initial accepted request IDs ASCII<=27, attempt IDs<=79, store ID<=77, selector
publication IDs<=99, SHA64 lowerhex. New unique publication ID<=83:11+3+10+39+20.
Recipe request IDs<=27. Capture SELECTED once/offer without fabricated/rewound base;
bind raw bytes, engine matching and independent predecessor/provenance proof.

Match request/reader load, timeout, source and terminal behavior in both variants.
Control FIRST, compact/reopen separately after100 accepts; report actual H ranges.
Separate success engine, all-attempt conflict, logical-slot end-to-end and phase
clocks, including transfer/backpressure/waits. Prove four overlapping frontends;
no serial substitute or reduced readers/extra loops. Preserve every result/failure.
No EXP_DEFER_REFRESH, hidden GC, owner deletion or valid-preparation revocation.
Freeze terminal schema before injecting fields/flags.

All THREE online phases need concurrent automatic work. A32/64/96-accept trigger
cadence is a proposal for helper review. Count actual generation/catalog/descriptor
succession, not Work-only CAS. Captured old reader must live>15s through two real
successors; retain native pin/file/result/age evidence and separate its hold clock.

## Initial setup decision still required

Original PZ maps all accepted roles but has no Discovery; conversion preserves
None. Automatic build skips mapped keys and rejects below128 removable files,
so an all-mapped partial bootstrap may fail to save its cursor. Never infer semantic
coverage from mapping or rewrite STORE to a loose control.

Proposed explicit SETUP: forced bootstrap chunks<=4096 at original H, then full
mapped conversion preserving seeded state, before initial lookup. Charge all
setup/coexistence; cursor-only births count as neither routine activation nor
automatic benefit. This proposal needs source/coordinator approval; routine force
remains false. After seeding/lookup, the100-control suffix can compact once.

## Source-derived accepted metadata bound

These calculations concern retained logical files, not cumulative bytes written
to replaced registries. Final source must preserve/check these preconditions.
Only selection inserts one prior-publication member and one OutcomeRef; failed
stale preparations stop before those writes. No-op graph/projection/changes roots
are unchanged; existing derived files are checked, not recopied. VIEW refresh has
no actions with the stated current-A/profile/static preconditions. New accepted
private permanent files are candidate/publication/result/hint/allocation;8KiB
per accepted publication conservatively covers their closed pretty-JSON bodies.
Root VIEW/PENDING/registry replacement stages are charged in the temporary envelope.

At the final-source preflight, audit membership/outcome trees: leaves<=64 pairs,
nonroot leaves>=32, branches<=32 children and nonroot branches>=16, height<=2,
no byte-driven earlier splits, no prior semantic checkpoint/bulk underfill, and
initial counts<=1,001/10,001. Only insertions follow. At max10,801 entries, height3
would require>=16,384 entries under these occupancy rules. Refuse/rebudget if any
precondition fails; do not silently rebuild or add an accepted checkpoint.

OutcomeRef compact JSON with request27, intent64, selected ID99/hash64,
result-hash64 and attempt79 is<=495 bytes (<4096 inline threshold). A JSON u8
vector is<=4n+1 bytes. Using exact compact tagged-node overheads26/42 bytes:

| Retained node | Source-derived maximum bytes |
| --- | ---: |
| member leaf,64 pairs, ID99, JSON-string SHA value66 |23,577|
| outcome leaf,64 pairs, hash key64, value<=495 |131,289|
| member branch,32 children of hash64/min99/max99 |9,321|
| outcome branch,32 children of hash64/min64/max64 |7,081|

Each accepted insertion rewrites at most one leaf and two ancestors per tree.
Across800 inserts per cohort, extra leaf splits per tree are bounded by
(31+floor800/32)+(312+floor800/32)=393. Extra branch splits per tree are bounded
by(1+floor56/16)+(19+floor337/16)=44; allow two extra roots per tree. These bounds
use the no-underfill/source insertion invariants, not an observed average.

The retained member/outcome object bound is
`1600*(23577+131289+2*9321+2*7081)+393*(23577+131289)+44*(9321+7081)+2*(9321+7081)`
=**361,888,830 bytes**. Add1600*8192 for accepted private files:
**374,996,030 bytes**. The old64KiB/publication forecast is not a source bound;
replace its104,857,600-byte component with this larger number.

## Conditional whole-history byte/file envelope

Keep original552,141,207 bytes/129,768 files. Preserve the existing64MiB physical
coexistence,16MiB inventories,16MiB generation/deferred bookkeeping and16MiB
temporary/handoff envelopes; final helper/source planning must substantiate them.
Do not treat a planned cleanup as capacity. The revised accepted/packing subtotal
is `774439319-104857600+374996030`=**1,044,577,749 bytes**.

The earlier closed no-op candidate bound is1,784 bytes for ASCII request<=64;
use it conservatively despite this recipe's27-byte IDs. Reserve all2,400 losers:
candidate4,281,600 bytes. Proposed final-source admission caps are aggregate
terminal control files<=4,102 bytes and<=2 files per loser (e.g. a<=4096-byte
receipt plus6-byte fence), plus4MiB NEW terminal/private temporary high-water.
These are **required source-proof envelopes**, not invented terminal-v2 fields.
If the actual schema/files exceed them, obtain a revised bound before execution.

Conditional peak: `1044577749+4281600+2400*4102+4194304`=
**1,062,898,453 bytes**, leaving**10,843,371 bytes** under1GiB. Retain the broader
old194,568-file forecast (accepted source-derived files are fewer), add7,200 loser
candidate/control files and64 extra temporary files: **201,832 files**<400,000.
No candidate/receipt/generation deletion credit is used. The eight live entries,
retained terminal charges and final accounting counters must independently fit
their engine quota rules; logical disk fit does not establish that admission.
This envelope is NOT final GO until every listed source/preflight condition and
terminal/physical envelope is proved. Current1GiB remains;768MiB is not applied.
After all mandatory history work qualifies, only a new actual quiescent census
can justify expiring unused forecast; it is not physical reclamation.

Byte basis is regular-file logical length. Count all private/temporary names.
Native regular-file allocation, directories and device usage are separately
measured/unmeasured as appropriate; do not infer their size from JSON bytes.
The global44GiB ceiling,140GiB free floor and protected closeout reserve remain.

## Output and helper interfaces

Old24MiB F12 forecast omitted terminal output. Proposed lossless gzip caps:
readers10MiB; publishers6.25MiB; transfers4MiB; phase metadata2MiB; lookups4MiB:
26.25MiB, requiring fresh admission and complete guard/RSS/final-output sizing.
Reserve a complete source-bounded response before launch; no dropped fields/rows,
less load or fake-ratio bound. EOF/censors fail qualification.

Driver.call(root,action,timeout,code,env,on_started,**fields): positive RSS after
resume precedes stdin; env separate from INPUT. Keep start/resume/stdin/done clocks
and binary SHA. Expected exit2 recovery validates exact INPUT/env/source/empty
violations. Driver.head capture needs authority proof. lookup_samples.py supplies
50/500; quiescent_oracle.py isolates roots and proves complete unchanged state;
v5.verify keeps known_requests/request_provenance. R1/terminal review precedes use.
Driver924 still plain JSONL; event_log is unreviewed/unintegrated. Review logging
adapter while preserving native ownership/RSS/start semantics. Parent coordinates
source-only new lanes; this document grants no engine/Git action.
