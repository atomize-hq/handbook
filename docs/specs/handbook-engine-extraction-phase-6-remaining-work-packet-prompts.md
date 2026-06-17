# Handbook Engine Extraction Phase 6 Remaining Work Packet Prompts

Task source: [handbook-engine-extraction-phase-6-remaining-work-tasks.md](./handbook-engine-extraction-phase-6-remaining-work-tasks.md)
Spec source: [handbook-engine-extraction-phase-6-remaining-work-spec.md](./handbook-engine-extraction-phase-6-remaining-work-spec.md)
Plan source: [handbook-engine-extraction-phase-6-remaining-work-plan.md](./handbook-engine-extraction-phase-6-remaining-work-plan.md)

These prompts are ready to paste into fresh orchestration sessions. Each one starts in `/goal`, requires fresh `GPT-5.4` `high` subagents, uses `$incremental-implementation` for implementation and fix rounds, uses `$code-review-and-quality` for review rounds, preserves commit boundaries between implementation, review, and fix work, and keeps execution bounded to the approved Phase 6 remaining-work seam.

This slice is approved only for the remaining Phase 6 packet set captured in this artifact: Lane B (flow required-import boundary cleanup + contract freeze, including Packets 6.B.1 through 6.B.4), Lane C Packet 6.C.1 (defer-or-activate decision), and Lane D (final Substrate import plan / review gate). Do not widen into executing the actual Substrate import, reopening Lane A, CLI shell redesign, compiler retirement, publication, crates.io work, or making `substrate-context` become handbook. If the narrow packet cannot land honestly without such widening, stop and report the blocker instead of silently expanding scope.

The orchestration session must not implement, review, or fix the packet work itself first. It must delegate implementation, review, and any fix rounds to fresh subagents exactly as directed below.

---

## Packet 6.B.1 Prompt — Gather Evidence

```text
/goal Orchestrate Phase 6 Remaining Work Packet 6.B.1: Gather Flow Required-Import Boundary Evidence in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 6.B.1 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-remaining-work-tasks.md.
- Verify live repo truth before changing anything.
- Use the approved seam authority at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-remaining-work-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-remaining-work-plan.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-remaining-work-tasks.md
- Treat the packet as one narrow evidence seam: record both the already-clean crate/type dependency boundary and the residual shell-owned/operator-facing copy still leaking through the public flow surface.
- `handbook-flow` is a required import target. This packet does not decide otherwise; it documents what still must be cleaned up before the contract is frozen.
- Stay inside Packet 6.B.1 only.

Hard rules:
- Do not implement, review, or fix Packet 6.B.1 work in the orchestration session yourself.
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, commit the implementation before any review round.
- Then spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them.
- Every fix subagent prompt must begin with `/goal ` and must explicitly use $incremental-implementation.
- After each accepted fix round that changes files, commit before dispatching the next fresh review subagent.
- Before every commit, run GitNexus detect-changes and verify the affected scope matches Packet 6.B.1 only.
- Stay inside Packet 6.B.1 scope.

Packet 6.B.1 scope:
- Capture `cargo tree -p handbook-flow` output and record it, showing only `handbook-engine` as the intra-workspace dependency.
- Capture `rg -n "handbook_compiler|handbook_cli|handbook_pipeline" crates/flow/src/ crates/flow/tests/` output and record zero matches.
- Trace transitive type dependencies for all in-boundary public symbols from `resolver`, `budget`, `packet_result` and confirm each resolves only to `handbook-engine` public types, std types, or flow-local types.
- Inspect `crates/flow/src/resolver.rs`, `crates/flow/src/packet_result.rs`, `crates/cli/src/rendering.rs`, and `crates/compiler/src/rendering/shared.rs` to record exactly what final shell-owned/operator-facing copy still leaks through flow.
- Explicitly distinguish typed next-action/status semantics that may remain machine-readable from final shell wording/command strings that Packet 6.B.2 must move out.
- Record all evidence into the evidence section of `docs/specs/handbook-flow-import-boundary-consumer-contract.md`.
- Expected files:
  - docs/specs/handbook-flow-import-boundary-consumer-contract.md (evidence section only)

Out of scope — do NOT touch:
- Packet 6.B.2 production cleanup work
- Packet 6.B.3 consumer-contract formalization
- Packet 6.B.4 verification wall
- Lane C (engine optional boundary freeze)
- Lane D (import/adoption plan)
- any production code changes in `crates/flow/src/**`, `crates/cli/src/rendering.rs`, or `crates/compiler/src/rendering/shared.rs`
- publication or crates.io work
- Substrate consumption or broader integration implementation
- any new packet authoring beyond this approved packet prompt artifact

Implementation subagent prompt requirements:
- Begin with `/goal Land Phase 6 Remaining Work Packet 6.B.1: Gather Flow Required-Import Boundary Evidence`.
- Tell the subagent to use $incremental-implementation.
- Require live verification with:
  - `git status --short --branch`
  - `cargo tree -p handbook-flow`
  - `rg -n "handbook_compiler|handbook_cli|handbook_pipeline" crates/flow/src/ crates/flow/tests/`
  - `sed -n '70,210p' crates/flow/src/resolver.rs`
  - `sed -n '1,140p' crates/flow/src/packet_result.rs`
  - `sed -n '518,590p' crates/cli/src/rendering.rs`
  - `sed -n '315,360p' crates/compiler/src/rendering/shared.rs`
- Require the implementation to:
  - run `cargo tree -p handbook-flow` and record the exact output
  - run `rg -n "handbook_compiler|handbook_cli|handbook_pipeline" crates/flow/src/ crates/flow/tests/` and record the zero-match result
  - inspect each public symbol in `resolver`, `budget`, `packet_result` and trace its type dependencies
  - record exactly what final shell-owned/operator-facing copy still leaks through flow and where it lives
  - explicitly distinguish typed next-action/status semantics that may remain machine-readable from final shell wording/command strings that must move out in Packet 6.B.2
  - record all evidence into the evidence section of `docs/specs/handbook-flow-import-boundary-consumer-contract.md`
  - avoid widening into production cleanup, contract formalization, verification, Lane C, Lane D, or any production code changes
- Require the subagent to stop after Packet 6.B.1 acceptance is met and report touched files, evidence captured, verification run, and residual risks.

Review subagent prompt requirements:
- Begin with `/goal Review Phase 6 Remaining Work Packet 6.B.1: Gather Flow Required-Import Boundary Evidence`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to review the packet against the approved seam spec, plan, tasks, and verification evidence.
- Require special attention to:
  - whether the clean crate/type dependency proof is complete and accurate
  - whether the residual shell-owned/operator-facing leakage is recorded honestly rather than hand-waved away
  - whether the packet distinguishes typed semantics that may remain from final shell wording/command strings that must move out
  - whether any evidence was paraphrased rather than recorded from live output
  - whether the evidence section is internally consistent with live source
- Require severity labels and explicit callouts if evidence is incomplete, inaccurate, or if scope leaked beyond Packet 6.B.1.

Fix loop:
- If the review is clean, stop and report Packet 6.B.1 complete.
- If the review finds issues, spawn one fresh GPT-5.4 high fix subagent per review round using `$incremental-implementation`.
- The fix prompt must cite the exact review findings and require only the minimal Packet-6.B.1-bounded changes needed to close them.
- Commit accepted fixes before dispatching the next fresh review subagent.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after implementation if Packet 6.B.1 lands cleanly.
- Before each commit, run GitNexus detect-changes and confirm the affected scope matches Packet 6.B.1 only.
- Commit after each accepted fix round.
- Commit messages must describe the evidence-gathering work clearly and standalone.

Stop conditions:
- Stop once Packet 6.B.1 is review-clean, committed, and the evidence clearly captures both the clean import boundary and the remaining shell-owned leakage.
- Stop and report blocked if the packet cannot close honestly without widening into production cleanup, consumer-contract authoring, verification, Lane C, Lane D, or broader integration implementation.
```

---

## Packet 6.B.2 Prompt — Clean Flow Import-Surface Shell Ownership

```text
/goal Orchestrate Phase 6 Remaining Work Packet 6.B.2: Clean Flow Import-Surface Shell Ownership in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 6.B.2 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-remaining-work-tasks.md.
- Verify live repo truth before changing anything.
- Use the approved seam authority at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-remaining-work-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-remaining-work-plan.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-remaining-work-tasks.md
- Treat the packet as one narrow production seam: remove final shell-owned/operator-facing copy from the public flow import surface while preserving typed next-action/status semantics wherever that keeps the cleanup narrow and honest.
- `handbook-flow` remains a required import target; `handbook-cli` remains the only product shell.
- This packet depends on Packet 6.B.1 being complete (the evidence must already distinguish clean boundary proof from residual shell leakage).
- Stay inside Packet 6.B.2 only.

Hard rules:
- Do not implement, review, or fix Packet 6.B.2 work in the orchestration session yourself.
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, commit the implementation before any review round.
- Then spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them.
- Every fix subagent prompt must begin with `/goal ` and must explicitly use $incremental-implementation.
- After each accepted fix round that changes files, commit before dispatching the next fresh review subagent.
- Before editing any production symbol or helper under `crates/flow/src/**`, `crates/cli/src/rendering.rs`, or `crates/compiler/src/rendering/shared.rs`, run GitNexus impact analysis first and report the blast radius. If the blast radius is HIGH or CRITICAL, stop and report it before editing.
- Before every commit, run GitNexus detect-changes and verify the affected scope matches Packet 6.B.2 only.
- Stay inside Packet 6.B.2 scope.

Packet 6.B.2 scope:
- Remove final shell-owned/operator-facing command strings or equivalent final rendered shell copy from the public flow surface.
- Keep typed next-action/status semantics if they remain useful machine-readable flow results after the cleanup.
- Update CLI/compiler rendering or adapter code only as needed so final shell wording lives outside flow.
- Add or update only the minimal tests needed to verify the cleaned import surface.
- Expected files:
  - crates/flow/src/resolver.rs
  - crates/flow/src/packet_result.rs
  - crates/cli/src/rendering.rs
  - crates/compiler/src/rendering/shared.rs
  - minimally impacted tests, if needed

Out of scope — do NOT touch:
- Full CLI shell redesign or doctor/setup redesign
- Removing typed next-safe-action semantics purely because they mention setup/doctor if they can remain as machine-readable data
- Packet 6.B.3 consumer-contract formalization
- Packet 6.B.4 verification wall
- Lane C (engine optional boundary freeze)
- Lane D (import/adoption plan)
- publication or crates.io work
- Substrate consumption or broader integration implementation
- any new packet authoring beyond this approved packet prompt artifact

Implementation subagent prompt requirements:
- Begin with `/goal Land Phase 6 Remaining Work Packet 6.B.2: Clean Flow Import-Surface Shell Ownership`.
- Tell the subagent to use $incremental-implementation.
- Require live verification with:
  - `git status --short --branch`
  - `sed -n '1,220p' docs/specs/handbook-flow-import-boundary-consumer-contract.md`
  - `sed -n '70,210p' crates/flow/src/resolver.rs`
  - `sed -n '1,140p' crates/flow/src/packet_result.rs`
  - `sed -n '518,590p' crates/cli/src/rendering.rs`
  - `sed -n '315,360p' crates/compiler/src/rendering/shared.rs`
  - `rg -n 'run `doctor`|handbook inspect --packet|handbook generate --packet|handbook setup' crates/flow/src/`
- Require the implementation to:
  - verify Packet 6.B.1 evidence exists before changing code
  - run GitNexus impact analysis before editing any production symbols and report the blast radius
  - remove final shell-owned/operator-facing copy from the public flow surface
  - preserve typed next-action/status semantics where they remain useful machine-readable data
  - update CLI/compiler rendering or adapter code only as needed so final shell wording remains outside flow
  - rerun and record the post-cleanup proof shape required by the tasks authority: source inspection of `crates/flow/src/resolver.rs` and `crates/flow/src/packet_result.rs`, the shell-copy spot-check `rg -n 'run \`doctor\`|handbook inspect --packet|handbook generate --packet|handbook setup' crates/flow/src/`, and `cargo test -p handbook-flow`
  - run `cargo test -p handbook-flow`
  - run `cargo check --workspace`
  - avoid widening into broader CLI redesign, contract formalization, verification, Lane C, Lane D, or actual Substrate import work
- Require the subagent to stop after Packet 6.B.2 acceptance is met and report touched files, impact-analysis results, the recorded post-cleanup source-inspection + shell-copy proof, verification run, and residual risks.

Review subagent prompt requirements:
- Begin with `/goal Review Phase 6 Remaining Work Packet 6.B.2: Clean Flow Import-Surface Shell Ownership`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to review the packet against the approved seam spec, plan, tasks, and Packet 6.B.1 evidence.
- Require special attention to:
  - whether final shell-owned/operator-facing copy is actually gone from the public flow surface
  - whether typed next-action/status semantics were preserved appropriately instead of being widened into a redesign
  - whether CLI/compiler now own the final shell wording without scope creep
  - whether the post-cleanup source inspection and shell-copy `rg` proof were rerun and recorded, not just the pre-change baseline
  - whether GitNexus impact analysis was run before production symbol edits and the blast radius stayed acceptable
  - whether any scope leaked beyond Packet 6.B.2
- Require severity labels and explicit callouts if the cleanup is incomplete, widened unnecessarily, or skipped the required impact analysis.

Fix loop:
- If the review is clean, stop and report Packet 6.B.2 complete.
- If the review finds issues, spawn one fresh GPT-5.4 high fix subagent per review round using `$incremental-implementation`.
- The fix prompt must cite the exact review findings and require only the minimal Packet-6.B.2-bounded changes needed to close them.
- Commit accepted fixes before dispatching the next fresh review subagent.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after implementation if Packet 6.B.2 lands cleanly.
- Before each commit, run GitNexus detect-changes and confirm the affected scope matches Packet 6.B.2 only.
- Commit after each accepted fix round.
- Commit messages must describe the import-surface cleanup clearly and standalone.

Stop conditions:
- Stop once Packet 6.B.2 is review-clean, committed, and the final shell-owned flow copy has been moved out without a broader redesign.
- Stop and report blocked if the packet cannot close honestly without widening into a larger CLI/doctor/setup redesign, Lane C, Lane D, or broader integration implementation.
```

---

## Packet 6.B.3 Prompt — Formalize Consumer Contract

```text
/goal Orchestrate Phase 6 Remaining Work Packet 6.B.3: Formalize Flow Import-Boundary Consumer Contract in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 6.B.3 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-remaining-work-tasks.md.
- Verify live repo truth before changing anything.
- Use the approved seam authority at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-remaining-work-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-remaining-work-plan.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-remaining-work-tasks.md
- Treat the packet as one narrow contract-formalization seam: write the standalone consumer contract document for the cleaned `handbook-flow` surface.
- This packet depends on Packet 6.B.1 evidence and Packet 6.B.2 cleanup both being complete.
- Stay inside Packet 6.B.3 only.

Hard rules:
- Do not implement, review, or fix Packet 6.B.3 work in the orchestration session yourself.
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, commit the implementation before any review round.
- Then spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them.
- Every fix subagent prompt must begin with `/goal ` and must explicitly use $incremental-implementation.
- After each accepted fix round that changes files, commit before dispatching the next fresh review subagent.
- Before every commit, run GitNexus detect-changes and verify the affected scope matches Packet 6.B.3 only.
- Stay inside Packet 6.B.3 scope.

Packet 6.B.3 scope:
- Write the full standalone consumer contract document at `docs/specs/handbook-flow-import-boundary-consumer-contract.md` that records:
  - The frozen in-boundary symbol set — all public re-exports from `budget`, `packet_result`, `resolver`
  - Their transitive type dependencies (engine-public, std, or flow-local only)
  - Which typed next-action/status semantics remain in-boundary after Packet 6.B.2
  - Which shell-owned/operator-facing copy and rendering responsibilities are explicitly out of boundary
  - The contract version function (`flow_contract_version()`) and its delegation to `handbook_engine::workspace_contract_version()`
  - Evidence references from Packet 6.B.1 and cleanup references from Packet 6.B.2
- Preserve the evidence section from Packet 6.B.1 and integrate it into the full document.
- Expected files:
  - docs/specs/handbook-flow-import-boundary-consumer-contract.md (full document)

Out of scope — do NOT touch:
- Packet 6.B.2 production cleanup work
- Packet 6.B.4 verification wall
- Any production code changes in `crates/flow/src/**` or any other crate source
- Lane C (engine optional boundary freeze)
- Lane D (import/adoption plan)
- publication or crates.io work
- Substrate consumption or broader integration implementation
- any new packet authoring beyond this approved packet prompt artifact

Implementation subagent prompt requirements:
- Begin with `/goal Land Phase 6 Remaining Work Packet 6.B.3: Formalize Flow Import-Boundary Consumer Contract`.
- Tell the subagent to use $incremental-implementation.
- Require live verification with:
  - `git status --short --branch`
  - `sed -n '1,220p' docs/specs/handbook-flow-import-boundary-consumer-contract.md`
  - `rg -n "pub (fn|struct|enum|type|const)" crates/flow/src/resolver.rs crates/flow/src/budget.rs crates/flow/src/packet_result.rs`
  - `rg -n "flow_contract_version" crates/flow/src/`
- Require the implementation to:
  - verify Packet 6.B.1 evidence exists and Packet 6.B.2 cleanup has landed before writing the full contract
  - record every public symbol from `resolver`, `budget`, `packet_result` exactly as it appears in live source
  - record which typed next-action/status semantics remain in-boundary after the cleanup
  - record which shell-owned/operator-facing copy and rendering responsibilities now live outside flow
  - record the contract version function and its delegation chain
  - reference the Packet 6.B.1 evidence and Packet 6.B.2 cleanup outcome
  - ensure the document is internally consistent with live source — no paraphrased or invented symbol names
  - avoid widening into verification, production code changes, Lane C, Lane D, or broader integration
- Require the subagent to stop after Packet 6.B.3 acceptance is met and report touched files, contract contents summary, verification run, and residual risks.

Review subagent prompt requirements:
- Begin with `/goal Review Phase 6 Remaining Work Packet 6.B.3: Formalize Flow Import-Boundary Consumer Contract`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to review the packet against the approved seam spec, plan, tasks, and Packet 6.B.1/6.B.2 outcomes.
- Require special attention to:
  - whether every public symbol is recorded accurately
  - whether the in-boundary typed semantics vs out-of-boundary shell copy distinction is explicit and honest
  - whether the consumer contract matches the cleaned live source rather than the pre-cleanup state
  - whether the contract version function and delegation chain are correct
  - whether the evidence section was preserved and integrated, not overwritten
  - whether any scope leaked beyond Packet 6.B.3
- Require severity labels and explicit callouts if the contract is incomplete, inaccurate, or out of sync with the cleaned surface.

Fix loop:
- If the review is clean, stop and report Packet 6.B.3 complete.
- If the review finds issues, spawn one fresh GPT-5.4 high fix subagent per review round using `$incremental-implementation`.
- The fix prompt must cite the exact review findings and require only the minimal Packet-6.B.3-bounded changes needed to close them.
- Commit accepted fixes before dispatching the next fresh review subagent.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after implementation if Packet 6.B.3 lands cleanly.
- Before each commit, run GitNexus detect-changes and confirm the affected scope matches Packet 6.B.3 only.
- Commit after each accepted fix round.
- Commit messages must describe the consumer contract formalization clearly and standalone.

Stop conditions:
- Stop once Packet 6.B.3 is review-clean, committed, and the consumer contract doc is internally consistent with the cleaned live source.
- Stop and report blocked if the packet cannot close honestly without widening into verification, production code changes, Lane C, Lane D, or broader integration implementation.
```

---

## Packet 6.B.4 Prompt — Verification Wall

```text
/goal Orchestrate Phase 6 Remaining Work Packet 6.B.4: Run Flow Import-Boundary Verification Wall in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 6.B.4 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-remaining-work-tasks.md.
- Verify live repo truth before changing anything.
- Use the approved seam authority at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-remaining-work-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-remaining-work-plan.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-remaining-work-tasks.md
- Treat the packet as one narrow verification seam: run the full Lane B verification wall, record pass/fail for each command, and record completion notes in the tasks doc.
- This packet depends on Packets 6.B.1, 6.B.2, and 6.B.3 being complete.
- Stay inside Packet 6.B.4 only.

Hard rules:
- Do not implement, review, or fix Packet 6.B.4 work in the orchestration session yourself.
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, commit the implementation before any review round.
- Then spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them.
- Every fix subagent prompt must begin with `/goal ` and must explicitly use $incremental-implementation.
- After each accepted fix round that changes files, commit before dispatching the next fresh review subagent.
- Before every commit, run GitNexus detect-changes and verify the affected scope matches Packet 6.B.4 only.
- Stay inside Packet 6.B.4 scope.

Packet 6.B.4 scope:
- Run each of the following verification checks and record pass/fail:
  - `cargo tree -p handbook-flow` — must show only `handbook-engine` as the intra-workspace dependency
  - `rg -n "handbook_compiler|handbook_cli|handbook_pipeline" crates/flow/src/ crates/flow/tests/` — must return zero matches
  - source-inspect the public `handbook-flow` surface (`crates/flow/src/lib.rs`, `crates/flow/src/budget.rs`, `crates/flow/src/packet_result.rs`, `crates/flow/src/resolver.rs`) against the Packet 6.B.3 consumer contract and prove no final shell-owned/operator-facing copy remains on that surface; any remaining next-action/status data must be typed/machine-readable only
  - `rg -n 'run `doctor`|handbook inspect --packet|handbook generate --packet|handbook setup' crates/flow/src/` — must return zero matches as a supporting spot-check, not the sole proof
  - `cargo test -p handbook-flow` — must pass
  - `cargo check --workspace` — must pass
  - `cargo fmt --all -- --check && cargo clippy --workspace --all-targets -- -D warnings` — must pass
- Record completion notes in `docs/specs/handbook-engine-extraction-phase-6-remaining-work-tasks.md` (Packet 6.B.4 section) with pass/fail status for each command plus the broader source-inspection proof outcome.
- If any command fails, stop and report the blocker — do not attempt to fix the underlying issue in this packet without explicit direction.
- Expected files:
  - docs/specs/handbook-engine-extraction-phase-6-remaining-work-tasks.md (completion notes section)

Out of scope — do NOT touch:
- Any production code changes in any crate
- Lane C (engine optional boundary freeze)
- Lane D (import/adoption plan)
- publication or crates.io work
- Substrate consumption or broader integration implementation
- any new packet authoring beyond this approved packet prompt artifact

Implementation subagent prompt requirements:
- Begin with `/goal Land Phase 6 Remaining Work Packet 6.B.4: Run Flow Import-Boundary Verification Wall`.
- Tell the subagent to use $incremental-implementation.
- Require live verification with:
  - `git status --short --branch`
  - `sed -n '1,220p' docs/specs/handbook-flow-import-boundary-consumer-contract.md`
  - `sed -n '1,240p' crates/flow/src/lib.rs`
  - `sed -n '1,260p' crates/flow/src/budget.rs`
  - `sed -n '1,220p' crates/flow/src/packet_result.rs`
  - `sed -n '1,320p' crates/flow/src/resolver.rs`
- Require the implementation to:
  - verify Packets 6.B.1, 6.B.2, and 6.B.3 are complete before running the wall
  - run `cargo tree -p handbook-flow` and record the exact output with pass/fail
  - run `rg -n "handbook_compiler|handbook_cli|handbook_pipeline" crates/flow/src/ crates/flow/tests/` and record the result with pass/fail
  - source-inspect `crates/flow/src/lib.rs`, `crates/flow/src/budget.rs`, `crates/flow/src/packet_result.rs`, and `crates/flow/src/resolver.rs` against the Packet 6.B.3 consumer contract and record an explicit proof that no final shell-owned/operator-facing copy remains on the public flow surface
  - record whether any remaining next-action/status data is still typed/machine-readable only and contract-approved
  - run `rg -n 'run `doctor`|handbook inspect --packet|handbook generate --packet|handbook setup' crates/flow/src/` and record the result with pass/fail as supporting spot-check evidence, not as the sole proof
  - run `cargo test -p handbook-flow` and record pass/fail
  - run `cargo check --workspace` and record pass/fail
  - run `cargo fmt --all -- --check && cargo clippy --workspace --all-targets -- -D warnings` and record pass/fail
  - record all results in the Packet 6.B.4 completion notes section of `docs/specs/handbook-engine-extraction-phase-6-remaining-work-tasks.md`, including the broader source-inspection proof outcome
  - if any command fails, stop and report the exact failure output — do not attempt fixes
  - avoid widening into production code changes, Lane C, Lane D, or broader integration
- Require the subagent to stop after Packet 6.B.4 acceptance is met and report touched files, verification results, and residual risks.

Review subagent prompt requirements:
- Begin with `/goal Review Phase 6 Remaining Work Packet 6.B.4: Run Flow Import-Boundary Verification Wall`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to review the packet against the approved seam spec, plan, tasks, and the recorded verification results.
- Require special attention to:
  - whether the broader source-inspection proof actually covers the full public `handbook-flow` surface rather than only a few known strings
  - whether the Packet 6.B.3 consumer contract and the verification notes agree about what remains in-boundary vs out-of-boundary
  - whether all verification commands were run and their output was recorded accurately
  - whether pass/fail status is honest and matches actual command output
  - whether the shell-copy spot-check in `crates/flow/src/` was recorded accurately and not overstated as the whole proof
  - whether any command output was paraphrased or omitted
  - whether the completion notes are consistent with the consumer contract doc and live source
  - whether any scope leaked beyond Packet 6.B.4
- Require severity labels and explicit callouts if any verification result is missing, inaccurate, or if scope leaked.

Fix loop:
- If the review is clean, stop and report Packet 6.B.4 complete.
- If the review finds issues, spawn one fresh GPT-5.4 high fix subagent per review round using `$incremental-implementation`.
- The fix prompt must cite the exact review findings and require only the minimal Packet-6.B.4-bounded changes needed to close them.
- Commit accepted fixes before dispatching the next fresh review subagent.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after implementation if Packet 6.B.4 lands cleanly.
- Before each commit, run GitNexus detect-changes and confirm the affected scope matches Packet 6.B.4 only.
- Commit after each accepted fix round.
- Commit messages must describe the verification wall results clearly and standalone.

Stop conditions:
- Stop once Packet 6.B.4 is review-clean, committed, and the verification notes honestly record both the broader public-surface proof and the supporting command results.
- Stop and report blocked if any verification command fails and the failure cannot be resolved within Packet 6.B.4 scope (i.e., the failure requires production code changes or widening into another lane).
```

---

## Packet 6.C.1 Prompt — Defer Or Activate (Decision Task)

```text
/goal Orchestrate Phase 6 Remaining Work Packet 6.C.1: Record Lane C Deferral Decision in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 6.C.1 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-remaining-work-tasks.md.
- Verify live repo truth before changing anything.
- Use the approved seam authority at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-remaining-work-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-remaining-work-plan.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-remaining-work-tasks.md
- Treat the packet as one narrow decision-recording seam: explicitly record that Lane C (engine optional boundary freeze) is deferred, with rationale that engine's current public surface is the working boundary and Lane C can be activated later if Lane D's import plan indicates a narrower surface is needed.
- Stay inside Packet 6.C.1 only.

Hard rules:
- Do not implement, review, or fix Packet 6.C.1 work in the orchestration session yourself.
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, commit the implementation before any review round.
- Then spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them.
- Every fix subagent prompt must begin with `/goal ` and must explicitly use $incremental-implementation.
- After each accepted fix round that changes files, commit before dispatching the next fresh review subagent.
- Before every commit, run GitNexus detect-changes and verify the affected scope matches Packet 6.C.1 only.
- Stay inside Packet 6.C.1 scope.

Packet 6.C.1 scope:
- Record in the Lane C section of `docs/specs/handbook-engine-extraction-phase-6-remaining-work-tasks.md` that Lane C is deferred.
- Include rationale: engine's current public surface is the working boundary; if Lane D's import plan indicates a narrower surface is needed, Lane C can be activated at that time.
- Reference the spec's Lane C section for the activation condition.
- Expected files:
  - docs/specs/handbook-engine-extraction-phase-6-remaining-work-tasks.md (Lane C section)

Out of scope — do NOT touch:
- Activating Lane C (gathering evidence, writing a consumer contract, running verification wall)
- Any production code changes in any crate
- Lane B or Lane D work
- any CLI shell, compiler, doctor/setup, or pipeline surface changes
- publication or crates.io work
- Substrate consumption or broader integration implementation
- any new packet authoring beyond this approved packet prompt artifact

Implementation subagent prompt requirements:
- Begin with `/goal Land Phase 6 Remaining Work Packet 6.C.1: Record Lane C Deferral Decision`.
- Tell the subagent to use $incremental-implementation.
- Require live verification with:
  - `git status --short --branch`
  - `sed -n '/Lane C/,/Lane D/p' docs/specs/handbook-engine-extraction-phase-6-remaining-work-tasks.md`
  - `sed -n '/Lane C/,/Lane D/p' docs/specs/handbook-engine-extraction-phase-6-remaining-work-spec.md`
- Require the implementation to:
  - read the Lane C section in both the tasks doc and spec doc
  - record an explicit "deferred" decision with rationale in the tasks doc Lane C section
  - reference the activation condition from the spec
  - avoid widening into Lane C activation, Lane B, Lane D, or any production code changes
- Require the subagent to stop after Packet 6.C.1 acceptance is met and report touched files, decision recorded, and residual risks.

Review subagent prompt requirements:
- Begin with `/goal Review Phase 6 Remaining Work Packet 6.C.1: Record Lane C Deferral Decision`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to review the packet against the approved seam spec, plan, and tasks.
- Require special attention to:
  - whether the deferral decision is explicit and unambiguous
  - whether the rationale is consistent with the spec's Lane C section
  - whether the activation condition is referenced correctly
  - whether any scope leaked beyond Packet 6.C.1
- Require severity labels and explicit callouts if the decision is ambiguous, inconsistent with the spec, or if scope leaked.

Fix loop:
- If the review is clean, stop and report Packet 6.C.1 complete.
- If the review finds issues, spawn one fresh GPT-5.4 high fix subagent per review round using `$incremental-implementation`.
- The fix prompt must cite the exact review findings and require only the minimal Packet-6.C.1-bounded changes needed to close them.
- Commit accepted fixes before dispatching the next fresh review subagent.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after implementation if Packet 6.C.1 lands cleanly.
- Before each commit, run GitNexus detect-changes and confirm the affected scope matches Packet 6.C.1 only.
- Commit after each accepted fix round.
- Commit messages must describe the Lane C deferral decision clearly and standalone.

Stop conditions:
- Stop once Packet 6.C.1 is review-clean, committed, and the deferral decision is explicit with rationale.
- Stop and report blocked if the packet cannot close honestly without widening into Lane C activation, Lane B, Lane D, or broader integration implementation.
```

---

## Packet 6.D.1 Prompt — Write Import/Adoption Plan

```text
/goal Orchestrate Phase 6 Remaining Work Packet 6.D.1: Write Final Substrate Import/Adoption Plan in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 6.D.1 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-remaining-work-tasks.md.
- Verify live repo truth before changing anything.
- Use the approved seam authority at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-remaining-work-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-remaining-work-plan.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-remaining-work-tasks.md
- Treat the packet as one narrow planning-artifact seam: write a standalone import/adoption plan doc at `docs/specs/handbook-substrate-import-adoption-plan.md` that records import order, rationale, per-crate frozen boundary summaries, adapter/facade assessment, import verification gates, and Substrate-side constraints.
- This packet depends on Lane B being complete (Packets 6.B.1, 6.B.2, 6.B.3, and 6.B.4 all landed). Lane C is optional and not blocking for Lane D; if its deferral note exists, treat it as context rather than a prerequisite. Verify Lane B is complete before starting.
- Stay inside Packet 6.D.1 only.

Hard rules:
- Do not implement, review, or fix Packet 6.D.1 work in the orchestration session yourself.
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, commit the implementation before any review round.
- Then spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them.
- Every fix subagent prompt must begin with `/goal ` and must explicitly use $incremental-implementation.
- After each accepted fix round that changes files, commit before dispatching the next fresh review subagent.
- Before every commit, run GitNexus detect-changes and verify the affected scope matches Packet 6.D.1 only.
- Stay inside Packet 6.D.1 scope.

Packet 6.D.1 scope:
- Write a standalone doc at `docs/specs/handbook-substrate-import-adoption-plan.md` that records:
  - Import order: engine first (no intra-workspace deps), then pipeline (depends on engine), then flow (depends on engine)
  - Rationale for the phased order
  - Per-crate frozen boundary summary:
    - Engine: current public surface (Lane C deferred) — modules: `artifact_manifest`, `author`, `baseline_validation`, `canonical_artifacts`, `freshness`, plus `workspace_contract_version()` and `engine_contract_version()`
    - Pipeline: documented frozen subset from Lane A closeout — in-boundary modules: `pipeline`, `pipeline_capture`, `pipeline_compile`, `pipeline_handoff`, `pipeline_route`, `route_state`, plus `pipeline_contract_version()`
    - Flow: Lane B consumer contract — cleaned import surface from `resolver`, `budget`, `packet_result`, with typed semantics only where contract-approved and final shell copy out of boundary (reference the consumer contract doc)
  - Adapter/facade assessment (current evidence: none needed; record the assessment with evidence)
  - Import verification gate per phase (what checks Substrate must pass after importing each crate)
  - Substrate-side constraints (resolved from live repo inspection, 2026-06-17):
    - License field: add `license = "MIT"` to the three crate Cargo.toml files before import
    - Workspace integration: recommend workspace member pattern (path deps) vs external dep
    - YAML crate divergence: `serde_yaml_bw` (handbook) vs `serde_yaml` (substrate) — record keep-both or migrate decision
    - No feature flags needed; edition/resolver/sha2/libc/serde all compatible
- Reference the Lane B consumer contract doc, Lane A closeout archive, and the spec's substrate-side constraints table as provenance.
- Expected files:
  - docs/specs/handbook-substrate-import-adoption-plan.md (full document)

Out of scope — do NOT touch:
- Executing the actual Substrate import — this is a planning artifact only
- Any production code changes in any crate
- Lane B or Lane C work
- any CLI shell, compiler, doctor/setup, or pipeline surface changes
- publication or crates.io work
- any new packet authoring beyond this approved packet prompt artifact

Implementation subagent prompt requirements:
- Begin with `/goal Land Phase 6 Remaining Work Packet 6.D.1: Write Final Substrate Import/Adoption Plan`.
- Tell the subagent to use $incremental-implementation.
- Require live verification with:
  - `git status --short --branch`
  - `sed -n '1,60p' docs/specs/handbook-flow-import-boundary-consumer-contract.md`
  - `ls docs/specs/archive/phase-6-pipeline-boundary-cleanup/`
  - `sed -n '/Substrate-Side Constraints/,/Resolved Open Questions/p' docs/specs/handbook-engine-extraction-phase-6-remaining-work-spec.md`
  - `rg -n "pub (fn|struct|enum|mod)" crates/engine/src/lib.rs`
  - `rg -n "pub (fn|struct|enum|mod)" crates/pipeline/src/lib.rs`
- Require the implementation to:
  - verify Lane B (Packets 6.B.1–6.B.4) is complete before writing the plan; treat Lane C as optional context, not a blocker
  - record the import order with rationale (engine first, then pipeline, then flow)
  - record per-crate frozen boundary summaries consistent with live source and the Lane A/B closeout artifacts
  - record the adapter/facade assessment with evidence (current evidence: none needed)
  - record the import verification gate per phase (what checks Substrate must pass after importing each crate)
  - record all four Substrate-side constraints from the spec's constraints table
  - reference the Lane B consumer contract doc, Lane A closeout archive, and spec constraints table as provenance
  - ensure the document is internally consistent with live crate surfaces and frozen boundaries
  - avoid widening into actual import execution, Lane B/C work, or broader integration
- Require the subagent to stop after Packet 6.D.1 acceptance is met and report touched files, plan contents summary, verification run, and residual risks.

Review subagent prompt requirements:
- Begin with `/goal Review Phase 6 Remaining Work Packet 6.D.1: Write Final Substrate Import/Adoption Plan`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to review the packet against the approved seam spec, plan, tasks, Lane A closeout archive, and Lane B consumer contract.
- Require special attention to:
  - whether the import order and rationale are correct and consistent with dependency truth (engine has no intra-workspace deps; pipeline depends on engine; flow depends on engine)
  - whether per-crate frozen boundary summaries match live source and Lane A/B closeout artifacts exactly
  - whether the adapter/facade assessment is honest and evidence-backed
  - whether the import verification gates are concrete and testable
  - whether all four Substrate-side constraints are recorded with correct values from the spec
  - whether the document is internally consistent — no contradictions between crate summaries, import order, and verification gates
  - whether the plan stays a planning artifact and does not execute any import
  - whether any scope leaked beyond Packet 6.D.1
- Require severity labels and explicit callouts if the plan is incomplete, inaccurate, contradicts frozen boundaries, or if scope leaked.

Fix loop:
- If the review is clean, stop and report Packet 6.D.1 complete.
- If the review finds issues, spawn one fresh GPT-5.4 high fix subagent per review round using `$incremental-implementation`.
- The fix prompt must cite the exact review findings and require only the minimal Packet-6.D.1-bounded changes needed to close them.
- Commit accepted fixes before dispatching the next fresh review subagent.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after implementation if Packet 6.D.1 lands cleanly.
- Before each commit, run GitNexus detect-changes and confirm the affected scope matches Packet 6.D.1 only.
- Commit after each accepted fix round.
- Commit messages must describe the import/adoption plan authoring clearly and standalone.

Stop conditions:
- Stop once Packet 6.D.1 is review-clean, committed, and the import plan doc is internally consistent with live crate surfaces and frozen boundaries.
- Stop and report blocked if the packet cannot close honestly without widening into actual import execution, Lane B/C work, or broader integration implementation.
```

---

## Packet 6.D.2 Prompt — Human Review Gate

```text
/goal Orchestrate Phase 6 Remaining Work Packet 6.D.2: Human Review Gate for Final Substrate Import/Adoption Plan in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 6.D.2 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-remaining-work-tasks.md.
- Verify live repo truth before changing anything.
- Use the approved seam authority at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-remaining-work-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-remaining-work-plan.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-remaining-work-tasks.md
- Treat the packet as a human-review-gate seam: prepare the import plan for human review, present it to the user for sign-off, and address any review feedback the user provides.
- This packet depends on Packet 6.D.1 being complete (import plan doc written and review-clean). Verify Packet 6.D.1 is landed before starting.
- Stay inside Packet 6.D.2 only.

Hard rules:
- Do not implement, review, or fix Packet 6.D.2 work in the orchestration session yourself.
- Spawn a fresh GPT-5.4 subagent on high for the preparation phase.
- The preparation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After preparation completes, commit any preparation changes before presenting to the user.
- Then spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If the automated review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them.
- Every fix subagent prompt must begin with `/goal ` and must explicitly use $incremental-implementation.
- After each accepted fix round that changes files, commit before dispatching the next fresh review subagent.
- After the automated review is clean, the orchestrator must present the import plan to the user for human sign-off.
- If the human reviewer provides feedback, spawn a fresh GPT-5.4 subagent on high to address the feedback using $incremental-implementation.
- Commit after each accepted feedback-fix round.
- Before every commit, run GitNexus detect-changes and verify the affected scope matches Packet 6.D.2 only.
- Stay inside Packet 6.D.2 scope.

Packet 6.D.2 scope:
- Verify Packet 6.D.1 is complete (import plan doc exists and is review-clean).
- Run a fresh automated review of the import plan against:
  - The three crate surfaces (engine, pipeline, flow)
  - The frozen boundaries (Lane A closeout, Lane B consumer contract)
  - The root plan's migration gate
  - The spec's substrate-side constraints
- Present the import plan to the user for human sign-off.
- Record human sign-off (or feedback) in the Packet 6.D.2 completion notes section of `docs/specs/handbook-engine-extraction-phase-6-remaining-work-tasks.md`.
- If the human reviewer provides feedback, address it with a fix subagent and re-present.
- Expected files:
  - docs/specs/handbook-engine-extraction-phase-6-remaining-work-tasks.md (completion note)
  - docs/specs/handbook-substrate-import-adoption-plan.md (if feedback requires edits)

Out of scope — do NOT touch:
- Executing the actual Substrate import
- Any production code changes in any crate
- Lane B or Lane C work
- Reopening Lane A
- any CLI shell, compiler, doctor/setup, or pipeline surface changes
- publication or crates.io work
- any new packet authoring beyond this approved packet prompt artifact

Preparation subagent prompt requirements:
- Begin with `/goal Prepare Phase 6 Remaining Work Packet 6.D.2: Human Review Gate for Final Substrate Import/Adoption Plan`.
- Tell the subagent to use $incremental-implementation.
- Require live verification with:
  - `git status --short --branch`
  - `sed -n '1,200p' docs/specs/handbook-substrate-import-adoption-plan.md`
  - `sed -n '/Lane A status/,/Tech Stack/p' docs/specs/handbook-engine-extraction-phase-6-remaining-work-spec.md`
  - `sed -n '/Substrate-Side Constraints/,/Resolved Open Questions/p' docs/specs/handbook-engine-extraction-phase-6-remaining-work-spec.md`
- Require the preparation to:
  - verify Packet 6.D.1 is complete (import plan doc exists)
  - verify the import plan is internally consistent with the three crate surfaces, frozen boundaries, and substrate-side constraints
  - prepare a concise review summary for the human reviewer highlighting: import order, per-crate boundary posture, adapter assessment, verification gates, and substrate-side constraints
  - flag any inconsistencies or gaps found during preparation for the human reviewer's attention
  - avoid widening into actual import execution, Lane B/C work, or broader integration
- Require the subagent to stop after preparation is complete and report the review summary, any flagged issues, and residual risks.

Review subagent prompt requirements:
- Begin with `/goal Review Phase 6 Remaining Work Packet 6.D.2: Human Review Gate for Final Substrate Import/Adoption Plan`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to review the import plan against:
  - the three crate surfaces (engine, pipeline, flow) — verify boundary summaries match live source
  - the frozen boundaries (Lane A closeout archive, Lane B consumer contract)
  - the root plan's migration gate
  - the spec's substrate-side constraints table
- Require special attention to:
  - whether the plan is consistent with live crate surfaces and frozen boundaries
  - whether the import order and rationale are correct
  - whether the substrate-side constraints are accurately recorded
  - whether the plan stays a planning artifact and does not execute any import
  - whether any scope leaked beyond Packet 6.D.2
- Require severity labels and explicit callouts if the plan contradicts frozen boundaries, has missing constraints, or if scope leaked.

Human review gate:
- After the automated review is clean, the orchestrator must present the import plan to the user.
- The presentation must include: the plan doc path, a concise summary of import order/boundaries/gates/constraints, and any flagged issues from the automated review.
- Wait for the user's sign-off or feedback.
- If the user provides feedback, spawn a fresh GPT-5.4 high fix subagent using `$incremental-implementation` to address the feedback.
- The fix prompt must cite the exact human feedback and require only the minimal Packet-6.D.2-bounded changes needed to close it.
- Commit accepted feedback-fixes.
- Re-present the updated plan to the user for final sign-off.
- Record the human sign-off in the Packet 6.D.2 completion notes section of `docs/specs/handbook-engine-extraction-phase-6-remaining-work-tasks.md`.

Fix loop (automated review):
- If the automated review is clean, proceed to the human review gate.
- If the automated review finds issues, spawn one fresh GPT-5.4 high fix subagent per review round using `$incremental-implementation`.
- Commit accepted fixes before dispatching the next fresh review subagent.
- Re-run a fresh review subagent after fixes.

Fix loop (human feedback):
- If the human reviewer provides feedback, spawn one fresh GPT-5.4 high fix subagent using `$incremental-implementation`.
- Commit accepted fixes.
- Re-present the updated plan to the user.

Commit policy:
- Commit once after preparation if Packet 6.D.2 preparation lands cleanly.
- Before each commit, run GitNexus detect-changes and confirm the affected scope matches Packet 6.D.2 only.
- Commit after each accepted fix round (automated or human-feedback).
- Commit messages must describe the human review gate preparation, feedback addressing, or sign-off recording clearly and standalone.

Stop conditions:
- Stop once Packet 6.D.2 is complete: human sign-off is recorded in the tasks doc and any feedback has been addressed and committed.
- Stop and report blocked if the human reviewer rejects the plan and the feedback cannot be addressed within Packet 6.D.2 scope (i.e., the feedback requires reopening Lane A/B/C, production code changes, or actual import execution).
```
