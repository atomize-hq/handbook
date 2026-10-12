# Bounded durable membership comparison

The unmodified immutable-chunkmap package is reusable for the ordered map,
balancing, snapshots and copy-on-write algorithms. Its public structural APIs
also support incremental durable encoding without serializing the whole map.
However, this adapter must eagerly reconstruct the selected tree on every fresh
process. That cost grows with history and dominates short membership operations.
The new lazy disk tree avoids that cost in this experiment, while its immediate
per-key persistence makes batches expensive. These results justify a human design
review; they do not select a production backend, format or dependency.

## Scope and method

Both variants share `src/main.rs`: semantic request/item/authority transitions,
bounded checksummed immutable objects, retained native directory/lock identity,
cooperative process locking, expected-root revalidation, root publication, fault
checkpoints, instrumentation and CLI. See [contract.md](contract.md) for exact
modeled obligations and [platforms.md](platforms.md) for native limits. Neither
variant imports a Handbook production crate. Existing capacity, contracts,
dependencies and phase behavior remain untouched.

`chunk` uses immutable-chunkmap **2.1.4**, unmodified, with 64 pairs per chunk.
`disk` is a new immutable B+tree with 64-pair leaves and internal fanout 32.
Both use the same key/value domain and initial payloads (1,000, 10,000, 100,000
unrelated keys, 96-byte values). Both enforce 128 KiB object, depth and input
bounds. There is no payload-file membership oracle, newest-survivor recovery,
reclamation or arbitrary external effect.

The main harness uses deterministic seed 20261012 and 30 samples per growth
measurement, including present/absent lookup, retained replay, fresh reservation
and terminal transition, one-key mutation, 64-key lookup/write and a 100-repeat
resident lookup. Every command starts a fresh OS process; the repeat case first
opens the index once. Initialization and previous samples may warm the OS cache.
The variants and size groups run in fixed order, without cache eviction or CPU
isolation. Observed p95 is nearest rank from 30 samples, not a capacity SLA or
confidence bound. A broad audit has only one sample per size.

Windows ran natively on Windows build 26200, NTFS, Rust 1.89.0/MSVC. Linux ran a
native GNU/Linux executable in the existing Ubuntu-24.04 WSL2 distribution on its
ext4 filesystem, kernel 6.18.40.1-microsoft-standard-WSL2, Rust 1.89.0. Its stores
were under `/home/spenser`, not `/mnt/c`. The main performance runs were sequential
between operating systems; ordinary host/background load was uncontrolled. WSL2
provides Linux kernel/filesystem behavior, not bare-metal performance equivalence
or independence from the Windows host and VHDX persistence stack. Full metadata,
commands and source/binary hashes are in each `evidence/*/environment.json`.

Fresh growth/write/helper measurements below use the repaired binary in
`evidence/windows-repaired-run` and `evidence/linux-repaired-run`. Initial results
remain in `evidence/windows` and `evidence/linux`; overlapping-agent results are
explicitly versioned separately below. [review.md](review.md) records the causal
review and repairs.

## Fresh process results

Command time includes opening the store, selected-root validation and lookup;
process wall time also includes launch, response collection and shutdown. Values
below are observed milliseconds, 30 samples each.

| Native run | Variant | Initial keys | Positive median | Positive p95 | Process median |
| --- | --- | ---: | ---: | ---: | ---: |
| Windows | chunk | 1,000 | 4.457 | 5.495 | 14.641 |
| Windows | chunk | 10,000 | 33.891 | 38.179 | 46.439 |
| Windows | chunk | 100,000 | 336.807 | 377.440 | 350.368 |
| Windows | disk | 1,000 | 1.608 | 2.243 | 14.009 |
| Windows | disk | 10,000 | 1.655 | 2.021 | 12.860 |
| Windows | disk | 100,000 | 1.809 | 2.087 | 14.328 |
| Linux/WSL2 | chunk | 1,000 | 1.384 | 1.591 | 1.965 |
| Linux/WSL2 | chunk | 10,000 | 12.699 | 14.188 | 13.412 |
| Linux/WSL2 | chunk | 100,000 | 129.200 | 141.558 | 130.747 |
| Linux/WSL2 | disk | 1,000 | 0.300 | 0.360 | 0.886 |
| Linux/WSL2 | disk | 10,000 | 0.337 | 0.612 | 1.058 |
| Linux/WSL2 | disk | 100,000 | 0.327 | 0.464 | 0.952 |

At 100,000 keys, positive lookup reads about **40.45 MB / 1,565 immutable
objects** for chunk, versus **33.6–33.7 KB / 6 objects** for disk (median instrumented
logical bytes/events; root metadata/object work is included). Windows observed
maximum positive-lookup process peak was 29,388 KiB for chunk and 5,408 KiB for
disk. Linux `getrusage` peak values are retained but should not be interpreted as
tree allocation: the fork/exec launcher can leave a pre-exec high-water floor.
The separate concurrent-agent observations below sample current resident memory.

| At 100,000 initial keys | Windows chunk | Windows disk | Linux chunk | Linux disk |
| --- | ---: | ---: | ---: | ---: |
| Negative fresh lookup, median ms | 338.742 | 1.718 | 129.451 | 0.214 |
| Exact retained replay, median ms | 334.959 | 2.286 | 130.608 | 1.676 |
| Fresh terminal request, median ms | 367.645 | 32.829 | 155.190 | 17.489 |
| One-key unrelated update, median ms | 354.468 | 17.947 | 152.204 | 11.341 |
| 64-key lookup, median ms | 290.839 | 41.222 | 130.048 | 12.981 |
| 64-key write, median ms | 387.811 | 724.295 | 152.388 | 386.968 |

The causal distinction is visible in work counters, not just elapsed time. Chunk
decodes all selected nodes before any positive or negative answer. Disk verifies
the selected path to a leaf for both presence and absence; it has no resident page
cache, so repeated queries reread paths. In the 100-repeat case, chunk's median
amortized lookup core was 0.337 microseconds/key on Windows and 0.215 on Linux;
disk's was 603.870 and 198.650. These divide one aggregate core interval, exclude
opening, and are not independent latency tails. A persistent owner process could
benefit from chunk's warm behavior, but its startup and ownership/memory costs
were not prototyped. A lazy pointer adaptation would require deeper changes to
the upstream structure; this experiment does not price or validate that fork.

## Writes, retention and helper work

Chunk batches 64 in-memory mutations before encoding the final changed paths;
disk persists each key's path immediately. At 100,000 keys, a 64-key write has
median 12 newly installed chunk objects / 285.7 KB, versus 253 disk objects /
1.954 MB, including common metadata. Disk has no dirty-page batch coalescing.
This is a concrete limitation of this implementation and an unfair basis for
claiming all disk trees have worse batches. Both reopen paths and batch policies
are visible in the source and raw counters.

| Retained bytes at 100,000 keys, Windows | chunk | disk |
| --- | ---: | ---: |
| Initial logical file bytes | 40,439,029 | 40,438,736 |
| After main measured mutations | 66,093,087 | 100,610,615 |
| Increase, no GC | 25,654,058 | 60,171,879 |
| Additional named objects | 1,098 | 8,022 |

The same Linux workload produced nearly identical logical bytes/object counts
(small native identity metadata differs). This workload includes replay setup,
30 fresh inserts, 30 one-key updates and 30 64-key writes. All old/proposed
immutable objects remain, including disk intermediate batch paths. Logical file
lengths do not measure allocated blocks, physical writes, compression or dedup.
No retained-reader or reclamation cost has been hidden by deleting history.

At 1,000 keys with 10,000 unrelated sibling entries, the explicit enumeration
guard increases Windows median lookup from 4.442 to 172.783 ms for chunk and
1.383 to 43.432 ms for disk; Linux from 1.415 to 83.507 and 0.345 to 22.880 ms.
Its repeated helper scans observe 560,000 and 140,000 entries respectively.
Exact-path guards enumerate zero entries. This stress test isolates an example
helper cost, not Handbook's full OperatingGuard implementation or an approved
change to damage-detection semantics. Core growth cases use exact-path guards;
the separate broad audit detects corrupt unreferenced named objects that an
otherwise valid exact lookup does not inspect.

Thirty alternating-order instrumentation-on/off pairs per variant/size yielded
median command ratios spanning 0.980–1.027. Negative deltas and cache/noise prevent
a precise overhead estimate. [summary.md](evidence/summary/summary.md) and
[summary.csv](evidence/summary/summary.csv) retain all groups and paired results.
Counters cover instrumented helper events and logical bytes, not total syscalls,
physical I/O, allocation counts or all upstream COW-copy events. Missing measures
remain unavailable rather than being inferred from installed-object counts.

## Overlapping agents at large history

`scripts/agent_growth.py` adds distinct requests to each established 100,000-key
growth store. Two cohorts each at 1/2/4/8 independent OS processes prepare outside
the writer lock and pause five seconds. Markers prove all processes have prepared;
the harness verifies all are alive and still paused before and after a short
current-residency sampling window. Every cohort commits exactly one root increment
per process and a reopened batched lookup verifies all original/proof/item bindings.
Both native runs pass 428 assertions, with no unavailable RSS observations.

These overlap measurements used the original common layer retained exactly in
`evidence/measured-main.rs`, before the two review repairs described below. The
repairs change publication/replay sequencing, not the preparation structures or
resident tree. Their initial timings remain versioned observations; no repaired
overlap-timing result is implied. The repaired code's full native concurrency and
fault suites separately verify its publication behavior.

| Processes | Windows chunk observed MiB | Windows disk observed MiB | Linux chunk observed MiB | Linux disk observed MiB |
| ---: | ---: | ---: | ---: | ---: |
| 1 | 28.55–28.65 | 4.78–4.79 | 25.13–25.25 | 2.44–2.50 |
| 2 | 57.05–57.09 | 9.57 | 50.44–50.45 | 4.94–5.00 |
| 4 | 114.23–114.30 | 19.13–19.14 | 100.54–100.71 | 9.75–9.87 |
| 8 | 228.23–228.31 | 38.26–38.29 | 201.66–201.68 | 19.41–19.50 |

These are sums of current OS-reported resident counters of overlapping processes,
not sums of individual peaks. Per-PID/timestamp records are retained in
`evidence/{windows,linux}-agents/simultaneous-memory.jsonl`. Sampling windows were
0.164–6.518 ms on Windows and 0.078–0.325 ms on Linux. Shared pages can be counted
in each process; Linux VmRSS may lag. This is not unique physical RAM or PSS.

At eight agents, both trials in each variant have one initial publisher and seven
conflict reopen/revalidations. Chunk's losing publishers reread the eager tree
under the writer lock: observed lock holds were about 330–438 ms on Windows and
153–170 ms on Linux, versus disk's 29–35 and 17–19 ms. The last chunk lock waits
reached 2.20 s / 0.98 s; disk 0.19 s / 0.12 s. All updates survived. Cohort wall
times, including the intentional five-second pause, were 8.10–8.21 s / 6.27–6.28 s
for chunk and 5.27 s / 5.15 s for disk. Two trials show the mechanism; they do not
estimate tails, sustained throughput or production capacity.

100,000 keys was sufficient to distinguish eager-history growth and per-agent
residency from selected-path work. The million-node decoder bound is a defensive
limit, not a measured supported capacity. The experiment stops at its authorized
geometric sizes rather than extrapolating to production history.

## Semantics, publication and recovery

Native separate processes test 1/2/4/8 publishers, stale preparation followed by
expected-root revalidation, same-original retries, changed content/proof/item,
competing requests for one item, and revocation during preparation. Each root
atomically binds the modeled request original/outcome, item, authority and stage.
Original equality is the decoded request/item/content/proof tuple, not raw CLI
JSON spelling or pause controls. Fresh and resumed work needs current original
proof; exact terminal replay returns retained outcome after revocation. Reserved,
accepted and attempted prefixes are distinct. Attempted without terminal remains
uncertain: there is no positive noncommit model, real external effect, full grant
calculus or exactly-once effect result in this prototype.

Both backends pass reached abrupt-exit and injected-error checkpoints for object
write/sync, object-name flush, staged root write/sync, root replacement,
publication flush and lost reply, plus a short-write error and external process
kill. Previously acknowledged originals remain replayable; new request and item
bindings are either both absent or both committed. Pre-publication faults retain
the old selected root; errors after replacement can expose committed state without
an acknowledgement. Recovery follows `ROOT`, never the newest surviving object.

Missing/torn/checksum-invalid roots, missing/corrupt selected nodes, lost OWNER,
lost/replaced LOCK and lost object namespace refuse explicitly. No missing
native binding is silently recreated. Held snapshot processes retain their old
root while new readers see the published state. Windows also verifies that a
ROOT handle excluding delete sharing makes replacement return an error; Unix
verifies the old open descriptor survives pathname replacement. No mmap or GC is
used. A deliberate coherent old-root restoration demonstrates the excluded
all-local rollback threat: hashes and retained local identities supply no external
freshness witness.

Windows uses native file identity, LockFileEx, FlushFileBuffers and MoveFileExW;
Linux uses dev/inode identity, flock, file fsync, rename and directory fsync.
The lock serializes cooperating expected-root checks; rename is not CAS.
Windows makes no directory-fsync or same-volume hardware power-cut guarantee.
Exit/kill/error tests preserve OS cache and do not simulate volatile storage,
guest/host reboot or device loss. [platforms.md](platforms.md) records the exact
boundary and the outstanding native macOS/APFS, F_FULLFSYNC, open-handle,
reclamation and crash/power qualification work. No macOS runner was available.

The independent review initially blocked completion on two common-layer gaps:
an existing checksum object could be an unflushed installed orphan, and exact
replay could acknowledge a root selected just before an interrupted publication
flush. The final code includes reused shards in name persistence and reflushes
selected ROOT/store boundaries under the retained lock before replay acknowledgement.
Reached object-install interruption and replay-repair checks passed on both native
systems. The original evidence/source is preserved; fresh growth measurements
and full correctness/fault checks were rerun for the repaired binary. See
[verification.md](verification.md) and the independent review record for versions.

## Reuse and decision boundary

[reuse.md](reuse.md) records exact package/revision/archive/source hashes and
full retained MIT/Apache notices. Upstream contributes 4,329 physical
implementation lines and 1,369 dedicated test lines; those are maintenance-surface
counts, not measured hours saved. No upstream code was rewritten. The new chunk
adapter is 345 implementation/comment/blank lines plus 189 test lines; the disk
tree is 494 plus 141; final common Rust logic is 896 lines (initial measured source
868). The common layer, durable
schemas, validation, bounded codec, proof/stage model, native publication,
recovery, fault harness and review remain Handbook-owned work under either choice.
The chunk memo also pins encoded historical handles for the lifetime of a live
snapshot lineage; long-lived memory growth is not solved by upstream snapshots.

The bounded question is answered: public-API chunkmap reuse is feasible, but its
eager fresh-process cost is material at the measured history/agent scales. The
small disk-oriented COW design better fits short fresh-process membership access
in this workload. Chunk retains strong warm/batch advantages; disk needs batch
coalescing and production validation. Neither result removes the shared native
authority/publication obligations. Stop here for human review before any
production selection or successor implementation plan.
