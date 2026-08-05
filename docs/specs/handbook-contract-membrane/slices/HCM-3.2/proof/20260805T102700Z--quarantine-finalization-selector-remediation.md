# HCM-3.2 quarantine-finalization selector remediation

Status: bounded planning remediation complete; different-fresh closure is
required before product recovery or implementation.

Parent:
`20260805T101500Z--HCM-3-2--context-resolution-quarantine-finalization`

Outcome: `hcm-3.2-context-resolution-quarantine-finalization`

Dispatch:
`20260805T102305Z--HCM-3-2--quarantine-finalization-selector-remediation`

Remediation agent: `/root/hcm32_qf_selector_remediation`

## Findings and disposition

- `HCM32-QF-SEL-DISC-001` P2 is fully remediated. The SPEC and plan stop on a
  fourth production path, consistently preserving the exact three-path
  production ceiling.
- `HCM32-QF-SEL-DISC-002` P2 is fully remediated. The SPEC requires planning
  CLEAN, one combined implementation/proof review ending CLEAN, and then
  mechanical P4 closeout with no review cycle.

No selector, packet, outcome, registry, authority, product, test, fixture,
validator, schema, template, skill, handoff, or ledger content changed.

## Exact bounded edits

1. Replaced `sixth production path` with `fourth production path` in the SPEC
   and plan stop conditions.
2. Replaced the SPEC's implementation/proof/final-closeout review sequence
   with the selected combined implementation/proof review followed by
   mechanical P4 closeout with no review cycle.

## Remediated planning-authority fingerprint

The acyclic seven-entry planning-authority manifest uses
`repo-path-null-sha256-newline-v1` and has aggregate fingerprint
`sha256:a52fa05711fee4f9f9364feaba6ee99406cbbdf32dddab3738a6316265a554c1`.
This embedded fingerprint excludes this proof so the closure subject can hash
the proof itself without a self-reference. The complete eight-entry closure
manifest and its aggregate are returned to the parent after replay.

## Validation record

- cross-file production-ceiling and cadence scan: PASS;
- strict UTF-8 decoding with no BOM and balanced Markdown fences: PASS;
- sorted manifest replay: PASS;
- `git diff --check`: PASS.

This is remediation evidence, not a review verdict. Planning remains blocked
until a different-fresh delta-focused reviewer returns CLEAN against the
recomputed complete subject fingerprint.
