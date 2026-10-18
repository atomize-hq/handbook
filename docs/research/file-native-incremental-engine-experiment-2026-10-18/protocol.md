# Private experimental protocol I1

Authorized standalone research, 2026-10-18; baseline 379af4e. No production format,
API, A/B selection or migration. Exact authored bytes, lowercase path comparison,
case-sensitive label comparison and duplicate-edge multiplicity are preserved.
Inputs/ contains byte-preserved dispatch documents. Existing contracts control.

## Authority and checked base

SELECTED.json binds an immutable publication by exact ID and SHA-256. Each
publication directly represents itself. Its immutable membership/outcome trees
cover exactly its selected predecessors; the successor inserts its previously
selected head, never itself. Original result bodies live separately from point
index entries. Directories, receipts, candidates and orphan publications confer
no selection authority. Missing/corrupt required evidence is integrity failure.

Fresh stores use I1 descriptors and bounded compact COW pages C1. Baseline JSON
pages remain an encoding control. Exact sources are separate immutable files.
One checked page decoder supplies point/range/full/diff/audit/GC traversal.
File length and encoded lengths are bounded before allocation. Tombstones count
as physical keys. Leaf capacity 64/fanout 32 are unchanged; byte splits may differ,
so results describe encoding plus packing and report height/pages/occupancy.

Checked semantic roots contain documents, independently addressable ordered
edges, lowercase paths, unchanged labels, reverse edges with ordinal multiplicity,
exact cost/count and projections. Initialization and explicit audit compare every
logical entry against full recomputation. Incremental preservation assumes a
checked base and does not audit unrelated corruption on every transaction.

| Rule | Dependency/work | Verification cases |
| --- | --- | --- |
| IDs, paths, kinds, exact source hashes | changed headers and captured bytes | identity, import-bytes, kind-change |
| Unique paths/labels | remove all old contributions, then final occupants | swap, duplicate-final, label-case |
| Targets and deletion | changed edges + incoming range against final overlay | duplicate-edge, delete-with-removals, delete-add |
| Kind rules (I1 extension) | source kind/outgoing or target kind/incoming | kind-change, incoming-5000 |
| Budget/overflow | checked old minus removed plus final contributions | final-budget-swap, overflow, skew |
| Opaque global adapter | explicit full validation fallback | fallback |
| Derived navigation | changed source and sources referring to renamed target | nav-dependency, skipped-refresh |

Operations are pure captured fixture operations, with final-batch validation.
I1 adds exact replacement, edge removal by ordinal, kind changes and explicit
maintenance/checkpoint operations. No automatic merge or read-set OCC.

## Private library boundary

Store owns identity/coordination/admission/lifecycle; Snapshot binds exact selected
state; Transaction binds base and pure inputs; Prepared has private construction;
typed outcomes distinguish accepted, absent-as-of and terminal cancellation.
Reopened preparation replays captured operations from its exact checked base,
derives its own affected set and compares roots/versions/evidence. B destination
verification is required before core selection. Two separately compiled small
consumers use the shared core. No general plugin ABI/public format promise.

## Roots and finite transitions

| Root | Lifetime / release |
| --- | --- |
| Selected membership/outcomes, original results/publications | permanent accepted evidence; never traverse provenance into old content for GC |
| Selected documents/constraints/projection | until replaced, then ordinary retained snapshot policy |
| Non-retired historical content | until explicit retirement; pins/readers/packets block retirement |
| Reserved/sealed candidate and private stages | charged and protected until selection transfer or fenced abandonment plus deletion/GC |
| Prepared checkpoint | reservation protects rebuilt roots; expected-head successor installs verified source/coverage/version binding |
| Pending A plan | target and prior roots protected through completion; stable divergence backups retained |
| Old history index versions | protect for live candidates/checkpoints; current roots cover permanent evidence |

| Transition | Boundary case names |
| --- | --- |
| Reserve before allocating; pessimistic charge survives death | reservation-durable, allocation, seal |
| Prepare roots, capture pure request, seal | semantic-root, projection-root, candidate-ready, replay-tamper |
| Expected-head gate and predecessor established | stale, predecessor-established |
| Install history roots and publication without selecting | membership-root, outcome-root, publication-installed, orphan |
| Atomic selector replacement and required sync | selector-staged, selector-visible (uncertain), selection-boundary (accepted) |
| Transfer reservation after selection | accounting-transfer, accounting-release |
| Checkpoint rebuild/verify/install | checkpoint-built, checkpoint-verified, checkpoint-stale |
| Persist A plan before edits; finish pending before advancing | refresh-plan, refresh-preserved, refresh-file, refresh-complete, reversal |
| Pin/register/release and retire | pin-registered, pin-release, retirement-durable |
| Fence abandonment, sweep then release charge | abandonment-fenced, reclaim-file, reclaim-accounting |
| Cancel under coordination, accepted dedup first | cancel-durable, admission-full-retry |

All new named boundaries receive finite process exits/recovery assertions.
Before selector replacement selection is forbidden; visible/incomplete boundary
stays uncertain; completed required boundary must retain original acceptance
beneath successors. Later recovery never relabels the interrupted event.

A is exact immutable state plus mutable readable files which can mix/lag.
Persist target-bound desired/prior path actions before edits; reconcile one final
action per path, preserving divergent source/derived bytes under stable identities.
Finish pending target before moving forward. Indexed coalesced change ranges
cover skipped selections without predecessor traversal. Full audit/rebuild is
separate from fast pending completion. Stable per-document navigation sections
avoid rewriting a collection-wide listing for unrelated content.

B installs/verifies a COMPLETE independent generation before selection. No
hardlinks between readable generations or authoritative sources. A selected-once
exact path binds coherent reads/copies. Packets retain exact ordered inputs,
transitive local closure/attachments, recipe/renderer, omissions/errors and output
hash. Commit-only, selected packet and combined workloads remain separate.

Checkpoint is a verified rebuild/repack of selected predecessor indexes followed
by an ordinary expected-head successor. Normal open/resolve do not replay history.
Explicit full audit, repair, listing and GC may scale with history.

## Integrity and durability scope

Cooperating processes, exact expected selector, per-store writer coordination;
no FIFO/fairness/SLA. Pure retry only. Immutable install and selection reuse the
native baseline I/O. Unix hard-link no-replace plus directory sync; Windows
MoveFileExW write-through with directory sync explicitly unsupported. Process
interruption does not establish reboot/power-loss persistence. Fresh process
does not imply device-cold caches. Source/schema validity beyond fixture grammar
and hostile root/namespace substitution remain outside the experiment.
