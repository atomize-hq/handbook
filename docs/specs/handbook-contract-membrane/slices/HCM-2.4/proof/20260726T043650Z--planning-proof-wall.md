# HCM-2.4 planning proof wall

Captured: `2026-07-26T04:36:50Z`

Planning baseline: `2b7ab4e14467800e5f0ecaa19561a1dd5d84ee48`

Status: pre-review planning evidence; no HCM-2.4 implementation was performed.

## Stage A admission

- Starting branch: `feat/handbook-contract-membrane`
- Starting HEAD: `2b56b33941d5bd70acbe6b2c2cf897502d424e1e`
- Authorized amendment: exactly seven Markdown files, 52 insertions and 30
  deletions
- Frozen amendment fingerprint:
  `sha256:82f0d2334f4988c0dfb28fbdd96a66e359269e3d661a8fe984e2a42e8e40e437`
- Different-fresh read-only verdict: CLEAN, no P1/P2/P3/P4
- Handoff validation, v1-admission self-test, orchestration-contract self-test,
  35 changed-document relative links, fences, JSON/Markdown, and
  `git diff --check`: PASS
- GitNexus staged change detection: LOW, eight documentation files, no affected
  execution process
- Landed amendment commit:
  `2b7ab4e14467800e5f0ecaa19561a1dd5d84ee48`
- Origin state after landing: ahead/behind `0/0`, clean

The explicit user instruction continued directly into planning; no synthetic
inter-stage handoff was created.

## Selective authority read

Planning read only the selected HCM-2.4 authority and precedent:

- `AGENTS.md`
- `00-README.md`
- `03-seam-crosswalk.md`
- HCM-2.4 and Phase 2 exit sections of `04-phase-slice-map.md`
- owner, artifact-kind, profile, intake, renderer, and orchestration contracts
  in `05-contracts-schemas-and-gates.md`
- open gates, both Phase 2 bridges, and HCM-2.3 evidence in
  `06-proof-and-regression-ledger.md`
- `07-orchestration-onboarding-prompt.md`
- `08-handoff-ledger-and-escalation-protocol.md`
- `09-review-finding-inventory.md`
- completed HCM-2.3 parent handoff
  `20260725T220053Z--HCM-2-3--orchestration--implementation-completed.json`
- bounded-review calibration closeout
  `20260726T021305Z--HCM-0-8--orchestration--bounded-review-calibration-completed.json`
- HCM-2.1–HCM-2.3 plans/proofs only where they established source/view,
  profile, renderer, registry-brief, or proof-wall precedent.

No indiscriminate historical dispatch load or contract-monolith replay was
used.

## Repository discovery

Source inspection proved:

- the catalog has exactly six shipped first-party kinds;
- only Charter has published intake and renderer definitions;
- the root profile has exactly three descriptors;
- Project Context is canonical YAML but lacks published intake/renderer refs;
- Environment Context has a schema/descriptor but the authoring path still
  writes Environment Inventory Markdown;
- Work Specification has a schema but Stage 10 captures Feature Spec Markdown;
- Decision and Risk have schemas/kinds but no root instances or first-party
  intake/render support;
- flow still has two bridge IDs, fixed bridge types, fixed artifact enum/order,
  fixed layout paths, fixed-sibling loading, and path-selected render logic; and
- HCM-2.3 registry-brief is custom-kind preservation evidence, not another
  first-party family.

The planning subject records exact canonical/view classification, deletion
triggers, independently reviewable packet order, maximum production/test/doc
selectors, Phase 2 exit evidence, review budget, and implementation-entry gates.

## GitNexus freshness and impact

`npx gitnexus analyze` refreshed the index at the Stage A commit. Analyzer
mirror-file rewrites were identified as generated side effects and reversed
exactly before planning; no analyzer mirror is in the subject.

The local FTS extension could not load, so semantic `query` returned no useful
results. Planning used exact source inspection, `context` where available, UID
disambiguation, and upstream `impact`. This degraded query state is explicit;
it does not downgrade UNKNOWN risk.

Fresh results used by the planning packet:

| Target | Result |
| --- | --- |
| `rendered_projection_for_path` | HIGH; 10 impacted including tests, 2 direct, 3 modules |
| `resolve_with_contract` | MEDIUM; 5 direct |
| `baseline_artifact_validations` | LOW; 6 impacted including tests |
| `packet_artifact_plans_for` | LOW; 2 upstream without test expansion |
| `present_fixture_sources_for` | LOW; 4 upstream |
| `emit_pipeline_handoff_bundle_with_storage_layout` | CRITICAL; 259 impacted, 2 direct, 51 processes, 20 modules |
| `validate_pipeline_handoff_bundle_with_storage_layout` | CRITICAL; 669 impacted, 1 direct, 51 processes, 20 modules |
| `derive_feature_id` | CRITICAL; 409 impacted, 1 direct, 51 processes, 20 modules |
| `shipped_profile_request` | CRITICAL; 404 impacted, 1 direct, 51 processes, 20 modules |
| `resolve_shipped_profile_decisions` | CRITICAL; 443 impacted, 39 direct, 51 processes, 20 modules |
| `CanonicalArtifactKind` enum UID | LOW/zero graph edges; known incomplete type-edge result |
| `CanonicalLayoutContract` struct UID | LOW/zero graph edges; known incomplete type-edge result |
| `profile_builtins::definition` | LOW/zero graph edges; known incomplete dynamic-source result |
| registry admission method and large private environment/pipeline helpers | UNKNOWN by name; exact UID/context required before implementation edit |

The SPEC isolates CRITICAL profile adoption and pipeline handoff work, isolates
the HIGH flow deletion, retains source-compatible public shells where required,
and makes unresolved UNKNOWN an implementation stop.

## Planning scope proof

Current planning subject paths are confined to:

- `docs/specs/handbook-contract-membrane/slices/HCM-2.4/SPEC.md`
- `docs/specs/handbook-contract-membrane/slices/HCM-2.4/tasks/plan.md`
- `docs/specs/handbook-contract-membrane/slices/HCM-2.4/tasks/todo.md`
- this proof

There are no Rust, test, schema, definition, template, Cargo, dependency,
package, unsafe-policy, public API, ledger, or handoff-record changes in the
planning subject.

## Static planning checks

- All four HCM-2.4 Markdown files are UTF-8-readable.
- Markdown fences are balanced.
- Relative links resolve.
- The source/view matrix covers six first-party kinds plus registry-brief
  preservation.
- The pre-Phase-3 output table classifies both legacy fixed outputs, both
  overlapping pipeline outputs, and deferred foundation/quality products.
- Both bridge IDs have exact deletion triggers.
- Packet order is P0 → P1A → P1B → P2–P5 → P6 → P7.
- Every HIGH/CRITICAL surface has an isolation and replay requirement.
- Non-goals prohibit HCM-3.x, generic Projections, SDK/transport, remote schema,
  generated commands, dynamic paths, migration/dual read, dependencies, Cargo,
  unsafe, package boundary, and public API changes.
- The review budget uses the landed causal-cascade allowance.
- `git diff --check` is required again after staging and after every review
  remediation.

## Review contract

The complete planning subject receives one bounded discovery review. All
demonstrated P1/P2 is consolidated before one remediation. A different-fresh
reviewer then examines the remediation delta and complete resulting subject.
At most two supplemental causal cycles are available only for P1/P2 directly
caused or unmasked by the preceding remediation. P3/P4 is fixed when clearly
worthwhile or registered/deduplicated without forcing another review.

No review may authorize implementation.
