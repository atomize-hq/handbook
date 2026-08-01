# HCM-3.1 vocabulary-resolution proof wall

Timestamp: `2026-08-01T13:59:34Z`

Parent orchestration: `20260801T122747Z--HCM-3-1--vocabulary-resolution`

Proof baseline: `07665506d2d7ce6a73cbd4831628567f674bb4f9`

Implementation discovery lineage: **CLEAN**

## Reviewed commit stack

- planning authority: `8004da7893b53f9fa2b06759ec3d11efad962537`
- typed vocabulary kernel: `bbe3140d0b1442bd5417d29e27548d88ce4e965b`
- profile/real-renderer adoption: `3be3200c283244f1065ceb7a42f594362c97209f`
- implementation-review closure: `07665506d2d7ce6a73cbd4831628567f674bb4f9`

## Proof outcomes

### Exact vocabulary and profile identity

- The shipped empty vocabulary is byte-unchanged from `f0b5f4c`; its file
  SHA-256 is
  `888c333475357ec7b8467dbed39f6ac881e8305d1c7d001f367f02921c0f26b1`
  and its exact semantic fingerprint remains
  `sha256:69113b1a9271ce207d45bdb91ebae8d6516249e16b59891b292546078364a22b`.
- The non-empty proof vocabulary fingerprint remains
  `sha256:0a28353460ce60a0fc53ba5e99ea2ec753abf94638489fe1eebd69b317d90ec4`
  after authored map/list reordering and deterministic normalization.
- The selected vocabulary must pin the exact selected stable-role registry
  ref/fingerprint. A registry mismatch refuses before profile completion.
- The resolved profile retains the exact vocabulary ref/fingerprint in its
  dependency closure and resolved-profile fingerprint. Artifact descriptors
  and stable machine identifiers do not change.

### Typed, untyped, ambiguity, and non-influence behavior

- Typed context resolves a registered stable role directly and never consumes
  an alias.
- Missing labels use the selected stable-role registry's canonical display
  label; duplicate display labels remain legal.
- Untyped unique aliases return one stable-role ID. Duplicate label/alias
  candidates return a sorted typed `Ambiguous` result; unknown text returns
  `Unknown`.
- Normalization proofs cover Unicode whitespace-run collapse, Unicode scalar
  lowercase expansion, and explicit non-folding of punctuation, accents, and
  compatibility characters.
- Capability IDs, unknown roles, constitutional authority, and non-workflow
  roles refuse as vocabulary roles/absorption targets.
- Changed production paths are limited to `handbook-engine` vocabulary/profile
  access and the existing fixed `handbook-pipeline` Work Specification
  renderer. No Cargo manifest, CLI, flow, SDK, Tauri, contract transport, or
  generated-command path changed. A production-diff scan found no
  `ContextResolution`, `ResolutionEnvelope`, `ProjectionDefinition`,
  `SnapshotMemory`, `PostureTransition`, public operation ID, generated
  command, or Tauri addition.

### Structural absorption

- Absorption targets are registered workflow roles; each target has one owner.
- Empty targets, duplicate unit IDs, duplicate targets, duplicate ownership,
  non-workflow targets, capability IDs, constitutional authority, multi-node
  cycles, and self-loops refuse.
- The deterministic Kahn traversal covers the directed graph
  `unit_id -> absorbs[]`; canonical role and unit ordering precedes hashing and
  rendering.
- The real renderer emits every valid directed edge. Unsupported silent
  flattening therefore cannot pass the exact edge-count/golden assertions.

### Real product path and deterministic replay

The existing Stage-10 Work Specification capture path opens the repository,
resolves its exact selected profile and vocabulary, admits the same canonical
YAML, and renders the fixed Markdown view. The non-empty proof profile changes
presentation only:

- heading: `# Work Specification — Feature`;
- two ordered vocabulary units;
- all three directed absorption edges;
- existing controlled Markdown escaping;
- exactly one terminal LF.

`capture_apply_stage_10_consumes_selected_vocabulary_without_absorption_loss`
runs the real preview path twice, asserts byte-identical Markdown against the
reviewed golden, asserts canonical YAML is unchanged, and checks exact unit and
edge counts. The existing shipped-empty capture golden remains unchanged. This
is a real `handbook-pipeline` consumer, not a unit-only vocabulary API.

## Command wall

| Command | Result |
|---|---|
| `cargo test -p handbook-engine --test vocabulary_registry` | PASS, 7/7 |
| `cargo test -p handbook-engine --test profile_selection` | PASS, 20/20 |
| `cargo test -p handbook-pipeline --test pipeline_capture` | PASS, 48/48 |
| `cargo test -p handbook-compiler --test pipeline_capture` | PASS, 39/39 |
| `cargo test -p handbook-cli --test cli_surface pipeline_capture_preview_stage_10_matches_shared_golden` | PASS, 1/1 |
| `cargo test --workspace --all-targets --all-features` | PASS, exit 0 after 993 seconds; includes engine, flow, pipeline, compiler, CLI, and all workspace regression targets |
| `cargo clippy --workspace --all-targets --all-features -- -D warnings` | PASS |
| `cargo fmt --all -- --check` | PASS |
| `git diff --check f0b5f4c..HEAD` | PASS |
| shipped-vocabulary diff and SHA-256 replay | PASS; no byte diff |
| production HCM-3.2/HCM-3.3/public-operation absence scan | PASS |
| Cargo/public-transport changed-path scan | PASS |
| ordinary handoff validation | PASS: 5 record schemas, 5 dispatch schemas, 2 templates, 78 records, 454 current dispatches, 8 admitted legacy dispatches, 78 ledger entries |
| historical v1.0 admission self-test | PASS |
| v1.4 causal/orchestration self-tests | PASS |

The handoff validators left `handoffs/ledger.jsonl` unchanged.

## GitNexus evidence

Before production edits, exact indexed struct/impl impact queries for
`VocabularyDefinition`, `AuthoredVocabulary`, and `ArtifactRepositoryV1`
returned LOW risk with zero indexed direct callers, processes, or modules.
Function-level queries for the Rust resolver/renderer/test functions were not
indexed and returned `UNKNOWN`; direct source call-path tracing and the CLEAN
selector bounded those edits. No HIGH or CRITICAL impact was returned.

After implementation and again at this proof baseline, both scoped and
compare-to-`main` change detection exited 1 with only the unavailable FTS
load-only warning. GitNexus concept/FTS and change-detection comparison are
therefore **unavailable**, not GREEN.

## Protected and deferred boundaries

All nine protected paths matched their initial SHA-256 values immediately
before the implementation-review commit and were absent from its index. The
unrelated untracked `docs/research/` and `docs/ideas/` material remains outside
this slice.

This proof does not implement or select HCM-3.2 Context Resolution, HCM-3.3
Projection, Snapshot Memory, posture transitions, task-gate runtime, adapter
contracts, public transports, new commands, dependencies, or a shipped
non-empty vocabulary. The Phase 3 exit remains open.
