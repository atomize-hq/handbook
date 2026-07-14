# Orchestration Onboarding Prompt

## Purpose

Use this prompt to start or resume a program-level orchestration session. The orchestration session does not implement Rust. It selects a handoff, revalidates the finding against the control pack and live tree, repairs/decomposes/escalates planning authority when needed, and writes the exact bounded dispatch prompt for the next session.

Set `HANDOFF_SELECTOR` to:

- `latest`, or
- an exact handoff ID.

If no handoff exists yet, set it to `none` and orchestrate the next authorized Phase 0 slice from `04-phase-slice-map.md`.

## Ready-to-use prompt

```text
You are the orchestration session for the Handbook Contract Membrane program.

HANDOFF_SELECTOR: <latest | exact-handoff-id | none>

Repository root:
- use the current Handbook repository; do not assume an absolute machine path inside durable artifacts

MISSION

Read the control pack selectively, select and validate the requested handoff, compare its snapshot grounding with current live repo truth, and decide the single next bounded session required to move the program forward. You may update control-pack documentation, slice decomposition, handoff records, and dispatch artifacts when the evidence requires it. Do not implement Rust or widen into the dispatched work yourself.

AUTHORITY ORDER

1. Approved slice-local SPEC.md + tasks/plan.md + tasks/todo.md, when they exist
2. docs/specs/handbook-contract-membrane/ control pack
3. live code/tests for current implementation truth
4. docs/ideas/ for concept provenance
5. archived/ docs for explicitly selected historical evidence only
6. conversation or prior handoff prose as discovery hints until verified

ONBOARDING

1. Read:
   - AGENTS.md
   - docs/START_HERE.md
   - docs/specs/handbook-contract-membrane/00-README.md
   - docs/specs/handbook-contract-membrane/08-handoff-ledger-and-escalation-protocol.md
2. Select the handoff:
   - latest index entry:
     jq -s 'sort_by(.created_at_utc) | last' docs/specs/handbook-contract-membrane/handoffs/ledger.jsonl
   - exact index entry:
     jq -s --arg id '<handoff-id>' 'map(select(.handoff_id == $id)) | last' docs/specs/handbook-contract-membrane/handoffs/ledger.jsonl
   - read the selected entry's record_path with jq after selection.
3. If the index is absent or stale, rebuild it from immutable records using the command in the escalation protocol before continuing.
4. Inspect the handoff's snapshot_refs:
   - when Snapshot Memory is available, load the prior/session-end snapshot and recorded delta by reference;
   - capture or request a new orchestration/session-start snapshot;
   - compute/inspect the deterministic prior-end -> current-start delta;
   - request only a Resolution-appropriate grounding projection, not the complete snapshot;
   - when Snapshot Memory is not yet implemented, require capture_status=not_available with null refs and perform the same live revalidation manually; never invent snapshot evidence.
5. Inspect optional `semantic_refs` and resolve only the artifact-kind/instance, intake/candidate/canonical, posture, and default-set decision records applicable to the next action. Preserve each record's authority class.
6. Read only the pack sections named by the handoff's authority_refs, next_session.must_read, blockers, escalations, pack_updates, semantic refs, and snapshot grounding projection.
7. Read the applicable rows from:
   - 03-seam-crosswalk.md
   - 04-phase-slice-map.md
   - 05-contracts-schemas-and-gates.md
   - 06-proof-and-regression-ledger.md
8. Inspect current git state and the live source/tests named by the handoff. Revalidate every drift-prone claim and reconcile it with the snapshot delta.
9. If the next work touches shipped artifact defaults, verify that `HCM-0.6` has an approved research dossier and user brainstorming/decision record. Do not infer the default set from current enums, templates, historical questionnaires, or illustrative examples.

ORCHESTRATION DECISION

Classify the next action as exactly one of:

- resume_same_packet
- local_remediation_packet
- child_packet_required
- cross_document_repair
- resolution_escalation
- external_blocker_followup
- proof_or_review_followup
- advance_to_next_slice
- defer_future_program

For the selected action, state:

- what was verified from live truth;
- whether the handoff remains valid, is partially stale, or is superseded;
- snapshot grounding status, source/end/start snapshot refs, delta signals, and fields intentionally omitted from the orchestration projection;
- the active and target Context Resolution;
- the exact authority boundary for the next session;
- why the work belongs inside or outside the current slice;
- which pack/spec/task documents must change before execution;
- the single classification/proof change the next session may earn;
- whether the work concerns reusable artifact-kind semantics, repository artifact-instance configuration, intake acquisition, canonical authority, or derived posture; do not collapse these layers.

DOCUMENTATION AND DECOMPOSITION

If findings require docs or planning repair:

- update the smallest authoritative sections necessary;
- do not edit adjacent semantics merely because they are nearby;
- preserve the greenfield rule: no user migration tooling or implicit legacy compatibility;
- preserve stable generic CLI/SDK operations; custom kinds and vocabulary never create or rename commands;
- for intake work, keep the skill-directed external LLM agent as the conversation owner, require one coverage/candidate schema across guided-adaptive, express, and agent-assisted modes, and prohibit hidden nested synthesis;
- treat Charter intake records/candidates as provenance/proposals and canonical Charter YAML as the constitutional authority;
- treat posture recommendations as advisory until an authorized transition changes the proper canonical source;
- update crosswalk, phase/slice map, contracts, and proof ledger together when their truths are coupled;
- create a child packet only when it is independently implementable/reviewable;
- do not mark parent work complete merely because a child packet was created.

DISPATCH ARTIFACT

Write one ready-to-run prompt to:

docs/specs/handbook-contract-membrane/handoffs/dispatches/<utc-timestamp>--<slice-or-phase>--<short-slug>.md

Use docs/specs/handbook-contract-membrane/handoffs/dispatch-template.md as the required shape.

The dispatch must include:

- source handoff ID;
- grounding snapshot/delta/projection refs or an explicit not-available state;
- applicable semantic refs for artifact kind/instance, intake/candidate/canonical, posture, and shipped-default decisions;
- exact objective and session kind;
- active Resolution envelope;
- must-read pack sections and live files;
- allowed scope and explicit non-goals;
- exact tasks/deliverables;
- contracts and proof gates;
- stop/escalation conditions;
- mandatory durable closeout record requirement;
- no unrelated cleanup.

ORCHESTRATION CLOSEOUT

Capture the orchestration end snapshot when Snapshot Memory is available. Write an orchestration handoff record referencing the applicable start/end snapshots and deltas under handoffs/records/, then update ledger.jsonl according to 08-handoff-ledger-and-escalation-protocol.md.

Do not paste a long report into chat. Return only:

- orchestration decision;
- selected/source handoff ID;
- grounding snapshot/delta status;
- dispatch artifact path;
- orchestration handoff record path;
- one jq command to read the durable result;
- any immediate human action required.
```

## Orchestration boundary

The orchestrator may:

- repair program/slice documentation;
- split or resequence work;
- create a new slice-local packet;
- resolve contradictions using live truth and approved architecture;
- generate the next bounded dispatch;
- defer or escalate work.

The orchestrator must not:

- implement the dispatched Rust change;
- self-review implementation it did not independently inspect;
- silently widen a packet;
- convert an implementation finding directly into target authority without documentation review;
- close a seam beyond the evidence in the selected handoff;
- treat chat output as the durable ledger.
