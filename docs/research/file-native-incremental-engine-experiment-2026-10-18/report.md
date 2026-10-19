# Incremental file-native engine experiment

Status: completed bounded experiment, with original failures/censors and
unperformed dependent cases retained. This is not a production-readiness claim.

This experiment implements a bounded compact COW core, declared constraint
indexes, predecessor-only history indexes, durable incremental A refresh,
complete independent B generations, exact packets and a small shared library.
The original baseline is `379af4eda9b95ee82be9d3afcfdcdca2d6d132f9`.
The final accounting-corrected core is `5a65a2155f9eb297d2cbb1b7c900fa749629605c`.
The original document matrix uses F3; continued publication/history measurements
use `7ac70f9`. Fresh matched-depth calibration and final contention use `7e00d22`.
That correction preflights selected projection/source evidence before audit rendering;
`5a65a21` repairs reservation recovery for two eligible different-intent requests
sharing one ID. These corrected paths have focused regressions, rather than a
replacement full document/history matrix on the latest pin.
Both browsing contracts remain open. No production architecture, format default,
migration, public API or Dolt adoption is selected.

## What the evidence can decide

| Area | Mechanism and evidence | Experimental disposition / limit |
| --- | --- | --- |
| Validation | Final-batch overlay; D/E/R/P/L indexes, exact count/cost, explicit full fallback; 2,000 seeded differential batches and complete audits | Affected-work approach implemented and testable. A checked base is required; unrelated corruption is found by explicit audit, not every small commit. |
| Metadata | C1 bounded tagged pages replace JSON numeric byte arrays in an otherwise unchanged control | Logical maps and observed structural packing agree in this control; encoding reduces bytes. Byte bounds can change packing in other workloads. No production encoding approval follows. |
| History | Current head directly represented; predecessor-only membership/outcome maps; original result bodies separate; verified checkpoint successor | Actual durable-depth results appear below. No chain walk on normal open/resolve; audit/checkpoint/GC remain explicitly full operations. |
| Rendering | Per-document navigation and indexed change ranges; persisted target/prior pending plans and stable divergence backups | Small unrelated A edits can update affected paths. A may lag or mix until completion. Independent 1k/10k checks passed; A100k renderer check was censored. B still copies/verifies complete independent generations. |
| Lifecycle | Pessimistic reservations, selected ownership transfer, reader/pin/candidate/pending roots, typed retirement | Two real GC defects required corrections. Focused corrected cases passed; long-history GC was unperformed after the full-proof export censor. Final eligible preparations include deterministic replay dependencies. This complexity needs continued scrutiny before adoption. |
| Library | Store/Snapshot/Transaction/private Prepared; typed outcomes; two separately linked concrete consumers | Demonstrates a narrow shared core and explicit projection/packet capabilities. No general plugin ABI or public compatibility promise. |

[Design and scope](design-and-scope.md), [private protocol](protocol.md) and
[finite allocation](allocation.md) define the contracts. The
[accepted brief](inputs/brief.md) and [reconciliation](inputs/reconciliation.md)
are byte-preserved inputs. The structured format standard remains proposed;
existing owner/versioned contracts and Contracts JSON remain controlling.

## Document growth and required work

The serial Windows F3 matrix measured 1k/10k A and B, separate validation traces,
scattered imports, one outgoing-edge edit in a 5,000-edge adjacency, a target
rename affecting 5,000 incoming sources, selected/combined packets and 1/2/4/8
process contention. Its 100k A initialization hit the 2,400-second cap after
selecting authority, with the readable view incomplete. That original event is
censored even if a later recovery finishes.

| F3 Windows observation | Controller elapsed | Work counters |
| --- | ---: | --- |
| A initialization, N=1k / 10k | 15.75 s / 201.64 s | Full initialization, exact source and readable work |
| B initialization, N=1k / 10k | 12.94 s / 172.76 s | Complete independent generations |
| B one-label successor, N=10k | 104.2 s | Complete generation copy and preselection destination verification |
| One low-degree import, N=1k / 10k | 57.23 ms / 61.02 ms | 2 document visits each; 33 / 44 page reads; 230,035 / 213,585 metadata bytes read |
| One edge in the 5,000-constituent adjacency case | 1.466 s | 5,005 document visits, 5,002 edge visits, 20,217 page reads, 1 touched path |
| Rename target with 5,000 incoming sources | 46.807 s | 15,000 document visits, 10,000 edge visits, 180,754 page reads, 5,002 touched paths |
| Selected 5,000-source closure | 25.034 s | Explicit packet workload, separate from commit |
| Combined publication and closure | 22.858 s | Explicit combined workload; timing difference is not evidence of reuse speedup |

These are sparse observations, not percentiles. Counts include repeated reads,
semantic checking and projection dependencies. The outgoing edit keeps adjacency
entries independent but still reads the affected source's navigation; it is not
constant work in degree. The incoming rename legitimately touches thousands of
navigation sections. A metadata/page reduction does not remove required packet,
namespace, sync or complete B work.
The outgoing case has 5,000 constituent edges plus two fixture relationships;
the 5,002 actual edges control its work count.

The requested 5,000-source closure actually contains 5,002 authored sources,
5,005 manifest files and 5,006 ZIP entries including the manifest. The exact ZIP
and manifest hashes are retained; selected and combined assembly produced equal
manifests. These actual counts control interpretation of the shorthand case name.

The post-matrix B100k control was censored before selection and retains its
preparing reservation of 30,067,108,864 conservative bytes. Its registry's
`used=0` is not a physical-storage measurement; the full allowance remains charged.
It has not been implicitly abandoned to free quota. Actual file-length census,
active reserved bytes and permanent publication ownership are separate quantities.
The independently bound B100k process sample records 360.547 seconds of CPU and
384,688,128 peak working-set bytes; the original controller censor remains
2,418.539 seconds. [Resource binding](evidence/windows-B100k-resource-binding.json)
states the reservation/PID/lifetime association used for that otherwise unselected
process.

Original requested imports 1/8/64 contained 1/7/56 genuinely different sources;
overlapping IDs reused exact bytes. Corrected `genuine-*` batches use separate
4 KiB variants. [Independent selected-chain/source verification](verify_genuine.py)
checks old versus final digests and exact captured bytes, rather than treating a
`changed` list as proof. The separately measured A100k resume completed in 501.440 seconds; this does
not change the original censored initialization. Post-matrix measurements and
original censors remain separate.

The corrected 4 KiB scattered-import observations use publication depths H=2/3/4
including initialization. Fresh N1k/10k calibration uses 7e, and achieved N100k
uses 7ac. Publication defers A refresh; each following refresh is a separate
invocation and touches exactly the listed number of changed source paths:

| N | Changed sources | Publication controller time | Separate A refresh | Publication document visits / object reads | Publication metadata read / written |
| --- | ---: | ---: | ---: | ---: | ---: |
| 1k | 1 | 58.744 ms | 16.656 ms | 2 / 33 | 230,035 / 47,373 bytes |
| 1k | 8 | 133.894 ms | 38.706 ms | 16 / 184 | 1,431,318 / 337,876 bytes |
| 1k | 64 | 598.824 ms | 249.450 ms | 129 / 1,051 | 8,802,444 / 664,672 bytes |
| 10k | 1 | 72.891 ms | 20.735 ms | 2 / 44 | 213,585 / 49,434 bytes |
| 10k | 8 | 171.701 ms | 38.949 ms | 16 / 242 | 1,340,043 / 364,164 bytes |
| 10k | 64 | 771.760 ms | 258.215 ms | 128 / 1,520 | 9,901,095 / 2,623,684 bytes |
| 100k | 1 | 66.121 ms | 15.478 ms | 2 / 55 | 212,502 / 52,435 bytes |
| 100k | 8 | 174.754 ms | 40.734 ms | 16 / 300 | 1,396,700 / 391,103 bytes |
| 100k | 64 | 919.755 ms | 286.008 ms | 128 / 1,990 | 10,606,617 / 2,972,437 bytes |

These calls do not load all 100k document headers. Full A100k audit separately
took 89.900 seconds on `7ac70f9`. The later corrected `7e00d22` strict audit passed
in 363.292 seconds per controller call (363.524-second enclosing wrapper), with
1,068,140 object-read calls, 4,686,559,902 metadata bytes and 2,567,442,542 payload
bytes read. These repeated verification reads are not unique stored bytes or
physical I/O. [Exact derived counters](evidence/strict-large-audit-derived.json)
remain separate from the independent projection censor below.
Fresh N1k/10k roots avoid the reused medium root's older contention/incoming-build
history. Their audits and independent selected-chain/source checks passed.
The 7e change affects audit preflight; these mixed-pin observations still do not
establish a perfectly isolated causal ablation or latency distribution.

## Controlled contention and reservations

The corrected 7e repeat accepted all 15 requests across 1/2/4/8-worker waves.
Each wave mixes one edit in the 5,002-edge adjacency with scattered label edits.
It deliberately waits at prepared-candidate barriers, inventories new files and
archives them before releasing selection. The complete trace took 347.781 seconds,
including collection/cleanup between waves; request response times below cover
their preparation, barrier/census waits and any cleanup/retry before acceptance.

| Workers | Accepted / attempts | Request response range | Observed reserved bytes | New file bytes at barrier | Existing-file growth |
| --- | ---: | ---: | ---: | ---: | ---: |
| 1 | 1 / 1 | 4.512 s | 67,109,568 | 403,801 | 424 |
| 2 | 2 / 3 | 4.555–18.210 s | 134,219,328 | 434,525 | 850 |
| 4 | 4 / 7 | 4.553–18.885 s | 268,438,848 | 509,490 | 1,700 |
| 8 | 8 / 15 | 4.714–20.575 s | 536,877,888 | 663,436 | 3,400 |

The census/archive pauses were 2.625/2.703/2.641/2.750 seconds. These controlled
latencies are not ordinary throughput trials. All observed whole-request times
fit 180 seconds; the implementation caps individual calls at 180 seconds and
four attempts, rather than implementing a separate aggregate-request watchdog.
The observed pending state is not continuous physical peak monitoring.
[Derived request/lock/retry data](evidence/contention-derived-final.json) retain
per-request clocks, refusals and failed-attempt logical writes. Those writes are
not unique discarded physical bytes. Exact pending ZIPs and quota receipts are
retained in `evidence/contention-pending/` and independently checked by
`derive_contention.py`.

## Encoding, validation and timing interpretation

The isolated encoding control copies the baseline full adapter/validator/renderer
and switches only JSON/C1 internal page encoding. Capacity stays leaf64/fanout32;
bounded byte splitting can change geometry. Independent enumeration compares
logical maps, hashes, height, pages, physical/tombstone keys and occupancy.

The development matched control's single-label metadata reads fell from
5,128,808 to 1,552,195 bytes at 1k and from 52,013,643 to 16,555,293 at 10k;
writes fell from 81,632 to 25,046 and from 82,570 to 25,723. The serial common-control
repeat reproduced those exact byte counters. Its sparse JSON/C1 controller pairs
were 123.155/89.429 ms at 1k and 710.474/491.741 ms at 10k. Both representations
use the same control executable with an encoding switch. Complete control-source
archives were not captured at its original compilation; unchanged committed
control sources and executable SHA associate the repeat, without proving exact
original compiler-input bytes. The final logical-map/geometry check is separate.
Caches remain uncontrolled; these pairs do not establish latency distributions.

Full-reference load plus semantic validation and complete incremental preparation
have different scopes. The F3 trace records both explicitly:

| N / changed labels | Full load + validation | Complete incremental preparation | Full / incremental document visits | Full / incremental page reads |
| --- | ---: | ---: | ---: | ---: |
| 1k / 1 | 3.048 ms | 28.113 ms | 1000 / 2 | 21 / 37 |
| 1k / 8 | 3.054 ms | 48.492 ms | 1000 / 16 | 21 / 203 |
| 1k / 64 | 2.981 ms | 142.937 ms | 1000 / 129 | 21 / 1342 |
| 10k / 1 | 30.597 ms | 26.994 ms | 10000 / 2 | 249 / 46 |
| 10k / 8 | 31.298 ms | 56.756 ms | 10000 / 16 | 249 / 248 |
| 10k / 64 | 29.899 ms | 236.543 ms | 10000 / 128 | 249 / 1699 |

Preparation includes persistence, projection work, reservation and sealing. Small
full scans can be cheaper than these complete preparations, and scattered larger
batches can read more pages despite visiting fewer documents. Their ratio is not
an end-to-end speedup or isolated validation ablation. Nested counters and phases explain work; the experiment does
not establish a perfectly isolated causal attribution for every integrated layer.
Cross-baseline end-to-end ratios are also limited by the new per-document
navigation layout and its different file count.

`controller_ms` includes process startup, capture and any watcher join/scan time.
Engine inclusive clocks overlap and must not be added. Lock wait/hold are recorded
separately. Resident history per-row time covers publication/selected parsing and
metric finish, excluding its independent ledger serialization/sync; the enclosing
history controller covers that ledger work. Watchdog filesystem scans can contend
with engine I/O. Small late diagnostics, corrected builds and harness staging overlapped part
of the history lane; their recorded intervals prevent calling the whole resident
run quiet. Outer history API counters cover setup only; per-publication
ledger counters provide append work. Fresh process does not imply device-cold caches. Early runs have no separately sampled CPU time. A bounded read-only Windows
observer started after A100k recovery captures complete-process CPU and peak
working set for observed later processes; short processes may be missed. No
retrospective CPU values are inferred. Missing/censored engine clocks are unavailable, not zero.
The final identity-checked join binds 97 of 7,396 captured Windows calls; unmatched
calls have unavailable sampled CPU/peak values. Both observers stopped with no
tracked live processes. [Final process bindings](evidence/windows-resource-bindings-final.json)
retain exact observer-line bytes, executable hashes and checked lifetimes. These
sampled complete-process clocks are distinct from engine-operation clocks.

The API names `page_reads` and `unique_pages` count immutable-object read calls
and distinct read IDs, including indirect values and verification of existing
objects. Independent enumeration counts structural tree pages separately. Byte,
sync and namespace counters instrument logical API work, rather than every kernel
metadata operation or physical I/O. Full audit may persist unselected reference
objects after checking selected evidence; whole-store censuses include them.

## Real durable history

The bounded 100,000-selection tier completed without append censoring and passed
the captured harness's independent verification: 100,001 publications including
initialization, 100,000 dense ledger receipts, exact predecessor membership/outcome
maps and original results, header transitions and final graph/index/source recipe.
The mandatory 10,000 depth passed the same checks earlier. No structural fixture
or bulk chain is substituted for live commits. The separate portable-proof export
was censored; fresh restoration and long-history collection were not performed.

The enclosing history controller ran 11,411.516 seconds including independent
checks, checkpoint/refresh and the later successor test, then exited1 at the
collection setup refusal described below. Its three real append phases were:

| Real selections in phase | Enclosing API controller | Resident loop | Observed complete-process CPU | Observed peak working-set bytes |
| --- | ---: | ---: | ---: | ---: |
| First 1,000 | 80.480 s | 38.638 s | 18.813 s | 6,889,472 |
| Next 9,000 | 631.902 s | 631.895 s | 203.063 s | 7,348,224 |
| Next 90,000 | 9,721.455 s | 9,710.126 s | 2,302.281 s | 8,687,616 |

Resident time includes durable selection and expected-ledger work. Controller
time additionally includes capture/watchdog joins and filesystem scans; the first
phase's large gap remains in the evidence. These clocks overlap and are not
summed. [Raw-line-bound phase extraction](evidence/history-phase-times-final.json)
distinguishes them from the per-row clocks below.

The fixed corpus has 16 documents, 148 KiB of authored source and degree at most2.
Content/restore operations
touch only d000000, alternating two exact 4 KiB variants; untouched sources retain
their fixed fixture sizes, including 16 KiB and 64 KiB files. Metadata changes use
16 mutable label slots with bounded values, rather than exactly 16 physical index
keys. Five categories alternate:
source variant, actual label change, already-zero cost assignment, restoration of
original bytes, and empty maintenance publication. The raw label `equal-content`
means that already-zero cost assignment; it does not measure source import/dedup.
No B generation or large packet is materialized per append; A refresh is deferred
until explicit checkpoint work. A resident harness performs every real selection
and synchronization and writes a dense, synced expected ledger afterward.

At each achieved depth, fresh processes resolve early/middle/recent/missing IDs,
read retained/current content, retry original intent and test cancellation.
Different-intent reuse is checked in the separate lost-reply successor test.
Independent Python decoding authenticates the complete selected
chain, exact predecessor membership/outcomes and original result hashes, and
reconstructs the **final** complete graph/index/source recipe from checked initial
state. Each source/restore/label mutation also receives an independent old/new header
check; equal-state and maintenance rows must preserve graph/projection roots.
This is a touched-header transition proof plus final complete oracle, rather than
full replay of every intermediate graph revision.

Fresh-process positive resolution at the independently verified depths:

| Durable depth | Early / middle / recent controller time | Early / middle / recent object-read calls | Early / middle / recent metadata bytes read |
| --- | ---: | ---: | ---: |
| 1,000 | 7.112 / 6.237 / 5.988 ms | 8 / 8 / 2 | 45,693 / 54,841 / 7,516 |
| 10,000 | 9.746 / 6.519 / 5.814 ms | 10 / 10 / 2 | 41,927 / 39,924 / 3,644 |
| 100,000 | 77.165 / 8.119 / 5.943 ms | 12 / 12 / 2 | 38,943 / 47,956 / 2,158 |

These sparse point observations show bounded index-path work rather than a read
of every predecessor. The current publication is represented directly; older
requests traverse the selected predecessor index. They do not establish a
latency distribution or device-cold performance.
The slower early 100k lookup is retained; bounded read counts do not guarantee
flat elapsed time.

The dense ledger also separates homogeneous append groups. These are medians
of per-row engine time, excluding expected-ledger serialization/sync; enclosing
controller time includes that work. Rows within a group are successive dependent
publications, not independent trials. The observer/watchdog overlap described
above applies, so the growth in elapsed time is not an isolated causal ablation.

| Append category | H1–1k median ms / object reads | H1k–10k median ms / object reads | H10k–100k median ms / object reads |
| --- | ---: | ---: | ---: |
| Source variant | 38.387 / 68 | 47.566 / 75 | 113.933 / 82 |
| Metadata | 36.389 / 62 | 43.004 / 68 | 99.357 / 74 |
| Already-zero cost (`equal-content`) | 34.828 / 56 | 42.470 / 62 | 99.056 / 68 |
| Restore exact source | 39.086 / 68 | 47.284 / 75 | 113.759 / 82 |
| Maintenance | 34.103 / 40 | 41.318 / 46 | 98.311 / 52 |

Each category has 200, 1,800 and 18,000 rows in these intervals, respectively.
Exact medians, maxima and metadata read/write bytes are in
[the measured tables](evidence/measured-tables-final.json).

Checkpoint rebuild, A refresh, lost reply beneath 1,000 actual successors and
bounded long-history collection are separate observations. Collection is allowed
to fail or censor; later successful resolution alone cannot prove reclamation.
The report uses its actual disposition and removed counts. Selected-tree geometry
is not a census of all retained historical files. The portable history export is
a proof capsule, not a complete restartable store backup.

The checkpoint completed in 11.542 seconds and separate A refresh in 2.719 seconds.
The harness then verified 100,002 publications, injected reply exit86, selected
1,000 actual successors and verified the original outcome beneath them. Its final
independent check covered 101,003 publications and the original 100,000 receipts.
The controller subsequently exited1 on `refused:disposable-root`: its captured
root was named `store`, which cannot arm collection. This original failure is
preserved. The separate full proof exporter reached its 900-second cap after
900.445 seconds, leaving a partial `part-000.jsonl.gz` without a completed
manifest. That file remains unchanged; it is not a usable proof capsule. Fresh
restoration, relocation, retirement, long-history collection and dependent
post-GC checks were not performed because their prerequisite export failed.
The history store remains at its original owned root, with no retirement/deletion
from that continuation. The saved reproduction script uses consistent
`disposable/store` paths for a future separately authorized run.

A prospectively bounded ledger-only fallback completed in 6.804 seconds. It
preserves all 246,560,316 raw bytes and 100,000 dense receipts, SHA-256
`6ab8bef045580f7642b0f404cf58ab7cb8211badfeb68d258bc62e984e02af18`,
through 15 losslessly verified gzip chunks. This is raw receipt transport, not a
replacement selected-history proof or full store backup. The original harness's
completed independent 101,003-publication verification remains a separate
observation. See [ledger manifest](evidence/history-ledger-transport-final/manifest.json),
[bounded controller](evidence/ledger-transport-controller.json) and
[prospective fallback bound](evidence/ledger-transport-amendment.md).

The pre-collection file census found 1,413,491 files and 5,464,941,402 logical
bytes in this history root. It includes historical pages, preparations, the
246,560,316-byte dense expected ledger and readable files, rather than only live
selected-tree geometry. Active reserved bytes were zero; 101,003 permanent owner
records sum to 6,788,903,037,184 conservative bytes. Cumulative ownership allowances
are not physical allocation. The unselected censored B100k root separately holds
2,660,013,821 logical file bytes and its 30,067,108,864-byte active reservation.
Windows allocated-file bytes are unavailable, not zero. See
[selected-store census](evidence/selected-store-census-final.json) and
[unselected B census](evidence/unselected-B-census-final.json).

The aggregate independent navigation/source/readable-file verifier passed the
matrix A/B roots at 1k and 10k, then was censored at 900.406 seconds while visiting
A100k. Its partial output does not verify A100k or subsequent roots. A separately
recorded 600-second continuation checked only the two fresh calibration roots:
both passed, with current readable views and no missing, edited or extra files,
in 7.007 seconds total. The original censor remains. These checks independently
render each document's navigation; they do not independently specify the static
collection prose or packet recipe. Selected digests and separate packet assertions
cover those portions. The planned post-GC current-history projection check was
unperformed with its dependent collection continuation.

Independent final selected-root enumeration completed in 5.162 seconds. Selected
structural geometry is distinct from the API's repeated object-read counters and
from all retained files in the census:

| Selected tree | Structural pages | Height | Physical keys | Tombstones |
| --- | ---: | ---: | ---: | ---: |
| Fresh N1k graph/constraints, H4 | 33 | 1 | 2,008 | 0 |
| Fresh N10k graph/constraints, H4 | 485 | 2 | 30,008 | 0 |
| A N100k graph/constraints, H4 | 3,390 | 3 | 210,008 | 0 |
| History head graph/constraints, N16 / H101,003 | 3 | 1 | 72 | 17 |
| History head predecessor membership | 1,641 | 3 | 101,002 | 0 |
| History head predecessor outcomes | 2,413 | 3 | 101,002 | 0 |
| History head change journal | 1,332 | 3 | 40,000 | 0 |

The current head is represented directly; predecessor trees exclude it. Complete
per-role geometry, occupancy and selected identities are in
[root geometry](evidence/current-root-geometry-final.json).

## Correctness, failures and native coverage

Final Windows `7ac70f9` passed locked workspace tests, A/B × project/research
semantic cases, 23 publication boundaries, causal supplement, adversarial damage,
16 checkpoint-specific boundaries, waiting cancellation/holder exit, namespace
and replay-retention regressions, 2,000 seeded differential batches and PowerShell
selected-once copies. Native Linux passed the targeted corrected 610-response
capsule. Native Mac also passed the corrected 610-response capsule with zero censors,
within its unchanged original deadline. WSL's 7ac common capsule and four
companions completed 771 responses (647 exit0, 71 expected exit2, 53 injected
exit86), with zero censors. Locked tests/build, both separately linked consumers,
1k A/B smoke and Bash selected-once copies passed. Its later audit/accounting
followups and final raw export completed separately. WSL passed the 42-response
strict audit correction and 181-response paired-intent/supplement/replay correction,
including locked builds/tests, with zero censors. Its source association check
verified 27 Rust/Cargo files in each of 15 session archives against the named
7ac/7e/5a pins and rehashed nine retained guest executables.

Important preserved failures and corrections:

1. A fixed packet allowance refused the real 5,000-source closure; admission now
   derives its bounded allowance from required work. Windows extended-path I/O
   corrected a deep divergence-backup path failure.
2. F3 collection classified any ancestor named `objects` as an object store and
   could delete selected originals. Byte roles similarly misclassified ancestors
   named `results`. `a1108c6` makes both classifications relative to the actual
   store root. F3 native primary role splits under `results` remain invalid;
   totals, object-metadata counters and outcomes retain their stated scope.
3. An eligible large candidate could lose intermediate replay objects to GC and
   later fail read-only reopening. `7ac70f9` private Candidate v2 records exact
   intermediate object/source/derived dependencies, retains them while eligible,
   and independently recomputes/compares the set at selection. A/B tests reject
   forged resealed lists, reopen after GC and reclaim abandoned intermediate
   sources. Publication/store versions remain1; no old evidence is migrated.
4. The first PowerShell copy runner selected Windows PowerShell5.1 in an inherited
   module environment without `Get-FileHash`; that failure is retained. The
   corrected helper chooses installed `pwsh`, records argv and passes both copies.
5. The pilot label recipe became equal-state after each ID's first visit. The final
   `i/80` cadence toggles labels on every revisit; the pilot is excluded from the
   sustained metadata-mutation result.
6. On `7ac70f9`, full audit rendered before traversing selected projection children
   and checking derived sources. Missing bulk-layout children/navigation files
   could silently regenerate; selected packet sources could be missing/corrupt
   while separate capsule copies passed. The retained N128 A/B before-probe
   reproduces both. `7e00d22` verifies every selected entry/source before rendering.
   Windows, native Linux and Mac strict suites each passed 18 damaged-evidence
   refusals without repair and 18 restored audits (42 responses, zero censors).
7. On `7e00d22`, two eligible preparations with one request ID and different
   intents could poison unrelated recovery after one selected, and prevent fencing
   the loser. The Windows A/B before-probe reproduced both failures. `5a65a21`
   looks up internal acceptance by ID, transfers only the exact accepted attempt
   with matching intent/ownership, and permits fencing another attempt. Public
   intent mismatch remains refused. Windows passed 32 paired-intent responses,
   109 causal supplement responses, 40 replay/GC responses and locked tests.
   Native Linux, Mac and WSL each passed the same 181-response affected capsule
   and both locked unit tests, with zero censors.
   No format changed.

Fault assertions distinguish forbidden preselection, visible/incomplete uncertainty
and completed-boundary acceptance surviving successors. Injected exit86 is process
interruption, not SIGKILL/reboot/power loss. Required missing/corrupt evidence is an
integrity failure; the late audit correction above is necessary to satisfy that
contract. Permanent accepted results are independent of content retirement.

Native Linux's post-work census found 2,238,350,270 logical bytes across owned
incremental paths, preserved evidence transports and conservative temporary test
fixtures; its recorded free space exceeds the 20 GiB reserve. No process matched
the snapshot's stated engine/Python filter; completed controller records cover
their own children. The filter does not detect every possible supervisor.
Mac's accounting snapshot found 2,506,657,276 logical bytes,
including its isolated Git database and six temporary test fixtures, with
32,547,360,768 free bytes. Both fit their 6 GiB allocation and 20 GiB reserve.
These are host-local snapshots at their recorded times, not a measurement of
only live selected state. The Mac snapshot precedes its own small receipt/commit.
Linux allocated-file totals sum blocks per pathname and can repeat hardlinks;
they do not measure unique physical/device usage.

The final Windows+WSL census completed in 136.690 seconds. Its conservative sum is
18,186,887,151 logical bytes against the 32 GiB limit: 16,826,800,029 local bytes
plus 1,360,087,122 guest bytes. It includes retained transports, sibling builds,
the entire managed worktree, the entire shared Git database and conservative
temporary bounds fixtures. Host free space was 228,535,316,480 bytes and guest
free space 517,021,782,016 bytes, exceeding 140 GiB / 40 GiB reserves; the guest
total fits its 6 GiB allocation. Neither stated process filter matched a live
process; completed controllers separately cover their known children. These
filters are not universal supervisor detection. The snapshot precedes its own
small receipts and final report/inventory/Git writes, and per-path lengths can
repeat shared data. See [final resource snapshot](evidence/final-resources.json)
and [bounded controller](evidence/final-resource-controller/completed.json).

## Source and durability qualifications

| Source | Meaning |
| --- | --- |
| F3 `f3e942f6a2e84ff33fd6ea860caed49ad650f86f` | Initial integrated matrix/native measurements, with retained limitations above |
| `f2c45c16886ae3c1c89d1084e43b05133383c087` | Companion deep checkpoint/copy helpers with F3 executable |
| `a1108c634cb8362aca50e1a6228766802b96f087` | Root-relative collection and byte-role correction |
| `a3baf52f36915cd87dc4a876893e6819b84131c0` | Genuine history cadence and independent final recipe/index oracle |
| `7ac70f93cce14ecd3d935852801f034c122000d8` | Final replay-dependency lifetime correction and portable PowerShell launcher |
| `7e00d22b470c1eed476127d2f585789036e79cf3` | Selected-evidence preflight before full audit rendering; ordinary publication unchanged |
| `5a65a2155f9eb297d2cbb1b7c900fa749629605c` | Exact-attempt accounting recovery/abandonment for paired different-intent preparations |

Windows F3 was compiled before source newline normalization. The exact compiled
Rust bytes are retained in a session ZIP; the source-binding receipt proves that
the pin differs only by CRLF/LF. Do not call that byte-exact Git-source compilation.
The captured source tree for the main Windows measurement build has 21 Rust
files matching `7ac70f9` byte for byte, as checked in
`evidence/windows-final-source-binding.json`. That tree also contains the separate
control crate; the engine build does not compile every captured file. The later
accounting build's captured Rust/Cargo inputs match `5a65a21`, separately recorded in
`evidence/windows-accounting-source-binding.json`; its focused reruns do not
replace the earlier matrix/history source pin.
Raw events bind recorded executable SHA-256; retained session archives provide
source-file hashes where captured. Some early development sessions retain only
executable/harness hashes, without a complete original compiler-input archive.
Session ZIPs capture working source at harness creation; an older executable can
run beside newer working Rust. The separate build/source receipts and session
association table distinguish those snapshots from associated compiler sources.
The [final association table](evidence/windows-session-source-associations-final.json)
contains 56 sessions and four independent compiler-source references. Six working
snapshots differ from the associated checked engine inputs: one F3 session and
five later 7e sessions captured beside the 5a lifetime correction. Those mismatches
remain explicit; the separate canonical compilation receipts supply the source
association.
Report/helper-only
commits do not silently change the core version attached to earlier observations.

Windows/NTFS uses MoveFileExW write-through; directory sync is explicitly
unsupported. WSL runs/builds on guest ext4, with Windows/VHDX durability limits.
Native Linux/ext4 and Mac/APFS exercise native no-replace install, directory/file
sync and process faults. Mac APFS is case/normalization insensitive; Bash3.2 is used.
No cross-machine OS ranking, device-cold cache, physical write-amplification,
hostile namespace/root substitution, kernel crash or power-loss qualification
follows. Mac peak-memory helper is unavailable, not zero.

All work stays in the standalone research directory and new owned runtime roots.
The Handbook workspace dependency graph and owners are unchanged. GitNexus's stale,
degraded index has no authoritative coverage of these symbols: impact is UNKNOWN.
Direct scoped diff, source inspection, independent review and actual checks supply
the recorded evidence; no zero-risk inference is made.

The import harness accepts bounded captured UTF-8 fixture bytes and preserves
them exactly. Final 4 KiB variants retain fixture Markdown/YAML/JSON syntax, but
this is not a production Handbook schema parser or owner-validation integration.
Domain kind adapters and their declared rule versions define this experiment's
semantic scope; opaque/global rules use the explicit full-validation route.

## Evidence and reproduction

The [evidence guide](evidence/README.md), [verification/reproduction](verification.md),
[independent review](review.md) and [final controller dispositions](evidence/final-dispositions.json)
provide entry points. The [final inventory](evidence/inventory-final.json)
authenticates standalone source/document/evidence membership, lengths and hashes;
it excludes itself, caches, build targets and Git metadata. Historical progress
inventories remain snapshots of their own earlier bytes.
Local Git attributes preserve exact source/evidence bytes across checkout newline
settings. This is an evidence-materialization rule for the standalone directory,
not a new engine format or production contract.

| Retained collection | Scope |
| --- | --- |
| [Windows raw](evidence/windows-raw-final/manifest.json) | 364 files / 7,396 responses across development and final work, including original errors and two engine-call censors; 46 source ZIPs and 10 explicitly missing early compiler-input captures |
| [WSL main](evidence/native-wsl-main/manifest.json), [audit](evidence/native-wsl-audit/manifest.json), [accounting](evidence/native-wsl-accounting/manifest.json) | 771 / 42 / 181 responses on their distinct pins, zero censors; exact transport and source/native-binary associations checked |
| [Native Linux raw](evidence/native-linux/manifest.json), [corrected replay](evidence/native-linux-replay/manifest.json), [audit](evidence/native-linux-audit/manifest.json), [accounting](evidence/native-linux-accounting/manifest.json) | Historical and affected capsules remain separate, with old failures and intermediate corrections retained |
| [Native Mac](native-macos/README.md), [latest portable verification](evidence/mac-transport-accounting-verification.json) | Eight lanes, 23 session ZIPs and 8 source TARs checked; no reproducible-build claim |
| [Dense ledger transport](evidence/history-ledger-transport-final/manifest.json) | All 100,000 receipts, exact byte round trip; not the censored full proof |

[Generic transport verification](evidence/transport-verification-final.json) passed
all 13 lossless manifests; that validates bytes/types/associations, not a claim that
every development case passed. Outer projection/proof censors are retained in
their own controllers. Exact closing outputs written after the main capture are
copied under `evidence/closeout-controller-raw/`. The incomplete full-proof archive
is preserved under `evidence/history-proof-final/` without a completed manifest.

Always use fresh disposable roots and the recorded source pins; preserved runs
are evidence, not rerun destinations. No further large workload, Pro consultation,
production migration or successor task is launched by this closeout. Raw refusals,
process exits and censors remain unchanged. A recovered result never relabels the
original interrupted observation.
