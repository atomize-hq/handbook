# Work Progression: contracts, relations and governance design experiment

**Status: PROPOSED product-model design for human discussion — 2026-10-07.**
This paper experiment recommends selections; it adopts no model, prepares no
formal slice/packet authority and authorizes no implementation. The two
[kind research](work-progression-configurable-kinds-research-2026-10-06.md) and
[decision exercise](work-progression-kind-decision-exercise-2026-10-06.md) reports
remain proposed and unchanged. Initial checkout: branch
`feat/handbook-contract-membrane-clean`, HEAD
`70019b7356bd2451b2d72d389741bec0f7f1f525`, with concurrent staged, unstaged and
untracked work. Only this new report is written by this experiment.

## Recommendation and source capsule

Recommend **bounded native work composition, optional immutable versioned kinds,
explicit contract-output/requirements/result roles, and independently resolved
scope obligations**. Use a single-parent containment forest plus direct local
prerequisites, allowing cross-branch prerequisites within one project. Require
actual closed-child truth for the proposed parent closure policies. Keep grouping,
display order and informational references outside completion unless an explicit
supported policy gives a fact a declared input role.

For governance, recommend an explicitly authorized applicability definition that
collects applicable obligations from project/scope rules, exact kind-associated
references, supported item selections and explicitly assigned parent scope rules.
Apply compatible obligations cumulatively through declared supported composition;
reject unresolved applicability or incompatible composition. Do not choose a
winner by tree position or guess which policy is stricter. A kind can standardize
a compatible local composition, but it cannot assign itself as the project's
governing standard or remove an independently applicable obligation.

The following is the focused source capsule, rather than a new source registry.
“Approved” below means semantic input to planning, not implemented capability.

| Source and status | Controlling use in this experiment |
| --- | --- |
| [Program entry point](../specs/handbook-contract-membrane/README.md) and [semantic package](../specs/handbook-contract-membrane/semantic-model/README.md#approved-decision), approved package 2026-09-30 | Authority classification; Work Progression semantics do not prove that a native work owner exists. |
| [Independent governed completion](../specs/handbook-contract-membrane/semantic-model/operational-reliance-and-actions.md#independently-governed-completion), [Work Progression ownership](../specs/handbook-contract-membrane/semantic-model/operational-reliance-and-actions.md#work-progression-ownership), [local closeout](../specs/handbook-contract-membrane/semantic-model/operational-reliance-and-actions.md#local-closeout), [parent eligibility](../specs/handbook-contract-membrane/semantic-model/operational-reliance-and-actions.md#parent-eligibility-and-parent-action), approved | Separate requirements, current qualification, finite relation closure, pure eligibility and actual state effects. |
| [G1–G6](../specs/handbook-contract-membrane/semantic-model/amendments/governance-defaults-and-delegated-operation.md#approved-rules), approved 2026-10-04 | Explicit choices/delegation; intrinsic conformance versus project qualification versus assignment; independent evaluation and supported current qualification; packs grant no authority. |
| [Approval dimensions](../specs/handbook-contract-membrane/semantic-model/amendments/approval-authority-and-assurance.md#independent-dimensions), human-selected intent 2026-10-05 | Required decision, authorized decision maker, delivery, identity assurance and composition remain distinct; a local assertion cannot fabricate actual review. |
| [Shared admission](../specs/handbook-contract-membrane/semantic-model/objects-and-governance.md#shared-policy-admission), [Contract permitted use](../specs/handbook-contract-membrane/semantic-model/objects-and-governance.md#contract-permitted-use-and-qualification), [exact references](../specs/handbook-contract-membrane/semantic-model/objects-and-governance.md#exact-references-do-not-transfer-authority), [Work Specification relationship](../specs/handbook-contract-membrane/semantic-model/operational-reliance-and-actions.md#work-specification-relationship), approved | Definition establishment does not activate governance; artifacts and references do not produce operational truth. |
| [Two requirements sources](../specs/handbook-contract-membrane/semantic-model/assurance-and-evaluation-preparation.md#complete-evaluation-requirements-from-two-sources), [typed exceptions](../specs/handbook-contract-membrane/semantic-model/assurance-and-evaluation-preparation.md#exceptions-are-typed-authority), [Engine/Coordination ownership](../specs/handbook-contract-membrane/semantic-model/assurance-and-evaluation-preparation.md#semantic-owner-placement-and-identity), approved | Direct and posture-selected requirements remain exact; neither provides operating authority; exceptions have separately attributable authority and effects. |
| [Architecture](../specs/handbook-contract-membrane/architecture-context.md#semantic-ownership-and-dependency-direction), [SDK-first sequencing](../specs/handbook-contract-membrane/phase-and-slice-map.md#current-delivery-sequencing), current boundaries | Native owners decide; SDK composes; later CLI/skills/adapters preserve owner truth. This experiment changes no current packet or delivery order. |
| [Format standard](../specs/handbook-contract-membrane/structured-data-format-standard.md), proposed general defaults with Contracts JSON fixed | No work schema, serialization, registry layout or human-view format is selected here. Exact identities below are conceptual, not serialized fields. |
| [Work-unit lifecycle](../specs/handbook-contract-membrane/work-unit-lifecycle.md) and [review policy](../specs/handbook-contract-membrane/review-and-closeout-policy.md#causal-review), current engineering policy | The checkout names do not redefine the repository's authorized slice/packet lifecycle. One focused local reviewer assesses this complete report under P1–P4. |

**Already follows approved semantics:** attribution of owners; exact immutable
inputs/results; separate qualification and assignment; positive applicability;
complete finite accounting; currentness and through-commit checks; independent
contract use; G2 delegation; eligibility distinct from completion; prior authority
for replacements. **New proposed selections:** the relation menu/cardinality/reach,
contract roles on work items, applicability composition, checkout policy content,
and optional kind use. None is a claim of a shipped feature. Physical owners,
storage and APIs remain unselected.

## Concrete checkout feature and bounded relations

Assume project P already has human-approved initiation, Charter/posture, governance
choices and bounded delegation. W names a future authoritative work-state/structure
responsibility, not a selected crate or service. W establishes items, exact
bindings, relation cuts and state mutations. Work Progression independently closes
those facts into eligibility. Contracts owns definitions and assessment truth;
Engine owns applicable posture/operating-authority truth; action authorization
remains separately attributable even if hosted beside W.

All IDs, contracts, policies and outcomes in this report are invented paper
subjects. Assume future owners support the declared bounded propositions; absent
such support, the affected operation refuses. “Phase”, “Slice”, “Packet” and
“Task” are illustrative local names, not fixed universal depths or authority roles.

```text
F  Feature: Guest checkout
├─ PL  Phase: Plan checkout
│  └─ CS  Slice: Specify checkout
│     ├─ CA  Task: Author checkout contract
│     └─ CV  Packet: Validate contract and close planning outputs
└─ IM  Phase: Implement checkout
   ├─ API  Slice: Checkout endpoint
   │  ├─ A1  Packet: Build endpoint candidate
   │  └─ A2  Packet: Verify endpoint delivery
   ├─ UI  Slice: Checkout interface
   │  └─ U1  Task: Build and verify guest checkout form
   └─ SEC  Slice: Security assessment
      └─ S1  Task: Perform required security review
```

PL and IM each have their own completion question and required direct child
closure. F asks whether the integrated checkout outcome meets its requirements,
in addition to PL/IM actually closing. Leaf items can have both local requirements
and outgoing prerequisites. A Task may later contain Packets, a Phase may be a
timebox in a different supported composition, and small work may combine supported
roles in one item. No spelling alone selects those meanings.

| Relation/role | Concrete meaning and exact population | Proposed initial scope/reach |
| --- | --- | --- |
| Containment | The tree above is W-owned membership. For example IM directly contains API, UI and SEC. | At most one containment parent per item; same project; finite forest, no cycles. Each parent closure policy consumes its complete direct-member cut. No transitive result rollup. |
| Prerequisite | CV requires CA actually closed; A1 requires PL actually closed; A2 requires A1 and S1 actually closed; U1 requires A1 and S1 actually closed; S1 requires A1 actually closed. | Directed from dependent to prerequisite, direct same-project work-item targets. Cross-branch edges are supported, including A2 → S1 and U1 → S1. Each edge declares the guarded operation and actual required proposition. |
| Required output | CA must produce one exact owner-established checkout Contract C@1 and its compatible definition closure. CV verifies that exact output and applicable planning obligations. | Exact artifact/definition role, distinct from an item dependency or state. Before C exists, record the required output question, not a fabricated exact reference to a future definition. |
| Requirements input | A1/A2/U1 bind exact C@1 with supported coverage of their bounded delivery questions. | Required input under the consuming policy; a reference alone establishes neither applicability nor satisfaction. Separate binding/coverage checks apply. |
| Informational reference | CA cites an earlier payment incident or decision rationale; a later Work Specification records an issued receipt. | Navigation/provenance only unless separately declared as a semantic input with owner, role and currentness. It does not imply a prerequisite. |
| Grouping | “Wave A” groups A1; “Wave B” groups A2, U1 and S1 for a planning view. | Finite named membership; no work-state or completion effect. Group membership is separate from containment. |
| Ordering | A view displays API before UI and SEC, or Wave A before Wave B. | Finite display/planning order; no start authorization, completion gate or scheduler. Enforced sequencing must use a supported explicit prerequisite. |

For this example, CV → CA, A1 → PL, A2 → A1/S1, U1 → A1 and S1 → A1 guard
the named dependent's execution admission and close action. **U1 → S1 guards
close only:** U1 can build the form after A1 closes, before S1 reviews it.
Prerequisites do not universally guard every edit or planning activity.
Require actual current closed-state truth plus the exact closure receipt linkage,
not eligibility. Starting an A1 implementation action still needs its own operating
and action authority; PL's receipt supplies no inherited authorization. A1 closes
only its explicitly bounded candidate-delivery obligation, assessed with its own
required API coverage and a separate prospective security assessment of that
candidate, obtained before A1 closes. That earlier assessment is distinct from S1.
After A1 closes and U1 builds its UI candidate, S1's security evaluation binds the
exact fixed endpoint and UI candidate snapshots for the broader A2/U1 delivery
questions. S1 needs those existing subject inputs, **not U1's closure**, so it
does not introduce a reverse closure prerequisite. Any later candidate mutation
requires the affected currentness/coverage checks and new assessment work.
S1's results cannot replace F's integration assessment. The narrower A1 outcome
cannot be presented as completed endpoint delivery.

Reject self-prerequisites and dependency cycles. Validate the finite combined
**closure dependency** relation: parent close requires declared direct children
closed; a prerequisite-guarded close requires its prerequisite closed. Thus CA
requiring PL closed is rejected: PL needs CS, CS needs CA, and CA would need PL.
Adding A1 → S1 also refuses because S1 already requires A1. This is a bounded
structural consistency check over the selected relations, not a universal temporal
planner or support for arbitrary condition graphs. Later relation changes repeat
the same check against one coherent owner cut.

Grouping and informational links may form navigation cycles without becoming
execution cycles. This does not permit same-occurrence semantic feedback in a
Contracts evaluation. The approved [causal rules](../specs/handbook-contract-membrane/semantic-model/objects-and-governance.md#causal-and-anti-cycle-rules)
still govern inputs/results. No external prerequisite state/effect adapter, multiple
containment parents, arbitrary predicate, dynamic hook or open-ended traversal is
selected. Cross-feature local prerequisites within P may use the same supported
direct relation; cross-project/external prerequisites need later exact resolver and
authority support. Unavailable verification refuses rather than copying “done”.

Parent scope obligations described below may deliberately cover a finite descendant
set. That separate applicability reach does not make parent eligibility recursively
consume every descendant's result. W supplies one exact complete descendant cut
for that applicability rule; required direct child closure establishes each level's
actual closure, with the parent's own obligations independently assessed.

## Follow C@1 from planning output to implementation completion

C@1 states checkout acceptance questions: accepted cart inputs, idempotent retry,
payment-failure handling and exclusion of raw card data. Its compatible definitions
specify exact claims/cases, applicability, observations/evaluators, evidence rules,
an explicitly selected metric inventory (possibly empty) and gate interpretation.
A text description or JSON syntax alone is insufficient. C@1's illustration uses
owner-established **Decision-seeking eligible** permitted use; that property says
which intent is allowed, not that the contract governs checkout or that it passes.

| Step | Attributable truth and required distinction |
| --- | --- |
| Plan | Pre-existing Work Specification WS@1 describes F's intended outcome and CA's output question. W records CA's operational target and supported completion composition. WS@1 is authored intent, not a work-state owner, executable policy or receipt. |
| Produce | The delegated author proposes C and compatible definitions. Contracts establishes exact complete C@1 under native owner rules. W/Work Progression resolve this issued definition as CA's required output. Intrinsic validation can succeed before project qualification or governing assignment. |
| Qualify and review where applicable | The exact active project/scope admission/use rules independently determine any required project qualification and any separately typed approval/review roles. Satisfy those roles under their actual authority; a review result is not conformance, assignment or activation. This example assumes planning review is delegated under approved rules, rather than requiring a new human review of every contract. |
| Close planning work | CA and CV need their own required decision-seeking assessments of authoring/output adequacy and applicable process obligations. These may inspect C@1 as subject data through a supported assessment role. Running C@1 against checkout implementation is a different question. No universal “validation succeeded, so CA is closed” shortcut or mandatory recursive validation-contract chain is introduced. After eligible/allow and separate authorization, W commits their closure receipts; CS and PL each separately qualify and close. |
| Assign implementation requirements | W records exact C@1 requirements bindings for A1/A2/U1, without rewriting WS@1. Work Progression independently resolves current governance for their actual effects/targets/scopes. An authorized governing relationship selects applicable policies, protocol and coverage, including the supported exact C@1 roles and any additional security obligations. C@1 can be an implementation input even before that assignment; dependent governed completion cannot proceed without the assignment. |
| Evaluate implementation | This illustration chooses a prospectively governed protocol for production delivery: Contracts Assurance Selection supplies complete `posture_selected` requirements under the applicable admitted mapping, including the exact C@1 definition and declared role coverage. Engine independently authorizes the exact Purpose/plan/reach from approved operating authority. Contracts Coordination checks capability/readiness and closes collection; Contracts supplies immutable evidence, claims, metrics and gate truth for the fixed implementation snapshots. C@1 is an input; result R-A2 is an assessment, not the contract or work state. |
| Qualify current reliance | Work Progression verifies governing assignment, target/coverage, complete result populations, permitted use, original `decision_seeking` intent, required process conditions, historical Engine validity and declared present-currentness. It also consumes exact prerequisite closure and every other required role. A gate pass alone cannot establish this closure. Valid unfavorable assessments follow the selected non-positive policy; malformed/missing required formation facts refuse. |
| Commit close | One local Determination with `eligible / allow` permits consideration of A2's exact close action. The action owner separately authorizes the congruent Intent; the admitted Attempt proceeds only with required owner checks. W enforces the same target, governing dependencies and declared conditions through durable commit and issues a Mutation Receipt. A2 is actually closed only then. A later WS@2 may reference that exact result/receipt; the mutation does not rewrite WS@1. |

The scenario's contract adequacy and security policy content are proposed examples,
not supplied executable definitions. A1/A2/U1 do not each pretend to satisfy every
checkout claim: exact owner-supported coverage relates their bounded target
questions to C@1's claim roles, and F has its separate whole-checkout assessment.
Every evaluation under C@1 preserves and accounts for its complete contract-rooted
expected-claim inventory through declared applicability; bounded result-role
coverage never selects a favorable subset or silently removes expected claims.
Unsupported coverage refuses. A prose assertion of “equivalent tests” is insufficient.
The available validation/evaluation mechanisms also do not prove that the chosen
engineering requirements are substantively adequate.

**Independent use remains first-class.** In another initialized project Q whose
exact scope is positively deliberately deferred, an authorized caller may author,
intrinsically validate and directly evaluate complete C@1 without creating CA,
F, a kind binding or any pack. Contracts resolves full `direct_contract`
requirements and compatible intent; independent Engine authority, capability,
readiness and evidence/truth closure still apply. A useful explanation is:
“Checkout retry: pass under exact C@1; requirements source: direct contract;
governance: deliberately deferred for Q's checked scope; governed delivery: not
assessed.” Complete contracts can also be used where established governance
positively resolves no qualification applicable, or required qualification succeeds.
Unknown/damaged governance state cannot be called deferred. Authoring/inspection
may continue while an executable use is refused.

If later governance expressly adopts bounded retrospective acceptance, a new
Work Progression determination may qualify an exact earlier direct,
decision-seeking result for declared current roles under G5. It must establish
independent requirements/protocol, exact coverage, currentness, original authority
and complete accounting including relevant failures. It cannot relabel the old
result, manufacture collection-time review, or import old evidence into a new
occurrence. If the adopted role demands prospective posture-selected provenance,
that earlier result cannot fill it. Both protocols are approved possibilities;
the prospective selection here is a proposed checkout choice, not a universal rule.

## Policy applicability: compare and select explicit cumulative obligations

Distinguish project **admission/use qualification** for definitions from actual
**governing assignment** for a work effect and from **current qualification** of
assessment results. Applying a rule in one dimension does not select the other two.

| Approach | Advantage | Failure/cost in this example | Recommendation |
| --- | --- | --- | --- |
| Item-only selection | Every item carries visible policy choices. | An author may omit project security obligations; repeated choices drift unless independently constrained. | Retain explicit item inputs, with independent applicability resolution. |
| Implicit parent inheritance / nearest parent wins | Familiar short configuration. | Moving U1 or adding a weaker parent can silently drop security; PL's contract-output rule may incorrectly spread to implementation. | Do not select. Parent relations alone grant no policy authority. |
| Global most-restrictive-wins / arbitrary merging | Appears to prevent weakening. | No universal strength order exists for “no raw card storage” versus “retain exact payment audit data”; blending populations, process timing or gates can change meaning. | Do not select. Approved bounded posture monotonicity is not global policy ordering. |
| Explicit scope applicability with supported cumulative composition | Obligations stay attributable; local reuse and additional requirements fit without precedence guesses. | Requires a complete finite applicability cut, compatibility checks and actionable conflict explanations. | **Select this bounded direction.** No generic rule engine or unrestricted policy algebra. |

Proposed resolution for one actual close operation:

1. Resolve its exact project, effect, target, scope and authoritative membership
   cut. A label or kind name does not supply these facts. Resolve current
   admission/use governance and independently authorized governing applicability
   from the exact active basis before forming dependent closures.
2. Collect every obligation matching the supported explicit applicability
   definition, with source identity, role, scope, required assessment/process
   proposition and currentness. Project-wide or feature-descendant reach must be
   expressly selected. A parent contributes descendant obligations only through
   an authorized scope rule, not merely because it contains the item.
3. Resolve kind-associated exact policy references and explicit item settings as
   compatible proposals/bindings through supported roles. A kind's whole-composition
   meaning cannot be silently overridden. A materially different local composition
   needs an established variant or explicit composition, without waiving governance.
4. Check supported combination and complete accounting. Preserve separate
   obligation roles and exact populations; do not blend gate meanings or choose
   favorable candidates. Count identical owner-established obligation/role/scope
   identities once when the declared resolver permits that; repeated policy names
   across distinct targets/roles are not duplicates. Additional compatible
   obligations remain separately satisfied within the declared closeout composition.
5. Refuse unresolved applicability, unsupported combination, incompatible
   requirements or incomplete boundaries before a Determination forms. Interpret
   valid adverse results using the established outcome policy; a missing required
   governing relationship is not an `indeterminate / hold` convenience result.

This selects a finite supported applicability/composition capability, not public
APIs, a policy selector syntax or a new runtime record family. Exact selection,
comparison and refusal remain native-owner responsibilities. LLM advice may explain
or propose remediation but cannot settle an owner-declared incompatibility by fiat.

**Checkout assignments.** Assume approved `Secure delivery@1` requires an independent
security assessment for governed implementation-close effects in F's payment scope,
including its finite matching descendants. Its applicability follows actual
effect/scope facts, not whether U1 is called Task, Experiment or Prototype. A1's
narrow candidate close still needs the security coverage applicable to that bounded
effect; A2/U1 need their declared review roles from S1. If one review can cover
multiple targets, the owner must explicitly support those exact target/role mappings,
current snapshots and coverage; an ancestor review is not universal reusable proof.
CA/CV face the separately selected authoring/planning obligations, not an automatic
requirement to pass implementation tests before code exists.

F additionally has parent-only `Checkout integration@1`: end-to-end payment failure
recovery and integrated accessibility acceptance over F's exact implementation
snapshot, plus direct PL/IM actual closure. These parent integration roles are
explicitly target-only, so they do not spread to every descendant. A2/U1 receipts
do not satisfy F's integration assessment. A kind such as `Implementation task@1`
may reference compatible `Bounded delivery@1`; it cannot replace Secure delivery.
An item can add a supported latency assessment; omitting it later requires prior
authorized revision and cannot remove the security role. This combination is
satisfiable because each separate proposition can be true under the declared
coverage and process rules; it is not a fabricated merged “super-policy”.

## Weaker proposals, conflicts, exceptions and authority

Suppose `Fast task@1` proposes local close after API tests, with no independent
security review. U1 proposes that composition after observing a failing security
assessment. Its supported local delivery rule may remain useful, but the actual
governed U1 close also owes Secure delivery@1. If the required review role is
missing, completion refuses the missing formation/input condition; if the complete
valid review is unfavorable, the selected closeout policy returns
`ineligible / block`. Renaming U1 or moving it between IM's branches cannot remove
the feature's still-applicable scope obligation. Moving it outside the declared
scope requires authorized truthful target/membership changes; it cannot falsely
discharge F's obligation or conceal removed required work.

Contrast genuinely incompatible requirements: a proposed C@2 requires retaining
raw card numbers, while Secure delivery@1 forbids it for that same target/use.
Both may be intrinsically intelligible; they cannot jointly govern this checkout
operation. Reject the dependent assignment/requirements composition under the
declared compatibility rule. Do not select whichever is “stricter”, report a pass
for their union or run a smaller contract as if it discharged the original work.
Correct C@2 or seek an authorized supported governance revision; replacement uses
prior terms, not C@2's request or a passing result. Missing/unreadable applicability
is a separate resolution failure, not evidence that security is inapplicable.

Example agent explanation, expressing semantic facts rather than a proposed DTO:

> U1 close cannot proceed. Secure delivery@1 applies to the actual payment-scope
> implementation effect under P's active assignment and F's exact descendant cut.
> Fast task@1 supplies API testing, which does not satisfy the independent security
> role. The resolved security assessment failed for target snapshot T-U1. Repair
> the findings and obtain the required current assessment within existing delegation.
> This agent has no authority to waive the role. The historical failure remains.

For a conflict, instead identify the two exact obligation sources, the incompatible
propositions and target/use, the failed supported compatibility condition, and
which authorized revision or reserved decision is needed. For unresolved
applicability, identify the missing/unverifiable owner fact and permitted recovery.
No invented status codes or public message APIs are required to make this useful.

**Bounded exception example.** Suppose a *previously authorized* typed
`Security review exception@1` expressly permits a designated security delegate
to substitute a qualified security agent for a human reviewer on one exact
non-production sandbox UI close. It specifies issuer, target/scope, effect,
compensating requirements and expiry. An exact authorized/admitted record X@1
may apply only that substitution to U-sandbox; actual independent review must
still occur and its result remain truthful. X@1 cannot apply to production U1,
remove the mandatory decision-seeking assessment, broaden permitted use, fabricate
past review or turn an unfavorable assessment into a pass. Expiry or changed scope
at a required through-commit check stops that action. Without the prior supported
exception mechanism and exact record, the agent has no exception path. This is a
proposed example of the approved typed-exception boundary, not a shipped exception
policy or a recommendation that every project permit it.

| Operation | Separately attributable authority/check |
| --- | --- |
| Author C, kind or policy candidate | Human/agent authority for the bounded authoring activity; candidate authorship creates no executable or governing authority. |
| Validate/establish exact definitions | Contracts or the relevant definition owner applies exact conformance/compatibility and permitted-use rules; syntax validation and conversational approval do not perform this. |
| Qualify definition/use | Applicable project admission/use rules and the responsible owner's Policy Admission Closure; intrinsic completeness alone cannot bypass required qualification. |
| Assign/activate governing obligations | Authorized governance/applicability and domain assignment lifecycles for actual effect/target/scope; available population and kind installation do not assign. |
| Revise/rebind/change membership | Prior approved replacement/change terms, expected predecessor and exact owner validation; new terms or replacement content cannot authorize their own activation. |
| Execute checks | Independent Engine operating authority, exact plan/Purpose/reach, capability/readiness and producer/source rules; requirements and work prerequisites grant none of this authority. |
| Determine eligibility and close | Work Progression's exact current determination, separate action authorization/attempt and W's guarded durable mutation/receipt. Eligible, approved and attempted remain non-substitutable. |

G2 permits routine supported operations under initial delegation without another
human prompt solely because a new owner checks an operation. Review decisions may
be delegated to a qualified project agent if the exact prior rules permit; required
independence/composition must actually hold. A configured role name grants none of
this by itself. Human review is needed for initially reserved choices, genuinely
unresolved material ambiguity or missing authority without an authorized resolution,
not universally for every contract, qualification, action or revision. Advisory
authority stays labeled as such; this report selects no credential platform.

## Active revisions, changed governance and historical closure

Consider each event against its exact affected boundary, rather than assuming a
global migration or rerun.

| Event | Proposed operation and consequence, within approved invariants |
| --- | --- |
| C@1 → C@2 adds a retry-after-cancellation claim | Contracts establishes a new immutable revision and compatible definition closure under the same applicable qualification rules. Updating an authorized head does not change A2/U1 bindings, assign C@2 or validate implementation against it. Their existing exact C@1 inputs remain recorded. |
| Active A2 adopts C@2 | Supported explicit authorized rebinding establishes a new A2 snapshot/binding with expected predecessor. Reconcile its actual requirements/coverage and current governance. Changed evaluation inputs require fresh Purpose/occurrence and affected owner closures; do not rename R-A2 as a C@2 result. If this rebind is unsupported, refuse it and expose the limitation. |
| Kind defaults recommend `Implementation task@2` | New proposals may use it. Active items keep exact v1 unless supported authorized rebinding commits. The kind remains optional; explicit composition can express the same supported semantics. A label/default change supplies no policy exception. |
| Governance changes G@1 → G@2 while U1 is active | Prior authorized governance terms govern replacement and committed cutover. Resolve independent current applicability at dependent checkpoints even if U1 pins its older kind/contract. New review/coverage conditions may invalidate present reliance; old content/history stays exact. Qualification of unchanged C@1 under a new applicable basis need not create C@2, but requires the actual owner outcome. |
| A previously passing direct R1 is offered under G@2 | Resolve G@2's supported protocol and requirements independently of R1. Retrospective acceptance needs a new exact current qualification for each declared role and complete populations, including failures. Required prospective provenance, stale target or added claim coverage demands the corresponding new work; merely passing R1 is insufficient. |
| IM was eligible over API/UI/SEC, then a new accessibility Slice AX is added | Authorized membership change produces a new finite relation cut and target basis. Account for AX, its scope obligations and required actual closure. Old IM eligibility remains history; it cannot authorize closing the changed IM. New parent basis/result and affected coverage are required; through-commit checks reject a stale attempt. |
| U1 moves between branches or a matching scope assignment changes | Preserve relation history and re-resolve affected applicability, prerequisites, parent cuts and declared-currentness. Scope coverage follows authorized actual facts. Removing U1 from one parent cannot make unperformed required feature work disappear; incompatible or incomplete changed boundaries refuse. |
| A1 was already closed under C@1/G@1 | Preserve exact work binding, assessment truth, authorization, receipt and historical membership cut. C@2/G@2 do not silently relabel that closure or claim new requirements were satisfied then. A new current use of A1's closure by A2/IM resolves its own governance/coverage/currentness. New remediation or follow-up may be required without altering the original receipt. Reopening/retroactive migration is unselected. |

New work starts at the earliest unsatisfied boundary. A continuing-current fact
may need new verification; changed requirement content may need a new assessment
or observations; a supported current-qualification protocol may reuse an exact
earlier result for unchanged declared roles. Changed requirements source or intent
requires a new occurrence even when references match. There is no whole-suite
rerun promise, automatic old-result acceptance or general evidence-import route.
The approved [revision/currentness rules](../specs/handbook-contract-membrane/semantic-model/objects-and-governance.md#revision-currentness-and-invalidation)
control source dependencies; a new unrelated authored revision does not invalidate
every result automatically.

For F's final closure, Work Progression requires W's exact current PL/IM closed-state
propositions linked to their committed receipts, complete direct-membership and
declared prerequisite accounting, and F's own qualifying integration/security
assessment roles. It does not sum eligible descendants. F needs its own
`eligible / allow`, authorization, Attempt and guarded durable receipt. If AX is
added before IM or F commits, the affected currentness checks stop the stale action.
If F already closed, its original receipt remains exact history; any newly claimed
completion of a changed feature scope requires a separately supported future action,
not silent alteration of the old truth.

## Scenario and outcome comparison, including adapter loss

| Scenario | Faithful outcome in the proposed model | Shortcut correctly refused/prevented |
| --- | --- | --- |
| CA outputs intrinsically complete C@1 | Required output can resolve; applicable process, qualification, assessment and close action remain separate. | A file exists or validates, therefore planning is operationally closed. |
| Complete C@1 directly evaluated without any item/pack | Exact direct result answers its bounded question under positive applicability and independent authority. | Treating the result as project-governed delivery or requiring kind registration for independent use. |
| A2 consumes C@1 and S1's exact qualifying security result | Resolve compatible governing roles plus actual A1/S1 closure; then determine and separately commit A2 closure. | Contract reference, security receipt or gate pass alone closes A2. |
| Required security versus weaker Fast task@1 | Compatible local API-testing obligations remain; required security cannot disappear. A valid unfavorable review yields ineligible/block under the selected policy. | Nearest-parent wins, kind rename, omitted field or agent-authored waiver. |
| Raw-card retention versus prohibition | Dependent composition refuses with exact incompatible obligations. | Invented universal strictness order or arbitrary merge. |
| A1 → S1 added while S1 → A1 exists | Reject the changed cyclic prerequisite cut before affected operation. | General graph support or indefinite waiting presented as usable progression. |
| Contract/governance/member changes after eligibility | Preserve earlier truth; form new affected binding/basis/qualification/result and enforce through commit. | Silent rebind, stale allow, bulk replay or automatic prior-result acceptance. |
| Future tracker adapter exposes only checkbox `done` and archive | Inspect/display the observed source fact with disclosed loss. An outward coarse done projection can be explicitly lossy. Inbound governed close/synchronization refuses without exact required source/state/authority facts. | Mapping archive, cancellation and successful governed closure to the same native receipt; flattening S1's cross-branch prerequisite into UI containment. |

The tracker example is a hypothetical capability gap, not a claim about a named
product's current API. A future adapter cannot infer the missing contract revision,
assessment intent/coverage, policy basis, prerequisite closure or commit authority
from a Boolean. Installing a pack adds none of them under G6. Even if a family calls
S1 a checklist entry and A2 a phase, the native relations retain their meanings.
If the adapter cannot preserve them for the selected effect, refuse that effect
and show the exact unsupported/lost meanings. Disclosure permits an honest view;
it does not authorize a lossy mutation. No adapter or skill is implemented here.

## Human product decisions and deferred realization

These choices are ready for discussion now; no further generic study is proposed
as a prerequisite. Each recommended answer below is **new proposed selection**
within the already approved semantic constraints.

| Decision | Recommended answer | Alternative and tradeoff |
| --- | --- | --- |
| What composition and relation surface should the first native model support? | Optional immutable kinds plus explicit composition; single-parent same-project containment; direct same-project prerequisites across branches; cycle rejection over closure dependencies; separate informational/group/order facts. | Composition-only lowers definition lifecycle cost with equal expressive power. Multiple parents/external dependencies expand resolver, completeness and authority obligations; defer those capabilities rather than simulate them. |
| How do planning and implementation use contracts? | Explicit required-output, exact requirements-input and assessment-result roles with per-target coverage. Producing/validating a contract is distinct from governing assignment and operational closure. | Prose-only links are simpler to author but cannot supply executable completion truth. A fixed planning/implementation kind taxonomy reduces choices but unnecessarily fixes names/depth. |
| How do obligations apply through hierarchy? | Explicit authorized project/scope applicability; kind/item compatible references; explicitly selected descendant reach; supported cumulative obligations and refusal on unresolved/conflicting composition. Parent integration may be target-only. | Item-only policies demand repeated independent scope safeguards. Implicit inheritance/precedence or global strictness creates unapproved, potentially evasive meaning. |
| Which assessment protocol should checkout delivery use? | Prospectively governed production-delivery assessments for this scenario; permit separately explicit supported retrospective roles where project authority selects them under G5. No automatic acceptance or universal human contract review. | Prospective-only is easier to explain but requires more new assessments when governance changes. Retrospective acceptance saves valid work at the cost of exact coverage/currentness/population qualification; cannot manufacture required past review. |
| How should active and closed work evolve? | Supported explicit authorized active-item rebinding, independently current governance/membership checks and preserved closed history. Meaning-preserving adapter effects, disclosed lossy views and truthful refusal. | Freeze all active bindings until new work is created for simplicity; still enforce current governing obligations. Automatic migration/coarse sync would change truth and cannot bypass approved invariants. |

Deferred implementation choices: physical work-state/structure, definition,
applicability and action-owner placement; concrete lifecycle states/operations and
reopening; registry versus reuse of an equivalent whole-composition mechanism;
identity allocation, resolver and concurrency mechanisms; exact policy content and
supported compatibility/coverage mappings; SDK request/result compositions;
persistence/retention, schemas, formats and authoring/import surfaces; human views;
later CLI/installed skills and selected family adapters. Contracts JSON remains fixed;
the format standard's other defaults remain proposed. Native owner/SDK design needs
separately authorized bounded planning and does not sequence or change current
packet work.

## Verification and review boundary

Verification is a paper check of controlling-source agreement, local links and
scenario consistency. Sources were read from the shared working tree; initial HEAD
and branch are recorded above. No live code exploration, symbol edit or runtime
test is claimed. GitNexus MCP tools were not exposed in this session; no code-graph
claim depends on them and no index rebuild/repair was attempted. Previous studies'
code/index findings remain dated evidence rather than freshly verified capability.

The exact complete report is the review subject. One focused local collaboration
reviewer reads it and the relevant controlling sections, returns a decisive verdict
with P1 Critical/P2 Required blocking and P3 Optional/Consider/P4 Nit advisory
classification, and any material remediation is verified and reviewed as required
by the causal-review policy. The session result carries the actual checks and review
disposition; this paragraph alone asserts no approval. No existing authority or
research document, code, staging, commit, branch, roadmap or durable orchestration
record is changed by this experiment.
