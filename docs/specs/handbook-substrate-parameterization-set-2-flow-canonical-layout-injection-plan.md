# Plan: Handbook Substrate Parameterization — Set 2: Flow Canonical-Layout Injection

Spec reference: [handbook-substrate-parameterization-set-2-flow-canonical-layout-injection-spec.md](./handbook-substrate-parameterization-set-2-flow-canonical-layout-injection-spec.md)

Upstream authority carried forward from Set 1:
- [handbook-substrate-parameterization-set-1-pipeline-import-layout-spec.md](./handbook-substrate-parameterization-set-1-pipeline-import-layout-spec.md)
- [handbook-substrate-parameterization-set-1-pipeline-import-layout-plan.md](./handbook-substrate-parameterization-set-1-pipeline-import-layout-plan.md)
- [handbook-substrate-parameterization-set-1-pipeline-import-layout-tasks.md](./handbook-substrate-parameterization-set-1-pipeline-import-layout-tasks.md)

## Objective

Translate the Set 2 seam into a bounded packet plan that makes `handbook-flow` consume the engine-owned canonical layout contract through a supported public import-facing surface, while keeping broader `.handbook` wording cleanup deferred unless it is inseparable from that seam.

## Current State (live repo truth)

1. `crates/engine/src/canonical_paths.rs` already defines the reusable owner surface:
   - `CanonicalLayoutContract`
   - `default_canonical_layout_contract()`
   - contract validation / repo-relative path derivation
2. `crates/engine/src/canonical_artifacts.rs` already exposes both:
   - `CanonicalArtifacts::load(...)` → explicit default helper path
   - `CanonicalArtifacts::load_with_contract(...)` → non-default contract path
3. Engine tests already prove non-default layout support:
   - `crates/engine/tests/canonical_artifacts_ingest.rs` uses `.custom_handbook/**` with `CanonicalLayoutContract::from_paths(...)`
   - `crates/engine/tests/baseline_validation.rs` proves baseline validation follows those loaded custom paths
4. `crates/flow/src/resolver.rs` still exposes only:
   - `pub fn resolve(repo_root, request) -> Result<ResolverResult, ManifestError>`
   - that entrypoint still calls `CanonicalArtifacts::load(repo_root.as_ref())`
5. `ResolveRequest` currently carries only `budget_policy` and `packet_id`; there is no public flow-facing way to supply a non-default canonical layout contract.
6. `crates/flow/tests/resolver_core.rs` still proves only default `.handbook/**` behavior.
7. Residual `.handbook` wording and default fallback behavior still remain in `resolver.rs`, including:
   - system-root refusal summaries
   - budget fallback paths that use `default_canonical_layout_contract().system_root_relative()`
8. Set 1 is already closed upstream and should remain closed; Set 2 should consume that result, not reopen it.

## Planned Seam Shape

The default planned shape for Set 2 is:

1. Add an **additive public contract-aware resolver entrypoint** in `handbook-flow` that consumes the engine-owned `CanonicalLayoutContract` directly.
2. Keep `resolve(...)` as the explicit default wrapper that preserves handbook-product behavior.
3. Route resolver internals through the supplied contract where contract-dependent loading or path/result surfaces would otherwise stay hard-wired to `.handbook/**`.
4. Leave broader wording-only cleanup to Set 3 unless the public API would remain structurally dishonest without it.

This keeps engine as the contract owner, avoids a second layout model, and avoids forcing every current caller to change at once.

## Components

### 1. Public flow contract shape

Define the supported flow-facing contract seam for downstream importers.

This component should:
- expose a public entrypoint that accepts `CanonicalLayoutContract`
- keep the current `resolve(...)` default behavior explicit
- avoid duplicating or wrapping the layout model in a competing flow-owned type
- keep the public surface narrow enough that Set 3 remains separate

### 2. Resolver-core contract adoption

Adopt the active contract inside flow resolution.

This component includes:
- switching the contract-aware path away from unconditional `CanonicalArtifacts::load(...)`
- routing contract-dependent fallback/path logic through the active contract
- updating the minimal public summaries or typed payloads that would otherwise lie about the injected root
- adding non-default-layout tests in `resolver_core.rs`
- updating the flow consumer contract doc if the public symbol story changes

This component does **not** include:
- a general `.handbook` wording sweep across all flow or engine text
- CLI/compiler rendering cleanup
- pipeline Set 1 work

### 3. Final proof and residual-default inventory

After the structural packets land:
- run the final verification wall
- inspect remaining `.handbook` references
- distinguish acceptable Set 3-deferred residual wording from structural failure of Set 2
- record the completion notes without letting Packet 2.3 become an implementation sink

## Packet Plan

## Packet 2.1 — Flow Public API Contract Shape

### Goal

Introduce the supported public flow-facing contract shape for canonical-layout injection.

### Work

- add the contract-aware public resolver entrypoint
- keep `resolve(...)` as the explicit handbook-product default wrapper
- wire only the minimal public re-export / surface changes needed for the new entrypoint
- keep `CanonicalLayoutContract` owned by engine rather than creating a flow-owned alias or wrapper
- update only the smallest amount of source needed to establish the public seam coherently

### Verification checkpoint

```bash
git status --short --branch
sed -n '1,120p' crates/flow/src/lib.rs
sed -n '400,470p' crates/flow/src/resolver.rs
sed -n '1,120p' crates/engine/src/lib.rs
sed -n '1,260p' crates/engine/src/canonical_paths.rs
cargo test -p handbook-flow --test resolver_core
```

### Exit condition

A downstream caller can see a supported flow-facing entrypoint for passing the engine-owned contract, and the default `resolve(...)` path remains explicit. Packet 2.1 is not responsible for all contract-dependent refusal/blocker cleanup or final proof.

## Packet 2.2 — Resolver Adoption And Test Coverage

### Goal

Make the resolver actually honor the supplied contract and prove it with focused tests.

### Work

- route the contract-aware flow path through `CanonicalArtifacts::load_with_contract(...)`
- thread the active contract into any contract-dependent fallback/path surfaces that would otherwise force default `.handbook/**` behavior back into the supported API
- update only the inseparable system-root/refusal/blocker summary text needed for structural honesty
- add positive non-default contract coverage in `crates/flow/tests/resolver_core.rs`
- refresh `docs/specs/handbook-flow-import-boundary-consumer-contract.md` if the public flow surface changed
- keep broader wording cleanup explicitly deferred to Set 3

### Verification checkpoint

```bash
git status --short --branch
sed -n '430,560p' crates/flow/src/resolver.rs
sed -n '1180,1460p' crates/flow/src/resolver.rs
sed -n '1,260p' crates/flow/tests/resolver_core.rs
sed -n '200,280p' crates/engine/src/canonical_artifacts.rs
cargo test -p handbook-engine --test canonical_artifacts_ingest
cargo test -p handbook-engine --test baseline_validation
cargo test -p handbook-flow --test resolver_core
```

### Exit condition

The supported flow-facing contract path honors a non-default canonical layout contract in both load and focused resolver proof surfaces, and the packet has not widened into a general Set 3 wording sweep.

## Packet 2.3 — Final Set Proof

### Goal

Run the final verification wall and record residual defaults honestly without absorbing unfinished structural work.

### Work

- verify Packets 2.1 and 2.2 landed first
- run the final verification wall
- inspect remaining `.handbook` references in flow/engine surfaces touched by this set
- record pass/fail and residual-default inventory in the Set 2 tasks doc
- reopen Packet 2.1 or 2.2 explicitly if structural work is still missing

### Verification checkpoint

```bash
git status --short --branch
cargo test -p handbook-engine --test canonical_artifacts_ingest
cargo test -p handbook-engine --test baseline_validation
cargo test -p handbook-flow --test resolver_core
cargo test -p handbook-flow
cargo check --workspace
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
rg -n "CanonicalArtifacts::load\(|load_with_contract|default_canonical_layout_contract|canonical \.handbook root|\.handbook" crates/flow/src crates/flow/tests crates/engine/src crates/engine/tests
```

### Exit condition

The set is reviewable as a complete structural flow canonical-layout injection story, and the tasks doc clearly separates acceptable Set 3-deferred residual wording from evidence that Packets 2.1 or 2.2 are incomplete.

## Implementation Order

1. **Packet 2.1 first** because Set 2 needs a supported public seam before deeper resolver adoption can be judged coherently.
2. **Packet 2.2 second** because the public seam is not honest until resolver internals and focused tests actually honor the supplied contract.
3. **Packet 2.3 last** because proof is only meaningful after the structural packets land.

## Verification Checkpoints And Stop Boundaries

- After **Packet 2.1**, stop if the public contract shape still implies a second layout model or a breaking `ResolveRequest` redesign.
- After **Packet 2.2**, stop if resolver results still force contract-aware callers back onto default `.handbook/**` behavior in any contract-dependent path surface.
- During **Packet 2.3**, stop and reopen an earlier packet if proof reveals missing structural work. Do not silently repair structural gaps inside the proof packet.

## Risks And Mitigations

### Risk 1: Set 2 accidentally invents a second layout model

- **Why it matters:** the user explicitly wants engine to remain the canonical contract owner.
- **Mitigation:** consume `CanonicalLayoutContract` directly from `handbook-engine`; do not create a flow-owned layout wrapper or alias.

### Risk 2: Set 2 widens into Set 3 wording cleanup

- **Why it matters:** the three-set map depends on a narrow structural seam here and a later honesty-cleanup seam.
- **Mitigation:** change only contract-dependent refusal/blocker/path text that would otherwise make the supported API structurally dishonest; defer broader wording cleanup.

### Risk 3: public seam lands but still hides default-root fallback inside resolver behavior

- **Why it matters:** that would make the new public seam nominally present but not truly usable for `.substrate/handbook/**`.
- **Mitigation:** require Packet 2.2 tests that exercise non-default roots on both ready and blocked/refusal paths.

### Risk 4: Packet 2.3 becomes an implementation sink

- **Why it matters:** proof packets that silently patch earlier structural work make future authority docs untrustworthy.
- **Mitigation:** record an explicit reopen rule for Packet 2.1 / 2.2 whenever proof finds missing structural adoption.

## Stop Boundary

Stop after Packet 2.3 for this set. Do not:
- reopen Set 1
- start Set 3 implementation
- widen into CLI/compiler cleanup
- execute actual Substrate import work
- convert Packet 2.3 into a catch-all structural repair packet
