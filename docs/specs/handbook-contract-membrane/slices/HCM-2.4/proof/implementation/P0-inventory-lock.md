# HCM-2.4 P0 inventory lock

Status: P0 acceptance record; effective only when
`20260726T165827Z--HCM-2-4--p0-evidence-review` returns CLEAN on the exact
manifested subject

Baseline: `0b177ef9415e942cde763fcb086cec546a9a24f3`

Branch: `feat/handbook-contract-membrane`

Parent orchestration:
`20260726T161851Z--HCM-2-4--implementation-orchestration`

Selected handoff:
`20260726T131857Z--HCM-2-4--orchestration--planning-amendment-completed`

## Entry and authority

- `git rev-parse --show-toplevel` resolved this repository.
- The selected v1.3 handoff record and ledger entry have exact parity.
- Its source handoff is
  `20260726T050447Z--HCM-2-4--orchestration--planning-completed`; it supersedes
  no immutable record and authorizes a fresh explicit implementation selection
  to enter read-only P0.
- The reviewed planning commit `ea6d733` is an ancestor of the baseline.
- The branch began clean and synchronized with its upstream at 0/0.
- Changes between `ea6d733` and the baseline are orchestration-control
  hardening only; no HCM-2.4 runtime, definition, template, fixture, or test
  source drift invalidates the packet.
- No unrelated or overlapping work was present.

Validation:

```text
uv run --with jsonschema==4.25.1 python docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py
uv run --with jsonschema==4.25.1 python docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-v1-admission
uv run --with jsonschema==4.25.1 python docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-orchestration-contract
```

Result: 57 records and 348 current dispatches validated after the new P0 dispatches;
both self-test modes passed. The exact frozen 35-record v1.2 corpus remains
unchanged.

## Selective context capsule

SLICE / OBJECTIVE:

HCM-2.4 only. Publish complete first-party intake/renderer support, move the
remaining Environment Context and Work Specification authorities to canonical
YAML, prove Decision/Risk support through explicit repository profiles, remove
the two temporary flow bridges and fixed selectors, and close Phase 2.

ACTIVE PACKET:

P0 inventory lock. P1A is the first runtime packet and remains unopened until
this record's exact review returns CLEAN.

DEPENDENCY / AUTHORIZATION PROOF:

HCM-2.1, HCM-2.2, and HCM-2.3 live source/test/control evidence is present.
The user explicitly selected the exact planning-amendment handoff. The
selected handoff is valid resume context and does not override slice
architecture.

SELECTED HANDOFF / VALIDITY:

The exact handoff named above is current-schema v1.3, completed planning,
source-linked, record/index-parity clean, and semantically consistent with the
selected slice. Snapshot Memory refs are explicitly unavailable.

ACTIVE RESOLUTION ENVELOPE:

Pre-Phase-3 execution. Fixed deterministic first-party renderers only;
`resolution_input: null`; no capitalized Projection or Context Resolution work.

GROUNDING SNAPSHOT / START DELTA:

Snapshot Memory capture is unavailable. The start delta is the live source
comparison from reviewed planning commit `ea6d733` to baseline `0b177ef`, with
no slice runtime drift.

TARGET AUTHORITY BOUNDARY:

HCM-2.4 `SPEC.md`, plan, todo, the HCM control pack, exact selected handoff,
then live source/tests. Sibling artifacts are preservation evidence only.

CURRENT REPO-TRUTH STATUS:

GitNexus was force-rebuilt at the baseline after an incremental index
corruption was detected. The current index has 18,645 nodes, 40,334 edges,
438 clusters, and 300 flows. Full-text search is unavailable because the local
FTS extension cannot load; exact UID context, Cypher lookup, source inspection,
and upstream impact are current and usable.

MUST-READ PACK SECTIONS:

- exact HCM-2.4 rows in `01-target-architecture.md` through
  `06-proof-and-regression-ledger.md`;
- `08-handoff-ledger-and-escalation-protocol.md`; and
- `09-review-finding-inventory.md`.

LIVE SOURCE / TESTS / PRECEDENT:

The six shipped kind/schema families, shipped-root 1.0/1.1, Charter
intake/renderer, HCM-2.1 Project Context, HCM-2.2 Charter, HCM-2.3 generic
artifact runtime, and the exact packet tests in
`contracts/implementation-packet-path-manifest-v1.0.md`.

SIBLING SEAMS IN CONTEXT:

HCM-2.1 Project Context, HCM-2.2 Charter, and HCM-2.3 registry brief are
preservation-only. P6 may rewrite only their fixed-bridge call sites while
retaining behavior.

ALLOWED AREAS:

Only the exact packet paths in
`contracts/implementation-packet-path-manifest-v1.0.md`, opened in P0-P7 order.

EXPLICIT NON-GOALS:

HCM-3.x, generic or Resolution-aware Projections, public API/transport/SDK
expansion, dependencies/Cargo/package/release changes, unsafe-policy changes,
dynamic command/path inference, migration/dual read/fallback, shipped-root
expansion, Charter authority reinterpretation, or registry-brief changes.

APPLICABLE CONTRACTS / PROOF GATES:

`PG-KIND-01`, `PG-ARTIFACT-01`, `PG-YAML-02`,
`BR-HCM-2-PILOT-FLOW-01`, `BR-HCM-2-CHARTER-FLOW-01`, and the exact Phase 2
exit rows.

REQUIRED SKILL CHAIN:

using-agent-skills fallback; context engineering; spec-driven development;
planning/task breakdown; documentation/ADRs; incremental implementation; TDD;
debugging on failures; independent code review; git workflow/closeout.

KNOWN CORRECTIONS OR CONFLICTS:

P0 repaired three bounded planning defects before runtime authority opened:

1. P2 now names every Environment Context legacy-authority consumer and direct
   manifest proof.
2. P1A now names direct kind/instance admission and schema/profile tests.
3. P6 now names the complete shared-type/fixed-loader compile closure,
   including the two HCM-2.1 loader calls.

P0 also resolved the circular record rule by permitting only evidence-backed
slice-local P0 control records/coupled contradiction repairs, each subject to
fresh review before acceptance.

KNOWN P3/P4 ADVISORIES:

None intersects the selected subject.

MAXIMUM PERMITTED CLASSIFICATION / PROOF CHANGE:

At most the earned Phase 2 gate/bridge changes in P7. No runtime seam can be
promoted beyond real-path proof.

EXIT PROOF:

Exact immutable fingerprints, path manifest, impacts, baseline tests,
schema-valid review dispatches, findings remediation, and different-fresh
closure results are recorded below. P0 opens P1A only after the evidence-review
dispatch is CLEAN.

STOP CONDITIONS:

Any unresolved UNKNOWN, materially wider HIGH/CRITICAL scope, missing
authority, public/dependency/package change, sibling semantic widening,
unreviewable packet, failed preservation wall, or mandatory delegation failure.

## Immutable baseline fingerprint replay

These SHA-256 values bind the existing released bytes. Definition-internal
fingerprints remain the semantic identity; raw hashes prove file immutability.

| Path | SHA-256 |
| --- | --- |
| `crates/engine/definitions/artifact-kinds/handbook.artifact-kind.project-authority/1.1.0.yaml` | `cdd03d8bb6a806e64b740b1793cc05ae9acf3369cf787dcb7255cccb2e1e8a09` |
| `crates/engine/definitions/artifact-kinds/handbook.artifact-kind.project-context/1.0.0.yaml` | `fa69328ee78dd0fb3532df56bcaf36938249ea2e06418071f636ca2bd02b1ddd` |
| `crates/engine/definitions/artifact-kinds/handbook.artifact-kind.environment-context/1.0.0.yaml` | `bff4e8dcedd4b1b95ea4cc690ba9681ac948aed3a12e8224fa220ab83cac15d0` |
| `crates/engine/definitions/artifact-kinds/handbook.artifact-kind.work-specification/1.0.0.yaml` | `a02908172ee2cc4254bf41685e4f551d60e98f0e30e6f03c0764051aa228d2e6` |
| `crates/engine/definitions/artifact-kinds/handbook.artifact-kind.decision-record/1.0.0.yaml` | `6e39319c6099b9eca13fcd98c2ec9cd5bf0d1d5fc745a1250d7983cc1f4a2a79` |
| `crates/engine/definitions/artifact-kinds/handbook.artifact-kind.risk-record/1.0.0.yaml` | `49d1fe1ba4ebae4eb7d3466bd5815cfe96501a8397263f25b53ee5eee485a4d1` |
| `crates/engine/definitions/schemas/handbook.schemas.artifacts.project-authority/1.1.0.entry.yaml` | `4c21f663032dd4741eaa71828a6b4a8f10c987277a8eb0a5aa62de8be7e66866` |
| `crates/engine/definitions/schemas/handbook.schemas.artifacts.project-authority/1.1.0.schema.json` | `7afabe1e26ea2b56752ca7c23b2d1bfeb0beef89b15dc54f898785d5cc337a33` |
| `crates/engine/definitions/schemas/handbook.schemas.artifacts.project-context/1.0.0.entry.yaml` | `9b81f997ccb279fe79d2e5818bfef082ff844183ebea9789b7d3641850c153ec` |
| `crates/engine/definitions/schemas/handbook.schemas.artifacts.project-context/1.0.0.schema.json` | `033943e504d56d50368e6e50ea7a113baa11eaa5ee52e4989dca8d5c4bea1961` |
| `crates/engine/definitions/schemas/handbook.schemas.artifacts.environment-context/1.0.0.entry.yaml` | `e4656ab69c2037ee69824f3c16c778d49d18e699f35931af10de6149cefabc18` |
| `crates/engine/definitions/schemas/handbook.schemas.artifacts.environment-context/1.0.0.schema.json` | `43499e8530e72db3809034edc1874994d1ed1f57e3c6afae11c6534d7f6206de` |
| `crates/engine/definitions/schemas/handbook.schemas.artifacts.work-specification/1.0.0.entry.yaml` | `673ed859ec8e6642938b99e1eddc94f6b0db43f6197355e0e7cbab6961d89292` |
| `crates/engine/definitions/schemas/handbook.schemas.artifacts.work-specification/1.0.0.schema.json` | `c4c6f489ff20c022538ae2e3512cf62f8559d8d9409944ab13e8f138cb2d5985` |
| `crates/engine/definitions/schemas/handbook.schemas.artifacts.decision-record/1.0.0.entry.yaml` | `6e9813a882459df78c5ebc3e4a418f7d7c94bef3745661487dacb4744097feab` |
| `crates/engine/definitions/schemas/handbook.schemas.artifacts.decision-record/1.0.0.schema.json` | `097b4897abce697634429680ec3ee26b780048c039ded5b7cc214eb91f7d2a8e` |
| `crates/engine/definitions/schemas/handbook.schemas.artifacts.risk-record/1.0.0.entry.yaml` | `00348de66e6c5dd679ab96ce4145177c12667ac0d8e888d148c5ba3c6944ffb2` |
| `crates/engine/definitions/schemas/handbook.schemas.artifacts.risk-record/1.0.0.schema.json` | `df90eec6f85652603c19e56c92870b77599b59a7011110cab91b443d6eb2b01c` |
| `crates/engine/definitions/profiles/handbook.profile.shipped-root/1.0.0.yaml` | `5439176ee37aaa8163d04b99fb0231adaf77f5bb43c4cb59f70251ad45ec4303` |
| `crates/engine/definitions/profiles/handbook.profile.shipped-root/1.1.0.yaml` | `56f92ee105319b609fe21bf299770cd118df121f23e9fcc8c83ebd4b64d66557` |
| `crates/engine/definitions/intakes/handbook.intake.charter/1.0.0.yaml` | `7ada384e4ff8e7fc627dee47e92e24a5806690688beb2e10747bf35d1cda0f67` |
| `crates/engine/definitions/renderers/handbook.renderer.charter-review-markdown/1.0.0.yaml` | `3c8c6ebe82f8770bb8a7023dd22ff3538e68dbd025c1896bc8b2b053093865fc` |

Released semantic identities:

- shipped-root 1.1 profile fingerprint:
  `sha256:6a7b41befa77b999b9ee20f513636051726a8401a81bf2f369501e8f3dd4fa74`;
- Project Authority 1.1 definition fingerprint:
  `sha256:3b3d0b353d9c45c20781c3f4b79e30cc27847fb168d1860115f2f941b8bbef0e`;
- stable-role registry:
  `sha256:0c85b1b53786e7980c4fd0d7975cd9cde1a3eae2bc8daceb23be1a1731263029`.

The full Project Authority descriptor is exactly the released shipped-root 1.1
row named in `SPEC.md`; P1A's new 1.2 row must match every field.

## Exact packet path lock

The exact maximum path surface is:

`docs/specs/handbook-contract-membrane/slices/HCM-2.4/contracts/implementation-packet-path-manifest-v1.0.md`

Reviewed subject fingerprint:
`sha256:73815f5b699360724f2c2c9698c313bef6502fdbe255fbbec9e5f915697e2c4c`.

All currently existing paths replayed. Every absent path is explicitly grouped
as planned-new. P6's type-removal compile closure is atomic and branch-limited;
P7 has eight editable and eleven read-only closeout paths with no overlap.

## Exact impact ledger

Every result below is exact at the baseline and includes tests to depth 3.
Graph-zero type/method results are explicitly treated as incomplete where live
source proves callers.

### P1A

| Symbol | Risk / impacted | Boundary |
| --- | --- | --- |
| `AuthoredArtifactKindDefinition.validate_later_owned_dependencies#1` | LOW / 1 | exact first-party renderer admission only |
| `ArtifactInstanceRegistry.resolve#3` | CRITICAL / 149; 19 direct; 8 processes; 10 modules | exact five frozen descriptors; all other refusals retained |
| `profile_builtins::definition` | LOW / 0 graph edges | dynamic-source edge is incomplete; full profile closure required |

### P1B / P1C

| Symbol | Risk / impacted | Boundary |
| --- | --- | --- |
| `CharterDefinitionRegistry.validate_selected_decisions#1` | LOW / 0 graph edges | six live callers plus sole planned seventh intake caller; eighth is RED |
| `evaluate_charter_intake` | HIGH / 42 | compatibility validation; frozen record identity |
| `validate_candidate_currentness` | HIGH / 17 | released Charter pair only |
| `validate_candidate_contract` | LOW / 2 | frozen candidate identity |
| `CharterPromotionWorkflowServiceV1.promote_at#2` | LOW / 1 | exact registry validation; frozen record pair |
| lifecycle `build_result` | LOW / 1 | frozen result identity |
| lifecycle `validate_definition_authority` | LOW / 2 | registry boundary only |
| `CharterAuthorityTransactionServiceV1.preflight#1` | HIGH / 27 | frozen promotion/candidate identity |
| compiler `doctor_report_from_inspection` | HIGH / 12; 2 direct; 1 process; 3 modules | read/proof-only sixth caller |
| `shipped_profile_request` | CRITICAL / 155; 7 processes; 10 modules | exact source list and root 1.2 ref only |
| `resolve_shipped_profile_decisions` | CRITICAL / 219; 74 direct; 8 processes; 13 modules | read/proof-only; immediate registry validation |

No-edit anchors:

- `validate_candidate_v13`: CRITICAL / 43; 14 processes.
- `validate_promotion_intent_v12`: HIGH / 18; 3 processes.

### P2

| Symbol | Risk / impacted |
| --- | --- |
| engine structured-input validation | LOW / 3 |
| engine Markdown renderer | LOW / 1 |
| engine structured-input parser | LOW / 0 |
| engine Markdown validator | LOW / 4 |
| compiler `author_environment_inventory_from_input` | MEDIUM / 8; 1 process |
| compiler input preflight | MEDIUM / 15; 1 process |
| shell authoring preconditions | LOW / 1; 1 process |
| shell canonical writer | LOW / 9; 1 process |
| CLI author command | LOW / 1; 1 process |
| CLI success renderer | LOW / 0 |
| template-library environment selection | LOW / 4 |
| fixed-sibling loader with contract | LOW / 10; 1 process |
| engine baseline validations | LOW / 3 |
| flow artifact validator | LOW / 0 |
| flow `resolve_with_contract` | MEDIUM / 5 |
| flow baseline validations | LOW / 6 |
| `AuthorCommand` enum | LOW / 0 graph edges; CLI help tests are authoritative |

### P3

| Symbol | Risk / impacted | Boundary |
| --- | --- | --- |
| `build_capture_plan` | MEDIUM / 97 | exact Stage 10 descriptor-selected target |
| `apply_capture_plan` | HIGH / 55 | capture/cache/rollback wall |
| `build_stage_10_capture_provenance_for_apply` | HIGH / 8; 1 process | canonical YAML provenance |
| `build_stage_10_feature_spec_capture_provenance` | CRITICAL / 14; 2 processes; 5 modules | capture and handoff preservation |
| `validate_stage_10_feature_spec_capture_provenance_match` | HIGH / 10; 1 process | exact canonical-source replay |
| handoff provenance loader | HIGH / 10; 1 process | exact canonical-source substitution |
| handoff emit with storage layout | LOW / 12 current graph | public bundle format frozen |
| handoff validate with storage layout | LOW / 1 current graph | public bundle format frozen |
| `derive_feature_id` | HIGH / 10; 1 process; 3 modules | preserve external identity |

The planning record conservatively classified handoff emit/validate as CRITICAL.
The clean force-rebuilt graph reports the lower current results above. P3 keeps
the conservative isolation and full capture/handoff proof wall.

### P4 / P5

No production symbol edit. New exact fixtures/tests consume unchanged HCM-2.3
generic runtime; any required runtime change stops the packet.

### P6

| Symbol | Risk / impacted | Boundary |
| --- | --- | --- |
| `rendered_projection_for_path` | HIGH / 10 | delete fixed path selection |
| `resolve_with_contract` | MEDIUM / 5 | signature/public behavior preserved |
| `baseline_artifact_validations` | LOW / 6 | selected-artifact validation first |
| `packet_artifact_plans_for` | LOW / 6 | order/disposition preserved |
| `present_fixture_sources_for` | LOW / 3 | identity/order preserved |
| compiler `canonical_artifact_kind_priority` | LOW / 3 | delete fixed ordering only |
| `CanonicalArtifactKind` | LOW / 0 graph edges | graph incomplete; exact textual compile closure is authoritative |
| `CanonicalLayoutContract` | LOW / 0 graph edges | graph incomplete; exact textual compile closure is authoritative |

## Baseline proof

Focused shipped-definition/profile wall:

```text
cargo test -p handbook-engine --test hcm_1_2_selected_kinds --test hcm_1_2_unselected_kinds --test hcm_1_4_profile_decisions --test hcm_1_4_profile_inspection --test hcm_2_2_definition_profile
```

Result: 11 passed, 0 failed.

Direct P1A guard/schema/profile wall:

```text
cargo test -p handbook-engine --test artifact_instances --test artifact_kind_registry --test profile_artifact_schemas --test profile_context_schemas --test profile_selection --test profile_work_decision_schemas --test profile_risk_schema
```

Result: 43 passed, 0 failed.

Manifest interface baseline:

```text
cargo test -p handbook-engine --test artifact_manifest_interface
cargo test -p handbook-compiler --test artifact_manifest_interface
```

Result: 10 passed, 0 failed.

HCM-2.1 fixed-loader preservation:

```text
cargo test -p handbook-engine --test hcm_2_1_project_context
```

Result: 12 passed, 0 failed. The only test-side fixed-loader calls are lines
591 and 609 in that exact target.

The first focused Cargo invocation was intentionally attempted with a
one-second shell timeout and was killed before Cargo returned a result. The
unchanged command passed under a build-aware timeout; there is no semantic test
failure.

`git diff --check` passes after each P0 remediation.

## P0 correction and review lineage

| Run | Built-in agent | Final status | Verdict |
| --- | --- | --- | --- |
| `20260726T162140Z--HCM-2-4--p0-selector-repair-review` | `/root/hcm_2_4_p0_selector_review` | `completed_read_only` | findings: one P2 |
| `20260726T162911Z--HCM-2-4--p0-selector-repair-closure-review` | `/root/hcm_2_4_p0_selector_closure` | `completed` | CLEAN |
| `20260726T164140Z--HCM-2-4--p0-path-manifest-review` | `/root/hcm_2_4_p0_manifest_review` | `completed` | findings: three P2 |
| `20260726T164958Z--HCM-2-4--p0-path-manifest-closure-review` | `/root/hcm_2_4_p0_manifest_closure` | `completed` | CLEAN |

Closed findings:

- `p0-selector-repair-discovery-P2-1`;
- `p0-path-manifest-discovery-P2-1`;
- `p0-path-manifest-discovery-P2-2`; and
- `p0-path-manifest-discovery-P2-3`.

No P3/P4 advisory intersects P0.

## P0 exit

P0 passes only when the exact evidence review named in the status returns
CLEAN. At that point:

- no production, test, definition, template, or fixture edit has occurred;
- P1A may open at this exact baseline and path contract;
- every code symbol is impact-analyzed again immediately before its packet
  edit, as required by `AGENTS.md`; and
- later packets stop on any path outside the manifest or any materially wider
  impact.
