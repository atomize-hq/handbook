# HCM-2.1 Planning Review 2 Remediation

## Review evidence

- Reviewer: `/root/hcm_2_1_planning_review_2`
- Built-in type: `default`
- Fresh/isolated/read-only: yes
- Dispatch:
  `docs/specs/handbook-contract-membrane/handoffs/dispatches/20260718T212159Z--HCM-2-1--fresh-planning-review-2.json`
- Reviewed subject:
  `sha256:5e98cbf3c421b95f05235d66136638a5422e0e889acd45a3735c1849ba9eb4f9`
- Verdict: `CHANGES_REQUIRED`
- Findings: three Required; no Critical, Optional, or Nit finding

The parent accepted all three findings after rereading the exact packet and live
source. Remediation changed planning/control documentation only.

## Finding dispositions

### R2-01 — Non-empty-list key framing contradiction

**Validated.** The closed emitter said every key used colon plus one space, then
required non-empty sequences to start on the next line and prohibited trailing
spaces. The literal golden correctly used colon plus immediate LF, so the prose
was internally contradictory.

**Remediation.** Scalar and empty-list keys now use `: `; non-empty-list keys use
`:` followed immediately by LF. The closed emitter, literal golden, test plan,
and proof gate now agree.

### R2-02 — Freshness input exceeded live encoding and allowed scope

**Validated.** Live `freshness.rs` hashes path, presence, content SHA-256, and
starter state under `reduced-v1-m8`; it does not hash source byte length. The
packet both claimed source length as an input and excluded that owner file,
without authorizing a schema/generation transition.

**Remediation.** The packet preserves the live `reduced-v1-m8` encoding and
manifest generation `1` without editing `freshness.rs`. Project Context uses the
selected YAML path, presence, source SHA-256, and existing starter-match flag;
source length and all rendered values are explicitly excluded from freshness.
Source length remains visible in the manifest/source-summary domain and rendered
length remains the Project Context budget domain.

### R2-03 — Nonexistent author JSON refusal surface

**Validated.** `author project-context` has no JSON flag or serializable JSON
refusal contract; only human text/stderr and process outcome/exit are live.
Doctor retains its actual JSON and text surfaces.

**Remediation.** The platform contract now requires exact author human-text/
stderr, typed outcome/category, exit, and no-filesystem-delta proof only. It adds
no author JSON mode and does not widen CLI grammar. Doctor JSON/text and flow
projection proof remain unchanged.

## Re-review gate

Review 2's immutable dispatch remains unchanged. The parent must replay all
planning checks, bind the complete repaired subject including both review
dispatches and both remediation proofs, write a new immutable dispatch, and use
a third different fresh isolated read-only built-in `default` reviewer. No
primary commit is permitted until a complete-subject reviewer returns `CLEAN`.

## Post-remediation replay

After the last contract edit, `git diff --check`, unchecked-task, absolute-path,
docs-only scope, and the nine-file/20-link Markdown checks passed. The handoff
validator passed with 42 records, 166 current dispatches, eight admitted legacy
dispatches, and 42 ledger entries; both historical-admission and orchestration-
contract self-tests passed. Focused current baselines also passed: engine author
core 9, compiler author 47, setup 11, doctor 3, flow resolver core 15, and CLI
author 22. These remain pre-implementation baseline evidence only.
