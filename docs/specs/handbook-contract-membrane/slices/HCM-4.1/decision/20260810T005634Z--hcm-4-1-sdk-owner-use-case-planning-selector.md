# HCM-4.1 SDK owner and use-case inventory planning selector

**Selector ID:** `HCM-4.1-SDK-OWNER-USE-CASE-PLANNING-SELECTOR-01`

**Packet ID:** `HCM-4.1-PLANNING-CONTROL-PACK`

**Integrated outcome:** `hcm-4-1-sdk-owner-use-case-inventory-planning`

**Status:** planning/control-pack admission only; subject to the internal discovery review named below.

## Bound orchestration identity

This selector is bound to increment task `019fe926-a636-72c0-af48-855e6fed7a03`
on host `local`, meta task `019fe86d-8591-75e3-b222-330527ba1fe4` on host
`local`, meta workflow `handbook-hcm-4-1-planning-20260810`, and dispatch nonce
`eabc0636e845a36c2745dba27ade56b9013808d8d0caa127c3c61d5d99b6dc2e`.

The admitted planning base is commit
`cc44a84c0f5b75f336301dac0c502cfae909c17e`, tree
`54309db1e88e6cfeb1611cf18dfab2d07c10d487`, with required ancestor
`cc44a84c0f5b75f336301dac0c502cfae909c17e`. Local-only publication, if this
planning outcome becomes review-clean, is limited to
`refs/heads/orchestration/handbook-hcm-4-1-planning-20260810T004658Z` by
expected-old compare-and-swap from that commit. The observed remote-tracking
baseline is `refs/remotes/origin/feat/handbook-contract-membrane` at
`1256e724a2b7da6b6250f57d6f63fced1e2cf949`; no push, fetch, pull, merge,
rebase, reset, clean, or force-update is authorized.

## Authority and dependency-order exception

`04-phase-slice-map.md` still orders Phase 3 before Phase 4. The operator
authorizes exactly one additive exception: HCM-4.1 **planning only** may proceed
while Phase 3 and HCM-3.6 remain open because the unresolved HCM-3.6
production-reachability seam needs the Phase-4 SDK/ordinary-consumer boundary.
The durable decision and proof are respectively:

- `docs/specs/handbook-contract-membrane/phase-3-exit/decision/20260810T005634Z--hcm-4-1-planning-dependency-order-exception.md`;
- `docs/specs/handbook-contract-membrane/phase-3-exit/proof/20260810T005634Z--hcm-4-1-planning-dependency-order-exception-proof.md`.

The exception does not close HCM-3.6, the strict-Clippy P2,
`HCM3EXIT-P2-CLIPPY-001`, HCM-3.5 Packet 5 or 6, Phase 3, Phase 4, or any
proof gate. It does not authorize HCM-4.1 implementation, HCM-4.2+, Phase 5,
Phase 6, release, publication, or push. A later Phase-3 closeout must consume
the real Phase-4 adoption evidence and repeat its strict-Clippy, regression,
causal-review, and exit gates.

## Admitted planning scope

This selector admits only the following documentation/control surfaces:

- the affected canonical truth in `00-README.md` through
  `06-proof-and-regression-ledger.md`;
- this additive HCM-4.1 slice package;
- the additive Phase-3-exit exception decision and proof above;
- immutable v1.4 internal dispatches, the final parent handoff, and the
  deterministically rebuilt ledger; and
- exact validated advisory registration/deduplication in `09` only if a review
  produces an unfixed P3/P4 finding.

It admits no Rust, test, fixture, Cargo manifest or lockfile, schema asset,
generated output, dependency, configuration, feature, lint policy, public
symbol, SDK crate, CLI behavior, compiler behavior, or transport behavior.
No historical record or dispatch may be changed.

## Frozen target and current-truth decisions

1. `handbook-sdk` is the ordinary-consumer facade. It composes owner crates and
   never becomes a semantic owner, a generic `Value` dispatcher, or a source of
   transport-specific wording.
2. `handbook-engine`, `handbook-flow`, `handbook-pipeline`, and the future
   `handbook-contracts` remain semantic owners. No owner crate may depend back
   on `handbook-sdk`, a transport, or Substrate.
3. `handbook-compiler` has no target ownership. Its remaining public root is a
   CLI-facing compatibility/support seam. Future ordinary composition moves to
   SDK, executable-shell work moves to CLI, and owner behavior stays in its
   existing owner crate. Any temporary compiler cutover adapter is one-way,
   non-public, and must have a deletion proof.
4. The `handbook_engine::project_posture` and Charter authority transaction
   path remain the ideal private, transport-free posture owner. Current source
   shows no production caller for `apply_posture_transition`; its three direct
   callers are crate-local tests. A real ordinary-consumer operation must not
   synthesize a call through promotion, recovery, startup, tests, or an
   unrelated service.
5. The planned real ingress is a future SDK-coordinated ordinary operation
   `posture.transition.apply`. It must bind an already persisted approved
   recommendation, exact evaluation-policy pair, sorted approval inputs,
   current heterogeneous canonical/lifecycle head, required reassessment
   validation inputs, and byte-exact compare-and-write basis. The SDK passes a
   typed intent to a later reviewed engine-facing bridge; it neither constructs
   private durable records nor decides recommendation, approval, head, or
   policy truth.

## Required planning deliverables

This selector requires a reviewable HCM-4.1 `SPEC.md`, executable
`tasks/plan.md`, `tasks/todo.md`, source/impact research, proof wall, canonical
cross-document reconciliation, and a complete ordinary-use-case inventory.
The inventory must state for each catalogued use case its semantic owner, SDK
coordination role, caller/transport route, typed request/result and
blocked/refused/error shape, mutation/idempotency/compare-and-write posture,
and required real-path proof.

Every inevitable new public Rust API, SDK crate/module/type, DTO/schema,
dependency, compiler/CLI cutover, and test effect is assigned to a later
fresh implementation authority. Planning does not select any of those edits.

## Required review and stop conditions

Before the reviewed planning commit, the parent must freeze a manifest of this
complete documentation subject, validate a v1.4 discovery dispatch, and run
one fresh read-only discovery review. If that review identifies valid material
P1/P2 work, the parent consolidates remediation and runs one different-fresh
closure review. The normal budget is one discovery, one consolidated
remediation, one closure after material repair, and at most two immediately
causal supplemental cycles; no review occurs after CLEAN. P3/P4 follow `09`.

GitNexus MCP tools are unavailable in this task environment. The planning
package records exact source-search caller evidence and marks GitNexus
context/impact/change detection and compare-to-main evidence **unavailable**,
never GREEN. A future code selector must rerun actual upstream impact before
every existing-symbol edit and warn before any HIGH/CRITICAL edit.

Stop for a need to implement a new public SDK surface, change a dependency,
choose a different mutation authority, promise compatibility, widen a phase,
alter immutable history, obtain inaccessible required evidence, or exceed the
causal allowance. Local documentation consistency, proof, selector mechanics,
and review remediation remain parent-owned work inside this slice.
