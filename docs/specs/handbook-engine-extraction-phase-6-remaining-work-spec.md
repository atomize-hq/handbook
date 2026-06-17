# Spec: Handbook Engine Extraction — Phase 6 Remaining Work

## Objective

Produce the single navigable planning authority for the remaining Phase 6 handbook engine extraction work: two execution lanes and one optional lane, with explicit crate-by-crate import readiness posture.

The three live decisions are:
1. **handbook-engine** — required import target and import-ready now. No remaining technical blocker.
2. **handbook-pipeline** — required import target; boundary frozen (documented frozen subset of current public surface, closed 2026-06-17). Import-ready pending Lane D adoption plan.
3. **handbook-flow** — required import target; the crate/type dependency boundary is already clean, but Lane B must remove the remaining shell-owned/operator-facing copy from the flow import surface before the stable consumer contract is formalized and frozen.

The two execution lanes are:
- **Lane B: Flow required-import boundary cleanup + contract freeze** — prove the clean crate/type boundary, capture the remaining shell-owned/operator-facing leakage still visible on the import surface, land the narrow cleanup packet that moves that final shell copy out of `handbook-flow` while keeping CLI as the only product shell, then formalize and freeze the stable consumer contract.
- **Lane D: Final Substrate import plan** — write the actual import/adoption plan for engine + pipeline + flow. This is a planning artifact, not import execution.

The one optional lane:
- **Lane C: Engine optional boundary freeze** — only if a stricter publishable API is later desired. Not a blocker for any lane.

### Lane A status (closed)

Lane A (pipeline boundary cleanup) is fully closed:
- Compiler-backed dev-dependency removed (commit `2dfb9b7`).
- Full verification wall passed (2026-06-17): `pipeline_catalog` (14), `pipeline_compile` (21), `pipeline_capture` (45), `pipeline_handoff` (8), compiler `author` (59), `cargo check --workspace` clean.
- Durable boundary decision: **documented frozen subset of the current public surface**.
- In-boundary modules: `pipeline`, `pipeline_capture`, `pipeline_compile`, `pipeline_handoff`, `pipeline_route`, `route_state`, plus `pipeline_contract_version()`.
- Out-of-boundary modules: `setup` (CLI/compiler only), `declarative_roots` (internal), and private `layout`/`repo_file_access`/`stage_10_feature_spec_provenance`.
- Triplet archived under `docs/specs/archive/phase-6-pipeline-boundary-cleanup/`.

Lane A is not a remaining lane. It is recorded here for completeness and to prevent reopening.

### Crate import-readiness posture (live repo truth, 2026-06-17)

| Crate | Dependencies | Import target? | Posture |
|-------|-------------|----------------|---------|
| `handbook-engine` | libc, serde, serde_yaml_bw | **Yes** | Import-ready. No handbook-* dependencies. Public surface is narrow and clean. |
| `handbook-pipeline` | handbook-engine, libc, serde, serde_json, serde_yaml_bw, sha2, time | **Yes** | Boundary frozen (documented frozen subset). No handbook-compiler dependency (runtime or dev). |
| `handbook-flow` | handbook-engine | **Yes** | Required import target. `handbook-flow -> handbook-engine` crate coupling is already clean. Remaining blocker: final shell-owned/operator-facing copy still leaks through the public flow surface and must be cleaned up before the consumer contract is frozen. |
| `handbook-cli` | clap + all extracted crates | **No** | Product shell. 574-line clap entrypoint. Explicit non-import target for this Phase 6 plan. |
| `handbook-compiler` | all extracted crates | **Open / not decided here** | Current transition/support seam in this repo. Lane D stays focused on engine + pipeline + flow and does not pre-decide compiler's eventual Substrate posture. |

## Tech Stack

- Rust 2021 edition workspace
- Crates: `crates/engine`, `crates/pipeline`, `crates/flow`, `crates/cli`, `crates/compiler`
- No external handbook-* dependencies in the three import-target crates beyond `handbook-engine`
- `#![forbid(unsafe_code)]` in engine and flow

## Commands

```bash
# Full workspace check
cargo check --workspace

# Per-crate verification
cargo test -p handbook-engine
cargo test -p handbook-pipeline --test pipeline_catalog
cargo test -p handbook-pipeline --test pipeline_compile
cargo test -p handbook-pipeline --test pipeline_capture
cargo test -p handbook-pipeline --test pipeline_handoff
cargo test -p handbook-flow
cargo test -p handbook-compiler --test author

# Dependency verification
cargo tree -p handbook-engine
cargo tree -p handbook-pipeline
cargo tree -p handbook-flow

# Coupling verification (should return nothing)
rg -n "handbook_compiler|handbook_cli|handbook_pipeline" crates/flow/src/ crates/flow/tests/

# Flow shell-copy spot-check (supporting evidence only after Packet 6.B.2)
# Use alongside source inspection of the public `handbook-flow` surface to prove
# no final shell-owned/operator-facing copy remains on that surface.
rg -n 'run `doctor`|handbook inspect --packet|handbook generate --packet|handbook setup' crates/flow/src/

# Full lint wall
cargo fmt --all -- --check && cargo clippy --workspace --all-targets -- -D warnings
```

## Project Structure

```
crates/
  engine/          → Import-ready: artifact_manifest, author, baseline_validation, canonical_artifacts, freshness
  pipeline/        → Boundary frozen: pipeline, pipeline_capture, pipeline_compile, pipeline_handoff, pipeline_route, route_state
  flow/            → Required import target; needs shell-ownership cleanup + contract freeze
  cli/             → Product shell (not an import target)
  compiler/        → Current transition/support seam; Lane D does not pre-decide its eventual Substrate posture
docs/specs/
  handbook-engine-extraction-phase-6-remaining-work-spec.md      → This file
  handbook-engine-extraction-phase-6-remaining-work-plan.md      → Implementation plan
  handbook-engine-extraction-phase-6-remaining-work-tasks.md     → Task breakdown
  archive/                                                         → All historical Phase 1–6 artifacts
```

## Code Style

Existing crate conventions (no style changes expected from this planning update):

```rust
// engine: explicit re-exports, forbid unsafe, workspace_contract_version() function
#![forbid(unsafe_code)]
pub fn workspace_contract_version() -> &'static str { "C-02" }

// flow: three modules, clean re-exports, flow_contract_version() delegates to engine
pub mod budget;
pub mod packet_result;
pub mod resolver;
pub fn flow_contract_version() -> &'static str {
    handbook_engine::workspace_contract_version()
}

// pipeline: frozen subset re-exports, pipeline_contract_version() delegates to engine
pub fn pipeline_contract_version() -> &'static str {
    handbook_engine::workspace_contract_version()
}
```

## Testing Strategy

- **Lane B (flow required-import boundary cleanup + contract freeze):** This is now a four-packet lane:
  - **Packet 6.B.1** records evidence for the already-clean crate/type dependency boundary and separately records the remaining shell-owned/operator-facing leakage still visible on the public flow surface.
  - **Packet 6.B.2** is the narrow production cleanup seam: remove the final shell-owned/operator-facing copy from `handbook-flow`'s import surface while preserving typed next-action/status semantics where they remain useful machine-readable flow results.
  - **Packet 6.B.3** formalizes the consumer contract after the cleanup lands and explicitly distinguishes in-boundary typed semantics from out-of-boundary shell rendering/copy ownership.
  - **Packet 6.B.4** runs the verification wall: dependency proof, zero extra-crate coupling proof, source-inspection proof that no final shell-owned/operator-facing copy remains on the public `handbook-flow` surface, a supporting literal-string grep spot-check in `crates/flow/src/`, tests, workspace check, fmt, and clippy.
- **Lane D (import plan):** No code changes. Verification is human review of the plan against the three import-target crate surfaces and the frozen pipeline/flow boundaries.
- **Lane C (optional):** If activated, follows the same proof-and-freeze pattern as Lane B but for engine. Not currently activated.

## Boundaries

- **Always:**
  - Ground every claim in live repo truth (cargo tree, rg, cargo test, source inspection) before recording it.
  - Keep Lane B and Lane D as separate lanes — do not collapse them.
  - Preserve the Phase 6 naming family (no Phase 7).
  - Reference archived artifacts by their archive paths, not their historical pre-archive paths.
  - Keep `handbook-cli` as the only product shell; the Lane B cleanup is about import-surface ownership, not turning flow into a shell.
- **Ask first:**
  - If Packet 6.B.2 reveals a broader redesign than moving final shell-owned copy out of the flow import surface.
  - If Packet 6.B.2 cannot close honestly without removing typed next-safe-action semantics entirely rather than just moving shell rendering/copy ownership.
  - If Lane D plan needs to introduce a narrower public facade for any crate before import.
  - If Lane C should be activated (it is currently optional/closed).
- **Never:**
  - Execute the actual Substrate import — that is beyond Phase 6 scope and requires separate authority.
  - Reopen Lane A (pipeline boundary cleanup is closed).
  - Widen into full CLI shell redesign, compiler retirement, publication, or crates.io work.
  - Make `substrate-context` become handbook.
  - Introduce compatibility aliases as a long-term architecture substitute.

## Success Criteria

1. **Lane B — Flow required-import boundary cleanup + contract freeze:**
   - Evidence proves `handbook-flow`'s public surface depends only on `handbook-engine`, std, and flow-local types transitively.
   - Evidence separately records the remaining shell-owned/operator-facing copy still exposed through the live flow import surface, instead of pretending that leakage does not exist.
   - A narrow cleanup packet removes that final shell-owned/operator-facing copy from the imported flow surface without widening into a broader CLI redesign.
   - Typed next-action/status semantics may remain on the flow surface if they stay machine-readable; final shell commands and product-shell wording must live outside `handbook-flow`.
   - A formalized consumer contract document records the frozen import boundary (in-boundary symbols, transitive type dependencies, what typed semantics remain, and what shell-owned copy/rendering is out of boundary).
   - The Lane B verification wall proves the clean dependency/coupling boundary: `cargo tree -p handbook-flow` shows `handbook-engine` as the only intra-workspace dependency, and `rg -n "handbook_compiler|handbook_cli|handbook_pipeline" crates/flow/src/ crates/flow/tests/` returns zero matches to prove zero extra-crate coupling.
   - The Lane B verification wall includes explicit source inspection of the public `handbook-flow` surface and proves the absence of final shell-owned/operator-facing copy across that surface; `rg -n 'run `doctor`|handbook inspect --packet|handbook generate --packet|handbook setup' crates/flow/src/` is supporting spot-check evidence only, not the whole proof.
   - `cargo test -p handbook-flow`, `cargo check --workspace`, and `cargo fmt --all -- --check && cargo clippy --workspace --all-targets -- -D warnings` all pass.

2. **Lane D — Final Substrate import plan:**
   - A written import/adoption plan for the required targets `handbook-engine` + `handbook-pipeline` + `handbook-flow` that:
     - States the import order and rationale.
     - Records the frozen boundary for each crate (engine: current surface; pipeline: documented frozen subset; flow: Lane B consumer contract).
     - Identifies any adapter or facade requirements (or explicitly states none are needed).
     - States the verification gate for the import step itself.
   - The plan is a planning artifact — it does not execute the import.

3. **Lane C — Engine optional boundary freeze:**
   - Remains optional. If not activated, the spec explicitly records that engine's current surface is the working boundary and Lane C is deferred.

4. **Triplet completeness:**
   - The spec, plan, and tasks docs are the single navigable authority for remaining Phase 6 work.
   - Archived Phase 6 families are referenced as provenance, not active authority.
   - No contradictions between this triplet and the root plan, slice map, or closeout map.

## Substrate-Side Constraints (resolved from live repo inspection, 2026-06-17)

Substrate repo: `/Users/spensermcconnell/__Active_Code/atomize-hq/substrate`

| Constraint | Handbook value | Substrate value | Impact |
|-----------|---------------|-----------------|--------|
| Edition | `2021` | `2021` | ✅ Compatible |
| Rust version | Not pinned | `1.89.0` (rust-toolchain.toml + rust-version) | ✅ Compatible — handbook crates build on 1.89 |
| Resolver | `2` | `2` | ✅ Compatible |
| License | Not specified | `MIT` | ⚠️ Handbook crates need a license field added before import (Substrate's `deny.toml` checks licenses) |
| `serde_yaml` | `serde_yaml_bw = "2.5.4"` (different crate) | `serde_yaml = "0.9"` | ⚠️ Different YAML implementations. Not a conflict (different crate names), but Substrate will carry both. `deny.toml` has `multiple-versions = "warn"` — won't block. |
| `sha2` | `0.10` | `0.10` (workspace dep) | ✅ Same version |
| `libc` | `0.2` | `0.2` | ✅ Same version |
| `serde` | `1` with derive | `1.0` with derive | ✅ Compatible |
| `deny.toml` bans | N/A | No crate bans; `unknown-registry = "warn"`, `unknown-git = "warn"` | ✅ Permissive |
| `#![forbid(unsafe_code)]` | Engine + flow | Lift crate also forbids; others use unsafe via nix/libc | ✅ No conflict — imported crates bring their own posture |
| Path deps | `path = "../engine"` etc. | `path = "../sibling"` pattern | ⚠️ Handbook crates must be added as workspace members or path deps in Substrate's workspace |
| Existing handbook refs | N/A | Zero references to "handbook" anywhere in Substrate | ✅ Clean slate |

**Constraints Lane D must address:**

1. **License field**: Add `license = "MIT"` (or matching) to the three import-target crate `Cargo.toml` files before import. Substrate's `deny.toml` runs license checks.
2. **Workspace integration**: Decide whether handbook crates become Substrate workspace members (like `crates/common`, `crates/shell`) or external path/git dependencies. Current Substrate pattern is workspace members with `path = "../sibling"`.
3. **YAML crate divergence**: `serde_yaml_bw` (handbook) vs `serde_yaml` (substrate) — both will coexist. Not a blocker but Lane D should record the decision: keep both, or migrate handbook crates to `serde_yaml` as a follow-up.
4. **No feature flags needed**: Handbook crates have no Cargo features. No feature-gate compatibility concern.

## Resolved Open Questions

1. ✅ **Consumer contract as standalone doc** — approved by human. Will be `docs/specs/handbook-flow-import-boundary-consumer-contract.md`.
2. ✅ **Phased rollout (engine first)** — approved by human. Engine has no intra-workspace deps, so it imports cleanly first.
3. ✅ **Substrate-side constraints** — resolved above. The main action items are: add license fields, decide workspace integration pattern, and record the YAML crate divergence decision.
4. ✅ **Lane B direction** — approved by human. Do the narrow production cleanup seam before contract formalization; do not narrow the authority to pretend shell-owned flow copy is already acceptable.
