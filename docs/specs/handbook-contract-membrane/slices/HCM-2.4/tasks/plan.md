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

This entry starts with read-only P0 discovery. It does not authorize any
production, test, definition, or template edit. Once discovery establishes the
exact evidence, P0 may write only the manifested slice-local inventory/proof
records and the smallest coupled SPEC/plan/todo correction needed to resolve a
demonstrated contradiction. Those documentation changes must complete fresh
internal review and any required different-fresh closure before P0 passes.
Runtime editing authority starts only after P0 freezes and passes the exact
live manifest/impact gate.

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
- recompute every intake, renderer, successor-kind, and profile `1.2`
  fingerprint from the uniform exact typed dependency closure established by
  HCM-2.2, including subordinate kind/schema/renderer/intake/lifecycle
  fingerprints rather than authored content alone;
- add exact built-in source mappings for the new definitions;
- route exactly the five new package intakes through full semantic admission
  with modes and coverage retained while preserving identity-only compatibility
  for the released Charter intake;
- constrain remediation to
  `ArtifactIntakeDefinitionV1::parse`,
  `AuthoredArtifactKindDefinition::validate`,
  `validate_authored_profile_fingerprints`, and
  `load_repository_intakes`, plus the operator-approved exact
  type-absent/string-valued-`const` branch in
  `ResolvedSchema::collect_coverage_leaf_shapes`; do not edit
  `ArtifactIntakeRegistry::load_with_builtin_compatibility` or
  `resolve_profile_selection`;
- preserve indeterminate refusal for every non-string `const`, type-absent
  `enum`/default/examples/annotation, composite, unsupported explicit type,
  open object, cycle, pointer, and reference case; do not edit released schema
  bytes or infer any other shape, and prove a string `const` paired with an
  unsupported explicit type remains refused;
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
- direct `artifact_kind_registry`, `artifact_instances`, schema-family,
  profile-selection, HCM-2.3 registration, and HCM-2.4 definition-support and
  runtime targets replay every accepted and refused admission branch;
- actual `ArtifactRepositoryV1::open` proof admits each exact new package
  intake and refuses stale dependency fingerprints, wrong kind/schema binding,
  missing coverage, unsupported mode, and accidental Charter compatibility
  expansion;
- the unchanged HCM-2.3 registration target retains exact SHA-256
  `86e6e98fe8fad63578be3d157d3a836c4bfe3a4dc7aa49f1ff41d87e33b61629`
  before and after remediation;
- no public API, Cargo, package metadata, external dependency, command, or
  Projection change.

Stop if definition admission cannot remain exact-ref, fail-closed, and
package-owned without a forbidden surface change.

Fresh depth-3 tests-included impact before selector closure is LOW/11 for the
intake parser, LOW/0 for the kind validator with incomplete graph edges,
CRITICAL/130 for the profile fingerprint validator, and CRITICAL/44 for the
repository intake loader. The two CRITICAL results are accepted only for the
exact branch-local correction above; any broader changed symbol, process,
module, public caller, or generic behavior stops remediation. Fresh
resumed-baseline impact for
`ResolvedSchema::collect_coverage_leaf_shapes` is LOW/1 with no indexed process;
it is the fifth and final production symbol, and needing a sixth stops P1A.

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

RED: the built-in request now selects the exact successor and the P1C
compatibility target passes 9/9, but the unchanged HCM-2.1 target passes only
3/12 because `selected_contract_matches` refuses the selected Project Context
kind `1.1`. Before editing the predicate, extend its existing unit test so the
exact `1.1` positive fails while the exact `1.0` positive remains green. After
that predicate repair, the discovery review also identified two Unix-only
projection assertions that remain RED because they expect the superseded
selected kind `1.0`. Direct WSL RED proof then found two preceding stale
expectations: the compiler test still expects doctor report schema `1.1.0`,
and the CLI test installs an obsolete inline Charter schema `1.0` instead of
the existing valid selected-Charter helper.

Work:

- update only `shipped_profile_request`'s selected exact profile and immutable
  schema/kind/intake/renderer source closure;
- extend only Project Context's existing private `selected_contract_matches`
  predicate so exact kind `1.0` and exact kind `1.1` are compatible over the
  unchanged schema `1.0` and canonical path; retain equality between decision
  and instance refs, all mismatch refusals, and add no production symbol or
  generic fallback;
- prove both exact positives and refuse both crossed `1.0`/`1.1` directions,
  unrelated identity, unlisted version, prefix, range, `latest`, bare/fallback,
  wrong schema, and wrong path values in the existing predicate unit test;
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
- in Unix compiler test
  `crates/compiler/tests/doctor.rs::doctor_api_projects_the_exact_stable_project_context_row`,
  update only report schema expectation `1.1.0` to `1.2.0` and Project Context
  kind expectation `1.0.0` to `1.1.0`;
- in Unix CLI test
  `crates/cli/tests/cli_surface.rs::doctor_reports_ready_when_required_artifacts_present`,
  replace only its obsolete inline Charter schema `1.0` setup with the existing
  `write_valid_selected_charter` helper and update only its Project Context
  kind expectation `1.0.0` to `1.1.0`; do not edit that helper, fixture assets,
  any other assertion, or any production surface;
- run both exact Unix projection tests and
  `doctor::tests::doctor_nulls_project_context_and_is_invalid_after_substitution_or_inode_aba`
  through WSL with an isolated Linux target directory; and
- replay the complete selected-profile and HCM-2.1–HCM-2.3 preservation wall.

GREEN:

- exactly three root descriptors resolve with identical Charter authority and
  condition semantics;
- generic consumers observe exact shipped-root 1.2;
- every Charter record and validator observes the frozen HCM-2.2 1.1 pair;
- neither Project Context nor Environment Context has a null intake or empty,
  extra, or mismatched renderer selection;
- both Unix doctor projections expose exact selected Project Context kind
  `handbook.artifact-kind.project-context@1.1.0`, compiler report schema
  `1.2.0`, released valid Charter setup, and the Unix substitution/ABA refusal
  remains green;
- Work/Decision/Risk remain source-available but unselected;
- no outcome changes outside intended Project Context/Environment support
  refs; and
- GitNexus change detection and scoped diff report exactly
  `shipped_profile_request`, `selected_contract_matches`, its existing in-file
  unit test, the three exact Unix-only assertion literals, and the one exact
  existing-helper setup substitution, with every read/proof anchor unchanged.

## P2 — Environment Context vertical

Entry: P1C accepted; Environment Context impact UIDs resolved; runtime entry is
blocked on the exact condition-evidence/evaluator authority described below.

Authority gate:

- HCM-0.6 deliberately leaves the condition record schema, exact input
  bindings, admitted evidence types, precedence, freshness thresholds, and
  evaluator implementation undecided;
- the live shipped resolver consequently returns only `unresolved` /
  `EvidenceContractUnavailable` and `indeterminate` for the selected
  Environment Context descriptor;
- P2 does not select `crates/engine/src/profile_decision.rs`, a new evaluator,
  or any evidence-contract surface; and
- structured Environment Context input, `applicability_basis`, profile opt-in,
  or mere artifact presence cannot authorize itself or be coerced to
  condition `true`/`false`.

Do not begin the RED-to-GREEN runtime loop until a separately approved,
fingerprinted condition-evidence/evaluator contract either enters HCM-2.4 with
an exact reviewed selector or is completed as a proven dependency. This gate
does not block the semantically independent P3–P5 packets, but P6 still waits
for P2–P5 GREEN replacement proof.

RED:

- the author command writes Environment Inventory Markdown;
- changing the legacy Markdown can influence flow/baseline behavior; and
- the selected conditional YAML is not the sole mutation/read authority.

Work:

- parse structured input into the exact Environment Context canonical schema;
- validate and atomically write `.handbook/project/environment.yaml` only when
  the selected descriptor condition applies;
- route CLI, setup, doctor, compiler, manifest, and flow reads to retained YAML;
- use only the exact Environment Inventory branches in the P2 selector,
  including the fixed canonical/baseline and flow adapters required to make
  legacy Markdown non-authoritative; leave aggregate bridge-type and
  fixed-selector deletion to P6;
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
- engine and compiler manifest generation prove the selected Environment
  Context identity/fingerprint, conditional absence, and zero influence from
  legacy Markdown mutation or deletion;
- legacy Markdown deletion or mutation has no outcome effect;
- Charter and Project Context focused regressions remain green.

Stop rather than weakening this packet if condition `true`/`false` would
require inference, an unchecked declaration, a self-referential Environment
Context field, `indeterminate` coercion, or an unselected evaluator edit.

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

## P3B — CLI surface proof integration

Entry: P3 closure review CLEAN. The first exact selector review was CLEAN, but
the first focused test demonstrated that the recursively copied fixture
repositories retained the pre-P3 Stage 10 contract. The additive fixture-only
selector repair was reviewed CLEAN before fixture synchronization. The next
focused preview advanced to the HCM-2.2 durable repository-identity
prerequisite. The exact test-only initialization amendment in
`decision/20260727-p3b-repository-identity-prerequisite-selector-repair.md`
was reviewed CLEAN. The next journey wall showed that the two foundation-flow
Work Specification inputs still carry a generic record ID instead of the
preserved M4 feature identity. The exact two-fixture amendment in
`decision/20260727-p3b-foundation-feature-identity-selector-repair.md` requires
fresh review before those lines change. Its corrected selector closed review
CLEAN, but the M5 test then exposed a test-only consumer that still treats the
generated Markdown view as bundle authority. The exact canonical-YAML consumer
and evidence repair is frozen in
`decision/20260727-p3b-m5-canonical-consumer-selector-repair.md` and requires
fresh review before implementation. After that repair, the full CLI wall
isolated only the paired shared compile goldens. Their exact proof-only update
is frozen in
`decision/20260727-p3b-shared-compile-golden-selector-repair.md`. Removing all
transient fixture outputs before the complete P3 wall then exposed one negative
CLI test that tried to delete an untracked canonical output before invoking the
product. Its exact one-assertion repair is frozen in
`decision/20260727-p3b-negative-fixture-independence-selector-repair.md`.
After that negative suite passed, the next P3 gate exposed one stale
`feature_spec_contract` equality between the rich M4 generated view and an
unchanged minimal renderer fixture for different canonical input. Its exact
cross-case test repair is frozen in
`decision/20260727-p3b-feature-spec-contract-selector-repair.md`.

RED: `cargo test --workspace` isolates eight Stage 10 failures in
`handbook-cli --test cli_surface`. After removing seeded outputs and repairing
the first assertions, the first focused preview still refuses because the
recursively copied fixture stage declares only the superseded
`FEATURE_SPEC.md` output. After synchronizing the reviewed fixture inputs, the
preview reaches artifact-repository admission and refuses because the
temporary repository has no setup-owned durable identity.

Work:

- edit only the exact paths and Stage 10 assertions frozen in
  `decision/20260727-p3b-cli-surface-proof-selector-repair.md` and
  `decision/20260727-p3b-fixture-contract-selector-repair.md`;
- synchronize only the ten selected fixture-authority inputs, keeping the six
  canonical `core/` mirrors byte-identical to P3 and the two profile-file pairs
  byte-identical across fixture roots;
- initialize one fresh setup-owned repository identity inside each selected
  temporary Stage 10 test repository through the existing engine service;
  never commit or inject a fixture identity;
- change only the `record_id` in the two foundation-flow Work Specification
  model outputs so the existing stable M4 feature identity remains the handoff
  and downstream planning identity;
- make the M5 test consumer read and parse canonical Work Specification YAML
  from the repo baseline and emitted bundle, then refresh only the exact
  generated views, plan, scorecard, and transcripts selected by the M5 repair;
- mechanically regenerate only the paired shared Stage 10 compile payload and
  explain goldens after fresh selector review, then replay the complete CLI,
  compiler, and pipeline direct-consumer suites;
- make the missing-Work-Specification refusal test assert that a freshly copied
  committed fixture starts without the canonical output instead of depending
  on untracked generated output solely so setup can delete it;
- replace the stale rich-view-to-minimal-fixture equality in
  `feature_spec_contract` with exact happy/skip canonical-input and
  generated-view equality while retaining every schema/template assertion and
  the real-path CLI renderer proof;
- remove only the two seeded post-capture outputs from Stage 10 test repos;
- update refusal, canonical-path, generated-view, and handoff assertions to the
  already reviewed P3 contract; and
- regenerate only the two exact journey transcripts from current deterministic
  CLI output.

GREEN: the eight failures, full 98-test CLI surface, complete P3 wall, and
workspace wall pass without a production or public-contract change.

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

Current bounded result: selection, descriptor/intake coverage, retained
read/validate, and the exact negative token-refusal regression pass. The
positive generic mutation test remains ignored-by-default and fails when run
with `--ignored` because planner-derived underscore-bearing coverage tokens
violate the unchanged lineage token grammar. P4 remains incomplete under
`decision/20260727-p4-p5-generic-mutation-token-authority-stop.md`.

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

Current bounded result: the independently selected Risk Record fixture,
coverage/intake, renderer, read/validate, and negative surface pass. The
positive generic mutation test remains ignored-by-default and reproduces the
same Store refusal under `--ignored`. P5 remains incomplete under the shared
authority-stop decision; no generic runtime or released-definition correction
is selected.

## P6 — Aggregate flow and fixed-selector deletion

Entry: P1A–P1C and P2–P5 individually committed and green; no bridge deletion
has occurred.

Current entry result: not satisfied. P2 lacks condition-evaluator authority,
and P4/P5 lack authorized positive generic mutation proof. Do not begin P6.

RED:

- both bridge IDs and bridge types exist;
- fixed enum/order/layout/path selectors decide flow authority; and
- bridge exceptions and legacy fixed-family tests remain.

Work:

- collect flow inputs from admitted profile/instance selection with stable
  ordering and retained bytes;
- treat deletion of `CanonicalArtifactKind` and `CanonicalLayoutContract` as
  one atomic compile closure across every exact textual consumer named in the
  P6 selector; the indivisibility is type removal only and does not authorize
  unrelated behavior in those files;
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
- limit closeout edits to the exact P7 paths in the reviewed manifest: affected
  `03`, `04`, `06`, and `09` rows, HCM-2.4 SPEC/plan/todo status and evidence,
  and new P7 proof/review records;
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
