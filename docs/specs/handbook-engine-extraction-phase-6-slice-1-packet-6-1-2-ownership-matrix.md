# Packet 6.1.2 Ownership Matrix And Boundary Reassessment

This note records the Packet 6.1.2 crate-boundary reassessment for Phase 6 Slice 1.

## Scope guard

- Packet scope: ownership/readiness reassessment only.
- No production code was changed.
- This note builds on committed Packet 6.1.1 truth at `30b22d5` (`Record Packet 6.1.1 migration-gate truth freeze`), which itself froze the representative migration-gate baseline from committed HEAD `5644ff7`.
- At Packet 6.1.2 capture time, the live working tree also contained unrelated local edits in `AGENTS.md` and `CLAUDE.md`; those edits were preserved and were not part of Packet 6.1.2.

## Evidence used

```bash
rg -n "pub use|pub mod|mod " crates/engine/src/lib.rs crates/pipeline/src/lib.rs crates/flow/src/lib.rs crates/compiler/src/lib.rs crates/cli/src/main.rs
cargo tree -p handbook-engine
cargo tree -p handbook-pipeline
cargo tree -p handbook-flow
cargo tree -p handbook-compiler
```

## Crate-by-crate ownership matrix

| Crate | Live evidence | Center of gravity | Should Substrate likely import it through a clean boundary? | Should ownership remain handbook-side longer? | Remaining handbook-product assumptions |
| --- | --- | --- | --- | --- | --- |
| `handbook-engine` | `crates/engine/src/lib.rs` exports canonical artifacts, authored-truth parsing/rendering/validation, baseline validation, freshness, and the canonical layout contract; `cargo tree -p handbook-engine` shows only foundational serde/yaml/hash dependencies | Handbook-domain core | Yes. The outward surface is already narrow and reusable enough to consume through an adapter boundary. | Yes. Nothing in the live surface proves substrate-specific ownership, so the root rule still points to handbook-owned/imported. | `handbook_product_canonical_layout_contract` and the charter/project-context/environment-inventory vocabulary keep explicit handbook semantics at the boundary. |
| `handbook-pipeline` | `crates/pipeline/src/lib.rs` exports declarative catalog loading, compile/capture/handoff, route-state, and setup surfaces; `cargo tree -p handbook-pipeline` shows the expected runtime dependency on `handbook-engine` plus support crates, but it also still carries a dev-dependency on `handbook-compiler`, and `crates/pipeline/tests/pipeline_catalog.rs` imports `handbook_compiler::author::template_library` for test coverage. | Handbook-domain reusable pipeline with an intentionally bounded runtime wedge | Probably yes, but only through the reviewed boundary that already exists. The live runtime shape is narrow, but the remaining compiler-owned test coupling means the boundary is not yet fully clean. | Yes. The bounded supported-target wedge is still intentional, and the remaining compiler-owned test coupling is another reason handbook should likely keep ownership longer than a future importer. | Stage/consumer-specific posture is still visible through `stage_10_feature_spec_provenance`, trusted-session/provenance types, supported capture/handoff surfaces, and the retained compiler-backed test fixture path. |
| `handbook-flow` | `crates/flow/src/lib.rs` exports only `budget`, `packet_result`, and `resolver`; `cargo tree -p handbook-flow` depends only on `handbook-engine` | Handbook-domain middle-layer composition | Maybe later, through a small clean boundary, but not as a current move target. | Yes. This is still the least-settled handbook-side composition seam, so the honest call is to keep ownership handbook-side longer. | Packet/proof selection, budget, and resolver semantics still read as handbook workflow concepts rather than substrate-specific ones. |
| `handbook-cli` | `crates/cli/src/main.rs` is a 574-line clap entrypoint that wires shell-only modules, dynamic help, command dispatch, prompting, rendering, and exit policy | Handbook product shell | No. The live shell is something Substrate should consume around, not import as a reusable owner layer. | Yes. This should remain handbook-owned unless there is a separate future product decision. | The command tree, help text, prompting, rendering, and exit behavior are all explicit handbook product assumptions. |
| retained `handbook-compiler` seam | `crates/compiler/src/lib.rs` still re-exports author/setup/doctor/rendering/refusal/resolver adapters while depending on `handbook-engine`, `handbook-flow`, and `handbook-pipeline`; `cargo tree -p handbook-compiler` confirms it sits above the extracted crates | Compatibility/support glue, not the implementation center | No. This is not the import target and should not be treated as the future center of gravity. | Yes. It should remain handbook-side as retained transition glue until later work can retire or narrow it further. | CLI-facing support surfaces still matter here: author/setup/doctor adapters, rendering, refusal, and root-level compatibility exports remain product-adjacent. |

## Packet 6.1.2 assessment call

- No live crate surface currently proves a substrate-domain center of gravity.
- The bounded runtime wedge remains intentional and is not, by itself, a migration blocker.
- `handbook-cli` remains a product shell rather than a move target.
- Retained `handbook-compiler` should be described as compatibility/support glue, not as the implementation center.
- This packet stops at the ownership/readiness matrix and does not issue the final Phase 6 readiness verdict or author any ownership/import plan.
