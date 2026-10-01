# Authority-Preparation Prompt

Use this prompt to assign one bounded phase-to-slice or slice-to-packet authority-preparation task to a fresh top-level task session. It operationalizes the [`Slice and Packet Work-Unit Lifecycle`](work-unit-lifecycle.md) and [`Review and Closeout Policy`](review-and-closeout-policy.md). It prepares current authority; it does not implement product work or continue into another planning level.

## Context capsule

```text
LEVEL: <phase | slice>
UNIT: <phase or slice identifier>
REQUEST_AUTHORITY: <user instruction or explicit phase-decomposition confirmation>
OBJECTIVE_SHAPE: <current high-level outcome and boundaries>
CURRENT_AUTHORITY: <current roadmap, phase, slice, architecture, or status sources>
TARGET_AUTHORITY: <current file to create or update>
ALLOWED_SCOPE: <authority and directly necessary status paths>
PREDECESSORS: <required prior outcomes and their current evidence>
NON_GOALS: <product edits and excluded planning levels or adjacent work>
CONTEXT: <minimal live source, test, type, and precedent paths or sections>
HISTORICAL_INPUTS: <bounded adopted historical sections | none>
SKILL_ROUTE: <applicable skills and the phase that activates each>
COMMIT_POSTURE: <commit | read-only | another explicit bounded instruction>
```

## Task instructions

Prepare only the assigned authority level. Repository instructions, `REQUEST_AUTHORITY`, current sources, the lifecycle, and the review policy control the work. The capsule curates context without expanding authority.

### Ground and route skills

1. Verify the branch, worktree, current authority, predecessor evidence, target, and allowed scope. Preserve unrelated work.
2. Treat current product source and tests as product truth. Reconcile verifiable stale status prose instead of treating documentation drift as an admission failure. A hard predecessor need not be complete to prepare later authority: record its truthful status and make completion an implementation admission condition. Stop only when the predecessor's outcome or another current source is too undefined or contradictory to support safe decomposition, or when current sources leave more than one consequential answer.
3. Treat archived material as non-authority unless `HISTORICAL_INPUTS` adopts a bounded section. Extract useful behavior, architecture, or lineage; never restore an old control package wholesale.
4. Use `agent-using-agent-skills` to select the smallest route justified by this planning subject, then load each skill only when its phase begins. Use `agent-context-engineering` to keep context focused. Typical planning skills are `agent-spec-driven-development` and `agent-planning-and-task-breakdown`; select `agent-api-and-interface-design`, `codebase-design`, `domain-modeling`, source, security, migration, or performance guidance only when the phase or slice actually implicates it.
5. Use the underlying skills for their reasoning and verification. This HCM prompt overrides their generic output locations and repeated human gates: do not invoke the `agent-spec` wrapper, create root `SPEC.md`, `tasks/agent-plan.md`, or `tasks/todo.md`, or pause between generic SPECIFY/PLAN/TASKS stages. `REQUEST_AUTHORITY` supplies approval within this bounded preparation task. Return `STOPPED` for an exact unresolved material decision rather than silently guessing.

### Prepare one level

For `LEVEL: phase`:

- Require an explicit user instruction or confirmation to decompose the named phase when its slices are absent.
- Define only the phase outcome, ordered slices, each slice's bounded outcome, hard dependencies, cross-slice invariants or decisions, and phase-exit conditions in the designated current program authority.
- Use high-level source, architecture, interface, seam, domain, compatibility, and external-standard evidence where relevant to shape the slices.
- Do not create packet identities or packet details, create individual slice-authority files, edit product code, or dispatch the first slice. Return control to the meta task.

For `LEVEL: slice`:

- Require a current phase or roadmap source that names the slice and its high-level outcome, plus an explicit user instruction to execute that slice or prepare its authority.
- Define exactly one slice's outcome, scope, non-goals, predecessors and their current status, acceptance boundary, ordered packet outcomes and dependencies, human review boundaries, and any bounded just-in-time packet-preparation delegation. State that product execution remains inadmissible until every hard predecessor is proven complete and the human has authorized the next increment's bounded plan.
- Choose packet or slice review boundaries according to the lifecycle's observable-or-notable-change rule. Explain what each review increment delivers and why any preparatory packets do not yet deliver an independently meaningful outcome. Detail only the next increment's plan, including a concrete example, a meaningful refusal or failure, owner/interface/persistence boundaries, and verification; do not elaborate the entire backlog.
- Adopt historical material only section by section and record any unresolved consequential decision at the narrowest level that must own it.
- Do not implement product work, prepare an adjacent slice, or dispatch Packet 1. Return control to the meta task.

An authority-preparation task never creates or coordinates another user-visible top-level task. It may use only local collaboration subagents internally and remains responsible for their output.

### Verify, review, commit, and return

1. Confirm the target owns one responsibility, contains testable outcomes and boundaries, agrees with current source and controlling documents, and does not duplicate lower-level technical detail.
2. Resolve every local Markdown link and run repository-required checks applicable to the changed subject.
3. When review applies, use one focused causal round and the review policy's `P1`–`P4` rubric. Remediate blocking `P1`/`P2`; report non-blocking `P3`/`P4` without manufacturing another round.
4. With `commit` posture, create one coherent local commit after verification and review. Do not create a mechanical authority/closeout pair, planning ledger, selector, proof wall, handoff, or durable context capsule.
5. Return control without beginning the next planning level or implementation.

## Final result

```text
STATUS: <AUTHORITY_READY | STOPPED | ADMISSION_FAILED>
UNIT: <phase | slice; identifier>
OUTCOME: <concise authority result>
CHANGED: <changed paths | none>
VERIFICATION: <checks and outcomes, including failures or limits>
REVIEW: <controlling verdict and compact P1–P4 disposition | not applicable — reason>
COMMIT: <commit hash | none>
NEXT: <return to meta-task decision | exact resumption need>
```

`AUTHORITY_READY` means the assigned authority level is current and committed when required; it does not mean the phase, slice, or any packet is implemented or that the human has approved the next increment's plan. `STOPPED` and `ADMISSION_FAILED` return the exact condition to meta adjudication and do not authorize the task to cross into another level.
