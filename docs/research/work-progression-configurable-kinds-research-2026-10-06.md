# Configurable operational work kinds and skill-family adapters

**Status: PROPOSED research for human product-model review — 2026-10-06.**
This report neither adopts a work model nor supplies phase, slice, packet, schema,
or implementation authority. Research baseline: branch
`feat/handbook-contract-membrane-clean`, HEAD
`70019b7356bd2451b2d72d389741bec0f7f1f525`; current working-tree sources were
inspected alongside existing staged and unstaged work. Only this report is written.

## Recommendation

Handbook should permit custom operational work names and bounded semantic
composition. **Prefer B only as an optional, reusable, versioned composition of
owner-supported semantics; defer a separate first-class work-kind repository
until a small mapping exercise demonstrates its value over roles and policies.**
The evidence establishes a need to distinguish meanings and completion questions,
but does not establish that each distinct meaning needs a new kind abstraction.
An owner-managed record under `.handbook`, changed through native SDK operations
and later consumed by CLI, is a plausible realization, not a selected storage,
format, owner, or lifecycle.

A is an adequate starting point if its fixed operational types support explicit
roles, policy selection and composition. Adopt B when the same validated bundle
must be reused, identified and evolved together across items or adapters. Reject C
for this product boundary: programmable orchestration would require a much larger
execution and authority model than the examined use cases justify. If B requires
arbitrary transitions, hooks or rule evaluation to represent the examples below,
defer that capability rather than disguise C as configurable kinds.

The main tradeoff is reuse versus another source of semantic identity, admission,
compatibility and historical resolution. B can reduce repeated configuration; it
cannot make unsupported meanings executable or reduce the approved completion
floor. These are recommendations, not approved semantics.

## What exists, and what would be new

The controlling [operational semantics](../specs/handbook-contract-membrane/semantic-model/operational-reliance-and-actions.md#work-progression-ownership)
are approved semantic input. Work Progression owns local and parent eligibility,
policies, complete reliance/accounting and replay. Authoritative work-state and
structure owners supply identities, state and relations; Work Progression cannot
invent or mutate them. Physical co-location is allowed with separately attributable
responsibilities. Concrete work types, hierarchy, state machines and placement
[remain deferred](../specs/handbook-contract-membrane/semantic-model/operational-reliance-and-actions.md#settled-versus-deferred).

| Primitive | Evidence and distinction |
| --- | --- |
| Labels and aliases | Implemented vocabulary presentation/lookup. A word cannot establish executable behavior, authority or an operational identity. |
| Stable roles | Implemented typed registry positions: `coordination_horizon`, `delivery_unit`, `implementation_unit`, `execution_envelope`, `atomic_action`. They are not five proven stateful, completable item types, actor roles, or Action Kinds. |
| Operational work kind | Proposed reusable classification/bundle for work instances. No inspected source establishes this owner/lifecycle. It must remain distinct from generic Artifact Kind and Action Kind. |
| Work instance | A particular owner-recognized body of work, with identity and state. Its concrete native model is unselected; a spec or label cannot supply it. |
| Relations | Approved requirement for exact finite owner-established boundaries. Concrete supported containment, prerequisite, cardinality, reach and cycle semantics are unselected. |
| Lifecycle states | Work-state owner meaning, distinct from labels and routing states. No configurable transition machine is approved. |
| Completion policy | Approved Local Closeout/Parent Eligibility responsibilities; concrete policy content is deferred. A kind may reference supported policies, but cannot waive independent governing assignment or assessment. |
| Adapter | Future bounded translation preserving exact owner meanings and losses. Installation is not admission, activation or authority. |
| Work Specification | Existing authored family for a bounded plan. Acceptance prose and authored `completed` status cannot establish operational completion. |

The inspected [core role registry](../../crates/engine/definitions/stable-roles/handbook.roles.core/1.1.0.yaml),
[registry loader](../../crates/engine/src/stable_role_registry.rs) and
[vocabulary loader](../../crates/engine/src/vocabulary_registry.rs) establish narrower
behavior than arbitrary custom semantics. Vocabulary loading pins a package-owned
builtin registry and fingerprint; labels/aliases must refer to recognized roles;
nonempty undeclared extensions refuse. Duplicate display labels are legal and
untyped lookup reports ambiguity. Absorption units can have a valid symbolic ID
outside the registry, but their targets must be recognized workflow roles;
non-workflow/constitutional targets, duplicate ownership and directed cycles
refuse. This is structural vocabulary configuration, not arbitrary work-kind or
dependency-graph execution.

The actual [HCM-3.1 fixture](../../crates/engine/tests/fixtures/hcm_3_1_vocabulary_resolution/definitions/vocabularies/example.vocabulary.hcm-3-1/1.0.0.yaml)
labels `delivery_unit` Feature and `implementation_unit` Slice, declares
absorptions, and makes `task` deliberately ambiguous between `atomic_action` and
`implementation_unit`. Its [tests](../../crates/engine/tests/vocabulary_registry.rs)
preserve typed lookup despite names; these tests were inspected, not executed.
The [shipped vocabulary](../../crates/engine/definitions/vocabularies/handbook.vocabulary.shipped-root/1.0.0.yaml)
has empty mappings and deterministic role-label fallback.

The [Work Specification schema](../../crates/engine/definitions/schemas/handbook.schemas.artifacts.work-specification/1.0.0.schema.json)
permits authored `completed`, while the approved
[Work Specification relationship](../specs/handbook-contract-membrane/semantic-model/operational-reliance-and-actions.md#work-specification-relationship)
expressly denies that label operational authority. [Pipeline RouteState](../../crates/pipeline/src/route_state.rs)
holds routing booleans, references, runner/profile and route basis; its existence
does not prove a work-state owner. Existing
[Contracts gate/local-closeout composition](../../crates/contracts/src/gate_composition.rs)
is explicitly pure legacy bounded truth, not the greenfield Work Progression owner.

Under the approved [ontology](../specs/handbook-contract-membrane/semantic-model/objects-and-governance.md#generic-artifact-inclusion-and-exclusion),
a future kind definition would fit the semantic-definition category, not a seventh
authored artifact family. Durable owner-established snapshots/results/receipts and
projections retain their own categories. This classification does not select the
concrete work-instance lifecycle or force everything into the artifact repository.

Bounded historical evidence at `8605d6539355f350c55b921d95fb450bca30ee18`
(`8c93d6f3^`) was inspected only in `02-semantic-model.md` sections “Stable
semantics and configurable language”, “Instance profile”, “Vocabulary model”, and
the `04-phase-slice-map.md` HCM-6.5 entry. It explicitly intended local
Feature/Phase/Slice/Packet/Task renderings, lexical conflation and structural
absorption without erasing authority. Kind/profile references there concern the
then-existing artifact/profile model; they do not approve a complete operational
kind system. No historical orchestration process is adopted here.

## Contrasting workflow evidence

Public primary documents were accessed on 2026-10-06. The sample tests conceptual
fit, not adapter implementation or a claim of complete family coverage.

| Family and provenance | Observed behavior | Consequence for Handbook — inference |
| --- | --- | --- |
| GSD Core, maintained `open-gsd/gsd-core`, [command reference at v1.13.0](https://raw.githubusercontent.com/open-gsd/gsd-core/v1.13.0/docs/COMMANDS.md), sections `spec-phase`, `execute-phase`, `audit-milestone`, `complete-milestone` | A phase has a deliverable specification, plans execute in waves, and phase verification is distinct from milestone audit and archive/tag effects. Milestone completion documents unresolved-item accounting and an unstarted-phase guard with a separate forced path. | Model phase delivery, execution grouping and milestone consideration separately. A summary file, wave completion, or family override cannot by itself satisfy Handbook governed completion or supply its effect authority. |
| OpenSpec, commit `9d4e5974e5c0d9a09b9c6c1e1eb0975e80ec4461`, [workflows](https://raw.githubusercontent.com/Fission-AI/OpenSpec/9d4e5974e5c0d9a09b9c6c1e1eb0975e80ec4461/docs/workflows.md) | Change-scoped work supports revisiting planning and parallel changes. Verification is optional/nonblocking for archive; the guide says incomplete tasks warn rather than universally block archive. | A Change may be a bounded work item, while proposal/design/task files are inputs or views. External archive truth must not become a Handbook governed-completion receipt. |
| Same OpenSpec commit, [customization](https://raw.githubusercontent.com/Fission-AI/OpenSpec/9d4e5974e5c0d9a09b9c6c1e1eb0975e80ec4461/docs/customization.md), “Schema Structure” and “Validate Your Schema” | Custom schemas define artifact IDs, generated paths, prompt instructions, `requires` dependencies and apply/task tracking; validation checks templates and circular dependencies. | This is useful evidence for bounded customization, but artifact-existence dependencies are not operational child closure, qualified assessment or action authorization. Supporting all custom instructions as executable Handbook semantics would expand toward C. |
| Installed Prototype skill family, `C:/Users/spmcc/.codex/skills/prototype/SKILL.md` and `LOGIC.md`, inspected 2026-10-06 | The workflow answers an explicit design question, exposes state for exploration and retains the answer; prototype code may be deleted or absorbed afterward. | Completion can be “question answered with adequate evidence”, even when the candidate idea is rejected. Production implementation completion is a different obligation. |

Prototype source SHA-256: `SKILL.md`
`9c750b700a6396697ddce3bcca04d322179d73c5d4f7d82268309eff84fd1176`;
`LOGIC.md` `3ce34ce7c2d06bcbdac4d846aafc488bbef0b2cd067134eeeda34c3804842f2f`.
These installed files are source evidence, not instructions invoked to build a
prototype. Their upstream release/commit was not established. The original
[GSD repository is archived](https://github.com/gsd-build/get-shit-done/commits/main/);
the successor/tag above supplies the comparison. Public version/commit pins are
research samples, not assertions that they are the newest releases.

## Alternative comparison

| Test | A — fixed operational kinds, configurable vocabulary | B — constrained versioned kinds | C — general workflow definitions |
| --- | --- | --- | --- |
| Expressiveness | Strong for terminology when supported types plus independent policies already express the work. Weak if “Phase” must always mean one operational type. | Names repeatable combinations of supported roles, relation participation, lifecycle selection and policy references. Adds no new primitive behavior. | Can encode bespoke transitions, scheduling and rules; fidelity is broad only after every relevant effect and owner is modeled. |
| Cost and extensibility | Smallest definition surface; semantic additions require product evolution. Per-item policy variation need not require new kinds. | More identities, compatibility checks and history, but reusable validated bundles reduce repeated choices. Novel semantics still require product evolution. | Runtime, rule validation, effects, retries, termination and debugging become product responsibilities. Custom definitions can still be wrong. |
| History | Vocabulary pins plus actual item/policy/state provenance suffice. Fixed types can also evolve; A is not version-free. | Pin exact bundle meaning and referenced semantics for each binding; preserve earlier versions and explicit rebinding history. Default changes cannot reinterpret old work. | Must preserve interpreter/rule versions, execution histories and behavior of earlier definitions as well. |
| Validation/authority | Known type conformance plus exact policies, assignment and prior authorization. Labels do not relax floors. | Validate supported composition and owner compatibility, then separately establish admissibility, selection/assignment and authorized change. Kind admission is not completion-policy assignment. | Static workflow validation cannot prove effect authority, currentness, termination or substantive adequacy of user rules. |
| Relations/cycles | Enough if supported relations have complete finite boundaries. Neither a fixed tree nor arbitrary graph follows from fixed types. | Same boundary obligation; kind configuration may constrain supported relations but cannot extend traversal or invent cycle semantics. | Greater graph freedom increases closure/cycle/recovery burden; it does not remove the approved finite-boundary requirement. |
| Adapter/standalone fit | Works without a pack; simple mappings succeed when owner meanings already fit. | Useful shared mapping contracts across several custom item types, if mapping remains attributable and losses explicit. Must also work without a skill family. | A family execution engine may fit its own workflow; adopting it as Handbook's universal core increases dependence on family semantics. |
| SDK-first fit | Easiest initial owner/SDK surface. | Compatible through native owner operations, with later thin CLI/adapters. A kind repository alone delivers no progression. | Prematurely broad execution surface for the current stage. |

**Roles/policies/composition without kinds are a real fourth design shape.** An
owner-recognized generic bounded work item can carry explicit supported roles and
exact policy/relation bindings; vocabulary renders it “Experiment” or “Change”.
If those bindings already compose deterministically and users do not need to reuse
or evolve a bundle as a unit, a separate kind duplicates information. B earns its
place only by owning the compatibility contract and identity of that reusable
bundle. It should not become a competing policy selector, parallel role registry
or mandatory taxonomy. This distinction is unresolved by the inspected code.

## Worked examples and counterexamples

These are proposed semantic tests, not selected schemas or available operations.
The approved [local closeout](../specs/handbook-contract-membrane/semantic-model/operational-reliance-and-actions.md#local-closeout)
and [parent rules](../specs/handbook-contract-membrane/semantic-model/operational-reliance-and-actions.md#parent-eligibility-and-parent-action)
control every governed case.

| Case | Faithful bounded interpretation | Incorrect shortcut / boundary |
| --- | --- | --- |
| Same word: Phase | “Phase Q4” can be a coordination timebox; “Phase Authentication” can be a deliverable. Explicit roles and owner-recognized targets disambiguate. Ending a timebox need not discharge a delivery obligation. | Treating both as “all children completed”, or assuming all coordination horizons require closeout. A new scheduling engine is unnecessary for recording this distinction. |
| Several roles in one item | A small Task combines implementation unit, execution envelope and atomic action, following explicit supported absorption/composition. One visible item retains all required propositions and authority boundaries. | Creating three mandatory hidden child items, or allowing the collapse to erase a separate required authorization/assessment. Existing absorption validation proves neither operational item creation nor multi-role state behavior. |
| Experiment disproves its hypothesis | Experiment E asks whether caching meets a stated performance need. Evidence establishes it does not. E's declared completion question is whether the investigation was adequately conducted and concluded; that assessment can be positive while the hypothesis outcome is negative. The delivery item still lacks its performance requirement. | Turning a failed delivery gate into success by renaming its target Experiment. A diagnostic result cannot fill the mandatory Decision-seeking eligible/decision-seeking assessment role for governed completion. If no supported policy can express adequate investigation, this capability remains unsupported. |
| Nested custom items | Investigation I contains Experiments E1/E2; E1 contains bounded trials only if the selected state/structure owner supports these items and relations. Every parent policy has its own finite relation cut and required propositions. | Inferring a universal tree, recursive transitive rollup, or parent eligibility from children merely labeled eligible. A policy requiring actual closure consumes current closed-state/receipt truth. |
| Shared/external prerequisites | E1 and E2 both depend on Dataset D; neither thereby contains or owns D. A deployment depends on an externally owned approval. Exact identities, direction, scope, resolver/currentness and complete accounting are required. | Duplicating D to force a tree, taking ownership of external state, or treating a copied external “done” label as proof. Without a supported authoritative verification route, refuse the affected reliance/effect; a custom kind cannot manufacture one. |
| Kind changes while items are active/closed | E1 remains bound to Experiment v1; newly created E2 may select v2. Explicit authorized rebinding of active E1 creates a new binding/current basis and rechecks affected relations/policies. Closed v1 history remains readable. Current governing obligations are independently resolved and can still change under their prior authorized terms. | Automatically migrating active/closed items on pack update, or freezing current governance merely because an item pins v1. Unsupported rebinding may be refused rather than promise a migration framework. |
| Membership changes after eligibility | Parent P is eligible over E1/E2 at a declared cut. E3 is then added. The old determination remains history; the changed membership requires a new parent basis/result, and declared through-commit relation checks prevent using the old closure for mutation. | “All children were eligible when checked” as permission to commit now. Correct eligibility is still distinct from committed completion and its receipt. |
| Lossy/unsupported adapter | An external tracker offers only one Boolean “done”, with no distinction between hypothesis verdict, investigated completion, cancelled work and archive. Read-only translation can disclose exact losses and source meaning. A mutating route must obtain the full owner-required facts or refuse. | Mapping every terminal state to governed completion, silently flattening shared dependencies or absorptions, or substituting an OpenSpec archive for Handbook assessment/authorization. Loss reporting does not authorize a lossy effect. |

Iteration is not automatically a dependency cycle: a new trial or revised plan can
be a later occurrence. Proposed initial support should refuse cyclic containment
and prerequisite closure where no supported cycle treatment exists. Future
convergence, recurrence or co-dependent groups would require separately selected
semantics; neither arbitrary DAG support nor one universal tree is established.

## Proposed configuration limits and delivery fit

Keep configurable names, descriptions and explicit vocabulary separate from
meaning. A reusable kind, if justified, may choose only owner-supported role
composition, lifecycle semantics, relation participation and exact admitted policy
references within declared compatibility limits. No executable hooks, arbitrary
rule language, caller-defined effects, automatic transition scheduling or generic
status setter bypass. Unsupported combinations should produce a specific refusal,
not inferred equivalence or an LLM-created semantic bridge.

Establish immutable definition versions and exact semantic content; distinguish
available definitions, selection defaults, item bindings, current governing
assignment and historical results. No `latest` reinterpretation, implicit pack
activation, or mutable in-place meaning. Changes use prior approved terms, not the
new candidate's self-authorization. Initial human operating/governance boundaries
remain attributable; supported routine changes may proceed within that delegation
without an automatic repeat prompt. These limits follow
[G1/G2/G5/G6](../specs/handbook-contract-membrane/semantic-model/amendments/governance-defaults-and-delegated-operation.md#approved-rules).

For local state, its native owner must establish current relation/state facts and
truthful commit/recovery. For external state, the external owner remains
authoritative; select a supported exact verification/effect route or limit the
adapter to honest inspection. A shared directory, SDK facade or component does not
merge state ownership, Work Progression, assessment and action authority.

`.handbook` owner-managed records fit an operations-led product direction, but
physical placement is unselected. The
[structured-data format standard](../specs/handbook-contract-membrane/structured-data-format-standard.md)
is **PROPOSED** except for explicit existing selections such as Contracts JSON.
Its suggested JSON default for machine-managed records does not select a work-kind
format. Direct authoring/import, activated records, exact identity and human views
are separate future choices; preserve existing versioned contracts. No schema,
serialization change, importer or migration is proposed by this report.

The [current stage](../specs/handbook-contract-membrane/phase-and-slice-map.md#current-delivery-sequencing)
is native Rust owners and usable SDK capability. New CLI/installed-skill/agent-led
workflows follow later. [HCM-6.5](../specs/handbook-contract-membrane/phase-and-slice-map.md#phase-6--consumer-adoption)
still requires adapter-owner/mapping decisions and follows bridge → publication →
direct Substrate adoption → Tauri → workflow adapters. Custom kinds must not
reorder that sequence or create another Packet 5. An eventual SDK caller should
be able to use the selected work semantics without installing any skill family.

The [current coverage map](../specs/handbook-contract-membrane/coverage-and-landing-map.md#coverage-and-responsibility)
places missing progression and work-state realization with their owners. The
dated [reliance research](hcm-coverage-landing-2026-10-01/reliance-and-actions.md)
and [future obligations](hcm-coverage-landing-2026-10-01/future-obligations.md)
support that gap diagnosis, but their pre-HR-007 prospective-only completion
language is superseded: current semantics also allow expressly adopted bounded
retrospective qualification of exact earlier results. The
2026-10-03 adapter discussion in the local, untracked
`governance-defaults-autonomy-reconciliation-2026-10-03.md` was intent evidence;
current [G6](../specs/handbook-contract-membrane/semantic-model/amendments/governance-defaults-and-delegated-operation.md#g6--future-packs-and-adapters)
controls and selects no pack mechanism. This landing retains the attribution
without depending on that unlanded report for its authority or navigation.

Semantic distinctions, rejection of C, optional composition and version/history
principles can be discussed independently of P3/P4 completion. Narrow later
reconciliation must identify the native work-state/structure owner, exact
assessment-result inputs and protocol roles, usable SDK composition and supported
through-commit facts. P3/P4 local-check delivery alone cannot select a work model
or prove progression. This report assigns no successor implementation work.

## Human decisions still needed

| Decision | Suggested default and tradeoff |
| --- | --- |
| Does a reusable kind earn a separate identity? | Begin with explicit roles/policies/composition; introduce optional versioned bundles only when repeated mapping/configuration needs one unit of reuse. Lower initial cost, less ready-made taxonomy. |
| What is operationally completable, and who owns it? | Select bounded work-state/structure semantics before promising kinds. Treat timeboxes/groupings as distinct from dischargeable obligations; preserve owner separation even if physically co-located. Hosting and local versus external support remain human choices. |
| Which relation and lifecycle capabilities ship first? | Small explicit supported set, complete finite relation boundaries, no unsupported cycles or implied transitive rollup. Exact containment cardinality, reach and state transitions remain unselected. Lower coverage, clearer refusal. |
| What happens on kind/default/policy updates? | No implicit item rebinding or historical rewriting. Explicit authorized active-item rebinding only where supported; independently enforce current governance and invalidate affected reliance. More deliberate updates, reproducible history. |
| How much adapter fidelity is required? | Preserve meaning for supported effects; disclose losses for inspection and refuse effect mappings lacking required facts. Start with one selected family mapping when authorized, accepting partial coverage rather than promising universal translation. |

These decisions cannot be settled by field naming or a storage convention. The
smallest further validation is **one paper mapping exercise**, not a prototype:
represent one GSD milestone/phase/plan example and one Experiment containing two
trials with a shared prerequisite, first using explicit roles/policies and then
using a reusable kind. Include one active version change and the Boolean-only
adapter refusal. Identify each authoritative fact and unsupported requirement.
If B removes no repeated compatibility decision or adds no meaningful shared
identity, choose composition without kinds. If it demonstrably improves reuse
within the same semantic limits, select that narrow B direction for later bounded
owner planning. No runtime or adapter adoption follows from this exercise.

## Verification and limits

Approved owner boundaries, inspected registry/schema/route source, bounded
historical evidence, and proposed choices are identified separately above. Public references
are primary documentation at the stated version/commit; documentation claims
were not validated by executing those products. GitNexus CLI query/context was
attempted because its MCP tools are unavailable in this session: query reported
missing FTS indexes, and symbol context returned not found. Multiple stale
`handbook` registrations exist; local metadata is indexed at `a35297d2` on
2026-08-25. Consequently, direct current source supports code claims, not inferred
graph coverage. No index repair/rebuild, broad runtime suite or symbol edit was
performed. The vocabulary tests were inspected only.

Verification is limited to source/claim and link checks plus report-only scope
checks; it does not certify a new model, complete external adapter fidelity, or
prove runtime completion behavior. One focused independent local review covers
the recommendation and semantic/authority fidelity; its P1–P4 disposition is
returned in the session result. The report remains untracked, with no staging,
commit, publication, product edit or existing authority-document edit.
