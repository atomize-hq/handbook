# Slice and Packet Work-Unit Lifecycle

This document is the normative lifecycle vocabulary for Handbook contract-membrane work. Humans, the task-session and meta-task prompts, and the thin routing skill consume these rules to identify the authorized outcome, divide it into execution units, and distinguish continuation from completion.

It defines execution-unit identity and lifecycle boundaries only. It does not authorize a slice, prescribe a prompt or result format, or require an orchestration artifact.

## Work units

- A **slice** is the complete authorized outcome. Its authorization defines the outcome conditions and any bounded packets.
- A **packet** is a bounded part of one slice. It never becomes a slice merely because it executes in its own top-level task session.
- The **meta task** selects an execution mode and owns packet selection, automatic sequencing, adjudication, and continuation across task sessions.
- A **task session** owns only its assigned whole slice or packet. It may work directly or delegate internally only to local subagents; it never dispatches or coordinates another user-visible top-level task.
- A **packet task** may not claim authority over the rest of the slice.

## Execution modes

A slice may execute in either mode:

- **Whole-slice mode:** one top-level task session performs the authorized slice.
- **Packet-by-packet mode:** one or more packets execute as separate top-level task sessions under the continuing meta task.

Both modes are valid. Choosing packet-by-packet mode changes the execution boundary, not the slice authority or completion boundary. Once the slice is admitted, the meta task continues through authorized packets without requiring packet-by-packet human confirmation.

## Completion boundaries

**Packet completion** means the packet's acceptance and verification boundary is satisfied. The packet task returns a compact result to the meta task. It does not claim slice completion, perform whole-slice closeout, or create durable orchestration records merely to transfer control.

**Slice completion** means every authorized packet and outcome condition is complete. Perform full closeout once, at that point. In this document, full closeout names a lifecycle point; it does not imply a ledger, handoff schema, proof wall, selector history, review cycle, or multi-commit sequence.

## Adjudication and genuine workflow stops

A task-session `STOPPED` or `ADMISSION_FAILED` result is an adjudication event, not automatically a slice stop. The meta task first uses current authority, planning, live repository state, and the task result to identify one safe in-scope continuation. It may correct the task, refresh its context, direct remediation, resume it, or create a fresh top-level task for the same unit.

A slice stops only when the meta task cannot derive or execute a safe continuation after bounded recovery because:

- a material decision has no single answer in current authority and requires new human or product authority;
- continuation would expand scope or perform an unapproved irreversible or external action; or
- an external-state, capability, workspace, context, or runtime condition leaves no executable in-authority alternative.

At a genuine stop, report only the stop information needed for safe resumption and return control without calling the slice complete. This is a stop closeout, not whole-slice closeout and does not require a file.

The following are not genuine workflow stops by themselves:

- a review finding;
- a failed check with an authority-supported diagnostic or remediation path;
- a malformed task result or stale context capsule that the meta task can correct;
- an internal delegation boundary;
- an intermediate packet completing;
- ordinary remediation;
- a child task timeout or context boundary that a fresh task can resume safely;
- a new task session starting; or
- a transient delay.

If task-session admission fails before product edits, return a bounded result to the meta task for adjudication. Do not manufacture commits, selector history, handoffs, ledgers, proof walls, review cycles, or mechanical closeout artifacts. Escalate beyond the meta task only when its current authority cannot resolve the failure.

## State transitions

| Event | Lifecycle result | Owner of the next decision | Closeout |
| --- | --- | --- | --- |
| Packet completes and authorized packets remain | Packet complete; slice remains active | Meta task integrates its checkpoint when needed and automatically sequences the next packet | None at slice level |
| Final packet completes and all outcome conditions are satisfied | Slice complete | Meta task confirms completion | Full closeout once |
| Whole-slice work satisfies all outcome conditions | Slice complete | Current top-level task confirms completion | Full closeout once |
| A task reports recoverable admission or execution friction | Current unit and slice remain active | Meta task corrects, resumes, or replaces the current task | None at slice level |
| No safe in-authority continuation can be derived or executed | Slice stopped, not complete | Required authority or resuming meta task after the condition clears | Only stop information needed for safe resumption |
| Initial slice admission fails before any task dispatch | Slice does not begin | Meta task resolves it when possible; otherwise required authority | Bounded result only; no manufactured artifacts |

Any later prompt, skill, or validator must preserve these identities and transitions. It may make execution more convenient, but it may not redefine packet completion as slice completion or add a new lifecycle boundary.
