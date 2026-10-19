# Incremental file-native engine experiment

Status: final integrated observations are still running. This draft must not be
read as a completion or production-readiness claim.

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
| Rendering | Per-document navigation and indexed change ranges; persisted target/prior pending plans and stable divergence backups | Small unrelated A edits can update affected paths. A may lag or mix until completion; full view audit remains separate. B still copies/verifies complete independent generations. |
| Lifecycle | Pessimistic reservations, selected ownership transfer, reader/pin/candidate/pending roots, typed retirement | Two real GC defects required corrections and are retained as failures. Final eligible preparations include every deterministic replay dependency. This complexity needs continued scrutiny before adoption. |
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
took 89.900 seconds on `7ac70f9`; its corrected preflight audit is a later check.
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

The API names `page_reads` and `unique_pages` count immutable-object read calls
and distinct read IDs, including indirect values and verification of existing
objects. Independent enumeration counts structural tree pages separately. Byte,
sync and namespace counters instrument logical API work, rather than every kernel
metadata operation or physical I/O. Full audit may persist unselected reference
objects after checking selected evidence; whole-store censuses include them.

## Real durable history

The bounded 100,000-selection tier completed without append censoring and passed
the running harness's independent verification: 100,001 publications including
initialization, 100,000 dense ledger receipts, exact predecessor membership/outcome
maps and original results, header transitions and final graph/index/source recipe.
The mandatory 10,000 depth passed the same checks earlier. No structural fixture
or bulk chain is substituted for live commits. Final portable-proof export and
fresh restoration remain separate pending observations.

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
preserved. The separately bounded, quiescent relocation/GC continuation and final
strict proof remain queued; the saved reproduction script uses consistent
`disposable/store` paths.

## Correctness, failures and native coverage

Final Windows `7ac70f9` passed locked workspace tests, A/B × project/research
semantic cases, 23 publication boundaries, causal supplement, adversarial damage,
16 checkpoint-specific boundaries, waiting cancellation/holder exit, namespace
and replay-retention regressions, 2,000 seeded differential batches and PowerShell
selected-once copies. Native Linux passed the targeted corrected 610-response
capsule. Native Mac also passed the corrected 610-response capsule with zero censors,
within its unchanged original deadline. WSL final observations are still queued.

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
   Native Linux and Mac each passed the same 181-response affected capsule and
   both locked unit tests, with zero censors; the focused WSL followup is queued.
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

Final evidence index and reproduction commands are being assembled. Always use
fresh disposable roots; preserved runs are evidence, not rerun destinations.
Raw expected refusals, process exits and censors remain unchanged. A recovered
result never relabels the original interrupted observation.
