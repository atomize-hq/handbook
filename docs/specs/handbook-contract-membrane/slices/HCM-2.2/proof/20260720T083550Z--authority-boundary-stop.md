# HCM-2.2 Authority-Boundary Stop

## Status

HCM-2.2 is not complete. The implementation is preserved only as a
non-authoritative checkpoint. No HCM-2.3 work is authorized.

## Terminal review state

Fresh final implementation Review 2 admitted the exact 175-path subject at
`sha256:8b53ec2230285592706b56227695df950c061591cffecef4604dee6f94034b90`
and returned `CHANGES_REQUIRED`. It found one Critical and three Required
defects. The Critical defect is that engine-produced candidates always contain
an empty `validation_result_refs` array, making the shipped
author-to-approve-to-promote path impossible, while promotion accepts any
non-empty synthetic ref without resolving validation authority.

Review dispatch:
`docs/specs/handbook-contract-membrane/handoffs/dispatches/20260720T075532Z--HCM-2-2--fresh-final-implementation-review-2.json`.

## Bounded remediation result

The immutable remediation dispatch
`20260720T081800Z--HCM-2-2--lifecycle-validation-authority-remediation`
replayed all 13 subject entries and aggregate
`sha256:3dbfcd29d827bd27bf75174c1c0fb3d82a3a1483b2009ecb4e6e045e320ce39d`.
The fresh remediation agent made no edits and returned `BLOCKED` after proving:

1. the candidate fingerprint preimage is the complete closed candidate after
   excluding only its own ID/fingerprint and class-specific audit fields;
2. the candidate contains `validation_result_refs`;
3. the required lifecycle validation result must bind the candidate
   fingerprint; and
4. candidate finalization precedes approval/promotion and is immutable.

The resulting dependency is
`candidate fingerprint -> validation-result ref -> validation result -> candidate fingerprint`.
The frozen authority defines no separate pre-finalization candidate identity.
Substituting normalized content plus intake lineage would invent authority.

Authority evidence:

- `slices/HCM-2.2/SPEC.md:1324-1330,1356-1360,1695-1700`;
- `05-contracts-schemas-and-gates.md:1389-1394,1445-1446,1458-1464`;
- `slices/HCM-2.2/SPEC.md:1855-1870` closed-graph and proof stop conditions.

## Validation retained

- Remediation subject admission: PASS, 13/13 entries and exact aggregate.
- Focused intake/lifecycle/promotion tests: PASS, 14/14; these tests do not
  close the missing validation-authority path.
- `git diff --check`: PASS before this stop record.
- GitNexus upstream impact: HIGH for `evaluate_charter_intake` with 7 direct
  callers, 11 impacted symbols, and 2 affected processes; HIGH for the compiler
  execution path with 1 direct caller, 7 impacted symbols, and 1 process.
- Remediation edits: none.

The other Review 2 durability and public test-hook findings remain unresolved
because the Critical authority graph must be repaired first.

## Exact resume condition

A human-approved additive authority revision must define a closed, acyclic,
engine-owned lifecycle-validation identity and its fingerprint preimages,
candidate/result refs, persistence class, currentness checks, and migration
effect. That authority must receive fresh planning review before implementation
resumes. The new packet must then remediate all four Review 2 findings and obtain
a different fresh complete-subject review returning `CLEAN`.
