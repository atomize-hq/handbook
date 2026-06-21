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

- Status: ACCEPTED — per the user's explicit instruction not to reopen earlier packets, Packet 2.3 normalized the existing flow formatting with `cargo fmt --all`, reran the full Set 2 proof wall, and the wall is now clean without widening beyond proof notes plus formatting-only cleanup in the already-landed flow files.
- Packet 2.1 / 2.2 landing check:
  - PASS — live source and recent history still show the intended structural seam already landed: `crates/flow/src/lib.rs` publicly re-exports `resolve_with_contract(...)`; `crates/flow/src/resolver.rs` keeps `resolve(...)` as the explicit default wrapper while loading through `CanonicalArtifacts::load_with_contract(...)`; and `crates/flow/tests/resolver_core.rs` contains non-default contract coverage. Relevant commits observed before proof: `25a01e6`, `91f91db`, `3437650`, `c369121`.
- Verification wall:
  - PASS — `cargo test -p handbook-engine --test canonical_artifacts_ingest`
  - PASS — `cargo test -p handbook-engine --test baseline_validation`
  - PASS — `cargo test -p handbook-flow --test resolver_core`
  - PASS — `cargo test -p handbook-flow`
  - PASS — `cargo check --workspace`
  - PASS — `cargo fmt --all -- --check` after first normalizing `crates/flow/src/resolver.rs` and `crates/flow/tests/resolver_core.rs` with `cargo fmt --all`
  - PASS — `cargo clippy --workspace --all-targets -- -D warnings`
- Residual bounded-default inventory:
  - PASS — `rg -n "CanonicalArtifacts::load\\(|load_with_contract|default_canonical_layout_contract|canonical \\.handbook root|\\.handbook" crates/flow/src crates/flow/tests crates/engine/src crates/engine/tests` completed and the remaining hits stay bounded.
  - Acceptable retained `.handbook` references that remain Set 3 honesty-cleanup or default-layout territory:
    - `crates/flow/src/resolver.rs` still keeps the explicit default wrapper via `*default_canonical_layout_contract()` and internal/default-labeled helper text such as `c03.handbook_root`; this does not make the supported contract-aware flow seam dishonest because `resolve_with_contract(...)` and its non-default tests already use the supplied contract.
    - `crates/engine/src/canonical_paths.rs` and `crates/engine/src/canonical_artifacts.rs` still define the engine-owned default `.handbook/**` contract and default-layout diagnostics; those hits are expected default-layout ownership, not evidence that flow still depends on default-only loading for the supported injected path.
    - `crates/flow/tests/resolver_core.rs` plus engine tests still contain many `.handbook` literals in default-layout fixtures, authored markdown samples, and default-path assertions; these are fixture/test/default-surface references, not proof that the public contract-aware seam regressed.
  - Structural blocker inventory:
    - No residual-default sweep evidence shows Packet 2.1 or Packet 2.2 missed the contract-aware flow seam itself; the supported non-default path, default-wrapper behavior, and non-default resolver coverage are all present.
    - No remaining Packet 2 structural blocker was observed after the formatting normalization and clean verification rerun.
- Proof-only rule:
  - Packet 2.3 stayed proof-bounded: no Set 1 / Set 3 work was touched, no semantic resolver/engine changes were introduced, and the only non-doc changes were the formatting-only cleanup needed to let the verification wall speak honestly.

---

## Stop Boundary

Stop after Packet 2.3 for this set. Do not:

- reopen Set 1 (`handbook-pipeline` import layout)
- start Set 3 (import-surface honesty cleanup)
- widen into CLI/compiler/product-shell cleanup
- execute the actual Substrate import
- generalize the flow boundary into a new multi-layout platform beyond the engine-owned canonical contract
