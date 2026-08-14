# Handbook Contract Membrane

This is the authority and navigation entry point for the Handbook contract-membrane program after the control-plane reset. It identifies current sources; it does not authorize an implementation slice or define an orchestration protocol.

## Current status

Commit `8c93d6f3eacf3312c935a6f83ff62244450dbf1b` removed the former control-plane documents and disabled the repository-local top-level orchestration skill while preserving the implemented product and 37 product-consumed fixtures.

At this checkpoint:

- the Rust product implementation remains in the workspace;
- only the retained executable fixtures remain under [`slices/`](slices/);
- no live slice-specific intent, remaining-slice schedule, or continuation authority is present in this directory; and
- the orchestration skill under `.codex/skills/.disabled/` is inactive and is not authority for new work.

Do not infer current program completion or future scope from pre-reset plans. No post-reset source currently establishes a remaining-slice status beyond the facts above.

## Authority and navigation

Use the source that owns the question:

| Question | Current source |
| --- | --- |
| What does the product implement? | The tracked Rust workspace under [`crates/`](../../../crates/), including crate-local definitions, plus tracked declarative product assets under [`core/`](../../../core/). Source at the current commit is product truth. |
| What behavior is executable and checked? | Crate and repository tests, together with the 37 retained files under [`slices/HCM-2.2`](slices/HCM-2.2/), [`slices/HCM-2.3`](slices/HCM-2.3/), and [`slices/HCM-2.4`](slices/HCM-2.4/). These fixtures are product inputs and test evidence, not control-plane or work authorization. |
| What is the current program-level authority boundary? | This file. It owns only post-reset authority classification and navigation. |
| What are the execution units and their lifecycle boundaries? | [`work-unit-lifecycle.md`](work-unit-lifecycle.md) defines slice and packet identity, ownership, completion, and genuine workflow stops. |
| What verification, review, and closeout apply at those boundaries? | [`review-and-closeout-policy.md`](review-and-closeout-policy.md) defines proportionate evidence, causal review rounds, escalation, and minimal closeout information. |
| Where does future slice-specific intent belong? | In a newly authorized, clearly identified slice-specific document under `slices/<slice-id>/`, separate from retained fixtures. No such document is created or implied here. |
| Where is architecture rationale? | [`docs/ideas/handbook-contract-membrane-architecture-memo.md`](../../ideas/handbook-contract-membrane-architecture-memo.md) and related idea documents provide design lineage only. They do not authorize implementation or establish current completion. |

If prose about implemented behavior conflicts with the current source and tests, treat the discrepancy as documentation drift; do not use the prose to override the product.

## Historical material

The Git ref `archive/handbook-contract-membrane-pre-clean-20260812` preserves the former membrane control plane. Content under [`archived/`](../../../archived/) also records prior designs, plans, and evidence. Both are historical provenance and non-current authority unless a future, explicit current document adopts a bounded part of them.

The repository root README, [`docs/START_HERE.md`](../../START_HERE.md), and [`docs/README.md`](../../README.md) still contain pre-reset links or control-pack status language. Until those indexes are repaired, do not treat those statements as the current membrane control-plane status; use this entry point instead.
