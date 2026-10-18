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
