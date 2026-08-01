# HCM-0.8 v1.4 same-parent resumption validation repair

Date: 2026-08-01

Status: selected protocol-repair packet; implementation requires planning CLEAN

## Identity and authority

- Parent orchestration:
  `20260801T022300Z--HCM-0-8--v1-4-same-parent-resumption-validation-repair`
- Integrated outcome:
  `hcm-0.8-v1.4-same-parent-resumption-validation-repair`
- Packet:
  `20260801-v1-4-same-parent-resumption-validation-repair`
- Outcome-registry fingerprint:
  `sha256:5394b405a66fe6ae99acad53b390cb8f39663128314fd2e7b5411c3bdbedf6a4`
- Causal budget:
  `sha256:f79081aff048f1b6891fe332525a7fe50a68dc73fee59e4b9882ea6c4117c8e0`
- Baseline product primary:
  `2609bf52569a8ee2e24e10d168f16f65006bdb97`

This is a separate HCM-0.8 orchestration-protocol repair. It does not reopen,
amend, or re-review the HCM-2.4 product subject. The two smoke authority-stop
records and all six smoke dispatches remain byte-immutable evidence.

## Reproduced contradiction

Current v1.4 population validation always treats a handoff as terminal. The
operator-authorized same-parent smoke resumption therefore exposes two records
to the predecessor validator's terminal-only rule:

1. `20260731T194418Z--HCM-2-4--orchestration--post-phase-2-smoke-repair-authority-blocked`
   sees the later proof-gap and final-complete-subject dispatches;
2. `20260731T221742Z--HCM-2-4--orchestration--post-phase-2-smoke-repair-second-authority-stop`
   sees those same two later dispatches.

The ordinary validator reports the first violation and stops. Under the
repaired link-local rule, however, the second authority stop's direct dual link
through both `source_handoff_ids` and `supersedes` makes the first stop a valid
immutable interim prefix. Before the smoke terminal successor is added,
ordinary validation must therefore fail exactly once at the latest
unsuperseded second stop because the proof and final-review dispatches are
later than its cutoff. That command remains non-zero and must never be
described as GREEN. The focused synthetic regressions and both self-tests must
nevertheless pass at the enabling-primary boundary.

## Acceptance contract

1. A completed v1.4 handoff is always terminal for its orchestration.
2. A non-completed handoff remains a fail-closed cutoff unless a later
   same-orchestration v1.4 handoff directly names it in both
   `source_handoff_ids` and `supersedes`.
3. A successor preserves exact `program_id`, `phase_id`, `slice_id`,
   `packet_id`, and `orchestration_id`, plus the complete integrated-outcome
   and causal-budget identity set derived from the dispatch prefix.
4. Every interim record validates exactly the dispatch prefix at or before its
   cutoff. It neither owns nor may omit a dispatch in that prefix.
5. The latest unsuperseded record owns the complete parent dispatch population
   and rejects every later dispatch.
6. Orphan later dispatches, missing one side of the direct source/supersedes
   pair, branching or broken chains, cross-parent successors, identity drift,
   and any handoff or dispatch after completion fail closed.
7. No historical handoff or dispatch is edited, deleted, reparented, reissued,
   or timestamp-adjusted.

## Selected implementation shape

Three mechanisms were considered:

1. Relax every non-completed cutoff whenever later dispatches exist. Rejected:
   orphan dispatches would become valid without a durable successor.
2. Add a schema field or new handoff version. Rejected: the existing direct
   `source_handoff_ids` plus `supersedes` pair already carries the required
   intent, and schema/version expansion is outside this repair.
3. Validate an additive same-orchestration v1.4 successor chain using existing
   record fields, and validate each record against its timestamp-bounded
   dispatch prefix. Selected as the smallest fail-closed repair.

The validator may factor private helpers for chain discovery, identity
derivation, or prefix validation inside the selected Python file. No public
schema, template, CLI, runtime, dependency, or product API changes are allowed.

## Exact path scope

Behavioral production/control changes are exactly:

1. `docs/specs/handbook-contract-membrane/08-handoff-ledger-and-escalation-protocol.md`
2. `docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py`

Validator regression tests are explicitly selected only inside
`run_v1_4_causal_contract_self_test` in the existing validator file. No
separate test path is authorized.

Control evidence may add only this selector, new-parent v1.4 review dispatches,
one compact protocol proof record if needed, one protocol-repair completed
handoff, the smoke completed handoff, and deterministic ledger rebuilds.
No `09` inventory edit is authorized unless a fresh review emits a validated
unfixed P3/P4.

## Required RED and GREEN regressions

RED must reproduce the live ordinary cutoff failure and focused synthetic
failure for a valid direct same-parent successor chain.

GREEN self-test coverage must include:

- the current two-stop direct-link shape before terminal closeout, where the
  first stop validates and only the latest unsuperseded second stop rejects
  its later dispatches;
- the current two-stop shape followed by one terminal completed successor;
- exact prefix reconciliation for each interim record;
- completed-terminal rejection for both later handoffs and later dispatches;
- an unsuperseded non-completed cutoff with an orphan later dispatch;
- source-only and supersedes-only broken links;
- branching/non-immediate successor chains;
- cross-orchestration successor linkage;
- drift in each required record identity and in integrated outcome or causal
  budget identity;
- omission or addition in an interim dispatch prefix; and
- a later dispatch after the latest unsuperseded record;
- a later dispatch allowance cannot retroactively invalidate an interim
  record's ancillary-authority prefix;
- each same-parent successor reconciles its cumulative ancillary prefix, with
  omissions and extras inside that prefix rejected; and
- a completed record retains the final-clean-review cutoff and rejects any
  post-review ancillary widening.

The focused regression command may exercise the orchestration self-test or a
private test entry point, but all cases must run through production validation
logic rather than duplicated test-only policy.

## Causal review stages

The new parent freezes one registry and one derived budget before review.

1. `planning`: fresh selector sufficiency discovery.
2. `implementation`: fresh complete implementation/regression discovery;
   FINDINGS receive one consolidated remediation and different-fresh closure,
   with supplemental causal cycles only under the standing v1.4 rules.
3. `proof`: after the implementation closure, a fresh discovery review enters
   through `planned_stage_transition` and assesses ancillary-prefix integration
   against the live two-stop corpus. Only P1/P2 FINDINGS permit consolidated
   remediation and a different-fresh proof closure; CLEAN ends the stage
   without a closure run.
4. `final_closeout`: after the smoke closeout converges ordinary validation,
   a different-fresh reviewer assesses the complete protocol subject and live
   corpus before the protocol handoff is written.

No review stage or budget is borrowed from the smoke orchestration. Reviewers
are read-only built-in fresh agents. Every dispatch must pass exact v1.4
verification before execution.

## Authorized four-commit dependency sequence

### Commit 1 — existing enabling protocol primary

Commit `e2cb4e1f7ad3d8f12f651a2b09d7d69bbbd49569` is the landed first
enabling commit. It contains the reviewed selector, protocol text, initial
validator implementation/tests, and planning/implementation review lineage.
It remains enabling evidence, not a completed closeout.

### Commit 2 — ancillary-prefix enabling commit

Commit only this selector amendment, cutoff-bounded ancillary reconciliation,
its embedded regressions, and the proof-stage discovery dispatch plus a
different-fresh closure dispatch only if P1/P2 remediation is required. This
second enabling boundary must satisfy:

- focused regressions and both self-tests pass;
- ordinary validation has exactly one known logical smoke cutoff failure at
  the latest unsuperseded second authority stop and remains non-zero;
- no protocol completed handoff or completed claim exists.

### Commit 3 — smoke mechanical closeout

Using the landed enabling validator, create one HCM-2.4 completed handoff that:

- keeps `2609bf52569a8ee2e24e10d168f16f65006bdb97` as both reviewed product primary
  and repository head in the reviewed state;
- directly consumes and supersedes only
  `20260731T221742Z--HCM-2-4--orchestration--post-phase-2-smoke-repair-second-authority-stop`;
- reconciles exactly all six existing smoke dispatches and their one existing
  causal budget;
- records no new smoke dispatch or product review;
- changes only the new smoke handoff and deterministic ledger; and
- passes ordinary validation plus both self-tests before commit.

### Commit 4 — protocol mechanical closeout

After commit 3, create and verify the final-closeout protocol review dispatch
against the now-converged live corpus. CLEAN permits one protocol handoff that
reconciles every new-parent dispatch and one deterministic ledger rebuild.
The fourth commit may contain only that final review dispatch, the protocol
handoff, the ledger, and an exact `09` registration if actually required.
Ordinary validation and both self-tests rerun before commit.

The protocol repair is not complete until commit 4 lands. The two enabling
commits form the reviewed protocol primary stack; the smoke terminal successor
then makes corpus-wide validation possible before final protocol closeout.

## Non-goals and stop conditions

- No HCM-2.4 product source, test, selector, proof, control row, or reviewed
  primary change.
- No history edit, new smoke dispatch, dispatch reparenting/reissue, timestamp
  fabrication, schema/template/version change, dependency, HCM-3.x work,
  release, publication, or push.
- Stop if semantics require another behavioral path, schema change, history
  rewrite, or relaxation that admits orphan/terminal resumption.
- Stop if any protected path hash changes or any protected path is staged.
- Stop if final review is not CLEAN after the permitted causal remediation
  allowance.

## Verification wall

- focused RED/GREEN successor-chain regressions;
- `--self-test-v1-admission`;
- `--self-test-orchestration-contract`;
- ordinary validation at the exact temporary and converged boundaries;
- JSON duplicate-key/schema/record-dispatch-ledger parity;
- Python syntax/compile validation;
- `git diff --check` and manifest whitespace replay;
- GitNexus impact before every validator symbol edit and change detection
  before each commit;
- exact hashes for `2609bf5`, both smoke authority stops, all six smoke
  dispatches, and all nine protected paths.
