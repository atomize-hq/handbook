# HCM-2.2 Authority-Repair Review 1 Remediation Proof

## Review result

Fresh isolated reviewer `/root/hcm_2_2_authority_repair_review_1` admitted all
12 paths and aggregate
`sha256:44275cdd2d217d2b86dc3a8042d9d83f2565d34f30ed205537adc57d0a8fc63a`
from immutable dispatch
[`../../../handoffs/dispatches/20260720T133841Z--HCM-2-2--authority-repair-planning-review.json`](../../../handoffs/dispatches/20260720T133841Z--HCM-2-2--authority-repair-planning-review.json).
It returned `CHANGES_REQUIRED` with one Required finding,
`HCM-2.2-ARPR-001`: promotion intent `1.2` lacked an exact fingerprint preimage,
schema/vector, ordered writer/marker protocol, and exhaustive crash-state
partition. There were no Critical, Optional, or Nit findings. The finding was
accepted without waiver.

## Remediation

The repaired planning subject adds:

- closed Draft 2020-12
  [`../contracts/promotion-transaction-intent-1.2.0.schema.json`](../contracts/promotion-transaction-intent-1.2.0.schema.json)
  with literal top-level/nested fields and create/amend closure;
- exact
  [`../contracts/promotion-transaction-intent-vectors-v1.0.json`](../contracts/promotion-transaction-intent-vectors-v1.0.json)
  amendment preimage/fingerprint, complete persisted-document bytes/hash,
  marker payload, pending path, sixteen-name grammar, terminal name sets,
  cross-record bindings, and negatives;
- controlling SPEC rules that exclude exactly `intent_fingerprint` from intent
  identity, include the engine transaction ID, persist JCS+LF, distinguish
  semantic fingerprints from raw document hashes/lengths, and bind every
  candidate/result/authority/output/recovery field;
- cumulative writer transitions `W0` through `W15`, exact-prefix partial-write
  semantics, atomic `.tmp` marker publication, marker causality, pre-existing
  exact-final classification, and exact terminal directory contents;
- an ordered exhaustive recovery partition with mismatch as the total final
  row, plus a required finite Cartesian-product conformance enumeration; and
- corresponding base-contract, plan, and checklist updates.

No prior dispatch or proof was rewritten.

## Remediation validation

- intent schema meta-validation: PASS;
- schema identity: 16,019 raw bytes,
  `sha256:c2f5cf51b833585bc10cfcd5b99ad2a2e2c0e952a79b3f2910ae6afbff746caa`;
- amendment intent fingerprint:
  `sha256:d7f3c3a9ee860288f0ed52c83254044d67828d2499909633584290d8ec265830`;
- persisted intent: 7,607 JCS+LF bytes,
  `sha256:ad48fe8c0be1d3ee72888a6409d7e9ef3278623bf2c68c5cb199a3db18a08654`;
- exact marker payload: the same `sha256:` line plus LF, 72 ASCII bytes;
- amendment schema, thirteen-definition equality, candidate/result/currentness,
  output/ref/fingerprint/document/length, target/old-snapshot, and marker
  bindings: PASS;
- synthesized create branch with null/absent closure: PASS;
- unknown field, crossed create/amend nullability, and branch-negative probes:
  refused as required; and
- sixteen owned names, forward-marker order, and create/amend committed/rolled-
  back terminal name sets: PASS.

The next review must use a different fresh isolated read-only reviewer over a
new exact complete-subject manifest. This proof grants no implementation or
HCM-2.3 authority.
