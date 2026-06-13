# Handbook Engine Extraction Phase 2 Slice 4 Packet Prompts

Task source: [handbook-engine-extraction-phase-2-slice-4-orchestration-target-parameterization-closeout-tasks.md](./handbook-engine-extraction-phase-2-slice-4-orchestration-target-parameterization-closeout-tasks.md)
Spec source: [handbook-engine-extraction-phase-2-slice-4-orchestration-target-parameterization-closeout-spec.md](./handbook-engine-extraction-phase-2-slice-4-orchestration-target-parameterization-closeout-spec.md)
Plan source: [handbook-engine-extraction-phase-2-slice-4-orchestration-target-parameterization-closeout-plan.md](./handbook-engine-extraction-phase-2-slice-4-orchestration-target-parameterization-closeout-plan.md)

These prompts are ready to paste into fresh orchestration sessions. Each one starts in `/goal`, requires fresh `GPT-5.4` `high` subagents, uses `$incremental-implementation` for implementation/fix rounds, uses `$code-review-and-quality` for review rounds, preserves commit boundaries between implementation, review, and fix work, and requires GitNexus impact analysis before production-symbol edits plus GitNexus detect-changes before each commit.

Do not advance to the next packet until the current packet is review-clean and committed.

## Packet 2.4.1 Prompt

```text
/goal Orchestrate Phase 2 Slice 4 Packet 2.4.1: Catalog-Backed Pipeline And Stage Target Closeout in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 2.4.1 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-2-slice-4-orchestration-target-parameterization-closeout-tasks.md.
- Verify live repo truth before changing anything, including that the landed Phase 2 Slice 2.1, 2.2, and 2.3 authority sets already exist and that Slice 2.4 is the closeout seam for remaining target hardcoding.
- Use the slice spec and plan at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-2-slice-4-orchestration-target-parameterization-closeout-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-2-slice-4-orchestration-target-parameterization-closeout-plan.md
- Use the closeout seam map at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-closeout-four-set-map.md
- Do not start Packet 2.4.2, 2.4.3, or 2.4.4.

Hard rules:
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them.
- Every fix subagent prompt must begin with `/goal ` and must explicitly use $incremental-implementation.
- Before editing any function, method, or other production symbol, run GitNexus impact analysis on the touched symbol(s) and report the blast radius. If GitNexus reports a stale index, refresh it first.
- Before every commit, run GitNexus detect-changes and verify the affected scope matches Packet 2.4.1 only.
- Commit changes after the implementation lands, and commit again after each accepted fix round that changes files.
- Stay inside Packet 2.4.1 scope.

Packet 2.4.1 scope:
- Remove singleton pipeline/stage ownership from the shared runtime target owner.
- Keep compile/capture/provenance adopter behavior stable while consuming that shared target owner.
- Expected files:
  - crates/pipeline/src/pipeline.rs
  - crates/pipeline/src/pipeline_compile.rs
  - crates/pipeline/src/pipeline_capture.rs
  - crates/pipeline/src/stage_10_feature_spec_provenance.rs
  - crates/pipeline/tests/pipeline_catalog.rs
  - optionally crates/pipeline/tests/pipeline_compile.rs
  - optionally crates/pipeline/tests/pipeline_capture.rs

Out of scope:
- bounded default-consumer ownership work from Packet 2.4.2
- CLI help and producer-command cleanup from Packet 2.4.3
- any generic multi-consumer or `core/consumers/**` platform work
- any template/library resolver work
- any Set 3 caller-rewire/compiler-narrowing work
- any Set 4 CLI shell redesign

Implementation subagent prompt requirements:
- Begin with `/goal Land Phase 2 Slice 4 Packet 2.4.1: Catalog-Backed Pipeline And Stage Target Closeout`.
- Tell the subagent to use $incremental-implementation.
- Require live verification that the Slice 2.1 contract and Slice 2.2/2.3 landings already exist before editing.
- Require the subagent to keep this packet bounded to pipeline/stage target ownership and to explicitly preserve:
  - declarative pipeline truth from `core/pipelines/**`
  - declarative stage truth from `core/stages/**`
  - the current supported wedge remaining unchanged
  - compile/capture/provenance-specific refusal and validation posture remaining local in their adopter modules
  - consumer ownership remaining deferred to Packet 2.4.2
- Require the subagent to remove singleton pipeline/stage owner constants as the de facto runtime truth rather than merely moving the same literals to a new Rust owner.
- Require GitNexus impact analysis before editing touched production symbols and require the subagent to report impacted callers/processes.
- Require targeted verification with:
  - `rg -n "SUPPORTED_PIPELINE_TARGET_ID|SUPPORTED_COMPILE_STAGE_TARGET_ID|SUPPORTED_CAPTURE_STAGE_TARGET_IDS|pipeline\\.foundation_inputs|stage\\.10_feature_spec" crates/pipeline/src/pipeline.rs crates/pipeline/src/pipeline_compile.rs crates/pipeline/src/pipeline_capture.rs crates/pipeline/src/stage_10_feature_spec_provenance.rs`
  - `cargo test -p handbook-pipeline --test pipeline_catalog`
  - `cargo test -p handbook-pipeline --test pipeline_compile`
  - `cargo test -p handbook-pipeline --test pipeline_capture`
- Require the subagent to stop after Packet 2.4.1 acceptance is met and report touched files, impact-analysis results, verification run, remaining hardcoded sites if any, and residual risks.

Review subagent prompt requirements:
- Begin with `/goal Review Phase 2 Slice 4 Packet 2.4.1: Catalog-Backed Pipeline And Stage Target Closeout`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to review the packet against the Slice 2.4 spec, plan, tasks, the prior Phase 2 authority set, and the verification evidence.
- Require special attention to:
  - whether pipeline/stage truth is now catalog-backed rather than singleton-constant-owned
  - whether compile/capture/provenance still preserve the current wedge and local refusal posture
  - whether consumer or CLI cleanup leaked in early
  - whether the implementation simply renamed duplicated literals instead of actually removing ownership duplication
- Require severity labels and explicit callouts if pipeline/stage ownership is still duplicated, if the current wedge drifted, or if scope widened beyond Packet 2.4.1.

Fix loop:
- If the review is clean, stop and report Packet 2.4.1 complete.
- If the review finds issues, spawn one fresh GPT-5.4 high fix subagent per review round using `$incremental-implementation`.
- The fix prompt must cite the exact review findings and require only the minimal Packet-2.4.1-bounded changes needed to close them.
- Re-run GitNexus impact analysis if new production symbols are touched.
- Re-run the bounded verification after each fix round.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after implementation if Packet 2.4.1 lands cleanly.
- Before each commit, run GitNexus detect-changes and confirm the affected scope matches Packet 2.4.1 only.
- Commit after each accepted fix round.
- Commit messages must describe the Packet 2.4.1 change clearly and standalone.

Stop conditions:
- Stop once Packet 2.4.1 is review-clean and committed.
- Stop and report blocked if Packet 2.4.1 cannot be completed without widening into Packet 2.4.2+, changing the approved Slice 2.4 spec/plan/tasks, or adding new supported targets.
```

## Packet 2.4.2 Prompt

```text
/goal Orchestrate Phase 2 Slice 4 Packet 2.4.2: Bounded Default-Consumer Ownership in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 2.4.2 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-2-slice-4-orchestration-target-parameterization-closeout-tasks.md.
- Assume Packet 2.4.1 is already landed, review-clean, and committed; verify live repo truth before changing anything.
- Use the slice spec and plan at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-2-slice-4-orchestration-target-parameterization-closeout-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-2-slice-4-orchestration-target-parameterization-closeout-plan.md
- Use the closeout seam map at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-closeout-four-set-map.md
- Do not reopen Packet 2.4.1 except where live repo truth proves a tiny corrective adjustment is strictly required for Packet 2.4.2 to land correctly.
- Do not start Packet 2.4.3 or 2.4.4.

Hard rules:
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them with `$incremental-implementation`.
- Before editing any function, method, or other production symbol, run GitNexus impact analysis on the touched symbol(s) and report the blast radius. If GitNexus reports a stale index, refresh it first.
- Before every commit, run GitNexus detect-changes and verify the affected scope matches Packet 2.4.2 only.
- Commit changes after the implementation lands, and commit again after each accepted fix round.
- Stay inside Packet 2.4.2 scope.

Packet 2.4.2 scope:
- Centralize the bounded current consumer in one code-owned validated default owner.
- Keep handoff validation and bundle emission behavior stable after consumer de-hardcoding.
- Expected files:
  - crates/pipeline/src/pipeline.rs
  - crates/pipeline/src/pipeline_handoff.rs
  - crates/pipeline/tests/pipeline_handoff.rs
  - crates/cli/src/pipeline.rs
  - optionally crates/cli/tests/cli_surface.rs

Out of scope:
- reopening pipeline/stage owner closeout beyond tiny corrective carry-forward from Packet 2.4.1
- CLI help wording and producer-command cleanup beyond what is strictly necessary to keep handoff behavior stable
- any new consumer, pipeline, or stage support
- any `core/consumers/**` tree or user-editable consumer configuration
- any Set 4 CLI shell redesign

Implementation subagent prompt requirements:
- Begin with `/goal Land Phase 2 Slice 4 Packet 2.4.2: Bounded Default-Consumer Ownership`.
- Tell the subagent to use $incremental-implementation.
- Require live verification that Packet 2.4.1 already landed and that shared pipeline/stage target ownership exists before editing.
- Require the subagent to keep this packet bounded to the current consumer owner and to explicitly preserve:
  - `feature-slice-decomposer` as the only supported consumer during this closeout
  - code-owned and validated default-consumer ownership
  - handoff-specific provenance matching, trust-class, bundle layout, and refusal posture remaining local in `pipeline_handoff.rs`
  - no generic consumer selection platform
  - only the minimal CLI adapter changes needed to keep bounded handoff behavior coherent
- Require the subagent to remove separate consumer/stage ownership literals from `pipeline_handoff.rs` if the shared owner can provide the same contract.
- Require GitNexus impact analysis before editing touched production symbols and require the subagent to report impacted callers/processes.
- Require targeted verification with:
  - `rg -n "SUPPORTED_CONSUMER_TARGET_ID|SUPPORTED_CONSUMER_ID|feature-slice-decomposer" crates/pipeline/src/pipeline.rs crates/pipeline/src/pipeline_handoff.rs crates/cli/src/pipeline.rs`
  - `cargo test -p handbook-pipeline --test pipeline_handoff`
  - `cargo test -p handbook-cli --test cli_surface`
- Require the subagent to stop after Packet 2.4.2 acceptance is met and report touched files, impact-analysis results, verification run, any remaining bounded-consumer literals with justification, and residual risks.

Review subagent prompt requirements:
- Begin with `/goal Review Phase 2 Slice 4 Packet 2.4.2: Bounded Default-Consumer Ownership`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to review the packet against the Slice 2.4 spec, plan, tasks, the landed Packet 2.4.1 state, and the verification evidence.
- Require special attention to:
  - whether the bounded current consumer is now owned in one validated location
  - whether handoff behavior stayed bounded and stable
  - whether any generic consumer platform or new support surface leaked in
  - whether CLI adapter changes stayed minimal instead of turning into Packet 2.4.3/Set 4 work
- Require severity labels and explicit callouts if consumer ownership is still duplicated, if handoff behavior regressed, or if scope widened beyond Packet 2.4.2.

Fix loop:
- If the review is clean, stop and report Packet 2.4.2 complete.
- If the review finds issues, spawn a fresh GPT-5.4 high fix subagent using `$incremental-implementation`.
- Keep fixes minimal and Packet-2.4.2-bounded.
- Re-run GitNexus impact analysis if new production symbols are touched.
- Re-run the bounded verification after each fix round.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after Packet 2.4.2 implementation lands cleanly.
- Before each commit, run GitNexus detect-changes and confirm the affected scope matches Packet 2.4.2 only.
- Commit after each accepted fix round.
- Commit messages must describe the Packet 2.4.2 change clearly and standalone.

Stop conditions:
- Stop once Packet 2.4.2 is review-clean and committed.
- Stop and report blocked if Packet 2.4.2 requires widening into Packet 2.4.3+, adding a new consumer surface, or changing the approved Slice 2.4 spec/plan/tasks.
```

## Packet 2.4.3 Prompt

```text
/goal Orchestrate Phase 2 Slice 4 Packet 2.4.3: CLI Help, Recovery, And Producer-Command Alignment in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 2.4.3 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-2-slice-4-orchestration-target-parameterization-closeout-tasks.md.
- Assume Packets 2.4.1 and 2.4.2 are already landed, review-clean, and committed; verify live repo truth before changing anything.
- Use the slice spec and plan at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-2-slice-4-orchestration-target-parameterization-closeout-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-2-slice-4-orchestration-target-parameterization-closeout-plan.md
- Use the closeout seam map at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-closeout-four-set-map.md
- Do not reopen Packets 2.4.1 or 2.4.2 except where live repo truth proves a tiny corrective adjustment is strictly required for Packet 2.4.3 to land correctly.
- Do not start Packet 2.4.4.

Hard rules:
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them with `$incremental-implementation`.
- Before editing any function, method, or other production symbol, run GitNexus impact analysis on the touched symbol(s) and report the blast radius. If GitNexus reports a stale index, refresh it first.
- Before every commit, run GitNexus detect-changes and verify the affected scope matches Packet 2.4.3 only.
- Commit changes after the implementation lands, and commit again after each accepted fix round.
- Stay inside Packet 2.4.3 scope.

Packet 2.4.3 scope:
- Source supported-target examples from shared bounded target ownership.
- Remove independent CLI ownership of help, recovery, and producer-command target literals.
- Expected files:
  - crates/cli/src/main.rs
  - crates/cli/src/pipeline.rs
  - crates/pipeline/src/pipeline_capture.rs
  - crates/pipeline/src/pipeline_handoff.rs
  - crates/cli/tests/cli_surface.rs
  - optionally crates/pipeline/tests/pipeline_capture.rs
  - optionally crates/pipeline/tests/pipeline_handoff.rs

Out of scope:
- broad CLI copy rewrite, shell restructuring, or Set 4 CLI shell closeout work
- expanding the supported wedge
- reopening pipeline/stage or consumer ownership except for tiny corrective edits strictly required to source shared target posture
- any new consumer-platform configuration

Implementation subagent prompt requirements:
- Begin with `/goal Land Phase 2 Slice 4 Packet 2.4.3: CLI Help, Recovery, And Producer-Command Alignment`.
- Tell the subagent to use $incremental-implementation.
- Require live verification that Packets 2.4.1 and 2.4.2 already landed and that shared bounded target ownership exists before editing.
- Require the subagent to keep this packet bounded to CLI/help/recovery/producer-command alignment and to explicitly preserve:
  - the current supported wedge and operator workflow
  - capture- and handoff-specific refusal wording remaining local where behavior-specific wording still belongs
  - CLI/product-shell wording changes staying as small as possible
  - no broad Set 4 shell cleanup
- Require `crates/cli/src/main.rs` and `crates/cli/src/pipeline.rs` to stop acting as independent owners of `pipeline.foundation_inputs`, `stage.10_feature_spec`, and `feature-slice-decomposer` examples if shared bounded target ownership can provide those values.
- Require GitNexus impact analysis before editing touched production symbols and require the subagent to report impacted callers/processes.
- Require targeted verification with:
  - `rg -n "pipeline\\.foundation_inputs|stage\\.10_feature_spec|feature-slice-decomposer" crates/cli/src/main.rs crates/cli/src/pipeline.rs crates/pipeline/src/pipeline_capture.rs crates/pipeline/src/pipeline_handoff.rs`
  - `cargo test -p handbook-cli --test cli_surface`
  - `cargo test -p handbook-pipeline --test pipeline_capture`
  - `cargo test -p handbook-pipeline --test pipeline_handoff`
- Require the subagent to stop after Packet 2.4.3 acceptance is met and report touched files, impact-analysis results, verification run, any remaining product-surface literals with justification, and residual risks.

Review subagent prompt requirements:
- Begin with `/goal Review Phase 2 Slice 4 Packet 2.4.3: CLI Help, Recovery, And Producer-Command Alignment`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to review the packet against the Slice 2.4 spec, plan, tasks, the landed Packet 2.4.1/2.4.2 state, and the verification evidence.
- Require special attention to:
  - whether CLI/help and producer-command surfaces now consume shared bounded target ownership instead of re-owning literals
  - whether capture and handoff recovery posture stayed correct
  - whether scope drifted into general CLI redesign
  - whether any supported-wedge expansion leaked in
- Require severity labels and explicit callouts if independent CLI ownership remains, if recovery/help behavior regressed, or if scope widened beyond Packet 2.4.3.

Fix loop:
- If the review is clean, stop and report Packet 2.4.3 complete.
- If the review finds issues, spawn a fresh GPT-5.4 high fix subagent using `$incremental-implementation`.
- Keep fixes minimal and Packet-2.4.3-bounded.
- Re-run GitNexus impact analysis if new production symbols are touched.
- Re-run the bounded verification after each fix round.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after Packet 2.4.3 implementation lands cleanly.
- Before each commit, run GitNexus detect-changes and confirm the affected scope matches Packet 2.4.3 only.
- Commit after each accepted fix round.
- Commit messages must describe the Packet 2.4.3 change clearly and standalone.

Stop conditions:
- Stop once Packet 2.4.3 is review-clean and committed.
- Stop and report blocked if Packet 2.4.3 requires widening into Packet 2.4.4 only to justify unrelated shell cleanup, expanding the supported wedge, or changing the approved Slice 2.4 spec/plan/tasks.
```

## Packet 2.4.4 Prompt

```text
/goal Orchestrate Phase 2 Slice 4 Packet 2.4.4: Final Closeout Proof in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 2.4.4 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-2-slice-4-orchestration-target-parameterization-closeout-tasks.md.
- Assume Packets 2.4.1, 2.4.2, and 2.4.3 are already landed, review-clean, and committed; verify live repo truth before changing anything.
- Use the slice spec and plan at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-2-slice-4-orchestration-target-parameterization-closeout-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-2-slice-4-orchestration-target-parameterization-closeout-plan.md
- Use the closeout seam map at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-closeout-four-set-map.md
- Do not reopen earlier packets except where live repo truth proves a tiny closeout fix is strictly required to make Packet 2.4.4 honestly green.

Hard rules:
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them with `$incremental-implementation`.
- Before editing any function, method, or other production symbol, run GitNexus impact analysis on the touched symbol(s) and report the blast radius. If GitNexus reports a stale index, refresh it first.
- Before every commit, run GitNexus detect-changes and verify the affected scope matches Packet 2.4.4 only.
- Commit changes after the implementation lands, and commit again after each accepted fix round.
- If the packet lands with no repo changes, do not create an empty commit; report that the packet was a no-op verification-only closeout instead.
- Stay inside Packet 2.4.4 scope.

Packet 2.4.4 scope:
- Re-run the full closeout verification wall.
- Preserve explicit deferrals for multi-consumer platforms, new consumer catalogs, and Set 4 CLI shell cleanup.
- Make the final closeout proof durable if a small repo-local note or handoff is needed to record the green wall and explicit deferrals.
- Expected files:
  - crates/pipeline/tests/pipeline_catalog.rs
  - crates/pipeline/tests/pipeline_compile.rs
  - crates/pipeline/tests/pipeline_capture.rs
  - crates/pipeline/tests/pipeline_handoff.rs
  - crates/cli/tests/cli_surface.rs
  - plus the smallest durable repo-local closeout note only if the explicit-deferral ledger is not already sufficiently preserved by existing authority docs

Out of scope:
- new feature work
- reopening earlier packets beyond the smallest closeout fix strictly required by failing verification
- Set 3 caller rewires or Set 4 CLI shell redesign
- any new supported target, consumer platform, or schema surface

Implementation subagent prompt requirements:
- Begin with `/goal Land Phase 2 Slice 4 Packet 2.4.4: Final Closeout Proof`.
- Tell the subagent to use $incremental-implementation.
- Require live verification that Packets 2.4.1-2.4.3 already landed before editing.
- Require the subagent to keep this packet bounded to final proof and explicit deferral capture and to explicitly preserve:
  - the currently supported wedge unchanged
  - explicit deferral of generalized multi-consumer/customizable-consumer work
  - explicit deferral of new consumer catalogs and Set 4 CLI shell cleanup
  - no opportunistic cleanup beyond the smallest fix needed to get the verification wall green
- Require the subagent to run the full closeout verification wall:
  - `cargo test -p handbook-pipeline --test pipeline_catalog`
  - `cargo test -p handbook-pipeline --test pipeline_compile`
  - `cargo test -p handbook-pipeline --test pipeline_capture`
  - `cargo test -p handbook-pipeline --test pipeline_handoff`
  - `cargo test -p handbook-cli --test cli_surface`
  - `cargo test --workspace`
- Require the subagent to run the remaining-literal evidence sweep:
  - `rg -n "SUPPORTED_PIPELINE_TARGET_ID|SUPPORTED_COMPILE_STAGE_TARGET_ID|SUPPORTED_CAPTURE_STAGE_TARGET_IDS|SUPPORTED_CONSUMER_TARGET_ID|SUPPORTED_CONSUMER_ID|SUPPORTED_STAGE_ID|pipeline\\.foundation_inputs|stage\\.10_feature_spec|feature-slice-decomposer" crates/pipeline/src crates/cli/src`
- If the wall fails, require only the smallest Packet-2.4.4-bounded fix needed to make the closeout honest.
- If explicit deferrals are not already durable enough in repo-local artifacts, require the subagent to create the smallest appropriate repo-local closeout note or handoff and include the green verification wall plus explicit deferrals.
- Require GitNexus impact analysis before any production-symbol fix and require the subagent to report impacted callers/processes.
- Require the subagent to stop after Packet 2.4.4 acceptance is met and report touched files, impact-analysis results if any, verification run, remaining literals if any, whether a durable closeout note was added, and residual risks.

Review subagent prompt requirements:
- Begin with `/goal Review Phase 2 Slice 4 Packet 2.4.4: Final Closeout Proof`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to review the packet against the Slice 2.4 spec, plan, tasks, the landed Packet 2.4.1-2.4.3 state, and the full verification evidence.
- Require special attention to:
  - whether the verification wall is actually green
  - whether explicit deferred scope is still honest and durable
  - whether any residual hardcoded ownership still blocks calling Set 2 closed
  - whether the packet snuck in unrelated cleanup under the guise of final proof
- Require severity labels and explicit callouts if the wall is incomplete, if deferrals are ambiguous, or if earlier-packet scope was reopened without necessity.

Fix loop:
- If the review is clean, stop and report Packet 2.4.4 complete.
- If the review finds issues, spawn a fresh GPT-5.4 high fix subagent using `$incremental-implementation`.
- Keep fixes minimal and Packet-2.4.4-bounded.
- Re-run GitNexus impact analysis if new production symbols are touched.
- Re-run the full verification wall after each fix round.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after Packet 2.4.4 implementation lands cleanly if files changed.
- Before each commit, run GitNexus detect-changes and confirm the affected scope matches Packet 2.4.4 only.
- Commit after each accepted fix round.
- Do not create an empty commit for a verification-only no-op closeout.
- Commit messages must describe the Packet 2.4.4 closeout change clearly and standalone.

Stop conditions:
- Stop once Packet 2.4.4 is review-clean and committed, or review-clean and honestly no-op with no file changes required.
- Stop and report blocked if Packet 2.4.4 uncovers scope that actually belongs to Set 3 or Set 4, requires broader new feature work, or requires changing the approved Slice 2.4 spec/plan/tasks.
```
