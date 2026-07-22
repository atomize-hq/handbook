# HCM-2.3 Planning Review 4 Remediation

## Review authority

- reviewer: `/root/hcm_2_3_planning_review_4`
- verdict: `CHANGES_REQUIRED`
- admitted subject: 22 paths, aggregate
  `sha256:e1e22c2fe71723cda140abe38beaab9771f2fe27dce67d43fb8c53379212c207`
- findings: zero Critical, two Required, zero Optional, zero Nit
- disposition: both Required findings were accepted; neither was waived

## Exact dispositions

| Finding | Accepted defect | Remediation |
|---|---|---|
| `HCM-2.3-PR4-001` | `installed_complete` verification was allowed as a basis but the schema unconditionally required `staged/...` refs, contradicting recovery authority that requires exact final installed refs | made `verifiedStage` basis-discriminated for all three transaction families; `staging` now requires the exact ordered `staged/<ordinal>-<token>.bin` tuple, while `installed_complete` requires the operation's exact ordered final-ref grammar; added one terminal-fingerprint positive per operation; retained the cross-record semantic rule that every ref/digest/length tuple must agree exactly with the bound intent; added staging/final substitution, reorder, and intent-disagreement negatives |
| `HCM-2.3-PR4-002` | the intent schema admitted null intake time and non-null candidate/promotion time despite intake identity requiring one durable sample and the other operations being timestamp-free | required a format-valid non-null `sampled_finalized_at_utc` for `intake.record.append`, required null for candidate append and promotion, added operation-specific negatives, and added an exact intake intent replay positive proving the same durable time and intent fingerprint are retained rather than resampled |

## Verification replay

The repaired control subject contains 29 positive records. Independent Draft
2020-12 and semantic replay reported:

- all 29 positives schema-valid with exact terminal fingerprints;
- three schema-valid `installed_complete` positives whose ordered refs, byte
  digests, and byte lengths equal the corresponding intent outputs;
- six basis/path substitutions rejected structurally;
- null intake time and both non-null timestamp-free-operation mutations
  rejected structurally;
- two schema-shaped reordered/substituted final-ref attacks rejected by exact
  cross-record intent equality; and
- the intake replay retaining `2026-07-21T00:00:00Z` and the original intent
  fingerprint byte-for-byte.

## Scope and next review

The remediation changed only the HCM-2.3 control schema/vector packet and this
proof record. It did not edit Rust, Cargo, runtime tests, production assets,
HCM-2.2 authority, a handoff ledger, or a preservation location. The repaired
complete documentation subject requires a different fresh isolated read-only
reviewer and remains unauthorized for implementation or commit until that
complete subject is CLEAN.
