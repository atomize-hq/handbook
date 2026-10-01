# Review and Closeout Policy

This document is the normative quality policy for work governed by the [`Slice and Packet Work-Unit Lifecycle`](work-unit-lifecycle.md). Humans, the authority-preparation, task-session, and meta-task prompts, and the thin routing skill consume it to verify bounded work, conduct review when applicable, and close a slice or genuine workflow stop without recreating control-plane ceremony.

It defines required behavior and information, not a prompt, receipt format, file layout, schema, validator, state store, or skill.

Repository-level quality requirements remain in force where they apply.

## Terms

- A **review subject** is the exact bounded change and evidence currently being evaluated. Its scope must be clear enough for the reviewer to return a decisive verdict.
- A **causal review round** evaluates one current review subject through one focused reviewer and produces one controlling verdict. Exceptional specialist input may be added only under the rule below.
- **Material remediation** changes implementation, authority-sensitive documentation, acceptance-relevant behavior, or evidence enough that the prior verdict no longer covers the subject.

## Proportionate verification

Verification must match the bounded subject, its acceptance conditions, and measured risk.

- Run the checks needed to support the claimed result, including repository-mandated checks.
- Identify the applicable commands or checks and report their outcomes honestly, including failures, limits, or checks not run.
- Prefer focused checks when they establish the relevant behavior. Do not require an omnibus proof wall or full-workspace replay when the subject and risk do not call for one.
- Treat evidence as invalidated when later changes can affect what it established. Rerun that evidence before relying on it.

Verification evidence may be referenced or summarized. This policy does not require a dedicated evidence file or serialization format.

## Causal review

When review applies, begin a round only after a bounded subject is ready for evaluation.

Review applies when acceptance conditions or repository policy require it, or when a measured risk calls for independent evaluation. Do not create a review round merely because a task or packet started, a delegation occurred, or work advanced to another phase.

The default round uses exactly one focused reviewer. The reviewer evaluates the subject and its evidence, classifies every valid finding under the priority rubric below, and returns one verdict. Do not launch a predetermined multi-lens burst or derive reviewer count from phase, packet count, task size, or the start of a task session.

### Finding priority and disposition

Use the highest priority demonstrated against the selected review subject and its authorized outcome:

| Priority | Review label | Meaning | Required disposition |
|---|---|---|---|
| `P1` | Critical | Completion would permit demonstrated data loss, a security-boundary break, authority corruption, unsafe mutation, or materially false success. | Blocking. Remediate and verify within current authority, then conduct one fresh causal round on the changed subject; otherwise use a genuine workflow stop. |
| `P2` | Required | The subject violates its selected contract or acceptance boundary, has reachable incorrect behavior or an authority bypass, lacks required fail-closed behavior, or lacks evidence required to support completion. | Blocking. Remediate and verify within current authority, then conduct one fresh causal round on the changed subject; otherwise use a genuine workflow stop. |
| `P3` | Optional / Consider | A defense-in-depth, maintainability, robustness, or additional-coverage improvement is plausible, but no selected contract or acceptance violation is demonstrated. | Non-blocking. Report it compactly; remediation and re-review are not required solely for this advisory. |
| `P4` | Nit | Naming, phrasing, formatting, local consistency, or another observation has no correctness or authority effect. | Non-blocking. Report it compactly; remediation and re-review are not required solely for this advisory. |

Priority follows the integrated selected outcome, not the severity of an isolated component observation. Do not classify an improvement as `P2` merely because it would be useful. When evidence cannot distinguish `P2` from `P3`, state the missing evidence and resolve that uncertainty before completion.

A verdict requiring remediation contains at least one valid `P1` or `P2`. A clean, proceed, or approve verdict means no unresolved valid `P1` or `P2`; it may include `P3` or `P4` advisories. `P3` and `P4` alone do not block completion or trigger another causal round. If optional advisory work is nevertheless performed and materially changes the subject, the ordinary changed-subject review rule still applies.

Keep advisory disposition in the compact review result and conversational packet or slice summary. This policy does not create an advisory inventory, backlog file, transcription step, or other durable orchestration state.

If blocking `P1` or `P2` remediation, or another voluntary material change, updates the subject and review remains applicable, conduct one subsequent causal review round with one focused reviewer. Do not re-review an unchanged subject. Ordinary continuation, delegation, verification, packet sequencing, or an unfixed `P3` or `P4` advisory does not create another round.

A blocking `P1` or `P2` finding directly caused or unmasked by the immediately preceding remediation may justify one next causal round only while scope, authority, and measured risk remain unchanged. That later round is delta-focused; it must not reopen general discovery. A `P3` or `P4` advisory does not justify another round. An unrelated finding or one requiring authority, scope, or risk expansion follows the lifecycle's genuine-stop rules; it must not silently widen into a review cascade.

Do not add discovery, closure, or supplemental review phases, fixed round budgets, counters, selectors, or lineage records around this causal sequence.

### Exceptional specialist input

A second reviewer within the same round is exceptional. It is allowed only when a named, measured risk leaves a distinct question that the first reviewer cannot adequately cover. Eligible risk areas include:

- public API behavior;
- compatibility or migration;
- security or unsafe behavior;
- ownership conflicts;
- high-impact cross-crate changes; or
- genuine disagreement between reviewer judgment and available evidence.

Eligibility does not mandate escalation. State the distinct unresolved question before requesting the specialist input. The second reviewer addresses only that question; the round still yields one controlling verdict and does not become a concurrent review burst. No dispatch or result archive is required.

An admission failure before product edits does not create a review cycle. Return the bounded result required by the lifecycle policy.

## Packet boundary

Packet-level verification and review remain bounded to the assigned packet. They establish whether that packet satisfies its acceptance boundary; they do not evaluate or close the entire slice.

When a packet completes, return control and its established evidence to the meta task. Do not perform slice closeout or manufacture durable review and closeout artifacts at the packet boundary.

## Human review recap

At the lifecycle's selected packet or slice human review boundary, the meta task returns to the persistent control conversation, presents a concise recap, and stops before successor planning or dispatch. An earlier observable or notable packet outcome also triggers this boundary. Technical review and a clean verdict do not replace the human's review of the delivered increment.

The recap answers:

- What changed, in ordinary English?
- What concrete example demonstrates it?
- What is correctly refused or prevented?
- What verification and independent review support it, and what are their limits?
- What remains unfinished or outside this increment?
- What should come next, and why, as a recommendation only?

State whether the changes are implemented, committed, or merged, with relevant commits when present. These are different states. Keep the recap short enough to discuss without reading the dense reference documents. Use existing plans, evidence, and conversational results; no recap file or additional technical review round is required merely for the human checkpoint.

## Whole-slice closeout

At slice completion, check the complete authorized outcome and cross-packet integration once. Reuse valid packet evidence rather than replaying each packet's ceremony. Rerun only checks needed for aggregate or integration confidence, or evidence invalidated by later work.

Record or emit the minimal closeout once. It contains:

- outcome and status;
- applicable verification evidence and outcomes;
- the controlling review verdict, or an explicit not-applicable result when review did not apply;
- relevant commit or commits; and
- either confirmation of completion or, for a genuine workflow stop, the condition and information needed for safe resumption.

These are information requirements, not a required Markdown, chat, file, or machine-readable shape.

At a genuine workflow stop, include only what is needed to resume safely and do not call the slice complete. The lifecycle policy determines whether a condition is a genuine stop.

## Forbidden ceremony

Neither packet work, slice completion, nor a genuine workflow stop requires a ledger, selector, proof wall, handoff schema, advisory transcription, reviewer dispatch or result archive, or mandatory mechanical second commit.

Later prompts, skills, or validators may make this policy easier to apply, but they may not create additional review or closeout boundaries or turn optional evidence presentation into mandatory control-plane state.
