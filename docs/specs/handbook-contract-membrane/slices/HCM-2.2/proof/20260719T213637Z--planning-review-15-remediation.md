# HCM-2.2 Planning Review 15 Remediation

- Recorded at: `2026-07-19T21:36:37Z`
- Review dispatch: `docs/specs/handbook-contract-membrane/handoffs/dispatches/20260719T211056Z--HCM-2-2--fresh-planning-review-15.json`
- Reviewed subject: 59 exact paths, aggregate `sha256:ce06e06c8ab5dd0bc81d414402a8148418deb9cc50f2d39871386ebcf9b93df8`
- Fresh reviewer: `/root/hcm_2_2_planning_review_15`
- Review verdict: `CHANGES_REQUIRED`
- Commit authorization: none

## Required findings accepted

1. Three changed schema files were pretty-printed while their registry entries
   bound minified sorted JSON plus LF, so the closure did not bind literal
   contract bytes.
2. The normative approval-policy exact field list omitted
   `authenticator_get_assertion_response_schema_ref` even though the machine
   record carried it.
3. The flag negatives proved RFU, AT, ED, UV, and trailing-data refusal but did
   not isolate backup eligibility or valid backup eligibility plus state.
4. Status representatives froze code/retryability but did not carry exact Admin
   next actions, refused Admin results admitted empty actions, and one published
   authorization result contradicted the table.
5. Status `0x3f` (`CTAP2_ERR_UV_INVALID`) was non-retryable authority refusal
   despite the cited CTAP table directing the platform to retry.

## Remediation

- Rewrote the three schema files to their already-bound literal minified sorted
  JSON plus LF bytes and added a direct raw-byte requirement to the SPEC.
- Added the omitted GetAssertion-response schema-ref field to the normative
  exact approval-policy key set.
- Added validly signed `0x0d` BE and `0x1d` BE+BS negatives and narrowed the RFU
  vector wording; the raw-response negative set is now 18.
- Added exact non-empty next actions and Admin projections to every status
  representative, required at least one next action in every refused Admin
  branch, and aligned the published `0x2e` Admin result with the table.
- Added retryable `authenticator_user_verification_retry` for `0x31`, `0x33`,
  `0x36`, `0x3b`, and `0x3f`; published a named-status source audit and explicit
  reserved/extension/vendor unknown classes.
- Recomputed the changed Admin schema-registry entry and every dependent
  approval/waiver/lifecycle/kind/intake/profile fingerprint. The approval
  definition now binds the changed status fixture raw hash.

## Proof replay

The parent replayed the complete proof wall after remediation. The exact-schema
closure passed for all eleven registry entries, including literal raw-byte
equality with minified sorted JSON plus LF for every schema. Eleven definition/
JCS vectors, fourteen runtime identities with seven published-schema
validations, thirteen Admin positives, seventeen Admin negatives, five raw
assertion successes, eighteen signed/raw assertion negatives, four credential-
selection vectors, all 33 security bounds, all 17 profile/nine security source
bindings, final-use/waiver/user-handle/status mapping cases, exact intake/raw
identity, locks, task/diff/scope/link checks, and all handoff modes passed. The
status table covered every received `0x01..0xff` byte exactly once, carried the
exact next action and Admin projection for every representative, source-audited
all named statuses, and kept reserved/extension/vendor bytes explicitly
classified. Focused Cargo baselines passed `1`, `8`, `49`, `14`, `17`, and `22`
tests respectively. The changed-path scope contained exactly 61 paths; 23
Markdown files and 66 relative links passed. Handoff validation passed 45
records, 198 current internal dispatches, eight legacy dispatches, and 45 ledger
rows. No product or definition implementation was executed.

This replay does not authorize a commit. The parent must dispatch the expanded
exact-path subject to a new fresh independent reviewer.
