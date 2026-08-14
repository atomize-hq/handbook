---
name: orchestrate-top-level-tasks
description: Route Handbook contract-membrane (HCM) work when a meta task owns one currently authorized slice and must choose whole-slice or sequential packet top-level task sessions, or when a task session must execute one assigned authorized slice or packet.
---

# Route HCM Top-Level Tasks

Use this skill only to route already-authorized work. Do not treat it as slice authority or permission to change product scope.

## Ground in current authority

Resolve paths from the repository root. Read the authority entry point first:

- `docs/specs/handbook-contract-membrane/README.md`

Require an explicit, current slice-specific authority. Treat archived documents and
`.codex/skills/.disabled/orchestrate-top-level-tasks/` as historical and non-authoritative unless the current authority adopts a bounded part.

Use the approved clean-room documents without restating their policy:

- `docs/specs/handbook-contract-membrane/work-unit-lifecycle.md`
- `docs/specs/handbook-contract-membrane/review-and-closeout-policy.md`
- `docs/specs/handbook-contract-membrane/task-session-prompt.md`
- `docs/specs/handbook-contract-membrane/meta-task-prompt.md`

If the role or current authority cannot be established, do not dispatch or edit. State the missing or stale authority required to proceed.

## Route by role

Identify a **meta task** by its ownership of a complete slice, execution mode, outcome conditions, and packet sequence when applicable. Identify a **task session** by its bounded whole-slice or packet assignment.

If those signals conflict or remain ambiguous, request the missing role boundary before acting.

### Meta task

1. Read the lifecycle, review policy, meta-task prompt, and only the current slice authority needed to route the slice.
2. Select `whole-slice` or `packet-by-packet` mode from current authority. Do not infer mode or packets from historical material.
3. Load the task-session prompt only when filling the current dispatch.
4. In whole-slice mode, dispatch one bounded whole-slice task session.
5. In packet-by-packet mode, dispatch exactly one authorized packet at a time. Do not preload or concurrently launch future packets.
6. Consume the task session's compact eight-field result and retain only one concise conversational summary per completed packet.
7. Own every continuation decision. After the final packet, perform one proportionate aggregate verification, review when applicable, and slice closeout.

Accept `SLICE_COMPLETE` only from whole-slice mode. Accept `PACKET_COMPLETE` only from packet mode. Halt cleanly on `STOPPED` or `ADMISSION_FAILED` as the meta-task prompt directs.

### Task session

1. Read the lifecycle, review policy, task-session prompt, assigned current authority, and only the live sources needed for the assigned unit.
2. Do not load the meta-task prompt, future packets, unrelated slice materials, or the disabled legacy skill.
3. Perform only the assigned whole slice or packet within its stated scope, acceptance, verification, and commit posture.
4. Return exactly the task-session prompt's eight semantic fields: `STATUS`, `UNIT`, `OUTCOME`, `CHANGED`, `VERIFICATION`, `REVIEW`, `COMMIT`, and `NEXT`.
5. From a packet, return `PACKET_COMPLETE` and control to the meta task. Never claim slice completion, select another packet, or perform aggregate closeout.

Return `ADMISSION_FAILED` before edits when current authority or another admission requirement is insufficient. Return `STOPPED` only for a genuine workflow stop defined by the lifecycle.

## Keep execution thin

- Load other applicable skills only when their phase begins; do not blanket-preload skills.
- Use one focused reviewer per causal review round when review applies. Add specialist input only for a named unresolved measured risk under the review policy.
- Keep routing state in conversation. Do not create ledgers, selectors, handoffs, proof walls, schemas, validators, dispatch archives, or other durable orchestration state.
- Create no mandatory mechanical second commit and no packet-level slice closeout.
- Follow repository instructions and the five approved documents when this skill is incomplete or ambiguous.
