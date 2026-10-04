# Governance defaults, delegated operation, and independent contract use

**Status: Proposed for human semantic sign-off.**

Prepared 2026-10-03 against `f62b1f450cacc31057280723970dc95ad18ad664`.
Revised 2026-10-03 above the preserved proposal baseline
`bf27981ff059bfb970e2c851e980599daccccabc` for bounded present-use qualification.
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
content, and history. Having seen results does not prohibit this setup. They
approve shipment requirements grounded in the actual obligation, a supported
assessment protocol, its finite eligible result boundaries, and delegated
shipment authority. The requirements cannot be chosen because C passed; relevant
failures must be accounted for too. Activation alone qualifies no old result.

Suppose C@1 answered the exact API question for target T1 with `direct_contract`
requirements and `decision_seeking` intent, producing R1. If the independently
selected shipment requirements include that exact question and the adopted
protocol permits retrospective acceptance, the action owner can produce a new
determination accepting R1 now for the declared API role after exact qualification,
coverage, currentness, historical authorization and complete-accounting checks.
R1 still says shipment was not assessed when it ran. Other shipment roles must
also be satisfied. A new governing assessment may thus use R1 without repeating
its observations; it is neither a relabeled R1 nor imported evidence.

If the target is now T2, coverage is missing or stale, or the protocol requires
governing approval before collection, the owner identifies the specific new
assessment or observation work needed. No automatic rerun or zero-rerun promise
applies. Once all current reliance conditions and separate delegated action
authority resolve, the agent may carry out the exact shipment action with checks
through commit and an authoritative receipt, without another prompt solely for
an owner operation. A reserved product choice or missing authority is escalated.

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

Direct source inspection also bounds the independent-operation claim:
[Context Resolution](../../../../../crates/engine/src/context_resolution.rs),
`load_authority_snapshot`, `ContextResolutionAuthorityAdmissionResolver::admit`
and `ContextResolutionEnvelope::resolve_root`, currently requires an exact
committed authority artifact, registry/publication lineage, mapped credential
assertion and current authority, plus exact profile/stack and constraint bindings.
`resolve_shipped_profile_decisions` in the linked profile request source validates
selected decisions through the fixed Charter registry; ordinary init alone does
not produce these artifacts or admissions. These are current source constraints,
not evidence that the proposed initial delegation path is delivered or that
project admission governance is the same as Engine operating authority. G4's
end-to-end requirement is target semantics for a supported local path. GitNexus
query/context was degraded (foreign index storage and failed search); this
inventory relies on direct inspection and claims no graph verification.

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
as the umbrella and separates its responsibilities. It MUST NOT become one
combined completeness, qualification, assignment, or permission status:

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

At least the supported independent local-check path's complete required
definition, capability and operating-authority chain MUST be satisfiable under
exact owner rules and the initially approved instruction/delegation without
project admission governance. No additional governance setup or human approval
is required solely because another owner validates or authorizes an in-scope
operation. Missing capability, unsupported definitions, out-of-scope reach and
required host restrictions still refuse at their actual boundaries. This is not
a promise for arbitrary tools or unsupported authority modes.

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
| Execute a complete direct contract and report its result | Owner-established contract and compatible full definitions; positive governance applicability resolution under G4; complete direct requirements; compatible intent; independent Engine authorization from approved operating authority; valid capability/readiness; exact collection, evidence, and truth closure. G4 requires a satisfiable supported local path under initial delegation without project admission governance; neither deferral nor the contract supplies execution authority. |
| Establish/admit project governance policies and assign them | First-establishment or prior replacement authority, applicable configuration and owner-rule qualification, exact authorized assignment, scope and currentness. Drafting, validating, admitting, packaging, or discovering a policy does not activate or assign it. |
| Select project posture assurance for a governed obligation | Actual scoped posture and qualified admitted selector/policy/exception inputs, governing relationship and complete selection. A Charter alone does not invent policy mappings; this governed route cannot fall back to independent evaluation when inputs are missing. |
| Use results to discharge governed shipment/completion or authorize its effect | Applicable qualified policies, independently resolved actual target, governing requirements and supported assessment protocol; mandatory assessment qualification/coverage, present reliance, separate action authority and through-commit checks. An earlier direct result may fill declared current roles only through the new owner determination below; it supplies no exemption or authority. |

#### Supported assessment protocols and current qualification

Work Progression or the relevant action domain MUST independently resolve the
actual project, effect, target, scope, governing assignment, complete applicable
requirements and accepted assessment protocol. Authoritative work-state owners
supply target/relations; Engine supplies any required posture; Contracts supplies
exact requirements and historical truth. Admission is not assignment. A submitted
result, available capability or favorable outcome MUST NOT select the standard.

An explicitly adopted, supported governing protocol MAY require prospectively
governed assessment, or permit bounded retrospective acceptance of an exact
earlier direct result for declared current assessment roles. These are owner
protocol choices under exact qualification and activation authority, not a
caller-created waiver, typed exception to a floor, or generic status-setter path.
The mandatory positive assessment still requires a **Decision-seeking eligible**
contract and original `decision_seeking` intent. Diagnostic results retain all
existing role/direction restrictions and cannot fill that mandatory role.

| Protocol | Required timing and provenance |
| --- | --- |
| Prospectively governed assessment | Resolve the exact actual target/selection relationship before collection or no-collection commitment. Use `posture_selected` requirements under the applicable governing posture and admitted assurance rules, with original decision-seeking intent. Current reliance independently verifies that relationship and coverage. A protocol requiring this route rejects an earlier direct result for that role, even if it passes. |
| Bounded retrospective acceptance | First resolve current governing requirements and the expressly adopted acceptance protocol independently of the proposed results. Then the action-domain reliance/coverage responsibility qualifies exact earlier `direct_contract`, decision-seeking truth for named current roles, through a NEW present-use determination. No governing relationship before that earlier collection is asserted. Any collection-time approval or prospective provenance required for the role must actually exist; current approval cannot manufacture it. |

For retrospective acceptance, the owner MUST establish and bind in the new
Operational Reliance Basis and atomic Determination:

- exact active governing configuration, qualified applicable policies, authorized
  selection/assignment, actual obligation/target/scope and complete current
  requirements, protocol and role inventory;
- each exact owner-resolved earlier result, its original permitted use, source,
  intent, subject/snapshot, observations, evidence closure, definitions, semantic
  times, historical validity and Engine authorization provenance;
- exact supported coverage of each current role by the selected contract revision,
  requirement and evaluator/evidence/metric/gate definitions, subject/target and
  scope relation. Matching names, copied passes or a general judgment of semantic
  equivalence are insufficient. Only owner-supported exact identity/role relations
  qualify; unsupported relations or changed required evaluation content need new
  work, not an inferred equivalence mechanism;
- every required present-currentness observation and process condition, including
  collection-time conditions and the explicit historical Engine validity versus
  continuing-current mode; and
- the finite eligible result boundary and complete accounting under the adopted
  protocol, including relevant failures and every required missing role. Additional
  requirements cannot disappear because one accepted role passes; composition of
  results is permitted only under exact declared owner rules.

This qualification belongs to present reliance, not policy admission of a result.
It may be carried by the existing Determination without a new record family.
Governance activation and unchanged-contract project qualification are separate;
neither qualifies R1 for a current role by itself. Qualification alone does not
require a new contract revision; changed content or permitted use still does.
Missing, stale, incompatible or unverifiable required formation facts cause
Reliance Refusal. A valid qualifying result with an unfavorable outcome follows
the declared non-positive outcome rules; acceptance does not mean a pass. A
positive result for one role does not prove overall `eligible / allow`.

Accepting R1 now answers a new governing question. A new governing assessment
may consist of this current qualification plus other required role outcomes;
it need not repeat original observations solely because governance was established.
Forming a new evaluation occurrence remains different: changed source, intent or
fixed evaluation inputs require its new Purpose and occurrence, with its own
collection/complete-pruning and evidence closure. Earlier evidence cannot be
copied into that closure. Existing admitted prior-fact roles remain bounded;
general cross-occurrence evidence import and semantic-equivalence machinery stay
deferred. New work is required for the earliest unsatisfied boundary: changed
target, stale facts, missing coverage or required prospective provenance can
require new verification, assessment or observations. Neither automatic reruns
nor automatic zero reruns is promised.

Governing applicability, actual target, required selection/qualification basis,
coverage and protocol conditions MUST be verified at reliance and preserved
through the completion commit boundary. An invalidating change stops the affected
reliance/action and requires new owner results without rewriting history. Every
operation discharging the same obligation remains subject to those checks.
Separate congruent action authorization, attempt checks, authoritative commit
truth and uncertainty/recovery rules remain unchanged.

#### Establishing governance after results are known

Prior knowledge of results alone MUST NOT prohibit first governance establishment
on a positively deferred scope. G3's fixed establishment rule, initial human
approval and exact owner validation supply its authority; a favorable result
supplies none. This is a bounded replacement of the existing unconditional ban
on policy selection/configuration after favorable inspection absent a predeclared
higher-order rule. Ordinary result-driven switching still refuses.

Before activation, the responsible owners MUST close requirements from the actual
obligation, approved Charter/posture and authoritative sources; the human approves
the exact initial requirements, protocol, finite eligible populations and delegated
boundaries under G2. Result knowledge and selection provenance MUST be retained.
The rule MUST have an independently justified requirement and population basis
that does not depend on obtaining a favorable outcome. Lowering requirements,
altering target/scope, moving time cuts or excluding candidates because a known
result fails is prohibited. Favorable inspection cannot itself justify any choice.

Each consumed role's population MUST bind permitted owners/resolvers or justified
exact references, kinds/intents, subject/target/scope/occurrence/time relations,
currentness modes, cardinality/order/composition and deterministic inclusion and
exclusion rules. Before qualification the owner MUST completely account for every
matching result and required role: selected, excluded (with rule and reason),
missing, duplicate, conflicting, unavailable and refused. Known relevant failures
cannot be hidden outside an artificially narrowed boundary; unresolved completeness
refuses. Exact references are allowed only with the same independent boundary
justification and accounting, not as a hand-picked passing subset.

Activation commits the standard, not shipment eligibility. A separate new
determination applies it to results, whether favorable or unfavorable. No earlier
higher-order rule is required solely to establish the first standard on an eligible
deferred scope. After establishment, authorized policy replacement still uses
the exact prior replacement terms and predecessor; new terms cannot authorize
themselves. A later result-informed replacement or policy choice is supported
only when prior authority expressly permits that process, owner validation applies
the same independent-basis and complete-accounting safeguards, and it is not
tailored to secure a pass. Otherwise retain the existing predeclared higher-order
choice requirement/refusal. A higher-order rule also cannot substitute a
non-governing standard. Material ambiguity in obligation, boundary or authority
requires specific escalation, not an agent's unconstrained judgment or silent waiver.

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

Governance establishment/replacement MUST NOT rewrite a historical result and
does not by itself establish that it satisfies a new obligation. Only a separate
current owner determination under the adopted protocol can do that. Original
source, intent, subject, observations, evidence closure, authority and governance
basis remain historical truth. A still-qualifying assessment need not be rerun for
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
Location names and quoted starts refer to `bf27981ff059bfb970e2c851e980599daccccabc`
(the base passages are unchanged from preparation). The prospective
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
> by itself satisfy mandatory governed completion or authorize an action. Current
> qualification of an earlier result follows the independently adopted G5 protocol;
> it changes neither this original requirements closure nor its occurrence.

Retain the complete inventory, formation compatibility in all eight rows, refusal,
no-fallback and fresh Purpose/occurrence rules. Make these exact completion/timing
edits at the same [requirements section](../assurance-and-evaluation-preparation.md#evaluation-requirements):

| Base location | Activation wording |
| --- | --- |
| Compatibility table, `Decision-seeking eligible / direct_contract / decision_seeking` row, final column `No.` | “Potentially, only through an explicitly adopted supported retrospective protocol and new current owner qualification under G5, with actual governing requirements/coverage and all reliance/action checks. No automatic completion.” |
| Same table, `Decision-seeking eligible / posture_selected / decision_seeking` row, final column | “Potentially, with the actual prospectively resolved governing selection, coverage, adopted protocol and all reliance/action checks under G5. Source alone is insufficient.” |
| Paragraph beginning “The following combinations assume”, after its last sentence | Add: “Completion-role acceptance is a later owner decision under G5, not formation compatibility or a change to original source/intent. Every diagnostic or incompatible row remains excluded from the mandatory positive role.” |
| Paragraph beginning “A held or refused project-selected evaluation”, sentence “A deliberately separate admissible development check may proceed but does not discharge the original obligation.” | “A deliberately separate admissible development check may proceed but does not by itself discharge the original obligation. Only independently adopted G5 qualification can accept an exact earlier decision-seeking result for a declared role; a held/refused route cannot invent that protocol or replace required prospective provenance.” |
| Paragraph beginning “Required formation and truth-affecting boundary/time rules” | Retain its original evaluation-stage timing. Append: “G5's new current-use qualification has its own independently fixed governing requirements/protocol and finite result accounting; it does not select or revise the original run's requirements or truth-affecting rules after observations.” |

In [Canonical model and vocabulary](../assurance-and-evaluation-preparation.md#canonical-model-and-vocabulary-1),
replace the paragraph beginning “Both requirements resolvers retain” through
“does not by itself establish that governing relationship” with:

> Both requirements resolvers retain their separately attributable responsibilities
> from complete evaluation requirements. For the G5 prospectively governed route,
> a request intended to discharge governed completion resolves the actual governing
> target and assurance-selection relationship before collection or no-collection
> commitment. No future completion Determination or Action Intent is needed.
> An arbitrary posture-selected result does not establish governing assignment.
> For supported retrospective acceptance, the action domain instead resolves the
> current relationship, requirements and protocol before new current qualification;
> the earlier evaluation retains its original timing and source. Required prospective
> provenance cannot be fabricated. Missing governed inputs cannot select this route
> as a fallback.

At the later [Responsibility map](../assurance-and-evaluation-preparation.md#responsibility-map-1),
`Work Progression and other Operational Reliance owners` row, replace “including
mandatory governing-completion selection and assessment coverage where applicable”
with “including independently resolved governing-completion requirements, adopted
protocol, selection/qualification and complete assessment coverage under G5 where
applicable”. Its prohibition on rewriting truth and diagnostic intent remains.
At [Settled versus deferred](../assurance-and-evaluation-preparation.md#settled-versus-deferred-1),
after “permitted posture-selected diagnostics without completion substitution”,
add “and protocol-specific prospective assessment or current qualification under G5”.
Keep the diagnostic scenarios and C1/C2-to-C3 example: neither diagnostic findings
nor another contract's pass become C3's historical pass.

At
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

At [Canonical evidence-set formation and closure](../evidence-and-evaluation.md#canonical-evidence-set-formation-and-closure),
after the paragraph beginning “Exact replay of an already accounted occurrence”,
add: “G5 permits new present-use qualification of an exact old result under a
supported governing protocol. It creates no new evaluation truth and transfers
no old-occurrence record into a new evidence closure.”

At [Final evaluation-context closure](../evidence-and-evaluation.md#final-evaluation-context-closure),
after the paragraph beginning “A new posture-selected occurrence cannot adopt”,
add: “A new current governing assessment under G5 may instead qualify exact
existing results for declared roles. It is not a new posture-selected evaluation
occurrence and does not adopt the earlier authorization, observations or evidence
as its own collection. Original source/intent and all occurrence boundaries remain.”
In its requirements-source cases, the `Exact eligible contract evaluated directly,
then selected by posture with equal rules` row retains the new-Purpose/occurrence
outcome when a new evaluation is requested; append: “Separately, G5 current
qualification may accept the unchanged earlier result for an exact role when
expressly supported; it neither relabels nor imports that occurrence.”

At [Responsibility map in metric/gate semantics](../evidence-and-evaluation.md#responsibility-map),
`Work Progression and other Operational Reliance owners` row, replace “within the
mandatory governing-completion selection and assessment boundary where applicable”
with “within G5's independently resolved governing requirements, adopted protocol,
selection/qualification and complete assessment boundary where applicable”.
Existing pre-existing fact/applicability roles, source terminals, sealed frontiers,
evidence-set membership and final-context cardinality are not expanded.

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

In that section replace the entire paragraph beginning “A policy MUST NOT be
selected, replaced, or configured after inspecting a favorable result direction”
with G5's **Establishing governance after results are known** subsection verbatim.
Retain “An Operational Reliance Policy MUST declare before result direction is
used” and the existing finite-boundary bullets and exact-once accounting rule:
they govern application/composition, while the added text defines the bounded
setup and previously authorized replacement treatment. Add to the accounting
paragraph: “A nominated-reference boundary still requires independent justification
and complete accounting under G5; it cannot hide relevant known failures.”

At [Independently governed completion](../operational-reliance-and-actions.md#governed-completion),
replace the governing-relationship bullet “the required Assurance Policy Selector,
Posture Scope, and assurance-selection basis for the mandatory assessment” with
“the complete actual requirements, adopted supported assessment protocol and
role inventory, plus required Assurance Policy Selector, Posture Scope and
assurance-selection basis where applicable, and current qualification basis
for accepted earlier results under G5”.

Replace “Engine supplies posture; Contracts Assurance Selection supplies the
requirements” with “Engine supplies required posture; Contracts Assurance
Selection supplies current governing requirements through the applicable exact
mapping, while direct requirement resolution and evaluation owners preserve
the earlier result's own requirements and truth. The action domain owns G5's
current qualification and cannot reconstruct those propositions”. Missing
governing mappings still refuse; an earlier result cannot supply them.

After “absence is not exemption or development mode”, add:

> Positive deliberate governance deferral under G3 permits the independent
> authoring/validation/evaluation uses in G5. It does not establish a governed
> completion standard or produce a governed-completion Determination. When
> governance is established, its authorized owner relationship determines whether
> requirements apply to this actual effect/target/scope; mere policy admission is
> not assignment. Required missing or uncertain relationships still refuse the
> affected completion operation. Independent historical results cannot be relabeled;
> only G5's separately adopted protocol and new current owner determination may
> accept an exact earlier result for declared current assessment roles.

Replace the block beginning “A request for the required assessment MUST resolve”
through “A still-applicable qualifying assessment need not be rerun for each
completion request” with G5's **Supported assessment protocols and current
qualification** subsection verbatim. This replaces the universal original-source
and pre-collection floor with protocol-specific requirements, not optional
governing applicability. Immediately after that block add:

> Ordinary policy, typed exception, wrapper, child result, higher-order selection
> or display label MUST NOT waive exact contract/intent, governing assignment,
> supported protocol, coverage, qualification or required process conditions.
> Properly authorized exceptions remain bounded by their existing owner rules;
> they cannot change historical provenance or fabricate collection-time authority.
> Supplemental development and diagnostic results may serve declared roles but
> cannot substitute for the mandatory assessment contrary to G5. An exact diagnostic
> failure may support a permitted remediation action; it is not completion of the
> original work. Still-applicable qualifying assessments need not be rerun for
> every completion request.

Retain the remainder beginning “Action authorization and attempt admission”,
including target/effect congruence, through-commit preservation, generic operations,
parent coverage, historical/continuing Engine modes, advisory authority and the
limit on substantiating adequate engineering requirements by provenance alone.

After [Authority provenance without a Handbook IAM platform](../operational-reliance-and-actions.md#authority-provenance-without-a-handbook-iam-platform)'s
advisory-authority paragraphs, add: “Initial human approval and routine delegated
operation follow G2. Owner authorization for each operation may be automatic under
approved boundaries; it is not inherently a new human approval request.” Keep
the revised mandatory assessment constraints, actual-target coverage, atomic Determination,
separate authorization, through-commit checks, receipts, and uncertainty rules.

Reconcile these exact repetitions in the same activation:

| Base location | Activation wording |
| --- | --- |
| [Work Progression ownership](../operational-reliance-and-actions.md#work-progression-ownership), bullet beginning “independently established governing-completion applicability, policy/assessment selection and target coverage” | “independently established governing-completion applicability, actual requirements and adopted protocol, prospective selection or current qualification and complete target/assessment coverage under G5; action-specific policy selection and complete candidate accounting”. Retain its ownership and non-mutation boundaries. |
| Same section, complete-pruning paragraph, sentences beginning “The actual governing role still requires” through “Direct or diagnostic provenance cannot fill that mandatory role.” | “The actual governing role still requires original Decision-seeking eligible/decision-seeking provenance, exact target/coverage and all required currentness under G5's adopted supported protocol, followed by exact outcome-policy interpretation. Required prospective posture-selected provenance remains mandatory under that protocol; only an expressly adopted retrospective protocol and new current owner qualification may accept an exact earlier direct result for declared roles. Diagnostic provenance cannot fill the mandatory positive role.” Preserve the paragraph's no-observations/no-pass/no-automatic-eligibility rule, exact all-N/A outcome, no universal N/A mapping, parent assessment/actual child-closeout limits, and governing conditions/action checks through commit. |
| [Currentness, invalidation, and replay](../operational-reliance-and-actions.md#currentness-invalidation-and-replay), `Governing completion selection, target coverage, or required assessment relationship changes` row | Replace “changed evaluation requirements or acceptance relation require a fresh qualifying assessment” with “changed requirements or acceptance relations require new current qualification or the specific new evaluation/observation work required by G5; unchanged old results qualify only under exact supported coverage/protocol rules”. Retain stop/refusal and no historical-only governing dependency. |
| [Semantic responsibility map](../operational-reliance-and-actions.md#semantic-responsibility-map), `Work Progression` row, opening phrase through “assessment coverage” | “Independently established governing-completion applicability/requirements and adopted protocol; prospective selection or current qualification of exact earlier results and complete assessment coverage under G5”. No ownership transfer or action authority. |
| Same map, `Other downstream action domains` row, “under the common floor” | “under the common floor and G5's governing requirements/protocol/current-qualification rules when applicable”. |
| [Discriminating scenario corpus](../operational-reliance-and-actions.md#discriminating-scenario-corpus), `Historical pass, current local closeout` row, phrase “posture-selected requirements” | “protocol-compatible assessment provenance under G5”. Preserve independently resolved relationship, exact coverage and every remaining condition. |
| Same corpus, `Ambient favorable-result selection` row | Retain refusal and exact boundary/accounting; append “First governance after results exist is allowed only under G5's independent standard, justified boundary and complete failure accounting, never by requesting the latest passing gate.” |
| Same corpus, additional governed-completion table, `An agent submits a direct pass ... as the required final assessment` row | “An earlier direct pass alone causes Reliance Refusal. Under an expressly adopted retrospective protocol, new G5 qualification may accept the exact result for declared roles; required prospective provenance or unsupported/missing coverage refuses. Permitted use is not assignment.” |
| Same additional table, `A required mapping is unavailable ...` row | Retain unchanged: no missing-governance exemption, narrower-child substitute or ambient direct fallback. G5 requires an actual supported mapping/protocol. |
| Same additional table, `Complete-pruning result considered for governed completion` row | Replace “posture-selected” with “protocol-qualified under G5”; replace “Direct/diagnostic or wrong-target substitution refuses” with “Unqualified direct, diagnostic or wrong-target substitution refuses; no prospective provenance is fabricated”. Retain exact outcome policy, all-N/A `not_applicable / no_effect` and no action authority. |
| Same additional table, `Governing requirements change after a qualifying pass ...` row | Replace “obtain the newly required assessment and owner decisions” with “obtain new qualification or specific newly required assessment/observation work under G5 and new owner decisions”. Retain stop on lost coverage and no invalidation from an unused definition alone. |
| [Settled versus deferred](../operational-reliance-and-actions.md#settled-versus-deferred), bullet beginning “independently established governing-completion applicability/selection” | “independently established governing-completion applicability/requirements, actual target coverage, mandatory Decision-seeking eligible/decision-seeking assessment roles under the adopted supported protocol, prospective governing selection or new current qualification under G5, exact outcome-policy treatment of complete-pruning results, and governing conditions through commit”. |

Add G5's unchanged-target, changed-target, stale/missing-coverage, prospective-
provenance, known-failure/setup and replacement cases from the acceptance table
below to the additional governed-completion table. Retain diagnostic cases.

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

In that reading map replace the two-line step starting “resolve governing
target/assessment relationship before commitment” with “resolve governing
target/assessment relationship before commitment for the prospectively governed
protocol under G5”. Replace the later current-relationship step with “resolve
current governing requirements/protocol and verify assessment coverage or qualify
exact earlier results under G5, when applicable”. The evaluation chain still forms
new truth only through its own occurrence; qualification is a later reliance step.

Replace the paragraph beginning “For governed completion, the responsible action
owner independently identifies” with:

> For governed completion, the action owner independently identifies the actual
> project standard, target and supported adopted assessment protocol. Mandatory
> positive roles require original Decision-seeking eligible/decision-seeking truth.
> A prospective protocol requires governing selection before collection or
> no-collection commitment and posture-selected requirements. An expressly adopted
> retrospective protocol may accept an exact earlier direct result through new
> current owner qualification under G5. Neither setup nor a pass supplies assignment
> or acceptance. Diagnostic results cannot substitute, and all required coverage,
> process conditions, separate authority and through-commit checks remain.

Keep the following C1/C2/C3 diagnostic-development example and prohibition on
transferring its evidence or passes; it is not a case of qualifying the same
unchanged eligible contract/result.

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
rewrite HR-001 through HR-006's historical records. In that decision's second
paragraph replace “Direct and diagnostic runs cannot replace the project-selected
assessment required for governed completion” with “Direct results can fill declared
current roles only after independently adopted G5 qualification; diagnostic runs
cannot fill the mandatory positive assessment role. Required prospective provenance
cannot be supplied retrospectively”. Keep the example with missing security coverage.

At [Principal decision 10](../README.md#10-qualifying-to-close-a-task-does-not-close-it-or-qualify-its-parent),
replace its first paragraph after the opening Work Progression sentence with:
“For governed completion the action owner independently resolves the actual target,
requirements and adopted protocol. Prospective assessment resolves the governing
relationship before collection/no-collection commitment; retrospective acceptance
uses new current qualification of exact earlier results under G5. Mandatory positive
roles retain original Decision-seeking eligible/decision-seeking provenance. Both
routes require current complete coverage through commit and separate action authority.”
Retain the opening owner/non-mutation sentence and child/parent example.

## Acceptance scenarios

These discriminate proposed semantics, not claims of executed product tests.

| Scenario | Required outcome |
| --- | --- |
| Human reviews evidence-based starting recommendations and declines governance. | Positive deferral on P/scope; no hidden default configuration. Initial approved Charter/posture and delegation remain attributable, not an implicit assurance mapping. |
| P deferred governance; C is a complete API compatibility contract with supported full definitions and authorized local checks. | Owner establishes intrinsic completeness and either supported permitted-use value; direct evaluation reports real findings/pass/failure with deferred governance provenance. At least the supported local path's full definition/capability/Engine authority chain resolves under initial delegation, without hidden governance setup or another approval solely for owner validation. No Draft label solely for deferral; no fabricated project qualification or shipment assessment. |
| C omits a necessary evidence rule, or Engine cannot authorize its requested check. | Intrinsic validation or the dependent evaluation boundary refuses accurately; completeness cannot be inferred from syntax or deferral. Authorized ordinary correction/retry needs no automatic human prompt. |
| Agent performs repeated supported operations within the approved boundaries. | Each owner check remains exact and recorded; no repeated human prompt solely for admission/evaluation/authorization. A reserved product decision or unsupported authority expansion gets a specific escalation. |
| Human later approves first governance for eligible P/scope. | Validate/authorize/commit against P's current deferred basis; retain identity, authored content and old results. Concurrent establishment or uncertain prior effect cannot falsely succeed or bootstrap again. |
| Current governing requirements select the exact unchanged eligible C@1/API question for T1; the adopted supported retrospective protocol permits earlier R1 with original direct/decision-seeking provenance. | Resolve current requirements/assignment independently, qualify applicable definitions/use, account for the entire eligible result population and all required roles, and form a new owner determination accepting R1 now for the API role if exact coverage/currentness/process conditions verify. R1 still reports no historical shipment assessment. Other roles, outcome policy, separate delegated action authority and commit checks remain. No new contract revision or repeated observations solely for governance establishment. |
| The action target changed from evaluated T1 to T2, even though contract names and displayed pass are unchanged. | No automatic rebinding or semantic-equivalence inference. Refuse unsupported target coverage; obtain the supported new assessment/observations for T2 and fresh current owner decisions. R1 remains exact truth about T1. |
| Target is unchanged, but a required current fact is stale, a shipment role lacks coverage, or a required identity is unavailable. | Refuse the affected formation boundary; identify new owner verification, assessment or collection needed. An accepted API role cannot hide missing security coverage. Do not manufacture evidence import, weaker requirements, automatic whole-suite reruns or automatic zero reruns. |
| Shipment's adopted protocol requires governing selection/approval before collection and original posture-selected requirements; R1 was independent. | Preserve R1; refuse it for that required role. Form the prospectively governed decision-seeking evaluation with its own Purpose/occurrence and required new work. Current approval does not fabricate original governance or collection-time authority; unrelated observations are not automatically repeated. |
| During first setup the user and agent already know Rpass and relevant Rfail; requirements/population are independently grounded in the actual obligation. | First establishment remains possible under G3 and initial human approval without an earlier higher-order rule. Bind exact justified finite boundaries, retain known-result/selection provenance and completely account for relevant failures, exclusions and missing roles. Activation supplies no eligibility; new current qualification applies the outcome rules to all required inputs. |
| A setup/replacement proposal lowers a requirement, narrows scope, shifts the time cut or nominates only Rpass because Rfail would block shipment. | Refuse result-fitting selection, even if a human liked the pass or the candidate calls itself safe. A favorable result supplies no authority or independent requirement basis. An ordinary higher-order selector cannot substitute a non-governing standard or erase failure accounting. |
| After establishment, an agent proposes a supported revision after inspecting results; prior approved terms explicitly delegate that revision process. | Validate prior authority and exact predecessor plus G5's independent requirement/population basis and complete accounting before activation. No repeat human prompt solely for an owner operation. A reserved change, missing prior authority or a successor claiming its own permission refuses/escalates; without the supported prior authorization retain the higher-order choice restriction. |
| A complete contract is intrinsically valid but fails applicable project qualification. | Preserve its intrinsic validity and history; refuse the dependent governed use with exact reasons. It does not govern that operation or become incomplete by implication. |
| Established governance applies admission qualification to direct API checks; agent requests a fresh contract or diagnostic/direct intent to evade it. | Apply the same required qualification to fresh/successor uses; refuse the evasion. Direct intent is a requirements source, not a governance exemption. |
| Governance is absent because established state is unreadable, or activation timed out. | Do not label deferred or initialize again. Refuse dependent executable use/resolve uncertainty; exact content may still be inspected without claiming permission. |
| An admitted policy is unassigned, or a new Agent Skills pack is installed. | Neither fact makes it govern every operation or grants authority. Required assignment, owner qualification and activation remain separate. |
| Governance changes after an independent or governed result closes. | Replay preserves its exact original basis and outcome. New current-use qualification/reliance is distinct; old direct results do not acquire posture-selected provenance. New evaluation occurrences keep separate evidence closures; qualification is not evidence import. |

## Open decisions

Human semantic sign-off must address these specific proposed answers. None is
silently delegated to implementation or converted to authority by this draft.

| Decision | Proposed answer | Alternative and consequence |
| --- | --- | --- |
| Admission split and independent permitted-use reach | Approve G4's intrinsic establishment plus applicable project qualification; retain both permitted-use values for exact supported independent questions. `Decision-seeking eligible` permits intent, not governed reliance. | Restrict independent use to Diagnostic-only until governance exists. Simpler qualification boundary, but imposes a project-governance ceiling on otherwise complete direct decision questions. Rejecting all independent canonical evaluation fails the clarified workflow. |
| Supported governed-assessment protocols | Approve G5's prospective route plus explicitly adopted bounded retrospective acceptance of exact earlier direct, decision-seeking results. Keep independent actual requirements, exact role/target coverage, currentness/process conditions, original truth, diagnostic exclusions and separate authority through commit. Owner qualification answers the new governing question; activation and passes do not. | Support only prospective posture-selected assessment. This deliberately rejects earlier direct results from mandatory roles even with unchanged exact coverage, and may require new observations. Allowing automatic grandfathering or generic equivalence instead would lose the proposed owner boundary and is not included. |
| Governance selection after results are known | Approve G5's bounded first-establishment treatment under fixed owner rules and human initial approval, plus supported later processes expressly authorized by prior terms. Independently justify actual requirements and finite populations, retain known-result provenance and complete failure accounting, and separate activation from qualification. | Retain the unconditional favorable-inspection ban except for previously predeclared higher-order choices. That can prevent normal first setup after independent work. Removing all safeguards would allow standards/populations chosen to fit a pass and is not proposed. |
| Governance applicability and later-establishment authority | Approve positive project/scope deferral, explicit owner applicability and first establishment on that same identity under G3. Initially approved instruction/delegation supports only the permitted independent checks; later initial governance is human-approved. | Treat missing configuration as deferral, or let recommendations activate. Easier setup but conflates damaged/uncertain state with permission and permits self-authorization; incompatible with the clarified intent and preserved authority boundaries. |
| Replacement of mixed supplied definitions | Approve behavior-level classification and explicit supported replacement under G1, retaining owner floors and initially approved boundaries. | Freeze the current contents as permanent policy, or remove mechanisms with the choices. The former conflicts with user direction; the latter loses truthful interpretation/authority. Exact preset contents, owner criteria, providers, resource bounds, APIs and representation remain later bounded selections, not approved defaults here. |

No policy engine, renderer/importer, IAM platform, preset catalog, pack system,
adapter implementation, schema/format migration, or successor coding slice is
selected. Existing Contracts JSON ownership and the proposed status of the
[structured-data standard](../../structured-data-format-standard.md) remain.
