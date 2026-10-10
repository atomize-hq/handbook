# Comparable storage probes

These are deliberately small **storage-only research models**, not work-state
owners, proposed production formats, native authority validators, or approved
architecture. The structured-data standard remains proposed; existing exact
owner contracts remain controlling. No retained project store is opened.

`prototypes.py` compares the probes below. The current source is the explicitly
labeled follow-up variant; the exact base source is recoverable using the patch
recipe below. Base C results were produced before the follow-up checksums.

| Probe | Publication | Derived index / currentness |
| --- | --- | --- |
| A: individual files | One sequence-named immutable envelope file; bytes synced; authoritative head synced before replacement | Checksummed append index log, loaded into Python maps; missing/stale cache rebuilt from the head-bounded history |
| B: segmented history | 1,000 envelopes per segment; closed segments unchanged; active tail synced; same head protocol as A | Same index log and maps as A; head controls committed prefix; uncommitted tail refuses new publication |
| C: SQLite | One `BEGIN IMMEDIATE` transaction inserts immutable row, updates current map and head; `COMMIT` in WAL / FULL | SQL indexes and current map updated atomically; foreign keys enabled; formatting, foreign-key and semantic checks explicit in audit |

All three retain the **same exact opaque native original and dependency proof
bytes** once, keyed by SHA-256. A synthetic envelope refers to their digests.
Neither extracts/re-encodes/replaces the native preimage. The envelope's research
checksum commits to its own exact JSON header, including those digests, and its
predecessor; this is a new synthetic identity, never a native receipt identity.
The benchmark represents 10,000/100,000 distinct envelopes using **one shared
native original and one proof**, not that many native transactions, unique
authority histories, or work-item registrations. Terminal and administration
labels model storage transitions; native terminal/admin proof validation is not
implemented. Native authority, governance, human scope, canonical submitted vs
effective meaning, physical retained-root guard implementation, and full owner
admission machinery are omitted equally. Counts therefore measure storage
feasibility and cannot certify the complete owner or predict native latency.

The mixed population is 60% accepted, 30% terminal, 10% administrative records.
Each terminal envelope references a preceding accepted envelope for its target.
Logical ordinary and administration counters are separate. Terminal records do
not consume either research admission counter. This tests a proposed protected
closure/reserve mechanism; it does not establish disk reservation or guarantee
revocation through physical I/O failure. One target never gets a second accepted
lineage, including after terminal state. Original-key replay runs before new
admission; it cannot allocate another sequence slot.

Selected lookup observes the current head, requested envelope, selected exact
original/proof bytes, and accepted envelope when terminal. Full audit additionally
observes every committed envelope and every referenced object (shared original
and proof verified once per audit). SQLite adds `integrity_check`,
`foreign_key_check` and current-map agreement. A/B also rebuild their index during
audit, so this audit timing includes cache regeneration; it is not an apples-to-
apples pure parser microbenchmark. Neither selected lookup nor a hash chain proves
that unrelated, unread bytes are intact. All head/digest data are locally
rewritable; coherent hostile rollback/rewrite lacks an independent trust anchor.

Normal reopening for A/B reads/hashes the full small index into memory to check
agreement with the locally rewritable head; it does
not inspect all original/history bytes. Warm lookup and incremental refresh can
be bounded by selected dependencies and competing appended rows. C reopening
uses SQLite's native page/index machinery; missing SQL indexes (`REINDEX`) and
lost owner current-map reconstruction are different obligations. The measured
`sql_reindex` timing is explicitly separate from A/B complete index rebuild.

## Durability and verified versions

Observed runtime: bundled Python 3.12.14 / SQLite 3.53.1. The PATH Python 3.11.9 /
SQLite 3.45.1 is intentionally rejected for this WAL contender. SQLite's
[WAL-reset documentation](https://sqlite.org/wal.html#walreset) says the concurrency
bug is fixed from 3.51.3, with 3.44.6 and 3.50.7 backports. 3.53.1 includes that
fix; the [release history](https://sqlite.org/changes.html) also records subsequent
3.53.x maintenance fixes. This chosen runtime is a reproducible research version,
not a product dependency-selection recommendation. `sqlite_source_id()` is saved
with every benchmark result. Sources checked 2026-10-10.

C verifies `journal_mode=wal`, sets `synchronous=FULL` on every connection,
`foreign_keys=ON`, `wal_autocheckpoint=1000`, and an 8 MiB SQLite page-cache target.
The [synchronous documentation](https://sqlite.org/pragma.html#pragma_synchronous)
states that WAL FULL adds a WAL sync for each committed transaction; WAL NORMAL
can lose a committed transaction after OS/power failure. NORMAL is therefore not
benchmarked as equivalent. [WAL documentation](https://sqlite.org/wal.html) describes
one writer at a time, automatic checkpoints, and the WAL file as persistent
database state. It must accompany a copied active database. Last-connection close
can checkpoint. Samples 2/3 of timed reopen include closing the previous reopened
connection; sample 1 has no prior connection to close. This is local same-host
scratch storage; no network-file
WAL feasibility is claimed.

A/B flush Python buffers and call `os.fsync` on record data before publication;
the staged head is flushed/synced before replacement. On Windows replacement uses
`MoveFileExW(REPLACE_EXISTING | WRITE_THROUGH)`. On POSIX, rename is followed by
directory `fsync`. Python documents
[buffer flush followed by fsync](https://docs.python.org/3.12/library/os.html#os.fsync).
Microsoft documents
[WRITE_THROUGH](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-movefileexw)
and specifically guarantees flush for copy/delete moves. These calls and ordering
are real, but a same-volume Windows head replacement has **not** been certified
against OS/hardware power loss. Do not infer production crash-consistency
equivalence merely from these timings. Hardware honoring flushes is also an
assumption of SQLite's documented durability; physical I/O failure is not a
promise of a committed result. Index cache writes need no durability: committed
head/history permit rebuild. A/B have more custom recovery/publication work.

All probes use the same exclusive byte-range owner lock for operation coherence,
including C. Root/lock identities are checked within a live open session. This
does not implement the native retained-root/handle guard and cannot authenticate
a substituted entire store on fresh reopening. The common lock makes the modest
two-writer comparison meaningful for serialized owner effects, but deliberately
does not measure SQLite's additional reader/writer concurrency opportunity.

## Reproduce

Use a **fresh task-owned scratch directory outside the checkout**, with enough
space. The harness refuses existing new-store roots. It never deletes scratch
stores. Exact native fixture generation is recorded by the sibling native
instrumentation harness; fixture paths may be substituted when reproducing.

```powershell
$researchPython = 'C:/Users/spmcc/.cache/codex-runtimes/codex-primary-runtime/dependencies/python/python.exe'
$probe = 'docs/research/scalable-work-state-storage-feasibility-2026-10-10/prototypes.py'
$original = 'docs/research/scalable-work-state-storage-feasibility-2026-10-10/fixtures/native-intent.json'
$proof = 'docs/research/scalable-work-state-storage-feasibility-2026-10-10/fixtures/native-authority.json'
$scratch = 'C:/Users/spmcc/AppData/Local/Temp/hcm-scalable-prototypes-20261010'
New-Item -ItemType Directory -Path $scratch -Force | Out-Null
& $researchPython $probe faults --fixture $original --dependency $proof --scratch $scratch --output "$scratch/faults.json"
& $researchPython $probe benchmark --backend A_files --count 10000 --fixture $original --dependency $proof --scratch $scratch --output "$scratch/A_files-10000.json"
& $researchPython $probe contention --backend A_files --count 10000 --root "$scratch/A_files-10000-base" --fixture $original --dependency $proof --scratch $scratch --output "$scratch/A_files-10000-contention.json"
```

Repeat sequentially for B_segments, C_sqlite and count 100000. Do not overlap
these runs with native profiling or other builds. Population setup is reported
separately: B/C publish batches of 1,000; A syncs each file and publishes the head
each 1,000. Therefore setup throughput is not a fair per-effect durable comparison.
Every measured append/admission, terminal and administration operation checks
the retained selected original/proof bytes and publishes one individually durable
effect using the stated settings. There are 40 warm
random selected lookups, 16 samples for each mutation category, 3 audits and 3
reopens; rebuild/reindex is one observation. The reported `median_ms` uses
`ordered[len(values)//2]`: for even sample counts it is the upper-middle order
statistic, not the average of the middle pair. P95 is nearest-rank; with three
observations it equals the maximum. These are not stable production-tail
estimates. Ordering is fixed (lookup, audit, accepted, terminal, admin, reopen,
rebuild), with no deliberate cache eviction. First complete audits are separately
reported below. Reopen observes warm OS caches and is not called cold disk.

Two child processes each append 12 accepted envelopes. Process startup/reopen
are included in the total wall time; per-effect and lock-wait samples are saved.
This is **modest contention**, not throughput/load testing. Python process working
set and peak are from Windows `GetProcessMemoryInfo`; measurements include the
runtime, current maps, library caches and earlier audit allocations, not OS file
cache. File byte totals are logical sizes, not allocated NTFS blocks. Raw stores
and child result files stay outside tracked research; compact result JSON retains
timing samples and fixture digests.

`faults` uses tiny isolated populations. Deliberate removal/corruption and logical
capacity are modeled damage experiments. Children call `os._exit(91)` after data
sync or after commit, leaving actual unclosed state for reopen; these are real
abrupt process-exit experiments, **not controller kill, OS crash, power-loss or
hardware durability proofs**. Missing replies are uncertain to the caller until
recovery, although the experiment controller knows the injection stage. SQLite
rolls back the interrupted noncommitted SQL transaction. A/B retain noncommitted
orphan/tail data and refuse fresh publication: completing original orphan
recovery is deliberately an unimplemented feasibility gap, never silently fixed
with a new key/grant. Missing/stale cache rebuild, missing authoritative head
fails closed, missing selected row/record/proof fails selected lookup, and unrelated
damage passes selected lookup but fails audit. This is the expected explicit
detection-scope difference.

## Change impact and scope

Before creating/changing research symbols, GitNexus CLI impact attempts for
`FileStore`, `SegmentedStore`, `SqliteStore`, `BoundStore`, `make_record`,
`benchmark`, `fault_suite`, `rebuild`, `audit`, `read_frame`, `existing_audit`,
`lookup`, `faults` and `main` returned target-not-found /
UNKNOWN. These new isolated symbols have no production imports/callers by direct
inspection; graph coverage and graph-certified risk are unclaimed. The direct
boundary is this standalone script and its new research README/results. No
production source/dependency/configuration/fixture is changed.

## Accepted observations and limits

All six base cohorts and their six contention runs completed with exit 0. Both
follow-up C cohorts and contention runs, both B streaming audit/rebuild runs,
and the native-fixture fault suites also completed with exit 0 after assertions.
The table preserves A/B base results and uses the separately verified C checksum
follow-up as the lead C comparison. Numbers are milliseconds; each entry is the
reported upper-middle median unless marked maximum.

| Probe / initial records | Selected lookup | Full audit median / maximum | Accepted | Terminal | Admin | Reopen | Rebuild / REINDEX |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| A base / 10,000 | 6.651 | 1,037.356 / **32,443.728** | 11.153 | 31.188 | 31.686 | 55.580 | 1,221.453 |
| A base / 100,000 | 6.312 | 10,552.730 / **321,725.990** | 15.291 | 15.848 | 15.688 | 690.030 | 10,982.344 |
| B base / 10,000 | 0.964 | 909.358 / 948.519 | 6.799 | 6.707 | 6.909 | 61.425 | 854.909 |
| B base / 100,000 | 4.532 | 8,813.302 / 8,965.134 | 13.066 | 13.571 | 12.636 | 719.602 | 8,868.285 |
| C checksum / 10,000 | 0.894 | 99.228 / 103.357 | 1.060 | 1.234 | 0.912 | 2.401 | 10.294 (REINDEX) |
| C checksum / 100,000 | 0.884 | 1,080.314 / 1,121.151 | 1.037 | 1.266 | 1.046 | 3.044 | 80.803 (REINDEX) |

A's first measured complete audit took **32.44 seconds at 10k and 321.73 seconds
at 100k**; later observations were much faster. These spans are inside the timed
`full_history_audit` call, not the later untimed file inventory. That call locks,
refreshes an unchanged head, walks the committed history and rebuilds the cache.
The seed cache already exists: this is not a missing-index initial rebuild. The
cause of the large first-observation difference is **unknown**; the wall spans do
not isolate CPU, I/O, cache, antivirus or directory effects. A's selected lookup
does **not** enumerate file names. Its observed latency therefore cannot be
attributed to a per-lookup directory scan or generalized to every indexed-file
design.

B base opens a segment handle for each envelope during audit/rebuild. The bounded
`--stream-audit` follow-up opens each segment once per pass and retains the same
record checks and cache regeneration. It reports audit median/maximum of
405.749/427.059 ms at 10k and 4,120.886/4,196.635 ms at 100k, with one missing-index
rebuild of 490.226 and 4,230.545 ms respectively. These reuse fully exercised
cohorts after 48 timed effects and 24 contended effects: their actual counts are
10,072 and 100,072. They are warm follow-ups, not matched first-pass comparisons;
point/mutation results were not rerun for the unchanged streaming path.

At 100k, logical file totals were 77,433,366 bytes / 100,053 files for A base,
77,901,172 bytes / 106 files for B base, and 68,800,513 bytes / 2 files for C
checksum after clean close. Working set / peak were 209,817,600 / 210,382,848
bytes, 212,996,096 / 213,192,704 bytes and 37,376,000 / 53,215,232 bytes respectively.
The 24-effect contention wall totals were 1,745.999 ms, 1,648.147 ms and 113.296 ms;
all appended effects were visible. These include child startup/reopen. SQLite's
REINDEX still does not represent full owner-map reconstruction or history audit.
The storage comparison makes C a useful next contender; native root/guard,
currentness, authority, capacity and recovery integration remain unproved.

## Current-pointer negative and checksum follow-up

`prototype-current-pointer-negative.json` records a base C counterexample:
accepted sequence 1, committed terminal sequence 2, then a direct SQL current
pointer rollback to 1. The row and foreign keys remain valid. Base point lookup
returns stale accepted state, while full audit refuses current-map disagreement.
Base timings are preserved as evidence of that weaker modeled boundary.

The C checksum follow-up adds locally computed proofs for `(target, sequence)`
current rows and `(count, chain, ordinary, admin)` head rows, updated with each
transaction, plus selected-row sequence/target coherence checks. Pointer-only
rollback now fails selected lookup. A coherent pointer **and local checksum**
rewrite still returns the old accepted state at point lookup; full audit detects
the disagreement. The checksum is local self-consistency, not authenticated
membership in the committed history or protection against coherent replay.
Stronger current-map/head binding and the exact owner damage assumptions must be
designed before relying on this boundary. A/B follow-up refresh also validates
cached counters and the last chain against the head. All local heads remain
rewritable without an independent trust anchor.

`prototype-faults-followup.json` records this negative alongside the original
replay, logical limits, missing dependencies, truncation, unrelated damage,
missing/stale cache and interrupted publication checks. No fault or timing claim
silently treats local checksums as external authentication.

## Source correspondence and exact reconstruction

`prototype-provenance.json` records fixture lengths/digests and these source hashes:

- Base: `409c93567a2b0be9c5c4892653d10fda209ad1e65917b1f18b8dc097055eadb6`.
- Current follow-up: `e476c9302ea62dabbb4d3a74da8dc07ee623d639d75fe3eaf26e88dc3b394f54`.

`prototype-followup.patch` is the complete base-to-current patch. Copy current
`prototypes.py` into a fresh external scratch directory under the same relative
path used by the patch, then run from that directory:

```powershell
git -c core.autocrlf=false apply --unidiff-zero --reverse 'C:/Users/spmcc/Documents/__Project_Code/handbook/docs/research/scalable-work-state-storage-feasibility-2026-10-10/prototype-followup.patch'
```

This exact command was verified to recover the base byte-for-byte, with valid
Python syntax. The configuration override matters on Windows: a plain `git apply`
can reconstruct equivalent text with CRLF. If another tool changes line endings,
normalize CRLF to LF before verifying the base SHA-256. Preserve raw fixture
bytes unchanged.

`prototype-{A_files,B_segments,C_sqlite}-{10000,100000}.json`, corresponding
`-contention.json`, and `prototype-faults.json` belong to the **base**. Current
default source is **not** claimed to have generated those base C timings.
`prototype-B_segments-{10000,100000}-streamed.json`,
`prototype-C_sqlite-{10000,100000}-checksum.json`, matching
`-checksum-contention.json`, and `prototype-faults-followup.json` belong to the
**follow-up**. Follow-up checksum/streamed JSON includes the exact source SHA-256;
base JSON is mapped by the retained source and provenance.

For follow-up C, use the benchmark command above with `--label checksum`, then
point contention at `C_sqlite-<count>-checksum` and use `--label checksum` there.
For B's follow-up, use the current script's `audit-existing` command with
`--backend B_segments --count <count> --root <existing-B-base-root> --stream-audit`.
Keep these warm reuse runs separately labeled. The complete patch also preserves
the changed fault checks; reconstruct the base before reproducing base faults.
