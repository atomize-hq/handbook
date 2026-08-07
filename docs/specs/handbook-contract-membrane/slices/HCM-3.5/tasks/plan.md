# HCM-3.5 Resolution-aware adoption — implementation-ready plan

## Plan status and boundary

This is an implementation-ready documentation plan for the single integrated
outcome `hcm-3-5-implementation-admission-and-contract-freeze` and Packet 0
`HCM-3.5-P0-implementation-admission-and-contract-freeze`. It records the
selected first engine operation/value boundary and the initial P1 ceiling; it
does not authorize code, public APIs, schemas, dependencies, package
publication, SDK/CLI/Substrate adoption, or a gate runtime. Packet 1 still
requires a fresh code selector, symbol-level impact analysis, and an
independent proof/review loop.

The live prerequisite is HCM-3.4's private source-pair boundary at
`ca765cc`/`cc6d849`. It remains private; `snapshot_delta` is relation-only.
The live Flow resolver (`resolve` / `resolve_with_contract` through packet
assembly) and pipeline compiler (`compile_pipeline_stage*` through scoped
document filtering) are current evidence seams, not APIs to extend by default.

## Frozen architecture decisions

1. **Concrete typed engine boundary.** Packet 1 will add only
   `handbook_engine::grounding::ground_resolution(repo_root: &Path,
   GroundingRequest) -> Result<GroundingOutcome, GroundingOperationError>`.
   `GroundingOutcome` is a grounded/refused value algebra; raw private sources
   never cross its boundary and its fields remain private. The four opaque
   exact-reference values have only `parse_exact` factories, `GroundingRequest`
   has only its named five-argument factory, and grounded/refused values have
   only named narrow read-only accessors. There is no generic JSON/Serde
   dispatcher, top-level re-export, public schema, or transport.
2. **Typed owner chain.** `handbook-engine` owns exact snapshot/delta binding,
   currentness, redaction-before-read, bounded Resolution Projection, and the
   bounded/redacted delta-signal summary. `handbook-flow` owns the future
   packet path consuming that result. `handbook-pipeline` owns future
   namespaced shared Resolution inclusion and transition sequencing. The
   acyclic consumers do not reimplement engine semantics.
3. **Greenfield adoption.** New purpose-named paths leave current resolver and
   compiler contracts unchanged. Existing callers are regression baselines.
   The only temporary bridge is an exact one-way mapping from legacy scoped
   metadata to the namespaced inclusion input; no raw-level fallback is
   allowed after a typed-path refusal.
4. **Bounded delta disclosure.** HCM-3.4 `snapshot_delta` keeps complete
   relation semantics. Engine derives an envelope-bound, redacted,
   definition-bounded summary; `reveal_delta_signals` consumes only that
   summary, never raw `/signals`.
5. **Descriptive transition refs.** Parent handoffs cite exact
   prior-end/start/grounding/end/delta refs after validation. They do not copy
   snapshots or convert observation into authority.
6. **Non-promoting evidence.** Flow/pipeline carry typed evidence refs,
   omissions, and separately named local/parent dimensions. Both are false or
   unavailable on indeterminate evidence. A future `handbook-contracts` owner
   alone evaluates gate policy and parent promotion.
7. **External composition.** Future `handbook-sdk` composes owner operations;
   standalone Handbook CLI adapts SDK results. Substrate consumes/wraps exact
   published crates.io SDK/library versions. The Tier 2 binary/JSON bridge is
   isolated transitional transport pending independent publication and
   current-tip real-seam proof.

## Dependency graph

```text
HCM-3.2 Resolution kernel + HCM-3.4 private snapshot/delta source pair
                                  |
                                  v
                   Engine typed grounding + bounded summary
                         /                    |                    \
                        v                     v                     v
                 Flow packet path     Pipeline shared inclusion   transition refs
                        \                     |                    /
                         \                    v                   /
                          typed non-promoting evidence boundary
                                             |
                                             v
                         future gate owner (separate HCM-5 authority)

owner operations -> SDK -> standalone CLI
published SDK/library -> Substrate wrapper/consumer
Tier 2 binary/JSON bridge --transitional--> published real seam
```

## Ordered future implementation packets

### Packet 0 — implementation admission and contract freeze

**Dependencies:** exact local baseline validation and fresh read-only inspection
of the HCM-3.4, engine, Flow, pipeline, handoff, SDK, CLI, and Substrate seams.

**Decision frozen:** use `handbook_engine::grounding` as the only cross-crate
engine grounding module; use `GroundingRequest` and a grounded/refused
`GroundingOutcome` whose grounded branch exposes only the engine-created
Flow/pipeline views, summary, provenance, omissions, and non-promoting
evidence. Do not make existing resolver/compiler request/result paths
mandatory for HCM-3.5.

**Acceptance criteria:**

- Names exact code symbols/path ceiling, visibility/refusal algebra,
  compatibility posture, and no-cycle dependency direction.
- Preserves relation-only `snapshot_delta`, bounded summary, and non-promoting
  gate boundary.
- Runs upstream impact analysis before each symbol edit; GitNexus absence is
  recorded as unavailable, never GREEN.

**Verification:** reviewed selector, source/caller inventory, public
compatibility matrix, and manifest/whitespace convergence.

### Packet 1 — engine grounding and bounded delta summary

**Exact owner:** `handbook-engine` only. It starts at the P0 source/test/proof
ceiling in `../SPEC.md`; Flow/pipeline code is context and regression evidence,
not an implementation surface.

**Future call path:** `GroundingRequest` exact current snapshot + exact
compatible relation-only delta + `ContextResolutionEnvelope` ->
private engine exact-ref resolver/currentness/redaction validation -> bounded
Projection and bounded/redacted delta-signal summary ->
`GroundingOutcome::{Grounded, Refused}`.

**Acceptance criteria:**

- Preserves exact source/policy/profile/vocabulary/definition/envelope
  provenance, all five currentness families, typed omissions, and
  `authority_effect: none`.
- Has definition-pinned bounded cardinality/order/overflow and redaction for
  the summary; reconciles it with `reveal_delta_signals`.
- Reads no raw source payload before a redaction refusal and never emits an
  unfiltered delta signal to a consumer.
- Does not alter `resolve`, `resolve_with_contract`, `compile_pipeline_stage*`,
  Cargo metadata, Handoff schemas, gate policy/runtime, or a transport surface.

**Verification:** positive bounded projection; absent/duplicate/substituted
sources; stale family/slot; incompatible/reversed delta; insufficient
Resolution; redaction-before-read; summary-overflow; omitted/unsupported
signal; and regression of the HCM-3.4 source-pair wall.

### Packet 2 — Flow packet adoption

**Likely owner:** `handbook-flow`, adjacent to the current resolver and packet
assembly seams, without changing them.

**Future call path:** purpose-named Flow request -> typed engine grounding
result -> packet fields/typed omissions/provenance -> ready or refused result.

**Acceptance criteria:**

- Resolution controls disclosure; byte budgets remain subordinate resource
  reporting.
- Flow neither reconstructs Snapshot Memory nor reads a full delta/signal.
- Existing `resolve` and `resolve_with_contract` behavior remains unchanged.

**Verification:** narrow/broad envelope cases, every omission kind, stale and
insufficient source refusal, byte-pressure non-widening, provenance replay,
and legacy resolver compatibility wall.

### Packet 3 — pipeline namespaced shared inclusion

**Likely owner:** `handbook-pipeline`, from the compile-stage seams through
document assembly and scoped filtering.

**Future call path:** purpose-named stage -> exact namespaced shared engine
Resolution input -> deterministic scoped inclusion -> compile provenance ->
capture/handoff evidence.

**Acceptance criteria:**

- No final include decision depends on raw `work_level` or L0–L3.
- A temporary mapping is definition-pinned, one-way, exact, current, and
provenance-bearing; no typed-path failure falls back to it.
- Unknown, stale, malformed, ambiguous, nested, or overbroad mapping refuses.

**Verification:** same-input replay; allow/narrow/deny cases; every mapping
refusal; raw-level no-fallback; preserved scoped-document corpus behavior;
and capture/provenance replay.

### Packet 4 — session transition references

**Likely owner:** the selected parent-handoff producer using existing nullable
`snapshot_refs` capacity; this plan assumes no schema change.

**Future call path:** prior end -> start -> compatible delta -> grounding
Projection -> work -> end -> final delta -> exact parent-handoff refs.

**Acceptance criteria:**

- Refs are exact, current, descriptive, and never copy snapshot content.
- Missing/partial/unstable/incompatible/stale capture reports its truthful
  status and blocks a grounding or promotion claim.

**Verification:** valid sequence plus absent, unstable, incompatible, stale,
reversed, redacted, sequencing, and partial-reference corpus cases.

### Packet 5 — non-promoting evidence integration

**Dependencies:** a separately selected `handbook-contracts` gate-runtime
owner. It is not an HCM-3.5 runtime implementation task.

**Acceptance criteria:**

- Local completion and parent-promotion eligibility remain separate exact
  policy/evidence evaluations.
- Packet/pipeline readiness, advisory score, or handoff existence cannot
  promote a parent.

**Verification:** complete hard/required/advisory/not-observed/omitted/
redacted/stale/indeterminate matrix and a regression proving local completion
cannot promote a parent.

### Packet 6 — ordinary-consumer and Substrate composition

**Dependencies:** HCM-4 SDK/JSON work and separate publication/adoption
authority.

**Acceptance criteria:**

- SDK composes owner operations; standalone Handbook CLI is a thin adapter.
- Tier 2 stays exact-versioned JSON, bounded, isolated, and removable.
- Permanent Substrate consumption/wrapping resolves exact crates.io
SDK/library versions at a named current-tip real seam without path/patch/CLI
fallback.

**Verification:** independently satisfy `PG-SDK-01`, `PG-JSON-01`,
`PG-SUB-CLI-01`, `PG-PUBLISH-01`, and `PG-SUB-RUST-01`; no proof substitutes
for another.

## Checkpoints and stop matrix

| Checkpoint | Must be true before advancing | Stop when |
|---|---|---|
| Engine -> Flow | Exact source/currentness/redaction/provenance result and bounded summary are proven. | A public/private boundary or summary requires a different owner, schema, dependency, or API decision. |
| Flow -> pipeline | Flow uses the typed engine result without legacy mutation. | Flow must read full delta data, recreate engine logic, or change retained resolver behavior. |
| Pipeline -> transition refs | Namespaced inclusion is deterministic and raw fallback is impossible. | Mapping cannot be exact/current/provenance-bearing or Handoff needs a schema change. |
| Evidence -> promotion | Local/parent distinction is carried without evaluation. | A consumer needs a promotion answer before the selected gate owner exists. |
| SDK/CLI/Substrate | Owner operations are independently available and published proof is authorized. | Tier 2 is treated as permanent, registry/publication proof is unavailable, or Substrate needs a semantic exception. |

## Explicit non-goals

- No code, tests, fixtures, source export, public API/signature, schema,
  config, dependency, package, registry, SDK, CLI, Substrate, or consumer
  implementation.
- No current caller migration, compatibility promise, or unbounded legacy
  bridge; no direct raw delta signal route; no duplicate snapshot model.
- No HCM-3.4 rewrite, HCM-3.6/HCM-4+/HCM-5 selection, gate runtime,
  publication, remote operation, or ref update. Packet 0 alone may make its
  reviewed local documentation primary and mechanical handoff/ledger closeout
  commits; it may not alter historical records or move a ref.
