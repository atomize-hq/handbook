# HCM-3.4 Snapshot Memory implementation authority admission

**Decision status:** active explicit user-authorized implementation authority,
pending only the new selector's different-fresh closure review.

## Exact source and identity

This durable admission records the complete rendered HCM-3.4 implementation
contract supplied directly by the user to the bound increment task, rather
than inferring implementation authority from planning, scheduling, a nonce, or
the prior true-stop prose.

- User-authorized increment: `HCM-3.4` whole-slice implementation.
- Bound increment task / host:
  `019fd50a-1e47-7a32-8ccd-3aaad8582e65` / `local`.
- Meta workflow / thread / host:
  `handbook-hcm-3-4-implementation-20260806` /
  `019fd4ae-f9ee-7e82-bd09-499df96f9aa5` / `local`.
- Dispatch nonce:
  `ae608ed25e00159bbd435b6a7b5bf1dd1f75cb285d8c6b07eb9c4864896e8fa2`.
- Active packet:
  `docs/specs/handbook-contract-membrane/slices/HCM-3.4/tasks/plan.md`.
- Required planning context:
  `20260806T022000Z--HCM-3-4--orchestration--snapshot-memory-planning-protected-checkout-stop`.
- Required base / tree:
  `d7877f7843afbcda78d65e0fa2bf7093d3c71e6a` /
  `47440ba606d2fd1ccce340fbc9ff13553d3602d1`.
- Local-only target:
  `refs/heads/orchestration/handbook-hcm-3-4-implementation-20260806`.

The user contract expressly authorizes one **fresh top-level** implementation
increment. It does not claim same-parent resumption of the planning
orchestration, does not provide an `authority_continuation`, and does not alter
the immutable planning record. The planning record remains an
`authority_boundary` true stop for its own planning parent; its exact
same-parent continuation rules therefore do not apply to this separately
authorized implementation parent.

## Narrow authority and ceilings

This admission authorizes only the private deterministic Snapshot Memory
capability selected by HCM-3.4: capture policy and source-family closure,
immutable normalized snapshots, consistency and paired boundaries,
deterministic deltas and catalog-backed drift, fail-closed redaction/retention/
deduplication, private proof through the existing generic Projection engine,
and PG-SNAP-01 through PG-SNAP-06 proof closure.

It authorizes neither a public API, schema/version/dependency/runtime
configuration change, shipped default, transport, SDK/CLI/pipeline, Handoff or
packet consumer adoption, HCM-3.5/HCM-3.6/Phase-3 exit, model interpretation,
remote operation, or work outside the assigned checkout. It requires a fresh
reviewed implementation selector, internal v1.4 child dispatches, independent
review/remediation/closure cadence, two local commits, protected-path
preservation, and local-only compare-and-swap publication. Those requirements
are ceilings, not invitations to widen the selected private boundary.

## Admission consequence

The successor selector must cite this decision, HCM-3.4/HCM-3.5 slice and seam
boundaries, applicable architecture invariants, all Snapshot contracts/proof
gates, and the live review/ledger protocol. It may admit code only after its
different-fresh closure is CLEAN. This decision is immutable after that review.
