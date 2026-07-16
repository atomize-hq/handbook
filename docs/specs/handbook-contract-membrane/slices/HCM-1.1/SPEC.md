# HCM-1.1 Specification: Artifact-Kind and Local Schema Registry

## Status and authority

This is the approved first implementation packet for HCM-1.1. It is authorized
only by the completed HCM-0.7 planning closeout and the `HCM-1.1` row in
[`../../04-phase-slice-map.md`](../../04-phase-slice-map.md). Execute it in a
new top-level orchestration run with:

```text
PHASE_ID: HCM-1
SLICE_ID: HCM-1.1
ACTIVE_PACKET: docs/specs/handbook-contract-membrane/slices/HCM-1.1
HANDOFF_SELECTOR: <exact completed HCM-0.7 parent handoff supplied by closeout>
```

The selected HCM-0.7 handoff is completed dependency/packet context. It does
not select HCM-1.1, widen this packet, or authorize a later slice.

Canonical authority for this packet is:

1. this `SPEC.md`, [`tasks/plan.md`](tasks/plan.md), and
   [`tasks/todo.md`](tasks/todo.md);
2. the schema policy, uniform identity, schema-registry, stable-role,
   artifact-kind, and semantic-capability sections in
   [`../../05-contracts-schemas-and-gates.md`](../../05-contracts-schemas-and-gates.md);
3. the HCM-1.1 row, Phase 1 ordering, and HCM-0.7 approval contract in
   [`../../04-phase-slice-map.md`](../../04-phase-slice-map.md);
4. the Artifact kind/schema registry and fixed-artifact coupling rows in
   [`../../03-seam-crosswalk.md`](../../03-seam-crosswalk.md); and
5. `PR-002`, `PR-004`, `PR-008`, `PG-KIND-01`, `PG-KIND-02`, and regression
   rules 1-4, 13-14, and 29-32 in
   [`../../06-proof-and-regression-ledger.md`](../../06-proof-and-regression-ledger.md).

## Objective

Land an additive `handbook-engine` owner boundary that can:

1. load exact versioned schema-registry entries from explicitly selected,
   trusted repository-relative sources;
2. resolve one bounded local Draft 2020-12 schema closure with deterministic
   byte and closure fingerprints;
3. load and meta-validate exact versioned `ArtifactKindDefinition` records
   independently from repository artifact-instance state;
4. resolve the selected stable-role registry exactly and refuse every non-empty
   later-owned semantic-capability or behavioral dependency field rather than
   trusting an unrecomputable caller-supplied fingerprint;
5. structurally validate a JSON data model through the selected kind's exact
   canonical schema; and
6. prove one capability-free repository-defined custom kind end to end without
   a new `CanonicalArtifactKind` variant, generated CLI command, executable
   hook, remote fetch, or generic Projection engine.

The implementation is a registry foundation, not a product-path cutover.

## Live baseline and blast-radius boundary

Live repository truth at packet approval:

- `crates/engine/src/canonical_artifacts.rs` exposes the fixed four-variant
  `CanonicalArtifactKind` and fixed `CanonicalArtifactDescriptor` table;
- `crates/engine/src/canonical_paths.rs` and current setup/flow code dispatch on
  that enum;
- `crates/engine/src/canonical_repo_support.rs` already supplies normalized
  repo-relative paths, component-by-component no-follow checks, and trusted
  regular-file reads;
- `crates/engine/src/freshness.rs` supplies a useful SHA-256 precursor but not
  the uniform RFC 8785 definition closure required here;
- `handbook-engine` has no first-class schema/kind registry and no direct JSON
  Schema dependency; and
- current engine/flow/setup/doctor behavior is preserved until HCM-1.3 and
  HCM-1.4.

Before changing any existing function, type, or method, run repository-required
GitNexus upstream impact analysis on that exact symbol and warn before HIGH or
CRITICAL changes. New symbols still require context inspection of the owning
module and callers at their integration point. HCM-1.1 must not turn the fixed
enum's high coupling into an opportunistic refactor.

## Required skill chain

The HCM-1.1 runner applies, in order as their phases begin:

1. `using-agent-skills`;
2. `context-engineering`;
3. `source-driven-development`;
4. `spec-driven-development` and `planning-and-task-breakdown` for packet
   revalidation only;
5. `api-and-interface-design`;
6. `security-and-hardening`;
7. `incremental-implementation`;
8. `test-driven-development`;
9. `debugging-and-error-recovery` for every failure;
10. `code-review-and-quality`; and
11. `git-workflow-and-versioning`.

## Exact allowed scope

### Existing files

- `crates/engine/Cargo.toml`;
- `Cargo.lock`;
- `crates/engine/src/lib.rs`;
- `crates/engine/src/canonical_repo_support.rs`, only for the smallest reusable
  visibility/API change required by registry code after impact analysis;
- `docs/specs/handbook-contract-membrane/03-seam-crosswalk.md` and
  `docs/specs/handbook-contract-membrane/06-proof-and-regression-ledger.md`,
  only for evidence-backed HCM-1.1 closeout truth; and
- this packet's todo/proof status.

### New source and asset areas

- `crates/engine/src/definition_identity.rs`;
- `crates/engine/src/stable_role_registry.rs`;
- `crates/engine/src/schema_registry.rs`;
- `crates/engine/src/artifact_kind_registry.rs`;
- `crates/engine/definitions/stable-roles/handbook.roles.core/1.0.0.yaml`;
- `crates/engine/definitions/stable-roles/handbook.roles.core/1.1.0.yaml`;
- focused `crates/engine/tests/*registry*.rs` and
  `crates/engine/tests/*definition*.rs` tests;
- `crates/engine/tests/fixtures/hcm_1_1_custom_kind/`; and
- HCM-1.1 proof/review dispatch evidence and the parent closeout artifacts
  allowed by canonical `07`/`08`.

No other existing production module is in scope. If the correct implementation
requires touching `canonical_artifacts.rs`, `canonical_paths.rs`, compiler,
flow, setup, doctor, CLI, pipeline, or a sibling crate, stop with
`authority_boundary` rather than widening.

## Dependency and official-source posture

Use exactly these direct engine dependency changes. The four registry/identity
dependencies are additive; safe descriptor-relative traversal replaces the
existing direct `libc` dependency with the safe `rustix` filesystem API after
Fresh Final Review 1 proved that path-wide metadata checks plus final-component
`O_NOFOLLOW` were raceable.

```toml
jsonschema = { version = "0.47.0", default-features = false }
rustix = { version = "1.1.4", features = ["fs"] } # cfg(unix); replaces libc
semver = "1"
serde_json = "1"
serde_json_canonicalizer = "0.3.2"
```

Keep existing `serde`, `serde_yaml_bw`, and `sha2` dependencies. The lockfile,
not a floating network install, records the concrete resolved graph.

Implementation must follow the current official APIs:

- `jsonschema` Draft 2020-12 meta-schema validation and reusable validators:
  <https://docs.rs/jsonschema/0.47.0/jsonschema/>;
- resolver features disabled with `default-features = false`, followed by an
  explicit in-memory registry rather than HTTP/file retrieval:
  <https://docs.rs/jsonschema/0.47.0/jsonschema/#external-references>;
- the linear-time regular-expression engine rather than default backtracking:
  <https://docs.rs/jsonschema/0.47.0/jsonschema/#example-use-the-regex-engine-instead>;
  and
- RFC 8785-compatible `to_vec` canonicalization:
  <https://docs.rs/serde_json_canonicalizer/0.3.2/serde_json_canonicalizer/>;
  and
- safe descriptor-relative `openat` plus `NOFOLLOW`/`DIRECTORY` flags:
  <https://docs.rs/rustix/1.1.4/rustix/fs/fn.openat.html> and
  <https://docs.rs/rustix/1.1.4/rustix/fs/struct.OFlags.html>.

The strict registry path is target-Unix and treats the explicitly selected
repository root as the caller-provided trust anchor, including when that root
is selected through a directory symlink. Every repo-relative component beneath
that anchor still uses descriptor-relative no-follow opens, with `NONBLOCK` on
the final component, retained-handle metadata, and a bounded retained-handle
read. Registry loads fail closed on non-Unix.
That strict path is separate from the pre-existing canonical-product reader:
the latter retains its prior non-Unix behavior so HCM-1.1 does not turn a
registry security boundary into a canonical-artifact regression.

Do not enable `resolve-http`, `resolve-file`, `resolve-async`, macros, custom
keywords, custom formats, or an HTTP/TLS client. A dependency or API conflict
with the pinned packet is a stop for bounded packet repair before code proceeds.

## Public owner boundary

`handbook-engine` exposes typed, transport-neutral library APIs. Exact Rust
layout may use submodules, but the public semantic surface must include:

- `ExactDefinitionRef` — validated `namespaced-id@full-SemVer`;
- `DefinitionFingerprint` — validated lowercase `sha256:<64-hex>`;
- `StableRoleRegistry` and `StableRoleDefinition`;
- `SchemaRegistryEntry`, `ResolvedSchema`, and `SchemaRegistry`;
- `ArtifactKindDefinition` and `ArtifactKindRegistry`;
- `ArtifactKindRegistryLoadRequest` with explicit source paths, allowed schema
  roots, and no untyped external dependency injection;
- `RegistryLoadError` with a typed `RegistryLoadErrorKind`; and
- `StructuralValidationError` with deterministic instance/schema locations.

The primary entry point is conceptually:

```rust,ignore
pub fn load_artifact_kind_registry(
    repo_root: impl AsRef<std::path::Path>,
    request: ArtifactKindRegistryLoadRequest,
) -> Result<ArtifactKindRegistry, RegistryLoadError>;
```

The registry exposes exact-ref lookups and a structural validation operation
conceptually equivalent to:

```rust,ignore
pub fn validate_json(
    &self,
    kind_ref: &ExactDefinitionRef,
    instance: &serde_json::Value,
) -> Result<(), Vec<StructuralValidationError>>;
```

Names may change only when implementation proves a materially simpler typed
surface and the packet is updated before code. Do not expose a public
`serde_json::Value` dispatcher for unrelated operations, a stringly command
router, or transport wording. This local public API is not published or
downstream-proven by HCM-1.1; `PG-PUBLISH-01` remains open.

## Exact source and asset topology

The request names every source explicitly. There is no directory scan, implicit
default, current-working-directory discovery, filename-derived identity, or
source-order winner.

- Built-in stable-role registry bytes are package-owned at the two exact
  `crates/engine/definitions/stable-roles/handbook.roles.core/` paths above and
  selected by exact ref; both exact HCM-0.2/HCM-0.6 fingerprints must reproduce.
- Schema-registry entries are YAML records at explicit repository-relative
  paths supplied in the request.
- Artifact-kind definitions are YAML records at explicit repository-relative
  paths supplied in the request.
- Canonical schema documents are JSON regular files selected only through a
  validated `SchemaRegistryEntry.document_ref` and an explicitly declared
  allowed schema root.
- HCM-1.1 accepts no caller-supplied opaque fingerprint as a dependency
  producer. Every non-empty semantic capability, required capability, semantic
  validator, renderer, lifecycle, review-trigger, Projection, extension, or
  other later-owned behavioral dependency field refuses. Its owning later
  slice must supply exact source bytes plus type-specific parsing,
  normalization, compatibility, and fingerprint recomputation before enabling
  that field.

The custom-kind proof fixture lives only under
`crates/engine/tests/fixtures/hcm_1_1_custom_kind/`. It is repository-defined
proof data, not a first-party shipped kind or product default.

## Uniform exact identity and fingerprint contract

### Exact refs

- HCM-1.1 definition identities are 3-255 ASCII bytes, contain at least two
  dot-separated segments, and use this exact segment grammar:
  `[a-z][a-z0-9]*(?:-[a-z0-9]+)*`. Each segment is 1-63 bytes.
- Lowercase ASCII bytes are authoritative. Do not trim, case-fold, Unicode-
  normalize, percent-decode, or otherwise normalize input. Uppercase, Unicode,
  `_`, `/`, `%`, `@`, whitespace, empty/double-dot segments, and leading,
  trailing, or repeated `-` refuse.
- Parse the declared identity and full SemVer independently. The version lexeme
  must be accepted by `semver::Version`, include major/minor/patch, and equal
  that parsed version's canonical `to_string()` bytes; ranges, partials,
  aliases, and non-canonical lexemes refuse.
- Derive the exact ref mechanically as `identity + "@" + version`.
- Reject bare IDs, version ranges, partial versions, `latest`, whitespace
  aliases, mismatched declared refs, or multiple records for one exact ref.
- Because the identity grammar forbids `@`, an exact ref contains exactly one
  unambiguous identity/version delimiter. This is an HCM-1.1 admission profile,
  not a retroactive rewrite of already frozen definition bytes; a later slice
  that needs a wider global grammar must obtain reviewed authority first.
- Never auto-substitute stable-role registry 1.1.0 for a 1.0.0 pin.

### Closed record decoding

- Stable-role registries, schema-registry entries, artifact-kind definitions,
  and every nested record use closed typed deserialization with unknown-field
  refusal (`deny_unknown_fields` or equivalent).
- Duplicate and unknown fields refuse before normalization or fingerprinting;
  silently dropped bytes can never disappear from a semantic preimage.
- A known instance/profile/intake/behavior field placed in the wrong record is
  still an unknown-field error; field-name recognition is type-local.
- `extensions` is exactly `{}` for both schema-registry entries and kind
  definitions in HCM-1.1. Non-empty extensions refuse until a later owning
  packet supplies an exact extension schema and recomputable fingerprint
  producer.

### Fingerprint normalization

- Parse YAML definition records into the JSON data model with duplicate mapping
  keys rejected.
- Parse JSON schema documents with duplicate object keys rejected.
- Deserialize admitted definition records through the closed type for that
  exact record schema/version before normalization.
- Remove only the record's own derived fingerprint field.
- Normalize each type's arrays before RFC 8785 serialization:
  - preserve `StableRoleRegistry.roles` authored order because both frozen core
    registry fingerprints bind that order;
  - reject non-empty later-owned dependency arrays before normalization; and
  - sort contract-unordered exact-ref sets lexicographically.
- Preserve arrays whose contract is explicitly ordered.
- Canonicalize with RFC 8785 and hash the UTF-8 bytes with SHA-256.
- Exclude timestamps, absolute machine paths, source filenames other than
  semantically owned normalized repo-relative refs, and presentation-only
  ordering.
- Recompute every admitted fingerprint from typed source bytes and refuse
  mismatch; callers never authorize a digest by supplying it. Because HCM-1.1
  has no type-specific producers for later-owned behavioral definitions, those
  fields are accepted only empty/null.

### Schema fingerprints

- `document_fingerprint` hashes the exact root document bytes.
- `closure_fingerprint` hashes an RFC 8785 array containing the root and every
  transitively resolved local schema document as
  `{document_ref, document_fingerprint}`, sorted by normalized
  `document_ref`.
- `entry_fingerprint` is the uniform exact-definition fingerprint of the
  registry entry after the recomputed document/closure fingerprints are bound.
- `schema_registry_fingerprint` hashes an RFC 8785 array of
  `{entry_ref, entry_fingerprint, closure_fingerprint}` sorted by `entry_ref`.
- `kind_registry_fingerprint` hashes an RFC 8785 array of
  `{kind_ref, definition_fingerprint}` sorted by `kind_ref`.
- `registry_fingerprint` and the capability-free kind
  `definition_fingerprint` include only their exact admitted typed closure and
  exclude only themselves. HCM-1.1 produces no capability fingerprint.

## Safe schema-closure algorithm

1. Normalize and validate every request source path and allowed schema root
   through the existing engine repo-relative path contract.
2. On Unix, open the explicitly selected repository root as a caller-provided
   trust anchor, then read every repo-relative path component through a retained
   parent descriptor with no-follow behavior; open the final component
   nonblocking, verify the retained handle is regular, and only then read it.
   Resolve an ambiguous intermediate `ENOTDIR` with descriptor-relative
   no-follow metadata so a symlink and an ordinary non-directory retain
   distinct typed failures. Registry loads fail closed on non-Unix without
   changing the separate legacy canonical-product reader.
3. Enforce fixed v1 bounds before parsing:
   - at most 1 MiB per source document;
   - at most 8 MiB total source bytes per load;
   - at most 128 schema documents in one closure; and
   - at most 32 transitive local-ref edges on one traversal path.
4. Parse every JSON document with duplicate-key rejection and validate it
   against the exact Draft 2020-12 meta-schema, then apply the narrower HCM-1.1
   v1 identifier/resource policy: authored `$id`, `$anchor`, `$dynamicAnchor`,
   `$recursiveAnchor`, `$recursiveRef`, and `$dynamicRef` are unsupported and
   refuse wherever they occur. Each admitted file is exactly one schema
   resource; no embedded authored resource or dynamic scope exists.
5. Walk every `$ref` value before validator construction:
   - a fragment-only reference must be an RFC 6901 JSON Pointer in the current
     document; plain-name anchors are unsupported and refuse;
   - a non-fragment reference must be a relative UTF-8 forward-slash path with
     an optional RFC 6901 JSON Pointer fragment;
   - absolute paths, URI schemes/authorities/queries, backslashes, percent-
     encoded traversal, and `..` components refuse;
   - the normalized target must remain under one explicitly allowed schema root;
   - each target is read through the same no-follow/regular-file/size/meta-
     schema rules; and
   - missing refs, cycles, over-depth, over-count, or conflicting bytes refuse.
6. Map each validated normalized path to one unique deterministic internal
   `handbook+repo:///...` resource identity. Percent-encode every non-
   unreserved UTF-8 path byte while preserving `/`, and prove the mapping is
   injective and reversible. The loader supplies that identity as the
   document's validator-time base without changing fingerprinted authored
   bytes. This internal identity has no authority and is never accepted as an
   authored ref. Duplicate internal identities or aliases refuse.
7. Resolve the prewalked reference table against those exact internal bases,
   register the same complete resource table in memory, and translate every
   already-prewalked `$ref` to an exact private absolute URI in a validator-only
   clone. Cross-document refs use the exact target resource; fragment-only refs
   use the current document's exact private resource. Apply the same injective
   encoding to the prewalked RFC 6901 fragment, preserving `/` and unreserved
   bytes, so spaces, non-ASCII tokens, and other reserved bytes remain valid
   private URI fragments. Source
   bytes and fingerprints remain authored-byte based. Configure Draft 2020-12
   with the linear-time regex engine and build the validator with every crate
   resolver feature disabled. Decode reported private path and fragment bytes
   back to the exact repo-relative path and JSON Pointer. The prewalk target,
   validator target, closure fingerprint member, and reported schema location
   must agree for every `$ref`; disagreement is a typed load failure.
8. Never register repository-provided executable keywords/formats, call a
   command, inspect `PATH`, load a shared library, fetch a URL, or read a file
   that is not in the prevalidated closure.

### Frozen v1 dialect and keyword profile

- Every admitted schema document root is an object whose `$schema` is present
  and exactly `https://json-schema.org/draft/2020-12/schema`; the selected
  `SchemaRegistryEntry.meta_schema_ref` must equal that URI. Missing, different,
  duplicate, or nested `$schema` declarations refuse.
- Boolean subschemas are allowed, but a document root cannot be boolean because
  it must carry the exact dialect declaration above.
- At schema-object positions the only admitted keywords are:
  - core: `$schema`, `$ref`, `$defs`, `$comment`;
  - applicator/unevaluated: `allOf`, `anyOf`, `oneOf`, `not`, `if`, `then`,
    `else`, `dependentSchemas`, `prefixItems`, `items`, `contains`,
    `properties`, `patternProperties`, `additionalProperties`, `propertyNames`,
    `unevaluatedItems`, and `unevaluatedProperties`;
  - validation: `type`, `enum`, `const`, `multipleOf`, `maximum`,
    `exclusiveMaximum`, `minimum`, `exclusiveMinimum`, `maxLength`,
    `minLength`, `pattern`, `maxItems`, `minItems`, `uniqueItems`,
    `maxContains`, `minContains`, `maxProperties`, `minProperties`, `required`,
    and `dependentRequired`; and
  - annotations: `title`, `description`, `default`, `deprecated`, `readOnly`,
    `writeOnly`, and `examples`.
- The schema-aware prewalk descends only through the exact subschema positions
  named by those admitted keywords. Map keys beneath `$defs`, `properties`,
  `patternProperties`, and `dependentSchemas` are names/patterns, while their
  values are schema nodes. Objects carried as instance data by `const`, `enum`,
  `default`, or `examples` are not misclassified as schema nodes.
- Every other keyword at a schema-object position refuses before closure
  fingerprinting or validator construction, including misspellings, unknown
  annotations, `$vocabulary`, `format`, and `content*`. Meta-schema validity is
  necessary but does not override this stricter allowlist.
- A later packet may widen this profile only by naming the exact vocabulary,
  deterministic semantics, fingerprint effect, dependency feature posture,
  and positive/negative proof. Unknown keywords never silently degrade to
  annotations in HCM-1.1.

Structural schema success grants no semantic, intake, approval,
constitutional, contract, or gate authority.

## Stable-role and capability-free kind meta-validation

### Stable-role registry

- accept only the exact record schema/version and full SemVer identity;
- reject every unknown top-level or nested role field before fingerprinting;
- require unique role IDs and one allowlisted category per role;
- reproduce the frozen 1.0.0 and 1.1.0 fingerprints from packaged bytes;
- keep `environment_context` distinct from `project_context`; and
- select one exact registry ref/fingerprint without ambient minor-version
  substitution.

### Later-owned semantic and behavioral definitions

HCM-1.1 publishes or loads no semantic-capability contract and exposes no
opaque dependency-injection API. It refuses non-empty semantic-capability
declarations, required capabilities, semantic validators, fixed renderers,
lifecycle policy, review triggers, Projection refs, and extensions before any
supplied ref/fingerprint can influence a kind fingerprint. This also refuses a
well-formed forged digest, changed bytes behind the same ref, wrong dependency
type, missing producer, and source-order substitution.

The first later slice that enables one of those fields must first freeze and
implement its type-specific source schema, normalization/fingerprint producer,
compatibility rules, and non-vacuous proof. Semantic-capability binding support
additionally requires a machine-readable expected-shape contract and exact
schema-shape compatibility algorithm; pointer existence alone is insufficient.

### Artifact-kind definitions

- accept only `handbook.artifact-kind-definition` / `1.0`;
- reject every unknown top-level or nested kind field before fingerprinting;
- derive `kind_id@kind_version`, require full SemVer, and allow only
  `compatibility: exact`;
- require the exact selected stable-role registry ref/fingerprint;
- resolve `canonical_schema_ref` to exactly one schema-registry entry;
- keep path, instance label, requiredness, instance dependency, materialization,
  and setup state structurally absent from the kind record;
- validate every supported role against the selected registry;
- require semantic capabilities, required capabilities, semantic validators,
  fixed renderers, lifecycle policy, and review triggers to be empty/null;
- keep intake compatibility one-way from intake definition to kind, so no
  intake definition appears in or fingerprints a kind;
- require `projection_definition_refs: []` in HCM-1.1 because Phase 3 has not
  landed;
- require `extensions: {}`; and
- derive `definition_fingerprint` over the normalized capability-free record,
  the selected stable-role registry fingerprint, and the resolved canonical
  schema-entry/closure fingerprints only.

The HCM-1.1 custom fixture intentionally uses no semantic capabilities,
required capabilities, semantic validators, renderers, lifecycle policy,
review triggers, projections, or extensions. Empty/null values are an explicit
valid HCM-1.1 contract path, not proof that those later definitions exist or
are supported by this registry version.

## Typed failure contract

`RegistryLoadErrorKind` must distinguish at least:

- invalid or unsafe source path;
- missing/non-regular/symlinked/read-failed source;
- source/document/aggregate limit exceeded;
- duplicate-key or syntax parse failure;
- unsupported record schema/version, meta-schema, media type, compatibility, or
  structural validation profile;
- missing/conflicting/nested dialect declaration or unknown/unsupported schema
  keyword at a schema-object position;
- unsupported authored identifier/resource/anchor/dynamic-reference keyword,
  invalid JSON Pointer fragment, duplicate internal resource identity, or
  prewalk/validator target mismatch;
- unknown top-level/nested definition field, field in the wrong record type, or
  non-empty schema-entry/kind extension;
- invalid identity/SemVer/exact ref/fingerprint syntax;
- fingerprint mismatch;
- duplicate/conflicting exact identity;
- remote/ambient/executable reference refused;
- local reference missing/outside root/cyclic/over-depth;
- stable-role registry mismatch or unknown role;
- schema ref missing or conflicting;
- unsupported non-empty semantic-capability, required-capability,
  semantic-validator, renderer, lifecycle, review-trigger, Projection,
  extension, or other later-owned dependency input; and
- caller-supplied opaque dependency producer refused.

Tests assert typed discriminants and stable structured locations, not prose.
Errors must not contain absolute machine paths or unbounded source contents.
Every registry error location is capped at 240 bytes with a stable fallback;
source-path, exact-identity, document-fingerprint, closure-fingerprint, and
canonical-schema-ref failures use stable field locations rather than echoing
authored values. Structural instance/schema locations are capped at 512 bytes;
an overlong or undecodable schema location uses the stable `schema_root`
fallback rather than an authored document path.

## Threat model and fail-closed posture

| Boundary | Abuse case | Required control |
|---|---|---|
| explicit source path | traversal or symlink escape | normalized repo-relative path plus component and final-file no-follow checks |
| schema resource/`$ref` | base rebinding, dynamic-scope substitution, SSRF, ambient file read, encoded traversal | refuse authored IDs/anchors/dynamic refs; JSON-Pointer-only relative prewalk; exact internal bases; allowed-root containment; resolver features disabled; in-memory closure; prewalk/validator target equality |
| schema keywords/format | executable extension or catastrophic regex | no custom keyword/format registration; linear-time regex engine |
| schema dialect/vocabulary | typo or unknown keyword silently treated as annotation | exact root Draft 2020-12 declaration; no nested dialect; closed schema-position keyword allowlist before fingerprint/validator construction |
| YAML/JSON record | duplicate-key smuggling or oversized input | duplicate rejection and fixed byte/document/depth limits |
| definition shape | typo/unknown-field or wrong-record semantic smuggling | closed typed deserialization at every nesting level; empty extensions; refusal before fingerprinting |
| identity/fingerprint | source-order substitution or stale digest | exact refs, recomputation, conflict refusal, deterministic typed closure |
| later-owned dependency | forged digest, incompatible binding shape, or label/filename authority | refuse all non-empty fields until an owning slice supplies type-specific bytes, normalization, compatibility, and non-vacuous proof |
| structural success | false semantic/constitutional authority | separate typed layer; no promotion beyond structural validation |

No fallback may convert an indeterminate or unsupported case into success.

## Required custom-kind proof

The fixture defines a primary repository-local, capability-free kind such as an
incident brief plus one distinct proof-only companion kind. It must include:

1. two distinct exact schema-registry entries;
2. two distinct local Draft 2020-12 content-schema roots, with the primary
   closure using at least one local `$ref` so the closure path is real;
3. two distinct artifact-kind definitions pinned to
   `handbook.roles.core@1.1.0`;
4. one valid YAML data instance for the primary kind parsed into the JSON data
   model; and
5. one invalid primary-kind instance generated by the test or fixture.

The proof must show:

- both accepted source permutations produce identical
  `schema_registry_fingerprint`, `kind_registry_fingerprint`, exact entry/kind
  lookup sets, per-entry closures, and validation behavior; a one-element
  reversal is not source-order proof;
- every entry/kind/closure fingerprint is deterministic across repeated loads;
- the valid instance passes and invalid instance returns deterministic
  structural locations;
- the custom kind exact ref is not a `CanonicalArtifactKind` variant;
- no production enum, layout, setup, doctor, flow, CLI, SDK, Tauri, Substrate,
  intake, renderer, Projection, contract, or dock path changes; and
- the packaged core stable-role assets are present in `handbook-engine` package
  contents without claiming crates.io publication.

## TDD and incremental implementation contract

Each task in [`tasks/plan.md`](tasks/plan.md) follows RED -> GREEN -> REFACTOR:

1. write the focused failing test and capture the intended failure;
2. implement the smallest owner-library behavior that makes it pass;
3. run the focused test and `cargo fmt --all -- --check`;
4. run affected engine tests before expanding; and
5. commit only after the increment is green and repository-required staged
   change detection passes.

Do not batch all source, assets, tests, and dependency changes into one
unreviewable increment. No task may opportunistically clean current fixed
artifact code.

## Full proof wall

Before HCM-1.1 closeout, preserve exact command/result evidence for:

### Focused positive proof

- exact-ref/SemVer/fingerprint normalization tests;
- both built-in stable-role registry fingerprint tests;
- safe root plus local-ref closure loading;
- Draft 2020-12 meta-schema acceptance;
- custom kind load/lookup and deterministic fingerprint replay;
- valid instance structural validation; and
- package-content presence for built-in registry assets.

### Negative and fail-closed proof

- duplicate YAML and JSON keys;
- unknown top-level and nested stable-role/schema-entry/kind fields, known
  instance/profile/intake/behavior fields inserted into the wrong record, and
  non-empty schema-entry or kind extensions;
- invalid/partial/range/latest/mismatched refs and fingerprints;
- duplicate and conflicting stable-role/schema/kind identities in both source
  orders;
- missing, absolute, escaping, symlinked, non-regular, over-size, over-total,
  over-count, and over-depth sources;
- remote/file/data/unknown-scheme refs, query refs, backslashes, encoded or raw
  traversal, unresolved refs, closure cycles, and byte drift;
- root/nested authored `$id`, relative/absolute identifiers, `$anchor`,
  `$dynamicAnchor`, `$dynamicRef`, recursive-reference keywords, plain-name
  fragments, duplicate internal resource identities/aliases, and any mismatch
  among prewalk, closure fingerprint, validator target, or schema location;
- unsupported dialect/media type/compatibility and invalid schema meta-shape;
- missing/wrong/nested `$schema`, root/nested keyword typos, unknown annotation
  keywords, `$vocabulary`, `format`, `content*`, and a proof that object-valued
  instance data under `const`/`enum`/`default`/`examples` is not misclassified;
- wrong role registry and unknown/category-invalid role;
- every non-empty semantic-capability, required-capability, semantic-validator,
  renderer, lifecycle, review-trigger, Projection, extension, opaque dependency,
  forged digest, changed-byte, wrong-kind, and source-order substitution input;
  and
- invalid custom-kind instance with stable structural error locations.
- long normalized document refs cannot escape the 512-byte structural-location
  bound, and admitted space/non-ASCII paths round-trip through collision-free
  internal URIs without reported-location drift.
- cross-document RFC 6901 fragments with space and non-ASCII tokens load,
  target the exact prewalked schema, and report the decoded pointer exactly.
- same-document RFC 6901 fragments with space and non-ASCII tokens satisfy the
  same exact target, fingerprint, validation, and decoded-location proof.
- a canonical product load selected through a repository-root directory symlink
  is byte-identical to the real-root load, while a symlink in any relative
  artifact component remains refused.

### Regression and repository proof

Run at minimum:

```bash
cargo fmt --all -- --check
cargo clippy -p handbook-engine --all-targets -- -D warnings
cargo test -p handbook-engine
cargo test --workspace
cargo tree -p handbook-engine -e features
cargo package -p handbook-engine --allow-dirty --no-verify
python3 tools/check_archive_boundary.py --self-test
python3 tools/check_archive_boundary.py
python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py
python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-v1-admission
python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-orchestration-contract
git diff --check
```

Additionally assert from `cargo tree`/package contents that:

- `jsonschema` resolver features are disabled;
- no `reqwest`, HTTP/TLS, async resolver, or executable schema hook dependency
  entered the engine graph;
- the two packaged stable-role assets are present; and
- changed paths stay within this packet's allowed scope.

Run GitNexus change detection before each commit and inspect affected symbols,
execution flows, and modules. A unit test alone cannot promote a product path
that still bypasses the new registry.

## Permitted classification and gate change

HCM-1.1 may, only after its complete proof and CLEAN final review:

- change the `Artifact kind/schema registry` crosswalk row from `TargetOnly` to
  `BoundaryLanded` for the exact `handbook-engine` owner-library boundary;
- record the exact kind/schema resolution and structural-validation subset of
  `PG-KIND-01` while leaving that lifecycle/Projection-dependent gate open; and
- record the registration and structural-validation subset of `PG-KIND-02`
  for the exact capability-free repository-defined custom-kind fixture while
  leaving that gate open until a later intake-owning slice exercises supplied
  intake coverage.

It may not promote `Canonical artifact identities`, `Canonical layout`, setup,
doctor, flow, shipped defaults, canonical YAML, intake, profile, transport,
publication, or consumer adoption. `RealPathAdopted`,
`ContractCorrectAndProven`, and `PG-PUBLISH-01` remain unavailable.

## Exit gate

HCM-1.1 completes only when:

- all public types, exact source paths, assets, and dependency features match
  this packet or an already-reviewed packet correction;
- the positive, negative, security, package, regression, and full workspace
  proof wall passes;
- no current product path or sibling slice was changed;
- the crosswalk/proof ledger promotes at most the permitted boundary/gates and
  names the exact evidence;
- a fresh isolated built-in `default` final reviewer returns CLEAN over the
  exact full subject and proof wall;
- accepted findings are remediated, all proof reruns, and a different fresh
  reviewer checks the remediation fingerprint;
- repository-required staged change detection and byte replay pass;
- the reviewed implementation is committed first; and
- one parent-owned v1.2 handoff plus rebuilt ledger validates and is committed
  separately without starting HCM-1.2.

## Stop conditions

Stop with an exact parent-owned top-level handoff rather than improvising when:

- HCM-0.7 dependency/packet identity does not validate;
- a required change reaches outside the allowed files or into HCM-1.2+;
- the pinned dependencies cannot implement Draft 2020-12 with all ambient
  resolver/executable behavior disabled;
- canonical `02`/`05` authority proves contradictory for this capability-free
  registry boundary and a bounded docs repair cannot resolve it;
- exact first-party content-schema, capability, renderer, lifecycle, intake,
  profile, or instance decisions become necessary;
- required workspace/package/platform proof is unavailable;
- a required public dependency would need publication/downstream proof inside
  this slice; or
- mandatory fresh built-in review is unavailable.

## Explicit non-goals

- replacing, renaming, or deleting `CanonicalArtifactKind` or its consumers;
- changing fixed canonical layout, artifact loading, manifest/freshness,
  baseline validation, setup, doctor, flow, compiler, CLI, or pipeline behavior;
- publishing the six HCM-0.6 first-party kinds, their content schemas, the
  shipped profile, its three instances, condition definition, intake
  definitions, renderers, lifecycle policies, or canonical YAML contents;
- defining or executing a generic Projection;
- dynamically discovering schema/kind sources or generating CLI commands;
- remote registries, network fetch, executable/custom schema hooks, ambient
  interpreters, or package-manager installation;
- changing crate versions, publishing to crates.io, or claiming downstream
  consumption;
- HCM-1.2, HCM-1.3, HCM-1.4, HCM-0.9, or unrelated cleanup; and
- claiming semantic, intake, constitutional, contract, evidence, verdict, or
  gate authority from structural schema success.
