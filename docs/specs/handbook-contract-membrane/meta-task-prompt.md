# Meta-Task Prompt

Use this prompt in a meta task that owns an HCM phase or slice request. The request may already have current slice authority or may require bounded phase-to-slice or slice-to-packet authority preparation first. This prompt operationalizes the [`Slice and Packet Work-Unit Lifecycle`](work-unit-lifecycle.md), [`Review and Closeout Policy`](review-and-closeout-policy.md), [`Authority-Preparation Prompt`](authority-preparation-prompt.md), and [`Task-Session Prompt`](task-session-prompt.md); those documents remain authoritative if this prompt is incomplete or ambiguous.

## Assignment

```text
REQUEST: <user instruction and intended terminal action>
PHASE: <phase identifier>
SLICE: <slice identifier | next | unresolved>
AUTHORITY: <current controlling phase or slice sources | missing>
MODE: <whole-slice | packet-by-packet | unresolved until slice preparation>
SLICE_OUTCOME: <complete outcome and acceptance | unresolved until slice preparation>
PACKETS: <declared packet order and preparation delegation | unresolved until slice preparation | none>
HUMAN_REVIEW_BOUNDARY: <packet or slice endpoint for the next increment, delivered outcome and preparatory-packet rationale | unresolved until planning>
PLAN_AUTHORIZATION: <human instruction authorizing that bounded plan | pending human review>
WORKSPACE: <project, branch, task environment, and completed-commit integration rule>
AGGREGATE_VERIFICATION: <cross-packet or whole-slice expectations | unresolved>
NON_GOALS: <explicit exclusions and authority limits>
COMMIT_POSTURE: <default top-level-task commit posture>
```

## Meta instructions

Own authority preparation, routing, and continuation within the human-approved increment for `REQUEST`. Do not implement product work except bounded aggregate verification or review. Stop in this control conversation for human review when a packet or slice delivers the selected observable or notable change, or an earlier packet unexpectedly delivers one. Do not plan or dispatch its successor until the human has reviewed the recap and authorized further planning.

### Classify the authority state

1. Read repository instructions, the current authority entry point, lifecycle, review policy, authority-preparation prompt, task-session prompt, and only the phase or slice sources needed to classify `REQUEST`. Inspect the workspace without disturbing unrelated work.
2. Treat archived material and disabled skills as non-authority unless a current source adopts a bounded part. Treat current source and tests as product truth; a predecessor-status claim contradicted by verifiable commits, source, and tests is correctable documentation drift, not an automatic stop.
3. Select exactly one state:
   - **Slice authority ready:** current slice authority defines the outcome, boundaries, mode, and ordered packets or whole-slice execution. Product dispatch additionally requires proven hard predecessors, a selected human review boundary, and human authorization of the next increment's bounded plan.
   - **Slice preparation required:** `REQUEST` explicitly asks to execute a named slice, and a current phase or roadmap source names its high-level outcome, but current slice authority or packet decomposition is absent.
   - **Phase preparation requires confirmation:** a high-level phase shape exists, but its slices are undefined and the user has not explicitly requested phase decomposition. Dispatch nothing. Ask one direct question: whether to dispatch a phase-level top-level task that defines slices only.
   - **Phase preparation approved:** the user explicitly requested or confirmed decomposition of that phase.
   - **Genuine admission gap:** no current high-level phase outcome, no requested slice shape, or another consequential authority choice prevents bounded preparation.
4. Do not return `ADMISSION_FAILED` merely because a requested roadmap slice lacks its own directory, packet list, or detailed authority. Use the preparation path when its conditions hold. Report only a genuine admission gap that bounded preparation and current evidence cannot resolve.

### Prepare missing authority

For phase or slice preparation, use the Codex app task tools to create one fresh user-visible top-level task. Never use a collaboration subagent for that layer.

1. Fill the authority-preparation prompt as a fresh context capsule. Use `agent-using-agent-skills` to select the smallest route for this planning subject and `agent-context-engineering` to curate current source, tests, architecture, seam, status, useful precedent, and bounded adopted historical sections.
2. For **phase preparation**, set `LEVEL: phase`. The task may define only ordered slices, their bounded outcomes and dependencies, cross-slice decisions, and phase-exit conditions in the designated current program authority. It may use high-level API/interface, codebase, domain, source, security, compatibility, migration, or performance skills when the phase actually implicates them. It may not define packets, create individual slice authority, edit product code, or dispatch a slice.
3. For **slice preparation**, set `LEVEL: slice`. The task may define exactly one named slice's current authority and packet decomposition. It may not implement product work, prepare adjacent slices, or dispatch Packet 1.
4. A planning task returns `AUTHORITY_READY` to this meta task. Validate its target, scope, review disposition, commit, and agreement with current evidence; ensure its committed state is present on the declared working branch using only the integration rule in `WORKSPACE`. A phase result returns here before any slice work; then select the next slice named by the new phase authority and create a separate slice-preparation task when `REQUEST` authorizes that planning. A slice result returns here before implementation; reload the new authority and present only the next increment's bounded plan and chosen human review boundary for human authorization. Prove every hard predecessor from current evidence and repair verifiable status drift within authorized scope before dispatch. An incomplete or still-unproven predecessor blocks implementation, not the completed preparation result. `AUTHORITY_READY` and a clean review do not authorize product dispatch.
5. Treat planning-task `STOPPED` or `ADMISSION_FAILED` as adjudication input. Correct and resume or replace the same planning level when current authority supplies one safe answer. Ask the user only for the exact unresolved consequential decision, missing phase-decomposition permission, authority expansion, or unavailable prerequisite.

Do not invoke the generic `agent-spec` wrapper, use its root `SPEC.md`, or preserve generic `tasks/agent-plan.md` and `tasks/todo.md` outputs. The HCM authority-preparation prompt controls the output location, human gate, and return boundary while using the applicable underlying skills for reasoning.

### Dispatch one implementation unit

Before product dispatch, present the next increment's short plan and obtain human authorization of that bounded work. Explain why it is next, prerequisites, intended behavior, a concrete example and meaningful refusal or failure, scope, owner/interface/persistence boundaries, verification, and `HUMAN_REVIEW_BOUNDARY`. An existing human instruction approving that plan is sufficient; do not ask again. A broad request to execute a slice is not approval of an as-yet-unreviewed increment plan.

- Choose a packet review boundary when that packet delivers an independently meaningful observable or notable outcome. Choose a slice boundary only when its packets are preparatory and the slice is the first such outcome; explain that rationale. Do not postpone review past an earlier meaningful packet result or silently move the boundary later.
- In `whole-slice` mode, fill the task-session prompt once with the complete authorized slice boundary only when it does not cross an earlier packet human review boundary.
- In `packet-by-packet` mode, select exactly one eligible packet within the approved increment. It is eligible when current authority names and bounds it, its predecessor is complete, and either packet authority exists or the slice delegates bounded just-in-time preparation within the approved plan. Several preparatory packets may run sequentially up to an approved slice boundary. Never pre-create future packet tasks or load context beyond the approved increment.
- The meta task chooses every next packet. A packet task never sequences itself.

Before each dispatch, use `agent-using-agent-skills` to select the minimal route for the current unit only. Use `agent-context-engineering` to fill one fresh task-session capsule with the current starting state, precise live source and test paths, one useful precedent when available, risks, preparation boundary when applicable, and each selected skill's activation phase. Do not persist or request return of the capsule or preload future skills.

### Run every top-level task loop

Use this same loop for authority-preparation and implementation tasks:

1. Resolve the authorized project and environment. Call `create_thread` once for one fresh user-visible task. If creation returns a pending client identity, resolve the resulting task without creating a duplicate. Do not use `fork_thread`; the created task may delegate internally only to local subagents and may not create another top-level task.
2. Keep the live host/task identity only in conversation. Use `wait_threads` as the only blocking monitor, with `timeoutMs: 600000` normally or `timeoutMs: 1200000` for a particularly large current unit. On timeout, carry forward the cursor and call `wait_threads` again. Never poll with `read_thread`, use a shorter polling interval, or exceed the twenty-minute ceiling.
3. When non-replacing user direction interrupts a wait, apply it and immediately resume `wait_threads` for the same task with the latest cursor and existing timeout. Do not strand or duplicate the task.
4. Only after `wait_threads` reports completion or attention, use `read_thread` with `turnLimit: 5` to inspect recent state. When current authority supplies one safe answer to requested friction, send it with `send_message_to_thread` and return to `wait_threads`.
5. If a task exhausts context but can resume safely, create a fresh top-level task for the same planning level or implementation unit with a refreshed capsule and truthful current state. Never advance a level or packet to bypass unfinished work.

Use bounded judgment instead of retry counters. Do not repeat an unchanged failed action; select an authority-supported correction or stop when none exists.

### Consume compact results

Authority-preparation tasks return the authority prompt's eight fields with `STATUS: AUTHORITY_READY | STOPPED | ADMISSION_FAILED`. Implementation tasks return the task-session prompt's eight fields with `STATUS: PACKET_COMPLETE | SLICE_COMPLETE | STOPPED | ADMISSION_FAILED`.

Validate the fields, unit, assigned boundary, authority, commit posture, and review disposition. Every valid finding must use the review policy's `P1`–`P4` classification. Send a concise presentation correction when fields or classifications are incomplete or a task claims a result outside its level. Do not serialize, schema-validate, or persist results.

- `AUTHORITY_READY` advances only to the meta task. A phase result permits separately authorized slice preparation; a slice result supplies the plan for human review and implementation admission. It never claims product completion or human plan approval.
- `PACKET_COMPLETE` is valid only for the assigned packet. Confirm its acceptance, evidence, review, and commit; ensure the committed state is present on the declared working branch using only the integration rule in `WORKSPACE`. At the selected packet boundary, or if the packet unexpectedly delivers an earlier observable or notable change, present the human recap and stop. Otherwise continue only with an eligible preparatory packet inside the already approved increment. Do not close the slice between packets. For the final packet, perform the single aggregate closeout before the slice recap and stop.
- `SLICE_COMPLETE` is valid only from whole-slice mode. Confirm every outcome condition, present the human recap, and stop before successor planning or dispatch.
- An early whole-slice `STOPPED` result identifying an unexpectedly completed meaningful packet requires the human recap and stop. Preserve that packet's completion and evidence, report the slice as unfinished, and do not resume or replace the task to execute the remainder before human review and approval of its next bounded plan. Other `STOPPED` and `ADMISSION_FAILED` results return to ordinary meta adjudication and halt the workflow only when no safe in-authority continuation can be derived or executed.

Blocking `P1`/`P2`, diagnosable failed checks, correctable status drift, missing context available from current sources, task-result defects, timeouts, and missing packet files covered by slice delegation do not require user adjudication. Report `P3`/`P4` compactly without making them required work or another causal round.

These recovery rules operate within the approved increment. They do not bypass its human review boundary. A completed assigned unit awaiting human review remains complete; do not label it `STOPPED`, resume it merely to advance, or use a clean verdict, admitted slice, or prepared successor as authorization. In the exceptional whole-slice early return, `STOPPED` describes the unfinished assignment, not a failure of the completed packet. The recap may recommend the next increment and why, but detailed successor planning waits for the human's direction, and its implementation waits for approval of its bounded plan.

### Close the slice once

After the final packet returns valid `PACKET_COMPLETE`, confirm every `SLICE_OUTCOME` condition and perform one proportionate aggregate closeout.

1. Reuse valid packet evidence and commit summaries. Rerun only aggregate checks and evidence invalidated by later work.
2. When review applies, use one focused reviewer and one controlling verdict. Only unresolved `P1`/`P2` block completion; `P3`/`P4` remain non-blocking advisories.
3. Do not create a top-level closeout packet, replay packet ceremony, or create a ledger, selector, handoff, proof wall, state snapshot, result file, advisory inventory, or mechanical second commit.
4. Report completion once with the human recap, aggregate verification, controlling verdict, relevant commits, and a clear return-and-stop for human review. Do not dispatch or plan the next slice.

## Meta result presentation

```text
STATUS: <PACKET_COMPLETE | SLICE_COMPLETE | STOPPED | ADMISSION_FAILED>
UNIT: <slice identifier; packet identifier when the human boundary is a packet>
OUTCOME: <plain-English behavior, concrete example, meaningful refusal or failure, and unfinished or excluded work | exact stop or admission condition>
VERIFICATION: <applicable bounded or aggregate checks and outcomes, including limits | not applicable — reason>
REVIEW: <controlling verdict and compact P1–P4 disposition | not applicable — reason>
COMMITS: <relevant commits | none>
NEXT: <human review before successor planning or dispatch; optional next-increment recommendation and why | safe-resumption requirement>
```
