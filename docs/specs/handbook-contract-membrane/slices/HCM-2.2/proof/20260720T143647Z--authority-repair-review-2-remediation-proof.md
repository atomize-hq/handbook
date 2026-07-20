# HCM-2.2 Authority-Repair Review 2 Remediation Proof

## Review result

Different fresh isolated reviewer `/root/hcm_2_2_authority_repair_review_2`
admitted all 15 paths and aggregate
`sha256:00c77046b279c1318424cc41cf6c3eb4feef00d5f7a7589efd22438d5d5be26e`
from immutable dispatch
[`../../../handoffs/dispatches/20260720T141108Z--HCM-2-2--authority-repair-planning-review-2.json`](../../../handoffs/dispatches/20260720T141108Z--HCM-2-2--authority-repair-planning-review-2.json).
It returned `CHANGES_REQUIRED` with four Required recovery-closure findings and
no Critical, Optional, or Nit findings. All four were accepted without waiver.

## Findings and repair

1. `HCM-2.2-ARPR2-001`: W3/W4 amendment rollback could not satisfy its exact
   terminal `canonical.old` requirement when the snapshot was absent/partial.
   Recovery now copies the still-exact retained old target when absent or
   append-completes only the missing suffix of a verified exact prefix, fsyncs,
   and revalidates fingerprint/document hash/length before publishing rollback.
   A repeated crash leaves another exact prefix and safely retries.
2. `HCM-2.2-ARPR2-002`: the exhaustive domain omitted an independent
   `canonical.new` dimension. The domain and vector now name
   `canonical.old`, `canonical.new`, `promotion-record.new`, and
   `lifecycle-transition.new` independently.
3. `HCM-2.2-ARPR2-003`: mutable recovery predicates did not constrain the
   directory suffix. No-directory, `.pending`, `.committed`, and `.rolled-back`
   predicates are now disjoint; only `.pending` can mutate, terminal suffixes
   are read-only verification, and every crossed/other suffix is mismatch.
4. `HCM-2.2-ARPR2-004`: partial `intent.tmp` could not be authenticated after
   process memory was lost. Pending intent temp was removed. The writer now
   constructs, fsyncs, reopens, and verifies the complete self-fingerprinted
   intent in non-authoritative sibling `.intent-staging/`, then atomically
   renames the whole file into a newly fsynced empty `.pending` directory.
   Recovery sees only empty pending or complete self-verifying `intent.json`;
   partial intent is mismatch. Scratch orphans never enter authority scanning.

The owned pending grammar is consequently fifteen names. The SPEC, vector,
base contract, implementation plan, and checklist were updated additively. No
prior dispatch or proof was rewritten.

## Validation

- both schemas meta-validate and create/amend positive records validate;
- candidate/result/final and promotion-intent identities/documents/marker still
  reproduce exactly; intent remains 7,607 bytes and its marker 72 bytes;
- the vector has fifteen unique owned pending names, no `intent.tmp`, fifteen
  independent state axes, six cross-record bindings, and nine negative classes;
- abstract enumeration over suffix, intent, target, commit, and empty-state
  selectors proves the seven non-catch recovery predicates pairwise disjoint;
- every amendment `canonical.old` prefix length from zero through 8,192 bytes
  was enumerated and append-completes to exactly 8,192 bytes before terminal
  rollback; an injected crash may leave only another exact prefix;
- controlling conformance authority requires the full finite product with
  independent old/new/promotion/lifecycle stages, finals, and marker states;
- 92 mutable-subject local Markdown targets exist; and
- `git diff --check` and the documentation-only scope check pass.

The next review must use another different fresh isolated read-only reviewer
over a new exact complete-subject manifest. This proof grants no implementation
or HCM-2.3 authority.
