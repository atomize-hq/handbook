# Decision: Generic repository source selection without premature SDK authority

## Status

Accepted as HCM-2.3 planning authority when the complete planning subject is
review-clean. It authorizes no implementation until the separate implementation
handoff is explicitly selected.

## Date

2026-07-21

## Context

HCM-2.3 must prove a repository-defined kind, instance, schema, canonical YAML,
and supplied intake through a stable generic product path. The proof cannot add
the example kind to the HCM-0.6 shipped catalog, discover definitions
ambiently, derive authority from filenames or Rust enums, or introduce a
kind-specific/generated command. At the same time, HCM-0.4 and Phase 4 reserve
the named `handbook-sdk`, public operation/bootstrap catalog, transport DTO/
JSON Schema program, Tauri, Substrate, publication, and downstream adoption.

The existing HCM-1.2 boundary already requires one exact profile selection with
explicit typed per-definition-class repository source paths. HCM-1.3 proves the
`registry-brief` custom kind and instance at the engine boundary, but it neither
selects intake nor exposes a product path. HCM-2.2's Charter path is first-party,
constitutional, version-specific, and intentionally unsuitable as implicit
generic behavior.

## Decision

1. Extend the HCM-1.3 `registry-brief` lineage with a new repository profile
   version and a separate exact `ArtifactIntakeDefinition`; do not mutate the
   old fixture and do not package the example as a shipped default.
2. Use one fixed, closed `.handbook/profile-selection.json` record beneath an
   explicitly supplied repository root to serialize the existing source-
   binding request. It binds exact refs to safe repository paths but cannot
   override profile fields, definition content, fingerprints, or operation
   semantics. Its exact JSON Schema and vectors freeze every source-class field,
   both source effects, and the additive intake-source mapping. The engine reads
   exactly that path and performs no scan.
3. Keep `kind_ref` and `instance_id` as ordinary operation request data. The
   descriptor must resolve to that exact kind and remains the sole owner of the
   trusted canonical path and selected intake ref.
4. Implement semantics as typed synchronous `handbook-engine` owner operations
   using the canonical stable operation IDs. Add one fixed generic
   `handbook artifact` CLI family as a thin product adapter. Neither definitions
   nor kinds can add commands or dispatch behavior.
5. Treat those engine APIs as future SDK inputs, not as a delivered SDK. Phase 4
   alone may create `handbook-sdk`, public schema/operation manifests,
   bootstrap/negotiation, complete JSON transport parity, Tauri/Substrate
   adapters, publication, or downstream adoption claims. HCM-2.3 direct CLI
   replay uses a distinct owner-domain ledger whose derived context is outside
   key/request identity and whose lookup precedes currentness. The later
   canonical Phase 4 ledger passes the same domain key/request pair to the
   owner; it cannot derive a second inner namespace, migrate, or reinterpret it.
   HCM-2.3 proves this as an owner-API/preimage exclusion only. Phase-4-first,
   direct-first, outer-ledger transition, and inter-ledger crash execution
   fixtures remain Phase 4 work because HCM-2.3 defines no outer state model.
6. Keep generic intake/persistence separate from HCM-2.2 Charter record
   versions, paths, approval, lifecycle, and recovery authority. This proof
   allocates intake `1.2`, generic validation result `1.0`, candidate `1.4`, and
   promotion `1.2` with exact closed schema/vector authority. It uses a null
   approval policy and may not translate that null into implicit constitutional
   approval.

## Alternatives considered

### Add `registry-brief` to the shipped definition catalog

Rejected. It would turn a proof fixture into a first-party default, contradict
HCM-0.6 authority, and make product convenience rather than an approved decision
select the artifact universe.

### Reuse the Charter `1.3` candidate and promotion path as the generic kernel

Rejected. Those identities bind exact-result, constitutional approval,
lifecycle, waiver, and recovery semantics that are not generic. Reuse would
silently grant first-party authority to a capability-free observational kind.

### Let CLI flags supply arbitrary definition bytes or discover source folders

Rejected. That would make callers or ambient filenames an authority source and
would prevent deterministic, reviewable source closure. The fixed closed
selection record provides explicit effect bindings while exact definitions and
the selected profile retain semantic authority.

### Generate one command or adapter per repository-defined kind

Rejected. It makes extension registration a code-generation/dynamic-dispatch
problem, violates the generic operation contract, and cannot prove kind/
instance IDs remain request data.

### Create `handbook-sdk` and public transport schemas in HCM-2.3

Rejected. Phase 4 owns those packages and cross-transport compatibility proofs.
Pulling them forward would create an incomplete competing public boundary and
would overstate one local CLI proof as SDK/transport readiness.

### Keep the proof at engine unit-test level

Rejected. Unit-only registry validation cannot justify real-path adoption. An
actual CLI binary must exercise explicit registration, descriptor selection,
canonical schema validation, supplied intake, persistence, restart, and replay.

## Consequences

- A later implementation must add one closed repository selection input shape
  and one generic CLI family, but no shipped artifact definition or new
  dependency is expected. The exact selection and runtime contract artifacts
  in this packet are the implementation oracle, not production assets.
- The additive artifact operation context, not the existing resolved-profile or
  HCM-2.2 fingerprint, binds the intake registry. Old exact fingerprints remain
  compatibility invariants.
- `ArtifactInstanceRegistry::resolve` is the one anticipated CRITICAL edit: it
  may carry a generic intake ref while retaining all other later-owned
  dependency refusals. The operation context must close that ref before use.
- The maximum classification change is one atomic set with exactly two
  `RealPathAdopted` subset cells: registry-brief under `Artifact kind/schema
  registry` and registry-brief under `Charter intake coverage`. The exact
  Charter classification stays fixed; broader intake, kind, artifact, SDK,
  transport, and publication gates remain open.
- If safe fixed-path selection, acyclic intake closure, durable generic state,
  or the Phase 4 boundary cannot be preserved, implementation stops rather than
  choosing another architecture implicitly.
