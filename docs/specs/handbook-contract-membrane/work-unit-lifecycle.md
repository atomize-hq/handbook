# Slice and Packet Work-Unit Lifecycle

This document is the normative lifecycle vocabulary for Handbook contract-membrane work. Humans, the authority-preparation, task-session, and meta-task prompts, and the thin routing skill consume these rules to identify the authorized outcome, prepare missing authority at the correct level, divide it into execution units, and distinguish continuation from completion.

It defines execution-unit identity and lifecycle boundaries only. It does not authorize a slice, prescribe a prompt or result format, or require an orchestration artifact.

## Work units

- A **slice** is the complete authorized outcome. Its authorization defines the outcome conditions, any bounded packet sequence, and whether named packets may resolve their remaining in-scope decisions just in time.
- A **packet** is a bounded part of one slice. It never becomes a slice merely because it executes in its own top-level task session.
- The **meta task** selects an execution mode and owns packet selection, automatic sequencing, adjudication, and continuation across task sessions.
- A **task session** owns only its assigned whole slice or packet. It may work directly or delegate internally only to local subagents; it never dispatches or coordinates another user-visible top-level task.
- A **packet task** may not claim authority over the rest of the slice.

## Authority-preparation boundaries

Authority preparation is planning work performed in a fresh top-level task session before product execution. It is not a phase, slice, or packet implementation result and cannot claim any of those outcomes complete.

- A **phase shape** states a phase's high-level outcome but may not yet define its constituent slices. When an execution request reaches that gap, the meta task must ask whether it should dispatch phase-level decomposition unless the user already explicitly requested that decomposition. After confirmation, the phase task defines only ordered slice outcomes, dependencies, cross-slice boundaries, and phase-exit conditions. It cannot define packets, create individual slice authority, implement product work, or dispatch a slice.
- A **slice shape** is a phase or roadmap entry that names one slice and its high-level outcome without supplying current slice authority or packet decomposition. An explicit instruction to execute that named slice authorizes the meta task to prepare its bounded slice authority; it does not by itself supply the missing technical decisions or authorize product edits before that authority is ready.
- A **slice-authority task** defines exactly one slice's outcome, boundaries, dependencies, acceptance, ordered packets, and any bounded just-in-time packet-preparation delegation. It cannot implement the slice, prepare an adjacent slice, or dispatch Packet 1.
- The **meta task** alone sequences these boundaries. It regains control after phase preparation, selects the next named slice, dispatches a separate slice-authority task, validates the result, and only then begins whole-slice or packet execution.

Each preparation task uses the [`Authority-Preparation Prompt`](authority-preparation-prompt.md), current source and tests, current program and architecture sources, and only bounded historical inputs explicitly admitted for the subject. Verifiable stale status prose is correctable planning work. An incomplete or unproven hard predecessor is recorded and blocks product execution, but does not block bounded authority preparation unless its intended outcome is itself too undefined or contradictory to support safe decomposition. An absent high-level phase outcome, unauthorized phase decomposition, or consequential choice with no single answer in permitted sources requires exact escalation rather than invention.

## Execution modes

A slice may execute in either mode:

- **Whole-slice mode:** one top-level task session performs the authorized slice.
- **Packet-by-packet mode:** one or more packets execute as separate top-level task sessions under the continuing meta task.

Both modes are valid. Choosing packet-by-packet mode changes the execution boundary, not the slice authority or completion boundary. An explicit instruction to execute an admitted slice delegates continuation through its declared packet sequence without packet-by-packet human confirmation.

## Just-in-time packet authority

A separate packet-authority file is useful when a packet's material decisions are already resolved, but it is not a universal admission requirement. A current slice authority may explicitly delegate bounded just-in-time preparation for a named packet when:

- the slice declares the packet's outcome, order, and non-goals;
- every predecessor is complete;
- the sources permitted for resolving the packet are identified; and
- preparation stays inside the authorized slice outcome.

In that case, the meta task dispatches the packet normally. Before product edits, the same fresh top-level packet task examines current slice authority, live source and tests, current architecture and program sources, and only the bounded historical planning that current authority explicitly adopts. If those sources yield one safe answer, the task records a compact current authority or decision checkpoint when useful, reviews a material decision under the causal-review policy, and proceeds with implementation in the same task. The checkpoint records or refines the slice-delegated answer; it does not create its own scope.

If the permitted sources conflict or leave more than one consequential answer and current authority cannot select one, the packet returns `STOPPED` before product edits with the exact missing decision. Missing packet documentation alone is correctable preparation work, not `ADMISSION_FAILED` or a slice stop. Missing slice authorization, an unnamed packet, an unsatisfied predecessor, scope outside the slice, or an actually unresolved material decision still prevents execution.

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
- an absent packet-authority file when the slice delegates bounded just-in-time preparation;
- absent slice authority when a user-requested roadmap slice has a current high-level outcome and can enter bounded slice-authority preparation;
- verifiable predecessor or program-status documentation drift that the assigned preparation scope can correct;
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
| A phase shape exists but its slices are absent and phase decomposition is not explicitly approved | Phase execution does not begin | Meta task asks the user whether to dispatch one phase-authority task | None |
| Approved phase preparation defines the ordered slices | Authority ready; no slice or packet is implemented | Meta task validates it and separately selects a slice for slice-authority preparation | None |
| A requested named slice has a current high-level shape but lacks slice authority or packets | Slice execution waits on bounded preparation | Meta task dispatches one slice-authority task without another human gate | None |
| Slice preparation defines current outcome authority and ordered packets | Authority ready; slice remains unimplemented | Meta task validates it, proves hard predecessors, and dispatches the first eligible execution task only when they are complete | None |
| Packet completes and declared packets remain | Packet complete; slice remains active | Meta task integrates its checkpoint when needed and automatically sequences the next packet, including delegated just-in-time preparation | None at slice level |
| A named next packet lacks a separate authority file but satisfies the slice's preparation delegation | Slice and packet remain admitted | Meta task dispatches the packet; that same task prepares bounded authority before product edits | None at slice level |
| Final packet completes and all outcome conditions are satisfied | Slice complete | Meta task confirms completion | Full closeout once |
| Whole-slice work satisfies all outcome conditions | Slice complete | Current top-level task confirms completion | Full closeout once |
| A task reports recoverable admission or execution friction | Current unit and slice remain active | Meta task corrects, resumes, or replaces the current task | None at slice level |
| No safe in-authority continuation can be derived or executed | Slice stopped, not complete | Required authority or resuming meta task after the condition clears | Only stop information needed for safe resumption |
| Initial slice admission fails before any task dispatch | Slice does not begin | Meta task resolves it when possible; otherwise required authority | Bounded result only; no manufactured artifacts |

Any later prompt, skill, or validator must preserve these identities and transitions. It may make execution more convenient, but it may not redefine packet completion as slice completion or add a new lifecycle boundary.
