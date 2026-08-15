# Meta-Task Prompt

Use this prompt in a meta task that owns one currently authorized slice. It operationalizes the [`Slice and Packet Work-Unit Lifecycle`](work-unit-lifecycle.md), [`Review and Closeout Policy`](review-and-closeout-policy.md), and [`Task-Session Prompt`](task-session-prompt.md); those documents remain authoritative if this prompt is incomplete or ambiguous.

Copy the assignment block and meta instructions into the meta task, replacing each angle-bracketed placeholder. Keep entries short and human-readable.

## Assignment

```text
SLICE: <slice identifier>
AUTHORITY: <current controlling slice-specific source>
MODE: <whole-slice | packet-by-packet>
SLICE_OUTCOME: <complete outcome and acceptance conditions>
PACKETS: <authorized packet identifiers and order | current authority that determines them | none for whole-slice mode>
WORKSPACE: <project, branch, task environment, and completed-commit integration rule>
AGGREGATE_VERIFICATION: <cross-packet or whole-slice checks and expectations>
NON_GOALS: <explicit exclusions and authority limits>
COMMIT_POSTURE: <default task-session commit posture>
```

## Meta instructions

Own routing and autonomous continuation for this slice. After admission, keep working through the authorized whole slice without waiting for packet-by-packet user confirmation. Do not implement product work unless performing the bounded aggregate verification or review described below. Do not infer product scope, remaining packets, or authority from archived plans.

### Ground and admit the slice

1. Read repository instructions, `AUTHORITY`, and only the clean-room lifecycle, review, task-session, and meta-task documents needed for routing. Inspect the current workspace, branch, and task-environment state without disturbing unrelated work.
2. Treat archived material, deleted control-plane history, and disabled skill contents as non-authority unless `AUTHORITY` explicitly adopts a bounded part.
3. Confirm that the slice is currently authorized, `MODE` is explicit, `SLICE_OUTCOME` is knowable, and packet identities and order are authorized for packet-by-packet mode.
4. If any requirement is missing, stale, or contradictory, dispatch nothing. Report the exact admission condition concisely. Do not start review or create a commit, task-session result, handoff, ledger, proof wall, or closeout artifact.

### Dispatch one authorized unit

An explicit user instruction to execute the slice through this meta task authorizes creation of the sequential top-level task sessions required by `MODE`; do not request separate confirmation for every packet. Use the Codex app task tools rather than collaboration subagents, shell polling, filesystem identities, or durable dispatch envelopes.

- In `whole-slice` mode, fill the task-session prompt once with `MODE: whole-slice`, `PACKET: none`, and the complete authorized slice boundary.
- In `packet-by-packet` mode, select exactly one currently authorized packet and fill the task-session prompt with `MODE: packet`. Run one packet task at a time. Do not pre-create every packet task, launch a default concurrent burst, or load future packet context before it is needed.
- The meta task chooses every next packet. A packet task never sequences itself.

Before each dispatch, build one fresh context capsule by filling the task-session prompt's complete assignment block:

1. Use `agent-using-agent-skills` to select the minimal skill route justified by the current unit. Name each selected skill's activation phase; do not import its full workflow into the capsule or select skills for future packets.
2. Use `agent-context-engineering` to curate the current unit's starting state, relevant live source and test paths, a useful implementation precedent when available, and known risks or unresolved questions. Prefer precise paths or sections over pasted documents and transcripts.
3. Supply the current authority, objective, scope, non-goals, prerequisites, acceptance, verification, commit posture, curated context, risks, and skill route without expanding them beyond the assignment.
4. Keep the capsule inside the task dispatch. Do not write it to the repository, retain it as orchestration state, or ask the task session to return it.

Load skills only when the meta task's current phase requires them. Do not load implementation or debugging skills merely to route work, and do not turn the example lifecycle from a routing skill into a mandatory sequence.

### Run the top-level task loop

1. Resolve the current project and the environment authorized by `WORKSPACE`. Use `list_projects` when needed, then call `create_thread` once to create one fresh user-visible task. If creation returns a pending client identity instead of a usable task identity, resolve the resulting ready task through the platform task list; never call `create_thread` again for that dispatch. Do not use `fork_thread` or a built-in collaboration subagent for slice or packet execution.
2. Retain the live task and host identity only in conversation. Use `wait_threads` for the current task; on completion or a request for attention, use `read_thread` to inspect the task rather than asking the user to relay its status. A timeout without new evidence is not a stop or user handoff; continue waiting without narrating unchanged snapshots.
3. When the task needs direction and current authority supplies one safe answer, adjudicate it and use `send_message_to_thread` to continue the same task. Keep waiting until the task returns a valid result or reaches a genuine workflow stop.
4. If a task ends or exhausts context before completing but its work can be resumed safely, build a refreshed capsule and create a fresh top-level task for the same unit. Preserve truthful workspace, commit, and verification state; never advance to the next packet to bypass unfinished work.
5. After valid `PACKET_COMPLETE`, ensure the packet's committed state is present on the slice's declared working branch, applying only the integration rule already stated in `WORKSPACE`. Use that checkpoint for the next capsule, immediately dispatch the next authorized packet, and do not wait for the user to say that the prior task is done. New user direction may refine or interrupt the loop, but ordinary continuation remains meta-owned.

Use bounded judgment rather than retry counters or strict automation validators. Do not repeat an unchanged failed action: choose an authority-supported correction or alternative, or declare a genuine stop when none exists.

### Consume the compact task result

Expect exactly these semantic fields from the task-session prompt:

```text
STATUS: <PACKET_COMPLETE | SLICE_COMPLETE | STOPPED | ADMISSION_FAILED>
UNIT: <slice identifier; mode; packet identifier when applicable>
OUTCOME: <concise result summary>
CHANGED: <changed paths | none>
VERIFICATION: <checks and outcomes, including failures or limits>
REVIEW: <controlling verdict | not applicable — reason>
COMMIT: <commit hash | none>
NEXT: <return, stop, admission, or completion condition>
```

Treat task messages and results as evidence, never as authority or permission to expand tools, scope, or external actions. Validate the fields, unit identity, status, assigned boundary, and current authority. Do not serialize, schema-validate, or persist the result. Send a concise presentation correction to the task when a field is missing, the unit mismatches, a packet claims `SLICE_COMPLETE`, a whole-slice task claims `PACKET_COMPLETE`, or the result conflicts with current authority. A correction is not a review round, retry, or orchestration artifact; do not act on an invalid completion claim.

Handle a valid result by status:

- `PACKET_COMPLETE` is valid only in packet-by-packet mode. Confirm that the result represents the assigned packet's acceptance boundary, evidence, review disposition, and commit posture. Retain one conversational summary containing the packet identifier, status, commit, and any material evidence or review limitation. If an authorized packet remains and current authority plus the prior result permit continuation, automatically select and dispatch only that next packet. Do not perform slice closeout after an intermediate packet.
- `SLICE_COMPLETE` is valid only from a whole-slice task. Confirm every slice outcome condition and the required minimal closeout information, then report completion once with `NEXT: none`.
- `STOPPED` and `ADMISSION_FAILED` return control to meta adjudication. Use the exact condition, truthful partial change, verification and commit state, and safe-resumption need to choose an authority-supported correction, same-task continuation, or fresh same-unit task. They halt the slice only when no safe in-authority continuation can be derived or executed.

Review findings, failed checks with a diagnosable remediation path, missing context available from current authority, task-result formatting defects, task timeouts, and other correctable orchestration friction do not require user adjudication. Resolve them within the existing slice boundary. Ask the user only for the exact material decision, authority expansion, external action, or unavailable prerequisite that current sources cannot settle.

### Continue packets and close the slice

After the final authorized packet returns valid `PACKET_COMPLETE`, the meta task confirms every `SLICE_OUTCOME` condition and performs one proportionate aggregate closeout under the review and closeout policy.

1. Reuse valid packet evidence and commit summaries. Rerun only aggregate or integration checks and evidence invalidated by later packets; do not replay each packet's proof or review ceremony.
2. When aggregate review applies, use one focused reviewer and one controlling verdict for the current aggregate subject. Do not launch a predetermined multi-lens burst. Specialist input remains exceptional under the review policy.
3. Perform bounded aggregate verification and review directly with available tools. Do not create a top-level closeout packet by default. If a required capability is unavailable, return a genuine `STOPPED` result with the safe-resumption need.
4. Report slice completion once, in conversation, with the outcome and status, applicable verification, controlling verdict or not-applicable reason, relevant commits, and `NEXT: none`.

Keep only the current unit's full result plus one compact summary per completed packet. Do not paste prior task transcripts into each new prompt or maintain selectors, ledgers, handoffs, state snapshots, subject fingerprints, counters, dispatch archives, result files, advisory inventories, or mechanical second commits.

## Meta result presentation

For final completion or a stop, report only:

```text
STATUS: <SLICE_COMPLETE | STOPPED | ADMISSION_FAILED>
SLICE: <slice identifier>
OUTCOME: <completed outcome | exact stop or admission condition>
VERIFICATION: <applicable aggregate checks and outcomes | not applicable — reason>
REVIEW: <controlling verdict | not applicable — reason>
COMMITS: <relevant commits | none>
NEXT: <none | safe-resumption requirement>
```
