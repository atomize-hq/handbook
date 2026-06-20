# Handbook Substrate Parameterization — Set 2 Packet Prompts

Task source: [handbook-substrate-parameterization-set-2-flow-canonical-layout-injection-tasks.md](./handbook-substrate-parameterization-set-2-flow-canonical-layout-injection-tasks.md)
Spec source: [handbook-substrate-parameterization-set-2-flow-canonical-layout-injection-spec.md](./handbook-substrate-parameterization-set-2-flow-canonical-layout-injection-spec.md)
Plan source: [handbook-substrate-parameterization-set-2-flow-canonical-layout-injection-plan.md](./handbook-substrate-parameterization-set-2-flow-canonical-layout-injection-plan.md)

These prompts are ready to paste into fresh orchestration sessions. Each one starts in `/goal`, requires fresh `GPT-5.4` `high` subagents, uses `$incremental-implementation` for implementation and fix rounds, uses `$code-review-and-quality` for review rounds, preserves commit boundaries between implementation, review, and fix work, and keeps execution bounded to Set 2 only.

This set is approved only for the `handbook-flow` canonical-layout injection seam captured in this artifact: Packet 2.1 (flow public API contract shape), Packet 2.2 (resolver adoption and test coverage), and Packet 2.3 (final set proof). Do not reopen Set 1, do not start Set 3, do not widen into CLI/compiler cleanup, and do not execute actual Substrate import work. If a packet cannot land honestly without widening beyond that seam, stop and report the blocker instead of silently expanding scope.

The orchestration session must not implement, review, or fix packet work itself first. It must delegate implementation, review, and any fix rounds to fresh subagents exactly as directed below.

---

## Packet 2.1 Prompt — Flow Public API Contract Shape

```text
/goal Orchestrate Set 2 Packet 2.1: Flow Public API Contract Shape in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 2.1 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-substrate-parameterization-set-2-flow-canonical-layout-injection-tasks.md.
- Verify live repo truth before changing anything.
- Use the approved set authority at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-substrate-parameterization-set-2-flow-canonical-layout-injection-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-substrate-parameterization-set-2-flow-canonical-layout-injection-plan.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-substrate-parameterization-set-2-flow-canonical-layout-injection-tasks.md
- Treat the packet as one narrow seam: expose a supported public flow-facing entrypoint that consumes the engine-owned canonical layout contract while keeping the existing default `resolve(...)` path explicit.
- Stay inside Packet 2.1 only.

Hard rules:
- Do not implement, review, or fix Packet 2.1 work in the orchestration session yourself.
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, commit the implementation before any review round.
- Then spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them.
- Every fix subagent prompt must begin with `/goal ` and must explicitly use $incremental-implementation.
- After each accepted fix round that changes files, commit before dispatching the next fresh review subagent.
- Preserve unrelated local edits, including any dirt already present in `AGENTS.md` and `CLAUDE.md`.
- Before editing any production symbol in `crates/flow/src/resolver.rs` or `crates/flow/src/lib.rs`, run GitNexus impact analysis first and report the blast radius. If GitNexus says the index is stale, run `npx gitnexus analyze` first. If the blast radius is HIGH or CRITICAL, stop and report it before editing.
- Before every commit, run GitNexus detect-changes and verify the affected scope matches Packet 2.1 only.
- Stay inside Packet 2.1 scope.

Packet 2.1 scope:
- Introduce the supported public contract-aware flow resolver entrypoint.
- Keep `resolve(...)` as the explicit handbook-product default wrapper.
- Consume the engine-owned `CanonicalLayoutContract` directly; do not invent a second layout model and do not create a flow-owned alias.
- Update only the minimal public surface wiring needed for Packet 2.1 to be coherent.
- Expected files:
  - crates/flow/src/resolver.rs
  - crates/flow/src/lib.rs

Out of scope — do NOT touch:
- Packet 2.2 resolver adoption beyond the minimal public seam needed for Packet 2.1 to compile coherently
- Packet 2.3 proof work
- Set 1 or Set 3 work
- CLI/compiler/product-shell cleanup
- actual Substrate import execution
- a `ResolveRequest` redesign unless the packet is blocked and you stop to report it

Implementation subagent prompt requirements:
- Begin with `/goal Land Set 2 Packet 2.1: Flow Public API Contract Shape`.
- Tell the subagent to use $incremental-implementation.
- Require live verification with:
  - `git status --short --branch`
  - `sed -n '1,120p' crates/flow/src/lib.rs`
  - `sed -n '400,470p' crates/flow/src/resolver.rs`
  - `sed -n '1,120p' crates/engine/src/lib.rs`
  - `sed -n '1,260p' crates/engine/src/canonical_paths.rs`
  - `sed -n '200,280p' crates/engine/src/canonical_artifacts.rs`
  - `cargo test -p handbook-flow --test resolver_core`
- Require the implementation to:
  - run GitNexus impact analysis before editing production symbols and report the blast radius
  - introduce the public flow-facing contract seam using the engine-owned canonical layout contract directly
  - keep `resolve(...)` as an explicit default wrapper instead of the only supported path
  - avoid inventing a second layout model or widening into refusal/blocker wording cleanup that belongs to Packet 2.2 or Set 3
  - stop after Packet 2.1 acceptance is met and report touched files, impact-analysis results, verification run, and residual risks

Review subagent prompt requirements:
- Begin with `/goal Review Set 2 Packet 2.1: Flow Public API Contract Shape`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to check:
  - whether the public flow contract shape is coherent and minimal
  - whether engine clearly remains the canonical layout contract owner
  - whether the default `resolve(...)` path stayed explicit rather than hidden fallback behavior
  - whether any scope leaked into Packet 2.2 resolver-adoption work or Set 3 wording cleanup
- Require severity labels and explicit callouts for scope drift, second-model risk, or false default-wrapper behavior.

Fix loop:
- If the review is clean, stop and report Packet 2.1 complete.
- If the review finds issues, spawn one fresh GPT-5.4 high fix subagent per review round using `$incremental-implementation`.
- The fix prompt must cite the exact review findings and require only the minimal Packet-2.1-bounded changes needed to close them.
- Commit accepted fixes before dispatching the next fresh review subagent.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after implementation if Packet 2.1 lands cleanly.
- Before each commit, run GitNexus detect-changes and confirm the affected scope matches Packet 2.1 only.
- Commit after each accepted fix round.
- Commit messages must describe the flow contract-shape work clearly and standalone.

Stop conditions:
- Stop once Packet 2.1 is review-clean, committed, and the supported public flow-facing contract seam exists with engine still owning the layout contract.
- Stop and report blocked if the packet cannot close honestly without redesigning `ResolveRequest`, inventing a second layout model, or widening into later packets.
```

---

## Packet 2.2 Prompt — Resolver Adoption And Test Coverage

```text
/goal Orchestrate Set 2 Packet 2.2: Resolver Adoption And Test Coverage in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 2.2 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-substrate-parameterization-set-2-flow-canonical-layout-injection-tasks.md.
- Verify live repo truth before changing anything.
- Use the approved set authority at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-substrate-parameterization-set-2-flow-canonical-layout-injection-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-substrate-parameterization-set-2-flow-canonical-layout-injection-plan.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-substrate-parameterization-set-2-flow-canonical-layout-injection-tasks.md
- Treat the packet as one narrow seam: make the resolver actually honor the supplied canonical layout contract and prove it with focused tests.
- Packet 2.1 must already be complete.
- Stay inside Packet 2.2 only.

Hard rules:
- Do not implement, review, or fix Packet 2.2 work in the orchestration session yourself.
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, commit the implementation before any review round.
- Then spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them.
- Every fix subagent prompt must begin with `/goal ` and must explicitly use $incremental-implementation.
- After each accepted fix round that changes files, commit before dispatching the next fresh review subagent.
- Preserve unrelated local edits, including any dirt already present in `AGENTS.md` and `CLAUDE.md`.
- Before editing any production symbol in `crates/flow/src/resolver.rs`, `crates/flow/src/lib.rs`, or any narrow engine boundary file that becomes necessary, run GitNexus impact analysis first and report the blast radius. If GitNexus says the index is stale, run `npx gitnexus analyze` first. If the blast radius is HIGH or CRITICAL, stop and report it before editing.
- Before every commit, run GitNexus detect-changes and verify the affected scope matches Packet 2.2 only.
- Stay inside Packet 2.2 scope.

Packet 2.2 scope:
- Route the contract-aware resolver path through `CanonicalArtifacts::load_with_contract(...)`.
- Thread the active contract into any contract-dependent fallback/path surfaces that would otherwise force default `.handbook/**` behavior back into the supported API.
- Update only the inseparable refusal/blocker summary text needed for structural honesty.
- Add positive non-default canonical-layout coverage in `crates/flow/tests/resolver_core.rs`.
- Refresh `docs/specs/handbook-flow-import-boundary-consumer-contract.md` if the public flow surface changed.
- Expected files:
  - crates/flow/src/resolver.rs
  - crates/flow/tests/resolver_core.rs
  - docs/specs/handbook-flow-import-boundary-consumer-contract.md
  - crates/flow/src/lib.rs (only if the public surface still needs alignment after Packet 2.1)
  - crates/engine/src/canonical_artifacts.rs (only if a narrow engine-boundary adjustment proves necessary)

Out of scope — do NOT touch:
- Packet 2.1 public contract design beyond minimal follow-through wiring
- Packet 2.3 proof notes except where proof-related assertions must be updated to match the new focused tests
- Set 1 or Set 3 work
- CLI/compiler/product-shell cleanup
- broad `.handbook` wording sweeps beyond the contract-dependent surfaces that would otherwise stay structurally wrong
- actual Substrate import execution

Implementation subagent prompt requirements:
- Begin with `/goal Land Set 2 Packet 2.2: Resolver Adoption And Test Coverage`.
- Tell the subagent to use $incremental-implementation.
- Require live verification with:
  - `git status --short --branch`
  - `sed -n '430,560p' crates/flow/src/resolver.rs`
  - `sed -n '1180,1460p' crates/flow/src/resolver.rs`
  - `sed -n '1,260p' crates/flow/tests/resolver_core.rs`
  - `sed -n '1,260p' crates/engine/src/canonical_paths.rs`
  - `sed -n '200,280p' crates/engine/src/canonical_artifacts.rs`
  - `cargo test -p handbook-engine --test canonical_artifacts_ingest`
  - `cargo test -p handbook-engine --test baseline_validation`
  - `cargo test -p handbook-flow --test resolver_core`
- Require the implementation to:
  - verify Packet 2.1 landed first
  - run GitNexus impact analysis before editing production symbols and report the blast radius
  - make the contract-aware flow path truly honor the supplied engine-owned contract
  - update only the contract-dependent refusal/blocker/path or summary surfaces that are inseparable from that honesty requirement
  - add focused non-default-layout tests that prove both a successful and a blocked/refusal path
  - refresh the flow consumer-contract doc if the public flow surface changed
  - avoid widening into a general Set 3 wording cleanup
  - stop after Packet 2.2 acceptance is met and report touched files, impact-analysis results, verification run, residual risks, and the exact commit hash/message if one is created

Review subagent prompt requirements:
- Begin with `/goal Review Set 2 Packet 2.2: Resolver Adoption And Test Coverage`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to check:
  - whether the contract actually flows through resolver loading and contract-dependent path/result behavior
  - whether engine still remains the single layout owner with no second model introduced
  - whether the new tests really prove non-default-layout behavior instead of only compile-time plumbing
  - whether any summary/path cleanup stayed narrowly inseparable from the structural seam rather than drifting into Set 3
  - whether scope leaked beyond `resolver.rs`, `resolver_core.rs`, and narrow boundary-doc sync
- Require severity labels and explicit callouts for false proof, hidden default fallback, or scope drift.

Fix loop:
- If the review is clean, stop and report Packet 2.2 complete.
- If the review finds issues, spawn one fresh GPT-5.4 high fix subagent per review round using `$incremental-implementation`.
- The fix prompt must cite the exact review findings and require only the minimal Packet-2.2-bounded changes needed to close them.
- Commit accepted fixes before dispatching the next fresh review subagent.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after implementation if Packet 2.2 lands cleanly.
- Before each commit, run GitNexus detect-changes and confirm the affected scope matches Packet 2.2 only.
- Commit after each accepted fix round.
- Commit messages must describe the resolver contract-adoption work clearly and standalone.

Stop conditions:
- Stop once Packet 2.2 is review-clean, committed, and focused tests prove the supported flow-facing seam can honor a non-default canonical layout contract.
- Stop and report blocked if the packet cannot close honestly without widening into Set 3 wording cleanup, CLI/compiler work, or a broader public-boundary redesign.
```

---

## Packet 2.3 Prompt — Final Set Proof

```text
/goal Orchestrate Set 2 Packet 2.3: Final Set Proof in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 2.3 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-substrate-parameterization-set-2-flow-canonical-layout-injection-tasks.md.
- Verify live repo truth before changing anything.
- Use the approved set authority at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-substrate-parameterization-set-2-flow-canonical-layout-injection-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-substrate-parameterization-set-2-flow-canonical-layout-injection-plan.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-substrate-parameterization-set-2-flow-canonical-layout-injection-tasks.md
- Treat the packet as one narrow proof seam: run the verification wall and record residual defaults honestly.
- Packets 2.1 and 2.2 must already be complete.
- Stay inside Packet 2.3 only.

Hard rules:
- Do not implement, review, or fix Packet 2.3 work in the orchestration session yourself.
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, commit the implementation before any review round.
- Then spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them.
- Every fix subagent prompt must begin with `/goal ` and must explicitly use $incremental-implementation.
- After each accepted fix round that changes files, commit before dispatching the next fresh review subagent.
- Preserve unrelated local edits, including any dirt already present in `AGENTS.md` and `CLAUDE.md`.
- Before every commit, run GitNexus detect-changes and verify the affected scope matches Packet 2.3 only.
- Stay inside Packet 2.3 scope.

Packet 2.3 scope:
- Run the full Set 2 verification wall.
- Record pass/fail in the completion notes of the Set 2 tasks doc.
- Record the bounded residual-default inventory honestly.
- Update docs only for the proof/notes unless a minimal packet-scoped fix is required.
- Expected files:
  - docs/specs/handbook-substrate-parameterization-set-2-flow-canonical-layout-injection-tasks.md

Out of scope — do NOT touch:
- silently reopening Packet 2.1 or Packet 2.2
- Set 1 or Set 3 work
- CLI/compiler/product-shell cleanup
- actual Substrate import execution
- speculative structural code changes beyond the minimal fix needed to resolve a proof failure already inside Set 2 scope

Implementation subagent prompt requirements:
- Begin with `/goal Land Set 2 Packet 2.3: Final Set Proof`.
- Tell the subagent to use $incremental-implementation.
- Require live verification with:
  - `git status --short --branch`
  - `cargo test -p handbook-engine --test canonical_artifacts_ingest`
  - `cargo test -p handbook-engine --test baseline_validation`
  - `cargo test -p handbook-flow --test resolver_core`
  - `cargo test -p handbook-flow`
  - `cargo check --workspace`
  - `cargo fmt --all -- --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `rg -n "CanonicalArtifacts::load\(|load_with_contract|default_canonical_layout_contract|canonical \.handbook root|\.handbook" crates/flow/src crates/flow/tests crates/engine/src crates/engine/tests`
- Require the implementation to:
  - verify Packets 2.1 and 2.2 landed first
  - run the full verification wall and record exact pass/fail results in the Packet 2.3 completion notes
  - distinguish acceptable retained `.handbook` references that remain Set 3 territory from structural blockers that require reopening Packet 2.1 or Packet 2.2
  - avoid silently reopening earlier packets; if structural work is still missing, report it as a blocker and cite the packet that must be reopened
  - stop after Packet 2.3 acceptance is met and report touched files, verification run, residual defaults, and any blockers

Review subagent prompt requirements:
- Begin with `/goal Review Set 2 Packet 2.3: Final Set Proof`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to check:
  - whether the verification wall actually ran and was recorded accurately
  - whether the residual-default inventory is honest about what is still Set 3 territory
  - whether the packet stayed proof-only instead of silently absorbing structural work from Packet 2.1 or Packet 2.2
  - whether the completion notes are internally consistent with the Set 2 spec, plan, tasks, and live verification evidence
- Require severity labels and explicit callouts for incomplete proof, inaccurate recording, or hidden scope widening.

Fix loop:
- If the review is clean, stop and report Packet 2.3 complete.
- If the review finds issues, spawn one fresh GPT-5.4 high fix subagent per review round using `$incremental-implementation`.
- The fix prompt must cite the exact review findings and require only the minimal Packet-2.3-bounded changes needed to close them.
- Commit accepted fixes before dispatching the next fresh review subagent.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after implementation if Packet 2.3 lands cleanly.
- Before each commit, run GitNexus detect-changes and confirm the affected scope matches Packet 2.3 only.
- Commit after each accepted fix round.
- Commit messages must describe the Set 2 proof work clearly and standalone.

Stop conditions:
- Stop once Packet 2.3 is review-clean, committed, and the tasks doc records both the verification wall and the bounded residual-default inventory honestly.
- Stop and report blocked if verification reveals unfinished structural work from Packet 2.1 or Packet 2.2 that must be reopened explicitly.
```
