---
name: orchestrate-top-level-tasks
description: Route and automatically continue Handbook contract-membrane (HCM) work when a meta task owns one currently authorized slice and must run whole-slice or sequential packet top-level task sessions, or when a task session must execute one assigned authorized slice or packet.
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
2. Select `whole-slice` or `packet-by-packet` mode and the current unit from current authority. Do not infer mode or packets from historical material.
3. Use `agent-using-agent-skills` to select the minimal skill route for the current unit only. Do not select or preload skills for future packets.
4. Use `agent-context-engineering` to fill the task-session prompt as one fresh context capsule containing only the current unit's starting state, relevant live sources and tests, a useful precedent when available, known risks or unknowns, and skill activation conditions.
5. Use the Codex app's task tools to create one fresh user-visible top-level task session. Call `create_thread` once; if it returns pending setup rather than a task identity, resolve that task without creating a duplicate. Do not use `fork_thread` or a collaboration subagent, and never concurrently launch future packets.
6. Stay active while the task runs: use `wait_threads`, inspect completion or attention with `read_thread`, and use `send_message_to_thread` to resolve in-authority friction. Do not require the user to monitor or manually return task results.
7. Consume the task session's compact eight-field result and retain only one concise conversational summary per completed packet.
8. Own every continuation decision. Automatically dispatch the next authorized packet after valid completion. After the final packet, perform one proportionate aggregate verification, review when applicable, and slice closeout.

Accept `SLICE_COMPLETE` only from whole-slice mode. Accept `PACKET_COMPLETE` only from packet mode. Treat task-session `STOPPED` and `ADMISSION_FAILED` results as adjudication inputs, not automatic slice stops. Correct, resume, or replace the current task when existing authority identifies one safe in-scope continuation. Escalate only when no such continuation can be derived or executed.

### Task session

1. Read the lifecycle, review policy, task-session prompt, assigned current authority, and the supplied context capsule. Treat the authority, not the capsule, as the source of scope.
2. Do not load the meta-task prompt, future packets, unrelated slice materials, or the disabled legacy skill.
3. Load each capsule-selected skill only when its activation phase begins. If an unexpected phase or failure makes that route insufficient, consult `agent-using-agent-skills` once for this unit and add only the newly applicable skill.
4. Perform only the assigned whole slice or packet within its stated scope, acceptance, verification, and commit posture. Resolve ordinary in-scope friction before returning control.
5. Return exactly the task-session prompt's eight semantic fields: `STATUS`, `UNIT`, `OUTCOME`, `CHANGED`, `VERIFICATION`, `REVIEW`, `COMMIT`, and `NEXT`.
6. From a packet, return `PACKET_COMPLETE` and control to the meta task. Never claim slice completion, select another packet, or perform aggregate closeout.

Return `ADMISSION_FAILED` before edits when current authority or another admission requirement is insufficient. Return `STOPPED` only for a genuine workflow stop defined by the lifecycle.

## Keep execution thin

- Treat the context capsule and skill route as transient dispatch content. Do not persist or return either one, and do not blanket-preload skills.
- Use one focused reviewer per causal review round when review applies. Add specialist input only for a named unresolved measured risk under the review policy.
- Keep routing state and live task identities in conversation. Do not create ledgers, selectors, handoffs, proof walls, schemas, validators, dispatch archives, or other durable orchestration state.
- Create no mandatory mechanical second commit and no packet-level slice closeout.
- Follow repository instructions and the five approved documents when this skill is incomplete or ambiguous.
