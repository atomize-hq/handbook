# Handbook Adapter Contract

## Authority boundary

This skill sits above, not inside, the Handbook v1.4 orchestration protocol. The meta orchestrator
may schedule and verify whole slices and genuine true-stop resumptions. It does not create internal
dispatches, perform implementation, write Handbook handoffs, rebuild the ledger, review its own
increments, or promote receipt content into repository authority.

The precedence inside an increment remains the live repository's `07` authority order. Runtime
meta state and receipts are external scheduling artifacts.

## Unit of work

One sequence entry is one explicitly authorized `PHASE_ID` / `SLICE_ID`, or a named same-slice
true-stop resumption. A fresh task receives:

- `PHASE_ID`;
- `SLICE_ID`;
- `ACTIVE_PACKET`;
- `HANDOFF_SELECTOR`;
- exact expected base commit/tree;
- a closed path/symbol/test/proof/risk envelope;
- protected paths and checkouts;
- exact successor correlation.

Do not split a slice merely to create more top-level tasks. Packet decomposition, proof gaps,
review, causal remediation, and cross-document repairs remain internal unless `07` says the run has
reached a genuine top-level stop.

## Causal review cadence

The local wrapper must preserve the default automatic budget from `07`, `08`, and `09`:

1. one complete-subject discovery review or same-fingerprint burst per typed stage;
2. one consolidated remediation pass;
3. one different-fresh delta-focused closure review;
4. at most two immediately causal supplemental remediation/closure cycles.

Supplementals are only for P1/P2 findings caused or unmasked by the preceding remediation and may
not widen scope, authority, or risk. Closure verifies the known repair and affected proof; it does
not restart general discovery. No cycle may follow CLEAN. Packet, selector, outcome, task, cycle,
or fingerprint renaming cannot reset the parent/outcome-derived budget. Mechanical closeout is
outside the cadence. An exhausted cadence stops non-completed unless a reviewed authority ref
explicitly grants a bounded extension; it never turns P1/P2 into accepted debt.

## Delegated meta adjudication

The user may preauthorize an exact sequence and delegate bounded adjudication. Within that
envelope, the meta orchestrator may launch the next clean slice task without another user turn. It
may also correct or resume a blocked slice when all of these remain unchanged:

- selected slice and integrated outcome;
- repository authority and causal budget;
- public/API/dependency/unsafe posture;
- path and risk ceiling;
- user-authorized sequence.

Escalate when any item changes, an immutable record would need rewriting, a P1/P2 would need a
waiver, native evidence is unavailable, or the sequence must expand. The receipt informs this
decision but never makes it automatically.

## Completion mapping

The receipt's `landed.primary_commits` is the reviewed primary commit sequence in ancestry order.
`landed.primary_tip` is its final commit and must match the completed handoff's reviewed baseline.
`landed.closeout_commit` is the mechanical handoff/ledger closeout and is also the receipt's final
`landed.commit`.

The receipt must reference:

- one completed repository-relative v1.4 parent handoff;
- the final CLEAN dispatch and digest;
- reviewed subject and dispatch-population fingerprints;
- causal-budget/cadence validation and any explicit bounded extension authority;
- `ledger.jsonl` and its digest;
- ordinary validation and both self-tests;
- P3/P4 disposition;
- exact protected-path verification.

The meta independently replays those claims before advancing.

## Local integration ref

Use a dedicated local ref that is not checked out in the protected product workspace. Seed it from
the exact operator-approved local tip, including already-unpushed Handbook commits. Each increment
starts from the ref's exact commit/tree and publishes only with an expected-old compare-and-swap.

Use the closeout commit as the new value and the dispatch base as the required old value:

```text
git update-ref <local-integration-ref> <closeout-commit> <expected-base-commit>
```

Do not omit the old-value argument. A failure is `BASE_DRIFT`, not permission to reconcile.

The remote is observation-only in this mode. Record its starting commit and require it to remain
unchanged. A nonzero local-ahead count is expected; behind must remain zero unless the user grants a
new reconciliation decision. Never merge, rebase, reset, clean, force-update, or push to reconcile
drift.

## GitNexus and protected paths

Every increment must follow live `AGENTS.md`:

- impact analysis before editing any existing function, class, or method;
- explicit HIGH/CRITICAL warning before such edits;
- scoped and compare-to-`main` change detection before the primary commit;
- an unavailable FTS/comparison result recorded as unavailable, never GREEN;
- exact initial/final hashes for every protected user-owned path;
- no staging or committing of protected paths.

The meta verifies those claims from repository truth and the completed handoff.
