# Intent-to-Outcome Fidelity

**Status:** Future idea

**Date:** 2026-07-28

**Context:** Handbook planning, implementation governance, and adaptive delivery controls

## Problem Statement

How might Handbook preserve fidelity from a user's actual intent through planning and implementation, detect where meaning or execution is likely to drift, and apply only the alignment and delivery controls needed for the work to land correctly?

Approval alone is not proof of shared understanding. A user can approve a coherent plan, the plan can be implemented faithfully, and the result can still be wrong because the plan recorded a different understanding from what the user intended. Separately, a plan can accurately represent the user's intent while implementation drifts away from it.

These are related failures at different boundaries:

```text
user intent
    -- intent-alignment risk --> approved plan
    -- implementation-drift risk --> landed implementation
    -- future outcome-validation risk --> observed outcome
```

## Recommended Direction

Build one **intent-to-outcome fidelity system** with independently evolvable evaluators for each boundary. Start with two subsystems:

1. **Intent Alignment Risk** estimates the chance that the proposed plan does not faithfully represent the user's actual intent, even if the user is likely to approve it.
2. **Implementation Drift Risk** estimates the chance that implementation will not faithfully land the approved plan and recommends an appropriate execution posture.

The evaluators should remain separate because they use different evidence and require different interventions. They can share a common recommendation interface containing the risk tier, confidence, supporting evidence, missing evidence, recommended controls, escalation triggers, and any recorded override.

The system should be explainable and adaptive rather than pretending to produce a perfectly precise probability. Low-risk work should remain lightweight. Higher-risk work should receive proportionally stronger alignment checks, smaller slices, closer monitoring, tighter review cycles, or more interactive implementation. The exact implementation modes are policy outputs and should not be permanently encoded into the core concept.

## Subsystem 1: Intent Alignment Risk

This subsystem asks:

> If this plan is implemented exactly as written, how likely is the user to say, "That is not what I meant"?

Candidate signals include:

- Ambiguous desired outcomes or success criteria.
- Material assumptions introduced by the planner but not made visible to the user.
- Large interpretive distance between the user's language and the plan's concrete behavior.
- Missing examples, counterexamples, edge cases, or explicit non-goals.
- Irreversible or high-cost choices hidden behind apparently simple approval.
- Conflicting preferences or unresolved stakeholder interpretations.
- Approval based on an abstract plan without a concrete preview of the resulting experience or behavior.

Possible recommendations include:

- Restate the intended outcome in plain language.
- Show what will and will not be true after implementation.
- Walk through representative scenarios and counterexamples.
- Isolate consequential assumptions as explicit user decisions.
- Produce a lightweight prototype, behavioral contract, or outcome preview before plan lock.
- Require stronger alignment evidence when misunderstanding would be expensive to reverse.

The goal is not to interrogate users about every detail. It is to identify when ordinary approval is weak evidence of actual alignment and selectively strengthen it.

## Subsystem 2: Implementation Drift Risk

This subsystem asks:

> Given the approved plan and implementation context, how likely is the work to diverge, expand, stall, or land incorrectly during execution?

Candidate signals include:

- Novelty, ambiguity, or unresolved technical decisions.
- Cross-cutting changes, high coupling, or large blast radius.
- Weak reversibility, testability, or observability.
- Volatile dependencies or unfamiliar components.
- Long feedback cycles between implementation and meaningful validation.
- Large slices with multiple independent failure modes.
- Scope churn, unexpected dependencies, repeated rework, or failed gates observed during execution.

Possible recommendations include:

- Reduce slice size and shorten feedback loops.
- Increase monitoring or review cadence.
- Add intermediate evidence, integration, or acceptance gates.
- Select a more appropriate implementation mode, ranging from autonomous execution through paired AI work to human-interactive implementation.
- Escalate when observed evidence exceeds predefined drift thresholds.

This assessment should be recomputed during implementation. Controls may tighten when evidence of drift appears and relax when uncertainty is retired and the work remains on-contract.

## Future Subsystem: Outcome Validation

A third subsystem should eventually ask whether the correctly understood and faithfully implemented change produced the intended real-world outcome.

This concept belongs in the overall fidelity chain now, but should be implemented and fully designed only after Intent Alignment Risk and Implementation Drift Risk have landed and become stable. Outcome validation will require trustworthy outcome definitions, observation periods, feedback capture, and attribution rules. When mature, it could also calibrate the earlier evaluators by showing which alignment and drift signals actually predicted unsuccessful outcomes.

## Key Assumptions to Validate

- [ ] Intent misalignment has detectable signals before plan lock — test against past work where the delivered result surprised or disappointed the user despite plan approval.
- [ ] Drift risk can be assessed early enough to change delivery behavior — compare initial assessments with later scope churn, rework, gate failures, and acceptance outcomes.
- [ ] Recommended controls reduce incorrect landings without burdening routine work — begin in advisory or shadow mode and measure recommendation usefulness, overrides, cycle time, and avoided rework.
- [ ] Users and implementers can understand and challenge the assessment — require every recommendation to cite evidence and expose uncertainty rather than returning an unexplained score.

## MVP Scope

The minimum useful version should be an explainable, rule-based advisory system rather than a learned prediction model.

- Evaluate one planned unit of work, such as a feature or implementation slice.
- Produce separate Intent Alignment Risk and Implementation Drift Risk assessments.
- Cite the evidence and missing information behind each assessment.
- Recommend proportional alignment checks and implementation controls from configurable policy.
- Reassess drift risk at a small number of implementation checkpoints.
- Allow a human or orchestrator to accept or override recommendations with a recorded reason.
- Retain assessment and outcome evidence for later calibration.

Success would mean detecting more misunderstandings before plan lock, reducing late rework and plan-to-implementation divergence, and preserving fast paths for straightforward work.

## Not Doing (and Why)

- **One universal fidelity score** — combining distinct failure boundaries would hide why work is risky and produce the wrong intervention.
- **False numerical precision** — early assessments should use explainable tiers, evidence, and confidence until outcome data supports calibration.
- **Fixed implementation-mode taxonomy** — autonomous, paired, and human-interactive methods will evolve; the system should recommend policy-defined postures rather than own permanent labels.
- **Automatic enforcement everywhere** — advisory operation should establish usefulness and false-positive rates before recommendations become hard gates.
- **Learned prediction in the first version** — there will not initially be enough clean, representative outcome data to justify it.
- **Outcome Validation in the first implementation** — capture its place in the architecture now, then design and add it after the first two subsystems are stable.
- **Replacing user or engineering judgment** — the system should make risk and tradeoffs visible, not claim authority it has not earned.

## Open Questions

- Which planning levels should be assessed independently: project, sprint, feature, task, or implementation slice?
- What evidence establishes that a user understood the concrete consequences of an approved plan?
- Which recommendations should remain advisory, and what explicit policy could authorize hard gates?
- How should project-specific risk profiles modify signals without making results incomparable?
- What constitutes ground truth for intent misalignment or implementation drift?
- How should the system distinguish a justified plan change from harmful drift?
- How can risk controls avoid becoming process theater or an incentive to underreport uncertainty?
