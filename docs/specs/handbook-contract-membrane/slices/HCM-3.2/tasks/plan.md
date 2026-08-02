# HCM-3.2 Resumed Implementation Plan

## Overview

Resume the same parent/outcome/packets from its independently reviewed planning
authority stop. Freeze and review the selected HCM-3.2-specific authority
binding/admission design before RED, then implement, prove, independently
review, close, and publish locally without resetting causal lineage.

## Architecture decisions

- Generalize the existing stack loader while preserving exact shipped bytes and
  fingerprints.
- Resolve envelopes only from an exact `ResolvedInstanceProfile` and supplied
  parent record; no ambient or latest lookup.
- Keep mutation matching pure and fail closed. Child authority is intersection,
  and unprovable selector containment refuses.
- Represent memory/validation outcomes and escalation/promotion transitions as
  typed owner-crate values; no persistence or adapter layer.
- Bind every uncached native assertion to fresh engine entropy and one exact
  authority-use subject; permit only in-resolver exact cached replay.
- Validate promotion from the actual resolved source envelope and stack into
  typed semantic-memory target/result values with pure compare-and-write
  semantics; do not implement HCM-3.4 storage.
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

### P2 — Authority admission, stack, and envelope kernel

The product authority selected the bounded HCM-3.2 record/resolver and accepted
the three named HIGH/CRITICAL edits. First amend the existing planning subject
and obtain the one fresh implementation-stage selector-admission discovery
review. Do not create another planning cycle and do not begin RED/Rust before
CLEAN.

- Write RED tests for configurable stack admission and six-dimension envelope
  behavior.
- Run GitNexus upstream impact for each existing symbol before editing it.
- Generalize stack validation without changing shipped identity.
- Add the namespaced kernel, matcher, typed decisions, transition admission,
  deterministic fingerprints, and exact exports.
- Iterate focused RED/GREEN/REFACTOR and preserve HCM-3.1/profile replay.

Checkpoint: focused engine tests and immutable-definition audit pass.

### P3 — Compatibility, proof, and control closeout

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
| Authority binding becomes a generic framework | Keep one closed HCM-3.2 artifact record/resolver; no catalog, dependency, or adapter |
| Generalization changes shipped identity | Pin exact ref/fingerprint and byte replay before and after |
| Child selector widens authority | Conservative containment plus parent/local intersection and deny precedence |
| One level becomes an aggregate score | Compare all six domains independently in tests and APIs |
| Transition record self-authorizes | Separate request/disposition types and one-terminal admission registry |
| Captured assertion replays in a fresh resolver | Fresh 32-byte challenge nonce plus signature verification; prove zero/nonzero-counter replay refusal |
| Promotion claims an untyped or non-higher target | Consume actual source envelope/stack and typed semantic-memory target/result; enforce strict rank and CAS |
| HCM-3.5 adoption leaks into this slice | No pipeline/flow source edit; run existing L0-L3 tests only |
| Public API grows without proof | Require exact callable signatures, counting rule, and misuse/export audit before resumption |
| Named stack edits regress broad callers | Preserve exact shipped replay and run the accepted HIGH/CRITICAL proportional wall |
| GitNexus FTS/compare unavailable | Record unavailable honestly; never label GREEN |

## Definition of done

The selector-admission review is CLEAN before RED; all focused and workspace
proof passes; later implementation/proof/final reviews close with no unresolved
P1/P2; the reviewed primary state and separate mechanical completed v1.4
closeout are committed; the dedicated local integration ref advances by exact
expected-old CAS; the remote remains unchanged and no push occurs.
