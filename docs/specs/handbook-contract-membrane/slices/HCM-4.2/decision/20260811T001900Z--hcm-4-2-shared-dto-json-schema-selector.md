# HCM-4.2 selector — shared DTO and JSON Schema contract

**Phase:** `HCM-4`
**Slice:** `HCM-4.2`
**Packet:** `docs/specs/handbook-contract-membrane/slices/HCM-4.2/SPEC.md`
**Selector state:** candidate for review admission
**Selector authority:** this file and its exact clean-review evidence

## Objective

Freeze the implementation-ready public contract for shared typed request,
result, blocker, refusal, error, reference, catalog, capability, bootstrap,
operation-definition, and schema DTOs owned by `handbook-sdk`, together with
their exact Draft 2020-12 generated-schema, deterministic serialization,
fingerprint, compatibility, and immutable discovery boundaries.

This is a planning/control-pack slice. It does not implement Rust types,
Serde derives, generated schemas, operation handlers, CLI JSON, Tauri, skills,
contract runtime, docks, publication, release, or downstream consumers.

## Current truth and predecessor boundary

The exact HCM-4.1 checkpoint is `28fa4ab4f2fced48532d6830a7a69d8bd12a0cd1`,
present as the parent of the assigned base. At the assigned base
`818d3662f57fcda867a59b7f9e035755fee1b393`,
`handbook_sdk::HandbookSdkV1::apply_posture_transition` is callable through
typed Rust linkage. It is not a discoverable machine operation: it has no
operation-definition entry, public operation request/result schema, generated
JSON Schema, bootstrap descriptor binding, catalog entry, CLI JSON surface, or
Tauri command.

HCM-4.2 may describe the future public boundary that makes this operation
discoverable, but it must not expose private HCM-3.6 records, move domain
validation into SDK DTOs, or manufacture a CLI/Tauri ingress. HCM-4.1 remains
the source of direct-owner equivalence and private-record preservation.

## Authority and required source set

The selector is bounded by these live authorities:

- `docs/specs/handbook-contract-membrane/01-target-architecture.md` — SDK
  ownership and inward dependency direction;
- `docs/specs/handbook-contract-membrane/02-semantic-model.md` — frozen
  semantic identities, posture, Resolution, and private-owner boundaries;
- `docs/specs/handbook-contract-membrane/03-seam-crosswalk.md` — current
  seam classifications and HCM-4.1 posture truth;
- `docs/specs/handbook-contract-membrane/04-phase-slice-map.md` — HCM-4.2
  boundary and HCM-4.3/HCM-4.4/HCM-4.5 separation;
- `docs/specs/handbook-contract-membrane/05-contracts-schemas-and-gates.md`
  — frozen SDK inventory, DTO, operation, bootstrap, transport, and
  compatibility contract;
- `docs/specs/handbook-contract-membrane/06-proof-and-regression-ledger.md`
  — proof gates and unresolved status;
- `docs/specs/handbook-contract-membrane/slices/HCM-4.1/SPEC.md` and
  `tasks/plan.md` — predecessor implementation truth and the complete
  ordinary-operation inventory;
- `slices/HCM-4.2/research/20260811T001900Z--source-impact-inventory.md` —
  current source/owner/inventory evidence and unavailable GitNexus evidence.

## Exact scope

The selected planning subject covers, in dependency order:

1. a transport-neutral shared DTO module owned by `handbook-sdk`;
2. closed discriminated request/result/problem/blocker/refusal/error/reference
   shapes, explicit null/default behavior, and bounded artifact/catalog refs;
3. exact operation-definition records for the frozen canonical inventory,
   with current-owner admission recorded without exposing future Phase-5
   owners;
4. full-SemVer schema IDs, operation refs, definition/schema fingerprints,
   RFC 8785 serialization and fingerprint preimages;
5. the checked-in Draft 2020-12 schema layout and deterministic regeneration
   source-of-truth rule;
6. immutable same-major bootstrap descriptor, capability discovery, paged
   operation/profile/schema catalogs, and exact snapshot/cursor binding;
7. compatibility, refusal/admission, idempotency/write-receipt, direct-Rust
   equivalence, and negative transport-ownership proof walls; and
8. future adapter seams for HCM-4.3 CLI JSON and HCM-4.4 Tauri without
   implementing either transport.

## Exact non-goals and authority stops

- No Rust, Cargo manifest, dependency, generated asset, fixture, product test,
  schema generator, DTO, operation handler, visibility change, or public API
  change is allowed in this slice.
- No `handbook-compiler` recreation or dependency is allowed.
- No CLI parsing, help, rendering, stdout/stderr, exit policy, or Tauri
  command name is part of a DTO or operation-definition contract.
- No contract runtime, dock executor, Phase-5 owner, Phase-6 consumer,
  publication, release, remote, or downstream change is allowed.
- No direct Rust method becomes discoverable merely because it is `pub` or
  reachable by a test.
- No private posture recommendation, policy, approval, canonical-head,
  reassessment, idempotency, transition, lifecycle, or recovery record is
  exposed as a public DTO. The future public posture DTO is a bounded view
  with exact bindings and domain-owned outcome mapping.
- No range, `latest`, ambient file, CLI help, example, prose, Tauri command,
  or process-local registry may select capability or schema truth.

Any request to cross these boundaries is an authority stop, not a planning
detail to be inferred.

## Operation and owner admission

The complete 62-ID inventory in `05` and HCM-4.1 `tasks/plan.md` is reproduced
and reconciled in the source/impact inventory. HCM-4.2 defines the common
operation-definition shape and the catalog admission predicate; it does not
claim that every row has a live owner implementation.

Current-owner rows may receive future typed definitions only when their owner
crate and semantic contract are live and the definition can bind exact request,
result, blocker, refusal, and error schemas. The SDK remains the ordinary
consumer coordinator; owner crates never depend on SDK. The frozen
`contract.*` and `dock.*` rows remain Phase-5-owned and are catalog-deferred in
this slice. No future owner is exposed early. `capabilities.describe` reports
only definitions whose exact schema closure and honest transport status are
present; it never reports the whole frozen inventory as implemented.

`posture.transition.apply` receives a public discovery path only after the
future operation definition binds a bounded public request/result DTO, the
domain-owner refusal mapping, the exact operation/schema fingerprints, and
the same-major bootstrap descriptor. The public result preserves the
recommendation, policy, approval, canonical/lifecycle head, reassessment,
idempotency, exact-byte, and compare-and-write bindings without carrying
private owner records. This is a discoverability plan, not a transport or
runtime implementation.

## Required future proof wall

The implementation packet must prove, at minimum: byte-identical schema
regeneration; Rust/schema fingerprint equality; RFC 8785 canonical bytes;
closed discriminants and unknown fields; null/default and absent-by-default
semantics; unsupported major and unadvertised minor behavior; stale, cold,
tampered, and cross-major bootstrap/catalog behavior; unknown operation,
missing schema/profile, and definition mismatch; page/cursor/snapshot
consistency; request/result/problem closure; idempotency and write receipts;
direct Rust equivalence; and the negative proof that CLI/Tauri/help/prose/
ambient files do not own discovery or domain branching.

The required planning package names exact candidate paths/symbols, dependency
direction, compatibility/migration boundaries, tests/proof gates, GitNexus
blast-radius evidence or its exact unavailability, risks, stop conditions,
rollback/recovery, and disallowed shortcuts in dependency order.

## Review and promotion

This selector received a fresh built-in discovery review before the remaining
package was authored. The review found no valid P1/P2 and no intersecting P3/P4.
That clean selector review permits package authoring within this scope; it does
not authorize implementation. The complete final package receives its own
fresh discovery review and, if any P1/P2 is repaired, a different-fresh delta
closure review.

## Selector review evidence

The selector review was dispatched as
`docs/specs/handbook-contract-membrane/handoffs/dispatches/20260811T002100Z--HCM-4-2--selector-discovery-review.json`.
The fresh built-in agent completed with status `completed` and verdict
`clean`; the first launch attempt failed before execution because the selected
model was at capacity and was retried with a different fresh built-in agent.
The reviewed pre-status-update subject fingerprint was
`sha256:d6a81de2e776edae2f4bc59be1041f3d30f1465d97a79b842abb012c757dccb2`.
The reviewer independently confirmed the exact 62-operation reconciliation,
typed-Rust-only HCM-4.1 posture truth, owner/boundary separation, and honest
GitNexus unavailability. No P1/P2 or intersecting P3/P4 was returned.
