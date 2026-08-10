# HCM-4.1 implementation admission selector

**Selector ID:** `HCM-4.1-IMPLEMENTATION-ADMISSION-SELECTOR-01`

**Packet ID:** `HCM-4.1-IMPLEMENTATION-ADMISSION`

**Integrated outcome:** `hcm-4-1-sdk-ordinary-use-case-and-cli-cutover`

**Status:** independently reviewed; not admitted. Discovery review
`20260810T133000Z--HCM-4-1--implementation-admission-discovery-review` found
an authority-bound P1 and P2, so no product edit is admitted.

## Independent review disposition

The fresh discovery review returned `findings`, not `CLEAN`:

- **HCM41-IA-01 (P1):** the completed planning package does not select an
  authority-bounded compatibility disposition for the eight existing normal
  CLI routes. Retiring compiler without it would force an unplanned decision
  to preserve, refuse, remove, or add SDK methods for a route.
- **HCM41-IA-02 (P2):** `posture.transition.apply` cannot be honestly exposed
  through capability discovery while this slice excludes the exact
  operation-definition/schema/bootstrap bindings that canonical `05`
  requires. Narrowing the slice to a no-public-operation skeleton would not
  establish the required ordinary production ingress.

Both findings require new product/compatibility authority. The selector and
its Phase-3-open exception remain safeguards, not implementation permission.
Resume only with an exact approved retained-route map and an authority decision
for the operation-definition/schema/bootstrap boundary.

## Bound implementation authority

This selector consumes completed planning handoff
`20260810T012500Z--HCM-4-1--orchestration--sdk-owner-use-case-planning-completed`
at `36524894aa6060150d78f50e9fa87275c1a99e7d`. It is bound to increment task
`019febcb-566f-77c1-b736-fad8ed53d4d5` on host `local`, meta workflow
`handbook-hcm-4-1-implementation-20260810`, and dispatch nonce
`bd27738ab78de09e9279a65b00b41c63538f8d86ffa30f29a110b0b325ec8075`.

The selector is implementation authority for HCM-4.1 only. It does not reopen
the completed planning handoff, HCM-3.6, HCM-3.5 Packet 5 or 6, Phase 3,
HCM-4.2+, Phase 5/6, publication, Substrate, Tauri, or JSON-schema work.

## Conditional Phase-3-open ordering exception

The companion decision
`phase-3-exit/decision/20260810T132347Z--hcm-4-1-implementation-dependency-order-exception.md`
permits this selector and, only after this selector's `CLEAN` review, the
bounded HCM-4.1 implementation sequence while Phase 3 stays open. It is an
ordering exception only. It preserves HCM-3.6, `HCM3EXIT-P2-CLIPPY-001`,
HCM-3.5 P5/P6, and every Phase-3 exit proof as open until their separately
authorized closeouts consume real production adoption evidence.

## Frozen candidate public boundary

If admitted, `handbook-sdk` is a new workspace library package at version
`0.1.0`, matching the workspace's initial package convention. Its public
surface is transport-free Rust only:

- `HandbookSdk::open(PathBuf)` establishes a repository-bound coordinator;
- `HandbookSdk::capabilities()` reports only implemented operation definitions;
- `HandbookSdk::apply_posture_transition(ApplyPostureTransitionRequest)` is the
  sole candidate mutating operation;
- all results use closed typed `ok`, `blocked`, `refused`, and `error` variants.

No public raw durable record, raw canonical payload, generic
`serde_json::Value`, schema/DTO serialization, transport wording, second
semantic owner, or unbounded operation dispatch is admitted. The sole
candidate capability is `posture.transition.apply@1.0.0`; every other frozen
catalog row is absent from discovery until a later admitted packet proves its
owner and exact boundary.

The engine-facing bridge, if admitted, must accept typed semantic intent and
return only bounded receipts for the canonical Charter, immutable
`PostureTransition`, lifecycle-transition `1.1`, and resulting kernel pair.
It must construct and validate private durable records inside engine. The SDK
may never construct, expose, or deserialize those private records.

## Candidate packet order and exact ceiling

1. SDK skeleton: `Cargo.toml`, `Cargo.lock`, `crates/sdk/Cargo.toml`,
   `crates/sdk/src/lib.rs`, `crates/sdk/src/capabilities.rs`,
   `crates/sdk/src/posture.rs`, and SDK-only tests/docs.
2. Posture bridge: `crates/engine/src/lib.rs`,
   `crates/engine/src/charter_authority_transaction.rs`,
   `crates/engine/src/charter_posture_transaction_intent_v1.rs`,
   `crates/engine/src/charter_lifecycle_transition_v11.rs`,
   `crates/engine/src/project_posture.rs`, and their narrowly coupled tests.
3. CLI/compiler cutover: `crates/cli/Cargo.toml`, selected `crates/cli/src/**`,
   `crates/compiler/Cargo.toml`, and selected `crates/compiler/src/**` only
   after packet 1 and packet 2 prove the SDK owner boundary.

The graph must be `CLI -> SDK -> engine` for every admitted normal route. No
owner crate, flow, pipeline, or future contracts crate may depend on SDK. The
posture bridge/recovery area is a CRITICAL-risk boundary: GitNexus found 24
upstream dependents of `recover_pending_locked`, including approval and
lifecycle-finalization processes. No recovery, promotion, startup, test, Flow,
pipeline, compiler, or legacy CLI route may become posture ingress.

## Admission proof and unresolved compatibility question

Admission requires an exact map from every retained normal CLI route to one
admitted SDK method, plus proof that no removed route changes an existing
compatibility promise. Current CLI routes include setup, author, approvers,
artifact, pipeline, generate, inspect, and doctor. The completed planning
package freezes only the ordinary-operation inventory; it does not select the
compatibility disposition or typed SDK method for every retained command.

Until independent review establishes that a bounded retained-route map is
already authorized, this selector cannot admit product edits. It must stop at
an authority boundary rather than remove, refuse, or silently reroute an
existing normal CLI command, create unplanned public methods, or leave a
normal CLI compiler/bypass path.

## Required proof wall after admission

Before any primary code commit: upstream GitNexus impact for every existing
symbol, an explicit user-facing warning before HIGH/CRITICAL work, exact
dependency/import scans, direct SDK consumer proof, targeted positive/negative
and recovery tests, full workspace tests, `cargo fmt --all -- --check`, and
`cargo clippy --workspace --all-targets --all-features -- -D warnings` without
suppression. A posture path additionally requires A01-A24, stale/missing input,
approval/head/reassessment/no-op/canonical-CAS/replay/crash/restart coverage,
and a genuine non-test production SDK caller.

No selector, test, visibility widening, dummy call, lint exception, or
compiler compatibility adapter is production-adoption proof.
