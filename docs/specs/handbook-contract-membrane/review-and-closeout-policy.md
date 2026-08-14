# Review and Closeout Policy

This document is the normative quality policy for work governed by the [`Slice and Packet Work-Unit Lifecycle`](work-unit-lifecycle.md). Humans, future task and meta-task prompts, and a future thin routing skill consume it to verify bounded work, conduct review when applicable, and close a slice or genuine workflow stop without recreating control-plane ceremony.

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

The default round uses exactly one focused reviewer. The reviewer evaluates the subject and its evidence, states actionable findings with their significance, and returns one verdict. Do not launch a predetermined multi-lens burst or derive reviewer count from phase, packet count, task size, or the start of a task session.

If material remediation updates the subject and review remains applicable, conduct one subsequent causal review round with one focused reviewer. Do not re-review an unchanged subject. Ordinary continuation, delegation, verification, or packet sequencing does not create another round.

A finding directly caused or unmasked by remediation may justify a later causal round after the subject changes again. A finding that requires unrelated authority or scope expansion follows the lifecycle's genuine-stop rules; it must not silently widen into a review cascade.

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
