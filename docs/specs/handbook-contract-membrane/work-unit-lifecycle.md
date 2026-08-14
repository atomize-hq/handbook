# Slice and Packet Work-Unit Lifecycle

This document is the normative lifecycle vocabulary for Handbook contract-membrane work. Humans, the task-session and meta-task prompts, and the thin routing skill consume these rules to identify the authorized outcome, divide it into execution units, and distinguish continuation from completion.

It defines execution-unit identity and lifecycle boundaries only. It does not authorize a slice, prescribe a prompt or result format, or require an orchestration artifact.

## Work units

- A **slice** is the complete authorized outcome. Its authorization defines the outcome conditions and any bounded packets.
- A **packet** is a bounded part of one slice. It never becomes a slice merely because it executes in its own top-level task session.
- The **meta task** selects an execution mode and owns packet selection, sequencing, and continuation across task sessions.
- A **packet task** owns only its assigned packet. It may not claim authority over the rest of the slice.

## Execution modes

A slice may execute in either mode:

- **Whole-slice mode:** one top-level task session performs the authorized slice.
- **Packet-by-packet mode:** one or more packets execute as separate top-level task sessions under the continuing meta task.

Both modes are valid. Choosing packet-by-packet mode changes the execution boundary, not the slice authority or completion boundary. The meta task remains responsible for deciding whether the next authorized packet can begin.

## Completion boundaries

**Packet completion** means the packet's acceptance and verification boundary is satisfied. The packet task returns a compact result to the meta task. It does not claim slice completion, perform whole-slice closeout, or create durable orchestration records merely to transfer control.

**Slice completion** means every authorized packet and outcome condition is complete. Perform full closeout once, at that point. In this document, full closeout names a lifecycle point; it does not imply a ledger, handoff schema, proof wall, selector history, review cycle, or multi-commit sequence.

## Genuine workflow stops

A workflow stop is genuine only when safe continuation is prevented by at least one of these conditions:

- a required authority or human decision;
- material scope expansion outside the current authorization;
- an external blocker or required external-state dependency;
- an unavailable required capability; or
- an unavoidable context or runtime boundary.

At a genuine stop, record only the durable stop information needed for safe resumption and return control without calling the slice complete. This is a stop closeout, not whole-slice closeout.

The following are not genuine workflow stops by themselves:

- a review finding;
- an internal delegation boundary;
- an intermediate packet completing;
- ordinary remediation;
- a new task session starting; or
- a transient delay.

If admission fails before product edits, return a bounded result and stop. Do not manufacture commits, selector history, handoffs, ledgers, proof walls, review cycles, or mechanical closeout artifacts. A later prompt may define the result's presentation without changing this lifecycle rule.

## State transitions

| Event | Lifecycle result | Owner of the next decision | Closeout |
| --- | --- | --- | --- |
| Packet completes and authorized packets remain | Packet complete; slice remains active | Meta task selects or sequences the next packet | None at slice level |
| Final packet completes and all outcome conditions are satisfied | Slice complete | Meta task confirms completion | Full closeout once |
| Whole-slice work satisfies all outcome conditions | Slice complete | Current top-level task confirms completion | Full closeout once |
| A genuine workflow-stop condition prevents safe continuation | Slice stopped, not complete | Required authority or resuming task after the condition clears | Only durable stop information needed for safe resumption |
| Admission fails before product edits | Attempt ends; slice completion state is unchanged | Meta task or required authority | Bounded result only; no manufactured artifacts |

Any later prompt, skill, or validator must preserve these identities and transitions. It may make execution more convenient, but it may not redefine packet completion as slice completion or add a new lifecycle boundary.
