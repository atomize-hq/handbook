---
name: orchestrate-top-level-tasks
description: Route Handbook contract-membrane (HCM) authority preparation and bounded execution through fresh top-level task sessions, returning to human review at the packet or slice that delivers an observable or notable change. Use when a meta task receives an HCM phase or slice request, including bounded decomposition, or a task prepares one authority level or executes one assigned unit.
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

For work involving structured file formats, serialization, canonicalization, or
human projections, read
[the structured data format standard](../../../docs/specs/handbook-contract-membrane/structured-data-format-standard.md)
and include it as a required source in the preparation or implementation capsule.
Carry its stated approval status into the assignment; this routing reference does
not approve proposed defaults or authorize migrations. Include audit excerpts and
reusable-tool sources only when the assigned decision needs that evidence.

Do not treat missing detailed authority as an automatic admission failure. A user-requested named slice with a current high-level outcome may enter slice preparation even while a hard predecessor is incomplete or its completion evidence is stale; that predecessor gates product execution after preparation. A phase shape with no slices requires explicit user approval before phase preparation. A separate packet file is not required when the slice names and bounds the packet, its predecessor is complete, and the slice delegates just-in-time preparation.

## Route by role

Identify a **meta task** by its ownership of an HCM phase or slice request, human review boundaries, and continuation within the approved increment. Identify an **authority-preparation session** by one phase-to-slice or slice-to-packet assignment. Identify an **implementation task session** by one bounded whole-slice or packet assignment.

If those signals conflict or remain ambiguous, request the missing role boundary before acting.

Apply the lifecycle's [AI model selection](../../../docs/specs/handbook-contract-membrane/work-unit-lifecycle.md#ai-model-selection) as actual session settings: `gpt-6-astra` with `thinking: high` for a fresh meta orchestrator, and `gpt-6.1-sol` with `thinking: xhigh` for each fresh top-level preparation or implementation task. Preserve the assigned role's settings during recovery and replacement. Adjudicate at meta level in Astra; do not switch child task sessions to Astra merely to remediate them. These selections do not change scope, authorization, internal subagent policy, or global defaults.

### Meta task

1. Read the lifecycle, review policy, meta-task prompt, and only the current program or slice authority needed to classify the request.
2. If a phase shape lacks slices, ask once for phase-decomposition approval unless the user already explicitly requested it. If a named requested slice has a current high-level outcome but lacks detailed authority or packets, dispatch slice preparation without another human gate.
3. For each preparation or implementation unit, use `agent-using-agent-skills` to select only the currently applicable route and `agent-context-engineering` to build one fresh capsule from focused current sources, tests, precedent, risks, and bounded adopted history.
4. Create one fresh user-visible top-level task with `create_thread`, setting `model: gpt-6.1-sol` and `thinking: xhigh`; never use `fork_thread` or a collaboration subagent for that layer and never launch future units concurrently.
5. Monitor every top-level task only with `wait_threads`, using `timeoutMs: 600000` normally or `1200000` for a particularly large unit. Continue waiting with the cursor after timeout or a non-replacing user interruption. Use `read_thread` with `turnLimit: 5` only after completion or attention, then direct recoverable friction with `send_message_to_thread` and return to waiting.
6. Validate each compact result. After phase `AUTHORITY_READY`, return to meta control and separately dispatch the selected slice's preparation only when that planning is authorized. After slice `AUTHORITY_READY`, present the next increment's short plan for human authorization and prove prerequisites before execution. Choose a packet review boundary when it delivers an independently meaningful observable or notable change; choose a slice boundary only when its packets are preparatory, and explain why. Pass the approved plan and boundary into each implementation capsule.
7. After `PACKET_COMPLETE`, present the human recap and stop at its selected review boundary or an unexpectedly earlier observable or notable outcome. Continue only through eligible preparatory packets inside an already approved slice increment. After the final packet, perform one aggregate closeout, recap, and stop; do the same recap-and-stop after whole-slice completion. Human review of the result precedes successor planning or dispatch. A recommendation of what should come next is allowed; detailed successor planning and implementation are separately bounded by the human's direction and plan approval.

Treat ordinary `STOPPED` and `ADMISSION_FAILED` results as adjudication inputs, not automatic workflow stops. Correct, resume, or replace the same unit when current sources identify one safe continuation within the approved increment. A whole-slice early `STOPPED` result for an unexpectedly completed meaningful packet instead requires a human recap and stop; do not recover or resume past that checkpoint. Escalate for missing permission, an exact unresolved consequential decision, authority expansion, or no executable safe continuation. A completed assigned unit awaiting human review retains its completion status; the early whole-slice `STOPPED` describes only the unfinished slice assignment, not a failed packet. Semantic approval, an admitted slice, `AUTHORITY_READY`, and a clean automated verdict cannot authorize crossing the checkpoint.

### Authority-preparation session

1. Read the lifecycle, review policy, authority-preparation prompt, assigned current sources, and supplied capsule. Do not load the implementation prompt or adjacent planning levels.
2. Use `agent-using-agent-skills` phase by phase. Apply specification and task-breakdown reasoning, plus API/interface, codebase, domain, source, security, migration, or performance skills only when the assigned phase or slice implicates them. Do not invoke the generic `agent-spec` wrapper or use its generic output paths and repeated gates.
3. For `LEVEL: phase`, define slices only. For `LEVEL: slice`, outline packets and human review boundaries for exactly one slice, with detailed planning only for the next increment. Do not edit product code, dispatch another top-level task, or cross into the next planning or execution level.
4. Work directly or delegate internally only to local collaboration subagents. Verify, use one focused causal review when applicable, commit coherently, and return the authority prompt's eight fields with `AUTHORITY_READY`, `STOPPED`, or `ADMISSION_FAILED`.

### Implementation task session

1. Read the lifecycle, review policy, task-session prompt, assigned current authority, and the supplied context capsule, including human plan authorization and review boundary. Treat the authority, not the capsule, as the source of scope. Slice-delegated just-in-time preparation applies only within the approved increment and is authority for the named packet, not for adjacent work.
2. Do not load the meta-task prompt, future packets, unrelated slice materials, or the disabled legacy skill.
3. Load each capsule-selected skill only when its activation phase begins. If an unexpected phase or failure makes that route insufficient, consult `agent-using-agent-skills` once for this unit and add only the newly applicable skill.
4. Perform only the assigned whole slice or packet within its stated scope, acceptance, verification, and commit posture. When delegated, resolve and record bounded packet decisions before product edits, review material selections through one focused causal round, and proceed in the same task. Return `STOPPED` when permitted sources cannot select one consequential answer or another lifecycle-defined genuine stop remains after bounded recovery.
5. Perform work directly or delegate internally only to local collaboration subagents. Never create, fork, monitor, or coordinate another user-visible top-level task; the meta task alone owns that layer.
6. Return exactly the task-session prompt's eight semantic fields: `STATUS`, `UNIT`, `OUTCOME`, `CHANGED`, `VERIFICATION`, `REVIEW`, `COMMIT`, and `NEXT`.
7. From a packet, return `PACKET_COMPLETE` and control to the meta task, describing the actual behavior, example, meaningful refusal, and unfinished work. Flag an earlier observable or notable outcome even when the packet was planned as preparatory. Never claim slice completion, select another packet, or perform aggregate closeout. The meta task applies the human review boundary.

In whole-slice mode, an unexpectedly completed meaningful preparatory packet must return before the next packet. Verify/review and commit that completed packet as applicable, then return `STOPPED` for the unfinished slice assignment with the packet's evidence, remaining work, and mandatory human-review `NEXT`. Do not perform slice closeout or treat the checkpoint as recoverable friction.

Return `ADMISSION_FAILED` before edits when current authority or another admission requirement is insufficient. Return `STOPPED` only for a genuine workflow stop or the explicit whole-slice early human checkpoint defined by the lifecycle.

## Keep execution thin

- Treat the context capsule and skill route as transient dispatch content. Do not persist or return either one, and do not blanket-preload skills.
- Use one focused reviewer per causal review round when review applies. Add specialist input only for a named unresolved measured risk under the review policy.
- Require the review policy's explicit `P1 Critical`, `P2 Required`, `P3 Optional/Consider`, and `P4 Nit` classification. Remediate and re-review blocking `P1`/`P2`; report non-blocking `P3`/`P4` without making them required work or another causal round.
- Keep routing state and live task identities in conversation. Do not create ledgers, selectors, handoffs, proof walls, schemas, validators, dispatch archives, or other durable orchestration state.
- Create no mandatory mechanical second commit and no packet-level slice closeout.
- Follow repository instructions and the six current clean-room documents listed above when this skill is incomplete or ambiguous.
