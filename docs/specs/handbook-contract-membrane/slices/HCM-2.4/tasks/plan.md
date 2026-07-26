# HCM-2.4 implementation plan

Status: reviewed planning input only; do not execute without a separate user
selection.

## Dependency graph

```text
P0 baseline/inventory
  -> P1A catalog publication/admission
       -> P1B Charter compatibility foundation
            -> P1C shipped-root adoption
                 -> P2 environment context --------\
                 -> P3 work specification ----------+-> P6 aggregate flow cleanup -> P7 exit proof
                 -> P4 decision record -------------+
                 -> P5 risk record -----------------/
```

P2–P5 may be prepared independently after P1C but must land serially when shared registry,
fixture, proof, or generated inventory files overlap. P6 cannot start until all
four are committed, clean, and independently green.

## Standing rules

For each packet:

1. verify branch, clean baseline, upstream 0/0, and exact intended paths;
2. refresh GitNexus if stale, resolve every edited symbol by UID/context, run
   upstream impact, and report HIGH/CRITICAL before edits;
3. establish a focused failing proof for the selected contract;
4. implement the smallest exact selector set;
5. run focused proof, affected-crate proof, then packet proof wall;
6. run GitNexus change detection and compare affected symbols/flows with the
   packet declaration;
7. freeze the packet fingerprint and obtain its bounded review before landing;
8. stop on scope, authority, public API, dependency, Cargo, unsafe, package,
   HCM-3.x, or risk-ceiling expansion.

UNKNOWN impact, a legacy fallback, or an unselected dynamic path is a RED stop,
not an invitation to improvise.

For P1B, the exact production process/module counts in SPEC are ceilings. Live
source has six existing callers, including compiler
`doctor_report_from_inspection`. Exactly one seventh
`validate_selected_decisions` production caller from
`evaluate_charter_intake` is planned so public intake fails closed. New
manifested test callers are also allowed. An eighth production caller, another
process/module/authority class/public surface, or an unexplained larger
HIGH/CRITICAL result is a parent stop.

## P0 — Baseline and inventory lock

Entry: separately selected implementation, reviewed planning handoff, clean
synchronized branch.

This entry authorizes read-only P0 discovery only. It does not authorize any
production, test, definition, template, or documentation edit. Editing authority
starts only after P0 freezes and passes the exact live manifest/impact gate.

Work:

- replay every existing kind/schema/profile/renderer/intake fingerprint;
- enumerate exact files and symbols for P1A–P7 from the SPEC selector ceilings;
- capture both exact shipped-root definitions, the three root descriptors, the
  complete shipped-root-to-Charter registry path, every Charter profile-identity
  producer/currentness consumer, and every existing fixed
  enum/path/bridge/helper/test selector;
- resolve the current GitNexus UID and impact for every intended symbol edit;
- freeze HCM-2.1, HCM-2.2, HCM-2.3, flow budget, pipeline handoff, and
  package/archive preservation commands;
- manifest every `hcm_2_2_*` engine integration target, engine lib/all-features
  matrices, compiler/CLI Charter targets, and the two no-edit
  `validate_candidate_v13` / `validate_promotion_intent_v12` anchors; and
- add the exact P2 CLI main/help/snapshot files and named tests from SPEC.

Acceptance:

- no undecided family, source, view, deletion, or proof row;
- exact file manifests replace every planning glob;
- HIGH/CRITICAL and UNKNOWN dispositions are explicit; and
- all compatibility symbols match the exact UIDs/ceilings in SPEC, inventorying
  compiler doctor as the sixth existing caller, allowing only the planned
  seventh intake caller, and permitting no other new production
  caller/process/module or authority class; and
- no production/test edit has occurred.

## P1A — Catalog publication and admission closure

Entry: P0 accepted.

RED:

- each of Project Context, Environment Context, Work Specification, Decision
  Record, and Risk Record lacks a published intake and renderer reference; and
- the current root profile does not select supported successors for Project
  Context and Environment Context.

Work:

- add the exact versioned kind/intake/renderer/profile definitions frozen in
  SPEC;
- keep all released files immutable;
- make intake coverage exhaustive against each canonical schema and explicit
  about unknown/required values;
- make each renderer fixed, deterministic, schema-bound, and Resolution-free;
- add exact built-in source mappings for the new definitions;
- replace the Project-Authority-only renderer-ref exception in
  `validate_later_owned_dependencies` with exact admitted first-party renderer
  dependency validation, while continuing to refuse Projection, lifecycle,
  trigger, capability, extension, missing, and mismatched refs;
- replace the corresponding non-Charter descriptor refusal in
  `artifact_instance.rs` only for the five exact Project Context, Environment
  Context, Work Specification, Decision Record, and Risk Record rows frozen in
  SPEC, including kind, intake, singleton renderer, role/path, requiredness,
  and empty later-owned fields; retain lifecycle, Projection, overlay,
  extension, missing, extra, and mismatched dependency refusal;
- leave loader signatures and public APIs unchanged; and
- prove the 1.2 `project_authority` descriptor equals the released 1.1 row in
  every field and freeze the completed 1.2 profile's literal authored
  fingerprint in the immutable HCM-2.4 definition vector.

GREEN:

- exact fingerprint and duplicate-safe validation passes;
- all five kinds resolve their first-party intake and renderer;
- the root profile successor has exactly three instances;
- the literal 1.2 fingerprint is replayable from the complete typed dependency
  closure and no placeholder remains;
- inline/unit admission proof accepts exactly the five frozen non-Charter
  descriptor rows and rejects every field mismatch before P3 exercises the Work
  Specification real repository path;
- no public API, Cargo, package metadata, external dependency, command, or
  Projection change.

Stop if definition admission cannot remain exact-ref, fail-closed, and
package-owned without a forbidden surface change.

## P1B — Versioned Charter compatibility foundation

Entry: P1A accepted; its literal 1.2 fingerprint and exact Project Authority
descriptor are frozen; all HIGH/CRITICAL warnings are acknowledged.

RED:

- exact 1.2 decisions are rejected by
  `CharterDefinitionRegistry::validate_selected_decisions`;
- changing the shipped request alone would fail immediately in
  `resolve_shipped_profile_decisions`; and
- Charter record producers currently copy the generic selected profile identity,
  so accepting 1.2 without a membrane would rewrite protected HCM-2.2 records.

Work:

- keep the shipped request on 1.1 for the entire packet;
- in the existing `validate_selected_decisions` symbol, admit only the exact
  1.1 ref/fingerprint tuple and the exact P1A-authored 1.2 tuple;
- compare every Project Authority descriptor field and subordinate definition
  from SPEC for both tuples, rejecting all other tuple/version/field shapes;
- leave `validate_selected_profile` 1.1-only and unchanged;
- make the existing HCM-2.2 profile ref/fingerprint constants `pub(crate)`
  without changing their values;
- update only `evaluate_charter_intake`,
  `validate_candidate_currentness`, `validate_candidate_contract`,
  `CharterPromotionWorkflowServiceV1::promote_at`, `build_result`,
  `validate_definition_authority`, and
  `CharterAuthorityTransactionServiceV1::preflight` so their Charter records
  and comparisons retain the HCM-2.2 pair after exact registry validation;
- retain compiler `doctor_report_from_inspection` as an unchanged sixth-caller
  read/proof anchor and prove its Charter definition closure remains resolved
  under direct 1.1 and selected 1.2;
- add exact
  `crates/engine/tests/hcm_2_4_charter_profile_compatibility.rs`, including
  `invalid_compatible_profile_decisions_cannot_produce_charter_intake`, so the
  new public intake edge rejects an invalid 1.2 tuple before producing records;
- do not edit candidate/result/promotion-intent validators, schemas, contracts,
  approval semantics, lineage/lifecycle formats, fingerprints, committed
  authority, or public signatures; and
- replay the complete HCM-2.2 wall and all direct HIGH/CRITICAL callers.

GREEN:

- direct 1.1 acceptance and released bytes/fingerprint are unchanged;
- explicit exact 1.2 decisions pass only the versioned membrane;
- every tuple, version, descriptor field, subordinate-definition,
  range/prefix, fallback, and second-read mutation fails closed;
- direct 1.1 and compatible 1.2 decisions produce byte/fingerprint-identical
  HCM-2.2 intake, candidate, lifecycle result, approval, promotion, intent,
  lineage, transaction, recovery/replay, and committed authority;
- every existing HCM-2.2 negative passes unchanged;
- the shipped request still resolves 1.1; and
- GitNexus change detection reports only the exact P1B symbols.

Stop if the 1.2 literal is absent or changes, any Project Authority field
differs, any HCM-2.2 record byte changes, a no-edit validator must change, or the
recorded production process/module ceiling is exceeded or an eighth production
caller appears.

## P1C — Shipped-root successor adoption

Entry: P1B accepted; CRITICAL request/resolver warnings acknowledged.

RED: the built-in request still selects `handbook.profile.shipped-root@1.1.0`
and omits the P1A successor sources.

Work:

- update only `shipped_profile_request`'s selected exact profile and immutable
  schema/kind/intake/renderer source closure;
- require the resolved `project_context` descriptor to select
  `handbook.intake.project-context@1.0.0` and exactly
  `handbook.renderer.project-context-review-markdown@1.0.0`;
- require the resolved `environment_context` descriptor to select
  `handbook.intake.environment-context@1.0.0` and exactly
  `handbook.renderer.environment-context-review-markdown@1.0.0`;
- preserve its three descriptors, condition handling, stable roles,
  vocabularies, Context Resolution sources, and every public signature;
- prove `resolve_shipped_profile_decisions` still immediately applies the
  Charter membrane; and
- replay the complete selected-profile and HCM-2.1–HCM-2.3 preservation wall.

GREEN:

- exactly three root descriptors resolve with identical Charter authority and
  condition semantics;
- generic consumers observe exact shipped-root 1.2;
- every Charter record and validator observes the frozen HCM-2.2 1.1 pair;
- neither Project Context nor Environment Context has a null intake or empty,
  extra, or mismatched renderer selection;
- Work/Decision/Risk remain source-available but unselected;
- no outcome changes outside intended Project Context/Environment support
  refs; and
- GitNexus change detection reports only the declared request/source closure.

## P2 — Environment Context vertical

Entry: P1C accepted; Environment Context impact UIDs resolved.

RED:

- the author command writes Environment Inventory Markdown;
- changing the legacy Markdown can influence flow/baseline behavior; and
- the selected conditional YAML is not the sole mutation/read authority.

Work:

- parse structured input into the exact Environment Context canonical schema;
- validate and atomically write `.handbook/project/environment.yaml` only when
  the selected descriptor condition applies;
- route CLI, setup, doctor, compiler, manifest, and flow reads to retained YAML;
- update `AuthorCommand::EnvironmentInventory` help in `crates/cli/src/main.rs`
  and only the exact inline `author_help_matches_snapshot`, the
  snapshot-consuming `author_environment_inventory_help_matches_snapshot`, and
  the one consumed subcommand snapshot listed in SPEC;
- render human-review Markdown from retained canonical bytes;
- remove Environment Inventory Markdown templates, authority writers, validators,
  fallback reads, and output-tag exceptions once their replacement tests pass.

GREEN:

- condition true/false, malformed/duplicate/oversize, no-follow, atomic refusal,
  retained observation, and ABA/concurrency proofs pass;
- renderer full-byte golden and fingerprint pass;
- legacy Markdown deletion or mutation has no outcome effect;
- Charter and Project Context focused regressions remain green.

## P3 — Work Specification / Stage 10 vertical

Entry: P1C accepted; exact pipeline UIDs resolved; CRITICAL warning acknowledged
within the implementation selection.

RED:

- Stage 10 accepts a completed `FEATURE_SPEC.md` as authoritative capture;
- provenance and handoff decisions fingerprint the Markdown path; and
- the shipped Work Specification kind lacks real-path proof.

Work:

- freeze a schema-valid Stage 10 Work Specification payload at
  `artifacts/work-specification/work-specification.yaml`;
- add and select the exact
  `example.profile.hcm-2-4-work-specification@1.0.0` repository profile under
  `crates/engine/tests/fixtures/hcm_2_4_work_specification/`, extending shipped
  root 1.2 with only the frozen `work_specification`/`delivery_unit` descriptor;
- validate capture against the exact work-specification schema before mutation;
- bind capture provenance, feature identity, trust, and handoff inputs to
  canonical YAML bytes;
- derive the existing fixed Feature Spec Markdown view deterministically;
- reject the Markdown view as capture/authority input;
- preserve fixed command and stage selectors, route-basis freshness, cache,
  rollback, trust-class, and bundle contracts.

GREEN:

- capture preview/apply/cache/rollback and provenance tamper/staleness tests pass;
- Stage 10 resolves the admitted Work Specification descriptor and its exact
  intake/renderer refs before using the path; a same-string hard-coded path
  fails the proof;
- handoff emit and validation replay across every affected process and module;
- canonical YAML produces an exact view golden;
- mutating/deleting the view cannot alter canonical identity or handoff outcome;
- no public format/API change escapes the existing internal fixed surface.

Because handoff emit/validate are CRITICAL, any required public bundle schema or
API change is a stop, not a packet extension.

## P4 — Decision Record support proof

Entry: P1C accepted.

RED: no repository-selected real-path intake/render proof exists.

Work:

- select `example.profile.hcm-2-4-decision-record@1.0.0` from
  `crates/engine/tests/fixtures/hcm_2_4_decision_record/.handbook/definitions/profiles/decision-record-root-1.0.0.yaml`;
- add only the exact `decision_record` descriptor at
  `.handbook/records/decision.yaml`, with null role, exact intake/singleton
  renderer, `always` requiredness, and empty later-owned fields;
- exercise schema-backed intake, generic canonical mutation/read/validate, and
  deterministic renderer through the existing HCM-2.3 generic operation path;
- prove no root instance, generated command, inferred filename, or persistent
  Markdown mirror.

GREEN: exact schema/coverage, generic-operation, renderer golden, safe path,
retained observation, and negative-surface tests pass independently.

## P5 — Risk Record support proof

Entry: P1C accepted.

Select `example.profile.hcm-2-4-risk-record@1.0.0` from
`crates/engine/tests/fixtures/hcm_2_4_risk_record/.handbook/definitions/profiles/risk-record-root-1.0.0.yaml`
and use only the exact `risk_record` descriptor at
`.handbook/records/risk.yaml`, with null role, exact intake/singleton renderer,
`always` requiredness, and empty later-owned fields. Repeat P4 for its vectors, canonical bytes,
golden, and evidence. Shared generic code is permitted; merged review evidence
is not. A combined P4/P5 commit requires a written atomicity rationale accepted
before edits by a fresh reviewer.

## P6 — Aggregate flow and fixed-selector deletion

Entry: P1A–P1C and P2–P5 individually committed and green; no bridge deletion
has occurred.

RED:

- both bridge IDs and bridge types exist;
- fixed enum/order/layout/path selectors decide flow authority; and
- bridge exceptions and legacy fixed-family tests remain.

Work:

- collect flow inputs from admitted profile/instance selection with stable
  ordering and retained bytes;
- select renderers through exact kind/instance refs;
- preserve committed Charter evidence and Project Context rendering;
- preserve packet disposition, budgets, summaries, fixture sources, blockers,
  refusals, and deterministic ordering;
- delete both bridges, `rendered_projection_for_path`, fixed-family/path
  authority, fixed-sibling loaders, legacy exceptions, and bridge-only tests;
- rewrite permanent behavior tests against selected descriptors.

GREEN:

- both bridge IDs and every forbidden selector are absent;
- no Markdown view influences validation, packets, budgets, or fingerprints;
- complete engine/compiler/flow/CLI focused proof passes;
- HCM-2.1–HCM-2.3 preservation proof passes.

P6 is genuinely atomic because partial deletion would leave parallel authority
paths. It is bounded to the existing selected-artifact flow and does not make
custom configured renderers or Projections generic.

## P7 — Phase 2 exit and true stop

Entry: P6 accepted.

Work:

- run the SPEC full proof wall and exact Phase 2 exit map;
- replay direct 1.1 acceptance, exact 1.2 compatibility, every
  Project-Authority-field negative, real shipped 1.2 selection, byte-identical
  HCM-2.2 record identity, and all unchanged HCM-2.2 negative/atomic/recovery
  matrices;
- update only earned bridge/gate/control-pack rows;
- run one complete-subject discovery review or bounded same-fingerprint burst;
- consolidate all P1/P2, remediate once, and obtain a different-fresh
  delta-focused closure review;
- use at most two supplemental causal cycles under the declared allowance;
- commit the reviewed implementation subject;
- create the parent-owned completed implementation handoff, rebuild the ledger,
  validate all handoff modes, and commit the mechanical closeout separately;
- push without force and verify clean origin 0/0.

Exit:

- unresolved P1/P2 is empty;
- P3/P4 is fixed or registered/deduplicated;
- every Phase 2 exit row is proven;
- the branch is clean and synchronized; and
- stop before HCM-3.x or any other implementation.

Planning completion does not authorize this plan. A later implementation
session must explicitly select the new exact completed amended-planning
handoff, and that selection again enters read-only P0 before any edit.
