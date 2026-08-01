# HCM-0.8 repository-local meta-orchestration adapter selector

Date: 2026-08-01

Status: selected administrative control-plane packet; implementation requires
planning CLEAN

## Identity and authority

- Parent orchestration:
  `20260801T190437Z--HCM-0-8--meta-orchestration-adapter`
- Integrated outcome: `hcm-0.8-meta-orchestration-adapter`
- Packet: `20260801-meta-orchestration-adapter`
- Outcome-registry fingerprint:
  `sha256:f7d819ec3d054cf484740c7bfb707f650c805a6f99b537f08f70ff4126ba193c`
- Causal budget:
  `sha256:3d876985f0b838387e7b27a8488c1f251461323f4aebb6b451583ece65ce8d04`
- Baseline: `9f5b072b15489f0037e1ea8946a31bd72a9876ed`

The operator explicitly selected this bounded HCM-0.8 adoption packet. It
admits the repository-local `orchestrate-top-level-tasks` skill and clarifies
how a separately authorized meta orchestrator schedules fresh slice runners
without weakening Handbook's existing v1.4 authority, causal review, or
true-stop rules.

## Exact semantic boundary

1. One increment orchestrator owns exactly one selected slice or genuine
   same-slice true-stop resumption. It performs the live `07`/`08` workflow,
   writes the parent-owned v1.4 handoff, closes the ledger, reports its commits,
   and stops. It never launches a sibling slice.
2. A separately operator-authorized meta orchestrator may create the next fresh
   top-level increment only when the operator preauthorized a closed ordered
   sequence of exact slice IDs (which may contain one slice), and it has
   independently verified the prior increment's
   completed handoff, ledger, local commits/ref state, protected paths, review
   cadence, and remaining authority. The meta orchestrator cannot derive,
   reorder, or extend that sequence.
3. Meta receipts and runtime state are external scheduling evidence. They are
   not Handbook dispatch, handoff, ledger, selector, review, or product
   authority and do not enter the v1.4 dispatch population.
4. A receipt never authorizes work by itself. Every new slice still requires
   its own live dependency proof and reviewed selector. A same-slice resumption
   preserves the existing parent/outcome-derived causal lineage.
5. Meta adjudication is limited to corrections and resumptions already inside
   the selected slice, causal budget, risk ceiling, and user-authorized
   sequence. It cannot expand scope, invent a slice, rewrite immutable
   evidence, waive P1/P2, reset a review budget, push, or start work outside the
   preauthorized envelope.

## Exact path ceiling

The reviewed primary packet may change only:

1. `.codex/skills/orchestrate-top-level-tasks/**`;
2. `docs/specs/handbook-contract-membrane/07-orchestration-onboarding-prompt.md`;
3. `docs/specs/handbook-contract-membrane/08-handoff-ledger-and-escalation-protocol.md`;
4. this selector; and
5. new v1.4 review dispatches required by this parent.

Mechanical closeout may add only one completed HCM-0.8 v1.4 handoff and the
deterministically rebuilt `handoffs/ledger.jsonl`. An exact `09` registration
is allowed only if fresh review emits a valid unfixed P3/P4.

No `00`-`06` product/feature contract, HCM-3.x authority, handoff/dispatch
schema, validator, Rust source, test, fixture, dependency, version, release,
remote ref, or historical record/dispatch is selected.

## Causal review plan

This parent freezes one outcome and one derived budget before review.

1. `planning`: one complete selector discovery review. P1/P2 findings receive
   one consolidated remediation and a different-fresh closure.
2. `final_closeout`: after the skill and both protocol clarifications validate,
   one complete-subject discovery review covers the exact primary packet.
   P1/P2 findings receive consolidated remediation and a different-fresh
   closure; only directly caused or unmasked findings may use the standing two
   supplemental causal allowances.
3. CLEAN ends each stage. Mechanical handoff/ledger closeout is not a review
   cycle and cannot reset or extend the budget.

No separate implementation or proof review stage is necessary because this is
a documentation/developer-tooling adoption with deterministic skill tests and
no product/runtime behavior. The final complete-subject review owns both
semantic integration and proof sufficiency.

## Acceptance

- the repository-local skill passes its self-test, receipt/state/evidence
  validation examples, prompt rendering, YAML metadata validation, Python
  compile/lint/format checks, JSON parsing, and whitespace checks;
- `07` states that its increment runner always true-stops and does not launch a
  sibling, while recognizing the separately authorized external meta scheduler;
- `08` makes receipts scheduling-only, excludes them from v1.4 populations and
  authority, preserves causal lineage, and bounds meta dispatch/adjudication;
- ordinary handoff validation and both self-tests pass unchanged;
- the final primary subject receives fresh CLEAN review with no unresolved
  P1/P2 and any P3/P4 disposed under `09`;
- GitNexus change detection is run and unavailable capabilities are recorded as
  unavailable rather than GREEN;
- all nine protected user-owned paths remain byte-identical, unstaged, and
  uncommitted; and
- two local commits land the reviewed primary and mechanical closeout without
  push or HCM-3.x work.

## Stop conditions

Stop instead of expanding this packet if review or validation requires a
schema/validator change, product/runtime edit, new trust or publication model,
history rewrite, broader phase-map change, more than the selected protocol
clarification, or any protected-path mutation.
