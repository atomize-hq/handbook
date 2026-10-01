# HCM Context Resolution / Posture / Gate / UX — ChatGPT Pro Deep Review

Date: 2026-09-01

Status: **complete; repository reconciliation complete**

Task/spec: Fresh, adversarial second-pass architecture and product review of the HCM Context Resolution, posture, assurance, evidence, metric/gate, explanation-projection, and enforcement seams.

Current approach: Posture selects Assurance Requirements; requirements may impose minimum Context Resolution; evidence records observation; verdict/evaluation determines claim truth; metric/gate composition summarizes closed truth; presentation density projects the same decision; enforcement consumes but does not mutate it. The review challenges this model and the first Pro consultation against the attached repository packet.

ChatGPT configuration verified before submission:

- Chat surface
- Pro enabled at 5 of 5
- GPT-5.6 Sol selected
- Fresh conversation: https://chatgpt.com/c/6a964c19-9190-83ea-8ca0-9bbde5d47a0a

Bounded sources supplied:

1. `architecture-context.md`
2. `implemented-baseline.md`
3. `phase-and-slice-map.md`
4. `metric-gate-semantic-decision.md`
5. `posture-assurance-policy-semantic-decision.md`
6. HCM-4.6 Production Snapshot Grounding Owner and Ordinary SDK Seam
7. HCM-5.2 Evidence, Verdict, and Gate Engine
8. the shipped Context Resolution root definition `1.0.0.yaml`
9. the first Pro consultation and local reconciliation

Advisory only; verify against local project truth and authoritative docs; do not reduce scope without user approval.

## Prompt

~~~text
We are already working on this project task.

Task/spec: Perform a fresh, substantially deeper second-pass architecture and product review of Handbook Contract Membrane's Context Resolution / posture / assurance / evidence / metric-gate / explanation-projection / enforcement seams. The immediate question is whether the two proposed semantic decisions can safely become planning input, what exact amendments or companion decisions are required first, and what user-workflow gaps remain. This is a greenfield product: current HCM-5.2 implementation shapes are repository evidence and owner-trust constraints, not compatibility obligations. Do not preserve a V1/V2 split or legacy behavior unless an architectural bridge is genuinely required.

Current approach: The working architecture is: posture is descriptive and selects exact Assurance Requirements; those requirements may select claims, policies, metrics, gates, and minimum observation Resolution; Context Resolution describes what a run is authorized and able to reach; evidence records what was observed; verdict/evaluation determines claim truth; metric/gate composition summarizes that closed truth; explanation projection changes only presentation density; enforcement consumes the decision downstream and never mutates gate truth. Repository-derived statement: Engine publicly owns Context Resolution ranks, dimensions, narrowing, authority, and escalation semantics. Repository-derived statement: Contracts serializes Resolution coordinates and owns their evidence/claim consequences rather than defining a competing resolution system. Repository-derived statement: requested Resolution is already included in the current evaluation-run fingerprint. Repository-derived statement: ordinary runtime adoption of the full execution envelope is narrow; production snapshot/grounding is the principal implemented Resolution-bounded projection consumer. The first Pro review, attached as a review target rather than authority, recommended ADJUST: amend both proposed decisions to close Context Resolution and underpowered-run semantics; create a narrow Evaluation Explanation Projection decision; and possibly create a separate Context Resolution Evaluation Readiness/Adoption decision. Our local reconciliation agreed with much of that but found several places where the first review generalized beyond repository truth.

Project sources: Read every attached source completely before answering, in this order:
1. architecture-context.md
2. implemented-baseline.md
3. phase-and-slice-map.md
4. metric-gate-semantic-decision.md
5. posture-assurance-policy-semantic-decision.md
6. the first README.md whose heading is "HCM-4.6 Production Snapshot Grounding Owner and Ordinary SDK Seam"
7. the second README.md whose heading is "HCM-5.2 Evidence, Verdict, and Gate Engine"
8. 1.0.0.yaml, the shipped Context Resolution root definition
9. hcm-context-resolution-posture-gate-ux-chatgpt-pro-consultation-2026-08-31.md, which contains the complete first Pro prompt, complete answer, and local repository reconciliation.
Treat the ninth file as a fallible prior analysis to challenge, not as repository authority. Cite the attached filename and section/heading for important conclusions. If the packet cannot establish a fact, label it "packet insufficient" instead of filling the gap from assumption.

Evidence, blockers, constraints, and validation: Use these repository-checked facts and unresolved questions as the bounded review agenda.

- Engine's shipped Context Resolution stack is strategic / coordination / execution / operation, with scope_horizon, detail_resolution, temporal_horizon, authority_horizon, memory_horizon, and validation_horizon dimensions. Do not collapse this to presentation detail.
- The full ContextResolutionEnvelope also includes constraints, mutation rules, authority, parentage, and escalation. Ordinary Codex tasks and every CLI/tool action are not automatically wrapped or host-enforced by it today.
- HCM-4.6 production snapshot/grounding does make a real Resolution- and disclosure-bounded projection, including omission/lossiness/refusal metadata. There is no universal Resolution-aware artifact or UI renderer.
- Contracts' durable ResolutionEnvelopeV1 delegates rank/minimum comparison to Engine's ContextResolutionStackDefinition/ResolutionObservationV1. Contracts owns evidence eligibility consequences.
- Current evaluation-run identity already binds requested_resolution, including exact stack binding. Determine what additional identity or provenance closure the proposed greenfield model actually needs without falsely claiming current HCM-5.2 omits requested Resolution.
- Current evidence admission computes effective Resolution as the coordinate-wise minimum of requested, actual, and optional producer ceiling. It loops across every observed claim id and rejects the whole candidate if any observed claim's minimum is unmet. This creates an unresolved atomicity question: can one multi-claim candidate establish claim A while being insufficient for claim B, or must candidates be partitioned, or should the model change?
- Current first-party Dock is narrower than a general producer: requested Resolution must equal the manifest ceiling, and completed actual Resolution must equal request/ceiling/requested. A producer whose ceiling is below the request is therefore a future generalization, not current Dock behavior.
- Current run preparation does not precompare the requested Resolution against all selected claim minima. Decide whether an underpowered run is refused before evidence collection, allowed only as an explicitly diagnostic/non-authoritative run, or evaluated into claim-specific indeterminate results; define the user-facing consequences.
- Current failure layers can differ: candidate admission may refuse without a canonical record; later verdict recheck may mark evidence insufficient/block if a record exists; gate consumes verdicts. Proposed richer gate semantics use pass/fail/indeterminate/not_applicable plus refusal-before-truth, while current HCM-5.2 uses passed/blocked. Carefully separate admission refusal, claim-specific insufficiency, evaluation readiness, gate truth, and downstream enforcement hold.
- Presentation preference must be type-distinct from Context Resolution detail_resolution. Candidate human levels are Quick Answer / Why / Audit Trail, all projections of the exact same closed evaluation, with decisive adverse facts and explicit withheld/unavailable markers retained.
- Posture must never grant authority or silently lower assurance. Assurance selection may choose claims/policies that carry minimum Context Resolution requirements. Runtime Resolution must not choose a lighter assurance profile.
- The Handbook CLI can enforce only owner operations it invokes. An envelope shown to an unrestricted agent is advisory. Future Substrate integration can enforce only when Substrate is the actual capability broker; external adopters remain outside Handbook's control.
- Engine owns posture and Context Resolution; Contracts owns contracts/metrics/gates. A physical assurance-selector placement must avoid an Engine-to-Contracts dependency back edge. The semantic decision may defer physical placement but must not imply an impossible dependency.
- The posture proposal has a heading "Resolution and selection invariants" using ordinary-language resolution while typed Context Resolution is absent. Treat this vocabulary collision explicitly.
- Escalation should create a fresh run/evaluation context, not mutate or retroactively upgrade a closed decision. Test whether that is sufficient.
- We need a user-centered mental model and workflows for at least: contract/policy author, posture author or owner, run operator/agent, evidence producer/adapter, reviewer/auditor, ordinary decision consumer, and enforcing host.
- We do not want one universal scalar rigor score, posture-specific gate truth, duplicated Resolution semantics, or multiple presentations that can disagree about the decision.
- No implementation code is requested. We are deciding the smallest semantically complete planning boundary before full task-level implementation planning.

Perform an adversarial second-pass, not a restatement of the first answer. Try to falsify both the proposed decisions and the prior Pro recommendation with concrete counterexamples and causal chains. Explicitly answer:

1. What is the cleanest domain model for capability, requirement, observation, evaluation readiness, decision truth, presentation density, disclosure, and enforcement? State owner and lifecycle of each.
2. Should Context Resolution appear in the evaluation request/context, selected Assurance Requirements, evidence candidate/record, verdict provenance, gate-result provenance, explanation projection request, or some subset? Give an exact responsibility/provenance matrix and explain every exclusion.
3. What is the precise relation between a multidimensional minimum requirement and an effective observation? Address stack identity/version, partial ordering, incomparable dimensions, producer ceiling, actual-vs-requested honesty, and stack evolution.
4. Resolve the multi-claim evidence-candidate atomicity problem. Compare whole-candidate refusal, per-claim admission, normalized candidate partitioning, and any better option. Identify canonical identity and audit implications.
5. Define a minimal evaluation-readiness and outcome state machine that distinguishes preflight refusal, diagnostic execution, evidence insufficiency, claim indeterminate, gate indeterminate/fail, and enforcement hold without multiplying redundant concepts.
6. Explain whether an underpowered requested Resolution should ever be allowed, under what explicit run intent, and what authority its outputs possess.
7. Test the current Dock equality boundary against the proposed general producer model. Which semantics belong now, which are future adapter semantics, and which would be dangerous to bake into the semantic decisions?
8. Define exact invariants for escalation, retries, supersession, cache reuse, identity/fingerprints, and closed-decision immutability.
9. Design the lowest-complexity user mental model and end-to-end workflows for the named actors. Include default UX, drill-down, remediation guidance, and how to prevent users from confusing "less detailed display" with "less rigorous evaluation."
10. Determine whether the minimal documentation decomposition is: amendments to the two proposals plus one explanation-projection decision; amendments plus separate explanation-projection and evaluation-readiness/adoption decisions; or another decomposition. Explain why each boundary is cohesive and why omitted documents are unnecessary.
11. List every approval blocker, ranked P1/P2/P3, and distinguish semantic blockers from adoption/implementation planning concerns.
12. Identify any errors, unjustified assumptions, or missing gaps in the first Pro review and the local reconciliation.
13. Provide concrete discriminating scenarios and decision tables sufficient to expose an incorrect implementation later.
14. Draft the exact normative clauses or section outlines that should be added to each existing proposal and any companion decision. Keep them narrow; do not draft implementation APIs or schemas.
15. End with a direct KEEP / ADJUST / PIVOT verdict, a smallest safe sequence before implementation planning, and a list of tempting scope expansions to defer.

Label each major statement as one of: REPOSITORY TRUTH, DERIVED INVARIANT, DESIGN RECOMMENDATION, or PACKET INSUFFICIENT. Distinguish "implemented semantic kernel", "production adoption", and "future enforceability" throughout. Prefer precise tables and counterexamples over generic prose.

Do not reduce the task or replace it with an easier alternative. Preserve the requested scope and project conventions.

Is the current approach the best way forward? Give a direct keep, adjust, or pivot recommendation; tradeoffs; what to keep/change first; inspections; validation; and pitfalls.
~~~

## Complete ChatGPT Pro response

Source conversation: https://chatgpt.com/c/6a964c19-9190-83ea-8ca0-9bbde5d47a0a

~~~text
Worked for 65m 3s
Adversarial second-pass review
Verdict: ADJUST

[DESIGN RECOMMENDATION] The core architecture should be retained:

posture → exact assurance requirements → evidence and claim evaluation → named metrics and semantic gate → explanation projection → downstream enforcement

There is no reason to pivot to posture-specific gate truth, a universal rigor score, a duplicated Resolution system, or host enforcement inside Contracts.

The two proposed decisions are not yet safe as the complete semantic input to implementation planning, however. The first review found a real omission—typed Context Resolution is absent from the proposals—but its proposed correction was still too compressed. It treated “run Resolution” as though one construct could simultaneously mean:

what the run is authorized to do;

what the evaluation requests;

what a producer is capable of;

what the producer actually observed; and

whether that observation meets a claim requirement.

Those are related, but they are not the same fact or authority.

The smallest safe boundary is:

amend both existing proposals;

approve a narrow Evaluation Capability and Readiness decision;

approve a separate Evaluation Explanation Projection decision;

only then begin integrated task-level planning;

defer Handbook-to-Substrate enforcement until an actual host/adopter is selected.

The proposals themselves remain expressly proposed semantic records rather than implementation authority, and current HCM-5.2 shapes are bounded implementation evidence rather than greenfield compatibility obligations. 

metric-gate-semantic-decision

 

posture-assurance-policy-semant…

1. Findings ordered by materiality
P1 Critical

None.

[DERIVED INVARIANT] The fundamental owner graph remains sound. Engine owns Context Resolution and posture semantics; Contracts owns contract, evidence, claim, metric, and gate truth; SDK and transports project owner truth; enforcement remains downstream. The architecture explicitly protects that dependency direction. 

architecture-context +1

P2 Required
P2-1 — The proposed correction must separate authorization, requested reach, producer capability, and actual observation

[REPOSITORY TRUTH] The shipped stack has six independently ranked dimensions, including authority_horizon with values from read_only through program_policy; it is not merely an observation-detail scale. 

1.0.0 +1

[REPOSITORY TRUTH] Current HCM-5.2 binds a requested six-dimensional envelope into the evaluation run and computes effective Resolution dimension-by-dimension from the request, optional producer ceiling, and actual observation. Contracts uses Engine-owned stack semantics and owns the evidence-eligibility consequence. 

README +1

[DERIVED INVARIANT] A componentwise “minimum observation Resolution” across all six dimensions cannot be adopted as an ambient assurance-quality rule. In particular, a higher authority_horizon means broader operational authority, not intrinsically stronger evidence. A hardened claim must not become “better proven” merely because the evaluator had more write authority.

A concrete failure chain would be:

Hardened posture
  -> claim requires "higher Resolution"
  -> generic six-coordinate minimum includes program_policy authority
  -> current run has read_only authority
  -> system requests or grants write authority merely to satisfy assurance

That violates least privilege and collapses requirement into authorization.

[DESIGN RECOMMENDATION] The amendments must distinguish at least four exact roles:

Role	Meaning	Owner
Run authorization binding	What this operation is permitted to inspect, retain, validate, or mutate	Engine Context Resolution authority
Requested observation envelope	The reach requested for this exact evaluation, bounded by the run authorization	Evaluation request/context owner
Producer capability ceiling	The maximum reach a particular producer declares and can substantiate	Producer, then independently admitted
Actual observation Resolution	What the producer actually observed	Source observation, then canonical admission

A selected claim or Evidence Evaluation Policy may declare an exact observation-adequacy requirement over an explicitly declared subset of Context Resolution dimensions. No dimension is relevant by default, and no cross-dimension compensation is permitted.

This is more precise than the first review’s phrase “current run capability.”

P2-2 — Multi-claim candidate atomicity and failure layering are unresolved

[REPOSITORY TRUTH] The bounded prompt states that current admission loops over every observed_claim_id and rejects the whole candidate if any observed claim’s minimum is unmet. It also records that current candidate admission, later verdict recheck, and gate behavior can fail at different layers. 

Pasted text

[REPOSITORY TRUTH] The greenfield metric proposal instead assigns canonical admission to source identity, provenance, authority, subject, context, unit, normalization, source validity, and duplicate checks, while claim evaluation owns evidence selection, sufficiency, currentness, consistency, uncertainty, and finding establishment. 

metric-gate-semantic-decision +1

Those two models do not automatically reconcile.

The current whole-candidate rule permits this defect:

one authentic observation
  -> bound to claim A and claim B
  -> actual detail = normal
  -> A requires summary
  -> B requires full
  -> B's insufficiency rejects the whole candidate
  -> A loses otherwise valid evidence

It also lets candidate construction determine atomicity: adding one harder claim to the same candidate can erase usable evidence for every easier claim.

The opposite extreme—partially admitting a candidate per claim—would make “canonical evidence record” mean partially trusted data and would complicate idempotency and audit identity.

[DESIGN RECOMMENDATION] Adopt the following rule:

Admit the observation closure once; qualify it independently per claim.

A canonical evidence record should represent one homogeneous observation closure: exact run, source, provenance, value, unit/scale, producer basis, actual Resolution, and complete ordered claim-binding inventory.

Claim-specific Resolution adequacy should not be part of the evidence-record admission decision or evidence-record fingerprint. It belongs in the claim-evaluation/verdict identity.

Therefore the same record may be:

eligible for claim A;

ineligible for claim B;

visible in both claims’ accounting.

A whole candidate should refuse only for candidate-wide defects such as malformed bytes, unauthorized source, inconsistent stack identity, impossible provenance, unselected claims, actual observation beyond authorization, or same-identity/different-bytes substitution.

If one submitted candidate contains observations with materially different provenance or actual Resolution, admission may deterministically normalize it into multiple observation-homogeneous records. It should not partition merely because the bound claims differ.

P2-3 — Candidate-admission refusal is currently conflated with evaluation refusal

The proposed scenario corpus distinguishes:

an applicable claim with no admitted record, which produces an indeterminate claim and gate; and

an unauthorized candidate, which is described as producing no claim evaluation, metric result, or gate decision. 

metric-gate-semantic-decision +1

[DERIVED INVARIANT] Those cases need a more exact boundary. Otherwise any unsolicited unauthorized submission could veto an otherwise valid evaluation.

The correct distinctions are:

Failure	Correct scope
Candidate bytes or source authority are invalid	Refuse that candidate; produce no canonical record for it
Required collection attempt is accounted but yields no eligible evidence	Claim may become indeterminate under the exact policy
Valid alternative evidence closes the claim	Evaluation may proceed; rejected candidate remains visible in accounting
Evidence-set identity, repository inventory, or collection accounting is ambiguous/corrupt	Refuse the evaluation; no semantic gate truth
Unsolicited invalid candidate is outside the selected source closure	Reject and account; it must not poison a closed evaluation

[DESIGN RECOMMENDATION] Amend the “Unauthorized source” scenario so that it distinguishes admission-operation refusal from the status of a later closed evaluation. A rejected candidate can never satisfy a claim, but it does not automatically imply that no gate may exist.

P2-4 — A valid but underpowered context needs a separate readiness lifecycle and explicit use intent

[REPOSITORY TRUTH] Current run preparation does not precompare requested Resolution against every selected claim minimum. The current fail-closed path therefore discovers some insufficiency later at admission or evaluation. 

Pasted text

The first review recommended allowing underpowered diagnostic runs, but it did not fully settle:

how such a run is intentionally requested;

whether a decision-seeking run may silently continue;

whether its output may authorize downstream progress;

how readiness, claim indeterminacy, gate truth, and enforcement hold differ;

what changes after escalation;

which identities and caches remain reusable.

[DESIGN RECOMMENDATION] Introduce a narrow Evaluation Capability and Readiness decision with:

an exact decision_seeking or diagnostic use intent;

a pure readiness result;

no gate or enforcement semantics;

exact escalation and fresh-context rules.

A valid but known-underpowered run should be legal only as an explicitly diagnostic run. A decision-seeking request that is already known unable to meet one or more applicable gate-relevant requirements should be held before execution or replaced by a new request.

Diagnostic output remains canonical semantic truth for its exact context. It is not a “weaker gate.” However, it must not produce a positive downstream disposition unless an exact Enforcement Policy explicitly accepts diagnostic provenance.

P2-5 — Explanation density and disclosure are two independent axes, not one “Resolution” control

[REPOSITORY TRUTH] The shipped stack already uses detail_resolution as a semantic Context Resolution coordinate. Production HCM-4.6 projection is bounded by exact Resolution, disclosure policy, omissions, lossiness, provenance, and rechecks, but that is one specific production projection path rather than a universal UI renderer. 

README

 

architecture-context

[DERIVED INVARIANT] A user choosing “show less detail” must not change:

requested evaluation Resolution;

producer eligibility;

canonical evidence;

claim findings;

metric results;

semantic gate truth.

The first review correctly separated explanation density from Context Resolution, but it did not make density and disclosure authorization sufficiently independent.

The correct projection model is:

canonical result and canonical explanation
  + presentation density preference
  + exact disclosure policy
  + viewer authorization/currentness
  -> bounded explanation projection

A user may request Audit Trail while being authorized to see only redacted summaries. Conversely, a fully authorized user may select Quick Answer. Those are different axes.

A low-density view must still preserve a decisive adverse fact at an authorized abstraction level. If the details are restricted, it may say:

A required security condition failed. Details are withheld under policy X.

It may not omit the failure and display only a high score.

P3 Important / planning constraints
P3-1 — Assurance-selector physical placement remains a dependency hazard

[REPOSITORY TRUTH] Engine must not acquire a dependency back to Contracts, while an Assurance Policy needs to select Contracts-owned contract, policy, metric, and gate identities. 

README

[DESIGN RECOMMENDATION] The semantic record may continue deferring physical placement, but implementation planning must place assurance selection either:

inside the Contracts side while preserving a distinct semantic responsibility; or

in a purpose-specific owner downstream of Engine and upstream of evaluation.

It must not be placed in Engine if doing so requires Engine to resolve Contracts-owned semantics.

P3-2 — Current first-party Dock equality is a strict adapter policy, not the universal producer model

The prompt and local reconciliation state that the current first-party Dock requires requested Resolution, manifest ceiling, and completed actual Resolution to be equal. A requested-greater-than-ceiling completion is therefore not current product behavior. 

Pasted text

 

hcm-context-resolution-posture-…

That equality rule is valid for that producer. It should not become a universal semantic invariant. A general producer model must support:

a ceiling higher than the requested reach;

actual observation lower than the request;

readiness incompatibility when the known ceiling is too low;

producer-specific all-or-nothing policies;

exact refusal if actual observation exceeds authorization, request, or admitted ceiling.

P3-3 — Current two-state gate migration is implementation planning, not semantic authority

Current HCM-5.2 exposes passed or blocked; the proposal introduces pass, fail, indeterminate, and not_applicable, with context-construction refusal outside the gate. 

README

 

metric-gate-semantic-decision

The mapping of current blocked reasons into future:

evaluation refusal;

claim indeterminate;

gate fail;

gate indeterminate;

enforcement hold

requires a bounded migration inspection, but it should not weaken the proposed semantics.

P3-4 — Host enforcement remains a separate future decision

Repository evidence establishes no current external Substrate adoption or task-wide capability enforcement. Phase 6 leaves the bridge, publication, and adopter unresolved or external-unknown. 

phase-and-slice-map

A CLI can enforce the owner operation it invokes. It cannot constrain an unrelated shell merely by displaying or prompting with a Context Resolution envelope.

2. Implemented kernel, production adoption, and future enforceability
Boundary	Bounded conclusion
Implemented semantic kernel	Engine owns the Context Resolution domains and stack semantics; Contracts currently serializes Resolution into its evaluation/evidence model and delegates ordering/comparison to Engine.
Production adoption	HCM-5.2 uses Resolution in evidence eligibility, and HCM-4.6 uses Resolution plus disclosure in one production snapshot/grounding projection. Adoption is real but narrow.
Future enforceability	No supplied source proves ordinary task-wide host enforcement, universal Resolution-aware rendering, or an operating Substrate bridge.

The architecture classifies the Context Resolution domain as current, snapshot-grounding projection as an implemented owner path, and publication or Substrate adoption as external-unknown. 

architecture-context

 

implemented-baseline

3. Clean domain model
Authoritative model
Concept	Plain-language name	Semantic owner	Lifecycle	Must not mean
Posture	Engineering standard	Posture authority / Engine-owned mutation boundary	Persisted scoped authority; immutable snapshots and authorized transitions	Current agent capability, display detail, gate truth
Resolved Assurance Requirements	Proof plan	Assurance-selection authority	Immutable selection from one exact posture snapshot and admitted selector inputs	Authorization grant, current producer ability
Run authorization binding	Allowed reach	Engine Context Resolution authority	Exact immutable grant/constraint binding for one operation or run	Proof requirement or actual observation
Requested observation envelope	Requested reach	Evaluation request/context owner	Exact request within the authorization binding	Automatic authority grant or actual reach
Producer capability ceiling	Tool capability	Producer, independently admitted	Versioned/currentness-bound capability fact	Actual observation or claim satisfaction
Actual observation Resolution	What the tool actually saw	Source observation; canonical admission validates it	Immutable observation fact	Requested or maximum capability
Observation-adequacy requirement	What this claim needs	Selected claim or compatible Evidence Evaluation Policy	Immutable requirement under one exact stack definition	Authority grant or universal assurance scalar
Evaluation readiness	Can this run answer the proof plan?	Separate readiness responsibility	Pure pre-execution derived result	Gate truth, claim truth, enforcement
Claim evaluation	What was established for this claim?	Contracts claim evaluator	Pure result over one closed context and evidence set	Readiness or operational disposition
Metric and gate	Semantic decision	Contracts metric/gate owners	Pure exact result	Mutation, privilege, workflow action
Presentation density	How much of the same answer to show	Explanation projection	Per-view request	Context Resolution detail_resolution
Disclosure authorization	What this viewer may see	Disclosure/authority policy	Rechecked for each projection	Display preference or gate truth
Enforcement disposition	What may happen next	Separate Enforcement Policy	Downstream pure policy result, followed by separately authorized effects	Rewritten semantic gate truth
Host capability enforcement	What was actually constrained	Actual host, eventually Substrate if selected	Runtime grant/restriction plus effect receipts	Handbook semantic inference

[DERIVED INVARIANT] Posture should not be described merely as “descriptive” if that suggests advisory text. It is better described as normative but non-executing configuration: it authoritatively selects requirements but does not grant capabilities, evaluate evidence, or enforce action.

4. Exact Context Resolution responsibility and provenance matrix
Artifact or stage	Context Resolution content that belongs there	Content explicitly excluded
Assurance Policy	Supported exact stack identities; exact selected claims or policies carrying compatible observation requirements	Actual run grant, requested envelope, producer ceiling, actual observation
Resolved Assurance Requirements	Transitive binding to every selected observation requirement and its exact stack ref/version/fingerprint	The current run’s requested or authorized reach
Evaluation request	Exact requested observation envelope; exact Engine authorization-binding reference; evaluation use intent; selected producer constraints	Claim findings, effective observation, gate outcome
Evaluation context	Admitted exact stack identity; exact authorization binding/currentness; exact requested envelope; exact Resolved Assurance Requirements	Ambient mutable posture; unbound host claims
Evidence candidate	Evaluation-context ref; producer ceiling if declared; actual observation envelope; exact source observation and provenance	Claim-adequacy conclusion, gate result
Canonical evidence record	Independently admitted stack/context/producer/actual bindings; defensively derived effective observation envelope; complete claim-binding inventory	Satisfaction or per-claim sufficiency as canonical admission truth
Claim evaluation/verdict	Exact selected requirement; evidence record; componentwise adequacy result and reasons	New stack semantics, authority escalation
Metric result	Only transitive claim-evaluation identities	Raw Resolution coordinates, scalarized “Resolution score”
Gate result	Transitive evaluation-context and verdict identities; decisive Resolution insufficiency in explanation when relevant	Independent Resolution recomputation or duplicated ordering
Explanation projection request	Separate density preference; optionally an exact viewer/disclosure Context Resolution binding	The evaluated run’s detail_resolution as the density selector
Enforcement Policy	May consume exact context/readiness/use-intent provenance if declared	Reinterpretation of Resolution or claim truth
Host integration	Exact translatable constraints and host grant identity	Invention of assurance or gate semantics
Why these exclusions matter

[DERIVED INVARIANT] The gate result does not need to duplicate every Resolution coordinate. Its content-addressed closure should bind the exact evaluation context and verdicts transitively. Duplicating the same semantics into gate fields would create another source of inconsistency.

[DERIVED INVARIANT] The evaluation’s requested Resolution must not appear in Resolved Assurance Requirements. Doing so would make requirements vary with current capability. Conversely, the selected minimum requirements must appear transitively in Resolved Assurance Requirements, because otherwise the proof plan is incomplete.

[DERIVED INVARIANT] The explanation projection may consume a viewer’s Context Resolution as a disclosure constraint, but not as a user-selected presentation-density control.

The current proposals already require exact, complete realization of one Resolved Assurance Requirements identity and refuse additions, omissions, substitutions, wrong-scope bindings, or unresolved dependencies before semantic truth. Context Resolution should be added to that closure without weakening those rules. 

metric-gate-semantic-decision

 

posture-assurance-policy-semant…

5. Precise multidimensional relation
Formal model

[DESIGN RECOMMENDATION] Let:

S be one exact Engine Context Resolution stack definition;

A be the exact authorized run ceiling derived from the Engine binding;

Q be the evaluation’s requested observation envelope;

P be an optional admitted producer ceiling;

O be the producer’s actual observation envelope;

E be the effective observation envelope;

M_c be claim c’s exact observation-adequacy requirement over a declared subset of dimensions.

All values must bind the same exact S ref/version/fingerprint unless a separately named, versioned compatibility transformation exists. No such transformation should be inferred from matching labels.

Formation and honesty rules
Q must be componentwise within A.
O must be componentwise within A.
O must be componentwise within Q.
If P exists, O must be componentwise within P.

A violation is not repaired by clamping. It is a refusal because:

O > A means observation exceeded authority;

O > Q means the producer observed more than requested;

O > P means actual behavior contradicts the admitted producer ceiling.

Effective observation

For defensive accounting:

E = meet_S(A, Q, P?, O)

After the honesty rules pass, E will normally equal O. The meet operation is retained for exact reproducibility and defense in depth; it must not sanitize an overreaching producer.

Claim adequacy

For the dimensions explicitly declared relevant to claim c:

meets(c, E) iff
  for every relevant dimension d:
    rank_S(E[d]) >= rank_S(M_c[d])

There is:

no averaging;

no distance score;

no cross-dimension compensation;

no “nearest” envelope;

no inference that higher authority means stronger assurance.

Partial ordering

Two envelopes may be incomparable:

Producer A: full detail, local scope
Producer B: normal detail, slice scope

Neither is globally higher. A particular claim may be satisfiable by A, B, both, or neither.

Stack evolution

A changed stack fingerprint creates distinct semantics even if every textual value remains unchanged.

Historical reconstruction may use the exact old immutable stack.

A fresh evaluation under a new current stack is a new context.

Cache reuse across stacks refuses unless an exact compatibility transformation is selected and included in identity and trace.

Same-ref changed stack bytes are substitution, not evolution.

6. Multi-claim evidence atomicity
Alternatives
Alternative	Benefit	Defect
Whole-candidate refusal	Simple record atomicity	One hard claim erases valid evidence for easier claims; candidate author controls failure blast radius
Per-claim partial admission	Preserves useful claim bindings	Makes one canonical record partly admitted and partly refused; difficult identity and audit semantics
Claim-based normalized partitioning	Produces fully valid claim-specific records	Duplicates observation data and makes claim inventory drive canonical observation identity
Admit once, qualify per claim	Preserves common observation truth, exact per-claim accounting, and canonical trust separation	More detailed verdict accounting; may retain evidence unusable for some claims
Recommended model

[DESIGN RECOMMENDATION] Use admit once, qualify per claim.

A canonical record binds:

original candidate identity;

exact evaluation context;

common source/provenance closure;

source-native and canonical value/unit/scale;

authorization, request, producer, and actual Resolution bindings;

complete ordered claim-binding inventory;

deterministic normalization identity, if the candidate contained multiple observation closures.

The record does not bind:

“meets claim A’s minimum”;

satisfaction;

finding;

gate consequence.

Those belong to the claim evaluation.

Identity effects

Changing common observation bytes, provenance, run, producer basis, actual Resolution, or claim-binding inventory changes the evidence-record identity.

Changing the selected requirement or adequacy result changes the claim-evaluation identity, not the evidence-record identity.

A duplicate candidate with identical bytes is idempotent.

Same candidate identity with changed bytes refuses.

No claim binding may disappear during normalization; every selected claim is observed, unobserved, excluded, refused, eligible, or ineligible exactly once.

Discriminating example
Record R:
  actual detail = normal
  bound claims = [A, B]

A requires summary:
  R is eligible
  A may establish a finding

B requires full:
  R is ineligible for B
  B becomes indeterminate unless other eligible evidence exists

The gate consumes the two separate claim states. There is no global “run Resolution pass.”

7. Minimal readiness and outcome state machine
State machine
untrusted evaluation request
    |
    +-- context formation failure
    |       -> FORMATION REFUSED
    |          no evaluation context, no claim truth, no gate truth
    |
    `-- admitted evaluation context
            |
            -> readiness assessment
                 READY
                 KNOWN_LIMITED
                 UNKNOWN
            |
            -> workflow use intent
                 DECISION_SEEKING
                 DIAGNOSTIC
            |
            -> evidence acquisition and candidate admission
            |
            -> claim evaluations
                 satisfied / unsatisfied / graded / indeterminate / not_applicable
            |
            -> metric results
                 computed / not_computable / not_applicable
            |
            -> semantic gate
                 pass / fail / indeterminate / not_applicable
            |
            -> enforcement
                 proceed / block / hold / no_effect / other exact disposition
Readiness states
State	Exact meaning	What it does not mean
READY	No known applicable selected requirement is impossible under the current request and admitted producer bases	A guarantee of pass
KNOWN_LIMITED	At least one known applicable gate-relevant requirement cannot be met under the current request/selected producer bases	Gate fail or gate indeterminate
UNKNOWN	The available pre-execution applicability or producer capability facts cannot establish ready or known-limited	Ready by default
FORMATION REFUSED	The context, scope, stack, authority, requirements, or exact dependency closure is invalid	A readiness result

Readiness must be pure, content-addressed, currentness-bound, and non-mutating.

It cannot be owned by:

Assurance selection, because that would let current capability alter requirements;

Engine alone, because Engine does not own Contracts claim requirements;

Claim evaluation, because no actual evidence yet exists;

Gate, because gate truth is downstream of evidence;

Enforcement, because enforcement may consume readiness but must not invent it.

A distinct readiness responsibility is therefore justified.

Use intent
Use intent	Known-limited behavior	Authority of output
DECISION_SEEKING	Must not silently proceed. Default is hold and require a new producer, new request, approved escalation, or explicit diagnostic request.	Eligible for downstream decision use if the exact Enforcement Policy accepts its complete provenance
DIAGNOSTIC	May execute despite known limits	Produces canonical semantic truth for that exact context, but no positive operational disposition unless an exact policy explicitly accepts diagnostic provenance

Changing from decision-seeking to diagnostic should create a new evaluation-context identity. The evaluator must not branch on intent when computing findings, metrics, or gate truth; the intent exists for provenance and downstream use.

Underpowered-run answer

[DESIGN RECOMMENDATION] An underpowered requested Resolution should be allowed only when:

its context is otherwise valid;

the request is within the Engine authorization binding;

the use intent is explicitly diagnostic;

all known limitations are shown before execution;

requirements remain unchanged;

no automatic capability escalation occurs.

The output can establish:

valid findings for claims it can observe;

adverse findings;

indeterminate claims;

an exact metric or gate result under the unchanged policy.

It cannot by itself authorize mutation or positive progression.

8. Failure-layer decision table
Condition	Correct layer	Correct outcome
Unknown stack ref, changed same-ref bytes, invalid coordinate	Context formation or candidate admission	Refusal
Requested reach exceeds authorized run binding	Context formation	Refusal
Requirement references an unsupported stack or undeclared dimension role	Requirements/context closure	Refusal
Known producer ceiling cannot meet a selected requirement	Readiness	KNOWN_LIMITED
Producer capability is unknown	Readiness	UNKNOWN
Producer actual is below a claim minimum	Claim evaluation	Evidence ineligible for that claim; claim may be indeterminate
Producer actual exceeds request or authority	Candidate admission	Refusal; never clamp
One authentic record meets claim A but not B	Claim evaluation	A and B receive different adequacy outcomes
Candidate source is unauthorized but collection closure remains valid	Candidate admission plus claim accounting	Candidate refused; evaluation may still proceed
Evidence repository or complete source inventory is ambiguous	Evaluation closure	Refusal; no gate
Applicable claim has no eligible sufficient evidence	Claim evaluation	Indeterminate determination
Claim is established false	Claim evaluation and gate	Gate may fail under policy
Gate cannot establish pass/fail	Gate	indeterminate
Gate passes but run was diagnostic	Enforcement	Normally hold/no positive action unless explicitly accepted
Host cannot realize required restriction	Host integration/enforcement	Hold/refuse; never claim enforcement
9. Current Dock equality versus general producer semantics
Current bounded behavior

[REPOSITORY TRUTH] The current first-party Dock uses an equality boundary:

requested Resolution
  = manifest ceiling
  = completed actual Resolution

That provides a strong all-or-nothing producer contract for the current path. It does not prove that heterogeneous producers, degraded completions, or request-below-ceiling execution are implemented.

Greenfield core semantics

The semantic decisions should permit, without requiring every adapter to implement:

Case	Core semantic treatment
Producer ceiling exceeds request	Compatible; request remains the operative bound
Producer ceiling equals request	Compatible
Producer ceiling is below request	Readiness known-limited or producer incompatible
Actual equals request	Honest full completion
Actual is lower than request	Honest degraded/partial observation if the producer policy allows completion; claim-specific adequacy applies
Actual exceeds request	Refusal
Actual exceeds admitted ceiling	Refusal
Producer has no admitted ceiling	Readiness unknown; actual must still be exact
Semantics to keep adapter-specific

Whether a producer may return a partial completed observation.

Whether request and ceiling must be equal.

Whether a producer must refuse before launch when it cannot meet the request.

Whether one execution may emit several observation closures.

Operational timeout, cleanup, and process rules.

Dangerous universalizations

Do not bake into the semantic decisions:

universal request/ceiling/actual equality;

silent clipping of actual reach;

“producer ceiling” as evidence that the producer actually observed that much;

selection of easier requirements because a producer is limited;

a presumption that every producer supports degraded completion.

10. Exact invariants for identity, escalation, retry, supersession, and caches
Invariant	Normative effect
Single Resolution authority	Only Engine defines stack dimensions, ranks, comparison, meet/narrowing, and non-expansion
Requirement does not grant capability	A selected minimum cannot expand scope, authority, memory, or validation access
Capability does not relax requirement	Low request, low ceiling, or low actual reach never selects an easier Assurance Policy
Exact stack closure	Same labels under a different stack fingerprint are semantically distinct
No ambient dimension relevance	Each observation requirement declares the dimensions it uses
No privilege-as-quality rule	Higher authority_horizon is not intrinsically better evidence or stronger assurance
Actual-observation honesty	Actual reach above request, authorization, or ceiling refuses; no clamping
Per-claim adequacy	One record may be eligible for one claim and ineligible for another
Readiness is not gate truth	Readiness may hold work but cannot produce pass, fail, or indeterminate
Diagnostic provenance is exact	Diagnostic intent cannot be relabeled after execution
Escalation is non-authorizing	An escalation request grants nothing
Escalation creates a fresh context	Approval creates a new Engine binding and a new evaluation context; old results remain unchanged
Retry does not overwrite	A new producer attempt produces a new observation/candidate identity unless it is an exact idempotent replay
Evidence-set changes create new results	New evidence root means new claim, metric, and gate identities
Supersession is a relation, not mutation	A later run may supersede an earlier result for workflow use but cannot rewrite historical truth
Cache reuse is exact	Reuse requires identical semantic dependencies, policies, stack, subject, time/currentness basis, evidence set, and relevant intent
Projection caches are viewer-bound	Explanation cache key includes canonical result, density, disclosure policy, viewer authority, and currentness
Lower-density output is not a source	A higher-density view must project from canonical explanation, not from a previous summary
Host enforcement requires evidence	A constraint is “enforced” only if the actual host applied it or refused execution

The posture proposal already correctly freezes a run to P_n and requires a fresh run under P_{n+1}; the same rule should be extended to Context Resolution escalation and stack changes. 

posture-assurance-policy-semant…

11. Lowest-complexity user mental model
Seven questions
User-facing concept	Question	Technical owner
Engineering standard	How rigorous should this scope be?	Posture
Proof plan	What exactly must be established?	Assurance Requirements
Run check	Is this run allowed and capable enough to attempt that proof?	Context Resolution binding plus readiness
What was observed	What did the producer actually inspect or measure?	Source observation and canonical evidence
Decision	What did that valid evidence establish?	Claim evaluation, metrics, gate
View	How much of the same answer should be shown?	Explanation projection
Next action	What may happen now?	Enforcement and actual host

The UI must not call both “run reach” and “view detail” Resolution.

Recommended labels:

Allowed reach — Engine authorization.

Requested reach — evaluation request.

Tool capability — producer ceiling.

Observed reach — actual observation.

Quick Answer / Why / Audit Trail — presentation density.

12. Actor workflows
Posture author or owner

Default experience

Engineering standard for scope: release pipeline

Baseline: Production
Security and privacy: Hardened
Developer tooling: Product
Other dimensions: Production

Then:

This standard currently selects Proof Plan secure-release@3.

The author should not need to choose a Context Resolution preset or producer.

A classification-derived suggestion remains visibly provisional; persisted explicit posture remains authoritative, as the posture proposal already requires. 

posture-assurance-policy-semant…

Contract or assurance-policy author

The authoring view should show:

exact scope supported;

posture dimensions consumed;

exact contract and claims selected;

Evidence Evaluation Policies;

each claim’s declared observation-relevant dimensions and minima;

metrics and gate;

exceptions;

monotonicity consequences.

Validation should reject:

a requirement that relies on an undeclared stack;

an ambient six-dimension “higher is better” assumption;

any rule that treats authority reach as assurance strength;

a policy that chooses easier requirements from current producer capability;

ambiguous or conflicting minima.

For incomparable posture vectors, preview two independent resolutions rather than labeling one globally stricter.

Run operator or agent

Default flow:

1. Resolve engineering standard and proof plan.
2. Bind allowed and requested reach.
3. Select candidate producers.
4. Run readiness.
5. Proceed, change producer/reach, request escalation, or run diagnostically.

Example:

Run check: known limitation
Two required claims need Full input detail.
Requested reach: Normal.
Selected validator capability: Normal.

Options: choose Full Validator, request broader reach, or create a Diagnostic run.

No requirement is silently lowered.

Evidence producer or adapter

The producer receives an exact request within an authorization binding.

It reports:

its exact capability basis or ceiling, if known;

actual observation Resolution;

source-native values and provenance;

omitted or unavailable observations.

It does not:

raise its own authorization;

claim requested reach as actual;

decide applicability;

decide whether the observation satisfies a claim;

choose another Assurance Policy;

produce a gate result.

Reviewer or auditor

The default view should show two separate lines:

Semantic decision: Could not determine
Operational disposition: Held

Quick Answer:

One required security claim could not be established.
The validator observed Normal detail; the claim requires Full.
Proof plan: secure-release@3.
Next action: choose the full-capability validator or request broader allowed reach.

Why expands the decisive conditions.

Audit Trail shows exact claims, evidence accounting, policies, metrics, gate conditions, identities, and provenance subject to disclosure.

Changing views does not rerun the evaluation. The canonical decision ref remains visible.

Ordinary decision consumer

The consumer should see:

semantic gate result;

enforcement disposition;

whether the run was decision-seeking or diagnostic;

material unresolved conditions;

exact canonical result reference.

It should never receive a single ambiguous red badge called “Blocked” for all of:

evaluation formation refusal;

gate fail;

gate indeterminate;

enforcement hold.

Enforcing host

A future host receives exact translatable constraints and disposition.

It may enforce:

allowed tools;

writable paths or mounts;

credentials;

process and network access;

memory access;

execution lifecycle constraints.

It must report:

what it actually granted;

what it denied;

what it could not represent;

provenance of actual effects.

Temporal, validation, or presentation dimensions need not map directly to operating-system restrictions. The host must not pretend every Context Resolution coordinate is a sandbox primitive.

13. Minimal documentation decomposition
Alternatives
Decomposition	Assessment
Amend the two proposals plus one explanation decision	Insufficient. It leaves underpowered-run intent, readiness, producer capability, escalation, and output authority unowned.
Amend the two proposals plus separate readiness and explanation decisions	Recommended. Each document owns one causal seam and neither duplicates Engine or gate semantics.
Add a general Context Resolution adoption decision now	Too broad. Ordinary adoption is consumer-specific, and host enforcement has no selected adopter.
Add a Handbook-to-Substrate enforcement decision now	Premature. The external host/adopter boundary remains unresolved.
Put readiness inside Assurance Policy	Incorrect. It would let current capability feed back into requirement selection.
Put explanation density into Context Resolution	Incorrect. It would make a presentation preference capable of changing evaluation semantics.
Recommended documents
A. Amend metric-gate-semantic-decision.md

Own:

typed Context Resolution provenance in evaluation closure;

per-claim observation adequacy;

Engine/Contracts ownership;

stack evolution;

candidate admission versus claim qualification;

candidate-refusal versus evaluation-refusal boundary;

gate’s transitive Resolution provenance.

B. Amend posture-assurance-policy-semantic-decision.md

Own:

exact non-equivalence vocabulary;

selection of requirements that carry observation minima;

no current-capability selector input;

no authority grant;

exact stack semantic binding in Resolved Assurance Requirements;

no cross-monotonicity between posture and Context Resolution ranks.

C. New evaluation-capability-and-readiness-semantic-decision.md

Own:

Engine authorization binding versus evaluation request;

producer capability bases;

readiness states;

decision-seeking versus diagnostic intent;

preflight;

escalation;

retries;

cache reuse and supersession;

advisory CLI versus actual host enforcement boundary.

It must not define new stack ranks, claim truth, metrics, gates, or host implementation.

D. New evaluation-explanation-projection-semantic-decision.md

Own:

canonical source result;

presentation density;

disclosure;

viewer authorization;

mandatory information floor;

omission/lossiness accounting;

same-result drill-down;

projection identity and reproducibility.

It must not own claim evaluation, gate truth, readiness, posture, or enforcement.

E. No host-enforcement decision yet

Create that record only once a real host and adopter are selected.

14. Errors and omissions in the first Pro review

The first review’s ADJUST direction was broadly correct, and the local reconciliation correctly repaired several factual overextensions. The second pass nevertheless finds the following remaining defects.

First review: unjustified or incomplete points
1. It treated Context Resolution too generically as “current run capability”

That phrase failed to distinguish:

Engine authorization;

requested evaluation reach;

producer capability ceiling;

actual observation.

This is not merely terminology. It determines which changes refuse, which cause readiness limitations, and which produce claim indeterminacy.

2. It did not identify the privilege-order hazard

The first review recommended adding exact Resolution requirements but did not challenge whether authority_horizon = program_policy can safely behave as an evidence-quality minimum. The shipped stack makes that a material concern. 

1.0.0

3. It assumed claim-specific A/B outcomes before resolving candidate atomicity

The first review said one observation could satisfy A while remaining insufficient for B. The local reconciliation correctly noted that current whole-candidate admission does not permit that unless candidates are partitioned. 

hcm-context-resolution-posture-…

Neither the first review nor the reconciliation selected the greenfield canonical model. This review selects admit-once/qualify-per-claim.

4. It generalized a producer-ceiling case beyond the current Dock

The local reconciliation correctly narrowed that claim: the current first-party Dock uses equality rather than a lower-ceiling completion. 

hcm-context-resolution-posture-…

5. It did not fully distinguish density from disclosure

It correctly made presentation density type-distinct from detail_resolution, but it left viewer/disclosure context largely as an optional projection input. Disclosure is a separate mandatory authority axis whenever protected explanation data exists.

6. Its “Evaluation Readiness and Adoption” record was too broad

Readiness is a coherent semantic question. General Context Resolution adoption across artifacts, CLI actions, tasks, and hosts is not one coherent decision. The companion should be narrowed to Evaluation Capability and Readiness.

7. It did not challenge the proposed unauthorized-source scenario

The proposal’s “no gate at all” result is under-specified unless the source failure also destroys evidence-set or collection closure. Candidate refusal and evaluation refusal need separate rules.

8. It did not specify diagnostic-output authority

Saying “diagnostic runs are legal” is insufficient without binding use intent and defining downstream restrictions.

Local reconciliation: remaining gaps

The local reconciliation correctly established that current evaluation-run identity already binds requested Resolution; therefore the gap is in the proposed semantic records, not a demonstrated current replay defect. 

hcm-context-resolution-posture-…

It also correctly identified:

current Dock equality;

whole-candidate claim-minimum rejection;

vocabulary collision;

the need to distinguish candidate admission, verdict recheck, and claim indeterminacy. 

hcm-context-resolution-posture-…

Its unresolved point was the exact greenfield answer. This review resolves that answer through:

exact role separation;

admit-once/qualify-per-claim;

a separate readiness lifecycle;

explicit diagnostic provenance;

density/disclosure separation.

15. Discriminating scenario corpus
Scenario	Required outcome
Same coordinate labels, changed stack fingerprint	New context or refusal; never same identity
Producer A has full detail/local scope; B has normal detail/slice scope	Envelopes are incomparable; no global “better producer”
Claim requires program_policy only because “higher is better”	Requirement definition refuses unless that dimension has an explicit compatible role; no authority grant
Request exceeds Engine authorization	Context formation refuses
Request is valid but below a known required claim minimum	Readiness KNOWN_LIMITED; no gate yet
Known-limited decision-seeking request	Hold or create new request; cannot silently continue
Same request explicitly diagnostic	May execute; unchanged requirements; downstream positive action requires explicit policy
One record has normal detail; A requires summary, B requires full	Record admitted; eligible for A, ineligible for B
Candidate common provenance is invalid	Whole candidate refuses
Unsolicited unauthorized candidate plus complete valid evidence	Unauthorized candidate rejected and accounted; it cannot veto valid closed evaluation
Required source returns unauthorized candidate and no alternative exists	No record from that source; affected claim becomes indeterminate unless evidence-set integrity itself is unclosable
Same identity appears with changed bytes	Integrity refusal; no semantic evaluation from ambiguous set
Producer ceiling below request	Readiness limitation or producer incompatibility; current strict Dock would refuse rather than degrade
Actual below request, within ceiling	Honest partial observation; claim-specific adequacy
Actual above request	Candidate refusal; never silently cap
Actual above producer ceiling	Candidate refusal for inconsistent capability claim
Posture selects hardened proof while current run is read-only	Requirements stay hardened; no automatic grant or relaxation
Posture changes mid-run	Existing run stays bound to old snapshot; fresh run required
Resolution escalation is approved mid-run	Old run remains unchanged; new context and run
Quick Answer omits decisive security failure and shows 99/100	Invalid projection
Audit Trail requested but evidence identities are restricted	Same gate; authorized explanation plus explicit withheld marker
Viewer changes Quick Answer to Audit Trail	Same canonical gate/result identity; new projection identity only
Stack policy changes after a cached readiness result	Readiness cache invalid; recompute or refuse
New evidence arrives after an indeterminate gate	New evidence root and new evaluation/gate; old result remains immutable
Later run supersedes earlier run	Workflow relation only; earlier truth is not rewritten
Same gate truth under two Enforcement Policies	Gate remains identical; dispositions may differ
CLI says read-only while unrestricted shell writes elsewhere	No claim of task-wide enforcement
Host cannot apply required network restriction	Hold/refuse; no “enforced” receipt
Diagnostic run happens to pass	Semantic pass remains exact, but positive progression is held unless policy accepts diagnostic provenance
16. Narrow normative amendments
A. Clauses for the metric/evaluation/gate proposal
New section: Typed Context Resolution and observation adequacy

Context Resolution authority. The Engine Context Resolution domain is the sole authority for stack identities, dimension domains, rank and comparison semantics, componentwise meet or narrowing, parent non-expansion, and escalation semantics. Contracts and every other consumer MUST use exact Engine-owned semantics and MUST NOT define a competing order, scalar, default, or compatibility rule.

Evaluation closure. An evaluation context whose evidence semantics depend on Context Resolution MUST bind the exact Context Resolution stack identity, version, semantic-content fingerprint, and currentness; the exact Engine-admitted run authorization binding or exact owner-produced projection of it; and the exact requested evaluation Resolution envelope. A changed or missing binding produces a distinct context or refuses.

Requirement closure. An exact selected claim, evidence requirement, or compatible Evidence Evaluation Policy MAY declare an observation-adequacy requirement over an explicitly named subset of Context Resolution dimensions. No dimension is relevant by implication. The requirement MUST bind the exact stack semantics under which its values are interpreted.

Requirement/capability separation. An observation-adequacy requirement does not grant scope, authority, memory, validation, or another capability. The current run authorization, request, producer ceiling, or actual observation MUST NOT relax the selected requirement or reselect Assurance Requirements.

No scalarization or privilege ordering. Context Resolution values MUST NOT be averaged, weighted, converted to a universal score, or treated as assurance strength. A higher rank in authority_horizon or another privilege-bearing dimension does not intrinsically establish stronger evidence.

Observation honesty. A candidate MUST bind its exact evaluation context, admitted producer ceiling when one exists, and actual observation Resolution. Actual observation beyond the authorized or requested envelope, beyond an admitted producer ceiling, or under a different unresolved stack MUST refuse; it MUST NOT be silently clamped or relabeled.

Effective observation. The effective observation Resolution is the exact Engine-owned componentwise meet of the applicable admitted authorization/request, producer ceiling when present, and actual observation envelope. Its derivation and every input remain visible in canonical accounting.

Per-claim adequacy. Contracts compares one admitted observation independently against each selected claim’s exact requirement. Failure to meet one claim’s requirement makes the observation ineligible for that claim and MUST NOT invalidate an otherwise valid binding to another claim.

Admission versus evaluation. Candidate-wide structural, authority, provenance, stack, scope, or identity failure may refuse the candidate. Claim-specific observation inadequacy is not a candidate-wide trust failure. A rejected candidate cannot satisfy a claim, but whether the evaluation may proceed depends on complete collection and evidence-set accounting.

Metric and gate separation. Metrics and gates MUST NOT recompute or aggregate raw Context Resolution. They consume exact claim evaluations. A gate result binds the evaluation context and claim evaluations transitively and exposes any decisive Resolution inadequacy in its canonical explanation.

Scenario amendments

Add:

same observation sufficient for A, insufficient for B;

actual observation exceeds request;

changed stack fingerprint with same labels;

unauthorized unsolicited candidate versus required-source failure;

diagnostic underpowered run;

privilege-bearing dimension cannot act as ambient assurance strength.

B. Clauses for the posture/assurance proposal

Rename:

Resolution and selection invariants

to:

Posture resolution and assurance selection invariants

Then add:

Typed-domain non-equivalence. Posture resolution in this record is not the Engine Context Resolution domain. Posture Level is not a Context Resolution preset or active level. Posture Scope is not scope_horizon. Assurance strictness is not validation_horizon or another Context Resolution rank. Explanation density is not detail_resolution.

Selected observation requirements. An Assurance Policy MAY select exact claims, evidence requirements, or compatible Evidence Evaluation Policies that carry Context Resolution observation-adequacy requirements. Resolved Assurance Requirements transitively bind those requirements and their exact stack semantic identities.

No runtime capability input. The current run authorization, requested Resolution, producer capability ceiling, actual observation Resolution, readiness result, or presentation preference MUST NOT be an Assurance Policy Selector input unless a later independently authorized policy defines a non-relaxing compatibility check that cannot change the selected requirements. No such ambient input exists by default.

No capability grant. An Assurance Policy and Resolved Assurance Requirements MUST NOT grant, broaden, or imply scope, authority, memory, validation, mutation, or host capability. A requirement that cannot be met under the current run remains selected.

No cross-domain monotonicity. The posture policy’s requirement-strength relation is independent of Context Resolution rank. Increasing a Context Resolution coordinate does not by itself mean stronger assurance, and increasing posture does not automatically authorize broader Context Resolution.

Fresh context after escalation. A Context Resolution escalation or changed stack semantic identity never rebinds an existing evaluation. A newly approved binding requires a fresh evaluation context while the prior posture snapshot and requirements remain separately current or are freshly resolved.

C. Outline for the Evaluation Capability and Readiness decision

Authority and non-goals

Pure pre-execution readiness.

No new Context Resolution stack.

No claim, gate, enforcement, or host implementation.

Canonical inputs

Resolved Assurance Requirements.

Exact subject/scope and available applicability facts.

Exact Engine authorization binding.

Requested observation envelope.

Exact producer capability bases and currentness.

Evaluation use intent.

Use intent

decision_seeking

diagnostic

Intent included in context identity but does not alter evaluation algorithms.

Readiness states

ready

known_limited

unknown

formation refusal remains outside readiness.

Comparison rules

Engine-owned stack semantics.

Per-claim and per-producer accounting.

No scalar or nearest-match selection.

Applicability unknown stays readiness unknown.

Workflow interpretation

Known-limited decision-seeking request holds.

Diagnostic continuation is explicit.

No requirement relaxation.

Escalation

Exact gap and requested change.

Non-authorizing request.

Approval creates fresh Engine binding and context.

Retry, reuse, and supersession

Exact idempotent replay.

New actual observation produces new evidence.

Cache keys and currentness.

Supersession does not mutate old truth.

Adapter and host boundary

Current strict Dock equality is one producer policy.

CLI advice is not host enforcement.

Scenarios

Known-limited, unknown, incomparable producers, partial actual, escalation, diagnostic use.

D. Outline for the Evaluation Explanation Projection decision

Canonical source

One exact canonical evaluation/gate result and explanation.

No re-evaluation.

Presentation-density type

Quick Answer

Why

Audit Trail

Explicitly distinct from Context Resolution.

Disclosure inputs

Exact disclosure policy.

Viewer authority/currentness.

Optional viewer Context Resolution binding only where the projection policy explicitly consumes it.

Mandatory floor

Gate decision or fact that no gate exists.

Decisive fail/indeterminate/refusal reason.

Material unknowns and stale/conflicting inputs.

Material posture/assurance basis.

Applied exception.

Next remediation or escalation category.

Canonical result ref.

Withheld-detail marker where applicable.

Lossiness and omissions

Exact omitted categories and reasons.

No decisive-fact omission.

No favorable-score-only projection.

Drill-down

Same canonical result.

Higher-density request may be partially or wholly withheld.

No rerun.

Identity and caching

Result ref + density + disclosure policy + viewer authorization/currentness.

Never derive a richer view from a lower-density view.

Scenarios

decisive security failure;

disclosure-denied audit;

viewer authorization change;

same decision under all densities.

17. Packet-insufficient points for later bounded inspection

[PACKET INSUFFICIENT] The supplied files do not independently expose the complete current source definitions for the full Engine ContextResolutionEnvelope; the bounded prompt supplies those facts as repository-checked agenda, but later planning should inspect the exact owner type and projection seam.

[PACKET INSUFFICIENT] The packet does not establish whether one current candidate can contain several independently resolved observation closures or only one shared actual Resolution.

[PACKET INSUFFICIENT] The packet does not establish an existing general-purpose viewer/disclosure authority suitable for gate explanation projection.

[PACKET INSUFFICIENT] The exact current Engine escalation request and approval call graph is not included.

[PACKET INSUFFICIENT] The complete current mapping of every HCM-5.2 blocked reason into the proposed refusal/fail/indeterminate model requires source and test inspection.

[PACKET INSUFFICIENT] No supplied source can identify actual Substrate capability primitives, bridge protocols, or enforcement receipts.

These are planning inspections, not reasons to leave the semantic seams ambiguous.

18. Validation required before implementation planning

The future semantic scenario suite should prove at minimum:

same coordinate labels under a changed stack fingerprint cannot reuse an evaluation identity;

a request above the Engine authorization binding refuses;

a producer actual above request or ceiling refuses rather than clamps;

a lower producer ceiling never lowers requirements;

one admitted record can be eligible for A and ineligible for B;

a rejected unsolicited candidate cannot veto an otherwise closed evaluation;

ambiguous evidence inventory refuses before gate truth;

known-limited readiness is not gate indeterminate;

diagnostic intent does not change findings, metrics, or gate;

diagnostic provenance cannot silently authorize positive progression;

approved escalation requires a fresh context;

a newer evaluation does not mutate an older decision;

caches invalidate on stack, policy, subject, time/currentness, evidence, producer basis, or authorization changes;

every explanation density preserves decisive facts;

disclosure denial produces an explicit withholding marker;

changing explanation density leaves the canonical result identity unchanged;

current run capability never feeds back into Assurance Policy selection;

higher posture never automatically grants broader authority;

host inability to enforce a declared restriction cannot produce an enforcement receipt.

19. Tradeoffs
More provenance

Binding exact stack, authorization, request, producer, actual, and requirement roles increases identity and explanation size.

That cost is justified because those facts can produce different semantic outcomes. Compressing them into one “Resolution” field would make replay and blame ambiguous.

More claim accounting

Admit-once/qualify-per-claim retains records that may be unusable for some claims and requires per-claim adequacy accounting.

That is preferable to losing valid evidence or letting candidate packaging determine semantic atomicity.

Diagnostic misuse risk

Allowing diagnostic runs creates a risk that consumers treat their output as approval.

Exact use intent and downstream Enforcement Policy checks are required. The alternative—declaring all underpowered contexts malformed—would erase useful diagnostic truth and conflate capability with context validity.

Two companion decisions instead of one

This adds documentation, but the seams are genuinely separate:

readiness answers whether a run can plausibly establish selected requirements;

explanation projection answers how the same completed truth is displayed.

Combining them would create a new umbrella “Resolution experience” abstraction with overlapping meanings.

20. Smallest safe sequence

Amend the metric/evaluation/gate proposal

typed Context Resolution closure;

authorization/request/producer/actual separation;

role-compatible minima;

admit-once/qualify-per-claim;

exact candidate-refusal/evaluation-refusal rules.

Amend the posture/assurance proposal

typed-domain non-equivalence;

exact requirement/stack binding;

no current-capability selector input;

no capability grant or cross-domain monotonicity.

Re-review the two amended proposals together

especially the shared Resolved Assurance Requirements → evaluation-context closure.

Approve the Evaluation Capability and Readiness decision

use intent, readiness states, producer bases, escalation, retry, caches, supersession.

Approve the Evaluation Explanation Projection decision

density, disclosure, mandatory floor, drill-down, identity.

Only then perform integrated implementation planning.

Defer host enforcement

until a real external adopter and capability broker are selected.

21. Scope expansions to defer

Do not add now:

a universal posture, assurance, evidence, or Resolution score;

“gate Resolution” or “posture Resolution” levels;

a scalar ranking of producer envelopes;

a generic all-Handbook rendering engine;

a universal producer marketplace;

automatic posture mutation from gate results;

automatic authority escalation to meet assurance;

cross-context gate composition;

a V1/V2 compatibility architecture merely to preserve current HCM-5.2 shapes;

host-capability APIs or Substrate transport design;

exact Rust types, operations, schemas, persistence, cache layout, or migrations;

broad Phase-6 publication or adoption planning.

Final answer

ADJUST.

The current architecture is the best way forward, but the proposals are not yet semantically complete enough to become the sole input to integrated implementation planning.

The most important corrected model is:

Posture establishes the engineering standard. Assurance selects the proof plan. Engine grants what the run may do. The evaluation requests reach within that grant. A producer declares what it can do and records what it actually observed. Readiness predicts whether the plan is feasible. Contracts decides what the observations establish. The gate states semantic truth. Projection changes only how that same truth is shown. Disclosure controls what may be revealed. Enforcement controls what may happen next.

None of those authorities should be allowed to impersonate another.

Sources
Is this conversation helpful so far?
~~~

## Local repository reconciliation

### Verdict

The second-pass `ADJUST` verdict is supported. The owner graph remains sound, but the two proposed semantic records are not yet sufficient as the sole input to integrated implementation planning. The smallest cohesive planning boundary is:

1. amend the metric/gate record;
2. amend the posture/assurance record;
3. cross-review those amendments together;
4. approve a narrow Evaluation Capability and Readiness decision;
5. approve a separate Evaluation Explanation Projection decision; and
6. then begin integrated task-level implementation planning.

Host/Substrate enforcement remains deferred until an actual capability broker and adopter are selected.

### Confirmed repository truths

- Engine owns the typed Context Resolution stack and full execution envelope. The full envelope binds the active stack and six dimensions together with parentage, constraints, mutation rules, escalation triggers, admitted authority, and an exact binding ([`context_resolution.rs`](../../crates/engine/src/context_resolution.rs)).
- The shipped stack ranks `authority_horizon` from `read_only` through `program_policy`, alongside five other independently ranked dimensions ([`1.0.0.yaml`](../../crates/engine/definitions/context-resolution/handbook.context-resolution.shipped-root/1.0.0.yaml)). Authority rank is therefore not intrinsically evidence quality.
- Current Contracts claims carry a complete six-dimension minimum, and evidence admission computes a componentwise minimum of request, actual observation, and optional producer ceiling before checking that effective envelope against every observed claim ([`evidence_admission.rs`](../../crates/contracts/src/evidence_admission.rs)).
- Current candidate admission rejects the entire candidate when any observed claim minimum is unmet. The later verdict evaluator independently rechecks Resolution per claim, but any insufficient record currently makes that claim tuple blocked even when another eligible record may exist ([`claim_verdict.rs`](../../crates/contracts/src/claim_verdict.rs)).
- Current evaluation-run identity already binds `requested_resolution`; the missing closure in the proposed records is not a demonstrated current replay omission ([`evaluation_run.rs`](../../crates/contracts/src/evaluation_run.rs)).
- A current evidence candidate contains one shared observation closure: one fact payload, one observation result, common sources/provenance, one requested envelope, and one actual envelope, with a complete claim partition ([`evaluation_identity.rs`](../../crates/contracts/src/evaluation_identity.rs)).
- The first-party Dock is intentionally stricter than the proposed general producer model: request construction requires requested Resolution equal the manifest ceiling, and completed output requires actual Resolution equal both ([`protocol.rs`](../../crates/dock/src/protocol.rs)).
- The proposed metric/gate record currently says an unauthorized candidate produces no later claim, metric, or gate result without distinguishing candidate-operation refusal from an otherwise closable evaluation ([`metric-gate-semantic-decision.md`](../specs/handbook-contract-membrane/metric-gate-semantic-decision.md)).
- HCM-4.6 provides a purpose-specific Resolution/disclosure-bounded snapshot-grounding projection, not a general gate-explanation viewer/disclosure authority ([`HCM-4.6`](../specs/handbook-contract-membrane/slices/HCM-4.6/README.md)).

### Accepted design corrections

1. **Separate the roles currently compressed into “Resolution.”** The semantic model must distinguish Engine authorization, requested observation reach, an admitted producer basis or ceiling, actual observation, and claim-specific observation adequacy. Requirements never grant authority; current capability never relaxes requirements.
2. **Declare dimension relevance rather than treating all six dimensions as ambient evidence strength.** In particular, `authority_horizon` must not become a proof-quality minimum merely because its ranks increase. This is a substantive greenfield model correction, not just missing prose.
3. **Admit one trustworthy observation closure, then qualify it independently per claim.** Claim-specific inadequacy should not erase an otherwise authentic record or make candidate packaging control the failure blast radius.
4. **Separate candidate refusal, evaluation-closure refusal, claim indeterminacy, gate truth, and enforcement hold.** An unsolicited rejected candidate must not automatically veto a complete valid evidence set.
5. **Add a distinct readiness lifecycle.** `ready`, `known_limited`, and `unknown` are pre-execution facts, not gate results. Explicit `decision_seeking` versus `diagnostic` intent belongs in evaluation-context provenance and must not alter claim, metric, or gate algorithms.
6. **Keep presentation density separate from both Context Resolution and disclosure.** Quick Answer, Why, and Audit Trail project the same canonical decision. Disclosure policy and viewer authority independently bound what each projection may reveal.
7. **Escalation and new evidence create fresh contexts/results.** Neither may retroactively upgrade or mutate a closed evaluation.

### Bounded corrections to the Pro recommendation

1. **Do not add multi-observation candidate normalization to this planning boundary.** The current candidate shape contains one shared observation closure. The Pro answer correctly labeled the broader shape packet-insufficient; local source inspection resolves it for current product truth. A future batch/compound producer can introduce normalization only when an actual input form requires it.
2. **Refine “producer capability ceiling.”** Current Dock `resolution_ceiling` is a host/admitted six-dimension operating contract, not merely a self-asserted evidence capability. A general readiness model should distinguish a producer's evidence-observation capability basis from an Engine/host-admitted operational ceiling. A producer can report capability; it cannot self-grant authority.
3. **The per-claim change is larger than removing one admission check.** Current verdict evaluation treats any insufficient relevant record as blocking the tuple. The greenfield rule needs an explicit Evidence Evaluation Policy for how ineligible records coexist with sufficient eligible alternatives; otherwise “admit once, qualify per claim” remains underspecified.
4. **An Engine authorization binding is not currently part of the Contracts run identity.** Contracts binds the requested six-coordinate envelope, while the richer `ContextResolutionEnvelope` remains Engine-owned. Adding an exact authorization-binding reference or owner-produced projection is a new seam that must preserve the Engine-to-Contracts dependency direction.
5. **Readiness must remain applicability-aware.** A known producer limitation may yield `known_limited` only for a known applicable, gate-relevant selected requirement. Unknown applicability stays `unknown`; readiness must not pre-evaluate claims or invent applicability truth.

### Approval blockers before integrated implementation planning

- exact dimension-role semantics and requirement representation, including the authority-as-quality prohibition;
- evidence-record admission versus per-claim adequacy, including coexistence of insufficient and sufficient records;
- candidate-operation refusal versus evaluation-closure refusal;
- authorization/request/producer/actual identity and currentness closure;
- readiness states, use intent, escalation, retries, caches, and supersession; and
- explanation density, disclosure, mandatory decisive-information floor, drill-down, and projection identity.

These blockers fit the four-document sequence above. They do not justify a fifth umbrella “Context Resolution adoption” record, a universal rigor score, a V1/V2 compatibility architecture, or premature Host/Substrate implementation design.

No proposal or decision document was changed during this consultation and reconciliation.
