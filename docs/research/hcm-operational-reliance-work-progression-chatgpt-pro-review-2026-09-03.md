# HCM operational reliance and work progression — ChatGPT Pro review

> Advisory record only. ChatGPT Pro did not have live repository access. Repository sources attached to the consultation remain authoritative; this transcript records external review input and the local reconciliation applied afterward.

- Date: 2026-09-03
- Model: GPT-5.6 Sol
- Thinking effort: Pro, 5/5
- Chat: https://chatgpt.com/c/6a992cfa-a5f8-83ea-883f-81466c0a7396
- Candidate initial SHA-256: `5B1817B059EF6A1208A6718ED1E201B31A3E85729E251AF2A423E5B96C712823`

## Bounded review bundle

- `docs/specs/handbook-contract-membrane/operational-reliance-and-work-progression-semantic-decision.md`
- `docs/specs/handbook-contract-membrane/canonical-object-ontology-and-ownership-semantic-decision.md`
- `docs/specs/handbook-contract-membrane/evaluation-identity-evidence-closure-semantic-decision.md`
- `docs/specs/handbook-contract-membrane/evaluation-authorization-capability-and-readiness-semantic-decision.md`
- `docs/specs/handbook-contract-membrane/authored-artifact-evaluation-relationships-semantic-decision.md`
- `docs/specs/handbook-contract-membrane/metric-gate-semantic-decision.md`
- `docs/specs/handbook-contract-membrane/posture-assurance-policy-semantic-decision.md`
- `docs/specs/handbook-contract-membrane/architecture-context.md`
- `docs/specs/handbook-contract-membrane/phase-and-slice-map.md`
- `docs/specs/handbook-contract-membrane/slices/HCM-5.2/README.md`

## Prompt

```text
We are already working on this project task.

Task/spec:
Review the attached proposed HCM semantic decision, operational-reliance-and-work-progression-semantic-decision.md, as a fresh independent architecture/domain-model review. This is a greenfield product. The record is semantic authority for later human-approved planning only, not implementation authority, not a migration plan, and not a request to preserve LocalCloseoutResultV1 or other V1 compatibility.

Current approach:
The candidate fixes the seam after immutable evaluation/claim/metric/semantic-gate truth:
1. A downstream action domain chooses one exact Action Kind and target.
2. An admitted Operational Reliance Policy forms an action-specific, occurrence-bound Operational Reliance Basis over exact historical results, present semantic time, exact current target, declared currentness facts, and complete accounting.
3. It yields pure Current-Use Eligibility (eligible/ineligible/indeterminate/not_applicable) and paired Operational Disposition (allow/block/hold/no_effect).
4. allow only permits forming an Action Intent and requesting a separately issued Action Authorization Binding.
5. Immediately before action, authorization and expected target state are reverified; the state owner alone mutates and emits an Action Terminal and, for a committed change, Mutation Receipt.
6. Work Progression owns local-closeout and parent-eligibility semantics. Local closeout is not a gate field and implies no parent proposition. Parent eligibility independently closes the exact parent/child/dependency graph and implies neither promotion nor mutation.
7. Work Specification remains authored planning truth and can only nominate inputs or later reference exact results/receipts.
8. Gate-to-posture feedback must pass through posture-observation admission, recommendation, and separately authorized Engine transition.

The deliberately selected currentness rule is asymmetric:
- Every action domain must resolve exact historical validity, current reliance policy, current target snapshot, result-to-target congruence, all policy-declared present-use verifications, and complete accounting.
- Continuing-current Engine Evaluation Authorization Binding is policy-conditional, not universal. It is required only when the exact admitted reliance policy declares it as a present dependency. Evaluation authorization remains historical provenance otherwise.
- Fresh action authorization and final expected-target verification are universal immediately before every action or mutation and cannot be policy-disabled.

Repository/source constraints:
- metric-gate-semantic-decision.md fixes a pure, non-mutating semantic gate; direct claim predicates and metrics remain distinct; a gate does not own closeout, promotion, enforcement, or posture.
- evaluation-authorization-capability-and-readiness-semantic-decision.md distinguishes historical evaluation authorization from operational reliance but currently contains broader wording that fresh Engine authorization verification is included before operational reliance. This candidate intentionally refines that as policy-conditional; if sound, later normalization must amend the broader wording.
- evaluation-identity-evidence-closure-semantic-decision.md fixes immutable occurrence/evidence/result chains, historical resolvability, and explicitly defers present-use eligibility to this action domain.
- authored-artifact-evaluation-relationships-semantic-decision.md fixes Work Specification as an authored feature/slice plan without evaluation or action authority, and requires exact causal reference direction.
- canonical-object-ontology-and-ownership-semantic-decision.md classifies results/eligibility/action receipts as owner-produced records rather than authored artifacts.
- posture-assurance-policy-semantic-decision.md keeps posture observation, recommendation, approval, and Engine transition separate.
- architecture-context.md and phase-and-slice-map.md say HCM-5.2 intentionally landed only a pure sibling local-closeout result; parent eligibility requires a future exact parent model, binding, dependency/workflow owner, policy, currentness, and consumer, and parent mutation remains separate.
- HCM-5.2 README is implementation history/source evidence only. The product is greenfield and the candidate explicitly does not preserve its V1 owner/shape as future semantic authority.

Constraints:
- Do not add V1 migration, aliases, bridges, dual behavior, or compatibility.
- Do not collapse authored artifacts, Contracts results, Engine authority, Work Progression, action authorization, state ownership, or posture transition.
- Do not invent a universal derived-artifact repository or new authored artifact family.
- Do not choose concrete policies, thresholds, work hierarchy/state machines, fields, schemas, APIs, storage, crates, UI, task slices, or implementation decomposition.
- P1/P2 findings require revision. Recommend P3 only where it prevents materially incompatible downstream semantics; defer stylistic or implementation detail as P4.
- Review the complete candidate and all attached sources, not this summary alone.
- Treat attached repository sources as authority over generic assumptions. Distinguish repo-truth conflicts from design preferences.

Please return:
1. Executive verdict: KEEP, ADJUST, or PIVOT.
2. P1 correctness defects: contradictions, authority transfer, causal cycles, silent truth changes, unsafe action/mutation paths, or errors in the policy-conditional Engine rule.
3. P2 missing semantics likely to produce incompatible implementations.
4. P3 clarifications worth landing now and P4 items that should remain deferred.
5. An adversarial audit of:
   - Historical Semantic Result vs Operational Reliance vs Current-Use Eligibility vs Operational Disposition vs Action Authorization vs Action Terminal/Mutation Receipt;
   - whether eligibility and the fixed disposition mapping are both necessary and non-redundant;
   - whether Work Progression is the correct semantic owner and whether it is too broad or too narrow;
   - local closeout vs parent eligibility vs parent promotion/action;
   - complete parent/child/dependency population and currentness accounting;
   - Work Specification input/outcome reference direction;
   - diagnostic provenance;
   - unavailable historical source resolution;
   - retries, concurrent target changes, uncertain commits, and replay;
   - gate-to-posture feedback and same-occurrence cycles.
6. A dedicated decision analysis of universal vs policy-declared continuing-current Engine evaluation authorization. Stress-test low-risk local closeout, high-risk deployment, remediation, negative/blocking reliance, purely advisory prioritization, posture-observation admission, and action after authorization revocation. State whether the selected rule is safe, coherent, and composable, and give a stronger alternative if not.
7. A stage-by-stage responsibility and authority audit from historical result resolution through mutation receipt.
8. For every recommended change, exact normative wording plus insertion/replacement location.
9. Whether the amended candidate would be ready as human-reviewable semantic authority and whether any companion document must be amended during later normalization.

Do not reduce this to a summary. Provide a deep, self-contained review.
```

## Response

```text
Executive verdict: ADJUST

The candidate has the correct architectural center of gravity and does not need a pivot. It preserves the most important separations:

historical evaluation truth is not present permission;

present-use eligibility is not actor authority;

actor authority is not execution;

execution is not proof of committed state change;

local closeout is not a gate field;

parent eligibility is not inherited from child eligibility and is not promotion;

Work Specification remains authored planning truth;

gate-to-posture feedback cannot mutate or reselect the posture governing the originating evaluation.

Those boundaries are consistent with the companion ontology, metric/gate, authored-artifact, posture, architecture, and phase records. The candidate also correctly treats the current Contracts-owned LocalCloseoutResultV1 as implementation history rather than greenfield semantic authority. 

operational-reliance-and-work-p…

 

architecture-context

 

README

It is not ready as written, however. I find three P1 correctness defects and seven P2 omissions or ambiguities. They are bounded corrections rather than reasons to redesign the whole model.

The policy-conditional continuing-current Engine rule is substantively sound. Universal continuing-current evaluation authorization would conflate permission to perform a historical evaluation with permission to perform every later action. The candidate nevertheless needs a stronger explicit-mode rule, universal historical-validity resolution, and a clear statement that a later action-authorization owner may independently require current Engine facts.

P1 correctness defects
P1-1 — The reliance result does not bind the exact requested effect

The Operational Reliance Basis binds Action Kind and Action Target Snapshot, but the exact requested effect and effect scope first appear in the later Action Intent. The action section then says a changed effect or bounded scope requires only a fresh Action Intent and authorization, not a fresh Operational Reliance Basis and eligibility determination. 

operational-reliance-and-work-p… +1

That permits an unsafe semantic expansion:

eligible / allow for:
  deploy canary C to environment E

reused for:
  deploy full release R to environment E

An implementation could argue that both requests share the same versioned deployment Action Kind and target snapshot, while the effect variation belongs only to Action Intent. The later authorization owner could then authorize an effect that the reliance policy never evaluated.

The model needs an exact action question before reliance formation:

Action Kind
+ Action Target Snapshot
+ bounded effect
+ effect scope
+ every action parameter capable of changing eligibility

Action Intent must be exactly congruent with that question. Authorization may add stricter conditions but may not broaden or substitute it. Any eligibility-relevant effect or scope change must cause a new reliance occurrence.

This is an unsafe action path and requires revision.

P1-2 — A non-idempotent retry remains possible after an uncertain commit

The retry rule allows a non-idempotent retry when authorization and action policy permit it and bound dependencies remain exact/current. It separately says an uncertain commit must not be reported as success, but it does not require the uncertainty to be resolved before another attempt. 

operational-reliance-and-work-p…

That is insufficient. A timeout or lost response can leave this state:

first attempt may have committed
no Mutation Receipt is currently resolvable
target presentation still appears unchanged
policy permits retry

A second non-idempotent attempt may duplicate an external effect, repeat a mutation, or act on an ABA-equivalent target state. Recomputing eligibility cannot establish that the first attempt did not commit.

After an uncertain effect or commit, an ordinary non-idempotent retry must be prohibited until either:

the authoritative effect/state owner positively establishes non-application or non-commit; or

a separately admitted recovery rule and fresh authorization explicitly account for possible prior application and prove duplicate-safe recovery.

This is an unsafe mutation path and requires revision.

P1-3 — The “eligible but unauthorized” scenario creates an impossible Action Terminal

The vocabulary defines Action Terminal as an account of an action under one Action Authorization Binding. The scenario corpus then says that when the authorization owner refuses the actor or scope, “the action has an exact refusal terminal.” No Binding exists in that case. 

operational-reliance-and-work-p… +1

That violates the candidate’s own causal order:

Action Intent
  -> authorization refusal
  -> no Action Authorization Binding
  -> therefore no authorized Action Attempt
  -> therefore no Action Terminal under a Binding

The model needs a distinct Action Authorization Refusal. A pre-attempt authorization refusal creates no Binding, Action Attempt, Action Terminal, or Mutation Receipt.

An Action Terminal may record a refusal only after a Binding exists and the execution-attempt boundary has been entered—for example, when the final target or authorization-currentness check fails.

P2 missing semantics likely to produce incompatible implementations
P2-1 — Action attempt identity, authorization provenance, and terminal ownership are incomplete

The candidate distinguishes Action Intent, Authorization Binding, Action Terminal, and Mutation Receipt, but it has no exact identity for an individual Action Attempt. It nevertheless permits a non-idempotent retry to be “a new action attempt.” Without an attempt identity, implementations cannot consistently distinguish:

delivery replay from re-execution;

one authorized attempt from a second attempt;

one terminal per attempt from multiple terminals under one Binding;

a retry permitted by the same Binding from a retry requiring a new Binding;

an executor refusal from an authorization refusal.

The authorization object is also less closed than the Engine Evaluation Authorization Binding. It binds actor, intent, target, reliance result, and conditions, but does not expressly bind an exact owner-recognized authorization rule or authority basis and owner-verifiable issuance lineage. The evaluation companion requires this kind of exact provenance for Engine authorization; the action seam needs the same semantic integrity without copying Engine’s private implementation. 

evaluation-authorization-capabi…

Finally, Action Terminal ownership is ambiguous. The vocabulary says “action-owner or executor-owned,” while the responsibility map combines “authoritative state owner and action executor.” The ontology requires each proposition to have one exact semantic owner and requires a composite to preserve separate constituent owners. 

canonical-object-ontology-and-o…

The correct separation is:

Action Authorization Refusal/Binding: authorization owner.

Action Attempt Identity and Action Terminal: executor, as durable attempt provenance.

Mutation Receipt and committed successor state: authoritative state owner.

A terminal may assert successful state mutation only by binding the exact state-owner receipt.

Physical colocation remains permitted, but semantic ownership must stay separately attributable.

P2-2 — The historical-result candidate population is not closed precisely enough

The policy declares accepted result kinds, cardinality, composition, currentness modes, and complete accounting. The Basis records included, excluded, missing, and conflicting inputs. It does not, however, define an exact finite eligible source-result boundary from which completeness is established. 

operational-reliance-and-work-p…

Without that boundary, implementations can disagree about whether reliance uses:

exactly the results explicitly nominated by a Work Specification;

the latest result for a target;

every result for a target;

every result from one evaluation occurrence;

one result selected by the caller;

one result selected by repository order;

all results before a semantic-time cut.

“Complete accounting” is not meaningful until the eligible population is finite and exact.

The evaluation-closure companion already establishes the correct pattern: exact owners/resolvers, occurrence and subject/scope binding, causal or semantic-time cut, deterministic membership, and complete inclusion/exclusion accounting. 

evaluation-identity-evidence-cl…

The operational record should require an analogous action-specific boundary. Exact nominated references are a valid finite boundary when the policy declares them as such. Ambient “current results,” favorable-result selection, current-head lookup, or repository-order selection must refuse.

A related ambiguity appears where the policy is said to handle “result refusal” and “unavailable historical resolution.” An exact upstream refusal record may be an accepted Historical Semantic Result. Failure to resolve that record or another required historical identity is different: it must always produce Reliance Refusal and may not be policy-mapped to ineligible, indeterminate, or not_applicable.

P2-3 — Parent relationship closure needs a finite population rule and proposition-role closure

The candidate correctly requires an exact parent snapshot, owner-established parent/child/dependency closure, and complete accounting. 

operational-reliance-and-work-p…

It does not fully state:

the exact owner/resolver and relation-population boundary;

whether closure includes direct relations, transitive relations, or another declared reach;

the causal or semantic-time cut;

cycle treatment;

the required proposition for each relation.

The last point is especially important. These are non-substitutable:

child is eligible for local closeout
child action was attempted
child closeout mutation committed
child is currently in a closed state

A parent-readiness policy may intentionally consume child Local Closeout Eligibility. That is consistent with the architecture context, which allows a later parent-readiness owner to consume a pure local result. But it does not establish that the child was actually closed. A parent policy that requires committed closeout must consume the appropriate current state and/or Mutation Receipt rather than infer completion from eligibility. 

architecture-context +1

The parent policy must therefore declare the exact proposition consumed for every relation role, and Work Progression must close the full matching relation population exactly once. Unsupported cycles, open-ended traversal, mixed cuts, or inability to establish completeness must cause Reliance Refusal.

P2-4 — The Engine rule needs explicit classification, not omission-by-default

The candidate’s central rule is correct in direction: continuing-current Evaluation Authorization is not universally required. But “if the policy declares it” versus “if it does not declare it” permits accidental policy omission to become a semantic choice. 

operational-reliance-and-work-p…

Every admitted Operational Reliance Policy that accepts a result whose closure binds an Evaluation Authorization Binding should be required to classify that binding explicitly as one of:

historical-validity-only, or

continuing-current at reliance.

Silence, caller choice, inheritance, or defaulting should make the policy invalid.

The rule also needs to distinguish two Engine propositions:

Historical resolution and historical validity of the exact Binding used by the closed result. This is universally required as part of resolving the Historical Semantic Result.

Current Evaluation Authorization status at reliance time. This is policy-conditional.

If Engine’s historical-resolution mechanism now yields historically_invalid, unverifiable, unavailable, ambiguous, or substituted provenance, operational reliance must refuse even under historical-validity-only mode. That does not rewrite the historical result; it prevents present operational use. The evaluation-identity decision already distinguishes historical resolvability from current applicability and says unavailable owner resolution neither rewrites history nor permits substitution. 

evaluation-identity-evidence-cl…

The candidate must also allow a later Action Authorization policy to independently require current Engine authorization. An Operational Reliance Policy’s historical-only choice must not prevent the action-authority owner from declaring a stricter authorization-time dependency.

The current evaluation-authorization companion uses broader wording that includes fresh Engine verification before operational reliance, so it must be normalized if this candidate is approved. 

evaluation-authorization-capabi…

P2-5 — Eligibility and disposition must be one atomic determination, not two independently current records

The fixed mapping is semantically sound:

eligible       -> allow
ineligible     -> block
indeterminate  -> hold
not_applicable -> no_effect

It prevents adapters from turning uncertainty into permission or treating N/A as success. 

operational-reliance-and-work-p…

But because the mapping is total and fixed, Operational Disposition contains no independently selected semantic information. If implementations create two independent owner records, they could acquire different identities, persistence, references, replay behavior, or currentness observations.

The correct model is one atomic Operational Reliance Determination containing:

the Current-Use Eligibility facet;

its mechanically fixed Operational Disposition facet;

one policy;

one Basis;

one occurrence;

one semantic time;

one identity;

one decisive accounting closure.

The concepts remain non-substitutable in meaning, but they are not independent decisions. A Work Specification may reference the determination and identify the facet being documented.

P2-6 — “Immediately before action” is not enough for concurrent authorization, target, and dependency changes

The candidate universally requires fresh Action Authorization and expected-target verification immediately before action. It also requires a final dependency recheck for parent action and an atomic state-owner decision. 

operational-reliance-and-work-p… +1

It does not expressly require the same exact authorization, target snapshot, and through-commit dependencies to hold at the mutation commit/linearization boundary.

A separate precheck followed by an unguarded gap is not sufficient:

verify authorization current
verify parent closure P1
dependency C3 is added
commit parent promotion against old closure

For a state-changing action, the state owner must establish at commit that:

the exact Authorization Binding remains current;

the exact expected target snapshot remains current;

every dependency declared to remain current through commit remains current;

the effect is exactly the authorized effect.

If a cross-owner causal or compare condition cannot be established, no state change may commit and no Mutation Receipt may issue. This is a semantic requirement; the concrete lease, token, transaction, or compare-and-write mechanism remains deferred.

The architecture context already requires parent mutation to retain a separate authority, currentness, concurrency, durability, and replay boundary. 

architecture-context

P2-7 — The new objects need explicit ontology placement

The ontology companion requires canonicality, authority, authored-artifact status, durable provenance, projections, and transient state to remain independent. It also distinguishes owner-produced results from origin/attempt provenance. 

canonical-object-ontology-and-o… +1

The candidate introduces several objects without classifying them:

Action Kind;

Action Target Snapshot;

Operational Reliance Policy and Basis;

Current-Use Eligibility and Disposition;

Action Intent;

Action Authorization Binding and refusal;

Action Attempt and Terminal;

Mutation Receipt.

Without classification, implementations can incorrectly place Action Intent or Action Terminal in an owner-result repository, treat Mutation Receipt as generic attempt provenance, or put the whole chain into the generic Artifact Instance lifecycle.

The required classification is:

Object	Ontology category
Action Kind definitions; Operational Reliance, Local Closeout, Parent Eligibility, and authorization policies	Canonical semantic definitions or policies
Action Target Snapshot; Operational Reliance Basis and Determination; Action Authorization Binding/Refusal; Local/Parent Eligibility; Mutation Receipt	Owner-produced canonical records/results
Closed Action Intent; Action Attempt; Action Terminal	Durable request/attempt provenance
Unclosed action request or in-progress attempt	Transient operational state
Renderings, compact explanations, CLI/Tauri payloads	Projection or representation

None is an Artifact Instance. The ontology’s current general row already places downstream disposition, current-use basis, action decisions, and mutation receipts outside the artifact lifecycle; the candidate should complete that classification for its newly named objects. 

canonical-object-ontology-and-o…

P3 clarifications worth landing now
P3-1 — Diagnostic acceptance should be role- and direction-specific

The candidate correctly preserves Evaluation Use Intent and permits one policy to reject diagnostic provenance while a remediation policy accepts it. 

operational-reliance-and-work-p…

The companion authorization record says diagnostic provenance may not support positive operational reliance unless an exact downstream policy explicitly permits it. 

evaluation-authorization-capabi…

Add that permission to use a diagnostic failure to:

block;

hold;

trigger remediation; or

admit a posture observation

does not permit the same diagnostic result to establish positive eligibility for deployment, release, closeout, or another Action Kind. Acceptance must be declared per consumed result role and permitted directional effect.

P3-2 — Pure advisory use and blocking dispositions should be explicitly non-mutating

The candidate says prioritization and recommendation trigger Operational Reliance, but Action Intent is formed only after allow. A pure advisory use may therefore stop at the reliance determination. That should be stated directly.

Similarly:

block does not mutate a work item to a blocked state;

hold does not create a queue or waiting state;

no_effect does not mark an action successful;

allow does not admit a Posture Observation or produce a recommendation.

Any such state or owner-result creation remains separately owned.

P3-3 — Exact replay should include refusals

For one Operational Reliance Occurrence Identity, exact replay rules currently describe successful Basis/eligibility/disposition formation. The same rule should cover Reliance Refusal: exact same formation content returns the same refusal identity and decisive owner provenance; changed content under the same occurrence refuses conflicting reuse.

The same principle should apply to Action Authorization Refusal and Action Terminal replay.

P3-4 — Formation of semantic records must not recursively become another action

Creating or retaining an Operational Reliance Basis, Determination, Authorization Binding, terminal, or receipt is not itself another action under this decision merely because a repository changes. Otherwise the model recursively requires authorization for issuing authorization records and receipts for issuing receipts.

The action chain governs the bounded operational effect named by the Action Intent. A semantic record’s own owner-admission and durability rules remain separate unless another independently named Action Kind expressly targets that record lifecycle.

P4 items that should remain deferred

There are no required P4 edits.

The candidate correctly defers concrete:

policy catalogs and thresholds;

work hierarchy and state-machine shapes;

authorization mechanisms;

actor and approver rules;

schemas, fields, APIs, storage, registries, repositories, and crates;

retry counts, timeouts, leases, lock mechanisms, and transaction technology;

SDK, CLI, Tauri, Flow, Pipeline, skill, and UI projections;

implementation slices, migrations, compatibility, and rollout.

Those details should not be pulled into this semantic record. 

operational-reliance-and-work-p…

Adversarial audit
1. Historical Result → Reliance → Eligibility → Disposition → Authorization → Terminal → Receipt

The conceptual separation is fundamentally correct.

Stage	Exact proposition	Correct owner	Must not be used as
Historical Semantic Result	What was true for one exact closed historical context	Contracts, Engine, Work Progression, or another original semantic owner	Current permission, target currentness, actor authority
Operational Reliance Basis	Which exact historical results and present facts are being considered now for one exact action question	Downstream action domain; Work Progression for local/parent questions	Evaluation truth, action authorization, mutation
Current-Use Eligibility	Whether the historical truth may support consideration of that action question now	Same downstream action domain	Actor permission, proof action happened
Operational Disposition	Fixed control-facing interpretation of eligibility	Same atomic determination	An independently selected policy result
Action Intent	Exact requester-owned request for the already evaluated action question	Requester/origin owner	Authorization or proof of attempt
Action Authorization Binding	Whether the exact actor may attempt the exact intent under exact current authority	Action-authorization owner	Eligibility, evaluation authorization, mutation
Action Attempt/Terminal	What the executor attempted or refused under one Binding	Action executor	Proof that state committed
Mutation Receipt	Proof that the authoritative state owner committed the exact authorized transition	State owner	Current state forever, general action authority

The candidate already captures most of these distinctions in its vocabulary and responsibility map. The required amendments close effect congruence, attempt identity, refusal, and terminal/receipt ownership. 

operational-reliance-and-work-p… +1

Critical non-substitution rules

A historical pass is not an allow.

An allow is not an Action Authorization Binding.

A Binding is not proof an attempt occurred.

An Action Terminal is not proof a mutation committed.

A Mutation Receipt is not proof the target is still current.

A Reliance Refusal is not ineligible / block.

An Action Authorization Refusal is not an Action Terminal.

A transport success is none of the above.

The candidate’s transport-without-receipt scenario correctly preserves the last distinction. 

operational-reliance-and-work-p…

2. Are eligibility and disposition both necessary?

Both vocabulary layers are useful, but they must not be two independent semantic decisions.

Eligibility answers:

May these exact historical results support consideration of this exact action question now?

Disposition tells generic consumers:

What control posture corresponds to that answer?

The fixed mapping is worthwhile because it prevents presentation or transport code from inventing mappings. It also lets a generic consumer react to allow, block, hold, and no_effect without pretending those are gate outcomes.

But the mapping is bijective. Therefore the pair is informationally redundant. The architecture should embrace that deliberately:

one shared determination identity;

two named facets;

one source of currentness and replay;

no possibility of eligible / hold or indeterminate / allow;

no separate disposition policy.

That makes the two terms non-substitutable in meaning, while preventing independent lifecycle semantics.

3. Is Work Progression the correct owner?

Yes.

The architecture and phase records left parent-readiness and dependency composition to a future workflow/dependency owner, expressly separate from Contracts local-closeout truth and actual parent mutation. 

architecture-context

 

phase-and-slice-map

Work Progression is appropriately scoped because local closeout and parent eligibility share:

exact work-target identity;

current work-state snapshot closure;

parent/child/dependency relation closure;

action-specific work policies;

present-use currentness;

local-to-parent composition;

replay and invalidation semantics.

It is not too broad, because the candidate excludes:

Contracts claim/metric/gate truth;

Engine evaluation authorization;

Work Specification authorship;

release/deployment/posture policies;

Action Authorization issuance by implication;

actual state mutation.

It is not too narrow, because parent eligibility cannot be owned honestly without also owning the exact work-relationship closure and the relation between local results and parent policy.

The phrase “Work Progression” could be misread as a workflow engine, but the normative boundary is sound: it owns progression eligibility semantics, not the authoritative progression state machine. The candidate states that source work relations and state remain owned by the authoritative work-structure/work-state owner. 

operational-reliance-and-work-p…

4. Local closeout vs parent eligibility vs parent promotion/action

The candidate gets the principal separation right:

local results
  -> Local Closeout Eligibility

complete parent relation closure
+ exact child results
+ parent-level inputs
  -> Parent Eligibility Result

Parent Eligibility eligible/allow
  -> separate Action Intent
  -> separate Action Authorization
  -> state-owner mutation

Local Closeout Eligibility proves neither:

local closeout was attempted;

local closeout committed;

parent relations are complete;

parent policy is satisfied;

parent promotion is authorized.

Parent Eligibility proves neither:

the parent changed;

promotion occurred;

child authorizations transfer;

target state remains unchanged until commit.

This aligns with the architecture context and the HCM-5.2 implementation-history boundary. 

architecture-context

 

README

The missing semantic point is the exact child proposition role. A policy must say whether it consumes child eligibility, child committed state, a terminal, a receipt, or another result. Implementations must not silently treat those as equivalent.

5. Complete parent/child/dependency population and currentness

The candidate requires complete accounting, but completeness needs an exact population boundary.

A sound Parent Eligibility Basis must establish:

authoritative relation owner/resolver
+ exact parent snapshot
+ exact relation kinds/directions
+ direct/transitive reach
+ exact semantic-time or causal cut
+ currentness rule
+ cycle treatment
+ deterministic relation membership
+ exact required result role per relation
+ included/excluded/missing/duplicate/conflicting accounting

Every relation matching the boundary must be accounted exactly once.

A changed child result, dependency membership, relation direction, or parent snapshot creates a new parent basis. For a later parent mutation, the exact closure required by the action must still hold at commit, not merely at an earlier precheck.

Unsupported cycles should not receive an implementation default. They should refuse unless the exact parent policy expressly defines valid cycle semantics.

6. Work Specification input and outcome direction

This part is strong and should be kept.

A Work Specification may:

nominate an exact already owner-recognized policy or other input;

be consumed as one exact authored revision when the policy declares its proposition and currentness mode;

later reference exact evaluation, reliance, terminal, or receipt results.

It may not:

create a policy through prose;

create eligibility or disposition;

issue action authorization;

prove a mutation;

rewrite itself silently when an operational action commits.

The later-reference direction is correct:

Work Specification W1
  -> evaluation/reliance/action M1
  -> later Work Specification W2 references M1

W2 is not retroactively an input to M1. This matches the authored-artifact companion’s exact-reference and anti-cycle rules. 

operational-reliance-and-work-p…

 

authored-artifact-evaluation-re… +1

The atomic determination amendment should merely clarify that a Disposition Reference names the shared Operational Reliance Determination and the disposition facet it records.

7. Diagnostic provenance

The candidate correctly refuses to erase diagnostic provenance. It also correctly allows different action policies to treat that provenance differently.

The necessary rule is directional:

diagnostic failure may block deployment;

diagnostic failure may permit remediation;

diagnostic evidence may support a posture observation;

none of those permissions implies that a diagnostic pass may positively authorize deployment or closeout.

A policy must expressly permit the exact use-intent/result-direction/action-kind combination. “Diagnostic accepted” must not become a blanket provenance waiver.

8. Unavailable historical source resolution

The candidate correctly chooses fail-closed behavior:

the historical result remains immutable;

a display may preserve the owner-attributed unavailable/unverifiable condition;

operational reliance refuses;

no current replacement, nearest policy version, projection, or copied label substitutes;

refusal is not rewritten as block or hold.

That matches the evaluation-identity record’s historical-resolvability contract. 

operational-reliance-and-work-p…

 

evaluation-identity-evidence-cl…

The policy text should clarify that it may determine the semantic effect of an exact resolved upstream refusal record, but may never decide that inability to resolve a required record is acceptable.

9. Retries, concurrent target changes, uncertain commits, and replay
Exact replay

Correct:

replay of a Reliance occurrence uses recorded historical inputs and observations;

it does not reacquire current authority;

replay of an Action Terminal or Receipt does not establish current state;

transport correlation does not create semantic occurrences.

New reliance

Correct:

a new present determination receives a new Operational Reliance Occurrence Identity, even with equal semantic content.

Action retries

Incomplete until Action Attempt Identity is added.

The amended rule should distinguish:

Event	Required outcome
Same attempt identity and same exact execution request	Return same terminal/receipt; do not re-execute
Same attempt identity with changed content	Refuse conflicting reuse
New attempt after a known no-commit refusal	Allowed only under exact authorization/policy and fresh checks
New attempt after uncertain commit/effect	Prohibited until authoritative non-commit or duplicate-safe recovery is established
Target changed before attempt	Refuse; new reliance required
Target changed after attempt began but before mutation commit	State-owner compare/commit condition fails; no receipt
Authorization revoked before commit	No mutation; refusal terminal under the existing attempt
Receipt replay	Historical proof only; current state comes from the state owner
Concurrent target changes

The candidate’s exact target-snapshot model is the right foundation. The amendment must move the universal target and authorization checks to the commit boundary for state-changing actions, preventing check-then-act races.

10. Gate-to-posture feedback and same-occurrence cycles

The no-cycle rule is correct:

P_n
 -> requirements
 -> evaluation
 -> gate
 -> Posture Observation
 -> Posture Recommendation
 -> authorized transition
 -> P_n+1
 -> later evaluation only

No gate may mutate posture, reselect the posture governing its own evaluation, or feed P_n+1 back into the originating occurrence. This matches the posture companion. 

posture-assurance-policy-semant…

The candidate’s path needs one additional semantic stage: the recommendation does not itself establish current eligibility for the transition. Before the transition Action Intent and Engine action authorization, the Engine posture-transition domain should form its own exact transition-specific Operational Reliance Basis and Determination over:

the exact recommendation;

exact current posture target;

required approvals;

transition policy;

every other declared current fact.

The transition then follows the universal action path. This preserves the distinction between recommendation, transition eligibility, actor authority, and committed posture mutation.

A pure posture-observation admission or recommendation remains an owner-produced result under its own policy. allow from an earlier action domain is not itself the observation or recommendation.

Dedicated analysis: universal versus policy-declared continuing-current Engine authorization
Decision

Keep the policy-conditional rule. Strengthen its formulation. Do not replace it with universal continuing-current Evaluation Authorization.

The Engine Evaluation Authorization Binding answers:

Was this exact evaluation purpose and requested reach admitted for the historical evaluation?

It does not answer:

May this actor perform this later action now?

That latter question belongs to the downstream reliance policy, Action Authorization owner, and state owner.

The companion evaluation records already distinguish issuance, currentness, supersession, historical verification, and operational reliance. The current evaluation-authorization record nevertheless uses broader operational-reliance wording that must be normalized if this candidate is approved. 

evaluation-authorization-capabi… +1

Universal invariants that must remain

Regardless of mode:

The exact historical Binding and issuance lineage must remain historically resolvable through an owner-declared historical-resolution mechanism.

The historical result must have been validly closed under the exact historical authorization checkpoint.

historically_invalid, unavailable, ambiguous, substituted, or unverifiable historical provenance causes Reliance Refusal.

The historical result is never rewritten.

Fresh Action Authorization and expected-target verification remain universal for every effectful action attempt.

State-changing actions additionally require authorization and target validity at commit.

Policy-conditional proposition

The reliance policy expressly chooses:

Historical-validity-only

The result’s exact historical authorization provenance must resolve, but current EAB status is not a reliance input.

Continuing-current

In addition to historical resolution, Engine must report the Binding current at reliance time. superseded, not_current, or unverifiable causes Reliance Refusal.

A later Action Authorization policy may independently require continuing-current EAB status even when the reliance policy selected historical-validity-only.

Stress tests
Scenario	Correct mode and outcome
Low-risk local closeout	Historical-validity-only can be coherent. Work Progression still requires current work target, result-target congruence, all policy-declared work/artifact facts, fresh Action Authorization, and final expected-state verification. Later EAB non-currentness alone need not erase the utility of a valid historical gate.
High-risk deployment	The deployment reliance policy should normally declare continuing-current EAB or another exact current Engine Context Resolution proposition. The deployment authorization policy may independently require the same or stricter Engine verification. Revocation then stops reliance or authorization without rewriting the gate.
Remediation	Historical-validity-only is often the safer composition. A historical failure may remain a valid reason to remediate after the original evaluation reach is no longer current. Universal current EAB would perversely obstruct corrective work. Current target, remediation scope, and fresh action authority still apply.
Negative/blocking reliance	A valid historical failure may continue to establish ineligible / block under a historical-only policy. If continuing-current mode was selected and Engine cannot verify it, reliance refuses. Either way, no allow exists, so an executor cannot proceed.
Pure advisory prioritization	Historical-validity-only is usually sufficient unless the advice claims present conditions. It may stop at the Reliance Determination; no Action Authorization is needed unless a later action changes authoritative priority/work state.
Posture-observation admission	If the observation proposition is “this gate failed under historical context X,” historical-only may be correct. If the observation purports to establish a current posture-relevant condition, its owner policy must require the appropriate fresh Engine/current facts or a fresh evaluation.
Action after Action Authorization revocation	Always stops. The final Action Authorization verification is universal and independent of the EAB reliance mode. Eligibility remains immutable history; no mutation receipt issues.
Why universal continuing-current EAB is weaker architecture

Universal current EAB would:

treat permission to evaluate as continuing certification of every later use;

make local closeout, remediation, negative blocking, historical incident response, and advisory analysis depend on an unrelated current evaluation permission;

encourage action domains to infer current Engine meaning from transitive provenance;

duplicate Action Authorization;

obscure which present Engine proposition the action genuinely needs.

The selected rule is safer because it requires the exact action policy to name the present Engine proposition it consumes.

Stronger formulation

The stronger formulation is not universal current EAB. It is:

universal explicit mode selection
+ universal exact historical resolution/validity
+ policy-declared current EAB at reliance
+ independently policy-declared current EAB at action authorization
+ universal fresh action authorization and target verification
+ commit-point verification for mutation

With that formulation, the rule is safe, coherent, and composable.

Stage-by-stage responsibility and authority audit
Stage	Responsible semantic owner	Required input/output	Authority that does not transfer
1. Historical result resolution	Original result and definition owners; Engine for EAB historical verification	Exact owner identity/content, historical closure, policy definitions, historical semantic times	Resolver does not make result current or operationally eligible
2. Action question closure	Downstream action domain; target owner supplies exact snapshot facts	Admitted Action Kind, exact target snapshot, bounded effect/scope, eligibility-relevant parameters	Requester does not create target truth or Action Kind authority
3. Operational Reliance Policy admission/selection	Exact downstream policy authority	Exact current policy and complete selection basis	Work Specification or caller prose cannot create/select policy ambiently
4. Operational Reliance Basis formation	Work Progression for local/parent; other action domain otherwise	Finite exact result population, target congruence, present facts, currentness observations, complete accounting	Domain consumes but does not reconstruct Contracts, Engine, artifact, or state-owner propositions
5. Operational Reliance Determination	Same action domain	One atomic eligibility/disposition result or Reliance Refusal	Determination grants no actor permission and performs no action
6. Action Intent	Requester/origin owner	Exact congruent action question, actor/principal, eligible/allow determination	Intent is untrusted durable request provenance, not authorization
7. Authorization decision	Exact action-authority owner	Intent, actor, exact reliance determination, target/effect, owner authority basis and current conditions	Authorization does not improve eligibility or alter historical truth
8. Authorization refusal or Binding	Action-authority owner	Exact Refusal or immutable Binding with currentness/supersession lineage	Refusal creates no attempt/terminal; Binding creates no effect
9. Action Attempt identity and pre-effect checks	Action executor, consuming owner-produced verification	Exact Binding, exact attempt identity, exact target, required current conditions	Executor does not self-produce another owner’s currentness
10. Effect execution	Executor and authoritative state/effect owner	Only exact authorized effect	Executor cannot broaden scope or report commit from transport success
11. Commit boundary	Authoritative state owner	Exact expected predecessor, current authorization, required through-commit facts	Precheck outside the commit boundary is insufficient
12. Action Terminal	Executor	One terminal for one attempt, including refusal/failure/uncertainty or completion	Terminal cannot prove committed target mutation without receipt
13. Mutation Receipt	Authoritative state owner	Exact attempt, predecessor, authorized effect, durable successor state, commit semantic time	Receipt does not prove present current state forever
14. Later artifact/posture use	Artifact owner, posture observation/recommendation/transition owners	Exact Outcome/Disposition references or new owner-specific basis	Later linkage never rewrites the earlier chain

SDK, CLI, Tauri, Flow, Pipeline, skills, and transports remain carriers, projections, and schedulers only. They may not select policy, reinterpret disposition, issue authorization, or report mutation without owner truth. 

operational-reliance-and-work-p…

Exact normative amendments
Amendment 1 — Bind the exact action question before reliance

Location: In ## Purpose and scope, revise the canonical chain after “exact action-kind and action-target selection.” In ## Canonical vocabulary, replace the Action Kind, Operational Reliance Basis, Current-Use Eligibility, and Action Intent rows. In ## Action authorization and mutation, replace the paragraph beginning “A changed effect, actor, target…”. 

operational-reliance-and-work-p… +1

Normative wording:

Action Kind is an immutable, named, versioned semantic definition admitted by one downstream action domain. It defines one bounded effect family and the compatibility rules for the exact action question. Equal verbs, labels, or user-interface actions do not make two Action Kinds interchangeable. An Action Kind grants no authority.

Before forming an Operational Reliance Basis, the responsible downstream domain MUST close the exact action question whose present use is being decided: one exact Action Kind, one exact Action Target Snapshot, the exact bounded effect, effect scope, and every action parameter whose value can change eligibility under the admitted Operational Reliance Policy.

The Operational Reliance Basis, Current-Use Eligibility, and Operational Disposition MUST bind that exact action question.

An Action Intent MUST be exactly congruent with the action question already decided by the Operational Reliance Determination. It MAY add only the exact actor or principal and execution-local details that the Operational Reliance Policy declares irrelevant to eligibility. It MUST NOT broaden, substitute, or change the Action Kind, Action Target Snapshot, expected state, effect, effect scope, or an eligibility-relevant action parameter.

An Action Authorization Binding MAY impose additional restrictions or execution conditions but MUST NOT authorize a broader or different action question.

A change to Action Kind, Action Target Snapshot, expected state, effect, effect scope, or any eligibility-relevant action parameter requires a deliberately new Operational Reliance Occurrence Identity, Basis, Operational Reliance Determination, Action Intent, and authorization path. A change only to the actor or principal requires a new Action Intent and authorization. A change only to the authorization owner’s rule, basis, or conditions may require a new authorization decision over the unchanged Intent; it does not rewrite that Intent.

Amendment 2 — Add Authorization Refusal and Action Attempt Identity; close terminal ownership

Location: In ## Canonical vocabulary, replace the Action Authorization Binding and Action Terminal rows and insert two new rows between them.

Normative wording:

Action Authorization Binding is the immutable authorization-owner result that binds the exact authorization-owner identity; exact owner-recognized authorization rule and authority basis; issuance semantic time and owner-verifiable lineage; one exact Action Intent; actor or principal; exact action question; exact eligible / allow Operational Reliance Determination; bounded authorization conditions; and the exact rule governing whether one or more Action Attempts are permitted. It cannot improve eligibility, broaden the Intent, or substitute for another authorization owner.

Action Authorization Refusal is the immutable authorization-owner result that the exact Action Intent cannot be authorized, with its exact decisive reason, authority basis, semantic time, and owner provenance. It creates no Action Authorization Binding, Action Attempt Identity, Action Terminal, or Mutation Receipt.

Action Attempt Identity is the exact action-executor-owned identity of one deliberately intended execution attempt under one exact Action Intent and Action Authorization Binding. It distinguishes exact delivery replay from a new attempt. It grants no authority and is not the Intent, Binding, Terminal, or Mutation Receipt.

Action Terminal is the action-executor-owned immutable durable provenance for one exact Action Attempt Identity under one exact Action Authorization Binding. It records the executor’s exact attempt outcome, including a refusal after the attempt boundary was entered, completion without target mutation, execution failure, or effect/commit uncertainty. It MAY report a committed target-state mutation only by binding the exact authoritative state-owner Mutation Receipt. A pre-attempt Action Authorization Refusal is not an Action Terminal.

Mutation Receipt is the authoritative state owner’s immutable proof that one exact Action Attempt committed the exact authorized mutation against the exact expected predecessor state and produced the exact successor state at the owner’s commit semantic time. No terminal, acknowledgement, transport success, or executor assertion substitutes for it.

Amendment 3 — Replace retry and execution-boundary rules

Location: Replace steps 3–5 and the retry paragraphs in ## Action authorization and mutation. 

operational-reliance-and-work-p…

Normative wording:

The exact action-authorization owner resolves the Action Intent and either issues one exact Action Authorization Binding or one exact Action Authorization Refusal. A refusal creates no Binding or Action Attempt.

Before an Action Attempt begins, the executor MUST allocate or resolve one exact Action Attempt Identity and obtain exact owner-produced positive verification of the Action Authorization Binding, exact Action Intent, exact Operational Reliance Determination, exact expected Action Target Snapshot, and every condition declared for attempt admission. The executor MUST NOT reconstruct another owner’s authorization, target-currentness, or dependency proposition.

For a state-changing action, the authoritative state owner MUST enforce at the mutation commit or linearization boundary that the same exact Action Authorization Binding remains current, the same exact expected Action Target Snapshot remains current, the effect is exactly the authorized effect, and every dependency declared to remain current through commit still verifies under its owner. A prior recheck separated from commit by an unguarded concurrency gap is insufficient.

The executor produces exactly one Action Terminal for the Action Attempt. The authoritative state owner produces a Mutation Receipt only after durable commit of the exact authorized state change. The Terminal may report committed mutation only by binding that Receipt.

Exact delivery replay under the same Action Attempt Identity and exact attempt content MUST return the same Action Terminal and any same Mutation Receipt without re-executing the effect. Changed attempt content under the same Action Attempt Identity MUST refuse.

A non-idempotent retry is a deliberately new Action Attempt. It is permitted only when the exact Action Authorization Binding or a fresh successor Binding permits another attempt, all required dependencies remain exact and current, and the authoritative effect or state owner has positively established that the prior attempt did not apply or commit.

When a prior attempt has an uncertain effect or commit status, an ordinary non-idempotent retry MUST NOT proceed. A later attempt is permissible only under a separately admitted recovery rule and fresh authorization that explicitly account for possible prior application and establish duplicate-safe recovery. Recomputed eligibility, unchanged target appearance, timeout, lost acknowledgement, or transport failure does not establish non-commit.

Amendment 4 — Strengthen the policy-conditional Engine rule

Location: Replace the four bullets and explanatory paragraph in ## Exact current-use and Engine-verification rule beginning with “Fresh current verification…”. 

operational-reliance-and-work-p…

Normative wording:

Every admitted Operational Reliance Policy that accepts a Historical Semantic Result whose exact closure binds an Engine Evaluation Authorization Binding MUST explicitly declare one reliance-time dependency mode for that Binding:

historical-validity-only; or

continuing-current.

An omitted, inherited, caller-selected, ambient, or defaulted mode makes the policy invalid and refuses policy admission or resolution.

Under both modes, the common exact-result floor requires owner-recognized historical resolution of the exact Evaluation Authorization Binding, its issuance lineage, and the historical-verification outcome required to establish that the Historical Semantic Result was validly closed. This requirement may be satisfied through any Engine-declared historical-resolution mechanism; it is not a current-authorization query.

An Engine historical-resolution outcome of historically_invalid, unverifiable, unavailable, ambiguous, or substituted causes Reliance Refusal. It does not rewrite the Historical Semantic Result and MUST NOT become ineligible, indeterminate, or not_applicable.

Under continuing-current mode, a fresh Engine-produced positive current verification at the Operational Reliance semantic time is additionally mandatory. superseded, not_current, unverifiable, unavailable, ambiguous, or substituted current verification causes Reliance Refusal.

Under historical-validity-only mode, current Evaluation Authorization status is not an Operational Reliance input. The action domain MUST NOT infer a current Engine conclusion from the historical Binding or allow an independently obtained current observation to alter the Operational Reliance Determination.

A later Action Authorization policy MAY independently declare the same Evaluation Authorization Binding or another Engine-owned proposition as an authorization-time or commit-time condition. When it does, only Engine’s exact fresh verification satisfies that condition. The Operational Reliance mode neither supplies nor forbids that distinct authorization-stage dependency.

Policy-conditional means policy-mandatory when declared, never caller-optional.

Amendment 5 — Make eligibility and disposition one atomic determination

Location: Insert after the eligibility/disposition mapping table and update references to “eligibility and disposition identities.” 

operational-reliance-and-work-p…

Normative wording:

One valid Operational Reliance Basis produces exactly one immutable Operational Reliance Determination under one Operational Reliance Occurrence Identity, policy, action question, target, semantic time, identity, and decisive accounting closure.

Current-Use Eligibility and Operational Disposition are two named facets of that one Determination. Operational Disposition is the fixed canonical mapping of the Eligibility facet and is not a second policy decision, separately current object, independently supersedable record, or independently replayable result.

No implementation may issue, persist, supersede, reference, or replay one facet as though the other could differ. A reference MAY identify the shared Operational Reliance Determination and the exact facet displayed or documented.

Reliance Refusal produces no Operational Reliance Determination and therefore no Eligibility or Disposition facet.

Amendment 6 — Define the finite eligible historical-result boundary

Location: Insert after the Operational Reliance Policy declaration bullets and before the occurrence replay rules. 

operational-reliance-and-work-p…

Normative wording:

For every consumed Historical Semantic Result role, an Operational Reliance Policy MUST declare one exact finite eligible source-result boundary. The boundary MUST bind:

the permitted semantic owners and exact repositories, resolvers, or exact nominated references;

accepted result kinds, Evaluation Use Intents, and result roles;

required subject, target, scope, evaluation-occurrence, context, and semantic-time relations;

historical-only or present-currentness modes;

cardinality, multiplicity, ordering, and composition semantics;

deterministic inclusion and exclusion rules; and

complete candidate, selected, excluded, missing, duplicate, conflicting, unavailable, and refused accounting.

Basis formation MUST account exactly once for every owner-recognized result matching that boundary and every required result role.

Caller-selected subsets, favorable-result selection, current-head or nearest-version lookup, repository order, open-ended “all current results” queries, or unrecorded omission MUST refuse.

An exact upstream refusal record MAY be consumed only when the policy declares that exact refusal kind as an accepted Historical Semantic Result role and the refusal resolves successfully from its owner.

Failure to resolve a required identity, semantic content, owner provenance, or historical validity is always Reliance Refusal. It cannot be mapped by policy to ineligible, indeterminate, or not_applicable.

Amendment 7 — Close the parent relationship population and proposition roles

Location: Insert after the opening paragraph and list in ## Parent eligibility and parent action. 

operational-reliance-and-work-p…

Normative wording:

Every Parent Eligibility Policy MUST declare one exact finite authoritative parent-relationship boundary. That boundary MUST bind:

the authoritative work-structure owner and exact resolver or snapshot identity;

the exact parent Action Target Snapshot;

supported child, dependency, role, ordering, and composition relation kinds and directions;

the exact direct, transitive, or otherwise declared relation reach;

the causal or semantic-time cut and currentness rule;

the policy’s cycle treatment;

deterministic relation inclusion and exclusion rules; and

complete included, excluded, missing, duplicate, conflicting, cyclic, superseded, stale, and unresolved accounting.

Work Progression MUST account exactly once for every owner-recognized relation matching that boundary and every policy-required result role. Open-ended traversal, unsupported cycles, mixed relation cuts, ambiguous direction, or inability to establish boundary completeness causes Reliance Refusal before Parent Eligibility.

For every relation role, the Parent Eligibility Policy MUST declare the exact proposition it consumes, including whether it requires a child Local Closeout Eligibility, Action Terminal, Mutation Receipt, current child work-state snapshot, or another exact owner result. Those propositions are non-substitutable.

Consuming Local Closeout Eligibility establishes only the child’s eligibility proposition. A parent policy that requires the child to have actually closed MUST consume the authoritative current closed-state proposition and/or exact Mutation Receipt declared by that policy; it MUST NOT infer committed closeout from eligibility.

For a later parent mutation, the state owner MUST establish at the mutation commit boundary that the exact parent snapshot and every parent/child/dependency relation declared to remain current through commit still match the Parent Eligibility and Action Authorization closures.

Amendment 8 — Add ontology placement

Location: Insert a new ## Ontology placement subsection after ## Canonical vocabulary.

Normative wording:

The objects introduced by this record retain the ontology categories fixed by the canonical-object ontology decision:

Action Kind definitions; Operational Reliance Policies; Local Closeout Policies; Parent Eligibility Policies; and any separately named Action Authorization policies are canonical semantic definitions or policies.

Action Target Snapshots; Operational Reliance Bases and Determinations; Reliance Refusals; Action Authorization Bindings and Refusals; Local Closeout Eligibility; Parent Eligibility Results; and Mutation Receipts are owner-produced canonical records or results.

A closed Action Intent, Action Attempt Identity and its durable attempt occurrence, and Action Terminal are durable request or attempt provenance.

An unclosed action request, in-progress attempt, or transport-local action failure before an owner occurrence closes is transient operational state.

Renderings, summaries, compact explanations, SDK DTOs, CLI/Tauri payloads, and other views are deterministic projections or representations as applicable.

None of these objects is an Artifact Instance. Persistence, project-local storage, human readability, content addressing, or physical colocation MUST NOT change its category or semantic owner.

Amendment 9 — Clarify pure advisory use and complete the posture transition path

Location: Insert after ## Historical truth and present use; replace the posture-feedback diagram and its following paragraph. 

operational-reliance-and-work-p… +1

Normative wording:

An Operational Reliance Determination MAY be the terminal result of a pure, non-mutating advisory use. When no effectful Action Intent is formed, no Action Authorization Binding, Action Attempt, Action Terminal, or Mutation Receipt exists.

A separately owned admission, prioritization, or recommendation MAY consume an exact Operational Reliance Determination only under that later owner’s own exact policy, admission, currentness, and accounting rules. eligible / allow is not the later admission, priority, recommendation, or action authority.

Formation or durable retention of the semantic records defined by this chain is not recursively another effectful action merely because a repository changes. The action chain governs the bounded operational effect named by an exact Action Intent unless another independently admitted Action Kind expressly targets that record lifecycle.

The gate-to-posture path is:

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

The Posture Recommendation does not establish transition eligibility or authorization. The transition-specific Operational Reliance Basis MUST independently close the exact recommendation, current posture target, transition policy, approvals, and every other declared present dependency.

No observation, recommendation, transition basis, authorization, terminal, receipt, or successor posture snapshot may alter the posture, requirements, Evaluation Purpose, evidence closure, Metric Definitions, Gate Policy, or semantic result of the evaluation occurrence that produced the source result.

Amendment 10 — Tighten diagnostic provenance and refusal replay

Location: Insert after the diagnostic scenario and after the occurrence replay bullets.

Normative wording:

Acceptance of diagnostic Evaluation Use Intent MUST be declared per consumed result role, result direction, and Operational Reliance action question.

Permission to use a diagnostic result to block, hold, trigger remediation, admit a posture observation, or support another negatively directed use does not permit that result to establish positive eligible / allow for a different Action Kind, target, effect, or scope.

No owner may erase, relabel, or infer a different Evaluation Use Intent.

For one Operational Reliance Occurrence Identity, exact replay of the same refused formation content MUST return the same Reliance Refusal identity and decisive owner provenance. Different content under the same occurrence identity MUST refuse conflicting reuse.

Exact replay of one Action Authorization request or Action Attempt likewise returns the same owner outcome for the same exact occurrence and content; changed content under the same semantic identity refuses.

Amendment 11 — Scenario corpus corrections and additions

Location: Replace the “Eligible but unauthorized” row and add the following rows to ## Discriminating scenario corpus.

Replacement row

Eligible but authorization refused. Local closeout is eligible / allow, but the action-authorization owner refuses the exact actor, authority basis, effect, or scope.
Expected outcome: The authorization owner produces one exact Action Authorization Refusal. No Action Authorization Binding, Action Attempt Identity, Action Terminal, mutation, or Mutation Receipt exists. The Operational Reliance Determination remains immutable history.
Invariant: Eligibility, authorization refusal, attempt provenance, and mutation truth are distinct propositions.

New row: changed effect

Same kind and target, changed effect. An Operational Reliance Determination establishes eligible / allow for exact effect E1 against target snapshot T1. A later Action Intent uses the same Action Kind and target label but requests materially different effect or effect scope E2.
Expected outcome: Intent formation or authorization refuses the incongruence. A deliberately new Operational Reliance Occurrence, Basis, Determination, Intent, and authorization path are required for E2.
Invariant: An allow cannot be broadened through later Action Intent parameters.

New row: uncertain commit retry

Uncertain commit followed by retry. An authorized non-idempotent Action Attempt loses its definitive state-owner outcome and has an uncertain terminal with no Mutation Receipt. The target presentation appears unchanged and the caller requests another attempt.
Expected outcome: The ordinary retry refuses. It may proceed only after the authoritative effect/state owner positively establishes non-application or under a separately admitted duplicate-safe recovery rule and fresh authorization that explicitly account for possible prior commit.
Invariant: Timeout, unchanged appearance, and recomputed eligibility do not prove non-commit.

New row: historical invalidity versus non-currentness

Historical-validity-only mode with later Engine changes. A valid historical gate binds Evaluation Authorization A1. Engine later reports A1 non-current, while historical resolution remains historically_valid. In a comparison case, Engine later reports historical resolution historically_invalid or unverifiable.
Expected outcome: Under historical-validity-only mode, non-currentness alone does not alter the Operational Reliance Determination. The historical-invalid or unverifiable comparison case causes Reliance Refusal without rewriting the gate.
Invariant: Historical validity and continuing-current authorization are separate Engine propositions.

New row: omitted parent relation and child proposition

Parent closure omits a required relation. The authoritative relation boundary contains child C3, but the proposed Parent Eligibility Basis accounts only for C1 and C2. In a comparison case, the policy requires actual child closeout but supplies only C3 Local Closeout Eligibility.
Expected outcome: Both cases refuse before Parent Eligibility. The first lacks complete relation membership; the second supplies the wrong child proposition and requires the declared current-state or Mutation Receipt proof.
Invariant: Complete relation closure and proposition-role congruence precede parent eligibility.

Replacement posture row

Gate feeds posture through later owner stages. A gate failure forms a valid posture-observation-specific Operational Reliance Determination and is separately admitted as a Posture Observation. A recommendation policy produces an exact Posture Recommendation. A distinct transition-specific Operational Reliance Basis and Determination then closes over the recommendation, exact current posture target, approvals, and other declared facts before Engine issues transition Action Authorization.
Expected outcome: Only the final Engine-authorized compare-and-write may create the successor posture snapshot and Mutation Receipt. The originating gate and evaluation remain unchanged and the successor posture affects only a later evaluation.
Invariant: Observation, recommendation, transition eligibility, transition authority, and mutation are separate and cannot form a same-occurrence cycle.

Readiness after amendment
As written

The candidate is not yet ready as human-reviewable semantic authority because:

the exact effect can expand after eligibility;

uncertain non-idempotent retry is not safely closed;

authorization refusal and Action Terminal contradict each other;

attempt identity and terminal ownership are incomplete;

result and parent populations are not finitely closed;

the Engine mode can be omitted rather than explicitly selected;

disposition may be implemented as an independent result;

commit-point cross-owner currentness is not fully normative;

the ontology placement of the new records is unstated.

After the amendments above

The amended candidate would be ready as human-reviewable semantic authority for later human-approved planning.

It would then completely settle the intended bounded seam without selecting:

concrete policies;

hierarchy/state-machine models;

schemas or APIs;

storage or crates;

authorization mechanisms;

implementation tasks;

migration or V1 compatibility.

No further architecture decision would be required merely to begin later task-level planning. Concrete work hierarchy, action policies, authorization mechanisms, and state-owner implementations would remain proper planning inputs rather than unresolved semantic contradictions.

Companion-document normalization required
1. evaluation-authorization-capability-and-readiness-semantic-decision.md — mandatory

Its third currentness checkpoint presently says operational reliance includes fresh Engine authorization verification, and its cache/scenario language similarly treats an authorization change as generally preventing current-use reuse. 

evaluation-authorization-capabi… +1

Replace the operational-reliance checkpoint with:

Before operational reliance. The downstream owner MUST apply the focused operational-reliance decision. It always resolves the exact historical Evaluation Authorization Binding and the historical validity required by the source result. Fresh current Engine verification is required only when the admitted Operational Reliance Policy or a later Action Authorization policy explicitly declares that Binding or another Engine proposition as a present dependency. Failure of a declared present dependency refuses the governed reliance or authorization boundary without rewriting historical truth.

Replace the current-result cache row with:

Current-result cache reuse. Require the complete final evaluation closure plus every exact downstream policy-declared present-use verification. A change in Evaluation Authorization currentness invalidates current-use reuse when the admitted Operational Reliance Policy or later Action Authorization policy declares continuing-current Engine authorization. Historical validity and exact historical resolvability remain universally required.

Its “authorization changes after a pass” scenario should use the same distinction.

2. metric-gate-semantic-decision.md — mandatory

The broad invariant that “any current operational reliance requires fresh owner verification” is compatible only after it is narrowed to the verifications declared by the downstream policy; the wording that a changed authorization basis necessarily requires a fresh evaluation is currently broader than the selected rule. 

metric-gate-semantic-decision

Replace that portion with:

A later authority change, revocation, supersession, stack change, or approved escalation MUST NOT rewrite a closed result or explanation. Any present operational reliance requires exact historical resolution of the result and every fresh owner verification declared by the admitted downstream Operational Reliance Policy. Continuing-current Evaluation Authorization is required only when that policy declares it. A later Action Authorization policy may independently require current Engine verification. A fresh evaluation is required when the applicable downstream policy requires one or when changed subject, target, scope, purpose, or other exact provenance makes the historical result incongruent with the new action question.

The “authorization changes across stages” scenario should be amended correspondingly.

3. canonical-object-ontology-and-ownership-semantic-decision.md — mandatory

Add the ontology classifications from Amendment 8. The ontology already reserves downstream policies for category 2, current-use/action decisions and receipts for category 3, and attempt provenance for category 4; the new exact objects should be named explicitly. 

canonical-object-ontology-and-o…

4. posture-assurance-policy-semantic-decision.md — mandatory if the transition-specific reliance stage is accepted

The posture record correctly separates observation, recommendation, and Engine transition, but it currently proceeds from recommendation directly to separately authorized transition. 

posture-assurance-policy-semant…

Insert:

A Posture Recommendation does not establish current transition eligibility. Before a Posture Transition Action Intent or Engine action authorization may form, the Engine posture-transition responsibility MUST close one exact transition-specific Operational Reliance Basis and Determination over the exact recommendation, current expected posture target, required approvals, transition policy, and every other declared present dependency. The transition authorization and state mutation remain later independent stages.

5. authored-artifact-evaluation-relationships-semantic-decision.md — small but normative

Its Outcome and Disposition Reference definitions should reflect the atomic Operational Reliance Determination:

A Disposition Reference to an operational reliance outcome names the exact shared Operational Reliance Determination and identifies its Operational Disposition facet. It does not imply that Current-Use Eligibility and Operational Disposition are independently formed or independently current records.

The rest of its Work Specification relationship and anti-cycle semantics already compose correctly. 

authored-artifact-evaluation-re… +1

6. architecture-context.md and phase-and-slice-map.md — status normalization, not implementation authorization

Both currently say the parent/dependency owner and policy remain unselected. Once this semantic decision is human-approved, they should say:

The greenfield semantic owner for local-closeout and parent-eligibility present-use decisions is Work Progression under the approved operational-reliance decision. The concrete work hierarchy, state owner, policies, action authorization, mutation implementation, consumer, and delivery slice remain unselected and unauthorized.

This records that the semantic owner question has been decided without implying implementation. 

architecture-context

 

phase-and-slice-map

7. evaluation-identity-evidence-closure-semantic-decision.md

No substantive correction is required. It already defers current-use eligibility and the universal-versus-policy-declared Engine rule to this decision and correctly separates historical resolvability from operational authority. Only cross-link/status normalization is needed. 

evaluation-identity-evidence-cl…

8. HCM-5.2 README

Do not rewrite its implementation history to pretend Work Progression or the greenfield model is implemented. It should remain truthful historical authority for the existing Contracts-owned sibling LocalCloseoutResultV1 and the absence of parent promotion. Any ordinary navigation note must preserve that historical status rather than introduce compatibility or migration semantics. 

README

Final disposition

ADJUST.

The proposed record’s central architecture should be retained. The policy-conditional Engine rule is the correct selection, Work Progression is the correct bounded semantic owner, local and parent eligibility are properly separated from mutation, Work Specification direction is correct, and the gate-to-posture cycle is properly forward-only.

After the exact amendments above—especially exact action-question closure, uncertain-commit retry prohibition, Action Authorization Refusal, Action Attempt Identity, finite result/parent populations, explicit Engine mode selection, atomic eligibility/disposition, and commit-point owner verification—the candidate would be ready as semantic authority for later human-approved planning.
```

## Local reconciliation

The review was accepted as an `ADJUST` result. Repository truth did not conflict
with its bounded findings. The original local drafting subagent revised only
the candidate record and incorporated the complete P1/P2 correction set plus
the material P3 clarifications:

- the exact action question, including effect, scope, and every eligibility-
  relevant parameter, now closes before Operational Reliance;
- Action Authorization Refusal and Action Attempt Identity are distinct from
  Binding, executor-owned Terminal, and state-owner Mutation Receipt;
- ordinary non-idempotent retry after an uncertain effect or commit is
  prohibited until authoritative non-application or separately admitted,
  freshly authorized duplicate-safe recovery is established;
- each consumed historical-result role and parent relationship uses a finite,
  exact population boundary with complete accounting and exact proposition
  roles;
- every applicable reliance policy must explicitly choose
  `historical_validity_only` or `continuing_current`; continued Engine
  evaluation-authorization currentness remains policy-conditional while exact
  historical validity is universal and later action authorization may impose a
  stricter Engine dependency;
- eligibility and disposition are facets of one atomic Operational Reliance
  Determination;
- action authorization, target, effect, and declared through-commit
  dependencies are enforced at the mutation commit or linearization boundary;
- every newly named object is placed in the six-category ontology, and
  diagnostic directionality, advisory non-mutation, refusal replay, semantic-
  record non-recursion, and posture-transition staging are explicit.

The companion-document amendments identified by Pro were deliberately not
applied here; they are retained as input to the later normalization pass. The
revised candidate remains `PROPOSED`, authorizes no implementation or
operational effect, and has SHA-256
`43BED08E1ACDDB3DBF64B51B96DAB46AB34A13DC0B71EF924D77AC835BB898BE`.
