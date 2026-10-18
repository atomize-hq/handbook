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
| Non-retired historical content | until explicit retirement; registered pins/readers/standalone packet preparations block retirement |
| Reserved/sealed candidate and private stages | charged and protected until selection transfer or fenced abandonment plus deletion/GC |
| Prepared checkpoint | reservation protects rebuilt roots; expected-head successor installs verified source/coverage/version binding |
| Pending A plan | target and prior roots protected through completion; selected root/sequence bindings and complete actions rederived on resume; stable divergence backups retained |
| Old history index versions | protect for live candidates/checkpoints; current roots cover permanent evidence |

Selection binds a permanent allocation-owner record for the attempt's entire
conservative allowance, including intermediate/rejected writes and stages. Only
after this selected transfer may the candidate reservation be released. These
selected-owner charges deliberately overcount shared and later collected bytes;
report them separately from actual unique retained storage and candidate quota.
Permanent selected-history growth is bounded by the run allocation, not an
outcome horizon. GC can reclaim unreachable intermediates without losing owners.

A combined packet follows its publication's explicit content-retirement lifetime;
registering a publication pin extends that lifetime. A standalone prepared packet
protects its exact base until explicit abandonment/release. A checkpoint carrying
an unchanged packet reference also retains that capsule's physical owner until
the last referencing live publication retires. External ordinary copies survive
independently. No packet disappears because only a source object was collected.

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

## Premeasurement corrections

The final common pin adds byte-role counters, bound-derived packet admission,
typed retained-snapshot packet assembly, sealed-candidate root checks, exact
pending-refresh change-root retention, and preselection attempt hints. Hints
only locate an authenticated publication/request; selected membership/outcomes
remain the acceptance authority. Compact hints and accepted result/publication
evidence survive content retirement. Private allocation journals precede result
creation; fenced collection removes unselected evidence and owned stages before
releasing its reservation. Explicit packet-only candidates cannot be selected
and retain graph/projection/packet contents without obsolete history indexes.

Earlier development measurements remain separate observations. In particular,
the first 5000-source packet refusal and Windows long-path failure are retained.
This is an experimental private format iteration, not an approved format default
or migration. The original baseline and control retain their separate contracts.

Final history recipe amendment, before the final history lane: metadata selects
ID `i % 16` at `i % 5 == 2`, with label suffix `(i / 80) % 2`. Each ID is revisited
after 80 selections, so its value now toggles on every metadata visit. The pilot's
`(i / 5) % 2` suffix became equal-content after each ID's first update; that pilot
is preserved and cannot be counted as a sustained metadata mutation mix. Final
verification reconstructs the complete expected small state from the captured
checked initial fixture and declared recipe, then independently compares every
logical semantic-index entry at each history depth. Full history selection and
outcome walking remains outside append clocks. Engine work has 225 minutes of
the four-hour history allocation, reserving 15 minutes for independent truth and
reporting after a censor. `store`/validation/projection/lifetime code is unchanged
from a1108c6; this amendment changes the resident test workload and its oracle.
