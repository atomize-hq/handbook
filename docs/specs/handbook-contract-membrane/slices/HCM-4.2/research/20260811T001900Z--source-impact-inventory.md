# HCM-4.2 source, owner, and impact inventory

**Grounding commit:** `818d3662f57fcda867a59b7f9e035755fee1b393`
**Grounding tree:** `6dd3872e93bcad6604e78534d05e2ec14a0f8aea`
**Slice:** `HCM-4.2`

## GitNexus availability

The repository `AGENTS.md` requires GitNexus `query`, `context`, upstream
`impact`, and `detect_changes`. In this task session no GitNexus MCP server or
tool is configured: `list_mcp_resources({server: "gitnexus"})` returned
`resources/list failed: unknown MCP server 'gitnexus'`, GitNexus tools were
absent from the enabled tool registry, and the checkout has no `.gitnexus`
directory or `run.cjs` fallback. This evidence is **unavailable**, not GREEN.

The inventory below is therefore a read-only conventional source scan and
control-pack reconciliation. No existing Rust function, class, or method is
edited in HCM-4.2 planning, so the repository's pre-edit symbol-impact gate is
not applicable to the documentation-only change. Any future implementation
that edits an existing symbol must run upstream impact first and warn before a
HIGH/CRITICAL edit.

## Live SDK/source seams

| Seam | Current evidence | Future HCM-4.2 planning consequence |
|---|---|---|
| SDK facade | `crates/sdk/src/lib.rs:68-104` defines `HandbookSdkV1`, `open`, and typed `apply_posture_transition`; the same file exposes existing route methods and owner re-exports. | Add future shared DTO/discovery modules beneath SDK ownership; keep public methods typed and keep transport adapters outside the SDK contract. |
| Posture owner facade | `crates/engine/src/posture_transition_facade.rs:163` and `:548`; `crates/engine/src/charter_authority_transaction.rs:1869-1999`. | Preserve engine validation, transaction, canonical bytes, lifecycle, idempotency, approval, and refusal truth; expose only a bounded projection in a later implementation. |
| Private transaction path | `crates/engine/src/charter_authority_transaction.rs` contains crate-private `apply_posture_transition*` and the locked writer. | Do not export private records or solve reachability with visibility changes. |
| SDK dependency direction | `crates/sdk/Cargo.toml` depends on engine, flow, and pipeline; workspace members are CLI, engine, pipeline, flow, and SDK. | Future DTO/schema assets remain SDK-owned and owners do not depend back on SDK. |
| Existing serialization seams | Engine and pipeline use Serde/Serde JSON; engine already has `serde_json_canonicalizer` and a fingerprint type. | Reuse only as a future implementation seam after an exact source-of-truth decision; do not treat existing record serialization as the public DTO contract. |
| Existing generic values | Current owner/test code contains `serde_json::Value` at semantic/internal boundaries. | Public HCM-4.2 DTOs must not use unbounded `Value`; internal owner representations remain private and require explicit typed mapping. |
| Existing public error carriers | SDK re-exports `Blocker`, `Refusal`, and route-specific result/error types from `crates/sdk/src/lib.rs`. | Define shared public problem/refusal/error DTOs without erasing owner-specific refusal semantics; map by explicit closed discriminants. |

## Direct source/caller inventory

The direct typed posture call has one production SDK linkage from
`crates/sdk/src/lib.rs:84-89` to the SDK-held
`handbook_engine::PostureTransitionEngineFacadeV1`. Engine production call
sites are the facade and locked transaction path listed above; the engine
transaction tests at `crates/engine/src/charter_posture_transaction_tests.rs`
are proof consumers, not discoverable machine callers. Conventional scan found
no operation catalog, bootstrap descriptor, public JSON request/response
module, or generated schema asset in `crates/sdk`.

GitNexus direct-caller, process, and risk output is unavailable as stated
above. The future implementation packet must retain an honest unavailable
result unless the MCP/index is restored; it may not claim a synthetic LOW or
GREEN result from this scan.

## Exact canonical operation reconciliation

The authoritative `05` inventory contains 62 stable operation IDs. A
read-only script compared those IDs with the HCM-4.1 `tasks/plan.md` inventory:
both sets contain 62 entries; missing-from-plan and extra-in-plan sets are
empty. The table below retains the complete order and current owner/effect.
HCM-4.2 must not add IDs.

| # | Operation ID | Owner/effect from `05` | HCM-4.2 admission posture |
|---:|---|---|---|
| 1 | `capabilities.describe` | SDK registry / read-only bootstrap catalog | HCM-4.2 foundation; report only implemented definitions |
| 2 | `profile.list` | engine / read-only catalog | current owner candidate; exact profile/catalog binding |
| 3 | `profile.resolve` | engine / read-only | current owner candidate |
| 4 | `schema.list` | engine / read-only catalog | HCM-4.2 foundation; exact schema catalog |
| 5 | `schema.read` | engine / read-only schema document | HCM-4.2 foundation; exact schema retrieval |
| 6 | `vocabulary.read` | engine / read-only | current owner candidate |
| 7 | `resolution.stack.read` | engine / read-only | current owner candidate |
| 8 | `projection.definition.read` | flow over engine definitions / read-only | current owner candidate only where definition exists |
| 9 | `artifact.kind.list` | engine / read-only | current owner candidate |
| 10 | `artifact.instance.list` | engine / read-only | current owner candidate |
| 11 | `artifact.read` | engine / read-only | current owner candidate |
| 12 | `artifact.validate` | engine / read-only | current owner candidate |
| 13 | `artifact.render` | engine / read-only fixed renderer view | current owner candidate; not generic Projection |
| 14 | `intake.definition.read` | engine / read-only | current owner candidate |
| 15 | `intake.coverage.evaluate` | engine / read-only | current owner candidate |
| 16 | `intake.record.append` | engine + SDK transaction / append-only | current owner candidate; typed mutation proof required |
| 17 | `record.list` | engine / snapshot-bound catalog | HCM-4.2 catalog foundation; closed family selector |
| 18 | `record.read` | engine / typed record | current owner candidate; private records remain bounded |
| 19 | `artifact.candidate.validate` | engine / read-only | current owner candidate |
| 20 | `artifact.candidate.append` | engine + SDK transaction / append-only | current owner candidate |
| 21 | `artifact.approval.append` | engine + SDK transaction / append-only | current owner candidate |
| 22 | `artifact.candidate.promote` | engine + SDK transaction / compare-and-write | current owner candidate; owner CAS remains authoritative |
| 23 | `posture.resolve` | engine / read-only | current owner candidate; private result mapping required |
| 24 | `posture.recommendation.evaluate` | engine / read-only | current owner candidate; closed recommendation union |
| 25 | `posture.recommendation.append` | engine + SDK transaction / append-only | current owner candidate |
| 26 | `posture.recommendation.acknowledge` | engine + SDK transaction / append-only | current owner candidate |
| 27 | `posture.transition.apply` | engine + SDK transaction / compare-and-write | direct Rust precursor only; HCM-4.2 defines future discovery binding |
| 28 | `projection.create` | flow over engine definitions / read-only | current owner candidate only after Projection definition authority |
| 29 | `resolution.escalation.request.append` | engine + SDK transaction / append-only | current owner candidate |
| 30 | `resolution.escalation.disposition.append` | engine + SDK transaction / append-only | current owner candidate |
| 31 | `memory.promotion.request.append` | engine + SDK transaction / append-only | current owner candidate |
| 32 | `memory.promotion.disposition.append` | engine + SDK transaction / compare-and-write | current owner candidate; semantic-memory write conditional |
| 33 | `snapshot.capture` | SDK over engine/repository readers / append-only | future SDK orchestration seam; no implicit capture |
| 34 | `snapshot.read` | engine storage through SDK / read-only | current owner candidate; exact redaction/currentness |
| 35 | `snapshot.delta` | engine / read-only | current owner candidate |
| 36 | `snapshot.project` | flow over engine snapshot semantics / read-only | current owner candidate only with projection authority |
| 37 | `snapshot.verify_current` | SDK over engine comparison / read-only | future SDK orchestration seam |
| 38 | `snapshot.resolve_applicable` | SDK over engine comparison / read-only | future SDK orchestration seam |
| 39 | `repository.setup.plan` | SDK over engine/profile / read-only | existing SDK composition candidate |
| 40 | `repository.setup.apply` | SDK transaction / compare-and-write | existing SDK composition candidate; no DTO implementation here |
| 41 | `repository.doctor` | SDK over owners / read-only | existing SDK composition candidate |
| 42 | `flow.resolve` | flow / read-only | current owner candidate |
| 43 | `pipeline.catalog.list` | pipeline / read-only | current owner candidate; no transport adoption here |
| 44 | `pipeline.catalog.read` | pipeline / read-only | current owner candidate |
| 45 | `pipeline.route.resolve` | pipeline / read-only | current owner candidate |
| 46 | `pipeline.compile` | pipeline / in-memory result | current owner candidate; no write |
| 47 | `pipeline.capture.plan` | pipeline / read-only plan | current owner candidate |
| 48 | `pipeline.capture.apply` | pipeline + SDK transaction / compare-and-write | current owner candidate |
| 49 | `pipeline.handoff.emit` | pipeline + SDK transaction / append-only | current owner candidate |
| 50 | `pipeline.state.apply` | pipeline + SDK transaction / compare-and-write | current owner candidate |
| 51 | `contract.definition.list` | contracts / read-only | Phase-5 owner; catalog-deferred |
| 52 | `contract.definition.read` | contracts / read-only | Phase-5 owner; catalog-deferred |
| 53 | `contract.definition.append` | contracts + SDK transaction / append-only | Phase-5 owner; catalog-deferred |
| 54 | `contract.lifecycle.transition` | contracts + SDK transaction / compare-and-write | Phase-5 owner; catalog-deferred |
| 55 | `contract.evidence.list` | contracts / read-only | Phase-5 owner; catalog-deferred |
| 56 | `contract.evidence.read` | contracts / read-only | Phase-5 owner; catalog-deferred |
| 57 | `contract.evidence.append` | contracts + SDK transaction / append-only | Phase-5 owner; catalog-deferred |
| 58 | `contract.verdict.evaluate` | contracts / read-only | Phase-5 owner; catalog-deferred |
| 59 | `contract.gate.evaluate` | contracts / read-only | Phase-5 owner; catalog-deferred |
| 60 | `dock.manifest.list` | contracts / read-only | Phase-5 owner; catalog-deferred |
| 61 | `dock.manifest.read` | contracts / read-only | Phase-5 owner; catalog-deferred |
| 62 | `dock.run` | contracts + separable process executor / append-only | Phase-5 owner; catalog-deferred |

The `contract.*` and `dock.*` rows remain in the inventory for parity and
future catalog closure. HCM-4.2 does not expose them as implemented capability.
