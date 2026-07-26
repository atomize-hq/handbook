# HCM-2.4 implementation plan

Status: reviewed planning input only; do not execute without a separate user
selection.

## Dependency graph

```text
P0 baseline/inventory
  -> P1A catalog publication/admission
       -> P1B shipped-root adoption
       -> P2 environment context --------\
       -> P3 work specification ----------+-> P6 aggregate flow cleanup -> P7 exit proof
       -> P4 decision record -------------+
       -> P5 risk record -----------------/
```

P2–P5 may be prepared independently after P1B but must land serially when shared registry,
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

## P0 — Baseline and inventory lock

Entry: separately selected implementation, reviewed planning handoff, clean
synchronized branch.

This entry authorizes read-only P0 discovery only. It does not authorize any
production, test, definition, template, or documentation edit. Editing authority
starts only after P0 freezes and passes the exact live manifest/impact gate.

Work:

- replay every existing kind/schema/profile/renderer/intake fingerprint;
- enumerate exact files and symbols for P1A–P7 from the SPEC selector ceilings;
- capture the three root descriptors and every existing fixed
  enum/path/bridge/helper/test selector;
- resolve the current GitNexus UID and impact for every intended symbol edit;
- freeze HCM-2.1, HCM-2.2, HCM-2.3, flow budget, pipeline handoff, and
  package/archive preservation commands.

Acceptance:

- no undecided family, source, view, deletion, or proof row;
- exact file manifests replace every planning glob;
- HIGH/CRITICAL and UNKNOWN dispositions are explicit; and
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
- leave loader signatures and public APIs unchanged.

GREEN:

- exact fingerprint and duplicate-safe validation passes;
- all five kinds resolve their first-party intake and renderer;
- the root profile successor has exactly three instances;
- inline/unit admission proof accepts exactly the five frozen non-Charter
  descriptor rows and rejects every field mismatch before P3 exercises the Work
  Specification real repository path;
- no public API, Cargo, package metadata, external dependency, command, or
  Projection change.

Stop if definition admission cannot remain exact-ref, fail-closed, and
package-owned without a forbidden surface change.

## P1B — Shipped-root successor adoption

Entry: P1A accepted; CRITICAL impact warning acknowledged.

RED: the built-in request still selects `handbook.profile.shipped-root@1.1.0`
and omits all P1A successor sources.

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
- replay all 51 affected flows and the exact HCM-2.2/HCM-2.3 preservation wall.

GREEN:

- exactly three root descriptors resolve with identical Charter authority and
  condition semantics;
- neither Project Context nor Environment Context has a null intake or empty,
  extra, or mismatched renderer selection;
- Work/Decision/Risk remain source-available but unselected;
- no outcome changes outside the intended Project Context/Environment support
  refs; and
- GitNexus change detection reports only the declared request/admission/source
  closure.

## P2 — Environment Context vertical

Entry: P1B accepted; Environment Context impact UIDs resolved.

RED:

- the author command writes Environment Inventory Markdown;
- changing the legacy Markdown can influence flow/baseline behavior; and
- the selected conditional YAML is not the sole mutation/read authority.

Work:

- parse structured input into the exact Environment Context canonical schema;
- validate and atomically write `.handbook/project/environment.yaml` only when
  the selected descriptor condition applies;
- route CLI, setup, doctor, compiler, manifest, and flow reads to retained YAML;
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

Entry: P1B accepted; exact pipeline UIDs resolved; CRITICAL warning acknowledged
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

Entry: P1B accepted.

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

Entry: P1B accepted.

Select `example.profile.hcm-2-4-risk-record@1.0.0` from
`crates/engine/tests/fixtures/hcm_2_4_risk_record/.handbook/definitions/profiles/risk-record-root-1.0.0.yaml`
and use only the exact `risk_record` descriptor at
`.handbook/records/risk.yaml`, with null role, exact intake/singleton renderer,
`always` requiredness, and empty later-owned fields. Repeat P4 for its vectors, canonical bytes,
golden, and evidence. Shared generic code is permitted; merged review evidence
is not. A combined P4/P5 commit requires a written atomicity rationale accepted
before edits by a fresh reviewer.

## P6 — Aggregate flow and fixed-selector deletion

Entry: P1A–P5 individually committed and green; no bridge deletion has occurred.

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
