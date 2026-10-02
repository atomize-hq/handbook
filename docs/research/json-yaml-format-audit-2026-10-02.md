# JSON/YAML intent, implementation, and reuse audit

Date: 2026-10-02. Product baseline: `e316fc6a84e32e92b13a1b2d9d632677fa111b7b`,
branch `feat/handbook-contract-membrane-clean`. Research and proposed documentation
only; no product or governance-plan edits. The [proposed standard](../specs/handbook-contract-membrane/structured-data-format-standard.md)
is the concise forward decision surface, awaiting human review.

## Conclusion and scope

The user's memory of substantial YAML migration is correct. The actual
`archived/` directory and later preserved HCM history describe a deliberate move
from Markdown-first project artifacts to structured canonical YAML, with reusable
validation, mutation and derived human views. Readability was only part of the
reason: structured downstream manipulation and avoiding duplicate truth mattered.

The historical rule was not "all human files YAML, all machine files JSON".
Machine-readable YAML state and caches were explicitly selected too. Contracts
JSON is a distinct current user selection and implemented domain choice, not an
accidental violation of the artifact migration. Neither the word "policy" nor a
`.yaml` extension is sufficient to determine intended storage semantics.

The user requested this audit before the pending governance implementation and
explicitly reaffirmed Contracts JSON and human-facing projections. Earlier chat
advice to switch governance storage to YAML was premature. This report separates
historical intent, selected current contracts, demonstrated implementation,
remaining gaps and proposed forward defaults; it does not reactivate archived
execution instructions or certify every old migration ambition complete.

A local research subagent inventoried 356 files in `archived/`, then read the
relevant migration, layout, renderer, state and legacy examples. This is a scoped
content investigation, not a claim that every archived file was read. The main
agent traced current parser/emitter/schema/owner/consumer paths and ran the focused
checks below. Extension counts were not used as behavioral proof.

## Historical intent and migration

| Evidence | What it establishes | Limit / current relation |
| --- | --- | --- |
| [June canonical-artifact alignment map](../../archived/specs/handbook-canonical-artifact-model-validation-alignment-map.md#L31) | Structured canonical YAML where useful; reusable validation, merge/update/reconstruction and derived views; explicitly rejects "YAML everywhere" and duplicate editable Markdown/YAML truth. | Planned broad direction, not proof every proposed family or operation shipped. The generic phrase "canonical contracts" means reusable artifact structures here, not today's `handbook-contracts` representation. |
| [Map sequencing](../../archived/specs/handbook-canonical-artifact-model-validation-alignment-map.md#L181) | Validation seam, foundation cutover, further core artifacts, then reconstruction/merge proof; family-specific choices. | Foundation/quality-gate ambitions were broader than the later six-family HCM conversion. |
| [Earlier reduced-v1 C-03](../../archived/reduced-v1/contracts/C-03-canonical-artifact-manifest-contract.md#L39) | Earlier canonical Markdown paths, with runtime state and derived views excluded from project-artifact authority. | Pre-migration baseline, not a reason to undo YAML conversion. |
| [Renderer/proof contract](../../archived/reduced-v1/contracts/C-05-renderer-and-proof-surfaces.md#L22) | Markdown operator output and JSON machine output over one typed result, with no recomputation of truth. | An output-format distinction, not a universal source-file rule. JSON itself can be a projection. |
| [Layout inventory](../../archived/specs/phase-1-layout-contract-and-inventory/handbook-engine-extraction-phase-1-slice-1-layout-contract-and-inventory-spec.md#L205) and [Flow layout injection](../../archived/specs/handbook-substrate-parameterization-set-2-flow-canonical-layout-injection-spec.md#L14) | Separately owned canonical, state, capture/provenance, handoff and authoring layouts. YAML caches coexist with JSON provenance. | Path/owner decoupling did not select a blanket format conversion. |
| [Pipeline state contract](../../archived/reduced-v1/contracts/pipeline-route-and-state-core.md#L85) and [capture contract](../../archived/reduced-v1/contracts/pipeline-capture-preview-and-apply.md#L131) | Explicit `.handbook/state/pipeline/<id>.yaml` and YAML capture-cache paths, despite machine-managed lifecycle. | Deliberate exceptions still present in current Rust state/capture code. |
| [Legacy library guidance](../../archived/reduced-v1/legacy/guides/mechanisms/library_and_templates.md#L20), [harness-state guidance](../../archived/reduced-v1/legacy/guides/mechanisms/harness_state.md#L1), [generated Charter input](../../archived/legacy-generated-artifacts/charter/CHARTER_INPUTS.yaml) and [generated state](../../archived/legacy-generated-artifacts/_harness_state.yaml) | YAML was used for both authoring and machine-readable structured state. | Historical examples, some skeletal; not proof of complete present policy or Work Progression owners. |

Later migration evidence is preserved at Git ref
`archive/handbook-contract-membrane-pre-clean-20260812`. These are Git-history
locations, not missing workspace links or current instructions:

- `docs/specs/handbook-contract-membrane/slices/HCM-2.1/SPEC.md`, lines 40–56:
  Project Context canonical YAML pilot, parsed into a JSON data model. Commit
  `d61670e` is `feat(handbook): land project context canonical YAML pilot`.
- `docs/specs/handbook-contract-membrane/slices/HCM-2.3/SPEC.md`, lines 151–160
  and 511–520: JSON profile selection, YAML definitions and canonical artifact,
  JSON Schema, deterministic YAML promotion with revalidation. Commit `628b672`
  is `feat(hcm-2.3): implement generic artifact lineage`.
- `docs/specs/handbook-contract-membrane/slices/HCM-2.4/SPEC.md`, lines 172–220:
  six-family conversion, explicit support/control-record exclusions, deterministic
  Markdown versus later Resolution-aware Projection, and separate/deferred
  foundation/test-strategy/quality-gate products.
- `docs/specs/handbook-contract-membrane/06-proof-and-regression-ledger.md`,
  line 199, `PG-YAML-02`: the six-family conversion boundary. Commit `e83b0c8`
  records Phase-2 closeout, not conversion of every repository file.

These sources explain the user's recollection and the narrowed implemented
boundary. Historical claims remain distinct from fresh tests and current source.

## Current format and lifecycle map

| Surface | Source-backed implementation | Status / implication |
| --- | --- | --- |
| Authored artifact promotion | [Generic promotion](../../crates/engine/src/artifact_mutation.rs#L1712) emits deterministic YAML, reparses, structurally validates and fingerprints output bytes. [Shipped profile](../../crates/engine/definitions/profiles/handbook.profile.shipped-root/1.2.0.yaml) selects three instances and registers six families. | Real runtime path. Work Specification, Decision Record and Risk Record support is not three additional default instances. |
| Charter and context authoring/views | [Charter method](../../core/library/authoring/charter_authoring_method.md#L3) selects canonical YAML and native promotion; line 31 prohibits direct edits to canonical YAML/Markdown. [Project Context](../../crates/engine/src/project_context_artifact.rs#L192) and [Environment Context](../../crates/engine/src/environment_context_artifact.rs#L179) provide typed serializers/renderers. | Human-readable source does not mean bypassing activation. Family-specific emitters may have their own byte ordering. |
| Work Specification | [Stage 10](../../core/stages/10_feature_spec.md#L8), [capture canonicalization](../../crates/pipeline/src/pipeline_capture.rs#L1908), [emission/rendering](../../crates/pipeline/src/pipeline_capture.rs#L2057), [provenance](../../crates/pipeline/src/stage_10_feature_spec_provenance.rs#L16). | Real selected-descriptor route and focused tests; broader work hierarchy/progression and old sprint/task templates are separate. |
| Profile selection versus definitions | [Selection JSON parser/path](../../crates/engine/src/artifact_intake_registry.rs#L11); profile/kind/intake YAML files and [definition parser](../../crates/engine/src/definition_identity.rs#L318). | Machine selection record differs from declarative definition content. Some `.yaml` built-ins are JSON-shaped text, valid YAML but not readable block-YAML presentation. |
| Contracts definitions/policies | [Owner paths](../../crates/contracts/src/owner_repository.rs#L2077) use JSON definitions, heads, transitions and idempotency records. [Lifecycle policy](../../crates/contracts/src/lifecycle_policy.rs#L9) and [evaluation policies](../../crates/contracts/src/evaluation_policy.rs#L26) embed canonical JSON. | Deliberate implementation and current user direction. No general YAML Contracts importer established by this audit. |
| Owner records/state | Artifact lineage, Contracts lifecycle/evidence and Snapshot Memory records use owner JSON; [Pipeline route state](../../crates/pipeline/src/route_state.rs#L719) and [capture cache](../../crates/pipeline/src/pipeline_capture.rs#L1329) serialize YAML, while [mutation journal](../../crates/pipeline/src/pipeline_mutation_journal.rs#L78) uses JSON. | Mixed formats have named owners and historical reasons; not grounds for a blanket conversion. |
| Dock/JSON Schema | [Completed HCM-5.4 authority](../specs/handbook-contract-membrane/slices/HCM-5.4/README.md#exact-validator-input-and-canonical-fingerprints), [validator wire](../../crates/dock-json-schema/src/wire.rs#L1), [Dock canonical framing](../../crates/dock/src/canonical.rs#L24). | JSON Schema adapter consumes exact inline JSON and offline local-ref closure. It contributes observations; Contracts owns semantic evidence/results. Contracts JSON and Dock wire JSON are distinct selected boundaries. No general YAML input adapter is implied. |
| Human/machine views | Typed Engine renderers plus [Flow packet](../../crates/flow/src/packet_result.rs) and [CLI JSON renderer](../../crates/cli/src/flow_rendering/json.rs). | A JSON or Markdown projection can carry owner truth without becoming its source. Not every semantic object has a shipped human view. |

## Reusable tooling

These are actual implementations, not a proposal to build a new universal codec.
"Public" means exported in the current crate; it does not imply every owner may
depend on that crate. Private helpers are precedents within their owners, not
ready-made cross-crate APIs.

| Tool / source | Reusable behavior | Limits and correct use |
| --- | --- | --- |
| Engine [`parse_definition_yaml`, `parse_schema_json`](../../crates/engine/src/definition_identity.rs#L318) | Public bounded parsers using recursive duplicate-key checks and JSON data values; 1 MiB source bound. | Parse/shape/domain admission remain separate. Do not advertise every YAML feature as rejected or portable without checking it. |
| Engine [`canonical_yaml_bytes`, `parse_canonical_yaml`](../../crates/engine/src/canonical_yaml.rs#L37) | Public deterministic block emitter, sorted keys, UTF-8/LF, quoted ambiguous strings, size bound and parse/emit/parse equality. | No comment/style retention. `parse_canonical_yaml` parses through the definition loader; its name alone does not prove supplied bytes equal the canonical emitter's output. |
| Engine [`DefinitionFingerprint`](../../crates/engine/src/definition_identity.rs#L191) | Public exact-byte hashing and canonical-JSON-value hashing. Private canonical JSON helper at line 333 backs value identity. | Byte identity and semantic value identity are not interchangeable. Promotion hashes emitted YAML bytes, while many definitions use JSON canonicalization. |
| Engine [`SchemaRegistryEntry::validate_json`](../../crates/engine/src/schema_registry.rs#L141) and [`ResolvedArtifactRegistry::validate_json`](../../crates/engine/src/artifact_registry.rs#L274) | Structural validation of parsed data under selected exact schemas; schema registry/closure machinery already exists. | Method name does not require JSON disk source. Structural conformance is not semantic admission or activation. |
| Engine typed context emitters/renderers and generic artifact intake/mutation | Domain validation, deterministic canonical output, derived Markdown, exact publication and lineage. | Reuse formatting/validation primitives without transferring generic authored-artifact lifecycle to unrelated owner records. No universal merge framework was demonstrated. |
| SDK [`canonical_json_bytes`, `parse_canonical_json`, `canonical_fingerprint`](../../crates/sdk/src/contract_membrane/serialization.rs#L26) | Public re-exports; bounded custom serializer/parser, duplicate checks, canonical lexical form, safe integers and UTF-16 key ordering. | Floats are rejected, unlike the broader Engine JSON-data-model parser's finite values. Keep this exact SDK domain; inward owners must not import SDK for convenience. |
| Contracts [`canonical_bytes`, `canonical_fingerprint`](../../crates/contracts/src/canonical.rs#L4) | Crate-private canonical JSON primitives used by exact definitions and repositories. | Serialization helper alone is not a complete untrusted-input parser. Typed constructors, validation and owner recovery still apply. |
| Dock [`admit_canonical_json`, framed JSON helpers](../../crates/dock/src/canonical.rs#L24) | Crate-private duplicate checking, bounded exact canonical bytes and one-line LF framing. | Stored canonical JSON and framed protocol JSON have different byte contracts. |
| Standalone validator [`canonical.rs`](../../crates/dock-json-schema/src/canonical.rs#L5), [`wire.rs`](../../crates/dock-json-schema/src/wire.rs#L1) | Local canonical hashing, bounded protocol handling, exact validator inputs. | Intentional independent executable dependency boundary. Do not add a Contracts/Engine dependency merely to deduplicate a small helper. |
| Engine [`RetainedRepositoryRootV1`](../../crates/engine/src/repository_durability.rs#L234) | Public bounded reads/listing, exclusive creation, durable publication, exact replacement and native locks. | Format-independent primitives. Private authority-tree guards and another owner's transaction semantics are not generic governance guarantees. |
| Python [`yaml_lite.py`](../../tools/yaml_lite.py#L1), [`harness.py`](../../tools/harness.py#L1), [`validate_profile.py`](../../tools/validate_profile.py#L1) | Existing subset loader/dumper and older pipeline/profile tooling. | Different grammar and guarantees: fallback `safe_load` returns the first stream document. Not interchangeable with bounded Rust owner admission; do not introduce a Python runtime dependency for this work. |

Dependency manifests already include `serde_json`, `serde_yaml_bw`,
`serde_json_canonicalizer`, and Rust `jsonschema` in the owners that use them.
Existing helper reuse needs no new serialization platform. Shared libraries do
not imply identical number domains, byte ordering, framing, limits, or lifecycle.

## Gaps and proposed disposition

1. **Migration was substantial but scoped.** Preserve six-family YAML and current
   selected Work Specification capture; do not claim all old foundation/quality-
   gate products or universal work-item management landed.
2. **No global format rule currently resolves every new object.** Adopt explicit
   forward defaults by role and supported interface, retaining current exceptions.
3. **YAML extension is insufficient proof of readability.** Existing JSON-shaped
   YAML assets are supported inputs; improving them is separate work with exact
   identity checks, not a silent rewrite during this audit.
4. **Authoring and activation are separate.** A YAML input can normalize to a
   selected JSON owner record, but there must be one authority and an explicit
   correspondence. Do not assume a file importer exists because a parser does.
5. **Merge/reconstruction ambitions exceed demonstrated reusable primitives.**
   Current evidence supports parsing, validation, deterministic emission, selected
   owner mutations and views. Comment-preserving round trips and universal merge
   semantics remain unproven; no new implementation obligation is created here.
6. **Pending governance plan:** its SDK-managed JSON path is defensible. Direct
   YAML candidate authoring is a distinct optional interface choice. Do not revise
   the plan's encoding merely from the noun "policy". Reconcile the human's
   configurable-ceiling clarification before implementation approval; 24 hours is
   an example/test value, not a mandatory default.

## Verification and limits

Fresh focused checks at the baseline, all passed:

| Command | Result |
| --- | --- |
| `cargo test -p handbook-engine --test definition_identity` | 5 passed: exact refs/bounds, fingerprints, duplicate YAML/JSON keys, document/aggregate bounds. |
| `cargo test -p handbook-engine --test hcm_2_3_registration_kernel canonical_yaml_is_utf8_lf_deterministic_and_round_trips_the_json_model` | 1 passed: deterministic block YAML, fixed byte fingerprint and semantic round trip. |
| `cargo test -p handbook-sdk --test contract_membrane canonical_json_is_bounded_sorted_lf_free_and_lexeme_preserving` | 1 passed: canonical bytes/lexemes, duplicate and numeric/size refusals. |
| `cargo test -p handbook-pipeline --test pipeline_capture stage_10` | 15 passed: actual selected YAML capture, derived view, duplicate/schema/Markdown-input/tampered-view refusals and related Stage-10 behavior. |

Other cited tests were inspected, not freshly executed. No full workspace suite,
external adoption proof, all-owner parser fuzzing, general YAML feature matrix or
global migration conformance is claimed.

GitNexus MCP was unavailable. `node .gitnexus/run.cjs query serialization` ran but
reported missing FTS tables, a foreign-worktree registry and an index 24 commits
behind; empty results establish no absence claim. No reindex was performed.
Documentation-only edits touch no product symbols; pre-commit change detection,
local-link checks and focused causal review are reported in the completion chat.
The three pre-existing untracked consultation files are outside this work.
