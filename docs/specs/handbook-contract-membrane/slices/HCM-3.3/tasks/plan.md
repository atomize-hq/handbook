# HCM-3.3 Deterministic Projection implementation plan

## Planning outcome

This plan describes the future implementation boundary selected by the
planning-only HCM-3.3 increment. It creates no code packet, code path, test,
or adoption claim. Future work starts only after a new explicit user selection.

## Dependency graph

```text
completed HCM-3.2 Context Resolution kernel
  + exact profile / vocabulary / source contracts
  + canonical Projection semantics
    -> exact ProjectionDefinition validation
    -> exact request/source compatibility and currentness validation
    -> per-rule Resolution / redaction / disclosure / support accounting
    -> deterministic reveal or derive output and provenance
    -> collapse/expand refusal-or-escalation handling
    -> selected implementation proof of PG-PROJ-01 and PG-PROJ-02
```

No Snapshot capture/delta producer, pipeline consumer, renderer migration,
SDK/CLI DTO, or transport appears in this dependency graph. Those remain later
slice seams.

## Future implementation work items

### 1. Freeze future typed request, definition, and result boundary

**Acceptance criteria:** exact paired refs/fingerprints, `exactly_one` source
selectors, resolved-profile catalog admission, allowed surfaces/operations,
mandatory currentness closure, fingerprinted disclosure/support dependencies,
complete six-dimension rules, and no executable/remote/model/transport hook
are represented and fail closed. A registered configured custom kind is
selected declaratively through the same exact closure and engine rather than a
hard-coded family path, new core operation, or source-order fallback.

**Verification:** positive configured-custom-kind replay/provenance plus
invalid-custom-configuration refusal; structural and semantic definition
negatives for source cardinality, profile-unlisted definition, stale/incompatible
closure, invalid classification/policy/evaluator, unsupported surface/operation,
cycles, duplicate target producers, and forbidden hook content. Check that
`none` currentness is null/empty, exact currentness is snapshot-selector-only
with captured family/adapter/slot tuples, and equal live revisions cannot green
a stale captured source.

**Dependencies:** HCM-3.2 plus canonical Projection semantics.

### 2. Implement deterministic rule evaluation

**Acceptance criteria:** applicable rules evaluate in fixed order; the engine
checks Resolution, upstream redaction, metadata-only disclosure, and the exact
versioned built-in metadata-only support evaluator before reading protected
bytes; only reveal/derive execute; each rule is included, typed-omitted, or
operation-mismatch `not_applicable`. The evaluator accepts only the canonical
metadata allowlist and chooses one deterministic first unsupported reason.

**Verification:** positive reveal/derive replay, no-read spies for denied
paths, each typed omission/reason/requiredness/claim combination, evaluator
substitution or stale/invalid registry refusal, forbidden evaluator-input
rejection, and first-unsupported-reason ordering.

**Dependencies:** work item 1.

### 3. Implement provenance, lossiness, and non-authority result semantics

**Acceptance criteria:** result pairs, captured-value currentness evidence,
full accounting, proof effects, derivation I/O, output/result fingerprints,
and the exact evaluator identity/semantic closure in definition/evaluation/
result fingerprints replay exactly, with fixed lossiness precedence, target
absence, and `authority_effect: none`.

**Verification:** byte-identical repeated requests; redacted/partial/collapsed/
lossless precedence matrix; evaluator-semantic drift changes fingerprints or
refuses; no false pass for omitted required claims; source authority remains
unchanged for artifact, snapshot, delta, and semantic-record sources.

**Dependencies:** work item 2.

### 4. Enforce collapse/expand and no-synthesis boundaries

**Acceptance criteria:** collapse never broadens an envelope; attempted
expansion returns only an authorized broader request path or typed escalation;
missing detail is never invented; synthesis cannot enter the core.

**Verification:** broader/finer request negatives, mutation/promotion absence,
and forbidden synthesis/remote/executable definition cases.

**Dependencies:** work items 1 through 3.

### 5. Prove only the selected Projection gates

**Acceptance criteria:** independently reviewable proof closes the exact
selected portions of `PG-PROJ-01` and `PG-PROJ-02`, with no Snapshot Memory,
pipeline, consumer, public API, or Phase-3 exit claim.

**Verification:** targeted and regression proof, independent discovery/
remediation/closure cadence, complete applicable workspace wall, and an exact
scope/classification statement.

**Dependencies:** work items 1 through 4 and a newly authorized implementation
selector.

## Risks and controls

| Risk | Control |
|---|---|
| A renderer view is mislabeled as a Projection | Preserve the fixed renderer-derived/capitalized Projection distinction in definitions, tests, and review. |
| Omitted information silently appears successful | Use complete per-rule accounting, required/claim proof effects, target absence, and fixed lossiness. |
| A request widens authority or reads protected values too soon | Compare all six dimensions, honor upstream redaction, and apply fail-closed metadata-only policy/support before any payload read. |
| A generic engine becomes a synthesis or transport layer | Permit only reveal/derive and ban model prompts, executable hooks, remote code, and transport business rules. |
| Future implementation scope grows into HCM-3.4+ | Require a fresh selector with exact source/test/path/proof bounds. |

## Planning completion boundary

The current slice completes when this plan and its selector/spec are review
clean. All five implementation work items remain unstarted and require a new
top-level authorization.
