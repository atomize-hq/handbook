# HCM-3.5-P0 implementation-admission preflight and seam inventory

## Bound identity and local baseline

- Bound task/thread: `019fda6a-f05f-7a23-ad68-89b30f372578`; host: `local`.
- Meta task/thread: `019fda65-3892-7900-a692-2dd0a0fccc7a`; workflow:
  `handbook-hcm-3-5-implementation-20260807`.
- Dispatch nonce:
  `d9432121a0209ee059502e74ed1480cb5ccd7c74d5c639a070d98b82b4d2571c`.
- Assigned root: `C:/Users/spmcc/.codex/worktrees/6383/handbook`.
- Observed detached `HEAD` and local target ref:
  `0aa11d2851caee63f373c19c394925ec70473a8a`.
- Observed tree: `ebbddc19ad869f108e021ac31674fd5b5efde4ba`; required ancestor
  `1a10bfd59e8433fa344dd2baa4880b4b87c7f8f6` is present; task worktree was
  clean before Packet 0 edits.
- The protected checkout is a distinct worktree at
  `C:/Users/spmcc/Documents/__Project_Code/handbook`, observed at the required
  ancestor and never mutated. No remote query, fetch, pull, push, or other
  remote operation was performed.

## Independent current seam facts

| Seam | Observed current truth | Packet 0 decision effect |
|---|---|---|
| HCM-3.4 source pair | `crates/engine/src/lib.rs` keeps `snapshot_memory` and `projection` private. `projection.rs` validates exact source pair/state provenance and all five currentness families; `snapshot_memory/delta.rs` owns the relation-only `signals` set. | Keep the private modules private; add only a later engine-owned public grounding facade. |
| Context Resolution | `ContextResolutionEnvelope` is public; its private Projection authority view requires current authority and preserves six dimensions/ranks. | It is the exact public envelope input to `GroundingRequest`; no caller-supplied rank/filter substitute. |
| Flow | `resolve` and `resolve_with_contract` build `ResolverResult` through private `build_packet_result`; CLI and compiler call the retained resolver. | Add no fields or branches. A later separate Flow operation accepts `FlowPacketGrounding`. |
| Pipeline | `compile_pipeline_stage*` reaches `assemble_documents`, `load_repo_relative_document`, and `filter_scoped_blocks` using `work_level`; CLI/compiler tests call the retained compiler path. | Add no overload or fallback. A later separate inclusion operation accepts `SharedResolutionInclusion`. |
| Handoff | v1.4 handoff schema has nullable prior/start/grounding/end/delta snapshot reference fields. | Preserve descriptive refs only; no schema edit, copied snapshot, or gate decision. |
| SDK/CLI/Substrate | Workspace has engine, flow, pipeline, compiler, and CLI crates. No `handbook-sdk` crate or Substrate checkout exists; CLI currently depends directly on owner and compiler crates. | Record only future composition: SDK over owner operations, standalone CLI over SDK, and exact published crates.io consumption/wrapping by Substrate. |

## Tooling and admission result

GitNexus MCP, CLI, and index are unavailable in this checkout. That condition
is recorded as unavailable rather than GREEN. Packet 0 edits documentation
only, so no code symbol is edited and no code-symbol impact result is claimed.
The new Packet 1 selector must repeat source/caller inspection and run impact
analysis before every existing symbol edit.

The above facts admit only the `handbook_engine::grounding` contract in the
Packet 0 decision. They do not admit implementation, source visibility change,
Flow/pipeline adoption, SDK/CLI/Substrate composition, package publication,
or a gate runtime.
