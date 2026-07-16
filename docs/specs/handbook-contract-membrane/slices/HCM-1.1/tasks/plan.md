# Implementation Plan: HCM-1.1 Artifact-Kind and Local Schema Registry

## Overview

Build the first additive `handbook-engine` registry boundary without replacing
the current fixed artifact product path. The implementation proceeds from exact
identity primitives to package-owned stable roles, safe local schema closure,
kind meta-validation, and one repository-defined custom-kind proof. Each task is
TDD-first, leaves the workspace green, and is independently reviewable.

Generic orchestration, review/remediation, true-stop handoff, ledger, and
two-commit closeout mechanics remain owned by canonical `07`/`08`.

## Authority and sequencing decisions

- Consume the exact completed HCM-0.7 handoff and this packet before edits.
- Keep all schema/kind behavior in `handbook-engine`.
- Reuse the existing trusted repo-relative/no-follow filesystem boundary rather
  than building a second path resolver.
- Add no behavior to `CanonicalArtifactKind` or its consumers.
- Load only explicitly selected sources; never scan directories or choose a
  source-order winner.
- Decode every definition and nested record as a closed type; duplicate,
  unknown, wrong-record, or non-empty extension fields refuse before
  fingerprinting.
- Disable every `jsonschema` resolver feature, refuse authored identifiers,
  anchors, and dynamic references under the frozen v1 policy, and register the
  complete prevalidated local resource closure in memory under exact internal
  bases shared by prewalk and validator.
- Publish only package-owned stable-role registry 1.0.0/1.1.0 assets in this
  slice; publish no first-party kind, content schema, profile, or instance.
- Expose no opaque exact-dependency producer and refuse every non-empty
  semantic-capability or later-owned behavioral field until its owning slice
  supplies type-specific source, normalization, compatibility, and proof.
- Use a capability-free custom kind for the registration and
  structural-validation subset of PG-KIND-02. Optional dependency fields are
  proved as empty/null; they do not imply later definitions exist.
- Promote at most the registry seam to `BoundaryLanded`; record only bounded
  structural subsets of PG-KIND-01 and PG-KIND-02, leaving both gates open for
  later lifecycle/Projection and supplied-intake proof.

## Dependency graph

```text
Task 1 exact identity/fingerprint primitives
  -> Task 2 stable-role package assets and explicit source loading
    -> Task 3 safe Draft 2020-12 schema registry/closure
      -> Task 4 capability-free artifact-kind meta-validation
        -> Task 5 repository-defined custom-kind proof
          -> Task 6 full proof, clean review, promotion, and closeout
```

All tasks are sequential because they share the public engine boundary and
fingerprint rules. Review agents are always read-only.

## Task 1: Exact definition identity and fingerprint primitives

**Description:** Add the direct dependencies and typed exact-ref/fingerprint,
SemVer, RFC 8785 normalization, SHA-256, duplicate-key, and bounded-source
primitives used by all later tasks. Start with failing focused tests.

**Acceptance criteria:**

- [ ] Exact refs derive only from namespaced identity plus full SemVer.
- [ ] Definition identities enforce the exact lowercase-ASCII dot/segment and
  byte-bound grammar; SemVer lexemes round-trip byte-identically through
  `semver::Version`; no trimming, case/Unicode/percent normalization, or
  delimiter ambiguity exists.
- [ ] Fingerprints accept only lowercase `sha256:<64-hex>` and are recomputed.
- [ ] Type-specific normalization sorts only contract-unordered arrays and
  produces deterministic RFC 8785 bytes.
- [ ] Duplicate YAML/JSON keys and per-document/aggregate limit breaches refuse
  with typed errors.
- [ ] Closed typed decoding rejects unknown top-level/nested and wrong-record
  fields before any semantic preimage is constructed.
- [ ] No ambient path, timestamp, or presentation-only value enters a semantic
  fingerprint.

**Verification:** RED proof for missing/incorrect primitives; focused identity
character/segment/case/Unicode/delimiter/boundary and SemVer canonical-roundtrip
tests; RFC 8785 reference examples; repeated/source-order fingerprints; invalid
ref/digest/duplicate/limit negatives; format check.

**Dependencies:** completed HCM-0.7 entry validation.

**Files likely touched:**

- `crates/engine/Cargo.toml`
- `Cargo.lock`
- `crates/engine/src/definition_identity.rs`
- `crates/engine/src/lib.rs`
- `crates/engine/tests/definition_identity.rs`

**Estimated scope:** M.

## Task 2: Package-owned stable roles and safe explicit source loading

**Description:** Add the two immutable core role-registry assets, typed parsing,
exact selection, fingerprint replay, and the smallest reusable exposure of the
existing engine no-follow source boundary.

**Acceptance criteria:**

- [ ] Core 1.0.0 and additive 1.1.0 bytes reproduce their frozen fingerprints.
- [ ] Role order, identity, display label, category, and version are validated;
  `environment_context` stays distinct.
- [ ] Source paths are explicit, normalized, bounded, regular, and no-follow.
- [ ] Unknown roles, duplicate roles, invalid categories, symlinks, directories,
  missing files, and exact-ref substitution refuse.
- [ ] Unknown top-level/nested registry or role fields refuse before
  fingerprinting.
- [ ] Both assets are included in `handbook-engine` package contents.

**Verification:** failing fingerprint/load tests first; focused positive and
negative tests; package list inspection; impact analysis before any visibility
change in `canonical_repo_support.rs`.

**Dependencies:** Task 1.

**Files likely touched:**

- `crates/engine/src/stable_role_registry.rs`
- `crates/engine/src/canonical_repo_support.rs`
- `crates/engine/src/lib.rs`
- `crates/engine/tests/stable_role_registry.rs`
- `crates/engine/definitions/stable-roles/handbook.roles.core/{1.0.0,1.1.0}.yaml`

**Estimated scope:** M.

## Task 3: Safe local Draft 2020-12 schema registry

**Description:** Implement `SchemaRegistryEntry`, explicit allowed roots, local
closure prewalk, deterministic fingerprints, in-memory resource registry, meta-
schema validation, and structural validators with every external resolver
feature disabled.

**Acceptance criteria:**

- [ ] Exact schema tuple/ref and entry/document/closure fingerprints resolve
  deterministically.
- [ ] Schema-entry top-level/nested unknown fields and non-empty extensions
  refuse before fingerprinting.
- [ ] Root and every transitive local ref use the same allowed-root/no-follow/
  regular-file/meta-schema/limit policy.
- [ ] Root/nested authored IDs, anchors, dynamic/recursive-reference keywords,
  and plain-name fragments refuse under the frozen HCM-1.1 v1 resource policy.
- [ ] Every document root declares the exact Draft 2020-12 `$schema`; missing,
  conflicting, or nested declarations and every schema-position keyword
  outside the frozen v1 allowlist refuse before fingerprinting.
- [ ] Schema-aware traversal distinguishes keyword maps/subschemas from object
  instance data carried by `const`, `enum`, `default`, and `examples`.
- [ ] Prewalk, internal resource bases, closure members, validator targets, and
  reported schema locations agree exactly for every admitted JSON Pointer ref.
- [ ] Draft 2020-12 validators use only the prevalidated in-memory closure and
  linear-time regex engine.
- [ ] Duplicate/conflicting tuples fail in both source orders.
- [ ] All remote/ambient/traversal/query/backslash/encoded/cyclic/missing/
  over-bound cases refuse with typed evidence.
- [ ] Identifier rebasing, duplicate resource identities/aliases, invalid
  pointer fragments, and prewalk/validator target mismatch refuse with typed
  evidence.
- [ ] Structural validation produces deterministic instance/schema locations.

**Verification:** failing schema-entry/closure tests first; positive closure and
meta-validation; absent/matching/conflicting/nested dialect, keyword-typo,
unknown-annotation, and schema-position-versus-instance-data cases; exhaustive
local-ref negatives; `cargo tree` confirms resolver features and HTTP/async/TLS
clients are absent.

**Dependencies:** Task 2.

**Files likely touched:**

- `crates/engine/src/schema_registry.rs`
- `crates/engine/src/lib.rs`
- `crates/engine/tests/schema_registry.rs`
- dynamically created temporary negative fixtures

**Estimated scope:** M.

## Task 4: Capability-free artifact-kind meta-validation

**Description:** Implement the typed kind record for the exact HCM-1.1
capability-free boundary, stable-role/schema resolution, direct definition
closure, and fail-closed refusal of every later-owned behavioral dependency.

**Acceptance criteria:**

- [ ] Kind refs derive from IDs/versions and only exact compatibility is
  accepted.
- [ ] Stable-role registry and canonical schema refs resolve exact fingerprints.
- [ ] Role support validates against the exact selected stable-role registry.
- [ ] Kind records cannot carry instance path/label/requiredness/setup state or
  intake definitions.
- [ ] Kind top-level/nested unknown fields, wrong-record fields, and non-empty
  extensions refuse before fingerprinting.
- [ ] Semantic capabilities, required capabilities, semantic validators,
  renderers, lifecycle policy, review triggers, Projection refs, and opaque
  dependency producers are empty/null or refuse.
- [ ] Kind fingerprints include only the normalized capability-free record plus
  exact stable-role and schema-entry/closure fingerprints.
- [ ] Forged well-formed digests, changed bytes, wrong dependency kinds,
  incompatible pointer-shape claims, and source-order substitutions cannot enter
  the closure because their non-empty inputs refuse before fingerprinting.

**Verification:** failing kind tests first; exact capability-free positive
records; unknown/mismatched/missing/duplicate role/schema negatives; every
non-empty later-owned dependency and forged-digest negative; format and engine
tests.

**Dependencies:** Task 3.

**Files likely touched:**

- `crates/engine/src/artifact_kind_registry.rs`
- `crates/engine/src/lib.rs`
- `crates/engine/tests/artifact_kind_registry.rs`
- dynamically created temporary negative fixtures

**Estimated scope:** M.

## Task 5: Repository-defined custom-kind end-to-end proof

**Description:** Add a primary capability-free custom kind plus one distinct
proof-only companion, two schema entries, two schema roots, a real local ref
closure, valid primary YAML data, and generated invalid data. Load and validate
them only through public owner APIs.

**Acceptance criteria:**

- [ ] The fixture's two distinct entry refs, two distinct kind refs, and all
  per-definition fingerprints replay deterministically.
- [ ] Both accepted source permutations produce identical schema/kind registry
  fingerprints, exact lookup sets, per-entry closures, and validation behavior.
- [ ] A local `$ref` is exercised, not merely present in an unused file.
- [ ] Valid YAML parsed into the JSON data model passes; invalid data reports
  stable structural locations.
- [ ] The custom kind requires no Rust enum variant, CLI command, product-path
  dispatch, executable hook, remote fetch, renderer, intake, or Projection.
- [ ] Existing engine/flow/compiler/CLI/pipeline tests remain unchanged and pass.

**Verification:** failing end-to-end test first; repeated loads with both
permutations of two valid schema entries and two valid kinds; exact set and
registry-fingerprint equality; valid/invalid validation; grep/assert no
`CanonicalArtifactKind` addition and no changed sibling product path.

**Dependencies:** Task 4.

**Files likely touched:**

- `crates/engine/tests/hcm_1_1_custom_kind.rs`
- `crates/engine/tests/fixtures/hcm_1_1_custom_kind/schema-entry.yaml`
- `crates/engine/tests/fixtures/hcm_1_1_custom_kind/root.schema.json`
- `crates/engine/tests/fixtures/hcm_1_1_custom_kind/fields.schema.json`
- `crates/engine/tests/fixtures/hcm_1_1_custom_kind/kind.yaml`
- `crates/engine/tests/fixtures/hcm_1_1_custom_kind/companion-schema-entry.yaml`
- `crates/engine/tests/fixtures/hcm_1_1_custom_kind/companion.schema.json`
- `crates/engine/tests/fixtures/hcm_1_1_custom_kind/companion-kind.yaml`
- `crates/engine/tests/fixtures/hcm_1_1_custom_kind/valid.yaml`

**Estimated scope:** M; the fixture files are one proof corpus.

## Task 6: Full proof, independent review, evidence promotion, and closeout

**Description:** Run the complete SPEC proof wall, update only the exact
crosswalk/gate truth earned, obtain fresh review over the complete implementation
and proof, remediate valid findings with different-fresh re-review, commit the
reviewed subject, then perform the separate mechanical v1.2 closeout.

**Acceptance criteria:**

- [ ] Focused, negative, package, engine, workspace, archive, handoff, formatting,
  lint, dependency-feature, and scope proof passes.
- [ ] The Artifact kind/schema registry row moves at most to `BoundaryLanded`;
  bounded PG-KIND-01 and PG-KIND-02 structural evidence is recorded without
  closing either gate, and all sibling gates remain open.
- [ ] Final review binds every implementation, asset, fixture, control-pack, and
  proof byte and returns CLEAN.
- [ ] Accepted findings receive typed remediation, full proof rerun, and a
  different fresh reviewer.
- [ ] Staged GitNexus change detection and byte replay match the clean subject.
- [ ] Primary implementation and mechanical handoff/ledger closeout are separate
  commits, and HCM-1.2 is not started.

**Verification:** every command and negative case in SPEC; exact final manifest
replay; all canonical handoff validator modes; scoped commit inspection and
clean attributable worktree.

**Dependencies:** Task 5.

**Files likely touched:**

- `docs/specs/handbook-contract-membrane/03-seam-crosswalk.md`
- `docs/specs/handbook-contract-membrane/06-proof-and-regression-ledger.md`
- HCM-1.1 proof/review evidence
- one parent handoff record and rebuilt ledger in the second commit

**Estimated scope:** M.

## Risks and mitigations

| Risk | Impact | Mitigation |
|---|---|---|
| Registry work becomes fixed-enum refactor | High | additive modules only; hard stop on current product-path files |
| JSON Schema library performs ambient I/O | Critical | default features off, prewalk/refusal, explicit in-memory resources, dependency-tree assertion |
| Repository schema executes code or expensive regex | High | no custom hooks/formats; linear regex; bounded bytes/docs/depth |
| Fingerprints depend on source order or machine path | High | type-specific normalization, JCS, typed sorted closure, replay tests |
| Kind and instance authority collapse | High | no path/label/requiredness fields; no profile/instance types in this slice |
| Optional future definitions are fabricated | High | no opaque dependency API; all non-empty later-owned fields refuse; capability-free proof fixture |
| Unit proof is overclaimed as product adoption | High | classification ceiling `BoundaryLanded`; fixed product paths remain unchanged |
| Public local API is mistaken for published | High | no crate version/publish work; PG-PUBLISH-01 stays open |

## Checkpoints

After each task:

- [ ] focused RED evidence was observed before GREEN;
- [ ] focused tests and format check pass;
- [ ] affected engine tests pass;
- [ ] diff contains only the task's logical increment; and
- [ ] staged change detection is clean before any incremental commit.

Final checkpoint uses the complete SPEC proof wall and independent review rather
than aggregating task-level confidence.
