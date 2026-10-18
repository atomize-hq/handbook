# Comparative findings

This experiment keeps both browsing contracts open. A's mutable current tree can
mix or lag; B pays for a complete retained readable generation that a reader
selects once. The reused COW tree changes metadata storage and point reads; it
does not remove full graph validation, copied payloads, retained packets or B's
directory guarantee. No production backend, browsing contract or format migration
is selected. Dolt remains comparator-only; no external comparator was integrated.

Windows completed all 16 scheduled phases. WSL completed 14 of 16; both 100k A
initializations reached the amended 2,400 s cap after selection, during mutable
view refresh. Both 100k B backends completed their three planned publications,
selected packet assembly and current/initial-retained point probes. A selected
publication after a cap is an authority result, not a completed initialization.

## Native correctness and repairs

Native Windows/MSVC/NTFS and Ubuntu-24.04 WSL2/GNU/ext4 lanes executed publication,
semantic conflict/revalidation, deduplication, cancellation, lost-reply,
independent-store, import, view-damage and retained-lifecycle cases. Full earlier
development suites and final-byte affected-boundary regressions are identified
separately in the [matrix](verification.md) and raw indexes. Their counts must not
be pooled as if every row ran against one executable.

Unix immutable installation now uses atomic no-replace link installation of a
fully written/synced owned stage. Racing same-content winners are verified;
different/corrupt installed content refuses. The same inode survived a retry after
death immediately after name installation. Nested mkdir interruption/retry and
pre-existing parent synchronization ran natively on ext4. Windows retained its
no-replace/write-through installation and actual no-delete-sharing refusal test.

The process oracle forbids selection before replacement, qualifies the visible
incomplete boundary, and requires original accepted results after successful
boundary/death and beneath successors. COW preparation has no backend HEAD:
application SELECTED is the sole authority. Historical reads additionally prove
positive selected lineage; a complete hash-valid orphan publication is refused.

These results qualify cooperating local processes with a functioning filesystem.
Windows directory sync remains unsupported. WSL's native Linux APIs do not
qualify the Windows-host/VHDX persistence stack after power loss. Separate-host
Linux/macOS, OS reboot/power-cut behavior and stronger physical flush guarantees
remain later qualification. GUI browsing is outside the user-selected scope.

## Why the old 100k cases stopped

[Read-only inspection](evidence/old-100k-progress-inspection.json) found 95,584
original directories in the old A store and 94,139 in B, with no attempt directory
or selector in either. Those 600-second attempts expired during common source
creation, before metadata or readable-generation preparation. Directory counts
are progress evidence, not completed document or publication counts.

The preserved Windows whole-manifest profile completed 10k A initialization in
104.8 s with per-original directories and 97.5 s with two-hex-digit shards. Its
sharded read helpers accumulated 66.3 s and sync helpers 13.4 s; these timers
overlap larger phases and cannot be added as exclusive work. Sharding cut created
directories from 10,017 to 273 in that sample. A single uncontrolled-cache pair
does not establish a universal 7% speedup or attribute all timing difference to
directory layout.

The evidence identifies expensive per-source creation, verification reads and
sync/name operations. It does not identify a 10k product capacity limit or a
specific kernel/security-software cause. The new 100k initialization cap was
declared at 1,800 s after this diagnosis, with per-1,000 creation progress and
periodic namespace counts during later phases. Final reached/censored results,
including any accepted A publication with incomplete view, must be read from the
final native evidence rather than inferred from surviving files.
Before any WSL 100k root started, native 10k A init (188.7 s) and full B commits
(approximately 63-66 s) projected beyond the original 1,800/600 s ceilings. The
[pre-launch amendment](evidence/harness-before-wsl-large-budget.json) therefore
sets WSL large init/publication caps to 2,400/900 s, with exact earlier harness
bytes archived. This changes the observation ceiling without reducing work,
changing the Rust engine or waiving disk/worker controls. It is not an overall
deadline or a capacity result.
During WSL's first 100k source creation, the 30k progress trace recorded 337.3 s
in sync helpers and 0.32 s in read helpers, with 180,263 directory syncs and zero
unsupported directory syncs. This is an interim cumulative helper observation,
not exclusive phase accounting or a whole-initialization total. Linux executes
native directory fsync here; the Windows implementation reports it unsupported.
The costs therefore include different platform I/O work, and this evidence does
not isolate a kernel, device, security-software or virtualization cause.

WSL's first 100k manifest/A initialization reached its 2,400 s controller cap
during view refresh. All 100k originals were created, its initial publication
was selected, and about 57,243 view files were present. Subsequent native
read-only inspection and request resolution confirmed the accepted initial
outcome and a pending view marker. The initialization observation remains
censored: accepted authority and incomplete A projection are distinct facts.
The scheduler retained the store and continued the independent B case.

WSL manifest/B completed its 100k initialization in 2,247.5 s, within the amended
2,400 s cap, with a 375.3 MB peak RSS. Source creation accounted for 1,596.0 s;
complete-generation materialization for 644.3 s and destination verification for
3.0 s. Sync helpers accumulated 2,214.9 s across this work. These inclusive
instrumented observations do not isolate an underlying device or virtualization
cause. The accepted result includes the full readable generation.

WSL COW/A also reached the 2,400 s cap during view refresh. Read-only native
inspection and initial-request resolution confirmed accepted selected authority
and a pending marker, with 100k originals and about 55,312 observed view names.
Those names describe namespace progress, not exact completed-document counts;
the pending marker's `completed: 0` is not a per-file progress counter. Both
original censored events remain in the raw log and growth status table.

After the scheduler exited, whole-manifest A recovery completed its selected
100k view in 460.4 s and then completed the two missing publications, retaining
three selected states. The recovery used a measured 895.4 MB additional-file
reserve; original selected authority was unchanged by resume. This is separately
timed post-cap work, not a completed initialization or a sum that can replace the
original censored observation.
The corresponding COW A recovery completed in 442.3 s using a 1,084.1 MB
additional-file reserve, then completed both missing publications. Both stores
ended with three selected publications and complete views bound to their current
head. Their post-cap recovery and follow-up commits ran after the measured matrix,
under the same uncontrolled-cache/shared-host qualifications.
Post-cap WSL A combined one/64-document commits took 43.6/36.1 s for manifest
and 40.2/13.0 s for COW. Separate selected packet observations were 41.0/8.1 s.
The packet copies the same requested closure on both routes; its accumulated
sync helpers were 39.5/6.8 s in those two observations. This large variation in
common copied/synced work is retained rather than attributed to the metadata
adapter. Current positive lookup medians were 156.8/13.7 ms; initial-retained
medians were 176.4/24.6 ms (three fresh processes per binding/key). Ordinary COW
metadata reads were about 85 KB versus the 48 MB manifest, with the integrity
scope and lineage-work distinctions described below.

WSL COW/B completed initialization in 2,306.6 s (327.6 MB maximum RSS), including
1,618.1 s source creation, 648.3 s readable materialization and 2.9 s destination
verification. Its one/64-document combined commits took 656.8/658.3 s; manifest/B
took 657.1/660.9 s. Selected packet assembly took 33.2 s for COW and 32.6 s for
manifest. Both routes still copied a full 100k readable generation per commit.
The similar complete-operation observations do not imply identical metadata
costs, controlled caches or a causal cross-platform performance ranking.

## Commit and assembly are different operations

The final Windows one-document engine observations illustrate the split:

| Documents/store | Backend / browsing | Commit, no new packet | Commit + packet | Packet from selected revision |
| ---: | --- | ---: | ---: | ---: |
| 1,000 | Manifest / A | 76.6 ms | 99.6 ms | 30.2 ms |
| 1,000 | Manifest / B | 1,112 ms | 1,145 ms | See operation table |
| 10,000 | Manifest / A | 486 ms | 5,655 ms | 5,083 ms |
| 10,000 | COW / A | 758 ms | 5,843 ms | 5,220 ms |
| 10,000 | Manifest / B | 12,005 ms | 14,940 ms | See operation table |
| 10,000 | COW / B | 11,925 ms | 11,866 ms | See operation table |
| 100,000 | Manifest / A | Not allocated at this tier | 21,049 ms | 5,290 ms |
| 100,000 | Manifest / B | Not allocated at this tier | 525,022 ms | 6,120 ms |
| 100,000 | COW / A | Not allocated at this tier | 17,404 ms | 6,082 ms |
| 100,000 | COW / B | Not allocated at this tier | 141,303 ms | 8,314 ms |

These are individual observations, not p95s, controlled-cache distributions or
causal estimates of every difference between rows. B copied its complete readable
generation in both packet variants. In particular, its observed packet/no-packet
timing difference cannot all be attributed to packet work: copied source counts
remain the same. At 10k A, requested assembly really does copy the overview's
5,000-constituent local closure; it is not an eight-file metadata operation.

Every requested packet retains eight ordered exact input revisions, all transitive
required local inputs/attachments, recipe/renderer, omissions/errors and output
digest. B retains complete collection closure in every generation. The
packet=false workload records explicitly that no assembly was requested. An
already-selected packet assembly is a separate operation and does not weaken B's
per-publication contract. Packet frequency/closure is a product choice left open.

The full [operation table](evidence/operation-table.md) separates engine timing
from [observed process latency](evidence/process-latency-table.md). Controller
latency includes process startup, response serialization/collection and exit;
engine timing ends before final stdout serialization. Inclusive stages overlap.
Namespace sampling and allocation inventories are reported separately.
An unexpectedly slow read-only PowerShell raw-log formatting probe overlapped
part of the Windows manifest/B 100k one-document successor, after initialization.
The probe was terminated; its exact lifetime/CPU cost was not sampled. That
observation includes any shared-host effect and is not corrected or treated as a
controlled benchmark. Other measured engine lanes did not run concurrently.
Read-only native inspection/resolution of the capped WSL manifest/A authority
also overlapped the next B case's source creation. Its exact native intervals
remain in the diagnostic raw log. Source/reporting/review reads and ordinary
host activity were not isolated or controlled. Serial benchmark scheduling does
not establish a quiet host, controlled caches or zero diagnostic interference.
Read-only [process samples](evidence/windows-100k-B-verification-process.jsonl)
show 211,931,136 additional read-transfer bytes over 65.5 s during that successor's
destination verification, with no additional payload writes. These Windows OS
process counters establish continuing read progress; they are not physical device
bytes, exclusive application counters or an explanation of the underlying delay.
The [batch table](evidence/batch-table.md) retains all allocated 1/8/64-document
observations rather than extrapolating from the one-document examples.
Changed IDs are contiguous from `d000000`, including the overview. At 10k+,
even the one-document update changes its large metadata indirection. These
observations do not establish costs for scattered-key updates.

## Whole manifest versus real lazy COW

The disk tree was reused from the local October 12 experiment. Ordinary document
metadata lives in leaves; the large overview metadata is an immutable indirect
object. A small descriptor binds the prepared root, consumer, budget and count.
Updates coalesce changed tree paths; deleted keys are tombstones. This is local
experimental code, not EchoDB source, a Dolt port or a new prolly implementation.

At 1k A, one-document metadata writes fell from 469,026 to 81,631 bytes, while the
complete no-packet commit observation rose from 76.6 to 107.7 ms. Positive lookup
engine medians across three fresh processes were 3.13 ms for manifest and 2.29 ms
for COW. Those component gains did not make complete commits uniformly faster.

The [amplification table](evidence/amplification-table.md),
[lookup table](evidence/lookup-table.md) and
[process lookup table](evidence/process-lookup-table.md) report the actual larger
and historical cases. COW point reads verify a descriptor/path plus any requested
indirect metadata. They return document metadata, not a full payload download or
an audit of unrelated pages. The manifest implementation reads its complete state
file and validates the graph. Commits on both routes still load/validate the full
graph and produce the requested readable work. COW's numeric byte-array encoding
and full-state diff/validation work can increase metadata read/CPU cost.
The `metadata-build` timer includes immutable node installation for COW but only
serialization for manifest; the caller subsequently installs the full state or
small descriptor. Those stage times do not measure identical complete-I/O scopes.
Whole-operation times and the declared byte counters retain that work on both
routes.
Timed positive lookups use ordinary metadata key `d000001`. The negative key
`not-present` is outside the existing `d*` range, so its times do not describe an
in-range-miss distribution. Large overview indirections are checked separately
as functional point-read cases.

Positive historical selection also walks publication history. That work appears
in total wall/application I/O, but precedes the point-inclusive stage timer. Large
initial changed-ID/result records can therefore matter even when tree point reads
are small. Fresh-process does not mean device-cold. Full audits detect an unrelated
damaged actual COW page that a valid point path need not read; these operations
have deliberately different integrity scopes.

The Windows manifest/A 100k case completed initialization in 1,430.4 s with a
308.9 MB peak working set, then retained three selected states and completed
current/initial positive and negative lookups. Its 64-document combined commit
took 9.1 s, less than the one-document observation's 21.0 s. This reversal is a
reason to retain the raw individual observations and uncontrolled-cache
qualification, rather than fit a per-document latency model. The large tier
allocates combined 1/64-document commits; packet-free commits are measured at
the small and medium tiers.
The Windows manifest/B 100k initialization completed in 890.8 s, including
destination verification of a complete readable generation, before selection.
Its first one-document successor completed in 525.0 s: materialization accounted
for 131.2 s and destination verification for 378.9 s. Initial destination
verification had taken 10.7 s. This large variation in a common full-directory
operation is retained explicitly; changed-document count alone cannot explain
the observed end-to-end time, and these samples do not isolate a cache/OS cause.
The 64-document successor completed in 503.2 s, with 477.1 s materialization and
10.8 s destination verification. Both copied all 100k readable documents. All
three planned manifest/B large publications were retained; packet assembly from
the selected revision then completed in 6.1 s. The change in which phase dominated
does not justify summing worst phases from different samples into an invented
latency bound.
Windows COW/A completed 100k initialization in 1,260.3 s and both combined
updates (17.4 s for one document, 11.8 s for 64), retaining three publications.
Its one-document commit wrote 380,390 metadata bytes but read 512,383,092 metadata
bytes across full-state work. The compact changed-page write is real; the graph
and readable-work costs remain real too. Initial peak working set was 259.0 MB;
the one-document commit peaked at 311.3 MB.
For this 100k A update, whole-manifest metadata writes were 48,041,926 bytes,
about 126 times the COW observation. Current positive lookup engine medians were
181.4 ms versus 16.2 ms; initial-retained medians were 194.2 ms versus 29.7 ms.
Each median has three fresh-process observations with uncontrolled caches. The
initial COW metadata write was 128.1 MB, compared with 48.0 MB for the manifest;
its numeric-byte-array page encoding has an up-front cost. Storage sharing must
therefore be read with the actual 20/10/3 retained-history depths, not assumed to
win equally at every tier or lifetime.
The three-publication COW/A store held 2,945,508,341 logical bytes, versus
2,960,799,131 for manifest/A (about 0.5% less). COW/B initialization took
1,015.6 s; its one/64-document combined commits took 141.3/141.2 s, each copying
all 100k readable documents, with approximately 113 s materialization and 11 s
destination verification. The much longer manifest/B verification observation
is a visible shared-I/O variation, not evidence that changing the metadata tree
caused that verification speedup. Selected COW/B packet assembly took 8.3 s.

The retained Windows A store totals show how much sits outside changed metadata:

| Documents | Selected publications | Manifest logical bytes | COW logical bytes |
| ---: | ---: | ---: | ---: |
| 1,000 | 20 | 37,591,345 | 31,029,060 |
| 10,000 | 10 | 635,192,161 | 597,866,735 |
| 100,000 | 3 | 2,960,799,131 | 2,945,508,341 |

These totals include payloads, views, packets and control files. Both collection
size and history depth change between rows; this is not a controlled retention
curve or an extrapolation to a long-lived production store.

## Contention and progress

The native lanes compare speculative final preparation and per-store admission in
bounded 1/2/4/8-process waves, with a 64-document request and one-document peers on
128-document fixtures. Speculation overlaps at a complete-candidate barrier;
admission keeps private thinking/capture outside coordination and serializes final
work. These are controlled contention cohorts, not a natural traffic/fairness
distribution or a large-store sustained load test.

Windows speculative waves required 1/3/7 pure retries at widths 2/4/8; admission
required none in those waves. Rejected candidates and their copied readable work
remain in the accounting. Application I/O on rejected attempts is distinguished
from unique retained orphan bytes: reusable payloads/pages are not automatically
all wasted. B's full copies make speculation especially visible.
The [storage table](evidence/storage-table.md) reports actual retained growth
files and directories alongside logical/file-allocation bytes; detailed native
accounting splits authoritative payloads, COW pages, views, generations, packets,
navigation, provenance and control records. Directory/MFT allocation is not
attributed. A stale sealed candidate is not automatically safe to collect.

Dedicated collectors record each process's actual observed endpoint. Request
latency includes all attempts, controller scheduling, explicit rebasing and pure
retries. Busy time is recorded on timeout, waiting cancellation and successful
acquisition. The diagnostic busy ceiling was explicitly 80 ms; default wait is
15 s. Killing a holder releases OS coordination. Signals are not terminal fences;
the later coordinated cancellation installs the terminal fence. Accepted same
intent resolves before a late signal or a now-invalid duplicate deletion.

No callback/external side effect is automatically replayed. Lock acquisition is
not FIFO, and bounded successful completion does not prove starvation freedom.
Larger-store serial costs inform future allocation; they are not substituted for
unperformed large-store contention observations.
Across all four platform/backend lanes, 240 observed requests finished within
the planned 60 s small-request observation budget; the maximum was 2.958 s,
including pure retries. Each observed request used at most two attempts. The
controller permits three retries after the initial attempt (four total), and
the 60 s aggregate budget has no separate watchdog. These measured endpoints
therefore qualify the executed cohorts, not a universally enforced request SLA.
The eight-worker ceiling also does not bound retained failed artifacts. Repeated
speculative cohorts accumulate 11 stale rejected candidates per store; four old
ones plus eight new candidates imply 12 unselected sealed artifacts at the final
barrier. They remain protected until explicit abandonment. A strict quota over
all retained candidates was not implemented; the allocation sheet qualifies its
original eight-candidate wording accordingly.

## Retention, ordinary readers and damaged views

Separate disposable fixtures exercise pin registration versus retirement,
preparation versus collection, actual suspended ordinary copies, packet release,
explicit retirement, interrupted sweep and reopen. Pins cannot silently rebind
names. A paused preparation retains a shared gate; collection refuses rather than
inferring death. Actual obsolete COW pages were reclaimed while shared/protected
pages remained. Publication/outcome/fence history is retained in this bounded
proof, independent of old snapshot retirement; no outcome horizon is selected.

Retirement uses a positive per-attempt tombstone, distinct from missing/corrupt
snapshot evidence. The tombstone gates reads even if process death prevents the
publication-side discoverability cache. GC deletes only canonically checked files
under owned disposable roots. It leaves directories, emitted packet copies and
outcome records conservatively. It is not a production collector, automatic TTL
policy or secure-erasure claim. Tool-free readers rely on an explicit retained
directory lifetime; they do not invisibly register pins. Cross-store copies and
their agreement/vector stay independently usable without global atomicity.

PowerShell/Bash walkthroughs start from files/instructions, positively bind a
selected publication, inspect named sources/navigation/attachments, copy retained
closure and verify its selected manifest/output. Actual suspended enumeration/
copy sees permitted mixed A content and coherent B fixed-path content. Exact
authored CRLF, JSON 1.00, Unicode/reserved titles and stable IDs survive. The fixture
link grammar is narrower than arbitrary Markdown/YAML; incompatible cross-depth
rename refuses rather than rewriting source bytes.
The sources are retained as fixture UTF-8 bytes. Growth JSON edits use whitespace;
generic edits do not enforce YAML/JSON/document schema validity. The graph
constraints are experimental metadata constraints, not a general source parser.

View audit detects missing/edited authored and derived projection files after a
complete marker. Rebuild preserves divergent bytes before replacement, including
navigation and packet outputs, and resumes after death immediately following
preservation. Immutable authority remains unchanged. A corrupt non-retired prior
view source blocks rebuilding with an integrity error; current-source-only repair
is not guaranteed for retained corruption.

## Decisions and hardening left open

| Question | Evidence supports | Still open |
| --- | --- | --- |
| A current tree | Useful cheap incremental ordinary browsing with explicit lag/mixing and repair | Whether that ordinary-read contract fits each consumer |
| B retained generations | Coherent selected-once terminal reads/copies with complete local closure | Per-publication copy, namespace and lifetime budgets |
| COW metadata | Real changed-path/point-read sharing and exercised root/page lifetime | End-to-end value after full graph/control-plane work; encoding and validation costs |
| Context packets | Exact source/provenance survives independent extraction | Frequency, closure policy and which operations may defer assembly |
| Coordination | Admission avoids speculative copies; speculation can prepare outside the slot | Latency budgets, larger sustained contention and fairness requirements |
| Retention | Explicit lifetimes and conservative actual reclamation work in the fixture | Production policy, durable pin ownership, horizons and long-history indexing |
| Database comparator | None was needed to execute the primary file/COW comparison | Optional future comparator only for a concrete unresolved requirement |

Before production: qualify native root/lock identities and hostile namespace/editor
races; define owner/domain schemas and exact source/import contracts; validate
platform persistence beyond process death; decide lifetime/audit policy; and test
large histories. The prototype's lineage walk is bounded (1,000 publications),
whereas executed history tiers are 20/10/3. No long-lived production owner or
automatic migration is approved by these measurements.
