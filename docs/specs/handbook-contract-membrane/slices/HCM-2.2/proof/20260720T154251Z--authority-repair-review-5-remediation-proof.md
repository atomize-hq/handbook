# HCM-2.2 Authority-Repair Review 5 Remediation Proof

## Review result

Different fresh isolated reviewer `/root/hcm_2_2_authority_repair_review_5`
admitted all 18 paths and aggregate
`sha256:8280881a94829e9833444f4094e114a09a9f335f09679f86a610270c7c2593a6`
from immutable dispatch
[`../../../handoffs/dispatches/20260720T152442Z--HCM-2-2--authority-repair-planning-review-5.json`](../../../handoffs/dispatches/20260720T152442Z--HCM-2-2--authority-repair-planning-review-5.json).
It returned `CHANGES_REQUIRED` with two Required findings,
`HCM-2.2-ARPR5-001` and `HCM-2.2-ARPR5-002`. Both findings were accepted
without waiver.

## Findings and repair

1. The validation-result prose required active observations in ascending ref
   order while the cross-record vector required byte-for-byte committed
   lifecycle-head order. Existing lifecycle heads retain event-precedence then
   event-fingerprint order, so a multi-observation head could not satisfy both.
   The repaired contract selects the current committed transition head's exact
   retained array order throughout result identity, intent binding, state
   fingerprint input, and promotion currentness. Validation never ref-sorts the
   array and never rewrites or migrates a retained head.
2. One RED checklist line still grouped candidate `1.1` with the current
   additive intake/approval/promotion work. It now names intake/approval/
   promotion `1.1`, candidate `1.2`, and lifecycle-validation-result `1.0`, and
   requires a mechanical version-surface check that candidate `1.0`/`1.1`
   remain historical-only.

The runtime vector now includes a two-observation positive whose committed
event order is intentionally the reverse of ascending ref order, its exact
expected validation array, a ref-sorted negative, and retained-head
compatibility. The plan and checklist require the same positive and duplicate,
reordered, incomplete, excess, and rewrite negatives. SPEC and the base
contract use the same ordering authority. No prior dispatch or proof was
rewritten.

## Remediation validation

- the order-divergent vector must prove expected validation order equals the
  committed head and differs from the ref-sorted negative;
- each observation ref basename must equal its fingerprint-derived ID and the
  positive array must be unique by ref and fingerprint;
- current SPEC, base-contract, vector, plan, and checklist wording must contain
  no active-observation ref-sort requirement;
- every current candidate implementation task must name `1.2`; any candidate
  `1.0`/`1.1` mention must be explicitly historical/preserved/not-modified;
- all prior identity, schema, crash-state, currentness, migration, status,
  links, whitespace, and documentation-only scope validations must remain
  passing.

The next review must use another different fresh isolated read-only reviewer
over a new exact complete-subject manifest. This proof grants no implementation
or HCM-2.3 authority.
