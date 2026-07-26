# HCM-2.4 — Remaining shipped artifact-family conversion

Status: planning subject; implementation is not authorized.

Planning baseline: `2b7ab4e14467800e5f0ecaa19561a1dd5d84ee48`

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

This document freezes implementation packets and proof obligations. It does not
authorize production, test, definition, template, or package edits.

## Authority and preserved decisions

The implementation must preserve these decisions byte-for-byte or
behavior-for-behavior unless an explicitly versioned HCM-2.4 successor is named
below:

1. Project Authority remains the Charter selected at
   `.handbook/project/charter.yaml`, using
   `handbook.artifact-kind.project-authority@1.1.0`,
   `handbook.intake.charter@1.0.0`, and
   `handbook.renderer.charter-review-markdown@1.0.0`. Candidate, result,
   promotion, approval, lineage, lifecycle, and committed-authority boundaries
   from HCM-2.2 are not reopened.
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

The executable order is P0 → P1A → P1B → (P2, P3, P4, P5) → P6 → P7.
P2–P5 are semantically independent and separately reviewable, but an
orchestrator must land them serially when they touch shared registries,
fixtures, or proof files.

### P0 — Baseline and inventory lock

Freeze the six-family matrix, released definition fingerprints, three-instance
root profile, legacy selector inventory, HCM-2.1–HCM-2.3 preservation vectors,
and per-symbol GitNexus impacts. No production change is allowed in P0.
Separate implementation selection authorizes this read-only preflight only.
Production, test, definition, template, and documentation edits remain blocked
until P0 records its exact live manifests/UIDs and passes its gate.

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

### P1B — Shipped-root successor adoption

Admit and select `handbook.profile.shipped-root@1.2.0` through the existing
built-in profile request, adding only the exact kind, intake, and renderer
sources from P1A. Preserve exactly three root instances and all existing
condition, vocabulary, and Context Resolution sources. This packet is isolated
because `shipped_profile_request` and `resolve_shipped_profile_decisions` are
CRITICAL-risk shared roots.
Resolve and assert the three exact descriptor rows above; non-null intake refs
and singleton renderer refs for Project Context and Environment Context are
required GREEN evidence, not publication-only metadata.

### P2 — Environment Context vertical

Cut structured authoring, CLI, setup/doctor, compiler, and flow consumers from
Environment Inventory Markdown to the selected Environment Context YAML. Prove
conditional absence, schema parity, atomic safe write, retained observation,
full-byte rendering, and zero legacy influence. Leave aggregate bridges intact
until P6.

### P3 — Work Specification / Stage 10 vertical

Make the fixed Stage 10 capture output schema-valid Work Specification YAML,
bind provenance and handoff authority to those canonical bytes, and generate
Feature Spec Markdown only as a view. Preserve frozen pipeline command names,
selectors, route-basis semantics, trust classes, and handoff contracts. This
packet is isolated because pipeline handoff emit/validate surfaces are
CRITICAL-risk.

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

### P6 — Aggregate flow and fixed-selector deletion

After P1A–P5 are green, replace the mixed fixed/bridge flow with one
descriptor-selected artifact collection, then remove both bridge IDs, bridge
types, fixed enum/order/path selection, legacy exceptions, and bridge-only
tests. Preserve packet order, budget outcomes, source summaries, fixture
semantics, Charter authority, Project Context behavior, and registry-brief
proof. This is the HIGH-risk flow choke point and must be one focused packet.

### P7 — Phase 2 exit proof and control-pack closeout

Run the complete proof wall, prove every Phase 2 exit row, update only earned
gate/bridge rows, perform the declared review flow, and produce the separate
implementation handoff/ledger closeout. P7 may not begin HCM-3.x.

## Exact implementation surface selectors

These selectors are the maximum planned implementation surface. A packet must
stop if it needs a surface outside its row unless the parent accepts a
same-scope correction after fresh impact analysis.

| Packet | Production/definition selectors | Test/proof selectors | Documentation selectors |
| --- | --- | --- | --- |
| P1A | exact new version files under `crates/engine/definitions/{artifact-kinds,intakes,renderers,profiles}/`; `crates/engine/src/profile_builtins.rs`; exact later-owned dependency guards in `crates/engine/src/{artifact_kind_registry,artifact_instance}.rs`; `artifact_intake_registry.rs` and `artifact_repository.rs` are proof/read surfaces unless P0 proves a minimal edit is necessary | `crates/engine/tests/{hcm_1_2_selected_kinds,hcm_1_2_unselected_kinds,hcm_1_4_profile_decisions,hcm_1_4_profile_inspection,hcm_2_2_definition_profile}.rs`; exact new vectors under `slices/HCM-2.4/contracts/` | HCM-2.4 packet only until P7 |
| P1B | `crates/engine/src/profile_decision.rs` exact `shipped_profile_request` source list and selected profile ref; no signature/public type changes | `crates/engine/tests/{hcm_1_2_selected_kinds,hcm_1_2_unselected_kinds,hcm_1_4_profile_decisions,hcm_1_4_profile_inspection,hcm_2_2_definition_profile}.rs` and all CRITICAL upstream preservation tests | packet proof only |
| P2 | `crates/engine/src/author/environment_inventory_core.rs`; `crates/compiler/src/author/{environment_inventory,environment_inventory_shell,mod}.rs`; `crates/compiler/src/layout.rs`; `crates/cli/src/author.rs`; three files under `core/library/environment_inventory/`; exact setup/doctor adapters resolved in P0 | `crates/engine/tests/author_core.rs`; Environment Inventory cases in `crates/compiler/tests/author.rs` and `crates/cli/tests/author_cli.rs`; affected canonical ingest/freshness/manifest suites; native Windows mutation/refusal cases | packet proof only |
| P3 | `core/stages/10_feature_spec.md`; two files under `core/library/feature_spec/`; `core/schemas/feature_spec.yaml`; `core/pipelines/{default,foundation_inputs}.yaml`; `crates/pipeline/src/{pipeline_capture,stage_10_feature_spec_provenance,pipeline_handoff}.rs`; exact fixed path/layout constants | `crates/pipeline/tests/{pipeline_capture,pipeline_handoff}.rs`; CLI handoff refusal tests; new exact fixture root `crates/engine/tests/fixtures/hcm_2_4_work_specification/`; exact mirrors under `tests/fixtures/pipeline_proof_corpus/foundation_inputs/` and `tests/fixtures/foundation_flow_demo/`; real-binary descriptor-selected capture/handoff proof | packet proof only |
| P4 | additive Decision Record definitions from P1A plus unchanged generic artifact runtime consumption | exact new fixture root `crates/engine/tests/fixtures/hcm_2_4_decision_record/` and a named HCM-2.4 integration test selected in P0 | packet proof only |
| P5 | additive Risk Record definitions from P1A plus unchanged generic artifact runtime consumption | exact new fixture root `crates/engine/tests/fixtures/hcm_2_4_risk_record/` and a named HCM-2.4 integration test selected in P0 | packet proof only |
| P6 | `crates/engine/src/{canonical_artifacts,canonical_paths,artifact_manifest}.rs`; `crates/compiler/src/layout.rs` plus exact fixed-sibling adapter; `crates/flow/src/{resolver,budget,packet_result}.rs`; only directly affected setup/doctor consumers | `crates/engine/tests/{canonical_artifacts_ingest,artifact_manifest_interface}.rs` and affected baseline/freshness suites; `crates/flow/tests/{resolver_core,budget_domains}.rs`; `crates/compiler/tests/rendering_surface.rs`; affected CLI tests | bridge rows and earned proof rows only in P7 |
| P7 | no new runtime behavior | full workspace wall and all focused regressions | `04-phase-slice-map.md`, `06-proof-and-regression-ledger.md`, `09-review-finding-inventory.md` only as earned; HCM-2.4 proof/review records |

The selector table does not authorize blanket directory rewrites. P0 must turn
each glob into an exact file manifest before its packet edits begin.

## GitNexus architecture and impact record

The index was refreshed at `2b7ab4e14467800e5f0ecaa19561a1dd5d84ee48`.
Full-text query is degraded because the local FTS extension cannot load;
context, direct source inspection, and upstream impact are the required
fallback. An UNKNOWN result is not “low risk”: implementation must resolve it
by UID/context or stop.

| Proposed symbol/surface | Upstream result | Planning consequence |
| --- | --- | --- |
| `rendered_projection_for_path` | HIGH; 10 impacted with tests, 2 direct, 3 modules; direct consumers are `packet_artifact_plans_for` and `present_fixture_sources_for` | Isolate in P6 and replay flow packet, fixture, budget, and resolver tests before deletion |
| `resolve_with_contract` | MEDIUM; 5 direct test consumers | P6 must preserve its public behavior and may not expand its API |
| `baseline_artifact_validations` | LOW; 6 impacted with tests | Rewrite only after selected artifact validation exists |
| `packet_artifact_plans_for` | LOW; 2 upstream (`resolve_with_contract`, `resolve`) without test expansion | Preserve packet order and disposition goldens |
| `present_fixture_sources_for` | LOW; 4 upstream through fixture/build/resolve | Preserve fixture-source identity and ordering |
| `CanonicalArtifactKind` enum UID | LOW; graph reports no upstream edges | Treat graph as incomplete because textual/type usage exists; use compiler/test wall, not the zero count, as authority |
| `CanonicalLayoutContract` struct UID | LOW; graph reports no upstream edges | Same incomplete-type-edge caveat; delete fixed fields only in P6 |
| `emit_pipeline_handoff_bundle_with_storage_layout` | CRITICAL; 259 impacted, 2 direct, 51 processes, 20 modules | P3 may make only the minimum canonical-source substitution and must replay all pipeline handoff, trust, route, and workspace tests |
| `validate_pipeline_handoff_bundle_with_storage_layout` | CRITICAL; 669 impacted, 1 direct, 51 processes, 20 modules | Treat emitted and validated bundle formats as frozen unless the packet proves an exact versioned internal successor without public API change |
| `shipped_profile_request` | CRITICAL; 404 impacted, 1 direct, 51 processes, 20 modules | Isolate exact source-list/profile-ref adoption in P1B and replay every selected-profile, Charter, authoring, flow, pipeline, and generic-operation consumer |
| `resolve_shipped_profile_decisions` | CRITICAL; 443 impacted, 39 direct, 51 processes, 20 modules | Preserve signature and decision semantics; P1B changes only its request's immutable source closure |
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

## Proof wall and Phase 2 exit mapping

### Per-family proof

Every P1A–P5 family row must prove:

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

### Phase 2 exit map

| Exit obligation | Required HCM-2.4 evidence |
| --- | --- |
| One editable canonical truth per targeted artifact | Six-family matrix plus per-family canonical/view and legacy-influence tests |
| Intakes converge on kind-selected schema and expose missing coverage | Five new intake definitions, coverage vectors, and guided/express/agent-assisted equality/refusal proof; preserve Charter |
| Charter boundary remains auditable and non-competing | HCM-2.2 focused regressions plus P6 common-loader proof against retained committed authority |
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
- automatic start of implementation after planning closeout.

Stop the active implementation packet and return to the parent when:

- exact UID impact remains UNKNOWN, or a HIGH/CRITICAL blast radius materially
  exceeds this plan;
- a replacement requires a public API, dependency, Cargo, unsafe, package, SDK,
  transport, remote-schema, or HCM-3.x change;
- a renderer needs Resolution input or becomes a generic Projection;
- a family needs filename discovery, a generated command, root-profile
  expansion, migration, or dual reads;
- Charter or registry-brief preservation proof fails; or
- bridge deletion begins before all P1A–P5 replacement proofs are green.

## Implementation-entry acceptance criteria

Implementation may be separately selected only when all of the following are
true:

- the completed HCM-2.4 planning handoff explicitly identifies the reviewed
  planning commit and exact final subject fingerprint;
- the branch is clean and synchronized, and the reviewed planning commit is an
  ancestor of the selected implementation baseline;
- all six first-party families plus registry-brief preservation have complete
  source/view/intake/renderer/deletion/proof rows with no undecided authority;
- released HCM-2.1–HCM-2.3 bytes and the exact three-instance root profile are
  frozen;
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
