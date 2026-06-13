# Handbook Engine Extraction Phase 5 Slice 3 Packet Prompts

Task source: [handbook-engine-extraction-phase-5-slice-3-cli-shell-closeout-tasks.md](./handbook-engine-extraction-phase-5-slice-3-cli-shell-closeout-tasks.md)
Spec source: [handbook-engine-extraction-phase-5-slice-3-cli-shell-closeout-spec.md](./handbook-engine-extraction-phase-5-slice-3-cli-shell-closeout-spec.md)
Plan source: [handbook-engine-extraction-phase-5-slice-3-cli-shell-closeout-plan.md](./handbook-engine-extraction-phase-5-slice-3-cli-shell-closeout-plan.md)

These prompts are ready to paste into fresh orchestration sessions. Each one starts in `/goal`, requires fresh `GPT-5.4` `high` subagents, uses `$incremental-implementation` for implementation/fix rounds, uses `$code-review-and-quality` for review rounds, preserves commit boundaries between implementation, review, and fix work, and requires GitNexus impact analysis before production-symbol edits plus GitNexus detect-changes before each commit.

Do not advance to the next packet until the current packet is review-clean and committed.

## Packet 5.3.1 Prompt

```text
/goal Orchestrate Phase 5 Slice 3 Packet 5.3.1: Prompting, Rendering, And Help Helper Extraction in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 5.3.1 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-5-slice-3-cli-shell-closeout-tasks.md.
- Verify live repo truth before changing anything, including that Phase 5 Slices 5.1 and 5.2 are already materially landed and that Slice 5.3 is now a CLI-shell ownership closeout seam rather than another broad command-family extraction.
- Use the slice spec and plan at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-5-slice-3-cli-shell-closeout-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-5-slice-3-cli-shell-closeout-plan.md
- Use the closeout seam map at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-closeout-four-set-map.md
- Preserve the already-landed Phase 5 shell split and do not start Packet 5.3.2.

Hard rules:
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, commit the implementation before any review round.
- Then spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them.
- Every fix subagent prompt must begin with `/goal ` and must explicitly use $incremental-implementation.
- After each accepted fix round that changes files, commit before dispatching the next fresh review subagent.
- Before editing any function, method, or other production symbol, run GitNexus impact analysis on the touched symbol(s) and report the blast radius. If GitNexus reports a stale index, refresh it first, e.g. with `npx gitnexus analyze`.
- Before every commit, run GitNexus detect-changes and verify the affected scope matches Packet 5.3.1 only.
- Stay inside Packet 5.3.1 scope.

Packet 5.3.1 scope:
- Isolate guided-prompt helper ownership from the oversized author shell module.
- Move CLI help summaries/examples out of reusable crates where they currently act as clap shell copy owners.
- Establish CLI-local last-mile presentation adapters for generate/inspect and adjacent shell output.
- Expected files:
  - crates/cli/src/author.rs
  - crates/cli/src/main.rs
  - crates/cli/src/pipeline.rs
  - crates/cli/src/rendering.rs
  - crates/cli/src/generate.rs
  - crates/cli/src/inspect.rs
  - optionally new CLI prompt/helper or help-text modules under `crates/cli/src/`
  - only the specific reusable help/rendering surfaces whose ownership is being narrowed
  - related targeted tests under `crates/cli/tests/**`

Out of scope:
- Packet 5.3.2 exit-code closeout beyond tiny compile-through adjustments strictly required for Packet 5.3.1
- any new command verbs, new supported targets, or CLI product-surface expansion
- Phase 1 layout/storage parameterization work
- Phase 2 orchestration-target parameterization work
- Phase 4 caller rewires or compiler narrowing as the main story
- broad CLI copywriting or vocabulary redesign
- a dumping-ground helper module that hides ownership instead of clarifying it

Implementation subagent prompt requirements:
- Begin with `/goal Land Phase 5 Slice 3 Packet 5.3.1: Prompting, Rendering, And Help Helper Extraction`.
- Tell the subagent to use $incremental-implementation.
- Require live verification that Phase 5 Slices 5.1 and 5.2 are already landed and that `crates/cli/src/main.rs` is already a thin clap/dispatch entrypoint before editing.
- Require the subagent to keep this packet bounded to prompt/help/render ownership and to explicitly preserve:
  - public help vocabulary and snapshot posture
  - guided interview behavior and author refusal posture
  - fixture-demo injection behavior for generate/inspect flows
  - the existing reduced-v1 command surface
  - reusable crates continuing to own typed runtime data, not final shell copy
  - Packet 5.3.2 exit-policy cleanup remaining deferred
- Require the subagent to isolate prompt helpers from `crates/cli/src/author.rs` without replacing them with trampolines or an unowned generic utils bucket.
- Require the subagent to move clap-facing help summaries/examples into CLI-owned surfaces if they currently live in reusable crates such as `crates/pipeline/src/pipeline.rs`.
- Require the subagent to establish CLI-owned last-mile rendering adapters for generate/inspect shell output instead of delegating final shell copy wholesale to reusable-crate renderers.
- Require GitNexus impact analysis before editing touched production symbols and require the subagent to report impacted callers/processes.
- Require targeted verification with:
  - `rg -n "prompt_|print_help|after_help|SUPPORTED_.*HELP|render_(markdown|inspect|json)|render_supported_handoff_emit_command|OUTCOME:|NEXT SAFE ACTION:" crates/cli/src crates/pipeline/src crates/compiler/src`
  - `cargo test -p handbook-cli --test author_cli`
  - `cargo test -p handbook-cli --test help_drift_guard`
  - `cargo test -p handbook-cli --test cli_surface`
  - `cargo test -p handbook-cli --test pipeline_handoff_refusals`
  - `cargo test -p handbook-cli --test manual_qa_fixture_checkout`
  - `cargo test -p handbook-cli --test feature_spec_contract`
- Require the subagent to stop after Packet 5.3.1 acceptance is met and report touched files, impact-analysis results, verification run, remaining shell-ownership leaks if any, and residual risks.

Review subagent prompt requirements:
- Begin with `/goal Review Phase 5 Slice 3 Packet 5.3.1: Prompting, Rendering, And Help Helper Extraction`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to review the packet against the Slice 5.3 spec, plan, tasks, the Phase 5 Slice 5.1/5.2 boundaries, and the verification evidence.
- Require special attention to:
  - whether prompt ownership is clearer without behavior drift in author flows
  - whether help summaries/examples are now CLI-owned instead of reusable-crate-owned
  - whether generate/inspect final shell presentation is now CLI-owned
  - whether Packet 5.3.2 exit-policy work leaked in early
  - whether the implementation introduced trampoline modules or generic helper buckets that weaken ownership clarity
- Require severity labels and explicit callouts if public help/output drifted, if reusable crates still clearly own final shell copy, or if scope widened beyond Packet 5.3.1.

Fix loop:
- If the review is clean, stop and report Packet 5.3.1 complete.
- If the review finds issues, spawn one fresh GPT-5.4 high fix subagent per review round using `$incremental-implementation`.
- The fix prompt must cite the exact review findings and require only the minimal Packet-5.3.1-bounded changes needed to close them.
- Re-run GitNexus impact analysis if new production symbols are touched.
- Re-run the bounded verification after each fix round.
- Commit accepted fixes before dispatching the next fresh review subagent.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after implementation if Packet 5.3.1 lands cleanly.
- Before each commit, run GitNexus detect-changes and confirm the affected scope matches Packet 5.3.1 only.
- Commit after each accepted fix round.
- Do not batch Packet 5.3.1 changes together with Packet 5.3.2 work.
- Commit messages must describe the Packet 5.3.1 change clearly and standalone.

Stop conditions:
- Stop once Packet 5.3.1 is review-clean and committed.
- Stop and report blocked if Packet 5.3.1 cannot be completed without widening into Packet 5.3.2, changing the approved Slice 5.3 spec/plan/tasks, or introducing broader architectural work that belongs to Sets 1 through 3.
```

## Packet 5.3.2 Prompt

```text
/goal Orchestrate Phase 5 Slice 3 Packet 5.3.2: Exit-Code And Final Shell Closeout in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 5.3.2 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-5-slice-3-cli-shell-closeout-tasks.md.
- Assume Packet 5.3.1 is already landed, review-clean, and committed; verify live repo truth before changing anything.
- Use the slice spec and plan at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-5-slice-3-cli-shell-closeout-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-5-slice-3-cli-shell-closeout-plan.md
- Use the closeout seam map at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-closeout-four-set-map.md
- Preserve Packet 5.3.1 ownership changes and do not reopen them except where live repo truth proves a tiny corrective adjustment is strictly required for Packet 5.3.2 to land correctly.

Hard rules:
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, commit the implementation before any review round.
- Then spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them with `$incremental-implementation`.
- After each accepted fix round that changes files, commit before dispatching the next fresh review subagent.
- Before editing any function, method, or other production symbol, run GitNexus impact analysis on the touched symbol(s) and report the blast radius. If GitNexus reports a stale index, refresh it first, e.g. with `npx gitnexus analyze`.
- Before every commit, run GitNexus detect-changes and verify the affected scope matches Packet 5.3.2 only.
- Stay inside Packet 5.3.2 scope.

Packet 5.3.2 scope:
- Centralize or clearly justify the remaining CLI exit-code policy boundaries.
- Remove or explicitly justify remaining reusable-crate final shell copy exports.
- Re-run the full closeout wall and confirm Phase 5 is honestly at the CLI-shell steady state.
- Expected files:
  - crates/cli/src/generate.rs
  - crates/cli/src/inspect.rs
  - crates/cli/src/doctor.rs
  - crates/cli/src/setup.rs
  - crates/cli/src/rendering.rs
  - crates/cli/src/doctor_rendering.rs
  - optionally narrowly-scoped CLI support modules needed to make exit ownership explicit
  - the specific reusable help/render surfaces still participating in final shell-copy ownership
  - related targeted tests under `crates/cli/tests/**`

Out of scope:
- reopening prompt/helper/help ownership broadly beyond tiny corrective carry-forward from Packet 5.3.1
- new command verbs, new supported targets, or product-shell feature additions
- Phase 1/2/4 architectural work
- broad copy rewrite or command-taxonomy redesign
- moving reusable runtime logic into the CLI instead of only final shell presentation/exit policy

Implementation subagent prompt requirements:
- Begin with `/goal Land Phase 5 Slice 3 Packet 5.3.2: Exit-Code And Final Shell Closeout`.
- Tell the subagent to use $incremental-implementation.
- Require live verification that Packet 5.3.1 already landed and that prompt/help/render ownership is already clearer before editing.
- Require the subagent to keep this packet bounded to exit-policy and final-shell closeout and to explicitly preserve:
  - existing success/failure semantics for setup/generate/inspect/doctor/pipeline flows
  - guided prompting behavior already preserved by Packet 5.3.1
  - public help vocabulary and snapshots
  - fixture-demo and feature-spec output behavior
  - `main.rs` remaining thin and honest
- Require the subagent to make CLI exit-code ownership explicit for the remaining shell flows instead of leaving it split across reusable crates and ad hoc command-family logic.
- Require the subagent to remove, narrow, or explicitly justify remaining reusable-crate exports whose only job is final operator-facing help/render/command copy.
- Require GitNexus impact analysis before editing touched production symbols and require the subagent to report impacted callers/processes.
- Require targeted verification with:
  - `rg -n "SUPPORTED_.*HELP|render_(markdown|inspect|json)|render_supported_handoff_emit_command|ExitCode" crates/cli/src crates/pipeline/src crates/compiler/src`
  - `cargo test -p handbook-cli --test cli_surface`
  - `cargo test -p handbook-cli --test manual_qa_fixture_checkout`
  - `cargo test -p handbook-cli --test feature_spec_contract`
  - `cargo test -p handbook-cli --test help_drift_guard`
  - `cargo test -p handbook-cli`
  - `cargo fmt --all -- --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`
- Require the subagent to stop after Packet 5.3.2 acceptance is met and report touched files, impact-analysis results, verification run, any remaining justified reusable-crate shell-copy exports, and residual risks.

Review subagent prompt requirements:
- Begin with `/goal Review Phase 5 Slice 3 Packet 5.3.2: Exit-Code And Final Shell Closeout`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to review the packet against the Slice 5.3 spec, plan, tasks, the landed Packet 5.3.1 state, and the verification evidence.
- Require special attention to:
  - whether exit-code ownership is now clearly CLI-local
  - whether remaining reusable-crate shell-copy exports were genuinely removed or explicitly justified
  - whether CLI behavior, help snapshots, and proof output stayed stable
  - whether `main.rs` and CLI module boundaries remain honest after the final closeout
  - whether any Packet-5.3.1 work was regressed or any neighboring-set architecture work leaked in
- Require severity labels and explicit callouts if exit semantics drifted, if final shell copy is still materially reusable-crate-owned, or if scope widened beyond Packet 5.3.2.

Fix loop:
- If the review is clean, stop and report Packet 5.3.2 complete.
- If the review finds issues, spawn a fresh GPT-5.4 high fix subagent using `$incremental-implementation`.
- The fix prompt must cite the exact review findings and require only the minimal Packet-5.3.2-bounded changes needed to close them.
- Re-run GitNexus impact analysis if new production symbols are touched.
- Re-run the bounded verification after each fix round.
- Commit accepted fixes before dispatching the next fresh review subagent.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after Packet 5.3.2 implementation if it lands cleanly.
- Before each commit, run GitNexus detect-changes and confirm the affected scope matches Packet 5.3.2 only.
- Commit after each accepted fix round.
- Commit messages must describe the Packet 5.3.2 change clearly and standalone.

Stop conditions:
- Stop once Packet 5.3.2 is review-clean, committed, and the full closeout wall passes.
- Stop and report blocked if Packet 5.3.2 requires widening beyond the approved Slice 5.3 spec/plan/tasks, regressing Packet 5.3.1 ownership work, or pulling in broader architectural work from Sets 1 through 3.
```
