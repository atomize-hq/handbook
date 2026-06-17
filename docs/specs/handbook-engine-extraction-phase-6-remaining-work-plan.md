# Plan: Handbook Engine Extraction — Phase 6 Remaining Work

Spec reference: [handbook-engine-extraction-phase-6-remaining-work-spec.md](./handbook-engine-extraction-phase-6-remaining-work-spec.md)

## Overview

Two execution lanes plus one optional lane, executed sequentially:

```
Lane B (flow boundary cleanup + contract freeze) → Lane D (final import plan)
                                                     ↑
                                         Lane C (optional, parallel-safe, not blocking)
```

## Lane B: Flow Required-Import Boundary Cleanup + Contract Freeze

### Goal

Make `handbook-flow` honestly import-ready as a required Substrate target by separating two truths:
1. the crate/type dependency boundary is already clean, and
2. the imported flow surface still leaks some final shell-owned/operator-facing copy that must be removed before the consumer contract is frozen.

### Current State (live repo truth, 2026-06-17)

- `handbook-flow` exports three modules: `budget`, `packet_result`, `resolver`.
- Depends only on `handbook-engine` (Cargo.toml has a single intra-workspace dependency).
- `#![forbid(unsafe_code)]`.
- Zero imports from `handbook_compiler`, `handbook_cli`, or `handbook_pipeline` in `src/` or `tests/`.
- Existing test: `crates/flow/tests/resolver_core.rs` — uses only `handbook_engine` and `handbook_flow` symbols.
- `flow_contract_version()` delegates to `handbook_engine::workspace_contract_version()`.
- `crates/flow/src/resolver.rs` still exposes typed next-safe-action variants plus ready-packet command strings such as ``run `doctor` `` and ``run `handbook inspect --packet ...` for proof``.
- `crates/flow/src/packet_result.rs` still exposes `PacketDecisionSummary.ready_next_safe_action: String`, which keeps final rendered shell copy on the public flow surface.
- `crates/cli/src/rendering.rs` and `crates/compiler/src/rendering/shared.rs` already render final shell wording for most typed next-safe-action cases, which bounds the likely cleanup seam: move the remaining final shell copy out of flow rather than redesign the decision model wholesale.

### Components

1. **Dependency proof** — `cargo tree -p handbook-flow` + `rg` evidence showing the only intra-workspace dependency is `handbook-engine`.

2. **Transitive type-dependency proof** — For each public symbol that Substrate would consume, trace its type dependencies and confirm they resolve only to `handbook-engine` public types, std types, or flow-local types. The in-boundary symbols are:
   - From `resolver`: `resolve`, `ResolveRequest`, `ResolverResult`, `ResolverRefusal`, `ResolverRefusalCategory`, `ResolverBlocker`, `ResolverBlockerCategory`, `ResolverNextSafeAction`, `ResolverSubjectRef`, `PacketSelection`, `PacketSelectionStatus`, `C04_RESULT_VERSION`
   - From `budget`: `evaluate_budget`, `BudgetDisposition`, `BudgetOutcome`, `BudgetPolicy`, `BudgetReason`, `BudgetTarget`, `NextSafeAction`
   - From `packet_result`: `PacketResult`, `PacketSection`, `PacketSectionMode`, `PacketBodyNote`, `PacketBodyNoteKind`, `PacketDecisionSummary`, `PacketFixtureContext`, `PacketSourceSummary`, `PacketVariant`

3. **Shell-ownership leakage inventory** — Record exactly what final shell-owned/operator-facing copy still leaks through the public flow surface today and separate it from the typed next-action/status semantics that may remain as machine-readable result data.

4. **Import-surface cleanup packet** — Land the narrow production seam that removes the remaining final shell copy from the flow import surface, while keeping CLI/compiler as the renderers of final shell wording and avoiding a broader CLI redesign.

5. **Consumer contract formalization** — A standalone doc recording:
   - The frozen in-boundary symbol set
   - Their transitive type dependencies (engine-public, std, or flow-local only)
   - Which typed next-action/status semantics remain in-boundary after the cleanup
   - Which shell-owned/operator-facing copy/rendering responsibilities are explicitly out of boundary
   - The contract version function

### Implementation Order

1. Gather evidence for the clean crate/type boundary and the residual shell-owned flow copy.
2. Land the narrow production cleanup packet.
3. Write the consumer contract doc against the cleaned surface.
4. Run the verification wall.
5. Record results in the tasks doc.

### Risks

- **Medium risk:** The remaining shell copy may be slightly entangled with result-shape fields (`ready_next_safe_action`) or CLI-facing wording.
- **Primary mitigation:** Keep Packet 6.B.2 tightly bounded to import-surface ownership. Preserve typed next-action/status semantics if they remain useful machine-readable results; move only the final shell wording/commands out of flow.
- **Escalation condition:** If the cleanup requires a broader redesign of doctor/setup behavior or removing typed semantics wholesale, stop and ask before widening.

### Verification Checkpoint

```bash
cargo tree -p handbook-flow
rg -n "handbook_compiler|handbook_cli|handbook_pipeline" crates/flow/src/ crates/flow/tests/
# Broad shell-ownership proof: inspect the public flow surface and confirm any
# remaining next-action/status data is typed/machine-readable only.
sed -n '1,240p' crates/flow/src/lib.rs
sed -n '1,260p' crates/flow/src/budget.rs
sed -n '1,220p' crates/flow/src/packet_result.rs
sed -n '1,320p' crates/flow/src/resolver.rs
# Supporting shell-copy spot-check only (not sufficient by itself)
rg -n 'run `doctor`|handbook inspect --packet|handbook generate --packet|handbook setup' crates/flow/src/
cargo test -p handbook-flow
cargo check --workspace
cargo fmt --all -- --check && cargo clippy --workspace --all-targets -- -D warnings
```

## Lane D: Final Substrate Import Plan

### Goal

Write the actual import/adoption plan for `handbook-engine` + `handbook-pipeline` + `handbook-flow`.

### Prerequisites

- Lane B must be complete (evidence, cleanup, contract, verification).
- Lane A is already complete (pipeline boundary frozen).

### Components

1. **Import order** — phased: engine first (no intra-workspace deps), then pipeline (depends on engine), then flow (depends on engine). Each phase has its own verification gate.

2. **Per-crate frozen boundary summary** — one paragraph per crate recording:
   - `handbook-engine`: current public surface is the working boundary (Lane C deferred).
   - `handbook-pipeline`: documented frozen subset (in-boundary modules from Lane A closeout).
   - `handbook-flow`: Lane B consumer contract — typed next-action/status semantics may remain as machine-readable flow results, but final shell wording/copy is out of boundary.

3. **Adapter/facade assessment** — explicitly state whether any adapter or facade is needed for import. Current evidence suggests none is needed beyond the Lane B import-surface cleanup, but the plan must record the assessment with evidence.

4. **Import verification gate** — what checks Substrate must pass after importing each crate. This is a planning statement, not execution.

5. **Substrate-side constraints** — resolved from live repo inspection (2026-06-17):
   - **License field**: Add `license = "MIT"` to the three import-target crate `Cargo.toml` files before import. Substrate's `deny.toml` runs license checks.
   - **Workspace integration**: Decide whether handbook crates become Substrate workspace members (current pattern: `path = "../sibling"`) or external path/git dependencies.
   - **YAML crate divergence**: `serde_yaml_bw` (handbook) vs `serde_yaml` (substrate) — both coexist; `deny.toml` `multiple-versions = "warn"` won't block. Record decision: keep both, or migrate as follow-up.
   - **No feature flags needed**: Handbook crates have no Cargo features.
   - Edition (`2021`), resolver (`2`), `sha2` (`0.10`), `libc` (`0.2`), `serde` (`1`/derive) are all compatible.

### Implementation Order

1. Read the three crate surfaces + Lane B consumer contract.
2. Write the import plan doc, including the Substrate-side constraints section.
3. Human review.

### Risks

- **Low risk:** This is a planning artifact, not execution. Substrate-side constraints have been resolved from live repo inspection.
- **Residual risk:** The workspace-integration decision (workspace member vs external dep) is still a design choice for Lane D to recommend.

### Verification Checkpoint

- Human review of the plan against the three crate surfaces and the frozen boundaries.
- No code changes, so no test wall needed.

## Lane C: Engine Optional Boundary Freeze (Optional)

### Goal

If activated: formally freeze `handbook-engine`'s public surface as a narrower publishable API.

### Current State

- Engine's public surface is already narrow: `artifact_manifest`, `author`, `baseline_validation`, `canonical_artifacts`, `freshness` modules plus `workspace_contract_version()` and `engine_contract_version()`.
- No handbook-* dependencies.
- `#![forbid(unsafe_code)]`.

### Activation Condition

Only activate if a stricter publishable API is desired or if Lane D's import plan reveals that Substrate needs a narrower surface than the current full re-export.

### If Activated

Follows the same proof-and-freeze pattern as Lane B:
1. Gather evidence.
2. Write a consumer contract doc.
3. Run verification wall.
4. Record results.

### If Not Activated

The spec and plan explicitly record that engine's current surface is the working boundary and Lane C is deferred. No further work.

## Cross-Lane Dependencies

```
Lane A (closed) ────────► Lane D (needs pipeline boundary)
Lane B (flow cleanup) ──► Lane D (needs cleaned flow consumer contract)
Lane C (optional) ───┐
                     ├───► Lane D (may narrow engine boundary)
                     │
Lane D depends on: Lane A (done) + Lane B (must complete first)
Lane C is parallel-safe with Lane B but not blocking for Lane D
```

## Execution Summary

| Lane | Status | Blocks Lane D? | Est. effort |
|------|--------|----------------|-------------|
| A | Closed (2026-06-17) | N/A — done | — |
| B | Next | Yes | Multiple packets: evidence + cleanup + contract + verification |
| C | Optional | No (if deferred) | One session if activated |
| D | After B | — | One session: plan doc + human review |
