# HCM-3.4 Snapshot Memory and deterministic delta engine — implementation plan

## Planning outcome

This is a future implementation plan, not a code packet. It decomposes the Snapshot Memory capability selected by HCM-3.4 without selecting current symbols, paths, APIs, schemas, or runtime behavior. The current increment creates no Snapshot record, capture policy, delta, hook, test, or adoption claim.

## Dependency graph

```text
completed HCM-3.2 Context Resolution kernel
  + completed HCM-3.3 deterministic Projection boundary
  + frozen Snapshot capture/delta/redaction/retention contracts
    -> exact capture-policy and source-family closure
    -> immutable normalized snapshot and derived consistency
    -> compatible deterministic delta and catalog signals
    -> fail-closed redaction, retention, and deduplication posture
    -> paired end/start boundary workflow
    -> private HCM-3.4 generic Projection integration proof
    -> later HCM-3.5 handoff/packet/pipeline adoption
```

HCM-3.3 is the immediate ordered predecessor. Its Projection completion does not authorize HCM-3.4, but it locks the generic non-authority and exact-fingerprint posture that later snapshot grounding will consume. HCM-3.2 supplies the resolved-envelope prerequisite. HCM-3.5 and HCM-3.6 are explicitly outside this plan's implementation authority.

## Future implementation packets

Each future packet must first name exact code/test paths, ownership, impact analysis, input/output manifests, and a bounded source of truth. No packet may combine product implementation with public transport adoption or schema/version expansion without separate authorization.

### 1. Freeze private capture-policy and source-family model

**Goal:** implement exact policy parsing, validation, fingerprinting, source-family adapters, static windows, and multi-slot composite revisions.

**Acceptance criteria:** policy identity, trigger, memory horizon, source adapter, window, comparison, drift, predecessor, redaction, retention, and consistency closure are exact and deterministic. Invocations supply only live revisions/cursors for declared slots and cannot widen policy scope.

**Proof:** canonical-order replay; duplicate/unknown family, source-slot, window, trigger, ref/fingerprint, horizon, and ambient-source refusal; changed live revision/cursor changes capture input rather than policy identity.

**Dependencies:** HCM-3.2 exact envelope/profile identity and frozen HCM-0.3 contracts.

### 2. Produce immutable normalized ContextMemorySnapshot records

**Goal:** capture every selected family into one immutable record with exact provenance, exclusions, predecessor link, and separate state/record fingerprints.

**Acceptance criteria:** selected-family coverage is total; source payload and revision envelopes are normalized; map/path/window/ref order is deterministic; state identity excludes volatile boundary data while record identity preserves the complete record; snapshots stay descriptive evidence.

**Proof:** identical-state replays; different trigger/time/sequence records with equal state fingerprints; payload/ref/order mutation failures; missing, duplicate, extra, or bare-ref state refusal; no promotion or source mutation.

**Dependencies:** packet 1.

### 3. Derive consistency and preserve paired boundary order

**Goal:** implement stable/bounded/unstable derivation, retries/refusal, and immediate prior-end/new-start predecessor handling.

**Acceptance criteria:** only the full selected-family aggregation table chooses top-level consistency/admissibility; bounded observations cite the exact policy-selected rule and captured revisions; sequences are unique, strictly increasing, immediate, and acyclic within repository/workspace/stream.

**Proof:** all-stable; valid bounded; active-plan-only drift; changed source; out-of-bound; whole-family exclusion; retry/refusal; self/future/cyclic/skipped/wrong-stream/wrong-trigger predecessor negatives. Unstable records cannot ground closeout, promotion, hard gate, or stable comparison.

**Dependencies:** packets 1 and 2.

### 4. Build deterministic SnapshotDelta and drift catalog evaluation

**Goal:** compare compatible stable/bounded endpoint snapshots without mutating either and derive only catalog-backed signals.

**Acceptance criteria:** every endpoint-selected family is compared or type-excluded once; normalized changes carry stable keys and fingerprints; every catalog rule is evaluated once in order; matched evaluations and signals are bijective; only durable rule-admitted evidence explains justified divergence.

**Proof:** equal-state empty-change delta; ordered changed-family vectors; expected, justified, unexplained, scope, proof, semantic, planning, efficiency, and stale-handoff signals; reversed endpoint distinction; incompatible, unstable, missing/excluded/duplicate family, stale catalog, missing/duplicate/contradictory evaluation, and uncataloged-signal refusal.

**Dependencies:** packets 2 and 3.

### 5. Enforce redaction, retention, and immutable storage optimization

**Goal:** apply fail-closed sensitive-surface protection before persistence and make policy-selected retention/deduplication safe.

**Acceptance criteria:** unmatched input omits; secret, unrestricted environment, secret-file, command, and unrestricted diff floors cannot weaken; original/retained pointer behavior is unambiguous; all allowed horizon/trigger/record-class tuples resolve exactly once; deduplication preserves record identity and compaction is reviewed/additive.

**Proof:** all action-matrix rows; known-unmatched, matcher-failed, and unknown/unclassifiable-surface omission before persistence; identical and omit overlap; incompatible non-omit overlap refusal; retained-pointer outside-subtree behavior; hold/reference/floor deletion refusal; immutable record replay after dedupe/compaction. The unknown/unclassifiable case must deterministically refuse to downgrade into a non-omit action or retained data.

**Dependencies:** packets 1 through 3.

### 6. Prove private Snapshot Projection integration without consumer adoption

**Goal:** exercise the exact snapshot/delta boundary only through the existing generic Projection contract, without inventing a Snapshot-specific projection model or adopting any consumer.

**Acceptance criteria:** projection inputs are the exact compatible snapshot/delta source pairs and Resolution minima; every selected family has one authorized disclosure or typed omission; unfiltered signals remain current only through their captured revisions; redaction preserves original/retained-pointer semantics without hidden reads; stale or insufficient inputs refuse; and authority_effect is none.

**Proof:** a deterministic private projection matrix covering authorized disclosure, typed omission, complete all-family accounting, exact captured revisions, original/retained-pointer source identity, hidden-data refusal, stale/insufficient-input refusal, and authority_effect: none. The matrix must prove that no Handoff, packet, pipeline, or other HCM-3.5 consumer adoption occurred.

**Dependencies:** packets 2 through 5 and the existing generic Projection contract completed by HCM-3.3.

### 7. Prove the HCM-3.4 capability without adopting consumers

**Goal:** close only the selected Snapshot gates with a complete deterministic matrix and independent review.

**Acceptance criteria:** PG-SNAP-01 through PG-SNAP-06 have exact evidence at the authorized private implementation boundary. The paired previous-end/new-start workflow is available as a capability, while HCM-3.5 owns handoff/packet/pipeline adoption and HCM-3.6 owns posture use.

**Proof:** the complete matrix in SPEC.md, including the private generic Projection matrix from packet 6; repeated exact replays; fail-closed negatives; scoped/full applicable regression wall; source immutability; no model interpretation; and a different-fresh review after each material repair.

**Dependencies:** packets 1 through 6 and a newly authorized implementation selector.

## Verification checkpoints

| Checkpoint | Required evidence before advancing |
|---|---|
| Model closure | policy/family/ref/fingerprint and negative validation matrix pass; no public/schema/API selection entered |
| Snapshot closure | coverage, ordering, two-fingerprint, consistency, and predecessor walls pass; unstable/promotion negatives are explicit |
| Delta closure | endpoint compatibility, exact family coverage, catalog completeness, signal bijection, and durable-justification cases pass |
| Security/storage closure | redaction action/pointer matrix and retention/hold/dedupe/compaction negatives pass |
| Private Projection closure | generic Projection matrix proves exact sources, typed disclosure/omission, complete accounting, captured-revision currentness, pointer semantics, hidden-data and stale refusal, and authority_effect: none without consumer adoption |
| HCM-3.4 proof closure | all six PG-SNAP rows pass at selected scope; no HCM-3.5 adoption, PG-HANDOFF-02, public surface, or Phase-3 exit claim |

## Risks and controls

| Risk | Control |
|---|---|
| A capture observes a moving world but labels it stable | derive consistency from complete pre/captured/post evidence and fail closed on exclusions or invalid bounds |
| A delta hides unavailable state as unchanged | require one compare-or-exclude disposition per selected family and refuse incomplete coverage |
| Drift becomes subjective/model-owned reasoning | bind every classification to one exact catalog rule and durable evidence/justification refs |
| Snapshot state leaks sensitive content | enforce fail-closed redaction before persistence and project retained fields independently |
| Deduplication erases provenance or retention safety | share payload storage only; preserve immutable record identity, references, holds, and floors |
| HCM-3.4 silently becomes HCM-3.5 adoption | reserve generic Projection, handoff references, packet grounding, pipeline use, and consumer migration to HCM-3.5 |

## Non-goals and human review gate

This plan does not authorize implementation. A human reviews this complete planning packet before any later task may choose implementation scope. That later task must begin from a newly reviewed selector and must not infer authority from this plan, the HCM-3.3 handoff, or a prior snapshot record.
