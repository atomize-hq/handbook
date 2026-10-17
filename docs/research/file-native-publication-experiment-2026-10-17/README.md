# File-native publication experiment — bounded native evidence

This experiment implements both browsing options over one standalone Rust
document/graph fixture. It selects neither a product browsing contract nor a
backend. The controlling brief is the October 17
[execution proposal](../dolt-gitbutler-file-engine-2026-10-16/experiment.md).
The broader [format standard](../../specs/handbook-contract-membrane/structured-data-format-standard.md)
remains proposed; existing production owners and exact JSON identities were not
changed. There are no production source or root dependency edits.

**Execution status: native Windows process evidence; qualification incomplete.**
Native Linux/ext4, successful ordinary Explorer/Linux browser exercises, OS-crash
durability and actual reclamation remain unproven. No WSL results substitute for
native Linux. Gate 3 was not entered: there is no integrated external comparator,
no new COW/backend comparison and no Dolt adoption claim.
All eight 1k configurations and the two 10k project admission configurations
completed their selected-history allocation. Both 100k project admission initial
commands reached their 600-second timeout before an initial selection. Remaining
large/medium configurations are unreached, not inferred failures or successes.

See [protocol and caps](protocol.md), [independent causal review](review.md),
[raw evidence index](evidence/raw-index.json), [build/source pins](evidence/pins.json),
[requirement matrix](verification.md), and [measurement tables](evidence/growth-table.md).
Generated originals, selected snapshots, packets, rejected candidates and failed
development runs remain outside the checkout in the three declared run roots.

## What was built and tested

The crate has its own Cargo workspace. Exact payload bytes live in immutable
originals; a whole-state JSON manifest records stable identities, placements,
typed relationships, uniqueness labels and a cross-document budget. A selected
publication binds exact predecessor publication identity, candidate state,
readable target and original request/result. Publication lineage differs from
content equality and orphan preparation. Receipts are reconstructible caches.

Under one native lock per store, selection checks positive history for dedup,
cancellation fences and expected publication identity. Speculative candidates
can conflict after completing preparation. The admission-slot policy performs
final preparation and selection under that store's lock. Pure fixture commands
may be explicitly resubmitted against a new base; arbitrary work is not replayed.

A publishes exact state before refresh. Its changing tree has a target/status
marker and whole-file replacements; recovery renders selected state without
mutating it again. A complete prior status lets the normal refresh skip unchanged
payload reads/copies, but state, namespace and navigation work still scale with
collection size. Interrupted refresh falls back to checking desired bytes.
The status marker is diagnostic, not a reader snapshot lock or a detector of
arbitrary external view edits.

B prepares a unique complete generation with documents, attachments, navigation,
ordered assembly inputs/provenance and output. It rereads/hashes destination bytes,
checks exact names and validates the fixture's local link grammar before selection.
A reader chooses one retained path and stays there. Per-open reselection is a
separate control that can mix revisions. A saved assembly packet includes local
relationship/attachment dependencies of its first eight ordered inputs; at 10k+
the overview pulls in its 5,000-constituent closure. That retention cost is counted.

The corrected small suite contains 120 repeated race runs (30 per option/store),
233 fault cases, 480 separately valid semantic-candidate races, two paused copies,
and nine supplemental validation checks. The 51 post-boundary fault rows require
the original accepted publication to survive before any retry; all did. Earlier
preselection rows require old authority, and the selector-visible checkpoint has
its own uncertainty interpretation. Named hooks distinguish state, publication,
selector, manifest, receipt, generation copy and A view-copy phases. Hooks sample
the first matching helper operation, not every individual byte write or nested
directory. Payload short-write injection leaves an actual partial owned stage.

The two stores have independent locks. Holding project publication did not block
research lookup/publication. The cross-store packet records an explicit root
vector/agreement and retained copies; it is not an atomic global transaction.
Import captures exact bytes and a base; later editor changes remain private.
Both JSON envelope and nested duplicate keys are rejected before Value decoding.
LF/CRLF, exact `1.00` JSON text, Unicode/case/reserved-title collisions, portable
paths, a deep attachment, permitted external misses and forbidden local misses
are represented. Portable ASCII slug plus stable ID is presentation; full titles
and exact payloads are retained independently.
Four additional 128-document configurations exercised one 64-document request
against three staggered small requests under both policies/options. All requests
eventually resolved after any explicit pure-command resubmission; this is bounded
progress evidence, not FIFO/starvation freedom. Two combined import-and-move
captures verified that later editor byte changes and filesystem moves do not
alter the admitted bytes, stable identity, selected path or retained old packet.
These small probes ran alongside the large initialization; the host was not
isolated for latency measurement. Their secondary 32 MiB allowance proved low
(43,471,912 allocated file bytes observed); this sub-allocation overrun remains
recorded. The original global data cap and free-space reserve were unchanged.

## Measurements and interpretation

The completed configuration table and every raw request sample are in
[evidence](evidence/raw-index.json). Initialization and publication timings are
engine wall times; raw events also retain parent-observed wall time and metrics.
Sync, read, copy, verification and inclusive preparation/refresh/selection times
overlap and must not be added as independent stage totals. Semantic validation,
lock wait/hold, helper counts, read/write bytes and peak process working set are
recorded. Diff construction, hashing CPU and acknowledgement are not all isolated
as exclusive timers. No p95 is reported for fewer than 20 homogeneous samples;
heterogeneous 1/8/64-document batches are shown separately. A fresh process has
uncontrolled OS/device cache; nothing here is a cold-device benchmark.

Every B size allocation retains initial plus 19/9/2 selected generations when
that configuration completes. Speculative abandoned generations also remain.
One 256 KiB captured attachment is included in the serial 64-document batch,
without adding a publication or changing that batch's document count. The
100k tier has only two later observations per configuration and supports no
meaningful tail estimate. [Cost findings](comparison.md) distinguish reached
workloads from declared allocations and carry the actual cap stop, if any.

Copying is an explicit 64 KiB userspace read/write loop into new regular files,
not `std::fs::copy`, reflinks or filesystem snapshots. Windows allocation uses
FileStandardInfo.AllocationSize for files. Counts include originals, generations,
attempts, manifests, state, publications, receipts and copies; MFT/directory-index
allocation is not attributed by that file counter. NTFS root compression was
reported disabled by `compact`; storage-level dedup was not controlled/qualified.
The earlier run's GetCompressedFileSizeW numbers are superseded and are not
presented as cluster allocation. Free-space checks include all volume use.

## What the native evidence does not establish

Windows uses MoveFileExW WRITE_THROUGH and file sync, with unsupported directory
sync counted explicitly. Its boundary is qualified for process interruption on a
functioning NTFS filesystem. It is not proof that every nested name persists after
OS crash or hardware power loss. The Unix rename/fsync path exists but has no
native Linux execution claim. Process exit/kill leaves OS cache alive; injected
post-sync errors do not simulate power removal. Windows handles without delete
sharing produced a qualified selection refusal/uncertainty followed by retry.

A configured candidate Linux SSH target was discovered; authentication failed
before remote `uname`/`findmnt` could establish its OS/filesystem. Its key first
failed format loading; a temporary normalized copy (same access control, original
unchanged, subsequently removed) loaded but the host still refused authentication.
No OS installation or host reconfiguration was attempted. Explorer was observed
through the Computer Use helper, but address-entry attempts did not navigate to
the fixture. That is an **unmet UI check**, not a successful shell-usability test.
Linux ordinary browser and macOS/APFS/Finder are also unperformed.

All advertised state/packet bases are retained. No TTL, cleanup or GC race is
qualified. Selected metadata/payload damage returns typed integrity errors when
checked; projection lag is separate. Exact state/point reads plus explicit audits
do not imply every operation audits all history. Local hashes cannot prove
freshness after a coherent rollback without an outside witness. Fixture reference
validation is not a general Markdown parser, YAML importer or graph query engine.

## Reproduce the recorded execution

Actual commands, from the repository root on native Windows:

```powershell
$env:CARGO_TARGET_DIR='C:/file-native-experiment-20261017-build'
cargo build --release --locked --manifest-path docs/research/file-native-publication-experiment-2026-10-17/Cargo.toml
$env:PYTHONPYCACHEPREFIX='C:/file-native-experiment-20261017-build/pycache'
python docs/research/file-native-publication-experiment-2026-10-17/run.py correctness
python docs/research/file-native-publication-experiment-2026-10-17/supplement.py
python docs/research/file-native-publication-experiment-2026-10-17/additional.py
python docs/research/file-native-publication-experiment-2026-10-17/run.py growth --tiers 1000 10000 100000
python docs/research/file-native-publication-experiment-2026-10-17/summarize.py
```

These are dated experiment commands, not an unattended future benchmark service.
The driver refuses reused roots and has the recorded fixed deadline. A later run
must declare a new Gate 0 budget/deadline and use fresh roots; do not silently
extend this experiment's caps. Raw JSONL commands can also be replayed individually
through the executable's stdin against new stores. Cargo.lock pins all resolved
dependencies; the exact measured executable and source hashes are retained.
After the 10k busy-refusal observation, actual workload continuation used `--cases`
to finish paired project admission cases at 10k, then attempt those pairs at 100k
before other remaining cases. Resumption uses selected history, preserves all
attempt samples and skips already accepted IDs. The exact cap guard is
[cap-guard.ps1](cap-guard.ps1). Failed and resumed driver phases share the same
append-only growth log; `seq` is local to each driver process, so use line order
as the unique raw-event position. Attempt durations are measured; total request
latency across a controller restart is not reconstructed from missing timestamps.

The original two development runs remain preserved. Review found a partial-final
write hazard, phase classification/import/duplicate-key issues, an incorrect deep
navigation locator, and a concurrent Windows original-install refusal. Corrections
were exercised in fresh v3 fixtures. A separate Python copy-reader slash mismatch
was fixed and the interrupted copy rerun at a new path. Earlier failed/superseded
samples are labeled separately, not pooled into corrected performance estimates.

Both product choices remain open: the usefulness/cost of a mixed or stale current
view versus retained complete generations, desired discovery ergonomics, retention
lifetime, integrity-audit strength and acceptable latency/storage budgets require
human decisions. Faster metadata alone would not eliminate B's measured full-copy
cost or A's retained dependency-packet work. Dolt remains a comparator only.
