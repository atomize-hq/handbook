# HR-005 / P2-03 — Proposed lifecycle when every observation is legitimately omitted

Status: local proposal for decision review. The user agreed with the direction
of supporting this bounded case and requested formulation of the missing lifecycle
semantics. The specific choices below are recommendations, not approved normative
amendments. A [focused Pro consultation](hcm-zero-observation-pro-consultation-2026-09-29.md)
was completed and retrieved on 2026-09-29. Pro recommends retaining the narrow route
and clarifying how closing checkpoint verifications bind the final prospective basis.
The proposed body below remains the submitted design, not a silently revised or
accepted amendment. HR-005 remains open; HR-006 is not resolved by this draft.

## Problem and scope

The current model permits Contracts to prove an expected claim prospectively
inapplicable and omit its exclusively required observations. But Observation Plan
requires one or more Producer Selections, commitment depends on durable collection
provenance, and the collection frontier presupposes that commitment. If every
required observation is legitimately omitted, the path is incomplete.

Support this boundary case without adding reasons to omit observations, a new
evaluation engine, fake producer activity, or automatic success. Frequency is not
established: this is lifecycle consistency for a permitted boundary case, not a
recommended everyday optimization.

The user's correction to the migration example is controlling context: absence of
declared migration work does not prove that a check for unexpected or malicious
database-changing behavior is unnecessary. An omission proof must entail the exact
claim's non-applicability under its admitted policy. The author saying a change is
out of scope, or an inventory that only covers declared files, cannot establish a
broader absence proposition. Requirement selection and subsequent applicability
remain separate decisions.

## Recommended entry boundary

The first-version route proposed here is **complete prospective pruning**, with:

- a complete, nonempty expected-claim inventory and its original observation-role
  relationships fixed by one exact Evaluation Requirements closure;
- a nonempty original observation obligation population, all removed by the
  existing positive-proof whole-claim applicability rules;
- every expected claim positively proved `prospectively_not_applicable`, with the
  complete exact supporting fact closure and compatible final applicability rules;
- no remaining observation obligation, shared role, or composition requirement;
- an exact admitted evidence-set Formation Policy/configuration and precommitted
  lifecycle/time rules that support closing this declared empty collection.

Requiring every expected claim to be positively pruned is a proposed conservative
scope choice, not a claim that the user already selected this exact restriction.
It avoids accidentally defining initially empty contracts, inherently observation-
free evaluations, or mixtures with retained observation-free claims. Those cases
are not enabled by HR-005 and need separate semantics if later required. Review
should confirm this boundary is appropriate for the accepted all-pruned case.

Missing, stale, ambiguous, unsupported, or conflicting applicability proof cannot
establish entry. The existing rules retain the affected claim and its observations;
the ordinary nonempty route can still be considered if otherwise valid. If a
caller nevertheless requests an empty plan, its formation refuses. Unavailable
tools, insufficient capability, cost, author intent, and lack of collected data
are never entry justifications. This is different from an ordinary nonempty plan
whose attempts produced no admitted evidence.

## Proposed end-to-end semantics

### 1. Close a real plan with zero Producer Selections

Evaluation Coordination consumes the exact Contracts-owned Prospective
Applicability Basis and verifies complete derivation of zero retained observation
roles. It forms an Observation Plan with an explicitly empty selection population,
retaining the complete original expected-claim inventory, original role relations,
every omission and its reason, and the exact applicability basis. Empty is an
established result of accounting, not an absent field or an unimplemented plan.

This is a bounded exception to the current positive selection cardinality. Any
plan with retained observation requirements still needs compatible selections for
all of them. No dummy producer, empty test runner, or synthetic source is created.

### 2. Form Purpose and obtain real Engine authorization

The normal Purpose and occurrence formation remains, including requirement-source
identity, contract permitted use, actual run intent, exact plan, Formation Policy,
and truth-affecting time/boundary rules. The no-collection path is fixed before
results are available and participates in that exact content through the plan and
precommitted lifecycle rule. It is not a late conversion of a failed normal run.

The complete requested-reach closure is explicitly an empty collection of
selection-bound requests. It is not a fabricated bottom coordinate, zero-valued
Resolution vector, wildcard permission, missing envelope, or producer request.
The exact project, subject/scope, objective, use, Resolved Profile/Resolution stack,
admitted Engine envelope and opaque authorization lineage remain required.

Engine admits or refuses the exact zero-collection Purpose under its existing
authority responsibilities. Vacuous request containment does not establish the
rest of authorization. Its Binding cannot authorize a later nonempty plan; adding
a selection changes Purpose content and requires a new occurrence and binding.
No collection is authorized through this route.

### 3. Establish readiness without invented producer facts

Use the existing Readiness Basis and `ready` assessment, with an explicit positive
justification: the exact omission proof and plan accounting establish that no
collection work remains, all required non-producer formation inputs are valid,
and Engine has positively verified the Binding for this checkpoint.

Producer capability, Dock ceiling, host-launch prerequisites, retry and partial-
completion facts are selection-dependent; there are no such selections to bind.
Do not fabricate favorable facts for imaginary producers. Every actually required
plan-independent owner input and verification remains required. The explanation
must say readiness is for the no-collection path, not tool readiness or test success.

Failure to establish the empty-plan justification is formation refusal, not
`known_limited` or `unknown` with permission to continue diagnostically. Existing
contract-use compatibility still precedes readiness. With valid formation, either
permitted intent can use the `ready` / `attemptable` path to form the Prospective
Evaluation Basis. Here `attemptable` permits advancement of this lifecycle, not
launch of a Producer Attempt. No new readiness state is proposed.

### 4. Commit the basis and seal collection through Evaluation Coordination

Add a narrowly defined no-collection transition owned by Evaluation Coordination.
Its successful owner outcome binds the exact occurrence, Purpose, empty Plan,
Prospective Evaluation Basis, applicability proof/accounting, Formation Policy,
precommitted checkpoint/time rule, and required owner verifications.

Immediately before this transition, Engine must positively verify current
authorization for the no-collection commitment checkpoint, replacing the role of
the pre-launch checkpoint where there is no launch. Other dependencies must satisfy
their existing declared verification rules. This does not introduce universal
"latest" lookup or settle HR-006's later retry-verification question.

The transition positively establishes that this occurrence has no admitted
collection activity and cannot admit any: zero selections, attempts, launches,
observations, submissions, and collection terminals. It then commits the occurrence
to this one basis and seals its Occurrence Collection Frontier as **collection
closed without collection activity**. Commitment and frontier closure are separately
attributable propositions in one successful owner outcome; no externally usable
partial success is allowed. No fabricated Producer Attempt, Host Enforcement Fact,
Dock terminal, or source no-attempt terminal is created.

This is an additional commitment condition alongside the existing real-collection
condition. It may be represented in the existing frontier/lifecycle outcome; it
does not mandate another service, persistent record family, or distributed
transaction. The no-collection outcome grants no observation/evidence truth.

Exact replay returns the same committed basis/frontier identities. Conflicting
reuse refuses. Any attempted producer activity under the empty plan is refused;
proven conflicting activity already attributed to this occurrence prevents valid
closure and is not erased to claim zero activity. Later discovery of a violated
closure follows the existing integrity boundary rather than rewriting history.
Unverifiable or uncertain transition outcome is not successful commitment or an
assumed empty frontier; resolve the owner's outcome before progressing.

### 5. Form an empty, fully accounted evidence set

Contracts evidence-set formation consumes the committed basis and sealed empty
frontier under the exact precommitted Formation Policy. The eligible boundary is
explicit and occurrence-bound, with zero permitted collection/admission members;
owner/repository resolution requirements remain those declared by the policy, not
an ambient "nothing found" query. It positively verifies the empty population and
complete plan/pruning accounting, then forms the ordinary Canonical Evidence Set
Closure with empty membership and zero in-frontier admissions awaiting terminals.

No evidence-admission record is fabricated. Prior facts supporting applicability
remain exact separately owned inputs in their admitted roles; they are not copied
into the empty evidence set. Unexpected in-boundary activity or open accounting
refuses closure; missing population knowledge is not an empty population.

### 6. Apply the normal final-context and applicability checks

Final context still requires its exact evidence closure, Purpose and requirements,
committed basis/frontier, readiness and use provenance, required policy/definition
closures, and positive Engine final-closure verification. It retains the exact
pre-existing applicability facts and their admitted role/temporal/currentness rules.
Those rules must establish the exclusions at final evaluation; this does not
silently substitute newer facts or require undeclared live collection.

Each prospectively excluded claim must still establish final positive
`not_applicable`. If any becomes applicable or cannot be proved inapplicable under
the exact final semantics, refuse the final truth chain. Do not add observations
to the sealed occurrence, silently revise the basis, or publish ordinary claim
indeterminacy for a requirement whose observations were omitted. A changed subject,
requirements, proof facts, or plan requires a new occurrence under the existing
identity rules. Existing historical records remain unchanged.

Checking owner provenance and declared currentness is still work; "no collection"
means no producer observations in this evaluation, not no processing or validation.
If the proof actually requires a new producer observation, the empty path cannot
supply it and its prerequisites have not been established.

### 7. Preserve result meaning and the completion boundary

Every expected claim remains accounted with final applicability. Metrics and gates
apply their already admitted meanings and empty/not-applicable population rules;
zero collection itself supplies no satisfied finding, fabricated metric value,
or automatic `pass`. This proposal does not define a new universal gate algorithm.
The existing all-not-applicable scenario still yields metric/gate `not_applicable`
and the stated reliance `not_applicable / no_effect` under its specified policies.

The completion owner still applies HR-004's eligible-contract, posture-selected,
decision-seeking, actual governing coverage and currentness requirements, plus
its exact outcome policy. No-collection status grants no completion exemption,
positive eligibility, action authority, or mutation receipt. Direct/diagnostic
provenance cannot be promoted. An empty evaluation must never be displayed merely
as "tests passed"; explanation retains all excluded claims and decisive proofs.

## Proposed wording core for subsequent review

An Observation Plan MAY have zero Producer Selections only through the explicitly
supported complete-pruning path, whose exact prospective applicability closure
positively establishes and accounts for every omitted obligation. Empty or missing
inputs, capability limits, unavailable tools, or author intent do not establish
that path. The complete contract requirements and expected-claim inventory remain.

For that path, Engine authorization MUST bind the exact Purpose and explicit empty
requested-collection closure. Evaluation Coordination MUST establish readiness
from complete omission proof and valid required owner inputs, without inventing
producer facts. A successful Coordination-owned no-collection transition MUST
commit the occurrence to its one prospective basis and seal the empty collection
frontier before evidence-set formation. It MUST NOT fabricate attempt or source
provenance. Replay preserves the same outcome; conflicting or uncertain closure
cannot authorize progression.

Contracts MUST close the empty evidence population with complete accounting and
perform the unchanged final authorization, applicability-congruence, metric, gate,
and explanation checks. Failure of any required exclusion proof prevents new
semantic truth; it cannot trigger collection under the sealed occurrence. Empty
collection is neither automatic pass nor permission for governed completion.

## Discriminating cases to review

| Case | Proposed outcome |
| --- | --- |
| Exact independent facts and admitted rules prove all claims inapplicable and remove all original observation roles. | Valid zero-selection Plan; authorized no-collection lifecycle may reach exact final applicability and policy-defined truth. |
| One claim remains unresolved or a shared observation role is still required. | No empty-plan eligibility; retain the affected requirements and use the ordinary route if otherwise valid. |
| Work description says no migrations; selected security claim checks for undeclared database-changing behavior. | Description alone cannot prove that claim inapplicable. Its observations remain required. |
| No tests were selected because tools are absent, or all ordinary attempts failed. | Not the no-collection route. Preserve ordinary requirements, refusal/readiness/attempt provenance and accounting. |
| Originally empty contract or an inherently observation-free claim remains. | Not authorized by this narrow complete-pruning proposal; do not infer generalized zero-observation support. |
| Exact empty-plan authorization or required current verification is missing. | Refuse the affected stage; empty reach supplies no authority. |
| Diagnostic-only contract requests decision-seeking intent with an otherwise valid empty plan. | Refuse before valid Purpose under HR-004; empty collection changes no permitted use. |
| A caller adds a producer or tries to reuse old execution evidence. | Changed plan needs a new occurrence; evidence import remains unsupported. Empty frontier membership cannot be expanded. |
| No-collection transition is replayed, conflicts, or returns an uncertain outcome. | Exact replay returns the same result; conflicts refuse; uncertainty requires owner resolution before progression. |
| Final verification cannot support an earlier exclusion. | No final applicability/claim/metric/gate truth; preserve history and use a new valid occurrence if requirements or proof must change. |
| All final claims are not applicable under the existing example's policies. | Metric/gate not_applicable and the specified no_effect reliance; no vacuous pass or automatic completion. |

## Expected amendment surface and review questions

- Preparation: zero-selection Plan exception; empty request closure versus Resolution
  coordinates; pre-launch/no-collection checkpoint terminology; positive readiness
  basis; meaning of attemptability without a producer; scenario assumptions.
- Evidence/evaluation: new commitment condition; frontier formation without producer
  terminals; empty boundary/member accounting; final applicability and identity;
  historical resolvability, time/replay, examples, and settled/deferred summaries.
- Objects/governance: attribution/classification of the no-collection lifecycle
  outcome using existing semantic owners; no automatic new artifact or runtime type.
- Reliance/actions and guide: exact non-applicability explanation and no completion
  exemption; conditional presentation of the lifecycle; HR-005 review disposition.

Review should specifically test: whether the conservative all-claims-pruned entry
restriction is appropriate; whether existing Engine semantics can express an
explicitly empty request population without fictional reach; whether one owner
outcome can coherently establish commitment and frontier with exact replay; and
whether final exclusion proofs remain available in their existing admitted roles
without importing them as evidence. These are semantic checks before implementation
choices, not reasons to add a new platform or resolve HR-006 prematurely.

Next: review and accept the proposed scope/lifecycle choices and Pro's checkpoint
refinement, amend the owning documents, and audit incorporation. The complete advice
is retained in the linked consultation record; no acceptance or incorporation is claimed.

## Subsequent decision — 2026-09-29

The user accepted the bounded complete-pruning route and Pro's closing-checkpoint
refinement after the plain-English example, then requested a narrow amendment of
the four documents and decision record followed by a thorough incorporation audit.
That amendment is now incorporated; the audit is pending. The earlier proposed or
pending-acceptance language above is retained as historical consultation context,
not a competing current disposition. Accepted scope is recorded in
[HR-005](../specs/handbook-contract-membrane/semantic-model/review-decision-register.md#hr-005--what-happens-when-prospective-pruning-removes-every-observation).
HR-005 is not yet audit-closed; HR-006, whole-package approval, and implementation
authorization remain open. No concrete schemas, services, storage, or IAM platform
are selected by this decision.

## Completed incorporation audit — 2026-09-29

The accepted amendment's focused Pro audit completed with no qualifying P1/P2
findings, no remaining accepted root cause, and no material delta regression.
HR-005/P2-03 is resolved at semantic level; implementation realization remains
deferred. Earlier audit-pending statements above are historical. The complete
response and reviewed snapshot identity are in the
[incorporation audit record](hcm-zero-observation-amendment-pro-audit-2026-09-29.md).
One optional collection-only replay sentence was normalized locally to include
complete-pruning commitment/frontier and evidence closure. HR-006, other package
follow-ups, whole-package approval, and implementation authorization remain open.