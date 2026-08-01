# HCM-3.1 Vocabulary Resolution

Status: planning discovery findings remediated; fresh closure required before runtime edits
Phase: HCM-3
Slice: HCM-3.1
Parent orchestration: `20260801T122747Z--HCM-3-1--vocabulary-resolution`
Integrated outcome: `hcm-3.1-vocabulary-resolution-full-slice`

## Objective

Land repository-selected Vocabulary Resolution without changing stable machine
meaning. A selected vocabulary may change presentation language and may declare
lossless workflow-role absorption, while every typed operation, command, schema,
artifact kind, capability, authority boundary, and stable-role identity remains
selected by its existing typed contract.

This slice closes only the vocabulary subset of profile application and the
Vocabulary seam. Context Resolution, capitalized Projection, Snapshot Memory,
posture, public adapters, and the Phase 3 exit remain open.

## Authority and dependencies

This contract is subordinate to the applicable HCM authority in `00` through
`06`, especially the Vocabulary model and contract, the Phase 3 sequencing, and
`PG-PROFILE-01`, `PG-VOCAB-01`, and regression rules 29, 32, 33, and 36.

The completed P7 Phase 2 handoff, post-Phase-2 smoke handoff, and HCM-0.8 v1.4
protocol-repair handoff are dependency evidence only. They do not select this
slice. Their completed state contains no unresolved P1/P2 blocker for HCM-3.1.

## Frozen owner and runtime path

`crates/engine/src/vocabulary_registry.rs` is the semantic owner for authored
vocabulary validation, canonical identity, label lookup, alias lookup, and
absorption validation. It resolves against the exact `StableRoleRegistry`
selected by the enclosing instance profile.

The existing profile path remains authoritative:

1. `profile_selection::resolve_profile_selection` chooses the instance profile,
   exact stable-role registry, and exact vocabulary.
2. `ResolvedInstanceProfile` retains the vocabulary ref and fingerprint in its
   resolved closure and fingerprint.
3. `ArtifactRepositoryV1` exposes that already-resolved vocabulary read-only to
   an admitted product renderer.
4. `pipeline_capture::admit_work_specification` passes it to the fixed Work
   Specification Markdown renderer.
5. The renderer preserves the current bytes for the shipped empty vocabulary;
   for a non-empty repository/test vocabulary it applies the selected
   `delivery_unit` label and emits every declared structural absorption with
   stable role IDs intact.

This is a renderer-derived view over already resolved profile data. It is not a
Context Resolution envelope and does not define or execute capitalized
Projection.

## Typed model

The authored YAML fields remain the contract fields already authorized by
`handbook.vocabulary-profile@1.0`; no schema field is added:

- `labels`: `stable_role_id -> display_label`;
- `aliases`: `stable_role_id -> [alias, ...]`;
- `absorptions`: a list of `{ unit_id, absorbs }`, where `absorbs` is a list of
  registered stable role IDs;
- `stable_role_registry`: the exact registry ref and fingerprint;
- `extensions`: empty in HCM-3.1 because no extension namespace is declared.

The existing `StableRoleCategory` is the semantic-axis authority. HCM-3.1 does
not invent a second category system or a new field. Labels and aliases may
address any registered stable role. Structural absorption may address only
roles whose existing category is `workflow`.

The runtime exposes typed results:

- a resolved display entry retains `role_id` and the chosen label;
- typed role lookup accepts a stable role ID and resolves that role directly,
  never through aliases;
- untyped lookup returns exactly one of unique role, ambiguous sorted candidate
  role IDs, or unknown/refusal;
- absorption entries retain `unit_id` plus all absorbed stable role IDs.

No vocabulary string is convertible into an operation, command, schema, kind,
capability, authority, or machine identifier.

## Label behavior

- An explicit non-empty label overrides presentation for its exact role only.
- A missing label falls back to the selected stable-role registry's canonical
  display label.
- Duplicate displayed labels across roles are legal. They create an ambiguous
  untyped lookup; they do not invalidate typed role lookup.
- Label application cannot change a role ID, artifact descriptor, operation
  context, command, schema, capability, or resolved-profile selection.

## Alias normalization and collision rule

Every authored label and alias must be non-empty after validation, must already
equal its trimmed form, and must not contain Unicode control characters.

For untyped lookup only, normalization is deterministic and dependency-free:

1. collapse every maximal Unicode-whitespace run to one ASCII space;
2. lowercase each Unicode scalar using Rust's deterministic
   `char::to_lowercase` expansion;
3. perform no punctuation stripping, accent folding, compatibility folding, or
   Unicode normalization.

The lookup index includes explicit labels, registry fallback labels for roles
without explicit labels, and authored aliases. Machine role IDs are not aliases.
Repeated normalized text for the same role is idempotent. A normalized token
that addresses multiple distinct roles is legal but ambiguous and returns the
lexicographically sorted stable-role candidate IDs. No first-wins rule exists.
Unknown text returns a typed unknown/refusal.

## Structural absorption graph

The graph used for validation is exact and finite:

- nodes are every authored `unit_id` and every registered stable role ID named
  by an `absorbs` edge;
- each declaration adds directed edges `unit_id -> absorbed_role_id`;
- a cycle exists when those directed edges contain any directed cycle, including
  a self-loop;
- traversal and topological tie-breaking are lexicographic by node ID.

Constraints:

- `unit_id` uses the existing lowercase symbolic identifier grammar and is
  unique within the vocabulary;
- every absorbed target is registered in the selected registry and has category
  `workflow`;
- if a `unit_id` is itself a registered role, it also has category `workflow`;
- each stable role may be owned by at most one absorption declaration;
- duplicate targets within one declaration are invalid;
- `constitutional_authority` is explicitly forbidden and also fails the
  workflow-category rule;
- governance, artifact, evidence, organizational, approval, and validation
  boundaries cannot be absorbed;
- capability IDs fail as unknown stable roles;
- all entries and targets are rendered by the selected real consumer, so an
  unsupported or silently flattened absorption is not permitted.

Local unit IDs may match workflow stable-role IDs; this is necessary for the
declared graph to express nesting and for cycle validation to be meaningful.
They never become stable role IDs or operations by virtue of that match.

## Canonical fingerprint

The vocabulary fingerprint covers the exact stable-role registry ref and
fingerprint plus every semantic vocabulary field except
`vocabulary_fingerprint` itself:

`schema_id`, `schema_version`, `vocabulary_id`, `vocabulary_version`,
`stable_role_registry`, `labels`, `aliases`, `absorptions`, and `extensions`.

The canonical preimage is the existing RFC 8785 canonical JSON byte sequence
with these ordering rules:

- object keys and role-keyed maps are lexicographic;
- aliases are represented by their normalized lookup tokens; each role's tokens
  are deduplicated and ordered lexicographically;
- absorption declarations are ordered by `unit_id`;
- each `absorbs` list is ordered by stable role ID;
- `extensions` is an empty object in this slice;
- the canonical JSON bytes have no insignificant whitespace and no appended
  delimiter or line ending before SHA-256.

Map/list authoring order therefore does not affect identity. Alias spelling
changes that normalize to the same token are semantically identical; a changed
normalized alias token, label, absorption, registry identity, or version changes
identity. The existing shipped empty-vocabulary bytes and fingerprint remain
unchanged; the canonical implementation must reproduce that pinned vector
exactly.

The immutable shipped canonical preimage is:

```json
{"absorptions":[],"aliases":{},"extensions":{},"labels":{},"schema_id":"handbook.vocabulary-profile","schema_version":"1.0","stable_role_registry":{"fingerprint":"sha256:0c85b1b53786e7980c4fd0d7975cd9cde1a3eae2bc8daceb23be1a1731263029","ref":"handbook.roles.core@1.1.0"},"vocabulary_id":"handbook.vocabulary.shipped-root","vocabulary_version":"1.0.0"}
```

It hashes without a trailing LF to
`sha256:69113b1a9271ce207d45bdb91ebae8d6516249e16b59891b292546078364a22b`.
The required non-empty test vector is:

```json
{"absorptions":[{"absorbs":["implementation_unit"],"unit_id":"delivery_unit"},{"absorbs":["atomic_action","execution_envelope"],"unit_id":"implementation_unit"}],"aliases":{"atomic_action":["task"],"delivery_unit":["feature"],"implementation_unit":["slice","task"]},"extensions":{},"labels":{"delivery_unit":"Feature","implementation_unit":"Slice"},"schema_id":"handbook.vocabulary-profile","schema_version":"1.0","stable_role_registry":{"fingerprint":"sha256:0c85b1b53786e7980c4fd0d7975cd9cde1a3eae2bc8daceb23be1a1731263029","ref":"handbook.roles.core@1.1.0"},"vocabulary_id":"example.vocabulary.hcm-3-1","vocabulary_version":"1.0.0"}
```

It hashes without a trailing LF to
`sha256:0a28353460ce60a0fc53ba5e99ea2ec753abf94638489fe1eebd69b317d90ec4`.

## Profile-selection integration

Profile selection continues to reject a vocabulary whose stable-role ref or
fingerprint does not match the selected registry. The resolved profile retains
the exact vocabulary ID, version, ref, and fingerprint, and the resolved-profile
fingerprint closes over that vocabulary fingerprint. Selecting a different
valid vocabulary therefore changes resolved profile identity but cannot change
the selected artifact descriptors, operation contexts, capability truth, or
machine IDs.

## Real consumer behavior

The Work Specification Markdown path is the single HCM-3.1 product adoption
target. Its fixed machine inputs and artifact admission remain unchanged.

- With the shipped empty vocabulary, output is byte-identical to the current
  golden, including `# Work Specification`.
- With an explicit `delivery_unit` label, the presentation heading changes from
  `# Work Specification` to
  `# Work Specification — <controlled-label>`.
- With valid absorptions, a deterministic vocabulary section renders each local
  unit and every absorbed stable role as `role_id` plus resolved display label.
- The section is ordered by canonical absorption order and cannot drop an edge.
- Aliases are presentation metadata only and do not select the Work
  Specification operation or artifact kind.

The exact renderer byte contract is:

- the existing Objective, Scope, Non-Goals, Acceptance Criteria, and Status
  blocks retain their current order and bytes;
- a heading suffix is present only when `delivery_unit` has an explicit label;
- label text is passed through the existing `controlled_markdown_text` behavior:
  whitespace runs collapse to one ASCII space and ASCII punctuation is escaped;
- when there are no absorptions, no Vocabulary section is emitted;
- otherwise the Vocabulary section is appended after Status, separated by one
  blank line, with declarations and targets in canonical lexical order;
- each unit and role ID is the validated lowercase symbolic ID in Markdown code;
- each absorbed label uses explicit-label-or-registry-fallback resolution and the
  same controlled Markdown escaping;
- exactly one terminal LF is emitted.

For the required non-empty vector, the suffix is exactly:

```markdown
## Vocabulary

### `delivery_unit`

- `implementation_unit`: Slice

### `implementation_unit`

- `atomic_action`: Atomic Action
- `execution_envelope`: Execution Envelope
```

Together with the unchanged body, its first line is exactly
`# Work Specification — Feature`. A consumer iteration-count assertion must
prove that the two declarations and three directed absorption edges all render.

No CLI JSON schema or generated command changes are authorized.

## Proof obligations

Focused RED/GREEN tests must prove:

- fallback and explicit labels, legal duplicate display labels, typed direct
  lookup, unique untyped aliases, ambiguous candidates, and unknown refusal;
- exact normalization behavior and deterministic candidate ordering;
- unknown roles, registry mismatch, capability IDs, constitutional absorption,
  non-workflow absorption, duplicate ownership, duplicate targets, and cycles;
- exact shipped fingerprint, non-empty exact fingerprint vectors, author-order
  invariance, and repeated-load/replay determinism;
- resolved-profile exact vocabulary retention and fingerprint influence;
- non-influence on stable machine identifiers and typed operations;
- unchanged shipped-empty renderer bytes;
- deterministic non-empty Work Specification output with no absorption loss.

The converged proof wall is:

1. focused vocabulary, stable-role, profile-selection, artifact-repository, and
   pipeline-capture tests;
2. relevant engine, flow, compiler, and CLI regression targets;
3. `cargo test --workspace --all-targets --all-features`;
4. `cargo fmt --all -- --check`;
5. `cargo clippy --workspace --all-targets --all-features -- -D warnings`;
6. `git diff --check`;
7. forbidden-influence scans and HCM-3.2/HCM-3.3 absence scans;
8. handoff ordinary validation and both handoff self-tests at closeout;
9. GitNexus scoped and compare-to-`main` change detection, reported unavailable
   rather than GREEN if the local engine cannot provide the requested mode.

Stop conditions are an unresolved P1/P2, a HIGH/CRITICAL unbounded impact, an
unexplained UNKNOWN impact not bounded by this selector, changed shipped-root
vocabulary bytes/fingerprint, required schema expansion, loss of a structural
edge at the consumer, a required Context Resolution/Projection implementation,
or a genuine authority/external/capability boundary.

## Production and test ceilings

Production edits are limited to:

- `crates/engine/src/vocabulary_registry.rs`;
- `crates/engine/src/profile_selection.rs` only to validate a vocabulary against
  the exact registry instance selected by the profile;
- `crates/engine/src/artifact_repository.rs` only for read-only access to the
  already-resolved vocabulary;
- `crates/pipeline/src/pipeline_capture.rs` only for the selected fixed renderer;
- minimal module exports if compilation requires them.

Test/fixture edits are limited to
`crates/engine/tests/vocabulary_registry.rs`,
`crates/engine/tests/profile_selection.rs`,
`crates/pipeline/tests/pipeline_capture.rs`, and
`crates/engine/tests/fixtures/hcm_3_1_vocabulary_resolution/**`. The latter may
contain the non-shipped repository vocabulary/profile and expected renderer
golden. The shipped-root vocabulary is never edited or replaced.

Control-plane edits are limited to this slice's spec, plan, todo, decision,
dispatch/proof/handoff records; earned rows in `00` through `06`; the review
inventory only if a fresh P3/P4 requires registration; and rebuilt
`handoffs/ledger.jsonl` in the mechanical closeout commit.

No Cargo dependency, unsafe/native integration, new public transport, new CLI
command, new artifact kind, remote loading, executable hook, compatibility
layer, or unrelated cleanup is authorized.

## Delivery packets

- `HCM-3.1-P1-vocabulary-kernel`: typed model, validation, normalization,
  absorption graph, fingerprint, and focused vocabulary tests.
- `HCM-3.1-P2-profile-renderer-adoption`: exact profile retention proof,
  read-only consumer access, fixed renderer adoption, real-path golden, and
  non-influence proof.
- `HCM-3.1-P3-proof-control-closeout`: aggregate proof wall, independent reviews,
  earned control-pack synchronization, v1.4 handoff/ledger, and local closeout.

Packets are ordered and share one parent outcome. A packet is not a new slice
and cannot authorize HCM-3.2 or later work.
