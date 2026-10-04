# Governance defaults, delegated operation, and independent contract use

**Status: Proposed for human semantic sign-off.**

Prepared 2026-10-03 against `f62b1f450cacc31057280723970dc95ad18ad664`.
This is a bounded amendment to the [approved semantic package](../README.md),
not implementation authority. The user approved this reconciliation work and
its direction; the proposed normative rules and choices below still require
human semantic sign-off. A clean automated review, local commit, or discovery
pointer does not activate them. Until sign-off, the identified base provisions
remain operative. Once approved, this amendment supersedes **only** the
provisions identified in [Activation edits](#activation-edits), in the stated
respects. All other approved meaning and completed delivery history remain.

## Intent and ordinary workflow

Handbook should help people choose governance, rather than permanently impose
the supplied choices. A person approves project initiation, Charter/posture,
initial governance if selected, and the boundaries within which agents may act.
The agent uses repository evidence and those approved sources to recommend
suitable starting requirements. Recommendations do not activate themselves.
Thereafter owner checks can apply approved rules automatically. Human
intervention is for reserved product decisions, material ambiguity, or missing
authority that approved sources cannot resolve.

A person may initialize project P and explicitly defer project governance.
They approve its Charter/posture and delegate bounded local authoring and checks.
An agent authors a complete API compatibility contract C, with its complete
claims, observation requirements, evaluators, evidence rules, metrics if any,
and gate. Contracts validates and establishes the exact contract and compatible
definitions for independent use. The agent runs its authorized checks; a real
pass or failure answers C's exact API question. The result is useful immediately
and is not labeled Draft merely because governance is deferred. It carries the
exact deferral basis and says no project-governed shipment obligation was assessed.

Later the person establishes governance for the same P, retaining its identity,
content, and history. They approve requirements for shipment and an agent's
delegated shipment boundaries. Admission qualification, governing assignment,
the required assessment, present reliance, action authorization, and commit
checks then apply to shipment. Under the proposed retention of the existing
governed-completion floor, C can be qualified and selected in that assessment;
the earlier independent pass remains development evidence and cannot become
the required shipment assessment by relabeling it. If all current requirements
and delegated authority permit the action, the agent proceeds without another
prompt. A required product choice or missing authority is escalated specifically.

These are target semantics. Current APIs do not yet deliver this whole workflow.

## Bounded source inventory

This inventory classifies behavior, not filenames. `core`, built-in packaging,
or current hardcoding does not establish a permanent project requirement.
References describe source at the preparation baseline; none selects a new
preset, numerical default, or implementation technique.

| Supplied behavior and evidence | Classification and proposed treatment | Actual selection constraint or unchanged mechanism |
| --- | --- | --- |
| Shipped profile and three Context Resolution policy references: [profile request](../../../../../crates/engine/src/profile_decision.rs), `shipped_profile_request`; [profile resolver](../../../../../crates/engine/src/profile_selection.rs), `policy_refs` in `resolve_profile_selection`. | Selecting `handbook.profile.shipped-root@1.2.0` and the exact mutation/escalation/memory `*.core@1.0.0` trio is a supplied choice. Recommend supported starting selections; deliberately adopt them or supported replacements. | The request supplies built-ins and the resolver reconstructs the trio independently. Changing a filename or adding another definition cannot replace that selection. Preserve exact source-use accounting, dependency compatibility, content fingerprints, and deterministic resolution. |
| [Shipped Context Resolution stack](../../../../../crates/engine/definitions/context-resolution/handbook.context-resolution.shipped-root/1.0.0.yaml): strategic/coordination/execution/operation levels and scope/detail/time/authority/memory/validation defaults. | Mixed: level names, dimension vocabulary/ranks, and their default combinations are replaceable supported operating definitions and recommendations. They are not automatically universal project authority. | Exact stack identity, supported coordinate interpretation, declared ordering, and request-within-envelope checks remain mechanisms. Replacing a vocabulary/order requires a compatible versioned definition and consumers, not arbitrary runtime values or silent reinterpretation. No new values or ranks are selected. |
| [Mutation matcher](../../../../../crates/engine/definitions/context-resolution-policies/handbook.mutation-matcher.core/1.0.0.yaml): ASCII glob grammar, case sensitivity, deny precedence, and bounds. | Primarily interpretation mechanics with supplied language/precedence/resource choices. Retain an exact supported matcher contract; do not declare this particular grammar, case mode, precedence, or numeric bounds eternal project policy. | Unambiguous path interpretation and checking actual mutation against authorized reach remain required. A future supported replacement must preserve that proof; this amendment supplies no alternate grammar or resource values. |
| [Escalation policy](../../../../../crates/engine/definitions/context-resolution-policies/handbook.resolution-escalation.core/1.0.0.yaml): rank-increase/allow-expansion/missing-context/missing-authority triggers and same-stack strict widening. | Mixed: trigger selection and supported proposal relation are replaceable operating choices. Exact current/proposed basis, requested authority, truthful terminal accounting, and no preapproval authority change are mechanisms. | A trigger requests resolution; it does not inherently require a new human prompt. The approved authority/replacement terms decide whether an agent can resolve it. Missing permission cannot be repaired by the proposal's own terms. |
| [Memory promotion policy](../../../../../crates/engine/definitions/context-resolution-policies/handbook.memory-promotion.core/1.0.0.yaml): strictly higher memory rank, semantic-memory target, exact sources and expected target fingerprint. | Mixed: the rank relation and supported promotion target are supplied choices. Exact sources, target/predecessor checking, truthful application/refusal/stale outcomes, and no cross-owner authority transfer remain mechanisms. | Supporting another target cannot give memory authority over canonical artifacts, contracts, or posture. Those effects need their actual owners' lifecycles and authority. The specific numeric/rank choices are not selected here. |
| [Constitutional review lock](../../../../../crates/engine/definitions/lifecycle-policies/handbook.lifecycle.constitutional-review-lock/1.0.0.yaml): review/reassessment triggers, coverage mappings, clearance via approved promotion. | Mixed: this review workflow, trigger-to-coverage mapping, and later approval requirements are replaceable choices within supported lifecycle behavior. Initial human approval and exact lineage/currentness/commit truth remain required under this proposal. | The [shipped profile](../../../../../crates/engine/definitions/profiles/handbook.profile.shipped-root/1.2.0.yaml) and [Project Authority kind](../../../../../crates/engine/definitions/artifact-kinds/handbook.artifact-kind.project-authority/1.1.0.yaml) bind the policy. [Charter registry](../../../../../crates/engine/src/charter_definition_registry.rs), `CHARTER_DEFINITION_REFS` and validation paths, also recognizes fixed subordinate references. A profile edit alone does not prove replacement support. |
| [Constitutional approval](../../../../../crates/engine/definitions/approval-policies/handbook.approval.constitutional-candidate/1.0.0.yaml) and [Charter method](../../../../../core/library/authoring/charter_authoring_method.md): required classes, registry quorum, FIDO2 user verification and exact approval binding. | Mixed: approval classes/quorum, required assurance mode, credential provider and repeated-human requirements are governance choices or provider-specific mechanics, not universal requirements for every action. Initial human approval remains; subsequent approvals follow approved delegation/reservation terms. | [Approval workflow](../../../../../crates/engine/src/charter_approval_workflow.rs), [intake](../../../../../crates/engine/src/charter_intake.rs), and [promotion](../../../../../crates/engine/src/charter_promotion_workflow.rs) fix the approval reference; registry/lineage paths constrain it too. Preserve exact content/class/authority binding and truthful assurance provenance. A selected FIDO2 rule must still verify FIDO2; advisory authority cannot masquerade as it. |

Two lifecycle anchors limit current claims: [ordinary SDK setup](../../../../../crates/sdk/src/setup_tests.rs),
`init_plans_root_creation_but_writes_no_selected_artifact`, creates `.handbook`
and a repository identity without selected artifacts;
[governance preparation](../../../../../crates/contracts/src/project_admission_governance/mod.rs),
`prepare`, refuses any existing `.handbook` root. The
[completed establishment slice](../../slices/admission-governance-establishment/README.md)
delivered fresh-project establishment for Contracts freshness-policy admission
and resolution. Ordinary policy admission, successor activation, and establishment
on an initialized project were excluded. This amendment preserves that history.

The approved Charter posture rubric/vector remains unchanged. This inventory
does not generalize every built-in into a freely editable setting. Supporting a
replacement means selecting an exact compatible definition through an authorized
owner path; unsupported replacements refuse. Selection constraints above are
future reconciliation obligations, not evidence that replacements work today.

## Proposed rules

All MUST/MAY statements in this document are proposed, pending sign-off.

### G1 — Choices, mechanics, and recommendations

Shipped governance choices MUST be supplied defaults or recommendations whose
project adoption and supported replacement are explicit. A name such as `core`
MUST NOT independently make a choice mandatory. Interpreting a selected rule
reliably remains mandatory: exact version/content identity, source attribution,
typed conformance, compatibility, complete selection/accounting, scope,
currentness, authority, preserved history, and truthful commit/replay.

The fixed versioned Handbook establishment rule and owner conformance floors
remain outside the candidate population they decide. They establish the mechanism
and non-relaxable semantic constraints, not a permanently selected review process,
engineering requirement set, credential provider, or numeric default. Project
replacement cannot alter those floors. Mixed definitions MUST distinguish their
supplied choices from the mechanics and owner boundaries needed to apply them.

During user-facing initialization the agent MUST ground recommendations in
available repository evidence and the proposed or approved Charter/posture,
explain their reasons and unknowns, and expose the proposed requirements and
delegated boundaries for human approval. Recommendation, human approval,
owner validation/admission, governing assignment, and committed activation are
distinct propositions even when presented in one review experience.

### G2 — Initial approval and delegated operation

The human MUST approve project initiation, initial Charter/posture, deliberate
governance deferral or initial governance choices, and the initial delegated
operating boundaries. A configured agent MAY carry out those exact approved
instructions; its configured role alone MUST NOT substitute for the human's
initial decision. One coherent review may cover several owner outcomes; separate
owners need not cause repeated human approval ceremonies.

Afterward, owners MUST apply their exact authorization and validation checks
for each operation. When the approved sources determine a permitted action
within delegated scope, an agent MAY perform it autonomously without asking
the human again solely because an owner operation occurs. This covers only
supported operations whose prerequisites actually resolve. Correctable refusals
MAY be repaired and retried within existing authority; no unfavorable result
automatically grants an exception or broader authority.

Escalate when a reserved product decision is reached, a material ambiguity cannot
be resolved from approved sources, or authority is missing and no authorized
resolution exists. Policy/Charter revision, replacement and authority expansion
follow the prior approved terms; they are not universally reserved or universally
delegated. New terms cannot authorize their own activation. Advisory instruction
authority remains possible where the exact rule permits it, honestly labeled;
no general IAM platform or credential provider is required by these semantics.

### G3 — Deliberate deferral and later establishment

Initialization MUST record whether project admission governance for the exact
scope was established or deliberately deferred under the approved instruction.
Deferral MUST NOT silently activate an equivalent default governance configuration.
Absence of a file MUST NOT prove deferral. Deferral, established governance,
damaged/unreadable established state, and uncertain establishment are distinct.
The governance responsibility MUST positively resolve the applicable state and
scope from its authoritative lifecycle. Concrete representation remains deferred.

For a project positively known to have deferred a scope and never established
governance there, the same fixed Handbook establishment lifecycle MUST support
first establishment later on its existing project identity. It MUST bind the exact
identity/scope, current deferred basis, human-approved configuration and delegated
terms, owner-rule validation, authority, and committed cutover. Concurrent or
already-established state MUST prevent stale first establishment. It MUST NOT
replace identity, rewrite content/history, infer approval from a recommendation,
or treat an uncertain earlier effect as a clean opportunity to establish again.
Adding a scope in a project with established governance follows its existing
authority, not a second bootstrap. No implicit scope inheritance or
priority is introduced.

This is a greenfield lifecycle, not legacy-root discovery, backfill, migration,
compatibility bridges, or dual behavior. Previously completed fresh-only work
does not satisfy this additional requirement. Later establishment does not rewrite
independent evaluations or claim they met newly adopted obligations.

### G4 — Separate conformance, governance qualification, and assignment

The smallest proposed reconciliation retains owner-controlled **admissibility**
as the umbrella and separates its responsibilities:

| Responsibility | Proposition and proposed boundary |
| --- | --- |
| Intrinsic owner validation and establishment | Does this exact contract/definition have complete supported semantics for its stated question? Owner-native validation establishes executable category-2 identity, conformance, compatible dependency closure, and Contract Permitted Use under exact versioned owner rules. It is not satisfied by schema syntax, authorship, self-attestation, or a bare link. It can succeed without project governance. |
| Project governance qualification | Does this exact definition/use meet the admission/use-qualification requirements applicable to this project and scope? Where such requirements apply, the responsible owner produces the existing Policy Admission Closure under the exact active configuration and owner-rule basis. Missing or unsatisfied required governance refuses that dependent use; successful intrinsic validation cannot bypass it. |
| Governing assignment and present reliance | Does an authorized relationship make these rules govern this actual operation/obligation, and does this result satisfy its current requirements? Assignment, required assessment, reliance, action authorization, and committed effect remain independent. Neither intrinsic establishment nor Policy Admission Closure makes a policy govern every operation. |

An independently usable contract and the complete definitions needed to run its
checks MAY be established under owner-native conformance rules without a project
configuration. This includes exact claims/applicability, evaluation/evidence-set
rules, metrics if selected, and gate interpretation, with independently admitted
capability and supported Engine/adapter definitions where required. The responsible
owners determine which definitions and roles support this route; a candidate,
pack, caller, or evaluation resolver cannot select its own admission rule or
declare a governance requirement irrelevant. This route does not admit project
governance policies for enforcement or authorize an operational effect.

Each consuming use MUST resolve its project/scope governance applicability
before dependent formation and at its required currentness/finalization boundaries.
For independent evaluation this is either positive deliberate deferral, or an
established basis that positively determines no project qualification is applicable,
or satisfied applicable project qualification. If established governance requires
qualification for that evaluation/definition, direct or diagnostic intent MUST NOT
bypass it. Missing, conflicting, unreadable, substituted, unsupported, or uncertain
required state refuses the affected use. Ordinary authoring and inspecting content
can continue without claiming that unresolved executable use is permitted.

The intrinsic determination and applicable project qualification MAY be carried
in one owner outcome; no new record/service split is required. An intrinsic
determination does not acquire fabricated configuration bindings. Policy Admission
Closure retains its exact governed basis when project qualification is required.
Changing immutable content or permitted use requires a new exact owner-established
revision; current-head changes retain owner authority and predecessor checks.
Fresh and successor candidates face the same applicable qualification rules.

**Contract Permitted Use** retains `Diagnostic-only` and `Decision-seeking eligible`
and the existing intent compatibility table. Owner validation establishes this
immutable run-intent property under exact supported owner criteria; applicable
project qualification is separately checked for its proposed use. The property
does not assert project approval, governing assignment, shipment permission,
maturity, readiness, or a successful evaluation. Both values require complete
semantics. Deliberately deferred governance alone MUST NOT make either value
Draft, incomplete, or unevaluable. A genuine incomplete candidate may still be
called a draft. Changing qualification alone does not rewrite permitted use or
historical results. Concrete owner criteria and project criteria remain unselected.

### G5 — Independent evaluation and governed reliance

| Use | What is required, and why |
| --- | --- |
| Author, reference, inspect, validate completeness | Exact owner/content/role truth and the applicable authoring or validation rules. Project admission governance is not inherently required to write a contract, refer to it as data, or determine intrinsic completeness. Such references cannot claim governance qualification. |
| Execute a complete direct contract and report its result | Owner-established contract and compatible full definitions; positive governance applicability resolution under G4; complete direct requirements; compatible intent; independent Engine authorization from approved operating authority; valid capability/readiness; exact collection, evidence, and truth closure. A project governance configuration is not inherently required. The initial approved instruction/delegation may support Engine authority under its supported exact rule; neither deferral nor the contract supplies that authority. |
| Establish/admit project governance policies and assign them | First-establishment or prior replacement authority, applicable configuration and owner-rule qualification, exact authorized assignment, scope and currentness. Drafting, validating, admitting, packaging, or discovering a policy does not activate or assign it. |
| Select project posture assurance for a governed obligation | Actual scoped posture and qualified admitted selector/policy/exception inputs, governing relationship and complete selection. A Charter alone does not invent policy mappings; this governed route cannot fall back to independent evaluation when inputs are missing. |
| Use results to discharge governed shipment/completion or authorize its effect | Applicable qualified policies, independently resolved actual target and governing relationship, mandatory assessment provenance/coverage, present reliance, separate action authority and through-commit checks. A prior direct pass answers its own question and supplies no generic exemption. |

G5 proposes keeping the existing governed-completion assessment floor:
Decision-seeking eligible contract, `posture_selected` requirements, and
`decision_seeking` intent, with all existing coverage/currentness and action
checks. The reason is to preserve the independently selected actual project
standard in this bounded change. It is not retained merely because it is called
core. Whether a future governed rule should be allowed to accept an earlier
direct assessment as the mandatory assessment is a material sign-off question
in [Open decisions](#open-decisions), not silently approved here.

Results MUST preserve their real findings, measurements, and gate outcomes and
separately communicate: intrinsic contract validity/completeness for the question;
exact requirements source and intent; governance state/applicability for the
project/scope at the checked boundary; exact qualification basis if applicable;
and whether any governed obligation was actually assessed. A useful summary is
“API compatibility: pass; requirements: direct contract C; project governance:
deliberately deferred; shipment obligation: not assessed.” These are semantic
facts, not a selected DTO, new combined status enum, renderer, or duplicate source.
An applicable but unmet qualification refuses the dependent operation; it does
not retroactively make an intrinsically complete contract incomplete. An unresolved
required governance basis cannot be presented as “deferred” or “not applicable.”

Governance establishment/replacement cannot rewrite a historical result or make
it satisfy a new obligation. Consumers resolve current applicable requirements
under their owners. A still-qualifying governed assessment need not be rerun for
every shipment solely because an owner check occurs. Ordinary direct evaluations
retain the same claim/metric/gate meaning, trust, evidence, authorization, and
replay guarantees; no observations is not a pass and evaluation is not permission.
This amendment promises no blanket availability of governance-dependent features
while governance is deferred.

### G6 — Future packs and adapters

A future coherent family such as Agent Skills may supply compatible definitions,
recommended choices, or bounded translation through a pack/adapter. Installation
or recognition MUST NOT grant authority, activate governance, widen delegation,
perform policy admission, or assign governing obligations. Selected contents
remain versioned owner inputs under G1–G5. The
[HCM-6.5 adapter foundation boundary](../../phase-and-slice-map.md#phase-6--consumer-adoption)
remains decision-required; this amendment selects no manifest, marketplace,
adapter, pack mechanism, or phase/slice decomposition.

## Activation edits

These are exact proposed replacement/additional rules for the affected passages.
Location names and quoted starts refer to the preparation baseline. The prospective
pointers added now do not apply these edits. On sign-off, activate the approved
wording and remove/retarget pointers in the **same** semantic integration; leaving
the old unconditional prerequisite operative would contradict this amendment.

### A1 — Objects and governance: shared admission

At [Shared policy admission](../objects-and-governance.md#policy-admission),
replace the opening two paragraphs (through “runtime object types”) with:

> This section owns shared definition admissibility and project policy admission
> for every chapter. Intrinsic owner validation/establishment, applicable project
> governance qualification, and governing assignment are separate responsibilities
> under G4–G5 of the approved governance-defaults amendment. Domain owners retain
> their exact vocabulary, compatibility, selection, and use constraints. Exception
> records and actions retain their own typed authorization and admission.
>
> A user, agent, artifact, or tool MAY author candidates. Before executable use,
> the responsible owner MUST establish complete supported semantics under its exact
> versioned owner rules. Independent contract/evaluation definitions may use the
> G4 route without project governance; this grants no project governance or action
> authority. When project qualification applies, the owner MUST independently and
> deterministically produce Policy Admission Closure under the exact active basis.
> One owner outcome may carry distinct conformance and qualification propositions;
> no separate Policy Admission Basis/Result runtime types are mandated.

Retain “The Closure MUST bind” and all its bullets for project qualification;
replace that lead-in with “For required project qualification, the Policy Admission
Closure MUST bind”. After the paragraph ending “unbounded self-admission chain”,
add G1 and G2 verbatim. This makes supplied choices and initial approval/delegation
discoverable at their owning boundary without changing unrelated admission checks.

At [Establishment and replacement](../objects-and-governance.md#establishment-and-replacement-of-governing-admission-requirements):

- Replace “Initialization MUST establish that initial setup applies to the exact
  project” with “First establishment MUST establish eligibility for the exact
  project and scope, either at initialization or later under G3”. Retain validation,
  instruction/replacement authority, damaged-state and scope-addition constraints.
- Add G3 verbatim before “Initialization and successor activation follow”. Replace
  that start with “First establishment and successor activation follow”.
- In the paragraph starting “Configuration versions MUST”, replace the sentence
  starting “Every supported admission MUST resolve” with “Every project-governed
  qualification MUST resolve one unambiguous governing configuration and owner-rule
  basis for its exact project and scope. Independent executable uses resolve
  applicability under G4 and cannot bypass applicable requirements.” Retain the
  rest of the paragraph's refusal and selection constraints for affected uses.
- Replace “Each new Policy Admission Closure MUST finalize” with “Each new required
  project qualification through Policy Admission Closure MUST finalize”. Add:
  “Intrinsic establishment and independent-use applicability checks retain their
  own exact owner basis and finalization/currentness boundaries under G4.”
- In “Replacement MUST NOT rewrite”, replace “It governs subsequent admissions”
  with “It governs subsequent qualifications within its applicable scope”. Retain
  the rest of the paragraph, including no automatic revocation/re-admission.
- Add an example row: “P explicitly deferred scope S and later requests its first
  establishment” → “Resolve P/S's exact eligible deferred basis, obtain initial
  human approval, validate and commit under G3 without replacing P's identity;
  uncertain or previously established state cannot use this route.”

The fixed Handbook rule, anti-self-authorization, exact identity/authority,
predecessor/cutover, replay, uncertainty and advisory-provenance rules remain.

### A2 — Objects and governance: contracts and repeated ownership rules

At [Contract permitted use](../objects-and-governance.md#contract-permitted-use),
replace “one exact governed” with “one exact owner-established”. Replace the
paragraph beginning “An author may propose a value” with:

> An author may propose a value; Contracts owner validation establishes supported
> explicit permitted use and complete candidate semantics under the exact owner
> rules in shared admissibility. Applicable project qualification is separately
> established under G4 and the exact active configuration before dependent use.
> Missing, ambiguous, unsupported, unverifiable, or unsatisfied required owner or
> qualification conditions refuse that dependent operation. A candidate cannot
> select its own criteria; a name, prompt, previous pass, or requested intent
> cannot supply missing authority.

Replace “Concrete qualification criteria remain project-specific and must be
selected before the dependent admission can succeed” with “Concrete owner
conformance criteria and project-specific qualification criteria remain distinct;
every applicable criterion must be selected and satisfied before its dependent
use can succeed”. Add G4's Contract Permitted Use paragraph and G5's result-status
paragraph verbatim. Keep both values, intent restriction, equal fresh/successor
qualification, immutable revisions, no mandatory promotion pipeline, and separate
governing assignment.

Make these exact bounded substitutions in the following repeated rules:

| Base location | Activation wording |
| --- | --- |
| [Canonical vocabulary](../objects-and-governance.md#canonical-vocabulary), `Canonical semantic definition or policy` row, sentence beginning “Ordinary project policy candidates require” | “Independent contract/evaluation definitions require owner establishment under G4; applicable project qualification requires Policy Admission Closure. Governing configurations use the bounded establishment lifecycle.” |
| Same table, `Policy Admission Closure` row, “one exact policy candidate” and its governing-basis description | Use “one exact project-qualification candidate”; append “Intrinsic validation and independent applicability may share this outcome but remain distinct propositions under G4; no fictitious configuration is bound when qualification is not applicable.” Retain its governed closure fields. |
| [Normative ontology invariants](../objects-and-governance.md#normative-ontology-invariants), bullet starting “All canonical definition and policy candidates follow” | “All canonical definition and policy candidates follow shared admissibility. Independent contract/evaluation definitions use owner establishment and applicability under G4. Applicable project qualification requires Policy Admission Closure; governing configurations use first establishment/activation under the fixed Handbook rule. Neither route admits a candidate's own rule or activates a policy by authorship.” |
| [Object classification](../objects-and-governance.md#object-classification), contract/evaluation definitions row, “permitted-use qualification under the active admission rules” | “owner-established permitted use and applicable project qualification under G4”. |
| Same table, `Policy Admission Closure` row, “closes admission or refusal under the exact governing configuration” | “closes required project qualification or refusal under the exact governing configuration”. |
| [Semantic responsibility map](../objects-and-governance.md#semantic-responsibility-map), `Definition and policy-admission owners` row, “resolving the active governing configuration and applicable owner-rule basis; deterministic Policy Admission Closures at the shared decision boundary” | “intrinsic owner establishment and independent-use applicability under G4; resolving the active governing configuration and owner-rule basis when project qualification applies; owner conformance outcomes and required project-qualification Policy Admission Closures at their exact decision boundaries”. |
| [Settled versus deferred](../objects-and-governance.md#settled-versus-deferred), first settled bullet, “under established admission rules” | “under exact owner rules and applicable project qualification under G4”. |
| Same settled list, bullet starting “deterministic policy-candidate admission as one owner-produced Policy Admission Closure” | Replace that opening with “intrinsic owner establishment and applicable project qualification through one owner outcome, including Policy Admission Closure when required”. Retain its prohibitions on unnecessary record splits, every-artifact requirements, authored-text waivers, open-ended judgment and self-admission. |
| [Ontology scenario corpus](../objects-and-governance.md#discriminating-scenario-corpus), `User- or agent-authored policy candidate` row | In inputs replace “has no Policy Admission Closure” with “has no intrinsic owner establishment or required project-qualification Policy Admission Closure”. In outcome replace “close admission or refusal” with “close intrinsic establishment and applicable project qualification, or refuse, under G4”. Missing owner establishment still prevents executable Assurance/Metric/Gate/reliance/authorization policy; the lack of project qualification alone cannot forbid a positively independent Metric/Gate definition. Keep exact source roles, accounting, approvals when required, and no authorship/bare-link substitution. |
| [Development history and contracts inspected as data](../objects-and-governance.md#development-history-and-contracts-inspected-as-data), final sentence “Unadmitted exploratory execution…” | “Exploratory execution without owner-established complete contract/evaluation definitions does not become canonical evaluation. Independent complete-contract evaluation under G4–G5 is owner-established and is not exploratory execution under an unfinished draft.” |
| [Decision Record to definitions, policies, and exceptions](../objects-and-governance.md#decision-record-to-definitions-policies-and-exceptions), sentence starting “For a definition, policy, or selector candidate” through “and supersession” | “For a definition, policy, or selector candidate, that includes intrinsic owner establishment and, when applicable project qualification is required, its Policy Admission Closure under G4. Exact source roles, scope, dependency/currentness modes, compatibility, required separately authorized approvals or exceptions, and supersession remain owner checks; a Decision Record cannot substitute for them.” |
| [Authored-relationship scenario corpus](../objects-and-governance.md#discriminating-scenario-corpus-1), `Decision prose resembles policy` row, sentence starting “No executable Gate or Operational Reliance Policy exists until” | “No executable Gate or Operational Reliance Policy exists until the responsible owner establishes complete supported semantics and any required project qualification under G4; required Policy Admission Closure binds the exact candidate, relevant governance-source revision and role, scope, dependency modes, required approvals or exceptions, complete accounting, and non-relaxable checks. An independently usable Gate needs intrinsic owner establishment without fabricated project qualification; neither a Gate nor an Operational Reliance Policy activates or assigns itself.” The supplied bare-link/incomplete candidate still refuses. |
| [Authored-relationship settled list](../objects-and-governance.md#settled-versus-deferred-1), bullet starting “user-, agent-, artifact-, and tool-authored policy candidates”, phrase “as permitted governance inputs that require deterministic owner-side Policy Admission Closure” | “as proposed inputs requiring intrinsic owner establishment and, when applicable, deterministic project-qualification Policy Admission Closure”. Retain exact source-role/accounting and anti-self-admission checks. |

Add one sentence to
[Policy candidates and governance sources](../objects-and-governance.md#policy-candidates-and-governance-sources):
“Shared admissibility distinguishes intrinsic establishment, applicable project
qualification, and governing assignment under G4; references do not perform any
of them.” Its source-role/currentness rules remain unchanged.

### A3 — Assurance and evaluation preparation

At [Admission before assurance selection](../assurance-and-evaluation-preparation.md#admission-before-assurance-selection),
retain the candidate inventory and owner-check requirement; after “shared policy
admission”, add “with intrinsic establishment, applicability and required project
qualification distinguished under G4”. In the following paragraph replace “for
those admissions” with “for the qualifications required by applicable project
governance”. Assurance Selection still cannot admit inputs or initialize governance.

At [Responsibility map](../assurance-and-evaluation-preparation.md#responsibility-map),
the `Policy-definition and admission owners` row, replace “deterministically
produce a Policy Admission Closure or refusal” with “deterministically establish
intrinsic conformance and, when applicable project qualification is required,
produce Policy Admission Closure or refusal under G4”. Preserve exact typed
vocabulary, required governance-source/approval/exception checks, accounting,
owner boundaries and prohibitions in that row.

At [Complete evaluation requirements](../assurance-and-evaluation-preparation.md#evaluation-requirements),
add G5's use table verbatim and this rule:

> `admitted` evaluation inputs mean exact owner-established, intrinsically complete
> definitions, with applicable project qualification satisfied under G4. Direct
> requirements may form while governance is positively deferred; no posture snapshot
> or assurance selector is fabricated. The common requirements closure MUST bind
> exact governance state/applicability and qualification when applicable, preserving
> them through Purpose, basis, final context, result, and explanation under G5.
> A damaged or uncertain established basis is not deferral. This route does not
> satisfy mandatory governed completion or authorize an action.

Retain the existing complete inventory, refusal rules, eight-row permitted-use/
source/intent table, no fallback, and fresh Purpose/occurrence rules. At
[Semantic owner placement and identity](../assurance-and-evaluation-preparation.md#semantic-owner-placement-and-identity),
replace “Their admission follows the shared active-at-decision rule” with
“Their owner establishment and any applicable project qualification follow G4;
required project qualification follows the shared active-at-decision rule”.
Keep authorization/capability/readiness ownership, launch checks and immutable bases.

### A4 — Evidence and evaluation truth

At [Responsibility map in metric/gate semantics](../evidence-and-evaluation.md#responsibility-map),
replace the opening sentence “All definition and policy admissions in this part…”
through its shared-policy-admission link with:

> All definitions and policies in this part, including Prospective Applicability
> and Canonical Evidence Set Formation Policies, require owner establishment under
> shared admissibility. Applicable project qualification uses its exact governing
> configuration, owner-rule basis, and active-at-decision boundary under G4.
> Independent complete-contract evaluation requires the same evidence/truth closure
> and preserves its exact governance applicability provenance under G5.

At [Measurement and metric semantics](../evidence-and-evaluation.md#measurement-and-metric-semantics),
add after the candidate shared-admission paragraph: “Owner establishment and
applicable project qualification follow G4; independent use cannot relax the
compatibility, numeric meaning, trust, accounting or closure requirements here.”
At the earlier [Semantic responsibility map](../evidence-and-evaluation.md#semantic-responsibility-map),
`Contracts definition authorities` row, replace “Deterministic Policy Admission
Closure” with “Owner establishment and applicable project qualification under G4,
including required deterministic Policy Admission Closure”. No evidence algorithm,
identity, occurrence, no-collection, claim, metric, gate or replay rule changes.

### A5 — Operational reliance and actions

At [Historical truth and present use](../operational-reliance-and-actions.md#historical-truth-and-present-use),
replace “Admission-governance initialization” with “Admission-governance first
establishment at initialization or after positive deferral under G3”. Retain the
setup exemption's exact bounded purpose and its separate propositions.

At [Operational Reliance Policy and basis formation](../operational-reliance-and-actions.md#operational-reliance-policy-and-basis-formation),
replace “These admissions bind” with “Required project qualifications bind” and
add: “Intrinsic definition establishment alone cannot activate or assign these
policies, authorize an effect, or supply missing governed-reliance inputs. G4's
independent evaluation route is not an independent governed-action route.”

At [Independently governed completion](../operational-reliance-and-actions.md#governed-completion),
after “absence is not exemption or development mode”, add:

> Positive deliberate governance deferral under G3 permits the independent
> authoring/validation/evaluation uses in G5. It does not establish a governed
> completion standard or produce a governed-completion Determination. When
> governance is established, its authorized owner relationship determines whether
> requirements apply to this actual effect/target/scope; mere policy admission is
> not assignment. Required missing or uncertain relationships still refuse the
> affected completion operation. Independent historical results remain useful but
> cannot be relabeled to fill this chapter's mandatory assessment role.

After [Authority provenance without a Handbook IAM platform](../operational-reliance-and-actions.md#authority-provenance-without-a-handbook-iam-platform)'s
advisory-authority paragraphs, add: “Initial human approval and routine delegated
operation follow G2. Owner authorization for each operation may be automatic under
approved boundaries; it is not inherently a new human approval request.” Keep
the mandatory assessment floor, actual-target coverage, atomic Determination,
separate authorization, through-commit checks, receipts, and uncertainty rules.

### A6 — Entry point and decision register

At sign-off, retain the September 30 approval history, record the new explicit
decision and scope in HR-007, and mark this amendment approved only to that scope.
Keep the entry-point amendment routing instruction; retarget prospective chapter
pointers to the approved rules. In [The complete path](../README.md#the-complete-path),
replace the first path line with “owner-established complete definitions + exact
governance applicability and required project qualification”. Replace “An admitted
contract is either” with “An owner-established complete contract is either”, and
append: “These run-intent values do not establish project qualification. Complete
direct contracts may be evaluated during positive governance deferral under G4–G5.”

Replace [Principal decision 2](../README.md#2-establish-policy-acceptance-requirements-before-accepting-a-proposed-policy)'s
first two paragraphs with:

> Handbook supplies an exact starting establishment rule and owner conformance
> floors. People approve initial project/Charter/posture choices and delegated
> boundaries, choosing project governance now or deliberately deferring it. When
> selected, governance activates exact versioned qualification requirements and
> replacement authority. Supplied choices are recommendations or replaceable
> defaults, not automatic project policy. Agents can apply approved rules without
> repeated prompts when their authority and required facts resolve.
>
> Project qualification must be established before dependent governed uses succeed;
> intrinsic owner establishment of complete contract/evaluation definitions does
> not inherently need it. Drafting, validating, qualifying, assigning a governing
> role, and authorizing an action are separate. Replacement cannot authorize itself.

At [Principal decision 3](../README.md#3-decide-what-must-be-proved-before-checking-whether-tools-can-prove-it),
replace “particular accepted contract” with “particular owner-established complete
contract” and append: “Its independent check can produce real results while
governance is deliberately deferred, with exact applicability provenance. It
cannot satisfy a later governed obligation by relabeling the result.” Do not
rewrite the other principal decisions or HR-001 through HR-006's historical records.

## Acceptance scenarios

These discriminate proposed semantics, not claims of executed product tests.

| Scenario | Required outcome |
| --- | --- |
| Human reviews evidence-based starting recommendations and declines governance. | Positive deferral on P/scope; no hidden default configuration. Initial approved Charter/posture and delegation remain attributable, not an implicit assurance mapping. |
| P deferred governance; C is a complete API compatibility contract with supported full definitions and authorized local checks. | Owner establishes intrinsic completeness and permitted use; direct evaluation reports real findings/pass/failure with deferred governance provenance. No Draft label solely for deferral; no fabricated project qualification or shipment assessment. |
| C omits a necessary evidence rule, or Engine cannot authorize its requested check. | Intrinsic validation or the dependent evaluation boundary refuses accurately; completeness cannot be inferred from syntax or deferral. Authorized ordinary correction/retry needs no automatic human prompt. |
| Agent performs repeated supported operations within the approved boundaries. | Each owner check remains exact and recorded; no repeated human prompt solely for admission/evaluation/authorization. A reserved product decision or unsupported authority expansion gets a specific escalation. |
| Human later approves first governance for eligible P/scope. | Validate/authorize/commit against P's current deferred basis; retain identity, authored content and old results. Concurrent establishment or uncertain prior effect cannot falsely succeed or bootstrap again. |
| Shipment now requires governed assurance, and the agent submits the earlier direct API pass. | Preserve the pass; refuse its substitution for the mandatory assessment. Qualify/select the applicable contract and run the required posture-selected decision-seeking assessment, then check reliance and separate delegated action authority through commit. No generic status-setter bypass. |
| A complete contract is intrinsically valid but fails applicable project qualification. | Preserve its intrinsic validity and history; refuse the dependent governed use with exact reasons. It does not govern that operation or become incomplete by implication. |
| Established governance applies admission qualification to direct API checks; agent requests a fresh contract or diagnostic/direct intent to evade it. | Apply the same required qualification to fresh/successor uses; refuse the evasion. Direct intent is a requirements source, not a governance exemption. |
| Governance is absent because established state is unreadable, or activation timed out. | Do not label deferred or initialize again. Refuse dependent executable use/resolve uncertainty; exact content may still be inspected without claiming permission. |
| An admitted policy is unassigned, or a new Agent Skills pack is installed. | Neither fact makes it govern every operation or grants authority. Required assignment, owner qualification and activation remain separate. |
| Governance changes after an independent or governed result closes. | Replay preserves its exact original basis and outcome. New current-use qualification/reliance is distinct; old direct results do not acquire posture-selected provenance. |

## Open decisions

Human semantic sign-off must address these specific proposed answers. None is
silently delegated to implementation or converted to authority by this draft.

| Decision | Proposed answer | Alternative and consequence |
| --- | --- | --- |
| Admission split and independent permitted-use reach | Approve G4's intrinsic establishment plus applicable project qualification; retain both permitted-use values for exact supported independent questions. `Decision-seeking eligible` permits intent, not governed reliance. | Restrict independent use to Diagnostic-only until governance exists. Simpler qualification boundary, but imposes a project-governance ceiling on otherwise complete direct decision questions. Rejecting all independent canonical evaluation fails the clarified workflow. |
| Mandatory governed-completion provenance | Retain the approved posture-selected/decision-seeking floor for this amendment. Earlier direct results can inform development and declared supplemental roles; a later qualifying assessment establishes the governing standard. | Permit an authorized governing rule to accept an earlier direct assessment in the mandatory role after exact qualification/coverage. This can avoid reruns, but materially changes the approved source/intent floor and needs additional bounded rules before approval. No such exception is included here. |
| Governance applicability and later-establishment authority | Approve positive project/scope deferral, explicit owner applicability and first establishment on that same identity under G3. Initially approved instruction/delegation supports only the permitted independent checks; later initial governance is human-approved. | Treat missing configuration as deferral, or let recommendations activate. Easier setup but conflates damaged/uncertain state with permission and permits self-authorization; incompatible with the clarified intent and preserved authority boundaries. |
| Replacement of mixed supplied definitions | Approve behavior-level classification and explicit supported replacement under G1, retaining owner floors and initially approved boundaries. | Freeze the current contents as permanent policy, or remove mechanisms with the choices. The former conflicts with user direction; the latter loses truthful interpretation/authority. Exact preset contents, owner criteria, providers, resource bounds, APIs and representation remain later bounded selections, not approved defaults here. |

No policy engine, renderer/importer, IAM platform, preset catalog, pack system,
adapter implementation, schema/format migration, or successor coding slice is
selected. Existing Contracts JSON ownership and the proposed status of the
[structured-data standard](../../structured-data-format-standard.md) remain.
