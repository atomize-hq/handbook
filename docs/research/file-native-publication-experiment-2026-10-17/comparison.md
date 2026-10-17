# Comparative findings and remaining choices

**Partial native Windows evidence. Both A and B remain open.** The corrected
fixture establishes the tested process-level publication/outcome/provenance
properties on Windows. It does not complete the required native Linux or ordinary
file-browser qualification, full growth allocation, metadata-backend comparison,
OS-crash durability or GC qualification. This is a bounded result for human review,
not a production architecture selection.

## Observed browsing and recovery

| Question | A: exact originals + mutable view | B: retained named generation |
| --- | --- | --- |
| Paused ordinary reader across publication | Old overview / new requirement observed | Old overview / old requirement at the chosen exact path |
| Paused directory copy across publication | Mixed old/new copy observed | Complete old copied manifest and packet verified |
| Exact API/retained assembly | Selected state and retained inputs/provenance stay exact; mutable view status is separate | Selected state plus complete generation/packet stay exact |
| Refresh failure | Already accepted; reports view-pending; resume renders without replaying mutation | Incomplete generation cannot select; complete losing candidates remain orphans |
| Reopening after lost reply beneath successor | Original request/result resolves from selected lineage | Same, including original result after successor deletes affected document |
| Per-open current lookup | Does not bind a multi-file reader | Alias-control reader mixes when it deliberately reselects for each file |
| Retention/GC | All advertised state/assembly bases retained; no GC proof | All advertised generations retained; no GC proof |

The corrected suite produced 120 main race observations, 480 semantic combination
observations and 233 process-fault rows. The independent reviewer checked 174
must-not-select rows, 51 must-retain rows, six visible-uncertain rows and two
partial-payload retry rows. There were no phase violations. Four additional
slow/small configurations and two combined import/move captures passed. These
counts describe fixture cases and raw observations, not exhaustive filesystem
fault coverage or a production concurrency proof.

Ordinary tool-free extraction through explicit files/navigation was exercised by
scripts; an unfamiliar-human usability study was not performed. Explorer's helper
could observe its window but failed to navigate to the fixture, so that required
UI check is unmet. Native Linux authentication failed before remote OS/filesystem
inspection. Neither WSL nor API pathname success was substituted.

## Cost observed in the reached workloads

See the [complete reached-case table](evidence/growth-table.md),
[stage/counter summaries](evidence/growth-summary.json) and
[raw samples](evidence/raw-index.json). The displayed serial samples are individual
observations, not population percentiles or product SLAs.

| Workload | A engine wall time | B engine wall time | Interpretation |
| --- | ---:| ---:| --- |
| 1k, one changed document, four store/policy serial samples | 82.07–102.87 ms | 1,123.80–1,145.97 ms | A copies changed view bytes plus the saved packet/dependency inputs; B copies all 1,000 source documents and verifies the complete generation |
| 10k project/admission, one document | 6,267.44 ms | 13,171.08 ms | A's saved packet includes the overview's 5,000-constituent closure; neither figure is metadata-only or just view-refresh time |
| 10k project/admission, eight documents | 5,914.66 ms | 12,863.89 ms | Same fixture/validation, inclusive preparation and publication costs |
| 10k project/admission, 64 documents | 6,443.03 ms | 13,327.61 ms | Includes one captured 256 KiB attachment in the same batch |

At 1k, A's single-document helper count was 12 file copies (nine retained packet
input/dependency files, one view source, two packet outputs); B copied 1,000 source
files. The corresponding instrumented logical I/O was about 2.31 MiB read / 0.75
MiB written for A versus 37.78 MiB read / 12.81 MiB written for B. Counts exclude
unmeasured kernel/device activity and include the declared rereads. They do not
imply physical disk traffic or a cold-device result.

The 10k completed project admission stores retained exactly 10 selected states:
A used 961,085,635 logical bytes and B 1,478,043,470 logical bytes. A's saved
dependency packets substantially narrow the storage difference at this fanout.
Actual file allocation, metadata object/name counts, selected and abandoned
generations are tabulated separately. These numbers are specific to this packet
frequency, closure policy, whole-manifest candidate representation and directory
layout. They are not a lower bound for every implementation of either option.

Speculative preparation creates measurable discarded work. Both 1k B speculative
stores retained 20 selected generations plus complete unselected candidates;
their orphan counts/bytes are in the summary. The admission policy retained its
selected generations without those speculative copies. Admission also makes
final preparation part of lock hold, and the 10k waves produced real busy
refusals at the fixed 15-second wait ceiling. Those attempts remain in the raw
log and byte totals. Explicit resubmission of missing pure commands subsequently
completed the 10-state allocation; already accepted IDs were skipped using
positive history. A lock does not imply FIFO, and one bounded sequence does not
establish starvation freedom.

Native successful-attempt lock wait/hold is recorded. Busy attempts have engine
wall times around 15 seconds; their lock-wait stage counter is absent because
the timeout path returns before recording that stage. Do not interpret the
missing counter as zero wait. Raw attempt durations do not reconstruct total
request latency across a controller restart. The slow/small probe separately
records driver-observed completion upper bounds, including explicit retries.

## Reached allocation versus declared allocation

| Allocation | Actual qualification |
| --- | --- |
| 1k, A/B × project/research × admission/speculative | All eight configurations complete; 20 selected publications each, including initial. B generations and speculative orphans retained |
| 10k, project admission A/B | Both complete; 10 selected publications each; 5,000-constituent closure exercised |
| 10k, remaining six consumer/policy/option configurations | Unreached after prioritizing paired large-tier attempts within the original time cap |
| 100k, project admission A | Initial command hit the 600-second per-case timeout before any selected initial publication. Prepared files remain; no positive state or large lookup/publication sample is inferred |
| 100k, project admission B | Initial command also hit the 600-second per-case timeout before any selected initial publication; files retained, no qualified large state or publication timing |
| 100k, remaining six configurations and later publications | Unreached within this run's caps; no p95 or capacity conclusion |

The 100k row is censored preparation evidence, not a completed 100k benchmark or
proof that file-native storage has a 10k product limit. This layout creates a
directory per immutable original, syncs each payload and installs it with a
Windows write-through move, then copies/verifies readable closure. Those choices,
background host activity and the fixed per-case budget matter. No advertised
snapshot was deleted to improve a growth curve.

The original 64 GiB data allocation and 160 GiB free-space reserve apply to all
three run roots plus builds. Final file-allocation accounting is in
[the inventory](evidence/inventory.json). NTFS MFT/directory-index allocation is
not attributed by FileStandardInfo file allocation; free-volume checks include
all use. The later optional probes exceeded their secondary 32 MiB sub-allocation
(43,471,912 allocated file bytes); that overrun is explicit and was not hidden by
deletion or a silent global cap increase. The cap guard stops remaining growth
processes at 16:48 UTC (12:48 PM EDT), leaving the reporting reserve inside the
original 80-minute total budget.
Final accounting was 9.62 GiB in allocated file bytes with about 230.26 GiB free.
Both large cases ended at their per-case timeout; the guard then observed no
remaining growth process. See [artifact verification](evidence/verification.json).

## Backend and native qualification

Gate 3 was not entered. There are **no newly integrated DoltLite, SQLite or lazy
COW comparative measurements** here. Prior tiny-value lazy-COW numbers are not
pooled with document/readable-publication results. Missing native/platform
evidence and remaining execution time do not justify building three adapters,
claiming external publication authority fits, or assuming retention/GC behavior.
DoltLite remains a pinned optional benchmark comparator only; no adoption,
fork, wrapper port or production dependency change occurred.

Windows evidence qualifies process exit/kill/reported-error behavior with a
functioning filesystem. File sync/MoveFileExW WRITE_THROUGH are recorded;
directory synchronization is explicitly unsupported. Process death leaves OS
cache alive. Linux/ext4, macOS/APFS/Finder, nested-name crash persistence, stronger
flush semantics and real power-cut/reboot behavior remain unproven. So do backend
pin registration/GC races, retirement and production owner integration. Local
hashes do not establish coherent rollback freshness without an outside witness.

## Human decisions left open

The evidence supports preserving both contracts as candidates. A offers a cheaper
ordinary incremental tree at small scale with mixed/stale observation and resume
semantics. B buys a retained complete directory snapshot for an ordinary reader
that chooses one path, at full-generation materialization and retention cost.
Neither can promise an always-current multi-file browsing session to arbitrary
unmodified applications.

Product decisions still include acceptable latency/storage budgets, assembly
frequency and dependency closure, discovery ergonomics, retention lifetime and
audit strength. Metadata-tree sharing may reduce metadata work, but it does not
by itself remove copied document bytes/names or saved packet closure. No browsing
contract/backend was selected, no production state migrated and no successor
task was launched.
