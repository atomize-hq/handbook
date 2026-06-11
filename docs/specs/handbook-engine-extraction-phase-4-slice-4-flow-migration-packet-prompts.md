# Handbook Engine Extraction Phase 4 Slice 4 Packet Prompts

Task source: [handbook-engine-extraction-phase-4-slice-4-flow-migration-tasks.md](./handbook-engine-extraction-phase-4-slice-4-flow-migration-tasks.md)
Spec source: [handbook-engine-extraction-phase-4-slice-4-flow-migration-spec.md](./handbook-engine-extraction-phase-4-slice-4-flow-migration-spec.md)
Plan source: [handbook-engine-extraction-phase-4-slice-4-flow-migration-plan.md](./handbook-engine-extraction-phase-4-slice-4-flow-migration-plan.md)

These prompts are ready to paste into fresh orchestration sessions. Each one starts in `/goal`, requires fresh `GPT-5.4` `high` subagents, uses `$incremental-implementation` for implementation/fix rounds, uses `$code-review-and-quality` for review rounds, preserves commit boundaries between implementation, review, and fix work, and keeps packet execution bounded to the approved Slice 4.4 scope.

## Packet 4.4.1 Prompt

```text
/goal Orchestrate Phase 4 Slice 4 Packet 4.4.1: Resolver Packet-Result And Budget Migration in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 4.4.1 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-4-slice-4-flow-migration-tasks.md.
- Verify live repo truth before changing anything, including that Slice 4.3 already made `handbook-pipeline` the real owner of the approved pipeline runtime families and that `handbook-flow` is still only a scaffold wrapper over `handbook-compiler`.
- Use the slice spec and plan at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-4-slice-4-flow-migration-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-4-slice-4-flow-migration-plan.md
- Stay inside Packet 4.4.1 only.

Hard rules:
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them.
- Every fix subagent prompt must begin with `/goal ` and must explicitly use $incremental-implementation.
- Before editing any function, method, or other production symbol, run GitNexus impact analysis on the touched symbol(s) and report the blast radius. If GitNexus reports a stale index, refresh it first.
- Before every commit, run GitNexus detect-changes and verify the affected scope matches Packet 4.4.1 only.
- Commit after the implementation round before sending the change to review.
- Commit again after each accepted fix round that changes files, before re-reviewing.
- Do not amend away prior packet-round commits during the loop.
- Stay inside Packet 4.4.1 scope.

Packet 4.4.1 scope:
- Flip the flow/compiler dependency posture so `handbook-flow` can become the real implementation owner for the approved Slice 4.4 seam.
- Move budget policy, targets, outcomes, and evaluation behavior into `handbook-flow`.
- Move packet-result models, notes, sections, packet decision summaries, and related packet-result types into `handbook-flow`.
- Move typed packet resolution, packet selection, refusal/blocker assembly, and decision-log-backed result construction into `handbook-flow`.
- Update typed downstream consumers so renderer and refusal-mapping paths consume flow-owned types without reassigning renderer ownership.
- Move or recreate regression coverage so the migrated flow seam is tested from `handbook-flow` itself.
- Expected files:
  - crates/flow/Cargo.toml
  - crates/compiler/Cargo.toml
  - crates/flow/src/lib.rs
  - crates/flow/src/budget.rs
  - crates/flow/src/packet_result.rs
  - crates/flow/src/resolver.rs
  - crates/compiler/src/lib.rs
  - crates/compiler/src/budget.rs
  - crates/compiler/src/packet_result.rs
  - crates/compiler/src/resolver.rs
  - crates/compiler/src/rendering/shared.rs
  - crates/compiler/src/rendering/model.rs
  - crates/compiler/src/rendering/inspect.rs
  - crates/compiler/src/rendering/markdown.rs
  - crates/compiler/src/rendering/json.rs
  - crates/flow/tests/**
  - optionally crates/compiler/tests/resolver_core.rs
  - optionally crates/compiler/tests/refusal_mapping.rs
  - optionally crates/compiler/tests/rendering_surface.rs
  - optionally crates/cli/tests/cli_surface.rs

Out of scope:
- moving `rendering`, `refusal`, `error`, or `doctor` into `handbook-flow`
- broad CLI import rewires or command/help wording changes
- retiring or broadly narrowing `handbook-compiler` beyond a temporary compatibility facade
- introducing a new shared crate unless live dependency truth proves this packet impossible otherwise
- changing C-04 blocker semantics or C-05 renderer semantics
- Phase 4.5 caller rewires or compiler narrowing

Implementation subagent prompt requirements:
- Begin with `/goal Land Phase 4 Slice 4 Packet 4.4.1: Resolver Packet-Result And Budget Migration`.
- Tell the subagent to use $incremental-implementation.
- Require live verification of current ownership, dependency, and consumer truth across:
  - `crates/flow/Cargo.toml`
  - `crates/flow/src/lib.rs`
  - `crates/compiler/Cargo.toml`
  - `crates/compiler/src/lib.rs`
  - `crates/compiler/src/budget.rs`
  - `crates/compiler/src/packet_result.rs`
  - `crates/compiler/src/resolver.rs`
  - `crates/compiler/src/rendering/shared.rs`
  - `crates/compiler/src/rendering/model.rs`
  - `crates/compiler/src/rendering/inspect.rs`
  - `crates/compiler/src/rendering/markdown.rs`
  - `crates/compiler/src/rendering/json.rs`
  - `crates/compiler/tests/resolver_core.rs`
  - `crates/compiler/tests/refusal_mapping.rs`
  - `crates/compiler/tests/rendering_surface.rs`
  - `crates/cli/tests/cli_surface.rs`
- Require the implementation to:
  - eliminate the scaffold-only `handbook-flow -> handbook-compiler` ownership posture for the Packet 4.4.1 seam
  - keep the resulting dependency graph acyclic
  - move `budget`, `packet_result`, and `resolver` behind `handbook-flow` as the one real implementation owner
  - keep any compiler support for those surfaces as thin compatibility re-exports or adapters only
  - update typed renderer/refusal-mapping consumers to read flow-owned types without broad renderer ownership changes
  - add or migrate flow-owned tests so `handbook-flow` proves the moved behavior directly
- Require the implementation to preserve budget semantics, packet-result semantics, refusal/blocker meaning, and renderer-facing typed behavior unless live repo truth proves a narrow compatibility adjustment is required.
- Require the implementation to avoid pulling direct caller rewires, compiler-retirement decisions, or broad renderer/refusal/error reclassification into this packet.
- Require GitNexus impact analysis before editing touched production symbols and require the subagent to report impacted callers/processes.
- Require targeted verification with:
  - `cargo tree -p handbook-flow -e normal`
  - `cargo tree -p handbook-compiler -e normal`
  - `cargo check --workspace`
  - `rg -n 'pub fn resolve|struct ResolveRequest|struct ResolverResult|struct PacketResult|enum PacketSelectionStatus|struct BudgetOutcome|enum BudgetDisposition' crates/flow crates/compiler/src`
  - `rg -n 'PacketResult|BudgetOutcome|BudgetDisposition|PacketSelectionStatus|ResolveRequest|ResolverResult' crates/compiler/src/rendering crates/compiler/tests crates/cli/tests`
  - `cargo test -p handbook-flow`
  - `cargo test -p handbook-compiler --test resolver_core`
  - `cargo test -p handbook-compiler --test refusal_mapping`
  - `cargo test -p handbook-compiler --test rendering_surface`
  - `cargo test -p handbook-cli --test cli_surface`
- Require the subagent to stop after Packet 4.4.1 acceptance is met and report touched files, impact-analysis results, dependency posture, compatibility posture, verification run, and residual risks.

Review subagent prompt requirements:
- Begin with `/goal Review Phase 4 Slice 4 Packet 4.4.1: Resolver Packet-Result And Budget Migration`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to review the packet against the Slice 4.4 spec, plan, tasks, and verification evidence.
- Require special attention to:
  - whether `handbook-flow` is now the real owner of `budget`, `packet_result`, and `resolver`
  - whether `handbook-compiler` kept only thin compatibility layers instead of duplicate implementation bodies
  - whether typed renderer and refusal-mapping consumers still align with the C-04 and C-05 contract surfaces
  - whether the dependency graph is acyclic and materially better than the scaffold posture
  - whether flow-owned tests really cover the migrated seam
  - whether any direct caller rewires, compiler-retirement work, or broad renderer/refusal/error ownership work leaked into Packet 4.4.1
- Require severity labels and explicit callouts if the packet leaves split-brain ownership, introduces cycles, weakens typed result guarantees, or hides migration gaps behind oversized compiler facades.

Fix loop:
- If the review is clean, stop and report Packet 4.4.1 complete.
- If the review finds issues, spawn one fresh GPT-5.4 high fix subagent per review round using `$incremental-implementation`.
- The fix prompt must cite the exact review findings and require only the minimal Packet-4.4.1-bounded changes needed to close them.
- Re-run GitNexus impact analysis if new production symbols are touched.
- Re-run the bounded verification after each fix round.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after implementation if Packet 4.4.1 lands cleanly, before review.
- Before each commit, run GitNexus detect-changes and confirm the affected scope matches Packet 4.4.1 only.
- Commit after each accepted fix round, before re-review.
- Commit messages must describe the Packet 4.4.1 change clearly and standalone.

Stop conditions:
- Stop once Packet 4.4.1 is review-clean and committed.
- Stop and report blocked if Packet 4.4.1 cannot be completed without widening into direct caller rewires, compiler narrowing/retirement, broad renderer/refusal/error ownership redesign, or later Phase 4 slices.
```
