# Tasks: Handbook Substrate Parameterization — Set 2: Flow Canonical-Layout Injection

Plan reference: [handbook-substrate-parameterization-set-2-flow-canonical-layout-injection-plan.md](./handbook-substrate-parameterization-set-2-flow-canonical-layout-injection-plan.md)

Spec reference: [handbook-substrate-parameterization-set-2-flow-canonical-layout-injection-spec.md](./handbook-substrate-parameterization-set-2-flow-canonical-layout-injection-spec.md)

---

## Packet 2.1: Flow Public API Contract Shape

- [ ] Task: Introduce the supported public flow-facing canonical-layout entrypoint
  - Acceptance: `handbook-flow` exposes a supported public resolver entrypoint that accepts the engine-owned `CanonicalLayoutContract`, and downstream callers no longer need to rely on the default-only `resolve(...)` path to integrate flow under `.substrate/handbook/**`.
  - Verify: Source inspection of `crates/flow/src/lib.rs`, `crates/flow/src/resolver.rs`, and `crates/engine/src/lib.rs`; `cargo test -p handbook-flow --test resolver_core`.
  - Files: `crates/flow/src/resolver.rs`, `crates/flow/src/lib.rs`

- [ ] Task: Preserve handbook-product default behavior as an explicit wrapper instead of the only supported mode
  - Acceptance: the existing default `resolve(...)` path remains available for handbook-product callers, but it clearly delegates through the new contract-aware seam rather than remaining the only public path. No second layout model or flow-owned alias is introduced.
  - Verify: Source inspection of `crates/flow/src/resolver.rs` and `crates/engine/src/canonical_paths.rs`; `rg -n "resolve_with_contract|CanonicalLayoutContract|default_canonical_layout_contract" crates/flow/src/resolver.rs crates/flow/src/lib.rs crates/engine/src/canonical_paths.rs`.
  - Files: `crates/flow/src/resolver.rs`, `crates/flow/src/lib.rs`

## Packet 2.2: Resolver Adoption And Test Coverage

- [ ] Task: Adopt the supplied canonical layout contract in resolver loading and contract-dependent fallback/path behavior
  - Acceptance: the supported contract-aware flow path no longer depends on unconditional `CanonicalArtifacts::load(...)`; any contract-dependent fallback/path payloads that would otherwise force default `.handbook/**` behavior now derive from the active engine-owned contract.
  - Verify: Source inspection of `crates/flow/src/resolver.rs` plus `cargo test -p handbook-engine --test canonical_artifacts_ingest && cargo test -p handbook-engine --test baseline_validation && cargo test -p handbook-flow --test resolver_core`.
  - Files: `crates/flow/src/resolver.rs`, `crates/engine/src/canonical_artifacts.rs` (only if a narrow engine-boundary adjustment proves necessary)

- [ ] Task: Update only the inseparable refusal/blocker summary or path surfaces needed for structural honesty
  - Acceptance: the flow public API no longer reports contract-dependent system-root/path results as if the caller were still using the default `.handbook/**` layout. Broader `.handbook` wording cleanup remains deferred to Set 3.
  - Verify: Source inspection of the refusal/blocker sections in `crates/flow/src/resolver.rs`; `rg -n "canonical \.handbook root|default_canonical_layout_contract\(\)\.system_root_relative\(" crates/flow/src/resolver.rs` cross-checked against the active contract-aware path.
  - Files: `crates/flow/src/resolver.rs`, `crates/flow/tests/resolver_core.rs`

- [ ] Task: Add positive non-default canonical-layout coverage and refresh boundary authority if needed
  - Acceptance: `crates/flow/tests/resolver_core.rs` proves at least one successful and one blocked/refusal path using a non-default canonical root, and `docs/specs/handbook-flow-import-boundary-consumer-contract.md` is refreshed if the public flow symbol story or dependency story changed.
  - Verify: `cargo test -p handbook-flow --test resolver_core` plus source inspection of `docs/specs/handbook-flow-import-boundary-consumer-contract.md` when touched.
  - Files: `crates/flow/tests/resolver_core.rs`, `docs/specs/handbook-flow-import-boundary-consumer-contract.md`

## Packet 2.3: Final Set Proof

- [ ] Task: Run the Set 2 verification wall
  - Acceptance: all of the following pass:
    - `cargo test -p handbook-engine --test canonical_artifacts_ingest`
    - `cargo test -p handbook-engine --test baseline_validation`
    - `cargo test -p handbook-flow --test resolver_core`
    - `cargo test -p handbook-flow`
    - `cargo check --workspace`
    - `cargo fmt --all -- --check`
    - `cargo clippy --workspace --all-targets -- -D warnings`
  - Verify: Run each command and record pass/fail in the completion notes below.
  - Files: `docs/specs/handbook-substrate-parameterization-set-2-flow-canonical-layout-injection-tasks.md`

- [ ] Task: Record the residual-default inventory honestly and keep Packet 2.3 proof-only
  - Acceptance: the completion notes explicitly distinguish:
    - acceptable retained `.handbook` references that are still Set 3 honesty-cleanup territory
    - structural blockers that prove Packet 2.1 or Packet 2.2 must be reopened
    Packet 2.3 must not silently absorb unfinished structural work.
  - Verify: `rg -n "CanonicalArtifacts::load\(|load_with_contract|default_canonical_layout_contract|canonical \.handbook root|\.handbook" crates/flow/src crates/flow/tests crates/engine/src crates/engine/tests` plus source inspection cross-referenced against the active contract-aware flow surface.
  - Files: `docs/specs/handbook-substrate-parameterization-set-2-flow-canonical-layout-injection-tasks.md`

#### Packet 2.3 completion notes

- Verification wall:
  - PENDING — `cargo test -p handbook-engine --test canonical_artifacts_ingest`
  - PENDING — `cargo test -p handbook-engine --test baseline_validation`
  - PENDING — `cargo test -p handbook-flow --test resolver_core`
  - PENDING — `cargo test -p handbook-flow`
  - PENDING — `cargo check --workspace`
  - PENDING — `cargo fmt --all -- --check`
  - PENDING — `cargo clippy --workspace --all-targets -- -D warnings`
- Residual bounded-default inventory:
  - PENDING — record acceptable Set 3-deferred `.handbook` wording/default helpers here.
  - PENDING — record any evidence that requires reopening Packet 2.1 or Packet 2.2 here.
- Proof-only rule:
  - If verification reveals unfinished structural work from Packet 2.1 or Packet 2.2, stop and reopen the relevant earlier packet explicitly instead of turning Packet 2.3 into an implementation sink.

---

## Stop Boundary

Stop after Packet 2.3 for this set. Do not:

- reopen Set 1 (`handbook-pipeline` import layout)
- start Set 3 (import-surface honesty cleanup)
- widen into CLI/compiler/product-shell cleanup
- execute the actual Substrate import
- generalize the flow boundary into a new multi-layout platform beyond the engine-owned canonical contract
