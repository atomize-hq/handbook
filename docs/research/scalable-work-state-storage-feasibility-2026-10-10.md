# Scalable work-state storage feasibility

Date: 2026-10-10. Source baseline: `889d9b24f0d3be33169c14951d4336dd86ee03e2`
on `feat/handbook-contract-membrane-clean`. Research only; architecture,
integrity-policy choices and implementation remain **PROPOSED**. No product
sources, configuration, limits, fixtures or workspace lockfile were changed.
Every experiment used a newly created scratch store. Unrelated untracked
research present at admission was preserved. This investigation creates no
phase, slice or packet identity and prepares no implementation packet.

## Decision and confidence

The current bottleneck is repeated guarded history observation and repeated
canonical reconstruction, rather than raw JSON/file throughput alone. Native
profiling confirms four full work-history scans in one fresh registration effect;
historical native proof validation is already reused once per distinct proof
within each scan. Improving that reuse alone will not remove the dominant costs.

**High confidence:** a bounded point lookup cannot preserve next-operation
detection of arbitrary corruption in unrelated unexamined history under the
current storage/trust model. Retaining that detection behavior requires history
observation proportional to retained bytes, whatever backend is chosen. An index,
hash chain, stored segment fingerprint or Merkle root does not certify unread
physical bytes. Changing detection scope/timing is a consequential human choice.

**High confidence:** submitted/effective meaning, exact original identity and
authority do not inherently require a 32-slot lifetime bound. Indexes, exact proof
reuse and atomic publication provide candidate mechanisms for larger history;
complete native guard and recovery integration remains unproven. They cannot make disk space
unbounded or promise durable revocation through physical I/O failure. A finite
administration budget can also exhaust, and pending-original closure needs peak
writable capacity even when it consumes no new logical admission slot.

The empirical candidate comparison and conditional preference below concern
storage mechanisms, not certified full native owners. No winner is forced from
the preceding report's segmented-history recommendation.

The existing 32 lifetime slots, 128 MiB inventory accounting, 1 MiB complete-record
bound and 256-file exact checkpoint bound all matter. Raising only the slot
constant cannot establish practical capacity, integrity or bounded operation
resources; none of these product limits was changed.

## Authority and preserved meaning

The [capacity follow-up](../specs/handbook-contract-membrane/coverage-and-landing-map.md#work-state-capacity-and-performance-follow-up)
and [current priority](../specs/handbook-contract-membrane/phase-and-slice-map.md#priorities-after-compositions-and-kinds-closeout)
place this investigation before selection or bounded implementation planning.
The [completed slice](../specs/handbook-contract-membrane/slices/work-compositions-and-kinds/README.md),
[Packet 1](../specs/handbook-contract-membrane/slices/work-compositions-and-kinds/packet-1.md),
[Packet 2](../specs/handbook-contract-membrane/slices/work-compositions-and-kinds/packet-2.md)
and [semantic amendment](../specs/handbook-contract-membrane/semantic-model/amendments/work-progression-kinds-and-relations.md)
control exact submitted/effective composition, canonical identity, retained
receipts, original native authority/dependencies, one current lineage,
human-only scope successors, unchanged-scope D→G with fresh activation, suspension
during pending administration, revocation/no reinstatement, and separate kind
approval versus registration permission. Original committed recovery precedes
current permission; pending recovery uses the original request/basis. Fresh
grants or keys cannot repair uncertainty. Physical owner/root/lock binding and
coherent competing effects remain required.

The [architecture](../specs/handbook-contract-membrane/architecture-context.md)
keeps semantic ownership inward of SDK. The
[format standard](../specs/handbook-contract-membrane/structured-data-format-standard.md)
has **PROPOSED general defaults**, **Contracts JSON fixed**, **existing
owner/versioned contracts controlling**. Research encodings adopt no product
format, database or migration. Greenfield APIs do not require permanent legacy
surfaces or migration tooling by habit; actual retained originals still cannot
be erased, re-encoded under old identities, or presented as new evidence.
Hierarchy, eligibility, closing, Context Resolution, general IAM and consumers
are outside this investigation.

## Native profiling: measured costs

The [preceding investigation](work-state-capacity-and-performance-investigation-2026-10-10.md)
remains accepted evidence: 588 timed observations, 160 summary groups, full 32
current reads about 1.3–1.65 s, fresh effects at 31 about 7.27–8.74 s excluding
preparation. It was not rerun wholesale. Its simple codec/file probes omitted
native proof/guard work; this profile addresses that gap.

The [recipe, limitations and exact instrumentation](scalable-work-state-storage-feasibility-2026-10-10/README.md)
and [raw spans](scalable-work-state-storage-feasibility-2026-10-10/native-profile.jsonl)
support a regenerable [summary](scalable-work-state-storage-feasibility-2026-10-10/native-summary.json).
A fresh external git archive adds 96 function spans, preserving production
checkout sources. Windows WPR CPU profiling was attempted but could not enable
its policy (`0xc5585011`); status afterward confirmed no recording. Release
Rust/Cargo 1.89.0 MSVC, opt-level 3/no LTO, two build jobs, Engine test-support,
32 MiB worker stack, same Windows/NTFS NVMe/i 9-12900KF/approximately 128 GiB host
as the preceding evidence. Runs were serialized with candidate benchmarks.

31 observations include three enabled and three disabled-span read controls at
each cohort/size, plus one instrumented effect. Process CPU is measured separately
with Windows [GetProcessTimes](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getprocesstimes)
(sum of user/kernel thread CPU); wall spans alone are not CPU/I/O attribution.
Its observed CPU quantization was 15.625 ms, so tiny exclusive CPU spans are noisy.
There is no ETW syscall/disk/scheduler trace, hardware power-loss test, cold-cache
eviction or new-process timing. Each native call reopens and revalidates its owner;
OS caches are warm after population. The disabled control retains inserted fast
branches and is not an unpatched binary. Median enabled/control differences
range −0.23% to +2.65%; this is an observed overhead/noise bound in these small
groups, not a proof of universal instrumentation overhead. Effect overhead has
no paired control. No p 95 claim follows from these samples.

All times below are **ms**. Shared histories alternate actual 27/256-byte native
questions, within supplied scope. Distinct 16 contains two exact proof populations,
before/after supported D→G and fresh unchanged-scope work activation; it does not
represent 16 independent authority epochs.

| Cohort / operation | Slots at entry | Disabled wall median (n=3) | Enabled wall median (n=3 except effect) | Enabled process CPU median |
| --- | ---: | ---: | ---: | ---: |
| Shared current item | 8 | 380.400 | 390.463 | 390.625 |
| Shared current item | 16 | 662.459 | 660.958 | 671.875 |
| Shared current item | 31 | 1280.432 | 1294.729 | 1296.875 |
| Shared current item | 32 | 1328.757 | 1353.298 | 1328.125 |
| Two native proof populations, current item | 16 | 685.531 | 689.916 | 687.500 |
| Shared fresh registration effect, n=1 | 31 | — | 7516.385 | 7359.375 |

The full 32 current read agrees closely with the preceding explicit baseline
(native 1318.265 ms median). The new effect is near the preceding explicit
7266.455 ms boundary-timed observation. Different sampling/input/cohort and
instrumentation preclude claiming a product speedup/regression. Warm process CPU
nearly matches wall time, including kernel work. This supports a CPU/syscall
cost diagnosis for these runs, not the assertion that cold storage or flush waits
are negligible on another machine.

### Detailed attribution and repeated passes

The next table uses **exclusive** wall spans for disjoint instrumented work;
inclusive spans overlap and cannot be added. Values are component medians for
the three read profiles and one effect observation.

| Instrumented work | Current item 32 calls / exclusive ms | Fresh effect 31 calls / exclusive ms |
| --- | ---: | ---: |
| Windows directory listing / uninstrumented work within listing | 758 /572.661 | 5527 /3093.480 |
| Canonical JSON primitive | 1166 /263.717 | 6398 /1165.933 |
| Windows handle information | 36586 /133.593 | 226498 /827.650 |
| Windows directory guard opens | 9656 /124.598 | 65491 /854.547 |
| Whole history `scan` (inclusive) | 1 /1340.636 | 4 /5334.011 |
| Parsed historical rows (inclusive) | 32 /381.623 | 126 /1519.657 |
| Historical native-authority validation (inclusive) | 1 /13.301 | 5 /64.630 |
| Existing lock acquisition (inclusive) | 1 /0.255 | 7 /1.802 |

The four dominant exclusive categories total about 81% of the read and 79% of
the effect elapsed time. These are measured call/span categories, not independent
physical-device costs. Retained file hashing, reads and duplicate-safe decoding
are represented separately in raw spans; none dominates these shared-proof warm
profiles. Store inventory, slot/owner validation, retained-tree checkpoint
construction and revalidation all traverse guarded paths. Current item has four
retained-tree validations; fresh effect has 31 across work/native/checkpoint use.
The effect checks native currentness ten times (818.047 ms inclusive), captures
original authority, observes work history four times and preserves before/after
publication proofs. This is much broader than an index lookup or durable append.

At 16, exact historical validation rises from one call/12.621 ms to two calls/
25.865 ms with D/G proof populations; total enabled read rises 660.958→689.916 ms.
This is consistent with distinct proof work growing, but not a controlled proof-
size-only intervention: one item is replaced by administration, and the captured
G authority is larger. The 27/256-byte question mix does not isolate a payload
sensitivity coefficient. Large native proofs near 1 MiB,4096-source closures,
many independent authority epochs and OS-cold behavior remain unmeasured; the
report does not generalize the one-shared-proof cost to those cases.

**Source-derived optimization opportunities without a semantic change:** retain
one exact checked history snapshot and derived canonical records within the held
operation; reuse proof validation across repeated passes only while original
dependencies remain pinned and revalidated; consolidate duplicate directory/
metadata reads and path guards without losing stable physical-root checks;
replace quadratic row searches with exact maps; avoid reserializing identical
objects. These require correctness review of the held checkpoint and changing
frontier, not blindly deleting validations. No such product optimization was
implemented or timed. Next-operation global observation still needs a byte pass.

The effect's four scans occupy 5.334 s; the observed 2.182 s outside those spans
is **not** an irreducible floor or an independent authority-only cost. It includes
currentness, guards, proof capture, writes and other work. Eliminating work scans
alone therefore cannot justify an end-to-end 100/500 ms effect promise. Preparation
is separately additional, as established in preceding evidence.

## Integrity feasibility: actual boundary and counterexamples

The [source analysis](scalable-work-state-storage-feasibility-2026-10-10/integrity-notes.md)
gives exact functions, line references and supporting tests. Current scans read
all closed work inventory, positively declared slots, original intents, copied
authority files and committed grant/kind dependencies, then validate retained
physical trees and metadata. Ordinary loss/corruption of an unrelated transaction
can fail the next operation too. Historical proof validation uses copied original
authority, not today's live Charter. Fresh effects additionally hold current
native guards through terminal publication. Historical reads, preparation and
current reads may express damage differently (`Uncertain` versus refusal/error).

| Threat/damage | Supported conclusion |
| --- | --- |
| Selected original/proof/dependency or unrelated work-history bytes changed | Current native full scan observes and checks them; point verification checks only supplied selected bytes. |
| Transaction/slot/dependency deleted, truncated or unsupported partial publication | Surviving positive declarations require exact originals; loss is uncertainty/refusal, never ordinary absence or a fresh permission. |
| Copied/substituted physical root or owner lock | Native root/lock identities, exact owner/anchor/bootstrap and held handles bind the original owner. A database must deliberately preserve this integration. |
| Stale/partial manifest or head rollback | Conflicting retained successor/slot/history evidence fails closed. Cached indexes cannot become absence proof without complete generation-bound coverage. |
| Coherent hostile in-place rewrite/rollback between opens | No authenticated monotonic-history guarantee found. Source counterexample: restore an earlier valid slot, remove its later transaction/displaced artifact, preserve original root/lock and earlier exact bytes. Fresh guards can observe an internally consistent earlier store. This is inferred, not an exercised native attack. |
| Application interruption versus hardware/I/O failure | Native prefix/flush rules support truthful recovery under platform assumptions; neither hashes nor transaction libraries guarantee successful revocation if physical writes fail. |

For the point-lookup impossibility, take two stores identical in every observed
index/head/Merkle node and selected dependency byte. Damage one unrelated payload
byte without changing the stored commitment. The algorithm sees identical inputs
and returns the same result. A membership proof establishes bytes supplied and
hashed against a trusted commitment; it cannot establish availability/integrity
of unread bytes. Full byte observation or an additional trusted observer is
necessary. Periodic/reopen scrubbing changes **when** damage is noticed. This
would require explicit human selection, even if the exact semantic identities
and selected-dependency checks remain intact.

Concrete serious mechanisms and failure cases are:

| Alternative | Proof/commit mechanism proposed | Assumptions and counterexample |
| --- | --- | --- |
| A indexed individual files / proof-reuse control | Positive immutable accepted declaration, exact byte-addressed original/proof objects, generation-bound complete indexes, checked selected lineage/dependencies; original acceptance before terminal publication; rebuild derived cache under original lock. | Complete authoritative generation/coverage and retained native root/lock are required. Unread file damage survives a point lookup; global detection adds a full pass. New records may reference exact reusable proof objects only under a selected versioned contract; old embedded preimages cannot be rewritten. |
| B segments / indexes | Framed records with length/checksum/sequence, sealed segment coverage/digest, authoritative accepted boundary/head, immutable terminals and index generations; tail recovery follows positive publication evidence. | Missing committed segment/boundary is damaged evidence. Stored seals do not observe unread segments. Cross-file acceptance/head/index publication and ambiguous tails need an explicit recovery protocol; index/head loss cannot silently reset authority. |
| C transactional embedded owner database | Immutable exact BLOB originals/proofs, unique request/lineage/dependency constraints, durable original-acceptance transaction, separately authority-revalidated terminal/head/index transaction; original-key recovery of unacknowledged COMMIT. | SQL atomicity does not supply human/native authority, exact semantic validation, physical retained-root binding or authenticated antirollback. A selected SQL query/structural check can succeed with unrelated payload damage or valid SQL deletion; semantic audit still needs authoritative byte/history coverage. |

No mechanism uses a cached hash as proof that all unexamined bytes remain intact.
The source notes distinguish mechanism proposals from what prototypes actually
implement. Any future reusable proof representation must preserve exact raw
objects and original preimage domains/receipts. It cannot retroactively swap an
embedded proof for a reference in retained old intent bytes.

**SQLite's remaining native-integrity integration gap is material.** Stable
parent/root binding and the original work lock can surround a SQLite operation,
but the existing immutable-file checkpoint cannot simply wrap a SQL write:
legitimate DB/WAL bytes and versions change. Standard Windows VFS sharing and
protocol byte-range locks do not exclude arbitrary raw out-of-band writes, and
an identity check alone cannot distinguish them from the owner's expected writes.
SQLite cache/mmap behavior also needs accounting when claiming physical-byte
currentness. A plausible mechanism is one serialized owner connection under a
VFS that holds compatible native path/handle identities and excludes other writable
handles for DB/WAL/SHM during the effect; another is a verifying VFS with exact own-
write/range accounting and protected physical rereads. Neither is implemented or
proven here, and a PRAGMA/ordinary SQL transaction does not establish it. This
is an explicit feasibility condition on C, not an approved relaxation of the
through-publication safety boundary. A/B similarly have unimplemented native
handle integration and stronger publication/recovery obligations.

## Comparable storage experiments

The [prototype source](scalable-work-state-storage-feasibility-2026-10-10/prototypes.py)
and [reproduction/official sources](scalable-work-state-storage-feasibility-2026-10-10/prototypes-README.md)
describe A indexed files, B segmented history and C SQLite. All use the same exact
native committed original/proof bytes, selected byte hashing, synthetic envelope
checksums, predecessor chains, intended current-map semantics and exclusive owner lock.
Opaque originals are retained once and referenced, never re-encoded. Records are
**unique synthetic envelopes**, sharing **one native original/proof**, not fully
authority-admitted native registrations. Native authority/currentness, semantic
canonical validation and retained native handle/root guard integration are
omitted equally. A record's terminal/admin label is a storage-model transition,
not a native receipt/revocation. Fair comparison requires reporting those omitted
costs separately; the native profile is their anchor, not an addable stopwatch.

Population is 60% accepted/30% terminal/10% administration, at 10k/ 100k initial
envelopes; terminal references its prior accepted record. Setup is batched and
excluded from per-effect comparisons. Timed mutations publish one effect with
real flush/COMMIT settings; no unflushed key lookup is described as native effect.
A/B sync data/staged head and use Windows MoveFileEx WRITE_THROUGH; same-volume
rename has not been hardware-power-loss certified. C uses SQLite 3.53.1 through
bundled Python 3.12.14, WAL, FULL per connection, foreign keys enabled,8 MiB page
cache and automatic 1000-page checkpoints. The PATH SQLite 3.45.1 is rejected.
The official [WAL-reset correction](https://www.sqlite.org/wal.html#walreset)
is included from 3.51.3 (with 3.44.6/3.50.7 backports).
[WAL FULL](https://www.sqlite.org/pragma.html#pragma_synchronous) synchronizes
commits; NORMAL's potential power-loss rollback is not benchmarked as equivalent.
Persistent WAL must accompany an active database. SQLite allows one writer;
the shared owner lock deliberately serializes all candidate effects.

Full audit and point lookup are separate operations. Audit rechecks all committed
envelopes and referenced objects; A/B additionally rebuild cache, C additionally
checks SQLite structure/foreign keys/current map. This is not identical pure
parser work, but each declares its validation obligation. Normal A/B reopen hashes
the full small index into memory; C uses DB/index pages. A/B complete missing-index
rebuild and SQL REINDEX are distinct operations, not interchangeable repair proofs.
Observed memory includes Python/runtime, maps, caches and earlier audit allocations;
logical bytes exclude NTFS allocation and OS page cache. Warm reopen is not cold disk.

### Accepted base matrix

All entries below are milliseconds unless labeled otherwise. These are reported
upper-middle medians (the estimator does not average the middle pair for even n),
with n=40 selected reads, n=16 per mutation, n=3 audit/reopen and n=1 rebuild.
Full samples and nearest-rank p 95 remain in the six compact result files:
[A10k](scalable-work-state-storage-feasibility-2026-10-10/prototype-A_files-10000.json),
[A100k](scalable-work-state-storage-feasibility-2026-10-10/prototype-A_files-100000.json),
[B10k](scalable-work-state-storage-feasibility-2026-10-10/prototype-B_segments-10000.json),
[B100k](scalable-work-state-storage-feasibility-2026-10-10/prototype-B_segments-100000.json),
[C10k](scalable-work-state-storage-feasibility-2026-10-10/prototype-C_sqlite-10000.json),
[C100k](scalable-work-state-storage-feasibility-2026-10-10/prototype-C_sqlite-100000.json).

**Base C has a demonstrated selected-current binding gap**, described below.
The base matrix is retained historical evidence, not a claim that all contenders
meet the native integrity contract. The later checksummed C cohort supplies the
more appropriate basis for its conditional preference.

| Backend / initial envelopes | Selected read | Durable accepted | Durable terminal | Durable administration | Warm reopen | Full audit | Missing-index rebuild / SQL REINDEX |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| A files / 10k | 6.651 | 11.153 | 31.188 | 31.686 | 55.580 | 1037.356 | 1221.453 rebuild |
| A files / 100k | 6.312 | 15.291 | 15.848 | 15.688 | 690.030 | 10552.730 | 10982.344 rebuild |
| B segments / 10k | 0.964 | 6.799 | 6.707 | 6.909 | 61.425 | 909.358 | 854.909 rebuild |
| B segments / 100k | 4.532 | 13.066 | 13.571 | 12.636 | 719.602 | 8813.302 | 8868.285 rebuild |
| C SQLite / 10k | 0.811 | 0.949 | 1.244 | 0.911 | 2.602 | 72.643 | 10.329 REINDEX |
| C SQLite / 100k | 0.811 | 0.937 | 1.235 | 0.902 | 2.520 | 892.560 | 73.070 REINDEX |

A's audit samples are particularly uneven: the first audits take **32,443.728 ms
at 10k and 321,725.990 ms at 100k**, versus repeat-audit medians about 1.04/10.55 s.
With only three samples, those maxima are also its reported nearest-rank p 95.
The cause is unprofiled; neither antivirus nor filesystem metadata costs are
established causal explanations. These are measured audit costs, not untimed
population or inventory work. B's base audit opens a segment handle per envelope;
the streaming follow-up below addresses that avoidable implementation cost.

Selected-read empirical p 95 at 100k is 7.167/8.253/0.878 ms for A/B/C. Durable
accepted/terminal/administration p 95 there is 16.335/16.661/22.417 ms for A,
14.806/25.376/13.443 ms for B and 1.803/1.369/1.027 ms for C. These modest fixed-order
samples establish observations on this host, not reliable production tails.

| Backend / initial envelopes | Logical bytes (MiB) | Files | Process working / peak (MiB) | Two writers, 24 visible commits, total wall ms |
| --- | ---: | ---: | ---: | ---: |
| A / 10k | 7.70 | 10053 | 41.02 /41.57 | 394.579 |
| A / 100k | 73.85 | 100053 | 200.10 /200.64 | 1745.999 |
| B / 10k | 7.74 | 16 | 41.82 /41.82 | 364.668 |
| B / 100k | 74.29 | 106 | 203.13 /203.32 | 1648.147 |
| C / 10k | 6.48 | 2 | 33.96 /33.96 | 117.146 |
| C / 100k | 61.13 | 2 | 36.19 /50.43 | 113.345 |

The six sibling `*-contention.json` files record per-effect and lock-wait samples.
All 24 effects are visible afterward. Total wall includes child startup and reopen;
A/B 100k must load the whole index before competing. This cannot be converted into
a clean mutation-throughput ratio. Two callers under the common exclusive owner
lock do not establish eight/sixteen-caller behavior or starvation bounds.

Byte counts are after 48 measured mutations and before 24 contention additions;
the count includes metadata/shared objects. C's two files are persistent database
and common owner lock after close/checkpoint, not a promise that a live SQLite
owner never has WAL/SHM files. The A/B memory growth includes full Python maps.
These approximately 61–74 MiB stores share one original/proof and therefore do
not predict 100k native transactions' retention. A/B initial population methods
also differ from C; their setup durations are retained but excluded from the
durable per-effect comparison.

### Transition, damage and interruption evidence

[Fault results](scalable-work-state-storage-feasibility-2026-10-10/prototype-faults.json)
contain three modeled suites and twelve actual abrupt child exits. They establish
the following bounded outcomes:

| Experiment | Observed result / limit |
| --- | --- |
| Accepted original before terminal; one lineage | Terminal requires the retained accepted envelope; second acceptance for the same target is refused, including after terminal. Native receipt semantics remain outside the prototype. |
| Missing reply after committed accepted/terminal | Reopen and exact original-key replay return the existing record; changed exact request bytes are refused. No new sequence/key repairs uncertainty. |
| Exit after data, before positive commit | A/B retain an orphan/tail and refuse fresh publication; original orphan recovery is deliberately omitted. C rolls back its noncommitted transaction, allowing exact-request retry; the prior accepted row survives interrupted terminal publication. |
| Logical ordinary capacity 2, separate administration capacity 2 | Ordinary saturation refuses further accepted rows; two protected administration rows remain available, then administration also refuses. Terminal closure remains possible after both logical counters fill. This provides no physical disk reservation or I/O-failure revocation guarantee. |
| Missing/stale A/B cache | Rebuild from positively bounded history; missing authoritative head refuses. C missing head also refuses; SQL REINDEX is not reconstruction of a lost owner current map. |
| Selected original, proof, accepted record, truncated row or missing B segment | Selected verification refuses damaged/missing evidence. No absence is promoted to permission. |
| Unrelated damaged record | Selected lookup succeeds; full audit refuses. This is the measured detection-policy counterexample, not semantic equivalence to the native next-operation global scan. |

The child deliberately calls `os._exit(91)` at instrumented stages. It is an
actual unclosed process exit, not a controller kill, OS crash, power-loss test
or hardware durability certification. Caller outcome is uncertain until original
recovery even when the experiment controller knows the injection stage.

### Focused fairness and selected-current follow-ups

Review of the prototype itself found two concrete questions worth measuring
within this research scope. The [provenance record](scalable-work-state-storage-feasibility-2026-10-10/prototype-provenance.json)
maps exact base/follow-up script hashes to every output. Reverse-applying the
[small source patch](scalable-work-state-storage-feasibility-2026-10-10/prototype-followup.patch)
to a scratch copy reconstructs the base source. Default checked-in source includes
the follow-ups; it must not be described as the unmodified base source.
The original base hash uses LF endings; the verified reconstruction command sets
`core.autocrlf=false` so global Git settings do not change those bytes.

First, B's optional streaming audit opens one handle per segment while preserving
record verification and cache rebuild. It was run on the already warm, already
audited base stores after 48 measured mutations and 24 contention records; these
are not fresh matched-cache replications. Only the affected audit/rebuild timings
were rerun, with the unchanged read/mutation base evidence retained.

| Streaming B cohort | Actual envelopes | Full audit median ms (n=3) | Missing-index rebuild ms (n=1) |
| --- | ---: | ---: | ---: |
| [10k](scalable-work-state-storage-feasibility-2026-10-10/prototype-B_segments-10000-streamed.json) | 10072 | 405.749 | 490.226 |
| [100k](scalable-work-state-storage-feasibility-2026-10-10/prototype-B_segments-100000-streamed.json) | 100072 | 4120.886 | 4230.545 |

Second, the [base C negative experiment](scalable-work-state-storage-feasibility-2026-10-10/prototype-current-pointer-negative.json)
changes a target's SQL `current` pointer back to its still-valid accepted row after
a committed terminal. Selected lookup returns stale `accepted`; full semantic
audit detects map/history disagreement. SQL foreign keys and valid retained rows
alone do not establish currentness. The follow-up adds selected row sequence/
target checks and checksums for head counters and current pointers, then reruns
both fresh C populations and contention using that code.

| Checksummed C cohort | Selected read ms | Durable accepted / terminal / administration ms | Warm reopen ms | Full audit ms |
| --- | ---: | ---: | ---: | ---: |
| [10k](scalable-work-state-storage-feasibility-2026-10-10/prototype-C_sqlite-10000-checksum.json) | 0.894 | 1.060 /1.234 /0.912 | 2.401 | 99.228 |
| [100k](scalable-work-state-storage-feasibility-2026-10-10/prototype-C_sqlite-100000-checksum.json) | 0.884 | 1.037 /1.266 /1.046 | 3.044 | 1080.314 |

The C 100k follow-up retains 68,800,513 logical bytes and peaks at 53,215,232 process
bytes; its two-writer 24-effect total is 113.296 ms. All 24 effects are visible in
both rerun contention cohorts. The [follow-up fault suite](scalable-work-state-storage-feasibility-2026-10-10/prototype-faults-followup.json)
repeats all interruption/damage/capacity checks and adds changed-current-pointer
refusal. A/B's original recovery omission and unrelated-damage policy contrast
remain unchanged. A checksum detects the tested unaccompanied pointer change;
it is not authenticated currentness. A separate follow-up fault coherently
rewrites the old pointer and its matching checksum: point lookup returns stale
accepted state, while full history audit refuses. Complete coverage/guard/trust
reasoning is still required. Native currentness and through-publication
integration therefore remain conditions rather than solved by this variant.

### Maintenance and ownership tradeoffs

| Candidate | Advantages shown / plausible | Obligations still unresolved |
| --- | --- | --- |
| Indexed individual files | Exact immutable objects and derived rebuildable indexes are simple to inspect; selected reads need not enumerate history. No external database library. | File count, full inventory/checkpoint work and map/reopen growth; unusually long first-audit samples; bespoke multi-file positive publication/orphan recovery and native handle integration. Proof reuse cannot rewrite old exact preimages. |
| Segments | Much smaller file count, bounded active tail, straightforward immutable export; streaming can remove per-envelope handle churn. | Custom frame/tail/accepted-boundary/index recovery, exact sequence/coverage proof, large in-memory maps and full-index reopen; native root/guard integration. Compaction/export cannot erase original truth or manufacture new receipt identities. |
| SQLite | Lowest observed storage-only operation costs and memory here; one atomic transaction maintains record/current/head/index agreement; documented WAL FULL protocol and rollback reduce bespoke interruption handling. | Dependency/version maintenance, DB/WAL/SHM lifecycle, checkpoint/backup discipline, schema constraints and semantic audit, selected-current binding, and the material native through-publication VFS/handle gap. SQL structural integrity alone does not prove exact native authority or complete history. |

The owner remains Engine-side. SDK should consume exact original locators and
owner results; exposing SQL rows/files or storage-specific authority to SDK would
cross the existing boundary. Human-readable projections can be derived without
becoming new evidence. No prototype demonstrates a migration, compaction,
export/reimport or compatible retained-store upgrade.

## Proposed practical envelope and conditional preference

The prior report's 10k/ 100k workload sensitivity remains useful, but its 50/100 ms
read and 250/500 ms effect goals were aspirations. The new native attribution
does not demonstrate that full guarded operations can attain them. Its 16–256 KiB
shared-proof planning range likewise remains unproven; one shared native fixture
does not measure realistic distinct authority/definition cardinality.

For human review, the justified narrower **PROPOSED** envelope is:

| Dimension | Proposed boundary and evidence |
| --- | --- |
| First native support candidate | 10,000 lifetime accepted transactions, with explicit retained administration/noncommit/terminal accounting and independently grown authority/kind histories. This is a validation target, not support delivered here.100,000 mixed synthetic envelopes are the demonstrated storage stress point;100,000 native transactions remain a later validation target. One million is a sensitivity only. |
| Retained data | Until realistic proof reuse is measured, budget using the prior native mix's 245,550–298,130 bytes/transaction: approximately 2.29–2.78 GiB at 10k and 22.87–27.77 GiB at 100k, before indexes, filesystem allocation, journals and peak publication/recovery reserve. Selected distinct proofs may exceed this mix. No compression ratio or disk-space guarantee is inferred. |
| Storage-component latency | On the selected local SSD host, propose a repeatable validation budget of empirical p 95 ≤10 ms for selected warm lookup and ≤50 ms for individually durable storage mutation at 10k/ 100k shared-proof stress, absent contention. The base samples fit; these are unapproved component budgets, not full native SLAs. Include every selected integrity check in any follow-up. |
| Full native latency | No new millisecond SLA is evidenced. Retain the measured 32-slot read≈1.3–1.65 s and 31-slot fresh effect≈7.27–8.74 s as baseline anchors; any proposed improvement must measure the complete held authority/guard/recovery path. Preserved global detection additionally scales with bytes. Do not advertise 100/500 ms from the storage probes. |
| Reopen / audit / recovery | A/B warm reopen is about 0.69–0.72 s at 100k; C about 2.5 ms. Full audit remains explicit O(history); A's observed first pass reaches 322 s. No universal 5 s full-verification promise. Require fail-closed original recovery and progress for long audits/rebuilds; measure unique proof bytes, cold OS behavior and lost-current-map repair separately. |
| Memory / contention | A provisional 512 MiB process validation budget at 100k shared-proof stress leaves headroom above the observed≈203 MiB A/B and≈50 MiB C peak; realistic native proof/map sizes remain unmeasured. Two competing local writers are evidenced;8/16-caller waits/starvation require separate validation. Owner serialization remains required unless authority/currentness coherence is separately established. |
| Saturation | Ordinary logical admission exhaustion must preserve explicit administration and pending-original closure capacity. Administration reserve itself is finite. Physical bytes/temporary journal/closure peak must be budgeted separately; when unavailable, return truthful refusal/uncertainty without fresh-key repair or a revocation guarantee. |
| Retained originals | Human-select new representation/version and actual retained-store treatment. Greenfield status does not require permanent legacy APIs/migration tooling, but actual original bytes, authority basis and receipts cannot be discarded/re-identified silently. |

**Moderate-confidence conditional preference: C, transactional embedded owner
storage, for the next feasibility proof.** Its measured operation, reopen and
memory costs plus atomic multi-object publication/recovery make it the strongest
storage mechanism in this bounded comparison. Confidence is limited by modeled
shared proofs, small warm samples, unequal full-audit implementations, current-map
binding and especially unproven native through-publication guard integration.
This is not selection of SQLite, a new owner contract or altered detection policy.
If database write/handle coherence cannot preserve that boundary within acceptable
complexity, B remains a credible alternative with explicit tail/publication
recovery; A remains a useful indexed-file control and possible smaller-store design.

**Smallest next step:** human review of detection scope/timing and the envelope,
followed, only if separately authorized, by a narrow native-integrity feasibility
proof for C's physical-root/DB/WAL/SHM write coherence and exact current-map/head
binding. It must distinguish own legitimate SQL writes from outside changes while
holding native authority through publication; normal Windows VFS sharing,
`locking_mode` or an identity-only handle check is insufficient. This named gap
precedes backend adoption. No implementation packets, detailed successor plan or
product edits are prepared by this investigation.

## Verification and limits

The external instrumented release build succeeded with the prior exact locked
dependencies; the native process completed with exit 0 and 31 observations.
Regenerating its summary from accepted raw observations yields the same SHA-256
(`508f7507dfedb7882d25a27b2e11fe2cf5ad06380e45bf45ddc9c1edc7963d4a`).
The serialized six base cohorts and their six contention runs completed with
asserted transitions and 24 visible effects per contention run. Compact JSON
retains raw samples, runtime/version details and exact shared-fixture digests.

GitNexus CLI 1.6.12 query/context lacked required index tables/definitions and
reported an index 66 commits stale. Required impact attempts for native
instrumented and new research symbols returned UNKNOWN/target-not-found; no
graph-certified production blast radius is claimed. Direct source inspection
bounds the instrumented copy and new standalone research harness. The mandated
comparison against `main` reports CRITICAL: 1,205 files, 678 symbols and 300 processes.
That includes inherited branch work, not this investigation's contribution;
staged-scope checking and direct product-diff checks distinguish it. No broad
index repair or inherited branch review was performed.

No full product suite was replayed for this research/docs-only change. WPR's
policy failure, absent cold/cache/power-loss testing, native-large-proof omission,
storage-model semantics, original orphan recovery omission, native VFS integration
gap and selected/global detection difference constrain every completion claim.
