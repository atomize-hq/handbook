# HCM-2.2 Option 1 Authority-Repair Planning Proof

## Scope and authority

This proof belongs to escalation `HCM-2.2-ESC-001` and the human-selected
Option 1 documentation-only repair. Baseline HEAD is
`db503f7a96479775fe25dbb864b9379fd3e61ac8` on
`feat/handbook-contract-membrane`; checkpoint commits
`684029d4dda0c800fe840a19b54bf7f87fb41123` and
`db503f7a96479775fe25dbb864b9379fd3e61ac8` remain unchanged.

The authority chain reconstructed for this packet is:

1. the selected HCM-2.2 planning and implementation handoffs are historical
   authority/evidence;
2. fresh implementation Review 2 is non-CLEAN;
3. the immutable terminal proof
   [`20260720T083550Z--authority-boundary-stop.md`](20260720T083550Z--authority-boundary-stop.md)
   proves the final-candidate/result identity cycle;
4. immutable handoff
   [`../../../handoffs/records/20260720T083855Z--HCM-2-2--orchestration--lifecycle-validation-authority-boundary.json`](../../../handoffs/records/20260720T083855Z--HCM-2-2--orchestration--lifecycle-validation-authority-boundary.json)
   records the escalation and stop; and
5. the human selected Option 1 and authorized this planning repair only.

The immutable escalation handoff's `status/plan.md` and `status/todo.md` path
names are erroneous. This packet uses the existing
[`../tasks/plan.md`](../tasks/plan.md) and
[`../tasks/todo.md`](../tasks/todo.md) paths and does not rewrite that record.

## Frozen repair

The planning subject freezes candidate `1.2` subject identity, result `1.0`,
and the sequence subject -> subject fingerprint -> result/ref -> final candidate
fingerprint/ID -> human approval -> promotion. The result never binds the final
candidate identity. Candidate subject identity is validation-only, and the
final candidate contains exactly one result ref.

It also freezes:

- exact create and amendment preimages/fingerprints/IDs/refs;
- a closed Draft 2020-12 result schema with thirteen exact ordered definition
  bindings and create/amend lifecycle branches;
- bounded no-follow content-addressed persistence, create-new rename-no-replace
  or exact-equal replay, and audit-only timestamp reuse;
- candidate/intake/content/canonical/profile/definition/lifecycle/observation/
  reopened-coverage cross-record equality and promotion-time currentness;
- additive re-author/re-evaluate/new-approval migration with no `1.1` rewrite,
  approval carry-forward, or implicit dual read;
- closed promotion intent `1.2`, exact marker grammar, total recovery, and
  journal-preserving refusal on every mismatch; and
- module-private `#[cfg(test)]` fault injection with no production surface.

The later implementation plan retains required fresh GitNexus analysis for
`evaluate_charter_intake` (known HIGH), generic `validate_record` (known
CRITICAL), and promotion recovery (known HIGH), and prefers a dedicated
lifecycle-validation result/service/store.

## Validation evidence before review dispatch

At baseline HEAD plus the mutable documentation subject:

- JSON parse: both new JSON documents passed;
- Draft 2020-12 meta-schema/result validation: `jsonschema==4.26.0` accepted the
  schema and both fully materialized result records;
- schema identity: 15,430 raw bytes,
  `sha256:1d7d8733b19599804fcda32fe119766b1b910e2223aa981fcd9eb92cc4d5c03f`;
- create vector: subject, result, and final candidate fingerprints/IDs/refs,
  exact schema, cross-record equality, order, and byte bound passed; persisted
  result is 4,334 bytes including LF;
- amendment vector: supporting intake, subject, result, and final candidate
  fingerprints/IDs/refs, exact schema, cross-record equality, order, and byte
  bound passed; persisted result is 5,009 bytes including LF;
- executable negative probes rejected the cycle edge, zero/two-result
  cardinality, a result-to-final-candidate field, stale subject binding, and
  non-exact persisted bytes; the six declared negative-vector classes and five
  cross-record binding groups are present;
- 81 local Markdown link targets across the mutable planning documents exist;
- `git diff --check` passed;
- changed/untracked scope contains no Rust, Cargo, crate, or implementation
  path; and
- no pre-existing handoff, dispatch, review, proof, or released immutable
  record is modified.

The full planning subject must next receive fresh isolated read-only review.
This proof records no review result and grants no implementation or HCM-2.3
authority.
