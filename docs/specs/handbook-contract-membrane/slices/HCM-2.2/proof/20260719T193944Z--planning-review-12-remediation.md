# HCM-2.2 Planning Review 12 Remediation

## Review authority

- Reviewer: `/root/hcm_2_2_planning_review_12`
- Dispatch:
  `20260719T191204Z--HCM-2-2--fresh-planning-review-12.json`
- Reviewed subject: 49 paths,
  `sha256:1290b9016edc93e78f5ab0e8b156629d21fb3327226b463c37704748e66e2dd4`
- Result: `CHANGES_REQUIRED`
- Required findings: 3
- Optional findings: 0
- Nit findings: 0

Review 12 authorizes no commit.

## Exact remediation

1. **Engine-owned bootstrap mappings.** Removed `approval_mappings` from the
   bootstrap request branch and positive vector. The engine now derives the
   bootstrap credential's unique sorted mappings from the caller's quorum plus
   the fixed registry-admin pair. Published supplied-bootstrap-mapping refusal
   and a separate add-mapping one-over negative, yielding twelve positives and
   sixteen mandatory negatives.
2. **No terminal-use admin lockout.** Defined usable as active with use sequence
   below 4096. Sequence 4095 is final-use-only: its next assertion may commit
   only when the same authority result leaves every required pair covered by a
   distinct usable credential. A same-transaction `add_credential` may count
   its new verified credential only at commit. Froze ordinary final-use refusal,
   replacement success, alternate-admin replacement, and crash/recovery proof.
3. **Canonical unique waiver acceptance.** Added schema-level challenge
   fingerprint uniqueness, engine-level duplicate ref/fingerprint refusal,
   exact approval-entry tuple order, exact challenge-fingerprint byte order,
   and pre-hash refusal for unsorted serialized challenges. Published two-waiver
   forward/permuted normalization and four duplicate/order negative vectors.

The admin and challenge schema changes regenerated all ten schema entries,
eleven definition/JCS vectors, the approval policy's exact engine-bootstrap and
waiver-order semantics, every downstream approval/waiver/lifecycle/kind/intake/
profile fingerprint, the intake's own fingerprint/raw SHA-256, and the closure
table. Runtime record and transcript identities remain unchanged. No product
implementation was performed.

## Required replay before Review 13

The parent must reproduce ten schemas/entries, eleven definition/JCS vectors,
thirteen runtime identities and six schema validations, twelve admin positives
and all sixteen negatives, bootstrap-without-mapping and mapping-injection
refusal, all 30 limit groups, two waiver permutation normalizations and four
order/duplicate negatives, two user-handle derivations, seven transcripts,
strict CBOR/P-256 and registry-resolved DER ES256, final-use/no-lockout cases,
shared cross-family recovery, all 16 profile sources, exact intake/raw identity,
links/tasks/diff/scope, all handoff modes, dependency object, retained tests,
and focused baselines. It must seal a new immutable subject and use a different
fresh read-only reviewer. Only later `CLEAN` over unchanged bytes can authorize
the primary planning commit.
