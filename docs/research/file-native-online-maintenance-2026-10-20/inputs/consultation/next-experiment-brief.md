# Bounded experiment: online physical maintenance and authenticated lookup

Status: proposed for human review after the October 20 Pro consultation. No implementation or dispatch has occurred. Advisory only; verify against local project truth and authoritative docs; do not reduce scope without user approval.

## Deliverable, outcome, and decision

Deliver one private-engine experiment with separately reviewable checkpoints for online cleanup, rolling eligible-history compaction, and fresh-process lookup. Its outcome is a demonstrated operating envelope for concurrent local agents, with retained outcomes and readable documents intact. The next decision is whether to dispatch this brief, including its explicit integrity-scope and resource terms. Passing one checkpoint does not complete the experiment.

Keep the immutable disk COW core and whole-publication transaction conflict rule. Introduce a narrow internal physical-store boundary: captured catalog handle, authenticated record lookup, expected-parent physical activation, and resumable reclamation/status. Do not freeze a public API or introduce a plugin framework, daemon, or scheduler.

Baseline core: `d70f3864f2888d46d9112213b4a03f8d82745a8e`. Integrated evidence: `a0b92fe5dc64269508601dcddf7e2e2ca477e30d`, branch `codex/file-native-qnp-20261019`. Preserve both and the authenticated operational archives. Some original disposable source copies have already been reclaimed after verified capture; use the retained archive/inventory, not an assumed still-present directory.

## Selected proposal and its tradeoffs

1. Use scoped cross-process physical-generation pins with a short capture/activation gate. New catalog activation must be able to proceed while a reader of an older generation remains open. That reader can postpone dependent reclamation and consume reserved headroom; it cannot be revoked to make a test pass. Explicit quota backpressure is allowed at the declared resource boundary. Unlimited publication progress behind indefinitely held readers is not promised.
2. Discover newly accepted history from authenticated semantic authority, and retain a durable bounded set of older deferred eligibility. Discovery progress and physical coverage are distinct. A record becoming eligible after its initial discovery must not be lost behind a watermark.
3. Start with bounded overlapping segments and disjoint ranges within each segment, rather than immediately adding a second general key-to-location database. Proposed limit: four relevant segments per key, within the existing 64-pack catalog cap. Prove negative physical lookup across every relevant segment. Consolidate or apply explicit admission backpressure before limits are exceeded.
4. Prefer a private versioned paged authenticated index inside a small number of containers. The local screen below eliminates simple whole-index shrinking under the proposed metadata envelope. Reuse suitable existing COW validation/decoding patterns where they actually simplify this implementation; do not assume a process-local epoch/refcount crate solves cross-process lifetime or restart.
5. A paged reader authenticates the complete required route and record; it may not discover unrelated corrupted pages until an explicit full audit. Candidate construction and activation admission require complete structural verification. Preserve the current legacy-format verification behavior. This changed eager-corruption-detection scope must be documented and tested rather than described as equivalent full-index verification.

Growth means bounded obsolete representation, temporary coexistence, and backlog relative to permanent accepted history and retained live content. Permanent accepted outcomes and B publication bindings still grow with accepted history. Report that retained-data floor explicitly.

## Checkpoint 0: source inspection, admission, and frozen controls

Before edits/builds/corpus copies, map all physical readers, lock acquisition and drop/cancel paths, persistent owners, physical-versus-semantic selection, native sharing flags, recovery/journal interpretation, path reuse, and direct-path consumers. Use the actual pinned source. GitNexus impact/detect rules apply to future edits/commits; if the isolated prototype is unindexed, preserve UNKNOWN and manual tracing. Do not run a package-manager indexing bootstrap that creates another large cache.

In particular:

- `Store::open` captures physical state before later semantic head reads. Include that interval in the protection proof.
- `STORE.json` holds configuration/physical catalog selection, while semantic selection uses `SELECTED.json`. Revalidate the physical comparison token without rejecting every concurrent semantic publication merely because the semantic head advanced beyond the captured cutoff.
- Existing publications have sequence, predecessor selector, attempt, request and result hash. Publication filenames do not sort in selected order. Prefer a bounded verified new predecessor suffix to a retained selected cutoff if sufficient; otherwise justify a structural COW diff or additional index. Never disguise a full predecessor walk as incremental discovery or use this maintenance cursor for ordinary outcome queries.
- Partial traversal needs a recoverable interval/cursor; do not advance a completed cutoff over undiscovered holes. Preserve the cutoff/cursor roots from GC. Active transfers and saved-packet owners need explicit inspection.
- Current `membership_entries()` enumerates all history, and `cohort()` also enumerates attempt files. Neither should remain the routine small-tail discovery path.
- Existing COW object helpers rebind Metrics.store_root, and semantic GC does not trace new physical index roots. Reuse needs correct namespace handling and selected/staged/reader-protected physical retention.

Perform a fresh cross-host ownership census and prospective admission. Keep 40 GiB / 5,000,000 files combined, including 1 GiB / 100,000 files reserved inside for closeout, prior protected originals/evidence, builds, shared-Git growth, derivatives, temporary coexistence and still-valid reservations. The old almost-full forecast is not a fresh measurement or allocation. No silent disposal of protected fixtures, sole backups, or rejected cleanup targets. Release only demonstrably unused forecasts, without claiming that actual bytes were reclaimed.

Require native host readiness and frozen per-cell watchdogs. Do not start executable work that cannot fit. Insufficient admission produces a concrete preflight report rather than an expanded allocation or a silently reduced matrix.

Freeze the baseline, proposed per-platform gates, corpus definitions and source bindings before optimized measurements. Separate capture/catalog checks, metadata/stat calls, opens, index hashing/parsing, allocation, record decode, and total spawned-process time. Instrumenting the baseline must preserve its behavior and remain distinguishable from the original frozen executable.

## Checkpoint 1: online cleanup with the existing index reader

Implement the capture/pin and retirement protocol before changing index lookup costs.

Reader: acquire a short shared capture gate; read a bounded valid physical binding; acquire and validate its generation pin; release the gate; perform normal I/O and semantic operations; release the pin after all dependent I/O ends. Preserve separate persistent preparation/snapshot/packet owners. Do not hold the capture gate across full index authentication or waits for the writer.

Activation: prepare/verify outside global exclusion; durably identify candidate ownership and its transition; briefly take exclusive capture access and try the writer lock. If the writer is unavailable, release the capture gate and retry outside it. Revalidate expected physical parent and bounded cutoff/admission facts; atomically select the new physical generation and seal the old generation against new acquisition; release global exclusion. Trace nested/internal Metrics captures to prove this is deadlock-free.

Cleanup: use bounded resumable batches outside writer/capture exclusion. Check all generations that could still access a loose fallback, including generations in which the record was unmapped. Packs shared by a current catalog, old reader or builder remain protected. Carry unfinished retirement and deferred work into successor catalogs; a second activation must not orphan it. Prevent a retired physical pathname from being re-adopted while deletion remains possible, using generation-scoped instances or an equivalent proven claim protocol. Bound and eventually reclaim generation/journal bookkeeping.

Windows sharing failures are deferred, with zero deletion credit until actual removal. Test release/retry. Microsoft documents that incompatible sharing modes prevent delete access: [CreateFileW](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilew). Native file-operation behavior must also be verified by the tests; no new power-loss claim follows.

Mandatory schedules:

- Hold an old reader beyond 15 seconds while new readers, publishers, and multiple catalog activations progress; later release it and reclaim safely.
- Pause between physical capture and semantic head capture; publish and compact intervening history; resume without losing loose fallback access.
- Retain a reader through two successor generations. Checking only the immediate predecessor must not permit deletion.
- Pause deletion, prepare a candidate that would reuse the same content-addressed pathname, and prove activation/deletion cannot race into removal of a live instance.
- Interrupt before/after catalog selection, before journal advancement, during loose deletion, and after an actual old-container removal. Selected authority governs recovery; stale journal text does not undo activation.
- Reader cancellation/death, maintainer death, two maintenance contenders, saved packets, active preparation, retained snapshots, retired-content refusal, and Windows no-delete-sharing deferral.

Completion: unchanged exact semantic results and readable content, ordinary agents progress during old-generation retention and deletion, bounded resumable cleanup and truthful accounting, all schedules covered. Maintenance may defer for resource protection; that is separately counted from frontend outages.

## Checkpoint 2: rolling tail with explicit coverage and bounded routing

Capture an authenticated selected cutoff and physical parent. Derive eligible permanent publication/result/hint keys from newly accepted history. Include finalized candidate/allocation records only under their existing ownership rules. Preserve deferred eligibility for records that cannot yet be packed. A queue or directory entry never establishes acceptance.

Select physical mapping, discovered cutoff/progress and deferred-work identity together. Writers beyond the cutoff remain readable loose tail or later segments and do not invalidate an otherwise eligible physical candidate. Later retry must recheck deferred records and absorb them when eligible.

Support at most four relevant chronological segments per key, with internally disjoint ranges, or document a simpler equivalent that meets the same bounds before implementation. Existing first-matching-descriptor routing is insufficient for overlapping segments. Prove both mapped hits and complete relevant-segment misses; conflicting raw identities refuse. Consolidation must fit pack/descriptor limits, bounded bookkeeping and temporary-space admission. Charge collateral rewriting of older records.

Run at least three real publish/compact/reopen cycles. Test an additional tail that forces the segment boundary, using a reduced limit in small fixtures when useful. Include old deferred records becoming eligible, oversize exclusions, backlog/cancellation, stale candidates, cutoff crossings, partial batches, retirement and packet protections, small quota refusal followed by recovery, and idempotent retry.

Completion: all eligible records through the declared cutoff are packed or explicitly deferred; no hidden full-history body/attempt-directory walk for a small delta; lookup/accepted outcomes survive every cycle; after producers stop and blockers release the unprotected backlog can drain within the frozen admitted work budget.

## Checkpoint 3: fresh-process authenticated lookup

Keep the original relevant-whole-index reader as the baseline. Evaluate at most two credible smaller designs, including feasibility screens. Do not implement a provably infeasible control just to fill a comparison table.

The local final-PZ catalog screen measured 62,970,609 index bytes. Keeping every whole authenticated index at or below 192 KiB would require at least 323 role-partitioned indexes even before extra headers, exceeding 64 packs under the current one-index-per-pack format. This rejects simple shrinking under that prospective gate, not under every conceivable relaxed gate. See [the screen](index-geometry-screen.json). Prioritize the paged candidate; any changed alternative envelope must be declared before candidate evaluation, not after a disappointing result.

For the paged candidate, authenticate the root, routing pages, bounded leaf ranges and record identities. A negative lookup must authenticate ordering/boundaries and absence in every relevant segment. Full validation before selection and the independent full verifier must check global ordering, coverage and cross-page boundaries. Touched metadata corruption refuses; mapped corruption must not fall back to a valid loose duplicate. Separately demonstrate that untouched corruption can remain undiscovered by a point query but is found by the full audit.

Use an explicit private format version and tested legacy read/restore/refusal behavior. Do not infer safety for mixed-version live clients; address initial activation and old-operation quiescence explicitly. Preserve exact record/payload bytes and semantic identities. Do not replace correctness with mtime/size trust, a warm daemon, mmap, or a verification cache that merely hides a full scan at startup.

Completion: lower measured first-operation work and latency, complete positive/negative integrity proofs, bounded cache/transient allocation and startup work, compatibility and full verification, with all new metadata/files included in storage accounting.

## Discriminating matrix

N denotes documents; H denotes actual accepted durable publications. Label synthetic structural tests separately.

| Cell | Required evidence |
|---|---|
| Small protocol | A/B around N64/H64; three cycles adding at least 16 accepted publications; complete adversarial schedules. |
| History and concurrency | A at fixed N128, H about 1,000 and 10,000; three cycles adding 100 accepted publications each; 1 and 4 publisher configurations, four reader processes, a held old reader. Fixed-delta discovery work is compared across H. |
| Document/view control | A/B N64 and N1k, shallow fixed history; matched payload/title operations with and without maintenance on one primary platform. This does not replace the deferred N10k-B navigation matrix. |
| Lookup | H1k, H10k, and the exact retained long-history cohort around H101007; matched copies/derived representations of the same accepted records and original request categories. Use authenticated admitted derivatives where writes are required; never open a protected source through a mutating recovery path. |
| Pressure | Small authentic stores with reduced quota/segment limits; stale candidates, deferred eligibility, old readers and Windows sharing blockers; demonstrate eventual recovery after the blocker ends. |
| Activation stress | At least 1,000 small real physical activations for an exploratory p99 and observed maximum, separate from the three end-to-end rolling cycles. Keep semantic publication counts distinct from physical activations. |

Use PZ for integrated maintenance, with a small matched P0 format/correctness control. Do not repeat the broad compression study. Retain six lookup categories: early, middle, recent, missing, cancelled, lost reply. Minimum 20 fresh processes/category and 500 warm queries/category; 50 fresh processes if admitted. Report all fresh samples and maxima; do not infer p99 reliability from 20/50 starts. Report engine operation and total process latency separately and preserve uncontrolled OS/device-cache labels.

Windows and native Linux provide primary correctness/availability evidence. The same final pin must pass native macOS recovery, compatibility and focused generation/file-lifetime checks. Run the applicable existing workspace, strict-input, packing and recovery capsules on that final pin. Use ordinary PowerShell/Bash extraction, including an old retained B selector captured once. WSL and GUI browsing are not substitutes for these native tests. A large performance matrix need not be repeated on all hosts.

## Acceptance and reporting

Hard gates: exact accepted outcomes/request-intent/dedup/lost replies; unchanged semantic authority and conflict rules under physical-only work; protected lifetimes and retired-content refusal; exact payload/graph/navigation/provenance and both browsing contracts; required-path corruption refusal and valid negative proofs; idempotent recovery with no stale-candidate damage; legacy compatibility, complete restored verification and an accepted successor.

Full independent oracles run after critical crashes on small fixtures, after scaled rolling-cycle boundaries, and after final restore. Verify the complete chosen long-history representation, not only sampled outcomes. Preserve original observed failures and censors.

Prospective performance/work targets, frozen per platform during baseline calibration before optimized runs:

| Area | Starting criterion |
|---|---|
| Global exclusion | Zero historical enumeration, full-index hashing/parsing, duplicate verification, rewriting or deletion while holding writer/global capture exclusion. Bounded binding/cutoff checks must be itemized. |
| Activation | Initial p95 ≤10 ms, exploratory p99 ≤25 ms, observed maximum ≤100 ms; retain every sample. Hardware calibration changes must be explicit before evaluation. |
| Ordinary availability | Zero maintenance-induced frontend timeout/refusal at admitted load. Distinguish semantic conflicts, maintenance-contender refusal and intentional resource backpressure. |
| Ordinary latency | Initial p95 ≤1.25× matched no-maintenance control; exploratory p99 ≤1.5× only with adequate samples. At smaller writer sample counts report p95/max and descriptive quantiles rather than declaring a p99 pass. |
| Cleanup | Up to 1,000 candidates or 50 ms cooperative work per batch; stop/check between native calls. Report a single blocking native call exceeding the target rather than pretending it is preemptible. |
| Discovery | No reopening unchanged historical publication bodies except explicit deferred work. At fixed delta, H10k work target ≤2× H1k, with metadata page visits reported separately. |
| Tail drainage | After producers stop and blockers release, initial target zero unprotected backlog within two admitted maintenance passes. All deferred/excluded items remain accounted. |
| Lookup | ≤64 KiB startup/routing plus ≤192 KiB index metadata per physical-key lookup within the admitted height/segment envelope; also report complete-operation totals, metadata/stat calls and allocation. |
| Fresh latency | At least 2× lower long-history engine-operation median for affected categories, with no unexplained category regression. |
| Warm latency | Regression tolerance: the larger of 20% and 0.2 ms. |
| Bounds | Retain the 64 MiB index cache/transient bound, 64-pack catalog and existing record/codec limits. Initial maintenance-bookkeeping target ≤64 MiB, streaming work; preserve existing stricter reader/checker/aggregate RSS and disk-free bounds. |

No scan may migrate into catalog reconstruction, retirement replay or cache initialization to make lookup counters look better. Latency targets are experimental, not production SLAs. An uncalibrated or missed target remains visible.

New automatic-packing policy to test: at least 128 removable loose records, at least four removable files per added representation file, and positive forecast benefit including metadata/coexistence. Forced smaller correctness fixtures are labeled. At H≥1k test a new ≥95% replaceable-representation file reduction, while also reporting the unchanged original whole-cohort denominator and B's required controls. The former A60.71% / B32.14% failures remain failures under their original rule.

Count the retained-data floor, packed/loose live records, obsolete bytes, backlog, all catalogs/indexes/generation locks/journals/deferred state, stale candidates, directory/file counts, native allocation where available, successful deletion credit, and coexistence headroom separately. Process death or planned deletion is not automatic space credit.

## Clock, scope, and final decision

Pro proposed 16 functional hours plus two protected closeout hours. Use that as the proposed maximum wall-clock budget, not an instruction to consume it. Local adjustment: count preflight/setup/host waits inside the 16-hour functional window from execution dispatch; do not create an unbounded admission period before the clock. No clock starts with this planning document. Establish fixed per-stage watchdogs and an early preflight checkpoint; if admission or host readiness cannot be met, close with named blockers before growing artifacts. Never silently extend the ceiling or restart a censored cell.

At closeout deliver the exact final pin, narrow changes, protocol and lock/recovery map, native evidence, complete byte/outcome oracles, raw measurements/counters, resource accounting, preserved negative results and a direct KEEP/ADJUST/PIVOT verdict. Any required missing native, history or recovery evidence means incomplete, even if a faster reader exists.

Non-goals: navigation range optimization or N10k-B completion; source-format migration; automatic merging; consensus/distribution; daemon/scheduling; broad dependency research; replacing the engine with Dolt; public format/API freeze; device-cold/power-loss qualification. A mandatory maintenance-window contract or a new general locator database is a subsequent explicit decision if this bounded protocol fails, not a way to mark the requested online outcome passed.
