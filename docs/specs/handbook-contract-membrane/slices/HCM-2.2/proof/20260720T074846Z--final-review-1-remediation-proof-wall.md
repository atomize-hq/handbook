# HCM-2.2 Final Review 1 Remediation Proof Wall

## Rejected subject and review result

- Rejected dispatch:
  `20260720T065241Z--HCM-2-2--fresh-final-implementation-review`
- Rejected subject: 171 exact paths
- Rejected aggregate:
  `sha256:eec48427216f45719a050ef995f6cbad0f1171f292bfac7d9606b27bceb84ca1`
- Fresh reviewer: `/root/hcm_2_2_final_review`
- Verdict: `CHANGES_REQUIRED`
- Findings: one Critical and one Required

The reviewer independently admitted all 171 hashes twice, replayed the exact
aggregate, reproduced both findings, and made no repository change. The rejected
subject authorizes neither a commit nor closeout.

## Critical authority finding

The public `CharterAuthorityTransactionServiceV1` exported caller-constructible
`CharterPromotionRequestV1`, a public direct `promote`, and public fault
injection. That low-level preflight validated loose content-addressed records
but did not establish the complete committed registry/approval journal,
repository-identity, current quorum/pair, and waiver authority enforced by
`CharterPromotionWorkflowServiceV1`. An external caller could therefore bypass
the workflow and commit canonical Charter truth from forged or orphan lineage.
The existing transaction integration test reproduced that unauthorized success
in a fresh repository without repository identity or committed approval and
registry journals.

GitNexus reported LOW graph impact for the low-level service and methods and LOW
impact for the request type (23 impacted symbols, four direct callers, one
workflow process), but the reproduced semantic authority risk was Critical.

## Required durability finding

The promotion writer and lifecycle reader agreed on the development path
`.handbook/state/promotion-transactions`, while the frozen protocol requires
`.handbook/state/transactions/promotions/<id>.<state>`. Their internal agreement
made tests pass but could not prove interoperability with conforming recovery
and observation tooling.

GitNexus classified the root helper as Critical: 16 upstream symbols, eight
direct callers, and five affected flows spanning retained authority, committed
reads, pending recovery, and promotion. It classified the lifecycle anchor as
High: 11 upstream symbols, one direct caller, and three lifecycle flows. The
unchanged exact 72-byte marker helper was Critical by reachability: 17 upstream
symbols, four direct callers, and eight flows.

## Test-first bounded repair

The external compile-fail proof was RED because outside code could import the
forgeable request/fault types and invoke both low-level mutators. The repair:

- makes the transaction module private;
- keeps only the committed-authority observer, commit/result/error types, and
  safe constructor public;
- makes request and retained mutation surfaces crate-private and compiles direct
  low-level mutation/fault methods only for in-crate tests;
- routes all production mutation through
  `CharterPromotionWorkflowServiceV1` and its retained complete authority checks;
- migrates marker, crash, partial-install, recovery, concurrency, replay,
  substitution, and amendment-chain tests into a private in-crate module;
- replaces the external low-level mutation integration test with proof that
  loose/orphan records do not become authority and forged canonical bytes
  without a committed workflow journal refuse; and
- changes writer, recovery, lifecycle anchor, compiler/CLI fixtures, and tests
  to the one exact `.handbook/state/transactions/promotions` root. No legacy
  development path is admitted as authority.

The compile-fail proof then turned GREEN. No public, doc-hidden, feature-gated,
or test-support mutation backdoor was added.

## Remediation verification

| Command or suite | Result |
|---|---|
| external low-level promotion compile-fail doctest | RED before sealing; GREEN 1 / 1 after |
| private transaction marker/atomicity/fault/recovery/concurrency suite | PASS, 9 / 9 |
| external loose/orphan/forged authority observer | PASS, 1 / 1 |
| lifecycle path, observation, event, and recovery interoperability | PASS, 6 / 6 |
| Charter observation | PASS, 2 / 2 |
| promotion workflow | PASS, 9 / 9 |
| compiler HCM-2.2 product path | PASS, 3 / 3 |
| flow HCM-2.2 product path | PASS, 4 / 4 |
| five directly affected CLI paths | PASS, 5 / 5 |
| complete `handbook-engine` tests and doctests | PASS, including seven compile-fail doctests |
| `cargo test --workspace --all-targets --all-features` | PASS on remediated complete subject |
| strict workspace Clippy | PASS |
| `cargo fmt --all -- --check` and `git diff --check` | PASS |
| engine package/member replay | PASS, 167 members, 164 source-identical, 62 exact definitions |

One migrated lifecycle fixture initially carried a stale result-state
fingerprint and failed 3 of 6 tests. The fixture was corrected to derive the
exact current state fingerprint, and the complete lifecycle suite passed 6 / 6.
The first aggregate workspace replay reached the command wrapper timeout after
compilation and its closed output pipe aborted Cargo; the immediate cached replay
completed every target successfully.

## Resumption gate

The rejected dispatch and result remain immutable. The parent must rebuild the
complete subject manifest, write a new immutable review dispatch, and use a
different fresh isolated read-only reviewer. No primary commit is allowed until
that exact remediated subject returns `CLEAN`.
