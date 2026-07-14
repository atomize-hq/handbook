---
dispatch_id: YYYYMMDDTHHMMSSZ--HCM-X-Y--short-slug
source_handoff_id: replace-me
orchestration_decision: replace-me
session_kind: implementation
phase_id: HCM-X
slice_id: HCM-X.Y
packet_id: null
snapshot_ref: null
delta_ref: null
snapshot_projection_ref: null
status: ready
---

# Dispatch: Replace With Exact Objective

## Mission

Execute exactly the bounded objective below. Do not widen into sibling work.

## Active Context Resolution

- Level:
- Scope horizon:
- Detail resolution:
- Temporal horizon:
- Authority horizon:
- Memory horizon:
- Validation horizon:

## Snapshot Grounding

- Prior/session-end snapshot ref:
- Current/session-start snapshot ref:
- Deterministic delta ref:
- Resolution-aware grounding projection ref:
- Included state families:
- Explicitly omitted state families:
- Drift signals and durable justification refs:
- Capture consistency: stable | bounded | unstable | not_available

Do not load or paste the complete snapshot when the active Resolution requires only a bounded projection.

## Authority Order

1. Slice-local `SPEC.md`, `tasks/plan.md`, and `tasks/todo.md`
2. Exact control-pack sections named below
3. Live code/tests for current implementation truth
4. Idea/archive context only when explicitly named

## Must Read

- Replace with exact pack sections.
- Replace with exact live files/tests.
- Read the source handoff record.

## Current Repo-Truth Statement

Replace with the freshly verified current boundary and semantic status.

## Artifact / Intake / Posture Boundary

- Artifact-kind definition refs:
- Artifact-instance/profile refs:
- Intake definition/candidate/canonical refs, if applicable:
- Posture kernel/recommendation/transition refs, if applicable:
- Shipped-default decision status: approved ref | not_applicable | unresolved

Do not infer shipped defaults from examples/current code, treat intake candidates as canonical, enact posture recommendations without approval, or introduce dynamic CLI commands for custom kinds/vocabulary.

## Allowed Scope

- Replace with exact paths/capabilities.

## Explicit Non-Goals

- Unrelated cleanup.
- Adjacent slice implementation.
- User migration tooling or implicit legacy compatibility.
- Dynamic/generated CLI commands for profiles, vocabulary, or artifact kinds.

## Tasks and Deliverables

1. Replace with exact ordered work.

## Contracts and Proof Gates

- Replace with exact `05` contract sections.
- Replace with exact `06` proof gates.

## Stop and Escalate When

- Target docs and live truth conflict in a behavior-changing way.
- Work requires a broader authority or Resolution horizon.
- A finding requires cross-document repair or child-packet decomposition.
- The required proof cannot be produced inside this packet.

## Mandatory Closeout

Before responding in chat:

1. write one immutable handoff record under `docs/specs/handbook-contract-membrane/handoffs/records/`;
2. capture/reference the session-end snapshot and session delta when Snapshot Memory is available, otherwise record `not_available` honestly;
3. update `handoffs/ledger.jsonl` according to `08-handoff-ledger-and-escalation-protocol.md`;
4. keep the chat response to status, handoff path, short summary, next action, and one `jq` read command.
