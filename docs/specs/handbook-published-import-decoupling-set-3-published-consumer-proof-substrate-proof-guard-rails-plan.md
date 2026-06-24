# Plan: Handbook Published-Import Decoupling — Set 3: Published Consumer Proof + Substrate Proof + Guard Rails

Spec reference: [handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-spec.md](./handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-spec.md)

## Overview

Set 3 closes the workstream by turning the Set 2 public boundary from a **packaged local truth** into a **released external truth** and a **real downstream Substrate truth**, then adding guard rails so the repo cannot drift back into false-complete claims.

This set is not about designing a broader API. The active public surface was already bounded in Set 2. Set 3 is about proving that exact surface honestly and reproducibly against the MAP objective.

## Current State (live repo truth)

- The active Set 2 boundary exists in live `system` source and the packaged-boundary proof passes locally.
- `tools/proof/handbook_pipeline_minimal_boundary.sh` proves the Set 2 retained public boundary from an unpacked packaged artifact, not from crates.io.
- The current Substrate proof worktree at `/Users/spensermcconnell/.codex/worktrees/substrate-packet-4-2-20260622-133054` resolves:
  - `handbook-engine v0.1.1`
  - `handbook-flow v0.1.1`
  - `handbook-pipeline v0.1.1`
- The current downstream proof seam in `/Users/spensermcconnell/.codex/worktrees/substrate-packet-4-2-20260622-133054/crates/shell/src/execution/prompt_fulfillment.rs` imports only `handbook_engine` + `handbook_flow`; it does **not** yet prove `handbook-pipeline` adoption.
- `cargo publish --dry-run -p handbook-pipeline` passes in `system`, but crates.io still already contains `handbook-pipeline 0.1.1`, so Set 3 needs a new released version before released-consumer proof can honestly count.
- The archived Set 3 parameterization docs remain provenance only and should not control the active packet structure.

## Intended Consumer Shape For Set 3

Set 3 has two proof consumers that must stay distinct:

1. **Released external consumer**
   - an out-of-tree scratch or fixture consumer pinned to the exact published crates.io version
   - exercises the retained Set 2 public families through public APIs only
   - proves the released boundary is real for a non-Substrate consumer

2. **Downstream Substrate consumer**
   - one narrow Substrate-owned production seam in a dedicated worktree
   - proves Substrate can use the capability it actually needs through the published boundary
   - preserves Substrate-owned wording, integration behavior, and runtime experience

The goal is not to make Substrate call every public function. The goal is to prove that the exact capabilities Substrate truly needs are reachable through the reviewed published boundary, while the broader retained families remain externally provable and honestly classified.

## Delivery Strategy

Set 3 should proceed in four sequential packets:

```text
Packet 3.1 (released-proof harness + release prep)
  -> Packet 3.2 (published crates.io proof)
  -> Packet 3.3 (downstream Substrate proof in dedicated worktree)
  -> Packet 3.4 (guard rails + honest closeout)
```

These packets are intentionally sequential because downstream proof must target a real published version, and the guard rails must encode the exact released proof and downstream proof shapes that actually landed.

## Packet Plan

## Packet 3.1 — Released-Proof Harness + Release Preparation

### Goal

Prepare the proof and release surfaces required to turn Set 2's packaged proof into a released proof, without widening the public boundary.

### Work

- add a new released-crate proof harness separate from the Set 2 packaged harness
- pin the external consumer fixture to an exact crates.io version instead of a packaged path dependency
- preserve the Set 2 packaged harness as a distinct proof tier rather than overwriting it
- determine the smallest honest version-release shape needed to publish the Set 2 boundary
- keep Packet 4.2 classified only as `engine + flow` proof in the active docs

### Verification checkpoint

Packet 3.1 is done only when:

- the repo has a clear released-boundary proof harness path and fixture location
- the harness is designed to fail if it falls back to sibling-path or source-tree dependency accidents
- the publish target version strategy is explicit enough to support the next packet
- no public API widening has been smuggled into release-prep work

### Packet 3.1 closeout note (2026-06-23)

- The released proof tier now has its own harness at `tools/proof/handbook_pipeline_released_boundary.sh` plus its own external fixture at `tests/fixtures/external_consumers/handbook_pipeline_released_boundary/`; the Set 2 packaged harness remains unchanged at `tools/proof/handbook_pipeline_minimal_boundary.sh`.
- The released fixture now pins `handbook-pipeline` to an exact crates.io version only, and the harness rejects path dependencies plus any `cargo metadata` resolution that points back into the local `system` source tree.
- `handbook-pipeline 0.1.1` is not an honest released-proof target because crates.io already contains that version while the retained Set 2 boundary being proved still lives only in the current unpublished source/package.
- The smallest honest publish target for Packet 3.2 is `handbook-pipeline 0.1.2` only. Packet 3.1 did not uncover any release coupling that would require a coordinated `handbook-engine` or `handbook-flow` version train.

## Packet 3.2 — Published Crates.io Proof

### Goal

Make the Set 2 boundary real on crates.io and prove it from an external exact-version consumer.

### Work

- publish the first crates.io version that actually contains the Set 2 `handbook-pipeline` public boundary
- run the released external consumer proof against that exact published version
- confirm the proof exercises the retained public capability families through public APIs only
- classify the proof honestly as released external proof, not downstream Substrate proof

### Verification checkpoint

Packet 3.2 is done only when:

- crates.io exposes the required Set 2 public boundary in a real published version
- the released external consumer proof passes against that exact version
- the proof does not import private-module paths or use path-based dependency accidents
- the docs still distinguish Set 2 packaged proof from Set 3 released proof

## Packet 3.3 — Downstream Substrate Published-Boundary Proof

### Goal

Prove one narrow real Substrate seam can use the published `handbook-pipeline` boundary while keeping wording and runtime behavior Substrate-owned.

### Work

- choose one narrow downstream seam in a dedicated Substrate worktree
- pin the worktree to the exact published crates.io version proved in Packet 3.2
- implement only the smallest downstream call site or provider/context boundary needed to demonstrate the real required capability
- keep final wording, runtime behavior, and downstream UX owned by Substrate
- record which retained `handbook-pipeline` capability families are actually consumed by the chosen downstream seam and which remain externally proved but unused downstream

### Verification checkpoint

Packet 3.3 is done only when:

- the downstream proof runs only in a dedicated Substrate worktree
- `cargo tree` in that worktree resolves the exact published version rather than sibling-path overrides
- the chosen seam uses published public APIs only
- the proof is honest about what downstream capability was exercised and what was not
- no main-checkout downstream edits were used to obtain the proof

## Packet 3.4 — Guard Rails + Honest Closeout

### Goal

Lock in the released proof and downstream proof so future work cannot regress into false-complete classification or unpublished dependency accidents.

### Work

- add proof/update guard rails that re-run released-boundary proof against the published version
- add documentation or verification rails that keep Packet 4.2 classified only as `engine + flow` proof
- add closeout notes that explicitly distinguish:
  - Set 2 packaged proof
  - Set 3 released external proof
  - Set 3 downstream Substrate proof
- confirm no extra public surface was added beyond the Set 2 retained/dropped matrix

### Verification checkpoint

Packet 3.4 is done only when:

- the released-boundary proof is rerunnable and version-pinned
- the downstream proof claim is tied to an exact seam and exact published version
- active docs cannot honestly overclaim `handbook-pipeline` proof by pointing only at Packet 4.2
- Set 3 closeout language satisfies the MAP intent and does not silently widen scope

## Sequential vs Parallel Notes

- **Not parallel-safe by default:** Packets 3.1–3.4 share the same published version target, proof classification language, and downstream seam assumptions.
- Packet 3.2 depends on 3.1 because the released proof harness must exist before publish-proof execution.
- Packet 3.3 depends on 3.2 because the downstream proof must target a real published version, not a packaged artifact.
- Packet 3.4 depends on 3.2 and 3.3 because the guard rails must encode the exact released proof and downstream proof that actually landed.
- Low-risk documentation phrasing cleanup may be parallelized only after the released version and downstream proof seam are stable.

## Risks And Mitigations

### Risk 1: publishing before the proof shape or target version is stable

- **Why it matters:** a premature publish could force another release just to repair proof harness shape or exact-version pinning mistakes.
- **Mitigation:** complete Packet 3.1 first, including the released-proof harness and the explicit `handbook-pipeline 0.1.2`-only target unless a later packet finds real release coupling.

### Risk 2: downstream proof drifts into product redesign

- **Why it matters:** Set 3 is a proof set, not a broad Substrate runtime redesign.
- **Mitigation:** choose one narrow downstream seam and keep the proof limited to the minimum call site or provider/context boundary required to show real capability.

### Risk 3: proof falls back to unpublished/path dependency accidents

- **Why it matters:** that would recreate the exact false-complete class the MAP is trying to prevent.
- **Mitigation:** exact crates.io version pins, `cargo tree` verification, and a released-boundary harness designed to fail if path fallbacks appear.

### Risk 4: API widening sneaks in under proof pressure

- **Why it matters:** expanding the public surface in Set 3 would violate the MAP intent of minimum unnecessary public surface.
- **Mitigation:** treat the Set 2 retained/dropped matrix as frozen unless authority is explicitly reopened; if the boundary proves insufficient, stop and reopen authority instead of silently widening.

### Risk 5: Packet 4.2 gets overclaimed again

- **Why it matters:** the current downstream baseline proves only `engine + flow`, and relabeling it as pipeline proof would invalidate the honesty contract.
- **Mitigation:** require every Set 3 proof artifact and closeout note to classify Packet 4.2 explicitly as `engine + flow` only.

## Verification Wall

Before Set 3 can be called complete, all of the following must be true together:

1. the Set 2 targeted regression wall still passes in `system`
2. the released external proof passes against an exact published crates.io version
3. the downstream Substrate proof passes in a dedicated worktree against that same published version
4. the docs still preserve the MAP ownership split and the MAP minimum-surface rule
5. the guard rails can catch the three banned regressions:
   - externally private but internally configurable
   - docs overclaiming the released surface
   - `engine + flow` proof being mislabeled as pipeline proof

## Completion Standard

Set 3 is complete only when:

- the released `handbook-pipeline` version is real on crates.io,
- the released external consumer proof is passing,
- the dedicated-worktree downstream Substrate proof is passing,
- the proof classification is explicit and honest,
- Packet 4.2 remains clearly limited to `engine + flow`,
- no new public surface was added outside the Set 2 retained/dropped matrix,
- and the closeout notes show that the full MAP objective is finally satisfied through a reviewed, stable, published boundary.
