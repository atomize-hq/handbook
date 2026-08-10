# HCM-4.1 — SDK owner and use-case inventory

**Packet:** `HCM-4.1-PLANNING-CONTROL-PACK`

**Status:** the planning/control package is complete. Its original HCM-4.1
implementation-admission selector and authority stop remain immutable
historical truth. The human operator subsequently waived v1.4 process
machinery only in
[`authority/20260810T160702Z--hcm-4-1-operator-no-v1-4-waiver.md`](authority/20260810T160702Z--hcm-4-1-operator-no-v1-4-waiver.md).
Implementation retains every selected product, proof, direct-review, and
protected-path gate.

## Objective

Implement the selected `handbook-sdk` boundary in reviewable packets: first
one direct, typed, transport-free Rust posture ingress, then owner composition,
eight-route CLI cutover, and compiler retirement. The first ingress is
intentionally undiscoverable; canonical operation discovery and all shared
DTO/schema/bootstrap work remain later slices. This package preserves the
ordinary-consumer ownership map, the compiler-retirement cutover, and the only
valid production ingress for the HCM-3.6 posture transition while Phase 3
remains open.

The governing selector is
[`decision/20260810T005634Z--hcm-4-1-sdk-owner-use-case-planning-selector.md`](decision/20260810T005634Z--hcm-4-1-sdk-owner-use-case-planning-selector.md).
The implementation admission selector is
[`decision/20260810T132347Z--hcm-4-1-implementation-admission-selector.md`](decision/20260810T132347Z--hcm-4-1-implementation-admission-selector.md).

## Non-negotiable ownership boundary

```text
semantic owner crates (engine / flow / pipeline / future contracts)
                           |
                           v
                  handbook-sdk composition
                           |
            +--------------+--------------+
            v                             v
   handbook-cli executable shell      future Tauri adapter

published SDK / advanced owner APIs --> Substrate consumer
```

- Owner crates retain semantic validation, repository truth, mutation, and
  domain refusal decisions.
- `handbook-sdk` owns ordinary-use-case composition, typed consumer-facing
  request/result/outcome values, operation identity selection, and repository
  transaction coordination where the owner contract requires it.
- CLI owns parsing, cwd/repository discovery, help, human rendering,
  stdout/stderr, and exit mapping. It must be an SDK consumer after cutover.
- Tauri and Substrate remain transports/consumers. Neither is an alternate
  semantic owner or posture mutation ingress.
- No owner depends on SDK; no SDK or transport dependency is allowed in
  `handbook-engine`, `handbook-flow`, `handbook-pipeline`, or
  `handbook-contracts`.

## Current source truth

The workspace contains engine, flow, pipeline, compiler, and CLI crates but no
`handbook-sdk` crate. `handbook-compiler` depends on all three current owners,
and `handbook-cli` directly depends on compiler plus all three owners. Its
public compiler root exports authoring, Charter command, setup, doctor,
resolver, rendering, and compatibility types. Those facts are migration
evidence, not a reason to freeze compiler ownership.

HCM-3.5 already has public engine grounding and narrow Flow/pipeline consumer
values. It remains a semantic-owner chain; the future SDK composes it and must
not duplicate Snapshot Memory, currentness, redaction, or delta semantics.

HCM-3.6 landed a private `project_posture` semantic path and crate-private
`CharterAuthorityTransactionServiceV1::apply_posture_transition`. It validates
and atomically writes canonical Charter bytes, a PostureTransition record, and
a lifecycle v1.1 record under the existing lock/recovery discipline. Its only
current direct call sites are crate-local tests. No current CLI, compiler,
Flow, pipeline, startup, recovery, or promotion route is a valid production
ingress.

## Ordinary-use-case contract

`tasks/plan.md` is the authoritative planning inventory for every operation in
the frozen `05` catalog. For every row, future implementation has one stable
dot-separated operation ID at `1.0.0`, one typed Rust SDK method, exact
request/result/blocker/refusal/error pairs, and eventual CLI JSON/Tauri parity
unless the frozen catalog says the operation belongs to a future Phase-5 owner.
Custom kind, profile, vocabulary, Resolution, and pipeline IDs remain typed
request data; they never generate methods, operations, or commands.

No public unbounded `serde_json::Value`, field-presence variant inference,
range/latest schema lookup, repo-file discovery, or transport-owned domain
branch is admissible.

## HCM-3.6 real posture ingress

The later `posture.transition.apply` operation is one specific future
ordinary-consumer path, not a generic escape hatch.

1. `posture.resolve` reads/derives a fingerprinted private-owner result through
   a typed engine bridge; it does not mutate.
2. `posture.recommendation.evaluate` returns exactly one typed
   recommendation-or-no-recommendation result. Evaluation remains pure.
3. `posture.recommendation.append` persists the exact approved-for-storage
   recommendation as an immutable semantic record; acknowledgment is separate.
4. `posture.transition.apply` accepts only a typed bounded intent containing
   the exact persisted recommendation, reassessment coverage, selected
   dimension change, repository identity, bounded idempotency key, and the
   caller's expected canonical ref/fingerprint/document-SHA-256/length. The
   engine resolves and revalidates the exact policy, source-kernel pair,
   required approval pairs/actor, canonical/lifecycle head, retained
   lifecycle-validation intake-record pair, and canonical bytes before the
   private atomic transaction. The resulting private transition and intent
   both durably bind repository identity; replay requires the same identity
   and expected canonical binding.

Expected semantic failures are typed blocked/refused outcomes: missing or
stale pair, policy/kernel disagreement, absent/wrong approval, stale/forked
head, invalid reassessment, no-op/invalid dimension, changed canonical bytes,
or unsupported capability. Operational integrity/durability failures are typed
errors without source payload. Success reports the exact three durable
transaction outputs: constitutional-root canonical truth, immutable
`PostureTransition`, and the lifecycle-transition 1.1 record bound to that
posture pair; the derived kernel is returned but never made canonical.

This selected ingress deliberately rejects promotion, candidate approval,
recovery, startup, test harnesses, and unrelated Flow/pipeline services as
callers. A future selector may not solve reachability with `pub`, test-only
calls, a lint exception, or a legacy compiler/CLI adapter.

## Operator-authorized implementation packets

| Order | Future packet and fresh authority | Candidate paths/symbols | Boundary, proof, and stop |
|---|---|---|---|
| 1 | HCM-4.1 direct SDK posture ingress | workspace `Cargo.toml`; new `crates/sdk/Cargo.toml`, `crates/sdk/src/lib.rs`; narrowly selected owner facade modules | The operator waiver admits the local-only crate/public API/dependency edge. Prove no SDK back-edge and obtain a different-fresh direct review. Stop on a dependency/version/public compatibility choice outside this plan. |
| 2 | HCM-4.1 repository and owner-operation composition | selected engine/flow/pipeline operations; compiler root call sites only after packet 1 proof | One typed method per catalogued operation; no raw owner records/transport DTO leakage. Prove direct-owner equivalence, typed blocked/refused/error behavior, and no semantic duplication. |
| 3 | HCM-4.1 HCM-3.6 proof replay | purpose-named engine-facing posture facade plus SDK coordinator; private `project_posture` and authority transaction remain owners | Prove exact recommendation/policy/approval/head/reassessment/CAS binding; positive, stale, refusal, replay, crash, restart, and public-value proof. Stop if a private record must become a public DTO or a new semantic owner is needed. |
| 4 | HCM-4.1 compiler retirement and CLI composition boundary | `crates/compiler/src/lib.rs`, selected compiler adapters; `crates/cli/**`; Cargo edges | Move only composition to SDK and shell behavior to CLI. Temporary scaffolding is one-way and removed after CLI uses SDK. Prove no CLI-to-compiler normal path, no new compiler downstream API, and preserved owner behavior. |
| 5 | HCM-4.2 fresh selector: shared DTO/JSON Schema | SDK DTO modules and generated schema assets only after exact approval | Full-SemVer schema parity, closed discriminants, schema/request/response fingerprints, exact idempotency/writes/receipts. No implicit transport changes. |
| 6 | HCM-4.3 / HCM-4.4 fresh selectors: CLI JSON and Tauri | CLI adapter and later Tauri adapter only | Each calls the same SDK method/DTO; one clean JSON response, exit/status parity, no normal CLI subprocess from Tauri. |
| 7 | HCM-3.5 Packet 5/6 and Phase-5/consumer selectors | gate runtime, publication, Substrate seam | HCM-3.5 local/parent evidence stays non-promoting until gate authority. Registry-only published proof and current-tip Substrate seam remain independent gates. |

## Required proof wall

The historical planning proof wall is at
[`proof/20260810T005634Z--hcm-4-1-sdk-owner-use-case-planning-preflight.md`](proof/20260810T005634Z--hcm-4-1-sdk-owner-use-case-planning-preflight.md).
It requires exact local-only base/ref/remote observations, protected-path
reservation, source/caller inventory, a no-SDK/no-transport-change assertion,
the exception consistency check, ordinary catalog parity, reviewer manifest
replay, documentation checks, handoff validation, and both v1.4 self-tests.
Those v1.4-only historical checks are not active for this waived continuation;
the continuation instead retains focused/package/workspace proof, direct
independent review, protected-path proof, and local expected-old CAS.

`PG-SDK-01`, `PG-JSON-01`, `PG-TAURI-01`, `PG-PUBLISH-01`,
`PG-SUB-RUST-01`, `PG-POSTURE-01`, and `PG-POSTURE-02` remain open. Planning
evidence cannot promote any of them.
