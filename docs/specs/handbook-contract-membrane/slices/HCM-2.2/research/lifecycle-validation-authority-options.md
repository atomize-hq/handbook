# HCM-2.2 Lifecycle-Validation Authority Options

## Status and conclusion

Research finding only; this note grants no implementation or control-pack
authority.

Select **Option 1** and authorize a planning-only authority-repair packet. It is
the smallest repair that preserves the approved lineage: the candidate keeps
its validation-result refs, approvals continue to bind the immutable final
candidate, and promotion continues to bind both. Implementation must remain
stopped until the revised identity graph receives a fresh `CLEAN` planning
review.

## Reconstructed blocker

The base contract puts `validation_result_refs` inside the candidate and the
promotion, derives the candidate fingerprint from its semantic fields, and
freezes append-only order as intake -> candidate -> approvals -> promotion
([`05`, candidate/promotion contracts, lines 1389-1466](../../../05-contracts-schemas-and-gates.md#intake-record-and-artifact-candidate-contracts)).
HCM-2.2 then requires a lifecycle-validation result that binds the candidate
fingerprint, current canonical basis, lifecycle policy/state, every active
observation, and every reopened coverage ID
([`SPEC`, lifecycle result binding, lines 1695-1701](../SPEC.md#deterministic-canonical-bytes-renderer-and-lifecycle)).
The candidate fingerprint preimage excludes only its own identity/fingerprint
and declared audit-only fields, so its validation refs participate in that
fingerprint ([`SPEC`, runtime-record identity, lines 1314-1379](../SPEC.md#acquisition-and-immutable-lineage)).

That produces the proven cycle:

```text
final candidate fingerprint
  -> validation-result ref
  -> lifecycle-validation result
  -> final candidate fingerprint
```

The live checkpoint confirms both halves of the defect: intake always builds an
empty validation-ref array (`crates/engine/src/charter_intake.rs:294-347`), while
promotion checks only that the array is non-empty and copies the unresolved
strings into the promotion (`crates/engine/src/charter_promotion_workflow.rs:897-920`).
The runtime vector likewise uses the non-content-addressed placeholder
`validation_001` in the candidate preimage and record
([`runtime vectors`, lines 87-146](../contracts/runtime-record-fingerprint-vectors-v1.0.json)),
and its cross-record table has no candidate-to-validation-result edge
([`runtime vectors`, `cross_record_bindings`](../contracts/runtime-record-fingerprint-vectors-v1.0.json)).
The terminal proof therefore correctly stopped rather than inventing a
pre-finalization identity
([`authority-boundary stop`, lines 21-45](../proof/20260720T083550Z--authority-boundary-stop.md#bounded-remediation-result)).

## Options compared

| Option | Contract fit | Safety and change cost | Finding |
|---|---|---|---|
| **1. Add `candidate_subject_fingerprint`** | High. Retains candidate validation refs, final-candidate approvals, promotion refs, and the approved explicit intake/candidate/approval/promotion boundary. | Medium. Adds one narrowly scoped identity plus a content-addressed lifecycle-validation record/store and exact currentness verification. The intake builder is still a **HIGH**-impact edit: 7 direct callers, 11 impacted symbols, and 2 processes ([terminal proof, lines 47-56](../proof/20260720T083550Z--authority-boundary-stop.md#validation-retained)). | **Recommended.** It fixes ordering without relocating authority or weakening validation. |
| **2. Move validation refs to promotion only** | Low-to-medium. Acyclic, but reopens the base candidate schema/readiness model, the runtime vectors, author/finalization flow, and the prior HCM-2.2 decision that fixed an explicit current-definition candidate boundary ([planning decisions D2-D3, lines 1062-1077](../../../handoffs/records/20260719T230914Z--HCM-2-2--orchestration--implementation-packet-approved.json)). | High. Validation would be created after the immutable candidate and potentially after approval, so candidate eligibility semantics, promotion construction, cross-record closure, persistence, migration, and refusal timing all need redesign and new proof. | Valid only if product authority intentionally wants lifecycle validation to be promotion-only. No source shows that broader redesign is needed. |
| **3. Defer HCM-2.2** | Exact fit with the current stop state. | Lowest immediate implementation risk, but it resolves nothing: the checkpoint remains non-authoritative and HCM-2.3 remains unauthorized ([program status, lines 215-224](../../../00-README.md#initial-program-conclusion)). | Choose only as an explicit scheduling/product deferral, not as a technical repair. |

Option 1 also follows established pack design: compatibility edges are kept
one-directional to preserve acyclicity, immutable records are not retrofitted
with forward links, and approvals bind a final immutable candidate
([semantic model, lines 208-222](../../../02-semantic-model.md#intake-records-candidates-and-promotion)).

## Exact authority that Option 1 must freeze

The planning packet should define and review all of the following before any
code edit:

1. **Subject preimage.** Compute `candidate_subject_fingerprint` over the
   complete candidate semantic preimage available before lifecycle validation,
   excluding exactly `candidate_id`, `candidate_fingerprint`,
   `candidate_subject_fingerprint`, and `validation_result_refs`. Include schema
   identity/version, intake ref, target refs, selected profile fingerprint,
   basis fingerprint, normalized-content ref, field sources, unresolved
   coverage, eligibility, and approval-policy ref. The subject fingerprint is
   validation binding only; it is neither candidate identity nor canonical
   authority.
2. **Acyclic sequence.** Freeze the order as candidate subject -> subject
   fingerprint -> lifecycle-validation result/ref -> final candidate
   fingerprint/ID -> approvals -> promotion. The subject fingerprint field and
   exact validation refs participate in the final candidate fingerprint;
   approvals and promotion bind only that final candidate fingerprint.
3. **Validation-result record.** Publish a closed schema and fingerprint vector
   for one engine-produced, content-addressed Charter lifecycle-validation
   result. It must bind the subject fingerprint, exact intake/content lineage,
   create-only null or current canonical basis, selected profile and definition
   fingerprints, lifecycle policy fingerprint, current lifecycle head/state,
   the complete ordered active-observation ref/fingerprint set, and the complete
   reopened-coverage set. Define timestamp classification and exact cardinality
   (prefer exactly one result for HCM-2.2).
4. **Refs and persistence.** Define an exact repo-local state partition and ref
   grammar. Require no-follow, create-new, exact-equal replay, bounded bytes,
   ref-basename/record-ID/fingerprint/exact-byte equality, and append-only
   retention. Add the candidate-to-validation-result edge to the normative
   cross-record table.
5. **Promotion currentness.** Under the retained promotion/lifecycle locks,
   recompute the candidate subject fingerprint, resolve every validation ref,
   and revalidate the current canonical basis, profile/definitions, lifecycle
   policy/head/state, active observations, and reopened coverage. Missing,
   forged, stale, duplicate, reordered, incomplete, or excess authority must
   refuse before any canonical, promotion, lifecycle, journal, or lineage
   mutation.
6. **Version and migration.** Prefer a new additive candidate record version
   rather than changing the reviewed `1.1` identity semantics in place. Never
   rewrite checkpoint candidates with empty refs. Require re-authoring/re-
   evaluation to create the new candidate; existing approvals cannot be reused
   because they bind the old final candidate fingerprint. Preserve the
   checkpoint only as non-authoritative evidence and add no dual-read selected
   product path.

The authority revision should also correct the stale `SPEC` status text that
currently calls HCM-2.2 completed ([`SPEC`, lines 3-10](../SPEC.md#status-and-authority));
the later slice-local stop proof and program status say it is incomplete and
non-authoritative. Existing handoff and proof records remain immutable.

Two more control-pack truth repairs belong in the same planning subject:

- The proof ledger's summary still classifies `PR-009` as
  `ContractCorrectAndProven`, closes `PG-INTAKE-01`, `PG-INTAKE-02`, and
  `PG-CHARTER-01`, and calls HCM-2.2 completed
  ([`06`, lines 114-169](../../../06-proof-and-regression-ledger.md#open-program-proof-gates)).
  Its later authoritative gate section correctly says the slice is incomplete
  and those gates remain open
  ([`06`, lines 931-988](../../../06-proof-and-regression-ledger.md#hcm-22-constitutional-root-implementation-proof-gate)).
  The optimistic summary rows must be reverted; otherwise the pack exposes two
  incompatible gate states.
- The immutable escalation handoff's `pack_updates` names nonexistent
  `slices/HCM-2.2/status/plan.md` and `status/todo.md` paths
  ([handoff `pack_updates`, lines 453-501](../../../handoffs/records/20260720T083855Z--HCM-2-2--orchestration--lifecycle-validation-authority-boundary.json));
  the actual files are `slices/HCM-2.2/tasks/plan.md` and
  `tasks/todo.md`. Do not rewrite the handoff. Record the corrected paths in the
  new planning dispatch/closeout and ensure link validation covers them.

## Safe continuation sequence

1. Human explicitly selects Option 1 and authorizes a **planning-only**
   authority-repair packet.
2. Revise the authoritative `SPEC`, the base candidate/promotion contract where
   necessary, record schemas/vectors/cross-record bindings, task plan/checklist,
   and status truth. Reconcile the contradictory `06` summary/gate rows and use
   the real `tasks/` paths in new records. Do not edit Rust yet.
3. Have a different fresh reviewer admit the complete authority-repair subject
   and return `CLEAN`. Stop if the graph is cyclic, currentness is incomplete,
   migration is implicit, or subject identity can be mistaken for authority.
4. Create a new implementation packet covering the lifecycle-validation repair
   **and all three remaining Review 2 findings**: durable promotion journal,
   mismatch-preserving recovery refusal, and test-only fault-injection surfaces
   ([escalation findings, lines 338-373](../../../handoffs/records/20260720T083855Z--HCM-2-2--orchestration--lifecycle-validation-authority-boundary.json)).
5. Before implementation edits, rerun GitNexus impact and explicitly acknowledge
   the HIGH intake/compiler blast radius. Implement test-first, replay the full
   proof wall, obtain a different fresh complete-subject `CLEAN` review, then
   perform the normal primary commit and separate handoff/ledger closeout.

No step authorizes HCM-2.3. These are the escalation's recorded resume and
acceptance conditions, not optional process
([handoff `resume`, lines 537-603](../../../handoffs/records/20260720T083855Z--HCM-2-2--orchestration--lifecycle-validation-authority-boundary.json)).

## Additive decision disposition

At `HCM-2.2-ESC-001`, the human authority explicitly selected Option 1 and
authorized only the documentation repair described here. The selected design is
now expanded normatively in [`../SPEC.md`](../SPEC.md), the base contract in
[`../../../05-contracts-schemas-and-gates.md`](../../../05-contracts-schemas-and-gates.md),
the result schema/vectors under [`../contracts/`](../contracts/), and the real
[`../tasks/plan.md`](../tasks/plan.md) and
[`../tasks/todo.md`](../tasks/todo.md) paths. This research note remains design
evidence, not independent implementation authority. The immutable escalation
handoff and its erroneous `status/` path names remain unchanged.
