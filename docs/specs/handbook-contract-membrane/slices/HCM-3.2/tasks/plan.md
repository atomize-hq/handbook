# HCM-3.2 Authority-Stop Plan

## Overview

Preserve the independently reviewed HCM-3.2 planning state and stop before
implementation because the live contracts do not supply the trusted admission
authority required by the selected envelope/transition kernel.

## Architecture decisions

- Generalize the existing stack loader while preserving exact shipped bytes and
  fingerprints.
- Resolve envelopes only from an exact `ResolvedInstanceProfile` and supplied
  parent record; no ambient or latest lookup.
- Keep mutation matching pure and fail closed. Child authority is intersection,
  and unprovable selector containment refuses.
- Represent memory/validation outcomes and escalation/promotion transitions as
  typed owner-crate values; no persistence or adapter layer.
- Preserve L0-L3 pipeline behavior unchanged and defer its real migration to
  HCM-3.5.

## Ordered packets

### P1 — Planning and selector freeze

- Write SPEC, plan, todo, and exact selector.
- Freeze one outcome registry, ceilings, public names, tests, proof wall, and
  non-goals.
- Obtain schema-valid fresh planning discovery review and, if needed,
  consolidated remediation plus different-fresh closure.

Checkpoint: planning discovery is durably closed as an authority stop before
any Rust edit.

### P2 — Stack and envelope kernel

Blocked. Do not execute this packet until exact reviewed resumption authority
closes the admission and CRITICAL-impact decisions recorded in the selector.

- Write RED tests for configurable stack admission and six-dimension envelope
  behavior.
- Run GitNexus upstream impact for each existing symbol before editing it.
- Generalize stack validation without changing shipped identity.
- Add the namespaced kernel, matcher, typed decisions, transition admission,
  deterministic fingerprints, and exact exports.
- Iterate focused RED/GREEN/REFACTOR and preserve HCM-3.1/profile replay.

Checkpoint: focused engine tests and immutable-definition audit pass.

### P3 — Compatibility, proof, and control closeout

Only blocked-state proof, review, handoff validation, and local durable
closeout are executable. Product proof and completed-slice publication are not.

- Replay unchanged pipeline L0-L3 scoped-filtering behavior.
- Run affected crate and complete workspace proof walls, strict Clippy/format,
  validators, and path/scope audits.
- Obtain implementation discovery review and causal closure if required.
- Run proof-stage and aggregate final-closeout reviews under the same outcome
  registry and budget.
- Run GitNexus scoped and compare-to-main change detection.
- Commit reviewed primary state, write/validate the v1.4 parent handoff and
  ledger, commit mechanical closeout, and atomically publish only the local
  integration ref.

Checkpoint: a schema-valid v1.4 true-stop handoff records the exact authority
boundary, remote remains unchanged, and no push occurs.

## Dependencies

P1 precedes P2. P2 precedes P3. Overlapping implementation agents are not
permitted. Reviewers are fresh, isolated, and read-only.

## Risks and mitigations

| Risk | Mitigation |
|---|---|
| No trusted resolver authenticates authority-bearing refs | Stop and request exact admission-owner/schema/resolver authority |
| Generalization changes shipped identity | Pin exact ref/fingerprint and byte replay before and after |
| Child selector widens authority | Conservative containment plus parent/local intersection and deny precedence |
| One level becomes an aggregate score | Compare all six domains independently in tests and APIs |
| Transition record self-authorizes | Separate request/disposition types and one-terminal admission registry |
| HCM-3.5 adoption leaks into this slice | No pipeline/flow source edit; run existing L0-L3 tests only |
| Public API grows without proof | Require exact callable signatures, counting rule, and misuse/export audit before resumption |
| Existing stack edits exceed risk ceiling | Require explicit reviewed CRITICAL-symbol posture before resumption |
| GitNexus FTS/compare unavailable | Record unavailable honestly; never label GREEN |

## Definition of done

For this non-completed stop, all discovery findings are accurately dispositioned,
the revised planning subject receives different-fresh closure, no Rust or
implementation check occurs, a committed v1.4 authority-stop handoff and ledger
validate, remote baseline remains unchanged, and the terminal receipt identifies
the exact authority needed to resume HCM-3.2.
