# Local checks while the consultation runs

These are coordinator observations and hypotheses, not Pro guidance or an approved execution plan. No engine changes or dispatch are authorized by this consultation.

The final source pin remains `d70f3864f2888d46d9112213b4a03f8d82745a8e` in the isolated integration worktree. The structured data format standard remains proposed; existing owner/version contracts and the user's readable-source choices control.

## Smallest online-cleanup candidate

Current physical activation already drains old operation-wide physical read leases before replacing the catalog. That is conservative and tested. A first candidate can retain this acquisition protocol and the single maintenance owner, but limit exclusion to the durable catalog/journal handoff. The deletion loop must then run with stable references and restart-safe accounting outside the writer lock and exclusive reader gate.

This is a hypothesis requiring proof, not permission to drop locks around the existing loop. The maintenance code currently stores its exclusive lease inside Metrics; ordinary read helpers can capture another physical lease. Lock order and reentrant acquisition need explicit tracing. Maintenance must preserve an active catalog capture for its own reads without retaining global exclusion, retain preparation/lifetime protection where still needed, and prevent another physical replacement from invalidating cleanup's assumptions. Other catalog candidates and archived journals must not become resumable after their dependencies have legitimately been reclaimed.

A deliberately held old reader may defer activation while ordinary agents continue; that can satisfy online cleanup without immediately introducing a catalog-generation lease registry. The experiment must distinguish maintenance deferral from reader/writer outage, prove that new-reader traffic cannot cause an unbounded hidden backlog at the admitted workload, and report the actual activation policy. If activation must complete despite a held old catalog reader, generation-specific protection becomes a separate design requirement with more recovery/accounting state.

## Catalog versus semantic snapshot race

`Store::open` captures physical state before later calls capture SELECTED. The current global read lease makes a physical switch impossible during that interval. A generation-lease redesign must not remove that safety accidentally: an old physical catalog combined with a newer semantic head could ask for a key that was formerly loose and then deleted by newer compaction. Capture/revalidate a compatible pair, or retain all necessary fallback files until relevant older captures are gone. Include a deterministic schedule paused between physical capture and semantic snapshot capture, with intervening commits and compaction.

## Rolling discovery can start with existing permanent links

Publication records already carry a monotonic sequence, predecessor selector, attempt, request, result hash, and membership/outcome roots. `membership_entries` currently enumerates the entire membership map and should not be the recurring tail-discovery API. Publication IDs include process/time components; lexical filename order is not a safe commit watermark.

One candidate is an exact selected-publication cutoff retained in maintenance state. Traverse only the new predecessor suffix from a captured selected head to the previously completed cutoff, verify exact selector/hash/store/sequence continuity, and derive record keys directly from accepted publications. This is bounded by newly accepted work when the cutoff is current. It is not a claim that ordinary history queries may return to linear predecessor scans. A second candidate is a COW-root difference iterator that skips identical subtrees, but there is no need to add that algorithm merely if a bounded tail suffix suffices.

The previous cutoff, its eligibility proof and any referenced object pages must survive GC. Unknown/mismatched cutoff lineage must refuse or use a separately admitted full reconciliation; it must not silently restart a whole-history scan under a claimed incremental budget. Transfer reconciliation, retired content, saved packet ownership, oversize excluded records, partial batches and retry idempotence require source inspection before selecting the watermark semantics.

## Existing COW point-index reuse

`objects::point` already uses `DiskIndex::open/get`; object reads enforce the content hash and read bounds. This makes existing COW page/index reuse a credible lookup candidate alongside an authenticated paged pack index. It is not established that the existing page geometry or branch validation is optimal for these typed physical keys.

There are concrete integration hazards. `objects::object_store` sets Metrics.store_root. Pointing it at another namespace could silently rebind the physical resolver root. `lifetime::collect` traces semantic graph/projection/history roots and then deletes unreachable files under `objects`, `originals`, and `derived`; it does not currently trace physical catalog roots. Reused physical-index pages placed in the ordinary object namespace would therefore need selected, staged, and reader-protected catalog roots included in retention, or an explicitly owned separate namespace with its own reclamation. Hash verification alone does not keep pages alive.

A fallback design that just splits whole indexes into more bounded ranges may be simpler, but needs actual first-lookup byte/work measurements and honest catalog/file-count growth. A warm daemon cache or trusting file mtime/size does not address the authenticated fresh-process requirement.

## Budget and qualification

The prior 40 GiB ceiling is not a fresh grant. Its closeout forecast had only 5,600,492 bytes unassigned before the independent review. A new experiment needs an actual continuing-ownership census, reconciliation of unused versus still-required reservations, and prospective admission before builds or corpus copies. Planning and a small guidance artifact do not establish that a new experiment fits.

Native correctness evidence from the prior pin is a baseline, not qualification of a changed reader/cleanup protocol. The eventual candidate must be tested on native Windows, Linux, and macOS, with appropriately bounded matrices and ordinary shell extraction. A separate WSL run cannot substitute for those native checks. Power interruption remains a separately scoped durability qualification.
