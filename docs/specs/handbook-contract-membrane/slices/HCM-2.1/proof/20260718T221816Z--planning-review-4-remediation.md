# HCM-2.1 Planning Review 4 Remediation

## Review evidence

- Reviewer: `/root/hcm_2_1_planning_review_4`
- Built-in type: `default`
- Fresh/isolated/read-only: yes
- Dispatch:
  `docs/specs/handbook-contract-membrane/handoffs/dispatches/20260718T220154Z--HCM-2-1--fresh-planning-review-4.json`
- Reviewed subject:
  `sha256:cb9eb56cad3a286270815d2610f893f3e7c095bebe01219bfa86e585f1f8065e`
- Verdict: `CHANGES_REQUIRED`
- Findings: two Required; no Critical, Optional, or Nit finding

The parent accepted both findings after direct inspection of the exhaustive CLI
setup reason match and engine identity/manifest/freshness carrier chain.

## Finding dispositions

### R4-001 — CLI setup reason renderer necessarily changes

**Validated.** The packet adds three public `ArtifactInspectionReason` variants,
and live CLI setup exhaustively matches that enum without a wildcard. The prior
scope treated setup as a test-only surface, so the required enum addition would
fail compilation or omit the new exact names.

**Remediation.** The packet now authorizes only three CLI setup match arms:
`typed_decode_failed`, `rendered_view_refused`, and
`observation_changed_during_inspection`. Focused proof freezes all other setup
human output, command behavior, decisions, and exits byte-for-byte. Compiler
setup remains test-only absent a separately proven current defect.

### R4-002 — Selected path could not enter static engine identity/freshness

**Validated.** Live `CanonicalArtifactIdentity.relative_path`, ingest issues,
and baseline validation expose static paths, while the selected descriptor path
is runtime-owned. C03 freshness sorts/encodes that identity path directly.
Forbidding a leak/static adapter while freezing all `freshness.rs` edits left no
implementable carrier.

**Remediation.** The packet now converts identity, ingest-issue, and baseline-
validation paths to owned `String`; manifest continues to own cloned identities.
Static sibling descriptor/layout inputs convert once and Project Context uses
its selected path. `freshness.rs` is authorized only for borrow/as-str ownership
adaptation. Encoder order, values, length prefixes, sort/issue order, constants,
generation, and SHA-256 remain unchanged, with independently reconstructed
fixed-sibling before/after and selected-path preimage goldens.

## Re-review gate

Review 4's dispatch remains immutable. The parent must replay every planning
gate, bind all four prior dispatches and remediation proofs in a new complete
manifest, and use a fifth different fresh isolated read-only built-in `default`
reviewer. No primary commit is permitted before a complete-subject `CLEAN`.

## Post-remediation replay

Diff, unchecked-task, absolute-path, docs-only scope, and the eleven-file/22-link
Markdown gates passed. Handoff validation passed with 42 records, 168 current
dispatches, eight admitted legacy dispatches, and 42 ledger entries; both self-
test modes passed. Focused current baselines passed: engine author core 9,
compiler author 47, setup 11, doctor 3, flow resolver core 15, and CLI author 22.
They remain pre-implementation baseline evidence only.
