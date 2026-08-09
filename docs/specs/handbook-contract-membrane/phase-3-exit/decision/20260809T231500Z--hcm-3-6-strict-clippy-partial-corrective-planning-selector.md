# HCM-3.6 strict-Clippy partial-corrective planning selector

**Status:** planning-only; pending fresh independent planning review. This is
not source-edit admission, a strict-Clippy completion, or a Phase-3 exit
selector.

**Parent orchestration:**
`handbook-hcm-3-6-strict-clippy-partial-corrective-planning-20260809`

**Packet:** `HCM-3-PHASE-3-STRICT-CLIPPY-PARTIAL-CORRECTIVE-PLANNING`

**Integrated outcome:** `hcm-3-6-strict-clippy-partial-corrective-planning`

## Scope and immutable starting truth

This planning-only parent consumes, without modifying or resuming, the
following immutable authority stops:

- [`20260809T152000Z--HCM-3-6--orchestration--phase-3-exit-strict-lint-authority-boundary`](../../handoffs/records/20260809T152000Z--HCM-3-6--orchestration--phase-3-exit-strict-lint-authority-boundary.json);
- [`20260809T173000Z--HCM-3-6--orchestration--strict-clippy-corrective-authority-boundary`](../../handoffs/records/20260809T173000Z--HCM-3-6--orchestration--strict-clippy-corrective-authority-boundary.json); and
- [`20260809T170900Z--hcm-3-6-strict-clippy-authority-stop.md`](../proof/20260809T170900Z--hcm-3-6-strict-clippy-authority-stop.md).

The exact current gate remains:

```text
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

At this planning baseline it exits `101` with 29 diagnostics. The previously
tested mechanical subset reduces that count by nine but leaves the exact 20
private-unreachable posture diagnostics recorded by
`HCM3EXIT-P2-CLIPPY-001`. The P2 remains open, blocking, and owned by the
authorized product/architecture decision described in the consumed authority
stop. A partial landing may never be described as resolving, accepting,
waiving, baselining, or inventorying that P2 as debt.

The planning baseline is `8c2e31b518007b4a2b14f36f128d35b04c6e632e`, tree
`dfe7fac4370e9f6ca20fba0fa22e9b4dd4098f4d`. It is clean and detached at the
local-only orchestration ref
`refs/heads/orchestration/handbook-phase-3-clippy-expansion-20260809`.
The protected checkout remains
`C:/Users/spmcc/Documents/__Project_Code/handbook`; it is read-only and out of
scope. No branch, remote, target ref, or protected checkout is changed by this
planning parent.

## Bounded future partial candidate

Only a later, separately admitted implementation child may consider the nine
mechanical fixes below. They are independent of the unresolved reachability
question, preserve the current result/byte/API behavior, and must be applied
as one reviewable partial subject or not at all.

| Path | Exact diagnostic(s) | Only permitted future change | Explicit invariant |
| --- | --- | --- | --- |
| `crates/engine/src/charter_authority_transaction.rs` | `clippy::needless_lifetimes` at `posture_output_for` (5800-5803) | Elide only the explicit lifetime in the private function signature. | Same borrowed input/output relationship, error behavior, posture transaction semantics, and visibility. |
| `crates/engine/src/snapshot_memory/policy.rs` | `clippy::ptr_arg` at `normalize_windows` (443) | Change only `&mut Vec<AuthoredStaticWindow>` to `&mut [AuthoredStaticWindow]`. | Same caller-owned vector, ordering, normalization, and serialized policy bytes. |
| `crates/engine/src/snapshot_memory/record.rs` | Three `clippy::unnecessary_map_or` diagnostics (524, 534, 893) | Replace only `as_deref().map_or(true, is_safe_text)` with `as_deref().is_none_or(is_safe_text)`. | Identical `None` acceptance, `Some` predicate, refusal kind, and capture validation. |
| `crates/engine/src/snapshot_memory/redaction.rs` | `clippy::redundant_closure` at 166 | Pass `valid_pointer_segment` directly to `Iterator::all`. | Identical JSON-pointer segment validation and redaction/refusal behavior. |
| `crates/engine/src/grounding.rs` | Two `clippy::type_complexity` diagnostics at `currentness_from_snapshot` (636-639) and `summarize` (688-691) | Add only private type aliases that spell the existing return values; use them in those signatures. | No boxing, allocation, data-layout, `GroundingOutcome`, `GroundedResolution`, variant-payload, or accessor change. |
| `crates/engine/src/snapshot_memory/tests.rs` | `clippy::unnecessary_sort_by` at 799 | Replace only the pure canonical-string comparator with `sort_by_key(canonical_json)`. | Same test input ordering and assertion behavior; no fixture, expectation, or coverage change. |

Before any future source edit, the implementation parent must reproduce the
baseline and run fresh upstream GitNexus `impact` and `context` for every
edited symbol. The planning parent changes no Rust symbol, so it performs no
symbol edit or new impact claim. GitNexus FTS is unavailable in this
environment and is recorded as unavailable, never GREEN. The prior
source-impact record remains evidence only; it is not a substitute for that
fresh implementation-time impact.

## Prohibited routes

The partial candidate does **not** authorize any route to make the remaining
20 diagnostics reachable or silent. In particular, later work must stop rather
than use any of the following:

- `#[cfg(test)]` production-reachability workarounds, test-only callers, or
  changed test expectations;
- `#[allow]`, `#[expect]`, lint policy/configuration, Cargo, feature, or
  warning-baseline changes;
- a `lib.rs`/parent-module/public visibility expansion, public type-graph
  expansion, or a synthetic production caller;
- legacy CLI/compiler wiring, SDK/transport wiring, candidate approval,
  promotion, lifecycle, recovery, or other semantic route misuse;
- schema, dependency, generated-asset, runtime-configuration, unsafe-policy,
  Flow, pipeline, HCM-5, package, remote, or protected-checkout changes; or
- Phase-4 implementation, a new slice, local publication/CAS, push, or a
  Phase-3 exit claim.

The deliberately excluded source owners are the 20 still-unreachable items in
`charter_authority_transaction.rs`,
`charter_lifecycle_transition_v11.rs`,
`charter_posture_transaction_intent_v1.rs`, and `project_posture.rs`. The
partial candidate may touch the first of those files only for the single
life-time-elision row above; it grants no posture reachability authority.

## Future proof and review plan

If a later source selector is independently admitted, it must:

1. bind a clean exact base, protected-path hashes, local-only target/ref state,
   and the six-path source/test manifest; re-run the strict gate and record the
   exact 29 baseline diagnostics;
2. run fresh impact/context before every existing-symbol edit and warn before
   any HIGH/CRITICAL result; stop on any changed API, public/schema/dependency/
   configuration, semantic, or non-six-path need;
3. apply only the nine rows above, with no additional cleanup, then prove the
   strict gate still fails only with the documented 20 remaining diagnostics;
4. run `cargo fmt --all -- --check`, `git diff --check`, the focused
   `handbook-engine` Snapshot Memory/grounding tests, `cargo test -p
   handbook-engine --all-features`, and the full workspace wall. A partial
   clean test result is not a strict-Clippy success claim;
5. preserve the exact permanent ordinary-validator result as failed/not GREEN
   only for its historical HCM-3.5 condition; any additional validator failure
   stops the work;
6. produce a byte-hashed, truthfully scoped GitNexus change-detection artifact.
   If FTS remains unavailable, record it as unavailable. Compare-to-main may
   be historical divergence and cannot be called a narrow green result; and
7. obtain a fresh complete-subject discovery review and, after any material
   repair, a different-fresh closure review under one new v1.4 causal budget.
   No review cycle follows CLEAN. The partial source subject must close at an
   authority boundary with the same P2 still open, never as `completed`.

## Program dependencies and non-goals

Phase 3 remains open. This selector does not satisfy strict Clippy, close
`HCM3EXIT-P2-CLIPPY-001`, or authorize a new Phase-3 exit audit. Phase 4 owns
the future SDK/CLI and machine-transport work. HCM-3.5 Packet 5 runtime/gate
integration and Packet 6 ordinary-consumer/Substrate composition remain later
dependencies, including their HCM-4/publication prerequisites. A possible
HCM-4 pathway for real posture adoption is future work, not an authorized
solution here.

No new or altered production test, fixture, external integration test, public
API, schema, documentation contract, consumer, runtime behavior, canonical
byte result, or transport is a goal of this partial candidate. The only test
path listed above retains its existing test behavior while satisfying its
single Clippy idiom diagnostic.

## Planning true-stop and resumption condition

This planning parent may close only after a fresh independent review validates
this selector as a documentation/control artifact and the parent records one
v1.4 handoff/ledger closeout. That closeout is a planning true stop, not a
source implementation closeout.

The next action remains blocked until an authorized top-level decision chooses
either:

1. a bounded implementation selector for this exact six-path, nine-diagnostic
   partial subject, preserving the P2 and ending at the same authority stop;
   or
2. a separate reachability/semantic authority that explicitly names its parent
   module, public/type/test/owner boundaries and is reviewed as a different
   corrective outcome.

Neither option may infer approval for the other. Any request to resolve the
remaining 20 diagnostics, alter their reachability, or claim Phase-3 exit is a
new authority decision.
