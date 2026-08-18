# Handbook Contract Membrane

This is the authority and navigation entry point for the Handbook contract-membrane program after the control-plane reset. It identifies current sources; it does not authorize an implementation slice or define an orchestration protocol.

## Current status

Commit `8c93d6f3eacf3312c935a6f83ff62244450dbf1b` removed the former control-plane documents and moved the former repository-local orchestration skill into the disabled historical area while preserving the implemented product and 37 product-consumed fixtures. A thin active router now operationalizes bounded authority preparation and execution without supplying product scope or unresolved decisions.

At this checkpoint:

- the Rust product implementation remains in the workspace;
- retained executable fixtures remain under [`slices/`](slices/), alongside the
  completed [`HCM-4.2 outcome authority`](slices/HCM-4.2/README.md), its Packet
  1–5 checkpoints, and the completed Packet-1–8 outcome and aggregate closeout
  for the [`HCM-4.3 slice authority`](slices/HCM-4.3/README.md);
- Phase 3 is exited under its selected reduced boundary; its explicitly rehomed
  work remains with later owners;
- the [`phase-and-slice map`](phase-and-slice-map.md) supplies current roadmap
  status, order, hard predecessors, and exit state; HCM-4.2 is complete, and
  HCM-4.3 has completed its sequential packet implementation, aggregate
  review, and slice closeout; the bounded
  [`HCM-4.4 slice authority`](slices/HCM-4.4/README.md) has completed Packet
  1–6 implementation and packet proof, while aggregate verification, review,
  and slice closeout remain pending;
- the former orchestration skill under `.codex/skills/.disabled/` is inactive and is not authority for new work; and
- the active thin router is routing guidance only and is not authority for product or slice work.

Do not infer current program completion or implementation authority from pre-reset plans or the roadmap. The phase-and-slice map supplies navigation and program status only; it does not prove completion or authorize implementation.

## Authority and navigation

Use the source that owns the question:

| Question | Current source |
| --- | --- |
| What does the product implement? | The tracked Rust workspace under [`crates/`](../../../crates/), including crate-local definitions, plus tracked declarative product assets under [`core/`](../../../core/). Source at the current commit is product truth. |
| What is implemented, precursor-only, absent/future, or external-unknown now? | [`implemented-baseline.md`](implemented-baseline.md) owns the compact current product-state classification. It is an implementation checkpoint, not slice authority, a schedule, or phase-exit evidence. |
| What stable architecture and ownership direction must current and future membrane work preserve? | [`architecture-context.md`](architecture-context.md) owns semantic ownership, dependency direction, adapter, trust, and external-proof boundaries. It does not authorize work or report program status. |
| How do current producer and consumer seams route toward bounded migration owners? | [`seam-crosswalk.md`](seam-crosswalk.md) maps seams and migration ownership. It does not authorize work or establish schedule, status, or completion. |
| What is the program roadmap status, order, and phase-exit state? | [`phase-and-slice-map.md`](phase-and-slice-map.md) owns roadmap status, ordering, hard predecessors, and exits. It does not authorize slice or packet execution. |
| What behavior is executable and checked? | Crate and repository tests, together with the 37 retained files under [`slices/HCM-2.2`](slices/HCM-2.2/), [`slices/HCM-2.3`](slices/HCM-2.3/), and [`slices/HCM-2.4`](slices/HCM-2.4/). These fixtures are product inputs and test evidence, not control-plane or work authorization. |
| What is the current program-level authority boundary? | This file. It owns only post-reset authority classification and navigation. |
| What are the execution units and their lifecycle boundaries? | [`work-unit-lifecycle.md`](work-unit-lifecycle.md) defines slice and packet identity, ownership, completion, and genuine workflow stops. |
| What verification, review, and closeout apply at those boundaries? | [`review-and-closeout-policy.md`](review-and-closeout-policy.md) defines proportionate evidence, causal review rounds, escalation, and minimal closeout information. |
| How is missing phase or slice authority prepared at the correct level? | [`authority-preparation-prompt.md`](authority-preparation-prompt.md) defines separate phase-to-slice and slice-to-packet top-level planning tasks. Each returns to the meta task before the next level. |
| How is one authorized slice or packet assigned to a top-level task session? | [`task-session-prompt.md`](task-session-prompt.md) provides the copy/paste-ready assignment prompt and compact result presentation. |
| How does a meta task prepare and continue requested HCM work? | [`meta-task-prompt.md`](meta-task-prompt.md) routes approved phase preparation, requested-slice preparation, whole-slice or sequential packet execution, and one aggregate closeout. |
| Where is the active routing entry point? | [`.codex/skills/orchestrate-top-level-tasks/SKILL.md`](../../../.codex/skills/orchestrate-top-level-tasks/SKILL.md) operationalizes role-specific preparation and execution. User instructions and current authority supply permission and scope; the skill does not invent either. |
| What is the completed HCM-4.2 outcome and evidence boundary? | [`slices/HCM-4.2/README.md`](slices/HCM-4.2/README.md) records the completed Packet 0–5 outcome, its single aggregate closeout, and links to the Packet 1–5 decision checkpoints. |
| What is the completed HCM-4.3 outcome and packet boundary? | [`slices/HCM-4.3/README.md`](slices/HCM-4.3/README.md) records complete CLI JSON parity, the machine/human/diagnostic/exit contract, compatibility posture, ordered Packets 1–8, and the aggregate slice closeout. |
| What is the current HCM-4.4 outcome and packet boundary? | [`slices/HCM-4.4/README.md`](slices/HCM-4.4/README.md) defines the bounded non-UI Tauri command facade, SDK DTO and dependency boundary, strict Packets 1–6, and aggregate acceptance. Packets 1–6 implementation and packet proof are complete; aggregate verification, review, and slice closeout remain pending. |
| Where does future slice-specific intent belong? | In a clearly identified current document under `slices/<slice-id>/`, separate from retained fixtures. A roadmap entry plus an explicit execution request may authorize bounded preparation of that document, but product execution begins only after the resulting current slice authority is ready. |
| Where is architecture rationale? | [`docs/ideas/handbook-contract-membrane-architecture-memo.md`](../../ideas/handbook-contract-membrane-architecture-memo.md) and related idea documents provide design lineage only. They do not authorize implementation or establish current completion. |

If prose about implemented behavior conflicts with the current source and tests, treat the discrepancy as documentation drift; do not use the prose to override the product.

## Historical material

The Git ref `archive/handbook-contract-membrane-pre-clean-20260812` preserves the former membrane control plane. Content under [`archived/`](../../../archived/) also records prior designs, plans, and evidence. Both are historical provenance and non-current authority unless a future, explicit current document adopts a bounded part of them.

### Pre-clean recovery checkpoint

The exact pre-clean checkpoint is commit `8605d6539355f350c55b921d95fb450bca30ee18`. It is preserved through these recovery anchors:

- local branch `feat/handbook-contract-membrane` and remote branch `origin/feat/handbook-contract-membrane`;
- annotated tag `archive/handbook-contract-membrane-pre-clean-20260812`; and
- the complete-history, machine-local Git bundle at `C:\Users\spmcc\Documents\__Project_Code\handbook-contract-membrane-8605d65.bundle`.

No durable old-branch worktree is required for recovery. Any detached Codex-managed worktree at this commit is incidental and may disappear; use the branches, tag, or bundle instead.

Use the checkpoint as historical reference when current work needs to inspect former documents or specifications for intended product behavior, architectural preferences, design lineage, or other historical detail. Do not copy a former document set or control-plane package back into the active branch wholesale. Extract only the bounded material needed for the current decision, verify it against current source and tests, and record the resulting decision in an explicit current authority document. Historical status, orchestration ceremony, and authority claims remain inactive unless a current document separately and explicitly adopts a bounded part.

The repository root README, [`docs/START_HERE.md`](../../START_HERE.md), and [`docs/README.md`](../../README.md) route current contract-membrane readers to this entry point.
