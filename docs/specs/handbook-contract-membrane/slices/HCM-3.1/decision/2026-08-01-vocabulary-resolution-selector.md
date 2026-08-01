# HCM-3.1 Vocabulary Resolution Selector

Date: 2026-08-01
Decision: discovery findings remediated; selected subject requires fresh closure
Slice authority: `../SPEC.md`

## Decision

HCM-3.1 will implement Vocabulary Resolution in the engine's existing semantic
kernel and apply it to one fixed Work Specification Markdown consumer. It will
not create a general Context Resolution or Projection subsystem.

## Orchestration registry

The unique parent orchestration is:

`20260801T122747Z--HCM-3-1--vocabulary-resolution`

The parent has one integrated outcome:

```json
[{"authority_ref":"docs/specs/handbook-contract-membrane/slices/HCM-3.1/decision/2026-08-01-vocabulary-resolution-selector.md","integrated_outcome_id":"hcm-3.1-vocabulary-resolution-full-slice","packet_ids":["HCM-3.1-P1-vocabulary-kernel","HCM-3.1-P2-profile-renderer-adoption","HCM-3.1-P3-proof-control-closeout"]}]
```

Registry fingerprint:

`sha256:d0f92e3453edc70c4ed9a185d69dfbbe631fbe84653bad07ecbf060d8ef20ee2`

Derived causal budget for the sole outcome:

`sha256:f32411b780227c90d7d3146339f933cd19df4911a82043d981958f67663b8758`

No child packet outside that frozen set is authorized. Review remediation uses
the same packet/outcome/budget and exact predecessor/finding lineage.

## Under-specified questions adjudicated

### 1. Semantic axes

Use `StableRoleCategory`, already owned by the exact selected stable-role
registry. A label or alias must key a registered role. An absorption target must
additionally be `workflow`. No new schema field, secondary category system, or
implicit string taxonomy is authorized.

Rationale: the existing categories distinguish governance, artifact, workflow,
evidence, and organizational semantics. Restricting structural conflation to
workflow roles prevents the vocabulary layer from erasing authority/evidence
boundaries while preserving the target workflow-language use case.

### 2. Alias normalization and collision

Normalize only for untyped lookup by collapsing Unicode whitespace and applying
Unicode scalar lowercase. Do not strip punctuation or perform normalization or
accent folding. Typed role lookup does not use this index.

Index explicit labels, fallback canonical labels, and aliases. A token mapping
to one role resolves uniquely; one mapping to several roles returns sorted
candidates; none returns typed unknown. Same-role duplicates are idempotent.
Cross-role collisions are legal ambiguity, never first-wins.

Rationale: this is deterministic with the current Rust standard library, keeps
the exact authored vocabulary visible, requires no Cargo dependency, and avoids
inventing locale-sensitive authority.

### 3. Absorption graph

Use directed edges `unit_id -> absorbed_role_id` over all declared unit IDs and
named stable roles. Validate the whole directed graph for self-loops and cycles
with lexical traversal/tie-breaking. Targets and registered unit IDs must be
workflow roles; targets are registered and single-owned.

Rationale: this is the smallest graph that represents structural absorption as
authored and detects nested cycles. Treating declarations as an edgeless table
would make the acyclicity requirement vacuous.

### 4. Real renderer/product consumer

Select the existing admitted Work Specification Markdown path in
`pipeline_capture.rs`. It already resolves the repository-selected instance
profile through `ArtifactRepositoryV1`, and its artifact descriptor uses stable
role `delivery_unit`.

The renderer receives the exact selected `VocabularyDefinition`, keeps current
bytes for the shipped empty vocabulary, applies an explicit `delivery_unit`
label only to presentation, and renders every absorption with stable IDs.

Rationale: this is a real stage-10 capture/product path with fixed rendering. It
does not add a CLI schema field, generated command, context envelope, projection
definition, or public transport.

## Fingerprint selector

Fingerprint the canonical semantic form, not authored map/list order. All
semantic fields and the exact stable-role registry identity participate; the
fingerprint field itself does not. Maps and declarations are lexical; aliases
are normalized lookup tokens, deduplicated and lexical per role; absorbed roles
are lexical. Hash the existing RFC 8785 canonical JSON bytes directly with no
appended LF or delimiter.

The existing shipped-root vector is an immutable compatibility constraint. Any
implementation that cannot reproduce it stops rather than rewriting its bytes
or published identity.

Exact replay vectors are frozen in `../SPEC.md`: the shipped empty vector hashes
to `sha256:69113b1a9271ce207d45bdb91ebae8d6516249e16b59891b292546078364a22b`
and the non-empty HCM-3.1 vector hashes to
`sha256:0a28353460ce60a0fc53ba5e99ea2ec753abf94638489fe1eebd69b317d90ec4`.

## Identity and compatibility posture

- Existing stable-role, vocabulary, profile, schema, operation, artifact, and
  capability IDs remain unchanged.
- Existing resolved-profile vocabulary retention is reused and strengthened by
  proof, not replaced.
- The shipped empty vocabulary remains the shipped selection and preserves
  current output through registry fallback labels.
- A non-empty vocabulary exists only in tests/repository fixtures for proof.
- No migration/compatibility layer justified only by Git history is added.
- New engine methods are immutable/read-only semantic queries; no remote loading
  or executable hooks are introduced.

## Exact ceilings

Production ceiling:

- `crates/engine/src/vocabulary_registry.rs`
- `crates/engine/src/profile_selection.rs` only for exact selected-registry
  vocabulary validation
- `crates/engine/src/artifact_repository.rs`
- `crates/pipeline/src/pipeline_capture.rs`
- minimal existing module exports only if compilation requires them

Test ceiling:

- `crates/engine/tests/vocabulary_registry.rs`
- existing profile-selection tests
- `crates/pipeline/tests/pipeline_capture.rs`
- `crates/engine/tests/fixtures/hcm_3_1_vocabulary_resolution/**` for the
  test-only repository vocabulary/profile and exact expected Markdown

Control ceiling:

- this slice's authority, decisions, tasks, proof, dispatches, and handoffs
- earned affected rows in HCM `00`–`06`
- inventory registration only for a fresh P3/P4 finding
- rebuilt global handoff ledger in the mechanical closeout commit

The nine protected working-copy paths are outside every ceiling and manifest.

## Test selector

The exact negative classes are unknown role, registry ref/fingerprint mismatch,
capability ID as role, constitutional absorption, non-workflow absorption,
duplicate ownership, duplicate target, cycle/self-loop, ambiguous untyped token,
unknown untyped token, adapter loss, fingerprint mismatch, and forbidden machine
influence.

The exact positives are fallback label, explicit presentation label, duplicate
display labels, typed direct role resolution, unique alias, valid nested acyclic
workflow absorption, canonical ordering/replay, exact profile retention, exact
shipped vector, unchanged empty renderer golden, and deterministic non-empty real
renderer golden.

The non-empty renderer golden uses the vector frozen in `../SPEC.md`. The first
line is `# Work Specification — Feature`; the existing five content sections are
unchanged; the Vocabulary suffix has two lexically ordered unit headings and
three lexically ordered role lines exactly as frozen there; IDs are Markdown
code, labels use existing controlled escaping, the section is omitted when
absorptions are empty, and the document has exactly one terminal LF.

## Stop conditions

Stop and issue durable authority-required evidence only if implementation would
require a new schema field, product choice beyond this selector, HCM-3.2/3.3
runtime, new public transport/CLI contract, modification of a pinned shipped
identity, or an unbounded HIGH/CRITICAL/UNKNOWN impact. Ordinary internal review
findings, proof gaps, and bounded packet remediation remain parent-owned.

## Rejected alternatives

- Changing doctor/compile JSON was rejected because those schemas are frozen and
  Phase 4 typed JSON refusal remains deferred.
- Replacing canonical artifact titles globally was rejected because the shipped
  empty vocabulary must preserve current output.
- A general-purpose render/projection engine was rejected as HCM-3.3 scope.
- A context-aware alias resolver was rejected as HCM-3.2 scope.
- New category or semantic-axis fields were rejected because the stable-role
  registry already supplies the necessary authority.
- Publishing a non-empty shipped vocabulary was rejected; proof uses test data.
