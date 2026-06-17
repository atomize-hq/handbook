# Phase 6 Remaining Work — Scratch Status

## Current live status

- **Lane A**: closed.
- **Lane B**: authority unblocked and re-shaped; execution still pending.
- **Lane C**: deferred / optional.
- **Lane D**: blocked on Lane B completion.

## Adopted Lane B direction

The authority decision is now made:

- `handbook-flow` is a **required** import target alongside `handbook-engine` and `handbook-pipeline`.
- `handbook-cli` remains the only product shell in this phase; Lane B is not a shell redesign.
- The old scratch framing of “Option A vs Option B” is retired.
- The active direction is the narrow production cleanup seam before consumer-contract formalization.

## Lane B packet structure now in force

1. **Packet 6.B.1 — Gather Evidence**
   - preserve the honest proof that `handbook-flow -> handbook-engine` crate coupling is already clean
   - record transitive type-dependency truth
   - separately record the residual shell-owned/operator-facing copy still leaking through the flow surface
2. **Packet 6.B.2 — Clean Flow Import-Surface Shell Ownership**
   - narrow production seam
   - remove final shell-owned/operator-facing copy from the public flow surface
   - keep the cleanup bounded; no broad CLI redesign
3. **Packet 6.B.3 — Formalize Consumer Contract**
   - freeze the cleaned import surface
   - explicitly record what typed semantics remain in-boundary vs what shell-owned copy is out of boundary
4. **Packet 6.B.4 — Verification Wall**
   - rerun dependency proof, shell-copy proof, tests, workspace check, fmt, and clippy

## Live repo truth the new lane shape is built around

- `handbook-flow` still has clean crate coupling: `handbook-flow -> handbook-engine` only.
- `crates/flow/src/resolver.rs` still exposes typed next-safe-action semantics and ready-packet command strings.
- `crates/flow/src/packet_result.rs` still exposes `PacketDecisionSummary.ready_next_safe_action: String` on the public flow surface.
- `crates/cli/src/rendering.rs` and `crates/compiler/src/rendering/shared.rs` already render shell wording for many next-safe-action cases, so the likely cleanup seam is moving the remaining final shell copy out of flow rather than redesigning the decision model.

## Active boundary decision

- **Typed next-action/status semantics may remain** on the flow surface if they stay machine-readable and import-safe.
- **Final shell-owned/operator-facing copy must move out** of the flow import surface before the consumer contract is frozen.
- CLI/compiler remain responsible for rendering final shell wording and commands.

## Lane D dependency update

Lane D now waits for **Packets 6.B.1–6.B.4** instead of the old three-packet Lane B shape.
Its flow boundary summary should reference the cleaned consumer contract, not the pre-cleanup evidence posture.
