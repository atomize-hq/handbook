# HCM-3.5 Resolution-aware adoption planning ledger

## Completed planning evidence

- [x] Verify the assigned base/ref/tree/ancestor, clean task worktree, local
  origin baseline, ownership, and protected-checkout baselines before edits.
- [x] Validate the selected completed HCM-3.4 source-pair handoff against the
  live v1.4 ledger and ordinary validator rather than assuming its claim.
- [x] Inspect the landed HCM-3.4 private Snapshot Memory/source-pair code,
  Flow resolver, pipeline scoped-inclusion call path, and v1.4 handoff fields.
- [x] Record the existing engine dependencies for Flow and pipeline and the
  absence of any non-engine Snapshot Memory consumer.
- [x] Correct the stale HCM-3.4 Snapshot Memory classification in the smallest
  coupled control-pack surfaces.

## Required before planning completion

- [ ] Receive an explicit public compatibility and gate-operation decision.
- [ ] Freeze an implementation selector that names exact exported engine and
  Flow/pipeline operation seams, compatibility posture, and risk ceiling.
- [ ] Re-run live source/dependency/impact analysis against that selector.
- [ ] Obtain implementation/proof review and prove every conditional task in
  `plan.md`; do not claim this planning outcome completed before then.

## Current closeout work

- [ ] Obtain a fresh discovery review of this complete authority-boundary
  documentation subject.
- [ ] Apply one consolidated remediation only if a valid P1/P2 is found, then
  obtain a different-fresh delta closure. Do not reopen discovery or run a
  review after CLEAN.
- [ ] Commit the reviewed authority-boundary documentation state, then commit
  only the v1.4 handoff/ledger closeout surface separately.

## Explicit non-goals

- [ ] No Rust, tests, fixtures, product checks, schema, Cargo, dependency,
  public API, SDK, CLI, transport, workflow/configuration, or consumer change.
- [ ] No HCM-3.4 evidence rewrite, HCM-3.6/HCM-4+/HCM-5 selection, remote
  operation, protected-checkout mutation, merge, rebase, reset, clean, or
  force-update.
