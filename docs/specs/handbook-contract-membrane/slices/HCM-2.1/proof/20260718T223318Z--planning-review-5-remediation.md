# HCM-2.1 Planning Review 5 Remediation

## Review evidence

- Reviewer: `/root/hcm_2_1_planning_review_5`
- Built-in type: `default`
- Fresh/isolated/read-only: yes
- Dispatch:
  `docs/specs/handbook-contract-membrane/handoffs/dispatches/20260718T222005Z--HCM-2-1--fresh-planning-review-5.json`
- Reviewed subject:
  `sha256:2687de17df820a15d3dfbe507497929e95f55826f546c907667310e10f833bf6`
- Verdict: `CHANGES_REQUIRED`
- Findings: one Required; no Critical, Optional, or Nit finding

Review 5 revalidated all thirteen earlier Required findings and their four
remediation rounds. The parent accepted its remaining author-result finding
after direct inspection of the public compiler author export chain.

## R5-001 — Project Context author result retained a static legacy path

**Validated.** Live `AuthorProjectContextResult` returns
`canonical_repo_relative_path: &'static str`, filled from legacy layout, while a
public fixed Markdown path constant is re-exported through author/module/crate
surfaces. The selected descriptor path is runtime-owned, so the prior packet
left only a prohibited leak/hard-code or an unspecified public type transition.

**Remediation.** The packet now freezes the result field as `String`, supplied
only by clone/move from the selected artifact decision. It deletes the legacy
fixed Project Context path constant and every re-export without replacing it
with a YAML-path alias. RED/API/CLI/file/stdin goldens prove selected-path result
projection and legacy layout/constant irrelevance. All required owner files were
already explicitly allowed.

## Re-review gate

Review 5's dispatch remains immutable. The parent must replay all planning
gates, bind all five prior dispatches and remediation proofs, create a new
immutable dispatch, and use a sixth different fresh isolated read-only built-in
`default` reviewer. No primary commit is permitted before complete-subject
`CLEAN`.

## Post-remediation replay

Diff, unchecked-task, absolute-path, docs-only scope, and the twelve-file/23-
link Markdown gates passed. Handoff validation passed with 42 records, 169
current dispatches, eight admitted legacy dispatches, and 42 ledger entries;
both self-test modes passed. Focused current baselines passed: engine author core
9, compiler author 47, setup 11, doctor 3, flow resolver core 15, and CLI author
22. They remain pre-implementation baseline evidence only.
