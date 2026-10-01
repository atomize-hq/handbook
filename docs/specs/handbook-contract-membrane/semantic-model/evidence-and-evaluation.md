# HCM semantic model — Evidence and evaluation truth

**Status: PROPOSED**

Part of the [combined semantic package](README.md). Its approval boundary and
shared greenfield/current-product distinction apply throughout.

This part follows one exact occurrence through prospective applicability, collection frontier, evidence closure, claims, metrics, gate truth, and historical replay. Formation and evaluation remain separately owned subresponsibilities.

## Reading map

- [Evaluation identity and evidence closure](#identity-evidence)
- [Claims, metrics, gates, and explanation](#metric-gate)

<a id="identity-evidence"></a>
## Evaluation identity and evidence closure

This chapter owns occurrence identity, stable selection versus checkpoint-current facts, prospective applicability, evidence-set formation and closure, semantic time, and historical resolvability. These refinements do not transfer another chapter's semantic authority.

### Purpose and scope

The exact lifecycle is:

```text
Evaluation Requirements (`posture_selected` or `direct_contract`)
  -> Prospective Applicability Basis, when any expected claim may be prospectively not applicable
  -> Observation Plan with exact Producer Selections
  -> Evaluation Purpose containing Evaluation Occurrence Identity and use intent
  -> Evaluation Authorization Request
  -> Engine Evaluation Authorization Binding
  -> Readiness Basis and Readiness Assessment
  -> Attempt Disposition
  -> Prospective Evaluation Basis, when attemptable
  -> ordinary Producer Attempts/source/submission activity and collection commitment
       OR complete-pruning Coordination no-collection commitment
  -> sealed Occurrence Collection Frontier (established with commitment for complete pruning)
  -> terminal completion of every Contracts admission operation causally attributable to an in-frontier submission
  -> Canonical Evidence Set Formation and Closure
  -> final Contracts evaluation-context closure
  -> final applicability, claims, metrics, semantic gate, and canonical explanation
  -> action-specific Operational Reliance Basis and atomic Determination, if used now
  -> separate Action Intent / authorization / attempt / terminal / receipt, if an effect is requested
```

Allocation of an Evaluation Occurrence Identity is part of Evaluation Purpose
formation, not a separate lifecycle stage. Evidence admission may occur more
than once before or after the collection frontier, but every admission causally
descending from an in-frontier candidate MUST be terminal before set formation.
Final evaluation-context formation may occur only after the one exact set
closure it consumes.

This record settles:

1. how exact replay differs from a deliberately new evaluation;
2. which Producer Selection changes reform the plan and purpose, and which
   current-fact changes reform readiness only;
3. when positive pre-existing applicability facts may establish an expected
   claim prospectively not applicable and thereby omit its observation
   relations before the plan closes;
4. how Contracts closes one complete, non-ambient evidence set before any
   claim evaluator selects from it; and
5. which semantic times and historical dependencies must remain resolvable.

The proposed
[authored-artifact and evaluation-relationship decision](objects-and-governance.md#authored-relationships)
owns detailed authored fields and relationship roles beyond the exact subject,
scope, and snapshot references required by these closures. The proposed
[operational-reliance and work-progression decision](operational-reliance-and-actions.md#reliance-actions)
owns present-use reliance, atomic eligibility/disposition, local closeout,
parent eligibility, action authorization, attempts, terminals, receipts, and
mutation boundaries. Those proposal links do not approve either record or
select physical placement, policies, hierarchy/state models, or implementation.

### Canonical vocabulary

Terms in separate rows are not synonyms.

| Term | Fixed meaning |
| --- | --- |
| **Evaluation Occurrence Identity** | The Contracts Evaluation Coordination-owned identity that distinguishes one deliberately intended evaluation occurrence from every other occurrence, even when their semantic inputs are otherwise equal. It is allocated as part of Evaluation Purpose formation, remains bound through every later object in that occurrence, and is neither content-derived nor a transport correlation. |
| **Evaluation Content Fingerprint** | A deterministic fingerprint over the exact semantic formation content of an Evaluation Purpose, excluding Evaluation Occurrence Identity, the Purpose's own owner-issued record identity, formation/delivery metadata, and transport correlation. Referenced semantic-owner identities remain part of the content. Equal fingerprints mean equal declared evaluation content, not the same occurrence, authorization, attempt, evidence set, or result. |
| **Exact formation replay** | Re-presentation or reconstruction of the same owner formation request under the same Evaluation Occurrence Identity and exact semantic content. It must return the same owner record or exact idempotent outcome; it does not create a new evaluation. |
| **Delivery replay** | Re-delivery of the same exact message or bytes under interface-local correlation. It may be deduplicated at that boundary but has no power to create, merge, or distinguish semantic occurrences. |
| **Deliberately new evaluation** | A new Evaluation Occurrence Identity and therefore a new Evaluation Purpose, request, Engine authorization issuance path, readiness closure, Prospective Evaluation Basis, evidence-set closure, and final context, even when its Evaluation Content Fingerprint equals an earlier one. |
| **Stable Producer Selection closure** | The immutable semantic portion of a Producer Selection: exact assigned observation-role relation, producer kind and configuration, Dock or adapter identity when applicable, Requested Observation Reach, exact admitted Producer Semantic Capability definition reference, exact operating-ceiling definition reference, declared host-prerequisite definitions, exact retry-policy and partial-completion-policy definition references when either behavior is permitted, and plan multiplicity/composition relation. It contains no checkpoint-current availability or prerequisite-satisfaction observation. |
| **Checkpoint-current producer fact** | An owner-produced observation at a named Readiness or launch semantic time about the currentness or applicability of a selected capability, ceiling, retry, or partial-completion definition; current producer availability; host-prerequisite satisfaction; or another declared operational condition. It cannot change the stable selection it verifies. |
| **Prospective Applicability Policy** | An immutable, named, versioned Contracts definition that can establish, solely from an exact closed set of pre-existing canonical facts, that one expected claim is positively not applicable to the exact subject/snapshot and scope before that claim's observation obligations are planned. It neither establishes the final Applicability Result nor prunes only part of an otherwise applicable claim. |
| **Eligible prospective fact boundary** | The exact finite, non-ambient semantic-owner, repository or resolver, fact-kind, proposition-scope, subject/snapshot, applicability-scope, causal/semantic-time-cut, currentness, reuse-restriction, and deterministic inclusion/exclusion boundary from which prospective applicability is decided. |
| **Prospective Applicability Basis** | The Contracts-owned immutable closure over one Evaluation Requirements identity, subject/snapshot and scope, one Prospective Applicability Policy, one eligible prospective fact boundary, applicability semantic time, exactly one prospective result for every expected claim, and complete derived observation-relation and fact accounting. |
| **Prospective prune** | Positive policy proof that one exact expected claim is not applicable to the exact subject/snapshot and scope. Every observation-role relation required solely for that claim is omitted for that claim. A shared role remains planned whenever another retained claim requires it. Missing, unknown, ambiguous, conflicting, stale, unsupported, or evaluation-produced information retains the claim and every relation required to evaluate it. |
| **Occurrence Collection Frontier** | The immutable Evaluation Coordination-owned terminal record for one Evaluation Occurrence Identity and one committed Prospective Evaluation Basis. It binds the exact owner-produced launch, refusal, attempt, source, submission, and terminal provenance required by every Producer Selection and plan relation; the exact selection-bound retry and partial-completion policies; and one causally closed frontier after which no new valid attempt, source observation, or candidate/submission occurrence may be created for that occurrence. For the complete-pruning route, one Coordination-owned outcome establishes both commitment and the empty frontier without origin terminals. It establishes no candidate admission, evidence membership, applicability, claim, metric, gate, or operational truth. |
| **Eligible evidence repository boundary** | The exact owner-repository identities, committed Prospective Evaluation Basis, subject/snapshot, scope, admitted evidence kinds, sealed Occurrence Collection Frontier, terminal admission-result population causally attributable to candidate/submission occurrences inside that frontier, and precommitted semantic-time rule from which Canonical Evidence Set Formation must account. It is a semantic membership boundary, not a storage technology or ambient query. |
| **Canonical Evidence Set Formation Policy** | An immutable, named, versioned Contracts definition that fixes the eligible repository boundary, closure preconditions, membership rule, per-selection and plan-relation accounting, retry and duplicate treatment, and set semantics. It cannot select evidence according to a desired claim or gate outcome. |
| **Canonical Evidence Set Formation** | The Contracts-owned deterministic operation that accounts for every required plan relation, attempt, candidate/submission occurrence, admission result, and canonical evidence record in the exact eligible boundary before producing either one closure or a formation refusal. |
| **Canonical Evidence Set Closure** | The immutable Contracts-produced record that binds the exact formation policy, eligible boundary, Occurrence Collection Frontier, unordered canonical evidence membership, complete formation-member/nonmember and terminal accounting, closure semantic time, and closure identity. It establishes no applicability, claim, metric, or gate truth. |
| **Final evaluation context** | The Contracts-owned closure formed only after the Canonical Evidence Set Closure exists. It binds that closure, the exact Prospective Evaluation Basis and occurrence, all required definitions and policies, the final-closure Engine verification, and the exact evaluation semantic time before truth evaluation begins. |
| **Semantic time** | A named owner-attributed temporal coordinate or causal checkpoint used by a semantic rule. It is part of provenance and currentness evaluation; it is not assumed to be a globally comparable wall-clock timestamp. |
| **Historical resolvability** | The ability to resolve an exact historical identity to its owner-recognized semantic content, owner provenance, applicable semantic-time relation, and required verification outcome for explanation or deterministic replay. It does not itself make the object current, grant present authority, or decide eligibility for an action. |
| **Exact historical replay** | Deterministic reconstruction of the same owner results from the same occurrence, exact historical inputs, definitions, policies, membership closures, semantic times, and owner verification facts. It performs no new observation and grants no present authority. |

Evaluation Occurrence Identity, Evaluation Content Fingerprint, Evaluation
Purpose identity, Evaluation Authorization Request identity, Engine binding
identity, Prospective Evaluation Basis identity, Producer Attempt identity,
Canonical Evidence Set Closure identity, and final evaluation-context identity
are distinct. No one of them may substitute for another merely because their
objects transitively bind common inputs.

The common [Evaluation Requirements](assurance-and-evaluation-preparation.md#evaluation-requirements)
role supplies complete proof obligations for both sources. References to that
role below require its exact owner closure; posture-specific Resolved Assurance
Requirements and their lineage remain mandatory for `posture_selected` only.
Contract permitted use is resolved from the admitted revision, not from a
caller label. Formation must already have refused incompatible run intent.

### Evaluation occurrence and purpose identity

Evaluation Coordination MUST allocate one Evaluation Occurrence Identity only
as part of forming an exact Evaluation Purpose. The purpose identity MUST bind:

- the Evaluation Occurrence Identity;
- the Evaluation Content Fingerprint;
- exact subject/snapshot and scope;
- objective and Evaluation Use Intent;
- Evaluation Requirements, exact requirement-selection source, and contract permitted use;
- the Prospective Applicability Basis when one influenced plan formation;
- the complete Observation Plan and Producer Selection identities; and
- the exact Canonical Evidence Set Formation Policy identity and
  configuration;
- every exact precommitted semantic-time or boundary-selection rule that can
  affect collection closure, evidence membership, evidence currentness, final
  applicability, claim truth, metric truth, or gate truth; and
- purpose-formation semantic time and owner provenance.

The content fingerprint MUST exclude the occurrence identity so two deliberate
evaluations with equal semantic content remain recognizable as equal-content
but different occurrences. The complete purpose identity MUST include both, so
those occurrences can never share authorization, attempts, evidence, closure,
or truth by implication.

The occurrence identity is semantic and owner-produced. It MUST NOT be
inferred from content, timestamps, a process identifier, an idempotency key, a
CLI or Tauri correlation, a Dock invocation, a random transport nonce, file
location, or evidence membership. Concrete allocation mechanics remain
deferred.

For one Evaluation Occurrence Identity:

- the exact same semantic formation content is exact formation replay and MUST
  return the same Evaluation Purpose identity and content;
- different semantic formation content MUST refuse as conflicting reuse;
- an exact delivery replay cannot allocate another occurrence or purpose; and
- a caller requesting a genuinely new evaluation MUST request deliberately new
  formation, and Evaluation Coordination MUST allocate the new occurrence
  identity even when every semantic-content input is equal.

Changing subject/snapshot, scope, objective, use intent, requirements,
Prospective Applicability Basis, Observation Plan, Producer Selection, role
assignment, composition/multiplicity, or Requested Observation Reach changes
purpose content and therefore requires a new occurrence identity, purpose,
authorization request and binding, readiness closure, and prospective basis.
The Formation Policy and every truth-affecting semantic-time or boundary rule
above participate in purpose semantic content and the Evaluation Content
Fingerprint. Changing any one likewise requires the complete new occurrence
chain.

Before occurrence commitment, Evaluation Coordination MAY form a fresh
Readiness Basis, Readiness Assessment or Refusal, Attempt Disposition, and
Prospective Evaluation Basis under the same occurrence only when all of the
following remain exact: subject/snapshot, scope, objective, Evaluation Use
Intent, Evaluation Requirements with the same source and contract permitted use,
any Prospective Applicability Basis
and prune-supporting fact closure, Observation Plan, every Producer Selection,
every stable producer, Dock, prerequisite, retry, and partial-completion
definition, Evaluation Purpose, Evaluation Authorization Binding, and every
precommitted formation or semantic-time rule. Every dependency requiring
current-use verification at the refreshed pre-launch or no-collection pre-commit
checkpoint MUST have a
positive exact owner verification, including Engine verification of the same
binding. Only checkpoint-current facts that change no stable proposition may
differ.

Every earlier Prospective Evaluation Basis remains immutable and MUST be
recorded as unused and ineligible for launch or no-collection commitment. It is
not overwritten or relabeled as the later basis.

An occurrence commits to one Prospective Evaluation Basis when any semantic
owner closes durable launch, launch-refusal, Host Enforcement Fact, Producer
Attempt, Dock invocation, source-observation, candidate/submission, or other
collection-effect provenance under that basis. Alternatively, the supported
[no-collection outcome](#no-collection-commitment) establishes commitment and the
sealed frontier together without prior collection. After commitment, the occurrence
cannot adopt another Prospective Evaluation Basis. A change requiring another
basis terminates or abandons that occurrence without new truth; later
evaluation requires a deliberately new occurrence and its applicable collection
or complete-pruning path. Later launches and permitted retries use the
[post-commit verification rule](#post-commit-verification): the committed basis
remains exact, and each governed boundary requires the applicable fresh owner
checks. A later check does not replace a formation-time observation.

### Stable selection and checkpoint-current facts

Producer Selection MUST bind stable semantic definitions and assignments, not
the transient result of checking them. Its stable closure includes exactly the
role relation, producer/configuration, applicable Dock or adapter, requested
reach, admitted capability-definition reference, operating-ceiling-definition
reference, declared host-prerequisite definitions, exact retry-policy and
partial-completion-policy definition references when applicable, and declared
composition or multiplicity relation.

The Readiness Basis separately binds, at its named semantic time:

- currentness/applicability verification for each exact capability definition;
- currentness/applicability verification for each exact operating-ceiling
  definition;
- currentness/applicability verification for every exact retry-policy and
  partial-completion-policy definition required by the selection;
- producer availability and any exact selected endpoint or instance facts the
  stable selection permits;
- actual host-prerequisite satisfaction and supported enforcement facts needed
  prospectively for launch;
- Engine authorization verification; and
- the owner identity, semantic time, and exact content of every verification or
  observation used.

A checkpoint-current fact may verify, limit, leave unknown, or refuse readiness
for the stable selection. It MUST NOT silently substitute a producer,
configuration, role, adapter, reach, capability definition, ceiling definition,
prerequisite definition, or plan relation.

An endpoint or producer instance may remain a checkpoint-current fact only
when the stable selection explicitly permits that bounded choice and its exact
identity does not change semantic capability, source authority, configuration,
role compatibility, or requested reach. If any of those propositions changes,
the producer/configuration changes and the stable selection MUST reform.

The reformation rule is closed:

| Change | Earliest closure that MUST change | What need not change when still exact and current |
| --- | --- | --- |
| Role assignment, producer kind/configuration, Dock/adapter identity, requested reach, capability, ceiling, prerequisite, retry, or partial-completion semantic definition, or multiplicity/composition relation | Producer Selection, Observation Plan, Evaluation Purpose with a new occurrence identity, request/binding, readiness, and Prospective Evaluation Basis | Nothing downstream may be reused merely because content looks compatible. |
| New formation-time currentness verification over the same exact capability, ceiling, retry, or partial-completion definition before commitment | Readiness Basis, Assessment, Attempt Disposition, and any later Prospective Evaluation Basis | Observation Plan, Purpose, occurrence identity, and binding need not reform only under the complete unchanged and positively current pre-commit rule above. |
| Formation-time availability or prerequisite-satisfaction change within the unchanged stable selection before commitment | Readiness Basis, Assessment, Attempt Disposition, and any later Prospective Evaluation Basis | The stable Selection, Plan, Purpose, occurrence, and binding need not reform only under that same pre-commit rule. |
| A verification establishes that the selected definition is no longer applicable and a successor definition must be selected | Producer Selection and the complete downstream chain, with a new occurrence identity | An historical reference may remain resolvable but cannot be swapped in place. |
| Later checkpoint verification after commitment confirms unchanged fixed inputs or an originally permitted bounded operational variation | A new exact owner observation and Coordination launch/refusal outcome; no replacement Readiness or Prospective Evaluation Basis | The committed chain remains immutable only under the complete post-commit rule below. |
| A later check fails, is unavailable, or cannot establish a required condition | Refuse the affected launch and retain the exact result and reason; recovery is governed below | Failure alone is neither a substituted fixed input nor permission to resume. |
| A change after commitment requires a replacement formation input or falls outside declared operational variation | The existing occurrence retains its exact history but cannot form new truth from a replacement basis; subsequent evaluation requires a deliberately new occurrence | No retry or later verification may substitute that input into the committed chain. |
| Transport, delivery correlation, rendering, or projection changes | No semantic closure | Interface-local delivery behavior only. |

<a id="post-commit-verification"></a>
### Later launch verification under a committed basis

After commitment, the original Readiness Basis, Assessment, Attempt Disposition,
and Prospective Evaluation Basis remain immutable formation-time records. Their
checkpoint observations establish what was verified then; they are not timeless
assertions of availability or authority. While collection remains open, a later
producer launch or permitted retry MUST have its own applicable owner checks
under the following closed rule:

1. **Declare the boundary before execution.** The exact bound definitions,
   selection, and checkpoint rules MUST distinguish fixed semantic inputs from
   bounded operational observations that may vary, and name their verifying
   owners, governed checkpoints, freshness/comparison rules, and any continuity
   or recovery conditions. Missing or ambiguous rules cannot authorize variation
   or recovery. An agent's judgment that a difference is minor is insufficient.
2. **Preserve fixed inputs.** Subject/snapshot, scope, objective, use intent,
   requirements and source, contract permitted use, applicability/prune-supporting
   facts, Plan and stable Selections, Purpose, exact Engine Binding, and all bound
   semantic definitions and time/boundary rules remain exact. A new check identity
   or semantic time alone does not replace these inputs. A replacement Binding,
   definition, pruning proof, or other fixed input follows the earliest affected
   reformation boundary; if another prospective basis is required after commitment,
   terminate or abandon this occurrence without new truth and start a new one.
3. **Bound operational variation.** Availability, prerequisite satisfaction, and
   endpoint/instance observations may differ only within the original declared
   selection and lifecycle rules, without changing their semantic requirements.
   A different worker is allowed only if bounded instance choice was already
   permitted and capability, source authority, configuration, role, and reach
   remain as required. A selection pinned to one worker cannot use this exception.
   A contradicting observation about a fixed input is not operational variation.
4. **Gate each launch separately.** Engine verifies the same exact Binding;
   each other responsible owner establishes its applicable conditions. Coordination
   exact-checks those results against the committed basis, selection, and governed
   launch/retry, including applicable plan-wide dependencies. Earlier permission
   for selection A does not authorize selection B. The original use-intent and
   readiness requirements still govern; a decision-seeking launch cannot continue
   with newly limited or unknown capability by relabeling it diagnostic or reducing
   the plan. Prospective host prerequisites do not replace an actual attempt-bound
   Host Enforcement Fact where enforcement is required.
5. **Refuse and account for failures.** An unavailable or unverifiable check,
   a negative currentness result, and a contradictory fixed-input fact have
   distinct owner outcomes and reasons. Each blocks its affected boundary and
   remains in history. A further verification or resumption is permitted only by
   the original lifecycle/retry/currentness rules, while collection is open and
   every required condition can be freshly established. A later positive check
   does not erase an intervening revocation, supersession, or failed continuity
   condition, or retroactively authorize activity during a failed boundary.
   If the declared rules require uninterrupted validity or make failure terminal,
   two positive endpoint checks cannot establish permission to resume. No new
   recovery policy may be selected after failure to keep the occurrence alive.
6. **Preserve finite closure.** A verification retry is not a Producer Attempt
   and cannot reset or extend producer retry allowances. A real new execution has
   its own attempt identity. Exhausted allowances and terminal occurrence/frontier
   states cannot be bypassed by rechecking. A sealed frontier never reopens.

Each later check retains its exact owner, definition/input references, semantic
checkpoint, result, and reason. Coordination binds those exact checks to the
committed basis, selection, and launch/retry request they govern, and records the
resulting permission, hold, or refusal. This consumer-side binding does not make
Engine inspect Contracts definitions or create another owner's verification.
Any resulting attempt
references that earlier outcome; a refusal before attempt creation does not
invent an attempt or producer terminal. Later checks need no forward reference
to a future frontier and do not rewrite the original basis. Check results,
refusals, permitted instance choices, and recovery/continuity accounting all
enter the occurrence's frontier provenance, including failures before a later
successful check. Omission cannot be repaired by presenting only the success.

These are separately attributable facts under existing owners, not a mandated
new service, artifact family, or storage format. Verification provenance is not
automatically subject evidence or an evidence-set member. Complete recording
does not make unauthorized activity valid. Final Contracts closure retains its
separate strict checks. The complete-pruning route commits and seals together;
it cannot use this continuation rule to start collection afterward.

Availability is not capability. Capability-definition currentness is not
operating-ceiling currentness. A prerequisite definition is not proof that the
prerequisite is satisfied. None is Engine authorization or a Host Enforcement
Fact.

Every Prospective Evaluation Basis MUST transitively bind, through its exact
Producer Selections and Readiness Basis, each retry and partial-completion
definition plus the checkpoint verification that permitted its use. A copied
boolean such as `retry_allowed` or `partial_allowed` is not a substitute for
those owner identities and semantics.

### Prospective applicability before plan closure

A Prospective Applicability Basis is required whenever an expected claim would
otherwise have its observation obligations omitted because it is considered
not applicable before collection. If no claim is prospectively pruned,
Evaluation Coordination MAY omit this basis only by retaining every expected
claim and every requirement-derived observation relation in the Observation
Plan and recording that no prospective applicability pruning occurred.

The basis MUST bind:

- one exact Evaluation Requirements identity and its complete expected-
  claim and observation-relation inventory;
- exact subject/snapshot and scope identities;
- one exact Prospective Applicability Policy identity, version, semantic
  content, entailment-level compatibility, and currentness verification;
- one exact eligible prospective fact boundary;
- an exact finite membership of pre-existing canonical fact identities, their
  semantic owners, proposition scopes, currentness, reuse restrictions, and
  inclusion or exclusion reasons at the applicability semantic time;
- exactly one result for every expected claim: `retained`,
  `prospectively_not_applicable`, or `unresolved_and_retained`;
- complete derivation and accounting of every resulting retained or omitted
  observation-role relation, without changing the requirements or expected-
  claim inventory; and
- complete decisive reasons, included/excluded facts, and conflict accounting.

The eligible prospective fact boundary MUST identify the exact semantic owners
and repositories or resolvers consulted; admissible fact kinds and proposition
scopes; subject/snapshot and applicability scope; causal or semantic-time cut;
every required owner verification and reuse restriction; and deterministic
inclusion and exclusion rules. Every owner-recognized fact matching that
boundary MUST be included exactly once or excluded with an exact policy-derived
reason. If boundary completeness, fact identity, owner attribution, proposition
scope, currentness, reuse restriction, or conflict accounting cannot be
established, the affected claim remains retained.

The policy MUST declare exact compatibility with the final claim-applicability
semantics bound by the same Evaluation Requirements. For the same
expected claim, subject/snapshot, scope, bound facts, and applicable time
relation, the prospective proof MUST be sufficient to entail final
`not_applicable`. Shape compatibility or equal labels are insufficient.

Only exact canonical facts that existed independently before this evaluation
occurrence may support a prune. Their semantic owners establish exact identity,
content, proposition, scope, required currentness, and any owner-declared reuse
limitation. Contracts alone applies the Prospective Applicability Policy and
establishes the prospective result; an owner fact does not establish its own
prune. Ambient repository state, mutable prose, a projection, a prior result
without an admitted reuse rule, anticipated evidence, producer capability,
availability, authorization, readiness, cost, inconvenience, or missing data
cannot support a prune.

An unresolved, stale, conflicting, unsupported, missing, or ambiguous fact
retains the expected claim and every observation relation required to evaluate
it. It does not prove `not_applicable`. Positive proof for one whole expected
claim omits every relation required solely for that claim. A shared observation
role remains in the plan whenever another retained claim requires it.

The Observation Plan MUST bind the exact Prospective Applicability Basis when
one exists and completely account for every expected claim and every derived
retained or omitted relation. Changing the policy, eligible boundary,
subject/snapshot, scope, fact membership, fact currentness or reuse basis,
semantic time, result, compatibility, or accounting requires a new basis, plan,
and complete new occurrence chain.

Final applicability remains Contracts-owned semantic truth inside the final
evaluation context. Congruence is mandatory:

- every `prospectively_not_applicable` claim may publish only final positive
  `not_applicable` under the exact compatible final semantics; `applicable` or
  `undetermined` produces an evaluation refusal because its required
  observation obligations were omitted;
- every retained claim may later be `applicable`, `not_applicable`, or
  `undetermined` because none of its required observation obligations was
  removed; and
- changed facts or policy may not retroactively justify an earlier prune.

A congruence failure produces no Applicability Result, claim, metric, gate, or
canonical explanation truth for that context. It is not evidence insufficiency,
`not_computable`, `indeterminate`, or an implicit waiver.

Relation-level pruning of an otherwise applicable claim is not selected. Such
behavior requires a separately defined Observation Obligation Applicability
result and matching final relation-level congruence semantics.

<a id="complete-pruning"></a>
### Complete pruning with no collection

An Observation Plan MAY contain zero Producer Selections only when all of the
following are positively established:

- one complete Evaluation Requirements closure has a nonempty expected-claim
  inventory and a nonempty original observation-obligation population;
- its exact Prospective Applicability Basis establishes every expected claim as
  `prospectively_not_applicable` under the existing whole-claim positive-proof rules;
- complete original/shared-role and composition accounting leaves no retained
  observation obligation; and
- the exact admitted Formation Policy/configuration and precommitted lifecycle
  and semantic-time rules support this complete-pruning path.

Original requirements, claim inventory, relation inventory, and every omission's
exact proof remain bound. Coordination verifies structural closure from the
Contracts-owned applicability result; it does not establish the underlying facts
or recompute applicability. Initially empty obligations and any retained
observation-free claim do not qualify; generalized observation-free evaluation
remains deferred. This is a limit on this route, not a new
universal contract-admission requirement. Unsupported empty plans refuse; an
ordinary nonempty route is available only when its own requirements can be met.

This adds no pruning justification. Missing tools, capability limits, cost,
failed attempts, missing data, and author assertions cannot establish entry.
For example, an inventory limited to declared migrations cannot prove that a
claim checking for undeclared database-changing behavior is inapplicable.
Unresolved proof retains the affected claims/relations under the existing rules.
Empty membership after failed ordinary collection is a different lifecycle.

The explicit empty requested-reach population, genuine Engine authorization,
positive readiness, compatible use intent, and final pre-commit basis follow
[evaluation preparation](assurance-and-evaluation-preparation.md#evaluation-preparation).
No empty-plan Binding authorizes a producer. This route supplies no evidence
import, automatic pass, or completion exemption.

<a id="no-collection-commitment"></a>
### No-collection commitment and frontier

For complete pruning, Evaluation Coordination MUST produce one successful owner
outcome that commits the occurrence to its exact eligible Prospective Evaluation
Basis and seals the Occurrence Collection Frontier as **collection closed without
collection activity**. It establishes both separately attributable propositions;
it does not require prior collection-derived commitment. The outcome binds the
exact occurrence, Purpose, Plan, final basis, applicability proof/accounting,
Formation Policy, precommitted lifecycle/time rules, required verifications, and
its exact occurrence-state and commitment/frontier semantic checkpoint.

Closing authorization/readiness verifications MUST be inputs to the final basis
being committed. Any required refresh before commitment follows the bounded
pre-commit reformation rule; earlier unused bases cannot be committed. The later
occurrence-state checks and successful checkpoint belong to the owner outcome.
The prospective basis MUST NOT be mutated to include them or the future frontier.
No redundant verification is required solely because the final basis just formed.

Coordination MUST establish no prior conflicting commitment, no competing accepted
basis, no disqualifying collection provenance, and no permitted future collection
under the empty Plan. It accounts for the occurrence's exact bounded owner state,
not just an empty list of admitted evidence. Proven conflicting launches, attempts,
sources, or submissions prevent valid closure and remain preserved. A request to
collect under this Plan refuses. Raw rejected transport delivery does not itself
create a Producer Attempt; any durable owner provenance retains its actual role.
No Producer Attempt, Host Enforcement Fact, Dock/source terminal, no-attempt
terminal, or evidence admission may be fabricated to support the empty branch.

One successful owner outcome MUST expose no usable partial commitment/frontier
success. It may use existing lifecycle records; no new runtime type, service, or
distributed transaction is mandated. It establishes no evaluation truth or work
completion, and cannot recursively require reliance on the truth it enables.

Exact replay returns the same outcome, basis/frontier identities, and recorded
checkpoint without re-execution or substitution of newer verification facts.
Conflicting reuse refuses. Uncertain delivery establishes neither success nor
non-commit; resolve the authoritative outcome before dependent progression or
commitment with another basis. Later-discovered contradictory provenance follows
the existing closure-integrity boundary without rewriting history. Final-context
authorization remains a separate mandatory check even after historical commitment
success has been resolved. This path enables no post-commit producer retry;
the later-launch verification rule applies only while ordinary collection remains open.

### Occurrence collection frontier

On the ordinary collection route, after activity for the committed Prospective Evaluation Basis is
finished, Evaluation Coordination MUST form exactly one Occurrence Collection
Frontier before evidence-set formation. It may form the frontier only from the
precommitted Plan, stable Producer Selections, composition/multiplicity rules,
retry and partial-completion policies, exact later checkpoint verifications and
Coordination launch/refusal outcomes, and exact origin-owner launch, refusal,
attempt, source, submission, and terminal records. It MUST NOT invent a
producer, Dock, host, source, submission, or terminal fact. Complete pruning uses
the alternative commitment/frontier outcome above. The following frontier
requirements apply to both paths; selection-dependent populations are explicitly
empty on the complete-pruning path, while original/pruned relations remain accounted.

The frontier MUST:

- bind the one Evaluation Occurrence Identity and one committed Prospective
  Evaluation Basis;
- bind every Producer Selection and requirement-derived plan relation;
- bind each exact retry and partial-completion definition plus all owner
  verifications and Coordination outcomes that governed collection, including
  failed checks, exact checkpoint-to-launch/selection relations, and any declared
  recovery/continuity accounting;
- account for every permitted or refused launch, attempt, source observation,
  candidate/submission occurrence, partial/completed terminal, and no-attempt
  terminal required by the plan;
- prove that the actual attempt/submission topology conforms to the plan and
  every selection-bound policy; and
- close one causal frontier after which no new valid attempt, source
  observation, or candidate/submission occurrence can be added to the
  occurrence.

The frontier closes occurrence-level collection, not Contracts admission.
Admission results bind their earlier candidate/source/attempt, occurrence,
basis, subject/snapshot, scope, selection relation, admission policy, and
admission semantic time. They MUST NOT bind the identity of a future frontier,
evidence closure, or closure cut. After the frontier forms, every Contracts
admission operation causally attributable to an in-frontier candidate MUST
reach an exact terminal owner result before evidence-set formation can close.

One Evaluation Occurrence Identity may have multiple immutable unused
pre-commit Prospective Evaluation Bases, but at most one committed Prospective
Evaluation Basis, one Occurrence Collection Frontier, one Canonical Evidence
Set Closure, and one final evaluation-context/result chain. Exact replay returns
the same identities and content. Different successful frontier, closure, or
final-context content under an already committed or closed occurrence refuses;
it cannot create a second truth for that occurrence.

### Canonical evidence-set formation and closure

Canonical Evidence Set Formation is a Contracts evidence responsibility,
separate from candidate admission and claim evaluation. It begins only from an
exact committed Prospective Evaluation Basis, its sealed Occurrence Collection
Frontier, terminal admission accounting for every in-frontier candidate, and
the precommitted Canonical Evidence Set Formation Policy bound by the Purpose.
It MUST form an exact eligible evidence repository boundary rather than query
ambient current state.

For complete pruning, Contracts consumes the successful no-collection outcome
and its committed basis/sealed frontier. It MUST positively establish the exact
empty collection/admission population and complete original/pruned-relation
accounting under the Formation Policy, then form an ordinary Canonical Evidence
Set Closure with empty membership and no in-frontier admissions awaiting terminals.
Empty permitted membership MUST NOT filter out conflicting occurrence provenance:
the accounting boundary must still expose unexpected activity or open accounting,
which refuses closure. Absence of population knowledge is not proof of emptiness.
Owner/repository resolution remains policy-declared, not an ambient nothing-found
query. Independently established applicability facts remain exact separately owned
inputs in their declared roles, not members imported into the empty evidence set.

The eligible boundary MUST fix:

- owner repository identities and exact admissible evidence namespaces;
- Evaluation Occurrence Identity and committed Prospective Evaluation Basis;
- subject/snapshot and scope;
- complete Observation Plan, Producer Selection, and plan-relation identities;
- the exact sealed Occurrence Collection Frontier;
- every terminal admission result causally attributable to its in-frontier
  candidates;
- closure semantic time, the precommitted time-selection rule, and every named
  cross-owner time-comparison basis; and
- admitted canonical evidence kinds and deterministic inclusion/exclusion
  rules.

Membership is deterministic and non-discretionary. Every canonical evidence
record whose immutable admission result binds the same occurrence, Prospective
Evaluation Basis, subject/snapshot, scope, and supported Producer Selection or
plan relation, and whose admission is causally attributable to an in-frontier
candidate in the eligible repository boundary, MUST be a member exactly once.
An evidence record or admission result MUST NOT bind the later Canonical
Evidence Set Closure or its causal-cut identity. Claim desirability, finding
direction, metric effect, gate effect, producer preference, arrival order, or
evaluator choice cannot change membership.

Formation MUST account for every Producer Selection and every required plan
relation, including:

- zero or more exact Producer Attempts permitted by the selection-bound retry
  policy;
- every attempt terminal, including refusal, timeout, cancellation, partial,
  and completed states under the exact partial-completion policy;
- every source observation and durable origin-closed candidate/submission
  occurrence inside the frontier;
- every Contracts admission replay, refusal, or idempotent admission outcome;
- every admitted canonical evidence record;
- every durable origin-closed duplicate attempt, source-observation, or
  candidate/submission occurrence inside the frontier;
- every formation nonmember or exclusion, with an exact reason; and
- whether the plan's multiplicity/composition and prospective-pruning
  accounting is complete.

Raw transport delivery replay, interface-local correlation, framing retries,
and adapter-local duplicate handling are not semantic occurrence records and
are not required in formation accounting. They enter that accounting only when
an appropriate origin owner independently closes a bounded durable occurrence
record. Such a record still cannot create a second evidence member from the
same exact canonical evidence identity.

Complete accounting does not assert that every selection produced sufficient
evidence. A selection may close with no attempt because launch was refused, or
with attempts that failed, remained partial, produced refused candidates, or
produced evidence later insufficient for a claim. The closure records those
facts exactly; claim evaluation decides their semantic consequence only after
the final context exists.

The Formation Policy defines deterministic accounting of retries, duplicates,
and partial terminals; the formation responsibility applies it. Neither may
grant a retry, authorize partial completion, redefine a selection's completion
contract, or treat a nonconforming topology as valid merely because it is fully
recorded. An unauthorized extra attempt, impermissible partial result, or
violated multiplicity/composition relation causes formation refusal.

Retry, duplicate, and identity rules are exact:

- raw delivery replay has no semantic member effect;
- same canonical evidence identity and bytes admits idempotently to one member;
- same identity with different content refuses and remains refusal accounting;
- every non-idempotent retry is a distinct attempt and all its durable
  observations, submissions, admission results, and admitted evidence remain
  separately accounted without replacing an earlier attempt; and
- all matching admitted records are members. Later claim policy may resolve
  conflicts or select a compatible subset only within that closed membership.

Canonical evidence membership has set semantics over exact canonical evidence
identity, not byte equality, value equality, or a content fingerprint alone.
Distinct admitted evidence identities remain distinct members even when their
values or encodings are equal. Multiple duplicate submission or admission
occurrences may account to one member only when the canonical evidence identity
itself is the same. Provenance accounting separately preserves the exact
multiplicity and causal graph of attempts, observations, submissions,
admissions, refusals, terminals, and formation exclusions.

Serialization may impose one deterministic canonical order for fingerprinting
and presentation, but arrival, repository, producer, retry, or serialization
order has no semantic precedence. When a claim or sampling policy needs a
sequence, that exact sequence relation must be present in the admitted evidence
or another named canonical input; it is not inferred from set ordering.

A **formation nonmember or exclusion** lies outside set membership under the
Formation Policy because it is outside the boundary, was refused, or otherwise
fails the deterministic membership rule. A **claim-policy nonselection** is an
existing member that one Evidence Evaluation Policy does not select for one
claim-evaluation role. Claim-policy nonselection MUST NOT alter, reopen, or
remove set membership.

Formation MUST refuse rather than close when:

- the frontier is absent, ambiguous, not causally closed, or inconsistent with
  an owner terminal;
- repository, occurrence, basis, subject, scope, selection, frontier, or
  semantic-time identity is absent, ambiguous, mixed, substituted, or ambient;
- any required selection or plan relation lacks terminal accounting;
- any candidate or admission operation causally descended from an in-frontier
  candidate remains open;
- actual attempt/submission topology violates the Plan, stable Selection,
  retry, partial-completion, multiplicity, or composition rules;
- an admitted matching record is omitted or appears more than once;
- a member belongs to another occurrence, Prospective Evaluation Basis,
  subject/snapshot, scope, or unsupported plan relation;
- retry, durable-duplicate, partial, prune, or formation-exclusion accounting
  is incomplete;
- the precommitted Formation Policy or any required owner identity cannot be
  resolved under the exact required owner observations at closure; or
- membership depends on a claim finding, metric, gate, or desired outcome.

Before a successful Closure exists, formation may return only the one Closure
determined by the precommitted policy and exact sealed frontier or a formation
refusal. Once closed, membership is immutable. Re-presentation of the same
exact inputs returns the same Closure; any different policy, boundary, member,
terminal, exclusion, accounting, or semantic-time content under the same
occurrence refuses. A different Closure or final evaluation requires a
deliberately new occurrence.

An attempt, source observation, or candidate/submission occurrence causally
after the sealed frontier is outside that Closure and remains bound to the
occurrence, Prospective Evaluation Basis, attempt, source, and owner under which
it actually occurred; it cannot be rebound to another occurrence. Its origin
owner may preserve it only as exact post-frontier or invalid occurrence
provenance, not as a valid addition to the sealed collection. An admission
result or matching canonical evidence record discovered after Closure but
causally descended from an in-frontier candidate proves that admission work was
open or matching membership was omitted when Closure formed. That is a closure-
integrity violation, not permissible late evidence and not a basis for mutating
membership.

Exact replay of an already accounted occurrence changes nothing. New semantic
truth requires a deliberately new evaluation and its own applicable occurrence-bound
collection or complete-pruning commitment/frontier and evidence closure. No
cross-occurrence transfer or automatic reuse of an old-occurrence record is authorized.

Claim evaluators MUST receive the one exact Canonical Evidence Set Closure from
the final evaluation context. An Evidence Evaluation Policy may select eligible
members from that already closed set and must account for every selected and
nonselected member. It cannot expand the set, query ambient repositories,
reopen formation, import a late record, or define the set whose result it is
intended to evaluate.

### Final evaluation-context closure

The final evaluation context forms only after Canonical Evidence Set Closure.
It MUST bind exactly:

- Evaluation Occurrence Identity and Evaluation Purpose;
- Evaluation Requirements with exact source and contract permitted use, and any Prospective Applicability Basis;
- Observation Plan and every Producer Selection;
- every selection-bound retry and partial-completion definition, its exact
  formation-time verification, and, through the frontier, the complete later
  launch-verification and recovery/refusal lineage;
- Evaluation Authorization Binding and the exact final-closure Authorization
  Verification Observation;
- Readiness Basis and Assessment, Attempt Disposition, and Prospective
  Evaluation Basis;
- Occurrence Collection Frontier, including the no-collection commitment outcome
  and checkpoint when applicable, and every terminal admission result in its
  eligible causal population;
- Canonical Evidence Set Formation Policy and Closure;
- contract/claim/applicability/evidence-policy/evaluator/metric/gate
  definitions and exact configurations required by the resolved requirements;
- final evaluation semantic time and every named time/currentness relation; and
- complete cross-owner identity and pre-truth closure accounting.

Context formation MUST refuse before semantic truth when any required identity
cannot be resolved, any closure is mixed or open, the evidence closure is
incomplete, or Engine's required final-closure verification is not positive.
After a valid context exists, Contracts determines final applicability and
checks it against any prospective prune before publishing an Applicability
Result. Incongruence produces an evaluation refusal and no Applicability
Result, claim evaluation, metric result, semantic gate decision, or canonical
explanation; it does not retroactively make the already exact context or
evidence-set membership ambiguous.

On the complete-pruning path, final applicability MUST resolve the exact declared
fact/proof inputs and apply its compatible bound semantics; copying the prospective
label is insufficient. A reference supplies no undeclared input role. Required
owner/currentness checks remain, but historical or formation-only facts do not
become universally continuing-current. A new producer observation cannot be hidden
inside verification. If the required proof needs such collection, this path cannot
supply it. Missing final-context inputs refuse formation; an otherwise valid
context with any pruned claim applicable or undetermined refuses the truth chain
as above. Neither failure reopens collection or rewrites earlier closures.

One final context consumes one occurrence and one Prospective Evaluation Basis.
It cannot combine occurrences, purposes, prospective bases, or evidence-set
closures even when their content fingerprints, subjects, policies, or members
appear equal. At most one final evaluation-context/result chain may close for
the occurrence; exact replay returns it, while different content refuses.

A new posture-selected occurrence cannot adopt a previous direct occurrence's
authorization, observations, evidence set, or result. Definitions may be resolved
again and history may be read/replayed, but evidence-set membership remains
occurrence-specific. Existing exact pre-existing fact roles for prospective
applicability remain conditional on their admitted rules and reuse restrictions;
they do not provide general evidence import. A new validation of an old report
establishes only the report property actually evaluated, not that the current
implementation passed the old execution's tests. The required selected plan
determines which observations must be collected; unrelated development tests
are not automatically required to repeat.

Discriminating requirements-source cases:

| Case | Required outcome |
| --- | --- |
| Exact eligible contract evaluated directly, then selected by posture with equal rules. | New source means new Purpose and occurrence; the prior pass and evidence cannot be relabeled or imported. |
| A direct closure omits a contract-rooted claim, required evaluator/policy binding, or dependency. | Refuse formation; direct evaluation is not incomplete evaluation or unknown readiness. |
| Permitted use changes with unchanged claims. | New admitted revision and new evaluation identity; earlier results retain their original permitted use and intent. |
| V validates Diagnostic-only D as data. | V's admitted rules and permitted use govern that evaluation. Its bounded result does not admit D, widen D's use, or supply D's execution evidence. |

For complete pruning, all expected claims remain accounted. Neither empty
membership nor final non-applicability creates a satisfied finding, fabricated
metric denominator/value, or automatic gate pass. Metrics and gates apply their
exact admitted population and non-applicability semantics. The existing all-N/A
example remains policy-specific; this route adds no universal gate algorithm or
N/A-to-completion mapping. Canonical explanation retains each exclusion and proof,
and projections cannot relabel no producer activity as tests passed.

### Idempotency and replay matrix

| Input or event | Required semantic behavior |
| --- | --- |
| Same occurrence identity and exact purpose-formation content | Exact formation replay returns the same Purpose identity and content. |
| Same occurrence identity with different formation content | Refuse conflicting reuse; do not mutate the earlier Purpose. |
| Request for deliberately new formation with an equal Evaluation Content Fingerprint | Evaluation Coordination allocates a new occurrence identity and forms a new Purpose and downstream chain. No earlier authorization, attempt, evidence, or result transfers. |
| Same semantic occurrence under a different transport correlation | Preserve one semantic occurrence; correlation remains interface-local. |
| Exact authorization-request delivery replay | Engine may return the same issuance outcome for the exact request identity and bytes; currentness still requires a separate checkpoint verification. |
| Same stable semantic chain with only a new checkpoint-current observation before occurrence commitment | Keep the Plan/Purpose/binding only under the complete unchanged and positive-currentness rule; form fresh readiness and any new Prospective Evaluation Basis. Record every earlier basis immutable, unused, and ineligible for launch or no-collection commitment. |
| Stable selection definition, role, producer/configuration, reach, adapter, prerequisite, retry, partial-completion, or composition change | New Selection, Plan, occurrence, Purpose, request/binding, readiness, and prospective basis. |
| Fresh positive later launch verification under the committed basis | Record a distinct exact checkpoint and launch outcome under the post-commit rule; preserve the formation checks and basis unchanged. |
| Permitted retry under one unchanged current Prospective Evaluation Basis | Require its applicable later checks and unexhausted declared allowance; create a new attempt identity in the same occurrence and selection. Exact delivery replay alone remains idempotent. |
| Fact change requiring a new prospective basis after occurrence commitment | Preserve and terminate or abandon the occurrence without new truth; subsequent evaluation requires a deliberately new occurrence and its applicable route. |
| Raw candidate delivery replay | Interface-local handling creates no required semantic occurrence. If an origin owner closes a durable duplicate submission, account for that occurrence without creating another evidence member from the same exact evidence identity. |
| Exact no-collection outcome replay, conflicting reuse, or uncertain delivery | Preserve the same committed basis/frontier and checkpoint on replay; refuse conflicting reuse; resolve uncertain owner outcome without assuming success or non-commit. No new checkpoint facts replace history. |
| New occurrence whose evidence membership is also empty | Empty membership does not merge identities. Form its own Purpose, authorization, readiness/basis, commitment/frontier, evidence closure, and final chain. |
| Same candidate or evidence identity with different content | Refuse; never overwrite, merge, or select the favorable variant. |
| Re-run of set formation over the exact sealed frontier, closed boundary, and policy | Return the same closure identity, membership, and accounting. |
| Same closure identity with different membership/accounting | Refuse as corruption or substitution. |
| Late activity after closure | Preserve every record under its original occurrence and basis. Post-frontier source activity is outside the Closure; omitted in-frontier admission or evidence demonstrates a closure-integrity violation. Neither changes membership or permits rebinding. New truth requires a new occurrence and its own applicable collection or complete-pruning closure. |
| Exact historical replay | Resolve the same historical chain and reproduce the same owner results without new current authority or collection. It neither grants nor denies operational reliance. |
| Present re-evaluation of the same subject and definitions | New occurrence, new Purpose and authorization path, new readiness/basis, new evidence closure, and new final context. |

### Semantic times and currentness

Semantic times are named owner facts. A timestamp string alone does not prove
ordering, comparability, freshness, or currentness. Every policy that compares
times across owners MUST name the admitted comparison basis or causal relation.
The lifecycle always preserves exact predecessor identities even when no global
total order is available.

Any rule for selecting a semantic time or causal frontier that can affect
membership, currentness, applicability, or truth MUST be fixed under an exact
named owner definition before the values whose direction it can affect are
available. The actual owner-issued checkpoint may form later at its proper
stage, but it MUST NOT be retrospectively selected from multiple permissible
times after inspecting candidate, evidence, claim, metric, or gate direction.

| Named semantic time or checkpoint | Owner and required binding | Currentness or reformation consequence |
| --- | --- | --- |
| **Requirements selection time** | The requirements-source owner binds the exact source closure, admitted contract/permitted use, definitions and currentness; Assurance Selection additionally binds snapshot, selector, policies, and exceptions. Direct resolution binds nomination and compatible-rule provenance. | A different requirements identity starts a new downstream chain. |
| **Prospective applicability time** | Contracts applicability responsibility binds the exact pre-existing fact cut and policy used before plan closure. | Changed facts, policy, scope, subject, or time basis that affects pruning requires a new applicability basis, plan, and occurrence. |
| **Plan and purpose formation time** | Evaluation Coordination binds exact plan content, occurrence identity, use intent, and formation provenance. | Content change requires a new occurrence; exact replay preserves the original time and result. |
| **Authorization issuance time** | Engine binds exact request/purpose/plan and private authority lineage. | Issuance remains historical; it is not a currentness observation. |
| **Authorization verification time** | Engine answers for the named launch, no-collection pre-commit, final-closure, or historical checkpoint. | A negative or unverifiable current-use result refuses the governed boundary without rewriting issuance or history. |
| **Readiness assessment time** | Evaluation Coordination binds current capability/ceiling verification, availability, prerequisites, Engine verification, and every other readiness fact. | Before commitment, fresh formation facts over stable definitions produce a fresh readiness closure. After commitment, permitted later launch checks are separate provenance under the unchanged basis; changed fixed semantics require the applicable new occurrence chain. |
| **No-collection commitment/frontier checkpoint** | Coordination binds the exact final pre-commit basis and its closing Engine/readiness checks, occurrence-state checks, and one outcome establishing commitment plus sealed empty frontier. | Exact replay preserves this outcome/time; unresolved or conflicting state cannot authorize progression. No producer retry is enabled. |
| **Attempt and observation time** | Producer, Dock, and host provenance bind exact occurrence, prospective basis, selection, invocation, terminals, actual reach, and enforcement facts. | Retries remain separate attempts within the same unchanged occurrence and basis; later facts do not rewrite earlier attempts. |
| **Occurrence collection-frontier time** | Evaluation Coordination binds the causally closed collection population and applicable origin-owner terminals, or the complete-pruning outcome establishing commitment and an empty frontier together. | No later launch, attempt, source, or submission activity is validly added to that occurrence. Admission work for in-frontier candidates must still become terminal. |
| **Evidence admission time** | Contracts admission records the exact source-validity and policy decision for candidate-to-evidence transition. | Admission-time validity remains historical; it is not evaluation-time evidence currentness. |
| **Evidence-set closure time** | Contracts applies the precommitted rule to the sealed frontier, terminal admission population, membership, formation exclusions, and complete accounting. | Later records cannot enter the set. Different closure content under the occurrence refuses. |
| **Final evaluation time** | Contracts binds evidence currentness, final applicability, claim/metric/gate definitions, final Engine verification, and the complete context. | Results are immutable historical truth for this time and context. |
| **Operational reliance time** | The downstream reliance owner universally resolves the exact historical Evaluation Authorization Binding and validity outcome, then applies its admitted `historical_validity_only` or `continuing_current` mode plus every other declared present-use dependency. | Current Engine verification is required only by `continuing_current`; either mode preserves historical truth and cannot inherit evaluation authorization as action authority. A later effect has a distinct fresh Action Authorization Binding and commit-time checks. |

The causal order between named stages is mandatory even when their time values
are not numerically comparable. No owner may substitute its local receipt time
for another owner's semantic time or infer currentness from recency alone.
Every cross-owner comparison MUST bind the exact comparison-policy identity,
each input owner and semantic-time identity, the admitted causal or comparison
relation, the comparison owner, and its exact owner-produced result. Transport
receipt and local wall-clock values may be retained as provenance but cannot
silently substitute for another owner's semantic time.

### Historical resolvability and exact replay

Every retained final evaluation context and result that claims deterministic
explanation, audit, or replay MUST preserve or be able to resolve, for the same
declared retention horizon, the exact historical chain below:

- Evaluation Requirements, exact source closure/owner and nomination or selection
  provenance, contract revision and permitted use, and Evaluation Use Intent;
- subject/snapshot and scope; Charter, posture snapshot, selector, policies,
  exceptions, Resolved Assurance Requirements, and their exact revisions when
  applicable;
- Prospective Applicability Policy, fact membership, Basis, results, and
  accounting, eligible fact boundary, and exact final-applicability
  compatibility when pruning occurred;
- Observation Plan, every Producer Selection and stable definition reference,
  including exact retry and partial-completion definitions,
  Evaluation Occurrence Identity, content fingerprint, Purpose, use intent, and
  authorization request occurrence, plus every precommitted formation,
  boundary-selection, and semantic-time rule;
- Engine Authorization Binding, issuance lineage sufficient for owner
  verification, verification observations, and supersession relations;
- Readiness Basis, every checkpoint-current owner fact, Assessment, Attempt
  Disposition, Prospective Evaluation Basis, and retry/partial-policy
  verification result; every later launch check and Coordination outcome with
  exact owner, input, time, selection/request/attempt relation, failure reason,
  bounded instance choice, and recovery/continuity accounting;
- every Producer Attempt, source observation, Actual Observed Reach, Host
  Enforcement Fact, Dock terminal, candidate/submission occurrence, evidence-
  admission result/refusal, and canonical evidence record in the closure's
  accounting boundary;
- Occurrence Collection Frontier and any no-collection commitment outcome, its
  closing checkpoint/basis relationship, original/pruned accounting, and owner
  occurrence-state proofs; Canonical Evidence Set Formation Policy,
  eligible boundary, exact membership, terminal and formation-exclusion
  accounting, and Closure;
- final evaluation context, applicability and claim results, Evidence
  Evaluation Policies, Metric Definitions/configurations and results, Gate
  Policy and decision, and complete canonical explanation; and
- each semantic owner's identity, exact semantic content/version,
  currentness-at-the-recorded-checkpoint fact, named semantic time, and any
  time-comparison or causal relation required to reproduce the decision.

Historical resolution MAY use retained content, an immutable owner repository,
an exact owner proof, or another later-authorized mechanism. This decision
chooses none. It MUST resolve the same owner-recognized semantics, not a current
replacement, nearest version, projection, rendered summary, cache guess, or
transport copy.

At final evaluation-context closure, Contracts MUST establish that every
required dependency is exactly identified and covered by an owner-declared
historical-resolution obligation compatible with the result's declared
retention horizon. An absent, ambiguous, substituted, or already unresolvable
dependency refuses closure.

A later owner outage may cause an exact owner-produced historical-verification
or resolution observation to report `unverifiable`, `unavailable`, or another
owner-defined non-success outcome. A permanent loss may violate that owner's
recorded retention obligation. Neither event rewrites the immutable historical
result or permits substitution of a similar current object.

A retained historical result may be projected only with the exact owner-
attributed resolution observations and an explicit statement that exact
historical replay is presently unavailable. This record introduces no new
cross-owner aggregate historical-verification status.

One canonical explanation remains source truth. A concise default human view and
requested deeper drill-down are deterministic projections over that same truth,
limited independently to the highest detail both available and authorized for
disclosure. Reduced or denied detail does not trigger reevaluation or create a
new semantic result. Concrete projection decisions, UI/API shape, and any fixed
number of density levels remain deferred.

Those observations and the retained result grant no present authority. Whether
any downstream action may rely on the result is decided by the separately
owned Operational Reliance Policy and exact present-use inputs within the
mandatory [governed-completion boundary](operational-reliance-and-actions.md#governed-completion).
That policy universally requires historical resolution/validity and explicitly
selects `historical_validity_only` or `continuing_current` for present Engine
status. This record neither grants nor universally denies operational
eligibility, and any later effect requires separate fresh action authorization.

Exact historical replay uses the recorded semantic times and historical owner
facts. It MUST NOT rerun producers, substitute current definitions, re-evaluate
under a new semantic time, call a current authorization result historical, or
claim present authority. A present evaluation, even of equal content, is a
deliberately new occurrence.

### Semantic responsibility map

| Responsibility | Owns | MUST NOT own or infer |
| --- | --- | --- |
| Contracts-domain Evaluation Coordination | Evaluation Occurrence Identity allocation; Evaluation Content Fingerprint and Purpose formation including requirements source and contract permitted-use compatibility; exact Plan/Purpose congruence; readiness reformation boundary; occurrence commitment to one prospective basis; later launch permission/hold/refusal over exact owner checks, with complete recovery accounting; one exact Occurrence Collection Frontier from origin-owner terminals or the supported no-collection outcome establishing commitment and frontier together. | Engine issuance/currentness, prospective or final applicability truth, invention of origin or admission terminals, evidence admission/set membership, claim/metric/gate truth, or operational reliance. |
| Contracts requirements resolvers | Assurance Selection produces posture-specific Resolved Assurance Requirements; direct requirement resolution produces complete direct-contract requirements. Both preserve exact contract permitted use and complete source provenance. | Self-admission, capability-driven requirement changes, reclassifying contracts, Engine authority, or governing-completion assignment. |
| Contracts definition authorities | Deterministic Policy Admission Closure, immutable identity, compatibility, currentness, and historical resolution of Prospective Applicability Policies and Canonical Evidence Set Formation Policies, including exact relevant governance-source roles/scope/dependency modes and complete candidate accounting. | Treating authored candidate text or a bare link as admitted policy, recursively letting a candidate admit its own rule, or applying a policy to select a desired plan, set membership, applicability, claim, metric, or gate outcome. |
| Contracts applicability responsibility | Exact resolution and application of an already admitted Prospective Applicability Policy; Prospective Applicability Basis formation; final applicability; prospective/final congruence. | Policy self-admission, ambient or evidence-dependent plan pruning, producer selection, Engine authority, or requirement relaxation. |
| Engine Context Resolution | Evaluation Authorization Binding issuance, private lineage, named verification observations, historical validity, currentness, and supersession. | Occurrence allocation, content fingerprints, applicability, plan adequacy, readiness, evidence membership, or replay reconstruction by Contracts. |
| Producer Capability, Dock/adapter, prerequisite, retry, partial-completion, and availability owners | Their exact semantic definitions and checkpoint-current verification or occurrence facts under independently attributable authority. | Altering Producer Selection, assurance requirements, Engine authorization, readiness outcome, or evidence truth. |
| Contracts evidence admission | Candidate/source/provenance validation and immutable canonical evidence records with exact admission time and basis. | Selecting evidence-set membership by outcome, closing the set, claim truth, or rewriting origin provenance. |
| Contracts evidence-set formation responsibility | Exact resolution and application of the precommitted admitted Formation Policy; eligible boundary over the sealed frontier and terminal admissions; exact set membership; complete plan/attempt/candidate/admission/terminal/formation-exclusion accounting; closure time and identity. | Policy self-admission, candidate admission, claim-specific eligibility or nonselection, findings, metrics, gates, or ambient repository queries. |
| Contracts final-context and evaluation responsibilities | Final context closure, final applicability congruence, claim-policy selection/nonselection within the closed set, claim/metric/gate truth, and canonical explanation. | Reopening evidence membership, importing late evidence, reconstructing Engine authority, or operational mutation. |
| Producer, Dock, adapter, and enforcing host | Exact attempt, source, candidate-submission, terminal, actual-reach, and applied/refused enforcement provenance for their bounded propositions. | Canonical evidence, set closure, applicability, truth evaluation, or current-use authority. |
| Definition and repository owners | Exact historical definition/content resolution, owner provenance, semantic-time/currentness observations, and declared retention obligations for their objects. | Replacing an historical identity with a current equivalent or making a consumer the semantic owner. |
| SDK, CLI, Tauri, Flow, Pipeline, skills, and transports | Bounded composition, rendering, correlation, delivery, and deterministic projections or assemblies of exact owner objects. | Occurrence identity, owner formation replay, evidence membership, semantic-time invention, historical verification, or truth reinterpretation. |
| Work Progression and other Operational Reliance owners | Exact action question, finite result/target closure, universal historical Binding validity, explicit `historical_validity_only` or `continuing_current` mode, Basis, atomic Determination with paired eligibility/disposition facets, and Reliance Refusal. Work Progression is the proposed local/parent eligibility owner. | Rewriting this occurrence's identity, evidence closure, historical truth, issuing action authority, or mutating state. |
| Action requester, authorization, executor, and state/effect owners | Exact Action Intent; Action Authorization Binding or Refusal with declared advisory/verified/enforced provenance; Action Attempt Identity and Terminal; through-commit verification; and authoritative Mutation Receipt. | Treating Engine evaluation authorization as action authorization, fabricating identity/delegation/enforcement, ordinary retry after uncertain commit, or claiming committed mutation without the state-owner receipt. |

Every object introduced by this record is a canonical semantic definition, an
owner-produced canonical record/result, durable provenance, or a bounded value
inside one of those categories under the companion ontology. None is an
Artifact Instance. Physical colocation or shared repository primitives do not
merge semantic owners.

### Discriminating scenario corpus

| Scenario and inputs | Expected semantic outcome | Invariant proved |
| --- | --- | --- |
| **Complete-pruning closure.** A and B originally require observations; exact independent facts and compatible rules prove both inapplicable. | Coordination commits the final closing-check basis and seals the empty frontier in one outcome; Contracts forms a fully accounted empty evidence closure and performs normal final applicability. | No fake activity or skipped closure. |
| **Unsupported empty or mixed contract.** No original observation obligations exist, or a retained observation-free claim remains. | The complete-pruning route refuses; do not drop the claim, invent a producer, or infer generalized observation-free support. | Narrow route eligibility does not redefine all contract admission. |
| **Conflicting collection or uncertain no-collection outcome.** Durable activity is attributed to the empty occurrence, or commitment delivery is unresolved. | Conflicting activity prevents valid closure and remains provenance. Uncertainty requires owner resolution before progression; a resolved historical success is preserved even if later final authorization refuses. | An empty member filter is not proof of absent activity; uncertainty is not non-commit. |
| **Two empty memberships.** A deliberately new evaluation also closes an empty evidence set. | It has its own occurrence, authorization, commitment/frontier, closure, and final chain; the empty set does not merge identities. | Empty membership is not evaluation identity. |
| **Equal content, deliberate rerun.** A user requests deliberately new formation for the same subject, requirements, plan, objective, and use intent after an earlier result. | The Evaluation Content Fingerprint may match, but Evaluation Coordination allocates a new Evaluation Occurrence Identity and creates a new Purpose, authorization path, readiness/basis, frontier, evidence closure, and final context. Nothing transfers automatically. | Semantic occurrence is owner-issued and not content identity. |
| **Exact purpose retry through another transport.** The same occurrence and exact formation content arrive under a different CLI/Tauri correlation. | Exact formation replay returns the same Purpose. The correlation creates no occurrence or authority. | Delivery and semantic idempotency are distinct. |
| **Occurrence key reused with changed plan.** One Producer Selection or requested reach changes while the occurrence identity is reused. | Purpose formation refuses conflicting reuse. A new occurrence and full downstream chain are required. | Occurrence identity cannot mask content substitution. |
| **Availability refresh before commitment.** Every upstream semantic input, precommitted formation/time rule, stable selection definition, prune-supporting fact, Purpose, and Engine binding remains exact and positively current; only permitted producer availability changes, and no durable launch or collection effect exists. | A fresh Readiness Basis/Assessment and prospective basis may form under the same plan, Purpose, and occurrence. The earlier basis remains immutable, unused, and ineligible for launch or no-collection commitment. | A bounded operational-fact refresh need not reform semantic selection. |
| **Capability version replaced.** Readiness discovers that the selected capability definition is no longer applicable and a successor is required. | Selection, Plan, occurrence, Purpose, Engine request/binding, readiness, and prospective basis reform. The successor is not substituted into the old selection. | Stable definitions and checkpoint facts have different change boundaries. |
| **Fixed prerequisite changes after partial collection.** One attempt produced a partial observation before a replacement prerequisite definition becomes necessary. | Preserve partial history; the occurrence cannot adopt a replacement basis or close new truth. A later evaluation needs a new occurrence. | One occurrence cannot mix prospective bases. |
| **Fresh confirmation before retry.** Selection B times out; its original policy permits another attempt. New owner checks confirm the same fixed inputs while collection is open. | Keep the committed basis and original readiness unchanged; record the new checks and B launch outcome, then create a distinct B attempt. Include the timeout and both checkpoints in closure accounting. | A new verification is not a replacement evaluation basis. |
| **Temporary check outage versus revocation.** A check service is unavailable, or the owner positively reports revocation; a later request returns positive. | Both first outcomes block launch and remain distinct history. Resume only if the originally bound owner/lifecycle rules permit that exact recovery and establish required continuity. A positive result alone cannot undo revocation or authorize intervening work. | Failure does not invent recovery permission. |
| **Permitted worker choice versus pinned worker.** The original selection permits equivalent instances in a bounded pool, or instead pins worker 1. | Worker 2 may serve only in the first case with exact declared semantic properties and fresh checks; record its actual identity. The pinned case needs a changed selection and new occurrence. | Operational variation must have been declared in advance. |
| **Staggered selections and sealed frontier.** A completed; B's required launch check fails. Alternatively, a retry is requested after the frontier sealed. | Preserve A's history and B's refusal without dropping B's obligations. No check can reopen the sealed frontier or reset retry limits. | Permission is selection/checkpoint-specific; closure stays final. |
| **Planning guess says inapplicable.** Project prose says a claim probably does not apply, but no named policy over a complete eligible canonical-fact boundary proves it. | The claim and every observation relation required to evaluate it remain in the Plan. Missing confidence cannot prune work. | Prospective pruning requires positive owner truth and complete fact accounting. |
| **Selective prospective facts.** One favorable fact would prove a claim not applicable, but another matching current fact inside the declared boundary conflicts with it. | Both facts are accounted. The claim is `unresolved_and_retained`; neither fact can be hidden to prune the claim. | A finite eligible-fact boundary prevents cherry-picking. |
| **Positive whole-claim prune with a shared role.** A named compatible policy over the complete exact fact boundary positively proves claim A not applicable; claim B remains retained and shares one observation role with A. | A is `prospectively_not_applicable`; relations required solely for A are omitted, but the shared role remains planned for B. The full expected-claim inventory remains unchanged. | Claim-level pruning derives complete relation accounting without inventing relation-level applicability. |
| **Prune contradicted at final applicability.** A plan omitted a claim's observation obligations under a bound prospective prune, but final applicability would be `applicable` or `undetermined`. | The already formed final context remains exact, but applicability processing refuses before publishing an Applicability Result, claim, metric, gate, or explanation truth. The omission cannot become evidence insufficiency. | Prospective proof must entail final claim-level non-applicability. |
| **Formation rule chosen after observations.** Two admitted Formation Policies would produce different membership, and the requester tries to choose after seeing the evidence direction. | Purpose formation or later congruence refuses because the exact policy and truth-affecting time/boundary rules were not precommitted in the Purpose. | Evidence-set rules cannot be selected to fit an outcome. |
| **Two retries disagree.** Two distinct permitted attempts under one selection yield admitted canonical evidence with opposing values. | Both evidence records are members of the closed set. The claim's named consistency/evidence policy accounts for the conflict; formation cannot select the favorable retry. | Set closure precedes and constrains claim selection. |
| **Raw and durable duplicate submission.** The same candidate bytes are delivered twice at transport, and the source owner separately closes two durable duplicate submission occurrences under one exact candidate identity. | Raw delivery replay requires no semantic record. Both durable submission occurrences and their admission outcomes are accounted, while the same canonical evidence identity remains one set member. Different content under that identity refuses. | Transport replay, provenance multiplicity, and evidence identity are distinct. |
| **Completed plan with no evidence for one selection.** A durable host refusal terminal exists for one required selection, the frontier accounts for every relation, and all in-frontier admission activity is terminal. | Evidence-set closure may record zero evidence for that selection with complete refusal accounting. Later claim evaluation may be indeterminate; closure itself does not fabricate sufficiency. | Closure completeness is complete accounting, not successful collection. |
| **Unauthorized extra retry.** Every attempt is recorded, but one retry or partial terminal violates the exact selection-bound retry or partial-completion definition. | Frontier or formation refuses despite complete recording. The Formation Policy cannot retroactively grant the attempt or redefine completion. | Complete provenance cannot make a nonconforming topology valid. |
| **Ambient repository query.** A formation request says to use “all current evidence” without exact repositories, a sealed frontier, terminal admissions, or precommitted time rules. | Formation refuses and no set closure or final context forms. | Membership must be finite, exact, terminal, and non-ambient. |
| **Post-frontier favorable source.** A source observation or candidate occurs after the sealed frontier and would change a metric. | It remains provenance of the original occurrence and basis but is outside the Closure and cannot be rebound. New truth requires a new occurrence and its applicable collection or complete-pruning closure. | A later event cannot rewrite closure or provenance. |
| **Omitted in-frontier admission.** A matching canonical evidence record discovered after Closure descends from an in-frontier candidate whose admission was open or omitted. | This proves a closure-integrity violation. The set is not mutated, and the record is not legitimate late evidence. | The frontier must be followed by terminal, complete admission accounting. |
| **Arrival-order policy.** A claim evaluator asks to use the last-arriving member even though no canonical sequence relation exists. | The request refuses; canonical serialization order and arrival order carry no semantic precedence. | Evidence membership is a set unless an exact source relation supplies sequence. |
| **Cross-owner clock labels disagree.** Two local timestamps appear ordered, but their owners supply no admitted comparison policy or causal relation. | No cross-owner ordering or currentness conclusion is drawn. Any later comparison binds its policy, input time identities and owners, causal relation, and result. | Local timestamps do not create a global semantic clock. |
| **Historical policy unavailable.** A retained gate result references a policy version whose owner currently reports an exact unavailable-resolution outcome. | The result is not rewritten or replaced by a similar policy. Its projection carries the owner observation and states that exact replay is presently unavailable. This record makes no operational-reliance decision. | Historical identity, owner resolution, replay availability, and present use are distinct. |
| **Exact historical replay.** Every exact owner dependency and recorded semantic time resolves successfully. | The same closure, claim/metric/gate truth, and explanation are reproduced without producer execution or new authority. | Replay is historical reconstruction, not reevaluation. |
| **Projection offered as history.** A compact gate view is available but its canonical explanation or evidence closure cannot be resolved. | The projection cannot repair the missing owner chain or serve as historical replay input. | Projection identity never replaces source-owner resolvability. |

### Settled versus deferred

This record settles:

- Evaluation Occurrence Identity, Evaluation Content Fingerprint, Purpose
  identity, exact formation replay, delivery replay, and deliberately new
  evaluation as non-substitutable concepts;
- occurrence commitment to one Prospective Evaluation Basis once collection
  effects begin or the complete-pruning commitment/frontier outcome succeeds,
  the complete bounded pre-commit readiness-refresh rule, and
  immutable unused prospective bases;
- stable Producer Selection closure versus checkpoint-current capability,
  ceiling, availability, prerequisite, retry, partial-completion, host, and
  Engine verification facts;
- the exact reformation boundary for definition changes versus current-fact
  refreshes;
- separate later launch checks under an immutable committed basis, predeclared
  operational variation and recovery/continuity conditions, and complete
  checkpoint/refusal accounting without reopening sealed collection;
- Contracts-owned Prospective Applicability Policy/Basis, positive-proof-only
  whole-claim pruning, exact finite eligible-fact boundary, entailment-level
  final-policy compatibility, complete derived relation accounting, and
  mandatory congruence with final claim applicability, while deferring
  relation-level obligation applicability;
- Evaluation Coordination-owned Occurrence Collection Frontier, terminal
  admission sequencing, one committed basis/frontier/closure/final chain per
  occurrence, and causal-direction separation from earlier admission records;
- the bounded complete-pruning entry rule, explicit empty Plan and request
  population, final closing-check inputs in the committed basis, no-collection
  commitment/frontier outcome, replay/uncertainty, empty evidence accounting,
  and unchanged final-proof/result/completion boundaries;

- Contracts-owned Canonical Evidence Set Formation Policy, eligible boundary,
  deterministic membership, complete attempt/candidate/admission/terminal and
  formation-exclusion accounting, retry and duplicate treatment, evidence-
  identity uniqueness, set semantics, closure time, immutable closure, and
  refusal for ambient, ambiguous, mixed, nonconforming, or open membership;
- claim-policy nonselection only within the already closed set, without
  changing membership, and final context only after closure;
- the named semantic-time/checkpoint vocabulary and prohibition on assuming a
  global clock or substituting receipt time, plus exact precommitment and
  cross-owner comparison rules; and
- the minimum full-chain historical-resolvability and exact-replay invariant,
  preserving exact owner non-success observations without creating an aggregate
  status, rewriting historical truth, or deciding operational eligibility.

The proposed
[authored-artifact and evaluation-relationship decision](objects-and-governance.md#authored-relationships)
owns concrete relationship semantics, references, and requiredness for Charter,
Project Context, Environment Context, Work Specification, Decision Record, and
Risk Record. This record binds only exact subject/snapshot and scope identities
supplied through their proper owners and does not approve that companion.

The proposed
[operational-reliance and work-progression decision](operational-reliance-and-actions.md#reliance-actions)
owns Current-Use Eligibility and Operational Disposition as facets of one
atomic Determination, Operational Reliance Basis and Refusal, local closeout,
parent eligibility, explicit `historical_validity_only` versus
`continuing_current` present-use modes, action authorization, attempts,
terminals, receipts, and mutation boundaries. Historical resolvability and
replay grant none of them. Work Progression is only the proposed semantic owner
of local/parent eligibility; physical placement, work hierarchy/state model,
concrete policies, and implementation remain deferred.

Also deferred to later human-approved task-level planning are concrete schemas,
Rust types, APIs, identity allocation algorithms, timestamps or logical-clock
technology, storage layouts, repository implementations, retention durations,
registry and resolver mechanics, cache implementation, status-code encoding,
transports, crate/module placement, retry counts, timeouts, policy catalogs,
task slices, acceptance planning, and rollout. No concrete applicability,
evidence-formation, claim, metric, gate, or producer policy is selected.
Generalized observation-free evaluation, including initially empty obligations
and retained observation-free claims outside complete pruning, remains deferred.
Cross-occurrence evidence import or reuse and operational repair, quarantine,
or incident response for a later-proven closure-integrity violation are also
deferred and are not implied by complete historical accounting.

<a id="metric-gate"></a>
## Claims, metrics, gates, and explanation

This chapter owns numeric meaning, claim/metric/gate semantics, and canonical explanation. Every context binds complete Evaluation Requirements and its exact source, contract permitted use, and run intent. The posture-selected branch additionally binds the exact snapshot and assurance-selection lineage.

### Canonical model and vocabulary

The canonical semantic chain is:

```text
Evaluation Requirements (`posture_selected` or `direct_contract`)
  -> prospective evaluation purpose and requested observation reach
  -> Engine-owned Evaluation Authorization Binding
  -> separately owned readiness determination
  -> Prospective Evaluation Basis
  -> ordinary collection: source observation / candidate / Contracts evidence admission
       OR complete-pruning no-collection commitment
  -> sealed collection frontier and terminal admission accounting
  -> exact canonical evidence-set closure (empty for complete pruning)
  -> final Contracts evaluation-context closure
  -> applicability and claim evaluation
  -> zero or more named metric results
  -> gate policy
  -> semantic gate decision and canonical explanation
  -> action-specific Operational Reliance Basis, when used operationally
  -> one atomic Operational Reliance Determination
       - Current-Use Eligibility facet
       - fixed Operational Disposition facet
  -> separate Action Intent and Action Authorization Binding or Refusal
  -> Action Attempt, Action Terminal, and state-owner Mutation Receipt only after committed mutation
```

The following terms are normative. Terms in one row are not synonyms for terms
in another row.

| Term | Fixed meaning |
| --- | --- |
| **Source observation** | A source provider's bounded account of what it observed, including source-native value, unit or scale, subject/snapshot identity, collection state, and provenance. It is not canonical evidence and cannot satisfy a claim by itself. |
| **Evidence candidate** | A dock- or adapter-produced, evidence-shaped submission that binds one or more source observations to one exact Prospective Evaluation Basis. It is untrusted input to admission. Do not call it an evidence record or shorten it to “evidence” where that would imply admission. |
| **Canonical evidence record** | The immutable, content-addressed record produced only by Contracts-owned admission after exact identity, provenance, authority, subject, Prospective Evaluation Basis, unit/scale, normalization, admission-time source validity, and duplicate checks. “Evidence record” means this admitted object. |
| **Evidence Evaluation Policy** | An immutable, named, versioned Contracts-owned policy for exactly one declared claim-evaluation role: evidence eligibility/selection, sufficiency, freshness/currentness, consistency/stability, or uncertainty treatment. A selected contract, claim, or evidence requirement declares the roles and compatibility constraints it supports. An Evaluation Requirements closure may bind exact compatible policies to those roles; no policy is an umbrella override. |
| **Evaluation Authorization Binding** | The immutable, exactly identified, purpose-bounded Engine-owned result issued from one exact currently admitted `ContextResolutionEnvelope`. It transitively and opaquely binds that full envelope and its Engine authority/admission issuance basis while exposing only stable evaluation-relevant provenance: exact subject or scope, objective, intended evaluation use, Resolution stack semantics, requested observation reach, and Engine's determination that the request is admitted for that purpose. Its issuance is historical fact; Engine-owned currentness and supersession verification remain separate owner observations. |
| **Prospective Evaluation Basis** | The exactly identified pre-observation provenance closure for one prospective evaluation. It binds the exact evaluation purpose, subject or scope, objective and intended use, requested reach, Evaluation Authorization Binding, and complete Evaluation Requirements with their source and contract permitted use, and every required readiness and use-intent input. It contains no evidence-set identity and produces no claim, metric, gate, or other semantic truth. Its physical owner and lifecycle mechanics remain deferred. |
| **Applicability result** | The Contracts-owned determination, made before evidence selection, that a claim is `applicable`, `not_applicable`, or `undetermined`, with an exact basis. `not_applicable` is positive semantic proof, not missing data. |
| **Claim evaluation** | The complete result for exactly one expected claim in one evaluation context: its applicability result and, when applicable, an established finding or an indeterminate determination, plus application of every exact compatible Evidence Evaluation Policy binding, the independent evidence-assessment dimensions, and complete accounting. |
| **Finding** | The semantic satisfaction result established by a claim evaluator: `satisfied`, `unsatisfied`, or a value on a claim-declared named grade/measurement scale. There is no universal partial-satisfaction scalar. |
| **Metric definition** | An immutable, named, versioned, and fully configured algorithm definition over a declared population of compatible claim evaluations, with explicit input contract, scale, unit, aggregation, parameters, and missing/indeterminate-input policy. Applying it in an evaluation context is the invocation that produces a metric result. |
| **Metric result** | The exact output of one metric definition: `computed` with a value on its named scale, `not_applicable`, or `not_computable`, plus included/excluded inputs and a reproducible calculation trace. There is no unqualified “aggregate score.” |
| **Gate policy** | An immutable semantic decision rule over direct claim-evaluation predicates and named metric-result predicates, including condition precedence and treatment of indeterminacy. |
| **Semantic gate decision** | The non-mutating result `pass`, `fail`, `indeterminate`, or `not_applicable`, with every condition result and decisive reason. It answers the policy's bounded semantic question only. |
| **Operational Reliance Determination** | The separately owned, immutable action-domain result produced from one exact Operational Reliance Basis. It contains Current-Use Eligibility and Operational Disposition as two fixed facets for one occurrence, policy, action question, target, semantic time, and decisive accounting closure. It grants no action authority and performs no mutation. |
| **Disposition Reference** | A reference to one exact Operational Reliance Determination plus its exact Operational Disposition facet being displayed or documented. It is not a second result and cannot let the paired eligibility/disposition facets diverge. |

A final **evaluation context** is formed only after its exact canonical
evidence-set identity is fixed. It binds that evidence set; one contract
revision; subject snapshot; evaluation time; claim-evaluator versions; metric
definition identities and configurations; exact Evidence Evaluation Policy
bindings; gate-policy revision; and one exact Prospective Evaluation Basis,
whose identity transitively binds the Evaluation Authorization Binding and
pre-observation provenance. It also binds the Engine-produced authorization-
currentness basis required at final closure. Results that do not share that
closure cannot be combined.

Before observation, the Prospective Evaluation Basis MUST be closed without an
evidence-set identity. Producers and source observations, evidence candidates,
and admitted canonical evidence MUST retain its exact identity and the
authorization provenance it binds so another purpose, subject, scope,
objective, use, stack, request, requirements set, or readiness basis cannot be
substituted or transferred. Canonical evidence MUST NOT depend on the final
evaluation context that is formed only after the evidence set exists.

Engine alone issues the Evaluation Authorization Binding and owns the
underlying envelope admission, request-within-envelope decision, authority
admission, currentness or historical-validity determination, and supersession
semantics. Contracts MAY exact-check the binding's non-substitution and its
congruence with Contracts-owned context facts, including subject or scope,
purpose or intended use, exact stack semantics, and requested reach. Contracts
MUST NOT reconstruct, approximate, broaden, or replace any Engine decision
from coordinate values, rank comparisons, parent or constraint relations,
producer capability, Dock limits, or actual observations.

The Evaluation Authorization Binding identity MUST transitively preserve the
exact full-envelope and Engine authority/admission lineage without making the
full private envelope, credentials, witnesses, or Engine admission machinery
part of the Contracts domain. Sharing a serializable coordinate shape across
an Engine authorization bound, a request, a producer capability, a Dock or
adapter operating ceiling, an actual observation, or a claim requirement does
not make those facts the same semantic type or permit substitution among them.
Viewer disclosure authority, posture strictness, and explanation density are
separate again and do not change evaluation authorization or truth.

Missing, malformed, substituted, stale, superseded, or otherwise unverifiable
authorization before Prospective Evaluation Basis formation MUST prevent or
refuse producer launch and collection as required by the later lifecycle
decision. A failed required Engine verification at final evaluation-context
formation MUST refuse new claim, metric, or gate truth. Earlier launch-check
failures and any recovery follow the [post-commit verification rule](#post-commit-verification)
while ordinary collection remains open; no recovery may replace fixed inputs,
bypass an originally required continuity or terminal-state condition, or
retroactively authorize collection. This record does not select the readiness
or diagnostic state machine used to report those stops.

Every final context MUST preserve its complete requirements source and owner
closure, exact admitted contract and permitted use, and immutable run intent.
The direct branch retains its direct-resolution and nomination provenance; it
does not manufacture posture lineage. Switching sources, changing permitted
use through a new revision, or changing intent cannot reuse an old occurrence
or relabel its results, even when the selected claims and outcomes are equal.
Canonical explanations and projections must make the source, permitted use,
intent, and bounded meaning of a pass explicit. Evaluators MUST NOT branch on
contract permitted use to relax claim, evidence, metric, or gate semantics.

For the `posture_selected` source, the evaluation context additionally fixes
the Resolved Posture Snapshot fingerprint; exact Assurance Policy Selector
identity, version, semantic-content identity, and currentness; exact admitted
policy-resolution basis identity and currentness; exact selected Assurance
Policy identity, version, semantic-content identity, and currentness; and
Resolved Assurance Requirements identity. The requirements identity
transitively binds the admitted exception inputs, every other declared
selection input, and complete selection accounting. Those bindings participate
in evaluation fingerprinting, currentness, and canonical explanation. Metrics
and gates consume the resolved requirements; they never reread or reinterpret
mutable posture, reselect requirements, or change metric meaning. This is the
cross-boundary invariant fixed with the
[companion posture/assurance record](assurance-and-evaluation-preparation.md#posture-assurance).

Every evaluation context is admissible only as the exact and
complete realization of one Evaluation Requirements identity. That
identity selects one exact contract revision and one complete contract-rooted
expected-claim inventory. Any selected claim-set reference MUST be rooted in
that same contract revision. The evaluation subject snapshot MUST satisfy the
exact bound evaluation scope, including Posture Scope for `posture_selected`.
The evaluation's contract revision and permitted use, expected-claim
inventory, Evidence Evaluation Policy bindings, Metric Definitions and full
configurations, and Gate Policy MUST be exactly the selected outputs. Every
selected requirement and every metric or gate dependency MUST be closed and
completely accounted. An addition, omission, substitution, wrong-scope
binding, incompatible multiplicity, or unresolved dependency MUST refuse
before producing claim, metric, or semantic-gate truth; it MUST NOT become
ordinary `not_applicable`, `not_computable`, or `indeterminate` truth.

Prospective-basis closure, requirements-to-context conformance, and final
evaluation-context closure are preconditions at their respective stages. A
contract, scope, expected-claim, evaluator-context, selected-policy, Metric
Definition, or Gate Policy closure violation refuses and produces no claim
evaluation, metric result, or semantic gate decision. The same refusal applies
at the applicable provenance boundary when required authorization is missing,
malformed, substituted, bound to the wrong subject, scope, objective, use,
stack, or request, stale, superseded, or otherwise unverifiable by Engine.
These are Prospective Evaluation Basis or final evaluation-context closure
failures, not evidence insufficiency. `not_computable` and `indeterminate`
apply only after a valid final evaluation context has been established and the
declared evidence or claim state prevents computation or decision. Proven
`not_applicable` likewise remains available only inside a valid final context
under the positive-proof rules.

Requirements spanning unrelated contract revisions require separate Evaluation
Requirements closures and evaluation contexts. Cross-context gate composition
would require a separately defined semantic owner and remains deferred; it MUST
NOT be inferred from plural references.

Admission-time source validity is an immutable historical fact about the
candidate-to-record transition. Evaluation-time currentness is recomputed for
the evaluation time under the claim's freshness policy; admission never makes a
durable claim-currentness decision.

### Independent dimensions

The model must retain these dimensions independently even when one artifact
contains several of them:

| Dimension | Allowed form | Aggregation and compensation rule |
| --- | --- | --- |
| Observed satisfaction / finding | Categorical satisfaction or a claim-declared named grade/measurement scale. An established finding is distinct from an indeterminate determination. | Scalar only when the named scale defines one. It cannot be inferred from evidence count, confidence, criticality, or gate outcome. |
| Applicability | `applicable`, `not_applicable`, or `undetermined`. | Never scalar or averaged. Only proven `not_applicable` removes the claim from an applicable population. |
| Sufficiency / coverage | Categorical sufficiency or an exact ratio over an explicitly named population. | Coverage may aggregate only population-compatible counts. Insufficiency cannot be converted to partial satisfaction or silently removed from a denominator. |
| Currentness | `current`, `stale`, or `unknown` against an explicit evaluation time and policy. | Never compensatory. Stale/unknown input cannot establish a current finding. |
| Consistency / stability | Categorical consistency, conflict, or stability result under a named resolver or sampling model. | Never a generic pass percentage. Conflicts are indeterminate unless the claim evaluator names a deterministic resolver. |
| Uncertainty / confidence | A method-specific exact interval, error bound, calibrated probability, or categorical unknown. | No universal confidence scalar or multiplier. It cannot multiply satisfaction, weight, criticality, or numerator contribution. |
| Criticality / severity | A category in an explicitly named taxonomy: claim impact/criticality or observed finding severity, as applicable. | Not intrinsically ordered, weighted, or a gate effect. It becomes scalar or aggregated only through an explicit compatible named rule. |
| Decision | The categorical semantic gate decision. | Never numeric or averaged. A direct non-compensatory condition cannot be offset by a metric or another condition. |

Repeated observations affect consistency or a named sampling model only when
their lineage and required independence are established. Repetition and
duplicate submission never increase confidence by default.

### Measurement and metric semantics

Every metric definition must state all of the following; omission makes the
definition invalid rather than supplying a default:

1. a stable metric name and version, plus the exact algorithm identity and
   version;
2. its complete configuration, including every parameter, coefficient, and
   constant;
3. the exact eligible-population rule and compatibility contract, including
   the subject, context, evaluator, time, scale, and unit closure required of
   every input;
4. every input and output scale, its direction, permitted operations, and
   bounds;
5. the quantity dimension and canonical unit wherever a value or coefficient
   is dimensioned;
6. the exact aggregation algorithm and semantics;
7. handling for `not_applicable`, indeterminate, insufficient, stale,
   conflicting, uncertain, missing, or incompatible inputs; and
8. complete included/excluded-input and calculation accounting, including
   conversions, intermediate values, bound checks, and the reason for any
   non-computed result.

There is no universal unnamed aggregate, default metric, or concrete metric
algorithm catalog selected by this record. A metric result always identifies
the exact named, versioned, configured definition that produced it.

Externally stable numeric values are exact. Finite decimals retain their exact
coefficient and scale; ratios retain exact numerator and positive denominator;
quantities retain exact value and unit; named categorical or ordinal scales
retain their scale identity and direction. No producer, adapter, metric, or
transport may implicitly coerce an IEEE-754 value, round a boundary, infer a
unit, average an ordinal scale, or collapse a quantity to a unitless number.
Exact conversions must be named, versioned, dimension-compatible, and visible
in the trace.

Every metric algorithm must declare finite public and intermediate numeric
bounds appropriate to its own semantics and representation. An out-of-bound
input, intermediate, or result refuses instead of clamping, rounding, wrapping,
or changing representation silently. Threshold comparison is exact and
declares whether equality passes.

A weight or other coefficient exists only as a parameter inside one named
metric definition. It is not intrinsic to a claim and never means obligation,
criticality, severity, confidence, producer trust, or gate effect. The metric
algorithm must declare each coefficient's meaning, dimension and unit,
permitted sign and treatment of zero, relative or absolute interpretation,
equivalence and normalization behavior, bounds, and role in aggregation. There
is no universal coefficient domain or normalization rule across algorithms.

Only when an algorithm defines dimensionless, strictly positive, relative
weights may exact rational or finite-decimal coefficients be normalized to a
primitive positive-integer vector. That normalization is valid only when it
preserves the algorithm's semantics and every declared public bound. Zero,
negative, absolute, or dimensioned coefficients must never be silently
coerced, discarded, or normalized under this rule.

An Evidence Evaluation Policy may fill only a role declared by the selected
contract, claim, or evidence requirement and only when its exact identity and
semantics satisfy that role's compatibility constraints. It MUST NOT change
canonical-admission authority, provenance, identity, or another non-relaxable
trust invariant; the claim proposition, finding scale, direction, or unit; a
Metric Definition; or Gate Policy meaning. Missing, incompatible, conflicting,
substituted, or unaccounted policy bindings refuse rather than falling back,
overriding claim semantics, or becoming an implicit relaxation.

Evidence Evaluation Policy, Metric Definition, and Gate Policy candidates MUST
follow [shared policy admission](objects-and-governance.md#policy-admission).
Admission preserves every compatibility, numeric-meaning, and non-relaxable trust
constraint in this chapter; policy authorship does not change claim or gate
meaning.

### Responsibility map

All definition and policy admissions in this part, including Prospective
Applicability and Canonical Evidence Set Formation Policies, use the governing
configuration, owner-rule basis, and active-at-decision boundary established by
[shared policy admission](objects-and-governance.md#policy-admission). A successor
configuration does not rewrite earlier admissions, evidence membership,
applicability, or evaluation truth, and does not substitute new policy definitions
into a formed purpose or context. Historical admission, current usability, and
compliance with successor admission requirements remain distinct propositions;
the exact owner and consumer dependency/currentness rules govern present use.

| Layer | Owns | Must not decide |
| --- | --- | --- |
| Engine Context Resolution authorization | Exact envelope admission and non-expansion; issuance of the purpose-bounded Evaluation Authorization Binding; verification of its issuance lineage, underlying envelope, request-within-envelope relation, authority admission, currentness or historical validity, and supersession. | Assurance selection, producer capability or readiness, Contracts context congruence, evidence adequacy, claim truth, metrics, gates, explanation density, or downstream action. |
| Contracts-domain Evaluation Coordination | The proposed Contracts-domain coordination responsibility forms the exact pre-observation binding of purpose, subject/scope, objective/use, requested reach, Engine authorization, complete Evaluation Requirements/source/contract permitted use, and readiness/use-intent provenance. Concrete crate, module, API, persistence, and transport placement remain deferred. | Evidence-set identity, claim or metric truth, gate decision, Engine admission, requirements relaxation, or host enforcement by implication. |
| Validators and source providers | Bounded source-specific execution, source observation, source snapshot identity, native values/units/scales, and collection diagnostics. | Applicability, claim findings, canonical admission, metric weights/values, gate policy, or downstream action. |
| Docks and adapters | Protocol execution/framing, exact translation, preservation of source semantics and lexemes, exact Prospective Evaluation Basis provenance, producer operating-ceiling enforcement, candidate construction, and import diagnostics. | Engine authorization; treating request/ceiling/actual equality as authority proof; canonical evidence; invented units or scale direction; silent normalization; claim meaning; applicability; metrics; or gates. |
| Contracts evaluation-context closure | Final binding of the exact canonical evidence set, Prospective Evaluation Basis, Engine-produced authorization-currentness result, and all contract/policy/evaluator/metric/gate/posture provenance; refusal before semantic truth when closure or required Engine verification is absent or negative. | Forming the pre-observation basis, issuing authorization, or reconstructing Engine rank, parent, constraint, mutation, escalation, authority-admission, currentness, or supersession semantics. |
| Contracts evidence admission | Exact candidate/source/run/subject and Prospective Evaluation Basis binding; authority and provenance validation; canonical normalization; unit/scale identity and permitted exact conversion; idempotent duplicate identity; immutable canonical evidence. | Final evidence-bound evaluation-context closure, claim satisfaction, criticality, cross-claim aggregation, gate outcome, or workflow action. |
| Contracts-owned Evidence Evaluation Policy authority | Canonical establishment, exact re-admission validation, and exact resolution of immutable named/versioned role-specific policies; declared role and contract/claim compatibility; and the policy's bounded evidence-evaluation semantics. | Canonical-admission trust, claim proposition or finding-scale changes, assurance selection, metric or gate meaning, downstream disposition, or mutation. |
| Claim evaluation | Applicability; exact application of the selected compatible Evidence Evaluation Policy bindings; complete evidence selection/accounting; sufficiency, currentness, consistency, uncertainty treatment; semantic interpretation; and one finding or indeterminate determination per expected claim. | Policy invention or fallback, cross-claim metric aggregation, global thresholds, gate precedence, enforcement, posture, promotion, or mutation. |
| Contracts-owned named metric-definition authority | Canonical establishment, exact re-admission validation, and exact resolution of immutable named, versioned, configured metric definitions; validation of the eligible-population and compatibility rules, input/output scales and units, algorithm identity/version, parameters and coefficients under their algorithm-declared semantics and bounds, aggregation semantics, and missing/indeterminate-input policy. | Evidence selection, finding interpretation or establishment, the actual evaluation-run population, metric computation, threshold/comparator application, gate decision, or downstream mutation. |
| Metric computation | Pure exact application of one exactly resolved named metric definition to compatible claim evaluations in one closed evaluation context, including derivation and accounting of the actual included/excluded population and calculation result. | Raw-source/evidence selection, applicability or finding changes, definition changes, gate threshold/comparator application, gate decision, or downstream disposition. |
| Gate policy | Direct non-compensatory claim-evaluation predicates, named metric-result predicates, precedence, indeterminacy treatment, and the semantic gate decision. | Raw evidence interpretation, unit/scale conversion, changing metric semantics, canonical admission, or any workflow/posture/promotion mutation. |
| Work Progression and other Operational Reliance owners | Resolve exact historical results and Evaluation Authorization Binding validity; apply one admitted action-specific Operational Reliance Policy with explicit `historical_validity_only` or `continuing_current` Engine dependency mode; close exact target/currentness/accounting within the mandatory governing-completion selection and assessment boundary where applicable; and produce one atomic Operational Reliance Determination with paired eligibility/disposition facets or a Reliance Refusal. Work Progression is the proposed semantic owner for local-closeout and parent eligibility. | Rewriting the semantic gate or canonical explanation, treating a metric as the decision, issuing action authority, mutating state, or silently selecting work hierarchy/state policy or physical placement. |
| Action requester and action-authorization owner | The requester owns one exact Action Intent after `eligible / allow`. The exact action-authorization owner independently produces an Action Authorization Binding or Action Authorization Refusal with the declared advisory, verified, or host-enforced authority provenance and fresh conditions. | Treating Evaluation Authorization, a gate, `allow`, a prompt, ambient OS permissions, or artifact approval as independently authenticated identity, verified delegation, host-enforced permission, or a mandate for a general Handbook IAM platform. |
| Action executor and authoritative state/effect owner | The executor owns one exact Action Attempt Identity and Action Terminal. The authoritative state/effect owner performs attempt- and commit-bound expected-state, effect, authorization, and declared currentness checks and alone issues the Mutation Receipt for a committed change. | Ordinary retry after uncertain commit, broadening the authorized effect, inferring commit from an executor report, or reporting mutation success without the authoritative receipt. |
| SDK, CLI, Docks, hosts, and other transports | Preserve exact owner results and provenance, including policy-permitted prompt- or configured-role-supplied advisory claims. A real enforcing host may supply stronger independently verified facts. | Becoming a semantic owner by transport; fabricating authentication, delegation, or host enforcement; waiving target/currentness checks; or satisfying a policy that requires enforcement with advisory claims. |

### Gate and explanation invariants

- A direct claim-evaluation predicate is a non-compensatory predicate over
  exactly one claim evaluation, including its applicability, determination,
  finding, sufficiency, currentness, consistency, or uncertainty dimensions. A
  named metric-result predicate is the separate policy construct over one exact
  metric result. A direct predicate is never encoded only as a large weight.
- Metric results do not compensate for one another unless the exact named gate
  policy explicitly declares a compatible composite rule. There is no ambient
  averaging across metrics.
- A gate decision is pure derived semantic truth. It persists or mutates no
  lifecycle, work unit, posture, parent, promotion, release, or deployment
  state and grants no authority to do so.
- Producers, source observations, evidence candidates, and canonical evidence
  MUST retain the exact Prospective Evaluation Basis and its authorization
  provenance without depending on a final evidence-bound evaluation context.
  Claim evaluations, metric results, the semantic gate decision, and canonical
  explanation then bind the exact Evaluation Authorization Binding
  transitively through their one final closed evaluation context and its exact
  Prospective Evaluation Basis. No layer may copy selective coordinate values
  and thereby detach truth from Engine-owned authorization provenance. An
  authorization-related refusal MUST retain any available decisive provenance
  and the exact reason without fabricating claim, metric, or gate truth.
- A later authority change, revocation, supersession, stack change, or approved
  escalation MUST NOT rewrite a closed result or explanation. That result
  remains immutable historical truth for its exact closure. Every present
  Operational Reliance Basis MUST resolve the exact Evaluation Authorization
  Binding and Engine-confirmed historical validity. Its admitted Operational
  Reliance Policy MUST additionally select exactly one continuing-status mode:
  `historical_validity_only` or `continuing_current`. Only the latter requires
  fresh positive Engine current verification at the reliance semantic time and,
  when that policy makes the changed proposition decisive, a fresh evaluation.
  Fresh action-specific authorization and target verification remain separate
  and universal before any later Action Attempt; neither reliance mode weakens
  the strict collection-time or final-evaluation-closure checks above.
- Producer capability, a Dock or adapter operating ceiling, equality among a
  request, operating ceiling, and actual observation, per-claim evidence
  adequacy, viewer disclosure authority, posture strictness, and explanation
  density are independently owned facts. None establishes or replaces Engine
  evaluation authorization.
- The four semantic gate outcomes have fixed meanings: `pass` means the policy
  applies and establishes its bounded proposition; `fail` means it applies and
  establishes its negation or a policy-declared fail-closed violation;
  `indeterminate` means it applies but cannot establish pass or fail under its
  declared treatment; and `not_applicable` means positive semantic proof
  establishes that the policy's governed population or question does not
  apply. Omission, missing input, and inability to compute never produce
  `not_applicable`.
- Complete claim accounting is mandatory. Every expected claim appears exactly
  once with one applicability result. Every `applicable` claim then appears
  exactly once with either one established finding or one indeterminate
  determination. An applicability result of `undetermined` establishes no
  finding. Omission is invalid.
- **Evidence degradation** means a change that, while subject, applicability,
  policy, and all other eligible information remain fixed, removes or weakens
  eligible information supporting the previous decision or adds unresolved
  adverse, conflicting, stale, or uncertain eligible information. Such
  degradation cannot change `fail` or `indeterminate` to `pass`. When it
  destroys the sole valid basis for a prior `pass`, the re-evaluated decision
  cannot remain `pass`; the changed accounting must be visible. Any ordering of
  downstream dispositions belongs to the downstream consumer's policy.
- The full canonical explanation is Contracts-owned canonical decision data,
  not reconstructed prose or a density-specific reevaluation. It preserves
  expected claim inventory; applicability basis; every included, excluded,
  disqualified, stale, conflicting, and duplicate evidence binding; finding and
  assessment dimensions; each metric definition, population, value/state,
  conversion, and calculation; every gate condition and precedence step; and
  the decisive reason/accounting chain. The default human view is concise, and
  a requested deeper drill-down reaches the highest source detail both
  available and independently authorized for that viewer, with omissions made
  explicit. Disclosure authority and source availability are independent from
  truth. No density view may change inputs, rerun evaluation, or become another
  gate. The number, names, transport shapes, and interaction design of those
  projections remain bounded later decisions; this record selects no fixed
  level count, UI layout, or API.

### Discriminating scenario corpus

Each scenario assumes exact identity/provenance and the stated immutable policy
unless the row identifies their failure.

One scenario uses the hypothetical metric definition
`scenario.decimal-coefficient-seam@1` and algorithm
`scenario.exact-relative-weighted-binary-ratio@1`. Its configured population is
exactly three named claims after exclusion of any claim proved
`not_applicable`; compatible inputs must be binary findings from the same
closed evaluation context. Its output is an exact dimensionless rational in
`0..=1`, higher is better; the algorithm divides the sum of coefficients for
satisfied eligible inputs by the sum for all eligible inputs. Its configured
coefficients are exact, dimensionless, strictly positive, relative finite
decimals `0.25`, `1.50`, and `2.25`. For this definition, multiplication by a
common positive factor is equivalent, primitive-positive-integer normalization
is permitted, and the configured normalized components and every intermediate
sum are bounded by `16`. Any eligible missing, indeterminate, stale,
conflicting, uncertain, or incompatible input makes the result
`not_computable`; an empty eligible population is `not_applicable`. Its trace
accounts for every input, exclusion, coefficient, normalized value, sum, and
exact division. This scenario-local definition tests seam semantics; it does
not authorize a product metric catalog entry.

| Scenario and discriminating inputs | Responsible layers | Expected semantic outcome | Invariant proved |
| --- | --- | --- | --- |
| **Hard failure with high aggregate.** One required security claim is established unsatisfied; a named, versioned, configured aggregate metric reports `99/100`. The gate requires that claim satisfied and separately requires that metric at least `95/100`; direct failure is decisive. | Claim evaluator; metric; gate policy. | Metric remains `99/100`; the direct predicate fails; gate decision is `fail`. | A compensatory aggregate cannot override a direct predicate. |
| **Missing versus not applicable.** Claim A is applicable but has no admitted record; Claim B has a proved false applicability rule. The gate requires each applicable required claim determinate and treats an indeterminate direct predicate or dependent `not_computable` metric as `indeterminate` when no fail is established. | Applicability and claim evaluators; metric; gate. | A has insufficient coverage and no finding; its determination is indeterminate. B is `not_applicable`. The dependent metric is `not_computable`; gate is `indeterminate`, not pass. | Absence, omission, and N/A are distinct. |
| **Stale evidence.** The only otherwise satisfying record exceeds its freshness policy at the explicit evaluation time. The gate requires the claim satisfied and treats an indeterminate direct predicate or dependent `not_computable` metric as `indeterminate`. | Claim evaluator; metric; gate. | Currentness is stale; no finding is established; claim determination is indeterminate; dependent metric is `not_computable`; gate is `indeterminate`. | Historical satisfaction is not current satisfaction. |
| **Conflicting evidence.** Two eligible authorized records support opposite findings, no resolver is named, and the gate treats an indeterminate direct predicate or dependent `not_computable` metric as `indeterminate`. | Claim evaluator; metric; gate. | Consistency is conflicting; no finding is established; claim determination is indeterminate; dependent metric is `not_computable`; gate is `indeterminate`. | Neither adapter nor metric may cherry-pick. |
| **Duplicate evidence.** The same candidate identity and bytes are submitted twice and the same closed evaluation context is re-evaluated. | Admission; claim evaluator; metric; gate. | Admission is idempotent to one canonical evidence identity; the finding, metric result, semantic gate decision, canonical identities, and accounting are unchanged. A same-identity/different-bytes submission refuses. | Duplication cannot increase weight or confidence. |
| **Flaky observation.** Ten distinct eligible observations contain nine satisfied and one unsatisfied result; the claim declares unanimity as its stability resolver and declares no sampling model. | Claim evaluator. | Consistency is conflicting; no finding is established; claim determination is indeterminate; no generic “90% satisfied” value is produced. | Repetition needs a named sampling model before scalar aggregation. |
| **Quantitative unit normalization.** Authorized observations report `1 s` and `1000 ms` for the same latency quantity, and admission permits one named exact conversion. | Admission; claim evaluator; metric. | Both normalize to the same exact canonical quantity and produce the same finding and metric value with source units retained in provenance. | Units are semantic and exact, not labels. |
| **Graded external scale.** An imported grade is `1` on a versioned ordinal scale where lower is better; a named claim evaluator maps it to a declared grade. | Adapter; admission; claim evaluator; compatible metric. | Direction is honored from the exact scale definition; a binary metric rejects the grade; only a metric naming that scale/mapping may consume it. | External numbers do not imply direction, binary truth, or weights. |
| **Decimal relative coefficients.** The scenario-local `scenario.decimal-coefficient-seam@1` definition receives findings satisfied, unsatisfied, satisfied. A single-condition gate requires that exact metric result to be at least `5/8`, with equality included. | Metric-definition authority; metric computation; gate policy. | The permitted normalization produces `1, 6, 9`; the calculation trace records numerator `10`, denominator `16`, and exact result `10/16 = 5/8`; the gate passes on equality. | Coefficient meaning, conditional normalization, computation, bounds, and threshold remain exact and separately owned without selecting a catalog algorithm. |
| **All not applicable.** Every claim in the gate's governed population is proved `not_applicable`; the named metric's definition declares an empty eligible population `not_applicable`; the gate policy declares that positive proof for the complete population makes its question not applicable; and an admitted action-specific Operational Reliance Policy maps that historical result into one atomic determination. | Applicability evaluator; metric; gate; Operational Reliance owner. | Metric and gate are both `not_applicable`, never `pass`; the Operational Reliance Determination has `not_applicable / no_effect` facets that share one identity and accounting closure. | Empty populations do not pass vacuously, and eligibility/disposition cannot diverge. |
| **Degradation monotonicity.** A prior `pass` depends on one current satisfying record. At a later evaluation time the same subject snapshot, applicability, policy, and other eligible information are unchanged, but that record is now stale; the gate treats the resulting indeterminate direct predicate as `indeterminate`. | Claim evaluator; metric; gate. | No finding is established; the dependent metric is `not_computable`; the re-evaluated gate is `indeterminate`, not `pass`, and changed accounting identifies the stale record. | Loss of the sole eligible basis for pass cannot preserve or improve the gate decision. |
| **Mixed population closure.** Candidate claim evaluations are presented as one metric population but differ in subject snapshot, contract revision, evaluator version, or evaluation time. | Evaluation-context admission; metric; gate. | The mixed closure refuses before metric or semantic-gate truth. It does not produce a `not_computable` metric or `indeterminate` gate. | Context substitution or construction failure is not a semantic result. |
| **Multiple named metrics.** A gate requires both a named functional-metric threshold and a named security-metric threshold. The first predicate passes and the second fails; failure is decisive. | Two metrics; gate policy. | Results remain separate; the security metric predicate is decisive and the gate is `fail`. No hidden cross-metric average exists. | One metric cannot silently compensate for another. |
| **Uncertain boundary.** A latency interval straddles the claim budget, no alternate evaluator is named, and the gate requires that claim satisfied while treating an indeterminate direct predicate as `indeterminate`. | Claim evaluator; dependent metric; gate. | Uncertainty is preserved; no finding is established; claim determination is indeterminate; dependent metric is `not_computable`; gate is `indeterminate`. | Midpoints or favorable endpoints cannot be selected silently. |
| **Unauthorized source.** An otherwise well-formed candidate binds a source actor that lacks current authority for the stated subject and observation class. | Contracts evidence admission. | Admission refuses before claim evaluation and produces no canonical evidence record, finding, metric result, or gate decision. | Structural validity and adapter output cannot substitute for source authority. |
| **Coordinates without authorization.** A prospective run supplies a valid exact stack and six requested coordinate values but no Evaluation Authorization Binding. Comparison cases reuse a real binding for another subject, scope, objective, use, parent or constraint closure, or pair identical coordinate labels with a changed stack fingerprint. | Engine authorization; prospective evaluation formation. | Prospective Evaluation Basis formation refuses, so no producer is launched and no observation is collected. The comparison cases require a fresh, congruent binding; matching coordinate labels do not establish compatibility or transferability. | Requested coordinates and structural equality are not Engine authorization provenance. |
| **Dock equality without Engine authority.** A proposed Dock request, admitted operating ceiling, and expected actual Resolution are exactly equal, but there is no verifiable Evaluation Authorization Binding. | Dock; Engine authorization; prospective evaluation formation. | Dock equality cannot close a Prospective Evaluation Basis, so the producer MUST NOT be launched under that proposal and no candidate or semantic truth is produced. | A producer operating contract neither grants nor proves Engine authority. |
| **Authorization changes across stages.** In one case Engine reports the binding stale or superseded before launch. In a second, canonical evidence exists, but the required final-closure verification reports the binding superseded and a successor Binding is required. In a third, a `pass` was already closed before authorization later became non-current; one Operational Reliance Policy declares `historical_validity_only` and another declares `continuing_current`. | Engine authorization; Evaluation Coordination; Contracts evaluation-context closure; Operational Reliance and action owners. | The first prevents collection. The second preserves historical evidence but refuses new claim, metric, or gate truth; use of the successor requires a new occurrence and fresh authorized basis/context, without adopting the old collection. The third preserves the immutable historical `pass` and canonical explanation. Both reliance paths resolve the exact Binding and historical validity; only `continuing_current` requires fresh positive Engine current verification and any policy-required fresh evaluation. Any effect under either path still requires a distinct fresh Action Authorization Binding, expected-target verification, exact attempt, Terminal, and state-owner receipt. | Collection authorization, final-truth authorization, historical validity, policy-selected continuing-current reliance, and fresh action authorization are distinct propositions. |
| **Approved Context Resolution escalation.** Engine approves a changed envelope after an earlier Prospective Evaluation Basis or final evaluation context was formed. | Engine authorization; separately owned readiness; prospective evaluation formation; Contracts evaluation-context closure. | The approved envelope requires a fresh Evaluation Authorization Binding, readiness determination, Prospective Evaluation Basis, and final evaluation context. Prior evidence and results remain bound to their earlier bases and contexts and are not upgraded. | Escalation is not in-place expansion or retroactive authorization. |
| **Exact resolved-requirements realization.** One Resolved Assurance Requirements identity for scope `S` selects contract-rooted expected-claim inventory `C`, exact Evidence Evaluation Policy bindings `E`, configured Metric Definitions `M`, and Gate Policy `G`. The valid evaluation uses a subject snapshot inside `S`, one exact policy in each supported required role, and exactly `C/E/M/G`. Comparison cases use a subject outside `S`; substitute an incompatible policy; bind an exact policy to an unsupported role; supply conflicting bindings for one role; change a metric configuration; omit one selected claim; add an unselected claim; or let `G` depend on an unselected claim or metric. | Assurance selection; evaluation-context admission; claim evaluation; metrics; gate. | The exact valid closure may evaluate reproducibly. Every comparison refuses before claim, metric, or semantic-gate truth; none falls back or becomes ordinary `not_applicable`, `not_computable`, or `indeterminate`. | The evaluation context must exactly and completely realize one selected requirements identity. |

### Settled versus deferred

The all-not-applicable scenario also applies after the complete-pruning lifecycle
when its exact entry, authorization, commitment/frontier, empty evidence closure,
and final-proof conditions hold. Its named metric/gate/reliance policies still
determine that outcome; the lifecycle itself supplies no numeric value, satisfied
finding, pass, or completion permission.

This record settles the vocabulary and relationships, independent dimensions,
exact measurement rules and bounds, weight ownership, Evidence Evaluation
Policy roles and compatibility, the one-contract-rooted
requirements-to-evaluation conformance rule for both sources, exact contract
permitted-use/intent provenance, responsibility map, gate purity
and precedence, exact Engine-owned evaluation-authorization provenance and
context-refusal boundary, the pre-observation Prospective Evaluation Basis and
later evidence-bound evaluation-context closure, universal historical Binding
resolution/validity plus the explicit `historical_validity_only` /
`continuing_current` present-reliance rule, separate fresh action authorization,
Policy Admission Closure requirements, canonical explanation truth and density-
projection principles, complete explanation/accounting requirement, and every
scenario outcome above. Later work may not reopen those decisions merely by
choosing a representation.

Deliberately deferred to human-approved task-level planning are all concrete
metric algorithms and catalog entries; schema, type, API, and operation shapes;
expression language; persistence layout; registry and version-resolution
mechanics; any cutover or temporary internal bridge; implementation packets;
acceptance planning; and rollout. Also out of scope are external adopter
selection, concrete local-closeout or parent-promotion policy, work hierarchy
and state model, posture algorithms, UI, a metric marketplace, and a universal
confidence score.

The proposed
[evaluation authorization, capability, and readiness decision](assurance-and-evaluation-preparation.md#evaluation-preparation)
owns evaluation-purpose formation, authorization issuance, Evaluation
Coordination, Prospective Evaluation Basis lifecycle, currentness checkpoints,
readiness, capability/ceiling separation, diagnostics, and retry/cache/
supersession/escalation semantics. The proposed
[operational-reliance and work-progression decision](operational-reliance-and-actions.md#reliance-actions)
owns the common present-use rule, atomic eligibility/disposition determination,
separate action chain, and Work Progression semantic responsibility for local-
closeout and parent eligibility. Concrete physical placement, work hierarchy and
state model, admitted policies, schemas, storage, transport, host/Substrate
mechanism, mutations, implementation, and delivery slices remain deferred. The
proposal does not silently approve any of them or require a general identity,
credential, authentication, delegation, capability-token, secrets, PKI, RBAC,
or ABAC platform.
