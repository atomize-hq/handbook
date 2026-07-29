# HCM-2.4 P6 aggregate-flow cleanup selector

Status: selected and entry-authorized for a later implementation session;
implementation is unstarted. Independent v1.4 selector review must be CLEAN
before this status is committed.

Date: 2026-07-29

## Trigger and earned entry authority

The operator explicitly selected only the HCM-2.4 P6 selector and entry
contract from true-stop handoff
`20260729T203832Z--HCM-2-4--orchestration--p4-true-stop-partial` at starting
HEAD `9df90f55e0c3ea16391866f3539157ed528d9f2e`.

The live dependency proof establishes all P6 prerequisites as ancestors of
that HEAD:

- P0-P1C: `5cf41d2debc7af6bbbcfe5fddf26c6882482333f`;
- P2 implementation and authority: `9b3edf2` and `f62141b`;
- P3/P3B: `5a2ccf65b18a37788a48e769984671ad6d587310`;
- P4/P5 shared prerequisite: `00dde0162fcb15576c83b6ed40ab7286488d0890`;
  and
- P4 negative proof: `158115daa2b85d41663b79dc677fce69e59fd1e2`.

Focused Charter, Project Context, Environment Context, Decision, Risk,
pipeline capture, and pipeline handoff baselines are GREEN. Both bridge IDs,
all four bridge types, fixed family/path selection, and bridge-only tests are
still present. P6 implementation has therefore not begun.

This decision is selection and entry authority only. It does not perform or
accept the implementation.

## Exact owner and execution-flow boundary

`handbook-flow::resolver::resolve_with_contract` is the sole P6 execution-flow
owner. Its selected implementation flow is exactly:

1. resolve the admitted repository profile and its instance decisions through
   the existing engine profile/registry boundary;
2. build one `CanonicalArtifacts` collection from those admitted descriptors;
3. retain, per collected entry, the admitted instance ID, exact kind ref,
   label, descriptor path, requiredness/applicability, renderer refs, exact
   canonical bytes, source byte length/fingerprint, presence, and ingest issue;
4. order entries by applicability rank `Required`, `Optional`, then
   `Indeterminate`, with exact instance ID as the deterministic tie-breaker;
5. select only already-implemented fixed renderers through the admitted
   instance/kind/renderer-ref tuple, retaining rendered bytes and their
   fingerprint separately from source bytes;
6. preserve the committed Charter authority check and authority evidence;
7. derive manifest/freshness, baseline validation, budget, packet plans,
   source summaries, fixture lineage, blockers, refusals, and packet result
   from that same collection; and
8. project the unchanged compiler/CLI output contract.

The engine owns profile/descriptor admission, safe bounded no-follow reads,
duplicate-free canonical validation, retained bytes, exact fingerprints, and
the existing first-party renderers. Flow owns packet selection and projection.
Compiler and CLI remain output adapters only. No second collection, path
lookup, or bridge is allowed.

`resolve_with_contract` keeps its public signature. A source-compatible
`CanonicalLayoutContract` shell may remain only where deletion would break the
existing call surface. Its system-root/fixture-root information may remain in
use, but none of its Charter, Project Context, Environment Context, or Feature
Spec path fields or switches may select an artifact. Canonical artifact paths
come only from admitted descriptors.

## Exact collection and fixed-renderer contract

The implementation must reshape existing symbols rather than add a public API:

- `CanonicalArtifacts` becomes the ordered descriptor-selected collection;
- `CanonicalArtifact` remains the per-entry holder and retains exact source
  bytes when present;
- `CanonicalArtifactIdentity` carries selected instance/kind identity instead
  of taking authority from `CanonicalArtifactKind`;
- `RenderedArtifactProjection` becomes family-neutral retained rendered state,
  not an enum selecting a family;
- `PacketArtifactInputs` carries the one collection and its renderer results,
  not three bridges plus fixed siblings; and
- `PacketArtifactPlan` is produced in collection order without a fixed array.

The only renderer dispatch admitted by P6 is the exact existing implementation
behind each of these selected tuples:

| Instance | Kind ref | Fixed renderer ref | Preserved behavior |
| --- | --- | --- | --- |
| `project_authority` | `handbook.artifact-kind.project-authority@1.1.0` | `handbook.renderer.charter-review-markdown@1.0.0` | selected Charter YAML, distinct source/render fingerprints, and committed authority evidence |
| `project_context` | `handbook.artifact-kind.project-context@1.1.0` | `handbook.renderer.project-context-review-markdown@1.0.0` | selected Project Context YAML and deterministic Markdown view |
| `environment_context` | `handbook.artifact-kind.environment-context@1.1.0` | `handbook.renderer.environment-context-review-markdown@1.0.0` | optional advisory context; missing/invalid remains non-blocking |

A descriptor without one of those exact already-executable tuples retains its
canonical bytes and follows the existing verbatim/summary/exclusion policy. P6
must not execute a configured renderer, infer an implementation from a string,
or add Work Specification, Decision Record, Risk Record, or custom-kind
renderer machinery. An unadmitted legacy fixed Feature Spec path is removed
from flow authority; this is the intended fixed-selector deletion, not a
permission to invent a replacement instance.

## Complete production path and symbol manifest

Only the live bridge/fixed-family branches of these paths are editable. The
named symbols are the complete planned authority and compile-propagation
surface; unlisted symbols in the same files are read-only preservation anchors.

| Path | Exact planned symbols/branches |
| --- | --- |
| `crates/engine/src/artifact_manifest.rs` | `ArtifactManifest`, `ArtifactManifest::from_canonical_artifacts`, `ArtifactManifest::generate` collection/fingerprint branches |
| `crates/engine/src/baseline_validation.rs` | `baseline_artifact_validations`, `baseline_artifact_validation`, `validation_for_descriptor`, `verdict_for_descriptor`, `canonical_artifact` |
| `crates/engine/src/canonical_artifacts.rs` | `CanonicalArtifactKind`, `CANONICAL_ARTIFACT_ORDER`, `CanonicalArtifactDescriptor`, `CANONICAL_ARTIFACT_DESCRIPTORS`, `canonical_artifact_descriptors`, `CanonicalArtifactIdentity`, `CanonicalArtifact`, `CanonicalArtifacts` and its load/identity methods, `canonical_root_scaffold_exists`, `load_one`, `missing_one`, `descriptor_for_layout`, `descriptor_for`; delete `load_fixed_siblings`, `load_fixed_siblings_with_contract`, and `load_with_contract_selection` |
| `crates/engine/src/canonical_paths.rs` | `CanonicalLayoutContract` and its fixed artifact switch/accessors, `default_canonical_layout_contract`, `validate_canonical_layout_contract`, `canonical_artifact_relative_path`, `CanonicalLayout` fixed artifact methods and constants |
| `crates/engine/src/freshness.rs` | `compute_freshness`, `fingerprint_bytes`, `canonical_artifact_kind_sort_key`, and the exact `OverrideTarget::CanonicalArtifact` identity branch |
| `crates/engine/src/lib.rs` | only exports made stale by the preceding exact type/helper deletion or reduction; no additive export |
| `crates/compiler/src/author/charter_shell.rs` | `preflight_author_charter`, `validate_authoring_preconditions` fixed-collection branches |
| `crates/compiler/src/author/mod.rs` | `validate_system_root_for_authoring`, `canonical_artifact_identity`, `canonical_artifact`, `baseline_authoring_eligibility` |
| `crates/compiler/src/baseline_validation.rs` | `baseline_artifact_validations`, `baseline_artifact_validation`, delete fixed `validate_artifact_markdown` dispatch |
| `crates/compiler/src/blocker.rs` | `author_or_fill_next_safe_action`, `required_artifact_blocker`, `cmp_subject`, `subject_kind_priority`, `canonical_artifact_kind_priority` |
| `crates/compiler/src/layout.rs` | delete `canonical_artifact_relative_path`; remove fixed artifact selection from `CanonicalLayout` and `AuthoringArtifactLayout` while preserving authoring lock paths |
| `crates/compiler/src/lib.rs` | only exports made stale by the exact type propagation; no additive export |
| `crates/compiler/src/refusal.rs` | only the `SubjectRef::CanonicalArtifact` identity field propagation |
| `crates/compiler/src/rendering/markdown.rs` | only `render_subject_ref` and `render_canonical_artifact_kind` identity/label projection |
| `crates/compiler/src/rendering/shared.rs` | only `render_packet_source_summary`, `render_packet_section`, `render_subject_ref`, and both `render_canonical_artifact_kind` projections |
| `crates/flow/src/resolver.rs` | delete both bridge ID constants, `CharterFlowBridgeFailure`, `CharterFlowBridge`, `ProjectContextFlowBridge`, `EnvironmentContextFlowBridge`, and `rendered_projection_for_path`; reshape `CharterAuthorityEvidence`, `RenderedArtifactProjection`, flow `baseline_artifact_validations`, `author_or_fill_next_safe_action`, `required_artifact_blocker`, `canonical_artifact_kind_priority`, `resolve_with_contract`, `BuildPacketResultInput`, `build_packet_result`, `PacketArtifactPlan`, `PacketArtifactInputs`, `packet_artifact_plans_for`, `included_sources_for`, `present_fixture_sources_for`, `packet_sections_for`, `fixture_context_for`, `compute_refusal`, and `compute_blockers` only as required by the one collection |
| `crates/flow/src/budget.rs` | `evaluate_budget`, `evaluate_budget_with_effective_bytes` identity propagation only; disposition/reason/target algorithm is frozen |
| `crates/flow/src/packet_result.rs` | `PacketSourceSummary`, `PacketSection`, and `PacketResult` identity propagation only; packet fields and semantics are frozen |
| `crates/cli/src/rendering.rs` | only `render_packet_source_summary`, `render_canonical_artifact_kind`, `render_subject_ref`, `flow_subject_ref_for_rendering`, and `prepare_flow_output` propagation |

No definition, schema, profile, Cargo, fixture, pipeline, or other production
path is part of P6. The recursive live inventory found no bridge/fixed-selector
consumer outside this SPEC ceiling.

## Complete test and proof manifest

The only executable proof paths editable in the later implementation are:

- engine: `crates/engine/tests/artifact_manifest_interface.rs`,
  `baseline_validation.rs`, `canonical_artifacts_ingest.rs`,
  `freshness_computation.rs`, `hcm_1_1_custom_kind.rs`, and
  `hcm_2_1_project_context.rs`;
- compiler: `crates/compiler/tests/artifact_manifest_interface.rs`, `author.rs`,
  `canonical_artifacts_ingest.rs`, `freshness_computation.rs`,
  `refusal_mapping.rs`, `rendering_surface.rs`, and `resolver_core.rs`;
- flow: `crates/flow/tests/resolver_core.rs` and `budget_domains.rs`; and
- CLI: `crates/cli/tests/author_cli.rs`, `cli_surface.rs`,
  `feature_spec_contract.rs`, and `pipeline_handoff_refusals.rs`.

No fixture asset may change. Existing fixtures may be copied into temporary
repositories and supplemented only with test-created selection/definition
files. Delete only bridge/fixed-selector assertions. Permanent behavior tests
must continue to prove exact source/render fingerprints, selected paths,
ordering, packet dispositions, budget byte domains, summaries, fixture
lineage, blockers, refusals, Charter committed authority, Project Context
legacy-Markdown non-influence, and Environment Context advisory omission.

The two bridge-only HCM-2.1 tests
`fixed_sibling_loader_never_ingests_retired_project_context_member` and
`fixed_sibling_loader_recognizes_selected_project_context_namespace_as_root_scaffold`
must be replaced by descriptor-selected tests with the same durable no-legacy
and root-recognition behavior. In
`hcm_2_2_flow_projects_selected_charter_yaml_and_ignores_legacy_markdown`, only
the bridge-ID assertion is deleted; promotion/lifecycle authority evidence and
all source/render assertions remain permanent.

## Live bridge and fixed-selector inventory

At selection time the exact temporary inventory is:

- IDs `BR-HCM-2-CHARTER-FLOW-01` and `BR-HCM-2-PILOT-FLOW-01`;
- `CharterFlowBridgeFailure`, `CharterFlowBridge`,
  `ProjectContextFlowBridge`, and `EnvironmentContextFlowBridge`;
- `RenderedArtifactProjection` as a three-variant family enum and
  `rendered_projection_for_path` as a path selector;
- `CanonicalArtifactKind`, `CANONICAL_ARTIFACT_ORDER`,
  `CanonicalArtifactDescriptor`, `CANONICAL_ARTIFACT_DESCRIPTORS`, fixed
  `CanonicalArtifacts` fields, and fixed setup templates;
- `CanonicalLayoutContract` artifact fields/switch,
  both engine/compiler `canonical_artifact_relative_path` switches, and fixed
  layout constants;
- `load_fixed_siblings`, `load_fixed_siblings_with_contract`,
  `load_with_contract_selection`, `descriptor_for_layout`, and `descriptor_for`;
- fixed `validate_artifact_markdown`, three-bridge
  `baseline_artifact_validations`, fixed-array `packet_artifact_plans_for`,
  fixed-array `present_fixture_sources_for`, and engine/compiler/flow kind
  priority switches; and
- the two fixed-sibling tests and the one Charter bridge-ID assertion named
  above.

P6 GREEN requires exact recursive absence of both IDs, all bridge types,
`rendered_projection_for_path`, both fixed-sibling loader names, and every
normal-path fixed family/path/order selector. A compatibility enum/layout shell
may remain only if it is source-compatible and recursively proven absent from
normal selection, loading, ordering, validation, rendering, packet, budget,
fixture, blocker, refusal, and fingerprint decisions.

## GitNexus upstream impact ledger

The index was refreshed at the live selection baseline with `--index-only`, so
AGENTS/CLAUDE/skills were not injected. FTS remains unavailable; exact UID/name
impact and source inventory are authoritative. Every result below is upstream,
depth 3, tests included. `D`, `P`, and `M` are direct callers, affected
processes, and affected modules. Graph-zero type edges are incomplete and the
recursive textual manifest above remains mandatory.

| Planned symbol | Risk / impacted | D / P / M | Direct callers and process boundary |
| --- | --- | --- | --- |
| `CanonicalArtifactKind` UID | LOW / 0 | 0 / 0 / 0 | incomplete type edge; textual compile closure controls |
| `CANONICAL_ARTIFACT_ORDER` | LOW / 0 | 0 / 0 / 0 | no graph caller; direct textual loop exists |
| `CanonicalArtifactDescriptor` | LOW / 6 | 1 / 0 / 0 | module construction |
| `CANONICAL_ARTIFACT_DESCRIPTORS` | LOW / 0 | 0 / 0 / 0 | incomplete constant edge; textual constructor/use scan controls |
| `canonical_artifact_descriptors` | MEDIUM / 40 | 4 / 0 / 2 | engine baseline validation plus compiler/CLI author fixtures |
| `CanonicalArtifactIdentity` UID | **HIGH / 33** | 12 / 2 / 3 | loaders, freshness/budget tests, bridge accessor; reaches `evaluate_intake_document` and `evaluate_committed_read` |
| `CanonicalArtifact` UID | **CRITICAL / 27** | 11 / 0 / 8 | blocker, refusal, rendering, loader, ingest, and bridge callers across the aggregate-flow closure |
| `CanonicalArtifacts` UID | LOW / 0 | 0 / 0 / 0 | incomplete type edge; textual compile closure controls |
| `CanonicalArtifacts::load` | LOW / 0 | 0 / 0 / 0 | incomplete method edge; exact textual call inventory controls |
| `CanonicalArtifacts::load_with_contract` | LOW / 3 | 3 / 0 / 2 | `load` plus two non-default-contract tests |
| `CanonicalArtifacts::identities` | **HIGH / 21** | 1 / 1 / 3 | manifest construction; reaches `emit_pipeline_handoff_bundle_with_storage_layout` |
| `load_fixed_siblings` | LOW / 2 | 2 / 0 / 1 | two HCM-2.1 bridge-only tests |
| `load_fixed_siblings_with_contract` | LOW / 9 | 2 / 0 / 2 | `load_fixed_siblings`, `resolve_with_contract` |
| `load_with_contract_selection` | LOW / 14 | 2 / 0 / 2 | `load_with_contract`, `load_fixed_siblings_with_contract` |
| `canonical_root_scaffold_exists` | LOW / 8 | 1 / 0 / 2 | fixed selection loader; transitive depth counts 1 / 2 / 5 |
| `load_one` | LOW / 8 | 1 / 0 / 2 | fixed selection loader; transitive depth counts 1 / 2 / 5 |
| `missing_one` | LOW / 9 | 2 / 0 / 2 | fixed selection loader and `load_one` |
| `descriptor_for_layout` | LOW / 5 | 2 / 0 / 2 | `load_one`, `missing_one` |
| `descriptor_for` | LOW / 15 | 3 / 0 / 2 | setup template, ingest issue, layout descriptor |
| `CanonicalLayoutContract` UID | LOW / 0 | 0 / 0 / 0 | incomplete type edge; textual compile closure controls |
| `CanonicalLayoutContract::artifact` | **HIGH / 10** | 2 / 0 / 3 | artifact-relative path and layout validation |
| contract `namespace_dir` accessors | LOW / 0 each | 0 / 0 / 0 | exact UIDs; textual fixed-field scan controls |
| `CanonicalLayoutContract::artifact_relative_path` | LOW / 6 | 2 / 0 / 2 | `descriptor_for_layout`, `CanonicalLayout::artifact_path` |
| engine `canonical_artifact_relative_path` | LOW / 4 | 1 / 0 / 1 | `CanonicalArtifactKind::relative_path` |
| `default_canonical_layout_contract` | **HIGH / 26** | 7 / 0 / 3 | loaders/layout helpers and `resolve` |
| `validate_canonical_layout_contract` | LOW / 5 | 1 / 0 / 1 | `CanonicalLayout::with_contract` |
| engine `CanonicalLayout` UID | LOW / 0 | 0 / 0 / 0 | incomplete type edge; textual compile closure controls |
| `CanonicalLayout::artifact_path` | **HIGH / 5** | 2 / 0 / 3 | scaffold detection and artifact loading |
| `CanonicalLayout::{namespace_dir,artifact_relative_path}` | LOW / 0 each | 0 / 0 / 0 | exact UIDs; textual fixed-method scan controls |
| `canonical_artifact_kind_sort_key` | **HIGH / 25** | 3 / 1 / 3 | freshness compute/fingerprint/override ordering; process `fingerprint_bytes` |
| `compute_freshness` | **HIGH / 31** | 12 / 1 / 3 | manifest, resolver, ten tests; reaches pipeline handoff emission |
| freshness `fingerprint_bytes` | LOW / 21 | 1 / 0 / 2 | `compute_freshness` |
| `OverrideTarget` UID | LOW / 0 | 0 / 0 / 0 | incomplete enum edge; exact variant scan controls |
| `ArtifactManifest` UID | LOW / 0 | 0 / 0 / 0 | incomplete type edge; textual compile closure controls |
| `ArtifactManifest::from_canonical_artifacts` | **HIGH / 24** | 4 / 1 / 4 | manifest tests, `generate`, resolver; reaches pipeline handoff emission |
| `ArtifactManifest::generate` | **HIGH / 36** | 11 / 1 / 4 | eight interface tests and three pipeline handoff/provenance callers |
| engine `baseline_artifact_validations` | LOW / 3 | 3 / 0 / 1 | engine baseline tests |
| engine `baseline_artifact_validation` | LOW / 2 | 2 / 0 / 1 | engine baseline tests |
| engine `validation_for_descriptor` | LOW / 6 | 2 / 0 / 1 | the two public baseline-validation entry points |
| engine `verdict_for_descriptor` | LOW / 7 | 1 / 0 / 2 | `validation_for_descriptor` |
| engine baseline `canonical_artifact` | LOW / 8 | 2 / 0 / 2 | descriptor validation/verdict |
| `preflight_author_charter` | LOW / 0 | 0 / 0 / 0 | private edge incomplete |
| `validate_authoring_preconditions` | LOW / 1 | 1 / 0 / 1 | Charter preflight |
| `validate_system_root_for_authoring` | LOW / 2 | 1 / 0 / 1 | Charter authoring preconditions |
| compiler `canonical_artifact_identity` | LOW / 2 | 1 / 0 / 1 | Charter preconditions |
| compiler author `canonical_artifact` | LOW / 3 | 1 / 0 / 1 | identity selection |
| `baseline_authoring_eligibility` | LOW / 2 | 1 / 0 / 1 | Charter preconditions |
| compiler `baseline_artifact_validations` | LOW / 0 | 0 / 0 / 0 | no indexed caller |
| compiler `baseline_artifact_validation` | LOW / 3 | 1 / 0 / 1 | authoring eligibility |
| `validate_artifact_markdown` | LOW / 4 | 1 / 0 / 2 | engine descriptor verdict callback |
| compiler `canonical_artifact_relative_path` | LOW / 4 | 1 / 0 / 1 | compiler layout artifact path |
| compiler `CanonicalLayout` UID | LOW / 1 | 1 / 0 / 0 | `RepoLayoutRoot::canonical` |
| `AuthoringArtifactLayout` UID | LOW / 2 | 2 / 0 / 0 | Charter and Project Context authoring layouts |
| compiler `author_or_fill_next_safe_action` | LOW / 4 | 1 / 0 / 1 | baseline blocker builder |
| compiler `required_artifact_blocker` | LOW / 4 | 1 / 0 / 1 | baseline blocker builder |
| compiler `cmp_subject` | LOW / 4 | 1 / 0 / 2 | blocker sorting |
| compiler `subject_kind_priority` | LOW / 3 | 1 / 0 / 2 | `cmp_subject` |
| compiler `canonical_artifact_kind_priority` | LOW / 3 | 1 / 0 / 2 | subject comparison |
| compiler layout `charter`, `project_context`, and canonical-target accessors | LOW / 0 each | 0 / 0 / 0 | incomplete private method edges; exact textual compile closure controls |
| compiler `SubjectRef` UID | LOW / 0 | 0 / 0 / 0 | incomplete enum edge; exact variant scan controls |
| compiler Markdown/shared kind and subject renderers | LOW / 0 each | 0 / 0 / 0 | incomplete private render edges |
| shared `render_packet_source_summary` | LOW / 0 | 0 / 0 / 0 | incomplete private render edge |
| shared `render_packet_section` | LOW / 1 | 1 / 0 / 1 | packet-body renderer |
| both bridge ID constants | LOW / 0 each | 0 / 0 / 0 | textual decision-log consumers only |
| bridge failure and three bridge type UIDs | LOW / 0 each | 0 / 0 / 0 | incomplete type edges; live methods/callers are in the manifest |
| `CharterAuthorityEvidence` | **HIGH / 7** | 1 / 0 / 3 | Charter load; authority evidence must survive type deletion |
| `RenderedArtifactProjection` UID | LOW / 0 | 0 / 0 / 0 | incomplete type edge; live packet consumers are explicit |
| `rendered_projection_for_path` | **HIGH / 10** | 2 / 0 / 4 | `packet_artifact_plans_for`, `present_fixture_sources_for` |
| flow `baseline_artifact_validations` | LOW / 6 | 1 / 0 / 2 | `resolve_with_contract` |
| flow `author_or_fill_next_safe_action` | **HIGH / 12** | 4 / 0 / 4 | blocker/refusal construction |
| flow `required_artifact_blocker` | **HIGH / 10** | 2 / 0 / 3 | blocker construction |
| flow `canonical_artifact_kind_priority` | LOW / 4 | 1 / 0 / 1 | subject comparison |
| `resolve_with_contract` | MEDIUM / 5 | 5 / 0 / 2 | `resolve` plus four non-default-contract tests |
| `BuildPacketResultInput` | LOW / 6 | 1 / 0 / 2 | resolver |
| `build_packet_result` | LOW / 6 | 1 / 0 / 2 | resolver |
| `PacketArtifactPlan` | **HIGH / 7** | 1 / 0 / 3 | packet-plan builder |
| `PacketArtifactInputs` | LOW / 6 | 1 / 0 / 2 | resolver |
| `packet_artifact_plans_for` | LOW / 6 | 1 / 0 / 2 | resolver |
| `included_sources_for` | **HIGH / 7** | 1 / 0 / 3 | packet-result builder |
| `present_fixture_sources_for` | LOW / 3 | 1 / 0 / 2 | fixture context |
| `packet_sections_for` | **HIGH / 7** | 1 / 0 / 3 | packet-result builder |
| `fixture_context_for` | **HIGH / 7** | 1 / 0 / 3 | packet-result builder |
| `compute_refusal` | LOW / 6 | 1 / 0 / 2 | resolver |
| `compute_blockers` | LOW / 6 | 1 / 0 / 2 | resolver |
| `PacketSourceSummary` | **HIGH / 5** | 2 / 0 / 3 | included and fixture source builders |
| `PacketSection` | LOW / 3 | 1 / 0 / 2 | section builder |
| `PacketResult` UID | **HIGH / 7** | 1 / 0 / 3 | packet-result builder |
| `evaluate_budget` | LOW / 0 | 0 / 0 / 0 | incomplete wrapper edge; exact call inventory controls |
| `evaluate_budget_with_effective_bytes` | **HIGH / 35** | 8 / 2 / 4 | resolver plus seven budget callers; reaches intake-document and committed-read processes |
| CLI source/kind/subject renderers | LOW / 0 each | 0 / 0 / 0 | incomplete private render edges |
| `flow_subject_ref_for_rendering` | LOW / 2 | 2 / 0 / 1 | flow refusal/blocker conversion |
| `prepare_flow_output` | LOW / 2 | 2 / 2 / 1 | generate and inspect `run` processes |

The `CanonicalArtifact` CRITICAL result and every HIGH result above were warned
before this selector was frozen. Any implementation-baseline HIGH/CRITICAL
surface must be warned again before editing. Any UNKNOWN, new process, new
module, or materially wider count not explained by the exact test additions
stops P6.

## Required RED baseline

Before any production edit, the implementation session must add or rewrite the
permanent tests first and record these exact failing proofs:

1. flow test
   `descriptor_selected_flow_preserves_packet_contract_without_bridges` in
   `crates/flow/tests/resolver_core.rs`, run alone with `--exact`; it must fail
   against the live bridge/fixed-array implementation while asserting admitted
   descriptor order, exact source/render fingerprints, committed Charter
   evidence without a bridge ID, Project Context legacy-Markdown
   non-influence, and advisory Environment Context behavior;
2. engine test
   `descriptor_selected_collection_retains_exact_source_bytes` in
   `crates/engine/tests/canonical_artifacts_ingest.rs`, run alone with
   `--exact`; it must fail until collection membership/path/order and retained
   bytes come from admitted descriptors; and
3. exact recursive absence scans for the two bridge IDs, four bridge type
   names, `rendered_projection_for_path`, fixed-sibling loader names, fixed
   order constant, and both path switches; those scans are expected RED at the
   baseline.

Do not manufacture RED by weakening fixtures or changing released profile,
schema, definition, or renderer bytes. If either new executable test passes
before production changes, stop: it is not discriminating the selected seam.

## Preserved behavior and proof obligations

P6 GREEN must preserve all of the following over the admitted collection:

- deterministic required/optional/indeterminate plus instance-ID order;
- exact source bytes, byte lengths, source fingerprints, rendered bytes,
  rendered fingerprints, and media type without conflating source and view;
- Charter selected YAML, committed-current authority, promotion ref,
  lifecycle-transition ref, refusal behavior, and every HCM-2.2 negative;
- Project Context selected YAML, rendering, path, source/render fingerprints,
  ABA/no-follow behavior, and zero legacy Markdown influence;
- Environment Context advisory-only inclusion, non-blocking missing/invalid
  omission, omission notes, and rendered-byte budgeting;
- packet selection, disposition, included-source metadata, section modes and
  bodies, notes, summaries, fixture basis/lineage, ready action, blockers,
  refusals, decision-log meaning, and C03/C04 fingerprints;
- budget thresholds, byte-domain choice, target order, summarize/exclude/refuse
  behavior, and next-safe-action semantics;
- system-root/refusal precedence and nested-repository behavior;
- engine manifest/freshness determinism and pipeline handoff/provenance
  consumers reached by the HIGH impact surfaces; and
- HCM-2.3 custom-kind proof that no product enum/generated command/path
  inference is required.

The implementation wall is every target in the test manifest, strict Clippy
for the four affected crates and selected tests, `cargo fmt --all -- --check`,
the HCM-2.1/HCM-2.2/HCM-2.3 preservation targets, exact absence scans, and the
full proportional workspace wall required by the SPEC. P7 alone may claim the
Phase 2 exit wall.

## Atomic indivisibility and ancillary allowance

P6 is one atomic packet because a partial result would leave competing
authority:

- deleting one bridge while another still supplies packet identity/rendering
  preserves mixed selected/fixed flow;
- changing collection membership without deleting fixed order/path/layout
  selectors leaves two authorities for the same artifact;
- deleting fixed loaders before retained-byte and renderer-ref selection is
  green loses required behavior; and
- changing packet/manifest identity without the complete compile propagation
  makes fingerprints, budgets, blockers, refusals, and renderers disagree.

Atomicity authorizes only the exact manifest above. It does not authorize
cleanup in a listed file that is unrelated to the bridge/fixed-family compile
closure.

No ancillary production path or public symbol is allowed. Existing listed
types may be reshaped, and existing private helpers may be rewritten or
deleted. If implementation requires a newly named private helper, it is
allowed only in `canonical_artifacts.rs` or `resolver.rs`, only to implement
safe descriptor collection or fixed-renderer dispatch, and only after its
exact name, callers, impact, and proof are appended to the implementation
dispatch before the edit. More than two such helpers total, any new public
item, any new type, or any helper outside those two files is a stop and needs a
new selector review.

## Stop conditions and non-goals

Stop P6 and return to the operator if implementation requires:

- a path, symbol, test, fixture, schema, definition, profile, renderer
  definition, Cargo file, dependency, module, command, or public API outside
  this selector;
- an unresolved impact, a materially wider HIGH/CRITICAL result, or a change
  to an affected pipeline/intake/read process contract;
- a fallback/dual read, compatibility layer, fixed or inferred filename,
  legacy Markdown authority, or fixed enum/path/layout/order authority in the
  normal flow;
- generic configured renderer execution, Work Specification/Decision/Risk
  renderer implementation, capitalized Projection, Resolution, task-gate
  runtime, or another family conversion;
- changed Charter authority or Project/Environment Context policy;
- a fixture edit, new dependency, Cargo/version change, unsafe-code policy
  change, or weakened/deleted durable negative; or
- P7, Phase 2 exit, HCM-3.x, automatic continuation, push, or release work.

P6 implementation may begin in a fresh session only from a clean descendant of
the reviewed selector commit, after replaying this selector/dispatch subject,
replaying the prerequisite commits and GREEN baselines, refreshing every
planned symbol impact at that baseline, warning on every HIGH/CRITICAL result,
and reproducing the required RED tests before production edits.

## Review requirement

The exact v1.4 dispatch beside this selector must bind the final subject
manifest and aggregate fingerprint. Commit is forbidden unless the mandatory
fresh built-in read-only review returns CLEAN with no unresolved P1/P2; the
dispatch is the immutable review-lineage record for this selector.
