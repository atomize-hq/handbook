# HCM Context Resolution Authorization Binding — ChatGPT Pro Follow-up

Date: 2026-09-01

Status: **complete; repository reconciliation complete**

Task/spec: Resolve whether an HCM evaluation that claims authorized observation reach must bind exact Engine-owned authorization provenance, and choose the narrow semantic seam before amending the metric/gate and posture/assurance decision records.

Current approach: Engine remains the sole owner of Context Resolution admission and currentness. Contracts owns evidence, verdict, metric, and gate semantics. The candidate approach is a purpose-bounded Engine-owned Evaluation Authorization Binding, derived from one exact admitted `ContextResolutionEnvelope`, with opaque lineage to that envelope and enough stable provenance for an evaluation to bind and for Engine to verify.

ChatGPT configuration verified before submission:

- Existing bounded deep-review conversation: https://chatgpt.com/c/6a964c19-9190-83ea-8ca0-9bbde5d47a0a
- Chat surface
- Pro enabled at 5 of 5
- GPT-5.6 Sol selected
- Follow-up duration reported by ChatGPT: 28m 22s

Bounded current-source packet supplied:

1. `crates/engine/src/context_resolution.rs`
2. `crates/contracts/src/evaluation_run.rs`
3. `crates/contracts/src/evaluation_identity.rs`
4. `crates/contracts/src/evidence_admission.rs`
5. `crates/contracts/src/claim_verdict.rs`
6. `crates/dock/src/protocol.rs`
7. `docs/research/hcm-context-resolution-posture-gate-ux-chatgpt-pro-deep-review-2026-09-01.md`

Advisory only; verify against local project truth and authoritative docs; do not reduce scope without user approval.

## Local repository reconciliation

The focused answer is consistent with the supplied implementation sources on the key point: current Contracts identity binds the requested six-coordinate `ResolutionEnvelopeV1`, while Engine's admitted `ContextResolutionEnvelope` carries materially richer identity and private authority-admission/currentness state. Contracts therefore cannot infer actual Engine authorization from coordinate equality, evidence admission, or Dock's strict ceiling equality.

The recommended hybrid is locally coherent:

- the cross-component object is a purpose-bounded, Engine-owned Evaluation Authorization Binding;
- it contains or transitively binds only evaluation-relevant stable facts;
- it carries opaque exact lineage to the underlying admitted Engine envelope;
- Engine alone issues it and evaluates admission/currentness;
- Contracts binds its exact identity, verifies structural congruence within its own domain, and refuses evaluation-context closure when required Engine verification is absent or negative;
- the full Engine envelope and raw/private authority material do not become Contracts DTOs.

The answer also correctly narrows document placement. The current metric/gate record must state the consumer-side closure invariant, and the posture/assurance record must state that assurance neither grants authority nor selects a weaker proof plan from current capability. The full authorization lifecycle belongs with the proposed Evaluation Capability and Readiness semantic decision, renamed or scoped to include authorization. A separate fifth decision is not justified by the current boundary.

One wording caution must carry into editing: “Contracts verifies” must never imply that Contracts can determine Engine currentness. Contracts may exact-check identifiers and relationships that it owns; any claim that the authorization is current must come from Engine verification. Physical APIs, persistence, registries, status vocabulary, and host enforcement remain deliberately undecided.

## Prompt

~~~text
Focused continuation review: Engine authorization provenance in the HCM evaluation context

This is a continuation of your prior deep HCM Context Resolution / posture / assurance / metric-gate / UX review in this same conversation. Read all seven newly attached current-source files completely before answering. The earlier attached architecture/spec packet and your prior review remain relevant context, but these newly attached files are authoritative for this specific seam and resolve the packet insufficiency you identified.

Task boundary

We are not yet editing the two candidate semantic decision records or performing implementation planning. We need to settle whether the current semantic work must explicitly define how an evaluation proves that its requested Context Resolution was actually admitted by Engine, and what the narrow cross-component seam should be. Give guidance precise enough to support later amendments, but do not design Rust APIs, schemas, persistence, transports, Substrate integration, or compatibility migration.

Repository-checked current facts

1. Engine owns Context Resolution semantics. In context_resolution.rs, ContextResolutionEnvelopeInput binds the objective, resolved profile, stack, active level, six dimensions, parent, constraint inputs, mutation rules, escalation triggers, and candidate binding. The admitted ContextResolutionEnvelope then binds exact identity plus admitted authority and authority admission. It exposes an exact binding. The full object also contains Engine-private/raw authority state and is not a public Contracts DTO.

2. Contracts owns evidence/evaluation/verdict/gate semantics and Engine must not depend back on Contracts. evaluation_run.rs currently makes requested_resolution part of evaluation identity, but does not bind the complete admitted Engine ContextResolutionEnvelope or an Engine-issued authorization receipt.

3. evaluation_identity.rs defines a serializable ResolutionEnvelopeV1 containing all six coordinates and the exact stack identity. EvidenceCandidateInputV1 carries one request_resolution and one actual_resolution. These coordinate values express requested/observed reach; by themselves they do not prove that Engine admitted the request for the exact subject, objective, authority constraints, parent context, or current run.

4. evidence_admission.rs computes the effective minimum over request, actual, and an optional producer ceiling. claim_verdict.rs rechecks Resolution per claim. Neither validates the request against an Engine-owned authorization binding.

5. Dock has a strict first-party protocol policy requiring requested_resolution to equal its manifest ceiling, and completion requires actual == request ceiling == requested. This is a producer operating contract; it is not proof of Engine authorization and should not be universalized.

6. Same six coordinate values and stack identity could occur under different subjects, objectives, parents, constraints, authority admissions, currentness states, or superseding escalations. Treating them as the same authorization would therefore be unsafe and replay-ambiguous.

7. This repository is greenfield. Existing HCM-5.2 shapes are evidence about the current model, not compatibility obligations. Do not add V1/V2 or legacy architecture solely to preserve them.

The decision to review

The candidate seam is: an evaluation context that claims its requested observation reach was authorized must bind exact Engine-owned authorization provenance. Contracts must not reconstruct or duplicate Engine admission semantics. The likely choices are:

A. Contracts binds an exact reference/fingerprint to the complete admitted ContextResolutionEnvelope; Engine privately resolves or re-admits that reference when needed.

B. Engine issues a purpose-bounded, Engine-owned Evaluation Authorization Binding / receipt / projection that exposes only the stable facts and exact provenance Contracts needs, while keeping the full envelope and private authority state inside Engine.

C. A materially better model you can justify from the supplied source.

Please determine:

1. Is settling this seam an approval blocker for the current amendments to the metric/gate and posture/assurance semantic decisions, or may it be deferred to the proposed Evaluation Capability and Readiness decision without leaving the two current records ambiguous?

2. Choose A, B, C, or a carefully delimited hybrid. Explain why it preserves Engine ownership and the dependency direction while allowing Contracts to make only the claims it is authorized to make.

3. Define the semantic owner and lifecycle:
   - who issues the binding;
   - at what point relative to Engine admission, readiness, evaluation formation, evidence collection, and gate calculation;
   - who verifies it;
   - whether verification means exact identity comparison, Engine resolution, Engine re-admission/currentness check, or some combination;
   - what creates a fresh binding rather than reusing one.

4. State the minimum semantic content/invariants that must be bound. Consider subject/scope, objective, exact stack semantics, admitted six-coordinate bounds, constraint identity, parent context, authority admission, mutation/escalation limits, issued/current/superseded status, intended evaluation use, and exact request relationship. Separate required provenance from details that must remain private or be referenced opaquely.

5. Clarify currentness, replay, cache, and escalation behavior:
   - exact retry of the same evaluation;
   - same coordinate labels under a changed stack;
   - same coordinates under a different subject/objective/parent/constraint set;
   - authorization superseded or revoked;
   - approved escalation;
   - old canonical results and explanations after authorization changes.

6. Separate these roles rigorously:
   - Engine authorization ceiling;
   - requested observation reach;
   - producer capability;
   - Dock/adapter operating ceiling;
   - actual observed reach;
   - per-claim evidence adequacy;
   - explanation disclosure/viewer authority.
   Say whether any may safely share a serialized shape while remaining different semantic types.

7. Recommend exactly where this semantic decision belongs:
   - clauses required now in metric-gate-semantic-decision.md;
   - clauses required now in posture-assurance-policy-semantic-decision.md;
   - content owned by the proposed Evaluation Capability and Readiness decision;
   - whether a separate fifth decision is genuinely necessary.
   Prefer the smallest document set that leaves no owner or invariant underspecified.

8. Provide concise normative clause candidates and discriminating scenarios for the current two-record amendments. Do not supply implementation types or code.

9. Explicitly identify what the supplied source establishes versus what you infer or recommend. Use these labels where applicable:
   [REPOSITORY TRUTH]
   [DERIVED INVARIANT]
   [DESIGN RECOMMENDATION]
   [PACKET INSUFFICIENT]

10. Give a final verdict of APPROVE AS-IS, ADJUST, or PIVOT for this proposed seam, followed by the smallest safe next sequence before we begin the annotated document work.

Constraints

- Preserve the established architecture: posture establishes the engineering standard; assurance selects the proof plan; Engine grants what the run may do; evaluation requests reach within that grant; producers declare capability/operating bounds and record actual reach; Contracts determines evidence/verdict/gate truth; projection changes presentation only; disclosure controls revealability; enforcement controls downstream action.
- Do not let assurance requirements grant authority or let current capability relax selected requirements.
- Do not make all six Context Resolution coordinates ambient evidence-quality minima.
- Do not broaden into host/Substrate enforcement design, UI layout, public APIs, exact schemas, migration, localization, or general Phase-6 planning.
- Do not introduce legacy/V1/V2 obligations in this greenfield repository.
- Do not reduce the task or replace it with an easier alternative.

Given your answer, what is the fastest way to execute or continue while fully complying with the original task and constraints? Do not reduce scope. Provide the exact order of operations, minimum files or areas to touch, commands, validation, and what to defer until after the next working slice.
~~~

## Complete ChatGPT Pro answer

~~~text
Worked for 28m 22s
Focused continuation review: Engine authorization provenance
Verdict: ADJUST

[DESIGN RECOMMENDATION] Choose a carefully delimited hybrid: B is the cross-component seam; A is mandatory opaque lineage beneath it.

Engine should issue a purpose-bounded, Engine-owned Evaluation Authorization Binding derived from one exact, currently admitted ContextResolutionEnvelope. Contracts should bind that authorization object—not the full private envelope—as part of the evaluation context. The authorization binding must transitively identify the underlying envelope by exact reference and fingerprint, and Engine must remain the only component that resolves, re-admits, and evaluates its authority/currentness.

In compact form:

exact admitted Engine ContextResolutionEnvelope
    -> Engine-owned Evaluation Authorization Binding
         binds exact evaluation purpose, subject/scope, requested reach,
         underlying envelope, authority provenance, and issuance basis
    -> Contracts evaluation context binds that exact authorization binding
    -> Engine verifies identity/currentness
    -> Contracts evaluates evidence, claims, metrics, and gate

This is not a new authority owner and does not require a fifth semantic decision.

1. Is this seam an approval blocker?
Direct answer

Yes, the core seam is an approval blocker for the current two-record amendments. It cannot be deferred wholesale to the Evaluation Capability and Readiness decision.

The amendments must settle now that:

requested coordinates do not prove authorization;

an evaluation claiming authorized reach must bind an exact Engine-owned, purpose-bounded authorization provenance object;

Contracts may exact-check congruence but may not reconstruct Engine admission;

missing, mismatched, stale, superseded, or unverifiable authorization provenance is an evaluation-context closure failure before claim, metric, or gate truth;

a later escalation creates a fresh authorization binding and fresh evaluation context.

The companion decision may own the fuller lifecycle: issuance workflow, readiness, producer selection, diagnostic intent, currentness checkpoints, retries, supersession, and cache rules.

Approval distinction
Approval question	Answer
May the two amendments be drafted before the companion decision?	Yes.
May they leave “Engine authorization binding or projection” as an unresolved A/B choice?	No.
May they normatively select the purpose-bounded Engine binding and defer detailed mechanics?	Yes.
Are the two amended records alone the complete input to implementation planning?	No. The authorization/capability/readiness companion remains a hard semantic predecessor.

[REPOSITORY TRUTH] The metric/gate proposal already says evaluation-context closure is a precondition to semantic evaluation and that wrong-scope, substituted, omitted, or unresolved dependencies refuse before claim, metric, or gate truth. An unproven authorization assertion therefore cannot consistently be treated as an ordinary evidence limitation. 

metric-gate-semantic-decision(2)

[DERIVED INVARIANT] Deferring the whole authorization seam would leave a material hole in that closure: a caller could provide a valid six-coordinate value without proving that Engine admitted it for this subject, objective, parent, constraints, authority basis, and use.

2. What the newly supplied source establishes
Engine’s existing authority object is richer than a coordinate envelope

[REPOSITORY TRUTH] ContextResolutionEnvelopeInput binds the objective, exact profile and stack, active level, six dimensions, parent, ordered constraint inputs, mutation rules, and escalation triggers. Those values participate in the envelope fingerprint. The materialized envelope separately retains Engine authority and ContextResolutionAuthorityAdmission. 

context_resolution +1

That separation is decisive:

envelope exact binding
    binds semantic envelope inputs

private admitted envelope object
    additionally carries authority admission and currentness witness

The exact envelope fingerprint does not itself contain the private authority admission or its currentness witness. It proves which semantic envelope was materialized, but a bare copied ref/fingerprint does not independently prove the current authority basis under which Engine admitted it.

[REPOSITORY TRUTH] ContextResolutionAuthorityAdmission separately binds an authority use, exact authority subject, and authority-binding fingerprint. Its currentness check revalidates repository identity, exact authority artifact bytes, semantic binding, registry state and head, credentials, and publisher authority. 

context_resolution

[REPOSITORY TRUTH] Engine already has a private bounded projection precedent: ProjectionAuthorityView exposes the exact envelope, profile, stack, active level, coordinate values, and ranks only after the envelope’s authority admission passes a currentness check. That view does not, however, bind an evaluation subject, intended use, requested-reach relation, or the full authority provenance needed here. 

context_resolution

Contracts currently binds a request, not its Engine authorization

[REPOSITORY TRUTH] Current run preparation accepts a Contracts subject and requested ResolutionEnvelopeV1, and the resulting run fingerprint binds that requested value. It has no complete Engine envelope binding or Engine-issued evaluation authorization provenance. 

evaluation_run +1

[REPOSITORY TRUTH] ResolutionEnvelopeV1 contains only an exact stack binding and the six coordinates. It has no objective, parent, constraint closure, authority admission, mutation limits, escalation lineage, intended use, or Engine currentness proof. 

evaluation_identity

[REPOSITORY TRUTH] Evidence candidates bind request and actual coordinate envelopes, while canonical evidence adds effective Resolution, Contracts currentness, source currentness, and evidence-source authority. None of those fields proves Engine authorization of the evaluation request. 

evaluation_identity +1

[REPOSITORY TRUTH] Evidence admission presently computes the minimum of request, actual, and optional producer ceiling, then compares it with claim minima. It does not check that the request is within an exact Engine-admitted evaluation authorization. 

evidence_admission

Dock equality is unrelated to Engine authorization proof

[REPOSITORY TRUTH] The Dock run binding copies the Contracts subject and requested coordinates. Its first-party protocol requires requested Resolution to equal the manifest operating ceiling, and completed output requires actual Resolution to equal both. 

protocol +2

That proves conformity to one producer operating contract:

Dock request = Dock operating ceiling = Dock completed actual

It does not prove:

Engine admitted this reach
for this evaluation subject
under this objective, parent, constraints, and authority state
3. Choice among A, B, and C
Assessment
Choice	Assessment
A. Bind only the complete envelope ref/fingerprint	Useful and required as lineage, but insufficient as the primary cross-component seam. A bare binding omits the purpose-specific evaluation relationship and does not itself carry the private authority admission/currentness proof. Making Contracts consume the full private object would also over-couple it to Engine internals.
B. Engine issues a purpose-bounded evaluation authorization binding	Correct primary seam. It lets Engine attest the exact request relationship while revealing only stable, evaluation-relevant facts.
C. Another owner or neutral authorization domain	Unnecessary and hazardous. It would either duplicate Engine admission semantics or introduce another authority owner between Engine and Contracts.
Selected hybrid	B as the only cross-component authority object, with A as mandatory opaque provenance inside B.
Why pure A is not the best model

Pure A has two possible interpretations, and neither is ideal.

A1: Contracts receives the full Engine object

That would expose or couple Contracts to:

Engine-private authority state;

currentness witnesses;

mutation-rule internals;

repository authority machinery;

full constraint and escalation implementation details.

It would make the Contracts DTO boundary track Engine’s internal authority representation.

A2: Contracts receives only the envelope ref/fingerprint

That avoids leakage but leaves Contracts with no stable proof that:

this evaluation subject is covered;

this intended evaluation use is covered;

its requested reach was admitted rather than merely copied;

the relevant authority admission was current;

the binding was not transferred from another evaluation purpose.

Engine could be called each time to recompute that relation, but the result of that owner decision would then be absent from the canonical evaluation identity. Once that result is made exact and provenance-bound, it is effectively choice B.

Why B must retain A lineage

A standalone projection that omits the exact underlying envelope would create another potential source of Resolution truth. Therefore the Evaluation Authorization Binding must transitively bind:

the full envelope’s exact ref/fingerprint;

the Engine authority/admission basis from which it was issued;

the exact requested-reach relationship Engine established.

Contracts consumes the projection. Engine resolves the underlying envelope.

Ownership result
Engine
  owns full envelope
  owns admission semantics
  owns authorization relation
  issues and verifies Evaluation Authorization Binding

Contracts
  binds exact Engine result
  checks local evaluation-context congruence
  owns evidence/claim/metric/gate consequences
  does not reconstruct authorization

Dock / producer
  conforms to its operating contract
  emits capability and observation facts
  does not prove or issue Engine authorization

SDK / transports
  preserve exact owner truth
  do not verify or reinterpret it

This preserves the established inward dependency direction in which Contracts may depend on narrow Engine facilities while Engine does not depend on Contracts. 

README

4. Semantic owner and lifecycle
Recommended lifecycle
Order	Stage	Semantic action	Owner
1	Posture and assurance selection	Resolve the engineering standard and exact proof plan without inspecting current run capability	Posture/assurance owners
2	Prospective evaluation formation	Establish exact subject/scope, objective, requested reach, intended use, and opaque evaluation-purpose basis	Evaluation coordination responsibility; physical placement deferred
3	Engine envelope admission	Resolve or re-admit the full current ContextResolutionEnvelope and all Engine-owned authority/constraint relationships	Engine
4	Authorization issuance	Establish that the exact requested reach is admitted for the exact evaluation purpose and issue the Evaluation Authorization Binding	Engine
5	Readiness	Compare the already-selected proof plan with the authorization binding and producer capability/operating facts	Evaluation Authorization, Capability, and Readiness responsibility
6	Evaluation-context formation	Bind the exact Engine authorization, request, requirements, contract, subject, policies, and use intent	Contracts/evaluation-context owner
7	Evidence collection	Execute only against the exact request and bounded authorization projection	Producer/Dock/adapter
8	Evidence admission	Confirm candidate/run/request congruence and obtain Engine verification appropriate to the admission boundary	Contracts plus Engine verification
9	Evaluation closure	Recheck relevant currentness before closing claim, metric, and gate truth	Contracts plus Engine verification
10	Historical replay or explanation	Resolve immutable historical provenance without treating it as current execution authority	Engine verifies provenance; Contracts/projection preserves historical truth
Who issues the binding?

[DESIGN RECOMMENDATION] Engine alone.

The caller may request authorization. The readiness owner may consume it. Contracts may require it. None may mint, modify, broaden, or reinterpret it.

When is it issued?

After Engine has admitted or re-admitted the complete envelope, but before:

readiness can claim that an allowed reach exists;

Contracts can form an evaluation context that claims authorization;

any producer is launched;

any evidence is collected.

This avoids two cycles:

readiness does not manufacture authorization;

Contracts does not have to produce a final evaluation run before Engine can attest the request.

The Engine binding can consume an opaque exact evaluation-purpose/formation identity without understanding Contracts semantics. When posture influenced selection, that purpose identity should transitively identify the exact Resolved Assurance Requirements. Binding it as purpose provenance does not let the requirements grant authority.

Who verifies it?

Verification is a combination of two responsibilities.

Contracts performs exact congruence checks

Contracts may verify that:

the evaluation subject/scope equals the authorization binding’s subject/scope;

the requested coordinate envelope exactly equals the authorized request;

the stack binding is exact;

the intended use is exact;

the evaluation-purpose binding is exact;

the authorization binding itself has not been substituted.

These are equality and closure checks over Engine-produced truth.

Engine performs authority verification

Engine must verify:

the authorization binding’s exact identity and issuance lineage;

the exact underlying envelope;

the authority admission that supported issuance;

the request-within-envelope decision;

currentness or historical validity, depending on the operation;

supersession or other not-current conditions.

Contracts must not reproduce the rank, parent, constraint, mutation, authority, or escalation decision that Engine used.

The architecture already distinguishes structurally valid input from owner-trusted admission: canonical or well-formed bytes do not establish the owner decision that authorized them. 

architecture-context

Resolution versus re-admission
Verification kind	Required for	Meaning
Exact identity resolution	All uses	The binding and its underlying envelope are the exact issued objects
Historical issuance verification	Historical replay and explanation	Engine validly issued the authorization under the recorded authority basis
Currentness/re-admission check	New execution, evidence collection, and closing a new current evaluation	The authorization remains current and unsuperseded at the required boundary
Contracts congruence check	Evaluation formation, evidence admission, gate closure	This Contracts context is exactly the purpose for which the Engine binding was issued

Exact identity comparison alone is insufficient. Currentness checking alone is also insufficient if the binding has been transferred to another subject or purpose.

5. Minimum semantic content and privacy boundary
Content that must be bound

The following is semantic content, not a proposed schema.

Content	Required treatment	Reason
Authorization-binding identity	Exact Engine-owned ref/fingerprint and semantic owner identity	Establishes which owner result the evaluation consumes
Underlying envelope	Exact opaque ContextResolutionEnvelope ref/fingerprint	Preserves full Engine lineage and prevents a second Resolution authority
Evaluation-purpose identity	Exact non-transferable purpose/formation binding	Prevents reuse for another run, proof plan, or use
Subject and scope	Exact subject snapshot and applicable scope binding	Same coordinates do not authorize another subject
Objective	Exact Engine objective identity as understood by the admitted envelope	Prevents reuse for a different operation or goal
Resolved profile and stack	Exact ref/version/semantic fingerprint	Coordinates have no meaning outside their exact stack
Active level and admitted six-coordinate bounds	Exact Engine-produced projection	Makes the permitted reach inspectable without exposing the full envelope
Requested observation reach	Exact requested coordinate envelope	Establishes the request that was evaluated
Engine request relationship	Exact owner conclusion that the request is admitted within the underlying envelope	Contracts must not reconstruct this decision
Parent context	Exact parent binding or explicit root status	Prevents reuse across different parent authority
Constraint closure	Complete ordered exact closure or an independently resolvable content-addressed root	Prevents omitted or substituted constraints
Authority-admission provenance	Exact opaque authority-basis identity, repository-identity fingerprint, and issuance currentness basis	Proves which authority state supported issuance without exposing credentials
Mutation, memory, validation, and escalation closure	Transitively bound through the full envelope; purpose-relevant authorization decisions exposed only where needed	Prevents coordinate-only inference of operational permission
Intended evaluation use	Exact decision_seeking or diagnostic provenance, or another later-approved closed vocabulary	Prevents diagnostic authorization from being relabeled
Issue boundary	Engine semantic time and exact authority/currentness basis observed at issuance	Supports replay and historical explanation
Supersession relation	Separate exact Engine-owned relation when one exists	Preserves immutability of the original binding
Recheck requirements	Exact statement of which boundaries require current verification	Prevents a historical binding from silently acting as current authority
What should remain opaque or private

The cross-component binding should not expose:

raw credentials or authenticator material;

challenge/response bytes;

the physical repository path;

private Engine witness objects;

raw authority capsule internals that Contracts does not need;

Engine caches, counters, or resolver state;

unrestricted mutation selector bodies;

raw constraint or escalation evidence when exact references and bounded outcomes suffice;

the full private ContextResolutionEnvelope object;

host capability or Substrate state.

[REPOSITORY TRUTH] Current Engine authority admission already retains a private currentness witness and performs substantial owner-side revalidation rather than exposing that machinery as ordinary DTO state. 

context_resolution

Issued, current, and superseded are different kinds of fact

[DERIVED INVARIANT] Do not put a mutable current: true/false field inside an immutable authorization binding.

Use this semantic split:

Fact	Lifecycle
Issued	Immutable historical fact: Engine issued this binding under this exact basis
Current	Owner observation made at a specified verification boundary
Superseded	Separate Engine-owned relation to a successor request, envelope, or binding
Revoked/not current	Owner currentness outcome; exact status vocabulary belongs to the companion decision

The existing Engine source supports current versus historical authority validation and has an exact escalation Superseded disposition, but it does not establish a unified public evaluation-authorization status vocabulary. 

context_resolution +1

[PACKET INSUFFICIENT] The supplied source does not establish an existing persisted or independently retrievable Evaluation Authorization Binding, nor a current public distinction among “revoked,” “stale,” and every possible not-current cause. Those are new semantic decisions, not current implementation facts.

6. What creates a fresh authorization binding?

A binding may be reused only for an exact retry of the same evaluation-purpose request while its required currentness basis remains exact.

A fresh binding is required when any of the following changes:

evaluation subject or subject snapshot;

Posture Scope or evaluation scope;

objective;

Resolved Assurance Requirements or opaque evaluation-purpose binding;

decision-seeking versus diagnostic intent;

requested observation reach;

exact Resolution stack reference or fingerprint;

active level or any admitted coordinate;

resolved profile;

parent context;

constraint closure;

authority-admission basis or relevant authority head;

applicable mutation, memory, validation, or escalation closure;

underlying admitted envelope;

approved escalation;

a deliberately new evaluation identity rather than an exact retry.

A changed display density or viewer disclosure authorization does not create a new evaluation authorization binding.

7. Currentness, replay, caches, and escalation
Case	Required semantic behavior
Exact retry before evaluation closure	The exact binding may be reused only if the complete purpose/request identity is unchanged and Engine verifies the required authority/currentness basis. Otherwise issue a fresh binding or refuse.
Deliberately new evaluation with the same apparent inputs	Use a distinct purpose/evaluation identity and fresh binding; do not silently transfer the earlier authorization.
Delivery retry of an already closed result	Replay the exact result. Do not reacquire authorization or rerun evidence collection merely to redeliver it.
Same coordinate labels under a changed stack fingerprint	Fresh envelope, authorization binding, readiness result, and evaluation context. Matching labels imply no compatibility.
Same stack and coordinates but different subject/objective	Fresh binding. Reuse refuses.
Same coordinates but different parent or constraints	Fresh binding. Reuse refuses.
Authorization becomes not current before evidence collection	Do not launch the producer. Readiness or formation becomes held/refused according to its layer.
Authorization becomes not current after collection but before evaluation closure	Do not close a new current gate under the stale context. Evaluation-context closure refuses; it is not an ordinary claim indeterminacy.
Authorization changes after a gate is closed	The old gate and canonical explanation remain immutable historical truth for their exact evaluation boundary. They do not become current authorization for a new effect.
Approved escalation	Engine admits the proposed envelope and issues a fresh authorization binding. Perform fresh readiness and form a fresh evaluation context. The prior binding, evidence, and result are not upgraded.
Refused escalation	Existing authorization remains unchanged if otherwise current; requirements remain unchanged.
Superseded escalation request	Follow the exact successor relation. The superseded request cannot later authorize a run.
Current result cache	Key by the complete evaluation closure, authorization-binding identity, and required owner currentness basis. Changed authorization state invalidates current-use reuse.
Historical result cache	Immutable result identity may remain reusable for exact historical display or replay; it must not be represented as current authorization.
Readiness cache	Also binds requirements, subject/applicability basis, producer capability basis, operating ceiling, use intent, and authorization currentness.
Closed-result immutability

A current view of an older result may display:

Gate result: pass
Authorized and evaluated under: EAB-17 at T1
Current authorization status: superseded at T2
Current operational use: requires a new evaluation

It must not mutate the old gate into fail or indeterminate, and it must not continue to label the old binding current.

8. Rigorous separation of the seven roles
Role	Owner/source	What it proves	What it does not prove
Engine authorization ceiling	Engine Evaluation Authorization Binding derived from an admitted full envelope	What this exact evaluation purpose was allowed to request under Engine authority	Producer capability, actual observation, claim adequacy, viewer disclosure
Requested observation reach	Evaluation formation request, admitted by Engine	What this evaluation asked to observe	That Engine admitted it, unless paired with the Engine binding
Producer capability	Producer capability declaration plus independent admission/currentness	What the producer can in principle observe or measure	Permission to exercise that capability
Dock/adapter operating ceiling	Admitted manifest/host operating contract	Maximum reach this exact execution path will operate under	Engine authorization, actual observation, claim satisfaction
Actual observed reach	Producer execution result/source observation	What was actually observed	What was requested, authorized, or sufficient
Per-claim evidence adequacy	Contracts claim evaluator under exact policy	Whether admitted observation is adequate for a particular claim	Authorization, producer permission, global run quality
Explanation disclosure/viewer authority	Disclosure authority used by projection	What this viewer may be shown	What the evaluator was authorized to observe, or how rigorous the evaluation was
Presentation density	Explanation-projection request	How much of the same authorized truth to display	Any form of Context Resolution or authorization
May any share a serialized shape?

[DESIGN RECOMMENDATION] Yes, but only at the structural-value layer.

The following may share a common primitive representing:

exact stack identity + six coordinate values

Engine admitted coordinate bound;

requested observation reach;

producer capability;

Dock operating ceiling;

actual observation;

perhaps a complete claim requirement when all six dimensions are explicitly relevant.

They must remain different semantic types or role-tagged bindings with different provenance and non-substitutability rules.

A bare common coordinate value must never cross an owner boundary as proof of its role.

In particular:

an AuthorizedCeiling is not a RequestedReach;

a ProducerCapability is not an OperatingCeiling;

an OperatingCeiling is not ActualObservedReach;

ActualObservedReach is not ClaimAdequacy;

viewer disclosure authorization is not the evaluation authorization;

presentation density should not reuse the coordinate shape at all.

Claims should normally declare only the Context Resolution dimensions relevant to their observation-adequacy rule. They must not acquire ambient minima over all six dimensions.

9. Exact document placement
Smallest complete document set
Document	What it must own
metric-gate-semantic-decision.md	Mandatory Engine authorization provenance in evaluation closure; Contracts consumer invariants; refusal boundary; transitive result provenance
posture-assurance-policy-semantic-decision.md	Requirements/authority orthogonality; no capability-driven relaxation; no authority grant; fresh evaluation after escalation
evaluation-authorization-capability-and-readiness-semantic-decision.md	Full Engine binding lifecycle, purpose formation, currentness, readiness, producer capability versus operating ceiling, diagnostic use, retry, cache, supersession, escalation
evaluation-explanation-projection-semantic-decision.md	Density, disclosure, viewer authority, lossiness, drill-down, canonical-result immutability
Separate fifth decision	Not needed

I recommend expanding the prior companion title from Evaluation Capability and Readiness to Evaluation Authorization, Capability, and Readiness. Authorization issuance, capability comparison, and readiness form one causal seam:

allowed reach
    + requested reach
    + producer capability/operating bounds
    + selected requirements
    -> readiness

Splitting authorization into a fifth document would divide one lifecycle and force readiness to depend on an underdefined authority object.

Clauses required now in the metric/gate record

The record must settle:

the named Engine-owned authorization-binding concept;

its mandatory underlying envelope lineage;

exact subject/scope/objective/use/request congruence;

Engine ownership of admission and currentness;

Contracts’ right to perform equality and closure checks only;

refusal before semantic truth when authorization provenance is absent or invalid;

transitive binding through evidence, verdict, metrics, gate, and canonical explanation;

historical immutability after later authorization changes;

explicit non-equivalence of Dock equality and Engine authorization.

It need not settle:

exact APIs or schemas;

physical storage;

resolver signatures;

transport;

cache implementation;

readiness algorithms.

Clauses required now in the posture/assurance record

The record must settle:

Assurance Requirements neither issue nor grant Engine authorization;

current authorization, producer capability, and readiness cannot select a lighter Assurance Policy;

Resolved Assurance Requirements may select proof obligations carrying observation requirements but not the run’s granted reach;

evaluation conformance separately requires an exact Engine authorization binding;

failure to obtain authorization does not rewrite or weaken the requirements;

escalation produces a fresh binding and evaluation, not a mutation of the proof plan or prior result.

The posture record should not duplicate the authorization binding’s full lifecycle.

Content owned by the companion decision

The companion should own:

evaluation-purpose formation;

Engine issuance of the Evaluation Authorization Binding;

minimum public projection and opaque lineage;

historical versus current verification;

readiness states;

decision-seeking versus diagnostic intent;

producer capability versus host/Dock operating ceiling;

currentness checkpoints;

exact retry and fresh-run rules;

escalation and supersession;

cache invalidation;

output authority of a diagnostic evaluation;

advisory CLI versus actual host-enforcement boundary, without designing Substrate.

10. Concise normative clause candidates
For metric-gate-semantic-decision.md
Engine authorization provenance

Engine authorization provenance. An evaluation context that represents its requested Context Resolution as authorized MUST bind one exact, Engine-owned Evaluation Authorization Binding. That binding MUST be issued from one exact admitted ContextResolutionEnvelope, transitively bind that envelope’s exact identity and Engine authority/currentness provenance, and bind the exact evaluation subject or scope, objective, intended evaluation use, Resolution stack semantics, requested observation reach, and Engine-owned determination that the request is admitted for that purpose.

Owner boundary

Authorization ownership. Engine alone owns envelope admission, authorization, non-expansion, authority/currentness, and supersession semantics. Contracts MAY require and exact-check congruence between the evaluation context and an Engine-produced authorization binding. Contracts MUST NOT reconstruct, approximate, broaden, or replace the Engine authorization decision from coordinate values, stack ranks, producer capability, Dock limits, or actual observation.

Context-closure rule

Authorization closure. Missing, malformed, substituted, wrong-subject, wrong-scope, wrong-objective, wrong-use, wrong-stack, mismatched-request, stale, superseded, or otherwise unverifiable authorization provenance is an evaluation-context closure failure. It refuses before claim evaluation, metric computation, or semantic-gate truth. It MUST NOT become ordinary evidence insufficiency, not_computable, indeterminate, or not_applicable.

Result provenance and currentness

Transitive provenance. The evaluation identity MUST bind the exact Evaluation Authorization Binding and the exact authorization-currentness basis required at evaluation closure. Canonical evidence, claim evaluations, metrics, and gate results MAY bind it transitively through the exact evaluation context rather than duplicating its raw content. Canonical explanation MUST expose the decisive authorization provenance and any authorization-related refusal.

Historical immutability

Authorization change after closure. A later authority change, revocation, supersession, stack change, or approved escalation MUST NOT rewrite a closed evaluation or gate result. A new current evaluation requires fresh Engine verification and, whenever the authorization purpose or basis changed, a fresh authorization binding and evaluation context.

Producer and Dock non-authority

Producer non-authority. Producer capability, a Dock or adapter operating ceiling, equality between request and operating ceiling, and actual observed Resolution do not establish Engine authorization. They remain independently owned inputs to readiness, evidence admission, and claim evaluation.

For posture-assurance-policy-semantic-decision.md
Requirement/authorization separation

Assurance does not grant authority. An Assurance Policy and Resolved Assurance Requirements select proof obligations. They MUST NOT issue, broaden, infer, or imply Engine Context Resolution authority. A selected observation requirement remains unchanged when the current evaluation authorization, requested reach, producer capability, operating ceiling, actual observation, or readiness is lower than that requirement.

No capability-driven selection

Current capability is not a selector input. An Evaluation Authorization Binding, requested observation reach, producer capability, Dock operating ceiling, actual observation, and readiness result MUST NOT select a lighter Assurance Policy or remove or weaken a resolved requirement. These facts determine whether already-selected requirements can be attempted or established, not which requirements govern.

Separate evaluation conformance

Authorization as separate evaluation provenance. Exact realization of Resolved Assurance Requirements is necessary but not sufficient to form an authorized evaluation context. The evaluation MUST also bind a separately issued, exact Engine Evaluation Authorization Binding congruent with the evaluation subject or scope, objective, intended use, and requested reach. The requirements identity does not contain or replace the run’s granted authority.

Escalation

Fresh context after escalation. Approval of a Context Resolution escalation creates a new Engine authorization basis and requires a fresh Evaluation Authorization Binding, readiness assessment, and evaluation context. It does not mutate the Resolved Assurance Requirements, retroactively authorize prior evidence, or upgrade a prior semantic decision.

11. Discriminating amendment scenarios
Scenario	Expected outcome	Defect exposed
Coordinate-only request. A run binds a valid stack and six requested coordinates but no Engine authorization binding.	Evaluation-context formation refuses before claim, metric, or gate truth.	Coordinates are not authorization proof.
Same coordinates, different subject. An authorization for subject A is attached to an otherwise identical evaluation of subject B.	Exact congruence fails; formation refuses.	Authorization is non-transferable.
Same coordinates and subject, different objective or parent.	Fresh binding required; reuse refuses.	Envelope context is more than coordinate values.
Constraint omission. A caller supplies a binding from an envelope admitted under a different constraint closure.	Engine verification or Contracts congruence refuses.	Constraint provenance cannot disappear.
Dock equality without Engine authorization. Request, manifest ceiling, and completed actual are all equal.	Candidate may satisfy Dock protocol structure but cannot enter an authorized evaluation context without the Engine binding.	Producer operating policy does not grant authority.
Authorization superseded before collection.	Producer is not launched; no evidence is collected under the superseded binding.	Readiness/currentness is not gate truth.
Authorization superseded after evidence but before closure.	The old context cannot produce a new current gate; formation/closure refuses and a fresh binding/context is required.	Mid-run authority changes do not become ordinary claim insufficiency.
Authorization changes after a closed pass.	The pass remains immutable historical truth; a current view marks its authorization as no longer current; no new action may rely on it without fresh policy/currentness.	Currentness does not retroactively rewrite truth.
Approved escalation.	New envelope, authorization binding, readiness result, and evaluation context; old result unchanged.	Escalation is not in-place expansion.
Hardened proof plan, low authorization.	Hardened requirements remain selected. Readiness may be known-limited or unknown; no easier policy is chosen.	Assurance does not shrink to capability.
Viewer has broader disclosure authority.	More explanation may be shown, but authorization, evidence, and gate identity remain unchanged.	Viewer authority is not evaluation authority.
Changed stack fingerprint with identical labels.	Fresh authorization and evaluation identities; no compatibility inference.	Coordinate labels do not carry stack semantics.
Exact delivery replay after supersession.	Return the same historical result and explanation, annotated as historical where current status is requested; do not rerun collection.	Result replay is not execution reauthorization.
12. What is now settled and what remains insufficient
Settled by supplied source plus derived invariants

[REPOSITORY TRUTH]

The full Engine envelope is richer than Contracts’ coordinate envelope.

The full envelope retains private authority admission/currentness state.

Current Contracts run identity binds requested coordinates but not exact Engine evaluation authorization provenance.

Evidence admission and verdict evaluation use Engine rank/comparison semantics but do not validate the request against an admitted Engine authorization.

Dock equality is an operating contract, not authorization proof.

Engine must remain inward of Contracts in the dependency graph.

[DERIVED INVARIANT]

A coordinate-only evaluation context is replay-ambiguous and insufficient.

Engine must produce the authorization relation that Contracts consumes.

The evaluation context must bind that exact owner result.

Authorization issuance and currentness cannot be reconstructed by Contracts.

Closed results must preserve historical provenance without remaining current operational authority forever.

[DESIGN RECOMMENDATION]

Select B with mandatory opaque A lineage.

Name the object Evaluation Authorization Binding.

Put its full lifecycle in the Evaluation Authorization, Capability, and Readiness decision.

Put its mandatory consumer invariant in the two amendments now.

Require fresh binding/context after escalation.

Remaining packet insufficiencies

[PACKET INSUFFICIENT]

No current source establishes an existing Evaluation Authorization Binding implementation or registry.

The exact physical placement of the future evaluation-coordination/readiness responsibility remains unselected.

The source does not establish a final public vocabulary distinguishing every not-current cause such as revoked, stale, expired, or superseded.

The repository’s canonical Markdown/link-validation command is not included in the packet.

Nothing in this review selects an API, schema, persistence model, transport, or host-enforcement mechanism.

These are later decision or planning questions. They do not prevent selection of the semantic seam.

13. Fastest fully compliant continuation
Next working slice: two-file semantic amendment only
Files to modify

Exactly these two files:

docs/specs/handbook-contract-membrane/metric-gate-semantic-decision.md
docs/specs/handbook-contract-membrane/posture-assurance-policy-semantic-decision.md
Current-source areas to inspect but not modify
crates/engine/src/context_resolution.rs
crates/contracts/src/evaluation_run.rs
crates/contracts/src/evaluation_identity.rs
crates/contracts/src/evidence_admission.rs
crates/contracts/src/claim_verdict.rs
crates/dock/src/protocol.rs

Do not modify in this slice:

architecture-context.md
implemented-baseline.md
phase-and-slice-map.md
HCM-5.2 or HCM-4.6 slice authorities
Rust source
schemas
SDK or transport code
Dock manifests
Phase-6 records

The two semantic proposals remain proposed records; no product-state or roadmap status changes merely because their prose is amended.

Exact order of operations
1. Establish the repository baseline
Bash
git status --short
git rev-parse HEAD

Stop and separate unrelated existing changes before using a two-path scope assertion.

2. Reconfirm the source anchors
Bash
rg -n \
  'materialize_envelope|ContextResolutionEnvelopeInput|ContextResolutionEnvelope \{|ContextResolutionAuthorityAdmission|ProjectionAuthorityView|require_current|ContextResolutionEscalationOutcome' \
  crates/engine/src/context_resolution.rs

rg -n \
  'PrepareEvaluationRunInputV1|ContractEvaluationRunV1|requested_resolution|EvaluationRunPreimage|owner_currentness' \
  crates/contracts/src/evaluation_run.rs

rg -n \
  'ResolutionEnvelopeV1|EvidenceCandidateInputV1|EvidenceRecordBodyV1|ResolutionAccountingV1|GateResultBodyV1' \
  crates/contracts/src/evaluation_identity.rs

rg -n \
  'validate_candidate_closure|validate_resolution|effective_resolution_minimum_v1|resolution_meets_minimum_v1|producer_ceiling' \
  crates/contracts/src/evidence_admission.rs

rg -n \
  'evaluate_claim_verdicts_v1|revalidate_records|meets_resolution|resolution_insufficient|requested_resolution' \
  crates/contracts/src/claim_verdict.rs

rg -n \
  'DockEvaluationRunBindingV1|requested_resolution|resolution_ceiling|actual_resolution|DockProcessResultV1::completed' \
  crates/dock/src/protocol.rs

The purpose is not another broad audit. Confirm only:

envelope identity versus private authority admission;

coordinate-only Contracts identity;

absence of Engine authorization provenance;

current minimum/comparison behavior;

Dock equality.

3. Amend the metric/gate decision first

Touch these areas:

canonical vocabulary;

evaluation-context closure;

responsibility map;

gate/provenance invariants;

discriminating scenarios;

settled-versus-deferred boundary.

Add the six metric-record clause groups above. Do not define APIs, data members, persistence, or transport.

4. Amend the posture/assurance decision second

Touch these areas:

typed-domain non-equivalence;

assurance-selection invariants;

trust invariants;

evaluation-context conformance;

causal order and run binding;

discriminating scenarios;

settled-versus-deferred boundary.

Add the four posture-record clause groups above. Ensure no wording implies that Resolved Assurance Requirements contain, issue, or select the actual run authorization.

5. Perform a joint causal-alignment pass

Check the combined text in this order:

posture
  -> Resolved Assurance Requirements
  -> prospective evaluation purpose
  -> Engine Evaluation Authorization Binding
  -> readiness
  -> Contracts evaluation context
  -> evidence / claim evaluation
  -> metrics / semantic gate

Verify specifically:

requirements are selected before current capability is considered;

requirements never grant authority;

Engine authorization never selects easier requirements;

Contracts never reconstructs authorization;

producer and Dock facts never substitute for authorization;

stale authorization is a context-closure failure, not claim insufficiency;

later authorization changes do not mutate closed truth.

6. Apply the two-path scope fence

Assuming the baseline was clean:

Bash
expected="$(
  printf '%s\n' \
    docs/specs/handbook-contract-membrane/metric-gate-semantic-decision.md \
    docs/specs/handbook-contract-membrane/posture-assurance-policy-semantic-decision.md |
  sort
)"

actual="$(git diff --name-only | sort)"

printf '%s\n' "$actual"
test "$actual" = "$expected"
7. Run mechanical diff checks
Bash
git diff --check

git diff --word-diff=plain -- \
  docs/specs/handbook-contract-membrane/metric-gate-semantic-decision.md \
  docs/specs/handbook-contract-membrane/posture-assurance-policy-semantic-decision.md
8. Run semantic-presence checks
Bash
rg -n \
  'Evaluation Authorization Binding|Engine-owned|requested observation|authorization provenance|currentness|supersed|fresh evaluation|historical' \
  docs/specs/handbook-contract-membrane/metric-gate-semantic-decision.md \
  docs/specs/handbook-contract-membrane/posture-assurance-policy-semantic-decision.md

rg -n \
  'MUST NOT.*grant|MUST NOT.*relax|MUST NOT.*reconstruct|MUST NOT.*substitute|refuse before.*gate|fresh.*authorization.*binding' \
  docs/specs/handbook-contract-membrane/metric-gate-semantic-decision.md \
  docs/specs/handbook-contract-membrane/posture-assurance-policy-semantic-decision.md

Then manually confirm the scenario corpus contains at least:

coordinate-only request
wrong subject/objective/parent/constraints
Dock equality without Engine authorization
stale before closure
authorization change after closure
approved escalation
same labels under changed stack
low authorization cannot lower assurance
9. Discover and run the repository’s existing documentation validator

[PACKET INSUFFICIENT] The packet does not name the repository’s authoritative Markdown/link-check target. Locate it rather than inventing one:

Bash
rg -n --hidden \
  -g '!target/**' \
  -g '!node_modules/**' \
  'markdownlint|lychee|link[-_ ]?check|docs[-_ ]?check|markdown.*check' \
  .github Makefile justfile Cargo.toml package.json 2>/dev/null

Run the existing repository-defined target returned by that search. Do not add a new linter or dependency for this bounded amendment.

10. Perform one bounded independent review of the exact two-file diff

The review question should be limited to:

Do the two amendments now require exact Engine-owned evaluation authorization provenance without duplicating Engine semantics, letting assurance grant authority, letting capability relax requirements, or prematurely specifying readiness implementation?

Required verdict: APPROVE, ADJUST, or PIVOT, with findings limited to this seam.

14. Work immediately after that slice

Once the two-file amendment review approves:

create evaluation-authorization-capability-and-readiness-semantic-decision.md;

make it the controlling lifecycle record for the Engine binding, readiness, diagnostic intent, retries, currentness, caches, escalation, and supersession;

review that decision together with the two amended records;

create the separate explanation-projection decision;

review the four-record semantic closure;

only then begin integrated implementation planning.

Navigation/index files should change only when the new companion file is actually introduced and repository conventions require navigation updates.

15. Defer until after the next working slice

Do not yet design or change:

Rust types or APIs;

exact DTO or schema shapes;

Engine registries or persistence;

Contracts repository layouts;

SDK operations;

CLI or Tauri surfaces;

Dock protocol or manifest behavior;

current HCM-5.2 migration;

V1/V2 compatibility;

caching implementation;

public status-code vocabulary;

UI layout;

explanation rendering;

Substrate or host enforcement;

general Phase-6 adoption;

new posture or gate Resolution levels;

a universal rigor, capability, or Resolution score.

Final decision

ADJUST.

The correct seam is:

Engine issues a purpose-bounded Evaluation Authorization Binding from one exact admitted full Context Resolution envelope. The binding carries the stable authorization facts Contracts needs and opaque lineage to the full envelope and authority basis. Contracts binds it, exact-checks evaluation congruence, and relies on Engine for identity, admission, currentness, and supersession verification. Requested coordinates, producer capability, Dock ceilings, actual observation, claim adequacy, and viewer authority remain separate facts.

The fastest safe continuation is the two-file semantic amendment slice above, followed by one combined Evaluation Authorization, Capability, and Readiness decision. A separate fifth authorization decision is unnecessary.

Sources
~~~

