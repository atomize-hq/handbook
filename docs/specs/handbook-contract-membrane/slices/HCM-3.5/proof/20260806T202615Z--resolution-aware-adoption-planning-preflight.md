# HCM-3.5 Resolution-aware adoption planning preflight

## Identity and local admission

- Meta workflow: `handbook-hcm-3-5-planning-20260806`.
- Dispatch nonce: `20505740734f5fcc7e2844b087ce2fa6e011742a7655ac9d2867e467500f695b`.
- Bound increment task/host: `019fd8b9-5172-7022-8f6a-89e45751a747` / `local`.
- Assigned checkout: `C:/Users/spmcc/.codex/worktrees/c032/handbook`.
- Observed HEAD and dedicated local integration ref:
  `1a10bfd59e8433fa344dd2baa4880b4b87c7f8f6`.
- Observed worktree/ref tree:
  `92225d653952cd1ca11a9e7edf449e98fc7eaff7`.
- Required ancestor `1a10bfd59e8433fa344dd2baa4880b4b87c7f8f6` is present.
- Observed local `origin/main` / `origin/HEAD` baseline:
  `a3babd20329027afacdcee9d8b7b9d638d15af5b`; no fetch or remote query ran.
- The task worktree is detached and clean before these planning changes.
- Protected checkout trees, statuses, and changed-content aggregates were
  recorded before editing. Nineteen were clean; one pre-existing dirty
  protected worktree was recorded and is outside this task's scope.

## Predecessor verification

The ordinary v1.4 validator passed against the live repository: 93 records,
539 current dispatches, all ledger entries, templates, and immutable
historical corpora validate. The selected HCM-3.4 predecessor is indexed as a
completed orchestration record and its primary-to-closeout delta contains only
its parent record and rebuilt ledger. Its primary commits are inspectable at
the assigned base:

- `ca765cc` adds private Snapshot Memory primitives;
- `cc6d849` adds the private Projection source pair and proof; and
- `1a10bfd` adds only HCM-3.4's mechanical closeout record and ledger update.

## Live seam evidence and authority boundary

`handbook-flow` and `handbook-pipeline` already depend on `handbook-engine`.
`ContextResolutionEnvelope` is publicly re-exported by `handbook-engine`, but
`snapshot_memory` and `projection` remain private modules. Flow's public
resolver is used by CLI and compiler callers; pipeline compile carries an
optional raw `work_level` through `compile_pipeline_stage*`, document assembly,
and `filter_scoped_blocks`. The v1.4 schema already admits nullable snapshot
references and the semantic gate contract already distinguishes local closure
from parent promotion, but neither fact exports a snapshot operation or
implements a gate runtime.

Therefore a future HCM-3.5 implementation needs an explicit public
compatibility and gate-operation decision. This planning subject records the
decision boundary and makes no source, product, or public-interface change.
