# Task-Session Prompt

Use this prompt to assign one authorized whole slice or one packet to a top-level task session. It operationalizes the [`Slice and Packet Work-Unit Lifecycle`](work-unit-lifecycle.md) and [`Review and Closeout Policy`](review-and-closeout-policy.md); those documents remain authoritative if this prompt is incomplete or ambiguous.

Copy the assignment block and task instructions into the task session, replacing each angle-bracketed placeholder. Keep entries short and human-readable.

## Assignment

```text
MODE: <whole-slice | packet>
SLICE: <slice identifier>
PACKET: <packet identifier | none for whole-slice mode>
OBJECTIVE: <bounded outcome>
AUTHORITY: <current controlling sources>
ALLOWED_SCOPE: <paths or other explicit boundary>
NON_GOALS: <explicit exclusions>
PREREQUISITES: <dependencies and relevant starting state>
ACCEPTANCE: <conditions that establish completion>
VERIFICATION: <proportionate checks or expectations>
COMMIT_POSTURE: <commit | read-only | another explicit bounded instruction>
```

## Task instructions

Perform only the assigned unit. Treat the assignment, repository instructions, current authority, lifecycle, and review policy as the work boundary. Do not infer additional scope from archived material or this prompt.

### Ground and admit

1. Read repository instructions, the assigned current authority, and only the live code, tests, and documentation needed for this unit. Archived material is non-authority unless a current source explicitly adopts a bounded part of it.
2. Inspect the current branch, worktree, pre-existing changes, required dependencies, scope and authority sufficiency, and required capabilities. Preserve unrelated user work.
3. Begin when safe scoped work is possible. If it is not possible before product edits, return `ADMISSION_FAILED` immediately using the result block below. Do not edit product files or manufacture a commit, review cycle, selector history, handoff, proof wall, ledger, or closeout artifact.

### Load context and skills when needed

- Keep context bounded to this unit. Do not blanket-load archived plans, unrelated documents, or skills.
- Discover and load an applicable skill only when its phase begins: debugging after an observed failure, Git workflow for commit work, review workflow when review applies, and other skills only when the assigned work genuinely triggers them.

### Execute and verify

1. Complete the bounded change or read-only task within `ALLOWED_SCOPE` and `NON_GOALS`.
2. Follow repository instructions. For this repository, that includes GitNexus impact analysis before editing an indexed function, class, or method; warning before a HIGH or CRITICAL impact edit; and change detection before committing.
3. Verify against `ACCEPTANCE`, `VERIFICATION`, and measured risk. Prefer focused checks when they are sufficient. Do not manufacture an omnibus proof wall or rerun unrelated workspace suites by default.
4. Report check failures, omissions, and environmental limits honestly.

### Review

Apply the review and closeout policy to the exact bounded subject.

- When review applies, use one focused reviewer and one controlling verdict for the current causal round. Do not launch a predetermined concurrent multi-lens burst.
- Material remediation may require one later causal round on the changed subject. Do not re-review an unchanged subject.
- Request specialist input only for a named, unresolved measured-risk question that the focused reviewer cannot adequately cover. An eligible risk category does not automatically add a reviewer.
- Do not create a review cycle for admission failure or ordinary continuation.

### Commit and return

- With `commit` posture and completed material work, verify and review the exact subject, confirm the effective Git author and committer are legitimate, run required pre-commit change detection, and create one coherent local commit unless current authority explicitly requires another bounded structure. Do not create a mechanical second commit.
- With `read-only` posture or no material change, do not manufacture a commit.
- In packet mode, return `PACKET_COMPLETE` only after the packet acceptance boundary is satisfied. Never claim `SLICE_COMPLETE`, choose the next packet, or perform whole-slice closeout; the meta task owns continuation.
- In whole-slice mode, return `SLICE_COMPLETE` only after every authorized outcome condition is satisfied and the minimal slice closeout has been performed once.
- If a genuine workflow-stop condition prevents safe continuation, return `STOPPED` with the exact condition and safe-resumption need. Do not call the slice complete.

## Final result

End the task session with exactly this compact, human-readable block. Replace placeholders without adding control-plane identifiers or durable result files.

```text
STATUS: <PACKET_COMPLETE | SLICE_COMPLETE | STOPPED | ADMISSION_FAILED>
UNIT: <slice identifier; mode; packet identifier when applicable>
OUTCOME: <concise result summary>
CHANGED: <changed paths | none>
VERIFICATION: <checks and outcomes, including failures or limits>
REVIEW: <controlling verdict | not applicable — reason>
COMMIT: <commit hash | none>
NEXT: <return to meta-task decision | exact stop or admission condition and resumption need | none>
```

For `PACKET_COMPLETE`, `NEXT` returns control to the meta task. For `STOPPED` or `ADMISSION_FAILED`, it states the exact condition and what is needed to resume safely. For `SLICE_COMPLETE`, it is `none`.

Do not add dispatch IDs, selectors, subject fingerprints, evidence manifests, review counters, handoff IDs, ledger paths, command recipes, or advisory inventories.
