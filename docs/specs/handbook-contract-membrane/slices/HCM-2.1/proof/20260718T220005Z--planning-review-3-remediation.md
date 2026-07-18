# HCM-2.1 Planning Review 3 Remediation

## Review evidence

- Reviewer: `/root/hcm_2_1_planning_review_3`
- Built-in type: `default`
- Fresh/isolated/read-only: yes
- Dispatch:
  `docs/specs/handbook-contract-membrane/handoffs/dispatches/20260718T214122Z--HCM-2-1--fresh-planning-review-3.json`
- Reviewed subject:
  `sha256:719507f473536dd6f700f2330592490989e5918bd84d192eb672435ee4908a94`
- Verdict: `CHANGES_REQUIRED`
- Findings: two Required; no Critical, Optional, or Nit finding

The reviewer explicitly withdrew its preliminary renderer-selection concern
after applying the HCM-2.1 phase authority that permits reuse of the existing
fixed renderer without definition/profile mutation. The parent accepted the two
final findings after direct live-source validation.

## Finding dispositions

### R3-001 — C04 result version disposition missing

**Validated.** Live flow constructs `C04_RESULT_VERSION=reduced-v1-m8.1` and
compiler rendering rejects a mismatched version against its separate constant
in `blocker.rs`. The packet changed the public C04 DTO and JSON shape but froze
only C03 version/generation and did not authorize the compiler C04 owner.

**Remediation.** The packet now advances both flow and compiler C04 constants
exactly to `reduced-v1-m8.2`, authorizes `compiler/src/blocker.rs`, and requires
flow construction, compiler acceptance of `.2`, exact rejection of `.1`, JSON/
shared/inspect/CLI goldens, and fixed-sibling regressions. C03 explicitly remains
`reduced-v1-m8` generation `1`.

### R3-002 — Selected-path ownership chain incomplete

**Validated.** Live budget, flow resolver subject/next-action, compiler subject/
next-action, blocker, refusal, and rendering surfaces carry canonical paths as
`&'static str`. Converting only packet/budget fields to `String` cannot project
the descriptor-selected path without widening the remaining owners or inventing
a static-path adapter/leak.

**Remediation.** The packet now enumerates every path-bearing variant in the
flow budget/resolver and compiler subject/next-action chain, authorizes
`compiler/src/refusal.rs`, and freezes move/clone ownership. Fixed paths call
`.to_owned()` once at the changed DTO boundary; selected paths remain owned.
Leaking, interning, hard-coded Project Context adapters, ambient lookup, and
lifetime coercion are forbidden. Compile/API and exact blocker/refusal/JSON/
inspect/CLI tests cover selected and sibling paths end to end.

## Re-review gate

Review 3's immutable dispatch remains unchanged. The parent must replay all
planning gates, bind the complete subject including all three prior dispatches
and remediation proofs, write another immutable dispatch, and use a fourth
different fresh isolated read-only built-in `default` reviewer. No primary
commit is permitted until a complete-subject reviewer returns `CLEAN`.

## Post-remediation replay

After the last contract edit, diff, unchecked-task, absolute-path, docs-only
scope, and the ten-file/21-link Markdown gates passed. Handoff validation passed
with 42 records, 167 current dispatches, eight admitted legacy dispatches, and
42 ledger entries; both self-test modes passed. Focused current baselines passed:
engine author core 9, compiler author 47, setup 11, doctor 3, flow resolver core
15, and CLI author 22. They remain pre-implementation baseline evidence only.
