# HCM-3.5 Resolution-aware adoption — authority-boundary planning packet

## Status

This is a documentation-only planning packet for integrated outcome
`hcm-3-5-resolution-aware-adoption-planning` and packet
`HCM-3.5-P1-resolution-aware-adoption-planning`. The selector permits a
reviewed authority-boundary result, not an implementation claim. It consumes
the completed HCM-3.4 source-pair handoff only as immutable predecessor
evidence.

## Dependency graph

```text
HCM-3.2 Context Resolution public kernel
        +
HCM-3.4 private snapshots/deltas + source-pair Projection proof
        |
        v
public engine-to-Flow compatibility decision  [REQUIRED]
        |
        +--> Flow Resolution-aware grounding packet
        |
        +--> Pipeline namespaced scoped inclusion
        |
        +--> Handoff start/end snapshot and delta references
        |
        +--> Local-closeout / parent-promotion policy operation
```

The first two dependencies are validated at the selected base. The third is
not selected by this task. No downstream task is implementable until it is.

## Conditional implementation sequence

### Task 0 — select the public compatibility and gate-operation authority

**Dependency:** explicit user/product authority.

**Decision required:** choose the engine export/Flow operation posture from
`../SPEC.md#authority-boundary-public-compatibility-and-gate-operation-decision`,
including SemVer/migration treatment and whether HCM-3.5 may consume a
concrete gate runtime or only an interface.

**Acceptance criteria:**

- The decision names the exact public owner, request/result types or operation
  IDs, compatibility promises, and forbidden public surfaces.
- It preserves the acyclic dependency graph and does not add a dependency.
- It says whether legacy Flow/pipeline operations are unchanged or versioned.

**Verification:** fresh implementation selector, upstream impact analysis for
every edited symbol, and a public-API compatibility matrix approved under that
selector.

### Task 1 — expose the selected engine grounding boundary

**Likely owner:** `handbook-engine`; current private sources are
`crates/engine/src/snapshot_memory/{policy,record,delta,redaction}.rs` and
`crates/engine/src/projection.rs`.

**Call path:** selected typed public operation -> private current snapshot plus
compatible delta -> generic Projection validation/currentness -> immutable
grounding result/provenance.

**Acceptance criteria:** no duplicate Flow snapshot model; exact source-pair
and five-family currentness survive; redaction occurs before payload read;
result preserves omissions/proof effects and `authority_effect: none`.

**Verification:** HCM-3.4 regression wall plus new API-only positive/refusal
tests for every source, profile, envelope, adapter, slot, currentness, and
retained-pointer substitution.

### Task 2 — add a purpose-named Flow grounding path

**Likely owner:** `crates/flow/src/resolver.rs` and `packet_result.rs`;
existing `resolve_with_contract` is the compatibility baseline.

**Call path:** selected Flow request -> engine grounding operation -> exact
Projection result -> packet sections/notes/provenance -> ready/refused result.

**Acceptance criteria:** Resolution determines eligible disclosure; budgets
remain subordinate resource reporting; every omitted field is typed; stale or
insufficient source prevents ready selection; existing resolver behavior is
unchanged unless Task 0 explicitly versions it.

**Verification:** new flow integration corpus covering narrow/broad envelopes,
same-source/different-envelope provenance, all omission kinds, stale pair,
budget pressure, and legacy resolver byte-compatibility.

### Task 3 — replace raw pipeline work-level authority

**Likely owner:** `crates/pipeline/src/pipeline_compile.rs` from
`compile_pipeline_stage*` through `assemble_documents`,
`load_repo_relative_document`, and `filter_scoped_blocks`, plus its tests.

**Call path:** selected stage -> exact namespaced Resolution mapping -> scoped
document inclusion -> compile payload/provenance -> capture/handoff consumer.

**Acceptance criteria:** raw `L0`–`L3` never remains the final semantic
authority; compatibility mapping is exact and versioned if retained; every
include decision is reproducible from the source/envelope pair; invalid or
indeterminate inputs refuse rather than default to `L1`.

**Verification:** deterministic same-input replay, deny/narrowing cases,
unknown/malformed mapping refusal, old scoped-block regression corpus, and
capture/handoff provenance replay.

### Task 4 — adopt session snapshot references at parent boundaries

**Likely owner:** current v1.4 parent-handoff producer and its completed
`snapshot_refs` schema fields; no schema change is presumed.

**Call path:** prior end -> new start -> compatible delta -> grounding
Projection -> session work -> end snapshot -> final delta -> parent handoff.

**Acceptance criteria:** refs are exact and current; records remain
descriptive; start/end/currentness failures produce the correct
`not_available`, `partial`, or `failed` posture; no handoff copies snapshot
contents or claims promotion.

**Verification:** deterministic handoff-record fixture/corpus with a valid
pair and every missing, stale, incompatible, unstable, sequence, and
redaction refusal.

### Task 5 — consume distinct closeout and promotion decisions

**Likely owner:** selected gate operation owner; semantic source is
`05-contracts-schemas-and-gates.md` only until Task 0 chooses runtime
authority.

**Acceptance criteria:** local completion and parent-promotion eligibility
remain independently fingerprinted, default false on indeterminate state, and
are never inferred from Flow/pipeline success.

**Verification:** hard/required/advisory/omitted/not-observed/stale policy
matrix and a regression proving local success cannot promote a parent.

## Risk and stop matrix

| Risk | Effect | Required response |
|---|---|---|
| Exporting private engine types changes published compatibility | Implementation cannot legally start under this packet. | Stop at Task 0 authority boundary; do not add `pub` or edit Cargo metadata. |
| Existing Flow/pipeline callers observe input/result changes | Legacy products could change behavior. | Select a versioned/new operation or explicit migration contract before edits. |
| Snapshot data is comprehensive | Sensitive or irrelevant data could reach a session. | Use only generic Projection disclosure/omission/redaction; refuse before read. |
| Raw work levels are treated as Resolution | Scope/authority semantics collapse into a byte/string shortcut. | Use a namespaced engine type and exact mapping; unknown mapping refuses. |
| Gate semantics are implemented in Flow/pipeline | A consumer becomes a competing contract authority. | Stop unless selected gate runtime owner supplies an exact operation. |

## Exit proof

This planning task proves only the live dependency assessment, the exact
authority boundary, and a reviewed durable stop. It cannot prove `PG-SNAP-*`,
`PG-HANDOFF-02`, a Flow/pipeline adoption, local/promotion runtime behavior,
or Phase-3 exit. Those claims require the selected implementation decision and
the proof matrix above.
