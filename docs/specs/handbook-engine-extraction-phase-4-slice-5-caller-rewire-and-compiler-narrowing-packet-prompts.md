# Handbook Engine Extraction Phase 4 Slice 5 Packet Prompts

Task source: [handbook-engine-extraction-phase-4-slice-5-caller-rewire-and-compiler-narrowing-tasks.md](./handbook-engine-extraction-phase-4-slice-5-caller-rewire-and-compiler-narrowing-tasks.md)
Spec source: [handbook-engine-extraction-phase-4-slice-5-caller-rewire-and-compiler-narrowing-spec.md](./handbook-engine-extraction-phase-4-slice-5-caller-rewire-and-compiler-narrowing-spec.md)
Plan source: [handbook-engine-extraction-phase-4-slice-5-caller-rewire-and-compiler-narrowing-plan.md](./handbook-engine-extraction-phase-4-slice-5-caller-rewire-and-compiler-narrowing-plan.md)

These prompts are ready to paste into fresh orchestration sessions. Each one starts in `/goal`, requires fresh `GPT-5.4` `high` subagents, uses `$incremental-implementation` for implementation/fix rounds, uses `$code-review-and-quality` for review rounds, preserves commit boundaries between implementation, review, and fix work, and keeps packet execution bounded to the approved Slice 4.5 scope.

## Packet 4.5.1 Prompt

```text
/goal Orchestrate Phase 4 Slice 5 Packet 4.5.1: Direct Caller Rewires To New Crates in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 4.5.1 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-4-slice-5-caller-rewire-and-compiler-narrowing-tasks.md.
- Verify live repo truth before changing anything, including that Slices 4.2, 4.3, and 4.4 already established real owner crates for the approved engine-safe, pipeline-safe, and flow-safe surfaces.
- Use the slice spec and plan at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-4-slice-5-caller-rewire-and-compiler-narrowing-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-4-slice-5-caller-rewire-and-compiler-narrowing-plan.md
- Stay inside Packet 4.5.1 only.

Hard rules:
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them.
- Every fix subagent prompt must begin with `/goal ` and must explicitly use $incremental-implementation.
- Before editing any function, method, or other production symbol, run GitNexus impact analysis on the touched symbol(s) and report the blast radius. If GitNexus reports a stale index, refresh it first.
- Before every commit, run GitNexus detect-changes and verify the affected scope matches Packet 4.5.1 only.
- Commit after the implementation round before sending the change to review.
- Commit again after each accepted fix round that changes files, before re-reviewing.
- Do not amend away prior packet-round commits during the loop.
- Stay inside Packet 4.5.1 scope.

Packet 4.5.1 scope:
- Inventory and rewire production CLI imports to the real owner crates.
- Rewire CLI tests and support callers to the real owner crates.
- Remove remaining scaffold-level compiler forwarding from `handbook-flow`.
- Align crate manifests with the rewired ownership graph.
- Preserve current CLI/test behavior while changing imports and dependency posture.
- Expected files:
  - crates/cli/Cargo.toml
  - crates/cli/src/main.rs
  - crates/cli/tests/author_cli.rs
  - crates/cli/tests/cli_surface.rs
  - crates/flow/Cargo.toml
  - crates/flow/src/lib.rs
  - optionally Cargo.toml
  - optionally crates/compiler/Cargo.toml
  - optionally crates/engine/Cargo.toml
  - optionally crates/pipeline/Cargo.toml

Out of scope:
- deciding the final compiler end state beyond the minimum dependency cleanup required for rewires
- broad CLI module decomposition, prompting/wording changes, or other Phase 5 shell-thinning work
- major renderer/refusal/error relocation
- new runtime features or semantic changes to engine, pipeline, or flow behavior
- retiring `handbook-compiler` outright as part of this packet

Implementation subagent prompt requirements:
- Begin with `/goal Land Phase 4 Slice 5 Packet 4.5.1: Direct Caller Rewires To New Crates`.
- Tell the subagent to use $incremental-implementation.
- Require live verification of current direct-caller and dependency truth across:
  - `crates/cli/Cargo.toml`
  - `crates/cli/src/main.rs`
  - `crates/cli/tests/author_cli.rs`
  - `crates/cli/tests/cli_surface.rs`
  - `crates/flow/Cargo.toml`
  - `crates/flow/src/lib.rs`
  - `crates/engine/src/lib.rs`
  - `crates/pipeline/src/lib.rs`
  - `crates/flow/src/lib.rs`
  - `crates/compiler/src/lib.rs`
- Require the implementation to:
  - inventory all direct `handbook_compiler::*` uses in `crates/cli/**` and `crates/flow/**`
  - rewire those call sites to `handbook-engine`, `handbook-pipeline`, and `handbook-flow` where those crates already own the invoked surfaces
  - leave only explicitly justified unresolved compiler imports, if any, for Packet 4.5.2 to evaluate
  - align manifests so `crates/cli` depends directly on the real owner crates
  - preserve public CLI behavior and current test expectations
- Require the implementation to avoid broad `main.rs` decomposition, help-text rewrites, or speculative compiler retirement.
- Require GitNexus impact analysis before editing touched production symbols and require the subagent to report impacted callers/processes.
- Require targeted verification with:
  - `rg -n 'handbook_compiler::|use handbook_compiler|extern crate handbook_compiler' crates/cli crates/flow`
  - `cargo tree -p handbook-cli -e normal`
  - `cargo test -p handbook-cli --test author_cli`
  - `cargo test -p handbook-cli --test cli_surface`
  - `cargo test -p handbook-engine`
  - `cargo test -p handbook-pipeline`
  - `cargo test -p handbook-flow`
- Require the subagent to stop after Packet 4.5.1 acceptance is met and report touched files, impact-analysis results, remaining compiler-import inventory, dependency posture, verification run, and residual risks.

Review subagent prompt requirements:
- Begin with `/goal Review Phase 4 Slice 5 Packet 4.5.1: Direct Caller Rewires To New Crates`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to review the packet against the Slice 4.5 spec, plan, tasks, and verification evidence.
- Require special attention to:
  - whether direct callers now use the real owner crates for extracted logic
  - whether any remaining compiler imports are explicit, minimal, and justified rather than accidental leftovers
  - whether manifest changes reflect the real ownership graph
  - whether public CLI behavior and fixture-backed tests remained stable
  - whether the packet stayed bounded and did not drift into Packet 4.5.2 compiler-end-state work or Phase 5 CLI thinning
- Require severity labels and explicit callouts if the packet leaves hidden compiler-facade dependence, weakens crate-boundary clarity, or mixes in broad shell refactors.

Fix loop:
- If the review is clean, stop and report Packet 4.5.1 complete.
- If the review finds issues, spawn one fresh GPT-5.4 high fix subagent per review round using `$incremental-implementation`.
- The fix prompt must cite the exact review findings and require only the minimal Packet-4.5.1-bounded changes needed to close them.
- Re-run GitNexus impact analysis if new production symbols are touched.
- Re-run the bounded verification after each fix round.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after implementation if Packet 4.5.1 lands cleanly, before review.
- Before each commit, run GitNexus detect-changes and confirm the affected scope matches Packet 4.5.1 only.
- Commit after each accepted fix round, before re-review.
- Commit messages must describe the Packet 4.5.1 change clearly and standalone.

Stop conditions:
- Stop once Packet 4.5.1 is review-clean and committed.
- Stop and report blocked if Packet 4.5.1 cannot be completed without deciding the final compiler end state, performing broad CLI shell-thinning, or changing runtime behavior beyond the approved caller rewire boundary.
```

## Packet 4.5.2 Prompt

```text
/goal Orchestrate Phase 4 Slice 5 Packet 4.5.2: Compiler Narrowing Or Retirement Decision Landing in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 4.5.2 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-4-slice-5-caller-rewire-and-compiler-narrowing-tasks.md.
- Assume Packet 4.5.1 is already landed; verify live repo truth before changing anything, including the remaining `handbook-compiler` surface after direct caller rewires.
- Use the slice spec and plan at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-4-slice-5-caller-rewire-and-compiler-narrowing-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-4-slice-5-caller-rewire-and-compiler-narrowing-plan.md
- Stay inside Packet 4.5.2 only.

Hard rules:
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them.
- Every fix subagent prompt must begin with `/goal ` and must explicitly use $incremental-implementation.
- Before editing any function, method, or other production symbol, run GitNexus impact analysis on the touched symbol(s) and report the blast radius. If GitNexus reports a stale index, refresh it first.
- Before every commit, run GitNexus detect-changes and verify the affected scope matches Packet 4.5.2 only.
- Commit after the implementation round before sending the change to review.
- Commit again after each accepted fix round that changes files, before re-reviewing.
- Do not amend away prior packet-round commits during the loop.
- Stay inside Packet 4.5.2 scope.

Packet 4.5.2 scope:
- Land the chosen compiler end state as a deliberate workspace posture.
- Update repo-facing ownership docs and guard rails to match the chosen compiler posture.
- Prove the chosen posture with the full workspace verification wall.
- Expected files:
  - Cargo.toml
  - crates/compiler/Cargo.toml
  - crates/compiler/src/lib.rs
  - optionally remaining compiler source files if compiler is retained narrowly
  - optionally compiler-removal edits if retirement is chosen
  - README.md
  - PLAN.md
  - docs/README.md
  - docs/contracts/C-02-rust-workspace-and-cli-command-surface.md
  - optionally crates/cli/tests/help_drift_guard.rs

Out of scope:
- reopening Packet 4.5.1 direct caller rewires beyond tiny corrective changes strictly required to make the chosen compiler posture coherent
- broad CLI module decomposition or new Phase 5 shell helpers
- new engine/pipeline/flow runtime features
- speculative extractions beyond the approved Phase 4 owner crates

Implementation subagent prompt requirements:
- Begin with `/goal Land Phase 4 Slice 5 Packet 4.5.2: Compiler Narrowing Or Retirement Decision Landing`.
- Tell the subagent to use $incremental-implementation.
- Require live verification that Packet 4.5.1 already landed and that the remaining compiler surface is small enough to make an explicit keep-narrow or retire decision.
- Require live verification across:
  - `Cargo.toml`
  - `crates/compiler/Cargo.toml`
  - `crates/compiler/src/lib.rs`
  - any remaining compiler source files implicated by the chosen posture
  - `README.md`
  - `PLAN.md`
  - `docs/README.md`
  - `docs/contracts/C-02-rust-workspace-and-cli-command-surface.md`
  - `crates/cli/tests/help_drift_guard.rs`
- Require the implementation to:
  - choose one explicit end state for `crates/compiler`: narrow compatibility/support crate or retirement
  - make that end state real in the workspace graph rather than leaving compiler as an accidental umbrella
  - keep any retained compiler surface reviewably small and justified if compiler remains
  - update the minimum docs and help guards needed so ownership claims remain truthful after the decision
- Require the implementation to avoid broad CLI reshaping or new runtime behavior.
- Require GitNexus impact analysis before editing touched production symbols and require the subagent to report impacted callers/processes.
- Require targeted verification with:
  - `cargo tree -p handbook-compiler -e normal`
  - `cargo test -p handbook-cli --test help_drift_guard`
  - `cargo fmt --all -- --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`
- Require the subagent to stop after Packet 4.5.2 acceptance is met and report the chosen compiler posture, touched files, impact-analysis results, workspace-graph posture, verification run, and residual risks.

Review subagent prompt requirements:
- Begin with `/goal Review Phase 4 Slice 5 Packet 4.5.2: Compiler Narrowing Or Retirement Decision Landing`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to review the packet against the Slice 4.5 spec, plan, tasks, and verification evidence.
- Require special attention to:
  - whether the chosen compiler end state is explicit, coherent, and actually reflected in the workspace graph
  - whether compiler still owns any extracted logic it should no longer own
  - whether retained compiler exports, if any, are small and justified rather than accidental leftovers
  - whether docs and help guards now tell the truth about the command-surface and crate-boundary posture
  - whether Packet 4.5.2 stayed out of Phase 5 shell-thinning and new runtime behavior
- Require severity labels and explicit callouts if the landing leaves compiler as a de facto umbrella, understates remaining ownership ambiguity, or ships stale ownership docs.

Fix loop:
- If the review is clean, stop and report Packet 4.5.2 complete.
- If the review finds issues, spawn one fresh GPT-5.4 high fix subagent per review round using `$incremental-implementation`.
- The fix prompt must cite the exact review findings and require only the minimal Packet-4.5.2-bounded changes needed to close them.
- Re-run GitNexus impact analysis if new production symbols are touched.
- Re-run the bounded verification after each fix round.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after implementation if Packet 4.5.2 lands cleanly, before review.
- Before each commit, run GitNexus detect-changes and confirm the affected scope matches Packet 4.5.2 only.
- Commit after each accepted fix round, before re-review.
- Commit messages must describe the Packet 4.5.2 change clearly and standalone.

Stop conditions:
- Stop once Packet 4.5.2 is review-clean, committed, and validated on the full workspace wall.
- Stop and report blocked if Packet 4.5.2 cannot choose a compiler end state without widening into Phase 5 shell-thinning, new runtime feature work, or speculative extractions beyond the approved Phase 4 seams.
```
