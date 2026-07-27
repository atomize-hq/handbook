# HCM-2.4 P1A definition-support implementation proof

Status: **implemented and closure-review CLEAN**

Different-fresh closure dispatch:
`20260726T205749Z--HCM-2-4--p1a-complete-subject-closure-review`;
subject
`sha256:1bdb1bd3a912f590ab9af31586afbca4acb538235e26146ddd7a17efecdd8ad9`.
The closure resolved the discovery P2 findings with no unresolved P1/P2.

## Immutable input replay

- Dispatch:
  `20260726T172336Z--HCM-2-4--p1a-definition-support-implementation`
- Subject:
  `sha256:5354e3a5458438ecdf7c0024cfb946abe473c5408f276eff36588086ee040a24`
- All 27 dispatch manifest entries replayed byte-for-byte before edits.
- The only pre-existing worktree changes were parent-owned P0/control-pack
  records. No runtime, test, or definition path overlapped P1A.

## Fresh impact analysis

GitNexus MCP tools were unavailable in this agent context, so the indexed CLI
fallback was used before production edits.

| Exact symbol | Upstream result | Disposition |
| --- | --- | --- |
| `AuthoredArtifactKindDefinition.validate_later_owned_dependencies#1` | LOW; 1 impacted, 1 direct, 0 processes, 1 module | Exact five kind/renderer pairs only |
| `ArtifactInstanceRegistry.resolve#3` | CRITICAL; 149 impacted, 19 direct, 8 processes, 10 modules | Dispatch-accepted radius; exact five frozen descriptor rows only |
| `profile_builtins::definition` | LOW; 0 graph edges, with dynamic-source caveat | Exact 16 new package mappings only |

No public signature, module, process, Cargo, package, dependency, command,
Projection, Resolution, or default shipped-request change was made.

The typed-closure and real-path admission correction used the same indexed CLI
fallback before each production-symbol edit:

| Exact symbol | Upstream result | Disposition |
| --- | --- | --- |
| `ArtifactIntakeDefinitionV1::parse` | LOW; 11 impacted, 3 direct, 1 process, 2 modules | Exact five P1A intake dependency closures only |
| `AuthoredArtifactKindDefinition::validate` | LOW; 0 indexed edges, with incomplete type-edge caveat | Exact five successor kind closures only |
| `validate_authored_profile_fingerprints` | CRITICAL; 130 impacted, 8 processes, 10 modules | Exact shipped-root `1.2` branch only |
| `load_repository_intakes` | CRITICAL; 44 impacted, 7 processes, 11 modules | Charter-only compatibility selection; exact five P1A intakes use semantic admission |
| `ResolvedSchema::collect_coverage_leaf_shapes` | LOW; 1 direct, 0 processes, 1 module | Only type-absent string-valued `const` leaves classify as String |

The two CRITICAL radii were operator-accepted only for the branch-local
correction. The final classifier is the fifth and last P1A production symbol.
No sixth symbol or wider module/process contract was used.

## RED proof

The first focused test was added before production changes:

```text
cargo test -p handbook-engine --test hcm_2_4_definition_support -- --nocapture
```

It exited 1 because
`artifact-kinds/handbook.artifact-kind.project-context/1.1.0.yaml` did not exist
(`os error 2`, test line 30). This established the missing 16-definition
publication set.

Subsequent focused RED proofs established each remaining boundary:

- provisional intake/renderer fingerprints differed from normalized content;
- provisional kind fingerprints differed from their schema closures;
- successor kind loading failed `UnsupportedDependency` because the existing
  guard refused every non-Project-Authority renderer;
- exact non-Charter descriptors failed at
  `renderer_definition_refs` because the existing guard refused later-owned
  dependencies;
- explicit built-in 1.2 selection failed `MissingSource`;
- after mapping, 1.2 selection failed `FingerprintMismatch` until the complete
  typed dependency closure was frozen.

The resumed real-path proof then established two more exact failures:

- `hcm_2_4_definition_runtime` was 5/6 because
  `ArtifactRepositoryV1::open` reached semantic intake admission and refused
  the released schemas' type-absent string-valued `schema_id` and
  `schema_version` `const` leaves as indeterminate; and
- the focused `coverage_leaf_shapes_` unit target was 1/2 before the fifth
  symbol changed: the positive string-`const` case failed at the existing
  unsupported/indeterminate-type refusal while all 11 negative cases passed.

## Implemented closure

- Published five exact artifact-kind `1.1.0` successors.
- Published five strict intake `1.0.0` definitions.
- Published five fixed renderer `1.0.0` definitions.
- Published shipped-root profile `1.2.0` with exactly three instances.
- Preserved the released Project Authority row field-for-field from profile
  `1.1.0`.
- Froze profile `1.2.0` at
  `sha256:63cd999c95efc3fe65ae3514c2915b5d1457290211cf6da7b57b2cd75bafaf83`.
- Added the exact 16 package-owned source mappings.
- Admitted only the five exact successor kind/renderer pairs.
- Admitted only the five exact frozen non-Charter descriptor rows, including
  exact ID, kind, role, label, path, requiredness, intake, singleton renderer,
  and empty later-owned fields.
- Retained fail-closed behavior for missing, extra, mismatched, lifecycle,
  Projection, overlay, trigger, capability, extension, and dependency
  cardinality mutations.
- Added immutable definition and renderer vectors.
- Computed the five intake fingerprints from their exact authored definition
  plus typed kind/schema dependency fingerprints using the exact package-owned
  definition bytes.
- Kept only the released Charter intake on the identity-only compatibility
  path; the five P1A intakes retain modes and coverage through semantic parse
  and registry validation.
- Refused `repository_path` rebinding for every package-owned intake exact ref
  while retaining repository sources for custom intake refs.
- Classified a coverage leaf as String only when its `type` member is absent
  and its `const` value is a JSON string.
- Preserved refusal for non-string `const` values, type-absent string `enum`,
  default/examples/annotation-only leaves, ambiguous composites, and string
  `const` paired with an unsupported explicit type.
- Left all released schema bytes unchanged.

## Verification

Focused target:

```text
cargo test -p handbook-engine --test hcm_2_4_definition_support -- --nocapture
```

Result: exit 0; 12 passed, 0 failed.

Focused classifier and real-path admission targets:

```text
cargo test -p handbook-engine coverage_leaf_shapes_ --lib -- --nocapture
cargo test -p handbook-engine \
  --test hcm_2_4_definition_runtime \
  --test hcm_2_4_definition_support \
  -- --nocapture
```

Result: exit 0; 2 classifier tests, 6 runtime tests, and 12 definition-support
tests passed. The runtime target includes the actual
`ArtifactRepositoryV1::open` path plus stale subordinate fingerprints, wrong
kind/schema binding, missing coverage, unsupported mode, and invalid coverage
target refusal.

The focused proof covers:

- duplicate-safe publication and absence of placeholder fingerprints;
- exact intake/renderer normalized-content fingerprints;
- exhaustive schema-property coverage, required user declaration, and
  unknown/contradiction blocking;
- exact kind schema-closure fingerprints;
- positive and negative kind dependency admission;
- all five full descriptor rows and field-level near misses;
- schema-valid renderer inputs and exact full-byte/hash goldens;
- actual replay of the existing Project Context renderer;
- full typed profile dependency closure;
- exact equality of the `1.1` and `1.2` Project Authority rows;
- explicit 1.2 built-in resolution while the live shipped request remains
  `1.1`.

Complete named P1A target wall:

```text
cargo test -p handbook-engine \
  --lib \
  --test artifact_instances \
  --test artifact_kind_registry \
  --test profile_artifact_schemas \
  --test profile_context_schemas \
  --test profile_selection \
  --test profile_work_decision_schemas \
  --test profile_risk_schema \
  --test hcm_1_2_selected_kinds \
  --test hcm_1_2_unselected_kinds \
  --test hcm_1_4_profile_decisions \
  --test hcm_1_4_profile_inspection \
  --test hcm_2_2_definition_profile \
  --test hcm_2_3_registration_kernel \
  --test hcm_2_4_definition_support \
  --test hcm_2_4_definition_runtime
```

Result after the complete-subject discovery findings were remediated: exit 0;
244 passed, 0 failed. This includes 7/7 `hcm_2_4_definition_runtime`, 13/13
`hcm_2_4_definition_support`, and the unchanged 16/16
`hcm_2_3_registration_kernel` boundary. The
`hcm_1_4_profile_inspection` target contains zero runnable tests on the Windows
target and exited 0.

Formatting and hygiene:

```text
cargo fmt --all -- --check
git diff --check
```

Result: both exit 0.

GitNexus change detection:

```text
npx gitnexus detect-changes --scope unstaged --repo handbook --limit 200
```

Result after index-only refresh: exit 0; 10 tracked files, 7 indexed symbols,
0 affected processes, LOW risk. The tool does not include the untracked
immutable definitions, vectors, tests, proof, or dispatches in its changed-file
count; those paths are bound separately by the complete review manifest. A
prior ordinary analyzer refresh rewrote its generated instruction blocks; the
eight analyzer-generated side effects were restored to the validated checkpoint
and the index was refreshed again with `--index-only` before this result.

## Proof boundary

P1A proves static package publication, normalized fingerprints, exhaustive
authored coverage, fixed renderer contracts/goldens, exact frozen
kind/descriptor admission, and actual semantic admission of all five new
package intakes through `ArtifactRepositoryV1::open`. Project Context and
Environment Context retain their exact three supported modes and authored
coverage; the released Charter intake remains compatibility-only.

P1A does not select shipped-root `1.2` as the live default, expand the root
descriptor set beyond three, execute P2-P5 vertical behavior, add generic
Projection/Resolution, or change any released schema, public API, Cargo,
dependency, package, command, or sibling-slice surface.

## Closeout

P1A is included in reviewed checkpoint commit
`5cf41d2f64d68cb7f78abb5eccab9acf307089d3`. Later packets do not widen its
exact const-string predicate, change a released schema, or add a sixth
production symbol.
