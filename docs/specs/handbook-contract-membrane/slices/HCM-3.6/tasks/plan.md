# HCM-3.6 implementation plan

## Planning result

This document selects the future implementation order only. It does not
authorize execution. The sole semantic owner will be
`handbook-engine::project_posture`; all other named systems remain consumers,
adapters, evidence sources, or future gate owners.

## Frozen dependency graph

```text
exact Charter/profile/approved override/condition/contract/evidence/snapshot pairs
  + explicit FreshnessEvaluationBasis
  -> resolve_project_posture_kernel
  -> ProjectPostureKernel
  -> evaluate_posture_recommendation
  -> NoRecommendation | Recommended | Refused

reviewed recommendation + exact approval/reassessment/current Charter target
  -> apply_posture_transition
  -> atomic Charter replace + immutable PostureTransition
  -> re-resolved ProjectPostureKernel
```

HCM-3.4/3.5 outputs enter only as current, redacted, exact evidence pairs.
Flow, pipeline, CLI, Tauri, SDK, Substrate, adapters, and HCM-5 are not in the
semantic-owner chain.

## Future packet sequence

### P0 — implementation admission

**Description:** before a later source change, validate a new slice selector,
upstream impact for every existing symbol to edit, the exact source/test scope,
and a no-public-surface ceiling.

**Acceptance criteria:**

- Exact `project_posture` owner/module boundary is selected in
  `handbook-engine` with no Flow/grounding/HCM-3.4/HCM-3.5 ownership transfer.
- Every existing symbol has upstream GitNexus impact evidence; HIGH/CRITICAL
  changes are surfaced before editing.
- The implementation packet owns no product schema, dependency, public API,
  adapter, gate, or promotion work.

**Verification:** selector and impact proof, source/test manifest replay,
fresh independent review.

**Dependencies:** a new explicit implementation authority after this planning
packet.

### P1 — kernel resolution

**Description:** construct `ExactPostureRef`, validate all exact inputs,
resolve profile currentness and explicit freshness basis, and derive the
fingerprinted global `ProjectPostureKernel`.

**Acceptance criteria:**

- Every input is an exact ref/fingerprint pair; blank, malformed, mutable-only,
  ambiguous, stale, and fingerprint-mismatched inputs refuse.
- The kernel reflects only global posture levels and carries explanatory
  applicability/omission/unresolved-condition references.
- Semantic fingerprints exclude presentation identifiers/timestamps and sort
  unordered lists stably.

**Verification:** future `P01`–`P03` and `P08` owner-local tests; no ambient
time or raw Snapshot payload reads.

**Dependencies:** P0.

### P2 — policy evaluation and advisory output

**Description:** evaluate hard triggers and policy-local accumulated rules;
produce exactly one-dimension immutable recommendation plus typed notification
intent, or closed no-recommendation/refusal output.

**Acceptance criteria:**

- Hard trigger and accumulated rule identities are mutually exclusive, exact,
  current, policy-bound, and evidence-linked.
- A two-dimension event emits two recommendations; scope remains metadata.
- Threshold/window/count misses, expired evidence, cooldown, and policy
  mismatches never become a recommendation through a fallback.

**Verification:** future `P04`, `P05`, `P06`, `P07`, `P08`, and `P11` tests.

**Dependencies:** P1.

### P3 — guarded constitutional transition

**Description:** implement `apply_posture_transition` as the only mutation
path, with approval/currentness/reassessment checks and one atomic
constitutional-root replace plus immutable transition record.

**Acceptance criteria:**

- Only one global-dimension `replace` at the mapped Charter path is admitted.
- Overrides, scoped paths, multi-change requests, wrong target class, stale
  targets, bad actors, and failed reassessment refuse before canonical mutation.
- Successful atomic commit yields resulting Charter/kernel pairs; all failure
  paths leave canonical authority untouched.

**Verification:** future `P06`, `P07`, `P09`, and `P10` tests, including
compare-and-write fault injection and no-write assertions.

**Dependencies:** P1 and P2.

### P4 — adapter/gate boundary proof

**Description:** prove only typed notification and acknowledgement intent/outcome
boundaries. No delivery adapter is selected, and no HCM-5 evaluation/promotion
capability is introduced.

**Acceptance criteria:**

- Adapter outcomes append acknowledgement disposition only and cannot change
  recipient, deadline, approval, recommendation, or policy.
- Recommendations, acknowledgements, packet/handoff completion, and advisory
  scores cannot mutate Charter, create a gate verdict, or promote a parent.
- Public/private owner chain remains acyclic and does not make Flow, pipeline,
  SDK, CLI, Tauri, Substrate, or HCM-5 a posture semantic owner.

**Verification:** future `P07`, `P11`, and `P12` boundary tests and a fresh
architecture review.

**Dependencies:** P2 and P3.

## Checkpoints and risks

| Checkpoint | Required evidence | Main risk controlled |
|---|---|---|
| Kernel admission | exact refs, currentness, explicit time basis | mutable or ambient authority |
| Evaluation | P01–P08 results and advisory-only proof | evidence becomes mutation/promotion |
| Transition | P06/P09/P10 atomic no-write/write proof | constitutional authority bypass |
| Boundary | P07/P11/P12 and independent review | adapter or gate ownership inversion |

Implementation must stop for source-authority conflict, public/schema/dependency
expansion, an unbounded adapter selection, HCM-5 promotion/gate scope, or a
failure to reach independent-review CLEAN inside the live causal budget.
