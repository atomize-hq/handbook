# Prospective finite allocation

Frozen before experimental execution, 2026-10-18. Serial expensive lanes only.
Scratch roots are newly owned C:/file-native-incremental-20261018 and siblings;
WSL /home/spenser/file-native-incremental-20261018; native Linux/Mac use distinct
new roots, preserving every prerequisite directory. No blanket cleanup.

| Work | Finite allocation / watchdog |
| --- | --- |
| Development/build/Stage 1 | 900 seconds per invocation; 2 hours combined |
| Differential correctness | seeds 11,29,71,113; 250 batches each at N=128/1024; 900 s per seed/tier |
| Named/fault/lifecycle cases | 180 s per process; 2 hours total; at most 128 boundary cases per lane |
| Matched stage traces | N=1k/10k, scattered 1/8/64; 600 s per operation |
| Integrated document growth | N=1k/10k/100k, short history initial+1/8/64, 2400 s init and 1200 s per publication |
| Real history | fixed N=16, fixed 4 KiB source variants, degree <=2, 16 active keys; A refresh deferred except explicit checkpoints; H=1k/10k/100k; 4 hours total controller cap |
| History truth | independent append-only expected ledger; fresh-process depth checkpoints; no full oracle per append |
| Synthetic stress | separately labeled 100k predecessor index fixture; <=900 s |
| B / packet | N=1k/10k complete B, one bounded 100k B control if forecast fits; real 5000 closure separate; 2400 s per case |
| Contention | medium N=10k, 1/2/4/8 processes; <=4 pure attempts/request, 180 s aggregate/request, 900 s per wave |
| Native affected coverage | same source pin; small A/B, both consumers, affected faults and 1k measurements, <=2 hours/host |
| Reporting/review reserve | 2 hours; no new broad workload after allocation expires |

Maximum new logical file allocation 32 GiB local+WSL combined, host free reserve
140 GiB; WSL free reserve 40 GiB. Native each <=6 GiB with 20 GiB free reserve.
Check allocation/free space before phases and periodically in resident histories.
Controller enforces deadlines, retains exact stdout/stderr bytes before parsing,
and stops its own child processes at caps. Censors/failures remain immutable
observations; a prospective amendment must precede its affected launch.

Engine candidate quota: <=8 reservations, conservative total <=32 GiB. Small
requests reserve a conservative page/payload/materialization allowance before
allocation, initialization reserves from declared corpus size. Every charged
write/copy must fit its allowance before it occurs. Shared object duplicates may
be charged repeatedly. Selected transfer releases candidate charges; abandoned
objects remain charged until coordinated GC/deletion. Accepted dedup precedes
new admission even at full quota. A released OS lock/TTL never abandons work.

Prospective final-lane amendment: correctness-driven source iterations exhausted
the original development estimate; those observations are retained as development,
not final common-pin measurements. After the retention corrections, allow one
final correctness pass (180 s/process, 2 h lane cap), one serial Windows matrix
(4 h), one Windows history lane (4 h including 10k required and bounded 100k),
and one WSL/native lane each (105 min work + 15 min reporting). Do not reset a
running lane deadline or silently replace a censor. Native outer run_lane.py
includes build and subprocess descendants in the cap; per-command ordinary
native cap 180 s, smoke 600 s. WSL reserve is 40 GiB and local+WSL allocation
remains 32 GiB. Unperformed/censored larger controls must be reported explicitly.

Measurements distinguish controller all-attempt time, engine inclusive operation,
selection boundary, A completion and nested phases; overlapping clocks are not
summed. Report total/unique pages, physical/tombstone keys, visited documents/
edges, touched paths, index updates, payload/metadata/control/result bytes,
namespace operations, sync/unsupported-sync counts, actual file bytes and quota.
Sparse observations are not percentiles or causal cross-host rankings.

Prospective final evidence amendment (21:30 UTC, before launch): after the serial
measurement/native lanes, allow <=900 seconds total for final-core 1/2/4/8-process
contention on the already owned N10k A root, with fresh request IDs. Preserve the
original F3 contention trace. This targeted repeat records simultaneous reserved
bytes and a deterministic snapshot of newly created preparation files at the
controlled barrier; its request latency includes that census/archive pause and
cleanup/retry work. It is not a replacement uncontrolled throughput benchmark.
Allow <=900 seconds for the synthetic pessimistic byte-ceiling A/B boundary,
which inflates a single real reservation's allowance to the declared 32 GiB cap,
checks refusal with one reservation, restores exact registry bytes and reclaims.
It does not allocate 32 GiB. Both small verification additions use the reporting
reserve; no original case cap/censor or large-tier deadline is extended.

Prospective matched-depth calibration amendment (before launch): the reused
post-matrix N10k root includes earlier contention/incoming-build publications,
whereas achieved N100k genuine batches have H=2/3/4 including initialization.
Allow <=900 seconds total for fresh final-core A N1k/10k initialization, identical
4 KiB genuine 1/8/64 source imports at H=2/3/4, separate refresh and full audit.
This small calibration uses the reporting reserve and does not rerun 100k or
replace its censored original initialization. It repairs the history-depth match
for the final document-size comparison; preserve both reused-root observations.

Prospective audit correction amendment (recorded before corrected build): allow
one <=900 s owned N128 A/B negative-evidence regression per Windows/Linux/macOS/
WSL, plus locked build/tests and a <=900 s strict audit of the final history root.
The old 7ac reproduction and long-running history remain version-qualified. The
correction only preflights selected projection/source evidence before rendering
the full audit oracle; it changes neither ordinary publication nor disk formats.
Native followups use a new explicit <=1800 s correction cap including build and
export; they do not reopen or reset the original native measurement windows.
The Windows reproducer's exact diagnostic interval is recorded and overlapped
the resident history lane; exclude that interval from quiet timing claims.

Before WSL launch, its four companion entrypoints receive a distinct harness
adapter while their original script bytes are retained as *.capsule.py. The
adapter reads the exact monotonic work deadline from run_lane's original native
adapter, clamps Driver and kills only its own descendant process group at that
deadline or <=890 seconds per companion (10 seconds remain for the existing
900-second host watchdog/reporting). It records expired work as unperformed.
This prevents companion invocations from resetting the 105-minute work window.
Rust and the original 7ac source TAR remain unchanged; source ZIPs record the
actual budget adapter and original-script copies. Guest free>=40GiB and combined
owned guest source/build/runtime<=6GiB checks remain explicit.

Prospective final-source audit check, before launch: after the queued serial
checks, permit one <=900-second strict 7e audit of the existing A100k selected
store. It checks the corrected preflight at the achieved document tier, without
repeating initialization or relabeling its original censor. It uses the existing
two-hour reporting reserve and is unperformed if its full cap cannot fit.

Prospective history proof/GC correction, before continuation: the already loaded
history harness uses a root named store, which cannot pass lifecycle setup's
disposable-component guard. Preserve that refusal. After all queued readers and
audits finish, allow <=900 seconds for a separately labeled continuation: record
SELECTED/config/ledger hashes, move only this quiescent owned store into its own
disposable/store subdirectory, verify unchanged hashes, setup, retire one early
snapshot, collect and check exact early/lost-reply outcomes. No live process is
moved and no original append deadline is extended. Capture the full history proof
before retiring content; private candidates may disappear, while transition proofs
still require historical graph pages. The continuation uses the reporting reserve
and is unperformed if its complete cap does not fit.
The continuation's inner controller is <=860 seconds within the <=900-second
outer cap, reserving logging/termination time. A separate <=180-second fresh
read-only check can verify original outcomes after a completed/censored GC; it
cannot reconstruct an unobserved GC response. Explicit post-retirement proof mode
still authenticates every receipt/result/index/intent and final full state, but
lists the exact header-transition row numbers unavailable due to authenticated
retirement markers. The pre-retirement proof retains all transition checks.

Prospective paired-intent recovery correction (before reproducer/build): allow
one <=900-second A/B old-core reproduction on Windows, then a separate corrected
build/tests and <=900-second paired-preparation/supplement/replay regression on
Windows. Permit one <=1800-second affected followup each on native Linux/macOS
and WSL, including build/tests, the same focused cases and evidence export. The
WSL followup must fit the existing reporting reserve. Existing native windows
and all long-history/document observations retain their original pins/caps. No
running binary is replaced. The correction only changes internal reservation
transfer/abandon identity lookup for a losing different-intent preparation; public
same-ID/different-intent publication must remain refused. No format is changed.
