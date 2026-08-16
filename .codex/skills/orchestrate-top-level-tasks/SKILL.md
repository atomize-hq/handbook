---
name: orchestrate-top-level-tasks
description: Route and automatically continue Handbook contract-membrane (HCM) work when a meta task owns one authorized slice and must run whole-slice or sequential packet top-level task sessions, including slice-delegated just-in-time packet authority preparation, or when a task session must execute one assigned slice or packet.
---

# Route HCM Top-Level Tasks

Use this skill only to route work authorized by current slice authority. A slice may delegate bounded just-in-time preparation for its named packets; the skill itself never supplies authority or permission to change product scope.

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

If the role or current slice authority cannot be established, do not dispatch or edit. A separate packet file is not required when the slice names and bounds the packet, its predecessor is complete, and the slice explicitly delegates just-in-time preparation.

## Route by role

Identify a **meta task** by its ownership of a complete slice, execution mode, outcome conditions, and packet sequence when applicable. Identify a **task session** by its bounded whole-slice or packet assignment.

If those signals conflict or remain ambiguous, request the missing role boundary before acting.

### Meta task

1. Read the lifecycle, review policy, meta-task prompt, and only the current slice authority needed to route the slice.
2. Select `whole-slice` or `packet-by-packet` mode and the current unit from current authority. Do not infer mode or packets from historical material. A named packet may use historical planning only where the slice adopts a bounded input.
3. Use `agent-using-agent-skills` to select the minimal skill route for the current unit only. Do not select or preload skills for future packets.
4. Use `agent-context-engineering` to fill the task-session prompt as one fresh context capsule containing only the current unit's starting state, relevant live sources and tests, a useful precedent when available, known risks or unknowns, skill activation conditions, and any slice-delegated preparation boundary and permitted inputs.
5. Use the Codex app's task tools to create one fresh user-visible top-level task session. Call `create_thread` once; if it returns pending setup rather than a task identity, resolve that task without creating a duplicate. Do not use `fork_thread` or a collaboration subagent, and never concurrently launch future packets.
6. Stay active while the task runs. Use `wait_threads` as the only blocking monitor, with `timeoutMs: 600000` by default or `timeoutMs: 1200000` for a particularly large current unit. After a timeout, continue with `wait_threads`; never poll with `read_thread`. When non-replacing user direction interrupts the wait, apply it and immediately resume `wait_threads` for the same task with the most recent cursor and existing timeout; do not strand or duplicate the task. Only after `wait_threads` reports task completion or attention, use `read_thread` with `turnLimit: 5` to inspect the recent task state, then use `send_message_to_thread` when in-authority friction needs direction. Do not require the user to monitor or manually return task results.
7. Consume the task session's compact eight-field result and retain only one concise conversational summary per completed packet.
8. Own every continuation decision. After valid completion, automatically dispatch the next packet declared by the slice when its predecessor is complete and either packet authority exists or just-in-time preparation is delegated. After the final packet, perform one proportionate aggregate verification, review when applicable, and slice closeout.

Accept `SLICE_COMPLETE` only from whole-slice mode. Accept `PACKET_COMPLETE` only from packet mode. Treat task-session `STOPPED` and `ADMISSION_FAILED` results as adjudication inputs, not automatic slice stops. Missing packet documentation covered by slice-delegated preparation is correctable friction. Correct, resume, or replace the current task when existing authority identifies one safe in-scope continuation. Escalate only when permitted sources leave an exact material decision unresolved or no safe continuation can be executed.

### Task session

1. Read the lifecycle, review policy, task-session prompt, assigned current authority, and the supplied context capsule. Treat the authority, not the capsule, as the source of scope. Slice-delegated just-in-time preparation is authority for the named packet, not for adjacent work.
2. Do not load the meta-task prompt, future packets, unrelated slice materials, or the disabled legacy skill.
3. Load each capsule-selected skill only when its activation phase begins. If an unexpected phase or failure makes that route insufficient, consult `agent-using-agent-skills` once for this unit and add only the newly applicable skill.
4. Perform only the assigned whole slice or packet within its stated scope, acceptance, verification, and commit posture. When delegated, resolve and record bounded packet decisions before product edits, review material selections through one focused causal round, and proceed in the same task. Return `STOPPED` when permitted sources cannot select one consequential answer or another lifecycle-defined genuine stop remains after bounded recovery.
5. Perform work directly or delegate internally only to local collaboration subagents. Never create, fork, monitor, or coordinate another user-visible top-level task; the meta task alone owns that layer.
6. Return exactly the task-session prompt's eight semantic fields: `STATUS`, `UNIT`, `OUTCOME`, `CHANGED`, `VERIFICATION`, `REVIEW`, `COMMIT`, and `NEXT`.
7. From a packet, return `PACKET_COMPLETE` and control to the meta task. Never claim slice completion, select another packet, or perform aggregate closeout.

Return `ADMISSION_FAILED` before edits when current authority or another admission requirement is insufficient. Return `STOPPED` only for a genuine workflow stop defined by the lifecycle.

## Keep execution thin

- Treat the context capsule and skill route as transient dispatch content. Do not persist or return either one, and do not blanket-preload skills.
- Use one focused reviewer per causal review round when review applies. Add specialist input only for a named unresolved measured risk under the review policy.
- Require the review policy's explicit `P1 Critical`, `P2 Required`, `P3 Optional/Consider`, and `P4 Nit` classification. Remediate and re-review blocking `P1`/`P2`; report non-blocking `P3`/`P4` without making them required work or another causal round.
- Keep routing state and live task identities in conversation. Do not create ledgers, selectors, handoffs, proof walls, schemas, validators, dispatch archives, or other durable orchestration state.
- Create no mandatory mechanical second commit and no packet-level slice closeout.
- Follow repository instructions and the five approved documents when this skill is incomplete or ambiguous.
