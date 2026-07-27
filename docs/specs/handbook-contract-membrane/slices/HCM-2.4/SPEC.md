# HCM-2.4 — Remaining shipped artifact-family conversion

Status: partially implemented. P0/P1A/P1B/P1C and P3/P3B are review-clean and
committed; P2 and P4/P5 are authority-blocked; P6 and Phase 2 exit are not
earned.

Planning-amendment baseline: `fa31f65fddd678b33ea89d6b7e41201240254986`

Selected resume lineage:
`20260726T192534Z--HCM-2-4--orchestration--p1a-selector-expansion-required`

Amendment evidence:
[proof/20260726T122626Z--planning-charter-compatibility-amendment.md](proof/20260726T122626Z--planning-charter-compatibility-amendment.md)
and
[proof/20260726T124918Z--planning-amendment-review-1-remediation.md](proof/20260726T124918Z--planning-amendment-review-1-remediation.md)
and
[proof/20260726T125938Z--planning-amendment-supplemental-causal-remediation-1.md](proof/20260726T125938Z--planning-amendment-supplemental-causal-remediation-1.md)

## Objective and completion boundary

HCM-2.4 closes Phase 2 by making the shipped artifact-kind catalog truthful end to
end:

- each shipped kind has a schema-backed first-party intake and one fixed,
  deterministic human-review renderer;
- every selected artifact has exactly one editable canonical YAML source;
- every pre-Phase-3 Markdown output is a derived human-review view only and is
  never an input to Resolution;
- the two temporary flow bridges and all fixed-family/path selection logic are
  removed after their replacements are proven; and
- the exact Phase 2 exit proofs are replayed without weakening the completed
  Charter, Project Context, or HCM-2.3 registry-brief authority.

This document freezes implementation packets and proof obligations. Runtime
work was authorized only through the exact selectors recorded by the
parent-owned orchestration. The completed partial packets do not authorize P2,
P4/P5 remediation, P6, Phase 2 exit, or another slice.

## Authority and preserved decisions

The implementation must preserve these decisions byte-for-byte or
behavior-for-behavior unless an explicitly versioned HCM-2.4 successor is named
below:

1. Project Authority remains the Charter selected at
   `.handbook/project/charter.yaml`, using
   `handbook.artifact-kind.project-authority@1.1.0`,
   `handbook.intake.charter@1.0.0`, and
   `handbook.renderer.charter-review-markdown@1.0.0`. Candidate, result,
   promotion, promotion-intent, approval, lineage, lifecycle, transaction,
   fingerprint, and committed-authority boundaries from HCM-2.2 are not
   reopened. Shipped-root 1.2 may enter that boundary only through the exact
   versioned compatibility membrane defined below; Charter records retain the
   released 1.1 profile ref and fingerprint.
2. Project Context remains selected at `.handbook/project/context.yaml`; the
   HCM-2.1 canonical bytes and renderer behavior remain regression anchors.
3. `handbook.profile.shipped-root@1.1.0` remains immutable. Its successor may
   still select exactly three root instances—Project Authority, Project Context,
   and conditional Environment Context. It must not add Work Specification,
   Decision Record, or Risk Record root instances.
4. The HCM-2.3 `registry-brief` custom kind, its 111-path subject fingerprint,
   generic operation authority, real-binary proof, and non-enumerated command
   surface remain unchanged. It is a preservation row, not a seventh shipped
   first-party kind.
5. Existing released schema, kind, profile, intake, and renderer definition
   bytes are immutable. HCM-2.4 uses additive exact-version successors where a
   definition must gain references.

## Versioned Charter compatibility membrane

The HCM-2.4 profile successor must not change the Charter authority selected by
HCM-2.2. The only authorized compatibility boundary is the existing
`CharterDefinitionRegistry::validate_selected_decisions` function, with its
signature and public API unchanged.

That boundary admits exactly two ref/fingerprint tuples:

1. `handbook.profile.shipped-root@1.1.0` with
   `sha256:6a7b41befa77b999b9ee20f513636051726a8401a81bf2f369501e8f3dd4fa74`;
   and
2. `handbook.profile.shipped-root@1.2.0` with the one literal authored profile
   fingerprint produced by P1A and frozen in its immutable definition vector
   before P1B begins.

There is no placeholder at P1B entry. No semver range, “newer than 1.1,” string
prefix, generic compatibility promise, repository-selected substitute, second
profile resolution, fallback, migration, alias, or dual-read behavior is
allowed.

For either tuple the resolved `project_authority` descriptor must equal the
HCM-2.2 closure exactly:

- closed descriptor schema `handbook.artifact-instance-descriptor@1.0`;
- id `project_authority`;
- kind `handbook.artifact-kind.project-authority@1.1.0`;
- role `constitutional_authority`;
- capabilities exactly [`constitutional_root`];
- label `Charter`;
- path `.handbook/project/charter.yaml`;
- requiredness `always` with null condition;
- no dependencies;
- lifecycle
  `handbook.lifecycle.constitutional-review-lock@1.0.0`;
- intake `handbook.intake.charter@1.0.0`;
- renderers exactly
  [`handbook.renderer.charter-review-markdown@1.0.0`];
- no Projections, validation overlays, or extensions; and
- exact subordinate definition presence in the released Charter registry.

The closed artifact-instance loader remains authoritative for descriptor schema
id/version and unknown-field refusal; the compatibility boundary compares every
resolved field. Any tuple, fingerprint, descriptor, subordinate definition, or
cardinality mismatch is rejected.

Generic selected decisions remain shipped-root 1.2 for HCM-2.4 consumers.
Charter intake/candidate/result/promotion/currentness/transaction producers
continue to emit and compare the released HCM-2.2 1.1 ref/fingerprint pair.
Candidate/result separation, promotion intent, approval, lineage, lifecycle,
recovery, replay, committed authority, and every HCM-2.2 negative case remain
unchanged. `validate_candidate_v13` and `validate_promotion_intent_v12` are
explicit no-edit anchors.

## Exact shipped artifact-family inventory

| Family | Existing exact kind/schema | Selected production instance | Current authority problem | HCM-2.4 target |
| --- | --- | --- | --- | --- |
| Project Authority / Charter | `handbook.artifact-kind.project-authority@1.1.0` / `handbook.schemas.artifacts.project-authority@1.1.0` | Always: `.handbook/project/charter.yaml` | None | Preservation-only; consume through the common selected-artifact path without changing Charter authority |
| Project Context | `handbook.artifact-kind.project-context@1.0.0` / `handbook.schemas.artifacts.project-context@1.0.0` | Always: `.handbook/project/context.yaml` | Canonical YAML exists, but the published kind/profile has no intake or renderer reference | Add `handbook.intake.project-context@1.0.0`, `handbook.renderer.project-context-review-markdown@1.0.0`, and an additive kind successor; preserve HCM-2.1 bytes and rendering |
| Environment Context | `handbook.artifact-kind.environment-context@1.0.0` / `handbook.schemas.artifacts.environment-context@1.0.0` | Conditional: `.handbook/project/environment.yaml` | The authoring and flow path still treats Environment Inventory Markdown as editable authority | Make selected Environment Context YAML the only editable source, publish intake/renderer support, and delete Markdown-authority helpers after equivalence proof |
| Work Specification | `handbook.artifact-kind.work-specification@1.0.0` / `handbook.schemas.artifacts.work-specification@1.0.0` | No shipped-root singleton | Stage 10 captures `artifacts/feature_spec/FEATURE_SPEC.md` as authority; the kind has no intake or renderer | Capture structured Work Specification YAML at the fixed path `artifacts/work-specification/work-specification.yaml`; retain `artifacts/feature_spec/FEATURE_SPEC.md` only as a disposable deterministic view; prove a repository-profile-selected real path |
| Decision Record | `handbook.artifact-kind.decision-record@1.0.0` / `handbook.schemas.artifacts.decision-record@1.0.0` | None by design | Kind is schema-only and has no first-party intake/renderer | Publish fixed intake/renderer support and prove it through an explicit repository-profile descriptor; create no root instance or command |
| Risk Record | `handbook.artifact-kind.risk-record@1.0.0` / `handbook.schemas.artifacts.risk-record@1.0.0` | None by design | Kind is schema-only and has no first-party intake/renderer | Publish fixed intake/renderer support and prove it through an explicit repository-profile descriptor; create no root instance or command |
| HCM-2.3 registry brief | Custom kind and repository profile frozen by HCM-2.3 | Repository-profile selected fixture only | None | Preservation-only regression anchor; no first-party-kind classification and no new public surface |

There are no other shipped first-party artifact kinds in the Phase 2 catalog.
Templates, pipeline prompts, Markdown products, provenance records, handoff
manifests, candidate/result/control records, and rendered views are support
artifacts, not additional editable artifact families.

## Canonical-source and view classification

| Family | Only editable canonical truth | Intake convergence | Human-review view | Persistence and influence rule |
| --- | --- | --- | --- | --- |
| Project Authority | `.handbook/project/charter.yaml` | Existing Charter intake and HCM-2.2 authority | Existing Charter review renderer | View is derived; never competes with committed Charter authority |
| Project Context | `.handbook/project/context.yaml` | New published intake covers the exact 1.0 schema and existing author path | New definition binds the existing deterministic renderer | View is generated from retained canonical bytes; no Markdown input or fallback |
| Environment Context | `.handbook/project/environment.yaml` when the selected condition is true | CLI structured input, guided/express/agent-assisted intake, setup, doctor, and flow validate the same selected schema | `handbook.renderer.environment-context-review-markdown@1.0.0` | No persisted Markdown authority. The legacy `.handbook/environment_inventory/ENVIRONMENT_INVENTORY.md` is removed from reads and writes; any displayed Markdown is regenerated |
| Work Specification | `artifacts/work-specification/work-specification.yaml` for the fixed Stage 10/repository-profile descriptor | Stage 10 capture and generic intake decode to the same work-specification schema | `handbook.renderer.work-specification-review-markdown@1.0.0`; existing `artifacts/feature_spec/FEATURE_SPEC.md` presentation may remain only as regenerated output | The Markdown file is rejected as canonical input, cannot affect fingerprints or handoff decisions, and is replaceable from retained YAML bytes |
| Decision Record | Fixture descriptor path `.handbook/records/decision.yaml`; no production default | All supported intake modes validate the decision-record schema | `handbook.renderer.decision-record-review-markdown@1.0.0` | Renderer output is on demand; no default instance, filename inference, persistent mirror, or root path |
| Risk Record | Fixture descriptor path `.handbook/records/risk.yaml`; no production default | All supported intake modes validate the risk-record schema | `handbook.renderer.risk-record-review-markdown@1.0.0` | Renderer output is on demand; no default instance, filename inference, persistent mirror, or root path |
| Registry brief | Frozen HCM-2.3 descriptor-selected YAML | Frozen HCM-2.3 generic mutation/validation path | Frozen HCM-2.3 behavior | No change |

“Renderer” in this slice means a fixed, deterministic, non-Resolution
first-party human-review transformation. It is not capitalized Projection. Each
renderer has `resolution_input: null`, an exact input schema ref, a closed
determinism profile, full-byte goldens, and no clock, environment, network, or
repository-discovery input.

The remaining pre-Phase-3 Markdown output inventory is classified as follows:

| Output | Classification |
| --- | --- |
| `.handbook/environment_inventory/ENVIRONMENT_INVENTORY.md` | Superseded authority; delete from normal reads/writes and do not retain as compatibility input |
| `artifacts/foundation/ENVIRONMENT_INVENTORY.md` | Overlapping Environment Context output; delete or regenerate only from selected Environment Context YAML, with zero canonical influence |
| `.handbook/feature_spec/FEATURE_SPEC.md` | Superseded fixed flow source; delete from canonical selection |
| `artifacts/feature_spec/FEATURE_SPEC.md` | Fixed deterministic Work Specification human-review view only; disposable and never canonical input |
| `artifacts/foundation/FOUNDATION_STRATEGY.md` | Non-HCM pipeline/operator product; no canonical-family authority and no input to HCM mutation |
| `artifacts/foundation/TECH_ARCH_BRIEF.md` | Non-HCM pipeline/operator product; no canonical-family authority and no input to HCM mutation |
| `artifacts/foundation/TEST_STRATEGY_BRIEF.md` | Deferred quality-strategy family; no HCM canonical authority |
| `artifacts/foundation/QUALITY_GATES_SPEC.md` | Deferred quality/gate product; `quality_gates.yaml` remains in the separate contract/evidence system |

The last four rows are not promoted into new artifact kinds or mislabeled as
renderers. Their negative authority is proved by showing that they do not
select, mutate, fingerprint, or validate an HCM canonical artifact.

## Versioned definition publication

Packet P1A must publish exact successors without mutating released bytes:

- `handbook.artifact-kind.project-context@1.1.0`;
- `handbook.artifact-kind.environment-context@1.1.0`;
- `handbook.artifact-kind.work-specification@1.1.0`;
- `handbook.artifact-kind.decision-record@1.1.0`;
- `handbook.artifact-kind.risk-record@1.1.0`;
- one `@1.0.0` first-party intake and renderer definition for each of those five
  kinds; and
- `handbook.profile.shipped-root@1.2.0`, selecting only the existing three root
  instances and referencing the Project Context and Environment Context
  successors.

The resolved instance fields are frozen, not inferred:

| Profile/instance | Kind ref | Intake ref | Renderer refs | Canonical path |
| --- | --- | --- | --- | --- |
| `handbook.profile.shipped-root@1.2.0` / `project_authority` | `handbook.artifact-kind.project-authority@1.1.0` | `handbook.intake.charter@1.0.0` | [`handbook.renderer.charter-review-markdown@1.0.0`] | `.handbook/project/charter.yaml` |
| `handbook.profile.shipped-root@1.2.0` / `project_context` | `handbook.artifact-kind.project-context@1.1.0` | `handbook.intake.project-context@1.0.0` | [`handbook.renderer.project-context-review-markdown@1.0.0`] | `.handbook/project/context.yaml` |
| `handbook.profile.shipped-root@1.2.0` / `environment_context` | `handbook.artifact-kind.environment-context@1.1.0` | `handbook.intake.environment-context@1.0.0` | [`handbook.renderer.environment-context-review-markdown@1.0.0`] | `.handbook/project/environment.yaml` |

Project Authority and Project Context remain `always`; Environment Context
retains the existing conditional requiredness and exact condition ref. All three
keep empty Projection refs. No profile field may remain null/empty where this
table names an intake or renderer.

P1A must prove that the 1.2 `project_authority` row is byte-for-byte equal to
the released 1.1 row and must freeze the completed 1.2 profile's literal
authored fingerprint in the HCM-2.4 definition vector. P1B cannot begin while
that literal is absent, provisional, recomputed from an unreviewed definition,
or inconsistent with the typed dependency closure.

The Work Specification real-path proof is also frozen:

- fixture root:
  `crates/engine/tests/fixtures/hcm_2_4_work_specification/`;
- selection file: `.handbook/profile-selection.json`;
- repository profile source:
  `.handbook/definitions/profiles/work-specification-root-1.0.0.yaml`;
- exact profile ref:
  `example.profile.hcm-2-4-work-specification@1.0.0`, extending
  `handbook.profile.shipped-root@1.2.0`;
- exact additional instance id/kind/role:
  `work_specification` /
  `handbook.artifact-kind.work-specification@1.1.0` / `delivery_unit`;
- exact canonical path:
  `artifacts/work-specification/work-specification.yaml`;
- exact intake/renderer:
  `handbook.intake.work-specification@1.0.0` and
  [`handbook.renderer.work-specification-review-markdown@1.0.0`]; and
- requiredness `always`, with no root-profile addition, condition, lifecycle,
  Projection, validation overlay, extension, or inferred filename.

The Stage 10 pipeline proof must resolve this admitted descriptor from the
fixture selection before it captures, fingerprints, renders, or hands off the
artifact. A hard-coded path equal to the descriptor is not proof of selection.

Decision and Risk use equally exact, independent repository-profile proofs:

| Fixture root | Profile source/ref | Added instance closure |
| --- | --- | --- |
| `crates/engine/tests/fixtures/hcm_2_4_decision_record/` | `.handbook/definitions/profiles/decision-record-root-1.0.0.yaml` / `example.profile.hcm-2-4-decision-record@1.0.0` | id `decision_record`; kind `handbook.artifact-kind.decision-record@1.1.0`; role `null`; path `.handbook/records/decision.yaml`; intake `handbook.intake.decision-record@1.0.0`; renderer [`handbook.renderer.decision-record-review-markdown@1.0.0`] |
| `crates/engine/tests/fixtures/hcm_2_4_risk_record/` | `.handbook/definitions/profiles/risk-record-root-1.0.0.yaml` / `example.profile.hcm-2-4-risk-record@1.0.0` | id `risk_record`; kind `handbook.artifact-kind.risk-record@1.1.0`; role `null`; path `.handbook/records/risk.yaml`; intake `handbook.intake.risk-record@1.0.0`; renderer [`handbook.renderer.risk-record-review-markdown@1.0.0`] |

Each selection file is `.handbook/profile-selection.json`; each repository
profile extends `handbook.profile.shipped-root@1.2.0`, adds exactly its named
instance, uses `always` requiredness, and keeps condition, lifecycle, Projection,
validation-overlay, extension, and filename inference empty. These fixtures do
not add shipped-root instances.

The implementation plan may correct these proposed version numbers only before
P1A starts and only if collision discovery proves that a named version already
exists. That is a bounded planning correction, not authority to mutate a
released definition.

Definition payloads are in-scope catalog data. Out of scope “package changes”
means Cargo/package metadata, crate boundaries, package include/exclude rules,
published archive membership policy, release versions, dependency features, or
new packages. The implementation must prove that ordinary definition discovery
already carries the additive files; if it does not, stop rather than expand the
package boundary.

## Bridge and superseded-helper deletion inventory

| Legacy surface | Disposition | Replacement proof required before deletion |
| --- | --- | --- |
| `BR-HCM-2-PILOT-FLOW-01` and `ProjectContextFlowBridge` | Delete in P6 | Project Context is selected, retained, validated, and rendered through the same generic selected-artifact path as other shipped kinds; no Project Context Markdown influence |
| `BR-HCM-2-CHARTER-FLOW-01` and `CharterFlowBridge` | Delete in P6 | Common path preserves committed Charter observation, candidate/result separation, fingerprint, authority evidence, and HCM-2.2 regressions |
| `rendered_projection_for_path` | Delete, not rename | Renderer selection is by admitted instance/kind renderer refs, never fixed path; packet and fixture consumers replay exact semantics |
| `CanonicalArtifactKind`, `CANONICAL_ARTIFACT_ORDER`, `CanonicalArtifactDescriptor`, fixed `CanonicalArtifacts` fields | Remove or reduce to non-authoritative compatibility types in P6 | Every flow-consumed shipped instance is enumerated from admitted profile/descriptor selection; no fixed enum determines authority |
| `CanonicalLayoutContract` fixed Charter/Context/Environment Inventory/Feature Spec fields and `canonical_artifact_relative_path` switch | Remove fixed-family/path selection in P6 | All canonical paths come from admitted descriptors; root discovery and safe-path rules remain |
| `load_fixed_siblings[_with_contract]`, `load_with_contract_selection`, `descriptor_for_layout`, `descriptor_for` | Delete in P6 | Generic selected-instance loader has retained bytes, exact source identity, bounded reads, no-follow semantics, duplicate rejection, and stable ordering |
| `validate_artifact_markdown`, `baseline_artifact_validations`, fixed `packet_artifact_plans_for`, `present_fixture_sources_for`, and `canonical_artifact_kind_priority` branches | Rewrite/delete in P6 | Validation, packet inclusion, fixture source reporting, ordering, and budget targeting operate on selected instance identities and derived renderer outputs |
| Environment Inventory input-to-Markdown author path in `crates/engine/src/author/environment_inventory_core.rs`, `crates/compiler/src/author/environment_inventory.rs`, CLI writer, templates, and directive | Replace/delete in P2 | Structured input emits canonical Environment Context YAML; schema validation and deterministic renderer equivalence pass; legacy Markdown cannot influence flow |
| `core/library/environment_inventory/ENVIRONMENT_INVENTORY.md.tmpl` and Markdown-authority directive | Delete in P2 | Canonical YAML template/intake or schema-guided authoring covers the selected instance |
| `FEATURE_SPEC_ARTIFACT_PATH`, heading parser, Markdown capture validator, fixed provenance fields, fixed handoff reads, Feature Spec template/directive assumptions | Replace in P3 | Stage 10 persists schema-valid canonical Work Specification YAML, derives the fixed view, and binds provenance/handoff decisions to YAML |
| Bridge/fixed-family tests | Delete only bridge assertions; rewrite durable behavior tests | Replacement tests prove selected descriptors, one truth, renderer-only views, identical budget semantics, and no legacy influence |

No helper is deleted in a family packet before its replacement is green. The
aggregate P6 deletion is intentionally atomic: leaving either bridge or either
fixed selector set in place would preserve two competing authority paths.
Exported `CanonicalArtifactKind` or `CanonicalLayoutContract` shells remain
source-compatible under the no-public-API ceiling if deletion would break public
consumers; in that case P6 removes their normal-path authority and proves the
generic path does not consult them.

## Implementation packet decomposition

The executable order is P0 → P1A → P1B → P1C → (P2, P3, P4, P5) → P6 → P7.
P2–P5 are semantically independent and separately reviewable, but an
orchestrator must land them serially when they touch shared registries,
fixtures, or proof files.

### P0 — Baseline and inventory lock

Freeze the six-family matrix, released definition fingerprints, both exact
shipped-root profile definitions, three-instance root descriptors, the complete
shipped-root-to-Charter validation call path, every HCM-2.2 Charter record
identity producer/currentness consumer, legacy selector inventory,
HCM-2.1–HCM-2.3 preservation vectors, and per-symbol GitNexus impacts. Include
the exact P2 CLI help/snapshot correction and the complete HCM-2.2 focused
command wall in the live manifest. No production change is allowed in P0.
Separate implementation selection authorizes this read-only preflight only.
Production, test, definition, and template edits remain blocked until P0
records its exact live manifests/UIDs and passes its gate. After read-only
discovery establishes the exact evidence, P0 may write only its manifested
slice-local control records and the smallest coupled SPEC/plan/todo correction
needed to resolve a demonstrated contradiction. Those documentation changes
must pass the declared fresh internal review/remediation loop before P0 is
accepted; they do not authorize runtime work.

### P1A — Catalog publication and admission closure

Publish the five exact kind successors, five intakes, five renderers, and root
profile successor. Extend the artifact-kind admission rule only from its frozen
Project-Authority special case to exact resolution of admitted first-party
renderer refs; admit the exact frozen Project Context, Environment Context, Work
Specification, Decision Record, and Risk Record instance rows while retaining
every mismatch and all other later-owned dependency refusals; keep
Projection/lifecycle/review-trigger/
capability ceilings unchanged. Add exact built-in source mappings. Prove
duplicate-safe parsing, exact fingerprints, schema coverage, renderer
determinism, and unchanged discovery boundaries. P1A does not add
Work/Decision/Risk to the shipped-root profile and does not change public APIs.
It also proves the 1.2 `project_authority` descriptor is exactly the released
HCM-2.2 row and freezes the literal authored 1.2 profile fingerprint in an
immutable HCM-2.4 vector.

The exact fingerprints use the uniform HCM-2.2 typed-closure envelope, not
authored-content-only hashing:

- each intake includes the exact selected artifact-kind and candidate-schema
  fingerprints;
- each renderer includes its exact input-schema fingerprint;
- each successor kind includes stable-role, canonical-schema entry/closure, and
  singleton renderer fingerprints; and
- profile `1.2` includes every exact kind, descriptor-registry, intake,
  renderer, lifecycle, condition, schema, stable-role, vocabulary, and Context
  Resolution dependency selected by the three-instance profile.

The exact five new package intakes must take the full semantic admission path
with retained modes and coverage. The released Charter intake alone retains its
existing identity-only compatibility path. P1A must prove this distinction
through the actual `ArtifactRepositoryV1::open` path, including stale
subordinate fingerprints, wrong kind/schema binding, missing coverage, and
unsupported mode refusal.

### P1B — Versioned Charter compatibility foundation

While the shipped request still selects 1.1, extend only
`CharterDefinitionRegistry::validate_selected_decisions` to admit the two exact
ref/fingerprint tuples and complete Project Authority descriptor closure frozen
above. Preserve `validate_selected_profile` as 1.1-only. Make the existing
HCM-2.2 Charter profile identity constants crate-visible without changing their
values, and update only the listed Charter record producers/currentness checks
to continue emitting and comparing that exact identity after the compatibility
gate passes.

P1B is GREEN only when direct 1.1 and explicit 1.2 decisions produce identical
Charter authority bytes/fingerprints, every descriptor-field and tuple mutation
is rejected, all HCM-2.2 negative cases pass unchanged, and the live shipped
selection remains 1.1. It may not edit candidate/result/promotion-intent
validators, schemas, vectors, lineage formats, lifecycle formats, approval
semantics, or committed-authority behavior.

### P1C — Shipped-root successor adoption

Only after P1B is green, select
`handbook.profile.shipped-root@1.2.0` through the existing built-in profile
request, adding only the exact kind, intake, and renderer sources from P1A.
Preserve exactly three root instances and all existing condition, vocabulary,
and Context Resolution sources. This packet is isolated because
`shipped_profile_request` and `resolve_shipped_profile_decisions` are
CRITICAL-risk shared roots.

The successor selects Project Context kind `1.1` over the unchanged Project
Context schema `1.0` and canonical path. Preserve the HCM-2.1 runtime by
extending only the existing private `selected_contract_matches` predicate to
admit exact kind `1.0` or exact kind `1.1` when decision and instance refs are
equal and the schema/path remain the frozen HCM-2.1 values. This is a
version-compatibility membrane, not generic inference: no other kind, schema,
path, fallback, public signature, or production symbol is authorized.

Resolve and assert the three exact descriptor rows above; non-null intake refs
and singleton renderer refs for Project Context and Environment Context are
required GREEN evidence, not publication-only metadata. The real shipped path
must show generic selected decisions at 1.2 while every Charter record and
HCM-2.2 validation path retains the exact 1.1 authority pair.

### P2 — Environment Context vertical

Cut structured authoring, CLI, setup/doctor, compiler, and flow consumers from
Environment Inventory Markdown to the selected Environment Context YAML. Prove
conditional absence, schema parity, atomic safe write, retained observation,
full-byte rendering, and zero legacy influence. Leave aggregate bridges intact
until P6.

P2 runtime work has an unmet authority prerequisite. The frozen HCM-0.6
condition decision leaves the condition record schema, exact input bindings,
admitted evidence types, precedence, freshness thresholds, and evaluator
implementation undecided. The live shipped resolver therefore returns only
`unresolved` / `EvidenceContractUnavailable` and `indeterminate` for the
selected Environment Context descriptor. Neither
`crates/engine/src/profile_decision.rs` nor a new evaluator surface is in the
P2 selector.

The condition-true/false and write-only-when-applicable clauses remain exit
requirements; they are not authority to invent an evaluator, coerce
`indeterminate`, treat structured Environment Context input or its
`applicability_basis` as independent condition evidence, make profile selection
an applicability flag, or write while applicability is unresolved. P2
behavior-changing work must remain paused until a separately approved,
fingerprinted condition-evidence/evaluator contract either enters this slice
with an exact reviewed selector or is completed as a proven dependency. P3–P5
remain semantically independent and may proceed serially, but P6 cannot begin
while P2 is blocked.

### P3 — Work Specification / Stage 10 vertical

Make the fixed Stage 10 capture output schema-valid Work Specification YAML,
bind provenance and handoff authority to those canonical bytes, and generate
Feature Spec Markdown only as a view. Preserve frozen pipeline command names,
selectors, route-basis semantics, trust classes, and handoff contracts. This
packet is isolated because pipeline handoff emit/validate surfaces are
CRITICAL-risk.

The final workspace wall exposed a test/proof integration omission rather than
a runtime defect. P3B began with the CLI-surface proof repair frozen in
`decision/20260727-p3b-cli-surface-proof-selector-repair.md`. Its first focused
test then proved that both recursively copied CLI fixture repositories still
carried the pre-P3 Stage 10 contract. The exact additive fixture-only repair is
frozen in `decision/20260727-p3b-fixture-contract-selector-repair.md`. After
that selector was reviewed CLEAN and synchronized, the next focused test proved
that Stage 10 authoring also requires the existing HCM-2.2 durable
repository-identity prerequisite. The exact test-only, fresh-per-temporary-repo
initialization amendment is frozen in
`decision/20260727-p3b-repository-identity-prerequisite-selector-repair.md`.
The next journey wall proved that the two foundation-flow Work Specification
inputs still used a generic record ID that would replace the preserved M4
handoff feature identity. The exact two-line input repair is frozen in
`decision/20260727-p3b-foundation-feature-identity-selector-repair.md`. After
that repair, M5 proved its test-only consumer still treated the generated
Markdown view as bundle authority. The bounded canonical-YAML consumer and
evidence repair is frozen in
`decision/20260727-p3b-m5-canonical-consumer-selector-repair.md`. The resulting
full CLI wall then isolated only the paired shared compile goldens still
describing the old Markdown-only output; their exact proof-only repair is
frozen in `decision/20260727-p3b-shared-compile-golden-selector-repair.md`.
P3B may edit only those named CLI/fixture paths, deterministic golden and
journey transcript bytes, and its proof record. It cannot commit a fixture
identity, change production behavior, or reopen P3 discovery.

### P4 — Decision Record support proof

Exercise the published intake and renderer through an admitted
repository-profile descriptor and generic HCM-2.3 operation path. Prove a real
canonical YAML path, no root default, no generated command, no dynamic filename,
and no capitalized Projection.

### P5 — Risk Record support proof

Repeat the independently reviewable Decision Record obligations for the Risk
Record schema and kind. P4 and P5 may share permanent generic implementation
code, but their fixtures, schema coverage, renderer goldens, and acceptance
evidence remain separate. Combining them is allowed only if a fresh reviewer
accepts a written indivisibility rationale before implementation.

P4 and P5 are currently bounded incomplete. Their exact selected fixtures,
read/validate paths, intake coverage, and available renderer/negative evidence
are retained, but the unchanged generic mutation path rejects the
planner-generated underscore-bearing intake-value tokens before establishment.
The accepted stop and separately selectable prerequisite are frozen in
`decision/20260727-p4-p5-generic-mutation-token-authority-stop.md`. This record
does not authorize token normalization, token-grammar widening, a released
coverage-ID edit, or completion/promotion of either packet.

### P6 — Aggregate flow and fixed-selector deletion

After P1A–P1C and P2–P5 are green, replace the mixed fixed/bridge flow with one
descriptor-selected artifact collection, then remove both bridge IDs, bridge
types, fixed enum/order/path selection, legacy exceptions, and bridge-only
tests. Preserve packet order, budget outcomes, source summaries, fixture
semantics, Charter authority, Project Context behavior, and registry-brief
proof. This is the HIGH-risk flow choke point and must be one focused packet.

### P7 — Phase 2 exit proof and control-pack closeout

Run the complete proof wall, prove every Phase 2 exit row, update only earned
gate/bridge rows, perform the declared review flow, and produce the separate
implementation handoff/ledger closeout. P7 may not begin HCM-3.x.

The current parent may perform only a truthful partial control-pack and
true-stop closeout. It must preserve the open Phase 2 exit rows and both bridge
IDs, cite the exact P2 and P4/P5 authority blockers, and must not represent the
partial reviewed commits as P7 or slice completion.

## Exact implementation surface selectors

These selectors are the maximum planned implementation surface. A packet must
stop if it needs a surface outside its row unless the parent accepts a
same-scope correction after fresh impact analysis.

| Packet | Production/definition selectors | Test/proof selectors | Documentation selectors |
| --- | --- | --- | --- |
| P1A | exact new version files under `crates/engine/definitions/{artifact-kinds,intakes,renderers,profiles}/`; `crates/engine/src/profile_builtins.rs`; exact `AuthoredArtifactKindDefinition::validate` and later-owned dependency branches in `artifact_kind_registry.rs`; the exact frozen-row branch in `artifact_instance.rs`; exact P1A typed-closure branch in `ArtifactIntakeDefinitionV1::parse`; exact shipped-root `1.2` branch in `validate_authored_profile_fingerprints`; exact Charter-only compatibility selection in `load_repository_intakes`; exact type-absent string-valued `const` branch in `ResolvedSchema::collect_coverage_leaf_shapes`; `ArtifactIntakeRegistry::load_with_builtin_compatibility` and `resolve_profile_selection` remain unchanged; no public signature or other generic repository-definition behavior change beyond the exact approved string-`const` predicate | editable `crates/engine/tests/{artifact_instances,artifact_kind_registry,profile_artifact_schemas,profile_context_schemas,profile_selection,profile_work_decision_schemas,profile_risk_schema,hcm_1_2_selected_kinds,hcm_1_2_unselected_kinds,hcm_1_4_profile_decisions,hcm_1_4_profile_inspection,hcm_2_2_definition_profile,hcm_2_4_definition_support,hcm_2_4_definition_runtime}.rs` plus test-only assertions inside `crates/engine/src/schema_registry.rs`; read/proof-only `hcm_2_3_registration_kernel.rs` with zero-byte delta; exact new vectors under `slices/HCM-2.4/contracts/`, including the literal typed-closure 1.2 profile fingerprint, full Project Authority descriptor, and renderer goldens | HCM-2.4 packet only until P7 |
| P1B | `crates/engine/src/charter_definition_registry.rs` exact `CharterDefinitionRegistry.validate_selected_decisions#1`; `crates/engine/src/charter_lifecycle_validation.rs` exact `SELECTED_PROFILE_REF`, `SELECTED_PROFILE_FINGERPRINT`, `build_result`, and `validate_definition_authority`; `crates/engine/src/charter_intake.rs` exact `evaluate_charter_intake`; `crates/engine/src/charter_approval_workflow.rs` exact `validate_candidate_currentness`; `crates/engine/src/charter_promotion_workflow.rs` exact `validate_candidate_contract` and `CharterPromotionWorkflowServiceV1.promote_at#2`; `crates/engine/src/charter_authority_transaction.rs` exact `CharterAuthorityTransactionServiceV1.preflight#1`; `crates/compiler/src/doctor.rs` exact `doctor_report_from_inspection` is a read/proof anchor and is not edited; no signature/public type/schema/vector change | exact new `crates/engine/tests/hcm_2_4_charter_profile_compatibility.rs`, including `invalid_compatible_profile_decisions_cannot_produce_charter_intake`; every `crates/engine/tests/hcm_2_2_*.rs`; exact compiler doctor unit and CLI doctor/profile test commands from the proof; HCM-2.2 compiler/CLI product-cutover/version/skill tests; all direct HIGH/CRITICAL upstream tests; exact 1.1/1.2 equality and per-field negative vectors | packet proof only |
| P1C | `crates/engine/src/profile_decision.rs` exact `shipped_profile_request` source list and selected profile ref; `crates/engine/src/project_context_artifact.rs` exact existing `selected_contract_matches` predicate and its existing unit test only, admitting the exact Project Context kind `1.0`/`1.1` pair over the unchanged schema `1.0` and canonical path without adding a production symbol or fallback; `resolve_shipped_profile_decisions` is a read/proof anchor and is not edited; no signature/public type changes | editable `crates/engine/tests/{hcm_1_2_selected_kinds,hcm_1_2_unselected_kinds,hcm_1_4_profile_decisions,hcm_1_4_profile_inspection,hcm_2_2_definition_profile}.rs` and P1B compatibility target; in Unix compiler test `doctor_api_projects_the_exact_stable_project_context_row`, only report schema-version expectation `1.1.0` to `1.2.0` and Project Context kind expectation `1.0.0` to selected `1.1.0`; in Unix CLI test `doctor_reports_ready_when_required_artifacts_present`, only replace its obsolete inline Charter 1.0 setup with the existing `write_valid_selected_charter` helper and change the Project Context kind expectation `1.0.0` to selected `1.1.0`; no helper implementation, fixture asset, or other assertion edit; `crates/engine/tests/hcm_2_1_project_context.rs` is read/proof-only with required zero-byte delta and 12/12 pass; every CRITICAL upstream preservation test | packet proof only |
| P2 | `crates/engine/src/author/environment_inventory_core.rs`; exact Environment Inventory branches in `crates/engine/src/{baseline_validation,canonical_artifacts,canonical_paths,lib}.rs`; `crates/compiler/src/author/{environment_inventory,environment_inventory_shell,mod}.rs`; `crates/compiler/src/layout.rs`; exact Environment Inventory branches in `crates/compiler/src/{baseline_validation,template_library,lib}.rs`; `crates/flow/src/resolver.rs` exact Environment Inventory validation/load/budget branches; `crates/cli/src/author.rs`; `crates/cli/src/main.rs` exact `AuthorCommand::EnvironmentInventory` help text; `crates/cli/src/rendering.rs` exact Environment Inventory labels only when required to preserve the existing public result shape; `core/library/environment_inventory/{ENVIRONMENT_INVENTORY_INPUTS.yaml.tmpl,environment_inventory_directive.md,ENVIRONMENT_INVENTORY.md.tmpl}`; exact setup/doctor adapters resolved in P0; P6 retains ownership of aggregate bridge-type and fixed-selector deletion | `crates/engine/tests/author_core.rs`; Environment Inventory cases in `crates/compiler/tests/author.rs` and `crates/cli/tests/author_cli.rs`; `crates/cli/tests/cli_surface.rs` exact inline `author_help_matches_snapshot` and snapshot-consuming `author_environment_inventory_help_matches_snapshot`; exact consumed `crates/cli/tests/snapshots/handbook-author-environment-inventory-help.txt`; `crates/engine/tests/{artifact_manifest_interface,baseline_validation,canonical_artifacts_ingest,freshness_computation}.rs`; `crates/compiler/tests/{artifact_manifest_interface,canonical_artifacts_ingest,freshness_computation,rendering_surface,resolver_core,setup,doctor}.rs`; `crates/flow/tests/resolver_core.rs`; native Windows mutation/refusal cases | packet proof only |
| P3 | `core/stages/10_feature_spec.md`; two files under `core/library/feature_spec/`; `core/schemas/feature_spec.yaml`; `core/pipelines/{default,foundation_inputs}.yaml`; `crates/pipeline/src/{pipeline_capture,stage_10_feature_spec_provenance,pipeline_handoff}.rs`; exact fixed path/layout constants | `crates/pipeline/tests/{pipeline_capture,pipeline_handoff}.rs`; CLI handoff refusal tests; new exact fixture root `crates/engine/tests/fixtures/hcm_2_4_work_specification/`; exact mirrors under `tests/fixtures/pipeline_proof_corpus/foundation_inputs/` and `tests/fixtures/foundation_flow_demo/`; real-binary descriptor-selected capture/handoff proof | packet proof only |
| P3B | no runtime or library edits | `crates/cli/tests/cli_surface.rs`, including only the private fresh repository-identity prerequisite helper and two invocation points frozen by `decision/20260727-p3b-repository-identity-prerequisite-selector-repair.md` plus the three private canonical-consumer helpers frozen by `decision/20260727-p3b-m5-canonical-consumer-selector-repair.md`; exact `pipeline_handoff_emit_refuses_when_feature_spec_artifact_is_missing` setup assertion in `crates/cli/tests/pipeline_handoff_refusals.rs` frozen by `decision/20260727-p3b-negative-fixture-independence-selector-repair.md`; exact `foundation_flow_demo_feature_specs_match_directive_and_template_contract` cross-case assertion in `crates/cli/tests/feature_spec_contract.rs` frozen by `decision/20260727-p3b-feature-spec-contract-selector-repair.md`; exact `tests/fixtures/foundation_flow_demo/evidence/{happy_path,skip_path}.transcript.txt` bytes and selected M5 scorecard; the ten exact Stage 10 fixture-authority paths named by `decision/20260727-p3b-fixture-contract-selector-repair.md`; the two foundation-flow Work Specification inputs and exact generated-view/slice-plan evidence named by the two feature-identity/M5 decisions; the paired shared compile payload/explain goldens named by `decision/20260727-p3b-shared-compile-golden-selector-repair.md`; generated fixture-repo outputs remain transient and no fixture identity may be committed | P3B proof record and eight exact selector decisions only |
| P4 | additive Decision Record definitions from P1A plus unchanged generic artifact runtime consumption | exact new fixture root `crates/engine/tests/fixtures/hcm_2_4_decision_record/` and a named HCM-2.4 integration test selected in P0 | packet proof only |
| P5 | additive Risk Record definitions from P1A plus unchanged generic artifact runtime consumption | exact new fixture root `crates/engine/tests/fixtures/hcm_2_4_risk_record/` and a named HCM-2.4 integration test selected in P0 | packet proof only |
| P6 | atomic compile closure for bridge-type deletion: `crates/engine/src/{artifact_manifest,baseline_validation,canonical_artifacts,canonical_paths,freshness,lib}.rs`; `crates/compiler/src/author/{charter_shell,environment_inventory_shell,mod}.rs`; `crates/compiler/src/{baseline_validation,blocker,layout,lib,refusal}.rs`; `crates/compiler/src/rendering/{markdown,shared}.rs`; `crates/flow/src/{resolver,budget,packet_result}.rs`; `crates/cli/src/rendering.rs`; only the bridge/fixed-family branches in these files are editable | `crates/engine/tests/{artifact_manifest_interface,baseline_validation,canonical_artifacts_ingest,freshness_computation,hcm_1_1_custom_kind,hcm_2_1_project_context}.rs`; `crates/compiler/tests/{artifact_manifest_interface,author,canonical_artifacts_ingest,freshness_computation,refusal_mapping,rendering_surface,resolver_core}.rs`; `crates/flow/tests/{resolver_core,budget_domains}.rs`; `crates/cli/tests/{author_cli,cli_surface,feature_spec_contract,pipeline_handoff_refusals}.rs` | bridge rows and earned proof rows only in P7 |
| P7 | no new runtime behavior | full workspace wall and all focused regressions | `03-seam-crosswalk.md`, `04-phase-slice-map.md`, `06-proof-and-regression-ledger.md`, `09-review-finding-inventory.md`, exact HCM-2.4 `SPEC.md`/plan/todo status and evidence, and new P7 proof/review records only as earned |

The selector table does not authorize blanket directory rewrites. P0 must turn
each glob into an exact file manifest before its packet edits begin.

The P1A typed-closure/runtime-admission correction is atomic for reviewability:
the five intake fingerprints cannot be changed to include their kind/schema
dependencies while the actual package-intake loader continues trusting those
fingerprints and discarding coverage, and profile `1.2` cannot freeze until the
same intake, renderer, kind, and descriptor fingerprints converge. The
correction is limited to the exact private functions named above, one new
runtime-proof target, and unchanged HCM-2.3 registration regression proof.
The operator-approved coverage correction adds only
`ResolvedSchema::collect_coverage_leaf_shapes`: after the existing reference
resolution and ambiguity checks, a type-absent leaf with a string-valued
`const` is classified as String. Non-string `const`, `enum`, default, examples,
annotations, composites, unsupported explicit types, and every other
indeterminate shape remain refused. Released schema bytes remain unchanged,
and a sixth production symbol or any broader inference stops P1A.
Fresh depth-3 tests-included impact at the correction baseline is LOW/11 for
`ArtifactIntakeDefinitionV1::parse`, LOW/0 for
`AuthoredArtifactKindDefinition::validate` with an incomplete-edge caveat,
CRITICAL/130 across 8 processes and 10 modules for
`validate_authored_profile_fingerprints`, and CRITICAL/44 across 7 processes
and 11 modules for `load_repository_intakes`; fresh resumed-baseline impact is
LOW/1 with no indexed process for
`ResolvedSchema::collect_coverage_leaf_shapes`. Those CRITICAL results authorize
only the exact P1A/1.2 branches above; any public caller, changed process/module
contract, or edit to `load_with_builtin_compatibility` /
`resolve_profile_selection` stops the packet.

## GitNexus architecture and impact record

The amendment index was current at
`fa31f65fddd678b33ea89d6b7e41201240254986`. Full-text query is degraded
because the local FTS extension cannot load; exact context, direct source
inspection, and upstream impact are the required fallback. The original
non-amendment rows below retain their reviewed planning evidence and their P0
refresh gates. Every newly proposed compatibility/P2 symbol was resolved
exactly with no UNKNOWN. An UNKNOWN result is not “low risk”: implementation
must resolve it by UID/context or stop.

| Proposed symbol/surface | Upstream result | Planning consequence |
| --- | --- | --- |
| `rendered_projection_for_path` | HIGH; 10 impacted with tests, 2 direct, 3 modules; direct consumers are `packet_artifact_plans_for` and `present_fixture_sources_for` | Isolate in P6 and replay flow packet, fixture, budget, and resolver tests before deletion |
| `resolve_with_contract` | MEDIUM; 5 direct test consumers | P6 must preserve its public behavior and may not expand its API |
| `baseline_artifact_validations` | LOW; 6 impacted with tests | Rewrite only after selected artifact validation exists |
| `packet_artifact_plans_for` | LOW; 6 impacted with tests, 1 direct, 1 module | Preserve packet order and disposition goldens |
| `present_fixture_sources_for` | LOW; 3 impacted with tests, 1 direct, 2 modules | Preserve fixture-source identity and ordering |
| `CanonicalArtifactKind` enum UID | LOW; graph reports no upstream edges | Treat graph as incomplete because textual/type usage exists; use compiler/test wall, not the zero count, as authority |
| `CanonicalLayoutContract` struct UID | LOW; graph reports no upstream edges | Same incomplete-type-edge caveat; delete fixed fields only in P6 |
| `emit_pipeline_handoff_bundle_with_storage_layout` | CRITICAL; 259 impacted, 2 direct, 51 processes, 20 modules | P3 may make only the minimum canonical-source substitution and must replay all pipeline handoff, trust, route, and workspace tests |
| `validate_pipeline_handoff_bundle_with_storage_layout` | CRITICAL; 669 impacted, 1 direct, 51 processes, 20 modules | Treat emitted and validated bundle formats as frozen unless the packet proves an exact versioned internal successor without public API change |
| `Function:crates/engine/src/profile_decision.rs:shipped_profile_request` | CRITICAL; 155 impacted, 1 direct, 7 processes, 10 modules | P1C changes only the exact source list/profile ref after P1B is green |
| `Function:crates/engine/src/profile_decision.rs:resolve_shipped_profile_decisions` | CRITICAL; 219 impacted, 74 direct, 8 processes, 13 modules | Read/proof anchor only; preserve signature, resolution semantics, and immediate registry validation |
| `Function:crates/engine/src/charter_definition_registry.rs:CharterDefinitionRegistry.validate_selected_decisions#1` | LOW graph result; 0 impacted, 0 processes/modules | Treat as an incomplete edge: live source proves six current production callers; P1B adds exactly one seventh caller from `evaluate_charter_intake` and changes only the exact tuple/full-descriptor boundary |
| `Const:crates/engine/src/charter_lifecycle_validation.rs:SELECTED_PROFILE_REF` and `Const:crates/engine/src/charter_lifecycle_validation.rs:SELECTED_PROFILE_FINGERPRINT` | LOW; 0 impacted each | Values are immutable; only crate visibility may change so all Charter producers share the released pair |
| `Function:crates/engine/src/charter_intake.rs:evaluate_charter_intake` | HIGH; 42 impacted, 6 direct, 1 process, 4 modules | Add compatibility validation and preserve HCM-2.2 output identity |
| `Function:crates/engine/src/charter_approval_workflow.rs:validate_candidate_currentness` | HIGH; 17 impacted, 1 direct, 1 process, 3 modules | Compare only with the frozen HCM-2.2 record pair after caller validation |
| `Function:crates/engine/src/charter_promotion_workflow.rs:validate_candidate_contract` | LOW; 2 impacted, 1 direct, 1 process, 1 module | Preserve frozen candidate identity |
| `Function:crates/engine/src/charter_promotion_workflow.rs:CharterPromotionWorkflowServiceV1.promote_at#2` | LOW; 1 impacted, 1 direct, 0 processes, 1 module | Replace the false selected-profile-1.1 gate with exact registry validation and emit the frozen record pair |
| `Function:crates/engine/src/charter_lifecycle_validation.rs:build_result` | LOW; 1 impacted, 1 direct, 1 process, 1 module | Preserve frozen lifecycle-result identity |
| `Function:crates/engine/src/charter_lifecycle_validation.rs:validate_definition_authority` | LOW; 2 impacted, 2 direct, 1 process, 1 module | Replace its direct selected-profile-1.1 gate only with the exact registry boundary |
| `Function:crates/engine/src/charter_authority_transaction.rs:CharterAuthorityTransactionServiceV1.preflight#1` | HIGH; 27 impacted, 2 direct, 1 process, 3 modules | Preserve frozen promotion/candidate identity after registry validation |
| `Function:crates/compiler/src/doctor.rs:doctor_report_from_inspection` | HIGH; 12 impacted, 2 direct, 1 process, 3 modules | Existing sixth registry caller; read/proof-only, retain exact definition-closure reporting under direct 1.1 and selected 1.2 |
| `Function:crates/engine/src/charter_lineage_store.rs:validate_candidate_v13` | CRITICAL preservation anchor; 43 impacted, 14 processes | No edit; exact HCM-2.2 record validation and negatives remain unchanged |
| `Function:crates/engine/src/charter_promotion_intent_v12.rs:validate_promotion_intent_v12` | HIGH preservation anchor; 18 impacted, 3 processes | No edit; exact promotion-intent authority remains unchanged |
| `Enum:crates/cli/src/main.rs:AuthorCommand` | LOW graph result; 0 impacted | P2 changes only the `EnvironmentInventory` help path; one inline CLI assertion and one consumed subcommand snapshot prove the otherwise-missing edge |
| `derive_feature_id` | CRITICAL; 409 impacted, 1 direct, 51 processes, 20 modules | P3 must derive the same external identity from canonical Work Specification fields/bytes and replay all handoff consumers |
| `profile_builtins::definition` | LOW; graph reports no upstream edges | Treat as an incomplete dynamic-source edge; prove every predecessor/new exact ref and full shipped profile closure |
| `validate_later_owned_dependencies` | UNKNOWN by method name | Resolve its method UID before P1A; change only exact first-party renderer-ref admission and retain every other later-owned dependency refusal |
| `ArtifactInstanceRegistry` descriptor dependency guard | UNKNOWN by method context | Resolve before P1A; admit only the five exact non-Charter instance rows frozen above; retain every mismatch and all lifecycle/Projection/overlay/extension refusal |
| Environment author/render helpers, Stage 10 private capture/provenance helpers | UNKNOWN by name in the refreshed graph | Resolve exact UID and rerun upstream impact immediately before any edit; unresolved UNKNOWN is a packet stop |
| HCM-2.3 generic registry, admitted loader, schema loader, profile selection, and generic mutation symbols | Prior HCM-2.3 evidence records CRITICAL surfaces; fresh name lookup is incomplete | Consume unchanged. Any proposed edit is outside the default packet and requires exact UID impact plus parent escalation |

No implementation symbol edit is authorized by this planning impact record.
Every packet repeats impact analysis at its actual baseline. HIGH or CRITICAL
results are reported before editing; a materially wider result stops that
packet.

For the compatibility amendment, the recorded process/module counts are hard
production ceilings. Live source has six existing registry callers, including
compiler `doctor_report_from_inspection`. P1B explicitly adds one seventh
`validate_selected_decisions` production caller from
`evaluate_charter_intake`; no other new production caller is allowed. P0/P1B
may also add only exact manifested test callers. An eighth production caller,
another process/module/authority class/public surface, or an increase not wholly
explained by the one planned intake edge and those test UIDs is materially
wider and stops the packet. The zero-edge registry and CLI enum results are
never authority to skip their source-proved callers or focused tests.

## Proof wall and Phase 2 exit mapping

### Per-family proof

Every P1A and P2–P5 family row must prove:

1. exact definition and schema refs, immutable predecessor bytes, canonical
   fingerprint replay, duplicate-key refusal, and unknown-field behavior;
2. all supported intake modes validate the same kind-selected schema, with
   explicit coverage gaps rather than invented values;
3. exactly one editable YAML truth, retained bytes, exact source identity,
   safe repository-relative no-follow access, bounded reads, stable observation,
   atomic write/refusal behavior, and ABA/concurrency checks where mutation
   exists;
4. a full-byte deterministic renderer golden and fingerprint, with
   `resolution_input: null` and no clock/environment/network/discovery input;
5. legacy Markdown is neither read nor fingerprinted as authority, and changing
   or deleting a view cannot change the canonical result;
6. real-path selected-descriptor proof, not only unit construction; and
7. native Windows and Unix-safe behavior for any mutation path.

Before P2–P5, P1B/P1C additionally prove:

1. released shipped-root 1.1 bytes/fingerprint and valid acceptance are
   unchanged;
2. shipped-root 1.2 is accepted only as the one literal ref/fingerprint tuple
   after every Project Authority descriptor field equals the HCM-2.2 closure;
3. each descriptor field, tuple member, subordinate definition, unlisted
   version, range/prefix, fallback, and second-read mutation is rejected;
4. direct 1.1 and compatible selected 1.2 decisions produce byte-identical
   Charter intake, candidate, lifecycle result, approval, promotion, intent,
   lineage, transaction, recovery/replay, and committed authority;
5. the real shipped resolver selects 1.2 for generic consumers while every
   Charter record retains the released 1.1 pair; and
6. all HCM-2.2 engine integration targets, engine lib/all-features matrices,
   compiler version/product-cutover targets, CLI product-cutover/skill targets,
   and HCM-2.1/HCM-2.3 regressions pass without deleting or weakening a
   negative.

### Phase 2 exit map

| Exit obligation | Required HCM-2.4 evidence |
| --- | --- |
| One editable canonical truth per targeted artifact | Six-family matrix plus per-family canonical/view and legacy-influence tests |
| Intakes converge on kind-selected schema and expose missing coverage | Five new intake definitions, coverage vectors, and guided/express/agent-assisted equality/refusal proof; preserve Charter |
| Charter boundary remains auditable and non-competing | Exact P1B/P1C compatibility membrane proof, byte-identical 1.1 record identity under selected 1.2, unchanged HCM-2.2 negatives, and P6 common-loader proof against retained committed authority |
| Custom kind works without enum or generated command | Replay the exact HCM-2.3 registry-brief real-binary, replay, and concurrency proof |
| Derived views are fixed deterministic first-party renderers only | Six renderer rows, exact goldens, null Resolution input, and no Projection definitions |
| Generic custom-kind Projection/Resolution stays deferred | Negative surface scan and unchanged HCM-3.2/HCM-3.3 authority |
| No migration or dual-read promise | Legacy Markdown mutation/deletion has no effect; no fallback reads, warnings, or compatibility promise |
| Every Phase 2 bridge is deleted | Both bridge IDs absent; no fixed family/path selector, legacy exception, or bridge-only test remains |

### Full implementation proof wall

P7 must run, record, and keep green:

- formatting and strict lint for every affected Rust crate;
- focused engine, compiler, CLI, flow, and pipeline tests from P1A–P6;
- full workspace tests, feature-tree checks, package/archive checks, and the
  repository's normal build wall;
- definition/schema/intake/renderer JSON/YAML parsing and duplicate-key vectors;
- canonical emitter and renderer full-byte goldens;
- native Windows refusal/mutation tests;
- HCM-2.1 Project Context, HCM-2.2 Charter, and HCM-2.3 registry-brief focused
  regressions;
- direct shipped-root 1.1 acceptance, exact shipped-root 1.2 compatibility,
  full Project Authority descriptor-mutation refusal, and real-path 1.2
  selection with byte-identical HCM-2.2 Charter record proof;
- bridge ID, fixed path, legacy Markdown influence, generated command, dynamic
  dispatch, Projection, and Resolution negative scans;
- changed-document links, anchors, fences, JSON, Markdown, formatting, secret,
  whitespace, and `git diff --check`;
- handoff validation plus both handoff self-tests;
- package manifest equality proving no Cargo/package-boundary/dependency change;
  and
- fresh GitNexus analysis and change detection limited to expected symbols and
  execution flows.

The only gate rows P7 may close or broaden are the exact earned portions of
`PG-KIND-01`, `PG-ARTIFACT-01`, and program-wide `PG-YAML-02`, plus deletion of
`BR-HCM-2-PILOT-FLOW-01` and `BR-HCM-2-CHARTER-FLOW-01`. It preserves
`PG-KIND-02`'s HCM-2.3 evidence and all closed Charter gates.

## Review budget

The HCM-2.4 implementation subject receives:

1. one complete-subject discovery review or bounded same-fingerprint burst;
2. one consolidation of all demonstrated P1/P2 findings and one remediation;
3. one different-fresh, delta-focused closure review; and
4. at most two supplemental remediation/closure cycles only for valid P1/P2
   findings directly caused or unmasked by the preceding remediation.

Each supplemental cycle stays within selected scope, authority, public API,
dependency, unsafe policy, package boundary, and risk ceiling; consolidates all
causally related findings; and does not reopen discovery. An unrelated P1/P2,
material scope/risk expansion, or exhausted allowance produces a bounded
non-completed stop. No review budget waives a demonstrated P1/P2. P3/P4 remains
visible in `09-review-finding-inventory.md` and does not force another loop.

## Explicit non-goals and authority stops

HCM-2.4 does not authorize:

- HCM-3.x Context Resolution or capitalized Projection work;
- generic configured custom-kind Projections;
- SDK, public transport, remote-schema, public API, or generated CLI expansion;
- dynamic filename, kind-to-command, or command dispatch;
- migration tooling, legacy dual reads, compatibility promises, or automatic
  conversion of user files;
- dependencies, Cargo files, unsafe-policy changes, crate/package boundary or
  release changes, package metadata, or feature changes;
- adding Work Specification, Decision Record, or Risk Record to the shipped
  root profile;
- reinterpretation of Charter authority or HCM-2.3 registry-brief semantics; or
- open-ended profile-version acceptance, a loose string exception, a changed
  HCM-2.2 Charter record identity, migration, fallback, or dual-read behavior;
  or
- automatic start of implementation after planning closeout.

Stop the active implementation packet and return to the parent when:

- exact UID impact remains UNKNOWN, or a HIGH/CRITICAL blast radius materially
  exceeds this plan;
- a replacement requires a public API, dependency, Cargo, unsafe, package, SDK,
  transport, remote-schema, or HCM-3.x change;
- a renderer needs Resolution input or becomes a generic Projection;
- a family needs filename discovery, a generated command, root-profile
  expansion, migration, or dual reads;
- P2 would need to infer, coerce, or self-authorize Environment Context
  applicability, or edit the unselected condition evaluator before its exact
  evidence/evaluator contract is approved;
- P4 or P5 would require generic coverage-token derivation normalization,
  lineage token-grammar widening, or a released intake-definition edit before
  a separately approved and impacted runtime selector exists;
- the P1A literal 1.2 fingerprint is absent or changes after compatibility
  review, any Project Authority descriptor field differs, any HCM-2.2 record
  byte/fingerprint changes, or `validate_candidate_v13` /
  `validate_promotion_intent_v12` would need an edit;
- Charter or registry-brief preservation proof fails; or
- bridge deletion begins before P1A–P1C and all P2–P5 replacement proofs are
  green.

## Implementation-entry acceptance criteria

Implementation may be separately selected only when all of the following are
true:

- the completed HCM-2.4 planning handoff explicitly identifies the reviewed
  amended planning commit, exact final subject fingerprint, and predecessor
  handoff `20260726T050447Z--HCM-2-4--orchestration--planning-completed`;
- the branch is clean and synchronized, and the reviewed planning commit is an
  ancestor of the selected implementation baseline;
- all six first-party families plus registry-brief preservation have complete
  source/view/intake/renderer/deletion/proof rows with no undecided authority;
- released HCM-2.1–HCM-2.3 bytes and the exact three-instance root profile are
  frozen;
- the exact versioned Charter compatibility membrane, P1A literal-fingerprint
  prerequisite, HCM-2.2 record-identity preservation, and P2 CLI help/snapshot
  correction are present;
- this reviewed plan freezes selector ceilings, dependencies, RED/GREEN checks,
  proof, rollback/stop conditions, and reviewable packet sizes;
- separate implementation selection enters read-only P0; P0 then freezes exact
  live file manifests and symbol UIDs before any implementation edit;
- implementation is instructed to rerun fresh GitNexus impact before every
  symbol edit and stop on unresolved UNKNOWN or unaccepted material expansion;
- the non-goals above are repeated in the implementation selection; and
- implementation is not inferred from this planning completion.

See [tasks/plan.md](tasks/plan.md) for packet execution and
[tasks/todo.md](tasks/todo.md) for the implementation checklist.
