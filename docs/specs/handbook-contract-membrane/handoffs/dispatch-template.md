# Internal Dispatch Authoring Guide

This Markdown file is explanatory guidance, not a machine dispatch and not a
schema-valid example. The only normative current machine template is
`internal-dispatch-template.json`; validate instantiated JSON dispatches against
`internal-dispatch.v1.3.schema.json`. Do not add front matter that claims a
machine schema identity here, and do not copy retired legacy fields into a
current dispatch.

# Dispatch: Replace With Exact Objective

## Mission

Execute exactly the bounded objective below. Do not widen into sibling work.

For execution_target=internal_subagent, return the result to the active parent through the built-in subagent channel. Do not create a global handoff, append ledger.jsonl, declare the parent slice complete, or ask the user to start another task.

## Execution Contract

- Execution target: internal_subagent | top_level_resume | human_interactive
- Parent orchestration ID:
- Built-in agent type: default
- Role: implementation | documentation | review | proof | remediation
- Review cycle: discovery | closure | supplemental_causal, stable cycle ID,
  exact causal run/finding refs | null for non-review
- Subject hygiene: text-files-no-trailing-whitespace-v1
- Fresh isolated context required: true
- Closeout owner: parent_orchestrator
- Ordered required skills:
- Editing authority: exact paths | read_only
- Parallel-safety statement:
- Expected built-in final status:

Built-in subagent execution must use the active session's spawn/wait/status/message/interrupt capabilities. Shell-launched Codex, codex exec, background processes, temporary-file reviewer transport, filesystem identities, and filesystem polling do not satisfy this contract.

## Structured Return Contract

Return to the parent:

- built-in agent ID or canonical task name;
- final built-in status;
- concise work/result summary;
- changed paths, or none for read-only review;
- commands/checks and raw-result references;
- findings first, ordered by severity, for review/proof roles;
- each finding's file/line, violated contract/gate, reasoning, smallest remediation, and missing proof;
- P1/P2 blocking disposition and P3/P4 fix, inventory, or duplicate disposition
  under `../09-review-finding-inventory.md`;
- blockers/escalations and exact resume condition;
- recommended parent disposition.

For review roles, `verdict: findings` means at least one valid P1/P2.
`verdict: clean` means no valid P1/P2 and may include P3/P4 advisories. Identify
findings independently before comparing P3/P4 with the inventory; cite the
existing finding ID instead of creating a duplicate when its comparison key and
underlying defect match.

Do not write a canonical handoff or global ledger entry for an internal delegated run.

## Replayable Subject Manifest

Current internal JSON dispatches record sorted repository-relative paths and each file's lowercase SHA-256. Encode each entry as path, NUL, SHA-256, newline; hash the concatenated bytes for subject_fingerprint. The reviewer and validator must be able to reconstruct the exact subject rather than trust a free-form fingerprint.

Before any reviewer is spawned, validate and replay its frozen dispatch:

```text
uv run --with jsonschema==4.25.1 python docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --verify-dispatch <repo-relative-dispatch-path>
```

The dependency-complete command admits only v1.3 and verifies its schema, all
live manifest hashes, aggregate fingerprint, and trailing-space/tab hygiene
across every UTF-8 text manifest entry. V1.1/v1.2 are immutable predecessor
evidence and cannot be used for new execution or closeout lineage.

## Active Context Resolution

- Level:
- Scope horizon:
- Detail resolution:
- Temporal horizon:
- Authority horizon:
- Memory horizon:
- Validation horizon:

## Snapshot Grounding

- Prior/top-level-end snapshot ref:
- Current/top-level-start snapshot ref:
- Deterministic delta ref:
- Resolution-aware grounding projection ref:
- Included state families:
- Explicitly omitted state families:
- Drift signals and durable justification refs:
- Capture consistency: stable | bounded | unstable | not_available

Do not load or paste the complete snapshot when the active Resolution requires only a bounded projection.

## Authority Order

1. Slice-local SPEC.md, tasks/plan.md, and tasks/todo.md
2. Exact control-pack sections named below
3. Selected handoff as resume context, not architecture or slice-selection authority
4. Live code/tests for current implementation truth
5. Idea/archive context only when explicitly named

## Must Read

- Replace with exact pack sections.
- Replace with exact live files/tests.
- Read only source handoffs named in the machine dispatch.

## Current Repo-Truth Statement

Replace with the freshly verified current boundary and semantic status.

## Affected Seams and Maximum Permitted Change

- Affected seam rows:
- Current classifications:
- Maximum classification/proof change this dispatch may support:
- Sibling/future seams that remain context-only:

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
- Parent-slice closeout by this internal agent.

## Tasks and Deliverables

1. Replace with exact ordered work.

## Contracts and Proof Gates

- Replace with exact 05 contract sections.
- Replace with exact 06 proof gates and regression scenarios.

## Stop and Escalate When

- Target docs and live truth conflict in a behavior-changing way.
- Work requires a broader authority or Resolution horizon.
- A finding cannot be remediated within the declared dispatch.
- Required proof cannot be produced inside this dispatch.
- Built-in subagent capability returns an explicit error.

For an internal delegated run, report the stop condition to the active parent. Do not convert it into a global handoff.

## Closeout by Execution Target

### internal_subagent

Return the structured result through the built-in subagent channel. The parent validates and reconciles it, continues the review/remediation loop, and owns proof, commit, handoff, and ledger closeout.

### top_level_resume

The new top-level parent resumes the explicit phase/slice under 07, completes the active loop, and writes one current-schema handoff only at its next genuine stop.

### human_interactive

Record the exact requested human action and observable completion/recheck condition. The active parent remains the closeout owner.
