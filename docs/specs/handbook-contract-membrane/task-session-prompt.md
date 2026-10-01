# Task-Session Prompt

Use this prompt to assign one authorized whole slice or one packet to a top-level task session. The completed assignment block is the task's transient context capsule. It operationalizes the [`Slice and Packet Work-Unit Lifecycle`](work-unit-lifecycle.md) and [`Review and Closeout Policy`](review-and-closeout-policy.md); those documents remain authoritative if this prompt is incomplete or ambiguous.

Copy the assignment block and task instructions into the task session, replacing each angle-bracketed placeholder. Keep entries short and human-readable.

## Task context capsule

```text
MODE: <whole-slice | packet>
SLICE: <slice identifier>
PACKET: <packet identifier | none for whole-slice mode>
OBJECTIVE: <bounded outcome>
HUMAN_REVIEW_BOUNDARY: <packet or slice endpoint of the human-approved increment; observable or notable outcome and any preparatory-packet rationale>
PLAN_AUTHORIZATION: <human instruction authorizing this bounded plan>
AUTHORITY: <current controlling slice and packet sources, including any slice-delegated just-in-time preparation boundary>
ALLOWED_SCOPE: <paths or other explicit boundary>
NON_GOALS: <explicit exclusions>
PREREQUISITES: <dependencies and relevant starting state>
ACCEPTANCE: <conditions that establish completion>
VERIFICATION: <proportionate checks or expectations>
COMMIT_POSTURE: <commit | read-only | another explicit bounded instruction>
CONTEXT: <minimal live source, test, and type paths or sections; one useful precedent when available>
KNOWN_RISKS: <known conflicts, measured risks, or unresolved questions | none>
SKILL_ROUTE: <applicable skill names and the phase that activates each | none>
```

## Task instructions

Perform only the assigned unit. Treat the capsule, repository instructions, current authority, lifecycle, and review policy as the work boundary. The capsule curates context but does not override or expand authority. Do not infer additional scope from archived material or this prompt.

### Ground and admit

1. Read repository instructions, the assigned current authority, and the capsule's live context. Load additional live code, tests, or documentation only when the assigned unit requires them. Archived material is non-authority unless a current source explicitly adopts a bounded part of it.
2. Inspect the current branch, worktree, pre-existing changes, required dependencies, scope and authority sufficiency, and required capabilities. Preserve unrelated user work.
3. Confirm that the capsule is current, internally consistent, and sufficient to begin safely, including the human-approved plan and review boundary. Resolve ordinary context gaps from current authority and live sources when one safe answer exists. A missing packet-authority file is not an admission failure when the controlling slice names and bounds the packet, its predecessor is complete, and the slice explicitly delegates just-in-time preparation within that approved increment. Return `ADMISSION_FAILED` before product edits only when bounded recovery cannot establish a safe start. Do not manufacture a commit, review cycle, selector history, handoff, proof wall, ledger, or closeout artifact.

### Load context and skills when needed

- Treat `SKILL_ROUTE` as the minimal starting route. Load each selected skill only when its named phase begins; do not preload skill bodies or a complete lifecycle.
- If an unexpected phase or failure makes the route insufficient, consult `agent-using-agent-skills` once for this unit and add only the newly applicable skill. This does not widen `AUTHORITY`, `ALLOWED_SCOPE`, or `ACCEPTANCE`.
- Keep context bounded to this unit. Do not blanket-load archived plans, future packets, unrelated documents, or skills.

### Prepare delegated packet authority before product edits

When the assigned packet already has complete current authority, proceed from it. When the slice delegates just-in-time preparation instead, perform that bounded phase inside this same top-level task:

1. Use the current slice authority, live source and tests, and current architecture, seam, baseline, and program documents named in the capsule. Read historical planning only where the slice explicitly adopts a bounded input for this packet; historical status, authority claims, and ceremony remain inactive.
2. Resolve each material packet decision against those sources. If they yield one safe in-scope answer, record a compact packet authority or decision checkpoint when it will prevent drift. The slice-level delegation supplies scope; the checkpoint only records or refines the selected answer.
3. Before product edits, review newly selected material decisions through one focused causal round. Classify every valid finding under the review policy. Resolve blocking `P1`/`P2` findings within current authority; report `P3`/`P4` advisories without treating them as required work. A separate authority commit is allowed when it forms a meaningful implementation checkpoint, but is not required mechanically.
4. If permitted sources conflict or leave more than one consequential answer and current authority cannot select one, return `STOPPED` before product edits with that exact decision and required authority. Do not turn documentation absence by itself into a stop.

After successful preparation within the human-approved plan, continue directly into implementation in this task. Routine coding choices do not require another confirmation. A new consequential choice outside the approved plan returns to meta control before product edits. Do not create a separate top-level planning task or cross the human review boundary.

### Execute and verify

1. Complete the bounded change or read-only task within `ALLOWED_SCOPE` and `NON_GOALS`. Work directly or coordinate local collaboration subagents when delegation helps. Do not create, fork, monitor, or coordinate another user-visible top-level task; the meta task alone owns that layer. Remain responsible for all delegated work and the final task result.
2. Follow repository instructions. For this repository, that includes GitNexus impact analysis before editing an indexed function, class, or method; warning before a HIGH or CRITICAL impact edit; and change detection before committing.
3. Verify against `ACCEPTANCE`, `VERIFICATION`, and measured risk. Prefer focused checks when they are sufficient. Do not manufacture an omnibus proof wall or rerun unrelated workspace suites by default.
4. Remediate failed checks, blocking `P1`/`P2` findings, and other required friction when current authority supplies an in-scope path. Report `P3`/`P4` advisories, failures, omissions, and environmental limits honestly; do not turn advisories into required work.

In whole-slice mode, if a declared preparatory packet unexpectedly delivers an independently meaningful observable or notable outcome, do not begin the next packet. Complete the applicable verification, review, and required commit for that completed packet, then return `STOPPED` for the unfinished whole-slice assignment. Identify the completed packet and its established behavior/evidence, remaining slice work, and `NEXT: human review before any remainder planning or execution`. This is an early human checkpoint, not recoverable execution friction; the completed packet remains complete and no slice closeout is due.

### Review

Apply the review and closeout policy to the exact bounded subject.

- When review applies, use one focused reviewer and one controlling verdict for the current causal round. Do not launch a predetermined concurrent multi-lens burst.
- Require every valid finding to be labeled `P1 Critical`, `P2 Required`, `P3 Optional/Consider`, or `P4 Nit`. An unlabeled required fix is not a complete review disposition.
- Only `P1`/`P2` block completion and require remediation plus a later causal round on the materially changed subject. `P3`/`P4` are non-blocking and do not require remediation or re-review solely because they were reported.
- A clean, proceed, or approve verdict means no unresolved valid `P1`/`P2` and may include compactly reported `P3`/`P4`. Do not create an advisory inventory or transcription step.
- Material remediation may require one later causal round on the changed subject. Do not re-review an unchanged subject.
- Request specialist input only for a named, unresolved measured-risk question that the focused reviewer cannot adequately cover. An eligible risk category does not automatically add a reviewer.
- Do not create a review cycle for admission failure or ordinary continuation.

### Commit and return

- With `commit` posture and completed material work, verify and review the exact subject, confirm the effective Git author and committer are legitimate, run required pre-commit change detection, and create one coherent local commit unless current authority explicitly requires another bounded structure. Do not create a mechanical second commit.
- With `read-only` posture or no material change, do not manufacture a commit.
- In packet mode, return `PACKET_COMPLETE` only after the packet acceptance boundary is satisfied. Describe the delivered behavior, a concrete example, meaningful refusal or failure, and anything unfinished in the compact result so the meta task can produce the human recap. Identify an observable or notable outcome even when this packet was planned as preparatory. Never claim `SLICE_COMPLETE`, choose the next packet, or perform whole-slice closeout; the meta task applies the human review boundary before continuation.
- In whole-slice mode, return `SLICE_COMPLETE` only after every authorized outcome condition is satisfied and the minimal slice closeout has been performed once. Apply the early human checkpoint above instead when an earlier meaningful packet completes unexpectedly.
- Before returning `STOPPED` for ordinary execution friction, exhaust safe bounded recovery that current authority identifies. The early whole-slice human checkpoint must return without advancing or attempting recovery past that boundary. Do not wait for a human decision inside the task session: return control to the meta task with the exact condition, attempts when relevant, truthful partial state, and safe-resumption need. Do not call the slice complete.

## Final result

End the task session with exactly this compact, human-readable block. Replace placeholders without adding control-plane identifiers or durable result files.

```text
STATUS: <PACKET_COMPLETE | SLICE_COMPLETE | STOPPED | ADMISSION_FAILED>
UNIT: <slice identifier; mode; packet identifier when applicable>
OUTCOME: <plain-English behavior, concrete example, meaningful refusal or failure, and unfinished or excluded work>
CHANGED: <changed paths | none>
VERIFICATION: <checks and outcomes, including failures or limits>
REVIEW: <controlling verdict and compact P1–P4 disposition | not applicable — reason>
COMMIT: <commit hash | none>
NEXT: <return to meta-task decision | exact stop or admission condition and resumption need | none>
```

For `PACKET_COMPLETE`, `NEXT` returns control to the meta task, identifying human review when this packet meets the selected boundary or delivers an earlier observable or notable outcome. For `STOPPED` or `ADMISSION_FAILED`, it states the exact condition and what is needed for meta adjudication or safe resumption. An early whole-slice human checkpoint requires recap-and-stop, not generic recovery; other friction follows ordinary meta adjudication. For `SLICE_COMPLETE`, it is `none` within the task; the meta task still owes the human recap and stop. Do not use `STOPPED` for a successfully completed assigned unit. In the early whole-slice return, only the unfinished slice assignment is stopped; describe the completed packet as complete.

Do not add dispatch IDs, selectors, subject fingerprints, evidence manifests, review counters, handoff IDs, ledger paths, command recipes, advisory inventories, the context capsule, or a skill-routing transcript.
