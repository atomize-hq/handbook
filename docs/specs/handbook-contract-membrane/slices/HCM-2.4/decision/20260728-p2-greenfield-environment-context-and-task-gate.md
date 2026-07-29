# HCM-2.4 P2 greenfield Environment Context and task-gate decision

Status: accepted product authority; supersedes the active conclusions of the
P2 evidence-evaluator amendment, P2 product-decision reassessment, and Option C
containment plan. Their commits, proof records, and dispatches remain
historical evidence only.

Date: 2026-07-28

## Decision

Environment Context is an optional canonical project artifact, not an
applicability oracle. A valid `.handbook/project/environment.yaml` may inform a
session whenever it is present. Its presence never selects an execution gate.

The shipped profile therefore selects Environment Context with `optional`
requiredness and no project-condition reference. Missing, invalid, unsafe, or
unreadable Environment Context is reportable but does not change repository
readiness while no task gate is selected.

An environment gate, when a later implementation-contract slice owns one, is
selected explicitly for one implementation task. Its decision is positive
only: that task's declared requirements must be a subset of current-session
capabilities that were positively verified. A selected gate never asks one
workstation to satisfy the project's complete environment matrix.

## Required behavior

| Environment Context | Gate | Task/session proof | Result |
| --- | --- | --- | --- |
| valid | not selected | irrelevant | load as advisory context; normal work is allowed |
| missing or invalid | not selected | irrelevant | continue without trusted context and report non-blockingly |
| valid | selected | task requirements positively satisfied | implementation is allowed for that task |
| valid | selected | requirements missing, failed, or unavailable | planning, inspection, diagnosis, and handoff only |
| missing or invalid | selected | unavailable | planning and diagnostic work only until repaired |

Planning, research, inspection, diagnosis, and handoff do not require
implementation proof. Unselected means only that enforcement is off; it makes
no claim about project-wide operational responsibility.

## Layered responsibility

1. Environment Context names the project's environments and their
   capabilities.
2. A feature/specification selects supported target environments, freezes its
   non-negotiable baseline constraints, and owns completion proof across every
   required target.
3. Each implementation task declares either portability or exact requirements
   drawn from Environment Context. A task may narrow or strengthen its
   feature/spec baseline but may not weaken or silently invent it.
4. Session execution authorization compares only that task declaration with
   positively verified session capabilities.
5. Sprint and orchestration groupings may aggregate readiness for scheduling
   and reporting. They are not execution authorities and cannot block a
   compatible task because another task needs a different environment.

If a task needs a new named environment or capability, Environment Context
changes first. The feature/spec completion proof and task execution proof are
separate obligations.

## Greenfield correction

There is no release-compatibility or migration obligation. Active definitions
are corrected in place when exact identity remains architecturally required;
Git history preserves the rejected design. No compatibility-only profile is
created.

The managed-operational-surface condition is removed from active shipped
resolution. Its evaluator definition, eight unadmitted evidence schemas, and
their test are removed because they have no current runtime consumer after this
decision. The condition definition remains only because shipped-root `1.1` is
an exact dependency of the HCM-2.2 Charter authority closure; it has no active
shipped applicability role and no evaluator. No owner identity, authenticator,
native/hardware adapter, signed declaration, evidence producer, transaction
chain, mutable head, freshness/revocation system, or six-state evaluator is
authorized.

Legacy Environment Inventory Markdown is not canonical authority. The retained
vertical is selected Environment Context YAML, schema-backed intake and generic
safe promotion, advisory read/validation, a deterministic fixed Markdown view,
and setup/doctor reporting.

## Future implementation-contract gate

The later implementation-contract slice cannot become GREEN unless all of the
following are implemented and proved:

- every implementation task declares `portable` or exact Environment Context
  environment/capability requirements;
- requirements cannot weaken or invent the owning feature/spec baseline;
- a selected task gate authorizes implementation only after subset matching
  against positively verified current-session capabilities;
- missing, invalid, unavailable, or unsatisfied proof permits only
  planning/diagnostic/handoff work;
- feature/spec cross-target completion proof remains separate and mandatory;
- sprint/orchestration aggregation remains reporting and scheduling only.

This packet records that contract but does not manufacture the future task,
session-proof, or feature-completion runtime types.

## Acceptance

- shipped Environment Context resolves as optional with no condition source;
- valid context is readable and deterministically renderable;
- absent or invalid optional context does not block setup/doctor readiness;
- selected generic intake/validation/promotion still writes only canonical
  Environment Context YAML through the existing safe mutation path;
- legacy Environment Inventory Markdown cannot affect readiness or flow;
- rejected evidence definitions and tests are absent;
- active HCM-0.6, HCM-2.4, P6/P7, and Phase 2 gates tell this same truth.
