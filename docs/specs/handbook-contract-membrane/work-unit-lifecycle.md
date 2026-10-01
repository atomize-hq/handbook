# Slice and Packet Work-Unit Lifecycle

This document is the normative lifecycle vocabulary for Handbook contract-membrane work. Humans, the authority-preparation, task-session, and meta-task prompts, and the thin routing skill consume these rules to identify the authorized outcome, prepare missing authority at the correct level, divide it into execution units, and distinguish continuation from completion.

It defines execution-unit identity and lifecycle boundaries only. It does not authorize a slice, prescribe a prompt or result format, or require an orchestration artifact.

## Work units

- A **slice** is the complete authorized outcome. Its authorization defines the outcome conditions, any bounded packet sequence, and whether named packets may resolve their remaining in-scope decisions just in time.
- A **packet** is a bounded part of one slice. It never becomes a slice merely because it executes in its own top-level task session.
- The **meta task** selects an execution mode and a human review boundary, and owns packet selection, adjudication, and continuation within the approved increment across task sessions.
- A **task session** owns only its assigned whole slice or packet. It may work directly or delegate internally only to local subagents; it never dispatches or coordinates another user-visible top-level task.
- A **packet task** may not claim authority over the rest of the slice.

## Authority-preparation boundaries

Authority preparation is planning work performed in a fresh top-level task session before product execution. It is not a phase, slice, or packet implementation result and cannot claim any of those outcomes complete.

- A **phase shape** states a phase's high-level outcome but may not yet define its constituent slices. When an execution request reaches that gap, the meta task must ask whether it should dispatch phase-level decomposition unless the user already explicitly requested that decomposition. After confirmation, the phase task defines only ordered slice outcomes, dependencies, cross-slice boundaries, and phase-exit conditions. It cannot define packets, create individual slice authority, implement product work, or dispatch a slice.
- A **slice shape** is a phase or roadmap entry that names one slice and its high-level outcome without supplying current slice authority or packet decomposition. An explicit instruction to execute that named slice authorizes the meta task to prepare its bounded slice authority; it does not by itself supply the missing technical decisions or authorize product edits before that authority is ready.
- A **slice-authority task** defines exactly one slice's outcome, boundaries, dependencies, acceptance, packet outline, human review boundaries, and any bounded just-in-time packet-preparation delegation. Detail only the next increment's plan; later packet outcomes and dependencies may remain broad. It cannot implement the slice, prepare an adjacent slice, or dispatch Packet 1.
- The **meta task** alone sequences these boundaries. It regains control after phase preparation, selects the next named slice, dispatches a separate slice-authority task, and validates the result. Product execution requires human authorization of the next bounded increment's plan as well as satisfied implementation prerequisites.

Each preparation task uses the [`Authority-Preparation Prompt`](authority-preparation-prompt.md), current source and tests, current program and architecture sources, and only bounded historical inputs explicitly admitted for the subject. Verifiable stale status prose is correctable planning work. An incomplete or unproven hard predecessor is recorded and blocks product execution, but does not block bounded authority preparation unless its intended outcome is itself too undefined or contradictory to support safe decomposition. An absent high-level phase outcome, unauthorized phase decomposition, or consequential choice with no single answer in permitted sources requires exact escalation rather than invention.

## Execution modes

A slice may execute in either mode:

- **Whole-slice mode:** one top-level task session performs the authorized slice.
- **Packet-by-packet mode:** one or more packets execute as separate top-level task sessions under the continuing meta task.

Both modes are valid. Execution mode does not determine the human review boundary or change slice identity and completion. Authorization covers only the approved increment up to its human review boundary. It may cover several preparatory packet sessions within that increment, but an admitted slice or a broad execution request does not by itself authorize crossing a human review boundary.

## Human review boundaries

A **review increment** ends where a packet or slice delivers one explainable, observable or notable change. Choose and explain that boundary in the plan before product execution:

- Use a **packet boundary** when the packet itself delivers a meaningful change in behavior, permitted use, refusal, authority, or another independently assessable outcome. An owner or API example can demonstrate it; a visible GUI change is not required.
- Use a **slice boundary** when its packets are preparatory steps and the slice is the first point that delivers such an outcome. Explain why the intermediate packets do not yet deliver one. Multiple task sessions or changed files alone do not require separate human checkpoints.
- Do not group independently meaningful packet outcomes under a slice boundary to postpone review. If whole-slice execution would cross an earlier meaningful packet outcome, use packet execution and stop there.

For each increment, the meta task presents a short plan: why it is next, existing prerequisites, intended behavior and a meaningful refusal or failure case, scope and owner/interface/persistence boundaries, verification, and the chosen review boundary. The human discusses and authorizes that bounded plan before implementation. Plan only the next increment in detail; use broad outcomes and dependencies for later work.

After implementation, proportionate verification, applicable independent review, and any required commit/integration, return to the persistent control conversation with the [human recap](review-and-closeout-policy.md#human-review-recap) and **stop**. The human reviews and discusses the result before successor planning or dispatch. The recap may recommend what should come next and why, but it may not elaborate or launch the successor.

If a preparatory packet unexpectedly delivers an observable or notable change before the selected slice boundary, execution stops there for human review rather than advancing. In packet mode, the task returns the completed packet normally and the meta task applies the checkpoint. In whole-slice mode, the task must return before the next packet: verify and review the completed packet as applicable, perform any required commit, and return `STOPPED` for the incomplete whole-slice assignment. Identify the completed declared packet, its established behavior and evidence, remaining slice work, and the required human review in `NEXT`. The meta task presents the recap and stops; it must not use generic recovery to resume the remainder. A missing or ambiguous boundary cannot authorize continuation; resolve it before dispatch. Do not silently move an approved boundary later.

Human review is distinct from causal technical review and from whole-slice closeout. Semantic approval, satisfied prerequisites, `AUTHORITY_READY`, an automated clean verdict, or the existence of the next packet cannot supply successor authorization. A completed assigned packet or slice awaiting human review retains its truthful completion status. In the exceptional whole-slice early return, `STOPPED` describes only the unfinished assignment; the completed packet remains complete and the recap must not describe it as failed.

## Just-in-time packet authority

A separate packet-authority file is useful when a packet's material decisions are already resolved, but it is not a universal admission requirement. A current slice authority may explicitly delegate bounded just-in-time preparation for a named packet when:

- the slice declares the packet's outcome, order, and non-goals;
- every predecessor is complete;
- the sources permitted for resolving the packet are identified; and
- preparation stays inside the authorized slice outcome.

In that case, the meta task dispatches the packet normally. Before product edits, the same fresh top-level packet task examines current slice authority, live source and tests, current architecture and program sources, and only the bounded historical planning that current authority explicitly adopts. If those sources yield one safe answer, the task records a compact current authority or decision checkpoint when useful, reviews a material decision under the causal-review policy, and proceeds with implementation in the same task. The checkpoint records or refines the slice-delegated answer; it does not create its own scope.

If the permitted sources conflict or leave more than one consequential answer and current authority cannot select one, the packet returns `STOPPED` before product edits with the exact missing decision. Missing packet documentation alone is correctable preparation work, not `ADMISSION_FAILED` or a slice stop. Missing slice authorization, an unnamed packet, an unsatisfied predecessor, scope outside the slice, or an actually unresolved material decision still prevents execution.

## Completion boundaries

**Packet completion** means the packet's acceptance and verification boundary is satisfied. The packet task returns a compact result to the meta task. It does not claim slice completion, perform whole-slice closeout, or create durable orchestration records merely to transfer control. The meta task then applies the human review boundary before any continuation.

**Slice completion** means every authorized packet and outcome condition is complete. Perform full closeout once, at that point. In this document, full closeout names a lifecycle point; it does not imply a ledger, handoff schema, proof wall, selector history, review cycle, or multi-commit sequence.

## Adjudication and genuine workflow stops

A task-session `STOPPED` or `ADMISSION_FAILED` result is an adjudication event, not automatically a slice stop. Except for the whole-slice early human checkpoint above, the meta task first uses current authority, planning, live repository state, and the task result to identify one safe in-scope continuation. It may correct the task, refresh its context, direct remediation, resume it, or create a fresh top-level task for the same unit within the approved increment. The early human checkpoint returns to the human instead; remaining work needs a newly authorized bounded plan after review.

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
- an intermediate preparatory packet completing within the approved increment (a human review boundary still requires the deliberate return-and-stop above);
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
| Slice preparation defines current outcome authority and packet outline | Authority ready; slice remains unimplemented | Meta task validates it, presents the next increment's plan for human authorization, and proves hard predecessors before dispatch | None |
| A packet completes at a human review boundary, or unexpectedly delivers an earlier observable or notable change | Packet complete; slice remains incomplete unless its final outcome is satisfied | Meta task presents the human recap and stops; human review precedes successor planning or dispatch | None at slice level |
| Whole-slice execution unexpectedly completes an earlier meaningful declared packet | Completed packet retained; unfinished whole-slice assignment returns `STOPPED` | Meta task presents the packet recap and stops for human review; no generic recovery or remainder dispatch | No slice closeout; truthful partial state only |
| A preparatory packet completes before the approved slice review boundary | Packet complete; approved increment remains active | Meta task may dispatch only the next eligible preparatory packet within that approved increment | None at slice level |
| A named next packet lacks a separate authority file but satisfies the slice's preparation delegation | Packet may execute only within the human-approved increment | Meta task dispatches the packet; that same task prepares bounded authority before product edits | None at slice level |
| Final packet completes and all outcome conditions are satisfied | Slice complete | Meta task confirms completion, presents the human recap, and stops before successor planning or dispatch | Full closeout once |
| Whole-slice work satisfies all outcome conditions | Slice complete | Current top-level task returns completion; meta task presents the human recap and stops before successor planning or dispatch | Full closeout once |
| A task reports recoverable admission or execution friction | Current unit and slice remain active | Meta task corrects, resumes, or replaces the current task | None at slice level |
| No safe in-authority continuation can be derived or executed | Slice stopped, not complete | Required authority or resuming meta task after the condition clears | Only stop information needed for safe resumption |
| Initial slice admission fails before any task dispatch | Slice does not begin | Meta task resolves it when possible; otherwise required authority | Bounded result only; no manufactured artifacts |

Any later prompt, skill, or validator must preserve these identities and transitions. It may make execution more convenient, but it may not redefine packet completion as slice completion or add a new lifecycle boundary.
