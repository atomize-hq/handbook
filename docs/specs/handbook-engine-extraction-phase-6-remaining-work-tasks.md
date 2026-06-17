# Tasks: Handbook Engine Extraction — Phase 6 Remaining Work

Plan reference: [handbook-engine-extraction-phase-6-remaining-work-plan.md](./handbook-engine-extraction-phase-6-remaining-work-plan.md)

Spec reference: [handbook-engine-extraction-phase-6-remaining-work-spec.md](./handbook-engine-extraction-phase-6-remaining-work-spec.md)

---

## Lane B: Flow Required-Import Boundary Cleanup + Contract Freeze

### Packet 6.B.1: Gather Evidence

- [ ] Task: Capture dependency-tree evidence for `handbook-flow`
  - Acceptance: `cargo tree -p handbook-flow` output recorded, showing only `handbook-engine` as the intra-workspace dependency.
  - Verify: `cargo tree -p handbook-flow`
  - Files: `docs/specs/handbook-flow-import-boundary-consumer-contract.md` (evidence section)

- [ ] Task: Capture coupling-exclusion evidence
  - Acceptance: `rg` output recorded, showing zero matches for `handbook_compiler`, `handbook_cli`, `handbook_pipeline` in `crates/flow/src/` and `crates/flow/tests/`.
  - Verify: `rg -n "handbook_compiler|handbook_cli|handbook_pipeline" crates/flow/src/ crates/flow/tests/`
  - Files: `docs/specs/handbook-flow-import-boundary-consumer-contract.md` (evidence section)

- [ ] Task: Trace transitive type dependencies for all in-boundary symbols
  - Acceptance: For each public symbol in the in-boundary set (from `resolver`, `budget`, `packet_result`), its type dependencies are recorded and confirmed to resolve only to `handbook-engine` public types, std types, or flow-local types.
  - Verify: Source inspection of `crates/flow/src/resolver.rs`, `crates/flow/src/budget.rs`, `crates/flow/src/packet_result.rs` cross-referenced against `crates/engine/src/lib.rs` exports.
  - Files: `docs/specs/handbook-flow-import-boundary-consumer-contract.md` (transitive dependency table)

- [ ] Task: Record residual shell-ownership leakage separately from the clean import boundary proof
  - Acceptance: The evidence section explicitly records that the crate/type dependency boundary is clean **and** that final shell-owned/operator-facing copy still leaks through the live flow import surface. It must distinguish typed next-action/status semantics that may remain machine-readable from final shell wording/command strings that Packet 6.B.2 must move out.
  - Verify: Source inspection cross-referencing `crates/flow/src/resolver.rs`, `crates/flow/src/packet_result.rs`, `crates/cli/src/rendering.rs`, and `crates/compiler/src/rendering/shared.rs`.
  - Files: `docs/specs/handbook-flow-import-boundary-consumer-contract.md` (evidence section)

### Packet 6.B.2: Clean Flow Import-Surface Shell Ownership

- [ ] Task: Remove final shell-owned/operator-facing copy from the public flow import surface
  - Acceptance: `handbook-flow` no longer returns final shell command strings or product-shell action wording from its public import surface. Any remaining next-action data exposed by flow is typed/machine-readable rather than final rendered shell copy.
  - Verify: Source inspection of `crates/flow/src/resolver.rs` and `crates/flow/src/packet_result.rs`; `rg -n 'run `doctor`|handbook inspect --packet|handbook generate --packet|handbook setup' crates/flow/src/`; `cargo test -p handbook-flow`.
  - Files: `crates/flow/src/resolver.rs`, `crates/flow/src/packet_result.rs`, impacted flow tests if needed

- [ ] Task: Keep CLI/compiler responsible for final shell rendering without widening into redesign
  - Acceptance: Any caller-side changes stay narrowly bounded to the rendering/adapter files required by the flow cleanup. Typed next-action/status semantics may remain, but final shell copy is rendered outside flow. No broader CLI shell redesign is introduced.
  - Verify: Source inspection of `crates/cli/src/rendering.rs`, `crates/compiler/src/rendering/shared.rs`; `cargo check --workspace`.
  - Files: `crates/cli/src/rendering.rs`, `crates/compiler/src/rendering/shared.rs`, impacted tests if needed

### Packet 6.B.3: Formalize Consumer Contract

- [ ] Task: Write the `handbook-flow` import-boundary consumer contract document against the cleaned surface
  - Acceptance: A standalone doc at `docs/specs/handbook-flow-import-boundary-consumer-contract.md` records:
    - The frozen in-boundary symbol set (public re-exports from `budget`, `packet_result`, `resolver`)
    - Their transitive type dependencies (engine-public, std, or flow-local only)
    - Which typed next-action/status semantics remain in-boundary after Packet 6.B.2
    - Which shell-owned/operator-facing copy and rendering responsibilities are explicitly out of boundary
    - The contract version function (`flow_contract_version()`)
    - Evidence references from Packet 6.B.1 and cleanup references from Packet 6.B.2
  - Verify: Doc exists and is internally consistent with live source.
  - Files: `docs/specs/handbook-flow-import-boundary-consumer-contract.md`

### Packet 6.B.4: Verification Wall

- [ ] Task: Run the Lane B verification wall
  - Acceptance: All of the following pass:
    - `cargo tree -p handbook-flow` shows only `handbook-engine` as the intra-workspace dependency
    - `rg -n "handbook_compiler|handbook_cli|handbook_pipeline" crates/flow/src/ crates/flow/tests/` returns zero matches
    - Source inspection of the public `handbook-flow` surface (`crates/flow/src/lib.rs`, `crates/flow/src/budget.rs`, `crates/flow/src/packet_result.rs`, `crates/flow/src/resolver.rs`) confirms no final shell-owned/operator-facing copy remains on that surface; any remaining next-action/status data is typed/machine-readable only and stays within the Packet 6.B.3 consumer contract
    - `rg -n 'run `doctor`|handbook inspect --packet|handbook generate --packet|handbook setup' crates/flow/src/` returns zero matches as a supporting spot-check, not the sole proof
    - `cargo test -p handbook-flow` passes
    - `cargo check --workspace` passes
    - `cargo fmt --all -- --check && cargo clippy --workspace --all-targets -- -D warnings` passes
  - Verify: Run each command, perform and record the required source inspection, and record pass/fail for both the broader surface proof and the supporting grep spot-check.
  - Files: `docs/specs/handbook-engine-extraction-phase-6-remaining-work-tasks.md` (completion notes)

---

## Lane C: Engine Optional Boundary Freeze (Optional — Currently Deferred)

### Packet 6.C.1: Defer Or Activate (Decision Task)

- [ ] Task: Record Lane C deferral decision
  - Acceptance: The tasks doc explicitly records that Lane C is deferred — engine's current public surface is the working boundary. If later review (e.g., from Lane D's import plan) indicates a narrower surface is needed, Lane C can be activated at that time.
  - Verify: This section states "deferred" with rationale.
  - Files: `docs/specs/handbook-engine-extraction-phase-6-remaining-work-tasks.md`

---

## Lane D: Final Substrate Import Plan

### Packet 6.D.1: Write Import/Adoption Plan

- [ ] Task: Write the phased import plan for engine + pipeline + flow
  - Acceptance: A standalone doc at `docs/specs/handbook-substrate-import-adoption-plan.md` records:
    - Import order: engine first (no intra-workspace deps), then pipeline (depends on engine), then flow (depends on engine)
    - Rationale for the phased order
    - Per-crate frozen boundary summary:
      - Engine: current public surface (Lane C deferred)
      - Pipeline: documented frozen subset from Lane A closeout (in-boundary modules listed)
      - Flow: Lane B consumer contract (clean import surface, typed semantics only where contract-approved, final shell copy out of boundary)
    - Adapter/facade assessment (current evidence: none needed beyond the Lane B cleanup; record the assessment with evidence)
    - Import verification gate per phase (what checks Substrate must pass after importing each crate)
    - Substrate-side constraints (resolved from live repo inspection, 2026-06-17):
      - License field: add `license = "MIT"` to the three crate Cargo.toml files before import
      - Workspace integration: recommend workspace member pattern (path deps) vs external dep
      - YAML crate divergence: `serde_yaml_bw` (handbook) vs `serde_yaml` (substrate) — record keep-both or migrate decision
      - No feature flags needed; edition/resolver/sha2/libc/serde all compatible
  - Verify: Doc exists and is consistent with the three crate surfaces and the Lane B consumer contract.
  - Files: `docs/specs/handbook-substrate-import-adoption-plan.md`

### Packet 6.D.2: Human Review Gate

- [ ] Task: Human review of the import plan
  - Acceptance: The plan has been reviewed by a human engineer who confirms it is consistent with live crate surfaces, frozen boundaries, and the root plan's migration gate. Any review feedback is addressed.
  - Verify: Human sign-off recorded in this tasks doc.
  - Files: `docs/specs/handbook-engine-extraction-phase-6-remaining-work-tasks.md` (completion note)

---

## Wider-Seam Guardrail

Stop after Lane B and Lane D land. Do not:
- Execute the actual Substrate import
- Reopen Lane A
- Widen into full CLI shell redesign, compiler retirement, publication, or crates.io work
- Make `substrate-context` become handbook
- Introduce compatibility aliases as a long-term architecture substitute

Those remain outside Phase 6 scope and require separate authority.

---

## Lane Status Summary

| Lane | Status | Blocks Lane D? |
|------|--------|----------------|
| A | Closed (2026-06-17) | N/A — done |
| B | Pending | Yes |
| C | Deferred (optional) | No |
| D | Pending (after B) | — |
