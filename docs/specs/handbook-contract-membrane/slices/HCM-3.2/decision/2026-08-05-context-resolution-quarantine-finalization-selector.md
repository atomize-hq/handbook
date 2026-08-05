# HCM-3.2 Context Resolution quarantine identity-ordering finalization selector

Status: fresh-parent planning subject. Independent v1.4 selector review must
return CLEAN before any Rust, test, fixture, or product-control edit.

## Fresh authority and additive lineage

- parent orchestration:
  `20260805T101500Z--HCM-3-2--context-resolution-quarantine-finalization`;
- integrated outcome:
  `hcm-3.2-context-resolution-quarantine-finalization`;
- expected base: `eb4b7ba55c6f3db40799e67275f3bbbb8610c87a`;
- expected tree: `d51327c442190edcf1bf56423db54fd2522c5e6f`;
- publication: local-only expected-old compare-and-swap to
  `refs/heads/orchestration/handbook-hcm-3-2-fresh-review-20260805`; never
  push.

This selector is additive. The complete HCM-3.2 product semantics frozen by
`2026-08-01-context-resolution-kernel-selector.md` remain exact. The prior
`2026-08-05-context-resolution-finalization-selector.md`, its immutable v1.4
dispatches, and its proof files are source evidence for the recovered subject;
they are not authority, completed review, a review-budget continuation, or a
replacement for this selector.

The operator grants this new parent and outcome one fresh default planning
selector cadence and one fresh default implementation cadence. The grant does
not rename or waive stable findings, authorize a cycle after CLEAN, or permit
post-CLEAN product work. The full 33-path WIP in protected worktree `018f` is
read-only recovery input. Its source bytes remain immutable.

HCM-0.11 is withdrawn. No validator, schema, template, skill, orchestration
tooling, dependency, crate, public API, unsafe/native/platform/transport,
HCM-3.3, remote-publication, or push work is selected.

## Frozen outcome registry and cadence

The exact registry preimage is this canonical JSON plus one LF:

```json
[{"authority_ref":"docs/specs/handbook-contract-membrane/slices/HCM-3.2/decision/2026-08-05-context-resolution-quarantine-finalization-selector.md","integrated_outcome_id":"hcm-3.2-context-resolution-quarantine-finalization","packet_ids":["HCM-3.2-QF-P1-selector-and-recovery","HCM-3.2-QF-P2-identity-ordering-repair","HCM-3.2-QF-P3-proof-and-review","HCM-3.2-QF-P4-closeout"]}]
```

- registry fingerprint:
  `sha256:af062b113b499761ef50df4e1b427bbb19a91d7158f27793c5ee269f4ef49d02`;
- causal budget:
  `sha256:a50c9dbfec9d454fe4300a5e88ba20cc76e839ed7c85c90b2e92768a6c03a237`.

Exactly these packet IDs are registered:

1. `HCM-3.2-QF-P1-selector-and-recovery`;
2. `HCM-3.2-QF-P2-identity-ordering-repair`;
3. `HCM-3.2-QF-P3-proof-and-review`;
4. `HCM-3.2-QF-P4-closeout`.

Planning and implementation each use one discovery review or same-fingerprint
burst, one consolidated remediation when findings exist, one different-fresh
delta-focused closure, and at most two immediately causal supplementals.
Supplementals address only P1/P2 findings caused or unmasked by the preceding
repair inside unchanged scope, authority, and risk. No general discovery is
reopened during closure, no cycle follows CLEAN, and no identity rename resets
the budget. Mechanical closeout consumes no review cycle.

## Recovered complete subject

The recovered product subject is the existing private JCS Context Resolution
authority capsule and six-dimension kernel through the real generic owner path:

```text
intake -> candidate -> HCM semantic validation -> authenticated registry
publication -> generic promotion -> committed lineage -> current authority
witness -> resolver/envelope/transition consumption
```

The prior WIP's production, tests, fixtures, proof, selector, and dispatches
may be recovered only after selector CLEAN. Recovery preserves and revalidates
the independently proven closure of stable findings
`HCM32-JCS-IMPL-DISC-001`, `002`, `003`, `004`, and `006`. Their IDs and source
lineage remain unchanged. Stable finding `HCM32-JCS-IMPL-DISC-005` remains the
named defect; this fresh parent does not relabel it as new discovery.

Historical committed proof remains audit-only. Operational use requires the
current committed capsule, retained publisher proof, live direct registry
mapping, exact predecessor quartet, repository/profile/stack identity, and
generic committed lineage. Exact-candidate crash recovery and generic non-HCM
fail-closed behavior remain mandatory compatibility constraints.

## Exact remaining product defect

`GenericArtifactLineageStoreV1::require_hcm_publication_quarantine_candidate_during_evaluation`
currently calls mutating `validate_hcm_quarantine_inventory` before comparing
the caller-supplied candidate ref, candidate fingerprint, and idempotency key
with the valid anchored open quarantine record. Inventory validation may
remove a partial `.committed.writing` scratch file before a different candidate
is refused.

The repair must establish identity before mutation:

1. create a valid open quarantine record and its independently valid anchor
   for candidate A;
2. create a partial completion scratch file;
3. supply a different otherwise-valid candidate B identity;
4. reject B before cleanup, publication, completion, reconciliation, or any
   other durable mutation; and
5. prove byte identity for the scratch, open record, anchor, completion state,
   canonical authority, transaction state, and surrounding quarantine
   inventory after rejection.

The negative test must not self-refingerprint A into B or fail first at anchor
validation. Candidate A and its anchor remain internally valid. Candidate B
uses a valid safe ref, candidate fingerprint, and deterministic
`hcm32crpub_<outer64hex>` key but differs from the recorded A identity.

The smallest preferred repair reads and validates A, compares the complete
caller identity, and refuses mismatch before invoking cleanup-capable inventory
validation. A separately explicit side-effect-free validation phase is allowed
only if it is smaller and equally fail closed. Exact A retry must retain crash
recovery and scratch cleanup after identity is established; anchor-only,
committed-only, malformed, conflicting, over-bound, and generic non-HCM states
retain their existing behavior.

## Exact production, test, and symbol boundary

The absolute production ceiling is exactly:

- `crates/engine/src/artifact_lineage_store.rs`;
- `crates/engine/src/artifact_mutation.rs` only for retained recovered work;
- `crates/engine/src/context_resolution.rs` only for retained recovered work.

The new repair is limited to
`GenericArtifactLineageStoreV1::require_hcm_publication_quarantine_candidate_during_evaluation`
in `artifact_lineage_store.rs` unless fresh impact analysis proves that the
repair cannot remain there. `validate_hcm_quarantine_inventory` is context and
may be edited only after its own fresh upstream impact analysis and only if a
side-effect-free split is strictly required. Every existing edited symbol gets
fresh upstream GitNexus impact first; HIGH/CRITICAL results are warned before
edit. Any unexpected HIGH/CRITICAL symbol outside the already reviewed Context
Resolution/quarantine seams is a true stop.

Tests remain limited to:

- `crates/engine/tests/context_resolution_kernel.rs`;
- `crates/engine/tests/hcm_2_3_generic_lineage.rs` for retained recovery proof;
- `crates/engine/tests/fixtures/hcm_3_2_context_resolution/**`.

The exact recovered proof and HCM-3.2 product docs may change. Prior v1.4
dispatches remain byte-identical source evidence. New v1.4 dispatch and
closeout artifacts use only this parent/outcome/packet registry.

## Proof wall and completion

Before implementation review, converge and record:

- RED then GREEN for the valid-anchor A / different-valid-B / partial-scratch
  identity-before-mutation test;
- complete Context Resolution kernel tests;
- generic lineage tests;
- exact definition/profile/vocabulary fixture replay;
- `handbook-engine` all-target/all-feature tests;
- workspace all-target/all-feature tests and check as applicable;
- strict workspace all-target/all-feature Clippy;
- formatting and `git diff --check`;
- ordinary handoff validation and both self-tests; and
- GitNexus scoped plus compare-to-`main` change detection before the primary
  commit, recording unavailable FTS/comparison as unavailable rather than
  GREEN.

Fresh implementation review must independently exercise the exact valid A / B
case and the retained closure of findings `001`, `002`, `003`, `004`, and `006`.
CLEAN requires no unresolved P1/P2. The primary commit contains the reviewed
product/control subject. A separate mechanical v1.4 handoff/ledger commit uses
that primary tip as its reviewed baseline. Expected-old local ref CAS occurs
only after ordinary validation, both self-tests, remote-baseline verification,
and protected-path equality.

## Stop conditions

Stop before edit or completion for another production path, public API,
dependency/crate, unsafe/native/platform/transport machinery, shipped
definition/profile/vocabulary identity change, generic ownership expansion,
HCM-0.11, HCM-3.3+, unexpected HIGH/CRITICAL scope, exhausted cadence,
mandatory delegation failure, validator/tooling change, unavailable required
external proof, or unresolved P1/P2.
