# Bounded transaction follow-up comparison

This experiment implements the same application transaction interface over both
existing adapters. It is a non-production follow-up to `c7f54e63`, with separate
source versions and native evidence. It does not select a backend, format,
migration, production capacity or dependency. The structured data format standard
remains proposed and existing owner/versioned contracts remain unchanged.

The bounded result is a working buffered interface and measured reduction in
intermediate writes and coordinated commit work. It leaves chunk's eager reopen
and per-process decoded memory costs intact. Whole-root conflicts still require
revalidation, and retained history still grows without reclamation. These results
support review of this experiment, without establishing a production backend choice.

## What changed and what was established

`Database::begin` retains an exact immutable root and native coordination identity.
Transactions provide snapshot point reads, read-your-writes, buffered `put`,
`put_if`, commit, cancel and drop. They reject closed/read-only misuse. Commit
checks the native owner/lock and exact expected root under the process lock,
rechecks recorded dependencies, persists a private proposal and publishes one
selected root. Whole-root mismatch returns an explicit conflict; the caller must
re-begin and revalidate. The generic API never retries an arbitrary callback.
The CLI explicitly rebuilds its pure semantic command where retry is enabled.
See [transaction-contract.md](transaction-contract.md) for the exact bounds.

Snapshot opening, request/item/authority reads, original and stage validation,
serialization and logical overlay construction happen before the native writer
lock. Preparation markers follow that work. The observation pause is only a test
control. Native validation, dependency recheck, tree mutation, installation,
flushes and final root publication remain under coordination. This is conservative
whole-root validation, with no disjoint-write merging or read-set OCC.

The selected P2 admitted-domain overflow is repaired by the shared 1 MiB object
and 4 MiB input bounds; 1,024-byte keys/values and 64-pair leaves/chunks are
unchanged. Maximal escaping/high-byte values are exercised with explicit UTF-8
child transport. The selected P3 audit blind spot is repaired by checking shard,
canonical name and actual enumerated regular-file path before reading; a corrupt
wrong-shard duplicate cannot hide behind its valid canonical counterpart.

Disk batches now group affected paths and write final nodes once. Chunk keeps its
existing in-memory COW mutations and final persistence. Both coalesced adapters
share one root publication. The final-code `coalesce:false` control persists each
intermediate adapter update and still publishes only one root, separating batch
policy from acknowledgement/atomicity.

## Native correctness and review

On the final frozen code, 18 unit tests pass on each operating system. The native
nine-section transaction suite passes 2,476 Windows and 2,464 Linux assertions;
publication repair passes 116 per platform. Dedicated races pass 1,278 per platform
at widths 1/2/4/8, covering actual prepared fresh and resumed authority revocation,
lost reply, and exact terminal replay after revocation. Each final suite has zero
failures/skips. Assertions are not independent statistical trials. Detailed raw
versions, hashes and reproduction are in
[transaction-verification.md](transaction-verification.md).

The independent causal review found one P2 in the first reviewed transaction
candidate: semantic `run` incorrectly honored generic `read_only:true`, so a
terminal replay skipped lock acquisition and required publication repair. Both
old native binaries reproduce the bypass with a positive ordinary-replay control.
The repaired CLI applies `read_only` only to generic `tx`; semantic replay always
coordinates and repairs. A read-only API commit explicitly rejects replay repair.
Final injected `replay_sync` errors return indeterminate without acknowledgement,
and normal flagged replay reaches the boundary before returning the original
outcome. The per-attempt typed `CommitFailure` also prevents an earlier successful
publication from contaminating a later known-prepublication failure.

The independent Round 2 controlling verdict is **approve the bounded experiment
for human review**, with no unresolved P1 Critical or P2 Required findings.
The final P4 wording correction about semantic cohort timer boundaries is closed.
See [transaction-review.md](transaction-review.md) for review scope and resolution.

## Fresh-process lookup and reopen

The final main growth runs use 30 observations per case at 1k/10k/100k history,
deterministic seed 20261012, 96-byte initial values and fresh OS processes.
Initialization/prior samples warm an uncontrolled OS cache; this is not a cold
device test. Variant/size groups use fixed order without CPU isolation, and the
operating-system timing runs are sequential. Command time includes store opening,
validation and the operation; process wall time also includes startup, response
collection and shutdown. Observed main p95 is nearest rank (29th of 30); it is not
a capacity SLA or confidence bound. Both final full main runs have zero failures
and skips: Windows 5,924 and Linux 5,912 assertions.

| Native run | Variant | Initial keys | Positive command median ms | p95 ms | Process median ms |
| --- | --- | ---: | ---: | ---: | ---: |
| Windows | chunk | 1,000 | 5.312 | 6.152 | 16.891 |
| Windows | chunk | 10,000 | 34.446 | 44.672 | 46.562 |
| Windows | chunk | 100,000 | 360.482 | 416.529 | 374.672 |
| Windows | disk | 1,000 | 1.709 | 1.891 | 13.677 |
| Windows | disk | 10,000 | 1.860 | 2.197 | 14.853 |
| Windows | disk | 100,000 | 1.945 | 2.184 | 14.532 |
| Linux/WSL2 | chunk | 1,000 | 1.387 | 1.502 | 2.026 |
| Linux/WSL2 | chunk | 10,000 | 12.823 | 13.661 | 13.554 |
| Linux/WSL2 | chunk | 100,000 | 131.974 | 147.028 | 133.195 |
| Linux/WSL2 | disk | 1,000 | 0.295 | 0.345 | 0.846 |
| Linux/WSL2 | disk | 10,000 | 0.306 | 0.379 | 0.888 |
| Linux/WSL2 | disk | 100,000 | 0.331 | 0.405 | 0.935 |

At 100k, chunk positive lookup reads a median 1,565 objects / 40.450 MB;
disk reads 6 / 33.6 KB, including common metadata. Measured reopen medians are
353.018 ms / 0.389 ms on Windows and 127.341 ms / 0.026 ms on Linux (chunk/disk).
The transaction shell does not make the eager adapter lazy. Negative command
medians are 358.300 / 1.849 ms on Windows and 132.801 / 0.226 ms on Linux.
These counters are application reads/framed bytes, not physical device I/O.

The 100-repeat resident case keeps chunk's tree open and uses in-memory queries;
disk rereads paths because it has no resident page cache. Its aggregate core
interval must be divided by 100 before interpreting a per-key figure, and excludes
opening. Median amortized core is 0.341 / 752.568 microseconds per key on Windows
and 0.219 / 193.932 on Linux (chunk/disk). These divide one aggregate interval;
they are not independent per-key latency tails. Fresh-process batch lookup still includes reopening: 64-key command
medians at 100k are 300.913 / 48.834 ms on Windows and 129.408 / 12.947 ms on Linux.
A resident owner or lazy upstream adaptation was not implemented or priced.

The [regenerated growth summary](evidence/tx-final-growth-summary-20261012a/summary.md)
and CSV contain all cases, helper metrics and paired instrumentation on/off results.
There are 716 groups from the two final native main runs; observation rows in
`nonpassing_checks` are descriptive records, not failed assertions.

Paired instrumentation enabled/disabled command-time median ratios range
0.9804–1.0320 across Windows variant/size groups and 0.9904–0.9989 on Linux
(30 alternating-order pairs each). Negative observed deltas reflect uncontrolled
timing noise; these measurements do not isolate a precise counter CPU cost.
Fresh-process timing also does not imply that eager decoded memory is cheap to
retain across several independent agents.

## Paired batching and cancellation

Each final `transaction_measure.py` run has 6,870 passing assertions, zero
failures/skips, and 96 summary groups. Independent native sequential/coalesced
stores receive identical ordered updates and values, with alternating A/B order.
Every commit starts a fresh process and publishes one final ROOT. Initialization,
retained-storage enumeration and verification are outside the timed command.
All table cells below are medians of 30 observations. Transaction summary p95
uses zero-based `floor(0.95 * (n-1))` (28th of 30), unlike main's nearest rank;
the tables use medians and do not combine those tail estimators.

| Native run | Variant | Initial keys | Sequential 64 command ms | Coalesced 64 command ms |
| --- | --- | ---: | ---: | ---: |
| Windows | chunk | 1,000 | 397.808 | 22.553 |
| Windows | chunk | 10,000 | 835.945 | 61.755 |
| Windows | chunk | 100,000 | 1,470.065 | 397.656 |
| Windows | disk | 1,000 | 527.365 | 17.633 |
| Windows | disk | 10,000 | 620.946 | 19.883 |
| Windows | disk | 100,000 | 782.563 | 21.419 |
| Linux/WSL2 | chunk | 1,000 | 356.213 | 14.772 |
| Linux/WSL2 | chunk | 10,000 | 692.805 | 30.896 |
| Linux/WSL2 | chunk | 100,000 | 1,197.080 | 153.334 |
| Linux/WSL2 | disk | 1,000 | 232.944 | 11.742 |
| Linux/WSL2 | disk | 10,000 | 295.002 | 12.825 |
| Linux/WSL2 | disk | 100,000 | 386.089 | 14.043 |

At 100k, each 64-key logical input is 6,848 bytes. New-node/helper byte counters
include common metadata; they are not allocated blocks, physical writes or
syscalls. Median node counts can be fractional when the middle observations differ.

| Native run | Variant | Sequential / coalesced new objects | Sequential / coalesced helper bytes written | Sequential / coalesced lock hold ms |
| --- | --- | ---: | ---: | ---: |
| Windows | chunk | 670.5 / 12 | 17,335,749.5 / 285,247 | 1,111.451 / 30.868 |
| Windows | disk | 257 / 6 | 2,071,187 / 58,374 | 781.213 / 20.057 |
| Linux/WSL2 | chunk | 670.5 / 12 | 17,335,691.5 / 285,191 | 1,062.783 / 23.661 |
| Linux/WSL2 | disk | 257 / 6 | 2,071,129 / 58,318 | 385.942 / 13.894 |

These work reductions establish real intermediate-path coalescing, rather than
a delayed loop of the old disk updates. Chunk's coalesced mode retains its prior
batch policy; its sequential mode is a deliberately less efficient control,
not the predecessor default. The small cross-platform byte differences include
native identity metadata. Final main's 64-key disk write separately observes
6 objects / about 57.3 KB, versus the predecessor's 253 / about 1.95 MB;
that historical comparison is not the controlled A/B basis.

Single-key controls at 100k remain similar between sequential/coalesced modes:
Windows chunk 341.103 / 339.058 ms, disk 20.136 / 19.745;
Linux chunk 153.290 / 152.944, disk 12.363 / 12.249. They install the same
median 11 chunk or 5 disk objects per mode. Coalesced 64 process-wall medians are
410.684 / 33.170 ms on Windows and 154.722 / 14.923 on Linux (chunk/disk).

The logical preparation timer starts **after snapshot opening** and ends before
the observation pause/lock. Coalesced 64 medians at 100k are 0.0446 / 0.0429 ms
on Windows and 0.0226 / 0.0204 on Linux (chunk/disk). Initial snapshot opening is
also outside coordination, but commit responses do not provide a separate
open/core interval. Uncontended median lock waits are 0.0060 / 0.0050 ms on
Windows and 0.0024 / 0.0015 on Linux. Preparation plus lock metrics are therefore
not a complete decomposition of command wall time.

Every measured cancel/drop at one and 64 buffered pairs leaves selected ROOT and
retained bytes unchanged, with zero object writes and zero lock hold. This removes
unpublished buffer disk waste; it does not remove opening or read work. At 100k
the canceled chunk process still loads 1,564 objects before discarding its buffer.

Raw paired samples/checks and per-mode summaries are in
`evidence/windows-tx-measure-20261012a` and
`evidence/linux-tx-measure-20261012a`. Linux's native output ID includes
`-evidence-`; its copied repository directory omits that component. Original
manifest arguments and records remain unchanged.

## Retained storage and simultaneous agents

The paired stores start with approximately 40.44 MB each. The following totals
are captured after all 30 repeated samples at 100k, **before cohort writes**;
both modes also pass final audit. These are logical file lengths and retained
object counts, not allocated blocks or physical device traffic. The histories
retain all intermediate objects because reclamation is absent.

| Native run | Variant | Sequential final bytes | Coalesced final bytes | Sequential / coalesced added objects |
| --- | --- | ---: | ---: | ---: |
| Windows | chunk | 550,832,355 | 56,539,671 | 19,760 / 681 |
| Windows | disk | 103,532,412 | 43,168,989 | 7,860 / 332 |
| Linux/WSL2 | chunk | 550,830,542 | 56,537,921 | 19,760 / 681 |
| Linux/WSL2 | disk | 103,530,599 | 43,167,239 | 7,860 / 332 |

Cancellation avoids installing its private buffer. Successful batching also
reduces retained intermediate history. Neither behavior supplies garbage
collection for superseded roots, objects or possible publication outcomes.

At 100k, current RSS is sampled while every child is alive in its common prepared
five-second window. Each cell is the median of three cohort sums, in MiB
(1,048,576 bytes). The generic transaction cohorts and semantic AGENT cohorts use
separate native stores and work paths; their small differences are descriptive.

| Native run / cohort | Variant | 1 process | 2 processes | 4 processes | 8 processes |
| --- | --- | ---: | ---: | ---: | ---: |
| Windows / generic tx | chunk | 28.65 | 57.32 | 114.47 | 228.84 |
| Windows / generic tx | disk | 5.15 | 10.07 | 20.48 | 41.01 |
| Linux/WSL2 / generic tx | chunk | 25.22 | 50.50 | 101.08 | 201.89 |
| Linux/WSL2 / generic tx | disk | 2.66 | 5.29 | 10.75 | 21.21 |
| Windows / semantic AGENT | chunk | 28.57 | 57.16 | 114.46 | 229.05 |
| Windows / semantic AGENT | disk | 5.43 | 10.21 | 21.28 | 41.97 |
| Linux/WSL2 / semantic AGENT | chunk | 25.29 | 50.47 | 100.96 | 202.14 |
| Linux/WSL2 / semantic AGENT | disk | 2.68 | 5.31 | 10.73 | 21.61 |

All 24 overlap records per OS and cohort family have available RSS and valid
live-before/live-after checks. Current RSS reads are sequential within that common
window: maximum sampling intervals are 5.265 / 0.381 ms for generic tx and
5.455 / 0.343 ms for semantic AGENT (Windows/Linux). They are not simultaneous
hardware instants or sums of child lifetime peaks. Shared resident pages can be
counted more than once and Linux VmRSS can lag; no PSS or unique physical memory
claim follows. Three trials do not establish a memory capacity bound. The observed
scaling nevertheless exposes the cost of retaining several eager chunk snapshots.

In each generic cohort exactly one original snapshot commits and the remaining
`width-1` attempts explicitly conflict. Per adapter and OS, 45 initial children
produce 12 winners and 33 conflicts; all 33 explicitly re-begun/revalidated updates
then commit. All 45 independent updates survive. This is the conservative whole-root
policy exercised under overlap, with no arbitrary callback retry or disjoint merge.

The semantic AGENT wrappers perform pure-command revalidation themselves. Each
final `*-tx-agents-20261012a` run passes 642 assertions with zero failures/skips.
Across three trials at widths 1/2/4/8 there are 45 committed children per adapter
and OS; exact request/item bindings and root sequence deltas survive. Median
cohort retry totals by width are 0/1/6/28 (Windows chunk and Linux chunk),
0/1/6/26 (Windows disk), and 0/1/6/22 (Linux disk). Revalidation totals equal
retry totals. A child retries at most seven times in these runs.

| Native run | Variant | Total retries across 45 children | Maximum child cumulative lock wait ms | Maximum child cumulative lock hold ms |
| --- | --- | ---: | ---: | ---: |
| Windows | chunk | 105 | 275.683 | 66.268 |
| Windows | disk | 101 | 171.023 | 43.072 |
| Linux/WSL2 | chunk | 105 | 1,220.280 | 291.826 |
| Linux/WSL2 | disk | 90 | 103.713 | 21.703 |

These maxima accumulate attempts within one child; they are not the maximum
duration of one lock interval. Reopening a fresh snapshot for revalidation is
outside the lock and outside the logical preparation timer. Linux chunk's larger
observed wait is retained without a causal attribution to the adapter alone:
host/VHDX load, scheduling and cache are uncontrolled. Semantic cohort wall time
includes the artificial preparation pause, startup, serialized publication and
retry work; before-snapshot and after-verification commands are excluded. It must
not be treated as throughput or production concurrency capacity.

The two native cohort families retain PID-linked command responses and current
memory records in `*-tx-measure-20261012a` and `*-tx-agents-20261012a`.

## Source reuse, maintenance and remaining qualification

EchoDB 0.8.0 at revision `17b41ae82f3deba4b7d3034b0c2206a688c29f35`
contributed specific interface/lifecycle/test patterns. **Zero source lines were
copied or adapted**, and no EchoDB dependency was added. Its in-memory map and
whole-write-transaction Tokio mutex do not provide these native durable adapters
or preparation outside a process lock. No development effort savings were
measured. [transaction-reuse.md](transaction-reuse.md) records exact packaged and
pinned hashes, attribution/license, API differences and physical line accounting.

The new transaction module has 402 physical lines, including 122 test lines;
248 are nonblank/noncomment outside-test lines. The whole current adapter/common
file counts include predecessor code and must not be presented as new work. The
application owns the overlay, bounds, conservative conflict/revalidation policy,
native protocol, error classification, tests and qualification. Disk's real batch
algorithm and the growing common CLI add maintenance; they are not borrowed
durability guarantees or an evaluated resident-owner architecture.

There is no reclamation: old roots, immutable objects, unselected proposals and
staging remain. Cancel/drop discard logical preparation before installation;
they cannot undo a possibly published commit. Attempted without terminal remains
uncertainty; arbitrary external effects are outside the guarantee. Cooperative
native locking/identity and content checks do not prevent coherent all-local
rollback or noncooperating mutation. Fault injection/process death preserve OS
cache and do not qualify hardware power cuts.

Windows measurements use native NTFS/MSVC; Linux uses the existing Ubuntu-24.04
WSL2 GNU executable on native ext4 under `/home/spenser`, not `/mnt/c`. WSL2 is
Linux kernel/filesystem evidence, with an unqualified host/VHDX persistence stack
and uncontrolled background load. Native macOS was unavailable. The concrete
APFS harness/qualification plan in [platforms.md](platforms.md) remains required,
including rename/open-handle behavior, locks and identity, directory persistence,
F_FULLFSYNC/device ordering, recovery and reader lifetime. No OS/service setup or
production change was made.

The executed mount capture is retained at
[linux-tx-platform-20261012a](evidence/linux-tx-platform-20261012a/provenance.json).
