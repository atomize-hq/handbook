# Local reconciliation of Pro guidance

Date: 2026-10-20. Task: scope the next standalone file-native engine experiment before dispatch. Current approach: immutable disk COW core and selected-publication transactions, with immutable physical catalogs and packing. Advisory only; verify against local project truth and authoritative docs; do not reduce scope without user approval.

Pro's final recommendation is KEEP the core, ADJUST the shared physical protocol. The full [answer](answer.md), exact [sent request](request.md), [bounded packet](bounded-review-packet.md), and [reconciled experiment brief](next-experiment-brief.md) are preserved. No implementation or worker dispatch occurred.

## What is carried forward

- One coordinated experiment with reviewable online-cleanup, rolling-tail and fresh-reader checkpoints. All three are required for completion, on one final qualified pin.
- Short physical capture/activation exclusion and generation-specific cross-process pins, with old readers allowed to outlive later activations. Persistent preparations and packets are independent of process pins.
- Conservative retirement across all older generations that can still use a loose fallback. This includes keys absent from an old catalog; waiting only for the previous generation would be unsafe.
- Durable work references survive later physical activations. Cancellation after selection is not rollback. Actual removal, not a journal label, controls space credit. Prevent deletion/path reuse races.
- An authenticated selected cutoff plus durable deferred eligibility. A discovery watermark is not a proof that every previously seen record is physically covered.
- Bounded overlapping segments and complete negative routing, initially at most four relevant segments per key within 64 packs. Existing first-descriptor selection cannot be retained for overlapping ranges.
- A versioned paged reader with full pre-activation verification and a separate complete audit, if whole-index shrinking cannot fit the prospective gate.
- Native final-pin Windows/Linux/macOS tests, ordinary shell extraction, exact outcomes and current source/navigation, compatibility/restore/successor proof, quotas and truthful censors.
- Permanent accepted information and B controls remain the retained-data floor. The 40 GiB ceiling does not permit constant-space promises or silent deletion of protected research.

## Local checks and refinements

1. **Smaller whole-index feasibility is already screened.** The retained final-PZ build receipt contains 62,970,609 index bytes across 16 packs. At the prospective 192 KiB maximum index read for every physical key, existing whole-index authentication plus one index/pack needs at least323 role-partitioned indexes. Extra headers would only increase that floor. This exceeds 64 packs before reserving rolling headroom. Prioritize the paged candidate; do not spend a checkpoint implementing the known loser under this envelope. This is conditional on the proposed gate, not a claim that every relaxed whole-index scheme is impossible. [Receipt](index-geometry-screen.json).

2. **Physical and semantic selection are separate in current source.** Store initialization and packing update STORE.json; semantic head selection reads SELECTED.json. Publication records already carry sequence/predecessor/attempt/result bindings. Use these facts to start the cutoff inspection. Filenames are not ordered by sequence, and membership_entries is a full enumeration. A verified new predecessor suffix can be a simpler candidate than adding a COW diff or new per-commit index; partial progress and deferred holes still need proof.

3. **COW reuse needs retention and namespace work.** Objects::object_store changes Metrics.store_root. Lifetime collection traces semantic roots and sweeps objects/originals/derived, without a physical catalog root. A physical index cannot be dropped into the shared object namespace without selected/staged/reader root retention. Inspect the existing codec/validation for reuse; do not assume that reusing the type alone solves lookup or GC.

4. **The weaker short-drain alternative remains documented but is not the selected proposal.** Releasing the writer/global reader exclusion after a fully drained activation would remove the187-second deletion pause with less new machinery. It still lets a held old operation defer every activation and can starve maintenance under overlapping long operations. The proposed experiment instead tests the stronger property Pro recommends: activate successors while old readers remain valid. Its cost is additional bounded generation/deferred-work state and headroom pressure. If this cost is unacceptable, report the alternative explicitly rather than silently lowering the online gate.

5. **Timing claims need enough samples.** Three rolling cycles are recovery evidence, not a reliable activation p99. Add a separate small activation stress cell with at least 1,000 physical activations, distinguish physical switches from semantic publications, and report observed maxima. Writer cases with smaller sample counts do not receive an unqualified p99 pass. All targets remain prospective and are frozen before optimized evaluation.

6. **Keep the clock fully bounded.** Pro suggested 16 functional hours plus 2 closeout hours beginning after admission/host readiness. The brief uses the same proposed 18-hour ceiling but counts setup/admission/host waits inside the functional window. This prevents an unbounded preflight interval. Nothing has been launched or timed by this planning turn.

7. **The retained long history is an authenticated archive, not an assumed directory.** A read-only geometry probe found the previously disposable unpacked source absent; it was already reclaimed after capture under the earlier workflow. The screen used the preserved final-PZ build receipt instead. Future benchmark derivatives must be explicitly restored/admitted from the correct archive and independently verified. No original was reconstructed or changed in this turn.

8. **Windows guidance checked against a primary source.** Microsoft's [CreateFileW documentation](https://learn.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilew) confirms the relationship between delete access and sharing modes. The new protocol still needs native sharing, cancellation/death and recovery tests; documentation is not qualification. This check adds no power-loss or unsupported directory-sync claim.

## Terms to make explicit when dispatch is authorized

The proposed generation protocol preserves long-held readers and permits new-generation operation; it applies declared resource backpressure when old protections consume headroom. It does not promise unlimited writes under fixed storage with an indefinitely paused reader.

The proposed paged index validates touched routing/record bytes immediately; unrelated damage may be discovered later by full audit. It must fully verify candidates before activation and preserve legacy semantics. This is an explicit experiment contract, not an unannounced equivalence claim.

The existing ceiling and closeout reserve remain controlling. Fresh actual census/admission can reveal an execution blocker. Releasing unused forecasts is distinct from reclaiming bytes. Any protected-data disposal, material allocation increase, maintenance-window product contract, or replacement general database remains a separate decision.

Routine block sizes, batching and implementation decomposition can be selected within the brief. There is no need for another broad architecture interview before reviewing this concrete proposal.
