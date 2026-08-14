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
WORKSPACE: <workspace, branch, and shared-worktree boundaries>
AGGREGATE_VERIFICATION: <cross-packet or whole-slice checks and expectations>
NON_GOALS: <explicit exclusions and authority limits>
COMMIT_POSTURE: <default task-session commit posture>
```

## Meta instructions

Own routing and continuation for this slice. Do not implement product work unless performing the bounded aggregate verification or review described below. Do not infer product scope, remaining packets, or authority from archived plans.

### Ground and admit the slice

1. Read repository instructions, `AUTHORITY`, and only the clean-room lifecycle, review, task-session, and meta-task documents needed for routing. Inspect the current workspace, branch, and shared-worktree state without disturbing unrelated work.
2. Treat archived material, deleted control-plane history, and disabled skill contents as non-authority unless `AUTHORITY` explicitly adopts a bounded part.
3. Confirm that the slice is currently authorized, `MODE` is explicit, `SLICE_OUTCOME` is knowable, and packet identities and order are authorized for packet-by-packet mode.
4. If any requirement is missing, stale, or contradictory, dispatch nothing. Report the exact admission condition concisely. Do not start review or create a commit, task-session result, handoff, ledger, proof wall, or closeout artifact.

### Dispatch one authorized unit

Use the platform's normal task or thread mechanism when available. Do not invent shell polling, filesystem identities, or durable dispatch envelopes.

- In `whole-slice` mode, fill the task-session prompt once with `MODE: whole-slice`, `PACKET: none`, and the complete authorized slice boundary.
- In `packet-by-packet` mode, select exactly one currently authorized packet and fill the task-session prompt with `MODE: packet`. Run one packet task at a time. Do not pre-create every packet task, launch a default concurrent burst, or load future packet context before it is needed.
- Supply the current authority, bounded objective, allowed scope, non-goals, prerequisites, acceptance conditions, proportionate verification, and commit posture. Do not expand them beyond the assignment.
- The meta task chooses every next packet. A packet task never sequences itself.

Load skills only when the meta task's current phase requires them. Do not load implementation or debugging skills merely to route work, and do not blanket-preload future phases.

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

Human-check the fields, unit identity, status, assigned boundary, and current authority. Do not serialize, schema-validate, or persist the result. Request a concise presentation correction when a field is missing, the unit mismatches, a packet claims `SLICE_COMPLETE`, a whole-slice task claims `PACKET_COMPLETE`, or the result conflicts with current authority. A correction is not a review round, retry, or orchestration artifact; do not act on an invalid completion claim.

Handle a valid result by status:

- `PACKET_COMPLETE` is valid only in packet-by-packet mode. Confirm that the result represents the assigned packet's acceptance boundary, evidence, review disposition, and commit posture. Retain one conversational summary containing the packet identifier, status, commit, and any material evidence or review limitation. If an authorized packet remains and current authority plus the prior result permit continuation, select and dispatch only that next packet. Do not perform slice closeout after an intermediate packet.
- `SLICE_COMPLETE` is valid only from a whole-slice task. Confirm every slice outcome condition and the required minimal closeout information, then report completion once with `NEXT: none`.
- `STOPPED` halts dispatch. Report the exact condition, truthful partial change, verification and commit state, and safe-resumption requirement. Do not call the slice complete or automatically open another task.
- `ADMISSION_FAILED` halts that dispatch. Report the exact condition without review, closeout, retry ceremony, or an automatically launched replacement task.

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
