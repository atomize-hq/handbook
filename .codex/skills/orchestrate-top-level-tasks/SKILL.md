---
name: orchestrate-top-level-tasks
description: Route Handbook contract-membrane (HCM) phase and slice authority preparation and automatically continue authorized whole-slice or sequential packet work through fresh top-level task sessions. Use when a meta task receives an HCM phase or slice request, including when phase slices or slice packets still need bounded decomposition, or when a top-level task must prepare one authority level or execute one assigned slice or packet.
---

# Route HCM Top-Level Tasks

Use this skill to route an explicit HCM request through phase-to-slice preparation, slice-to-packet preparation, and authorized execution. The user's request or confirmation may delegate bounded preparation; the skill never invents product scope or unresolved technical decisions.

## Ground in current authority

Resolve paths from the repository root. Read the authority entry point first:

- `docs/specs/handbook-contract-membrane/README.md`

Require either current slice authority or the lifecycle's bounded preparation conditions. Treat archived documents and
`.codex/skills/.disabled/orchestrate-top-level-tasks/` as historical and non-authoritative unless the current authority adopts a bounded part.

Use the approved clean-room documents without restating their policy:

- `docs/specs/handbook-contract-membrane/work-unit-lifecycle.md`
- `docs/specs/handbook-contract-membrane/review-and-closeout-policy.md`
- `docs/specs/handbook-contract-membrane/authority-preparation-prompt.md`
- `docs/specs/handbook-contract-membrane/task-session-prompt.md`
- `docs/specs/handbook-contract-membrane/meta-task-prompt.md`

Do not treat missing detailed authority as an automatic admission failure. A user-requested named slice with a current high-level outcome may enter slice preparation even while a hard predecessor is incomplete or its completion evidence is stale; that predecessor gates product execution after preparation. A phase shape with no slices requires explicit user approval before phase preparation. A separate packet file is not required when the slice names and bounds the packet, its predecessor is complete, and the slice delegates just-in-time preparation.

## Route by role

Identify a **meta task** by its ownership of an HCM phase or slice request and cross-task continuation. Identify an **authority-preparation session** by one phase-to-slice or slice-to-packet assignment. Identify an **implementation task session** by one bounded whole-slice or packet assignment.

If those signals conflict or remain ambiguous, request the missing role boundary before acting.

### Meta task

1. Read the lifecycle, review policy, meta-task prompt, and only the current program or slice authority needed to classify the request.
2. If a phase shape lacks slices, ask once for phase-decomposition approval unless the user already explicitly requested it. If a named requested slice has a current high-level outcome but lacks detailed authority or packets, dispatch slice preparation without another human gate.
3. For each preparation or implementation unit, use `agent-using-agent-skills` to select only the currently applicable route and `agent-context-engineering` to build one fresh capsule from focused current sources, tests, precedent, risks, and bounded adopted history.
4. Create one fresh user-visible top-level task with `create_thread`; never use `fork_thread` or a collaboration subagent for that layer and never launch future units concurrently.
5. Monitor every top-level task only with `wait_threads`, using `timeoutMs: 600000` normally or `1200000` for a particularly large unit. Continue waiting with the cursor after timeout or a non-replacing user interruption. Use `read_thread` with `turnLimit: 5` only after completion or attention, then direct recoverable friction with `send_message_to_thread` and return to waiting.
6. Validate each compact result. After phase `AUTHORITY_READY`, return to meta control and separately dispatch slice preparation for the selected slice when execution was requested. After slice `AUTHORITY_READY`, reload and admit it before execution. After `PACKET_COMPLETE`, dispatch only the next eligible packet. After the final packet, perform one aggregate closeout.

Treat `STOPPED` and `ADMISSION_FAILED` as adjudication inputs, not automatic workflow stops. Correct, resume, or replace the same unit when current sources identify one safe continuation. Escalate only for missing phase-decomposition permission, an exact unresolved consequential decision, authority expansion, or no executable safe continuation.

### Authority-preparation session

1. Read the lifecycle, review policy, authority-preparation prompt, assigned current sources, and supplied capsule. Do not load the implementation prompt or adjacent planning levels.
2. Use `agent-using-agent-skills` phase by phase. Apply specification and task-breakdown reasoning, plus API/interface, codebase, domain, source, security, migration, or performance skills only when the assigned phase or slice implicates them. Do not invoke the generic `agent-spec` wrapper or use its generic output paths and repeated gates.
3. For `LEVEL: phase`, define slices only. For `LEVEL: slice`, define packets for exactly one slice. Do not edit product code, dispatch another top-level task, or cross into the next planning or execution level.
4. Work directly or delegate internally only to local collaboration subagents. Verify, use one focused causal review when applicable, commit coherently, and return the authority prompt's eight fields with `AUTHORITY_READY`, `STOPPED`, or `ADMISSION_FAILED`.

### Implementation task session

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
- Follow repository instructions and the six current clean-room documents listed above when this skill is incomplete or ambiguous.
