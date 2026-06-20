# Spec: Handbook Substrate Parameterization — Set 2: Flow Canonical-Layout Injection

## Assumptions I'm Making

1. Set 1 is already complete enough to serve as the upstream authority for this set, so this triplet should not reopen pipeline declarative-root or storage-layout parameterization work.
2. The engine-owned `CanonicalLayoutContract` is already the canonical owner for handbook artifact roots; this set should make `handbook-flow` consume that existing contract through a supported public import-facing seam rather than inventing a second layout model.
3. The intended downstream integration target remains a Substrate-owned namespace such as `.substrate/handbook/**`, so downstream callers must be able to resolve flow packets without depending on handbook-product default `.handbook/**` placement.
4. `handbook-cli` and `handbook-compiler` remain out of scope even if they still contain handbook-product assumptions.
5. Residual `.handbook` wording cleanup is mostly a Set 3 concern, but any refusal/blocker/path surface that would make the Set 2 public API structurally dishonest for an injected contract is in scope here.
6. Default handbook-product behavior should remain available through an explicit default path, not through hidden fallback that downstream consumers must discover by reading implementation details.

## Objective

Create the second implementation set required by the three-set map: make the public `handbook-flow` import surface consume an engine-owned canonical layout contract so downstream importers can resolve packets against a non-default handbook root under `.substrate/handbook/**`.

The live structural gap is currently:

1. `handbook-flow::resolve(...)` still calls `CanonicalArtifacts::load(...)`, which hard-wires flow resolution to the handbook-product default canonical root.
2. Downstream consumers cannot inject a non-default `CanonicalLayoutContract` through the supported public flow surface.
3. Engine already owns the reusable contract surface (`CanonicalLayoutContract`, `default_canonical_layout_contract()`, `CanonicalArtifacts::load_with_contract(...)`), but flow does not yet consume that owner through its public import-facing seam.
4. Residual `.handbook` refusal/blocker wording still exists in flow and engine. Most of that honesty cleanup belongs to Set 3, but any contract-dependent public result that would stay structurally wrong after injection is part of Set 2.

This set is about the **structural import seam** only. It is not a general wording cleanup set, not a Set 1 reopen, and not permission to redesign the Lane B flow consumer boundary more broadly than necessary.

## Tech Stack

- Rust 2021 workspace
- Primary crate under change: `crates/flow`
- Narrow supporting contract owner: `crates/engine`
- Existing engine public contract surface:
  - `CanonicalLayoutContract`
  - `default_canonical_layout_contract()`
  - `CanonicalArtifacts::load_with_contract(...)`
- Primary verification surfaces:
  - `crates/flow/src/resolver.rs`
  - `crates/flow/src/lib.rs`
  - `crates/flow/tests/resolver_core.rs`
  - `crates/engine/src/canonical_paths.rs`
  - `crates/engine/src/canonical_artifacts.rs`
  - `crates/engine/tests/canonical_artifacts_ingest.rs`
  - `crates/engine/tests/baseline_validation.rs`

## Commands

```bash
# Workspace baseline
cargo check --workspace

# Engine contract-owner proof
cargo test -p handbook-engine --test canonical_artifacts_ingest
cargo test -p handbook-engine --test baseline_validation

# Focused handbook-flow proof
cargo test -p handbook-flow --test resolver_core
cargo test -p handbook-flow

# Full lint / format wall
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings

# Live structural evidence sweep
rg -n "CanonicalArtifacts::load\(|load_with_contract|default_canonical_layout_contract|canonical \.handbook root|\.handbook" crates/flow/src crates/flow/tests crates/engine/src crates/engine/tests
```

## Project Structure

```text
docs/specs/
  handbook-substrate-parameterization-three-set-map.md                                   → set sequencing authority
  handbook-substrate-parameterization-gap-map.md                                         → live gap inventory
  handbook-substrate-import-adoption-plan.md                                             → downstream import order / boundary posture
  handbook-substrate-parameterization-set-2-flow-canonical-layout-injection-spec.md      → this file
  handbook-substrate-parameterization-set-2-flow-canonical-layout-injection-plan.md      → set 2 plan
  handbook-substrate-parameterization-set-2-flow-canonical-layout-injection-tasks.md     → set 2 packets/tasks
  handbook-substrate-parameterization-set-2-flow-canonical-layout-injection-packet-prompts.md
                                                                                         → one orchestration prompt per packet
  handbook-flow-import-boundary-consumer-contract.md                                     → active flow boundary authority that may need refresh once Set 2 lands

crates/flow/src/
  lib.rs                  → public re-exports / public flow import surface
  resolver.rs             → public resolution entrypoint and refusal/blocker behavior
  budget.rs               → typed budget actions that may reference canonical paths
  packet_result.rs        → typed packet output surface

crates/flow/tests/
  resolver_core.rs        → current default-root resolver proof surface; will need non-default contract coverage

crates/engine/src/
  lib.rs                  → re-exports `CanonicalLayoutContract` and default helper
  canonical_paths.rs      → canonical layout contract owner and validation
  canonical_artifacts.rs  → `load(...)` vs `load_with_contract(...)`

crates/engine/tests/
  canonical_artifacts_ingest.rs → proves custom layout identities / ingest issue paths
  baseline_validation.rs        → proves validation follows loaded custom layout paths
```

## Code Style

Prefer an additive public wrapper pattern that keeps engine as the layout owner and keeps the default handbook-product path explicit:

```rust
pub fn resolve(
    repo_root: impl AsRef<Path>,
    request: ResolveRequest,
) -> Result<ResolverResult, ManifestError> {
    resolve_with_contract(
        repo_root,
        request,
        *default_canonical_layout_contract(),
    )
}

pub fn resolve_with_contract(
    repo_root: impl AsRef<Path>,
    request: ResolveRequest,
    contract: CanonicalLayoutContract,
) -> Result<ResolverResult, ManifestError> {
    let canonical_artifacts =
        CanonicalArtifacts::load_with_contract(repo_root.as_ref(), contract)?;
    // ... resolver logic uses the supplied engine-owned contract ...
}
```

Code-shape expectations for this set:

- keep `CanonicalLayoutContract` owned by `handbook-engine`
- do **not** create a flow-owned alias, duplicate layout type, or competing wrapper model
- keep `resolve(...)` as the explicit handbook-product default helper path if an additive entrypoint is introduced
- update only the refusal/blocker/path text that is inseparable from making the injected contract structurally honest
- defer broader `.handbook` wording sweeps to Set 3

## Testing Strategy

This set is implemented as three sequential packets inside one set triplet:

1. **Packet 2.1 — Flow Public API Contract Shape**
   - Introduce the supported public flow-facing entrypoint for consuming the engine-owned canonical layout contract.
   - Preserve current default behavior through an explicit default wrapper.
   - Primary verification:
     - source inspection of `crates/flow/src/lib.rs` and `crates/flow/src/resolver.rs`
     - `cargo test -p handbook-flow --test resolver_core`

2. **Packet 2.2 — Resolver Adoption And Test Coverage**
   - Thread the supplied contract through resolver loading and any contract-dependent refusal/blocker/path surfaces that would otherwise remain structurally wrong.
   - Add positive non-default-layout coverage in `resolver_core.rs` using the engine-owned contract.
   - Refresh `docs/specs/handbook-flow-import-boundary-consumer-contract.md` if the public symbol story changes.
   - Primary verification:
     - `cargo test -p handbook-engine --test canonical_artifacts_ingest`
     - `cargo test -p handbook-engine --test baseline_validation`
     - `cargo test -p handbook-flow --test resolver_core`

3. **Packet 2.3 — Final Set Proof**
   - Run the final verification wall.
   - Record any retained `.handbook` references honestly as either acceptable Set 3-deferred residuals or proof that Packet 2.1 / 2.2 must be reopened.
   - Primary verification:
     - `cargo test -p handbook-flow`
     - `cargo check --workspace`
     - `cargo fmt --all -- --check`
     - `cargo clippy --workspace --all-targets -- -D warnings`
     - targeted `rg` sweep for residual default-root wording and fallback behavior

## Boundaries

- **Always:**
  - Keep Set 2 focused on `handbook-flow` canonical-layout injection only.
  - Keep the engine canonical layout contract as the single owner.
  - Distinguish structural import-seam work from broader honesty cleanup.
  - Preserve an explicit default handbook-product path for existing callers.
  - Record residual `.handbook` assumptions honestly instead of pretending they are already gone.

- **Ask first:**
  - If the cleanest implementation appears to require changing `ResolveRequest` instead of adding a separate contract-aware entrypoint.
  - If flow can only consume the engine contract by introducing a second flow-owned layout abstraction.
  - If Set 2 reveals a wider redesign of refusal/blocker textual surfaces than the inseparable contract-dependent cases identified here.
  - If implementation would require widening into `handbook-cli`, `handbook-compiler`, or Set 3 cleanup to land honestly.

- **Never:**
  - Reopen Set 1 pipeline parameterization work as part of Set 2.
  - Pre-author or start Set 3 implementation from this set.
  - Invent a second layout model alongside the engine contract.
  - Widen into CLI/compiler/product-shell cleanup.
  - Execute the actual Substrate import.

## Success Criteria

1. A supported public `handbook-flow` path can resolve using a non-default engine-owned `CanonicalLayoutContract`.
2. `handbook-flow` no longer depends exclusively on `CanonicalArtifacts::load(...)` for its supported import-facing resolution path.
3. Default handbook-product behavior remains available through an explicit default path rather than as the only supported mode.
4. Any contract-dependent refusal/blocker/path payloads that would otherwise force callers back to `.handbook/**` are made consistent with the active contract.
5. No second layout model is introduced; engine remains the canonical contract owner.
6. The flow-facing canonical-root behavior stays consistent with the engine-owned canonical layout contract.
7. Downstream consumers do not need handbook-product default canonical-root behavior to integrate flow under `.substrate/handbook/**`.
8. Residual `.handbook` wording that is not structurally inseparable is explicitly deferred to Set 3 rather than silently absorbed here.

## Open Questions

1. **Public API shape choice**
   - Default plan for this set: add an additive contract-aware entrypoint (for example `resolve_with_contract(...)`) and keep `resolve(...)` as the explicit default wrapper.
   - If maintainers prefer embedding `CanonicalLayoutContract` inside `ResolveRequest` instead, ask before implementation because that changes the frozen flow boundary more deeply.

2. **Boundary-doc refresh timing**
   - Default plan for this set: update `docs/specs/handbook-flow-import-boundary-consumer-contract.md` during Packet 2.2 once the public shape and tests are real.
   - If maintainers want that contract doc updated only in Packet 2.3, confirm before implementation so the proof packet does not become a structural documentation sink.
