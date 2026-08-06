# HCM-3.4 Snapshot Memory implementation selector remediation

**Decision status:** remediation of discovery findings `HCM34-I1-SEL-001`
through `HCM34-I1-SEL-003`; no implementation code is admitted until this
selector's different-fresh closure returns CLEAN.

## Explicit authority and predecessor posture

This selector is authorized only by the durable direct-user implementation
admission at
`docs/specs/handbook-contract-membrane/slices/HCM-3.4/decision/20260806T032800Z--snapshot-memory-implementation-authority-admission.md`.
It records the explicit whole-slice user grant bound to task
`019fd50a-1e47-7a32-8ccd-3aaad8582e65`, host `local`, and nonce
`ae608ed25e00159bbd435b6a7b5bf1dd1f75cb285d8c6b07eb9c4864896e8fa2`.
It is not a continuation grant and does not resume, amend, or reinterpret the
planning parent's immutable authority-boundary true stop.

The HCM-3.4 planning true-stop handoff
`20260806T022000Z--HCM-3-4--orchestration--snapshot-memory-planning-protected-checkout-stop`
and completed HCM-3.3 Projection handoff are required predecessor context only.
The assigned worktree is the exact clean base
`d7877f7843afbcda78d65e0fa2bf7093d3c71e6a` with tree
`47440ba606d2fd1ccce340fbc9ff13553d3602d1`; the only publication target is
`refs/heads/orchestration/handbook-hcm-3-4-implementation-20260806` in
`local_only` mode. No protected checkout was entered or inspected by this task,
and no remote has been queried, fetched, changed, or pushed.

## Required authority pack and selected seam

The authoritative implementation pack is: `01` **Snapshot Memory posture** and
**Non-negotiable invariants**; `02` **Snapshot Memory**, **Capture
consistency**, **Snapshot deltas**, **Resolution-aware snapshot projection**,
and **Security, redaction, and retention**; `03` **Snapshot Memory** and
**Projection/Resolution** seam rows; `04` the `HCM-3.4` and `HCM-3.5` rows;
`05` all Snapshot policy/record/delta/projection/redaction-retention contracts;
`06` PG-SNAP-01 through PG-SNAP-06 plus regression rules 44-47; and live `07`,
`08`, and `09` selector, delegation, causal-review, closeout, and finding
protocols. HCM-3.5 is an excluded consumer-adoption sibling, not a dependency
that this slice may advance.

The sole integrated outcome remains
`hcm-3.4-snapshot-memory-implementation`; the sole packet remains
`HCM-3.4-I1-snapshot-memory-implementation-causal-review`. The frozen parent,
outcome registry, and parent/outcome-derived causal budget are unchanged from
the discovery dispatch. This remediated selector replaces the defective
prospective selector for future implementation selection while preserving the
original dispatch as immutable discovery evidence.

## Exact private envelope and seven internal packets

Only these private engine/test paths may be selected after CLEAN:

1. `crates/engine/src/lib.rs` — one private `mod snapshot_memory;` declaration.
2. `crates/engine/src/snapshot_memory/mod.rs` — private module wiring/shared
   internal types.
3. `crates/engine/src/snapshot_memory/policy.rs` — capture policy/source family,
   slot/window/comparison/predecessor/drift/retention closure.
4. `crates/engine/src/snapshot_memory/record.rs` — immutable normalized
   snapshots, coverage, consistency, fingerprints, and predecessors.
5. `crates/engine/src/snapshot_memory/delta.rs` — compatible deltas, complete
   comparison, catalog evaluation, and typed signals.
6. `crates/engine/src/snapshot_memory/redaction.rs` — fail-closed redaction,
   pointer/retention/deduplication/compaction controls.
7. `crates/engine/src/snapshot_memory/projection.rs` — private source-byte
   conversion only; the generic Projection engine remains behaviorally
   unchanged.
8. `crates/engine/src/snapshot_memory/tests.rs` and exactly these fixture files:
   `fixtures/policy-family.json`, `fixtures/snapshot-record.json`,
   `fixtures/delta-catalog.json`, `fixtures/redaction-retention.json`, and
   `fixtures/projection-source.json`.
9. `crates/engine/src/projection/tests.rs` — one private generic Projection
   integration proof; `execute_projection` is called but not changed.

Packets remain internal child work in this order: (1) policy/source-family,
(2) normalized record, (3) consistency/paired boundaries, (4) delta/drift,
(5) redaction/retention/dedupe, (6) private Projection proof, and (7) complete
PG-SNAP closure. Before each packet the parent freezes its exact paths,
owner/call-path, contract delta, proof gates, and non-goals in a fresh
schema-valid v1.4 dispatch. No packet is a top-level task.

## Proof, risk, and hard non-goals

The proof wall is exactly PG-SNAP-01 through PG-SNAP-06: normalized
identical-state/record-identity replay; all stable/bounded/unstable/refusal and
predecessor topology cases; compatible ordered delta/catalog/signal bijection;
private generic Projection exact-source, captured-revision, typed
omission/retained-pointer/no-hidden-read/stale refusal, and
`authority_effect: none`; fail-closed redaction/retention/holds/deduplication;
and deterministic catalog-backed drift without free-form model meaning.

GitNexus full-text search is unavailable (LadybugDB FTS extension absent), not
GREEN. Exact call-graph analysis marked existing `execute_projection` HIGH risk
with 12 direct test callers and three affected test processes; it is not
modified. Any later edit to an existing function, class, or method requires a
fresh upstream impact report and a selector repair before the edit.

No public export, Cargo/version/dependency/runtime setting, schema, default
catalog, renderer, transport, SDK, CLI, pipeline, Handoff/packet consumer,
source mutation, model interpretation, protected-path edit, remote operation,
HCM-3.5/HCM-3.6 work, or Phase-3 exit claim is selected. The final parent still
requires independent causal review, GitNexus scoped and compare-to-main change
detection, two commits, and only the required local ref compare-and-swap.
