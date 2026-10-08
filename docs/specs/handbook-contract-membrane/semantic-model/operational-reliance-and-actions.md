# HCM semantic model — Operational reliance and actions

**Status: APPROVED as semantic input — package approval 2026-09-30; HR-007 integrated 2026-10-04**

Part of the [combined semantic package](README.md). Its approval boundary and
shared greenfield/current-product distinction apply throughout.

The human-selected [Work Progression kinds and relations amendment](amendments/work-progression-kinds-and-relations.md)
supplements this part with the initial semantic composition, containment,
prerequisite, contract-role and scoped-governance rules. Its integration selects
no physical owner, concrete work-state lifecycle, policy content or implementation.

This part owns present-use reliance, Work Progression, and the boundaries between eligibility, action authorization, execution, and committed mutation. A shared file does not collapse those semantic owners.

## Reading map

- [Operational reliance, Work Progression, and actions](#reliance-actions)

<a id="reliance-actions"></a>
## Operational reliance, Work Progression, and actions

This chapter separates immutable historical truth from present reliance and from authority to act. Current HCM-5.2 local-closeout source is evidence for purity and owner separation, not a compatibility obligation.

### Purpose and scope

The canonical downstream chain is:

```text
immutable historical evaluation / claim / metric / semantic-gate truth
  -> exact action question: kind + target + effect + scope + eligibility-relevant parameters
  -> admitted Operational Reliance Policy
  -> action-specific Operational Reliance Basis at a present semantic time
  -> one Operational Reliance Determination
       - Current-Use Eligibility facet
       - fixed Operational Disposition facet
  -> exact Action Intent, only after `allow`
  -> Action Authorization Binding or Action Authorization Refusal
  -> exact Action Attempt Identity and attempt-bound checks, only after a Binding
  -> separately applied effect or mutation
  -> Action Terminal and, only after committed state change, Mutation Receipt
```

For work progression, two distinct reliance chains may exist:

```text
local work result(s)
  -> Local Closeout Operational Reliance Determination
       - Local Closeout Eligibility facet
       - fixed Operational Disposition facet

exact parent + complete child/dependency closure
  + exact child Local Closeout Eligibility results and other declared inputs
  -> parent-action Operational Reliance Determination
       - Parent Eligibility Result facet
       - fixed Operational Disposition facet
```

Neither chain performs a closeout or parent-state mutation. Parent eligibility
is not promotion. An actual local closeout, parent advancement, parent
promotion, release, deployment, posture transition, or other effect follows
its own separately authorized action path.

This record settles the common present-use vocabulary and the exact Work
Progression ownership needed for local closeout and parent eligibility. It does
not select any concrete closeout, parent, release, deployment, or posture
policy; thresholds; concrete realization of the amendment's selected hierarchy
and relations; work-state machine; or mutation.

### Canonical vocabulary

Terms in separate rows are not synonyms.

| Term | Fixed meaning |
| --- | --- |
| **Historical Semantic Result** | An immutable owner-produced evaluation, applicability, claim, metric, semantic-gate, explanation, or other semantic result for one exact historical input closure and semantic time. It remains exact history after its dependencies become non-current. It is not present permission, eligibility, disposition, or action authority. |
| **Operational Reliance** | A present-tense use of one or more exact Historical Semantic Results to decide whether a named action for an exact target may be considered now. Historical display, deterministic explanation, and exact replay that produce no present operational effect are not Operational Reliance. |
| **Action Kind** | An immutable, admitted, named, and versioned semantic definition owned by one downstream action domain. It defines one bounded effect family and the compatibility rules for its exact action question, such as local closeout, parent advancement, release, deployment, posture-observation admission, or posture transition. Equal verbs, labels, or UI actions do not make two Action Kinds interchangeable. It grants no authority. |
| **Action Target Snapshot** | The immutable owner-produced identity of the exact target state, scope, and applicable structural relations against which present reliance and any later action are decided. For parent eligibility it includes the exact parent subject and its complete child/dependency closure. It is neither a Work Specification revision nor an evaluation subject merely because either refers to the same work. |
| **Action Question** | The immutable bounded tuple closed before reliance from one exact Action Kind, one exact Action Target Snapshot and expected state, the exact bounded effect and effect scope, and every action parameter whose value can change eligibility under the Operational Reliance Policy. It is part of the Basis and Determination identities, grants no authority, and is not an Action Intent because it contains no actor or execution request. |
| **Operational Reliance Policy** | An immutable, admitted, named, and versioned action-domain rule that declares one Action Kind; accepted source-result kinds and use intents; exact target and subject-congruence requirements; complete present-use dependency and currentness modes; result-composition and indeterminacy treatment; and the rules that produce one Operational Reliance Determination with fixed Current-Use Eligibility and Operational Disposition facets. It grants no action authority. |
| **Operational Reliance Occurrence Identity** | The action-domain owner-issued identity of one deliberately intended present-use determination. It distinguishes repeated reliance at different times or for different intended actions even when the referenced results and policy are equal. It is not an evaluation occurrence, action authorization, transport correlation, or content fingerprint. |
| **Operational Reliance Basis** | The immutable, action-specific input closure for one Operational Reliance Occurrence Identity. It binds the exact action question—Action Kind, Action Target Snapshot, bounded effect, effect scope, and every eligibility-relevant action parameter—plus the Operational Reliance Policy, finite exact Historical Semantic Result population, accepted evaluation use intent, present reliance semantic time, every required owner resolution/currentness observation, complete included/excluded/missing/conflicting accounting, exact policy-selection provenance, and mandatory governing-completion applicability, assessment provenance, and coverage when that boundary applies. It contains no Action Authorization Binding and performs no mutation. |
| **Operational Reliance Determination** | The one immutable action-domain result produced from one exact Operational Reliance Basis under one occurrence, policy, action question, target, semantic time, identity, and decisive accounting closure. It contains Current-Use Eligibility and Operational Disposition as two fixed facets, grants no action authority, and performs no mutation. |
| **Current-Use Eligibility** | The semantic facet of one Operational Reliance Determination: `eligible`, `ineligible`, `indeterminate`, or `not_applicable`. It answers only whether the bound historical truth may support consideration of the exact action question for the exact target now. It is not independently formed, current, superseded, or replayed apart from its Determination. |
| **Reliance Refusal** | The owner-produced refusal to form a valid Operational Reliance Basis or Determination because a required identity, policy, target relation, owner resolution, currentness observation, use-intent rule, cardinality, or accounting boundary is missing, malformed, ambiguous, stale where currentness is required, substituted, conflicting, or unverifiable. It is not `ineligible`, `indeterminate`, or `not_applicable`. |
| **Operational Disposition** | The mechanically fixed control-facing facet of one Operational Reliance Determination: `allow`, `block`, `hold`, or `no_effect`. It is not a second policy decision or independently current result. `allow` permits only an Action Intent and request for separate action authorization; it neither performs nor authorizes the action. |
| **Action Intent** | The immutable, exactly identified requester-owned request to perform the action question already decided by one exact `eligible / allow` Operational Reliance Determination. It MUST be exactly congruent with that Action Kind, Action Target Snapshot, expected state, effect, effect scope, and every eligibility-relevant parameter. It may add only the actor or principal, claimed delegation or authority provenance, and execution-local details that the Operational Reliance Policy declares irrelevant to eligibility. Those claims remain claims until the action-authorization owner verifies or explicitly admits them under an advisory-authority rule. The Intent is untrusted until admitted and grants nothing by itself. |
| **Action Authorization Binding** | The immutable authorization-owner semantic result and provenance closure that binds the exact authorization-owner identity; owner-recognized authorization rule; declared authority mode and basis, including whether principal or delegation facts are claimed, owner-verified, or host-enforced; issuance semantic time and owner-verifiable lineage; one exact Action Intent; actor or principal; exact action question; exact `eligible / allow` Operational Reliance Determination; bounded conditions; and the rule governing whether one or more Action Attempts are permitted. It is not itself a credential, capability token, authenticated identity, delegation instrument, or proof of host enforcement. It cannot improve eligibility, broaden the Intent, or substitute for another authorization owner. Evaluation authorization cannot substitute for it. |
| **Action Authorization Refusal** | The immutable authorization-owner result that one exact Action Intent cannot be authorized, with the decisive reason, declared authority mode and basis, semantic time, and owner provenance. It creates no Action Authorization Binding, Action Attempt Identity, Action Terminal, or Mutation Receipt. |
| **Action Attempt Identity** | The exact action-executor-owned identity of one deliberately intended execution attempt under one exact Action Intent and Action Authorization Binding. It distinguishes exact delivery replay from a new attempt. It grants no authority and is not the Intent, Binding, Terminal, or Mutation Receipt. |
| **Action Terminal** | The action-executor-owned immutable durable provenance for one exact Action Attempt Identity under one exact Action Authorization Binding. It records the executor's exact attempt outcome, including a refusal after the attempt boundary was entered, completion without target mutation, execution failure, or effect/commit uncertainty. It may report a committed target-state mutation only by binding the exact authoritative state-owner Mutation Receipt. A pre-attempt Action Authorization Refusal is not an Action Terminal. |
| **Mutation Receipt** | The authoritative state owner's immutable proof that one exact Action Attempt committed the exact authorized mutation against the exact expected predecessor state and produced the exact successor state at the owner's commit semantic time. No terminal, acknowledgement, transport success, or executor assertion substitutes for it. |
| **Local Closeout Eligibility** | The Current-Use Eligibility facet of one atomic Work Progression-owned Operational Reliance Determination for the `local_closeout` Action Kind and one exact local work target. It is produced from its own Local Closeout Policy and Basis. It is not an independent result, the local closeout mutation, or a parent proposition. |
| **Parent Eligibility Result** | The Current-Use Eligibility facet of one atomic Work Progression-owned Operational Reliance Determination for one exact parent-target Action Kind, parent snapshot, and complete declared child/dependency closure. It has its own Parent Eligibility Policy and Basis. It is not an independent result, Local Closeout Eligibility, or parent promotion. |
| **Work Progression** | The downstream semantic responsibility that owns work-target and parent/dependency closure for reliance, Local Closeout and Parent Eligibility policies, Operational Reliance Bases, and atomic Determinations with fixed Current-Use Eligibility and Operational Disposition facets for those action kinds. It does not own Contracts truth, authored Work Specification content, or state mutation by implication. |

`Current` is not a free-standing status. Every currentness observation names its
semantic owner, exact object and proposition, use boundary, semantic time, and
verification rule. A current Historical Semantic Result is still not an action
authorization.

### Ontology placement

The objects introduced by this record retain the six ontology categories fixed
by the canonical-object ontology decision:

- No object introduced by this record is an authored Artifact Instance. A Work
  Specification remains an existing authored artifact when it references a
  result from this seam.
- Action Kind definitions; Operational Reliance Policies; Local Closeout
  Policies; Parent Eligibility Policies; and separately named Action
  Authorization policies are canonical semantic definitions or policies.
- Action Target Snapshots; Operational Reliance Bases and Determinations;
  Reliance Refusals; Action Authorization Bindings and Refusals; Local Closeout
  Eligibility and Parent Eligibility Results as facets of their Determinations;
  and Mutation Receipts are owner-produced canonical records or results.
- A closed Action Intent, Action Attempt Identity and its durable attempt
  occurrence, and Action Terminal are durable request or attempt provenance.
- An unclosed action request, in-progress attempt, or transport-local action
  failure before an owner occurrence closes is transient operational state.
- Renderings, summaries, compact explanations, SDK DTOs, CLI/Tauri payloads,
  and other views are deterministic projections or representations as
  applicable.

None of these objects is an Artifact Instance. Persistence, project-local
storage, human readability, content addressing, or physical colocation MUST
NOT change its category or semantic owner.

Action Question is a bounded value inside its Basis, Determination, Intent, and
authorization closures. It normally participates in those identities rather
than acquiring an independent repository or lifecycle.

### Historical truth and present use

A Historical Semantic Result MUST remain immutable for its exact evaluation
occurrence, evidence closure, definitions, policies, semantic times, and owner
verification facts. Later changes may alter whether an action domain can rely
on it now, but MUST NOT change its decision, explanation, identity, or recorded
historical validity.

Operational Reliance MUST begin from exact owner-recognized result identities.
A summary, projection, rendered document, Work Specification status, SDK DTO,
CLI output, copied gate label, current-head lookup, nearest policy version, or
ambient repository query cannot substitute for the source result. A projection
may help a person understand a decision, but its source result must be resolved
before operational use.

Historical display and exact replay require no Operational Reliance Basis when
they cause no present effect. The moment a result is used to allow, block, hold,
prioritize, close, advance, promote, release, deploy, admit a posture
observation, recommend a transition, or otherwise affect current work, the
responsible action domain MUST form an action-specific basis and result.

Equal Historical Semantic Results may legitimately produce different
Current-Use Eligibility for different Action Kinds, targets, policies, or
reliance times. The difference belongs to the downstream policy and MUST NOT be
described as a changed or reinterpreted gate.

An Operational Reliance Determination MAY be the terminal result of a pure,
non-mutating advisory use. When no effectful Action Intent is formed, no Action
Authorization Binding, Action Attempt, Action Terminal, or Mutation Receipt
exists. `block` does not mutate a work item into a blocked state; `hold` does
not create a queue or waiting state; `no_effect` does not mark an action
successful; and `allow` does not admit an observation, create a priority, or
produce a recommendation.

A separately owned admission, prioritization, or recommendation MAY consume an
exact Operational Reliance Determination only under that later owner's own
exact policy, admission, currentness, and accounting rules. `eligible / allow`
is not the later admission, priority, recommendation, or action authority.

Formation or durable retention of the semantic records defined by this chain
is not recursively another effectful action merely because a repository
changes. The action chain governs the bounded operational effect named by an
exact Action Intent unless another independently admitted Action Kind expressly
targets that record lifecycle.

The [complete-pruning commitment/frontier outcome](evidence-and-evaluation.md#no-collection-commitment)
is native evaluation-lifecycle closure under Evaluation Coordination. It establishes
commitment to an evaluation basis and absence of collection activity, not work
closeout or another operational effect. It MUST NOT recursively require a reliance
Determination based on the evaluation truth that it enables. Later operational
use still follows this chapter's full reliance and action chain.

Admission-governance first establishment at initialization or after positive deferral under G3 and successor activation follow the bounded
governance lifecycle in
[Shared policy admission](objects-and-governance.md#establishment-and-replacement-of-governing-admission-requirements).
They do not require a fabricated historical evaluation or a project action policy
whose admission depends on the configuration being established. Validation,
authority for the exact activation, expected predecessor/currentness, cutover,
and committed effect remain separately attributable propositions. This is the
explicit setup/replacement boundary, not a general exemption for configuration
changes or other effects from the Operational Reliance and action rules.

### Later collection checks and action boundaries

Historical evaluation resolution includes the exact later launch checks,
Coordination outcomes, and recovery/refusal accounting retained through its
frontier under [post-commit verification](evidence-and-evaluation.md#post-commit-verification).
A later positive check cannot repair unauthorized collection or replace the
committed basis. Neither present-use mode waives launch-time or final Contracts
checks. A later refusal does not itself rewrite an already valid closed result;
present reliance still follows its exact admitted policy and historical validity.

Permission to continue ordinary evaluation collection belongs to its existing
Engine, input-owner, and Coordination boundaries. It supplies no action authority
and does not relax Action Authorization, target/expected-state checks, or the
separate recovery rule for uncertain mutation. No general identity or permission
platform is introduced by recording these later evaluation checks.

### Operational Reliance Policy and basis formation

Every Action Kind, Operational Reliance Policy, Action Authorization policy, and
recovery policy consumed by this chain MUST be an exact category-2 definition
admitted by its responsible owner through
[shared policy admission](objects-and-governance.md#policy-admission). Authorship
and organizational rationale neither admit a policy nor authorize a particular
action; the action chain below retains its own separate owner results.

Required project qualifications bind the governing configuration and owner-rule basis active at
the shared decision boundary. Replacing that configuration neither grants action
authority nor changes historical results or existing policy identities. Current
policy usability remains subject to the exact owner's and consumer's declared
dependency/currentness rules; neither blanket revocation nor automatic successor-
rule compliance follows from replacement.

Intrinsic definition establishment alone cannot activate or assign these policies, authorize an effect, or supply missing governed-reliance inputs. G4's independent evaluation route is not an independent governed-action route.

An Operational Reliance Policy MUST declare before result direction is used:

- exactly one Action Kind and policy owner;
- supported Action Target Snapshot kind, scope, and congruence relation;
- every accepted Historical Semantic Result kind, semantic owner, cardinality,
  composition relation, and Evaluation Use Intent;
- mandatory governing-completion dependencies when the effect discharges such
  an obligation, and whether each other dependency is historical-identity-only or requires a fresh
  positive currentness verification at reliance time;
- every additional current fact, definition, policy, artifact revision,
  posture snapshot, work-structure relation, or authority-independent
  prerequisite it consumes;
- exact handling of `pass`, `fail`, `indeterminate`, `not_applicable`, result
  direction, accepted exact upstream refusal-record kinds, and mixed semantic
  inputs, while failed historical resolution remains Reliance Refusal;
- the exact predicates that establish `eligible`, `ineligible`,
  `indeterminate`, or `not_applicable` and the fixed disposition mapping; and
- complete accounting, semantic-time, invalidation, and replay rules.

Before forming an Operational Reliance Basis, the responsible downstream
domain MUST close the exact action question whose present use is being decided:
one exact Action Kind, one exact Action Target Snapshot, the exact bounded
effect, effect scope, and every action parameter whose value can change
eligibility under the admitted Operational Reliance Policy. The Basis and its
Operational Reliance Determination MUST bind that exact action question.

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

For every consumed Historical Semantic Result role, an Operational Reliance
Policy MUST declare one exact finite eligible source-result boundary. The
boundary MUST bind:

- permitted semantic owners and exact repositories, resolvers, or exact
  nominated references;
- accepted result kinds, Evaluation Use Intents, and result roles;
- required subject, target, scope, evaluation-occurrence, context, and
  semantic-time relations;
- historical-only or present-currentness modes;
- cardinality, multiplicity, ordering, and composition semantics;
- deterministic inclusion and exclusion rules; and
- complete candidate, selected, excluded, missing, duplicate, conflicting,
  unavailable, and refused accounting.

Basis formation MUST account exactly once for every owner-recognized result
matching that boundary and every required result role. Caller-selected subsets,
favorable-result selection, current-head or nearest-version lookup, repository
order, open-ended “all current results” queries, or unrecorded omission MUST
refuse.

A nominated-reference boundary still requires independent justification and complete accounting under G5; it cannot hide relevant known failures.

An exact upstream refusal record MAY be consumed only when the policy declares
that exact refusal kind as an accepted Historical Semantic Result role and the
refusal resolves successfully from its owner. Failure to resolve a required
identity, semantic content, owner provenance, or historical validity is always
Reliance Refusal. It cannot be mapped by policy to `ineligible`,
`indeterminate`, or `not_applicable`.

Acceptance of diagnostic Evaluation Use Intent MUST be declared per consumed
result role, result direction, and exact Operational Reliance action question.
Permission to use a diagnostic result to block, hold, trigger remediation,
admit a posture observation, or support another negatively directed use does
not permit that result to establish positive `eligible / allow` for a different
Action Kind, target, effect, or scope. No owner may erase, relabel, or infer a
different Evaluation Use Intent. These permissions remain subject to the
[governed-completion floor](#governed-completion); no diagnostic result may fill
its mandatory assessment role, even under an otherwise permissive policy.

For one exact Operational Reliance Occurrence Identity:

- exact same basis-formation content is replay and returns the same Basis,
  Operational Reliance Determination identity and both facets;
- exact same refused formation content returns the same Reliance Refusal
  identity and decisive owner provenance;
- changed content under that identity refuses;
- a deliberately new present-use determination receives a new occurrence
  identity even when every semantic input is otherwise equal; and
- transport retry or correlation cannot create, merge, or distinguish
  reliance occurrences.

Exact replay of one Action Authorization request or Action Attempt likewise
returns the same owner outcome for the same exact occurrence and content;
changed content under the same semantic identity refuses.

Basis formation MUST resolve and completely account for every policy-declared
input and every mandatory governed-completion input where that boundary applies. It refuses before a Determination when any required owner, identity,
subject/target relation, cardinality, semantic time, source-result integrity,
currentness observation, or policy-selection fact is absent, ambiguous,
substituted, conflicting, or unverifiable. A valid but expressly uncertain
policy input may produce `indeterminate` only when the admitted policy declares
that uncertainty as a semantic input; missing formation truth is never
`indeterminate`.

<a id="governed-completion"></a>
### Independently governed completion

The [approved HR-007 amendment](amendments/governance-defaults-and-delegated-operation.md)
was integrated on 2026-10-04. References to G1–G6 below name its approved rules.

For an effect that discharges a project-governed completion, promotion, or
release obligation, Work Progression's policy/lifecycle responsibility (for
local and parent work) or the relevant action domain MUST independently
establish the **governing-completion relationship**. It is an owner-established
semantic relationship, not a mandatory new runtime record or service. It binds:

- exact project, Action Kind/effect, actual target and scope, and whether that
  effect discharges a governed obligation;
- the applicable completion/reliance policy and exact governing selection basis;
- the complete actual requirements, adopted supported assessment protocol and role inventory, plus required Assurance Policy Selector, Posture Scope and assurance-selection basis where applicable, and current qualification basis for accepted earlier results under G5;
- exact evaluation-subject-to-action-target relationship and required coverage;
- authoritative work/parent/dependency boundary and currentness requirements.

The authoritative work-state/structure owner supplies actual target and relation
facts; Engine supplies required posture; Contracts Assurance Selection supplies current governing requirements through the applicable exact mapping, while direct requirement resolution and evaluation owners preserve the earlier result's own requirements and truth. The action domain owns G5's current qualification and cannot reconstruct those owners' propositions.
Governing applicability/assignment follows its authorized owner definition and
lifecycle boundary, independently of the submitted development result. Admission
of a policy or contract into an available population is not governing assignment.
HR-001 governs admission authority; it does not assign every admitted definition
to every completion request. Concrete mappings remain for later planning, but
missing, ambiguous, conflicting, or unverifiable governing relationships refuse
the affected completion operation; absence is not exemption or development mode.

Positive deliberate governance deferral under G3 permits the independent
authoring/validation/evaluation uses in G5. It does not establish a governed
completion standard or produce a governed-completion Determination. When
governance is established, its authorized owner relationship determines whether
requirements apply to this actual effect/target/scope; mere policy admission is
not assignment. Required missing or uncertain relationships still refuse the
affected completion operation. Independent historical results cannot be relabeled;
only G5's separately adopted protocol and new current owner determination may
accept an exact earlier result for declared current assessment roles.

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

Ordinary policy, typed exception, wrapper, child result, higher-order selection
or display label MUST NOT waive exact contract/intent, governing assignment,
supported protocol, coverage, qualification or required process conditions.
Properly authorized exceptions remain bounded by their existing owner rules;
they cannot change historical provenance or fabricate collection-time authority.
Supplemental development and diagnostic results may serve declared roles but
cannot substitute for the mandatory assessment contrary to G5. An exact diagnostic
failure may support a permitted remediation action; it is not completion of the
original work. Still-applicable qualifying assessments need not be rerun for
every completion request.

Action authorization and attempt admission require the exact eligible
Determination and congruent effect/target. The authoritative state owner MUST
preserve the governing conditions through commit. Every supported operation that
establishes the same governed-completion proposition, including a generic status
setter, is subject to these checks. A genuinely narrower child outcome cannot
discharge a parent's separate required assessment or replace complete parent
accounting and actual child-closure facts. This does not classify every local
development check or informational state as governed completion.

These checks do not change the separate `historical_validity_only` and
`continuing_current` modes for historical Engine authorization. Advisory action
authority remains permitted where the exact rule allows it; stronger identity
or extra confirmation is not required by this decision, and no authority mode
repairs missing eligibility or target/commit checks. Supported owner operations
enforce these boundaries when honored; this is not protection against replacing
authoritative state or bypassing Handbook. Prose alone is not committed completion.

Authorized governance changes may alter configurable qualification or completion
requirements within fixed constraints through the relevant owner lifecycle.
Development intent gives no extra change authority. Exact provenance does not
prove the governing engineering requirements are substantively adequate; those
requirements must still be defined. No universal hierarchy or deployment policy,
new evidence-import route, or separate identity platform is selected here.

### Exact current-use and Engine-verification rule

The common present-use floor is mandatory for every action domain:

1. close the exact action question and its eligibility-relevant parameters;
2. resolve the exact Historical Semantic Result and verify its immutable owner
   identity, content, input closure, and recorded historical validity;
3. resolve the exact current Operational Reliance Policy and its complete
   selection basis;
4. resolve an exact Action Target Snapshot and obtain a fresh positive owner
   verification that it is the current expected target for this reliance
   occurrence;
5. establish exact result-to-target subject and scope congruence;
6. obtain every fresh owner verification the policy declares for present use,
   without reconstructing one owner's proposition in another domain; and
7. complete all inclusion, exclusion, missing, conflict, use-intent, and
   currentness accounting before producing eligibility or disposition.

Fresh current verification of the historical evaluation's Engine-issued
**Evaluation Authorization Binding is policy-conditional, not universal**.
Every admitted Operational Reliance Policy that accepts a Historical Semantic
Result whose exact closure binds such a Binding MUST explicitly declare exactly
one reliance-time dependency mode for it:

- **`historical_validity_only`**; or
- **`continuing_current`**.

An omitted, inherited, caller-selected, ambient, or defaulted mode makes the
policy invalid and refuses policy admission or resolution.

Under both modes, the common exact-result floor requires owner-recognized
historical resolution of the exact Evaluation Authorization Binding, its
issuance lineage, and the historical-verification outcome required to establish
that the Historical Semantic Result was validly closed. Any Engine-declared
historical-resolution mechanism may satisfy this requirement; it is not a
current-authorization query. An outcome of `historically_invalid`,
`unverifiable`, unavailable, ambiguous, or substituted causes Reliance Refusal.
It does not rewrite the Historical Semantic Result and MUST NOT become
`ineligible`, `indeterminate`, or `not_applicable`.

Under `continuing_current`, a fresh Engine-produced positive current
verification at the Operational Reliance semantic time is additionally
mandatory. `superseded`, `not_current`, `unverifiable`, unavailable, ambiguous,
or substituted current verification causes Reliance Refusal.

Under `historical_validity_only`, current Evaluation Authorization status is
not an Operational Reliance input. The action domain MUST NOT infer a current
Engine conclusion from the historical Binding or allow an independently
obtained current observation to alter the Operational Reliance Determination.

A later Action Authorization policy MAY independently declare the same
Evaluation Authorization Binding or another Engine-owned proposition as an
authorization-time or commit-time condition. When it does, only Engine's exact
fresh verification satisfies that condition. The Operational Reliance mode
neither supplies nor forbids that distinct authorization-stage dependency.
Any other Engine-owned target, artifact, posture, authority, or Context
Resolution fact declared by either policy likewise requires Engine's exact
verification. Policy-conditional means policy-mandatory when declared, never
caller-optional.

This is the focused downstream rule to which the companion decisions defer. It
refines broader language suggesting fresh evaluation-authorization verification
for every operational reliance. Evaluation authorization is permission to
perform one historical evaluation under one purpose and reach. It is neither
permission to act now nor a universal statement that every possible action
depends on continuing evaluation reach. Universal continuing-current status
would conflate evaluation and action authority; allowing an omitted mode would
make accidental policy silence authoritative. The explicit two-mode rule avoids
both.

Fresh **Action Authorization Binding** and expected-target verification are
universal before every Action Attempt and cannot be policy-disabled. A state-
changing action MUST additionally enforce the same Binding, target, exact
authorized effect, mandatory governing-completion conditions where applicable,
and every additional declared current-through-commit dependency at its
commit or linearization boundary. Missing, non-current, superseded, mismatched,
or unverifiable verification refuses the governed attempt or commit without
changing the already closed Operational Reliance Determination.

### Eligibility, disposition, and refusal vocabulary

One valid Operational Reliance Basis produces exactly one immutable Operational
Reliance Determination. That Determination contains one Current-Use Eligibility
facet and one mechanically paired Operational Disposition facet:

| Current-Use Eligibility | Fixed meaning | Operational Disposition |
| --- | --- | --- |
| `eligible` | The exact policy positively establishes that the bound Historical Semantic Results may support consideration of this exact Action Kind for this exact target now. | `allow` |
| `ineligible` | The exact policy positively establishes that the bound results may not support this exact action for this target now. | `block` |
| `indeterminate` | The basis is valid, but an expressly admitted uncertain or unresolved semantic input prevents the policy from establishing eligible or ineligible. | `hold` |
| `not_applicable` | Positive policy proof establishes that the action question does not apply to this exact target and scope. | `no_effect` |

The mapping is fixed so interfaces cannot turn `indeterminate` into `allow`,
turn `not_applicable` into success, or reinterpret a positive blocker as a
temporary hold. An action-specific policy controls the bounded question. Thus a
failed security gate may make local closeout `ineligible` while making an exact
remediation action `eligible`; the gate truth is unchanged in both cases.

The two facets share one Operational Reliance Occurrence Identity, policy,
action question, target, semantic time, determination identity, and decisive
accounting closure. Operational Disposition is not a second policy decision,
separately current object, independently supersedable record, or independently
replayable result. No implementation may issue, persist, supersede, reference,
or replay one facet as though the other could differ. A reference MAY identify
the shared Operational Reliance Determination and the exact facet displayed or
documented.

Reliance Refusal produces no Operational Reliance Determination and therefore
no Eligibility or Disposition facet. Adapters MAY render it operationally as
“no action permitted,” but MUST preserve the refusal kind and decisive owner
reason rather than fabricate `block` or `hold`. `allow` alone never permits an
executor or mutation owner to act without a current exact Action Authorization
Binding.

### Work Progression ownership

Work Progression is the exact downstream semantic owner for local closeout and
parent eligibility. It owns:

- exact work-target and parent/child/dependency relationship closure needed by
  those decisions;
- admission and resolution of Local Closeout Policies and Parent Eligibility
  Policies;
- independently established governing-completion applicability, actual requirements and adopted protocol, prospective selection or current qualification and complete target/assessment coverage under G5; action-specific policy selection and complete
  candidate accounting;
- Operational Reliance Occurrence Identity and Basis formation;
- Local Closeout and Parent Eligibility Operational Reliance Determinations,
  including their Current-Use Eligibility and Operational Disposition facets;
  and
- exact historical replay and currentness/invalidation observations for those
  work-progression results.

The authoritative work-structure and work-state owner supplies exact work
identities, current state snapshots, and parent/child/dependency relations.
Work Progression resolves and completely closes those owner facts into its
Action Target Snapshots; it MUST NOT invent or rewrite them. This target model
does not retain the current Contracts-owned `LocalCloseoutResultV1` as the
greenfield local-closeout authority. A future local result is the distinct
Work Progression-owned Local Closeout Eligibility defined here.

Work Progression MUST NOT own or infer Contracts evidence, claim, metric, or
gate truth; Engine evaluation authorization; artifact promotion; Action
Authorization Binding issuance merely because it produced `allow`; or an actual
work-state mutation. One future component may host Work Progression and an
action-authority or state-owner responsibility, but their policies, identities,
decisions, refusals, currentness, and receipts MUST remain separately
attributable.

Release, deployment, environment change, posture observation, posture
recommendation, posture transition, and other action kinds remain owned by
their separately named downstream domains. They MAY reuse the common reliance
vocabulary and floor fixed here but MUST form their own policies, bases,
atomic Determinations, authorizations, and effects. This record creates no
universal enforcement owner.

A valid [complete-pruning evaluation](evidence-and-evaluation.md#complete-pruning)
may produce an exact assessment with no producer observations. Its presence does
not itself imply a missing assessment, a pass, positive eligibility, or completion.
The actual governing role still requires original Decision-seeking eligible/decision-seeking provenance, exact target/coverage and all required currentness under G5's adopted supported protocol, followed by exact outcome-policy interpretation. Required prospective posture-selected provenance remains mandatory under that protocol; only an expressly adopted retrospective protocol and new current owner qualification may accept an exact earlier direct result for declared roles. Diagnostic provenance cannot fill the mandatory positive role. This rule
neither exempts empty collection from completion requirements nor universally
forbids completion based on correctly established non-applicability. The existing
all-not-applicable example retains `not_applicable / no_effect` under its named
policy; no universal N/A-to-eligibility mapping is introduced. An all-pruned child
assessment cannot replace the parent's governing assessment or prove actual child
closeout. Governing conditions and action/commit checks remain required.

### Local closeout

One Local Closeout Operational Reliance Determination MUST be formed
independently from one exact Local Closeout Policy and one exact local-work
Operational Reliance Basis. Its Local Closeout Eligibility facet and the Basis
MUST bind:

- the exact local Action Target Snapshot and currentness verification;
- every exact evaluation, claim, metric, gate, or other owner result the policy
  declares, including Evaluation Use Intent;
- exact subject/scope congruence between those results and the local target;
- every policy-declared artifact, dependency, completion, or currentness fact;
- exact policy identity, selection provenance, and reliance semantic time; and
- complete included, excluded, missing, conflicting, stale, and indeterminate
  accounting.

A Local Closeout Policy MAY declare that one exact gate `pass` is sufficient
only when its complete predeclared input and currentness closure, including the
mandatory governed-completion relationship and qualifying assessment where
applicable, makes that true. No universal `gate passed -> local closeout eligible` rule exists.
`fail`, `indeterminate`, or `not_applicable` retain their Contracts meanings;
the Local Closeout Policy maps them only for its bounded action question.

Local Closeout Eligibility is pure. It MUST NOT mutate a Work Specification,
work state, Pipeline state, parent, release, deployment, posture, or any other
object. A local closeout action may proceed only after `eligible / allow`, a
congruent Action Intent, a separate exact Action Authorization Binding, and an
exact Action Attempt. The authoritative work-state owner alone enforces the
target, mandatory governing-completion dependencies, and every additional
declared through-commit currentness condition, commits the
mutation, and issues its Mutation Receipt.

Local closeout's exact result role must preserve contract permitted use,
requirement-selection source, and run intent. Calling a request "incremental
completion" cannot waive the governing obligation when its actual effect
discharges that obligation. A genuinely bounded child check remains about that
child, without implying parent eligibility or promotion.

### Parent eligibility and parent action

Parent Eligibility is a separate Work Progression decision, not a field inside
Local Closeout Eligibility, a gate, or a score. Its exact Parent Eligibility
Policy and Operational Reliance Basis MUST bind:

- one exact parent Action Target Snapshot and currentness verification;
- the exact owner-established parent-to-child and dependency closure at the
  reliance semantic time;
- every child, dependency, role, cardinality, ordering or composition relation
  the policy declares;
- each exact Local Closeout Eligibility or other owner result consumed for each
  required relation, with its subject/scope congruence and current-use basis;
- every required parent-level gate, policy, artifact, authority-independent
  prerequisite, and present-use owner verification; and
- complete included, excluded, missing, duplicate, conflicting, stale,
  superseded, and unresolved accounting.

Every Parent Eligibility Policy MUST declare one exact finite authoritative
parent-relationship boundary. That boundary MUST bind:

- the authoritative work-structure owner and exact resolver or snapshot
  identity;
- the exact parent Action Target Snapshot;
- supported child, dependency, role, ordering, and composition relation kinds
  and directions;
- the exact direct, transitive, or otherwise declared relation reach;
- the causal or semantic-time cut and currentness rule;
- the policy's cycle treatment;
- deterministic relation inclusion and exclusion rules; and
- complete included, excluded, missing, duplicate, conflicting, cyclic,
  superseded, stale, and unresolved accounting.

Work Progression MUST account exactly once for every owner-recognized relation
matching that boundary and every policy-required result role. Open-ended
traversal, unsupported cycles, mixed relation cuts, ambiguous direction, or
inability to establish boundary completeness causes Reliance Refusal before
Parent Eligibility.

For every relation role, the Parent Eligibility Policy MUST declare the exact
proposition it consumes, including whether it requires a child Local Closeout
Eligibility, Action Terminal, Mutation Receipt, current child work-state
snapshot, or another exact owner result. Those propositions are non-
substitutable. Consuming Local Closeout Eligibility establishes only the
child's eligibility proposition. A parent policy that requires the child to
have actually closed MUST consume the authoritative current closed-state
proposition and/or exact Mutation Receipt declared by that policy; it MUST NOT
infer committed closeout from eligibility.

A parent policy MUST NOT treat an absent child, unaccounted dependency,
ambiguous hierarchy, historical local result without required current-use
proof, or copied `eligible` label as satisfied. A changed parent snapshot,
child/dependency membership, relation, child target, Local Closeout result, or
declared currentness fact requires a new parent basis and result.

`Parent Eligibility Result = eligible` means only that the exact policy
positively permits consideration of the named parent Action Kind for the exact
parent snapshot. It does not:

- close or mutate the parent;
- promote, merge, release, deploy, or advance anything;
- inherit any child's Action Authorization Binding;
- authorize a parent actor or state owner;
- prove that an action attempt occurred; or
- imply that a later target-state recheck will succeed.

Any parent advancement or promotion is a separately named action. It requires
its own current exact Action Authorization Binding, expected parent snapshot,
an exact Action Attempt Identity, through-commit dependency/currentness
enforcement, an atomic state-owner decision, and Action Terminal. At the
mutation commit boundary, the state owner MUST establish that the exact parent
snapshot, mandatory governing-completion conditions, and every additional
parent/child/dependency relation declared to remain current
through commit still match the Parent Eligibility and Action Authorization
closures. A committed change additionally requires a Mutation Receipt. Parent
eligibility remains immutable history if that later action refuses or the
target changes.

Where the parent action discharges a governed obligation, its own
[governing-completion relationship](#governed-completion), required assessment,
and through-commit coverage are mandatory in addition to all parent closure
rules above. Child eligibility, child mutation receipts, or child assessments
cannot be wrapped into a substitute for the parent's required assessment.

### Action authorization and mutation

Eligibility and authority are deliberately ordered and non-substitutable:

1. Work Progression or another action domain produces one exact Operational
   Reliance Determination with `eligible / allow` facets for one closed action
   question.
2. The requester forms an exact Action Intent congruent with that action
   question and adds only the actor or principal, claimed delegation or authority
   provenance, and policy-permitted execution-local details.
3. The exact action-authorization owner resolves the Action Intent and either
   issues one exact Action Authorization Binding or one exact Action
   Authorization Refusal. A refusal creates no Binding or Action Attempt.
   Evaluation Authorization, a gate, `allow`, artifact approval, transport
   authentication, or a prior authorization cannot substitute.
4. Before an Action Attempt begins, the executor allocates or resolves one
   exact Action Attempt Identity and obtains exact owner-produced positive
   verification of the Binding, Intent, Operational Reliance Determination,
   expected Action Target Snapshot, and every condition declared for attempt
   admission. The executor MUST NOT reconstruct another owner's authorization,
   target-currentness, or dependency proposition.
5. For a state-changing action, the authoritative state owner enforces at the
   mutation commit or linearization boundary that the same exact Binding
   remains current, the same expected target snapshot remains current, the
   effect is exactly the authorized effect, mandatory governing-completion
   conditions hold where applicable, and every additional dependency declared to
   remain current through commit still verifies under its owner. A prior
   recheck separated from commit by an unguarded concurrency gap is
   insufficient. If a required cross-owner causal or compare condition cannot
   be established, no state change may commit and no Mutation Receipt may
   issue.
6. The executor produces exactly one Action Terminal for the Action Attempt.
   The authoritative state owner produces a Mutation Receipt only after durable
   commit of the exact authorized state change. The Terminal may report
   committed mutation only by binding that Receipt.

#### Authority provenance without a Handbook IAM platform

This semantic action chain does **not** require Handbook to build a general
identity, credential, authentication, delegation, capability-token, secrets,
PKI, RBAC, or ABAC platform. An Action Authorization Binding is the action-
authorization owner's exact semantic result and provenance contract. Concrete
authentication, credential, host-permission, or enforcement mechanisms remain
owned by the systems that actually provide them. Engine Context Resolution
retains its separate semantic admission and verification responsibility for
evaluation reach; neither action authorization nor this clarification replaces
or weakens it.

A standalone CLI or SDK workflow MAY use explicitly permitted **advisory
authority**. Under such an admitted action-authorization rule, a user prompt or
configured agent role may supply action intent, a claimed principal or
delegation, and declared authority. The Binding MUST preserve that exact source
and label those propositions as advisory claims rather than independently
authenticated identity, verified delegation, host-enforced permission, or a
fine-grained credential. Ambient operating-system permissions are real execution
capability, but they are not by themselves Handbook proof of the claimed
principal, delegation, scope, or policy authorization.

Advisory authority may support an effectful Action Attempt only when the exact
action-authorization policy explicitly permits that authority mode for the
Action Kind and target. It cannot fabricate verified facts, satisfy a policy
that requires independently authenticated, owner-verified, or host-enforced
authority, waive the exact `eligible / allow` Determination, or bypass target,
currentness, attempt-admission, and through-commit checks. A real enforcing host
may supply stronger exact facts under its own authority; Substrate is only a
possible future example and is not selected here. Changing advisory source,
claimed principal or delegation, declared authority, assurance mode, or any
verified authority fact requires a new authorization decision and cannot be
hidden inside delivery replay.

An action authorization does not improve Current-Use Eligibility. An
ineligible, indeterminate, not-applicable, or refused reliance result cannot be
overridden by broader action authority. Conversely, an eligible result cannot
manufacture authority. An Action Authorization Binding MAY impose additional
restrictions or execution conditions but MUST NOT authorize a broader or
different action question.

An Action Intent MUST NOT broaden, substitute, or change the Action Kind,
Action Target Snapshot, expected state, effect, effect scope, or any
eligibility-relevant action parameter already decided by the Operational
Reliance Determination. A change to any of those values requires a deliberately
new Operational Reliance Occurrence Identity, Basis, Determination, Action
Intent, and authorization path. A change only to the actor or principal,
claimed delegation, declared authority, or its source requires a new Action
Intent and authorization. A change only to the authorization owner's rule,
basis, or conditions may require a new authorization decision over the
unchanged Intent; it does not rewrite that Intent.

Exact delivery replay under the same Action Attempt Identity and exact attempt
content MUST return the same Action Terminal and any same Mutation Receipt
without re-executing the effect. Changed attempt content under the same Action
Attempt Identity MUST refuse.

A non-idempotent retry is a deliberately new Action Attempt. It is permitted
only when the exact Action Authorization Binding or a fresh successor Binding
permits another attempt, all required dependencies remain exact and current,
and the authoritative effect or state owner has positively established that
the prior attempt did not apply or commit.

When a prior attempt has an uncertain effect or commit status, an ordinary non-
idempotent retry MUST NOT proceed. A later attempt is permissible only under a
separately admitted recovery rule and fresh authorization that explicitly
account for possible prior application and establish duplicate-safe recovery.
Recomputed eligibility, unchanged target appearance, timeout, lost
acknowledgement, or transport failure does not establish non-commit.

Initial human approval and routine delegated operation follow G2. Owner authorization for each operation may be automatic under approved boundaries; it is not inherently a new human approval request.

### Work Specification relationship

A Work Specification remains the authored plan for one feature, slice,
delivery unit, or other bounded body of work. It may:

- nominate an exact already owner-recognized Operational Reliance Policy or
  other input through a Declared Input Reference when the consuming Work
  Progression definition permits that role;
- be bound as one exact authored input when the policy explicitly declares its
  proposition, scope, revision, and currentness mode; and
- in a later revision, add exact Outcome or Disposition References to already
  closed evaluation, Operational Reliance Determination, Action Authorization
  Refusal, Action Terminal, or Mutation Receipt identities. A Disposition
  Reference to a reliance outcome names the shared Determination and identifies
  its Operational Disposition facet; it does not make the two facets
  independent records.

It MUST NOT create an executable reliance policy, Current-Use Eligibility,
Operational Disposition, Operational Reliance Determination, Action
Authorization Binding or Refusal, Action Attempt, Action Terminal, or Mutation
Receipt merely through acceptance language, a `completed` field, approval
prose, a checkbox, or a reference. Work Progression independently resolves and
closes every declared input. The action authority and state owner independently
authorize and apply any effect.

An operational mutation MUST NOT silently rewrite the Work Specification
revision that preceded it. A later revision may record the exact outcome. That
reference preserves causal order and does not make the artifact the action
owner or the later revision an input to the earlier reliance occurrence.

### Posture-feedback boundary

A semantic gate, Local Closeout Eligibility, Parent Eligibility Result,
Operational Disposition, Action Terminal, or Mutation Receipt does not change
posture. A gate or work-progression result may enter posture reasoning only
through this forward path:

```text
exact Historical Semantic Result
  -> posture-observation-specific Operational Reliance Basis and Determination
  -> separately owner-admitted Posture Observation
  -> recommendation-specific exact policy, basis, and owner result
  -> Posture Recommendation
  -> transition-specific Operational Reliance Basis and Determination
  -> exact Posture Transition Action Intent
  -> Engine-issued current Action Authorization Binding
  -> final authority, approval, and expected-posture-state verification
  -> Engine Action Terminal and, after committed state change, Mutation Receipt
  -> new Resolved Posture Snapshot for a later evaluation
```

The posture-observation owner applies its own exact policy, target/scope
congruence, current-use rule, and admission boundary. The recommendation owner
cannot admit its own observations. A Posture Recommendation does not establish
transition eligibility or authorization. The transition-specific Operational
Reliance Basis MUST independently close the exact recommendation, current
posture target, transition policy, approvals, and every other declared present
dependency. The Engine transition owner then requires its own fresh authority,
current expected posture state, approvals, through-commit verification, and
transition validity.

No observation, recommendation, transition basis, authorization, terminal,
receipt, or successor posture snapshot may alter the posture, requirements,
Evaluation Purpose, evidence closure, Metric Definitions, Gate Policy, or
semantic result of the evaluation occurrence that produced the source result.

### Currentness, invalidation, and replay

The following changes preserve all historical objects but prevent reuse of an
existing Operational Reliance Basis for a new present decision:

| Change or condition | Required consequence |
| --- | --- |
| Action Kind, policy identity/content, policy-selection basis, accepted-result rule, bounded effect, effect scope, or eligibility-relevant parameter changes | Form a deliberately new reliance occurrence, Basis, Determination, Intent, and authorization path under the exact changed action question. |
| Action Target Snapshot, expected state, or result-to-target congruence changes | The prior basis remains history but cannot govern the changed target. Form a new basis and Determination; no rebinding is permitted. |
| A Historical Semantic Result changes | Results are immutable, so this is substitution or corruption and refuses. A genuinely new evaluation result requires a new reliance occurrence. |
| A policy-declared historical-only source receives a newer revision | The exact old source remains usable only as the policy explicitly permits; the newer revision is not substituted and no present authority is implied. |
| A dependency declared current-at-reliance becomes non-current, superseded, stale, or unverifiable | No Operational Reliance Determination may be formed from the old observation. Obtain a fresh positive owner verification or form a new upstream result as the policy requires. |
| Evaluation Authorization Binding becomes non-current under explicit `historical_validity_only`, while its exact historical resolution remains valid | The immutable result retains its historical authorization provenance. No Engine current-use conclusion is inferred, and this change alone does not alter the Determination. All other declared present-use dependencies and separate action authority still apply. |
| Engine reports the exact historical Binding `historically_invalid`, unverifiable, unavailable, ambiguous, or substituted under either mode | Reliance formation refuses without rewriting the Historical Semantic Result. No Eligibility or Disposition facet is produced. |
| The action policy declares `continuing_current` and Engine reports non-current, superseded, unverifiable, unavailable, ambiguous, or substituted | Reliance formation refuses because the required positive owner verification is absent. It never treats the authorization as current or reconstructs Engine truth. A fresh evaluation is required when the policy requires one. |
| Governing completion selection, target coverage, or required assessment relationship changes | Preserve history; stop affected reliance/action when the relationship is invalid. New reliance is required; changed requirements or acceptance relations require new current qualification or the specific new evaluation/observation work required by G5; unchanged old results qualify only under exact supported coverage/protocol rules. These dependencies cannot be made historical-only by policy. |
| Work parent/child/dependency membership or relation changes | A prior Parent Eligibility Result remains history but cannot govern the changed closure. Form a new exact parent basis with complete accounting. |
| Local work state or a required local result changes after Parent Eligibility closes | The parent result remains history. Present parent action stops at final recheck and requires a new parent reliance occurrence. |
| Action Authorization Binding expires, is superseded, no longer matches the target/effect, or its advisory/verified authority source, claimed principal/delegation, authority mode, or required verification facts change before attempt or commit | The already closed Determination remains history; the attempt or mutation refuses. A new authorization decision is required, and new authority never retroactively changes the Determination. |
| Target state, a mandatory governing-completion condition, or an additional dependency declared current-through-commit changes after `allow` but before mutation | Commit-boundary enforcement refuses the mutation and no Mutation Receipt issues. Recompute reliance against the new target or dependency closure before requesting new authorization. |
| Exact historical dependency is temporarily unavailable for resolution | Historical results are not rewritten. A display may report the owner-attributed non-success, but operational reliance refuses because the common exact-result floor cannot be established. |

Exact reliance replay resolves the same occurrence, policy, action question,
target snapshot, historical results, reliance-time observations, Determination,
and both facets, or returns the same exact Reliance Refusal for the same refused
formation content.
It MUST NOT substitute current definitions, reacquire evidence, rerun producers,
create new owner verification facts, or claim current action authority. A
present determination, even over equal source content, is a deliberately new
reliance occurrence.

Exact replay of the same Action Authorization request returns the same Binding
or Action Authorization Refusal. Exact replay of the same Action Attempt returns
the same Action Terminal and any bound Mutation Receipt without re-execution.
Changed content under any one of those semantic identities refuses.

An exact historical Action Terminal or Mutation Receipt remains immutable.
Current state is established from the state owner, never by replaying the
receipt or inferring that an earlier successful action still governs the target.

### Semantic responsibility map

| Responsibility | Owns | MUST NOT own or infer |
| --- | --- | --- |
| Contracts evaluation responsibilities | Historical applicability, claim, metric, semantic-gate, and canonical-explanation truth for one closed evaluation context. | Operational Reliance Policy, local closeout, parent eligibility, action authorization, work-state mutation, or posture transition. |
| Engine Context Resolution | Historical and current verification of Evaluation Authorization Bindings and every other declared Engine-owned Context Resolution proposition. | Current-Use Eligibility, action-policy selection, Work Progression truth, action authorization merely because an evaluation was authorized, or a general Handbook IAM platform. |
| Engine artifact/profile domain | Exact Work Specification and other artifact revisions, Artifact Subject Binding, artifact currentness, promotion, and lineage under the authored-artifact lifecycle. | Operational work completion, local/parent eligibility, action disposition, or action authority from artifact status or prose. |
| Authoritative work-structure and work-state owner | Exact work identities, current state snapshots, parent/child/dependency relations, state currentness, concurrency, and committed successor state. | Contracts truth, Work Progression policy or eligibility, action authorization merely from state ownership, or mutation without exact authorization. |
| Policy-definition and admission owners | Exact candidate population, deterministic Policy Admission Closure, typed vocabulary and conformance, relevant governance-source roles/scope/currentness, required separately authorized approvals or exceptions, non-relaxable invariant checks, admission/refusal, currentness, and supersession for each Action Kind, reliance, authorization, or recovery policy. | Treating candidate authorship, a bare link, rendered prose, an AI's unconstrained judgment, or a policy's own desired outcome as admission; converting authored text into runtime evidence or a waiver; or recursively letting a candidate admit its own admission rule. |
| Work Progression | Independently established governing-completion applicability/requirements and adopted protocol; prospective selection or current qualification of exact earlier results and complete assessment coverage under G5; work target and finite parent/dependency closure for its decisions; Local Closeout and Parent Eligibility policies; reliance occurrence, basis, atomic Determination with eligibility/disposition facets, currentness, and replay. | Contracts truth, evaluation authorization, Work Specification authorship, release/deployment policy, action authorization by implication, or mutation. |
| Other downstream action domains | Their exact action-specific reliance policies, bases, atomic Determinations, currentness, and replay under the common floor and G5's governing requirements/protocol/current-qualification rules when applicable. | A universal enforcement policy, another action domain's policy, Contracts truth, or Engine authority. |
| Action requester or origin owner | Exact Action Intent request content and durable request occurrence provenance. | Action Kind or target truth, eligibility, authorization, attempt, or effect merely because it requested one. |
| Action authority owner | Exact authorization rule; declared advisory, owner-verified, or host-enforced authority mode and basis; exact provenance of claimed versus verified principal/delegation facts; Action Kind, target, expected state, bounded effect, Operational Reliance Determination binding, issuance time and lineage, refusal, Binding, currentness, supersession, and permitted-attempt rule. | Improving eligibility, rewriting a gate, treating Evaluation Authorization as action authorization, claiming authentication/delegation/enforcement not established by its exact inputs, mandating a general Handbook IAM implementation, or proving an attempt or effect. |
| Action executor | Action Attempt Identity, attempt-admission checks from exact owner facts, exact authorized execution, one Action Terminal per attempt, delivery replay, failure, refusal after the attempt boundary, and effect/commit uncertainty. | Self-producing another owner's authorization/currentness, broadening the effect, or asserting committed state change without the state-owner receipt. |
| Authoritative state or effect owner | Exact expected predecessor, through-commit verification, atomic state change or effect decision, durability, concurrency, exact successor, commit semantic time, and Mutation Receipt. | Inventing eligibility, broadening authority, changing policy, or reporting an uncertain commit as success. |
| Posture observation owner | Exact policy-driven admission of permitted historical results as immutable Posture Observations. | Gate reinterpretation, recommendation by implication, or Engine posture mutation. |
| Posture recommendation owner | Exact policy-driven recommendation from admitted observations. | Observation self-admission, approval, or transition. |
| Engine posture-transition owner | Fresh authority/currentness/approval checks and atomic transition from one exact posture authority state to its successor. | Rewriting the source gate, eligibility, disposition, or recommendation. |
| SDK, CLI, Tauri, Flow, Pipeline, skills, and transports | Bounded composition, presentation, deterministic projections/assemblies, correlation, scheduling, transport-local errors, and exact carriage of prompt- or role-supplied advisory-authority claims around owner objects. | Authenticating a claimed principal/delegation, fabricating host enforcement, satisfying an owner policy that requires stronger facts, policy selection, current-use formation, disposition reinterpretation, action authorization, or mutation merely because they carry or invoke an operation. |

Physical colocation does not collapse these owners. In particular, a future
Engine-hosted Work Progression implementation would still consume rather than
reconstruct Contracts truth, and its eligibility responsibility would remain
separate from any Engine action-authority or mutation responsibility.

### Discriminating scenario corpus

| Scenario and inputs | Expected semantic outcome | Invariant proved |
| --- | --- | --- |
| **Historical pass, current local closeout.** Gate `G1` passed for exact work snapshot `W1`. The independently resolved governing completion relationship accepts its Decision-seeking eligible contract, protocol-compatible assessment provenance under G5, decision-seeking intent, and exact required coverage; the current Local Closeout Policy accepts that provenance, `W1` remains the current target, all mandatory governing conditions and declared dependencies verify, and no other closeout predicate is missing. | Work Progression forms a new reliance occurrence and basis, produces one Determination with `eligible / allow` facets, and stops before action authority. `G1` remains an unchanged Historical Semantic Result. | Historical truth may support current consideration without becoming the action. |
| **Pass is insufficient under another policy.** The same `G1` is supplied to a Local Closeout Policy that also requires an exact unresolved dependency disposition. | If the dependency is a missing formation input, Reliance Refusal produces no Determination. If the policy admits an exact uncertain dependency state, the Determination has `indeterminate / hold` facets. `G1` does not fill the gap. | Gate pass is neither a universal closeout rule nor a missing-input default. |
| **Failed gate enables remediation but blocks closeout.** Exact gate `G2` is `fail`. The Local Closeout Policy makes closeout ineligible, while a separately admitted remediation policy positively requires that failure. | The closeout Determination contains `ineligible / block`; the remediation Determination may contain `eligible / allow`. Neither changes `G2`. | Disposition is action-question-specific, not gate reinterpretation. |
| **Pure advisory prioritization.** A policy forms an exact `eligible / allow` Determination whose action question is limited to a non-mutating prioritization recommendation, and no effectful Action Intent is requested. | The Determination is the terminal result. No priority state is mutated and no Action Authorization Binding, Action Attempt, Action Terminal, or Mutation Receipt exists. Any later authoritative priority change requires its own action chain. | A pure disposition or advisory result is non-mutating and need not recurse into action authorization. |
| **Diagnostic result.** A diagnostic evaluation closes a valid failed gate. One policy permits that exact diagnostic role and direction to trigger remediation; the mandatory governed-completion role cannot accept it for positive closeout even if a policy attempts to permit it. | The remediation basis may form under its exact policy. Closeout reliance refuses for incompatible use intent. Permission for remediation, blocking, holding, or observation admission does not make the diagnostic result decision-seeking or permit a different positive action question. | Evaluation Use Intent acceptance is role-, direction-, and action-question-specific. |
| **Ambient favorable-result selection.** A caller asks Work Progression to use “the latest passing gate” for a target without an exact nominated-reference or owner/repository/time-cut boundary. | Reliance formation refuses. Repository order, current-head lookup, result direction, or caller-selected subset cannot establish the finite source-result population. | Complete result accounting requires an exact eligible boundary before selection. First governance after results exist is allowed only under G5's independent standard, justified boundary and complete failure accounting, never by requesting the latest passing gate. |
| **Historical-validity-only mode with later Engine changes.** A valid historical gate binds Evaluation Authorization `A1`. Engine later reports `A1` non-current while historical resolution remains `historically_valid`. In a comparison case, Engine reports historical resolution `historically_invalid` or `unverifiable`. | Under `historical_validity_only`, non-currentness alone does not alter the Determination. The historical-invalid or unverifiable comparison causes Reliance Refusal without rewriting the gate. | Historical validity and continuing-current authorization are separate Engine propositions. |
| **Continuing Engine dependency declared.** A high-risk deployment policy expressly requires the evaluation binding to remain current at reliance time, and Engine reports it superseded. | The action domain preserves the historical result but cannot treat the binding as current. Reliance refuses, no Determination is produced, and a fresh evaluation is required under the policy. | Policy-conditional Engine verification is mandatory when declared and owner-produced. |
| **Eligible but authorization refused.** Local closeout has an `eligible / allow` Determination, but the action-authorization owner refuses the exact actor, authority basis, effect, or scope. | The authorization owner produces one exact Action Authorization Refusal. No Action Authorization Binding, Action Attempt Identity, Action Terminal, mutation, or Mutation Receipt exists. The Determination remains immutable history. | Eligibility, authorization refusal, attempt provenance, and mutation truth are distinct propositions. |
| **Authorized but ineligible.** A broad action credential exists, but the current reliance result is `ineligible / block`. | No action-intent authorization may validly bind the ineligible result, and no mutation occurs. Broader credentials cannot override the action-domain decision. | Authority cannot improve semantic eligibility. |
| **Same kind and target, changed effect.** A Determination establishes `eligible / allow` for exact effect `E1` against target snapshot `T1`. A later Action Intent uses the same Action Kind and target label but requests materially different effect or effect scope `E2`. | Intent formation or authorization refuses the incongruence. A deliberately new reliance occurrence, Basis, Determination, Intent, and authorization path are required for `E2`. | An allow cannot be broadened through later Action Intent parameters. |
| **Target changes between allow and commit.** Local closeout is allowed and an exact Action Authorization Binding is issued for target snapshot `W1`, but the work-state owner now reports `W2`. | Final expected-state verification refuses. The prior Basis, Determination, and authorization remain history; a new reliance occurrence against `W2` is required. | Present-use closure does not bypass mutation-time concurrency. |
| **Uncertain commit followed by retry.** An authorized non-idempotent Action Attempt loses its definitive state-owner outcome and has an uncertain Terminal with no Mutation Receipt. The target presentation appears unchanged and the caller requests another attempt. | The ordinary retry refuses. It may proceed only after the authoritative effect/state owner positively establishes non-application or under a separately admitted duplicate-safe recovery rule and fresh authorization that explicitly account for possible prior commit. | Timeout, unchanged appearance, and recomputed eligibility do not prove non-commit. |
| **One child is locally eligible.** Child `C1` has exact Local Closeout Eligibility `eligible`; parent `P1` has another required child `C2`. | No Parent Eligibility Result can be inferred from `C1`. Work Progression must form the exact parent/child closure and account for `C2` under a Parent Eligibility Policy. | Local eligibility never implies parent eligibility. |
| **Complete parent closure.** Parent snapshot `P1` has exact required children `C1` and `C2`; the policy consumes current exact local results for both plus one parent-level gate, and all positively satisfy it. | Work Progression may produce one parent-action Operational Reliance Determination with `eligible / allow` facets. The parent remains unchanged and no promotion has occurred. | Parent eligibility is a separate pure composition facet. |
| **Parent closure omits a required relation.** The finite authoritative boundary contains child `C3`, but the proposed parent Basis accounts only for `C1` and `C2`. In a comparison case, the policy requires actual `C3` closeout but supplies only `C3` Local Closeout Eligibility. | Both cases refuse before Parent Eligibility. The first lacks complete relation membership; the second supplies the wrong child proposition and requires the declared current-state or Mutation Receipt proof. | Complete relation closure and proposition-role congruence precede parent eligibility. |
| **Dependency added after parent eligibility.** `P1` was eligible, then the work-structure owner adds required child `C3` before parent action. | The historical parent result remains exact for its old closure. Final currentness recheck stops the action; a new parent basis must account for `C3`. | Parent membership currentness is part of present use and cannot be patched in place. |
| **Parent eligible, promotion refused.** Parent eligibility is `eligible / allow`, but action authorization is superseded or the compare-and-write expected parent state no longer matches. | No promotion commits and no Mutation Receipt exists. The parent Determination is not rewritten to `ineligible`. | Parent eligibility, promotion authority, and mutation success are three propositions. |
| **Work Specification says completed.** A current Work Specification revision contains `completed` prose and cites a gate summary, but no exact owner result is resolved. | No Local Closeout or Parent Eligibility forms. The artifact remains authored project truth only. | Authored planning state and rendered references grant no operational authority. |
| **Later Work Specification records outcome.** After a valid local mutation receipt `M1`, a new Work Specification revision adds an Outcome Reference to `M1`. | The new artifact revision records the relationship. It neither caused `M1` nor changes the earlier reliance, authorization, action, or receipt. | Artifact references preserve causal direction and owner boundaries. |
| **Compact gate projection offered for action.** A UI supplies a concise gate view but the canonical gate result cannot be resolved. | Operational Reliance refuses. The projection remains useful presentation but cannot satisfy the exact-result floor. | Presentation density does not create action truth. |
| **Gate feeds posture through later owner stages.** A gate failure forms a valid posture-observation-specific Operational Reliance Determination and is separately admitted as a Posture Observation. A recommendation policy produces an exact Posture Recommendation. A distinct transition-specific Basis and Determination then close over the recommendation, exact current posture target, approvals, and other declared facts before Engine issues transition Action Authorization. | Only the final Engine-authorized compare-and-write may create the successor posture snapshot and Mutation Receipt. The originating gate and evaluation remain unchanged and the successor posture affects only a later evaluation. | Observation, recommendation, transition eligibility, transition authority, and mutation are separate and cannot form a same-occurrence cycle. |
| **Historical replay after policy supersession.** An old Basis, Determination, and Action Terminal are replayed after the Operational Reliance Policy has a successor. | Exact replay returns the old historical results and policy identity. It grants no current disposition or authority under the successor; present use requires a new occurrence. | Historical reconstruction and present reliance are distinct. |
| **Exact attempt replay.** The same Action Attempt Identity and exact attempt content are delivered again after its Terminal and Mutation Receipt closed. A comparison case changes the effect under that attempt identity. | Exact replay returns the same Terminal and Receipt without re-executing. The comparison refuses conflicting identity reuse. | Attempt replay and a deliberately new execution are distinct. |
| **Transport reports success without receipt.** A CLI or host reports a successful call, but the state owner cannot establish whether the write committed and emits no Mutation Receipt. | The effect is uncertain and MUST NOT be presented as a successful mutation. Eligibility and authorization cannot repair the missing state-owner truth. | Transport success and action intent are not durable mutation truth. |
| **Standalone advisory authority is explicitly permitted.** A local CLI action policy permits advisory authority for one exact Action Kind. A user prompt supplies the intent, claimed principal, claimed delegation, and declared authority; the process also has ambient operating-system permission to write the target. | The action-authorization owner may issue a Binding that preserves the prompt provenance and labels the authority facts advisory. The attempt still requires the exact `eligible / allow` Determination, target-currentness and attempt checks, and through-commit state-owner verification and Receipt. Neither the Binding nor ambient OS permission is represented as independent authentication, verified delegation, or fine-grained Handbook credential. | Advisory authority can participate in the complete effectful chain without fabricating stronger assurance or bypassing mutation safeguards. |
| **Prompt offered where enforced authority is required.** Another Action Authorization policy requires an independently authenticated principal and host-enforced permission. A prompt or configured agent role declares both but supplies no owner-verifiable fact. | Authorization refuses. The declared claims cannot satisfy the stronger policy, and advisory mode cannot waive the requirement or target/currentness checks. | Policy-required enforcement cannot be fabricated by advisory inputs. |
| **Agent-authored policy candidate.** An agent proposes an Operational Reliance Policy and cites one Decision Record by Markdown link, omits the exact scope and candidate population, and asks the admission owner to judge whether it is generally safe. | Policy admission refuses. A valid Policy Admission Closure requires exact candidate content, typed roles and scope, relevant exact governance-source revisions and dependency modes, complete selection/accounting, and mechanical invariant checks. It neither requires every artifact family nor turns the Decision Record into runtime evidence or a waiver. | Candidate authorship and organizational rationale are permitted, but deterministic owner admission is separate and bounded. |

Additional governed-completion scenarios:

| Case | Required result |
| --- | --- |
| An agent submits a direct pass from a Decision-seeking eligible contract as the required final assessment. | An earlier direct pass alone causes Reliance Refusal. Under an expressly adopted retrospective protocol, new G5 qualification may accept the exact result for declared roles; required prospective provenance or unsupported/missing coverage refuses. Permitted use is not assignment. |
| A fully ready posture-selected diagnostic run passes. | Valid historical truth; cannot fill the mandatory decision-seeking completion role. Remediation and other declared non-substituting uses remain separate. |
| An agent creates a weak fresh contract, assigns a promising name, or selects a weaker admitted completion policy. | Qualification applies to fresh candidates too. Admission alone is not governing assignment; the action owner independently resolves the actual standard and scope. |
| A required mapping is unavailable, or a caller narrows the target to a convenient child. | Missing governing applicability refuses; no exemption or direct fallback. A legitimate child result stays bounded and cannot discharge the parent's obligation. |
| Complete-pruning result considered for governed completion. | An exact eligible-contract, protocol-qualified under G5, decision-seeking assessment with actual governing coverage may be considered under the exact outcome policy; the existing all-N/A policy produces `not_applicable / no_effect`, not closeout. Unqualified direct, diagnostic or wrong-target substitution refuses; no prospective provenance is fabricated. No empty-route result itself grants action authority or proves a mutation. |
| Governing requirements change after a qualifying pass but before completion. | Old truth remains history. Stop when current coverage fails; obtain new qualification or specific newly required assessment/observation work under G5 and new owner decisions. An unused new definition alone does not cause this. |
| The exact qualifying assessment and reliance exist and the action rule accepts an ordinary user instruction as advisory authority. | The effect may proceed with target and commit checks. No additional login, human confirmation, or fabricated authentication follows from this amendment. |
| Current governing requirements select the exact unchanged eligible C@1/API question for T1; the adopted supported retrospective protocol permits earlier R1 with original direct/decision-seeking provenance. | Resolve current requirements/assignment independently, qualify applicable definitions/use, account for the entire eligible result population and all required roles, and form a new owner determination accepting R1 now for the API role if exact coverage/currentness/process conditions verify. R1 still reports no historical shipment assessment. Other roles, outcome policy, separate delegated action authority and commit checks remain. No new contract revision or repeated observations solely for governance establishment. |
| The action target changed from evaluated T1 to T2, even though contract names and displayed pass are unchanged. | No automatic rebinding or semantic-equivalence inference. Refuse unsupported target coverage; obtain the supported new assessment/observations for T2 and fresh current owner decisions. R1 remains exact truth about T1. |
| Target is unchanged, but a required current fact is stale, a shipment role lacks coverage, or a required identity is unavailable. | Refuse the affected formation boundary; identify new owner verification, assessment or collection needed. An accepted API role cannot hide missing security coverage. Do not manufacture evidence import, weaker requirements, automatic whole-suite reruns or automatic zero reruns. |
| Shipment's adopted protocol requires governing selection/approval before collection and original posture-selected requirements; R1 was independent. | Preserve R1; refuse it for that required role. Form the prospectively governed decision-seeking evaluation with its own Purpose/occurrence and required new work. Current approval does not fabricate original governance or collection-time authority; unrelated observations are not automatically repeated. |
| During first setup the user and agent already know Rpass and relevant Rfail; requirements/population are independently grounded in the actual obligation. | First establishment remains possible under G3 and initial human approval without an earlier higher-order rule. Bind exact justified finite boundaries, retain known-result/selection provenance and completely account for relevant failures, exclusions and missing roles. Activation supplies no eligibility; new current qualification applies the outcome rules to all required inputs. |
| A setup/replacement proposal lowers a requirement, narrows scope, shifts the time cut or nominates only Rpass because Rfail would block shipment. | Refuse result-fitting selection, even if a human liked the pass or the candidate calls itself safe. A favorable result supplies no authority or independent requirement basis. An ordinary higher-order selector cannot substitute a non-governing standard or erase failure accounting. |
| After establishment, an agent proposes a supported revision after inspecting results; prior approved terms explicitly delegate that revision process. | Validate prior authority and exact predecessor plus G5's independent requirement/population basis and complete accounting before activation. No repeat human prompt solely for an owner operation. A reserved change, missing prior authority or a successor claiming its own permission refuses/escalates; without the supported prior authorization retain the higher-order choice restriction. |

### Settled versus deferred

This record settles:

- Historical Semantic Result, Operational Reliance, Action Kind, Action Target
  Snapshot, Action Question, Operational Reliance Policy and Occurrence Identity, Operational
  Reliance Basis and Determination, Current-Use Eligibility, Reliance Refusal,
  Operational Disposition, Action Intent, Action Authorization Binding and
  Refusal, Action Attempt Identity, Action Terminal, and Mutation Receipt as
  non-substitutable concepts;
- exact action-question closure over kind, target, effect, effect scope, and
  every eligibility-relevant parameter before reliance, plus mandatory Intent
  and authorization congruence;
- the common exact-result, current-policy, current-target, congruence,
  declared-currentness, and complete-accounting floor for present use;
- independently established governing-completion applicability/requirements, actual target coverage, mandatory Decision-seeking eligible/decision-seeking assessment roles under the adopted supported protocol, prospective governing selection or new current qualification under G5, exact outcome-policy treatment of complete-pruning results, and governing conditions through commit;

- finite exact eligible historical-result boundaries, diagnostic provenance by
  role/direction/action question, and refusal/replay behavior;
- universal historical resolution and validity of the exact Evaluation
  Authorization Binding, mandatory explicit `historical_validity_only` or
  `continuing_current` policy mode, and fresh Engine verification exactly when
  a reliance or later authorization policy declares a present Engine-owned
  dependency;
- fresh action authorization, exact Action Attempt identity, and commit-boundary
  authorization/target/through-commit verification that no policy may disable;
- the exact eligibility states and fixed disposition mapping as facets of one
  atomic Operational Reliance Determination, plus the formation-refusal
  boundary;
- Work Progression as semantic owner of local closeout and parent eligibility,
  including target/dependency closure, policies, bases, results, currentness,
  accounting, and replay;
- Local Closeout Eligibility and Parent Eligibility Result as separate pure
  decisions with separate policies and currentness closures;
- finite authoritative parent-relation boundaries, explicit relation reach and
  cycle treatment, complete membership accounting, and non-substitutable child
  proposition roles;
- the rule that local eligibility never establishes parent eligibility, parent
  eligibility never establishes promotion, and every state effect requires
  separately authorized action and state-owner receipt;
- prohibition of ordinary non-idempotent retry after uncertain effect or
  commit absent authoritative non-application or admitted duplicate-safe
  recovery with fresh authorization;
- explicit placement of every introduced object in the canonical six-category
  ontology without creating another Artifact Instance family;
- deterministic owner-side Policy Admission Closure for user-, agent-, artifact-,
  or tool-authored policy candidates, with exact relevant governance inputs,
  selection/accounting, mechanical non-relaxable checks, and no bare-link proof,
  authored-text waiver, open-ended AI safety judgment, or recursive self-admission;
- Action Authorization Binding as a semantic result/provenance contract rather
  than a mandate for a general Handbook IAM platform, including explicitly
  policy-permitted advisory authority and its non-substitution for authenticated,
  owner-verified, or host-enforced facts;
- Work Specification reference and causal-order boundaries;
- the gate-to-posture observation/recommendation/transition boundary; and
- currentness, invalidation, retry, exact replay, and scenario outcomes above.

Deliberately deferred are all concrete Local Closeout, Parent Eligibility,
release, deployment, remediation, posture-observation, or other action-policy
content; thresholds; selected result populations; concrete realization of the
[selected work composition and relation rules](amendments/work-progression-kinds-and-relations.md#selected-kinds-and-hierarchy);
work-state or parent-state machine and operations, reopen/cancel/removal
consequences; actor and approver rules; concrete
authorization mechanism; mutation effects; persistence and retention policy;
and the physical placement of Work Progression, action authority, or state
ownership. No concrete parent promotion, release, deployment, or posture
transition is selected.

Also deferred to later human-approved task-level planning are schemas, fields,
Rust types, APIs, operations, repositories, registry/resolver mechanics,
identity allocation, cache implementation, status-code encoding, crate/module
placement, SDK/CLI/Tauri/Flow/Pipeline/skill integration, UI and explanation
projection, task slices, acceptance criteria, implementation sequence, and
rollout. This record does not authorize changes to completed HCM-5.2 source or
claim that the proposed Work Progression owner exists today.
